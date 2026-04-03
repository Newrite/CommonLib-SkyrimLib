use alloc::vec::Vec;

use crate::re::{CFilter, NiPoint3, TESObjectCELL};

use super::filters::best_hit_with;
use super::raycast::{
    raycast_all_delta, raycast_all_segment, raycast_segment_layers, split_raycast_with_filter,
};
use super::types::{
    ClearanceProbePattern, LayerMask, LineOfSightAggregationPolicy, LineOfSightPolicy,
    RaycastHitFilter, RaycastSegment, SpawnPointValidation, SpawnPointValidationOptions,
};

const CARDINAL_DIRECTIONS: [(f32, f32); 4] = [(1.0, 0.0), (-1.0, 0.0), (0.0, 1.0), (0.0, -1.0)];
const DIAGONAL_DIRECTIONS: [(f32, f32); 4] = [(1.0, 1.0), (1.0, -1.0), (-1.0, 1.0), (-1.0, -1.0)];

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
