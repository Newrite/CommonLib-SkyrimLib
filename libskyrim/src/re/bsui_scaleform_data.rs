use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_BSUIScaleformData;
use crate::offsets::offsets_vtable::VTABLE_BSUIScaleformData;
use crate::re::{GFxEvent, IUIMessageData};
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::BSUIScaleformData`
#[repr(C)]
pub struct BSUIScaleformData {
    pub base: IUIMessageData,           // 00
    pub scaleform_event: *mut GFxEvent, // 10
}

const _: () = assert!(core::mem::size_of::<BSUIScaleformData>() == 0x18);
const _: () = assert!(core::mem::offset_of!(BSUIScaleformData, base) == 0x00);
const _: () = assert!(core::mem::offset_of!(BSUIScaleformData, scaleform_event) == 0x10);

inherit!(BSUIScaleformData : IUIMessageData);

impl RttiType for BSUIScaleformData {
    const RTTI: VariantID = RTTI_BSUIScaleformData;
}

impl BSUIScaleformData {
    pub const RTTI: VariantID = RTTI_BSUIScaleformData;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BSUIScaleformData;
    pub const CLASS_NAME: &'static str = "BSUIScaleformData";

    // override (IUIMessageData)
    // ~BSUIScaleformData() override; // 00
}
