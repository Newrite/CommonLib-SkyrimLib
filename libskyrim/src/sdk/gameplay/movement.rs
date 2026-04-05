//! Movement / translation helpers built on top of `ExtraRefrPath`.
//!
//! This SDK layer keeps the low-level `ExtraRefrPath` and
//! `TESObjectCELL::add_translate_object(...)` seams explicit in `re`, while
//! offering a narrow ergonomic surface for the common "translate this actor or
//! reference to a point" workflows.
//!
//! Reach for this module when the plugin wants engine-driven translation, not
//! direct teleports:
//!
//! - compute relative points around a target with [`point_behind`],
//!   [`point_left_of`], or [`point_right_of`]
//! - move one reference toward a world-space point with [`translate_to`]
//! - move one reference relative to another with [`translate_behind`],
//!   [`translate_left`], or [`translate_right`]
//!
//! Example:
//!
//! ```rust,ignore
//! use libskyrim::sdk::gameplay::movement;
//!
//! fn sidestep(
//!     reference: &mut libskyrim::re::TESObjectREFR,
//!     target: &libskyrim::re::TESObjectREFR,
//! ) {
//!     let _ = movement::translate_left(reference, target, 96.0, 900.0, 4.0);
//! }
//! ```

use crate::re::{
    ExtraRefrPath, ExtraRefrPathPathType, NiPoint3, TESObjectREFR, TESObjectREFRChangeFlags,
};
use crate::sdk::core::ResolvableHandle;
use libm::{cosf, sinf};

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RelativeOffsetSide {
    /// Offset behind the target's facing direction.
    Behind,
    /// Offset to the target's local left.
    Left,
    /// Offset to the target's local right.
    Right,
}

#[inline(always)]
fn is_valid_scalar(value: f32) -> bool {
    value.is_finite()
}

#[inline(always)]
fn is_valid_point(value: NiPoint3) -> bool {
    is_valid_scalar(value.x) && is_valid_scalar(value.y) && is_valid_scalar(value.z)
}

fn ensure_translation_path(reference: &mut TESObjectREFR) -> *mut ExtraRefrPath {
    let mut extra_path = reference.extra_list.get_by_type_typed::<ExtraRefrPath>();
    if extra_path.is_null() {
        extra_path = ExtraRefrPath::create();
        reference.extra_list.add(extra_path.cast());
        let _ = reference
            .base
            .add_change(TESObjectREFRChangeFlags::GAME_ONLY_EXTRA.bits());
    }

    unsafe {
        (*extra_path).current_parameter = 0.0;
        (*extra_path).type_ = ExtraRefrPathPathType::Translation.into();
    }

    extra_path
}

#[inline(always)]
fn relative_offset_point(
    target: &TESObjectREFR,
    side: RelativeOffsetSide,
    distance: f32,
) -> NiPoint3 {
    let origin = target.get_position();
    let yaw = target.get_angle_z();
    let yaw_sin = sinf(yaw);
    let yaw_cos = cosf(yaw);
    match side {
        RelativeOffsetSide::Behind => NiPoint3::new(
            origin.x - distance * yaw_sin,
            origin.y - distance * yaw_cos,
            origin.z,
        ),
        RelativeOffsetSide::Left => NiPoint3::new(
            origin.x - distance * yaw_cos,
            origin.y + distance * yaw_sin,
            origin.z,
        ),
        RelativeOffsetSide::Right => NiPoint3::new(
            origin.x + distance * yaw_cos,
            origin.y - distance * yaw_sin,
            origin.z,
        ),
    }
}

/// Compute a point directly behind the target at the supplied distance.
///
/// This is the cheapest relative-position helper when plugin code only needs a
/// world-space point and does not want to start a translation yet.
pub fn point_behind(target: &TESObjectREFR, distance: f32) -> NiPoint3 {
    relative_offset_point(target, RelativeOffsetSide::Behind, distance)
}

/// Compute a point to the left of the target at the supplied distance.
///
/// This is useful for sidestep/dodge candidates before deciding whether to use
/// movement translation, teleportation, or spatial validation.
pub fn point_left_of(target: &TESObjectREFR, distance: f32) -> NiPoint3 {
    relative_offset_point(target, RelativeOffsetSide::Left, distance)
}

/// Compute a point to the right of the target at the supplied distance.
///
/// This is useful for sidestep/dodge candidates before deciding whether to use
/// movement translation, teleportation, or spatial validation.
pub fn point_right_of(target: &TESObjectREFR, distance: f32) -> NiPoint3 {
    relative_offset_point(target, RelativeOffsetSide::Right, distance)
}

