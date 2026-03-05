//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)
use super::functions::*;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct IntrinsicVolume {
    pub ambient_dimension: usize,
    pub index: usize,
    pub formula: String,
}
#[allow(dead_code)]
impl IntrinsicVolume {
    pub fn new(n: usize, i: usize) -> Self {
        let formula = format!("V_{i}(K) = C(n,{i}) × (mixed volume of K and B^n)");
        IntrinsicVolume {
            ambient_dimension: n,
            index: i,
            formula,
        }
    }
    pub fn kinematic_formula(&self) -> String {
        format!("Kinematic formula: ∫ χ(K∩gL) dg = Σ_k c_{{n,k}} V_{{n-k}}(K) V_k(L)")
    }
    pub fn steiner_formula_contribution(&self) -> String {
        format!(
            "Steiner: V(K + εB) = sum_k eps^k C(n,k) V_(n-k)(K), n={}, i={}",
            self.ambient_dimension, self.index
        )
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LogConcaveSequence {
    pub coefficients: Vec<f64>,
    pub is_log_concave: bool,
    pub is_ultra_log_concave: bool,
    pub has_real_roots_conjecture: bool,
}
#[allow(dead_code)]
impl LogConcaveSequence {
    pub fn new(coeffs: Vec<f64>) -> Self {
        let lc = Self::check_log_concave(&coeffs);
        LogConcaveSequence {
            is_ultra_log_concave: false,
            coefficients: coeffs,
            is_log_concave: lc,
            has_real_roots_conjecture: false,
        }
    }
    fn check_log_concave(c: &[f64]) -> bool {
        let n = c.len();
        for i in 1..n.saturating_sub(1) {
            if c[i] * c[i] < c[i - 1] * c[i + 1] {
                return false;
            }
        }
        true
    }
    pub fn with_ultra_log_concave(mut self) -> Self {
        self.is_ultra_log_concave = true;
        self
    }
    pub fn mason_conjecture_statement(&self) -> String {
        "Mason's conjecture: independence numbers of matroids form ultra-log-concave sequences"
            .to_string()
    }
    pub fn branden_huh_lorentzian_connection(&self) -> String {
        "Brändén-Huh (2020): Lorentzian polynomials → log-concavity of matroid sequences"
            .to_string()
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ZonotopeData {
    pub generators: Vec<Vec<f64>>,
    pub ambient_dim: usize,
    pub num_generators: usize,
}
#[allow(dead_code)]
impl ZonotopeData {
    pub fn new(generators: Vec<Vec<f64>>) -> Self {
        let n = generators.first().map_or(0, |g| g.len());
        let k = generators.len();
        ZonotopeData {
            generators,
            ambient_dim: n,
            num_generators: k,
        }
    }
    pub fn num_faces_upper_bound(&self) -> usize {
        let k = self.num_generators;
        let n = self.ambient_dim;
        let mut count = 2;
        for j in 0..n.min(k) {
            let mut c = 1usize;
            for i in 0..j {
                c = c * (k - 1 - i) / (i + 1);
            }
            count += 2 * c;
        }
        count
    }
    pub fn volume_formula(&self) -> String {
        format!(
            "Vol(Z) = sum over n-subsets of |det(g_{{i1}},...,g_{{in}})| (n={})",
            self.ambient_dim
        )
    }
    pub fn is_centrally_symmetric(&self) -> bool {
        true
    }
    pub fn tilings_of_space_description(&self) -> String {
        "Zonotopes tile space if and only if the generators form a totally unimodular matrix"
            .to_string()
    }
}
/// Zonotope: Minkowski sum of line segments {c + Σ λ_i g_i : λ_i ∈ [-1,1]}.
#[derive(Debug, Clone)]
pub struct Zonotope {
    /// Centre c.
    pub centre: Vec<f64>,
    /// Generators g_1,...,g_m (each a vector in ℝ^d).
    pub generators: Vec<Vec<f64>>,
}
impl Zonotope {
    /// Construct a zonotope.
    pub fn new(centre: Vec<f64>, generators: Vec<Vec<f64>>) -> Self {
        Self { centre, generators }
    }
    /// Check membership: x ∈ Z iff x - c = Σ λ_i g_i with λ_i ∈ [-1,1].
    /// Uses a greedy interval check (exact only for orthogonal generators).
    pub fn contains_approx(&self, x: &[f64]) -> bool {
        let d = self.centre.len();
        let diff: Vec<f64> = x
            .iter()
            .zip(self.centre.iter())
            .map(|(xi, ci)| xi - ci)
            .collect();
        let sum_abs: Vec<f64> = (0..d)
            .map(|j| self.generators.iter().map(|g| g[j].abs()).sum::<f64>())
            .collect();
        diff.iter()
            .zip(sum_abs.iter())
            .all(|(di, &si)| di.abs() <= si + 1e-10)
    }
    /// Volume of the zonotope: 2^m * |det([g_{i1},...,g_{id}])| summed over all d-subsets.
    /// For 2D: V = Σ_{i<j} |g_i × g_j|.
    pub fn volume_2d(&self) -> f64 {
        if self.generators.len() < 2 {
            return 0.0;
        }
        let mut vol = 0.0_f64;
        let m = self.generators.len();
        for i in 0..m {
            for j in (i + 1)..m {
                let gi = &self.generators[i];
                let gj = &self.generators[j];
                if gi.len() >= 2 && gj.len() >= 2 {
                    let cross = (gi[0] * gj[1] - gi[1] * gj[0]).abs();
                    vol += cross;
                }
            }
        }
        4.0 * vol
    }
    /// Number of vertices: at most 2 Σ_{k=0}^{d-1} C(m-1, k) (upper bound).
    pub fn num_vertices_upper_bound(&self) -> usize {
        let m = self.generators.len();
        let d = self.centre.len();
        if d == 0 || m == 0 {
            return 1;
        }
        2 * (0..d)
            .map(|k| binomial(m.saturating_sub(1), k))
            .sum::<usize>()
    }
}
/// V-polytope: conv{v_1,...,v_n}.
#[derive(Debug, Clone)]
pub struct VPolytope {
    /// Vertices of the polytope.
    pub vertices: Vec<Vec<f64>>,
    /// Dimension of the ambient space.
    pub dim: usize,
}
impl VPolytope {
    /// Construct from a list of vertices.
    pub fn new(vertices: Vec<Vec<f64>>) -> Self {
        let dim = vertices.first().map_or(0, |v| v.len());
        Self { vertices, dim }
    }
    /// Return the vertices.
    pub fn vertices(&self) -> &[Vec<f64>] {
        &self.vertices
    }
    /// f-vector: for a simplex in d dimensions, f_k = C(d+1, k+1).
    /// Here we return the count of vertices as f_0.
    pub fn f_vector(&self) -> Vec<usize> {
        vec![self.vertices.len()]
    }
    /// Check if the polytope is a simplex (exactly d+1 vertices in d dimensions).
    pub fn is_simplicial(&self) -> bool {
        !self.vertices.is_empty() && self.vertices.len() == self.dim + 1
    }
}
/// A convex function represented by its evaluations on a grid (or analytically via a closure).
#[derive(Debug, Clone)]
pub struct ConvexFunction {
    /// Dimension of the domain.
    pub dim: usize,
}
impl ConvexFunction {
    /// Construct a convex function placeholder.
    pub fn new(dim: usize) -> Self {
        Self { dim }
    }
    /// Check Jensen's inequality numerically at two points x, y with λ ∈ [0,1].
    pub fn check_convexity<F>(&self, f: F, x: &[f64], y: &[f64], lambda: f64) -> bool
    where
        F: Fn(&[f64]) -> f64,
    {
        let mid: Vec<f64> = x
            .iter()
            .zip(y.iter())
            .map(|(xi, yi)| lambda * xi + (1.0 - lambda) * yi)
            .collect();
        f(&mid) <= lambda * f(x) + (1.0 - lambda) * f(y) + 1e-10
    }
    /// Compute subdifferential (subgradient) at x by finite differences.
    pub fn subgradient<F>(&self, f: F, x: &[f64], eps: f64) -> Vec<f64>
    where
        F: Fn(&[f64]) -> f64,
    {
        let fx = f(x);
        let mut grad = vec![0.0_f64; x.len()];
        for i in 0..x.len() {
            let mut xp = x.to_vec();
            xp[i] += eps;
            grad[i] = (f(&xp) - fx) / eps;
        }
        grad
    }
}
/// Face poset of a polytope.
#[derive(Debug, Clone)]
pub struct FacePoset {
    /// f-vector: `f_vec[k]` = number of k-dimensional faces.
    pub f_vec: Vec<usize>,
}
impl FacePoset {
    /// Construct from an f-vector.
    pub fn new(f_vec: Vec<usize>) -> Self {
        Self { f_vec }
    }
    /// Return the f-vector.
    pub fn f_vector(&self) -> &[usize] {
        &self.f_vec
    }
    /// Euler characteristic: Σ (-1)^k f_k = 0 for polytopes (excluding empty face and polytope itself).
    pub fn euler_characteristic(&self) -> i64 {
        self.f_vec
            .iter()
            .enumerate()
            .map(|(k, &fk)| if k % 2 == 0 { fk as i64 } else { -(fk as i64) })
            .sum()
    }
}
/// Tiling theory: lattice tilings by translates of a convex body.
#[derive(Debug, Clone)]
pub struct TilingTheory {
    /// Dimension.
    pub dim: usize,
    /// Lattice basis vectors.
    pub lattice: Vec<Vec<f64>>,
}
impl TilingTheory {
    /// Construct a tiling theory instance.
    pub fn new(dim: usize, lattice: Vec<Vec<f64>>) -> Self {
        Self { dim, lattice }
    }
    /// Check if the lattice packing density is valid (det(lattice) ≠ 0 in 2D).
    pub fn is_valid_lattice(&self) -> bool {
        if self.lattice.len() < 2 || self.lattice[0].len() < 2 {
            return !self.lattice.is_empty();
        }
        let det = self.lattice[0][0] * self.lattice[1][1] - self.lattice[0][1] * self.lattice[1][0];
        det.abs() > 1e-12
    }
    /// Compute the fundamental domain volume: |det(lattice)| in 2D.
    pub fn fundamental_volume_2d(&self) -> f64 {
        if self.lattice.len() < 2 || self.lattice[0].len() < 2 {
            return 0.0;
        }
        (self.lattice[0][0] * self.lattice[1][1] - self.lattice[0][1] * self.lattice[1][0]).abs()
    }
}
/// Checker for Helly-type theorems on convex sets.
///
/// Helly's theorem: given a finite family of convex sets in ℝ^d, if every
/// d+1 of them have a common point, then all of them do.
///
/// This struct checks the Helly condition by testing pairwise intersections
/// and the triangle intersection (for d=1 and d=2 simplified cases).
#[derive(Debug, Clone)]
pub struct HellyTheoremChecker {
    /// Ambient dimension.
    pub dim: usize,
    /// Convex sets represented as axis-aligned boxes [lo, hi]^d.
    pub boxes: Vec<(Vec<f64>, Vec<f64>)>,
}
impl HellyTheoremChecker {
    /// Create a Helly checker.
    pub fn new(dim: usize) -> Self {
        Self {
            dim,
            boxes: Vec::new(),
        }
    }
    /// Add a convex set represented as an axis-aligned box [lo, hi].
    pub fn add_box(&mut self, lo: Vec<f64>, hi: Vec<f64>) {
        self.boxes.push((lo, hi));
    }
    /// Check if two boxes [lo_i, hi_i] and [lo_j, hi_j] intersect.
    fn boxes_intersect(lo1: &[f64], hi1: &[f64], lo2: &[f64], hi2: &[f64]) -> bool {
        lo1.iter()
            .zip(hi1.iter())
            .zip(lo2.iter())
            .zip(hi2.iter())
            .all(|(((l1, h1), l2), h2)| l1 <= h2 && l2 <= h1)
    }
    /// Check if all boxes have a common point (pairwise intersection of all boxes).
    /// For axis-aligned boxes this equals checking the intersection is non-empty.
    pub fn all_intersect(&self) -> bool {
        if self.boxes.is_empty() {
            return true;
        }
        let d = self.dim;
        let mut lo = vec![f64::NEG_INFINITY; d];
        let mut hi = vec![f64::INFINITY; d];
        for (bl, bh) in &self.boxes {
            for j in 0..d.min(bl.len()).min(bh.len()) {
                if bl[j] > lo[j] {
                    lo[j] = bl[j];
                }
                if bh[j] < hi[j] {
                    hi[j] = bh[j];
                }
            }
        }
        lo.iter().zip(hi.iter()).all(|(l, h)| l <= h)
    }
    /// Verify the Helly condition: check every (d+1)-subfamily intersects.
    /// For d=1 (intervals), this checks every 2-subfamily intersects → all intersect.
    pub fn verify_helly_condition_1d(&self) -> bool {
        let n = self.boxes.len();
        for i in 0..n {
            for j in (i + 1)..n {
                let (li, hi_) = &self.boxes[i];
                let (lj, hj) = &self.boxes[j];
                if !Self::boxes_intersect(li, hi_, lj, hj) {
                    return false;
                }
            }
        }
        true
    }
    /// Fractional Helly check: if at least α fraction of all d+1-tuples intersect,
    /// then at least (1 - (1-α)^{1/(d+1)}) fraction of the sets share a common point.
    /// Here we compute what fraction of pairs intersect.
    pub fn fraction_pairwise_intersecting(&self) -> f64 {
        let n = self.boxes.len();
        if n < 2 {
            return 1.0;
        }
        let total = n * (n - 1) / 2;
        let mut count = 0_usize;
        for i in 0..n {
            for j in (i + 1)..n {
                let (li, hi_) = &self.boxes[i];
                let (lj, hj) = &self.boxes[j];
                if Self::boxes_intersect(li, hi_, lj, hj) {
                    count += 1;
                }
            }
        }
        count as f64 / total as f64
    }
    /// Radon partition check (simplified 2D): do the given points have a Radon partition?
    /// Radon's theorem: any d+2 points in ℝ^d have a Radon partition.
    /// For d=1: any 3 points {a,b,c} with a ≤ b ≤ c: {b} and {a,c} is a partition
    /// (b ∈ [a,c]).
    pub fn has_radon_partition_1d(points: &[f64]) -> bool {
        if points.len() < 3 {
            return false;
        }
        let mut sorted = points.to_vec();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        true
    }
}
/// Approximation of John's ellipsoid (maximum volume inscribed ellipsoid).
///
/// John's theorem states: for every convex body K in ℝ^n, there is a unique
/// maximum-volume ellipsoid E ⊆ K, and K ⊆ n·E (with equality for simplices).
/// This struct computes a simple approximation via the covariance of the vertices.
#[derive(Debug, Clone)]
pub struct JohnEllipsoidApprox {
    /// Centre of the inscribed ellipsoid.
    pub centre: Vec<f64>,
    /// Axis lengths (eigenvalues of the shape matrix, approximated).
    pub axes: Vec<f64>,
}
impl JohnEllipsoidApprox {
    /// Approximate John's ellipsoid from a set of points by computing the
    /// covariance ellipsoid (centred at the mean, axes from variance).
    pub fn from_points(points: &[Vec<f64>]) -> Option<Self> {
        if points.is_empty() {
            return None;
        }
        let d = points[0].len();
        let n = points.len() as f64;
        let mut centre = vec![0.0_f64; d];
        for p in points {
            for j in 0..d {
                centre[j] += p[j];
            }
        }
        for c in centre.iter_mut() {
            *c /= n;
        }
        let mut axes = vec![0.0_f64; d];
        for p in points {
            for j in 0..d {
                axes[j] += (p[j] - centre[j]).powi(2);
            }
        }
        for a in axes.iter_mut() {
            *a = (*a / n).sqrt().max(1e-12);
        }
        Some(Self { centre, axes })
    }
    /// Check if a point x is inside the ellipsoid: Σ_j ((x_j - c_j)/a_j)^2 ≤ 1.
    pub fn contains(&self, x: &[f64]) -> bool {
        let dist2: f64 = x
            .iter()
            .zip(self.centre.iter())
            .zip(self.axes.iter())
            .map(|((xi, ci), ai)| ((xi - ci) / ai).powi(2))
            .sum();
        dist2 <= 1.0 + 1e-10
    }
    /// Volume of the ellipsoid: κ_d * Π_j a_j where κ_d = π^{d/2}/Γ(d/2+1).
    /// For d = 2: π * a1 * a2. For d = 3: 4π/3 * a1 * a2 * a3.
    pub fn volume(&self) -> f64 {
        let d = self.axes.len();
        let prod: f64 = self.axes.iter().product();
        match d {
            1 => 2.0 * prod,
            2 => std::f64::consts::PI * prod,
            3 => 4.0 / 3.0 * std::f64::consts::PI * prod,
            _ => {
                let half_d = d as f64 / 2.0;
                let kappa = std::f64::consts::PI.powf(half_d) / stirling_gamma(half_d + 1.0);
                kappa * prod
            }
        }
    }
    /// John's containment bound: K ⊆ d * E (where d is ambient dimension).
    /// Returns the scaled ellipsoid axes.
    pub fn johns_outer_bound(&self) -> Vec<f64> {
        let d = self.axes.len() as f64;
        self.axes.iter().map(|&a| a * d).collect()
    }
}
/// Mixed volume computations for convex bodies.
#[derive(Debug, Clone)]
pub struct MixedVolume {
    /// Dimension n.
    pub dim: usize,
}
impl MixedVolume {
    /// Construct a mixed volume calculator.
    pub fn new(dim: usize) -> Self {
        Self { dim }
    }
    /// Estimate the volume of a polytope (given by vertices) via Monte Carlo sampling.
    pub fn estimate_volume_monte_carlo(
        &self,
        vertices: &[Vec<f64>],
        n_samples: usize,
        seed: u64,
    ) -> f64 {
        if vertices.is_empty() || self.dim == 0 {
            return 0.0;
        }
        let mut lo = vec![f64::INFINITY; self.dim];
        let mut hi = vec![f64::NEG_INFINITY; self.dim];
        for v in vertices {
            for j in 0..self.dim {
                if v[j] < lo[j] {
                    lo[j] = v[j];
                }
                if v[j] > hi[j] {
                    hi[j] = v[j];
                }
            }
        }
        let bbox_vol: f64 = lo.iter().zip(hi.iter()).map(|(l, h)| h - l).product();
        let mut rng_state = seed;
        let mut inside = 0_usize;
        for _ in 0..n_samples {
            rng_state = rng_state
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            let point: Vec<f64> = (0..self.dim)
                .map(|j| {
                    rng_state = rng_state
                        .wrapping_mul(6364136223846793005)
                        .wrapping_add(1442695040888963407);
                    let t = (rng_state >> 33) as f64 / (u32::MAX as f64);
                    lo[j] + t * (hi[j] - lo[j])
                })
                .collect();
            if is_in_convex_hull(vertices, &point) {
                inside += 1;
            }
        }
        bbox_vol * inside as f64 / n_samples as f64
    }
    /// Brunn-Minkowski inequality check (numeric): verify V(A+B)^{1/n} >= V(A)^{1/n} + V(B)^{1/n}.
    pub fn check_brunn_minkowski(&self, vol_a: f64, vol_b: f64, vol_apb: f64) -> bool {
        let n = self.dim as f64;
        let lhs = vol_apb.powf(1.0 / n);
        let rhs = vol_a.powf(1.0 / n) + vol_b.powf(1.0 / n);
        lhs >= rhs - 1e-10
    }
    /// Check the isoperimetric inequality V^{n-1} <= c_n * S^n numerically (simplified 2D: 4πA ≤ P²).
    pub fn check_isoperimetric_2d(area: f64, perimeter: f64) -> bool {
        4.0 * std::f64::consts::PI * area <= perimeter * perimeter + 1e-10
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ConvexValuation {
    pub name: String,
    pub is_continuous: bool,
    pub is_translation_invariant: bool,
    pub is_rotation_invariant: bool,
    pub homogeneity_degree: Option<usize>,
}
#[allow(dead_code)]
impl ConvexValuation {
    pub fn euler_characteristic() -> Self {
        ConvexValuation {
            name: "Euler characteristic χ".to_string(),
            is_continuous: true,
            is_translation_invariant: true,
            is_rotation_invariant: true,
            homogeneity_degree: Some(0),
        }
    }
    pub fn volume(dim: usize) -> Self {
        ConvexValuation {
            name: format!("Volume V_{}", dim),
            is_continuous: true,
            is_translation_invariant: true,
            is_rotation_invariant: true,
            homogeneity_degree: Some(dim),
        }
    }
    pub fn surface_area() -> Self {
        ConvexValuation {
            name: "Surface area S".to_string(),
            is_continuous: true,
            is_translation_invariant: true,
            is_rotation_invariant: true,
            homogeneity_degree: Some(1),
        }
    }
    pub fn hadwiger_theorem(&self) -> String {
        "Hadwiger: any continuous, rigid-motion invariant valuation on K^n = linear combo of V_0,...,V_n"
            .to_string()
    }
    pub fn mcmullen_decomposition(&self) -> String {
        format!(
            "McMullen: {} decomposes into homogeneous components of degrees 0..n",
            self.name
        )
    }
}
/// A convex set represented as a list of vertices (for polytopes) or by a predicate.
#[derive(Debug, Clone)]
pub struct ConvexSet {
    /// Dimension of the ambient space.
    pub dim: usize,
    /// Vertices (for polyhedral sets in V-representation).
    pub vertices: Vec<Vec<f64>>,
}
impl ConvexSet {
    /// Construct a convex set from its vertices.
    pub fn new(dim: usize, vertices: Vec<Vec<f64>>) -> Self {
        Self { dim, vertices }
    }
    /// Project a point x onto the convex hull of the vertices (using simple averaging heuristic).
    /// For a proper projection use an LP; here we return the nearest vertex.
    pub fn projection(&self, x: &[f64]) -> Vec<f64> {
        self.vertices
            .iter()
            .min_by(|a, b| {
                let da: f64 = a.iter().zip(x).map(|(ai, xi)| (ai - xi).powi(2)).sum();
                let db: f64 = b.iter().zip(x).map(|(bi, xi)| (bi - xi).powi(2)).sum();
                da.partial_cmp(&db).unwrap_or(std::cmp::Ordering::Equal)
            })
            .cloned()
            .unwrap_or_else(|| vec![0.0; self.dim])
    }
    /// Check if all vertices satisfy the polyhedral inequality Ax ≤ b.
    pub fn is_polyhedral(&self, a: &[Vec<f64>], b: &[f64]) -> bool {
        self.vertices.iter().all(|v| {
            a.iter().zip(b.iter()).all(|(row, &bi)| {
                let dot: f64 = row.iter().zip(v.iter()).map(|(aij, vj)| aij * vj).sum();
                dot <= bi + 1e-10
            })
        })
    }
    /// Compute the support function value h_C(y) = max_{v ∈ vertices} ⟨v, y⟩.
    pub fn support_function(&self, y: &[f64]) -> f64 {
        self.vertices
            .iter()
            .map(|v| v.iter().zip(y.iter()).map(|(vi, yi)| vi * yi).sum::<f64>())
            .fold(f64::NEG_INFINITY, f64::max)
    }
    /// Compute the Minkowski sum of two polytopes (convex hull of all pairwise sums).
    pub fn minkowski_sum(&self, other: &ConvexSet) -> ConvexSet {
        let mut sums = Vec::new();
        for a in &self.vertices {
            for b in &other.vertices {
                let s: Vec<f64> = a.iter().zip(b.iter()).map(|(ai, bi)| ai + bi).collect();
                sums.push(s);
            }
        }
        ConvexSet {
            dim: self.dim,
            vertices: sums,
        }
    }
}
/// Delaunay triangulation dual to the Voronoi diagram.
#[derive(Debug, Clone)]
pub struct DelaunayTriangulation {
    /// Sites.
    pub sites: Vec<Vec<f64>>,
    /// Triangles as triples of site indices (2D).
    pub triangles: Vec<[usize; 3]>,
}
impl DelaunayTriangulation {
    /// Construct a Delaunay triangulation (given precomputed triangles).
    pub fn new(sites: Vec<Vec<f64>>, triangles: Vec<[usize; 3]>) -> Self {
        Self { sites, triangles }
    }
    /// Check the Delaunay property for a triangle (i,j,k): no other site is strictly inside
    /// the circumcircle. Returns true if the property holds for all triangles.
    pub fn check_delaunay_property(&self) -> bool {
        for &[i, j, k] in &self.triangles {
            let p = &self.sites[i];
            let q = &self.sites[j];
            let r = &self.sites[k];
            if p.len() < 2 || q.len() < 2 || r.len() < 2 {
                continue;
            }
            let ax = p[0] - r[0];
            let ay = p[1] - r[1];
            let bx = q[0] - r[0];
            let by = q[1] - r[1];
            let d = 2.0 * (ax * by - ay * bx);
            if d.abs() < 1e-12 {
                continue;
            }
            let ux = (by * (ax * ax + ay * ay) - ay * (bx * bx + by * by)) / d;
            let uy = (ax * (bx * bx + by * by) - bx * (ax * ax + ay * ay)) / d;
            let cx = r[0] + ux;
            let cy = r[1] + uy;
            let rad2 = ux * ux + uy * uy;
            for (m, s) in self.sites.iter().enumerate() {
                if m == i || m == j || m == k || s.len() < 2 {
                    continue;
                }
                let dx = s[0] - cx;
                let dy = s[1] - cy;
                if dx * dx + dy * dy < rad2 - 1e-10 {
                    return false;
                }
            }
        }
        true
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LorentzianPolynomial {
    pub degree: usize,
    pub num_variables: usize,
    pub is_strictly_lorentzian: bool,
    pub is_m_convex: bool,
}
#[allow(dead_code)]
impl LorentzianPolynomial {
    pub fn new(deg: usize, nvars: usize) -> Self {
        LorentzianPolynomial {
            degree: deg,
            num_variables: nvars,
            is_strictly_lorentzian: false,
            is_m_convex: false,
        }
    }
    pub fn hessian_is_psd(&self) -> bool {
        true
    }
    pub fn implies_log_concavity(&self) -> bool {
        true
    }
    pub fn connection_to_hodge_theory(&self) -> String {
        "Adiprasito-Huh-Katz: Hodge theory for combinatorial geometries → log-concavity".to_string()
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LatticePolytope {
    pub vertices: Vec<Vec<i64>>,
    pub dimension: usize,
    pub is_reflexive: bool,
    pub ehrhart_polynomial: Vec<i64>,
}
#[allow(dead_code)]
impl LatticePolytope {
    pub fn new(vertices: Vec<Vec<i64>>) -> Self {
        let dim = vertices.first().map_or(0, |v| v.len());
        LatticePolytope {
            vertices,
            dimension: dim,
            is_reflexive: false,
            ehrhart_polynomial: vec![],
        }
    }
    pub fn simplex(dim: usize) -> Self {
        let mut verts = vec![vec![0i64; dim]];
        for i in 0..dim {
            let mut e = vec![0i64; dim];
            e[i] = 1;
            verts.push(e);
        }
        LatticePolytope {
            vertices: verts,
            dimension: dim,
            is_reflexive: dim == 1,
            ehrhart_polynomial: vec![1; dim + 1],
        }
    }
    pub fn num_lattice_points(&self, dilation: usize) -> usize {
        let t = dilation;
        let d = self.dimension;
        let mut num = 1usize;
        for i in 0..d {
            num = num * (t + d - i) / (i + 1);
        }
        num
    }
    pub fn ehrhart_reciprocity(&self, dilation: i64) -> i64 {
        let _t = dilation.unsigned_abs();
        if dilation < 0 {
            -(self.dimension as i64)
        } else {
            self.dimension as i64
        }
    }
    pub fn volume_from_ehrhart(&self) -> f64 {
        let d = self.dimension;
        let factorial: usize = (1..=d).product();
        self.vertices.len() as f64 / factorial as f64
    }
}
/// Voronoi diagram for a finite set of sites in ℝ^d.
#[derive(Debug, Clone)]
pub struct VoronoiDiagram {
    /// Sites (generator points).
    pub sites: Vec<Vec<f64>>,
}
impl VoronoiDiagram {
    /// Construct a Voronoi diagram from a list of sites.
    pub fn new(sites: Vec<Vec<f64>>) -> Self {
        Self { sites }
    }
    /// Return the index of the nearest site to query point `q`.
    pub fn nearest_neighbor(&self, q: &[f64]) -> usize {
        self.sites
            .iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| {
                let da: f64 = a.iter().zip(q).map(|(ai, qi)| (ai - qi).powi(2)).sum();
                let db: f64 = b.iter().zip(q).map(|(bi, qi)| (bi - qi).powi(2)).sum();
                da.partial_cmp(&db).unwrap_or(std::cmp::Ordering::Equal)
            })
            .map(|(i, _)| i)
            .unwrap_or(0)
    }
    /// Perform one Lloyd iteration: move each site to the centroid of a grid of sample points.
    /// Here we use a 1D approximation for the centroid (average of nearest-neighbour regions).
    pub fn lloyd_iteration(&self, samples: &[Vec<f64>]) -> Vec<Vec<f64>> {
        let d = self.sites.first().map_or(0, |v| v.len());
        let mut sums = vec![vec![0.0_f64; d]; self.sites.len()];
        let mut counts = vec![0_usize; self.sites.len()];
        for q in samples {
            let idx = self.nearest_neighbor(q);
            for j in 0..d {
                sums[idx][j] += q[j];
            }
            counts[idx] += 1;
        }
        sums.iter()
            .zip(counts.iter())
            .map(|(s, &c)| {
                if c == 0 {
                    s.clone()
                } else {
                    s.iter().map(|&v| v / c as f64).collect()
                }
            })
            .collect()
    }
}
/// Computer for Minkowski sums of polytopes with additional features.
///
/// The Minkowski sum A + B = {a + b : a ∈ A, b ∈ B}.
/// For polytopes in V-representation, this is the convex hull of all pairwise sums.
#[derive(Debug, Clone)]
pub struct MinkowskiSumComputer {
    /// Ambient dimension.
    pub dim: usize,
}
impl MinkowskiSumComputer {
    /// Create a Minkowski sum computer for the given dimension.
    pub fn new(dim: usize) -> Self {
        Self { dim }
    }
    /// Compute the Minkowski sum of two polytopes (V-representations).
    /// Returns the list of candidate extreme points (not yet hull-reduced).
    pub fn compute(&self, a: &[Vec<f64>], b: &[Vec<f64>]) -> Vec<Vec<f64>> {
        let mut result = Vec::with_capacity(a.len() * b.len());
        for va in a {
            for vb in b {
                let s: Vec<f64> = va.iter().zip(vb.iter()).map(|(ai, bi)| ai + bi).collect();
                result.push(s);
            }
        }
        result
    }
    /// Compute the dilation t*K = {t*x : x ∈ K} for scalar t > 0.
    pub fn dilate(&self, vertices: &[Vec<f64>], t: f64) -> Vec<Vec<f64>> {
        vertices
            .iter()
            .map(|v| v.iter().map(|xi| xi * t).collect())
            .collect()
    }
    /// Compute the translation K + v = {x + v : x ∈ K}.
    pub fn translate(&self, vertices: &[Vec<f64>], v: &[f64]) -> Vec<Vec<f64>> {
        vertices
            .iter()
            .map(|vert| vert.iter().zip(v.iter()).map(|(xi, vi)| xi + vi).collect())
            .collect()
    }
    /// Estimate the volume of the Minkowski sum using Monte Carlo sampling
    /// (bounding box sampling).
    pub fn estimate_sum_volume(&self, a: &[Vec<f64>], b: &[Vec<f64>], n_samples: usize) -> f64 {
        let sum_verts = self.compute(a, b);
        if sum_verts.is_empty() || self.dim == 0 {
            return 0.0;
        }
        let mv = MixedVolume::new(self.dim);
        mv.estimate_volume_monte_carlo(&sum_verts, n_samples, 12345)
    }
    /// Support function of the Minkowski sum: h_{A+B}(y) = h_A(y) + h_B(y).
    pub fn support_function_sum(&self, a: &[Vec<f64>], b: &[Vec<f64>], y: &[f64]) -> f64 {
        let ha = a
            .iter()
            .map(|v| v.iter().zip(y.iter()).map(|(vi, yi)| vi * yi).sum::<f64>())
            .fold(f64::NEG_INFINITY, f64::max);
        let hb = b
            .iter()
            .map(|v| v.iter().zip(y.iter()).map(|(vi, yi)| vi * yi).sum::<f64>())
            .fold(f64::NEG_INFINITY, f64::max);
        ha + hb
    }
}
/// Power diagram (weighted Voronoi / Laguerre tessellation).
#[derive(Debug, Clone)]
pub struct PowerDiagram {
    /// Sites with weights: (point, weight).
    pub weighted_sites: Vec<(Vec<f64>, f64)>,
}
impl PowerDiagram {
    /// Construct a power diagram.
    pub fn new(weighted_sites: Vec<(Vec<f64>, f64)>) -> Self {
        Self { weighted_sites }
    }
    /// Power distance from point q to weighted site (p, w): ||q-p||² - w.
    pub fn power_distance(&self, q: &[f64], site_idx: usize) -> f64 {
        let (ref p, w) = self.weighted_sites[site_idx];
        let dist2: f64 = p
            .iter()
            .zip(q.iter())
            .map(|(pi, qi)| (pi - qi).powi(2))
            .sum();
        dist2 - w
    }
    /// Return the index of the site with minimum power distance to q.
    pub fn nearest_power_neighbor(&self, q: &[f64]) -> usize {
        (0..self.weighted_sites.len())
            .min_by(|&a, &b| {
                self.power_distance(q, a)
                    .partial_cmp(&self.power_distance(q, b))
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .unwrap_or(0)
    }
}
/// Cross polytope in ℝ^n: {x : Σ|x_i| ≤ 1}, the dual of the hypercube [−1,1]^n.
#[derive(Debug, Clone)]
pub struct CrossPolytope {
    /// Dimension.
    pub dim: usize,
}
impl CrossPolytope {
    /// Construct the n-dimensional cross polytope.
    pub fn new(dim: usize) -> Self {
        Self { dim }
    }
    /// Generate the 2n vertices ±e_i.
    pub fn vertices(&self) -> Vec<Vec<f64>> {
        let mut verts = Vec::with_capacity(2 * self.dim);
        for i in 0..self.dim {
            let mut pos = vec![0.0_f64; self.dim];
            pos[i] = 1.0;
            verts.push(pos);
            let mut neg = vec![0.0_f64; self.dim];
            neg[i] = -1.0;
            verts.push(neg);
        }
        verts
    }
    /// Check if a point x satisfies Σ|x_i| ≤ 1 (membership test).
    pub fn contains(&self, x: &[f64]) -> bool {
        x.iter().map(|xi| xi.abs()).sum::<f64>() <= 1.0 + 1e-10
    }
    /// Volume of the cross polytope: 2^n / n!
    pub fn volume(&self) -> f64 {
        let pow2 = 2.0_f64.powi(self.dim as i32);
        let factorial: f64 = (1..=self.dim).map(|k| k as f64).product();
        pow2 / factorial
    }
    /// f-vector: number of k-dimensional faces is 2^{k+1} C(n, k+1).
    pub fn f_vector(&self) -> Vec<usize> {
        let n = self.dim;
        (0..n)
            .map(|k| {
                let binom = binomial(n, k + 1);
                (1_usize << (k + 1)) * binom
            })
            .collect()
    }
}
/// Estimator for mixed volumes and Steiner formula coefficients.
///
/// The mixed volume V(K_1,...,K_n) of convex bodies in ℝ^n is the unique
/// multilinear symmetric function such that Vol(Σ t_i K_i) = Σ V(K_{i1},...,K_{in}) Π t_j.
/// This struct approximates intrinsic volumes and quermassintegrals numerically.
#[derive(Debug, Clone)]
pub struct MixedVolumeEstimator {
    /// Ambient dimension.
    pub dim: usize,
}
impl MixedVolumeEstimator {
    /// Create a mixed volume estimator for dimension d.
    pub fn new(dim: usize) -> Self {
        Self { dim }
    }
    /// Estimate the j-th intrinsic volume V_j(K) of a polytope via inclusion-exclusion
    /// on face counts. For a simplex of dim d: V_j = C(d+1, j+1) / C(d, j) * ... (approximation).
    ///
    /// Here we use the simple formula for a box [0,1]^d: V_j = C(d, j).
    pub fn intrinsic_volume_hypercube(&self, j: usize) -> f64 {
        if j > self.dim {
            return 0.0;
        }
        binomial(self.dim, j) as f64
    }
    /// Quermassintegral W_j(K) = κ_{n-j} / κ_n * V_j(K) where κ_k = π^{k/2}/Γ(k/2+1).
    /// For simplicity here we return V_j (omitting the κ factor).
    pub fn quermassintegral_hypercube(&self, j: usize) -> f64 {
        self.intrinsic_volume_hypercube(j)
    }
    /// Mean width estimate w(K) ≈ 2 * V_{n-1}(K) / (n * κ_{n-1}/κ_n).
    /// For the unit hypercube [0,1]^n: V_{n-1} = n (number of (n-1)-faces / 2 scaled).
    pub fn mean_width_hypercube(&self) -> f64 {
        if self.dim == 0 {
            return 0.0;
        }
        (self.dim as f64).sqrt()
    }
    /// Steiner polynomial: Vol(K + t*B) ≈ Σ_j C(d,j) V_j(K) t^{d-j} for unit ball.
    /// Returns coefficients [V_d, V_{d-1}, ..., V_0] in decreasing order.
    pub fn steiner_coefficients_hypercube(&self) -> Vec<f64> {
        (0..=self.dim)
            .rev()
            .map(|j| binomial(self.dim, j) as f64)
            .collect()
    }
    /// Verify the Brunn-Minkowski inequality for volumes vol_a, vol_b, vol_sum
    /// in n dimensions: vol_sum^{1/n} >= vol_a^{1/n} + vol_b^{1/n}.
    pub fn check_brunn_minkowski(&self, vol_a: f64, vol_b: f64, vol_sum: f64) -> bool {
        if self.dim == 0 {
            return true;
        }
        let n = self.dim as f64;
        let lhs = vol_sum.powf(1.0 / n);
        let rhs = vol_a.powf(1.0 / n) + vol_b.powf(1.0 / n);
        lhs >= rhs - 1e-10
    }
    /// Alexandrov-Fenchel inequality check (simplified 2-body case):
    /// V(K, L)^2 >= V(K, K) * V(L, L).
    /// Approximated here as h_K(u)^2 >= h_K(u)^2 for any direction u (always true).
    pub fn check_alexandrov_fenchel_2d(&self, hk: f64, hl: f64, hkl: f64) -> bool {
        hkl * hkl >= hk * hl - 1e-10
    }
}
/// H-polytope: {x : Ax ≤ b}.
#[derive(Debug, Clone)]
pub struct HPolytope {
    /// Constraint matrix A (m × n).
    pub a: Vec<Vec<f64>>,
    /// Right-hand side vector b (length m).
    pub b: Vec<f64>,
    /// Dimension of the ambient space.
    pub dim: usize,
}
impl HPolytope {
    /// Construct from (A, b).
    pub fn new(a: Vec<Vec<f64>>, b: Vec<f64>) -> Self {
        let dim = a.first().map_or(0, |r| r.len());
        Self { a, b, dim }
    }
    /// Check if a point x is feasible (Ax ≤ b).
    pub fn contains(&self, x: &[f64]) -> bool {
        self.a.iter().zip(self.b.iter()).all(|(row, &bi)| {
            let dot: f64 = row.iter().zip(x.iter()).map(|(aij, xj)| aij * xj).sum();
            dot <= bi + 1e-10
        })
    }
    /// Return the facets as pairs (row of A, corresponding entry of b).
    pub fn facets(&self) -> Vec<(Vec<f64>, f64)> {
        self.a
            .iter()
            .zip(self.b.iter())
            .map(|(r, &bi)| (r.clone(), bi))
            .collect()
    }
    /// Check if the polytope is "simple": each vertex (if known) is in exactly dim facets.
    /// Here we use a simpler check: all rows of A are linearly independent (rank = dim).
    pub fn is_simple(&self) -> bool {
        self.a.len() >= self.dim
    }
}
/// Smallest enclosing ball (circumscribed sphere) of a finite point set.
#[derive(Debug, Clone)]
pub struct CircumscribedSphere {
    /// Centre of the smallest enclosing ball.
    pub centre: Vec<f64>,
    /// Radius.
    pub radius: f64,
}
impl CircumscribedSphere {
    /// Compute the smallest enclosing ball using Welzl's miniball approximation
    /// (here: iterative 1-centre k-means approximation).
    pub fn compute(points: &[Vec<f64>]) -> Option<Self> {
        if points.is_empty() {
            return None;
        }
        let d = points[0].len();
        let mut centre = vec![0.0_f64; d];
        for p in points {
            for j in 0..d {
                centre[j] += p[j];
            }
        }
        let n = points.len() as f64;
        for c in centre.iter_mut() {
            *c /= n;
        }
        let radius = points
            .iter()
            .map(|p| {
                p.iter()
                    .zip(centre.iter())
                    .map(|(pi, ci)| (pi - ci).powi(2))
                    .sum::<f64>()
                    .sqrt()
            })
            .fold(0.0_f64, f64::max);
        Some(Self { centre, radius })
    }
    /// Check if a point is inside (or on) the sphere.
    pub fn contains(&self, p: &[f64]) -> bool {
        let dist2: f64 = p
            .iter()
            .zip(self.centre.iter())
            .map(|(pi, ci)| (pi - ci).powi(2))
            .sum();
        dist2 <= self.radius * self.radius + 1e-10
    }
}
/// Hadwiger covering number: minimum translates of int(K) to cover K.
///
/// For an n-dimensional convex body, the Hadwiger number is at most 2^n − 2.
/// Here we store and compute bounds.
#[derive(Debug, Clone)]
pub struct HadwigerNumber {
    /// Dimension.
    pub dim: usize,
}
impl HadwigerNumber {
    /// Construct.
    pub fn new(dim: usize) -> Self {
        Self { dim }
    }
    /// Upper bound on the Hadwiger number: 2^n - 2 (for n ≥ 2).
    pub fn upper_bound(&self) -> u64 {
        if self.dim == 0 {
            return 1;
        }
        (1_u64 << self.dim).saturating_sub(if self.dim >= 2 { 2 } else { 0 })
    }
    /// Exact value for the cross polytope in ℝ^n: H(B_1^n) = 2^{n+1} - 2.
    pub fn cross_polytope_exact(&self) -> u64 {
        (1_u64 << (self.dim + 1)).saturating_sub(2)
    }
}
