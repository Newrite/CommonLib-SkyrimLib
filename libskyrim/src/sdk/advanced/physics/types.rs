use crate::re::{CFilter, ColLayer, NiAVObject, NiPoint3, TESObjectREFR, hkpCollidable};
use crate::sdk::core::Resolved;

use super::geometry::{backoff_point, offset_point_along_normal};

/// Post-query collision-layer mask used to decide which Havok hits should
/// count as blockers.
///
/// This is intentionally separate from the low-level [`CFilter`] used to ask
/// Havok for hits in the first place. The common workflow is:
///
/// 1. perform a raycast or probe with a broad [`CFilter`]
/// 2. keep only the returned hits whose collision layer matches this mask
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LayerMask {
    bits: u64,
}

impl LayerMask {
    /// Returns an empty mask that matches no layers.
    #[inline(always)]
    pub const fn empty() -> Self {
        Self { bits: 0 }
    }

    /// Returns a mask that matches every layer bit.
    #[inline(always)]
    pub const fn all() -> Self {
        Self { bits: u64::MAX }
    }

    /// Creates a mask from raw layer bits.
    #[inline(always)]
    pub const fn from_bits(bits: u64) -> Self {
        Self { bits }
    }

    /// Returns the raw layer bits stored by this mask.
    #[inline(always)]
    pub const fn bits(self) -> u64 {
        self.bits
    }

    /// Creates a mask that matches exactly one Havok collision layer.
    #[inline(always)]
    pub const fn from_layer(layer: ColLayer) -> Self {
        Self {
            bits: layer_bit(layer),
        }
    }

    /// Builds a mask that matches any layer contained in the input slice.
    #[inline(always)]
    pub fn from_slice(layers: &[ColLayer]) -> Self {
        let mut mask = Self::empty();
        for &layer in layers {
            mask.insert(layer);
        }
        mask
    }

    /// Returns whether the mask contains the given collision layer.
    #[inline(always)]
    pub const fn contains_layer(self, layer: ColLayer) -> bool {
        (self.bits & layer_bit(layer)) != 0
    }

    /// Returns whether a converted raycast hit matches this layer mask.
    #[inline(always)]
    pub fn contains_hit(self, hit: &RaycastHit) -> bool {
        hit.matches_layers(self)
    }

    /// Adds one collision layer to the mask.
    #[inline(always)]
    pub fn insert(&mut self, layer: ColLayer) {
        self.bits |= layer_bit(layer);
    }

    /// Removes one collision layer from the mask.
    #[inline(always)]
    pub fn remove(&mut self, layer: ColLayer) {
        self.bits &= !layer_bit(layer);
    }

    /// Returns whether the two masks share at least one layer bit.
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
    /// Normalized hit fraction along the original cast segment.
    pub hit_fraction: f32,
    /// World-space point where the hit occurred.
    pub hit_point: NiPoint3,
    /// Surface normal reported by Havok at the hit point.
    pub normal: NiPoint3,
    /// Root collidable that produced the hit, if Havok reported one.
    pub root_collidable: *const hkpCollidable,
    /// Converted collision layer for the hit, when one could be decoded.
    pub collision_layer: Option<ColLayer>,
    /// Raw reference pointer associated with the hit.
    pub reference: *mut TESObjectREFR,
    /// Raw scene object pointer associated with the hit.
    pub object: *mut NiAVObject,
}

impl RaycastHit {
    /// Attempts to upgrade the raw reference pointer into a resolved handle.
    #[inline(always)]
    pub fn resolved_reference(self) -> Option<Resolved<TESObjectREFR>> {
        Resolved::try_from_ptr(self.reference)
    }

    /// Returns whether this hit belongs to one of the allowed layers.
    #[inline(always)]
    pub fn matches_layers(self, layers: LayerMask) -> bool {
        self.collision_layer
            .is_some_and(|layer| layers.contains_layer(layer))
    }

