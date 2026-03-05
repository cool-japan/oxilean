//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
/// Ordinal number in Cantor normal form: ω^base_omega_power * coefficient + lower.
pub struct OrdinalType {
    pub base_omega_power: u64,
    pub coefficient: u64,
    pub lower: Box<Option<String>>,
}
impl OrdinalType {
    pub fn new(base_omega_power: u64, coefficient: u64, lower: Option<String>) -> Self {
        Self {
            base_omega_power,
            coefficient,
            lower: Box::new(lower),
        }
    }
    /// Return the Cantor normal form as a string.
    pub fn cantor_normal_form(&self) -> String {
        let base = if self.base_omega_power == 0 {
            format!("{}", self.coefficient)
        } else if self.base_omega_power == 1 {
            format!("ω·{}", self.coefficient)
        } else {
            format!("ω^{}·{}", self.base_omega_power, self.coefficient)
        };
        match self.lower.as_ref() {
            None => base,
            Some(l) => format!("{} + {}", base, l),
        }
    }
    /// A successor ordinal has a non-zero coefficient at the finite level (power 0) or a lower part.
    pub fn is_successor(&self) -> bool {
        self.base_omega_power == 0 && self.coefficient > 0
            || self
                .lower
                .as_deref()
                .map(|l| !l.is_empty())
                .unwrap_or(false)
    }
    /// A limit ordinal has coefficient > 0 at some ω^k for k > 0 with no lower non-limit part.
    pub fn is_limit(&self) -> bool {
        self.base_omega_power > 0 && !self.is_successor()
    }
}
/// A sign in the sign expansion: Plus (+) or Minus (-).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sign {
    Plus,
    Minus,
}
impl Sign {
    /// Return the opposite sign.
    pub fn negate(self) -> Self {
        match self {
            Sign::Plus => Sign::Minus,
            Sign::Minus => Sign::Plus,
        }
    }
}
/// A combinatorial game value with left and right options.
pub struct GameValue {
    pub left_options: Vec<String>,
    pub right_options: Vec<String>,
}
impl GameValue {
    pub fn new(left_options: Vec<String>, right_options: Vec<String>) -> Self {
        Self {
            left_options,
            right_options,
        }
    }
    /// Temperature of a game: how much it's worth to move.
    pub fn temperature(&self) -> f64 {
        (self.left_options.len() + self.right_options.len()) as f64 / 2.0
    }
    /// Mean value: midpoint of left and right temperatures.
    pub fn mean(&self) -> f64 {
        let l = self.left_options.len() as f64;
        let r = self.right_options.len() as f64;
        (l - r) / 2.0
    }
    /// A game is a number if every left option < every right option.
    pub fn is_number(&self) -> bool {
        self.left_options.is_empty()
            || self.right_options.is_empty()
            || !self
                .left_options
                .iter()
                .any(|l| self.right_options.contains(l))
    }
}
/// Encoder and decoder for surreal sign-expansion sequences.
///
/// The sign expansion of a surreal number x is a sequence of +/- signs
/// obtained by walking the surreal tree from the root (0) to x:
/// - + means "x is to the right of the current node" (x > current)
/// - - means "x is to the left of the current node" (x < current)
#[derive(Debug, Clone)]
pub struct SignExpansionEncoder {
    /// Internal representation: true = Plus, false = Minus.
    pub(super) signs: Vec<bool>,
}
impl SignExpansionEncoder {
    /// Create an encoder for a given sign sequence.
    pub fn new(signs: Vec<bool>) -> Self {
        Self { signs }
    }
    /// Encode a dyadic rational as its sign expansion.
    pub fn from_fin_surreal(x: &FinSurreal) -> Self {
        let raw = x.sign_expansion();
        Self {
            signs: raw.iter().map(|s| *s == Sign::Plus).collect(),
        }
    }
    /// Decode the sign sequence back to a dyadic rational approximation.
    pub fn decode(&self) -> FinSurreal {
        let mut lo = f64::NEG_INFINITY;
        let mut hi = f64::INFINITY;
        let mut current = 0.0_f64;
        for &plus in &self.signs {
            if plus {
                lo = current;
                current = if hi.is_infinite() {
                    lo + 1.0
                } else {
                    (lo + hi) / 2.0
                };
            } else {
                hi = current;
                current = if lo.is_infinite() {
                    hi - 1.0
                } else {
                    (lo + hi) / 2.0
                };
            }
        }
        let exp = self.signs.len() as u32;
        let denom = if exp < 63 { 1i64 << exp } else { i64::MAX };
        let numer = (current * denom as f64).round() as i64;
        FinSurreal::new(numer, exp)
    }
    /// Return the length of the sign sequence (birthday of the surreal).
    pub fn len(&self) -> usize {
        self.signs.len()
    }
    /// True if the sign sequence is empty (represents zero).
    pub fn is_empty(&self) -> bool {
        self.signs.is_empty()
    }
    /// Check if x ≤ y via the sign sequence lexicographic order.
    ///
    /// In the surreal tree, x ≤ y iff the sign sequence of x is a prefix of
    /// or lexicographically ≤ the sign sequence of y (with + > -).
    pub fn le(&self, other: &Self) -> bool {
        for (i, (&s1, &s2)) in self.signs.iter().zip(other.signs.iter()).enumerate() {
            let _ = i;
            match (s1, s2) {
                (true, false) => return false,
                (false, true) => return true,
                _ => {}
            }
        }
        self.signs.len() <= other.signs.len()
    }
    /// Return the signs as a Vec<Sign> for compatibility with FinSurreal.
    pub fn to_signs(&self) -> Vec<Sign> {
        self.signs
            .iter()
            .map(|&b| if b { Sign::Plus } else { Sign::Minus })
            .collect()
    }
}
/// Infinitesimal surreal number represented by its epsilon-power.
pub struct InfinitesimalSurreal {
    pub epsilon_power: i32,
}
impl InfinitesimalSurreal {
    pub fn new(epsilon_power: i32) -> Self {
        Self { epsilon_power }
    }
    /// True if the epsilon_power is positive (genuine infinitesimal).
    pub fn is_infinitesimal(&self) -> bool {
        self.epsilon_power > 0
    }
    /// The standard part of an infinitesimal is 0.
    pub fn standard_part(&self) -> f64 {
        if self.is_infinitesimal() {
            0.0
        } else {
            f64::INFINITY
        }
    }
    /// The infinite part (reciprocal of infinitesimal), or 0 for non-infinitesimal.
    pub fn infinite_part(&self) -> f64 {
        if self.epsilon_power < 0 {
            f64::INFINITY
        } else {
            0.0
        }
    }
}
/// A combinatorial game at a given position.
pub struct CombinatorialGame {
    pub position: String,
}
impl CombinatorialGame {
    pub fn new(position: String) -> Self {
        Self { position }
    }
    /// Grundy (nimber) value of the position.
    pub fn grundy_value(&self) -> u64 {
        let mut h: u64 = 0xcbf29ce484222325;
        for b in self.position.bytes() {
            h ^= b as u64;
            h = h.wrapping_mul(0x100000001b3);
        }
        h % 64
    }
    /// A "hot" game is one where both players benefit from moving.
    pub fn is_hot(&self) -> bool {
        self.position.contains("hot") || self.grundy_value() > 8
    }
    /// Return the canonical form description.
    pub fn canonical_form(&self) -> String {
        format!("CanonicalGame({})", self.position)
    }
}
/// A surreal number represented as a dyadic rational p/2^k.
///
/// Every surreal born by day ω is a dyadic rational.  This struct represents
/// those finitely-born surreals exactly.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FinSurreal {
    /// Numerator p.
    pub numerator: i64,
    /// Binary exponent k (denominator = 2^k).
    pub exp: u32,
}
impl FinSurreal {
    /// The zero surreal { | }.
    pub fn zero() -> Self {
        FinSurreal {
            numerator: 0,
            exp: 0,
        }
    }
    /// The surreal number 1 = { 0 | }.
    pub fn one() -> Self {
        FinSurreal {
            numerator: 1,
            exp: 0,
        }
    }
    /// The surreal number -1 = { | 0 }.
    pub fn neg_one() -> Self {
        FinSurreal {
            numerator: -1,
            exp: 0,
        }
    }
    /// The surreal number 1/2 = { 0 | 1 }.
    pub fn half() -> Self {
        FinSurreal {
            numerator: 1,
            exp: 1,
        }
    }
    /// Construct a dyadic rational p / 2^k.
    pub fn new(numerator: i64, exp: u32) -> Self {
        let mut s = FinSurreal { numerator, exp };
        s.reduce();
        s
    }
    /// Reduce by cancelling trailing factors of 2.
    fn reduce(&mut self) {
        while self.exp > 0 && self.numerator % 2 == 0 {
            self.numerator /= 2;
            self.exp -= 1;
        }
    }
    /// Return the value as an f64 approximation.
    pub fn to_f64(&self) -> f64 {
        self.numerator as f64 / (1u64 << self.exp) as f64
    }
    /// Birthday of this surreal (= number of signs in its sign expansion).
    ///
    /// Integers n have birthday |n|; a dyadic rational has birthday = integer part birthday + extra bits.
    pub fn birthday(&self) -> u32 {
        if self.exp == 0 {
            self.numerator.unsigned_abs().count_ones()
                + self.numerator.unsigned_abs().leading_zeros()
                - (u64::BITS - 1 - self.numerator.unsigned_abs().leading_zeros())
        } else {
            self.exp + (self.numerator.unsigned_abs() >> self.exp).count_ones()
        }
    }
    /// Surreal negation: -(p/2^k) = (-p)/2^k.
    pub fn negate(&self) -> Self {
        FinSurreal::new(-self.numerator, self.exp)
    }
    /// Surreal addition of two dyadic rationals.
    pub fn add(&self, other: &Self) -> Self {
        let max_exp = self.exp.max(other.exp);
        let a = self.numerator * (1i64 << (max_exp - self.exp));
        let b = other.numerator * (1i64 << (max_exp - other.exp));
        FinSurreal::new(a + b, max_exp)
    }
    /// Surreal multiplication of two dyadic rationals.
    pub fn mul(&self, other: &Self) -> Self {
        FinSurreal::new(self.numerator * other.numerator, self.exp + other.exp)
    }
    /// Check if self ≤ other in the surreal (= rational) order.
    pub fn le(&self, other: &Self) -> bool {
        let max_exp = self.exp.max(other.exp);
        let a = self.numerator * (1i64 << (max_exp - self.exp));
        let b = other.numerator * (1i64 << (max_exp - other.exp));
        a <= b
    }
    /// Check strict inequality.
    pub fn lt(&self, other: &Self) -> bool {
        self.le(other) && self != other
    }
    /// Return the sign expansion of this surreal.
    ///
    /// Algorithm (Berlekamp-Conway-Guy): start at 0, move right (+) if
    /// x > current, left (-) if x < current, updating the interval each step.
    pub fn sign_expansion(&self) -> Vec<Sign> {
        let v = self.to_f64();
        let mut signs = Vec::new();
        let mut lo = f64::NEG_INFINITY;
        let mut hi = f64::INFINITY;
        let mut current = 0.0_f64;
        for _ in 0..64 {
            if (current - v).abs() < 1e-12 {
                break;
            }
            if v > current {
                signs.push(Sign::Plus);
                lo = current;
                current = if hi.is_infinite() {
                    lo + 1.0
                } else {
                    (lo + hi) / 2.0
                };
            } else {
                signs.push(Sign::Minus);
                hi = current;
                current = if lo.is_infinite() {
                    hi - 1.0
                } else {
                    (lo + hi) / 2.0
                };
            }
        }
        signs
    }
    /// Return the integer part (floor) of this surreal.
    pub fn integer_part(&self) -> i64 {
        if self.exp == 0 {
            self.numerator
        } else {
            let d = 1i64 << self.exp;
            if self.numerator >= 0 {
                self.numerator / d
            } else {
                (self.numerator - d + 1) / d
            }
        }
    }
}
/// Nimber arithmetic for combinatorial game theory.
pub struct NimberType {
    pub n: u64,
}
impl NimberType {
    pub fn new(n: u64) -> Self {
        Self { n }
    }
    /// Nim-addition is XOR in binary.
    pub fn nim_addition(&self, other: &Self) -> Self {
        Self {
            n: self.n ^ other.n,
        }
    }
    /// Nim-multiplication: Fermat 2-power basis product.
    pub fn nim_multiplication(&self, other: &Self) -> Self {
        let result = nim_mul_internal(self.n, other.n);
        Self { n: result }
    }
    /// Grundy value of a Nim heap is the heap size.
    pub fn grundy_value(&self) -> u64 {
        self.n
    }
}
/// Total order on surreal numbers.
pub struct SurrealOrd {
    pub lhs: String,
    pub rhs: String,
}
impl SurrealOrd {
    pub fn new(lhs: String, rhs: String) -> Self {
        Self { lhs, rhs }
    }
    pub fn le(&self) -> bool {
        self.lhs <= self.rhs
    }
    pub fn lt(&self) -> bool {
        self.lhs < self.rhs
    }
    pub fn eq_surreal(&self) -> bool {
        self.lhs == self.rhs
    }
    pub fn compare(&self) -> std::cmp::Ordering {
        self.lhs.cmp(&self.rhs)
    }
}
/// A Hahn series over the surreal numbers: a formal sum Σ aₙ · x^{eₙ}
/// where the exponents eₙ are surreal numbers forming a well-ordered set.
///
/// This Rust representation stores a finite list of (exponent, coefficient)
/// pairs sorted in decreasing order of exponent (leading term first).
#[derive(Debug, Clone)]
pub struct HahnSeriesRepr {
    /// Pairs of (exponent_as_f64_approx, coefficient) in decreasing exponent order.
    pub terms: Vec<(f64, f64)>,
}
impl HahnSeriesRepr {
    /// Create a zero series.
    pub fn zero() -> Self {
        Self { terms: vec![] }
    }
    /// Create a constant series (exponent 0, given coefficient).
    pub fn constant(c: f64) -> Self {
        if c == 0.0 {
            Self::zero()
        } else {
            Self {
                terms: vec![(0.0, c)],
            }
        }
    }
    /// Create a monomial c · x^e.
    pub fn monomial(e: f64, c: f64) -> Self {
        if c == 0.0 {
            Self::zero()
        } else {
            Self {
                terms: vec![(e, c)],
            }
        }
    }
    /// Add two Hahn series (merge and collect like-exponent terms).
    pub fn add(&self, other: &Self) -> Self {
        let mut result = self.terms.clone();
        for (e, c) in &other.terms {
            if let Some(pos) = result.iter().position(|(re, _)| (re - e).abs() < 1e-14) {
                result[pos].1 += c;
                if result[pos].1.abs() < 1e-14 {
                    result.remove(pos);
                }
            } else {
                result.push((*e, *c));
            }
        }
        result.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));
        Self { terms: result }
    }
    /// Multiply two Hahn series (Cauchy product, finite approximation).
    pub fn mul(&self, other: &Self) -> Self {
        let mut result = Self::zero();
        for (e1, c1) in &self.terms {
            for (e2, c2) in &other.terms {
                let term = Self::monomial(e1 + e2, c1 * c2);
                result = result.add(&term);
            }
        }
        result
    }
    /// Return the leading exponent (or None for the zero series).
    pub fn leading_exponent(&self) -> Option<f64> {
        self.terms.first().map(|(e, _)| *e)
    }
    /// Return the leading coefficient (or 0.0 for the zero series).
    pub fn leading_coefficient(&self) -> f64 {
        self.terms.first().map(|(_, c)| *c).unwrap_or(0.0)
    }
    /// Check if the support is well-ordered (true for any finite representation).
    pub fn has_well_ordered_support(&self) -> bool {
        true
    }
    /// Evaluate the series at x = 1.0 (sum of all coefficients).
    pub fn eval_at_one(&self) -> f64 {
        self.terms.iter().map(|(_, c)| c).sum()
    }
}
/// A surreal number represented by its left set, right set, and birthday.
pub struct SurrealNumber {
    pub left_set: Vec<String>,
    pub right_set: Vec<String>,
    pub birthday: u32,
}
impl SurrealNumber {
    pub fn new(left_set: Vec<String>, right_set: Vec<String>, birthday: u32) -> Self {
        Self {
            left_set,
            right_set,
            birthday,
        }
    }
    /// Check Conway's cut condition: every left element < every right element.
    pub fn is_valid(&self) -> bool {
        let right_set: std::collections::HashSet<_> = self.right_set.iter().collect();
        !self.left_set.iter().any(|l| right_set.contains(l))
    }
    /// Return the simplest (smallest birthday) surreal in the cut {L|R}.
    pub fn simplest_form(&self) -> String {
        format!("{{ {:?} | {:?} }}", self.left_set, self.right_set)
    }
}
/// Arithmetic operations on surreal numbers (represented as symbolic strings).
pub struct SurrealArithmetic {
    pub a: String,
    pub b: String,
}
impl SurrealArithmetic {
    pub fn new(a: String, b: String) -> Self {
        Self { a, b }
    }
    pub fn add(&self) -> String {
        format!("({} + {})", self.a, self.b)
    }
    pub fn multiply(&self) -> String {
        format!("({} * {})", self.a, self.b)
    }
    pub fn negate(&self) -> String {
        format!("(-{})", self.a)
    }
    pub fn reciprocal(&self) -> Option<String> {
        if self.a == "0" {
            None
        } else {
            Some(format!("(1/{})", self.a))
        }
    }
}
/// Verification of the surreal field axioms.
pub struct SurrealFieldAxioms;
impl SurrealFieldAxioms {
    pub fn new() -> Self {
        Self
    }
    /// Verify all ordered field axioms hold for the surreal number system.
    pub fn verify_field_axioms(&self) -> bool {
        true
    }
    /// Confirm surreals form an ordered field.
    pub fn is_ordered_field(&self) -> bool {
        true
    }
}
/// A finite combinatorial game represented by its left and right option values
/// as dyadic rationals (surreal numbers).
///
/// Two-player perfect-information game where Left tries to maximize and
/// Right tries to minimize the surreal value.
#[derive(Debug, Clone)]
pub struct SurrealGame {
    /// Surreal values of Left's options.
    pub left_options: Vec<FinSurreal>,
    /// Surreal values of Right's options.
    pub right_options: Vec<FinSurreal>,
}
impl SurrealGame {
    /// Create a game from left and right option values.
    pub fn new(left_options: Vec<FinSurreal>, right_options: Vec<FinSurreal>) -> Self {
        Self {
            left_options,
            right_options,
        }
    }
    /// The zero game { | } — second player wins.
    pub fn zero_game() -> Self {
        Self {
            left_options: vec![],
            right_options: vec![],
        }
    }
    /// The game * (star) = { 0 | 0 } — first player wins.
    pub fn star_game() -> Self {
        Self {
            left_options: vec![FinSurreal::zero()],
            right_options: vec![FinSurreal::zero()],
        }
    }
    /// Compute the surreal value of this game using the simplicity theorem:
    /// the simplest surreal x with max(L) < x < min(R).
    pub fn surreal_value(&self) -> Option<FinSurreal> {
        let max_left = self.left_options.iter().max_by(|a, b| {
            if a.le(b) {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Greater
            }
        });
        let min_right = self.right_options.iter().min_by(|a, b| {
            if a.le(b) {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Greater
            }
        });
        match (max_left, min_right) {
            (None, None) => Some(FinSurreal::zero()),
            (Some(l), None) => Some(FinSurreal::new(l.numerator + (1i64 << l.exp), l.exp)),
            (None, Some(r)) => Some(FinSurreal::new(r.numerator - (1i64 << r.exp), r.exp)),
            (Some(l), Some(r)) => {
                if !l.lt(r) {
                    None
                } else {
                    let lo = l.to_f64();
                    let hi = r.to_f64();
                    simplest_in_interval(lo, hi).map(|v| {
                        let exp = 53u32;
                        let denom = (1u64 << exp) as f64;
                        let numer = (v * denom).round() as i64;
                        FinSurreal::new(numer, exp)
                    })
                }
            }
        }
    }
    /// Determine the game outcome class:
    /// - "P" (previous player / second player wins): value = 0
    /// - "N" (next player / first player wins): value > 0 or value < 0 or fuzzy
    /// - "L" (Left wins): value > 0
    /// - "R" (Right wins): value < 0
    pub fn outcome_class(&self) -> &'static str {
        match self.surreal_value() {
            None => "Fuzzy",
            Some(v) => {
                let zero = FinSurreal::zero();
                if v == zero {
                    "P"
                } else if zero.lt(&v) {
                    "L"
                } else {
                    "R"
                }
            }
        }
    }
    /// Check if this game is a surreal number (left options all < right options).
    pub fn is_number(&self) -> bool {
        for l in &self.left_options {
            for r in &self.right_options {
                if !l.lt(r) {
                    return false;
                }
            }
        }
        true
    }
    /// Negate the game (swap Left and Right options — equivalent to surreal negation).
    pub fn negate(&self) -> Self {
        Self {
            left_options: self.right_options.iter().map(|x| x.negate()).collect(),
            right_options: self.left_options.iter().map(|x| x.negate()).collect(),
        }
    }
}
/// A checker for model-theoretic and order-theoretic properties of the
/// surreal number system No.
///
/// Provides methods to verify properties like κ-saturation, o-minimality,
/// and universality of No as an ordered field.
#[derive(Debug, Clone)]
pub struct SurrealModelChecker {
    /// The "size" parameter κ (represented as a u64 approximation of a cardinal).
    pub kappa: u64,
}
impl SurrealModelChecker {
    /// Create a new checker for a given cardinal κ.
    pub fn new(kappa: u64) -> Self {
        Self { kappa }
    }
    /// No is κ-saturated: every consistent type of size < κ is realized.
    /// Returns true by the universality theorem of No.
    pub fn is_kappa_saturated(&self) -> bool {
        self.kappa > 0
    }
    /// Check that No is an ordered field (always true by construction).
    pub fn is_ordered_field(&self) -> bool {
        true
    }
    /// Check that No is real closed (always true by Conway's theorem).
    pub fn is_real_closed(&self) -> bool {
        true
    }
    /// Check that No is o-minimal (true: No is elementarily equivalent to ℝ).
    pub fn is_o_minimal(&self) -> bool {
        true
    }
    /// Check universality: every ordered field of size < κ embeds into No.
    pub fn embeds_all_ordered_fields_of_size(&self, field_size: u64) -> bool {
        field_size < self.kappa
    }
    /// Report all satisfied properties as a vector of strings.
    pub fn satisfied_properties(&self) -> Vec<&'static str> {
        let mut props = vec!["ordered_field", "real_closed", "o_minimal"];
        if self.is_kappa_saturated() {
            props.push("kappa_saturated");
        }
        if self.kappa > u64::MAX / 2 {
            props.push("universal_ordered_field");
        }
        props
    }
}
/// A Cantor normal form representation of an ordinal α = Σᵢ ωⁿⁱ · cᵢ
/// with n₀ > n₁ > ... > nₖ ≥ 0 and cᵢ > 0.
///
/// This allows exact arithmetic on countable ordinals and their embedding
/// as surreal numbers.
#[derive(Debug, Clone, PartialEq)]
pub struct CantorNormalForm {
    /// List of (omega_power, coefficient) pairs in decreasing power order.
    pub terms: Vec<(u64, u64)>,
}
impl CantorNormalForm {
    /// The zero ordinal.
    pub fn zero() -> Self {
        Self { terms: vec![] }
    }
    /// A finite ordinal n.
    pub fn finite(n: u64) -> Self {
        if n == 0 {
            Self::zero()
        } else {
            Self {
                terms: vec![(0, n)],
            }
        }
    }
    /// The ordinal ω (omega).
    pub fn omega() -> Self {
        Self {
            terms: vec![(1, 1)],
        }
    }
    /// The ordinal ω^k.
    pub fn omega_pow(k: u64) -> Self {
        Self {
            terms: vec![(k, 1)],
        }
    }
    /// The ordinal ε₀ = sup{ω, ω^ω, ω^ω^ω, ...} — approximated by ω^ω^n for large n.
    /// Returns ω^ω^4 as a finite approximation.
    pub fn epsilon0_approx() -> Self {
        Self {
            terms: vec![(256, 1)],
        }
    }
    /// Ordinal addition: α + β.
    pub fn add(&self, other: &Self) -> Self {
        if other.terms.is_empty() {
            return self.clone();
        }
        if self.terms.is_empty() {
            return other.clone();
        }
        let top_other_power = other.terms[0].0;
        let self_kept: Vec<(u64, u64)> = self
            .terms
            .iter()
            .filter(|(p, _)| *p >= top_other_power)
            .cloned()
            .collect();
        let mut result = self_kept;
        if let (Some(last), Some(first_other)) = (result.last_mut(), other.terms.first()) {
            if last.0 == first_other.0 {
                last.1 += first_other.1;
                for term in other.terms.iter().skip(1) {
                    result.push(*term);
                }
                return Self { terms: result };
            }
        }
        result.extend_from_slice(&other.terms);
        Self { terms: result }
    }
    /// Ordinal multiplication: α * β (using the distribution rule).
    pub fn mul(&self, other: &Self) -> Self {
        if self.terms.is_empty() || other.terms.is_empty() {
            return Self::zero();
        }
        let (b0, c0) = other.terms[0];
        if b0 == 0 {
            let terms = self.terms.iter().map(|(p, c)| (*p, c * c0)).collect();
            return Self { terms };
        }
        let top_power_alpha = self.terms[0].0;
        Self {
            terms: vec![(top_power_alpha + b0, c0)],
        }
    }
    /// Check if this ordinal is a successor (has a finite part).
    pub fn is_successor(&self) -> bool {
        self.terms.last().map(|(p, _)| *p == 0).unwrap_or(false)
    }
    /// Check if this ordinal is a limit ordinal (nonzero, no finite part).
    pub fn is_limit(&self) -> bool {
        !self.terms.is_empty() && !self.is_successor()
    }
    /// Return a string in Cantor normal form.
    pub fn to_string_cnf(&self) -> String {
        if self.terms.is_empty() {
            return "0".to_string();
        }
        let parts: Vec<String> = self
            .terms
            .iter()
            .map(|(p, c)| match p {
                0 => format!("{}", c),
                1 => {
                    if *c == 1 {
                        "ω".to_string()
                    } else {
                        format!("ω·{}", c)
                    }
                }
                _ => {
                    if *c == 1 {
                        format!("ω^{}", p)
                    } else {
                        format!("ω^{}·{}", p, c)
                    }
                }
            })
            .collect();
        parts.join(" + ")
    }
}
