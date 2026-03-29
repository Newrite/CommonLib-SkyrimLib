use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_PeakValueModifierEffect;
use crate::offsets::offsets_vtable::VTABLE_PeakValueModifierEffect;
use crate::re::{ActiveEffect, ValueModifierEffect};
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::PeakValueModifierEffect`
#[repr(C)]
pub struct PeakValueModifierEffect {
    pub base: ValueModifierEffect, // 00
    // TODO: replace this raw pointer stand-in with the real `BSTSmartPointer<ActiveEffect>`
    // once `ActiveEffect`'s source-backed intrusive refcount/delete contract is translated;
    // intended end state: the exact smart-pointer field from `PeakValueModifierEffect.h`.
    pub next: *mut ActiveEffect, // 98 - BSTSmartPointer<ActiveEffect>
}

const _: () = assert!(core::mem::size_of::<PeakValueModifierEffect>() == 0xA0);
const _: () = assert!(core::mem::offset_of!(PeakValueModifierEffect, base) == 0x00);
const _: () = assert!(core::mem::offset_of!(PeakValueModifierEffect, next) == 0x98);

impl RttiType for PeakValueModifierEffect {
    const RTTI: VariantID = RTTI_PeakValueModifierEffect;
}

inherit!(PeakValueModifierEffect : ValueModifierEffect);

impl PeakValueModifierEffect {
    pub const RTTI: VariantID = RTTI_PeakValueModifierEffect;
    pub const VTABLE: &'static [VariantID] = &VTABLE_PeakValueModifierEffect;

    // override (ActiveEffect)
    // void OnAdd(MagicTarget*) override;                          // 01
    // void OnRemove() override;                                   // 02
    // void EvaluateConditions(float, bool) override;              // 05
    // void FinishLoadGame(BGSLoadFormBuffer*) override;           // 0A
    // std::int32_t Compare(ActiveEffect*) override;               // 0C
    // void ClearTargetImpl() override;                            // 12
    // ~PeakValueModifierEffect() override;                        // 13
}
