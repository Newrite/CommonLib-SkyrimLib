use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_ReadyWeaponHandler;
use crate::offsets::offsets_vtable::VTABLE_ReadyWeaponHandler;
use crate::re::PlayerInputHandler;
use crate::relocation::{RttiType, VariantID};

#[repr(C)]
pub struct ReadyWeaponHandler {
    pub base: PlayerInputHandler,
}

const _: () = assert!(core::mem::size_of::<ReadyWeaponHandler>() == 0x10);
const _: () = assert!(core::mem::offset_of!(ReadyWeaponHandler, base) == 0x00);

impl RttiType for ReadyWeaponHandler {
    const RTTI: VariantID = RTTI_ReadyWeaponHandler;
}

inherit!(ReadyWeaponHandler : PlayerInputHandler, base);

impl ReadyWeaponHandler {
    pub const RTTI: VariantID = RTTI_ReadyWeaponHandler;
    pub const VTABLE: &'static [VariantID] = &VTABLE_ReadyWeaponHandler;
}
