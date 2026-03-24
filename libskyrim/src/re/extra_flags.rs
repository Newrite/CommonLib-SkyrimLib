use core_util::{EnumSet, inherit};

use crate::offsets::offsets_rtti::RTTI_ExtraFlags;
use crate::offsets::offsets_vtable::VTABLE_ExtraFlags;
use crate::re::{BSExtraData, ExtraDataType, ExtraDataTyped};
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::ExtraFlags::Flag`
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExtraFlagsFlag {
    None = 0,
    BlockActivate = 1 << 0,
    BlockPlayerActivate = 1 << 1,
    BlockLoadEvents = 1 << 2,
    BlockActivateText = 1 << 3,
    PlayerHasTaken = 1 << 5,
}

core_util::impl_enumset_type!(ExtraFlagsFlag => u32);

/// C++ `RE::ExtraFlags`
#[repr(C)]
pub struct ExtraFlags {
    pub base: BSExtraData,                   // 00
    pub flags: EnumSet<ExtraFlagsFlag, u32>, // 10
    pub pad14: u32,                          // 14
}

const _: () = assert!(core::mem::size_of::<ExtraFlags>() == 0x18);
const _: () = assert!(core::mem::offset_of!(ExtraFlags, base) == 0x00);
const _: () = assert!(core::mem::offset_of!(ExtraFlags, flags) == 0x10);
const _: () = assert!(core::mem::offset_of!(ExtraFlags, pad14) == 0x14);

impl RttiType for ExtraFlags {
    const RTTI: VariantID = RTTI_ExtraFlags;
}

impl ExtraDataTyped for ExtraFlags {
    const EXTRADATATYPE: ExtraDataType = ExtraDataType::Flags;
}

inherit!(ExtraFlags : BSExtraData);

impl ExtraFlags {
    pub const RTTI: VariantID = RTTI_ExtraFlags;
    pub const VTABLE: &'static [VariantID] = &VTABLE_ExtraFlags;
    pub const EXTRADATATYPE: ExtraDataType = ExtraDataType::Flags;

    // override (BSExtraData)
    // ExtraDataType GetType() const override;  // 01 - { return kFlags; }

    #[inline(always)]
    pub fn is_activation_blocked(&self) -> bool {
        self.flags.all(ExtraFlagsFlag::BlockActivate)
    }
}
