//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::basic::MetaContext;
use super::tactic::TacticState;
use super::types::{
    LibAnalysisPass, LibConfig, LibConfigValue, LibDiagnostics, LibDiff, LibExtConfig1300,
    LibExtConfigVal1300, LibExtDiag1300, LibExtDiff1300, LibExtPass1300, LibExtPipeline1300,
    LibExtResult1300, LibPipeline, LibResult, MetaCache, MetaFeatures, MetaLibBuilder,
    MetaLibCounterMap, MetaLibExtMap, MetaLibExtUtil, MetaLibStateMachine, MetaLibWindow,
    MetaLibWorkQueue, MetaStats, PerfStats, ProofStateReport, ScoredCandidate, TacticGroup,
    TacticRegistry,
};
/// Version of the meta layer.
pub const META_VERSION: &str = "0.9.0-alpha";
/// Return the meta layer version string.
pub fn meta_version_str() -> &'static str {
    META_VERSION
}
/// Helper to create a minimal `MetaContext` for testing.
pub fn mk_test_ctx() -> MetaContext {
    MetaContext::new(oxilean_kernel::Environment::new())
}
/// Build a default tactic registry.
pub fn default_tactic_registry() -> TacticRegistry {
    let mut reg = TacticRegistry::new();
    for tac in [
        "intro",
        "intros",
        "exact",
        "assumption",
        "refl",
        "trivial",
        "sorry",
        "apply",
        "cases",
        "induction",
        "rw",
        "rewrite",
        "simp",
        "simp_only",
        "have",
        "show",
        "obtain",
        "use",
        "exists",
        "constructor",
        "left",
        "right",
        "split",
        "exfalso",
        "clear",
        "revert",
        "subst",
        "rename",
        "ring",
        "linarith",
        "omega",
        "norm_num",
        "push_neg",
        "by_contra",
        "by_contradiction",
        "contrapose",
        "field_simp",
        "simp_all",
        "rfl",
        "all_goals",
        "first",
        "repeat",
        "try",
    ] {
        reg.register(tac);
    }
    reg
}
/// Check if a tactic name is a core tactic.
pub fn is_core_tactic(name: &str) -> bool {
    matches!(
        name,
        "intro"
            | "intros"
            | "exact"
            | "assumption"
            | "refl"
            | "trivial"
            | "sorry"
            | "apply"
            | "cases"
            | "induction"
            | "rw"
            | "rewrite"
            | "simp"
            | "have"
            | "show"
            | "obtain"
            | "use"
            | "constructor"
            | "left"
            | "right"
            | "split"
            | "exfalso"
            | "clear"
            | "revert"
            | "subst"
            | "rename"
            | "ring"
            | "linarith"
            | "omega"
            | "push_neg"
            | "by_contra"
    )
}
/// Check if a tactic is an automation tactic.
pub fn is_automation_tactic(name: &str) -> bool {
    matches!(
        name,
        "simp"
            | "simp_all"
            | "omega"
            | "linarith"
            | "ring"
            | "norm_num"
            | "decide"
            | "trivial"
            | "tauto"
            | "aesop"
            | "field_simp"
    )
}
/// Describe a tactic's purpose.
pub fn tactic_description(name: &str) -> Option<&'static str> {
    match name {
        "intro" | "intros" => Some("Introduce binders from the goal"),
        "exact" => Some("Close the goal with a proof term"),
        "apply" => Some("Apply a lemma to the goal"),
        "assumption" => Some("Close goal using a hypothesis"),
        "refl" | "rfl" => Some("Close a reflexivity goal"),
        "cases" => Some("Case-split on an inductive type"),
        "induction" => Some("Induct on an inductive type"),
        "rw" | "rewrite" => Some("Rewrite the goal using an equation"),
        "simp" => Some("Simplify using simp lemmas"),
        "have" => Some("Introduce a local lemma"),
        "split" => Some("Split a conjunction or iff goal"),
        "sorry" => Some("Close goal with sorry (unsound)"),
        _ => None,
    }
}
/// Collect statistics from a `MetaContext`.
pub fn collect_meta_stats(ctx: &MetaContext) -> MetaStats {
    MetaStats {
        num_expr_mvars: ctx.num_mvars(),
        num_assigned_expr: 0,
        num_level_mvars: 0,
        num_assigned_levels: 0,
        num_postponed: ctx.num_postponed(),
    }
}
/// Sort candidates by descending score.
pub fn sort_candidates<T: Clone>(candidates: &mut [ScoredCandidate<T>]) {
    candidates.sort_by_key(|k| std::cmp::Reverse(k.score));
}
#[cfg(test)]
mod meta_lib_tests {
    use super::*;
    #[test]
    fn test_meta_version_str() {
        assert!(!meta_version_str().is_empty());
    }
    #[test]
    fn test_tactic_registry_register() {
        let mut reg = TacticRegistry::new();
        let idx = reg.register("intro");
        assert_eq!(reg.lookup("intro"), Some(idx));
        assert_eq!(reg.lookup("nonexistent"), None);
    }
    #[test]
    fn test_tactic_registry_idempotent() {
        let mut reg = TacticRegistry::new();
        assert_eq!(reg.register("intro"), reg.register("intro"));
    }
    #[test]
    fn test_tactic_registry_name_of() {
        let mut reg = TacticRegistry::new();
        let idx = reg.register("apply");
        assert_eq!(reg.name_of(idx), Some("apply"));
        assert_eq!(reg.name_of(999), None);
    }
    #[test]
    fn test_default_tactic_registry() {
        let reg = default_tactic_registry();
        assert!(reg.len() > 10);
        assert!(reg.lookup("intro").is_some());
    }
    #[test]
    fn test_is_core_tactic() {
        assert!(is_core_tactic("intro"));
        assert!(!is_core_tactic("nonexistent"));
    }
    #[test]
    fn test_is_automation_tactic() {
        assert!(is_automation_tactic("simp"));
        assert!(!is_automation_tactic("intro"));
    }
    #[test]
    fn test_tactic_description() {
        assert!(tactic_description("intro").is_some());
        assert_eq!(tactic_description("nonexistent_xyz"), None);
    }
    #[test]
    fn test_meta_cache_basic() {
        let mut cache: MetaCache<String, i32> = MetaCache::with_capacity(10);
        cache.insert("key".into(), 42);
        assert_eq!(cache.get(&"key".to_string()), Some(&42));
        assert_eq!(cache.get(&"missing".to_string()), None);
    }
    #[test]
    fn test_meta_cache_hit_rate() {
        let mut cache: MetaCache<String, i32> = MetaCache::with_capacity(10);
        cache.insert("key".into(), 1);
        let _ = cache.get(&"key".to_string());
        let _ = cache.get(&"miss".to_string());
        assert!((cache.hit_rate() - 0.5).abs() < 1e-9);
    }
    #[test]
    fn test_meta_cache_clear() {
        let mut cache: MetaCache<String, i32> = MetaCache::with_capacity(10);
        cache.insert("a".into(), 1);
        cache.clear();
        assert!(cache.is_empty());
    }
    #[test]
    fn test_scored_candidate() {
        let c = ScoredCandidate::new("lemma", 100i64);
        assert_eq!(c.candidate, "lemma");
    }
    #[test]
    fn test_sort_candidates() {
        let mut v = vec![
            ScoredCandidate::new("a", 1i64),
            ScoredCandidate::new("b", 3i64),
            ScoredCandidate::new("c", 2i64),
        ];
        sort_candidates(&mut v);
        assert_eq!(v[0].candidate, "b");
    }
    #[test]
    fn test_collect_meta_stats() {
        let ctx = mk_test_ctx();
        let stats = collect_meta_stats(&ctx);
        assert_eq!(stats.num_expr_mvars, 0);
    }
    #[test]
    fn test_proof_state_report() {
        let mut ctx = mk_test_ctx();
        let goal_ty = oxilean_kernel::Expr::Const(oxilean_kernel::Name::str("P"), vec![]);
        let (mvar_id, _) = ctx.mk_fresh_expr_mvar(goal_ty, crate::basic::MetavarKind::Natural);
        let state = TacticState::single(mvar_id);
        let report = ProofStateReport::from_state(&state);
        assert_eq!(report.open_goals, 1);
        assert!(!report.is_complete);
    }
    #[test]
    fn test_tactic_registry_all_names() {
        let mut reg = TacticRegistry::new();
        reg.register("a");
        reg.register("b");
        assert_eq!(reg.all_names().len(), 2);
    }
    #[test]
    fn test_mk_test_ctx() {
        let ctx = mk_test_ctx();
        assert_eq!(ctx.num_mvars(), 0);
    }
}
#[cfg(test)]
mod perf_stats_tests {
    use super::*;
    #[test]
    fn test_perf_stats_empty() {
        let s = PerfStats::new();
        assert_eq!(s.elab_attempts, 0);
        assert!((s.elab_success_rate() - 0.0).abs() < 1e-9);
    }
    #[test]
    fn test_perf_stats_success_rate() {
        let mut s = PerfStats::new();
        s.elab_attempts = 10;
        s.elab_successes = 7;
        assert!((s.elab_success_rate() - 0.7).abs() < 1e-9);
    }
    #[test]
    fn test_perf_stats_merge() {
        let mut a = PerfStats::new();
        a.elab_attempts = 5;
        let b = PerfStats::new();
        a.merge(&b);
        assert_eq!(a.elab_attempts, 5);
    }
    #[test]
    fn test_perf_stats_unif_success_rate_no_attempts() {
        let s = PerfStats::new();
        assert!((s.unif_success_rate() - 0.0).abs() < 1e-9);
    }
    #[test]
    fn test_perf_stats_merge_sums() {
        let mut a = PerfStats::new();
        a.elab_attempts = 5;
        a.elab_successes = 3;
        let mut b = PerfStats::new();
        b.elab_attempts = 3;
        b.elab_successes = 2;
        a.merge(&b);
        assert_eq!(a.elab_attempts, 8);
        assert_eq!(a.elab_successes, 5);
    }
    #[test]
    fn test_perf_stats_elapsed() {
        let mut s = PerfStats::new();
        s.elapsed_us = 1000;
        assert_eq!(s.elapsed_us, 1000);
    }
    #[test]
    fn test_perf_stats_default() {
        let s = PerfStats::default();
        assert_eq!(s.elab_attempts, 0);
    }
    #[test]
    fn test_perf_stats_unif() {
        let mut s = PerfStats::new();
        s.unif_attempts = 4;
        s.unif_successes = 3;
        assert!((s.unif_success_rate() - 0.75).abs() < 1e-9);
    }
}
/// Return the standard tactic groups.
pub fn standard_tactic_groups() -> Vec<TacticGroup> {
    vec![
        TacticGroup::new("introduction", "Tactics that introduce hypotheses")
            .add("intro")
            .add("intros"),
        TacticGroup::new("closing", "Tactics that close goals")
            .add("exact")
            .add("assumption")
            .add("refl")
            .add("rfl")
            .add("trivial")
            .add("sorry"),
        TacticGroup::new("rewriting", "Tactics that rewrite the goal")
            .add("rw")
            .add("rewrite")
            .add("simp")
            .add("simp_all"),
        TacticGroup::new("structural", "Tactics that split/analyze goals")
            .add("cases")
            .add("induction")
            .add("split")
            .add("constructor")
            .add("left")
            .add("right"),
        TacticGroup::new("automation", "Automated solving tactics")
            .add("omega")
            .add("linarith")
            .add("ring")
            .add("norm_num")
            .add("decide"),
    ]
}
/// Find the group that a tactic belongs to.
pub fn tactic_group_for(tactic: &str) -> Option<&'static str> {
    match tactic {
        "intro" | "intros" => Some("introduction"),
        "exact" | "assumption" | "refl" | "rfl" | "trivial" | "sorry" => Some("closing"),
        "rw" | "rewrite" | "simp" | "simp_all" => Some("rewriting"),
        "cases" | "induction" | "split" | "constructor" | "left" | "right" => Some("structural"),
        "omega" | "linarith" | "ring" | "norm_num" | "decide" => Some("automation"),
        _ => None,
    }
}
#[cfg(test)]
mod meta_features_tests {
    use super::*;
    #[test]
    fn test_meta_features_default() {
        let f = MetaFeatures::default();
        assert!(f.discr_tree);
        assert!(f.instance_synth);
        assert!(!f.proof_recording);
    }
    #[test]
    fn test_meta_features_all_enabled() {
        let f = MetaFeatures::all_enabled();
        assert!(f.proof_recording);
        assert!(f.whnf_cache);
    }
    #[test]
    fn test_meta_features_minimal() {
        let f = MetaFeatures::minimal();
        assert!(!f.discr_tree);
        assert!(!f.instance_synth);
    }
    #[test]
    fn test_meta_features_any_caching_default() {
        let f = MetaFeatures::default();
        assert!(f.any_caching());
    }
    #[test]
    fn test_meta_features_any_caching_minimal() {
        let f = MetaFeatures::minimal();
        assert!(!f.any_caching());
    }
    #[test]
    fn test_tactic_group_contains() {
        let g = TacticGroup::new("test", "desc").add("intro").add("intros");
        assert!(g.contains("intro"));
        assert!(!g.contains("exact"));
    }
    #[test]
    fn test_standard_tactic_groups_nonempty() {
        let groups = standard_tactic_groups();
        assert!(!groups.is_empty());
    }
    #[test]
    fn test_tactic_group_for_intro() {
        assert_eq!(tactic_group_for("intro"), Some("introduction"));
    }
    #[test]
    fn test_tactic_group_for_exact() {
        assert_eq!(tactic_group_for("exact"), Some("closing"));
    }
    #[test]
    fn test_tactic_group_for_unknown() {
        assert_eq!(tactic_group_for("foobar_nonexistent"), None);
    }
    #[test]
    fn test_tactic_group_for_omega() {
        assert_eq!(tactic_group_for("omega"), Some("automation"));
    }
}
#[cfg(test)]
mod metalib_ext2_tests {
    use super::*;
    #[test]
    fn test_metalib_ext_util_basic() {
        let mut u = MetaLibExtUtil::new("test");
        u.push(10);
        u.push(20);
        assert_eq!(u.sum(), 30);
        assert_eq!(u.len(), 2);
    }
    #[test]
    fn test_metalib_ext_util_min_max() {
        let mut u = MetaLibExtUtil::new("mm");
        u.push(5);
        u.push(1);
        u.push(9);
        assert_eq!(u.min_val(), Some(1));
        assert_eq!(u.max_val(), Some(9));
    }
    #[test]
    fn test_metalib_ext_util_flags() {
        let mut u = MetaLibExtUtil::new("flags");
        u.set_flag(3);
        assert!(u.has_flag(3));
        assert!(!u.has_flag(2));
    }
    #[test]
    fn test_metalib_ext_util_pop() {
        let mut u = MetaLibExtUtil::new("pop");
        u.push(42);
        assert_eq!(u.pop(), Some(42));
        assert!(u.is_empty());
    }
    #[test]
    fn test_metalib_ext_map_basic() {
        let mut m: MetaLibExtMap<i32> = MetaLibExtMap::new();
        m.insert("key", 42);
        assert_eq!(m.get("key"), Some(&42));
        assert!(m.contains("key"));
        assert!(!m.contains("other"));
    }
    #[test]
    fn test_metalib_ext_map_get_or_default() {
        let mut m: MetaLibExtMap<i32> = MetaLibExtMap::new();
        m.insert("k", 5);
        assert_eq!(m.get_or_default("k"), 5);
        assert_eq!(m.get_or_default("missing"), 0);
    }
    #[test]
    fn test_metalib_ext_map_keys_sorted() {
        let mut m: MetaLibExtMap<i32> = MetaLibExtMap::new();
        m.insert("z", 1);
        m.insert("a", 2);
        m.insert("m", 3);
        let keys = m.keys_sorted();
        assert_eq!(keys[0].as_str(), "a");
        assert_eq!(keys[2].as_str(), "z");
    }
    #[test]
    fn test_metalib_window_mean() {
        let mut w = MetaLibWindow::new(3);
        w.push(1.0);
        w.push(2.0);
        w.push(3.0);
        assert!((w.mean() - 2.0).abs() < 1e-10);
    }
    #[test]
    fn test_metalib_window_evict() {
        let mut w = MetaLibWindow::new(2);
        w.push(10.0);
        w.push(20.0);
        w.push(30.0);
        assert_eq!(w.len(), 2);
        assert!((w.mean() - 25.0).abs() < 1e-10);
    }
    #[test]
    fn test_metalib_window_std_dev() {
        let mut w = MetaLibWindow::new(10);
        for i in 0..10 {
            w.push(i as f64);
        }
        assert!(w.std_dev() > 0.0);
    }
    #[test]
    fn test_metalib_builder_basic() {
        let b = MetaLibBuilder::new("test")
            .add_item("a")
            .add_item("b")
            .set_config("key", "val");
        assert_eq!(b.item_count(), 2);
        assert!(b.has_config("key"));
        assert_eq!(b.get_config("key"), Some("val"));
    }
    #[test]
    fn test_metalib_builder_summary() {
        let b = MetaLibBuilder::new("suite").add_item("x");
        let s = b.build_summary();
        assert!(s.contains("suite"));
    }
    #[test]
    fn test_metalib_state_machine_start() {
        let mut sm = MetaLibStateMachine::new();
        assert!(sm.start());
        assert!(sm.state.is_running());
    }
    #[test]
    fn test_metalib_state_machine_complete() {
        let mut sm = MetaLibStateMachine::new();
        sm.start();
        sm.complete();
        assert!(sm.state.is_terminal());
    }
    #[test]
    fn test_metalib_state_machine_fail() {
        let mut sm = MetaLibStateMachine::new();
        sm.fail("oops");
        assert!(sm.state.is_terminal());
        assert_eq!(sm.state.error_msg(), Some("oops"));
    }
    #[test]
    fn test_metalib_state_machine_no_transition_after_terminal() {
        let mut sm = MetaLibStateMachine::new();
        sm.complete();
        assert!(!sm.start());
    }
    #[test]
    fn test_metalib_work_queue_basic() {
        let mut wq = MetaLibWorkQueue::new(10);
        wq.enqueue("task1".to_string());
        wq.enqueue("task2".to_string());
        assert_eq!(wq.pending_count(), 2);
        let t = wq.dequeue();
        assert_eq!(t, Some("task1".to_string()));
        assert_eq!(wq.processed_count(), 1);
    }
    #[test]
    fn test_metalib_work_queue_capacity() {
        let mut wq = MetaLibWorkQueue::new(2);
        wq.enqueue("a".to_string());
        wq.enqueue("b".to_string());
        assert!(wq.is_full());
        assert!(!wq.enqueue("c".to_string()));
    }
    #[test]
    fn test_metalib_counter_map_basic() {
        let mut cm = MetaLibCounterMap::new();
        cm.increment("apple");
        cm.increment("apple");
        cm.increment("banana");
        assert_eq!(cm.count("apple"), 2);
        assert_eq!(cm.count("banana"), 1);
        assert_eq!(cm.num_unique(), 2);
    }
    #[test]
    fn test_metalib_counter_map_frequency() {
        let mut cm = MetaLibCounterMap::new();
        cm.increment("a");
        cm.increment("a");
        cm.increment("b");
        assert!((cm.frequency("a") - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_metalib_counter_map_most_common() {
        let mut cm = MetaLibCounterMap::new();
        cm.increment("x");
        cm.increment("y");
        cm.increment("x");
        let (k, v) = cm
            .most_common()
            .expect("most_common should return a value after increments");
        assert_eq!(k.as_str(), "x");
        assert_eq!(v, 2);
    }
}
#[cfg(test)]
mod lib_analysis_tests {
    use super::*;
    #[test]
    fn test_lib_result_ok() {
        let r = LibResult::Ok("success".to_string());
        assert!(r.is_ok());
        assert!(!r.is_err());
        assert_eq!(r.ok_msg(), Some("success"));
        assert!((r.progress() - 1.0).abs() < 1e-10);
    }
    #[test]
    fn test_lib_result_err() {
        let r = LibResult::Err("failure".to_string());
        assert!(r.is_err());
        assert_eq!(r.err_msg(), Some("failure"));
        assert!((r.progress() - 0.0).abs() < 1e-10);
    }
    #[test]
    fn test_lib_result_partial() {
        let r = LibResult::Partial { done: 3, total: 10 };
        assert!(r.is_partial());
        assert!((r.progress() - 0.3).abs() < 1e-10);
    }
    #[test]
    fn test_lib_result_skipped() {
        let r = LibResult::Skipped;
        assert!(r.is_skipped());
    }
    #[test]
    fn test_lib_analysis_pass_run() {
        let mut p = LibAnalysisPass::new("test_pass");
        let r = p.run("hello");
        assert!(r.is_ok());
        assert_eq!(p.total_runs, 1);
        assert_eq!(p.success_count(), 1);
    }
    #[test]
    fn test_lib_analysis_pass_empty_input() {
        let mut p = LibAnalysisPass::new("empty_test");
        let r = p.run("");
        assert!(r.is_err());
        assert_eq!(p.error_count(), 1);
    }
    #[test]
    fn test_lib_analysis_pass_success_rate() {
        let mut p = LibAnalysisPass::new("rate_test");
        p.run("a");
        p.run("b");
        p.run("");
        assert!((p.success_rate() - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_lib_analysis_pass_disable() {
        let mut p = LibAnalysisPass::new("disable_test");
        p.disable();
        assert!(!p.enabled);
        p.enable();
        assert!(p.enabled);
    }
    #[test]
    fn test_lib_pipeline_basic() {
        let mut pipeline = LibPipeline::new("main_pipeline");
        pipeline.add_pass(LibAnalysisPass::new("pass1"));
        pipeline.add_pass(LibAnalysisPass::new("pass2"));
        assert_eq!(pipeline.num_passes(), 2);
        let results = pipeline.run_all("test_input");
        assert_eq!(results.len(), 2);
    }
    #[test]
    fn test_lib_pipeline_disabled_pass() {
        let mut pipeline = LibPipeline::new("partial");
        let mut p = LibAnalysisPass::new("disabled");
        p.disable();
        pipeline.add_pass(p);
        pipeline.add_pass(LibAnalysisPass::new("enabled"));
        assert_eq!(pipeline.num_enabled_passes(), 1);
        let results = pipeline.run_all("input");
        assert_eq!(results.len(), 1);
    }
    #[test]
    fn test_lib_diff_basic() {
        let mut d = LibDiff::new();
        d.add("new_item");
        d.remove("old_item");
        d.keep("same_item");
        assert!(!d.is_empty());
        assert_eq!(d.total_changes(), 2);
        assert_eq!(d.net_additions(), 0);
    }
    #[test]
    fn test_lib_diff_summary() {
        let mut d = LibDiff::new();
        d.add("x");
        d.add("y");
        d.remove("z");
        let s = d.summary();
        assert!(s.contains("+2"));
    }
    #[test]
    fn test_lib_config_set_get() {
        let mut cfg = LibConfig::new();
        cfg.set_bool("debug", true);
        cfg.set_int("max_iter", 100);
        cfg.set_str("name", "test");
        assert_eq!(cfg.get_bool("debug"), Some(true));
        assert_eq!(cfg.get_int("max_iter"), Some(100));
        assert_eq!(cfg.get_str("name"), Some("test"));
    }
    #[test]
    fn test_lib_config_read_only() {
        let mut cfg = LibConfig::new();
        cfg.set_bool("key", true);
        cfg.lock();
        assert!(!cfg.set_bool("key", false));
        assert_eq!(cfg.get_bool("key"), Some(true));
        cfg.unlock();
        assert!(cfg.set_bool("key", false));
    }
    #[test]
    fn test_lib_config_remove() {
        let mut cfg = LibConfig::new();
        cfg.set_int("x", 42);
        assert!(cfg.has("x"));
        cfg.remove("x");
        assert!(!cfg.has("x"));
    }
    #[test]
    fn test_lib_diagnostics_basic() {
        let mut diag = LibDiagnostics::new(10);
        diag.error("something went wrong");
        diag.warning("maybe check this");
        diag.note("fyi");
        assert!(diag.has_errors());
        assert!(!diag.is_clean());
        assert_eq!(diag.num_errors(), 1);
        assert_eq!(diag.num_warnings(), 1);
    }
    #[test]
    fn test_lib_diagnostics_max_errors() {
        let mut diag = LibDiagnostics::new(2);
        diag.error("e1");
        diag.error("e2");
        diag.error("e3");
        assert_eq!(diag.num_errors(), 2);
        assert!(diag.at_error_limit());
    }
    #[test]
    fn test_lib_diagnostics_clear() {
        let mut diag = LibDiagnostics::new(10);
        diag.error("e1");
        diag.clear();
        assert!(diag.is_clean());
    }
    #[test]
    fn test_lib_config_value_types() {
        let b = LibConfigValue::Bool(true);
        assert_eq!(b.type_name(), "bool");
        assert_eq!(b.as_bool(), Some(true));
        assert_eq!(b.as_int(), None);
        let i = LibConfigValue::Int(42);
        assert_eq!(i.type_name(), "int");
        assert_eq!(i.as_int(), Some(42));
        let f = LibConfigValue::Float(2.5);
        assert_eq!(f.type_name(), "float");
        assert!((f.as_float().expect("Float variant should return as_float") - 2.5).abs() < 1e-10);
        let s = LibConfigValue::Str("hello".to_string());
        assert_eq!(s.type_name(), "str");
        assert_eq!(s.as_str(), Some("hello"));
        let l = LibConfigValue::List(vec!["a".to_string(), "b".to_string()]);
        assert_eq!(l.type_name(), "list");
        assert_eq!(l.as_list().map(|v| v.len()), Some(2));
    }
}
#[cfg(test)]
mod lib_ext_tests_1300 {
    use super::*;
    #[test]
    fn test_lib_ext_result_ok_1300() {
        let r = LibExtResult1300::Ok("success".to_string());
        assert!(r.is_ok());
        assert!(!r.is_err());
        assert_eq!(r.ok_msg(), Some("success"));
        assert!((r.progress() - 1.0).abs() < 1e-10);
    }
    #[test]
    fn test_lib_ext_result_err_1300() {
        let r = LibExtResult1300::Err("failure".to_string());
        assert!(r.is_err());
        assert_eq!(r.err_msg(), Some("failure"));
        assert!((r.progress() - 0.0).abs() < 1e-10);
    }
    #[test]
    fn test_lib_ext_result_partial_1300() {
        let r = LibExtResult1300::Partial { done: 3, total: 10 };
        assert!(r.is_partial());
        assert!((r.progress() - 0.3).abs() < 1e-10);
    }
    #[test]
    fn test_lib_ext_result_skipped_1300() {
        let r = LibExtResult1300::Skipped;
        assert!(r.is_skipped());
    }
    #[test]
    fn test_lib_ext_pass_run_1300() {
        let mut p = LibExtPass1300::new("test_pass");
        let r = p.run("hello");
        assert!(r.is_ok());
        assert_eq!(p.total_runs, 1);
        assert_eq!(p.success_count(), 1);
    }
    #[test]
    fn test_lib_ext_pass_empty_1300() {
        let mut p = LibExtPass1300::new("empty_test");
        let r = p.run("");
        assert!(r.is_err());
        assert_eq!(p.error_count(), 1);
    }
    #[test]
    fn test_lib_ext_pass_rate_1300() {
        let mut p = LibExtPass1300::new("rate_test");
        p.run("a");
        p.run("b");
        p.run("");
        assert!((p.success_rate() - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_lib_ext_pass_disable_1300() {
        let mut p = LibExtPass1300::new("disable_test");
        p.disable();
        assert!(!p.enabled);
        p.enable();
        assert!(p.enabled);
    }
    #[test]
    fn test_lib_ext_pipeline_basic_1300() {
        let mut pipeline = LibExtPipeline1300::new("main_pipeline");
        pipeline.add_pass(LibExtPass1300::new("pass1"));
        pipeline.add_pass(LibExtPass1300::new("pass2"));
        assert_eq!(pipeline.num_passes(), 2);
        let results = pipeline.run_all("test_input");
        assert_eq!(results.len(), 2);
    }
    #[test]
    fn test_lib_ext_pipeline_disabled_1300() {
        let mut pipeline = LibExtPipeline1300::new("partial");
        let mut p = LibExtPass1300::new("disabled");
        p.disable();
        pipeline.add_pass(p);
        pipeline.add_pass(LibExtPass1300::new("enabled"));
        assert_eq!(pipeline.num_enabled_passes(), 1);
        let results = pipeline.run_all("input");
        assert_eq!(results.len(), 1);
    }
    #[test]
    fn test_lib_ext_diff_basic_1300() {
        let mut d = LibExtDiff1300::new();
        d.add("new_item");
        d.remove("old_item");
        d.keep("same_item");
        assert!(!d.is_empty());
        assert_eq!(d.total_changes(), 2);
        assert_eq!(d.net_additions(), 0);
    }
    #[test]
    fn test_lib_ext_config_set_get_1300() {
        let mut cfg = LibExtConfig1300::new();
        cfg.set_bool("debug", true);
        cfg.set_int("max_iter", 100);
        cfg.set_str("name", "test");
        assert_eq!(cfg.get_bool("debug"), Some(true));
        assert_eq!(cfg.get_int("max_iter"), Some(100));
        assert_eq!(cfg.get_str("name"), Some("test"));
    }
    #[test]
    fn test_lib_ext_config_read_only_1300() {
        let mut cfg = LibExtConfig1300::new();
        cfg.set_bool("key", true);
        cfg.lock();
        assert!(!cfg.set_bool("key", false));
        assert_eq!(cfg.get_bool("key"), Some(true));
        cfg.unlock();
        assert!(cfg.set_bool("key", false));
    }
    #[test]
    fn test_lib_ext_config_remove_1300() {
        let mut cfg = LibExtConfig1300::new();
        cfg.set_int("x", 42);
        assert!(cfg.has("x"));
        cfg.remove("x");
        assert!(!cfg.has("x"));
    }
    #[test]
    fn test_lib_ext_diagnostics_basic_1300() {
        let mut diag = LibExtDiag1300::new(10);
        diag.error("something went wrong");
        diag.warning("maybe check this");
        diag.note("fyi");
        assert!(diag.has_errors());
        assert!(!diag.is_clean());
        assert_eq!(diag.num_errors(), 1);
        assert_eq!(diag.num_warnings(), 1);
    }
    #[test]
    fn test_lib_ext_diagnostics_max_errors_1300() {
        let mut diag = LibExtDiag1300::new(2);
        diag.error("e1");
        diag.error("e2");
        diag.error("e3");
        assert_eq!(diag.num_errors(), 2);
        assert!(diag.at_error_limit());
    }
    #[test]
    fn test_lib_ext_diagnostics_clear_1300() {
        let mut diag = LibExtDiag1300::new(10);
        diag.error("e1");
        diag.clear();
        assert!(diag.is_clean());
    }
    #[test]
    fn test_lib_ext_config_value_types_1300() {
        let b = LibExtConfigVal1300::Bool(true);
        assert_eq!(b.type_name(), "bool");
        assert_eq!(b.as_bool(), Some(true));
        assert_eq!(b.as_int(), None);
        let i = LibExtConfigVal1300::Int(42);
        assert_eq!(i.type_name(), "int");
        assert_eq!(i.as_int(), Some(42));
        let f = LibExtConfigVal1300::Float(2.5);
        assert_eq!(f.type_name(), "float");
        assert!((f.as_float().expect("Float variant should return as_float") - 2.5).abs() < 1e-10);
        let s = LibExtConfigVal1300::Str("hello".to_string());
        assert_eq!(s.type_name(), "str");
        assert_eq!(s.as_str(), Some("hello"));
        let l = LibExtConfigVal1300::List(vec!["a".to_string(), "b".to_string()]);
        assert_eq!(l.type_name(), "list");
        assert_eq!(l.as_list().map(|v| v.len()), Some(2));
    }
}
