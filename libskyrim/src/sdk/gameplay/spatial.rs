//! Higher-level spatial candidate evaluation built on top of world, actor, and
//! physics helpers.

use alloc::vec::Vec;

use crate::re::{NiPoint3, TESObjectCELL, TESObjectREFR};
use crate::sdk::advanced::physics::{
    RaycastHitFilter, SpawnPointValidation, SpawnPointValidationOptions,
    validate_spawn_point_with_filter,
};
use crate::sdk::gameplay::navmesh::{
    self, NavMeshPointQuery, NavMeshReachabilityHeuristics, NavMeshReachabilityHeuristicsOptions,
};
use crate::sdk::gameplay::world::{self, WorldSceneSnapshot};

#[derive(Debug, Clone, Copy)]
pub struct SpatialCandidateEvaluationOptions {
    pub scene_radius: f32,
    pub minimum_actor_distance: f32,
    pub minimum_hostile_distance: f32,
    pub maximum_navmesh_distance: f32,
    pub reachability_origin: Option<NiPoint3>,
    pub reachability: NavMeshReachabilityHeuristicsOptions,
    pub physics: SpawnPointValidationOptions,
}

#[derive(Debug, Clone)]
pub struct SpatialCandidateEvaluation {
    pub candidate: NiPoint3,
    pub scene_center: Option<NiPoint3>,
    pub scene: WorldSceneSnapshot,
    pub physics: SpawnPointValidation,
    pub nearest_actor_distance: Option<f32>,
    pub nearest_hostile_distance: Option<f32>,
    pub navmesh: NavMeshPointQuery,
    pub reachability: Option<NavMeshReachabilityHeuristics>,
    pub satisfies_actor_distance: bool,
    pub satisfies_hostile_distance: bool,
    pub satisfies_navmesh_distance: bool,
    pub satisfies_reachability: bool,
    pub is_valid: bool,
}

#[derive(Debug, Clone, Copy)]
pub struct SpatialCandidateScoreWeights {
    pub valid_bonus: f32,
    pub actor_distance: f32,
    pub hostile_distance: f32,
    pub navmesh_distance: f32,
    pub actor_density_penalty: f32,
    pub hostile_density_penalty: f32,
    pub navmesh_support_bonus: f32,
    pub same_mesh_bonus: f32,
    pub same_triangle_bonus: f32,
    pub reachability_bonus: f32,
    pub cross_mesh_graph_bonus: f32,
    pub path_cost_bonus: f32,
    pub headroom_bonus: f32,
    pub clearance_bonus: f32,
    pub line_of_sight_bonus: f32,
}

