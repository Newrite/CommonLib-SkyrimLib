use alloc::vec::Vec;
use core::cmp::Ordering;
use core::ops::ControlFlow;

use libm::{cosf, sinf};

use crate::re::{Actor, ActorHandle, NiPoint3, Projectile, TESObjectREFR};
use crate::sdk::advanced::physics::{
    RaycastHitFilter, actor_line_of_sight_filter, has_line_of_sight_with_filter,
};
use crate::sdk::gameplay::{actors, world};

use super::behavior::{clear_projectile_desired_target, set_projectile_desired_target};
use super::runtime::projectile_shooter_actor;
use super::shared::{game_ptr, is_valid_point, is_valid_radius, reference_linear_velocity};
use super::types::{
    ProjectileRetargetStrategy, ProjectileTargetDisposition, ProjectileTargetSearchOptions,
    ProjectileTargetSnapshot,
};

#[inline(always)]
fn projectile_target_search_origin(
    caster: &Actor,
    options: ProjectileTargetSearchOptions,
) -> Option<NiPoint3> {
    let origin = options
        .search_origin
        .unwrap_or_else(|| caster.base.get_position());
    if is_valid_point(origin) {
        Some(origin)
    } else {
        None
    }
}

#[inline(always)]
fn projectile_target_view_origin(
    caster: &Actor,
    options: ProjectileTargetSearchOptions,
) -> Option<NiPoint3> {
    let origin = caster.calculate_los_location(options.source_los_location);
    if is_valid_point(origin) {
        Some(origin)
    } else {
        None
    }
}

#[inline(always)]
fn projectile_target_position(
    actor: &Actor,
    options: ProjectileTargetSearchOptions,
) -> Option<NiPoint3> {
    let position = actor.calculate_los_location(options.target_los_location);
    if is_valid_point(position) {
        Some(position)
    } else {
        None
    }
}

#[inline(always)]
fn actor_aim_direction(actor: &Actor) -> NiPoint3 {
    let pitch = actor.get_aim_angle();
    let yaw = actor.get_aim_heading();
    let cos_pitch = cosf(pitch);
    let mut direction = NiPoint3::new(sinf(yaw) * cos_pitch, cosf(yaw) * cos_pitch, sinf(pitch));
    direction.unitize();
    direction
}

#[inline(always)]
fn view_alignment_dot(origin: NiPoint3, facing: NiPoint3, target: NiPoint3) -> Option<f32> {
    let mut delta = target - origin;
    if delta.unitize() <= f32::EPSILON {
        return None;
    }

    Some(facing.dot(delta))
}

#[inline(always)]
pub(super) fn view_cone_threshold_dot(maximum_view_cone_degrees: f32) -> f32 {
    if !maximum_view_cone_degrees.is_finite() {
        return -1.0;
    }

    if maximum_view_cone_degrees <= 0.0 {
        return 1.0;
    }

    if maximum_view_cone_degrees >= 180.0 {
        return -1.0;
    }

    cosf(maximum_view_cone_degrees * (core::f32::consts::PI / 180.0))
}

#[inline(always)]
fn is_aggressive_to_caster(candidate: &Actor, caster_handle: ActorHandle) -> bool {
    candidate.get_actor_runtime_data().current_combat_target == caster_handle
}

#[inline(always)]
fn disposition_matches(
    options: ProjectileTargetSearchOptions,
    hostile_to_caster: bool,
    aggressive_to_caster: bool,
) -> bool {
    match options.disposition {
        ProjectileTargetDisposition::AggressiveToCaster => aggressive_to_caster,
        ProjectileTargetDisposition::HostileToCaster => hostile_to_caster,
        ProjectileTargetDisposition::AnyActor => true,
    }
}

