use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_KinectStateChangeData;
use crate::offsets::offsets_vtable::VTABLE_KinectStateChangeData;
use crate::re::{BSFixedString, IUIMessageData};
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::KinectStateChangeData`
#[repr(C)]
pub struct KinectStateChangeData {
    pub base: IUIMessageData, // 00
    pub unk10: u64,           // 10
    pub unk18: BSFixedString, // 18
}

const _: () = assert!(core::mem::size_of::<KinectStateChangeData>() == 0x20);
const _: () = assert!(core::mem::offset_of!(KinectStateChangeData, base) == 0x00);
const _: () = assert!(core::mem::offset_of!(KinectStateChangeData, unk10) == 0x10);
const _: () = assert!(core::mem::offset_of!(KinectStateChangeData, unk18) == 0x18);

inherit!(KinectStateChangeData : IUIMessageData);

impl RttiType for KinectStateChangeData {
    const RTTI: VariantID = RTTI_KinectStateChangeData;
}

impl KinectStateChangeData {
    pub const RTTI: VariantID = RTTI_KinectStateChangeData;
    pub const VTABLE: &'static [VariantID] = &VTABLE_KinectStateChangeData;
    pub const CLASS_NAME: &'static str = "KinectStateChangeData";

    // override (IUIMessageData)
    // ~KinectStateChangeData() override; // 00
}
