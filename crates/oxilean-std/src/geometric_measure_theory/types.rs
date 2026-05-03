//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)
use super::functions::*;

/// A discrete BV function on a 1D grid {0, 1, …, n-1}.
///
/// The total variation TV(f) = Σ_{i=0}^{n-2} |f(i+1) - f(i)|.
#[derive(Debug, Clone)]
pub struct DiscreteBVFunction {
    /// Function values at grid points 0, 1, …, n-1.
    pub values: Vec<f64>,
}
impl DiscreteBVFunction {
    /// Create from a vector of function values.
    pub fn new(values: Vec<f64>) -> Self {
        Self { values }
    }
    /// Total variation TV(f) = Σ |f(i+1) - f(i)|.
    pub fn total_variation(&self) -> f64 {
        self.values.windows(2).map(|w| (w[1] - w[0]).abs()).sum()
    }
    /// Check if the function is in BV (total variation is finite).
    /// For discrete functions this is always true; returns true here.
    pub fn is_bv(&self) -> bool {
        self.total_variation().is_finite()
    }
    /// Approximate the distributional derivative as differences.
    pub fn distributional_derivative(&self) -> Vec<f64> {
        self.values.windows(2).map(|w| w[1] - w[0]).collect()
    }
    /// L¹ norm of f.
    pub fn l1_norm(&self) -> f64 {
        self.values.iter().map(|v| v.abs()).sum::<f64>() / self.values.len() as f64
    }
    /// Sobolev-type BV semi-norm (total variation per unit length).
    pub fn bv_seminorm(&self) -> f64 {
        if self.values.len() <= 1 {
            return 0.0;
        }
        self.total_variation() / (self.values.len() - 1) as f64
    }
}
/// Current (generalized surface) in geometric measure theory.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct IntegralCurrentNew {
    pub dimension: usize,
    pub mass: f64,
    pub boundary_mass: f64,
    pub is_closed: bool,
}
#[allow(dead_code)]
impl IntegralCurrentNew {
    pub fn new(dim: usize, mass: f64, bdry: f64) -> Self {
        IntegralCurrentNew {
            dimension: dim,
            mass,
            boundary_mass: bdry,
            is_closed: bdry < 1e-12,
        }
    }
    pub fn cycle(dim: usize, mass: f64) -> Self {
        IntegralCurrentNew::new(dim, mass, 0.0)
    }
    pub fn flat_norm(&self) -> f64 {
        self.mass + self.boundary_mass
    }
    pub fn comass(&self) -> f64 {
        self.mass
    }
    pub fn is_integer_multiplicity(&self) -> bool {
        true
    }
}
/// Federer-Fleming compactness theorem (abstract).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CompactnessTheorem {
    pub n: usize,
    pub k: usize,
    pub mass_bound: f64,
}
#[allow(dead_code)]
impl CompactnessTheorem {
    pub fn new(n: usize, k: usize, m: f64) -> Self {
        CompactnessTheorem {
            n,
            k,
            mass_bound: m,
        }
    }
    /// A sequence of integral k-currents with bounded mass has a convergent subsequence.
    pub fn has_convergent_subsequence(&self) -> bool {
        self.mass_bound < f64::INFINITY
    }
    pub fn limit_is_integral_current(&self) -> bool {
        true
    }
}
/// A piecewise-linear Lipschitz map f : \[0, 1\] → \[0, 1\] defined on a uniform grid.
#[derive(Debug, Clone)]
pub struct PiecewiseLinearMap {
    /// Values at grid points 0/n, 1/n, …, n/n.
    pub values: Vec<f64>,
}
impl PiecewiseLinearMap {
    /// Create from a list of n+1 values on \[0,1\].
    pub fn new(values: Vec<f64>) -> Self {
        Self { values }
    }
    /// Lipschitz constant: max |f(x+h) - f(x)| / h over all grid intervals.
    pub fn lipschitz_constant(&self) -> f64 {
        let n = (self.values.len() - 1) as f64;
        self.values
            .windows(2)
            .map(|w| (w[1] - w[0]).abs() * n)
            .fold(0.0_f64, f64::max)
    }
    /// 1-dimensional Jacobian at grid point i (|f'(xᵢ)|).
    pub fn jacobian_at(&self, i: usize) -> f64 {
        if i + 1 >= self.values.len() {
            return 0.0;
        }
        let n = (self.values.len() - 1) as f64;
        (self.values[i + 1] - self.values[i]).abs() * n
    }
    /// Discrete area formula: Σ_{y} #{preimages} ≈ Σ_i |f'(xᵢ)| Δx.
    pub fn discrete_area_formula(&self) -> f64 {
        if self.values.len() <= 1 {
            return 0.0;
        }
        let dx = 1.0 / (self.values.len() - 1) as f64;
        (0..self.values.len() - 1)
            .map(|i| self.jacobian_at(i) * dx)
            .sum()
    }
}
/// Estimates both Hausdorff measure and Hausdorff dimension for a point cloud in ℝ².
///
/// Uses a multi-scale δ-cover strategy to approximate H^s_δ(E) for a range of s,
/// then finds the critical dimension dim_H(E) where the estimated measure transitions.
#[derive(Debug, Clone)]
pub struct HausdorffMeasureEstimator {
    /// Point cloud in ℝ², as (x, y) pairs.
    pub points: Vec<(f64, f64)>,
}
impl HausdorffMeasureEstimator {
    /// Create a new estimator from a point cloud.
    pub fn new(points: Vec<(f64, f64)>) -> Self {
        Self { points }
    }
    /// Estimate H^s_δ content using a greedy δ-cover.
    pub fn hausdorff_content(&self, s: f64, delta: f64) -> f64 {
        if self.points.is_empty() {
            return 0.0;
        }
        let mut covered = vec![false; self.points.len()];
        let mut total = 0.0f64;
        let diam = 2.0 * delta;
        for i in 0..self.points.len() {
            if covered[i] {
                continue;
            }
            covered[i] = true;
            for j in (i + 1)..self.points.len() {
                if !covered[j] {
                    let dx = self.points[i].0 - self.points[j].0;
                    let dy = self.points[i].1 - self.points[j].1;
                    if dx * dx + dy * dy <= delta * delta {
                        covered[j] = true;
                    }
                }
            }
            total += diam.powf(s);
        }
        total
    }
    /// Estimate Hausdorff dimension by binary search over s ∈ \[0, 2\].
    ///
    /// Finds the critical exponent where H^s content transitions from large to small.
    pub fn estimate_dimension(&self, delta: f64) -> f64 {
        if self.points.is_empty() {
            return 0.0;
        }
        let content_at_s = |s: f64| self.hausdorff_content(s, delta);
        let ref_val = content_at_s(1.0);
        if ref_val <= 0.0 {
            return 0.0;
        }
        let mut lo = 0.0_f64;
        let mut hi = 2.0_f64;
        for _ in 0..20 {
            let mid = (lo + hi) * 0.5;
            if content_at_s(mid) < ref_val * 0.5 {
                hi = mid;
            } else {
                lo = mid;
            }
        }
        (lo + hi) * 0.5
    }
    /// Estimate Hausdorff measure at the estimated dimension.
    pub fn estimated_measure(&self, delta: f64) -> f64 {
        let dim = self.estimate_dimension(delta);
        self.hausdorff_content(dim, delta)
    }
}
/// Solves a discrete minimal surface problem via variational relaxation.
///
/// Minimises the area functional A(u) = ∫ √(1 + |∇u|²) dx over u : \[0,1\]² → ℝ
/// with prescribed Dirichlet boundary data, using gradient descent.
#[derive(Debug, Clone)]
pub struct MinimalSurfaceRelaxation {
    /// Grid size (n × n internal points, including boundary).
    pub n: usize,
    /// Current height values; row-major, length n².
    pub u: Vec<f64>,
    /// Boundary mask: true means Dirichlet (fixed) node.
    pub boundary: Vec<bool>,
    /// Step size for gradient descent.
    pub step_size: f64,
}
impl MinimalSurfaceRelaxation {
    /// Create from boundary data (a flat n×n grid with Dirichlet values on boundary).
    ///
    /// Interior points are initialised to the average of boundary values.
    pub fn new(n: usize, boundary_values: Vec<f64>, step_size: f64) -> Option<Self> {
        if boundary_values.len() != n * n {
            return None;
        }
        let boundary: Vec<bool> = (0..n * n)
            .map(|k| {
                let i = k / n;
                let j = k % n;
                i == 0 || i == n - 1 || j == 0 || j == n - 1
            })
            .collect();
        let avg = boundary_values
            .iter()
            .zip(boundary.iter())
            .filter(|(_, &b)| b)
            .map(|(&v, _)| v)
            .sum::<f64>()
            / boundary.iter().filter(|&&b| b).count().max(1) as f64;
        let u: Vec<f64> = boundary_values
            .iter()
            .zip(boundary.iter())
            .map(|(&v, &b)| if b { v } else { avg })
            .collect();
        Some(Self {
            n,
            u,
            boundary,
            step_size,
        })
    }
    /// Compute area functional A(u) = Σ_{i,j} h² √(1 + |∇u|²).
    pub fn area(&self) -> f64 {
        let n = self.n;
        let h = 1.0 / n as f64;
        let mut total = 0.0f64;
        for i in 0..n.saturating_sub(1) {
            for j in 0..n.saturating_sub(1) {
                let du_di = (self.u[(i + 1) * n + j] - self.u[i * n + j]) / h;
                let du_dj = (self.u[i * n + j + 1] - self.u[i * n + j]) / h;
                total += (1.0 + du_di * du_di + du_dj * du_dj).sqrt() * h * h;
            }
        }
        total
    }
    /// Perform one gradient descent step on interior nodes.
    pub fn step(&mut self) {
        let n = self.n;
        let h = 1.0 / n as f64;
        let u_old = self.u.clone();
        for i in 1..n.saturating_sub(1) {
            for j in 1..n.saturating_sub(1) {
                if self.boundary[i * n + j] {
                    continue;
                }
                let du_r = (u_old[i * n + j + 1] - u_old[i * n + j]) / h;
                let du_l = (u_old[i * n + j] - u_old[i * n + j - 1]) / h;
                let du_d = (u_old[(i + 1) * n + j] - u_old[i * n + j]) / h;
                let du_u = (u_old[i * n + j] - u_old[(i - 1) * n + j]) / h;
                let w_r = 1.0 / (1.0 + du_r * du_r).sqrt();
                let w_l = 1.0 / (1.0 + du_l * du_l).sqrt();
                let w_d = 1.0 / (1.0 + du_d * du_d).sqrt();
                let w_u = 1.0 / (1.0 + du_u * du_u).sqrt();
                let grad = (w_r * (u_old[i * n + j + 1] - u_old[i * n + j])
                    - w_l * (u_old[i * n + j] - u_old[i * n + j - 1])
                    + w_d * (u_old[(i + 1) * n + j] - u_old[i * n + j])
                    - w_u * (u_old[i * n + j] - u_old[(i - 1) * n + j]))
                    / (h * h);
                self.u[i * n + j] += self.step_size * grad;
            }
        }
    }
    /// Run `iters` gradient descent steps and return final area.
    pub fn relax(&mut self, iters: usize) -> f64 {
        for _ in 0..iters {
            self.step();
        }
        self.area()
    }
}
/// A Caccioppoli set (set of finite perimeter) in R^n.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CaccioppoliSet {
    /// Dimension n.
    pub dim: usize,
    /// Approximate perimeter (measure of the reduced boundary).
    pub perimeter: f64,
    /// Volume (Lebesgue measure).
    pub volume: f64,
    /// Whether the set is a convex body.
    pub is_convex: bool,
}
#[allow(dead_code)]
impl CaccioppoliSet {
    /// Create a new Caccioppoli set descriptor.
    pub fn new(dim: usize, perimeter: f64, volume: f64) -> Self {
        CaccioppoliSet {
            dim,
            perimeter,
            volume,
            is_convex: false,
        }
    }
    /// Isoperimetric inequality: P^n ≥ n^n ω_n V^{n-1} (with equality for balls).
    /// Returns true if the set satisfies the isoperimetric inequality (approximately).
    pub fn satisfies_isoperimetric(&self) -> bool {
        if self.dim == 0 || self.volume <= 0.0 {
            return true;
        }
        let n = self.dim as f64;
        let omega_n = std::f64::consts::PI.powf(n / 2.0) / gamma_approx(n / 2.0 + 1.0);
        let rhs = n.powf(n) * omega_n * self.volume.powf(n - 1.0);
        self.perimeter.powf(n) >= rhs - 1e-6
    }
    /// Co-area formula (Federer): ∫ |∇u| dx = ∫_R P({u > t}) dt.
    /// Returns the co-area integral approximation for a piecewise linear function.
    pub fn coarea_formula_approx(&self, gradient_magnitude: f64) -> f64 {
        self.perimeter * gradient_magnitude
    }
    /// Reduced boundary: the set of points x in ∂E where the measure-theoretic
    /// outer unit normal ν_E(x) exists.
    /// Returns the approximate reduced boundary measure.
    pub fn reduced_boundary_measure(&self) -> f64 {
        self.perimeter
    }
    /// Plateau's problem: minimal surface bounded by a curve.
    /// For Caccioppoli sets, existence of area-minimizing representatives.
    pub fn area_minimizing_representative_exists(&self) -> bool {
        true
    }
}
/// Discrete density ratio θ(r) = M(V ∩ B(x, r)) / (ω_k r^k) for a 2D varifold.
///
/// Given a finite set of weighted points representing a k-varifold, compute
/// density ratios at increasing radii to verify monotonicity.
#[derive(Debug, Clone)]
pub struct DiscreteDensityRatio {
    /// Points and weights (xᵢ, yᵢ, wᵢ) representing the varifold mass.
    pub points: Vec<(f64, f64, f64)>,
    /// Center point x₀.
    pub center: (f64, f64),
    /// Dimension k (used for ω_k r^k normalization).
    pub k: f64,
}
impl DiscreteDensityRatio {
    /// Create a new density ratio computer.
    pub fn new(points: Vec<(f64, f64, f64)>, center: (f64, f64), k: f64) -> Self {
        Self { points, center, k }
    }
    /// Compute density ratio θ(r) = (Σ_{|xᵢ - x₀| ≤ r} wᵢ) / (ω_k r^k).
    pub fn density_at_radius(&self, r: f64) -> f64 {
        if r <= 0.0 {
            return 0.0;
        }
        let mass: f64 = self
            .points
            .iter()
            .filter(|(x, y, _w)| {
                let dx = x - self.center.0;
                let dy = y - self.center.1;
                dx * dx + dy * dy <= r * r
            })
            .map(|(_, _, w)| w)
            .sum();
        let omega_k = if (self.k - 1.0).abs() < 1e-9 {
            2.0
        } else if (self.k - 2.0).abs() < 1e-9 {
            std::f64::consts::PI
        } else {
            1.0
        };
        mass / (omega_k * r.powf(self.k))
    }
    /// Check monotonicity: θ(r₁) ≤ θ(r₂) for r₁ ≤ r₂ over a grid of radii.
    pub fn is_monotone_nondecreasing(&self, radii: &[f64]) -> bool {
        let densities: Vec<f64> = radii.iter().map(|&r| self.density_at_radius(r)).collect();
        densities.windows(2).all(|w| w[0] <= w[1] + 1e-12)
    }
}
/// Represents an integral current of dimension k in R^n.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct IntegralCurrent {
    /// Ambient dimension n.
    pub ambient_dim: usize,
    /// Current dimension k.
    pub dim: usize,
    /// Mass M(T) = ∫ |T| dH^k.
    pub mass: f64,
    /// Boundary mass M(∂T).
    pub boundary_mass: f64,
    /// Whether the current has integer multiplicity.
    pub integer_multiplicity: bool,
}
#[allow(dead_code)]
impl IntegralCurrent {
    /// Create a new integral current.
    pub fn new(ambient_dim: usize, dim: usize, mass: f64, boundary_mass: f64) -> Self {
        IntegralCurrent {
            ambient_dim,
            dim,
            mass,
            boundary_mass,
            integer_multiplicity: true,
        }
    }
    /// Boundary operator ∂: dimension k → k-1.
    pub fn boundary_dimension(&self) -> usize {
        self.dim.saturating_sub(1)
    }
    /// Federer-Fleming deformation theorem: every integral current can be deformed
    /// to a polyhedral current with the same boundary.
    pub fn deformation_theorem_applies(&self) -> bool {
        self.integer_multiplicity && self.ambient_dim >= self.dim
    }
    /// Compactness theorem: sequences of integral currents with bounded mass and
    /// bounded boundary mass have convergent subsequences.
    pub fn compactness_bound(&self) -> f64 {
        self.mass + self.boundary_mass
    }
    /// Constancy theorem: a current with zero boundary in a connected open set
    /// is a constant multiple of the fundamental class.
    pub fn constancy_theorem_applies(&self) -> bool {
        self.boundary_mass < 1e-12
    }
    /// Flat norm: F(T) = inf_{T = S + ∂R} { M(S) + M(R) }.
    /// Simplified: returns mass as an upper bound.
    pub fn flat_norm_upper_bound(&self) -> f64 {
        self.mass
    }
}
/// Marstrand's projection theorem: almost every orthogonal projection of a
/// Borel set E ⊂ R^n with dim_H(E) > k has positive k-dimensional Lebesgue measure.
#[allow(dead_code)]
pub struct MarstrandProjection {
    /// Hausdorff dimension of the set E.
    pub hausdorff_dim: f64,
    /// Ambient dimension n.
    pub ambient_dim: usize,
    /// Projection dimension k.
    pub proj_dim: usize,
}
#[allow(dead_code)]
impl MarstrandProjection {
    /// Create a new Marstrand projection instance.
    pub fn new(hausdorff_dim: f64, ambient_dim: usize, proj_dim: usize) -> Self {
        MarstrandProjection {
            hausdorff_dim,
            ambient_dim,
            proj_dim,
        }
    }
    /// Marstrand's theorem: if dim_H(E) > k, almost every projection has positive
    /// k-dimensional Lebesgue measure.
    pub fn projection_has_positive_measure(&self) -> bool {
        self.hausdorff_dim > self.proj_dim as f64
    }
    /// Exceptional set: the set of directions where the projection may be small
    /// has Hausdorff dimension at most n - 1 - (dim_H(E) - k).
    pub fn exceptional_set_dim_bound(&self) -> f64 {
        let n = self.ambient_dim as f64;
        let k = self.proj_dim as f64;
        (n - 1.0 - (self.hausdorff_dim - k)).max(0.0)
    }
    /// Falconer's theorem: for almost all (t, E+t), the Hausdorff dimension
    /// of E+t is dim_H(E) (translation-invariance).
    pub fn falconer_translation_stable(&self) -> bool {
        self.hausdorff_dim <= self.ambient_dim as f64
    }
    /// Slicing: the k-dimensional slice of a set with Hausdorff dimension s
    /// typically has dimension s - (n - k).
    pub fn typical_slice_dimension(&self) -> f64 {
        let n = self.ambient_dim as f64;
        let k = self.proj_dim as f64;
        (self.hausdorff_dim - (n - k)).max(0.0)
    }
}
/// Rectifiable set: a set that can be approximated by Lipschitz images.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RectifiableSet {
    pub dimension: usize,
    pub ambient_dimension: usize,
    pub hausdorff_measure: f64,
    pub is_countably_rectifiable: bool,
}
#[allow(dead_code)]
impl RectifiableSet {
    pub fn new(dim: usize, ambient: usize, measure: f64) -> Self {
        RectifiableSet {
            dimension: dim,
            ambient_dimension: ambient,
            hausdorff_measure: measure,
            is_countably_rectifiable: true,
        }
    }
    pub fn smooth_submanifold(dim: usize, ambient: usize, vol: f64) -> Self {
        RectifiableSet::new(dim, ambient, vol)
    }
    pub fn is_integer_rectifiable(&self) -> bool {
        self.is_countably_rectifiable
    }
    pub fn lower_density(&self) -> f64 {
        1.0
    }
}
/// Numerically computes the co-area formula for a Lipschitz function f : \[0,1\]² → ℝ.
///
/// Co-area formula: ∫_{\[0,1\]²} g(x) |∇f(x)| dx = ∫_{-∞}^{∞} (∫_{f=t} g dH¹) dt.
///
/// We approximate the left side via finite differences on an n×n grid.
#[derive(Debug, Clone)]
pub struct CoAreaComputer {
    /// Grid size (n × n).
    pub n: usize,
    /// Values of f at grid points; row-major.
    pub f_values: Vec<f64>,
    /// Values of g at grid points; row-major (integrand weight).
    pub g_values: Vec<f64>,
}
impl CoAreaComputer {
    /// Create from a grid of f and g values (both length n²).
    pub fn new(n: usize, f_values: Vec<f64>, g_values: Vec<f64>) -> Option<Self> {
        if f_values.len() != n * n || g_values.len() != n * n {
            return None;
        }
        Some(Self {
            n,
            f_values,
            g_values,
        })
    }
    /// Compute the left-hand side of the co-area formula: ∫ g |∇f| dx.
    pub fn lhs_integral(&self) -> f64 {
        let n = self.n;
        let h = 1.0 / n as f64;
        let mut total = 0.0f64;
        for i in 1..n.saturating_sub(1) {
            for j in 1..n.saturating_sub(1) {
                let df_di =
                    (self.f_values[(i + 1) * n + j] - self.f_values[(i - 1) * n + j]) / (2.0 * h);
                let df_dj =
                    (self.f_values[i * n + j + 1] - self.f_values[i * n + j - 1]) / (2.0 * h);
                let grad_norm = (df_di * df_di + df_dj * df_dj).sqrt();
                total += self.g_values[i * n + j] * grad_norm * h * h;
            }
        }
        total
    }
    /// Approximate the right-hand side via histogram: group grid cells by f-level
    /// and sum g * H¹_approx({f ≈ t}) for each level bin.
    ///
    /// Uses `num_bins` level bins covering the range \[f_min, f_max\].
    pub fn rhs_integral(&self, num_bins: usize) -> f64 {
        let n = self.n;
        let h = 1.0 / n as f64;
        if num_bins == 0 || self.f_values.is_empty() {
            return 0.0;
        }
        let f_min = self.f_values.iter().cloned().fold(f64::INFINITY, f64::min);
        let f_max = self
            .f_values
            .iter()
            .cloned()
            .fold(f64::NEG_INFINITY, f64::max);
        if (f_max - f_min).abs() < 1e-14 {
            return 0.0;
        }
        let bin_width = (f_max - f_min) / num_bins as f64;
        let mut total = 0.0f64;
        for b in 0..num_bins {
            let t = f_min + (b as f64 + 0.5) * bin_width;
            for i in 0..n.saturating_sub(1) {
                for j in 0..n {
                    let f0 = self.f_values[i * n + j];
                    let f1 = self.f_values[(i + 1) * n + j];
                    if (f0 - t) * (f1 - t) <= 0.0 {
                        let g_mid =
                            (self.g_values[i * n + j] + self.g_values[(i + 1) * n + j]) * 0.5;
                        total += g_mid * h * bin_width;
                    }
                }
            }
            for i in 0..n {
                for j in 0..n.saturating_sub(1) {
                    let f0 = self.f_values[i * n + j];
                    let f1 = self.f_values[i * n + j + 1];
                    if (f0 - t) * (f1 - t) <= 0.0 {
                        let g_mid = (self.g_values[i * n + j] + self.g_values[i * n + j + 1]) * 0.5;
                        total += g_mid * h * bin_width;
                    }
                }
            }
        }
        total
    }
}
/// A discrete set in a 2D grid (n × n), represented as a boolean mask.
#[derive(Debug, Clone)]
pub struct DiscreteSet2D {
    /// Grid size (n × n).
    pub n: usize,
    /// Membership: `mask\[i * n + j\]` is true iff (i, j) ∈ E.
    pub mask: Vec<bool>,
}
impl DiscreteSet2D {
    /// Create a new discrete set from a grid size and membership mask.
    pub fn new(n: usize, mask: Vec<bool>) -> Option<Self> {
        if mask.len() != n * n {
            return None;
        }
        Some(Self { n, mask })
    }
    /// Create a disk of radius r centered at (cx, cy) in an n × n grid.
    pub fn disk(n: usize, cx: f64, cy: f64, r: f64) -> Self {
        let mask = (0..n)
            .flat_map(|i| {
                (0..n).map(move |j| {
                    let dx = i as f64 - cx;
                    let dy = j as f64 - cy;
                    dx * dx + dy * dy <= r * r
                })
            })
            .collect();
        Self { n, mask }
    }
    /// Discrete perimeter: count boundary edges (pairs of adjacent cells (i,j), (i',j')
    /// where exactly one is in E).
    pub fn perimeter(&self) -> usize {
        let n = self.n;
        let mut count = 0;
        for i in 0..n {
            for j in 0..n {
                let here = self.mask[i * n + j];
                if j + 1 < n {
                    let right = self.mask[i * n + j + 1];
                    if here != right {
                        count += 1;
                    }
                } else if here {
                    count += 1;
                }
                if i + 1 < n {
                    let down = self.mask[(i + 1) * n + j];
                    if here != down {
                        count += 1;
                    }
                } else if here {
                    count += 1;
                }
            }
        }
        count
    }
    /// Area (number of cells in E).
    pub fn area(&self) -> usize {
        self.mask.iter().filter(|&&b| b).count()
    }
    /// Check discrete isoperimetric ratio Per² / Area (should be ≥ 4π ≈ 12.57 for disk).
    pub fn isoperimetric_ratio(&self) -> f64 {
        let a = self.area() as f64;
        let p = self.perimeter() as f64;
        if a == 0.0 {
            return 0.0;
        }
        p * p / a
    }
}
/// Approximation of the s-dimensional Hausdorff content of a finite point set in ℝ².
///
/// Uses a greedy δ-cover: iteratively pick the best cover ball of radius δ
/// to minimize total diameter^s over disjoint sub-collections.
#[derive(Debug, Clone)]
pub struct HausdorffContentEstimate {
    /// Points in ℝ² (stored as (x, y) pairs).
    pub points: Vec<(f64, f64)>,
    /// Covering radius δ.
    pub delta: f64,
    /// Hausdorff dimension parameter s.
    pub s: f64,
}
impl HausdorffContentEstimate {
    /// Create a new estimate for points with given δ and dimension s.
    pub fn new(points: Vec<(f64, f64)>, delta: f64, s: f64) -> Self {
        Self { points, delta, s }
    }
    /// Compute the s-dimensional Hausdorff δ-content H^s_δ(E).
    ///
    /// Uses a greedy ball cover: groups points within distance 2δ together and
    /// sums (2δ)^s for each group as a rough upper bound.
    pub fn content(&self) -> f64 {
        if self.points.is_empty() {
            return 0.0;
        }
        let mut covered = vec![false; self.points.len()];
        let mut total = 0.0f64;
        let diam = 2.0 * self.delta;
        for i in 0..self.points.len() {
            if covered[i] {
                continue;
            }
            covered[i] = true;
            for j in (i + 1)..self.points.len() {
                if !covered[j] {
                    let dx = self.points[i].0 - self.points[j].0;
                    let dy = self.points[i].1 - self.points[j].1;
                    if dx * dx + dy * dy <= self.delta * self.delta {
                        covered[j] = true;
                    }
                }
            }
            total += diam.powf(self.s);
        }
        total
    }
    /// Estimate Hausdorff dimension by computing content for multiple values of s
    /// and finding the threshold where the content transitions from ∞ to 0.
    pub fn estimate_dimension(&self) -> f64 {
        let s_values = [0.0, 0.25, 0.5, 0.75, 1.0, 1.25, 1.5, 1.75, 2.0];
        let reference = HausdorffContentEstimate {
            points: self.points.clone(),
            delta: self.delta,
            s: 1.0,
        };
        let ref_content = reference.content();
        if ref_content == 0.0 {
            return 0.0;
        }
        for &s in &s_values {
            let est = HausdorffContentEstimate {
                points: self.points.clone(),
                delta: self.delta,
                s,
            };
            if est.content() < ref_content * 0.5 {
                return s;
            }
        }
        2.0
    }
}
/// Checks approximate k-rectifiability of a finite point set in ℝ² for k = 1.
///
/// A point set is approximately 1-rectifiable if most points lie close to
/// a collection of lines (1-dimensional Lipschitz images).  We use a simple
/// PCA-based approach: test whether the point cloud has a dominant direction.
#[derive(Debug, Clone)]
pub struct RectifiabilityChecker {
    /// Point cloud in ℝ².
    pub points: Vec<(f64, f64)>,
}
impl RectifiabilityChecker {
    /// Create from a point cloud.
    pub fn new(points: Vec<(f64, f64)>) -> Self {
        Self { points }
    }
    /// Compute the centroid of the point cloud.
    pub fn centroid(&self) -> (f64, f64) {
        if self.points.is_empty() {
            return (0.0, 0.0);
        }
        let n = self.points.len() as f64;
        let cx = self.points.iter().map(|p| p.0).sum::<f64>() / n;
        let cy = self.points.iter().map(|p| p.1).sum::<f64>() / n;
        (cx, cy)
    }
    /// Compute the 2×2 covariance matrix entries (cxx, cxy, cyy).
    pub fn covariance(&self) -> (f64, f64, f64) {
        if self.points.len() < 2 {
            return (0.0, 0.0, 0.0);
        }
        let (cx, cy) = self.centroid();
        let n = self.points.len() as f64;
        let cxx = self
            .points
            .iter()
            .map(|p| (p.0 - cx) * (p.0 - cx))
            .sum::<f64>()
            / n;
        let cxy = self
            .points
            .iter()
            .map(|p| (p.0 - cx) * (p.1 - cy))
            .sum::<f64>()
            / n;
        let cyy = self
            .points
            .iter()
            .map(|p| (p.1 - cy) * (p.1 - cy))
            .sum::<f64>()
            / n;
        (cxx, cxy, cyy)
    }
    /// Eigenvalues of the 2×2 covariance matrix (λ₁ ≥ λ₂).
    pub fn eigenvalues(&self) -> (f64, f64) {
        let (cxx, cxy, cyy) = self.covariance();
        let trace = cxx + cyy;
        let det = cxx * cyy - cxy * cxy;
        let disc = (trace * trace * 0.25 - det).max(0.0).sqrt();
        let lam1 = trace * 0.5 + disc;
        let lam2 = (trace * 0.5 - disc).max(0.0);
        (lam1, lam2)
    }
    /// Linearity ratio λ₁ / (λ₁ + λ₂): close to 1 means approximately 1-rectifiable.
    pub fn linearity_ratio(&self) -> f64 {
        let (l1, l2) = self.eigenvalues();
        if l1 + l2 <= 0.0 {
            return 0.0;
        }
        l1 / (l1 + l2)
    }
    /// Returns true if linearity ratio exceeds the given threshold (e.g., 0.9).
    pub fn is_approximately_rectifiable(&self, threshold: f64) -> bool {
        self.linearity_ratio() >= threshold
    }
}
/// Plateau's problem (existence of area-minimizing surfaces).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PlateauProblem {
    pub boundary_curve: String,
    pub ambient_dim: usize,
    pub solution_exists: bool,
    pub solution_unique: bool,
}
#[allow(dead_code)]
impl PlateauProblem {
    pub fn new(boundary: &str, n: usize) -> Self {
        PlateauProblem {
            boundary_curve: boundary.to_string(),
            ambient_dim: n,
            solution_exists: true,
            solution_unique: false,
        }
    }
    pub fn soap_film_analogy() -> &'static str {
        "Minimal surfaces minimize area subject to a fixed boundary"
    }
}
/// Approximates the perimeter of a set defined by a level-set function on a 2D grid.
///
/// Given a smooth function u : \[0,1\]² → ℝ (discretised on an n×n grid),
/// approximates Per({u > 0}) via the co-area formula:
///   Per({u > 0}) ≈ ∫_{ℝ} H^{n-1}({u = t}) dt evaluated at t = 0.
#[derive(Debug, Clone)]
pub struct PerimeterApprox {
    /// Grid size (n × n).
    pub n: usize,
    /// Values of u at grid points; row-major order (u\[i * n + j\]).
    pub values: Vec<f64>,
}
impl PerimeterApprox {
    /// Create from a grid size and flattened value array of length n².
    pub fn new(n: usize, values: Vec<f64>) -> Option<Self> {
        if values.len() != n * n {
            return None;
        }
        Some(Self { n, values })
    }
    /// Approximate perimeter via finite-difference gradient magnitude:
    /// Per({u > 0}) ≈ ∫ |∇u| δ(u) dx ≈ Σ_{i,j} |∇u(i,j)| * step(neighborhood).
    ///
    /// Uses a simple approximation: sum of |∇u| over the transition band |u| < h.
    pub fn perimeter(&self, threshold: f64) -> f64 {
        let n = self.n;
        let h = 1.0 / n as f64;
        let mut perim = 0.0f64;
        for i in 1..n.saturating_sub(1) {
            for j in 1..n.saturating_sub(1) {
                let u = self.values[i * n + j];
                if u.abs() < threshold {
                    let du_di =
                        (self.values[(i + 1) * n + j] - self.values[(i - 1) * n + j]) / (2.0 * h);
                    let du_dj =
                        (self.values[i * n + j + 1] - self.values[i * n + j - 1]) / (2.0 * h);
                    perim += (du_di * du_di + du_dj * du_dj).sqrt() * h * h;
                }
            }
        }
        perim
    }
    /// Volume (area) of the region {u > 0}.
    pub fn volume(&self) -> f64 {
        let n = self.n;
        let h = 1.0 / n as f64;
        let count = self.values.iter().filter(|&&v| v > 0.0).count();
        count as f64 * h * h
    }
    /// Isoperimetric deficit: Per² / (4π Volume) - 1. Should be ≥ 0 by isoperimetric ineq.
    pub fn isoperimetric_deficit(&self, threshold: f64) -> f64 {
        let p = self.perimeter(threshold);
        let v = self.volume();
        if v <= 0.0 {
            return 0.0;
        }
        (p * p) / (4.0 * std::f64::consts::PI * v) - 1.0
    }
}
