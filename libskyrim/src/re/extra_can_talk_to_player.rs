use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_ExtraCanTalkToPlayer;
use crate::offsets::offsets_vtable::VTABLE_ExtraCanTalkToPlayer;
use crate::re::{BSExtraData, ExtraDataType, ExtraDataTyped};
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::ExtraCanTalkToPlayer`
#[repr(C)]
pub struct ExtraCanTalkToPlayer {
    pub base: BSExtraData, // 00
    pub talk: bool,        // 10
    pub pad11: u8,         // 11
    pub pad12: u16,        // 12
    pub pad14: u32,        // 14
}

const _: () = assert!(core::mem::size_of::<ExtraCanTalkToPlayer>() == 0x18);
const _: () = assert!(core::mem::offset_of!(ExtraCanTalkToPlayer, talk) == 0x10);

impl RttiType for ExtraCanTalkToPlayer {
    const RTTI: VariantID = RTTI_ExtraCanTalkToPlayer;
}

impl ExtraDataTyped for ExtraCanTalkToPlayer {
    const EXTRADATATYPE: ExtraDataType = ExtraDataType::CanTalkToPlayer;
}

inherit!(ExtraCanTalkToPlayer : BSExtraData);

impl Default for ExtraCanTalkToPlayer {
    fn default() -> Self {
        Self::new(false)
    }
}

impl ExtraCanTalkToPlayer {
    pub const RTTI: VariantID = RTTI_ExtraCanTalkToPlayer;
    pub const VTABLE: &'static [VariantID] = &VTABLE_ExtraCanTalkToPlayer;
    pub const EXTRADATATYPE: ExtraDataType = ExtraDataType::CanTalkToPlayer;

    #[inline(always)]
    pub fn new(can_talk: bool) -> Self {
        Self {
            base: BSExtraData {
                vtable: Self::VTABLE[0].address() as *const usize,
                next: core::ptr::null_mut(),
            },
            talk: can_talk,
            pad11: 0,
            pad12: 0,
            pad14: 0,
        }
    }
}
