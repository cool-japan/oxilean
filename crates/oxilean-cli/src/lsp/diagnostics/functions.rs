//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lsp::{
    analyze_document, Diagnostic, DiagnosticSeverity, Document, Position, Range, TextEdit,
};
use oxilean_kernel::{Environment, Name};

use super::types::{
    CodeActionKind, DiagnosticAggregator, DiagnosticBatch, DiagnosticBudget, DiagnosticCache,
    DiagnosticCode, DiagnosticCollector, DiagnosticDiffTracker, DiagnosticEnricher,
    DiagnosticFilter, DiagnosticGroup, DiagnosticOutputFormat, DiagnosticPipeline,
    DiagnosticPriority, DiagnosticRateLimiter, DiagnosticReport, DiagnosticSnapshot,
    DiagnosticSubscription, DiagnosticThreshold, DiagnosticWorkspaceAggregator, ExtendedDiagnostic,
    InlineAnnotation, QuickFix, RelatedInfo, RichDiagnostic,
};

/// Get the expected closing delimiter for an opener.
pub fn closing_for(ch: char) -> char {
    match ch {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        _ => ch,
    }
}
/// Compute quick fixes for a diagnostic.
pub fn compute_quick_fixes(
    diagnostic: &Diagnostic,
    doc: &Document,
    env: &Environment,
) -> Vec<QuickFix> {
    let mut fixes = Vec::new();
    let msg = &diagnostic.message;
    if msg.contains("shadows") {
        if let Some(fix) = suggest_rename_fix(diagnostic, doc) {
            fixes.push(fix);
        }
    }
    if msg.contains("type mismatch") || msg.contains("type error") {
        fixes.extend(suggest_type_fix(diagnostic, doc));
    }
    if msg.contains("unknown identifier") || msg.contains("unresolved") {
        fixes.extend(suggest_import_fix(diagnostic, env));
        fixes.extend(suggest_typo_fix(diagnostic, doc, env));
    }
    if msg.contains("unused variable") {
        if let Some(fix) = suggest_underscore_prefix(diagnostic, doc) {
            fixes.push(fix);
        }
    }
    fixes
}
/// Suggest renaming a shadowed variable by adding a prime suffix.
fn suggest_rename_fix(diagnostic: &Diagnostic, doc: &Document) -> Option<QuickFix> {
    let range = &diagnostic.range;
    let line = doc.get_line(range.start.line)?;
    let start = range.start.character as usize;
    let end = (range.end.character as usize).min(line.len());
    if start >= end || start >= line.len() {
        return None;
    }
    let name = &line[start..end];
    if name.is_empty() || !name.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return None;
    }
    let new_name = format!("{}'", name);
    Some(QuickFix {
        title: format!("Rename to '{}'", new_name),
        edits: vec![TextEdit::new(range.clone(), new_name)],
        diagnostic: diagnostic.clone(),
    })
}
/// Suggest type fixes when there is a type mismatch.
pub fn suggest_type_fix(diagnostic: &Diagnostic, _doc: &Document) -> Vec<QuickFix> {
    vec![QuickFix {
        title: "Replace with sorry".to_string(),
        edits: vec![TextEdit::new(diagnostic.range.clone(), "sorry")],
        diagnostic: diagnostic.clone(),
    }]
}
/// Suggest imports for unresolved names.
pub fn suggest_import_fix(diagnostic: &Diagnostic, _env: &Environment) -> Vec<QuickFix> {
    let mut fixes = Vec::new();
    let common_modules = ["Init", "Std", "Mathlib"];
    for m in &common_modules {
        fixes.push(QuickFix {
            title: format!("Add 'import {}'", m),
            edits: vec![TextEdit::new(
                Range::single_line(0, 0, 0),
                format!("import {}\n", m),
            )],
            diagnostic: diagnostic.clone(),
        });
    }
    fixes
}
/// Suggest fixes for typos using edit distance.
pub fn suggest_typo_fix(
    diagnostic: &Diagnostic,
    doc: &Document,
    env: &Environment,
) -> Vec<QuickFix> {
    let mut fixes = Vec::new();
    let range = &diagnostic.range;
    let name = match doc.get_line(range.start.line) {
        Some(line) => {
            let start = range.start.character as usize;
            let end = (range.end.character as usize).min(line.len());
            if start < end && start < line.len() {
                &line[start..end]
            } else {
                return fixes;
            }
        }
        None => return fixes,
    };
    for env_name in env.constant_names() {
        let env_str = env_name.to_string();
        let dist = edit_distance(name, &env_str);
        if dist > 0 && dist <= 2 {
            fixes.push(QuickFix {
                title: format!("Did you mean '{}'?", env_str),
                edits: vec![TextEdit::new(range.clone(), &env_str)],
                diagnostic: diagnostic.clone(),
            });
        }
    }
    fixes
}
/// Suggest prefixing an unused variable with underscore.
fn suggest_underscore_prefix(diagnostic: &Diagnostic, doc: &Document) -> Option<QuickFix> {
    let range = &diagnostic.range;
    let line = doc.get_line(range.start.line)?;
    let start = range.start.character as usize;
    let end = (range.end.character as usize).min(line.len());
    if start >= end || start >= line.len() {
        return None;
    }
    let name = &line[start..end];
    if name.starts_with('_') {
        return None;
    }
    let new_name = format!("_{}", name);
    Some(QuickFix {
        title: format!("Prefix with underscore: '{}'", new_name),
        edits: vec![TextEdit::new(range.clone(), new_name)],
        diagnostic: diagnostic.clone(),
    })
}
/// Compute the Levenshtein edit distance between two strings.
pub fn edit_distance(a: &str, b: &str) -> usize {
    let a_bytes = a.as_bytes();
    let b_bytes = b.as_bytes();
    let m = a_bytes.len();
    let n = b_bytes.len();
    if m == 0 {
        return n;
    }
    if n == 0 {
        return m;
    }
    let mut prev_row: Vec<usize> = (0..=n).collect();
    let mut curr_row: Vec<usize> = vec![0; n + 1];
    for i in 1..=m {
        curr_row[0] = i;
        for j in 1..=n {
            let cost = if a_bytes[i - 1] == b_bytes[j - 1] {
                0
            } else {
                1
            };
            curr_row[j] = (prev_row[j] + 1)
                .min(curr_row[j - 1] + 1)
                .min(prev_row[j - 1] + cost);
        }
        std::mem::swap(&mut prev_row, &mut curr_row);
    }
    prev_row[n]
}
/// Format a diagnostic as a human-readable string.
pub fn format_diagnostic(diag: &RichDiagnostic) -> String {
    let prefix = severity_prefix(diag.diagnostic.severity);
    let code_str = diag.code.as_str();
    format!("{} [{}]: {}", prefix, code_str, diag.diagnostic.message)
}
/// Format a diagnostic with source context showing the error location.
pub fn format_diagnostic_range(diag: &RichDiagnostic, source: &str) -> String {
    let mut result = format_diagnostic(diag);
    let range = &diag.diagnostic.range;
    let lines: Vec<&str> = source.lines().collect();
    let line_idx = range.start.line as usize;
    if line_idx < lines.len() {
        let line = lines[line_idx];
        result.push_str(&format!(
            "\n  --> line {}:{}\n",
            line_idx + 1,
            range.start.character + 1
        ));
        result.push_str(&format!("   | {}\n", line));
        let start = range.start.character as usize;
        let end = (range.end.character as usize).min(line.len());
        let arrow_len = if end > start { end - start } else { 1 };
        result.push_str(&format!(
            "   | {}{}",
            " ".repeat(start),
            "^".repeat(arrow_len)
        ));
    }
    for info in &diag.related {
        result.push_str(&format!("\n  note: {}", info.message));
    }
    result
}
/// Return the severity prefix string.
pub fn severity_prefix(severity: DiagnosticSeverity) -> &'static str {
    match severity {
        DiagnosticSeverity::Error => "error",
        DiagnosticSeverity::Warning => "warning",
        DiagnosticSeverity::Information => "info",
        DiagnosticSeverity::Hint => "hint",
    }
}
/// Build a list of related information messages for a diagnostic.
pub fn related_information(
    diag: &RichDiagnostic,
    doc: &Document,
    env: &Environment,
) -> Vec<RelatedInfo> {
    let mut related = Vec::new();
    match diag.code {
        DiagnosticCode::Shadowing => {
            related.push(RelatedInfo {
                message: "original definition is in the environment".to_string(),
                uri: doc.uri.clone(),
                range: diag.diagnostic.range.clone(),
            });
        }
        DiagnosticCode::UnresolvedName => {
            let range = &diag.diagnostic.range;
            if let Some(line) = doc.get_line(range.start.line) {
                let start = range.start.character as usize;
                let end = (range.end.character as usize).min(line.len());
                if start < end && start < line.len() {
                    let name = &line[start..end];
                    for env_name in env.constant_names() {
                        let env_str = env_name.to_string();
                        if edit_distance(name, &env_str) <= 2 {
                            related.push(RelatedInfo {
                                message: format!("did you mean '{}'?", env_str),
                                uri: doc.uri.clone(),
                                range: range.clone(),
                            });
                        }
                    }
                }
            }
        }
        _ => {}
    }
    related
}
#[cfg(test)]
mod tests {
    use super::*;
    fn make_doc(content: &str) -> Document {
        Document::new("file:///test.lean", 1, content)
    }
    #[test]
    fn test_diagnostic_code_as_str() {
        assert_eq!(DiagnosticCode::LexError.as_str(), "E001");
        assert_eq!(DiagnosticCode::ParseError.as_str(), "E002");
        assert_eq!(DiagnosticCode::TypeError.as_str(), "E003");
        assert_eq!(DiagnosticCode::UnusedVariable.as_str(), "W001");
        assert_eq!(DiagnosticCode::Shadowing.as_str(), "W002");
    }
    #[test]
    fn test_diagnostic_code_description() {
        assert_eq!(DiagnosticCode::LexError.description(), "lexer error");
        assert_eq!(DiagnosticCode::ParseError.description(), "parse error");
    }
    #[test]
    fn test_edit_distance_equal() {
        assert_eq!(edit_distance("hello", "hello"), 0);
    }
    #[test]
    fn test_edit_distance_one_char() {
        assert_eq!(edit_distance("hello", "hallo"), 1);
    }
    #[test]
    fn test_edit_distance_insert() {
        assert_eq!(edit_distance("hell", "hello"), 1);
    }
    #[test]
    fn test_edit_distance_delete() {
        assert_eq!(edit_distance("hello", "hell"), 1);
    }
    #[test]
    fn test_edit_distance_empty() {
        assert_eq!(edit_distance("", "hello"), 5);
        assert_eq!(edit_distance("hello", ""), 5);
        assert_eq!(edit_distance("", ""), 0);
    }
    #[test]
    fn test_edit_distance_swap() {
        assert_eq!(edit_distance("ab", "ba"), 2);
    }
    #[test]
    fn test_severity_prefix() {
        assert_eq!(severity_prefix(DiagnosticSeverity::Error), "error");
        assert_eq!(severity_prefix(DiagnosticSeverity::Warning), "warning");
        assert_eq!(severity_prefix(DiagnosticSeverity::Information), "info");
        assert_eq!(severity_prefix(DiagnosticSeverity::Hint), "hint");
    }
    #[test]
    fn test_collector_empty_document() {
        let env = Environment::new();
        let collector = DiagnosticCollector::new(&env, 100);
        let doc = make_doc("");
        let diags = collector.collect_diagnostics(&doc);
        assert!(diags.is_empty());
    }
    #[test]
    fn test_collector_valid_tokens() {
        let env = Environment::new();
        let collector = DiagnosticCollector::new(&env, 100);
        let doc = make_doc("def foo : Nat := 1");
        let lex_errors = collector.collect_lex_errors(&doc.content);
        assert!(lex_errors.is_empty());
    }
    #[test]
    fn test_collector_unmatched_paren() {
        let env = Environment::new();
        let collector = DiagnosticCollector::new(&env, 100);
        let doc = make_doc("(def foo := 1");
        let parse_errors = collector.collect_parse_errors(&doc.content);
        assert!(!parse_errors.is_empty());
        assert!(parse_errors
            .iter()
            .any(|d| d.diagnostic.message.contains("unclosed")));
    }
    #[test]
    fn test_format_diagnostic_error() {
        let diag = RichDiagnostic {
            diagnostic: Diagnostic::error(Range::single_line(0, 0, 5), "test error"),
            code: DiagnosticCode::ParseError,
            related: Vec::new(),
        };
        let formatted = format_diagnostic(&diag);
        assert!(formatted.contains("error"));
        assert!(formatted.contains("E002"));
        assert!(formatted.contains("test error"));
    }
    #[test]
    fn test_format_diagnostic_range_with_source() {
        let diag = RichDiagnostic {
            diagnostic: Diagnostic::error(Range::single_line(0, 4, 7), "bad token"),
            code: DiagnosticCode::LexError,
            related: Vec::new(),
        };
        let source = "def @#$ := 1";
        let formatted = format_diagnostic_range(&diag, source);
        assert!(formatted.contains("^^^"));
        assert!(formatted.contains("line 1"));
    }
    #[test]
    fn test_code_action_kind_as_str() {
        assert_eq!(CodeActionKind::QuickFix.as_str(), "quickfix");
        assert_eq!(CodeActionKind::Refactor.as_str(), "refactor");
        assert_eq!(CodeActionKind::Source.as_str(), "source");
        assert_eq!(CodeActionKind::RefactorExtract.as_str(), "refactor.extract");
        assert_eq!(CodeActionKind::RefactorInline.as_str(), "refactor.inline");
    }
    #[test]
    fn test_closing_for() {
        assert_eq!(closing_for('('), ')');
        assert_eq!(closing_for('['), ']');
        assert_eq!(closing_for('{'), '}');
    }
    #[test]
    fn test_suggest_underscore_prefix() {
        let diag = Diagnostic::warning(Range::single_line(0, 4, 7), "unused variable 'foo'");
        let doc = make_doc("def foo := 1");
        let fix = suggest_underscore_prefix(&diag, &doc);
        assert!(fix.is_some());
        let fix = fix.expect("test operation should succeed");
        assert!(fix.title.contains("_foo"));
    }
    #[test]
    fn test_compute_quick_fixes_unused() {
        let diag = Diagnostic::warning(Range::single_line(0, 4, 7), "unused variable 'foo'");
        let doc = make_doc("def foo := 1");
        let env = Environment::new();
        let fixes = compute_quick_fixes(&diag, &doc, &env);
        assert!(!fixes.is_empty());
    }
    #[test]
    fn test_rich_diagnostic_related_info() {
        let diag = RichDiagnostic {
            diagnostic: Diagnostic::warning(Range::single_line(0, 0, 3), "shadows"),
            code: DiagnosticCode::Shadowing,
            related: vec![RelatedInfo {
                message: "original here".to_string(),
                uri: "file:///test.lean".to_string(),
                range: Range::single_line(0, 0, 3),
            }],
        };
        assert_eq!(diag.related.len(), 1);
        assert_eq!(diag.related[0].message, "original here");
    }
}
/// Map a `DiagnosticCode` to its default severity.
pub fn default_severity_for_code(code: DiagnosticCode) -> DiagnosticSeverity {
    match code {
        DiagnosticCode::LexError
        | DiagnosticCode::ParseError
        | DiagnosticCode::TypeError
        | DiagnosticCode::UnresolvedName
        | DiagnosticCode::MissingImport => DiagnosticSeverity::Error,
        DiagnosticCode::UnusedVariable
        | DiagnosticCode::Shadowing
        | DiagnosticCode::Deprecation
        | DiagnosticCode::RedundantImport
        | DiagnosticCode::StyleWarning => DiagnosticSeverity::Warning,
    }
}
#[cfg(test)]
mod filter_tests {
    use super::*;
    fn mk_rich(code: DiagnosticCode, severity: DiagnosticSeverity) -> RichDiagnostic {
        let diag = if severity == DiagnosticSeverity::Error {
            Diagnostic::error(Range::single_line(0, 0, 1), "err")
        } else {
            Diagnostic::warning(Range::single_line(0, 0, 1), "warn")
        };
        RichDiagnostic {
            diagnostic: diag,
            code,
            related: vec![],
        }
    }
    #[test]
    fn test_filter_accept_all() {
        let f = DiagnosticFilter::accept_all();
        let d = mk_rich(DiagnosticCode::TypeError, DiagnosticSeverity::Error);
        assert!(f.accepts(&d));
    }
    #[test]
    fn test_filter_errors_only_rejects_warning() {
        let f = DiagnosticFilter::errors_only();
        let d = mk_rich(DiagnosticCode::UnusedVariable, DiagnosticSeverity::Warning);
        assert!(!f.accepts(&d));
    }
    #[test]
    fn test_filter_errors_only_accepts_error() {
        let f = DiagnosticFilter::errors_only();
        let d = mk_rich(DiagnosticCode::TypeError, DiagnosticSeverity::Error);
        assert!(f.accepts(&d));
    }
    #[test]
    fn test_filter_suppress() {
        let f = DiagnosticFilter::accept_all().suppress(DiagnosticCode::UnusedVariable);
        let d = mk_rich(DiagnosticCode::UnusedVariable, DiagnosticSeverity::Warning);
        assert!(!f.accepts(&d));
    }
    #[test]
    fn test_filter_limit() {
        let f = DiagnosticFilter::accept_all().limit(2);
        let items = vec![
            mk_rich(DiagnosticCode::LexError, DiagnosticSeverity::Error),
            mk_rich(DiagnosticCode::ParseError, DiagnosticSeverity::Error),
            mk_rich(DiagnosticCode::TypeError, DiagnosticSeverity::Error),
        ];
        let result = f.apply(&items);
        assert_eq!(result.len(), 2);
    }
    #[test]
    fn test_diagnostic_batch_error_count() {
        let mut b = DiagnosticBatch::new();
        b.add(mk_rich(
            DiagnosticCode::TypeError,
            DiagnosticSeverity::Error,
        ));
        b.add(mk_rich(
            DiagnosticCode::UnusedVariable,
            DiagnosticSeverity::Warning,
        ));
        assert_eq!(b.error_count(), 1);
        assert_eq!(b.warning_count(), 1);
        assert!(b.has_errors());
    }
    #[test]
    fn test_diagnostic_batch_empty() {
        let b = DiagnosticBatch::new();
        assert!(b.is_empty());
        assert_eq!(b.len(), 0);
    }
    #[test]
    fn test_default_severity_for_code_lex_error() {
        assert_eq!(
            default_severity_for_code(DiagnosticCode::LexError),
            DiagnosticSeverity::Error
        );
    }
    #[test]
    fn test_default_severity_for_code_unused_variable() {
        assert_eq!(
            default_severity_for_code(DiagnosticCode::UnusedVariable),
            DiagnosticSeverity::Warning
        );
    }
    #[test]
    fn test_diagnostic_batch_filter() {
        let mut b = DiagnosticBatch::new();
        b.add(mk_rich(
            DiagnosticCode::TypeError,
            DiagnosticSeverity::Error,
        ));
        b.add(mk_rich(
            DiagnosticCode::UnusedVariable,
            DiagnosticSeverity::Warning,
        ));
        let f = DiagnosticFilter::errors_only();
        let result = b.filter(&f);
        assert_eq!(result.len(), 1);
    }
}
/// Sort diagnostics by line and then by severity (errors before warnings).
#[allow(dead_code)]
pub fn sort_diagnostics(diagnostics: &mut Vec<RichDiagnostic>) {
    diagnostics.sort_by(|a, b| {
        let line_cmp = a
            .diagnostic
            .range
            .start
            .line
            .cmp(&b.diagnostic.range.start.line);
        if line_cmp != std::cmp::Ordering::Equal {
            return line_cmp;
        }
        let col_cmp = a
            .diagnostic
            .range
            .start
            .character
            .cmp(&b.diagnostic.range.start.character);
        if col_cmp != std::cmp::Ordering::Equal {
            return col_cmp;
        }
        a.diagnostic.severity.cmp(&b.diagnostic.severity)
    });
}
/// Deduplicate diagnostics that have the same range and message.
#[allow(dead_code)]
pub fn deduplicate_diagnostics(diagnostics: &mut Vec<RichDiagnostic>) {
    let mut seen = std::collections::HashSet::new();
    diagnostics.retain(|d| {
        let key = (
            d.diagnostic.range.start.line,
            d.diagnostic.range.start.character,
            d.diagnostic.message.clone(),
        );
        seen.insert(key)
    });
}
/// Generate inline annotations from rich diagnostics.
#[allow(dead_code)]
pub fn inline_annotations(diagnostics: &[RichDiagnostic]) -> Vec<InlineAnnotation> {
    diagnostics
        .iter()
        .map(|d| InlineAnnotation {
            line: d.diagnostic.range.start.line,
            message: d.diagnostic.message.clone(),
            severity: d.diagnostic.severity,
        })
        .collect()
}
/// Print source code annotated with diagnostic markers.
#[allow(dead_code)]
pub fn print_annotated_source(source: &str, diagnostics: &[RichDiagnostic]) -> String {
    let lines: Vec<&str> = source.lines().collect();
    let mut output = String::new();
    for (line_idx, &line_text) in lines.iter().enumerate() {
        let line_num = line_idx as u32;
        output.push_str(&format!("{:4} | {}\n", line_num + 1, line_text));
        let line_diags: Vec<&RichDiagnostic> = diagnostics
            .iter()
            .filter(|d| d.diagnostic.range.start.line == line_num)
            .collect();
        for diag in &line_diags {
            let col = diag.diagnostic.range.start.character as usize;
            let end_col = diag.diagnostic.range.end.character as usize;
            let len = if end_col > col { end_col - col } else { 1 };
            let prefix = " ".repeat(7 + col);
            let arrows = "^".repeat(len);
            let prefix_str = match diag.diagnostic.severity {
                DiagnosticSeverity::Error => "error",
                DiagnosticSeverity::Warning => "warning",
                DiagnosticSeverity::Information => "info",
                DiagnosticSeverity::Hint => "hint",
            };
            output.push_str(&format!(
                "     | {}{} {}: {}\n",
                prefix, arrows, prefix_str, diag.diagnostic.message
            ));
        }
    }
    output
}
/// Serialize a list of rich diagnostics to a plain-text report.
#[allow(dead_code)]
pub fn serialize_diagnostics_plain(diagnostics: &[RichDiagnostic]) -> String {
    if diagnostics.is_empty() {
        return "No diagnostics.\n".to_string();
    }
    let mut output = String::new();
    for (i, d) in diagnostics.iter().enumerate() {
        output.push_str(&format!(
            "[{}] {} ({}): {} at line {}:{}\n",
            i + 1,
            severity_prefix(d.diagnostic.severity),
            d.code.as_str(),
            d.diagnostic.message,
            d.diagnostic.range.start.line + 1,
            d.diagnostic.range.start.character + 1,
        ));
        for related in &d.related {
            output.push_str(&format!("     note: {}\n", related.message));
        }
    }
    output
}
/// Serialize diagnostics as a compact JSON-like string (for logging).
#[allow(dead_code)]
pub fn serialize_diagnostics_json_lines(diagnostics: &[RichDiagnostic]) -> String {
    diagnostics
        .iter()
        .map(|d| {
            format!(
                r#"{{"severity":"{}","code":"{}","message":"{}","line":{}}}"#,
                severity_prefix(d.diagnostic.severity),
                d.code.as_str(),
                d.diagnostic.message.replace('"', "\\\""),
                d.diagnostic.range.start.line
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}
#[cfg(test)]
mod diagnostics_extended_tests {
    use super::*;
    fn make_rich(code: DiagnosticCode, severity: DiagnosticSeverity, msg: &str) -> RichDiagnostic {
        let diag = if severity == DiagnosticSeverity::Error {
            Diagnostic::error(Range::single_line(0, 0, 3), msg)
        } else {
            Diagnostic::warning(Range::single_line(0, 0, 3), msg)
        };
        RichDiagnostic {
            diagnostic: diag,
            code,
            related: vec![],
        }
    }
    #[test]
    fn test_extended_diagnostic_new() {
        let rich = make_rich(
            DiagnosticCode::TypeError,
            DiagnosticSeverity::Error,
            "bad type",
        );
        let ext = ExtendedDiagnostic::new(rich);
        assert_eq!(ext.priority, DiagnosticPriority::High);
        assert!(!ext.auto_fixable);
    }
    #[test]
    fn test_extended_diagnostic_with_fix() {
        let rich = make_rich(
            DiagnosticCode::UnusedVariable,
            DiagnosticSeverity::Warning,
            "unused x",
        );
        let ext = ExtendedDiagnostic::new(rich).with_fix_count(2);
        assert_eq!(ext.fix_count, 2);
        assert!(ext.auto_fixable);
    }
    #[test]
    fn test_extended_diagnostic_tag() {
        let rich = make_rich(
            DiagnosticCode::Deprecation,
            DiagnosticSeverity::Warning,
            "deprecated",
        );
        let ext = ExtendedDiagnostic::new(rich)
            .with_tag("deprecated")
            .with_tag("auto-fixable");
        assert!(ext.has_tag("deprecated"));
        assert!(!ext.has_tag("other"));
    }
    #[test]
    fn test_diagnostic_group_counts() {
        let mut g = DiagnosticGroup::new("MyDecl");
        g.add(make_rich(
            DiagnosticCode::TypeError,
            DiagnosticSeverity::Error,
            "err",
        ));
        g.add(make_rich(
            DiagnosticCode::UnusedVariable,
            DiagnosticSeverity::Warning,
            "warn",
        ));
        assert_eq!(g.error_count(), 1);
        assert_eq!(g.warning_count(), 1);
        assert!(g.has_errors());
    }
    #[test]
    fn test_diagnostic_report_summary() {
        let mut report = DiagnosticReport::new("file:///a.lean");
        let mut g = DiagnosticGroup::new("decl");
        g.add(make_rich(
            DiagnosticCode::ParseError,
            DiagnosticSeverity::Error,
            "parse error",
        ));
        report.add_group(g);
        let s = report.summary();
        assert!(s.contains("error"));
        assert!(!report.is_clean());
    }
    #[test]
    fn test_diagnostic_report_clean() {
        let report = DiagnosticReport::new("file:///b.lean");
        assert!(report.is_clean());
        assert!(report.summary().contains("no issues"));
    }
    #[test]
    fn test_sort_diagnostics() {
        let mut diags = vec![
            make_rich(
                DiagnosticCode::TypeError,
                DiagnosticSeverity::Error,
                "line2",
            ),
            make_rich(
                DiagnosticCode::UnusedVariable,
                DiagnosticSeverity::Warning,
                "line1",
            ),
        ];
        diags[0].diagnostic.range.start.line = 5;
        diags[1].diagnostic.range.start.line = 2;
        sort_diagnostics(&mut diags);
        assert_eq!(diags[0].diagnostic.range.start.line, 2);
    }
    #[test]
    fn test_deduplicate_diagnostics() {
        let mut diags = vec![
            make_rich(DiagnosticCode::TypeError, DiagnosticSeverity::Error, "dup"),
            make_rich(DiagnosticCode::TypeError, DiagnosticSeverity::Error, "dup"),
            make_rich(
                DiagnosticCode::TypeError,
                DiagnosticSeverity::Error,
                "unique",
            ),
        ];
        deduplicate_diagnostics(&mut diags);
        assert_eq!(diags.len(), 2);
    }
    #[test]
    fn test_inline_annotations() {
        let diags = vec![make_rich(
            DiagnosticCode::LexError,
            DiagnosticSeverity::Error,
            "lex err",
        )];
        let anns = inline_annotations(&diags);
        assert_eq!(anns.len(), 1);
        assert_eq!(anns[0].line, 0);
        assert_eq!(anns[0].message, "lex err");
    }
    #[test]
    fn test_diagnostic_rate_limiter() {
        let mut limiter = DiagnosticRateLimiter::new(3);
        assert!(limiter.allow("file:///a.lean"));
        assert!(limiter.allow("file:///a.lean"));
        assert!(limiter.allow("file:///a.lean"));
        assert!(!limiter.allow("file:///a.lean"));
        limiter.reset("file:///a.lean");
        assert!(limiter.allow("file:///a.lean"));
    }
    #[test]
    fn test_serialize_diagnostics_plain() {
        let diags = vec![make_rich(
            DiagnosticCode::ParseError,
            DiagnosticSeverity::Error,
            "test error",
        )];
        let output = serialize_diagnostics_plain(&diags);
        assert!(output.contains("error"));
        assert!(output.contains("test error"));
    }
    #[test]
    fn test_serialize_diagnostics_json_lines() {
        let diags = vec![make_rich(
            DiagnosticCode::TypeError,
            DiagnosticSeverity::Error,
            "type err",
        )];
        let output = serialize_diagnostics_json_lines(&diags);
        assert!(output.contains("severity"));
        assert!(output.contains("type err"));
    }
    #[test]
    fn test_diagnostic_aggregator() {
        let mut agg = DiagnosticAggregator::new();
        let diags = vec![
            make_rich(DiagnosticCode::TypeError, DiagnosticSeverity::Error, "e1"),
            make_rich(
                DiagnosticCode::UnusedVariable,
                DiagnosticSeverity::Warning,
                "w1",
            ),
        ];
        agg.record("file:///a.lean", &diags);
        assert_eq!(agg.total_errors(), 1);
        assert_eq!(agg.total_warnings(), 1);
        let worst = agg.worst_file();
        assert!(worst.is_some());
    }
    #[test]
    fn test_diagnostic_snapshot_capture_and_diff() {
        let diags1 = vec![make_rich(
            DiagnosticCode::LexError,
            DiagnosticSeverity::Error,
            "lex",
        )];
        let diags2 = vec![make_rich(
            DiagnosticCode::ParseError,
            DiagnosticSeverity::Error,
            "parse",
        )];
        let snap1 = DiagnosticSnapshot::capture("file:///a.lean", &diags1);
        let snap2 = DiagnosticSnapshot::capture("file:///a.lean", &diags2);
        let diffs = snap1.diff(&snap2);
        assert!(!diffs.is_empty());
    }
    #[test]
    fn test_diagnostic_snapshot_count_by_code() {
        let diags = vec![
            make_rich(DiagnosticCode::LexError, DiagnosticSeverity::Error, "e1"),
            make_rich(DiagnosticCode::LexError, DiagnosticSeverity::Error, "e2"),
            make_rich(DiagnosticCode::ParseError, DiagnosticSeverity::Error, "e3"),
        ];
        let snap = DiagnosticSnapshot::capture("file:///a.lean", &diags);
        assert_eq!(snap.count_by_code("E001"), 2);
        assert_eq!(snap.count_by_code("E002"), 1);
    }
    #[test]
    fn test_print_annotated_source_basic() {
        let source = "def foo := 1\ndef bar := 2";
        let diags = vec![make_rich(
            DiagnosticCode::TypeError,
            DiagnosticSeverity::Error,
            "type error",
        )];
        let output = print_annotated_source(source, &diags);
        assert!(output.contains("def foo"));
        assert!(output.contains("^"));
    }
    #[test]
    fn test_diagnostic_priority_ordering() {
        assert!(DiagnosticPriority::Critical > DiagnosticPriority::High);
        assert!(DiagnosticPriority::High > DiagnosticPriority::Normal);
        assert!(DiagnosticPriority::Normal > DiagnosticPriority::Low);
    }
}
/// Format a diagnostic for output in a given format.
#[allow(dead_code)]
pub fn format_diagnostic_output(diag: &Diagnostic, format: DiagnosticOutputFormat) -> String {
    let severity = match diag.severity {
        DiagnosticSeverity::Error => "error",
        DiagnosticSeverity::Warning => "warning",
        DiagnosticSeverity::Information => "info",
        DiagnosticSeverity::Hint => "hint",
    };
    match format {
        DiagnosticOutputFormat::Text => {
            format!("[{}] {:?}: {}", severity, diag.range, diag.message)
        }
        DiagnosticOutputFormat::Json => {
            format!(
                "{{\"severity\":\"{}\",\"message\":\"{}\"}}",
                severity,
                diag.message.replace('"', "\\\"")
            )
        }
        DiagnosticOutputFormat::Compact => format!("[{}] {}", severity, diag.message),
        DiagnosticOutputFormat::Annotated => {
            format!("[{}] {}\n  ^ at {:?}", severity, diag.message, diag.range)
        }
    }
}
/// Return the diagnostics module version.
#[allow(dead_code)]
pub fn diagnostics_module_version() -> &'static str {
    "0.1.1"
}
/// Return feature list for diagnostics module.
#[allow(dead_code)]
pub fn diagnostics_features() -> Vec<&'static str> {
    vec![
        "collect",
        "deduplicate",
        "sort",
        "suppress",
        "enrich",
        "publish",
        "budget",
        "threshold",
        "output-formats",
        "pipeline",
    ]
}
#[cfg(test)]
mod diagnostics_extra_tests {
    use super::*;
    #[test]
    fn test_diagnostic_pipeline() {
        let pipeline = DiagnosticPipeline::default_pipeline();
        let names = pipeline.stage_names();
        assert!(names.contains(&"collect"));
        assert!(names.contains(&"publish"));
        let enabled = pipeline.enabled_stages();
        assert_eq!(enabled.len(), names.len());
    }
    #[test]
    fn test_diagnostic_budget() {
        let budget = DiagnosticBudget::default();
        let diags: Vec<Diagnostic> = (0..600)
            .map(|i| Diagnostic {
                range: Range {
                    start: Position::new(i, 0),
                    end: Position::new(i, 5),
                },
                severity: DiagnosticSeverity::Warning,
                code: None,
                source: None,
                message: format!("diag {}", i),
            })
            .collect();
        let (trimmed, truncated) = budget.apply(diags);
        assert_eq!(trimmed.len(), 500);
        assert_eq!(truncated, 100);
    }
    #[test]
    fn test_enricher() {
        let enricher = DiagnosticEnricher::new();
        let enriched = enricher.enrich_message(DiagnosticCode::UnusedVariable, "unused var x");
        assert!(enriched.contains("prefix with _"));
    }
    #[test]
    fn test_diagnostic_threshold() {
        let threshold = DiagnosticThreshold {
            promote_warnings_to_errors: true,
            ..Default::default()
        };
        let result = threshold.apply(DiagnosticSeverity::Warning);
        assert_eq!(result, DiagnosticSeverity::Error);
    }
    #[test]
    fn test_diagnostics_features() {
        let features = diagnostics_features();
        assert!(features.contains(&"collect"));
        assert!(features.contains(&"budget"));
    }
    #[test]
    fn test_diagnostics_module_version() {
        assert!(!diagnostics_module_version().is_empty());
    }
}
#[cfg(test)]
mod diagnostics_cache_tests {
    use super::*;
    #[test]
    fn test_diagnostic_cache() {
        let mut cache = DiagnosticCache::new(10);
        let diag = Diagnostic {
            range: Range {
                start: Position::new(0, 0),
                end: Position::new(0, 5),
            },
            severity: DiagnosticSeverity::Error,
            code: None,
            source: None,
            message: "test".to_string(),
        };
        cache.store("file:///a.lean".to_string(), "v1".to_string(), vec![diag]);
        assert!(cache.get("file:///a.lean", "v1").is_some());
        assert!(cache.get("file:///a.lean", "v2").is_none());
        cache.invalidate_uri("file:///a.lean");
        assert!(cache.get("file:///a.lean", "v1").is_none());
    }
    #[test]
    fn test_diagnostic_subscription() {
        let sub = DiagnosticSubscription::all();
        let diag = Diagnostic {
            range: Range {
                start: Position::new(0, 0),
                end: Position::new(0, 5),
            },
            severity: DiagnosticSeverity::Hint,
            code: None,
            source: None,
            message: "hint".to_string(),
        };
        assert!(sub.matches("file:///a.lean", &diag));
        let err_sub = DiagnosticSubscription::errors_and_warnings("a.lean");
        assert!(!err_sub.matches("file:///a.lean", &diag));
    }
}
#[cfg(test)]
mod diff_tracker_tests {
    use super::*;
    fn make_diag(line: u32, msg: &str) -> Diagnostic {
        Diagnostic {
            range: Range {
                start: Position::new(line, 0),
                end: Position::new(line, 5),
            },
            severity: DiagnosticSeverity::Error,
            code: None,
            source: None,
            message: msg.to_string(),
        }
    }
    #[test]
    fn test_diff_tracker_new_resolved() {
        let mut tracker = DiagnosticDiffTracker::new();
        let run1 = vec![make_diag(0, "error A"), make_diag(1, "error B")];
        let (new1, resolved1) = tracker.update(&run1);
        assert_eq!(new1, 2);
        assert_eq!(resolved1, 0);
        let run2 = vec![make_diag(1, "error B"), make_diag(2, "error C")];
        let (new2, resolved2) = tracker.update(&run2);
        assert_eq!(new2, 1);
        assert_eq!(resolved2, 1);
    }
}
/// No-op placeholder.
#[allow(dead_code)]
pub fn diagnostics_noop() {}
#[cfg(test)]
mod aggregator_tests {
    use super::*;
    fn make_err(line: u32) -> Diagnostic {
        Diagnostic {
            range: Range {
                start: Position::new(line, 0),
                end: Position::new(line, 5),
            },
            severity: DiagnosticSeverity::Error,
            code: None,
            source: None,
            message: "err".to_string(),
        }
    }
    #[test]
    fn test_aggregator() {
        let mut agg = DiagnosticWorkspaceAggregator::new();
        agg.set_for_file("file:///a.lean".to_string(), vec![make_err(0), make_err(1)]);
        agg.set_for_file("file:///b.lean".to_string(), vec![make_err(5)]);
        assert_eq!(agg.total_errors(), 3);
        assert_eq!(agg.worst_file(), Some("file:///a.lean"));
    }
}
