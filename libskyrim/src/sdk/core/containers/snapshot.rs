use alloc::vec::Vec;

use super::traversal::contiguous_sequence_bounds_named;
use super::types::{ContiguousSequence, ContiguousSequenceIterationOptions};

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
