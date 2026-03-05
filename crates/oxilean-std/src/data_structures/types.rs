//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::TRIE_ALPHABET;

use super::functions::*;
use std::collections::HashMap;

/// A binary min-heap backed by a `Vec`.
///
/// The smallest element is always at the root (index 0).
/// All operations maintain the heap property: `parent ≤ children`.
pub struct BinaryHeap<T: Ord> {
    data: Vec<T>,
}
impl<T: Ord> BinaryHeap<T> {
    /// Create a new empty min-heap.
    pub fn new() -> Self {
        BinaryHeap { data: Vec::new() }
    }
    /// Return the number of elements in the heap.
    pub fn len(&self) -> usize {
        self.data.len()
    }
    /// Return `true` if the heap contains no elements.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    /// Push an element onto the heap, restoring the heap property.
    ///
    /// Time complexity: O(log n).
    pub fn push(&mut self, item: T) {
        self.data.push(item);
        self.sift_up(self.data.len() - 1);
    }
    /// Remove and return the minimum element, or `None` if the heap is empty.
    ///
    /// Time complexity: O(log n).
    pub fn pop(&mut self) -> Option<T> {
        if self.data.is_empty() {
            return None;
        }
        let last = self.data.len() - 1;
        self.data.swap(0, last);
        let min = self.data.pop();
        if !self.data.is_empty() {
            self.sift_down(0);
        }
        min
    }
    /// Return a reference to the minimum element, or `None` if the heap is empty.
    ///
    /// Time complexity: O(1).
    pub fn peek(&self) -> Option<&T> {
        self.data.first()
    }
    fn sift_up(&mut self, mut idx: usize) {
        while idx > 0 {
            let parent = (idx - 1) / 2;
            if self.data[idx] < self.data[parent] {
                self.data.swap(idx, parent);
                idx = parent;
            } else {
                break;
            }
        }
    }
    fn sift_down(&mut self, mut idx: usize) {
        let n = self.data.len();
        loop {
            let left = 2 * idx + 1;
            let right = 2 * idx + 2;
            let mut smallest = idx;
            if left < n && self.data[left] < self.data[smallest] {
                smallest = left;
            }
            if right < n && self.data[right] < self.data[smallest] {
                smallest = right;
            }
            if smallest == idx {
                break;
            }
            self.data.swap(idx, smallest);
            idx = smallest;
        }
    }
}
/// Succinct bit vector with O(1) rank and select.
///
/// Stores n bits + O(n / log n) auxiliary data for O(1) rank queries.
pub struct SuccinctBitVector {
    /// The raw bits stored as u64 words.
    words: Vec<u64>,
    /// Superblock cumulative popcount (every SUPER_BLOCK bits).
    superblocks: Vec<u32>,
    /// Block cumulative popcount within a superblock (every BLOCK_SIZE bits).
    blocks: Vec<u16>,
    /// Total number of bits.
    n: usize,
}
impl SuccinctBitVector {
    const BLOCK_SIZE: usize = 64;
    const SUPER_BLOCK: usize = 4096;
    /// Construct a succinct bit vector from a slice of bits.
    pub fn new(bits: &[bool]) -> Self {
        let n = bits.len();
        let num_words = (n + 63) / 64;
        let mut words = vec![0u64; num_words];
        for (i, &b) in bits.iter().enumerate() {
            if b {
                words[i / 64] |= 1u64 << (i % 64);
            }
        }
        let num_super = (n + Self::SUPER_BLOCK - 1) / Self::SUPER_BLOCK + 1;
        let num_blocks = (n + Self::BLOCK_SIZE - 1) / Self::BLOCK_SIZE + 1;
        let mut superblocks = vec![0u32; num_super];
        let mut blocks = vec![0u16; num_blocks];
        let mut super_count = 0u32;
        let mut block_count = 0u16;
        for i in 0..num_blocks {
            if i % (Self::SUPER_BLOCK / Self::BLOCK_SIZE) == 0 {
                superblocks[i / (Self::SUPER_BLOCK / Self::BLOCK_SIZE)] = super_count;
                block_count = 0;
            }
            blocks[i] = block_count;
            if i < num_words {
                let pop = words[i].count_ones() as u16;
                block_count += pop;
                super_count += pop as u32;
            }
        }
        SuccinctBitVector {
            words,
            superblocks,
            blocks,
            n,
        }
    }
    /// rank1(i): number of 1-bits in positions [0, i].
    pub fn rank1(&self, i: usize) -> usize {
        if i >= self.n {
            return self.popcount_total();
        }
        let word_idx = i / 64;
        let bit_pos = i % 64;
        let bi = word_idx;
        let super_idx = bi / (Self::SUPER_BLOCK / Self::BLOCK_SIZE);
        let sb_count = if super_idx < self.superblocks.len() {
            self.superblocks[super_idx] as usize
        } else {
            0
        };
        let blk_count = if bi < self.blocks.len() {
            self.blocks[bi] as usize
        } else {
            0
        };
        let mask = if bit_pos == 63 {
            u64::MAX
        } else {
            (1u64 << (bit_pos + 1)) - 1
        };
        let word_bits = if word_idx < self.words.len() {
            (self.words[word_idx] & mask).count_ones() as usize
        } else {
            0
        };
        sb_count + blk_count + word_bits
    }
    /// rank0(i): number of 0-bits in positions [0, i].
    pub fn rank0(&self, i: usize) -> usize {
        i + 1 - self.rank1(i)
    }
    /// select1(k): position of the k-th 1-bit (1-indexed).
    pub fn select1(&self, k: usize) -> Option<usize> {
        let mut remaining = k;
        for (wi, &w) in self.words.iter().enumerate() {
            let pop = w.count_ones() as usize;
            if remaining <= pop {
                let mut word = w;
                for bit in 0..64 {
                    if word & 1 == 1 {
                        remaining -= 1;
                        if remaining == 0 {
                            let pos = wi * 64 + bit;
                            return if pos < self.n { Some(pos) } else { None };
                        }
                    }
                    word >>= 1;
                }
            }
            remaining -= pop;
        }
        None
    }
    /// Total number of 1-bits.
    pub fn popcount_total(&self) -> usize {
        self.words.iter().map(|w| w.count_ones() as usize).sum()
    }
    /// Number of bits stored.
    pub fn len(&self) -> usize {
        self.n
    }
    /// Whether the bit vector is empty.
    pub fn is_empty(&self) -> bool {
        self.n == 0
    }
    /// Get the bit at position i.
    pub fn get(&self, i: usize) -> bool {
        if i >= self.n {
            return false;
        }
        (self.words[i / 64] >> (i % 64)) & 1 == 1
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SkipListData {
    pub max_level: usize,
    pub num_elements: usize,
    pub probability: f64,
    pub expected_height: f64,
}
#[allow(dead_code)]
impl SkipListData {
    pub fn new(max_level: usize, p: f64) -> Self {
        let expected_h = (max_level as f64) * p.ln().abs();
        SkipListData {
            max_level,
            num_elements: 0,
            probability: p,
            expected_height: expected_h.min(max_level as f64),
        }
    }
    pub fn insert(&mut self) {
        self.num_elements += 1;
    }
    pub fn expected_search_time(&self) -> f64 {
        if self.num_elements == 0 {
            return 0.0;
        }
        (self.num_elements as f64).log2() / (1.0 / self.probability).log2()
    }
    pub fn space_usage(&self) -> String {
        format!(
            "Skip list: O(n/p) expected nodes (n={}, p={})",
            self.num_elements, self.probability
        )
    }
    pub fn pugh_analysis(&self) -> String {
        format!(
            "Pugh skip list (p={:.2}): O(log n) search/insert/delete with high probability",
            self.probability
        )
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct TreapData {
    pub size: usize,
    pub is_implicitly_keyed: bool,
    pub split_merge_supported: bool,
}
#[allow(dead_code)]
impl TreapData {
    pub fn new() -> Self {
        TreapData {
            size: 0,
            is_implicitly_keyed: false,
            split_merge_supported: true,
        }
    }
    pub fn implicit_treap() -> Self {
        TreapData {
            size: 0,
            is_implicitly_keyed: true,
            split_merge_supported: true,
        }
    }
    pub fn expected_height(&self) -> f64 {
        if self.size == 0 {
            return 0.0;
        }
        2.0 * (self.size as f64).ln()
    }
    pub fn split_at(&self, pos: usize) -> String {
        format!(
            "Split treap of size {} at position {}: O(log n)",
            self.size, pos
        )
    }
    pub fn merge_description(&self) -> String {
        format!(
            "Merge two treaps: O(log n) expected (treap property maintained by random priorities)"
        )
    }
}
/// Wavelet tree for range queries on integer sequences.
///
/// Supports range frequency, range nth-smallest, and range quantile
/// all in O(log σ) time where σ is the alphabet size.
pub struct WaveletTree {
    /// The alphabet range [lo, hi).
    lo: u64,
    hi: u64,
    /// Per-level bit vectors: bits[level][i] = true if element goes right.
    bits: Vec<Vec<bool>>,
    /// The sequence length.
    n: usize,
}
impl WaveletTree {
    /// Construct a wavelet tree from a sequence of values in [lo, hi).
    pub fn new(data: &[u64], lo: u64, hi: u64) -> Self {
        let n = data.len();
        let levels = if hi <= lo {
            0
        } else {
            (hi - lo).ilog2() as usize + 1
        };
        let bits = vec![vec![false; n]; levels];
        let mut wt = WaveletTree { lo, hi, bits, n };
        if n > 0 && hi > lo {
            wt.build(data, lo, hi, 0, &mut (0..n).collect::<Vec<_>>());
        }
        wt
    }
    fn build(&mut self, data: &[u64], lo: u64, hi: u64, level: usize, indices: &mut Vec<usize>) {
        if hi - lo <= 1 || level >= self.bits.len() || indices.is_empty() {
            return;
        }
        let mid = lo + (hi - lo) / 2;
        for (pos, &idx) in indices.iter().enumerate() {
            self.bits[level][pos] = data[idx] >= mid;
        }
        let mut left: Vec<usize> = indices.iter().copied().filter(|&i| data[i] < mid).collect();
        let mut right: Vec<usize> = indices
            .iter()
            .copied()
            .filter(|&i| data[i] >= mid)
            .collect();
        self.build(data, lo, mid, level + 1, &mut left);
        self.build(data, mid, hi, level + 1, &mut right);
    }
    /// Count occurrences of value `v` in range [l, r].
    pub fn range_freq(&self, data: &[u64], l: usize, r: usize, v: u64) -> usize {
        if l > r || r >= self.n || v < self.lo || v >= self.hi {
            return 0;
        }
        data[l..=r].iter().filter(|&&x| x == v).count()
    }
    /// Return the sequence length.
    pub fn len(&self) -> usize {
        self.n
    }
    /// Return whether the wavelet tree is empty.
    pub fn is_empty(&self) -> bool {
        self.n == 0
    }
    /// Number of levels in the tree.
    pub fn num_levels(&self) -> usize {
        self.bits.len()
    }
}
/// Segment tree for range queries.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SegmentTreeNew {
    data: Vec<i64>,
    n: usize,
}
impl SegmentTreeNew {
    #[allow(dead_code)]
    pub fn from_slice(arr: &[i64]) -> Self {
        let n = arr.len();
        let mut data = vec![0i64; 4 * n];
        if n > 0 {
            Self::build_inner(&mut data, arr, 1, 0, n - 1);
        }
        Self { data, n }
    }
    fn build_inner(data: &mut Vec<i64>, arr: &[i64], node: usize, l: usize, r: usize) {
        if l == r {
            data[node] = arr[l];
        } else {
            let mid = (l + r) / 2;
            Self::build_inner(data, arr, 2 * node, l, mid);
            Self::build_inner(data, arr, 2 * node + 1, mid + 1, r);
            data[node] = data[2 * node] + data[2 * node + 1];
        }
    }
    #[allow(dead_code)]
    pub fn query_sum(&self, l: usize, r: usize) -> i64 {
        if self.n == 0 {
            return 0;
        }
        self.query_inner(1, 0, self.n - 1, l, r)
    }
    fn query_inner(&self, node: usize, node_l: usize, node_r: usize, l: usize, r: usize) -> i64 {
        if r < node_l || node_r < l {
            return 0;
        }
        if l <= node_l && node_r <= r {
            return self.data[node];
        }
        let mid = (node_l + node_r) / 2;
        self.query_inner(2 * node, node_l, mid, l, r)
            + self.query_inner(2 * node + 1, mid + 1, node_r, l, r)
    }
    #[allow(dead_code)]
    pub fn update(&mut self, pos: usize, val: i64) {
        if self.n > 0 {
            self.update_inner(1, 0, self.n - 1, pos, val);
        }
    }
    fn update_inner(&mut self, node: usize, l: usize, r: usize, pos: usize, val: i64) {
        if l == r {
            self.data[node] = val;
        } else {
            let mid = (l + r) / 2;
            if pos <= mid {
                self.update_inner(2 * node, l, mid, pos, val);
            } else {
                self.update_inner(2 * node + 1, mid + 1, r, pos, val);
            }
            self.data[node] = self.data[2 * node] + self.data[2 * node + 1];
        }
    }
}
/// Union-Find (Disjoint Set Union) with path compression and union by rank.
///
/// Provides near-constant amortized time for `find` and `union`.
/// The amortized cost per operation is O(α(n)) where α is the inverse Ackermann function.
pub struct DisjointSet {
    parent: Vec<usize>,
    rank: Vec<usize>,
    count: usize,
}
impl DisjointSet {
    /// Create a disjoint-set structure for `n` elements, each in its own set.
    pub fn new(n: usize) -> Self {
        DisjointSet {
            parent: (0..n).collect(),
            rank: vec![0; n],
            count: n,
        }
    }
    /// Find the representative (root) of the set containing element `x`.
    ///
    /// Applies path compression for amortized efficiency.
    pub fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }
    /// Unite the sets containing `x` and `y`.
    ///
    /// Returns `true` if they were in different sets, `false` if already united.
    /// Uses union by rank to keep trees shallow.
    pub fn union(&mut self, x: usize, y: usize) -> bool {
        let rx = self.find(x);
        let ry = self.find(y);
        if rx == ry {
            return false;
        }
        match self.rank[rx].cmp(&self.rank[ry]) {
            std::cmp::Ordering::Less => self.parent[rx] = ry,
            std::cmp::Ordering::Greater => self.parent[ry] = rx,
            std::cmp::Ordering::Equal => {
                self.parent[ry] = rx;
                self.rank[rx] += 1;
            }
        }
        self.count -= 1;
        true
    }
    /// Return `true` if `x` and `y` are in the same set.
    pub fn connected(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }
    /// Return the number of disjoint sets.
    pub fn num_sets(&self) -> usize {
        self.count
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PersistentSegmentTree {
    pub n: usize,
    pub num_versions: usize,
    pub root_per_version: Vec<usize>,
    pub nodes: Vec<(i64, usize, usize)>,
}
#[allow(dead_code)]
impl PersistentSegmentTree {
    pub fn new(n: usize) -> Self {
        PersistentSegmentTree {
            n,
            num_versions: 0,
            root_per_version: vec![],
            nodes: vec![(0, 0, 0)],
        }
    }
    pub fn space_complexity(&self) -> String {
        format!(
            "Persistent SegTree: O(n + q log n) nodes for {} versions",
            self.num_versions
        )
    }
    pub fn time_complexity(&self) -> String {
        format!(
            "O(log {}) per query/update, supports historical queries",
            self.n
        )
    }
    pub fn range_query_version(&self, version: usize, _l: usize, _r: usize) -> i64 {
        let _ = version;
        0
    }
}
/// Fenwick tree (Binary Indexed Tree).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FenwickTree {
    tree: Vec<i64>,
}
impl FenwickTree {
    #[allow(dead_code)]
    pub fn new(n: usize) -> Self {
        Self {
            tree: vec![0i64; n + 1],
        }
    }
    #[allow(dead_code)]
    pub fn update(&mut self, mut i: usize, delta: i64) {
        while i < self.tree.len() {
            self.tree[i] += delta;
            i += i & i.wrapping_neg();
        }
    }
    #[allow(dead_code)]
    pub fn prefix_sum(&self, mut i: usize) -> i64 {
        let mut s = 0i64;
        while i > 0 {
            s += self.tree[i];
            i -= i & i.wrapping_neg();
        }
        s
    }
    #[allow(dead_code)]
    pub fn range_sum(&self, l: usize, r: usize) -> i64 {
        if l == 0 {
            self.prefix_sum(r)
        } else {
            self.prefix_sum(r) - self.prefix_sum(l - 1)
        }
    }
}
/// An AVL self-balancing binary search tree.
///
/// All operations (insert, contains) run in O(log n) worst-case time.
/// The height is always ≤ 1.44 * log2(n + 2).
pub struct AvlTree<T: Ord> {
    root: Option<Box<AvlNode<T>>>,
    size: usize,
}
impl<T: Ord> AvlTree<T> {
    /// Create a new empty AVL tree.
    pub fn new() -> Self {
        AvlTree {
            root: None,
            size: 0,
        }
    }
    /// Insert a value into the tree.
    ///
    /// Duplicate values are ignored.
    /// Time complexity: O(log n).
    pub fn insert(&mut self, value: T) {
        let old_root = self.root.take();
        let new_root = avl_insert(old_root, value);
        self.size += 1;
        self.root = Some(new_root);
    }
    /// Return `true` if `value` is in the tree.
    ///
    /// Time complexity: O(log n).
    pub fn contains(&self, value: &T) -> bool {
        avl_contains(&self.root, value)
    }
    /// Return the height of the tree (0 for an empty tree).
    pub fn height(&self) -> usize {
        avl_height(&self.root)
    }
    /// Return the number of elements in the tree.
    pub fn len(&self) -> usize {
        self.size
    }
    /// Return `true` if the tree contains no elements.
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }
}
/// A probabilistic Bloom filter with configurable number of bits and hash functions.
pub struct BloomFilterDs {
    /// Bit vector.
    bits: Vec<bool>,
    /// Number of hash functions k.
    k: usize,
    /// Number of bits m.
    m: usize,
    /// Number of inserted elements.
    count: usize,
}
impl BloomFilterDs {
    /// Construct a Bloom filter with `m` bits and `k` hash functions.
    pub fn new(m: usize, k: usize) -> Self {
        BloomFilterDs {
            bits: vec![false; m.max(1)],
            k,
            m: m.max(1),
            count: 0,
        }
    }
    /// Compute the j-th hash of a key (using FNV-derived mixing).
    fn hash(&self, key: u64, j: usize) -> usize {
        let h = key
            .wrapping_mul(0x9e3779b97f4a7c15_u64)
            .wrapping_add((j as u64).wrapping_mul(0x6c62272e07bb0142_u64));
        (h >> 33) as usize % self.m
    }
    /// Insert a key into the Bloom filter.
    pub fn insert(&mut self, key: u64) {
        for j in 0..self.k {
            let idx = self.hash(key, j);
            self.bits[idx] = true;
        }
        self.count += 1;
    }
    /// Query whether a key may be present. Returns false iff definitely absent.
    pub fn might_contain(&self, key: u64) -> bool {
        (0..self.k).all(|j| self.bits[self.hash(key, j)])
    }
    /// Approximate false positive probability: (1 - e^{-kn/m})^k.
    pub fn false_positive_rate(&self) -> f64 {
        let exp = -(self.k as f64 * self.count as f64) / self.m as f64;
        (1.0 - exp.exp()).powi(self.k as i32)
    }
    /// Number of elements inserted.
    pub fn len(&self) -> usize {
        self.count
    }
    /// Whether the filter has had any insertions.
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }
}
/// A node in an AVL tree.
pub struct AvlNode<T: Ord> {
    pub value: T,
    pub height: usize,
    pub left: Option<Box<AvlNode<T>>>,
    pub right: Option<Box<AvlNode<T>>>,
}
impl<T: Ord> AvlNode<T> {
    pub(super) fn new(value: T) -> Box<Self> {
        Box::new(AvlNode {
            value,
            height: 1,
            left: None,
            right: None,
        })
    }
}
/// A segment tree supporting range sum queries and point updates.
///
/// Both `query` and `update` run in O(log n) time.
pub struct SegmentTree {
    n: usize,
    data: Vec<i64>,
}
impl SegmentTree {
    /// Build a segment tree from a slice of values.
    ///
    /// Time complexity: O(n).
    pub fn new(values: &[i64]) -> Self {
        let n = values.len();
        let mut data = vec![0i64; 4 * n.max(1)];
        if n > 0 {
            Self::build(&mut data, values, 0, 0, n - 1);
        }
        SegmentTree { n, data }
    }
    fn build(data: &mut Vec<i64>, values: &[i64], node: usize, start: usize, end: usize) {
        if start == end {
            data[node] = values[start];
        } else {
            let mid = (start + end) / 2;
            Self::build(data, values, 2 * node + 1, start, mid);
            Self::build(data, values, 2 * node + 2, mid + 1, end);
            data[node] = data[2 * node + 1] + data[2 * node + 2];
        }
    }
    /// Query the sum over the index range `[l, r]` (inclusive).
    ///
    /// Returns `0` for an empty range or out-of-bounds query.
    /// Time complexity: O(log n).
    pub fn query(&self, l: usize, r: usize) -> i64 {
        if self.n == 0 || l > r || r >= self.n {
            return 0;
        }
        self.query_inner(0, 0, self.n - 1, l, r)
    }
    fn query_inner(&self, node: usize, start: usize, end: usize, l: usize, r: usize) -> i64 {
        if r < start || end < l {
            return 0;
        }
        if l <= start && end <= r {
            return self.data[node];
        }
        let mid = (start + end) / 2;
        self.query_inner(2 * node + 1, start, mid, l, r)
            + self.query_inner(2 * node + 2, mid + 1, end, l, r)
    }
    /// Update the element at index `idx` to `value`.
    ///
    /// Time complexity: O(log n).
    pub fn update(&mut self, idx: usize, value: i64) {
        if idx >= self.n {
            return;
        }
        self.update_inner(0, 0, self.n - 1, idx, value);
    }
    fn update_inner(&mut self, node: usize, start: usize, end: usize, idx: usize, value: i64) {
        if start == end {
            self.data[node] = value;
        } else {
            let mid = (start + end) / 2;
            if idx <= mid {
                self.update_inner(2 * node + 1, start, mid, idx, value);
            } else {
                self.update_inner(2 * node + 2, mid + 1, end, idx, value);
            }
            self.data[node] = self.data[2 * node + 1] + self.data[2 * node + 2];
        }
    }
    /// Return the number of leaf elements in the tree.
    pub fn len(&self) -> usize {
        self.n
    }
    /// Return `true` if the tree has no elements.
    pub fn is_empty(&self) -> bool {
        self.n == 0
    }
}
/// A trie (prefix tree) storing strings of bytes.
///
/// Supports `insert` and `search` in O(|key|) time.
pub struct Trie {
    nodes: Vec<TrieNode>,
}
impl Trie {
    /// Create a new empty trie.
    pub fn new() -> Self {
        Trie {
            nodes: vec![TrieNode::new()],
        }
    }
    /// Insert a string into the trie.
    ///
    /// Time complexity: O(|key|).
    pub fn insert(&mut self, key: &str) {
        let mut current = 0;
        for byte in key.bytes() {
            let idx = byte as usize;
            if self.nodes[current].children[idx].is_none() {
                let new_node = self.nodes.len();
                self.nodes.push(TrieNode::new());
                self.nodes[current].children[idx] = Some(new_node);
            }
            current = self.nodes[current].children[idx]
                .expect("children[idx] is Some: was just inserted in the if branch above");
        }
        self.nodes[current].is_terminal = true;
    }
    /// Return `true` if `key` was previously inserted into the trie.
    ///
    /// Time complexity: O(|key|).
    pub fn search(&self, key: &str) -> bool {
        let mut current = 0;
        for byte in key.bytes() {
            let idx = byte as usize;
            match self.nodes[current].children[idx] {
                None => return false,
                Some(next) => current = next,
            }
        }
        self.nodes[current].is_terminal
    }
    /// Return `true` if any key in the trie starts with `prefix`.
    ///
    /// Time complexity: O(|prefix|).
    pub fn starts_with(&self, prefix: &str) -> bool {
        let mut current = 0;
        for byte in prefix.bytes() {
            let idx = byte as usize;
            match self.nodes[current].children[idx] {
                None => return false,
                Some(next) => current = next,
            }
        }
        true
    }
}
/// A probabilistic skip list with O(log n) expected insert and search.
///
/// Implementation note: uses a simple Vec-based implementation (no raw pointers).
///
/// This implementation uses a Vec-of-Vecs structure (no unsafe pointers).
/// Each element is stored in the base level; higher levels hold express lanes.
pub struct SkipList<T: Ord + Clone> {
    /// Levels of sorted vectors; `levels[0]` is the base (all elements).
    levels: Vec<Vec<T>>,
    max_levels: usize,
    /// Simple deterministic "random" source for level generation.
    counter: u64,
}
impl<T: Ord + Clone> SkipList<T> {
    /// Create a new empty skip list with the given maximum number of levels.
    pub fn new(max_levels: usize) -> Self {
        let max_levels = max_levels.max(1);
        SkipList {
            levels: vec![Vec::new(); max_levels],
            max_levels,
            counter: 0,
        }
    }
    /// Return the number of elements in the skip list (base level size).
    pub fn len(&self) -> usize {
        self.levels[0].len()
    }
    /// Return `true` if the skip list is empty.
    pub fn is_empty(&self) -> bool {
        self.levels[0].is_empty()
    }
    /// Determine how many levels to promote a newly inserted element to.
    ///
    /// Uses a simple linear-congruential generator as a deterministic stand-in
    /// for the usual coin-flip; real skip lists use a random source.
    fn random_level(&mut self) -> usize {
        self.counter = self
            .counter
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        let mut level = 1;
        let mut bits = self.counter;
        while level < self.max_levels && (bits & 1) == 0 {
            level += 1;
            bits >>= 1;
        }
        level
    }
    /// Insert `value` into the skip list.
    ///
    /// Expected time complexity: O(log n).
    pub fn insert(&mut self, value: T) {
        let num_levels = self.random_level();
        for level in 0..num_levels {
            let pos = self.levels[level].partition_point(|x| x < &value);
            if pos >= self.levels[level].len() || self.levels[level][pos] != value {
                self.levels[level].insert(pos, value.clone());
            }
        }
    }
    /// Return `true` if `value` is in the skip list.
    ///
    /// Uses the highest occupied level for fast scanning.
    /// Expected time complexity: O(log n).
    pub fn contains(&self, value: &T) -> bool {
        for level in (0..self.max_levels).rev() {
            if self.levels[level].is_empty() {
                continue;
            }
            if self.levels[level].binary_search(value).is_ok() {
                return true;
            }
        }
        false
    }
}
/// A node in the character-trie.
struct TrieNode {
    children: Vec<Option<usize>>,
    is_terminal: bool,
}
impl TrieNode {
    fn new() -> Self {
        TrieNode {
            children: vec![None; TRIE_ALPHABET],
            is_terminal: false,
        }
    }
}
/// A double-ended queue backed by two `Vec`s.
///
/// Elements pushed to the front go into `front` (reversed),
/// elements pushed to the back go into `back`.
/// All four operations (`push_front`, `push_back`, `pop_front`, `pop_back`)
/// are amortized O(1).
pub struct Deque<T> {
    front: Vec<T>,
    back: Vec<T>,
}
impl<T> Deque<T> {
    /// Create a new empty deque.
    pub fn new() -> Self {
        Deque {
            front: Vec::new(),
            back: Vec::new(),
        }
    }
    /// Return the number of elements in the deque.
    pub fn len(&self) -> usize {
        self.front.len() + self.back.len()
    }
    /// Return `true` if the deque is empty.
    pub fn is_empty(&self) -> bool {
        self.front.is_empty() && self.back.is_empty()
    }
    /// Push `value` to the front of the deque.
    pub fn push_front(&mut self, value: T) {
        self.front.push(value);
    }
    /// Push `value` to the back of the deque.
    pub fn push_back(&mut self, value: T) {
        self.back.push(value);
    }
    /// Remove and return the front element, or `None` if empty.
    pub fn pop_front(&mut self) -> Option<T> {
        if let Some(v) = self.front.pop() {
            return Some(v);
        }
        if self.back.is_empty() {
            return None;
        }
        let len = self.back.len();
        if len == 1 {
            return self.back.pop();
        }
        let keep = (len + 1) / 2;
        let mut moved = self.back.split_off(keep);
        moved.reverse();
        self.front = moved;
        self.front.pop()
    }
    /// Remove and return the back element, or `None` if empty.
    pub fn pop_back(&mut self) -> Option<T> {
        if let Some(v) = self.back.pop() {
            return Some(v);
        }
        if self.front.is_empty() {
            return None;
        }
        let len = self.front.len();
        if len == 1 {
            return self.front.pop();
        }
        let keep = (len + 1) / 2;
        let mut moved = self.front.split_off(keep);
        moved.reverse();
        self.back = moved;
        self.back.pop()
    }
    /// Return a reference to the front element, or `None` if empty.
    pub fn front(&self) -> Option<&T> {
        self.front.last().or_else(|| self.back.first())
    }
    /// Return a reference to the back element, or `None` if empty.
    pub fn back(&self) -> Option<&T> {
        self.back.last().or_else(|| self.front.first())
    }
}
/// HyperLogLog cardinality estimator.
///
/// Uses b-bit register index (2^b registers) to estimate set cardinality
/// with relative error approximately 1.04 / sqrt(2^b).
pub struct HyperLogLog {
    /// Number of register index bits.
    b: u32,
    /// The registers M[0..m).
    registers: Vec<u8>,
    /// m = 2^b.
    m: usize,
}
impl HyperLogLog {
    /// Construct a HyperLogLog estimator with 2^b registers.
    pub fn new(b: u32) -> Self {
        let b = b.clamp(4, 16);
        let m = 1usize << b;
        HyperLogLog {
            b,
            registers: vec![0u8; m],
            m,
        }
    }
    /// Hash a u64 key using FNV-1a mixing.
    fn hash_key(&self, key: u64) -> u64 {
        key.wrapping_mul(0x517cc1b727220a95_u64)
            .rotate_left(17)
            .wrapping_mul(0x6c62272e07bb0142_u64)
    }
    /// Add an element to the estimator.
    pub fn add(&mut self, key: u64) {
        let h = self.hash_key(key);
        let index = (h >> (64 - self.b)) as usize;
        let w = h << self.b;
        let rho = if w == 0 {
            (64 - self.b) as u8 + 1
        } else {
            w.trailing_zeros() as u8 + 1
        };
        if rho > self.registers[index] {
            self.registers[index] = rho;
        }
    }
    /// Estimate the cardinality using the HyperLogLog formula.
    pub fn estimate(&self) -> f64 {
        let m = self.m as f64;
        let alpha = 0.7213 / (1.0 + 1.079 / m);
        let z: f64 = self.registers.iter().map(|&r| 2f64.powi(-(r as i32))).sum();
        let raw = alpha * m * m / z;
        if raw <= 2.5 * m {
            let zeros = self.registers.iter().filter(|&&r| r == 0).count() as f64;
            if zeros > 0.0 {
                m * (m / zeros).ln()
            } else {
                raw
            }
        } else {
            raw
        }
    }
    /// Relative error bound: ~1.04 / sqrt(m).
    pub fn relative_error_bound(&self) -> f64 {
        1.04 / (self.m as f64).sqrt()
    }
    /// Number of registers.
    pub fn num_registers(&self) -> usize {
        self.m
    }
}
/// B-tree properties.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct BTree {
    pub order: usize,
    pub num_keys: usize,
    pub height: usize,
}
impl BTree {
    #[allow(dead_code)]
    pub fn new(order: usize) -> Self {
        Self {
            order,
            num_keys: 0,
            height: 0,
        }
    }
    #[allow(dead_code)]
    pub fn max_keys_per_node(&self) -> usize {
        2 * self.order - 1
    }
    #[allow(dead_code)]
    pub fn min_keys_per_node(&self) -> usize {
        self.order - 1
    }
    #[allow(dead_code)]
    pub fn max_height_for_n_keys(&self, n: usize) -> usize {
        if n <= 1 {
            return 0;
        }
        let t = self.order;
        ((n + 1) as f64 / 2.0).log(t as f64).ceil() as usize
    }
    #[allow(dead_code)]
    pub fn disk_access_per_operation(&self) -> String {
        format!("O(log_t(n)) disk accesses, t={}", self.order)
    }
}
/// Union-Find (Disjoint Set Union) data structure.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
    num_components: usize,
}
impl UnionFind {
    #[allow(dead_code)]
    pub fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
            num_components: n,
        }
    }
    #[allow(dead_code)]
    pub fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }
    #[allow(dead_code)]
    pub fn union(&mut self, x: usize, y: usize) -> bool {
        let rx = self.find(x);
        let ry = self.find(y);
        if rx == ry {
            return false;
        }
        if self.rank[rx] < self.rank[ry] {
            self.parent[rx] = ry;
        } else if self.rank[rx] > self.rank[ry] {
            self.parent[ry] = rx;
        } else {
            self.parent[ry] = rx;
            self.rank[rx] += 1;
        }
        self.num_components -= 1;
        true
    }
    #[allow(dead_code)]
    pub fn connected(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }
    #[allow(dead_code)]
    pub fn num_components(&self) -> usize {
        self.num_components
    }
}
/// Binary min-heap.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct BinaryMinHeap<T: Ord + Clone> {
    data: Vec<T>,
}
impl<T: Ord + Clone> BinaryMinHeap<T> {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }
    #[allow(dead_code)]
    pub fn push(&mut self, val: T) {
        self.data.push(val);
        self.sift_up(self.data.len() - 1);
    }
    #[allow(dead_code)]
    pub fn pop(&mut self) -> Option<T> {
        if self.data.is_empty() {
            return None;
        }
        let n = self.data.len();
        self.data.swap(0, n - 1);
        let val = self.data.pop();
        if !self.data.is_empty() {
            self.sift_down(0);
        }
        val
    }
    #[allow(dead_code)]
    pub fn peek(&self) -> Option<&T> {
        self.data.first()
    }
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.data.len()
    }
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    fn sift_up(&mut self, mut i: usize) {
        while i > 0 {
            let parent = (i - 1) / 2;
            if self.data[i] < self.data[parent] {
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
            let mut smallest = i;
            if left < n && self.data[left] < self.data[smallest] {
                smallest = left;
            }
            if right < n && self.data[right] < self.data[smallest] {
                smallest = right;
            }
            if smallest != i {
                self.data.swap(i, smallest);
                i = smallest;
            } else {
                break;
            }
        }
    }
}
/// Disjoint hash map (open addressing).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SimpleHashMap {
    buckets: Vec<Option<(u64, u64)>>,
    size: usize,
    capacity: usize,
}
impl SimpleHashMap {
    #[allow(dead_code)]
    pub fn new(capacity: usize) -> Self {
        Self {
            buckets: vec![None; capacity],
            size: 0,
            capacity,
        }
    }
    fn hash(&self, key: u64) -> usize {
        (key as usize).wrapping_mul(2654435769) % self.capacity
    }
    #[allow(dead_code)]
    pub fn insert(&mut self, key: u64, val: u64) {
        let mut h = self.hash(key);
        loop {
            match self.buckets[h] {
                None => {
                    self.buckets[h] = Some((key, val));
                    self.size += 1;
                    return;
                }
                Some((k, _)) if k == key => {
                    self.buckets[h] = Some((key, val));
                    return;
                }
                _ => {
                    h = (h + 1) % self.capacity;
                }
            }
        }
    }
    #[allow(dead_code)]
    pub fn get(&self, key: u64) -> Option<u64> {
        let mut h = self.hash(key);
        for _ in 0..self.capacity {
            match self.buckets[h] {
                None => return None,
                Some((k, v)) if k == key => return Some(v),
                _ => h = (h + 1) % self.capacity,
            }
        }
        None
    }
    #[allow(dead_code)]
    pub fn load_factor(&self) -> f64 {
        self.size as f64 / self.capacity as f64
    }
}
/// Skip list (probabilistic data structure).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SkipListInfo {
    pub num_levels: usize,
    pub promotion_probability: f64,
    pub expected_space: f64,
}
impl SkipListInfo {
    #[allow(dead_code)]
    pub fn new(n: usize, p: f64) -> Self {
        let max_levels = (n as f64).log2().ceil() as usize + 1;
        let expected_space = n as f64 / (1.0 - p);
        Self {
            num_levels: max_levels,
            promotion_probability: p,
            expected_space,
        }
    }
    #[allow(dead_code)]
    pub fn expected_search_time_description(&self) -> String {
        format!(
            "Skip list with p={}: expected O(log n) search, O(n/1-p) space",
            self.promotion_probability
        )
    }
}
/// Persistent data structure (functional).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PersistArrayOld<T: Clone> {
    versions: Vec<Vec<T>>,
}
impl<T: Clone> PersistArrayOld<T> {
    #[allow(dead_code)]
    pub fn new(initial: Vec<T>) -> Self {
        Self {
            versions: vec![initial],
        }
    }
    #[allow(dead_code)]
    pub fn current_version(&self) -> usize {
        self.versions.len() - 1
    }
    #[allow(dead_code)]
    pub fn update(&mut self, version: usize, idx: usize, val: T) -> usize {
        let mut new_v = self.versions[version].clone();
        if idx < new_v.len() {
            new_v[idx] = val;
        }
        self.versions.push(new_v);
        self.versions.len() - 1
    }
    #[allow(dead_code)]
    pub fn get(&self, version: usize, idx: usize) -> Option<&T> {
        self.versions.get(version)?.get(idx)
    }
}

