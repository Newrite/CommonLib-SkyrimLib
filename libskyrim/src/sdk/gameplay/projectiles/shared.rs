use alloc::vec::Vec;

use libm::{acosf, asinf, atan2f, sqrtf};

use crate::re::{
    Actor, ArrowProjectile, BarrierProjectile, BeamProjectile, ConeProjectile, FlameProjectile,
    GrenadeProjectile, MissileProjectile, NiPoint3, ObjectRefHandle, Projectile, ProjectileHandle,
    ProjectileImpactData, ProjectileRot, TESObjectREFR,
};
use crate::sdk::core::{
    ContiguousSequenceIterationOptions, GamePtr, Resolved, snapshot_contiguous_copied_named,
};

use super::types::{ProjectileImpactSnapshot, ProjectileInterceptSnapshot, ProjectileKind};

pub(super) const MAX_REASONABLE_MANAGER_HANDLES: u32 = 0x10000;
pub(super) const MAX_REASONABLE_IMPACTS: usize = 0x400;

#[inline(always)]
pub(super) fn projectile_handle_iteration_options() -> ContiguousSequenceIterationOptions {
    ContiguousSequenceIterationOptions::new()
        .with_max_reasonable_len(MAX_REASONABLE_MANAGER_HANDLES)
}

#[inline(always)]
pub(super) fn is_valid_radius(radius: f32, _caller: &'static str) -> bool {
    if !radius.is_finite() || radius <= 0.0 {
        crate::defensive_sdk_warn!(
            "{} ignored non-positive or non-finite radius={}",
            _caller,
            radius
        );
        false
    } else {
        true
    }
}

#[inline(always)]
pub(super) fn is_valid_point(point: NiPoint3) -> bool {
    point.x.is_finite() && point.y.is_finite() && point.z.is_finite()
}

#[inline(always)]
pub(super) fn is_valid_non_negative_seconds(delta_seconds: f32) -> bool {
    delta_seconds.is_finite() && delta_seconds >= 0.0
}

#[inline(always)]
pub(super) fn is_valid_positive_speed(speed: f32) -> bool {
    speed.is_finite() && speed > f32::EPSILON
}

#[inline(always)]
pub(super) fn clamp_unit_scalar(value: f32) -> f32 {
    value.clamp(-1.0, 1.0)
}

#[inline(always)]
pub(super) fn game_ptr<T>(raw: *mut T) -> GamePtr<T> {
    unsafe { GamePtr::from_raw(raw) }
}

#[inline(always)]
pub(super) fn handle_to_ptr(handle: ObjectRefHandle) -> GamePtr<TESObjectREFR> {
    if !handle.has_value() {
        return GamePtr::null();
    }

    game_ptr(handle.get().get())
}

#[inline(always)]
pub(super) fn reference_linear_velocity(reference: &TESObjectREFR) -> NiPoint3 {
    let mut velocity = NiPoint3::default();
    reference.get_linear_velocity(&mut velocity);
    if is_valid_point(velocity) {
        velocity
    } else {
        NiPoint3::default()
    }
}

#[inline(always)]
pub(super) fn anticipated_position_from_velocity(
    position: NiPoint3,
    velocity: NiPoint3,
    delta_seconds: f32,
) -> Option<NiPoint3> {
    if !is_valid_non_negative_seconds(delta_seconds)
        || !is_valid_point(position)
        || !is_valid_point(velocity)
    {
        return None;
    }

    let anticipated = position + velocity * delta_seconds;
    is_valid_point(anticipated).then_some(anticipated)
}

#[inline(always)]
pub(super) fn projectile_effective_range(projectile: &Projectile) -> Option<f32> {
    let runtime_range = projectile.get_projectile_runtime_data().range;
    if runtime_range.is_finite() && runtime_range > 0.0 {
        return Some(runtime_range);
    }

    super::runtime::projectile_base(projectile)
        .with(|base| {
            let range = base.data.range;
            (range.is_finite() && range > 0.0).then_some(range)
        })
        .flatten()
}

#[inline(always)]
pub(super) fn target_reference_is_alive(reference: &TESObjectREFR) -> bool {
    let actor =
        game_ptr(reference as *const TESObjectREFR as *mut TESObjectREFR).try_cast::<Actor>();
    actor
        .as_ref()
        .map(|actor| !actor.base.is_dead(false))
        .unwrap_or(true)
}

#[inline(always)]
pub(super) fn projectile_current_linear_speed(projectile: &Projectile) -> Option<f32> {
    let runtime_speed = projectile
        .get_projectile_runtime_data()
        .linear_velocity
        .length();
    if is_valid_positive_speed(runtime_speed) {
        return Some(runtime_speed);
    }

    let projectile_speed = projectile.get_speed();
    is_valid_positive_speed(projectile_speed).then_some(projectile_speed)
}

