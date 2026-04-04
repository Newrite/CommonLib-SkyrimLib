use crate::re::{NiPoint3, ObjectRefHandle, Projectile, ProjectileRot, TESObjectREFR};

use super::runtime::projectile_desired_target;
use super::shared::{
    anticipated_position_from_velocity, apply_projectile_rotation,
    build_projectile_intercept_snapshot, is_valid_non_negative_seconds, is_valid_point,
    is_valid_positive_speed, projectile_current_linear_speed, projectile_effective_range,
    projectile_rotation_from_direction, projectile_rotation_from_points, reference_linear_velocity,
    steer_velocity_with_constant_acceleration, steer_velocity_with_constant_turn_rate,
    target_reference_is_alive,
};
use super::targeting::reacquire_projectile_desired_target;
use super::types::{
    ProjectileInterceptSnapshot, ProjectileRetargetStrategy, ProjectileSteeringBehavior,
    ProjectileTargetSearchOptions, ProjectileTargetSnapshot,
};

#[inline(always)]
fn validated_non_negative_seconds(
    delta_seconds: f32,
    _caller: &'static str,
    _field_name: &'static str,
) -> Option<f32> {
    if is_valid_non_negative_seconds(delta_seconds) {
        Some(delta_seconds)
    } else {
        crate::defensive_sdk_warn!(
            "{} ignored negative or non-finite {}={}",
            _caller,
            _field_name,
            delta_seconds
        );
        None
    }
}

#[inline(always)]
fn validated_reference_position(
    reference: &TESObjectREFR,
    _caller: &'static str,
    _subject_name: &'static str,
) -> Option<NiPoint3> {
    let position = reference.get_position();
    if is_valid_point(position) {
        Some(position)
    } else {
        crate::defensive_sdk_warn!(
            "{} ignored {} with non-finite position",
            _caller,
            _subject_name
        );
        None
    }
}

#[inline(always)]
fn validated_projectile_origin(projectile: &Projectile, _caller: &'static str) -> Option<NiPoint3> {
    let origin = projectile.base.get_position();
    if is_valid_point(origin) {
        Some(origin)
    } else {
        crate::defensive_sdk_warn!("{} ignored projectile with non-finite origin", _caller);
        None
    }
}

#[inline(always)]
fn validated_projectile_speed(projectile: &Projectile, _caller: &'static str) -> Option<f32> {
    let speed = projectile.get_speed();
    if is_valid_positive_speed(speed) {
        Some(speed)
    } else {
        crate::defensive_sdk_warn!(
            "{} ignored projectile with non-positive or non-finite speed={}",
            _caller,
            speed
        );
        None
    }
}

#[inline(always)]
fn validated_target_point(
    target_point: NiPoint3,
    _caller: &'static str,
    _subject_name: &'static str,
) -> Option<NiPoint3> {
    if is_valid_point(target_point) {
        Some(target_point)
    } else {
        crate::defensive_sdk_warn!("{} ignored non-finite {}", _caller, _subject_name);
        None
    }
}

#[inline(always)]
fn validated_target_points(
    origin: NiPoint3,
    target: NiPoint3,
    caller: &'static str,
) -> Option<(NiPoint3, NiPoint3)> {
    Some((
        validated_target_point(origin, caller, "origin")?,
        validated_target_point(target, caller, "target")?,
    ))
}

#[inline(always)]
fn build_intercept_snapshot_from_velocity(
    origin: NiPoint3,
    target_position: NiPoint3,
    target_velocity: NiPoint3,
    projectile_speed: f32,
    initial_prediction_seconds: f32,
    caller: &'static str,
) -> Option<ProjectileInterceptSnapshot> {
    validated_non_negative_seconds(
        initial_prediction_seconds,
        caller,
        "initial_prediction_seconds",
    )?;
    if !is_valid_point(origin) || !is_valid_point(target_position) {
        crate::defensive_sdk_warn!(
            "{} ignored non-finite projectile origin or target position",
            caller
        );
        return None;
    }

    if !is_valid_positive_speed(projectile_speed) {
        crate::defensive_sdk_warn!(
            "{} ignored projectile with non-positive or non-finite speed={}",
            caller,
            projectile_speed
        );
        return None;
    }

    build_projectile_intercept_snapshot(
        origin,
        target_position,
        target_velocity,
        projectile_speed,
        initial_prediction_seconds,
    )
}

