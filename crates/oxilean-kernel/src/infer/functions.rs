//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::{BinderInfo, Environment, Expr, FVarId, Level, Literal, Name};

use super::types::{
    ConfigNode, DecisionNode, Either2, FlatSubstitution, FocusStack, InferCache, InferStats,
    LabelSet, NonEmptyVec, PathBuf, RewriteRule, RewriteRuleSet, SimpleDag, SlidingSum, SmallMap,
    SparseVec, StackCalc, StatSummary, Stopwatch, StringPool, TokenBucket, TransformStat,
    TransitiveClosure, TypeChecker, TypeKind, TypingJudgment, VersionedRecord, WindowIterator,
    WriteOnce,
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{BinderInfo, Declaration};
    #[test]
    fn test_infer_sort() {
        let env = Environment::new();
        let mut tc = TypeChecker::new(&env);
        let prop = Expr::Sort(Level::zero());
        let prop_ty = tc.infer_type(&prop).expect("prop_ty should be present");
        assert_eq!(prop_ty, Expr::Sort(Level::succ(Level::zero())));
    }
    #[test]
    fn test_infer_constant() {
        let mut env = Environment::new();
        let nat_ty = Expr::Sort(Level::succ(Level::zero()));
        env.add(Declaration::Axiom {
            name: Name::str("Nat"),
            univ_params: vec![],
            ty: nat_ty.clone(),
        })
        .expect("value should be present");
        let mut tc = TypeChecker::new(&env);
        let nat_const = Expr::Const(Name::str("Nat"), vec![]);
        let result = tc.infer_type(&nat_const).expect("result should be present");
        assert_eq!(result, nat_ty);
    }
    #[test]
    fn test_infer_lambda() {
        let env = Environment::new();
        let mut tc = TypeChecker::new(&env);
        let ty = Expr::Sort(Level::zero());
        let body = Expr::BVar(0);
        let lam = Expr::Lam(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(ty.clone()),
            Box::new(body),
        );
        let result = tc.infer_type(&lam).expect("result should be present");
        assert!(matches!(result, Expr::Pi(_, _, _, _)));
    }
    #[test]
    fn test_infer_literal() {
        let env = Environment::new();
        let mut tc = TypeChecker::new(&env);
        let nat_lit = Expr::Lit(Literal::Nat(42));
        let result = tc.infer_type(&nat_lit).expect("result should be present");
        assert_eq!(result, Expr::Const(Name::str("Nat"), vec![]));
    }
    #[test]
    fn test_ensure_sort() {
        let env = Environment::new();
        let mut tc = TypeChecker::new(&env);
        let prop = Expr::Sort(Level::zero());
        let level = tc.ensure_sort(&prop).expect("level should be present");
        assert_eq!(level, Level::succ(Level::zero()));
    }
    #[test]
    fn test_def_eq_integration() {
        let mut env = Environment::new();
        let val = Expr::Lit(Literal::Nat(42));
        env.add(Declaration::Definition {
            name: Name::str("answer"),
            univ_params: vec![],
            ty: Expr::Const(Name::str("Nat"), vec![]),
            val: val.clone(),
            hint: crate::ReducibilityHint::Regular(1),
        })
        .expect("value should be present");
        let mut tc = TypeChecker::new(&env);
        let answer = Expr::Const(Name::str("answer"), vec![]);
        assert!(tc.is_def_eq(&answer, &val));
    }
    #[test]
    fn test_whnf_integration() {
        let mut env = Environment::new();
        let val = Expr::Lit(Literal::Nat(42));
        env.add(Declaration::Definition {
            name: Name::str("x"),
            univ_params: vec![],
            ty: Expr::Const(Name::str("Nat"), vec![]),
            val: val.clone(),
            hint: crate::ReducibilityHint::Abbrev,
        })
        .expect("value should be present");
        let mut tc = TypeChecker::new(&env);
        let result = tc.whnf(&Expr::Const(Name::str("x"), vec![]));
        assert_eq!(result, val);
    }
    #[test]
    fn test_infer_only_mode() {
        let env = Environment::new();
        let mut tc = TypeChecker::new_infer_only(&env);
        let ty = Expr::FVar(FVarId(999));
        let body = Expr::BVar(0);
        let lam = Expr::Lam(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(ty),
            Box::new(body),
        );
        let result = tc.infer_type(&lam);
        assert!(result.is_ok());
    }
    #[test]
    fn test_const_with_levels() {
        let mut env = Environment::new();
        let ci = crate::ConstantInfo::Axiom(crate::declaration::AxiomVal {
            common: crate::declaration::ConstantVal {
                name: Name::str("List"),
                level_params: vec![Name::str("u")],
                ty: Expr::Sort(Level::succ(Level::param(Name::str("u")))),
            },
            is_unsafe: false,
        });
        env.add_constant(ci).expect("value should be present");
        let mut tc = TypeChecker::new(&env);
        let list_type1 = Expr::Const(Name::str("List"), vec![Level::zero()]);
        let result = tc
            .infer_type(&list_type1)
            .expect("result should be present");
        assert_eq!(result, Expr::Sort(Level::succ(Level::zero())));
    }
}
#[cfg(test)]
mod extended_tests {
    use super::*;
    use crate::{BinderInfo, Literal};
    #[test]
    fn test_infer_app_chain_no_args() {
        let env = Environment::new();
        let mut tc = TypeChecker::new(&env);
        let nat_ty = Expr::Sort(Level::succ(Level::zero()));
        let fvar = tc.fresh_fvar(Name::str("f"), nat_ty.clone());
        let result = tc.infer_app_chain(&Expr::FVar(fvar), &[]);
        assert!(result.is_ok());
        assert_eq!(result.expect("result should be valid"), nat_ty);
    }
    #[test]
    fn test_telescope_type_no_pis() {
        let env = Environment::new();
        let mut tc = TypeChecker::new(&env);
        let nat = Expr::Const(Name::str("Nat"), vec![]);
        let (fvars, body) = tc.telescope_type(&nat, 5);
        assert!(fvars.is_empty());
        assert_eq!(body, nat);
    }
    #[test]
    fn test_telescope_type_with_pi() {
        let env = Environment::new();
        let mut tc = TypeChecker::new(&env);
        let nat = Expr::Const(Name::str("Nat"), vec![]);
        let pi = Expr::Pi(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(nat.clone()),
            Box::new(nat.clone()),
        );
        let (fvars, body) = tc.telescope_type(&pi, 1);
        assert_eq!(fvars.len(), 1);
        assert_eq!(body, nat);
    }
    #[test]
    fn test_count_pi_binders_none() {
        let env = Environment::new();
        let mut tc = TypeChecker::new(&env);
        let nat = Expr::Const(Name::str("Nat"), vec![]);
        assert_eq!(tc.count_pi_binders(&nat), 0);
    }
    #[test]
    fn test_count_pi_binders_two() {
        let env = Environment::new();
        let mut tc = TypeChecker::new(&env);
        let nat = Expr::Const(Name::str("Nat"), vec![]);
        let pi = Expr::Pi(
            BinderInfo::Default,
            Name::str("a"),
            Box::new(nat.clone()),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("b"),
                Box::new(nat.clone()),
                Box::new(nat.clone()),
            )),
        );
        assert_eq!(tc.count_pi_binders(&pi), 2);
    }
    #[test]
    fn test_close_type_over_fvars_empty() {
        let env = Environment::new();
        let mut tc = TypeChecker::new(&env);
        let ty = Expr::Const(Name::str("Nat"), vec![]);
        let result = tc.close_type_over_fvars(&[], ty.clone());
        assert_eq!(result, ty);
    }
    #[test]
    fn test_close_term_over_fvars_empty() {
        let env = Environment::new();
        let mut tc = TypeChecker::new(&env);
        let term = Expr::Lit(Literal::Nat(42));
        let result = tc.close_term_over_fvars(&[], term.clone());
        assert_eq!(result, term);
    }
    #[test]
    fn test_try_check_correct() {
        let env = Environment::new();
        let mut tc = TypeChecker::new(&env);
        let prop = Expr::Sort(Level::zero());
        let type1 = Expr::Sort(Level::succ(Level::zero()));
        assert!(tc.try_check(&prop, &type1));
    }
    #[test]
    fn test_try_check_wrong() {
        let env = Environment::new();
        let mut tc = TypeChecker::new(&env);
        let prop = Expr::Sort(Level::zero());
        assert!(!tc.try_check(&prop, &prop));
    }
    #[test]
    fn test_normalize_literal() {
        let env = Environment::new();
        let mut tc = TypeChecker::new(&env);
        let lit = Expr::Lit(Literal::Nat(5));
        let result = tc.normalize(&lit);
        assert_eq!(result, lit);
    }
    #[test]
    fn test_verify_declaration_ok() {
        let env = Environment::new();
        let mut tc = TypeChecker::new(&env);
        let ty = Expr::Sort(Level::succ(Level::zero()));
        assert!(tc.verify_declaration(&Name::str("Foo"), &ty, None).is_ok());
    }
    #[test]
    fn test_infer_stats_default() {
        let stats = InferStats::default();
        assert_eq!(stats.infer_calls, 0);
        assert_eq!(stats.total_ops(), 0);
    }
    #[test]
    fn test_infer_stats_total_ops() {
        let stats = InferStats {
            infer_calls: 3,
            whnf_calls: 2,
            def_eq_calls: 1,
            const_lookups: 4,
            cache_hits: 0,
        };
        assert_eq!(stats.total_ops(), 6);
    }
}
/// Classify the surface shape of an expression.
#[allow(dead_code)]
pub fn classify_expr(expr: &Expr) -> TypeKind {
    match expr {
        Expr::Sort(l) => {
            if let Some(n) = l.to_nat() {
                if n == 0 {
                    TypeKind::Prop
                } else if n == 1 {
                    TypeKind::Type0
                } else {
                    TypeKind::LargeType
                }
            } else {
                TypeKind::Universe
            }
        }
        Expr::Pi(_, _, _, _) => TypeKind::Pi,
        Expr::Lam(_, _, _, _) => TypeKind::Lambda,
        Expr::App(_, _) => TypeKind::Application,
        Expr::FVar(_) => TypeKind::FreeVar,
        Expr::Const(_, _) => TypeKind::Constant,
        Expr::Lit(_) => TypeKind::Literal,
        _ => TypeKind::Unknown,
    }
}
/// Check if an expression is syntactically a universe level (Sort).
#[allow(dead_code)]
pub fn is_sort(expr: &Expr) -> bool {
    matches!(expr, Expr::Sort(_))
}
/// Check if an expression syntactically looks like a Prop (Sort 0).
#[allow(dead_code)]
pub fn is_prop(expr: &Expr) -> bool {
    matches!(expr, Expr::Sort(l) if l.is_zero())
}
/// Check if an expression syntactically looks like a function type (Pi).
#[allow(dead_code)]
pub fn is_pi(expr: &Expr) -> bool {
    matches!(expr, Expr::Pi(_, _, _, _))
}
/// Extract the domain and codomain of a Pi type.
///
/// Returns `None` if not a Pi.
#[allow(dead_code)]
pub fn pi_components(expr: &Expr) -> Option<(&BinderInfo, &Name, &Expr, &Expr)> {
    match expr {
        Expr::Pi(bi, name, ty, body) => Some((bi, name, ty, body)),
        _ => None,
    }
}
/// Compute the sort of a Pi type given the sorts of its domain and codomain.
///
/// This follows the standard rules:
/// - If codomain is Prop → Pi is Prop (impredicativity)
/// - Otherwise → Sort(max(u, v))
#[allow(dead_code)]
pub fn pi_sort(domain_sort: &Level, codomain_sort: &Level) -> Level {
    if codomain_sort.is_zero() {
        Level::zero()
    } else {
        Level::max(domain_sort.clone(), codomain_sort.clone())
    }
}
#[cfg(test)]
mod extra_infer_tests {
    use super::*;
    #[test]
    fn test_infer_cache_insert_get() {
        let mut cache = InferCache::new(10);
        let expr = Expr::Lit(Literal::Nat(42));
        let ty = Expr::Const(Name::str("Nat"), vec![]);
        cache.insert(expr.clone(), ty.clone());
        assert_eq!(cache.get(&expr), Some(&ty));
    }
    #[test]
    fn test_infer_cache_miss() {
        let cache = InferCache::new(10);
        let expr = Expr::Lit(Literal::Nat(1));
        assert_eq!(cache.get(&expr), None);
    }
    #[test]
    fn test_infer_cache_eviction() {
        let mut cache = InferCache::new(2);
        let e0 = Expr::Lit(Literal::Nat(0));
        let e1 = Expr::Lit(Literal::Nat(1));
        let e2 = Expr::Lit(Literal::Nat(2));
        let ty = Expr::Sort(Level::zero());
        cache.insert(e0.clone(), ty.clone());
        cache.insert(e1.clone(), ty.clone());
        cache.insert(e2.clone(), ty.clone());
        assert_eq!(cache.get(&e0), None);
        assert!(cache.get(&e1).is_some());
        assert!(cache.get(&e2).is_some());
    }
    #[test]
    fn test_infer_cache_clear() {
        let mut cache = InferCache::new(5);
        cache.insert(Expr::Lit(Literal::Nat(1)), Expr::Sort(Level::zero()));
        cache.clear();
        assert!(cache.is_empty());
    }
    #[test]
    fn test_typing_judgment_ok() {
        let j = TypingJudgment::ok(
            Expr::Lit(Literal::Nat(1)),
            Expr::Const(Name::str("Nat"), vec![]),
        );
        assert!(j.is_ok());
    }
    #[test]
    fn test_typing_judgment_fail() {
        let j = TypingJudgment::fail(Expr::Lit(Literal::Nat(0)));
        assert!(!j.is_ok());
    }
    #[test]
    fn test_classify_expr_prop() {
        let prop = Expr::Sort(Level::zero());
        assert_eq!(classify_expr(&prop), TypeKind::Prop);
    }
    #[test]
    fn test_classify_expr_type0() {
        let t0 = Expr::Sort(Level::succ(Level::zero()));
        assert_eq!(classify_expr(&t0), TypeKind::Type0);
    }
    #[test]
    fn test_classify_expr_pi() {
        let pi = Expr::Pi(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(Expr::Sort(Level::zero())),
            Box::new(Expr::Sort(Level::zero())),
        );
        assert_eq!(classify_expr(&pi), TypeKind::Pi);
    }
    #[test]
    fn test_classify_expr_lambda() {
        let lam = Expr::Lam(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(Expr::Sort(Level::zero())),
            Box::new(Expr::BVar(0)),
        );
        assert_eq!(classify_expr(&lam), TypeKind::Lambda);
    }
    #[test]
    fn test_classify_expr_literal() {
        let lit = Expr::Lit(Literal::Nat(5));
        assert_eq!(classify_expr(&lit), TypeKind::Literal);
    }
    #[test]
    fn test_is_sort_true() {
        assert!(is_sort(&Expr::Sort(Level::zero())));
    }
    #[test]
    fn test_is_sort_false() {
        assert!(!is_sort(&Expr::Lit(Literal::Nat(1))));
    }
    #[test]
    fn test_is_prop_true() {
        assert!(is_prop(&Expr::Sort(Level::zero())));
    }
    #[test]
    fn test_is_prop_false() {
        assert!(!is_prop(&Expr::Sort(Level::succ(Level::zero()))));
    }
    #[test]
    fn test_is_pi_true() {
        let pi = Expr::Pi(
            BinderInfo::Default,
            Name::str("_"),
            Box::new(Expr::Sort(Level::zero())),
            Box::new(Expr::Sort(Level::zero())),
        );
        assert!(is_pi(&pi));
    }
    #[test]
    fn test_pi_components() {
        let pi = Expr::Pi(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(Expr::Sort(Level::zero())),
            Box::new(Expr::Sort(Level::succ(Level::zero()))),
        );
        let result = pi_components(&pi);
        assert!(result.is_some());
        let (_, name, _, _) = result.expect("result should be valid");
        assert_eq!(*name, Name::str("x"));
    }
    #[test]
    fn test_pi_sort_prop_codomain() {
        let dom = Level::succ(Level::zero());
        let cod = Level::zero();
        let result = pi_sort(&dom, &cod);
        assert!(result.is_zero());
    }
    #[test]
    fn test_pi_sort_type_codomain() {
        let dom = Level::succ(Level::zero());
        let cod = Level::succ(Level::zero());
        let result = pi_sort(&dom, &cod);
        assert!(!result.is_zero());
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
