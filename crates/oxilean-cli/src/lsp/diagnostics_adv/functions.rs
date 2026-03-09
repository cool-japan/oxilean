//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lsp::{
    analyze_document, DiagnosticSeverity, Document, DocumentStore, JsonValue, Location, Range,
    SymbolKind, TextEdit,
};
use oxilean_kernel::{Environment, Name};
use std::collections::{HashMap, HashSet};

use super::types::{
    AdvDiagnostic, AdvDiagnosticCache, AdvDiagnosticCode, AdvDiagnosticCollector,
    DiagnosticAnnotator, DiagnosticExportFormat, DiagnosticExporter, DiagnosticGroup,
    DiagnosticHeatmap, DiagnosticQuery, DiagnosticRateLimiter, DiagnosticState,
    DiagnosticStateMachine, DiagnosticSummary, DiagnosticSummaryReport, DiagnosticSuppression,
    DiagnosticTrend, FixKind, SeverityConfig, SuppressionRegistry, TrendDirection,
};

/// Helper function.
pub fn closing_for(ch: char) -> char {
    match ch {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        _ => ch,
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn make_doc(content: &str) -> Document {
        Document::new("file:///test.lean", 1, content)
    }
    #[test]
    fn test_severity_config_default() {
        let config = SeverityConfig::default();
        assert!(!config.warnings_as_errors);
        assert!(!config.suppress_hints);
    }
    #[test]
    fn test_severity_config_suppress() {
        let mut config = SeverityConfig::new();
        config.suppress("W001");
        assert!(config.is_suppressed("W001"));
        assert!(!config.is_suppressed("W002"));
        config.unsuppress("W001");
        assert!(!config.is_suppressed("W001"));
    }
    #[test]
    fn test_severity_config_override() {
        let mut config = SeverityConfig::new();
        config.set_override("W001", DiagnosticSeverity::Error);
        let effective = config.effective_severity("W001", DiagnosticSeverity::Warning);
        assert_eq!(effective, Some(DiagnosticSeverity::Error));
    }
    #[test]
    fn test_severity_config_warnings_as_errors() {
        let mut config = SeverityConfig::new();
        config.warnings_as_errors = true;
        let effective = config.effective_severity("W999", DiagnosticSeverity::Warning);
        assert_eq!(effective, Some(DiagnosticSeverity::Error));
    }
    #[test]
    fn test_adv_diagnostic_code_str() {
        assert_eq!(AdvDiagnosticCode::LexError.as_str(), "E001");
        assert_eq!(AdvDiagnosticCode::UnusedVariable.as_str(), "W001");
        assert_eq!(AdvDiagnosticCode::StyleNaming.as_str(), "S001");
    }
    #[test]
    fn test_adv_diagnostic_collector_lex_errors() {
        let env = Environment::new();
        let collector = AdvDiagnosticCollector::new(&env);
        let doc = make_doc("def x := 42");
        let diags = collector.collect_lex_errors(&doc);
        assert!(diags.is_empty());
    }
    #[test]
    fn test_adv_diagnostic_collector_style() {
        let env = Environment::new();
        let mut collector = AdvDiagnosticCollector::new(&env);
        collector.set_max_line_length(10);
        let doc = make_doc("def x := this_is_a_very_long_line_that_exceeds_limits");
        let diags = collector.collect_style_warnings(&doc);
        assert!(!diags.is_empty());
    }
    #[test]
    fn test_adv_diagnostic_collector_trailing_whitespace() {
        let env = Environment::new();
        let collector = AdvDiagnosticCollector::new(&env);
        let doc = make_doc("def x := 1   \ndef y := 2");
        let diags = collector.collect_style_warnings(&doc);
        let trailing = diags
            .iter()
            .filter(|d| d.code == AdvDiagnosticCode::StyleTrailingWhitespace)
            .count();
        assert!(trailing > 0);
    }
    #[test]
    fn test_diagnostic_summary() {
        let summary = DiagnosticSummary {
            errors: 3,
            warnings: 5,
            info: 2,
            hints: 1,
            files_with_issues: 2,
            total_files: 4,
        };
        assert!(summary.has_errors());
        assert_eq!(summary.total(), 11);
        let formatted = summary.format();
        assert!(formatted.contains("3 error(s)"));
    }
    #[test]
    fn test_adv_diagnostic_cache() {
        let mut cache = AdvDiagnosticCache::new();
        assert!(cache.is_empty());
        cache.store("file:///a.lean", 1, Vec::new());
        assert_eq!(cache.len(), 1);
        assert!(cache.get("file:///a.lean", 1).is_some());
        assert!(cache.get("file:///a.lean", 2).is_none());
        cache.invalidate("file:///a.lean");
        assert!(cache.is_empty());
    }
    #[test]
    fn test_fix_kind_str() {
        assert_eq!(FixKind::QuickFix.as_str(), "quickfix");
        assert_eq!(FixKind::OrganizeImports.as_str(), "source.organizeImports");
    }
    #[test]
    fn test_adv_diagnostic_to_json() {
        let diag = AdvDiagnostic {
            code: AdvDiagnosticCode::TypeError,
            severity: DiagnosticSeverity::Error,
            range: Range::single_line(0, 0, 5),
            message: "type mismatch".to_string(),
            related: Vec::new(),
            fixes: Vec::new(),
            tags: Vec::new(),
            uri: "file:///test.lean".to_string(),
        };
        let json = diag.to_json();
        let msg = json.get("message").and_then(|v| v.as_str());
        assert_eq!(msg, Some("type mismatch"));
    }
}
/// A composable filter for advanced diagnostics.
#[allow(dead_code)]
pub trait DiagnosticFilter: Send + Sync {
    /// Return true if this diagnostic passes the filter.
    fn accepts(&self, diag: &AdvDiagnostic) -> bool;
}
/// Return the diagnostics_adv module version.
#[allow(dead_code)]
pub fn diagnostics_adv_version() -> &'static str {
    "0.1.1"
}
#[cfg(test)]
mod adv_extra_tests {
    use super::*;
    use crate::lsp::diagnostics_adv::*;
    use crate::lsp::DiagnosticSeverity;
    fn make_diag(line: u32, msg: &str) -> AdvDiagnostic {
        AdvDiagnostic {
            code: AdvDiagnosticCode::TypeError,
            severity: DiagnosticSeverity::Error,
            range: Range::single_line(line, 0, 5),
            message: msg.to_string(),
            related: vec![],
            fixes: vec![],
            tags: vec![],
            uri: "file:///test.lean".to_string(),
        }
    }
    #[test]
    fn test_diagnostic_trend() {
        let mut trend = DiagnosticTrend::new("file:///a.lean", 5);
        trend.record(3);
        trend.record(4);
        trend.record(5);
        assert_eq!(trend.direction(), TrendDirection::Increasing);
        assert!((trend.moving_average() - 4.0).abs() < 0.01);
    }
    #[test]
    fn test_diagnostic_trend_decreasing() {
        let mut trend = DiagnosticTrend::new("file:///a.lean", 5);
        trend.record(10);
        trend.record(5);
        trend.record(2);
        assert_eq!(trend.direction(), TrendDirection::Decreasing);
    }
    #[test]
    fn test_suppression_registry() {
        let mut reg = SuppressionRegistry::new();
        reg.add(DiagnosticSuppression::new(
            "file:///test.lean",
            0,
            10,
            vec![AdvDiagnosticCode::TypeError],
            "intentional",
        ));
        let d = make_diag(5, "msg");
        assert!(reg.is_suppressed(&d));
        let d2 = make_diag(20, "msg");
        assert!(!reg.is_suppressed(&d2));
    }
    #[test]
    fn test_diagnostic_annotator() {
        let d = make_diag(0, "boom");
        let mut ann = DiagnosticAnnotator::new();
        ann.add_from_diagnostic(&d);
        let src = "let x : Nat := 1";
        let rendered = ann.render(src);
        assert!(rendered.contains("boom"));
        assert!(rendered.contains('^'));
    }
    #[test]
    fn test_diagnostic_exporter_plain() {
        let d = make_diag(0, "test error");
        let out = DiagnosticExporter::export(&[d], DiagnosticExportFormat::Plain);
        assert!(out.contains("[ERROR]"));
        assert!(out.contains("test error"));
    }
    #[test]
    fn test_diagnostic_exporter_csv() {
        let d = make_diag(2, "test");
        let csv = DiagnosticExporter::export(&[d], DiagnosticExportFormat::Csv);
        assert!(csv.contains("uri,line"));
        assert!(csv.contains("error"));
    }
    #[test]
    fn test_diagnostic_exporter_sarif() {
        let d = make_diag(0, "test");
        let sarif = DiagnosticExporter::export(&[d], DiagnosticExportFormat::Sarif);
        assert!(sarif.contains("2.1.0"));
        assert!(sarif.contains("oxilean"));
    }
    #[test]
    fn test_diagnostic_exporter_json() {
        let d = make_diag(0, "test");
        let json = DiagnosticExporter::export(&[d], DiagnosticExportFormat::Json);
        assert!(json.starts_with('['));
        assert!(json.contains("message"));
    }
    #[test]
    fn test_diagnostic_heatmap() {
        let diags = vec![make_diag(0, "e1"), make_diag(0, "e2"), make_diag(5, "w1")];
        let heatmap = DiagnosticHeatmap::build("file:///a.lean", &diags, 10);
        assert_eq!(heatmap.hottest_line(), Some(0));
        let bars = heatmap.render_bars(10);
        assert!(bars.contains("##"));
    }
    #[test]
    fn test_summary_report() {
        let report = DiagnosticSummaryReport {
            total_errors: 3,
            total_warnings: 5,
            total_info: 1,
            total_hints: 0,
            files_with_errors: 2,
            files_checked: 10,
            top_error_codes: vec![],
        };
        assert_eq!(report.total(), 9);
        assert!(report.has_errors());
        let rendered = report.render();
        assert!(rendered.contains("Errors:"));
    }
    #[test]
    fn test_diagnostics_adv_version() {
        assert!(!diagnostics_adv_version().is_empty());
    }
}
/// Deduplicates diagnostics by URI+range+message key.
#[allow(dead_code)]
pub fn dedup_diagnostics(diagnostics: Vec<AdvDiagnostic>) -> Vec<AdvDiagnostic> {
    let mut seen = std::collections::HashSet::new();
    diagnostics
        .into_iter()
        .filter(|d| {
            let key = format!(
                "{}:{}:{}:{}",
                d.uri, d.range.start.line, d.range.start.character, d.message
            );
            seen.insert(key)
        })
        .collect()
}
/// Assigns priority scores to diagnostics for display ordering.
#[allow(dead_code)]
pub fn prioritize_diagnostics(diagnostics: &mut Vec<AdvDiagnostic>) {
    diagnostics.sort_by(|a, b| {
        let sev_rank = |s: &DiagnosticSeverity| match s {
            DiagnosticSeverity::Error => 0,
            DiagnosticSeverity::Warning => 1,
            DiagnosticSeverity::Information => 2,
            DiagnosticSeverity::Hint => 3,
        };
        sev_rank(&a.severity)
            .cmp(&sev_rank(&b.severity))
            .then(a.range.start.line.cmp(&b.range.start.line))
    });
}
/// Group diagnostics by URI.
#[allow(dead_code)]
pub fn group_diagnostics_by_file(diagnostics: Vec<AdvDiagnostic>) -> Vec<DiagnosticGroup> {
    let mut map: std::collections::HashMap<String, Vec<AdvDiagnostic>> =
        std::collections::HashMap::new();
    for d in diagnostics {
        map.entry(d.uri.clone()).or_default().push(d);
    }
    let mut groups: Vec<DiagnosticGroup> = map
        .into_iter()
        .map(|(uri, ds)| DiagnosticGroup {
            uri,
            diagnostics: ds,
        })
        .collect();
    groups.sort_by(|a, b| a.uri.cmp(&b.uri));
    groups
}
#[cfg(test)]
mod adv_dedup_tests {
    use super::*;
    fn make_d(line: u32, msg: &str) -> AdvDiagnostic {
        AdvDiagnostic {
            code: AdvDiagnosticCode::TypeError,
            severity: DiagnosticSeverity::Error,
            range: Range::single_line(line, 0, 5),
            message: msg.to_string(),
            related: vec![],
            fixes: vec![],
            tags: vec![],
            uri: "file:///a.lean".to_string(),
        }
    }
    #[test]
    fn test_dedup_diagnostics() {
        let diags = vec![
            make_d(0, "error A"),
            make_d(0, "error A"),
            make_d(1, "error B"),
        ];
        let deduped = dedup_diagnostics(diags);
        assert_eq!(deduped.len(), 2);
    }
    #[test]
    fn test_prioritize_diagnostics() {
        let mut diags = vec![make_d(3, "late error"), make_d(0, "early error")];
        prioritize_diagnostics(&mut diags);
        assert_eq!(diags[0].range.start.line, 0);
    }
    #[test]
    fn test_group_diagnostics_by_file() {
        let mut d2 = make_d(1, "b");
        d2.uri = "file:///b.lean".to_string();
        let diags = vec![make_d(0, "a"), d2];
        let groups = group_diagnostics_by_file(diags);
        assert_eq!(groups.len(), 2);
    }
    #[test]
    fn test_diagnostic_state_machine() {
        let d = make_d(0, "test");
        let mut sm = DiagnosticStateMachine::new();
        sm.update(&[d.clone()]);
        sm.update(&[d.clone()]);
        let state = sm.state_of(&d);
        assert_eq!(state, DiagnosticState::Persistent);
        sm.update(&[]);
        let state2 = sm.state_of(&d);
        assert_eq!(state2, DiagnosticState::Resolved);
    }
    #[test]
    fn test_diagnostic_group_counts() {
        let d1 = make_d(0, "e1");
        let mut d2 = make_d(1, "w1");
        d2.severity = super::DiagnosticSeverity::Warning;
        let group = DiagnosticGroup {
            uri: "file:///a.lean".to_string(),
            diagnostics: vec![d1, d2],
        };
        assert_eq!(group.error_count(), 1);
        assert_eq!(group.warning_count(), 1);
    }
}
/// Return the version string for this module.
#[allow(dead_code)]
pub fn diagnostics_adv_ext_version() -> &'static str {
    "0.1.1"
}
#[cfg(test)]
mod rate_limiter_tests {
    use super::*;
    #[test]
    fn test_rate_limiter_allows_first() {
        let mut limiter = DiagnosticRateLimiter::new(1000);
        assert!(limiter.should_publish("file:///a.lean"));
    }
    #[test]
    fn test_rate_limiter_blocks_second() {
        let mut limiter = DiagnosticRateLimiter::new(10000);
        limiter.should_publish("file:///a.lean");
        assert!(!limiter.should_publish("file:///a.lean"));
    }
    #[test]
    fn test_rate_limiter_reset() {
        let mut limiter = DiagnosticRateLimiter::new(10000);
        limiter.should_publish("file:///a.lean");
        limiter.reset("file:///a.lean");
        assert!(limiter.should_publish("file:///a.lean"));
    }
}
#[cfg(test)]
mod query_tests {
    use super::*;
    fn make_err(msg: &str) -> AdvDiagnostic {
        AdvDiagnostic {
            code: AdvDiagnosticCode::TypeError,
            severity: DiagnosticSeverity::Error,
            range: Range::single_line(0, 0, 5),
            message: msg.to_string(),
            related: vec![],
            fixes: vec![],
            tags: vec![],
            uri: "file:///a.lean".to_string(),
        }
    }
    #[test]
    fn test_query_all() {
        let d = make_err("test");
        assert!(DiagnosticQuery::All.matches(&d));
    }
    #[test]
    fn test_query_errors() {
        let d = make_err("test");
        assert!(DiagnosticQuery::Errors.matches(&d));
        assert!(!DiagnosticQuery::Warnings.matches(&d));
    }
    #[test]
    fn test_query_keyword() {
        let d = make_err("type mismatch found");
        assert!(DiagnosticQuery::HasKeyword("mismatch".to_string()).matches(&d));
        assert!(!DiagnosticQuery::HasKeyword("unify".to_string()).matches(&d));
    }
    #[test]
    fn test_query_and_or() {
        let d = make_err("type error");
        let q = DiagnosticQuery::And(
            Box::new(DiagnosticQuery::Errors),
            Box::new(DiagnosticQuery::HasKeyword("type".to_string())),
        );
        assert!(q.matches(&d));
        let q2 = DiagnosticQuery::Or(
            Box::new(DiagnosticQuery::Warnings),
            Box::new(DiagnosticQuery::HasKeyword("type".to_string())),
        );
        assert!(q2.matches(&d));
    }
    #[test]
    fn test_query_not() {
        let d = make_err("test");
        let q = DiagnosticQuery::Not(Box::new(DiagnosticQuery::Warnings));
        assert!(q.matches(&d));
    }
}
