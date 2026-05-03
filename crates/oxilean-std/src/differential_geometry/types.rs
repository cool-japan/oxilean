//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;

/// First fundamental form coefficients (metric tensor)
///
/// Given a surface patch r(u,v):
/// - E = g_uu = r_u · r_u
/// - F = g_uv = r_u · r_v
/// - G = g_vv = r_v · r_v
pub struct FirstFundamentalForm {
    pub e: f64,
    pub f: f64,
    pub g: f64,
}
impl FirstFundamentalForm {
    pub fn new(e: f64, f: f64, g: f64) -> Self {
        Self { e, f, g }
    }
    /// Determinant EG - F^2
    pub fn det(&self) -> f64 {
        self.e * self.g - self.f * self.f
    }
    /// Area element sqrt(EG - F^2)
    pub fn area_element(&self) -> f64 {
        self.det().sqrt()
    }
    /// Check positive definiteness: det > 0 and E > 0
    pub fn is_positive_definite(&self) -> bool {
        self.det() > 0.0 && self.e > 0.0
    }
}
/// Euler integrator for the geodesic equation on a 2D Riemannian manifold.
///
/// Integrates  d²x^k/dt² = -Γ^k_ij (dx^i/dt)(dx^j/dt)
/// using the forward Euler method.
pub struct GeodesicIntegrator {
    /// Current position (u, v)
    pub pos: [f64; 2],
    /// Current velocity (u̇, v̇)
    pub vel: [f64; 2],
}
impl GeodesicIntegrator {
    pub fn new(pos: [f64; 2], vel: [f64; 2]) -> Self {
        Self { pos, vel }
    }
    /// Advance by one step of size dt using the Euler method.
    ///
    /// Requires the Christoffel symbols at the current position.
    pub fn step(&mut self, metric: &RiemannianMetric2D, gamma: &[[[f64; 2]; 2]; 2], dt: f64) {
        let acc = metric.geodesic_acceleration(gamma, &self.vel);
        self.pos[0] += self.vel[0] * dt;
        self.pos[1] += self.vel[1] * dt;
        self.vel[0] += acc[0] * dt;
        self.vel[1] += acc[1] * dt;
    }
    /// Integrate for `n` steps and return trajectory of positions.
    pub fn integrate(
        &mut self,
        metric: &RiemannianMetric2D,
        gamma: &[[[f64; 2]; 2]; 2],
        dt: f64,
        n: usize,
    ) -> Vec<[f64; 2]> {
        let mut traj = Vec::with_capacity(n + 1);
        traj.push(self.pos);
        for _ in 0..n {
            self.step(metric, gamma, dt);
            traj.push(self.pos);
        }
        traj
    }
}
/// A torus with major radius R (center to tube center) and minor radius r (tube radius)
pub struct Torus {
    pub major_radius: f64,
    pub minor_radius: f64,
}
impl Torus {
    /// Gaussian curvature at angle theta (measured from outer equator)
    ///
    /// K(θ) = cos(θ) / (r(R + r cos(θ)))
    pub fn gaussian_curvature_at(&self, theta: f64) -> f64 {
        let r = self.minor_radius;
        let big_r = self.major_radius;
        theta.cos() / (r * (big_r + r * theta.cos()))
    }
    /// Euler characteristic χ = 0
    pub fn euler_characteristic(&self) -> i32 {
        0
    }
    /// Surface area = 4π² R r
    pub fn area(&self) -> f64 {
        4.0 * std::f64::consts::PI * std::f64::consts::PI * self.major_radius * self.minor_radius
    }
}
/// A sphere of given radius
pub struct Sphere {
    pub radius: f64,
}
impl Sphere {
    /// Gaussian curvature K = 1/R^2 (constant)
    pub fn gaussian_curvature(&self) -> f64 {
        1.0 / (self.radius * self.radius)
    }
    /// Euler characteristic χ = 2
    pub fn euler_characteristic(&self) -> i32 {
        2
    }
    /// Surface area = 4πR²
    pub fn area(&self) -> f64 {
        4.0 * std::f64::consts::PI * self.radius * self.radius
    }
    /// First fundamental form at any point (E=R², F=0, G=R² sin²θ)
    /// Using standard spherical coords (θ=polar, φ=azimuthal), at θ=π/2: G=R²
    pub fn first_fundamental_form_equator(&self) -> FirstFundamentalForm {
        let r2 = self.radius * self.radius;
        FirstFundamentalForm::new(r2, 0.0, r2)
    }
    /// Second fundamental form at equator: L=R, M=0, N=R
    pub fn second_fundamental_form_equator(&self) -> SecondFundamentalForm {
        SecondFundamentalForm::new(self.radius, 0.0, self.radius)
    }
}
/// Second fundamental form coefficients (shape operator)
///
/// Given unit normal N:
/// - L = h_uu = N · r_uu
/// - M = h_uv = N · r_uv
/// - N_coeff = h_vv = N · r_vv  (renamed to avoid clash with normal N)
pub struct SecondFundamentalForm {
    pub l: f64,
    pub m: f64,
    pub n: f64,
}
impl SecondFundamentalForm {
    pub fn new(l: f64, m: f64, n: f64) -> Self {
        Self { l, m, n }
    }
    /// Gaussian curvature K = (LN - M^2) / (EG - F^2)
    pub fn gaussian_curvature(&self, first: &FirstFundamentalForm) -> f64 {
        let det_first = first.det();
        if det_first.abs() < 1e-12 {
            return 0.0;
        }
        (self.l * self.n - self.m * self.m) / det_first
    }
    /// Mean curvature H = (EN - 2FM + GL) / (2(EG - F^2))
    pub fn mean_curvature(&self, first: &FirstFundamentalForm) -> f64 {
        let det_first = first.det();
        if det_first.abs() < 1e-12 {
            return 0.0;
        }
        (first.e * self.n - 2.0 * first.f * self.m + first.g * self.l) / (2.0 * det_first)
    }
    /// Principal curvatures κ₁, κ₂ (eigenvalues of shape operator)
    ///
    /// H = (κ₁ + κ₂)/2 and K = κ₁ κ₂, so κ = H ± sqrt(H^2 - K)
    pub fn principal_curvatures(&self, first: &FirstFundamentalForm) -> (f64, f64) {
        let h = self.mean_curvature(first);
        let k = self.gaussian_curvature(first);
        let discriminant = (h * h - k).max(0.0);
        let sqrt_disc = discriminant.sqrt();
        (h - sqrt_disc, h + sqrt_disc)
    }
}
/// A 2D Lorentzian metric g = diag(-c², 1) for flat spacetime.
///
/// This models the (t, x) plane with speed of light c.
/// The signature is (-,+): timelike vectors v have g(v,v) < 0.
#[allow(dead_code)]
pub struct LorentzianMetric2D {
    /// Speed of light squared: c²
    pub c_sq: f64,
}
#[allow(dead_code)]
impl LorentzianMetric2D {
    /// Flat Minkowski metric with c=1.
    pub fn minkowski() -> Self {
        Self { c_sq: 1.0 }
    }
    /// Metric tensor g_ij: g\[0\]\[0\]=-c², g\[0\]\[1\]=g\[1\]\[0\]=0, g\[1\]\[1\]=1.
    pub fn metric_tensor(&self) -> [[f64; 2]; 2] {
        [[-self.c_sq, 0.0], [0.0, 1.0]]
    }
    /// Inner product g(u, v) = -c² u\[0\]v\[0\] + u\[1\]v\[1\].
    pub fn inner_product(&self, u: &[f64; 2], v: &[f64; 2]) -> f64 {
        -self.c_sq * u[0] * v[0] + u[1] * v[1]
    }
    /// Check if a vector is timelike: g(v,v) < 0.
    pub fn is_timelike(&self, v: &[f64; 2]) -> bool {
        self.inner_product(v, v) < 0.0
    }
    /// Check if a vector is spacelike: g(v,v) > 0.
    pub fn is_spacelike(&self, v: &[f64; 2]) -> bool {
        self.inner_product(v, v) > 0.0
    }
    /// Check if a vector is null (lightlike): g(v,v) = 0.
    pub fn is_null(&self, v: &[f64; 2]) -> bool {
        self.inner_product(v, v).abs() < 1e-12
    }
    /// Proper time along a timelike curve parameterized by coordinate time.
    ///
    /// For a worldline with spatial velocity dx/dt = v_x, the proper time is
    /// τ = ∫ sqrt(c² - v_x²) / c dt ≈ sqrt(1 - v²/c²) · Δt (time dilation).
    pub fn proper_time_element(&self, vx: f64) -> f64 {
        let c_sq = self.c_sq;
        let val = c_sq - vx * vx;
        if val <= 0.0 {
            0.0
        } else {
            val.sqrt() / c_sq.sqrt()
        }
    }
}
/// Hodge star operator ★ for differential forms in flat Euclidean R^n.
///
/// For an oriented orthonormal basis (e_0, ..., e_{n-1}), the Hodge star of
/// dx^{i_1} ∧ ... ∧ dx^{i_k} is the unique (n-k)-form such that
/// ω ∧ ★ω = |ω|² vol_n.
pub struct HodgeStar {
    /// Ambient dimension n
    pub dim: usize,
}
impl HodgeStar {
    pub fn new(dim: usize) -> Self {
        Self { dim }
    }
    /// Apply Hodge star to a k-form, returning an (n-k)-form.
    ///
    /// Uses the formula: ★(dx^{i_1} ∧ ... ∧ dx^{i_k}) = ε · dx^{j_1} ∧ ... ∧ dx^{j_{n-k}}
    /// where {j_1, ..., j_{n-k}} is the complementary set and ε = ±1 is the sign
    /// of the permutation (i_1, ..., i_k, j_1, ..., j_{n-k}).
    pub fn apply(&self, form: &DifferentialFormWedge) -> DifferentialFormWedge {
        assert_eq!(
            form.dim, self.dim,
            "form dimension must match HodgeStar dimension"
        );
        let n = self.dim;
        let mut result_terms = Vec::new();
        for (coeff, indices) in &form.terms {
            let mut complement: Vec<usize> = (0..n).filter(|j| !indices.contains(j)).collect();
            complement.sort();
            let mut perm: Vec<usize> = indices.clone();
            perm.extend_from_slice(&complement);
            let sign = permutation_sign(&perm);
            result_terms.push((coeff * sign, complement));
        }
        DifferentialFormWedge::new(self.dim, result_terms)
    }
    /// Compute the L² inner product ⟨α, β⟩ = ∫ α ∧ ★β over the unit cube.
    ///
    /// For flat Euclidean space this is just matching coefficients:
    /// ⟨α, β⟩ = Σ_I α_I β_I
    pub fn inner_product(
        &self,
        alpha: &DifferentialFormWedge,
        beta: &DifferentialFormWedge,
    ) -> f64 {
        let star_beta = self.apply(beta);
        let wedge = alpha.wedge(&star_beta);
        let full_indices: Vec<usize> = (0..self.dim).collect();
        wedge
            .terms
            .iter()
            .find(|(_, i)| *i == full_indices)
            .map_or(0.0, |(c, _)| *c)
    }
}
/// Computes the Weyl tensor decomposition in dimension n=3 (where Weyl vanishes identically)
/// and n=4 (generic case).
///
/// Weyl = Riemann - (1/(n-2))(Ricci ∧ g) + (R / ((n-1)(n-2)))(g ∧ g)
/// where ∧ denotes the Kulkarni-Nomizu product.
#[allow(dead_code)]
pub struct WeylTensorComputer {
    /// Dimension n
    pub dim: usize,
    /// Scalar curvature R
    pub scalar_curvature: f64,
}
#[allow(dead_code)]
impl WeylTensorComputer {
    pub fn new(dim: usize, scalar_curvature: f64) -> Self {
        Self {
            dim,
            scalar_curvature,
        }
    }
    /// In dimension 3, the Weyl tensor vanishes identically: W = 0.
    pub fn weyl_vanishes_in_dim3(&self) -> bool {
        self.dim == 3
    }
    /// Compute the coefficient multiplying (Ricci ∧ g) in the Weyl decomposition.
    pub fn ricci_coefficient(&self) -> f64 {
        if self.dim < 3 {
            return 0.0;
        }
        -1.0 / (self.dim as f64 - 2.0)
    }
    /// Compute the coefficient multiplying (g ∧ g) in the Weyl decomposition.
    pub fn metric_coefficient(&self) -> f64 {
        let n = self.dim as f64;
        if n < 3.0 {
            return 0.0;
        }
        self.scalar_curvature / ((n - 1.0) * (n - 2.0))
    }
    /// Compute the Weyl norm estimate for a conformally flat manifold (Weyl = 0).
    pub fn is_conformally_flat(&self, weyl_norm: f64) -> bool {
        weyl_norm < 1e-10
    }
}
/// A curve in R^3: γ(t) = (x(t), y(t), z(t)), given as sampled points
pub struct Curve3D {
    pub points: Vec<[f64; 3]>,
}
impl Curve3D {
    pub fn new(points: Vec<[f64; 3]>) -> Self {
        Self { points }
    }
    /// Approximate arc length by summing chord lengths
    pub fn length(&self) -> f64 {
        if self.points.len() < 2 {
            return 0.0;
        }
        self.points
            .windows(2)
            .map(|w| norm3(&sub3(&w[1], &w[0])))
            .sum()
    }
    /// Numerical curvature |γ'' × γ'| / |γ'|^3 using finite differences
    pub fn curvature_at(&self, i: usize) -> f64 {
        let n = self.points.len();
        if i == 0 || i + 1 >= n {
            return 0.0;
        }
        let prev = &self.points[i - 1];
        let curr = &self.points[i];
        let next = &self.points[i + 1];
        let d1 = scale3(&sub3(next, prev), 0.5);
        let d2 = sub3(&add3(next, prev), &scale3(curr, 2.0));
        let cross = cross3(&d2, &d1);
        let cross_norm = norm3(&cross);
        let d1_norm = norm3(&d1);
        if d1_norm < 1e-12 {
            return 0.0;
        }
        cross_norm / (d1_norm * d1_norm * d1_norm)
    }
    /// Numerical torsion using finite differences
    pub fn torsion_at(&self, i: usize) -> f64 {
        let n = self.points.len();
        if i < 2 || i + 2 >= n {
            return 0.0;
        }
        let prev2 = &self.points[i - 2];
        let prev1 = &self.points[i - 1];
        let curr = &self.points[i];
        let next1 = &self.points[i + 1];
        let next2 = &self.points[i + 2];
        let d1 = scale3(&sub3(next1, prev1), 0.5);
        let d2 = sub3(&add3(next1, prev1), &scale3(curr, 2.0));
        let d2_prev = sub3(&add3(curr, prev2), &scale3(prev1, 2.0));
        let d2_next = sub3(&add3(next2, curr), &scale3(next1, 2.0));
        let d3 = scale3(&sub3(&d2_next, &d2_prev), 0.5);
        let cross_d1_d2 = cross3(&d1, &d2);
        let denom = dot3(&cross_d1_d2, &cross_d1_d2);
        if denom < 1e-12 {
            return 0.0;
        }
        dot3(&cross_d1_d2, &d3) / denom
    }
    /// Check if curve is closed: first ≈ last point
    pub fn is_closed(&self) -> bool {
        if self.points.len() < 2 {
            return false;
        }
        let first = self
            .points
            .first()
            .expect("points has at least 2 elements: checked by early return");
        let last = self
            .points
            .last()
            .expect("points has at least 2 elements: checked by early return");
        norm3(&sub3(last, first)) < 1e-6
    }
    /// Frenet-Serret frame (T, N, B) at index i
    pub fn frenet_frame_at(&self, i: usize) -> Option<([f64; 3], [f64; 3], [f64; 3])> {
        let n = self.points.len();
        if i == 0 || i + 1 >= n {
            return None;
        }
        let prev = &self.points[i - 1];
        let next = &self.points[i + 1];
        let curr = &self.points[i];
        let d1 = scale3(&sub3(next, prev), 0.5);
        let tangent = normalize3(&d1);
        if norm3(&tangent) < 1e-12 {
            return None;
        }
        let d2 = sub3(&add3(next, prev), &scale3(curr, 2.0));
        let d2_dot_t = dot3(&d2, &tangent);
        let d2_perp = sub3(&d2, &scale3(&tangent, d2_dot_t));
        let normal = normalize3(&d2_perp);
        if norm3(&normal) < 1e-12 {
            return None;
        }
        let binormal = cross3(&tangent, &normal);
        Some((tangent, normal, binormal))
    }
}
/// Christoffel symbols for a Riemannian manifold (first kind).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ChristoffelSymbols {
    pub dim: usize,
    pub gamma: Vec<Vec<Vec<f64>>>,
}
#[allow(dead_code)]
impl ChristoffelSymbols {
    pub fn new(dim: usize) -> Self {
        let gamma = vec![vec![vec![0.0; dim]; dim]; dim];
        ChristoffelSymbols { dim, gamma }
    }
    pub fn set(&mut self, k: usize, i: usize, j: usize, val: f64) {
        self.gamma[k][i][j] = val;
        self.gamma[k][j][i] = val;
    }
    pub fn get(&self, k: usize, i: usize, j: usize) -> f64 {
        self.gamma[k][i][j]
    }
    /// For flat Euclidean space, all Christoffel symbols vanish.
    pub fn is_flat(&self) -> bool {
        self.gamma
            .iter()
            .all(|r| r.iter().all(|c| c.iter().all(|&v| v.abs() < 1e-12)))
    }
}
/// Curvature form (abstract Riemann curvature tensor representation).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CurvatureTensor {
    pub dim: usize,
    pub scalar_curvature: f64,
    pub ricci: Vec<Vec<f64>>,
}
#[allow(dead_code)]
impl CurvatureTensor {
    pub fn flat(dim: usize) -> Self {
        CurvatureTensor {
            dim,
            scalar_curvature: 0.0,
            ricci: vec![vec![0.0; dim]; dim],
        }
    }
    pub fn sphere_unit(dim: usize) -> Self {
        let r = (dim * (dim - 1)) as f64;
        let ricci: Vec<Vec<f64>> = (0..dim)
            .map(|i| {
                (0..dim)
                    .map(|j| if i == j { r / dim as f64 } else { 0.0 })
                    .collect()
            })
            .collect();
        CurvatureTensor {
            dim,
            scalar_curvature: r,
            ricci,
        }
    }
    pub fn is_einstein(&self, lambda: f64) -> bool {
        for i in 0..self.dim {
            for j in 0..self.dim {
                let expected = if i == j { lambda } else { 0.0 };
                if (self.ricci[i][j] - expected).abs() > 1e-9 {
                    return false;
                }
            }
        }
        true
    }
    pub fn ricci_scalar(&self) -> f64 {
        (0..self.dim).map(|i| self.ricci[i][i]).sum()
    }
}
/// Checks if a 2-plane (given by two tangent vectors) is calibrated by a 2-form φ.
///
/// A 2-plane spanned by (u, v) is calibrated if φ(u,v) = Area(u,v) = |u×v|.
#[allow(dead_code)]
pub struct CalibrationChecker {
    /// The calibration 2-form stored as skew-symmetric matrix: φ_ij with φ(e_i, e_j) = mat\[i\]\[j\]
    pub form: [[f64; 3]; 3],
}
#[allow(dead_code)]
impl CalibrationChecker {
    pub fn new(form: [[f64; 3]; 3]) -> Self {
        Self { form }
    }
    /// Special Lagrangian calibration form Re(dz_1 ∧ dz_2 ∧ dz_3) restricted to R^3.
    /// In the (e_1, e_2, e_3) basis: φ = dx ∧ dy ∧ dz (volume form in R^3 treated as 3D).
    /// Here we return a simpler 2D version: φ = dx ∧ dy.
    pub fn area_form_r3() -> Self {
        Self {
            form: [[0.0, 1.0, 0.0], [-1.0, 0.0, 0.0], [0.0, 0.0, 0.0]],
        }
    }
    /// Evaluate φ(u, v) = Σ_{ij} φ_{ij} u^i v^j.
    pub fn evaluate(&self, u: &[f64; 3], v: &[f64; 3]) -> f64 {
        let mut sum = 0.0;
        for i in 0..3 {
            for j in 0..3 {
                sum += self.form[i][j] * u[i] * v[j];
            }
        }
        sum
    }
    /// Area of the 2-plane spanned by u, v: |u × v|.
    pub fn area(&self, u: &[f64; 3], v: &[f64; 3]) -> f64 {
        let c = cross3(u, v);
        norm3(&c)
    }
    /// Check if the 2-plane spanned by u, v is calibrated: φ(u,v) ≥ Area(u,v) · comass.
    pub fn is_calibrated(&self, u: &[f64; 3], v: &[f64; 3]) -> bool {
        let phi_uv = self.evaluate(u, v);
        let area_uv = self.area(u, v);
        if area_uv < 1e-12 {
            return true;
        }
        (phi_uv / area_uv - 1.0).abs() < 1e-9
    }
}
/// Symbolic representation of a differential p-form as a sum of basis monomials.
///
/// A p-form in R^n is represented as a list of (coefficient, sorted index list) pairs.
/// For example, in R^3:
///   2 dx^0 ∧ dx^1 + 3 dx^1 ∧ dx^2 is `[(2.0, vec!\[0,1\]), (3.0, vec!\[1,2\])]`
#[derive(Clone, Debug)]
pub struct DifferentialFormWedge {
    /// List of (coefficient, sorted basis indices)
    pub terms: Vec<(f64, Vec<usize>)>,
    /// Ambient dimension n
    pub dim: usize,
}
impl DifferentialFormWedge {
    /// Construct a k-form from explicit terms.
    pub fn new(dim: usize, terms: Vec<(f64, Vec<usize>)>) -> Self {
        let mut form = Self { dim, terms };
        form.normalize();
        form
    }
    /// Basis 1-form dx^i in R^n.
    pub fn basis_1form(dim: usize, i: usize) -> Self {
        Self::new(dim, vec![(1.0, vec![i])])
    }
    /// Sort indices and apply sign from permutation, merge duplicates.
    fn normalize(&mut self) {
        let mut normalized: Vec<(f64, Vec<usize>)> = Vec::new();
        for (coeff, indices) in &self.terms {
            if *coeff == 0.0 {
                continue;
            }
            let mut idx = indices.clone();
            let mut sign = 1.0_f64;
            let n = idx.len();
            let mut has_dup = false;
            for i in 0..n {
                for j in 0..n - 1 - i {
                    if idx[j] > idx[j + 1] {
                        idx.swap(j, j + 1);
                        sign *= -1.0;
                    } else if idx[j] == idx[j + 1] {
                        has_dup = true;
                    }
                }
            }
            if has_dup {
                continue;
            }
            if let Some(existing) = normalized.iter_mut().find(|(_, idxs)| *idxs == idx) {
                existing.0 += coeff * sign;
            } else {
                normalized.push((coeff * sign, idx));
            }
        }
        normalized.retain(|(c, _)| c.abs() > 1e-14);
        self.terms = normalized;
    }
    /// Wedge product α ∧ β.
    pub fn wedge(&self, other: &DifferentialFormWedge) -> DifferentialFormWedge {
        assert_eq!(self.dim, other.dim, "dimension mismatch in wedge product");
        let mut result_terms = Vec::new();
        for (ca, ia) in &self.terms {
            for (cb, ib) in &other.terms {
                let mut indices = ia.clone();
                indices.extend_from_slice(ib);
                result_terms.push((ca * cb, indices));
            }
        }
        DifferentialFormWedge::new(self.dim, result_terms)
    }
    /// Scale by a scalar.
    pub fn scale(&self, s: f64) -> DifferentialFormWedge {
        let terms = self.terms.iter().map(|(c, i)| (c * s, i.clone())).collect();
        DifferentialFormWedge::new(self.dim, terms)
    }
    /// Add two forms.
    pub fn add(&self, other: &DifferentialFormWedge) -> DifferentialFormWedge {
        assert_eq!(self.dim, other.dim, "dimension mismatch in form addition");
        let mut terms = self.terms.clone();
        terms.extend_from_slice(&other.terms);
        DifferentialFormWedge::new(self.dim, terms)
    }
    /// Degree (number of indices in first term, or 0 for zero form).
    pub fn degree(&self) -> usize {
        self.terms.first().map_or(0, |(_, i)| i.len())
    }
    /// Check if this is the zero form.
    pub fn is_zero(&self) -> bool {
        self.terms.is_empty()
    }
}
/// A Randers-type Finsler metric on R: F(x, v) = α(x)|v| + β(x)·v
///
/// where α(x) > 0 and |β(x)| < α(x) for strong convexity.
/// This is the simplest non-reversible Finsler metric.
#[allow(dead_code)]
pub struct RandersFinsler {
    /// Riemannian part α (constant here for simplicity)
    pub alpha: f64,
    /// 1-form part β (constant)
    pub beta: f64,
}
#[allow(dead_code)]
impl RandersFinsler {
    /// Create a Randers metric. Requires |beta| < alpha.
    pub fn new(alpha: f64, beta: f64) -> Option<Self> {
        if beta.abs() < alpha && alpha > 0.0 {
            Some(Self { alpha, beta })
        } else {
            None
        }
    }
    /// Finsler norm F(v) = alpha |v| + beta v.
    pub fn norm(&self, v: f64) -> f64 {
        self.alpha * v.abs() + self.beta * v
    }
    /// Check strong convexity: F > 0 for v ≠ 0.
    pub fn is_strongly_convex(&self) -> bool {
        self.alpha + self.beta > 0.0 && self.alpha - self.beta > 0.0
    }
    /// Fundamental tensor (second derivative of F²/2 w.r.t. v): g = α² + αβ sgn(v)
    pub fn fundamental_tensor(&self, v: f64) -> f64 {
        if v.abs() < 1e-12 {
            return self.alpha * self.alpha;
        }
        let sign_v = if v > 0.0 { 1.0 } else { -1.0 };
        let factor = self.alpha * sign_v + self.beta;
        factor * factor
    }
}
/// A point on a surface parameterized as (u,v) → (x,y,z)
pub struct SurfacePoint {
    pub u: f64,
    pub v: f64,
    pub position: [f64; 3],
}
/// Riemannian metric tensor at a point (as a symmetric positive-definite matrix).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RiemannMetric {
    pub dim: usize,
    pub components: Vec<Vec<f64>>,
}
#[allow(dead_code)]
impl RiemannMetric {
    pub fn new(dim: usize) -> Self {
        let components = (0..dim)
            .map(|i| (0..dim).map(|j| if i == j { 1.0 } else { 0.0 }).collect())
            .collect();
        RiemannMetric { dim, components }
    }
    pub fn euclidean(dim: usize) -> Self {
        Self::new(dim)
    }
    pub fn set_component(&mut self, i: usize, j: usize, val: f64) {
        self.components[i][j] = val;
        self.components[j][i] = val;
    }
    /// Compute determinant (for 2x2 only, simplified).
    pub fn det_2d(&self) -> Option<f64> {
        if self.dim == 2 {
            Some(
                self.components[0][0] * self.components[1][1]
                    - self.components[0][1] * self.components[1][0],
            )
        } else {
            None
        }
    }
    pub fn inner_product(&self, u: &[f64], v: &[f64]) -> f64 {
        assert_eq!(u.len(), self.dim);
        assert_eq!(v.len(), self.dim);
        let mut result = 0.0;
        for i in 0..self.dim {
            for j in 0..self.dim {
                result += self.components[i][j] * u[i] * v[j];
            }
        }
        result
    }
    pub fn norm(&self, v: &[f64]) -> f64 {
        self.inner_product(v, v).sqrt()
    }
}
/// Lie derivative of a tensor field along a vector field (abstract).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LieDerivative {
    pub vector_field: String,
    pub tensor_name: String,
    pub result_name: String,
}
#[allow(dead_code)]
impl LieDerivative {
    pub fn new(v: &str, t: &str) -> Self {
        LieDerivative {
            vector_field: v.to_string(),
            tensor_name: t.to_string(),
            result_name: format!("L_{}({})", v, t),
        }
    }
    /// Cartan's magic formula: L_X ω = i_X(dω) + d(i_X ω).
    pub fn cartan_formula() -> &'static str {
        "L_X = i_X ∘ d + d ∘ i_X"
    }
    /// L_X f = X(f) for a smooth function f.
    pub fn on_function(v: &str, f: &str) -> String {
        format!("{}({})", v, f)
    }
}
/// Element of SO(3): 3×3 rotation matrix stored row-major.
#[derive(Clone, Debug)]
pub struct LieGroupSO3 {
    /// Row-major 3×3 rotation matrix
    pub matrix: [[f64; 3]; 3],
}
impl LieGroupSO3 {
    /// Identity rotation.
    pub fn identity() -> Self {
        Self {
            matrix: [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
        }
    }
    /// Rodrigues' rotation formula: R(ω) = I + sin(θ)/θ \[ω\]× + (1-cos(θ))/θ² \[ω\]×²
    ///
    /// where ω is the rotation axis-angle vector (|ω| = θ).
    pub fn from_axis_angle(omega: &[f64; 3]) -> Self {
        let theta = norm3(omega);
        if theta < 1e-12 {
            return Self::identity();
        }
        let k = [omega[0] / theta, omega[1] / theta, omega[2] / theta];
        let s = theta.sin();
        let c = theta.cos();
        let one_minus_c = 1.0 - c;
        let matrix = [
            [
                c + k[0] * k[0] * one_minus_c,
                k[0] * k[1] * one_minus_c - k[2] * s,
                k[0] * k[2] * one_minus_c + k[1] * s,
            ],
            [
                k[1] * k[0] * one_minus_c + k[2] * s,
                c + k[1] * k[1] * one_minus_c,
                k[1] * k[2] * one_minus_c - k[0] * s,
            ],
            [
                k[2] * k[0] * one_minus_c - k[1] * s,
                k[2] * k[1] * one_minus_c + k[0] * s,
                c + k[2] * k[2] * one_minus_c,
            ],
        ];
        Self { matrix }
    }
    /// Apply rotation to a 3-vector v → R·v.
    pub fn apply(&self, v: &[f64; 3]) -> [f64; 3] {
        let r = &self.matrix;
        [
            r[0][0] * v[0] + r[0][1] * v[1] + r[0][2] * v[2],
            r[1][0] * v[0] + r[1][1] * v[1] + r[1][2] * v[2],
            r[2][0] * v[0] + r[2][1] * v[1] + r[2][2] * v[2],
        ]
    }
    /// Compose two rotations: (self * other).apply(v) = self.apply(other.apply(v))
    pub fn compose(&self, other: &LieGroupSO3) -> LieGroupSO3 {
        let a = &self.matrix;
        let b = &other.matrix;
        let mut c = [[0.0f64; 3]; 3];
        for i in 0..3 {
            for j in 0..3 {
                for k in 0..3 {
                    c[i][j] += a[i][k] * b[k][j];
                }
            }
        }
        LieGroupSO3 { matrix: c }
    }
    /// Transpose = inverse for SO(3).
    pub fn transpose(&self) -> LieGroupSO3 {
        let m = &self.matrix;
        LieGroupSO3 {
            matrix: [
                [m[0][0], m[1][0], m[2][0]],
                [m[0][1], m[1][1], m[2][1]],
                [m[0][2], m[1][2], m[2][2]],
            ],
        }
    }
    /// Extract the axis-angle vector ω from R using the logarithm map.
    ///
    /// Returns ω such that from_axis_angle(ω) ≈ self.
    pub fn log_map(&self) -> [f64; 3] {
        let m = &self.matrix;
        let trace = m[0][0] + m[1][1] + m[2][2];
        let cos_theta = ((trace - 1.0) / 2.0).clamp(-1.0, 1.0);
        let theta = cos_theta.acos();
        if theta.abs() < 1e-12 {
            return [0.0, 0.0, 0.0];
        }
        let factor = theta / (2.0 * theta.sin());
        [
            factor * (m[2][1] - m[1][2]),
            factor * (m[0][2] - m[2][0]),
            factor * (m[1][0] - m[0][1]),
        ]
    }
    /// Check if the matrix is a valid rotation (det ≈ 1, R·Rᵀ ≈ I).
    pub fn is_valid(&self) -> bool {
        let rt = self.transpose();
        let prod = self.compose(&rt);
        let m = &prod.matrix;
        let identity_err = (m[0][0] - 1.0).abs()
            + (m[1][1] - 1.0).abs()
            + (m[2][2] - 1.0).abs()
            + m[0][1].abs()
            + m[0][2].abs()
            + m[1][0].abs()
            + m[1][2].abs()
            + m[2][0].abs()
            + m[2][1].abs();
        identity_err < 1e-10
    }
}
/// Geodesic on a Riemannian manifold (approximated by straight lines for flat space).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Geodesic {
    pub start: Vec<f64>,
    pub end: Vec<f64>,
    pub n_steps: usize,
}
#[allow(dead_code)]
impl Geodesic {
    pub fn new(start: Vec<f64>, end: Vec<f64>, steps: usize) -> Self {
        assert_eq!(start.len(), end.len());
        Geodesic {
            start,
            end,
            n_steps: steps,
        }
    }
    pub fn dim(&self) -> usize {
        self.start.len()
    }
    pub fn point_at(&self, t: f64) -> Vec<f64> {
        self.start
            .iter()
            .zip(self.end.iter())
            .map(|(s, e)| s + t * (e - s))
            .collect()
    }
    pub fn euclidean_length(&self) -> f64 {
        self.start
            .iter()
            .zip(self.end.iter())
            .map(|(s, e)| (e - s).powi(2))
            .sum::<f64>()
            .sqrt()
    }
    pub fn sample_points(&self) -> Vec<Vec<f64>> {
        (0..=self.n_steps)
            .map(|i| {
                let t = i as f64 / self.n_steps as f64;
                self.point_at(t)
            })
            .collect()
    }
}
/// A 3D Riemannian metric g_{ij}(x) given as a symmetric 3×3 matrix.
pub struct RiemannianMetric3D {
    /// Symmetric 3×3 metric tensor g\[i\]\[j\]
    pub g: [[f64; 3]; 3],
}
impl RiemannianMetric3D {
    pub fn new(g: [[f64; 3]; 3]) -> Self {
        Self { g }
    }
    /// Flat Euclidean metric on R^3
    pub fn euclidean() -> Self {
        Self {
            g: [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
        }
    }
    /// Determinant of a 3×3 matrix via cofactor expansion.
    pub fn det(&self) -> f64 {
        let g = &self.g;
        g[0][0] * (g[1][1] * g[2][2] - g[1][2] * g[2][1])
            - g[0][1] * (g[1][0] * g[2][2] - g[1][2] * g[2][0])
            + g[0][2] * (g[1][0] * g[2][1] - g[1][1] * g[2][0])
    }
    /// Inverse metric g^{ij} using the adjugate / cofactor formula.
    pub fn inverse(&self) -> [[f64; 3]; 3] {
        let g = &self.g;
        let d = self.det();
        if d.abs() < 1e-15 {
            return [[0.0; 3]; 3];
        }
        let mut inv = [[0.0f64; 3]; 3];
        inv[0][0] = (g[1][1] * g[2][2] - g[1][2] * g[2][1]) / d;
        inv[0][1] = (g[0][2] * g[2][1] - g[0][1] * g[2][2]) / d;
        inv[0][2] = (g[0][1] * g[1][2] - g[0][2] * g[1][1]) / d;
        inv[1][0] = (g[1][2] * g[2][0] - g[1][0] * g[2][2]) / d;
        inv[1][1] = (g[0][0] * g[2][2] - g[0][2] * g[2][0]) / d;
        inv[1][2] = (g[0][2] * g[1][0] - g[0][0] * g[1][2]) / d;
        inv[2][0] = (g[1][0] * g[2][1] - g[1][1] * g[2][0]) / d;
        inv[2][1] = (g[0][1] * g[2][0] - g[0][0] * g[2][1]) / d;
        inv[2][2] = (g[0][0] * g[1][1] - g[0][1] * g[1][0]) / d;
        inv
    }
    /// Christoffel symbols Γ^k_ij given metric partial derivatives dg\[i\]\[j\]\[k\] = ∂_k g_{ij}.
    ///
    /// Returns gamma\[k\]\[i\]\[j\] = (1/2) g^{kl} (∂_i g_{jl} + ∂_j g_{il} - ∂_l g_{ij}).
    pub fn christoffel(&self, dg: &[[[f64; 3]; 3]; 3]) -> [[[f64; 3]; 3]; 3] {
        let g_inv = self.inverse();
        let mut gamma = [[[0.0f64; 3]; 3]; 3];
        for k in 0..3 {
            for i in 0..3 {
                for j in 0..3 {
                    let mut val = 0.0;
                    for l in 0..3 {
                        let term = dg[j][l][i] + dg[i][l][j] - dg[i][j][l];
                        val += g_inv[k][l] * term;
                    }
                    gamma[k][i][j] = 0.5 * val;
                }
            }
        }
        gamma
    }
    /// Volume element sqrt(det g).
    pub fn volume_element(&self) -> f64 {
        self.det().abs().sqrt()
    }
}
/// Differential form of degree k on an n-dimensional manifold.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DifferentialForm {
    pub degree: usize,
    pub ambient_dim: usize,
    pub is_closed: bool,
    pub is_exact: bool,
}
#[allow(dead_code)]
impl DifferentialForm {
    pub fn new(degree: usize, n: usize) -> Self {
        DifferentialForm {
            degree,
            ambient_dim: n,
            is_closed: false,
            is_exact: false,
        }
    }
    pub fn zero_form(n: usize) -> Self {
        DifferentialForm::new(0, n)
    }
    pub fn volume_form(n: usize) -> Self {
        let mut f = DifferentialForm::new(n, n);
        f.is_closed = true;
        f
    }
    /// Exact forms are closed: d(dω) = 0.
    pub fn mark_exact(&mut self) {
        self.is_exact = true;
        self.is_closed = true;
    }
    /// Dimension of the space of k-forms on R^n: C(n,k).
    pub fn space_dimension(&self) -> usize {
        let n = self.ambient_dim;
        let k = self.degree;
        if k > n {
            return 0;
        }
        let mut result = 1usize;
        for i in 0..k {
            result = result * (n - i) / (i + 1);
        }
        result
    }
    pub fn is_top_form(&self) -> bool {
        self.degree == self.ambient_dim
    }
}
/// Approximate holonomy by integrating parallel transport around a small loop.
///
/// Uses the formula: Hol(γ) ≈ exp(-∫∫_D R^{1}_{2 12} du dv)
/// for a 2D manifold with Gaussian curvature K.
#[allow(dead_code)]
pub struct HolonomyComputer {
    /// Christoffel symbols Γ\[k\]\[i\]\[j\] at the base point
    pub christoffel: [[[f64; 2]; 2]; 2],
}
#[allow(dead_code)]
impl HolonomyComputer {
    pub fn new(christoffel: [[[f64; 2]; 2]; 2]) -> Self {
        Self { christoffel }
    }
    /// Parallel transport a 2D vector v along the direction (du, dv) for a small step.
    ///
    /// dv^k = -Γ^k_{ij} v^i dx^j
    pub fn parallel_transport_step(&self, v: &[f64; 2], dx: &[f64; 2]) -> [f64; 2] {
        let gamma = &self.christoffel;
        let mut dv = [0.0f64; 2];
        for k in 0..2 {
            for i in 0..2 {
                for j in 0..2 {
                    dv[k] -= gamma[k][i][j] * v[i] * dx[j];
                }
            }
        }
        [v[0] + dv[0], v[1] + dv[1]]
    }
    /// Approximate holonomy angle around a small square loop of size ε.
    ///
    /// Returns the rotation angle φ such that Hol ≈ R(φ).
    /// Uses: φ ≈ R^{1}_{2 12} · ε²  (area law).
    pub fn holonomy_angle_square_loop(&self, eps: f64) -> f64 {
        let gamma = &self.christoffel;
        let v0 = [1.0_f64, 0.0_f64];
        let steps = [
            [eps, 0.0_f64],
            [0.0_f64, eps],
            [-eps, 0.0_f64],
            [0.0_f64, -eps],
        ];
        let mut v = v0;
        for dx in &steps {
            v = self.parallel_transport_step(&v, dx);
        }
        let cos_phi = v[0] * v0[0] + v[1] * v0[1];
        let sin_phi = v[1] * v0[0] - v[0] * v0[1];
        let _ = gamma;
        sin_phi.atan2(cos_phi)
    }
}
/// A 2D Riemannian metric g_ij(u,v) given by coefficient functions.
///
/// The metric tensor g = [\[g00, g01\], \[g10, g11\]] with g10 = g01.
/// Used to compute Christoffel symbols Γ^k_ij and geodesic acceleration.
pub struct RiemannianMetric2D {
    /// g_00 component
    pub g00: f64,
    /// g_01 = g_10 component
    pub g01: f64,
    /// g_11 component
    pub g11: f64,
}
impl RiemannianMetric2D {
    pub fn new(g00: f64, g01: f64, g11: f64) -> Self {
        Self { g00, g01, g11 }
    }
    /// Flat Euclidean metric on R^2
    pub fn euclidean() -> Self {
        Self::new(1.0, 0.0, 1.0)
    }
    /// Determinant of g
    pub fn det(&self) -> f64 {
        self.g00 * self.g11 - self.g01 * self.g01
    }
    /// Inverse metric g^{ij}
    pub fn inverse(&self) -> [[f64; 2]; 2] {
        let d = self.det();
        if d.abs() < 1e-15 {
            return [[0.0; 2]; 2];
        }
        [[self.g11 / d, -self.g01 / d], [-self.g01 / d, self.g00 / d]]
    }
    /// Christoffel symbols Γ^k_ij from finite-difference perturbation of the metric.
    ///
    /// This version accepts partial derivatives of g_ij: dg\[i\]\[j\]\[k\] = ∂_k g_ij.
    /// Returns Γ\[k\]\[i\]\[j\] = (1/2) g^{kl} (∂_i g_{jl} + ∂_j g_{il} - ∂_l g_{ij}).
    pub fn christoffel(&self, dg: &[[[f64; 2]; 2]; 2]) -> [[[f64; 2]; 2]; 2] {
        let g_inv = self.inverse();
        let mut gamma = [[[0.0f64; 2]; 2]; 2];
        for k in 0..2 {
            for i in 0..2 {
                for j in 0..2 {
                    let mut val = 0.0;
                    for l in 0..2 {
                        let term = dg[j][l][i] + dg[i][l][j] - dg[i][j][l];
                        val += g_inv[k][l] * term;
                    }
                    gamma[k][i][j] = 0.5 * val;
                }
            }
        }
        gamma
    }
    /// Geodesic acceleration: ẍ^k = -Γ^k_ij ẋ^i ẋ^j
    pub fn geodesic_acceleration(&self, gamma: &[[[f64; 2]; 2]; 2], vel: &[f64; 2]) -> [f64; 2] {
        let mut acc = [0.0f64; 2];
        for k in 0..2 {
            for i in 0..2 {
                for j in 0..2 {
                    acc[k] -= gamma[k][i][j] * vel[i] * vel[j];
                }
            }
        }
        acc
    }
}
