use crate::re::{CFilter, ColLayer, NiAVObject, NiPoint3, TESObjectREFR, hkpCollidable};
use crate::sdk::core::Resolved;

use super::geometry::{backoff_point, offset_point_along_normal};

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

#[inline(always)]
const fn layer_bit(layer: ColLayer) -> u64 {
    1u64 << (layer as u32)
}
