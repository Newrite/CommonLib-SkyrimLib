use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_StaggerEffect;
use crate::offsets::offsets_vtable::VTABLE_StaggerEffect;
use crate::re::ActiveEffect;
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::StaggerEffect`
#[repr(C)]
pub struct StaggerEffect {
    pub base: ActiveEffect, // 00
}

const _: () = assert!(core::mem::size_of::<StaggerEffect>() == 0x90);
const _: () = assert!(core::mem::offset_of!(StaggerEffect, base) == 0x00);

impl RttiType for StaggerEffect {
    const RTTI: VariantID = RTTI_StaggerEffect;
}

inherit!(StaggerEffect : ActiveEffect);

impl StaggerEffect {
    pub const RTTI: VariantID = RTTI_StaggerEffect;
    pub const VTABLE: &'static [VariantID] = &VTABLE_StaggerEffect;

    // override (ActiveEffect)
    // ~StaggerEffect() override;  // 13
    // void Start() override;      // 14
}
