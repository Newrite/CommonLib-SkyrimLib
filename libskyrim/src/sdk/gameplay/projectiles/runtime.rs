use alloc::borrow::ToOwned;
use alloc::vec::Vec;

use crate::re::{
    Actor, BGSProjectile, MagicItem, ObjectRefHandle, Projectile, ProjectileHandle,
    ProjectileManager, TESAmmo, TESObjectREFR, TESObjectWEAP,
};
use crate::sdk::core::{GamePtr, GameRef, Resolved};

use super::shared::{
    MAX_REASONABLE_IMPACTS, game_ptr, handle_to_ptr, projectile_kind_from_ptr,
    snapshot_handle_bucket, snapshot_impact,
};
use super::types::{
    ProjectileBaseSnapshot, ProjectileImpactSnapshot, ProjectileKind, ProjectileManagerSnapshot,
    ProjectileSnapshot,
};

#[inline(always)]
/// Returns the live projectile manager singleton.
///
/// This is the strict entrypoint when higher-level helpers need direct access
/// to the engine-owned manager object.
pub fn singleton() -> GameRef<ProjectileManager> {
    unsafe { GameRef::from_raw(ProjectileManager::get_singleton()) }
}

/// Snapshot the projectile manager's current handle buckets.
///
/// This is the lightest entry point when plugin code wants a broad view of the
/// manager before deciding whether to resolve live projectiles or not.
pub fn manager_snapshot() -> ProjectileManagerSnapshot {
    singleton().with(|manager| ProjectileManagerSnapshot {
        unlimited: snapshot_handle_bucket(
            &manager.unlimited,
            "sdk::gameplay::projectiles::manager_snapshot()/unlimited",
        ),
        limited: snapshot_handle_bucket(
            &manager.limited,
            "sdk::gameplay::projectiles::manager_snapshot()/limited",
        ),
        pending: snapshot_handle_bucket(
            &manager.pending,
            "sdk::gameplay::projectiles::manager_snapshot()/pending",
        ),
    })
}

/// Resolve all currently managed projectiles reported by the manager.
///
/// Prefer this over manual bucket traversal when the next step needs live
/// `Resolved<Projectile>` values rather than just snapshot metadata.
pub fn collect_managed_projectiles() -> Vec<Resolved<Projectile>> {
    manager_snapshot().managed_projectiles()
}

/// Snapshot all currently managed projectiles.
///
/// This is the common entry point for diagnostic, targeting, and behavior code
/// that wants one flat runtime view per projectile.
pub fn collect_managed_projectile_snapshots() -> Vec<ProjectileSnapshot> {
    collect_managed_projectiles()
        .into_iter()
        .map(|projectile| snapshot_projectile(projectile.as_ref()))
        .collect()
}

/// Snapshot only the managed projectiles that satisfy a predicate.
///
/// Use this when plugin logic naturally filters projectiles through one
/// snapshot-driven predicate without needing to hold live `Resolved` values.
pub fn collect_managed_projectile_snapshots_matching(
    mut predicate: impl FnMut(&ProjectileSnapshot) -> bool,
) -> Vec<ProjectileSnapshot> {
    let mut projectiles = Vec::new();
    for snapshot in collect_managed_projectile_snapshots() {
        if predicate(&snapshot) {
            projectiles.push(snapshot);
        }
    }
    projectiles
}

/// Find the first managed projectile snapshot satisfying a predicate.
///
/// This is a convenience query for "find one interesting projectile" flows
/// such as nearest/first active behavior scans.
pub fn find_managed_projectile_snapshot_matching(
    mut predicate: impl FnMut(&ProjectileSnapshot) -> bool,
) -> Option<ProjectileSnapshot> {
    for snapshot in collect_managed_projectile_snapshots() {
        if predicate(&snapshot) {
            return Some(snapshot);
        }
    }
    None
}

#[inline(always)]
/// Returns the projectile base form currently attached to one projectile.
pub fn projectile_base(projectile: &Projectile) -> GamePtr<BGSProjectile> {
    game_ptr(projectile.get_projectile_base())
}

/// Classify a live projectile into the SDK-facing [`ProjectileKind`] family.
#[inline(always)]
pub fn projectile_kind(projectile: &Projectile) -> Option<ProjectileKind> {
    let base = projectile_base(projectile);
    base.with(ProjectileKind::from_base)
        .flatten()
        .or_else(|| projectile_kind_from_ptr(projectile))
}

#[inline(always)]
/// Returns the shooter's recorded handle without resolving it.
pub fn projectile_shooter_handle(projectile: &Projectile) -> ObjectRefHandle {
    projectile.get_projectile_runtime_data().shooter
}

/// Resolve the shooter reference currently recorded in one projectile.
#[inline(always)]
pub fn projectile_shooter(projectile: &Projectile) -> GamePtr<TESObjectREFR> {
    handle_to_ptr(projectile_shooter_handle(projectile))
}

/// Resolve the shooter as an actor when that downcast is valid.
#[inline(always)]
pub fn projectile_shooter_actor(projectile: &Projectile) -> GamePtr<Actor> {
    projectile_shooter(projectile).try_cast::<Actor>()
}

#[inline(always)]
/// Returns the desired target handle without resolving it.
pub fn projectile_desired_target_handle(projectile: &Projectile) -> ObjectRefHandle {
    projectile.get_projectile_runtime_data().desired_target
}

/// Resolve the projectile's current desired target reference.
#[inline(always)]
pub fn projectile_desired_target(projectile: &Projectile) -> GamePtr<TESObjectREFR> {
    handle_to_ptr(projectile_desired_target_handle(projectile))
}

