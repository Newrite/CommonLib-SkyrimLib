use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_ExtraHealth;
use crate::offsets::offsets_vtable::VTABLE_ExtraHealth;
use crate::re::{BSExtraData, ExtraDataType, ExtraDataTyped};
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::ExtraHealth`
#[repr(C)]
pub struct ExtraHealth {
    pub base: BSExtraData, // 00
    pub health: f32,       // 10
    pub pad14: u32,        // 14
}

const _: () = assert!(core::mem::size_of::<ExtraHealth>() == 0x18);
const _: () = assert!(core::mem::offset_of!(ExtraHealth, health) == 0x10);

impl RttiType for ExtraHealth {
    const RTTI: VariantID = RTTI_ExtraHealth;
}

impl ExtraDataTyped for ExtraHealth {
    const EXTRADATATYPE: ExtraDataType = ExtraDataType::Health;
}

inherit!(ExtraHealth : BSExtraData);

impl ExtraHealth {
    pub const RTTI: VariantID = RTTI_ExtraHealth;
    pub const VTABLE: &'static [VariantID] = &VTABLE_ExtraHealth;
    pub const EXTRADATATYPE: ExtraDataType = ExtraDataType::Health;

    // override (BSExtraData)
    // ExtraDataType GetType() const override;  // 01 - { return kHealth; }
    // bool IsNotEqual(const BSExtraData* a_rhs) const override;  // 02

    #[inline(always)]
    pub fn new(health: f32) -> Self {
        Self {
            base: BSExtraData {
                vtable: Self::VTABLE[0].address() as *const usize,
                next: core::ptr::null_mut(),
            },
            health,
            pad14: 0,
        }
    }

    #[inline(always)]
    pub fn is_not_equal_impl(&self, rhs: *const BSExtraData) -> bool {
        let rhs = rhs.cast::<Self>();
        rhs.is_null() || self.health != unsafe { (*rhs).health }
    }
}
