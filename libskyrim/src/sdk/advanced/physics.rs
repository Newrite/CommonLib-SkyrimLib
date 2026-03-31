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
use crate::sdk::core::{Resolved, snapshot_contiguous_copied_named};

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

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RaycastSegment {
    pub from: NiPoint3,
    pub to: NiPoint3,
}

impl RaycastSegment {
    #[inline(always)]
    pub fn new(from: NiPoint3, to: NiPoint3) -> Self {
        Self { from, to }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SplitRaycastHit {
    pub segment_index: usize,
    pub segment: RaycastSegment,
    pub hit: RaycastHit,
}

#[derive(Debug, Clone, Copy)]
pub struct RaycastHitFilter<'a> {
    pub layers: Option<LayerMask>,
    pub ignored_references: &'a [*mut TESObjectREFR],
    pub ignored_objects: &'a [*mut NiAVObject],
    pub ignored_collidables: &'a [*const hkpCollidable],
}

impl RaycastHitFilter<'static> {
    #[inline(always)]
    pub const fn new() -> Self {
        Self {
            layers: None,
            ignored_references: &[],
            ignored_objects: &[],
            ignored_collidables: &[],
        }
    }
}

impl<'a> RaycastHitFilter<'a> {
    #[inline(always)]
    pub const fn with_layers(mut self, layers: LayerMask) -> Self {
        self.layers = Some(layers);
        self
    }

    #[inline(always)]
    pub const fn ignore_references(mut self, references: &'a [*mut TESObjectREFR]) -> Self {
        self.ignored_references = references;
        self
    }

    #[inline(always)]
    pub const fn ignore_objects(mut self, objects: &'a [*mut NiAVObject]) -> Self {
        self.ignored_objects = objects;
        self
    }

    #[inline(always)]
    pub const fn ignore_collidables(mut self, collidables: &'a [*const hkpCollidable]) -> Self {
        self.ignored_collidables = collidables;
        self
    }