#[inline(always)]
fn line_of_sight_to_target(
    caster: &Actor,
    candidate: &Actor,
    view_origin: NiPoint3,
    target_position: NiPoint3,
) -> Option<bool> {
    let cell = unsafe { caster.base.get_parent_cell().as_ref() }?;
    let ignored = [
        caster as *const Actor as *mut Actor as *mut TESObjectREFR,
        candidate as *const Actor as *mut Actor as *mut TESObjectREFR,
    ];
    let filter = actor_line_of_sight_filter(caster);
    let hit_filter = RaycastHitFilter::new().ignore_references(&ignored);
    Some(has_line_of_sight_with_filter(
        cell,
        view_origin,
        target_position,
        filter,
        &hit_filter,
    ))
}

pub(super) fn nearest_target_index(candidates: &[ProjectileTargetSnapshot]) -> Option<usize> {
    candidates
        .iter()
        .enumerate()
        .min_by(|(_, left), (_, right)| {
            left.squared_distance
                .partial_cmp(&right.squared_distance)
                .unwrap_or(Ordering::Equal)
        })
        .map(|(index, _)| index)
}

pub(super) fn view_aligned_target_index(candidates: &[ProjectileTargetSnapshot]) -> Option<usize> {
    candidates
        .iter()
        .enumerate()
        .max_by(
            |(_, left), (_, right)| match (left.view_alignment_dot, right.view_alignment_dot) {
                (Some(left_dot), Some(right_dot)) => left_dot
                    .partial_cmp(&right_dot)
                    .unwrap_or(Ordering::Equal)
                    .then_with(|| {
                        right
                            .squared_distance
                            .partial_cmp(&left.squared_distance)
                            .unwrap_or(Ordering::Equal)
                    }),
                (Some(_), None) => Ordering::Greater,
                (None, Some(_)) => Ordering::Less,
                (None, None) => right
                    .squared_distance
                    .partial_cmp(&left.squared_distance)
                    .unwrap_or(Ordering::Equal),
            },
        )
        .map(|(index, _)| index)
}

#[inline(always)]
pub(super) fn target_index_for_strategy(
    candidates: &[ProjectileTargetSnapshot],
    strategy: ProjectileRetargetStrategy,
) -> Option<usize> {
    match strategy {
        ProjectileRetargetStrategy::Nearest => nearest_target_index(candidates),
        ProjectileRetargetStrategy::ViewAligned => view_aligned_target_index(candidates),
    }
}

fn evaluate_projectile_target_candidate(
    caster: &Actor,
    candidate: &Actor,
    search_origin: NiPoint3,
    view_origin: NiPoint3,
    facing: NiPoint3,
    view_threshold_dot: Option<f32>,
    options: ProjectileTargetSearchOptions,
) -> Option<ProjectileTargetSnapshot> {
    if core::ptr::eq(candidate, caster) {
        return None;
    }

    if candidate.base.is_dead(false) || candidate.base.is_disabled() {
        return None;
    }

    let target_position = projectile_target_position(candidate, options)?;
    let squared_distance = search_origin.get_squared_distance(target_position);
    if squared_distance > options.radius * options.radius {
        return None;
    }

    let hostile_to_caster = actors::is_hostile_to(candidate, caster);
    let aggressive_to_caster = is_aggressive_to_caster(candidate, caster.get_handle());
    if !disposition_matches(options, hostile_to_caster, aggressive_to_caster) {
        return None;
    }

    let view_alignment_dot = if let Some(threshold_dot) = view_threshold_dot {
        let dot = view_alignment_dot(view_origin, facing, target_position)?;
        if dot < threshold_dot {
            return None;
        }
        Some(dot)
    } else {
        None
    };

    let line_of_sight_clear = if options.require_line_of_sight {
        let clear = line_of_sight_to_target(caster, candidate, view_origin, target_position)?;
        if !clear {
            return None;
        }
        Some(clear)
    } else {
        None
    };
    let velocity = reference_linear_velocity(&candidate.base);

    Some(ProjectileTargetSnapshot {
        actor: game_ptr(candidate as *const Actor as *mut Actor),
        handle: candidate.get_handle(),
        position: target_position,
        velocity,
        speed: velocity.length(),
        distance: search_origin.get_distance(target_position),
        squared_distance,
        hostile_to_caster,
        aggressive_to_caster,
        line_of_sight_clear,
        view_alignment_dot,
    })
}

