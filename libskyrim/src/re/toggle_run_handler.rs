use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_ToggleRunHandler;
use crate::offsets::offsets_vtable::VTABLE_ToggleRunHandler;
use crate::re::PlayerInputHandler;
use crate::relocation::{RttiType, VariantID};

#[repr(C)]
pub struct ToggleRunHandler {
    pub base: PlayerInputHandler,
}

const _: () = assert!(core::mem::size_of::<ToggleRunHandler>() == 0x10);
const _: () = assert!(core::mem::offset_of!(ToggleRunHandler, base) == 0x00);

impl RttiType for ToggleRunHandler {
    const RTTI: VariantID = RTTI_ToggleRunHandler;
}

inherit!(ToggleRunHandler : PlayerInputHandler, base);

impl ToggleRunHandler {
    pub const RTTI: VariantID = RTTI_ToggleRunHandler;
    pub const VTABLE: &'static [VariantID] = &VTABLE_ToggleRunHandler;
}