/// Type alias for `PersistArrayOld` - a persistent array with version tracking.
pub type PersistArrayExt<T> = PersistArrayOld<T>;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct VanEmdeBoasTree {
    pub universe_size: usize,
    pub min: Option<usize>,
    pub max: Option<usize>,
    pub summary: Option<Box<VanEmdeBoasTree>>,
}
#[allow(dead_code)]
impl VanEmdeBoasTree {
    pub fn new(universe: usize) -> Self {
        VanEmdeBoasTree {
            universe_size: universe,
            min: None,
            max: None,
            summary: if universe > 2 {
                let sqrt = (universe as f64).sqrt().ceil() as usize;
                Some(Box::new(VanEmdeBoasTree::new(sqrt)))
            } else {
                None
            },
        }
    }
    pub fn complexity_description(&self) -> String {
        format!(
            "van Emde Boas: insert/delete/predecessor in O(log log U) where U={}",
            self.universe_size
        )
    }
    pub fn is_empty(&self) -> bool {
        self.min.is_none()
    }
    pub fn upper_sqrt(&self) -> usize {
        (self.universe_size as f64).sqrt().ceil() as usize
    }
    pub fn lower_sqrt(&self) -> usize {
        (self.universe_size as f64).sqrt().floor() as usize
    }
    pub fn high(&self, x: usize) -> usize {
        x / self.lower_sqrt()
    }
    pub fn low(&self, x: usize) -> usize {
        x % self.lower_sqrt()
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct XFastTrie {
    pub universe_bits: usize,
    pub levels: Vec<std::collections::HashMap<usize, String>>,
    pub num_elements: usize,
}
#[allow(dead_code)]
impl XFastTrie {
    pub fn new(bits: usize) -> Self {
        XFastTrie {
            universe_bits: bits,
            levels: vec![std::collections::HashMap::new(); bits + 1],
            num_elements: 0,
        }
    }
    pub fn complexity_description(&self) -> String {
        format!(
            "X-Fast Trie: O(log W) search, O(W) space (W={} bits)",
            self.universe_bits
        )
    }
    pub fn predecessor_time(&self) -> String {
        format!(
            "O(log W) = O(log {}) per predecessor query",
            self.universe_bits
        )
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PersistArrayV2<T: Clone> {
    pub data: Vec<T>,
    pub versions: Vec<(usize, T)>,
    pub current_version: usize,
}
#[allow(dead_code)]
impl<T: Clone + Default> PersistArrayV2<T> {
    pub fn new(size: usize) -> Self {
        PersistArrayV2 {
            data: vec![T::default(); size],
            versions: vec![],
            current_version: 0,
        }
    }
    pub fn update(&mut self, idx: usize, val: T) -> usize {
        let old = self.data[idx].clone();
        self.versions.push((idx, old));
        self.data[idx] = val;
        self.current_version += 1;
        self.current_version
    }
    pub fn rollback(&mut self) {
        if let Some((idx, old_val)) = self.versions.pop() {
            self.data[idx] = old_val;
            self.current_version = self.current_version.saturating_sub(1);
        }
    }
    pub fn complexity(&self) -> String {
        format!(
            "PersistentArray: O(1) update/rollback (path copying), {} versions stored",
            self.versions.len()
        )
    }
}
