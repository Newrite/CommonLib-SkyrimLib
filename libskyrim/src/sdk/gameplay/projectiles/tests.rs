use alloc::vec;

use core_util::EnumSet;
use libm::{cosf, sinf};

use crate::re::{ACTOR_LOS_LOCATION, BGSProjectileType, NiPoint3, ProjectileHandle};
use crate::sdk::core::GamePtr;

use super::shared::{
    build_projectile_intercept_snapshot, projectile_rotation_from_direction,
    projectile_rotation_from_points, solve_constant_speed_intercept_time,
    steer_velocity_with_constant_acceleration, steer_velocity_with_constant_turn_rate,
};
use super::targeting::{nearest_target_index, view_aligned_target_index};
use super::{
    ProjectileKind, ProjectileManagerSnapshot, ProjectileSteeringBehavior,
    ProjectileTargetDisposition, ProjectileTargetSearchOptions, ProjectileTargetSnapshot,
};

#[test]
fn projectile_kind_prefers_more_specific_projectile_bits() {
    let arrow_and_missile = EnumSet::from_underlying(
        (BGSProjectileType::Arrow as u16) | (BGSProjectileType::Missile as u16),
    );
    let flame = EnumSet::from_underlying(BGSProjectileType::Flamethrower as u16);

    assert_eq!(
        ProjectileKind::from_types(arrow_and_missile),
        Some(ProjectileKind::Arrow)
    );
    assert_eq!(
        ProjectileKind::from_types(flame),
        Some(ProjectileKind::Flame)
    );
    assert_eq!(ProjectileKind::from_types(EnumSet::default()), None);
}

#[test]
fn manager_snapshot_deduplicates_handles_across_buckets() {
    let a = ProjectileHandle { handle: 0x10 };
    let b = ProjectileHandle { handle: 0x20 };
    let snapshot = ProjectileManagerSnapshot {
        unlimited: vec![a, b],
        limited: vec![b],
        pending: vec![ProjectileHandle::new()],
    };

    assert_eq!(snapshot.total_entries(), 4);
    assert_eq!(snapshot.managed_handles(), vec![a, b]);
    assert!(!snapshot.is_empty());
}

#[test]
fn target_search_options_default_to_hostile_actor_matching() {
    let options = ProjectileTargetSearchOptions::new(1024.0);

    assert_eq!(options.radius, 1024.0);
    assert_eq!(
        options.disposition,
        ProjectileTargetDisposition::HostileToCaster
    );
    assert!(!options.require_line_of_sight);
    assert_eq!(options.maximum_view_cone_degrees, None);
    assert_eq!(options.search_origin, None);
    assert_eq!(options.source_los_location, ACTOR_LOS_LOCATION::Head);
    assert_eq!(options.target_los_location, ACTOR_LOS_LOCATION::Torso);
}

#[test]
fn nearest_target_index_prefers_smallest_distance() {
    let candidates = [
        ProjectileTargetSnapshot {
            actor: GamePtr::null(),
            handle: Default::default(),
            position: NiPoint3::default(),
            velocity: NiPoint3::default(),
            speed: 0.0,
            distance: 10.0,
            squared_distance: 100.0,
            hostile_to_caster: true,
            aggressive_to_caster: false,
            line_of_sight_clear: None,
            view_alignment_dot: Some(0.2),
        },
        ProjectileTargetSnapshot {
            actor: GamePtr::null(),
            handle: Default::default(),
            position: NiPoint3::default(),
            velocity: NiPoint3::default(),
            speed: 0.0,
            distance: 5.0,
            squared_distance: 25.0,
            hostile_to_caster: true,
            aggressive_to_caster: true,
            line_of_sight_clear: Some(true),
            view_alignment_dot: Some(0.9),
        },
    ];

    assert_eq!(nearest_target_index(&candidates), Some(1));
}

#[test]
fn view_aligned_target_index_prefers_best_alignment_then_distance() {
    let candidates = [
        ProjectileTargetSnapshot {
            actor: GamePtr::null(),
            handle: Default::default(),
            position: NiPoint3::default(),
            velocity: NiPoint3::default(),
            speed: 0.0,
            distance: 20.0,
            squared_distance: 400.0,
            hostile_to_caster: true,
            aggressive_to_caster: false,
            line_of_sight_clear: None,
            view_alignment_dot: Some(0.8),
        },
        ProjectileTargetSnapshot {
            actor: GamePtr::null(),
            handle: Default::default(),
            position: NiPoint3::default(),
            velocity: NiPoint3::default(),
            speed: 0.0,
            distance: 12.0,
            squared_distance: 144.0,
            hostile_to_caster: true,
            aggressive_to_caster: false,
            line_of_sight_clear: None,
            view_alignment_dot: Some(0.95),
        },
        ProjectileTargetSnapshot {
            actor: GamePtr::null(),
            handle: Default::default(),
            position: NiPoint3::default(),
            velocity: NiPoint3::default(),
            speed: 0.0,
            distance: 6.0,
            squared_distance: 36.0,
            hostile_to_caster: true,
            aggressive_to_caster: false,
            line_of_sight_clear: None,
            view_alignment_dot: Some(0.95),
        },
    ];

    assert_eq!(view_aligned_target_index(&candidates), Some(2));
}

