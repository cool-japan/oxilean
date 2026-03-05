//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
/// Represents a well-quasi-order (WQO) over a finite carrier.
///
/// An encoding of the WQO property: every infinite sequence has a good pair
/// (indices i < j with carrier[i] ≤ carrier[j]).
#[allow(dead_code)]
pub struct WqoInstance {
    /// The elements of the carrier set (finite approximation).
    pub carrier: Vec<u64>,
    /// The quasi-order as a boolean matrix (le_matrix[i][j] = carrier[i] ≤ carrier[j]).
    pub le_matrix: Vec<Vec<bool>>,
}
#[allow(dead_code)]
impl WqoInstance {
    /// Create a WQO instance from a carrier and a comparator.
    pub fn new(carrier: Vec<u64>, le: impl Fn(u64, u64) -> bool) -> Self {
        let n = carrier.len();
        let mut le_matrix = vec![vec![false; n]; n];
        for i in 0..n {
            for j in 0..n {
                le_matrix[i][j] = le(carrier[i], carrier[j]);
            }
        }
        Self { carrier, le_matrix }
    }
    /// Check if a sequence (given as indices into carrier) has a good pair.
    pub fn has_good_pair(&self, seq: &[usize]) -> bool {
        for i in 0..seq.len() {
            for j in (i + 1)..seq.len() {
                if self.le_matrix[seq[i]][seq[j]] {
                    return true;
                }
            }
        }
        false
    }
    /// Return the first good pair in a sequence, if one exists.
    pub fn find_good_pair(&self, seq: &[usize]) -> Option<(usize, usize)> {
        for i in 0..seq.len() {
            for j in (i + 1)..seq.len() {
                if self.le_matrix[seq[i]][seq[j]] {
                    return Some((i, j));
                }
            }
        }
        None
    }
    /// Number of elements in the carrier.
    pub fn size(&self) -> usize {
        self.carrier.len()
    }
    /// Check if the relation is reflexive.
    pub fn is_reflexive(&self) -> bool {
        (0..self.size()).all(|i| self.le_matrix[i][i])
    }
    /// Check if the relation is transitive.
    pub fn is_transitive(&self) -> bool {
        let n = self.size();
        for i in 0..n {
            for j in 0..n {
                for k in 0..n {
                    if self.le_matrix[i][j] && self.le_matrix[j][k] && !self.le_matrix[i][k] {
                        return false;
                    }
                }
            }
        }
        true
    }
}
/// A finite linear order (totally ordered finite set).
///
/// Elements are represented as indices 0..n, with a permutation defining the order.
#[allow(dead_code)]
pub struct FiniteLinearOrder {
    /// Number of elements.
    pub size: usize,
    /// The permutation defining the order: order[i] is the i-th smallest element.
    pub order: Vec<usize>,
}
#[allow(dead_code)]
impl FiniteLinearOrder {
    /// Create the identity linear order on {0, ..., n-1}.
    pub fn identity(n: usize) -> Self {
        Self {
            size: n,
            order: (0..n).collect(),
        }
    }
    /// Create a linear order from a permutation.
    pub fn from_permutation(perm: Vec<usize>) -> Option<Self> {
        let n = perm.len();
        let mut seen = vec![false; n];
        for &x in &perm {
            if x >= n || seen[x] {
                return None;
            }
            seen[x] = true;
        }
        Some(Self {
            size: n,
            order: perm,
        })
    }
    /// Compare two elements a, b (by their rank in the order).
    pub fn compare(&self, a: usize, b: usize) -> Ordering {
        let rank_a = self.order.iter().position(|&x| x == a);
        let rank_b = self.order.iter().position(|&x| x == b);
        match (rank_a, rank_b) {
            (Some(ra), Some(rb)) => Ordering::from_std(ra.cmp(&rb)),
            _ => Ordering::Equal,
        }
    }
    /// Get the rank (0-indexed position) of an element.
    pub fn rank(&self, a: usize) -> Option<usize> {
        self.order.iter().position(|&x| x == a)
    }
    /// Get the minimum element.
    pub fn min_elem(&self) -> Option<usize> {
        self.order.first().copied()
    }
    /// Get the maximum element.
    pub fn max_elem(&self) -> Option<usize> {
        self.order.last().copied()
    }
    /// Check if the order is well-founded (always true for finite sets).
    pub fn is_well_founded(&self) -> bool {
        true
    }
    /// Reverse the order.
    pub fn reverse_order(&self) -> FiniteLinearOrder {
        let mut rev = self.order.clone();
        rev.reverse();
        FiniteLinearOrder {
            size: self.size,
            order: rev,
        }
    }
}
/// A builder that accumulates multiple comparison results and returns the
/// first non-`Equal` one.
#[derive(Clone, Debug)]
pub struct OrderingBuilder {
    result: Ordering,
}
impl OrderingBuilder {
    /// Begin a new builder (starts as `Equal`).
    pub fn new() -> Self {
        Self {
            result: Ordering::Equal,
        }
    }
    /// Add another comparison step.
    pub fn then(mut self, next: Ordering) -> Self {
        self.result = self.result.then(next);
        self
    }
    /// Add a lazily-evaluated step.
    pub fn then_with<F: FnOnce() -> Ordering>(mut self, f: F) -> Self {
        self.result = self.result.then_with(f);
        self
    }
    /// Add a field comparison.
    pub fn field<T: std::cmp::Ord>(self, a: &T, b: &T) -> Self {
        self.then(cmp(a, b))
    }
    /// Finalise and return the accumulated ordering.
    pub fn build(self) -> Ordering {
        self.result
    }
}
/// A range structure for ordered types.
///
/// Represents the interval [lo, hi] in an ordered set, supporting
/// membership tests, containment, and iteration for integer ranges.
#[allow(dead_code)]
pub struct OrderedRange<T: std::cmp::Ord + Clone> {
    /// Lower bound (inclusive).
    pub lo: T,
    /// Upper bound (inclusive).
    pub hi: T,
}
#[allow(dead_code)]
impl<T: std::cmp::Ord + Clone> OrderedRange<T> {
    /// Create a new range [lo, hi]. Panics if lo > hi.
    pub fn new(lo: T, hi: T) -> Self {
        assert!(lo <= hi, "lower bound must not exceed upper bound");
        Self { lo, hi }
    }
    /// Check if a value is within the range.
    pub fn contains(&self, x: &T) -> bool {
        x >= &self.lo && x <= &self.hi
    }
    /// Check if another range is fully contained within this range.
    pub fn contains_range(&self, other: &OrderedRange<T>) -> bool {
        other.lo >= self.lo && other.hi <= self.hi
    }
    /// Check if two ranges overlap.
    pub fn overlaps(&self, other: &OrderedRange<T>) -> bool {
        self.lo <= other.hi && other.lo <= self.hi
    }
    /// Return the lower bound.
    pub fn lower(&self) -> &T {
        &self.lo
    }
    /// Return the upper bound.
    pub fn upper(&self) -> &T {
        &self.hi
    }
}
/// Represents an ordinal in Cantor Normal Form.
///
/// An ordinal in CNF is written as ω^a₁·c₁ + ω^a₂·c₂ + ... + ω^aₙ·cₙ
/// where a₁ > a₂ > ... > aₙ and cᵢ > 0 are natural number coefficients.
/// We represent exponents as u64 for simplicity.
#[allow(dead_code)]
pub struct OrdinalCnf {
    /// Terms in the CNF representation: (exponent, coefficient).
    /// Stored in decreasing order of exponent.
    pub terms: Vec<(u64, u64)>,
}
#[allow(dead_code)]
impl OrdinalCnf {
    /// The ordinal zero.
    pub fn zero() -> Self {
        Self { terms: vec![] }
    }
    /// A finite ordinal n (= ω^0 · n for n > 0).
    pub fn finite(n: u64) -> Self {
        if n == 0 {
            Self::zero()
        } else {
            Self {
                terms: vec![(0, n)],
            }
        }
    }
    /// The ordinal ω (= ω^1 · 1).
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
    /// Check if this ordinal is zero.
    pub fn is_zero(&self) -> bool {
        self.terms.is_empty()
    }
    /// Check if this ordinal is finite.
    pub fn is_finite(&self) -> bool {
        self.terms.is_empty() || (self.terms.len() == 1 && self.terms[0].0 == 0)
    }
    /// Get the finite value, if this ordinal is finite.
    pub fn as_finite(&self) -> Option<u64> {
        if self.terms.is_empty() {
            Some(0)
        } else if self.terms.len() == 1 && self.terms[0].0 == 0 {
            Some(self.terms[0].1)
        } else {
            None
        }
    }
    /// Add two ordinals in CNF.
    pub fn add(&self, other: &OrdinalCnf) -> OrdinalCnf {
        if other.is_zero() {
            return OrdinalCnf {
                terms: self.terms.clone(),
            };
        }
        if self.is_zero() {
            return OrdinalCnf {
                terms: other.terms.clone(),
            };
        }
        let leading_exp = other.terms[0].0;
        let mut result: Vec<(u64, u64)> = self
            .terms
            .iter()
            .filter(|(e, _)| *e > leading_exp)
            .cloned()
            .collect();
        result.extend_from_slice(&other.terms);
        OrdinalCnf { terms: result }
    }
    /// Compare two ordinals in CNF lexicographically.
    pub fn ord_cmp(&self, other: &OrdinalCnf) -> Ordering {
        for (a, b) in self.terms.iter().zip(other.terms.iter()) {
            match a.0.cmp(&b.0) {
                std::cmp::Ordering::Greater => return Ordering::Greater,
                std::cmp::Ordering::Less => return Ordering::Less,
                std::cmp::Ordering::Equal => match a.1.cmp(&b.1) {
                    std::cmp::Ordering::Greater => return Ordering::Greater,
                    std::cmp::Ordering::Less => return Ordering::Less,
                    std::cmp::Ordering::Equal => {}
                },
            }
        }
        Ordering::from_std(self.terms.len().cmp(&other.terms.len()))
    }
    /// Number of terms in the CNF.
    pub fn num_terms(&self) -> usize {
        self.terms.len()
    }
}
/// A simple table that maps keys to values, sorted by a comparison function.
///
/// This is a lightweight alternative to `BTreeMap` using `Ordering`.
#[allow(dead_code)]
pub struct OrderedTable<K: std::cmp::Ord, V> {
    entries: Vec<(K, V)>,
}
#[allow(dead_code)]
impl<K: std::cmp::Ord, V> OrderedTable<K, V> {
    /// Create an empty table.
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }
    /// Insert a key-value pair.
    pub fn insert(&mut self, key: K, value: V) {
        let pos = self.entries.partition_point(|(k, _)| *k < key);
        self.entries.insert(pos, (key, value));
    }
    /// Look up a key.
    pub fn get(&self, key: &K) -> Option<&V> {
        let pos = self.entries.partition_point(|(k, _)| k < key);
        if pos < self.entries.len() && self.entries[pos].0 == *key {
            Some(&self.entries[pos].1)
        } else {
            None
        }
    }
    /// Remove a key, returning its value.
    pub fn remove(&mut self, key: &K) -> Option<V> {
        let pos = self.entries.partition_point(|(k, _)| k < key);
        if pos < self.entries.len() && self.entries[pos].0 == *key {
            Some(self.entries.remove(pos).1)
        } else {
            None
        }
    }
    /// Number of entries.
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    /// Whether the table is empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
    /// Iterate over entries in sorted order.
    pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
        self.entries.iter().map(|(k, v)| (k, v))
    }
    /// Check if a key exists.
    pub fn contains_key(&self, key: &K) -> bool {
        self.get(key).is_some()
    }
    /// Return all keys in sorted order.
    pub fn keys(&self) -> Vec<&K> {
        self.entries.iter().map(|(k, _)| k).collect()
    }
    /// Return all values in key-sorted order.
    pub fn values(&self) -> Vec<&V> {
        self.entries.iter().map(|(_, v)| v).collect()
    }
}
/// A Lean 4-style ordering value (mirrors `Ordering` in the kernel env).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Ordering {
    /// First < second.
    Less,
    /// First == second.
    Equal,
    /// First > second.
    Greater,
}
impl Ordering {
    /// Reverse the ordering: `Less ↔ Greater`.
    pub fn reverse(self) -> Self {
        match self {
            Ordering::Less => Ordering::Greater,
            Ordering::Equal => Ordering::Equal,
            Ordering::Greater => Ordering::Less,
        }
    }
    /// Lexicographic combinator: if `self == Equal`, return `other`.
    pub fn then(self, other: Ordering) -> Self {
        match self {
            Ordering::Equal => other,
            _ => self,
        }
    }
    /// `then` with a lazily-evaluated second comparison.
    pub fn then_with<F: FnOnce() -> Ordering>(self, f: F) -> Self {
        match self {
            Ordering::Equal => f(),
            _ => self,
        }
    }
    /// `true` if `Less`.
    pub fn is_lt(self) -> bool {
        self == Ordering::Less
    }
    /// `true` if `Equal`.
    pub fn is_eq(self) -> bool {
        self == Ordering::Equal
    }
    /// `true` if `Greater`.
    pub fn is_gt(self) -> bool {
        self == Ordering::Greater
    }
    /// `true` if `Less` or `Equal`.
    pub fn is_le(self) -> bool {
        self != Ordering::Greater
    }
    /// `true` if `Greater` or `Equal`.
    pub fn is_ge(self) -> bool {
        self != Ordering::Less
    }
    /// Convert to a signed integer: `-1`, `0`, or `1`.
    pub fn to_signum(self) -> i32 {
        match self {
            Ordering::Less => -1,
            Ordering::Equal => 0,
            Ordering::Greater => 1,
        }
    }
    /// Construct from a signum integer.
    ///
    /// Negative → `Less`, zero → `Equal`, positive → `Greater`.
    pub fn from_signum(n: i32) -> Self {
        match n.cmp(&0) {
            std::cmp::Ordering::Less => Ordering::Less,
            std::cmp::Ordering::Equal => Ordering::Equal,
            std::cmp::Ordering::Greater => Ordering::Greater,
        }
    }
    /// Convert from `std::cmp::Ordering`.
    pub fn from_std(o: std::cmp::Ordering) -> Self {
        match o {
            std::cmp::Ordering::Less => Ordering::Less,
            std::cmp::Ordering::Equal => Ordering::Equal,
            std::cmp::Ordering::Greater => Ordering::Greater,
        }
    }
    /// Convert to `std::cmp::Ordering`.
    pub fn to_std(self) -> std::cmp::Ordering {
        match self {
            Ordering::Less => std::cmp::Ordering::Less,
            Ordering::Equal => std::cmp::Ordering::Equal,
            Ordering::Greater => std::cmp::Ordering::Greater,
        }
    }
}
/// A Dedekind cut representation over the rationals (using integer pairs p/q).
///
/// A Dedekind cut (L, R) partitions the rationals into a lower set L and
/// upper set R. We represent it by the cut value as a rational number p/q.
#[allow(dead_code)]
pub struct DedekindCutQ {
    /// Numerator of the cut value.
    pub numerator: i64,
    /// Denominator of the cut value (always positive).
    pub denominator: u64,
}
#[allow(dead_code)]
impl DedekindCutQ {
    /// Create a Dedekind cut at p/q (q > 0).
    pub fn new(p: i64, q: u64) -> Self {
        assert!(q > 0, "denominator must be positive");
        Self {
            numerator: p,
            denominator: q,
        }
    }
    /// Check if a rational number r/s is in the lower set.
    pub fn in_lower(&self, r: i64, s: u64) -> bool {
        r * (self.denominator as i64) < self.numerator * (s as i64)
    }
    /// Check if a rational number r/s is in the upper set.
    pub fn in_upper(&self, r: i64, s: u64) -> bool {
        r * (self.denominator as i64) > self.numerator * (s as i64)
    }
    /// Approximate value as f64.
    pub fn as_f64(&self) -> f64 {
        self.numerator as f64 / self.denominator as f64
    }
    /// Compare two cuts.
    pub fn cut_cmp(&self, other: &DedekindCutQ) -> Ordering {
        let lhs = self.numerator * (other.denominator as i64);
        let rhs = other.numerator * (self.denominator as i64);
        Ordering::from_std(lhs.cmp(&rhs))
    }
    /// Return a cut strictly between self and other.
    pub fn mediant(&self, other: &DedekindCutQ) -> DedekindCutQ {
        DedekindCutQ::new(
            self.numerator + other.numerator,
            self.denominator + other.denominator,
        )
    }
}
/// A partial order on a finite set, stored as an adjacency matrix.
#[allow(dead_code)]
pub struct FinitePartialOrder {
    /// Number of elements.
    pub size: usize,
    /// The order relation as a boolean matrix: le[i][j] = (i ≤ j).
    pub le: Vec<Vec<bool>>,
}
#[allow(dead_code)]
impl FinitePartialOrder {
    /// Create the discrete partial order (only x ≤ x for all x).
    pub fn discrete(n: usize) -> Self {
        let mut le = vec![vec![false; n]; n];
        for i in 0..n {
            le[i][i] = true;
        }
        Self { size: n, le }
    }
    /// Check reflexivity: x ≤ x for all x.
    pub fn is_reflexive(&self) -> bool {
        (0..self.size).all(|i| self.le[i][i])
    }
    /// Check antisymmetry: x ≤ y and y ≤ x implies x = y.
    pub fn is_antisymmetric(&self) -> bool {
        for i in 0..self.size {
            for j in 0..self.size {
                if i != j && self.le[i][j] && self.le[j][i] {
                    return false;
                }
            }
        }
        true
    }
    /// Check transitivity: x ≤ y and y ≤ z implies x ≤ z.
    pub fn is_transitive(&self) -> bool {
        let n = self.size;
        for i in 0..n {
            for j in 0..n {
                for k in 0..n {
                    if self.le[i][j] && self.le[j][k] && !self.le[i][k] {
                        return false;
                    }
                }
            }
        }
        true
    }
    /// Check if this is a valid partial order.
    pub fn is_valid(&self) -> bool {
        self.is_reflexive() && self.is_antisymmetric() && self.is_transitive()
    }
    /// Check totality: x ≤ y or y ≤ x for all x, y.
    pub fn is_total(&self) -> bool {
        for i in 0..self.size {
            for j in 0..self.size {
                if !self.le[i][j] && !self.le[j][i] {
                    return false;
                }
            }
        }
        true
    }
    /// Find all maximal elements (elements with nothing strictly above them).
    pub fn maximal_elements(&self) -> Vec<usize> {
        (0..self.size)
            .filter(|&i| (0..self.size).all(|j| !self.le[i][j] || i == j || self.le[j][i]))
            .collect()
    }
    /// Compute the transitive closure using Floyd-Warshall.
    pub fn transitive_closure(&self) -> FinitePartialOrder {
        let n = self.size;
        let mut le = self.le.clone();
        for k in 0..n {
            for i in 0..n {
                for j in 0..n {
                    if le[i][k] && le[k][j] {
                        le[i][j] = true;
                    }
                }
            }
        }
        FinitePartialOrder { size: n, le }
    }
}
