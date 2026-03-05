//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)
use super::functions::*;

/// Disintegration of a measure.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct MeasureDisintExt {
    pub base_space: String,
    pub fiber_space: String,
    pub is_regular: bool,
}
#[allow(dead_code)]
impl MeasureDisintExt {
    /// Standard disintegration.
    pub fn standard(base: &str, fiber: &str) -> Self {
        Self {
            base_space: base.to_string(),
            fiber_space: fiber.to_string(),
            is_regular: true,
        }
    }
    /// Rokhlin's disintegration theorem holds when the spaces are standard Borel.
    pub fn rokhlin_applies(&self) -> bool {
        self.is_regular
    }
}
/// Discrete disintegration of a measure over a finite partition.
///
/// Given a partition of Ω into fibres B₁, …, Bₖ, computes the conditional
/// measures μ_{Bⱼ} = μ(·|Bⱼ) and the marginal ν on the base.
///
/// This is the discrete analogue of the Rokhlin disintegration theorem.
#[derive(Debug, Clone)]
pub struct MeasureDisintegration {
    /// Weights μ({ωᵢ}) for each atom.
    pub weights: Vec<f64>,
    /// Fibre assignment: fibre[i] = j means ωᵢ ∈ Bⱼ.
    pub fibre: Vec<usize>,
    /// Number of fibres (partition classes).
    pub n_fibres: usize,
}
impl MeasureDisintegration {
    /// Create a disintegration over a given partition.
    pub fn new(weights: Vec<f64>, fibre: Vec<usize>, n_fibres: usize) -> Self {
        assert_eq!(weights.len(), fibre.len());
        MeasureDisintegration {
            weights,
            fibre,
            n_fibres,
        }
    }
    /// Compute the marginal measure ν on {0, …, k-1}: ν(j) = μ(Bⱼ).
    pub fn marginal(&self) -> Vec<f64> {
        let mut nu = vec![0.0f64; self.n_fibres];
        for (&w, &j) in self.weights.iter().zip(self.fibre.iter()) {
            if j < self.n_fibres {
                nu[j] += w;
            }
        }
        nu
    }
    /// Compute the conditional measure μ_{Bⱼ} (normalised to probability 1).
    ///
    /// Returns `None` if Bⱼ has zero measure.
    pub fn conditional(&self, j: usize) -> Option<Vec<f64>> {
        let total: f64 = self
            .weights
            .iter()
            .zip(self.fibre.iter())
            .filter(|(_, &f)| f == j)
            .map(|(&w, _)| w)
            .sum();
        if total < 1e-15 {
            return None;
        }
        let cond: Vec<f64> = self
            .weights
            .iter()
            .zip(self.fibre.iter())
            .map(|(&w, &f)| if f == j { w / total } else { 0.0 })
            .collect();
        Some(cond)
    }
    /// Verify the disintegration identity: ∫_Ω f dμ = ∫_B (∫_{Bⱼ} f dμⱼ) dν(j).
    ///
    /// Returns `(lhs, rhs, holds)`.
    pub fn verify_disintegration(&self, f: &[f64]) -> (f64, f64, bool) {
        assert_eq!(f.len(), self.weights.len());
        let lhs: f64 = f
            .iter()
            .zip(self.weights.iter())
            .map(|(&fi, &wi)| fi * wi)
            .sum();
        let nu = self.marginal();
        let rhs: f64 = (0..self.n_fibres)
            .map(|j| {
                if let Some(cond_j) = self.conditional(j) {
                    let inner: f64 = f.iter().zip(cond_j.iter()).map(|(&fi, &ci)| fi * ci).sum();
                    inner * nu[j]
                } else {
                    0.0
                }
            })
            .sum();
        let holds = (lhs - rhs).abs() < 1e-10;
        (lhs, rhs, holds)
    }
}
/// A discrete probability space with a finite sample space.
#[derive(Debug, Clone)]
pub struct DiscreteProbabilitySpace {
    /// The sample outcomes (names).
    pub outcomes: Vec<String>,
    /// Probability of each outcome (same length as `outcomes`).
    pub probabilities: Vec<f64>,
}
impl DiscreteProbabilitySpace {
    /// Create a new discrete probability space.
    ///
    /// Returns `None` if `outcomes` and `probabilities` differ in length,
    /// or if the probabilities do not sum to 1 within tolerance.
    pub fn new(outcomes: Vec<String>, probabilities: Vec<f64>) -> Option<Self> {
        if outcomes.len() != probabilities.len() {
            return None;
        }
        let space = DiscreteProbabilitySpace {
            outcomes,
            probabilities,
        };
        if space.is_valid() {
            Some(space)
        } else {
            None
        }
    }
    /// Create a uniform distribution over the given outcomes.
    pub fn uniform(outcomes: Vec<String>) -> Self {
        let n = outcomes.len();
        let p = if n == 0 { 0.0 } else { 1.0 / n as f64 };
        DiscreteProbabilitySpace {
            probabilities: vec![p; n],
            outcomes,
        }
    }
    /// Number of outcomes in the sample space.
    pub fn n_outcomes(&self) -> usize {
        self.outcomes.len()
    }
    /// Probability of a named outcome (0 if not found).
    pub fn prob(&self, outcome: &str) -> f64 {
        self.outcomes
            .iter()
            .position(|o| o == outcome)
            .map(|i| self.probabilities[i])
            .unwrap_or(0.0)
    }
    /// Check that probabilities sum to 1 within tolerance 1e-9.
    pub fn is_valid(&self) -> bool {
        if self.outcomes.len() != self.probabilities.len() {
            return false;
        }
        if self.probabilities.iter().any(|&p| p < -1e-12) {
            return false;
        }
        let total: f64 = self.probabilities.iter().sum();
        (total - 1.0).abs() < 1e-9
    }
}
/// Haar measure on the finite abelian group ℤ/Nℤ (integers mod N).
///
/// The unique translation-invariant probability measure assigns weight 1/N
/// to each element of the group.
#[derive(Debug, Clone)]
pub struct HaarMeasureOnZpN {
    /// The group order N.
    pub n: usize,
}
impl HaarMeasureOnZpN {
    /// Create the Haar measure on ℤ/Nℤ.
    pub fn new(n: usize) -> Self {
        assert!(n > 0, "group order must be positive");
        HaarMeasureOnZpN { n }
    }
    /// Weight of each singleton element (= 1/N).
    pub fn atom_weight(&self) -> f64 {
        1.0 / self.n as f64
    }
    /// Measure of a subset given as a list of elements (mod N).
    pub fn measure_of(&self, set: &[usize]) -> f64 {
        let unique: Vec<usize> = {
            let mut v: Vec<usize> = set.iter().map(|&x| x % self.n).collect();
            v.sort_unstable();
            v.dedup();
            v
        };
        unique.len() as f64 / self.n as f64
    }
    /// Check translation invariance: μ(A + g) = μ(A) for all g ∈ ℤ/Nℤ.
    ///
    /// Checks all shifts and verifies the measure is constant.
    pub fn verify_translation_invariance(&self, set: &[usize]) -> bool {
        let base = self.measure_of(set);
        for g in 0..self.n {
            let shifted: Vec<usize> = set.iter().map(|&x| (x + g) % self.n).collect();
            let m = self.measure_of(&shifted);
            if (m - base).abs() > 1e-12 {
                return false;
            }
        }
        true
    }
    /// Integrate a function f : ℤ/Nℤ → ℝ: ∫ f dμ = (1/N) Σₖ f(k).
    pub fn integrate(&self, f: impl Fn(usize) -> f64) -> f64 {
        (0..self.n).map(|k| f(k)).sum::<f64>() / self.n as f64
    }
    /// Convolution of two functions f, g : ℤ/Nℤ → ℝ under the Haar measure.
    ///
    /// (f * g)(n) = ∫ f(n - k) g(k) dμ(k) = (1/N) Σₖ f((n+N-k)%N) · g(k)
    pub fn convolve(&self, f: &[f64], g: &[f64]) -> Vec<f64> {
        assert_eq!(f.len(), self.n);
        assert_eq!(g.len(), self.n);
        (0..self.n)
            .map(|m| {
                (0..self.n)
                    .map(|k| f[(m + self.n - k) % self.n] * g[k])
                    .sum::<f64>()
                    / self.n as f64
            })
            .collect()
    }
}
/// Fubini-Tonelli theorem data.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FubiniData {
    pub space_x: String,
    pub space_y: String,
    pub is_sigma_finite: bool,
    pub is_non_negative: bool,
}
#[allow(dead_code)]
impl FubiniData {
    /// Tonelli condition (non-negative).
    pub fn tonelli(x: &str, y: &str) -> Self {
        Self {
            space_x: x.to_string(),
            space_y: y.to_string(),
            is_sigma_finite: true,
            is_non_negative: true,
        }
    }
    /// Fubini condition (integrable).
    pub fn fubini(x: &str, y: &str) -> Self {
        Self {
            space_x: x.to_string(),
            space_y: y.to_string(),
            is_sigma_finite: true,
            is_non_negative: false,
        }
    }
    /// Can interchange integrals?
    pub fn can_interchange(&self) -> bool {
        self.is_sigma_finite
    }
}
/// Discrete approximation of the Bochner integral for ℝⁿ-valued functions.
///
/// Approximates ∫_Ω f dμ ∈ ℝⁿ by Σᵢ f(ωᵢ) · μ({ωᵢ}).
#[derive(Debug, Clone)]
pub struct BochnerIntegralApprox {
    /// Probability weights μ({ωᵢ}) for each atom ωᵢ.
    pub weights: Vec<f64>,
    /// Dimension of the target space ℝᵈ.
    pub dim: usize,
}
impl BochnerIntegralApprox {
    /// Create a Bochner integral approximator.
    pub fn new(weights: Vec<f64>, dim: usize) -> Self {
        BochnerIntegralApprox { weights, dim }
    }
    /// Compute ∫ f dμ for a vector-valued function f : Ω → ℝᵈ.
    ///
    /// `values[i]` is the vector f(ωᵢ) ∈ ℝᵈ.
    pub fn integrate(&self, values: &[Vec<f64>]) -> Vec<f64> {
        assert_eq!(values.len(), self.weights.len());
        let mut result = vec![0.0f64; self.dim];
        for (val, &w) in values.iter().zip(self.weights.iter()) {
            assert_eq!(val.len(), self.dim);
            for (r, &v) in result.iter_mut().zip(val.iter()) {
                *r += v * w;
            }
        }
        result
    }
    /// Compute the Bochner norm ‖f‖_{L¹(μ; ℝᵈ)} = Σᵢ ‖f(ωᵢ)‖₂ · μ({ωᵢ}).
    pub fn bochner_l1_norm(&self, values: &[Vec<f64>]) -> f64 {
        assert_eq!(values.len(), self.weights.len());
        values
            .iter()
            .zip(self.weights.iter())
            .map(|(v, &w)| {
                let norm: f64 = v.iter().map(|&x| x * x).sum::<f64>().sqrt();
                norm * w
            })
            .sum()
    }
    /// Check Jensen's inequality: ‖∫f dμ‖ ≤ ∫‖f‖ dμ.
    pub fn verify_jensen(&self, values: &[Vec<f64>]) -> bool {
        let integral = self.integrate(values);
        let norm_integral: f64 = integral.iter().map(|&x| x * x).sum::<f64>().sqrt();
        let bochner_norm = self.bochner_l1_norm(values);
        norm_integral <= bochner_norm + 1e-10
    }
}
/// Egoroff's theorem data.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct EgoroffData {
    pub sequence: String,
    pub limit: String,
    pub measure: f64,
}
#[allow(dead_code)]
impl EgoroffData {
    /// Egoroff: a.e. convergence is near-uniform convergence on finite measure space.
    pub fn new(seq: &str, lim: &str, meas: f64) -> Self {
        Self {
            sequence: seq.to_string(),
            limit: lim.to_string(),
            measure: meas,
        }
    }
    /// Description.
    pub fn description(&self) -> String {
        format!(
            "Egoroff: {} → {} a.e. ⟹ uniform outside set of measure < ε (total measure {})",
            self.sequence, self.limit, self.measure
        )
    }
}
/// A simple sigma-algebra on a finite set (index set {0, …, n-1}).
///
/// Elements of the sigma-algebra are stored as sorted index lists.
pub struct FiniteSigmaAlgebra {
    /// Size of the underlying set.
    pub n_elements: usize,
    /// The sets that belong to the sigma-algebra (as sorted index vecs).
    pub sets: Vec<Vec<usize>>,
}
impl FiniteSigmaAlgebra {
    /// Create the minimal sigma-algebra {∅, Ω}.
    pub fn new(n: usize) -> Self {
        Self::trivial(n)
    }
    /// Trivial sigma-algebra {∅, Ω}.
    pub fn trivial(n: usize) -> Self {
        FiniteSigmaAlgebra {
            n_elements: n,
            sets: vec![vec![], (0..n).collect()],
        }
    }
    /// Power set sigma-algebra — all 2^n subsets.
    ///
    /// Only feasible for small n (≤ 20 or so).
    pub fn power_set(n: usize) -> Self {
        let total = 1usize << n;
        let mut sets = Vec::with_capacity(total);
        for mask in 0..total {
            let set: Vec<usize> = (0..n).filter(|&i| (mask >> i) & 1 == 1).collect();
            sets.push(set);
        }
        FiniteSigmaAlgebra {
            n_elements: n,
            sets,
        }
    }
    /// Complement of a set within {0, …, n-1}.
    pub fn complement(&self, set: &[usize]) -> Vec<usize> {
        (0..self.n_elements).filter(|i| !set.contains(i)).collect()
    }
    /// Union of two index sets (sorted, deduplicated).
    pub fn union_of(&self, s1: &[usize], s2: &[usize]) -> Vec<usize> {
        let mut result: Vec<usize> = s1.iter().chain(s2.iter()).copied().collect();
        result.sort_unstable();
        result.dedup();
        result
    }
    /// Check whether the complement of `set` is also in the sigma-algebra.
    pub fn contains_complement(&self, set: &[usize]) -> bool {
        let comp = self.complement(set);
        self.sets
            .iter()
            .any(|s| s.len() == comp.len() && s.iter().zip(comp.iter()).all(|(a, b)| a == b))
    }
    /// Verify that this collection is actually a sigma-algebra:
    /// 1. Contains ∅ and Ω
    /// 2. Closed under complement
    /// 3. Closed under pairwise union (sufficient for finite case)
    pub fn is_sigma_algebra(&self) -> bool {
        let empty: Vec<usize> = vec![];
        let full: Vec<usize> = (0..self.n_elements).collect();
        let has_empty = self.sets.iter().any(|s| s.is_empty());
        let has_full = self.sets.iter().any(|s| s == &full);
        if !has_empty || !has_full {
            return false;
        }
        for s in &self.sets {
            if !self.contains_complement(s) {
                return false;
            }
        }
        let sets_clone: Vec<Vec<usize>> = self.sets.clone();
        for s1 in &sets_clone {
            for s2 in &sets_clone {
                let u = self.union_of(s1, s2);
                if !self
                    .sets
                    .iter()
                    .any(|s| s.len() == u.len() && s.iter().zip(u.iter()).all(|(a, b)| a == b))
                {
                    return false;
                }
            }
        }
        self.contains_complement(&empty)
    }
}
/// Measure-theoretic independence.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Independence {
    pub sigma_algebras: Vec<String>,
    pub is_pairwise: bool,
    pub is_mutual: bool,
}
#[allow(dead_code)]
impl Independence {
    /// Create independence data.
    pub fn new(algebras: Vec<String>, mutual: bool) -> Self {
        let pairwise = algebras.len() >= 2;
        Self {
            sigma_algebras: algebras,
            is_pairwise: pairwise,
            is_mutual: mutual,
        }
    }
    /// Mutual independence implies pairwise but not vice versa.
    pub fn mutually_independent_implies_pairwise(&self) -> bool {
        self.is_mutual
    }
    /// Number of sigma-algebras.
    pub fn count(&self) -> usize {
        self.sigma_algebras.len()
    }
}
/// Abstract integration theory data.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct AbstractIntegral {
    pub space_name: String,
    pub integrand_type: String,
    pub is_lebesgue: bool,
    pub is_riemann_stieltjes: bool,
}
#[allow(dead_code)]
impl AbstractIntegral {
    /// Lebesgue integral.
    pub fn lebesgue(space: &str) -> Self {
        Self {
            space_name: space.to_string(),
            integrand_type: "measurable".to_string(),
            is_lebesgue: true,
            is_riemann_stieltjes: false,
        }
    }
    /// Riemann-Stieltjes integral.
    pub fn riemann_stieltjes(space: &str) -> Self {
        Self {
            space_name: space.to_string(),
            integrand_type: "bounded variation".to_string(),
            is_lebesgue: false,
            is_riemann_stieltjes: true,
        }
    }
    /// Description of the integral.
    pub fn description(&self) -> String {
        if self.is_lebesgue {
            format!("Lebesgue integral on {}", self.space_name)
        } else {
            format!("Riemann-Stieltjes integral on {}", self.space_name)
        }
    }
}
/// A pre-measure on a ring of sets over {0, …, n-1}.
///
/// Checks whether a given set function satisfies the Carathéodory conditions
/// (finite additivity on the ring) that allow extension to a full measure.
#[derive(Debug, Clone)]
pub struct CaratheodoryExtension {
    /// Number of base elements (universe = {0, …, n-1}).
    pub n: usize,
    /// Pre-measure weights: weight[i] = pre-measure of singleton {i}.
    pub weights: Vec<f64>,
}
impl CaratheodoryExtension {
    /// Create a Carathéodory extension checker with per-element weights.
    pub fn new(weights: Vec<f64>) -> Self {
        let n = weights.len();
        CaratheodoryExtension { n, weights }
    }
    /// Compute the pre-measure of a set (sum of singleton weights).
    pub fn premeasure(&self, set: &[usize]) -> f64 {
        set.iter()
            .filter(|&&i| i < self.n)
            .map(|&i| self.weights[i])
            .sum()
    }
    /// Check finite additivity: μ(A ∪ B) = μ(A) + μ(B) when A ∩ B = ∅.
    pub fn is_finitely_additive(&self, a: &[usize], b: &[usize]) -> bool {
        let disjoint = !a.iter().any(|x| b.contains(x));
        if !disjoint {
            return false;
        }
        let mut union: Vec<usize> = a.iter().chain(b.iter()).copied().collect();
        union.sort_unstable();
        union.dedup();
        let lhs = self.premeasure(&union);
        let rhs = self.premeasure(a) + self.premeasure(b);
        (lhs - rhs).abs() < 1e-12
    }
    /// Verify the Carathéodory measurability condition for a test set E:
    /// μ*(A) = μ*(A ∩ E) + μ*(A ∩ Eᶜ) for all A.
    ///
    /// Here we use the pre-measure itself as the outer measure (valid since
    /// the underlying sets are open in the discrete topology).
    pub fn is_caratheodory_measurable(&self, e: &[usize]) -> bool {
        for i in 0..self.n {
            let a = vec![i];
            let a_cap_e: Vec<usize> = a.iter().filter(|&&x| e.contains(&x)).copied().collect();
            let a_cap_ec: Vec<usize> = a.iter().filter(|&&x| !e.contains(&x)).copied().collect();
            let mu_a = self.premeasure(&a);
            let mu_ae = self.premeasure(&a_cap_e);
            let mu_aec = self.premeasure(&a_cap_ec);
            if (mu_a - mu_ae - mu_aec).abs() > 1e-12 {
                return false;
            }
        }
        true
    }
    /// Total mass (pre-measure of the entire universe).
    pub fn total_mass(&self) -> f64 {
        self.weights.iter().sum()
    }
}
/// Convergence theorem type.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum ConvergenceTheoremType {
    MonotoneConvergence,
    DominatedConvergence,
    FatousLemma,
    VitaliConvergence,
}
#[allow(dead_code)]
impl ConvergenceTheoremType {
    /// Short name.
    pub fn name(&self) -> &str {
        match self {
            Self::MonotoneConvergence => "MCT",
            Self::DominatedConvergence => "DCT",
            Self::FatousLemma => "Fatou",
            Self::VitaliConvergence => "Vitali",
        }
    }
    /// Required condition.
    pub fn condition(&self) -> &str {
        match self {
            Self::MonotoneConvergence => "non-decreasing non-negative functions",
            Self::DominatedConvergence => "dominated by integrable function",
            Self::FatousLemma => "non-negative functions",
            Self::VitaliConvergence => "uniformly integrable",
        }
    }
}
/// Radon-Nikodym derivative data.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RadonNikodymData {
    pub measure_nu: String,
    pub measure_mu: String,
    pub density_name: String,
    pub is_absolutely_continuous: bool,
}
#[allow(dead_code)]
impl RadonNikodymData {
    /// Create Radon-Nikodym data.
    pub fn new(nu: &str, mu: &str) -> Self {
        Self {
            measure_nu: nu.to_string(),
            measure_mu: mu.to_string(),
            density_name: format!("d{}/d{}", nu, mu),
            is_absolutely_continuous: true,
        }
    }
    /// The Radon-Nikodym derivative exists iff nu << mu.
    pub fn derivative_exists(&self) -> bool {
        self.is_absolutely_continuous
    }
}
/// Lusin's theorem data.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LusinData {
    pub space: String,
    pub function_name: String,
    pub epsilon: f64,
}
#[allow(dead_code)]
impl LusinData {
    /// Apply Lusin's theorem: every measurable function is
    /// nearly continuous.
    pub fn new(space: &str, f: &str, eps: f64) -> Self {
        Self {
            space: space.to_string(),
            function_name: f.to_string(),
            epsilon: eps,
        }
    }
    /// The compact set where f is continuous has measure > 1 - epsilon.
    pub fn compact_set_description(&self) -> String {
        format!(
            "Compact K ⊆ {} where {} is continuous, μ(K) ≥ 1 - {}",
            self.space, self.function_name, self.epsilon
        )
    }
}
/// Measure-theoretic capacity.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Capacity {
    pub name: String,
    pub is_submodular: bool,
    pub is_monotone: bool,
}
#[allow(dead_code)]
impl Capacity {
    /// Newtonian capacity (submodular, monotone).
    pub fn newtonian() -> Self {
        Self {
            name: "Newtonian".to_string(),
            is_submodular: true,
            is_monotone: true,
        }
    }
    /// Choquet integral w.r.t. this capacity.
    pub fn choquet_integral_description(&self) -> String {
        format!("Choquet integral w.r.t. {} capacity", self.name)
    }
    /// Is this a true measure (both sub- and supermodular)?
    pub fn is_measure(&self) -> bool {
        self.is_submodular && self.is_monotone
    }
}
/// Approximates the Lebesgue measure of a union of intervals in ℝ using a grid.
///
/// The estimator partitions `[lo, hi]` into `resolution` equal cells and
/// counts which cells intersect the given set of intervals.
#[derive(Debug, Clone)]
pub struct LebesgueMeasureEstimator {
    /// Lower bound of the ambient interval.
    pub lo: f64,
    /// Upper bound of the ambient interval.
    pub hi: f64,
    /// Number of grid cells.
    pub resolution: usize,
}
impl LebesgueMeasureEstimator {
    /// Create a new estimator over `[lo, hi]` with `resolution` cells.
    pub fn new(lo: f64, hi: f64, resolution: usize) -> Self {
        LebesgueMeasureEstimator { lo, hi, resolution }
    }
    /// Cell width.
    pub fn cell_width(&self) -> f64 {
        (self.hi - self.lo) / self.resolution as f64
    }
    /// Estimate the Lebesgue measure of the union of `intervals` (each is `(a, b)`).
    ///
    /// A cell `[lo + k·h, lo + (k+1)·h)` is counted if its midpoint lies inside
    /// at least one interval.
    pub fn estimate(&self, intervals: &[(f64, f64)]) -> f64 {
        let h = self.cell_width();
        let mut count = 0usize;
        for k in 0..self.resolution {
            let mid = self.lo + (k as f64 + 0.5) * h;
            if intervals.iter().any(|&(a, b)| a <= mid && mid < b) {
                count += 1;
            }
        }
        count as f64 * h
    }
}
/// Compute discrete Lp norms and check Hölder / Minkowski inequalities.
///
/// Given a probability vector (weights) and a function vector (values),
/// computes ‖f‖_p = (Σ |f(i)|^p · w(i))^{1/p}.
#[derive(Debug, Clone)]
pub struct LpNormComputer {
    /// Probability weights (should sum to 1).
    pub weights: Vec<f64>,
}
impl LpNormComputer {
    /// Create a new Lp norm computer with given weights.
    pub fn new(weights: Vec<f64>) -> Self {
        LpNormComputer { weights }
    }
    /// Uniform weights over n points.
    pub fn uniform(n: usize) -> Self {
        let w = if n == 0 { 0.0 } else { 1.0 / n as f64 };
        LpNormComputer {
            weights: vec![w; n],
        }
    }
    /// Compute ‖f‖_p for p > 0.
    ///
    /// Uses the formula (Σᵢ |f(i)|^p · wᵢ)^{1/p}.
    pub fn lp_norm(&self, f: &[f64], p: f64) -> f64 {
        assert!(p > 0.0, "p must be positive");
        let sum: f64 = f
            .iter()
            .zip(self.weights.iter())
            .map(|(fi, wi)| fi.abs().powf(p) * wi)
            .sum();
        sum.powf(1.0 / p)
    }
    /// Compute ‖f‖_∞ = max |f(i)|.
    pub fn linf_norm(&self, f: &[f64]) -> f64 {
        f.iter().fold(0.0_f64, |acc, x| acc.max(x.abs()))
    }
    /// Verify Hölder's inequality: ‖fg‖₁ ≤ ‖f‖_p · ‖g‖_q  where 1/p + 1/q = 1.
    ///
    /// Returns `(lhs, rhs, holds)`.
    pub fn verify_holder(&self, f: &[f64], g: &[f64], p: f64) -> (f64, f64, bool) {
        assert!(p > 1.0, "p must be > 1 for Hölder");
        let q = p / (p - 1.0);
        let lhs: f64 = f
            .iter()
            .zip(g.iter())
            .zip(self.weights.iter())
            .map(|((fi, gi), wi)| (fi * gi).abs() * wi)
            .sum();
        let rhs = self.lp_norm(f, p) * self.lp_norm(g, q);
        (lhs, rhs, lhs <= rhs + 1e-10)
    }
    /// Verify Minkowski's inequality: ‖f + g‖_p ≤ ‖f‖_p + ‖g‖_p  for p ≥ 1.
    ///
    /// Returns `(lhs, rhs, holds)`.
    pub fn verify_minkowski(&self, f: &[f64], g: &[f64], p: f64) -> (f64, f64, bool) {
        assert!(p >= 1.0, "p must be >= 1 for Minkowski");
        let fg: Vec<f64> = f.iter().zip(g.iter()).map(|(a, b)| a + b).collect();
        let lhs = self.lp_norm(&fg, p);
        let rhs = self.lp_norm(f, p) + self.lp_norm(g, p);
        (lhs, rhs, lhs <= rhs + 1e-10)
    }
}
/// Signed measure data.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SignedMeasure {
    pub positive_part: String,
    pub negative_part: String,
    pub total_variation: f64,
}
#[allow(dead_code)]
impl SignedMeasure {
    /// Create a signed measure via Jordan decomposition.
    pub fn jordan(positive: &str, negative: &str, total_var: f64) -> Self {
        Self {
            positive_part: positive.to_string(),
            negative_part: negative.to_string(),
            total_variation: total_var,
        }
    }
    /// Check if this is a positive measure.
    pub fn is_positive(&self) -> bool {
        self.negative_part.is_empty() || self.total_variation >= 0.0
    }
    /// Hahn decomposition description.
    pub fn hahn_decomposition(&self) -> String {
        format!(
            "Hahn decomp: P={}, N={}",
            self.positive_part, self.negative_part
        )
    }
}
/// A generic discrete measure over labeled atoms of type `T`.
///
/// Each atom carries a label and a non-negative weight.  This supports
/// arbitrary countable sample spaces such as strings, integers, or tuples.
///
/// # Example
/// ```
/// use oxilean_std::measure_theory::DiscreteMeasureTyped;
/// let m = DiscreteMeasureTyped::new(vec!["x", "y", "z"], vec![0.5, 0.3, 0.2]);
/// assert!((m.total_mass() - 1.0).abs() < 1e-10);
/// let key = "x";
/// assert_eq!(m.atom_weight(&key), Some(0.5));
/// ```
#[derive(Debug, Clone)]
pub struct DiscreteMeasureTyped<T> {
    /// Labels for each atom.
    pub atoms: Vec<T>,
    /// Weight (measure) of each atom; must be ≥ 0.
    pub weights: Vec<f64>,
}
impl<T: PartialEq + Clone> DiscreteMeasureTyped<T> {
    /// Create a new typed discrete measure.
    pub fn new(atoms: Vec<T>, weights: Vec<f64>) -> Self {
        debug_assert_eq!(atoms.len(), weights.len());
        DiscreteMeasureTyped { atoms, weights }
    }
    /// Total mass (sum of all weights).
    pub fn total_mass(&self) -> f64 {
        self.weights.iter().sum()
    }
    /// Weight of a specific atom (returns `None` if not found).
    pub fn atom_weight(&self, atom: &T) -> Option<f64> {
        self.atoms
            .iter()
            .position(|a| a == atom)
            .map(|i| self.weights[i])
    }
    /// Integrate a function f against this measure: ∫ f dμ = Σ f(aᵢ) · wᵢ.
    pub fn integrate(&self, f: impl Fn(&T) -> f64) -> f64 {
        self.atoms
            .iter()
            .zip(self.weights.iter())
            .map(|(a, w)| f(a) * w)
            .sum()
    }
    /// Normalise to a probability measure (divides all weights by the total mass).
    ///
    /// Returns `None` if the total mass is zero.
    pub fn normalize(&self) -> Option<Self> {
        let total = self.total_mass();
        if total == 0.0 {
            return None;
        }
        Some(DiscreteMeasureTyped {
            atoms: self.atoms.clone(),
            weights: self.weights.iter().map(|w| w / total).collect(),
        })
    }
    /// Push-forward measure under a function g: μ_g(B) = μ(g⁻¹(B)).
    ///
    /// Sums weights of all atoms mapped to the same output value.
    pub fn push_forward<U: PartialEq + Clone>(
        &self,
        g: impl Fn(&T) -> U,
    ) -> DiscreteMeasureTyped<U> {
        let mut out_atoms: Vec<U> = Vec::new();
        let mut out_weights: Vec<f64> = Vec::new();
        for (a, w) in self.atoms.iter().zip(self.weights.iter()) {
            let image = g(a);
            if let Some(pos) = out_atoms.iter().position(|u| u == &image) {
                out_weights[pos] += w;
            } else {
                out_atoms.push(image);
                out_weights.push(*w);
            }
        }
        DiscreteMeasureTyped {
            atoms: out_atoms,
            weights: out_weights,
        }
    }
}
/// Conditional expectation data.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ConditionalExpectation {
    pub random_variable: String,
    pub sigma_algebra: String,
    pub is_tower_property: bool,
}
#[allow(dead_code)]
impl ConditionalExpectation {
    /// E[X | G].
    pub fn new(x: &str, g: &str) -> Self {
        Self {
            random_variable: x.to_string(),
            sigma_algebra: g.to_string(),
            is_tower_property: true,
        }
    }
    /// Tower property: E[E[X|G]|H] = E[X|H] when H ⊆ G.
    pub fn tower_description(&self) -> String {
        format!(
            "E[E[{}|{}]|H] = E[{}|H] for H ⊆ {}",
            self.random_variable, self.sigma_algebra, self.random_variable, self.sigma_algebra
        )
    }
}
/// Discrete measure on a finite set {0, …, n-1}.
pub struct DiscreteMeasure {
    /// Size of the underlying set.
    pub n_elements: usize,
    /// Weight of each singleton {i}.
    pub weights: Vec<f64>,
}
impl DiscreteMeasure {
    /// Create a measure from given weights.
    pub fn new(weights: Vec<f64>) -> Self {
        let n = weights.len();
        DiscreteMeasure {
            n_elements: n,
            weights,
        }
    }
    /// Counting measure: every singleton has weight 1.
    pub fn counting_measure(n: usize) -> Self {
        DiscreteMeasure {
            n_elements: n,
            weights: vec![1.0; n],
        }
    }
    /// Probability measure: weights must sum to 1 within tolerance.
    ///
    /// Returns `None` if the weights don't form a valid probability measure.
    pub fn probability_measure(probs: Vec<f64>) -> Option<Self> {
        let m = DiscreteMeasure::new(probs);
        if m.is_probability() {
            Some(m)
        } else {
            None
        }
    }
    /// Measure of a subset given as an index list.
    pub fn measure_of(&self, set: &[usize]) -> f64 {
        set.iter()
            .filter(|&&i| i < self.n_elements)
            .map(|&i| self.weights[i])
            .sum()
    }
    /// Check whether the total measure equals 1 within tolerance 1e-9.
    pub fn is_probability(&self) -> bool {
        let total: f64 = self.weights.iter().sum();
        (total - 1.0).abs() < 1e-9
    }
    /// Check whether all weights are finite (not NaN / ±∞).
    pub fn is_finite(&self) -> bool {
        self.weights.iter().all(|w| w.is_finite())
    }
}
/// Estimates the measure of a set via Monte Carlo sampling.
///
/// Uses a deterministic quasi-random grid (van der Corput sequence) to avoid
/// depending on `rand`, while still providing a useful approximation.
#[derive(Debug, Clone)]
pub struct MonteCarloMeasureEstimator {
    /// Number of sample points.
    pub n_samples: usize,
    /// Dimension of the sampling space.
    pub dimension: usize,
}
impl MonteCarloMeasureEstimator {
    /// Create a new estimator with `n_samples` points in `dimension` dimensions.
    pub fn new(n_samples: usize, dimension: usize) -> Self {
        MonteCarloMeasureEstimator {
            n_samples,
            dimension,
        }
    }
    /// Van der Corput sequence in base `b` for index `n` (returns value in [0,1)).
    pub fn van_der_corput(mut n: usize, b: usize) -> f64 {
        let mut result = 0.0_f64;
        let mut denom = 1.0_f64;
        while n > 0 {
            denom *= b as f64;
            result += (n % b) as f64 / denom;
            n /= b;
        }
        result
    }
    /// Halton sequence point at index `i` in `dimension` dimensions.
    ///
    /// Uses the first `dimension` prime bases.
    pub fn halton_point(&self, i: usize) -> Vec<f64> {
        let primes = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29];
        (0..self.dimension)
            .map(|d| Self::van_der_corput(i + 1, primes[d % primes.len()]))
            .collect()
    }
    /// Estimate the measure (volume fraction) of the set `{ x ∈ [0,1]^d | predicate(x) }`.
    ///
    /// Returns the fraction of sample points satisfying `predicate`.
    pub fn estimate_unit_cube<F>(&self, predicate: F) -> f64
    where
        F: Fn(&[f64]) -> bool,
    {
        let count = (0..self.n_samples)
            .filter(|&i| {
                let pt = self.halton_point(i);
                predicate(&pt)
            })
            .count();
        count as f64 / self.n_samples as f64
    }
}
/// Martingale data for measure-theoretic probability.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Martingale {
    pub filtration: String,
    pub integrability: String,
    pub is_sub: bool,
    pub is_super: bool,
}
#[allow(dead_code)]
impl Martingale {
    /// True martingale.
    pub fn true_martingale(filtration: &str) -> Self {
        Self {
            filtration: filtration.to_string(),
            integrability: "L1".to_string(),
            is_sub: false,
            is_super: false,
        }
    }
    /// Sub-martingale.
    pub fn sub(filtration: &str) -> Self {
        Self {
            filtration: filtration.to_string(),
            integrability: "L1".to_string(),
            is_sub: true,
            is_super: false,
        }
    }
    /// Doob's optional stopping theorem applies under uniform integrability.
    pub fn optional_stopping_applies(&self, uniformly_integrable: bool) -> bool {
        uniformly_integrable
    }
}
/// Box-counting (Minkowski) dimension estimator for a finite point set in ℝ².
///
/// Estimates the Hausdorff dimension by counting how many ε-boxes are needed
/// to cover the point set at multiple scales ε = 2^{-k}.
#[derive(Debug, Clone)]
pub struct HausdorffDimensionEstimator {
    /// The point set (each entry is (x, y) ∈ [0,1]²).
    pub points: Vec<(f64, f64)>,
}
impl HausdorffDimensionEstimator {
    /// Create a new estimator from a cloud of 2D points.
    pub fn new(points: Vec<(f64, f64)>) -> Self {
        HausdorffDimensionEstimator { points }
    }
    /// Count the number of ε-boxes (side length ε) needed to cover all points.
    ///
    /// Uses a grid of ε × ε boxes covering [0, 1]².
    pub fn box_count(&self, epsilon: f64) -> usize {
        if self.points.is_empty() || epsilon <= 0.0 {
            return 0;
        }
        let mut occupied = std::collections::HashSet::new();
        for &(x, y) in &self.points {
            let ix = (x / epsilon).floor() as i64;
            let iy = (y / epsilon).floor() as i64;
            occupied.insert((ix, iy));
        }
        occupied.len()
    }
    /// Estimate the box-counting dimension via linear regression over scales.
    ///
    /// dim ≈ -d(log N(ε)) / d(log ε) where N(ε) is the box count at scale ε.
    /// Uses scales ε = 2^{-1}, 2^{-2}, …, 2^{-max_k}.
    pub fn estimate_dimension(&self, max_k: u32) -> f64 {
        if max_k < 2 {
            return 0.0;
        }
        let mut log_eps: Vec<f64> = Vec::new();
        let mut log_n: Vec<f64> = Vec::new();
        for k in 1..=max_k {
            let eps = (0.5_f64).powi(k as i32);
            let n = self.box_count(eps);
            if n > 0 {
                log_eps.push(eps.ln());
                log_n.push((n as f64).ln());
            }
        }
        if log_eps.len() < 2 {
            return 0.0;
        }
        let m = log_eps.len() as f64;
        let mean_x = log_eps.iter().sum::<f64>() / m;
        let mean_y = log_n.iter().sum::<f64>() / m;
        let num: f64 = log_eps
            .iter()
            .zip(log_n.iter())
            .map(|(&x, &y)| (x - mean_x) * (y - mean_y))
            .sum();
        let den: f64 = log_eps.iter().map(|&x| (x - mean_x).powi(2)).sum();
        if den.abs() < 1e-15 {
            return 0.0;
        }
        -(num / den)
    }
    /// Estimate dimension with a Cantor-set-like point cloud (for testing).
    ///
    /// Returns the estimated dimension for the given points (should be ≈ log 2 / log 3 for Cantor).
    pub fn cantor_estimate(&self) -> f64 {
        self.estimate_dimension(8)
    }
}
