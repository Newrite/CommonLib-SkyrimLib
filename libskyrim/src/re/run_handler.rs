use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_RunHandler;
use crate::offsets::offsets_vtable::VTABLE_RunHandler;
use crate::re::HeldStateHandler;
use crate::relocation::{RttiType, VariantID};

#[repr(C)]
pub struct RunHandler {
    pub base: HeldStateHandler,
}

const _: () = assert!(core::mem::size_of::<RunHandler>() == 0x10);
const _: () = assert!(core::mem::offset_of!(RunHandler, base) == 0x00);

impl RttiType for RunHandler {
    const RTTI: VariantID = RTTI_RunHandler;
}

inherit!(RunHandler : HeldStateHandler, base);

impl RunHandler {
    pub const RTTI: VariantID = RTTI_RunHandler;
    pub const VTABLE: &'static [VariantID] = &VTABLE_RunHandler;
}
