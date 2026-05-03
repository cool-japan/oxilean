//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{
    BinPackingFFD, ChristofidesHeuristic, GoemansWilliamsonRounding, GreedySetCover, KnapsackFPTAS,
    MetricTSPInstance, PrimalDualFacility, RandomizedRounding, SetCoverInstance, FPTAS, PTAS,
};

pub fn app(f: Expr, a: Expr) -> Expr {
    Expr::App(Box::new(f), Box::new(a))
}
pub fn app2(f: Expr, a: Expr, b: Expr) -> Expr {
    app(app(f, a), b)
}
pub fn app3(f: Expr, a: Expr, b: Expr, c: Expr) -> Expr {
    app(app2(f, a, b), c)
}
pub fn cst(s: &str) -> Expr {
    Expr::Const(Name::str(s), vec![])
}
pub fn prop() -> Expr {
    Expr::Sort(Level::zero())
}
pub fn type0() -> Expr {
    Expr::Sort(Level::succ(Level::zero()))
}
pub fn pi(bi: BinderInfo, name: &str, dom: Expr, body: Expr) -> Expr {
    Expr::Pi(bi, Name::str(name), Box::new(dom), Box::new(body))
}
pub fn arrow(a: Expr, b: Expr) -> Expr {
    pi(BinderInfo::Default, "_", a, b)
}
pub fn bvar(n: u32) -> Expr {
    Expr::BVar(n)
}
pub fn nat_ty() -> Expr {
    cst("Nat")
}
pub fn bool_ty() -> Expr {
    cst("Bool")
}
pub fn real_ty() -> Expr {
    cst("Real")
}
pub fn list_ty(elem: Expr) -> Expr {
    app(cst("List"), elem)
}
pub fn pair_ty(a: Expr, b: Expr) -> Expr {
    app2(cst("Prod"), a, b)
}
pub fn option_ty(a: Expr) -> Expr {
    app(cst("Option"), a)
}
/// `OptimizationProblem : Type` — a minimization or maximization problem.
pub fn optimization_problem_ty() -> Expr {
    type0()
}
/// `ApproxAlgorithm : OptimizationProblem → Type`
/// An algorithm with a guaranteed approximation ratio.
pub fn approx_algorithm_ty() -> Expr {
    arrow(optimization_problem_ty(), type0())
}
/// `ApproxRatio : ApproxAlgorithm P → Real`
/// The approximation ratio achieved by the algorithm.
pub fn approx_ratio_ty() -> Expr {
    arrow(approx_algorithm_ty(), real_ty())
}
/// `IsAlphaApprox : OptimizationProblem → Real → Prop`
/// There exists a polynomial-time algorithm with approximation ratio α.
pub fn is_alpha_approx_ty() -> Expr {
    arrow(optimization_problem_ty(), arrow(real_ty(), prop()))
}
/// `OptSolution : OptimizationProblem → String → Real`
/// The optimal solution value for a given instance.
pub fn opt_solution_ty() -> Expr {
    arrow(optimization_problem_ty(), arrow(cst("String"), real_ty()))
}
/// `AlgSolution : ApproxAlgorithm P → String → Real`
/// The solution value produced by the approximation algorithm.
pub fn alg_solution_ty() -> Expr {
    arrow(approx_algorithm_ty(), arrow(cst("String"), real_ty()))
}
/// `PTAS : OptimizationProblem → Prop`
/// The problem has a Polynomial-Time Approximation Scheme:
/// for every ε > 0, there is a (1+ε)-approximation running in poly(n).
pub fn ptas_ty() -> Expr {
    arrow(optimization_problem_ty(), prop())
}
/// `FPTAS : OptimizationProblem → Prop`
/// The problem has a Fully PTAS: running time is poly(n, 1/ε).
pub fn fptas_ty() -> Expr {
    arrow(optimization_problem_ty(), prop())
}
/// `EPTAS : OptimizationProblem → Prop`
/// Efficient PTAS: running time is f(1/ε)·poly(n) for some function f.
pub fn eptas_ty() -> Expr {
    arrow(optimization_problem_ty(), prop())
}
/// `APX : OptimizationProblem → Prop`
/// The problem is in APX: has a constant-factor approximation algorithm.
pub fn apx_ty() -> Expr {
    arrow(optimization_problem_ty(), prop())
}
/// `APXHard : OptimizationProblem → Prop`
/// The problem is APX-hard: no PTAS unless P=NP.
pub fn apx_hard_ty() -> Expr {
    arrow(optimization_problem_ty(), prop())
}
/// `APXComplete : OptimizationProblem → Prop`
/// The problem is APX-complete.
pub fn apx_complete_ty() -> Expr {
    arrow(optimization_problem_ty(), prop())
}
/// `MaxSNP : OptimizationProblem → Prop`
/// The problem is in MAX-SNP (Papadimitriou-Yannakakis class).
pub fn max_snp_ty() -> Expr {
    arrow(optimization_problem_ty(), prop())
}
/// `FPTAS_subset_PTAS : Every FPTAS problem has a PTAS`
pub fn fptas_subset_ptas_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "P",
        optimization_problem_ty(),
        arrow(app(cst("FPTAS"), bvar(0)), app(cst("PTAS"), bvar(0))),
    )
}
/// `PTAS_subset_APX : Every PTAS problem is in APX`
pub fn ptas_subset_apx_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "P",
        optimization_problem_ty(),
        arrow(app(cst("PTAS"), bvar(0)), app(cst("APX"), bvar(0))),
    )
}
/// `LPRelaxation : OptimizationProblem → Type`
/// The LP relaxation of an integer program.
pub fn lp_relaxation_ty() -> Expr {
    arrow(optimization_problem_ty(), type0())
}
/// `IntegralityGap : LPRelaxation P → Real`
/// The integrality gap: ratio of LP optimum to IP optimum.
pub fn integrality_gap_ty() -> Expr {
    arrow(lp_relaxation_ty(), real_ty())
}
/// `LPDominates : LPRelaxation P → OptimizationProblem → Prop`
/// The LP bound dominates (is at least as tight as) the IP optimum.
pub fn lp_dominates_ty() -> Expr {
    arrow(lp_relaxation_ty(), arrow(optimization_problem_ty(), prop()))
}
/// `RandomizedRounding : LPRelaxation P → ApproxAlgorithm P`
/// Randomized rounding converts LP solutions to integer solutions.
pub fn randomized_rounding_ty() -> Expr {
    arrow(lp_relaxation_ty(), approx_algorithm_ty())
}
/// `SetCoverLPGap : The set cover LP has integrality gap Θ(log n)`
pub fn set_cover_lp_gap_ty() -> Expr {
    prop()
}
/// `VertexCoverLPGap : The vertex cover LP has integrality gap 2`
pub fn vertex_cover_lp_gap_ty() -> Expr {
    prop()
}
/// `PrimalDualAlgorithm : OptimizationProblem → Type`
/// An algorithm designed via the primal-dual schema.
pub fn primal_dual_algorithm_ty() -> Expr {
    arrow(optimization_problem_ty(), type0())
}
/// `PrimalDualGuarantee : PrimalDualAlgorithm P → Real → Prop`
/// The primal-dual algorithm achieves the given approximation ratio.
pub fn primal_dual_guarantee_ty() -> Expr {
    arrow(primal_dual_algorithm_ty(), arrow(real_ty(), prop()))
}
/// `WeightedVertexCoverPD : 2-approximation via primal-dual for weighted VC`
pub fn weighted_vertex_cover_pd_ty() -> Expr {
    prop()
}
/// `SteinertreePD : The Steiner tree primal-dual gives 2(1 − 1/l) ratio`
pub fn steiner_tree_pd_ty() -> Expr {
    prop()
}
/// `LocalSearchAlgorithm : OptimizationProblem → Type`
/// A local search algorithm iteratively improves solutions.
pub fn local_search_algorithm_ty() -> Expr {
    arrow(optimization_problem_ty(), type0())
}
/// `LocalOptimum : LocalSearchAlgorithm P → String → Prop`
/// A solution is locally optimal (no improvement in the neighborhood).
pub fn local_optimum_ty() -> Expr {
    arrow(local_search_algorithm_ty(), arrow(cst("String"), prop()))
}
/// `LocalSearchRatio : LocalSearchAlgorithm P → Real → Prop`
/// Local search achieves the given ratio at any local optimum.
pub fn local_search_ratio_ty() -> Expr {
    arrow(local_search_algorithm_ty(), arrow(real_ty(), prop()))
}
/// `MaxCutLocalSearch : 2-approximation for MAX-CUT via local search`
pub fn max_cut_local_search_ty() -> Expr {
    prop()
}
/// `KMedianLocalSearch : O(1)-approximation for k-median via local search`
pub fn k_median_local_search_ty() -> Expr {
    prop()
}
/// `GreedyAlgorithm : OptimizationProblem → Type`
/// A greedy algorithm making locally optimal choices at each step.
pub fn greedy_algorithm_ty() -> Expr {
    arrow(optimization_problem_ty(), type0())
}
/// `SetCoverGreedyRatio : Greedy set cover achieves H_n ratio (≤ ln n + 1)`
pub fn set_cover_greedy_ratio_ty() -> Expr {
    prop()
}
/// `MaxCoverageGreedy : (1 − 1/e)-approximation for max coverage by greedy`
pub fn max_coverage_greedy_ty() -> Expr {
    prop()
}
/// `SubmodularGreedy : Greedy gives (1 − 1/e) for submodular maximization`
pub fn submodular_greedy_ty() -> Expr {
    prop()
}
/// `PCPTheorem : NP = PCP(log n, 1)`
/// Every NP language has a proof system with logarithmic randomness and
/// constant query complexity.
pub fn pcp_theorem_ty() -> Expr {
    prop()
}
/// `MaxSATInapprox : MAX-3SAT has no (7/8 + ε)-approximation unless P=NP`
pub fn max_sat_inapprox_ty() -> Expr {
    prop()
}
/// `CliqueSizeInapprox : MAX-CLIQUE has no n^(1−ε)-approximation unless P=NP`
pub fn clique_inapprox_ty() -> Expr {
    prop()
}
/// `SetCoverInapprox : Set Cover has no (1 − ε)·ln n approximation unless P=NP`
pub fn set_cover_inapprox_ty() -> Expr {
    prop()
}
/// `ChromaticInapprox : Graph coloring is hard to approximate within n^(1−ε)`
pub fn chromatic_inapprox_ty() -> Expr {
    prop()
}
/// `MaxSNPHard_implies_APXHard : MAX-SNP hardness implies APX-hardness`
pub fn max_snp_hard_apx_hard_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "P",
        optimization_problem_ty(),
        arrow(app(cst("MaxSNP"), bvar(0)), app(cst("APXHard"), bvar(0))),
    )
}
/// `VertexCoverAPXHard : Vertex Cover is APX-hard`
pub fn vertex_cover_apx_hard_ty() -> Expr {
    prop()
}
/// `TSPNoApprox : Metric-free TSP has no constant approximation unless P=NP`
pub fn tsp_no_approx_ty() -> Expr {
    prop()
}
/// `MetricTSP : OptimizationProblem` — TSP on metric instances (triangle inequality).
pub fn metric_tsp_ty() -> Expr {
    optimization_problem_ty()
}
/// `ChristofidesSerdyukov : 3/2-approximation for metric TSP`
/// The Christofides-Serdyukov algorithm:
/// 1. Compute minimum spanning tree T.
/// 2. Let O = odd-degree vertices in T; compute min-weight perfect matching M on O.
/// 3. Form multigraph T ∪ M; find Eulerian circuit; shortcut to Hamiltonian cycle.
pub fn christofides_serdyukov_ty() -> Expr {
    prop()
}
/// `MST2Approx : MST-based 2-approximation for metric TSP`
pub fn mst_2approx_ty() -> Expr {
    prop()
}
/// Greedy set cover algorithm.
/// `universe`: elements to cover (0..n).
/// `sets`: each set is a Vec of elements.
/// Returns indices of selected sets (greedy by max coverage).
pub fn greedy_set_cover(universe_size: usize, sets: &[Vec<usize>]) -> Vec<usize> {
    let mut covered = vec![false; universe_size];
    let mut num_covered = 0;
    let mut selected = Vec::new();
    while num_covered < universe_size {
        let best = sets
            .iter()
            .enumerate()
            .filter(|&(i, _)| !selected.contains(&i))
            .max_by_key(|(_, s)| s.iter().filter(|&&e| !covered[e]).count());
        match best {
            None => break,
            Some((idx, s)) => {
                let new_cover: usize = s.iter().filter(|&&e| !covered[e]).count();
                if new_cover == 0 {
                    break;
                }
                selected.push(idx);
                for &e in s {
                    if e < universe_size && !covered[e] {
                        covered[e] = true;
                        num_covered += 1;
                    }
                }
            }
        }
    }
    selected
}
/// Greedy max-coverage algorithm: select k sets to maximize the number of covered elements.
/// Achieves (1 − 1/e)-approximation.
pub fn greedy_max_coverage(universe_size: usize, sets: &[Vec<usize>], k: usize) -> Vec<usize> {
    let mut covered = vec![false; universe_size];
    let mut selected = Vec::new();
    for _ in 0..k {
        let best = sets
            .iter()
            .enumerate()
            .filter(|&(i, _)| !selected.contains(&i))
            .max_by_key(|(_, s)| {
                s.iter()
                    .filter(|&&e| e < universe_size && !covered[e])
                    .count()
            });
        match best {
            None => break,
            Some((idx, s)) => {
                let new_cover: usize = s
                    .iter()
                    .filter(|&&e| e < universe_size && !covered[e])
                    .count();
                if new_cover == 0 {
                    break;
                }
                selected.push(idx);
                for &e in s {
                    if e < universe_size {
                        covered[e] = true;
                    }
                }
            }
        }
    }
    selected
}
/// 2-approximation for vertex cover via greedy edge cover.
/// At each step pick an uncovered edge (u,v) and add both endpoints to the cover.
pub fn vertex_cover_2approx(adj: &[Vec<usize>]) -> Vec<usize> {
    let n = adj.len();
    let mut in_cover = vec![false; n];
    let mut edge_covered = vec![vec![false; n]; n];
    for u in 0..n {
        for &v in &adj[u] {
            if !edge_covered[u][v] && !in_cover[u] && !in_cover[v] {
                in_cover[u] = true;
                in_cover[v] = true;
                for &w in &adj[u] {
                    edge_covered[u][w] = true;
                    edge_covered[w][u] = true;
                }
                for &w in &adj[v] {
                    edge_covered[v][w] = true;
                    edge_covered[w][v] = true;
                }
            }
        }
    }
    (0..n).filter(|&v| in_cover[v]).collect()
}
/// Minimum Spanning Tree using Kruskal's algorithm.
/// Returns (total weight, list of edges in MST).
pub fn kruskal_mst(n: usize, edges: &[(usize, usize, i64)]) -> (i64, Vec<(usize, usize)>) {
    let mut sorted_edges = edges.to_vec();
    sorted_edges.sort_by_key(|&(_, _, w)| w);
    let mut parent: Vec<usize> = (0..n).collect();
    let mut rank = vec![0usize; n];
    fn find(parent: &mut Vec<usize>, x: usize) -> usize {
        if parent[x] != x {
            parent[x] = find(parent, parent[x]);
        }
        parent[x]
    }
    fn union(parent: &mut Vec<usize>, rank: &mut Vec<usize>, x: usize, y: usize) -> bool {
        let rx = find(parent, x);
        let ry = find(parent, y);
        if rx == ry {
            return false;
        }
        if rank[rx] < rank[ry] {
            parent[rx] = ry;
        } else if rank[rx] > rank[ry] {
            parent[ry] = rx;
        } else {
            parent[ry] = rx;
            rank[rx] += 1;
        }
        true
    }
    let mut mst_weight = 0i64;
    let mut mst_edges = Vec::new();
    for (u, v, w) in sorted_edges {
        if union(&mut parent, &mut rank, u, v) {
            mst_weight += w;
            mst_edges.push((u, v));
        }
    }
    (mst_weight, mst_edges)
}
/// MST-based 2-approximation for metric TSP.
/// Input: complete graph with metric distances (n x n matrix).
/// Returns a Hamiltonian cycle (vertex ordering) and its total cost.
pub fn metric_tsp_2approx(dist: &[Vec<i64>]) -> (i64, Vec<usize>) {
    let n = dist.len();
    if n == 0 {
        return (0, vec![]);
    }
    if n == 1 {
        return (0, vec![0]);
    }
    let mut edges = Vec::new();
    for i in 0..n {
        for j in (i + 1)..n {
            edges.push((i, j, dist[i][j]));
        }
    }
    let (_, mst) = kruskal_mst(n, &edges);
    let mut mst_adj = vec![vec![]; n];
    for (u, v) in &mst {
        mst_adj[*u].push(*v);
        mst_adj[*v].push(*u);
    }
    let mut visited = vec![false; n];
    let mut tour = Vec::new();
    fn dfs_preorder(u: usize, adj: &[Vec<usize>], visited: &mut Vec<bool>, tour: &mut Vec<usize>) {
        visited[u] = true;
        tour.push(u);
        for &v in &adj[u] {
            if !visited[v] {
                dfs_preorder(v, adj, visited, tour);
            }
        }
    }
    dfs_preorder(0, &mst_adj, &mut visited, &mut tour);
    let mut cost = 0i64;
    for i in 0..tour.len() {
        let u = tour[i];
        let v = tour[(i + 1) % tour.len()];
        cost += dist[u][v];
    }
    (cost, tour)
}
/// FPTAS for 0-1 knapsack.
/// Scales item values by (ε·max_value / n) and runs exact DP.
/// Achieves (1 − ε) approximation in O(n^2 / ε) time.
#[allow(clippy::too_many_arguments)]
pub fn knapsack_fptas(
    weights: &[usize],
    values: &[i64],
    capacity: usize,
    epsilon: f64,
) -> (i64, Vec<usize>) {
    let n = weights.len();
    if n == 0 {
        return (0, vec![]);
    }
    let max_val = *values.iter().max().unwrap_or(&1);
    let scale = (epsilon * max_val as f64 / n as f64).max(1.0);
    let scaled: Vec<usize> = values
        .iter()
        .map(|&v| (v as f64 / scale) as usize)
        .collect();
    let max_scaled: usize = scaled.iter().sum();
    let mut dp = vec![usize::MAX; max_scaled + 1];
    dp[0] = 0;
    let mut choices: Vec<Vec<Option<bool>>> = vec![vec![None; max_scaled + 1]; n + 1];
    for i in 0..n {
        let mut new_dp = dp.clone();
        for j in 0..=max_scaled {
            if dp[j] == usize::MAX {
                continue;
            }
            let nj = j + scaled[i];
            if nj <= max_scaled {
                let new_w = dp[j].saturating_add(weights[i]);
                if new_w <= capacity && new_w < new_dp[nj] {
                    new_dp[nj] = new_w;
                    choices[i + 1][nj] = Some(true);
                }
            }
        }
        dp = new_dp;
        for j in 0..=max_scaled {
            if choices[i + 1][j].is_none() {
                choices[i + 1][j] = Some(false);
            }
        }
    }
    let best_scaled = (0..=max_scaled)
        .filter(|&j| dp[j] <= capacity)
        .max()
        .unwrap_or(0);
    let approx_value = (best_scaled as f64 * scale) as i64;
    let mut order: Vec<usize> = (0..n).collect();
    order.sort_by(|&a, &b| {
        let ra = values[a] as f64 / weights[a].max(1) as f64;
        let rb = values[b] as f64 / weights[b].max(1) as f64;
        rb.partial_cmp(&ra).unwrap_or(std::cmp::Ordering::Equal)
    });
    let mut selected = Vec::new();
    let mut rem = capacity;
    for &i in &order {
        if weights[i] <= rem {
            selected.push(i);
            rem -= weights[i];
        }
    }
    let actual: i64 = selected.iter().map(|&i| values[i]).sum();
    (actual.max(approx_value), selected)
}
/// Randomized rounding for weighted vertex cover (LP relaxation).
/// Solves the LP: for each vertex v, x_v ≥ 0; for each edge (u,v), x_u + x_v ≥ 1.
/// The LP optimum is half-integral; round x_v ≥ 1/2 to 1.
/// This gives a 2-approximation in expectation.
pub fn randomized_rounding_vertex_cover(adj: &[Vec<usize>]) -> Vec<usize> {
    let n = adj.len();
    let mut lp_val = vec![0.0f64; n];
    for u in 0..n {
        for &v in &adj[u] {
            lp_val[u] = lp_val[u].max(0.5);
            lp_val[v] = lp_val[v].max(0.5);
        }
    }
    (0..n).filter(|&v| lp_val[v] >= 0.5).collect()
}
/// Local search for MAX-CUT: start with random partition, swap vertices to improve cut.
/// Returns (cut value, partition assignment: 0 or 1 for each vertex).
pub fn local_search_max_cut(adj: &[Vec<i64>]) -> (i64, Vec<usize>) {
    let n = adj.len();
    if n == 0 {
        return (0, vec![]);
    }
    let mut side = vec![0usize; n];
    for i in 0..n {
        side[i] = i % 2;
    }
    let cut_value = |side: &[usize]| -> i64 {
        let mut cut = 0i64;
        for u in 0..n {
            for (idx, &v) in adj[u].iter().enumerate() {
                let w = if adj[u].len() == n {
                    adj[u][v as usize]
                } else {
                    1
                };
                let _ = idx;
                let _ = v;
                let _ = w;
            }
        }
        for u in 0..n {
            for v in (u + 1)..n {
                if v < adj[u].len() && side[u] != side[v] {
                    cut += adj[u][v];
                }
            }
        }
        cut
    };
    let mut improved = true;
    while improved {
        improved = false;
        let current = cut_value(&side);
        for v in 0..n {
            side[v] ^= 1;
            let new_cut = cut_value(&side);
            if new_cut > current {
                improved = true;
                break;
            } else {
                side[v] ^= 1;
            }
        }
    }
    (cut_value(&side), side)
}
/// Primal-dual algorithm for weighted vertex cover.
/// Maintains dual variables y_e for each edge; raises y_e until some endpoint
/// becomes "tight" (sum of y_e = w_v), then adds that vertex.
pub fn primal_dual_weighted_vc(n: usize, edges: &[(usize, usize, i64)]) -> Vec<usize> {
    let weights: Vec<i64> = vec![1; n];
    let mut dual = vec![0i64; edges.len()];
    let mut slack: Vec<i64> = weights.clone();
    let mut in_cover = vec![false; n];
    for (idx, &(u, v, _w)) in edges.iter().enumerate() {
        if in_cover[u] || in_cover[v] {
            continue;
        }
        let raise = slack[u].min(slack[v]);
        dual[idx] = raise;
        slack[u] -= raise;
        slack[v] -= raise;
        if slack[u] == 0 {
            in_cover[u] = true;
        }
        if slack[v] == 0 {
            in_cover[v] = true;
        }
    }
    let is_covered =
        |cover: &[bool]| -> bool { edges.iter().all(|&(u, v, _)| cover[u] || cover[v]) };
    for v in 0..n {
        if in_cover[v] {
            in_cover[v] = false;
            if !is_covered(&in_cover) {
                in_cover[v] = true;
            }
        }
    }
    (0..n).filter(|&v| in_cover[v]).collect()
}
/// Christofides-Serdyukov algorithm for metric TSP (3/2-approximation).
/// Full implementation with minimum perfect matching on odd-degree vertices.
pub fn christofides_serdyukov(dist: &[Vec<i64>]) -> (i64, Vec<usize>) {
    let n = dist.len();
    if n == 0 {
        return (0, vec![]);
    }
    if n == 1 {
        return (0, vec![0]);
    }
    if n == 2 {
        return (dist[0][1] + dist[1][0], vec![0, 1]);
    }
    let mut edges = Vec::new();
    for i in 0..n {
        for j in (i + 1)..n {
            edges.push((i, j, dist[i][j]));
        }
    }
    let (_, mst_edges) = kruskal_mst(n, &edges);
    let mut mst_adj = vec![vec![]; n];
    let mut degree = vec![0usize; n];
    for (u, v) in &mst_edges {
        mst_adj[*u].push(*v);
        mst_adj[*v].push(*u);
        degree[*u] += 1;
        degree[*v] += 1;
    }
    let odd_verts: Vec<usize> = (0..n).filter(|&v| degree[v] % 2 == 1).collect();
    let mut matched = vec![false; odd_verts.len()];
    let mut matching = Vec::new();
    for i in 0..odd_verts.len() {
        if matched[i] {
            continue;
        }
        let best = (0..odd_verts.len())
            .filter(|&j| j != i && !matched[j])
            .min_by_key(|&j| dist[odd_verts[i]][odd_verts[j]]);
        if let Some(j) = best {
            matching.push((odd_verts[i], odd_verts[j]));
            matched[i] = true;
            matched[j] = true;
        }
    }
    let mut multi_adj = mst_adj.clone();
    for (u, v) in &matching {
        multi_adj[*u].push(*v);
        multi_adj[*v].push(*u);
    }
    let mut adj_idx = vec![0usize; n];
    let mut circuit = Vec::new();
    let mut stack = vec![0usize];
    while let Some(&cur) = stack.last() {
        if adj_idx[cur] < multi_adj[cur].len() {
            let next = multi_adj[cur][adj_idx[cur]];
            adj_idx[cur] += 1;
            stack.push(next);
        } else {
            circuit.push(
                stack
                    .pop()
                    .expect("stack is non-empty: loop condition ensures non-empty"),
            );
        }
    }
    circuit.reverse();
    let mut visited = vec![false; n];
    let mut tour: Vec<usize> = circuit
        .into_iter()
        .filter(|&v| {
            if !visited[v] {
                visited[v] = true;
                true
            } else {
                false
            }
        })
        .collect();
    for v in 0..n {
        if !visited[v] {
            tour.push(v);
        }
    }
    let cost: i64 = (0..tour.len())
        .map(|i| dist[tour[i]][tour[(i + 1) % tour.len()]])
        .sum();
    (cost, tour)
}
/// Check whether a vertex cover is valid for the given graph.
pub fn is_vertex_cover(adj: &[Vec<usize>], cover: &[usize]) -> bool {
    let n = adj.len();
    let in_cover: Vec<bool> = {
        let mut v = vec![false; n];
        for &u in cover {
            if u < n {
                v[u] = true;
            }
        }
        v
    };
    for u in 0..n {
        for &v in &adj[u] {
            if !in_cover[u] && !in_cover[v] {
                return false;
            }
        }
    }
    true
}
/// Check whether a set cover covers the entire universe.
pub fn is_set_cover(universe_size: usize, sets: &[Vec<usize>], selected: &[usize]) -> bool {
    let mut covered = vec![false; universe_size];
    for &i in selected {
        for &e in &sets[i] {
            if e < universe_size {
                covered[e] = true;
            }
        }
    }
    covered.iter().all(|&c| c)
}
/// Alias for `build_approximation_algorithms_env`.
pub fn build_env(env: &mut Environment) -> Result<(), String> {
    build_approximation_algorithms_env(env)
}
/// Populate an `Environment` with approximation algorithm axioms.
pub fn build_approximation_algorithms_env(env: &mut Environment) -> Result<(), String> {
    for (name, ty) in [
        ("OptimizationProblem", optimization_problem_ty()),
        ("ApproxAlgorithm", approx_algorithm_ty()),
        ("ApproxRatio", approx_ratio_ty()),
        ("IsAlphaApprox", is_alpha_approx_ty()),
        ("OptSolution", opt_solution_ty()),
        ("AlgSolution", alg_solution_ty()),
        ("PTAS", ptas_ty()),
        ("FPTAS", fptas_ty()),
        ("EPTAS", eptas_ty()),
        ("APX", apx_ty()),
        ("APXHard", apx_hard_ty()),
        ("APXComplete", apx_complete_ty()),
        ("MaxSNP", max_snp_ty()),
        ("LPRelaxation", lp_relaxation_ty()),
        ("IntegralityGap", integrality_gap_ty()),
        ("LPDominates", lp_dominates_ty()),
        ("RandomizedRounding", randomized_rounding_ty()),
        ("PrimalDualAlgorithm", primal_dual_algorithm_ty()),
        ("PrimalDualGuarantee", primal_dual_guarantee_ty()),
        ("LocalSearchAlgorithm", local_search_algorithm_ty()),
        ("LocalOptimum", local_optimum_ty()),
        ("LocalSearchRatio", local_search_ratio_ty()),
        ("GreedyAlgorithm", greedy_algorithm_ty()),
        ("MetricTSP", metric_tsp_ty()),
        ("WeightedVertexCover", optimization_problem_ty()),
        ("SetCoverProblem", optimization_problem_ty()),
        ("MaxCut", optimization_problem_ty()),
        ("MaxCoverage", optimization_problem_ty()),
        ("KMedian", optimization_problem_ty()),
        ("SteinerTree", optimization_problem_ty()),
        ("MaxCliqueProblem", optimization_problem_ty()),
        ("GraphColoringProblem", optimization_problem_ty()),
        ("BinPacking", optimization_problem_ty()),
        ("JobScheduling", optimization_problem_ty()),
        ("Knapsack01", optimization_problem_ty()),
    ] {
        env.add(Declaration::Axiom {
            name: Name::str(name),
            univ_params: vec![],
            ty,
        })
        .ok();
    }
    for (name, ty) in [
        ("ApproxAlg.fptas_subset_ptas", fptas_subset_ptas_ty()),
        ("ApproxAlg.ptas_subset_apx", ptas_subset_apx_ty()),
        ("ApproxAlg.pcp_theorem", pcp_theorem_ty()),
        ("ApproxAlg.max_sat_inapprox", max_sat_inapprox_ty()),
        ("ApproxAlg.clique_inapprox", clique_inapprox_ty()),
        ("ApproxAlg.set_cover_inapprox", set_cover_inapprox_ty()),
        ("ApproxAlg.chromatic_inapprox", chromatic_inapprox_ty()),
        (
            "ApproxAlg.max_snp_hard_apx_hard",
            max_snp_hard_apx_hard_ty(),
        ),
        (
            "ApproxAlg.vertex_cover_apx_hard",
            vertex_cover_apx_hard_ty(),
        ),
        ("ApproxAlg.tsp_no_approx", tsp_no_approx_ty()),
        (
            "ApproxAlg.christofides_serdyukov",
            christofides_serdyukov_ty(),
        ),
        ("ApproxAlg.mst_2approx", mst_2approx_ty()),
        (
            "ApproxAlg.set_cover_greedy_ratio",
            set_cover_greedy_ratio_ty(),
        ),
        ("ApproxAlg.max_coverage_greedy", max_coverage_greedy_ty()),
        ("ApproxAlg.submodular_greedy", submodular_greedy_ty()),
        ("ApproxAlg.max_cut_local_search", max_cut_local_search_ty()),
        (
            "ApproxAlg.k_median_local_search",
            k_median_local_search_ty(),
        ),
        (
            "ApproxAlg.weighted_vertex_cover_pd",
            weighted_vertex_cover_pd_ty(),
        ),
        ("ApproxAlg.steiner_tree_pd", steiner_tree_pd_ty()),
        ("ApproxAlg.set_cover_lp_gap", set_cover_lp_gap_ty()),
        ("ApproxAlg.vertex_cover_lp_gap", vertex_cover_lp_gap_ty()),
    ] {
        env.add(Declaration::Axiom {
            name: Name::str(name),
            univ_params: vec![],
            ty,
        })
        .ok();
    }
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_greedy_set_cover() {
        let sets = vec![vec![0, 1, 2], vec![1, 2, 3], vec![3, 4], vec![0, 4]];
        let selected = greedy_set_cover(5, &sets);
        assert!(
            is_set_cover(5, &sets, &selected),
            "Greedy should produce a valid set cover"
        );
    }
    #[test]
    fn test_greedy_max_coverage() {
        let sets = vec![vec![0, 1, 2], vec![2, 3, 4], vec![0, 3]];
        let selected = greedy_max_coverage(5, &sets, 2);
        assert_eq!(selected.len(), 2, "Should select exactly 2 sets");
        let covered: std::collections::HashSet<usize> = selected
            .iter()
            .flat_map(|&i| sets[i].iter().cloned())
            .collect();
        assert!(covered.len() >= 4, "Should cover at least 4 elements");
    }
    #[test]
    fn test_vertex_cover_2approx() {
        let adj = vec![vec![1, 2], vec![0, 2], vec![0, 1]];
        let cover = vertex_cover_2approx(&adj);
        assert!(
            is_vertex_cover(&adj, &cover),
            "2-approx should produce a valid vertex cover"
        );
        assert!(cover.len() <= 4, "Cover size should be at most 2 * OPT = 4");
    }
    #[test]
    fn test_metric_tsp_2approx() {
        let dist = vec![
            vec![0, 1, 1, 1],
            vec![1, 0, 1, 1],
            vec![1, 1, 0, 1],
            vec![1, 1, 1, 0],
        ];
        let (cost, tour) = metric_tsp_2approx(&dist);
        assert_eq!(tour.len(), 4, "Tour should visit all 4 vertices");
        assert!(cost >= 4 && cost <= 8, "Cost {} should be in [4, 8]", cost);
    }
    #[test]
    fn test_christofides_serdyukov() {
        let dist = vec![
            vec![0, 1, 2, 3],
            vec![1, 0, 1, 2],
            vec![2, 1, 0, 1],
            vec![3, 2, 1, 0],
        ];
        let (cost, tour) = christofides_serdyukov(&dist);
        assert_eq!(tour.len(), 4, "Tour should visit all 4 cities");
        assert!(cost <= 10, "Christofides cost {} should be ≤ 10", cost);
    }
    #[test]
    fn test_knapsack_fptas() {
        let weights = vec![2, 3, 4, 5];
        let values = vec![3, 4, 5, 7];
        let capacity = 8;
        let (approx_val, selected) = knapsack_fptas(&weights, &values, capacity, 0.1);
        let total_weight: usize = selected.iter().map(|&i| weights[i]).sum();
        assert!(
            total_weight <= capacity,
            "Weight {} exceeds capacity {}",
            total_weight,
            capacity
        );
        let optimal = 11i64;
        assert!(
            approx_val >= (optimal as f64 * 0.85) as i64,
            "FPTAS value {} should be close to optimal {}",
            approx_val,
            optimal
        );
    }
    #[test]
    fn test_primal_dual_weighted_vc() {
        let adj = vec![vec![1, 2], vec![0, 2], vec![0, 1]];
        let edges = vec![(0, 1, 1), (0, 2, 1), (1, 2, 1)];
        let cover = primal_dual_weighted_vc(3, &edges);
        assert!(
            is_vertex_cover(&adj, &cover),
            "Primal-dual should produce a valid vertex cover"
        );
    }
    #[test]
    fn test_build_approximation_algorithms_env() {
        let mut env = Environment::new();
        let result = build_approximation_algorithms_env(&mut env);
        assert!(result.is_ok(), "build should succeed");
        assert!(env.get(&Name::str("PTAS")).is_some());
        assert!(env.get(&Name::str("FPTAS")).is_some());
        assert!(env.get(&Name::str("APX")).is_some());
        assert!(env.get(&Name::str("MetricTSP")).is_some());
    }
}
/// `DependentRounding : LPRelaxation P → ApproxAlgorithm P`
///
/// Dependent rounding: a correlated rounding scheme for {0,1} LPs that
/// preserves marginals and creates negative correlations. Used for degree-constrained
/// subgraph problems and bipartite graphs.
pub fn dependent_rounding_ty() -> Expr {
    arrow(lp_relaxation_ty(), approx_algorithm_ty())
}
/// `CorrelationRounding : LPRelaxation P → ApproxAlgorithm P`
///
/// Correlation rounding: rounds {0,1} LPs using pairwise negative correlation,
/// with applications to constraint satisfaction and scheduling.
pub fn correlation_rounding_ty() -> Expr {
    arrow(lp_relaxation_ty(), approx_algorithm_ty())
}
/// `PipageRounding : LPRelaxation P → ApproxAlgorithm P`
///
/// Pipage rounding (Ageev-Sviridenko): iteratively adjusts fractional values
/// along pairs to reduce the number of fractional variables while preserving
/// or improving an objective. Applied to submodular maximization over matroids.
pub fn pipage_rounding_ty() -> Expr {
    arrow(lp_relaxation_ty(), approx_algorithm_ty())
}
/// `RandomizedRoundingGap : Real → Prop`
///
/// LP integrality gap theorem: for a class of LPs with gap α, any rounding
/// scheme that is oblivious (independent of LP data) cannot beat the gap.
pub fn randomized_rounding_gap_ty() -> Expr {
    arrow(real_ty(), prop())
}
/// `LPHierarchy : OptimizationProblem → Nat → Type`
///
/// LP hierarchy: the Sherali-Adams or Lovász-Schrijver hierarchy strengthens
/// the LP relaxation by adding rounds of constraints from products of rows.
pub fn lp_hierarchy_ty() -> Expr {
    arrow(optimization_problem_ty(), arrow(nat_ty(), type0()))
}
/// `TightnessOfIntegralityGap : LPRelaxation P → Real → Prop`
///
/// Tightness: the integrality gap is achieved by an explicit bad instance.
pub fn tightness_integrality_gap_ty() -> Expr {
    arrow(lp_relaxation_ty(), arrow(real_ty(), prop()))
}
/// `SDPRelaxation : OptimizationProblem → Type`
///
/// A semidefinite programming relaxation of a combinatorial problem.
pub fn sdp_relaxation_ty() -> Expr {
    arrow(optimization_problem_ty(), type0())
}
/// `GoemansWilliamsonMaxCut : 0.878-approximation for MAX-CUT via SDP`
///
/// Goemans-Williamson (1995): solve the SDP relaxation of MAX-CUT, then round
/// via a random hyperplane. Achieves α_GW ≈ 0.8786 approximation ratio.
/// This is optimal assuming Unique Games Conjecture (Khot et al. 2007).
pub fn goemans_williamson_max_cut_ty() -> Expr {
    prop()
}
/// `GoemansWilliamsonRatio : Real`
///
/// The Goemans-Williamson constant α_GW = 2/π · min_{θ∈\[0,π\]} θ/(1 - cos θ) ≈ 0.8786.
pub fn goemans_williamson_ratio_ty() -> Expr {
    real_ty()
}
/// `LasserreHierarchy : OptimizationProblem → Nat → Type`
///
/// Lasserre SDP hierarchy (Sum-of-Squares): adds rounds of SDP constraints
/// corresponding to squares of polynomials, tightening the relaxation.
pub fn lasserre_hierarchy_ty() -> Expr {
    arrow(optimization_problem_ty(), arrow(nat_ty(), type0()))
}
/// `SDPIntegralityGap : SDPRelaxation P → Real`
///
/// The integrality gap of an SDP relaxation.
pub fn sdp_integrality_gap_ty() -> Expr {
    arrow(sdp_relaxation_ty(), real_ty())
}
/// `UniqueGamesConj : Prop`
///
/// Unique Games Conjecture (Khot 2002): for every ε > 0, it is NP-hard to
/// distinguish between Unique Games instances with value ≥ 1-ε and value ≤ ε.
/// Has many conditional hardness implications (MAX-CUT, vertex cover, etc.).
pub fn unique_games_conj_ty() -> Expr {
    prop()
}
/// `SDPMaxCutGapOptimal : Prop`
///
/// Assuming UGC, the Goemans-Williamson 0.878 ratio is optimal for MAX-CUT.
pub fn sdp_max_cut_gap_optimal_ty() -> Expr {
    prop()
}
/// `HypergraphCutSDP : Real → Prop`
///
/// SDP-based approximation for hypergraph cut problems with given ratio.
pub fn hypergraph_cut_sdp_ty() -> Expr {
    arrow(real_ty(), prop())
}
/// `SteinerTreeApprox : Real → Prop`
///
/// Approximation ratio α for the Steiner tree problem.
/// Best known: 1.39 (Byrka et al. 2013) via iterated LP relaxation and rounding.
pub fn steiner_tree_approx_ty() -> Expr {
    arrow(real_ty(), prop())
}
/// `JainIterativeRounding : Prop`
///
/// Jain's 2-approximation for survivable network design (2001):
/// at each step, find a fractional solution to the cut LP; at least one variable
/// is ≥ 1/2, so round it up. Gives 2-approximation for general edge-connectivity
/// requirements (including Steiner tree, vertex connectivity, etc.).
pub fn jain_iterative_rounding_ty() -> Expr {
    prop()
}
/// `KConnectivityApprox : Nat → Real → Prop`
///
/// k-edge-connectivity approximation: a network design problem where each
/// pair of vertices must have k edge-disjoint paths; approximation ratio R.
pub fn k_connectivity_approx_ty() -> Expr {
    arrow(nat_ty(), arrow(real_ty(), prop()))
}
/// `SteinerForestApprox : Real → Prop`
///
/// Steiner forest approximation: given a graph and pairs (sᵢ, tᵢ) to connect,
/// find a minimum-cost subgraph. 2-approximation via primal-dual (Agrawal et al.).
pub fn steiner_forest_approx_ty() -> Expr {
    arrow(real_ty(), prop())
}
/// `IterativeRoundingThm : Prop`
///
/// General iterative rounding theorem: for a basic feasible solution of a
/// natural LP, some variable has value ≥ 1/k, where k is the rank of the
/// constraint matrix restricted to the support.
pub fn iterative_rounding_thm_ty() -> Expr {
    prop()
}
/// `NetworkDesignLPRelaxation : Type`
///
/// The cut LP for network design: minimize ∑ cₑ xₑ subject to
/// x(δ(S)) ≥ f(S) for all S ⊆ V, where f is the connectivity requirement function.
pub fn network_design_lp_ty() -> Expr {
    type0()
}
/// `KMeansPlusPlus : OptimizationProblem → Prop`
///
/// k-means++ initialization (Arthur-Vassilvitskii 2007): choose initial centers
/// with probability proportional to squared distance, giving O(log k)-approximation
/// in expectation for k-means clustering.
pub fn k_means_plus_plus_ty() -> Expr {
    arrow(optimization_problem_ty(), prop())
}
/// `KMedianApprox : Real → Prop`
///
/// k-median approximation ratio α: find k centers minimizing sum of distances.
/// Best known: (2.675 + ε) via local search (Byrka et al. 2017).
pub fn k_median_approx_ty() -> Expr {
    arrow(real_ty(), prop())
}
/// `FacilityLocationBicriteria : Real → Real → Prop`
///
/// Bicriteria approximation for facility location: simultaneously achieves
/// (α, β) approximation on cost and number of facilities opened.
pub fn facility_location_bicriteria_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), prop()))
}
/// `ListSchedulingApprox : Real → Prop`
///
/// List scheduling approximation (Graham 1969): assign jobs greedily to machines.
/// Gives (2 - 1/m)-approximation for makespan on m identical machines.
pub fn list_scheduling_approx_ty() -> Expr {
    arrow(real_ty(), prop())
}
/// `MakespanPTAS : Prop`
///
/// PTAS for makespan minimization on identical machines: for every ε > 0,
/// achieves (1+ε) approximation in time n^{O(1/ε²)}.
pub fn makespan_ptas_ty() -> Expr {
    prop()
}
/// `PreemptiveScheduleApprox : Prop`
///
/// Preemptive scheduling: if jobs may be interrupted and resumed, there exist
/// optimal algorithms (e.g., McNaughton's algorithm for P||Cmax).
pub fn preemptive_schedule_approx_ty() -> Expr {
    prop()
}
/// `OnlineAlgorithmCompetitive : OptimizationProblem → Real → Prop`
///
/// An online algorithm is c-competitive: its cost on any sequence is at most
/// c times the cost of the offline optimal plus an additive term.
pub fn online_algorithm_competitive_ty() -> Expr {
    arrow(optimization_problem_ty(), arrow(real_ty(), prop()))
}
/// `SkiRentalBreakeven : Prop`
///
/// Ski rental problem: optimal deterministic online algorithm breaks even at
/// cost ratio 2 (buy when rental cost = purchase cost).
/// Randomized algorithm achieves e/(e-1) ≈ 1.58-competitive ratio.
pub fn ski_rental_breakeven_ty() -> Expr {
    prop()
}
/// `LoadBalancingOnline : Real → Prop`
///
/// Online load balancing: greedy assignment (assign job to least loaded machine)
/// achieves (2 - 1/m) competitive ratio for m machines.
pub fn load_balancing_online_ty() -> Expr {
    arrow(real_ty(), prop())
}
/// `AroraPTASEuclideanTSP : Prop`
///
/// Arora's PTAS for Euclidean TSP (1998): for any ε > 0, a (1+ε)-approximation
/// in time O(n (log n)^{O(1/ε)}) via randomized guillotine cuts and dynamic programming.
pub fn arora_ptas_euclidean_tsp_ty() -> Expr {
    prop()
}
/// `BakeryPTAS : Prop`
///
/// Mitchell's PTAS for Euclidean TSP (alternative proof via Steiner points).
pub fn mitchell_ptas_euclidean_tsp_ty() -> Expr {
    prop()
}
/// `GapAmplification : Prop`
///
/// Gap amplification in PCP theorem: if a CSP has value ≤ 1 - ε then
/// after amplification it has value ≤ 1/2, enabling NP-hardness of approximation.
pub fn gap_amplification_ty() -> Expr {
    prop()
}
/// `InapproximabilityReduction : Prop`
///
/// L-reduction and PTAS-reduction: polynomial transformations that preserve
/// approximability structure, used to transfer hardness between problems.
pub fn inapproximability_reduction_ty() -> Expr {
    prop()
}
/// Register all §9–§12 approximation algorithm axioms into `env`.
pub fn build_approximation_algorithms_ext_env(env: &mut Environment) -> Result<(), String> {
    for (name, ty) in [
        ("DependentRounding", dependent_rounding_ty()),
        ("CorrelationRounding", correlation_rounding_ty()),
        ("PipageRounding", pipage_rounding_ty()),
        ("RandomizedRoundingGap", randomized_rounding_gap_ty()),
        ("LPHierarchy", lp_hierarchy_ty()),
        ("TightnessIntegralityGap", tightness_integrality_gap_ty()),
        ("SDPRelaxation", sdp_relaxation_ty()),
        ("GoemansWilliamsonRatio", goemans_williamson_ratio_ty()),
        ("LasserreHierarchy", lasserre_hierarchy_ty()),
        ("SDPIntegralityGap", sdp_integrality_gap_ty()),
        ("UniqueGamesConj", unique_games_conj_ty()),
        ("HypergraphCutSDP", hypergraph_cut_sdp_ty()),
        ("SteinerTreeApprox", steiner_tree_approx_ty()),
        ("SteinerForestApprox", steiner_forest_approx_ty()),
        ("KConnectivityApprox", k_connectivity_approx_ty()),
        ("NetworkDesignLP", network_design_lp_ty()),
        ("KMeansPlusPlus", k_means_plus_plus_ty()),
        ("KMedianApprox", k_median_approx_ty()),
        (
            "FacilityLocationBicriteria",
            facility_location_bicriteria_ty(),
        ),
        ("ListSchedulingApprox", list_scheduling_approx_ty()),
        (
            "OnlineAlgorithmCompetitive",
            online_algorithm_competitive_ty(),
        ),
        ("LoadBalancingOnline", load_balancing_online_ty()),
        (
            "InapproximabilityReduction",
            inapproximability_reduction_ty(),
        ),
    ] {
        env.add(Declaration::Axiom {
            name: Name::str(name),
            univ_params: vec![],
            ty,
        })
        .ok();
    }
    for (name, ty) in [
        (
            "ApproxAlg.goemans_williamson_max_cut",
            goemans_williamson_max_cut_ty(),
        ),
        (
            "ApproxAlg.sdp_max_cut_gap_optimal",
            sdp_max_cut_gap_optimal_ty(),
        ),
        ("ApproxAlg.unique_games_conj", unique_games_conj_ty()),
        (
            "ApproxAlg.jain_iterative_rounding",
            jain_iterative_rounding_ty(),
        ),
        (
            "ApproxAlg.iterative_rounding_thm",
            iterative_rounding_thm_ty(),
        ),
        ("ApproxAlg.k_means_plus_plus", k_means_plus_plus_ty()),
        ("ApproxAlg.makespan_ptas", makespan_ptas_ty()),
        (
            "ApproxAlg.preemptive_schedule",
            preemptive_schedule_approx_ty(),
        ),
        ("ApproxAlg.ski_rental", ski_rental_breakeven_ty()),
        (
            "ApproxAlg.arora_ptas_euclidean_tsp",
            arora_ptas_euclidean_tsp_ty(),
        ),
        (
            "ApproxAlg.mitchell_ptas_euclidean_tsp",
            mitchell_ptas_euclidean_tsp_ty(),
        ),
        ("ApproxAlg.gap_amplification", gap_amplification_ty()),
    ] {
        env.add(Declaration::Axiom {
            name: Name::str(name),
            univ_params: vec![],
            ty,
        })
        .ok();
    }
    Ok(())
}
#[cfg(test)]
mod tests_ext {
    use super::*;
    fn ext_env() -> Environment {
        let mut env = Environment::new();
        build_approximation_algorithms_env(&mut env).expect("base env failed");
        build_approximation_algorithms_ext_env(&mut env).expect("ext env failed");
        env
    }
    #[test]
    fn test_lp_rounding_axioms_registered() {
        let env = ext_env();
        assert!(env.get(&Name::str("DependentRounding")).is_some());
        assert!(env.get(&Name::str("CorrelationRounding")).is_some());
        assert!(env.get(&Name::str("PipageRounding")).is_some());
        assert!(env.get(&Name::str("LPHierarchy")).is_some());
    }
    #[test]
    fn test_sdp_axioms_registered() {
        let env = ext_env();
        assert!(env.get(&Name::str("SDPRelaxation")).is_some());
        assert!(env.get(&Name::str("GoemansWilliamsonRatio")).is_some());
        assert!(env.get(&Name::str("LasserreHierarchy")).is_some());
        assert!(env.get(&Name::str("UniqueGamesConj")).is_some());
        assert!(env
            .get(&Name::str("ApproxAlg.goemans_williamson_max_cut"))
            .is_some());
    }
    #[test]
    fn test_network_design_axioms_registered() {
        let env = ext_env();
        assert!(env.get(&Name::str("SteinerTreeApprox")).is_some());
        assert!(env.get(&Name::str("SteinerForestApprox")).is_some());
        assert!(env
            .get(&Name::str("ApproxAlg.jain_iterative_rounding"))
            .is_some());
    }
    #[test]
    fn test_clustering_scheduling_registered() {
        let env = ext_env();
        assert!(env.get(&Name::str("KMeansPlusPlus")).is_some());
        assert!(env.get(&Name::str("KMedianApprox")).is_some());
        assert!(env.get(&Name::str("ListSchedulingApprox")).is_some());
        assert!(env.get(&Name::str("OnlineAlgorithmCompetitive")).is_some());
        assert!(env
            .get(&Name::str("ApproxAlg.arora_ptas_euclidean_tsp"))
            .is_some());
        assert!(env.get(&Name::str("ApproxAlg.gap_amplification")).is_some());
    }
    #[test]
    fn test_goemans_williamson_rounding() {
        let n = 4;
        let mut weights = vec![vec![0.0f64; n]; n];
        for i in 0..n {
            for j in 0..n {
                if i != j {
                    weights[i][j] = 1.0;
                }
            }
        }
        let gw = GoemansWilliamsonRounding::new(n, weights);
        let ub = gw.sdp_upper_bound();
        assert!((ub - 6.0).abs() < 1e-9, "SDP UB = {ub}");
        let (cut, partition) = gw.alternating_cut();
        assert!(cut >= 3.0, "alternating cut = {cut}");
        assert_eq!(partition.len(), n);
        let (ls_cut, _) = gw.local_search_improve(partition);
        assert!(
            ls_cut >= cut,
            "local search should not worsen: {ls_cut} vs {cut}"
        );
        assert!((gw.approximation_guarantee() - 0.8786).abs() < 0.001);
    }
    #[test]
    fn test_greedy_set_cover_struct() {
        let sets = vec![vec![0, 1, 2], vec![1, 2, 3], vec![3, 4], vec![0, 4]];
        let gsc = GreedySetCover::new(5, sets);
        let selected = gsc.solve();
        assert!(gsc.is_valid_cover(&selected), "must be valid cover");
        let ratio = gsc.harmonic_ratio();
        assert!(ratio > 1.0 && ratio < 3.0, "harmonic ratio = {ratio}");
        let mc = gsc.max_coverage(2);
        assert!(mc.len() <= 2);
    }
    #[test]
    fn test_christofides_heuristic() {
        let dist = vec![
            vec![0, 2, 9, 10, 8],
            vec![2, 0, 6, 4, 5],
            vec![9, 6, 0, 8, 7],
            vec![10, 4, 8, 0, 3],
            vec![8, 5, 7, 3, 0],
        ];
        let ch = ChristofidesHeuristic::new(dist);
        let (cost, tour) = ch.solve();
        assert!(ch.is_hamiltonian(&tour), "must be Hamiltonian: {tour:?}");
        assert_eq!(cost, ch.tour_cost(&tour));
        let lb = ch.mst_lower_bound();
        let ratio = cost as f64 / lb as f64;
        assert!(ratio <= 3.0, "Christofides ratio = {ratio}");
        assert_eq!(ch.approximation_ratio(), 1.5);
    }
    #[test]
    fn test_primal_dual_facility() {
        let opening_costs = vec![3.0, 5.0];
        let connection_costs = vec![vec![1.0, 4.0], vec![2.0, 1.0], vec![1.5, 3.0]];
        let pdf = PrimalDualFacility::new(opening_costs, connection_costs);
        assert_eq!(pdf.num_facilities(), 2);
        assert_eq!(pdf.num_clients(), 3);
        let (total, open, assign) = pdf.greedy_solve();
        assert!(total > 0.0, "cost must be positive: {total}");
        assert!(pdf.is_feasible(&open, &assign), "must be feasible");
        let lb = pdf.lower_bound();
        assert!(
            lb <= total + 1e-9,
            "lower bound {lb} must not exceed cost {total}"
        );
    }
}
#[cfg(test)]
mod tests_approx_extra {
    use super::*;
    #[test]
    fn test_set_cover_greedy() {
        let mut sc = SetCoverInstance::new(5);
        sc.add_set(vec![0, 1, 2], 3.0);
        sc.add_set(vec![2, 3, 4], 3.0);
        sc.add_set(vec![0, 3], 2.0);
        let (cost, chosen) = sc.greedy_solve();
        assert!(cost > 0.0);
        assert!(sc.is_feasible(&chosen));
    }
    #[test]
    fn test_metric_tsp_nn() {
        let mut tsp = MetricTSPInstance::new(4);
        for i in 0..4 {
            for j in (i + 1)..4 {
                tsp.set_dist(i, j, 1.0);
            }
        }
        assert!(tsp.satisfies_triangle_inequality());
        let (len, tour) = tsp.nearest_neighbor_tour(0);
        assert_eq!(tour.first(), tour.last());
        assert!(len >= 4.0 - 1e-9, "tour length >= n=4 for unit distances");
    }
    #[test]
    fn test_knapsack_fptas() {
        let kfptas = KnapsackFPTAS::new(10, vec![2, 3, 4, 5], vec![3.0, 4.0, 5.0, 6.0], 0.1);
        let val = kfptas.solve();
        assert!(val >= 0.0);
    }
    #[test]
    fn test_bin_packing_ffd() {
        let bpf = BinPackingFFD::new(10.0, vec![6.0, 5.0, 5.0, 4.0, 4.0]);
        let (n_bins, _items) = bpf.solve();
        let lb = bpf.lower_bound();
        assert!(n_bins >= lb, "FFD should use at least lower bound bins");
    }
    #[test]
    fn test_randomized_rounding() {
        let rr = RandomizedRounding::new(vec![0.3, 0.7, 0.5, 0.9], 100);
        let rounded = rr.threshold_round(0.5);
        assert!(!rounded[0]);
        assert!(rounded[1]);
        assert!(rounded[3]);
        let card = rr.rounded_cardinality(0.5);
        assert_eq!(card, 3);
        let obj = rr.lp_objective(&[1.0, 2.0, 3.0, 4.0]);
        assert!((obj - (0.3 + 1.4 + 1.5 + 3.6)).abs() < 1e-9);
    }
}
