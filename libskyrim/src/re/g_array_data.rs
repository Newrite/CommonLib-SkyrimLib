#![allow(non_camel_case_types)]

use core::ffi::c_void;

use core_util::inherit;

use crate::re::{GAllocatorTraits, GArrayDataBase, GArrayDefaultPolicy, GArraySizePolicy};

/// C++ `RE::GArrayData<T, Allocator, SizePolicy>`
#[repr(C)]
pub struct GArrayData<T, Allocator: GAllocatorTraits<T>, SizePolicy = GArrayDefaultPolicy> {
    pub base: GArrayDataBase<T, Allocator, SizePolicy>, // 00
}

const _: () = assert!(
    core::mem::size_of::<
        GArrayData<*mut c_void, crate::re::GAllocatorGH<*mut c_void>, GArrayDefaultPolicy>,
    >() == 0x18
);
const _: () = assert!(
    core::mem::offset_of!(
        GArrayData<*mut c_void, crate::re::GAllocatorGH<*mut c_void>, GArrayDefaultPolicy>,
        base
    ) == 0x0
);

inherit!(for[T, Allocator: GAllocatorTraits<T>, SizePolicy] GArrayData<T, Allocator, SizePolicy> : GArrayDataBase<T, Allocator, SizePolicy>, base);

impl<T, Allocator, SizePolicy> GArrayData<T, Allocator, SizePolicy>
where
    Allocator: GAllocatorTraits<T>,
    SizePolicy: Default,
{
    #[inline(always)]
    pub fn new() -> Self {
        Self {
            base: GArrayDataBase::new(),
        }
    }
}

impl<T, Allocator, SizePolicy> Default for GArrayData<T, Allocator, SizePolicy>
where
    Allocator: GAllocatorTraits<T>,
    SizePolicy: Default,
{
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}

impl<T, Allocator, SizePolicy> Clone for GArrayData<T, Allocator, SizePolicy>
where
    SizePolicy: Clone + GArraySizePolicy,
    Allocator: GAllocatorTraits<T>,
    T: Clone,
{
    #[inline(always)]
    fn clone(&self) -> Self {
        let mut out = Self {
            base: GArrayDataBase::with_policy(self.base.policy.clone()),
        };
        out.append(self.base.data, self.base.size);
        out
    }
}

impl<T, Allocator, SizePolicy> GArrayData<T, Allocator, SizePolicy>
where
    SizePolicy: GArraySizePolicy + Default,
    Allocator: GAllocatorTraits<T>,
    T: Default,
{
    #[inline(always)]
    pub fn with_size(size: i32) -> Self {
        let mut out = Self::new();
        out.resize(size.max(0) as usize);
        out
    }
}

impl<T, Allocator, SizePolicy> GArrayData<T, Allocator, SizePolicy>
where
    Allocator: GAllocatorTraits<T>,
{
    #[inline(always)]
    pub fn with_policy(policy: SizePolicy) -> Self {
        Self {
            base: GArrayDataBase::with_policy(policy),
        }
    }
}

impl<T, Allocator, SizePolicy> GArrayData<T, Allocator, SizePolicy>
where
    SizePolicy: GArraySizePolicy,
    Allocator: GAllocatorTraits<T>,
{
    #[inline(always)]
    pub fn reserve(&mut self, new_capacity: usize) {
        self.base
            .reserve(self as *const Self as *const c_void, new_capacity);
    }

    #[inline(always)]
    pub fn push_back_alt<S>(&mut self, value: S)
    where
        S: Into<T>,
    {
        let old_size = self.base.size;
        self.base
            .resize_no_construct(self as *const Self as *const c_void, old_size + 1);
        unsafe {
            Allocator::construct_alt(self.base.data.add(old_size), value);
        }
    }
}

impl<T, Allocator, SizePolicy> GArrayData<T, Allocator, SizePolicy>
where
    SizePolicy: GArraySizePolicy,
    Allocator: GAllocatorTraits<T>,
    T: Clone,
{
    #[inline(always)]
    pub fn push_back(&mut self, value: &T) {
        let old_size = self.base.size;
        self.base
            .resize_no_construct(self as *const Self as *const c_void, old_size + 1);
        unsafe {
            Allocator::construct_copy(self.base.data.add(old_size), value);
        }
    }

    #[inline(always)]
    pub fn append(&mut self, other: *const T, count: usize) {
        if count == 0 {
            return;
        }

        let old_size = self.base.size;
        self.base
            .resize_no_construct(self as *const Self as *const c_void, old_size + count);
        unsafe {
            Allocator::construct_array_from(self.base.data.add(old_size), count, other);
        }
    }
}

impl<T, Allocator, SizePolicy> GArrayData<T, Allocator, SizePolicy>
where
    SizePolicy: GArraySizePolicy,
    Allocator: GAllocatorTraits<T>,
    T: Default,
{
    #[inline(always)]
    pub fn resize(&mut self, new_size: usize) {
        let old_size = self.base.size;
        self.base
            .resize_no_construct(self as *const Self as *const c_void, new_size);
        if new_size > old_size {
            unsafe {
                Allocator::construct_array(self.base.data.add(old_size), new_size - old_size);
            }
        }
    }
}
