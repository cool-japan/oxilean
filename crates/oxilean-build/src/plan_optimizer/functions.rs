//! Build Plan Optimizer — critical path analysis and greedy scheduling.

use super::types::{BuildNode, BuildPlan, BuildSchedule};
use std::collections::{HashMap, VecDeque};

// ============================================================
// Internal graph utilities
// ============================================================

/// Build a consolidated adjacency list from both the node dependency
/// fields and the explicit edge list stored in the plan.
///
/// Returns `(deps_of, successors_of)` where:
/// - `deps_of[id]`  = list of predecessor node IDs
/// - `successors_of[id]` = list of successor node IDs
pub(super) fn build_adjacency(plan: &BuildPlan) -> (Vec<Vec<usize>>, Vec<Vec<usize>>) {
    let n = plan.nodes.len();
    let mut deps_of: Vec<Vec<usize>> = vec![Vec::new(); n];
    let mut successors_of: Vec<Vec<usize>> = vec![Vec::new(); n];

    // Build index from id → position in nodes vec.
    let id_to_idx: HashMap<usize, usize> = plan
        .nodes
        .iter()
        .enumerate()
        .map(|(i, node)| (node.id, i))
        .collect();

    // Edges from node.dependencies.
    for node in &plan.nodes {
        let to_idx = match id_to_idx.get(&node.id) {
            Some(&i) => i,
            None => continue,
        };
        for &dep_id in &node.dependencies {
            if let Some(&from_idx) = id_to_idx.get(&dep_id) {
                deps_of[to_idx].push(from_idx);
                successors_of[from_idx].push(to_idx);
            }
        }
    }

    // Additional explicit edges: (dependency_id, dependent_id).
    for &(dep_id, dep_on_id) in &plan.edges {
        if let (Some(&from_idx), Some(&to_idx)) =
            (id_to_idx.get(&dep_id), id_to_idx.get(&dep_on_id))
        {
            // Avoid duplicates.
            if !deps_of[to_idx].contains(&from_idx) {
                deps_of[to_idx].push(from_idx);
            }
            if !successors_of[from_idx].contains(&to_idx) {
                successors_of[from_idx].push(to_idx);
            }
        }
    }

    (deps_of, successors_of)
}

/// Topological sort (Kahn's algorithm) on the node index space `[0, n)`.
/// Returns `None` if the graph contains a cycle.
pub(super) fn topological_sort(
    n: usize,
    deps_of: &[Vec<usize>],
    successors_of: &[Vec<usize>],
) -> Option<Vec<usize>> {
    let mut in_degree: Vec<usize> = deps_of.iter().map(|d| d.len()).collect();
    let mut queue: VecDeque<usize> = (0..n).filter(|&i| in_degree[i] == 0).collect();
    let mut order: Vec<usize> = Vec::with_capacity(n);

    while let Some(idx) = queue.pop_front() {
        order.push(idx);
        for &succ in &successors_of[idx] {
            in_degree[succ] = in_degree[succ].saturating_sub(1);
            if in_degree[succ] == 0 {
                queue.push_back(succ);
            }
        }
    }

    if order.len() == n {
        Some(order)
    } else {
        None
    }
}

// ============================================================
// Public API
// ============================================================

