use alloc::vec::Vec;

use crate::re::{NiPoint3, TESObjectCELL, TESObjectREFR};
use crate::sdk::advanced::physics::{RaycastHitFilter, validate_spawn_point_with_filter};
use crate::sdk::gameplay::navmesh::{self};
use crate::sdk::gameplay::world::{self, WorldSceneSnapshot};

use super::shared::{
    best_index_by_score, is_valid_radius, nearest_distance_to_points, normalized_distance_score,
    normalized_scene_distance_score, satisfies_minimum_distance, satisfies_navmesh_distance,
};
use super::types::{
    ScoredSpatialCandidateEvaluation, SpatialCandidateEvaluation,
    SpatialCandidateEvaluationOptions, SpatialCandidateScore, SpatialCandidateScoreWeights,
    SpatialCandidateSetEvaluation,
};

pub fn evaluate_spawn_candidate(
    cell: &TESObjectCELL,
    candidate: NiPoint3,
    options: SpatialCandidateEvaluationOptions,
) -> SpatialCandidateEvaluation {
    evaluate_spawn_candidate_with_filter(cell, candidate, options, &RaycastHitFilter::new())
}

pub fn evaluate_spawn_candidate_with_filter(
    cell: &TESObjectCELL,
    candidate: NiPoint3,
    options: SpatialCandidateEvaluationOptions,
    hit_filter: &RaycastHitFilter<'_>,
) -> SpatialCandidateEvaluation {
    if !is_valid_radius(
        options.scene_radius,
        "sdk::gameplay::spatial::evaluate_spawn_candidate_with_filter()",
    ) {
        return SpatialCandidateEvaluation {
            candidate,
            scene_center: None,
            scene: WorldSceneSnapshot::default(),
            physics: validate_spawn_point_with_filter(cell, candidate, options.physics, hit_filter),
            nearest_actor_distance: None,
            nearest_hostile_distance: None,
            navmesh: navmesh::query_point_in_cell(cell, candidate, 0.0),
            reachability: None,
            satisfies_actor_distance: false,
            satisfies_hostile_distance: false,
            satisfies_navmesh_distance: false,
            satisfies_reachability: false,
            is_valid: false,
        };
    }

    let physics = validate_spawn_point_with_filter(cell, candidate, options.physics, hit_filter);
    let navmesh_query = navmesh::query_point_in_cell(cell, candidate, 0.0);
    let scene_center = physics.snapped_point;
    let Some(scene_center) = scene_center else {
        return SpatialCandidateEvaluation {
            candidate,
            scene_center: None,
            scene: WorldSceneSnapshot::default(),
            physics,
            nearest_actor_distance: None,
            nearest_hostile_distance: None,
            navmesh: navmesh_query,
            reachability: None,
            satisfies_actor_distance: false,
            satisfies_hostile_distance: false,
            satisfies_navmesh_distance: false,
            satisfies_reachability: false,
            is_valid: false,
        };
    };

    let scene = world::snapshot_scene_in_cell_range(cell, scene_center, options.scene_radius);
    let nearest_actor_distance = nearest_distance_to_points(scene_center, &scene.actor_positions);
    let nearest_hostile_distance =
        nearest_distance_to_points(scene_center, &scene.hostile_actor_positions);
    let satisfies_actor_distance =
        satisfies_minimum_distance(options.minimum_actor_distance, nearest_actor_distance);
    let satisfies_hostile_distance =
        satisfies_minimum_distance(options.minimum_hostile_distance, nearest_hostile_distance);
    let navmesh = navmesh::query_point_in_cell(cell, scene_center, 0.0);
    let satisfies_navmesh_distance = satisfies_navmesh_distance(
        options.maximum_navmesh_distance,
        navmesh.nearest_support_distance(),
    );
    let reachability = options.reachability_origin.map(|origin| {
        navmesh::evaluate_reachability_in_cell(cell, origin, scene_center, options.reachability)
    });
    let satisfies_reachability = reachability
        .as_ref()
        .is_none_or(|reachability| reachability.likely_reachable);
    let is_valid = physics.is_valid
        && satisfies_actor_distance
        && satisfies_hostile_distance
        && satisfies_navmesh_distance
        && satisfies_reachability;

    SpatialCandidateEvaluation {
        candidate,
        scene_center: Some(scene_center),
        scene,
        physics,
        nearest_actor_distance,
        nearest_hostile_distance,
        navmesh,
        reachability,
        satisfies_actor_distance,
        satisfies_hostile_distance,
        satisfies_navmesh_distance,
        satisfies_reachability,
        is_valid,
    }
}

