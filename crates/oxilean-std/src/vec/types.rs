//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)
use super::functions::*;

/// Difference list: a representation for O(1)-append list concatenation.
#[allow(dead_code)]
pub struct DList<T: 'static> {
    run: Box<dyn Fn(Vec<T>) -> Vec<T>>,
    size_hint: usize,
}
impl<T: Clone + 'static> DList<T> {
    /// Create an empty DList.
    #[allow(dead_code)]
    pub fn empty() -> Self {
        Self {
            run: Box::new(|rest| rest),
            size_hint: 0,
        }
    }
    /// Create a singleton DList.
    #[allow(dead_code)]
    pub fn singleton(x: T) -> Self {
        Self {
            run: Box::new(move |mut rest| {
                rest.insert(0, x.clone());
                rest
            }),
            size_hint: 1,
        }
    }
    /// Convert to a Vec.
    #[allow(dead_code)]
    pub fn to_vec(self) -> Vec<T> {
        (self.run)(Vec::new())
    }
    /// Return the size hint.
    #[allow(dead_code)]
    pub fn size_hint(&self) -> usize {
        self.size_hint
    }
}
/// A circular buffer backed by a fixed-capacity ring array.
#[allow(dead_code)]
pub struct CircularBuffer<T> {
    data: Vec<T>,
    head: usize,
    tail: usize,
    len: usize,
    capacity: usize,
}
impl<T> CircularBuffer<T> {
    /// Create an empty circular buffer with the given capacity.
    #[allow(dead_code)]
    pub fn new(capacity: usize) -> Self {
        Self {
            data: Vec::with_capacity(capacity),
            head: 0,
            tail: 0,
            len: 0,
            capacity,
        }
    }
    /// Return the number of elements currently in the buffer.
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.len
    }
    /// Return true if the buffer is empty.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
    /// Return the capacity of the buffer.
    #[allow(dead_code)]
    pub fn capacity(&self) -> usize {
        self.capacity
    }
    /// Return true if the buffer is full.
    #[allow(dead_code)]
    pub fn is_full(&self) -> bool {
        self.len == self.capacity
    }
}
/// Prefix scan result container.
#[allow(dead_code)]
pub struct PrefixScan<T> {
    pub values: Vec<T>,
    pub inclusive: bool,
    pub direction: &'static str,
}
impl<T: Clone> PrefixScan<T> {
    /// Create an inclusive prefix scan result.
    #[allow(dead_code)]
    pub fn inclusive(values: Vec<T>) -> Self {
        Self {
            values,
            inclusive: true,
            direction: "left",
        }
    }
    /// Create an exclusive prefix scan result.
    #[allow(dead_code)]
    pub fn exclusive(values: Vec<T>) -> Self {
        Self {
            values,
            inclusive: false,
            direction: "left",
        }
    }
    /// Return the values.
    #[allow(dead_code)]
    pub fn values(&self) -> &[T] {
        &self.values
    }
    /// Return the length of the scan result.
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.values.len()
    }
    /// Return true if empty.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}
/// A Fenwick tree (Binary Indexed Tree) for prefix sums.
#[allow(dead_code)]
pub struct FenwickTree {
    data: Vec<i64>,
    n: usize,
}
impl FenwickTree {
    /// Create a FenwickTree of size n (all zeros).
    #[allow(dead_code)]
    pub fn new(n: usize) -> Self {
        Self {
            data: vec![0; n + 1],
            n,
        }
    }
    /// Add `delta` to position `i` (1-indexed).
    #[allow(dead_code)]
    pub fn update(&mut self, mut i: usize, delta: i64) {
        while i <= self.n {
            self.data[i] += delta;
            i += i & i.wrapping_neg();
        }
    }
    /// Compute prefix sum [1..=i].
    #[allow(dead_code)]
    pub fn query(&self, mut i: usize) -> i64 {
        let mut sum = 0i64;
        while i > 0 {
            sum += self.data[i];
            i -= i & i.wrapping_neg();
        }
        sum
    }
    /// Return the size.
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.n
    }
    /// Return true if empty.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.n == 0
    }
}
/// A sparse vector storing only non-zero/non-default entries.
#[allow(dead_code)]
pub struct SparseVec<T> {
    entries: Vec<(usize, T)>,
    length: usize,
    default_val: T,
}
impl<T: Clone + PartialEq> SparseVec<T> {
    /// Create a sparse vec with given length and default value.
    #[allow(dead_code)]
    pub fn new(length: usize, default_val: T) -> Self {
        Self {
            entries: Vec::new(),
            length,
            default_val,
        }
    }
    /// Set position `i` to `value`.
    #[allow(dead_code)]
    pub fn set(&mut self, i: usize, value: T) {
        if i < self.length {
            if let Some(entry) = self.entries.iter_mut().find(|(idx, _)| *idx == i) {
                entry.1 = value;
            } else {
                self.entries.push((i, value));
            }
        }
    }
    /// Get the value at position `i`.
    #[allow(dead_code)]
    pub fn get(&self, i: usize) -> &T {
        self.entries
            .iter()
            .find(|(idx, _)| *idx == i)
            .map(|(_, v)| v)
            .unwrap_or(&self.default_val)
    }
    /// Return the logical length.
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.length
    }
    /// Return true if logical length is zero.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.length == 0
    }
    /// Return the number of non-default entries.
    #[allow(dead_code)]
    pub fn nnz(&self) -> usize {
        self.entries.len()
    }
}
/// A wrapper around a fixed-length boxed slice, representing a length-checked
/// array.
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FixedVec<T> {
    data: Box<[T]>,
}
impl<T: Clone> FixedVec<T> {
    /// Create a `FixedVec` from a `Vec`.
    pub fn from_vec(v: Vec<T>) -> Self {
        Self {
            data: v.into_boxed_slice(),
        }
    }
    /// Return the length.
    pub fn len(&self) -> usize {
        self.data.len()
    }
    /// Return `true` if empty.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    /// Index into the array, returning `None` for out-of-bounds.
    pub fn get(&self, i: usize) -> Option<&T> {
        self.data.get(i)
    }
    /// Convert back to a `Vec`.
    pub fn to_vec(&self) -> Vec<T> {
        self.data.to_vec()
    }
    /// Iterate over references.
    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.data.iter()
    }
}
