//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{
    DefEqAnalysisPass, DefEqBuilder, DefEqConfig, DefEqConfigStore, DefEqConfigValue,
    DefEqCounterMap, DefEqDiagnostics, DefEqDiff, DefEqExtConfig2100, DefEqExtConfigVal2100,
    DefEqExtDiag2100, DefEqExtDiff2100, DefEqExtMap, DefEqExtPass2100, DefEqExtPipeline2100,
    DefEqExtResult2100, DefEqExtUtil, DefEqPipeline, DefEqResult, DefEqStateMachine, DefEqWindow,
    DefEqWorkQueue, MetaDefEq, UnifConstraint, UnifConstraintQueue, UnificationResult,
    UnificationStats,
};
use crate::basic::{MVarId, MetaContext};
use oxilean_kernel::{Expr, FVarId};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::def_eq::*;
    use crate::MetavarKind;
    use oxilean_kernel::{BinderInfo, Environment, Level, Literal, Name};
    fn mk_env() -> Environment {
        Environment::new()
    }
    #[test]
    fn test_def_eq_identical() {
        let mut deq = MetaDefEq::new();
        let mut ctx = MetaContext::new(mk_env());
        let e = Expr::Lit(Literal::Nat(42));
        assert!(deq.is_def_eq(&e, &e, &mut ctx).is_equal());
    }
    #[test]
    fn test_def_eq_different_lits() {
        let mut deq = MetaDefEq::new();
        let mut ctx = MetaContext::new(mk_env());
        let e1 = Expr::Lit(Literal::Nat(1));
        let e2 = Expr::Lit(Literal::Nat(2));
        assert!(deq.is_def_eq(&e1, &e2, &mut ctx).is_not_equal());
    }
    #[test]
    fn test_def_eq_mvar_assignment() {
        let mut deq = MetaDefEq::new();
        let mut ctx = MetaContext::new(mk_env());
        let ty = Expr::Sort(Level::zero());
        let (id, placeholder) = ctx.mk_fresh_expr_mvar(ty, MetavarKind::Natural);
        let val = Expr::Lit(Literal::Nat(42));
        let result = deq.is_def_eq(&placeholder, &val, &mut ctx);
        assert!(result.is_equal());
        assert!(ctx.is_mvar_assigned(id));
        assert_eq!(ctx.get_mvar_assignment(id), Some(&val));
    }
    #[test]
    fn test_def_eq_mvar_both_sides() {
        let mut deq = MetaDefEq::new();
        let mut ctx = MetaContext::new(mk_env());
        let ty = Expr::Sort(Level::zero());
        let (id1, p1) = ctx.mk_fresh_expr_mvar(ty.clone(), MetavarKind::Natural);
        let (_id2, p2) = ctx.mk_fresh_expr_mvar(ty, MetavarKind::Natural);
        let result = deq.is_def_eq(&p1, &p2, &mut ctx);
        assert!(result.is_equal());
        assert!(ctx.is_mvar_assigned(id1));
    }
    #[test]
    fn test_def_eq_beta_reduction() {
        let mut deq = MetaDefEq::new();
        let mut ctx = MetaContext::new(mk_env());
        let lam = Expr::Lam(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(Expr::Sort(Level::zero())),
            Box::new(Expr::BVar(0)),
        );
        let arg = Expr::Lit(Literal::Nat(42));
        let app = Expr::App(Box::new(lam), Box::new(arg.clone()));
        let result = deq.is_def_eq(&app, &arg, &mut ctx);
        assert!(result.is_equal());
    }
    #[test]
    fn test_def_eq_sorts() {
        let mut deq = MetaDefEq::new();
        let mut ctx = MetaContext::new(mk_env());
        let s1 = Expr::Sort(Level::zero());
        let s2 = Expr::Sort(Level::zero());
        assert!(deq.is_def_eq(&s1, &s2, &mut ctx).is_equal());
        let s3 = Expr::Sort(Level::succ(Level::zero()));
        assert!(deq.is_def_eq(&s1, &s3, &mut ctx).is_not_equal());
    }
    #[test]
    fn test_def_eq_pi() {
        let mut deq = MetaDefEq::new();
        let mut ctx = MetaContext::new(mk_env());
        let p1 = Expr::Pi(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(Expr::Sort(Level::zero())),
            Box::new(Expr::Sort(Level::zero())),
        );
        let p2 = Expr::Pi(
            BinderInfo::Default,
            Name::str("y"),
            Box::new(Expr::Sort(Level::zero())),
            Box::new(Expr::Sort(Level::zero())),
        );
        assert!(deq.is_def_eq(&p1, &p2, &mut ctx).is_equal());
    }
    #[test]
    fn test_occurs_check() {
        let mut deq = MetaDefEq::new();
        let mut ctx = MetaContext::new(mk_env());
        let ty = Expr::Sort(Level::zero());
        let (id, placeholder) = ctx.mk_fresh_expr_mvar(ty, MetavarKind::Natural);
        let bad_val = Expr::App(
            Box::new(Expr::Const(Name::str("f"), vec![])),
            Box::new(placeholder.clone()),
        );
        let result = deq.is_def_eq(&placeholder, &bad_val, &mut ctx);
        assert!(result.is_not_equal());
        assert!(!ctx.is_mvar_assigned(id));
    }
    #[test]
    fn test_def_eq_consts() {
        let mut deq = MetaDefEq::new();
        let mut ctx = MetaContext::new(mk_env());
        let c1 = Expr::Const(Name::str("Nat"), vec![]);
        let c2 = Expr::Const(Name::str("Nat"), vec![]);
        assert!(deq.is_def_eq(&c1, &c2, &mut ctx).is_equal());
        let c3 = Expr::Const(Name::str("Bool"), vec![]);
        assert!(deq.is_def_eq(&c1, &c3, &mut ctx).is_not_equal());
    }
    #[test]
    fn test_def_eq_apps() {
        let mut deq = MetaDefEq::new();
        let mut ctx = MetaContext::new(mk_env());
        let f = Expr::Const(Name::str("f"), vec![]);
        let a = Expr::Lit(Literal::Nat(1));
        let app1 = Expr::App(Box::new(f.clone()), Box::new(a.clone()));
        let app2 = Expr::App(Box::new(f), Box::new(a));
        assert!(deq.is_def_eq(&app1, &app2, &mut ctx).is_equal());
    }
    #[test]
    fn test_def_eq_projs() {
        let mut deq = MetaDefEq::new();
        let mut ctx = MetaContext::new(mk_env());
        let e = Expr::BVar(0);
        let p1 = Expr::Proj(Name::str("Prod"), 0, Box::new(e.clone()));
        let p2 = Expr::Proj(Name::str("Prod"), 0, Box::new(e.clone()));
        assert!(deq.is_def_eq(&p1, &p2, &mut ctx).is_equal());
        let p3 = Expr::Proj(Name::str("Prod"), 1, Box::new(e));
        assert!(deq.is_def_eq(&p1, &p3, &mut ctx).is_not_equal());
    }
    #[test]
    fn test_def_eq_mvar_in_app() {
        let mut deq = MetaDefEq::new();
        let mut ctx = MetaContext::new(mk_env());
        let ty = Expr::Sort(Level::zero());
        let (id, placeholder) = ctx.mk_fresh_expr_mvar(ty, MetavarKind::Natural);
        let lhs = Expr::App(
            Box::new(Expr::Const(Name::str("f"), vec![])),
            Box::new(placeholder),
        );
        let rhs = Expr::App(
            Box::new(Expr::Const(Name::str("f"), vec![])),
            Box::new(Expr::Lit(Literal::Nat(42))),
        );
        let result = deq.is_def_eq(&lhs, &rhs, &mut ctx);
        assert!(result.is_equal());
        assert!(ctx.is_mvar_assigned(id));
    }
    #[test]
    fn test_step_counting() {
        let mut deq = MetaDefEq::new();
        let mut ctx = MetaContext::new(mk_env());
        let e1 = Expr::Lit(Literal::Nat(1));
        let e2 = Expr::Lit(Literal::Nat(2));
        let _ = deq.is_def_eq(&e1, &e2, &mut ctx);
        assert!(deq.num_steps() > 0);
        deq.reset_steps();
        assert_eq!(deq.num_steps(), 0);
    }
}
/// Check whether all pairs in `pairs` are definitionally equal.
///
/// Returns `Equal` if all pairs pass, `NotEqual` at the first failure,
/// or `Postponed` if any pair is postponed.
#[allow(dead_code)]
pub fn all_def_eq(
    pairs: &[(Expr, Expr)],
    deq: &mut MetaDefEq,
    ctx: &mut MetaContext,
) -> UnificationResult {
    let mut postponed = false;
    for (lhs, rhs) in pairs {
        match deq.is_def_eq(lhs, rhs, ctx) {
            UnificationResult::Equal => {}
            UnificationResult::NotEqual => return UnificationResult::NotEqual,
            UnificationResult::Postponed => postponed = true,
        }
    }
    if postponed {
        UnificationResult::Postponed
    } else {
        UnificationResult::Equal
    }
}
#[cfg(test)]
mod extra_def_eq_tests {
    use super::*;
    use crate::def_eq::*;
    use oxilean_kernel::{BinderInfo, Environment, Level, Literal, Name};
    fn mk_env() -> Environment {
        Environment::new()
    }
    #[test]
    fn test_unif_constraint_trivial() {
        let e = Expr::Lit(Literal::Nat(1));
        let c = UnifConstraint::new(e.clone(), e.clone(), 0);
        assert!(c.is_trivial());
    }
    #[test]
    fn test_unif_constraint_not_trivial() {
        let lhs = Expr::Lit(Literal::Nat(1));
        let rhs = Expr::Lit(Literal::Nat(2));
        let c = UnifConstraint::new(lhs, rhs, 0);
        assert!(!c.is_trivial());
    }
    #[test]
    fn test_unif_queue_push_pop() {
        let mut q = UnifConstraintQueue::new();
        let e1 = Expr::Lit(Literal::Nat(1));
        let e2 = Expr::Lit(Literal::Nat(2));
        q.push(e1.clone(), e2.clone(), 0);
        assert_eq!(q.len(), 1);
        let c = q.pop().expect("collection should not be empty");
        assert_eq!(c.lhs, e1);
        assert_eq!(c.rhs, e2);
        assert!(q.is_empty());
    }
    #[test]
    fn test_unif_queue_drain_trivial() {
        let mut q = UnifConstraintQueue::new();
        let e = Expr::Lit(Literal::Nat(42));
        q.push(e.clone(), e.clone(), 0);
        q.push(e.clone(), Expr::Lit(Literal::Nat(99)), 0);
        assert_eq!(q.len(), 2);
        q.drain_trivial();
        assert_eq!(q.len(), 1);
    }
    #[test]
    fn test_all_def_eq_empty() {
        let mut deq = MetaDefEq::new();
        let mut ctx = MetaContext::new(mk_env());
        let result = all_def_eq(&[], &mut deq, &mut ctx);
        assert!(result.is_equal());
    }
    #[test]
    fn test_all_def_eq_all_equal() {
        let mut deq = MetaDefEq::new();
        let mut ctx = MetaContext::new(mk_env());
        let pairs = vec![
            (Expr::Lit(Literal::Nat(1)), Expr::Lit(Literal::Nat(1))),
            (Expr::Sort(Level::zero()), Expr::Sort(Level::zero())),
        ];
        let result = all_def_eq(&pairs, &mut deq, &mut ctx);
        assert!(result.is_equal());
    }
    #[test]
    fn test_all_def_eq_one_not_equal() {
        let mut deq = MetaDefEq::new();
        let mut ctx = MetaContext::new(mk_env());
        let pairs = vec![
            (Expr::Lit(Literal::Nat(1)), Expr::Lit(Literal::Nat(1))),
            (Expr::Lit(Literal::Nat(1)), Expr::Lit(Literal::Nat(2))),
        ];
        let result = all_def_eq(&pairs, &mut deq, &mut ctx);
        assert!(result.is_not_equal());
    }
    #[test]
    fn test_def_eq_lam_same_types() {
        let mut deq = MetaDefEq::new();
        let mut ctx = MetaContext::new(mk_env());
        let l1 = Expr::Lam(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(Expr::Sort(Level::zero())),
            Box::new(Expr::BVar(0)),
        );
        let l2 = Expr::Lam(
            BinderInfo::Default,
            Name::str("y"),
            Box::new(Expr::Sort(Level::zero())),
            Box::new(Expr::BVar(0)),
        );
        assert!(deq.is_def_eq(&l1, &l2, &mut ctx).is_equal());
    }
    #[test]
    fn test_unification_result_methods() {
        assert!(UnificationResult::Equal.is_equal());
        assert!(!UnificationResult::Equal.is_not_equal());
        assert!(UnificationResult::NotEqual.is_not_equal());
        assert!(!UnificationResult::NotEqual.is_equal());
        assert!(!UnificationResult::Postponed.is_equal());
        assert!(!UnificationResult::Postponed.is_not_equal());
    }
    #[test]
    fn test_def_eq_bvars_equal() {
        let mut deq = MetaDefEq::new();
        let mut ctx = MetaContext::new(mk_env());
        let b1 = Expr::BVar(2);
        let b2 = Expr::BVar(2);
        assert!(deq.is_def_eq(&b1, &b2, &mut ctx).is_equal());
    }
    #[test]
    fn test_def_eq_bvars_not_equal() {
        let mut deq = MetaDefEq::new();
        let mut ctx = MetaContext::new(mk_env());
        let b1 = Expr::BVar(0);
        let b2 = Expr::BVar(1);
        assert!(deq.is_def_eq(&b1, &b2, &mut ctx).is_not_equal());
    }
}
/// Quick check: are two expressions structurally identical (no reduction needed)?
pub fn is_structurally_equal(a: &Expr, b: &Expr) -> bool {
    a == b
}
/// Quick check: do two expressions have the same head constant name?
pub fn same_head(a: &Expr, b: &Expr) -> bool {
    fn head_name(e: &Expr) -> Option<&oxilean_kernel::Name> {
        match e {
            Expr::Const(n, _) => Some(n),
            Expr::App(f, _) => head_name(f),
            _ => None,
        }
    }
    match (head_name(a), head_name(b)) {
        (Some(na), Some(nb)) => na == nb,
        _ => false,
    }
}
/// Check if an expression is a de Bruijn variable.
pub fn is_bvar(e: &Expr) -> bool {
    matches!(e, Expr::BVar(_))
}
/// Check if an expression is a free variable.
pub fn is_fvar(e: &Expr) -> bool {
    matches!(e, Expr::FVar(_))
}
#[cfg(test)]
mod def_eq_config_tests {
    use super::*;
    use crate::def_eq::*;
    #[test]
    fn test_def_eq_config_default() {
        let cfg = DefEqConfig::default();
        assert!(cfg.proof_irrelevance);
        assert!(cfg.eta_reduction);
        assert!(cfg.lazy_delta);
    }
    #[test]
    fn test_def_eq_config_strict() {
        let cfg = DefEqConfig::strict();
        assert!(!cfg.proof_irrelevance);
        assert!(!cfg.eta_reduction);
        assert!(!cfg.lazy_delta);
        assert_eq!(cfg.max_delta_steps, 0);
    }
    #[test]
    fn test_def_eq_config_lenient() {
        let cfg = DefEqConfig::lenient();
        assert!(cfg.proof_irrelevance);
        assert_eq!(cfg.max_delta_steps, 4096);
    }
    #[test]
    fn test_unification_stats_record() {
        let mut s = UnificationStats::new();
        s.record(UnificationResult::Equal, 10);
        s.record(UnificationResult::NotEqual, 5);
        s.record(UnificationResult::Postponed, 3);
        assert_eq!(s.successes, 1);
        assert_eq!(s.failures, 1);
        assert_eq!(s.postponed, 1);
        assert_eq!(s.steps, 18);
    }
    #[test]
    fn test_unification_stats_success_rate() {
        let mut s = UnificationStats::new();
        s.record(UnificationResult::Equal, 1);
        s.record(UnificationResult::Equal, 1);
        s.record(UnificationResult::NotEqual, 1);
        let rate = s.success_rate();
        assert!((rate - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_unification_stats_empty_rate() {
        let s = UnificationStats::new();
        assert_eq!(s.success_rate(), 1.0);
    }
    #[test]
    fn test_is_structurally_equal() {
        let a = Expr::BVar(0);
        let b = Expr::BVar(0);
        let c = Expr::BVar(1);
        assert!(is_structurally_equal(&a, &b));
        assert!(!is_structurally_equal(&a, &c));
    }
    #[test]
    fn test_same_head() {
        let a = Expr::Const(oxilean_kernel::Name::str("Nat"), vec![]);
        let b = Expr::Const(oxilean_kernel::Name::str("Nat"), vec![]);
        let c = Expr::Const(oxilean_kernel::Name::str("Bool"), vec![]);
        assert!(same_head(&a, &b));
        assert!(!same_head(&a, &c));
    }
    #[test]
    fn test_is_bvar() {
        assert!(is_bvar(&Expr::BVar(0)));
        assert!(!is_bvar(&Expr::Const(
            oxilean_kernel::Name::str("Nat"),
            vec![]
        )));
    }
    #[test]
    fn test_unification_stats_display() {
        let s = UnificationStats {
            successes: 3,
            failures: 1,
            postponed: 0,
            steps: 10,
        };
        let txt = format!("{}", s);
        assert!(txt.contains("ok: 3"));
    }
}
#[cfg(test)]
mod defeq_ext2_tests {
    use super::*;
    use crate::def_eq::*;
    #[test]
    fn test_defeq_ext_util_basic() {
        let mut u = DefEqExtUtil::new("test");
        u.push(10);
        u.push(20);
        assert_eq!(u.sum(), 30);
        assert_eq!(u.len(), 2);
    }
    #[test]
    fn test_defeq_ext_util_min_max() {
        let mut u = DefEqExtUtil::new("mm");
        u.push(5);
        u.push(1);
        u.push(9);
        assert_eq!(u.min_val(), Some(1));
        assert_eq!(u.max_val(), Some(9));
    }
    #[test]
    fn test_defeq_ext_util_flags() {
        let mut u = DefEqExtUtil::new("flags");
        u.set_flag(3);
        assert!(u.has_flag(3));
        assert!(!u.has_flag(2));
    }
    #[test]
    fn test_defeq_ext_util_pop() {
        let mut u = DefEqExtUtil::new("pop");
        u.push(42);
        assert_eq!(u.pop(), Some(42));
        assert!(u.is_empty());
    }
    #[test]
    fn test_defeq_ext_map_basic() {
        let mut m: DefEqExtMap<i32> = DefEqExtMap::new();
        m.insert("key", 42);
        assert_eq!(m.get("key"), Some(&42));
        assert!(m.contains("key"));
        assert!(!m.contains("other"));
    }
    #[test]
    fn test_defeq_ext_map_get_or_default() {
        let mut m: DefEqExtMap<i32> = DefEqExtMap::new();
        m.insert("k", 5);
        assert_eq!(m.get_or_default("k"), 5);
        assert_eq!(m.get_or_default("missing"), 0);
    }
    #[test]
    fn test_defeq_ext_map_keys_sorted() {
        let mut m: DefEqExtMap<i32> = DefEqExtMap::new();
        m.insert("z", 1);
        m.insert("a", 2);
        m.insert("m", 3);
        let keys = m.keys_sorted();
        assert_eq!(keys[0].as_str(), "a");
        assert_eq!(keys[2].as_str(), "z");
    }
    #[test]
    fn test_defeq_window_mean() {
        let mut w = DefEqWindow::new(3);
        w.push(1.0);
        w.push(2.0);
        w.push(3.0);
        assert!((w.mean() - 2.0).abs() < 1e-10);
    }
    #[test]
    fn test_defeq_window_evict() {
        let mut w = DefEqWindow::new(2);
        w.push(10.0);
        w.push(20.0);
        w.push(30.0);
        assert_eq!(w.len(), 2);
        assert!((w.mean() - 25.0).abs() < 1e-10);
    }
    #[test]
    fn test_defeq_window_std_dev() {
        let mut w = DefEqWindow::new(10);
        for i in 0..10 {
            w.push(i as f64);
        }
        assert!(w.std_dev() > 0.0);
    }
    #[test]
    fn test_defeq_builder_basic() {
        let b = DefEqBuilder::new("test")
            .add_item("a")
            .add_item("b")
            .set_config("key", "val");
        assert_eq!(b.item_count(), 2);
        assert!(b.has_config("key"));
        assert_eq!(b.get_config("key"), Some("val"));
    }
    #[test]
    fn test_defeq_builder_summary() {
        let b = DefEqBuilder::new("suite").add_item("x");
        let s = b.build_summary();
        assert!(s.contains("suite"));
    }
    #[test]
    fn test_defeq_state_machine_start() {
        let mut sm = DefEqStateMachine::new();
        assert!(sm.start());
        assert!(sm.state.is_running());
    }
    #[test]
    fn test_defeq_state_machine_complete() {
        let mut sm = DefEqStateMachine::new();
        sm.start();
        sm.complete();
        assert!(sm.state.is_terminal());
    }
    #[test]
    fn test_defeq_state_machine_fail() {
        let mut sm = DefEqStateMachine::new();
        sm.fail("oops");
        assert!(sm.state.is_terminal());
        assert_eq!(sm.state.error_msg(), Some("oops"));
    }
    #[test]
    fn test_defeq_state_machine_no_transition_after_terminal() {
        let mut sm = DefEqStateMachine::new();
        sm.complete();
        assert!(!sm.start());
    }
    #[test]
    fn test_defeq_work_queue_basic() {
        let mut wq = DefEqWorkQueue::new(10);
        wq.enqueue("task1".to_string());
        wq.enqueue("task2".to_string());
        assert_eq!(wq.pending_count(), 2);
        let t = wq.dequeue();
        assert_eq!(t, Some("task1".to_string()));
        assert_eq!(wq.processed_count(), 1);
    }
    #[test]
    fn test_defeq_work_queue_capacity() {
        let mut wq = DefEqWorkQueue::new(2);
        wq.enqueue("a".to_string());
        wq.enqueue("b".to_string());
        assert!(wq.is_full());
        assert!(!wq.enqueue("c".to_string()));
    }
    #[test]
    fn test_defeq_counter_map_basic() {
        let mut cm = DefEqCounterMap::new();
        cm.increment("apple");
        cm.increment("apple");
        cm.increment("banana");
        assert_eq!(cm.count("apple"), 2);
        assert_eq!(cm.count("banana"), 1);
        assert_eq!(cm.num_unique(), 2);
    }
    #[test]
    fn test_defeq_counter_map_frequency() {
        let mut cm = DefEqCounterMap::new();
        cm.increment("a");
        cm.increment("a");
        cm.increment("b");
        assert!((cm.frequency("a") - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_defeq_counter_map_most_common() {
        let mut cm = DefEqCounterMap::new();
        cm.increment("x");
        cm.increment("y");
        cm.increment("x");
        let (k, v) = cm.most_common().expect("most_common should succeed");
        assert_eq!(k.as_str(), "x");
        assert_eq!(v, 2);
    }
}
#[cfg(test)]
mod defeq_analysis_tests {
    use super::*;
    use crate::def_eq::*;
    #[test]
    fn test_defeq_result_ok() {
        let r = DefEqResult::Ok("success".to_string());
        assert!(r.is_ok());
        assert!(!r.is_err());
        assert_eq!(r.ok_msg(), Some("success"));
        assert!((r.progress() - 1.0).abs() < 1e-10);
    }
    #[test]
    fn test_defeq_result_err() {
        let r = DefEqResult::Err("failure".to_string());
        assert!(r.is_err());
        assert_eq!(r.err_msg(), Some("failure"));
        assert!((r.progress() - 0.0).abs() < 1e-10);
    }
    #[test]
    fn test_defeq_result_partial() {
        let r = DefEqResult::Partial { done: 3, total: 10 };
        assert!(r.is_partial());
        assert!((r.progress() - 0.3).abs() < 1e-10);
    }
    #[test]
    fn test_defeq_result_skipped() {
        let r = DefEqResult::Skipped;
        assert!(r.is_skipped());
    }
    #[test]
    fn test_defeq_analysis_pass_run() {
        let mut p = DefEqAnalysisPass::new("test_pass");
        let r = p.run("hello");
        assert!(r.is_ok());
        assert_eq!(p.total_runs, 1);
        assert_eq!(p.success_count(), 1);
    }
    #[test]
    fn test_defeq_analysis_pass_empty_input() {
        let mut p = DefEqAnalysisPass::new("empty_test");
        let r = p.run("");
        assert!(r.is_err());
        assert_eq!(p.error_count(), 1);
    }
    #[test]
    fn test_defeq_analysis_pass_success_rate() {
        let mut p = DefEqAnalysisPass::new("rate_test");
        p.run("a");
        p.run("b");
        p.run("");
        assert!((p.success_rate() - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_defeq_analysis_pass_disable() {
        let mut p = DefEqAnalysisPass::new("disable_test");
        p.disable();
        assert!(!p.enabled);
        p.enable();
        assert!(p.enabled);
    }
    #[test]
    fn test_defeq_pipeline_basic() {
        let mut pipeline = DefEqPipeline::new("main_pipeline");
        pipeline.add_pass(DefEqAnalysisPass::new("pass1"));
        pipeline.add_pass(DefEqAnalysisPass::new("pass2"));
        assert_eq!(pipeline.num_passes(), 2);
        let results = pipeline.run_all("test_input");
        assert_eq!(results.len(), 2);
    }
    #[test]
    fn test_defeq_pipeline_disabled_pass() {
        let mut pipeline = DefEqPipeline::new("partial");
        let mut p = DefEqAnalysisPass::new("disabled");
        p.disable();
        pipeline.add_pass(p);
        pipeline.add_pass(DefEqAnalysisPass::new("enabled"));
        assert_eq!(pipeline.num_enabled_passes(), 1);
        let results = pipeline.run_all("input");
        assert_eq!(results.len(), 1);
    }
    #[test]
    fn test_defeq_diff_basic() {
        let mut d = DefEqDiff::new();
        d.add("new_item");
        d.remove("old_item");
        d.keep("same_item");
        assert!(!d.is_empty());
        assert_eq!(d.total_changes(), 2);
        assert_eq!(d.net_additions(), 0);
    }
    #[test]
    fn test_defeq_diff_summary() {
        let mut d = DefEqDiff::new();
        d.add("x");
        d.add("y");
        d.remove("z");
        let s = d.summary();
        assert!(s.contains("+2"));
    }
    #[test]
    fn test_defeq_config_set_get() {
        let mut cfg = DefEqConfigStore::new();
        cfg.set_bool("debug", true);
        cfg.set_int("max_iter", 100);
        cfg.set_str("name", "test");
        assert_eq!(cfg.get_bool("debug"), Some(true));
        assert_eq!(cfg.get_int("max_iter"), Some(100));
        assert_eq!(cfg.get_str("name"), Some("test"));
    }
    #[test]
    fn test_defeq_config_read_only() {
        let mut cfg = DefEqConfigStore::new();
        cfg.set_bool("key", true);
        cfg.lock();
        assert!(!cfg.set_bool("key", false));
        assert_eq!(cfg.get_bool("key"), Some(true));
        cfg.unlock();
        assert!(cfg.set_bool("key", false));
    }
    #[test]
    fn test_defeq_config_remove() {
        let mut cfg = DefEqConfigStore::new();
        cfg.set_int("x", 42);
        assert!(cfg.has("x"));
        cfg.remove("x");
        assert!(!cfg.has("x"));
    }
    #[test]
    fn test_defeq_diagnostics_basic() {
        let mut diag = DefEqDiagnostics::new(10);
        diag.error("something went wrong");
        diag.warning("maybe check this");
        diag.note("fyi");
        assert!(diag.has_errors());
        assert!(!diag.is_clean());
        assert_eq!(diag.num_errors(), 1);
        assert_eq!(diag.num_warnings(), 1);
    }
    #[test]
    fn test_defeq_diagnostics_max_errors() {
        let mut diag = DefEqDiagnostics::new(2);
        diag.error("e1");
        diag.error("e2");
        diag.error("e3");
        assert_eq!(diag.num_errors(), 2);
        assert!(diag.at_error_limit());
    }
    #[test]
    fn test_defeq_diagnostics_clear() {
        let mut diag = DefEqDiagnostics::new(10);
        diag.error("e1");
        diag.clear();
        assert!(diag.is_clean());
    }
    #[test]
    fn test_defeq_config_value_types() {
        let b = DefEqConfigValue::Bool(true);
        assert_eq!(b.type_name(), "bool");
        assert_eq!(b.as_bool(), Some(true));
        assert_eq!(b.as_int(), None);
        let i = DefEqConfigValue::Int(42);
        assert_eq!(i.type_name(), "int");
        assert_eq!(i.as_int(), Some(42));
        let f = DefEqConfigValue::Float(2.5);
        assert_eq!(f.type_name(), "float");
        assert!((f.as_float().expect("as_float should succeed") - 2.5).abs() < 1e-10);
        let s = DefEqConfigValue::Str("hello".to_string());
        assert_eq!(s.type_name(), "str");
        assert_eq!(s.as_str(), Some("hello"));
        let l = DefEqConfigValue::List(vec!["a".to_string(), "b".to_string()]);
        assert_eq!(l.type_name(), "list");
        assert_eq!(l.as_list().map(|v| v.len()), Some(2));
    }
}
#[cfg(test)]
mod def_eq_ext_tests_2100 {
    use super::*;
    use crate::def_eq::*;
    #[test]
    fn test_def_eq_ext_result_ok_2100() {
        let r = DefEqExtResult2100::Ok("success".to_string());
        assert!(r.is_ok());
        assert!(!r.is_err());
        assert_eq!(r.ok_msg(), Some("success"));
        assert!((r.progress() - 1.0).abs() < 1e-10);
    }
    #[test]
    fn test_def_eq_ext_result_err_2100() {
        let r = DefEqExtResult2100::Err("failure".to_string());
        assert!(r.is_err());
        assert_eq!(r.err_msg(), Some("failure"));
        assert!((r.progress() - 0.0).abs() < 1e-10);
    }
    #[test]
    fn test_def_eq_ext_result_partial_2100() {
        let r = DefEqExtResult2100::Partial { done: 3, total: 10 };
        assert!(r.is_partial());
        assert!((r.progress() - 0.3).abs() < 1e-10);
    }
    #[test]
    fn test_def_eq_ext_result_skipped_2100() {
        let r = DefEqExtResult2100::Skipped;
        assert!(r.is_skipped());
    }
    #[test]
    fn test_def_eq_ext_pass_run_2100() {
        let mut p = DefEqExtPass2100::new("test_pass");
        let r = p.run("hello");
        assert!(r.is_ok());
        assert_eq!(p.total_runs, 1);
        assert_eq!(p.success_count(), 1);
    }
    #[test]
    fn test_def_eq_ext_pass_empty_2100() {
        let mut p = DefEqExtPass2100::new("empty_test");
        let r = p.run("");
        assert!(r.is_err());
        assert_eq!(p.error_count(), 1);
    }
    #[test]
    fn test_def_eq_ext_pass_rate_2100() {
        let mut p = DefEqExtPass2100::new("rate_test");
        p.run("a");
        p.run("b");
        p.run("");
        assert!((p.success_rate() - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_def_eq_ext_pass_disable_2100() {
        let mut p = DefEqExtPass2100::new("disable_test");
        p.disable();
        assert!(!p.enabled);
        p.enable();
        assert!(p.enabled);
    }
    #[test]
    fn test_def_eq_ext_pipeline_basic_2100() {
        let mut pipeline = DefEqExtPipeline2100::new("main_pipeline");
        pipeline.add_pass(DefEqExtPass2100::new("pass1"));
        pipeline.add_pass(DefEqExtPass2100::new("pass2"));
        assert_eq!(pipeline.num_passes(), 2);
        let results = pipeline.run_all("test_input");
        assert_eq!(results.len(), 2);
    }
    #[test]
    fn test_def_eq_ext_pipeline_disabled_2100() {
        let mut pipeline = DefEqExtPipeline2100::new("partial");
        let mut p = DefEqExtPass2100::new("disabled");
        p.disable();
        pipeline.add_pass(p);
        pipeline.add_pass(DefEqExtPass2100::new("enabled"));
        assert_eq!(pipeline.num_enabled_passes(), 1);
        let results = pipeline.run_all("input");
        assert_eq!(results.len(), 1);
    }
    #[test]
    fn test_def_eq_ext_diff_basic_2100() {
        let mut d = DefEqExtDiff2100::new();
        d.add("new_item");
        d.remove("old_item");
        d.keep("same_item");
        assert!(!d.is_empty());
        assert_eq!(d.total_changes(), 2);
        assert_eq!(d.net_additions(), 0);
    }
    #[test]
    fn test_def_eq_ext_config_set_get_2100() {
        let mut cfg = DefEqExtConfig2100::new();
        cfg.set_bool("debug", true);
        cfg.set_int("max_iter", 100);
        cfg.set_str("name", "test");
        assert_eq!(cfg.get_bool("debug"), Some(true));
        assert_eq!(cfg.get_int("max_iter"), Some(100));
        assert_eq!(cfg.get_str("name"), Some("test"));
    }
    #[test]
    fn test_def_eq_ext_config_read_only_2100() {
        let mut cfg = DefEqExtConfig2100::new();
        cfg.set_bool("key", true);
        cfg.lock();
        assert!(!cfg.set_bool("key", false));
        assert_eq!(cfg.get_bool("key"), Some(true));
        cfg.unlock();
        assert!(cfg.set_bool("key", false));
    }
    #[test]
    fn test_def_eq_ext_config_remove_2100() {
        let mut cfg = DefEqExtConfig2100::new();
        cfg.set_int("x", 42);
        assert!(cfg.has("x"));
        cfg.remove("x");
        assert!(!cfg.has("x"));
    }
    #[test]
    fn test_def_eq_ext_diagnostics_basic_2100() {
        let mut diag = DefEqExtDiag2100::new(10);
        diag.error("something went wrong");
        diag.warning("maybe check this");
        diag.note("fyi");
        assert!(diag.has_errors());
        assert!(!diag.is_clean());
        assert_eq!(diag.num_errors(), 1);
        assert_eq!(diag.num_warnings(), 1);
    }
    #[test]
    fn test_def_eq_ext_diagnostics_max_errors_2100() {
        let mut diag = DefEqExtDiag2100::new(2);
        diag.error("e1");
        diag.error("e2");
        diag.error("e3");
        assert_eq!(diag.num_errors(), 2);
        assert!(diag.at_error_limit());
    }
    #[test]
    fn test_def_eq_ext_diagnostics_clear_2100() {
        let mut diag = DefEqExtDiag2100::new(10);
        diag.error("e1");
        diag.clear();
        assert!(diag.is_clean());
    }
    #[test]
    fn test_def_eq_ext_config_value_types_2100() {
        let b = DefEqExtConfigVal2100::Bool(true);
        assert_eq!(b.type_name(), "bool");
        assert_eq!(b.as_bool(), Some(true));
        assert_eq!(b.as_int(), None);
        let i = DefEqExtConfigVal2100::Int(42);
        assert_eq!(i.type_name(), "int");
        assert_eq!(i.as_int(), Some(42));
        let f = DefEqExtConfigVal2100::Float(2.5);
        assert_eq!(f.type_name(), "float");
        assert!((f.as_float().expect("as_float should succeed") - 2.5).abs() < 1e-10);
        let s = DefEqExtConfigVal2100::Str("hello".to_string());
        assert_eq!(s.type_name(), "str");
        assert_eq!(s.as_str(), Some("hello"));
        let l = DefEqExtConfigVal2100::List(vec!["a".to_string(), "b".to_string()]);
        assert_eq!(l.type_name(), "list");
        assert_eq!(l.as_list().map(|v| v.len()), Some(2));
    }
}
