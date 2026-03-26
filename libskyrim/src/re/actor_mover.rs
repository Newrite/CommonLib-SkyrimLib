use crate::offsets::offsets_rtti::RTTI_ActorMover;
use crate::offsets::offsets_vtable::VTABLE_ActorMover;
use crate::re::Actor;
use crate::relocation::{RttiType, VariantID};

/// Minimal C++ `RE::ActorMover`.
#[repr(C)]
pub struct ActorMover {
    pub vtable: *const usize, // 00
    pub actor: *mut Actor,    // 08
}

const _: () = assert!(core::mem::size_of::<ActorMover>() == 0x10);
const _: () = assert!(core::mem::offset_of!(ActorMover, vtable) == 0x00);
const _: () = assert!(core::mem::offset_of!(ActorMover, actor) == 0x08);

impl RttiType for ActorMover {
    const RTTI: VariantID = RTTI_ActorMover;
}

impl ActorMover {
    pub const RTTI: VariantID = RTTI_ActorMover;
    pub const VTABLE: &'static [VariantID] = &VTABLE_ActorMover;
}
