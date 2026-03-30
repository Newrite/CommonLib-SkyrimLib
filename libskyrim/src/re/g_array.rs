#![allow(non_camel_case_types)]

use core_util::inherit;

use crate::re::{
    GAllocatorGH, GArrayBase, GArrayData, GArrayDefaultPolicy, GArraySizePolicy, GStatGroup,
};

/// C++ `RE::GArray<T, SID, SizePolicy>`
#[repr(C)]
pub struct GArray<
    T,
    const SID: u32 = { GStatGroup::DEFAULT_MEM as u32 },
    SizePolicy = GArrayDefaultPolicy,
> {
    pub base: GArrayBase<GArrayData<T, GAllocatorGH<T, SID>, SizePolicy>>, // 00
}

const _: () = assert!(core::mem::size_of::<GArray<*mut core::ffi::c_void>>() == 0x18);
const _: () = assert!(core::mem::offset_of!(GArray<*mut core::ffi::c_void>, base) == 0x0);

inherit!(for[T, const SID: u32, SizePolicy] GArray<T, SID, SizePolicy> : GArrayBase<GArrayData<T, GAllocatorGH<T, SID>, SizePolicy>>, base);

impl<T, const SID: u32, SizePolicy> GArray<T, SID, SizePolicy>
where
    SizePolicy: Default,
{
    #[inline(always)]
    pub fn new() -> Self {
        Self {
            base: GArrayBase::new(),
        }
    }
}

impl<T, const SID: u32, SizePolicy> Default for GArray<T, SID, SizePolicy>
where
    SizePolicy: Default,
{
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}

impl<T, const SID: u32, SizePolicy> Clone for GArray<T, SID, SizePolicy>
where
    SizePolicy: GArraySizePolicy + Clone,
    T: Clone,
{
    #[inline(always)]
    fn clone(&self) -> Self {
        Self {
            base: self.base.clone(),
        }
    }
}

impl<T, const SID: u32, SizePolicy> GArray<T, SID, SizePolicy>
where
    SizePolicy: GArraySizePolicy + Default,
    T: Default,
{
    #[inline(always)]
    pub fn with_size(size: i32) -> Self {
        Self {
            base: GArrayBase::with_size(size),
        }
    }
}

impl<T, const SID: u32, SizePolicy> GArray<T, SID, SizePolicy>
where
    SizePolicy: GArraySizePolicy + Default,
{
    #[inline(always)]
    pub fn with_policy(policy: SizePolicy) -> Self {
        let mut out = Self::new();
        out.set_size_policy(policy);
        out
    }
}
