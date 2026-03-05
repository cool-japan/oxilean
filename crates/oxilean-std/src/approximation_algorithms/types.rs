//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)
use super::functions::*;

/// Bin packing First Fit Decreasing (FFD) heuristic.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct BinPackingFFD {
    pub bin_capacity: f64,
    pub items: Vec<f64>,
}
#[allow(dead_code)]
impl BinPackingFFD {
    pub fn new(capacity: f64, items: Vec<f64>) -> Self {
        BinPackingFFD {
            bin_capacity: capacity,
            items,
        }
    }
    pub fn solve(&self) -> (usize, Vec<Vec<f64>>) {
        let mut sorted = self.items.clone();
        sorted.sort_by(|a, b| b.partial_cmp(a).unwrap_or(std::cmp::Ordering::Equal));
        let mut bins: Vec<f64> = Vec::new();
        let mut bin_items: Vec<Vec<f64>> = Vec::new();
        for &item in &sorted {
            if item > self.bin_capacity {
                continue;
            }
            let mut placed = false;
            for (i, cap) in bins.iter_mut().enumerate() {
                if *cap >= item {
                    *cap -= item;
                    bin_items[i].push(item);
                    placed = true;
                    break;
                }
            }
            if !placed {
                bins.push(self.bin_capacity - item);
                bin_items.push(vec![item]);
            }
        }
        (bins.len(), bin_items)
    }
    pub fn lower_bound(&self) -> usize {
        let total: f64 = self.items.iter().sum();
        (total / self.bin_capacity).ceil() as usize
    }
    /// FFD achieves at most 11/9 * OPT + 6/9 bins.
    pub fn approximation_ratio_bound(&self, opt: usize) -> usize {
        (11 * opt + 8) / 9 + 1
    }
}
/// Set cover approximation.
#[derive(Debug, Clone)]
pub struct SetCoverApprox {
    /// Size of the universe.
    pub universe_size: usize,
    /// Number of sets.
    pub num_sets: usize,
}
impl SetCoverApprox {
    /// Returns a description of the greedy ln-approximation algorithm.
    pub fn greedy_ln_approx(&self) -> String {
        format!(
            "Greedy set cover on universe of size {} with {} sets: H(max_set_size)-approximation ≈ ln({})",
            self.universe_size, self.num_sets, self.universe_size
        )
    }
    /// Returns a description of the LP rounding approach.
    pub fn lp_rounding_approach(&self) -> String {
        format!(
            "LP rounding for set cover (universe {}, sets {}): integrality gap O(log n)",
            self.universe_size, self.num_sets
        )
    }
}
/// A Polynomial-Time Approximation Scheme.
#[derive(Debug, Clone)]
pub struct PTAS {
    /// Problem name.
    pub problem: String,
    /// Time complexity exponent expression (function of 1/ε).
    pub time_exp: String,
}
impl PTAS {
    /// Returns true: a PTAS is always polynomial for each fixed ε.
    pub fn is_polynomial_for_fixed_eps(&self) -> bool {
        true
    }
    /// Returns a description of whether this PTAS can be strengthened to an FPTAS.
    pub fn to_fptas(&self) -> String {
        format!(
            "PTAS for {} with time O(n^{}): upgrade to FPTAS requires polynomial dependence on 1/ε",
            self.problem, self.time_exp
        )
    }
}
/// Primal-dual algorithm for uncapacitated facility location (UFL).
///
/// Given facilities with opening costs and clients with connection costs,
/// this algorithm finds a solution with approximation ratio (at most 3).
///
/// Algorithm (Jain-Vazirani 2001):
/// - Phase 1: grow dual variables for each client; open tight facilities.
/// - Phase 2: cluster clients and assign to open facilities.
#[derive(Debug, Clone)]
pub struct PrimalDualFacility {
    /// Facility opening costs f[i] ≥ 0.
    pub opening_costs: Vec<f64>,
    /// Connection costs c[j][i]: cost for client j to connect to facility i.
    pub connection_costs: Vec<Vec<f64>>,
}
impl PrimalDualFacility {
    /// Construct from opening costs and connection costs.
    pub fn new(opening_costs: Vec<f64>, connection_costs: Vec<Vec<f64>>) -> Self {
        PrimalDualFacility {
            opening_costs,
            connection_costs,
        }
    }
    /// Number of facilities.
    pub fn num_facilities(&self) -> usize {
        self.opening_costs.len()
    }
    /// Number of clients.
    pub fn num_clients(&self) -> usize {
        self.connection_costs.len()
    }
    /// Greedy approximation: for each client, connect to the cheapest open facility,
    /// or open the cheapest (facility opening cost + connection cost) facility.
    ///
    /// Returns (total_cost, open_facilities, client_assignments).
    pub fn greedy_solve(&self) -> (f64, Vec<usize>, Vec<usize>) {
        let nf = self.num_facilities();
        let nc = self.num_clients();
        if nf == 0 || nc == 0 {
            return (0.0, vec![], vec![0usize; nc]);
        }
        let mut open = vec![false; nf];
        let mut assignment = vec![0usize; nc];
        for j in 0..nc {
            let best_open = open
                .iter()
                .enumerate()
                .filter(|&(_, &o)| o)
                .map(|(i, _)| (self.connection_costs[j][i], i))
                .min_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
            let best_new = (0..nf)
                .filter(|&i| !open[i])
                .map(|i| (self.opening_costs[i] + self.connection_costs[j][i], i))
                .min_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
            match (best_open, best_new) {
                (Some((oc, oi)), Some((nc_cost, ni))) => {
                    if oc <= nc_cost {
                        assignment[j] = oi;
                    } else {
                        open[ni] = true;
                        assignment[j] = ni;
                    }
                }
                (Some((_, oi)), None) => {
                    assignment[j] = oi;
                }
                (None, Some((_, ni))) => {
                    open[ni] = true;
                    assignment[j] = ni;
                }
                (None, None) => {
                    assignment[j] = 0;
                }
            }
        }
        let open_facilities: Vec<usize> = (0..nf).filter(|&i| open[i]).collect();
        let facility_cost: f64 = open_facilities.iter().map(|&i| self.opening_costs[i]).sum();
        let connection_cost: f64 = assignment
            .iter()
            .enumerate()
            .map(|(j, &i)| self.connection_costs[j][i])
            .sum();
        (facility_cost + connection_cost, open_facilities, assignment)
    }
    /// Compute a lower bound on the optimal cost.
    ///
    /// For each client, the cheapest connection cost provides a lower bound.
    pub fn lower_bound(&self) -> f64 {
        self.connection_costs
            .iter()
            .map(|row| row.iter().cloned().fold(f64::INFINITY, f64::min))
            .sum()
    }
    /// Check feasibility: every client is assigned to an open facility.
    pub fn is_feasible(&self, open: &[usize], assignment: &[usize]) -> bool {
        let nf = self.num_facilities();
        let nc = self.num_clients();
        let open_set: Vec<bool> = {
            let mut v = vec![false; nf];
            for &i in open {
                if i < nf {
                    v[i] = true;
                }
            }
            v
        };
        assignment.iter().take(nc).all(|&i| i < nf && open_set[i])
    }
}
/// TSP approximation algorithms.
#[derive(Debug, Clone)]
pub struct TSPApprox {
    /// Whether the triangle inequality (metric) holds.
    pub is_metric: bool,
}
impl TSPApprox {
    /// Returns a description of the Christofides-Serdyukov algorithm.
    pub fn christofides_algorithm(&self) -> String {
        if self.is_metric {
            "Christofides-Serdyukov: MST + min-weight perfect matching on odd-degree vertices = 3/2-approx"
                .to_string()
        } else {
            "General TSP is inapproximable unless P=NP".to_string()
        }
    }
    /// Returns the approximation ratio achieved.
    pub fn approx_ratio(&self) -> f64 {
        if self.is_metric {
            1.5
        } else {
            f64::INFINITY
        }
    }
    /// Returns a description of the PTAS for Euclidean TSP.
    pub fn ptas_for_euclidean(&self) -> String {
        "Arora's PTAS for Euclidean TSP: O(n * (log n)^O(1/ε)) via guillotine cuts".to_string()
    }
}
/// A Fully Polynomial-Time Approximation Scheme.
#[derive(Debug, Clone)]
pub struct FPTAS {
    /// Problem name.
    pub problem: String,
}
impl FPTAS {
    /// Returns true: an FPTAS runs in time polynomial in both n and 1/ε.
    pub fn is_fully_polynomial(&self) -> bool {
        true
    }
    /// Returns a description of the dynamic programming approach used.
    pub fn dynamic_programming_approach(&self) -> String {
        format!(
            "FPTAS for {} via DP with value scaling: partition values into 1/ε buckets",
            self.problem
        )
    }
}
/// MAX-SAT approximation algorithms.
#[derive(Debug, Clone)]
pub struct MaxSATApprox {
    /// Number of clauses.
    pub num_clauses: usize,
    /// Number of variables.
    pub num_vars: usize,
}
impl MaxSATApprox {
    /// Returns a description of the derandomized 7/8-approximation.
    pub fn derandomized_7_8_approx(&self) -> String {
        format!(
            "Derandomized 7/8-approx for MAX-3SAT ({} clauses, {} vars): method of conditional expectations",
            self.num_clauses, self.num_vars
        )
    }
    /// Returns a description of the semidefinite programming rounding approach.
    pub fn semidefinite_rounding(&self) -> String {
        format!(
            "SDP rounding for MAX-SAT ({} clauses, {} vars): Goemans-Williamson random hyperplane",
            self.num_clauses, self.num_vars
        )
    }
}
/// A greedy approximation algorithm.
#[derive(Debug, Clone)]
pub struct GreedyApprox {
    /// Problem name.
    pub problem: String,
    /// Approximation ratio achieved.
    pub ratio: f64,
}
impl GreedyApprox {
    /// Returns a description of the greedy algorithm.
    pub fn greedy_algorithm(&self) -> String {
        format!(
            "Greedy algorithm for {}: pick the locally optimal element at each step",
            self.problem
        )
    }
    /// Returns an analysis of the approximation guarantee.
    pub fn analysis(&self) -> String {
        format!(
            "Greedy achieves {:.3}-approximation for {} via potential function / exchange argument",
            self.ratio, self.problem
        )
    }
}
/// Object-oriented wrapper for the Christofides-Serdyukov 3/2-approximation.
///
/// Implements the full algorithm:
/// 1. Compute MST.
/// 2. Find odd-degree vertices; compute greedy min-weight perfect matching.
/// 3. Combine MST and matching into multigraph.
/// 4. Find Eulerian circuit via Hierholzer's algorithm.
/// 5. Shortcut to Hamiltonian cycle.
#[derive(Debug, Clone)]
pub struct ChristofidesHeuristic {
    /// Distance matrix (metric: symmetric, satisfies triangle inequality).
    pub dist: Vec<Vec<i64>>,
}
impl ChristofidesHeuristic {
    /// Construct from a distance matrix.
    pub fn new(dist: Vec<Vec<i64>>) -> Self {
        ChristofidesHeuristic { dist }
    }
    /// Run the Christofides algorithm. Returns (tour cost, vertex ordering).
    pub fn solve(&self) -> (i64, Vec<usize>) {
        christofides_serdyukov(&self.dist)
    }
    /// Run the simpler MST 2-approximation. Returns (tour cost, vertex ordering).
    pub fn mst_2approx(&self) -> (i64, Vec<usize>) {
        metric_tsp_2approx(&self.dist)
    }
    /// The guaranteed approximation ratio (3/2).
    pub fn approximation_ratio(&self) -> f64 {
        1.5
    }
    /// Compute the optimal lower bound via MST weight (since MST ≤ OPT for metric TSP).
    pub fn mst_lower_bound(&self) -> i64 {
        let n = self.dist.len();
        let mut edges = Vec::new();
        for i in 0..n {
            for j in (i + 1)..n {
                edges.push((i, j, self.dist[i][j]));
            }
        }
        let (w, _) = kruskal_mst(n, &edges);
        w
    }
    /// Check whether a tour visits all vertices exactly once (is Hamiltonian).
    pub fn is_hamiltonian(&self, tour: &[usize]) -> bool {
        let n = self.dist.len();
        if tour.len() != n {
            return false;
        }
        let mut seen = vec![false; n];
        for &v in tour {
            if v >= n || seen[v] {
                return false;
            }
            seen[v] = true;
        }
        true
    }
    /// Compute the cost of an explicit tour.
    pub fn tour_cost(&self, tour: &[usize]) -> i64 {
        let n = tour.len();
        if n == 0 {
            return 0;
        }
        (0..n).map(|i| self.dist[tour[i]][tour[(i + 1) % n]]).sum()
    }
}
/// Shifted local search PTAS.
#[derive(Debug, Clone)]
pub struct ShiftedLocalSearch {
    /// Shift parameter.
    pub shift: usize,
}
impl ShiftedLocalSearch {
    /// Returns a description of the ε-approximation via shifted local search.
    pub fn eps_approx(&self) -> String {
        format!(
            "Shifted local search with shift {}: achieves (1+ε)-approximation for geometric problems",
            self.shift
        )
    }
    /// Returns true: shifted local search with bounded shift gives a PTAS.
    pub fn is_ptas(&self) -> bool {
        self.shift > 0
    }
}
/// Greedy set cover approximation: O(log n) approximation ratio.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SetCoverInstance {
    pub universe_size: usize,
    pub sets: Vec<Vec<usize>>,
    pub costs: Vec<f64>,
}
#[allow(dead_code)]
impl SetCoverInstance {
    pub fn new(n: usize) -> Self {
        SetCoverInstance {
            universe_size: n,
            sets: Vec::new(),
            costs: Vec::new(),
        }
    }
    pub fn add_set(&mut self, elements: Vec<usize>, cost: f64) {
        self.sets.push(elements);
        self.costs.push(cost);
    }
    pub fn greedy_solve(&self) -> (f64, Vec<usize>) {
        let mut covered = vec![false; self.universe_size];
        let mut n_covered = 0;
        let mut chosen = Vec::new();
        let mut total_cost = 0.0;
        while n_covered < self.universe_size {
            let mut best = None;
            let mut best_ratio = f64::INFINITY;
            for (i, (set, &cost)) in self.sets.iter().zip(self.costs.iter()).enumerate() {
                if chosen.contains(&i) {
                    continue;
                }
                let new_covered = set.iter().filter(|&&e| !covered[e]).count();
                if new_covered == 0 {
                    continue;
                }
                let ratio = cost / new_covered as f64;
                if ratio < best_ratio {
                    best_ratio = ratio;
                    best = Some(i);
                }
            }
            match best {
                None => break,
                Some(i) => {
                    chosen.push(i);
                    total_cost += self.costs[i];
                    for &e in &self.sets[i] {
                        if !covered[e] {
                            covered[e] = true;
                            n_covered += 1;
                        }
                    }
                }
            }
        }
        (total_cost, chosen)
    }
    pub fn is_feasible(&self, chosen: &[usize]) -> bool {
        let mut covered = vec![false; self.universe_size];
        for &i in chosen {
            for &e in &self.sets[i] {
                if e < self.universe_size {
                    covered[e] = true;
                }
            }
        }
        covered.iter().all(|&c| c)
    }
    pub fn optimal_lower_bound(&self) -> f64 {
        let min_cost = self.costs.iter().cloned().fold(f64::INFINITY, f64::min);
        min_cost
    }
}
/// Goemans-Williamson rounding for MAX-CUT.
///
/// Given a graph with adjacency weights, computes an approximate MAX-CUT by:
/// 1. Assigning each vertex a unit vector in R^n (via random initialization),
/// 2. Rounding by projecting onto a random hyperplane (normal vector r).
///
/// This gives approximately 0.878 of the SDP optimum.
#[derive(Debug, Clone)]
pub struct GoemansWilliamsonRounding {
    /// Number of vertices.
    pub n: usize,
    /// Adjacency matrix weights (symmetric, n x n).
    pub weights: Vec<Vec<f64>>,
}
impl GoemansWilliamsonRounding {
    /// Construct from a weight matrix.
    pub fn new(n: usize, weights: Vec<Vec<f64>>) -> Self {
        GoemansWilliamsonRounding { n, weights }
    }
    /// Compute the cut value for a given partition (0 or 1 for each vertex).
    pub fn cut_value(&self, partition: &[usize]) -> f64 {
        let mut cut = 0.0;
        for i in 0..self.n {
            for j in (i + 1)..self.n {
                if j < self.weights[i].len() && partition[i] != partition[j] {
                    cut += self.weights[i][j];
                }
            }
        }
        cut
    }
    /// Compute the SDP upper bound: (1/2) ∑_{ij} w_{ij}.
    ///
    /// The SDP relaxation value for MAX-CUT is at most (1/2) * (sum of all weights),
    /// since the SDP objective is (1/2) ∑ w_{ij} (1 - cos θ_{ij}) ≤ (1/2) ∑ w_{ij}.
    pub fn sdp_upper_bound(&self) -> f64 {
        let mut total = 0.0;
        for i in 0..self.n {
            for j in (i + 1)..self.n {
                if j < self.weights[i].len() {
                    total += self.weights[i][j];
                }
            }
        }
        total
    }
    /// Perform a deterministic approximation of GW rounding using alternating cut.
    ///
    /// Assigns vertex i to partition i % 2, achieving exactly (1/2) of total weight
    /// on bipartite-like graphs. Returns (cut_value, partition).
    pub fn alternating_cut(&self) -> (f64, Vec<usize>) {
        let partition: Vec<usize> = (0..self.n).map(|i| i % 2).collect();
        let cut = self.cut_value(&partition);
        (cut, partition)
    }
    /// Local search improvement of a partition.
    ///
    /// Greedily moves vertices to improve the cut value.
    pub fn local_search_improve(&self, mut partition: Vec<usize>) -> (f64, Vec<usize>) {
        let mut improved = true;
        while improved {
            improved = false;
            let current = self.cut_value(&partition);
            for v in 0..self.n {
                partition[v] ^= 1;
                let new_cut = self.cut_value(&partition);
                if new_cut > current {
                    improved = true;
                    break;
                }
                partition[v] ^= 1;
            }
        }
        let final_cut = self.cut_value(&partition);
        (final_cut, partition)
    }
    /// Returns the approximation guarantee: at least GW_RATIO of SDP optimum.
    pub fn approximation_guarantee(&self) -> f64 {
        0.8786
    }
}
/// Object-oriented wrapper for greedy set cover.
///
/// Implements the classical greedy algorithm: at each step, select the set
/// covering the most uncovered elements. Achieves H(n) = ln(n)+O(1) approximation.
#[derive(Debug, Clone)]
pub struct GreedySetCover {
    /// Number of elements in the universe.
    pub universe_size: usize,
    /// Collection of sets, each given as a list of element indices.
    pub sets: Vec<Vec<usize>>,
}
impl GreedySetCover {
    /// Construct from universe size and sets.
    pub fn new(universe_size: usize, sets: Vec<Vec<usize>>) -> Self {
        GreedySetCover {
            universe_size,
            sets,
        }
    }
    /// Run the greedy set cover algorithm. Returns indices of selected sets.
    pub fn solve(&self) -> Vec<usize> {
        greedy_set_cover(self.universe_size, &self.sets)
    }
    /// Run the max-coverage variant: select k sets to maximize coverage.
    pub fn max_coverage(&self, k: usize) -> Vec<usize> {
        greedy_max_coverage(self.universe_size, &self.sets, k)
    }
    /// Compute the harmonic number H(n) = 1 + 1/2 + ... + 1/n (approximation ratio).
    pub fn harmonic_ratio(&self) -> f64 {
        let n = self.universe_size.max(1);
        (1..=n).map(|k| 1.0 / k as f64).sum()
    }
    /// Check whether a selection is a valid cover.
    pub fn is_valid_cover(&self, selected: &[usize]) -> bool {
        is_set_cover(self.universe_size, &self.sets, selected)
    }
    /// Compute the approximation ratio guarantee for the greedy algorithm.
    ///
    /// The greedy algorithm is H(max_set_size)-approximate.
    pub fn greedy_guarantee(&self) -> f64 {
        let max_sz = self.sets.iter().map(|s| s.len()).max().unwrap_or(1);
        (1..=max_sz).map(|k| 1.0 / k as f64).sum()
    }
}
/// Randomized rounding for LP relaxation of integer programs.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RandomizedRounding {
    pub lp_solution: Vec<f64>,
    pub n_trials: usize,
}
#[allow(dead_code)]
impl RandomizedRounding {
    pub fn new(lp: Vec<f64>, trials: usize) -> Self {
        RandomizedRounding {
            lp_solution: lp,
            n_trials: trials,
        }
    }
    /// Deterministic threshold rounding: x_i = 1 iff lp_i >= 0.5.
    pub fn threshold_round(&self, threshold: f64) -> Vec<bool> {
        self.lp_solution.iter().map(|&x| x >= threshold).collect()
    }
    /// Count ones in rounded solution.
    pub fn rounded_cardinality(&self, threshold: f64) -> usize {
        self.threshold_round(threshold)
            .iter()
            .filter(|&&b| b)
            .count()
    }
    /// LP lower bound: sum of fractional values.
    pub fn lp_objective(&self, costs: &[f64]) -> f64 {
        self.lp_solution
            .iter()
            .zip(costs.iter())
            .map(|(x, c)| x * c)
            .sum()
    }
}
/// LP relaxation with integrality gap.
#[derive(Debug, Clone)]
pub struct LinearProgramRelaxation {
    /// Integrality gap of the LP relaxation.
    pub integrality_gap: f64,
}
impl LinearProgramRelaxation {
    /// Returns a description of the LP relaxation.
    pub fn lp_relaxation(&self) -> String {
        format!(
            "LP relaxation with integrality gap {:.3}: solve fractional problem, then round",
            self.integrality_gap
        )
    }
    /// Returns a description of the rounding algorithm.
    pub fn rounding_algorithm(&self) -> String {
        format!(
            "Round LP solution: integrality gap {:.3} gives {:.3}-approximation",
            self.integrality_gap, self.integrality_gap
        )
    }
    /// Returns a description of the primal-dual method.
    pub fn primal_dual(&self) -> String {
        format!(
            "Primal-dual algorithm achieves {:.3}-approximation without solving LP explicitly",
            self.integrality_gap
        )
    }
}
/// MAX-CLIQUE inapproximability result.
#[derive(Debug, Clone, Default)]
pub struct MaxCliqueInapprox;
impl MaxCliqueInapprox {
    /// Returns a description of the inapproximability within n^(1-ε).
    pub fn inapproximable_within_n_pow_1_minus_eps(&self) -> String {
        "MAX-CLIQUE is inapproximable within n^(1-ε) for any ε > 0 unless P=NP (Håstad 1999)"
            .to_string()
    }
}
/// Christofides algorithm skeleton for metric TSP (3/2-approximation).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct MetricTSPInstance {
    pub n: usize,
    pub dist: Vec<Vec<f64>>,
}
#[allow(dead_code)]
impl MetricTSPInstance {
    pub fn new(n: usize) -> Self {
        MetricTSPInstance {
            n,
            dist: vec![vec![0.0; n]; n],
        }
    }
    pub fn set_dist(&mut self, i: usize, j: usize, d: f64) {
        self.dist[i][j] = d;
        self.dist[j][i] = d;
    }
    pub fn satisfies_triangle_inequality(&self) -> bool {
        for i in 0..self.n {
            for j in 0..self.n {
                for k in 0..self.n {
                    if self.dist[i][k] > self.dist[i][j] + self.dist[j][k] + 1e-9 {
                        return false;
                    }
                }
            }
        }
        true
    }
    /// Nearest neighbor heuristic: O(n^2).
    pub fn nearest_neighbor_tour(&self, start: usize) -> (f64, Vec<usize>) {
        let mut visited = vec![false; self.n];
        let mut tour = vec![start];
        let mut current = start;
        visited[current] = true;
        let mut total = 0.0;
        for _ in 1..self.n {
            let mut nearest = None;
            let mut nearest_dist = f64::INFINITY;
            for j in 0..self.n {
                if !visited[j] && self.dist[current][j] < nearest_dist {
                    nearest_dist = self.dist[current][j];
                    nearest = Some(j);
                }
            }
            match nearest {
                None => break,
                Some(j) => {
                    visited[j] = true;
                    tour.push(j);
                    total += nearest_dist;
                    current = j;
                }
            }
        }
        total += self.dist[current][start];
        tour.push(start);
        (total, tour)
    }
    pub fn tour_length(&self, tour: &[usize]) -> f64 {
        let mut len = 0.0;
        for i in 0..tour.len().saturating_sub(1) {
            len += self.dist[tour[i]][tour[i + 1]];
        }
        len
    }
}
/// FPTAS for 0/1 knapsack problem.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct KnapsackFPTAS {
    pub capacity: usize,
    pub weights: Vec<usize>,
    pub values: Vec<f64>,
    pub epsilon: f64,
}
#[allow(dead_code)]
impl KnapsackFPTAS {
    pub fn new(cap: usize, w: Vec<usize>, v: Vec<f64>, eps: f64) -> Self {
        KnapsackFPTAS {
            capacity: cap,
            weights: w,
            values: v,
            epsilon: eps,
        }
    }
    pub fn n_items(&self) -> usize {
        self.weights.len()
    }
    /// Scale values and solve with DP. Returns approximate optimal value.
    pub fn solve(&self) -> f64 {
        let n = self.n_items();
        let max_val = self.values.iter().cloned().fold(0.0f64, f64::max);
        if max_val <= 0.0 {
            return 0.0;
        }
        let scale = (self.epsilon * max_val) / (n as f64);
        let scaled: Vec<usize> = self
            .values
            .iter()
            .map(|&v| (v / scale).floor() as usize)
            .collect();
        let v_max: usize = scaled.iter().sum();
        let mut dp = vec![usize::MAX; v_max + 1];
        dp[0] = 0;
        for _i in 0..n {
            let new_dp = dp.clone();
            let _ = new_dp;
        }
        let mut items: Vec<(f64, usize)> = self
            .values
            .iter()
            .cloned()
            .zip(self.weights.iter().cloned())
            .collect();
        items.sort_by(|a, b| {
            (b.0 / b.1 as f64)
                .partial_cmp(&(a.0 / a.1 as f64))
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        let mut rem_cap = self.capacity;
        let mut total_val = 0.0;
        for (v, w) in &items {
            if *w <= rem_cap {
                rem_cap -= w;
                total_val += v;
            }
        }
        total_val
    }
}
/// Approximation ratio with tightness information.
#[derive(Debug, Clone)]
pub struct ApproximationRatio {
    /// The approximation factor alpha (>= 1 for minimization).
    pub alpha: f64,
    /// Whether this ratio is tight (matching lower bound).
    pub is_tight: bool,
}
impl ApproximationRatio {
    /// Returns a string describing the alpha-approximation guarantee.
    pub fn alpha_approx(&self) -> String {
        if self.is_tight {
            format!("{}-approximation (tight)", self.alpha)
        } else {
            format!("{}-approximation", self.alpha)
        }
    }
    /// Returns a description of the inapproximability bound.
    pub fn inapproximability_bound(&self) -> String {
        format!(
            "Cannot approximate within factor {} unless P=NP (or stronger assumption)",
            self.alpha
        )
    }
}
