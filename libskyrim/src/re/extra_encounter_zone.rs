use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_ExtraEncounterZone;
use crate::offsets::offsets_vtable::VTABLE_ExtraEncounterZone;
use crate::re::{BGSEncounterZone, BSExtraData, ExtraDataType, ExtraDataTyped};
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::ExtraEncounterZone`
#[repr(C)]
pub struct ExtraEncounterZone {
    pub base: BSExtraData,           // 00
    pub zone: *mut BGSEncounterZone, // 10
}

const _: () = assert!(core::mem::size_of::<ExtraEncounterZone>() == 0x18);
const _: () = assert!(core::mem::offset_of!(ExtraEncounterZone, zone) == 0x10);

impl RttiType for ExtraEncounterZone {
    const RTTI: VariantID = RTTI_ExtraEncounterZone;
}

impl ExtraDataTyped for ExtraEncounterZone {
    const EXTRADATATYPE: ExtraDataType = ExtraDataType::EncounterZone;
}

inherit!(ExtraEncounterZone : BSExtraData);

impl ExtraEncounterZone {
    pub const RTTI: VariantID = RTTI_ExtraEncounterZone;
    pub const VTABLE: &'static [VariantID] = &VTABLE_ExtraEncounterZone;
    pub const EXTRADATATYPE: ExtraDataType = ExtraDataType::EncounterZone;

    // override (BSExtraData)
    // ExtraDataType GetType() const override;  // 01 - { return kEncounterZone; }
    // bool IsNotEqual(const BSExtraData* a_rhs) const override;  // 02

    #[inline(always)]
    pub fn new(zone: *mut BGSEncounterZone) -> Self {
        Self {
            base: BSExtraData {
                vtable: Self::VTABLE[0].address() as *const usize,
                next: core::ptr::null_mut(),
            },
            zone,
        }
    }

    #[inline(always)]
    pub fn is_not_equal_impl(&self, rhs: *const BSExtraData) -> bool {
        let rhs = rhs.cast::<Self>();
        rhs.is_null() || self.zone != unsafe { (*rhs).zone }
    }
}
