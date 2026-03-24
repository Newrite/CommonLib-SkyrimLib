use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_ExtraAshPileRef;
use crate::offsets::offsets_vtable::VTABLE_ExtraAshPileRef;
use crate::re::{BSExtraData, ExtraDataType, ExtraDataTyped, ObjectRefHandle};
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::ExtraAshPileRef`
#[repr(C)]
pub struct ExtraAshPileRef {
    pub base: BSExtraData,             // 00
    pub ash_pile_ref: ObjectRefHandle, // 10
    pub pad14: u32,                    // 14
}

const _: () = assert!(core::mem::size_of::<ExtraAshPileRef>() == 0x18);
const _: () = assert!(core::mem::offset_of!(ExtraAshPileRef, ash_pile_ref) == 0x10);

impl RttiType for ExtraAshPileRef {
    const RTTI: VariantID = RTTI_ExtraAshPileRef;
}

impl ExtraDataTyped for ExtraAshPileRef {
    const EXTRADATATYPE: ExtraDataType = ExtraDataType::AshPileRef;
}

inherit!(ExtraAshPileRef : BSExtraData);

impl ExtraAshPileRef {
    pub const RTTI: VariantID = RTTI_ExtraAshPileRef;
    pub const VTABLE: &'static [VariantID] = &VTABLE_ExtraAshPileRef;
    pub const EXTRADATATYPE: ExtraDataType = ExtraDataType::AshPileRef;

    // override (BSExtraData)
    // ExtraDataType GetType() const override;  // 01 - { return kAshPileRef; }

    #[inline(always)]
    pub fn new(ash_pile_ref: ObjectRefHandle) -> Self {
        Self {
            base: BSExtraData {
                vtable: Self::VTABLE[0].address() as *const usize,
                next: core::ptr::null_mut(),
            },
            ash_pile_ref,
            pad14: 0,
        }
    }
}