#[inline(always)]
fn choose_smallest_non_negative_time(first: f32, second: f32) -> Option<f32> {
    let first_valid = first.is_finite() && first >= 0.0;
    let second_valid = second.is_finite() && second >= 0.0;

    match (first_valid, second_valid) {
        (true, true) => Some(first.min(second)),
        (true, false) => Some(first),
        (false, true) => Some(second),
        (false, false) => None,
    }
}

pub(super) fn solve_constant_speed_intercept_time(
    origin: NiPoint3,
    target_position: NiPoint3,
    target_velocity: NiPoint3,
    projectile_speed: f32,
) -> Option<f32> {
    if !is_valid_point(origin)
        || !is_valid_point(target_position)
        || !is_valid_point(target_velocity)
        || !is_valid_positive_speed(projectile_speed)
    {
        return None;
    }

    let relative = target_position - origin;
    let c = relative.sqr_length();
    if c <= f32::EPSILON {
        return Some(0.0);
    }

    let speed_squared = projectile_speed * projectile_speed;
    let target_speed_squared = target_velocity.sqr_length();
    let a = target_speed_squared - speed_squared;
    let b = 2.0 * relative.dot(target_velocity);

    if a.abs() <= f32::EPSILON {
        if b.abs() <= f32::EPSILON {
            return None;
        }

        let time = -c / b;
        return (time.is_finite() && time >= 0.0).then_some(time);
    }

    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 || !discriminant.is_finite() {
        return None;
    }

    let root = sqrtf(discriminant);
    let denominator = 2.0 * a;
    choose_smallest_non_negative_time((-b - root) / denominator, (-b + root) / denominator)
}

pub(super) fn build_projectile_intercept_snapshot(
    origin: NiPoint3,
    target_position: NiPoint3,
    target_velocity: NiPoint3,
    projectile_speed: f32,
    initial_prediction_seconds: f32,
) -> Option<ProjectileInterceptSnapshot> {
    let anticipated_target = anticipated_position_from_velocity(
        target_position,
        target_velocity,
        initial_prediction_seconds,
    )?;
    let travel_time_seconds = solve_constant_speed_intercept_time(
        origin,
        anticipated_target,
        target_velocity,
        projectile_speed,
    )?;
    let intercept_point = anticipated_position_from_velocity(
        anticipated_target,
        target_velocity,
        travel_time_seconds,
    )?;
    let mut direction = intercept_point - origin;
    if direction.unitize() <= f32::EPSILON {
        return None;
    }

    Some(ProjectileInterceptSnapshot {
        origin,
        target_position,
        target_velocity,
        initial_prediction_seconds,
        travel_time_seconds,
        projectile_speed,
        intercept_point,
        direction,
    })
}

pub(super) fn projectile_rotation_from_direction(direction: NiPoint3) -> Option<ProjectileRot> {
    if !is_valid_point(direction) {
        return None;
    }

    let mut direction = direction;
    if direction.unitize() <= f32::EPSILON {
        return None;
    }

    Some(ProjectileRot {
        x: asinf(clamp_unit_scalar(direction.z)),
        z: atan2f(direction.x, direction.y),
    })
}

pub(super) fn projectile_rotation_from_points(
    origin: NiPoint3,
    target: NiPoint3,
) -> Option<ProjectileRot> {
    if !is_valid_point(origin) || !is_valid_point(target) {
        return None;
    }

    projectile_rotation_from_direction(target - origin)
}

pub(super) fn steer_velocity_with_constant_turn_rate(
    current_velocity: NiPoint3,
    target_direction: NiPoint3,
    radians_per_second: f32,
    delta_seconds: f32,
) -> Option<NiPoint3> {
    if !is_valid_non_negative_seconds(delta_seconds)
        || !radians_per_second.is_finite()
        || radians_per_second <= 0.0
        || !is_valid_point(current_velocity)
        || !is_valid_point(target_direction)
    {
        return None;
    }

    let current_speed = current_velocity.length();
    if !is_valid_positive_speed(current_speed) {
        return None;
    }

    let mut current_direction = current_velocity;
    current_direction.unitize();
    let mut target_direction = target_direction;
    if target_direction.unitize() <= f32::EPSILON {
        return None;
    }

    let dot = clamp_unit_scalar(current_direction.dot(target_direction));
    let total_angle = acosf(dot);
    let max_angle_step = radians_per_second * delta_seconds;
    if !max_angle_step.is_finite() {
        return None;
    }

    if total_angle <= max_angle_step || total_angle <= f32::EPSILON {
        return Some(target_direction * current_speed);
    }

    let blend = (max_angle_step / total_angle).clamp(0.0, 1.0);
    let mut blended_direction = current_direction * (1.0 - blend) + target_direction * blend;
    if blended_direction.unitize() <= f32::EPSILON {
        return Some(current_direction * current_speed);
    }

    Some(blended_direction * current_speed)
}

