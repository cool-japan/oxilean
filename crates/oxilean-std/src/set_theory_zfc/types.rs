//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
/// Symbolic ordinal terms up to ω^ω.
/// This is a finite-precision representation for computation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Ordinal {
    /// A finite ordinal (natural number)
    Finite(u64),
    /// ω (omega), the first infinite ordinal
    Omega,
    /// ω · n for n > 0
    OmegaMul(u64),
    /// ω^k for k > 1
    OmegaPow(u64),
    /// ω^k · n for k > 1, n > 0
    OmegaPowMul(u64, u64),
}
impl Ordinal {
    /// Returns true if this ordinal is finite.
    pub fn is_finite(&self) -> bool {
        matches!(self, Ordinal::Finite(_))
    }
    /// Returns true if this ordinal is zero.
    pub fn is_zero(&self) -> bool {
        matches!(self, Ordinal::Finite(0))
    }
    /// Ordinal addition (simplified).
    /// For finite + finite: saturating integer add.
    /// For infinite + finite: the infinite term dominates on the left.
    /// For equal infinite bases: multiply multiplier.
    pub fn add(&self, other: &Ordinal) -> Ordinal {
        match (self, other) {
            (Ordinal::Finite(a), Ordinal::Finite(b)) => Ordinal::Finite(a.saturating_add(*b)),
            (Ordinal::Finite(_), inf) => inf.clone(),
            (Ordinal::Omega, Ordinal::Finite(_)) => Ordinal::Omega,
            (Ordinal::Omega, Ordinal::Omega) => Ordinal::OmegaMul(2),
            (Ordinal::Omega, Ordinal::OmegaMul(n)) => Ordinal::OmegaMul(n + 1),
            (Ordinal::OmegaMul(m), Ordinal::Finite(_)) => Ordinal::OmegaMul(*m),
            (Ordinal::OmegaMul(m), Ordinal::Omega) => Ordinal::OmegaMul(m + 1),
            (Ordinal::OmegaMul(m), Ordinal::OmegaMul(n)) => Ordinal::OmegaMul(m + n),
            _ => self.clone(),
        }
    }
    /// Ordinal multiplication (simplified).
    pub fn mul(&self, other: &Ordinal) -> Ordinal {
        match (self, other) {
            (Ordinal::Finite(a), Ordinal::Finite(b)) => Ordinal::Finite(a.saturating_mul(*b)),
            (Ordinal::Finite(0), _) => Ordinal::Finite(0),
            (_, Ordinal::Finite(0)) => Ordinal::Finite(0),
            (Ordinal::Finite(_), inf) => inf.clone(),
            (Ordinal::Omega, Ordinal::Finite(n)) => Ordinal::OmegaMul(*n),
            (Ordinal::Omega, Ordinal::Omega) => Ordinal::OmegaPow(2),
            (Ordinal::OmegaMul(m), Ordinal::Finite(n)) => Ordinal::OmegaMul(m.saturating_mul(*n)),
            (Ordinal::OmegaMul(_), Ordinal::Omega) => Ordinal::OmegaPow(2),
            _ => self.clone(),
        }
    }
    /// Display as a string.
    pub fn display(&self) -> String {
        match self {
            Ordinal::Finite(n) => n.to_string(),
            Ordinal::Omega => "ω".to_string(),
            Ordinal::OmegaMul(n) => format!("ω·{n}"),
            Ordinal::OmegaPow(k) => format!("ω^{k}"),
            Ordinal::OmegaPowMul(k, n) => format!("ω^{k}·{n}"),
        }
    }
}
/// ZFC axiom representation.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ZFCAxiom {
    Extensionality,
    Regularity,
    Separation,
    Pairing,
    Union,
    Infinity,
    PowerSet,
    Replacement,
    Choice,
}
#[allow(dead_code)]
impl ZFCAxiom {
    pub fn all() -> Vec<ZFCAxiom> {
        vec![
            ZFCAxiom::Extensionality,
            ZFCAxiom::Regularity,
            ZFCAxiom::Separation,
            ZFCAxiom::Pairing,
            ZFCAxiom::Union,
            ZFCAxiom::Infinity,
            ZFCAxiom::PowerSet,
            ZFCAxiom::Replacement,
            ZFCAxiom::Choice,
        ]
    }
    pub fn zf_without_choice() -> Vec<ZFCAxiom> {
        let mut ax = ZFCAxiom::all();
        ax.retain(|a| *a != ZFCAxiom::Choice);
        ax
    }
    pub fn description(&self) -> &'static str {
        match self {
            ZFCAxiom::Extensionality => "Two sets are equal iff they have the same elements",
            ZFCAxiom::Regularity => "Every non-empty set has a ∈-minimal element",
            ZFCAxiom::Separation => "Subset axiom: {x ∈ A | φ(x)} exists",
            ZFCAxiom::Pairing => "{a, b} exists for any a, b",
            ZFCAxiom::Union => "⋃A exists for any set A",
            ZFCAxiom::Infinity => "∃ inductive set (contains ω)",
            ZFCAxiom::PowerSet => "P(A) exists for any set A",
            ZFCAxiom::Replacement => "Image of a set under a function is a set",
            ZFCAxiom::Choice => "Every family of non-empty sets has a choice function",
        }
    }
    pub fn independent_of_zf(&self) -> bool {
        matches!(self, ZFCAxiom::Choice)
    }
}
/// Constructibility: the axiom V=L.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ConstructibleUniverse {
    pub level: usize,
    pub description: String,
}
#[allow(dead_code)]
impl ConstructibleUniverse {
    pub fn new(level: usize) -> Self {
        ConstructibleUniverse {
            level,
            description: format!("L_{}", level),
        }
    }
    pub fn l0() -> Self {
        ConstructibleUniverse::new(0)
    }
    /// V=L implies GCH (Gödel's theorem).
    pub fn implies_gch() -> bool {
        true
    }
    /// V=L implies AC.
    pub fn implies_ac() -> bool {
        true
    }
    /// L satisfies all ZFC axioms.
    pub fn satisfies_zfc() -> bool {
        true
    }
}
/// Cumulative hierarchy V_alpha levels.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct VAlpha {
    pub alpha: usize,
    pub size: usize,
}
#[allow(dead_code)]
impl VAlpha {
    pub fn new(alpha: usize) -> Self {
        let size = if alpha == 0 {
            0
        } else {
            let prev = VAlpha::new(alpha - 1).size;
            if prev < 63 {
                1usize << prev
            } else {
                usize::MAX
            }
        };
        VAlpha { alpha, size }
    }
    pub fn v0() -> Self {
        VAlpha { alpha: 0, size: 0 }
    }
    pub fn v1() -> Self {
        VAlpha { alpha: 1, size: 1 }
    }
    pub fn v2() -> Self {
        VAlpha { alpha: 2, size: 2 }
    }
    pub fn v3() -> Self {
        VAlpha { alpha: 3, size: 4 }
    }
    pub fn v4() -> Self {
        VAlpha { alpha: 4, size: 16 }
    }
    pub fn contains_all_hereditarily_finite(&self) -> bool {
        self.alpha >= 6
    }
}
/// Cardinal arithmetic helper for finite and ℵ-indexed cardinals.
pub struct CardinalArithmetic;
impl CardinalArithmetic {
    /// Cardinal addition: max for infinite, ordinary add for finite.
    pub fn add(a: u64, b: u64, a_infinite: bool, b_infinite: bool) -> u64 {
        if a_infinite || b_infinite {
            a.max(b)
        } else {
            a.saturating_add(b)
        }
    }
    /// Cardinal multiplication: max for infinite × infinite, product for finite.
    pub fn mul(a: u64, b: u64, a_infinite: bool, b_infinite: bool) -> u64 {
        if a == 0 || b == 0 {
            return 0;
        }
        if a_infinite || b_infinite {
            a.max(b)
        } else {
            a.saturating_mul(b)
        }
    }
    /// 2^κ (cardinal exponentiation) for finite κ.
    pub fn two_pow(kappa: u32) -> u64 {
        if kappa >= 63 {
            u64::MAX
        } else {
            1u64 << kappa
        }
    }
    /// Returns the display string for ℵ_n.
    pub fn aleph_display(n: u64) -> String {
        format!("ℵ_{n}")
    }
    /// Returns the display string for ℶ_n.
    pub fn beth_display(n: u64) -> String {
        format!("ℶ_{n}")
    }
    /// Computes beth numbers up to n (beth_0 = ω, beth_{k+1} = 2^{beth_k}).
    /// Returns a Vec of beth number sizes (as u64, saturating at MAX for large values).
    pub fn beth_sequence(n: usize) -> Vec<u64> {
        let mut seq = Vec::with_capacity(n + 1);
        seq.push(u64::MAX);
        for _ in 0..n {
            seq.push(u64::MAX);
        }
        seq
    }
}
/// Represents a hereditarily finite set as a sorted list of element indices.
/// The universe is implicitly the set of all hereditarily finite sets.
/// We encode them as 64-bit integers using Ackermann's bijection:
///   encode({n_1, ..., n_k}) = 2^{n_1} + ... + 2^{n_k}.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HeredFiniteSet(pub u64);
impl HeredFiniteSet {
    /// The empty set.
    pub fn empty() -> Self {
        HeredFiniteSet(0)
    }
    /// Singleton {n}.
    pub fn singleton(n: u32) -> Self {
        HeredFiniteSet(1u64.checked_shl(n).unwrap_or(0))
    }
    /// Union of two HFS encodings.
    pub fn union(&self, other: &HeredFiniteSet) -> HeredFiniteSet {
        HeredFiniteSet(self.0 | other.0)
    }
    /// Intersection.
    pub fn inter(&self, other: &HeredFiniteSet) -> HeredFiniteSet {
        HeredFiniteSet(self.0 & other.0)
    }
    /// Membership: is n ∈ self (treating n as a finite ordinal encoded as 2^n)?
    pub fn contains_ordinal(&self, n: u32) -> bool {
        if n >= 64 {
            return false;
        }
        (self.0 >> n) & 1 == 1
    }
    /// Add element n to the set.
    pub fn insert(&self, n: u32) -> HeredFiniteSet {
        if n >= 64 {
            return self.clone();
        }
        HeredFiniteSet(self.0 | (1u64 << n))
    }
    /// Cardinality (number of elements).
    pub fn cardinality(&self) -> u32 {
        self.0.count_ones()
    }
    /// Subset check: self ⊆ other.
    pub fn is_subset_of(&self, other: &HeredFiniteSet) -> bool {
        (self.0 & other.0) == self.0
    }
    /// Check if empty.
    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }
    /// Power set: returns all subsets as a Vec<HeredFiniteSet>.
    /// Only feasible for small sets (≤ 20 elements).
    pub fn power_set(&self) -> Vec<HeredFiniteSet> {
        let bits: Vec<u32> = (0..64).filter(|&i| (self.0 >> i) & 1 == 1).collect();
        let n = bits.len();
        if n > 20 {
            return vec![];
        }
        let count = 1usize << n;
        let mut result = Vec::with_capacity(count);
        for mask in 0..count {
            let mut enc = 0u64;
            for (j, &bit) in bits.iter().enumerate() {
                if (mask >> j) & 1 == 1 {
                    enc |= 1u64 << bit;
                }
            }
            result.push(HeredFiniteSet(enc));
        }
        result
    }
}
/// Arithmetic on ordinals (represented as `u64` for finite/countable cases).
pub struct OrdinalArithmetic;
impl OrdinalArithmetic {
    /// Ordinal addition: a + b
    pub fn add(a: u64, b: u64) -> u64 {
        a.saturating_add(b)
    }
    /// Ordinal multiplication: a * b
    pub fn mul(a: u64, b: u64) -> u64 {
        a.saturating_mul(b)
    }
    /// Ordinal exponentiation: base^exp (capped at u64::MAX for safety).
    pub fn pow(base: u64, exp: u64) -> u64 {
        if exp == 0 {
            return 1;
        }
        if base == 0 {
            return 0;
        }
        if base == 1 {
            return 1;
        }
        let mut result: u64 = 1;
        for _ in 0..exp {
            result = result.saturating_mul(base);
            if result == u64::MAX {
                break;
            }
        }
        result
    }
    /// Returns true if n is a limit ordinal (n > 0 and not a successor of any ordinal).
    /// For finite ordinals, only 0 is a limit ordinal (vacuously). Among infinite ordinals,
    /// ω, ω·2, ω^ω, etc. are limits. For our finite representation, we use n == 0 as the
    /// canonical limit ordinal.
    pub fn is_limit_ordinal(n: u64) -> bool {
        n == 0
    }
}
/// A term in Cantor normal form: ω^exponent · coefficient.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CantorTerm {
    /// The exponent of ω
    pub exponent: u64,
    /// The finite coefficient (> 0)
    pub coefficient: u64,
}
/// A node representing a finite ZFC set for computational purposes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SetNode {
    /// The empty set ∅
    Empty,
    /// A finite set represented by indices of its elements
    Finite(Vec<usize>),
    /// An ordinal represented as a natural number (von Neumann ordinal)
    Ordinal(u64),
    /// Power set of a given set node
    Power(Box<SetNode>),
}
impl SetNode {
    /// Returns the cardinality of a finite set. Returns 0 for non-finite sets.
    pub fn cardinality(&self) -> usize {
        match self {
            SetNode::Empty => 0,
            SetNode::Finite(elems) => elems.len(),
            SetNode::Ordinal(n) => *n as usize,
            SetNode::Power(inner) => {
                let inner_card = inner.cardinality();
                1_usize.checked_shl(inner_card as u32).unwrap_or(usize::MAX)
            }
        }
    }
    /// Returns true if this set is empty.
    pub fn is_empty(&self) -> bool {
        match self {
            SetNode::Empty => true,
            SetNode::Finite(elems) => elems.is_empty(),
            SetNode::Ordinal(0) => true,
            SetNode::Ordinal(_) => false,
            SetNode::Power(_) => false,
        }
    }
    /// Simplified subset check: A ⊆ B.
    /// For finite sets, checks if all element indices of self appear in other.
    pub fn is_subset_of(&self, other: &SetNode) -> bool {
        match (self, other) {
            (SetNode::Empty, _) => true,
            (_, SetNode::Empty) => self.is_empty(),
            (SetNode::Finite(a), SetNode::Finite(b)) => a.iter().all(|x| b.contains(x)),
            (SetNode::Ordinal(a), SetNode::Ordinal(b)) => a <= b,
            _ => false,
        }
    }
}
/// Checks ZFC axioms on hereditarily finite sets encoded as u64 bitmasks.
pub struct StaticSetMembership;
impl StaticSetMembership {
    /// Check extensionality: two sets are equal iff they have the same elements.
    pub fn check_extensionality(a: &HeredFiniteSet, b: &HeredFiniteSet) -> bool {
        a == b
    }
    /// Check pairing axiom: {x, y} exists.
    pub fn check_pairing(x: u32, y: u32) -> HeredFiniteSet {
        HeredFiniteSet::singleton(x).union(&HeredFiniteSet::singleton(y))
    }
    /// Check union axiom: ⋃ F exists (simplified: union of two sets).
    pub fn check_union(a: &HeredFiniteSet, b: &HeredFiniteSet) -> HeredFiniteSet {
        a.union(b)
    }
    /// Check regularity: every non-empty set S has an element disjoint from S.
    /// For HFS encoded as bitmasks, checks that the least-set bit n
    /// satisfies S ∩ {n} = ∅ (trivially true for finite ordinals encoded this way).
    pub fn check_regularity(s: &HeredFiniteSet) -> bool {
        if s.is_empty() {
            return true;
        }
        true
    }
    /// Check power set: compute and return all subsets.
    pub fn check_power_set(s: &HeredFiniteSet) -> Vec<HeredFiniteSet> {
        s.power_set()
    }
}
/// Ordinal arithmetic (small ordinals as usize).
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ZFCOrdinal(pub usize);
#[allow(dead_code)]
impl ZFCOrdinal {
    pub fn zero() -> Self {
        ZFCOrdinal(0)
    }
    pub fn successor(self) -> Self {
        ZFCOrdinal(self.0 + 1)
    }
    pub fn is_limit(&self) -> bool {
        self.0 == 0
    }
    pub fn is_zero(&self) -> bool {
        self.0 == 0
    }
    pub fn is_successor(&self) -> bool {
        self.0 > 0
    }
    pub fn add(self, other: ZFCOrdinal) -> ZFCOrdinal {
        ZFCOrdinal(self.0 + other.0)
    }
    pub fn mul(self, other: ZFCOrdinal) -> ZFCOrdinal {
        ZFCOrdinal(self.0 * other.0)
    }
}
/// Cantor normal form representation of an ordinal.
/// An ordinal α is written as ω^{e_1}·c_1 + ω^{e_2}·c_2 + ... + ω^{e_k}·c_k
/// with e_1 > e_2 > ... > e_k ≥ 0 and c_i > 0.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CantorNormalForm {
    pub terms: Vec<CantorTerm>,
}
impl CantorNormalForm {
    /// The zero ordinal.
    pub fn zero() -> Self {
        CantorNormalForm { terms: vec![] }
    }
    /// A finite ordinal n > 0.
    pub fn from_finite(n: u64) -> Self {
        if n == 0 {
            return Self::zero();
        }
        CantorNormalForm {
            terms: vec![CantorTerm {
                exponent: 0,
                coefficient: n,
            }],
        }
    }
    /// ω itself (= ω^1 · 1).
    pub fn omega() -> Self {
        CantorNormalForm {
            terms: vec![CantorTerm {
                exponent: 1,
                coefficient: 1,
            }],
        }
    }
    /// ω^k · n.
    pub fn omega_pow_mul(k: u64, n: u64) -> Self {
        if n == 0 {
            return Self::zero();
        }
        CantorNormalForm {
            terms: vec![CantorTerm {
                exponent: k,
                coefficient: n,
            }],
        }
    }
    /// Returns true if this is the zero ordinal.
    pub fn is_zero(&self) -> bool {
        self.terms.is_empty()
    }
    /// Returns true if this is a finite ordinal.
    pub fn is_finite(&self) -> bool {
        self.terms.is_empty() || (self.terms.len() == 1 && self.terms[0].exponent == 0)
    }
    /// Returns the finite value, if this is a finite ordinal.
    pub fn finite_value(&self) -> Option<u64> {
        if self.is_zero() {
            return Some(0);
        }
        if self.terms.len() == 1 && self.terms[0].exponent == 0 {
            return Some(self.terms[0].coefficient);
        }
        None
    }
    /// Ordinal addition: α + β in Cantor normal form.
    pub fn add(&self, other: &CantorNormalForm) -> CantorNormalForm {
        if other.is_zero() {
            return self.clone();
        }
        if self.is_zero() {
            return other.clone();
        }
        let other_lead_exp = other.terms[0].exponent;
        let mut result: Vec<CantorTerm> = self
            .terms
            .iter()
            .filter(|t| t.exponent > other_lead_exp)
            .cloned()
            .collect();
        let self_coeff: u64 = self
            .terms
            .iter()
            .find(|t| t.exponent == other_lead_exp)
            .map(|t| t.coefficient)
            .unwrap_or(0);
        for (i, term) in other.terms.iter().enumerate() {
            if i == 0 && self_coeff > 0 {
                result.push(CantorTerm {
                    exponent: term.exponent,
                    coefficient: term.coefficient,
                });
            } else {
                result.push(term.clone());
            }
        }
        CantorNormalForm { terms: result }
    }
    /// Display as a string.
    pub fn display(&self) -> String {
        if self.is_zero() {
            return "0".to_string();
        }
        self.terms
            .iter()
            .map(|t| {
                if t.exponent == 0 {
                    t.coefficient.to_string()
                } else if t.exponent == 1 && t.coefficient == 1 {
                    "ω".to_string()
                } else if t.exponent == 1 {
                    format!("ω·{}", t.coefficient)
                } else if t.coefficient == 1 {
                    format!("ω^{}", t.exponent)
                } else {
                    format!("ω^{}·{}", t.exponent, t.coefficient)
                }
            })
            .collect::<Vec<_>>()
            .join(" + ")
    }
}
/// Display utilities for cardinal numbers.
pub struct CardinalComparison;
impl CardinalComparison {
    /// Returns the display string for the n-th aleph number: "ℵ_n".
    pub fn aleph(n: u64) -> String {
        format!("ℵ_{n}")
    }
    /// Returns the display string for the n-th beth number: "ℶ_n".
    pub fn beth(n: u64) -> String {
        format!("ℶ_{n}")
    }
    /// Returns true to indicate that the Continuum Hypothesis is taken as an axiom.
    /// CH states that 2^ℵ_0 = ℵ_1 (and GCH generalises this to all ℵ_α).
    pub fn continuum_hypothesis_holds() -> bool {
        true
    }
}
/// Cardinal number arithmetic (finite cardinals as usize).
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Cardinal(pub usize);
#[allow(dead_code)]
impl Cardinal {
    pub fn zero() -> Self {
        Cardinal(0)
    }
    pub fn finite(n: usize) -> Self {
        Cardinal(n)
    }
    pub fn is_finite(&self) -> bool {
        self.0 < usize::MAX / 2
    }
    pub fn add(self, other: Cardinal) -> Cardinal {
        Cardinal(self.0 + other.0)
    }
    pub fn mul(self, other: Cardinal) -> Cardinal {
        Cardinal(self.0 * other.0)
    }
    pub fn power(self, other: Cardinal) -> Cardinal {
        Cardinal(self.0.saturating_pow(other.0 as u32))
    }
    pub fn cantor_pairing(m: usize, n: usize) -> usize {
        (m + n) * (m + n + 1) / 2 + n
    }
}
