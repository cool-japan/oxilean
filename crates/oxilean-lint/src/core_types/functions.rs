//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

pub use crate::framework::{
    AutoFix, LintConfig, LintContext, LintDiagnostic, LintEngine, LintId, LintRegistry, LintRule,
    LintSuppression, Severity,
};

use super::types::{
    LintAggregator, LintBaseline, LintBudget, LintCategory, LintConfigBuilder, LintConfigValidator,
    LintCooldown, LintDatabase, LintDiff, LintEntry, LintEventKind, LintEventLog, LintFilter,
    LintFormatter, LintIgnoreList, LintLevel, LintMetadata, LintOutputFormat, LintPass,
    LintPriorityQueue, LintProfile, LintReport, LintResult, LintRuleGroup, LintRuleMetadata,
    LintRuleSet, LintRunOptions, LintRunSummary, LintSessionContext, LintStats, LintSummaryReport,
    LintSuppressAnnotation, LintTrend,
};

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_lint_pass_new() {
        let pass = LintPass::new("style");
        assert_eq!(pass.name, "style");
        assert!(pass.enabled);
        assert!(!pass.can_fix);
    }
    #[test]
    fn test_lint_pass_with_lint() {
        let pass = LintPass::new("unused").with_lint("unused_variable");
        assert_eq!(pass.lint_ids.len(), 1);
        assert_eq!(pass.lint_ids[0].as_str(), "unused_variable");
    }
    #[test]
    fn test_lint_pass_disabled() {
        let pass = LintPass::new("x").disabled();
        assert!(!pass.enabled);
    }
    #[test]
    fn test_lint_pass_with_fixes() {
        let pass = LintPass::new("x").with_fixes();
        assert!(pass.can_fix);
    }
    #[test]
    fn test_lint_category_display() {
        assert_eq!(format!("{}", LintCategory::Correctness), "correctness");
        assert_eq!(format!("{}", LintCategory::Style), "style");
        assert_eq!(format!("{}", LintCategory::Naming), "naming");
    }
    #[test]
    fn test_lint_metadata_new() {
        let meta = LintMetadata::new(
            "unused_variable",
            LintCategory::Redundancy,
            "Unused variable",
            Severity::Warning,
        );
        assert_eq!(meta.id.as_str(), "unused_variable");
        assert_eq!(meta.severity, Severity::Warning);
        assert!(!meta.fixable);
    }
    #[test]
    fn test_lint_metadata_fixable() {
        let meta = LintMetadata::new("x", LintCategory::Style, "X", Severity::Hint).fixable();
        assert!(meta.fixable);
    }
    #[test]
    fn test_lint_metadata_with_explanation() {
        let meta = LintMetadata::new("x", LintCategory::Style, "X", Severity::Hint)
            .with_explanation("More details here.");
        assert!(!meta.explanation.is_empty());
    }
    #[test]
    fn test_lint_stats_default() {
        let s = LintStats::new();
        assert!(!s.has_errors());
        assert!(s.is_clean());
        assert_eq!(s.total_diagnostics, 0);
    }
    #[test]
    fn test_lint_stats_record_error() {
        let mut s = LintStats::new();
        s.record(Severity::Error);
        assert!(s.has_errors());
        assert!(!s.is_clean());
        assert_eq!(s.errors, 1);
    }
    #[test]
    fn test_lint_stats_record_warning() {
        let mut s = LintStats::new();
        s.record(Severity::Warning);
        assert!(!s.has_errors());
        assert!(!s.is_clean());
        assert_eq!(s.warnings, 1);
    }
    #[test]
    fn test_lint_stats_display() {
        let mut s = LintStats::new();
        s.record(Severity::Error);
        s.record(Severity::Warning);
        let text = format!("{}", s);
        assert!(text.contains("total: 2"));
        assert!(text.contains("errors: 1"));
    }
    #[test]
    fn test_lint_suppress_annotation_single() {
        let ann = LintSuppressAnnotation::single("unused_variable", 5);
        assert_eq!(ann.ids.len(), 1);
        assert_eq!(ann.line, 5);
        assert!(!ann.is_file_level);
    }
    #[test]
    fn test_lint_suppress_annotation_file_level() {
        let ann = LintSuppressAnnotation::file_level(vec!["dead_code", "unused_import"]);
        assert_eq!(ann.ids.len(), 2);
        assert!(ann.is_file_level);
    }
    #[test]
    fn test_lint_suppress_annotation_suppresses() {
        let ann = LintSuppressAnnotation::single("unused_variable", 0);
        assert!(ann.suppresses(&LintId::new("unused_variable")));
        assert!(!ann.suppresses(&LintId::new("dead_code")));
    }
    #[test]
    fn test_lint_report_empty() {
        let r = LintReport::empty("foo.ox");
        assert_eq!(r.filename, "foo.ox");
        assert!(r.is_clean());
        assert!(r.diagnostics.is_empty());
    }
    #[test]
    fn test_lint_report_display() {
        let r = LintReport::empty("bar.ox");
        let s = format!("{}", r);
        assert!(s.contains("bar.ox"));
    }
    #[test]
    fn test_lint_id_matches_pattern_wildcard() {
        let id = LintId::new("unused_variable");
        assert!(id.matches_pattern("*"));
        assert!(id.matches_pattern("unused_*"));
        assert!(!id.matches_pattern("dead_*"));
    }
    #[test]
    fn test_lint_stats_hints_and_infos() {
        let mut s = LintStats::new();
        s.record(Severity::Info);
        s.record(Severity::Hint);
        assert_eq!(s.infos, 1);
        assert_eq!(s.hints, 1);
        assert!(s.is_clean());
    }
    #[test]
    fn test_lint_metadata_with_reference() {
        let meta = LintMetadata::new("x", LintCategory::Correctness, "x", Severity::Error)
            .with_reference("https://oxilean.org/lint/x");
        assert_eq!(meta.references.len(), 1);
    }
}
#[cfg(test)]
mod extra_tests {
    use super::*;
    #[test]
    fn test_lint_rule_set_new() {
        let s = LintRuleSet::new("default");
        assert_eq!(s.name, "default");
        assert!(s.is_empty());
    }
    #[test]
    fn test_lint_rule_set_add() {
        let mut s = LintRuleSet::new("style");
        s.add("unused_variable");
        assert_eq!(s.len(), 1);
        assert!(s.contains(&LintId::new("unused_variable")));
    }
    #[test]
    fn test_lint_rule_set_contains_false() {
        let s = LintRuleSet::new("x");
        assert!(!s.contains(&LintId::new("nonexistent")));
    }
    #[test]
    fn test_lint_rule_set_display() {
        let mut s = LintRuleSet::new("perf");
        s.add("redundant_clone");
        let txt = format!("{}", s);
        assert!(txt.contains("perf"));
        assert!(txt.contains("1 rules"));
    }
    #[test]
    fn test_lint_level_ordering() {
        assert!(LintLevel::Deny > LintLevel::Warn);
        assert!(LintLevel::Warn > LintLevel::Allow);
    }
    #[test]
    fn test_lint_level_display() {
        assert_eq!(format!("{}", LintLevel::Allow), "allow");
        assert_eq!(format!("{}", LintLevel::Warn), "warn");
        assert_eq!(format!("{}", LintLevel::Deny), "deny");
    }
    #[test]
    fn test_lint_pass_multiple_lints() {
        let pass = LintPass::new("all")
            .with_lint("unused_variable")
            .with_lint("dead_code")
            .with_lint("unused_import");
        assert_eq!(pass.lint_ids.len(), 3);
    }
    #[test]
    fn test_lint_report_add_and_severity() {
        use crate::framework::Severity;
        let r = LintReport::empty("test.ox");
        assert!(r.is_clean());
        let _ = r.at_severity(Severity::Error);
    }
    #[test]
    fn test_lint_stats_multiple_records() {
        use crate::framework::Severity;
        let mut s = LintStats::new();
        s.record(Severity::Error);
        s.record(Severity::Warning);
        s.record(Severity::Info);
        s.record(Severity::Hint);
        assert_eq!(s.total_diagnostics, 4);
        assert_eq!(s.errors, 1);
        assert_eq!(s.warnings, 1);
        assert_eq!(s.infos, 1);
        assert_eq!(s.hints, 1);
    }
}
#[cfg(test)]
mod lint_result_tests {
    use super::*;
    use crate::framework::{LintId, Severity};
    fn mk_diag(sev: Severity) -> LintDiagnostic {
        use crate::framework::SourceRange;
        LintDiagnostic::new(
            LintId::new("test"),
            sev,
            "test message",
            SourceRange::default(),
        )
    }
    #[test]
    fn test_lint_result_empty() {
        let r = LintResult::new();
        assert!(r.is_clean());
        assert_eq!(r.len(), 0);
    }
    #[test]
    fn test_lint_result_add() {
        let mut r = LintResult::new();
        r.add(mk_diag(Severity::Warning));
        assert!(r.has_diagnostics());
        assert_eq!(r.len(), 1);
    }
    #[test]
    fn test_lint_result_at_severity() {
        let mut r = LintResult::new();
        r.add(mk_diag(Severity::Warning));
        r.add(mk_diag(Severity::Error));
        let errors = r.at_severity(Severity::Error);
        assert_eq!(errors.len(), 1);
    }
    #[test]
    fn test_lint_result_merge() {
        let mut r1 = LintResult::new();
        let mut r2 = LintResult::new();
        r1.add(mk_diag(Severity::Warning));
        r2.add(mk_diag(Severity::Error));
        r1.merge(r2);
        assert_eq!(r1.len(), 2);
    }
    #[test]
    fn test_lint_result_display() {
        let r = LintResult::new();
        let s = format!("{}", r);
        assert!(s.contains("LintResult"));
    }
    #[test]
    fn test_lint_config_builder() {
        let cfg = LintConfigBuilder::new()
            .allow("dead_code")
            .deny("unused_variable")
            .build();
        assert!(cfg.is_allowed(&LintId::new("dead_code")));
        assert!(cfg.is_denied(&LintId::new("unused_variable")));
    }
    #[test]
    fn test_lint_category_all_variants() {
        let cats = vec![
            LintCategory::Correctness,
            LintCategory::Style,
            LintCategory::Performance,
            LintCategory::Complexity,
            LintCategory::Deprecation,
            LintCategory::Documentation,
            LintCategory::Naming,
            LintCategory::Redundancy,
        ];
        for cat in cats {
            let s = format!("{}", cat);
            assert!(!s.is_empty());
        }
    }
    #[test]
    fn test_lint_suppress_annotation_suppresses_false() {
        let ann = LintSuppressAnnotation::single("unused_variable", 0);
        assert!(!ann.suppresses(&LintId::new("dead_code")));
    }
    #[test]
    fn test_lint_rule_set_add_multiple() {
        let mut s = LintRuleSet::new("default");
        for name in ["a", "b", "c", "d"] {
            s.add(name);
        }
        assert_eq!(s.len(), 4);
    }
}
#[cfg(test)]
mod lint_profile_tests {
    use super::*;
    fn mk_diag_with_id(id: &str, sev: Severity) -> LintDiagnostic {
        use crate::framework::{LintId, SourceRange};
        LintDiagnostic::new(LintId::new(id), sev, "msg", SourceRange::default())
    }
    #[test]
    fn test_lint_filter_no_constraints() {
        let filter = LintFilter::new();
        let diag = mk_diag_with_id("unused_variable", Severity::Warning);
        assert!(filter.accepts(&diag));
    }
    #[test]
    fn test_lint_filter_min_severity_ok() {
        let filter = LintFilter::new().min_severity(Severity::Warning);
        let warn = mk_diag_with_id("x", Severity::Warning);
        let info = mk_diag_with_id("x", Severity::Info);
        assert!(filter.accepts(&warn));
        assert!(!filter.accepts(&info));
    }
    #[test]
    fn test_lint_filter_include_pattern() {
        let filter = LintFilter::new().include("unused_*");
        let pass = mk_diag_with_id("unused_variable", Severity::Warning);
        let fail = mk_diag_with_id("dead_code", Severity::Warning);
        assert!(filter.accepts(&pass));
        assert!(!filter.accepts(&fail));
    }
    #[test]
    fn test_lint_filter_exclude_pattern() {
        let filter = LintFilter::new().exclude("dead_*");
        let pass = mk_diag_with_id("unused_variable", Severity::Warning);
        let fail = mk_diag_with_id("dead_code", Severity::Warning);
        assert!(filter.accepts(&pass));
        assert!(!filter.accepts(&fail));
    }
    #[test]
    fn test_lint_filter_apply() {
        let filter = LintFilter::new().min_severity(Severity::Error);
        let diags = vec![
            mk_diag_with_id("a", Severity::Error),
            mk_diag_with_id("b", Severity::Warning),
            mk_diag_with_id("c", Severity::Info),
        ];
        let accepted = filter.apply(&diags);
        assert_eq!(accepted.len(), 1);
    }
    #[test]
    fn test_lint_output_format_display() {
        assert_eq!(format!("{}", LintOutputFormat::Text), "text");
        assert_eq!(format!("{}", LintOutputFormat::Json), "json");
        assert_eq!(format!("{}", LintOutputFormat::Count), "count");
    }
    #[test]
    fn test_lint_output_format_from_str() {
        assert_eq!(
            LintOutputFormat::parse("json"),
            Some(LintOutputFormat::Json)
        );
        assert_eq!(LintOutputFormat::parse("unknown"), None);
    }
    #[test]
    fn test_lint_profile_basic() {
        let profile = LintProfile::new("strict");
        assert_eq!(profile.name, "strict");
        assert!(profile.rule_sets.is_empty());
    }
    #[test]
    fn test_lint_profile_with_rule_set() {
        let mut rs = LintRuleSet::new("style");
        rs.add("unused_variable");
        rs.add("dead_code");
        let profile = LintProfile::new("standard").with_rule_set(rs);
        assert_eq!(profile.all_ids().len(), 2);
    }
    #[test]
    fn test_lint_profile_overrides() {
        let profile = LintProfile::new("strict").with_override("dead_code", LintLevel::Deny);
        let id = LintId::new("dead_code");
        assert_eq!(profile.effective_level(&id), Some(LintLevel::Deny));
        let id2 = LintId::new("nonexistent");
        assert_eq!(profile.effective_level(&id2), None);
    }
    #[test]
    fn test_lint_stats_is_clean_after_info_only() {
        let mut s = LintStats::new();
        s.record(Severity::Info);
        assert!(s.is_clean());
    }
    #[test]
    fn test_lint_filter_both_include_and_exclude() {
        let filter = LintFilter::new()
            .include("unused_*")
            .exclude("unused_import");
        let pass = mk_diag_with_id("unused_variable", Severity::Hint);
        let excluded = mk_diag_with_id("unused_import", Severity::Hint);
        assert!(filter.accepts(&pass));
        assert!(!filter.accepts(&excluded));
    }
}
#[cfg(test)]
mod lib_extended_tests {
    use super::*;
    fn mk_diag(id: &str, severity: Severity) -> LintDiagnostic {
        use crate::framework::SourceRange;
        LintDiagnostic::new(
            LintId::new(id),
            severity,
            "test",
            SourceRange::with_file(0, 0, "test.ox".to_string()),
        )
    }
    #[test]
    fn lint_database_register_and_get() {
        let mut db = LintDatabase::new();
        let entry = LintEntry::new("unused_import", "Remove unused imports", Severity::Warning)
            .with_tag("style")
            .with_autofix();
        db.register(entry);
        assert!(!db.is_empty());
        let found = db.get("unused_import").expect("key should exist");
        assert!(found.has_autofix);
        assert!(found.tags.contains(&"style".to_string()));
    }
    #[test]
    fn lint_database_by_tag() {
        let mut db = LintDatabase::new();
        db.register(LintEntry::new("a", "a", Severity::Info).with_tag("security"));
        db.register(LintEntry::new("b", "b", Severity::Info).with_tag("style"));
        db.register(LintEntry::new("c", "c", Severity::Info).with_tag("security"));
        let sec = db.by_tag("security");
        assert_eq!(sec.len(), 2);
    }
    #[test]
    fn lint_database_with_autofix() {
        let mut db = LintDatabase::new();
        db.register(LintEntry::new("fixable", "fixable", Severity::Warning).with_autofix());
        db.register(LintEntry::new("not_fixable", "no fix", Severity::Warning));
        let fixable = db.with_autofix();
        assert_eq!(fixable.len(), 1);
    }
    #[test]
    fn lint_run_options_default() {
        let opts = LintRunOptions::default_opts();
        assert!(opts.include_info);
        assert!(!opts.include_hints);
        assert!(!opts.auto_apply_fixes);
        assert!(!opts.fail_fast);
    }
    #[test]
    fn lint_run_options_strict() {
        let opts = LintRunOptions::strict();
        assert!(opts.include_hints);
        assert!(opts.fail_fast);
    }
    #[test]
    fn lint_category_display() {
        assert_eq!(format!("{}", LintCategory::Style), "style");
        assert_eq!(format!("{}", LintCategory::Security), "security");
        assert_eq!(
            format!("{}", LintCategory::Custom("my_cat".to_string())),
            "custom:my_cat"
        );
    }
    #[test]
    fn lint_summary_report_add() {
        let mut report = LintSummaryReport::new();
        let diag = mk_diag("test", Severity::Warning);
        report.add(&diag);
        assert_eq!(report.total_diagnostics, 1);
        assert!(!report.is_clean());
    }
    #[test]
    fn lint_summary_report_clean_with_info_only() {
        let mut report = LintSummaryReport::new();
        report.add(&mk_diag("test", Severity::Info));
        assert!(report.is_clean());
    }
    #[test]
    fn lint_ignore_list_filters() {
        let mut ignore = LintIgnoreList::new();
        ignore.ignore("dead_code");
        ignore.ignore("unused_import");
        let diags = vec![
            mk_diag("dead_code", Severity::Warning),
            mk_diag("naming_convention", Severity::Warning),
        ];
        let filtered = ignore.filter(&diags);
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].lint_id.as_str(), "naming_convention");
    }
    #[test]
    fn lint_ignore_list_is_ignored() {
        let mut ignore = LintIgnoreList::new();
        ignore.ignore("foo");
        assert!(ignore.is_ignored("foo"));
        assert!(!ignore.is_ignored("bar"));
        assert_eq!(ignore.len(), 1);
    }
    #[test]
    fn lint_output_format_display() {
        assert_eq!(format!("{}", LintOutputFormat::Text), "text");
        assert_eq!(format!("{}", LintOutputFormat::Json), "json");
        assert_eq!(
            format!("{}", LintOutputFormat::GitHubActions),
            "github-actions"
        );
        assert_eq!(format!("{}", LintOutputFormat::Count), "count");
    }
    #[test]
    fn lint_formatter_text() {
        let formatter = LintFormatter::new(LintOutputFormat::Text);
        let diag = mk_diag("unused_import", Severity::Warning);
        let output = formatter.format_one(&diag);
        assert!(output.contains("unused_import"));
        assert!(output.contains("test.ox"));
    }
    #[test]
    fn lint_formatter_github() {
        let formatter = LintFormatter::new(LintOutputFormat::GitHubActions);
        let diag = mk_diag("unused_import", Severity::Warning);
        let output = formatter.format_one(&diag);
        assert!(output.starts_with("::warning"));
    }
    #[test]
    fn lint_formatter_json() {
        let formatter = LintFormatter::new(LintOutputFormat::Json);
        let diag = mk_diag("foo", Severity::Error);
        let output = formatter.format_one(&diag);
        assert!(output.starts_with('{'));
        assert!(output.contains("\"id\":\"foo\""));
    }
    #[test]
    fn lint_formatter_compact() {
        let formatter = LintFormatter::new(LintOutputFormat::Count);
        let diag = mk_diag("bar", Severity::Info);
        let output = formatter.format_one(&diag);
        assert!(output.contains("bar"));
    }
    #[test]
    fn lint_formatter_format_all() {
        let formatter = LintFormatter::new(LintOutputFormat::Count);
        let diags = vec![
            mk_diag("a", Severity::Warning),
            mk_diag("b", Severity::Info),
        ];
        let output = formatter.format_all(&diags);
        assert!(output.contains('\n'));
    }
    #[test]
    fn lint_trend_improving() {
        let mut trend = LintTrend::new();
        trend.record("v1", 10);
        trend.record("v2", 5);
        assert!(trend.is_improving());
        assert_eq!(trend.latest_count(), 5);
        assert_eq!(trend.snapshot_count(), 2);
    }
    #[test]
    fn lint_trend_not_improving() {
        let mut trend = LintTrend::new();
        trend.record("v1", 3);
        trend.record("v2", 7);
        assert!(!trend.is_improving());
    }
    #[test]
    fn lint_baseline_filters_known() {
        let diag = mk_diag("dead_code", Severity::Warning);
        let mut baseline = LintBaseline::new();
        baseline.add(&diag);
        assert!(baseline.is_known(&diag));
        let new_diag = mk_diag("new_lint", Severity::Warning);
        assert!(!baseline.is_known(&new_diag));
        let all = vec![diag, new_diag];
        let new_only = baseline.new_diagnostics(&all);
        assert_eq!(new_only.len(), 1);
        assert_eq!(new_only[0].lint_id.as_str(), "new_lint");
    }
    #[test]
    fn lint_rule_group_contains() {
        let mut group = LintRuleGroup::new("style", "Style rules");
        group.add_rule("naming_convention");
        group.add_rule("unused_import");
        assert!(group.contains("naming_convention"));
        assert!(!group.contains("dead_code"));
        assert_eq!(group.rule_count(), 2);
    }
    #[test]
    fn lint_aggregator_collects() {
        let mut agg = LintAggregator::new();
        agg.add(mk_diag("a", Severity::Warning));
        agg.add(mk_diag("b", Severity::Error));
        agg.add_all(vec![mk_diag("c", Severity::Info)]);
        assert_eq!(agg.count(), 3);
        assert_eq!(agg.count_by_severity(Severity::Warning), 1);
        assert_eq!(agg.count_by_severity(Severity::Error), 1);
    }
    #[test]
    fn lint_aggregator_into_diagnostics() {
        let mut agg = LintAggregator::new();
        agg.add(mk_diag("x", Severity::Info));
        let diags = agg.into_diagnostics();
        assert_eq!(diags.len(), 1);
    }
    #[test]
    fn lint_event_log_basic() {
        let mut log = LintEventLog::new();
        let id = log.log(LintEventKind::RuleStarted, "checking naming_convention");
        assert_eq!(log.total(), 1);
        assert_eq!(log.events()[0].id, id);
    }
    #[test]
    fn lint_diff_no_change() {
        let fingerprints = vec!["a".to_string(), "b".to_string()];
        let diff = LintDiff::compute(&fingerprints, &fingerprints);
        assert!(diff.is_empty());
    }
    #[test]
    fn lint_diff_new_and_removed() {
        let before = vec!["a".to_string(), "b".to_string()];
        let after = vec!["b".to_string(), "c".to_string()];
        let diff = LintDiff::compute(&before, &after);
        assert!(!diff.is_empty());
        assert!(diff.added.contains(&"c".to_string()));
        assert!(diff.removed.contains(&"a".to_string()));
    }
}
#[cfg(test)]
mod lib_final_tests {
    use super::*;
    fn mk_diag(id: &str, severity: Severity) -> LintDiagnostic {
        use crate::framework::SourceRange;
        LintDiagnostic::new(LintId::new(id), severity, "test", SourceRange::new(0, 0))
    }
    #[test]
    fn lint_rule_metadata_basic() {
        let meta = LintRuleMetadata::new(
            "unused_import",
            "Unused Import",
            LintCategory::Style,
            Severity::Warning,
        )
        .with_description("Detects unused imports.")
        .with_rationale("Unused imports add noise.")
        .with_example("simple", "import Unused", "-- no import");
        assert_eq!(meta.id.as_str().to_string(), "unused_import");
        assert_eq!(meta.examples.len(), 1);
        assert!(!meta.deprecated);
    }
    #[test]
    fn lint_rule_metadata_deprecated() {
        let meta =
            LintRuleMetadata::new("old_lint", "Old Lint", LintCategory::Style, Severity::Info)
                .mark_deprecated();
        assert!(meta.deprecated);
    }
    #[test]
    fn lint_priority_queue_orders_by_severity() {
        let mut pq = LintPriorityQueue::new();
        pq.push(mk_diag("info_lint", Severity::Info));
        pq.push(mk_diag("error_lint", Severity::Error));
        pq.push(mk_diag("warning_lint", Severity::Warning));
        let first = pq.pop().expect("queue should not be empty");
        assert_eq!(first.severity, Severity::Error);
        let second = pq.pop().expect("queue should not be empty");
        assert_eq!(second.severity, Severity::Warning);
    }
    #[test]
    fn lint_priority_queue_empty() {
        let mut pq = LintPriorityQueue::new();
        assert!(pq.is_empty());
        assert!(pq.pop().is_none());
    }
    #[test]
    fn lint_budget_total_limit() {
        let mut budget = LintBudget::new(2, 10);
        assert!(budget.try_spend("a.ox"));
        assert!(budget.try_spend("b.ox"));
        assert!(!budget.try_spend("c.ox"));
        assert_eq!(budget.remaining_total(), 0);
    }
    #[test]
    fn lint_budget_per_file_limit() {
        let mut budget = LintBudget::new(100, 2);
        assert!(budget.try_spend("a.ox"));
        assert!(budget.try_spend("a.ox"));
        assert!(!budget.try_spend("a.ox"));
    }
    #[test]
    fn lint_cooldown_emits_once_then_suppresses() {
        let mut cd = LintCooldown::new(3);
        assert!(cd.should_emit("lint:a.ox:1"));
        assert!(!cd.should_emit("lint:a.ox:1"));
        cd.tick();
        cd.tick();
        cd.tick();
        assert!(cd.should_emit("lint:a.ox:1"));
    }
    #[test]
    fn lint_cooldown_different_fingerprints() {
        let mut cd = LintCooldown::new(5);
        assert!(cd.should_emit("fp1"));
        assert!(cd.should_emit("fp2"));
    }
}
#[cfg(test)]
mod lint_session_tests {
    use super::*;
    #[test]
    fn lint_session_context_average() {
        let mut ctx = LintSessionContext::new("sess-1");
        ctx.record_file(10, 50);
        ctx.record_file(20, 100);
        assert_eq!(ctx.files_processed, 2);
        assert!((ctx.average_diagnostics_per_file() - 15.0).abs() < 1e-9);
        assert_eq!(ctx.elapsed_ms, 150);
    }
    #[test]
    fn lint_config_builder_builds() {
        let config = LintConfigBuilder::new()
            .allow("unused_import")
            .deny("dead_code")
            .build();
        assert_eq!(config.enabled.len(), 1);
        assert_eq!(config.disabled.len(), 1);
    }
    #[test]
    fn lint_config_validator_no_conflict() {
        let config = LintConfigBuilder::new()
            .allow("lint_a")
            .allow("lint_b")
            .build();
        let errors = LintConfigValidator::validate(&config);
        assert!(errors.is_empty());
    }
    #[test]
    fn lint_config_validator_with_conflict() {
        let config = LintConfigBuilder::new()
            .allow("conflict_lint")
            .deny("conflict_lint")
            .build();
        let errors = LintConfigValidator::validate(&config);
        assert!(!errors.is_empty());
        assert!(errors[0].contains("conflict_lint"));
    }
}
#[cfg(test)]
mod lint_run_summary_tests {
    use super::*;
    #[test]
    fn lint_run_summary_is_success() {
        let mut s = LintRunSummary::new();
        assert!(s.is_success());
        s.errors = 1;
        assert!(!s.is_success());
    }
    #[test]
    fn lint_run_summary_throughput() {
        let s = LintRunSummary {
            total_diagnostics: 100,
            elapsed_ms: 50,
            ..LintRunSummary::new()
        };
        assert!((s.throughput() - 2.0).abs() < 1e-9);
    }
    #[test]
    fn lint_run_summary_zero_elapsed() {
        let s = LintRunSummary::new();
        assert_eq!(s.throughput(), 0.0);
    }
}
