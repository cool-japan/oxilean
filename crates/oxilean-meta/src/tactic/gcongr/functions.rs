//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{
    ArgMonotonicity, GCongrConfig, GCongrRelation, GCongrResult, GCongrSubgoal, GCongrTacBuilder,
    GCongrTacCounterMap, GCongrTacExtMap, GCongrTacExtUtil, GCongrTacStateMachine, GCongrTacWindow,
    GCongrTacWorkQueue, GCongrTactic, GcongrExtConfig1000, GcongrExtConfigVal1000,
    GcongrExtDiag1000, GcongrExtDiff1000, GcongrExtPass1000, GcongrExtPipeline1000,
    GcongrExtResult1000, MonotoneEntry, MonotoneRegistry, TacticGcongrAnalysisPass,
    TacticGcongrConfig, TacticGcongrConfigValue, TacticGcongrDiagnostics, TacticGcongrDiff,
    TacticGcongrPipeline, TacticGcongrResult,
};
#[allow(unused_imports)]
use crate::basic::{MVarId, MetaContext, MetavarKind};
use crate::tactic::state::{TacticError, TacticResult, TacticState};
use oxilean_kernel::{Expr, Level, Name};

/// Find the first occurrence of `op` (a multi-char operator like ` <= `)
/// at parenthesis depth 0 in `s`.
pub(super) fn find_op_at_depth0(s: &str, op: &str) -> Option<usize> {
    let mut depth: i32 = 0;
    let bytes = s.as_bytes();
    let op_bytes = op.as_bytes();
    let op_len = op_bytes.len();
    if op_len > s.len() {
        return None;
    }
    for i in 0..=(s.len() - op_len) {
        match bytes[i] {
            b'(' => depth += 1,
            b')' => depth -= 1,
            _ => {}
        }
        if depth == 0 && &bytes[i..i + op_len] == op_bytes {
            return Some(i);
        }
    }
    None
}
/// Split an application expression into head + arguments.
///
/// `"f a b"` -> `["f", "a", "b"]`
/// `"(f a) b"` -> `["(f a)", "b"]`
pub(super) fn split_application(s: &str) -> Vec<&str> {
    let mut parts = Vec::new();
    let mut start = 0;
    let mut depth: i32 = 0;
    let bytes = s.as_bytes();
    for (i, &b) in bytes.iter().enumerate() {
        match b {
            b'(' => depth += 1,
            b')' => depth -= 1,
            b' ' if depth == 0 => {
                let token = s[start..i].trim();
                if !token.is_empty() {
                    parts.push(token);
                }
                start = i + 1;
            }
            _ => {}
        }
    }
    let last = s[start..].trim();
    if !last.is_empty() {
        parts.push(last);
    }
    parts
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::tactic::gcongr::*;
    #[test]
    fn test_relation_parse() {
        assert_eq!(GCongrRelation::parse("<="), Some(GCongrRelation::Le));
        assert_eq!(GCongrRelation::parse("<"), Some(GCongrRelation::Lt));
        assert_eq!(GCongrRelation::parse(">="), Some(GCongrRelation::Ge));
        assert_eq!(GCongrRelation::parse(">"), Some(GCongrRelation::Gt));
        assert_eq!(GCongrRelation::parse("="), Some(GCongrRelation::Eq));
        assert!(GCongrRelation::parse("").is_none());
    }
    #[test]
    fn test_relation_flip() {
        assert_eq!(GCongrRelation::Le.flip(), GCongrRelation::Ge);
        assert_eq!(GCongrRelation::Lt.flip(), GCongrRelation::Gt);
        assert_eq!(GCongrRelation::Ge.flip(), GCongrRelation::Le);
        assert_eq!(GCongrRelation::Gt.flip(), GCongrRelation::Lt);
        assert_eq!(GCongrRelation::Eq.flip(), GCongrRelation::Eq);
    }
    #[test]
    fn test_relation_compatibility() {
        assert!(GCongrRelation::Le.is_compatible(&GCongrRelation::Le));
        assert!(GCongrRelation::Le.is_compatible(&GCongrRelation::Lt));
        assert!(!GCongrRelation::Le.is_compatible(&GCongrRelation::Ge));
        assert!(GCongrRelation::Eq.is_compatible(&GCongrRelation::Le));
    }
    #[test]
    fn test_relation_join() {
        assert_eq!(
            GCongrRelation::Le.join(&GCongrRelation::Lt),
            Some(GCongrRelation::Le)
        );
        assert_eq!(
            GCongrRelation::Eq.join(&GCongrRelation::Le),
            Some(GCongrRelation::Le)
        );
        assert_eq!(
            GCongrRelation::Le.join(&GCongrRelation::Le),
            Some(GCongrRelation::Le)
        );
        assert!(GCongrRelation::Le.join(&GCongrRelation::Ge).is_none());
    }
    #[test]
    fn test_registry_with_defaults() {
        let reg = MonotoneRegistry::with_defaults();
        assert!(!reg.is_empty());
        assert!(reg.len() > 5);
        let add_entries = reg.lookup("Nat.add");
        assert!(!add_entries.is_empty());
    }
    #[test]
    fn test_registry_custom_entry() {
        let mut reg = MonotoneRegistry::new();
        reg.register(MonotoneEntry::uniform("MyFunc", 3, GCongrRelation::Le));
        assert_eq!(reg.len(), 1);
        let entries = reg.lookup("MyFunc");
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].arity, 3);
    }
    #[test]
    fn test_decompose_le_goal() {
        let tac = GCongrTactic::new();
        let goal = tac.decompose_goal("Nat.add a b <= Nat.add c d");
        assert!(goal.is_some());
        let g = goal.expect("g should be present");
        assert_eq!(g.relation, GCongrRelation::Le);
        assert_eq!(g.lhs_head.as_deref(), Some("Nat.add"));
        assert_eq!(g.rhs_head.as_deref(), Some("Nat.add"));
        assert_eq!(g.lhs_args, vec!["a", "b"]);
        assert_eq!(g.rhs_args, vec!["c", "d"]);
    }
    #[test]
    fn test_decompose_lt_goal() {
        let tac = GCongrTactic::new();
        let goal = tac.decompose_goal("f x < f y");
        assert!(goal.is_some());
        let g = goal.expect("g should be present");
        assert_eq!(g.relation, GCongrRelation::Lt);
        assert!(g.heads_match());
    }
    #[test]
    fn test_apply_basic_add_le() {
        let tac = GCongrTactic::new();
        let result = tac.apply("Nat.add a b <= Nat.add c d");
        assert!(result.success);
        assert_eq!(result.subgoals.len(), 2);
        assert_eq!(result.subgoals[0].to_goal_string(), "a <= c");
        assert_eq!(result.subgoals[1].to_goal_string(), "b <= d");
    }
    #[test]
    fn test_apply_reflexive_args_filtered() {
        let tac = GCongrTactic::new();
        let result = tac.apply("Nat.add a b <= Nat.add a d");
        assert!(result.success);
        assert_eq!(result.subgoals.len(), 1);
        assert_eq!(result.subgoals[0].to_goal_string(), "b <= d");
    }
    #[test]
    fn test_apply_head_mismatch_fails() {
        let tac = GCongrTactic::new();
        let result = tac.apply("f a <= g b");
        assert!(!result.success);
        assert!(result.message.contains("head functions do not match"));
    }
    #[test]
    fn test_apply_no_relation_fails() {
        let tac = GCongrTactic::new();
        let result = tac.apply("just some expression");
        assert!(!result.success);
        assert!(result.message.contains("could not decompose"));
    }
    #[test]
    fn test_apply_succ_single_arg() {
        let tac = GCongrTactic::new();
        let result = tac.apply("Nat.succ a <= Nat.succ b");
        assert!(result.success);
        assert_eq!(result.subgoals.len(), 1);
        assert_eq!(result.subgoals[0].to_goal_string(), "a <= b");
    }
    #[test]
    fn test_apply_with_antitone_sub() {
        let tac = GCongrTactic::new();
        let result = tac.apply("HSub.hSub a b <= HSub.hSub c d");
        assert!(result.success);
        assert_eq!(result.subgoals.len(), 2);
        assert_eq!(result.subgoals[0].lhs, "a");
        assert_eq!(result.subgoals[0].rhs, "c");
        assert_eq!(result.subgoals[1].lhs, "d");
        assert_eq!(result.subgoals[1].rhs, "b");
    }
    #[test]
    fn test_apply_structural_fallback() {
        let tac = GCongrTactic::new();
        let result = tac.apply("unknown_fn x y <= unknown_fn a b");
        assert!(result.success);
        assert_eq!(result.subgoals.len(), 2);
        assert_eq!(result.subgoals[0].to_goal_string(), "x <= a");
        assert_eq!(result.subgoals[1].to_goal_string(), "y <= b");
    }
    #[test]
    fn test_config_le_only() {
        let tac = GCongrTactic::with_config(GCongrConfig::le_only());
        let result = tac.apply("Nat.add a b <= Nat.add c d");
        assert!(result.success);
        let result2 = tac.apply("Nat.add a b < Nat.add c d");
        assert!(result2.success);
    }
    #[test]
    fn test_config_max_depth_zero() {
        let config = GCongrConfig {
            max_depth: 0,
            ..Default::default()
        };
        let tac = GCongrTactic::with_config(config);
        let result = tac.apply("Nat.add a b <= Nat.add c d");
        assert!(!result.success);
        assert!(result.message.contains("max depth"));
    }
    #[test]
    fn test_monotone_entry_uniform() {
        let entry = MonotoneEntry::uniform("test_fn", 3, GCongrRelation::Le);
        assert_eq!(entry.arity, 3);
        assert_eq!(entry.monotone_arg_count(), 3);
        assert!(entry.matches_function("test_fn"));
        assert!(!entry.matches_function("other_fn"));
    }
    #[test]
    fn test_subgoal_display() {
        let sg = GCongrSubgoal {
            lhs: "a".to_string(),
            rhs: "b".to_string(),
            relation: GCongrRelation::Le,
            arg_index: 0,
            monotonicity: ArgMonotonicity::Monotone,
        };
        assert_eq!(sg.to_goal_string(), "a <= b");
        assert!(!sg.is_reflexive());
        let refl = GCongrSubgoal {
            lhs: "x".to_string(),
            rhs: "x".to_string(),
            relation: GCongrRelation::Le,
            arg_index: 0,
            monotonicity: ArgMonotonicity::Monotone,
        };
        assert!(refl.is_reflexive());
    }
    #[test]
    fn test_result_helpers() {
        let r = GCongrResult::success(vec![], None);
        assert!(r.is_closed());
        assert_eq!(r.remaining_goals(), 0);
        let r2 = GCongrResult::failure("test");
        assert!(!r2.success);
        assert!(!r2.is_closed());
    }
    #[test]
    fn test_apply_eq_goal() {
        let tac = GCongrTactic::new();
        let result = tac.apply("HAdd.hAdd a b = HAdd.hAdd c d");
        assert!(result.success);
    }
    #[test]
    fn test_registry_function_names() {
        let reg = MonotoneRegistry::with_defaults();
        let names = reg.function_names();
        assert!(names.contains(&"Nat.add"));
        assert!(names.contains(&"Nat.mul"));
    }
    #[test]
    fn test_relation_strict_nonstrict() {
        assert!(GCongrRelation::Le.is_non_strict());
        assert!(!GCongrRelation::Le.is_strict());
        assert!(GCongrRelation::Lt.is_strict());
        assert!(!GCongrRelation::Lt.is_non_strict());
    }
}
#[cfg(test)]
mod gcongrtac_ext2_tests {
    use crate::tactic::gcongr::*;
    #[test]
    fn test_gcongrtac_ext_util_basic() {
        let mut u = GCongrTacExtUtil::new("test");
        u.push(10);
        u.push(20);
        assert_eq!(u.sum(), 30);
        assert_eq!(u.len(), 2);
    }
    #[test]
    fn test_gcongrtac_ext_util_min_max() {
        let mut u = GCongrTacExtUtil::new("mm");
        u.push(5);
        u.push(1);
        u.push(9);
        assert_eq!(u.min_val(), Some(1));
        assert_eq!(u.max_val(), Some(9));
    }
    #[test]
    fn test_gcongrtac_ext_util_flags() {
        let mut u = GCongrTacExtUtil::new("flags");
        u.set_flag(3);
        assert!(u.has_flag(3));
        assert!(!u.has_flag(2));
    }
    #[test]
    fn test_gcongrtac_ext_util_pop() {
        let mut u = GCongrTacExtUtil::new("pop");
        u.push(42);
        assert_eq!(u.pop(), Some(42));
        assert!(u.is_empty());
    }
    #[test]
    fn test_gcongrtac_ext_map_basic() {
        let mut m: GCongrTacExtMap<i32> = GCongrTacExtMap::new();
        m.insert("key", 42);
        assert_eq!(m.get("key"), Some(&42));
        assert!(m.contains("key"));
        assert!(!m.contains("other"));
    }
    #[test]
    fn test_gcongrtac_ext_map_get_or_default() {
        let mut m: GCongrTacExtMap<i32> = GCongrTacExtMap::new();
        m.insert("k", 5);
        assert_eq!(m.get_or_default("k"), 5);
        assert_eq!(m.get_or_default("missing"), 0);
    }
    #[test]
    fn test_gcongrtac_ext_map_keys_sorted() {
        let mut m: GCongrTacExtMap<i32> = GCongrTacExtMap::new();
        m.insert("z", 1);
        m.insert("a", 2);
        m.insert("m", 3);
        let keys = m.keys_sorted();
        assert_eq!(keys[0].as_str(), "a");
        assert_eq!(keys[2].as_str(), "z");
    }
    #[test]
    fn test_gcongrtac_window_mean() {
        let mut w = GCongrTacWindow::new(3);
        w.push(1.0);
        w.push(2.0);
        w.push(3.0);
        assert!((w.mean() - 2.0).abs() < 1e-10);
    }
    #[test]
    fn test_gcongrtac_window_evict() {
        let mut w = GCongrTacWindow::new(2);
        w.push(10.0);
        w.push(20.0);
        w.push(30.0);
        assert_eq!(w.len(), 2);
        assert!((w.mean() - 25.0).abs() < 1e-10);
    }
    #[test]
    fn test_gcongrtac_window_std_dev() {
        let mut w = GCongrTacWindow::new(10);
        for i in 0..10 {
            w.push(i as f64);
        }
        assert!(w.std_dev() > 0.0);
    }
    #[test]
    fn test_gcongrtac_builder_basic() {
        let b = GCongrTacBuilder::new("test")
            .add_item("a")
            .add_item("b")
            .set_config("key", "val");
        assert_eq!(b.item_count(), 2);
        assert!(b.has_config("key"));
        assert_eq!(b.get_config("key"), Some("val"));
    }
    #[test]
    fn test_gcongrtac_builder_summary() {
        let b = GCongrTacBuilder::new("suite").add_item("x");
        let s = b.build_summary();
        assert!(s.contains("suite"));
    }
    #[test]
    fn test_gcongrtac_state_machine_start() {
        let mut sm = GCongrTacStateMachine::new();
        assert!(sm.start());
        assert!(sm.state.is_running());
    }
    #[test]
    fn test_gcongrtac_state_machine_complete() {
        let mut sm = GCongrTacStateMachine::new();
        sm.start();
        sm.complete();
        assert!(sm.state.is_terminal());
    }
    #[test]
    fn test_gcongrtac_state_machine_fail() {
        let mut sm = GCongrTacStateMachine::new();
        sm.fail("oops");
        assert!(sm.state.is_terminal());
        assert_eq!(sm.state.error_msg(), Some("oops"));
    }
    #[test]
    fn test_gcongrtac_state_machine_no_transition_after_terminal() {
        let mut sm = GCongrTacStateMachine::new();
        sm.complete();
        assert!(!sm.start());
    }
    #[test]
    fn test_gcongrtac_work_queue_basic() {
        let mut wq = GCongrTacWorkQueue::new(10);
        wq.enqueue("task1".to_string());
        wq.enqueue("task2".to_string());
        assert_eq!(wq.pending_count(), 2);
        let t = wq.dequeue();
        assert_eq!(t, Some("task1".to_string()));
        assert_eq!(wq.processed_count(), 1);
    }
    #[test]
    fn test_gcongrtac_work_queue_capacity() {
        let mut wq = GCongrTacWorkQueue::new(2);
        wq.enqueue("a".to_string());
        wq.enqueue("b".to_string());
        assert!(wq.is_full());
        assert!(!wq.enqueue("c".to_string()));
    }
    #[test]
    fn test_gcongrtac_counter_map_basic() {
        let mut cm = GCongrTacCounterMap::new();
        cm.increment("apple");
        cm.increment("apple");
        cm.increment("banana");
        assert_eq!(cm.count("apple"), 2);
        assert_eq!(cm.count("banana"), 1);
        assert_eq!(cm.num_unique(), 2);
    }
    #[test]
    fn test_gcongrtac_counter_map_frequency() {
        let mut cm = GCongrTacCounterMap::new();
        cm.increment("a");
        cm.increment("a");
        cm.increment("b");
        assert!((cm.frequency("a") - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_gcongrtac_counter_map_most_common() {
        let mut cm = GCongrTacCounterMap::new();
        cm.increment("x");
        cm.increment("y");
        cm.increment("x");
        let (k, v) = cm.most_common().expect("most_common should succeed");
        assert_eq!(k.as_str(), "x");
        assert_eq!(v, 2);
    }
}
#[cfg(test)]
mod tacticgcongr_analysis_tests {
    use crate::tactic::gcongr::*;
    #[test]
    fn test_tacticgcongr_result_ok() {
        let r = TacticGcongrResult::Ok("success".to_string());
        assert!(r.is_ok());
        assert!(!r.is_err());
        assert_eq!(r.ok_msg(), Some("success"));
        assert!((r.progress() - 1.0).abs() < 1e-10);
    }
    #[test]
    fn test_tacticgcongr_result_err() {
        let r = TacticGcongrResult::Err("failure".to_string());
        assert!(r.is_err());
        assert_eq!(r.err_msg(), Some("failure"));
        assert!((r.progress() - 0.0).abs() < 1e-10);
    }
    #[test]
    fn test_tacticgcongr_result_partial() {
        let r = TacticGcongrResult::Partial { done: 3, total: 10 };
        assert!(r.is_partial());
        assert!((r.progress() - 0.3).abs() < 1e-10);
    }
    #[test]
    fn test_tacticgcongr_result_skipped() {
        let r = TacticGcongrResult::Skipped;
        assert!(r.is_skipped());
    }
    #[test]
    fn test_tacticgcongr_analysis_pass_run() {
        let mut p = TacticGcongrAnalysisPass::new("test_pass");
        let r = p.run("hello");
        assert!(r.is_ok());
        assert_eq!(p.total_runs, 1);
        assert_eq!(p.success_count(), 1);
    }
    #[test]
    fn test_tacticgcongr_analysis_pass_empty_input() {
        let mut p = TacticGcongrAnalysisPass::new("empty_test");
        let r = p.run("");
        assert!(r.is_err());
        assert_eq!(p.error_count(), 1);
    }
    #[test]
    fn test_tacticgcongr_analysis_pass_success_rate() {
        let mut p = TacticGcongrAnalysisPass::new("rate_test");
        p.run("a");
        p.run("b");
        p.run("");
        assert!((p.success_rate() - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_tacticgcongr_analysis_pass_disable() {
        let mut p = TacticGcongrAnalysisPass::new("disable_test");
        p.disable();
        assert!(!p.enabled);
        p.enable();
        assert!(p.enabled);
    }
    #[test]
    fn test_tacticgcongr_pipeline_basic() {
        let mut pipeline = TacticGcongrPipeline::new("main_pipeline");
        pipeline.add_pass(TacticGcongrAnalysisPass::new("pass1"));
        pipeline.add_pass(TacticGcongrAnalysisPass::new("pass2"));
        assert_eq!(pipeline.num_passes(), 2);
        let results = pipeline.run_all("test_input");
        assert_eq!(results.len(), 2);
    }
    #[test]
    fn test_tacticgcongr_pipeline_disabled_pass() {
        let mut pipeline = TacticGcongrPipeline::new("partial");
        let mut p = TacticGcongrAnalysisPass::new("disabled");
        p.disable();
        pipeline.add_pass(p);
        pipeline.add_pass(TacticGcongrAnalysisPass::new("enabled"));
        assert_eq!(pipeline.num_enabled_passes(), 1);
        let results = pipeline.run_all("input");
        assert_eq!(results.len(), 1);
    }
    #[test]
    fn test_tacticgcongr_diff_basic() {
        let mut d = TacticGcongrDiff::new();
        d.add("new_item");
        d.remove("old_item");
        d.keep("same_item");
        assert!(!d.is_empty());
        assert_eq!(d.total_changes(), 2);
        assert_eq!(d.net_additions(), 0);
    }
    #[test]
    fn test_tacticgcongr_diff_summary() {
        let mut d = TacticGcongrDiff::new();
        d.add("x");
        d.add("y");
        d.remove("z");
        let s = d.summary();
        assert!(s.contains("+2"));
    }
    #[test]
    fn test_tacticgcongr_config_set_get() {
        let mut cfg = TacticGcongrConfig::new();
        cfg.set_bool("debug", true);
        cfg.set_int("max_iter", 100);
        cfg.set_str("name", "test");
        assert_eq!(cfg.get_bool("debug"), Some(true));
        assert_eq!(cfg.get_int("max_iter"), Some(100));
        assert_eq!(cfg.get_str("name"), Some("test"));
    }
    #[test]
    fn test_tacticgcongr_config_read_only() {
        let mut cfg = TacticGcongrConfig::new();
        cfg.set_bool("key", true);
        cfg.lock();
        assert!(!cfg.set_bool("key", false));
        assert_eq!(cfg.get_bool("key"), Some(true));
        cfg.unlock();
        assert!(cfg.set_bool("key", false));
    }
    #[test]
    fn test_tacticgcongr_config_remove() {
        let mut cfg = TacticGcongrConfig::new();
        cfg.set_int("x", 42);
        assert!(cfg.has("x"));
        cfg.remove("x");
        assert!(!cfg.has("x"));
    }
    #[test]
    fn test_tacticgcongr_diagnostics_basic() {
        let mut diag = TacticGcongrDiagnostics::new(10);
        diag.error("something went wrong");
        diag.warning("maybe check this");
        diag.note("fyi");
        assert!(diag.has_errors());
        assert!(!diag.is_clean());
        assert_eq!(diag.num_errors(), 1);
        assert_eq!(diag.num_warnings(), 1);
    }
    #[test]
    fn test_tacticgcongr_diagnostics_max_errors() {
        let mut diag = TacticGcongrDiagnostics::new(2);
        diag.error("e1");
        diag.error("e2");
        diag.error("e3");
        assert_eq!(diag.num_errors(), 2);
        assert!(diag.at_error_limit());
    }
    #[test]
    fn test_tacticgcongr_diagnostics_clear() {
        let mut diag = TacticGcongrDiagnostics::new(10);
        diag.error("e1");
        diag.clear();
        assert!(diag.is_clean());
    }
    #[test]
    fn test_tacticgcongr_config_value_types() {
        let b = TacticGcongrConfigValue::Bool(true);
        assert_eq!(b.type_name(), "bool");
        assert_eq!(b.as_bool(), Some(true));
        assert_eq!(b.as_int(), None);
        let i = TacticGcongrConfigValue::Int(42);
        assert_eq!(i.type_name(), "int");
        assert_eq!(i.as_int(), Some(42));
        let f = TacticGcongrConfigValue::Float(2.5);
        assert_eq!(f.type_name(), "float");
        assert!((f.as_float().expect("as_float should succeed") - 2.5).abs() < 1e-10);
        let s = TacticGcongrConfigValue::Str("hello".to_string());
        assert_eq!(s.type_name(), "str");
        assert_eq!(s.as_str(), Some("hello"));
        let l = TacticGcongrConfigValue::List(vec!["a".to_string(), "b".to_string()]);
        assert_eq!(l.type_name(), "list");
        assert_eq!(l.as_list().map(|v| v.len()), Some(2));
    }
}
#[cfg(test)]
mod gcongr_ext_tests_1000 {
    use crate::tactic::gcongr::*;
    #[test]
    fn test_gcongr_ext_result_ok_1000() {
        let r = GcongrExtResult1000::Ok("success".to_string());
        assert!(r.is_ok());
        assert!(!r.is_err());
        assert_eq!(r.ok_msg(), Some("success"));
        assert!((r.progress() - 1.0).abs() < 1e-10);
    }
    #[test]
    fn test_gcongr_ext_result_err_1000() {
        let r = GcongrExtResult1000::Err("failure".to_string());
        assert!(r.is_err());
        assert_eq!(r.err_msg(), Some("failure"));
        assert!((r.progress() - 0.0).abs() < 1e-10);
    }
    #[test]
    fn test_gcongr_ext_result_partial_1000() {
        let r = GcongrExtResult1000::Partial { done: 3, total: 10 };
        assert!(r.is_partial());
        assert!((r.progress() - 0.3).abs() < 1e-10);
    }
    #[test]
    fn test_gcongr_ext_result_skipped_1000() {
        let r = GcongrExtResult1000::Skipped;
        assert!(r.is_skipped());
    }
    #[test]
    fn test_gcongr_ext_pass_run_1000() {
        let mut p = GcongrExtPass1000::new("test_pass");
        let r = p.run("hello");
        assert!(r.is_ok());
        assert_eq!(p.total_runs, 1);
        assert_eq!(p.success_count(), 1);
    }
    #[test]
    fn test_gcongr_ext_pass_empty_1000() {
        let mut p = GcongrExtPass1000::new("empty_test");
        let r = p.run("");
        assert!(r.is_err());
        assert_eq!(p.error_count(), 1);
    }
    #[test]
    fn test_gcongr_ext_pass_rate_1000() {
        let mut p = GcongrExtPass1000::new("rate_test");
        p.run("a");
        p.run("b");
        p.run("");
        assert!((p.success_rate() - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_gcongr_ext_pass_disable_1000() {
        let mut p = GcongrExtPass1000::new("disable_test");
        p.disable();
        assert!(!p.enabled);
        p.enable();
        assert!(p.enabled);
    }
    #[test]
    fn test_gcongr_ext_pipeline_basic_1000() {
        let mut pipeline = GcongrExtPipeline1000::new("main_pipeline");
        pipeline.add_pass(GcongrExtPass1000::new("pass1"));
        pipeline.add_pass(GcongrExtPass1000::new("pass2"));
        assert_eq!(pipeline.num_passes(), 2);
        let results = pipeline.run_all("test_input");
        assert_eq!(results.len(), 2);
    }
    #[test]
    fn test_gcongr_ext_pipeline_disabled_1000() {
        let mut pipeline = GcongrExtPipeline1000::new("partial");
        let mut p = GcongrExtPass1000::new("disabled");
        p.disable();
        pipeline.add_pass(p);
        pipeline.add_pass(GcongrExtPass1000::new("enabled"));
        assert_eq!(pipeline.num_enabled_passes(), 1);
        let results = pipeline.run_all("input");
        assert_eq!(results.len(), 1);
    }
    #[test]
    fn test_gcongr_ext_diff_basic_1000() {
        let mut d = GcongrExtDiff1000::new();
        d.add("new_item");
        d.remove("old_item");
        d.keep("same_item");
        assert!(!d.is_empty());
        assert_eq!(d.total_changes(), 2);
        assert_eq!(d.net_additions(), 0);
    }
    #[test]
    fn test_gcongr_ext_config_set_get_1000() {
        let mut cfg = GcongrExtConfig1000::new();
        cfg.set_bool("debug", true);
        cfg.set_int("max_iter", 100);
        cfg.set_str("name", "test");
        assert_eq!(cfg.get_bool("debug"), Some(true));
        assert_eq!(cfg.get_int("max_iter"), Some(100));
        assert_eq!(cfg.get_str("name"), Some("test"));
    }
    #[test]
    fn test_gcongr_ext_config_read_only_1000() {
        let mut cfg = GcongrExtConfig1000::new();
        cfg.set_bool("key", true);
        cfg.lock();
        assert!(!cfg.set_bool("key", false));
        assert_eq!(cfg.get_bool("key"), Some(true));
        cfg.unlock();
        assert!(cfg.set_bool("key", false));
    }
    #[test]
    fn test_gcongr_ext_config_remove_1000() {
        let mut cfg = GcongrExtConfig1000::new();
        cfg.set_int("x", 42);
        assert!(cfg.has("x"));
        cfg.remove("x");
        assert!(!cfg.has("x"));
    }
    #[test]
    fn test_gcongr_ext_diagnostics_basic_1000() {
        let mut diag = GcongrExtDiag1000::new(10);
        diag.error("something went wrong");
        diag.warning("maybe check this");
        diag.note("fyi");
        assert!(diag.has_errors());
        assert!(!diag.is_clean());
        assert_eq!(diag.num_errors(), 1);
        assert_eq!(diag.num_warnings(), 1);
    }
    #[test]
    fn test_gcongr_ext_diagnostics_max_errors_1000() {
        let mut diag = GcongrExtDiag1000::new(2);
        diag.error("e1");
        diag.error("e2");
        diag.error("e3");
        assert_eq!(diag.num_errors(), 2);
        assert!(diag.at_error_limit());
    }
    #[test]
    fn test_gcongr_ext_diagnostics_clear_1000() {
        let mut diag = GcongrExtDiag1000::new(10);
        diag.error("e1");
        diag.clear();
        assert!(diag.is_clean());
    }
    #[test]
    fn test_gcongr_ext_config_value_types_1000() {
        let b = GcongrExtConfigVal1000::Bool(true);
        assert_eq!(b.type_name(), "bool");
        assert_eq!(b.as_bool(), Some(true));
        assert_eq!(b.as_int(), None);
        let i = GcongrExtConfigVal1000::Int(42);
        assert_eq!(i.type_name(), "int");
        assert_eq!(i.as_int(), Some(42));
        let f = GcongrExtConfigVal1000::Float(2.5);
        assert_eq!(f.type_name(), "float");
        assert!((f.as_float().expect("as_float should succeed") - 2.5).abs() < 1e-10);
        let s = GcongrExtConfigVal1000::Str("hello".to_string());
        assert_eq!(s.type_name(), "str");
        assert_eq!(s.as_str(), Some("hello"));
        let l = GcongrExtConfigVal1000::List(vec!["a".to_string(), "b".to_string()]);
        assert_eq!(l.type_name(), "list");
        assert_eq!(l.as_list().map(|v| v.len()), Some(2));
    }
}

/// `gcongr` — generalized congruence tactic.
///
/// When the current goal has the form `f a₁ … aₙ ≤ f b₁ … bₙ` (or `<`, `=`, `≥`, `>`),
/// this tactic applies the monotonicity rule for `f` to reduce the goal to a list of
/// positional subgoals `aᵢ ≤ bᵢ` (skipping reflexive arguments where `aᵢ = bᵢ`).
///
/// Each non-trivial subgoal is registered as a fresh `Prop`-typed metavariable so that
/// subsequent tactics can fill them in.  If the tactic cannot decompose the goal it
/// returns [`TacticError::GoalMismatch`].
pub fn tac_gcongr(state: &mut TacticState, ctx: &mut MetaContext) -> TacticResult<()> {
    let goal = state.current_goal()?;
    let target = ctx
        .get_mvar_type(goal)
        .cloned()
        .ok_or_else(|| TacticError::Internal("gcongr: goal has no type".into()))?;
    let target = ctx.instantiate_mvars(&target);
    let target_str = target.to_string();

    let tac = GCongrTactic::new();
    let result = tac.apply(&target_str);

    if !result.success {
        return Err(TacticError::GoalMismatch(format!(
            "gcongr: {}",
            result.message
        )));
    }

    // If the goal is already closed (no non-reflexive subgoals), assign a trivial proof.
    if result.is_closed() {
        let proof = Expr::Const(Name::str("gcongr.refl"), vec![]);
        state.close_goal(proof, ctx)?;
        return Ok(());
    }

    // For each non-reflexive subgoal, create a fresh Prop-typed metavariable.
    let prop_sort = Expr::Sort(Level::zero());
    let mut new_goal_ids: Vec<MVarId> = Vec::with_capacity(result.subgoals.len());
    for subgoal in &result.subgoals {
        if subgoal.is_reflexive() {
            continue;
        }
        let (sub_id, _sub_expr) = ctx.mk_fresh_expr_mvar(prop_sort.clone(), MetavarKind::Natural);
        new_goal_ids.push(sub_id);
    }

    // Assign the current goal's metavar to a synthetic congruence application
    // and replace the focused goal with the fresh subgoal metavars.
    let congr_proof = Expr::Const(Name::str("gcongr.apply"), vec![]);
    ctx.assign_mvar(goal, congr_proof);
    state.replace_goal(new_goal_ids);

    Ok(())
}
