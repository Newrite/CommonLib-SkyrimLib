#![allow(non_camel_case_types)]

use core_util::inherit;

use crate::re::GRefCountImplOps;

/// C++ `RE::GRefCountBaseStatImpl<Base, StatType>`
#[repr(transparent)]
pub struct GRefCountBaseStatImpl<Base, const STAT_TYPE: u32> {
    pub base: Base,
}

const _: () =
    assert!(core::mem::size_of::<GRefCountBaseStatImpl<crate::re::GRefCountImpl, 0>>() == 0x10);
const _: () =
    assert!(core::mem::offset_of!(GRefCountBaseStatImpl<crate::re::GRefCountImpl, 0>, base) == 0x0);

inherit!(for[Base, const STAT_TYPE: u32] GRefCountBaseStatImpl<Base, STAT_TYPE> : Base, base);

impl<Base, const STAT_TYPE: u32> GRefCountBaseStatImpl<Base, STAT_TYPE> {
    pub const STAT_TYPE_VALUE: u32 = STAT_TYPE;

    // TODO: `GRefCountBaseStatImpl.h` also injects the Scaleform heap-backed
    // C++ `new` / `delete` override layer for refcounted bases. Rust preserves
    // the layout/stat surface plus refcount helpers here, but still lacks a
    // source-backed direct allocation/delete story for these template bases.
    // Keep construction on engine-side factories or existing ABI-safe bridges.
}

impl<Base, const STAT_TYPE: u32> GRefCountBaseStatImpl<Base, STAT_TYPE>
where
    Base: GRefCountImplOps,
{
    #[inline(always)]
    pub fn add_ref(&mut self) {
        self.base.add_ref_impl();
    }

    #[inline(always)]
    pub fn release(&mut self) {
        self.base.release_impl();
    }
}

impl<Base, const STAT_TYPE: u32> AsRef<GRefCountBaseStatImpl<Base, STAT_TYPE>>
    for GRefCountBaseStatImpl<Base, STAT_TYPE>
{
    #[inline(always)]
    fn as_ref(&self) -> &GRefCountBaseStatImpl<Base, STAT_TYPE> {
        self
    }
}

impl<Base, const STAT_TYPE: u32> AsMut<GRefCountBaseStatImpl<Base, STAT_TYPE>>
    for GRefCountBaseStatImpl<Base, STAT_TYPE>
{
    #[inline(always)]
    fn as_mut(&mut self) -> &mut GRefCountBaseStatImpl<Base, STAT_TYPE> {
        self
    }
}

pub trait GRefCountBaseStatImplExt<Base, const STAT_TYPE: u32>:
    AsRef<GRefCountBaseStatImpl<Base, STAT_TYPE>> + AsMut<GRefCountBaseStatImpl<Base, STAT_TYPE>>
where
    Base: GRefCountImplOps,
{
    #[inline(always)]
    fn add_ref(&mut self) {
        self.as_mut().add_ref();
    }

    #[inline(always)]
    fn release(&mut self) {
        self.as_mut().release();
    }

    #[inline(always)]
    fn k_stat_type(&self) -> u32 {
        GRefCountBaseStatImpl::<Base, STAT_TYPE>::STAT_TYPE_VALUE
    }
}

impl<T, Base, const STAT_TYPE: u32> GRefCountBaseStatImplExt<Base, STAT_TYPE> for T
where
    T: AsRef<GRefCountBaseStatImpl<Base, STAT_TYPE>>
        + AsMut<GRefCountBaseStatImpl<Base, STAT_TYPE>>,
    Base: GRefCountImplOps,
{
}