pub fn evaluate_spawn_candidate_from_reference(
    reference: &TESObjectREFR,
    candidate: NiPoint3,
    options: SpatialCandidateEvaluationOptions,
) -> Option<SpatialCandidateEvaluation> {
    evaluate_spawn_candidate_from_reference_with_filter(
        reference,
        candidate,
        options,
        &RaycastHitFilter::new(),
    )
}

pub fn evaluate_spawn_candidate_from_reference_with_filter(
    reference: &TESObjectREFR,
    candidate: NiPoint3,
    mut options: SpatialCandidateEvaluationOptions,
    hit_filter: &RaycastHitFilter<'_>,
) -> Option<SpatialCandidateEvaluation> {
    let cell = unsafe { reference.get_parent_cell().as_ref() }?;
    if options.reachability_origin.is_none() {
        options.reachability_origin = Some(reference.get_position());
    }
    Some(evaluate_spawn_candidate_with_filter(
        cell, candidate, options, hit_filter,
    ))
}

pub fn score_spawn_candidate(
    evaluation: &SpatialCandidateEvaluation,
    options: SpatialCandidateEvaluationOptions,
    weights: SpatialCandidateScoreWeights,
) -> SpatialCandidateScore {
    let valid_component = if evaluation.is_valid {
        weights.valid_bonus
    } else {
        0.0
    };
    let actor_distance_component =
        normalized_scene_distance_score(evaluation.nearest_actor_distance, options.scene_radius)
            * weights.actor_distance;
    let hostile_distance_component =
        normalized_scene_distance_score(evaluation.nearest_hostile_distance, options.scene_radius)
            * weights.hostile_distance;
    let navmesh_distance_component = if options.maximum_navmesh_distance <= f32::EPSILON {
        0.0
    } else {
        normalized_distance_score(
            evaluation
                .navmesh
                .nearest_support_distance()
                .map(|distance| (options.maximum_navmesh_distance - distance).max(0.0)),
            options.maximum_navmesh_distance,
        ) * weights.navmesh_distance
    };
    let actor_density_component =
        -(evaluation.scene.actor_positions.len() as f32) * weights.actor_density_penalty;
    let hostile_density_component =
        -(evaluation.scene.hostile_actor_positions.len() as f32) * weights.hostile_density_penalty;
    let navmesh_support_component = if evaluation.navmesh.has_support() {
        weights.navmesh_support_bonus
    } else {
        0.0
    };
    let same_mesh_component = if evaluation
        .reachability
        .is_some_and(|reachability| reachability.same_mesh)
    {
        weights.same_mesh_bonus
    } else {
        0.0
    };
    let same_triangle_component = if evaluation
        .reachability
        .is_some_and(|reachability| reachability.same_triangle)
    {
        weights.same_triangle_bonus
    } else {
        0.0
    };
    let reachability_component = if evaluation
        .reachability
        .is_some_and(|reachability| reachability.likely_reachable)
    {
        weights.reachability_bonus
    } else {
        0.0
    };
    let cross_mesh_graph_component = if evaluation
        .reachability
        .is_some_and(|reachability| reachability.used_cross_mesh_graph)
    {
        weights.cross_mesh_graph_bonus
    } else {
        0.0
    };
    let path_cost_component = match evaluation.reachability {
        Some(reachability) if options.reachability.maximum_path_cost.is_some() => {
            normalized_distance_score(
                reachability.approximate_path_cost.map(|cost| {
                    (options.reachability.maximum_path_cost.unwrap_or_default() - cost).max(0.0)
                }),
                options.reachability.maximum_path_cost.unwrap_or_default(),
            ) * weights.path_cost_bonus
        }
        _ => 0.0,
    };
    let headroom_component = if evaluation.physics.headroom_clear {
        weights.headroom_bonus
    } else {
        0.0
    };
    let clearance_component = if evaluation.physics.clearance_clear {
        weights.clearance_bonus
    } else {
        0.0
    };
    let line_of_sight_component = match evaluation.physics.line_of_sight_clear {
        Some(false) => weights.line_of_sight_bonus,
        Some(true) => -weights.line_of_sight_bonus,
        None => 0.0,
    };
    let total = valid_component
        + actor_distance_component
        + hostile_distance_component
        + navmesh_distance_component
        + actor_density_component
        + hostile_density_component
        + navmesh_support_component
        + same_mesh_component
        + same_triangle_component
        + reachability_component
        + cross_mesh_graph_component
        + path_cost_component
        + headroom_component
        + clearance_component
        + line_of_sight_component;

    SpatialCandidateScore {
        total,
        valid_component,
        actor_distance_component,
        hostile_distance_component,
        navmesh_distance_component,
        actor_density_component,
        hostile_density_component,
        navmesh_support_component,
        same_mesh_component,
        same_triangle_component,
        reachability_component,
        cross_mesh_graph_component,
        path_cost_component,
        headroom_component,
        clearance_component,
        line_of_sight_component,
    }
}

