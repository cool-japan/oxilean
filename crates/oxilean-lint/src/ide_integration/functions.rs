//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{
    CompletionItem, CompletionKind, DiagnosticDeduplifier, DiagnosticFilter, DiagnosticSorter,
    DocumentVersion, ExpectationChecker, FileChangeCache, FileHasher, HoverProvider, IdeLintServer,
    IncrementalLintServer, InlayHint, InlayHintKind, InlayHintProvider, LineIndexer,
    LintAnnotation, LintAnnotationKind, LintAnnotationParser, LintBudget, LintCompletionProvider,
    LintDiagnostic, LintHoverDocFormatter, LintRuleDocumentation, LintRuleDocumentationRegistry,
    LintRulesIndex, LintSession, LintSeverity, LintSummaryFormatter, LspCodeAction,
    LspCodeActionProvider, RealTimeLinter, SeverityCounter, SortKey, WorkspaceScanner,
};

/// Maximum line length before a warning is emitted.
pub(super) const MAX_LINE_LEN: usize = 100;
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn lint_severity_lsp_codes() {
        assert_eq!(LintSeverity::Error.to_lsp_code(), 1);
        assert_eq!(LintSeverity::Warning.to_lsp_code(), 2);
        assert_eq!(LintSeverity::Info.to_lsp_code(), 3);
        assert_eq!(LintSeverity::Hint.to_lsp_code(), 4);
    }
    #[test]
    fn lint_diagnostic_to_json() {
        let d = LintDiagnostic::new("E001", "bad thing", LintSeverity::Error, 0, 10);
        let json = d.to_json();
        assert!(json.contains("\"code\":\"E001\""));
        assert!(json.contains("\"message\":\"bad thing\""));
        assert!(json.contains("\"severity\":1"));
        assert!(json.contains("\"start\":0"));
        assert!(json.contains("\"end\":10"));
    }
    #[test]
    fn lint_diagnostic_json_escapes_quotes() {
        let d = LintDiagnostic::new("E002", "say \"hello\"", LintSeverity::Warning, 0, 5);
        let json = d.to_json();
        assert!(json.contains("\\\"hello\\\""));
    }
    #[test]
    fn ide_lint_server_update_and_get() {
        let mut server = IdeLintServer::new();
        let diags = vec![LintDiagnostic::new(
            "W001",
            "warn",
            LintSeverity::Warning,
            0,
            5,
        )];
        server.update_file("foo.lean", "source", diags);
        let got = server.get_diagnostics("foo.lean");
        assert_eq!(got.len(), 1);
        assert_eq!(got[0].code, "W001");
    }
    #[test]
    fn ide_lint_server_clear_file() {
        let mut server = IdeLintServer::new();
        server.update_file(
            "a.lean",
            "",
            vec![LintDiagnostic::new("X", "x", LintSeverity::Error, 0, 1)],
        );
        server.clear_file("a.lean");
        assert!(server.get_diagnostics("a.lean").is_empty());
    }
    #[test]
    fn ide_lint_server_counts() {
        let mut server = IdeLintServer::new();
        server.update_file(
            "a.lean",
            "",
            vec![
                LintDiagnostic::new("E1", "e", LintSeverity::Error, 0, 1),
                LintDiagnostic::new("W1", "w", LintSeverity::Warning, 1, 2),
                LintDiagnostic::new("W2", "w", LintSeverity::Warning, 2, 3),
            ],
        );
        assert_eq!(server.error_count(), 1);
        assert_eq!(server.warning_count(), 2);
    }
    #[test]
    fn run_lint_pass_detects_long_line() {
        let long_line = "x".repeat(101);
        let diags = RealTimeLinter::run_lint_pass(&long_line);
        assert!(diags.iter().any(|d| d.code == "long_line"));
    }
    #[test]
    fn run_lint_pass_detects_todo() {
        let source = "-- TODO: fix this\n";
        let diags = RealTimeLinter::run_lint_pass(source);
        assert!(diags.iter().any(|d| d.code == "todo_comment"));
    }
    #[test]
    fn run_lint_pass_detects_trailing_whitespace() {
        let source = "let x = 1   \n";
        let diags = RealTimeLinter::run_lint_pass(source);
        assert!(diags.iter().any(|d| d.code == "trailing_whitespace"));
    }
    #[test]
    fn real_time_linter_lint_source_stores_results() {
        let mut linter = RealTimeLinter::new(200);
        let source = "-- TODO: something\n";
        let diags = linter.lint_source("test.lean", source);
        assert!(!diags.is_empty());
        assert!(!linter.server.get_diagnostics("test.lean").is_empty());
    }
}
#[cfg(test)]
mod ide_extended_tests {
    use super::*;
    #[test]
    fn document_version_bump_and_get() {
        let mut dv = DocumentVersion::new();
        assert_eq!(dv.get("a.lean"), 0);
        let v1 = dv.bump("a.lean");
        assert_eq!(v1, 1);
        let v2 = dv.bump("a.lean");
        assert_eq!(v2, 2);
        assert_eq!(dv.get("a.lean"), 2);
    }
    #[test]
    fn document_version_reset() {
        let mut dv = DocumentVersion::new();
        dv.bump("f.lean");
        dv.reset("f.lean");
        assert_eq!(dv.get("f.lean"), 0);
    }
    #[test]
    fn document_version_tracked_paths_sorted() {
        let mut dv = DocumentVersion::new();
        dv.bump("z.lean");
        dv.bump("a.lean");
        dv.bump("m.lean");
        let paths = dv.tracked_paths();
        assert_eq!(paths, ["a.lean", "m.lean", "z.lean"]);
    }
    #[test]
    fn diagnostic_filter_min_severity() {
        let filter = DiagnosticFilter::new().with_min_severity(2);
        let error_diag = LintDiagnostic::new("E1", "err", LintSeverity::Error, 0, 1);
        let warn_diag = LintDiagnostic::new("W1", "warn", LintSeverity::Warning, 0, 1);
        let info_diag = LintDiagnostic::new("I1", "info", LintSeverity::Info, 0, 1);
        assert!(filter.accepts(&error_diag));
        assert!(filter.accepts(&warn_diag));
        assert!(!filter.accepts(&info_diag));
    }
    #[test]
    fn diagnostic_filter_code_prefix() {
        let filter = DiagnosticFilter::new().with_code_prefix("unused_");
        let pass = LintDiagnostic::new("unused_variable", "x", LintSeverity::Warning, 0, 1);
        let fail = LintDiagnostic::new("dead_code", "y", LintSeverity::Warning, 0, 1);
        assert!(filter.accepts(&pass));
        assert!(!filter.accepts(&fail));
    }
    #[test]
    fn diagnostic_filter_apply() {
        let filter = DiagnosticFilter::new().with_min_severity(1);
        let diags = vec![
            LintDiagnostic::new("E", "e", LintSeverity::Error, 0, 1),
            LintDiagnostic::new("W", "w", LintSeverity::Warning, 1, 2),
        ];
        let accepted = filter.apply(&diags);
        assert_eq!(accepted.len(), 1);
        assert_eq!(accepted[0].code, "E");
    }
    #[test]
    fn lint_session_basic() {
        let mut session = LintSession::new();
        let source = "-- TODO: fix\n";
        let diags = session.lint("test.lean", source);
        assert!(diags.iter().any(|d| d.code == "todo_comment"));
    }
    #[test]
    fn lint_session_close_file() {
        let mut session = LintSession::new();
        session.lint("a.lean", "-- TODO: x\n");
        assert!(!session.session_diagnostics_for("a.lean").is_empty());
        session.close_file("a.lean");
        assert!(session.session_diagnostics_for("a.lean").is_empty());
    }
    #[test]
    fn line_indexer_single_line() {
        let idx = LineIndexer::new("hello");
        assert_eq!(idx.line(0), 0);
        assert_eq!(idx.column(3), 3);
        assert_eq!(idx.num_lines(), 1);
    }
    #[test]
    fn line_indexer_multi_line() {
        let src = "abc\ndef\nghi";
        let idx = LineIndexer::new(src);
        assert_eq!(idx.line(0), 0);
        assert_eq!(idx.line(4), 1);
        assert_eq!(idx.line(8), 2);
        assert_eq!(idx.column(4), 0);
        assert_eq!(idx.column(5), 1);
    }
    #[test]
    fn line_indexer_line_col() {
        let src = "abc\ndef";
        let idx = LineIndexer::new(src);
        assert_eq!(idx.line_col(5), (1, 1));
    }
    #[test]
    fn dedup_removes_duplicates() {
        let diags = vec![
            LintDiagnostic::new("A", "m", LintSeverity::Warning, 0, 5),
            LintDiagnostic::new("A", "m", LintSeverity::Warning, 0, 5),
            LintDiagnostic::new("B", "n", LintSeverity::Info, 5, 10),
        ];
        let count = DiagnosticDeduplifier::duplicate_count(&diags);
        assert_eq!(count, 1);
        let unique = DiagnosticDeduplifier::dedup(diags);
        assert_eq!(unique.len(), 2);
    }
    #[test]
    fn lint_budget_consume() {
        let mut budget = LintBudget::new(3);
        assert!(budget.consume());
        assert!(budget.consume());
        assert!(budget.consume());
        assert!(!budget.consume());
        assert!(budget.exhausted());
    }
    #[test]
    fn lint_budget_apply() {
        let budget = LintBudget::new(2);
        let diags = vec![
            LintDiagnostic::new("A", "x", LintSeverity::Error, 0, 1),
            LintDiagnostic::new("B", "y", LintSeverity::Error, 1, 2),
            LintDiagnostic::new("C", "z", LintSeverity::Error, 2, 3),
        ];
        let result = budget.apply(diags);
        assert_eq!(result.len(), 2);
    }
    #[test]
    fn severity_counter_record_all() {
        let diags = vec![
            LintDiagnostic::new("E", "e", LintSeverity::Error, 0, 1),
            LintDiagnostic::new("W", "w", LintSeverity::Warning, 1, 2),
            LintDiagnostic::new("I", "i", LintSeverity::Info, 2, 3),
            LintDiagnostic::new("H", "h", LintSeverity::Hint, 3, 4),
        ];
        let mut counter = SeverityCounter::new();
        counter.record_all(&diags);
        assert_eq!(counter.errors, 1);
        assert_eq!(counter.warnings, 1);
        assert_eq!(counter.infos, 1);
        assert_eq!(counter.hints, 1);
        assert_eq!(counter.total(), 4);
        assert!(counter.has_errors());
        assert!(!counter.is_clean());
    }
    #[test]
    fn severity_counter_merge() {
        let mut a = SeverityCounter::new();
        let mut b = SeverityCounter::new();
        a.errors = 2;
        b.warnings = 3;
        a.merge(&b);
        assert_eq!(a.errors, 2);
        assert_eq!(a.warnings, 3);
    }
    #[test]
    fn file_hasher_stable() {
        let h1 = FileHasher::hash("hello world");
        let h2 = FileHasher::hash("hello world");
        assert_eq!(h1, h2);
    }
    #[test]
    fn file_hasher_differs() {
        let h1 = FileHasher::hash("abc");
        let h2 = FileHasher::hash("abd");
        assert_ne!(h1, h2);
    }
    #[test]
    fn file_change_cache_unchanged() {
        let mut cache = FileChangeCache::new();
        assert!(!cache.is_unchanged("a.lean", "hello"));
        assert!(cache.is_unchanged("a.lean", "hello"));
        assert!(!cache.is_unchanged("a.lean", "world"));
    }
    #[test]
    fn file_change_cache_invalidate() {
        let mut cache = FileChangeCache::new();
        cache.is_unchanged("a.lean", "content");
        cache.invalidate("a.lean");
        assert!(!cache.is_unchanged("a.lean", "content"));
    }
    #[test]
    fn incremental_lint_server_skips_unchanged() {
        let mut server = IncrementalLintServer::new();
        let src = "-- TODO: something\n";
        let d1 = server.lint_if_changed("f.lean", src);
        let count1 = d1.len();
        let d2 = server.lint_if_changed("f.lean", src);
        let count2 = d2.len();
        assert_eq!(count1, count2);
    }
    #[test]
    fn annotation_parser_suppress() {
        let src = "-- @[nolint unused_variable]\nlet x := 1\n";
        let anns = LintAnnotationParser::parse(src);
        assert_eq!(anns.len(), 1);
        assert!(anns[0].suppresses("unused_variable"));
    }
    #[test]
    fn annotation_parser_expect() {
        let src = "-- @[expect_lint dead_code]\n";
        let anns = LintAnnotationParser::parse(src);
        assert_eq!(anns.len(), 1);
        assert!(anns[0].expects("dead_code"));
    }
    #[test]
    fn annotation_parser_note() {
        let src = "-- @[lint_note this is a note]\n";
        let anns = LintAnnotationParser::parse(src);
        assert_eq!(anns.len(), 1);
        assert!(matches!(&anns[0].kind, LintAnnotationKind::Note(_)));
    }
    #[test]
    fn expectation_checker_satisfied() {
        let anns = vec![LintAnnotation::new(
            LintAnnotationKind::Expect("todo_comment".to_string()),
            0,
            0,
        )];
        let diags = vec![LintDiagnostic::new(
            "todo_comment",
            "x",
            LintSeverity::Info,
            0,
            1,
        )];
        let outcome = ExpectationChecker::check(&anns, &diags);
        assert_eq!(outcome.satisfied.len(), 1);
        assert!(outcome.unsatisfied.is_empty());
    }
    #[test]
    fn expectation_checker_unsatisfied() {
        let anns = vec![LintAnnotation::new(
            LintAnnotationKind::Expect("dead_code".to_string()),
            0,
            0,
        )];
        let diags: Vec<LintDiagnostic> = Vec::new();
        let outcome = ExpectationChecker::check(&anns, &diags);
        assert_eq!(outcome.unsatisfied.len(), 1);
    }
    #[test]
    fn diagnostic_sorter_by_offset() {
        let mut diags = vec![
            LintDiagnostic::new("B", "b", LintSeverity::Warning, 10, 15),
            LintDiagnostic::new("A", "a", LintSeverity::Warning, 0, 5),
        ];
        DiagnosticSorter::sort(&mut diags, SortKey::Offset);
        assert_eq!(diags[0].code, "A");
        assert_eq!(diags[1].code, "B");
    }
    #[test]
    fn diagnostic_sorter_by_severity() {
        let mut diags = vec![
            LintDiagnostic::new("W", "w", LintSeverity::Warning, 5, 10),
            LintDiagnostic::new("E", "e", LintSeverity::Error, 0, 5),
        ];
        DiagnosticSorter::sort(&mut diags, SortKey::Severity);
        assert_eq!(diags[0].code, "E");
    }
    #[test]
    fn diagnostic_sorter_by_code() {
        let diags = vec![
            LintDiagnostic::new("zzz", "z", LintSeverity::Hint, 0, 1),
            LintDiagnostic::new("aaa", "a", LintSeverity::Hint, 1, 2),
        ];
        let sorted = DiagnosticSorter::sorted(diags, SortKey::Code);
        assert_eq!(sorted[0].code, "aaa");
    }
    #[test]
    fn format_brief_no_issues() {
        let counter = SeverityCounter::new();
        assert_eq!(
            LintSummaryFormatter::format_brief(&counter),
            "No issues found."
        );
    }
    #[test]
    fn format_brief_mixed() {
        let mut counter = SeverityCounter::new();
        counter.errors = 1;
        counter.warnings = 2;
        let s = LintSummaryFormatter::format_brief(&counter);
        assert!(s.contains("1 error(s)"));
        assert!(s.contains("2 warning(s)"));
    }
    #[test]
    fn format_json_array() {
        let diags = vec![LintDiagnostic::new("E1", "bad", LintSeverity::Error, 0, 3)];
        let json = LintSummaryFormatter::format_json(&diags);
        assert!(json.starts_with('['));
        assert!(json.ends_with(']'));
        assert!(json.contains("E1"));
    }
    #[test]
    fn format_github_actions() {
        let diags = vec![LintDiagnostic::new(
            "todo_comment",
            "TODO here",
            LintSeverity::Warning,
            5,
            10,
        )];
        let out = LintSummaryFormatter::format_github_actions("foo.lean", &diags);
        assert!(out.contains("::warning file=foo.lean"));
    }
    #[test]
    fn code_action_trailing_whitespace() {
        let diags = vec![LintDiagnostic::new(
            "trailing_whitespace",
            "Trailing ws",
            LintSeverity::Hint,
            10,
            11,
        )];
        let actions = LspCodeActionProvider::code_actions_for(&diags);
        assert!(!actions.is_empty());
        assert!(actions[0].is_preferred);
    }
    #[test]
    fn code_action_to_json() {
        let action = LspCodeAction::new("Fix it", "E001", "replacement", 0, 5).preferred();
        let json = action.to_json();
        assert!(json.contains("\"title\":\"Fix it\""));
        assert!(json.contains("\"isPreferred\":true"));
    }
    #[test]
    fn workspace_scanner_scan_all() {
        let mut scanner = WorkspaceScanner::new();
        let files = [
            ("a.lean", "-- TODO: something\n"),
            ("b.lean", "let x = 1\n"),
        ];
        let result = scanner.scan_all(&files);
        assert_eq!(result.files_scanned, 2);
        assert!(result.total_diagnostics > 0);
    }
    #[test]
    fn workspace_scanner_no_errors_on_clean() {
        let mut scanner = WorkspaceScanner::new();
        let files = [("clean.lean", "theorem foo : True := trivial\n")];
        let result = scanner.scan_all(&files);
        assert_eq!(result.total_errors, 0);
    }
}
#[cfg(test)]
mod ide_extra_tests {
    use super::*;
    #[test]
    fn lint_rule_doc_markdown() {
        let doc = LintRuleDocumentation::new("long_line", "Line too long")
            .with_explanation("Keep lines under 100 chars.")
            .with_examples("-- xxxxxxxxxx (too long)", "-- ok")
            .with_docs_url("https://example.com");
        let md = doc.to_markdown();
        assert!(md.contains("long_line"));
        assert!(md.contains("Line too long"));
        assert!(md.contains("Bad:"));
        assert!(md.contains("Good:"));
        assert!(md.contains("[Documentation]"));
    }
    #[test]
    fn lint_rule_doc_no_examples() {
        let doc = LintRuleDocumentation::new("x", "X rule");
        let md = doc.to_markdown();
        assert!(md.contains("x"));
        assert!(!md.contains("Bad:"));
    }
    #[test]
    fn registry_with_builtins() {
        let reg = LintRuleDocumentationRegistry::with_builtins();
        assert!(!reg.is_empty());
        assert!(reg.get("long_line").is_some());
        assert!(reg.get("todo_comment").is_some());
        assert!(reg.get("trailing_whitespace").is_some());
    }
    #[test]
    fn registry_all_codes_sorted() {
        let reg = LintRuleDocumentationRegistry::with_builtins();
        let codes = reg.all_codes();
        let mut sorted_copy = codes.clone();
        sorted_copy.sort();
        assert_eq!(codes, sorted_copy);
    }
    #[test]
    fn hover_provider_known_rule() {
        let provider = HoverProvider::new();
        let diags = vec![LintDiagnostic::new(
            "long_line",
            "Line too long",
            LintSeverity::Warning,
            0,
            50,
        )];
        let hover = provider.hover_at(&diags, 10);
        assert!(hover.is_some());
        let text = hover.expect("hover info should be present");
        assert!(text.contains("long_line"));
    }
    #[test]
    fn hover_provider_unknown_rule_fallback() {
        let provider = HoverProvider::new();
        let diags = vec![LintDiagnostic::new(
            "mystery_lint",
            "Something weird",
            LintSeverity::Info,
            5,
            15,
        )];
        let hover = provider.hover_at(&diags, 7);
        assert!(hover.is_some());
        assert!(hover
            .expect("hover info should be present")
            .contains("mystery_lint"));
    }
    #[test]
    fn hover_provider_no_diag_at_cursor() {
        let provider = HoverProvider::new();
        let diags = vec![LintDiagnostic::new(
            "A",
            "msg",
            LintSeverity::Hint,
            100,
            110,
        )];
        assert!(provider.hover_at(&diags, 0).is_none());
    }
    #[test]
    fn completion_item_json() {
        let item = CompletionItem::new("@[nolint x]", CompletionKind::Suppression, "@[nolint x]")
            .with_detail("Suppress x");
        let json = item.to_json();
        assert!(json.contains("@[nolint x]"));
        assert!(json.contains("Suppress x"));
    }
    #[test]
    fn completion_provider_returns_items() {
        let provider = LintCompletionProvider::new();
        let items = provider.completions_at(0);
        assert!(!items.is_empty());
        assert!(items.iter().all(|i| i.kind == CompletionKind::Suppression));
    }
    #[test]
    fn lint_rules_index_search() {
        let mut idx = LintRulesIndex::new();
        idx.add("todo_comment", "TODO comment found");
        idx.add("trailing_whitespace", "Trailing whitespace");
        idx.add("long_line", "Line too long");
        let results = idx.search("todo");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].0, "todo_comment");
        let results2 = idx.search("trailing");
        assert_eq!(results2.len(), 1);
        let results3 = idx.search("line");
        assert!(!results3.is_empty());
    }
    #[test]
    fn lint_rules_index_empty_search() {
        let idx = LintRulesIndex::new();
        assert!(idx.search("anything").is_empty());
    }
    #[test]
    fn lint_rules_index_case_insensitive() {
        let mut idx = LintRulesIndex::new();
        idx.add("TODO_COMMENT", "TODO marker");
        let results = idx.search("todo");
        assert_eq!(results.len(), 1);
    }
    #[test]
    fn format_brief_errors_only() {
        let mut counter = SeverityCounter::new();
        counter.errors = 3;
        let s = LintSummaryFormatter::format_brief(&counter);
        assert!(s.contains("3 error(s)"));
        assert!(!s.contains("warning"));
    }
    #[test]
    fn format_brief_hints_only() {
        let mut counter = SeverityCounter::new();
        counter.hints = 5;
        let s = LintSummaryFormatter::format_brief(&counter);
        assert!(s.contains("5 hint(s)"));
    }
}
#[cfg(test)]
mod ide_extra_tests2 {
    use super::*;
    #[test]
    fn hover_doc_formatter_short_tooltip() {
        let tip = LintHoverDocFormatter::short_tooltip("unused_import", "Remove unused imports");
        assert!(tip.contains("unused_import"));
        assert!(tip.contains("Remove unused imports"));
    }
    #[test]
    fn hover_doc_formatter_full_doc() {
        let doc = LintHoverDocFormatter::full_doc(
            "dead_code",
            "Detects unreachable code.",
            "Unreachable code clutters the codebase.",
            Some(("def unused := 1", "-- removed")),
        );
        assert!(doc.contains("dead_code"));
        assert!(doc.contains("Rationale"));
        assert!(doc.contains("Example"));
        assert!(doc.contains("Bad:"));
    }
    #[test]
    fn inlay_hint_provider_autofix_hints() {
        let hints = InlayHintProvider::autofix_hints(&[10, 20, 30]);
        assert_eq!(hints.len(), 3);
        assert!(hints
            .iter()
            .all(|h| h.kind == InlayHintKind::AutoFixAvailable));
    }
    #[test]
    fn inlay_hint_provider_filter_by_kind() {
        let hints = vec![
            InlayHint {
                position: 1,
                label: "a".to_string(),
                kind: InlayHintKind::AutoFixAvailable,
            },
            InlayHint {
                position: 2,
                label: "b".to_string(),
                kind: InlayHintKind::LintWarning,
            },
        ];
        let filtered = InlayHintProvider::filter_by_kind(&hints, InlayHintKind::AutoFixAvailable);
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].position, 1);
    }
}
/// Return the total number of inlay hints in a slice.
#[allow(dead_code)]
pub fn count_inlay_hints(hints: &[InlayHint]) -> usize {
    hints.len()
}
/// Return the positions of all inlay hints in a slice.
#[allow(dead_code)]
pub fn inlay_hint_positions(hints: &[InlayHint]) -> Vec<usize> {
    hints.iter().map(|h| h.position).collect()
}
