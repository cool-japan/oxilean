//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

/// Mellin transform Tauberian theorem: asymptotics from poles of the Mellin transform.
pub struct MellintransformThm {
    /// Whether the Mellin transform has been analytically continued.
    pub analytically_continued: bool,
}
impl MellintransformThm {
    /// Create a new MellintransformThm.
    pub fn new(analytically_continued: bool) -> Self {
        Self {
            analytically_continued,
        }
    }
    /// Asymptotics from poles: residues of the Mellin transform determine asymptotics.
    pub fn asymptotics_from_poles(&self) -> String {
        "Mellin-Tauberian: The asymptotics of f(x) as x → ∞ are governed by the \
         rightmost pole of the Mellin transform M[f](s) = ∫₀^∞ f(t)t^{s-1}dt. \
         A simple pole at s = s₀ contributes ~ C·x^{s₀}."
            .to_string()
    }
    /// Ikehara-Wiener theorem: PNT from the simple pole of the Dirichlet series.
    pub fn ikehara_wiener(&self) -> String {
        let analytic = if self.analytically_continued {
            "analytically continued"
        } else {
            "formal"
        };
        format!(
            "Ikehara-Wiener ({} form): If the Dirichlet series D(s) = ∑ a_n·n^{{-s}} \
             has a simple pole at s=1 with residue 1 and D(s) - 1/(s-1) extends \
             continuously to Re(s)=1, then ∑_{{n≤x}} a_n ~ x. This implies the PNT.",
            analytic
        )
    }
}
/// Computes Cesàro (C,α) sums of a sequence for integer orders α ≥ 1.
///
/// The (C,k) mean of (aₙ) is σₙ^(k) = (1/(C(n+k,k))) ∑_{j=0}^n C(n-j+k-1, k-1) aⱼ.
/// This struct iteratively applies the (C,1) averaging to compute (C,k) means.
#[derive(Debug, Clone)]
pub struct CesaroSumComputer {
    /// The terms of the series a₀, a₁, ..., aₙ.
    pub terms: Vec<f64>,
}
impl CesaroSumComputer {
    /// Construct from a slice of terms.
    pub fn new(terms: Vec<f64>) -> Self {
        CesaroSumComputer { terms }
    }
    /// Compute partial sums S₀, S₁, ..., Sₙ.
    pub fn partial_sums(&self) -> Vec<f64> {
        let mut sums = Vec::with_capacity(self.terms.len());
        let mut acc = 0.0;
        for &a in &self.terms {
            acc += a;
            sums.push(acc);
        }
        sums
    }
    /// Compute first-order Cesàro means σₙ = (S₀ + ... + Sₙ) / (n+1).
    pub fn cesaro_c1(&self) -> Vec<f64> {
        let sums = self.partial_sums();
        let mut means = Vec::with_capacity(sums.len());
        let mut acc = 0.0;
        for (n, &s) in sums.iter().enumerate() {
            acc += s;
            means.push(acc / (n + 1) as f64);
        }
        means
    }
    /// Compute k-th order Cesàro means by iterating first-order averaging k times.
    pub fn cesaro_ck(&self, k: usize) -> Vec<f64> {
        if k == 0 {
            return self.partial_sums();
        }
        let mut current = self.partial_sums();
        for _ in 0..k {
            let mut next = Vec::with_capacity(current.len());
            let mut acc = 0.0;
            for (n, &s) in current.iter().enumerate() {
                acc += s;
                next.push(acc / (n + 1) as f64);
            }
            current = next;
        }
        current
    }
    /// Returns the last Cesàro (C,k) mean as an estimate of the sum.
    pub fn cesaro_sum(&self, order: usize) -> Option<f64> {
        let means = self.cesaro_ck(order);
        means.last().copied()
    }
    /// Check if the sequence appears Cesàro summable of order k (last few means are close).
    pub fn appears_cesaro_summable(&self, order: usize, tolerance: f64) -> bool {
        let means = self.cesaro_ck(order);
        let n = means.len();
        if n < 4 {
            return false;
        }
        let last = means[n - 1];
        means[n - 4..].iter().all(|&m| (m - last).abs() < tolerance)
    }
}
/// Container for comparing different summability methods on a sequence.
///
/// Checks whether ordinary convergence, Cesàro summability, and Abel summability
/// agree (Abel ← Cesàro ← ordinary convergence, but not conversely).
#[derive(Debug, Clone)]
pub struct SummabilityComparison {
    /// The sequence of partial sums S_n = ∑_{k=0}^n aₖ.
    pub partial_sums: Vec<f64>,
}
impl SummabilityComparison {
    /// Construct from a sequence of terms.
    pub fn from_terms(terms: &[f64]) -> Self {
        let mut partial_sums = Vec::with_capacity(terms.len());
        let mut acc = 0.0;
        for &a in terms {
            acc += a;
            partial_sums.push(acc);
        }
        SummabilityComparison { partial_sums }
    }
    /// Compute Cesàro means: σ_n = (S₀ + S₁ + ... + Sₙ) / (n+1).
    pub fn cesaro_means(&self) -> Vec<f64> {
        let mut means = Vec::with_capacity(self.partial_sums.len());
        let mut sum = 0.0;
        for (n, &s) in self.partial_sums.iter().enumerate() {
            sum += s;
            means.push(sum / (n + 1) as f64);
        }
        means
    }
    /// Check if the ordinary series appears to converge (last few partial sums close).
    pub fn appears_convergent(&self, tolerance: f64) -> bool {
        let n = self.partial_sums.len();
        if n < 4 {
            return false;
        }
        let last = self.partial_sums[n - 1];
        self.partial_sums[n - 4..]
            .iter()
            .all(|&s| (s - last).abs() < tolerance)
    }
    /// Check if the Cesàro means appear to converge.
    pub fn cesaro_appears_convergent(&self, tolerance: f64) -> bool {
        let means = self.cesaro_means();
        let n = means.len();
        if n < 4 {
            return false;
        }
        let last = means[n - 1];
        means[n - 4..].iter().all(|&m| (m - last).abs() < tolerance)
    }
}
/// Tauberian conditions that imply convergence from summability.
#[derive(Debug, Clone, PartialEq)]
pub enum TauberianCondition {
    /// Hardy's one-sided Tauberian condition: n·a_n = O(1).
    Hardy,
    /// Karamata's condition for regularly varying functions.
    Karamata,
    /// Littlewood's condition: n·a_n = o(1) (weaker).
    Littlewood,
    /// The (C,1/2) condition (half-order Cesàro).
    OneHalf,
}
impl TauberianCondition {
    /// Returns a description of the Tauberian condition.
    pub fn description(&self) -> &'static str {
        match self {
            TauberianCondition::Hardy => {
                "Hardy's condition: n·a_n = O(1). The strongest classical Tauberian condition."
            }
            TauberianCondition::Karamata => {
                "Karamata's condition: related to regular variation of the generating function."
            }
            TauberianCondition::Littlewood => {
                "Littlewood's condition: n·a_n = o(1). Strengthens Abel's theorem to full convergence."
            }
            TauberianCondition::OneHalf => {
                "The (C,1/2)-Tauberian condition: Cesàro summability of order 1/2."
            }
        }
    }
    /// Returns whether this condition, combined with Abel summability, implies convergence.
    pub fn implies_convergence(&self) -> bool {
        match self {
            TauberianCondition::Hardy => true,
            TauberianCondition::Karamata => true,
            TauberianCondition::Littlewood => true,
            TauberianCondition::OneHalf => false,
        }
    }
}
/// Abel summability: A(x) = Σ a_n x^n as x → 1^-.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct AbelSum {
    pub coefficients: Vec<f64>,
}
#[allow(dead_code)]
impl AbelSum {
    pub fn new() -> Self {
        AbelSum {
            coefficients: Vec::new(),
        }
    }
    pub fn add_coeff(&mut self, c: f64) {
        self.coefficients.push(c);
    }
    /// Evaluate power series at x (|x| < 1).
    pub fn evaluate_at(&self, x: f64) -> f64 {
        assert!(x.abs() < 1.0, "x must be in open unit disk");
        let mut sum = 0.0;
        let mut xn = 1.0;
        for &c in &self.coefficients {
            sum += c * xn;
            xn *= x;
        }
        sum
    }
    /// Approximated Abel sum: evaluate near x=1.
    pub fn abel_limit_approx(&self, steps: usize) -> Vec<(f64, f64)> {
        (0..steps)
            .map(|k| {
                let x = 1.0 - (k + 1) as f64 / (steps + 1) as f64;
                (x, self.evaluate_at(x))
            })
            .collect()
    }
}
/// Hardy-Littlewood Tauberian theorem (abstract representation).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct TauberianTheorem {
    pub name: String,
    pub summability_from: SummabilityMethod,
    pub summability_to: SummabilityMethod,
    pub condition: TauberCond,
}
#[allow(dead_code)]
impl TauberianTheorem {
    pub fn new(
        name: &str,
        from: SummabilityMethod,
        to: SummabilityMethod,
        cond: TauberCond,
    ) -> Self {
        TauberianTheorem {
            name: name.to_string(),
            summability_from: from,
            summability_to: to,
            condition: cond,
        }
    }
    pub fn hardylittlewood() -> Self {
        TauberianTheorem::new(
            "Hardy-Littlewood",
            SummabilityMethod::AbelSumm,
            SummabilityMethod::Ordinary,
            TauberCond::littlewood_condition(),
        )
    }
    pub fn wiener_ikehara() -> Self {
        TauberianTheorem::new(
            "Wiener-Ikehara",
            SummabilityMethod::Borel,
            SummabilityMethod::Ordinary,
            TauberCond::slow_oscillation(),
        )
    }
    pub fn is_valid_direction(&self) -> bool {
        self.summability_from >= self.summability_to
    }
}
/// Checks various Tauberian conditions on a sequence and computes associated bounds.
///
/// Given a sequence (aₙ) and its partial sums (Sₙ), this struct verifies:
/// - the one-sided condition n·aₙ ≥ -M,
/// - the slow oscillation condition,
/// - the O(1/n) condition on term growth,
/// - and estimates the Tauberian error remainder.
#[derive(Debug, Clone)]
pub struct TauberianBoundChecker {
    /// The sequence terms a₀, a₁, ..., aₙ.
    pub terms: Vec<f64>,
    /// The claimed summability limit (Abel or Cesàro limit).
    pub claimed_limit: f64,
}
impl TauberianBoundChecker {
    /// Construct with terms and a claimed limit.
    pub fn new(terms: Vec<f64>, claimed_limit: f64) -> Self {
        TauberianBoundChecker {
            terms,
            claimed_limit,
        }
    }
    /// Check the one-sided Tauberian condition n·aₙ ≥ -M for all n.
    /// Returns `Some(M)` if satisfied, `None` if M is unbounded.
    pub fn one_sided_condition(&self) -> Option<f64> {
        let m = self
            .terms
            .iter()
            .enumerate()
            .map(|(n, &a)| -(n as f64) * a)
            .fold(f64::NEG_INFINITY, f64::max);
        if m.is_finite() {
            Some(m.max(0.0))
        } else {
            None
        }
    }
    /// Check the Littlewood condition: n·aₙ = o(1), i.e., n·aₙ → 0.
    /// Tests by checking whether |n·aₙ| < tolerance for all n beyond a threshold.
    pub fn littlewood_condition(&self, threshold: usize, tolerance: f64) -> bool {
        self.terms
            .iter()
            .enumerate()
            .skip(threshold)
            .all(|(n, &a)| (n as f64 * a).abs() < tolerance)
    }
    /// Check the boundedness condition: |Sₙ| ≤ M for all n.
    /// Returns the bound M if satisfied.
    pub fn boundedness_condition(&self) -> f64 {
        let mut acc = 0.0;
        self.terms
            .iter()
            .map(|&a| {
                acc += a;
                acc.abs()
            })
            .fold(0.0_f64, f64::max)
    }
    /// Estimate the Tauberian remainder: max_{n} |Sₙ - claimed_limit|.
    ///
    /// For a Tauberian theorem, if conditions hold then |Sₙ - L| → 0.
    /// This computes the sup norm of the error over the finite prefix.
    pub fn tauberian_remainder(&self) -> f64 {
        let mut acc = 0.0;
        self.terms
            .iter()
            .map(|&a| {
                acc += a;
                (acc - self.claimed_limit).abs()
            })
            .fold(0.0_f64, f64::max)
    }
    /// Check slow oscillation: for large n and n/m ≤ λ, |Sₙ - Sₘ| < ε.
    ///
    /// Tests this for a specific λ and ε starting from a given index.
    pub fn slow_oscillation_check(&self, lambda: f64, epsilon: f64, start: usize) -> bool {
        let partial_sums: Vec<f64> = {
            let mut acc = 0.0;
            self.terms
                .iter()
                .map(|&a| {
                    acc += a;
                    acc
                })
                .collect()
        };
        let n = partial_sums.len();
        for i in start..n {
            for j in i..n {
                if j as f64 <= i as f64 * lambda {
                    if (partial_sums[j] - partial_sums[i]).abs() >= epsilon {
                        return false;
                    }
                }
            }
        }
        true
    }
    /// Estimate the rate of convergence of the Cesàro means to the limit.
    ///
    /// Returns the index at which the Cesàro means first enter the ε-ball around L.
    pub fn cesaro_convergence_index(&self, epsilon: f64) -> Option<usize> {
        let computer = CesaroSumComputer::new(self.terms.clone());
        let means = computer.cesaro_c1();
        means.iter().enumerate().find_map(|(n, &m)| {
            if (m - self.claimed_limit).abs() < epsilon {
                Some(n)
            } else {
                None
            }
        })
    }
}
/// Abel's theorem: if a power series converges to S at the boundary, it is Abel-summable to S.
pub struct AbelThm {
    /// The series (e.g., "sum a_n x^n").
    pub series: String,
}
impl AbelThm {
    /// Create a new AbelThm.
    pub fn new(series: impl Into<String>) -> Self {
        Self {
            series: series.into(),
        }
    }
    /// Abel summability implies Cesàro summability (under mild conditions).
    pub fn abel_summability_implies_cesaro(&self) -> String {
        format!(
            "If the series '{}' is Abel-summable to L, then it is also Cesàro-summable to L \
             (by Frobenius's theorem, 1880).",
            self.series
        )
    }
    /// Power series boundary behavior: Abel's theorem on radial limits.
    pub fn power_series_boundary_behavior(&self) -> String {
        format!(
            "For the power series '{}', if the partial sums converge to S as x→1⁻ \
             along the real axis, then the series is Abel-summable to S.",
            self.series
        )
    }
}
/// A regularly varying function with index ρ (Karamata's class).
pub struct RegularVariation {
    /// The variation index ρ (0 means slowly varying).
    pub index: f64,
}
impl RegularVariation {
    /// Create a new RegularVariation with given index.
    pub fn new(index: f64) -> Self {
        Self { index }
    }
    /// Returns true if this is slowly varying (index = 0).
    pub fn is_slowly_varying(&self) -> bool {
        self.index.abs() < 1e-12
    }
    /// Returns whether Karamata's Tauberian theorem applies.
    pub fn karamata_thm_applies(&self) -> bool {
        self.index > -1.0
    }
}
/// Summability method comparison (hierarchy).
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum SummabilityMethod {
    Ordinary,
    Cesaro1,
    Cesaro2,
    AbelSumm,
    Borel,
    Ramanujan,
}
#[allow(dead_code)]
impl SummabilityMethod {
    pub fn is_regular(&self) -> bool {
        matches!(
            self,
            SummabilityMethod::Cesaro1
                | SummabilityMethod::Cesaro2
                | SummabilityMethod::AbelSumm
                | SummabilityMethod::Borel
        )
    }
    pub fn stronger_than_ordinary(&self) -> bool {
        *self != SummabilityMethod::Ordinary
    }
    pub fn description(&self) -> &'static str {
        match self {
            SummabilityMethod::Ordinary => "Classical convergence",
            SummabilityMethod::Cesaro1 => "Cesaro C_1: arithmetic mean of partial sums",
            SummabilityMethod::Cesaro2 => "Cesaro C_2: iterated arithmetic mean",
            SummabilityMethod::AbelSumm => "Abel: lim_{x→1^-} Σ a_n x^n",
            SummabilityMethod::Borel => "Borel: via exponential generating function",
            SummabilityMethod::Ramanujan => "Ramanujan: regularization extending Abel",
        }
    }
}
/// Checks the slowly varying property for a tabulated function.
///
/// Verifies that L(tx)/L(x) → 1 as x → ∞ for several values of t by comparing
/// ratios at large x against thresholds.
#[derive(Debug, Clone)]
pub struct KaramataSlowVariation {
    /// Sample points x₀, x₁, ..., xₙ (increasing).
    pub x_values: Vec<f64>,
    /// Corresponding function values L(xᵢ).
    pub l_values: Vec<f64>,
}
impl KaramataSlowVariation {
    /// Construct from x-values and corresponding L-values.
    pub fn new(x_values: Vec<f64>, l_values: Vec<f64>) -> Self {
        KaramataSlowVariation { x_values, l_values }
    }
    /// Interpolate L at point x using linear interpolation between nearest samples.
    pub fn interpolate(&self, x: f64) -> Option<f64> {
        let n = self.x_values.len();
        if n == 0 {
            return None;
        }
        let pos = self.x_values.partition_point(|&xi| xi <= x);
        if pos == 0 {
            return Some(self.l_values[0]);
        }
        if pos >= n {
            return Some(self.l_values[n - 1]);
        }
        let x0 = self.x_values[pos - 1];
        let x1 = self.x_values[pos];
        let l0 = self.l_values[pos - 1];
        let l1 = self.l_values[pos];
        let t = (x - x0) / (x1 - x0);
        Some(l0 + t * (l1 - l0))
    }
    /// Check the slowly varying property L(tx)/L(x) ≈ 1 at a given large x and t.
    ///
    /// Returns the ratio L(tx)/L(x). For a slowly varying function this should be
    /// close to 1 for large x.
    pub fn slow_variation_ratio(&self, t: f64, x: f64) -> Option<f64> {
        let lx = self.interpolate(x)?;
        let ltx = self.interpolate(t * x)?;
        if lx == 0.0 {
            return None;
        }
        Some(ltx / lx)
    }
    /// Test slowly varying property for a list of (t, x) pairs.
    ///
    /// Returns `true` if all ratios are within `tolerance` of 1.
    pub fn is_slowly_varying(&self, test_pairs: &[(f64, f64)], tolerance: f64) -> bool {
        test_pairs.iter().all(|&(t, x)| {
            self.slow_variation_ratio(t, x)
                .map(|r| (r - 1.0).abs() < tolerance)
                .unwrap_or(false)
        })
    }
    /// Estimate the Karamata index by log-linear regression on the tail.
    ///
    /// For an RV_ρ function, log L(x) ≈ ρ log x + const in the tail.
    pub fn estimate_index(&self) -> f64 {
        let n = self.x_values.len();
        if n < 2 {
            return 0.0;
        }
        let start = n / 2;
        let log_x: Vec<f64> = self.x_values[start..].iter().map(|&x| x.ln()).collect();
        let log_l: Vec<f64> = self.l_values[start..]
            .iter()
            .map(|&l| l.abs().ln())
            .collect();
        let m = log_x.len() as f64;
        let sum_x: f64 = log_x.iter().sum();
        let sum_y: f64 = log_l.iter().sum();
        let sum_xx: f64 = log_x.iter().map(|&x| x * x).sum();
        let sum_xy: f64 = log_x.iter().zip(log_l.iter()).map(|(&x, &y)| x * y).sum();
        let denom = m * sum_xx - sum_x * sum_x;
        if denom.abs() < 1e-12 {
            return 0.0;
        }
        (m * sum_xy - sum_x * sum_y) / denom
    }
}
/// Cesàro summability of order α.
pub struct CesaroSummability {
    /// The order α of Cesàro summability (α = 1 is classical).
    pub order: f64,
}
impl CesaroSummability {
    /// Create a new CesaroSummability with given order.
    pub fn new(order: f64) -> Self {
        Self { order }
    }
    /// Returns true: Cesàro (C,α) summability includes Abel summability when α ≥ 0.
    pub fn includes_abel_summability(&self) -> bool {
        self.order >= 0.0
    }
    /// Hardy's theorem: (C,1) summability + Tauberian condition ⇒ convergence.
    pub fn hardy_theorem(&self) -> String {
        format!(
            "Hardy's theorem (order {}): If a series is (C,{:.1})-summable to L \
             and satisfies n·a_n = O(1), then it converges to L.",
            self.order, self.order
        )
    }
}
/// A simple prime sieve for PNT verification.
///
/// Provides π(x) (prime counting function) and ψ(x) (Chebyshev function)
/// for small ranges.
#[derive(Debug, Clone)]
pub struct PrimeSieve {
    /// is_prime[n] = true if n is prime.
    pub is_prime: Vec<bool>,
    /// Upper bound N of the sieve.
    pub limit: usize,
}
impl PrimeSieve {
    /// Build a prime sieve up to `limit` using the Sieve of Eratosthenes.
    pub fn new(limit: usize) -> Self {
        let mut is_prime = vec![true; limit + 1];
        is_prime[0] = false;
        if limit >= 1 {
            is_prime[1] = false;
        }
        let mut d = 2usize;
        while d * d <= limit {
            if is_prime[d] {
                let mut mult = d * d;
                while mult <= limit {
                    is_prime[mult] = false;
                    mult += d;
                }
            }
            d += 1;
        }
        PrimeSieve { is_prime, limit }
    }
    /// Compute π(x) = #{primes ≤ x}.
    pub fn prime_counting(&self, x: usize) -> usize {
        let x = x.min(self.limit);
        self.is_prime[..=x].iter().filter(|&&b| b).count()
    }
    /// Compute ψ(x) = ∑_{p^k ≤ x} log p (Chebyshev's ψ function).
    pub fn chebyshev_psi(&self, x: usize) -> f64 {
        let mut psi = 0.0;
        for p in 2..=self.limit {
            if self.is_prime[p] {
                let log_p = (p as f64).ln();
                let mut pk = p;
                while pk <= x {
                    psi += log_p;
                    pk = match pk.checked_mul(p) {
                        Some(v) => v,
                        None => break,
                    };
                }
            }
        }
        psi
    }
    /// Check PNT: π(x) / (x / ln x) should be close to 1 for large x.
    pub fn pnt_ratio(&self, x: usize) -> f64 {
        if x < 2 {
            return 0.0;
        }
        let pi_x = self.prime_counting(x) as f64;
        let approx = x as f64 / (x as f64).ln();
        pi_x / approx
    }
}
/// Hardy-Littlewood Tauberian theorem: one-sided conditions for series convergence.
pub struct HardyLittlewood {
    /// Description of the series under consideration.
    pub series_desc: String,
}
impl HardyLittlewood {
    /// Create a new HardyLittlewood instance.
    pub fn new(series_desc: impl Into<String>) -> Self {
        Self {
            series_desc: series_desc.into(),
        }
    }
    /// Statement of the Hardy-Littlewood Tauberian theorem.
    pub fn tauberian_theorem_statement(&self) -> String {
        format!(
            "Hardy-Littlewood (1914): If the series '{}' is Abel-summable to L, \
             and n·a_n ≥ -C for all n (one-sided condition), then the series converges to L.",
            self.series_desc
        )
    }
    /// Slowly varying condition required for the theorem.
    pub fn slowly_varying_condition(&self) -> String {
        "Slowly varying condition: L(tx)/L(x) → 1 as x → ∞ for all t > 0. \
         This is the minimal regularity required for Karamata-type Tauberian theorems."
            .to_string()
    }
}
/// Computes the Abel sum of a sequence by evaluating the generating function
/// at values close to x = 1 and checking for a limit.
///
/// For a sequence (aₙ), the Abel generating function is f(x) = ∑ aₙ xⁿ.
/// The Abel sum (if it exists) is lim_{x→1⁻} f(x).
#[derive(Debug, Clone)]
pub struct AbelSumComputer {
    /// The terms a₀, a₁, ..., aₙ of the sequence.
    pub terms: Vec<f64>,
}
impl AbelSumComputer {
    /// Construct from terms.
    pub fn new(terms: Vec<f64>) -> Self {
        AbelSumComputer { terms }
    }
    /// Evaluate f(x) = ∑ aₙ xⁿ at the given x ∈ [0, 1).
    pub fn generating_function(&self, x: f64) -> f64 {
        let mut sum = 0.0;
        let mut power = 1.0;
        for &a in &self.terms {
            sum += a * power;
            power *= x;
            if power < 1e-15 {
                break;
            }
        }
        sum
    }
    /// Estimate the Abel sum by evaluating f at several x values approaching 1.
    ///
    /// Returns a vector of (x, f(x)) pairs.
    pub fn approach_one(&self, steps: usize) -> Vec<(f64, f64)> {
        (0..steps)
            .map(|i| {
                let x = 1.0 - 10.0_f64.powi(-((i + 1) as i32));
                (x, self.generating_function(x))
            })
            .collect()
    }
    /// Estimate the Abel sum as lim_{x→1⁻} f(x) using Richardson extrapolation.
    ///
    /// Uses two points x₁ = 1 - h and x₂ = 1 - 2h and Richardson's formula.
    pub fn estimate_abel_sum(&self, h: f64) -> f64 {
        let x1 = 1.0 - h;
        let x2 = 1.0 - 2.0 * h;
        let f1 = self.generating_function(x1);
        let f2 = self.generating_function(x2);
        2.0 * f1 - f2
    }
    /// Check if the Abel sum is consistent with the ordinary sum (if series converges).
    pub fn ordinary_sum_matches_abel(&self, h: f64, tolerance: f64) -> bool {
        let ordinary: f64 = self.terms.iter().sum();
        let abel = self.estimate_abel_sum(h);
        (ordinary - abel).abs() < tolerance
    }
    /// Check the one-sided Tauberian condition: n · aₙ ≥ -M.
    /// Returns the best (smallest non-negative) M.
    pub fn one_sided_tauberian_constant(&self) -> f64 {
        self.terms
            .iter()
            .enumerate()
            .map(|(n, &a)| -(n as f64) * a)
            .fold(0.0_f64, f64::max)
            .max(0.0)
    }
}
/// A regularly varying function, specified by its index ρ and a slowly varying
/// part L (approximated by its value at a reference point).
#[derive(Debug, Clone)]
pub struct RegularlyVaryingFn {
    /// The variation index ρ.
    pub index: f64,
    /// Reference value L(x₀) of the slowly varying component.
    pub slowly_varying_ref: f64,
    /// Reference point x₀ for the slowly varying component.
    pub ref_point: f64,
}
impl RegularlyVaryingFn {
    /// Create a regularly varying function with given index and slowly varying part.
    pub fn new(index: f64, slowly_varying_ref: f64, ref_point: f64) -> Self {
        RegularlyVaryingFn {
            index,
            slowly_varying_ref,
            ref_point,
        }
    }
    /// Evaluate f(x) ≈ L(x₀) · (x/x₀)^ρ · L(x)/L(x₀).
    ///
    /// For this simplified model, we assume L is approximately constant,
    /// so f(x) ≈ L(x₀) · (x/x₀)^ρ.
    pub fn evaluate(&self, x: f64) -> f64 {
        if self.ref_point <= 0.0 || x <= 0.0 {
            return 0.0;
        }
        self.slowly_varying_ref * (x / self.ref_point).powf(self.index)
    }
    /// Check the defining property: f(tx)/f(x) → t^ρ for fixed t > 0.
    ///
    /// Tests at a large x to verify the ratio is close to t^ρ.
    pub fn check_rv_property(&self, t: f64, x: f64) -> f64 {
        let fx = self.evaluate(x);
        if fx == 0.0 {
            return 0.0;
        }
        self.evaluate(t * x) / fx
    }
    /// The Karamata index (variation index) of this function.
    pub fn karamata_index(&self) -> f64 {
        self.index
    }
    /// Whether this function is slowly varying (index = 0).
    pub fn is_slowly_varying(&self) -> bool {
        self.index.abs() < 1e-12
    }
}
/// A slowly varying function L (Karamata's definition).
pub struct SlowlyVaryingFn {
    /// Name or formula for the slowly varying function.
    pub fn_name: String,
}
impl SlowlyVaryingFn {
    /// Create a new SlowlyVaryingFn.
    pub fn new(fn_name: impl Into<String>) -> Self {
        Self {
            fn_name: fn_name.into(),
        }
    }
    /// Karamata representation theorem for slowly varying functions.
    pub fn karamata_representation(&self) -> String {
        format!(
            "Karamata representation: '{}' is slowly varying iff it can be written as \
             L(x) = c(x)·exp(∫₁ˣ ε(t)/t dt) where c(x) → c > 0 and ε(t) → 0.",
            self.fn_name
        )
    }
    /// Potter's bound: slowly varying functions satisfy |L(ty)/L(y)| ≤ C·max(t^δ, t^{-δ}).
    pub fn potter_bound(&self) -> String {
        format!(
            "Potter's bound for '{}': For every δ > 0 there exist C, x₀ such that \
             for x, tx ≥ x₀: L(tx)/L(x) ≤ C·max(t^δ, t^{{-δ}}).",
            self.fn_name
        )
    }
}
/// Karamata's Tauberian theorem for power series with regularly varying coefficients.
pub struct KaramataThm {
    /// The regular variation index ρ.
    pub rho: f64,
}
impl KaramataThm {
    /// Create a new KaramataThm with variation index ρ.
    pub fn new(rho: f64) -> Self {
        Self { rho }
    }
    /// Statement: regular variation implies asymptotic for partial sums.
    pub fn regular_variation_implies(&self) -> String {
        format!(
            "Karamata (1930): If a_n ~ n^{:.3} · L(n) / Γ(ρ+1) where L is slowly varying, \
             then the power series sum a_n·x^n ~ (1-x)^{{-{:.3}-1}} · L(1/(1-x)) as x → 1⁻.",
            self.rho, self.rho
        )
    }
    /// Abelian version (the easier direction): regular variation of f implies that of integral.
    pub fn abelian_theorem(&self) -> String {
        format!(
            "Abelian direction (Karamata): If f(x) ~ x^{:.3} · L(x) as x → ∞ \
             with L slowly varying, then ∫₀ˣ f(t)dt ~ x^{:.3}·L(x)/(ρ+1).",
            self.rho, self.rho
        )
    }
}
/// A sequence together with its Abel generating function value at x.
///
/// Used to test Tauberian conditions: if the limit as x → 1⁻ exists and
/// the sequence satisfies a Tauberian condition, the series converges.
#[derive(Debug, Clone)]
pub struct AbelSummableSequence {
    /// The sequence terms aₙ (finite prefix for computation).
    pub terms: Vec<f64>,
    /// The Abel sum (limit of the generating function at x = 1).
    pub abel_sum: f64,
}
impl AbelSummableSequence {
    /// Construct an `AbelSummableSequence` from terms and claimed Abel sum.
    pub fn new(terms: Vec<f64>, abel_sum: f64) -> Self {
        AbelSummableSequence { terms, abel_sum }
    }
    /// Evaluate the generating function ∑ aₙ xⁿ at a given x ∈ [0, 1).
    pub fn generating_function(&self, x: f64) -> f64 {
        let mut sum = 0.0;
        let mut power = 1.0;
        for &a in &self.terms {
            sum += a * power;
            power *= x;
        }
        sum
    }
    /// Check the one-sided Tauberian condition: n aₙ ≥ -M for some M.
    /// Returns Some(M) if the condition holds, None if M would be infinite.
    pub fn tauberian_constant(&self) -> Option<f64> {
        let m = self
            .terms
            .iter()
            .enumerate()
            .map(|(n, &a)| -(n as f64) * a)
            .fold(0.0_f64, f64::max);
        if m.is_finite() {
            Some(m.max(0.0))
        } else {
            None
        }
    }
    /// Check if the ordinary sum of (the finite prefix of) terms equals the Abel sum.
    pub fn ordinary_sum_matches_abel(&self, tolerance: f64) -> bool {
        let sum: f64 = self.terms.iter().sum();
        (sum - self.abel_sum).abs() < tolerance
    }
}
/// Cesaro summability: C_k sum.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CesaroSum {
    pub order: u32,
    pub partial_sums: Vec<f64>,
}
#[allow(dead_code)]
impl CesaroSum {
    pub fn new(order: u32) -> Self {
        CesaroSum {
            order,
            partial_sums: Vec::new(),
        }
    }
    pub fn add_term(&mut self, term: f64) {
        let prev = self.partial_sums.last().cloned().unwrap_or(0.0);
        self.partial_sums.push(prev + term);
    }
    /// Cesaro C_1 mean (arithmetic mean of partial sums).
    pub fn cesaro_1_mean(&self) -> Option<f64> {
        if self.partial_sums.is_empty() {
            return None;
        }
        let sum: f64 = self.partial_sums.iter().sum();
        Some(sum / self.partial_sums.len() as f64)
    }
    /// Check if the Cesaro mean converges to a given limit.
    pub fn converges_to(&self, limit: f64, tol: f64) -> bool {
        self.cesaro_1_mean()
            .map(|m| (m - limit).abs() < tol)
            .unwrap_or(false)
    }
    pub fn n_terms(&self) -> usize {
        self.partial_sums.len()
    }
}
/// Wiener's general Tauberian theorem via convolution and the Fourier transform.
pub struct WienerTaube {
    /// The kernel function K used in the convolution.
    pub kernel: String,
}
impl WienerTaube {
    /// Create a new WienerTaube instance.
    pub fn new(kernel: impl Into<String>) -> Self {
        Self {
            kernel: kernel.into(),
        }
    }
    /// Convolution Tauberian theorem: limits of convolution integrals.
    pub fn convolution_tauberian(&self) -> String {
        format!(
            "Wiener's Tauberian theorem: Let K = '{}'. If K̂(ξ) ≠ 0 for all ξ ∈ ℝ, \
             and (K * f)(x) → L·∫K as x → ∞, then (K₁ * f)(x) → L·∫K₁ for any K₁ ∈ L¹(ℝ).",
            self.kernel
        )
    }
    /// Norbert Wiener's general theorem (1932).
    pub fn norbert_wiener_general(&self) -> String {
        format!(
            "Wiener (1932) — General Tauberian theorem: A closed ideal I ⊂ L¹(ℝ) equals all \
             of L¹(ℝ) iff no common zero of Fourier transforms exists. \
             Kernel '{}' spans L¹ iff its Fourier transform is nonvanishing.",
            self.kernel
        )
    }
}
/// Tauberian condition: slowness / one-sidedness conditions.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct TauberCond {
    pub name: String,
    pub description: String,
    pub is_one_sided: bool,
}
#[allow(dead_code)]
impl TauberCond {
    pub fn new(name: &str, desc: &str, one_sided: bool) -> Self {
        TauberCond {
            name: name.to_string(),
            description: desc.to_string(),
            is_one_sided: one_sided,
        }
    }
    pub fn slow_oscillation() -> Self {
        TauberCond::new(
            "slow-oscillation",
            "a_n is slowly oscillating (Hardy-Littlewood type)",
            false,
        )
    }
    pub fn one_sided_bounded() -> Self {
        TauberCond::new(
            "one-sided-bounded",
            "a_n ≥ -M for all n (Tauber's original)",
            true,
        )
    }
    pub fn littlewood_condition() -> Self {
        TauberCond::new("Littlewood", "n * a_n is bounded", true)
    }
}
/// A discrete approximation to the Laplace transform of a step function.
///
/// Approximates (Lf)(s) = ∑_{n=0}^N f(n) e^{-sn} Δn for a discretely
/// sampled function f.
#[derive(Debug, Clone)]
pub struct LaplaceTransformApprox {
    /// Sampled values f(0), f(1), ..., f(N).
    pub samples: Vec<f64>,
    /// Step size Δt between samples.
    pub step: f64,
}
impl LaplaceTransformApprox {
    /// Create a `LaplaceTransformApprox` from samples with given step size.
    pub fn new(samples: Vec<f64>, step: f64) -> Self {
        LaplaceTransformApprox { samples, step }
    }
    /// Evaluate the discrete Laplace transform at real parameter s.
    pub fn evaluate(&self, s: f64) -> f64 {
        self.samples
            .iter()
            .enumerate()
            .map(|(n, &f)| f * (-s * n as f64 * self.step).exp() * self.step)
            .sum()
    }
    /// Compute the asymptotic behavior as s → 0⁺: (Lf)(s) should behave like A/s
    /// if f(t) → A as t → ∞.
    pub fn limiting_value_times_s(&self, s: f64) -> f64 {
        s * self.evaluate(s)
    }
    /// Estimate the long-time limit of f(t) by evaluating lim_{s→0} s·(Lf)(s).
    pub fn estimate_long_time_limit(&self) -> f64 {
        let s = 1e-3 / (self.samples.len() as f64 * self.step);
        self.limiting_value_times_s(s)
    }
}
