//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{default_simp_lemmas, SimpConfig, SimpLemma, SimpResult, SimpTheorems};
use oxilean_kernel::{Expr, Name};

use super::simp_types::{
    ModExtConfig800, ModExtConfigVal800, ModExtDiag800, ModExtDiff800, ModExtPass800,
    ModExtPipeline800, ModExtResult800, SimpBudget, SimpContext, SimpLemmaCache, SimpLemmaDatabase,
    SimpLemmaFilter, SimpModBuilder, SimpModCounterMap, SimpModExtMap, SimpModExtUtil,
    SimpModStateMachine, SimpModWindow, SimpModWorkQueue, SimpNormalForm, SimpReport,
    SimpScheduler, SimpStats, SimpTrace, TacticSimpModAnalysisPass, TacticSimpModConfig,
    TacticSimpModConfigValue, TacticSimpModDiagnostics, TacticSimpModDiff, TacticSimpModPipeline,
    TacticSimpModResult,
};
#[cfg(test)]
mod tests {
    use super::*;
    use oxilean_kernel::{Expr, Level, Name};
    fn nat_expr() -> Expr {
        Expr::Const(Name::str("Nat"), vec![])
    }
    fn prop_expr() -> Expr {
        Expr::Sort(Level::zero())
    }
    #[test]
    fn test_simp_stats_default() {
        let s = SimpStats::new();
        assert_eq!(s.rewrites_applied, 0);
        assert!(!s.any_progress());
        assert!(s.all_side_goals_discharged());
    }
    #[test]
    fn test_simp_stats_merge() {
        let mut s1 = SimpStats {
            rewrites_applied: 3,
            beta_steps: 2,
            ..SimpStats::default()
        };
        let s2 = SimpStats {
            rewrites_applied: 1,
            iota_steps: 5,
            ..SimpStats::default()
        };
        s1.merge(&s2);
        assert_eq!(s1.rewrites_applied, 4);
        assert_eq!(s1.iota_steps, 5);
        assert_eq!(s1.beta_steps, 2);
    }
    #[test]
    fn test_simp_stats_any_progress() {
        let s = SimpStats {
            rewrites_applied: 1,
            ..SimpStats::default()
        };
        assert!(s.any_progress());
    }
    #[test]
    fn test_simp_stats_all_side_goals_discharged_false() {
        let s = SimpStats {
            side_goals_failed: 1,
            ..SimpStats::default()
        };
        assert!(!s.all_side_goals_discharged());
    }
    #[test]
    fn test_simp_stats_total_reduction_steps() {
        let s = SimpStats {
            beta_steps: 1,
            eta_steps: 2,
            iota_steps: 3,
            zeta_steps: 4,
            ..SimpStats::default()
        };
        assert_eq!(s.total_reduction_steps(), 10);
    }
    #[test]
    fn test_simp_stats_display() {
        let s = SimpStats {
            rewrites_applied: 5,
            lemmas_tried: 10,
            ..SimpStats::default()
        };
        let display = format!("{}", s);
        assert!(display.contains("rewrites: 5"));
    }
    #[test]
    fn test_simp_context_consume_budget() {
        let config = SimpConfig::default();
        let theorems = SimpTheorems::new();
        let mut ctx = SimpContext::new(&config, &theorems);
        assert!(ctx.has_budget());
        for _ in 0..100 {
            ctx.consume_budget();
        }
        assert_eq!(ctx.budget, config.max_steps - 100);
    }
    #[test]
    fn test_simp_context_exclude() {
        let config = SimpConfig::default();
        let theorems = SimpTheorems::new();
        let mut ctx = SimpContext::new(&config, &theorems);
        let name = Name::str("bad_lemma");
        ctx.exclude(name.clone());
        assert!(ctx.is_excluded(&name));
        assert!(!ctx.is_excluded(&Name::str("good_lemma")));
    }
    #[test]
    fn test_simp_report_unchanged() {
        let r = SimpReport::unchanged(nat_expr());
        assert!(!r.proved);
        assert!(!r.simplified);
        assert!(r.result.is_some());
    }
    #[test]
    fn test_simp_report_simplified() {
        let r = SimpReport::simplified(nat_expr(), SimpStats::new());
        assert!(!r.proved);
        assert!(r.simplified);
    }
    #[test]
    fn test_simp_report_proved() {
        let r = SimpReport::proved(SimpStats::new());
        assert!(r.proved);
        assert!(r.simplified);
        assert!(r.result.is_none());
    }
    #[test]
    fn test_simp_report_record_lemma() {
        let mut r = SimpReport::unchanged(nat_expr());
        r.record_lemma(Name::str("add_comm"));
        r.record_lemma(Name::str("add_comm"));
        assert_eq!(r.lemmas_used.len(), 1);
    }
    #[test]
    fn test_simp_report_display() {
        let r = SimpReport::unchanged(nat_expr());
        let s = format!("{}", r);
        assert!(s.contains("unchanged"));
    }
    #[test]
    fn test_simp_lemma_database_new() {
        let db = SimpLemmaDatabase::new("test");
        assert_eq!(db.label, "test");
        assert_eq!(db.version, 0);
        assert!(db.is_empty());
    }
    #[test]
    fn test_simp_lemma_database_default_db() {
        let db = SimpLemmaDatabase::default_db();
        assert_eq!(db.label, "default");
        assert!(db.version > 0);
    }
    #[test]
    fn test_simp_lemma_database_remove_bumps_version() {
        let mut db = SimpLemmaDatabase::new("test");
        let v0 = db.version;
        db.remove(&Name::str("nonexistent"));
        assert_eq!(db.version, v0 + 1);
    }
    #[test]
    fn test_simp_config_default() {
        let cfg = SimpConfig::default();
        assert!(cfg.beta);
        assert!(cfg.use_default_lemmas);
        assert!(!cfg.simp_hyps);
    }
    #[test]
    fn test_simp_config_only() {
        let cfg = SimpConfig::only();
        assert!(!cfg.use_default_lemmas);
    }
    #[test]
    fn test_simp_context_budget_exhausted() {
        let config = SimpConfig {
            max_steps: 2,
            ..SimpConfig::default()
        };
        let theorems = SimpTheorems::new();
        let mut ctx = SimpContext::new(&config, &theorems);
        assert!(ctx.consume_budget());
        assert!(ctx.consume_budget());
        assert!(!ctx.consume_budget());
        assert!(ctx.stats.budget_exhausted);
    }
    #[test]
    fn test_simp_result_is_simplified() {
        let r = SimpResult::Simplified {
            new_expr: nat_expr(),
            proof: Some(prop_expr()),
        };
        assert!(r.is_simplified());
        assert!(!r.is_proved());
    }
    #[test]
    fn test_simp_result_unchanged() {
        let r = SimpResult::Unchanged;
        assert!(!r.is_simplified());
    }
}
#[cfg(test)]
mod extra_tests {
    use super::*;
    use oxilean_kernel::Level;
    #[test]
    fn test_simp_lemma_filter_all_passes() {
        let f = SimpLemmaFilter::all();
        let n = Name::str("add_comm");
        assert!(f.passes(&n));
    }
    #[test]
    fn test_simp_lemma_filter_prefix() {
        let f = SimpLemmaFilter::with_prefix("add_");
        assert!(f.passes(&Name::str("add_comm")));
        assert!(!f.passes(&Name::str("mul_comm")));
    }
    #[test]
    fn test_simp_trace_enabled_records() {
        let mut t = SimpTrace::enabled();
        t.record(Name::str("add_zero"));
        assert_eq!(t.len(), 1);
    }
    #[test]
    fn test_simp_trace_disabled_does_not_record() {
        let mut t = SimpTrace::new();
        t.record(Name::str("add_zero"));
        assert_eq!(t.len(), 0);
    }
    #[test]
    fn test_simp_trace_clear() {
        let mut t = SimpTrace::enabled();
        t.record(Name::str("x"));
        t.clear();
        assert!(t.is_empty());
    }
    #[test]
    fn test_simp_trace_display() {
        let t = SimpTrace::enabled();
        let s = format!("{}", t);
        assert!(s.contains("SimpTrace"));
    }
    #[test]
    fn test_simp_normal_form_unchanged() {
        let e = Expr::Sort(Level::zero());
        let nf = SimpNormalForm::unchanged(e);
        assert!(!nf.changed);
        assert!(nf.lemmas.is_empty());
    }
    #[test]
    fn test_simp_normal_form_changed() {
        let e = Expr::Sort(Level::zero());
        let nf = SimpNormalForm::changed(e, vec![Name::str("add_comm")]);
        assert!(nf.changed);
        assert_eq!(nf.lemmas.len(), 1);
    }
    #[test]
    fn test_simp_stats_budget_exhausted_merge() {
        let mut s1 = SimpStats::default();
        let s2 = SimpStats {
            budget_exhausted: true,
            ..SimpStats::default()
        };
        s1.merge(&s2);
        assert!(s1.budget_exhausted);
    }
    #[test]
    fn test_simp_context_add_local_lemma() {
        let cfg = SimpConfig::default();
        let theorems = SimpTheorems::new();
        let mut ctx = SimpContext::new(&cfg, &theorems);
        let lemma = SimpLemma {
            name: Name::str("my_lemma"),
            lhs: Expr::Sort(Level::zero()),
            rhs: Expr::Sort(Level::zero()),
            proof: Expr::Sort(Level::zero()),
            is_conditional: false,
            is_forward: true,
            priority: 1000,
        };
        ctx.add_local_lemma(lemma);
        assert_eq!(ctx.local_lemmas.len(), 1);
    }
}
/// Additional builder methods for `SimpConfig`.
pub trait SimpConfigExt {
    /// Enable all reductions.
    fn all_reductions(self) -> SimpConfig;
    /// Disable all reductions (lemma-only mode).
    fn lemma_only(self) -> SimpConfig;
    /// Set a custom max_steps.
    fn with_steps(self, n: u32) -> SimpConfig;
}
impl SimpConfigExt for SimpConfig {
    fn all_reductions(mut self) -> SimpConfig {
        self.beta = true;
        self.eta = true;
        self.iota = true;
        self.zeta = true;
        self
    }
    fn lemma_only(mut self) -> SimpConfig {
        self.beta = false;
        self.eta = false;
        self.iota = false;
        self.zeta = false;
        self
    }
    fn with_steps(mut self, n: u32) -> SimpConfig {
        self.max_steps = n;
        self
    }
}
#[cfg(test)]
mod ext_tests {
    use super::*;
    use oxilean_kernel::Name;
    #[test]
    fn test_simp_config_all_reductions() {
        use super::SimpConfigExt;
        let cfg = SimpConfig::default().all_reductions();
        assert!(cfg.beta);
        assert!(cfg.iota);
    }
    #[test]
    fn test_simp_config_lemma_only() {
        let cfg = SimpConfig::default().lemma_only();
        assert!(!cfg.beta);
        assert!(!cfg.iota);
    }
    #[test]
    fn test_simp_config_with_steps() {
        let cfg = SimpConfig::default().with_steps(999);
        assert_eq!(cfg.max_steps, 999);
    }
    #[test]
    fn test_simp_lemma_cache_record_lookup() {
        let mut c = SimpLemmaCache::new();
        c.record_lookup(&Name::str("add_comm"));
        c.record_lookup(&Name::str("add_comm"));
        assert_eq!(
            *c.lookups
                .get("add_comm")
                .expect("element at \'add_comm\' should exist"),
            2
        );
    }
    #[test]
    fn test_simp_lemma_cache_total() {
        let mut c = SimpLemmaCache::new();
        c.record_lookup(&Name::str("a"));
        c.record_lookup(&Name::str("b"));
        assert_eq!(c.total_lookups(), 2);
    }
    #[test]
    fn test_simp_lemma_cache_hottest() {
        let mut c = SimpLemmaCache::new();
        c.record_lookup(&Name::str("rare"));
        c.record_lookup(&Name::str("hot"));
        c.record_lookup(&Name::str("hot"));
        assert_eq!(c.hottest_lemma(), Some("hot"));
    }
    #[test]
    fn test_simp_lemma_cache_display() {
        let c = SimpLemmaCache::new();
        let s = format!("{}", c);
        assert!(s.contains("SimpLemmaCache"));
    }
    #[test]
    fn test_simp_lemma_database_add_bumps_version() {
        use super::types::SimpLemma;
        use oxilean_kernel::{Expr, Level};
        let mut db = SimpLemmaDatabase::new("test");
        let v0 = db.version;
        let lemma = SimpLemma {
            name: Name::str("test_lemma"),
            lhs: Expr::Sort(Level::zero()),
            rhs: Expr::Sort(Level::zero()),
            proof: Expr::Sort(Level::zero()),
            is_conditional: false,
            is_forward: true,
            priority: 1000,
        };
        db.add(lemma);
        assert_eq!(db.version, v0 + 1);
        assert_eq!(db.len(), 1);
    }
}
#[cfg(test)]
mod scheduler_budget_tests {
    use super::*;
    #[test]
    fn test_simp_scheduler_register_order() {
        let mut s = SimpScheduler::new();
        s.register(Name::str("low"), 10);
        s.register(Name::str("high"), 1000);
        s.register(Name::str("mid"), 500);
        let names: Vec<_> = s.iter_by_priority().collect();
        assert_eq!(names[0], &Name::str("high"));
        assert_eq!(names[2], &Name::str("low"));
    }
    #[test]
    fn test_simp_scheduler_deregister() {
        let mut s = SimpScheduler::new();
        s.register(Name::str("a"), 100);
        s.deregister(&Name::str("a"));
        assert!(s.is_empty());
    }
    #[test]
    fn test_simp_scheduler_top() {
        let mut s = SimpScheduler::new();
        s.register(Name::str("x"), 50);
        s.register(Name::str("y"), 200);
        assert_eq!(s.top(), Some(&Name::str("y")));
    }
    #[test]
    fn test_simp_scheduler_display() {
        let s = SimpScheduler::new();
        let d = format!("{}", s);
        assert!(d.contains("SimpScheduler"));
    }
    #[test]
    fn test_simp_budget_consume() {
        let mut b = SimpBudget::new(10);
        assert!(b.consume(5));
        assert_eq!(b.remaining(), 5);
        assert!(!b.is_exhausted());
    }
    #[test]
    fn test_simp_budget_exhausted() {
        let mut b = SimpBudget::new(3);
        assert!(!b.consume(5));
        assert!(b.is_exhausted());
        assert_eq!(b.remaining(), 0);
    }
    #[test]
    fn test_simp_budget_fraction_used() {
        let mut b = SimpBudget::new(100);
        b.consume(25);
        assert!((b.fraction_used() - 0.25).abs() < 1e-5);
    }
    #[test]
    fn test_simp_budget_used() {
        let mut b = SimpBudget::new(20);
        b.consume(7);
        assert_eq!(b.used(), 7);
    }
    #[test]
    fn test_simp_budget_zero_total() {
        let b = SimpBudget::new(0);
        assert_eq!(b.fraction_used(), 0.0);
    }
    #[test]
    fn test_simp_scheduler_len() {
        let mut s = SimpScheduler::new();
        assert_eq!(s.len(), 0);
        s.register(Name::str("a"), 1);
        s.register(Name::str("b"), 2);
        assert_eq!(s.len(), 2);
    }
}
#[cfg(test)]
mod simpmod_ext2_tests {
    use super::*;
    #[test]
    fn test_simpmod_ext_util_basic() {
        let mut u = SimpModExtUtil::new("test");
        u.push(10);
        u.push(20);
        assert_eq!(u.sum(), 30);
        assert_eq!(u.len(), 2);
    }
    #[test]
    fn test_simpmod_ext_util_min_max() {
        let mut u = SimpModExtUtil::new("mm");
        u.push(5);
        u.push(1);
        u.push(9);
        assert_eq!(u.min_val(), Some(1));
        assert_eq!(u.max_val(), Some(9));
    }
    #[test]
    fn test_simpmod_ext_util_flags() {
        let mut u = SimpModExtUtil::new("flags");
        u.set_flag(3);
        assert!(u.has_flag(3));
        assert!(!u.has_flag(2));
    }
    #[test]
    fn test_simpmod_ext_util_pop() {
        let mut u = SimpModExtUtil::new("pop");
        u.push(42);
        assert_eq!(u.pop(), Some(42));
        assert!(u.is_empty());
    }
    #[test]
    fn test_simpmod_ext_map_basic() {
        let mut m: SimpModExtMap<i32> = SimpModExtMap::new();
        m.insert("key", 42);
        assert_eq!(m.get("key"), Some(&42));
        assert!(m.contains("key"));
        assert!(!m.contains("other"));
    }
    #[test]
    fn test_simpmod_ext_map_get_or_default() {
        let mut m: SimpModExtMap<i32> = SimpModExtMap::new();
        m.insert("k", 5);
        assert_eq!(m.get_or_default("k"), 5);
        assert_eq!(m.get_or_default("missing"), 0);
    }
    #[test]
    fn test_simpmod_ext_map_keys_sorted() {
        let mut m: SimpModExtMap<i32> = SimpModExtMap::new();
        m.insert("z", 1);
        m.insert("a", 2);
        m.insert("m", 3);
        let keys = m.keys_sorted();
        assert_eq!(keys[0].as_str(), "a");
        assert_eq!(keys[2].as_str(), "z");
    }
    #[test]
    fn test_simpmod_window_mean() {
        let mut w = SimpModWindow::new(3);
        w.push(1.0);
        w.push(2.0);
        w.push(3.0);
        assert!((w.mean() - 2.0).abs() < 1e-10);
    }
    #[test]
    fn test_simpmod_window_evict() {
        let mut w = SimpModWindow::new(2);
        w.push(10.0);
        w.push(20.0);
        w.push(30.0);
        assert_eq!(w.len(), 2);
        assert!((w.mean() - 25.0).abs() < 1e-10);
    }
    #[test]
    fn test_simpmod_window_std_dev() {
        let mut w = SimpModWindow::new(10);
        for i in 0..10 {
            w.push(i as f64);
        }
        assert!(w.std_dev() > 0.0);
    }
    #[test]
    fn test_simpmod_builder_basic() {
        let b = SimpModBuilder::new("test")
            .add_item("a")
            .add_item("b")
            .set_config("key", "val");
        assert_eq!(b.item_count(), 2);
        assert!(b.has_config("key"));
        assert_eq!(b.get_config("key"), Some("val"));
    }
    #[test]
    fn test_simpmod_builder_summary() {
        let b = SimpModBuilder::new("suite").add_item("x");
        let s = b.build_summary();
        assert!(s.contains("suite"));
    }
    #[test]
    fn test_simpmod_state_machine_start() {
        let mut sm = SimpModStateMachine::new();
        assert!(sm.start());
        assert!(sm.state.is_running());
    }
    #[test]
    fn test_simpmod_state_machine_complete() {
        let mut sm = SimpModStateMachine::new();
        sm.start();
        sm.complete();
        assert!(sm.state.is_terminal());
    }
    #[test]
    fn test_simpmod_state_machine_fail() {
        let mut sm = SimpModStateMachine::new();
        sm.fail("oops");
        assert!(sm.state.is_terminal());
        assert_eq!(sm.state.error_msg(), Some("oops"));
    }
    #[test]
    fn test_simpmod_state_machine_no_transition_after_terminal() {
        let mut sm = SimpModStateMachine::new();
        sm.complete();
        assert!(!sm.start());
    }
    #[test]
    fn test_simpmod_work_queue_basic() {
        let mut wq = SimpModWorkQueue::new(10);
        wq.enqueue("task1".to_string());
        wq.enqueue("task2".to_string());
        assert_eq!(wq.pending_count(), 2);
        let t = wq.dequeue();
        assert_eq!(t, Some("task1".to_string()));
        assert_eq!(wq.processed_count(), 1);
    }
    #[test]
    fn test_simpmod_work_queue_capacity() {
        let mut wq = SimpModWorkQueue::new(2);
        wq.enqueue("a".to_string());
        wq.enqueue("b".to_string());
        assert!(wq.is_full());
        assert!(!wq.enqueue("c".to_string()));
    }
    #[test]
    fn test_simpmod_counter_map_basic() {
        let mut cm = SimpModCounterMap::new();
        cm.increment("apple");
        cm.increment("apple");
        cm.increment("banana");
        assert_eq!(cm.count("apple"), 2);
        assert_eq!(cm.count("banana"), 1);
        assert_eq!(cm.num_unique(), 2);
    }
    #[test]
    fn test_simpmod_counter_map_frequency() {
        let mut cm = SimpModCounterMap::new();
        cm.increment("a");
        cm.increment("a");
        cm.increment("b");
        assert!((cm.frequency("a") - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_simpmod_counter_map_most_common() {
        let mut cm = SimpModCounterMap::new();
        cm.increment("x");
        cm.increment("y");
        cm.increment("x");
        let (k, v) = cm.most_common().expect("most_common should succeed");
        assert_eq!(k.as_str(), "x");
        assert_eq!(v, 2);
    }
}
#[cfg(test)]
mod tacticsimpmod_analysis_tests {
    use super::*;
    #[test]
    fn test_tacticsimpmod_result_ok() {
        let r = TacticSimpModResult::Ok("success".to_string());
        assert!(r.is_ok());
        assert!(!r.is_err());
        assert_eq!(r.ok_msg(), Some("success"));
        assert!((r.progress() - 1.0).abs() < 1e-10);
    }
    #[test]
    fn test_tacticsimpmod_result_err() {
        let r = TacticSimpModResult::Err("failure".to_string());
        assert!(r.is_err());
        assert_eq!(r.err_msg(), Some("failure"));
        assert!((r.progress() - 0.0).abs() < 1e-10);
    }
    #[test]
    fn test_tacticsimpmod_result_partial() {
        let r = TacticSimpModResult::Partial { done: 3, total: 10 };
        assert!(r.is_partial());
        assert!((r.progress() - 0.3).abs() < 1e-10);
    }
    #[test]
    fn test_tacticsimpmod_result_skipped() {
        let r = TacticSimpModResult::Skipped;
        assert!(r.is_skipped());
    }
    #[test]
    fn test_tacticsimpmod_analysis_pass_run() {
        let mut p = TacticSimpModAnalysisPass::new("test_pass");
        let r = p.run("hello");
        assert!(r.is_ok());
        assert_eq!(p.total_runs, 1);
        assert_eq!(p.success_count(), 1);
    }
    #[test]
    fn test_tacticsimpmod_analysis_pass_empty_input() {
        let mut p = TacticSimpModAnalysisPass::new("empty_test");
        let r = p.run("");
        assert!(r.is_err());
        assert_eq!(p.error_count(), 1);
    }
    #[test]
    fn test_tacticsimpmod_analysis_pass_success_rate() {
        let mut p = TacticSimpModAnalysisPass::new("rate_test");
        p.run("a");
        p.run("b");
        p.run("");
        assert!((p.success_rate() - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_tacticsimpmod_analysis_pass_disable() {
        let mut p = TacticSimpModAnalysisPass::new("disable_test");
        p.disable();
        assert!(!p.enabled);
        p.enable();
        assert!(p.enabled);
    }
    #[test]
    fn test_tacticsimpmod_pipeline_basic() {
        let mut pipeline = TacticSimpModPipeline::new("main_pipeline");
        pipeline.add_pass(TacticSimpModAnalysisPass::new("pass1"));
        pipeline.add_pass(TacticSimpModAnalysisPass::new("pass2"));
        assert_eq!(pipeline.num_passes(), 2);
        let results = pipeline.run_all("test_input");
        assert_eq!(results.len(), 2);
    }
    #[test]
    fn test_tacticsimpmod_pipeline_disabled_pass() {
        let mut pipeline = TacticSimpModPipeline::new("partial");
        let mut p = TacticSimpModAnalysisPass::new("disabled");
        p.disable();
        pipeline.add_pass(p);
        pipeline.add_pass(TacticSimpModAnalysisPass::new("enabled"));
        assert_eq!(pipeline.num_enabled_passes(), 1);
        let results = pipeline.run_all("input");
        assert_eq!(results.len(), 1);
    }
    #[test]
    fn test_tacticsimpmod_diff_basic() {
        let mut d = TacticSimpModDiff::new();
        d.add("new_item");
        d.remove("old_item");
        d.keep("same_item");
        assert!(!d.is_empty());
        assert_eq!(d.total_changes(), 2);
        assert_eq!(d.net_additions(), 0);
    }
    #[test]
    fn test_tacticsimpmod_diff_summary() {
        let mut d = TacticSimpModDiff::new();
        d.add("x");
        d.add("y");
        d.remove("z");
        let s = d.summary();
        assert!(s.contains("+2"));
    }
    #[test]
    fn test_tacticsimpmod_config_set_get() {
        let mut cfg = TacticSimpModConfig::new();
        cfg.set_bool("debug", true);
        cfg.set_int("max_iter", 100);
        cfg.set_str("name", "test");
        assert_eq!(cfg.get_bool("debug"), Some(true));
        assert_eq!(cfg.get_int("max_iter"), Some(100));
        assert_eq!(cfg.get_str("name"), Some("test"));
    }
    #[test]
    fn test_tacticsimpmod_config_read_only() {
        let mut cfg = TacticSimpModConfig::new();
        cfg.set_bool("key", true);
        cfg.lock();
        assert!(!cfg.set_bool("key", false));
        assert_eq!(cfg.get_bool("key"), Some(true));
        cfg.unlock();
        assert!(cfg.set_bool("key", false));
    }
    #[test]
    fn test_tacticsimpmod_config_remove() {
        let mut cfg = TacticSimpModConfig::new();
        cfg.set_int("x", 42);
        assert!(cfg.has("x"));
        cfg.remove("x");
        assert!(!cfg.has("x"));
    }
    #[test]
    fn test_tacticsimpmod_diagnostics_basic() {
        let mut diag = TacticSimpModDiagnostics::new(10);
        diag.error("something went wrong");
        diag.warning("maybe check this");
        diag.note("fyi");
        assert!(diag.has_errors());
        assert!(!diag.is_clean());
        assert_eq!(diag.num_errors(), 1);
        assert_eq!(diag.num_warnings(), 1);
    }
    #[test]
    fn test_tacticsimpmod_diagnostics_max_errors() {
        let mut diag = TacticSimpModDiagnostics::new(2);
        diag.error("e1");
        diag.error("e2");
        diag.error("e3");
        assert_eq!(diag.num_errors(), 2);
        assert!(diag.at_error_limit());
    }
    #[test]
    fn test_tacticsimpmod_diagnostics_clear() {
        let mut diag = TacticSimpModDiagnostics::new(10);
        diag.error("e1");
        diag.clear();
        assert!(diag.is_clean());
    }
    #[test]
    fn test_tacticsimpmod_config_value_types() {
        let b = TacticSimpModConfigValue::Bool(true);
        assert_eq!(b.type_name(), "bool");
        assert_eq!(b.as_bool(), Some(true));
        assert_eq!(b.as_int(), None);
        let i = TacticSimpModConfigValue::Int(42);
        assert_eq!(i.type_name(), "int");
        assert_eq!(i.as_int(), Some(42));
        let f = TacticSimpModConfigValue::Float(2.5);
        assert_eq!(f.type_name(), "float");
        assert!((f.as_float().expect("as_float should succeed") - 2.5).abs() < 1e-10);
        let s = TacticSimpModConfigValue::Str("hello".to_string());
        assert_eq!(s.type_name(), "str");
        assert_eq!(s.as_str(), Some("hello"));
        let l = TacticSimpModConfigValue::List(vec!["a".to_string(), "b".to_string()]);
        assert_eq!(l.type_name(), "list");
        assert_eq!(l.as_list().map(|v| v.len()), Some(2));
    }
}
#[cfg(test)]
mod mod_ext_tests_800 {
    use super::*;
    #[test]
    fn test_mod_ext_result_ok_800() {
        let r = ModExtResult800::Ok("success".to_string());
        assert!(r.is_ok());
        assert!(!r.is_err());
        assert_eq!(r.ok_msg(), Some("success"));
        assert!((r.progress() - 1.0).abs() < 1e-10);
    }
    #[test]
    fn test_mod_ext_result_err_800() {
        let r = ModExtResult800::Err("failure".to_string());
        assert!(r.is_err());
        assert_eq!(r.err_msg(), Some("failure"));
        assert!((r.progress() - 0.0).abs() < 1e-10);
    }
    #[test]
    fn test_mod_ext_result_partial_800() {
        let r = ModExtResult800::Partial { done: 3, total: 10 };
        assert!(r.is_partial());
        assert!((r.progress() - 0.3).abs() < 1e-10);
    }
    #[test]
    fn test_mod_ext_result_skipped_800() {
        let r = ModExtResult800::Skipped;
        assert!(r.is_skipped());
    }
    #[test]
    fn test_mod_ext_pass_run_800() {
        let mut p = ModExtPass800::new("test_pass");
        let r = p.run("hello");
        assert!(r.is_ok());
        assert_eq!(p.total_runs, 1);
        assert_eq!(p.success_count(), 1);
    }
    #[test]
    fn test_mod_ext_pass_empty_800() {
        let mut p = ModExtPass800::new("empty_test");
        let r = p.run("");
        assert!(r.is_err());
        assert_eq!(p.error_count(), 1);
    }
    #[test]
    fn test_mod_ext_pass_rate_800() {
        let mut p = ModExtPass800::new("rate_test");
        p.run("a");
        p.run("b");
        p.run("");
        assert!((p.success_rate() - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_mod_ext_pass_disable_800() {
        let mut p = ModExtPass800::new("disable_test");
        p.disable();
        assert!(!p.enabled);
        p.enable();
        assert!(p.enabled);
    }
    #[test]
    fn test_mod_ext_pipeline_basic_800() {
        let mut pipeline = ModExtPipeline800::new("main_pipeline");
        pipeline.add_pass(ModExtPass800::new("pass1"));
        pipeline.add_pass(ModExtPass800::new("pass2"));
        assert_eq!(pipeline.num_passes(), 2);
        let results = pipeline.run_all("test_input");
        assert_eq!(results.len(), 2);
    }
    #[test]
    fn test_mod_ext_pipeline_disabled_800() {
        let mut pipeline = ModExtPipeline800::new("partial");
        let mut p = ModExtPass800::new("disabled");
        p.disable();
        pipeline.add_pass(p);
        pipeline.add_pass(ModExtPass800::new("enabled"));
        assert_eq!(pipeline.num_enabled_passes(), 1);
        let results = pipeline.run_all("input");
        assert_eq!(results.len(), 1);
    }
    #[test]
    fn test_mod_ext_diff_basic_800() {
        let mut d = ModExtDiff800::new();
        d.add("new_item");
        d.remove("old_item");
        d.keep("same_item");
        assert!(!d.is_empty());
        assert_eq!(d.total_changes(), 2);
        assert_eq!(d.net_additions(), 0);
    }
    #[test]
    fn test_mod_ext_config_set_get_800() {
        let mut cfg = ModExtConfig800::new();
        cfg.set_bool("debug", true);
        cfg.set_int("max_iter", 100);
        cfg.set_str("name", "test");
        assert_eq!(cfg.get_bool("debug"), Some(true));
        assert_eq!(cfg.get_int("max_iter"), Some(100));
        assert_eq!(cfg.get_str("name"), Some("test"));
    }
    #[test]
    fn test_mod_ext_config_read_only_800() {
        let mut cfg = ModExtConfig800::new();
        cfg.set_bool("key", true);
        cfg.lock();
        assert!(!cfg.set_bool("key", false));
        assert_eq!(cfg.get_bool("key"), Some(true));
        cfg.unlock();
        assert!(cfg.set_bool("key", false));
    }
    #[test]
    fn test_mod_ext_config_remove_800() {
        let mut cfg = ModExtConfig800::new();
        cfg.set_int("x", 42);
        assert!(cfg.has("x"));
        cfg.remove("x");
        assert!(!cfg.has("x"));
    }
    #[test]
    fn test_mod_ext_diagnostics_basic_800() {
        let mut diag = ModExtDiag800::new(10);
        diag.error("something went wrong");
        diag.warning("maybe check this");
        diag.note("fyi");
        assert!(diag.has_errors());
        assert!(!diag.is_clean());
        assert_eq!(diag.num_errors(), 1);
        assert_eq!(diag.num_warnings(), 1);
    }
    #[test]
    fn test_mod_ext_diagnostics_max_errors_800() {
        let mut diag = ModExtDiag800::new(2);
        diag.error("e1");
        diag.error("e2");
        diag.error("e3");
        assert_eq!(diag.num_errors(), 2);
        assert!(diag.at_error_limit());
    }
    #[test]
    fn test_mod_ext_diagnostics_clear_800() {
        let mut diag = ModExtDiag800::new(10);
        diag.error("e1");
        diag.clear();
        assert!(diag.is_clean());
    }
    #[test]
    fn test_mod_ext_config_value_types_800() {
        let b = ModExtConfigVal800::Bool(true);
        assert_eq!(b.type_name(), "bool");
        assert_eq!(b.as_bool(), Some(true));
        assert_eq!(b.as_int(), None);
        let i = ModExtConfigVal800::Int(42);
        assert_eq!(i.type_name(), "int");
        assert_eq!(i.as_int(), Some(42));
        let f = ModExtConfigVal800::Float(2.5);
        assert_eq!(f.type_name(), "float");
        assert!((f.as_float().expect("as_float should succeed") - 2.5).abs() < 1e-10);
        let s = ModExtConfigVal800::Str("hello".to_string());
        assert_eq!(s.type_name(), "str");
        assert_eq!(s.as_str(), Some("hello"));
        let l = ModExtConfigVal800::List(vec!["a".to_string(), "b".to_string()]);
        assert_eq!(l.type_name(), "list");
        assert_eq!(l.as_list().map(|v| v.len()), Some(2));
    }
}
