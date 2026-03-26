use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_LookHandler;
use crate::offsets::offsets_vtable::VTABLE_LookHandler;
use crate::re::PlayerInputHandler;
use crate::relocation::{RttiType, VariantID};

#[repr(C)]
pub struct LookHandler {
    pub base: PlayerInputHandler,
}

const _: () = assert!(core::mem::size_of::<LookHandler>() == 0x10);
const _: () = assert!(core::mem::offset_of!(LookHandler, base) == 0x00);

impl RttiType for LookHandler {
    const RTTI: VariantID = RTTI_LookHandler;
}

inherit!(LookHandler : PlayerInputHandler, base);

impl LookHandler {
    pub const RTTI: VariantID = RTTI_LookHandler;
    pub const VTABLE: &'static [VariantID] = &VTABLE_LookHandler;
}
