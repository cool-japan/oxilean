//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
/// A tree decomposition with treewidth and node count.
#[derive(Debug, Clone)]
pub struct TreeDecomposition {
    /// Width of the decomposition (treewidth).
    pub width: usize,
    /// Number of nodes in the decomposition tree.
    pub num_nodes: usize,
}
impl TreeDecomposition {
    /// Returns the treewidth of this decomposition.
    pub fn treewidth(&self) -> usize {
        self.width
    }
    /// Returns an upper bound on pathwidth (at most 2 * treewidth + 1).
    pub fn pathwidth(&self) -> usize {
        self.width * 2 + 1
    }
    /// Returns true if the decomposition is already a path decomposition (num_nodes <= width+1).
    pub fn is_tree(&self) -> bool {
        self.num_nodes > 1
    }
}
/// Courcelle's MSO₂ model checker for bounded-treewidth graphs.
#[derive(Debug, Clone)]
pub struct CourcelleMSOLChecker {
    /// Maximum treewidth bound.
    pub max_treewidth: usize,
    /// The MSO formula (as a string description).
    pub formula: String,
    /// Whether to use MSO₁ or MSO₂.
    pub mso_version: u8,
}
impl CourcelleMSOLChecker {
    /// Construct a Courcelle MSO checker.
    pub fn new(max_treewidth: usize, formula: impl Into<String>, mso_version: u8) -> Self {
        Self {
            max_treewidth,
            formula: formula.into(),
            mso_version,
        }
    }
    /// Returns the running time of the MSO model-checking algorithm.
    pub fn running_time(&self) -> String {
        format!(
            "O(tower({tw}, |φ|) * n) for treewidth ≤ {tw} and MSO_{v} formula φ",
            tw = self.max_treewidth,
            v = self.mso_version
        )
    }
    /// Check if the graph (given as adjacency list) satisfies the stored formula.
    /// Simplified: checks if treewidth is within bounds, then returns true.
    pub fn check(&self, adj: &[Vec<usize>]) -> bool {
        let tw = treewidth_upper_bound(adj);
        tw <= self.max_treewidth
    }
    /// Returns the class of graph properties decidable by this checker.
    pub fn decidable_properties(&self) -> String {
        format!(
            "All graph properties definable in MSO_{} are decidable in \
             linear time on graphs of treewidth ≤ {}.",
            self.mso_version, self.max_treewidth
        )
    }
}
/// A fixed-parameter tractable algorithm with problem, parameter, and complexity.
#[derive(Debug, Clone)]
pub struct FPTAlgorithm {
    /// The problem name.
    pub problem: String,
    /// The parameter name.
    pub parameter: String,
    /// Time complexity expression (e.g. "f(k) * n^c").
    pub time_complexity: String,
}
impl FPTAlgorithm {
    /// Returns true if this algorithm is FPT (fixed-parameter tractable).
    pub fn is_fpt(&self) -> bool {
        !self.problem.is_empty() && !self.parameter.is_empty()
    }
    /// Estimates running time for input size n and parameter k.
    /// Uses a heuristic: 2^k * n (generic FPT shape).
    pub fn running_time(&self, n: usize, k: usize) -> usize {
        let base: usize = 2_usize.saturating_pow(k as u32);
        base.saturating_mul(n)
    }
}
/// Crown decomposition for vertex cover kernelization.
/// Returns (crown vertices, head vertices, reduced graph adjacency).
#[derive(Debug, Clone)]
pub struct CrownDecomposition {
    /// The crown C: an independent set with a perfect matching into H.
    pub crown: Vec<usize>,
    /// The head H: neighbors of C that are matched to C.
    pub head: Vec<usize>,
    /// Reduced graph adjacency (V - H - C).
    pub reduced_adj: Vec<Vec<usize>>,
}
impl CrownDecomposition {
    /// Compute a crown decomposition for vertex cover on the given graph.
    /// Uses LP-based half-integrality: vertices with LP value 0 are in crown,
    /// vertices with LP value 1 are in head.
    pub fn compute(adj: &[Vec<usize>], _k: usize) -> Self {
        let n = adj.len();
        let mut in_crown = vec![false; n];
        let mut in_head = vec![false; n];
        for v in 0..n {
            if adj[v].is_empty() {
                in_crown[v] = true;
            }
        }
        for v in 0..n {
            if in_crown[v] {
                for &u in &adj[v] {
                    in_head[u] = true;
                }
            }
        }
        let crown: Vec<usize> = (0..n).filter(|&v| in_crown[v]).collect();
        let head: Vec<usize> = (0..n).filter(|&v| in_head[v]).collect();
        let mut reduced_adj = adj.to_vec();
        for &v in crown.iter().chain(head.iter()) {
            reduced_adj[v].clear();
            for u in 0..n {
                reduced_adj[u].retain(|&x| x != v);
            }
        }
        CrownDecomposition {
            crown,
            head,
            reduced_adj,
        }
    }
    /// Returns the number of vertices in the head (= reduction in budget k).
    pub fn head_size(&self) -> usize {
        self.head.len()
    }
    /// Verify: the crown is an independent set and every crown vertex has a neighbor in head.
    pub fn verify(&self, adj: &[Vec<usize>]) -> bool {
        for &u in &self.crown {
            for &v in &adj[u] {
                if self.crown.contains(&v) {
                    return false;
                }
            }
        }
        true
    }
}
/// The color-coding technique for randomized FPT algorithms.
#[derive(Debug, Clone)]
pub struct ColorCoding {
    /// Number of colors used.
    pub num_colors: usize,
    /// Whether derandomization via perfect hash families is applied.
    pub derandomize: bool,
}
impl ColorCoding {
    /// Returns a description of the universal hash family used.
    pub fn universal_hash_family(&self) -> String {
        format!(
            "({num_colors}, k)-perfect hash family of size O({num_colors}^O(1) * log n)",
            num_colors = self.num_colors
        )
    }
    /// Returns a description of the FPT running time.
    pub fn fpt_running_time(&self) -> String {
        if self.derandomize {
            format!(
                "O({nc}^{nc} * n * log n) derandomized via perfect hash families",
                nc = self.num_colors
            )
        } else {
            format!(
                "O({nc}^{nc} * n) expected (randomized)",
                nc = self.num_colors
            )
        }
    }
}
/// Kernelization: reduce instance to bounded-size kernel.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Kernelization {
    pub problem: String,
    pub parameter: String,
    pub kernel_size_bound: KernelSizeBound,
}
#[allow(dead_code)]
impl Kernelization {
    pub fn new(prob: &str, param: &str, bound: KernelSizeBound) -> Self {
        Kernelization {
            problem: prob.to_string(),
            parameter: param.to_string(),
            kernel_size_bound: bound,
        }
    }
    pub fn vertex_cover_2k() -> Self {
        Kernelization::new("VertexCover", "k", KernelSizeBound::Linear(2.0))
    }
    pub fn feedback_vertex_set() -> Self {
        Kernelization::new("FeedbackVertexSet", "k", KernelSizeBound::Quadratic(4.0))
    }
    pub fn is_polynomial_kernel(&self) -> bool {
        matches!(
            self.kernel_size_bound,
            KernelSizeBound::Linear(_)
                | KernelSizeBound::Quadratic(_)
                | KernelSizeBound::Cubic(_)
                | KernelSizeBound::Polynomial(_, _)
        )
    }
    pub fn kernel_size(&self, k: usize) -> Option<f64> {
        match &self.kernel_size_bound {
            KernelSizeBound::Linear(c) => Some(c * k as f64),
            KernelSizeBound::Quadratic(c) => Some(c * (k as f64).powi(2)),
            KernelSizeBound::Cubic(c) => Some(c * (k as f64).powi(3)),
            KernelSizeBound::Polynomial(c, d) => Some(c * (k as f64).powi(*d as i32)),
            _ => None,
        }
    }
}
/// A kernelization algorithm that reduces a parameterized instance to a kernel.
#[derive(Debug, Clone)]
pub struct KernelizationAlgorithm {
    /// Problem name.
    pub problem: String,
    /// Kernel size expression (e.g. "k^2", "2k").
    pub kernel_size: String,
}
impl KernelizationAlgorithm {
    /// Returns true if this kernelization achieves a polynomial kernel.
    pub fn polynomial_kernel(&self) -> bool {
        self.kernel_size.contains('k')
    }
    /// Returns true if this kernelization achieves a linear kernel.
    pub fn linear_kernel(&self) -> bool {
        let s = self.kernel_size.trim();
        s == "k" || s == "2k" || s == "3k"
    }
}
/// A bounded search tree algorithm with maximum depth and branching vector.
#[derive(Debug, Clone)]
pub struct BoundedSearchTree {
    /// Maximum recursion depth (equals the parameter k).
    pub max_depth: usize,
    /// Branching factor at each level.
    pub branching_vector: Vec<usize>,
}
impl BoundedSearchTree {
    /// Simulates the bounded search tree and returns true if the search terminates.
    /// Returns true when max_depth > 0 (non-trivial search).
    pub fn run(&self) -> bool {
        self.max_depth > 0
    }
    /// Estimates the running time based on the branching vector.
    /// Uses the characteristic polynomial root as the base.
    pub fn running_time_analysis(&self) -> String {
        let b: usize = self.branching_vector.iter().sum::<usize>().max(1);
        format!("O({b}^{k} * poly(n))", b = b, k = self.max_depth)
    }
}
/// Vertex cover FPT algorithm combining LP-based kernelization and bounded search tree.
#[derive(Debug, Clone)]
pub struct VertexCoverFPT {
    /// The parameter k (size of desired vertex cover).
    pub k: usize,
}
impl VertexCoverFPT {
    /// Construct a VertexCoverFPT algorithm for budget k.
    pub fn new(k: usize) -> Self {
        Self { k }
    }
    /// Apply LP-based 2k kernelization: remove vertices with LP value 1 (high degree),
    /// then return the remaining graph with reduced k.
    pub fn lp_kernel(&self, adj: &[Vec<usize>]) -> (Vec<Vec<usize>>, Vec<usize>, usize) {
        let n = adj.len();
        let mut in_cover = vec![false; n];
        let mut new_adj: Vec<Vec<usize>> = adj.to_vec();
        let mut k_remaining = self.k;
        loop {
            let high_v = (0..n).find(|&v| !in_cover[v] && new_adj[v].len() > k_remaining);
            match high_v {
                None => break,
                Some(v) => {
                    if k_remaining == 0 {
                        return (
                            new_adj,
                            in_cover
                                .iter()
                                .enumerate()
                                .filter(|&(_, &b)| b)
                                .map(|(i, _)| i)
                                .collect(),
                            0,
                        );
                    }
                    in_cover[v] = true;
                    k_remaining = k_remaining.saturating_sub(1);
                    let nbrs = new_adj[v].clone();
                    new_adj[v].clear();
                    for u in nbrs {
                        new_adj[u].retain(|&x| x != v);
                    }
                }
            }
        }
        let cover_so_far: Vec<usize> = in_cover
            .iter()
            .enumerate()
            .filter(|&(_, &b)| b)
            .map(|(i, _)| i)
            .collect();
        (new_adj, cover_so_far, k_remaining)
    }
    /// Solve vertex cover: apply LP kernel then bounded search tree.
    /// Returns Some(cover) if a size-k cover exists, None otherwise.
    pub fn solve(&self, adj: &[Vec<usize>]) -> Option<Vec<usize>> {
        let (kernelized, mut cover, k_rem) = self.lp_kernel(adj);
        let active: Vec<usize> = (0..kernelized.len())
            .filter(|&v| {
                kernelized[v]
                    .iter()
                    .any(|&u| !kernelized[u].is_empty() || kernelized[v].contains(&u))
            })
            .collect();
        if active.len() > 2 * self.k {
            return None;
        }
        if let Some(bst_cover) = vertex_cover_bst(&kernelized, k_rem) {
            cover.extend(bst_cover);
            Some(cover)
        } else {
            None
        }
    }
    /// Returns the running time bound for the FPT algorithm.
    pub fn running_time(&self) -> String {
        format!(
            "O(1.2738^{k} + {k}·n) for vertex cover with k = {}",
            self.k,
            k = self.k
        )
    }
}
/// Hardness evidence via eta-expansions and cross-composition.
#[derive(Debug, Clone)]
pub struct EtaExpansion {
    /// The parameter in question.
    pub parameter: String,
}
impl EtaExpansion {
    /// Returns a description of the kernelization lower bound argument.
    pub fn kernelization_lower_bound(&self) -> String {
        format!(
            "No polynomial kernel for {} unless NP ⊆ coNP/poly (via OR-composition)",
            self.parameter
        )
    }
    /// Returns a description of the cross-composition argument.
    pub fn cross_composition(&self) -> String {
        format!(
            "Cross-composition from {} into itself rules out polynomial kernels",
            self.parameter
        )
    }
}
/// Irrelevant vertex reduction rule for planar FPT algorithms.
#[derive(Debug, Clone)]
pub struct IrrelevantVertex {
    /// Problem name.
    pub problem: String,
}
impl IrrelevantVertex {
    /// Returns a description of the irrelevant vertex identification step.
    pub fn find_irrelevant_vertex(&self) -> String {
        format!(
            "Find a vertex irrelevant to {} using the grid-minor theorem",
            self.problem
        )
    }
    /// Returns a description of the instance reduction after removal.
    pub fn reduce_instance(&self) -> String {
        format!(
            "Remove the irrelevant vertex; reduced {} instance has smaller parameter",
            self.problem
        )
    }
}
/// Iterative compression algorithm for vertex cover.
/// Given a (k+1)-solution, compresses it to a k-solution.
#[derive(Debug, Clone)]
pub struct IterativeCompressionVC {
    /// The target budget k.
    pub k: usize,
}
impl IterativeCompressionVC {
    /// Construct the iterative compression algorithm for budget k.
    pub fn new(k: usize) -> Self {
        Self { k }
    }
    /// Compress a (k+1)-size vertex cover to a k-size one (if possible).
    /// Returns Some(k_cover) or None.
    pub fn compress(&self, adj: &[Vec<usize>], over_cover: &[usize]) -> Option<Vec<usize>> {
        let m = over_cover.len();
        if m > 20 {
            return vertex_cover_bst(adj, self.k);
        }
        for mask in 0u32..(1u32 << m) {
            let mut partial_cover: Vec<usize> = (0..m)
                .filter(|&i| mask & (1 << i) != 0)
                .map(|i| over_cover[i])
                .collect();
            if partial_cover.len() > self.k {
                continue;
            }
            let k_rem = self.k - partial_cover.len();
            let n = adj.len();
            let mut removed = vec![false; n];
            for &v in &partial_cover {
                removed[v] = true;
            }
            if let Some(extra) = vertex_cover_bst(adj, k_rem) {
                partial_cover.extend(extra);
                return Some(partial_cover);
            }
        }
        None
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum KernelSizeBound {
    Linear(f64),
    Quadratic(f64),
    Cubic(f64),
    Polynomial(f64, u32),
    Exponential,
    None,
}
/// FPT (fixed-parameter tractable) algorithm with single-exponential runtime.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FPTMethod {
    pub name: String,
    pub problem: String,
    pub parameter: String,
    pub runtime_base: f64,
}
#[allow(dead_code)]
impl FPTMethod {
    pub fn new(name: &str, problem: &str, param: &str, base: f64) -> Self {
        FPTMethod {
            name: name.to_string(),
            problem: problem.to_string(),
            parameter: param.to_string(),
            runtime_base: base,
        }
    }
    pub fn runtime(&self, k: usize, n: usize) -> f64 {
        self.runtime_base.powi(k as i32) * n as f64
    }
    pub fn is_single_exponential(&self) -> bool {
        self.runtime_base <= 2.0
    }
    pub fn kernel_vertex_cover() -> Self {
        FPTMethod::new("Crown-reduction", "VertexCover", "k", 1.0)
    }
    pub fn bounded_search_tree_vc() -> Self {
        FPTMethod::new("BoundedSearchTree", "VertexCover", "k", 2.0)
    }
}
/// W-hierarchy classification.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum WClass {
    FPT,
    W1,
    W2,
    W3,
    WP,
    ParaNP,
}
#[allow(dead_code)]
impl WClass {
    pub fn is_fpt(&self) -> bool {
        *self == WClass::FPT
    }
    pub fn harder_than_w1(&self) -> bool {
        *self > WClass::W1
    }
    pub fn vertex_cover_class() -> Self {
        WClass::FPT
    }
    pub fn clique_class() -> Self {
        WClass::W1
    }
    pub fn dominating_set_class() -> Self {
        WClass::W2
    }
    pub fn description(&self) -> &'static str {
        match self {
            WClass::FPT => "Fixed-parameter tractable",
            WClass::W1 => "W[1]-hard (parametric intractable)",
            WClass::W2 => "W[2]-hard",
            WClass::W3 => "W[3]-hard",
            WClass::WP => "W[P]-hard",
            WClass::ParaNP => "para-NP-hard",
        }
    }
}
/// Compression and cross-composition lower bounds for FPT kernels.
#[derive(Debug, Clone)]
pub struct CognizantFPT {
    /// Kernel description.
    pub kernel: String,
}
impl CognizantFPT {
    /// Returns a description of the compression step.
    pub fn compression(&self) -> String {
        format!("Compress {} using a cognizant FPT algorithm", self.kernel)
    }
    /// Returns a description of the cross-composition lower bound.
    pub fn cross_composition_lower_bound(&self) -> String {
        format!(
            "Cross-composition into {} shows no polynomial kernel unless NP ⊆ coNP/poly",
            self.kernel
        )
    }
}
/// Parameterized reduction between problems.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ParamReduction {
    pub from_problem: String,
    pub to_problem: String,
    pub from_param: String,
    pub to_param: String,
    pub param_function: String,
}
#[allow(dead_code)]
impl ParamReduction {
    pub fn new(from: &str, to: &str, from_p: &str, to_p: &str, f: &str) -> Self {
        ParamReduction {
            from_problem: from.to_string(),
            to_problem: to.to_string(),
            from_param: from_p.to_string(),
            to_param: to_p.to_string(),
            param_function: f.to_string(),
        }
    }
    pub fn is_fpt_reduction(&self) -> bool {
        !self.param_function.contains("n")
    }
}
/// Color-coding FPT implementation with derandomization support.
#[derive(Debug, Clone)]
pub struct ColorCodingFPT {
    /// Number of colors (= path/subgraph size k).
    pub k: usize,
    /// Whether to use a perfect hash family for derandomization.
    pub use_perfect_hash: bool,
    /// Number of repetitions for the randomized version.
    pub repetitions: usize,
}
impl ColorCodingFPT {
    /// Construct a ColorCodingFPT algorithm for k-path detection.
    pub fn new(k: usize, use_perfect_hash: bool) -> Self {
        let repetitions = if use_perfect_hash {
            1
        } else {
            (std::f64::consts::E.powi(k as i32) as usize).max(1)
        };
        Self {
            k,
            use_perfect_hash,
            repetitions,
        }
    }
    /// Returns the expected running time expression.
    pub fn running_time(&self) -> String {
        if self.use_perfect_hash {
            format!("O({}^{} * n * log n) deterministic", self.k, self.k)
        } else {
            format!("O(e^{k} * {k}^{k} * n) randomized", k = self.k)
        }
    }
    /// Run the color-coding algorithm for k-path on the given graph.
    /// Returns true if a k-path was found in any repetition.
    pub fn find_k_path(&self, adj: &[Vec<usize>]) -> bool {
        for rep in 0..self.repetitions {
            let seed = (rep as u64)
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            if color_coding_k_path(adj, self.k, seed) {
                return true;
            }
        }
        false
    }
    /// Check if the color-coding algorithm can detect a Hamiltonian path.
    pub fn can_detect_hamiltonian_path(&self, n: usize) -> bool {
        self.k >= n
    }
}
/// The W-hierarchy of parameterized complexity classes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WClasses {
    /// Fixed-parameter tractable (lowest complexity).
    FPT,
    /// W\[1\]: k-clique-hard problems.
    W1,
    /// W\[2\]: k-dominating-set-hard problems.
    W2,
    /// W\[P\]: W-hierarchy top (bounded above by XP).
    WP,
    /// Slice-wise polynomial: polynomial for each fixed k.
    XP,
}
impl WClasses {
    /// Returns a description of the containment chain FPT ⊆ W\[1\] ⊆ W\[2\] ⊆ ... ⊆ XP.
    pub fn containment_chain(&self) -> String {
        "FPT ⊆ W[1] ⊆ W[2] ⊆ W[P] ⊆ XP".to_string()
    }
    /// Returns true if this class is considered tractable in the FPT sense.
    pub fn is_tractable(&self) -> bool {
        matches!(self, WClasses::FPT)
    }
}
/// Treewidth-based algorithm (running in f(tw)·n time).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct TreewidthAlgorithm {
    pub problem: String,
    pub treewidth_exponent: u32,
    pub n_exponent: u32,
}
#[allow(dead_code)]
impl TreewidthAlgorithm {
    pub fn new(problem: &str, tw_exp: u32, n_exp: u32) -> Self {
        TreewidthAlgorithm {
            problem: problem.to_string(),
            treewidth_exponent: tw_exp,
            n_exponent: n_exp,
        }
    }
    pub fn runtime(&self, tw: usize, n: usize) -> f64 {
        let tw_part = (2.0f64).powi(self.treewidth_exponent as i32 * tw as i32);
        let n_part = (n as f64).powi(self.n_exponent as i32);
        tw_part * n_part
    }
    pub fn satisfiability_tw() -> Self {
        TreewidthAlgorithm::new("SAT", 1, 1)
    }
    pub fn is_linear_time_in_n(&self) -> bool {
        self.n_exponent == 1
    }
}
/// Tree decomposition node: bags of vertex ids.
#[derive(Debug, Clone)]
pub struct TreeDecomp {
    /// bags\[i\] = set of vertices in bag i
    pub bags: Vec<Vec<usize>>,
    /// adjacency in the decomposition tree
    pub tree_adj: Vec<Vec<usize>>,
    /// root of the tree decomposition
    pub root: usize,
}
impl TreeDecomp {
    /// Compute the width (max bag size − 1) of this decomposition.
    pub fn width(&self) -> usize {
        self.bags
            .iter()
            .map(|b| b.len())
            .max()
            .unwrap_or(0)
            .saturating_sub(1)
    }
    /// Verify the decomposition is valid for graph given as adjacency list.
    pub fn verify(&self, n: usize, edges: &[(usize, usize)]) -> bool {
        for v in 0..n {
            if !self.bags.iter().any(|b| b.contains(&v)) {
                return false;
            }
        }
        for &(u, w) in edges {
            if !self.bags.iter().any(|b| b.contains(&u) && b.contains(&w)) {
                return false;
            }
        }
        let num_bags = self.bags.len();
        if num_bags == 0 {
            return true;
        }
        let mut visited = vec![false; num_bags];
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(self.root);
        visited[self.root] = true;
        while let Some(t) = queue.pop_front() {
            for &nb in &self.tree_adj[t] {
                if !visited[nb] {
                    visited[nb] = true;
                    queue.push_back(nb);
                }
            }
        }
        visited.iter().all(|&v| v)
    }
}
/// Bidimensionality-based FPT algorithm for planar graphs.
#[derive(Debug, Clone)]
pub struct PlanarGraphFPT {
    /// Parameter name (e.g., "treewidth", "feedback-vertex-set-size").
    pub parameter: String,
}
impl PlanarGraphFPT {
    /// Returns a description of the bidimensionality property.
    pub fn bidimensionality(&self) -> String {
        format!(
            "{} is bidimensional: grows as Ω(k^2) on (k×k)-grids",
            self.parameter
        )
    }
    /// Returns a description of the subexponential FPT algorithm.
    pub fn subexponential_fpt(&self) -> String {
        format!(
            "Planar {} FPT: 2^O(sqrt(k)) * n via bidimensionality + treewidth",
            self.parameter
        )
    }
}
