//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

/// Ergodic decomposition theorem data.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ErgodicDecompositionV2 {
    pub system_name: String,
    pub invariant_measures: Vec<String>,
}
impl ErgodicDecompositionV2 {
    #[allow(dead_code)]
    pub fn new(system: &str, measures: Vec<&str>) -> Self {
        Self {
            system_name: system.to_string(),
            invariant_measures: measures.into_iter().map(String::from).collect(),
        }
    }
    #[allow(dead_code)]
    pub fn statement(&self) -> String {
        format!(
            "Every invariant measure on {} decomposes into ergodic components",
            self.system_name
        )
    }
    #[allow(dead_code)]
    pub fn num_components(&self) -> usize {
        self.invariant_measures.len()
    }
}
/// Relationship between metric and topological entropy.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EntropyRelation {
    /// h_μ(T) < h_top(T) for the given invariant measure.
    KSLessTopological,
    /// h_μ(T) = h_top(T) — μ is a measure of maximal entropy.
    KSEqualsTopological,
    /// The variational principle: h_top(T) = sup_μ h_μ(T).
    VariationalPrinciple,
}
/// Measure of maximal entropy.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct MaximalEntropyMeasure {
    pub system_name: String,
    pub entropy: f64,
    pub is_unique: bool,
}
impl MaximalEntropyMeasure {
    #[allow(dead_code)]
    pub fn new(system: &str, entropy: f64, unique: bool) -> Self {
        Self {
            system_name: system.to_string(),
            entropy,
            is_unique: unique,
        }
    }
    #[allow(dead_code)]
    pub fn achieves_topological_entropy(&self) -> bool {
        true
    }
}
/// Orthogonality of characteristic functions in L²(μ) for ergodic systems.
#[derive(Debug, Clone)]
pub struct OrthogonalityRelation {
    /// Name of the measure-preserving system.
    pub system: String,
}
impl OrthogonalityRelation {
    /// Create an orthogonality relation record.
    pub fn new(system: impl Into<String>) -> Self {
        Self {
            system: system.into(),
        }
    }
    /// Statement that indicator functions of disjoint invariant sets are orthogonal.
    pub fn characteristic_functions_orthogonal(&self) -> String {
        format!(
            "Orthogonality in L²(μ) for system '{}': \
             If A, B are measurable sets with A ∩ B = ∅ and both T-invariant, \
             then ⟨1_A, 1_B⟩_{{L²}} = ∫ 1_A · 1_B dμ = μ(A ∩ B) = 0. \
             More generally, the Koopman operator U_T is unitary on L²(μ), \
             and the eigenspaces for distinct eigenvalues are orthogonal. \
             For ergodic systems, the only eigenvalue-1 eigenspace is the constants.",
            self.system
        )
    }
}
/// Canonical ergodic transformations on standard spaces.
#[derive(Debug, Clone, PartialEq)]
pub enum ErgodicTransformation {
    /// Irrational rotation T(x) = x + α (mod 1), α ∈ ℝ \ ℚ.
    Rotation(f64),
    /// Doubling map T(x) = 2x (mod 1) on [0,1).
    Doubling,
    /// Baker's map on the unit square — area-preserving, hyperbolic.
    BakerMap,
    /// Arnold's cat map on the 2-torus — Anosov diffeomorphism.
    CatMap,
    /// Full shift on a finite alphabet — prototypical Bernoulli system.
    Shift,
}
impl ErgodicTransformation {
    /// Returns `true` if the transformation is ergodic with respect to Lebesgue measure.
    pub fn is_ergodic(&self) -> bool {
        match self {
            ErgodicTransformation::Rotation(alpha) => {
                let frac = alpha.fract().abs();
                frac > 1e-12 && (1.0 - frac) > 1e-12
            }
            ErgodicTransformation::Doubling => true,
            ErgodicTransformation::BakerMap => true,
            ErgodicTransformation::CatMap => true,
            ErgodicTransformation::Shift => true,
        }
    }
    /// Returns `true` if the transformation is (strong) mixing.
    pub fn is_mixing(&self) -> bool {
        match self {
            ErgodicTransformation::Rotation(_) => false,
            ErgodicTransformation::Doubling => true,
            ErgodicTransformation::BakerMap => true,
            ErgodicTransformation::CatMap => true,
            ErgodicTransformation::Shift => true,
        }
    }
    /// Returns `true` if the transformation is weak mixing.
    pub fn is_weak_mixing(&self) -> bool {
        match self {
            ErgodicTransformation::Rotation(_) => false,
            ErgodicTransformation::Doubling => true,
            ErgodicTransformation::BakerMap => true,
            ErgodicTransformation::CatMap => true,
            ErgodicTransformation::Shift => true,
        }
    }
}
/// A distal dynamical system (proximal relation = diagonal).
#[derive(Debug, Clone)]
pub struct DistalSystem {
    /// Name of the distal system.
    pub name: String,
}
impl DistalSystem {
    /// Create a distal system record.
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
    /// Returns a statement that the proximal relation is trivial (= diagonal).
    pub fn proximal_relation_trivial(&self) -> String {
        format!(
            "Distal system '{}': A compact metric dynamical system (X, T) is distal \
             when for all x ≠ y, inf_{{n ∈ ℤ}} d(T^n x, T^n y) > 0. \
             Equivalently, the proximal relation P ⊆ X × X equals the diagonal Δ. \
             Furstenberg's structure theorem: every distal system is an inverse limit \
             of isometric extensions, and distal systems are never weakly mixing \
             (unless trivial).",
            self.name
        )
    }
    /// Returns `true` — by definition a distal system has trivial proximal relation.
    pub fn is_distal(&self) -> bool {
        true
    }
}
/// Ergodic action of a group.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ErgodicGroupAction {
    pub group: String,
    pub space: String,
    pub is_free: bool,
    pub is_measure_preserving: bool,
}
impl ErgodicGroupAction {
    #[allow(dead_code)]
    pub fn new(group: &str, space: &str, free: bool, mp: bool) -> Self {
        Self {
            group: group.to_string(),
            space: space.to_string(),
            is_free: free,
            is_measure_preserving: mp,
        }
    }
    #[allow(dead_code)]
    pub fn amenable_group_description(&self) -> String {
        format!(
            "Amenable group {} action: Folner sequences, mean ergodic theorem",
            self.group
        )
    }
    #[allow(dead_code)]
    pub fn orbit_equivalence_description(&self) -> String {
        format!(
            "OE: actions of {} and H on {} are orbit-equivalent if orbits agree a.e.",
            self.group, self.space
        )
    }
}
/// A measurable partition used as a generating basis for KS entropy.
#[derive(Debug, Clone)]
pub struct KolmogorovSinaiBasis {
    /// Name identifying this partition scheme.
    pub name: String,
    /// List of partition atom names.
    pub partitions: Vec<String>,
}
/// Kolmogorov-Sinai entropy of a partition.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct KolmogorovSinaiEntropyV2 {
    pub system_name: String,
    pub partition_name: String,
    pub entropy: f64,
}
impl KolmogorovSinaiEntropyV2 {
    #[allow(dead_code)]
    pub fn new(system: &str, partition: &str, h: f64) -> Self {
        Self {
            system_name: system.to_string(),
            partition_name: partition.to_string(),
            entropy: h,
        }
    }
    #[allow(dead_code)]
    pub fn generating_partition_description(&self) -> String {
        format!(
            "h(T) = h(T, P) when P is a generating partition for {}",
            self.system_name
        )
    }
    #[allow(dead_code)]
    pub fn bernoulli_shift_entropy(probabilities: &[f64]) -> f64 {
        -probabilities
            .iter()
            .filter(|&&p| p > 0.0)
            .map(|&p| p * p.ln())
            .sum::<f64>()
    }
}
/// A subshift over a finite alphabet defined by its set of forbidden words.
#[derive(Debug, Clone)]
pub struct SubShift {
    /// Size of the alphabet Σ (number of symbols).
    pub alphabet_size: usize,
    /// List of forbidden words (each word is a string over the alphabet symbols).
    pub forbidden_words: Vec<String>,
}
impl SubShift {
    /// Create a new subshift with the given alphabet size and no forbidden words
    /// (the full shift).
    pub fn new(alphabet_size: usize) -> Self {
        Self {
            alphabet_size,
            forbidden_words: Vec::new(),
        }
    }
    /// Add a forbidden word to define the subshift.
    pub fn add_forbidden_word(&mut self, word: impl Into<String>) {
        self.forbidden_words.push(word.into());
    }
    /// Topological entropy of the subshift: log(alphabet_size) minus corrections
    /// from forbidden words (exact computation requires the transfer matrix; this
    /// gives the full-shift upper bound when no corrections are applied).
    pub fn entropy(&self) -> f64 {
        if self.alphabet_size == 0 {
            return 0.0;
        }
        (self.alphabet_size as f64).ln()
    }
    /// A subshift is sofic if it is the image of a shift of finite type under a
    /// 1-block code. For a subshift defined only by forbidden words, this is always
    /// true (every SFT is sofic).
    pub fn is_sofic(&self) -> bool {
        true
    }
    /// A subshift of finite type (SFT) is defined by a finite list of forbidden words
    /// of bounded length. Here we treat all our subshifts as SFTs.
    pub fn is_shift_of_finite_type(&self) -> bool {
        true
    }
}
/// An approximate ergodic decomposition computed from empirical orbit data.
///
/// Given a finite collection of observed orbits, we cluster them into
/// approximate ergodic components by comparing empirical time averages.
#[derive(Debug, Clone)]
pub struct ErgodicDecompositionApprox {
    /// Number of approximate ergodic components found.
    pub num_components: usize,
    /// Empirical weight (relative frequency) of each component.
    pub weights: Vec<f64>,
    /// Representative time average of a test function for each component.
    pub representative_averages: Vec<f64>,
}
impl ErgodicDecompositionApprox {
    /// Construct directly from known weights and averages.
    pub fn new(weights: Vec<f64>, representative_averages: Vec<f64>) -> Self {
        let n = weights.len();
        Self {
            num_components: n,
            weights,
            representative_averages,
        }
    }
    /// Approximate decomposition from a list of empirical time averages.
    ///
    /// Groups observations by proximity (within `tol`) and computes cluster weights.
    pub fn from_time_averages(averages: &[f64], tol: f64) -> Self {
        if averages.is_empty() {
            return Self::new(vec![], vec![]);
        }
        let mut sorted = averages.to_vec();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        let mut clusters: Vec<(f64, usize)> = Vec::new();
        for val in &sorted {
            if let Some(last) = clusters.last_mut() {
                if (last.0 - val).abs() <= tol {
                    let n = last.1;
                    last.0 = (last.0 * n as f64 + val) / (n + 1) as f64;
                    last.1 += 1;
                    continue;
                }
            }
            clusters.push((*val, 1));
        }
        let total = averages.len() as f64;
        let weights: Vec<f64> = clusters
            .iter()
            .map(|&(_, cnt)| cnt as f64 / total)
            .collect();
        let rep_avgs: Vec<f64> = clusters.iter().map(|&(mean, _)| mean).collect();
        Self::new(weights, rep_avgs)
    }
    /// Returns `true` if the system appears to be ergodic (single component).
    pub fn is_ergodic(&self) -> bool {
        self.num_components <= 1
    }
    /// KS entropy lower bound from the decomposition: Σ w_i * h_i.
    /// Here we approximate h_i ≈ 0 (no information from averages alone).
    pub fn total_weight(&self) -> f64 {
        self.weights.iter().sum()
    }
    /// Description of the decomposition.
    pub fn description(&self) -> String {
        if self.num_components == 0 {
            "Empty decomposition".to_string()
        } else if self.num_components == 1 {
            format!(
                "Ergodic system: single component with average {:.4}",
                self.representative_averages[0]
            )
        } else {
            let parts: Vec<String> = self
                .weights
                .iter()
                .zip(self.representative_averages.iter())
                .enumerate()
                .map(|(i, (w, avg))| format!("C_{}: w={:.3}, avg={:.4}", i, w, avg))
                .collect();
            format!(
                "{} ergodic components: {}",
                self.num_components,
                parts.join("; ")
            )
        }
    }
}
/// Furstenberg's correspondence principle between combinatorics and ergodic theory.
#[derive(Debug, Clone)]
pub struct FurstenbergCorrespondence {
    /// Description of the combinatorial set being studied.
    pub set_description: String,
}
impl FurstenbergCorrespondence {
    /// Create a Furstenberg correspondence.
    pub fn new(set_description: impl Into<String>) -> Self {
        Self {
            set_description: set_description.into(),
        }
    }
    /// Return the combinatorial implications derived via the correspondence.
    pub fn combinatorial_implications(&self) -> String {
        format!(
            "Furstenberg Correspondence Principle for '{}': \
             Every set S ⊆ ℕ with positive upper Banach density d*(S) > 0 corresponds \
             to a measure-preserving system (X, μ, T) and a measurable set A with μ(A) = d*(S), \
             such that d*(S ∩ (S-n₁) ∩ … ∩ (S-nₖ)) ≥ μ(A ∩ T^{{-n₁}}A ∩ … ∩ T^{{-nₖ}}A). \
             This reduces Szemerédi-type theorems to ergodic recurrence results.",
            self.set_description
        )
    }
}
/// Records data for orbit equivalence between two measure-preserving systems.
///
/// Dye's theorem: all ergodic probability-measure-preserving (pmp) actions of ℤ
/// are orbit equivalent.  More generally, two pmp actions of amenable groups are
/// orbit equivalent iff they generate the same equivalence relation μ-a.e.
#[derive(Debug, Clone)]
pub struct OrbitEquivalenceRecord {
    /// Name of the first system.
    pub system_a: String,
    /// Name of the second system.
    pub system_b: String,
    /// Whether the two systems are orbit equivalent.
    pub orbit_equivalent: bool,
    /// Whether the orbit equivalence is strong (also conjugate return-time partitions).
    pub strong: bool,
}
impl OrbitEquivalenceRecord {
    /// Construct an orbit equivalence record.
    pub fn new(
        system_a: impl Into<String>,
        system_b: impl Into<String>,
        orbit_equivalent: bool,
        strong: bool,
    ) -> Self {
        Self {
            system_a: system_a.into(),
            system_b: system_b.into(),
            orbit_equivalent,
            strong,
        }
    }
    /// Statement of Dye's theorem.
    pub fn dye_theorem_statement() -> &'static str {
        "Dye's Theorem: Any two ergodic probability-measure-preserving actions of ℤ \
         (or more generally of any countable amenable group) on a standard probability \
         space are orbit equivalent. In particular, the orbit equivalence class of a \
         free ergodic pmp ℤ-action is determined solely by ergodicity — not by entropy \
         or other spectral properties."
    }
    /// Returns a description of the orbit equivalence relation.
    pub fn describe(&self) -> String {
        if self.orbit_equivalent {
            let strength = if self.strong { "strongly" } else { "weakly" };
            format!(
                "Systems '{}' and '{}' are {} orbit equivalent.",
                self.system_a, self.system_b, strength
            )
        } else {
            format!(
                "Systems '{}' and '{}' are NOT orbit equivalent.",
                self.system_a, self.system_b
            )
        }
    }
}
/// Kolmogorov-Sinai (metric) entropy of a measure-preserving system.
#[derive(Debug, Clone)]
pub struct MetricEntropy {
    /// Name of the associated dynamical system.
    pub system: String,
    /// Entropy value h_μ(T) ≥ 0 (in nats or bits depending on convention).
    pub value: f64,
    /// `true` if the entropy is finite (always for standard systems with finite partitions).
    pub is_finite: bool,
}
impl MetricEntropy {
    /// Create a new metric entropy record.
    pub fn new(system: impl Into<String>, value: f64) -> Self {
        let is_finite = value.is_finite();
        Self {
            system: system.into(),
            value,
            is_finite,
        }
    }
    /// Compute entropy from a uniform partition of `n` atoms:
    ///   h = log(n)  (the entropy of the Bernoulli(1/n, …, 1/n) shift).
    pub fn from_partition(system: impl Into<String>, n_atoms: usize) -> Self {
        let value = if n_atoms > 0 {
            (n_atoms as f64).ln()
        } else {
            0.0
        };
        Self::new(system, value)
    }
    /// Check whether this entropy equals log(λ) up to `tol`, as expected for
    /// a Bernoulli shift with eigenvalue `lambda`.
    pub fn is_bernoulli_entropy(&self, lambda: f64, tol: f64) -> bool {
        (self.value - lambda.ln()).abs() < tol
    }
}
/// An invariant measure together with ergodicity information.
#[derive(Debug, Clone)]
pub struct InvariantMeasure {
    /// Name of the associated dynamical system.
    pub system: String,
    /// Whether the measure is a probability measure (total mass 1).
    pub is_probability: bool,
    /// Whether the measure is ergodic (only trivial invariant sets).
    pub is_ergodic: bool,
}
impl InvariantMeasure {
    /// Create a new invariant measure descriptor.
    pub fn new(system: impl Into<String>, is_probability: bool, is_ergodic: bool) -> Self {
        Self {
            system: system.into(),
            is_probability,
            is_ergodic,
        }
    }
    /// Returns `true` if this is a probability measure.
    pub fn is_probability_measure(&self) -> bool {
        self.is_probability
    }
    /// Poincaré recurrence is guaranteed whenever the measure is a finite
    /// (in particular probability) measure.
    pub fn satisfies_poincare_recurrence(&self) -> bool {
        self.is_probability
    }
}
/// A measure-preserving dynamical system (X, μ, T).
///
/// Captures the triple (space, measure, transformation) where T preserves μ,
/// i.e., μ(T⁻¹(A)) = μ(A) for all measurable A.
#[derive(Debug, Clone)]
pub struct MeasurePreservingSystem {
    /// Name of the underlying measurable space X.
    pub space: String,
    /// Name of the T-invariant measure μ.
    pub measure: String,
    /// Name / description of the measurable transformation T : X → X.
    pub transformation: String,
}
impl MeasurePreservingSystem {
    /// Create a new measure-preserving system.
    pub fn new(
        space: impl Into<String>,
        measure: impl Into<String>,
        transformation: impl Into<String>,
    ) -> Self {
        Self {
            space: space.into(),
            measure: measure.into(),
            transformation: transformation.into(),
        }
    }
}
/// Computes Lyapunov exponents for a linear cocycle over an ergodic system.
///
/// For a measure-preserving system (X, μ, T) and a measurable cocycle A: X → GL(d, ℝ),
/// Oseledets' theorem guarantees the existence of Lyapunov exponents λ_1 ≥ … ≥ λ_d.
#[derive(Debug, Clone)]
pub struct LyapunovExponentComputer {
    /// Name of the base system.
    pub system: String,
    /// Dimension d of the linear cocycle (number of Lyapunov exponents).
    pub dimension: usize,
    /// Approximate Lyapunov exponents (in decreasing order).
    pub exponents: Vec<f64>,
}
impl LyapunovExponentComputer {
    /// Construct with known exponents (e.g. from numerical simulation).
    pub fn new(system: impl Into<String>, exponents: Vec<f64>) -> Self {
        let dim = exponents.len();
        Self {
            system: system.into(),
            dimension: dim,
            exponents,
        }
    }
    /// Returns the largest Lyapunov exponent λ_max.
    pub fn max_exponent(&self) -> Option<f64> {
        self.exponents.first().copied()
    }
    /// Returns the sum of all Lyapunov exponents (= log|det A| by Liouville).
    pub fn exponent_sum(&self) -> f64 {
        self.exponents.iter().sum()
    }
    /// Returns `true` if the system is non-uniformly hyperbolic (λ_max > 0 and λ_min < 0).
    pub fn is_non_uniformly_hyperbolic(&self) -> bool {
        let max = self
            .exponents
            .iter()
            .cloned()
            .fold(f64::NEG_INFINITY, f64::max);
        let min = self.exponents.iter().cloned().fold(f64::INFINITY, f64::min);
        max > 0.0 && min < 0.0
    }
    /// Pesin formula: h_μ(T) = Σ_{λ_i > 0} λ_i (for C² diffeomorphisms with SRB measure).
    pub fn pesin_entropy_formula(&self) -> f64 {
        self.exponents.iter().filter(|&&e| e > 0.0).sum()
    }
    /// Oseledets theorem statement.
    pub fn oseledets_theorem(&self) -> String {
        format!(
            "Oseledets Multiplicative Ergodic Theorem for '{}' (dim={}): \
             For μ-a.e. x ∈ X, the limits \
             λ_i(x) = lim_{{n→∞}} (1/n) log ‖Aⁿ(x) v‖ exist and take values \
             in {{λ_1 ≥ λ_2 ≥ … ≥ λ_{}}}. The Lyapunov spectrum is \
             μ-a.e. constant (by ergodicity). Exponents: {:?}",
            self.system, self.dimension, self.dimension, self.exponents
        )
    }
    /// Ruelle inequality: h_μ(T) ≤ Σ_{λ_i > 0} λ_i.
    pub fn ruelle_inequality_satisfied(&self, ks_entropy: f64) -> bool {
        ks_entropy <= self.pesin_entropy_formula() + 1e-10
    }
}
/// A shift space (subshift) represented by its transition matrix.
///
/// Subshifts of finite type (SFTs) are specified by a 0/1 adjacency matrix A:
/// allowed transitions are those with A[i][j] = 1.  The topological entropy
/// equals log(spectral_radius(A)).
#[derive(Debug, Clone)]
pub struct SymbolicDynamicsShift {
    /// Number of states (alphabet size of the edge shift).
    pub num_states: usize,
    /// Adjacency matrix (row i, column j = 1 iff i → j is allowed).
    pub adjacency: Vec<Vec<u8>>,
    /// Whether this shift is known to be mixing (topologically mixing SFT).
    pub is_mixing: bool,
}
impl SymbolicDynamicsShift {
    /// Construct a shift from an adjacency matrix.
    pub fn new(adjacency: Vec<Vec<u8>>, is_mixing: bool) -> Self {
        let n = adjacency.len();
        Self {
            num_states: n,
            adjacency,
            is_mixing,
        }
    }
    /// Construct the full shift on `n` symbols (all transitions allowed).
    pub fn full_shift(n: usize) -> Self {
        let adj = vec![vec![1u8; n]; n];
        Self::new(adj, true)
    }
    /// Construct the golden mean shift (no two consecutive 1s).
    /// States: 0 (last symbol was 0), 1 (last symbol was 1).
    /// Transitions: 0→0, 0→1, 1→0 (but NOT 1→1).
    pub fn golden_mean_shift() -> Self {
        let adj = vec![vec![1, 1], vec![1, 0]];
        Self::new(adj, true)
    }
    /// Compute the spectral radius of the adjacency matrix via the power method.
    #[allow(clippy::needless_range_loop)]
    pub fn spectral_radius(&self) -> f64 {
        let n = self.num_states;
        if n == 0 {
            return 0.0;
        }
        let mut v = vec![1.0_f64; n];
        let mut radius = 0.0_f64;
        for _ in 0..200 {
            let mut w = vec![0.0_f64; n];
            for i in 0..n {
                for j in 0..n {
                    w[i] += self.adjacency[i][j] as f64 * v[j];
                }
            }
            let norm: f64 = w.iter().map(|x| x * x).sum::<f64>().sqrt();
            if norm < 1e-300 {
                break;
            }
            radius = norm / (v.iter().map(|x| x * x).sum::<f64>().sqrt());
            v = w.iter().map(|x| x / norm).collect();
        }
        radius
    }
    /// Topological entropy: log(spectral_radius(A)).
    pub fn topological_entropy(&self) -> f64 {
        let r = self.spectral_radius();
        if r > 1e-300 {
            r.ln()
        } else {
            f64::NEG_INFINITY
        }
    }
    /// Number of admissible words of length `n` (traces of A^n).
    #[allow(clippy::needless_range_loop)]
    pub fn count_words(&self, n: usize) -> u64 {
        let sz = self.num_states;
        if sz == 0 || n == 0 {
            return sz as u64;
        }
        let mut mat: Vec<Vec<u64>> = self
            .adjacency
            .iter()
            .map(|row| row.iter().map(|&x| x as u64).collect())
            .collect();
        for _ in 1..n {
            let mut next = vec![vec![0u64; sz]; sz];
            for i in 0..sz {
                for k in 0..sz {
                    if mat[i][k] == 0 {
                        continue;
                    }
                    for j in 0..sz {
                        next[i][j] = next[i][j]
                            .saturating_add(mat[i][k].saturating_mul(self.adjacency[k][j] as u64));
                    }
                }
            }
            mat = next;
        }
        mat.iter().flat_map(|row| row.iter()).sum()
    }
    /// Returns `true` if the shift is irreducible (adjacency matrix is irreducible).
    pub fn is_irreducible(&self) -> bool {
        let n = self.num_states;
        if n == 0 {
            return true;
        }
        let mut reachable = vec![false; n];
        let mut stack = vec![0usize];
        reachable[0] = true;
        while let Some(s) = stack.pop() {
            for t in 0..n {
                if self.adjacency[s][t] == 1 && !reachable[t] {
                    reachable[t] = true;
                    stack.push(t);
                }
            }
        }
        reachable.iter().all(|&r| r)
    }
    /// Returns `true` for a mixing SFT: irreducible + aperiodic (gcd of cycle lengths = 1).
    pub fn is_topologically_mixing(&self) -> bool {
        self.is_mixing && self.is_irreducible()
    }
}
/// Unipotent flow on a homogeneous space.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct UnipotentFlow {
    pub group: String,
    pub lattice: String,
    pub unipotent_element: String,
}
impl UnipotentFlow {
    #[allow(dead_code)]
    pub fn new(g: &str, gamma: &str, u: &str) -> Self {
        Self {
            group: g.to_string(),
            lattice: gamma.to_string(),
            unipotent_element: u.to_string(),
        }
    }
    #[allow(dead_code)]
    pub fn ratner_theorem_statement(&self) -> String {
        format!(
            "Ratner: orbit closure of unipotent flow on {}/{} is homogeneous",
            self.group, self.lattice
        )
    }
    #[allow(dead_code)]
    pub fn is_equidistributed(&self) -> bool {
        true
    }
}
/// Furstenberg correspondence principle.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FurstenbergCorrespondenceV2 {
    pub combinatorial_statement: String,
    pub ergodic_statement: String,
}
impl FurstenbergCorrespondenceV2 {
    #[allow(dead_code)]
    pub fn szemeredi() -> Self {
        Self {
            combinatorial_statement: "Every subset of integers with positive upper density contains arithmetic progressions of all lengths (Szemeredi)"
                .to_string(),
            ergodic_statement: "Every ergodic system with invariant measure > 0 set has multiple recurrence (Furstenberg)"
                .to_string(),
        }
    }
    #[allow(dead_code)]
    pub fn principle_statement(&self) -> String {
        "Furstenberg correspondence: combinatorial density results <-> ergodic multiple recurrence"
            .to_string()
    }
}
/// The Perron-Frobenius theorem for non-negative matrices.
#[derive(Debug, Clone)]
pub struct PerronFrobeniusThm {
    /// Type of matrix: "positive", "non-negative", "primitive", "irreducible".
    pub matrix_type: String,
}
impl PerronFrobeniusThm {
    /// Create a Perron-Frobenius theorem record.
    pub fn new(matrix_type: impl Into<String>) -> Self {
        Self {
            matrix_type: matrix_type.into(),
        }
    }
    /// Statement that the leading eigenvalue (spectral radius) is positive and real.
    pub fn leading_eigenvalue_positive(&self) -> String {
        format!(
            "Perron-Frobenius ({} matrix): The spectral radius ρ(A) > 0 is a simple \
             eigenvalue of A (the Perron root). The corresponding left and right \
             eigenvectors (Perron vectors) have strictly positive entries. \
             All other eigenvalues satisfy |λ| ≤ ρ(A), with strict inequality \
             for primitive matrices.",
            self.matrix_type
        )
    }
    /// Statement that there is a unique stationary probability distribution.
    pub fn unique_stationary(&self) -> String {
        match self.matrix_type.as_str() {
            "positive" | "primitive" => {
                format!(
                    "Unique stationary distribution ({}): A primitive/positive row-stochastic \
                 matrix has a unique stationary distribution π > 0 (componentwise), \
                 and the Markov chain converges to π from any initial distribution.",
                    self.matrix_type
                )
            }
            "irreducible" => "Unique stationary distribution (irreducible): \
                 An irreducible non-negative matrix has a unique (up to scaling) \
                 Perron eigenvector with positive entries."
                .to_string(),
            t => {
                format!(
                    "Stationary distribution for {} matrix: existence and uniqueness \
                 depend on primitivity/irreducibility.",
                    t
                )
            }
        }
    }
}
/// Topological entropy of a system.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct TopologicalEntropyV2 {
    pub system_name: String,
    pub value: f64,
    pub algorithm: String,
}
impl TopologicalEntropyV2 {
    #[allow(dead_code)]
    pub fn new(system: &str, h: f64) -> Self {
        Self {
            system_name: system.to_string(),
            value: h,
            algorithm: "Bowen-Dinaburg definition".to_string(),
        }
    }
    #[allow(dead_code)]
    pub fn shift_map(alphabet_size: usize) -> Self {
        let h = (alphabet_size as f64).ln();
        Self::new(&format!("{alphabet_size}-shift"), h)
    }
    #[allow(dead_code)]
    pub fn variational_principle(&self) -> String {
        format!(
            "h_top({}) = sup over invariant measures of measure-theoretic entropy",
            self.system_name
        )
    }
    #[allow(dead_code)]
    pub fn is_finite(&self) -> bool {
        self.value.is_finite()
    }
}
/// Joinings of ergodic systems.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct JoiningStructure {
    pub system_a: String,
    pub system_b: String,
    pub has_trivial_joining: bool,
}
impl JoiningStructure {
    #[allow(dead_code)]
    pub fn new(a: &str, b: &str) -> Self {
        Self {
            system_a: a.to_string(),
            system_b: b.to_string(),
            has_trivial_joining: false,
        }
    }
    #[allow(dead_code)]
    pub fn self_joining(system: &str) -> Self {
        Self::new(system, system)
    }
    #[allow(dead_code)]
    pub fn disjoint_systems_description(&self) -> String {
        format!(
            "Disjoint: only joining of {} and {} is product measure",
            self.system_a, self.system_b
        )
    }
}
/// Haar measure on a locally compact group.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct HaarMeasure {
    pub group_name: String,
    pub is_unimodular: bool,
    pub modular_function: String,
}
impl HaarMeasure {
    #[allow(dead_code)]
    pub fn compact_group(group: &str) -> Self {
        Self {
            group_name: group.to_string(),
            is_unimodular: true,
            modular_function: "Delta = 1 (compact group)".to_string(),
        }
    }
    #[allow(dead_code)]
    pub fn abelian_group(group: &str) -> Self {
        Self {
            group_name: group.to_string(),
            is_unimodular: true,
            modular_function: "Delta = 1 (abelian group)".to_string(),
        }
    }
    #[allow(dead_code)]
    pub fn is_finite(&self) -> bool {
        self.group_name.contains("compact")
            || self.group_name.starts_with("S")
            || self.group_name.starts_with("U")
            || self.group_name.starts_with("SO")
            || self.group_name.starts_with("SU")
    }
    #[allow(dead_code)]
    pub fn left_invariance(&self) -> String {
        format!(
            "mu(gA) = mu(A) for all g in {} and measurable A",
            self.group_name
        )
    }
}
/// Square transition / adjacency matrix for an SFT or Markov chain.
#[derive(Debug, Clone)]
pub struct TransitionMatrix {
    /// Dimension n of the n×n matrix.
    pub size: usize,
    /// Row-major entries: entries[i][j] = probability / count of i → j transition.
    pub entries: Vec<Vec<f64>>,
}
impl TransitionMatrix {
    /// Create a new n×n zero transition matrix.
    pub fn new(size: usize) -> Self {
        Self {
            size,
            entries: vec![vec![0.0; size]; size],
        }
    }
    /// Set the (i, j) entry of the matrix.
    pub fn set_entry(&mut self, i: usize, j: usize, value: f64) {
        if i < self.size && j < self.size {
            self.entries[i][j] = value;
        }
    }
    /// Approximate spectral radius via the power method (100 iterations).
    /// For a non-negative matrix this equals the Perron-Frobenius eigenvalue.
    #[allow(clippy::needless_range_loop)]
    pub fn spectral_radius(&self) -> f64 {
        if self.size == 0 {
            return 0.0;
        }
        let mut v = vec![1.0_f64; self.size];
        let mut radius = 0.0_f64;
        for _ in 0..200 {
            let mut w = vec![0.0_f64; self.size];
            for i in 0..self.size {
                for j in 0..self.size {
                    w[i] += self.entries[i][j] * v[j];
                }
            }
            let norm: f64 = w.iter().map(|x| x * x).sum::<f64>().sqrt();
            if norm < 1e-300 {
                break;
            }
            radius = norm / (v.iter().map(|x| x * x).sum::<f64>().sqrt());
            v = w.iter().map(|x| x / norm).collect();
        }
        radius
    }
    /// Topological entropy of the SFT with this transition matrix: log(spectral_radius(A)).
    pub fn entropy(&self) -> f64 {
        let r = self.spectral_radius();
        if r > 0.0 {
            r.ln()
        } else {
            0.0
        }
    }
}
/// Kolmogorov-Sinai (metric) entropy of a named system.
#[derive(Debug, Clone)]
pub struct KolmogorovSinaiEntropy {
    /// Name of the measure-preserving system.
    pub system: String,
    /// Computed entropy value h_μ(T) ≥ 0.
    pub value: f64,
}
impl KolmogorovSinaiEntropy {
    /// Create a KS entropy record.
    pub fn new(system: impl Into<String>, value: f64) -> Self {
        Self {
            system: system.into(),
            value,
        }
    }
    /// Compute the entropy value (returns the stored value).
    pub fn compute_entropy(&self) -> f64 {
        self.value
    }
    /// Returns a statement that KS entropy is an isomorphism invariant.
    pub fn entropy_is_invariant(&self) -> String {
        format!(
            "KS Entropy Invariance: h_μ(T) = {} is a complete invariant for Bernoulli shifts \
             (Ornstein's theorem). Two Bernoulli shifts are isomorphic iff they have the \
             same entropy. For system '{}', KS entropy is preserved under measurable \
             isomorphisms of measure-preserving systems.",
            self.value, self.system
        )
    }
}
/// Checks Weyl equidistribution of polynomial sequences mod 1.
///
/// Weyl's theorem: the sequence {nα}, {n²α}, …, or more generally
/// {p(n)} for a non-constant polynomial p with at least one irrational
/// non-constant coefficient is equidistributed mod 1.
#[derive(Debug, Clone)]
pub struct WeylEquidistributionChecker {
    /// Coefficients of the polynomial p(n) = a_k n^k + … + a_1 n + a_0.
    pub coefficients: Vec<f64>,
    /// Number of terms used in the empirical check.
    pub num_terms: usize,
}
impl WeylEquidistributionChecker {
    /// Construct with polynomial coefficients (ascending: a_0, a_1, …, a_k).
    pub fn new(coefficients: Vec<f64>, num_terms: usize) -> Self {
        Self {
            coefficients,
            num_terms,
        }
    }
    /// Evaluate p(n) mod 1.
    pub fn eval_mod1(&self, n: f64) -> f64 {
        let val: f64 = self
            .coefficients
            .iter()
            .enumerate()
            .map(|(k, &a)| a * n.powi(k as i32))
            .sum();
        val.fract().abs()
    }
    /// Empirical check: fraction of {p(1), …, p(N)} mod 1 landing in [a, b].
    pub fn empirical_frequency(&self, a: f64, b: f64) -> f64 {
        let count = (1..=self.num_terms)
            .filter(|&n| {
                let v = self.eval_mod1(n as f64);
                v >= a && v < b
            })
            .count();
        count as f64 / self.num_terms as f64
    }
    /// Returns `true` if the empirical frequency approximates (b - a) within `tol`.
    pub fn is_equidistributed(&self, a: f64, b: f64, tol: f64) -> bool {
        (self.empirical_frequency(a, b) - (b - a)).abs() < tol
    }
    /// Weyl's theorem statement.
    pub fn weyl_theorem_statement() -> &'static str {
        "Weyl's Equidistribution Theorem: Let p(n) be a polynomial with at least one \
         irrational coefficient among the non-constant terms. Then the sequence \
         (p(n) mod 1)_{n=1}^∞ is equidistributed in [0,1), i.e., for every interval \
         [a,b) ⊆ [0,1), the proportion of n ≤ N with {p(n)} ∈ [a,b) converges to b-a. \
         Equivalently, for every non-zero integer h, (1/N) Σ_{n=1}^N e^{2πihp(n)} → 0."
    }
}
/// A single ergodic component in an ergodic decomposition.
#[derive(Debug, Clone)]
pub struct ErgodicComponent {
    /// Index identifying this component.
    pub index: usize,
    /// Weight (barycentric coefficient) of this component; must be non-negative.
    pub weight: f64,
    /// Name / description of the ergodic probability measure.
    pub measure: String,
}
/// A measure-preserving system described by name strings.
#[derive(Debug, Clone)]
pub struct MeasurePreservingSystemEx {
    /// Name of the measurable space X.
    pub space: String,
    /// Name of the T-invariant measure μ.
    pub measure: String,
    /// Description of the transformation T.
    pub transformation: String,
}
impl MeasurePreservingSystemEx {
    /// Create a new named measure-preserving system.
    pub fn new(
        space: impl Into<String>,
        measure: impl Into<String>,
        transformation: impl Into<String>,
    ) -> Self {
        Self {
            space: space.into(),
            measure: measure.into(),
            transformation: transformation.into(),
        }
    }
    /// Returns `true` when the system is ergodic.
    /// Ergodicity is encoded as: transformation name contains "ergodic" or "doubling"
    /// or "Baker" or "Cat" or "Shift" (canonical ergodic maps).
    pub fn is_ergodic(&self) -> bool {
        let t = self.transformation.to_lowercase();
        t.contains("ergodic")
            || t.contains("doubling")
            || t.contains("baker")
            || t.contains("cat")
            || t.contains("shift")
            || t.contains("bernoulli")
    }
    /// Returns `true` when the system is (strong) mixing.
    pub fn is_mixing(&self) -> bool {
        let t = self.transformation.to_lowercase();
        t.contains("mixing")
            || t.contains("doubling")
            || t.contains("baker")
            || t.contains("cat")
            || t.contains("shift")
            || t.contains("bernoulli")
    }
    /// Returns `true` when the system is weakly mixing.
    pub fn is_weakly_mixing(&self) -> bool {
        self.is_mixing() || {
            let t = self.transformation.to_lowercase();
            t.contains("weak") || t.contains("liouville")
        }
    }
}
/// Topological entropy of a continuous map.
#[derive(Debug, Clone)]
pub struct TopologicalEntropy {
    /// Name / description of the map.
    pub map: String,
    /// Topological entropy value h_top(T) ≥ 0.
    pub value: f64,
}
/// A joining of two measure-preserving systems.
///
/// A joining of (X, μ, T) and (Y, ν, S) is a T×S-invariant measure ρ on X×Y
/// with marginals μ and ν respectively.
#[derive(Debug, Clone)]
pub struct JoiningRecord {
    /// Name of the first system.
    pub system_x: String,
    /// Name of the second system.
    pub system_y: String,
    /// Type of joining: "product", "off-diagonal", "relative-independent", "minimal".
    pub joining_type: String,
    /// Whether this joining is the product joining (disjoint systems).
    pub is_product: bool,
}
impl JoiningRecord {
    /// Construct a joining record.
    pub fn new(
        system_x: impl Into<String>,
        system_y: impl Into<String>,
        joining_type: impl Into<String>,
        is_product: bool,
    ) -> Self {
        Self {
            system_x: system_x.into(),
            system_y: system_y.into(),
            joining_type: joining_type.into(),
            is_product,
        }
    }
    /// Two systems are disjoint if their only joining is the product measure.
    pub fn systems_are_disjoint(&self) -> bool {
        self.is_product
    }
    /// Description of this joining.
    pub fn describe(&self) -> String {
        format!(
            "Joining of '{}' and '{}': type='{}', is_product={}",
            self.system_x, self.system_y, self.joining_type, self.is_product
        )
    }
    /// Minimal self-joinings theorem statement.
    pub fn minimal_self_joining_statement() -> &'static str {
        "Minimal Self-Joining Theorem (del Junco-Rahe-Swanson): A measure-preserving \
         system (X, μ, T) has minimal self-joinings (MSJ) if the only self-joinings of \
         order k are the off-diagonal measures. MSJ systems are prime, have trivial \
         commutant (only powers of T commute with T), and are disjoint from their \
         non-trivial factors. Almost every Poisson suspension of a rank-one system has MSJ."
    }
}
/// Wraps the formal statement of the Birkhoff ergodic theorem.
pub struct BirkhoffTheoremStatement {
    /// Human-readable statement string.
    pub statement: String,
}
/// Classification of ergodicity strength.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErgodicityType {
    /// Merely ergodic (no non-trivial invariant sets).
    Ergodic,
    /// Weak mixing (ergodic + no non-trivial rigid factor).
    WeakMixing,
    /// Strong mixing (correlations decay to zero).
    StrongMixing,
    /// Bernoulli — isomorphic to a Bernoulli shift (strongest property).
    Bernoulli,
}
/// Running computation of a Birkhoff time average  (1/n) Σ_{k=0}^{n-1} f(T^k x).
#[derive(Debug, Clone)]
pub struct BirkhoffAverage {
    /// The underlying transformation (for metadata only).
    pub transformation: ErgodicTransformation,
    /// Number of observations added so far.
    pub n_iterations: u64,
    /// Cumulative sum of observations.
    pub sum: f64,
    /// Whether the running average has converged (within some tolerance).
    pub converged: bool,
}
impl BirkhoffAverage {
    /// Create a fresh Birkhoff average accumulator.
    pub fn new(transformation: ErgodicTransformation) -> Self {
        Self {
            transformation,
            n_iterations: 0,
            sum: 0.0,
            converged: false,
        }
    }
    /// Record one new observation f(T^k x).
    pub fn add_observation(&mut self, x: f64) {
        self.sum += x;
        self.n_iterations += 1;
        self.converged = false;
    }
    /// Return the current Cesàro average (1/n) Σ f(T^k x).
    pub fn current_average(&self) -> f64 {
        if self.n_iterations == 0 {
            0.0
        } else {
            self.sum / self.n_iterations as f64
        }
    }
    /// Check whether the running average has stabilised within `tol` over the last
    /// observation (heuristic: the incremental update is small).
    pub fn has_converged(&self, tol: f64) -> bool {
        if self.n_iterations < 2 {
            return false;
        }
        let delta = (self.sum - (self.n_iterations - 1) as f64 * self.current_average()).abs()
            / self.n_iterations as f64;
        delta < tol
    }
}
/// A mixing coefficient α(t), β(t), … together with its decay rate.
#[derive(Debug, Clone)]
pub struct MixingCoefficient {
    /// Which coefficient type this records.
    pub coefficient_type: MixingType,
    /// Current value of the coefficient.
    pub value: f64,
    /// Exponential decay rate r > 0 such that the coefficient ≤ C·e^{-r·t}.
    pub decay_rate: f64,
}
impl MixingCoefficient {
    /// Create a new mixing coefficient record.
    pub fn new(coefficient_type: MixingType, value: f64, decay_rate: f64) -> Self {
        Self {
            coefficient_type,
            value,
            decay_rate,
        }
    }
    /// Returns `true` if the coefficient decays to zero (positive decay rate and small value).
    pub fn decays_to_zero(&self, t: f64) -> bool {
        self.value * (-self.decay_rate * t).exp() < 1e-12
    }
    /// Approximate mixing time: smallest t such that the coefficient < ε.
    /// Derived from C·e^{-r·t} < ε  ⟹  t > ln(C/ε)/r.
    pub fn mixing_time(&self, epsilon: f64) -> f64 {
        if self.decay_rate <= 0.0 || epsilon <= 0.0 {
            return f64::INFINITY;
        }
        (self.value / epsilon).ln() / self.decay_rate
    }
}
/// A join decomposition of a measure-preserving system into factor systems.
#[derive(Debug, Clone)]
pub struct JoinDecomposition {
    /// Names of the factor systems in the join.
    pub factors: Vec<String>,
}
impl JoinDecomposition {
    /// Create a join decomposition.
    pub fn new(factors: Vec<String>) -> Self {
        Self { factors }
    }
    /// Returns `true` if the system is prime (cannot be decomposed into non-trivial joins).
    pub fn is_prime(&self) -> bool {
        self.factors.len() <= 1
    }
    /// Number of factors.
    pub fn num_factors(&self) -> usize {
        self.factors.len()
    }
    /// Description of the join.
    pub fn description(&self) -> String {
        if self.factors.is_empty() {
            "Empty join (trivial system)".to_string()
        } else {
            format!("Join of factors: {}", self.factors.join(" ⋈ "))
        }
    }
}
/// Rohlin tower decomposition.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RohlinTower {
    pub system_name: String,
    pub height: usize,
    pub error: f64,
}
impl RohlinTower {
    #[allow(dead_code)]
    pub fn new(system: &str, height: usize, error: f64) -> Self {
        Self {
            system_name: system.to_string(),
            height,
            error,
        }
    }
    #[allow(dead_code)]
    pub fn rohlin_lemma(&self) -> String {
        format!(
            "Rohlin lemma: in aperiodic system {}, for any n>0, epsilon>0, \
             exists tower of height n with measure >= 1 - epsilon",
            self.system_name
        )
    }
}
/// L² convergence data for the von Neumann mean ergodic theorem.
#[derive(Debug, Clone)]
pub struct L2Convergence {
    /// Initial vector (or L² norm of f - P_inv f at n=1).
    pub initial: f64,
    /// L² limit (projection onto the space of T-invariant functions).
    pub limit: f64,
    /// Convergence rate coefficient r ∈ (0,1) such that ‖A_n f - f*‖ ≤ C·r^n.
    pub rate: f64,
}
impl L2Convergence {
    /// Create a new L² convergence record.
    pub fn new(initial: f64, limit: f64, rate: f64) -> Self {
        Self {
            initial,
            limit,
            rate,
        }
    }
    /// Returns `true` if the rate indicates convergence (rate < 1).
    pub fn converges(&self) -> bool {
        self.rate < 1.0 && self.rate >= 0.0
    }
    /// Upper bound on the L²-error ‖A_n f - f*‖₂ after `n` steps.
    pub fn error_at(&self, n: u64) -> f64 {
        self.initial * self.rate.powi(n as i32)
    }
}
/// Data associated with Poincaré recurrence for a measurable set A.
#[derive(Debug, Clone)]
pub struct PoincareRecurrence {
    /// Name / description of the measurable set A.
    pub set: String,
    /// μ(A) — the measure of the set.
    pub measure: f64,
    /// Optional empirical / computed mean return time (in iterates).
    pub return_time: Option<u64>,
}
impl PoincareRecurrence {
    /// Create a new Poincaré recurrence record.
    pub fn new(set: impl Into<String>, measure: f64, return_time: Option<u64>) -> Self {
        Self {
            set: set.into(),
            measure,
            return_time,
        }
    }
    /// Recurrence is guaranteed when μ(A) > 0 (and the total measure is finite).
    pub fn recurrence_guaranteed(&self) -> bool {
        self.measure > 0.0
    }
    /// Average return time = 1 / μ(A) by Kac's lemma (for ergodic probability systems).
    pub fn average_return_time(&self) -> Option<f64> {
        if self.measure > 0.0 {
            Some(1.0 / self.measure)
        } else {
            None
        }
    }
}
/// Formal wrapper for the Birkhoff ergodic theorem statement.
#[derive(Debug, Clone)]
pub struct BirkhoffErgodicThm {
    /// Name of the system this statement applies to.
    pub system: String,
}
impl BirkhoffErgodicThm {
    /// Create a Birkhoff ergodic theorem record.
    pub fn new(system: impl Into<String>) -> Self {
        Self {
            system: system.into(),
        }
    }
    /// Return the convergence statement for this system.
    pub fn convergence_statement(&self) -> String {
        format!(
            "Birkhoff Ergodic Theorem for system '{}': \
             For every f ∈ L¹(μ), for μ-a.e. x, \
             (1/n) Σ_{{k=0}}^{{n-1}} f(T^k x) → f*(x) as n → ∞, \
             where f* is a T-invariant function with ∫ f* dμ = ∫ f dμ. \
             If the system is ergodic, f*(x) = ∫ f dμ for μ-a.e. x.",
            self.system
        )
    }
    /// Return the pointwise convergence claim.
    pub fn pointwise_convergence(&self) -> String {
        format!(
            "Pointwise convergence (system '{}'): \
             The time average A_n f(x) = (1/n) Σ_{{k=0}}^{{n-1}} f(T^k x) \
             converges pointwise μ-a.e., not just in L¹ norm. \
             The limit function f* ∈ L¹(μ) is T-invariant: f* ∘ T = f* μ-a.e.",
            self.system
        )
    }
}
/// The ergodic decomposition of a T-invariant measure μ:
///   μ = ∫ μ_x dμ(x), where each μ_x is T-ergodic.
#[derive(Debug, Clone)]
pub struct ErgodicDecomposition {
    /// The ergodic components with their weights.
    pub components: Vec<ErgodicComponent>,
}
impl ErgodicDecomposition {
    /// Create an empty decomposition.
    pub fn new() -> Self {
        Self {
            components: Vec::new(),
        }
    }
    /// Add an ergodic component with the given weight and measure name.
    pub fn add_component(&mut self, weight: f64, measure: impl Into<String>) {
        let index = self.components.len();
        self.components.push(ErgodicComponent {
            index,
            weight,
            measure: measure.into(),
        });
    }
    /// Normalise all weights so that they sum to 1.
    pub fn normalize(&mut self) {
        let total = self.total_weight();
        if total > 0.0 {
            for c in &mut self.components {
                c.weight /= total;
            }
        }
    }
    /// Return the number of ergodic components.
    pub fn num_components(&self) -> usize {
        self.components.len()
    }
    /// Return the sum of all component weights.
    pub fn total_weight(&self) -> f64 {
        self.components.iter().map(|c| c.weight).sum()
    }
}
/// Approximate Kolmogorov-Sinai entropy via iterated partition refinement.
///
/// For a Bernoulli system with a given probability vector `probs`, the KS
/// entropy equals the Shannon entropy H(probs) = -Σ p_i log p_i.  This struct
/// provides the computation together with the generator theorem statement.
#[derive(Debug, Clone)]
pub struct KolmogorovSinaiEntropyComputer {
    /// Name of the measure-preserving system under study.
    pub system: String,
    /// Probability weights of the generating partition atoms; must sum to 1.
    pub partition_weights: Vec<f64>,
}
impl KolmogorovSinaiEntropyComputer {
    /// Construct a new computer for the given system and partition.
    pub fn new(system: impl Into<String>, partition_weights: Vec<f64>) -> Self {
        Self {
            system: system.into(),
            partition_weights,
        }
    }
    /// Shannon entropy H(p) = -Σ p_i log(p_i) of the partition; equals h_μ(T)
    /// by the generator theorem when the partition is a generator.
    pub fn shannon_entropy(&self) -> f64 {
        self.partition_weights
            .iter()
            .filter(|&&p| p > 0.0)
            .map(|&p| -p * p.ln())
            .sum()
    }
    /// Returns the stored entropy (Shannon entropy of the partition).
    pub fn compute(&self) -> f64 {
        self.shannon_entropy()
    }
    /// Check whether this entropy value equals log(n) (uniform Bernoulli with n atoms).
    pub fn is_uniform_bernoulli(&self, tol: f64) -> bool {
        let n = self.partition_weights.len();
        if n == 0 {
            return false;
        }
        let expected = (n as f64).ln();
        (self.shannon_entropy() - expected).abs() < tol
    }
    /// The generator theorem: if P is a generating partition, h_μ(T) = h_μ(T, P).
    pub fn generator_theorem_statement(&self) -> String {
        format!(
            "Generator Theorem (Sinai): If P is a generating partition for system '{}', \
             then the KS entropy satisfies h_μ(T) = h_μ(T, P) = lim_{{n→∞}} (1/n) H(P^n), \
             where P^n = P ∨ T⁻¹P ∨ … ∨ T^{{-(n-1)}}P is the join of n iterates.",
            self.system
        )
    }
    /// The Pinsker factor description: the maximal zero-entropy factor of the system.
    pub fn pinsker_factor_statement(&self) -> String {
        format!(
            "Pinsker Factor of system '{}': The Pinsker factor Π(X, μ, T) is the maximal \
             factor with zero entropy. The system has positive entropy (h_μ(T) = {:.4}) iff \
             the Pinsker factor is trivial. Bernoulli systems have trivial Pinsker factor.",
            self.system,
            self.compute()
        )
    }
}
/// Lyapunov exponents of a linear cocycle.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LyapunovSpectrum {
    pub system_name: String,
    pub exponents: Vec<f64>,
}
impl LyapunovSpectrum {
    #[allow(dead_code)]
    pub fn new(system: &str, exponents: Vec<f64>) -> Self {
        Self {
            system_name: system.to_string(),
            exponents,
        }
    }
    #[allow(dead_code)]
    pub fn max_exponent(&self) -> f64 {
        self.exponents
            .iter()
            .cloned()
            .fold(f64::NEG_INFINITY, f64::max)
    }
    #[allow(dead_code)]
    pub fn is_hyperbolic(&self) -> bool {
        self.exponents.iter().all(|&e| e.abs() > 1e-10)
    }
    #[allow(dead_code)]
    pub fn sum(&self) -> f64 {
        self.exponents.iter().sum()
    }
    #[allow(dead_code)]
    pub fn oseledets_theorem(&self) -> String {
        format!(
            "Oseledets: measurable linear cocycle over {} has Lyapunov spectrum {:?}",
            self.system_name, self.exponents
        )
    }
}
/// Symbolic dynamical system.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SymbolicSystem {
    pub alphabet_size: usize,
    pub constraints: Vec<String>,
    pub entropy: f64,
}
impl SymbolicSystem {
    #[allow(dead_code)]
    pub fn full_shift(n: usize) -> Self {
        Self {
            alphabet_size: n,
            constraints: Vec::new(),
            entropy: (n as f64).ln(),
        }
    }
    #[allow(dead_code)]
    pub fn sofic(entropy: f64, alphabet: usize) -> Self {
        Self {
            alphabet_size: alphabet,
            constraints: vec!["sofic: shifts with a finite edge-labeled graph".to_string()],
            entropy,
        }
    }
    #[allow(dead_code)]
    pub fn is_subshift_of_finite_type(&self) -> bool {
        self.constraints.is_empty()
    }
    #[allow(dead_code)]
    pub fn is_sofic(&self) -> bool {
        !self.constraints.is_empty() || self.is_subshift_of_finite_type()
    }
}
/// Multiple Birkhoff recurrence theorem (Furstenberg-Sárközy-type).
#[derive(Debug, Clone)]
pub struct MultipleBirkhoffRecurrence {
    /// Number of commuting transformations T₁, …, T_k.
    pub num_commuting: usize,
}
impl MultipleBirkhoffRecurrence {
    /// Create a multiple recurrence record.
    pub fn new(num_commuting: usize) -> Self {
        Self { num_commuting }
    }
    /// Furstenberg's multiple recurrence theorem statement.
    pub fn furstenberg_theorem(&self) -> String {
        format!(
            "Furstenberg Multiple Recurrence Theorem ({k} commuting transformations): \
             Let (X, μ) be a probability space and T₁, …, T_{k} commuting \
             measure-preserving transformations. For every A with μ(A) > 0, \
             lim inf_{{N→∞}} (1/N) Σ_{{n=0}}^{{N-1}} μ(A ∩ T₁^{{-n}}A ∩ … ∩ T_{k}^{{-n}}A) > 0. \
             This implies Szemerédi's theorem: every set of positive density \
             contains arithmetic progressions of length {k}+1.",
            k = self.num_commuting
        )
    }
}
/// Classification of mixing / dependence coefficients.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MixingType {
    /// α-mixing (strong mixing of Rosenblatt): α(n) → 0.
    Alpha,
    /// β-mixing (absolute regularity): β(n) → 0.
    Beta,
    /// φ-mixing (uniform mixing): φ(n) → 0.
    Phi,
    /// ρ-mixing (maximal correlation): ρ(n) → 0.
    Rho,
    /// ψ-mixing: ψ(n) → 0 (strongest among the five).
    Psi,
}
/// Measurable partition and sigma-algebra.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct MeasurablePartition {
    pub space_name: String,
    pub partition_name: String,
    pub is_generating: bool,
}
impl MeasurablePartition {
    #[allow(dead_code)]
    pub fn new(space: &str, name: &str, gen: bool) -> Self {
        Self {
            space_name: space.to_string(),
            partition_name: name.to_string(),
            is_generating: gen,
        }
    }
    #[allow(dead_code)]
    pub fn conditional_entropy_description(&self) -> String {
        format!(
            "H(P|Q): conditional entropy of {} given another partition",
            self.partition_name
        )
    }
}
