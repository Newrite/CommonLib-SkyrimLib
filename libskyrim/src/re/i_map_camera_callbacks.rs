use crate::offsets::offsets_rtti::RTTI_IMapCameraCallbacks;
use crate::offsets::offsets_vtable::VTABLE_IMapCameraCallbacks;
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::IMapCameraCallbacks`
#[repr(C)]
pub struct IMapCameraCallbacks {
    pub vtable: *const usize, // 00
}

const _: () = assert!(core::mem::size_of::<IMapCameraCallbacks>() == 0x8);

impl RttiType for IMapCameraCallbacks {
    const RTTI: VariantID = RTTI_IMapCameraCallbacks;
}

impl IMapCameraCallbacks {
    pub const RTTI: VariantID = RTTI_IMapCameraCallbacks;
    pub const VTABLE: &'static [VariantID] = &VTABLE_IMapCameraCallbacks;

    crate::virtual_method! {
        pub const VFUNC_UNK_00: usize = 0x00;
        pub fn unk_00()
    }

    crate::virtual_method! {
        pub const VFUNC_UNK_01: usize = 0x01;
        pub fn unk_01()
    }
}
