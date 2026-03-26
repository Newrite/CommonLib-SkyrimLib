use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_RefAttachTechniqueInput;
use crate::offsets::offsets_vtable::VTABLE_RefAttachTechniqueInput;
use crate::re::{AttachTechniqueInput, BSFixedString, TESObjectREFR, TESRace, bhkWorld};
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::RefAttachTechniqueInput`
#[repr(C)]
pub struct RefAttachTechniqueInput {
    pub base: AttachTechniqueInput,   // 00
    pub actor: *mut TESObjectREFR,    // 20
    pub actor_race: *mut TESRace,     // 28
    pub physics_world: *mut bhkWorld, // 30
    pub collision_filter: u32,        // 38
    pub unk3c: u32,                   // 3C
    pub node_name: BSFixedString,     // 40
}

const _: () = assert!(core::mem::size_of::<RefAttachTechniqueInput>() == 0x48);
const _: () = assert!(core::mem::offset_of!(RefAttachTechniqueInput, base) == 0x00);
const _: () = assert!(core::mem::offset_of!(RefAttachTechniqueInput, actor) == 0x20);
const _: () = assert!(core::mem::offset_of!(RefAttachTechniqueInput, actor_race) == 0x28);
const _: () = assert!(core::mem::offset_of!(RefAttachTechniqueInput, physics_world) == 0x30);
const _: () = assert!(core::mem::offset_of!(RefAttachTechniqueInput, collision_filter) == 0x38);
const _: () = assert!(core::mem::offset_of!(RefAttachTechniqueInput, node_name) == 0x40);

inherit!(RefAttachTechniqueInput : AttachTechniqueInput);

impl RttiType for RefAttachTechniqueInput {
    const RTTI: VariantID = RTTI_RefAttachTechniqueInput;
}

impl RefAttachTechniqueInput {
    pub const RTTI: VariantID = RTTI_RefAttachTechniqueInput;
    pub const VTABLE: &'static [VariantID] = &VTABLE_RefAttachTechniqueInput;

    // override (BSAttachTechniques::AttachTechniqueInput)
    // 00 ~RefAttachTechniqueInput
    // 01 Clear
}
