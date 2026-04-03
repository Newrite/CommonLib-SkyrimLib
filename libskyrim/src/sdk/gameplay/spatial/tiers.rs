use alloc::vec::Vec;

use crate::re::{NiPoint3, TESObjectCELL, TESObjectREFR};
use crate::sdk::advanced::physics::RaycastHitFilter;

use super::evaluation::evaluate_spawn_candidates_with_filter;
use super::shared::best_index_by_score;
use super::types::{
    SpatialCandidateEvaluation, SpatialCandidateEvaluationOptions,
    SpatialCandidateFallbackSelection, SpatialCandidateScoreWeights, SpatialCandidateSetEvaluation,
    SpatialCandidateTier, SpatialCandidateTierCandidateDiagnostics,
    SpatialCandidateTierDiagnostics, SpatialCandidateTierRejectionReason,
    SpatialCandidateTierRejectionSummary, SpatialCandidateTierSelection,
};

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