fn for_each_target_reference_near_origin(
    caster: &Actor,
    search_origin: NiPoint3,
    radius: f32,
    mut visit: impl FnMut(&TESObjectREFR) -> ControlFlow<()>,
) -> ControlFlow<()> {
    if let Some(cell) = unsafe { caster.base.get_parent_cell().as_ref() } {
        world::for_each_reference_in_cell_range(cell, search_origin, radius, visit)
    } else {
        let radius_squared = radius * radius;
        world::for_each_reference(|reference| {
            if reference.get_position().get_squared_distance(search_origin) <= radius_squared {
                visit(reference)
            } else {
                ControlFlow::Continue(())
            }
        })
    }
}

fn collect_projectile_targets_from_origin(
    caster: &Actor,
    search_origin: NiPoint3,
    options: ProjectileTargetSearchOptions,
    _caller: &'static str,
) -> Vec<ProjectileTargetSnapshot> {
    if !is_valid_radius(options.radius, _caller) {
        return Vec::new();
    }

    if !is_valid_point(search_origin) {
        crate::defensive_sdk_warn!("{} ignored non-finite search origin", _caller);
        return Vec::new();
    }

    let Some(view_origin) = projectile_target_view_origin(caster, options) else {
        crate::defensive_sdk_warn!("{} ignored actor with non-finite LOS origin", _caller);
        return Vec::new();
    };

    if options.require_line_of_sight && unsafe { caster.base.get_parent_cell().as_ref() }.is_none()
    {
        crate::defensive_sdk_warn!(
            "{} could not evaluate LOS because the caster had no parent cell",
            _caller
        );
        return Vec::new();
    }

    let facing = actor_aim_direction(caster);
    let view_threshold_dot = options
        .maximum_view_cone_degrees
        .map(view_cone_threshold_dot);
    let mut targets = Vec::new();
    let _ =
        for_each_target_reference_near_origin(caster, search_origin, options.radius, |reference| {
            let actor = game_ptr(reference as *const TESObjectREFR as *mut TESObjectREFR)
                .try_cast::<Actor>();
            let Some(actor) = actor.as_ref() else {
                return ControlFlow::Continue(());
            };

            if let Some(snapshot) = evaluate_projectile_target_candidate(
                caster,
                actor,
                search_origin,
                view_origin,
                facing,
                view_threshold_dot,
                options,
            ) {
                targets.push(snapshot);
            }

            ControlFlow::Continue(())
        });
    targets
}

fn apply_reacquired_desired_target(
    projectile: &mut Projectile,
    snapshot: Option<ProjectileTargetSnapshot>,
) -> Option<ProjectileTargetSnapshot> {
    let Some(snapshot) = snapshot else {
        clear_projectile_desired_target(projectile);
        return None;
    };

    let reference = snapshot.reference();
    let Some(reference) = reference.as_ref() else {
        clear_projectile_desired_target(projectile);
        return None;
    };

    set_projectile_desired_target(projectile, reference);
    Some(snapshot)
}

fn select_projectile_target(
    targets: Vec<ProjectileTargetSnapshot>,
    strategy: ProjectileRetargetStrategy,
) -> Option<ProjectileTargetSnapshot> {
    let index = target_index_for_strategy(&targets, strategy)?;
    Some(targets[index])
}

/// Collect all valid projectile targets around one caster using the supplied
/// search controls.
pub fn collect_projectile_targets(
    caster: &Actor,
    options: ProjectileTargetSearchOptions,
) -> Vec<ProjectileTargetSnapshot> {
    let Some(search_origin) = projectile_target_search_origin(caster, options) else {
        crate::defensive_sdk_warn!(
            "sdk::gameplay::projectiles::collect_projectile_targets() ignored non-finite search origin"
        );
        return Vec::new();
    };

    collect_projectile_targets_from_origin(
        caster,
        search_origin,
        options,
        "sdk::gameplay::projectiles::collect_projectile_targets()",
    )
}

