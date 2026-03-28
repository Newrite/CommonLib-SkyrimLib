//! Havok and physics-oriented helpers.

use crate::re::{
    CFilter, NiPoint3, TESHavokUtilities, TESObjectCELL, TESObjectREFR, bhkPickData, bhkWorld,
    hkVector4, hkpCollidable,
};
use crate::sdk::core::GamePtr;

#[derive(Debug)]
pub struct RaycastHit {
    pub hit_fraction: f32,
    pub hit_point: NiPoint3,
    pub normal: NiPoint3,
    pub root_collidable: *const hkpCollidable,
    pub reference: GamePtr<TESObjectREFR>,
}

#[inline(always)]
pub const fn raw_filter(filter: u32) -> CFilter {
    CFilter { filter }
}

#[inline(always)]
pub fn raycast_segment(
    cell: &TESObjectCELL,
    from: NiPoint3,
    to: NiPoint3,
    filter: CFilter,
) -> Option<RaycastHit> {
    let world = unsafe { cell.get_bhk_world().as_mut() }?;
    let world_scale = bhkWorld::get_world_scale();

    let mut pick_data = bhkPickData::default();
    pick_data.ray_input.from = hkVector4::from(from * world_scale);
    pick_data.ray_input.to = hkVector4::from(to * world_scale);
    pick_data.ray_input.filter_info = filter;

    if !world.pick_object_with(&mut pick_data) || !pick_data.ray_output.has_hit() {
        return None;
    }

    let hit_output = &pick_data.ray_output.base.base;
    let hit_point = from + (to - from) * hit_output.hit_fraction;
    let root_collidable = pick_data.ray_output.root_collidable;
    let reference = if root_collidable.is_null() {
        GamePtr::null()
    } else {
        let raw = unsafe { TESHavokUtilities::find_collidable_ref(&*root_collidable) };
        unsafe { GamePtr::from_raw(raw) }
    };

    Some(RaycastHit {
        hit_fraction: hit_output.hit_fraction,
        hit_point,
        normal: ni_point3_from_hk(&hit_output.normal),
        root_collidable,
        reference,
    })
}

#[inline(always)]
pub fn raycast_segment_mask(
    cell: &TESObjectCELL,
    from: NiPoint3,
    to: NiPoint3,
    filter: u32,
) -> Option<RaycastHit> {
    raycast_segment(cell, from, to, raw_filter(filter))
}

#[inline(always)]
pub fn raycast_delta(
    cell: &TESObjectCELL,
    from: NiPoint3,
    delta: NiPoint3,
    filter: CFilter,
) -> Option<RaycastHit> {
    raycast_segment(cell, from, from + delta, filter)
}

#[inline(always)]
pub fn raycast_delta_mask(
    cell: &TESObjectCELL,
    from: NiPoint3,
    delta: NiPoint3,
    filter: u32,
) -> Option<RaycastHit> {
    raycast_delta(cell, from, delta, raw_filter(filter))
}

#[inline(always)]
pub fn raycast_from_reference(
    reference: &TESObjectREFR,
    delta: NiPoint3,
    filter: CFilter,
) -> Option<RaycastHit> {
    let cell = unsafe { reference.get_parent_cell().as_ref() }?;
    raycast_delta(cell, reference.get_position(), delta, filter)
}

#[inline(always)]
pub fn raycast_from_reference_mask(
    reference: &TESObjectREFR,
    delta: NiPoint3,
    filter: u32,
) -> Option<RaycastHit> {
    raycast_from_reference(reference, delta, raw_filter(filter))
}

#[inline(always)]
fn ni_point3_from_hk(vector: &hkVector4) -> NiPoint3 {
    NiPoint3::new(vector.quad[0], vector.quad[1], vector.quad[2])
}
