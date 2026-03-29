use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_TargetValueModifierEffect;
use crate::offsets::offsets_vtable::VTABLE_TargetValueModifierEffect;
use crate::re::ValueModifierEffect;
use crate::relocation::{RttiType, VariantID};
use crate::virtual_method;

/// C++ `RE::TargetValueModifierEffect`
#[repr(C)]
pub struct TargetValueModifierEffect {
    pub base: ValueModifierEffect, // 00
}

const _: () = assert!(core::mem::size_of::<TargetValueModifierEffect>() == 0x98);
const _: () = assert!(core::mem::offset_of!(TargetValueModifierEffect, base) == 0x00);

impl RttiType for TargetValueModifierEffect {
    const RTTI: VariantID = RTTI_TargetValueModifierEffect;
}

impl AsRef<TargetValueModifierEffect> for TargetValueModifierEffect {
    #[inline(always)]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsMut<TargetValueModifierEffect> for TargetValueModifierEffect {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

inherit!(TargetValueModifierEffect : ValueModifierEffect);

impl TargetValueModifierEffect {
    pub const RTTI: VariantID = RTTI_TargetValueModifierEffect;
    pub const VTABLE: &'static [VariantID] = &VTABLE_TargetValueModifierEffect;

    // override (ValueModifierEffect)
    // bool GetAllowMultipleCastingSourceStacking() override;  // 11 - { return 0; }
    // ~TargetValueModifierEffect() override;                  // 13
    // void Start() override;                                  // 14

    virtual_method! {
        pub const VFUNC_GET_TARGET_VALUE: usize = 0x21;
        pub fn get_target_value() -> f32
    }
}

pub trait TargetValueModifierEffectExt {
    fn get_target_value(&self) -> f32;
}

impl<T: AsRef<TargetValueModifierEffect> + AsMut<TargetValueModifierEffect>>
    TargetValueModifierEffectExt for T
{
    fn get_target_value(&self) -> f32 {
        self.as_ref().get_target_value()
    }
}
