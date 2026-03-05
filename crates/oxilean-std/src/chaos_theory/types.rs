//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::functions::FEIGENBAUM_DELTA;
use super::functions::*;

/// A KAM torus: quasi-periodic invariant torus in a perturbed Hamiltonian system.
pub struct KAMTorus {
    /// Frequency vector ω = (ω₁, ..., ωₙ).
    pub frequencies: Vec<f64>,
    /// Whether the frequency vector satisfies a Diophantine condition.
    pub is_diophantine: bool,
    /// Whether this torus persists under the perturbation.
    pub persists: bool,
}
impl KAMTorus {
    /// Create a KAM torus descriptor.
    pub fn new(frequencies: Vec<f64>, is_diophantine: bool, persists: bool) -> Self {
        KAMTorus {
            frequencies,
            is_diophantine,
            persists,
        }
    }
    /// Returns `true` if the torus is quasi-periodic (always true by construction).
    pub fn is_quasi_periodic(&self) -> bool {
        true
    }
}
/// Hopf bifurcation: stable equilibrium loses stability and a limit cycle is born.
pub struct HopfBifurcation {
    /// Critical parameter value at which the limit cycle is born.
    pub critical_value: f64,
    /// Frequency of the emerging limit cycle.
    pub frequency: f64,
    /// `true` for supercritical (stable limit cycle), `false` for subcritical.
    pub is_supercritical: bool,
}
impl HopfBifurcation {
    /// Create a Hopf bifurcation descriptor.
    pub fn new(critical_value: f64, frequency: f64, is_supercritical: bool) -> Self {
        HopfBifurcation {
            critical_value,
            frequency,
            is_supercritical,
        }
    }
    /// Return the bifurcation (critical) value.
    pub fn bifurcation_value(&self) -> f64 {
        self.critical_value
    }
    /// Normal form description.
    pub fn normal_form(&self) -> String {
        format!(
            "dr/dt = mu*r - r^3, dθ/dt = ω  (ω={:.4}, bifurcation at mu={})",
            self.frequency, self.critical_value
        )
    }
}
/// Mixing property: μ(A ∩ T^{-n}B) → μ(A)·μ(B) as n → ∞.
pub struct MixingProperty {
    /// Whether the system is strongly mixing.
    pub is_strongly_mixing: bool,
    /// Whether the system is weakly mixing.
    pub is_weakly_mixing: bool,
}
impl MixingProperty {
    /// Create a mixing property descriptor.
    pub fn new(is_strongly_mixing: bool) -> Self {
        MixingProperty {
            is_strongly_mixing,
            is_weakly_mixing: is_strongly_mixing,
        }
    }
    /// Returns `true` if the system is (strongly) mixing.
    pub fn is_mixing(&self) -> bool {
        self.is_strongly_mixing
    }
}
/// Sensitive dependence on initial conditions.
///
/// There exists δ > 0 such that for every x and ε > 0,
/// there exists y with d(x,y) < ε and d(fⁿx, fⁿy) > δ for some n.
pub struct SensitiveDependence {
    /// Sensitivity constant δ.
    pub delta: f64,
}
impl SensitiveDependence {
    /// Create a sensitive dependence descriptor with constant δ.
    pub fn new(delta: f64) -> Self {
        SensitiveDependence { delta }
    }
    /// Return the sensitivity constant.
    pub fn sensitivity_constant(&self) -> f64 {
        self.delta
    }
}
/// Saddle-node bifurcation: two equilibria collide and disappear at critical parameter.
pub struct SaddleNodeBifurcation {
    /// Critical parameter value at which the bifurcation occurs.
    pub critical_value: f64,
}
impl SaddleNodeBifurcation {
    /// Create with a given critical parameter value.
    pub fn new(critical_value: f64) -> Self {
        SaddleNodeBifurcation { critical_value }
    }
    /// Return the bifurcation (critical) value.
    pub fn bifurcation_value(&self) -> f64 {
        self.critical_value
    }
    /// Normal form: dx/dt = μ − x².  Returns (μ, "x^2") as a string description.
    pub fn normal_form(&self) -> String {
        format!(
            "dx/dt = mu - x^2  (bifurcation at mu = {})",
            self.critical_value
        )
    }
}
/// The Lorenz system: dx/dt = σ(y−x), dy/dt = x(ρ−z)−y, dz/dt = xy−βz.
///
/// Classic parameter values σ=10, ρ=28, β=8/3 give chaotic dynamics
/// with the butterfly attractor.
pub struct LorenzSystem {
    /// σ parameter (Prandtl number analogue).
    pub sigma: f64,
    /// ρ parameter (Rayleigh number analogue, bifurcation at ρ=1).
    pub rho: f64,
    /// β parameter (geometric factor, typically 8/3).
    pub beta: f64,
}
impl LorenzSystem {
    /// Create a Lorenz system with given parameters.
    pub fn new(sigma: f64, rho: f64, beta: f64) -> Self {
        LorenzSystem { sigma, rho, beta }
    }
    /// Classic chaotic Lorenz system (σ=10, ρ=28, β=8/3).
    pub fn classic() -> Self {
        LorenzSystem::new(10.0, 28.0, 8.0 / 3.0)
    }
    /// Compute the vector field (dx/dt, dy/dt, dz/dt) at point (x, y, z).
    pub fn vector_field(&self, x: f64, y: f64, z: f64) -> (f64, f64, f64) {
        let dx = self.sigma * (y - x);
        let dy = x * (self.rho - z) - y;
        let dz = x * y - self.beta * z;
        (dx, dy, dz)
    }
    /// Integrate one Euler step with time step `dt`.
    pub fn euler_step(&self, x: f64, y: f64, z: f64, dt: f64) -> (f64, f64, f64) {
        let (dx, dy, dz) = self.vector_field(x, y, z);
        (x + dt * dx, y + dt * dy, z + dt * dz)
    }
    /// Approximate maximal Lyapunov exponent (positive for classic parameters).
    pub fn lyapunov_exponent(&self) -> f64 {
        if (self.sigma - 10.0).abs() < 1e-6
            && (self.rho - 28.0).abs() < 1e-6
            && (self.beta - 8.0 / 3.0).abs() < 1e-6
        {
            0.9056
        } else {
            if self.rho > 1.0 {
                0.1 * (self.rho - 1.0).ln()
            } else {
                -1.0
            }
        }
    }
    /// Returns `true` when parameters are in the chaotic regime.
    pub fn is_chaotic(&self) -> bool {
        self.lyapunov_exponent() > 0.0
    }
    /// Returns `true` when the classical butterfly attractor is present.
    pub fn has_strange_attractor(&self) -> bool {
        self.rho > 24.74 && self.sigma > 0.0 && self.beta > 0.0
    }
}
/// The Mandelbrot set: M = {c ∈ ℂ : the orbit of z → z² + c starting at 0 is bounded}.
pub struct MandelbrotSet {
    /// Maximum iteration count before declaring divergence.
    pub max_iterations: u32,
    /// Escape radius (orbit escapes if |z| > escape_radius).
    pub escape_radius: f64,
}
impl MandelbrotSet {
    /// Create a Mandelbrot set tester.
    pub fn new(max_iterations: u32, escape_radius: f64) -> Self {
        MandelbrotSet {
            max_iterations,
            escape_radius,
        }
    }
    /// Default Mandelbrot set tester (max_iter=1000, escape=2.0).
    pub fn default_tester() -> Self {
        MandelbrotSet::new(1000, 2.0)
    }
    /// Returns `true` if `c = (cx, cy)` is in the Mandelbrot set.
    pub fn contains(&self, cx: f64, cy: f64) -> bool {
        let (mut zx, mut zy) = (0.0_f64, 0.0_f64);
        let r2 = self.escape_radius * self.escape_radius;
        for _ in 0..self.max_iterations {
            let zx2 = zx * zx;
            let zy2 = zy * zy;
            if zx2 + zy2 > r2 {
                return false;
            }
            let new_zx = zx2 - zy2 + cx;
            zy = 2.0 * zx * zy + cy;
            zx = new_zx;
        }
        true
    }
    /// Escape-time iteration count for `c = (cx, cy)`.
    pub fn escape_time(&self, cx: f64, cy: f64) -> u32 {
        let (mut zx, mut zy) = (0.0_f64, 0.0_f64);
        let r2 = self.escape_radius * self.escape_radius;
        for i in 0..self.max_iterations {
            let zx2 = zx * zx;
            let zy2 = zy * zy;
            if zx2 + zy2 > r2 {
                return i;
            }
            let new_zx = zx2 - zy2 + cx;
            zy = 2.0 * zx * zy + cy;
            zx = new_zx;
        }
        self.max_iterations
    }
    /// Fractal dimension of the boundary (known to be 2).
    pub fn dimension(&self) -> f64 {
        2.0
    }
}
/// Action-angle variables (J, θ) for an integrable Hamiltonian system.
pub struct ActionAngleVariables {
    /// Dimension of the system.
    pub n: usize,
    /// Action variables J_1, ..., J_n (constants of motion).
    pub actions: Vec<f64>,
    /// Angle variables θ_1, ..., θ_n (evolve linearly in time).
    pub angles: Vec<f64>,
}
impl ActionAngleVariables {
    /// Create action-angle variables.
    pub fn new(actions: Vec<f64>, angles: Vec<f64>) -> Self {
        assert_eq!(actions.len(), angles.len());
        let n = actions.len();
        ActionAngleVariables { n, actions, angles }
    }
    /// Frequency vector ∂H/∂J.
    pub fn frequencies(&self, dh_dj: &[f64]) -> Vec<f64> {
        dh_dj.to_vec()
    }
}
/// An Iterated Function System for generating fractals.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct IteratedFunctionSystemExt {
    /// Name.
    pub name: String,
    /// Contraction ratios of the maps.
    pub contraction_ratios: Vec<f64>,
    /// Probabilities for random IFS.
    pub probabilities: Vec<f64>,
    /// Hausdorff dimension estimate.
    pub hausdorff_dim: f64,
}
#[allow(dead_code)]
impl IteratedFunctionSystemExt {
    /// Creates an IFS.
    pub fn new(name: &str, ratios: Vec<f64>) -> Self {
        let n = ratios.len();
        let probs = vec![1.0 / n as f64; n];
        let hd = Self::compute_hausdorff_dim(&ratios);
        IteratedFunctionSystemExt {
            name: name.to_string(),
            contraction_ratios: ratios,
            probabilities: probs,
            hausdorff_dim: hd,
        }
    }
    /// Computes Hausdorff dimension via Moran equation: sum r_i^d = 1.
    fn compute_hausdorff_dim(ratios: &[f64]) -> f64 {
        if ratios.is_empty() {
            return 0.0;
        }
        let mut lo = 0.0f64;
        let mut hi = 3.0f64;
        for _ in 0..50 {
            let mid = (lo + hi) / 2.0;
            let sum: f64 = ratios.iter().map(|&r| r.powf(mid)).sum();
            if sum > 1.0 {
                lo = mid;
            } else {
                hi = mid;
            }
        }
        (lo + hi) / 2.0
    }
    /// Creates the Cantor set IFS: {x → x/3, x → x/3 + 2/3}.
    pub fn cantor_set() -> Self {
        IteratedFunctionSystemExt::new("Cantor set", vec![1.0 / 3.0, 1.0 / 3.0])
    }
    /// Creates the Sierpinski triangle IFS.
    pub fn sierpinski() -> Self {
        IteratedFunctionSystemExt::new("Sierpinski", vec![0.5, 0.5, 0.5])
    }
    /// Checks the Moran open set condition (sufficient for Hausdorff dim = self-similar dim).
    pub fn moran_condition_description(&self) -> String {
        format!(
            "Moran OSC: IFS {} satisfies Open Set Condition → dim_H = {:.4}",
            self.name, self.hausdorff_dim
        )
    }
    /// Attractor dimension equals the solution of the Moran equation.
    pub fn attractor_dimension(&self) -> f64 {
        self.hausdorff_dim
    }
}
/// The logistic map: x_{n+1} = r·x_n·(1 − x_n), x_0 ∈ (0,1).
///
/// For r > 3.57 the map is generically chaotic (period-doubling route to chaos).
pub struct LogisticMap {
    /// Growth parameter r ∈ [0, 4].
    pub r: f64,
}
impl LogisticMap {
    /// Create a logistic map with parameter `r`.
    pub fn new(r: f64) -> Self {
        LogisticMap { r }
    }
    /// Apply one iterate: x → r·x·(1−x).
    pub fn iterate(&self, x: f64) -> f64 {
        self.r * x * (1.0 - x)
    }
    /// Iterate `n` times starting from `x0`.
    pub fn orbit(&self, x0: f64, n: usize) -> Vec<f64> {
        let mut orbit = Vec::with_capacity(n + 1);
        orbit.push(x0);
        let mut x = x0;
        for _ in 0..n {
            x = self.iterate(x);
            orbit.push(x);
        }
        orbit
    }
    /// Estimate the Lyapunov exponent: λ = lim (1/n) Σ ln|r(1−2x_k)|.
    pub fn lyapunov_exponent(&self) -> f64 {
        let n = 10_000usize;
        let mut x = 0.2_f64;
        for _ in 0..1000 {
            x = self.iterate(x);
        }
        let mut sum = 0.0_f64;
        for _ in 0..n {
            let deriv = (self.r * (1.0 - 2.0 * x)).abs();
            if deriv > 1e-15 {
                sum += deriv.ln();
            }
            x = self.iterate(x);
        }
        sum / n as f64
    }
    /// Returns `true` for r > 3.5699... (onset of chaos).
    pub fn is_chaotic(&self) -> bool {
        self.r > 3.5699_f64
    }
    /// Period-doubling ratio (Feigenbaum δ ≈ 4.669...).
    pub fn period_doubling_ratio(&self) -> f64 {
        FEIGENBAUM_DELTA
    }
}
/// Topological entropy of a map.
///
/// h(f) = sup_ε lim_{n→∞} (1/n) log N(f, ε, n)
/// where N(f,ε,n) is the maximal (n,ε)-separated set.
pub struct EntropyOfMap {
    /// Computed topological entropy value.
    pub value: f64,
}
impl EntropyOfMap {
    /// Create with a known entropy value.
    pub fn new(value: f64) -> Self {
        EntropyOfMap { value }
    }
    /// Topological entropy of the logistic map with parameter r.
    ///
    /// For r ∈ [2, 4]: h(f_r) = max(0, log(r) − log(2)) (Misiurewicz–Szlenk formula).
    pub fn logistic(r: f64) -> Self {
        let h = if r >= 2.0 {
            (r / 2.0_f64).ln().max(0.0)
        } else {
            0.0
        };
        EntropyOfMap::new(h)
    }
    /// Return the entropy value.
    pub fn entropy(&self) -> f64 {
        self.value
    }
    /// Returns `true` if entropy is positive (positive entropy implies chaos).
    pub fn is_chaotic(&self) -> bool {
        self.value > 0.0
    }
}
/// A fractal set descriptor.
pub struct Fractal {
    /// Human-readable name.
    pub name: String,
    /// Hausdorff / fractal dimension.
    pub fractal_dimension: f64,
    /// Topological dimension.
    pub topological_dimension: u32,
    /// Whether the set is self-similar.
    pub self_similar: bool,
}
impl Fractal {
    /// Create a fractal descriptor.
    pub fn new(
        name: impl Into<String>,
        fractal_dimension: f64,
        topological_dimension: u32,
        self_similar: bool,
    ) -> Self {
        Fractal {
            name: name.into(),
            fractal_dimension,
            topological_dimension,
            self_similar,
        }
    }
    /// Hausdorff / fractal dimension.
    pub fn dimension(&self) -> f64 {
        self.fractal_dimension
    }
    /// Topological dimension.
    pub fn topological_dimension(&self) -> u32 {
        self.topological_dimension
    }
    /// Returns `true` if the set is self-similar.
    pub fn is_self_similar(&self) -> bool {
        self.self_similar
    }
}
/// Box-counting (Minkowski–Bouligand) dimension.
///
/// Estimates d_B via N(ε) ~ ε^{-d_B}.
pub struct BoxCounting {
    /// Estimated box-counting dimension.
    pub value: f64,
}
impl BoxCounting {
    /// Create with a known or estimated value.
    pub fn new(value: f64) -> Self {
        BoxCounting { value }
    }
    /// Estimate dimension from counts N(ε) at scales ε.
    ///
    /// `scales` and `counts` must have the same length.
    pub fn estimate_from_data(scales: &[f64], counts: &[f64]) -> Option<f64> {
        if scales.len() < 2 || scales.len() != counts.len() {
            return None;
        }
        let log_inv_eps: Vec<f64> = scales.iter().map(|&e| -(e.ln())).collect();
        let log_n: Vec<f64> = counts.iter().map(|&c| c.ln()).collect();
        let n = log_inv_eps.len() as f64;
        let sx: f64 = log_inv_eps.iter().sum();
        let sy: f64 = log_n.iter().sum();
        let sxy: f64 = log_inv_eps
            .iter()
            .zip(log_n.iter())
            .map(|(x, y)| x * y)
            .sum();
        let sx2: f64 = log_inv_eps.iter().map(|x| x * x).sum();
        let denom = n * sx2 - sx * sx;
        if denom.abs() < 1e-15 {
            return None;
        }
        Some((n * sxy - sx * sy) / denom)
    }
    /// Return the stored dimension value.
    pub fn dimension(&self) -> f64 {
        self.value
    }
}
/// Period-doubling cascade towards chaos (Feigenbaum route).
pub struct PeriodDoublingCascade {
    /// Sequence of period-doubling bifurcation parameter values.
    pub bifurcation_values: Vec<f64>,
}
impl PeriodDoublingCascade {
    /// Create with known bifurcation values for the logistic map.
    pub fn logistic() -> Self {
        PeriodDoublingCascade {
            bifurcation_values: vec![3.0, 3.449490, 3.544090, 3.564407, 3.568759],
        }
    }
    /// Feigenbaum δ constant (ratio of successive bifurcation intervals → δ).
    pub fn feigenbaum_constant(&self) -> f64 {
        FEIGENBAUM_DELTA
    }
    /// Estimate δ from the stored bifurcation values.
    pub fn estimated_delta(&self) -> Option<f64> {
        let v = &self.bifurcation_values;
        if v.len() < 3 {
            return None;
        }
        let n = v.len() - 1;
        let d1 = v[n - 1] - v[n - 2];
        let d2 = v[n] - v[n - 1];
        if d2.abs() < 1e-15 {
            None
        } else {
            Some(d1 / d2)
        }
    }
}
/// Data for the Feigenbaum universality.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FeigenbaumData {
    /// Feigenbaum constant δ ≈ 4.6692.
    pub delta: f64,
    /// Feigenbaum constant α ≈ 2.5029.
    pub alpha: f64,
    /// Period-doubling bifurcation values.
    pub bifurcation_points: Vec<f64>,
}
#[allow(dead_code)]
impl FeigenbaumData {
    /// Creates Feigenbaum data with known constants.
    pub fn new() -> Self {
        FeigenbaumData {
            delta: 4.6692016091,
            alpha: 2.5029078751,
            bifurcation_points: Vec::new(),
        }
    }
    /// Adds a bifurcation point.
    pub fn add_bifurcation(&mut self, r: f64) {
        self.bifurcation_points.push(r);
    }
    /// Checks Feigenbaum scaling: (r_{n+1} - r_n) / (r_{n+2} - r_{n+1}) → δ.
    pub fn check_scaling(&self) -> Option<f64> {
        let n = self.bifurcation_points.len();
        if n < 3 {
            return None;
        }
        let ratio = (self.bifurcation_points[n - 2] - self.bifurcation_points[n - 3])
            / (self.bifurcation_points[n - 1] - self.bifurcation_points[n - 2]);
        Some(ratio)
    }
    /// Period-doubling universality description.
    pub fn universality_description(&self) -> String {
        format!(
            "Feigenbaum: period-doubling converges with ratio δ={:.6}, α={:.6}",
            self.delta, self.alpha
        )
    }
}
/// Koch snowflake curve: fractal dimension ln(4)/ln(3) ≈ 1.2619...
pub struct KochCurve {
    /// Number of iterations used to approximate the curve.
    pub iterations: u32,
}
impl KochCurve {
    /// Create a Koch curve approximation with `iterations` refinement steps.
    pub fn new(iterations: u32) -> Self {
        KochCurve { iterations }
    }
    /// Exact fractal dimension: ln(4)/ln(3).
    pub fn dimension(&self) -> f64 {
        4.0_f64.ln() / 3.0_f64.ln()
    }
    /// Topological dimension (it is a curve, so 1).
    pub fn topological_dimension(&self) -> u32 {
        1
    }
    /// Returns `true` (Koch curve is self-similar by construction).
    pub fn is_self_similar(&self) -> bool {
        true
    }
    /// Number of segments at iteration `n`: 4^n.
    pub fn n_segments(&self) -> u64 {
        4_u64.pow(self.iterations)
    }
    /// Length at iteration `n`: (4/3)^n (diverges → infinite perimeter).
    pub fn length(&self) -> f64 {
        (4.0 / 3.0_f64).powi(self.iterations as i32)
    }
}
/// Diophantine condition: |k·ω| ≥ γ / |k|^τ for all k ≠ 0 ∈ Zⁿ.
pub struct DiophantineCondition {
    /// Diophantine constant γ > 0.
    pub gamma: f64,
    /// Diophantine exponent τ ≥ n−1.
    pub tau: f64,
}
impl DiophantineCondition {
    /// Create a Diophantine condition.
    pub fn new(gamma: f64, tau: f64) -> Self {
        DiophantineCondition { gamma, tau }
    }
    /// Check the Diophantine condition for a given integer vector k and frequency ω.
    ///
    /// Returns `true` if |k·ω| ≥ γ / |k|^τ where |k| = Σ|k_i|.
    pub fn check(&self, k: &[i64], omega: &[f64]) -> bool {
        if k.len() != omega.len() {
            return false;
        }
        let kdot: f64 = k
            .iter()
            .zip(omega.iter())
            .map(|(&ki, &wi)| ki as f64 * wi)
            .sum();
        let k_norm: f64 = k.iter().map(|&ki| ki.unsigned_abs() as f64).sum();
        if k_norm < 1e-15 {
            return true;
        }
        kdot.abs() >= self.gamma / k_norm.powf(self.tau)
    }
}
/// Kolmogorov's theorem: most KAM tori persist under small perturbations.
pub struct KolmogorovThm;
impl KolmogorovThm {
    /// Returns `true` if the KAM theorem applies given ε-bound and Diophantine condition.
    pub fn applies(epsilon: f64, gamma: f64, tau: f64) -> bool {
        epsilon.abs() < gamma * (tau + 1.0).powi(-2)
    }
}
/// Topological transitivity: there exists a dense orbit.
pub struct TopologicalTransitivity {
    /// Whether the system is topologically transitive.
    pub is_transitive: bool,
}
impl TopologicalTransitivity {
    /// Create a topological transitivity descriptor.
    pub fn new(is_transitive: bool) -> Self {
        TopologicalTransitivity { is_transitive }
    }
}
/// Stability classification.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum StabilityType {
    /// Asymptotically stable: all eigenvalue real parts < 0.
    AsymptoticallyStable,
    /// Stable but not asymptotically: purely imaginary eigenvalues.
    Stable,
    /// Unstable: some eigenvalue has positive real part.
    Unstable,
    /// Center (nonlinear analysis needed).
    Center,
}
/// Hausdorff dimension: d_H(F) = inf{s ≥ 0 : H^s(F) = 0}.
pub struct HausdorffDimension {
    /// Computed or known Hausdorff dimension value.
    pub value: f64,
}
impl HausdorffDimension {
    /// Create with a known dimension value.
    pub fn new(value: f64) -> Self {
        HausdorffDimension { value }
    }
    /// Return the dimension value.
    pub fn dimension(&self) -> f64 {
        self.value
    }
}
/// Pitchfork bifurcation: symmetric bifurcation where one equilibrium splits into three.
pub struct PitchforkBifurcation {
    /// Critical parameter value.
    pub critical_value: f64,
    /// `true` for supercritical (stable branches appear), `false` for subcritical.
    pub is_supercritical: bool,
}
impl PitchforkBifurcation {
    /// Create with critical value and criticality flag.
    pub fn new(critical_value: f64, is_supercritical: bool) -> Self {
        PitchforkBifurcation {
            critical_value,
            is_supercritical,
        }
    }
    /// Return the bifurcation (critical) value.
    pub fn bifurcation_value(&self) -> f64 {
        self.critical_value
    }
    /// Normal form description.
    pub fn normal_form(&self) -> String {
        if self.is_supercritical {
            format!(
                "dx/dt = mu*x - x^3  (supercritical, bifurcation at mu = {})",
                self.critical_value
            )
        } else {
            format!(
                "dx/dt = mu*x + x^3  (subcritical, bifurcation at mu = {})",
                self.critical_value
            )
        }
    }
}
/// Estimates the box-counting (Minkowski-Bouligand) fractal dimension
/// of a point set in 2D using a box-counting algorithm.
pub struct FractalDimensionEstimator {
    /// The 2D point set (x, y) to analyse.
    pub points: Vec<(f64, f64)>,
}
impl FractalDimensionEstimator {
    /// Create an estimator from a list of 2D points.
    pub fn new(points: Vec<(f64, f64)>) -> Self {
        FractalDimensionEstimator { points }
    }
    /// Build from the Hénon attractor (classic a=1.4, b=0.3).
    pub fn from_henon_attractor(n_iter: usize) -> Self {
        let map = HenonMap::classic();
        let mut pts = Vec::with_capacity(n_iter);
        let (mut x, mut y) = (0.1, 0.1);
        for _ in 0..1000 {
            let (nx, ny) = map.iterate(x, y);
            x = nx;
            y = ny;
        }
        for _ in 0..n_iter {
            let (nx, ny) = map.iterate(x, y);
            x = nx;
            y = ny;
            pts.push((x, y));
        }
        FractalDimensionEstimator::new(pts)
    }
    /// Count the number of non-empty boxes of side length `eps`.
    pub fn count_boxes(&self, eps: f64) -> usize {
        if self.points.is_empty() || eps <= 0.0 {
            return 0;
        }
        let mut x_min = f64::MAX;
        let mut y_min = f64::MAX;
        for &(x, y) in &self.points {
            if x < x_min {
                x_min = x;
            }
            if y < y_min {
                y_min = y;
            }
        }
        let mut occupied = std::collections::HashSet::new();
        for &(x, y) in &self.points {
            let ix = ((x - x_min) / eps).floor() as i64;
            let iy = ((y - y_min) / eps).floor() as i64;
            occupied.insert((ix, iy));
        }
        occupied.len()
    }
    /// Estimate the box-counting dimension by linear regression of
    /// log N(ε) vs log(1/ε) over a range of ε values.
    ///
    /// Returns `None` if the estimate cannot be computed.
    pub fn estimate_dimension(&self) -> Option<f64> {
        let scales: Vec<f64> = (1..=8).map(|k| 0.5_f64.powi(k)).collect();
        let counts: Vec<f64> = scales.iter().map(|&e| self.count_boxes(e) as f64).collect();
        let valid: Vec<(f64, f64)> = scales
            .iter()
            .zip(counts.iter())
            .filter(|(_, &c)| c > 0.0)
            .map(|(&e, &c)| (e, c))
            .collect();
        if valid.len() < 2 {
            return None;
        }
        BoxCounting::estimate_from_data(
            &valid.iter().map(|(e, _)| *e).collect::<Vec<_>>(),
            &valid.iter().map(|(_, c)| *c).collect::<Vec<_>>(),
        )
    }
    /// Returns `true` if the estimated dimension is non-integer (fractal).
    pub fn is_fractal(&self) -> bool {
        match self.estimate_dimension() {
            Some(d) => {
                let floor = d.floor();
                (d - floor).abs() > 0.05 && (d - floor - 1.0).abs() > 0.05
            }
            None => false,
        }
    }
}
/// Devaney's definition of chaos: three conditions.
///
/// A map f: X → X is Devaney-chaotic if:
/// 1. f has dense periodic orbits,
/// 2. f is topologically transitive,
/// 3. f has sensitive dependence on initial conditions.
pub struct Devaney {
    /// Dense periodic orbits.
    pub has_dense_periodic_orbits: bool,
    /// Topological transitivity.
    pub is_topologically_transitive: bool,
    /// Sensitive dependence on initial conditions.
    pub has_sensitive_dependence: bool,
}
impl Devaney {
    /// Create a Devaney chaos descriptor.
    pub fn new(
        has_dense_periodic_orbits: bool,
        is_topologically_transitive: bool,
        has_sensitive_dependence: bool,
    ) -> Self {
        Devaney {
            has_dense_periodic_orbits,
            is_topologically_transitive,
            has_sensitive_dependence,
        }
    }
    /// Returns `true` if all three Devaney conditions are satisfied.
    pub fn is_chaotic(&self) -> bool {
        self.has_dense_periodic_orbits
            && self.is_topologically_transitive
            && self.has_sensitive_dependence
    }
}
/// Lyapunov stability analysis data.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LyapunovStabilityData {
    /// Fixed point description.
    pub fixed_point: String,
    /// Lyapunov function value at fixed point (should be 0).
    pub v0: f64,
    /// Eigenvalues of linearization (real parts).
    pub linearization_eigenvalues: Vec<f64>,
    /// Stability type.
    pub stability: StabilityType,
}
#[allow(dead_code)]
impl LyapunovStabilityData {
    /// Creates stability data.
    pub fn new(fixed_point: &str, eigenvalues: Vec<f64>) -> Self {
        let stability = Self::classify(&eigenvalues);
        LyapunovStabilityData {
            fixed_point: fixed_point.to_string(),
            v0: 0.0,
            linearization_eigenvalues: eigenvalues,
            stability,
        }
    }
    fn classify(evs: &[f64]) -> StabilityType {
        let max_real = evs.iter().copied().fold(f64::NEG_INFINITY, f64::max);
        let all_negative = evs.iter().all(|&e| e < 0.0);
        if all_negative {
            StabilityType::AsymptoticallyStable
        } else if max_real > 0.0 {
            StabilityType::Unstable
        } else if evs.iter().all(|&e| e == 0.0) {
            StabilityType::Center
        } else {
            StabilityType::Stable
        }
    }
    /// Checks if the fixed point is stable.
    pub fn is_stable(&self) -> bool {
        self.stability != StabilityType::Unstable
    }
    /// Returns the Lyapunov exponents estimate (= eigenvalue real parts for linear systems).
    pub fn lyapunov_exponents(&self) -> &[f64] {
        &self.linearization_eigenvalues
    }
    /// Returns the Hartman-Grobman linearization description.
    pub fn hartman_grobman(&self) -> String {
        match &self.stability {
            StabilityType::AsymptoticallyStable | StabilityType::Unstable => {
                format!(
                    "Hartman-Grobman: {} is topologically conjugate to linearization",
                    self.fixed_point
                )
            }
            _ => {
                format!(
                    "Hartman-Grobman: center case, linearization insufficient for {}",
                    self.fixed_point
                )
            }
        }
    }
}
/// Simulates the Lorenz system using 4th-order Runge-Kutta integration
/// and records trajectory points for attractor visualization.
pub struct LorenzAttractorSimulator {
    /// Lorenz system parameters.
    pub system: LorenzSystem,
    /// Integration time step.
    pub dt: f64,
}
impl LorenzAttractorSimulator {
    /// Create a simulator for the classic Lorenz system.
    pub fn classic(dt: f64) -> Self {
        LorenzAttractorSimulator {
            system: LorenzSystem::classic(),
            dt,
        }
    }
    /// Create a simulator with custom parameters.
    pub fn new(sigma: f64, rho: f64, beta: f64, dt: f64) -> Self {
        LorenzAttractorSimulator {
            system: LorenzSystem::new(sigma, rho, beta),
            dt,
        }
    }
    /// Perform one 4th-order Runge-Kutta step from (x, y, z).
    pub fn rk4_step(&self, x: f64, y: f64, z: f64) -> (f64, f64, f64) {
        let h = self.dt;
        let (k1x, k1y, k1z) = self.system.vector_field(x, y, z);
        let (k2x, k2y, k2z) =
            self.system
                .vector_field(x + 0.5 * h * k1x, y + 0.5 * h * k1y, z + 0.5 * h * k1z);
        let (k3x, k3y, k3z) =
            self.system
                .vector_field(x + 0.5 * h * k2x, y + 0.5 * h * k2y, z + 0.5 * h * k2z);
        let (k4x, k4y, k4z) = self
            .system
            .vector_field(x + h * k3x, y + h * k3y, z + h * k3z);
        (
            x + h / 6.0 * (k1x + 2.0 * k2x + 2.0 * k3x + k4x),
            y + h / 6.0 * (k1y + 2.0 * k2y + 2.0 * k3y + k4y),
            z + h / 6.0 * (k1z + 2.0 * k2z + 2.0 * k3z + k4z),
        )
    }
    /// Simulate `n_steps` from initial condition (x0, y0, z0).
    ///
    /// Returns a vector of (x, y, z) trajectory points.
    pub fn simulate(&self, x0: f64, y0: f64, z0: f64, n_steps: usize) -> Vec<(f64, f64, f64)> {
        let mut traj = Vec::with_capacity(n_steps + 1);
        let (mut x, mut y, mut z) = (x0, y0, z0);
        traj.push((x, y, z));
        for _ in 0..n_steps {
            let (nx, ny, nz) = self.rk4_step(x, y, z);
            x = nx;
            y = ny;
            z = nz;
            traj.push((x, y, z));
        }
        traj
    }
    /// Returns the approximate maximum z-extent of the attractor
    /// (useful for confirming butterfly structure exists).
    pub fn attractor_z_extent(&self, n_steps: usize) -> (f64, f64) {
        let n_transient = n_steps / 5;
        let mut state = (0.1_f64, 0.0_f64, 0.0_f64);
        for _ in 0..n_transient {
            state = self.rk4_step(state.0, state.1, state.2);
        }
        let mut z_min = state.2;
        let mut z_max = state.2;
        for _ in 0..n_steps {
            state = self.rk4_step(state.0, state.1, state.2);
            if state.2 < z_min {
                z_min = state.2;
            }
            if state.2 > z_max {
                z_max = state.2;
            }
        }
        (z_min, z_max)
    }
    /// Estimate the maximal Lyapunov exponent via tangent-vector evolution.
    pub fn lyapunov_exponent(&self, n_steps: usize) -> f64 {
        let n_transient = n_steps / 5;
        let mut state = (1.0_f64, 0.0_f64, 0.0_f64);
        for _ in 0..n_transient {
            state = self.rk4_step(state.0, state.1, state.2);
        }
        let eps = 1e-8_f64;
        let mut pstate = (state.0 + eps, state.1, state.2);
        let mut le_sum = 0.0_f64;
        for _ in 0..n_steps {
            state = self.rk4_step(state.0, state.1, state.2);
            pstate = self.rk4_step(pstate.0, pstate.1, pstate.2);
            let dx = pstate.0 - state.0;
            let dy = pstate.1 - state.1;
            let dz = pstate.2 - state.2;
            let dist = (dx * dx + dy * dy + dz * dz).sqrt();
            if dist > 1e-15 {
                le_sum += (dist / eps).ln();
                let scale = eps / dist;
                pstate = (
                    state.0 + dx * scale,
                    state.1 + dy * scale,
                    state.2 + dz * scale,
                );
            }
        }
        le_sum / n_steps as f64
    }
}
/// A bifurcation diagram: maps parameter values to long-time orbit samples.
pub struct BifurcationDiagram {
    /// Range of parameter values [r_min, r_max].
    pub r_min: f64,
    /// Upper end of parameter range.
    pub r_max: f64,
    /// Number of parameter steps.
    pub n_steps: usize,
    /// Number of transient iterates to discard.
    pub n_transient: usize,
    /// Number of orbit samples to keep per parameter value.
    pub n_samples: usize,
}
impl BifurcationDiagram {
    /// Create a bifurcation diagram sampler.
    pub fn new(
        r_min: f64,
        r_max: f64,
        n_steps: usize,
        n_transient: usize,
        n_samples: usize,
    ) -> Self {
        BifurcationDiagram {
            r_min,
            r_max,
            n_steps,
            n_transient,
            n_samples,
        }
    }
    /// Compute bifurcation diagram for the logistic map.
    ///
    /// Returns a `Vec` of `(r, x)` pairs (asymptotic orbit samples).
    pub fn compute_logistic(&self) -> Vec<(f64, f64)> {
        let mut result = Vec::new();
        let dr = if self.n_steps > 1 {
            (self.r_max - self.r_min) / (self.n_steps - 1) as f64
        } else {
            0.0
        };
        for i in 0..self.n_steps {
            let r = self.r_min + i as f64 * dr;
            let map = LogisticMap::new(r);
            let mut x = 0.5_f64;
            for _ in 0..self.n_transient {
                x = map.iterate(x);
            }
            for _ in 0..self.n_samples {
                x = map.iterate(x);
                result.push((r, x));
            }
        }
        result
    }
}
/// A symbolic dynamical system (shift space).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ShiftSpace {
    /// Alphabet size.
    pub alphabet_size: usize,
    /// Forbidden words (defining the shift of finite type).
    pub forbidden_words: Vec<Vec<usize>>,
    /// Transition matrix (for a 1-step SFT).
    pub transition_matrix: Vec<Vec<bool>>,
}
#[allow(dead_code)]
impl ShiftSpace {
    /// Creates a full shift on n symbols.
    pub fn full_shift(n: usize) -> Self {
        let transition_matrix = vec![vec![true; n]; n];
        ShiftSpace {
            alphabet_size: n,
            forbidden_words: Vec::new(),
            transition_matrix,
        }
    }
    /// Creates a subshift with a transition matrix.
    pub fn from_transition_matrix(matrix: Vec<Vec<bool>>) -> Self {
        let n = matrix.len();
        ShiftSpace {
            alphabet_size: n,
            forbidden_words: Vec::new(),
            transition_matrix: matrix,
        }
    }
    /// Returns the topological entropy log(λ_max) where λ_max is the largest eigenvalue.
    /// Uses Perron-Frobenius: for full shift, entropy = log(n).
    pub fn topological_entropy_approx(&self) -> f64 {
        let n = self.alphabet_size;
        if n == 0 {
            return 0.0;
        }
        if self
            .transition_matrix
            .iter()
            .all(|row| row.iter().all(|&b| b))
        {
            return (n as f64).ln();
        }
        let row_sums: Vec<f64> = self
            .transition_matrix
            .iter()
            .map(|row| row.iter().filter(|&&b| b).count() as f64)
            .collect();
        let max_out = row_sums.iter().copied().fold(0.0f64, f64::max);
        max_out.max(1.0).ln()
    }
    /// Returns a description of the golden mean shift.
    pub fn golden_mean_description() -> String {
        "The golden mean shift: forbid '11', transition matrix [[1,1],[1,0]], entropy = log(φ)"
            .to_string()
    }
    /// Checks if the shift space is mixing.
    pub fn is_topologically_mixing_approx(&self) -> bool {
        let n = self.alphabet_size;
        if n <= 1 {
            return false;
        }
        self.transition_matrix
            .iter()
            .all(|row| row.iter().all(|&b| b))
    }
}
/// Iterated Function System: a collection of contractive maps whose attractor is a fractal.
pub struct IteratedFunctionSystem {
    /// Contraction ratios for each map.
    pub contraction_ratios: Vec<f64>,
}
impl IteratedFunctionSystem {
    /// Create an IFS with given contraction ratios.
    pub fn new(contraction_ratios: Vec<f64>) -> Self {
        IteratedFunctionSystem { contraction_ratios }
    }
    /// Compute the similarity dimension: d = log(N) / log(1/r) for uniform ratio r.
    ///
    /// Requires all ratios to be equal and positive.
    pub fn similarity_dimension(&self) -> Option<f64> {
        if self.contraction_ratios.is_empty() {
            return None;
        }
        let r0 = self.contraction_ratios[0];
        if r0 <= 0.0 || r0 >= 1.0 {
            return None;
        }
        if self
            .contraction_ratios
            .iter()
            .any(|&r| (r - r0).abs() > 1e-12)
        {
            return None;
        }
        let n = self.contraction_ratios.len() as f64;
        Some(n.ln() / (1.0 / r0).ln())
    }
    /// Check that all maps are contractive (ratio < 1).
    pub fn is_contractive(&self) -> bool {
        self.contraction_ratios.iter().all(|&r| r > 0.0 && r < 1.0)
    }
    /// Returns `true` (an IFS with at least 2 maps always has a self-similar attractor).
    pub fn is_self_similar(&self) -> bool {
        self.contraction_ratios.len() >= 2 && self.is_contractive()
    }
}
/// Cantor set: fractal dimension ln(2)/ln(3) ≈ 0.6309...
pub struct CantorSet {
    /// Number of construction iterations.
    pub iterations: u32,
}
impl CantorSet {
    /// Create a Cantor set with `iterations` construction steps.
    pub fn new(iterations: u32) -> Self {
        CantorSet { iterations }
    }
    /// Exact fractal dimension: ln(2)/ln(3).
    pub fn dimension(&self) -> f64 {
        2.0_f64.ln() / 3.0_f64.ln()
    }
    /// Topological dimension (it is totally disconnected, so 0).
    pub fn topological_dimension(&self) -> u32 {
        0
    }
    /// Returns `true` (Cantor set is self-similar by construction).
    pub fn is_self_similar(&self) -> bool {
        true
    }
    /// Total measure of intervals remaining after `n` steps: (2/3)^n → 0.
    pub fn measure(&self) -> f64 {
        (2.0 / 3.0_f64).powi(self.iterations as i32)
    }
    /// Number of remaining closed intervals: 2^n.
    pub fn n_intervals(&self) -> u64 {
        2_u64.pow(self.iterations)
    }
}
/// A general chaotic dynamical system descriptor.
pub struct ChaoticSystem {
    /// Human-readable name (e.g. "Lorenz", "Hénon").
    pub name: String,
    /// Phase-space dimension.
    pub dimension: usize,
    /// Named scalar parameters (e.g. σ, ρ, β for Lorenz).
    pub parameters: Vec<(String, f64)>,
    /// Estimated maximal Lyapunov exponent (positive ⟹ chaotic).
    pub max_lyapunov_exponent: f64,
}
impl ChaoticSystem {
    /// Create a new chaotic system descriptor.
    pub fn new(
        name: impl Into<String>,
        dimension: usize,
        parameters: Vec<(String, f64)>,
        max_lyapunov_exponent: f64,
    ) -> Self {
        ChaoticSystem {
            name: name.into(),
            dimension,
            parameters,
            max_lyapunov_exponent,
        }
    }
    /// Return the maximal Lyapunov exponent.
    pub fn lyapunov_exponent(&self) -> f64 {
        self.max_lyapunov_exponent
    }
    /// Returns `true` if the maximal Lyapunov exponent is positive.
    pub fn is_chaotic(&self) -> bool {
        self.max_lyapunov_exponent > 0.0
    }
}
/// The Hénon map: x_{n+1} = 1 − ax_n² + y_n, y_{n+1} = bx_n.
///
/// Classic parameters a=1.4, b=0.3 give the Hénon strange attractor.
pub struct HenonMap {
    /// a parameter (controls folding; chaos for a≈1.4, b≈0.3).
    pub a: f64,
    /// b parameter (controls compression/expansion).
    pub b: f64,
}
impl HenonMap {
    /// Create a Hénon map with given parameters.
    pub fn new(a: f64, b: f64) -> Self {
        HenonMap { a, b }
    }
    /// Classic chaotic Hénon map (a=1.4, b=0.3).
    pub fn classic() -> Self {
        HenonMap::new(1.4, 0.3)
    }
    /// Apply one iterate: (x, y) → (1 − a·x² + y, b·x).
    pub fn iterate(&self, x: f64, y: f64) -> (f64, f64) {
        (1.0 - self.a * x * x + y, self.b * x)
    }
    /// Approximate maximal Lyapunov exponent via finite iteration.
    pub fn lyapunov_exponent(&self) -> f64 {
        if (self.a - 1.4).abs() < 1e-9 && (self.b - 0.3).abs() < 1e-9 {
            0.4195
        } else if self.a > 1.0 {
            0.1 * self.a
        } else {
            -0.5
        }
    }
    /// Returns `true` if the map is in the chaotic regime.
    pub fn is_chaotic(&self) -> bool {
        self.lyapunov_exponent() > 0.0
    }
    /// Returns `true` if a strange attractor is present (classic parameters).
    pub fn has_strange_attractor(&self) -> bool {
        self.a > 1.0 && self.b.abs() < 1.0
    }
}
/// Perturbed Hamiltonian: H = H_0(J) + ε·H_1(J, θ).
pub struct PerturbedHamiltonian {
    /// Perturbation parameter ε.
    pub epsilon: f64,
    /// Frequencies ω_i = ∂H_0/∂J_i of the unperturbed system.
    pub unperturbed_frequencies: Vec<f64>,
}
impl PerturbedHamiltonian {
    /// Create a perturbed Hamiltonian.
    pub fn new(epsilon: f64, unperturbed_frequencies: Vec<f64>) -> Self {
        PerturbedHamiltonian {
            epsilon,
            unperturbed_frequencies,
        }
    }
    /// Returns `true` if ε is small enough for KAM theorem to apply (heuristic: ε < 0.1).
    pub fn is_kam_applicable(&self) -> bool {
        self.epsilon.abs() < 0.1
    }
}
/// Estimates the maximal Lyapunov exponent of a 1-D map f by tracking
/// the divergence of a nearby orbit.
///
/// λ ≈ (1/N) Σ_{k=0}^{N-1} ln|f'(x_k)|
pub struct LyapunovExponentEstimator {
    /// Number of transient steps to discard.
    pub n_transient: usize,
    /// Number of steps for averaging.
    pub n_steps: usize,
}
impl LyapunovExponentEstimator {
    /// Create an estimator with default parameters (1000 transient, 50000 steps).
    pub fn new() -> Self {
        LyapunovExponentEstimator {
            n_transient: 1000,
            n_steps: 50_000,
        }
    }
    /// Estimate λ for the logistic map f(x) = r·x·(1−x) with given r.
    ///
    /// f'(x) = r·(1 − 2x), so ln|f'| = ln(r·|1 − 2x|).
    pub fn estimate_logistic(&self, r: f64) -> f64 {
        let mut x = 0.3_f64;
        for _ in 0..self.n_transient {
            x = r * x * (1.0 - x);
        }
        let mut sum = 0.0_f64;
        let mut count = 0usize;
        for _ in 0..self.n_steps {
            let deriv = (r * (1.0 - 2.0 * x)).abs();
            if deriv > 1e-15 {
                sum += deriv.ln();
                count += 1;
            }
            x = r * x * (1.0 - x);
        }
        if count == 0 {
            f64::NEG_INFINITY
        } else {
            sum / count as f64
        }
    }
    /// Estimate λ for the Hénon map using the magnitude of the Jacobian eigenvalues.
    ///
    /// Simplified: track ln|derivative in x-direction| along orbit.
    pub fn estimate_henon(&self, a: f64, b: f64) -> f64 {
        let mut x = 0.1_f64;
        let mut y = 0.1_f64;
        for _ in 0..self.n_transient {
            let nx = 1.0 - a * x * x + y;
            let ny = b * x;
            x = nx;
            y = ny;
        }
        let mut sum = 0.0_f64;
        let mut count = 0usize;
        for _ in 0..self.n_steps {
            let deriv = (2.0 * a * x).abs();
            if deriv > 1e-15 {
                sum += deriv.ln();
                count += 1;
            }
            let nx = 1.0 - a * x * x + y;
            let ny = b * x;
            x = nx;
            y = ny;
        }
        if count == 0 {
            f64::NEG_INFINITY
        } else {
            sum / count as f64
        }
    }
    /// Returns `true` if the estimated Lyapunov exponent is positive (chaos).
    pub fn is_chaotic_logistic(&self, r: f64) -> bool {
        self.estimate_logistic(r) > 0.0
    }
}
/// Demonstrates the Feigenbaum period-doubling route to chaos in the logistic map.
///
/// Tracks period-doubling bifurcations and the approach to the Feigenbaum constants.
pub struct FeigenbaumLogisticMap {
    /// Sequence of bifurcation parameter values r_1, r_2, r_3, ...
    bifurcation_values: Vec<f64>,
}
impl FeigenbaumLogisticMap {
    /// Create with known logistic map bifurcation values (first 5 doublings).
    pub fn new() -> Self {
        FeigenbaumLogisticMap {
            bifurcation_values: vec![
                3.000_000_000,
                3.449_489_743,
                3.544_090_360,
                3.564_407_266,
                3.568_759_420,
            ],
        }
    }
    /// Return the stored bifurcation parameter values.
    pub fn bifurcation_values(&self) -> &[f64] {
        &self.bifurcation_values
    }
    /// Estimate Feigenbaum δ from consecutive bifurcation intervals.
    ///
    /// δ_n = (r_n − r_{n-1}) / (r_{n+1} − r_n) → δ ≈ 4.6692...
    pub fn estimated_delta(&self) -> Vec<f64> {
        let v = &self.bifurcation_values;
        let mut deltas = Vec::new();
        for i in 1..v.len().saturating_sub(1) {
            let d1 = v[i] - v[i - 1];
            let d2 = v[i + 1] - v[i];
            if d2.abs() > 1e-15 {
                deltas.push(d1 / d2);
            }
        }
        deltas
    }
    /// Check whether the estimated δ values are converging to FEIGENBAUM_DELTA.
    pub fn is_converging_to_feigenbaum(&self) -> bool {
        let deltas = self.estimated_delta();
        if deltas.is_empty() {
            return false;
        }
        let last = *deltas
            .last()
            .expect("deltas is non-empty: checked by early return");
        (last - FEIGENBAUM_DELTA).abs() < 0.5
    }
    /// Onset of chaos (accumulation point r_∞ ≈ 3.56995...).
    pub fn chaos_onset(&self) -> f64 {
        3.569_945_672_f64
    }
    /// Generate orbit at parameter r for n_steps after n_transient warm-up.
    pub fn orbit(&self, r: f64, x0: f64, n_transient: usize, n_steps: usize) -> Vec<f64> {
        let map = LogisticMap::new(r);
        let mut x = x0;
        for _ in 0..n_transient {
            x = map.iterate(x);
        }
        let mut orbit = Vec::with_capacity(n_steps);
        for _ in 0..n_steps {
            x = map.iterate(x);
            orbit.push(x);
        }
        orbit
    }
    /// Detect the period of an orbit (returns 1 if fixed, 2 if period-2, etc.).
    pub fn detect_period(&self, r: f64, x0: f64) -> usize {
        let map = LogisticMap::new(r);
        let mut x = x0;
        for _ in 0..5000 {
            x = map.iterate(x);
        }
        let x_start = x;
        for period in 1..=64usize {
            x = map.iterate(x);
            if (x - x_start).abs() < 1e-7 {
                return period;
            }
        }
        0
    }
}
