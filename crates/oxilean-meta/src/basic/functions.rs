//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::metacontext_type::MetaContext;
use super::types::{
    BasicAnalysisPass, BasicConfig, BasicConfigValue, BasicDiagnostics, BasicDiff,
    BasicExtConfig3800, BasicExtConfigVal3800, BasicExtDiag3800, BasicExtDiff3800,
    BasicExtPass3800, BasicExtPipeline3800, BasicExtResult3800, BasicPipeline, BasicResult,
    LocalContext, MVarId, MetaBasicBuilder, MetaBasicCounterMap, MetaBasicExtMap, MetaBasicExtUtil,
    MetaBasicStateMachine, MetaBasicWindow, MetaBasicWorkQueue, MetaConfig, MetaStatistics,
    MetaVarPool, MetavarKind, UnificationTrace,
};
use oxilean_kernel::{BinderInfo, ConstantInfo, Environment, Expr, FVarId, Level, Name};

/// Offset added to MVarId to create FVarId placeholders.
/// This allows encoding metavariables as free variables.
/// Offset used to encode metavariable IDs as FVarIds.
pub const MVAR_FVAR_OFFSET: u64 = 1_000_000_000;
/// Abstract a free variable in an expression, replacing it with BVar(idx).
pub fn abstract_fvar_in_expr(expr: &Expr, fvar_id: FVarId, idx: u32) -> Expr {
    match expr {
        Expr::FVar(fid) if *fid == fvar_id => Expr::BVar(idx),
        Expr::FVar(_) | Expr::BVar(_) | Expr::Sort(_) | Expr::Const(_, _) | Expr::Lit(_) => {
            expr.clone()
        }
        Expr::App(f, a) => {
            let f2 = abstract_fvar_in_expr(f, fvar_id, idx);
            let a2 = abstract_fvar_in_expr(a, fvar_id, idx);
            Expr::App(Box::new(f2), Box::new(a2))
        }
        Expr::Lam(info, name, ty, body) => {
            let ty2 = abstract_fvar_in_expr(ty, fvar_id, idx);
            let body2 = abstract_fvar_in_expr(body, fvar_id, idx + 1);
            Expr::Lam(*info, name.clone(), Box::new(ty2), Box::new(body2))
        }
        Expr::Pi(info, name, ty, body) => {
            let ty2 = abstract_fvar_in_expr(ty, fvar_id, idx);
            let body2 = abstract_fvar_in_expr(body, fvar_id, idx + 1);
            Expr::Pi(*info, name.clone(), Box::new(ty2), Box::new(body2))
        }
        Expr::Let(name, ty, val, body) => {
            let ty2 = abstract_fvar_in_expr(ty, fvar_id, idx);
            let val2 = abstract_fvar_in_expr(val, fvar_id, idx);
            let body2 = abstract_fvar_in_expr(body, fvar_id, idx + 1);
            Expr::Let(name.clone(), Box::new(ty2), Box::new(val2), Box::new(body2))
        }
        Expr::Proj(name, pidx, e) => {
            let e2 = abstract_fvar_in_expr(e, fvar_id, idx);
            Expr::Proj(name.clone(), *pidx, Box::new(e2))
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::basic::*;
    use oxilean_kernel::Level;
    fn mk_env() -> Environment {
        Environment::new()
    }
    #[test]
    fn test_create_meta_context() {
        let ctx = MetaContext::new(mk_env());
        assert_eq!(ctx.num_mvars(), 0);
        assert_eq!(ctx.num_locals(), 0);
        assert_eq!(ctx.depth(), 0);
    }
    #[test]
    fn test_fresh_expr_mvar() {
        let mut ctx = MetaContext::new(mk_env());
        let ty = Expr::Sort(Level::zero());
        let (id, placeholder) = ctx.mk_fresh_expr_mvar(ty, MetavarKind::Natural);
        assert_eq!(id, MVarId(0));
        assert!(MetaContext::is_mvar_expr(&placeholder).is_some());
        assert_eq!(ctx.num_mvars(), 1);
    }
    #[test]
    fn test_assign_mvar() {
        let mut ctx = MetaContext::new(mk_env());
        let ty = Expr::Sort(Level::zero());
        let (id, _) = ctx.mk_fresh_expr_mvar(ty, MetavarKind::Natural);
        assert!(!ctx.is_mvar_assigned(id));
        let val = Expr::Lit(oxilean_kernel::Literal::Nat(42));
        assert!(ctx.assign_mvar(id, val.clone()));
        assert!(ctx.is_mvar_assigned(id));
        assert_eq!(ctx.get_mvar_assignment(id), Some(&val));
        assert!(!ctx.assign_mvar(id, Expr::Lit(oxilean_kernel::Literal::Nat(0))));
    }
    #[test]
    fn test_instantiate_mvars() {
        let mut ctx = MetaContext::new(mk_env());
        let ty = Expr::Sort(Level::zero());
        let (id, placeholder) = ctx.mk_fresh_expr_mvar(ty, MetavarKind::Natural);
        let val = Expr::Lit(oxilean_kernel::Literal::Nat(42));
        ctx.assign_mvar(id, val.clone());
        let result = ctx.instantiate_mvars(&placeholder);
        assert_eq!(result, val);
    }
    #[test]
    fn test_instantiate_mvars_in_app() {
        let mut ctx = MetaContext::new(mk_env());
        let ty = Expr::Sort(Level::zero());
        let (id, placeholder) = ctx.mk_fresh_expr_mvar(ty, MetavarKind::Natural);
        let app = Expr::App(
            Box::new(Expr::Const(Name::str("f"), vec![])),
            Box::new(placeholder),
        );
        let val = Expr::Lit(oxilean_kernel::Literal::Nat(42));
        ctx.assign_mvar(id, val.clone());
        let result = ctx.instantiate_mvars(&app);
        let expected = Expr::App(Box::new(Expr::Const(Name::str("f"), vec![])), Box::new(val));
        assert_eq!(result, expected);
    }
    #[test]
    fn test_has_unassigned_mvars() {
        let mut ctx = MetaContext::new(mk_env());
        let ty = Expr::Sort(Level::zero());
        let (id, placeholder) = ctx.mk_fresh_expr_mvar(ty, MetavarKind::Natural);
        assert!(ctx.has_unassigned_mvars(&placeholder));
        ctx.assign_mvar(id, Expr::Lit(oxilean_kernel::Literal::Nat(42)));
        assert!(!ctx.has_unassigned_mvars(&placeholder));
    }
    #[test]
    fn test_local_decl() {
        let mut ctx = MetaContext::new(mk_env());
        let fvar = ctx.mk_local_decl(
            Name::str("x"),
            Expr::Sort(Level::zero()),
            BinderInfo::Default,
        );
        assert!(ctx.find_local_decl(fvar).is_some());
        assert_eq!(ctx.num_locals(), 1);
        assert_eq!(
            ctx.get_fvar_type(fvar).expect("value should be present"),
            &Expr::Sort(Level::zero())
        );
    }
    #[test]
    fn test_let_decl() {
        let mut ctx = MetaContext::new(mk_env());
        let fvar = ctx.mk_let_decl(
            Name::str("x"),
            Expr::Sort(Level::zero()),
            Expr::Lit(oxilean_kernel::Literal::Nat(42)),
        );
        let decl = ctx.find_local_decl(fvar).expect("decl should be present");
        assert!(decl.value.is_some());
        assert_eq!(
            ctx.get_fvar_value(fvar).expect("value should be present"),
            &Expr::Lit(oxilean_kernel::Literal::Nat(42))
        );
    }
    #[test]
    fn test_with_local_decl() {
        let mut ctx = MetaContext::new(mk_env());
        assert_eq!(ctx.num_locals(), 0);
        let result = ctx.with_local_decl(
            Name::str("x"),
            Expr::Sort(Level::zero()),
            BinderInfo::Default,
            |ctx, fvar| {
                assert_eq!(ctx.num_locals(), 1);
                assert!(ctx.find_local_decl(fvar).is_some());
                42
            },
        );
        assert_eq!(result, 42);
        assert_eq!(ctx.num_locals(), 0);
    }
    #[test]
    fn test_save_restore_state() {
        let mut ctx = MetaContext::new(mk_env());
        let ty = Expr::Sort(Level::zero());
        let (id, _) = ctx.mk_fresh_expr_mvar(ty.clone(), MetavarKind::Natural);
        let state = ctx.save_state();
        ctx.assign_mvar(id, Expr::Lit(oxilean_kernel::Literal::Nat(42)));
        let _ = ctx.mk_fresh_expr_mvar(ty, MetavarKind::Natural);
        ctx.postpone(Expr::BVar(0), Expr::BVar(1));
        assert!(ctx.is_mvar_assigned(id));
        assert_eq!(ctx.num_postponed(), 1);
        ctx.restore_state(state);
        assert!(!ctx.is_mvar_assigned(id));
        assert_eq!(ctx.num_postponed(), 0);
    }
    #[test]
    fn test_fresh_level_mvar() {
        let mut ctx = MetaContext::new(mk_env());
        let l1 = ctx.mk_fresh_level_mvar();
        let l2 = ctx.mk_fresh_level_mvar();
        assert_ne!(l1, l2);
    }
    #[test]
    fn test_level_mvar_assignment() {
        let mut ctx = MetaContext::new(mk_env());
        let l = ctx.mk_fresh_level_mvar();
        if let Level::MVar(oxilean_kernel::LevelMVarId(id)) = &l {
            ctx.assign_level_mvar(*id, Level::zero());
            let result = ctx.instantiate_level_mvars(&l);
            assert_eq!(result, Level::zero());
        } else {
            panic!("Expected Level::MVar");
        }
    }
    #[test]
    fn test_unassigned_mvars() {
        let mut ctx = MetaContext::new(mk_env());
        let ty = Expr::Sort(Level::zero());
        let (id1, _) = ctx.mk_fresh_expr_mvar(ty.clone(), MetavarKind::Natural);
        let (id2, _) = ctx.mk_fresh_expr_mvar(ty.clone(), MetavarKind::Natural);
        let (_id3, _) = ctx.mk_fresh_expr_mvar(ty, MetavarKind::Natural);
        ctx.assign_mvar(id1, Expr::BVar(0));
        let unassigned = ctx.unassigned_mvars();
        assert_eq!(unassigned.len(), 2);
        assert!(unassigned.contains(&id2));
    }
    #[test]
    fn test_depth_management() {
        let mut ctx = MetaContext::new(mk_env());
        assert_eq!(ctx.depth(), 0);
        ctx.push_depth();
        assert_eq!(ctx.depth(), 1);
        ctx.push_depth();
        assert_eq!(ctx.depth(), 2);
        ctx.pop_depth();
        assert_eq!(ctx.depth(), 1);
    }
    #[test]
    fn test_mvar_expr_detection() {
        let mvar = Expr::FVar(FVarId::new(MVAR_FVAR_OFFSET + 5));
        assert_eq!(MetaContext::is_mvar_expr(&mvar), Some(MVarId(5)));
        let regular = Expr::FVar(FVarId::new(3));
        assert_eq!(MetaContext::is_mvar_expr(&regular), None);
    }
    #[test]
    fn test_postpone_constraint() {
        let mut ctx = MetaContext::new(mk_env());
        ctx.postpone(Expr::BVar(0), Expr::BVar(1));
        ctx.postpone(Expr::BVar(2), Expr::BVar(3));
        assert_eq!(ctx.num_postponed(), 2);
        ctx.clear_postponed();
        assert_eq!(ctx.num_postponed(), 0);
    }
    #[test]
    fn test_config_default() {
        let config = MetaConfig::default();
        assert!(config.fo_approx);
        assert!(!config.const_approx);
        assert!(config.proof_irrelevance);
    }
    #[test]
    fn test_mk_lambda() {
        let mut ctx = MetaContext::new(mk_env());
        let fvar = ctx.mk_local_decl(
            Name::str("x"),
            Expr::Sort(Level::zero()),
            BinderInfo::Default,
        );
        let body = Expr::FVar(fvar);
        let lam = ctx.mk_lambda(&[fvar], body);
        assert!(matches!(lam, Expr::Lam(BinderInfo::Default, _, _, _)));
    }
    #[test]
    fn test_mk_pi() {
        let mut ctx = MetaContext::new(mk_env());
        let fvar = ctx.mk_local_decl(
            Name::str("x"),
            Expr::Sort(Level::zero()),
            BinderInfo::Implicit,
        );
        let body = Expr::FVar(fvar);
        let pi = ctx.mk_pi(&[fvar], body);
        assert!(matches!(pi, Expr::Pi(BinderInfo::Implicit, _, _, _)));
    }
}
/// Extension methods for `MetaContext`.
pub trait MetaContextExt {
    /// Count total metavariables.
    fn mvar_count(&self) -> usize;
    /// Check if all metavariables are resolved.
    fn is_fully_resolved(&self) -> bool {
        self.unassigned_mvars_ext().is_empty()
    }
    /// Get unassigned mvar IDs (extension version).
    fn unassigned_mvars_ext(&self) -> Vec<MVarId>;
}
#[cfg(test)]
mod basic_extra_tests {
    use super::*;
    use crate::basic::*;
    use oxilean_kernel::{Environment, Expr, Level, Name};
    fn mk_env() -> Environment {
        Environment::new()
    }
    #[test]
    fn test_meta_statistics_from_ctx() {
        let mut ctx = MetaContext::new(mk_env());
        let ty = Expr::Sort(Level::zero());
        let (id, _) = ctx.mk_fresh_expr_mvar(ty.clone(), MetavarKind::Natural);
        let _ = ctx.mk_fresh_expr_mvar(ty.clone(), MetavarKind::Natural);
        ctx.assign_mvar(id, Expr::BVar(0));
        let stats = MetaStatistics::from_ctx(&ctx);
        assert_eq!(stats.total_created, 2);
        assert_eq!(stats.total_assigned, 1);
        assert_eq!(stats.total_pending, 1);
        assert!(!stats.all_resolved());
    }
    #[test]
    fn test_meta_statistics_all_resolved() {
        let ctx = MetaContext::new(mk_env());
        let stats = MetaStatistics::from_ctx(&ctx);
        assert!(stats.all_resolved());
    }
    #[test]
    fn test_meta_statistics_summary() {
        let ctx = MetaContext::new(mk_env());
        let stats = MetaStatistics::from_ctx(&ctx);
        let s = stats.summary();
        assert!(s.contains("MetaStatistics"));
        assert!(s.contains("created=0"));
    }
    #[test]
    fn test_local_context_add_get() {
        let mut lc = LocalContext::new();
        let name = Name::str("h");
        let ty = Expr::Sort(Level::zero());
        lc.add(name.clone(), ty.clone());
        assert_eq!(lc.len(), 1);
        assert!(lc.contains(&name));
        assert_eq!(lc.get(&name), Some(&ty));
    }
    #[test]
    fn test_local_context_remove() {
        let mut lc = LocalContext::new();
        let name = Name::str("h");
        lc.add(name.clone(), Expr::Sort(Level::zero()));
        assert!(lc.remove(&name));
        assert!(lc.is_empty());
        assert!(!lc.remove(&name));
    }
    #[test]
    fn test_local_context_names() {
        let mut lc = LocalContext::new();
        lc.add(Name::str("a"), Expr::Sort(Level::zero()));
        lc.add(Name::str("b"), Expr::Sort(Level::zero()));
        let names = lc.names();
        assert_eq!(names.len(), 2);
    }
    #[test]
    fn test_metavar_pool_sequential() {
        let mut pool = MetaVarPool::new(0);
        assert_eq!(pool.next(), 0);
        assert_eq!(pool.next(), 1);
        assert_eq!(pool.next(), 2);
        assert_eq!(pool.count_issued(), 3);
    }
    #[test]
    fn test_metavar_pool_reserve_batch() {
        let mut pool = MetaVarPool::new(0);
        let (start, size) = pool.reserve_batch();
        assert_eq!(start, 0);
        assert_eq!(size, 64);
        assert_eq!(pool.next(), 64);
    }
    #[test]
    fn test_unification_trace_enabled() {
        let mut trace = UnificationTrace::new();
        trace.record("A", "B", true, 0);
        trace.record("C", "D", false, 1);
        assert_eq!(trace.len(), 2);
        assert_eq!(trace.success_count(), 1);
        assert_eq!(trace.failure_count(), 1);
    }
    #[test]
    fn test_unification_trace_disabled() {
        let mut trace = UnificationTrace::disabled();
        trace.record("A", "B", true, 0);
        assert!(trace.is_empty());
    }
    #[test]
    fn test_unification_trace_clear() {
        let mut trace = UnificationTrace::new();
        trace.record("X", "Y", true, 0);
        trace.clear();
        assert!(trace.is_empty());
    }
    #[test]
    fn test_unification_trace_set_enabled() {
        let mut trace = UnificationTrace::new();
        assert!(trace.is_enabled());
        trace.set_enabled(false);
        assert!(!trace.is_enabled());
        trace.record("A", "B", true, 0);
        assert!(trace.is_empty());
    }
    #[test]
    fn test_meta_context_ext_mvar_count() {
        let mut ctx = MetaContext::new(mk_env());
        let ty = Expr::Sort(Level::zero());
        ctx.mk_fresh_expr_mvar(ty.clone(), MetavarKind::Natural);
        ctx.mk_fresh_expr_mvar(ty, MetavarKind::Natural);
        assert_eq!(ctx.mvar_count(), 2);
    }
    #[test]
    fn test_meta_context_ext_is_fully_resolved() {
        let mut ctx = MetaContext::new(mk_env());
        assert!(ctx.is_fully_resolved());
        let ty = Expr::Sort(Level::zero());
        ctx.mk_fresh_expr_mvar(ty, MetavarKind::Natural);
        assert!(!ctx.is_fully_resolved());
    }
}
#[cfg(test)]
mod metabasic_ext2_tests {
    use super::*;
    use crate::basic::*;
    #[test]
    fn test_metabasic_ext_util_basic() {
        let mut u = MetaBasicExtUtil::new("test");
        u.push(10);
        u.push(20);
        assert_eq!(u.sum(), 30);
        assert_eq!(u.len(), 2);
    }
    #[test]
    fn test_metabasic_ext_util_min_max() {
        let mut u = MetaBasicExtUtil::new("mm");
        u.push(5);
        u.push(1);
        u.push(9);
        assert_eq!(u.min_val(), Some(1));
        assert_eq!(u.max_val(), Some(9));
    }
    #[test]
    fn test_metabasic_ext_util_flags() {
        let mut u = MetaBasicExtUtil::new("flags");
        u.set_flag(3);
        assert!(u.has_flag(3));
        assert!(!u.has_flag(2));
    }
    #[test]
    fn test_metabasic_ext_util_pop() {
        let mut u = MetaBasicExtUtil::new("pop");
        u.push(42);
        assert_eq!(u.pop(), Some(42));
        assert!(u.is_empty());
    }
    #[test]
    fn test_metabasic_ext_map_basic() {
        let mut m: MetaBasicExtMap<i32> = MetaBasicExtMap::new();
        m.insert("key", 42);
        assert_eq!(m.get("key"), Some(&42));
        assert!(m.contains("key"));
        assert!(!m.contains("other"));
    }
    #[test]
    fn test_metabasic_ext_map_get_or_default() {
        let mut m: MetaBasicExtMap<i32> = MetaBasicExtMap::new();
        m.insert("k", 5);
        assert_eq!(m.get_or_default("k"), 5);
        assert_eq!(m.get_or_default("missing"), 0);
    }
    #[test]
    fn test_metabasic_ext_map_keys_sorted() {
        let mut m: MetaBasicExtMap<i32> = MetaBasicExtMap::new();
        m.insert("z", 1);
        m.insert("a", 2);
        m.insert("m", 3);
        let keys = m.keys_sorted();
        assert_eq!(keys[0].as_str(), "a");
        assert_eq!(keys[2].as_str(), "z");
    }
    #[test]
    fn test_metabasic_window_mean() {
        let mut w = MetaBasicWindow::new(3);
        w.push(1.0);
        w.push(2.0);
        w.push(3.0);
        assert!((w.mean() - 2.0).abs() < 1e-10);
    }
    #[test]
    fn test_metabasic_window_evict() {
        let mut w = MetaBasicWindow::new(2);
        w.push(10.0);
        w.push(20.0);
        w.push(30.0);
        assert_eq!(w.len(), 2);
        assert!((w.mean() - 25.0).abs() < 1e-10);
    }
    #[test]
    fn test_metabasic_window_std_dev() {
        let mut w = MetaBasicWindow::new(10);
        for i in 0..10 {
            w.push(i as f64);
        }
        assert!(w.std_dev() > 0.0);
    }
    #[test]
    fn test_metabasic_builder_basic() {
        let b = MetaBasicBuilder::new("test")
            .add_item("a")
            .add_item("b")
            .set_config("key", "val");
        assert_eq!(b.item_count(), 2);
        assert!(b.has_config("key"));
        assert_eq!(b.get_config("key"), Some("val"));
    }
    #[test]
    fn test_metabasic_builder_summary() {
        let b = MetaBasicBuilder::new("suite").add_item("x");
        let s = b.build_summary();
        assert!(s.contains("suite"));
    }
    #[test]
    fn test_metabasic_state_machine_start() {
        let mut sm = MetaBasicStateMachine::new();
        assert!(sm.start());
        assert!(sm.state.is_running());
    }
    #[test]
    fn test_metabasic_state_machine_complete() {
        let mut sm = MetaBasicStateMachine::new();
        sm.start();
        sm.complete();
        assert!(sm.state.is_terminal());
    }
    #[test]
    fn test_metabasic_state_machine_fail() {
        let mut sm = MetaBasicStateMachine::new();
        sm.fail("oops");
        assert!(sm.state.is_terminal());
        assert_eq!(sm.state.error_msg(), Some("oops"));
    }
    #[test]
    fn test_metabasic_state_machine_no_transition_after_terminal() {
        let mut sm = MetaBasicStateMachine::new();
        sm.complete();
        assert!(!sm.start());
    }
    #[test]
    fn test_metabasic_work_queue_basic() {
        let mut wq = MetaBasicWorkQueue::new(10);
        wq.enqueue("task1".to_string());
        wq.enqueue("task2".to_string());
        assert_eq!(wq.pending_count(), 2);
        let t = wq.dequeue();
        assert_eq!(t, Some("task1".to_string()));
        assert_eq!(wq.processed_count(), 1);
    }
    #[test]
    fn test_metabasic_work_queue_capacity() {
        let mut wq = MetaBasicWorkQueue::new(2);
        wq.enqueue("a".to_string());
        wq.enqueue("b".to_string());
        assert!(wq.is_full());
        assert!(!wq.enqueue("c".to_string()));
    }
    #[test]
    fn test_metabasic_counter_map_basic() {
        let mut cm = MetaBasicCounterMap::new();
        cm.increment("apple");
        cm.increment("apple");
        cm.increment("banana");
        assert_eq!(cm.count("apple"), 2);
        assert_eq!(cm.count("banana"), 1);
        assert_eq!(cm.num_unique(), 2);
    }
    #[test]
    fn test_metabasic_counter_map_frequency() {
        let mut cm = MetaBasicCounterMap::new();
        cm.increment("a");
        cm.increment("a");
        cm.increment("b");
        assert!((cm.frequency("a") - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_metabasic_counter_map_most_common() {
        let mut cm = MetaBasicCounterMap::new();
        cm.increment("x");
        cm.increment("y");
        cm.increment("x");
        let (k, v) = cm.most_common().expect("most_common should succeed");
        assert_eq!(k.as_str(), "x");
        assert_eq!(v, 2);
    }
}
#[cfg(test)]
mod basic_analysis_tests {
    use super::*;
    use crate::basic::*;
    #[test]
    fn test_basic_result_ok() {
        let r = BasicResult::Ok("success".to_string());
        assert!(r.is_ok());
        assert!(!r.is_err());
        assert_eq!(r.ok_msg(), Some("success"));
        assert!((r.progress() - 1.0).abs() < 1e-10);
    }
    #[test]
    fn test_basic_result_err() {
        let r = BasicResult::Err("failure".to_string());
        assert!(r.is_err());
        assert_eq!(r.err_msg(), Some("failure"));
        assert!((r.progress() - 0.0).abs() < 1e-10);
    }
    #[test]
    fn test_basic_result_partial() {
        let r = BasicResult::Partial { done: 3, total: 10 };
        assert!(r.is_partial());
        assert!((r.progress() - 0.3).abs() < 1e-10);
    }
    #[test]
    fn test_basic_result_skipped() {
        let r = BasicResult::Skipped;
        assert!(r.is_skipped());
    }
    #[test]
    fn test_basic_analysis_pass_run() {
        let mut p = BasicAnalysisPass::new("test_pass");
        let r = p.run("hello");
        assert!(r.is_ok());
        assert_eq!(p.total_runs, 1);
        assert_eq!(p.success_count(), 1);
    }
    #[test]
    fn test_basic_analysis_pass_empty_input() {
        let mut p = BasicAnalysisPass::new("empty_test");
        let r = p.run("");
        assert!(r.is_err());
        assert_eq!(p.error_count(), 1);
    }
    #[test]
    fn test_basic_analysis_pass_success_rate() {
        let mut p = BasicAnalysisPass::new("rate_test");
        p.run("a");
        p.run("b");
        p.run("");
        assert!((p.success_rate() - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_basic_analysis_pass_disable() {
        let mut p = BasicAnalysisPass::new("disable_test");
        p.disable();
        assert!(!p.enabled);
        p.enable();
        assert!(p.enabled);
    }
    #[test]
    fn test_basic_pipeline_basic() {
        let mut pipeline = BasicPipeline::new("main_pipeline");
        pipeline.add_pass(BasicAnalysisPass::new("pass1"));
        pipeline.add_pass(BasicAnalysisPass::new("pass2"));
        assert_eq!(pipeline.num_passes(), 2);
        let results = pipeline.run_all("test_input");
        assert_eq!(results.len(), 2);
    }
    #[test]
    fn test_basic_pipeline_disabled_pass() {
        let mut pipeline = BasicPipeline::new("partial");
        let mut p = BasicAnalysisPass::new("disabled");
        p.disable();
        pipeline.add_pass(p);
        pipeline.add_pass(BasicAnalysisPass::new("enabled"));
        assert_eq!(pipeline.num_enabled_passes(), 1);
        let results = pipeline.run_all("input");
        assert_eq!(results.len(), 1);
    }
    #[test]
    fn test_basic_diff_basic() {
        let mut d = BasicDiff::new();
        d.add("new_item");
        d.remove("old_item");
        d.keep("same_item");
        assert!(!d.is_empty());
        assert_eq!(d.total_changes(), 2);
        assert_eq!(d.net_additions(), 0);
    }
    #[test]
    fn test_basic_diff_summary() {
        let mut d = BasicDiff::new();
        d.add("x");
        d.add("y");
        d.remove("z");
        let s = d.summary();
        assert!(s.contains("+2"));
    }
    #[test]
    fn test_basic_config_set_get() {
        let mut cfg = BasicConfig::new();
        cfg.set_bool("debug", true);
        cfg.set_int("max_iter", 100);
        cfg.set_str("name", "test");
        assert_eq!(cfg.get_bool("debug"), Some(true));
        assert_eq!(cfg.get_int("max_iter"), Some(100));
        assert_eq!(cfg.get_str("name"), Some("test"));
    }
    #[test]
    fn test_basic_config_read_only() {
        let mut cfg = BasicConfig::new();
        cfg.set_bool("key", true);
        cfg.lock();
        assert!(!cfg.set_bool("key", false));
        assert_eq!(cfg.get_bool("key"), Some(true));
        cfg.unlock();
        assert!(cfg.set_bool("key", false));
    }
    #[test]
    fn test_basic_config_remove() {
        let mut cfg = BasicConfig::new();
        cfg.set_int("x", 42);
        assert!(cfg.has("x"));
        cfg.remove("x");
        assert!(!cfg.has("x"));
    }
    #[test]
    fn test_basic_diagnostics_basic() {
        let mut diag = BasicDiagnostics::new(10);
        diag.error("something went wrong");
        diag.warning("maybe check this");
        diag.note("fyi");
        assert!(diag.has_errors());
        assert!(!diag.is_clean());
        assert_eq!(diag.num_errors(), 1);
        assert_eq!(diag.num_warnings(), 1);
    }
    #[test]
    fn test_basic_diagnostics_max_errors() {
        let mut diag = BasicDiagnostics::new(2);
        diag.error("e1");
        diag.error("e2");
        diag.error("e3");
        assert_eq!(diag.num_errors(), 2);
        assert!(diag.at_error_limit());
    }
    #[test]
    fn test_basic_diagnostics_clear() {
        let mut diag = BasicDiagnostics::new(10);
        diag.error("e1");
        diag.clear();
        assert!(diag.is_clean());
    }
    #[test]
    fn test_basic_config_value_types() {
        let b = BasicConfigValue::Bool(true);
        assert_eq!(b.type_name(), "bool");
        assert_eq!(b.as_bool(), Some(true));
        assert_eq!(b.as_int(), None);
        let i = BasicConfigValue::Int(42);
        assert_eq!(i.type_name(), "int");
        assert_eq!(i.as_int(), Some(42));
        let f = BasicConfigValue::Float(2.5);
        assert_eq!(f.type_name(), "float");
        assert!((f.as_float().expect("as_float should succeed") - 2.5).abs() < 1e-10);
        let s = BasicConfigValue::Str("hello".to_string());
        assert_eq!(s.type_name(), "str");
        assert_eq!(s.as_str(), Some("hello"));
        let l = BasicConfigValue::List(vec!["a".to_string(), "b".to_string()]);
        assert_eq!(l.type_name(), "list");
        assert_eq!(l.as_list().map(|v| v.len()), Some(2));
    }
}
#[cfg(test)]
mod basic_ext_tests_3800 {
    use super::*;
    use crate::basic::*;
    #[test]
    fn test_basic_ext_result_ok_3800() {
        let r = BasicExtResult3800::Ok("success".to_string());
        assert!(r.is_ok());
        assert!(!r.is_err());
        assert_eq!(r.ok_msg(), Some("success"));
        assert!((r.progress() - 1.0).abs() < 1e-10);
    }
    #[test]
    fn test_basic_ext_result_err_3800() {
        let r = BasicExtResult3800::Err("failure".to_string());
        assert!(r.is_err());
        assert_eq!(r.err_msg(), Some("failure"));
        assert!((r.progress() - 0.0).abs() < 1e-10);
    }
    #[test]
    fn test_basic_ext_result_partial_3800() {
        let r = BasicExtResult3800::Partial { done: 3, total: 10 };
        assert!(r.is_partial());
        assert!((r.progress() - 0.3).abs() < 1e-10);
    }
    #[test]
    fn test_basic_ext_result_skipped_3800() {
        let r = BasicExtResult3800::Skipped;
        assert!(r.is_skipped());
    }
    #[test]
    fn test_basic_ext_pass_run_3800() {
        let mut p = BasicExtPass3800::new("test_pass");
        let r = p.run("hello");
        assert!(r.is_ok());
        assert_eq!(p.total_runs, 1);
        assert_eq!(p.success_count(), 1);
    }
    #[test]
    fn test_basic_ext_pass_empty_3800() {
        let mut p = BasicExtPass3800::new("empty_test");
        let r = p.run("");
        assert!(r.is_err());
        assert_eq!(p.error_count(), 1);
    }
    #[test]
    fn test_basic_ext_pass_rate_3800() {
        let mut p = BasicExtPass3800::new("rate_test");
        p.run("a");
        p.run("b");
        p.run("");
        assert!((p.success_rate() - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_basic_ext_pass_disable_3800() {
        let mut p = BasicExtPass3800::new("disable_test");
        p.disable();
        assert!(!p.enabled);
        p.enable();
        assert!(p.enabled);
    }
    #[test]
    fn test_basic_ext_pipeline_basic_3800() {
        let mut pipeline = BasicExtPipeline3800::new("main_pipeline");
        pipeline.add_pass(BasicExtPass3800::new("pass1"));
        pipeline.add_pass(BasicExtPass3800::new("pass2"));
        assert_eq!(pipeline.num_passes(), 2);
        let results = pipeline.run_all("test_input");
        assert_eq!(results.len(), 2);
    }
    #[test]
    fn test_basic_ext_pipeline_disabled_3800() {
        let mut pipeline = BasicExtPipeline3800::new("partial");
        let mut p = BasicExtPass3800::new("disabled");
        p.disable();
        pipeline.add_pass(p);
        pipeline.add_pass(BasicExtPass3800::new("enabled"));
        assert_eq!(pipeline.num_enabled_passes(), 1);
        let results = pipeline.run_all("input");
        assert_eq!(results.len(), 1);
    }
    #[test]
    fn test_basic_ext_diff_basic_3800() {
        let mut d = BasicExtDiff3800::new();
        d.add("new_item");
        d.remove("old_item");
        d.keep("same_item");
        assert!(!d.is_empty());
        assert_eq!(d.total_changes(), 2);
        assert_eq!(d.net_additions(), 0);
    }
    #[test]
    fn test_basic_ext_config_set_get_3800() {
        let mut cfg = BasicExtConfig3800::new();
        cfg.set_bool("debug", true);
        cfg.set_int("max_iter", 100);
        cfg.set_str("name", "test");
        assert_eq!(cfg.get_bool("debug"), Some(true));
        assert_eq!(cfg.get_int("max_iter"), Some(100));
        assert_eq!(cfg.get_str("name"), Some("test"));
    }
    #[test]
    fn test_basic_ext_config_read_only_3800() {
        let mut cfg = BasicExtConfig3800::new();
        cfg.set_bool("key", true);
        cfg.lock();
        assert!(!cfg.set_bool("key", false));
        assert_eq!(cfg.get_bool("key"), Some(true));
        cfg.unlock();
        assert!(cfg.set_bool("key", false));
    }
    #[test]
    fn test_basic_ext_config_remove_3800() {
        let mut cfg = BasicExtConfig3800::new();
        cfg.set_int("x", 42);
        assert!(cfg.has("x"));
        cfg.remove("x");
        assert!(!cfg.has("x"));
    }
    #[test]
    fn test_basic_ext_diagnostics_basic_3800() {
        let mut diag = BasicExtDiag3800::new(10);
        diag.error("something went wrong");
        diag.warning("maybe check this");
        diag.note("fyi");
        assert!(diag.has_errors());
        assert!(!diag.is_clean());
        assert_eq!(diag.num_errors(), 1);
        assert_eq!(diag.num_warnings(), 1);
    }
    #[test]
    fn test_basic_ext_diagnostics_max_errors_3800() {
        let mut diag = BasicExtDiag3800::new(2);
        diag.error("e1");
        diag.error("e2");
        diag.error("e3");
        assert_eq!(diag.num_errors(), 2);
        assert!(diag.at_error_limit());
    }
    #[test]
    fn test_basic_ext_diagnostics_clear_3800() {
        let mut diag = BasicExtDiag3800::new(10);
        diag.error("e1");
        diag.clear();
        assert!(diag.is_clean());
    }
    #[test]
    fn test_basic_ext_config_value_types_3800() {
        let b = BasicExtConfigVal3800::Bool(true);
        assert_eq!(b.type_name(), "bool");
        assert_eq!(b.as_bool(), Some(true));
        assert_eq!(b.as_int(), None);
        let i = BasicExtConfigVal3800::Int(42);
        assert_eq!(i.type_name(), "int");
        assert_eq!(i.as_int(), Some(42));
        let f = BasicExtConfigVal3800::Float(2.5);
        assert_eq!(f.type_name(), "float");
        assert!((f.as_float().expect("as_float should succeed") - 2.5).abs() < 1e-10);
        let s = BasicExtConfigVal3800::Str("hello".to_string());
        assert_eq!(s.type_name(), "str");
        assert_eq!(s.as_str(), Some("hello"));
        let l = BasicExtConfigVal3800::List(vec!["a".to_string(), "b".to_string()]);
        assert_eq!(l.type_name(), "list");
        assert_eq!(l.as_list().map(|v| v.len()), Some(2));
    }
}
