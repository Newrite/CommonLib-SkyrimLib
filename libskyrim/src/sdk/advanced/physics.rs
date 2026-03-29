//! Havok and physics-oriented helpers.
//!
//! This module intentionally separates two layers that many C++ SKSE projects
//! blur together:
//!
//! - [`CFilter`] is the Havok query filter info sent into the raycast.
//! - [`LayerMask`] is a post-query collision-layer mask used to decide which
//!   hits should count as blockers.

use alloc::vec::Vec;
use core::cmp::Ordering;
use core::ptr;

use crate::re::{
    Actor, BSReadLockGuard, CFilter, ColLayer, NiAVObject, NiPoint3, TESHavokUtilities,
    TESObjectCELL, TESObjectREFR, bhkPickData, bhkWorld, hkVector4, hkpAllRayHitCollector,
    hkpCollidable, hkpWorldRayCastOutput,
};
use crate::sdk::core::Resolved;

/// Post-query collision-layer mask used to decide which Havok hits should
/// count as blockers.
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LayerMask {
    bits: u64,
}

impl LayerMask {
    #[inline(always)]
    pub const fn empty() -> Self {
        Self { bits: 0 }
    }

    #[inline(always)]
    pub const fn all() -> Self {
        Self { bits: u64::MAX }
    }

    #[inline(always)]
    pub const fn from_bits(bits: u64) -> Self {
        Self { bits }
    }

    #[inline(always)]
    pub const fn bits(self) -> u64 {
        self.bits
    }

    #[inline(always)]
    pub const fn from_layer(layer: ColLayer) -> Self {
        Self {
            bits: layer_bit(layer),
        }
    }

    #[inline(always)]
    pub fn from_slice(layers: &[ColLayer]) -> Self {
        let mut mask = Self::empty();
        for &layer in layers {
            mask.insert(layer);
        }
        mask
    }

    #[inline(always)]
    pub const fn contains_layer(self, layer: ColLayer) -> bool {
        (self.bits & layer_bit(layer)) != 0
    }

    #[inline(always)]
    pub fn contains_hit(self, hit: &RaycastHit) -> bool {
        hit.matches_layers(self)
    }

    #[inline(always)]
    pub fn insert(&mut self, layer: ColLayer) {
        self.bits |= layer_bit(layer);
    }

    #[inline(always)]
    pub fn remove(&mut self, layer: ColLayer) {
        self.bits &= !layer_bit(layer);
    }

    #[inline(always)]
    pub const fn intersects(self, other: Self) -> bool {
        (self.bits & other.bits) != 0
    }
}

impl From<ColLayer> for LayerMask {
    #[inline(always)]
    fn from(value: ColLayer) -> Self {
        Self::from_layer(value)
    }
}

/// Converted world-space raycast hit.
///
/// Low-level Havok-owned pointers stay raw on purpose. Runtime references can
/// be upgraded into [`Resolved<TESObjectREFR>`] on demand through
/// [`RaycastHit::resolved_reference`].
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RaycastHit {
    pub hit_fraction: f32,
    pub hit_point: NiPoint3,
    pub normal: NiPoint3,
    pub root_collidable: *const hkpCollidable,
    pub collision_layer: Option<ColLayer>,
    pub reference: *mut TESObjectREFR,
    pub object: *mut NiAVObject,
}

impl RaycastHit {
    #[inline(always)]
    pub fn resolved_reference(self) -> Option<Resolved<TESObjectREFR>> {
        Resolved::try_from_ptr(self.reference)
    }

    #[inline(always)]
    pub fn matches_layers(self, layers: LayerMask) -> bool {
        self.collision_layer
            .is_some_and(|layer| layers.contains_layer(layer))
    }

    #[inline(always)]
    pub fn offset_along_normal(self, distance: f32) -> NiPoint3 {
        offset_point_along_normal(self.hit_point, self.normal, distance)
    }

    #[inline(always)]
    pub fn backoff_along_segment(self, from: NiPoint3, to: NiPoint3, distance: f32) -> NiPoint3 {
        backoff_point(from, to, self.hit_fraction, distance)
    }
}

#[inline(always)]
pub const fn raw_filter(filter: u32) -> CFilter {
    CFilter { filter }
}

/// Build a minimal Havok filter that casts as the given collision layer.
#[inline(always)]
pub fn filter_for_layer(layer: ColLayer) -> CFilter {
    let mut filter = CFilter::default();
    filter.set_collision_layer(layer);
    filter
}

/// Reuse the actor's system-group filter data but cast as a different query
/// layer. This matches the common C++ pattern used for LOS / item-pick rays.
#[inline(always)]
pub fn actor_filter(actor: &Actor, layer: ColLayer) -> CFilter {
    let mut filter = actor.get_collision_filter();
    filter.set_collision_layer(layer);
    filter
}

