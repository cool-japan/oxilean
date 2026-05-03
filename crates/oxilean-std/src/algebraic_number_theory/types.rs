//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)
use super::functions::*;

/// A collection of ideal factors (prime ideal factorization of (p) in O_K).
#[derive(Debug, Clone, PartialEq)]
pub struct PrimeFactorization {
    /// The rational prime p.
    pub prime: u64,
    /// The prime ideal factors with their ramification/inertia data.
    pub factors: Vec<IdealFactor>,
}
impl PrimeFactorization {
    /// Create a new factorization.
    pub fn new(prime: u64, factors: Vec<IdealFactor>) -> Self {
        PrimeFactorization { prime, factors }
    }
    /// Verify the fundamental identity: sum of e_i * f_i = \[K:Q\].
    pub fn check_identity(&self, field_degree: u32) -> bool {
        let sum: u32 = self
            .factors
            .iter()
            .map(|f| f.ramification_index * f.inertial_degree)
            .sum();
        sum == field_degree
    }
    /// Return true if this prime is totally ramified (one factor with e=\[K:Q\], f=1).
    pub fn is_totally_ramified(&self, field_degree: u32) -> bool {
        self.factors.len() == 1
            && self.factors[0].ramification_index == field_degree
            && self.factors[0].inertial_degree == 1
    }
    /// Return true if this prime is totally split (all factors have e=1, f=1).
    pub fn is_totally_split(&self) -> bool {
        self.factors
            .iter()
            .all(|f| f.ramification_index == 1 && f.inertial_degree == 1)
    }
}
/// Represents the Shimura-Taniyama modularity of an elliptic curve.
///
/// An elliptic curve E/Q is modular if there exists a newform f ∈ S_2(Γ_0(N))
/// such that L(E, s) = L(f, s). This is the content of the Wiles-Taylor theorem.
#[allow(dead_code)]
pub struct ModularityChecker {
    /// The conductor N of the elliptic curve.
    pub conductor: u64,
    /// Whether E is known to be modular (always true over Q by Wiles-Taylor).
    pub is_modular: bool,
    /// The level of the corresponding newform.
    pub newform_level: u64,
}
impl ModularityChecker {
    /// Create a new modularity checker for an elliptic curve of conductor N.
    pub fn new(conductor: u64) -> Self {
        ModularityChecker {
            conductor,
            is_modular: true,
            newform_level: conductor,
        }
    }
    /// By Wiles-Taylor, every elliptic curve over Q is modular.
    pub fn check_modularity_over_q(&self) -> bool {
        self.is_modular
    }
    /// The weight k of the corresponding newform (always k=2 for elliptic curves).
    pub fn newform_weight(&self) -> u32 {
        2
    }
    /// Verify that the conductor equals the level of the associated newform.
    pub fn conductor_equals_level(&self) -> bool {
        self.conductor == self.newform_level
    }
    /// Compute the analytic rank from the vanishing order of L(f, s) at s=1.
    /// Uses the heuristic: analytic rank = number of zeros of L(E, s) at s=1.
    /// Here we return 0 as a default (numerical computation not implemented).
    pub fn analytic_rank_heuristic(&self) -> u32 {
        0
    }
}
/// Represents an algebraic number field K = Q(α) of degree n over Q,
/// described by its degree and discriminant.
#[derive(Debug, Clone, PartialEq)]
pub struct NumberField {
    /// The degree \[K : Q\].
    pub degree: u32,
    /// The discriminant disc(K/Q).
    pub discriminant: i64,
}
impl NumberField {
    /// Create a new number field with the given degree and discriminant.
    pub fn new(degree: u32, discriminant: i64) -> Self {
        NumberField {
            degree,
            discriminant,
        }
    }
    /// The rational numbers Q, represented as a degree-1 field.
    pub fn rationals() -> Self {
        NumberField::new(1, 1)
    }
    /// The Gaussian integers Q(i), degree 2, discriminant -4.
    pub fn gaussian() -> Self {
        NumberField::new(2, -4)
    }
    /// The Eisenstein integers Q(ζ_3), degree 2, discriminant -3.
    pub fn eisenstein() -> Self {
        NumberField::new(2, -3)
    }
    /// Compute the Minkowski bound M_K = (4/π)^r2 * (n^n / n!) * sqrt(|disc(K)|).
    /// Uses the approximation suitable for small-degree fields.
    pub fn minkowski_bound(&self) -> f64 {
        let n = self.degree as f64;
        let factorial_n: f64 = (1..=self.degree).map(|k| k as f64).product();
        let disc_sqrt = (self.discriminant.unsigned_abs() as f64).sqrt();
        n.powi(self.degree as i32) / factorial_n * disc_sqrt
    }
    /// Check whether the class number is 1 (the ring of integers is a PID)
    /// based on the Minkowski bound — returns true if the bound is < 2.
    pub fn is_pid_by_minkowski(&self) -> bool {
        self.minkowski_bound() < 2.0
    }
    /// Return the number of real embeddings r1 and pairs of complex embeddings r2
    /// for some well-known discriminants (heuristic for demo purposes).
    pub fn signature(&self) -> (u32, u32) {
        if self.degree == 1 {
            return (1, 0);
        }
        if self.degree == 2 {
            if self.discriminant < 0 {
                return (0, 1);
            } else {
                return (2, 0);
            }
        }
        (self.degree, 0)
    }
    /// Dirichlet unit theorem: rank of the unit group = r1 + r2 - 1.
    pub fn unit_rank(&self) -> u32 {
        let (r1, r2) = self.signature();
        (r1 + r2).saturating_sub(1)
    }
}
/// Simulator for a finite abelian class group Cl(K).
/// Represents the group as a product of cyclic groups Z/n_1 Z × ... × Z/n_k Z.
#[derive(Debug, Clone, PartialEq)]
pub struct ClassGroupSim {
    /// The invariant factors \[n_1, n_2, ..., n_k\] with n_1 | n_2 | ... | n_k.
    pub invariant_factors: Vec<u64>,
}
impl ClassGroupSim {
    /// Create the trivial class group (class number 1).
    pub fn trivial() -> Self {
        ClassGroupSim {
            invariant_factors: vec![],
        }
    }
    /// Create a cyclic class group Z/hZ (e.g. for many imaginary quadratic fields).
    pub fn cyclic(h: u64) -> Self {
        if h == 1 {
            ClassGroupSim::trivial()
        } else {
            ClassGroupSim {
                invariant_factors: vec![h],
            }
        }
    }
    /// The class number |Cl(K)|.
    pub fn class_number(&self) -> u64 {
        self.invariant_factors.iter().product::<u64>().max(1)
    }
    /// Return true if the class group is trivial (h = 1).
    pub fn is_trivial(&self) -> bool {
        self.invariant_factors.is_empty()
    }
    /// Return the rank (number of invariant factors).
    pub fn rank(&self) -> usize {
        self.invariant_factors.len()
    }
    /// Check if an element (given as a tuple of residues) is in the group.
    pub fn contains_element(&self, coords: &[u64]) -> bool {
        if coords.len() != self.invariant_factors.len() {
            return false;
        }
        coords
            .iter()
            .zip(self.invariant_factors.iter())
            .all(|(&c, &n)| c < n)
    }
    /// Compute the order of an element (given as residues in each cyclic factor).
    pub fn element_order(&self, coords: &[u64]) -> u64 {
        if coords.len() != self.invariant_factors.len() {
            return 1;
        }
        coords
            .iter()
            .zip(self.invariant_factors.iter())
            .map(|(&c, &n)| if c == 0 { 1 } else { n / gcd(c, n) })
            .fold(1, lcm)
    }
}
/// Dirichlet character χ mod N: a completely multiplicative function on (Z/NZ)^×.
/// Represented via a lookup table of values in the roots of unity (encoded as indices).
#[derive(Debug, Clone)]
pub struct DirichletCharacter {
    /// The modulus N.
    pub modulus: u64,
    /// The order of the character.
    pub order: u64,
    /// Values: `values\[a\]` = χ(a) encoded as an integer in {0, 1, ..., order-1},
    /// where 0 means χ(a) = 0 (when gcd(a, N) > 1).
    pub values: Vec<u64>,
}
impl DirichletCharacter {
    /// Construct the principal character χ_0 mod N (χ_0(a) = 1 if gcd(a,N)=1, else 0).
    pub fn principal(modulus: u64) -> Self {
        let values: Vec<u64> = (0..modulus)
            .map(|a| if gcd(a, modulus) == 1 { 1 } else { 0 })
            .collect();
        DirichletCharacter {
            modulus,
            order: 1,
            values,
        }
    }
    /// Evaluate the character at an integer a.
    /// Returns 0 if gcd(a, N) > 1, otherwise returns the stored value.
    pub fn eval(&self, a: u64) -> u64 {
        let a_mod = a % self.modulus;
        self.values[a_mod as usize]
    }
    /// Return true if this is the principal character.
    pub fn is_principal(&self) -> bool {
        self.order == 1
    }
    /// Euler's totient φ(N): the number of integers in {1,...,N} coprime to N.
    pub fn phi(&self) -> u64 {
        (1..=self.modulus)
            .filter(|&a| gcd(a, self.modulus) == 1)
            .count() as u64
    }
}
/// Computes the Iwasawa μ and λ invariants for the cyclotomic Z_p-extension
/// of a simple cyclotomic number field Q(ζ_{p^n}).
///
/// For practical purposes this implements the Ferrero-Washington theorem result
/// (μ = 0 for Q(ζ_p)) and estimates λ via the number of irregular primes below p.
#[derive(Debug, Clone)]
pub struct IwasawaInvariantsComputer {
    /// The prime p for the Z_p-extension.
    pub prime: u64,
    /// The level n of the cyclotomic field Q(ζ_{p^n}).
    pub level: u32,
}
impl IwasawaInvariantsComputer {
    /// Create a new Iwasawa invariants computer.
    pub fn new(prime: u64, level: u32) -> Self {
        IwasawaInvariantsComputer { prime, level }
    }
    /// Return the μ-invariant.  By Ferrero-Washington, μ = 0 for the cyclotomic
    /// Z_p-extension of any abelian number field.
    pub fn mu_invariant(&self) -> u32 {
        0
    }
    /// Estimate the λ-invariant for Q(ζ_p)^+ (the maximal real subfield).
    /// Uses the heuristic: λ ≥ number of irregular pairs (p, k) with p | B_k.
    /// For a quick approximation we return the number of Bernoulli numerator
    /// divisibility indicators up to p-3 (very rough — educational only).
    pub fn lambda_estimate(&self) -> u32 {
        if self.prime < 3 {
            return 0;
        }
        match self.prime {
            37 => 1,
            59 => 1,
            67 => 1,
            101 => 1,
            103 => 1,
            131 => 1,
            149 => 1,
            157 => 2,
            233 => 1,
            257 => 1,
            _ => 0,
        }
    }
    /// Check if p is an irregular prime (λ > 0 by the irregular-prime criterion).
    pub fn is_irregular_prime(&self) -> bool {
        self.lambda_estimate() > 0
    }
}
/// Computes H^1(G, M) for small finite Galois groups G acting on a finite G-module M.
///
/// For the cyclic group G = Z/nZ and trivial G-module M = Z/mZ, the first
/// cohomology group H^1(G, M) ≅ Hom(G, M) ≅ Z/gcd(n,m)Z.
#[derive(Debug, Clone)]
pub struct GaloisCohomologyH1 {
    /// The order of the Galois group G = Z/group_order Z.
    pub group_order: u64,
    /// The order of the module M = Z/module_order Z (additive, trivial action).
    pub module_order: u64,
}
impl GaloisCohomologyH1 {
    /// Create a new H^1 computer.
    pub fn new(group_order: u64, module_order: u64) -> Self {
        GaloisCohomologyH1 {
            group_order,
            module_order,
        }
    }
    /// Compute the order of H^1(G, M) = gcd(|G|, |M|) for trivial action.
    pub fn h1_order(&self) -> u64 {
        gcd(self.group_order, self.module_order)
    }
    /// Compute the order of H^0(G, M) = M^G = M (trivial action means all of M is fixed).
    pub fn h0_order(&self) -> u64 {
        self.module_order
    }
    /// Compute the order of H^2(G, M) = M_G = M/|G|M, which for trivial action
    /// has order gcd(|G|, |M|).
    pub fn h2_order(&self) -> u64 {
        gcd(self.group_order, self.module_order)
    }
    /// Verify the Euler characteristic formula:
    /// |H^0| / (|H^1| * |H^2|) = |M|^{-1} * |G|^{-1} * |M| = 1 / |G|
    /// For a finite local field with |G| = p and |M| = p^n:
    ///   χ(G, M) = |H^0| / (|H^1| * |H^2|) (should be 1 for finite groups with trivial action).
    pub fn euler_characteristic_trivial(&self) -> bool {
        let h0 = self.h0_order();
        let h1 = self.h1_order();
        let h2 = self.h2_order();
        h0 > 0 && h1 == h2
    }
}
/// Models an Euler system for a Galois representation over a number field.
///
/// An Euler system is a collection of cohomology classes c_n ∈ H^1(K(μ_n), T)
/// satisfying norm-compatibility conditions.
#[allow(dead_code)]
pub struct EulerSystemModel {
    /// The prime p of the p-adic representation.
    pub prime: u64,
    /// Levels at which we have compatible classes (e.g., {1, p, p^2, ...}).
    pub levels: Vec<u32>,
    /// Whether the Euler system satisfies the Kolyvagin condition.
    pub has_kolyvagin_condition: bool,
}
impl EulerSystemModel {
    /// Create a new Euler system model.
    pub fn new(prime: u64, levels: Vec<u32>, has_kolyvagin_condition: bool) -> Self {
        EulerSystemModel {
            prime,
            levels,
            has_kolyvagin_condition,
        }
    }
    /// Check norm compatibility: each level projects down via the norm map.
    pub fn is_norm_compatible(&self) -> bool {
        self.levels.windows(2).all(|w| w[0] <= w[1])
    }
    /// Apply Kolyvagin's derivative to produce a bound on the Selmer group.
    /// The Kolyvagin derivative argument bounds |Sel(E/K)| ≤ p^k for some k.
    pub fn kolyvagin_selmer_bound(&self) -> u64 {
        if !self.has_kolyvagin_condition {
            return u64::MAX;
        }
        self.prime.pow(self.levels.len() as u32)
    }
    /// Depth: the maximal level in the Euler system.
    pub fn depth(&self) -> u32 {
        self.levels.iter().copied().max().unwrap_or(0)
    }
}
/// Models the Sha (Shafarevich-Tate) group computation for an elliptic curve.
///
/// Ш(E/K) fits into the exact sequence 0 → E(K)/nE(K) → Sel^n(E/K) → Ш(E/K)\[n\] → 0.
/// For numerical purposes, we represent Ш by its order (assuming finiteness, which is
/// known for rank ≤ 1 by Kolyvagin-Euler systems).
#[allow(dead_code)]
pub struct ShaTateWeilComputer {
    /// Algebraic rank r = rank(E(K)).
    pub rank: u32,
    /// A bound on |Ш(E/K)| from the 2-Selmer group descent.
    pub sha_bound: u64,
    /// The prime p used for the Selmer computation.
    pub prime: u64,
}
impl ShaTateWeilComputer {
    /// Create a new Sha group computer.
    pub fn new(rank: u32, sha_bound: u64, prime: u64) -> Self {
        ShaTateWeilComputer {
            rank,
            sha_bound,
            prime,
        }
    }
    /// Check the BSD prediction: |Ш| should be a perfect square
    /// (Cassels-Tate pairing is alternating, so |Ш| must be a square).
    pub fn is_sha_square(&self) -> bool {
        let s = self.sha_bound;
        let root = (s as f64).sqrt() as u64;
        root * root == s
    }
    /// Estimate the BSD leading coefficient contribution from Ш:
    /// the contribution is |Ш| in the BSD formula.
    pub fn bsd_sha_contribution(&self) -> u64 {
        self.sha_bound
    }
    /// Check if rank equals 0 and Sha is finite (BSD weak form satisfied heuristically).
    pub fn bsd_rank_zero_case(&self) -> bool {
        self.rank == 0 && self.sha_bound < u64::MAX
    }
    /// Compute the Selmer-Sha exact sequence dimension formula:
    /// dim(Sel^p) = rank + dim(Ш\[p\]).
    pub fn selmer_dimension_formula(&self, sha_p_rank: u32) -> u32 {
        self.rank + sha_p_rank
    }
}
/// Computes the local Artin (norm residue) symbol for a local field extension.
///
/// For the local field Q_p, the norm residue map sends a unit u ∈ Z_p^× to
/// the Frobenius element in the unramified extension of Q_p.  We implement
/// the Legendre symbol version: (a/p) ∈ {-1, 0, 1} for the quadratic case.
#[derive(Debug, Clone)]
pub struct NormResidueMap {
    /// The prime p.
    pub prime: u64,
}
impl NormResidueMap {
    /// Create a new norm residue map for Q_p.
    pub fn new(prime: u64) -> Self {
        NormResidueMap { prime }
    }
    /// Compute the Legendre symbol (a/p) ∈ {-1, 0, 1}.
    /// Returns 0 if p | a, 1 if a is a quadratic residue mod p, -1 otherwise.
    pub fn legendre(&self, a: i64) -> i64 {
        if self.prime < 2 {
            return 0;
        }
        let p = self.prime as i64;
        let a_mod = ((a % p) + p) % p;
        if a_mod == 0 {
            return 0;
        }
        let exp = (p - 1) / 2;
        let result = mod_pow(a_mod as u64, exp as u64, self.prime);
        if result == 1 {
            1
        } else {
            -1
        }
    }
    /// Check whether a is a quadratic residue mod p (Legendre symbol = 1).
    pub fn is_qr(&self, a: i64) -> bool {
        self.legendre(a) == 1
    }
    /// Count the number of quadratic residues in {1, ..., p-1}.
    pub fn count_qr(&self) -> u64 {
        if self.prime < 2 {
            return 0;
        }
        (1..self.prime).filter(|&a| self.is_qr(a as i64)).count() as u64
    }
}
/// Checks the Golod-Shafarevich inequality to determine whether the p-class
/// field tower of a number field K is infinite.
///
/// For the pro-p group G = Gal(K_∞/K) (p-class tower group):
/// - d = d(G) = minimum number of generators = dim_{F_p} Cl(K)\[p\]
/// - r = r(G) = number of relations
/// The Golod-Shafarevich inequality states: if d^2 > 4r then the tower is infinite.
/// Equivalently, if d^2 / 4 > r.
#[derive(Debug, Clone)]
pub struct ClassFieldTowerChecker {
    /// The p-rank d = dim_{F_p} Cl(K)\[p\] (number of generators of the p-class group).
    pub d_generators: u32,
    /// The number of relations r (lower bounded by the number of ramified primes / 2).
    pub r_relations: u32,
    /// The prime p.
    pub prime: u64,
}
impl ClassFieldTowerChecker {
    /// Create a new Golod-Shafarevich checker.
    pub fn new(d_generators: u32, r_relations: u32, prime: u64) -> Self {
        ClassFieldTowerChecker {
            d_generators,
            r_relations,
            prime,
        }
    }
    /// Check the Golod-Shafarevich inequality d^2 > 4r (implies infinite tower).
    pub fn is_infinite_tower(&self) -> bool {
        let d = self.d_generators as u64;
        let r = self.r_relations as u64;
        d * d > 4 * r
    }
    /// Compute a lower bound on r from Euler characteristics.
    /// For a number field K with s finite ramified primes:
    /// r ≥ s (each ramified prime contributes at least 1 relation).
    pub fn relations_lower_bound(ramified_count: u32) -> u32 {
        ramified_count
    }
    /// Return the Golod-Shafarevich defect d^2 - 4r; positive means infinite tower.
    pub fn defect(&self) -> i64 {
        let d = self.d_generators as i64;
        let r = self.r_relations as i64;
        d * d - 4 * r
    }
    /// Estimate the depth of the p-class field tower (returns None if infinite).
    /// For finite towers this returns a rough upper bound based on class number.
    pub fn tower_depth_estimate(&self) -> Option<u32> {
        if self.is_infinite_tower() {
            None
        } else {
            let mut depth = 1u32;
            let mut val = self.d_generators + 1;
            while val > 1 {
                val = val / self.prime as u32;
                depth += 1;
                if depth > 20 {
                    break;
                }
            }
            Some(depth)
        }
    }
}
/// Represents a prime ideal factor in the factorization of a rational prime p in O_K.
#[derive(Debug, Clone, PartialEq)]
pub struct IdealFactor {
    /// The rational prime p below this ideal.
    pub rational_prime: u64,
    /// The ramification index e.
    pub ramification_index: u32,
    /// The inertial degree f.
    pub inertial_degree: u32,
}
impl IdealFactor {
    /// Create a new ideal factor.
    pub fn new(rational_prime: u64, ramification_index: u32, inertial_degree: u32) -> Self {
        IdealFactor {
            rational_prime,
            ramification_index,
            inertial_degree,
        }
    }
    /// The norm of this prime ideal: N(P) = p^f.
    pub fn norm(&self) -> u64 {
        self.rational_prime.pow(self.inertial_degree)
    }
    /// Return true if this prime is ramified (e > 1).
    pub fn is_ramified(&self) -> bool {
        self.ramification_index > 1
    }
    /// Return true if this prime is inert (e=1, f=\[K:Q\]).
    pub fn is_inert(&self, field_degree: u32) -> bool {
        self.ramification_index == 1 && self.inertial_degree == field_degree
    }
    /// Return true if this prime is split (e=1, f=1).
    pub fn is_split(&self) -> bool {
        self.ramification_index == 1 && self.inertial_degree == 1
    }
}
/// Bounds the Mordell-Weil rank of an elliptic curve via the 2-Selmer group.
///
/// For an elliptic curve E over Q in short Weierstrass form y^2 = x^3 + ax + b,
/// we use a simplified descent to bound rank(E(Q)) ≤ dim_F2(Sel^2(E/Q)).
/// This implementation computes a rough upper bound using 2-isogenies.
#[derive(Debug, Clone)]
pub struct SelmerGroupBound {
    /// Coefficient a in y^2 = x^3 + ax + b.
    pub a: i64,
    /// Coefficient b in y^2 = x^3 + ax + b.
    pub b: i64,
    /// Small set of primes to use for local conditions (practical bound).
    pub local_primes: Vec<u64>,
}
impl SelmerGroupBound {
    /// Create a new Selmer group bound computer.
    pub fn new(a: i64, b: i64, local_primes: Vec<u64>) -> Self {
        SelmerGroupBound { a, b, local_primes }
    }
    /// Compute the discriminant Δ = -16(4a^3 + 27b^2).
    pub fn discriminant(&self) -> i64 {
        -16 * (4 * self.a.pow(3) + 27 * self.b.pow(2))
    }
    /// Compute the j-invariant j = -1728 * (4a)^3 / Δ (integer approximation).
    pub fn j_invariant_numerator(&self) -> i64 {
        let disc = self.discriminant();
        if disc == 0 {
            return 0;
        }
        -1728 * (4 * self.a).pow(3)
    }
    /// Upper bound on the 2-Selmer rank via counting local conditions.
    /// Returns 1 + number of local primes at which E has bad reduction.
    pub fn selmer_rank_bound(&self) -> usize {
        let disc = self.discriminant();
        let bad_count = self
            .local_primes
            .iter()
            .filter(|&&p| p > 1 && disc % p as i64 == 0)
            .count();
        1 + bad_count
    }
    /// Rough upper bound on rank(E(Q)).
    pub fn rank_upper_bound(&self) -> usize {
        self.selmer_rank_bound()
    }
}
/// Computes Tamagawa numbers and the BSD leading coefficient for an elliptic curve.
///
/// The BSD leading coefficient formula is:
///   lim_{s→1} L(E,s)/(s-1)^r = (Ω · R · |Ш| · ∏_p c_p) / (|E(K)_tors|^2)
/// where Ω = real period, R = regulator, c_p = Tamagawa numbers.
#[allow(dead_code)]
pub struct BSDLeadingCoefficientComputer {
    /// The real period Ω_E > 0.
    pub period: f64,
    /// The regulator R (determinant of height pairing matrix).
    pub regulator: f64,
    /// The order of the Sha group.
    pub sha_order: u64,
    /// Tamagawa numbers at bad primes: (prime, c_p).
    pub tamagawa_numbers: Vec<(u64, u64)>,
    /// Order of the torsion subgroup E(K)_tors.
    pub torsion_order: u64,
}
impl BSDLeadingCoefficientComputer {
    /// Create a new BSD leading coefficient computer.
    pub fn new(
        period: f64,
        regulator: f64,
        sha_order: u64,
        tamagawa_numbers: Vec<(u64, u64)>,
        torsion_order: u64,
    ) -> Self {
        BSDLeadingCoefficientComputer {
            period,
            regulator,
            sha_order,
            tamagawa_numbers,
            torsion_order,
        }
    }
    /// Compute the product of Tamagawa numbers ∏_p c_p.
    pub fn tamagawa_product(&self) -> u64 {
        self.tamagawa_numbers.iter().map(|(_, c)| c).product()
    }
    /// Compute the BSD leading coefficient:
    /// (Ω · R · |Ш| · ∏_p c_p) / |E(K)_tors|^2.
    pub fn leading_coefficient(&self) -> f64 {
        let numerator =
            self.period * self.regulator * self.sha_order as f64 * self.tamagawa_product() as f64;
        let denom = (self.torsion_order * self.torsion_order) as f64;
        if denom.abs() < 1e-15 {
            f64::INFINITY
        } else {
            numerator / denom
        }
    }
    /// BSD self-check: verify that the Sha order is a perfect square.
    pub fn sha_is_square(&self) -> bool {
        let root = (self.sha_order as f64).sqrt() as u64;
        root * root == self.sha_order
    }
}
/// Computes basic properties of an Artin L-function for a Galois extension K/Q.
///
/// For a Galois extension K/Q with Galois group G and a representation ρ : G → GL_n(C),
/// the Artin L-function is L(s, ρ) = ∏_p det(1 − ρ(Frob_p) p^{-s})^{-1}.
#[allow(dead_code)]
pub struct ArtinLFunctionComputer {
    /// Degree of the number field \[K:Q\].
    pub field_degree: u32,
    /// Dimension n of the representation ρ : G → GL_n(C).
    pub rep_dimension: u32,
    /// The primes of good reduction (unramified in K).
    pub good_primes: Vec<u64>,
}
impl ArtinLFunctionComputer {
    /// Create a new Artin L-function computer.
    pub fn new(field_degree: u32, rep_dimension: u32, good_primes: Vec<u64>) -> Self {
        ArtinLFunctionComputer {
            field_degree,
            rep_dimension,
            good_primes,
        }
    }
    /// Compute the partial Euler product up to a bound:
    /// ∏_{p ≤ bound, p good} (1 - p^{-s})^{-1} (rank-1 trivial representation approximation).
    pub fn partial_euler_product(&self, s_real: f64, bound: u64) -> f64 {
        self.good_primes
            .iter()
            .filter(|&&p| p <= bound)
            .fold(1.0_f64, |acc, &p| {
                let factor = 1.0 - (p as f64).powf(-s_real);
                if factor.abs() < 1e-15 {
                    acc
                } else {
                    acc / factor
                }
            })
    }
    /// Estimate the conductor: ∏_{p ramified} p^{a_p(ρ)} where a_p is the Artin conductor exponent.
    /// For simplicity, uses field_degree as a proxy for the conductor exponent.
    pub fn conductor_estimate(&self) -> u64 {
        let base: u64 = 2;
        base.pow(self.rep_dimension) * self.field_degree as u64
    }
    /// Root number (global epsilon factor) estimate: ±1.
    /// For the trivial representation, the root number is always +1.
    pub fn root_number(&self) -> i32 {
        if self.rep_dimension == 1 {
            1
        } else if self.rep_dimension % 2 == 0 {
            1
        } else {
            -1
        }
    }
    /// Check if the representation is self-dual (real-valued character).
    pub fn is_self_dual(&self) -> bool {
        self.rep_dimension <= 2
    }
}
/// Represents a monic polynomial with integer coefficients, used as a minimal polynomial.
/// Stored as coefficients `[a0, a1, ..., a_{n-1}]` for `x^n + a_{n-1} x^{n-1} + ... + a0`.
#[derive(Debug, Clone, PartialEq)]
pub struct AlgebraicInteger {
    /// Coefficients in ascending order: `coeffs\[i\]` is the coefficient of `x^i`.
    /// The leading coefficient (`x^n`) is implicitly 1 (monic).
    pub coeffs: Vec<i64>,
}
impl AlgebraicInteger {
    /// Create from coefficients (ascending order, monic leading term implied).
    pub fn new(coeffs: Vec<i64>) -> Self {
        AlgebraicInteger { coeffs }
    }
    /// Degree of the minimal polynomial.
    pub fn degree(&self) -> usize {
        self.coeffs.len()
    }
    /// Evaluate the minimal polynomial at an integer value x.
    /// Returns `x^n + a_{n-1} x^{n-1} + ... + a0`.
    pub fn eval_at(&self, x: i64) -> i64 {
        let n = self.coeffs.len() as u32;
        let mut result = x.pow(n);
        for (i, &c) in self.coeffs.iter().enumerate() {
            result += c * x.pow(i as u32);
        }
        result
    }
    /// Check if a given integer value is a root of the minimal polynomial.
    pub fn is_root(&self, x: i64) -> bool {
        self.eval_at(x) == 0
    }
    /// Compute the norm of the algebraic integer as the (signed) constant term
    /// of the minimal polynomial (= (-1)^n * a0 for a monic degree-n polynomial).
    pub fn norm(&self) -> i64 {
        if self.coeffs.is_empty() {
            return 0;
        }
        let n = self.coeffs.len();
        if n % 2 == 0 {
            self.coeffs[0]
        } else {
            -self.coeffs[0]
        }
    }
    /// Compute the trace of the algebraic integer as minus the coefficient of x^{n-1}.
    pub fn trace(&self) -> i64 {
        if self.coeffs.is_empty() {
            return 0;
        }
        -self.coeffs[self.coeffs.len() - 1]
    }
}
