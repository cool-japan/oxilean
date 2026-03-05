//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use oxilean_kernel::{Declaration, Environment, Expr, Name};
use std::f64::consts::PI;

/// Brun's sieve for estimating twin prime counts.
pub struct BrunSieve {
    /// The sieve level (depth of inclusion-exclusion).
    pub level: u32,
}
impl BrunSieve {
    /// Create a new `BrunSieve` with given inclusion-exclusion `level`.
    pub fn new(level: u32) -> Self {
        BrunSieve { level }
    }
    /// Counts pairs (p, p+2) with p ≤ n both prime, using a simple direct count.
    pub fn count_twin_primes_up_to(&self, n: u64) -> usize {
        if n < 3 {
            return 0;
        }
        let mut sieve = SieveOfEratosthenes::new(n);
        sieve.run();
        let primes = sieve.primes();
        primes.windows(2).filter(|w| w[1] - w[0] == 2).count()
    }
    /// Returns Brun's constant B₂ ≈ 1.902160583... (sum of reciprocals of twin primes).
    pub fn brun_constant_approx() -> f64 {
        1.902_160_583
    }
}
/// The Grand Riemann Hypothesis: all non-trivial zeros of all automorphic L-functions
/// lie on the critical line Re(s) = 1/2.
pub struct GrandRiemannHypothesis {
    /// Whether all non-trivial zeros are on the critical line (conjectured).
    pub all_nontrivial_zeros_on_critical_line: bool,
}
impl GrandRiemannHypothesis {
    /// Create a new `GrandRiemannHypothesis` instance (conjectured = true).
    pub fn new() -> Self {
        GrandRiemannHypothesis {
            all_nontrivial_zeros_on_critical_line: true,
        }
    }
    /// Returns the formal statement of the Grand Riemann Hypothesis.
    pub fn statement(&self) -> &'static str {
        "All non-trivial zeros of all automorphic L-functions have real part 1/2."
    }
}
/// A Dirichlet character χ of given modulus.
pub struct DirichletCharacter {
    /// The modulus q of the character.
    pub modulus: u64,
    /// Whether this is the principal character χ_0.
    pub is_principal: bool,
    /// The order of the character in the character group.
    pub order: u64,
}
impl DirichletCharacter {
    /// Create the principal Dirichlet character modulo `modulus`.
    pub fn new(modulus: u64) -> Self {
        DirichletCharacter {
            modulus,
            is_principal: true,
            order: 1,
        }
    }
    /// Evaluate the character at `n`.
    ///
    /// For the principal character: returns 1.0 if gcd(n, q) = 1, else 0.0.
    pub fn evaluate(&self, n: u64) -> f64 {
        if self.modulus == 0 {
            return 0.0;
        }
        if gcd(n, self.modulus) == 1 {
            1.0
        } else {
            0.0
        }
    }
    /// Returns whether this character is primitive (conductor = modulus).
    pub fn is_primitive(&self) -> bool {
        if self.is_principal {
            self.modulus == 1
        } else {
            true
        }
    }
}
/// Provides upper bounds for exponential sums using various methods.
#[derive(Debug, Clone)]
pub struct ExponentialSumBound {
    /// Number of terms N in the sum.
    pub n: usize,
    /// First derivative bound: |f'(x)| ~ λ₁ (for van der Corput A-process).
    pub lambda1: f64,
    /// Second derivative bound: |f''(x)| ~ λ₂ (for van der Corput B-process).
    pub lambda2: f64,
}
impl ExponentialSumBound {
    /// Create a new `ExponentialSumBound`.
    pub fn new(n: usize, lambda1: f64, lambda2: f64) -> Self {
        ExponentialSumBound {
            n,
            lambda1,
            lambda2,
        }
    }
    /// Trivial bound: |S| ≤ N.
    pub fn trivial_bound(&self) -> f64 {
        self.n as f64
    }
    /// Van der Corput A-process (first derivative test):
    /// |S| ≪ N λ₁^{1/2} + λ₁^{-1/2} (if λ₁ ≤ 1).
    pub fn van_der_corput_a(&self) -> f64 {
        if self.lambda1 <= 0.0 {
            return self.trivial_bound();
        }
        let n = self.n as f64;
        (n * self.lambda1.sqrt() + self.lambda1.powf(-0.5)).min(n)
    }
    /// Van der Corput B-process (second derivative test):
    /// |S| ≪ (N λ₂)^{1/2} + N^{-1} λ₂^{-1/2} (if λ₂ ≤ 1).
    pub fn van_der_corput_b(&self) -> f64 {
        if self.lambda2 <= 0.0 {
            return self.trivial_bound();
        }
        let n = self.n as f64;
        ((n * self.lambda2).sqrt() + self.lambda2.powf(-0.5) / n).min(n)
    }
    /// Weyl's inequality for polynomials f(n) = α_k n^k + ... + α_1 n:
    /// |S| ≪ N^{1 + ε} (N^{-1} + N^{-k} / q + q / N^k)^{δ}
    /// Simplified: returns N^{1 - 2^{1-k}}.
    pub fn weyl_bound(&self, k: u32) -> f64 {
        let n = self.n as f64;
        let exponent = 1.0 - 2.0_f64.powi(1 - k as i32);
        n.powf(exponent)
    }
    /// Exponent pair (k, l) bound: |S| ≪ N^l (N λ₂)^k.
    /// Standard pair (1/6, 2/3): classic van der Corput result.
    pub fn exponent_pair_bound(&self, kk: f64, ll: f64) -> f64 {
        let n = self.n as f64;
        (n.powf(ll) * (n * self.lambda2).powf(kk)).min(n)
    }
    /// Best available bound: minimum of all estimates.
    pub fn best_bound(&self, poly_degree: u32) -> f64 {
        let trivial = self.trivial_bound();
        let vdca = self.van_der_corput_a();
        let vdcb = self.van_der_corput_b();
        let weyl = self.weyl_bound(poly_degree);
        trivial.min(vdca).min(vdcb).min(weyl)
    }
}
/// Linnik's theorem on the least prime in an arithmetic progression.
pub struct LinnikThm {
    /// Linnik's constant L.
    pub linnik_constant: f64,
}
impl LinnikThm {
    /// Create a new LinnikThm with the given constant L.
    pub fn new(l: f64) -> Self {
        Self { linnik_constant: l }
    }
    /// Statement of Linnik's theorem.
    pub fn least_prime_in_ap(&self) -> String {
        format!(
            "The least prime p = a (mod q) satisfies p <= C*q^{:.2}.",
            self.linnik_constant
        )
    }
    /// Known bound on Linnik's constant.
    pub fn linnik_constant_bound(&self) -> f64 {
        5.2_f64.min(self.linnik_constant)
    }
}
/// An exponential sum S = sum_{n=1}^{N} e(f(n)) with phase function and term count.
pub struct ExponentialSumV2 {
    /// String representation of the phase function f.
    pub phase_fn: String,
    /// Number of terms N.
    pub num_terms: usize,
}
impl ExponentialSumV2 {
    /// Create a new ExponentialSumV2.
    pub fn new(phase_fn: String, num_terms: usize) -> Self {
        Self {
            phase_fn,
            num_terms,
        }
    }
    /// Weyl's differencing bound.
    pub fn weyl_bound(&self) -> f64 {
        (self.num_terms as f64).powf(0.75)
    }
    /// Van der Corput bound.
    pub fn van_der_corput(&self) -> f64 {
        (self.num_terms as f64).sqrt()
    }
}
/// The Sieve of Eratosthenes up to a given limit.
pub struct SieveOfEratosthenes {
    /// The sieve limit N: primes up to N are found.
    pub limit: u64,
    /// The sieve array: `sieve[i]` is `true` if `i + 2` is prime.
    pub sieve: Vec<bool>,
}
impl SieveOfEratosthenes {
    /// Create a new sieve for primes up to `limit`.
    pub fn new(limit: u64) -> Self {
        let size = if limit >= 2 { (limit - 1) as usize } else { 0 };
        SieveOfEratosthenes {
            limit,
            sieve: vec![true; size],
        }
    }
    /// Run the sieve, marking composites in `self.sieve`.
    pub fn run(&mut self) {
        if self.limit < 2 {
            return;
        }
        let size = self.sieve.len();
        let mut p = 2usize;
        while p * p <= self.limit as usize {
            if p >= 2 && p - 2 < size && self.sieve[p - 2] {
                let mut multiple = p * p;
                while multiple <= self.limit as usize {
                    self.sieve[multiple - 2] = false;
                    multiple += p;
                }
            }
            p += 1;
        }
    }
    /// Returns all primes found by the sieve (must call `run()` first).
    pub fn primes(&self) -> Vec<u64> {
        self.sieve
            .iter()
            .enumerate()
            .filter(|(_, &is_prime)| is_prime)
            .map(|(i, _)| (i + 2) as u64)
            .collect()
    }
    /// Returns π(limit) = number of primes up to `limit`.
    pub fn prime_count(&self) -> usize {
        self.sieve.iter().filter(|&&b| b).count()
    }
}
/// Represents a Dirichlet arithmetic progression a mod q.
#[allow(dead_code)]
pub struct ArithmeticProgression {
    /// The residue a.
    pub a: u64,
    /// The modulus q.
    pub q: u64,
}
#[allow(dead_code)]
impl ArithmeticProgression {
    /// Create a new arithmetic progression a mod q.
    pub fn new(a: u64, q: u64) -> Self {
        ArithmeticProgression { a, q }
    }
    /// Check that gcd(a, q) = 1 (necessary for Dirichlet's theorem).
    pub fn is_valid(&self) -> bool {
        gcd_u64(self.a, self.q) == 1
    }
    /// Dirichlet's theorem: there are infinitely many primes ≡ a (mod q).
    /// Returns true if gcd(a, q) = 1.
    pub fn dirichlet_theorem_applies(&self) -> bool {
        self.is_valid()
    }
    /// Siegel-Walfisz theorem: for any A > 0,
    /// π(x; q, a) = li(x) / φ(q) + O(x / log^A x).
    /// Returns the main term li(x) / φ(q).
    pub fn siegel_walfisz_main_term(&self, x: f64) -> f64 {
        let _a = self.a;
        let q = self.q;
        let li_x = x / x.ln();
        let phi_q = euler_totient_f64(q);
        li_x / phi_q
    }
    /// Bombieri-Vinogradov theorem: on average over q ≤ x^{1/2} / log^B x,
    /// the error in π(x; q, a) is small. Returns the threshold Q(x, B).
    pub fn bombieri_vinogradov_threshold(x: f64, _b: f64) -> f64 {
        x.sqrt() / x.ln()
    }
    /// Elliott-Halberstam conjecture: the Bombieri-Vinogradov range extends
    /// to Q = x^{1-ε}. Returns the conjectured threshold.
    pub fn elliott_halberstam_threshold(x: f64) -> f64 {
        let epsilon = 0.01;
        x.powf(1.0 - epsilon)
    }
}
/// Encapsulates the prime-counting function π(x) and its approximation.
pub struct PrimeCountingFunction {
    /// The bound x for counting primes.
    pub x: f64,
}
impl PrimeCountingFunction {
    /// Create a new `PrimeCountingFunction` for bound `x`.
    pub fn new(x: f64) -> Self {
        PrimeCountingFunction { x }
    }
    /// Returns the logarithmic integral `li(x) = ∫_2^x dt/ln(t)` as an approximation to π(x).
    ///
    /// Uses a simple numerical approximation: `x / ln(x)` scaled to match `li(x)`.
    pub fn li_approximation(&self) -> f64 {
        if self.x <= 2.0 {
            return 0.0;
        }
        let ln_x = self.x.ln();
        (self.x / ln_x) * (1.0 + 1.0 / ln_x + 2.0 / (ln_x * ln_x))
    }
    /// Returns an estimate of the relative error `|π(x) - li(x)| / π(x)`.
    ///
    /// The PNT guarantees this tends to 0 as x → ∞.
    pub fn relative_error(&self) -> f64 {
        if self.x <= 2.0 {
            return 1.0;
        }
        1.0 / self.x.ln()
    }
}
/// Weight of a modular form.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ModularWeight {
    /// Integer weight k.
    Integer(i64),
    /// Half-integer weight k + 1/2.
    HalfInteger(i64),
}
/// Represents a prime gap between consecutive primes p and q = nextprime(p).
pub struct PrimeGap {
    /// The smaller prime p.
    pub p: u64,
    /// The larger prime q (the prime immediately after p).
    pub q: u64,
    /// The gap g = q - p.
    pub gap: u64,
}
impl PrimeGap {
    /// Create a new `PrimeGap` between primes `p` and `q`.
    pub fn new(p: u64, q: u64) -> Self {
        PrimeGap {
            p,
            q,
            gap: q.saturating_sub(p),
        }
    }
    /// Returns whether this gap is a maximal gap (larger than all preceding prime gaps).
    ///
    /// This is a simplified heuristic: gaps ≥ 2 · ln(p) are considered notable.
    pub fn is_maximal_gap(&self) -> bool {
        if self.p < 2 {
            return false;
        }
        let expected = 2.0 * (self.p as f64).ln();
        self.gap as f64 >= expected
    }
}
/// Represents the Riemann zeta function ζ(s).
///
/// The argument `s` is simplified to a real value (`Option<f64>`);
/// `None` indicates a symbolic/unspecified argument.
pub struct ZetaFunction {
    /// The real part of the complex argument s.
    pub s: Option<f64>,
}
impl ZetaFunction {
    /// Create a new `ZetaFunction` with argument `s`.
    pub fn new(s: f64) -> Self {
        ZetaFunction { s: Some(s) }
    }
    /// Euler product factor at prime `p`: `(1 - p^{-s})^{-1}`.
    ///
    /// Returns the local factor at `p` in the Euler product `ζ(s) = ∏_p (1 - p^{-s})^{-1}`.
    pub fn euler_product_factor(&self, p: u64) -> f64 {
        match self.s {
            Some(s) => {
                let p_neg_s = (p as f64).powf(-s);
                1.0 / (1.0 - p_neg_s)
            }
            None => 1.0,
        }
    }
    /// Returns the critical strip as `(left, right)` = `(0.0, 1.0)`.
    ///
    /// The non-trivial zeros of ζ(s) lie in the critical strip `0 < Re(s) < 1`.
    pub fn critical_strip(&self) -> (f64, f64) {
        (0.0, 1.0)
    }
    /// Returns the trivial zeros of ζ(s): the negative even integers `{-2, -4, -6, ...}`.
    pub fn trivial_zeros(&self) -> Vec<i64> {
        (1..=10).map(|k| -2 * k).collect()
    }
}
/// Represents an elliptic curve E over Q in simplified Weierstrass form:
/// y^2 = x^3 + ax + b.
#[allow(dead_code)]
pub struct EllipticCurveLFunction {
    /// Coefficient a.
    pub a: i64,
    /// Coefficient b.
    pub b: i64,
    /// The analytic rank (conjectured by BSD).
    pub analytic_rank: u64,
    /// The algebraic rank.
    pub algebraic_rank: u64,
}
#[allow(dead_code)]
impl EllipticCurveLFunction {
    /// Create a new elliptic curve L-function descriptor.
    pub fn new(a: i64, b: i64, analytic_rank: u64, algebraic_rank: u64) -> Self {
        EllipticCurveLFunction {
            a,
            b,
            analytic_rank,
            algebraic_rank,
        }
    }
    /// The discriminant Δ = -16(4a^3 + 27b^2) (must be non-zero for non-singularity).
    pub fn discriminant(&self) -> i64 {
        -16 * (4 * self.a.pow(3) + 27 * self.b.pow(2))
    }
    /// Returns true if the curve is non-singular (Δ ≠ 0).
    pub fn is_non_singular(&self) -> bool {
        self.discriminant() != 0
    }
    /// BSD conjecture (weak form): ord_{s=1} L(E, s) = rank(E(Q)).
    pub fn bsd_weak_holds(&self) -> bool {
        self.analytic_rank == self.algebraic_rank
    }
    /// The conductor N_E (rough estimate via discriminant).
    pub fn conductor_estimate(&self) -> u64 {
        let d = self.discriminant().unsigned_abs();
        if d == 0 {
            1
        } else {
            d
        }
    }
    /// L-function convergence: converges for Re(s) > 3/2.
    pub fn convergence_abscissa(&self) -> f64 {
        1.5
    }
    /// The functional equation relates L(E, s) and L(E, 2-s).
    /// Returns the symmetry center s = 1.
    pub fn functional_equation_center(&self) -> f64 {
        1.0
    }
    /// Approximate the leading coefficient in the Taylor expansion at s=1:
    /// L(E, s) ~ c_r (s-1)^r + ... near s=1 where r = analytic rank.
    pub fn leading_coefficient_sign(&self) -> i32 {
        if self.analytic_rank % 2 == 0 {
            1
        } else {
            -1
        }
    }
    /// The Birch-Swinnerton-Dyer product over primes approximation.
    /// ∏_{p ≤ P} (a_p / p) for local factors.
    pub fn bsd_product_approx(&self, _p_max: u64) -> f64 {
        1.0
    }
}
/// Checks properties of zeros of L-functions numerically (simplified model).
#[derive(Debug, Clone)]
pub struct LFunctionZeroChecker {
    /// The name of the L-function.
    pub name: String,
    /// The conductor.
    pub conductor: u64,
    /// Known non-trivial zeros as (re, im) pairs.
    pub known_zeros: Vec<(f64, f64)>,
}
impl LFunctionZeroChecker {
    /// Create a new `LFunctionZeroChecker` for the Riemann zeta function.
    pub fn riemann_zeta() -> Self {
        LFunctionZeroChecker {
            name: "Riemann zeta".to_string(),
            conductor: 1,
            known_zeros: vec![
                (0.5, 14.134_725),
                (0.5, 21.022_040),
                (0.5, 25.010_858),
                (0.5, 30.424_876),
                (0.5, 32.935_062),
            ],
        }
    }
    /// Create a new `LFunctionZeroChecker` for a Dirichlet L-function mod `q`.
    pub fn dirichlet(q: u64) -> Self {
        LFunctionZeroChecker {
            name: format!("Dirichlet L-function mod {}", q),
            conductor: q,
            known_zeros: Vec::new(),
        }
    }
    /// Check if all known zeros lie on the critical line Re(s) = 1/2.
    pub fn all_zeros_on_critical_line(&self) -> bool {
        self.known_zeros
            .iter()
            .all(|(re, _)| (re - 0.5).abs() < 1e-6)
    }
    /// Check if a given (re, im) pair lies in the critical strip 0 < Re(s) < 1.
    pub fn in_critical_strip(&self, re: f64) -> bool {
        re > 0.0 && re < 1.0
    }
    /// Estimate the zero-free region: σ > 1 - c / log(t) for |Im(s)| = t.
    pub fn zero_free_bound(&self, t: f64) -> f64 {
        if t <= 1.0 {
            return 0.0;
        }
        let c = 1.0 / (5.7 * (self.conductor as f64).ln().max(1.0));
        1.0 - c / t.ln()
    }
    /// Returns whether a Siegel zero might exist for this L-function.
    /// Heuristic: for small conductors, Siegel zeros are very rare.
    pub fn siegel_zero_possible(&self) -> bool {
        self.conductor > 1000
    }
    /// Count zeros with |Im(s)| ≤ T (Weyl law approximation: ~ T/(2π) log(T/(2πe))).
    pub fn zero_count_estimate(&self, big_t: f64) -> f64 {
        if big_t <= 1.0 {
            return 0.0;
        }
        big_t / (2.0 * PI) * (big_t / (2.0 * PI * std::f64::consts::E)).ln().max(0.0)
    }
}
/// The Birch and Swinnerton-Dyer Conjecture for an elliptic curve E.
pub struct BirchSwinnertonDyerConjecture {
    /// Identifier for the elliptic curve (e.g., "E: y^2 = x^3 - x").
    pub curve: String,
    /// The algebraic rank r = rank E(Q) (conjectured to equal the analytic rank).
    pub rank: i64,
}
impl BirchSwinnertonDyerConjecture {
    /// Create a new `BirchSwinnertonDyerConjecture` instance.
    pub fn new() -> Self {
        BirchSwinnertonDyerConjecture {
            curve: "E: y^2 = x^3 - x".to_string(),
            rank: 0,
        }
    }
    /// Returns the formal statement of the BSD conjecture.
    pub fn bsd_statement(&self) -> &'static str {
        "For an elliptic curve E/Q, the order of vanishing of L(E, s) at s = 1 \
         equals the rank of the Mordell-Weil group E(Q), and the leading coefficient \
         is given by the BSD formula involving periods, Tamagawa numbers, and Sha."
    }
}
/// Zero-free region for the Riemann zeta function.
pub struct ZeroFreeRegion {
    /// Width parameter of the zero-free region.
    pub zero_free_width: f64,
}
impl ZeroFreeRegion {
    /// Create a new ZeroFreeRegion with the given width parameter.
    pub fn new(zero_free_width: f64) -> Self {
        Self { zero_free_width }
    }
    /// Returns true if a Siegel zero might exist.
    pub fn siegel_zero_exists(&self) -> bool {
        self.zero_free_width < 1e-3
    }
    /// De la Vallee-Poussin zero-free region statement.
    pub fn de_la_vallee_poussin(&self) -> String {
        format!(
            "zeta(s) != 0 for Re(s) > 1 - {:.4} / ln(|Im(s)|).",
            self.zero_free_width
        )
    }
}
/// A Dirichlet L-function `L(s, χ)` associated to a character χ.
pub struct DirichletLFunction {
    /// The Dirichlet character associated to this L-function.
    pub character: DirichletCharacter,
}
impl DirichletLFunction {
    /// Create a new `DirichletLFunction` with the principal character mod 1.
    pub fn new() -> Self {
        DirichletLFunction {
            character: DirichletCharacter::new(1),
        }
    }
    /// Returns a string describing the functional equation of `L(s, χ)`.
    pub fn functional_equation_statement(&self) -> String {
        format!(
            "L(s, χ) satisfies a functional equation relating L(s, χ) to L(1-s, χ̄) \
             via a Gamma factor, with conductor q = {}.",
            self.character.modulus
        )
    }
}
/// Implements the Hardy-Littlewood-Ramanujan circle method for partition
/// and Waring-Goldbach type problems.
#[allow(dead_code)]
pub struct WaringCircleMethod {
    /// The number n for which we estimate the representation count.
    pub n: u64,
    /// Number of terms s in Waring's problem: n = x_1^k + ... + x_s^k.
    pub s: u64,
    /// The exponent k in Waring's problem.
    pub k: u64,
}
#[allow(dead_code)]
impl WaringCircleMethod {
    /// Create a new `WaringCircleMethod` instance.
    pub fn new(n: u64, s: u64, k: u64) -> Self {
        WaringCircleMethod { n, s, k }
    }
    /// The major arc contribution in the circle method (approximate).
    /// Returns the singular series times the singular integral approximation.
    pub fn major_arc_contribution(&self) -> f64 {
        let n_f = self.n as f64;
        let s_f = self.s as f64;
        let k_f = self.k as f64;
        let gamma_ratio = 1.0;
        gamma_ratio * n_f.powf(s_f / k_f - 1.0)
    }
    /// The minor arc bound: O(n^{s/k - δ}) for some δ > 0.
    /// Returns the order of magnitude of the minor arc estimate.
    pub fn minor_arc_order(&self) -> f64 {
        let n_f = self.n as f64;
        let s_f = self.s as f64;
        let k_f = self.k as f64;
        let _delta = 0.01;
        n_f.powf(s_f / k_f - 0.01)
    }
    /// Waring's problem: Hilbert-Waring theorem states that g(k) exists.
    /// Returns the known values of g(k) for small k.
    pub fn waring_g(&self) -> Option<u64> {
        match self.k {
            2 => Some(4),
            3 => Some(9),
            4 => Some(19),
            5 => Some(37),
            6 => Some(73),
            7 => Some(143),
            8 => Some(279),
            _ => None,
        }
    }
    /// Goldbach-like: every sufficiently large even number is sum of two primes.
    /// Returns a heuristic count via the twin-prime analog.
    pub fn goldbach_count_heuristic(&self) -> f64 {
        let n_f = self.n as f64;
        2.0 * n_f / (n_f.ln() * n_f.ln())
    }
    /// Hardy-Ramanujan asymptotic for partition function p(n).
    /// p(n) ~ exp(π√(2n/3)) / (4n√3).
    pub fn partition_asymptotic(n: u64) -> f64 {
        let n_f = n as f64;
        let exp_arg = PI * (2.0 * n_f / 3.0_f64).sqrt();
        exp_arg.exp() / (4.0 * n_f * 3.0_f64.sqrt())
    }
}
/// Estimates for various sieve methods in analytic number theory.
#[derive(Debug, Clone)]
pub struct SieveEstimator {
    /// The sieve bound N.
    pub n: u64,
    /// The sieve level Q (primes up to Q are sieved out).
    pub q: u64,
}
impl SieveEstimator {
    /// Create a new `SieveEstimator` for sieving up to `n` with primes up to `q`.
    pub fn new(n: u64, q: u64) -> Self {
        SieveEstimator { n, q }
    }
    /// Large sieve inequality bound: (N + Q^2) * sum of coefficients squared.
    /// Returns the factor (N + Q^2).
    pub fn large_sieve_factor(&self) -> f64 {
        (self.n as f64) + (self.q as f64).powi(2)
    }
    /// Bombieri-Vinogradov theorem bound estimate.
    /// Returns Q such that primes are well-distributed in AP mod q ≤ Q ≈ √N / (log N)^B.
    pub fn bombieri_vinogradov_q(&self) -> f64 {
        if self.n < 2 {
            return 1.0;
        }
        let ln_n = (self.n as f64).ln();
        (self.n as f64).sqrt() / ln_n.powi(5)
    }
    /// Selberg sieve upper bound estimate: main term ~ N / log(Q).
    pub fn selberg_upper_bound(&self) -> f64 {
        if self.q < 2 {
            return self.n as f64;
        }
        let ln_q = (self.q as f64).ln();
        (self.n as f64) / ln_q
    }
    /// Brun-Titchmarsh upper bound for π(x; q, a): primes ≡ a (mod q) up to x.
    /// Bound: 2x / (φ(q) log(x/q)) for x > q^2.
    pub fn brun_titchmarsh_bound(&self, a: u64) -> f64 {
        if self.q == 0 || self.n <= self.q {
            return 0.0;
        }
        let phi_q = euler_totient(self.q) as f64;
        let ln_xq = ((self.n as f64) / (self.q as f64)).ln().max(1.0);
        let _ = a;
        2.0 * (self.n as f64) / (phi_q * ln_xq)
    }
    /// Estimate the number of primes detected by the sieve in [N/2, N].
    pub fn sieve_prime_count_estimate(&self) -> u64 {
        if self.n < 2 {
            return 0;
        }
        let ln_n = (self.n as f64).ln();
        ((self.n as f64) / (2.0 * ln_n)) as u64
    }
}
/// Represents the Dirichlet convolution (f * g)(n) = Σ_{d|n} f(d) g(n/d).
pub struct DirichletConvolution {
    /// Name of the first function f.
    pub f: String,
    /// Name of the second function g.
    pub g: String,
}
impl DirichletConvolution {
    /// Create a new `DirichletConvolution` of f and g.
    pub fn new() -> Self {
        DirichletConvolution {
            f: "f".to_string(),
            g: "g".to_string(),
        }
    }
    /// Compute (f * g)(n) = Σ_{d|n} f(d) g(n/d).
    pub fn compute(&self, n: u64, f: impl Fn(u64) -> f64, g: impl Fn(u64) -> f64) -> f64 {
        if n == 0 {
            return 0.0;
        }
        (1..=n)
            .filter(|&d| n % d == 0)
            .map(|d| f(d) * g(n / d))
            .sum()
    }
}
/// A finite exponential sum Σ A_j e^{i φ_j} represented as a list of (amplitude, phase) terms.
pub struct ExponentialSum {
    /// The terms (amplitude A_j, phase φ_j) of the sum.
    pub terms: Vec<(f64, f64)>,
}
impl ExponentialSum {
    /// Create a new empty `ExponentialSum`.
    pub fn new() -> Self {
        ExponentialSum { terms: Vec::new() }
    }
    /// Add a term with given amplitude and phase to the sum.
    pub fn add_term(&mut self, amp: f64, phase: f64) {
        self.terms.push((amp, phase));
    }
    /// Evaluate the sum, returning `(Re S, Im S)`.
    pub fn evaluate(&self) -> (f64, f64) {
        let re: f64 = self.terms.iter().map(|(a, phi)| a * phi.cos()).sum();
        let im: f64 = self.terms.iter().map(|(a, phi)| a * phi.sin()).sum();
        (re, im)
    }
    /// Returns the absolute value |S| of the sum.
    pub fn absolute_value(&self) -> f64 {
        let (re, im) = self.evaluate();
        (re * re + im * im).sqrt()
    }
}
/// A formal multiplicative arithmetic function f: ℕ → ℂ.
pub struct MultiplicativeFunction {
    /// Human-readable name of the function.
    pub name: String,
    /// Whether f(mn) = f(m)f(n) for ALL m, n (not just coprime pairs).
    pub is_completely_multiplicative: bool,
}
impl MultiplicativeFunction {
    /// Create a new `MultiplicativeFunction` with given name and multiplicativity type.
    pub fn new(name: &str, is_completely_mult: bool) -> Self {
        MultiplicativeFunction {
            name: name.to_string(),
            is_completely_multiplicative: is_completely_mult,
        }
    }
    /// Returns `true` since by definition all instances of this struct are multiplicative.
    pub fn satisfies_multiplicativity(&self) -> bool {
        true
    }
}
/// Riemann zeta function with complex argument s = s_real + i*s_imag.
pub struct RiemannZeta {
    /// Real part of s.
    pub s_real: f64,
    /// Imaginary part of s.
    pub s_imag: f64,
}
impl RiemannZeta {
    /// Create a new RiemannZeta at s = s_real + i*s_imag.
    pub fn new(s_real: f64, s_imag: f64) -> Self {
        Self { s_real, s_imag }
    }
    /// Returns true if the Euler product converges, i.e., Re(s) > 1.
    pub fn euler_product_converges(&self) -> bool {
        self.s_real > 1.0
    }
    /// Returns true if s lies in the critical strip 0 < Re(s) < 1.
    pub fn critical_strip_condition(&self) -> bool {
        self.s_real > 0.0 && self.s_real < 1.0
    }
}
/// Sieve methods used in analytic number theory.
#[derive(Debug, Clone, PartialEq)]
pub enum SieveMethod {
    /// Classical sieve of Eratosthenes.
    Eratosthenes,
    /// Brun's pure sieve for twin primes.
    Brun,
    /// Selberg's upper-bound sieve.
    Selberg,
    /// Large sieve inequality.
    LargeSieve,
}
impl SieveMethod {
    /// Returns a short description of the sieve method.
    pub fn description(&self) -> &'static str {
        match self {
            SieveMethod::Eratosthenes => "Sieve of Eratosthenes: iteratively mark composites.",
            SieveMethod::Brun => "Brun's sieve: inclusion-exclusion truncated for twin primes.",
            SieveMethod::Selberg => "Selberg's sieve: minimises a quadratic form for upper bounds.",
            SieveMethod::LargeSieve => {
                "Large sieve: bounds exponential sums via Montgomery-Vaughan."
            }
        }
    }
}
/// Hardy-Littlewood circle method.
pub struct CircleMethod {
    /// Major arc rational approximations (numerator, denominator).
    pub major_arcs: Vec<(u64, u64)>,
    /// Whether minor arcs contribute non-trivially.
    pub minor_arcs: bool,
}
impl CircleMethod {
    /// Create a new CircleMethod.
    pub fn new(major_arcs: Vec<(u64, u64)>, minor_arcs: bool) -> Self {
        Self {
            major_arcs,
            minor_arcs,
        }
    }
    /// Vinogradov's theorem (1937).
    pub fn vinogradov_theorem(&self) -> String {
        "Every sufficiently large odd integer N can be written as N = p1 + p2 + p3 \
         where p1, p2, p3 are prime (Vinogradov 1937)."
            .to_string()
    }
}
/// A general automorphic L-function with given analytic data.
pub struct LFunction {
    /// Human-readable name (e.g., "Riemann zeta", "L(s, chi)").
    pub name: String,
    /// The conductor N of the L-function.
    pub conductor: u64,
    /// The root number ε ∈ {±1} (sign of the functional equation).
    pub root_number: f64,
    /// The degree d (number of Gamma factors).
    pub degree: usize,
}
impl LFunction {
    /// Create a new `LFunction` (Riemann zeta as default).
    pub fn new() -> Self {
        LFunction {
            name: "Riemann zeta".to_string(),
            conductor: 1,
            root_number: 1.0,
            degree: 1,
        }
    }
    /// Returns the Gamma factor `Γ_ℝ(s)^d` normalisation constant (simplified as `1.0`).
    pub fn functional_equation_factor(&self) -> f64 {
        (self.conductor as f64).sqrt()
    }
    /// Returns the (approximate) central value L(1/2) when available.
    ///
    /// Returns `None` for L-functions where the central value is not computable here.
    pub fn central_value(&self) -> Option<f64> {
        if self.name == "Riemann zeta" {
            Some(-1.460_354_508)
        } else {
            None
        }
    }
}
/// Computes Gauss sums and Jacobi sums numerically (as complex (re, im) pairs).
#[derive(Debug, Clone)]
pub struct GaussSumComputer {
    /// The modulus q.
    pub modulus: u64,
}
impl GaussSumComputer {
    /// Create a new `GaussSumComputer` for modulus `q`.
    pub fn new(modulus: u64) -> Self {
        GaussSumComputer { modulus }
    }
    /// Compute the Gauss sum τ(χ_0) for the principal character mod q.
    /// For the principal character: τ = μ(q) (Ramanujan sum at 1).
    pub fn principal_gauss_sum(&self) -> (f64, f64) {
        let q = self.modulus;
        let (mut re, mut im) = (0.0f64, 0.0f64);
        for a in 1..=q {
            if gcd(a, q) == 1 {
                let theta = 2.0 * PI * (a as f64) / (q as f64);
                re += theta.cos();
                im += theta.sin();
            }
        }
        (re, im)
    }
    /// Compute |τ(χ)|^2 for a primitive character: equals q.
    pub fn magnitude_squared_primitive(&self) -> f64 {
        self.modulus as f64
    }
    /// Compute a Jacobi sum J(χ₁, χ₂) (simplified: returns (q)^{1/2} for primitive chars).
    pub fn jacobi_sum_bound(&self) -> f64 {
        (self.modulus as f64).sqrt()
    }
    /// Compute the Ramanujan sum c_q(n) = Σ_{a: gcd(a,q)=1} e^{2πian/q}.
    pub fn ramanujan_sum(&self, n: u64) -> f64 {
        let q = self.modulus;
        (1..=q)
            .filter(|&a| gcd(a, q) == 1)
            .map(|a| {
                let theta = 2.0 * PI * (a as f64) * (n as f64) / (q as f64);
                theta.cos()
            })
            .sum()
    }
    /// Compute the Kloosterman sum S(a, b; q) = Σ_{x: gcd(x,q)=1} e^{2πi(ax+bx̄)/q}.
    /// Here x̄ denotes the modular inverse of x.
    pub fn kloosterman_sum(&self, a: i64, b: i64) -> (f64, f64) {
        let q = self.modulus;
        let (mut re, mut im) = (0.0f64, 0.0f64);
        for x in 1..q {
            if gcd(x, q) != 1 {
                continue;
            }
            if let Some(x_inv) = mod_inverse(x, q) {
                let phase = 2.0 * PI * ((a * x as i64 + b * x_inv as i64) as f64) / (q as f64);
                re += phase.cos();
                im += phase.sin();
            }
        }
        (re, im)
    }
    /// Returns the Weil bound on Kloosterman sums: 2√q.
    pub fn weil_bound(&self) -> f64 {
        2.0 * (self.modulus as f64).sqrt()
    }
}
/// Goldbach's conjecture.
pub struct GoldbachConjecture {
    /// Whether the conjecture has been proven.
    pub is_proven: bool,
    /// Description of the best known result.
    pub best_result: String,
}
impl GoldbachConjecture {
    /// Create a new GoldbachConjecture.
    pub fn new(is_proven: bool, best_result: String) -> Self {
        Self {
            is_proven,
            best_result,
        }
    }
    /// Returns the statement of Goldbach's conjecture.
    pub fn statement(&self) -> String {
        format!(
            "Every even integer > 2 is the sum of two primes. (Proven: {}. Best: {})",
            self.is_proven, self.best_result
        )
    }
}
/// A Dirichlet series `Σ a_n / n^s` with given coefficients.
pub struct DirichletSeries {
    /// Coefficients a_1, a_2, ... of the series.
    pub coefficients: Vec<f64>,
    /// The abscissa of convergence σ_c.
    pub abscissa_of_convergence: f64,
}
impl DirichletSeries {
    /// Create a new empty `DirichletSeries`.
    pub fn new() -> Self {
        DirichletSeries {
            coefficients: Vec::new(),
            abscissa_of_convergence: 1.0,
        }
    }
    /// Compute the partial sum `Σ_{n=1}^{N} a_n / n^s`.
    pub fn partial_sum(&self, n: usize, s: f64) -> f64 {
        let limit = n.min(self.coefficients.len());
        (0..limit)
            .map(|i| self.coefficients[i] / ((i + 1) as f64).powf(s))
            .sum()
    }
    /// Returns the abscissa of absolute convergence `σ_a ≥ σ_c`.
    pub fn abscissa_of_absolute_convergence(&self) -> f64 {
        self.abscissa_of_convergence + 1.0
    }
}
/// Prime Number Theorem: π(x) ~ x / ln(x).
pub struct PrimeNumberThm;
impl PrimeNumberThm {
    /// Create a new PrimeNumberThm instance.
    pub fn new() -> Self {
        Self
    }
    /// Logarithmic integral approximation Li(x).
    pub fn li_approximation(x: f64) -> f64 {
        if x <= 1.0 {
            return 0.0;
        }
        let ln_x = x.ln();
        x / ln_x * (1.0 + 1.0 / ln_x + 2.0 / (ln_x * ln_x))
    }
    /// Estimate π(x) via Li(x).
    pub fn prime_counting_estimate(x: f64) -> u64 {
        Self::li_approximation(x).round() as u64
    }
    /// Relative error bound.
    pub fn relative_error(x: f64) -> f64 {
        if x <= 2.0 {
            return 0.0;
        }
        let ln_x = x.ln();
        (-(0.1 * ln_x.sqrt())).exp()
    }
}
/// A Kloosterman sum S(a, b; m) = Σ_{gcd(x,m)=1} e^{2πi(ax + b x̄)/m}.
pub struct KloostermanSum {
    /// The parameter a.
    pub a: i64,
    /// The parameter b.
    pub b: i64,
    /// The modulus m.
    pub modulus: u64,
}
impl KloostermanSum {
    /// Create a new `KloostermanSum` S(a, b; m).
    pub fn new(a: i64, b: i64, m: u64) -> Self {
        KloostermanSum { a, b, modulus: m }
    }
    /// Returns Weil's bound: |S(a, b; m)| ≤ 2√m (for prime m with m ∤ a, m ∤ b).
    pub fn weil_bound(&self) -> f64 {
        2.0 * (self.modulus as f64).sqrt()
    }
}
/// A modular form descriptor: level, weight, and character.
#[allow(dead_code)]
pub struct ModularForm {
    /// The level N (a positive integer).
    pub level: u64,
    /// The weight of the form.
    pub weight: ModularWeight,
    /// Whether the form is a cusp form.
    pub is_cusp_form: bool,
    /// Whether the form is an eigenform (Hecke eigenform).
    pub is_eigenform: bool,
}
#[allow(dead_code)]
impl ModularForm {
    /// Create a new modular form descriptor.
    pub fn new(level: u64, weight: ModularWeight, is_cusp_form: bool, is_eigenform: bool) -> Self {
        ModularForm {
            level,
            weight,
            is_cusp_form,
            is_eigenform,
        }
    }
    /// Returns the dimension of the space of cusp forms S_k(Γ_0(N))
    /// using the Riemann-Roch theorem approximation:
    /// dim S_k(Γ_0(N)) ≈ (k-1)(N-1)/12 for large k, N.
    pub fn cusp_form_dimension_approx(&self) -> f64 {
        match self.weight {
            ModularWeight::Integer(k) if k >= 2 => {
                let k_f = k as f64;
                let n_f = self.level as f64;
                ((k_f - 1.0) * (n_f - 1.0)) / 12.0
            }
            _ => 0.0,
        }
    }
    /// The Ramanujan-Petersson conjecture: for a Hecke eigenform of weight k,
    /// the Hecke eigenvalue λ_p satisfies |λ_p| ≤ 2 p^{(k-1)/2}.
    /// Returns the Ramanujan bound at prime p.
    pub fn ramanujan_bound(&self, p: u64) -> f64 {
        match self.weight {
            ModularWeight::Integer(k) if k >= 1 => 2.0 * (p as f64).powf((k as f64 - 1.0) / 2.0),
            _ => 2.0,
        }
    }
    /// The Petersson inner product normalization constant (approximate).
    pub fn petersson_norm_approx(&self) -> f64 {
        match self.weight {
            ModularWeight::Integer(k) if k >= 2 => {
                let k_f = k as f64;
                let factorial_k1 = (1..k as u64).map(|i| i as f64).product::<f64>();
                factorial_k1 / (4.0 * PI).powf(k_f)
            }
            _ => 1.0,
        }
    }
    /// The L-function attached to the modular form: convergence abscissa.
    /// For weight k, the L-function converges for Re(s) > k.
    pub fn l_function_convergence_abscissa(&self) -> f64 {
        match self.weight {
            ModularWeight::Integer(k) => k as f64,
            ModularWeight::HalfInteger(k) => k as f64 + 0.5,
        }
    }
    /// Returns the functional equation symmetry point s = k/2.
    pub fn functional_equation_center(&self) -> f64 {
        match self.weight {
            ModularWeight::Integer(k) => k as f64 / 2.0,
            ModularWeight::HalfInteger(k) => (k as f64 + 0.5) / 2.0,
        }
    }
}
/// A Gauss sum τ(χ) = Σ_{a mod q} χ(a) e^{2πi a/q}.
pub struct GaussSum {
    /// The Dirichlet character used in the sum.
    pub character: DirichletCharacter,
    /// The modulus q.
    pub modulus: u64,
}
impl GaussSum {
    /// Create a new `GaussSum` for the principal character modulo `modulus`.
    pub fn new(modulus: u64) -> Self {
        GaussSum {
            character: DirichletCharacter::new(modulus),
            modulus,
        }
    }
    /// Returns |τ(χ)|^2 = q for a primitive character mod q, or 0 for the principal character.
    pub fn magnitude_squared(&self) -> f64 {
        if self.character.is_primitive() && self.modulus > 1 {
            self.modulus as f64
        } else {
            0.0
        }
    }
    /// Returns whether this is a Ramanujan sum c_q(n) (sum with principal character).
    pub fn is_ramanujan_sum(&self) -> bool {
        self.character.is_principal
    }
}
/// Represents a finite subset A of an abelian group (modeled as Vec<i64>).
#[allow(dead_code)]
pub struct AdditiveSet {
    pub elements: Vec<i64>,
}
#[allow(dead_code)]
impl AdditiveSet {
    /// Create a new `AdditiveSet` from a sorted, deduplicated list.
    pub fn new(mut elements: Vec<i64>) -> Self {
        elements.sort();
        elements.dedup();
        AdditiveSet { elements }
    }
    /// Returns |A|.
    pub fn size(&self) -> usize {
        self.elements.len()
    }
    /// The sumset A + A = {a + b : a, b ∈ A}.
    pub fn sumset(&self) -> AdditiveSet {
        let mut result = std::collections::BTreeSet::new();
        for &a in &self.elements {
            for &b in &self.elements {
                result.insert(a + b);
            }
        }
        AdditiveSet::new(result.into_iter().collect())
    }
    /// The difference set A - A = {a - b : a, b ∈ A}.
    pub fn difference_set(&self) -> AdditiveSet {
        let mut result = std::collections::BTreeSet::new();
        for &a in &self.elements {
            for &b in &self.elements {
                result.insert(a - b);
            }
        }
        AdditiveSet::new(result.into_iter().collect())
    }
    /// Freiman-Ruzsa doubling constant: |A + A| / |A|.
    pub fn doubling_constant(&self) -> f64 {
        if self.elements.is_empty() {
            return 0.0;
        }
        self.sumset().size() as f64 / self.size() as f64
    }
    /// Plünnecke-Ruzsa inequality: if |A + B| ≤ K|A| then |nB - mB| ≤ K^{n+m} |A|.
    /// Returns the Plünnecke-Ruzsa bound for given K, n, m.
    pub fn plunnecke_ruzsa_bound(_k: f64, n: u32, m: u32, a_size: usize) -> f64 {
        let exponent = (n + m) as f64;
        _k.powf(exponent) * a_size as f64
    }
    /// Balog-Szemerédi-Gowers theorem: a set with many additive quadruples
    /// contains a large subset with small doubling constant.
    /// Returns the size lower bound for the structured subset.
    pub fn bsg_structured_subset_bound(&self, additive_energy: u64) -> f64 {
        let a = self.size() as f64;
        if additive_energy == 0 {
            return 0.0;
        }
        a * a / (additive_energy as f64).sqrt()
    }
    /// Additive energy E(A) = |{(a1,a2,b1,b2) : a1+a2 = b1+b2}|.
    pub fn additive_energy(&self) -> u64 {
        let mut energy = 0u64;
        let n = self.elements.len();
        use super::functions::*;
        use std::collections::HashMap;
        let mut sums: HashMap<i64, u64> = HashMap::new();
        for i in 0..n {
            for j in 0..n {
                *sums.entry(self.elements[i] + self.elements[j]).or_insert(0) += 1;
            }
        }
        for &count in sums.values() {
            energy += count * count;
        }
        energy
    }
    /// Sarkozy's theorem: any dense subset of {1,...,N} contains an arithmetic
    /// progression. Returns heuristic min density threshold.
    pub fn sarkozy_density_threshold(n: u64) -> f64 {
        1.0 / (n as f64).ln().ln().max(1.0)
    }
}
/// Represents the Riemann Hypothesis.
pub struct RiemannHypothesis {
    /// Formal statement of the hypothesis.
    pub statement: String,
}
impl RiemannHypothesis {
    /// Create a new `RiemannHypothesis` instance with the standard statement.
    pub fn new() -> Self {
        RiemannHypothesis {
            statement: riemann_hypothesis_statement().to_string(),
        }
    }
    /// Returns the real part of the critical line: `1/2`.
    pub fn critical_line(&self) -> f64 {
        0.5
    }
}
