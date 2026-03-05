//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{
    ApplyRulesCache, ApplyRulesConfig, ApplyRulesExtConfig3400, ApplyRulesExtConfigVal3400,
    ApplyRulesExtDiag3400, ApplyRulesExtDiff3400, ApplyRulesExtPass3400, ApplyRulesExtPipeline3400,
    ApplyRulesExtResult3400, ApplyRulesLogger, ApplyRulesPriorityQueue, ApplyRulesRegistry,
    ApplyRulesResult, ApplyRulesStats, ApplyRulesTactic, ApplyRulesUtil0, ReasoningMode,
    RuleApplication, RuleEntry, RuleSet, RuleShape, TacticApplyRulesAnalysisPass,
    TacticApplyRulesConfig, TacticApplyRulesConfigValue, TacticApplyRulesDiagnostics,
    TacticApplyRulesDiff, TacticApplyRulesPipeline, TacticApplyRulesResult,
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tactic::apply_rules::*;
    #[test]
    fn test_reasoning_mode_defaults() {
        assert_eq!(ReasoningMode::default(), ReasoningMode::Backward);
        assert!(ReasoningMode::Backward.allows_backward());
        assert!(!ReasoningMode::Backward.allows_forward());
        assert!(ReasoningMode::Forward.allows_forward());
        assert!(!ReasoningMode::Forward.allows_backward());
        assert!(ReasoningMode::Both.allows_backward());
        assert!(ReasoningMode::Both.allows_forward());
    }
    #[test]
    fn test_reasoning_mode_display() {
        assert_eq!(ReasoningMode::Backward.to_string(), "backward");
        assert_eq!(ReasoningMode::Forward.to_string(), "forward");
        assert_eq!(ReasoningMode::Both.to_string(), "both");
    }
    #[test]
    fn test_rule_entry_new() {
        let rule = RuleEntry::new("my_lemma", 500);
        assert_eq!(rule.name, "my_lemma");
        assert_eq!(rule.priority, 500);
        assert!(rule.safe);
        assert_eq!(rule.shape, RuleShape::Unknown);
    }
    #[test]
    fn test_rule_entry_builder() {
        let rule = RuleEntry::new("And.intro", 300)
            .with_shape(RuleShape::MultiSubgoal(2))
            .with_params(2)
            .with_conclusion("And")
            .with_tag("logic")
            .unsafe_rule();
        assert_eq!(rule.name, "And.intro");
        assert_eq!(rule.shape, RuleShape::MultiSubgoal(2));
        assert_eq!(rule.num_params, 2);
        assert_eq!(rule.conclusion_pattern, Some("And".to_string()));
        assert_eq!(rule.tag, Some("logic".to_string()));
        assert!(!rule.safe);
    }
    #[test]
    fn test_rule_entry_matches_goal() {
        let rule = RuleEntry::new("test", 100).with_conclusion("<=");
        assert!(rule.matches_goal("a <= b"));
        assert!(!rule.matches_goal("a = b"));
        let rule_any = RuleEntry::new("test", 100);
        assert!(rule_any.matches_goal("anything"));
    }
    #[test]
    fn test_rule_entry_can_close() {
        let closing = RuleEntry::new("refl", 100).with_shape(RuleShape::Closing);
        assert!(closing.can_close());
        let non_closing = RuleEntry::new("split", 200).with_shape(RuleShape::MultiSubgoal(2));
        assert!(!non_closing.can_close());
    }
    #[test]
    fn test_rule_set_from_names() {
        let set = RuleSet::from_names(&["a", "b", "c"]);
        assert_eq!(set.len(), 3);
        assert!(!set.is_empty());
    }
    #[test]
    fn test_rule_set_with_defaults() {
        let set = RuleSet::with_defaults();
        assert!(set.len() >= 5);
        let order_rules = set.rules_by_tag("order");
        assert!(!order_rules.is_empty());
        let logic_rules = set.rules_by_tag("logic");
        assert!(!logic_rules.is_empty());
    }
    #[test]
    fn test_rule_set_matching_rules() {
        let set = RuleSet::with_defaults();
        let le_rules = set.matching_rules("a <= b");
        assert!(!le_rules.is_empty());
        assert!(le_rules.iter().any(|r| r.name == "Nat.le_refl"));
    }
    #[test]
    fn test_rule_set_safe_rules() {
        let set = RuleSet::with_defaults();
        let safe = set.safe_rules();
        assert!(safe.iter().all(|r| r.safe));
    }
    #[test]
    fn test_apply_rules_closing() {
        let tac = ApplyRulesTactic::new();
        let result = tac.apply("a <= b");
        assert!(result.success);
        assert!(result.num_applications > 0);
    }
    #[test]
    fn test_apply_rules_no_match() {
        let set = RuleSet::from_names(&[]);
        let tac = ApplyRulesTactic::with_rules(set);
        let result = tac.apply("impossible goal qwerty");
        assert_eq!(result.num_applications, 0);
    }
    #[test]
    fn test_apply_rules_with_hyps_forward() {
        let mut set = RuleSet::new();
        set.add(
            RuleEntry::new("my_forward_rule", 100)
                .with_hyp_pattern("P")
                .with_shape(RuleShape::Closing),
        );
        let config = ApplyRulesConfig::forward(5);
        let tac = ApplyRulesTactic::with_config_and_rules(config, set);
        let result = tac.apply_with_hyps("Q", &["P holds"]);
        assert!(result.trace.iter().any(|a| a.forward));
    }
    #[test]
    fn test_apply_rules_safe_only() {
        let config = ApplyRulesConfig::safe_backward(5);
        let tac = ApplyRulesTactic::with_config(config);
        let result = tac.apply("a <= b");
        for app in &result.trace {
            let rule = tac
                .rules()
                .all_rules()
                .iter()
                .find(|r| r.name == app.rule_name);
            if let Some(r) = rule {
                assert!(r.safe, "used unsafe rule: {}", r.name);
            }
        }
    }
    #[test]
    fn test_apply_rules_result_helpers() {
        let r = ApplyRulesResult::success(vec![], vec![]);
        assert!(r.is_closed());
        assert_eq!(r.num_applications, 0);
        let r2 = ApplyRulesResult::failure("no rules");
        assert!(!r2.success);
        assert!(!r2.is_closed());
    }
    #[test]
    fn test_apply_rules_distinct_rules() {
        let tac = ApplyRulesTactic::new();
        let result = tac.apply("a <= b");
        let distinct = result.distinct_rules_applied();
        assert!(distinct <= result.num_applications);
    }
    #[test]
    fn test_config_with_tag_filter() {
        let config = ApplyRulesConfig::default().with_tag("order".to_string());
        assert!(config.tag_filter.is_some());
        let tags = config.tag_filter.expect("tags should be present");
        assert!(tags.contains(&"order".to_string()));
    }
    #[test]
    fn test_rule_application_display() {
        let app = RuleApplication {
            rule_name: "my_rule".to_string(),
            goal_before: "a <= b".to_string(),
            subgoals_after: vec![],
            depth: 0,
            forward: false,
        };
        let s = app.to_string();
        assert!(s.contains("my_rule"));
        assert!(s.contains("closed"));
        let app2 = RuleApplication {
            rule_name: "split".to_string(),
            goal_before: "P And Q".to_string(),
            subgoals_after: vec!["P".to_string(), "Q".to_string()],
            depth: 1,
            forward: false,
        };
        let s2 = app2.to_string();
        assert!(s2.contains("2 sub-goal"));
    }
    #[test]
    fn test_rule_set_tags() {
        let set = RuleSet::with_defaults();
        let tags = set.tags();
        assert!(tags.contains(&"order"));
        assert!(tags.contains(&"logic"));
    }
    #[test]
    fn test_apply_rules_and_goal() {
        let tac = ApplyRulesTactic::new();
        let result = tac.apply("P And Q");
        assert!(result.success);
        assert!(result.trace.iter().any(|a| a.rule_name == "And.intro"));
    }
    #[test]
    fn test_rule_matches_hypothesis() {
        let rule = RuleEntry::new("test", 100).with_hyp_pattern("le");
        assert!(rule.matches_hypothesis("a le b"));
        assert!(!rule.matches_hypothesis("a = b"));
        let rule_no_hyp = RuleEntry::new("test", 100);
        assert!(!rule_no_hyp.matches_hypothesis("anything"));
    }
}
/// Compute a simple hash of a ApplyRules name.
#[allow(dead_code)]
pub fn applyrules_hash(name: &str) -> u64 {
    let mut h: u64 = 14695981039346656037;
    for b in name.bytes() {
        h ^= b as u64;
        h = h.wrapping_mul(1099511628211);
    }
    h
}
/// Check if a ApplyRules name is valid.
#[allow(dead_code)]
pub fn applyrules_is_valid_name(name: &str) -> bool {
    !name.is_empty() && name.chars().all(|c| c.is_alphanumeric() || c == '_')
}
/// Count the occurrences of a character in a ApplyRules string.
#[allow(dead_code)]
pub fn applyrules_count_char(s: &str, c: char) -> usize {
    s.chars().filter(|&ch| ch == c).count()
}
/// Truncate a ApplyRules string to a maximum length.
#[allow(dead_code)]
pub fn applyrules_truncate(s: &str, max_len: usize) -> &str {
    if s.len() <= max_len {
        s
    } else {
        &s[..max_len]
    }
}
/// Join ApplyRules strings with a separator.
#[allow(dead_code)]
pub fn applyrules_join(parts: &[&str], sep: &str) -> String {
    parts.join(sep)
}
#[cfg(test)]
mod applyrules_ext_tests {
    use crate::tactic::apply_rules::*;
    #[test]
    fn test_applyrules_util_new() {
        let u = ApplyRulesUtil0::new(1, "test", 42);
        assert_eq!(u.id, 1);
        assert_eq!(u.name, "test");
        assert_eq!(u.value, 42);
        assert!(u.is_active());
    }
    #[test]
    fn test_applyrules_util_tag() {
        let u = ApplyRulesUtil0::new(2, "tagged", 10).with_tag("important");
        assert!(u.has_tag("important"));
        assert_eq!(u.tag_count(), 1);
    }
    #[test]
    fn test_applyrules_util_disable() {
        let u = ApplyRulesUtil0::new(3, "disabled", 100).disable();
        assert!(!u.is_active());
        assert_eq!(u.score(), 0);
    }
    #[test]
    fn test_applyrules_registry_register() {
        let mut reg = ApplyRulesRegistry::new(10);
        let u = ApplyRulesUtil0::new(1, "a", 1);
        assert!(reg.register(u));
        assert_eq!(reg.count(), 1);
    }
    #[test]
    fn test_applyrules_registry_lookup() {
        let mut reg = ApplyRulesRegistry::new(10);
        reg.register(ApplyRulesUtil0::new(5, "five", 5));
        assert!(reg.lookup(5).is_some());
        assert!(reg.lookup(99).is_none());
    }
    #[test]
    fn test_applyrules_registry_capacity() {
        let mut reg = ApplyRulesRegistry::new(2);
        reg.register(ApplyRulesUtil0::new(1, "a", 1));
        reg.register(ApplyRulesUtil0::new(2, "b", 2));
        assert!(reg.is_full());
        assert!(!reg.register(ApplyRulesUtil0::new(3, "c", 3)));
    }
    #[test]
    fn test_applyrules_registry_score() {
        let mut reg = ApplyRulesRegistry::new(10);
        reg.register(ApplyRulesUtil0::new(1, "a", 10));
        reg.register(ApplyRulesUtil0::new(2, "b", 20));
        assert_eq!(reg.total_score(), 30);
    }
    #[test]
    fn test_applyrules_cache_hit_miss() {
        let mut cache = ApplyRulesCache::new();
        cache.insert("key1", 42);
        assert_eq!(cache.get("key1"), Some(42));
        assert_eq!(cache.get("key2"), None);
        assert_eq!(cache.hits, 1);
        assert_eq!(cache.misses, 1);
    }
    #[test]
    fn test_applyrules_cache_hit_rate() {
        let mut cache = ApplyRulesCache::new();
        cache.insert("k", 1);
        cache.get("k");
        cache.get("k");
        cache.get("nope");
        assert!((cache.hit_rate() - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_applyrules_cache_clear() {
        let mut cache = ApplyRulesCache::new();
        cache.insert("k", 1);
        cache.clear();
        assert_eq!(cache.size(), 0);
        assert_eq!(cache.hits, 0);
    }
    #[test]
    fn test_applyrules_logger_basic() {
        let mut logger = ApplyRulesLogger::new(100);
        logger.log("msg1");
        logger.log("msg2");
        assert_eq!(logger.count(), 2);
        assert_eq!(logger.last(), Some("msg2"));
    }
    #[test]
    fn test_applyrules_logger_capacity() {
        let mut logger = ApplyRulesLogger::new(2);
        logger.log("a");
        logger.log("b");
        logger.log("c");
        assert_eq!(logger.count(), 2);
    }
    #[test]
    fn test_applyrules_stats_success() {
        let mut stats = ApplyRulesStats::new();
        stats.record_success(100);
        stats.record_success(200);
        assert_eq!(stats.total_ops, 2);
        assert_eq!(stats.successful_ops, 2);
        assert!((stats.success_rate() - 1.0).abs() < 1e-9);
    }
    #[test]
    fn test_applyrules_stats_failure() {
        let mut stats = ApplyRulesStats::new();
        stats.record_success(100);
        stats.record_failure();
        assert!((stats.success_rate() - 0.5).abs() < 1e-9);
    }
    #[test]
    fn test_applyrules_stats_merge() {
        let mut a = ApplyRulesStats::new();
        let mut b = ApplyRulesStats::new();
        a.record_success(100);
        b.record_failure();
        a.merge(&b);
        assert_eq!(a.total_ops, 2);
    }
    #[test]
    fn test_applyrules_priority_queue() {
        let mut pq = ApplyRulesPriorityQueue::new();
        pq.push(ApplyRulesUtil0::new(1, "low", 1), 1);
        pq.push(ApplyRulesUtil0::new(2, "high", 2), 100);
        let (_, p) = pq.pop().expect("collection should not be empty");
        assert_eq!(p, 100);
    }
    #[test]
    fn test_applyrules_hash() {
        let h1 = applyrules_hash("foo");
        let h2 = applyrules_hash("foo");
        assert_eq!(h1, h2);
        let h3 = applyrules_hash("bar");
        assert_ne!(h1, h3);
    }
    #[test]
    fn test_applyrules_valid_name() {
        assert!(applyrules_is_valid_name("foo_bar"));
        assert!(!applyrules_is_valid_name("foo-bar"));
        assert!(!applyrules_is_valid_name(""));
    }
    #[test]
    fn test_applyrules_join() {
        let parts = ["a", "b", "c"];
        assert_eq!(applyrules_join(&parts, ", "), "a, b, c");
    }
}
#[cfg(test)]
mod tacticapplyrules_analysis_tests {
    use crate::tactic::apply_rules::*;
    #[test]
    fn test_tacticapplyrules_result_ok() {
        let r = TacticApplyRulesResult::Ok("success".to_string());
        assert!(r.is_ok());
        assert!(!r.is_err());
        assert_eq!(r.ok_msg(), Some("success"));
        assert!((r.progress() - 1.0).abs() < 1e-10);
    }
    #[test]
    fn test_tacticapplyrules_result_err() {
        let r = TacticApplyRulesResult::Err("failure".to_string());
        assert!(r.is_err());
        assert_eq!(r.err_msg(), Some("failure"));
        assert!((r.progress() - 0.0).abs() < 1e-10);
    }
    #[test]
    fn test_tacticapplyrules_result_partial() {
        let r = TacticApplyRulesResult::Partial { done: 3, total: 10 };
        assert!(r.is_partial());
        assert!((r.progress() - 0.3).abs() < 1e-10);
    }
    #[test]
    fn test_tacticapplyrules_result_skipped() {
        let r = TacticApplyRulesResult::Skipped;
        assert!(r.is_skipped());
    }
    #[test]
    fn test_tacticapplyrules_analysis_pass_run() {
        let mut p = TacticApplyRulesAnalysisPass::new("test_pass");
        let r = p.run("hello");
        assert!(r.is_ok());
        assert_eq!(p.total_runs, 1);
        assert_eq!(p.success_count(), 1);
    }
    #[test]
    fn test_tacticapplyrules_analysis_pass_empty_input() {
        let mut p = TacticApplyRulesAnalysisPass::new("empty_test");
        let r = p.run("");
        assert!(r.is_err());
        assert_eq!(p.error_count(), 1);
    }
    #[test]
    fn test_tacticapplyrules_analysis_pass_success_rate() {
        let mut p = TacticApplyRulesAnalysisPass::new("rate_test");
        p.run("a");
        p.run("b");
        p.run("");
        assert!((p.success_rate() - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_tacticapplyrules_analysis_pass_disable() {
        let mut p = TacticApplyRulesAnalysisPass::new("disable_test");
        p.disable();
        assert!(!p.enabled);
        p.enable();
        assert!(p.enabled);
    }
    #[test]
    fn test_tacticapplyrules_pipeline_basic() {
        let mut pipeline = TacticApplyRulesPipeline::new("main_pipeline");
        pipeline.add_pass(TacticApplyRulesAnalysisPass::new("pass1"));
        pipeline.add_pass(TacticApplyRulesAnalysisPass::new("pass2"));
        assert_eq!(pipeline.num_passes(), 2);
        let results = pipeline.run_all("test_input");
        assert_eq!(results.len(), 2);
    }
    #[test]
    fn test_tacticapplyrules_pipeline_disabled_pass() {
        let mut pipeline = TacticApplyRulesPipeline::new("partial");
        let mut p = TacticApplyRulesAnalysisPass::new("disabled");
        p.disable();
        pipeline.add_pass(p);
        pipeline.add_pass(TacticApplyRulesAnalysisPass::new("enabled"));
        assert_eq!(pipeline.num_enabled_passes(), 1);
        let results = pipeline.run_all("input");
        assert_eq!(results.len(), 1);
    }
    #[test]
    fn test_tacticapplyrules_diff_basic() {
        let mut d = TacticApplyRulesDiff::new();
        d.add("new_item");
        d.remove("old_item");
        d.keep("same_item");
        assert!(!d.is_empty());
        assert_eq!(d.total_changes(), 2);
        assert_eq!(d.net_additions(), 0);
    }
    #[test]
    fn test_tacticapplyrules_diff_summary() {
        let mut d = TacticApplyRulesDiff::new();
        d.add("x");
        d.add("y");
        d.remove("z");
        let s = d.summary();
        assert!(s.contains("+2"));
    }
    #[test]
    fn test_tacticapplyrules_config_set_get() {
        let mut cfg = TacticApplyRulesConfig::new();
        cfg.set_bool("debug", true);
        cfg.set_int("max_iter", 100);
        cfg.set_str("name", "test");
        assert_eq!(cfg.get_bool("debug"), Some(true));
        assert_eq!(cfg.get_int("max_iter"), Some(100));
        assert_eq!(cfg.get_str("name"), Some("test"));
    }
    #[test]
    fn test_tacticapplyrules_config_read_only() {
        let mut cfg = TacticApplyRulesConfig::new();
        cfg.set_bool("key", true);
        cfg.lock();
        assert!(!cfg.set_bool("key", false));
        assert_eq!(cfg.get_bool("key"), Some(true));
        cfg.unlock();
        assert!(cfg.set_bool("key", false));
    }
    #[test]
    fn test_tacticapplyrules_config_remove() {
        let mut cfg = TacticApplyRulesConfig::new();
        cfg.set_int("x", 42);
        assert!(cfg.has("x"));
        cfg.remove("x");
        assert!(!cfg.has("x"));
    }
    #[test]
    fn test_tacticapplyrules_diagnostics_basic() {
        let mut diag = TacticApplyRulesDiagnostics::new(10);
        diag.error("something went wrong");
        diag.warning("maybe check this");
        diag.note("fyi");
        assert!(diag.has_errors());
        assert!(!diag.is_clean());
        assert_eq!(diag.num_errors(), 1);
        assert_eq!(diag.num_warnings(), 1);
    }
    #[test]
    fn test_tacticapplyrules_diagnostics_max_errors() {
        let mut diag = TacticApplyRulesDiagnostics::new(2);
        diag.error("e1");
        diag.error("e2");
        diag.error("e3");
        assert_eq!(diag.num_errors(), 2);
        assert!(diag.at_error_limit());
    }
    #[test]
    fn test_tacticapplyrules_diagnostics_clear() {
        let mut diag = TacticApplyRulesDiagnostics::new(10);
        diag.error("e1");
        diag.clear();
        assert!(diag.is_clean());
    }
    #[test]
    fn test_tacticapplyrules_config_value_types() {
        let b = TacticApplyRulesConfigValue::Bool(true);
        assert_eq!(b.type_name(), "bool");
        assert_eq!(b.as_bool(), Some(true));
        assert_eq!(b.as_int(), None);
        let i = TacticApplyRulesConfigValue::Int(42);
        assert_eq!(i.type_name(), "int");
        assert_eq!(i.as_int(), Some(42));
        let f = TacticApplyRulesConfigValue::Float(2.5);
        assert_eq!(f.type_name(), "float");
        assert!((f.as_float().expect("as_float should succeed") - 2.5).abs() < 1e-10);
        let s = TacticApplyRulesConfigValue::Str("hello".to_string());
        assert_eq!(s.type_name(), "str");
        assert_eq!(s.as_str(), Some("hello"));
        let l = TacticApplyRulesConfigValue::List(vec!["a".to_string(), "b".to_string()]);
        assert_eq!(l.type_name(), "list");
        assert_eq!(l.as_list().map(|v| v.len()), Some(2));
    }
}
#[cfg(test)]
mod apply_rules_ext_tests_3400 {
    use crate::tactic::apply_rules::*;
    #[test]
    fn test_apply_rules_ext_result_ok_3400() {
        let r = ApplyRulesExtResult3400::Ok("success".to_string());
        assert!(r.is_ok());
        assert!(!r.is_err());
        assert_eq!(r.ok_msg(), Some("success"));
        assert!((r.progress() - 1.0).abs() < 1e-10);
    }
    #[test]
    fn test_apply_rules_ext_result_err_3400() {
        let r = ApplyRulesExtResult3400::Err("failure".to_string());
        assert!(r.is_err());
        assert_eq!(r.err_msg(), Some("failure"));
        assert!((r.progress() - 0.0).abs() < 1e-10);
    }
    #[test]
    fn test_apply_rules_ext_result_partial_3400() {
        let r = ApplyRulesExtResult3400::Partial { done: 3, total: 10 };
        assert!(r.is_partial());
        assert!((r.progress() - 0.3).abs() < 1e-10);
    }
    #[test]
    fn test_apply_rules_ext_result_skipped_3400() {
        let r = ApplyRulesExtResult3400::Skipped;
        assert!(r.is_skipped());
    }
    #[test]
    fn test_apply_rules_ext_pass_run_3400() {
        let mut p = ApplyRulesExtPass3400::new("test_pass");
        let r = p.run("hello");
        assert!(r.is_ok());
        assert_eq!(p.total_runs, 1);
        assert_eq!(p.success_count(), 1);
    }
    #[test]
    fn test_apply_rules_ext_pass_empty_3400() {
        let mut p = ApplyRulesExtPass3400::new("empty_test");
        let r = p.run("");
        assert!(r.is_err());
        assert_eq!(p.error_count(), 1);
    }
    #[test]
    fn test_apply_rules_ext_pass_rate_3400() {
        let mut p = ApplyRulesExtPass3400::new("rate_test");
        p.run("a");
        p.run("b");
        p.run("");
        assert!((p.success_rate() - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_apply_rules_ext_pass_disable_3400() {
        let mut p = ApplyRulesExtPass3400::new("disable_test");
        p.disable();
        assert!(!p.enabled);
        p.enable();
        assert!(p.enabled);
    }
    #[test]
    fn test_apply_rules_ext_pipeline_basic_3400() {
        let mut pipeline = ApplyRulesExtPipeline3400::new("main_pipeline");
        pipeline.add_pass(ApplyRulesExtPass3400::new("pass1"));
        pipeline.add_pass(ApplyRulesExtPass3400::new("pass2"));
        assert_eq!(pipeline.num_passes(), 2);
        let results = pipeline.run_all("test_input");
        assert_eq!(results.len(), 2);
    }
    #[test]
    fn test_apply_rules_ext_pipeline_disabled_3400() {
        let mut pipeline = ApplyRulesExtPipeline3400::new("partial");
        let mut p = ApplyRulesExtPass3400::new("disabled");
        p.disable();
        pipeline.add_pass(p);
        pipeline.add_pass(ApplyRulesExtPass3400::new("enabled"));
        assert_eq!(pipeline.num_enabled_passes(), 1);
        let results = pipeline.run_all("input");
        assert_eq!(results.len(), 1);
    }
    #[test]
    fn test_apply_rules_ext_diff_basic_3400() {
        let mut d = ApplyRulesExtDiff3400::new();
        d.add("new_item");
        d.remove("old_item");
        d.keep("same_item");
        assert!(!d.is_empty());
        assert_eq!(d.total_changes(), 2);
        assert_eq!(d.net_additions(), 0);
    }
    #[test]
    fn test_apply_rules_ext_config_set_get_3400() {
        let mut cfg = ApplyRulesExtConfig3400::new();
        cfg.set_bool("debug", true);
        cfg.set_int("max_iter", 100);
        cfg.set_str("name", "test");
        assert_eq!(cfg.get_bool("debug"), Some(true));
        assert_eq!(cfg.get_int("max_iter"), Some(100));
        assert_eq!(cfg.get_str("name"), Some("test"));
    }
    #[test]
    fn test_apply_rules_ext_config_read_only_3400() {
        let mut cfg = ApplyRulesExtConfig3400::new();
        cfg.set_bool("key", true);
        cfg.lock();
        assert!(!cfg.set_bool("key", false));
        assert_eq!(cfg.get_bool("key"), Some(true));
        cfg.unlock();
        assert!(cfg.set_bool("key", false));
    }
    #[test]
    fn test_apply_rules_ext_config_remove_3400() {
        let mut cfg = ApplyRulesExtConfig3400::new();
        cfg.set_int("x", 42);
        assert!(cfg.has("x"));
        cfg.remove("x");
        assert!(!cfg.has("x"));
    }
    #[test]
    fn test_apply_rules_ext_diagnostics_basic_3400() {
        let mut diag = ApplyRulesExtDiag3400::new(10);
        diag.error("something went wrong");
        diag.warning("maybe check this");
        diag.note("fyi");
        assert!(diag.has_errors());
        assert!(!diag.is_clean());
        assert_eq!(diag.num_errors(), 1);
        assert_eq!(diag.num_warnings(), 1);
    }
    #[test]
    fn test_apply_rules_ext_diagnostics_max_errors_3400() {
        let mut diag = ApplyRulesExtDiag3400::new(2);
        diag.error("e1");
        diag.error("e2");
        diag.error("e3");
        assert_eq!(diag.num_errors(), 2);
        assert!(diag.at_error_limit());
    }
    #[test]
    fn test_apply_rules_ext_diagnostics_clear_3400() {
        let mut diag = ApplyRulesExtDiag3400::new(10);
        diag.error("e1");
        diag.clear();
        assert!(diag.is_clean());
    }
    #[test]
    fn test_apply_rules_ext_config_value_types_3400() {
        let b = ApplyRulesExtConfigVal3400::Bool(true);
        assert_eq!(b.type_name(), "bool");
        assert_eq!(b.as_bool(), Some(true));
        assert_eq!(b.as_int(), None);
        let i = ApplyRulesExtConfigVal3400::Int(42);
        assert_eq!(i.type_name(), "int");
        assert_eq!(i.as_int(), Some(42));
        let f = ApplyRulesExtConfigVal3400::Float(2.5);
        assert_eq!(f.type_name(), "float");
        assert!((f.as_float().expect("as_float should succeed") - 2.5).abs() < 1e-10);
        let s = ApplyRulesExtConfigVal3400::Str("hello".to_string());
        assert_eq!(s.type_name(), "str");
        assert_eq!(s.as_str(), Some("hello"));
        let l = ApplyRulesExtConfigVal3400::List(vec!["a".to_string(), "b".to_string()]);
        assert_eq!(l.type_name(), "list");
        assert_eq!(l.as_list().map(|v| v.len()), Some(2));
    }
}
