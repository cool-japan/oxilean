//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)
use super::functions::*;

/// A fixed-capacity circular (ring) buffer backed by a flat array.
///
/// Supports O(1) push-front, push-back, pop-front, and pop-back.
/// Useful for implementing deques and sliding-window algorithms.
#[allow(dead_code)]
pub struct CircularBuffer {
    data: Vec<i64>,
    head: usize,
    tail: usize,
    len: usize,
    cap: usize,
}
#[allow(dead_code)]
impl CircularBuffer {
    /// Create a new circular buffer with the given capacity.
    pub fn new(cap: usize) -> Self {
        CircularBuffer {
            data: vec![0; cap],
            head: 0,
            tail: 0,
            len: 0,
            cap,
        }
    }
    /// Return the number of elements currently stored.
    pub fn len(&self) -> usize {
        self.len
    }
    /// Return `true` if the buffer contains no elements.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
    /// Return `true` if the buffer is at full capacity.
    pub fn is_full(&self) -> bool {
        self.len == self.cap
    }
    /// Push an element to the back. Returns `false` if the buffer is full.
    pub fn push_back(&mut self, val: i64) -> bool {
        if self.is_full() {
            return false;
        }
        self.data[self.tail] = val;
        self.tail = (self.tail + 1) % self.cap;
        self.len += 1;
        true
    }
    /// Pop an element from the front. Returns `None` if empty.
    pub fn pop_front(&mut self) -> Option<i64> {
        if self.is_empty() {
            return None;
        }
        let val = self.data[self.head];
        self.head = (self.head + 1) % self.cap;
        self.len -= 1;
        Some(val)
    }
    /// Peek at the front element without removing it.
    pub fn front(&self) -> Option<i64> {
        if self.is_empty() {
            None
        } else {
            Some(self.data[self.head])
        }
    }
}
/// A simple sparse table for O(1) range-minimum queries (RMQ).
///
/// Build time: O(n log n). Query time: O(1).
/// Works for idempotent operations (min, max, gcd).
#[allow(dead_code)]
pub struct SparseTable {
    table: Vec<Vec<i64>>,
    log2: Vec<usize>,
    n: usize,
}
#[allow(dead_code)]
impl SparseTable {
    /// Build the sparse table from a slice.
    pub fn build(data: &[i64]) -> Self {
        let n = data.len();
        if n == 0 {
            return SparseTable {
                table: vec![],
                log2: vec![],
                n: 0,
            };
        }
        let mut log2 = vec![0usize; n + 1];
        for i in 2..=n {
            log2[i] = log2[i / 2] + 1;
        }
        let k = log2[n] + 1;
        let mut table = vec![vec![i64::MAX; n]; k];
        table[0][..n].copy_from_slice(data);
        for j in 1..k {
            for i in 0..=n.saturating_sub(1 << j) {
                table[j][i] = table[j - 1][i].min(table[j - 1][i + (1 << (j - 1))]);
            }
        }
        SparseTable { table, log2, n }
    }
    /// Query the minimum over the inclusive range `[lo, hi]`.
    pub fn query_min(&self, lo: usize, hi: usize) -> i64 {
        if lo > hi || hi >= self.n {
            return i64::MAX;
        }
        let j = self.log2[hi - lo + 1];
        self.table[j][lo].min(self.table[j][hi + 1 - (1 << j)])
    }
}
/// Difference array for O(1) range-update, O(n) reconstruction.
///
/// Supports range additions: add `val` to all elements in `[lo, hi]`.
#[allow(dead_code)]
pub struct DifferenceArray {
    diff: Vec<i64>,
}
#[allow(dead_code)]
impl DifferenceArray {
    /// Build a difference array from an initial data slice.
    pub fn build(data: &[i64]) -> Self {
        let n = data.len();
        let mut diff = vec![0i64; n + 1];
        if n > 0 {
            diff[0] = data[0];
        }
        for i in 1..n {
            diff[i] = data[i] - data[i - 1];
        }
        DifferenceArray { diff }
    }
    /// Add `val` to all elements in the inclusive range `[lo, hi]`.
    pub fn range_add(&mut self, lo: usize, hi: usize, val: i64) {
        self.diff[lo] += val;
        if hi + 1 < self.diff.len() {
            self.diff[hi + 1] -= val;
        }
    }
    /// Reconstruct the full array from the difference array.
    pub fn reconstruct(&self) -> Vec<i64> {
        let n = self.diff.len() - 1;
        let mut result = vec![0i64; n];
        if n == 0 {
            return result;
        }
        result[0] = self.diff[0];
        for i in 1..n {
            result[i] = result[i - 1] + self.diff[i];
        }
        result
    }
}
/// Prefix-sum array over a slice of integers.
///
/// Supports O(1) range-sum queries after O(n) construction.
///
/// # Example
/// ```
/// # use oxilean_std::array::PrefixSum;
/// let ps = PrefixSum::build(&[1, 2, 3, 4, 5]);
/// assert_eq!(ps.range_sum(1, 3), 9); // 2 + 3 + 4
/// ```
#[allow(dead_code)]
pub struct PrefixSum {
    prefix: Vec<i64>,
}
#[allow(dead_code)]
impl PrefixSum {
    /// Build a prefix-sum structure from a slice.
    ///
    /// `prefix\[0\] = 0`, `prefix[i+1] = prefix\[i\] + data\[i\]`.
    pub fn build(data: &[i64]) -> Self {
        let mut prefix = vec![0i64; data.len() + 1];
        for (i, &x) in data.iter().enumerate() {
            prefix[i + 1] = prefix[i] + x;
        }
        PrefixSum { prefix }
    }
    /// Query the inclusive range sum `data[lo..=hi]`.
    ///
    /// Returns the sum of elements from index `lo` to `hi` inclusive.
    /// Panics if `lo > hi` or indices are out of range.
    pub fn range_sum(&self, lo: usize, hi: usize) -> i64 {
        self.prefix[hi + 1] - self.prefix[lo]
    }
    /// Number of original data elements.
    pub fn len(&self) -> usize {
        self.prefix.len() - 1
    }
    /// Returns `true` if there are no data elements.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
