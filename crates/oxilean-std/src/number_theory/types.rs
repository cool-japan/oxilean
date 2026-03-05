//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
/// L-function data.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LFunctionData {
    pub name: String,
    pub degree: usize,
    pub conductor: u64,
    pub sign: f64,
    pub central_value: Option<f64>,
}
impl LFunctionData {
    #[allow(dead_code)]
    pub fn riemann_zeta() -> Self {
        Self {
            name: "zeta(s)".to_string(),
            degree: 1,
            conductor: 1,
            sign: 1.0,
            central_value: None,
        }
    }
    #[allow(dead_code)]
    pub fn elliptic_curve_l(curve: &str, conductor: u64) -> Self {
        Self {
            name: format!("L(E_{curve}, s)"),
            degree: 2,
            conductor,
            sign: 1.0,
            central_value: None,
        }
    }
    #[allow(dead_code)]
    pub fn functional_equation_description(&self) -> String {
        let _k = self.degree;
        format!("Completed L: Lambda(s) = (N/pi^k)^(s/2) Gamma(s) L(s); Lambda(s)=eps*Lambda(1-s)")
    }
    #[allow(dead_code)]
    pub fn analytic_rank_hypothesis(&self) -> String {
        format!("BSD: analytic rank = algebraic rank for {}", self.name)
    }
}
/// Algebraic number field.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct NumberField {
    pub name: String,
    pub degree: usize,
    pub discriminant: i64,
    pub class_number: u64,
}
impl NumberField {
    #[allow(dead_code)]
    pub fn rationals() -> Self {
        Self {
            name: "Q".to_string(),
            degree: 1,
            discriminant: 1,
            class_number: 1,
        }
    }
    #[allow(dead_code)]
    pub fn gaussian_integers() -> Self {
        Self {
            name: "Q(i)".to_string(),
            degree: 2,
            discriminant: -4,
            class_number: 1,
        }
    }
    #[allow(dead_code)]
    pub fn quadratic(d: i64) -> Self {
        let disc = if d % 4 == 1 { d } else { 4 * d };
        Self {
            name: format!("Q(sqrt({d}))"),
            degree: 2,
            discriminant: disc,
            class_number: 1,
        }
    }
    #[allow(dead_code)]
    pub fn is_pid(&self) -> bool {
        self.class_number == 1
    }
    #[allow(dead_code)]
    pub fn unit_rank(&self) -> usize {
        if self.degree == 1 {
            0
        } else {
            self.degree - 1
        }
    }
}
/// Elliptic curve point counting over GF(p) by brute force.
///
/// WARNING: O(p²) — educational only.
#[derive(Debug, Clone)]
pub struct EllipticCurvePointCounting {
    /// Coefficient a in y² = x³ + ax + b mod p.
    pub a: u64,
    /// Coefficient b in y² = x³ + ax + b mod p.
    pub b: u64,
    /// Prime modulus.
    pub p: u64,
}
impl EllipticCurvePointCounting {
    /// Create a new elliptic curve E: y² = x³ + ax + b over GF(p).
    pub fn new(a: u64, b: u64, p: u64) -> Self {
        Self { a, b, p }
    }
    /// Check if the curve is non-singular: 4a³ + 27b² ≢ 0 (mod p).
    pub fn is_nonsingular(&self) -> bool {
        let a3 = mod_mul(mod_mul(self.a, self.a, self.p), self.a, self.p);
        let b2 = mod_mul(self.b, self.b, self.p);
        (4 * a3 + 27 * b2) % self.p != 0
    }
    /// Count all affine points plus the point at infinity.
    pub fn count_points(&self) -> u64 {
        let mut count = 1u64;
        for x in 0..self.p {
            let x2 = mod_mul(x, x, self.p);
            let x3 = mod_mul(x2, x, self.p);
            let ax = (self.a % self.p) * x % self.p;
            let rhs = (x3 + ax + self.b) % self.p;
            for y in 0..self.p {
                if mod_mul(y, y, self.p) == rhs {
                    count += 1;
                }
            }
        }
        count
    }
    /// Hasse's theorem: |#E - (p+1)| ≤ 2√p.
    pub fn satisfies_hasse_bound(&self) -> bool {
        let n = self.count_points() as i64;
        let diff = (n - (self.p as i64 + 1)).abs();
        diff as f64 <= 2.0 * (self.p as f64).sqrt()
    }
    /// Trace of Frobenius: t = p + 1 - #E(GF(p)).
    pub fn trace_of_frobenius(&self) -> i64 {
        self.p as i64 + 1 - self.count_points() as i64
    }
}
/// Zero-free region of the Riemann zeta function.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ZeroFreeRegion {
    pub description: String,
    pub implies_pnt_error: String,
}
impl ZeroFreeRegion {
    #[allow(dead_code)]
    pub fn classical() -> Self {
        Self {
            description: "sigma >= 1 - c / log(|t| + 2) for some c > 0".to_string(),
            implies_pnt_error: "pi(x) = Li(x) + O(x * exp(-c' * sqrt(log x)))".to_string(),
        }
    }
    #[allow(dead_code)]
    pub fn riemann_hypothesis() -> Self {
        Self {
            description: "All nontrivial zeros have sigma = 1/2".to_string(),
            implies_pnt_error: "pi(x) = Li(x) + O(sqrt(x) log x)".to_string(),
        }
    }
}
/// Sieve of Eratosthenes as a reusable struct with O(1) primality queries.
#[derive(Debug, Clone)]
pub struct SieveOfEratosthenes {
    pub(super) is_prime_sieve: Vec<bool>,
}
impl SieveOfEratosthenes {
    /// Build the sieve for all integers up to `limit`.
    pub fn new(limit: usize) -> Self {
        let mut sieve = vec![true; limit + 1];
        if limit >= 1 {
            sieve[0] = false;
            sieve[1] = false;
        }
        let mut i = 2;
        while i * i <= limit {
            if sieve[i] {
                let mut j = i * i;
                while j <= limit {
                    sieve[j] = false;
                    j += i;
                }
            }
            i += 1;
        }
        Self {
            is_prime_sieve: sieve,
        }
    }
    /// Returns true iff `n` is prime.
    pub fn is_prime(&self, n: usize) -> bool {
        n < self.is_prime_sieve.len() && self.is_prime_sieve[n]
    }
    /// Return all primes up to the sieve limit.
    pub fn primes(&self) -> Vec<usize> {
        self.is_prime_sieve
            .iter()
            .enumerate()
            .filter_map(|(i, &p)| if p { Some(i) } else { None })
            .collect()
    }
    /// Count primes up to the sieve limit.
    pub fn count_primes(&self) -> usize {
        self.is_prime_sieve.iter().filter(|&&p| p).count()
    }
    /// Return the sieve limit.
    pub fn limit(&self) -> usize {
        self.is_prime_sieve.len().saturating_sub(1)
    }
}
/// Quadratic Sieve — educational sketch of the quadratic sieve factoring idea.
///
/// WARNING: Educational only. Not efficient for large numbers.
#[derive(Debug, Clone)]
pub struct QuadraticSieve {
    /// The number to factor.
    pub n: u64,
    factor_base: Vec<u64>,
}
impl QuadraticSieve {
    /// Create a new quadratic sieve instance for `n`.
    pub fn new(n: u64) -> Self {
        let bound = ((n as f64).sqrt() as usize).min(50).max(10);
        let factor_base = sieve_of_eratosthenes(bound)
            .into_iter()
            .map(|p| p as u64)
            .collect();
        Self { n, factor_base }
    }
    /// Attempt to find a non-trivial factor of n.
    pub fn find_factor(&self) -> Option<u64> {
        if self.n <= 1 || is_prime(self.n) {
            return None;
        }
        for &p in &self.factor_base {
            if p >= 2 && self.n % p == 0 {
                return Some(p);
            }
        }
        let f = pollard_rho(self.n);
        if f > 1 && f < self.n {
            Some(f)
        } else {
            None
        }
    }
    /// Fully factor n into primes (sorted).
    pub fn full_factorization(&self) -> Vec<u64> {
        let mut result = Vec::new();
        let mut remaining = self.n;
        while remaining > 1 {
            if is_prime(remaining) {
                result.push(remaining);
                break;
            }
            let f = pollard_rho(remaining);
            if f == remaining {
                result.push(remaining);
                break;
            }
            result.push(f);
            remaining /= f;
        }
        result.sort_unstable();
        result
    }
}
/// Adele ring of a number field.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct AdeleRing {
    pub field: String,
}
impl AdeleRing {
    #[allow(dead_code)]
    pub fn new(field: &str) -> Self {
        Self {
            field: field.to_string(),
        }
    }
    #[allow(dead_code)]
    pub fn strong_approximation_description(&self) -> String {
        format!(
            "A_K = K * (R x prod_p Z_p): strong approximation for {}",
            self.field
        )
    }
    #[allow(dead_code)]
    pub fn product_formula(&self) -> String {
        "Product formula: prod_v |x|_v = 1 for x in K*".to_string()
    }
}
/// Class field theory (Artin reciprocity).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ClassFieldTheory {
    pub base_field: String,
    pub abelian_extension: String,
}
impl ClassFieldTheory {
    #[allow(dead_code)]
    pub fn new(base: &str, ext: &str) -> Self {
        Self {
            base_field: base.to_string(),
            abelian_extension: ext.to_string(),
        }
    }
    #[allow(dead_code)]
    pub fn artin_reciprocity(&self) -> String {
        format!(
            "Artin map: Gal({}/{}) <-> quotient of idele class group of {}",
            self.abelian_extension, self.base_field, self.base_field
        )
    }
    #[allow(dead_code)]
    pub fn existence_theorem(&self) -> String {
        "Every open subgroup of finite index in id. class group comes from a unique abelian ext"
            .to_string()
    }
}
/// Milnor K-group K_n^M(F) of a field.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct MilnorKGroup {
    pub field: String,
    pub n: usize,
}
impl MilnorKGroup {
    #[allow(dead_code)]
    pub fn new(field: &str, n: usize) -> Self {
        Self {
            field: field.to_string(),
            n,
        }
    }
    #[allow(dead_code)]
    pub fn relations(&self) -> Vec<String> {
        vec![
            format!(
                "Steinberg relation: {{a, 1-a}} = 0 for a != 0, 1 in K^M_2({})",
                self.field
            ),
            format!("K^M_0({}) = Z", self.field),
            format!("K^M_1({}) = {}*", self.field, self.field),
        ]
    }
    #[allow(dead_code)]
    pub fn milnor_conjecture_description(&self) -> String {
        format!(
            "Milnor conjecture (Voevodsky): K^M_n(F)/2 = H^n(F,Z/2) (Galois cohomology) for {}",
            self.field
        )
    }
}
/// Precomputed Möbius function table up to a given limit using a linear sieve.
#[derive(Debug, Clone)]
pub struct MobiusFunctionTable {
    mu_vals: Vec<i8>,
    pub(super) mertens_vals: Vec<i64>,
}
impl MobiusFunctionTable {
    /// Build the table for all k up to `limit`.
    pub fn new(limit: usize) -> Self {
        if limit == 0 {
            return Self {
                mu_vals: vec![0],
                mertens_vals: vec![0],
            };
        }
        let mut mu = vec![0i8; limit + 1];
        let mut is_composite = vec![false; limit + 1];
        let mut primes: Vec<usize> = Vec::new();
        mu[1] = 1;
        for i in 2..=limit {
            if !is_composite[i] {
                primes.push(i);
                mu[i] = -1;
            }
            for &p in &primes {
                let ip = i * p;
                if ip > limit {
                    break;
                }
                is_composite[ip] = true;
                if i % p == 0 {
                    mu[ip] = 0;
                    break;
                }
                mu[ip] = -mu[i];
            }
        }
        let mut mvals = vec![0i64; limit + 1];
        for k in 1..=limit {
            mvals[k] = mvals[k - 1] + mu[k] as i64;
        }
        Self {
            mu_vals: mu,
            mertens_vals: mvals,
        }
    }
    /// Return μ(k).
    pub fn mu(&self, k: usize) -> i64 {
        if k >= self.mu_vals.len() {
            moebius(k as u64)
        } else {
            self.mu_vals[k] as i64
        }
    }
    /// Return M(k) = Σ_{j=1}^{k} μ(j).
    pub fn mertens(&self, k: usize) -> i64 {
        if k >= self.mertens_vals.len() {
            mertens(k as u64)
        } else {
            self.mertens_vals[k]
        }
    }
    /// Return the table limit.
    pub fn size(&self) -> usize {
        self.mu_vals.len().saturating_sub(1)
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SplittingType {
    Split,
    Inert,
    Ramified,
    PartiallyRamified,
}
/// Weil conjecture (proved by Deligne).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct WeilConjectureData {
    pub variety: String,
    pub dimension: usize,
}
impl WeilConjectureData {
    #[allow(dead_code)]
    pub fn new(variety: &str, dim: usize) -> Self {
        Self {
            variety: variety.to_string(),
            dimension: dim,
        }
    }
    #[allow(dead_code)]
    pub fn rationality(&self) -> String {
        format!(
            "Z(X/F_q, T) = P_1(T)*P_3(T)*... / (P_0(T)*P_2(T)*...) rational for {}",
            self.variety
        )
    }
    #[allow(dead_code)]
    pub fn functional_equation(&self) -> String {
        format!(
            "Z(q^n/T) = pm q^(ne/2) T^e Z(T) for {} (dim {})",
            self.variety, self.dimension
        )
    }
    #[allow(dead_code)]
    pub fn riemann_hypothesis(&self) -> String {
        let n = self.dimension;
        format!(
            "Eigenvalues of Frobenius on H^i have |alpha| = q^(i/2) (Deligne) for {} (dim {})",
            self.variety, n
        )
    }
}
/// Sieve methods.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SieveMethod {
    pub name: String,
    pub result: String,
    pub example: String,
}
impl SieveMethod {
    #[allow(dead_code)]
    pub fn sieve_of_eratosthenes(n: u64) -> Vec<u64> {
        let n = n as usize;
        let mut sieve = vec![true; n + 1];
        sieve[0] = false;
        if n >= 1 {
            sieve[1] = false;
        }
        let mut p = 2;
        while p * p <= n {
            if sieve[p] {
                let mut multiple = p * p;
                while multiple <= n {
                    sieve[multiple] = false;
                    multiple += p;
                }
            }
            p += 1;
        }
        sieve
            .iter()
            .enumerate()
            .filter(|(_, &b)| b)
            .map(|(i, _)| i as u64)
            .collect()
    }
    #[allow(dead_code)]
    pub fn brun_sieve() -> Self {
        Self {
            name: "Brun sieve".to_string(),
            result: "Sigma 1/p for twin primes p converges (Brun's constant ~ 1.9022)".to_string(),
            example: "Twin prime count <= C * x / log^2(x)".to_string(),
        }
    }
    #[allow(dead_code)]
    pub fn selberg_sieve() -> Self {
        Self {
            name: "Selberg sieve".to_string(),
            result: "Upper bounds for primes in short intervals".to_string(),
            example: "Goldbach: every even n is sum of at most 3 primes (via circle method)"
                .to_string(),
        }
    }
}
/// Arithmetic progression containing primes (Dirichlet's theorem).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DirichletProgression {
    pub a: u64,
    pub d: u64,
}
impl DirichletProgression {
    #[allow(dead_code)]
    pub fn new(a: u64, d: u64) -> Self {
        Self { a, d }
    }
    #[allow(dead_code)]
    pub fn has_infinitely_many_primes(&self) -> bool {
        gcd_ext(self.a, self.d) == 1
    }
    #[allow(dead_code)]
    pub fn density_among_primes(&self) -> f64 {
        1.0 / euler_phi(self.d) as f64
    }
}
/// Ideal factorization in a number field.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct IdealFactorization {
    pub prime: u64,
    pub factorization: Vec<(String, u32)>,
    pub splitting_type: SplittingType,
}
impl IdealFactorization {
    #[allow(dead_code)]
    pub fn new(p: u64, factors: Vec<(&str, u32)>, split: SplittingType) -> Self {
        Self {
            prime: p,
            factorization: factors
                .into_iter()
                .map(|(s, e)| (s.to_string(), e))
                .collect(),
            splitting_type: split,
        }
    }
    #[allow(dead_code)]
    pub fn is_totally_split(&self) -> bool {
        matches!(self.splitting_type, SplittingType::Split)
    }
}
/// Dirichlet series.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DirichletSeries {
    pub name: String,
    pub abscissa_of_convergence: f64,
}
impl DirichletSeries {
    #[allow(dead_code)]
    pub fn zeta() -> Self {
        Self {
            name: "Riemann zeta sum n^-s".to_string(),
            abscissa_of_convergence: 1.0,
        }
    }
    #[allow(dead_code)]
    pub fn dirichlet_l(chi_name: &str) -> Self {
        Self {
            name: format!("L(s,{chi_name})"),
            abscissa_of_convergence: 0.0,
        }
    }
    #[allow(dead_code)]
    pub fn dedekind_zeta(field: &str) -> Self {
        Self {
            name: format!("zeta_K(s) for K={field}"),
            abscissa_of_convergence: 1.0,
        }
    }
}