#[inline(always)]
pub fn projectile_target_linear_velocity(target: &TESObjectREFR) -> NiPoint3 {
    reference_linear_velocity(target)
}

pub fn anticipated_projectile_target_position(
    target: &TESObjectREFR,
    delta_seconds: f32,
) -> Option<NiPoint3> {
    let caller = "sdk::gameplay::projectiles::anticipated_projectile_target_position()";
    let delta_seconds = validated_non_negative_seconds(delta_seconds, caller, "delta_seconds")?;
    let position = validated_reference_position(target, caller, "target")?;

    anticipated_position_from_velocity(
        position,
        projectile_target_linear_velocity(target),
        delta_seconds,
    )
}

pub fn projectile_intercept_snapshot(
    projectile: &Projectile,
    target: &TESObjectREFR,
    initial_prediction_seconds: f32,
) -> Option<ProjectileInterceptSnapshot> {
    let caller = "sdk::gameplay::projectiles::projectile_intercept_snapshot()";
    build_intercept_snapshot_from_velocity(
        validated_projectile_origin(projectile, caller)?,
        validated_reference_position(target, caller, "target")?,
        projectile_target_linear_velocity(target),
        validated_projectile_speed(projectile, caller)?,
        initial_prediction_seconds,
        caller,
    )
}

pub fn projectile_intercept_snapshot_for_target(
    projectile: &Projectile,
    target: ProjectileTargetSnapshot,
    initial_prediction_seconds: f32,
) -> Option<ProjectileInterceptSnapshot> {
    let caller = "sdk::gameplay::projectiles::projectile_intercept_snapshot_for_target()";
    build_intercept_snapshot_from_velocity(
        validated_projectile_origin(projectile, caller)?,
        target.position,
        target.velocity,
        validated_projectile_speed(projectile, caller)?,
        initial_prediction_seconds,
        caller,
    )
}

pub fn projectile_rotation_to_direction(direction: NiPoint3) -> Option<ProjectileRot> {
    let direction = validated_target_point(
        direction,
        "sdk::gameplay::projectiles::projectile_rotation_to_direction()",
        "direction",
    )?;

    projectile_rotation_from_direction(direction)
}

pub fn projectile_rotation_to_point(origin: NiPoint3, target: NiPoint3) -> Option<ProjectileRot> {
    let (origin, target) = validated_target_points(
        origin,
        target,
        "sdk::gameplay::projectiles::projectile_rotation_to_point()",
    )?;

    projectile_rotation_from_points(origin, target)
}

#[inline(always)]
pub fn set_projectile_desired_target(projectile: &mut Projectile, target: &TESObjectREFR) {
    projectile.get_projectile_runtime_data_mut().desired_target = target.get_handle();
}

#[inline(always)]
pub fn clear_projectile_desired_target(projectile: &mut Projectile) {
    projectile.get_projectile_runtime_data_mut().desired_target = ObjectRefHandle::new();
}

pub fn should_keep_projectile_desired_target(projectile: &Projectile) -> bool {
    let desired_target = projectile_desired_target(projectile);
    let Some(desired_target) = desired_target.as_ref() else {
        return false;
    };

    if desired_target.is_disabled() || desired_target.is_marked_for_deletion() {
        return false;
    }

    if !target_reference_is_alive(desired_target) {
        return false;
    }

    let Some(range) = projectile_effective_range(projectile) else {
        return true;
    };

    let origin = projectile.base.get_position();
    let target_position = desired_target.get_position();
    if !is_valid_point(origin) || !is_valid_point(target_position) {
        return false;
    }

    origin.get_squared_distance(target_position) <= range * range
}

pub fn refresh_projectile_desired_target(
    projectile: &mut Projectile,
    strategy: ProjectileRetargetStrategy,
    options: ProjectileTargetSearchOptions,
) -> Option<ProjectileTargetSnapshot> {
    reacquire_projectile_desired_target(projectile, strategy, options)
}

