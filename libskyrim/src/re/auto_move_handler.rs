use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_AutoMoveHandler;
use crate::offsets::offsets_vtable::VTABLE_AutoMoveHandler;
use crate::re::PlayerInputHandler;
use crate::relocation::{RttiType, VariantID};

#[repr(C)]
pub struct AutoMoveHandler {
    pub base: PlayerInputHandler,
}

const _: () = assert!(core::mem::size_of::<AutoMoveHandler>() == 0x10);
const _: () = assert!(core::mem::offset_of!(AutoMoveHandler, base) == 0x00);

impl RttiType for AutoMoveHandler {
    const RTTI: VariantID = RTTI_AutoMoveHandler;
}

inherit!(AutoMoveHandler : PlayerInputHandler, base);

impl AutoMoveHandler {
    pub const RTTI: VariantID = RTTI_AutoMoveHandler;
    pub const VTABLE: &'static [VariantID] = &VTABLE_AutoMoveHandler;
}
