//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};
use std::collections::{HashMap, HashSet};

/// A Čech complex: nerve of balls of radius ε around data points.
#[derive(Debug, Clone)]
pub struct CechComplex {
    /// The resulting simplicial complex.
    pub complex: SimplicialComplex,
    /// The radius parameter ε.
    pub epsilon: f64,
}
impl CechComplex {
    /// Build a Čech complex from point coordinates and radius ε.
    ///
    /// `points[i]` = coordinate vector of point i.
    /// A simplex {i₀, …, iₖ} is included iff the intersection of balls B(iⱼ, ε) is non-empty,
    /// which is approximated here by: circumradius of the set of points ≤ ε.
    pub fn build(points: &[Vec<f64>], epsilon: f64) -> Self {
        let n = points.len();
        let mut complex = SimplicialComplex::new();
        for i in 0..n {
            complex.add_simplex(Simplex::new(vec![i]));
        }
        for i in 0..n {
            for j in (i + 1)..n {
                if Self::euclidean_dist(&points[i], &points[j]) / 2.0 <= epsilon {
                    complex.add_simplex(Simplex::new(vec![i, j]));
                }
            }
        }
        Self { complex, epsilon }
    }
    fn euclidean_dist(a: &[f64], b: &[f64]) -> f64 {
        a.iter()
            .zip(b.iter())
            .map(|(x, y)| (x - y).powi(2))
            .sum::<f64>()
            .sqrt()
    }
}
/// A node in a Reeb graph.
#[derive(Debug, Clone)]
pub struct ReebNode {
    /// Node index.
    pub id: usize,
    /// Filter value at this node.
    pub value: f64,
    /// Node type: Minimum, Maximum, Saddle, Regular.
    pub node_type: ReebNodeType,
}
/// Computes and stores the persistence landscape λₖ(t) for a persistence diagram.
///
/// The k-th landscape function is defined as the k-th largest tent function:
/// λₖ(t) = kth-max_{[b,d] ∈ dgm} min(t - b, d - t)₊.
#[allow(dead_code)]
pub struct PersistenceLandscapeComputer {
    /// The persistence intervals (finite only).
    pub intervals: Vec<(f64, f64)>,
    /// Evaluation grid.
    pub t_values: Vec<f64>,
}
impl PersistenceLandscapeComputer {
    /// Create a landscape computer for a diagram, evaluated at `n_pts` points in [lo, hi].
    pub fn new(diagram: &PersistenceDiagram, lo: f64, hi: f64, n_pts: usize) -> Self {
        let intervals: Vec<(f64, f64)> = diagram
            .intervals
            .iter()
            .filter(|i| i.persistence().is_finite() && i.persistence() > 0.0)
            .map(|i| (i.birth, i.death))
            .collect();
        let n = if n_pts == 0 { 1 } else { n_pts };
        let dt = if n <= 1 {
            0.0
        } else {
            (hi - lo) / (n - 1) as f64
        };
        let t_values: Vec<f64> = (0..n).map(|i| lo + i as f64 * dt).collect();
        Self {
            intervals,
            t_values,
        }
    }
    /// Evaluate the k-th landscape function (1-indexed) at all grid points.
    pub fn landscape_k(&self, k: usize) -> Vec<f64> {
        if k == 0 {
            return vec![0.0; self.t_values.len()];
        }
        self.t_values
            .iter()
            .map(|&t| {
                let mut vals: Vec<f64> = self
                    .intervals
                    .iter()
                    .map(|&(b, d)| {
                        let v = (t - b).min(d - t);
                        if v > 0.0 {
                            v
                        } else {
                            0.0
                        }
                    })
                    .collect();
                vals.sort_by(|a, b_| b_.partial_cmp(a).unwrap_or(std::cmp::Ordering::Equal));
                vals.get(k - 1).copied().unwrap_or(0.0)
            })
            .collect()
    }
    /// Compute the L_p norm of the k-th landscape function.
    pub fn landscape_lp_norm(&self, k: usize, p: f64) -> f64 {
        let vals = self.landscape_k(k);
        let n = vals.len();
        if n < 2 {
            return 0.0;
        }
        let dt = if n <= 1 {
            0.0
        } else {
            (self.t_values[n - 1] - self.t_values[0]) / (n - 1) as f64
        };
        let integral: f64 = vals.iter().map(|&v| v.powf(p)).sum::<f64>() * dt;
        integral.powf(1.0 / p)
    }
}
/// A persistence interval [birth, death).
#[derive(Debug, Clone, PartialEq)]
pub struct PersistenceInterval {
    /// Homological dimension.
    pub dimension: usize,
    /// Birth filtration value.
    pub birth: f64,
    /// Death filtration value (f64::INFINITY for essential classes).
    pub death: f64,
}
impl PersistenceInterval {
    /// Create a persistence interval.
    pub fn new(dimension: usize, birth: f64, death: f64) -> Self {
        Self {
            dimension,
            birth,
            death,
        }
    }
    /// Persistence = death - birth.
    pub fn persistence(&self) -> f64 {
        self.death - self.birth
    }
    /// True if this is an essential (infinite) bar.
    pub fn is_essential(&self) -> bool {
        self.death.is_infinite()
    }
}
/// A zigzag filtration over a sequence of simplicial complexes.
///
/// Consists of a sequence X₀ ← X₁ → X₂ ← X₃ → … of complexes with
/// inclusions in alternating directions.
#[allow(dead_code)]
pub struct ZigzagFiltrationBuilder {
    /// The sequence of simplicial complexes.
    pub complexes: Vec<SimplicialComplex>,
    /// True if the map from index i to i+1 is forward (inclusion Xi → Xi+1),
    /// false if backward (inclusion Xi+1 → Xi).
    pub directions: Vec<bool>,
}
impl ZigzagFiltrationBuilder {
    /// Create a new zigzag filtration.
    pub fn new() -> Self {
        Self {
            complexes: Vec::new(),
            directions: Vec::new(),
        }
    }
    /// Append a complex with a forward inclusion.
    pub fn push_forward(&mut self, complex: SimplicialComplex) {
        self.complexes.push(complex);
        if !self.directions.is_empty() || self.complexes.len() > 1 {
            self.directions.push(true);
        }
    }
    /// Append a complex with a backward inclusion.
    pub fn push_backward(&mut self, complex: SimplicialComplex) {
        self.complexes.push(complex);
        if self.complexes.len() > 1 {
            self.directions.push(false);
        }
    }
    /// Compute Euler characteristics at each step.
    pub fn euler_characteristics(&self) -> Vec<i64> {
        self.complexes
            .iter()
            .map(|c| c.euler_characteristic())
            .collect()
    }
    /// Compute the number of simplices at each step (rough measure of complexity).
    pub fn simplex_counts(&self) -> Vec<usize> {
        self.complexes.iter().map(|c| c.simplices.len()).collect()
    }
    /// Number of complexes in the filtration.
    pub fn len(&self) -> usize {
        self.complexes.len()
    }
    /// True if the filtration is empty.
    pub fn is_empty(&self) -> bool {
        self.complexes.is_empty()
    }
}
/// A persistence image: a stable vectorization of a persistence diagram.
///
/// The persistence image maps a persistence diagram to a 2D grid of weighted
/// Gaussian kernel densities.
#[allow(dead_code)]
pub struct PersistenceImage {
    /// Resolution in each axis (pixels × pixels grid).
    pub resolution: usize,
    /// Bandwidth (standard deviation of Gaussian kernel).
    pub bandwidth: f64,
    /// Weight function for each bar (e.g., persistence^2).
    pub weight_power: f64,
    /// The image pixels (resolution × resolution matrix).
    pub pixels: Vec<Vec<f64>>,
    /// Bounding box of the diagram [min_birth, max_birth] × [0, max_persistence].
    pub birth_range: (f64, f64),
    /// Maximum persistence value.
    pub max_persistence: f64,
}
impl PersistenceImage {
    /// Compute a persistence image from a persistence diagram.
    pub fn compute(
        diagram: &PersistenceDiagram,
        resolution: usize,
        bandwidth: f64,
        weight_power: f64,
    ) -> Self {
        let finite: Vec<(&PersistenceInterval, f64)> = diagram
            .intervals
            .iter()
            .filter(|i| i.persistence().is_finite() && i.persistence() > 0.0)
            .map(|i| (i, i.persistence()))
            .collect();
        let birth_min = finite
            .iter()
            .map(|(i, _)| i.birth)
            .fold(f64::INFINITY, f64::min);
        let birth_max = finite
            .iter()
            .map(|(i, _)| i.birth)
            .fold(f64::NEG_INFINITY, f64::max);
        let pers_max = finite.iter().map(|(_, p)| *p).fold(0.0_f64, f64::max);
        let birth_lo = if birth_min.is_finite() {
            birth_min
        } else {
            0.0
        };
        let birth_hi = if birth_max.is_finite() && birth_max > birth_lo {
            birth_max
        } else {
            birth_lo + 1.0
        };
        let pers_hi = if pers_max > 0.0 { pers_max } else { 1.0 };
        let res = if resolution == 0 { 1 } else { resolution };
        let bw = if bandwidth <= 0.0 { 1.0 } else { bandwidth };
        let db = (birth_hi - birth_lo) / res as f64;
        let dp = pers_hi / res as f64;
        let mut pixels = vec![vec![0.0f64; res]; res];
        for (interval, persistence) in &finite {
            let weight = persistence.powf(weight_power);
            let bx = interval.birth;
            let by = *persistence;
            for bi in 0..res {
                let bv = birth_lo + (bi as f64 + 0.5) * db;
                for pi in 0..res {
                    let pv = (pi as f64 + 0.5) * dp;
                    let dx = (bv - bx) / bw;
                    let dy = (pv - by) / bw;
                    let gauss =
                        (-0.5 * (dx * dx + dy * dy)).exp() / (2.0 * std::f64::consts::PI * bw * bw);
                    pixels[pi][bi] += weight * gauss;
                }
            }
        }
        Self {
            resolution: res,
            bandwidth: bw,
            weight_power,
            pixels,
            birth_range: (birth_lo, birth_hi),
            max_persistence: pers_hi,
        }
    }
    /// Compute the L2 distance between two persistence images (must have same resolution).
    pub fn l2_distance(&self, other: &PersistenceImage) -> f64 {
        let r = self.resolution.min(other.resolution);
        let mut sum = 0.0f64;
        for i in 0..r {
            for j in 0..r {
                let diff = self.pixels[i][j] - other.pixels[i][j];
                sum += diff * diff;
            }
        }
        sum.sqrt()
    }
    /// Flatten the image to a 1D vector (row-major).
    pub fn to_vector(&self) -> Vec<f64> {
        self.pixels
            .iter()
            .flat_map(|row| row.iter().copied())
            .collect()
    }
}
/// A contour tree: an ordered Reeb graph for simply connected domains.
#[derive(Debug, Clone)]
pub struct ContourTree {
    /// The underlying Reeb graph.
    pub reeb_graph: ReebGraph,
    /// Sorted order of nodes by filter value.
    pub sorted_nodes: Vec<usize>,
}
impl ContourTree {
    /// Build a contour tree from a Reeb graph by sorting nodes by value.
    pub fn from_reeb(reeb_graph: ReebGraph) -> Self {
        let mut sorted_nodes: Vec<usize> = (0..reeb_graph.nodes.len()).collect();
        sorted_nodes.sort_by(|&a, &b| {
            reeb_graph.nodes[a]
                .value
                .partial_cmp(&reeb_graph.nodes[b].value)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        Self {
            reeb_graph,
            sorted_nodes,
        }
    }
    /// Return the persistence of each branch (max - min).
    pub fn branch_persistence(&self) -> Vec<f64> {
        if self.reeb_graph.nodes.is_empty() {
            return vec![];
        }
        let min_val = self
            .reeb_graph
            .nodes
            .iter()
            .map(|n| n.value)
            .fold(f64::INFINITY, f64::min);
        let max_val = self
            .reeb_graph
            .nodes
            .iter()
            .map(|n| n.value)
            .fold(f64::NEG_INFINITY, f64::max);
        vec![max_val - min_val]
    }
}
/// A persistent homology representation: decomposition by bars.
#[derive(Debug, Clone)]
pub struct PersistentHomologyRepresentation {
    /// Persistence pairs organized by dimension.
    pub pairs_by_dim: HashMap<usize, Vec<PersistencePair>>,
    /// The underlying persistence diagram.
    pub diagram: PersistenceDiagram,
}
impl PersistentHomologyRepresentation {
    /// Create from a persistence diagram and pairs.
    pub fn new(diagram: PersistenceDiagram, pairs: Vec<PersistencePair>) -> Self {
        let mut pairs_by_dim: HashMap<usize, Vec<PersistencePair>> = HashMap::new();
        for pair in pairs {
            pairs_by_dim.entry(pair.dimension).or_default().push(pair);
        }
        Self {
            pairs_by_dim,
            diagram,
        }
    }
    /// Return Betti numbers at scale t.
    pub fn betti_at(&self, t: f64, max_dim: usize) -> Vec<usize> {
        (0..=max_dim)
            .map(|k| self.diagram.persistent_betti(k, t))
            .collect()
    }
}
/// An Alpha complex: sub-complex of the Delaunay triangulation.
#[derive(Debug, Clone)]
pub struct AlphaComplex {
    /// The resulting simplicial complex.
    pub complex: SimplicialComplex,
    /// The alpha parameter.
    pub alpha: f64,
}
impl AlphaComplex {
    /// Build an approximate Alpha complex (using Vietoris-Rips as a proxy here).
    pub fn build(points: &[Vec<f64>], alpha: f64) -> Self {
        let n = points.len();
        let dist: Vec<Vec<f64>> = (0..n)
            .map(|i| {
                (0..n)
                    .map(|j| {
                        points[i]
                            .iter()
                            .zip(points[j].iter())
                            .map(|(x, y)| (x - y).powi(2))
                            .sum::<f64>()
                            .sqrt()
                    })
                    .collect()
            })
            .collect();
        let vr = VietorisRipsComplex::build(&dist, alpha * 2.0, 2);
        Self {
            complex: vr.complex,
            alpha,
        }
    }
}
/// An elementary interval in one dimension: either [k, k+1] (non-degenerate) or [k, k] (degenerate).
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ElementaryInterval {
    /// Left endpoint.
    pub left: i64,
    /// True if the interval is non-degenerate: [left, left+1].
    pub non_degenerate: bool,
}
impl ElementaryInterval {
    /// Create a non-degenerate interval [left, left+1].
    pub fn non_deg(left: i64) -> Self {
        Self {
            left,
            non_degenerate: true,
        }
    }
    /// Create a degenerate interval [left, left].
    pub fn deg(left: i64) -> Self {
        Self {
            left,
            non_degenerate: false,
        }
    }
    /// The dimension (1 if non-degenerate, 0 if degenerate).
    pub fn dimension(&self) -> usize {
        if self.non_degenerate {
            1
        } else {
            0
        }
    }
}
/// A Reeb graph: quotient of the domain by connected components of level sets.
#[derive(Debug, Clone)]
pub struct ReebGraph {
    /// Nodes.
    pub nodes: Vec<ReebNode>,
    /// Edges: (from_node_id, to_node_id).
    pub edges: Vec<(usize, usize)>,
}
impl ReebGraph {
    /// Create an empty Reeb graph.
    pub fn new() -> Self {
        Self {
            nodes: vec![],
            edges: vec![],
        }
    }
    /// Add a node.
    pub fn add_node(&mut self, value: f64, node_type: ReebNodeType) -> usize {
        let id = self.nodes.len();
        self.nodes.push(ReebNode {
            id,
            value,
            node_type,
        });
        id
    }
    /// Add an edge.
    pub fn add_edge(&mut self, from: usize, to: usize) {
        self.edges.push((from, to));
    }
    /// Return all local minima.
    pub fn minima(&self) -> Vec<&ReebNode> {
        self.nodes
            .iter()
            .filter(|n| n.node_type == ReebNodeType::Minimum)
            .collect()
    }
    /// Return all local maxima.
    pub fn maxima(&self) -> Vec<&ReebNode> {
        self.nodes
            .iter()
            .filter(|n| n.node_type == ReebNodeType::Maximum)
            .collect()
    }
    /// Return all saddles.
    pub fn saddles(&self) -> Vec<&ReebNode> {
        self.nodes
            .iter()
            .filter(|n| n.node_type == ReebNodeType::Saddle)
            .collect()
    }
}
/// A filter (lens) function h: data points → ℝ.
#[derive(Debug, Clone)]
pub struct TomographicProjection {
    /// Name of the filter function.
    pub name: String,
    /// The filter values for each data point.
    pub values: Vec<f64>,
}
impl TomographicProjection {
    /// Create a projection onto a given coordinate axis.
    pub fn coordinate_projection(points: &[Vec<f64>], axis: usize) -> Self {
        let values: Vec<f64> = points
            .iter()
            .map(|p| p.get(axis).copied().unwrap_or(0.0))
            .collect();
        Self {
            name: format!("proj_{}", axis),
            values,
        }
    }
    /// Create a filter using the L2 norm.
    pub fn l2_norm(points: &[Vec<f64>]) -> Self {
        let values: Vec<f64> = points
            .iter()
            .map(|p| p.iter().map(|x| x.powi(2)).sum::<f64>().sqrt())
            .collect();
        Self {
            name: "l2_norm".to_string(),
            values,
        }
    }
    /// Minimum filter value.
    pub fn min_val(&self) -> f64 {
        self.values.iter().cloned().fold(f64::INFINITY, f64::min)
    }
    /// Maximum filter value.
    pub fn max_val(&self) -> f64 {
        self.values
            .iter()
            .cloned()
            .fold(f64::NEG_INFINITY, f64::max)
    }
}
/// TDA summary statistics for a dataset.
#[derive(Debug, Clone)]
pub struct TDASummaryStatistic {
    /// Betti numbers β₀, β₁, β₂, … at a given scale.
    pub betti_numbers: Vec<usize>,
    /// Persistence entropy of H₀, H₁, …
    pub persistence_entropies: Vec<f64>,
    /// Maximum persistence in each dimension.
    pub max_persistence: Vec<f64>,
    /// Number of bars in each dimension.
    pub num_bars: Vec<usize>,
}
impl TDASummaryStatistic {
    /// Compute summary statistics from a persistence diagram at scale `t`.
    pub fn compute(diagram: &PersistenceDiagram, t: f64, max_dim: usize) -> Self {
        let betti_numbers: Vec<usize> = (0..=max_dim)
            .map(|k| diagram.persistent_betti(k, t))
            .collect();
        let persistence_entropies: Vec<f64> = (0..=max_dim)
            .map(|k| {
                let sub_diag = PersistenceDiagram {
                    intervals: diagram.in_dimension(k).into_iter().cloned().collect(),
                };
                sub_diag.persistence_entropy()
            })
            .collect();
        let max_persistence: Vec<f64> = (0..=max_dim)
            .map(|k| {
                diagram
                    .in_dimension(k)
                    .iter()
                    .map(|i| i.persistence())
                    .filter(|p| p.is_finite())
                    .fold(0.0_f64, f64::max)
            })
            .collect();
        let num_bars: Vec<usize> = (0..=max_dim)
            .map(|k| diagram.in_dimension(k).len())
            .collect();
        Self {
            betti_numbers,
            persistence_entropies,
            max_persistence,
            num_bars,
        }
    }
}
/// A discrete Morse function: assigns a real value to each simplex such that
/// each simplex has at most one coface with smaller or equal value.
#[derive(Debug, Clone)]
pub struct DiscreteMorseFunction {
    /// Map from simplex (as vertex set) to Morse value.
    pub values: HashMap<Vec<usize>, f64>,
}
impl DiscreteMorseFunction {
    /// Create an empty discrete Morse function.
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }
    /// Assign a value to a simplex.
    pub fn set_value(&mut self, simplex: &Simplex, value: f64) {
        self.values.insert(simplex.vertices.clone(), value);
    }
    /// Get the value of a simplex (None if not assigned).
    pub fn get_value(&self, simplex: &Simplex) -> Option<f64> {
        self.values.get(&simplex.vertices).copied()
    }
}
/// A persistence diagram: multiset of (birth, death) pairs per dimension.
#[derive(Debug, Clone)]
pub struct PersistenceDiagram {
    /// The persistence intervals.
    pub intervals: Vec<PersistenceInterval>,
}
impl PersistenceDiagram {
    /// Create an empty persistence diagram.
    pub fn new() -> Self {
        Self { intervals: vec![] }
    }
    /// Add a persistence interval.
    pub fn add(&mut self, interval: PersistenceInterval) {
        self.intervals.push(interval);
    }
    /// Return all intervals in dimension `k`.
    pub fn in_dimension(&self, k: usize) -> Vec<&PersistenceInterval> {
        self.intervals.iter().filter(|i| i.dimension == k).collect()
    }
    /// Compute the k-th persistent Betti number at scale t:
    /// #{intervals [b,d) in dim k with b ≤ t < d}.
    pub fn persistent_betti(&self, k: usize, t: f64) -> usize {
        self.intervals
            .iter()
            .filter(|i| i.dimension == k && i.birth <= t && t < i.death)
            .count()
    }
    /// Compute the bottleneck distance to another diagram (same dimension).
    pub fn bottleneck_distance(&self, other: &PersistenceDiagram) -> f64 {
        let mut a: Vec<(f64, f64)> = self.intervals.iter().map(|i| (i.birth, i.death)).collect();
        let mut b: Vec<(f64, f64)> = other.intervals.iter().map(|i| (i.birth, i.death)).collect();
        while a.len() < b.len() {
            let mid = (b[a.len()].0 + b[a.len()].1) / 2.0;
            a.push((mid, mid));
        }
        while b.len() < a.len() {
            let mid = (a[b.len()].0 + a[b.len()].1) / 2.0;
            b.push((mid, mid));
        }
        a.iter()
            .zip(b.iter())
            .map(|((b1, d1), (b2, d2))| (b1 - b2).abs().max((d1 - d2).abs()))
            .fold(0.0_f64, f64::max)
    }
    /// Compute the p-Wasserstein distance to another diagram.
    pub fn wasserstein_distance(&self, other: &PersistenceDiagram, p: f64) -> f64 {
        let mut a: Vec<(f64, f64)> = self.intervals.iter().map(|i| (i.birth, i.death)).collect();
        let mut b: Vec<(f64, f64)> = other.intervals.iter().map(|i| (i.birth, i.death)).collect();
        while a.len() < b.len() {
            let mid = (b[a.len()].0 + b[a.len()].1) / 2.0;
            a.push((mid, mid));
        }
        while b.len() < a.len() {
            let mid = (a[b.len()].0 + a[b.len()].1) / 2.0;
            b.push((mid, mid));
        }
        let sum: f64 = a
            .iter()
            .zip(b.iter())
            .map(|((b1, d1), (b2, d2))| {
                let dist = (b1 - b2).abs().max((d1 - d2).abs());
                dist.powf(p)
            })
            .sum();
        sum.powf(1.0 / p)
    }
    /// Compute persistence entropy: -Σ (pᵢ/L) log(pᵢ/L) where L = Σ pᵢ.
    pub fn persistence_entropy(&self) -> f64 {
        let persistences: Vec<f64> = self
            .intervals
            .iter()
            .filter(|i| i.persistence().is_finite())
            .map(|i| i.persistence())
            .collect();
        if persistences.is_empty() {
            return 0.0;
        }
        let total: f64 = persistences.iter().sum();
        if total == 0.0 {
            return 0.0;
        }
        -persistences
            .iter()
            .map(|p| {
                let q = p / total;
                if q > 0.0 {
                    q * q.ln()
                } else {
                    0.0
                }
            })
            .sum::<f64>()
    }
}
/// A cubical complex: a finite collection of elementary cubes closed under taking faces.
#[allow(dead_code)]
pub struct CubicalComplexBuilder {
    /// All cubes in the complex.
    pub cubes: HashSet<ElementaryCubeData>,
    /// Embedding dimension.
    pub embedding_dim: usize,
}
impl CubicalComplexBuilder {
    /// Create an empty cubical complex.
    pub fn new(embedding_dim: usize) -> Self {
        Self {
            cubes: HashSet::new(),
            embedding_dim,
        }
    }
    /// Add a cube and all its faces (closure property).
    pub fn add_cube(&mut self, cube: ElementaryCubeData) {
        let faces = cube.faces();
        self.cubes.insert(cube);
        for f in faces {
            self.add_cube(f);
        }
    }
    /// Return the dimension of the complex (max cube dimension).
    pub fn dimension(&self) -> usize {
        self.cubes.iter().map(|c| c.dimension()).max().unwrap_or(0)
    }
    /// Compute the Euler characteristic: Σ_k (-1)^k * #{k-cubes}.
    pub fn euler_characteristic(&self) -> i64 {
        let mut counts: HashMap<usize, i64> = HashMap::new();
        for c in &self.cubes {
            *counts.entry(c.dimension()).or_insert(0) += 1;
        }
        counts
            .iter()
            .map(|(k, &cnt)| if k % 2 == 0 { cnt } else { -cnt })
            .sum()
    }
    /// Return all k-dimensional cubes.
    pub fn k_cubes(&self, k: usize) -> Vec<&ElementaryCubeData> {
        self.cubes.iter().filter(|c| c.dimension() == k).collect()
    }
    /// Build a cubical complex from a binary 2D image (true = filled pixel).
    pub fn from_binary_image(image: &[Vec<bool>]) -> Self {
        let rows = image.len();
        let cols = if rows > 0 { image[0].len() } else { 0 };
        let mut complex = Self::new(2);
        for i in 0..rows {
            for j in 0..cols {
                if image[i][j] {
                    let cube = ElementaryCubeData::new(vec![
                        ElementaryInterval::non_deg(i as i64),
                        ElementaryInterval::non_deg(j as i64),
                    ]);
                    complex.add_cube(cube);
                }
            }
        }
        complex
    }
}
/// Result of the Mapper algorithm.
#[derive(Debug, Clone)]
pub struct MapperResult {
    /// The computed Mapper graph.
    pub graph: MapperGraph,
    /// The cover elements used.
    pub cover: Vec<CoverElement>,
    /// The filter function values for each data point.
    pub filter_values: Vec<f64>,
}
/// A cover element: preimage of a cover interval with clustering.
#[derive(Debug, Clone)]
pub struct CoverElement {
    /// Index of the cover interval.
    pub interval_index: usize,
    /// Lower bound of the filter interval.
    pub lower: f64,
    /// Upper bound of the filter interval.
    pub upper: f64,
    /// Point indices in this preimage.
    pub points: Vec<usize>,
    /// Cluster assignment for each point (by index within `points`).
    pub clusters: Vec<usize>,
    /// Number of clusters found.
    pub num_clusters: usize,
}
impl CoverElement {
    /// Create a new cover element for a filter interval [lower, upper).
    pub fn new(interval_index: usize, lower: f64, upper: f64, points: Vec<usize>) -> Self {
        let n = points.len();
        Self {
            interval_index,
            lower,
            upper,
            points,
            clusters: vec![0; n],
            num_clusters: if n > 0 { 1 } else { 0 },
        }
    }
}
/// An elementary cube: a Cartesian product of elementary intervals.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ElementaryCubeData {
    /// The elementary intervals in each coordinate.
    pub intervals: Vec<ElementaryInterval>,
}
impl ElementaryCubeData {
    /// Create an elementary cube from a list of elementary intervals.
    pub fn new(intervals: Vec<ElementaryInterval>) -> Self {
        Self { intervals }
    }
    /// Embedding dimension (number of coordinates).
    pub fn embedding_dim(&self) -> usize {
        self.intervals.len()
    }
    /// Cube dimension (number of non-degenerate intervals).
    pub fn dimension(&self) -> usize {
        self.intervals.iter().filter(|i| i.non_degenerate).count()
    }
    /// Compute all cubical faces (co-dimension 1 faces).
    pub fn faces(&self) -> Vec<ElementaryCubeData> {
        let mut result = Vec::new();
        for (idx, interval) in self.intervals.iter().enumerate() {
            if interval.non_degenerate {
                let mut left_face = self.intervals.clone();
                left_face[idx] = ElementaryInterval::deg(interval.left);
                result.push(ElementaryCubeData::new(left_face));
                let mut right_face = self.intervals.clone();
                right_face[idx] = ElementaryInterval::deg(interval.left + 1);
                result.push(ElementaryCubeData::new(right_face));
            }
        }
        result
    }
}
/// A witness complex built from a landmark set and a witness set.
///
/// A simplex σ ⊆ L is a strong witness simplex if there exists a witness point w
/// such that for all l ∈ σ, l is among the nearest neighbors of w in L.
#[allow(dead_code)]
pub struct WitnessComplexBuilder {
    /// Landmark points (indices into the full dataset).
    pub landmarks: Vec<usize>,
    /// All data points (witnesses + landmarks).
    pub points: Vec<Vec<f64>>,
    /// Maximum dimension to compute.
    pub max_dim: usize,
    /// The resulting simplicial complex.
    pub complex: SimplicialComplex,
}
impl WitnessComplexBuilder {
    /// Build a witness complex from a dataset, landmark indices, and max dimension.
    pub fn build(points: Vec<Vec<f64>>, landmarks: Vec<usize>, max_dim: usize) -> Self {
        let n_land = landmarks.len();
        let n_pts = points.len();
        let mut complex = SimplicialComplex::new();
        for (li, _) in landmarks.iter().enumerate() {
            complex.add_simplex(Simplex::new(vec![li]));
        }
        if max_dim >= 1 {
            let mut nearest_map: Vec<usize> = vec![0; n_pts];
            for (i, pt) in points.iter().enumerate() {
                let mut best = 0;
                let mut best_d = f64::INFINITY;
                for (li, &l_idx) in landmarks.iter().enumerate() {
                    let d = tda_ext_euclidean_dist(pt, &points[l_idx]);
                    if d < best_d {
                        best_d = d;
                        best = li;
                    }
                }
                nearest_map[i] = best;
            }
            let mut witness_sets: HashMap<usize, Vec<usize>> = HashMap::new();
            for (i, _) in points.iter().enumerate() {
                witness_sets.entry(nearest_map[i]).or_default().push(i);
            }
            for li in 0..n_land {
                for lj in (li + 1)..n_land {
                    let found = points.iter().any(|pt| {
                        let mut dists: Vec<(f64, usize)> = landmarks
                            .iter()
                            .enumerate()
                            .map(|(lk, &l_idx)| (tda_ext_euclidean_dist(pt, &points[l_idx]), lk))
                            .collect();
                        dists.sort_by(|a, b| {
                            a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal)
                        });
                        dists.len() >= 2 && {
                            let top2: Vec<usize> = dists[..2].iter().map(|x| x.1).collect();
                            top2.contains(&li) && top2.contains(&lj)
                        }
                    });
                    if found {
                        complex.add_simplex(Simplex::new(vec![li, lj]));
                    }
                }
            }
        }
        Self {
            landmarks,
            points,
            max_dim,
            complex,
        }
    }
    /// Return the number of landmark points.
    pub fn num_landmarks(&self) -> usize {
        self.landmarks.len()
    }
    /// Compute the Euler characteristic of the witness complex.
    pub fn euler_characteristic(&self) -> i64 {
        self.complex.euler_characteristic()
    }
}
/// A boundary matrix in column-reduced form.
#[derive(Debug, Clone)]
pub struct ReducedBoundaryMatrix {
    /// Number of rows (k-1 simplices).
    pub num_rows: usize,
    /// Columns as sparse lists of row indices (sorted, low entry = last).
    pub columns: Vec<Vec<usize>>,
    /// Pivot column mapping: low_entry → column index.
    pub pivot_map: HashMap<usize, usize>,
}
impl ReducedBoundaryMatrix {
    /// Build a reduced boundary matrix from a dense matrix using the standard algorithm.
    pub fn reduce(matrix: &[Vec<i64>]) -> Self {
        let num_rows = matrix.len();
        let num_cols = if num_rows == 0 { 0 } else { matrix[0].len() };
        let mut columns: Vec<Vec<usize>> = (0..num_cols)
            .map(|j| {
                let mut col: Vec<usize> = (0..num_rows)
                    .filter(|&i| matrix[i][j].abs() % 2 == 1)
                    .collect();
                col.sort_unstable();
                col
            })
            .collect();
        let mut pivot_map: HashMap<usize, usize> = HashMap::new();
        for j in 0..num_cols {
            loop {
                let low = columns[j].last().copied();
                match low {
                    None => break,
                    Some(r) => {
                        if let Some(&k) = pivot_map.get(&r) {
                            let col_k = columns[k].clone();
                            let col_j = &mut columns[j];
                            let mut merged: Vec<usize> = Vec::new();
                            let mut ik = 0;
                            let mut ij = 0;
                            while ik < col_k.len() && ij < col_j.len() {
                                match col_k[ik].cmp(&col_j[ij]) {
                                    std::cmp::Ordering::Less => {
                                        merged.push(col_k[ik]);
                                        ik += 1;
                                    }
                                    std::cmp::Ordering::Greater => {
                                        merged.push(col_j[ij]);
                                        ij += 1;
                                    }
                                    std::cmp::Ordering::Equal => {
                                        ik += 1;
                                        ij += 1;
                                    }
                                }
                            }
                            merged.extend_from_slice(&col_k[ik..]);
                            merged.extend_from_slice(&col_j[ij..]);
                            columns[j] = merged;
                        } else {
                            pivot_map.insert(r, j);
                            break;
                        }
                    }
                }
            }
        }
        Self {
            num_rows,
            columns,
            pivot_map,
        }
    }
    /// Extract persistence pairs from the reduced matrix.
    pub fn persistence_pairs(&self, dim_of_col: &[usize]) -> Vec<PersistencePair> {
        let mut pairs = vec![];
        let mut killed: HashSet<usize> = HashSet::new();
        for (j, col) in self.columns.iter().enumerate() {
            if let Some(&r) = col.last() {
                let d = dim_of_col[j];
                if d > 0 {
                    pairs.push(PersistencePair::finite(d - 1, r, j));
                    killed.insert(r);
                }
            }
        }
        for j in 0..self.columns.len() {
            if self.columns[j].is_empty() && !killed.contains(&j) {
                let d = dim_of_col[j];
                pairs.push(PersistencePair::essential(d, j));
            }
        }
        pairs
    }
}
/// A gradient pair (σ, τ) where dim(τ) = dim(σ) + 1.
#[derive(Debug, Clone)]
pub struct MorsePair {
    /// The lower-dimensional simplex (face).
    pub sigma: Simplex,
    /// The higher-dimensional simplex (coface).
    pub tau: Simplex,
}
impl MorsePair {
    /// Create a Morse gradient pair.
    pub fn new(sigma: Simplex, tau: Simplex) -> Option<Self> {
        if tau.dimension() == sigma.dimension() + 1 && tau.contains_face(&sigma) {
            Some(Self { sigma, tau })
        } else {
            None
        }
    }
    /// Check validity: σ must be a face of τ.
    pub fn is_valid(&self) -> bool {
        self.tau.contains_face(&self.sigma) && self.tau.dimension() == self.sigma.dimension() + 1
    }
}
/// A persistence pair: (birth simplex, death simplex).
#[derive(Debug, Clone)]
pub struct PersistencePair {
    /// Dimension of the created homology class.
    pub dimension: usize,
    /// Index of the birth simplex in the filtration.
    pub birth_simplex: usize,
    /// Index of the death simplex in the filtration (None if essential).
    pub death_simplex: Option<usize>,
}
impl PersistencePair {
    /// Create a finite persistence pair.
    pub fn finite(dimension: usize, birth: usize, death: usize) -> Self {
        Self {
            dimension,
            birth_simplex: birth,
            death_simplex: Some(death),
        }
    }
    /// Create an essential (infinite) persistence pair.
    pub fn essential(dimension: usize, birth: usize) -> Self {
        Self {
            dimension,
            birth_simplex: birth,
            death_simplex: None,
        }
    }
    /// True if this pair is essential.
    pub fn is_essential(&self) -> bool {
        self.death_simplex.is_none()
    }
}
/// A simplex: a sorted set of vertex indices.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Simplex {
    /// Vertex indices in sorted order.
    pub vertices: Vec<usize>,
}
impl Simplex {
    /// Create a simplex from a vertex list (sorts automatically).
    pub fn new(mut vertices: Vec<usize>) -> Self {
        vertices.sort_unstable();
        vertices.dedup();
        Self { vertices }
    }
    /// Dimension of the simplex (number of vertices - 1).
    pub fn dimension(&self) -> usize {
        self.vertices.len().saturating_sub(1)
    }
    /// Compute all proper faces (all subsets of size k-1).
    pub fn faces(&self) -> Vec<Simplex> {
        let n = self.vertices.len();
        if n <= 1 {
            return vec![];
        }
        (0..n)
            .map(|i| {
                let mut verts = self.vertices.clone();
                verts.remove(i);
                Simplex::new(verts)
            })
            .collect()
    }
    /// True if `other` is a face of `self`.
    pub fn contains_face(&self, other: &Simplex) -> bool {
        other.vertices.iter().all(|v| self.vertices.contains(v))
    }
}
/// A simplicial complex: a collection of simplices closed under taking faces.
#[derive(Debug, Clone)]
pub struct SimplicialComplex {
    /// All simplices in the complex.
    pub simplices: HashSet<Simplex>,
}
impl SimplicialComplex {
    /// Create an empty simplicial complex.
    pub fn new() -> Self {
        Self {
            simplices: HashSet::new(),
        }
    }
    /// Add a simplex and all its faces (closure property).
    pub fn add_simplex(&mut self, s: Simplex) {
        let faces = s.faces();
        self.simplices.insert(s);
        for f in faces {
            self.add_simplex(f);
        }
    }
    /// Return the dimension of the complex (max simplex dimension).
    pub fn dimension(&self) -> usize {
        self.simplices
            .iter()
            .map(|s| s.dimension())
            .max()
            .unwrap_or(0)
    }
    /// Compute the Euler characteristic: Σ_k (-1)^k * #{k-simplices}.
    pub fn euler_characteristic(&self) -> i64 {
        let mut counts: HashMap<usize, i64> = HashMap::new();
        for s in &self.simplices {
            *counts.entry(s.dimension()).or_insert(0) += 1;
        }
        counts
            .iter()
            .map(|(k, &c)| if k % 2 == 0 { c } else { -c })
            .sum()
    }
    /// Return all k-simplices.
    pub fn k_simplices(&self, k: usize) -> Vec<&Simplex> {
        self.simplices
            .iter()
            .filter(|s| s.dimension() == k)
            .collect()
    }
    /// Compute the k-th boundary matrix as a Vec<Vec<i8>>.
    /// Rows = (k-1)-simplices, Columns = k-simplices, entries ∈ {-1, 0, +1}.
    pub fn boundary_matrix(&self, k: usize) -> Vec<Vec<i64>> {
        let col_simplices: Vec<&Simplex> = {
            let mut v = self.k_simplices(k);
            v.sort();
            v
        };
        if col_simplices.is_empty() || k == 0 {
            return vec![];
        }
        let row_simplices: Vec<&Simplex> = {
            let mut v = self.k_simplices(k - 1);
            v.sort();
            v
        };
        let mut mat = vec![vec![0i64; col_simplices.len()]; row_simplices.len()];
        for (j, sigma) in col_simplices.iter().enumerate() {
            for (sign_idx, face) in sigma.faces().iter().enumerate() {
                if let Some(i) = row_simplices.iter().position(|s| *s == face) {
                    mat[i][j] = if sign_idx % 2 == 0 { 1 } else { -1 };
                }
            }
        }
        mat
    }
}
/// A barcode: a collection of intervals [birth, death) for one homological dimension.
#[derive(Debug, Clone)]
pub struct BarCode {
    /// Homological dimension.
    pub dimension: usize,
    /// The bars.
    pub bars: Vec<(f64, f64)>,
}
impl BarCode {
    /// Create an empty barcode for dimension `k`.
    pub fn new(dimension: usize) -> Self {
        Self {
            dimension,
            bars: vec![],
        }
    }
    /// Add a bar [birth, death).
    pub fn add_bar(&mut self, birth: f64, death: f64) {
        self.bars.push((birth, death));
    }
    /// Return the total persistence.
    pub fn total_persistence(&self) -> f64 {
        self.bars.iter().map(|(b, d)| (d - b).abs()).sum()
    }
}
/// A Vietoris-Rips complex built from pairwise distances.
#[derive(Debug, Clone)]
pub struct VietorisRipsComplex {
    /// The resulting simplicial complex.
    pub complex: SimplicialComplex,
    /// The threshold parameter ε.
    pub epsilon: f64,
    /// Number of data points.
    pub num_points: usize,
}
impl VietorisRipsComplex {
    /// Build a Vietoris-Rips complex from a distance matrix and threshold ε.
    ///
    /// `dist[i][j]` = distance between points i and j.
    pub fn build(dist: &[Vec<f64>], epsilon: f64, max_dim: usize) -> Self {
        let n = dist.len();
        let mut complex = SimplicialComplex::new();
        for i in 0..n {
            complex.add_simplex(Simplex::new(vec![i]));
        }
        let mut edges: Vec<[usize; 2]> = vec![];
        for i in 0..n {
            for j in (i + 1)..n {
                if dist[i][j] <= epsilon {
                    complex.add_simplex(Simplex::new(vec![i, j]));
                    edges.push([i, j]);
                }
            }
        }
        if max_dim >= 2 {
            for dim in 2..=max_dim {
                let k_simplices = Self::candidate_simplices(n, dim, dist, epsilon);
                for s in k_simplices {
                    complex.add_simplex(s);
                }
            }
        }
        Self {
            complex,
            epsilon,
            num_points: n,
        }
    }
    /// Generate all candidate simplices of given dimension where all pairwise distances ≤ ε.
    fn candidate_simplices(n: usize, dim: usize, dist: &[Vec<f64>], epsilon: f64) -> Vec<Simplex> {
        let k = dim + 1;
        let mut result = vec![];
        Self::combinations(n, k, &mut vec![], &mut result, dist, epsilon);
        result
    }
    fn combinations(
        n: usize,
        k: usize,
        current: &mut Vec<usize>,
        result: &mut Vec<Simplex>,
        dist: &[Vec<f64>],
        epsilon: f64,
    ) {
        if current.len() == k {
            let all_close = current
                .iter()
                .enumerate()
                .all(|(i, &u)| current[i + 1..].iter().all(|&v| dist[u][v] <= epsilon));
            if all_close {
                result.push(Simplex::new(current.clone()));
            }
            return;
        }
        let start = current.last().map(|&x| x + 1).unwrap_or(0);
        for v in start..n {
            current.push(v);
            Self::combinations(n, k, current, result, dist, epsilon);
            current.pop();
        }
    }
}
/// The Mapper graph: nodes are (cover_element, cluster) pairs, edges from non-empty intersections.
#[derive(Debug, Clone)]
pub struct MapperGraph {
    /// Nodes: (cover_element_index, cluster_index).
    pub nodes: Vec<(usize, usize)>,
    /// Edges: pairs of node indices.
    pub edges: Vec<(usize, usize)>,
    /// Node coloring (e.g., mean filter value).
    pub node_colors: Vec<f64>,
}
impl MapperGraph {
    /// Create an empty mapper graph.
    pub fn new() -> Self {
        Self {
            nodes: vec![],
            edges: vec![],
            node_colors: vec![],
        }
    }
    /// Add a node.
    pub fn add_node(&mut self, cover_idx: usize, cluster_idx: usize, color: f64) -> usize {
        let id = self.nodes.len();
        self.nodes.push((cover_idx, cluster_idx));
        self.node_colors.push(color);
        id
    }
    /// Add an edge.
    pub fn add_edge(&mut self, u: usize, v: usize) {
        self.edges.push((u, v));
    }
    /// Return the number of connected components (using Union-Find).
    pub fn num_components(&self) -> usize {
        let n = self.nodes.len();
        if n == 0 {
            return 0;
        }
        let mut parent: Vec<usize> = (0..n).collect();
        fn find(parent: &mut Vec<usize>, x: usize) -> usize {
            if parent[x] != x {
                parent[x] = find(parent, parent[x]);
            }
            parent[x]
        }
        for &(u, v) in &self.edges {
            let pu = find(&mut parent, u);
            let pv = find(&mut parent, v);
            if pu != pv {
                parent[pu] = pv;
            }
        }
        let roots: HashSet<usize> = (0..n).map(|i| find(&mut parent, i)).collect();
        roots.len()
    }
}
/// The topological type of a Reeb graph node.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReebNodeType {
    /// Local minimum
    Minimum,
    /// Local maximum
    Maximum,
    /// Saddle point
    Saddle,
    /// Regular (pass-through) node
    Regular,
}
/// A Morse complex: critical simplices + Morse boundary operator.
#[derive(Debug, Clone)]
pub struct MorseComplex {
    /// The critical simplices.
    pub critical_simplices: Vec<FormanCriticalSimplex>,
    /// The gradient pairs.
    pub gradient_pairs: Vec<MorsePair>,
}
impl MorseComplex {
    /// Build a Morse complex from a simplicial complex and discrete Morse function.
    pub fn build(complex: &SimplicialComplex, morse_fn: &DiscreteMorseFunction) -> Self {
        let mut all_simplices: Vec<Simplex> = complex.simplices.iter().cloned().collect();
        all_simplices.sort();
        let mut paired: HashSet<Vec<usize>> = HashSet::new();
        let mut gradient_pairs: Vec<MorsePair> = vec![];
        for sigma in &all_simplices {
            if paired.contains(&sigma.vertices) {
                continue;
            }
            let best_coface = all_simplices
                .iter()
                .filter(|tau| {
                    tau.dimension() == sigma.dimension() + 1
                        && tau.contains_face(sigma)
                        && !paired.contains(&tau.vertices)
                })
                .min_by(|a, b| {
                    let va = morse_fn.get_value(a).unwrap_or(f64::INFINITY);
                    let vb = morse_fn.get_value(b).unwrap_or(f64::INFINITY);
                    va.partial_cmp(&vb).unwrap_or(std::cmp::Ordering::Equal)
                });
            if let Some(tau) = best_coface {
                let vs = morse_fn.get_value(sigma).unwrap_or(f64::INFINITY);
                let vt = morse_fn.get_value(tau).unwrap_or(f64::INFINITY);
                if vt <= vs {
                    if let Some(pair) = MorsePair::new(sigma.clone(), tau.clone()) {
                        paired.insert(sigma.vertices.clone());
                        paired.insert(tau.vertices.clone());
                        gradient_pairs.push(pair);
                    }
                }
            }
        }
        let critical_simplices: Vec<FormanCriticalSimplex> = all_simplices
            .into_iter()
            .filter(|s| !paired.contains(&s.vertices))
            .map(FormanCriticalSimplex::new)
            .collect();
        Self {
            critical_simplices,
            gradient_pairs,
        }
    }
    /// Return critical simplices of a given Morse index (dimension).
    pub fn critical_cells(&self, index: usize) -> Vec<&FormanCriticalSimplex> {
        self.critical_simplices
            .iter()
            .filter(|c| c.morse_index == index)
            .collect()
    }
    /// Morse inequality: #{critical k-cells} ≥ Betti number k.
    pub fn num_critical_cells(&self, k: usize) -> usize {
        self.critical_cells(k).len()
    }
}
/// A Forman critical simplex: a simplex not in any gradient pair.
#[derive(Debug, Clone)]
pub struct FormanCriticalSimplex {
    /// The critical simplex.
    pub simplex: Simplex,
    /// Morse index = dimension of the simplex.
    pub morse_index: usize,
}
impl FormanCriticalSimplex {
    /// Create a critical simplex.
    pub fn new(simplex: Simplex) -> Self {
        let morse_index = simplex.dimension();
        Self {
            simplex,
            morse_index,
        }
    }
}
