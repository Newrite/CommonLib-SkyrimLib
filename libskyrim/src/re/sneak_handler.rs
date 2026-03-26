use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_SneakHandler;
use crate::offsets::offsets_vtable::VTABLE_SneakHandler;
use crate::re::PlayerInputHandler;
use crate::relocation::{RttiType, VariantID};

#[repr(C)]
pub struct SneakHandler {
    pub base: PlayerInputHandler,
}

const _: () = assert!(core::mem::size_of::<SneakHandler>() == 0x10);
const _: () = assert!(core::mem::offset_of!(SneakHandler, base) == 0x00);

impl RttiType for SneakHandler {
    const RTTI: VariantID = RTTI_SneakHandler;
}

inherit!(SneakHandler : PlayerInputHandler, base);

impl SneakHandler {
    pub const RTTI: VariantID = RTTI_SneakHandler;
    pub const VTABLE: &'static [VariantID] = &VTABLE_SneakHandler;
}
