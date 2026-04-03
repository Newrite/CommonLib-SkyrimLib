use crate::re::NiPoint3;

use super::types::ScoredSpatialCandidateEvaluation;

#[inline(always)]
pub(super) fn is_valid_radius(radius: f32, _caller: &'static str) -> bool {
    if !radius.is_finite() || radius <= 0.0 {
        crate::defensive_sdk_warn!(
            "{} ignored non-positive or non-finite radius={}",
            _caller,
            radius
        );
        false
    } else {
        true
    }
}

#[inline(always)]
pub(super) fn nearest_distance_to_points(origin: NiPoint3, points: &[NiPoint3]) -> Option<f32> {
    points
        .iter()
        .map(|point| origin.get_distance(*point))
        .min_by(|a, b| a.partial_cmp(b).unwrap_or(core::cmp::Ordering::Equal))
}

#[inline(always)]
pub(super) fn satisfies_minimum_distance(minimum: f32, actual: Option<f32>) -> bool {
    if minimum <= f32::EPSILON {
        true
    } else {
        actual.is_none_or(|distance| distance >= minimum)
    }
}

#[inline(always)]
pub(super) fn normalized_distance_score(distance: Option<f32>, scene_radius: f32) -> f32 {
    if scene_radius <= f32::EPSILON {
        return 0.0;
    }

    distance
        .map(|distance| (distance / scene_radius).clamp(0.0, 1.0))
        .unwrap_or(0.0)
}

#[inline(always)]
pub(super) fn normalized_scene_distance_score(distance: Option<f32>, scene_radius: f32) -> f32 {
    if scene_radius <= f32::EPSILON {
        return 0.0;
    }

    distance
        .map(|distance| (distance / scene_radius).clamp(0.0, 1.0))
        .unwrap_or(1.0)
}

#[inline(always)]
pub(super) fn satisfies_navmesh_distance(maximum: f32, actual: Option<f32>) -> bool {
    if maximum <= f32::EPSILON {
        true
    } else {
        actual.is_some_and(|distance| distance <= maximum)
    }
}

pub(super) fn best_index_by_score(
    candidates: &[ScoredSpatialCandidateEvaluation],
    predicate: impl Fn(&ScoredSpatialCandidateEvaluation) -> bool,
) -> Option<usize> {
    candidates
        .iter()
        .enumerate()
        .filter(|(_, candidate)| predicate(candidate))
        .max_by(|(_, a), (_, b)| {
            a.score
                .total
                .partial_cmp(&b.score.total)
                .unwrap_or(core::cmp::Ordering::Equal)
        })
        .map(|(index, _)| index)
}
