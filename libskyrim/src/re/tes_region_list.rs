use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_TESRegionList;
use crate::offsets::offsets_vtable::VTABLE_TESRegionList;
use crate::re::{BSSimpleList, TESRegion};
use crate::relocation::{RttiType, VariantID};

type TESRegionListBase = BSSimpleList<*mut TESRegion>;

#[repr(C)]
pub struct TESRegionList {
    pub vtable: *const usize,
    pub base: TESRegionListBase,
    pub owns_region_memory: bool,
    pub pad19: u8,
    pub pad1a: u16,
    pub pad1c: u32,
}

const _: () = assert!(core::mem::size_of::<TESRegionList>() == 0x20);
const _: () = assert!(core::mem::offset_of!(TESRegionList, base) == 0x08);
const _: () = assert!(core::mem::offset_of!(TESRegionList, owns_region_memory) == 0x18);
const _: () = assert!(core::mem::offset_of!(TESRegionList, pad19) == 0x19);
const _: () = assert!(core::mem::offset_of!(TESRegionList, pad1a) == 0x1A);
const _: () = assert!(core::mem::offset_of!(TESRegionList, pad1c) == 0x1C);

impl RttiType for TESRegionList {
    const RTTI: VariantID = RTTI_TESRegionList;
}

inherit!(TESRegionList => TESRegionListBase, base);

impl TESRegionList {
    pub const RTTI: VariantID = RTTI_TESRegionList;
    pub const VTABLE: &'static [VariantID] = &VTABLE_TESRegionList;

    crate::virtual_method! {
        pub const VFUNC_DTOR: usize = 0x00;
        pub fn dtor()
    }
}
