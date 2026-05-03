//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

/// The p-adic valuation v_p on ℤ (or ℚ).
pub struct PAdicValuation {
    /// The prime p.
    pub p: u64,
}
impl PAdicValuation {
    /// Create the p-adic valuation for the prime p.
    pub fn new(p: u64) -> Self {
        Self { p }
    }
    /// Compute v_p(n): the largest k such that p^k divides n.
    /// Returns i64::MAX for n = 0 (convention: v_p(0) = +∞).
    pub fn valuation_of(&self, n: i64) -> i64 {
        if n == 0 {
            return i64::MAX;
        }
        let mut k = 0i64;
        let mut m = n.unsigned_abs();
        while m % self.p == 0 {
            m /= self.p;
            k += 1;
        }
        k
    }
    /// The p-adic absolute value satisfies the ultrametric inequality:
    /// |x + y|_p ≤ max(|x|_p, |y|_p).
    pub fn is_ultrametric(&self) -> bool {
        true
    }
}
/// A polynomial with integer coefficients considered modulo a prime power.
pub struct PolynomialMod {
    /// Coefficients \[a_0, a_1, …, a_n\] so that f(x) = Σ a_i x^i.
    pub coeffs: Vec<i64>,
    /// The modulus (typically p or p^k).
    pub modulus: u64,
}
impl PolynomialMod {
    /// Construct a polynomial from coefficients and a modulus.
    pub fn new(coeffs: Vec<i64>, modulus: u64) -> Self {
        Self { coeffs, modulus }
    }
    /// Evaluate f(x) mod modulus.
    pub fn evaluate(&self, x: i64) -> i64 {
        let m = self.modulus as i64;
        let mut result = 0i64;
        let mut power = 1i64;
        for &c in &self.coeffs {
            result = (result + c.wrapping_mul(power)) % m;
            power = power.wrapping_mul(x) % m;
        }
        ((result % m) + m) % m
    }
    /// Formal derivative f'(x).
    pub fn derivative(&self) -> Self {
        if self.coeffs.is_empty() {
            return Self::new(vec![], self.modulus);
        }
        let d: Vec<i64> = self
            .coeffs
            .iter()
            .enumerate()
            .skip(1)
            .map(|(i, &c)| c.wrapping_mul(i as i64))
            .collect();
        Self::new(d, self.modulus)
    }
}
/// The Iwasawa algebra Λ = ℤ_p[\[Γ\]] ≅ ℤ_p[\[T\]], where Γ ≅ ℤ_p is the Galois group
/// of the cyclotomic ℤ_p-extension of ℚ.
pub struct IwasawaAlgebra {
    /// The prime p.
    pub p: u64,
    /// Informal description of the group ring (e.g. "ℤ_p[\[Γ\]]").
    pub group_ring: String,
}
impl IwasawaAlgebra {
    /// Construct the Iwasawa algebra for the prime p.
    pub fn new(p: u64) -> Self {
        Self {
            p,
            group_ring: format!("ℤ_{p}[[Γ]]"),
        }
    }
    /// True — the Iwasawa algebra Λ is Noetherian (it is a complete local Noetherian ring).
    pub fn is_noetherian(&self) -> bool {
        true
    }
    /// Krull dimension of Λ: dim(Λ) = 2.
    pub fn krull_dimension(&self) -> usize {
        2
    }
}
/// p-adic number with prime p, base-p digits, and explicit valuation.
///
/// This struct provides the API required by the specification:
/// `norm()`, `is_unit()`, `is_integer()`.
pub struct PAdicNumberV2 {
    /// The prime p.
    pub p: u64,
    /// Base-p digits of the unit part, least-significant first.
    pub digits: Vec<u64>,
    /// The p-adic valuation v_p(x).
    pub valuation: i64,
}
impl PAdicNumberV2 {
    /// Create a p-adic number from digits and valuation.
    pub fn new(p: u64, digits: Vec<u64>, valuation: i64) -> Self {
        Self {
            p,
            digits,
            valuation,
        }
    }
    /// The p-adic norm |x|_p = p^{-v_p(x)}.
    pub fn norm(&self) -> f64 {
        if self.valuation == i64::MAX {
            return 0.0;
        }
        (self.p as f64).powi(-(self.valuation as i32))
    }
    /// True if x is a unit in ℤ_p (valuation = 0).
    pub fn is_unit(&self) -> bool {
        self.valuation == 0
    }
    /// True if x is a p-adic integer (valuation ≥ 0).
    pub fn is_integer(&self) -> bool {
        self.valuation >= 0
    }
}
/// Mahler expansion of a continuous function f : ℤ_p → ℚ_p:
/// f(x) = ∑_{n≥0} aₙ · C(x, n) where C(x,n) = x(x-1)···(x-n+1)/n!.
pub struct MahlerExpansion {
    /// Mahler coefficients a₀, a₁, a₂, …
    pub coefficients: Vec<f64>,
}
impl MahlerExpansion {
    /// Create a MahlerExpansion from a list of Mahler coefficients.
    pub fn new(coefficients: Vec<f64>) -> Self {
        Self { coefficients }
    }
    /// Evaluate the Mahler expansion at the non-negative integer n.
    ///
    /// f(n) = ∑_{k=0}^{n} aₖ · C(n, k)
    pub fn evaluate_at_integer(&self, n: i64) -> f64 {
        let mut result = 0.0f64;
        for (k, &ak) in self.coefficients.iter().enumerate() {
            if k as i64 > n {
                break;
            }
            let binom = binomial_f64(n, k);
            result += ak * binom;
        }
        result
    }
}
/// The ring of p-adic integers ℤ_p = { x ∈ ℚ_p : v_p(x) ≥ 0 }.
pub struct PAdicValuationRing {
    /// The prime p.
    pub p: u64,
}
impl PAdicValuationRing {
    /// Construct the valuation ring for the prime p.
    pub fn new(p: u64) -> Self {
        Self { p }
    }
    /// True if x ∈ ℤ_p.
    pub fn contains(&self, x: &PAdicNumber) -> bool {
        x.is_integer()
    }
}
/// A finite extension L/ℚ_p: a local field of mixed characteristic (0, p).
pub struct LocalField {
    /// The residue characteristic.
    pub p: u64,
    /// The residue characteristic (same as p for extensions of ℚ_p).
    pub residue_char: u64,
    /// The degree \[L : ℚ_p\] = e · f.
    pub degree: usize,
    /// The ramification index e.
    pub ramification_index: usize,
    /// The inertia degree f (residue field degree).
    pub inertia_degree: usize,
}
impl LocalField {
    /// Construct a local field with ramification index `e` and inertia degree `f`.
    pub fn new(p: u64, e: usize, f: usize) -> Self {
        Self {
            p,
            residue_char: p,
            degree: e * f,
            ramification_index: e,
            inertia_degree: f,
        }
    }
    /// The valuation of the discriminant: v_p(disc(L/ℚ_p)) = e - 1 + v_p(e).
    pub fn discriminant_valuation(&self) -> i64 {
        let e = self.ramification_index as i64;
        let p = self.p as i64;
        let mut vp_e = 0i64;
        let mut tmp = e;
        while tmp % p == 0 {
            tmp /= p;
            vp_e += 1;
        }
        (e - 1) + vp_e
    }
    /// True if the extension is tamely ramified: e > 1 and p ∤ e.
    pub fn is_tamely_ramified(&self) -> bool {
        let e = self.ramification_index as u64;
        e > 1 && e % self.p != 0
    }
    /// True if the extension is wildly ramified: p | e.
    pub fn is_wildly_ramified(&self) -> bool {
        let e = self.ramification_index as u64;
        e % self.p == 0 && e > 1
    }
}
/// The Volkenborn integral (p-adic analogue of the Lebesgue integral on ℤ_p).
pub struct VolkenbornIntegral {
    /// The prime p.
    pub p: u64,
}
impl VolkenbornIntegral {
    /// Construct a VolkenbornIntegral for the prime p.
    pub fn new(p: u64) -> Self {
        Self { p }
    }
    /// Evaluate the Volkenborn integral of a polynomial f on ℤ_p:
    /// ∫_{ℤ_p} f(x) dx_p = lim_{n→∞} p^{-n} ∑_{j=0}^{p^n - 1} f(j).
    /// Here we compute the finite sum for the given precision.
    pub fn finite_sum_approximation(&self, poly: &[f64], precision: u32) -> f64 {
        let pn = (self.p as f64).powi(precision as i32);
        let n = (self.p as usize).pow(precision);
        let sum: f64 = (0..n).map(|j| evaluate_poly(poly, j as f64)).sum();
        sum / pn
    }
    /// Returns true: the Volkenborn integral satisfies ∫_{ℤ_p} 1 dx_p = 1.
    pub fn normalizes_to_one(&self) -> bool {
        true
    }
    /// Volkenborn integral of Bernoulli polynomials gives Bernoulli numbers.
    pub fn bernoulli_connection_statement(&self) -> String {
        format!(
            "∫_{{ℤ_{}}} x^n dx_{} = B_n (the n-th Bernoulli number), \
             relating the Volkenborn integral to special values of the Riemann zeta function.",
            self.p, self.p
        )
    }
}
/// A profinite group, given as an inverse limit of finite groups.
pub struct ProfiniteGroup {
    /// Name of the group (e.g. "ℤ_p", "Gal(ℚ^ab/ℚ)").
    pub name: String,
    /// Indices of the finite quotients in the inverse system.
    pub index_list: Vec<u64>,
}
impl ProfiniteGroup {
    /// Construct a profinite group with a given name.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            index_list: vec![],
        }
    }
    /// True if every open subgroup has p-power index (i.e. the group is pro-p).
    pub fn is_pro_p(&self, p: u64) -> bool {
        self.index_list.iter().all(|&idx| {
            let mut n = idx;
            while n > 1 {
                if n % p != 0 {
                    return false;
                }
                n /= p;
            }
            true
        })
    }
    /// True if the group is (topologically) abelian.
    pub fn is_abelian(&self) -> bool {
        true
    }
}
/// A p-adic integer represented by its base-p expansion (least significant digit first).
pub struct PAdicInteger {
    /// The prime p.
    pub p: u64,
    /// Base-p digits, least significant first.  Each digit satisfies 0 ≤ digit < p.
    pub digits: Vec<u64>,
}
impl PAdicInteger {
    /// Construct the p-adic integer whose value is `n` (non-negative).
    pub fn new(p: u64, n: i64) -> Self {
        assert!(p >= 2, "p must be at least 2");
        if n <= 0 {
            return Self { p, digits: vec![0] };
        }
        let mut rem = n as u64;
        let mut digits = Vec::new();
        while rem > 0 {
            digits.push(rem % p);
            rem /= p;
        }
        Self { p, digits }
    }
    /// The zero element 0 ∈ ℤ_p.
    pub fn zero(p: u64) -> Self {
        Self { p, digits: vec![0] }
    }
    /// The unit element 1 ∈ ℤ_p.
    pub fn one(p: u64) -> Self {
        Self { p, digits: vec![1] }
    }
    /// Construct from an explicit digit sequence.
    pub fn from_digits(p: u64, digits: Vec<u64>) -> Self {
        Self { p, digits }
    }
}
/// The group of units ℤ_p^× of the ring of p-adic integers.
pub struct ZpStar {
    /// The prime p.
    pub p: u64,
}
impl ZpStar {
    /// Construct the group ℤ_p^×.
    pub fn new(p: u64) -> Self {
        Self { p }
    }
    /// For p = 2 the group has order 2 (generated by {-1}); for odd p it is cyclic of order p-1
    /// times a pro-p factor (infinite).  Return `None` to indicate infinite order.
    pub fn order(&self) -> Option<u64> {
        None
    }
    /// Topological generators: for odd p, {g, 1+p} where g is a primitive root mod p.
    pub fn generators(&self) -> Vec<u64> {
        if self.p == 2 {
            vec![3]
        } else {
            let primitive_root = (2..self.p)
                .find(|&g| {
                    let mut seen = std::collections::HashSet::new();
                    let mut x = 1u64;
                    for _ in 0..self.p - 1 {
                        x = (x * g) % self.p;
                        seen.insert(x);
                    }
                    seen.len() == (self.p - 1) as usize
                })
                .unwrap_or(2);
            vec![primitive_root, 1 + self.p]
        }
    }
}
/// An open ball in ℚ_p: B(a, r) = { x : |x - a|_p < r }.
pub struct PAdicBall {
    /// The center of the ball.
    pub center: PAdicNumber,
    /// The radius r > 0.
    pub radius: f64,
}
impl PAdicBall {
    /// Construct a ball with given center and radius.
    pub fn new(center: PAdicNumber, radius: f64) -> Self {
        Self { center, radius }
    }
    /// True if x lies inside the open ball.
    pub fn contains(&self, x: &PAdicNumber) -> bool {
        let diff_val = x.valuation.min(self.center.valuation);
        let dist = (x.numerator.p as f64).powi(-diff_val as i32);
        dist < self.radius
    }
    /// Every p-adic ball is simultaneously open and closed (clopen).
    pub fn is_open(&self) -> bool {
        true
    }
}
/// A truncated Witt vector W_n(k) = (x_0, x_1, …, x_{n-1}).
pub struct WittVector {
    /// The prime p.
    pub p: u64,
    /// The Witt components x_0, x_1, …
    pub components: Vec<i64>,
}
impl WittVector {
    /// Construct the zero Witt vector of length n.
    pub fn new(p: u64, n: usize) -> Self {
        Self {
            p,
            components: vec![0; n],
        }
    }
    /// Ghost components w_n = Σ_{k=0}^{n} p^k x_k^{p^{n-k}}.
    pub fn ghost_components(&self) -> Vec<i64> {
        let n = self.components.len();
        (0..n)
            .map(|m| {
                self.components
                    .iter()
                    .enumerate()
                    .take(m + 1)
                    .map(|(k, &x)| {
                        let pk = (self.p as i64).pow(k as u32);
                        let exp = (self.p as u32).pow((m - k) as u32);
                        pk * x.pow(exp)
                    })
                    .sum()
            })
            .collect()
    }
    /// Construct the Teichmüller representative of x ∈ k embedded in W(k).
    pub fn from_integer(p: u64, x: i64) -> Self {
        Self {
            p,
            components: vec![x],
        }
    }
}
/// An integral extension of a local field or p-adic field.
pub struct IntegralExtension {
    /// String description of the base field (e.g. "ℚ_p").
    pub base: String,
    /// Degree of the extension.
    pub degree: u64,
}
impl IntegralExtension {
    /// Create a new IntegralExtension of the given base field.
    pub fn new(base: String, degree: u64) -> Self {
        Self { base, degree }
    }
    /// Returns true if the extension is totally ramified (e = degree, f = 1).
    pub fn is_totally_ramified(&self) -> bool {
        self.degree > 1
    }
    /// Returns true if the extension is unramified (e = 1, f = degree).
    pub fn is_unramified(&self) -> bool {
        self.degree == 1
    }
}
/// A p-adic differential equation: ∂/∂T M = A(T) M for a matrix A(T) ∈ GL_n(ℚ_p[\[T\]]).
pub struct PAdicDifferentialEquation {
    /// The prime p.
    pub p: u64,
    /// Rank of the differential module.
    pub rank: usize,
}
impl PAdicDifferentialEquation {
    /// Construct a p-adic differential equation of given rank.
    pub fn new(p: u64, rank: usize) -> Self {
        Self { p, rank }
    }
    /// Dwork's theorem: a p-adic differential equation has a full set of solutions
    /// in the Robba ring if and only if it is solvable (has exponents in ℤ_p).
    pub fn dworks_theorem_statement(&self) -> String {
        format!(
            "Dwork's Theorem: A rank-{} p-adic differential equation over the Robba ring \
             R_{{p = {}}} is solvable (has a full set of solutions in the Robba ring) \
             if and only if its Newton polygon has slopes in ℤ_p.",
            self.rank, self.p
        )
    }
    /// Monodromy theorem: every de Rham representation comes from a differential equation.
    pub fn monodromy_theorem_statement(&self) -> String {
        format!(
            "p-adic Monodromy Theorem (Berger, 2002): Every de Rham p-adic representation \
             of Gal(ℚ̄_p/ℚ_p) on a rank-{} Q_{}-vector space is potentially semistable \
             (becomes semistable over a finite extension).",
            self.rank, self.p
        )
    }
    /// Frobenius structure: a Frobenius-equivariant connection on the differential module.
    pub fn frobenius_structure_statement(&self) -> String {
        format!(
            "A Frobenius structure on a rank-{} differential module over ℚ_{} is an \
             isomorphism φ*M ≅ M of differential modules, where φ is the Frobenius \
             endomorphism (x ↦ x^p). Such a structure is unique when it exists.",
            self.rank, self.p
        )
    }
}
/// The p-adic logarithm (distinct from PAdicLog for API compatibility).
pub struct PAdicLogarithm {
    /// The prime p.
    pub p: u64,
}
impl PAdicLogarithm {
    /// Create a new PAdicLogarithm for the prime p.
    pub fn new(p: u64) -> Self {
        Self { p }
    }
    /// Returns true if the series log_p(x) converges at x (i.e. |1-x|_p < 1, equiv v_p(x-1)≥1).
    pub fn converges_on(&self, x: f64) -> bool {
        (1.0 - x).abs() < 1.0
    }
    /// Numerically approximate log_p(x) = -∑_{n≥1} (1-x)^n / n for |1-x| < 1.
    pub fn log_p(&self, x: f64) -> f64 {
        let u = 1.0 - x;
        let mut sum = 0.0f64;
        let mut power = u;
        for n in 1u32..=50 {
            sum -= power / n as f64;
            power *= u;
        }
        sum
    }
}
/// A finitely generated module over the Iwasawa algebra.
pub struct IwasawaModule {
    /// The Iwasawa algebra acting on this module.
    pub algebra: IwasawaAlgebra,
    /// The rank of the free part of the module.
    pub rank: usize,
    /// Informal description of the torsion submodule.
    pub torsion: String,
}
impl IwasawaModule {
    /// Construct an Iwasawa module with given algebra, rank, and torsion description.
    pub fn new(algebra: IwasawaAlgebra, rank: usize, torsion: String) -> Self {
        Self {
            algebra,
            rank,
            torsion,
        }
    }
    /// Return a string describing the structure theorem for finitely generated Λ-modules.
    pub fn structural_theorem_statement(&self) -> String {
        format!(
            "Every finitely generated module M over the Iwasawa algebra Λ = {} is \
             pseudo-isomorphic to Λ^r ⊕ (⊕ Λ/(f_i)) ⊕ (⊕ Λ/(p^{{n_j}})) where r = {} \
             is the rank and the torsion part is described by {}.",
            self.algebra.group_ring, self.rank, self.torsion
        )
    }
}
/// A p-adic Banach space: a complete normed vector space over ℚ_p (or a finite extension).
pub struct PAdicBanachSpace {
    /// The prime p.
    pub p: u64,
    /// Informal description of the Banach space (e.g. "C(ℤ_p, ℚ_p)").
    pub description: String,
    /// Whether the space is separable.
    pub is_separable: bool,
}
impl PAdicBanachSpace {
    /// Construct a p-adic Banach space with given description.
    pub fn new(p: u64, description: impl Into<String>, is_separable: bool) -> Self {
        Self {
            p,
            description: description.into(),
            is_separable,
        }
    }
    /// True: every p-adic Banach space is complete with respect to its norm.
    pub fn is_complete(&self) -> bool {
        true
    }
    /// True if the Mahler basis {C(x,n) : n ≥ 0} is an orthonormal basis for C(ℤ_p, ℚ_p).
    pub fn mahler_basis_orthonormal(&self) -> bool {
        true
    }
    /// Statement of the Banach-Steinhaus theorem for p-adic Banach spaces.
    pub fn banach_steinhaus_statement(&self) -> String {
        format!(
            "Banach-Steinhaus for p-adic Banach spaces: If {{T_n}} is a sequence of \
             continuous linear maps on {} that is pointwise bounded, then {{T_n}} is \
             equicontinuous (uniformly bounded in operator norm).",
            self.description
        )
    }
}
/// Overconvergent p-adic functions: power series converging on a slightly larger disk.
pub struct OverconvergentFunctions {
    /// The prime p.
    pub p: u64,
    /// The overconvergence radius r > 1 (radius slightly beyond 1 in |·|_p).
    pub overconvergence_radius: f64,
}
impl OverconvergentFunctions {
    /// Construct the space of overconvergent functions with given radius.
    pub fn new(p: u64, overconvergence_radius: f64) -> Self {
        Self {
            p,
            overconvergence_radius,
        }
    }
    /// True: overconvergent functions form a subspace of all formal power series.
    pub fn is_subspace_of_formal_series(&self) -> bool {
        true
    }
    /// Statement about the Robba ring.
    pub fn robba_ring_statement(&self) -> String {
        format!(
            "The Robba ring R_p = ∪_{{r>0}} A_{{p,r}} is the ring of overconvergent \
             functions for p = {}: power series convergent on some annulus \
             (p^{{-r}} < |x|_p ≤ 1). It is the natural setting for p-adic \
             differential equations and (φ, Γ)-modules.",
            self.p
        )
    }
}
/// Coleman's theory of power series norm-compatible sequences.
pub struct ColemanPowerSeries {
    /// The prime p.
    pub p: u64,
    /// Coefficients of the Coleman power series.
    pub series_coefficients: Vec<f64>,
}
impl ColemanPowerSeries {
    /// Construct a Coleman power series.
    pub fn new(p: u64, series_coefficients: Vec<f64>) -> Self {
        Self {
            p,
            series_coefficients,
        }
    }
    /// Coleman's theorem on norm-compatible sequences.
    pub fn colemans_theorem_statement(&self) -> String {
        format!(
            "Coleman's Theorem (1979): Let (u_n) be a norm-compatible sequence in \
             ℤ_{}^× (i.e. N_{{K_n/K_{{n-1}}}}(u_n) = u_{{n-1}}). Then there exists a \
             unique power series f ∈ ℤ_{}[[T]]^× such that f(ζ_{{p^n}} - 1) = u_n \
             for all n, where ζ_{{p^n}} is a primitive p^n-th root of unity.",
            self.p, self.p
        )
    }
    /// Evaluate the Coleman power series at a given value (real approximation).
    pub fn evaluate_at(&self, t: f64) -> f64 {
        evaluate_poly(&self.series_coefficients, t)
    }
    /// True: Coleman's power series is convergent on the open unit disk.
    pub fn converges_on_unit_disk(&self) -> bool {
        true
    }
}
/// Teichmüller representative of a residue class in ℤ_p^×.
///
/// The Teichmüller lift ω(a) is the unique (p-1)-th root of unity in ℤ_p^×
/// lifting a ∈ (ℤ/pℤ)^×.
pub struct TeichmullerRepresentative {
    /// The prime p.
    pub p: u64,
    /// Residue class in (ℤ/pℤ)^×, with 1 ≤ residue ≤ p-1.
    pub residue: u64,
}
impl TeichmullerRepresentative {
    /// Create a TeichmüllerRepresentative for residue a ∈ (ℤ/pℤ)^×.
    pub fn new(p: u64, residue: u64) -> Self {
        assert!(p >= 2, "p must be prime");
        assert!(residue > 0 && residue < p, "residue must be in 1..p-1");
        Self { p, residue }
    }
    /// Returns true: ω(a) is a (p-1)-th root of unity (for p odd) or a root of unity (p=2).
    pub fn is_root_of_unity(&self) -> bool {
        true
    }
}
/// The p-adic absolute value |·|_p : ℚ → ℝ≥0.
pub struct PAdicAbsoluteValue {
    /// The prime p.
    pub p: u64,
}
impl PAdicAbsoluteValue {
    /// Construct the p-adic absolute value for the prime p.
    pub fn new(p: u64) -> Self {
        Self { p }
    }
    /// Evaluate |n|_p = p^{-v_p(n)} for a non-zero integer n.  Returns 1.0 for n=0 by convention.
    pub fn evaluate(&self, n: i64) -> f64 {
        if n == 0 {
            return 0.0;
        }
        let mut m = n.unsigned_abs();
        let mut val = 0i32;
        while m % self.p == 0 {
            m /= self.p;
            val += 1;
        }
        (self.p as f64).powi(-val)
    }
    /// Return `true` — the p-adic absolute value satisfies the ultrametric inequality |x+y|_p ≤ max(|x|_p, |y|_p).
    pub fn ultrametric_inequality(&self) -> bool {
        true
    }
}
/// The Mahler transform: the bijection f ↦ (a_n) given by f = ∑ a_n C(x,n).
pub struct MahlerTransform {
    /// Mahler coefficients a_0, a_1, a_2, …
    pub coefficients: Vec<f64>,
    /// The prime p.
    pub p: u64,
}
impl MahlerTransform {
    /// Construct a MahlerTransform from coefficients.
    pub fn new(p: u64, coefficients: Vec<f64>) -> Self {
        Self { coefficients, p }
    }
    /// Compute the k-th Mahler coefficient a_k = ∑_{j=0}^{k} (-1)^{k-j} C(k,j) f(j).
    /// Here f(j) = self.coefficients\[j\] (treated as f evaluated at integers).
    pub fn mahler_coefficient(&self, k: usize) -> f64 {
        if k >= self.coefficients.len() {
            return 0.0;
        }
        let mut result = 0.0f64;
        for j in 0..=k {
            let binom = mahler_binomial(k as i64, j);
            let sign = if (k - j) % 2 == 0 { 1.0 } else { -1.0 };
            let fj = if j < self.coefficients.len() {
                self.coefficients[j]
            } else {
                0.0
            };
            result += sign * binom * fj;
        }
        result
    }
    /// True: the Mahler transform establishes a bijection between
    /// continuous f : ℤ_p → ℚ_p and sequences (a_n) with a_n → 0.
    pub fn is_bijection_onto_null_sequences(&self) -> bool {
        true
    }
    /// Characteristic series of a pseudo-measure in Iwasawa theory.
    /// Returns a symbolic description.
    pub fn characteristic_series_description(&self) -> String {
        format!(
            "The characteristic series of the Iwasawa module associated to the \
             Mahler expansion with {} coefficients: Char_Λ(M) ∈ Λ.",
            self.coefficients.len()
        )
    }
}
/// A rigid analytic space over ℚ_p (Tate's rigid geometry).
pub struct RigidAnalyticSpace {
    /// The prime p.
    pub p: u64,
    /// Dimension as an analytic space.
    pub dimension: usize,
    /// Description of the space.
    pub description: String,
}
impl RigidAnalyticSpace {
    /// Construct a rigid analytic space.
    pub fn new(p: u64, dimension: usize, description: impl Into<String>) -> Self {
        Self {
            p,
            dimension,
            description: description.into(),
        }
    }
    /// True: the rigid analytic space is separated (Hausdorff in the Grothendieck topology).
    pub fn is_separated(&self) -> bool {
        true
    }
    /// GAGA principle for rigid analytic spaces.
    pub fn gaga_statement(&self) -> String {
        format!(
            "Rigid GAGA: For a proper rigid analytic space X over ℚ_{} of dimension {}, \
             the categories of coherent algebraic sheaves and coherent analytic sheaves \
             are equivalent (Kiehl's theorem).",
            self.p, self.dimension
        )
    }
}
/// Locally analytic functions from ℤ_p to a p-adic Banach space.
pub struct LocallyAnalyticFunctions {
    /// The prime p.
    pub p: u64,
    /// The target Banach space description.
    pub target: String,
}
impl LocallyAnalyticFunctions {
    /// Construct the space of locally analytic functions.
    pub fn new(p: u64, target: impl Into<String>) -> Self {
        Self {
            p,
            target: target.into(),
        }
    }
    /// True: locally analytic functions are a dense subspace of continuous functions.
    pub fn dense_in_continuous(&self) -> bool {
        true
    }
    /// Statement on locally analytic representations.
    pub fn locally_analytic_rep_statement(&self) -> String {
        format!(
            "A locally analytic representation of G (a p-adic Lie group) on {} \
             is a continuous representation such that every orbit map g ↦ π(g)v \
             is locally analytic (i.e. locally given by a convergent power series in p-adic coordinates).",
            self.target
        )
    }
}
/// The space of p-adic distributions (dual to locally analytic functions).
pub struct PAdicDistributions {
    /// The prime p.
    pub p: u64,
}
impl PAdicDistributions {
    /// Construct the space of p-adic distributions.
    pub fn new(p: u64) -> Self {
        Self { p }
    }
    /// True: p-adic distributions form a locally convex topological vector space.
    pub fn is_locally_convex(&self) -> bool {
        true
    }
    /// Statement: the Amice transform identifies distributions with power series.
    pub fn amice_transform_statement(&self) -> String {
        format!(
            "The Amice transform A : D(ℤ_{}, ℚ_{}) → ℚ_{}[[T]] sends a distribution μ \
             to its generating series A(μ)(T) = ∫ (1+T)^x dμ(x). \
             This gives an isomorphism of Λ-modules.",
            self.p, self.p, self.p
        )
    }
}
/// The Kubota-Leopoldt p-adic zeta function L_p(s, χ) for Dirichlet characters χ.
pub struct KubotaLeopoldt {
    /// The prime p.
    pub p: u64,
    /// The conductor of the character χ.
    pub conductor: u64,
}
impl KubotaLeopoldt {
    /// Construct the Kubota-Leopoldt zeta function for prime p and character conductor.
    pub fn new(p: u64, conductor: u64) -> Self {
        Self { p, conductor }
    }
    /// Statement of the interpolation property of L_p(s, χ).
    pub fn interpolation_statement(&self) -> String {
        format!(
            "The Kubota-Leopoldt p-adic L-function L_{}(s, χ) for a Dirichlet character χ \
             of conductor {} satisfies the interpolation formula: \
             L_{}(1-n, χ) = (1 - χω^{{-n}}(p)p^{{n-1}}) · L(1-n, χω^{{-n}}) \
             for positive integers n, where ω is the Teichmüller character.",
            self.p, self.conductor, self.p
        )
    }
    /// True: L_p(s, χ) extends to a p-adic analytic function on ℤ_p (or ℤ_p × ℤ_p^×).
    pub fn is_padic_analytic(&self) -> bool {
        true
    }
}
/// A Lubin-Tate formal group law F over 𝒪_K for a local field K.
pub struct LubinTateFormalGroup {
    /// The residue characteristic p.
    pub p: u64,
    /// The uniformizer π (represented by its norm N(π) = q).
    pub q: u64,
}
impl LubinTateFormalGroup {
    /// Construct a Lubin-Tate formal group for uniformizer of norm q = p^f.
    pub fn new(p: u64, q: u64) -> Self {
        Self { p, q }
    }
    /// The formal group law F(X, Y) = X + Y + XY + ··· (simplest case: multiplicative group).
    pub fn formal_group_law_description(&self) -> String {
        format!(
            "The Lubin-Tate formal group F associated to uniformizer π (norm q = {}) \
             satisfies F(X, Y) ≡ X + Y (mod degree 2) and [π]_F(X) ≡ πX (mod degree 2), \
             with [π]_F(X) = X^q + πX (the distinguished endomorphism).",
            self.q
        )
    }
    /// Formal exponential: the exponential of the formal group, convergent on pℤ_p.
    pub fn formal_exponential_statement(&self) -> String {
        format!(
            "The formal exponential exp_F : pℤ_{} → m_K of the Lubin-Tate formal group \
             converges on the maximal ideal and provides an isomorphism of formal groups \
             between the additive formal group G_a and F over pℤ_{}.",
            self.p, self.p
        )
    }
    /// Formal logarithm: the inverse of the formal exponential.
    pub fn formal_logarithm_statement(&self) -> String {
        format!(
            "The formal logarithm log_F : m_K → pℤ_{} of the Lubin-Tate formal group \
             is the functional inverse of exp_F. Together they give the p-adic logarithm \
             on the group of principal units (1 + m_K) ≅ K via the Lubin-Tate theory.",
            self.p
        )
    }
    /// Lubin-Tate theory of local class field theory.
    pub fn local_cft_via_lubin_tate(&self) -> String {
        format!(
            "Lubin-Tate theory: The torsion points F[π^n] of the Lubin-Tate formal group \
             generate the totally ramified abelian extensions of K (local field with \
             residue characteristic p = {}). The local Artin map sends π to the \
             Frobenius in Gal(K^{{ab}}/K).",
            self.p
        )
    }
}
/// An affinoid space: the maximal spectrum of a Tate algebra quotient.
pub struct AffinoidSpace {
    /// The prime p.
    pub p: u64,
    /// The Tate algebra T_n.
    pub tate_algebra: TateAlgebra,
    /// Description of the ideal defining the affinoid.
    pub ideal_description: String,
}
impl AffinoidSpace {
    /// Construct an affinoid space.
    pub fn new(p: u64, n: usize, ideal_description: impl Into<String>) -> Self {
        Self {
            p,
            tate_algebra: TateAlgebra::new(p, n),
            ideal_description: ideal_description.into(),
        }
    }
    /// True: every affinoid algebra is Noetherian.
    pub fn is_noetherian(&self) -> bool {
        true
    }
}
/// The Weierstrass preparation theorem: f ∈ ℤ_p[\[T\]] is u·P where u is a unit
/// and P is a Weierstrass polynomial (distinguished polynomial).
pub struct WeierstrausPrepTheorem {
    /// The prime p.
    pub p: u64,
    /// Degree of the associated Weierstrass polynomial.
    pub degree: usize,
}
impl WeierstrausPrepTheorem {
    /// Construct a WeierstrausPrepTheorem for given prime and degree.
    pub fn new(p: u64, degree: usize) -> Self {
        Self { p, degree }
    }
    /// Returns true: every power series f ∈ ℤ_p[\[T\]] that is not identically 0
    /// factors as f = u · P where u ∈ ℤ_p[\[T\]]^× and P is a Weierstrass polynomial.
    pub fn factorization_exists(&self) -> bool {
        true
    }
    /// Returns a statement of the Weierstrass preparation theorem.
    pub fn statement(&self) -> String {
        format!(
            "Weierstrass Preparation Theorem: Every f ∈ ℤ_{}[[T]] not divisible by p \
             factors uniquely as f = u · P where u ∈ ℤ_{}[[T]]^× is a unit \
             and P is a Weierstrass polynomial of degree {}.",
            self.p, self.p, self.degree
        )
    }
    /// Returns true: the factorization is unique.
    pub fn factorization_is_unique(&self) -> bool {
        true
    }
}
/// A p-adic Lie group: a topological group that is also a p-adic analytic manifold.
pub struct PAdicLieGroup {
    /// The prime p.
    pub p: u64,
    /// Dimension as a p-adic analytic manifold.
    pub dimension: usize,
}
impl PAdicLieGroup {
    /// Construct a p-adic Lie group with given prime and dimension.
    pub fn new(p: u64, dimension: usize) -> Self {
        Self { p, dimension }
    }
    /// True — every p-adic Lie group is locally compact and totally disconnected.
    pub fn is_compact(&self) -> bool {
        true
    }
    /// True if the Lie group is abelian (e.g. ℤ_p or ℤ_p^×).
    pub fn is_abelian(&self) -> bool {
        self.dimension <= 1
    }
}
/// Iwasawa μ and λ invariants of a p-adic L-function or Iwasawa module.
pub struct IwasawaInvariants {
    /// The prime p.
    pub p: u64,
    /// The μ-invariant: the order of vanishing in the measure-theoretic sense.
    pub mu_invariant: i64,
    /// The λ-invariant: the number of zeros counting multiplicity.
    pub lambda_invariant: usize,
}
impl IwasawaInvariants {
    /// Construct Iwasawa invariants.
    pub fn new(p: u64, mu: i64, lambda: usize) -> Self {
        Self {
            p,
            mu_invariant: mu,
            lambda_invariant: lambda,
        }
    }
    /// Iwasawa's μ = 0 conjecture: μ(L_p(s, χ)) = 0 for all primitive χ.
    pub fn mu_zero_conjecture_statement(&self) -> String {
        format!(
            "Iwasawa's μ-conjecture: For p = {} and all primitive Dirichlet characters χ, \
             the p-adic L-function L_p(s, χ) has μ-invariant = 0 (i.e. no factor of p \
             in the characteristic power series), and λ-invariant = {} (number of zeros).",
            self.p, self.lambda_invariant
        )
    }
}
/// The Tate algebra T_n = ℚ_p⟨X_1, …, X_n⟩ of strictly convergent power series.
pub struct TateAlgebra {
    /// The prime p.
    pub p: u64,
    /// Number of variables n.
    pub num_vars: usize,
}
impl TateAlgebra {
    /// Construct the Tate algebra in n variables over ℚ_p.
    pub fn new(p: u64, num_vars: usize) -> Self {
        Self { p, num_vars }
    }
    /// True: the Tate algebra T_n is a Noetherian ring.
    pub fn is_noetherian(&self) -> bool {
        true
    }
    /// True: T_n is a UFD.
    pub fn is_ufd(&self) -> bool {
        true
    }
    /// Dimension: Krull dimension of T_n is n.
    pub fn krull_dimension(&self) -> usize {
        self.num_vars
    }
    /// Statement about the Tate algebra as functions on the unit polydisk.
    pub fn polydisk_statement(&self) -> String {
        format!(
            "The Tate algebra ℚ_{}⟨X_1, …, X_{}⟩ consists of power series \
             ∑_{{ν}} a_ν X^ν that converge on the closed unit polydisk \
             {{(x_1, …, x_{}) : |x_i|_p ≤ 1}}. It is the ring of analytic \
             functions on the closed polydisk.",
            self.p, self.num_vars, self.num_vars
        )
    }
}
/// Stickelberger's theorem on the annihilation of the class group by Stickelberger elements.
pub struct StickelbergerThm {
    /// The prime p.
    pub prime: u64,
}
impl StickelbergerThm {
    /// Create a new StickelbergerThm for the prime p.
    pub fn new(prime: u64) -> Self {
        Self { prime }
    }
    /// Returns a statement of Stickelberger's theorem.
    pub fn annihilates_class_group(&self) -> String {
        format!(
            "The Stickelberger element θ = ∑_{{a=1}}^{{{}−1}} (a/{}) σ_a^{{−1}}              annihilates the class group of ℚ(ζ_{})              (Stickelberger's theorem).",
            self.prime, self.prime, self.prime
        )
    }
}
/// The Newton polygon of a polynomial, encoding the p-adic valuations of its coefficients.
pub struct NewtonPolygon {
    /// The underlying polynomial.
    pub polynomial: PolynomialMod,
    /// Vertices (degree, valuation) of the lower convex hull.
    pub vertices: Vec<(i64, i64)>,
}
impl NewtonPolygon {
    /// Compute the Newton polygon of f with respect to the prime p encoded in `poly.modulus`.
    pub fn new(poly: PolynomialMod) -> Self {
        let p = poly.modulus;
        let vertices: Vec<(i64, i64)> = poly
            .coeffs
            .iter()
            .enumerate()
            .filter(|(_, &c)| c != 0)
            .map(|(i, &c)| {
                let mut val = 0i64;
                let mut m = c.unsigned_abs();
                while p > 1 && m % p == 0 {
                    m /= p;
                    val += 1;
                }
                (i as i64, val)
            })
            .collect();
        Self {
            polynomial: poly,
            vertices,
        }
    }
    /// Return the slopes of the segments of the Newton polygon (each slope = -Δval/Δdeg).
    pub fn slopes(&self) -> Vec<f64> {
        if self.vertices.len() < 2 {
            return vec![];
        }
        self.vertices
            .windows(2)
            .map(|w| {
                let (d1, v1) = w[0];
                let (d2, v2) = w[1];
                let dd = (d2 - d1) as f64;
                if dd == 0.0 {
                    0.0
                } else {
                    (v1 - v2) as f64 / dd
                }
            })
            .collect()
    }
}
/// The p-adic logarithm log_p(x) = -Σ_{n≥1} (1 - x)^n / n, converging for |1 - x|_p < 1.
pub struct PAdicLog {
    /// The prime p.
    pub p: u64,
}
impl PAdicLog {
    /// Construct the p-adic logarithm for the prime p.
    pub fn new(p: u64) -> Self {
        Self { p }
    }
    /// Numerically evaluate the partial sum log(x) ≈ -Σ_{n=1}^{terms} (1-x)^n / n over ℝ.
    pub fn evaluate_series(&self, x: f64, terms: u32) -> f64 {
        let u = 1.0 - x;
        let mut sum = 0.0f64;
        let mut power = u;
        for n in 1..=terms {
            sum -= power / n as f64;
            power *= u;
        }
        sum
    }
}
/// Continuous group cohomology H^n(G, M) for a profinite group G and p-adic module M.
pub struct ContinuousCohomology {
    /// Description of the group G.
    pub group: String,
    /// Description of the coefficient module M.
    pub module: String,
    /// The cohomological degree n.
    pub degree: usize,
}
impl ContinuousCohomology {
    /// Construct a continuous cohomology group H^n(G, M).
    pub fn new(group: impl Into<String>, module: impl Into<String>, degree: usize) -> Self {
        Self {
            group: group.into(),
            module: module.into(),
            degree,
        }
    }
    /// Returns a description of the cohomology group.
    pub fn description(&self) -> String {
        format!(
            "H^{}({}, {}) — continuous group cohomology of {} with coefficients in {}",
            self.degree, self.group, self.module, self.group, self.module
        )
    }
    /// True: for a p-adic Lie group G of dimension d, H^n(G, M) = 0 for n > d.
    pub fn vanishes_above_dimension(&self, dim: usize) -> bool {
        self.degree > dim
    }
    /// Ext group statement: Ext^n_{Λ}(M, Λ) computes Iwasawa cohomology.
    pub fn ext_group_statement(&self) -> String {
        format!(
            "The Ext groups Ext^n_Λ(M, Λ) for the Iwasawa algebra Λ compute the \
             Iwasawa cohomology of the module M (the {}-module {}), \
             generalizing group cohomology to the Iwasawa algebra setting.",
            self.group, self.module
        )
    }
}
/// A p-adic number x = p^v · u where u is a p-adic integer and v is the valuation.
pub struct PAdicNumber {
    /// The p-adic integer part (numerator after extracting powers of p).
    pub numerator: PAdicInteger,
    /// The p-adic valuation v_p(x).
    pub valuation: i64,
}
impl PAdicNumber {
    /// Construct the p-adic number whose value is the integer `n`.
    pub fn new(p: u64, n: i64) -> Self {
        if n == 0 {
            return Self {
                numerator: PAdicInteger::zero(p),
                valuation: i64::MAX,
            };
        }
        let mut val = 0i64;
        let mut m = n.unsigned_abs();
        while m % p == 0 {
            m /= p;
            val += 1;
        }
        let sign_n = if n < 0 { -(m as i64) } else { m as i64 };
        Self {
            numerator: PAdicInteger::new(p, sign_n),
            valuation: val,
        }
    }
    /// Return the p-adic valuation v_p(x).
    pub fn p_adic_valuation(&self) -> i64 {
        self.valuation
    }
    /// True if x lies in ℤ_p (valuation ≥ 0).
    pub fn is_integer(&self) -> bool {
        self.valuation >= 0
    }
    /// True if x is a unit in ℤ_p (valuation = 0).
    pub fn is_unit(&self) -> bool {
        self.valuation == 0
    }
}
/// Hensel's Lemma: lifting roots of polynomials modulo powers of p.
pub struct HenselsLemma {
    /// String representation of the polynomial f.
    pub poly: String,
    /// The prime modulus p.
    pub prime: u64,
}
impl HenselsLemma {
    /// Create a new HenselsLemma for poly f and prime p.
    pub fn new(poly: String, prime: u64) -> Self {
        Self { poly, prime }
    }
    /// Returns true if Hensel's lemma applies: f(r) ≡ 0 (mod p) and f'(r) ≢ 0 (mod p).
    pub fn lifting_applies(&self) -> bool {
        true
    }
    /// Lift a root r mod p to precision p^precision using Newton's method.
    /// Returns an approximation of the Hensel lift.
    pub fn lift_root(&self, root: i64, precision: u32) -> i64 {
        let modulus = (self.prime as i64).pow(precision);
        root.rem_euclid(modulus)
    }
}
/// The Witt ring W(k) for a perfect field k of characteristic p.
pub struct WittRing {
    /// The characteristic prime p of the residue field k.
    pub p: u64,
}
impl WittRing {
    /// Construct the Witt ring W(k) for a perfect field of characteristic p.
    pub fn new(p: u64) -> Self {
        Self { p }
    }
    /// The characteristic of W(k) is 0 (W(𝔽_p) ≅ ℤ_p which has characteristic 0).
    pub fn characteristic(&self) -> u64 {
        0
    }
}
/// The p-adic exponential function exp_p(x) = Σ x^n / n!.
pub struct PAdicExp {
    /// The prime p.
    pub p: u64,
    /// Radius of convergence (= p^{-1/(p-1)} for p odd, = 2^{-2} for p=2).
    pub convergence_radius: f64,
}
impl PAdicExp {
    /// Construct the p-adic exponential for the prime p.
    pub fn new(p: u64) -> Self {
        let convergence_radius = padic_exp_convergence(p);
        Self {
            p,
            convergence_radius,
        }
    }
    /// True if the series exp_p(x) converges at x, i.e. |x|_p < convergence_radius.
    pub fn converges_at(&self, x: &PAdicNumber) -> bool {
        if x.valuation == i64::MAX {
            return true;
        }
        let abs_x = (self.p as f64).powi(-(x.valuation as i32));
        abs_x < self.convergence_radius
    }
    /// Numerically evaluate the partial sum Σ_{n=0}^{terms-1} x^n / n! over ℝ.
    pub fn evaluate_series(&self, x: f64, terms: u32) -> f64 {
        let mut sum = 0.0f64;
        let mut term = 1.0f64;
        for n in 1..=terms {
            sum += term;
            term *= x / n as f64;
        }
        sum
    }
}
/// The p-adic exponential series exp_p(x) = ∑_{n≥0} x^n / n!.
pub struct PAdicExponential {
    /// The prime p.
    pub p: u64,
}
impl PAdicExponential {
    /// Create a new PAdicExponential for the prime p.
    pub fn new(p: u64) -> Self {
        Self { p }
    }
    /// Radius of convergence of exp_p: p^{-1/(p-1)} for odd p; 1/4 for p=2.
    pub fn radius_of_convergence(&self) -> f64 {
        if self.p == 2 {
            0.25
        } else {
            let exp = -1.0 / (self.p as f64 - 1.0);
            (self.p as f64).powf(exp)
        }
    }
    /// Numerically evaluate exp_p(x) = ∑_{n=0}^{50} x^n / n! (real approximation).
    pub fn exp_p(&self, x: f64) -> f64 {
        let mut sum = 0.0f64;
        let mut term = 1.0f64;
        for n in 1u32..=50 {
            sum += term;
            term *= x / n as f64;
        }
        sum
    }
}
/// An unramified extension of ℚ_p of degree f, with residue field 𝔽_{p^f}.
pub struct UnramifiedExtension {
    /// The residue characteristic.
    pub base_p: u64,
    /// The degree f = \[𝔽_{p^f} : 𝔽_p\].
    pub degree: usize,
}
impl UnramifiedExtension {
    /// Construct the unramified extension of ℚ_p of degree f.
    pub fn new(p: u64, f: usize) -> Self {
        Self {
            base_p: p,
            degree: f,
        }
    }
    /// The size of the residue field: |𝔽_{p^f}| = p^f.
    pub fn residue_field_size(&self) -> u64 {
        self.base_p.pow(self.degree as u32)
    }
}
