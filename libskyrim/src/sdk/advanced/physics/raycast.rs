use alloc::vec::Vec;

use crate::re::{CFilter, NiPoint3, TESObjectCELL, TESObjectREFR, hkpAllRayHitCollector};
use crate::sdk::core::snapshot_contiguous_copied_named;

use super::filters::{best_hit_with, raw_filter};
use super::shared::{
    build_pick_data, compare_hit_fraction, raycast_hit_from_output, with_cell_world_read,
};
use super::types::{LayerMask, RaycastHit, RaycastHitFilter, RaycastSegment, SplitRaycastHit};

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
