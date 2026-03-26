use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_JumpHandler;
use crate::offsets::offsets_vtable::VTABLE_JumpHandler;
use crate::re::PlayerInputHandler;
use crate::relocation::{RttiType, VariantID};

#[repr(C)]
pub struct JumpHandler {
    pub base: PlayerInputHandler,
}

const _: () = assert!(core::mem::size_of::<JumpHandler>() == 0x10);
const _: () = assert!(core::mem::offset_of!(JumpHandler, base) == 0x00);

impl RttiType for JumpHandler {
    const RTTI: VariantID = RTTI_JumpHandler;
}

inherit!(JumpHandler : PlayerInputHandler, base);

impl JumpHandler {
    pub const RTTI: VariantID = RTTI_JumpHandler;
    pub const VTABLE: &'static [VariantID] = &VTABLE_JumpHandler;
}