pub fn align_projectile_rotation_to_linear_velocity(
    projectile: &mut Projectile,
) -> Option<ProjectileRot> {
    let linear_velocity = projectile.get_projectile_runtime_data().linear_velocity;
    let rotation = projectile_rotation_from_direction(linear_velocity)?;
    apply_projectile_rotation(projectile, rotation);
    Some(rotation)
}

pub fn aim_projectile_at_point(
    projectile: &mut Projectile,
    target_point: NiPoint3,
) -> Option<ProjectileRot> {
    let caller = "sdk::gameplay::projectiles::aim_projectile_at_point()";
    let origin = validated_projectile_origin(projectile, caller)?;
    let target_point = validated_target_point(target_point, caller, "target point")?;

    let rotation = projectile_rotation_from_points(origin, target_point)?;
    apply_projectile_rotation(projectile, rotation);
    Some(rotation)
}

pub fn aim_projectile_at_target(
    projectile: &mut Projectile,
    target: &TESObjectREFR,
    initial_prediction_seconds: f32,
) -> Option<ProjectileRot> {
    let aim_point = projectile_intercept_snapshot(projectile, target, initial_prediction_seconds)
        .map(|snapshot| snapshot.intercept_point)
        .or_else(|| anticipated_projectile_target_position(target, initial_prediction_seconds))?;
    aim_projectile_at_point(projectile, aim_point)
}

pub fn aim_projectile_at_desired_target(
    projectile: &mut Projectile,
    initial_prediction_seconds: f32,
) -> Option<ProjectileRot> {
    let desired_target = projectile_desired_target(projectile);
    let target = desired_target.as_ref()?;
    aim_projectile_at_target(projectile, target, initial_prediction_seconds)
}

pub fn steer_projectile_towards_point(
    projectile: &mut Projectile,
    target_point: NiPoint3,
    delta_seconds: f32,
    behavior: ProjectileSteeringBehavior,
) -> Option<NiPoint3> {
    let caller = "sdk::gameplay::projectiles::steer_projectile_towards_point()";
    let delta_seconds = validated_non_negative_seconds(delta_seconds, caller, "delta_seconds")?;
    let origin = validated_projectile_origin(projectile, caller)?;
    let target_point = validated_target_point(target_point, caller, "target point")?;

    let current_velocity = projectile.get_projectile_runtime_data().linear_velocity;
    let target_direction = target_point - origin;
    let desired_speed = projectile_current_linear_speed(projectile)?;
    let steered_velocity = match behavior {
        ProjectileSteeringBehavior::ConstantTurnRateRadians { radians_per_second } => {
            steer_velocity_with_constant_turn_rate(
                current_velocity,
                target_direction,
                radians_per_second,
                delta_seconds,
            )?
        }
        ProjectileSteeringBehavior::ConstantAcceleration { acceleration } => {
            steer_velocity_with_constant_acceleration(
                current_velocity,
                target_direction,
                desired_speed,
                acceleration,
            )?
        }
    };

    projectile.get_projectile_runtime_data_mut().linear_velocity = steered_velocity;
    let _ = align_projectile_rotation_to_linear_velocity(projectile);
    Some(steered_velocity)
}

pub fn steer_projectile_towards_target(
    projectile: &mut Projectile,
    target: &TESObjectREFR,
    delta_seconds: f32,
    behavior: ProjectileSteeringBehavior,
    initial_prediction_seconds: f32,
) -> Option<ProjectileInterceptSnapshot> {
    let snapshot = projectile_intercept_snapshot(projectile, target, initial_prediction_seconds)?;
    let _ = steer_projectile_towards_point(
        projectile,
        snapshot.intercept_point,
        delta_seconds,
        behavior,
    )?;
    Some(snapshot)
}

pub fn steer_projectile_towards_desired_target(
    projectile: &mut Projectile,
    delta_seconds: f32,
    behavior: ProjectileSteeringBehavior,
    initial_prediction_seconds: f32,
) -> Option<ProjectileInterceptSnapshot> {
    let desired_target = projectile_desired_target(projectile);
    let target = desired_target.as_ref()?;
    steer_projectile_towards_target(
        projectile,
        target,
        delta_seconds,
        behavior,
        initial_prediction_seconds,
    )
}
