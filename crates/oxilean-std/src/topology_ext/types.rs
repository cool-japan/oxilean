//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use std::collections::HashMap;

/// Represents data of a fiber bundle E → B with fiber F.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FiberBundle {
    /// Name of the total space.
    pub total_space: String,
    /// Name of the base space.
    pub base_space: String,
    /// Name of the fiber.
    pub fiber: String,
    /// Whether the bundle has a section.
    pub has_section: bool,
    /// Whether the bundle is trivial.
    pub is_trivial: bool,
    /// Structure group description.
    pub structure_group: Option<String>,
}
#[allow(dead_code)]
impl FiberBundle {
    /// Creates a fiber bundle.
    pub fn new(total: &str, base: &str, fiber: &str) -> Self {
        FiberBundle {
            total_space: total.to_string(),
            base_space: base.to_string(),
            fiber: fiber.to_string(),
            has_section: false,
            is_trivial: false,
            structure_group: None,
        }
    }
    /// Sets the structure group.
    pub fn with_structure_group(mut self, g: &str) -> Self {
        self.structure_group = Some(g.to_string());
        self
    }
    /// Marks the bundle as trivial.
    pub fn trivial(mut self) -> Self {
        self.is_trivial = true;
        self.has_section = true;
        self
    }
    /// Returns the long exact sequence description.
    pub fn les_description(&self) -> String {
        format!(
            "... → π_n({}) → π_n({}) → π_n({}) → π_{{n-1}}({}) → ...",
            self.fiber, self.total_space, self.base_space, self.fiber
        )
    }
    /// Checks if the bundle is principal (fiber = structure group).
    pub fn is_principal(&self) -> bool {
        match &self.structure_group {
            Some(g) => *g == self.fiber,
            None => false,
        }
    }
}
/// A finite abstract simplicial complex.
///
/// Stores a vertex set and a list of simplices (each simplex is a sorted
/// list of vertex indices).  The structure does NOT automatically close under
/// faces — callers must add all faces explicitly if needed.
pub struct SimplicialComplex {
    /// The set of vertex indices present in the complex.
    pub vertices: Vec<usize>,
    /// Each simplex is a sorted, duplicate-free list of vertex indices.
    pub simplices: Vec<Vec<usize>>,
}
impl SimplicialComplex {
    /// Create an empty simplicial complex with no vertices or simplices.
    pub fn new() -> Self {
        SimplicialComplex {
            vertices: Vec::new(),
            simplices: Vec::new(),
        }
    }
    /// Add a simplex (given as a list of vertex indices) to the complex.
    ///
    /// The simplex is sorted and deduplicated before storage.  Any new
    /// vertices are also recorded in `self.vertices`.
    pub fn add_simplex(&mut self, mut simplex: Vec<usize>) {
        simplex.sort_unstable();
        simplex.dedup();
        for &v in &simplex {
            if !self.vertices.contains(&v) {
                self.vertices.push(v);
            }
        }
        self.vertices.sort_unstable();
        if !self.simplices.contains(&simplex) {
            self.simplices.push(simplex);
        }
    }
    /// Return the list of simplices of a given dimension `dim`.
    ///
    /// A simplex has dimension `k` when it contains exactly `k+1` vertices.
    pub(super) fn simplices_of_dim(&self, dim: usize) -> Vec<&Vec<usize>> {
        self.simplices
            .iter()
            .filter(|s| s.len() == dim + 1)
            .collect()
    }
    /// Compute the boundary matrix ∂_dim : C_dim → C_{dim-1}.
    ///
    /// Rows are indexed by (dim-1)-simplices; columns by dim-simplices.
    /// Entry (i, j) is the sign ±1 that vertex elimination gives, or 0.
    ///
    /// Returns a matrix with `rows = #{(dim-1)-simplices}` and
    /// `cols = #{dim-simplices}`.  Returns an empty matrix when `dim == 0`.
    pub fn boundary_matrix(&self, dim: usize) -> Vec<Vec<i64>> {
        if dim == 0 {
            return Vec::new();
        }
        let lower = self.simplices_of_dim(dim - 1);
        let upper = self.simplices_of_dim(dim);
        let rows = lower.len();
        let cols = upper.len();
        let mut mat = vec![vec![0i64; cols]; rows];
        for (col, sigma) in upper.iter().enumerate() {
            for i in 0..sigma.len() {
                let mut face = sigma[..i].to_vec();
                face.extend_from_slice(&sigma[i + 1..]);
                if let Some(row) = lower.iter().position(|f| **f == face) {
                    mat[row][col] = if i % 2 == 0 { 1 } else { -1 };
                }
            }
        }
        mat
    }
    /// Compute the Euler characteristic χ = ∑_k (-1)^k * #{k-simplices}.
    pub fn euler_characteristic(&self) -> i64 {
        let max_dim = self.simplices.iter().map(|s| s.len()).max().unwrap_or(0);
        let mut chi = 0i64;
        for k in 0..max_dim {
            let count = self.simplices_of_dim(k).len() as i64;
            if k % 2 == 0 {
                chi += count;
            } else {
                chi -= count;
            }
        }
        chi
    }
    /// Compute Betti numbers β_0, …, β_{max_dim}.
    ///
    /// β_k = rank(H_k) = dim(ker ∂_k) - dim(im ∂_{k+1}).
    ///
    /// Uses a simplified rank computation over ℤ (Gaussian elimination over ℚ
    /// would give exact ranks; here we use column reduction mod 2 for speed).
    pub fn betti_numbers(&self, max_dim: usize) -> Vec<usize> {
        homology_ranks(self, max_dim)
    }
}
/// A persistence diagram point (birth, death).
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub struct PersistencePoint {
    /// Birth time.
    pub birth: f64,
    /// Death time (f64::INFINITY for essential classes).
    pub death: f64,
    /// Homological dimension.
    pub dimension: usize,
}
#[allow(dead_code)]
impl PersistencePoint {
    /// Creates a new persistence point.
    pub fn new(birth: f64, death: f64, dimension: usize) -> Self {
        PersistencePoint {
            birth,
            death,
            dimension,
        }
    }
    /// Returns the persistence (lifetime) of this class.
    pub fn persistence(&self) -> f64 {
        self.death - self.birth
    }
    /// Returns true if this is an essential (infinite lifetime) class.
    pub fn is_essential(&self) -> bool {
        self.death == f64::INFINITY
    }
    /// Returns the midpoint of the persistence interval.
    pub fn midpoint(&self) -> f64 {
        if self.is_essential() {
            self.birth
        } else {
            (self.birth + self.death) / 2.0
        }
    }
}
/// A table of topological invariants computed for a simplicial complex.
#[derive(Debug, Clone)]
pub struct TopologicalInvariantTable {
    /// Number of vertices, edges, triangles, tetrahedra, …
    pub cell_counts: Vec<usize>,
    /// Euler characteristic.
    pub euler_characteristic: i64,
    /// Betti numbers β_0, β_1, …
    pub betti_numbers: Vec<usize>,
}
impl TopologicalInvariantTable {
    /// Compute the invariant table for the given simplicial complex up to
    /// the specified maximum dimension.
    pub fn compute(complex: &SimplicialComplex, max_dim: usize) -> Self {
        let cell_counts: Vec<usize> = (0..=max_dim)
            .map(|k| complex.simplices_of_dim(k).len())
            .collect();
        let euler_characteristic = complex.euler_characteristic();
        let betti_numbers = homology_ranks(complex, max_dim);
        TopologicalInvariantTable {
            cell_counts,
            euler_characteristic,
            betti_numbers,
        }
    }
    /// Pretty-print the table to a String.
    pub fn display(&self) -> String {
        let mut out = String::new();
        out.push_str("Topological Invariants\n");
        out.push_str("======================\n");
        for (k, &c) in self.cell_counts.iter().enumerate() {
            out.push_str(&format!("  #{k}-cells    = {c}\n"));
        }
        out.push_str(&format!("  χ (Euler)   = {}\n", self.euler_characteristic));
        for (k, &b) in self.betti_numbers.iter().enumerate() {
            out.push_str(&format!("  β_{k}         = {b}\n"));
        }
        out
    }
}
/// Computes characteristic class numbers for a vector bundle represented
/// as its Chern character data.
///
/// Uses the splitting principle: the total Stiefel–Whitney class (mod 2) and
/// Pontryagin class are computed from the eigenvalues of the curvature matrix.
pub struct CharacteristicClassComputer {
    /// Formal Chern roots (eigenvalues of curvature / 2πi), stored as `f64`.
    pub chern_roots: Vec<f64>,
}
impl CharacteristicClassComputer {
    /// Create from a list of formal Chern roots.
    pub fn new(chern_roots: Vec<f64>) -> Self {
        CharacteristicClassComputer { chern_roots }
    }
    /// Total Chern class c(E) = ∏(1 + x_i) expanded as elementary symmetric polynomials.
    ///
    /// Returns the coefficients c_0, c_1, …, c_n of the total Chern class.
    pub fn total_chern_class(&self) -> Vec<f64> {
        let n = self.chern_roots.len();
        let mut c = vec![0.0f64; n + 1];
        c[0] = 1.0;
        for &x in &self.chern_roots {
            for k in (1..=n).rev() {
                c[k] += x * c[k - 1];
            }
        }
        c
    }
    /// Chern character ch(E) = ∑ exp(x_i): returns partial sums up to degree `max_deg`.
    pub fn chern_character(&self, max_deg: usize) -> Vec<f64> {
        let mut ch = vec![0.0f64; max_deg + 1];
        for &x in &self.chern_roots {
            let mut power = 1.0f64;
            let mut factorial = 1.0f64;
            for k in 0..=max_deg {
                if k > 0 {
                    power *= x;
                    factorial *= k as f64;
                }
                ch[k] += power / factorial;
            }
        }
        ch
    }
    /// Euler characteristic (integral of the top Chern class over a manifold of the
    /// same dimension).  For a rank-r bundle over a 2r-manifold this is c_r.
    pub fn euler_number(&self) -> f64 {
        let c = self.total_chern_class();
        *c.last().unwrap_or(&0.0)
    }
}
/// A page of a spectral sequence E_r^{p,q}.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SpectralSequencePage {
    /// Page number r.
    pub r: usize,
    /// Data: sparse map from (p, q) to group description.
    pub data: std::collections::HashMap<(i32, i32), String>,
}
#[allow(dead_code)]
impl SpectralSequencePage {
    /// Creates a new page.
    pub fn new(r: usize) -> Self {
        SpectralSequencePage {
            r,
            data: std::collections::HashMap::new(),
        }
    }
    /// Sets E_r^{p,q} = group.
    pub fn set(&mut self, p: i32, q: i32, group: &str) {
        self.data.insert((p, q), group.to_string());
    }
    /// Returns E_r^{p,q} as a string.
    pub fn get(&self, p: i32, q: i32) -> &str {
        self.data.get(&(p, q)).map(|s| s.as_str()).unwrap_or("0")
    }
    /// Returns the total degree n entry: sum of E_r^{p,q} with p+q = n.
    pub fn total_degree(&self, n: i32) -> Vec<&str> {
        self.data
            .iter()
            .filter(|&(&(p, q), _)| p + q == n)
            .map(|(_, v)| v.as_str())
            .collect()
    }
    /// Returns the E_r differential d_r: E_r^{p,q} → E_r^{p+r, q-r+1}.
    pub fn differential_target(&self, p: i32, q: i32) -> (i32, i32) {
        (p + self.r as i32, q - self.r as i32 + 1)
    }
}
/// A ball tree for an ultrametric space (non-Archimedean metric).
///
/// In an ultrametric space every ball is both open and closed, and any two balls
/// are either disjoint or one contains the other.  This structure exploits that
/// property to build a hierarchy of balls.
pub struct UltrametricBallTree {
    /// All points as `f64` values on the p-adic "number line" (approximation).
    pub points: Vec<f64>,
    /// The prime base for the ultrametric (p-adic context).
    pub prime: u32,
}
impl UltrametricBallTree {
    /// Create a new ball tree for the given points.
    pub fn new(points: Vec<f64>, prime: u32) -> Self {
        UltrametricBallTree { points, prime }
    }
    /// p-adic valuation of a rational approximation.
    ///
    /// Returns the largest k such that p^k divides the numerator of x (after
    /// rounding to nearest integer).  Returns 0 for 0.
    pub fn padic_valuation(&self, x: f64) -> i32 {
        let n = x.round().abs() as u64;
        if n == 0 {
            return i32::MAX;
        }
        let p = self.prime as u64;
        let mut val = 0i32;
        let mut m = n;
        while m % p == 0 {
            val += 1;
            m /= p;
        }
        val
    }
    /// The ultrametric distance d(x, y) = p^{-v_p(x-y)}.
    pub fn ultrametric_dist(&self, x: f64, y: f64) -> f64 {
        let diff = x - y;
        if diff.abs() < f64::EPSILON {
            return 0.0;
        }
        let v = self.padic_valuation(diff);
        if v == i32::MAX {
            0.0
        } else {
            (self.prime as f64).powi(-v)
        }
    }
    /// Check the strong triangle inequality: d(x,z) ≤ max(d(x,y), d(y,z)).
    pub fn check_ultrametric_inequality(&self, x: f64, y: f64, z: f64) -> bool {
        let dxz = self.ultrametric_dist(x, z);
        let dxy = self.ultrametric_dist(x, y);
        let dyz = self.ultrametric_dist(y, z);
        dxz <= dxy.max(dyz) + f64::EPSILON
    }
    /// Find all points in the ball B(center, radius) under the ultrametric.
    pub fn ball(&self, center: f64, radius: f64) -> Vec<f64> {
        self.points
            .iter()
            .filter(|&&p| self.ultrametric_dist(center, p) <= radius)
            .copied()
            .collect()
    }
}
/// Represents a CW complex with cell attachments.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CWComplex {
    /// Name of the complex.
    pub name: String,
    /// Cells by dimension: cells[k] = number of k-cells.
    pub cells: Vec<u32>,
}
#[allow(dead_code)]
impl CWComplex {
    /// Creates a CW complex.
    pub fn new(name: &str) -> Self {
        CWComplex {
            name: name.to_string(),
            cells: Vec::new(),
        }
    }
    /// Adds cells in dimension k.
    pub fn add_cells(mut self, dim: usize, count: u32) -> Self {
        while self.cells.len() <= dim {
            self.cells.push(0);
        }
        self.cells[dim] += count;
        self
    }
    /// Creates S^n.
    pub fn sphere(n: usize) -> Self {
        let mut cw = CWComplex::new(&format!("S^{n}"));
        cw = cw.add_cells(0, 1);
        cw = cw.add_cells(n, 1);
        cw
    }
    /// Creates T^2 (torus).
    pub fn torus() -> Self {
        CWComplex::new("T^2")
            .add_cells(0, 1)
            .add_cells(1, 2)
            .add_cells(2, 1)
    }
    /// Creates RP^2 (real projective plane).
    pub fn rp2() -> Self {
        CWComplex::new("RP^2")
            .add_cells(0, 1)
            .add_cells(1, 1)
            .add_cells(2, 1)
    }
    /// Euler characteristic from cells (alternating sum).
    pub fn euler_characteristic(&self) -> i64 {
        self.cells
            .iter()
            .enumerate()
            .map(|(k, &c)| {
                let sign: i64 = if k % 2 == 0 { 1 } else { -1 };
                sign * c as i64
            })
            .sum()
    }
    /// Dimension of the complex.
    pub fn dimension(&self) -> Option<usize> {
        self.cells
            .iter()
            .enumerate()
            .rev()
            .find(|(_, &c)| c > 0)
            .map(|(k, _)| k)
    }
}
/// A Morse function on a CW complex (discrete version).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DiscreteMorseFunction {
    /// Number of cells in each dimension.
    pub cells: Vec<usize>,
    /// Critical cells in each dimension.
    pub critical_cells: Vec<Vec<usize>>,
    /// Gradient vector field: (cell_dim, cell_idx) -> (higher_dim, cell_idx).
    pub gradient_pairs: Vec<(usize, usize, usize, usize)>,
}
#[allow(dead_code)]
impl DiscreteMorseFunction {
    /// Creates a discrete Morse function with given cell counts.
    pub fn new(cells: Vec<usize>) -> Self {
        let n = cells.len();
        DiscreteMorseFunction {
            cells,
            critical_cells: vec![Vec::new(); n],
            gradient_pairs: Vec::new(),
        }
    }
    /// Marks cell (dim, idx) as critical.
    pub fn mark_critical(&mut self, dim: usize, idx: usize) {
        if dim < self.critical_cells.len() {
            self.critical_cells[dim].push(idx);
        }
    }
    /// Adds a gradient pair: cell of dim paired with cell of dim+1.
    pub fn add_gradient_pair(&mut self, dim: usize, lower_idx: usize, upper_idx: usize) {
        if dim + 1 < self.cells.len() {
            self.gradient_pairs
                .push((dim, lower_idx, dim + 1, upper_idx));
        }
    }
    /// Returns the number of critical cells in each dimension.
    pub fn morse_inequalities_lhs(&self) -> Vec<usize> {
        self.critical_cells.iter().map(|v| v.len()).collect()
    }
    /// Checks weak Morse inequality: #critical_k >= Betti_k.
    pub fn check_weak_morse_inequality(&self, betti: &[usize]) -> bool {
        let critical = self.morse_inequalities_lhs();
        betti
            .iter()
            .enumerate()
            .all(|(k, &b)| critical.get(k).copied().unwrap_or(0) >= b)
    }
    /// Returns the Euler characteristic from critical cells.
    pub fn euler_characteristic(&self) -> i64 {
        self.critical_cells
            .iter()
            .enumerate()
            .map(|(k, cells)| {
                let sign: i64 = if k % 2 == 0 { 1 } else { -1 };
                sign * cells.len() as i64
            })
            .sum()
    }
}
/// Checks (approximately) whether a function f : {0,…,n-1} → {0,…,m-1}
/// is uniformly continuous with respect to two finite metric spaces, on
/// a finite sample.
///
/// Uniform continuity here means: ∀ε>0, ∃δ>0 such that
/// d_X(x,y) < δ ⟹ d_Y(f(x), f(y)) < ε.
///
/// On a finite sample we just verify the condition for the supplied ε and δ.
pub struct UniformContinuityChecker<'a> {
    /// Source metric space.
    pub source: &'a MetricSpace,
    /// Target metric space.
    pub target: &'a MetricSpace,
    /// The function as a mapping from source index to target index.
    pub mapping: Vec<usize>,
}
impl<'a> UniformContinuityChecker<'a> {
    /// Create a new checker.
    pub fn new(source: &'a MetricSpace, target: &'a MetricSpace, mapping: Vec<usize>) -> Self {
        UniformContinuityChecker {
            source,
            target,
            mapping,
        }
    }
    /// Check whether `f` is uniformly continuous for the given ε and δ on the
    /// finite sample.
    pub fn is_uniformly_continuous(&self, epsilon: f64, delta: f64) -> bool {
        let n = self.source.n;
        for i in 0..n {
            for j in 0..n {
                if self.source.dist[i][j] < delta {
                    let fi = self.mapping[i];
                    let fj = self.mapping[j];
                    if self.target.dist[fi][fj] >= epsilon {
                        return false;
                    }
                }
            }
        }
        true
    }
}
/// A finite discrete metric space (for computational use).
///
/// Distances are stored as a symmetric matrix.  The main operations are
/// checking whether a sample of points looks "Cauchy" within some tolerance,
/// and computing ε-nets.
pub struct MetricSpace {
    /// Number of points.
    pub n: usize,
    /// Distance matrix (n × n), symmetric with zeros on the diagonal.
    pub dist: Vec<Vec<f64>>,
}
impl MetricSpace {
    /// Construct a new metric space from a distance matrix.
    ///
    /// Panics if `dist` is not square.
    pub fn new(dist: Vec<Vec<f64>>) -> Self {
        let n = dist.len();
        for row in &dist {
            assert_eq!(row.len(), n, "distance matrix must be square");
        }
        MetricSpace { n, dist }
    }
    /// Return the distance between points `i` and `j`.
    pub fn distance(&self, i: usize, j: usize) -> f64 {
        self.dist[i][j]
    }
    /// Check whether the whole point set is totally bounded (has a finite ε-net)
    /// by computing the greedy ε-net and checking whether every point is within ε
    /// of some net point.
    pub fn is_totally_bounded(&self, epsilon: f64) -> bool {
        if self.n == 0 {
            return true;
        }
        let net = self.greedy_epsilon_net(epsilon);
        (0..self.n).all(|i| net.iter().any(|&j| self.dist[i][j] <= epsilon))
    }
    /// Greedy construction of an ε-net: a minimal set of points such that
    /// every other point is within distance ε of some net point.
    pub fn greedy_epsilon_net(&self, epsilon: f64) -> Vec<usize> {
        let mut net: Vec<usize> = Vec::new();
        for i in 0..self.n {
            if !net.iter().any(|&j| self.dist[i][j] <= epsilon) {
                net.push(i);
            }
        }
        net
    }
    /// Heuristic "completeness" check on a finite metric space.
    ///
    /// A finite metric space is always trivially complete (every Cauchy
    /// sequence is eventually constant).  This function checks whether the
    /// space looks "ε-dense" — a proxy for completeness of a sampled space.
    pub fn looks_complete(&self, epsilon: f64) -> bool {
        self.is_totally_bounded(epsilon)
    }
}
/// Limit of a pro-object: a cofiltered inverse system of finite sets.
///
/// Stores a sequence of finite sets (levels) and projection maps between them.
/// The limit is computed as the subset of the product consistent with all projections.
pub struct ProObjectLimit {
    /// Each level is a finite set represented as `Vec<u32>`.
    pub levels: Vec<Vec<u32>>,
    /// projection[i] maps level i+1 → level i: `projection[i][j]` is the index in level i
    /// that element j of level i+1 maps to.
    pub projections: Vec<Vec<usize>>,
}
impl ProObjectLimit {
    /// Create a new pro-object from levels and projections.
    pub fn new(levels: Vec<Vec<u32>>, projections: Vec<Vec<usize>>) -> Self {
        ProObjectLimit {
            levels,
            projections,
        }
    }
    /// Compute the inverse limit: tuples (x_0, x_1, …, x_{n-1}) consistent with
    /// all projections.
    ///
    /// A tuple is consistent if for each level i: projection[i][x_{i+1}] == x_i.
    pub fn compute_limit(&self) -> Vec<Vec<usize>> {
        if self.levels.is_empty() {
            return vec![];
        }
        let mut threads: Vec<Vec<usize>> = (0..self.levels[0].len()).map(|i| vec![i]).collect();
        for (level_idx, proj) in self.projections.iter().enumerate() {
            let next_level_size = if level_idx + 1 < self.levels.len() {
                self.levels[level_idx + 1].len()
            } else {
                break;
            };
            let mut new_threads: Vec<Vec<usize>> = Vec::new();
            for j in 0..next_level_size {
                let parent = proj[j];
                for thread in &threads {
                    if *thread
                        .last()
                        .expect("threads are always non-empty: initialized with vec![i]")
                        == parent
                    {
                        let mut new_thread = thread.clone();
                        new_thread.push(j);
                        new_threads.push(new_thread);
                    }
                }
            }
            threads = new_threads;
        }
        threads
    }
    /// Return the cardinality of the computed inverse limit.
    pub fn limit_size(&self) -> usize {
        self.compute_limit().len()
    }
}
/// Checks shape equivalence of two finite CW complexes by comparing Betti numbers.
///
/// Two spaces are shape-equivalent if all their Čech cohomology groups agree.
/// For finite CW complexes this reduces to comparing homology (Betti numbers)
/// in all degrees, plus the Euler characteristic as a quick pre-check.
pub struct ShapeEquivalenceChecker {
    /// Betti numbers of the first space.
    pub betti_x: Vec<usize>,
    /// Betti numbers of the second space.
    pub betti_y: Vec<usize>,
}
impl ShapeEquivalenceChecker {
    /// Create from two simplicial complexes (compared up to `max_dim`).
    pub fn from_complexes(cx: &SimplicialComplex, cy: &SimplicialComplex, max_dim: usize) -> Self {
        ShapeEquivalenceChecker {
            betti_x: homology_ranks(cx, max_dim),
            betti_y: homology_ranks(cy, max_dim),
        }
    }
    /// Euler characteristic from a Betti number list.
    fn euler_from_betti(betti: &[usize]) -> i64 {
        betti
            .iter()
            .enumerate()
            .map(|(k, &b)| if k % 2 == 0 { b as i64 } else { -(b as i64) })
            .sum()
    }
    /// Quick necessary check: Euler characteristics must agree.
    pub fn euler_agrees(&self) -> bool {
        Self::euler_from_betti(&self.betti_x) == Self::euler_from_betti(&self.betti_y)
    }
    /// Full necessary check: all Betti numbers agree.
    pub fn betti_agree(&self) -> bool {
        let len = self.betti_x.len().max(self.betti_y.len());
        for k in 0..len {
            let bx = self.betti_x.get(k).copied().unwrap_or(0);
            let by = self.betti_y.get(k).copied().unwrap_or(0);
            if bx != by {
                return false;
            }
        }
        true
    }
    /// Return `true` if the heuristic shape-equivalence check passes.
    pub fn are_shape_equivalent(&self) -> bool {
        self.betti_agree()
    }
}
/// Represents knot invariant data.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct KnotInvariant {
    /// Name of the knot (e.g., "trefoil", "figure-eight").
    pub name: String,
    /// Alexander polynomial coefficients (by degree).
    pub alexander_poly: Vec<i64>,
    /// Jones polynomial coefficients (by degree, Laurent in q^{1/2}).
    pub jones_data: Vec<(i32, i64)>,
    /// Signature of the knot.
    pub signature: i64,
    /// Determinant: |Δ(-1)|.
    pub determinant: u64,
}
#[allow(dead_code)]
impl KnotInvariant {
    /// Creates a knot invariant record.
    pub fn new(name: &str) -> Self {
        KnotInvariant {
            name: name.to_string(),
            alexander_poly: Vec::new(),
            jones_data: Vec::new(),
            signature: 0,
            determinant: 1,
        }
    }
    /// Sets the Alexander polynomial.
    pub fn with_alexander(mut self, coeffs: Vec<i64>) -> Self {
        self.alexander_poly = coeffs;
        self
    }
    /// Sets the signature.
    pub fn with_signature(mut self, sig: i64) -> Self {
        self.signature = sig;
        self
    }
    /// Sets the determinant.
    pub fn with_determinant(mut self, det: u64) -> Self {
        self.determinant = det;
        self
    }
    /// Evaluates Alexander polynomial at t=1 (should be ±1 for a knot).
    pub fn alexander_at_one(&self) -> i64 {
        self.alexander_poly.iter().sum()
    }
    /// Returns the degree of the Alexander polynomial.
    pub fn alexander_degree(&self) -> usize {
        self.alexander_poly.len().saturating_sub(1)
    }
    /// Checks if the knot is potentially fibered (Alexander poly is monic).
    pub fn potentially_fibered(&self) -> bool {
        if let (Some(&first), Some(&last)) =
            (self.alexander_poly.first(), self.alexander_poly.last())
        {
            first.abs() == 1 && last.abs() == 1
        } else {
            false
        }
    }
}
/// A cell in a CW complex.
#[derive(Debug, Clone)]
pub struct Cell {
    /// Dimension of the cell (0 = vertex, 1 = edge, 2 = face, …).
    pub dim: usize,
    /// A human-readable label.
    pub label: String,
    /// Indices (into the cells vector) of the boundary cells.
    pub attaching_map: Vec<usize>,
}
/// A persistence diagram: a collection of (birth, death) pairs.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct PersistenceDiagram {
    /// The persistence points.
    pub points: Vec<PersistencePoint>,
}
#[allow(dead_code)]
impl PersistenceDiagram {
    /// Creates an empty persistence diagram.
    pub fn new() -> Self {
        PersistenceDiagram { points: Vec::new() }
    }
    /// Adds a persistence point.
    pub fn add(&mut self, p: PersistencePoint) {
        self.points.push(p);
    }
    /// Returns points in the given dimension.
    pub fn in_dimension(&self, dim: usize) -> Vec<&PersistencePoint> {
        self.points.iter().filter(|p| p.dimension == dim).collect()
    }
    /// Returns Betti numbers by dimension (count of essential classes).
    pub fn betti(&self, dim: usize) -> usize {
        self.points
            .iter()
            .filter(|p| p.dimension == dim && p.is_essential())
            .count()
    }
    /// Computes bottleneck distance to another diagram (approximate, greedy).
    pub fn bottleneck_distance_approx(&self, other: &PersistenceDiagram) -> f64 {
        let mut max_dist: f64 = 0.0;
        for p in &self.points {
            let d = other
                .points
                .iter()
                .map(|q| ((p.birth - q.birth).abs()).max((p.death - q.death).abs()))
                .fold(f64::INFINITY, f64::min);
            max_dist = max_dist.max(d);
        }
        max_dist
    }
    /// Returns the total persistence (sum of lifetimes for finite points).
    pub fn total_persistence(&self) -> f64 {
        self.points
            .iter()
            .filter(|p| !p.is_essential())
            .map(|p| p.persistence())
            .sum()
    }
    /// Filters points with persistence above a threshold.
    pub fn significant_features(&self, threshold: f64) -> Vec<&PersistencePoint> {
        self.points
            .iter()
            .filter(|p| p.persistence() > threshold || p.is_essential())
            .collect()
    }
}
/// Builder for a finite CW complex.
///
/// Cells are added in order; attaching maps refer to previously added cells
/// by index.
pub struct CWComplexBuilder {
    /// All cells, ordered by insertion.
    pub cells: Vec<Cell>,
}
impl CWComplexBuilder {
    /// Create an empty CW complex builder.
    pub fn new() -> Self {
        CWComplexBuilder { cells: Vec::new() }
    }
    /// Add a cell of dimension `dim` with the given label and attaching map.
    ///
    /// Returns the index of the new cell.
    pub fn add_cell(
        &mut self,
        dim: usize,
        label: impl Into<String>,
        attaching: Vec<usize>,
    ) -> usize {
        let idx = self.cells.len();
        self.cells.push(Cell {
            dim,
            label: label.into(),
            attaching_map: attaching,
        });
        idx
    }
    /// Count cells in each dimension up to `max_dim`.
    pub fn cell_counts(&self, max_dim: usize) -> Vec<usize> {
        let mut counts = vec![0usize; max_dim + 1];
        for cell in &self.cells {
            if cell.dim <= max_dim {
                counts[cell.dim] += 1;
            }
        }
        counts
    }
    /// Compute the Euler characteristic from the cell counts.
    pub fn euler_characteristic(&self) -> i64 {
        let max_dim = self.cells.iter().map(|c| c.dim).max().unwrap_or(0);
        let counts = self.cell_counts(max_dim);
        counts
            .iter()
            .enumerate()
            .map(|(k, &c)| if k % 2 == 0 { c as i64 } else { -(c as i64) })
            .sum()
    }
}
/// The Banach–Mazur game (a.k.a. Baire category game) played on the unit
/// interval [0, 1] discretised into `grid_size` equally spaced points.
///
/// Player I (the "meager" player) picks a closed interval; Player II (the
/// "dense" player) picks a sub-interval.  Player II wins if the intersection
/// of all played intervals is non-empty.
///
/// This simulation runs a fixed number of rounds where each player halves
/// the remaining interval.
#[derive(Debug)]
pub struct BaireCategoryGame {
    /// Number of grid points (resolution of the discretised interval).
    pub grid_size: usize,
    /// Current left endpoint (grid index).
    pub left: usize,
    /// Current right endpoint (grid index).
    pub right: usize,
    /// Number of rounds played so far.
    pub round: usize,
}
impl BaireCategoryGame {
    /// Start a new game on [0, grid_size-1].
    pub fn new(grid_size: usize) -> Self {
        assert!(grid_size >= 2, "grid must have at least 2 points");
        BaireCategoryGame {
            grid_size,
            left: 0,
            right: grid_size - 1,
            round: 0,
        }
    }
    /// Player I selects the left third of the remaining interval.
    pub fn player_one_move(&mut self) {
        let width = self.right - self.left;
        let new_right = self.left + width / 3;
        if new_right > self.left {
            self.right = new_right;
        }
        self.round += 1;
    }
    /// Player II selects the middle third of the remaining interval.
    pub fn player_two_move(&mut self) {
        let width = self.right - self.left;
        let third = width / 3;
        self.left += third;
        self.right = self.right.saturating_sub(third);
        self.round += 1;
    }
    /// Check whether the current interval is non-empty (Player II is still winning).
    pub fn player_two_winning(&self) -> bool {
        self.left <= self.right
    }
    /// Run `rounds` pairs of moves (I then II) and return whether Player II wins.
    pub fn simulate(&mut self, rounds: usize) -> bool {
        for _ in 0..rounds {
            self.player_one_move();
            if !self.player_two_winning() {
                return false;
            }
            self.player_two_move();
            if !self.player_two_winning() {
                return false;
            }
        }
        self.player_two_winning()
    }
}
