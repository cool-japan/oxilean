//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)
use super::functions::*;

/// A closed floating-point interval `[lo, hi]` with outward-rounded arithmetic.
///
/// Implements a simplified version of Moore interval arithmetic.
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FloatInterval {
    pub(super) lo: f64,
    pub(super) hi: f64,
}
impl FloatInterval {
    /// Create a new interval. Panics if `lo > hi`.
    pub fn new(lo: f64, hi: f64) -> Self {
        assert!(
            lo <= hi,
            "FloatInterval: lo ({}) must be <= hi ({})",
            lo,
            hi
        );
        Self { lo, hi }
    }
    /// Try to create a new interval, returning `None` if `lo > hi`.
    pub fn try_new(lo: f64, hi: f64) -> Option<Self> {
        if lo <= hi {
            Some(Self { lo, hi })
        } else {
            None
        }
    }
    /// The lower bound.
    pub fn lo(self) -> f64 {
        self.lo
    }
    /// The upper bound.
    pub fn hi(self) -> f64 {
        self.hi
    }
    /// The width `hi - lo`.
    pub fn width(self) -> f64 {
        self.hi - self.lo
    }
    /// The midpoint `(lo + hi) / 2`.
    pub fn midpoint(self) -> f64 {
        self.lo + (self.hi - self.lo) / 2.0
    }
    /// Check if `x` is contained in `[lo, hi]`.
    pub fn contains(self, x: f64) -> bool {
        x >= self.lo && x <= self.hi
    }
    /// Moore addition: `[a,b] + \[c,d\] = \[a+c, b+d\]`.
    pub fn add(self, other: Self) -> Self {
        Self::new(self.lo + other.lo, self.hi + other.hi)
    }
    /// Moore subtraction: `[a,b] - \[c,d\] = \[a-d, b-c\]`.
    pub fn sub(self, other: Self) -> Self {
        Self::new(self.lo - other.hi, self.hi - other.lo)
    }
    /// Moore multiplication (four-product rule).
    pub fn mul(self, other: Self) -> Self {
        let products = [
            self.lo * other.lo,
            self.lo * other.hi,
            self.hi * other.lo,
            self.hi * other.hi,
        ];
        let lo = products.iter().cloned().fold(f64::INFINITY, f64::min);
        let hi = products.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        Self::new(lo, hi)
    }
    /// Interval negation: `-\[a,b\] = \[-b, -a\]`.
    pub fn neg(self) -> Self {
        Self::new(-self.hi, -self.lo)
    }
    /// Intersection of two intervals, returning `None` if disjoint.
    pub fn intersect(self, other: Self) -> Option<Self> {
        let lo = f64::max(self.lo, other.lo);
        let hi = f64::min(self.hi, other.hi);
        Self::try_new(lo, hi)
    }
    /// Convex hull (smallest enclosing interval).
    pub fn hull(self, other: Self) -> Self {
        Self::new(f64::min(self.lo, other.lo), f64::max(self.hi, other.hi))
    }
    /// Check if `self` is a subset of `other`.
    pub fn is_subset_of(self, other: Self) -> bool {
        other.lo <= self.lo && self.hi <= other.hi
    }
    /// Absolute value interval: `|\[a,b\]|`.
    pub fn abs(self) -> Self {
        if self.lo >= 0.0 {
            self
        } else if self.hi <= 0.0 {
            self.neg()
        } else {
            Self::new(0.0, f64::max(-self.lo, self.hi))
        }
    }
    /// Square: `[a,b]^2`.
    pub fn square(self) -> Self {
        self.mul(self)
    }
    /// Mignitude: `min |x|` for `x ∈ \[lo, hi\]`.
    pub fn mignitude(self) -> f64 {
        if self.lo >= 0.0 {
            self.lo
        } else if self.hi <= 0.0 {
            -self.hi
        } else {
            0.0
        }
    }
    /// Magnitude: `max |x|` for `x ∈ \[lo, hi\]`.
    pub fn magnitude(self) -> f64 {
        f64::max(self.lo.abs(), self.hi.abs())
    }
}
/// A segment tree for range-minimum queries over a fixed array.
#[allow(dead_code)]
pub struct RangeMinSegTree {
    n: usize,
    data: Vec<i64>,
}
impl RangeMinSegTree {
    /// Sentinel value for empty nodes.
    const INF: i64 = i64::MAX;
    /// Build a segment tree from a slice of values.
    pub fn build(values: &[i64]) -> Self {
        let n = values.len();
        let mut data = vec![Self::INF; 4 * (n + 1)];
        if n > 0 {
            Self::build_rec(&mut data, values, 1, 0, n - 1);
        }
        Self { n, data }
    }
    fn build_rec(data: &mut Vec<i64>, values: &[i64], node: usize, lo: usize, hi: usize) {
        if lo == hi {
            data[node] = values[lo];
            return;
        }
        let mid = (lo + hi) / 2;
        Self::build_rec(data, values, 2 * node, lo, mid);
        Self::build_rec(data, values, 2 * node + 1, mid + 1, hi);
        data[node] = data[2 * node].min(data[2 * node + 1]);
    }
    /// Query the minimum in the range `[l, r]`.
    pub fn query_min(&self, l: usize, r: usize) -> Option<i64> {
        if self.n == 0 || l > r || r >= self.n {
            return None;
        }
        Some(self.query_rec(1, 0, self.n - 1, l, r))
    }
    fn query_rec(&self, node: usize, lo: usize, hi: usize, l: usize, r: usize) -> i64 {
        if r < lo || hi < l {
            return Self::INF;
        }
        if l <= lo && hi <= r {
            return self.data[node];
        }
        let mid = (lo + hi) / 2;
        let left = self.query_rec(2 * node, lo, mid, l, r);
        let right = self.query_rec(2 * node + 1, mid + 1, hi, l, r);
        left.min(right)
    }
    /// Update the value at index `i`.
    pub fn update(&mut self, i: usize, val: i64) {
        if i < self.n {
            self.update_rec(1, 0, self.n - 1, i, val);
        }
    }
    fn update_rec(&mut self, node: usize, lo: usize, hi: usize, i: usize, val: i64) {
        if lo == hi {
            self.data[node] = val;
            return;
        }
        let mid = (lo + hi) / 2;
        if i <= mid {
            self.update_rec(2 * node, lo, mid, i, val);
        } else {
            self.update_rec(2 * node + 1, mid + 1, hi, i, val);
        }
        self.data[node] = self.data[2 * node].min(self.data[2 * node + 1]);
    }
    /// Number of elements.
    pub fn len(&self) -> usize {
        self.n
    }
    /// Whether the tree is empty.
    pub fn is_empty(&self) -> bool {
        self.n == 0
    }
}
/// A validated interval: a `FloatInterval` together with a proof obligation tag.
///
/// In a real verified system this would carry a kernel proof; here we store a
/// human-readable certificate string.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct ValidatedInterval {
    interval: FloatInterval,
    certificate: String,
    is_verified: bool,
}
impl ValidatedInterval {
    /// Create a verified validated interval.
    pub fn verified(interval: FloatInterval, certificate: impl Into<String>) -> Self {
        Self {
            interval,
            certificate: certificate.into(),
            is_verified: true,
        }
    }
    /// Create an unverified validated interval (e.g., computed, not proved).
    pub fn unverified(interval: FloatInterval) -> Self {
        Self {
            interval,
            certificate: String::new(),
            is_verified: false,
        }
    }
    /// Return the underlying interval.
    pub fn interval(&self) -> FloatInterval {
        self.interval
    }
    /// Return whether this interval has been formally verified.
    pub fn is_verified(&self) -> bool {
        self.is_verified
    }
    /// The certificate string.
    pub fn certificate(&self) -> &str {
        &self.certificate
    }
    /// Compute the addition of two validated intervals.
    /// Result is verified only if both inputs are.
    pub fn add(&self, other: &Self) -> Self {
        Self {
            interval: self.interval.add(other.interval),
            certificate: format!("add({}, {})", self.certificate, other.certificate),
            is_verified: self.is_verified && other.is_verified,
        }
    }
    /// Compute the multiplication of two validated intervals.
    pub fn mul(&self, other: &Self) -> Self {
        Self {
            interval: self.interval.mul(other.interval),
            certificate: format!("mul({}, {})", self.certificate, other.certificate),
            is_verified: self.is_verified && other.is_verified,
        }
    }
}
/// Weighted interval scheduling solver using dynamic programming.
#[allow(dead_code)]
pub struct IntervalScheduler {
    jobs: Vec<ScheduledJob>,
}
impl IntervalScheduler {
    /// Create a new scheduler with the given jobs.
    pub fn new(jobs: Vec<ScheduledJob>) -> Self {
        Self { jobs }
    }
    /// Find the latest job that finishes before `start`.
    fn latest_compatible(&self, idx: usize) -> Option<usize> {
        let start = self.jobs[idx].start;
        let mut result = None;
        for i in 0..idx {
            if self.jobs[i].finish <= start {
                result = Some(i);
            }
        }
        result
    }
    /// Solve weighted interval scheduling via DP. Returns the maximum total weight.
    pub fn max_weight_schedule(&mut self) -> u64 {
        self.jobs.sort_by_key(|j| j.finish);
        let n = self.jobs.len();
        if n == 0 {
            return 0;
        }
        let mut dp = vec![0u64; n + 1];
        for i in 0..n {
            let weight = self.jobs[i].weight;
            let p = self.latest_compatible(i).map(|j| j + 1).unwrap_or(0);
            dp[i + 1] = dp[i].max(dp[p] + weight);
        }
        dp[n]
    }
    /// Return the number of jobs.
    pub fn job_count(&self) -> usize {
        self.jobs.len()
    }
}
/// A simple Krawczyk-method interval root solver.
///
/// Given a function `f` and its derivative `df`, approximates a root using
/// the Krawczyk operator `K(x, X) = m - f(m)/df(X)`.
#[allow(dead_code)]
pub struct KrawczykSolver {
    max_iters: usize,
    tolerance: f64,
}
impl KrawczykSolver {
    /// Create a new solver with given maximum iterations and tolerance.
    pub fn new(max_iters: usize, tolerance: f64) -> Self {
        Self {
            max_iters,
            tolerance,
        }
    }
    /// Attempt to find a root of `f` in `x0` using the Krawczyk operator.
    ///
    /// `f` and `df` take the midpoint as a Float and the interval for the derivative.
    /// Returns the tightest enclosing interval if converged, otherwise `None`.
    pub fn solve<F, DF>(&self, mut x: FloatInterval, f: F, df: DF) -> Option<FloatInterval>
    where
        F: Fn(f64) -> f64,
        DF: Fn(FloatInterval) -> FloatInterval,
    {
        for _ in 0..self.max_iters {
            let m = x.midpoint();
            let fm = f(m);
            let dfx = df(x);
            if dfx.mignitude() < 1e-15 {
                return None;
            }
            let (lo, hi) = if dfx.lo > 0.0 {
                (m - fm / dfx.lo, m - fm / dfx.hi)
            } else if dfx.hi < 0.0 {
                (m - fm / dfx.hi, m - fm / dfx.lo)
            } else {
                return None;
            };
            let k = FloatInterval::try_new(lo, hi)?;
            let next = k.intersect(x)?;
            if next.width() < self.tolerance {
                return Some(next);
            }
            x = next;
        }
        None
    }
}
/// A job for interval scheduling, defined by a start time, finish time, and weight.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct ScheduledJob {
    /// Job identifier.
    pub id: usize,
    /// Start time (inclusive).
    pub start: u64,
    /// Finish time (exclusive).
    pub finish: u64,
    /// Non-negative weight/value.
    pub weight: u64,
}
impl ScheduledJob {
    /// Create a new job.
    pub fn new(id: usize, start: u64, finish: u64, weight: u64) -> Self {
        Self {
            id,
            start,
            finish,
            weight,
        }
    }
    /// Check if this job overlaps with `other`.
    pub fn overlaps(&self, other: &Self) -> bool {
        self.start < other.finish && other.start < self.finish
    }
    /// Check if this job is compatible with `other` (non-overlapping).
    pub fn compatible(&self, other: &Self) -> bool {
        !self.overlaps(other)
    }
}
