use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_ConcussionEffect;
use crate::offsets::offsets_vtable::VTABLE_ConcussionEffect;
use crate::re::ActiveEffect;
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::ConcussionEffect`
#[repr(C)]
pub struct ConcussionEffect {
    pub base: ActiveEffect, // 00
}

const _: () = assert!(core::mem::size_of::<ConcussionEffect>() == 0x90);
const _: () = assert!(core::mem::offset_of!(ConcussionEffect, base) == 0x00);

impl RttiType for ConcussionEffect {
    const RTTI: VariantID = RTTI_ConcussionEffect;
}

inherit!(ConcussionEffect : ActiveEffect);

impl ConcussionEffect {
    pub const RTTI: VariantID = RTTI_ConcussionEffect;
    pub const VTABLE: &'static [VariantID] = &VTABLE_ConcussionEffect;

    // override (ActiveEffect)
    // ~ConcussionEffect() override;  // 13
}
