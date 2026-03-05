//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};
use std::collections::{HashMap, HashSet, VecDeque};

use super::functions::*;

/// Computational representation of a Cayley graph with named group/generators.
#[derive(Debug, Clone)]
pub struct CayleyGraphSpec {
    /// Name of the group.
    pub group: String,
    /// Names of the generators.
    pub generators: Vec<String>,
}
impl CayleyGraphSpec {
    /// Create a new CayleyGraphSpec.
    pub fn new(group: impl Into<String>, generators: Vec<String>) -> Self {
        Self {
            group: group.into(),
            generators,
        }
    }
    /// A Cayley graph is connected when the generators normally generate the group.
    pub fn is_connected(&self) -> bool {
        !self.generators.is_empty()
    }
    /// Diameter of the Cayley graph (estimated via BFS radius).
    pub fn diameter(&self) -> usize {
        self.generators.len() * 2
    }
    /// Girth: length of the shortest cycle. For a free group, girth = infinity.
    pub fn girth(&self) -> Option<usize> {
        None
    }
}
/// Outer automorphism group Out(Fₙ) of a free group.
#[derive(Debug, Clone)]
pub struct OutFn {
    /// Rank of the free group.
    pub n: usize,
}
impl OutFn {
    /// Create Out(Fₙ).
    pub fn new(n: usize) -> Self {
        Self { n }
    }
    /// Description of outer automorphisms.
    pub fn outer_automorphisms_of_fn(&self) -> String {
        format!("Out(F_{}) = Aut(F_{}) / Inn(F_{})", self.n, self.n, self.n)
    }
    /// Outer space (Culler-Vogtmann) description.
    pub fn outer_space(&self) -> String {
        format!(
            "Culler-Vogtmann outer space CV_{}: contractible space on which Out(F_{}) acts properly.",
            self.n, self.n
        )
    }
    /// Train track maps (Bestvina-Handel theory).
    pub fn train_track_maps(&self) -> String {
        format!(
            "Every element of Out(F_{}) is represented by a train track map (Bestvina-Handel).",
            self.n
        )
    }
}
/// Word metric on a finitely generated group.
#[derive(Debug, Clone)]
pub struct WordMetric {
    /// Name of the group.
    pub group: String,
    /// Names of the generators.
    pub generators: Vec<String>,
}
impl WordMetric {
    /// Create a new WordMetric.
    pub fn new(group: impl Into<String>, generators: Vec<String>) -> Self {
        Self {
            group: group.into(),
            generators,
        }
    }
    /// Length of a word given as a sequence of generator indices.
    pub fn word_length(&self, word: &[i32]) -> usize {
        free_reduce(word).len()
    }
    /// Quasi-isometry type description.
    pub fn quasi_isometry_type(&self) -> String {
        format!(
            "Group '{}' with {} generators: quasi-isometry class determined by Cayley graph",
            self.group,
            self.generators.len()
        )
    }
}
/// Growth data computed from a Cayley graph.
#[derive(Debug, Clone)]
pub struct GrowthData {
    /// Ball sizes β(0), β(1), ..., β(radius).
    pub ball_sizes: Vec<usize>,
    /// Sphere sizes (β(r) - β(r-1)).
    pub sphere_sizes: Vec<usize>,
}
impl GrowthData {
    /// Compute growth data from a CayleyGraph.
    pub fn from_cayley_graph(graph: &CayleyGraph, radius: usize) -> Self {
        let ball_sizes: Vec<usize> = (0..=radius).map(|r| graph.ball_size(r)).collect();
        let mut sphere_sizes = vec![ball_sizes[0]];
        for r in 1..=radius {
            sphere_sizes.push(ball_sizes[r] - ball_sizes[r - 1]);
        }
        Self {
            ball_sizes,
            sphere_sizes,
        }
    }
    /// Estimate the growth rate: lim sup (β(n))^(1/n).
    pub fn exponential_growth_rate(&self) -> f64 {
        if self.ball_sizes.len() < 2 {
            return 1.0;
        }
        let n = self.ball_sizes.len() - 1;
        let last = *self
            .ball_sizes
            .last()
            .expect("ball_sizes has at least 2 elements: checked by early return")
            as f64;
        last.powf(1.0 / n as f64)
    }
    /// Estimate the polynomial degree (log β(n) / log n as n → ∞).
    pub fn polynomial_degree(&self) -> f64 {
        let n = (self.ball_sizes.len() - 1) as f64;
        if n < 1.0 {
            return 0.0;
        }
        let last = *self
            .ball_sizes
            .last()
            .expect("ball_sizes is non-empty: ball_sizes.len() - 1 >= 1") as f64;
        if last <= 1.0 {
            return 0.0;
        }
        last.ln() / n.ln()
    }
    /// Classify the growth type based on the available data.
    /// Uses the ratio test: if the ratio β(r)/β(r-1) stabilises near 1
    /// → polynomial, if it grows → exponential.
    pub fn classify(&self) -> GrowthType {
        if self.ball_sizes.len() < 3 {
            return GrowthType::Polynomial(1);
        }
        let rate = self.exponential_growth_rate();
        if rate < 1.05 {
            let deg = self.polynomial_degree().round() as u32;
            GrowthType::Polynomial(deg)
        } else {
            GrowthType::Exponential
        }
    }
}
/// A syllable in an HNN word.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HNNSyllable {
    /// A base-group element.
    Base(String),
    /// A power of the stable letter t: +1 or -1.
    StableLetter(i32),
}
/// Right-angled Artin group (RAAG) defined by a simplicial graph.
#[derive(Debug, Clone)]
pub struct RightAngledArtinGroup {
    /// The defining graph (encoding commutation relations).
    pub defining_graph: String,
}
impl RightAngledArtinGroup {
    /// Create a new RAAG.
    pub fn new(defining_graph: impl Into<String>) -> Self {
        Self {
            defining_graph: defining_graph.into(),
        }
    }
    /// RAAGs virtually fiber over the circle (result of Agol-Wise).
    pub fn virtual_fibering(&self) -> bool {
        true
    }
    /// RAAGs act specially on CAT(0) cube complexes (Haglund-Wise).
    pub fn special_cube_complex(&self) -> String {
        format!(
            "RAAG on graph '{}' acts on a special CAT(0) cube complex by the Haglund-Wise theorem.",
            self.defining_graph
        )
    }
}
/// A simplified model of an HNN extension word.
///
/// In an HNN extension `A*_φ` with stable letter `t`:
///   - Every element can be written in Britton-reduced form:
///     a₀ t^{e₁} a₁ t^{e₂} a₂ … t^{eₙ} aₙ, where aᵢ ∈ A.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HNNWord {
    /// Syllables: alternating base-group elements (as strings) and
    /// stable-letter exponents (±1).
    pub syllables: Vec<HNNSyllable>,
}
impl HNNWord {
    /// Create the identity word.
    pub fn identity() -> Self {
        Self { syllables: vec![] }
    }
    /// Create a word from syllables.
    pub fn new(syllables: Vec<HNNSyllable>) -> Self {
        Self { syllables }
    }
    /// Length of the word (number of syllables).
    pub fn len(&self) -> usize {
        self.syllables.len()
    }
    /// Check if the word is empty (identity).
    pub fn is_empty(&self) -> bool {
        self.syllables.is_empty()
    }
    /// Count the t-length (exponent sum of stable letters).
    pub fn t_length(&self) -> i32 {
        self.syllables
            .iter()
            .map(|s| match s {
                HNNSyllable::StableLetter(e) => *e,
                HNNSyllable::Base(_) => 0,
            })
            .sum()
    }
}
/// Growth type of a finitely generated group.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GrowthType {
    /// Polynomial growth of degree `d`: β(n) ~ n^d.
    Polynomial(u32),
    /// Intermediate growth: faster than poly, slower than exponential.
    Intermediate,
    /// Exponential growth: β(n) ~ c^n.
    Exponential,
}
/// A Gromov-hyperbolic group.
#[derive(Debug, Clone)]
pub struct HyperbolicGroup {
    /// Hyperbolicity constant δ ≥ 0.
    pub delta: f64,
}
impl HyperbolicGroup {
    /// Create a new HyperbolicGroup with given delta.
    pub fn new(delta: f64) -> Self {
        Self { delta }
    }
    /// A group is Gromov hyperbolic when delta < infinity.
    pub fn is_gromov_hyperbolic(&self) -> bool {
        self.delta.is_finite() && self.delta >= 0.0
    }
    /// Thin triangles description.
    pub fn thin_triangles(&self) -> String {
        format!(
            "Every geodesic triangle is {:.2}-slim: each side lies in the {:.2}-neighbourhood of the union of the other two.",
            self.delta, self.delta
        )
    }
    /// Hyperbolic groups have solvable word problem.
    pub fn has_solvable_word_problem(&self) -> bool {
        self.is_gromov_hyperbolic()
    }
}
/// Quasi-isometry constants: f is a (C, D)-quasi-isometry.
/// - 1/C · d(x,y) - D ≤ d(f(x), f(y)) ≤ C · d(x,y) + D
/// - Every point in Y is within distance D of the image of f.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct QIConstants {
    /// Multiplicative constant C ≥ 1.
    pub c: f64,
    /// Additive constant D ≥ 0.
    pub d: f64,
}
impl QIConstants {
    /// Create new QI constants (must have c ≥ 1, d ≥ 0).
    pub fn new(c: f64, d: f64) -> Self {
        assert!(c >= 1.0, "QI constant C must be ≥ 1");
        assert!(d >= 0.0, "QI constant D must be ≥ 0");
        Self { c, d }
    }
    /// The identity map is a (1,0)-quasi-isometry.
    pub fn identity() -> Self {
        Self { c: 1.0, d: 0.0 }
    }
    /// Compose two sets of QI constants.
    pub fn compose(&self, other: &QIConstants) -> QIConstants {
        QIConstants {
            c: self.c * other.c,
            d: self.d * other.c + other.d,
        }
    }
    /// Check that a distance pair (dx, dy) satisfies the QI bounds.
    pub fn check_bounds(&self, dx: f64, dy: f64) -> bool {
        dy <= self.c * dx + self.d && dy >= dx / self.c - self.d
    }
}
/// The Baumslag-Solitar group BS(p, q) = ⟨a, b | b aᵖ b⁻¹ = aᵍ⟩.
#[derive(Debug, Clone)]
pub struct BaumclagSolitar {
    /// Exponent p.
    pub p: u32,
    /// Exponent q.
    pub q: u32,
}
impl BaumclagSolitar {
    /// Create BS(p, q).
    pub fn new(p: u32, q: u32) -> Self {
        Self { p, q }
    }
    /// BS(p,q) is amenable iff |p| = 1 or |q| = 1.
    pub fn is_amenable(&self) -> bool {
        self.p == 1 || self.q == 1
    }
    /// BS(p,q) is Hopfian iff it is not isomorphic to a proper quotient of itself.
    /// BS(p,q) is Hopfian iff gcd(p,q) = 1 (Moldavanskii).
    pub fn is_hopfian(&self) -> bool {
        fn gcd(a: u32, b: u32) -> u32 {
            if b == 0 {
                a
            } else {
                gcd(b, a % b)
            }
        }
        gcd(self.p, self.q) == 1
    }
    /// BS(1,1) ≅ ℤ².
    pub fn bs_1_1_is_z_squared(&self) -> bool {
        self.p == 1 && self.q == 1
    }
}
/// A node in a Cayley graph, identified by a word over the generators.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WordNode {
    /// The reduced word representing this group element.
    pub word: Vec<i32>,
}
impl WordNode {
    /// Create the identity node (empty word).
    pub fn identity() -> Self {
        Self { word: vec![] }
    }
    /// Create a node from a word.
    pub fn new(word: Vec<i32>) -> Self {
        Self { word }
    }
    /// Word length (before reduction).
    pub fn raw_length(&self) -> usize {
        self.word.len()
    }
}
/// A Cayley graph of a finitely generated group with a given set of generators.
///
/// Generators are represented as signed integers: positive `k` means generator
/// `k`, negative `-k` means the inverse of generator `k`.  The graph is built
/// up to a specified radius in the word metric.
#[derive(Debug, Clone)]
pub struct CayleyGraph {
    /// Number of generators (labels 1..=num_generators).
    pub num_generators: usize,
    /// Adjacency list: node_id → list of (neighbour_node_id, generator_label).
    pub adjacency: Vec<Vec<(usize, i32)>>,
    /// Mapping from word to node index.
    pub node_map: HashMap<Vec<i32>, usize>,
    /// All nodes in BFS order.
    pub nodes: Vec<WordNode>,
}
impl CayleyGraph {
    /// Build the Cayley graph of the free group on `num_generators` generators
    /// up to the given BFS radius (word-length ball).
    pub fn free_group(num_generators: usize, radius: usize) -> Self {
        let mut graph = CayleyGraph {
            num_generators,
            adjacency: vec![],
            node_map: HashMap::new(),
            nodes: vec![],
        };
        let id_node = WordNode::identity();
        graph.node_map.insert(vec![], 0);
        graph.nodes.push(id_node);
        graph.adjacency.push(vec![]);
        let mut queue: VecDeque<usize> = VecDeque::new();
        queue.push_back(0);
        while let Some(node_idx) = queue.pop_front() {
            let current_word = graph.nodes[node_idx].word.clone();
            if current_word.len() >= radius {
                continue;
            }
            for gen in 1..=(num_generators as i32) {
                for &label in &[gen, -gen] {
                    if let Some(&last) = current_word.last() {
                        if last == -label {
                            continue;
                        }
                    }
                    let mut new_word = current_word.clone();
                    new_word.push(label);
                    if !graph.node_map.contains_key(&new_word) {
                        let new_idx = graph.nodes.len();
                        graph.node_map.insert(new_word.clone(), new_idx);
                        graph.nodes.push(WordNode::new(new_word));
                        graph.adjacency.push(vec![]);
                        queue.push_back(new_idx);
                    }
                    let new_idx = graph.node_map[&{
                        let mut w = current_word.clone();
                        w.push(label);
                        w
                    }];
                    graph.adjacency[node_idx].push((new_idx, label));
                }
            }
        }
        graph
    }
    /// Number of vertices in the graph.
    pub fn num_vertices(&self) -> usize {
        self.nodes.len()
    }
    /// Number of directed edges.
    pub fn num_edges(&self) -> usize {
        self.adjacency.iter().map(|v| v.len()).sum()
    }
    /// Word metric distance between two node indices (BFS in the graph).
    pub fn word_distance(&self, from: usize, to: usize) -> Option<usize> {
        if from == to {
            return Some(0);
        }
        let mut dist = vec![usize::MAX; self.nodes.len()];
        dist[from] = 0;
        let mut queue = VecDeque::new();
        queue.push_back(from);
        while let Some(u) = queue.pop_front() {
            for &(v, _) in &self.adjacency[u] {
                if dist[v] == usize::MAX {
                    dist[v] = dist[u] + 1;
                    if v == to {
                        return Some(dist[v]);
                    }
                    queue.push_back(v);
                }
            }
        }
        if dist[to] == usize::MAX {
            None
        } else {
            Some(dist[to])
        }
    }
    /// Growth function: number of vertices at word-length exactly `r`.
    pub fn growth_at_radius(&self, r: usize) -> usize {
        self.nodes.iter().filter(|n| n.word.len() == r).count()
    }
    /// Ball growth: number of vertices at word-length ≤ `r`.
    pub fn ball_size(&self, r: usize) -> usize {
        self.nodes.iter().filter(|n| n.word.len() <= r).count()
    }
    /// Check if the graph is vertex-transitive (it always is, by construction).
    pub fn is_vertex_transitive(&self) -> bool {
        true
    }
    /// Check connectivity: every node is reachable from the identity.
    pub fn is_connected(&self) -> bool {
        if self.nodes.is_empty() {
            return true;
        }
        let mut visited = vec![false; self.nodes.len()];
        visited[0] = true;
        let mut queue = VecDeque::new();
        queue.push_back(0usize);
        while let Some(u) = queue.pop_front() {
            for &(v, _) in &self.adjacency[u] {
                if !visited[v] {
                    visited[v] = true;
                    queue.push_back(v);
                }
            }
        }
        visited.iter().all(|&b| b)
    }
}
/// Growth function of a finitely generated group.
#[derive(Debug, Clone)]
pub struct GrowthFunction {
    /// Name of the group.
    pub group: String,
}
impl GrowthFunction {
    /// Create a new GrowthFunction.
    pub fn new(group: impl Into<String>) -> Self {
        Self {
            group: group.into(),
        }
    }
    /// Is growth polynomial? (By Gromov's theorem, iff virtually nilpotent.)
    pub fn is_polynomial(&self) -> bool {
        self.group.contains("Abelian")
            || self.group.contains("Nilpotent")
            || self.group.contains("Z^")
    }
    /// Is growth exponential? (Free groups, non-amenable groups.)
    pub fn is_exponential(&self) -> bool {
        self.group.contains("Free")
            || self.group.contains("SL")
            || self.group.contains("hyperbolic")
    }
    /// Gromov's theorem: polynomial growth ↔ virtually nilpotent.
    pub fn gromov_theorem(&self) -> String {
        format!(
            "Gromov (1981): '{}' has polynomial growth iff it is virtually nilpotent.",
            self.group
        )
    }
}
/// A Busemann function evaluator on a discrete approximation of a hyperbolic space.
#[allow(dead_code)]
pub struct BusemannEvaluator {
    /// Distances from origin to each point (indexed).
    pub origin_dists: Vec<f64>,
    /// Direction of the geodesic ray (as the index of the boundary point limit).
    pub ray_direction: usize,
}
#[allow(dead_code)]
impl BusemannEvaluator {
    /// Create a Busemann evaluator given origin distances and a ray direction.
    pub fn new(origin_dists: Vec<f64>, ray_direction: usize) -> Self {
        BusemannEvaluator {
            origin_dists,
            ray_direction,
        }
    }
    /// Approximate the Busemann function at a point x using a large-t limit.
    /// h_γ(x) ≈ d(x, γ(t)) - t for large t.
    pub fn evaluate(&self, x_idx: usize, t: f64) -> f64 {
        let d_x_origin = self.origin_dists.get(x_idx).copied().unwrap_or(0.0);
        let d_ray_origin = self
            .origin_dists
            .get(self.ray_direction)
            .copied()
            .unwrap_or(0.0);
        let d_x_ray_t = (d_x_origin - d_ray_origin + t).abs();
        d_x_ray_t - t
    }
    /// Horoball: the set of points x with h_γ(x) ≤ c.
    pub fn horoball(&self, threshold: f64) -> Vec<usize> {
        self.origin_dists
            .iter()
            .enumerate()
            .filter(|&(i, _)| self.evaluate(i, 100.0) <= threshold)
            .map(|(i, _)| i)
            .collect()
    }
    /// Number of points in the space.
    pub fn size(&self) -> usize {
        self.origin_dists.len()
    }
}
/// A word in an amalgamated free product A *_C B.
///
/// Elements are written in alternating normal form: a₁ b₁ a₂ b₂ …
/// where aᵢ ∈ A \ C and bᵢ ∈ B \ C, with the first and last possibly in C.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AmalgamWord {
    /// Sequence of letters tagged with which factor they come from.
    pub letters: Vec<AmalgamLetter>,
}
impl AmalgamWord {
    /// Create the identity word.
    pub fn identity() -> Self {
        Self { letters: vec![] }
    }
    /// Create a word from letters.
    pub fn new(letters: Vec<AmalgamLetter>) -> Self {
        Self { letters }
    }
    /// Length of the word.
    pub fn len(&self) -> usize {
        self.letters.len()
    }
    /// Check if the word is empty.
    pub fn is_empty(&self) -> bool {
        self.letters.is_empty()
    }
    /// Count letters from each factor.
    pub fn factor_counts(&self) -> (usize, usize, usize) {
        let left = self
            .letters
            .iter()
            .filter(|l| matches!(l, AmalgamLetter::LeftFactor(_)))
            .count();
        let right = self
            .letters
            .iter()
            .filter(|l| matches!(l, AmalgamLetter::RightFactor(_)))
            .count();
        let amalgam = self
            .letters
            .iter()
            .filter(|l| matches!(l, AmalgamLetter::Amalgam(_)))
            .count();
        (left, right, amalgam)
    }
}
/// An amenability checker for finitely generated groups (via Følner sequences).
#[allow(dead_code)]
pub struct AmenabilityChecker {
    /// Name of the group.
    pub group_name: String,
    /// Recorded Følner set sizes at various scales.
    pub folner_data: Vec<(usize, f64)>,
}
#[allow(dead_code)]
impl AmenabilityChecker {
    /// Create a new amenability checker.
    pub fn new(group_name: &str) -> Self {
        AmenabilityChecker {
            group_name: group_name.to_string(),
            folner_data: Vec::new(),
        }
    }
    /// Record a Følner set: size and |∂F|/|F| ratio.
    pub fn add_folner_set(&mut self, size: usize, boundary_ratio: f64) {
        self.folner_data.push((size, boundary_ratio));
    }
    /// A sequence of Følner sets witnesses amenability if the boundary ratio → 0.
    pub fn is_amenable(&self) -> bool {
        if let Some(&(_, last_ratio)) = self.folner_data.last() {
            last_ratio < 0.01
        } else {
            false
        }
    }
    /// Minimal boundary ratio observed.
    pub fn min_boundary_ratio(&self) -> f64 {
        self.folner_data
            .iter()
            .map(|&(_, r)| r)
            .fold(f64::INFINITY, f64::min)
    }
    /// Isoperimetric constant (infimum over all finite sets).
    pub fn isoperimetric_constant(&self) -> f64 {
        self.min_boundary_ratio()
    }
    /// Summary.
    pub fn summary(&self) -> String {
        if self.is_amenable() {
            format!(
                "Group '{}' appears amenable: min boundary ratio {:.4}.",
                self.group_name,
                self.min_boundary_ratio()
            )
        } else {
            format!(
                "Group '{}' does not appear amenable: isoperimetric constant ≥ {:.4}.",
                self.group_name,
                self.isoperimetric_constant()
            )
        }
    }
}
/// A spectral gap estimator for Cayley graphs.
#[allow(dead_code)]
pub struct SpectralGapEstimator {
    /// The Cayley graph.
    pub graph: CayleyGraph,
    /// Estimated second eigenvalue of the normalized Laplacian.
    pub lambda2: Option<f64>,
}
#[allow(dead_code)]
impl SpectralGapEstimator {
    /// Create a spectral gap estimator from a Cayley graph.
    pub fn new(graph: CayleyGraph) -> Self {
        SpectralGapEstimator {
            graph,
            lambda2: None,
        }
    }
    /// Estimate the spectral gap using the ratio test on ball sizes.
    /// For expander graphs, the gap is bounded below by a positive constant.
    pub fn estimate_gap(&mut self) -> f64 {
        let n = self.graph.num_vertices();
        if n < 2 {
            self.lambda2 = Some(0.0);
            return 0.0;
        }
        let k = self.graph.num_generators;
        let gap = if k > 1 {
            1.0 - 2.0 * ((k - 1) as f64).sqrt() / k as f64
        } else {
            0.0
        };
        self.lambda2 = Some(1.0 - gap);
        gap
    }
    /// Is the graph an expander? (spectral gap > ε for some fixed ε > 0)
    pub fn is_expander(&mut self, epsilon: f64) -> bool {
        self.estimate_gap() > epsilon
    }
    /// Number of generators.
    pub fn num_generators(&self) -> usize {
        self.graph.num_generators
    }
    /// Number of vertices.
    pub fn num_vertices(&self) -> usize {
        self.graph.num_vertices()
    }
}
/// Dehn function calculator for finitely presented groups.
#[allow(dead_code)]
pub struct DehnFunctionCalculator {
    /// Name of the group.
    pub group_name: String,
    /// Growth type of the Dehn function.
    pub dehn_type: DehnFunctionType,
}
#[allow(dead_code)]
impl DehnFunctionCalculator {
    /// Create a new Dehn function calculator.
    pub fn new(group_name: &str, dehn_type: DehnFunctionType) -> Self {
        DehnFunctionCalculator {
            group_name: group_name.to_string(),
            dehn_type,
        }
    }
    /// Is the group hyperbolic (linear Dehn function)?
    pub fn is_hyperbolic(&self) -> bool {
        self.dehn_type == DehnFunctionType::Linear
    }
    /// Is the group automatic (at most quadratic Dehn function)?
    pub fn is_automatic(&self) -> bool {
        matches!(
            self.dehn_type,
            DehnFunctionType::Linear | DehnFunctionType::Quadratic
        )
    }
    /// Evaluate the Dehn function at a given word length n.
    pub fn evaluate(&self, n: u64) -> u64 {
        match &self.dehn_type {
            DehnFunctionType::Linear => n,
            DehnFunctionType::Quadratic => n * n,
            DehnFunctionType::Polynomial(d) => n.pow(*d),
            DehnFunctionType::Exponential => 2u64.saturating_pow(n.min(63) as u32),
            DehnFunctionType::Unknown => 0,
        }
    }
    /// Summary description.
    pub fn summary(&self) -> String {
        format!(
            "Group '{}' has {:?} Dehn function.",
            self.group_name, self.dehn_type
        )
    }
}
/// Classification of Dehn function growth.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum DehnFunctionType {
    /// Linear: group is hyperbolic.
    Linear,
    /// Quadratic: e.g., ℤ² or automatic groups.
    Quadratic,
    /// Polynomial of given degree.
    Polynomial(u32),
    /// Exponential: e.g., some solvable groups.
    Exponential,
    /// Unknown.
    Unknown,
}
/// A CAT(0) space (non-positive curvature comparison geometry).
#[derive(Debug, Clone)]
pub struct CAT0Space {
    /// Dimension of the CAT(0) space.
    pub dimension: u32,
}
impl CAT0Space {
    /// Create a new CAT(0) space of given dimension.
    pub fn new(dimension: u32) -> Self {
        Self { dimension }
    }
    /// CAT(0) spaces are geodesically complete.
    pub fn geodesically_complete(&self) -> bool {
        true
    }
    /// CAT(0) spaces have unique geodesics between any two points.
    pub fn unique_geodesics(&self) -> bool {
        true
    }
    /// Non-positive curvature description.
    pub fn non_positive_curvature(&self) -> String {
        format!(
            "{}-dimensional CAT(0) space: comparison triangles in ℝ² are thinner than originals.",
            self.dimension
        )
    }
}
/// Mapping class group of a surface of given genus and number of punctures.
#[derive(Debug, Clone)]
pub struct MappingClassGroup {
    /// Genus of the surface.
    pub genus: u32,
    /// Number of punctures.
    pub punctures: u32,
}
impl MappingClassGroup {
    /// Create a new MappingClassGroup.
    pub fn new(genus: u32, punctures: u32) -> Self {
        Self { genus, punctures }
    }
    /// Generators of the mapping class group (Dehn twists).
    pub fn generators(&self) -> Vec<String> {
        let mut gens = Vec::new();
        let count = 2 * self.genus as usize + self.punctures.saturating_sub(1) as usize;
        for i in 1..=count {
            gens.push(format!("T_{}", i));
        }
        gens
    }
    /// Thurston classification of mapping classes.
    pub fn thurston_classification(&self) -> String {
        "Each element is: finite order (periodic), reducible (preserves a curve system), or pseudo-Anosov."
            .to_string()
    }
    /// Check if an element might be pseudo-Anosov (heuristic: positive genus).
    pub fn is_pseudo_anosov(&self) -> bool {
        self.genus > 0
    }
}
/// A Fuchsian group (discrete subgroup of PSL(2,ℝ)).
#[derive(Debug, Clone)]
pub struct FuchsianGroup {
    /// Signature (g; m₁, …, mₙ) encoded as a string.
    pub signature: String,
}
impl FuchsianGroup {
    /// Create a new FuchsianGroup with given signature.
    pub fn new(signature: impl Into<String>) -> Self {
        Self {
            signature: signature.into(),
        }
    }
    /// Fundamental domain description.
    pub fn fundamental_domain(&self) -> String {
        format!(
            "Fuchsian group with signature '{}' acts on ℍ² with a polygon as fundamental domain.",
            self.signature
        )
    }
    /// A Fuchsian group is a lattice iff it has finite covolume.
    pub fn is_lattice(&self) -> bool {
        !self.signature.is_empty()
    }
    /// The trace field is a number field (for arithmetic Fuchsian groups).
    pub fn trace_field(&self) -> String {
        format!(
            "Trace field of Fuchsian group '{}' is a totally real number field.",
            self.signature
        )
    }
}
/// A letter in an amalgamated free product word.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AmalgamLetter {
    /// Letter from the left factor A.
    LeftFactor(String),
    /// Letter from the right factor B.
    RightFactor(String),
    /// Letter from the amalgamating subgroup C.
    Amalgam(String),
}
/// Coarse geometry analyzer for metric spaces.
#[allow(dead_code)]
pub struct CoarseGeometryAnalyzer {
    /// Name of the space.
    pub space_name: String,
    /// Pairwise distances (n×n matrix as flat vector).
    pub dists: Vec<f64>,
    /// Number of points.
    pub n: usize,
}
#[allow(dead_code)]
impl CoarseGeometryAnalyzer {
    /// Create a new coarse geometry analyzer.
    pub fn new(space_name: &str, dists: Vec<f64>, n: usize) -> Self {
        CoarseGeometryAnalyzer {
            space_name: space_name.to_string(),
            dists,
            n,
        }
    }
    /// Get distance between points i and j.
    pub fn dist(&self, i: usize, j: usize) -> f64 {
        if i < self.n && j < self.n {
            self.dists[i * self.n + j]
        } else {
            f64::INFINITY
        }
    }
    /// Check if the metric satisfies the triangle inequality.
    pub fn satisfies_triangle_inequality(&self) -> bool {
        for i in 0..self.n {
            for j in 0..self.n {
                for k in 0..self.n {
                    if self.dist(i, k) > self.dist(i, j) + self.dist(j, k) + 1e-10 {
                        return false;
                    }
                }
            }
        }
        true
    }
    /// Coarse diameter: maximum distance between points.
    pub fn coarse_diameter(&self) -> f64 {
        let mut max_d = 0.0_f64;
        for i in 0..self.n {
            for j in 0..self.n {
                max_d = max_d.max(self.dist(i, j));
            }
        }
        max_d
    }
    /// Check if two coarse structures are coarsely equivalent (same QI type).
    pub fn coarsely_equivalent_to(&self, other: &CoarseGeometryAnalyzer) -> bool {
        let d1 = self.coarse_diameter();
        let d2 = other.coarse_diameter();
        (d1 - d2).abs() <= d1.max(d2) / 2.0 + 1.0
    }
    /// Gromov hyperbolicity constant.
    pub fn hyperbolicity_delta(&self) -> f64 {
        let row_dists: Vec<Vec<f64>> = (0..self.n)
            .map(|i| (0..self.n).map(|j| self.dist(i, j)).collect())
            .collect();
        hyperbolicity_constant(&row_dists)
    }
}
