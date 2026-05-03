//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)
use super::functions::*;

/// A net: a function from a directed index set to a topological space.
/// Here modeled as a sequence of real values indexed by integers (simplified).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Net {
    /// Values of the net.
    pub values: Vec<f64>,
    /// Whether the net converges.
    pub converges: bool,
    /// The limit, if it converges.
    pub limit: Option<f64>,
}
#[allow(dead_code)]
impl Net {
    /// Creates a net from values.
    pub fn new(values: Vec<f64>) -> Self {
        Net {
            values,
            converges: false,
            limit: None,
        }
    }
    /// Checks if the net is eventually in the epsilon-ball around l.
    pub fn eventually_in_ball(&self, l: f64, eps: f64) -> bool {
        let n = self.values.len();
        if n == 0 {
            return false;
        }
        let start = n / 2;
        self.values[start..].iter().all(|&x| (x - l).abs() < eps)
    }
    /// Detects convergence by checking Cauchy criterion (for subsequences).
    pub fn detect_convergence(&mut self, tol: f64) {
        let n = self.values.len();
        if n < 4 {
            return;
        }
        let tail = &self.values[n / 2..];
        let avg: f64 = tail.iter().sum::<f64>() / tail.len() as f64;
        let max_dev = tail.iter().map(|&x| (x - avg).abs()).fold(0.0f64, f64::max);
        if max_dev < tol {
            self.converges = true;
            self.limit = Some(avg);
        }
    }
    /// Returns a subnet (every k-th element).
    pub fn subnet(&self, k: usize) -> Net {
        if k == 0 {
            return Net::new(Vec::new());
        }
        let values: Vec<f64> = self.values.iter().cloned().step_by(k).collect();
        Net::new(values)
    }
}
/// Verifies monotonicity of a discrete function given as a sorted list of (input, output) pairs.
///
/// A function f is monotone if i ≤ j implies f(i) ≤ f(j).
pub struct MonotoneFnChecker {
    pairs: Vec<(f64, f64)>,
}
impl MonotoneFnChecker {
    /// Build from a list of (x, f(x)) pairs, which need not be sorted.
    pub fn new(mut pairs: Vec<(f64, f64)>) -> Self {
        pairs.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
        MonotoneFnChecker { pairs }
    }
    /// Return `true` if f is (non-strictly) monotone increasing.
    pub fn is_monotone(&self) -> bool {
        self.pairs.windows(2).all(|w| w[0].1 <= w[1].1)
    }
    /// Return `true` if f is strictly monotone increasing.
    pub fn is_strict_mono(&self) -> bool {
        self.pairs.windows(2).all(|w| w[0].1 < w[1].1)
    }
    /// Return `true` if f is (non-strictly) monotone decreasing (antitone).
    pub fn is_antitone(&self) -> bool {
        self.pairs.windows(2).all(|w| w[0].1 >= w[1].1)
    }
}
/// Compute limsup and liminf of a finite sequence of `f64` values.
///
/// For a finite sequence these correspond to the maximum and minimum,
/// which equals the analytic lim sup / lim inf for eventually-constant sequences.
pub struct LimSupInf {
    values: Vec<f64>,
}
impl LimSupInf {
    /// Create from a sequence of values.
    pub fn new(values: Vec<f64>) -> Self {
        LimSupInf { values }
    }
    /// The limsup: the infimum over n of the supremum of the tail {a_k : k ≥ n}.
    /// For a finite sequence this is the maximum value.
    pub fn limsup(&self) -> Option<f64> {
        self.values.iter().cloned().reduce(f64::max)
    }
    /// The liminf: the supremum over n of the infimum of the tail {a_k : k ≥ n}.
    /// For a finite sequence this is the minimum value.
    pub fn liminf(&self) -> Option<f64> {
        self.values.iter().cloned().reduce(f64::min)
    }
    /// True when limsup = liminf, i.e., the sequence converges.
    pub fn converges(&self) -> bool {
        match (self.limsup(), self.liminf()) {
            (Some(sup), Some(inf)) => (sup - inf).abs() < f64::EPSILON,
            _ => false,
        }
    }
    /// The limit, if the sequence converges.
    pub fn limit(&self) -> Option<f64> {
        if self.converges() {
            self.limsup()
        } else {
            None
        }
    }
}
/// Arithmetic on ordinals represented as Cantor normal form.
///
/// An ordinal is stored as a list of (exponent, coefficient) pairs in
/// decreasing order of exponent.  Exponents and coefficients are `u64`.
///
/// Supported: addition, comparison, and Cantor normal form validity check.
pub struct OrdinalArithmetic {
    /// Cantor normal form: sorted descending by exponent.
    pub cnf: Vec<(u64, u64)>,
}
impl OrdinalArithmetic {
    /// Create from a list of (exponent, coefficient) pairs.
    /// Normalizes by sorting and merging equal exponents.
    pub fn new(mut terms: Vec<(u64, u64)>) -> Self {
        terms.sort_by_key(|b| std::cmp::Reverse(b.0));
        let mut cnf: Vec<(u64, u64)> = Vec::new();
        for (exp, coef) in terms {
            if coef == 0 {
                continue;
            }
            if let Some(last) = cnf.last_mut() {
                if last.0 == exp {
                    last.1 += coef;
                    continue;
                }
            }
            cnf.push((exp, coef));
        }
        OrdinalArithmetic { cnf }
    }
    /// The zero ordinal.
    pub fn zero() -> Self {
        OrdinalArithmetic { cnf: vec![] }
    }
    /// True if this is the zero ordinal.
    pub fn is_zero(&self) -> bool {
        self.cnf.is_empty()
    }
    /// Ordinal addition: α + β.
    /// In Cantor normal form: terms of β with exponent ≥ leading exponent of β
    /// absorb the trailing part of α.
    pub fn add(&self, other: &OrdinalArithmetic) -> OrdinalArithmetic {
        if other.is_zero() {
            return OrdinalArithmetic {
                cnf: self.cnf.clone(),
            };
        }
        if self.is_zero() {
            return OrdinalArithmetic {
                cnf: other.cnf.clone(),
            };
        }
        let lead_other = other.cnf[0].0;
        let mut terms: Vec<(u64, u64)> = self
            .cnf
            .iter()
            .filter(|&&(e, _)| e >= lead_other)
            .copied()
            .collect();
        terms.extend_from_slice(&other.cnf);
        OrdinalArithmetic::new(terms)
    }
    /// Lexicographic comparison (returns std::cmp::Ordering).
    pub fn compare(&self, other: &OrdinalArithmetic) -> std::cmp::Ordering {
        for i in 0..self.cnf.len().max(other.cnf.len()) {
            let (e1, c1) = self.cnf.get(i).copied().unwrap_or((0, 0));
            let (e2, c2) = other.cnf.get(i).copied().unwrap_or((0, 0));
            match e1.cmp(&e2) {
                std::cmp::Ordering::Equal => match c1.cmp(&c2) {
                    std::cmp::Ordering::Equal => continue,
                    other_ord => return other_ord,
                },
                other_ord => return other_ord,
            }
        }
        std::cmp::Ordering::Equal
    }
}
/// A directed set: a preorder with every pair having an upper bound.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DirectedSet {
    /// Elements.
    pub elements: Vec<String>,
    /// Relation: (i, j) means i <= j.
    pub relations: Vec<(usize, usize)>,
}
#[allow(dead_code)]
impl DirectedSet {
    /// Creates a new directed set.
    pub fn new() -> Self {
        DirectedSet {
            elements: Vec::new(),
            relations: Vec::new(),
        }
    }
    /// Adds an element.
    pub fn add_element(&mut self, name: &str) -> usize {
        let idx = self.elements.len();
        self.elements.push(name.to_string());
        self.relations.push((idx, idx));
        idx
    }
    /// Adds a relation i <= j.
    pub fn add_relation(&mut self, i: usize, j: usize) {
        if i < self.elements.len() && j < self.elements.len() {
            self.relations.push((i, j));
        }
    }
    /// Checks if i <= j.
    pub fn leq(&self, i: usize, j: usize) -> bool {
        self.relations.iter().any(|&(a, b)| a == i && b == j)
    }
    /// Finds an upper bound of i and j (first one found).
    pub fn upper_bound(&self, i: usize, j: usize) -> Option<usize> {
        (0..self.elements.len()).find(|&k| self.leq(i, k) && self.leq(j, k))
    }
    /// Checks if the preorder is directed.
    pub fn is_directed(&self) -> bool {
        let n = self.elements.len();
        for i in 0..n {
            for j in 0..n {
                if self.upper_bound(i, j).is_none() {
                    return false;
                }
            }
        }
        true
    }
}
/// Data for a Tychonoff (completely regular) space.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct TychonoffSpaceData {
    /// Description of the space.
    pub description: String,
    /// Whether the space is compact (Hausdorff implies normal).
    pub is_compact: bool,
    /// Whether the space is Lindelöf.
    pub is_lindelof: bool,
    /// Stone-Čech compactification description.
    pub stone_cech: Option<String>,
}
#[allow(dead_code)]
impl TychonoffSpaceData {
    /// Creates Tychonoff space data.
    pub fn new(desc: &str) -> Self {
        TychonoffSpaceData {
            description: desc.to_string(),
            is_compact: false,
            is_lindelof: false,
            stone_cech: None,
        }
    }
    /// Sets the Stone-Čech compactification.
    pub fn with_stone_cech(mut self, beta_x: &str) -> Self {
        self.stone_cech = Some(beta_x.to_string());
        self
    }
    /// Checks if it's paracompact (compact or Lindelöf implies paracompact for Hausdorff).
    pub fn is_paracompact(&self) -> bool {
        self.is_compact || self.is_lindelof
    }
    /// Returns the embedding theorem description.
    pub fn embedding_theorem(&self) -> String {
        "Every Tychonoff space embeds as a dense subspace of its Stone-Čech compactification βX"
            .to_string()
    }
}
/// Operations on vector lattices (Riesz spaces) represented as `f64` vectors.
///
/// A vector lattice is a partially ordered vector space that is also a lattice.
/// The positive part, negative part, and absolute value operations are fundamental.
pub struct VectorLatticeOps {
    /// The elements of the vector lattice (component-wise order).
    pub dim: usize,
}
impl VectorLatticeOps {
    /// Create a vector lattice of the given dimension.
    pub fn new(dim: usize) -> Self {
        VectorLatticeOps { dim }
    }
    /// Component-wise join (pointwise maximum): x ∨ y.
    pub fn join(&self, x: &[f64], y: &[f64]) -> Vec<f64> {
        x.iter().zip(y.iter()).map(|(&a, &b)| a.max(b)).collect()
    }
    /// Component-wise meet (pointwise minimum): x ∧ y.
    pub fn meet(&self, x: &[f64], y: &[f64]) -> Vec<f64> {
        x.iter().zip(y.iter()).map(|(&a, &b)| a.min(b)).collect()
    }
    /// Positive part: x⁺ = x ∨ 0.
    pub fn pos_part(&self, x: &[f64]) -> Vec<f64> {
        let zero = vec![0.0f64; self.dim];
        self.join(x, &zero)
    }
    /// Negative part: x⁻ = (-x) ∨ 0 = -(x ∧ 0).
    pub fn neg_part(&self, x: &[f64]) -> Vec<f64> {
        let neg_x: Vec<f64> = x.iter().map(|&a| -a).collect();
        let zero = vec![0.0f64; self.dim];
        self.join(&neg_x, &zero)
    }
    /// Absolute value (modulus): |x| = x⁺ + x⁻.
    pub fn abs_val(&self, x: &[f64]) -> Vec<f64> {
        x.iter().map(|&a| a.abs()).collect()
    }
    /// Check if x ≤ y component-wise.
    pub fn le(&self, x: &[f64], y: &[f64]) -> bool {
        x.iter().zip(y.iter()).all(|(&a, &b)| a <= b + f64::EPSILON)
    }
    /// Riesz decomposition: verify x⁺ - x⁻ = x and x⁺ ∧ x⁻ = 0.
    pub fn check_riesz_decomposition(&self, x: &[f64]) -> bool {
        let xp = self.pos_part(x);
        let xm = self.neg_part(x);
        let diff: Vec<f64> = xp.iter().zip(xm.iter()).map(|(&a, &b)| a - b).collect();
        let diff_ok = diff
            .iter()
            .zip(x.iter())
            .all(|(&d, &xi)| (d - xi).abs() < 1e-10);
        let meet = self.meet(&xp, &xm);
        let meet_ok = meet.iter().all(|&m| m.abs() < 1e-10);
        diff_ok && meet_ok
    }
}
/// Finite approximation to the Sorgenfrey line topology.
///
/// The Sorgenfrey line uses half-open intervals [a, b) as a basis.
/// On a finite discrete set this is approximated by keeping track of
/// which points lie in [lo, hi) for various pairs (lo, hi).
pub struct SorgenfreyLineTopology {
    /// Sorted grid of rational approximation points.
    pub points: Vec<f64>,
}
impl SorgenfreyLineTopology {
    /// Build from an unsorted collection of points.
    pub fn new(mut points: Vec<f64>) -> Self {
        points.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        points.dedup_by(|a, b| (*a - *b).abs() < f64::EPSILON);
        SorgenfreyLineTopology { points }
    }
    /// Test whether `x` lies in the half-open interval [lo, hi).
    pub fn in_basis_set(lo: f64, hi: f64, x: f64) -> bool {
        x >= lo && x < hi
    }
    /// Return all basis elements [points\[i\], points\[j\]) for i < j.
    pub fn basis_sets(&self) -> Vec<(f64, f64)> {
        let mut result = Vec::new();
        for i in 0..self.points.len() {
            for j in (i + 1)..self.points.len() {
                result.push((self.points[i], self.points[j]));
            }
        }
        result
    }
    /// Count points in the basis set [lo, hi).
    pub fn count_in_basis(&self, lo: f64, hi: f64) -> usize {
        self.points
            .iter()
            .filter(|&&x| Self::in_basis_set(lo, hi, x))
            .count()
    }
    /// Check: in the Sorgenfrey topology, [lo, hi) ∩ [lo', hi') = [max(lo,lo'), min(hi,hi'))
    /// if non-empty, otherwise empty.
    pub fn intersect_basis(lo1: f64, hi1: f64, lo2: f64, hi2: f64) -> Option<(f64, f64)> {
        let lo = lo1.max(lo2);
        let hi = hi1.min(hi2);
        if lo < hi {
            Some((lo, hi))
        } else {
            None
        }
    }
}
/// Generates the basis of open sets for the order topology on a finite ordered set.
///
/// Basis elements are open intervals (a, b) and open rays (-∞, b) and (a, ∞).
pub struct OrderTopologyBasis {
    /// Sorted list of distinct points.
    points: Vec<f64>,
}
impl OrderTopologyBasis {
    /// Build from an unsorted collection of points.
    pub fn new(mut points: Vec<f64>) -> Self {
        points.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        points.dedup_by(|a, b| (*a - *b).abs() < f64::EPSILON);
        OrderTopologyBasis { points }
    }
    /// Return all open intervals (points\[i\], points\[j\]) for i < j.
    pub fn open_intervals(&self) -> Vec<(f64, f64)> {
        let mut result = Vec::new();
        for i in 0..self.points.len() {
            for j in (i + 1)..self.points.len() {
                result.push((self.points[i], self.points[j]));
            }
        }
        result
    }
    /// Return all open rays (a, ∞) for each point a in the set.
    pub fn open_rays_right(&self) -> Vec<f64> {
        self.points.clone()
    }
    /// Return all open rays (-∞, b) for each point b in the set.
    pub fn open_rays_left(&self) -> Vec<f64> {
        self.points.clone()
    }
    /// Test whether a given point lies in the open interval (lo, hi).
    pub fn in_open_interval(lo: f64, hi: f64, x: f64) -> bool {
        lo < x && x < hi
    }
    /// Count the total number of basis elements (intervals + rays).
    pub fn basis_size(&self) -> usize {
        let n = self.points.len();
        (n * n.saturating_sub(1)) / 2 + 2 * n
    }
}
/// A metric space with a compatible partial order.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct OrderedMetricSpace {
    /// Points (as real numbers for simplicity).
    pub points: Vec<f64>,
    /// Whether the order is compatible: x <= y => d(z, x) <= d(z, y) for appropriate z.
    pub order_compatible: bool,
}
#[allow(dead_code)]
impl OrderedMetricSpace {
    /// Creates from a sorted list of points (natural order on R).
    pub fn from_sorted_reals(mut points: Vec<f64>) -> Self {
        points.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        OrderedMetricSpace {
            points,
            order_compatible: true,
        }
    }
    /// Returns the metric d(x, y) = |x - y|.
    pub fn metric(&self, i: usize, j: usize) -> Option<f64> {
        if i < self.points.len() && j < self.points.len() {
            Some((self.points[i] - self.points[j]).abs())
        } else {
            None
        }
    }
    /// Checks the order: points\[i\] <= points\[j\].
    pub fn order_leq(&self, i: usize, j: usize) -> bool {
        if i < self.points.len() && j < self.points.len() {
            self.points[i] <= self.points[j]
        } else {
            false
        }
    }
    /// Finds the infimum of two elements (for real line = min).
    pub fn infimum(&self, i: usize, j: usize) -> Option<f64> {
        if i < self.points.len() && j < self.points.len() {
            Some(self.points[i].min(self.points[j]))
        } else {
            None
        }
    }
    /// Finds the supremum of two elements (for real line = max).
    pub fn supremum(&self, i: usize, j: usize) -> Option<f64> {
        if i < self.points.len() && j < self.points.len() {
            Some(self.points[i].max(self.points[j]))
        } else {
            None
        }
    }
    /// Checks if the space is a Riesz space (vector lattice) — trivially true for R.
    pub fn is_riesz_space(&self) -> bool {
        true
    }
}
/// A real interval with topology data.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RealInterval {
    /// Left bound.
    pub left: IntervalBound,
    /// Right bound.
    pub right: IntervalBound,
}
#[allow(dead_code)]
impl RealInterval {
    /// Creates \[a, b\].
    pub fn closed(a: f64, b: f64) -> Self {
        RealInterval {
            left: IntervalBound::Closed(a),
            right: IntervalBound::Closed(b),
        }
    }
    /// Creates (a, b).
    pub fn open(a: f64, b: f64) -> Self {
        RealInterval {
            left: IntervalBound::Open(a),
            right: IntervalBound::Open(b),
        }
    }
    /// Creates (-∞, ∞).
    pub fn real_line() -> Self {
        RealInterval {
            left: IntervalBound::Unbounded,
            right: IntervalBound::Unbounded,
        }
    }
    /// Checks if x is in this interval.
    pub fn contains(&self, x: f64) -> bool {
        let left_ok = match self.left {
            IntervalBound::Closed(a) => x >= a,
            IntervalBound::Open(a) => x > a,
            IntervalBound::Unbounded => true,
        };
        let right_ok = match self.right {
            IntervalBound::Closed(b) => x <= b,
            IntervalBound::Open(b) => x < b,
            IntervalBound::Unbounded => true,
        };
        left_ok && right_ok
    }
    /// Checks if the interval is compact (closed and bounded).
    pub fn is_compact(&self) -> bool {
        matches!(
            (&self.left, &self.right),
            (IntervalBound::Closed(_), IntervalBound::Closed(_))
        )
    }
    /// Returns the length of the interval.
    pub fn length(&self) -> Option<f64> {
        match (&self.left, &self.right) {
            (
                IntervalBound::Closed(a) | IntervalBound::Open(a),
                IntervalBound::Closed(b) | IntervalBound::Open(b),
            ) => Some(b - a),
            _ => None,
        }
    }
}
/// MacNeille completion of a finite poset.
///
/// Computes the Dedekind-MacNeille completion of a finite poset given by a
/// partial order relation on a set of elements `{0, …, n-1}`.
///
/// The completion is the set of all "closed" sets C where C = C^{↑↓},
/// i.e., C is the down-closure of the up-closure of C.
pub struct MacNeilleCompletion {
    /// Number of elements in the base poset.
    pub n: usize,
    /// Relation matrix: `order\[i\]\[j\]` is true iff i ≤ j.
    pub order: Vec<Vec<bool>>,
}
impl MacNeilleCompletion {
    /// Create from an `n × n` order matrix (reflexive, transitive, antisymmetric).
    pub fn new(order: Vec<Vec<bool>>) -> Self {
        let n = order.len();
        MacNeilleCompletion { n, order }
    }
    /// Upper bounds of S: all elements ≥ every element of S.
    pub fn up_set(&self, s: &[usize]) -> Vec<usize> {
        (0..self.n)
            .filter(|&j| s.is_empty() || s.iter().all(|&i| self.order[i][j]))
            .collect()
    }
    /// Lower bounds of S: all elements ≤ every element of S.
    pub fn down_set(&self, s: &[usize]) -> Vec<usize> {
        (0..self.n)
            .filter(|&i| s.is_empty() || s.iter().all(|&j| self.order[i][j]))
            .collect()
    }
    /// Closure of S: down-set of up-set of S (MacNeille closed set).
    /// By convention, closure(∅) = ∅.
    pub fn closure(&self, s: &[usize]) -> Vec<usize> {
        if s.is_empty() {
            return Vec::new();
        }
        let up = self.up_set(s);
        self.down_set(&up)
    }
    /// Check if S is MacNeille-closed (S = closure(S)).
    pub fn is_closed(&self, s: &[usize]) -> bool {
        let mut sorted_s = s.to_vec();
        sorted_s.sort_unstable();
        let mut c = self.closure(s);
        c.sort_unstable();
        sorted_s == c
    }
    /// Enumerate all MacNeille-closed sets (elements of the completion).
    pub fn all_closed_sets(&self) -> Vec<Vec<usize>> {
        let mut result = Vec::new();
        for mask in 0u32..(1u32 << self.n) {
            let s: Vec<usize> = (0..self.n).filter(|&i| (mask >> i) & 1 == 1).collect();
            if self.is_closed(&s) {
                result.push(s);
            }
        }
        result
    }
}
/// Represents an interval in an ordered set.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum IntervalBound {
    /// Closed bound (inclusive).
    Closed(f64),
    /// Open bound (exclusive).
    Open(f64),
    /// Unbounded.
    Unbounded,
}
/// Represents a Scott-open set in a dcpo modelled on a finite set of `u64` elements.
///
/// A subset U of a dcpo is Scott-open if:
///   1. U is an upper set: x ∈ U and x ≤ y implies y ∈ U.
///   2. U is inaccessible by directed joins: if sup(D) ∈ U then D ∩ U ≠ ∅.
pub struct ScottOpenSet {
    /// The elements of the universe, sorted ascending.
    universe: Vec<u64>,
    /// Membership indicator (same length as `universe`).
    membership: Vec<bool>,
}
impl ScottOpenSet {
    /// Create a new Scott open set from a universe and a membership predicate.
    pub fn new(universe: Vec<u64>, predicate: impl Fn(u64) -> bool) -> Self {
        let membership = universe.iter().map(|&x| predicate(x)).collect();
        ScottOpenSet {
            universe,
            membership,
        }
    }
    /// Test whether element `x` belongs to this set.
    pub fn contains(&self, x: u64) -> bool {
        self.universe
            .iter()
            .position(|&u| u == x)
            .map(|i| self.membership[i])
            .unwrap_or(false)
    }
    /// Verify the upper-set condition: for all x ∈ U and y in the universe with x ≤ y, y ∈ U.
    pub fn is_upper_set(&self) -> bool {
        for (i, &in_set) in self.membership.iter().enumerate() {
            if in_set {
                let x = self.universe[i];
                for (j, &y) in self.universe.iter().enumerate() {
                    if y >= x && !self.membership[j] {
                        return false;
                    }
                }
            }
        }
        true
    }
    /// Return the members of this Scott-open set.
    pub fn members(&self) -> Vec<u64> {
        self.universe
            .iter()
            .zip(self.membership.iter())
            .filter(|(_, &m)| m)
            .map(|(&u, _)| u)
            .collect()
    }
}
/// Simple ordinal arithmetic up to ω^2.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SmallOrdinal {
    /// Ordinal as ω * a + b.
    pub omega_coeff: u64,
    pub finite_part: u64,
}
#[allow(dead_code)]
impl SmallOrdinal {
    /// Creates a finite ordinal n.
    pub fn finite(n: u64) -> Self {
        SmallOrdinal {
            omega_coeff: 0,
            finite_part: n,
        }
    }
    /// Creates ω * a + b.
    pub fn omega_times(a: u64, b: u64) -> Self {
        SmallOrdinal {
            omega_coeff: a,
            finite_part: b,
        }
    }
    /// Creates ω (the first infinite ordinal).
    pub fn omega() -> Self {
        SmallOrdinal {
            omega_coeff: 1,
            finite_part: 0,
        }
    }
    /// Ordinal addition: (ω*a + b) + (ω*c + d).
    pub fn add(&self, other: &SmallOrdinal) -> SmallOrdinal {
        if other.omega_coeff > 0 {
            SmallOrdinal {
                omega_coeff: self.omega_coeff + other.omega_coeff,
                finite_part: other.finite_part,
            }
        } else {
            SmallOrdinal {
                omega_coeff: self.omega_coeff,
                finite_part: self.finite_part + other.finite_part,
            }
        }
    }
    /// Ordinal multiplication: (ω*a + b) * (ω*c + d).
    pub fn mul(&self, other: &SmallOrdinal) -> SmallOrdinal {
        if other.omega_coeff > 0 {
            let leading = if self.omega_coeff > 0 {
                self.omega_coeff
            } else {
                self.finite_part
            };
            SmallOrdinal {
                omega_coeff: leading * other.omega_coeff,
                finite_part: 0,
            }
        } else {
            SmallOrdinal {
                omega_coeff: self.omega_coeff * other.finite_part,
                finite_part: self.finite_part * other.finite_part,
            }
        }
    }
    /// Returns true if this ordinal is a limit ordinal.
    pub fn is_limit(&self) -> bool {
        self.finite_part == 0 && (self.omega_coeff > 0)
    }
    /// Returns true if this ordinal is a successor ordinal.
    pub fn is_successor(&self) -> bool {
        self.finite_part > 0
    }
    /// Returns true if this is zero.
    pub fn is_zero(&self) -> bool {
        self.omega_coeff == 0 && self.finite_part == 0
    }
}
/// A closed interval \[lo, hi\] in an ordered type.
///
/// Supports membership testing and overlap detection.
#[derive(Debug, Clone, PartialEq)]
pub struct OrderedInterval<T: PartialOrd + Clone> {
    /// Lower bound (inclusive).
    pub lo: T,
    /// Upper bound (inclusive).
    pub hi: T,
}
impl<T: PartialOrd + Clone> OrderedInterval<T> {
    /// Create a new interval \[lo, hi\]. Returns `None` if lo > hi.
    pub fn new(lo: T, hi: T) -> Option<Self> {
        if lo <= hi {
            Some(OrderedInterval { lo, hi })
        } else {
            None
        }
    }
    /// Test whether a point `x` lies in \[lo, hi\].
    pub fn contains(&self, x: &T) -> bool {
        &self.lo <= x && x <= &self.hi
    }
    /// Test whether two intervals share at least one point.
    pub fn overlaps(&self, other: &OrderedInterval<T>) -> bool {
        self.lo <= other.hi && other.lo <= self.hi
    }
    /// Return the intersection of two intervals, or `None` if they are disjoint.
    pub fn intersect(&self, other: &OrderedInterval<T>) -> Option<OrderedInterval<T>> {
        let new_lo = if self.lo >= other.lo {
            self.lo.clone()
        } else {
            other.lo.clone()
        };
        let new_hi = if self.hi <= other.hi {
            self.hi.clone()
        } else {
            other.hi.clone()
        };
        OrderedInterval::new(new_lo, new_hi)
    }
}
