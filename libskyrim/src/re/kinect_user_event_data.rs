use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_KinectUserEventData;
use crate::offsets::offsets_vtable::VTABLE_KinectUserEventData;
use crate::re::{BSFixedString, IUIMessageData};
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::KinectUserEventData`
#[repr(C)]
pub struct KinectUserEventData {
    pub base: IUIMessageData, // 00
    pub unk10: BSFixedString, // 10
    pub unk18: BSFixedString, // 18
}

const _: () = assert!(core::mem::size_of::<KinectUserEventData>() == 0x20);
const _: () = assert!(core::mem::offset_of!(KinectUserEventData, base) == 0x00);
const _: () = assert!(core::mem::offset_of!(KinectUserEventData, unk10) == 0x10);
const _: () = assert!(core::mem::offset_of!(KinectUserEventData, unk18) == 0x18);

inherit!(KinectUserEventData : IUIMessageData);

impl RttiType for KinectUserEventData {
    const RTTI: VariantID = RTTI_KinectUserEventData;
}

impl KinectUserEventData {
    pub const RTTI: VariantID = RTTI_KinectUserEventData;
    pub const VTABLE: &'static [VariantID] = &VTABLE_KinectUserEventData;
    pub const CLASS_NAME: &'static str = "KinectUserEventData";

    // override (IUIMessageData)
    // ~KinectUserEventData() override; // 00
}