#[test]
fn target_snapshot_anticipated_position_uses_velocity() {
    let snapshot = ProjectileTargetSnapshot {
        actor: GamePtr::null(),
        handle: Default::default(),
        position: NiPoint3::new(10.0, 0.0, -2.0),
        velocity: NiPoint3::new(4.0, 2.0, 1.0),
        speed: NiPoint3::new(4.0, 2.0, 1.0).length(),
        distance: 0.0,
        squared_distance: 0.0,
        hostile_to_caster: true,
        aggressive_to_caster: false,
        line_of_sight_clear: None,
        view_alignment_dot: None,
    };

    assert_eq!(
        snapshot.anticipated_position(0.5),
        Some(NiPoint3::new(12.0, 1.0, -1.5))
    );
    assert!(snapshot.is_moving());
}

#[test]
fn intercept_solver_prefers_smallest_non_negative_root() {
    let time = solve_constant_speed_intercept_time(
        NiPoint3::default(),
        NiPoint3::new(10.0, 0.0, 0.0),
        NiPoint3::new(1.0, 0.0, 0.0),
        5.0,
    );

    assert_eq!(time, Some(2.5));
}

#[test]
fn intercept_snapshot_applies_initial_prediction_before_solving() {
    let snapshot = build_projectile_intercept_snapshot(
        NiPoint3::default(),
        NiPoint3::new(10.0, 0.0, 0.0),
        NiPoint3::new(1.0, 0.0, 0.0),
        5.0,
        1.0,
    )
    .expect("intercept snapshot should exist");

    assert!((snapshot.travel_time_seconds - 2.75).abs() < 0.0001);
    assert!((snapshot.total_lead_seconds() - 3.75).abs() < 0.0001);
    assert!((snapshot.intercept_point.x - 13.75).abs() < 0.0001);
    assert!((snapshot.direction.length() - 1.0).abs() < 0.0001);
}

#[test]
fn rotation_from_direction_matches_projectile_aim_convention() {
    let direction = NiPoint3::new(1.0, 1.0, 0.5);
    let rotation =
        projectile_rotation_from_direction(direction).expect("rotation should be created");

    let cos_pitch = cosf(rotation.x);
    let rebuilt = NiPoint3::new(
        sinf(rotation.z) * cos_pitch,
        cosf(rotation.z) * cos_pitch,
        sinf(rotation.x),
    );
    let mut rebuilt = rebuilt;
    rebuilt.unitize();
    let mut direction = direction;
    direction.unitize();

    assert!((rebuilt.x - direction.x).abs() < 0.0001);
    assert!((rebuilt.y - direction.y).abs() < 0.0001);
    assert!((rebuilt.z - direction.z).abs() < 0.0001);
}

#[test]
fn rotation_from_points_matches_expected_cardinal_angles() {
    let rotation = projectile_rotation_from_points(
        NiPoint3::new(0.0, 0.0, 0.0),
        NiPoint3::new(0.0, 10.0, 0.0),
    )
    .expect("rotation should exist");
    assert!(rotation.x.abs() < 0.0001);
    assert!(rotation.z.abs() < 0.0001);

    let rotation = projectile_rotation_from_points(
        NiPoint3::new(0.0, 0.0, 0.0),
        NiPoint3::new(10.0, 0.0, 0.0),
    )
    .expect("rotation should exist");
    assert!(rotation.x.abs() < 0.0001);
    assert!((rotation.z - core::f32::consts::FRAC_PI_2).abs() < 0.0001);
}

#[test]
fn constant_turn_rate_steering_preserves_speed_and_clamps_turn() {
    let velocity = steer_velocity_with_constant_turn_rate(
        NiPoint3::new(0.0, 10.0, 0.0),
        NiPoint3::new(10.0, 0.0, 0.0),
        core::f32::consts::FRAC_PI_2,
        0.5,
    )
    .expect("steered velocity should exist");
    let mut direction = velocity;
    let speed = direction.unitize();

    assert!((speed - 10.0).abs() < 0.0001);
    assert!(
        (direction.dot(NiPoint3::new(0.0, 1.0, 0.0)) - cosf(core::f32::consts::FRAC_PI_4)).abs()
            < 0.0001
    );
}

#[test]
fn constant_acceleration_steering_preserves_requested_speed() {
    let velocity = steer_velocity_with_constant_acceleration(
        NiPoint3::new(0.0, 8.0, 0.0),
        NiPoint3::new(1.0, 0.0, 0.0),
        12.0,
        2.0,
    )
    .expect("steered velocity should exist");

    assert!((velocity.length() - 12.0).abs() < 0.0001);
    assert!(velocity.x > 0.0);
}

#[test]
fn steering_behavior_constructors_preserve_parameters() {
    assert_eq!(
        ProjectileSteeringBehavior::constant_turn_rate_radians(3.0),
        ProjectileSteeringBehavior::ConstantTurnRateRadians {
            radians_per_second: 3.0
        }
    );
    assert_eq!(
        ProjectileSteeringBehavior::constant_acceleration(5.0),
        ProjectileSteeringBehavior::ConstantAcceleration { acceleration: 5.0 }
    );
}