/// Translate a reference toward a world-space position with explicit angular
/// and speed parameters.
///
/// Use this when the plugin wants engine-driven movement interpolation through
/// `ExtraRefrPath` instead of an immediate teleport.
pub fn translate_to(
    reference: &mut TESObjectREFR,
    position: NiPoint3,
    angle_radians: NiPoint3,
    speed: f32,
    max_rotation_speed_radians: f32,
) -> bool {
    if reference.get_parent_cell().is_null() {
        crate::defensive_sdk_warn!(
            "sdk::gameplay::movement::translate_to() skipped because the reference has no parent cell"
        );
        return false;
    }

    if reference.get_3d().is_null() {
        crate::defensive_sdk_warn!(
            "sdk::gameplay::movement::translate_to() skipped because the reference has no loaded 3D"
        );
        return false;
    }

    if !is_valid_point(position) || !is_valid_point(angle_radians) {
        crate::defensive_sdk_warn!(
            "sdk::gameplay::movement::translate_to() ignored non-finite position or angle"
        );
        return false;
    }

    if !speed.is_finite()
        || speed <= 0.0
        || !max_rotation_speed_radians.is_finite()
        || max_rotation_speed_radians <= 0.0
    {
        crate::defensive_sdk_warn!(
            "sdk::gameplay::movement::translate_to() ignored non-positive or non-finite speed inputs"
        );
        return false;
    }

    let extra_path = ensure_translation_path(reference);
    unsafe {
        (*extra_path).setup_translation(
            &reference.get_position(),
            &reference.get_angle(),
            &position,
            &angle_radians,
            speed,
            max_rotation_speed_radians,
        );
    }

    let mut handle = reference.create_ref_handle();
    if handle.is_null() {
        crate::defensive_sdk_warn!(
            "sdk::gameplay::movement::translate_to() skipped because the reference handle was null"
        );
        return false;
    }

    let cell = unsafe { &mut *reference.get_parent_cell() };
    cell.add_translate_object(&mut handle)
}

/// Translate a reference toward a point defined relative to another target.
///
/// This is the general relative-translation entry point used by the narrower
/// `translate_behind/left/right` helpers.
///
/// Reach for this when the target-relative side is dynamic in plugin code and
/// the narrower directional helpers would only add branching at the call site.
pub fn translate_relative_to(
    reference: &mut TESObjectREFR,
    target: &TESObjectREFR,
    side: RelativeOffsetSide,
    distance: f32,
    angle_radians: NiPoint3,
    speed: f32,
    max_rotation_speed_radians: f32,
) -> bool {
    if !distance.is_finite() || distance <= 0.0 {
        crate::defensive_sdk_warn!(
            "sdk::gameplay::movement::translate_relative_to() ignored non-positive or non-finite distance={}",
            distance
        );
        return false;
    }

    translate_to(
        reference,
        relative_offset_point(target, side, distance),
        angle_radians,
        speed,
        max_rotation_speed_radians,
    )
}

/// Translate a reference behind a target.
///
/// This is the narrow convenience helper for the common "move behind target"
/// gameplay pattern.
pub fn translate_behind(
    reference: &mut TESObjectREFR,
    target: &TESObjectREFR,
    distance: f32,
    speed: f32,
    max_rotation_speed_radians: f32,
) -> bool {
    translate_relative_to(
        reference,
        target,
        RelativeOffsetSide::Behind,
        distance,
        reference.get_angle(),
        speed,
        max_rotation_speed_radians,
    )
}

/// Translate a reference to the left of a target.
///
/// This is the narrow convenience helper for sidestep/strafe style movement.
pub fn translate_left(
    reference: &mut TESObjectREFR,
    target: &TESObjectREFR,
    distance: f32,
    speed: f32,
    max_rotation_speed_radians: f32,
) -> bool {
    translate_relative_to(
        reference,
        target,
        RelativeOffsetSide::Left,
        distance,
        reference.get_angle(),
        speed,
        max_rotation_speed_radians,
    )
}

/// Translate a reference to the right of a target.
///
/// This is the narrow convenience helper for sidestep/strafe style movement.
pub fn translate_right(
    reference: &mut TESObjectREFR,
    target: &TESObjectREFR,
    distance: f32,
    speed: f32,
    max_rotation_speed_radians: f32,
) -> bool {
    translate_relative_to(
        reference,
        target,
        RelativeOffsetSide::Right,
        distance,
        reference.get_angle(),
        speed,
        max_rotation_speed_radians,
    )
}
