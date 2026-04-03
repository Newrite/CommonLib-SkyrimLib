//! Defensive helpers for iterating contiguous engine-owned containers.
//!
//! These helpers live in the SDK layer on purpose: they provide reusable
//! defensive traversal over raw RE container storage without pretending the
//! underlying RE types expose a universally safe slice API.

use alloc::vec::Vec;
use core::ops::ControlFlow;

use crate::re::{BSStaticArray, BSTArray, BSTArrayAllocator, hkArray, hkArrayBase, hkInplaceArray};

use super::sealed;

#[derive(Debug, Clone, Copy)]
pub struct ContiguousSequenceIterationOptions {
    pub max_reasonable_len: Option<u32>,
    pub require_capacity_at_least_len: bool,
}

impl ContiguousSequenceIterationOptions {
    #[inline(always)]
    pub const fn new() -> Self {
        Self {
            max_reasonable_len: None,
            require_capacity_at_least_len: true,
        }
    }

    #[inline(always)]
    pub const fn with_max_reasonable_len(mut self, max_reasonable_len: u32) -> Self {
        self.max_reasonable_len = Some(max_reasonable_len);
        self
    }

    #[inline(always)]
    pub const fn with_capacity_check(mut self, require_capacity_at_least_len: bool) -> Self {
        self.require_capacity_at_least_len = require_capacity_at_least_len;
        self
    }
}

impl Default for ContiguousSequenceIterationOptions {
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}

pub trait ContiguousSequence<T>: sealed::Sealed {
    fn len(&self) -> u32;
    fn data(&self) -> *const T;
    fn capacity_hint(&self) -> Option<u32>;
}

impl<T, A> sealed::Sealed for BSTArray<T, A> where A: BSTArrayAllocator {}

impl<T, A> ContiguousSequence<T> for BSTArray<T, A>
where
    A: BSTArrayAllocator,
{
    #[inline(always)]
    fn len(&self) -> u32 {
        self.len()
    }

    #[inline(always)]
    fn data(&self) -> *const T {
        self.data()
    }

    #[inline(always)]
    fn capacity_hint(&self) -> Option<u32> {
        Some(self.capacity())
    }
}

impl<T> sealed::Sealed for BSStaticArray<T> {}

impl<T> ContiguousSequence<T> for BSStaticArray<T> {
    #[inline(always)]
    fn len(&self) -> u32 {
        self.len()
    }

    #[inline(always)]
    fn data(&self) -> *const T {
        self.data()
    }

    #[inline(always)]
    fn capacity_hint(&self) -> Option<u32> {
        Some(self.len())
    }
}

impl<T> sealed::Sealed for hkArrayBase<T> {}

impl<T> ContiguousSequence<T> for hkArrayBase<T> {
    #[inline(always)]
    fn len(&self) -> u32 {
        self.len().min(u32::MAX as usize) as u32
    }

    #[inline(always)]
    fn data(&self) -> *const T {
        self.data()
    }

    #[inline(always)]
    fn capacity_hint(&self) -> Option<u32> {
        Some(self.capacity().max(0) as u32)
    }
}

impl<T, Allocator> sealed::Sealed for hkArray<T, Allocator> {}

impl<T, Allocator> ContiguousSequence<T> for hkArray<T, Allocator> {
    #[inline(always)]
    fn len(&self) -> u32 {
        self.base.len().min(u32::MAX as usize) as u32
    }

    #[inline(always)]
    fn data(&self) -> *const T {
        self.base.data()
    }

    #[inline(always)]
    fn capacity_hint(&self) -> Option<u32> {
        Some(self.base.capacity().max(0) as u32)
    }
}

impl<T, const N: usize, Allocator> sealed::Sealed for hkInplaceArray<T, N, Allocator> {}

impl<T, const N: usize, Allocator> ContiguousSequence<T> for hkInplaceArray<T, N, Allocator> {
    #[inline(always)]
    fn len(&self) -> u32 {
        self.base.base.len().min(u32::MAX as usize) as u32
    }

    #[inline(always)]
    fn data(&self) -> *const T {
        self.base.base.data()
    }

    #[inline(always)]
    fn capacity_hint(&self) -> Option<u32> {
        Some(self.base.base.capacity().max(0) as u32)
    }
}

#[inline(always)]
pub fn contiguous_sequence_bounds<T, S>(
    sequence: &S,
    options: ContiguousSequenceIterationOptions,
) -> Option<(*const T, u32)>
where
    S: ContiguousSequence<T>,
{
    contiguous_sequence_bounds_named(sequence, core::any::type_name::<S>(), options)
}

