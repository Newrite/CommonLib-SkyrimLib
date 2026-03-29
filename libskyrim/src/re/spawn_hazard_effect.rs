use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_SpawnHazardEffect;
use crate::offsets::offsets_vtable::VTABLE_SpawnHazardEffect;
use crate::re::{ActiveEffect, ActorCause, NiPointer, ObjectRefHandle};
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::SpawnHazardEffect`
#[repr(C)]
pub struct SpawnHazardEffect {
    pub base: ActiveEffect,                 // 00
    pub actor_cause: NiPointer<ActorCause>, // 90
    pub hazard: ObjectRefHandle,            // 98
    pub pad9c: u32,                         // 9C
}

const _: () = assert!(core::mem::size_of::<SpawnHazardEffect>() == 0xA0);
const _: () = assert!(core::mem::offset_of!(SpawnHazardEffect, base) == 0x00);
const _: () = assert!(core::mem::offset_of!(SpawnHazardEffect, actor_cause) == 0x90);
const _: () = assert!(core::mem::offset_of!(SpawnHazardEffect, hazard) == 0x98);

impl RttiType for SpawnHazardEffect {
    const RTTI: VariantID = RTTI_SpawnHazardEffect;
}

inherit!(SpawnHazardEffect : ActiveEffect);

impl SpawnHazardEffect {
    pub const RTTI: VariantID = RTTI_SpawnHazardEffect;
    pub const VTABLE: &'static [VariantID] = &VTABLE_SpawnHazardEffect;

    // override (ActiveEffect)
    // void HandleQueuedStart() override;  // 0F
    // ~SpawnHazardEffect() override;      // 13
    // void Start() override;              // 14
    // void Finish() override;             // 15
}
