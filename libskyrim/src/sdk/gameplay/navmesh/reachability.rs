use crate::re::{TESObjectCELL, TESObjectREFR};
use crate::sdk::core::GamePtr;

use super::query::snapshot_cell_navmeshes;
use super::shared::{
    approximate_cross_mesh_snapshot_path_cost, approximate_same_mesh_triangle_path_cost,
    empty_point_query, nav_mesh_ref, query_point_in_snapshot, reference_from_ptr,
    validate_reachability_options,
};
use super::types::{NavMeshReachabilityHeuristics, NavMeshReachabilityHeuristicsOptions};

/// Evaluate approximate reachability between two points inside one cell.
///
/// This combines nearest-support queries with same-mesh or cross-mesh graph
/// heuristics and returns a detailed [`NavMeshReachabilityHeuristics`] summary.
pub fn evaluate_reachability_in_cell(
    cell: &TESObjectCELL,
    from: crate::re::NiPoint3,
    to: crate::re::NiPoint3,
    options: NavMeshReachabilityHeuristicsOptions,
) -> NavMeshReachabilityHeuristics {
    let direct_distance = from.get_distance(to);
    if !validate_reachability_options(options) {
        return NavMeshReachabilityHeuristics {
            from,
            to,
            direct_distance,
            from_query: empty_point_query(from),
            to_query: empty_point_query(to),
            same_mesh: false,
            same_triangle: false,
            support_ok: false,
            path_cost_ok: false,
            triangle_hops: None,
            mesh_hops: None,
            approximate_path_cost: None,
            used_cross_mesh_graph: false,
            likely_reachable: false,
        };
    }

    let snapshot = snapshot_cell_navmeshes(cell);
    let from_query = query_point_in_snapshot(&snapshot, from, options.minimum_offset);
    let to_query = query_point_in_snapshot(&snapshot, to, options.minimum_offset);
    let support_ok = from_query
        .nearest_support_distance()
        .is_some_and(|distance| distance <= options.maximum_support_distance)
        && to_query
            .nearest_support_distance()
            .is_some_and(|distance| distance <= options.maximum_support_distance);

    let same_mesh = matches!(
        (from_query.nearest_triangle_support, to_query.nearest_triangle_support),
        (Some(from_support), Some(to_support)) if from_support.mesh_index == to_support.mesh_index
    );
    let same_triangle = matches!(
        (from_query.nearest_triangle_support, to_query.nearest_triangle_support),
        (Some(from_support), Some(to_support))
            if from_support.mesh_index == to_support.mesh_index
                && from_support.triangle_index == to_support.triangle_index
    );

    let (approximate_path_cost, triangle_hops, mesh_hops, used_cross_mesh_graph) = match (
        from_query.nearest_triangle_support,
        to_query.nearest_triangle_support,
    ) {
        (Some(from_support), Some(to_support))
            if from_support.mesh_index == to_support.mesh_index =>
        {
            match snapshot
                .meshes
                .get(from_support.mesh_index)
                .and_then(nav_mesh_ref)
                .and_then(|nav_mesh| {
                    approximate_same_mesh_triangle_path_cost(
                        nav_mesh,
                        from_support.triangle_index,
                        to_support.triangle_index,
                    )
                }) {
                Some((triangle_center_cost, hops)) => (
                    Some(from_support.distance + triangle_center_cost + to_support.distance),
                    Some(hops),
                    None,
                    false,
                ),
                None => (None, None, None, false),
            }
        }
        (Some(from_support), Some(to_support)) if options.allow_cross_mesh_graph => {
            match approximate_cross_mesh_snapshot_path_cost(&snapshot, from_support, to_support) {
                Some((graph_cost, hops)) => (
                    Some(from_support.distance + graph_cost + to_support.distance),
                    None,
                    Some(hops),
                    true,
                ),
                None => (None, None, None, false),
            }
        }
        _ => (None, None, None, false),
    };

    let path_cost_ok = match (options.maximum_path_cost, approximate_path_cost) {
        (Some(maximum_path_cost), Some(path_cost)) => path_cost <= maximum_path_cost,
        (Some(_), None) => false,
        (None, Some(_)) => true,
        (None, None) => false,
    };
    let likely_reachable = support_ok && approximate_path_cost.is_some() && path_cost_ok;

    NavMeshReachabilityHeuristics {
        from,
        to,
        direct_distance,
        from_query,
        to_query,
        same_mesh,
        same_triangle,
        support_ok,
        path_cost_ok,
        triangle_hops,
        mesh_hops,
        approximate_path_cost,
        used_cross_mesh_graph,
        likely_reachable,
    }
}

/// Evaluate approximate reachability from a reference's current position to a
/// goal point.
pub fn evaluate_reachability_from_reference(
    reference: &TESObjectREFR,
    to: crate::re::NiPoint3,
    options: NavMeshReachabilityHeuristicsOptions,
) -> Option<NavMeshReachabilityHeuristics> {
    let cell = unsafe { reference.get_parent_cell().as_ref() }?;
    Some(evaluate_reachability_in_cell(
        cell,
        reference.get_position(),
        to,
        options,
    ))
}

pub fn evaluate_reachability_from_reference_ptr(
    reference: GamePtr<TESObjectREFR>,
    to: crate::re::NiPoint3,
    options: NavMeshReachabilityHeuristicsOptions,
) -> Option<NavMeshReachabilityHeuristics> {
    let reference = reference_from_ptr(
        reference,
        "sdk::gameplay::navmesh::evaluate_reachability_from_reference_ptr()",
    )?;
    evaluate_reachability_from_reference(reference.as_ref(), to, options)
}
