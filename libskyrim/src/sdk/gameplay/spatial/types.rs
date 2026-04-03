use alloc::vec::Vec;

use crate::re::NiPoint3;
use crate::sdk::advanced::physics::{SpawnPointValidation, SpawnPointValidationOptions};
use crate::sdk::gameplay::navmesh::{
    NavMeshPointQuery, NavMeshReachabilityHeuristics, NavMeshReachabilityHeuristicsOptions,
};
use crate::sdk::gameplay::world::WorldSceneSnapshot;

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
    pub(super) fn record(&mut self, reason: SpatialCandidateTierRejectionReason) {
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
