use crate::re::{
    ActiveEffect, Actor, EffectSetting, MagicItem,
    magic_system::{CannotCastReason, CastingSource},
};
use crate::sdk::core::GamePtr;

/// Options for immediate-cast helper entrypoints.
///
/// These are the knobs that source-backed immediate cast helpers expose most
/// often: art suppression, effectiveness scaling, optional blame actor, and
/// one-shot magnitude overrides.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ImmediateCastOptions {
    /// Suppress hit-effect art during the immediate cast.
    pub no_hit_effect_art: bool,
    /// Global effectiveness multiplier applied to the cast.
    pub effectiveness: f32,
    /// Restrict the effectiveness multiplier to hostile effects only.
    pub hostile_effectiveness_only: bool,
    /// Optional magnitude override used by source-backed cast entry points.
    pub magnitude_override: f32,
    /// Optional actor blamed for the cast.
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

/// Result of a castability check.
///
/// This is the lightweight diagnostic returned by `caster` helpers when a
/// plugin wants both the boolean outcome and the engine-reported failure
/// reason.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CastCheck {
    /// Whether the cast can proceed.
    pub can_cast: bool,
    /// Engine-reported failure reason when the cast is blocked.
    pub reason: CannotCastReason,
    /// Effective strength that would be used for the cast.
    pub effect_strength: f32,
}

impl CastCheck {
    /// Convenience synonym for [`Self::can_cast`].
    #[inline(always)]
    pub const fn success(self) -> bool {
        self.can_cast
    }
}

/// Snapshot of one live active magic effect.
///
/// This is intended for read-mostly inspection flows that want stable values
/// without repeatedly touching the raw `ActiveEffect` instance.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ActiveEffectSnapshot {
    /// Live active-effect instance.
    pub effect: GamePtr<ActiveEffect>,
    /// Owning spell or magic item.
    pub spell: GamePtr<MagicItem>,
    /// Effect setting backing the active effect.
    pub base: GamePtr<EffectSetting>,
    pub magnitude: f32,
    pub duration: f32,
    pub elapsed_seconds: f32,
    pub casting_source: CastingSource,
}

/// Snapshot of one casting-source slot on an actor.
///
/// This is useful for gameplay or HUD code that wants to inspect selected and
/// current spells for one hand/source in a single query.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CastingSourceSnapshot {
    /// Left hand, right hand, instant, etc.
    pub source: CastingSource,
    /// Spell currently selected into that source.
    pub selected_spell: GamePtr<MagicItem>,
    /// Spell currently being cast from that source.
    pub current_spell: GamePtr<MagicItem>,
    /// Current spell cost resolved for the source.
    pub current_spell_cost: f32,
    pub dual_casting: bool,
    pub has_magic_caster: bool,
}
