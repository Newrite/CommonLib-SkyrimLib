use alloc::vec::Vec;

use crate::re::{Actor, CFilter, ColLayer, TESObjectREFR};

use super::shared::compare_hit_fraction;
use super::types::{LayerMask, RaycastHit, RaycastHitFilter};

/// Wraps one raw Havok filter bitfield as a typed [`CFilter`].
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

/// Returns the actor's collision filter retargeted for line-of-sight queries.
#[inline(always)]
pub fn actor_line_of_sight_filter(actor: &Actor) -> CFilter {
    actor_filter(actor, ColLayer::LineOfSight)
}

/// Returns the actor's collision filter retargeted for item-pick queries.
#[inline(always)]
pub fn actor_item_pick_filter(actor: &Actor) -> CFilter {
    actor_filter(actor, ColLayer::ItemPick)
}

/// Filters a converted hit list by layer mask, ignored references, and a predicate.
///
/// This is the convenience path when the caller does not need the full
/// [`RaycastHitFilter`] struct but still wants the common “layers + ignored
/// refs + custom predicate” flow.
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

/// Returns the nearest hit that survives layer filtering, ignored references,
/// and an additional predicate.
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

/// Filters a converted hit list with an explicit post-query hit filter.
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

/// Returns the nearest hit that survives an explicit post-query hit filter.
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

/// Returns the nearest hit whose collision layer matches `layers`.
#[inline(always)]
pub fn closest_hit_in_layers(hits: &[RaycastHit], layers: LayerMask) -> Option<&RaycastHit> {
    hits.iter()
        .filter(|hit| hit.matches_layers(layers))
        .min_by(|a, b| compare_hit_fraction(a, b))
}
