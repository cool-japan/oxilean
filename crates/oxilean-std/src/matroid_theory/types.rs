//! # Matroid Theory — Type Definitions
//!
//! Core types for matroid theory: independence systems, rank functions, circuits,
//! bases, graphic matroids, linear matroids, transversal matroids, and more.

use std::collections::{HashMap, HashSet};
use std::fmt;

// ─── Ground Set Element ───────────────────────────────────────────────────────

/// An element of a matroid's ground set, identified by a non-negative integer index.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Element(pub usize);

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "e{}", self.0)
    }
}

// ─── Independence System ──────────────────────────────────────────────────────

/// An independence system `(E, I)` where `E` is the ground set and `I ⊆ 2^E`
/// satisfies the hereditary property: if `A ∈ I` and `B ⊆ A` then `B ∈ I`.
///
/// A matroid additionally satisfies the augmentation axiom (I3).
#[derive(Debug, Clone)]
pub struct IndependenceSystem {
    /// Ground set elements.
    pub ground_set: Vec<Element>,
    /// The family of independent sets (stored as sorted vectors for canonicity).
    pub independent_sets: HashSet<Vec<Element>>,
}

impl IndependenceSystem {
    /// Create an empty independence system over a ground set of size `n`.
    pub fn new(n: usize) -> Self {
        let ground_set: Vec<Element> = (0..n).map(Element).collect();
        let mut independent_sets = HashSet::new();
        independent_sets.insert(vec![]); // empty set is always independent
        IndependenceSystem {
            ground_set,
            independent_sets,
        }
    }

    /// Test whether `set` is independent.
    pub fn is_independent(&self, set: &[Element]) -> bool {
        let mut sorted = set.to_vec();
        sorted.sort();
        self.independent_sets.contains(&sorted)
    }

    /// Return the rank of `set`: the maximum size of an independent subset.
    pub fn rank_of(&self, set: &[Element]) -> usize {
        let mut max_rank = 0;
        for indep in &self.independent_sets {
            // Check if indep is a subset of set
            let set_hash: HashSet<Element> = set.iter().copied().collect();
            if indep.iter().all(|e| set_hash.contains(e)) && indep.len() > max_rank {
                max_rank = indep.len();
            }
        }
        max_rank
    }

    /// Overall rank of the matroid (rank of ground set).
    pub fn rank(&self) -> usize {
        self.rank_of(&self.ground_set.clone())
    }
}

// ─── Matroid Variants ─────────────────────────────────────────────────────────

/// A **uniform matroid** `U(k, n)` over ground set `{0, ..., n-1}`:
/// every subset of size ≤ k is independent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UniformMatroid {
    /// Ground set size.
    pub n: usize,
    /// Rank parameter: maximum independent set size.
    pub k: usize,
}

impl UniformMatroid {
    /// Construct `U(k, n)`.  Panics if `k > n`.
    pub fn new(k: usize, n: usize) -> Self {
        assert!(k <= n, "UniformMatroid: k must be ≤ n");
        UniformMatroid { n, k }
    }

    /// Test independence: a set is independent iff its size ≤ k.
    pub fn is_independent(&self, set: &[Element]) -> bool {
        set.len() <= self.k
    }

    /// Rank of a subset S: min(|S|, k).
    pub fn rank_of(&self, set: &[Element]) -> usize {
        set.len().min(self.k)
    }

    /// Overall rank.
    pub fn rank(&self) -> usize {
        self.k
    }

    /// Dual of `U(k,n)` is `U(n-k, n)`.
    pub fn dual(&self) -> UniformMatroid {
        UniformMatroid {
            n: self.n,
            k: self.n - self.k,
        }
    }
}

// ─── Graphic Matroid ──────────────────────────────────────────────────────────

/// An undirected graph edge.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Edge {
    /// Source vertex.
    pub u: usize,
    /// Target vertex.
    pub v: usize,
}

