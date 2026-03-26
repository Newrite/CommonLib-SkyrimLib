use core_util::EnumSet;

use crate::offsets::offsets_rtti::RTTI_TESRegionData;
use crate::offsets::offsets_vtable::VTABLE_TESRegionData;
use crate::re::TESForm;
use crate::relocation::{RttiType, VariantID};

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TESRegionDataHeaderFlag {
    None = 0,
    Override = 1 << 0,
}

core_util::impl_enumset_type!(TESRegionDataHeaderFlag => u8);

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TESRegionDataType {
    Objects = 2,
    Weather = 3,
    Map = 4,
    Land = 5,
    Grass = 6,
    Sound = 7,
    Imposter = 8,
}

#[repr(C)]
pub struct TESRegionDataHeader {
    pub flags: EnumSet<TESRegionDataHeaderFlag, u8>,
    pub unk09: u8,
    pub priority: u8,
    pub unk0b: u8,
    pub unk0c: u32,
}

const _: () = assert!(core::mem::size_of::<TESRegionDataHeader>() == 0x8);
const _: () = assert!(core::mem::offset_of!(TESRegionDataHeader, flags) == 0x0);
const _: () = assert!(core::mem::offset_of!(TESRegionDataHeader, priority) == 0x2);
const _: () = assert!(core::mem::offset_of!(TESRegionDataHeader, unk0c) == 0x4);

#[repr(C)]
pub struct TESRegionData {
    pub vtable: *const usize,
    pub data_header: TESRegionDataHeader,
}

const _: () = assert!(core::mem::size_of::<TESRegionData>() == 0x10);
const _: () = assert!(core::mem::offset_of!(TESRegionData, data_header) == 0x08);

impl RttiType for TESRegionData {
    const RTTI: VariantID = RTTI_TESRegionData;
}

impl TESRegionData {
    pub const RTTI: VariantID = RTTI_TESRegionData;
    pub const VTABLE: &'static [VariantID] = &VTABLE_TESRegionData;

    crate::virtual_method! {
        pub const VFUNC_DTOR: usize = 0x00;
        pub fn dtor()
    }

    crate::virtual_method! {
        pub const VFUNC_LOAD_DATA_HEADER: usize = 0x01;
        pub fn load_data_header(arg1: *mut core::ffi::c_void) -> bool
    }

    crate::virtual_method! {
        pub const VFUNC_LOAD_DATA: usize = 0x02;
        pub fn load_data(form: *mut TESForm)
    }

    crate::virtual_method! {
        pub const VFUNC_GET_TYPE: usize = 0x03;
        pub fn get_type() -> TESRegionDataType
    }

    crate::virtual_method! {
        pub const VFUNC_CONSTRUCT_SELF: usize = 0x04;
        pub fn construct_self()
    }

    crate::virtual_method! {
        pub const VFUNC_UNK_05: usize = 0x05;
        pub fn unk_05()
    }

    crate::virtual_method! {
        pub const VFUNC_COPY_FROM: usize = 0x06;
        pub fn copy_from(src: *mut TESRegionData, copy: bool)
    }

    crate::virtual_method! {
        pub const VFUNC_IS_LOADED: usize = 0x07;
        pub fn is_loaded() -> bool
    }
}