pub fn evaluate_and_score_spawn_candidate(
    cell: &TESObjectCELL,
    candidate: NiPoint3,
    options: SpatialCandidateEvaluationOptions,
    weights: SpatialCandidateScoreWeights,
) -> ScoredSpatialCandidateEvaluation {
    evaluate_and_score_spawn_candidate_with_filter(
        cell,
        candidate,
        options,
        weights,
        &RaycastHitFilter::new(),
    )
}

pub fn evaluate_and_score_spawn_candidate_with_filter(
    cell: &TESObjectCELL,
    candidate: NiPoint3,
    options: SpatialCandidateEvaluationOptions,
    weights: SpatialCandidateScoreWeights,
    hit_filter: &RaycastHitFilter<'_>,
) -> ScoredSpatialCandidateEvaluation {
    let evaluation = evaluate_spawn_candidate_with_filter(cell, candidate, options, hit_filter);
    let score = score_spawn_candidate(&evaluation, options, weights);
    ScoredSpatialCandidateEvaluation { evaluation, score }
}

pub fn best_spawn_candidate(
    cell: &TESObjectCELL,
    candidates: &[NiPoint3],
    options: SpatialCandidateEvaluationOptions,
    weights: SpatialCandidateScoreWeights,
) -> Option<ScoredSpatialCandidateEvaluation> {
    best_spawn_candidate_with_filter(cell, candidates, options, weights, &RaycastHitFilter::new())
}

pub fn best_spawn_candidate_with_filter(
    cell: &TESObjectCELL,
    candidates: &[NiPoint3],
    options: SpatialCandidateEvaluationOptions,
    weights: SpatialCandidateScoreWeights,
    hit_filter: &RaycastHitFilter<'_>,
) -> Option<ScoredSpatialCandidateEvaluation> {
    candidates
        .iter()
        .copied()
        .map(|candidate| {
            evaluate_and_score_spawn_candidate_with_filter(
                cell, candidate, options, weights, hit_filter,
            )
        })
        .max_by(|a, b| {
            a.score
                .total
                .partial_cmp(&b.score.total)
                .unwrap_or(core::cmp::Ordering::Equal)
        })
}

pub fn evaluate_spawn_candidates(
    cell: &TESObjectCELL,
    candidates: &[NiPoint3],
    options: SpatialCandidateEvaluationOptions,
    weights: SpatialCandidateScoreWeights,
) -> SpatialCandidateSetEvaluation {
    evaluate_spawn_candidates_with_filter(
        cell,
        candidates,
        options,
        weights,
        &RaycastHitFilter::new(),
    )
}

pub fn evaluate_spawn_candidates_with_filter(
    cell: &TESObjectCELL,
    candidates: &[NiPoint3],
    options: SpatialCandidateEvaluationOptions,
    weights: SpatialCandidateScoreWeights,
    hit_filter: &RaycastHitFilter<'_>,
) -> SpatialCandidateSetEvaluation {
    let candidates = candidates
        .iter()
        .copied()
        .map(|candidate| {
            evaluate_and_score_spawn_candidate_with_filter(
                cell, candidate, options, weights, hit_filter,
            )
        })
        .collect::<Vec<_>>();
    let best_overall_index = best_index_by_score(&candidates, |_| true);
    let best_valid_index =
        best_index_by_score(&candidates, |candidate| candidate.evaluation.is_valid);

    SpatialCandidateSetEvaluation {
        candidates,
        best_overall_index,
        best_valid_index,
    }
}

pub fn evaluate_spawn_candidates_from_reference(
    reference: &TESObjectREFR,
    candidates: &[NiPoint3],
    options: SpatialCandidateEvaluationOptions,
    weights: SpatialCandidateScoreWeights,
) -> Option<SpatialCandidateSetEvaluation> {
    evaluate_spawn_candidates_from_reference_with_filter(
        reference,
        candidates,
        options,
        weights,
        &RaycastHitFilter::new(),
    )
}

pub fn evaluate_spawn_candidates_from_reference_with_filter(
    reference: &TESObjectREFR,
    candidates: &[NiPoint3],
    mut options: SpatialCandidateEvaluationOptions,
    weights: SpatialCandidateScoreWeights,
    hit_filter: &RaycastHitFilter<'_>,
) -> Option<SpatialCandidateSetEvaluation> {
    let cell = unsafe { reference.get_parent_cell().as_ref() }?;
    if options.reachability_origin.is_none() {
        options.reachability_origin = Some(reference.get_position());
    }

    Some(evaluate_spawn_candidates_with_filter(
        cell, candidates, options, weights, hit_filter,
    ))
}