    #[inline(always)]
    pub fn matches(&self, hit: &RaycastHit) -> bool {
        self.layers.is_none_or(|layers| hit.matches_layers(layers))
            && !self.ignored_references.contains(&hit.reference)
            && !self.ignored_objects.contains(&hit.object)
            && !self.ignored_collidables.contains(&hit.root_collidable)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ClearanceProbePattern {
    Cardinal,
    CardinalAndDiagonal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LineOfSightAggregationPolicy {
    AnyClear,
    AllClear,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LineOfSightPolicy {
    Ignore,
    RequireClear,
    RequireBlocked,
}

#[derive(Debug, Clone, Copy)]
pub struct SpawnPointValidationOptions {
    pub filter: CFilter,
    pub blocking_layers: LayerMask,
    pub ground_probe_height: f32,
    pub ground_probe_depth: f32,
    pub headroom_height: f32,
    pub clearance_radius: f32,
    pub clearance_height: f32,
    pub line_of_sight_origin: Option<NiPoint3>,
    pub line_of_sight_policy: LineOfSightPolicy,
}

impl SpawnPointValidationOptions {
    #[inline(always)]
    pub const fn new(filter: CFilter, blocking_layers: LayerMask) -> Self {
        Self {
            filter,
            blocking_layers,
            ground_probe_height: 128.0,
            ground_probe_depth: 512.0,
            headroom_height: 128.0,
            clearance_radius: 24.0,
            clearance_height: 32.0,
            line_of_sight_origin: None,
            line_of_sight_policy: LineOfSightPolicy::Ignore,
        }
    }

    #[inline(always)]
    pub const fn respawn_default(filter: CFilter, blocking_layers: LayerMask) -> Self {
        Self {
            filter,
            blocking_layers,
            ground_probe_height: 160.0,
            ground_probe_depth: 768.0,
            headroom_height: 128.0,
            clearance_radius: 32.0,
            clearance_height: 40.0,
            line_of_sight_origin: None,
            line_of_sight_policy: LineOfSightPolicy::Ignore,
        }
    }

    #[inline(always)]
    pub const fn with_ground_probe(mut self, height: f32, depth: f32) -> Self {
        self.ground_probe_height = height;
        self.ground_probe_depth = depth;
        self
    }

    #[inline(always)]
    pub const fn with_headroom(mut self, headroom_height: f32) -> Self {
        self.headroom_height = headroom_height;
        self
    }

    #[inline(always)]
    pub const fn with_clearance(mut self, clearance_radius: f32, clearance_height: f32) -> Self {
        self.clearance_radius = clearance_radius;
        self.clearance_height = clearance_height;
        self
    }

    #[inline(always)]
    pub const fn with_line_of_sight(
        mut self,
        origin: Option<NiPoint3>,
        policy: LineOfSightPolicy,
    ) -> Self {
        self.line_of_sight_origin = origin;
        self.line_of_sight_policy = policy;
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SpawnPointValidation {
    pub candidate: NiPoint3,
    pub snapped_point: Option<NiPoint3>,
    pub ground_hit: Option<RaycastHit>,
    pub headroom_clear: bool,
    pub clearance_clear: bool,
    pub line_of_sight_clear: Option<bool>,
    pub line_of_sight_ok: bool,
    pub is_valid: bool,
}

const CARDINAL_DIRECTIONS: [(f32, f32); 4] = [(1.0, 0.0), (-1.0, 0.0), (0.0, 1.0), (0.0, -1.0)];
const DIAGONAL_DIRECTIONS: [(f32, f32); 4] = [(1.0, 1.0), (1.0, -1.0), (-1.0, 1.0), (-1.0, -1.0)];

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

    let mut hits: Vec<_> = snapshot_contiguous_copied_named(
        &collector.hits,
        "sdk::advanced::physics::raycast_all_segment()",
        Default::default(),
    )
    .into_iter()
    .filter_map(|output| raycast_hit_from_output(from, to, &output))
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

pub fn split_raycast(
    cell: &TESObjectCELL,
    segments: &[RaycastSegment],
    filter: CFilter,
    blocking_layers: LayerMask,
) -> Option<SplitRaycastHit> {
    for (segment_index, &segment) in segments.iter().enumerate() {
        if let Some(hit) =
            raycast_segment_layers(cell, segment.from, segment.to, filter, blocking_layers)
        {
            return Some(SplitRaycastHit {
                segment_index,
                segment,
                hit,
            });
        }
    }

    None
}

#[inline(always)]
pub fn split_raycast_mask(
    cell: &TESObjectCELL,
    segments: &[RaycastSegment],
    filter: u32,
    blocking_layers: LayerMask,
) -> Option<SplitRaycastHit> {
    split_raycast(cell, segments, raw_filter(filter), blocking_layers)
}

pub fn split_raycast_with_filter(
    cell: &TESObjectCELL,
    segments: &[RaycastSegment],
    filter: CFilter,
    hit_filter: &RaycastHitFilter<'_>,
) -> Option<SplitRaycastHit> {
    for (segment_index, &segment) in segments.iter().enumerate() {
        let hits = raycast_all_segment(cell, segment.from, segment.to, filter);
        if let Some(hit) = best_hit_with(&hits, hit_filter, |_| true) {
            return Some(SplitRaycastHit {
                segment_index,
                segment,
                hit,
            });
        }
    }

    None
}

pub fn clearance_probe_segments(
    origin: NiPoint3,
    radius: f32,
    pattern: ClearanceProbePattern,
) -> Vec<RaycastSegment> {
    if !radius.is_finite() || radius <= f32::EPSILON {
        return Vec::new();
    }

    let mut segments = Vec::with_capacity(match pattern {
        ClearanceProbePattern::Cardinal => CARDINAL_DIRECTIONS.len(),
        ClearanceProbePattern::CardinalAndDiagonal => {
            CARDINAL_DIRECTIONS.len() + DIAGONAL_DIRECTIONS.len()
        }
    });

    for &(x, y) in &CARDINAL_DIRECTIONS {
        segments.push(RaycastSegment::new(
            origin,
            origin + NiPoint3::new(radius * x, radius * y, 0.0),
        ));
    }

    if pattern == ClearanceProbePattern::CardinalAndDiagonal {
        const INV_SQRT_2: f32 = 0.70710677;
        for &(x, y) in &DIAGONAL_DIRECTIONS {
            segments.push(RaycastSegment::new(
                origin,
                origin + NiPoint3::new(radius * x * INV_SQRT_2, radius * y * INV_SQRT_2, 0.0),
            ));
        }
    }

    segments
}

pub fn clearance_probe_segments_at_heights(
    origin: NiPoint3,
    radius: f32,
    heights: &[f32],
    pattern: ClearanceProbePattern,
) -> Vec<RaycastSegment> {
    let mut segments = Vec::new();
    for &height in heights {
        if !height.is_finite() {
            continue;
        }
        segments.extend(clearance_probe_segments(
            origin + NiPoint3::new(0.0, 0.0, height),
            radius,
            pattern,
        ));
    }
    segments
}

pub fn filter_hits(
    hits: &[RaycastHit],
    layers: LayerMask,
    ignored_references: &[*mut TESObjectREFR],
    predicate: impl FnMut(&RaycastHit) -> bool,
) -> Vec<RaycastHit> {
    let hit_filter = if layers == LayerMask::empty() {
        RaycastHitFilter::new().ignore_references(ignored_references)
    } else {
        RaycastHitFilter::new()
            .with_layers(layers)
            .ignore_references(ignored_references)
    };
    filter_hits_with(hits, &hit_filter, predicate)
}

pub fn best_hit(
    hits: &[RaycastHit],
    layers: LayerMask,
    ignored_references: &[*mut TESObjectREFR],
    predicate: impl FnMut(&RaycastHit) -> bool,
) -> Option<RaycastHit> {
    let hit_filter = if layers == LayerMask::empty() {
        RaycastHitFilter::new().ignore_references(ignored_references)
    } else {
        RaycastHitFilter::new()
            .with_layers(layers)
            .ignore_references(ignored_references)
    };
    best_hit_with(hits, &hit_filter, predicate)
}

pub fn filter_hits_with(
    hits: &[RaycastHit],
    hit_filter: &RaycastHitFilter<'_>,
    mut predicate: impl FnMut(&RaycastHit) -> bool,
) -> Vec<RaycastHit> {
    hits.iter()
        .copied()
        .filter(|hit| hit_filter.matches(hit) && predicate(hit))
        .collect()
}

pub fn best_hit_with(
    hits: &[RaycastHit],
    hit_filter: &RaycastHitFilter<'_>,
    mut predicate: impl FnMut(&RaycastHit) -> bool,
) -> Option<RaycastHit> {
    hits.iter()
        .copied()
        .filter(|hit| hit_filter.matches(hit) && predicate(hit))
        .min_by(compare_hit_fraction)
}

pub fn clearance_is_clear_with_filter(
    cell: &TESObjectCELL,
    origin: NiPoint3,
    radius: f32,
    heights: &[f32],
    pattern: ClearanceProbePattern,
    filter: CFilter,
    hit_filter: &RaycastHitFilter<'_>,
) -> bool {
    if radius <= f32::EPSILON {
        return true;
    }

    let segments = clearance_probe_segments_at_heights(origin, radius, heights, pattern);
    if segments.is_empty() {
        return true;
    }

    split_raycast_with_filter(cell, &segments, filter, hit_filter).is_none()
}

pub fn line_of_sight_from_points_with_filter(
    cell: &TESObjectCELL,
    origins: &[NiPoint3],
    target: NiPoint3,
    filter: CFilter,
    hit_filter: &RaycastHitFilter<'_>,
    policy: LineOfSightAggregationPolicy,
) -> Option<bool> {
    let mut saw_any = false;
    match policy {
        LineOfSightAggregationPolicy::AnyClear => {
            for &origin in origins {
                if !origin.x.is_finite() || !origin.y.is_finite() || !origin.z.is_finite() {
                    continue;
                }
                saw_any = true;
                if has_line_of_sight_with_filter(cell, origin, target, filter, hit_filter) {
                    return Some(true);
                }
            }
            saw_any.then_some(false)
        }
        LineOfSightAggregationPolicy::AllClear => {
            for &origin in origins {
                if !origin.x.is_finite() || !origin.y.is_finite() || !origin.z.is_finite() {
                    continue;
                }
                saw_any = true;
                if !has_line_of_sight_with_filter(cell, origin, target, filter, hit_filter) {
                    return Some(false);
                }
            }
            saw_any.then_some(true)
        }
    }
}

#[inline(always)]
pub fn closest_hit_in_layers(hits: &[RaycastHit], layers: LayerMask) -> Option<&RaycastHit> {
    hits.iter()
        .filter(|hit| hit.matches_layers(layers))
        .min_by(|a, b| compare_hit_fraction(a, b))
}

pub fn validate_spawn_point(
    cell: &TESObjectCELL,
    candidate: NiPoint3,
    options: SpawnPointValidationOptions,
) -> SpawnPointValidation {
    validate_spawn_point_with_filter(cell, candidate, options, &RaycastHitFilter::new())
}

pub fn validate_spawn_point_with_filter(
    cell: &TESObjectCELL,
    candidate: NiPoint3,
    options: SpawnPointValidationOptions,
    hit_filter: &RaycastHitFilter<'_>,
) -> SpawnPointValidation {
    if !candidate.x.is_finite() || !candidate.y.is_finite() || !candidate.z.is_finite() {
        crate::defensive_sdk_warn!(
            "sdk::advanced::physics::validate_spawn_point_with_filter() ignored non-finite candidate point"
        );
        return SpawnPointValidation {
            candidate,
            snapped_point: None,
            ground_hit: None,
            headroom_clear: false,
            clearance_clear: false,
            line_of_sight_clear: None,
            line_of_sight_ok: false,
            is_valid: false,
        };
    }

    if !options.ground_probe_height.is_finite()
        || options.ground_probe_height < 0.0
        || !options.ground_probe_depth.is_finite()
        || options.ground_probe_depth <= 0.0
        || !options.headroom_height.is_finite()
        || options.headroom_height < 0.0
        || !options.clearance_radius.is_finite()
        || options.clearance_radius < 0.0
        || !options.clearance_height.is_finite()
        || options.clearance_height < 0.0
    {
        crate::defensive_sdk_warn!(
            "sdk::advanced::physics::validate_spawn_point_with_filter() ignored invalid validation options"
        );
        return SpawnPointValidation {
            candidate,
            snapped_point: None,
            ground_hit: None,
            headroom_clear: false,
            clearance_clear: false,
            line_of_sight_clear: None,
            line_of_sight_ok: false,
            is_valid: false,
        };
    }

    let ground_probe_start = candidate + NiPoint3::new(0.0, 0.0, options.ground_probe_height);
    let ground_hits = raycast_all_delta(
        cell,
        ground_probe_start,
        NiPoint3::new(
            0.0,
            0.0,
            -(options.ground_probe_height + options.ground_probe_depth),
        ),
        options.filter,
    );
    let layer_filter = RaycastHitFilter {
        layers: Some(options.blocking_layers),
        ignored_references: hit_filter.ignored_references,
        ignored_objects: hit_filter.ignored_objects,
        ignored_collidables: hit_filter.ignored_collidables,
    };
    let ground_hit = best_hit_with(&ground_hits, &layer_filter, |_| true);
    let Some(ground_hit) = ground_hit else {
        return SpawnPointValidation {
            candidate,
            snapped_point: None,
            ground_hit: None,
            headroom_clear: false,
            clearance_clear: false,
            line_of_sight_clear: None,
            line_of_sight_ok: false,
            is_valid: false,
        };
    };

    let snapped_point = ground_hit.hit_point;
    let headroom_clear = if options.headroom_height <= f32::EPSILON {
        true
    } else {
        segment_is_clear_with_filter(
            cell,
            snapped_point,
            snapped_point + NiPoint3::new(0.0, 0.0, options.headroom_height),
            options.filter,
            &layer_filter,
        )
    };

    let clearance_clear = if options.clearance_radius <= f32::EPSILON {
        true
    } else {
        let lower_height = options.clearance_height.max(0.0);
        let upper_height =
            (options.clearance_height + (options.headroom_height * 0.5)).max(lower_height);
        let heights = if upper_height > lower_height + f32::EPSILON {
            [lower_height, upper_height]
        } else {
            [lower_height, lower_height]
        };
        clearance_is_clear_with_filter(
            cell,
            snapped_point,
            options.clearance_radius,
            if heights[0] == heights[1] {
                &heights[..1]
            } else {
                &heights
            },
            ClearanceProbePattern::CardinalAndDiagonal,
            options.filter,
            &layer_filter,
        )
    };

    let line_of_sight_clear = options.line_of_sight_origin.map(|origin| {
        has_line_of_sight_with_filter(cell, origin, snapped_point, options.filter, &layer_filter)
    });
    let line_of_sight_ok = match (options.line_of_sight_policy, line_of_sight_clear) {
        (LineOfSightPolicy::Ignore, _) => true,
        (LineOfSightPolicy::RequireClear, Some(clear)) => clear,
        (LineOfSightPolicy::RequireBlocked, Some(clear)) => !clear,
        (_, None) => false,
    };

    // TODO: Fold honest navmesh sanity into this bundled validator once we have
    // a point-based source-backed SDK surface instead of only ref-relative
    // `TESObjectREFR::find_nearest_vertex(...)` / `move_to_nearest_navmesh(...)`.
    let is_valid = headroom_clear && clearance_clear && line_of_sight_ok;
    SpawnPointValidation {
        candidate,
        snapped_point: Some(snapped_point),
        ground_hit: Some(ground_hit),
        headroom_clear,
        clearance_clear,
        line_of_sight_clear,
        line_of_sight_ok,
        is_valid,
    }
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

pub fn segment_is_clear_with_filter(
    cell: &TESObjectCELL,
    from: NiPoint3,
    to: NiPoint3,
    filter: CFilter,
    hit_filter: &RaycastHitFilter<'_>,
) -> bool {
    let hits = raycast_all_segment(cell, from, to, filter);
    best_hit_with(&hits, hit_filter, |_| true).is_none()
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
pub fn has_line_of_sight_with_filter(
    cell: &TESObjectCELL,
    from: NiPoint3,
    to: NiPoint3,
    filter: CFilter,
    hit_filter: &RaycastHitFilter<'_>,
) -> bool {
    segment_is_clear_with_filter(cell, from, to, filter, hit_filter)
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
        unsafe { (*root_collidable).try_get_collision_layer() }
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
