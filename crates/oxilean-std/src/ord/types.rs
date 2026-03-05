//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
/// Approximates the least fixpoint of a monotone function on a finite ordered set
/// by iterative ascent from the bottom element.
#[allow(dead_code)]
pub struct FixpointIterator<T: Ord + Clone + Eq> {
    current: T,
    bottom: T,
    max_iters: usize,
}
impl<T: Ord + Clone + Eq> FixpointIterator<T> {
    /// Create a new fixpoint iterator starting from `bottom`.
    pub fn new(bottom: T, max_iters: usize) -> Self {
        Self {
            current: bottom.clone(),
            bottom,
            max_iters,
        }
    }
    /// Compute the fixpoint of `f` starting from `bottom`.
    /// Returns `Some(fp)` if converged within `max_iters`, `None` otherwise.
    pub fn compute<F: Fn(&T) -> T>(&mut self, f: &F) -> Option<T> {
        self.current = self.bottom.clone();
        for _ in 0..self.max_iters {
            let next = f(&self.current);
            if next == self.current {
                return Some(self.current.clone());
            }
            self.current = next;
        }
        None
    }
    /// Current approximation.
    pub fn current(&self) -> &T {
        &self.current
    }
    /// Reset to the bottom element.
    pub fn reset(&mut self) {
        self.current = self.bottom.clone();
    }
}
/// A closed interval `[lo, hi]` over an ordered type with membership and operations.
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BoundedRange<T: Ord + Clone> {
    lo: T,
    hi: T,
}
impl<T: Ord + Clone> BoundedRange<T> {
    /// Create a new `BoundedRange`. Panics if `lo > hi`.
    pub fn new(lo: T, hi: T) -> Self {
        assert!(lo <= hi, "BoundedRange: lo must be <= hi");
        Self { lo, hi }
    }
    /// Try to create a `BoundedRange`, returning `None` if `lo > hi`.
    pub fn try_new(lo: T, hi: T) -> Option<Self> {
        if lo <= hi {
            Some(Self { lo, hi })
        } else {
            None
        }
    }
    /// Check if `val` lies in this closed interval.
    pub fn contains(&self, val: &T) -> bool {
        val >= &self.lo && val <= &self.hi
    }
    /// The lower bound.
    pub fn lo(&self) -> &T {
        &self.lo
    }
    /// The upper bound.
    pub fn hi(&self) -> &T {
        &self.hi
    }
    /// Clamp `val` to this interval.
    pub fn clamp(&self, val: T) -> T {
        if val < self.lo {
            self.lo.clone()
        } else if val > self.hi {
            self.hi.clone()
        } else {
            val
        }
    }
    /// Compute the intersection of two ranges, returning `None` if disjoint.
    pub fn intersect(&self, other: &Self) -> Option<Self> {
        let lo = if self.lo >= other.lo {
            self.lo.clone()
        } else {
            other.lo.clone()
        };
        let hi = if self.hi <= other.hi {
            self.hi.clone()
        } else {
            other.hi.clone()
        };
        Self::try_new(lo, hi)
    }
    /// Check if two ranges overlap.
    pub fn overlaps(&self, other: &Self) -> bool {
        self.lo <= other.hi && other.lo <= self.hi
    }
    /// Check if `other` is entirely contained in `self`.
    pub fn includes(&self, other: &Self) -> bool {
        self.lo <= other.lo && other.hi <= self.hi
    }
}
/// The three possible ordering outcomes, mirroring Lean 4's `Ordering`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum OrdResult {
    /// First argument is less than the second.
    Less,
    /// Both arguments are equal.
    Equal,
    /// First argument is greater than the second.
    Greater,
}
impl OrdResult {
    /// Swap the ordering: `Less ↔ Greater`, `Equal` stays `Equal`.
    pub fn swap(self) -> Self {
        match self {
            OrdResult::Less => OrdResult::Greater,
            OrdResult::Equal => OrdResult::Equal,
            OrdResult::Greater => OrdResult::Less,
        }
    }
    /// Lexicographic "then": if `self` is `Equal`, return `other`; otherwise `self`.
    pub fn then(self, other: OrdResult) -> Self {
        match self {
            OrdResult::Equal => other,
            _ => self,
        }
    }
    /// `true` iff the result is `Less`.
    pub fn is_lt(self) -> bool {
        self == OrdResult::Less
    }
    /// `true` iff the result is `Equal`.
    pub fn is_eq(self) -> bool {
        self == OrdResult::Equal
    }
    /// `true` iff the result is `Greater`.
    pub fn is_gt(self) -> bool {
        self == OrdResult::Greater
    }
    /// `true` iff the result is `Less` or `Equal`.
    pub fn is_le(self) -> bool {
        self != OrdResult::Greater
    }
    /// `true` iff the result is `Greater` or `Equal`.
    pub fn is_ge(self) -> bool {
        self != OrdResult::Less
    }
    /// Convert to a signed integer: -1, 0, or 1.
    pub fn to_signum(self) -> i32 {
        match self {
            OrdResult::Less => -1,
            OrdResult::Equal => 0,
            OrdResult::Greater => 1,
        }
    }
    /// Convert from a `std::cmp::Ordering`.
    pub fn from_std(o: std::cmp::Ordering) -> Self {
        match o {
            std::cmp::Ordering::Less => OrdResult::Less,
            std::cmp::Ordering::Equal => OrdResult::Equal,
            std::cmp::Ordering::Greater => OrdResult::Greater,
        }
    }
    /// Convert to a `std::cmp::Ordering`.
    pub fn to_std(self) -> std::cmp::Ordering {
        match self {
            OrdResult::Less => std::cmp::Ordering::Less,
            OrdResult::Equal => std::cmp::Ordering::Equal,
            OrdResult::Greater => std::cmp::Ordering::Greater,
        }
    }
}
/// A sorted `Vec`-based set.
#[derive(Clone, Debug, Default)]
pub struct SortedSet<T: Ord> {
    items: Vec<T>,
}
impl<T: Ord> SortedSet<T> {
    /// Create an empty `SortedSet`.
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }
    /// Insert `item` (no-op if already present).
    pub fn insert(&mut self, item: T) {
        match self.items.binary_search(&item) {
            Ok(_) => {}
            Err(i) => self.items.insert(i, item),
        }
    }
    /// `true` if `item` is in the set.
    pub fn contains(&self, item: &T) -> bool {
        self.items.binary_search(item).is_ok()
    }
    /// Remove `item`, returning `true` if it was present.
    pub fn remove(&mut self, item: &T) -> bool {
        match self.items.binary_search(item) {
            Ok(i) => {
                self.items.remove(i);
                true
            }
            Err(_) => false,
        }
    }
    /// Number of items.
    pub fn len(&self) -> usize {
        self.items.len()
    }
    /// `true` if empty.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
    /// Iterate over items in order.
    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.items.iter()
    }
    /// Set union.
    pub fn union(&self, other: &Self) -> Self
    where
        T: Clone,
    {
        let mut result = self.clone();
        for item in &other.items {
            result.insert(item.clone());
        }
        result
    }
    /// Set intersection.
    pub fn intersection(&self, other: &Self) -> Self
    where
        T: Clone,
    {
        let mut result = Self::new();
        for item in &self.items {
            if other.contains(item) {
                result.insert(item.clone());
            }
        }
        result
    }
    /// Set difference (`self \ other`).
    pub fn difference(&self, other: &Self) -> Self
    where
        T: Clone,
    {
        let mut result = Self::new();
        for item in &self.items {
            if !other.contains(item) {
                result.insert(item.clone());
            }
        }
        result
    }
}
/// A simple sorted `Vec`-based map for small collections.
///
/// Keys are kept in sorted order; lookups are O(log n) via binary search.
#[derive(Clone, Debug)]
pub struct SortedMap<K: Ord, V> {
    entries: Vec<(K, V)>,
}
impl<K: Ord, V> SortedMap<K, V> {
    /// Create an empty `SortedMap`.
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }
    /// Insert or replace the value for `key`.
    pub fn insert(&mut self, key: K, value: V) {
        match self.entries.binary_search_by(|(k, _)| k.cmp(&key)) {
            Ok(i) => self.entries[i].1 = value,
            Err(i) => self.entries.insert(i, (key, value)),
        }
    }
    /// Look up the value for `key`.
    pub fn get(&self, key: &K) -> Option<&V> {
        self.entries
            .binary_search_by(|(k, _)| k.cmp(key))
            .ok()
            .map(|i| &self.entries[i].1)
    }
    /// Remove the entry for `key`, returning the old value if present.
    pub fn remove(&mut self, key: &K) -> Option<V> {
        self.entries
            .binary_search_by(|(k, _)| k.cmp(key))
            .ok()
            .map(|i| self.entries.remove(i).1)
    }
    /// `true` if `key` is in the map.
    pub fn contains_key(&self, key: &K) -> bool {
        self.entries.binary_search_by(|(k, _)| k.cmp(key)).is_ok()
    }
    /// Number of entries.
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    /// `true` if the map is empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
    /// Iterate over `(key, value)` pairs in key order.
    pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
        self.entries.iter().map(|(k, v)| (k, v))
    }
    /// Return all keys in order.
    pub fn keys(&self) -> impl Iterator<Item = &K> {
        self.entries.iter().map(|(k, _)| k)
    }
    /// Return all values in key order.
    pub fn values(&self) -> impl Iterator<Item = &V> {
        self.entries.iter().map(|(_, v)| v)
    }
}
/// A concrete representation of a Galois connection between two ordered sets.
///
/// A Galois connection is a pair of monotone functions `l : A → B` and `r : B → A`
/// such that for all `a ∈ A` and `b ∈ B`: `l(a) ≤ b ↔ a ≤ r(b)`.
#[allow(dead_code)]
pub struct GaloisPair<A, B, L, R>
where
    A: Ord,
    B: Ord,
    L: Fn(&A) -> B,
    R: Fn(&B) -> A,
{
    left_adjoint: L,
    right_adjoint: R,
    _phantom: std::marker::PhantomData<(A, B)>,
}
impl<A, B, L, R> GaloisPair<A, B, L, R>
where
    A: Ord,
    B: Ord,
    L: Fn(&A) -> B,
    R: Fn(&B) -> A,
{
    /// Construct a new Galois pair from left and right adjoints.
    pub fn new(left_adjoint: L, right_adjoint: R) -> Self {
        Self {
            left_adjoint,
            right_adjoint,
            _phantom: std::marker::PhantomData,
        }
    }
    /// Apply the left adjoint (lower adjoint) to an element.
    pub fn left(&self, a: &A) -> B {
        (self.left_adjoint)(a)
    }
    /// Apply the right adjoint (upper adjoint) to an element.
    pub fn right(&self, b: &B) -> A {
        (self.right_adjoint)(b)
    }
    /// Check the Galois condition: `l(a) ≤ b ↔ a ≤ r(b)` for given `a` and `b`.
    pub fn check_galois_condition(&self, a: &A, b: &B) -> bool {
        let la = self.left(a);
        let rb = self.right(b);
        (la <= *b) == (*a <= rb)
    }
    /// The closure operator `a ↦ r(l(a))` is a closure operator on `A`.
    pub fn closure(&self, a: &A) -> A {
        let la = self.left(a);
        self.right(&la)
    }
    /// The kernel operator `b ↦ l(r(b))` is a kernel operator on `B`.
    pub fn kernel(&self, b: &B) -> B {
        let rb = self.right(b);
        self.left(&rb)
    }
}
/// A permutation of indices `0..n`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Permutation {
    pub perm: Vec<usize>,
}
impl Permutation {
    /// Create the identity permutation of size `n`.
    pub fn identity(n: usize) -> Self {
        Self {
            perm: (0..n).collect(),
        }
    }
    /// Create a permutation from a sorted-order vector.
    ///
    /// `perm[i] = j` means that position `i` in the sorted order
    /// came from position `j` in the original.
    pub fn from_sort_order<T: Ord>(v: &[T]) -> Self {
        let mut indices: Vec<usize> = (0..v.len()).collect();
        indices.sort_by(|&a, &b| v[a].cmp(&v[b]));
        Self { perm: indices }
    }
    /// Apply the permutation to a slice, returning a new Vec.
    pub fn apply<T: Clone>(&self, v: &[T]) -> Vec<T> {
        self.perm.iter().map(|&i| v[i].clone()).collect()
    }
    /// Compute the inverse permutation.
    pub fn inverse(&self) -> Self {
        let n = self.perm.len();
        let mut inv = vec![0usize; n];
        for (i, &j) in self.perm.iter().enumerate() {
            inv[j] = i;
        }
        Self { perm: inv }
    }
    /// Compose two permutations (`self` after `other`).
    pub fn compose(&self, other: &Self) -> Self {
        assert_eq!(self.perm.len(), other.perm.len());
        let perm = other.perm.iter().map(|&i| self.perm[i]).collect();
        Self { perm }
    }
    /// Size of the permutation.
    pub fn len(&self) -> usize {
        self.perm.len()
    }
    /// Check if empty.
    pub fn is_empty(&self) -> bool {
        self.perm.is_empty()
    }
    /// Check if this is the identity permutation.
    pub fn is_identity(&self) -> bool {
        self.perm.iter().enumerate().all(|(i, &j)| i == j)
    }
}
/// A chain in a partial order: a totally ordered subset tracked as a sorted Vec.
#[allow(dead_code)]
pub struct MonotoneChain<T: Ord + Clone> {
    elements: Vec<T>,
}
impl<T: Ord + Clone> MonotoneChain<T> {
    /// Create an empty chain.
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
        }
    }
    /// Try to extend the chain with `elem`. Returns `true` if successful
    /// (i.e., `elem` is greater than the last element).
    pub fn push(&mut self, elem: T) -> bool {
        if self.elements.is_empty()
            || *self
                .elements
                .last()
                .expect("elements is non-empty: checked by is_empty")
                < elem
        {
            self.elements.push(elem);
            true
        } else {
            false
        }
    }
    /// Length of the chain.
    pub fn len(&self) -> usize {
        self.elements.len()
    }
    /// Whether the chain is empty.
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
    /// The minimum element of the chain.
    pub fn min_elem(&self) -> Option<&T> {
        self.elements.first()
    }
    /// The maximum element of the chain.
    pub fn max_elem(&self) -> Option<&T> {
        self.elements.last()
    }
    /// Iterate over chain elements in order.
    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.elements.iter()
    }
    /// Build the longest increasing subsequence from a slice (patience sorting).
    pub fn lis_from_slice(v: &[T]) -> Self {
        let mut tails: Vec<T> = Vec::new();
        for x in v {
            let pos = tails.partition_point(|t| t < x);
            if pos == tails.len() {
                tails.push(x.clone());
            } else {
                tails[pos] = x.clone();
            }
        }
        let mut chain = Self::new();
        for t in tails {
            chain.elements.push(t);
        }
        chain
    }
}
/// A max-heap backed by a `Vec`, supporting arbitrary `Ord` elements.
#[allow(dead_code)]
pub struct OrderedHeap<T: Ord> {
    data: Vec<T>,
}
impl<T: Ord> OrderedHeap<T> {
    /// Create an empty heap.
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }
    /// Push an element onto the heap.
    pub fn push(&mut self, val: T) {
        self.data.push(val);
        self.sift_up(self.data.len() - 1);
    }
    /// Pop the maximum element.
    pub fn pop(&mut self) -> Option<T> {
        if self.data.is_empty() {
            return None;
        }
        let n = self.data.len();
        self.data.swap(0, n - 1);
        let max = self.data.pop();
        if !self.data.is_empty() {
            self.sift_down(0);
        }
        max
    }
    /// Peek at the maximum element without removing it.
    pub fn peek(&self) -> Option<&T> {
        self.data.first()
    }
    /// Number of elements.
    pub fn len(&self) -> usize {
        self.data.len()
    }
    /// Whether the heap is empty.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    fn sift_up(&mut self, mut i: usize) {
        while i > 0 {
            let parent = (i - 1) / 2;
            if self.data[i] > self.data[parent] {
                self.data.swap(i, parent);
                i = parent;
            } else {
                break;
            }
        }
    }
    fn sift_down(&mut self, mut i: usize) {
        let n = self.data.len();
        loop {
            let left = 2 * i + 1;
            let right = 2 * i + 2;
            let mut largest = i;
            if left < n && self.data[left] > self.data[largest] {
                largest = left;
            }
            if right < n && self.data[right] > self.data[largest] {
                largest = right;
            }
            if largest == i {
                break;
            }
            self.data.swap(i, largest);
            i = largest;
        }
    }
    /// Build a heap from a Vec (heapify).
    pub fn from_vec(mut v: Vec<T>) -> Self {
        let n = v.len();
        let mut heap = Self {
            data: std::mem::take(&mut v),
        };
        if n > 1 {
            let start = (n - 2) / 2;
            for i in (0..=start).rev() {
                heap.sift_down(i);
            }
        }
        heap
    }
}
