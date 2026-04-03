use crate::re::{
    ActiveEffect, Actor, EffectSetting, MagicItem,
    magic_system::{CannotCastReason, CastingSource},
};
use crate::sdk::core::GamePtr;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ImmediateCastOptions {
    pub no_hit_effect_art: bool,
    pub effectiveness: f32,
    pub hostile_effectiveness_only: bool,
    pub magnitude_override: f32,
    pub blame_actor: GamePtr<Actor>,
}

impl Default for ImmediateCastOptions {
    fn default() -> Self {
        Self {
            no_hit_effect_art: false,
            effectiveness: 1.0,
            hostile_effectiveness_only: false,
            magnitude_override: 0.0,
            blame_actor: GamePtr::null(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CastCheck {
    pub can_cast: bool,
    pub reason: CannotCastReason,
    pub effect_strength: f32,
}

impl CastCheck {
    #[inline(always)]
    pub const fn success(self) -> bool {
        self.can_cast
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ActiveEffectSnapshot {
    pub effect: GamePtr<ActiveEffect>,
    pub spell: GamePtr<MagicItem>,
    pub base: GamePtr<EffectSetting>,
    pub magnitude: f32,
    pub duration: f32,
    pub elapsed_seconds: f32,
    pub casting_source: CastingSource,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CastingSourceSnapshot {
    pub source: CastingSource,
    pub selected_spell: GamePtr<MagicItem>,
    pub current_spell: GamePtr<MagicItem>,
    pub current_spell_cost: f32,
    pub dual_casting: bool,
    pub has_magic_caster: bool,
}