    /// Moves the hit point outward along the reported surface normal.
    #[inline(always)]
    pub fn offset_along_normal(self, distance: f32) -> NiPoint3 {
        offset_point_along_normal(self.hit_point, self.normal, distance)
    }

    /// Backs away from the hit along the original cast segment.
    #[inline(always)]
    pub fn backoff_along_segment(self, from: NiPoint3, to: NiPoint3, distance: f32) -> NiPoint3 {
        backoff_point(from, to, self.hit_fraction, distance)
    }
}

/// Simple world-space ray segment used by split and batched probes.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RaycastSegment {
    /// Start point of the segment.
    pub from: NiPoint3,
    /// End point of the segment.
    pub to: NiPoint3,
}

impl RaycastSegment {
    /// Creates a new segment from two world-space endpoints.
    #[inline(always)]
    pub fn new(from: NiPoint3, to: NiPoint3) -> Self {
        Self { from, to }
    }
}

/// Raycast hit annotated with the segment that produced it.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SplitRaycastHit {
    /// Index of the segment inside the original segmented query.
    pub segment_index: usize,
    /// Segment that produced the hit.
    pub segment: RaycastSegment,
    /// Converted hit reported for the segment.
    pub hit: RaycastHit,
}

/// Post-processing filter for already-converted raycast hits.
///
/// This is useful when the engine query is intentionally broad but the caller
/// wants a tighter gameplay-specific blocker policy without re-running the
/// probe.
#[derive(Debug, Clone, Copy)]
pub struct RaycastHitFilter<'a> {
    /// Optional layer mask that the hit must match.
    pub layers: Option<LayerMask>,
    /// References that should be ignored even if they were hit.
    pub ignored_references: &'a [*mut TESObjectREFR],
    /// Scene objects that should be ignored even if they were hit.
    pub ignored_objects: &'a [*mut NiAVObject],
    /// Havok collidables that should be ignored even if they were hit.
    pub ignored_collidables: &'a [*const hkpCollidable],
}

impl RaycastHitFilter<'static> {
    /// Returns a permissive filter that accepts every hit.
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
    /// Restricts the filter to hits whose collision layer matches `layers`.
    #[inline(always)]
    pub const fn with_layers(mut self, layers: LayerMask) -> Self {
        self.layers = Some(layers);
        self
    }

    /// Ignores hits whose runtime reference pointer appears in `references`.
    #[inline(always)]
    pub const fn ignore_references(mut self, references: &'a [*mut TESObjectREFR]) -> Self {
        self.ignored_references = references;
        self
    }

    /// Ignores hits whose scene object pointer appears in `objects`.
    #[inline(always)]
    pub const fn ignore_objects(mut self, objects: &'a [*mut NiAVObject]) -> Self {
        self.ignored_objects = objects;
        self
    }

    /// Ignores hits whose root collidable appears in `collidables`.
    #[inline(always)]
    pub const fn ignore_collidables(mut self, collidables: &'a [*const hkpCollidable]) -> Self {
        self.ignored_collidables = collidables;
        self
    }

    /// Returns whether a converted hit survives this post-query filter.
    #[inline(always)]
    pub fn matches(&self, hit: &RaycastHit) -> bool {
        self.layers.is_none_or(|layers| hit.matches_layers(layers))
            && !self.ignored_references.contains(&hit.reference)
            && !self.ignored_objects.contains(&hit.object)
            && !self.ignored_collidables.contains(&hit.root_collidable)
    }
}

/// Pattern used for radial clearance probes around a candidate point.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ClearanceProbePattern {
    /// Probe only the cardinal directions around the candidate.
    Cardinal,
    /// Probe both cardinal and diagonal directions around the candidate.
    CardinalAndDiagonal,
}

/// Aggregation rule for multi-ray line-of-sight checks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LineOfSightAggregationPolicy {
    /// Treat the group as clear when at least one probe is clear.
    AnyClear,
    /// Treat the group as clear only when every probe is clear.
    AllClear,
}

