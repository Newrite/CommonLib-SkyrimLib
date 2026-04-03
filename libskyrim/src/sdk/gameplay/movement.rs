//! Movement / translation helpers built on top of `ExtraRefrPath`.
//!
//! This SDK layer keeps the low-level `ExtraRefrPath` and
//! `TESObjectCELL::add_translate_object(...)` seams explicit in `re`, while
//! offering a narrow ergonomic surface for the common "translate this actor or
//! reference to a point" workflows.

use crate::re::{
    ExtraRefrPath, ExtraRefrPathPathType, NiPoint3, TESObjectREFR, TESObjectREFRChangeFlags,
};
use crate::sdk::core::ResolvableHandle;
use libm::{cosf, sinf};

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RelativeOffsetSide {
    Behind,
    Left,
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

pub fn point_behind(target: &TESObjectREFR, distance: f32) -> NiPoint3 {
    relative_offset_point(target, RelativeOffsetSide::Behind, distance)
}

pub fn point_left_of(target: &TESObjectREFR, distance: f32) -> NiPoint3 {
    relative_offset_point(target, RelativeOffsetSide::Left, distance)
}

pub fn point_right_of(target: &TESObjectREFR, distance: f32) -> NiPoint3 {
    relative_offset_point(target, RelativeOffsetSide::Right, distance)
}

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