#[inline(always)]
pub fn contiguous_sequence_bounds_named<T, S>(
    sequence: &S,
    _caller: &'static str,
    options: ContiguousSequenceIterationOptions,
) -> Option<(*const T, u32)>
where
    S: ContiguousSequence<T>,
{
    let len = sequence.len();
    if len == 0 {
        return None;
    }

    if let Some(capacity) = sequence.capacity_hint() {
        if options.require_capacity_at_least_len && capacity < len {
            crate::defensive_sdk_warn!(
                "{} observed {} with len={} > capacity={}",
                _caller,
                core::any::type_name::<S>(),
                len,
                capacity
            );
            return None;
        }
    }

    if let Some(max_reasonable_len) = options.max_reasonable_len {
        if len > max_reasonable_len {
            crate::defensive_sdk_warn!(
                "{} observed suspicious {} len={} (max={})",
                _caller,
                core::any::type_name::<S>(),
                len,
                max_reasonable_len
            );
            return None;
        }
    }

    let data = sequence.data();
    if data.is_null() {
        crate::defensive_sdk_warn!(
            "{} observed {}<{}> with non-zero len={} and null data",
            _caller,
            core::any::type_name::<S>(),
            core::any::type_name::<T>(),
            len
        );
        return None;
    }

    let _ = (data as usize).checked_add(len as usize * core::mem::size_of::<T>())?;
    Some((data, len))
}

#[inline(always)]
pub fn for_each_contiguous_sequence<T, S>(
    sequence: &S,
    options: ContiguousSequenceIterationOptions,
    visit: impl FnMut(&T) -> ControlFlow<()>,
) -> ControlFlow<()>
where
    S: ContiguousSequence<T>,
{
    for_each_contiguous_sequence_named(sequence, core::any::type_name::<S>(), options, visit)
}

#[inline(always)]
pub fn for_each_contiguous_sequence_named<T, S>(
    sequence: &S,
    _caller: &'static str,
    options: ContiguousSequenceIterationOptions,
    mut visit: impl FnMut(&T) -> ControlFlow<()>,
) -> ControlFlow<()>
where
    S: ContiguousSequence<T>,
{
    let Some((data, len)) = contiguous_sequence_bounds_named(sequence, _caller, options) else {
        return ControlFlow::Continue(());
    };

    for index in 0..len as usize {
        let item = unsafe { &*data.add(index) };
        if let ControlFlow::Break(()) = visit(item) {
            return ControlFlow::Break(());
        }
    }

    ControlFlow::Continue(())
}

#[inline(always)]
pub fn snapshot_contiguous_copied<T, S>(
    sequence: &S,
    options: ContiguousSequenceIterationOptions,
) -> Vec<T>
where
    T: Copy,
    S: ContiguousSequence<T>,
{
    snapshot_contiguous_copied_named(sequence, core::any::type_name::<S>(), options)
}

#[inline(always)]
pub fn snapshot_contiguous_copied_named<T, S>(
    sequence: &S,
    _caller: &'static str,
    options: ContiguousSequenceIterationOptions,
) -> Vec<T>
where
    T: Copy,
    S: ContiguousSequence<T>,
{
    let Some((data, len)) = contiguous_sequence_bounds_named(sequence, _caller, options) else {
        return Vec::new();
    };

    let mut out = Vec::with_capacity(len as usize);
    for index in 0..len as usize {
        out.push(unsafe { *data.add(index) });
    }
    out
}

#[inline(always)]
pub fn snapshot_contiguous_cloned<T, S>(
    sequence: &S,
    options: ContiguousSequenceIterationOptions,
) -> Vec<T>
where
    T: Clone,
    S: ContiguousSequence<T>,
{
    snapshot_contiguous_cloned_named(sequence, core::any::type_name::<S>(), options)
}

#[inline(always)]
pub fn snapshot_contiguous_cloned_named<T, S>(
    sequence: &S,
    _caller: &'static str,
    options: ContiguousSequenceIterationOptions,
) -> Vec<T>
where
    T: Clone,
    S: ContiguousSequence<T>,
{
    let Some((data, len)) = contiguous_sequence_bounds_named(sequence, _caller, options) else {
        return Vec::new();
    };

    let mut out = Vec::with_capacity(len as usize);
    for index in 0..len as usize {
        out.push(unsafe { (&*data.add(index)).clone() });
    }
    out
}