/// Policy applied when validating a candidate against line of sight.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LineOfSightPolicy {
    /// Skip line-of-sight evaluation entirely.
    Ignore,
    /// Accept the candidate only if the path to the origin is clear.
    RequireClear,
    /// Accept the candidate only if the path to the origin is blocked.
    RequireBlocked,
}

/// Options for `ground + headroom + clearance + optional LOS` validation.
///
/// This is the main policy object for spawn-point and teleport-target checks.
/// The defaults are tuned for common gameplay placement use cases and can then
/// be narrowed with the builder-style helpers below.
#[derive(Debug, Clone, Copy)]
pub struct SpawnPointValidationOptions {
    /// Broad Havok query filter used for the underlying probes.
    pub filter: CFilter,
    /// Post-query mask describing which layers should count as blockers.
    pub blocking_layers: LayerMask,
    /// Height above the candidate from which the ground probe starts.
    pub ground_probe_height: f32,
    /// Maximum depth below the candidate that the ground probe may travel.
    pub ground_probe_depth: f32,
    /// Vertical space that must stay clear above the snapped point.
    pub headroom_height: f32,
    /// Radius used by radial clearance probes around the snapped point.
    pub clearance_radius: f32,
    /// Height at which radial clearance probes are evaluated.
    pub clearance_height: f32,
    /// Optional LOS origin used to validate visibility to the candidate.
    pub line_of_sight_origin: Option<NiPoint3>,
    /// LOS rule applied when `line_of_sight_origin` is present.
    pub line_of_sight_policy: LineOfSightPolicy,
}

impl SpawnPointValidationOptions {
    /// Returns balanced defaults for general placement and teleport checks.
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

    /// Returns roomier defaults tuned for respawn-style placement.
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

    /// Overrides the ground-snapping probe dimensions.
    #[inline(always)]
    pub const fn with_ground_probe(mut self, height: f32, depth: f32) -> Self {
        self.ground_probe_height = height;
        self.ground_probe_depth = depth;
        self
    }

    /// Overrides the required vertical headroom above the snapped point.
    #[inline(always)]
    pub const fn with_headroom(mut self, headroom_height: f32) -> Self {
        self.headroom_height = headroom_height;
        self
    }

    /// Overrides the radial clearance probe dimensions.
    #[inline(always)]
    pub const fn with_clearance(mut self, clearance_radius: f32, clearance_height: f32) -> Self {
        self.clearance_radius = clearance_radius;
        self.clearance_height = clearance_height;
        self
    }

    /// Enables or reconfigures line-of-sight validation against an origin.
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

/// Diagnostic result returned by spawn-point validation helpers.
///
/// Callers that only need a yes/no answer can read [`Self::is_valid`], while
/// more advanced placement flows can inspect the intermediate probe results to
/// understand why a candidate failed and to derive fallback positions.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SpawnPointValidation {
    /// Original candidate supplied by the caller.
    pub candidate: NiPoint3,
    /// Ground-snapped position chosen for follow-up probes, when available.
    pub snapped_point: Option<NiPoint3>,
    /// Ground hit used to derive `snapped_point`, when one was found.
    pub ground_hit: Option<RaycastHit>,
    /// Whether the vertical headroom probe succeeded.
    pub headroom_clear: bool,
    /// Whether the radial clearance probe succeeded.
    pub clearance_clear: bool,
    /// Raw LOS result when LOS validation was requested.
    pub line_of_sight_clear: Option<bool>,
    /// Whether the LOS policy accepted the candidate.
    pub line_of_sight_ok: bool,
    /// Final verdict combining ground, headroom, clearance, and LOS rules.
    pub is_valid: bool,
}

#[inline(always)]
const fn layer_bit(layer: ColLayer) -> u64 {
    1u64 << (layer as u32)
}
