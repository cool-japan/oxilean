//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{
    SimpContext, SimpEngine, SimpEngineAnalysisPass, SimpEngineBuilder, SimpEngineConfig,
    SimpEngineConfigStore, SimpEngineConfigValue, SimpEngineCounterMap, SimpEngineDiagnostics,
    SimpEngineDiff, SimpEngineExtConfig3600, SimpEngineExtConfigVal3600, SimpEngineExtDiag3600,
    SimpEngineExtDiff3600, SimpEngineExtMap, SimpEngineExtPass3600, SimpEngineExtPipeline3600,
    SimpEngineExtResult3600, SimpEngineExtUtil, SimpEnginePipeline, SimpEngineResult,
    SimpEngineStateMachine, SimpEngineWindow, SimpEngineWorkQueue, SimpLemmaDb, SimpLemmaEntry,
    SimpResult, SimpStats,
};
use oxilean_kernel::{BinderInfo, Expr, Literal, Name};

/// Collect an application chain into (head, args).
pub(super) fn collect_app_chain(expr: &Expr) -> (&Expr, Vec<&Expr>) {
    let mut args = Vec::new();
    let mut current = expr;
    while let Expr::App(f, a) = current {
        args.push(a.as_ref());
        current = f;
    }
    args.reverse();
    (current, args)
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::simp_engine::*;
    fn create_nat(n: u64) -> Expr {
        Expr::Lit(Literal::Nat(n))
    }
    fn create_const(name: &str) -> Expr {
        Expr::Const(Name::str(name), vec![])
    }
    fn create_app(f: Expr, arg: Expr) -> Expr {
        Expr::App(Box::new(f), Box::new(arg))
    }
    #[test]
    fn test_simp_engine_creation() {
        let engine = SimpEngine::new();
        assert_eq!(engine.stats.rewrites, 0);
        assert_eq!(engine.stats.congruences, 0);
    }
    #[test]
    fn test_simp_config_default() {
        let config = SimpEngineConfig::default();
        assert_eq!(config.max_steps, 10000);
        assert_eq!(config.max_depth, 100);
        assert!(config.beta_reduce);
    }
    #[test]
    fn test_simp_config_aggressive() {
        let config = SimpEngineConfig::aggressive();
        assert_eq!(config.max_steps, 50000);
        assert_eq!(config.max_depth, 200);
    }
    #[test]
    fn test_simp_config_conservative() {
        let config = SimpEngineConfig::conservative();
        assert_eq!(config.max_steps, 1000);
        assert_eq!(config.max_depth, 50);
        assert!(!config.eta_reduce);
    }
    #[test]
    fn test_simp_result_unchanged() {
        let expr = create_nat(42);
        let result = SimpResult::unchanged(expr.clone());
        assert!(!result.changed);
        assert_eq!(result.steps_used, 0);
    }
    #[test]
    fn test_simp_result_changed() {
        let _expr1 = create_nat(42);
        let expr2 = create_nat(43);
        let result = SimpResult::changed(expr2, create_const("proof"), 1);
        assert!(result.changed);
        assert_eq!(result.steps_used, 1);
    }
    #[test]
    fn test_simp_lemma_db_add_and_find() {
        let mut db = SimpLemmaDb::new();
        let lhs = create_const("P");
        let rhs = create_const("Q");
        let lemma = SimpLemmaEntry::new(create_const("eq"), lhs.clone(), rhs, 10);
        db.add_lemma(Name::str("test"), lemma);
        let candidates = db.find_candidates(&lhs);
        assert_eq!(candidates.len(), 1);
    }
    #[test]
    fn test_simp_lemma_entry_new() {
        let lhs = create_nat(42);
        let rhs = create_nat(42);
        let entry = SimpLemmaEntry::new(create_const("proof"), lhs, rhs, 5);
        assert!(!entry.conditional);
        assert_eq!(entry.priority, 5);
    }
    #[test]
    fn test_simp_lemma_entry_conditional() {
        let lhs = create_nat(42);
        let rhs = create_nat(43);
        let side_cond = create_const("True");
        let entry = SimpLemmaEntry::conditional(create_const("proof"), lhs, rhs, 5, side_cond);
        assert!(entry.conditional);
        assert!(entry.side_condition.is_some());
    }
    #[test]
    fn test_simp_context_creation() {
        let expr = create_nat(42);
        let ctx = SimpContext::new(expr);
        assert_eq!(ctx.depth, 0);
        assert_eq!(ctx.rewrite_count, 0);
    }
    #[test]
    fn test_simp_context_record_visit() {
        let expr = create_nat(42);
        let mut ctx = SimpContext::new(expr);
        assert!(ctx.record_visit("test"));
        assert!(!ctx.record_visit("test"));
    }
    #[test]
    fn test_simp_context_position_tracking() {
        let expr = create_nat(42);
        let mut ctx = SimpContext::new(expr);
        assert_eq!(ctx.position.len(), 0);
        ctx.push_position("pos1".to_string());
        assert_eq!(ctx.position.len(), 1);
        ctx.pop_position();
        assert_eq!(ctx.position.len(), 0);
    }
    #[test]
    fn test_simp_context_depth_tracking() {
        let expr = create_nat(42);
        let mut ctx = SimpContext::new(expr);
        ctx.inc_depth();
        assert_eq!(ctx.depth, 1);
        ctx.dec_depth();
        assert_eq!(ctx.depth, 0);
    }
    #[test]
    fn test_simp_stats_creation() {
        let stats = SimpStats::new();
        assert_eq!(stats.rewrites, 0);
        assert_eq!(stats.congruences, 0);
        assert_eq!(stats.conditional_rewrites, 0);
    }
    #[test]
    fn test_simp_stats_record_rewrite() {
        let mut stats = SimpStats::new();
        stats.record_rewrite();
        assert_eq!(stats.rewrites, 1);
    }
    #[test]
    fn test_simp_stats_record_congruence() {
        let mut stats = SimpStats::new();
        stats.record_congruence();
        assert_eq!(stats.congruences, 1);
    }
    #[test]
    fn test_simp_stats_record_conditional_rewrite() {
        let mut stats = SimpStats::new();
        stats.record_conditional_rewrite();
        assert_eq!(stats.conditional_rewrites, 1);
    }
    #[test]
    fn test_simp_stats_record_cache_hit() {
        let mut stats = SimpStats::new();
        stats.record_cache_hit();
        assert_eq!(stats.cache_hits, 1);
    }
    #[test]
    fn test_simp_stats_update_depth_max() {
        let mut stats = SimpStats::new();
        stats.update_depth_max(5);
        assert_eq!(stats.depth_max, 5);
        stats.update_depth_max(3);
        assert_eq!(stats.depth_max, 5);
        stats.update_depth_max(10);
        assert_eq!(stats.depth_max, 10);
    }
    #[test]
    fn test_simp_engine_with_config() {
        let config = SimpEngineConfig::aggressive();
        let engine = SimpEngine::with_config(config);
        assert_eq!(engine.config.max_steps, 50000);
    }
    #[test]
    fn test_simp_engine_simple_expr() {
        let mut engine = SimpEngine::new();
        let expr = create_nat(42);
        let result = engine.simp(&expr);
        assert_eq!(result.steps_used, 0);
    }
    #[test]
    fn test_simp_engine_add_lemma() {
        let mut engine = SimpEngine::new();
        let lhs = create_const("P");
        let rhs = create_const("Q");
        let entry = SimpLemmaEntry::new(create_const("proof"), lhs, rhs, 10);
        engine.add_lemma(Name::str("test"), entry);
        assert_eq!(engine.lemmas.all_lemmas().len(), 1);
    }
    #[test]
    fn test_simp_engine_cache_hit() {
        let mut engine = SimpEngine::new();
        let expr = create_nat(42);
        let _ = engine.simp(&expr);
        let stats1 = engine.stats.cache_hits;
        let _ = engine.simp(&expr);
        let stats2 = engine.stats.cache_hits;
        assert!(stats2 >= stats1);
    }
    #[test]
    fn test_simp_engine_clear_cache() {
        let mut engine = SimpEngine::new();
        let expr = create_nat(42);
        let _ = engine.simp(&expr);
        engine.clear_cache();
        assert_eq!(engine.cache.len(), 0);
    }
    #[test]
    fn test_expr_match_constants() {
        let engine = SimpEngine::new();
        let e1 = create_const("Nat");
        let e2 = create_const("Nat");
        assert!(engine.exprs_match(&e1, &e2));
    }
    #[test]
    fn test_expr_match_literals() {
        let engine = SimpEngine::new();
        let e1 = create_nat(42);
        let e2 = create_nat(42);
        assert!(engine.exprs_match(&e1, &e2));
    }
    #[test]
    fn test_expr_match_literals_different() {
        let engine = SimpEngine::new();
        let e1 = create_nat(42);
        let e2 = create_nat(43);
        assert!(!engine.exprs_match(&e1, &e2));
    }
    #[test]
    fn test_simp_lemma_db_clear() {
        let mut db = SimpLemmaDb::new();
        let entry = SimpLemmaEntry::new(
            create_const("proof"),
            create_const("P"),
            create_const("Q"),
            10,
        );
        db.add_lemma(Name::str("test"), entry);
        assert_eq!(db.all_lemmas().len(), 1);
        db.clear();
        assert_eq!(db.all_lemmas().len(), 0);
    }
    #[test]
    fn test_simp_engine_depth_limit() {
        let config = SimpEngineConfig {
            max_depth: 0,
            ..SimpEngineConfig::default()
        };
        let mut engine = SimpEngine::with_config(config);
        let expr = create_nat(42);
        let result = engine.simp(&expr);
        assert_eq!(result.steps_used, 0);
    }
    #[test]
    fn test_simp_context_multiple_positions() {
        let expr = create_nat(42);
        let mut ctx = SimpContext::new(expr);
        ctx.push_position("a".to_string());
        ctx.push_position("b".to_string());
        ctx.push_position("c".to_string());
        assert_eq!(ctx.position.len(), 3);
        ctx.pop_position();
        ctx.pop_position();
        ctx.pop_position();
        assert_eq!(ctx.position.len(), 0);
    }
    #[test]
    fn test_simp_engine_stats_default() {
        let engine = SimpEngine::new();
        assert_eq!(engine.stats().rewrites, 0);
        assert_eq!(engine.stats().congruences, 0);
        assert_eq!(engine.stats().conditional_rewrites, 0);
        assert_eq!(engine.stats().cache_hits, 0);
    }
    #[test]
    fn test_simp_engine_multiple_lemmas() {
        let mut engine = SimpEngine::new();
        for i in 0..5 {
            let entry = SimpLemmaEntry::new(
                create_const(&format!("proof{}", i)),
                create_const(&format!("P{}", i)),
                create_const(&format!("Q{}", i)),
                i,
            );
            engine.add_lemma(Name::str(format!("lemma{}", i)), entry);
        }
        assert_eq!(engine.lemmas.all_lemmas().len(), 5);
    }
    #[test]
    fn test_simp_result_proof_field() {
        let expr = create_nat(42);
        let proof = create_const("my_proof");
        let result = SimpResult::changed(expr, proof.clone(), 5);
        match result.proof {
            Expr::Const(ref name, _) => {
                assert_eq!(name.to_string(), "my_proof");
            }
            _ => panic!("Expected const proof"),
        }
    }
    #[test]
    fn test_simp_context_rewrite_count() {
        let expr = create_nat(42);
        let mut ctx = SimpContext::new(expr);
        assert_eq!(ctx.rewrite_count, 0);
        ctx.record_rewrite();
        assert_eq!(ctx.rewrite_count, 1);
        ctx.record_rewrite();
        assert_eq!(ctx.rewrite_count, 2);
    }
    #[test]
    fn test_simp_lemma_db_multiple_adds() {
        let mut db = SimpLemmaDb::new();
        let lhs1 = create_const("P");
        let lhs2 = create_const("Q");
        let rhs = create_const("R");
        db.add_lemma(
            Name::str("l1"),
            SimpLemmaEntry::new(create_const("p1"), lhs1.clone(), rhs.clone(), 5),
        );
        db.add_lemma(
            Name::str("l2"),
            SimpLemmaEntry::new(create_const("p2"), lhs2, rhs, 5),
        );
        assert_eq!(db.all_lemmas().len(), 2);
        let candidates = db.find_candidates(&lhs1);
        assert_eq!(candidates.len(), 1);
    }
    #[test]
    fn test_simp_engine_norm_num_disabled() {
        let config = SimpEngineConfig {
            norm_num: false,
            ..SimpEngineConfig::default()
        };
        let mut engine = SimpEngine::with_config(config);
        let expr = create_nat(42);
        let result = engine.simp(&expr);
        assert!(!result.changed);
    }
    #[test]
    fn test_simp_stats_multiple_operations() {
        let mut stats = SimpStats::new();
        stats.record_rewrite();
        stats.record_rewrite();
        stats.record_congruence();
        stats.record_conditional_rewrite();
        stats.record_cache_hit();
        assert_eq!(stats.rewrites, 2);
        assert_eq!(stats.congruences, 1);
        assert_eq!(stats.conditional_rewrites, 1);
        assert_eq!(stats.cache_hits, 1);
    }
    #[test]
    fn test_discharge_refl_eq() {
        let engine = SimpEngine::new();
        let ctx = SimpContext::new(create_nat(0));
        let eq = Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::App(
                    Box::new(Expr::Const(Name::str("Eq"), vec![])),
                    Box::new(create_const("Nat")),
                )),
                Box::new(create_nat(42)),
            )),
            Box::new(create_nat(42)),
        );
        assert!(engine.discharge_side_condition(&ctx, &eq));
    }
    #[test]
    fn test_discharge_refl_neq_fails() {
        let engine = SimpEngine::new();
        let ctx = SimpContext::new(create_nat(0));
        let eq = Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::App(
                    Box::new(Expr::Const(Name::str("Eq"), vec![])),
                    Box::new(create_const("Nat")),
                )),
                Box::new(create_nat(1)),
            )),
            Box::new(create_nat(2)),
        );
        assert!(!engine.discharge_side_condition(&ctx, &eq));
    }
    #[test]
    fn test_discharge_numeric_lt_true() {
        let engine = SimpEngine::new();
        let ctx = SimpContext::new(create_nat(0));
        let lt = Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("LT.lt"), vec![])),
                Box::new(create_nat(3)),
            )),
            Box::new(create_nat(5)),
        );
        assert!(engine.discharge_side_condition(&ctx, &lt));
    }
    #[test]
    fn test_discharge_numeric_lt_false() {
        let engine = SimpEngine::new();
        let ctx = SimpContext::new(create_nat(0));
        let lt = Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("LT.lt"), vec![])),
                Box::new(create_nat(5)),
            )),
            Box::new(create_nat(3)),
        );
        assert!(!engine.discharge_side_condition(&ctx, &lt));
    }
    #[test]
    fn test_discharge_numeric_le_equal() {
        let engine = SimpEngine::new();
        let ctx = SimpContext::new(create_nat(0));
        let le = Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("LE.le"), vec![])),
                Box::new(create_nat(4)),
            )),
            Box::new(create_nat(4)),
        );
        assert!(engine.discharge_side_condition(&ctx, &le));
    }
    #[test]
    fn test_discharge_propositional_true() {
        let engine = SimpEngine::new();
        let ctx = SimpContext::new(create_nat(0));
        let t = create_const("True");
        assert!(engine.discharge_side_condition(&ctx, &t));
    }
    #[test]
    fn test_discharge_propositional_not_false() {
        let engine = SimpEngine::new();
        let ctx = SimpContext::new(create_nat(0));
        let not_false = Expr::App(
            Box::new(create_const("Not")),
            Box::new(create_const("False")),
        );
        assert!(engine.discharge_side_condition(&ctx, &not_false));
    }
    #[test]
    fn test_discharge_unknown_fails() {
        let engine = SimpEngine::new();
        let ctx = SimpContext::new(create_nat(0));
        let opaque = create_const("SomeOpaqueCondition");
        assert!(!engine.discharge_side_condition(&ctx, &opaque));
    }
    #[test]
    fn test_discharge_ex_falso() {
        let engine = SimpEngine::new();
        let ctx = SimpContext::new(create_nat(0));
        let ex_falso = Expr::Pi(
            oxilean_kernel::BinderInfo::Default,
            Name::str("h"),
            Box::new(create_const("False")),
            Box::new(create_const("P")),
        );
        assert!(engine.discharge_side_condition(&ctx, &ex_falso));
    }
}
#[cfg(test)]
mod simpengine_ext2_tests {
    use super::*;
    use crate::simp_engine::*;
    #[test]
    fn test_simpengine_ext_util_basic() {
        let mut u = SimpEngineExtUtil::new("test");
        u.push(10);
        u.push(20);
        assert_eq!(u.sum(), 30);
        assert_eq!(u.len(), 2);
    }
    #[test]
    fn test_simpengine_ext_util_min_max() {
        let mut u = SimpEngineExtUtil::new("mm");
        u.push(5);
        u.push(1);
        u.push(9);
        assert_eq!(u.min_val(), Some(1));
        assert_eq!(u.max_val(), Some(9));
    }
    #[test]
    fn test_simpengine_ext_util_flags() {
        let mut u = SimpEngineExtUtil::new("flags");
        u.set_flag(3);
        assert!(u.has_flag(3));
        assert!(!u.has_flag(2));
    }
    #[test]
    fn test_simpengine_ext_util_pop() {
        let mut u = SimpEngineExtUtil::new("pop");
        u.push(42);
        assert_eq!(u.pop(), Some(42));
        assert!(u.is_empty());
    }
    #[test]
    fn test_simpengine_ext_map_basic() {
        let mut m: SimpEngineExtMap<i32> = SimpEngineExtMap::new();
        m.insert("key", 42);
        assert_eq!(m.get("key"), Some(&42));
        assert!(m.contains("key"));
        assert!(!m.contains("other"));
    }
    #[test]
    fn test_simpengine_ext_map_get_or_default() {
        let mut m: SimpEngineExtMap<i32> = SimpEngineExtMap::new();
        m.insert("k", 5);
        assert_eq!(m.get_or_default("k"), 5);
        assert_eq!(m.get_or_default("missing"), 0);
    }
    #[test]
    fn test_simpengine_ext_map_keys_sorted() {
        let mut m: SimpEngineExtMap<i32> = SimpEngineExtMap::new();
        m.insert("z", 1);
        m.insert("a", 2);
        m.insert("m", 3);
        let keys = m.keys_sorted();
        assert_eq!(keys[0].as_str(), "a");
        assert_eq!(keys[2].as_str(), "z");
    }
    #[test]
    fn test_simpengine_window_mean() {
        let mut w = SimpEngineWindow::new(3);
        w.push(1.0);
        w.push(2.0);
        w.push(3.0);
        assert!((w.mean() - 2.0).abs() < 1e-10);
    }
    #[test]
    fn test_simpengine_window_evict() {
        let mut w = SimpEngineWindow::new(2);
        w.push(10.0);
        w.push(20.0);
        w.push(30.0);
        assert_eq!(w.len(), 2);
        assert!((w.mean() - 25.0).abs() < 1e-10);
    }
    #[test]
    fn test_simpengine_window_std_dev() {
        let mut w = SimpEngineWindow::new(10);
        for i in 0..10 {
            w.push(i as f64);
        }
        assert!(w.std_dev() > 0.0);
    }
    #[test]
    fn test_simpengine_builder_basic() {
        let b = SimpEngineBuilder::new("test")
            .add_item("a")
            .add_item("b")
            .set_config("key", "val");
        assert_eq!(b.item_count(), 2);
        assert!(b.has_config("key"));
        assert_eq!(b.get_config("key"), Some("val"));
    }
    #[test]
    fn test_simpengine_builder_summary() {
        let b = SimpEngineBuilder::new("suite").add_item("x");
        let s = b.build_summary();
        assert!(s.contains("suite"));
    }
    #[test]
    fn test_simpengine_state_machine_start() {
        let mut sm = SimpEngineStateMachine::new();
        assert!(sm.start());
        assert!(sm.state.is_running());
    }
    #[test]
    fn test_simpengine_state_machine_complete() {
        let mut sm = SimpEngineStateMachine::new();
        sm.start();
        sm.complete();
        assert!(sm.state.is_terminal());
    }
    #[test]
    fn test_simpengine_state_machine_fail() {
        let mut sm = SimpEngineStateMachine::new();
        sm.fail("oops");
        assert!(sm.state.is_terminal());
        assert_eq!(sm.state.error_msg(), Some("oops"));
    }
    #[test]
    fn test_simpengine_state_machine_no_transition_after_terminal() {
        let mut sm = SimpEngineStateMachine::new();
        sm.complete();
        assert!(!sm.start());
    }
    #[test]
    fn test_simpengine_work_queue_basic() {
        let mut wq = SimpEngineWorkQueue::new(10);
        wq.enqueue("task1".to_string());
        wq.enqueue("task2".to_string());
        assert_eq!(wq.pending_count(), 2);
        let t = wq.dequeue();
        assert_eq!(t, Some("task1".to_string()));
        assert_eq!(wq.processed_count(), 1);
    }
    #[test]
    fn test_simpengine_work_queue_capacity() {
        let mut wq = SimpEngineWorkQueue::new(2);
        wq.enqueue("a".to_string());
        wq.enqueue("b".to_string());
        assert!(wq.is_full());
        assert!(!wq.enqueue("c".to_string()));
    }
    #[test]
    fn test_simpengine_counter_map_basic() {
        let mut cm = SimpEngineCounterMap::new();
        cm.increment("apple");
        cm.increment("apple");
        cm.increment("banana");
        assert_eq!(cm.count("apple"), 2);
        assert_eq!(cm.count("banana"), 1);
        assert_eq!(cm.num_unique(), 2);
    }
    #[test]
    fn test_simpengine_counter_map_frequency() {
        let mut cm = SimpEngineCounterMap::new();
        cm.increment("a");
        cm.increment("a");
        cm.increment("b");
        assert!((cm.frequency("a") - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_simpengine_counter_map_most_common() {
        let mut cm = SimpEngineCounterMap::new();
        cm.increment("x");
        cm.increment("y");
        cm.increment("x");
        let (k, v) = cm.most_common().expect("most_common should succeed");
        assert_eq!(k.as_str(), "x");
        assert_eq!(v, 2);
    }
}
#[cfg(test)]
mod simpengine_analysis_tests {
    use super::*;
    use crate::simp_engine::*;
    #[test]
    fn test_simpengine_result_ok() {
        let r = SimpEngineResult::Ok("success".to_string());
        assert!(r.is_ok());
        assert!(!r.is_err());
        assert_eq!(r.ok_msg(), Some("success"));
        assert!((r.progress() - 1.0).abs() < 1e-10);
    }
    #[test]
    fn test_simpengine_result_err() {
        let r = SimpEngineResult::Err("failure".to_string());
        assert!(r.is_err());
        assert_eq!(r.err_msg(), Some("failure"));
        assert!((r.progress() - 0.0).abs() < 1e-10);
    }
    #[test]
    fn test_simpengine_result_partial() {
        let r = SimpEngineResult::Partial { done: 3, total: 10 };
        assert!(r.is_partial());
        assert!((r.progress() - 0.3).abs() < 1e-10);
    }
    #[test]
    fn test_simpengine_result_skipped() {
        let r = SimpEngineResult::Skipped;
        assert!(r.is_skipped());
    }
    #[test]
    fn test_simpengine_analysis_pass_run() {
        let mut p = SimpEngineAnalysisPass::new("test_pass");
        let r = p.run("hello");
        assert!(r.is_ok());
        assert_eq!(p.total_runs, 1);
        assert_eq!(p.success_count(), 1);
    }
    #[test]
    fn test_simpengine_analysis_pass_empty_input() {
        let mut p = SimpEngineAnalysisPass::new("empty_test");
        let r = p.run("");
        assert!(r.is_err());
        assert_eq!(p.error_count(), 1);
    }
    #[test]
    fn test_simpengine_analysis_pass_success_rate() {
        let mut p = SimpEngineAnalysisPass::new("rate_test");
        p.run("a");
        p.run("b");
        p.run("");
        assert!((p.success_rate() - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_simpengine_analysis_pass_disable() {
        let mut p = SimpEngineAnalysisPass::new("disable_test");
        p.disable();
        assert!(!p.enabled);
        p.enable();
        assert!(p.enabled);
    }
    #[test]
    fn test_simpengine_pipeline_basic() {
        let mut pipeline = SimpEnginePipeline::new("main_pipeline");
        pipeline.add_pass(SimpEngineAnalysisPass::new("pass1"));
        pipeline.add_pass(SimpEngineAnalysisPass::new("pass2"));
        assert_eq!(pipeline.num_passes(), 2);
        let results = pipeline.run_all("test_input");
        assert_eq!(results.len(), 2);
    }
    #[test]
    fn test_simpengine_pipeline_disabled_pass() {
        let mut pipeline = SimpEnginePipeline::new("partial");
        let mut p = SimpEngineAnalysisPass::new("disabled");
        p.disable();
        pipeline.add_pass(p);
        pipeline.add_pass(SimpEngineAnalysisPass::new("enabled"));
        assert_eq!(pipeline.num_enabled_passes(), 1);
        let results = pipeline.run_all("input");
        assert_eq!(results.len(), 1);
    }
    #[test]
    fn test_simpengine_diff_basic() {
        let mut d = SimpEngineDiff::new();
        d.add("new_item");
        d.remove("old_item");
        d.keep("same_item");
        assert!(!d.is_empty());
        assert_eq!(d.total_changes(), 2);
        assert_eq!(d.net_additions(), 0);
    }
    #[test]
    fn test_simpengine_diff_summary() {
        let mut d = SimpEngineDiff::new();
        d.add("x");
        d.add("y");
        d.remove("z");
        let s = d.summary();
        assert!(s.contains("+2"));
    }
    #[test]
    fn test_simpengine_config_set_get() {
        let mut cfg = SimpEngineConfigStore::new();
        cfg.set_bool("debug", true);
        cfg.set_int("max_iter", 100);
        cfg.set_str("name", "test");
        assert_eq!(cfg.get_bool("debug"), Some(true));
        assert_eq!(cfg.get_int("max_iter"), Some(100));
        assert_eq!(cfg.get_str("name"), Some("test"));
    }
    #[test]
    fn test_simpengine_config_read_only() {
        let mut cfg = SimpEngineConfigStore::new();
        cfg.set_bool("key", true);
        cfg.lock();
        assert!(!cfg.set_bool("key", false));
        assert_eq!(cfg.get_bool("key"), Some(true));
        cfg.unlock();
        assert!(cfg.set_bool("key", false));
    }
    #[test]
    fn test_simpengine_config_remove() {
        let mut cfg = SimpEngineConfigStore::new();
        cfg.set_int("x", 42);
        assert!(cfg.has("x"));
        cfg.remove("x");
        assert!(!cfg.has("x"));
    }
    #[test]
    fn test_simpengine_diagnostics_basic() {
        let mut diag = SimpEngineDiagnostics::new(10);
        diag.error("something went wrong");
        diag.warning("maybe check this");
        diag.note("fyi");
        assert!(diag.has_errors());
        assert!(!diag.is_clean());
        assert_eq!(diag.num_errors(), 1);
        assert_eq!(diag.num_warnings(), 1);
    }
    #[test]
    fn test_simpengine_diagnostics_max_errors() {
        let mut diag = SimpEngineDiagnostics::new(2);
        diag.error("e1");
        diag.error("e2");
        diag.error("e3");
        assert_eq!(diag.num_errors(), 2);
        assert!(diag.at_error_limit());
    }
    #[test]
    fn test_simpengine_diagnostics_clear() {
        let mut diag = SimpEngineDiagnostics::new(10);
        diag.error("e1");
        diag.clear();
        assert!(diag.is_clean());
    }
    #[test]
    fn test_simpengine_config_value_types() {
        let b = SimpEngineConfigValue::Bool(true);
        assert_eq!(b.type_name(), "bool");
        assert_eq!(b.as_bool(), Some(true));
        assert_eq!(b.as_int(), None);
        let i = SimpEngineConfigValue::Int(42);
        assert_eq!(i.type_name(), "int");
        assert_eq!(i.as_int(), Some(42));
        let f = SimpEngineConfigValue::Float(2.5);
        assert_eq!(f.type_name(), "float");
        assert!((f.as_float().expect("as_float should succeed") - 2.5).abs() < 1e-10);
        let s = SimpEngineConfigValue::Str("hello".to_string());
        assert_eq!(s.type_name(), "str");
        assert_eq!(s.as_str(), Some("hello"));
        let l = SimpEngineConfigValue::List(vec!["a".to_string(), "b".to_string()]);
        assert_eq!(l.type_name(), "list");
        assert_eq!(l.as_list().map(|v| v.len()), Some(2));
    }
}
#[cfg(test)]
mod simp_engine_ext_tests_3600 {
    use super::*;
    use crate::simp_engine::*;
    #[test]
    fn test_simp_engine_ext_result_ok_3600() {
        let r = SimpEngineExtResult3600::Ok("success".to_string());
        assert!(r.is_ok());
        assert!(!r.is_err());
        assert_eq!(r.ok_msg(), Some("success"));
        assert!((r.progress() - 1.0).abs() < 1e-10);
    }
    #[test]
    fn test_simp_engine_ext_result_err_3600() {
        let r = SimpEngineExtResult3600::Err("failure".to_string());
        assert!(r.is_err());
        assert_eq!(r.err_msg(), Some("failure"));
        assert!((r.progress() - 0.0).abs() < 1e-10);
    }
    #[test]
    fn test_simp_engine_ext_result_partial_3600() {
        let r = SimpEngineExtResult3600::Partial { done: 3, total: 10 };
        assert!(r.is_partial());
        assert!((r.progress() - 0.3).abs() < 1e-10);
    }
    #[test]
    fn test_simp_engine_ext_result_skipped_3600() {
        let r = SimpEngineExtResult3600::Skipped;
        assert!(r.is_skipped());
    }
    #[test]
    fn test_simp_engine_ext_pass_run_3600() {
        let mut p = SimpEngineExtPass3600::new("test_pass");
        let r = p.run("hello");
        assert!(r.is_ok());
        assert_eq!(p.total_runs, 1);
        assert_eq!(p.success_count(), 1);
    }
    #[test]
    fn test_simp_engine_ext_pass_empty_3600() {
        let mut p = SimpEngineExtPass3600::new("empty_test");
        let r = p.run("");
        assert!(r.is_err());
        assert_eq!(p.error_count(), 1);
    }
    #[test]
    fn test_simp_engine_ext_pass_rate_3600() {
        let mut p = SimpEngineExtPass3600::new("rate_test");
        p.run("a");
        p.run("b");
        p.run("");
        assert!((p.success_rate() - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_simp_engine_ext_pass_disable_3600() {
        let mut p = SimpEngineExtPass3600::new("disable_test");
        p.disable();
        assert!(!p.enabled);
        p.enable();
        assert!(p.enabled);
    }
    #[test]
    fn test_simp_engine_ext_pipeline_basic_3600() {
        let mut pipeline = SimpEngineExtPipeline3600::new("main_pipeline");
        pipeline.add_pass(SimpEngineExtPass3600::new("pass1"));
        pipeline.add_pass(SimpEngineExtPass3600::new("pass2"));
        assert_eq!(pipeline.num_passes(), 2);
        let results = pipeline.run_all("test_input");
        assert_eq!(results.len(), 2);
    }
    #[test]
    fn test_simp_engine_ext_pipeline_disabled_3600() {
        let mut pipeline = SimpEngineExtPipeline3600::new("partial");
        let mut p = SimpEngineExtPass3600::new("disabled");
        p.disable();
        pipeline.add_pass(p);
        pipeline.add_pass(SimpEngineExtPass3600::new("enabled"));
        assert_eq!(pipeline.num_enabled_passes(), 1);
        let results = pipeline.run_all("input");
        assert_eq!(results.len(), 1);
    }
    #[test]
    fn test_simp_engine_ext_diff_basic_3600() {
        let mut d = SimpEngineExtDiff3600::new();
        d.add("new_item");
        d.remove("old_item");
        d.keep("same_item");
        assert!(!d.is_empty());
        assert_eq!(d.total_changes(), 2);
        assert_eq!(d.net_additions(), 0);
    }
    #[test]
    fn test_simp_engine_ext_config_set_get_3600() {
        let mut cfg = SimpEngineExtConfig3600::new();
        cfg.set_bool("debug", true);
        cfg.set_int("max_iter", 100);
        cfg.set_str("name", "test");
        assert_eq!(cfg.get_bool("debug"), Some(true));
        assert_eq!(cfg.get_int("max_iter"), Some(100));
        assert_eq!(cfg.get_str("name"), Some("test"));
    }
    #[test]
    fn test_simp_engine_ext_config_read_only_3600() {
        let mut cfg = SimpEngineExtConfig3600::new();
        cfg.set_bool("key", true);
        cfg.lock();
        assert!(!cfg.set_bool("key", false));
        assert_eq!(cfg.get_bool("key"), Some(true));
        cfg.unlock();
        assert!(cfg.set_bool("key", false));
    }
    #[test]
    fn test_simp_engine_ext_config_remove_3600() {
        let mut cfg = SimpEngineExtConfig3600::new();
        cfg.set_int("x", 42);
        assert!(cfg.has("x"));
        cfg.remove("x");
        assert!(!cfg.has("x"));
    }
    #[test]
    fn test_simp_engine_ext_diagnostics_basic_3600() {
        let mut diag = SimpEngineExtDiag3600::new(10);
        diag.error("something went wrong");
        diag.warning("maybe check this");
        diag.note("fyi");
        assert!(diag.has_errors());
        assert!(!diag.is_clean());
        assert_eq!(diag.num_errors(), 1);
        assert_eq!(diag.num_warnings(), 1);
    }
    #[test]
    fn test_simp_engine_ext_diagnostics_max_errors_3600() {
        let mut diag = SimpEngineExtDiag3600::new(2);
        diag.error("e1");
        diag.error("e2");
        diag.error("e3");
        assert_eq!(diag.num_errors(), 2);
        assert!(diag.at_error_limit());
    }
    #[test]
    fn test_simp_engine_ext_diagnostics_clear_3600() {
        let mut diag = SimpEngineExtDiag3600::new(10);
        diag.error("e1");
        diag.clear();
        assert!(diag.is_clean());
    }
    #[test]
    fn test_simp_engine_ext_config_value_types_3600() {
        let b = SimpEngineExtConfigVal3600::Bool(true);
        assert_eq!(b.type_name(), "bool");
        assert_eq!(b.as_bool(), Some(true));
        assert_eq!(b.as_int(), None);
        let i = SimpEngineExtConfigVal3600::Int(42);
        assert_eq!(i.type_name(), "int");
        assert_eq!(i.as_int(), Some(42));
        let f = SimpEngineExtConfigVal3600::Float(2.5);
        assert_eq!(f.type_name(), "float");
        assert!((f.as_float().expect("as_float should succeed") - 2.5).abs() < 1e-10);
        let s = SimpEngineExtConfigVal3600::Str("hello".to_string());
        assert_eq!(s.type_name(), "str");
        assert_eq!(s.as_str(), Some("hello"));
        let l = SimpEngineExtConfigVal3600::List(vec!["a".to_string(), "b".to_string()]);
        assert_eq!(l.type_name(), "list");
        assert_eq!(l.as_list().map(|v| v.len()), Some(2));
    }
}
