//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{
    BitSet64, Counter, Diagnostic, DiagnosticBag, DiagnosticLevel, DirectedGraph, FreshNameGen,
    Located, MinHeap, MultiMap, NameTable, ScopeTable, Span, StringSet, Trie,
};

#[cfg(test)]
mod lib_extended_tests {
    use super::*;
    #[test]
    fn test_span_merge() {
        let a = Span::new(0, 5, 1, 1);
        let b = Span::new(3, 10, 1, 4);
        let m = a.merge(&b);
        assert_eq!(m.start, 0);
        assert_eq!(m.end, 10);
    }
    #[test]
    fn test_located_map() {
        let l = Located::dummy(42u32);
        let l2 = l.map(|x| x * 2);
        assert_eq!(l2.value, 84);
    }
    #[test]
    fn test_name_table() {
        let mut t = NameTable::new();
        let id_a = t.intern("alpha");
        let id_b = t.intern("beta");
        let id_a2 = t.intern("alpha");
        assert_eq!(id_a, id_a2);
        assert_ne!(id_a, id_b);
        assert_eq!(t.lookup(id_a), Some("alpha"));
        assert_eq!(t.len(), 2);
    }
    #[test]
    fn test_diagnostic_bag() {
        let mut bag = DiagnosticBag::new();
        assert!(!bag.has_errors());
        bag.push(Diagnostic::warning("minor issue"));
        assert!(!bag.has_errors());
        bag.push(Diagnostic::error("fatal problem"));
        assert!(bag.has_errors());
        assert_eq!(bag.len(), 2);
        let drained = bag.drain();
        assert_eq!(drained.len(), 2);
        assert!(bag.is_empty());
    }
    #[test]
    fn test_scope_table() {
        let mut s: ScopeTable<&str, u32> = ScopeTable::new();
        s.define("x", 1);
        s.push_scope();
        s.define("x", 2);
        assert_eq!(s.lookup(&"x"), Some(&2));
        s.pop_scope();
        assert_eq!(s.lookup(&"x"), Some(&1));
    }
    #[test]
    fn test_counter_and_fresh_name() {
        let mut c = Counter::new();
        assert_eq!(c.next(), 0);
        assert_eq!(c.next(), 1);
        assert_eq!(c.peek(), 2);
        c.reset();
        assert_eq!(c.next(), 0);
        let mut gen = FreshNameGen::new("var");
        let n0 = gen.fresh();
        let n1 = gen.fresh();
        assert_eq!(n0, "var_0");
        assert_eq!(n1, "var_1");
    }
    #[test]
    fn test_string_set_operations() {
        let mut s = StringSet::new();
        assert!(s.insert("banana"));
        assert!(s.insert("apple"));
        assert!(!s.insert("apple"));
        assert!(s.contains("apple"));
        assert!(!s.contains("cherry"));
        assert_eq!(s.len(), 2);
        assert!(s.remove("apple"));
        assert!(!s.contains("apple"));
        let mut t = StringSet::new();
        t.insert("cherry");
        t.insert("banana");
        let u = s.union(&t);
        assert!(u.contains("banana"));
        assert!(u.contains("cherry"));
    }
    #[test]
    fn test_multi_map() {
        let mut m: MultiMap<&str, u32> = MultiMap::new();
        m.insert("key", 1);
        m.insert("key", 2);
        m.insert("other", 3);
        assert_eq!(m.get(&"key"), &[1, 2]);
        assert_eq!(m.key_count(), 2);
        let removed = m.remove(&"key");
        assert_eq!(removed, vec![1, 2]);
        assert!(!m.contains_key(&"key"));
    }
    #[test]
    fn test_trie() {
        let mut t: Trie<u32> = Trie::new();
        t.insert(b"hello", 1);
        t.insert(b"help", 2);
        t.insert(b"world", 3);
        assert_eq!(t.get(b"hello"), Some(&1));
        assert_eq!(t.get(b"help"), Some(&2));
        assert!(t.get(b"helo").is_none());
        assert!(t.contains(b"world"));
        let pfx = t.keys_with_prefix(b"hel");
        assert_eq!(pfx.len(), 2);
    }
    #[test]
    fn test_bitset64() {
        let mut bs = BitSet64::empty();
        assert!(bs.is_empty());
        bs.set(5);
        bs.set(10);
        assert!(bs.test(5));
        assert!(bs.test(10));
        assert!(!bs.test(0));
        assert_eq!(bs.count(), 2);
        bs.clear(5);
        assert!(!bs.test(5));
        let ones: Vec<u8> = bs.iter_ones().collect();
        assert_eq!(ones, vec![10]);
    }
    #[test]
    fn test_min_heap() {
        let mut heap: MinHeap<u32, &str> = MinHeap::new();
        heap.push(5, "five");
        heap.push(1, "one");
        heap.push(3, "three");
        assert_eq!(heap.len(), 3);
        let (p, v) = heap.pop().expect("pop should succeed");
        assert_eq!(p, 1);
        assert_eq!(v, "one");
        let (p2, _) = heap.pop().expect("pop should succeed");
        assert_eq!(p2, 3);
    }
    #[test]
    fn test_directed_graph_topo_sort() {
        let mut g = DirectedGraph::new(4);
        g.add_edge(0, 1);
        g.add_edge(0, 2);
        g.add_edge(1, 3);
        g.add_edge(2, 3);
        let order = g.topological_sort().expect("should be a DAG");
        assert_eq!(order.len(), 4);
        let pos: Vec<usize> = {
            let mut p = vec![0usize; 4];
            for (i, &node) in order.iter().enumerate() {
                p[node] = i;
            }
            p
        };
        assert!(pos[0] < pos[1]);
        assert!(pos[0] < pos[2]);
        assert!(pos[1] < pos[3]);
        assert!(pos[2] < pos[3]);
    }
    #[test]
    fn test_directed_graph_scc() {
        let mut g = DirectedGraph::new(4);
        g.add_edge(0, 1);
        g.add_edge(1, 2);
        g.add_edge(2, 0);
        let sccs = g.strongly_connected_components();
        assert_eq!(sccs.len(), 2);
    }
    #[test]
    fn test_diagnostic_level_ordering() {
        assert!(DiagnosticLevel::Note < DiagnosticLevel::Warning);
        assert!(DiagnosticLevel::Warning < DiagnosticLevel::Error);
        assert!(DiagnosticLevel::Error < DiagnosticLevel::Bug);
        assert!(DiagnosticLevel::Error.is_fatal());
        assert!(!DiagnosticLevel::Warning.is_fatal());
    }
}
