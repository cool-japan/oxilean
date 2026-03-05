//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{
    AnnotationTable, BeforeAfter, BiMap, DiagMeta, EventCounter, FrequencyTable, Generation,
    IdDispenser, IntervalSet, LoopClock, MemoSlot, Name, NameGenerator, NameGeneratorExt, NameMap,
    NameMapping, NamePool, NameResolver, NameSet, NameSetExt, NameTrie, NameTrieExt, QualifiedName,
    QualifiedNameExt, RingBuffer, ScopeStack, SeqNum, SimpleLruCache, Slot, SparseBitSet,
    StringInterner, Timestamp, TypedId, WorkQueue, WorkStack,
};

/// Return all direct children of `ns` in a `NameSet`.
///
/// A name `n` is a direct child of `ns` if `n = ns.something` with no further dots.
pub fn direct_children<'a>(ns: &Name, names: &'a NameSet) -> Vec<&'a Name> {
    let ns_str = ns.to_string();
    names
        .iter()
        .filter(|n| {
            let s = n.to_string();
            if ns.is_anonymous() {
                !s.contains('.') && s != "_"
            } else {
                s.starts_with(&format!("{ns_str}.")) && !s[ns_str.len() + 1..].contains('.')
            }
        })
        .collect()
}
/// Compute the longest common prefix of two names.
pub fn longest_common_prefix(a: &Name, b: &Name) -> Name {
    let comps_a = a.components();
    let comps_b = b.components();
    let prefix: Vec<String> = comps_a
        .iter()
        .zip(comps_b.iter())
        .take_while(|(x, y)| x == y)
        .map(|(x, _)| x.clone())
        .collect();
    Name::from_components(&prefix)
}
/// Check whether two names share a common namespace prefix.
pub fn share_namespace(a: &Name, b: &Name) -> bool {
    let lcp = longest_common_prefix(a, b);
    !lcp.is_anonymous()
}
/// Macro for creating simple string names.
///
/// # Example
/// ```ignore
/// let n = name!("Nat");
/// let m = name!("Nat", "add");
/// ```
#[macro_export]
macro_rules! name {
    ($s:expr) => {
        $crate::Name::str($s)
    };
    ($first:expr, $($rest:expr),+) => {
        { let mut n = $crate::Name::str($first); $(n = n.append_str($rest);)+ n }
    };
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_name_display() {
        let n1 = Name::Anonymous;
        let n2 = Name::str("Nat");
        let n3 = Name::str("Nat").append_str("add");
        let n4 = Name::str("x").append_num(42);
        assert_eq!(n1.to_string(), "_");
        assert_eq!(n2.to_string(), "Nat");
        assert_eq!(n3.to_string(), "Nat.add");
        assert_eq!(n4.to_string(), "x.42");
    }
    #[test]
    fn test_name_macro() {
        let n1 = name!("Nat");
        let n2 = name!("Nat", "add", "comm");
        assert_eq!(n1.to_string(), "Nat");
        assert_eq!(n2.to_string(), "Nat.add.comm");
    }
    #[test]
    fn test_name_from_str() {
        let n = Name::from_str("Nat.add.comm");
        assert_eq!(n.to_string(), "Nat.add.comm");
        let n2 = Name::from_str("foo");
        assert_eq!(n2.to_string(), "foo");
    }
    #[test]
    fn test_name_depth() {
        assert_eq!(Name::Anonymous.depth(), 0);
        assert_eq!(Name::str("Nat").depth(), 1);
        assert_eq!(Name::str("Nat").append_str("add").depth(), 2);
    }
    #[test]
    fn test_name_last_str() {
        let n = Name::str("Nat").append_str("add");
        assert_eq!(n.last_str(), Some("add"));
        assert_eq!(Name::Anonymous.last_str(), None);
    }
    #[test]
    fn test_name_root() {
        let n = Name::str("Nat").append_str("add").append_str("comm");
        assert_eq!(n.root(), Some("Nat"));
    }
    #[test]
    fn test_name_has_prefix() {
        let ns = Name::str("Nat");
        let n = Name::str("Nat").append_str("add");
        assert!(n.has_prefix(&ns));
        assert!(!ns.has_prefix(&ns));
    }
    #[test]
    fn test_name_components() {
        let n = Name::str("Nat").append_str("add").append_str("comm");
        assert_eq!(n.components(), vec!["Nat", "add", "comm"]);
    }
    #[test]
    fn test_name_from_components() {
        let comps = vec!["Nat".to_string(), "add".to_string()];
        let n = Name::from_components(&comps);
        assert_eq!(n.to_string(), "Nat.add");
    }
    #[test]
    fn test_name_replace_last() {
        let n = Name::str("Nat").append_str("add");
        let n2 = n.replace_last("sub");
        assert_eq!(n2.to_string(), "Nat.sub");
    }
    #[test]
    fn test_name_freshen() {
        let n = Name::str("x");
        let n2 = n.freshen(3);
        assert_eq!(n2.to_string(), "x.3");
    }
    #[test]
    fn test_name_set() {
        let mut s = NameSet::new();
        s.insert(Name::str("Nat"));
        s.insert(Name::str("Int"));
        assert_eq!(s.len(), 2);
        assert!(s.contains(&Name::str("Nat")));
        assert!(!s.contains(&Name::str("Bool")));
    }
    #[test]
    fn test_name_set_union() {
        let mut a = NameSet::new();
        a.insert(Name::str("Nat"));
        let mut b = NameSet::new();
        b.insert(Name::str("Int"));
        let u = a.union(&b);
        assert_eq!(u.len(), 2);
    }
    #[test]
    fn test_name_map() {
        let mut m: NameMap<u32> = NameMap::new();
        m.insert(Name::str("a"), 1);
        m.insert(Name::str("b"), 2);
        assert_eq!(m.get(&Name::str("a")), Some(&1));
        assert_eq!(m.len(), 2);
    }
    #[test]
    fn test_name_map_remove() {
        let mut m: NameMap<u32> = NameMap::new();
        m.insert(Name::str("x"), 99);
        let v = m.remove(&Name::str("x"));
        assert_eq!(v, Some(99));
        assert!(m.is_empty());
    }
    #[test]
    fn test_name_generator() {
        let mut gen = NameGenerator::with_base("_fresh");
        let n1 = gen.next();
        let n2 = gen.next();
        assert_eq!(n1.to_string(), "_fresh.0");
        assert_eq!(n2.to_string(), "_fresh.1");
        assert_eq!(gen.count(), 2);
    }
    #[test]
    fn test_name_ord() {
        let mut names = [Name::str("Nat"), Name::str("Bool"), Name::str("Int")];
        names.sort();
        assert_eq!(names[0].to_string(), "Bool");
    }
    #[test]
    fn test_longest_common_prefix() {
        let a = Name::str("Nat").append_str("add").append_str("comm");
        let b = Name::str("Nat").append_str("add").append_str("assoc");
        let lcp = longest_common_prefix(&a, &b);
        assert_eq!(lcp.to_string(), "Nat.add");
    }
    #[test]
    fn test_share_namespace() {
        let a = Name::str("Nat").append_str("add");
        let b = Name::str("Nat").append_str("sub");
        assert!(share_namespace(&a, &b));
        let c = Name::str("Int").append_str("add");
        assert!(!share_namespace(&a, &c));
    }
}
#[cfg(test)]
mod trie_tests {
    use super::*;
    #[test]
    fn test_name_trie_insert_lookup() {
        let mut trie: NameTrie<u32> = NameTrie::new();
        let n = Name::str("Nat").append_str("add");
        trie.insert(&n, 42);
        assert_eq!(trie.lookup(&n), Some(&42));
        assert_eq!(trie.lookup(&Name::str("Nat")), None);
    }
    #[test]
    fn test_name_trie_count() {
        let mut trie: NameTrie<u32> = NameTrie::new();
        trie.insert(&Name::str("a"), 1);
        trie.insert(&Name::str("b"), 2);
        trie.insert(&Name::str("a").append_str("x"), 3);
        assert_eq!(trie.count(), 3);
    }
    #[test]
    fn test_name_trie_to_vec() {
        let mut trie: NameTrie<u32> = NameTrie::new();
        trie.insert(&Name::str("a"), 1);
        trie.insert(&Name::str("b"), 2);
        let v = trie.to_vec();
        assert_eq!(v.len(), 2);
    }
    #[test]
    fn test_qualified_name() {
        let canonical = Name::str("Nat").append_str("add");
        let alias = Name::str("add");
        let qn = QualifiedName::with_alias(canonical.clone(), alias.clone());
        assert_eq!(qn.preferred(), &alias);
        let qn2 = QualifiedName::new(canonical.clone());
        assert_eq!(qn2.preferred(), &canonical);
    }
    #[test]
    fn test_name_set_in_namespace() {
        let mut ns_set = NameSet::new();
        ns_set.insert(Name::str("Nat").append_str("add"));
        ns_set.insert(Name::str("Nat").append_str("sub"));
        ns_set.insert(Name::str("Int").append_str("add"));
        let nat_names = ns_set.in_namespace(&Name::str("Nat"));
        assert_eq!(nat_names.len(), 2);
    }
    #[test]
    fn test_name_set_to_sorted_vec() {
        let mut s = NameSet::new();
        s.insert(Name::str("z_name"));
        s.insert(Name::str("a_name"));
        s.insert(Name::str("m_name"));
        let sorted = s.to_sorted_vec();
        assert_eq!(sorted[0].to_string(), "a_name");
        assert_eq!(sorted[2].to_string(), "z_name");
    }
    #[test]
    fn test_name_map_filter_by_namespace() {
        let mut m: NameMap<u32> = NameMap::new();
        m.insert(Name::str("Nat").append_str("add"), 1);
        m.insert(Name::str("Nat").append_str("sub"), 2);
        m.insert(Name::str("Int").append_str("add"), 3);
        let nat_entries = m.filter_by_namespace(&Name::str("Nat"));
        assert_eq!(nat_entries.len(), 2);
    }
    #[test]
    fn test_name_generator_reset() {
        let mut gen = NameGenerator::with_base("x");
        gen.next();
        gen.next();
        assert_eq!(gen.count(), 2);
        gen.reset();
        assert_eq!(gen.count(), 0);
        let n = gen.next();
        assert_eq!(n.to_string(), "x.0");
    }
    #[test]
    fn test_direct_children() {
        let mut names = NameSet::new();
        names.insert(Name::str("Nat"));
        names.insert(Name::str("Nat").append_str("add"));
        names.insert(Name::str("Nat").append_str("add").append_str("comm"));
        let children = direct_children(&Name::str("Nat"), &names);
        assert_eq!(children.len(), 1);
        assert_eq!(children[0].to_string(), "Nat.add");
    }
    #[test]
    fn test_name_prepend() {
        let n = Name::str("add");
        let ns = Name::str("Nat");
        let full = n.prepend(ns);
        assert_eq!(full.to_string(), "Nat.add");
    }
    #[test]
    fn test_name_to_ident_string() {
        let n = Name::str("Nat").append_str("add");
        let s = n.to_ident_string();
        assert!(!s.contains('.'));
    }
}
#[cfg(test)]
mod tests_name_extra {
    use super::*;
    #[test]
    fn test_name_pool() {
        let mut pool = NamePool::new();
        let id1 = pool.intern("Nat");
        let id2 = pool.intern("Nat");
        assert_eq!(id1, id2);
        let id3 = pool.intern("Bool");
        assert_ne!(id1, id3);
        assert_eq!(pool.get(id1), Some("Nat"));
        assert_eq!(pool.len(), 2);
    }
    #[test]
    fn test_qualified_name() {
        let qn = QualifiedNameExt::from_dot_str("Nat.succ");
        assert_eq!(qn.unqualified(), "succ");
        assert_eq!(qn.depth(), 2);
        let ns = qn.namespace().expect("ns should be present");
        assert_eq!(ns.to_string(), "Nat");
        let child = QualifiedNameExt::from_dot_str("Nat.succ.aux");
        let parent = QualifiedNameExt::from_dot_str("Nat.succ");
        assert!(child.is_sub_of(&parent));
    }
    #[test]
    fn test_name_mapping() {
        let mut nm = NameMapping::new();
        let id = nm.register("Foo");
        assert_eq!(nm.id_of("Foo"), Some(id));
        assert_eq!(nm.name_of(id), Some("Foo"));
        assert_eq!(nm.register("Foo"), id);
        assert_eq!(nm.len(), 1);
    }
    #[test]
    fn test_name_trie() {
        let mut trie = NameTrieExt::new();
        trie.insert("Nat.succ");
        trie.insert("Nat.zero");
        trie.insert("Bool.true");
        assert!(trie.contains("Nat.succ"));
        assert!(!trie.contains("Nat.pred"));
        let nat_names = trie.with_prefix("Nat");
        assert_eq!(nat_names.len(), 2);
    }
    #[test]
    fn test_name_resolver() {
        let mut resolver = NameResolver::new();
        resolver.register("Nat.succ");
        resolver.enter("Nat");
        let resolved = resolver.resolve("succ");
        assert_eq!(resolved, "Nat.succ");
        resolver.exit();
        assert_eq!(resolver.current_ns(), "");
    }
}
#[cfg(test)]
mod tests_name_extra2 {
    use super::*;
    #[test]
    fn test_name_generator() {
        let mut gen = NameGeneratorExt::new("_x");
        assert_eq!(gen.next(), "_x0");
        assert_eq!(gen.next(), "_x1");
        assert_eq!(gen.count(), 2);
        gen.reset();
        assert_eq!(gen.next(), "_x0");
    }
    #[test]
    fn test_name_set() {
        let mut s = NameSetExt::new();
        s.insert("Foo");
        s.insert("Bar");
        assert!(s.contains("Foo"));
        s.remove("Foo");
        assert!(!s.contains("Foo"));
        let mut t = NameSetExt::new();
        t.insert("Bar");
        t.insert("Baz");
        let u = s.union(&t);
        assert!(u.contains("Bar"));
        assert!(u.contains("Baz"));
        let inter = s.intersect(&t);
        assert!(inter.contains("Bar"));
        assert!(!inter.contains("Baz"));
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