/// Compute the critical path through the build DAG.
///
/// The critical path is the longest weighted path (where edge weights are
/// the `estimated_cost_ms` of each node).  It represents the minimum
/// possible build time regardless of the number of workers.
///
/// Returns the node IDs on the critical path in topological order.
/// Returns an empty `Vec` for an empty plan or if the graph is cyclic.
pub fn critical_path(plan: &BuildPlan) -> Vec<usize> {
    let n = plan.nodes.len();
    if n == 0 {
        return Vec::new();
    }

    let (deps_of, successors_of) = build_adjacency(plan);
    let topo = match topological_sort(n, &deps_of, &successors_of) {
        Some(t) => t,
        None => return Vec::new(), // cycle — degenerate
    };

    // Longest-path DP in topological order.
    // dp_cost[i]   = maximum cost to reach END by passing through node i
    // dp_succ[i]   = index of successor chosen on the longest path
    let costs: Vec<u64> = plan.nodes.iter().map(|n| n.estimated_cost_ms).collect();
    let mut dp_cost: Vec<u64> = costs.clone();
    let mut dp_succ: Vec<Option<usize>> = vec![None; n];

    // Process in REVERSE topological order so each node can look at its
    // successors' already-computed values.
    for &idx in topo.iter().rev() {
        for &succ in &successors_of[idx] {
            let candidate = costs[idx] + dp_cost[succ];
            if candidate > dp_cost[idx] {
                dp_cost[idx] = candidate;
                dp_succ[idx] = Some(succ);
            }
        }
    }

    // Find the starting node (maximum dp_cost over all nodes with zero in-degree).
    let start = topo
        .iter()
        .filter(|&&i| deps_of[i].is_empty())
        .copied()
        .max_by_key(|&i| dp_cost[i]);

    let start_idx = match start {
        Some(s) => s,
        None => return Vec::new(),
    };

    // Walk the chosen path.
    let mut path_indices: Vec<usize> = Vec::new();
    let mut cur = Some(start_idx);
    while let Some(idx) = cur {
        path_indices.push(idx);
        cur = dp_succ[idx];
    }

    // Convert internal indices back to node IDs.
    path_indices.into_iter().map(|i| plan.nodes[i].id).collect()
}

/// Schedule `plan` onto `num_workers` parallel workers.
///
/// The scheduler uses a greedy, list-scheduling algorithm:
///
/// 1. Compute a topological order weighted by "bottom-level" cost
///    (longest remaining path), giving priority to nodes on the critical path.
/// 2. Process nodes in that priority order.  A node becomes *ready* once all
///    its dependencies have been assigned.
/// 3. Assign each ready node to the worker whose current load (sum of
///    assigned node costs) is smallest — minimising estimated makespan.
///
/// Returns a [`BuildSchedule`] whose `lanes[i]` lists the node IDs for
/// worker `i` in execution order.
pub fn schedule(plan: &BuildPlan, num_workers: usize) -> BuildSchedule {
    let n = plan.nodes.len();
    let num_workers = num_workers.max(1);

    if n == 0 {
        return BuildSchedule {
            lanes: vec![Vec::new(); num_workers],
            critical_path: Vec::new(),
            estimated_makespan_ms: 0,
        };
    }

    let (deps_of, successors_of) = build_adjacency(plan);
    let topo = match topological_sort(n, &deps_of, &successors_of) {
        Some(t) => t,
        None => {
            // Cyclic graph — return a trivial single-worker schedule.
            let all_ids: Vec<usize> = plan.nodes.iter().map(|nd| nd.id).collect();
            let cp = critical_path(plan);
            return BuildSchedule {
                lanes: {
                    let mut lanes = vec![Vec::new(); num_workers];
                    lanes[0] = all_ids;
                    lanes
                },
                critical_path: cp,
                estimated_makespan_ms: plan.nodes.iter().map(|nd| nd.estimated_cost_ms).sum(),
            };
        }
    };

    let costs: Vec<u64> = plan.nodes.iter().map(|nd| nd.estimated_cost_ms).collect();

    // Bottom-level cost: longest path from this node to any sink (inclusive).
    let mut bottom_level: Vec<u64> = costs.clone();
    for &idx in topo.iter().rev() {
        for &succ in &successors_of[idx] {
            let candidate = costs[idx] + bottom_level[succ];
            if candidate > bottom_level[idx] {
                bottom_level[idx] = candidate;
            }
        }
    }

    // Worker state: time at which the worker becomes free.
    let mut worker_free_at: Vec<u64> = vec![0u64; num_workers];
    // lanes[worker] = ordered list of node indices assigned so far.
    let mut lanes_idx: Vec<Vec<usize>> = vec![Vec::new(); num_workers];
    // Finish time of each node (when it is fully done).
    let mut finish_at: Vec<u64> = vec![0u64; n];

    // Track how many predecessors of each node still haven't been assigned.
    let mut unmet: Vec<usize> = deps_of.iter().map(|d| d.len()).collect();
    // Queue of nodes that are ready to be scheduled (unmet == 0).
    // We use a priority queue ordered by bottom_level (descending).
    // For simplicity we use a sorted Vec since n is typically small; for
    // very large graphs a proper heap would be preferable.
    let mut ready: Vec<usize> = topo.iter().copied().filter(|&i| unmet[i] == 0).collect();
    ready.sort_by(|&a, &b| bottom_level[b].cmp(&bottom_level[a]));

    while !ready.is_empty() {
        // Pop highest-priority (critical) node.
        let idx = ready.remove(0);

        // Earliest the node can start: max of all predecessor finish times.
        let earliest_start: u64 = deps_of[idx]
            .iter()
            .map(|&dep| finish_at[dep])
            .max()
            .unwrap_or(0);

        // Pick the worker that can start this node the earliest:
        // the worker is free at worker_free_at[w]; we need
        // max(worker_free_at[w], earliest_start) to be minimal.
        let worker = worker_free_at
            .iter()
            .enumerate()
            .min_by_key(|&(_, &free)| free.max(earliest_start))
            .map(|(w, _)| w)
            .unwrap_or(0);

        let start_time = worker_free_at[worker].max(earliest_start);
        let end_time = start_time + costs[idx];

        worker_free_at[worker] = end_time;
        finish_at[idx] = end_time;
        lanes_idx[worker].push(idx);

        // Decrement unmet count for each successor; enqueue if now ready.
        for &succ in &successors_of[idx] {
            unmet[succ] = unmet[succ].saturating_sub(1);
            if unmet[succ] == 0 {
                // Insert into ready list maintaining descending bottom_level order.
                let pos = ready
                    .binary_search_by(|&x| bottom_level[succ].cmp(&bottom_level[x]))
                    .unwrap_or_else(|e| e);
                ready.insert(pos, succ);
            }
        }
    }

    // Convert internal node indices → public node IDs for lanes.
    let lanes: Vec<Vec<usize>> = lanes_idx
        .iter()
        .map(|lane| lane.iter().map(|&i| plan.nodes[i].id).collect())
        .collect();

    let cp = critical_path(plan);
    // Actual makespan is the maximum finish time over all nodes.
    let makespan = finish_at.into_iter().max().unwrap_or(0);

    BuildSchedule {
        lanes,
        critical_path: cp,
        estimated_makespan_ms: makespan,
    }
}

