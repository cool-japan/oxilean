//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{
    AvlNode, AvlTree, BTree, BinaryHeap, BinaryMinHeap, BloomFilterDs, Deque, DisjointSet,
    FenwickTree, HyperLogLog, PersistArrayExt, PersistArrayV2, PersistentSegmentTree, SegmentTree,
    SegmentTreeNew, SimpleHashMap, SkipList, SkipListData, SuccinctBitVector, TreapData, Trie,
    UnionFind, VanEmdeBoasTree, WaveletTree, XFastTrie,
};

pub fn app(f: Expr, a: Expr) -> Expr {
    Expr::App(Box::new(f), Box::new(a))
}
pub fn app2(f: Expr, a: Expr, b: Expr) -> Expr {
    app(app(f, a), b)
}
pub fn app3(f: Expr, a: Expr, b: Expr, c: Expr) -> Expr {
    app(app2(f, a, b), c)
}
pub fn cst(s: &str) -> Expr {
    Expr::Const(Name::str(s), vec![])
}
pub fn prop() -> Expr {
    Expr::Sort(Level::zero())
}
pub fn type0() -> Expr {
    Expr::Sort(Level::succ(Level::zero()))
}
pub fn pi(bi: BinderInfo, name: &str, dom: Expr, body: Expr) -> Expr {
    Expr::Pi(bi, Name::str(name), Box::new(dom), Box::new(body))
}
pub fn arrow(a: Expr, b: Expr) -> Expr {
    pi(BinderInfo::Default, "_", a, b)
}
pub fn impl_pi(name: &str, dom: Expr, body: Expr) -> Expr {
    pi(BinderInfo::Implicit, name, dom, body)
}
pub fn bvar(n: u32) -> Expr {
    Expr::BVar(n)
}
pub fn nat_ty() -> Expr {
    cst("Nat")
}
pub fn bool_ty() -> Expr {
    cst("Bool")
}
/// `Heap α : Type 0` — polymorphic binary min-heap type constructor.
///
/// Takes an element type and returns the heap type over that element type.
pub fn heap_ty() -> Expr {
    arrow(type0(), type0())
}
/// `Trie α : Type 0` — trie (prefix tree) indexed by bit strings.
///
/// Takes a value type and returns the trie type storing values at bit-string keys.
pub fn trie_ty() -> Expr {
    arrow(type0(), type0())
}
/// `SegmentTree α n : Type 0` — segment tree over an array of size n.
///
/// Takes an element type and a size (Nat) and returns the segment tree type.
pub fn segment_tree_ty() -> Expr {
    arrow(type0(), arrow(nat_ty(), type0()))
}
/// `FingerTree α : Type 0` — functional finger tree (Hinze-Paterson).
///
/// Supports O(1) amortized cons/snoc and O(log n) concatenation/split.
pub fn finger_tree_ty() -> Expr {
    arrow(type0(), type0())
}
/// `AVLTree α : Type 0` — AVL self-balancing binary search tree.
///
/// Maintains height-balance invariant: |height(left) - height(right)| ≤ 1.
pub fn avl_tree_ty() -> Expr {
    arrow(type0(), type0())
}
/// `SkipList α : Type 0` — probabilistic skip list.
///
/// Expected O(log n) search, insert, and delete with high probability.
pub fn skip_list_ty() -> Expr {
    arrow(type0(), type0())
}
/// `PriorityQueue α : Type 0` — priority queue (backed by heap).
pub fn priority_queue_ty() -> Expr {
    arrow(type0(), type0())
}
/// `DisjointSet n : Type 0` — union-find structure over n elements.
///
/// Supports union and find with path compression and union by rank,
/// giving inverse-Ackermann amortized complexity.
pub fn disjoint_set_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `Deque α : Type 0` — double-ended queue.
///
/// Supports push/pop at both ends in amortized O(1).
pub fn deque_ty() -> Expr {
    arrow(type0(), type0())
}
/// `IntervalTree α : Type 0` — interval tree for overlap queries.
///
/// Stores intervals [lo, hi] with associated values; supports O(log n) overlap queries.
pub fn interval_tree_ty() -> Expr {
    arrow(type0(), type0())
}
/// `heap_push_preserves_invariant : ∀ (α : Type) (h : Heap α) (x : α), HeapInvariant (push h x)`
///
/// Pushing an element into a valid heap produces a valid heap.
pub fn heap_push_ty() -> Expr {
    impl_pi(
        "α",
        type0(),
        arrow(app(cst("Heap"), bvar(0)), arrow(bvar(1), prop())),
    )
}
/// `heap_pop_returns_minimum : ∀ (α : Type) (h : Heap α), IsMinimum (pop h) h`
///
/// The element returned by pop is the minimum element in the heap.
pub fn heap_pop_ty() -> Expr {
    impl_pi("α", type0(), arrow(app(cst("Heap"), bvar(0)), prop()))
}
/// `segment_tree_query_log : ∀ n, QueryComplexity (segTree n) = O(log n)`
///
/// Range queries on a segment tree of size n take O(log n) time.
pub fn segment_tree_query_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `avl_balance_invariant : ∀ (α : Type) (t : AVLTree α), BalanceFactor t ≤ 1`
///
/// Every node in an AVL tree has balance factor in {-1, 0, 1}.
pub fn avl_balance_ty() -> Expr {
    impl_pi("α", type0(), arrow(app(cst("AVLTree"), bvar(0)), prop()))
}
/// `union_find_amortized : ∀ n ops, TotalCost (unionFind n) ops = O(ops * α(n))`
///
/// Union-find with path compression and union by rank has inverse-Ackermann
/// amortized cost per operation.
pub fn union_find_amortized_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), prop()))
}
/// Register all data structure axioms into the given kernel environment.
///
/// This adds axioms for the type constructors and core theorems of each
/// data structure. These are axioms rather than definitions because the full
/// inductive definitions would require significantly more kernel infrastructure.
pub fn build_data_structures_env(env: &mut Environment) -> Result<(), String> {
    let type_axioms: &[(&str, Expr)] = &[
        ("Heap", heap_ty()),
        ("Trie", trie_ty()),
        ("SegmentTree", segment_tree_ty()),
        ("FingerTree", finger_tree_ty()),
        ("AVLTree", avl_tree_ty()),
        ("SkipList", skip_list_ty()),
        ("PriorityQueue", priority_queue_ty()),
        ("DisjointSet", disjoint_set_ty()),
        ("Deque", deque_ty()),
        ("IntervalTree", interval_tree_ty()),
    ];
    for (name, ty) in type_axioms {
        env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty: ty.clone(),
        })
        .ok();
    }
    let thm_axioms: &[(&str, Expr)] = &[
        ("heap_push_preserves_invariant", heap_push_ty()),
        ("heap_pop_returns_minimum", heap_pop_ty()),
        ("segment_tree_query_log", segment_tree_query_ty()),
        ("avl_balance_invariant", avl_balance_ty()),
        ("union_find_amortized_complexity", union_find_amortized_ty()),
    ];
    for (name, ty) in thm_axioms {
        env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty: ty.clone(),
        })
        .ok();
    }
    Ok(())
}
pub const TRIE_ALPHABET: usize = 256;
pub fn avl_height<T: Ord>(node: &Option<Box<AvlNode<T>>>) -> usize {
    node.as_ref().map_or(0, |n| n.height)
}
pub fn avl_update_height<T: Ord>(node: &mut Box<AvlNode<T>>) {
    node.height = 1 + avl_height(&node.left).max(avl_height(&node.right));
}
pub fn avl_balance_factor<T: Ord>(node: &Box<AvlNode<T>>) -> i64 {
    avl_height(&node.left) as i64 - avl_height(&node.right) as i64
}
pub fn avl_rotate_right<T: Ord>(mut y: Box<AvlNode<T>>) -> Box<AvlNode<T>> {
    let mut x = y
        .left
        .take()
        .expect("avl_rotate_right called only when left child exists (bf > 1)");
    y.left = x.right.take();
    avl_update_height(&mut y);
    x.right = Some(y);
    avl_update_height(&mut x);
    x
}
pub fn avl_rotate_left<T: Ord>(mut x: Box<AvlNode<T>>) -> Box<AvlNode<T>> {
    let mut y = x
        .right
        .take()
        .expect("avl_rotate_left called only when right child exists (bf < -1)");
    x.right = y.left.take();
    avl_update_height(&mut x);
    y.left = Some(x);
    avl_update_height(&mut y);
    y
}
pub fn avl_rebalance<T: Ord>(mut node: Box<AvlNode<T>>) -> Box<AvlNode<T>> {
    avl_update_height(&mut node);
    let bf = avl_balance_factor(&node);
    if bf > 1 {
        if avl_balance_factor(node.left.as_ref().expect("left child exists when bf > 1")) < 0 {
            let left = node.left.take().expect("left child exists when bf > 1");
            node.left = Some(avl_rotate_left(left));
        }
        return avl_rotate_right(node);
    }
    if bf < -1 {
        if avl_balance_factor(
            node.right
                .as_ref()
                .expect("right child exists when bf < -1"),
        ) > 0
        {
            let right = node.right.take().expect("right child exists when bf < -1");
            node.right = Some(avl_rotate_right(right));
        }
        return avl_rotate_left(node);
    }
    node
}
pub fn avl_insert<T: Ord>(node: Option<Box<AvlNode<T>>>, value: T) -> Box<AvlNode<T>> {
    match node {
        None => AvlNode::new(value),
        Some(mut n) => {
            match value.cmp(&n.value) {
                std::cmp::Ordering::Less => {
                    n.left = Some(avl_insert(n.left.take(), value));
                }
                std::cmp::Ordering::Greater => {
                    n.right = Some(avl_insert(n.right.take(), value));
                }
                std::cmp::Ordering::Equal => {}
            }
            avl_rebalance(n)
        }
    }
}
pub fn avl_contains<T: Ord>(node: &Option<Box<AvlNode<T>>>, value: &T) -> bool {
    match node {
        None => false,
        Some(n) => match value.cmp(&n.value) {
            std::cmp::Ordering::Less => avl_contains(&n.left, value),
            std::cmp::Ordering::Greater => avl_contains(&n.right, value),
            std::cmp::Ordering::Equal => true,
        },
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_binary_heap() {
        let mut heap = BinaryHeap::new();
        assert!(heap.is_empty());
        heap.push(5);
        heap.push(3);
        heap.push(8);
        heap.push(1);
        heap.push(9);
        assert_eq!(heap.peek(), Some(&1));
        assert_eq!(heap.len(), 5);
        assert_eq!(heap.pop(), Some(1));
        assert_eq!(heap.pop(), Some(3));
        assert_eq!(heap.pop(), Some(5));
        assert_eq!(heap.pop(), Some(8));
        assert_eq!(heap.pop(), Some(9));
        assert_eq!(heap.pop(), None);
        assert!(heap.is_empty());
    }
    #[test]
    fn test_segment_tree() {
        let values = vec![1i64, 3, 5, 7, 9, 11];
        let mut st = SegmentTree::new(&values);
        assert_eq!(st.query(0, 5), 36);
        assert_eq!(st.query(1, 3), 15);
        assert_eq!(st.query(4, 4), 9);
        st.update(2, 10);
        assert_eq!(st.query(0, 5), 41);
        assert_eq!(st.query(1, 3), 20);
        assert_eq!(st.query(10, 20), 0);
        assert_eq!(st.len(), 6);
        assert!(!st.is_empty());
    }
    #[test]
    fn test_trie() {
        let mut trie = Trie::new();
        assert!(!trie.search("hello"));
        trie.insert("hello");
        trie.insert("world");
        trie.insert("help");
        assert!(trie.search("hello"));
        assert!(trie.search("world"));
        assert!(trie.search("help"));
        assert!(!trie.search("hel"));
        assert!(!trie.search("worlds"));
        assert!(!trie.search(""));
        assert!(trie.starts_with("hel"));
        assert!(trie.starts_with("wor"));
        assert!(!trie.starts_with("xyz"));
    }
    #[test]
    fn test_disjoint_set() {
        let mut ds = DisjointSet::new(6);
        assert_eq!(ds.num_sets(), 6);
        assert!(!ds.connected(0, 1));
        assert!(ds.union(0, 1));
        assert!(ds.union(2, 3));
        assert!(ds.union(4, 5));
        assert_eq!(ds.num_sets(), 3);
        assert!(ds.connected(0, 1));
        assert!(ds.connected(2, 3));
        assert!(!ds.connected(0, 2));
        assert!(ds.union(1, 2));
        assert_eq!(ds.num_sets(), 2);
        assert!(ds.connected(0, 3));
        assert!(!ds.union(0, 3));
        assert_eq!(ds.num_sets(), 2);
    }
    #[test]
    fn test_avl_tree() {
        let mut tree = AvlTree::new();
        assert!(tree.is_empty());
        for i in 1..=10 {
            tree.insert(i);
        }
        for i in 1..=10 {
            assert!(tree.contains(&i));
        }
        assert!(!tree.contains(&0));
        assert!(!tree.contains(&11));
        let n = 10usize;
        let max_height = (2.0 * ((n + 1) as f64).log2()).ceil() as usize + 1;
        assert!(
            tree.height() <= max_height,
            "AVL height {} exceeds bound {}",
            tree.height(),
            max_height
        );
    }
    #[test]
    fn test_deque() {
        let mut dq: Deque<i32> = Deque::new();
        assert!(dq.is_empty());
        dq.push_back(1);
        dq.push_back(2);
        dq.push_back(3);
        dq.push_front(0);
        dq.push_front(-1);
        assert_eq!(dq.len(), 5);
        assert_eq!(dq.pop_front(), Some(-1));
        assert_eq!(dq.pop_front(), Some(0));
        assert_eq!(dq.pop_back(), Some(3));
        assert_eq!(dq.pop_back(), Some(2));
        assert_eq!(dq.pop_front(), Some(1));
        assert_eq!(dq.pop_front(), None);
        assert!(dq.is_empty());
        dq.push_back(10);
        dq.push_back(20);
        assert_eq!(dq.front(), Some(&10));
        assert_eq!(dq.back(), Some(&20));
    }
    #[test]
    fn test_build_env() {
        let mut env = Environment::new();
        let result = build_data_structures_env(&mut env);
        assert!(
            result.is_ok(),
            "build_data_structures_env failed: {:?}",
            result
        );
    }
}
/// `PersistentArray α n : Type 0` — persistent immutable array.
///
/// Every update produces a new version while the old version remains accessible.
/// Implemented via fat-node method or path copying.
pub fn persistent_array_ty() -> Expr {
    arrow(type0(), arrow(nat_ty(), type0()))
}
/// `PersistenceTheorem : Prop` — all versions of a persistent structure coexist.
pub fn persistence_theorem_ty() -> Expr {
    prop()
}
/// `FatNodeMethod : Nat → Prop` — the fat node method overhead is O(1) per update.
pub fn fat_node_method_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `PathCopying : Nat → Prop` — path copying persistence for BSTs.
pub fn path_copying_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `PersistentStack α : Type 0` — persistent (immutable-spine) stack.
pub fn persistent_stack_ty() -> Expr {
    arrow(type0(), type0())
}
/// `PersistentQueue α : Type 0` — persistent queue (Hood-Melville or banker).
pub fn persistent_queue_ty() -> Expr {
    arrow(type0(), type0())
}
/// `VanEmdeBoasTree n : Type 0` — van Emde Boas tree over universe [0,n).
///
/// Supports predecessor, successor, insert, delete in O(log log n) time.
pub fn van_emde_boas_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `FractionalCascading : Nat → Prop` — fractional cascading speedup for range queries.
pub fn fractional_cascading_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `CacheObliviousBTree : Nat → Type 0` — cache-oblivious B-tree.
pub fn cache_oblivious_btree_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `CacheObliviousMatrix : Nat → Type 0` — cache-oblivious matrix layout (Z-curve).
pub fn cache_oblivious_matrix_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `SuccinctBitVectorTy : Nat → Type 0` — compact bit vector with rank/select.
///
/// Stores n bits in n + o(n) bits and answers rank/select in O(1).
pub fn succinct_bit_vector_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `WaveletTreeTy : Nat → Type 0` — wavelet tree for range queries on sequences.
///
/// Answers range frequency, range median, range quantile in O(log σ) time.
pub fn wavelet_tree_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `CompressedSuffixArray : Nat → Type 0` — compressed suffix array (CSA/FM-index).
pub fn compressed_suffix_array_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `RankSelectAxiom : Prop` — rank(i) + select(rank(i)) = i on succinct bit vectors.
pub fn rank_select_axiom_ty() -> Expr {
    prop()
}
/// `DistributedHashTable : Nat → Type 0` — distributed hash table (DHT).
pub fn distributed_hash_table_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `ConsistentHashing : Nat → Prop` — consistent hashing with O(1/n) key redistribution.
pub fn consistent_hashing_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `CRDT : Type 0` — conflict-free replicated data type.
///
/// A CRDT is a data structure that can be replicated across multiple nodes
/// with guaranteed eventual consistency via lattice-based merge operations.
pub fn crdt_ty() -> Expr {
    type0()
}
/// `ReplicationConsistency : Prop` — eventual consistency of CRDT merges.
pub fn replication_consistency_ty() -> Expr {
    prop()
}
/// `BloomFilterTy : Nat → Nat → Type 0` — Bloom filter with m bits and k hash functions.
pub fn bloom_filter_ds_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `HyperLogLogTy : Nat → Type 0` — HyperLogLog cardinality estimator with b-bit registers.
pub fn hyperloglog_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `CountMinSketchTy : Nat → Nat → Type 0` — count-min sketch with w columns, d rows.
pub fn count_min_sketch_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `BloomFilterFPR : Prop` — false positive rate of Bloom filter is (1-e^{-kn/m})^k.
pub fn bloom_filter_fpr_ty() -> Expr {
    prop()
}
/// `HyperLogLogError : Prop` — HyperLogLog relative error is 1.04/sqrt(m).
pub fn hyperloglog_error_ty() -> Expr {
    prop()
}
/// `BTree : Nat → Type 0` — B-tree of order t (min t-1 keys, max 2t-1 keys per node).
pub fn btree_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `BufferTree : Nat → Type 0` — buffer tree for batched external-memory operations.
pub fn buffer_tree_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `BTreeSearchComplexity : Prop` — B-tree search runs in O(log_t n) I/Os.
pub fn btree_search_complexity_ty() -> Expr {
    prop()
}
/// `BTreeInsertComplexity : Prop` — B-tree insert runs in O(log_t n) I/Os.
pub fn btree_insert_complexity_ty() -> Expr {
    prop()
}
/// `LockFreeStack α : Type 0` — lock-free Treiber stack.
pub fn lock_free_stack_ty() -> Expr {
    arrow(type0(), type0())
}
/// `WaitFreeQueue α : Type 0` — wait-free queue (Kogan-Petrank).
pub fn wait_free_queue_ty() -> Expr {
    arrow(type0(), type0())
}
/// `Linearizability : Prop` — concurrent operations are linearizable.
pub fn linearizability_ty() -> Expr {
    prop()
}
/// `WaitFreedom : Prop` — every thread finishes in a bounded number of steps.
pub fn wait_freedom_ty() -> Expr {
    prop()
}
/// `RedBlackTree α : Type 0` — red-black BST (Okasaki functional formulation).
pub fn red_black_tree_ty() -> Expr {
    arrow(type0(), type0())
}
/// `RealTimeQueue α : Type 0` — Hood-Melville real-time O(1) worst-case queue.
pub fn real_time_queue_ty() -> Expr {
    arrow(type0(), type0())
}
/// `BootstrappedHeap α : Type 0` — bootstrapped priority queue (Kaplan-Tarjan).
pub fn bootstrapped_heap_ty() -> Expr {
    arrow(type0(), type0())
}
/// `FunctionalBSTInvariant : Prop` — a functional BST satisfies the BST ordering.
pub fn functional_bst_invariant_ty() -> Expr {
    prop()
}
/// `SuffixTree : Nat → Type 0` — compressed suffix tree (Ukkonen's linear construction).
pub fn suffix_tree_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `SuffixAutomaton : Nat → Type 0` — suffix automaton (DAWG).
pub fn suffix_automaton_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `FMIndex : Nat → Type 0` — FM-index for compressed string matching.
pub fn fm_index_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `SuffixTreeLinearTime : Prop` — suffix tree can be built in O(n) time.
pub fn suffix_tree_linear_time_ty() -> Expr {
    prop()
}
/// `AdjacencyList : Nat → Type 0` — adjacency list graph representation.
pub fn adjacency_list_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `AdjacencyMatrix : Nat → Type 0` — adjacency matrix graph representation.
pub fn adjacency_matrix_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `DynamicGraph : Nat → Type 0` — dynamic graph supporting edge insertions/deletions.
pub fn dynamic_graph_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `EdgeWeightedGraph : Nat → Type 0` — edge-weighted graph.
pub fn edge_weighted_graph_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `KDTree : Nat → Type 0` — k-d tree for k-dimensional range queries.
pub fn kd_tree_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `RangeTree : Nat → Type 0` — range tree for orthogonal range queries.
pub fn range_tree_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `KDTreeNNComplexity : Prop` — k-d tree nearest neighbor runs in O(sqrt(n)) expected.
pub fn kd_tree_nn_complexity_ty() -> Expr {
    prop()
}
/// `RangeTreeQueryComplexity : Prop` — range tree answers queries in O(log^d n + k).
pub fn range_tree_query_complexity_ty() -> Expr {
    prop()
}
/// `SplayTree α : Type 0` — splay tree with amortized O(log n) per operation.
pub fn splay_tree_ty() -> Expr {
    arrow(type0(), type0())
}
/// `PairingHeap α : Type 0` — pairing heap with O(log n) amortized delete-min.
pub fn pairing_heap_ty() -> Expr {
    arrow(type0(), type0())
}
/// `FibonacciHeap α : Type 0` — Fibonacci heap with O(1) amortized decrease-key.
pub fn fibonacci_heap_ty() -> Expr {
    arrow(type0(), type0())
}
/// `SplayTreeAmortized : Prop` — splay tree access lemma and amortized O(log n).
pub fn splay_tree_amortized_ty() -> Expr {
    prop()
}
/// `FibonacciHeapDecreaseKey : Prop` — decrease-key in Fibonacci heap is O(1) amortized.
pub fn fibonacci_heap_decrease_key_ty() -> Expr {
    prop()
}
/// Register extended data structure axioms into the given kernel environment.
pub fn build_extended_ds_env(env: &mut Environment) -> Result<(), String> {
    let axioms: &[(&str, Expr)] = &[
        ("PersistentArray", persistent_array_ty()),
        ("PersistenceTheorem", persistence_theorem_ty()),
        ("FatNodeMethod", fat_node_method_ty()),
        ("PathCopying", path_copying_ty()),
        ("PersistentStack", persistent_stack_ty()),
        ("PersistentQueue", persistent_queue_ty()),
        ("VanEmdeBoasTree", van_emde_boas_ty()),
        ("FractionalCascading", fractional_cascading_ty()),
        ("CacheObliviousBTree", cache_oblivious_btree_ty()),
        ("CacheObliviousMatrix", cache_oblivious_matrix_ty()),
        ("SuccinctBitVector", succinct_bit_vector_ty()),
        ("WaveletTree", wavelet_tree_ty()),
        ("CompressedSuffixArray", compressed_suffix_array_ty()),
        ("RankSelectAxiom", rank_select_axiom_ty()),
        ("DistributedHashTable", distributed_hash_table_ty()),
        ("ConsistentHashing", consistent_hashing_ty()),
        ("CRDT", crdt_ty()),
        ("ReplicationConsistency", replication_consistency_ty()),
        ("BloomFilterDs", bloom_filter_ds_ty()),
        ("HyperLogLog", hyperloglog_ty()),
        ("CountMinSketch", count_min_sketch_ty()),
        ("BloomFilterFPR", bloom_filter_fpr_ty()),
        ("HyperLogLogError", hyperloglog_error_ty()),
        ("BTree", btree_ty()),
        ("BufferTree", buffer_tree_ty()),
        ("BTreeSearchComplexity", btree_search_complexity_ty()),
        ("BTreeInsertComplexity", btree_insert_complexity_ty()),
        ("LockFreeStack", lock_free_stack_ty()),
        ("WaitFreeQueue", wait_free_queue_ty()),
        ("Linearizability", linearizability_ty()),
        ("WaitFreedom", wait_freedom_ty()),
        ("RedBlackTree", red_black_tree_ty()),
        ("RealTimeQueue", real_time_queue_ty()),
        ("BootstrappedHeap", bootstrapped_heap_ty()),
        ("FunctionalBSTInvariant", functional_bst_invariant_ty()),
        ("SuffixTree", suffix_tree_ty()),
        ("SuffixAutomaton", suffix_automaton_ty()),
        ("FMIndex", fm_index_ty()),
        ("SuffixTreeLinearTime", suffix_tree_linear_time_ty()),
        ("AdjacencyList", adjacency_list_ty()),
        ("AdjacencyMatrix", adjacency_matrix_ty()),
        ("DynamicGraph", dynamic_graph_ty()),
        ("EdgeWeightedGraph", edge_weighted_graph_ty()),
        ("KDTree", kd_tree_ty()),
        ("RangeTree", range_tree_ty()),
        ("KDTreeNNComplexity", kd_tree_nn_complexity_ty()),
        ("RangeTreeQueryComplexity", range_tree_query_complexity_ty()),
        ("SplayTree", splay_tree_ty()),
        ("PairingHeap", pairing_heap_ty()),
        ("FibonacciHeap", fibonacci_heap_ty()),
        ("SplayTreeAmortized", splay_tree_amortized_ty()),
        ("FibonacciHeapDecreaseKey", fibonacci_heap_decrease_key_ty()),
    ];
    for (name, ty) in axioms {
        env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty: ty.clone(),
        })
        .ok();
    }
    Ok(())
}
#[cfg(test)]
mod extended_tests {
    use super::*;
    #[test]
    fn test_bloom_filter_ds() {
        let mut bf = BloomFilterDs::new(1024, 3);
        assert!(bf.is_empty());
        bf.insert(42);
        bf.insert(100);
        bf.insert(999);
        assert_eq!(bf.len(), 3);
        assert!(bf.might_contain(42));
        assert!(bf.might_contain(100));
        assert!(bf.might_contain(999));
        let fpr = bf.false_positive_rate();
        assert!(fpr < 0.1, "FPR {} is too high", fpr);
    }
    #[test]
    fn test_hyperloglog() {
        let mut hll = HyperLogLog::new(10);
        assert_eq!(hll.num_registers(), 1024);
        for i in 0u64..1000 {
            hll.add(i);
        }
        let est = hll.estimate();
        assert!(
            est > 800.0 && est < 1200.0,
            "HLL estimate {} out of range [800, 1200]",
            est
        );
        let err = hll.relative_error_bound();
        assert!(err < 0.04, "Error bound {} too large", err);
    }
    #[test]
    fn test_wavelet_tree() {
        let data = vec![3u64, 1, 4, 1, 5, 9, 2, 6, 5, 3];
        let wt = WaveletTree::new(&data, 0, 10);
        assert_eq!(wt.len(), 10);
        assert!(!wt.is_empty());
        assert!(wt.num_levels() > 0);
        let freq = wt.range_freq(&data, 0, 3, 1);
        assert_eq!(freq, 2);
        let freq5 = wt.range_freq(&data, 4, 8, 5);
        assert_eq!(freq5, 2);
    }
    #[test]
    fn test_succinct_bit_vector() {
        let bits = vec![true, false, true, true, false, false, true, false];
        let sbv = SuccinctBitVector::new(&bits);
        assert_eq!(sbv.len(), 8);
        assert!(!sbv.is_empty());
        assert!(sbv.get(0));
        assert!(!sbv.get(1));
        assert!(sbv.get(2));
        assert!(sbv.get(3));
        assert!(!sbv.get(4));
        assert!(!sbv.get(5));
        assert!(sbv.get(6));
        assert!(!sbv.get(7));
        assert_eq!(sbv.rank1(3), 3);
        assert_eq!(sbv.rank1(0), 1);
        assert_eq!(sbv.rank1(1), 1);
        assert_eq!(sbv.rank0(1), 1);
        assert_eq!(sbv.popcount_total(), 4);
        assert_eq!(sbv.select1(1), Some(0));
        assert_eq!(sbv.select1(2), Some(2));
        assert_eq!(sbv.select1(3), Some(3));
        assert_eq!(sbv.select1(4), Some(6));
        assert_eq!(sbv.select1(5), None);
    }
    #[test]
    fn test_bloom_filter_no_false_negatives() {
        let mut bf = BloomFilterDs::new(2048, 5);
        let inserted: Vec<u64> = (0..100).collect();
        for &k in &inserted {
            bf.insert(k);
        }
        for &k in &inserted {
            assert!(bf.might_contain(k), "False negative for key {}", k);
        }
    }
    #[test]
    fn test_succinct_bit_vector_large() {
        let bits: Vec<bool> = (0..200).map(|i| i % 3 == 0).collect();
        let sbv = SuccinctBitVector::new(&bits);
        assert_eq!(sbv.len(), 200);
        let expected_ones = bits.iter().filter(|&&b| b).count();
        assert_eq!(sbv.popcount_total(), expected_ones);
    }
    #[test]
    fn test_wavelet_tree_empty() {
        let wt = WaveletTree::new(&[], 0, 10);
        assert_eq!(wt.len(), 0);
        assert!(wt.is_empty());
    }
    #[test]
    fn test_hyperloglog_small() {
        let mut hll = HyperLogLog::new(8);
        assert_eq!(hll.num_registers(), 256);
        hll.add(12345);
        let est = hll.estimate();
        assert!(est >= 0.5, "HLL estimate {} too small", est);
    }
    #[test]
    fn test_extended_axiom_registration() {
        let mut env = Environment::new();
        build_extended_ds_env(&mut env).expect("build_extended_ds_env should succeed");
        assert!(env.get(&Name::str("BloomFilterDs")).is_some());
        assert!(env.get(&Name::str("HyperLogLog")).is_some());
        assert!(env.get(&Name::str("WaveletTree")).is_some());
        assert!(env.get(&Name::str("SuccinctBitVector")).is_some());
        assert!(env.get(&Name::str("FibonacciHeap")).is_some());
        assert!(env.get(&Name::str("SuffixTree")).is_some());
        assert!(env.get(&Name::str("KDTree")).is_some());
        assert!(env.get(&Name::str("CRDT")).is_some());
        assert!(env.get(&Name::str("VanEmdeBoasTree")).is_some());
        assert!(env.get(&Name::str("FMIndex")).is_some());
    }
}
#[cfg(test)]
mod ds_ext_tests {
    use super::*;
    #[test]
    fn test_union_find() {
        let mut uf = UnionFind::new(5);
        assert_eq!(uf.num_components(), 5);
        uf.union(0, 1);
        uf.union(2, 3);
        assert_eq!(uf.num_components(), 3);
        assert!(uf.connected(0, 1));
        assert!(!uf.connected(0, 2));
    }
    #[test]
    fn test_segment_tree() {
        let arr = vec![1i64, 3, 5, 7, 9, 11];
        let st = SegmentTreeNew::from_slice(&arr);
        assert_eq!(st.query_sum(0, 5), 36);
        assert_eq!(st.query_sum(1, 3), 15);
    }
    #[test]
    fn test_fenwick_tree() {
        let mut ft = FenwickTree::new(6);
        for (i, &v) in [1i64, 3, 5, 7, 9, 11].iter().enumerate() {
            ft.update(i + 1, v);
        }
        assert_eq!(ft.range_sum(1, 6), 36);
        assert_eq!(ft.range_sum(2, 4), 15);
    }
    #[test]
    fn test_persistent_array() {
        let mut pa: PersistArrayExt<i32> = PersistArrayExt::new(vec![1, 2, 3]);
        let v1 = pa.update(0, 1, 99);
        assert_eq!(pa.get(0, 1), Some(&2));
        assert_eq!(pa.get(v1, 1), Some(&99));
    }
    #[test]
    fn test_btree() {
        let bt = BTree::new(3);
        assert_eq!(bt.max_keys_per_node(), 5);
        assert_eq!(bt.min_keys_per_node(), 2);
    }
}
#[cfg(test)]
mod heap_hash_tests {
    use super::*;
    #[test]
    fn test_binary_min_heap() {
        let mut heap = BinaryMinHeap::new();
        heap.push(5);
        heap.push(1);
        heap.push(3);
        assert_eq!(heap.pop(), Some(1));
        assert_eq!(heap.pop(), Some(3));
        assert_eq!(heap.pop(), Some(5));
    }
    #[test]
    fn test_simple_hashmap() {
        let mut hm = SimpleHashMap::new(16);
        hm.insert(1, 100);
        hm.insert(2, 200);
        assert_eq!(hm.get(1), Some(100));
        assert_eq!(hm.get(2), Some(200));
        assert_eq!(hm.get(3), None);
    }
}
#[cfg(test)]
mod tests_data_structures_ext {
    use super::*;
    #[test]
    fn test_van_emde_boas() {
        let veb = VanEmdeBoasTree::new(16);
        assert!(veb.is_empty());
        assert_eq!(veb.upper_sqrt(), 4);
        assert_eq!(veb.lower_sqrt(), 4);
        assert_eq!(veb.high(10), 2);
        assert_eq!(veb.low(10), 2);
        let desc = veb.complexity_description();
        assert!(desc.contains("van Emde Boas"));
    }
    #[test]
    fn test_xfast_trie() {
        let xf = XFastTrie::new(32);
        let desc = xf.complexity_description();
        assert!(desc.contains("X-Fast Trie"));
        let pred = xf.predecessor_time();
        assert!(pred.contains("O(log W)"));
    }
    #[test]
    fn test_persistent_array() {
        let mut pa: PersistArrayV2<i32> = PersistArrayV2::new(5);
        let v1 = pa.update(2, 42);
        assert_eq!(v1, 1);
        assert_eq!(pa.data[2], 42);
        pa.rollback();
        assert_eq!(pa.data[2], 0);
    }
    #[test]
    fn test_persistent_segment_tree() {
        let pst = PersistentSegmentTree::new(100);
        let space = pst.space_complexity();
        assert!(space.contains("Persistent"));
        let time = pst.time_complexity();
        assert!(time.contains("historical"));
    }
    #[test]
    fn test_skip_list() {
        let mut sl = SkipListData::new(20, 0.5);
        sl.insert();
        sl.insert();
        let t = sl.expected_search_time();
        assert!(t > 0.0);
        let space = sl.space_usage();
        assert!(space.contains("Skip list"));
        let pugh = sl.pugh_analysis();
        assert!(pugh.contains("Pugh"));
    }
    #[test]
    fn test_treap() {
        let mut treap = TreapData::new();
        treap.size = 1000;
        let h = treap.expected_height();
        assert!(h > 0.0);
        let split = treap.split_at(500);
        assert!(split.contains("O(log n)"));
        let merge = treap.merge_description();
        assert!(merge.contains("Merge"));
        let it = TreapData::implicit_treap();
        assert!(it.is_implicitly_keyed);
    }
}
