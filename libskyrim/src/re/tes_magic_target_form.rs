use crate::offsets::offsets_rtti::RTTI_TESMagicTargetForm;
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::TESMagicTargetForm`
///
/// RTTI-only marker mixin with no vtable.
#[repr(C)]
pub struct TESMagicTargetForm {
    pub marker: u8,
}

const _: () = assert!(core::mem::size_of::<TESMagicTargetForm>() == 0x1);

impl RttiType for TESMagicTargetForm {
    const RTTI: VariantID = RTTI_TESMagicTargetForm;
}

impl TESMagicTargetForm {
    pub const RTTI: VariantID = RTTI_TESMagicTargetForm;
}