// ============================================================
// Validation helpers (public utility functions)
// ============================================================

/// Returns `true` if the plan contains no dependency cycles.
pub fn is_acyclic(plan: &BuildPlan) -> bool {
    let n = plan.nodes.len();
    if n == 0 {
        return true;
    }
    let (deps_of, successors_of) = build_adjacency(plan);
    topological_sort(n, &deps_of, &successors_of).is_some()
}

/// Returns the total estimated cost if all nodes were executed sequentially
/// (sum of all `estimated_cost_ms`).
pub fn sequential_cost(plan: &BuildPlan) -> u64 {
    plan.nodes.iter().map(|n| n.estimated_cost_ms).sum()
}

/// Returns the minimum possible makespan (cost of the critical path).
pub fn minimum_makespan(plan: &BuildPlan) -> u64 {
    let cp = critical_path(plan);
    let id_to_cost: HashMap<usize, u64> = plan
        .nodes
        .iter()
        .map(|n| (n.id, n.estimated_cost_ms))
        .collect();
    cp.iter()
        .map(|id| id_to_cost.get(id).copied().unwrap_or(0))
        .sum()
}

/// Returns the nodes that have no dependencies (roots of the DAG).
pub fn root_nodes(plan: &BuildPlan) -> Vec<usize> {
    plan.nodes
        .iter()
        .filter(|n| n.dependencies.is_empty())
        .map(|n| n.id)
        .collect()
}

/// Returns the nodes that no other node depends on (leaves/sinks of the DAG).
pub fn leaf_nodes(plan: &BuildPlan) -> Vec<usize> {
    let (_, successors_of) = build_adjacency(plan);
    plan.nodes
        .iter()
        .enumerate()
        .filter(|(i, _)| successors_of[*i].is_empty())
        .map(|(_, n)| n.id)
        .collect()
}