impl Edge {
    /// Create a normalized edge (u ≤ v).
    pub fn new(u: usize, v: usize) -> Self {
        if u <= v {
            Edge { u, v }
        } else {
            Edge { u: v, v: u }
        }
    }
}

impl fmt::Display for Edge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.u, self.v)
    }
}

/// A **graphic matroid** `M(G)` where the ground set is the edge set of graph G,
/// and independent sets are forests (acyclic subgraphs).
#[derive(Debug, Clone)]
pub struct GraphicMatroid {
    /// Number of vertices.
    pub num_vertices: usize,
    /// The edges (ground set), indexed by position.
    pub edges: Vec<Edge>,
}

impl GraphicMatroid {
    /// Create a graphic matroid from a graph.
    pub fn new(num_vertices: usize, edges: Vec<Edge>) -> Self {
        GraphicMatroid {
            num_vertices,
            edges,
        }
    }

    /// Test if the subset of edges (by index) forms a forest (acyclic subgraph).
    pub fn is_independent(&self, edge_indices: &[usize]) -> bool {
        // Union-Find to detect cycles
        let mut parent: Vec<usize> = (0..self.num_vertices).collect();
        for &idx in edge_indices {
            if idx >= self.edges.len() {
                return false;
            }
            let e = self.edges[idx];
            let pu = find_root(&mut parent, e.u);
            let pv = find_root(&mut parent, e.v);
            if pu == pv {
                return false; // cycle found
            }
            parent[pu] = pv;
        }
        true
    }

    /// Rank of a subset: maximum acyclic subgraph size = |V| - (connected components).
    pub fn rank_of(&self, edge_indices: &[usize]) -> usize {
        if self.num_vertices == 0 {
            return 0;
        }
        let mut parent: Vec<usize> = (0..self.num_vertices).collect();
        let mut forest_size = 0;
        for &idx in edge_indices {
            if idx >= self.edges.len() {
                continue;
            }
            let e = self.edges[idx];
            let pu = find_root(&mut parent, e.u);
            let pv = find_root(&mut parent, e.v);
            if pu != pv {
                parent[pu] = pv;
                forest_size += 1;
            }
        }
        forest_size
    }

    /// Overall rank = |V| - 1 (if connected), generally |V| - c(G).
    pub fn rank(&self) -> usize {
        let all: Vec<usize> = (0..self.edges.len()).collect();
        self.rank_of(&all)
    }
}

/// Union-Find: find root with path compression.
pub(super) fn find_root(parent: &mut Vec<usize>, x: usize) -> usize {
    if parent[x] != x {
        parent[x] = find_root(parent, parent[x]);
    }
    parent[x]
}

// ─── Partition Matroid ────────────────────────────────────────────────────────

/// A **partition matroid** defined by a partition of the ground set into groups,
/// each with a local rank cap: a set is independent iff it takes ≤ `k_i` elements
/// from the i-th group.
#[derive(Debug, Clone)]
pub struct PartitionMatroid {
    /// Ground set elements.
    pub ground_set: Vec<Element>,
    /// Groups: each group is a set of element indices into `ground_set`.
    pub groups: Vec<Vec<usize>>,
    /// Local rank caps: `caps\[i\]` is the max elements allowed from group i.
    pub caps: Vec<usize>,
}

impl PartitionMatroid {
    /// Create a partition matroid.
    pub fn new(ground_set: Vec<Element>, groups: Vec<Vec<usize>>, caps: Vec<usize>) -> Self {
        assert_eq!(
            groups.len(),
            caps.len(),
            "PartitionMatroid: groups and caps must have same length"
        );
        PartitionMatroid {
            ground_set,
            groups,
            caps,
        }
    }

    /// Test independence of a set (given as element indices into ground_set).
    pub fn is_independent(&self, indices: &[usize]) -> bool {
        let set: HashSet<usize> = indices.iter().copied().collect();
        for (i, group) in self.groups.iter().enumerate() {
            let count = group.iter().filter(|&&g| set.contains(&g)).count();
            if count > self.caps[i] {
                return false;
            }
        }
        true
    }