/// Resolve the current desired target as an actor when possible.
#[inline(always)]
pub fn projectile_desired_target_actor(projectile: &Projectile) -> GamePtr<Actor> {
    projectile_desired_target(projectile).try_cast::<Actor>()
}

/// Check whether the projectile currently carries any desired target handle.
#[inline(always)]
/// Returns whether the projectile currently carries any desired target handle.
pub fn projectile_has_desired_target(projectile: &Projectile) -> bool {
    projectile_desired_target_handle(projectile).has_value()
}

/// Resolve the spell source currently recorded in one projectile.
#[inline(always)]
pub fn projectile_spell(projectile: &Projectile) -> GamePtr<MagicItem> {
    game_ptr(projectile.get_projectile_runtime_data().spell)
}

/// Resolve the weapon source currently recorded in one projectile.
#[inline(always)]
pub fn projectile_weapon_source(projectile: &Projectile) -> GamePtr<TESObjectWEAP> {
    game_ptr(projectile.get_projectile_runtime_data().weapon_source)
}

/// Resolve the ammo source currently recorded in one projectile.
#[inline(always)]
pub fn projectile_ammo_source(projectile: &Projectile) -> GamePtr<TESAmmo> {
    game_ptr(projectile.get_projectile_runtime_data().ammo_source)
}

/// Snapshot one projectile base form into a stable SDK-facing value.
pub fn snapshot_projectile_base(projectile: &BGSProjectile) -> ProjectileBaseSnapshot {
    ProjectileBaseSnapshot {
        projectile: game_ptr(projectile as *const BGSProjectile as *mut BGSProjectile),
        form_id: projectile.base.get_form_id(),
        editor_id: projectile.base.get_form_editor_id_as_str().to_owned(),
        display_name: projectile.full_name.get_name_as_str().to_owned(),
        kind: ProjectileKind::from_base(projectile),
        flags: projectile.data.flags,
        types: projectile.data.types,
        gravity: projectile.data.gravity,
        speed: projectile.data.speed,
        range: projectile.data.range,
        tracer_chance: projectile.data.tracer_chance,
        explosion_proximity: projectile.data.explosion_proximity,
        explosion_timer: projectile.data.explosion_timer,
        explosion_type: game_ptr(projectile.data.explosion_type),
        cone_spread: projectile.data.cone_spread,
        collision_radius: projectile.data.collision_radius,
        lifetime: projectile.data.lifetime,
        relaunch_interval: projectile.data.relaunch_interval,
        default_weapon_source: game_ptr(projectile.data.default_weapon_source),
    }
}

#[inline(always)]
/// Snapshot the current projectile base, if one is attached.
///
/// Prefer this over [`snapshot_projectile_base`] when the caller starts from a
/// live `Projectile` rather than a `BGSProjectile` form.
pub fn snapshot_projectile_base_of(projectile: &Projectile) -> Option<ProjectileBaseSnapshot> {
    projectile_base(projectile).with(snapshot_projectile_base)
}

/// Snapshot all stored impact records for one projectile.
///
/// This intentionally returns a bounded, defensive snapshot rather than
/// exposing the raw retained impact pointers directly.
pub fn snapshot_projectile_impacts(projectile: &Projectile) -> Vec<ProjectileImpactSnapshot> {
    let runtime = projectile.get_projectile_runtime_data();
    let mut impacts = Vec::new();
    for impact in runtime.impacts.iter() {
        if impacts.len() >= MAX_REASONABLE_IMPACTS {
            crate::defensive_sdk_warn!(
                "sdk::gameplay::projectiles::snapshot_projectile_impacts() truncated suspiciously large impact list at {} entries",
                MAX_REASONABLE_IMPACTS
            );
            break;
        }

        let Some(impact) = (unsafe { (*impact).as_ref() }) else {
            continue;
        };
        impacts.push(snapshot_impact(impact));
    }
    impacts
}

/// Snapshot one live projectile into the main flat runtime result.
///
/// This is the main bridge from live `Projectile` references into the SDK's
/// targeting/behavior/debugging workflows.
pub fn snapshot_projectile(projectile: &Projectile) -> ProjectileSnapshot {
    let runtime = projectile.get_projectile_runtime_data();
    let base = projectile_base(projectile);
    ProjectileSnapshot {
        projectile: game_ptr(projectile as *const Projectile as *mut Projectile),
        handle: ProjectileHandle::from_ptr(projectile as *const Projectile as *mut Projectile),
        kind: projectile_kind(projectile),
        base,
        base_snapshot: base.with(snapshot_projectile_base),
        shooter_handle: runtime.shooter,
        desired_target_handle: runtime.desired_target,
        spell: game_ptr(runtime.spell),
        av_effect: game_ptr(runtime.av_effect),
        explosion: game_ptr(runtime.explosion),
        weapon_source: game_ptr(runtime.weapon_source),
        ammo_source: game_ptr(runtime.ammo_source),
        casting_source: runtime.casting_source,
        position: projectile.base.get_position(),
        angle_x: projectile.base.get_angle_x(),
        angle_z: projectile.base.get_angle_z(),
        velocity: runtime.velocity,
        linear_velocity: runtime.linear_velocity,
        power: runtime.power,
        speed: projectile.get_speed(),
        speed_mult: runtime.speed_mult,
        range: runtime.range,
        living_time: runtime.living_time,
        weapon_damage: runtime.weapon_damage,
        transparency: runtime.transparency,
        explosion_timer: runtime.explosion_timer,
        distance_moved: runtime.distance_moved,
        scale: runtime.scale,
        flags: runtime.flags,
        impact_count: runtime.impacts.len(),
    }
}
