use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_ValueModifierEffect;
use crate::offsets::offsets_vtable::VTABLE_ValueModifierEffect;
use crate::re::{ActiveEffect, Actor, ActorValue, MagicTarget};
use crate::relocation::{RttiType, VariantID};
use crate::virtual_method;

/// C++ `RE::ValueModifierEffect`
#[repr(C)]
pub struct ValueModifierEffect {
    pub base: ActiveEffect,      // 00
    pub actor_value: ActorValue, // 90
    pub value: f32,              // 94
}

const _: () = assert!(core::mem::size_of::<ValueModifierEffect>() == 0x98);
const _: () = assert!(core::mem::offset_of!(ValueModifierEffect, base) == 0x00);
const _: () = assert!(core::mem::offset_of!(ValueModifierEffect, actor_value) == 0x90);
const _: () = assert!(core::mem::offset_of!(ValueModifierEffect, value) == 0x94);

impl RttiType for ValueModifierEffect {
    const RTTI: VariantID = RTTI_ValueModifierEffect;
}

impl AsRef<ValueModifierEffect> for ValueModifierEffect {
    #[inline(always)]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsMut<ValueModifierEffect> for ValueModifierEffect {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

inherit!(ValueModifierEffect : ActiveEffect);

impl ValueModifierEffect {
    pub const RTTI: VariantID = RTTI_ValueModifierEffect;
    pub const VTABLE: &'static [VariantID] = &VTABLE_ValueModifierEffect;

    // override (ActiveEffect)
    // void OnAdd(MagicTarget* a_target) override;             // 01
    // void Update(float a_delta) override;                    // 04
    // bool IsCausingHealthDamage() override;                  // 06
    // bool GetAllowMultipleCastingSourceStacking() override;  // 11 - { return 1; }
    // void ClearTargetImpl() override;                        // 12
    // ~ValueModifierEffect() override;                        // 13
    // void Start() override;                                  // 14
    // void Finish() override;                                 // 15
    // bool CheckCustomSkillUseConditions() const override;    // 17
    // float GetCustomSkillUseMagnitudeMultiplier(float) const override;  // 18

    virtual_method! {
        pub const VFUNC_SET_ACTOR_VALUE: usize = 0x19;
        pub fn set_actor_value(actor_value: ActorValue)
    }

    virtual_method! {
        pub const VFUNC_SHOULD_MODIFY_ON_START: usize = 0x1A;
        pub fn should_modify_on_start() -> bool
    }

    virtual_method! {
        pub const VFUNC_MODIFY_ON_START: usize = 0x1B;
        pub fn modify_on_start()
    }

    virtual_method! {
        pub const VFUNC_SHOULD_MODIFY_ON_UPDATE: usize = 0x1C;
        pub fn should_modify_on_update() -> bool
    }

    virtual_method! {
        pub const VFUNC_MODIFY_ON_UPDATE: usize = 0x1D;
        pub fn modify_on_update(delta: f32)
    }

    virtual_method! {
        pub const VFUNC_SHOULD_MODIFY_ON_FINISH: usize = 0x1E;
        pub fn should_modify_on_finish() -> bool
    }

    virtual_method! {
        pub const VFUNC_MODIFY_ON_FINISH: usize = 0x1F;
        pub fn modify_on_finish(caster: *mut Actor, target: *mut Actor, value: f32)
    }

    virtual_method! {
        pub const VFUNC_MODIFY_ACTOR_VALUE: usize = 0x20;
        pub fn modify_actor_value(actor: *mut Actor, value: f32, actor_value: ActorValue)
    }
}

pub trait ValueModifierEffectExt {
    fn on_add(&mut self, target: *mut MagicTarget);
    fn update(&mut self, delta: f32);
    fn is_causing_health_damage(&self) -> bool;
    fn get_allow_multiple_casting_source_stacking(&self) -> bool;
    fn clear_target_impl(&mut self);
    fn start(&mut self);
    fn finish(&mut self);
    fn check_custom_skill_use_conditions(&self) -> bool;
    fn get_custom_skill_use_magnitude_multiplier(&self, mult: f32) -> f32;
    fn set_actor_value(&mut self, actor_value: ActorValue);
    fn should_modify_on_start(&mut self) -> bool;
    fn modify_on_start(&mut self);
    fn should_modify_on_update(&self) -> bool;
    fn modify_on_update(&mut self, delta: f32);
    fn should_modify_on_finish(&self) -> bool;
    fn modify_on_finish(&mut self, caster: *mut Actor, target: *mut Actor, value: f32);
    fn modify_actor_value(&mut self, actor: *mut Actor, value: f32, actor_value: ActorValue);
}

impl<T: AsRef<ValueModifierEffect> + AsMut<ValueModifierEffect>> ValueModifierEffectExt for T {
    fn on_add(&mut self, target: *mut MagicTarget) {
        ActiveEffect::on_add(self.as_mut().as_mut(), target)
    }

    fn update(&mut self, delta: f32) {
        ActiveEffect::update(self.as_mut().as_mut(), delta)
    }

    fn is_causing_health_damage(&self) -> bool {
        ActiveEffect::is_causing_health_damage(AsRef::<ActiveEffect>::as_ref(self.as_ref()))
    }

    fn get_allow_multiple_casting_source_stacking(&self) -> bool {
        ActiveEffect::get_allow_multiple_casting_source_stacking(AsRef::<ActiveEffect>::as_ref(
            self.as_ref(),
        ))
    }

    fn clear_target_impl(&mut self) {
        ActiveEffect::clear_target_impl(self.as_mut().as_mut())
    }

    fn start(&mut self) {
        ActiveEffect::start(self.as_mut().as_mut())
    }

    fn finish(&mut self) {
        ActiveEffect::finish(self.as_mut().as_mut())
    }

    fn check_custom_skill_use_conditions(&self) -> bool {
        ActiveEffect::check_custom_skill_use_conditions(AsRef::<ActiveEffect>::as_ref(
            self.as_ref(),
        ))
    }

    fn get_custom_skill_use_magnitude_multiplier(&self, mult: f32) -> f32 {
        ActiveEffect::get_custom_skill_use_magnitude_multiplier(
            AsRef::<ActiveEffect>::as_ref(self.as_ref()),
            mult,
        )
    }

    fn set_actor_value(&mut self, actor_value: ActorValue) {
        ValueModifierEffect::set_actor_value(self.as_mut(), actor_value)
    }

    fn should_modify_on_start(&mut self) -> bool {
        ValueModifierEffect::should_modify_on_start(self.as_mut())
    }

    fn modify_on_start(&mut self) {
        ValueModifierEffect::modify_on_start(self.as_mut())
    }

    fn should_modify_on_update(&self) -> bool {
        self.as_ref().should_modify_on_update()
    }

    fn modify_on_update(&mut self, delta: f32) {
        ValueModifierEffect::modify_on_update(self.as_mut(), delta)
    }

    fn should_modify_on_finish(&self) -> bool {
        self.as_ref().should_modify_on_finish()
    }

    fn modify_on_finish(&mut self, caster: *mut Actor, target: *mut Actor, value: f32) {
        ValueModifierEffect::modify_on_finish(self.as_mut(), caster, target, value)
    }

    fn modify_actor_value(&mut self, actor: *mut Actor, value: f32, actor_value: ActorValue) {
        ValueModifierEffect::modify_actor_value(self.as_mut(), actor, value, actor_value)
    }
}
