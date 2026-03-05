//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::{Expr, Name};
use std::collections::{HashMap, HashSet};

use super::types::{
    ConfigNode, DecisionNode, DetailedTerminationResult, Either2, Fixture, FlatSubstitution,
    FocusStack, LabelSet, MinHeap, NameIndex, NonEmptyVec, ParamInfo, PathBuf, PrefixCounter,
    RecCallInfo, RewriteRule, RewriteRuleSet, SimpleDag, SlidingSum, SmallMap, SparseVec,
    StackCalc, StatSummary, Stopwatch, StringPool, StringTrie, TerminationCache,
    TerminationChecker, TokenBucket, TransformStat, TransitiveClosure, VersionedRecord,
    WfRelationKind, WindowIterator, WriteOnce,
};

/// Collect function head and arguments from a nested application.
pub(super) fn collect_app_args(expr: &Expr) -> (&Expr, Vec<&Expr>) {
    let mut args = Vec::new();
    let mut e = expr;
    while let Expr::App(f, a) = e {
        args.push(a.as_ref());
        e = f;
    }
    args.reverse();
    (e, args)
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::Level;
    #[test]
    fn test_non_recursive() {
        let mut checker = TerminationChecker::new();
        let body = Expr::Lit(crate::Literal::Nat(42));
        assert!(checker.check_terminates(&Name::str("f"), &body).is_ok());
    }
    #[test]
    fn test_simple_recursive() {
        let mut checker = TerminationChecker::new();
        let f = Name::str("fact");
        let n_var = Expr::BVar(0);
        checker.add_smaller(n_var.clone(), Expr::BVar(1));
        let body = Expr::App(
            Box::new(Expr::Const(f.clone(), vec![])),
            Box::new(Expr::BVar(1)),
        );
        assert!(checker.check_terminates(&f, &body).is_ok());
    }
    #[test]
    fn test_lambda_body() {
        let mut checker = TerminationChecker::new();
        let f = Name::str("f");
        let body = Expr::Lam(
            crate::BinderInfo::Default,
            Name::str("x"),
            Box::new(Expr::Sort(Level::zero())),
            Box::new(Expr::BVar(0)),
        );
        assert!(checker.check_terminates(&f, &body).is_ok());
    }
    #[test]
    fn test_depth_limit() {
        let mut checker = TerminationChecker::new();
        let f = Name::str("f");
        let mut body = Expr::BVar(0);
        for _ in 0..250 {
            body = Expr::App(
                Box::new(Expr::Const(Name::str("succ"), vec![])),
                Box::new(body),
            );
        }
        let result = checker.check_terminates(&f, &body);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("too deeply nested"));
    }
    #[test]
    fn test_smaller_transitivity() {
        let mut checker = TerminationChecker::new();
        let a = Expr::Lit(crate::Literal::Nat(3));
        let b = Expr::Lit(crate::Literal::Nat(2));
        let c = Expr::Lit(crate::Literal::Nat(1));
        checker.add_smaller(a.clone(), b.clone());
        checker.add_smaller(b, c.clone());
        assert!(checker.is_smaller(&a, &c));
    }
    #[test]
    fn test_collect_app_args() {
        let f = Expr::Const(Name::str("f"), vec![]);
        let a = Expr::Lit(crate::Literal::Nat(1));
        let b = Expr::Lit(crate::Literal::Nat(2));
        let app = Expr::App(
            Box::new(Expr::App(Box::new(f.clone()), Box::new(a.clone()))),
            Box::new(b.clone()),
        );
        let (head, args) = collect_app_args(&app);
        assert_eq!(head, &f);
        assert_eq!(args.len(), 2);
        assert_eq!(args[0], &a);
        assert_eq!(args[1], &b);
    }
    #[test]
    fn test_register_params() {
        let mut checker = TerminationChecker::new();
        checker.register_params(
            Name::str("f"),
            vec![ParamInfo {
                name: Name::str("n"),
                pos: 0,
                inductive_type: Some(Name::str("Nat")),
            }],
        );
    }
    #[test]
    fn test_no_recursive_calls() {
        let mut checker = TerminationChecker::new();
        let f = Name::str("const_fn");
        let body = Expr::Lit(crate::Literal::Nat(0));
        assert!(checker.check_terminates(&f, &body).is_ok());
        assert!(checker
            .get_calls(&f)
            .expect("value should be present")
            .is_empty());
    }
}
/// Guard against infinite descent.
#[allow(dead_code)]
pub fn has_obvious_nontermination(calls: &[RecCallInfo]) -> bool {
    calls.iter().any(|c| !c.is_decreasing)
}
/// Build a termination certificate for a structurally recursive function.
#[allow(dead_code)]
pub fn try_structural_certificate(
    _function_name: &Name,
    calls: &[RecCallInfo],
    param_infos: &[ParamInfo],
) -> Option<WfRelationKind> {
    if calls.is_empty() {
        return None;
    }
    for param in param_infos {
        let all_decrease_at_param = calls
            .iter()
            .all(|c| c.arg_pos == param.pos && c.is_decreasing);
        if all_decrease_at_param {
            if let Some(ind_ty) = &param.inductive_type {
                return Some(WfRelationKind::Structural {
                    inductive_type: ind_ty.clone(),
                    param_index: param.pos,
                });
            }
        }
    }
    if calls.iter().all(|c| c.is_decreasing) {
        return Some(WfRelationKind::NatSub);
    }
    None
}
#[cfg(test)]
mod extended_termination_tests {
    use super::*;
    #[test]
    fn test_wf_relation_structural() {
        let wf = WfRelationKind::Structural {
            inductive_type: Name::str("Nat"),
            param_index: 0,
        };
        assert!(wf.is_structural());
        assert_eq!(wf.lex_depth(), 1);
    }
    #[test]
    fn test_wf_relation_measure() {
        let wf = WfRelationKind::Measure {
            measure_fn: Name::str("size"),
        };
        assert!(!wf.is_structural());
        assert!(wf.description().contains("size"));
    }
    #[test]
    fn test_wf_relation_lex_depth() {
        let wf = WfRelationKind::Lexicographic {
            components: vec![WfRelationKind::NatSub, WfRelationKind::NatSub],
        };
        assert_eq!(wf.lex_depth(), 2);
    }
    #[test]
    fn test_wf_relation_nat_sub() {
        let wf = WfRelationKind::NatSub;
        assert_eq!(wf.description(), "nat subtraction");
    }
    #[test]
    fn test_detailed_result_non_recursive() {
        let r = DetailedTerminationResult::non_recursive(Name::str("f"));
        assert!(r.terminates);
        assert!(r.wf_relation.is_none());
        assert!(r.recursive_calls.is_empty());
    }
    #[test]
    fn test_detailed_result_success() {
        let wf = WfRelationKind::NatSub;
        let r = DetailedTerminationResult::success(Name::str("f"), wf, vec![]);
        assert!(r.terminates);
        assert!(r.wf_relation.is_some());
    }
    #[test]
    fn test_detailed_result_failure() {
        let r = DetailedTerminationResult::failure(Name::str("f"), vec![], "not structural");
        assert!(!r.terminates);
        assert_eq!(r.explanation, "not structural");
    }
    #[test]
    fn test_obvious_nontermination() {
        let calls = vec![RecCallInfo {
            callee: Name::str("f"),
            arg_pos: 0,
            arg: Expr::BVar(0),
            is_decreasing: false,
        }];
        assert!(has_obvious_nontermination(&calls));
        let good = vec![RecCallInfo {
            callee: Name::str("f"),
            arg_pos: 0,
            arg: Expr::BVar(0),
            is_decreasing: true,
        }];
        assert!(!has_obvious_nontermination(&good));
    }
    #[test]
    fn test_termination_cache() {
        let mut cache = TerminationCache::new();
        assert!(cache.is_empty());
        cache.mark_terminating(Name::str("fact"));
        cache.mark_nonterminating(Name::str("loop"));
        assert_eq!(cache.is_known_terminating(&Name::str("fact")), Some(true));
        assert_eq!(cache.is_known_terminating(&Name::str("loop")), Some(false));
        assert_eq!(cache.is_known_terminating(&Name::str("unknown")), None);
        assert_eq!(cache.len(), 2);
    }
    #[test]
    fn test_termination_cache_clear() {
        let mut cache = TerminationCache::new();
        cache.mark_terminating(Name::str("f"));
        cache.clear();
        assert!(cache.is_empty());
    }
    #[test]
    fn test_try_structural_certificate_empty() {
        let result = try_structural_certificate(&Name::str("f"), &[], &[]);
        assert!(result.is_none());
    }
    #[test]
    fn test_try_structural_certificate_success() {
        let calls = vec![RecCallInfo {
            callee: Name::str("f"),
            arg_pos: 0,
            arg: Expr::BVar(0),
            is_decreasing: true,
        }];
        let params = vec![ParamInfo {
            name: Name::str("n"),
            pos: 0,
            inductive_type: Some(Name::str("Nat")),
        }];
        let result = try_structural_certificate(&Name::str("f"), &calls, &params);
        assert!(result.is_some());
        assert!(result.expect("result should be valid").is_structural());
    }
    #[test]
    fn test_wf_custom_description() {
        let wf = WfRelationKind::Custom {
            relation: Name::str("myRel"),
        };
        assert!(wf.description().contains("myRel"));
    }
}
/// Version tag for this module.
#[allow(dead_code)]
pub const MODULE_VERSION: &str = "1.0.0";
/// Marker trait for types that can be used in module-specific contexts.
///
/// This is a doc-only trait providing context about the module's design philosophy.
#[allow(dead_code)]
pub trait ModuleMarker: Sized + Clone + std::fmt::Debug {}
/// Generic result type for operations in this module.
#[allow(dead_code)]
pub type ModuleResult<T> = Result<T, String>;
/// Create a module-level error string.
#[allow(dead_code)]
pub fn module_err(msg: impl Into<String>) -> String {
    format!("[{module}] {msg}", module = "termination", msg = msg.into())
}
/// Compute the Levenshtein distance between two string slices.
///
/// This is used for providing "did you mean?" suggestions in error messages.
#[allow(dead_code)]
pub fn levenshtein_distance(a: &str, b: &str) -> usize {
    let la = a.len();
    let lb = b.len();
    if la == 0 {
        return lb;
    }
    if lb == 0 {
        return la;
    }
    let mut row: Vec<usize> = (0..=lb).collect();
    for (i, ca) in a.chars().enumerate() {
        let mut prev = i;
        row[0] = i + 1;
        for (j, cb) in b.chars().enumerate() {
            let old = row[j + 1];
            row[j + 1] = if ca == cb {
                prev
            } else {
                1 + old.min(row[j]).min(prev)
            };
            prev = old;
        }
    }
    row[lb]
}
/// Find the closest match in a list of candidates using Levenshtein distance.
///
/// Returns None if candidates is empty.
#[allow(dead_code)]
pub fn closest_match<'a>(query: &str, candidates: &[&'a str]) -> Option<&'a str> {
    candidates
        .iter()
        .min_by_key(|&&c| levenshtein_distance(query, c))
        .copied()
}
/// Format a list of names for display in an error message.
#[allow(dead_code)]
pub fn format_name_list(names: &[&str]) -> String {
    match names.len() {
        0 => "(none)".to_string(),
        1 => names[0].to_string(),
        2 => format!("{} and {}", names[0], names[1]),
        _ => {
            let mut s = names[..names.len() - 1].join(", ");
            s.push_str(", and ");
            s.push_str(names[names.len() - 1]);
            s
        }
    }
}
/// Collect all strings from a trie node.
pub(super) fn collect_strings(node: &StringTrie, results: &mut Vec<String>) {
    if let Some(v) = &node.value {
        results.push(v.clone());
    }
    for child in node.children.values() {
        collect_strings(child, results);
    }
}
#[cfg(test)]
mod utility_tests {
    use super::*;
    #[test]
    fn test_levenshtein_same_string() {
        assert_eq!(levenshtein_distance("hello", "hello"), 0);
    }
    #[test]
    fn test_levenshtein_empty() {
        assert_eq!(levenshtein_distance("", "abc"), 3);
        assert_eq!(levenshtein_distance("abc", ""), 3);
    }
    #[test]
    fn test_levenshtein_one_edit() {
        assert_eq!(levenshtein_distance("cat", "bat"), 1);
        assert_eq!(levenshtein_distance("cat", "cats"), 1);
        assert_eq!(levenshtein_distance("cats", "cat"), 1);
    }
    #[test]
    fn test_closest_match_found() {
        let candidates = &["intro", "intros", "exact", "apply"];
        let result = closest_match("intoo", candidates);
        assert!(result.is_some());
        assert_eq!(result.expect("result should be valid"), "intro");
    }
    #[test]
    fn test_closest_match_empty() {
        let result = closest_match("x", &[]);
        assert!(result.is_none());
    }
    #[test]
    fn test_format_name_list_empty() {
        assert_eq!(format_name_list(&[]), "(none)");
    }
    #[test]
    fn test_format_name_list_one() {
        assert_eq!(format_name_list(&["foo"]), "foo");
    }
    #[test]
    fn test_format_name_list_two() {
        assert_eq!(format_name_list(&["a", "b"]), "a and b");
    }
    #[test]
    fn test_format_name_list_many() {
        let result = format_name_list(&["a", "b", "c"]);
        assert!(result.contains("a"));
        assert!(result.contains("b"));
        assert!(result.contains("c"));
        assert!(result.contains("and"));
    }
    #[test]
    fn test_string_trie_insert_contains() {
        let mut trie = StringTrie::new();
        trie.insert("hello");
        trie.insert("help");
        trie.insert("world");
        assert!(trie.contains("hello"));
        assert!(trie.contains("help"));
        assert!(trie.contains("world"));
        assert!(!trie.contains("hell"));
        assert_eq!(trie.len(), 3);
    }
    #[test]
    fn test_string_trie_starts_with() {
        let mut trie = StringTrie::new();
        trie.insert("hello");
        trie.insert("help");
        trie.insert("world");
        let results = trie.starts_with("hel");
        assert_eq!(results.len(), 2);
    }
    #[test]
    fn test_string_trie_empty_prefix() {
        let mut trie = StringTrie::new();
        trie.insert("a");
        trie.insert("b");
        let results = trie.starts_with("");
        assert_eq!(results.len(), 2);
    }
    #[test]
    fn test_name_index_basic() {
        let mut idx = NameIndex::new();
        let id1 = idx.insert("Nat");
        let id2 = idx.insert("Bool");
        let id3 = idx.insert("Nat");
        assert_eq!(id1, id3);
        assert_ne!(id1, id2);
        assert_eq!(idx.len(), 2);
    }
    #[test]
    fn test_name_index_get() {
        let mut idx = NameIndex::new();
        let id = idx.insert("test");
        assert_eq!(idx.get_id("test"), Some(id));
        assert_eq!(idx.get_name(id), Some("test"));
        assert_eq!(idx.get_id("missing"), None);
    }
    #[test]
    fn test_name_index_empty() {
        let idx = NameIndex::new();
        assert!(idx.is_empty());
        assert_eq!(idx.len(), 0);
    }
}
#[cfg(test)]
mod tests_padding_infra {
    use super::*;
    #[test]
    fn test_stat_summary() {
        let mut ss = StatSummary::new();
        ss.record(10.0);
        ss.record(20.0);
        ss.record(30.0);
        assert_eq!(ss.count(), 3);
        assert!((ss.mean().expect("mean should succeed") - 20.0).abs() < 1e-9);
        assert_eq!(ss.min().expect("min should succeed") as i64, 10);
        assert_eq!(ss.max().expect("max should succeed") as i64, 30);
    }
    #[test]
    fn test_transform_stat() {
        let mut ts = TransformStat::new();
        ts.record_before(100.0);
        ts.record_after(80.0);
        let ratio = ts.mean_ratio().expect("ratio should be present");
        assert!((ratio - 0.8).abs() < 1e-9);
    }
    #[test]
    fn test_small_map() {
        let mut m: SmallMap<u32, &str> = SmallMap::new();
        m.insert(3, "three");
        m.insert(1, "one");
        m.insert(2, "two");
        assert_eq!(m.get(&2), Some(&"two"));
        assert_eq!(m.len(), 3);
        let keys = m.keys();
        assert_eq!(*keys[0], 1);
        assert_eq!(*keys[2], 3);
    }
    #[test]
    fn test_label_set() {
        let mut ls = LabelSet::new();
        ls.add("foo");
        ls.add("bar");
        ls.add("foo");
        assert_eq!(ls.count(), 2);
        assert!(ls.has("bar"));
        assert!(!ls.has("baz"));
    }
    #[test]
    fn test_config_node() {
        let mut root = ConfigNode::section("root");
        let child = ConfigNode::leaf("key", "value");
        root.add_child(child);
        assert_eq!(root.num_children(), 1);
    }
    #[test]
    fn test_versioned_record() {
        let mut vr = VersionedRecord::new(0u32);
        vr.update(1);
        vr.update(2);
        assert_eq!(*vr.current(), 2);
        assert_eq!(vr.version(), 2);
        assert!(vr.has_history());
        assert_eq!(*vr.at_version(0).expect("value should be present"), 0);
    }
    #[test]
    fn test_simple_dag() {
        let mut dag = SimpleDag::new(4);
        dag.add_edge(0, 1);
        dag.add_edge(1, 2);
        dag.add_edge(2, 3);
        assert!(dag.can_reach(0, 3));
        assert!(!dag.can_reach(3, 0));
        let order = dag.topological_sort().expect("order should be present");
        assert_eq!(order, vec![0, 1, 2, 3]);
    }
    #[test]
    fn test_focus_stack() {
        let mut fs: FocusStack<&str> = FocusStack::new();
        fs.focus("a");
        fs.focus("b");
        assert_eq!(fs.current(), Some(&"b"));
        assert_eq!(fs.depth(), 2);
        fs.blur();
        assert_eq!(fs.current(), Some(&"a"));
    }
}
#[cfg(test)]
mod tests_extra_iterators {
    use super::*;
    #[test]
    fn test_window_iterator() {
        let data = vec![1u32, 2, 3, 4, 5];
        let windows: Vec<_> = WindowIterator::new(&data, 3).collect();
        assert_eq!(windows.len(), 3);
        assert_eq!(windows[0], &[1, 2, 3]);
        assert_eq!(windows[2], &[3, 4, 5]);
    }
    #[test]
    fn test_non_empty_vec() {
        let mut nev = NonEmptyVec::singleton(10u32);
        nev.push(20);
        nev.push(30);
        assert_eq!(nev.len(), 3);
        assert_eq!(*nev.first(), 10);
        assert_eq!(*nev.last(), 30);
    }
}
#[cfg(test)]
mod tests_padding2 {
    use super::*;
    #[test]
    fn test_sliding_sum() {
        let mut ss = SlidingSum::new(3);
        ss.push(1.0);
        ss.push(2.0);
        ss.push(3.0);
        assert!((ss.sum() - 6.0).abs() < 1e-9);
        ss.push(4.0);
        assert!((ss.sum() - 9.0).abs() < 1e-9);
        assert_eq!(ss.count(), 3);
    }
    #[test]
    fn test_path_buf() {
        let mut pb = PathBuf::new();
        pb.push("src");
        pb.push("main");
        assert_eq!(pb.as_str(), "src/main");
        assert_eq!(pb.depth(), 2);
        pb.pop();
        assert_eq!(pb.as_str(), "src");
    }
    #[test]
    fn test_string_pool() {
        let mut pool = StringPool::new();
        let s = pool.take();
        assert!(s.is_empty());
        pool.give("hello".to_string());
        let s2 = pool.take();
        assert!(s2.is_empty());
        assert_eq!(pool.free_count(), 0);
    }
    #[test]
    fn test_transitive_closure() {
        let mut tc = TransitiveClosure::new(4);
        tc.add_edge(0, 1);
        tc.add_edge(1, 2);
        tc.add_edge(2, 3);
        assert!(tc.can_reach(0, 3));
        assert!(!tc.can_reach(3, 0));
        let r = tc.reachable_from(0);
        assert_eq!(r.len(), 4);
    }
    #[test]
    fn test_token_bucket() {
        let mut tb = TokenBucket::new(100, 10);
        assert_eq!(tb.available(), 100);
        assert!(tb.try_consume(50));
        assert_eq!(tb.available(), 50);
        assert!(!tb.try_consume(60));
        assert_eq!(tb.capacity(), 100);
    }
    #[test]
    fn test_rewrite_rule_set() {
        let mut rrs = RewriteRuleSet::new();
        rrs.add(RewriteRule::unconditional(
            "beta",
            "App(Lam(x, b), v)",
            "b[x:=v]",
        ));
        rrs.add(RewriteRule::conditional("comm", "a + b", "b + a"));
        assert_eq!(rrs.len(), 2);
        assert_eq!(rrs.unconditional_rules().len(), 1);
        assert_eq!(rrs.conditional_rules().len(), 1);
        assert!(rrs.get("beta").is_some());
        let disp = rrs
            .get("beta")
            .expect("element at \'beta\' should exist")
            .display();
        assert!(disp.contains("→"));
    }
}
#[cfg(test)]
mod tests_padding3 {
    use super::*;
    #[test]
    fn test_decision_node() {
        let tree = DecisionNode::Branch {
            key: "x".into(),
            val: "1".into(),
            yes_branch: Box::new(DecisionNode::Leaf("yes".into())),
            no_branch: Box::new(DecisionNode::Leaf("no".into())),
        };
        let mut ctx = std::collections::HashMap::new();
        ctx.insert("x".into(), "1".into());
        assert_eq!(tree.evaluate(&ctx), "yes");
        ctx.insert("x".into(), "2".into());
        assert_eq!(tree.evaluate(&ctx), "no");
        assert_eq!(tree.depth(), 1);
    }
    #[test]
    fn test_flat_substitution() {
        let mut sub = FlatSubstitution::new();
        sub.add("foo", "bar");
        sub.add("baz", "qux");
        assert_eq!(sub.apply("foo and baz"), "bar and qux");
        assert_eq!(sub.len(), 2);
    }
    #[test]
    fn test_stopwatch() {
        let mut sw = Stopwatch::start();
        sw.split();
        sw.split();
        assert_eq!(sw.num_splits(), 2);
        assert!(sw.elapsed_ms() >= 0.0);
        for &s in sw.splits() {
            assert!(s >= 0.0);
        }
    }
    #[test]
    fn test_either2() {
        let e: Either2<i32, &str> = Either2::First(42);
        assert!(e.is_first());
        let mapped = e.map_first(|x| x * 2);
        assert_eq!(mapped.first(), Some(84));
        let e2: Either2<i32, &str> = Either2::Second("hello");
        assert!(e2.is_second());
        assert_eq!(e2.second(), Some("hello"));
    }
    #[test]
    fn test_write_once() {
        let wo: WriteOnce<u32> = WriteOnce::new();
        assert!(!wo.is_written());
        assert!(wo.write(42));
        assert!(!wo.write(99));
        assert_eq!(wo.read(), Some(42));
    }
    #[test]
    fn test_sparse_vec() {
        let mut sv: SparseVec<i32> = SparseVec::new(100);
        sv.set(5, 10);
        sv.set(50, 20);
        assert_eq!(*sv.get(5), 10);
        assert_eq!(*sv.get(50), 20);
        assert_eq!(*sv.get(0), 0);
        assert_eq!(sv.nnz(), 2);
        sv.set(5, 0);
        assert_eq!(sv.nnz(), 1);
    }
    #[test]
    fn test_stack_calc() {
        let mut calc = StackCalc::new();
        calc.push(3);
        calc.push(4);
        calc.add();
        assert_eq!(calc.peek(), Some(7));
        calc.push(2);
        calc.mul();
        assert_eq!(calc.peek(), Some(14));
    }
}
#[cfg(test)]
mod tests_final_padding {
    use super::*;
    #[test]
    fn test_min_heap() {
        let mut h = MinHeap::new();
        h.push(5u32);
        h.push(1u32);
        h.push(3u32);
        assert_eq!(h.peek(), Some(&1));
        assert_eq!(h.pop(), Some(1));
        assert_eq!(h.pop(), Some(3));
        assert_eq!(h.pop(), Some(5));
        assert!(h.is_empty());
    }
    #[test]
    fn test_prefix_counter() {
        let mut pc = PrefixCounter::new();
        pc.record("hello");
        pc.record("help");
        pc.record("world");
        assert_eq!(pc.count_with_prefix("hel"), 2);
        assert_eq!(pc.count_with_prefix("wor"), 1);
        assert_eq!(pc.count_with_prefix("xyz"), 0);
    }
    #[test]
    fn test_fixture() {
        let mut f = Fixture::new();
        f.set("key1", "val1");
        f.set("key2", "val2");
        assert_eq!(f.get("key1"), Some("val1"));
        assert_eq!(f.get("key3"), None);
        assert_eq!(f.len(), 2);
    }
}
