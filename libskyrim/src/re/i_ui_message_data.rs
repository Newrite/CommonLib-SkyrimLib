use crate::offsets::offsets_rtti::RTTI_IUIMessageData;
use crate::offsets::offsets_vtable::VTABLE_IUIMessageData;
use crate::relocation::{RttiType, VariantID};
use crate::virtual_method;

/// C++ `RE::IUIMessageData`
#[repr(C)]
pub struct IUIMessageData {
    pub vtable: *const usize, // 00
    pub unk08: u16,           // 08
    pub pad0a: u16,           // 0A
    pub pad0c: u32,           // 0C
}

const _: () = assert!(core::mem::size_of::<IUIMessageData>() == 0x10);
const _: () = assert!(core::mem::offset_of!(IUIMessageData, vtable) == 0x0);
const _: () = assert!(core::mem::offset_of!(IUIMessageData, unk08) == 0x8);
const _: () = assert!(core::mem::offset_of!(IUIMessageData, pad0a) == 0xA);
const _: () = assert!(core::mem::offset_of!(IUIMessageData, pad0c) == 0xC);

impl RttiType for IUIMessageData {
    const RTTI: VariantID = RTTI_IUIMessageData;
}

impl IUIMessageData {
    pub const RTTI: VariantID = RTTI_IUIMessageData;
    pub const VTABLE: &'static [VariantID] = &VTABLE_IUIMessageData;

    virtual_method! {
        pub const VFUNC_DTOR: usize = 0x0;
        pub fn dtor()
    }
}