    /// Rank of a subset.
    pub fn rank_of(&self, indices: &[usize]) -> usize {
        let set: HashSet<usize> = indices.iter().copied().collect();
        let mut rank = 0;
        for (i, group) in self.groups.iter().enumerate() {
            let count = group.iter().filter(|&&g| set.contains(&g)).count();
            rank += count.min(self.caps[i]);
        }
        rank
    }

    /// Overall rank.
    pub fn rank(&self) -> usize {
        self.caps.iter().sum()
    }
}

// ─── Transversal Matroid ──────────────────────────────────────────────────────

/// A **transversal matroid** defined by a family of sets `A_1, ..., A_k` over a
/// ground set E. Independent sets are partial systems of distinct representatives
/// (matchings in the bipartite graph E ↔ {1..k}).
#[derive(Debug, Clone)]
pub struct TransversalMatroid {
    /// Ground set size.
    pub ground_size: usize,
    /// The family of sets; each `sets\[i\]` contains element indices from ground set.
    pub sets: Vec<Vec<usize>>,
}

impl TransversalMatroid {
    /// Create a transversal matroid.
    pub fn new(ground_size: usize, sets: Vec<Vec<usize>>) -> Self {
        TransversalMatroid { ground_size, sets }
    }

    /// Test if a collection of elements can be a partial transversal.
    /// Uses augmenting-path matching (Hopcroft-Karp lite).
    pub fn is_independent(&self, elements: &[usize]) -> bool {
        // Build bipartite graph: elements → sets
        // We need: can we match all `elements` to distinct sets that contain them?
        let k = self.sets.len();
        let mut match_set: Vec<Option<usize>> = vec![None; k]; // match_set[j] = matched element
        for &elem in elements {
            // Try to find augmenting path for `elem`
            let mut visited = vec![false; k];
            if !augment_transversal(&self.sets, elem, &mut match_set, &mut visited) {
                return false;
            }
        }
        true
    }

    /// Rank of a subset = maximum matching size.
    pub fn rank_of(&self, elements: &[usize]) -> usize {
        let k = self.sets.len();
        let mut match_set: Vec<Option<usize>> = vec![None; k];
        let mut count = 0;
        for &elem in elements {
            let mut visited = vec![false; k];
            if augment_transversal(&self.sets, elem, &mut match_set, &mut visited) {
                count += 1;
            }
        }
        count
    }

    /// Overall rank.
    pub fn rank(&self) -> usize {
        let all: Vec<usize> = (0..self.ground_size).collect();
        self.rank_of(&all)
    }
}

/// Augmenting path for bipartite matching.
fn augment_transversal(
    sets: &[Vec<usize>],
    elem: usize,
    match_set: &mut Vec<Option<usize>>,
    visited: &mut Vec<bool>,
) -> bool {
    for (j, set_j) in sets.iter().enumerate() {
        if visited[j] || !set_j.contains(&elem) {
            continue;
        }
        visited[j] = true;
        let can_augment = match match_set[j] {
            None => true,
            Some(matched_elem) => augment_transversal(sets, matched_elem, match_set, visited),
        };
        if can_augment {
            match_set[j] = Some(elem);
            return true;
        }
    }
    false
}

// ─── General Matroid (Oracle Model) ──────────────────────────────────────────

/// A matroid given by an explicit list of its **bases** (maximal independent sets).
///
/// All bases have the same cardinality (the rank), and the exchange axiom holds.
#[derive(Debug, Clone)]
pub struct BasisMatroid {
    /// Ground set.
    pub ground_set: Vec<Element>,
    /// Collection of bases (each is a sorted subset of ground_set indices).
    pub bases: Vec<Vec<usize>>,
    /// Rank (cardinality of any basis).
    pub rank: usize,
}

