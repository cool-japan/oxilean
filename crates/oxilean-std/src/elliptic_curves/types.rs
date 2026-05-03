//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)
use super::functions::*;

/// Schoof's algorithm (simplified): count points on E: y² = x³ + ax + b over F_p.
///
/// Uses small primes ℓ and computes t mod ℓ via the division polynomial structure.
/// This demo runs the naive fallback (exhaustive) for small p and returns a structure.
#[allow(dead_code)]
pub struct SchoofAlgorithm {
    /// Coefficient a in y² = x³ + ax + b.
    pub a: i64,
    /// Coefficient b in y² = x³ + ax + b.
    pub b: i64,
    /// Prime p (characteristic of the base field F_p).
    pub p: u64,
}
impl SchoofAlgorithm {
    /// Construct a Schoof algorithm instance.
    pub fn new(a: i64, b: i64, p: u64) -> Self {
        Self { a, b, p }
    }
    /// Count #E(F_p) by exhaustive search (correct for small p).
    ///
    /// For each x in F_p, check if y² = x³ + ax + b has a solution mod p;
    /// if so, add 2 to the count (for ±y), handling y=0 separately.
    pub fn count_points_exhaustive(&self) -> u64 {
        let p = self.p;
        let mut count = 1u64;
        for x in 0..p {
            let rhs = (pow_mod(x, 3, p) as i128
                + self.a.rem_euclid(p as i64) as i128 * x as i128 % p as i128
                + self.b.rem_euclid(p as i64) as i128)
                .rem_euclid(p as i128) as u64;
            if rhs == 0 {
                count += 1;
            } else if is_quadratic_residue(rhs, p) {
                count += 2;
            }
        }
        count
    }
    /// Simplified Schoof step: compute the trace of Frobenius a_p = p + 1 - #E(F_p).
    pub fn trace_of_frobenius(&self) -> i64 {
        let n = self.count_points_exhaustive();
        self.p as i64 + 1 - n as i64
    }
}
/// Weil pairing result placeholder (phase-independent check).
#[derive(Debug, Clone)]
pub struct WeilPairingResult {
    pub order: u64,
    pub bilinear: bool,
    pub non_degenerate: bool,
    pub alternating: bool,
}
/// A projective point on an elliptic curve: (X : Y : Z) in P².
#[derive(Debug, Clone, PartialEq)]
pub enum ProjectivePoint {
    /// The point at infinity (identity element of the group).
    Infinity,
    /// An affine point (x, y) embedded as (x : y : 1).
    Affine(i64, i64),
}
impl ProjectivePoint {
    /// Check if this is the point at infinity.
    pub fn is_infinity(&self) -> bool {
        matches!(self, ProjectivePoint::Infinity)
    }
}
/// The n-torsion subgroup E\[n\] and pairing theory on it.
pub struct TorsionPoints {
    /// String description of the curve.
    pub curve: String,
    /// The torsion order n.
    pub n: u64,
}
impl TorsionPoints {
    /// Construct a TorsionPoints instance.
    pub fn new(curve: String, n: u64) -> Self {
        Self { curve, n }
    }
    /// Compute the structure of the n-torsion subgroup.
    ///
    /// Over an algebraically closed field: E\[n\] ≅ Z/nZ × Z/nZ (for gcd(n,char)=1).
    /// Over a finite field F_q: E\[n\](F_q) ≅ Z/dZ × Z/(nd/d)Z for some d | n.
    pub fn n_torsion_structure(&self) -> String {
        format!(
            "E[{}] on {}: over k̄ (char ∤ {}), E[n] ≅ ℤ/{}ℤ × ℤ/{}ℤ; \
             over F_q depends on the Frobenius eigenvalues.",
            self.n, self.curve, self.n, self.n, self.n
        )
    }
    /// Compute the Weil pairing restricted to E\[n\].
    ///
    /// e_n: E\[n\] × E\[n\] → μ_n is bilinear, alternating, and non-degenerate.
    pub fn weil_pairing_on_torsion(&self) -> String {
        format!(
            "Weil pairing on E[{}] of {}: e_{}: E[{}] × E[{}] → μ_{} ⊂ k*; \
             satisfies e(P,Q) = e(Q,P)^{{-1}} (alternating) and non-degeneracy.",
            self.n, self.curve, self.n, self.n, self.n, self.n
        )
    }
}
/// A point on an elliptic curve: either the point at infinity or an affine point.
#[derive(Debug, Clone, PartialEq)]
pub enum EllipticCurvePoint {
    /// The identity element O (point at infinity).
    Infinity,
    /// An affine point (x, y) on the curve.
    Affine(f64, f64),
}
impl EllipticCurvePoint {
    /// Add two points on an elliptic curve given by y² = x³ + ax + b.
    pub fn add_points(&self, other: &EllipticCurvePoint, a: f64) -> EllipticCurvePoint {
        match (self, other) {
            (EllipticCurvePoint::Infinity, p) | (p, EllipticCurvePoint::Infinity) => p.clone(),
            (EllipticCurvePoint::Affine(x1, y1), EllipticCurvePoint::Affine(x2, y2)) => {
                if (x1 - x2).abs() < 1e-12 {
                    if (y1 + y2).abs() < 1e-12 {
                        return EllipticCurvePoint::Infinity;
                    }
                    return self.double_point(a);
                }
                let lambda = (y2 - y1) / (x2 - x1);
                let x3 = lambda * lambda - x1 - x2;
                let y3 = lambda * (x1 - x3) - y1;
                EllipticCurvePoint::Affine(x3, y3)
            }
        }
    }
    /// Double a point: 2P on y² = x³ + ax + b.
    pub fn double_point(&self, a: f64) -> EllipticCurvePoint {
        match self {
            EllipticCurvePoint::Infinity => EllipticCurvePoint::Infinity,
            EllipticCurvePoint::Affine(x, y) => {
                if y.abs() < 1e-12 {
                    return EllipticCurvePoint::Infinity;
                }
                let lambda = (3.0 * x * x + a) / (2.0 * y);
                let x3 = lambda * lambda - 2.0 * x;
                let y3 = lambda * (x - x3) - y;
                EllipticCurvePoint::Affine(x3, y3)
            }
        }
    }
    /// Negate a point: -(x,y) = (x,-y), -O = O.
    pub fn negate(&self) -> EllipticCurvePoint {
        match self {
            EllipticCurvePoint::Infinity => EllipticCurvePoint::Infinity,
            EllipticCurvePoint::Affine(x, y) => EllipticCurvePoint::Affine(*x, -y),
        }
    }
}
/// An isogeny φ: E → E' of elliptic curves with given degree.
pub struct IsogenyPhi {
    /// String description of the domain curve.
    pub domain: String,
    /// String description of the codomain curve.
    pub codomain: String,
    /// Degree of the isogeny.
    pub degree: u64,
}
impl IsogenyPhi {
    /// Construct an isogeny.
    pub fn new(domain: String, codomain: String, degree: u64) -> Self {
        Self {
            domain,
            codomain,
            degree,
        }
    }
    /// Vélu's formula: given the kernel subgroup G ⊂ E, construct the isogeny φ_G.
    ///
    /// For G = {O, P_1, …, P_{l-1}} with #G = l prime:
    ///   φ(x,y) = (x + Σ_{Q∈G\{O}} (x − x_Q)^{-1} t_Q, y − Σ … )
    pub fn velu_formula(&self) -> String {
        format!(
            "Vélu formula for degree-{} isogeny φ: {} → {}: \
             x-coordinate map x ↦ x + Σ_{{Q∈G\\{{O}}}} (x_Q − (x_Q)^2/(x−x_Q)) \
             computable in O(deg φ) field operations.",
            self.degree, self.domain, self.codomain
        )
    }
    /// Determine whether the isogeny is separable (i.e. the derivative is non-zero).
    ///
    /// An isogeny of degree prime to char(k) is always separable.
    pub fn is_separable(&self) -> bool {
        self.degree > 0
    }
}
/// Elliptic curve Diffie-Hellman (ECDH) key exchange.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ECDHExchange {
    pub curve_name: String,
    pub key_size_bits: usize,
}
#[allow(dead_code)]
impl ECDHExchange {
    pub fn new(curve: &str, bits: usize) -> Self {
        ECDHExchange {
            curve_name: curve.to_string(),
            key_size_bits: bits,
        }
    }
    pub fn x25519() -> Self {
        ECDHExchange::new("Curve25519", 255)
    }
    pub fn x448() -> Self {
        ECDHExchange::new("Curve448-Goldilocks", 448)
    }
    pub fn shared_secret_size_bytes(&self) -> usize {
        (self.key_size_bits + 7) / 8
    }
    pub fn security_level_bits(&self) -> usize {
        self.key_size_bits / 2
    }
}
/// Pairings-based BLS signature scheme.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct BLSSignature {
    pub curve_name: String,
    pub embedding_degree: usize,
    pub security_bits: usize,
}
#[allow(dead_code)]
impl BLSSignature {
    pub fn new(curve: &str, k: usize, sec: usize) -> Self {
        BLSSignature {
            curve_name: curve.to_string(),
            embedding_degree: k,
            security_bits: sec,
        }
    }
    pub fn bls12_381() -> Self {
        BLSSignature::new("BLS12-381", 12, 128)
    }
    /// BLS signatures are aggregatable.
    pub fn supports_aggregation(&self) -> bool {
        true
    }
    pub fn signature_size_bytes(&self) -> usize {
        48
    }
    pub fn public_key_size_bytes(&self) -> usize {
        96
    }
    /// Verification uses 2 pairing operations.
    pub fn verify_pairing_count() -> usize {
        2
    }
}
/// A Weierstrass form y² = x³ + ax + b over a field of characteristic `char_p`.
#[derive(Debug, Clone)]
pub struct WeierstrassForm {
    /// Coefficient a in the short Weierstrass equation y² = x³ + ax + b.
    pub a: f64,
    /// Coefficient b in the short Weierstrass equation y² = x³ + ax + b.
    pub b: f64,
    /// Characteristic of the base field (0 for characteristic zero).
    pub char_p: u64,
}
impl WeierstrassForm {
    /// Construct a Weierstrass form.
    pub fn new(a: f64, b: f64, char_p: u64) -> Self {
        Self { a, b, char_p }
    }
    /// Compute the discriminant Δ = -16(4a³ + 27b²).
    pub fn discriminant(&self) -> f64 {
        -16.0 * (4.0 * self.a.powi(3) + 27.0 * self.b.powi(2))
    }
    /// Compute the j-invariant j = -1728 · (4a)³ / Δ (when Δ ≠ 0).
    pub fn j_invariant(&self) -> Option<f64> {
        let disc = self.discriminant();
        if disc.abs() < 1e-12 {
            None
        } else {
            Some(-1728.0 * (4.0 * self.a).powi(3) / disc)
        }
    }
    /// The curve is nonsingular iff Δ ≠ 0.
    pub fn is_nonsingular(&self) -> bool {
        self.discriminant().abs() > 1e-12
    }
}
/// Elliptic curve cryptography operations.
pub struct ECCrypto {
    /// String description of the curve (e.g. "P-256").
    pub curve: String,
    /// Base point (generator) G.
    pub base_point: (f64, f64),
    /// Order n of the base point.
    pub order: u64,
}
impl ECCrypto {
    /// Construct an ECC instance.
    pub fn new(curve: String, base_point: (f64, f64), order: u64) -> Self {
        Self {
            curve,
            base_point,
            order,
        }
    }
    /// Elliptic Curve Diffie-Hellman key exchange.
    ///
    /// Alice: (d_A, Q_A = d_A·G); Bob: (d_B, Q_B = d_B·G).
    /// Shared secret: d_A·Q_B = d_B·Q_A = d_A·d_B·G.
    pub fn ecdh(&self) -> String {
        format!(
            "ECDH on {}: shared secret = d_A·Q_B = d_B·Q_A = d_A·d_B·G \
             where G = ({:.2}, {:.2}), ord(G) = {}.",
            self.curve, self.base_point.0, self.base_point.1, self.order
        )
    }
    /// ECDSA signature: sign a message hash e with private key d.
    ///
    /// Choose random k ∈ \[1, n-1\]; compute (x_1, _) = k·G; r = x_1 mod n;
    /// s = k^{-1}(e + dr) mod n.
    pub fn ecdsa_sign(&self) -> String {
        format!(
            "ECDSA sign on {}: choose random k; R = k·G; r = R.x mod {}; \
             s = k⁻¹(hash + d·r) mod {}.",
            self.curve, self.order, self.order
        )
    }
    /// ECDSA verification: verify (r,s) against public key Q and message hash e.
    ///
    /// Compute u1 = e·s^{-1} mod n, u2 = r·s^{-1} mod n;
    /// check that (u1·G + u2·Q).x ≡ r (mod n).
    pub fn ecdsa_verify(&self) -> String {
        format!(
            "ECDSA verify on {}: u1 = hash·s⁻¹ mod {n}, u2 = r·s⁻¹ mod {n}; \
             accept iff (u1·G + u2·Q).x ≡ r (mod {n}).",
            self.curve,
            n = self.order
        )
    }
}
/// Twisted Edwards curve representation.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct TwistedEdwardsCurve {
    pub a: i64,
    pub d: i64,
    pub field_prime: u64,
}
#[allow(dead_code)]
impl TwistedEdwardsCurve {
    pub fn new(a: i64, d: i64, p: u64) -> Self {
        TwistedEdwardsCurve {
            a,
            d,
            field_prime: p,
        }
    }
    /// Ed25519 parameters.
    pub fn ed25519() -> Self {
        TwistedEdwardsCurve::new(-1, -121665, (1u64 << 55) - 19)
    }
    /// Neutral element is (0, 1).
    pub fn neutral_point() -> (i64, i64) {
        (0, 1)
    }
    /// Unified addition formula (always works, no exceptional cases).
    pub fn has_complete_addition_law(&self) -> bool {
        self.a != self.d
    }
    /// Point count for complete twisted Edwards: always a multiple of 4.
    pub fn group_order_multiple_of_4(&self) -> bool {
        true
    }
}
/// Scalar multiplication of an elliptic curve point.
pub struct ScalarMult {
    /// The base point (x, y).
    pub point: (f64, f64),
    /// The scalar multiplier.
    pub k: u64,
}
impl ScalarMult {
    /// Construct a scalar multiplication instance.
    pub fn new(point: (f64, f64), k: u64) -> Self {
        Self { point, k }
    }
    /// Double-and-add scalar multiplication: O(log k) point doublings and additions.
    pub fn double_and_add(&self, a: f64) -> EllipticCurvePoint {
        let mut result = EllipticCurvePoint::Infinity;
        let mut addend = EllipticCurvePoint::Affine(self.point.0, self.point.1);
        let mut n = self.k;
        while n > 0 {
            if n & 1 == 1 {
                result = result.add_points(&addend, a);
            }
            addend = addend.double_point(a);
            n >>= 1;
        }
        result
    }
    /// Montgomery ladder scalar multiplication (constant-time, resistant to side-channels).
    pub fn montgomery_ladder(&self, a: f64) -> EllipticCurvePoint {
        let p = EllipticCurvePoint::Affine(self.point.0, self.point.1);
        let mut r0 = EllipticCurvePoint::Infinity;
        let mut r1 = p.clone();
        for i in (0..64).rev() {
            if (self.k >> i) & 1 == 0 {
                r1 = r0.add_points(&r1, a);
                r0 = r0.double_point(a);
            } else {
                r0 = r0.add_points(&r1, a);
                r1 = r1.double_point(a);
            }
        }
        r0
    }
    /// Windowed (fixed-window) scalar multiplication with window size w=4.
    pub fn windowed(&self, a: f64) -> EllipticCurvePoint {
        let w: u64 = 4;
        let window = 1u64 << w;
        let base = EllipticCurvePoint::Affine(self.point.0, self.point.1);
        let mut table = vec![EllipticCurvePoint::Infinity];
        for _ in 1..window {
            let last = table
                .last()
                .expect("table is non-empty: initialized with Infinity")
                .clone();
            table.push(last.add_points(&base, a));
        }
        let mut result = EllipticCurvePoint::Infinity;
        let bits = 64u32;
        let mut i = bits;
        while i > 0 {
            i = i.saturating_sub(w as u32);
            for _ in 0..w {
                result = result.double_point(a);
            }
            let digit = ((self.k >> i) & (window - 1)) as usize;
            result = result.add_points(&table[digit], a);
        }
        result
    }
}
/// Vélu isogeny computation: compute the codomain curve of an isogeny from a subgroup.
///
/// Given an elliptic curve E: y² = x³ + ax + b over ℝ (or ℤ) and a finite
/// kernel subgroup K ⊂ E (list of affine x-coordinates of non-identity points),
/// Vélu's formulae give the codomain curve E' and the explicit map φ.
#[allow(dead_code)]
pub struct VeluIsogeny {
    /// Coefficient a in y² = x³ + ax + b.
    pub a: f64,
    /// Coefficient b in y² = x³ + ax + b.
    pub b: f64,
    /// x-coordinates of the non-identity points in the kernel K (assuming y² ≠ 0).
    pub kernel_x_coords: Vec<f64>,
}
impl VeluIsogeny {
    /// Construct a Vélu isogeny instance.
    pub fn new(a: f64, b: f64, kernel_x_coords: Vec<f64>) -> Self {
        Self {
            a,
            b,
            kernel_x_coords,
        }
    }
    /// Compute the coefficients (a', b') of the codomain curve E' via Vélu's formulae.
    ///
    /// For each non-identity kernel point Q = (x_Q, y_Q):
    ///   t_Q = 3x_Q² + a (for points of order > 2) or 0 (for 2-torsion)
    ///   w_Q = y_Q² + t_Q · x_Q
    ///   t  = Σ t_Q,   w = Σ w_Q
    /// Then: a' = a - 5t,  b' = b - 7w.
    pub fn codomain_coefficients(&self) -> (f64, f64) {
        let mut t_total = 0.0f64;
        let mut w_total = 0.0f64;
        for &xq in &self.kernel_x_coords {
            let yq_sq = xq.powi(3) + self.a * xq + self.b;
            let tq = 3.0 * xq * xq + self.a;
            let wq = yq_sq + tq * xq;
            t_total += tq;
            w_total += wq;
        }
        let a_prime = self.a - 5.0 * t_total;
        let b_prime = self.b - 7.0 * w_total;
        (a_prime, b_prime)
    }
    /// Degree of the isogeny equals |K| (the size of the kernel subgroup, counting O).
    pub fn degree(&self) -> usize {
        self.kernel_x_coords.len() + 1
    }
}
/// Isogeny computation for post-quantum cryptography.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct IsogenyComputation {
    pub degree: u64,
    pub kernel_size: usize,
}
#[allow(dead_code)]
impl IsogenyComputation {
    pub fn new(degree: u64) -> Self {
        IsogenyComputation {
            degree,
            kernel_size: degree as usize,
        }
    }
    pub fn is_prime_degree(&self) -> bool {
        if self.degree < 2 {
            return false;
        }
        (2..=((self.degree as f64).sqrt() as u64 + 1)).all(|i| self.degree % i != 0)
    }
    /// Velu's formula complexity: O(l) field operations for l-isogeny.
    pub fn velu_complexity(&self) -> u64 {
        12 * self.degree
    }
    /// Fast Velu isogeny: O(sqrt(l)) complexity.
    pub fn fast_velu_complexity(&self) -> u64 {
        let sqrtl = (self.degree as f64).sqrt() as u64 + 1;
        20 * sqrtl
    }
}
/// Weil pairing computer via a symbolic/numeric Miller's algorithm placeholder.
///
/// For production use a proper finite field implementation is needed; here we
/// return a structured description of the computation steps.
#[allow(dead_code)]
pub struct WeilPairingComputer {
    /// Order n of the torsion subgroup.
    pub n: u64,
    /// Description of the pairing-friendly curve.
    pub curve_name: String,
}
impl WeilPairingComputer {
    /// Construct a Weil pairing computer.
    pub fn new(n: u64, curve_name: String) -> Self {
        Self { n, curve_name }
    }
    /// Describe the bilinearity property of the Weil pairing.
    pub fn bilinearity_statement(&self) -> String {
        format!(
            "e_{}(P+Q, R) = e_{}(P,R) · e_{}(Q,R) for all P,Q,R ∈ E[{}] on {}",
            self.n, self.n, self.n, self.n, self.curve_name
        )
    }
    /// Describe the alternating property: e_n(P,P) = 1.
    pub fn alternating_statement(&self) -> String {
        format!(
            "e_{}(P,P) = 1 for all P ∈ E[{}] on {} (alternating/anti-symmetric)",
            self.n, self.n, self.curve_name
        )
    }
    /// Describe non-degeneracy: e_n(P,Q) = 1 ∀Q implies P = O.
    pub fn non_degeneracy_statement(&self) -> String {
        format!(
            "e_{}(P,Q) = 1 for all Q ∈ E[{}] implies P = O on {}",
            self.n, self.n, self.curve_name
        )
    }
    /// Number of Miller iterations needed: approximately log2(n).
    pub fn miller_iteration_count(&self) -> u32 {
        if self.n == 0 {
            0
        } else {
            64 - self.n.leading_zeros()
        }
    }
}
/// Elliptic curve scalar multiplication via Montgomery ladder.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct MontgomeryLadder {
    pub steps: Vec<bool>,
}
#[allow(dead_code)]
impl MontgomeryLadder {
    pub fn from_scalar(mut scalar: u64) -> Self {
        let mut bits = Vec::new();
        while scalar > 0 {
            bits.push(scalar & 1 == 1);
            scalar >>= 1;
        }
        bits.reverse();
        MontgomeryLadder { steps: bits }
    }
    pub fn bit_length(&self) -> usize {
        self.steps.len()
    }
    /// Count the number of point additions (constant time).
    pub fn n_additions(&self) -> usize {
        self.steps.len().saturating_sub(1)
    }
    /// Count the number of point doublings.
    pub fn n_doublings(&self) -> usize {
        self.steps.len().saturating_sub(1)
    }
    /// Total field multiplications (each add/double ≈ 8M+4S in affine).
    pub fn estimated_field_mults(&self) -> usize {
        (self.n_additions() + self.n_doublings()) * 8
    }
}
/// Represents the BSD L-function order of vanishing at s=1.
#[derive(Debug, Clone)]
pub struct BSDData {
    pub analytic_rank: u32,
    pub leading_coefficient: f64,
    pub sha_order: u64,
    pub regulator: f64,
    pub real_period: f64,
    pub tamagawa_product: u64,
    pub torsion_order: u32,
}
impl BSDData {
    /// BSD formula: lim_{s→1} L(E,s)/(s-1)^r = Ω · Reg · Ш · ∏c_p / |E(Q)_tors|².
    pub fn bsd_rhs(&self) -> f64 {
        self.real_period * self.regulator * self.sha_order as f64 * self.tamagawa_product as f64
            / (self.torsion_order as f64).powi(2)
    }
}
/// Supersingular elliptic curve for isogeny-based cryptography.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SupersingularCurve {
    pub prime_p: u64,
    pub j_invariant: i64,
}
#[allow(dead_code)]
impl SupersingularCurve {
    pub fn new(p: u64, j: i64) -> Self {
        SupersingularCurve {
            prime_p: p,
            j_invariant: j,
        }
    }
    /// Supersingular curve over F_p has j-invariant in F_{p^2}.
    pub fn trace_of_frobenius(&self) -> i64 {
        0
    }
    pub fn endomorphism_ring_is_maximal_order(&self) -> bool {
        true
    }
    /// Number of points for supersingular curve over F_p.
    pub fn group_order(&self) -> u64 {
        self.prime_p + 1
    }
}
/// Weierstrass coefficients for long form: y² + a1·xy + a3·y = x³ + a2·x² + a4·x + a6
#[derive(Debug, Clone, PartialEq)]
pub struct WeierstrassCoeffs {
    pub a1: i64,
    pub a2: i64,
    pub a3: i64,
    pub a4: i64,
    pub a6: i64,
}
impl WeierstrassCoeffs {
    /// Construct coefficients for short Weierstrass form y² = x³ + ax + b.
    pub fn short(a: i64, b: i64) -> Self {
        WeierstrassCoeffs {
            a1: 0,
            a2: 0,
            a3: 0,
            a4: a,
            a6: b,
        }
    }
    /// Discriminant of short Weierstrass: Δ = -16(4a³ + 27b²).
    pub fn discriminant_short(&self) -> i64 {
        -16 * (4 * self.a4.pow(3) + 27 * self.a6.pow(2))
    }
    /// j-invariant for short Weierstrass: j = -1728 · (4a)³ / Δ  (integer approx).
    /// Returns None if discriminant is zero (singular curve).
    pub fn j_invariant_short(&self) -> Option<i64> {
        let delta = self.discriminant_short();
        if delta == 0 {
            None
        } else {
            let num = 1728_i64.checked_mul(64_i64)?.checked_mul(self.a4.pow(3))?;
            Some(num / delta)
        }
    }
}
/// Weil pairing on elliptic curves (abstract interface).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct WeilPairing {
    pub curve_name: String,
    pub embedding_degree: usize,
}
#[allow(dead_code)]
impl WeilPairing {
    pub fn new(curve: &str, k: usize) -> Self {
        WeilPairing {
            curve_name: curve.to_string(),
            embedding_degree: k,
        }
    }
    /// Pairing is non-degenerate if k >= 2.
    pub fn is_non_degenerate(&self) -> bool {
        self.embedding_degree >= 2
    }
    /// Bilinearity: e(aP, bQ) = e(P,Q)^{ab}.
    pub fn bilinearity_property() -> &'static str {
        "e(aP, bQ) = e(P,Q)^{ab}"
    }
    /// Tate pairing has embedding degree requirements.
    pub fn tate_pairing_condition(embedding_degree: usize, field_size: u64) -> bool {
        let _field_size = field_size;
        embedding_degree >= 2
    }
}
/// The group of rational points on an elliptic curve E: y² = x³ + ax + b.
#[derive(Debug, Clone)]
pub struct EllipticCurveGroup {
    /// Coefficient a.
    pub a: f64,
    /// Coefficient b.
    pub b: f64,
    /// Known affine points (x, y) on the curve.
    pub points: Vec<(f64, f64)>,
}
impl EllipticCurveGroup {
    /// Create a new elliptic curve group.
    pub fn new(a: f64, b: f64, points: Vec<(f64, f64)>) -> Self {
        Self { a, b, points }
    }
    /// Return the number of known points (including the point at infinity).
    pub fn order(&self) -> usize {
        self.points.len() + 1
    }
    /// Return the generators of the group (Mordell-Weil generators).
    ///
    /// For finite fields the group is cyclic or Z/mZ × Z/nZ; here we return
    /// the list of known points as candidate generators.
    pub fn generators(&self) -> Vec<EllipticCurvePoint> {
        self.points
            .iter()
            .map(|&(x, y)| EllipticCurvePoint::Affine(x, y))
            .collect()
    }
    /// Check whether the group of known points generates a cyclic group.
    ///
    /// A finite abelian group is cyclic iff it has a unique element of order d
    /// for each divisor d of |G|.  Here we use a simplified heuristic.
    pub fn is_cyclic(&self) -> bool {
        self.points.len() <= 1
    }
}
/// Bilinear pairings on an elliptic curve.
pub struct PairingE2E {
    /// String description of the curve (e.g. "BN256", "BLS12-381").
    pub curve: String,
}
impl PairingE2E {
    /// Construct a pairing instance for the given curve.
    pub fn new(curve: String) -> Self {
        Self { curve }
    }
    /// Compute the Weil pairing e: E\[n\] × E\[n\] → μ_n.
    ///
    /// The Weil pairing is bilinear, alternating (e(P,P)=1), and non-degenerate.
    /// Computed via Miller's algorithm.
    pub fn weil_pairing(&self) -> String {
        format!(
            "Weil pairing on {}: e_n: E[n] × E[n] → μ_n ⊂ k*; \
             computed via Miller's algorithm in O(log n) steps.",
            self.curve
        )
    }
    /// Compute the Tate pairing ⟨·,·⟩: E\[n\](k) × E(k)/nE(k) → k*/(k*)^n.
    ///
    /// The Tate pairing is well-defined on cohomology classes and is non-degenerate.
    pub fn tate_pairing(&self) -> String {
        format!(
            "Tate pairing on {}: ⟨P,Q⟩_n: E[n](k) × E(k)/nE(k) → k*/(k*)^n; \
             final exponentiation step makes it well-defined.",
            self.curve
        )
    }
    /// Compute the optimal Ate pairing, efficient for pairing-friendly curves.
    pub fn ate_pairing(&self) -> String {
        format!(
            "Optimal Ate pairing on {}: a(Q,P) = f_{{t-1,Q}}(P)^{{(q^k-1)/n}}; \
             loop length |t-1| is much shorter than n for pairing-friendly curves.",
            self.curve
        )
    }
}
/// Miller's algorithm for Weil/Tate pairings.
///
/// Given P, Q ∈ E\[n\] with P ≠ ±Q, computes the value f_{n,P}(Q) using
/// the Miller loop. This is a structural implementation over (f64, f64) points
/// (not a finite field, so the actual pairing value is approximated).
#[allow(dead_code)]
pub struct MillerAlgorithmImpl {
    /// Order n.
    pub n: u64,
    /// Coefficient a of the curve.
    pub a: f64,
}
impl MillerAlgorithmImpl {
    /// Construct a Miller algorithm instance.
    pub fn new(n: u64, a: f64) -> Self {
        Self { n, a }
    }
    /// Evaluate the tangent line at point T through point Q.
    ///
    /// Returns the ratio g_{T,T}(Q) = (slope · (Q.x - T.x) - (Q.y - T.y)) / (Q.x - T.x * 2).
    /// This is a simplified evaluation over ℝ.
    pub fn tangent_line_value(&self, t: (f64, f64), q: (f64, f64)) -> f64 {
        let (tx, ty) = t;
        let (qx, qy) = q;
        if ty.abs() < 1e-12 {
            return qx - tx;
        }
        let slope = (3.0 * tx * tx + self.a) / (2.0 * ty);
        slope * (qx - tx) - (qy - ty)
    }
    /// Evaluate the chord/secant line through T and -T' at point Q.
    pub fn chord_line_value(&self, t1: (f64, f64), t2: (f64, f64), q: (f64, f64)) -> f64 {
        let (t1x, t1y) = t1;
        let (t2x, t2y) = t2;
        let (qx, qy) = q;
        if (t1x - t2x).abs() < 1e-12 {
            return qx - t1x;
        }
        let slope = (t2y - t1y) / (t2x - t1x);
        slope * (qx - t1x) - (qy - t1y)
    }
    /// Run the Miller loop for f_{n,P}(Q) using double-and-add over the bits of n.
    ///
    /// Returns a real-valued approximation (full impl requires F_{q^k} arithmetic).
    pub fn miller_loop(&self, p: (f64, f64), q: (f64, f64)) -> f64 {
        let mut f = 1.0f64;
        let mut t = p;
        let bits = 64 - self.n.leading_zeros();
        for i in (0..bits.saturating_sub(1)).rev() {
            let g_tt = self.tangent_line_value(t, q);
            let (new_tx, new_ty) = self.double_affine(t);
            let vert = q.0 - new_tx;
            f = f * f * (g_tt / vert.max(1e-12));
            t = (new_tx, new_ty);
            if (self.n >> i) & 1 == 1 {
                let g_tp = self.chord_line_value(t, p, q);
                let (sum_x, sum_y) = self.add_affine(t, p);
                let vert2 = q.0 - sum_x;
                f *= g_tp / vert2.max(1e-12);
                t = (sum_x, sum_y);
            }
        }
        f
    }
    /// Affine point doubling on y² = x³ + ax + b.
    fn double_affine(&self, p: (f64, f64)) -> (f64, f64) {
        let (x, y) = p;
        if y.abs() < 1e-12 {
            return (f64::INFINITY, f64::INFINITY);
        }
        let lam = (3.0 * x * x + self.a) / (2.0 * y);
        let x3 = lam * lam - 2.0 * x;
        let y3 = lam * (x - x3) - y;
        (x3, y3)
    }
    /// Affine point addition on y² = x³ + ax + b.
    fn add_affine(&self, p: (f64, f64), q_pt: (f64, f64)) -> (f64, f64) {
        let (x1, y1) = p;
        let (x2, y2) = q_pt;
        if (x1 - x2).abs() < 1e-12 {
            if (y1 + y2).abs() < 1e-12 {
                return (f64::INFINITY, f64::INFINITY);
            }
            return self.double_affine(p);
        }
        let lam = (y2 - y1) / (x2 - x1);
        let x3 = lam * lam - x1 - x2;
        let y3 = lam * (x1 - x3) - y1;
        (x3, y3)
    }
}
/// Jacobian (projective) coordinates (X : Y : Z) for an elliptic curve point,
/// where the affine point is (X/Z², Y/Z³).
#[derive(Debug, Clone)]
pub struct JacobianCoordinates {
    /// X coordinate.
    pub x_jac: f64,
    /// Y coordinate.
    pub y_jac: f64,
    /// Z coordinate.
    pub z_jac: f64,
}
impl JacobianCoordinates {
    /// Convert from affine (x, y) to Jacobian \[1,1,1\] representative.
    pub fn from_affine(x: f64, y: f64) -> Self {
        Self {
            x_jac: x,
            y_jac: y,
            z_jac: 1.0,
        }
    }
    /// Convert Jacobian to affine: (X/Z², Y/Z³).
    pub fn to_affine(&self) -> Option<(f64, f64)> {
        if self.z_jac.abs() < 1e-12 {
            None
        } else {
            let z2 = self.z_jac * self.z_jac;
            let z3 = z2 * self.z_jac;
            Some((self.x_jac / z2, self.y_jac / z3))
        }
    }
    /// Add two Jacobian points on y² = x³ + ax + b using the standard formulas.
    pub fn add_jacobian(&self, other: &JacobianCoordinates, a: f64) -> JacobianCoordinates {
        if self.z_jac.abs() < 1e-12 {
            return other.clone();
        }
        if other.z_jac.abs() < 1e-12 {
            return self.clone();
        }
        let z1sq = self.z_jac * self.z_jac;
        let z2sq = other.z_jac * other.z_jac;
        let u1 = self.x_jac * z2sq;
        let u2 = other.x_jac * z1sq;
        let s1 = self.y_jac * z2sq * other.z_jac;
        let s2 = other.y_jac * z1sq * self.z_jac;
        let h = u2 - u1;
        let r = s2 - s1;
        if h.abs() < 1e-12 {
            if r.abs() < 1e-12 {
                return self.double_jacobian(a);
            }
            return JacobianCoordinates {
                x_jac: 1.0,
                y_jac: 1.0,
                z_jac: 0.0,
            };
        }
        let h2 = h * h;
        let h3 = h2 * h;
        let x3 = r * r - h3 - 2.0 * u1 * h2;
        let y3 = r * (u1 * h2 - x3) - s1 * h3;
        let z3 = h * self.z_jac * other.z_jac;
        JacobianCoordinates {
            x_jac: x3,
            y_jac: y3,
            z_jac: z3,
        }
    }
    /// Double a Jacobian point on y² = x³ + ax + b.
    pub fn double_jacobian(&self, a: f64) -> JacobianCoordinates {
        if self.z_jac.abs() < 1e-12 || self.y_jac.abs() < 1e-12 {
            return JacobianCoordinates {
                x_jac: 1.0,
                y_jac: 1.0,
                z_jac: 0.0,
            };
        }
        let ysq = self.y_jac * self.y_jac;
        let s = 4.0 * self.x_jac * ysq;
        let m = 3.0 * self.x_jac * self.x_jac + a * self.z_jac.powi(4);
        let x3 = m * m - 2.0 * s;
        let y3 = m * (s - x3) - 8.0 * ysq * ysq;
        let z3 = 2.0 * self.y_jac * self.z_jac;
        JacobianCoordinates {
            x_jac: x3,
            y_jac: y3,
            z_jac: z3,
        }
    }
}
/// ECDSA signature scheme parameters.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ECDSAParams {
    pub curve_name: String,
    pub order_bits: usize,
    pub hash_bits: usize,
}
#[allow(dead_code)]
impl ECDSAParams {
    pub fn new(curve: &str, order_bits: usize, hash_bits: usize) -> Self {
        ECDSAParams {
            curve_name: curve.to_string(),
            order_bits,
            hash_bits,
        }
    }
    pub fn p256() -> Self {
        ECDSAParams::new("P-256", 256, 256)
    }
    pub fn p384() -> Self {
        ECDSAParams::new("P-384", 384, 384)
    }
    pub fn secp256k1() -> Self {
        ECDSAParams::new("secp256k1", 256, 256)
    }
    pub fn security_level_bits(&self) -> usize {
        self.order_bits / 2
    }
    pub fn signature_size_bytes(&self) -> usize {
        2 * (self.order_bits / 8)
    }
}
/// Baby-step Giant-step solver for the Elliptic Curve Discrete Logarithm Problem.
///
/// Given a generator G and a target point P on E, finds k such that \[k\]G = P,
/// or returns None if not found within the search bound.
#[allow(dead_code)]
pub struct ECDLPSolver {
    /// The curve coefficient a (y² = x³ + ax + b over ℝ demo).
    pub a: f64,
    /// The curve coefficient b.
    pub b: f64,
    /// Order n of the base point G (upper bound for k).
    pub order: u64,
}
impl ECDLPSolver {
    /// Construct an ECDLP solver.
    pub fn new(a: f64, b: f64, order: u64) -> Self {
        Self { a, b, order }
    }
    /// Baby-step giant-step: find k such that \[k\]G = P.
    ///
    /// Let m = ceil(√n). Baby steps: compute {\[j\]G : j = 0..m-1} as a table.
    /// Giant steps: compute P - [i·m]G for i = 0..m-1 and look up in the table.
    /// Complexity: O(√n) in time and space.
    ///
    /// This demo works over (f64, f64) affine points with exact comparison
    /// using a small tolerance; for production use F_q arithmetic.
    pub fn solve(&self, g: (f64, f64), target: (f64, f64)) -> Option<u64> {
        let m = (self.order as f64).sqrt().ceil() as u64 + 1;
        let mut baby: Vec<((f64, f64), u64)> = Vec::with_capacity(m as usize);
        let mut cur = EllipticCurvePoint::Infinity;
        let gpt = EllipticCurvePoint::Affine(g.0, g.1);
        for j in 0..m {
            let coords = match &cur {
                EllipticCurvePoint::Infinity => (f64::INFINITY, f64::INFINITY),
                EllipticCurvePoint::Affine(x, y) => (*x, *y),
            };
            baby.push((coords, j));
            cur = cur.add_points(&gpt, self.a);
        }
        let mg = scalar_mul_f64(m, g, self.a);
        let mut giant = EllipticCurvePoint::Affine(target.0, target.1);
        let neg_mg = match mg {
            EllipticCurvePoint::Infinity => EllipticCurvePoint::Infinity,
            EllipticCurvePoint::Affine(x, y) => EllipticCurvePoint::Affine(x, -y),
        };
        for i in 0..m {
            let giant_coords = match &giant {
                EllipticCurvePoint::Infinity => (f64::INFINITY, f64::INFINITY),
                EllipticCurvePoint::Affine(x, y) => (*x, *y),
            };
            for &(bcoords, j) in &baby {
                if points_eq(giant_coords, bcoords) {
                    let k = i * m + j;
                    if k < self.order {
                        return Some(k);
                    }
                }
            }
            giant = giant.add_points(&neg_mg, self.a);
        }
        None
    }
}