impl Default for SpatialCandidateScoreWeights {
    fn default() -> Self {
        Self {
            valid_bonus: 10.0,
            actor_distance: 2.0,
            hostile_distance: 4.0,
            navmesh_distance: 1.5,
            actor_density_penalty: 0.1,
            hostile_density_penalty: 0.25,
            navmesh_support_bonus: 1.0,
            same_mesh_bonus: 1.0,
            same_triangle_bonus: 1.0,
            reachability_bonus: 2.0,
            cross_mesh_graph_bonus: 0.5,
            path_cost_bonus: 1.0,
            headroom_bonus: 1.0,
            clearance_bonus: 1.0,
            line_of_sight_bonus: 1.5,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SpatialCandidateScore {
    pub total: f32,
    pub valid_component: f32,
    pub actor_distance_component: f32,
    pub hostile_distance_component: f32,
    pub navmesh_distance_component: f32,
    pub actor_density_component: f32,
    pub hostile_density_component: f32,
    pub navmesh_support_component: f32,
    pub same_mesh_component: f32,
    pub same_triangle_component: f32,
    pub reachability_component: f32,
    pub cross_mesh_graph_component: f32,
    pub path_cost_component: f32,
    pub headroom_component: f32,
    pub clearance_component: f32,
    pub line_of_sight_component: f32,
}

#[derive(Debug, Clone)]
pub struct ScoredSpatialCandidateEvaluation {
    pub evaluation: SpatialCandidateEvaluation,
    pub score: SpatialCandidateScore,
}

#[derive(Debug, Clone)]
pub struct SpatialCandidateSetEvaluation {
    pub candidates: Vec<ScoredSpatialCandidateEvaluation>,
    pub best_overall_index: Option<usize>,
    pub best_valid_index: Option<usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SpatialCandidateTier {
    pub name: &'static str,
    pub require_ground_snap: bool,
    pub require_headroom: bool,
    pub require_clearance: bool,
    pub require_broken_line_of_sight: bool,
    pub require_actor_distance: bool,
    pub require_hostile_distance: bool,
    pub require_navmesh_distance: bool,
    pub require_reachability: bool,
}

impl SpatialCandidateTier {
    #[must_use]
    pub const fn strict_respawn() -> Self {
        Self {
            name: "strict_respawn",
            require_ground_snap: true,
            require_headroom: true,
            require_clearance: true,
            require_broken_line_of_sight: false,
            require_actor_distance: true,
            require_hostile_distance: true,
            require_navmesh_distance: true,
            require_reachability: true,
        }
    }

    #[must_use]
    pub const fn occluded_respawn() -> Self {
        Self {
            name: "occluded_respawn",
            require_ground_snap: true,
            require_headroom: true,
            require_clearance: true,
            require_broken_line_of_sight: true,
            require_actor_distance: true,
            require_hostile_distance: true,
            require_navmesh_distance: true,
            require_reachability: true,
        }
    }

    #[must_use]
    pub const fn relaxed_respawn() -> Self {
        Self {
            name: "relaxed_respawn",
            require_ground_snap: true,
            require_headroom: true,
            require_clearance: true,
            require_broken_line_of_sight: false,
            require_actor_distance: false,
            require_hostile_distance: true,
            require_navmesh_distance: true,
            require_reachability: false,
        }
    }

    #[must_use]
    pub const fn emergency_respawn() -> Self {
        Self {
            name: "emergency_respawn",
            require_ground_snap: true,
            require_headroom: false,
            require_clearance: false,
            require_broken_line_of_sight: false,
            require_actor_distance: false,
            require_hostile_distance: false,
            require_navmesh_distance: true,
            require_reachability: false,
        }
    }

    #[must_use]
    pub const fn score_only(name: &'static str) -> Self {
        Self {
            name,
            require_ground_snap: false,
            require_headroom: false,
            require_clearance: false,
            require_broken_line_of_sight: false,
            require_actor_distance: false,
            require_hostile_distance: false,
            require_navmesh_distance: false,
            require_reachability: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SpatialCandidateTierSelection {
    pub tier_index: usize,
    pub tier: SpatialCandidateTier,
    pub candidate_index: usize,
    pub selected: ScoredSpatialCandidateEvaluation,
    pub diagnostics: SpatialCandidateTierDiagnostics,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpatialCandidateTierRejectionReason {
    GroundSnapRequired,
    HeadroomRequired,
    ClearanceRequired,
    BrokenLineOfSightRequired,
    ActorDistanceRequired,
    HostileDistanceRequired,
    NavmeshDistanceRequired,
    ReachabilityRequired,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SpatialCandidateTierRejectionSummary {
    pub ground_snap_required: usize,
    pub headroom_required: usize,
    pub clearance_required: usize,
    pub broken_line_of_sight_required: usize,
    pub actor_distance_required: usize,
    pub hostile_distance_required: usize,
    pub navmesh_distance_required: usize,
    pub reachability_required: usize,
}

impl SpatialCandidateTierRejectionSummary {
    fn record(&mut self, reason: SpatialCandidateTierRejectionReason) {
        match reason {
            SpatialCandidateTierRejectionReason::GroundSnapRequired => {
                self.ground_snap_required += 1;
            }
            SpatialCandidateTierRejectionReason::HeadroomRequired => {
                self.headroom_required += 1;
            }
            SpatialCandidateTierRejectionReason::ClearanceRequired => {
                self.clearance_required += 1;
            }
            SpatialCandidateTierRejectionReason::BrokenLineOfSightRequired => {
                self.broken_line_of_sight_required += 1;
            }
            SpatialCandidateTierRejectionReason::ActorDistanceRequired => {
                self.actor_distance_required += 1;
            }
            SpatialCandidateTierRejectionReason::HostileDistanceRequired => {
                self.hostile_distance_required += 1;
            }
            SpatialCandidateTierRejectionReason::NavmeshDistanceRequired => {
                self.navmesh_distance_required += 1;
            }
            SpatialCandidateTierRejectionReason::ReachabilityRequired => {
                self.reachability_required += 1;
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct SpatialCandidateTierCandidateDiagnostics {
    pub candidate_index: usize,
    pub candidate: NiPoint3,
    pub score_total: f32,
    pub passes_tier: bool,
    pub rejection_reasons: Vec<SpatialCandidateTierRejectionReason>,
}

#[derive(Debug, Clone)]
pub struct SpatialCandidateTierDiagnostics {
    pub tier_index: usize,
    pub tier: SpatialCandidateTier,
    pub best_candidate_index: Option<usize>,
    pub passed_candidate_count: usize,
    pub rejected_candidate_count: usize,
    pub rejection_summary: SpatialCandidateTierRejectionSummary,
    pub candidate_diagnostics: Vec<SpatialCandidateTierCandidateDiagnostics>,
}

#[derive(Debug, Clone)]
pub struct SpatialCandidateFallbackSelection {
    pub evaluations: SpatialCandidateSetEvaluation,
    pub tier_diagnostics: Vec<SpatialCandidateTierDiagnostics>,
    pub selected: Option<SpatialCandidateTierSelection>,
}

#[inline(always)]
fn is_valid_radius(radius: f32, caller: &'static str) -> bool {
    if !radius.is_finite() || radius <= 0.0 {
        crate::defensive_sdk_warn!(
            "{} ignored non-positive or non-finite radius={}",
            caller,
            radius
        );
        false
    } else {
        true
    }
}

#[inline(always)]
fn nearest_distance_to_points(origin: NiPoint3, points: &[NiPoint3]) -> Option<f32> {
    points
        .iter()
        .map(|point| origin.get_distance(*point))
        .min_by(|a, b| a.partial_cmp(b).unwrap_or(core::cmp::Ordering::Equal))
}

#[inline(always)]
fn satisfies_minimum_distance(minimum: f32, actual: Option<f32>) -> bool {
    if minimum <= f32::EPSILON {
        true
    } else {
        actual.is_none_or(|distance| distance >= minimum)
    }
}

#[inline(always)]
fn normalized_distance_score(distance: Option<f32>, scene_radius: f32) -> f32 {
    if scene_radius <= f32::EPSILON {
        return 0.0;
    }

    distance
        .map(|distance| (distance / scene_radius).clamp(0.0, 1.0))
        .unwrap_or(0.0)
}

#[inline(always)]
fn normalized_scene_distance_score(distance: Option<f32>, scene_radius: f32) -> f32 {
    if scene_radius <= f32::EPSILON {
        return 0.0;
    }

    distance
        .map(|distance| (distance / scene_radius).clamp(0.0, 1.0))
        .unwrap_or(1.0)
}

#[inline(always)]
fn satisfies_navmesh_distance(maximum: f32, actual: Option<f32>) -> bool {
    if maximum <= f32::EPSILON {
        true
    } else {
        actual.is_some_and(|distance| distance <= maximum)
    }
}

#[inline(always)]
fn has_ground_snap(evaluation: &SpatialCandidateEvaluation) -> bool {
    evaluation.physics.snapped_point.is_some()
}

#[inline(always)]
fn has_broken_line_of_sight(evaluation: &SpatialCandidateEvaluation) -> bool {
    evaluation.physics.line_of_sight_clear == Some(false)
}

fn tier_rejection_reasons(
    evaluation: &SpatialCandidateEvaluation,
    tier: SpatialCandidateTier,
) -> Vec<SpatialCandidateTierRejectionReason> {
    let mut reasons = Vec::new();

    if tier.require_ground_snap && !has_ground_snap(evaluation) {
        reasons.push(SpatialCandidateTierRejectionReason::GroundSnapRequired);
    }
    if tier.require_headroom && !evaluation.physics.headroom_clear {
        reasons.push(SpatialCandidateTierRejectionReason::HeadroomRequired);
    }
    if tier.require_clearance && !evaluation.physics.clearance_clear {
        reasons.push(SpatialCandidateTierRejectionReason::ClearanceRequired);
    }
    if tier.require_broken_line_of_sight && !has_broken_line_of_sight(evaluation) {
        reasons.push(SpatialCandidateTierRejectionReason::BrokenLineOfSightRequired);
    }
    if tier.require_actor_distance && !evaluation.satisfies_actor_distance {
        reasons.push(SpatialCandidateTierRejectionReason::ActorDistanceRequired);
    }
    if tier.require_hostile_distance && !evaluation.satisfies_hostile_distance {
        reasons.push(SpatialCandidateTierRejectionReason::HostileDistanceRequired);
    }
    if tier.require_navmesh_distance && !evaluation.satisfies_navmesh_distance {
        reasons.push(SpatialCandidateTierRejectionReason::NavmeshDistanceRequired);
    }
    if tier.require_reachability && !evaluation.satisfies_reachability {
        reasons.push(SpatialCandidateTierRejectionReason::ReachabilityRequired);
    }

    reasons
}

#[inline(always)]
fn satisfies_tier(evaluation: &SpatialCandidateEvaluation, tier: SpatialCandidateTier) -> bool {
    tier_rejection_reasons(evaluation, tier).is_empty()
}

fn tier_diagnostics(
    evaluations: &SpatialCandidateSetEvaluation,
    tier_index: usize,
    tier: SpatialCandidateTier,
) -> SpatialCandidateTierDiagnostics {
    let candidate_diagnostics = evaluations
        .candidates
        .iter()
        .enumerate()
        .map(|(candidate_index, candidate)| {
            let rejection_reasons = tier_rejection_reasons(&candidate.evaluation, tier);
            SpatialCandidateTierCandidateDiagnostics {
                candidate_index,
                candidate: candidate.evaluation.candidate,
                score_total: candidate.score.total,
                passes_tier: rejection_reasons.is_empty(),
                rejection_reasons,
            }
        })
        .collect::<Vec<_>>();

    let best_candidate_index = candidate_diagnostics
        .iter()
        .filter(|candidate| candidate.passes_tier)
        .max_by(|a, b| {
            a.score_total
                .partial_cmp(&b.score_total)
                .unwrap_or(core::cmp::Ordering::Equal)
        })
        .map(|candidate| candidate.candidate_index);

    let mut rejection_summary = SpatialCandidateTierRejectionSummary::default();
    let mut passed_candidate_count = 0usize;
    let mut rejected_candidate_count = 0usize;
    for candidate in &candidate_diagnostics {
        if candidate.passes_tier {
            passed_candidate_count += 1;
        } else {
            rejected_candidate_count += 1;
            for reason in &candidate.rejection_reasons {
                rejection_summary.record(*reason);
            }
        }
    }

    SpatialCandidateTierDiagnostics {
        tier_index,
        tier,
        best_candidate_index,
        passed_candidate_count,
        rejected_candidate_count,
        rejection_summary,
        candidate_diagnostics,
    }
}

pub fn diagnose_spawn_candidates_for_tier(
    evaluations: &SpatialCandidateSetEvaluation,
    tier: SpatialCandidateTier,
) -> SpatialCandidateTierDiagnostics {
    tier_diagnostics(evaluations, 0, tier)
}

fn best_index_by_score(
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

pub fn best_spawn_candidate_for_tier(
    evaluations: &SpatialCandidateSetEvaluation,
    tier: SpatialCandidateTier,
) -> Option<SpatialCandidateTierSelection> {
    let diagnostics = tier_diagnostics(evaluations, 0, tier);
    let candidate_index = best_index_by_score(&evaluations.candidates, |candidate| {
        satisfies_tier(&candidate.evaluation, tier)
    })?;

    Some(SpatialCandidateTierSelection {
        tier_index: 0,
        tier,
        candidate_index,
        selected: evaluations.candidates[candidate_index].clone(),
        diagnostics,
    })
}

pub fn select_spawn_candidate_with_fallbacks(
    cell: &TESObjectCELL,
    candidates: &[NiPoint3],
    options: SpatialCandidateEvaluationOptions,
    weights: SpatialCandidateScoreWeights,
    tiers: &[SpatialCandidateTier],
) -> SpatialCandidateFallbackSelection {
    select_spawn_candidate_with_fallbacks_and_filter(
        cell,
        candidates,
        options,
        weights,
        tiers,
        &RaycastHitFilter::new(),
    )
}

pub fn select_spawn_candidate_with_fallbacks_and_filter(
    cell: &TESObjectCELL,
    candidates: &[NiPoint3],
    options: SpatialCandidateEvaluationOptions,
    weights: SpatialCandidateScoreWeights,
    tiers: &[SpatialCandidateTier],
    hit_filter: &RaycastHitFilter<'_>,
) -> SpatialCandidateFallbackSelection {
    let evaluations =
        evaluate_spawn_candidates_with_filter(cell, candidates, options, weights, hit_filter);
    let tier_diagnostics = tiers
        .iter()
        .copied()
        .enumerate()
        .map(|(tier_index, tier)| tier_diagnostics(&evaluations, tier_index, tier))
        .collect::<Vec<_>>();
    let selected = tiers
        .iter()
        .copied()
        .enumerate()
        .find_map(|(tier_index, tier)| {
            best_index_by_score(&evaluations.candidates, |candidate| {
                satisfies_tier(&candidate.evaluation, tier)
            })
            .map(|candidate_index| SpatialCandidateTierSelection {
                tier_index,
                tier,
                candidate_index,
                selected: evaluations.candidates[candidate_index].clone(),
                diagnostics: tier_diagnostics[tier_index].clone(),
            })
        });

    SpatialCandidateFallbackSelection {
        evaluations,
        tier_diagnostics,
        selected,
    }
}

pub fn select_spawn_candidate_from_reference_with_fallbacks(
    reference: &TESObjectREFR,
    candidates: &[NiPoint3],
    options: SpatialCandidateEvaluationOptions,
    weights: SpatialCandidateScoreWeights,
    tiers: &[SpatialCandidateTier],
) -> Option<SpatialCandidateFallbackSelection> {
    select_spawn_candidate_from_reference_with_fallbacks_and_filter(
        reference,
        candidates,
        options,
        weights,
        tiers,
        &RaycastHitFilter::new(),
    )
}

pub fn select_spawn_candidate_from_reference_with_fallbacks_and_filter(
    reference: &TESObjectREFR,
    candidates: &[NiPoint3],
    mut options: SpatialCandidateEvaluationOptions,
    weights: SpatialCandidateScoreWeights,
    tiers: &[SpatialCandidateTier],
    hit_filter: &RaycastHitFilter<'_>,
) -> Option<SpatialCandidateFallbackSelection> {
    let cell = unsafe { reference.get_parent_cell().as_ref() }?;
    if options.reachability_origin.is_none() {
        options.reachability_origin = Some(reference.get_position());
    }

    Some(select_spawn_candidate_with_fallbacks_and_filter(
        cell, candidates, options, weights, tiers, hit_filter,
    ))
}
