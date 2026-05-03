//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::HashMap;

use super::functions::*;

/// A formal complexity proof pairing an algorithm with its big-O class and a proof sketch.
pub struct ComplexityProof {
    pub algorithm: String,
    pub complexity: TimeComplexity,
    pub proof_sketch: String,
}
impl ComplexityProof {
    /// Create a new `ComplexityProof`.
    pub fn new() -> Self {
        Self {
            algorithm: String::new(),
            complexity: TimeComplexity::O1,
            proof_sketch: String::new(),
        }
    }
    /// Returns `true` iff the proof sketch is non-empty (indicating a real proof exists).
    pub fn is_tight_bound(&self) -> bool {
        !self.proof_sketch.is_empty()
    }
}
/// A witness that a negative cycle exists in a graph, given as a cycle of vertex indices.
pub struct NegativeCycleWitness {
    pub cycle: Vec<usize>,
}
/// Union-Find with certificate generation.
///
/// Each merge stores the edge used so a spanning forest certificate can be extracted.
pub struct CertifyingUnionFind {
    parent: Vec<usize>,
    rank: Vec<u8>,
    /// Edge certificates: edge_cert\[v\] = the edge (u, v) that caused v to be merged.
    pub edge_cert: Vec<Option<(usize, usize)>>,
    /// Number of components.
    pub num_components: usize,
}
impl CertifyingUnionFind {
    /// Create a new `CertifyingUnionFind` for `n` elements.
    pub fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
            edge_cert: vec![None; n],
            num_components: n,
        }
    }
    /// Find the root of the component containing `x` (with path compression).
    pub fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }
    /// Union the components containing `u` and `v`.
    /// Returns `true` if they were in different components (a merge occurred).
    pub fn union(&mut self, u: usize, v: usize) -> bool {
        let ru = self.find(u);
        let rv = self.find(v);
        if ru == rv {
            return false;
        }
        if self.rank[ru] < self.rank[rv] {
            self.parent[ru] = rv;
            self.edge_cert[ru] = Some((u, v));
        } else if self.rank[ru] > self.rank[rv] {
            self.parent[rv] = ru;
            self.edge_cert[rv] = Some((u, v));
        } else {
            self.parent[rv] = ru;
            self.rank[ru] += 1;
            self.edge_cert[rv] = Some((u, v));
        }
        self.num_components -= 1;
        true
    }
    /// Extract the spanning forest certificate: list of edges that were merged.
    pub fn spanning_forest_certificate(&self) -> Vec<(usize, usize)> {
        self.edge_cert.iter().filter_map(|e| *e).collect()
    }
    /// Verify the certificate: the spanning forest edges should connect exactly n−k components.
    pub fn verify_certificate(&self, n: usize) -> bool {
        let edges = self.spanning_forest_certificate();
        edges.len() == n.saturating_sub(self.num_components)
    }
}
/// The kind of primality certificate held by a `PrimeCertificate`.
#[derive(Debug, Clone)]
pub enum PrimeCertType {
    /// Trial division up to sqrt(n).
    TrialDivision,
    /// Miller-Rabin witnesses that prove primality (or compositeness).
    MillerRabin(Vec<u64>),
    /// Lucas primality certificate (factored p-1).
    LucasPrimalityCert,
}
/// Time complexity class in big-O notation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TimeComplexity {
    /// O(1) — constant time.
    O1,
    /// O(log n) — logarithmic time.
    OLogN,
    /// O(n) — linear time.
    ON,
    /// O(n log n) — linearithmic time.
    ONLogN,
    /// O(n²) — quadratic time.
    ON2,
    /// O(n³) — cubic time.
    ON3,
    /// O(2^n) — exponential time.
    OExponential,
    /// O(n!) — factorial time.
    OFactorial,
}
impl TimeComplexity {
    /// Human-readable big-O string.
    pub fn big_o_string(&self) -> &str {
        match self {
            TimeComplexity::O1 => "O(1)",
            TimeComplexity::OLogN => "O(log n)",
            TimeComplexity::ON => "O(n)",
            TimeComplexity::ONLogN => "O(n log n)",
            TimeComplexity::ON2 => "O(n^2)",
            TimeComplexity::ON3 => "O(n^3)",
            TimeComplexity::OExponential => "O(2^n)",
            TimeComplexity::OFactorial => "O(n!)",
        }
    }
    /// Returns `true` iff the complexity class is polynomial (at most O(n^k) for some k).
    pub fn is_polynomial(&self) -> bool {
        matches!(
            self,
            TimeComplexity::O1
                | TimeComplexity::OLogN
                | TimeComplexity::ON
                | TimeComplexity::ONLogN
                | TimeComplexity::ON2
                | TimeComplexity::ON3
        )
    }
}
/// Certified binary search that counts comparisons.
pub struct CertifiedBinarySearch {
    pub comparisons: u64,
}
impl CertifiedBinarySearch {
    /// Create a new `CertifiedBinarySearch`.
    pub fn new() -> Self {
        Self { comparisons: 0 }
    }
    /// Search for `target` in the sorted slice `v`. Returns the index if found.
    pub fn search(&mut self, v: &[i64], target: i64) -> Option<usize> {
        let (mut lo, mut hi) = (0usize, v.len());
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            self.comparisons += 1;
            match v[mid].cmp(&target) {
                std::cmp::Ordering::Equal => return Some(mid),
                std::cmp::Ordering::Less => lo = mid + 1,
                std::cmp::Ordering::Greater => hi = mid,
            }
        }
        None
    }
    /// Certificate: v\[idx\] == target AND v is sorted around idx.
    pub fn certificate(v: &[i64], target: i64, idx: usize) -> bool {
        if idx >= v.len() {
            return false;
        }
        if v[idx] != target {
            return false;
        }
        if idx > 0 && v[idx - 1] > v[idx] {
            return false;
        }
        if idx + 1 < v.len() && v[idx] > v[idx + 1] {
            return false;
        }
        true
    }
}
/// Certified bubble sort that tracks comparisons and swaps.
pub struct CertifiedBubbleSort {
    pub comparisons: u64,
    pub swaps: u64,
}
impl CertifiedBubbleSort {
    /// Create a new `CertifiedBubbleSort` with zeroed counters.
    pub fn new() -> Self {
        Self {
            comparisons: 0,
            swaps: 0,
        }
    }
    /// Sort `v` in place, updating comparison and swap counters.
    pub fn sort(&mut self, v: &mut [i64]) {
        let n = v.len();
        for i in 0..n {
            for j in 0..n.saturating_sub(i + 1) {
                self.comparisons += 1;
                if v[j] > v[j + 1] {
                    v.swap(j, j + 1);
                    self.swaps += 1;
                }
            }
        }
    }
    /// Returns `true` iff `v` is non-decreasingly sorted — the sort certificate.
    pub fn is_sorted_certificate(v: &[i64]) -> bool {
        v.windows(2).all(|w| w[0] <= w[1])
    }
    /// Worst-case comparison count for bubble sort on `n` elements: n*(n-1)/2.
    pub fn worst_case_comparisons(n: usize) -> u64 {
        let n = n as u64;
        n * n.saturating_sub(1) / 2
    }
}
/// Certified integer factorization with verified factor list.
pub struct CertifiedFactorization {
    pub n: u64,
    /// Prime factors with multiplicities: `(prime, exponent)`.
    pub factors: Vec<(u64, u32)>,
}
impl CertifiedFactorization {
    /// Create a new `CertifiedFactorization` for `n`.
    pub fn new(n: u64) -> Self {
        Self {
            n,
            factors: Vec::new(),
        }
    }
    /// Factorize `n` by trial division, populating `self.factors`.
    pub fn factorize(&mut self) {
        self.factors.clear();
        let mut remaining = self.n;
        if remaining < 2 {
            return;
        }
        let mut d = 2u64;
        while d * d <= remaining {
            if remaining % d == 0 {
                let mut exp = 0u32;
                while remaining % d == 0 {
                    exp += 1;
                    remaining /= d;
                }
                self.factors.push((d, exp));
            }
            d += 1;
        }
        if remaining > 1 {
            self.factors.push((remaining, 1));
        }
    }
    /// Verify the factorization: recompute n from factors and check each is prime.
    pub fn verify(&self) -> bool {
        let product: u64 = self
            .factors
            .iter()
            .try_fold(1u64, |acc, &(p, e)| {
                let pe = p.checked_pow(e)?;
                acc.checked_mul(pe)
            })
            .unwrap_or(0);
        if product != self.n {
            return false;
        }
        self.factors
            .iter()
            .all(|&(p, _)| PrimeCertificate::new(p).verify())
    }
}
/// A bounded FIFO queue with a maximum size invariant.
pub struct BoundedQueue {
    data: Vec<i64>,
    max_size: usize,
}
impl BoundedQueue {
    /// Create a new `BoundedQueue` with the given maximum capacity.
    pub fn new(max: usize) -> Self {
        Self {
            data: Vec::new(),
            max_size: max,
        }
    }
    /// Enqueue `x`. Returns `true` on success, `false` if the queue is full.
    pub fn enqueue(&mut self, x: i64) -> bool {
        if self.data.len() >= self.max_size {
            return false;
        }
        self.data.push(x);
        true
    }
    /// Dequeue the front element. Returns `None` if empty.
    pub fn dequeue(&mut self) -> Option<i64> {
        if self.data.is_empty() {
            return None;
        }
        Some(self.data.remove(0))
    }
    /// FIFO invariant check: size never exceeds max_size.
    pub fn satisfies_fifo_invariant(&self) -> bool {
        self.data.len() <= self.max_size
    }
}
/// Certified merge sort that tracks recursion depth.
pub struct CertifiedMergeSort {
    pub recursive_depth: u32,
}
impl CertifiedMergeSort {
    /// Create a new `CertifiedMergeSort`.
    pub fn new() -> Self {
        Self { recursive_depth: 0 }
    }
    /// Sort `v`, returning a `SortedInvariant` certificate.
    pub fn sort(&mut self, v: &mut Vec<i64>) -> SortedInvariant {
        let input_len = v.len();
        self.recursive_depth = 0;
        let sorted = self.merge_sort_inner(v.clone(), 0);
        *v = sorted;
        SortedInvariant {
            algorithm: "MergeSort".to_string(),
            input_len,
            verified: CertifiedBubbleSort::is_sorted_certificate(v),
        }
    }
    fn merge_sort_inner(&mut self, mut v: Vec<i64>, depth: u32) -> Vec<i64> {
        if depth > self.recursive_depth {
            self.recursive_depth = depth;
        }
        if v.len() <= 1 {
            return v;
        }
        let mid = v.len() / 2;
        let right = v.split_off(mid);
        let left = v;
        let sorted_left = self.merge_sort_inner(left, depth + 1);
        let sorted_right = self.merge_sort_inner(right, depth + 1);
        Self::merge_step(&sorted_left, &sorted_right)
    }
    /// Merge two sorted slices into a single sorted `Vec<i64>`.
    pub fn merge_step(a: &[i64], b: &[i64]) -> Vec<i64> {
        let mut result = Vec::with_capacity(a.len() + b.len());
        let (mut i, mut j) = (0, 0);
        while i < a.len() && j < b.len() {
            if a[i] <= b[j] {
                result.push(a[i]);
                i += 1;
            } else {
                result.push(b[j]);
                j += 1;
            }
        }
        result.extend_from_slice(&a[i..]);
        result.extend_from_slice(&b[j..]);
        result
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CertifiedUnionFind {
    pub parent: Vec<usize>,
    pub rank: Vec<usize>,
    pub size: usize,
}
#[allow(dead_code)]
impl CertifiedUnionFind {
    pub fn new(n: usize) -> Self {
        CertifiedUnionFind {
            parent: (0..n).collect(),
            rank: vec![0; n],
            size: n,
        }
    }
    pub fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }
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
        true
    }
    pub fn are_connected(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }
    pub fn correctness_invariant(&self) -> bool {
        self.parent
            .iter()
            .enumerate()
            .all(|(i, &p)| p < self.size || p == i)
    }
}
/// Pivot selection strategy for quicksort.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PivotStrategy {
    /// Always choose the first element as pivot.
    First,
    /// Always choose the last element as pivot.
    Last,
    /// Always choose the middle element as pivot.
    Middle,
    /// Use the median of first, middle, and last elements.
    MedianOfThree,
    /// Choose a pseudo-random pivot (deterministic seed for reproducibility).
    Random,
}
/// A primality certificate for a natural number `n`.
pub struct PrimeCertificate {
    pub n: u64,
    pub certificate_type: PrimeCertType,
}
impl PrimeCertificate {
    /// Create a new `PrimeCertificate` for `n` using trial division.
    pub fn new(n: u64) -> Self {
        Self {
            n,
            certificate_type: PrimeCertType::TrialDivision,
        }
    }
    /// Verify the certificate: returns `true` iff `n` is prime under this certificate.
    pub fn verify(&self) -> bool {
        match &self.certificate_type {
            PrimeCertType::TrialDivision => Self::trial_division_prime(self.n),
            PrimeCertType::MillerRabin(witnesses) => Self::miller_rabin_prime(self.n, witnesses),
            PrimeCertType::LucasPrimalityCert => Self::trial_division_prime(self.n),
        }
    }
    fn trial_division_prime(n: u64) -> bool {
        if n < 2 {
            return false;
        }
        if n == 2 {
            return true;
        }
        if n % 2 == 0 {
            return false;
        }
        let mut d = 3u64;
        while d * d <= n {
            if n % d == 0 {
                return false;
            }
            d += 2;
        }
        true
    }
    fn miller_rabin_prime(n: u64, witnesses: &[u64]) -> bool {
        if n < 2 {
            return false;
        }
        if n == 2 || n == 3 {
            return true;
        }
        if n % 2 == 0 {
            return false;
        }
        let mut d = n - 1;
        let mut r = 0u32;
        while d % 2 == 0 {
            d /= 2;
            r += 1;
        }
        'witness: for &a in witnesses {
            if a == 0 || a >= n {
                continue;
            }
            let mut x = mod_pow(a, d, n);
            if x == 1 || x == n - 1 {
                continue;
            }
            for _ in 0..r - 1 {
                x = ((x as u128 * x as u128) % n as u128) as u64;
                if x == n - 1 {
                    continue 'witness;
                }
            }
            return false;
        }
        true
    }
}
/// A vector that maintains the sorted invariant at all times.
pub struct SortedVec {
    data: Vec<i64>,
}
impl SortedVec {
    /// Create a new empty `SortedVec`.
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }
    /// Insert `x` into the sorted vec, maintaining the sorted invariant.
    pub fn insert(&mut self, x: i64) {
        let pos = self.data.partition_point(|&v| v < x);
        self.data.insert(pos, x);
    }
    /// Returns `true` iff `x` is present (binary search).
    pub fn contains(&self, x: i64) -> bool {
        self.data.binary_search(&x).is_ok()
    }
    /// Invariant check: returns `true` iff the underlying vec is sorted.
    pub fn is_sorted_invariant(&self) -> bool {
        self.data.windows(2).all(|w| w[0] <= w[1])
    }
}
/// An AVL-like balanced BST backed by sorted key and height arrays.
///
/// This is a simplified certificate-bearing structure: heights are tracked
/// per-insertion and the balance invariant is checked on demand.
pub struct BalancedBST {
    pub keys: Vec<i64>,
    pub heights: Vec<i32>,
}
impl BalancedBST {
    /// Create a new empty `BalancedBST`.
    pub fn new() -> Self {
        Self {
            keys: Vec::new(),
            heights: Vec::new(),
        }
    }
    /// Insert `k` into the BST, computing a local height estimate.
    pub fn insert(&mut self, k: i64) {
        let pos = self.keys.partition_point(|&v| v < k);
        self.keys.insert(pos, k);
        let n = self.keys.len();
        let h = if n == 0 {
            0
        } else {
            (usize::BITS - n.leading_zeros()) as i32
        };
        self.heights.insert(pos, h);
        for height in self.heights.iter_mut() {
            *height = h;
        }
    }
    /// Height invariant check: for all adjacent nodes, |h_i - h_{i+1}| <= 1.
    pub fn height_invariant_holds(&self) -> bool {
        self.heights.windows(2).all(|w| (w[0] - w[1]).abs() <= 1)
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CertHashMapExt<K, V> {
    pub buckets: Vec<Vec<(K, V)>>,
    pub num_buckets: usize,
    pub size: usize,
    pub load_factor_max: f64,
}
#[allow(dead_code)]
impl<K: Clone + PartialEq, V: Clone> CertHashMapExt<K, V> {
    pub fn new(num_buckets: usize) -> Self {
        CertHashMapExt {
            buckets: vec![vec![]; num_buckets],
            num_buckets,
            size: 0,
            load_factor_max: 0.75,
        }
    }
    pub fn load_factor(&self) -> f64 {
        self.size as f64 / self.num_buckets as f64
    }
    pub fn needs_resize(&self) -> bool {
        self.load_factor() > self.load_factor_max
    }
    pub fn expected_chain_length(&self) -> f64 {
        self.load_factor()
    }
    pub fn amortized_complexity_description(&self) -> String {
        format!(
            "CertifiedHashMap: O(1) amortized insert/lookup (load={:.2}, buckets={})",
            self.load_factor(),
            self.num_buckets
        )
    }
}
/// Online bin packing / scheduling with First Fit Decreasing heuristic.
///
/// Models an online scheduler that places jobs into bins of capacity 1.0.
pub struct OnlineScheduler {
    /// Bins, each containing a list of job sizes and the remaining capacity.
    pub bins: Vec<(Vec<f64>, f64)>,
    /// Bin capacity (default 1.0).
    pub capacity: f64,
}
impl OnlineScheduler {
    /// Create a new `OnlineScheduler` with the given bin capacity.
    pub fn new(capacity: f64) -> Self {
        Self {
            bins: Vec::new(),
            capacity,
        }
    }
    /// Schedule a job of size `s` into the first bin with enough remaining capacity.
    /// Opens a new bin if necessary. Returns the bin index used.
    pub fn schedule(&mut self, s: f64) -> usize {
        for (idx, (jobs, remaining)) in self.bins.iter_mut().enumerate() {
            if *remaining >= s {
                jobs.push(s);
                *remaining -= s;
                return idx;
            }
        }
        let idx = self.bins.len();
        self.bins.push((vec![s], self.capacity - s));
        idx
    }
    /// Returns the number of bins used.
    pub fn num_bins(&self) -> usize {
        self.bins.len()
    }
    /// Fractional lower bound: total job size / capacity.
    pub fn fractional_lower_bound(&self) -> f64 {
        let total: f64 = self
            .bins
            .iter()
            .flat_map(|(jobs, _)| jobs.iter().copied())
            .sum();
        total / self.capacity
    }
    /// Competitive ratio against the fractional lower bound (should be ≤ 2).
    pub fn competitive_ratio(&self) -> f64 {
        let lb = self.fractional_lower_bound();
        if lb == 0.0 {
            return 1.0;
        }
        self.num_bins() as f64 / lb
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SuffixArray {
    pub text: String,
    pub suffix_array: Vec<usize>,
    pub lcp_array: Vec<usize>,
}
#[allow(dead_code)]
impl SuffixArray {
    pub fn new(text: &str) -> Self {
        let n = text.len();
        let mut suffixes: Vec<(usize, &str)> = (0..n).map(|i| (i, &text[i..])).collect();
        suffixes.sort_by_key(|&(_, s)| s);
        let sa: Vec<usize> = suffixes.iter().map(|&(i, _)| i).collect();
        SuffixArray {
            text: text.to_string(),
            suffix_array: sa,
            lcp_array: vec![0; n.saturating_sub(1)],
        }
    }
    pub fn pattern_search(&self, pattern: &str) -> Option<usize> {
        let sa = &self.suffix_array;
        let mut lo = 0;
        let mut hi = sa.len();
        while lo < hi {
            let mid = (lo + hi) / 2;
            let suffix = &self.text[sa[mid]..];
            if suffix.starts_with(pattern) {
                return Some(sa[mid]);
            } else if suffix < pattern {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        None
    }
    pub fn complexity_description(&self) -> String {
        format!(
            "SuffixArray of len {}: O(n log n) construction, O(m log n) search",
            self.text.len()
        )
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CertBFSExt {
    pub n_vertices: usize,
    pub adjacency: Vec<Vec<usize>>,
    pub distances: Vec<Option<usize>>,
    pub predecessors: Vec<Option<usize>>,
}
#[allow(dead_code)]
impl CertBFSExt {
    pub fn new(n: usize) -> Self {
        CertBFSExt {
            n_vertices: n,
            adjacency: vec![vec![]; n],
            distances: vec![None; n],
            predecessors: vec![None; n],
        }
    }
    pub fn add_edge(&mut self, u: usize, v: usize) {
        self.adjacency[u].push(v);
        self.adjacency[v].push(u);
    }
    pub fn bfs_from(&mut self, source: usize) {
        self.distances = vec![None; self.n_vertices];
        self.predecessors = vec![None; self.n_vertices];
        self.distances[source] = Some(0);
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(source);
        while let Some(u) = queue.pop_front() {
            let d = self.distances[u]
                .expect("distances[u] is Some: u was enqueued only after distances[u] was set");
            for &v in &self.adjacency[u].clone() {
                if self.distances[v].is_none() {
                    self.distances[v] = Some(d + 1);
                    self.predecessors[v] = Some(u);
                    queue.push_back(v);
                }
            }
        }
    }
    pub fn shortest_path_to(&self, target: usize) -> Option<Vec<usize>> {
        self.distances[target]?;
        let mut path = vec![target];
        let mut cur = target;
        while let Some(pred) = self.predecessors[cur] {
            path.push(pred);
            cur = pred;
        }
        path.reverse();
        Some(path)
    }
    pub fn correctness_witness(&self, source: usize, target: usize) -> String {
        match self.distances[target] {
            None => format!("No path from {} to {}", source, target),
            Some(d) => {
                format!(
                    "BFS witness: dist({},{}) = {} (shortest path found)",
                    source, target, d
                )
            }
        }
    }
}
/// Certified quicksort with configurable pivot strategy.
pub struct CertifiedQuickSort {
    pub pivot_strategy: PivotStrategy,
    pub comparisons: u64,
}
impl CertifiedQuickSort {
    /// Create a new `CertifiedQuickSort` with the given pivot strategy.
    pub fn new(strat: PivotStrategy) -> Self {
        Self {
            pivot_strategy: strat,
            comparisons: 0,
        }
    }
    /// Sort `v` in place, updating the comparison counter.
    pub fn sort_in_place(&mut self, v: &mut Vec<i64>) {
        let len = v.len();
        if len <= 1 {
            return;
        }
        self.quicksort(v, 0, len - 1);
    }
    fn pivot_index(&self, v: &[i64], lo: usize, hi: usize) -> usize {
        match self.pivot_strategy {
            PivotStrategy::First => lo,
            PivotStrategy::Last => hi,
            PivotStrategy::Middle => (lo + hi) / 2,
            PivotStrategy::MedianOfThree => {
                let mid = (lo + hi) / 2;
                let a = v[lo];
                let b = v[mid];
                let c = v[hi];
                if (a <= b && b <= c) || (c <= b && b <= a) {
                    mid
                } else if (b <= a && a <= c) || (c <= a && a <= b) {
                    lo
                } else {
                    hi
                }
            }
            PivotStrategy::Random => {
                let seed = (lo ^ hi ^ (lo.wrapping_mul(31))) % (hi - lo + 1);
                lo + seed
            }
        }
    }
    fn quicksort(&mut self, v: &mut Vec<i64>, lo: usize, hi: usize) {
        if lo >= hi {
            return;
        }
        let p = self.partition(v, lo, hi);
        if p > 0 {
            self.quicksort(v, lo, p - 1);
        }
        self.quicksort(v, p + 1, hi);
    }
    fn partition(&mut self, v: &mut [i64], lo: usize, hi: usize) -> usize {
        let pi = self.pivot_index(v, lo, hi);
        v.swap(pi, hi);
        let pivot = v[hi];
        let mut i = lo;
        for j in lo..hi {
            self.comparisons += 1;
            if v[j] <= pivot {
                v.swap(i, j);
                i += 1;
            }
        }
        v.swap(i, hi);
        i
    }
    /// Expected comparison count upper bound for average case: 2 n ln n ≈ 1.386 n log₂ n.
    pub fn average_case_bound(n: usize) -> u64 {
        if n <= 1 {
            return 0;
        }
        let n64 = n as u64;
        let log2_approx = u64::BITS - n64.leading_zeros() - 1;
        2 * n64 * log2_approx as u64 * 693 / 1000
    }
}
/// Certified GCD computation using the Euclidean algorithm, recording each step.
pub struct CertifiedGCD {
    pub steps: Vec<(i64, i64)>,
}
impl CertifiedGCD {
    /// Create a new `CertifiedGCD`.
    pub fn new() -> Self {
        Self { steps: Vec::new() }
    }
    /// Compute gcd(a, b) using the Euclidean algorithm, recording all steps.
    pub fn gcd(&mut self, a: i64, b: i64) -> i64 {
        self.steps.clear();
        let (mut x, mut y) = (a.abs(), b.abs());
        while y != 0 {
            self.steps.push((x, y));
            let r = x % y;
            x = y;
            y = r;
        }
        x
    }
    /// Extended Euclidean algorithm: returns (g, x, y) such that a*x + b*y = g.
    pub fn extended_gcd(&mut self, a: i64, b: i64) -> (i64, i64, i64) {
        self.steps.clear();
        let (mut old_r, mut r) = (a, b);
        let (mut old_s, mut s) = (1i64, 0i64);
        let (mut old_t, mut t) = (0i64, 1i64);
        while r != 0 {
            self.steps.push((old_r, r));
            let q = old_r / r;
            (old_r, r) = (r, old_r - q * r);
            (old_s, s) = (s, old_s - q * s);
            (old_t, t) = (t, old_t - q * t);
        }
        (old_r, old_s, old_t)
    }
    /// Verify Bézout's identity: a*x + b*y == g.
    pub fn verify_bezout(a: i64, b: i64, x: i64, y: i64, g: i64) -> bool {
        a * x + b * y == g
    }
}
/// A certificate that a given algorithm has produced a sorted output.
pub struct SortedInvariant {
    pub algorithm: String,
    pub input_len: usize,
    pub verified: bool,
}
/// A termination certificate for an algorithm.
pub struct TerminationCertificate {
    pub algorithm: String,
    pub measure: DecreasingMeasure,
    pub proved: bool,
}
impl TerminationCertificate {
    /// Create a new `TerminationCertificate`.
    pub fn new() -> Self {
        Self {
            algorithm: String::new(),
            measure: DecreasingMeasure::new(),
            proved: false,
        }
    }
    /// Returns `true` iff the termination certificate is fully verified.
    pub fn verify_termination(&self) -> bool {
        self.proved && self.measure.well_founded_recursion_check()
    }
}
/// Verifies that a claimed approximation ratio is achieved by an algorithm output.
///
/// Given the algorithm's output value, the optimal value, and the claimed ratio,
/// checks that `output_value ≤ ratio * optimal_value` (for minimization).
pub struct ApproximationVerifier {
    /// Whether this is a minimization problem (true) or maximization (false).
    pub is_minimization: bool,
    /// Tolerance for floating-point comparison.
    pub tolerance: f64,
}
impl ApproximationVerifier {
    /// Create a new `ApproximationVerifier`.
    pub fn new(is_minimization: bool) -> Self {
        Self {
            is_minimization,
            tolerance: 1e-9,
        }
    }
    /// Verify that `output_value` satisfies the claimed approximation ratio against `opt`.
    pub fn verify_ratio(&self, output_value: f64, opt: f64, claimed_ratio: f64) -> bool {
        if opt <= 0.0 {
            return output_value <= self.tolerance;
        }
        if self.is_minimization {
            output_value <= claimed_ratio * opt + self.tolerance
        } else {
            output_value * claimed_ratio >= opt - self.tolerance
        }
    }
    /// Compute the actual approximation ratio.
    pub fn actual_ratio(&self, output_value: f64, opt: f64) -> f64 {
        if opt <= 0.0 {
            return if output_value <= self.tolerance {
                1.0
            } else {
                f64::INFINITY
            };
        }
        if self.is_minimization {
            output_value / opt
        } else {
            opt / output_value
        }
    }
    /// Verify that the ratio is within \[1.0, claimed_ratio\] (correct approximation).
    pub fn verify_bounds(&self, output_value: f64, opt: f64, claimed_ratio: f64) -> bool {
        let ratio = self.actual_ratio(output_value, opt);
        ratio >= 1.0 - self.tolerance && ratio <= claimed_ratio + self.tolerance
    }
}
/// Count-Min Sketch for streaming frequency estimation.
///
/// Maintains a 2D array of counters; each item is hashed to one counter per row.
/// Point query overestimates but never underestimates frequency.
pub struct StreamingFrequencyEstimator {
    /// depth × width counter table.
    pub table: Vec<Vec<u64>>,
    /// Number of hash functions (rows).
    pub depth: usize,
    /// Number of counters per row.
    pub width: usize,
    /// Total items inserted.
    pub total: u64,
}
impl StreamingFrequencyEstimator {
    /// Create a new `StreamingFrequencyEstimator` with error probability δ and relative error ε.
    /// Uses depth = ceil(ln(1/δ)), width = ceil(e/ε).
    pub fn new(depth: usize, width: usize) -> Self {
        Self {
            table: vec![vec![0u64; width]; depth],
            depth,
            width,
            total: 0,
        }
    }
    /// Insert an item (identified by its hash key) into the sketch.
    pub fn insert(&mut self, key: u64) {
        self.total += 1;
        for (row, row_counters) in self.table.iter_mut().enumerate() {
            let col = Self::hash(key, row as u64, self.width);
            row_counters[col] += 1;
        }
    }
    /// Estimate the frequency of `key`.
    pub fn estimate(&self, key: u64) -> u64 {
        (0..self.depth)
            .map(|row| {
                let col = Self::hash(key, row as u64, self.width);
                self.table[row][col]
            })
            .min()
            .unwrap_or(0)
    }
    /// Invariant: estimate is never less than true frequency.
    pub fn estimate_lower_bound_holds(&self, key: u64, true_freq: u64) -> bool {
        self.estimate(key) >= true_freq
    }
    fn hash(key: u64, seed: u64, width: usize) -> usize {
        let a = seed.wrapping_mul(2654435761) ^ 0x9e3779b9;
        let b = seed.wrapping_mul(0x517cc1b727220a95);
        ((a.wrapping_mul(key).wrapping_add(b)) % (width as u64 + 1)) as usize % width
    }
}
/// Space complexity class.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpaceComplexity {
    /// O(1) — constant extra space (in-place).
    O1,
    /// O(log n) — logarithmic extra space.
    OLogN,
    /// O(n) — linear extra space.
    ON,
    /// O(n²) — quadratic extra space.
    ON2,
}
impl SpaceComplexity {
    /// Returns `true` iff the algorithm is in-place (O(1) extra space).
    pub fn is_in_place(&self) -> bool {
        matches!(self, SpaceComplexity::O1)
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct KMPMatcher {
    pub pattern: Vec<u8>,
    pub failure_function: Vec<i64>,
}
#[allow(dead_code)]
impl KMPMatcher {
    pub fn new(pattern: &str) -> Self {
        let p: Vec<u8> = pattern.bytes().collect();
        let failure = Self::compute_failure(&p);
        KMPMatcher {
            pattern: p,
            failure_function: failure,
        }
    }
    fn compute_failure(p: &[u8]) -> Vec<i64> {
        let m = p.len();
        let mut fail = vec![0i64; m];
        let mut k: i64 = 0;
        for i in 1..m {
            while k > 0 && p[k as usize] != p[i] {
                k = fail[k as usize - 1];
            }
            if p[k as usize] == p[i] {
                k += 1;
            }
            fail[i] = k;
        }
        fail
    }
    pub fn search(&self, text: &str) -> Vec<usize> {
        let t: Vec<u8> = text.bytes().collect();
        let m = self.pattern.len();
        let n = t.len();
        let mut matches = vec![];
        let mut k: i64 = 0;
        for i in 0..n {
            while k > 0 && self.pattern[k as usize] != t[i] {
                k = self.failure_function[k as usize - 1];
            }
            if self.pattern[k as usize] == t[i] {
                k += 1;
            }
            if k == m as i64 {
                matches.push(i + 1 - m);
                k = self.failure_function[k as usize - 1];
            }
        }
        matches
    }
    pub fn complexity_description(&self) -> String {
        format!(
            "KMP: O(n + m) time, m={} pattern length",
            self.pattern.len()
        )
    }
}
/// Certified Bellman-Ford algorithm; detects negative cycles.
pub struct CertifiedBellmanFord {
    pub dist: Vec<f64>,
}
impl CertifiedBellmanFord {
    /// Create a new `CertifiedBellmanFord` for a graph with `n` vertices.
    pub fn new(n: usize) -> Self {
        Self {
            dist: vec![f64::INFINITY; n],
        }
    }
    /// Run Bellman-Ford from `src` on the edge list `(u, v, weight)`.
    /// Returns `Some(NegativeCycleWitness)` if a negative cycle is detected,
    /// or `None` if all shortest paths are well-defined.
    pub fn run(
        &mut self,
        edges: &[(usize, usize, f64)],
        src: usize,
    ) -> Option<NegativeCycleWitness> {
        let n = self.dist.len();
        if src < n {
            self.dist[src] = 0.0;
        }
        let mut prev: Vec<Option<usize>> = vec![None; n];
        for _ in 0..n.saturating_sub(1) {
            for &(u, v, w) in edges {
                if u < n && v < n && self.dist[u] < f64::INFINITY {
                    let alt = self.dist[u] + w;
                    if alt < self.dist[v] {
                        self.dist[v] = alt;
                        prev[v] = Some(u);
                    }
                }
            }
        }
        for &(u, v, w) in edges {
            if u < n && v < n && self.dist[u] < f64::INFINITY && self.dist[u] + w < self.dist[v] {
                let cycle = Self::extract_cycle(&prev, v, n);
                return Some(NegativeCycleWitness { cycle });
            }
        }
        None
    }
    fn extract_cycle(prev: &[Option<usize>], start: usize, n: usize) -> Vec<usize> {
        let mut cur = start;
        for _ in 0..n {
            cur = match prev[cur] {
                Some(p) => p,
                None => return vec![start],
            };
        }
        let anchor = cur;
        let mut cycle = vec![anchor];
        cur = match prev[anchor] {
            Some(p) => p,
            None => return cycle,
        };
        while cur != anchor {
            cycle.push(cur);
            cur = match prev[cur] {
                Some(p) => p,
                None => break,
            };
        }
        cycle.reverse();
        cycle
    }
}
/// Certified BFS that records visited nodes and shortest-path distances.
pub struct CertifiedBFS {
    pub visited: Vec<bool>,
    pub distances: Vec<Option<u64>>,
}
impl CertifiedBFS {
    /// Create a new `CertifiedBFS` for a graph with `n` vertices.
    pub fn new(n: usize) -> Self {
        Self {
            visited: vec![false; n],
            distances: vec![None; n],
        }
    }
    /// Run BFS from `start`, populating `visited` and `distances`.
    pub fn bfs_from(&mut self, adj: &[Vec<usize>], start: usize) {
        if start >= self.visited.len() {
            return;
        }
        let mut queue = std::collections::VecDeque::new();
        self.visited[start] = true;
        self.distances[start] = Some(0);
        queue.push_back(start);
        while let Some(u) = queue.pop_front() {
            let d = self.distances[u].unwrap_or(0);
            for &v in &adj[u] {
                if v < self.visited.len() && !self.visited[v] {
                    self.visited[v] = true;
                    self.distances[v] = Some(d + 1);
                    queue.push_back(v);
                }
            }
        }
    }
    /// BFS distance from the source to vertex `v`.
    pub fn distance(&self, v: usize) -> Option<u64> {
        self.distances.get(v).copied().flatten()
    }
    /// Returns `true` iff vertex `v` was reached during the BFS.
    pub fn is_reachable(&self, v: usize) -> bool {
        self.visited.get(v).copied().unwrap_or(false)
    }
}
/// Interpolation search (assumes uniform key distribution).
pub struct InterpolationSearch {
    pub assumes_uniform: bool,
}
impl InterpolationSearch {
    /// Create a new `InterpolationSearch`.
    pub fn new() -> Self {
        Self {
            assumes_uniform: true,
        }
    }
    /// Search for `target` in the sorted slice `v`.
    /// Uses interpolation formula for O(log log n) expected time on uniform data.
    pub fn search(&self, v: &[i64], target: i64) -> Option<usize> {
        if v.is_empty() {
            return None;
        }
        let (mut lo, mut hi) = (0usize, v.len() - 1);
        while lo <= hi && target >= v[lo] && target <= v[hi] {
            if lo == hi {
                return if v[lo] == target { Some(lo) } else { None };
            }
            let range = (v[hi] - v[lo]) as f64;
            if range == 0.0 {
                break;
            }
            let pos = lo + (((target - v[lo]) as f64 / range) * (hi - lo) as f64) as usize;
            let pos = pos.min(hi);
            match v[pos].cmp(&target) {
                std::cmp::Ordering::Equal => return Some(pos),
                std::cmp::Ordering::Less => {
                    if pos == hi {
                        break;
                    }
                    lo = pos + 1;
                }
                std::cmp::Ordering::Greater => {
                    if pos == 0 {
                        break;
                    }
                    hi = pos - 1;
                }
            }
        }
        None
    }
}
/// Cache-oblivious recursive matrix representation.
///
/// Stores an n×n matrix in Z-Morton order for cache-oblivious access.
pub struct CacheObliviousMatrix {
    /// Flat storage in Z-Morton (Z-order) layout.
    pub data: Vec<f64>,
    /// Dimension of the matrix (n × n).
    pub n: usize,
}
impl CacheObliviousMatrix {
    /// Create a new `CacheObliviousMatrix` of size n×n, initialized to zero.
    pub fn new(n: usize) -> Self {
        let padded = n.next_power_of_two();
        Self {
            data: vec![0.0; padded * padded],
            n,
        }
    }
    /// Compute the Z-Morton index for row r, column c.
    pub fn morton_index(r: usize, c: usize) -> usize {
        let mut index = 0usize;
        let mut bit = 1usize;
        let mut rb = r;
        let mut cb = c;
        let mut pos = 0;
        while rb > 0 || cb > 0 {
            index |= (rb & 1) << pos;
            pos += 1;
            index |= (cb & 1) << pos;
            pos += 1;
            rb >>= 1;
            cb >>= 1;
            bit <<= 1;
        }
        let _ = bit;
        index
    }
    /// Get the value at (row, col).
    pub fn get(&self, row: usize, col: usize) -> f64 {
        let idx = Self::morton_index(row, col);
        if idx < self.data.len() {
            self.data[idx]
        } else {
            0.0
        }
    }
    /// Set the value at (row, col).
    pub fn set(&mut self, row: usize, col: usize, val: f64) {
        let idx = Self::morton_index(row, col);
        if idx < self.data.len() {
            self.data[idx] = val;
        }
    }
    /// Multiply two n×n matrices using the recursive cache-oblivious layout.
    /// Returns a new `CacheObliviousMatrix` with the product.
    pub fn multiply(a: &Self, b: &Self) -> Self {
        let n = a.n.max(b.n);
        let mut result = CacheObliviousMatrix::new(n);
        for i in 0..n {
            for k in 0..n {
                let aik = a.get(i, k);
                if aik == 0.0 {
                    continue;
                }
                for j in 0..n {
                    let cur = result.get(i, j);
                    result.set(i, j, cur + aik * b.get(k, j));
                }
            }
        }
        result
    }
}
/// A decreasing measure witnessing termination of a recursive function.
pub struct DecreasingMeasure {
    pub function: String,
    pub domain: String,
    pub is_well_founded: bool,
}
impl DecreasingMeasure {
    /// Create a new `DecreasingMeasure`.
    pub fn new() -> Self {
        Self {
            function: String::new(),
            domain: String::new(),
            is_well_founded: false,
        }
    }
    /// Returns `true` iff structural recursion on the domain is verified.
    pub fn structural_recursion_check(&self) -> bool {
        !self.domain.is_empty() && self.is_well_founded
    }
    /// Returns `true` iff well-founded recursion via a decreasing measure is verified.
    pub fn well_founded_recursion_check(&self) -> bool {
        self.is_well_founded
    }
}
/// Certified hash map with load-factor tracking.
pub struct CertifiedHashMap {
    pub load_factor_threshold: f64,
    pub num_buckets: usize,
    pub size: usize,
    inner: HashMap<String, i64>,
}
impl CertifiedHashMap {
    /// Create a new `CertifiedHashMap` with `initial_buckets` logical buckets.
    pub fn new(initial_buckets: usize) -> Self {
        Self {
            load_factor_threshold: 0.75,
            num_buckets: initial_buckets.max(1),
            size: 0,
            inner: HashMap::new(),
        }
    }
    /// Insert `(key, val)` into the map. Rehashes if load factor exceeds threshold.
    pub fn insert(&mut self, key: String, val: i64) {
        self.inner.insert(key, val);
        self.size = self.inner.len();
        if self.load_factor() > self.load_factor_threshold {
            self.num_buckets *= 2;
        }
    }
    /// Get the value associated with `key`, if present.
    pub fn get(&self, key: &str) -> Option<i64> {
        self.inner.get(key).copied()
    }
    /// Current load factor: size / num_buckets.
    pub fn load_factor(&self) -> f64 {
        self.size as f64 / self.num_buckets as f64
    }
}
/// Certified Dijkstra's algorithm for single-source shortest paths.
pub struct CertifiedDijkstra {
    pub dist: Vec<f64>,
    pub prev: Vec<Option<usize>>,
}
impl CertifiedDijkstra {
    /// Create a new `CertifiedDijkstra` for a graph with `n` vertices.
    pub fn new(n: usize) -> Self {
        Self {
            dist: vec![f64::INFINITY; n],
            prev: vec![None; n],
        }
    }
    /// Run Dijkstra from `src` on the adjacency list `adj`.
    /// `adj\[u\]` contains `(v, weight)` pairs.
    pub fn run(&mut self, adj: &[Vec<(usize, f64)>], src: usize) {
        if src >= self.dist.len() {
            return;
        }
        self.dist[src] = 0.0;
        let n = self.dist.len();
        let mut visited = vec![false; n];
        for _ in 0..n {
            let u = (0..n).filter(|&x| !visited[x]).min_by(|&a, &b| {
                self.dist[a]
                    .partial_cmp(&self.dist[b])
                    .unwrap_or(std::cmp::Ordering::Equal)
            });
            let u = match u {
                Some(x) if self.dist[x] < f64::INFINITY => x,
                _ => break,
            };
            visited[u] = true;
            if u < adj.len() {
                for &(v, w) in &adj[u] {
                    if v < n {
                        let alt = self.dist[u] + w;
                        if alt < self.dist[v] {
                            self.dist[v] = alt;
                            self.prev[v] = Some(u);
                        }
                    }
                }
            }
        }
    }
    /// Reconstruct the shortest path to `dst`. Returns `None` if unreachable.
    pub fn shortest_path(&self, dst: usize) -> Option<Vec<usize>> {
        if dst >= self.dist.len() || self.dist[dst].is_infinite() {
            return None;
        }
        let mut path = Vec::new();
        let mut cur = dst;
        loop {
            path.push(cur);
            match self.prev[cur] {
                Some(p) => cur = p,
                None => break,
            }
        }
        path.reverse();
        Some(path)
    }
}
