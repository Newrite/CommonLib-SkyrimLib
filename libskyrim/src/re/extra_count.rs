use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_ExtraCount;
use crate::offsets::offsets_vtable::VTABLE_ExtraCount;
use crate::re::{BSExtraData, ExtraDataType, ExtraDataTyped};
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::ExtraCount`
#[repr(C)]
pub struct ExtraCount {
    pub base: BSExtraData, // 00
    pub count: i16,        // 10
    pub pad12: u16,        // 12
    pub pad14: i32,        // 14
}

const _: () = assert!(core::mem::size_of::<ExtraCount>() == 0x18);
const _: () = assert!(core::mem::offset_of!(ExtraCount, count) == 0x10);

impl RttiType for ExtraCount {
    const RTTI: VariantID = RTTI_ExtraCount;
}

impl ExtraDataTyped for ExtraCount {
    const EXTRADATATYPE: ExtraDataType = ExtraDataType::Count;
}

inherit!(ExtraCount : BSExtraData);

impl ExtraCount {
    pub const RTTI: VariantID = RTTI_ExtraCount;
    pub const VTABLE: &'static [VariantID] = &VTABLE_ExtraCount;
    pub const EXTRADATATYPE: ExtraDataType = ExtraDataType::Count;

    // override (BSExtraData)
    // ExtraDataType GetType() const override;  // 01 - { return kCount; }
    // bool IsNotEqual(const BSExtraData* a_rhs) const override;  // 02

    #[inline(always)]
    pub fn new(count: i16) -> Self {
        Self {
            base: BSExtraData {
                vtable: Self::VTABLE[0].address() as *const usize,
                next: core::ptr::null_mut(),
            },
            count,
            pad12: 0,
            pad14: 0,
        }
    }

    #[inline(always)]
    pub fn is_not_equal_impl(&self, rhs: *const BSExtraData) -> bool {
        let rhs = rhs.cast::<Self>();
        rhs.is_null() || self.count != unsafe { (*rhs).count }
    }
}