/// Collect all valid projectile targets for one live projectile by reusing the
/// projectile's shooter when possible.
pub fn collect_projectile_targets_for_projectile(
    projectile: &Projectile,
    options: ProjectileTargetSearchOptions,
) -> Vec<ProjectileTargetSnapshot> {
    let shooter = projectile_shooter_actor(projectile);
    let Some(caster) = shooter.as_ref() else {
        crate::defensive_sdk_warn!(
            "sdk::gameplay::projectiles::collect_projectile_targets_for_projectile() ignored projectile without actor shooter"
        );
        return Vec::new();
    };

    let search_origin = options
        .search_origin
        .unwrap_or_else(|| projectile.base.get_position());
    collect_projectile_targets_from_origin(
        caster,
        search_origin,
        options,
        "sdk::gameplay::projectiles::collect_projectile_targets_for_projectile()",
    )
}

/// Find the nearest valid projectile target.
pub fn find_nearest_projectile_target(
    caster: &Actor,
    options: ProjectileTargetSearchOptions,
) -> Option<ProjectileTargetSnapshot> {
    find_projectile_target(caster, ProjectileRetargetStrategy::Nearest, options)
}

pub fn find_nearest_projectile_target_for_projectile(
    projectile: &Projectile,
    options: ProjectileTargetSearchOptions,
) -> Option<ProjectileTargetSnapshot> {
    find_projectile_target_for_projectile(projectile, ProjectileRetargetStrategy::Nearest, options)
}

/// Find the best view-aligned valid projectile target.
pub fn find_view_aligned_projectile_target(
    caster: &Actor,
    options: ProjectileTargetSearchOptions,
) -> Option<ProjectileTargetSnapshot> {
    find_projectile_target(caster, ProjectileRetargetStrategy::ViewAligned, options)
}

pub fn find_view_aligned_projectile_target_for_projectile(
    projectile: &Projectile,
    options: ProjectileTargetSearchOptions,
) -> Option<ProjectileTargetSnapshot> {
    find_projectile_target_for_projectile(
        projectile,
        ProjectileRetargetStrategy::ViewAligned,
        options,
    )
}

/// Find the best projectile target using the supplied retarget strategy.
///
/// This is the strategy-parameterized entrypoint behind the convenience
/// nearest/view-aligned helpers.
pub fn find_projectile_target(
    caster: &Actor,
    strategy: ProjectileRetargetStrategy,
    options: ProjectileTargetSearchOptions,
) -> Option<ProjectileTargetSnapshot> {
    select_projectile_target(collect_projectile_targets(caster, options), strategy)
}

/// Find the best projectile target for one live projectile.
///
/// This reuses the projectile's shooter when possible and falls back to the
/// projectile position as the search origin.
pub fn find_projectile_target_for_projectile(
    projectile: &Projectile,
    strategy: ProjectileRetargetStrategy,
    options: ProjectileTargetSearchOptions,
) -> Option<ProjectileTargetSnapshot> {
    select_projectile_target(
        collect_projectile_targets_for_projectile(projectile, options),
        strategy,
    )
}

/// Reacquire and store a projectile desired target using the supplied strategy.
pub fn reacquire_projectile_desired_target(
    projectile: &mut Projectile,
    strategy: ProjectileRetargetStrategy,
    options: ProjectileTargetSearchOptions,
) -> Option<ProjectileTargetSnapshot> {
    let snapshot = find_projectile_target_for_projectile(projectile, strategy, options);
    apply_reacquired_desired_target(projectile, snapshot)
}

/// Reacquire and store the nearest valid desired target.
pub fn reacquire_nearest_projectile_desired_target(
    projectile: &mut Projectile,
    options: ProjectileTargetSearchOptions,
) -> Option<ProjectileTargetSnapshot> {
    reacquire_projectile_desired_target(projectile, ProjectileRetargetStrategy::Nearest, options)
}

/// Reacquire and store the best view-aligned desired target.
pub fn reacquire_view_aligned_projectile_desired_target(
    projectile: &mut Projectile,
    options: ProjectileTargetSearchOptions,
) -> Option<ProjectileTargetSnapshot> {
    reacquire_projectile_desired_target(
        projectile,
        ProjectileRetargetStrategy::ViewAligned,
        options,
    )
}