#[inline(always)]
pub fn actor_line_of_sight_filter(actor: &Actor) -> CFilter {
    actor_filter(actor, ColLayer::LineOfSight)
}

#[inline(always)]
pub fn actor_item_pick_filter(actor: &Actor) -> CFilter {
    actor_filter(actor, ColLayer::ItemPick)
}

#[inline(always)]
pub fn raycast_segment(
    cell: &TESObjectCELL,
    from: NiPoint3,
    to: NiPoint3,
    filter: CFilter,
) -> Option<RaycastHit> {
    let mut pick_data = build_pick_data(from, to, filter);
    let picked = with_cell_world_read(cell, |world| unsafe {
        (*world).pick_object_with(&mut pick_data)
    })?;
    if !picked || !pick_data.ray_output.has_hit() {
        return None;
    }

    raycast_hit_from_output(from, to, &pick_data.ray_output)
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
pub fn raycast_segment_layers(
    cell: &TESObjectCELL,
    from: NiPoint3,
    to: NiPoint3,
    filter: CFilter,
    layers: LayerMask,
) -> Option<RaycastHit> {
    raycast_all_segment(cell, from, to, filter)
        .into_iter()
        .find(|hit| hit.matches_layers(layers))
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
pub fn raycast_delta_layers(
    cell: &TESObjectCELL,
    from: NiPoint3,
    delta: NiPoint3,
    filter: CFilter,
    layers: LayerMask,
) -> Option<RaycastHit> {
    raycast_segment_layers(cell, from, from + delta, filter, layers)
}

#[inline(always)]
pub fn raycast_down(
    cell: &TESObjectCELL,
    from: NiPoint3,
    max_distance: f32,
    filter: CFilter,
) -> Option<RaycastHit> {
    raycast_delta(cell, from, NiPoint3::new(0.0, 0.0, -max_distance), filter)
}

#[inline(always)]
pub fn raycast_down_layers(
    cell: &TESObjectCELL,
    from: NiPoint3,
    max_distance: f32,
    filter: CFilter,
    layers: LayerMask,
) -> Option<RaycastHit> {
    raycast_delta_layers(
        cell,
        from,
        NiPoint3::new(0.0, 0.0, -max_distance),
        filter,
        layers,
    )
}

#[inline(always)]
pub fn ground_snap_point(
    cell: &TESObjectCELL,
    from: NiPoint3,
    max_distance: f32,
    filter: CFilter,
    layers: LayerMask,
) -> Option<NiPoint3> {
    raycast_down_layers(cell, from, max_distance, filter, layers).map(|hit| hit.hit_point)
}

#[inline(always)]
pub fn raycast_all_segment(
    cell: &TESObjectCELL,
    from: NiPoint3,
    to: NiPoint3,
    filter: CFilter,
) -> Vec<RaycastHit> {
    let mut pick_data = build_pick_data(from, to, filter);
    let mut collector = hkpAllRayHitCollector::default();
    pick_data.all_ray_hit_collector = &mut collector;

    let picked = with_cell_world_read(cell, |world| unsafe {
        (*world).pick_object_with(&mut pick_data)
    })
    .unwrap_or(false);
    if !picked {
        return Vec::new();
    }

    let mut hits: Vec<_> = collector
        .hits
        .as_slice()
        .iter()
        .filter_map(|output| raycast_hit_from_output(from, to, output))
        .collect();
    hits.sort_by(compare_hit_fraction);
    hits
}

#[inline(always)]
pub fn raycast_all_segment_mask(
    cell: &TESObjectCELL,
    from: NiPoint3,
    to: NiPoint3,
    filter: u32,
) -> Vec<RaycastHit> {
    raycast_all_segment(cell, from, to, raw_filter(filter))
}

#[inline(always)]
pub fn raycast_all_delta(
    cell: &TESObjectCELL,
    from: NiPoint3,
    delta: NiPoint3,
    filter: CFilter,
) -> Vec<RaycastHit> {
    raycast_all_segment(cell, from, from + delta, filter)
}

#[inline(always)]
pub fn raycast_all_delta_mask(
    cell: &TESObjectCELL,
    from: NiPoint3,
    delta: NiPoint3,
    filter: u32,
) -> Vec<RaycastHit> {
    raycast_all_delta(cell, from, delta, raw_filter(filter))
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
pub fn raycast_from_reference_layers(
    reference: &TESObjectREFR,
    delta: NiPoint3,
    filter: CFilter,
    layers: LayerMask,
) -> Option<RaycastHit> {
    let cell = unsafe { reference.get_parent_cell().as_ref() }?;
    raycast_delta_layers(cell, reference.get_position(), delta, filter, layers)
}

#[inline(always)]
pub fn raycast_all_from_reference(
    reference: &TESObjectREFR,
    delta: NiPoint3,
    filter: CFilter,
) -> Vec<RaycastHit> {
    let Some(cell) = (unsafe { reference.get_parent_cell().as_ref() }) else {
        return Vec::new();
    };
    raycast_all_delta(cell, reference.get_position(), delta, filter)
}

#[inline(always)]
pub fn raycast_all_from_reference_mask(
    reference: &TESObjectREFR,
    delta: NiPoint3,
    filter: u32,
) -> Vec<RaycastHit> {
    raycast_all_from_reference(reference, delta, raw_filter(filter))
}

#[inline(always)]
pub fn closest_hit_in_layers(hits: &[RaycastHit], layers: LayerMask) -> Option<&RaycastHit> {
    hits.iter()
        .filter(|hit| hit.matches_layers(layers))
        .min_by(|a, b| compare_hit_fraction(a, b))
}

#[inline(always)]
pub fn segment_is_clear(
    cell: &TESObjectCELL,
    from: NiPoint3,
    to: NiPoint3,
    filter: CFilter,
    blocking_layers: LayerMask,
) -> bool {
    raycast_segment_layers(cell, from, to, filter, blocking_layers).is_none()
}

#[inline(always)]
pub fn has_line_of_sight(
    cell: &TESObjectCELL,
    from: NiPoint3,
    to: NiPoint3,
    filter: CFilter,
    blocking_layers: LayerMask,
) -> bool {
    segment_is_clear(cell, from, to, filter, blocking_layers)
}

#[inline(always)]
pub fn point_on_segment(from: NiPoint3, to: NiPoint3, fraction: f32) -> NiPoint3 {
    from + (to - from) * fraction
}

#[inline(always)]
pub fn backoff_point(from: NiPoint3, to: NiPoint3, fraction: f32, distance: f32) -> NiPoint3 {
    let delta = to - from;
    let length = delta.length();
    if length <= f32::EPSILON {
        return point_on_segment(from, to, fraction);
    }

    let backed_off_fraction = (fraction - (distance / length)).clamp(0.0, 1.0);
    point_on_segment(from, to, backed_off_fraction)
}

#[inline(always)]
pub fn offset_point_along_normal(point: NiPoint3, normal: NiPoint3, distance: f32) -> NiPoint3 {
    let mut direction = normal;
    direction.unitize();
    point + direction * distance
}

#[inline(always)]
const fn layer_bit(layer: ColLayer) -> u64 {
    1u64 << (layer as u32)
}

fn with_cell_world_read<R>(cell: &TESObjectCELL, f: impl FnOnce(*mut bhkWorld) -> R) -> Option<R> {
    let world = cell.get_bhk_world();
    if world.is_null() {
        return None;
    }

    let _lock = unsafe { BSReadLockGuard::from_ptr(ptr::addr_of_mut!((*world).world_lock)) };
    Some(f(world))
}

#[inline(always)]
fn build_pick_data(from: NiPoint3, to: NiPoint3, filter: CFilter) -> bhkPickData {
    let world_scale = bhkWorld::get_world_scale();
    let mut pick_data = bhkPickData::default();
    pick_data.ray_input.from = hkVector4::from(from * world_scale);
    pick_data.ray_input.to = hkVector4::from(to * world_scale);
    pick_data.ray_input.filter_info = filter;
    pick_data.ray = hkVector4::from((to - from) * world_scale);
    pick_data
}

fn raycast_hit_from_output(
    from: NiPoint3,
    to: NiPoint3,
    output: &hkpWorldRayCastOutput,
) -> Option<RaycastHit> {
    if !output.has_hit() {
        return None;
    }

    let root_collidable = output.root_collidable;
    let collision_layer = if root_collidable.is_null() {
        None
    } else {
        Some(unsafe { (*root_collidable).get_collision_layer() })
    };

    let (reference, object) = if root_collidable.is_null() {
        (ptr::null_mut(), ptr::null_mut())
    } else {
        let collidable = unsafe { &*root_collidable };
        (
            TESHavokUtilities::find_collidable_ref(collidable),
            TESHavokUtilities::find_collidable_object(collidable),
        )
    };

    let hit_fraction = output.base.base.hit_fraction;
    Some(RaycastHit {
        hit_fraction,
        hit_point: point_on_segment(from, to, hit_fraction),
        normal: ni_point3_from_hk(&output.base.base.normal),
        root_collidable,
        collision_layer,
        reference,
        object,
    })
}

#[inline(always)]
fn compare_hit_fraction(a: &RaycastHit, b: &RaycastHit) -> Ordering {
    a.hit_fraction
        .partial_cmp(&b.hit_fraction)
        .unwrap_or(Ordering::Equal)
}

#[inline(always)]
fn ni_point3_from_hk(vector: &hkVector4) -> NiPoint3 {
    NiPoint3::new(vector.quad[0], vector.quad[1], vector.quad[2])
}