impl BasisMatroid {
    /// Create a matroid from an explicit list of bases.
    /// Returns `None` if the base list is empty or bases have inconsistent sizes.
    pub fn new(ground_set: Vec<Element>, bases: Vec<Vec<usize>>) -> Option<Self> {
        if bases.is_empty() {
            return None;
        }
        let rank = bases[0].len();
        if bases.iter().any(|b| b.len() != rank) {
            return None;
        }
        Some(BasisMatroid {
            ground_set,
            bases,
            rank,
        })
    }

    /// Test independence: a set is independent iff it is a subset of some basis.
    pub fn is_independent(&self, set: &[usize]) -> bool {
        let set_h: HashSet<usize> = set.iter().copied().collect();
        self.bases
            .iter()
            .any(|b| set_h.iter().all(|e| b.contains(e)))
    }

    /// Rank of a subset.
    pub fn rank_of(&self, set: &[usize]) -> usize {
        let set_h: HashSet<usize> = set.iter().copied().collect();
        let mut max_r = 0;
        // For each basis, compute size of intersection with set
        for b in &self.bases {
            let r = b.iter().filter(|e| set_h.contains(*e)).count();
            if r > max_r {
                max_r = r;
            }
        }
        max_r
    }

    /// Dual matroid: bases are complements of original bases.
    pub fn dual(&self) -> BasisMatroid {
        let n = self.ground_set.len();
        let all: HashSet<usize> = (0..n).collect();
        let dual_bases: Vec<Vec<usize>> = self
            .bases
            .iter()
            .map(|b| {
                let b_set: HashSet<usize> = b.iter().copied().collect();
                let mut compl: Vec<usize> = all.difference(&b_set).copied().collect();
                compl.sort();
                compl
            })
            .collect();
        BasisMatroid {
            ground_set: self.ground_set.clone(),
            bases: dual_bases,
            rank: n - self.rank,
        }
    }
}

// ─── Circuit ──────────────────────────────────────────────────────────────────

/// A **circuit** is a minimal dependent set: it is dependent, but every proper
/// subset is independent.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Circuit {
    /// Elements forming the circuit (sorted).
    pub elements: Vec<usize>,
}

impl Circuit {
    /// Create a circuit from a sorted element list.
    pub fn new(mut elements: Vec<usize>) -> Self {
        elements.sort();
        Circuit { elements }
    }
}

impl fmt::Display for Circuit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Circuit{:?}", self.elements)
    }
}

// ─── Matroid Intersection ─────────────────────────────────────────────────────

/// Input for the matroid intersection algorithm.
///
/// Given two matroids on the same ground set `{0..n-1}`, find the maximum-size
/// common independent set (a set independent in both matroids).
#[derive(Debug, Clone)]
pub struct MatroidIntersectionInput {
    /// Ground set size.
    pub n: usize,
    /// Adjacency relation for matroid 1: is `set` independent in M1?
    pub m1_bases: Vec<Vec<usize>>,
    /// Adjacency relation for matroid 2: is `set` independent in M2?
    pub m2_bases: Vec<Vec<usize>>,
}

/// Result of the matroid intersection algorithm.
#[derive(Debug, Clone)]
pub struct MatroidIntersectionResult {
    /// A maximum common independent set.
    pub common_independent: Vec<usize>,
    /// Its cardinality.
    pub size: usize,
}

// ─── Weighted Matroid (for Greedy Algorithm) ──────────────────────────────────

/// A weighted element on a matroid's ground set.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WeightedElement {
    /// Element index.
    pub index: usize,
    /// Non-negative weight.
    pub weight: f64,
}

impl WeightedElement {
    /// Create a weighted element.
    pub fn new(index: usize, weight: f64) -> Self {
        WeightedElement { index, weight }
    }
}

/// Result of the greedy algorithm: a maximum-weight basis.
#[derive(Debug, Clone)]
pub struct GreedyResult {
    /// The maximum-weight basis.
    pub basis: Vec<usize>,
    /// Total weight of the basis.
    pub total_weight: f64,
}