pub(super) fn steer_velocity_with_constant_acceleration(
    current_velocity: NiPoint3,
    target_direction: NiPoint3,
    desired_speed: f32,
    acceleration: f32,
) -> Option<NiPoint3> {
    if !acceleration.is_finite()
        || acceleration <= 0.0
        || !is_valid_positive_speed(desired_speed)
        || !is_valid_point(current_velocity)
        || !is_valid_point(target_direction)
    {
        return None;
    }

    let mut target_direction = target_direction;
    if target_direction.unitize() <= f32::EPSILON {
        return None;
    }

    let desired_velocity = target_direction * desired_speed;
    let mut delta = desired_velocity - current_velocity;
    if delta.unitize() <= f32::EPSILON {
        return Some(desired_velocity);
    }

    let mut steered_velocity = current_velocity + delta * acceleration;
    if steered_velocity.unitize() <= f32::EPSILON {
        return Some(desired_velocity);
    }

    Some(steered_velocity * desired_speed)
}

#[inline(always)]
pub(super) fn apply_projectile_rotation(projectile: &mut Projectile, rotation: ProjectileRot) {
    let current = projectile.base.get_angle();
    projectile
        .base
        .set_angle(&NiPoint3::new(rotation.x, current.y, rotation.z));
}

#[inline(always)]
pub(super) fn push_unique_handle(handles: &mut Vec<ProjectileHandle>, handle: ProjectileHandle) {
    if handle.has_value() && !handles.contains(&handle) {
        handles.push(handle);
    }
}

#[inline(always)]
pub(super) fn resolve_managed_handle(handle: ProjectileHandle) -> Option<Resolved<Projectile>> {
    if !handle.has_value() {
        return None;
    }

    Resolved::from_handle(handle)
}

#[inline(always)]
pub(super) fn launched_projectile(
    handle: ProjectileHandle,
    _caller: &'static str,
) -> Option<Resolved<Projectile>> {
    if !handle.has_value() {
        crate::defensive_sdk_warn!("{} failed to produce a projectile handle", _caller);
        return None;
    }

    let Some(projectile) = Resolved::from_handle(handle) else {
        crate::defensive_sdk_warn!(
            "{} produced handle {:08X} that could not be resolved immediately",
            _caller,
            handle.value()
        );
        return None;
    };

    Some(projectile)
}

#[inline(always)]
pub(super) fn snapshot_handle_bucket(
    bucket: &crate::re::BSTArray<ProjectileHandle>,
    _caller: &'static str,
) -> Vec<ProjectileHandle> {
    snapshot_contiguous_copied_named(bucket, _caller, projectile_handle_iteration_options())
        .into_iter()
        .filter(ProjectileHandle::has_value)
        .collect()
}

#[inline(always)]
pub(super) fn snapshot_impact(impact: &ProjectileImpactData) -> ProjectileImpactSnapshot {
    ProjectileImpactSnapshot {
        collidee_handle: impact.collidee,
        material: game_ptr(impact.material),
        collided_layer: impact.collided_layer,
        impact_result: impact.impact_result,
        desired_target_loc: impact.desired_target_loc,
        negative_velocity: impact.negative_velocity,
    }
}

#[inline(always)]
pub(super) fn projectile_kind_from_ptr(projectile: &Projectile) -> Option<ProjectileKind> {
    let projectile = game_ptr(projectile as *const Projectile as *mut Projectile);
    if projectile.try_cast::<ArrowProjectile>().is_some() {
        Some(ProjectileKind::Arrow)
    } else if projectile.try_cast::<BarrierProjectile>().is_some() {
        Some(ProjectileKind::Barrier)
    } else if projectile.try_cast::<BeamProjectile>().is_some() {
        Some(ProjectileKind::Beam)
    } else if projectile.try_cast::<ConeProjectile>().is_some() {
        Some(ProjectileKind::Cone)
    } else if projectile.try_cast::<FlameProjectile>().is_some() {
        Some(ProjectileKind::Flame)
    } else if projectile.try_cast::<GrenadeProjectile>().is_some() {
        Some(ProjectileKind::Grenade)
    } else if projectile.try_cast::<MissileProjectile>().is_some() {
        Some(ProjectileKind::Missile)
    } else {
        None
    }
}
