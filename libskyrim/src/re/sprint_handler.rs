use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_SprintHandler;
use crate::offsets::offsets_vtable::VTABLE_SprintHandler;
use crate::re::HeldStateHandler;
use crate::relocation::{RttiType, VariantID};

#[repr(C)]
pub struct SprintHandler {
    pub base: HeldStateHandler,
}

const _: () = assert!(core::mem::size_of::<SprintHandler>() == 0x10);
const _: () = assert!(core::mem::offset_of!(SprintHandler, base) == 0x00);

impl RttiType for SprintHandler {
    const RTTI: VariantID = RTTI_SprintHandler;
}

inherit!(SprintHandler : HeldStateHandler, base);

impl SprintHandler {
    pub const RTTI: VariantID = RTTI_SprintHandler;
    pub const VTABLE: &'static [VariantID] = &VTABLE_SprintHandler;
}