// ─── Matroid Minor ────────────────────────────────────────────────────────────

/// A **minor** of a matroid is obtained by deletion and/or contraction.
///
/// - **Deletion** `M \ e`: remove element `e` from ground set, keep independent sets
///   not containing `e`.
/// - **Contraction** `M / e`: contract element `e`; a set `S` is independent in `M/e`
///   iff `S ∪ {e}` is independent in M.
#[derive(Debug, Clone)]
pub struct MatroidMinor {
    /// Original basis matroid.
    pub original: BasisMatroid,
    /// Elements deleted from the ground set.
    pub deleted: Vec<usize>,
    /// Elements contracted out.
    pub contracted: Vec<usize>,
}

impl MatroidMinor {
    /// Create a minor by deletion and contraction.
    pub fn new(original: BasisMatroid, deleted: Vec<usize>, contracted: Vec<usize>) -> Self {
        MatroidMinor {
            original,
            deleted,
            contracted,
        }
    }

    /// Test if a set is independent in the minor.
    pub fn is_independent(&self, set: &[usize]) -> bool {
        // No deleted elements allowed
        for &e in set {
            if self.deleted.contains(&e) {
                return false;
            }
        }
        // Combine with contracted elements
        let mut extended: Vec<usize> = set.to_vec();
        for &c in &self.contracted {
            if !extended.contains(&c) {
                extended.push(c);
            }
        }
        self.original.is_independent(&extended)
    }
}

// ─── Matroid Rank Function ─────────────────────────────────────────────────────

/// A **rank function** `r : 2^E → ℕ` characterizes a matroid via:
///
/// 1. `0 ≤ r(A) ≤ |A|`
/// 2. `A ⊆ B ⟹ r(A) ≤ r(B)` (monotonicity)
/// 3. `r(A ∪ B) + r(A ∩ B) ≤ r(A) + r(B)` (submodularity)
///
/// Submodularity is a key property exploited in optimization.
#[derive(Debug, Clone)]
pub struct RankFunction {
    /// Ground set size.
    pub n: usize,
    /// Precomputed rank values for all subsets (indexed as bitmasks for small n).
    /// Only valid when `n ≤ 20` to avoid exponential space.
    pub values: HashMap<u32, usize>,
}

impl RankFunction {
    /// Create from a uniform matroid U(k,n).
    pub fn from_uniform(k: usize, n: usize) -> Self {
        assert!(n <= 20, "RankFunction: n must be ≤ 20 for bitmask encoding");
        let mut values = HashMap::new();
        for mask in 0u32..(1u32 << n) {
            let set_size = mask.count_ones() as usize;
            values.insert(mask, set_size.min(k));
        }
        RankFunction { n, values }
    }

    /// Evaluate rank on a bitmask-encoded subset.
    pub fn eval(&self, mask: u32) -> Option<usize> {
        self.values.get(&mask).copied()
    }

    /// Verify the submodularity axiom for all pairs (brute-force, small n only).
    pub fn verify_submodular(&self) -> bool {
        for a in 0u32..(1u32 << self.n) {
            for b in 0u32..(1u32 << self.n) {
                let union = a | b;
                let inter = a & b;
                let ra = match self.values.get(&a) {
                    Some(&v) => v,
                    None => return false,
                };
                let rb = match self.values.get(&b) {
                    Some(&v) => v,
                    None => return false,
                };
                let runion = match self.values.get(&union) {
                    Some(&v) => v,
                    None => return false,
                };
                let rinter = match self.values.get(&inter) {
                    Some(&v) => v,
                    None => return false,
                };
                if runion + rinter > ra + rb {
                    return false;
                }
            }
        }
        true
    }
}

// ─── Matroid Truncation / Elongation ─────────────────────────────────────────

