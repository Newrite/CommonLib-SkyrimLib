use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_MovementHandler;
use crate::offsets::offsets_vtable::VTABLE_MovementHandler;
use crate::re::PlayerInputHandler;
use crate::relocation::{RttiType, VariantID};

#[repr(C)]
pub struct MovementHandler {
    pub base: PlayerInputHandler,
}

const _: () = assert!(core::mem::size_of::<MovementHandler>() == 0x10);
const _: () = assert!(core::mem::offset_of!(MovementHandler, base) == 0x00);

impl RttiType for MovementHandler {
    const RTTI: VariantID = RTTI_MovementHandler;
}

inherit!(MovementHandler : PlayerInputHandler, base);

impl MovementHandler {
    pub const RTTI: VariantID = RTTI_MovementHandler;
    pub const VTABLE: &'static [VariantID] = &VTABLE_MovementHandler;
}
