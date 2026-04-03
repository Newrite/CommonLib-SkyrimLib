use crate::re::{
    Actor, CastingSource, NiPoint3, Projectile, ProjectileHandle, ProjectileLaunchData,
    ProjectileRot, SpellItem, TESAmmo, TESObjectWEAP,
};
use crate::sdk::core::Resolved;

use super::shared::launched_projectile;

pub fn launch_with_data(data: &mut ProjectileLaunchData) -> Option<Resolved<Projectile>> {
    let mut result = ProjectileHandle::new();
    Projectile::launch(&mut result, data);
    launched_projectile(result, "sdk::gameplay::projectiles::launch_with_data()")
}

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
