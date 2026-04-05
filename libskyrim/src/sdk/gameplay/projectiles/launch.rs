use crate::re::{
    Actor, CastingSource, NiPoint3, Projectile, ProjectileHandle, ProjectileLaunchData,
    ProjectileRot, SpellItem, TESAmmo, TESObjectWEAP,
};
use crate::sdk::core::Resolved;

use super::shared::launched_projectile;

/// Launch a projectile using a fully prepared `ProjectileLaunchData` block.
///
/// Prefer this when plugin code already has a source-backed launch packet and
/// wants the SDK only for safe-ish handle resolution afterward.
pub fn launch_with_data(data: &mut ProjectileLaunchData) -> Option<Resolved<Projectile>> {
    let mut result = ProjectileHandle::new();
    Projectile::launch(&mut result, data);
    launched_projectile(result, "sdk::gameplay::projectiles::launch_with_data()")
}

/// Launch a spell projectile from an explicit origin and rotation.
///
/// This is the manual placement variant for spell launches when the plugin
/// wants to override the engine's automatic origin/aim choice.
pub fn launch_spell(
    shooter: &mut Actor,
    spell: &SpellItem,
    origin: &NiPoint3,
    angles: &ProjectileRot,
) -> Option<Resolved<Projectile>> {
    let mut result = ProjectileHandle::new();
    Projectile::launch_spell(
        &mut result,
        shooter as *mut Actor,
        spell as *const SpellItem as *mut SpellItem,
        origin,
        angles,
    );
    launched_projectile(result, "sdk::gameplay::projectiles::launch_spell()")
}

/// Launch a spell projectile from one actor/casting source combination.
///
/// Prefer this when the engine should derive the origin/aiming transform from
/// the actor and casting source.
pub fn launch_spell_from_source(
    shooter: &mut Actor,
    spell: &SpellItem,
    source: CastingSource,
) -> Option<Resolved<Projectile>> {
    let mut result = ProjectileHandle::new();
    Projectile::launch_spell_from_source(
        &mut result,
        shooter as *mut Actor,
        spell as *const SpellItem as *mut SpellItem,
        source,
    );
    launched_projectile(
        result,
        "sdk::gameplay::projectiles::launch_spell_from_source()",
    )
}

/// Launch an arrow projectile with explicit origin and rotation.
///
/// This is the manual placement variant for arrow-style launches.
pub fn launch_arrow(
    shooter: &mut Actor,
    ammo: &TESAmmo,
    weapon: &TESObjectWEAP,
    origin: &NiPoint3,
    angles: &ProjectileRot,
) -> Option<Resolved<Projectile>> {
    let mut result = ProjectileHandle::new();
    Projectile::launch_arrow(
        &mut result,
        shooter as *mut Actor,
        ammo as *const TESAmmo as *mut TESAmmo,
        weapon as *const TESObjectWEAP as *mut TESObjectWEAP,
        origin,
        angles,
    );
    launched_projectile(result, "sdk::gameplay::projectiles::launch_arrow()")
}

/// Launch an arrow projectile using the engine's automatic origin/angle logic.
///
/// Prefer this when the plugin wants normal bow launch semantics and only
/// needs the resolved projectile handle afterward.
pub fn launch_arrow_auto(
    shooter: &mut Actor,
    ammo: &TESAmmo,
    weapon: &TESObjectWEAP,
) -> Option<Resolved<Projectile>> {
    let mut result = ProjectileHandle::new();
    Projectile::launch_arrow_auto(
        &mut result,
        shooter as *mut Actor,
        ammo as *const TESAmmo as *mut TESAmmo,
        weapon as *const TESObjectWEAP as *mut TESObjectWEAP,
    );
    launched_projectile(result, "sdk::gameplay::projectiles::launch_arrow_auto()")
}
