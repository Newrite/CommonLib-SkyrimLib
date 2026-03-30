#![allow(non_camel_case_types)]

use core::ffi::c_void;
use core::marker::PhantomData;

use crate::re::{GAllocatorTraits, GArrayDefaultPolicy, GArraySizePolicy};

/// C++ `RE::GArrayDataBase<T, Allocator, SizePolicy>`
#[repr(C)]
pub struct GArrayDataBase<T, Allocator: GAllocatorTraits<T>, SizePolicy = GArrayDefaultPolicy> {
    pub data: *mut T,       // 00
    pub size: usize,        // 08
    pub policy: SizePolicy, // 10
    pub _allocator: PhantomData<fn() -> Allocator>,
}

const _: () = assert!(
    core::mem::size_of::<
        GArrayDataBase<*mut c_void, crate::re::GAllocatorGH<*mut c_void>, GArrayDefaultPolicy>,
    >() == 0x18
);
const _: () = assert!(
    core::mem::offset_of!(
        GArrayDataBase<*mut c_void, crate::re::GAllocatorGH<*mut c_void>, GArrayDefaultPolicy>,
        data
    ) == 0x0
);
const _: () = assert!(
    core::mem::offset_of!(
        GArrayDataBase<*mut c_void, crate::re::GAllocatorGH<*mut c_void>, GArrayDefaultPolicy>,
        size
    ) == 0x8
);
const _: () = assert!(
    core::mem::offset_of!(
        GArrayDataBase<*mut c_void, crate::re::GAllocatorGH<*mut c_void>, GArrayDefaultPolicy>,
        policy
    ) == 0x10
);

impl<T, Allocator: GAllocatorTraits<T>, SizePolicy> GArrayDataBase<T, Allocator, SizePolicy>
where
    SizePolicy: Default,
{
    #[inline(always)]
    pub fn new() -> Self {
        Self {
            data: core::ptr::null_mut(),
            size: 0,
            policy: SizePolicy::default(),
            _allocator: PhantomData,
        }
    }
}

impl<T, Allocator, SizePolicy> Default for GArrayDataBase<T, Allocator, SizePolicy>
where
    Allocator: GAllocatorTraits<T>,
    SizePolicy: Default,
{
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}

impl<T, Allocator, SizePolicy> GArrayDataBase<T, Allocator, SizePolicy>
where
    Allocator: GAllocatorTraits<T>,
{
    #[inline(always)]
    pub fn with_policy(policy: SizePolicy) -> Self {
        Self {
            data: core::ptr::null_mut(),
            size: 0,
            policy,
            _allocator: PhantomData,
        }
    }

    #[inline(always)]
    pub fn as_ptr(&self) -> *const T {
        self.data.cast_const()
    }

    #[inline(always)]
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.data
    }
}

impl<T, Allocator, SizePolicy> GArrayDataBase<T, Allocator, SizePolicy>
where
    Allocator: GAllocatorTraits<T>,
    SizePolicy: GArraySizePolicy,
{
    #[inline(always)]
    pub fn get_capacity(&self) -> usize {
        self.policy.get_capacity()
    }
}

impl<T, Allocator, SizePolicy> GArrayDataBase<T, Allocator, SizePolicy>
where
    SizePolicy: GArraySizePolicy,
    Allocator: GAllocatorTraits<T>,
{
    #[inline(always)]
    pub fn clear_and_release(&mut self) {
        if !self.data.is_null() {
            unsafe {
                Allocator::destruct_array(self.data, self.size);
                Allocator::free(self.data.cast());
            }
            self.data = core::ptr::null_mut();
            self.size = 0;
            self.policy.set_capacity(0);
        }
    }

    #[inline(always)]
    pub fn reserve(&mut self, heap_addr: *const c_void, mut new_capacity: usize) {
        if self.policy.never_shrinking() && new_capacity < self.get_capacity() {
            return;
        }

        if new_capacity < self.policy.get_min_capacity() {
            new_capacity = self.policy.get_min_capacity();
        }

        if new_capacity == 0 {
            if !self.data.is_null() {
                Allocator::free(self.data.cast());
                self.data = core::ptr::null_mut();
            }
            self.policy.set_capacity(0);
            return;
        }

        let granularity = self.policy.get_granularity().max(1);
        let rounded_capacity = new_capacity
            .saturating_add(granularity - 1)
            .checked_div(granularity)
            .unwrap_or(0)
            .saturating_mul(granularity);

        unsafe {
            if self.data.is_null() {
                self.data =
                    Allocator::alloc(heap_addr, core::mem::size_of::<T>() * rounded_capacity)
                        .cast();
            } else if Allocator::is_movable() {
                self.data = Allocator::realloc(
                    self.data.cast(),
                    core::mem::size_of::<T>() * rounded_capacity,
                )
                .cast();
            } else {
                // TODO: `GArrayDataBase.h` copy-constructs into the new buffer
                // and destructs the old elements for non-movable allocators.
                // The current translated constructor layer still models
                // `GConstructorMov<T>::is_movable()` as always true, so this
                // fallback path is not exercised by source-backed allocator
                // combinations today. Revisit this branch if a real
                // non-movable Scaleform constructor policy is translated.
                let new_data =
                    Allocator::alloc(heap_addr, core::mem::size_of::<T>() * rounded_capacity)
                        .cast::<T>();
                for i in 0..self.size {
                    new_data.add(i).write(self.data.add(i).read());
                }
                Allocator::free(self.data.cast());
                self.data = new_data;
            }
        }

        self.policy.set_capacity(rounded_capacity);
    }

    #[inline(always)]
    pub fn resize_no_construct(&mut self, heap_addr: *const c_void, new_size: usize) {
        let old_size = self.size;

        if new_size < old_size {
            unsafe {
                Allocator::destruct_array(self.data.add(new_size), old_size - new_size);
            }
            if new_size < (self.get_capacity() >> 1) {
                self.reserve(heap_addr, new_size);
            }
        } else if new_size >= self.get_capacity() {
            self.reserve(heap_addr, new_size.saturating_add(new_size >> 2));
        }

        self.size = new_size;
    }
}

impl<T, Allocator, SizePolicy> Drop for GArrayDataBase<T, Allocator, SizePolicy>
where
    Allocator: GAllocatorTraits<T>,
{
    #[inline(always)]
    fn drop(&mut self) {
        if !self.data.is_null() {
            unsafe {
                Allocator::destruct_array(self.data, self.size);
                Allocator::free(self.data.cast());
            }
        }
    }
}
