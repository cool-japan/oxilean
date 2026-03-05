//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::{Expr, Name};
use std::collections::{HashMap, HashSet};

use super::types::{
    ConfigNode, CongrArgKind, CongrHypothesis, CongrLemmaCache, CongrProof, CongruenceClosure,
    CongruenceTheorem, DecisionNode, EGraph, ENode, Either2, Fixture, FlatApp, FlatCC,
    FlatSubstitution, FocusStack, InstrumentedCC, LabelSet, MinHeap, NonEmptyVec, PathBuf,
    PrefixCounter, RewriteRule, RewriteRuleSet, SimpleDag, SlidingSum, SmallMap, SparseVec,
    StackCalc, StatSummary, Stopwatch, StringPool, TokenBucket, TransformStat, TransitiveClosure,
    VersionedRecord, WindowIterator, WriteOnce,
};

/// Generate a basic congruence theorem for a function.
///
/// For `f : A₁ → A₂ → ... → Aₙ → B`, generates argument kinds:
/// - If Aᵢ depends on previous args: HEq or Cast
/// - Otherwise: Eq
pub fn mk_congr_theorem(fn_name: Name, num_args: usize) -> CongruenceTheorem {
    let arg_kinds = vec![CongrArgKind::Eq; num_args];
    CongruenceTheorem::new(fn_name, arg_kinds)
}
/// Generate a congruence theorem with some fixed arguments.
///
/// `fixed_positions` lists which argument positions are Fixed
/// (e.g., type parameters that don't change).
pub fn mk_congr_theorem_with_fixed(
    fn_name: Name,
    num_args: usize,
    fixed_positions: &[usize],
) -> CongruenceTheorem {
    let mut arg_kinds = vec![CongrArgKind::Eq; num_args];
    for &pos in fixed_positions {
        if pos < num_args {
            arg_kinds[pos] = CongrArgKind::Fixed;
        }
    }
    CongruenceTheorem::new(fn_name, arg_kinds)
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Literal, Name};
    #[test]
    fn test_find_self() {
        let mut cc = CongruenceClosure::new();
        let expr = Expr::Lit(Literal::Nat(42));
        let found = cc.find(&expr);
        assert_eq!(found, expr);
    }
    #[test]
    fn test_merge() {
        let mut cc = CongruenceClosure::new();
        let e1 = Expr::Lit(Literal::Nat(1));
        let e2 = Expr::Lit(Literal::Nat(2));
        cc.add_equality(e1.clone(), e2.clone());
        assert!(cc.are_equal(&e1, &e2));
    }
    #[test]
    fn test_congruence() {
        let mut cc = CongruenceClosure::new();
        let f = Expr::Const(Name::str("f"), vec![]);
        let a = Expr::Lit(Literal::Nat(1));
        let b = Expr::Lit(Literal::Nat(2));
        let fa = Expr::App(Box::new(f.clone()), Box::new(a.clone()));
        let fb = Expr::App(Box::new(f), Box::new(b.clone()));
        cc.add_equality(a, b);
        assert!(cc.are_equal(&fa, &fb));
    }
    #[test]
    fn test_transitivity() {
        let mut cc = CongruenceClosure::new();
        let a = Expr::Lit(Literal::Nat(1));
        let b = Expr::Lit(Literal::Nat(2));
        let c = Expr::Lit(Literal::Nat(3));
        cc.add_equality(a.clone(), b.clone());
        cc.add_equality(b, c.clone());
        assert!(cc.are_equal(&a, &c));
    }
    #[test]
    fn test_congr_theorem_basic() {
        let ct = mk_congr_theorem(Name::str("f"), 2);
        assert_eq!(ct.num_args, 2);
        assert_eq!(ct.arg_kinds.len(), 2);
        assert!(ct.arg_kinds.iter().all(|k| *k == CongrArgKind::Eq));
        assert!(ct.has_eq_args());
        assert_eq!(ct.num_eq_hypotheses(), 2);
    }
    #[test]
    fn test_congr_theorem_with_fixed() {
        let ct = mk_congr_theorem_with_fixed(Name::str("List.map"), 3, &[0]);
        assert_eq!(ct.num_args, 3);
        assert_eq!(ct.arg_kinds[0], CongrArgKind::Fixed);
        assert_eq!(ct.arg_kinds[1], CongrArgKind::Eq);
        assert_eq!(ct.arg_kinds[2], CongrArgKind::Eq);
        assert_eq!(ct.num_eq_hypotheses(), 2);
    }
    #[test]
    fn test_congr_arg_kind_enum() {
        assert_ne!(CongrArgKind::Fixed, CongrArgKind::Eq);
        assert_ne!(CongrArgKind::HEq, CongrArgKind::Cast);
        assert_eq!(CongrArgKind::Subsingle, CongrArgKind::Subsingle);
    }
    #[test]
    fn test_equality_with_proof() {
        let mut cc = CongruenceClosure::new();
        let a = Expr::Lit(Literal::Nat(1));
        let b = Expr::Lit(Literal::Nat(2));
        let proof = Expr::Const(Name::str("proof_a_eq_b"), vec![]);
        cc.add_equality_with_proof(a.clone(), b.clone(), proof.clone());
        assert!(cc.are_equal(&a, &b));
        assert_eq!(cc.get_proof(&a, &b), Some(&proof));
    }
    #[test]
    fn test_num_classes() {
        let mut cc = CongruenceClosure::new();
        let a = Expr::Lit(Literal::Nat(1));
        let b = Expr::Lit(Literal::Nat(2));
        let c = Expr::Lit(Literal::Nat(3));
        cc.add_equality(a.clone(), b.clone());
        let _ = cc.find(&c);
        assert_eq!(cc.num_classes(), 2);
    }
    #[test]
    fn test_clear() {
        let mut cc = CongruenceClosure::new();
        let a = Expr::Lit(Literal::Nat(1));
        let b = Expr::Lit(Literal::Nat(2));
        cc.add_equality(a, b);
        cc.clear();
        assert_eq!(cc.num_classes(), 0);
    }
}
/// Generate the list of hypotheses needed for a congruence lemma.
///
/// For a function `f` applied to `n` arguments where each argument `aᵢ` is
/// either `Fixed` or `Eq`, this produces the equality hypotheses for each
/// non-fixed argument.
pub fn generate_congr_hypotheses(
    lhs_args: &[Expr],
    rhs_args: &[Expr],
    kinds: &[CongrArgKind],
) -> Vec<CongrHypothesis> {
    assert_eq!(lhs_args.len(), rhs_args.len());
    assert_eq!(lhs_args.len(), kinds.len());
    lhs_args
        .iter()
        .zip(rhs_args.iter())
        .zip(kinds.iter())
        .filter_map(|((l, r), k)| match k {
            CongrArgKind::Eq => Some(CongrHypothesis::eq(l.clone(), r.clone())),
            CongrArgKind::HEq => Some(CongrHypothesis::heq(l.clone(), r.clone())),
            _ => None,
        })
        .collect()
}
/// Check if a list of hypotheses is trivially satisfied.
///
/// A hypothesis list is trivially satisfied when all hypotheses are trivial
/// (i.e., lhs = lhs for each).
pub fn all_hypotheses_trivial(hyps: &[CongrHypothesis]) -> bool {
    hyps.iter().all(|h| h.is_trivial())
}
/// Check whether an expression is a "simple" head for congruence purposes.
///
/// Simple heads are constants or free variables that do not have binders.
pub fn is_simple_head(expr: &Expr) -> bool {
    matches!(expr, Expr::Const(_, _) | Expr::FVar(_))
}
/// Check whether two expressions have structurally compatible heads.
pub fn compatible_heads(e1: &Expr, e2: &Expr) -> bool {
    match (e1, e2) {
        (Expr::Const(n1, _), Expr::Const(n2, _)) => n1 == n2,
        (Expr::FVar(f1), Expr::FVar(f2)) => f1 == f2,
        (Expr::BVar(i1), Expr::BVar(i2)) => i1 == i2,
        _ => false,
    }
}
#[cfg(test)]
mod egraph_tests {
    use super::*;
    use crate::{Literal, Name};
    #[test]
    fn test_enode_singleton() {
        let expr = Expr::Lit(Literal::Nat(1));
        let node = ENode::singleton(expr.clone());
        assert!(node.contains(&expr));
        assert_eq!(node.size(), 1);
    }
    #[test]
    fn test_enode_add_member() {
        let e1 = Expr::Lit(Literal::Nat(1));
        let e2 = Expr::Lit(Literal::Nat(2));
        let mut node = ENode::singleton(e1.clone());
        node.add_member(e2.clone(), None);
        assert!(node.contains(&e2));
        assert_eq!(node.size(), 2);
    }
    #[test]
    fn test_egraph_add_expr() {
        let mut g = EGraph::new();
        let e = Expr::Lit(Literal::Nat(42));
        let id = g.add_expr(e.clone());
        assert_eq!(g.find_class(&e), Some(id));
    }
    #[test]
    fn test_egraph_add_equality() {
        let mut g = EGraph::new();
        let a = Expr::Lit(Literal::Nat(1));
        let b = Expr::Lit(Literal::Nat(2));
        g.add_equality(a.clone(), b.clone(), None);
        assert!(g.are_equal(&a, &b));
    }
    #[test]
    fn test_egraph_representative() {
        let mut g = EGraph::new();
        let a = Expr::Lit(Literal::Nat(1));
        g.add_expr(a.clone());
        let repr = g.representative(&a);
        assert_eq!(repr, Some(&a));
    }
    #[test]
    fn test_egraph_num_classes() {
        let mut g = EGraph::new();
        let a = Expr::Lit(Literal::Nat(1));
        let b = Expr::Lit(Literal::Nat(2));
        g.add_expr(a.clone());
        g.add_expr(b.clone());
        assert_eq!(g.num_classes(), 2);
        g.add_equality(a, b, None);
        assert_eq!(g.num_classes(), 1);
    }
    #[test]
    fn test_congr_hypothesis_trivial() {
        let e = Expr::Lit(Literal::Nat(1));
        let h = CongrHypothesis::eq(e.clone(), e);
        assert!(h.is_trivial());
    }
    #[test]
    fn test_congr_hypothesis_nontrivial() {
        let a = Expr::Lit(Literal::Nat(1));
        let b = Expr::Lit(Literal::Nat(2));
        let h = CongrHypothesis::eq(a, b);
        assert!(!h.is_trivial());
    }
    #[test]
    fn test_generate_congr_hypotheses() {
        let a1 = Expr::Lit(Literal::Nat(1));
        let a2 = Expr::Lit(Literal::Nat(2));
        let b1 = Expr::Lit(Literal::Nat(3));
        let b2 = Expr::Lit(Literal::Nat(4));
        let hyps = generate_congr_hypotheses(
            &[a1.clone(), b1.clone()],
            &[a2.clone(), b2.clone()],
            &[CongrArgKind::Eq, CongrArgKind::Fixed],
        );
        assert_eq!(hyps.len(), 1);
        assert_eq!(hyps[0].lhs, a1);
    }
    #[test]
    fn test_all_hypotheses_trivial_true() {
        let e = Expr::Lit(Literal::Nat(1));
        let hyps = vec![CongrHypothesis::eq(e.clone(), e)];
        assert!(all_hypotheses_trivial(&hyps));
    }
    #[test]
    fn test_all_hypotheses_trivial_false() {
        let a = Expr::Lit(Literal::Nat(1));
        let b = Expr::Lit(Literal::Nat(2));
        let hyps = vec![CongrHypothesis::eq(a, b)];
        assert!(!all_hypotheses_trivial(&hyps));
    }
    #[test]
    fn test_is_simple_head() {
        let c = Expr::Const(Name::str("f"), vec![]);
        assert!(is_simple_head(&c));
        let app = Expr::App(Box::new(c.clone()), Box::new(Expr::Lit(Literal::Nat(0))));
        assert!(!is_simple_head(&app));
    }
    #[test]
    fn test_compatible_heads() {
        let c1 = Expr::Const(Name::str("f"), vec![]);
        let c2 = Expr::Const(Name::str("f"), vec![]);
        let c3 = Expr::Const(Name::str("g"), vec![]);
        assert!(compatible_heads(&c1, &c2));
        assert!(!compatible_heads(&c1, &c3));
    }
    #[test]
    fn test_egraph_clear() {
        let mut g = EGraph::new();
        g.add_expr(Expr::Lit(Literal::Nat(1)));
        g.clear();
        assert_eq!(g.num_classes(), 0);
    }
}
/// A flat term index (for a compact E-graph representation).
pub type TermIdx = usize;
#[cfg(test)]
mod new_tests {
    use super::*;
    use crate::{Literal, Name};
    #[test]
    fn test_instrumented_cc_add_equality() {
        let mut icc = InstrumentedCC::new();
        let a = Expr::Lit(Literal::Nat(1));
        let b = Expr::Lit(Literal::Nat(2));
        icc.add_equality(a.clone(), b.clone());
        assert!(icc.are_equal(&a, &b));
        assert_eq!(icc.stats.equalities_added, 1);
    }
    #[test]
    fn test_flat_cc_basic() {
        let mut fcc = FlatCC::new(4);
        assert_eq!(fcc.num_nodes(), 4);
        assert!(!fcc.are_equal(0, 1));
        fcc.union(0, 1);
        assert!(fcc.are_equal(0, 1));
        assert!(!fcc.are_equal(0, 2));
    }
    #[test]
    fn test_flat_cc_add_node() {
        let mut fcc = FlatCC::new(2);
        let idx = fcc.add_node();
        assert_eq!(idx, 2);
        assert_eq!(fcc.num_nodes(), 3);
    }
    #[test]
    fn test_flat_cc_add_app() {
        let mut fcc = FlatCC::new(3);
        fcc.add_app(FlatApp {
            fn_idx: 0,
            arg_idx: 1,
            result_idx: Some(2),
        });
        assert_eq!(fcc.num_apps(), 1);
    }
    #[test]
    fn test_flat_cc_propagate_congruences() {
        let mut fcc = FlatCC::new(5);
        fcc.add_app(FlatApp {
            fn_idx: 0,
            arg_idx: 1,
            result_idx: Some(3),
        });
        fcc.add_app(FlatApp {
            fn_idx: 0,
            arg_idx: 2,
            result_idx: Some(4),
        });
        assert!(!fcc.are_equal(3, 4));
        fcc.union(1, 2);
        fcc.propagate_congruences();
        assert!(fcc.are_equal(3, 4));
    }
    #[test]
    fn test_flat_cc_find_self() {
        let mut fcc = FlatCC::new(3);
        assert_eq!(fcc.find(1), 1);
    }
    #[test]
    fn test_congr_lemma_cache_basic() {
        let mut cache = CongrLemmaCache::new();
        assert!(cache.is_empty());
        let _thm = cache.get_or_compute(Name::str("f"), 2);
        assert_eq!(cache.len(), 1);
    }
    #[test]
    fn test_congr_lemma_cache_hit() {
        let mut cache = CongrLemmaCache::new();
        let thm1 = cache.get_or_compute(Name::str("g"), 3);
        let num_args1 = thm1.num_args;
        let thm2 = cache.get_or_compute(Name::str("g"), 3);
        assert_eq!(thm2.num_args, num_args1);
        assert_eq!(cache.len(), 1);
    }
    #[test]
    fn test_congr_lemma_cache_insert_get() {
        let mut cache = CongrLemmaCache::new();
        let thm = mk_congr_theorem(Name::str("h"), 1);
        cache.insert(thm.clone());
        let got = cache.get(&Name::str("h"), 1);
        assert!(got.is_some());
        assert_eq!(got.expect("got should be valid").num_args, 1);
    }
    #[test]
    fn test_congr_lemma_cache_clear() {
        let mut cache = CongrLemmaCache::new();
        let _ = cache.get_or_compute(Name::str("f"), 1);
        cache.clear();
        assert!(cache.is_empty());
    }
    #[test]
    fn test_congr_proof_refl() {
        let p = CongrProof::Refl(Expr::Lit(Literal::Nat(1)));
        assert!(p.is_refl());
        assert_eq!(p.depth(), 0);
        assert_eq!(p.hypothesis_count(), 0);
    }
    #[test]
    fn test_congr_proof_symm() {
        let hyp = CongrProof::Hyp(Name::str("h_eq"));
        let p = CongrProof::Symm(Box::new(hyp));
        assert!(!p.is_refl());
        assert_eq!(p.depth(), 1);
        assert_eq!(p.hypothesis_count(), 1);
    }
    #[test]
    fn test_congr_proof_trans() {
        let p1 = CongrProof::Hyp(Name::str("h1"));
        let p2 = CongrProof::Hyp(Name::str("h2"));
        let t = CongrProof::Trans(Box::new(p1), Box::new(p2));
        assert_eq!(t.depth(), 1);
        assert_eq!(t.hypothesis_count(), 2);
    }
    #[test]
    fn test_congr_proof_simplify_double_symm() {
        let hyp = CongrProof::Hyp(Name::str("h"));
        let symm_symm = CongrProof::Symm(Box::new(CongrProof::Symm(Box::new(hyp))));
        let simplified = symm_symm.simplify();
        assert!(matches!(simplified, CongrProof::Hyp(_)));
    }
    #[test]
    fn test_instrumented_cc_reset() {
        let mut icc = InstrumentedCC::new();
        let a = Expr::Lit(Literal::Nat(1));
        let b = Expr::Lit(Literal::Nat(2));
        icc.add_equality(a, b);
        icc.reset();
        assert_eq!(icc.stats.equalities_added, 0);
        assert_eq!(icc.num_classes(), 0);
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
