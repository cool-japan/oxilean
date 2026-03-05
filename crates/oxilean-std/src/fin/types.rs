//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)
use super::functions::*;

/// A function from `Fin n` to some type `T`.
#[derive(Debug, Clone)]
pub struct FinFun<T> {
    values: Vec<T>,
}
impl<T: Clone> FinFun<T> {
    /// Create a `FinFun` from a vector of length `n`.
    pub fn from_vec(values: Vec<T>) -> Self {
        FinFun { values }
    }
    /// Create a constant function always returning `val`.
    pub fn constant(bound: usize, val: T) -> Self {
        FinFun {
            values: vec![val; bound],
        }
    }
    /// Create a function from a closure.
    pub fn from_fn(bound: usize, f: impl Fn(Fin) -> T) -> Self {
        let values = (0..bound).map(|i| f(Fin { val: i, bound })).collect();
        FinFun { values }
    }
    /// Apply the function to a `Fin` element.
    pub fn apply(&self, i: Fin) -> Option<&T> {
        self.values.get(i.val)
    }
    /// The domain size.
    pub fn bound(&self) -> usize {
        self.values.len()
    }
    /// Iterate over (Fin i, &T) pairs.
    pub fn iter(&self) -> impl Iterator<Item = (Fin, &T)> {
        let bound = self.values.len();
        self.values
            .iter()
            .enumerate()
            .map(move |(i, v)| (Fin { val: i, bound }, v))
    }
}
/// A permutation of `{0, ..., n-1}` stored as a vector.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FinExtPerm {
    /// The permutation as a map: index i → perm[i].
    pub perm: Vec<usize>,
}
#[allow(dead_code)]
impl FinExtPerm {
    /// Create the identity permutation on n elements.
    pub fn identity(n: usize) -> Self {
        FinExtPerm {
            perm: (0..n).collect(),
        }
    }
    /// Create a permutation from a vector. Returns None if not a valid permutation.
    pub fn from_vec(v: Vec<usize>) -> Option<Self> {
        let n = v.len();
        let mut seen = vec![false; n];
        for &x in &v {
            if x >= n || seen[x] {
                return None;
            }
            seen[x] = true;
        }
        Some(FinExtPerm { perm: v })
    }
    /// The size of the permutation.
    pub fn len(&self) -> usize {
        self.perm.len()
    }
    /// Returns true if the permutation is empty.
    pub fn is_empty(&self) -> bool {
        self.perm.is_empty()
    }
    /// Apply the permutation to index i.
    pub fn apply(&self, i: usize) -> Option<usize> {
        self.perm.get(i).copied()
    }
    /// Compose self ∘ other (apply other first, then self).
    pub fn compose(&self, other: &Self) -> Option<Self> {
        if self.len() != other.len() {
            return None;
        }
        let perm = (0..self.len()).map(|i| self.perm[other.perm[i]]).collect();
        Some(FinExtPerm { perm })
    }
    /// Compute the inverse permutation.
    pub fn inverse(&self) -> Self {
        let mut inv = vec![0usize; self.len()];
        for (i, &j) in self.perm.iter().enumerate() {
            inv[j] = i;
        }
        FinExtPerm { perm: inv }
    }
    /// Compute the sign (parity) of the permutation: +1 for even, -1 for odd.
    pub fn sign(&self) -> i32 {
        let n = self.len();
        let mut visited = vec![false; n];
        let mut sign = 1i32;
        for i in 0..n {
            if !visited[i] {
                let mut cycle_len = 0;
                let mut j = i;
                while !visited[j] {
                    visited[j] = true;
                    j = self.perm[j];
                    cycle_len += 1;
                }
                if cycle_len % 2 == 0 {
                    sign = -sign;
                }
            }
        }
        sign
    }
    /// Count the number of cycles.
    pub fn cycle_count(&self) -> usize {
        let n = self.len();
        let mut visited = vec![false; n];
        let mut count = 0;
        for i in 0..n {
            if !visited[i] {
                count += 1;
                let mut j = i;
                while !visited[j] {
                    visited[j] = true;
                    j = self.perm[j];
                }
            }
        }
        count
    }
    /// Check if this permutation is a derangement (no fixed points).
    pub fn is_derangement(&self) -> bool {
        self.perm.iter().enumerate().all(|(i, &p)| p != i)
    }
    /// Return the order of the permutation (smallest k > 0 with σ^k = id).
    pub fn order(&self) -> usize {
        let n = self.len();
        if n == 0 {
            return 1;
        }
        let mut visited = vec![false; n];
        let mut lcm = 1usize;
        for i in 0..n {
            if !visited[i] {
                let mut cycle_len = 0;
                let mut j = i;
                while !visited[j] {
                    visited[j] = true;
                    j = self.perm[j];
                    cycle_len += 1;
                }
                lcm = lcm_usize(lcm, cycle_len);
            }
        }
        lcm
    }
}
/// A bijection between `Fin m * Fin n` and `Fin (m * n)`.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub struct FinExtProduct {
    /// The left bound m.
    pub m: usize,
    /// The right bound n.
    pub n: usize,
}
#[allow(dead_code)]
impl FinExtProduct {
    /// Create a new product structure.
    pub fn new(m: usize, n: usize) -> Self {
        FinExtProduct { m, n }
    }
    /// The total size m * n.
    pub fn size(&self) -> usize {
        self.m * self.n
    }
    /// Encode (i, j) into Fin (m*n).
    pub fn encode(&self, i: usize, j: usize) -> Option<usize> {
        if i >= self.m || j >= self.n {
            return None;
        }
        Some(i * self.n + j)
    }
    /// Decode index k into (i, j).
    pub fn decode(&self, k: usize) -> Option<(usize, usize)> {
        if self.n == 0 || k >= self.size() {
            return None;
        }
        Some((k / self.n, k % self.n))
    }
    /// Sum over all elements using a function f(i, j) -> u64.
    pub fn sum_over<F: Fn(usize, usize) -> u64>(&self, f: F) -> u64 {
        let mut total = 0u64;
        for i in 0..self.m {
            for j in 0..self.n {
                total += f(i, j);
            }
        }
        total
    }
    /// Row sum: sum f(i, j) for fixed i over all j.
    pub fn row_sum<F: Fn(usize, usize) -> u64>(&self, i: usize, f: F) -> u64 {
        (0..self.n).map(|j| f(i, j)).sum()
    }
    /// Column sum: sum f(i, j) for fixed j over all i.
    pub fn col_sum<F: Fn(usize, usize) -> u64>(&self, j: usize, f: F) -> u64 {
        (0..self.m).map(|i| f(i, j)).sum()
    }
}
/// A bounded integer value in `[0, n)`. Host-side representation of `Fin n`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Fin {
    /// The numeric value.
    pub val: usize,
    /// The (exclusive) upper bound.
    pub bound: usize,
}
impl Fin {
    /// Create a new `Fin` if `val < bound`, otherwise `None`.
    pub fn new(val: usize, bound: usize) -> Option<Self> {
        if val < bound {
            Some(Fin { val, bound })
        } else {
            None
        }
    }
    /// Create a `Fin` representing zero for bound `n > 0`.
    pub fn zero(bound: usize) -> Option<Self> {
        if bound > 0 {
            Some(Fin { val: 0, bound })
        } else {
            None
        }
    }
    /// Create the last element (`bound - 1`) of `Fin bound`.
    pub fn last(bound: usize) -> Option<Self> {
        if bound > 0 {
            Some(Fin {
                val: bound - 1,
                bound,
            })
        } else {
            None
        }
    }
    /// Return the successor, wrapping modulo `bound`.
    pub fn succ_wrap(self) -> Self {
        Fin {
            val: (self.val + 1) % self.bound,
            bound: self.bound,
        }
    }
    /// Return the predecessor, wrapping modulo `bound`.
    pub fn pred_wrap(self) -> Self {
        Fin {
            val: if self.val == 0 {
                self.bound - 1
            } else {
                self.val - 1
            },
            bound: self.bound,
        }
    }
    /// Return the additive inverse (complement): `bound - 1 - val`.
    pub fn complement(self) -> Self {
        Fin {
            val: self.bound - 1 - self.val,
            bound: self.bound,
        }
    }
    /// Add two `Fin` values with the same bound (modular).
    #[allow(clippy::should_implement_trait)]
    pub fn add(self, other: Self) -> Option<Self> {
        if self.bound != other.bound {
            return None;
        }
        Some(Fin {
            val: (self.val + other.val) % self.bound,
            bound: self.bound,
        })
    }
    /// Multiply two `Fin` values with the same bound (modular).
    #[allow(clippy::should_implement_trait)]
    pub fn mul(self, other: Self) -> Option<Self> {
        if self.bound != other.bound {
            return None;
        }
        Some(Fin {
            val: (self.val * other.val) % self.bound,
            bound: self.bound,
        })
    }
    /// Subtract (modular). Returns `None` if bounds differ.
    #[allow(clippy::should_implement_trait)]
    pub fn sub(self, other: Self) -> Option<Self> {
        if self.bound != other.bound {
            return None;
        }
        let v = (self.val + self.bound - other.val) % self.bound;
        Some(Fin {
            val: v,
            bound: self.bound,
        })
    }
    /// Cast into a larger bound (`n ≤ m`).
    pub fn cast(self, new_bound: usize) -> Option<Self> {
        if self.val < new_bound {
            Some(Fin {
                val: self.val,
                bound: new_bound,
            })
        } else {
            None
        }
    }
    /// Return all elements of `Fin n` in order.
    pub fn all(bound: usize) -> Vec<Self> {
        (0..bound).map(|v| Fin { val: v, bound }).collect()
    }
    /// Return true if this is the zero element.
    pub fn is_zero(&self) -> bool {
        self.val == 0
    }
    /// Return true if this is the last element.
    pub fn is_last(&self) -> bool {
        self.val + 1 == self.bound
    }
    /// Embed into `usize`.
    pub fn as_usize(self) -> usize {
        self.val
    }
    /// Embed into a `u64`.
    pub fn as_u64(self) -> u64 {
        self.val as u64
    }
}
/// Young tableau shape: a partition of n stored as a non-increasing sequence.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FinExtYoungShape {
    /// The partition: parts[i] is the length of the i-th row.
    pub parts: Vec<usize>,
}
#[allow(dead_code)]
impl FinExtYoungShape {
    /// Create a Young shape from a partition (must be non-increasing).
    pub fn new(parts: Vec<usize>) -> Option<Self> {
        if parts.windows(2).all(|w| w[0] >= w[1]) {
            let parts: Vec<usize> = parts.into_iter().filter(|&x| x > 0).collect();
            Some(FinExtYoungShape { parts })
        } else {
            None
        }
    }
    /// The total number of cells (size of the partition).
    pub fn size(&self) -> usize {
        self.parts.iter().sum()
    }
    /// Number of rows.
    pub fn rows(&self) -> usize {
        self.parts.len()
    }
    /// Length of row i (0-indexed).
    pub fn row_len(&self, i: usize) -> usize {
        self.parts.get(i).copied().unwrap_or(0)
    }
    /// The conjugate (transpose) partition.
    pub fn conjugate(&self) -> Self {
        if self.parts.is_empty() {
            return FinExtYoungShape { parts: vec![] };
        }
        let max_col = self.parts[0];
        let conj_parts: Vec<usize> = (0..max_col)
            .map(|j| self.parts.iter().filter(|&&r| r > j).count())
            .collect();
        FinExtYoungShape { parts: conj_parts }
    }
    /// Check if this shape is a valid Young diagram (non-increasing rows).
    pub fn is_valid(&self) -> bool {
        self.parts.windows(2).all(|w| w[0] >= w[1])
    }
    /// Count standard Young tableaux using the hook length formula.
    pub fn hook_length_count(&self) -> u64 {
        let n = self.size();
        if n == 0 {
            return 1;
        }
        let factorial_n: u64 = (1..=n as u64).product();
        let mut hook_product = 1u64;
        for (i, &ri) in self.parts.iter().enumerate() {
            for j in 0..ri {
                let arm = ri - j - 1;
                let leg = self.parts[i + 1..].iter().filter(|&&r| r > j).count();
                let hook = arm + leg + 1;
                hook_product *= hook as u64;
            }
        }
        factorial_n / hook_product
    }
}
/// An iterator over all `Fin n` elements.
pub struct FinIter {
    pub(super) current: usize,
    pub(super) bound: usize,
}
impl FinIter {
    /// Create an iterator over `Fin n`.
    pub fn new(bound: usize) -> Self {
        FinIter { current: 0, bound }
    }
}