/// **Truncation** of a matroid to rank `k`: `T_k(M)` has independent sets
/// = independent sets of M with size ≤ k.
#[derive(Debug, Clone)]
pub struct TruncatedMatroid {
    /// Underlying basis matroid.
    pub base: BasisMatroid,
    /// Truncation rank.
    pub trunc_rank: usize,
}

impl TruncatedMatroid {
    /// Create a truncation.
    pub fn new(base: BasisMatroid, trunc_rank: usize) -> Self {
        TruncatedMatroid { base, trunc_rank }
    }

    /// Test independence in the truncation.
    pub fn is_independent(&self, set: &[usize]) -> bool {
        set.len() <= self.trunc_rank && self.base.is_independent(set)
    }

    /// Rank of a subset in the truncation.
    pub fn rank_of(&self, set: &[usize]) -> usize {
        self.base.rank_of(set).min(self.trunc_rank)
    }
}

// ─── Free Extension / Co-extension ───────────────────────────────────────────

/// Result of a matroid **free extension** by adding a new element `e_new`.
///
/// In the free extension, `e_new` can be added to any independent set
/// that has size < rank(M), giving maximal freedom.
#[derive(Debug, Clone)]
pub struct FreeExtension {
    /// Original matroid.
    pub original: BasisMatroid,
    /// Index of the newly added element (= original.ground_set.len()).
    pub new_element: usize,
}

impl FreeExtension {
    /// Create a free extension.
    pub fn new(original: BasisMatroid) -> Self {
        let new_element = original.ground_set.len();
        FreeExtension {
            original,
            new_element,
        }
    }

    /// Test independence in the free extension.
    pub fn is_independent(&self, set: &[usize]) -> bool {
        if set.contains(&self.new_element) {
            // S is independent iff S \ {e_new} is independent and |S \ {e_new}| < rank
            let sub: Vec<usize> = set
                .iter()
                .filter(|&&x| x != self.new_element)
                .copied()
                .collect();
            let sub_rank = self.original.rank_of(&sub);
            self.original.is_independent(&sub) && sub_rank < self.original.rank
        } else {
            self.original.is_independent(set)
        }
    }
}

// ─── Matroid Union ────────────────────────────────────────────────────────────

/// The **union** of two matroids `M1` and `M2` on the same ground set.
///
/// A set `S` is independent in `M1 ∨ M2` iff `S = I1 ∪ I2` for some
/// `I1 ∈ I(M1)` and `I2 ∈ I(M2)`.
#[derive(Debug, Clone)]
pub struct MatroidUnion {
    /// First matroid.
    pub m1: BasisMatroid,
    /// Second matroid.
    pub m2: BasisMatroid,
}

impl MatroidUnion {
    /// Create the union of two matroids on the same ground set.
    pub fn new(m1: BasisMatroid, m2: BasisMatroid) -> Self {
        MatroidUnion { m1, m2 }
    }

    /// Rank of the union: `r_union(S) = min over T⊆S of (r1(T) + r2(S\T) + |S\T|)`.
    /// Actually: `r_union(S) = min_{T ⊆ S} [r1(T) + r2(S) - r2(T) + |S| - |T|]`?
    ///
    /// Uses the matroid union rank formula:
    /// `r_{M1 ∨ M2}(S) = min_{T ⊆ S} { r1(T) + r2(T) + |S \ T| }`.
    pub fn rank_of(&self, set: &[usize]) -> usize {
        // Enumerate all subsets T of set
        let n = set.len();
        let mut min_val = set.len() + self.m1.rank + self.m2.rank; // large upper bound
        for mask in 0u32..(1u32 << n) {
            let t: Vec<usize> = (0..n)
                .filter(|&i| mask & (1 << i) != 0)
                .map(|i| set[i])
                .collect();
            let s_minus_t_size = n - t.len();
            let val = self.m1.rank_of(&t) + self.m2.rank_of(&t) + s_minus_t_size;
            if val < min_val {
                min_val = val;
            }
        }
        min_val
    }
}
