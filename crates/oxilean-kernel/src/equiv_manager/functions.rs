//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::Expr;
use std::collections::{HashMap, HashSet};

use super::types::{
    AnnotationTable, BeforeAfter, BiMap, CheckedEquiv, DiagMeta, EquivClass, EquivManager,
    EquivManagerStats, EquivProofTerm, EquivQuery, EquivStats, EquivalenceTable, EventCounter,
    ExprEquivCache, FrequencyTable, Generation, IdDispenser, IndexEquivManager,
    InstrumentedEquivManager, IntervalSet, LoopClock, MemoSlot, PersistentEquivManager, RingBuffer,
    ScopeStack, ScopedEquivManager, SeqNum, SimpleLruCache, Slot, SparseBitSet, StringInterner,
    SymmetricRelation, Timestamp, TypedId, UnionFind, WorkQueue, WorkStack,
};

/// Create a canonical pair ordering for cache lookups.
///
/// We need `(a, b)` and `(b, a)` to map to the same cache key.
/// We use a deterministic ordering based on expression debug representation.
pub(super) fn canonicalize_pair(a: Expr, b: Expr) -> (Expr, Expr) {
    if format!("{:?}", a) <= format!("{:?}", b) {
        (a, b)
    } else {
        (b, a)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Level, Name};
    fn nat() -> Expr {
        Expr::Const(Name::str("Nat"), vec![])
    }
    fn bool_expr() -> Expr {
        Expr::Const(Name::str("Bool"), vec![])
    }
    fn int() -> Expr {
        Expr::Const(Name::str("Int"), vec![])
    }
    #[test]
    fn test_equiv_reflexive() {
        let mut mgr = EquivManager::new();
        assert!(mgr.is_equiv(&nat(), &nat()));
    }
    #[test]
    fn test_add_and_check_equiv() {
        let mut mgr = EquivManager::new();
        mgr.add_equiv(&nat(), &bool_expr());
        assert!(mgr.is_equiv(&nat(), &bool_expr()));
        assert!(mgr.is_equiv(&bool_expr(), &nat()));
    }
    #[test]
    fn test_equiv_transitivity() {
        let mut mgr = EquivManager::new();
        mgr.add_equiv(&nat(), &bool_expr());
        mgr.add_equiv(&bool_expr(), &int());
        assert!(mgr.is_equiv(&nat(), &int()));
    }
    #[test]
    fn test_failure_cache() {
        let mut mgr = EquivManager::new();
        mgr.add_failure(&nat(), &bool_expr());
        assert!(mgr.is_failure(&nat(), &bool_expr()));
        assert!(mgr.is_failure(&bool_expr(), &nat()));
        assert!(!mgr.is_failure(&nat(), &int()));
    }
    #[test]
    fn test_not_equiv() {
        let mut mgr = EquivManager::new();
        assert!(!mgr.is_equiv(&nat(), &bool_expr()));
    }
    #[test]
    fn test_clear() {
        let mut mgr = EquivManager::new();
        mgr.add_equiv(&nat(), &bool_expr());
        mgr.add_failure(&nat(), &int());
        assert_eq!(mgr.num_equiv(), 1);
        assert_eq!(mgr.num_failures(), 1);
        mgr.clear();
        assert_eq!(mgr.num_equiv(), 0);
        assert_eq!(mgr.num_failures(), 0);
        assert!(!mgr.is_equiv(&nat(), &bool_expr()));
    }
    #[test]
    fn test_many_equivalences() {
        let mut mgr = EquivManager::new();
        let exprs: Vec<Expr> = (0..10)
            .map(|i| Expr::Const(Name::str(format!("T{}", i)), vec![]))
            .collect();
        for i in 0..9 {
            mgr.add_equiv(&exprs[i], &exprs[i + 1]);
        }
        assert!(mgr.is_equiv(&exprs[0], &exprs[9]));
        assert!(mgr.is_equiv(&exprs[3], &exprs[7]));
    }
    #[test]
    fn test_sort_equivalence() {
        let mut mgr = EquivManager::new();
        let s0 = Expr::Sort(Level::zero());
        let s1 = Expr::Sort(Level::succ(Level::zero()));
        mgr.add_equiv(&s0, &s1);
        assert!(mgr.is_equiv(&s0, &s1));
    }
}
#[cfg(test)]
mod equiv_extended_tests {
    use super::*;
    use crate::Name;
    fn nat() -> Expr {
        Expr::Const(Name::str("Nat"), vec![])
    }
    fn bool_expr() -> Expr {
        Expr::Const(Name::str("Bool"), vec![])
    }
    fn int() -> Expr {
        Expr::Const(Name::str("Int"), vec![])
    }
    fn str_ty() -> Expr {
        Expr::Const(Name::str("String"), vec![])
    }
    #[test]
    fn test_scoped_manager_basic() {
        let mut mgr = ScopedEquivManager::new();
        mgr.add_equiv(&nat(), &bool_expr());
        assert!(mgr.is_equiv(&nat(), &bool_expr()));
        assert_eq!(mgr.scope_depth(), 1);
    }
    #[test]
    fn test_scoped_manager_push_pop() {
        let mut mgr = ScopedEquivManager::new();
        mgr.add_equiv(&nat(), &bool_expr());
        mgr.push_scope();
        assert_eq!(mgr.scope_depth(), 2);
        mgr.add_equiv(&nat(), &int());
        assert!(mgr.is_equiv(&nat(), &int()));
        mgr.pop_scope();
        assert_eq!(mgr.scope_depth(), 1);
        assert!(mgr.is_equiv(&nat(), &bool_expr()));
    }
    #[test]
    fn test_scoped_manager_total_equivs() {
        let mut mgr = ScopedEquivManager::new();
        mgr.add_equiv(&nat(), &bool_expr());
        mgr.push_scope();
        mgr.add_equiv(&int(), &str_ty());
        assert_eq!(mgr.total_equivs(), 2);
    }
    #[test]
    fn test_equiv_stats_default() {
        let stats = EquivStats::new();
        assert_eq!(stats.total_queries, 0);
        assert_eq!(stats.hit_rate(), 1.0);
    }
    #[test]
    fn test_equiv_stats_recording() {
        let mut stats = EquivStats::new();
        stats.record_equiv_hit();
        stats.record_equiv_hit();
        stats.record_miss();
        assert_eq!(stats.total_queries, 3);
        assert_eq!(stats.equiv_hits, 2);
        assert!((stats.hit_rate() - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_instrumented_manager() {
        let mut mgr = InstrumentedEquivManager::new();
        mgr.add_equiv(&nat(), &bool_expr());
        let result = mgr.is_equiv(&nat(), &bool_expr());
        assert!(result);
        assert_eq!(mgr.stats().equiv_additions, 1);
        assert_eq!(mgr.stats().total_queries, 1);
    }
    #[test]
    fn test_instrumented_manager_clear() {
        let mut mgr = InstrumentedEquivManager::new();
        mgr.add_equiv(&nat(), &bool_expr());
        mgr.clear();
        assert_eq!(mgr.stats().total_queries, 0);
        assert_eq!(mgr.stats().equiv_additions, 0);
    }
    #[test]
    fn test_equiv_query() {
        let mut mgr = EquivManager::new();
        mgr.add_failure(&nat(), &bool_expr());
        let query = EquivQuery::new(&mgr);
        assert!(query.is_known_failure(&nat(), &bool_expr()));
        assert!(!query.is_known_failure(&nat(), &int()));
        assert_eq!(query.num_failures(), 1);
    }
    #[test]
    fn test_scoped_manager_pop_at_root() {
        let mut mgr = ScopedEquivManager::new();
        mgr.pop_scope();
        assert_eq!(mgr.scope_depth(), 1);
    }
    #[test]
    fn test_instrumented_add_failure() {
        let mut mgr = InstrumentedEquivManager::new();
        mgr.add_failure(&nat(), &bool_expr());
        assert!(mgr.is_failure(&nat(), &bool_expr()));
        assert_eq!(mgr.stats().failure_additions, 1);
    }
}
/// Compute the equivalence classes of a set of expressions.
///
/// Groups expressions that are known to be equivalent with each other.
/// Uses simple transitive closure via repeated passes.
pub fn compute_equiv_classes(exprs: &[Expr], pairs: &[(Expr, Expr)]) -> Vec<Vec<usize>> {
    let n = exprs.len();
    let mut parent: Vec<usize> = (0..n).collect();
    fn find(parent: &mut Vec<usize>, x: usize) -> usize {
        if parent[x] != x {
            parent[x] = find(parent, parent[x]);
        }
        parent[x]
    }
    fn union(parent: &mut Vec<usize>, x: usize, y: usize) {
        let rx = find(parent, x);
        let ry = find(parent, y);
        if rx != ry {
            parent[rx] = ry;
        }
    }
    for (a, b) in pairs {
        let ia = exprs.iter().position(|e| e == a);
        let ib = exprs.iter().position(|e| e == b);
        if let (Some(i), Some(j)) = (ia, ib) {
            union(&mut parent, i, j);
        }
    }
    let mut classes: std::collections::HashMap<usize, Vec<usize>> =
        std::collections::HashMap::new();
    for i in 0..n {
        let root = find(&mut parent, i);
        classes.entry(root).or_default().push(i);
    }
    classes.into_values().collect()
}
#[cfg(test)]
mod persistent_equiv_tests {
    use super::*;
    use crate::Name;
    fn nat() -> Expr {
        Expr::Const(Name::str("Nat"), vec![])
    }
    fn bool_expr() -> Expr {
        Expr::Const(Name::str("Bool"), vec![])
    }
    fn int() -> Expr {
        Expr::Const(Name::str("Int"), vec![])
    }
    #[test]
    fn test_persistent_manager_basic() {
        let mut mgr = PersistentEquivManager::new();
        mgr.add_equiv(&nat(), &bool_expr());
        assert!(mgr.is_equiv(&nat(), &bool_expr()));
        assert!(mgr.is_equiv(&bool_expr(), &nat()));
        assert!(!mgr.is_equiv(&nat(), &int()));
    }
    #[test]
    fn test_persistent_manager_reflexive() {
        let mgr = PersistentEquivManager::new();
        assert!(mgr.is_equiv(&nat(), &nat()));
    }
    #[test]
    fn test_persistent_manager_no_dups() {
        let mut mgr = PersistentEquivManager::new();
        mgr.add_equiv(&nat(), &bool_expr());
        mgr.add_equiv(&nat(), &bool_expr());
        assert_eq!(mgr.len(), 1);
    }
    #[test]
    fn test_persistent_manager_serialize() {
        let mut mgr = PersistentEquivManager::new();
        mgr.add_equiv(&nat(), &bool_expr());
        let serialized = mgr.serialize();
        assert_eq!(serialized.len(), 1);
    }
    #[test]
    fn test_compute_equiv_classes_basic() {
        let exprs = vec![
            Expr::Const(Name::str("A"), vec![]),
            Expr::Const(Name::str("B"), vec![]),
            Expr::Const(Name::str("C"), vec![]),
        ];
        let pairs = vec![(exprs[0].clone(), exprs[1].clone())];
        let classes = compute_equiv_classes(&exprs, &pairs);
        assert_eq!(classes.len(), 2);
        let sizes: std::collections::HashSet<usize> = classes.iter().map(|c| c.len()).collect();
        assert!(sizes.contains(&2));
        assert!(sizes.contains(&1));
    }
    #[test]
    fn test_compute_equiv_classes_all_separate() {
        let exprs = vec![
            Expr::Const(Name::str("A"), vec![]),
            Expr::Const(Name::str("B"), vec![]),
        ];
        let classes = compute_equiv_classes(&exprs, &[]);
        assert_eq!(classes.len(), 2);
        assert!(classes.iter().all(|c| c.len() == 1));
    }
    #[test]
    fn test_compute_equiv_classes_all_same() {
        let nat = Expr::Const(Name::str("A"), vec![]);
        let bool_e = Expr::Const(Name::str("B"), vec![]);
        let int_e = Expr::Const(Name::str("C"), vec![]);
        let exprs = vec![nat.clone(), bool_e.clone(), int_e.clone()];
        let pairs = vec![
            (nat.clone(), bool_e.clone()),
            (bool_e.clone(), int_e.clone()),
        ];
        let classes = compute_equiv_classes(&exprs, &pairs);
        assert_eq!(classes.len(), 1);
        assert_eq!(classes[0].len(), 3);
    }
    #[test]
    fn test_persistent_manager_is_empty() {
        let mgr = PersistentEquivManager::new();
        assert!(mgr.is_empty());
        let mut mgr2 = PersistentEquivManager::new();
        mgr2.add_equiv(&nat(), &bool_expr());
        assert!(!mgr2.is_empty());
    }
    #[test]
    fn test_persistent_manager_sorted_output() {
        let mut mgr = PersistentEquivManager::new();
        mgr.add_equiv(&int(), &nat());
        mgr.add_equiv(&bool_expr(), &nat());
        let serialized = mgr.serialize();
        for i in 0..serialized.len().saturating_sub(1) {
            assert!(serialized[i] <= serialized[i + 1]);
        }
    }
}
/// Compute the transitive closure of a relation on a set of expressions.
///
/// Given a list of initial pairs, returns all pairs (i, j) such that
/// i and j are in the same equivalence class.
#[allow(dead_code)]
pub fn transitive_closure(pairs: &[(usize, usize)], n: usize) -> Vec<(usize, usize)> {
    let mut parent: Vec<usize> = (0..n).collect();
    fn find(parent: &mut Vec<usize>, x: usize) -> usize {
        if parent[x] != x {
            parent[x] = find(parent, parent[x]);
        }
        parent[x]
    }
    for &(a, b) in pairs {
        if a < n && b < n {
            let ra = find(&mut parent, a);
            let rb = find(&mut parent, b);
            if ra != rb {
                parent[ra] = rb;
            }
        }
    }
    let mut result = Vec::new();
    for i in 0..n {
        for j in (i + 1)..n {
            if find(&mut parent, i) == find(&mut parent, j) {
                result.push((i, j));
            }
        }
    }
    result
}
/// Map an expression list to its equivalence-class representatives.
///
/// Returns an index array where `repr[i]` is the representative index for `i`.
#[allow(dead_code)]
pub fn compute_representatives(pairs: &[(usize, usize)], n: usize) -> Vec<usize> {
    let mut parent: Vec<usize> = (0..n).collect();
    fn find(parent: &mut Vec<usize>, x: usize) -> usize {
        if parent[x] != x {
            parent[x] = find(parent, parent[x]);
        }
        parent[x]
    }
    for &(a, b) in pairs {
        if a < n && b < n {
            let ra = find(&mut parent, a);
            let rb = find(&mut parent, b);
            if ra != rb {
                parent[ra] = rb;
            }
        }
    }
    (0..n).map(|i| find(&mut parent, i)).collect()
}
/// Find all equivalence classes given an explicit list of pairs.
///
/// Returns a `Vec<Vec<usize>>` where each inner vec is one class.
#[allow(dead_code)]
pub fn find_classes(pairs: &[(usize, usize)], n: usize) -> Vec<Vec<usize>> {
    let repr = compute_representatives(pairs, n);
    let mut classes: std::collections::HashMap<usize, Vec<usize>> =
        std::collections::HashMap::new();
    for (i, r) in repr.into_iter().enumerate() {
        classes.entry(r).or_default().push(i);
    }
    classes.into_values().collect()
}
#[cfg(test)]
mod extra_equiv_tests {
    use super::*;
    use crate::Name;
    #[test]
    fn test_transitive_closure_basic() {
        let pairs = vec![(0, 1), (1, 2)];
        let closure = transitive_closure(&pairs, 3);
        assert!(closure.contains(&(0, 2)));
    }
    #[test]
    fn test_transitive_closure_empty() {
        let closure = transitive_closure(&[], 3);
        assert!(closure.is_empty());
    }
    #[test]
    fn test_compute_representatives_basic() {
        let pairs = vec![(0, 1)];
        let repr = compute_representatives(&pairs, 3);
        assert_eq!(repr[0], repr[1]);
        assert_ne!(repr[0], repr[2]);
    }
    #[test]
    fn test_find_classes_two_classes() {
        let pairs = vec![(0, 1), (2, 3)];
        let classes = find_classes(&pairs, 4);
        assert_eq!(classes.len(), 2);
    }
    #[test]
    fn test_index_equiv_manager_basic() {
        let mut mgr = IndexEquivManager::new(5);
        mgr.union(0, 1);
        mgr.union(1, 2);
        assert!(mgr.same_class(0, 2));
        assert!(!mgr.same_class(0, 3));
    }
    #[test]
    fn test_index_equiv_manager_num_classes() {
        let mut mgr = IndexEquivManager::new(5);
        assert_eq!(mgr.num_classes(), 5);
        mgr.union(0, 1);
        mgr.union(2, 3);
        assert_eq!(mgr.num_classes(), 3);
    }
    #[test]
    fn test_index_equiv_manager_find_self() {
        let mut mgr = IndexEquivManager::new(3);
        assert_eq!(mgr.find(0), 0);
    }
    #[test]
    fn test_equiv_manager_default() {
        let mgr = EquivManager::default();
        assert_eq!(mgr.num_equiv(), 0);
        assert_eq!(mgr.num_failures(), 0);
    }
    #[test]
    fn test_persistent_equiv_manager_clone() {
        let mut mgr = PersistentEquivManager::new();
        mgr.add_equiv(
            &Expr::Const(Name::str("A"), vec![]),
            &Expr::Const(Name::str("B"), vec![]),
        );
        let cloned = mgr.clone();
        assert_eq!(cloned.len(), 1);
    }
    #[test]
    fn test_scoped_manager_reflexive() {
        let mut mgr = ScopedEquivManager::new();
        let e = Expr::Const(Name::str("A"), vec![]);
        assert!(mgr.is_equiv(&e, &e));
    }
    #[test]
    fn test_union_find_all_separate() {
        let pairs: Vec<(usize, usize)> = vec![];
        let repr = compute_representatives(&pairs, 4);
        let classes: std::collections::HashSet<_> = repr.into_iter().collect();
        assert_eq!(classes.len(), 4);
    }
    #[test]
    fn test_find_classes_all_same() {
        let pairs = vec![(0, 1), (1, 2), (2, 3)];
        let classes = find_classes(&pairs, 4);
        assert_eq!(classes.len(), 1);
        assert_eq!(classes[0].len(), 4);
    }
}
#[cfg(test)]
mod tests_equiv_extra {
    use super::*;
    #[test]
    fn test_equiv_class() {
        let mut ec = EquivClass::singleton(0);
        ec.members.push(1);
        assert!(ec.contains(0));
        assert!(ec.contains(1));
        assert!(!ec.contains(2));
        assert_eq!(ec.size(), 2);
    }
    #[test]
    fn test_union_find() {
        let mut uf = UnionFind::new(5);
        assert_eq!(uf.num_classes(), 5);
        assert!(uf.union(0, 1));
        assert!(uf.union(2, 3));
        assert_eq!(uf.num_classes(), 3);
        assert!(uf.same(0, 1));
        assert!(!uf.same(0, 2));
        uf.union(1, 2);
        assert!(uf.same(0, 3));
    }
    #[test]
    fn test_equivalence_table() {
        let mut tbl = EquivalenceTable::new(8);
        tbl.register(100);
        tbl.register(200);
        assert!(!tbl.equiv(100, 200));
        tbl.merge(100, 200);
        assert!(tbl.equiv(100, 200));
    }
    #[test]
    fn test_expr_equiv_cache() {
        let mut cache = ExprEquivCache::new();
        assert_eq!(cache.query(1, 2), None);
        cache.mark_equal(1, 2);
        assert_eq!(cache.query(1, 2), Some(true));
        assert_eq!(cache.query(2, 1), Some(true));
        cache.mark_unequal(3, 4);
        assert_eq!(cache.query(3, 4), Some(false));
        assert_eq!(cache.proven_count(), 1);
    }
    #[test]
    fn test_equiv_manager_stats() {
        let mut stats = EquivManagerStats::new();
        stats.checks = 100;
        stats.hits = 80;
        assert!((stats.hit_rate() - 0.8).abs() < 1e-9);
    }
}
#[cfg(test)]
mod tests_equiv_proof {
    use super::*;
    #[test]
    fn test_equiv_proof_term() {
        let refl = EquivProofTerm::Refl(42);
        assert_eq!(refl.depth(), 0);
        assert!(refl.is_refl());
        let symm = EquivProofTerm::Symm(Box::new(EquivProofTerm::Axiom("h".into())));
        assert_eq!(symm.depth(), 1);
        assert!(!symm.is_refl());
    }
    #[test]
    fn test_checked_equiv() {
        let eq = CheckedEquiv::refl(99);
        assert!(eq.is_trivial());
        assert!(eq.proof().is_refl());
        let ax = CheckedEquiv::by_axiom(1, 2, "comm");
        assert!(!ax.is_trivial());
        assert_eq!(ax.lhs(), 1);
        assert_eq!(ax.rhs(), 2);
    }
}
#[cfg(test)]
mod tests_symmetric_relation {
    use super::*;
    #[test]
    fn test_symmetric_relation() {
        let mut rel = SymmetricRelation::new();
        rel.add(1, 2);
        assert!(rel.contains(1, 2));
        assert!(rel.contains(2, 1));
        assert!(!rel.contains(1, 3));
        assert_eq!(rel.len(), 1);
    }
}
#[cfg(test)]
mod tests_common_infra {
    use super::*;
    #[test]
    fn test_event_counter() {
        let mut ec = EventCounter::new();
        ec.inc("hit");
        ec.inc("hit");
        ec.inc("miss");
        assert_eq!(ec.get("hit"), 2);
        assert_eq!(ec.get("miss"), 1);
        assert_eq!(ec.total(), 3);
        ec.reset();
        assert_eq!(ec.total(), 0);
    }
    #[test]
    fn test_diag_meta() {
        let mut m = DiagMeta::new();
        m.add("os", "linux");
        m.add("arch", "x86_64");
        assert_eq!(m.get("os"), Some("linux"));
        assert_eq!(m.len(), 2);
        let s = m.to_string();
        assert!(s.contains("os=linux"));
    }
    #[test]
    fn test_scope_stack() {
        let mut ss = ScopeStack::new();
        ss.push("Nat");
        ss.push("succ");
        assert_eq!(ss.current(), Some("succ"));
        assert_eq!(ss.depth(), 2);
        assert_eq!(ss.path(), "Nat.succ");
        ss.pop();
        assert_eq!(ss.current(), Some("Nat"));
    }
    #[test]
    fn test_annotation_table() {
        let mut tbl = AnnotationTable::new();
        tbl.annotate("doc", "first line");
        tbl.annotate("doc", "second line");
        assert_eq!(tbl.get_all("doc").len(), 2);
        assert!(tbl.has("doc"));
        assert!(!tbl.has("other"));
    }
    #[test]
    fn test_work_stack() {
        let mut ws = WorkStack::new();
        ws.push(1u32);
        ws.push(2u32);
        assert_eq!(ws.pop(), Some(2));
        assert_eq!(ws.len(), 1);
    }
    #[test]
    fn test_work_queue() {
        let mut wq = WorkQueue::new();
        wq.enqueue(1u32);
        wq.enqueue(2u32);
        assert_eq!(wq.dequeue(), Some(1));
        assert_eq!(wq.len(), 1);
    }
    #[test]
    fn test_sparse_bit_set() {
        let mut bs = SparseBitSet::new(128);
        bs.set(5);
        bs.set(63);
        bs.set(64);
        assert!(bs.get(5));
        assert!(bs.get(63));
        assert!(bs.get(64));
        assert!(!bs.get(0));
        assert_eq!(bs.count_ones(), 3);
        bs.clear(5);
        assert!(!bs.get(5));
    }
    #[test]
    fn test_loop_clock() {
        let mut clk = LoopClock::start();
        for _ in 0..10 {
            clk.tick();
        }
        assert_eq!(clk.iters(), 10);
        assert!(clk.elapsed_us() >= 0.0);
    }
}
#[cfg(test)]
mod tests_extra_data_structures {
    use super::*;
    #[test]
    fn test_simple_lru_cache() {
        let mut cache: SimpleLruCache<&str, u32> = SimpleLruCache::new(3);
        cache.put("a", 1);
        cache.put("b", 2);
        cache.put("c", 3);
        assert_eq!(cache.get(&"a"), Some(&1));
        cache.put("d", 4);
        assert!(cache.len() <= 3);
    }
    #[test]
    fn test_string_interner() {
        let mut si = StringInterner::new();
        let id1 = si.intern("hello");
        let id2 = si.intern("hello");
        assert_eq!(id1, id2);
        let id3 = si.intern("world");
        assert_ne!(id1, id3);
        assert_eq!(si.get(id1), Some("hello"));
        assert_eq!(si.len(), 2);
    }
    #[test]
    fn test_frequency_table() {
        let mut ft = FrequencyTable::new();
        ft.record("a");
        ft.record("b");
        ft.record("a");
        ft.record("a");
        assert_eq!(ft.freq(&"a"), 3);
        assert_eq!(ft.freq(&"b"), 1);
        assert_eq!(ft.most_frequent(), Some((&"a", 3)));
        assert_eq!(ft.total(), 4);
        assert_eq!(ft.distinct(), 2);
    }
    #[test]
    fn test_bimap() {
        let mut bm: BiMap<u32, &str> = BiMap::new();
        bm.insert(1, "one");
        bm.insert(2, "two");
        assert_eq!(bm.get_b(&1), Some(&"one"));
        assert_eq!(bm.get_a(&"two"), Some(&2));
        assert_eq!(bm.len(), 2);
    }
}
#[cfg(test)]
mod tests_interval_set {
    use super::*;
    #[test]
    fn test_interval_set() {
        let mut s = IntervalSet::new();
        s.add(1, 5);
        s.add(3, 8);
        assert_eq!(s.num_intervals(), 1);
        assert_eq!(s.cardinality(), 8);
        assert!(s.contains(4));
        assert!(!s.contains(9));
        s.add(10, 15);
        assert_eq!(s.num_intervals(), 2);
    }
}
/// Returns the current timestamp.
#[allow(dead_code)]
pub fn now_us() -> Timestamp {
    let us = std::time::SystemTime::UNIX_EPOCH
        .elapsed()
        .map(|d| d.as_micros() as u64)
        .unwrap_or(0);
    Timestamp::from_us(us)
}
#[cfg(test)]
mod tests_typed_utilities {
    use super::*;
    #[test]
    fn test_timestamp() {
        let t1 = Timestamp::from_us(1000);
        let t2 = Timestamp::from_us(1500);
        assert_eq!(t2.elapsed_since(t1), 500);
        assert!(t1 < t2);
    }
    #[test]
    fn test_typed_id() {
        struct Foo;
        let id: TypedId<Foo> = TypedId::new(42);
        assert_eq!(id.raw(), 42);
        assert_eq!(format!("{id}"), "#42");
    }
    #[test]
    fn test_id_dispenser() {
        struct Bar;
        let mut disp: IdDispenser<Bar> = IdDispenser::new();
        let a = disp.next();
        let b = disp.next();
        assert_eq!(a.raw(), 0);
        assert_eq!(b.raw(), 1);
        assert_eq!(disp.count(), 2);
    }
    #[test]
    fn test_slot() {
        let mut slot: Slot<u32> = Slot::empty();
        assert!(!slot.is_filled());
        slot.fill(99);
        assert!(slot.is_filled());
        assert_eq!(slot.get(), Some(&99));
        let v = slot.take();
        assert_eq!(v, Some(99));
        assert!(!slot.is_filled());
    }
    #[test]
    #[should_panic]
    fn test_slot_double_fill() {
        let mut slot: Slot<u32> = Slot::empty();
        slot.fill(1);
        slot.fill(2);
    }
    #[test]
    fn test_memo_slot() {
        let mut ms: MemoSlot<u32> = MemoSlot::new();
        assert!(!ms.is_cached());
        let val = ms.get_or_compute(|| 42);
        assert_eq!(*val, 42);
        assert!(ms.is_cached());
        ms.invalidate();
        assert!(!ms.is_cached());
    }
}
#[cfg(test)]
mod tests_ring_buffer {
    use super::*;
    #[test]
    fn test_ring_buffer() {
        let mut rb = RingBuffer::new(3);
        rb.push(1u32);
        rb.push(2u32);
        rb.push(3u32);
        assert!(rb.is_full());
        rb.push(4u32);
        assert_eq!(rb.pop(), Some(2));
        assert_eq!(rb.len(), 2);
    }
    #[test]
    fn test_before_after() {
        let ba = BeforeAfter::new(10u32, 10u32);
        assert!(ba.unchanged());
        let ba2 = BeforeAfter::new(10u32, 20u32);
        assert!(!ba2.unchanged());
    }
    #[test]
    fn test_seq_num() {
        let s = SeqNum::ZERO;
        assert_eq!(s.value(), 0);
        let s2 = s.next();
        assert_eq!(s2.value(), 1);
        assert!(s < s2);
    }
    #[test]
    fn test_generation() {
        let g0 = Generation::INITIAL;
        let g1 = g0.advance();
        assert_eq!(g0.number(), 0);
        assert_eq!(g1.number(), 1);
        assert_ne!(g0, g1);
    }
}
