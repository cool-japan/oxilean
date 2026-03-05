//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::HashMap;

use super::contextualhelp_type::ContextualHelp;
use super::examplefixes_type::ExampleFixes;
use super::types::{
    ElabMessage, ErrorChain, ErrorCode, ErrorContext, ErrorFormatter, ErrorReport, ErrorStats,
    ExtErrorCode, FixSuggestion, Language, MessageBatch, MsgSeverity, MultiSpanDiagnostic,
    NameDatabase, PrettyPrintOptions, Span, SpanLabel, Suggestions,
};

/// Compute Levenshtein distance between two strings
pub fn edit_distance(s1: &str, s2: &str) -> usize {
    let len1 = s1.len();
    let len2 = s2.len();
    let mut matrix = vec![vec![0; len2 + 1]; len1 + 1];
    for (i, row) in matrix.iter_mut().enumerate().take(len1 + 1) {
        row[0] = i;
    }
    for (j, val) in matrix[0].iter_mut().enumerate().take(len2 + 1) {
        *val = j;
    }
    for i in 1..=len1 {
        for j in 1..=len2 {
            let cost = if s1.chars().nth(i - 1) == s2.chars().nth(j - 1) {
                0
            } else {
                1
            };
            matrix[i][j] = std::cmp::min(
                std::cmp::min(matrix[i - 1][j] + 1, matrix[i][j - 1] + 1),
                matrix[i - 1][j - 1] + cost,
            );
        }
    }
    matrix[len1][len2]
}
/// Find similar names within edit distance threshold
pub fn find_similar(target: &str, candidates: &[&str], max_distance: usize) -> Vec<String> {
    let mut results: Vec<(String, usize)> = candidates
        .iter()
        .filter_map(|c| {
            let dist = edit_distance(target, c);
            if dist <= max_distance && dist > 0 {
                Some((c.to_string(), dist))
            } else {
                None
            }
        })
        .collect();
    results.sort_by_key(|x| x.1);
    results.into_iter().map(|(s, _)| s).collect()
}
/// Format a suggestion message
pub fn format_suggestion(suggestion: &str, kind: &str) -> String {
    format!("  --> Did you mean {}? `{}`", kind, suggestion)
}
/// Highlight code snippet around an error position
pub fn highlight_code(code: &str, line: usize, column: usize) -> String {
    let lines: Vec<&str> = code.lines().collect();
    if line == 0 || line > lines.len() {
        return String::new();
    }
    let error_line = lines[line - 1];
    let mut result = String::new();
    result.push_str(error_line);
    result.push('\n');
    for _ in 0..column {
        result.push(' ');
    }
    result.push_str("^\n");
    result
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::error_msg::*;
    #[test]
    fn test_error_code_number() {
        assert_eq!(ErrorCode::E1000.code_number(), 1000);
        assert_eq!(ErrorCode::E2000.code_number(), 2000);
        assert_eq!(ErrorCode::E3000.code_number(), 3000);
        assert_eq!(ErrorCode::E4000.code_number(), 4000);
        assert_eq!(ErrorCode::E5000.code_number(), 5000);
    }
    #[test]
    fn test_error_code_description() {
        assert_eq!(ErrorCode::E1000.description(), "Unexpected token");
        assert_eq!(ErrorCode::E2000.description(), "Type mismatch");
        assert_eq!(ErrorCode::E3000.description(), "Undefined variable");
    }
    #[test]
    fn test_edit_distance_identical() {
        assert_eq!(edit_distance("hello", "hello"), 0);
        assert_eq!(edit_distance("", ""), 0);
    }
    #[test]
    fn test_edit_distance_one_char() {
        assert_eq!(edit_distance("a", ""), 1);
        assert_eq!(edit_distance("", "a"), 1);
    }
    #[test]
    fn test_edit_distance_substitution() {
        assert_eq!(edit_distance("cat", "bat"), 1);
        assert_eq!(edit_distance("kitten", "sitting"), 3);
    }
    #[test]
    fn test_find_similar() {
        let candidates = vec!["hello", "hallo", "world", "hero"];
        let similar = find_similar("hello", &candidates, 2);
        assert!(!similar.is_empty());
        assert!(similar.contains(&"hallo".to_string()));
    }
    #[test]
    fn test_format_suggestion() {
        let sugg = format_suggestion("list_concat", "function");
        assert!(sugg.contains("list_concat"));
        assert!(sugg.contains("function"));
    }
    #[test]
    fn test_error_formatter_type_error() {
        let msg = ErrorFormatter::format_type_error("Nat", "String", "function application");
        assert!(msg.contains("E2000"));
        assert!(msg.contains("Nat"));
        assert!(msg.contains("String"));
    }
    #[test]
    fn test_error_formatter_name_error() {
        let suggestions = vec!["foo".to_string(), "foobar".to_string()];
        let msg = ErrorFormatter::format_name_error("fo", &suggestions);
        assert!(msg.contains("E3000"));
        assert!(msg.contains("foo"));
    }
    #[test]
    fn test_suggestions_similar_name() {
        let available = vec!["append", "apply", "aprove"];
        let sugg = Suggestions::suggest_similar_name("appley", &available);
        assert!(sugg.is_some());
    }
    #[test]
    fn test_suggestions_import() {
        let modules = vec!["Data.List", "Data.Set", "Data.Array"];
        let sugg = Suggestions::suggest_import("List", &modules);
        assert!(sugg.is_some());
        assert!(sugg
            .expect("test operation should succeed")
            .contains("Data.List"));
    }
    #[test]
    fn test_suggestions_tactic() {
        let tactics = Suggestions::suggest_tactic("∀ x, P x");
        assert!(!tactics.is_empty());
        assert!(tactics.contains(&"intro".to_string()));
    }
    #[test]
    fn test_error_context_creation() {
        let ctx = ErrorContext::new(ErrorCode::E2000, "Type mismatch".to_string(), 5, 10);
        assert_eq!(ctx.code, ErrorCode::E2000);
        assert_eq!(ctx.line, 5);
        assert_eq!(ctx.column, 10);
    }
    #[test]
    fn test_error_context_with_suggestions() {
        let ctx = ErrorContext::new(ErrorCode::E3000, "Not found".to_string(), 1, 1)
            .with_suggestions(vec!["append".to_string()]);
        assert_eq!(ctx.suggestions.len(), 1);
    }
    #[test]
    fn test_error_context_format() {
        let ctx = ErrorContext::new(ErrorCode::E2000, "Type mismatch".to_string(), 5, 10);
        let formatted = ctx.format_full();
        assert!(formatted.contains("E2000"));
        assert!(formatted.contains("line 5"));
        assert!(formatted.contains("column 10"));
    }
    #[test]
    fn test_error_report() {
        let mut report = ErrorReport::new();
        assert!(!report.has_errors());
        let error = ErrorContext::new(ErrorCode::E2000, "Error".to_string(), 1, 1);
        report.add_error(error);
        assert!(report.has_errors());
        assert_eq!(report.error_count(), 1);
    }
    #[test]
    fn test_contextual_help_type_mismatch() {
        let help = ContextualHelp::help_for_type_mismatch();
        assert!(help.contains("Type mismatch"));
        assert!(help.contains("annotation"));
    }
    #[test]
    fn test_example_fix_type_mismatch() {
        let example = ExampleFixes::example_fix_type_mismatch();
        assert!(example.contains("BEFORE"));
        assert!(example.contains("AFTER"));
    }
    #[test]
    fn test_error_code_display() {
        let code = ErrorCode::E2000;
        assert_eq!(format!("{}", code), "E2000");
    }
}
#[cfg(test)]
mod extra_error_msg_tests {
    use super::*;
    use crate::error_msg::*;
    #[test]
    fn test_msg_severity_ordering() {
        assert!(MsgSeverity::Info < MsgSeverity::Warning);
        assert!(MsgSeverity::Warning < MsgSeverity::Error);
        assert!(MsgSeverity::Error < MsgSeverity::Fatal);
    }
    #[test]
    fn test_msg_severity_display() {
        assert_eq!(format!("{}", MsgSeverity::Error), "error");
        assert_eq!(format!("{}", MsgSeverity::Warning), "warning");
    }
    #[test]
    fn test_elab_message_error() {
        let m = ElabMessage::error(ErrorCode::E2000, "type mismatch");
        assert_eq!(m.severity, MsgSeverity::Error);
        assert!(m.text.contains("type mismatch"));
    }
    #[test]
    fn test_elab_message_warning() {
        let m = ElabMessage::warning(ErrorCode::E1004, "shadow");
        assert_eq!(m.severity, MsgSeverity::Warning);
    }
    #[test]
    fn test_elab_message_info() {
        let m = ElabMessage::info("just a note");
        assert!(m.code.is_none());
        assert_eq!(m.severity, MsgSeverity::Info);
    }
    #[test]
    fn test_elab_message_at() {
        let m = ElabMessage::error(ErrorCode::E2000, "x").at(5, 10);
        assert_eq!(m.location, Some((5, 10)));
    }
    #[test]
    fn test_elab_message_format_diagnostic() {
        let m = ElabMessage::error(ErrorCode::E2000, "bad type").at(3, 7);
        let s = m.format_diagnostic();
        assert!(s.contains("E2000"));
        assert!(s.contains("bad type"));
        assert!(s.contains("3:7"));
    }
    #[test]
    fn test_message_batch_has_errors() {
        let mut b = MessageBatch::new();
        b.add(ElabMessage::info("ok"));
        assert!(!b.has_errors());
        b.add(ElabMessage::error(ErrorCode::E2000, "err"));
        assert!(b.has_errors());
    }
    #[test]
    fn test_message_batch_errors_warnings() {
        let mut b = MessageBatch::new();
        b.add(ElabMessage::error(ErrorCode::E2000, "e"));
        b.add(ElabMessage::warning(ErrorCode::E1000, "w"));
        assert_eq!(b.errors().len(), 1);
        assert_eq!(b.warnings().len(), 1);
    }
    #[test]
    fn test_message_batch_len() {
        let mut b = MessageBatch::new();
        assert!(b.is_empty());
        b.add(ElabMessage::info("x"));
        assert_eq!(b.len(), 1);
    }
}
/// Deduplicate a list of error contexts by (code, message, line, column).
#[allow(dead_code)]
pub fn deduplicate_errors(errors: Vec<ErrorContext>) -> Vec<ErrorContext> {
    let mut seen: std::collections::HashSet<(u32, String, usize, usize)> =
        std::collections::HashSet::new();
    let mut result = Vec::new();
    for err in errors {
        let key = (
            err.code.code_number(),
            err.message.clone(),
            err.line,
            err.column,
        );
        if seen.insert(key) {
            result.push(err);
        }
    }
    result
}
/// Deduplicate `ElabMessage` by (severity, text).
#[allow(dead_code)]
pub fn deduplicate_messages(messages: Vec<ElabMessage>) -> Vec<ElabMessage> {
    let mut seen: std::collections::HashSet<(String, String)> = std::collections::HashSet::new();
    let mut result = Vec::new();
    for msg in messages {
        let key = (format!("{}", msg.severity), msg.text.clone());
        if seen.insert(key) {
            result.push(msg);
        }
    }
    result
}
/// Group a list of errors by their error code.
#[allow(dead_code)]
pub fn group_errors_by_code(errors: &[ErrorContext]) -> HashMap<u32, Vec<&ErrorContext>> {
    let mut groups: HashMap<u32, Vec<&ErrorContext>> = HashMap::new();
    for err in errors {
        groups.entry(err.code.code_number()).or_default().push(err);
    }
    groups
}
/// Group errors by source line.
#[allow(dead_code)]
pub fn group_errors_by_line(errors: &[ErrorContext]) -> HashMap<usize, Vec<&ErrorContext>> {
    let mut groups: HashMap<usize, Vec<&ErrorContext>> = HashMap::new();
    for err in errors {
        groups.entry(err.line).or_default().push(err);
    }
    groups
}
/// Return only errors at or above a minimum severity level.
#[allow(dead_code)]
pub fn filter_by_severity(messages: &[ElabMessage], min: MsgSeverity) -> Vec<&ElabMessage> {
    messages.iter().filter(|m| m.severity >= min).collect()
}
/// Produce a minimal SARIF JSON string for a list of `ElabMessage`s.
///
/// This is a stub — real SARIF requires a full JSON serializer.
#[allow(dead_code)]
pub fn to_sarif_json(messages: &[ElabMessage], tool_name: &str) -> String {
    let mut results = Vec::new();
    for msg in messages {
        let code_str = msg
            .code
            .map(|c| format!("{}", c))
            .unwrap_or_else(|| "NONE".to_string());
        let loc_str = msg
            .location
            .map(|(l, c)| format!("\"line\":{},\"column\":{}", l, c))
            .unwrap_or_else(|| "\"line\":0,\"column\":0".to_string());
        results
            .push(
                format!(
                    "{{\"ruleId\":\"{}\",\"message\":{{\"text\":\"{}\"}},\"locations\":[{{\"physicalLocation\":{{{}}}}}]}}",
                    code_str, msg.text.replace('"', "'"), loc_str,
                ),
            );
    }
    format!(
        "{{\"version\":\"2.1.0\",\"runs\":[{{\"tool\":{{\"driver\":{{\"name\":\"{}\"}}}},\"results\":[{}]}}]}}",
        tool_name, results.join(","),
    )
}
/// Translate a `MsgSeverity` label into the requested language.
#[allow(dead_code)]
pub fn localise_severity(severity: MsgSeverity, lang: Language) -> &'static str {
    match (severity, lang) {
        (MsgSeverity::Error, Language::Japanese) => "エラー",
        (MsgSeverity::Warning, Language::Japanese) => "警告",
        (MsgSeverity::Info, Language::Japanese) => "情報",
        (MsgSeverity::Hint, Language::Japanese) => "ヒント",
        (MsgSeverity::Fatal, Language::Japanese) => "致命的エラー",
        (MsgSeverity::Error, Language::German) => "Fehler",
        (MsgSeverity::Warning, Language::German) => "Warnung",
        (MsgSeverity::Info, Language::German) => "Info",
        (MsgSeverity::Hint, Language::German) => "Hinweis",
        (MsgSeverity::Fatal, Language::German) => "Kritischer Fehler",
        (MsgSeverity::Error, Language::French) => "Erreur",
        (MsgSeverity::Warning, Language::French) => "Avertissement",
        (MsgSeverity::Info, Language::French) => "Information",
        (MsgSeverity::Hint, Language::French) => "Indication",
        (MsgSeverity::Fatal, Language::French) => "Erreur fatale",
        (MsgSeverity::Error, _) => "error",
        (MsgSeverity::Warning, _) => "warning",
        (MsgSeverity::Info, _) => "info",
        (MsgSeverity::Hint, _) => "hint",
        (MsgSeverity::Fatal, _) => "fatal",
    }
}
/// Pretty-print an `ErrorContext` according to the given options.
#[allow(dead_code)]
pub fn pretty_print_error(err: &ErrorContext, opts: &PrettyPrintOptions) -> String {
    let severity_label = "error";
    let mut lines = Vec::new();
    lines.push(format!(
        "[{}] {}: {}",
        err.code, severity_label, err.message
    ));
    lines.push(format!("  --> {}:{}", err.line, err.column));
    if opts.show_suggestions {
        for sugg in &err.suggestions {
            lines.push(format!("  help: {}", sugg));
        }
    }
    if opts.show_help {
        if let Some(help) = &err.help {
            lines.push(format!("  note: {}", help));
        }
    }
    lines.join("\n")
}
/// Pretty-print a `MessageBatch` according to the given options.
#[allow(dead_code)]
pub fn pretty_print_batch(batch: &MessageBatch, opts: &PrettyPrintOptions) -> String {
    batch
        .iter()
        .map(|m| {
            let sev = localise_severity(m.severity, opts.language);
            let code_str = m.code.map(|c| format!("[{}] ", c)).unwrap_or_default();
            let loc_str = m
                .location
                .map(|(l, c)| format!(" ({}:{})", l, c))
                .unwrap_or_default();
            format!("{}{}: {}{}", code_str, sev, m.text, loc_str)
        })
        .collect::<Vec<_>>()
        .join("\n")
}
/// Annotate source lines around an error, emitting diff-style context.
///
/// Lines before and after the error are shown with line numbers.
/// The error line is marked with `>`.
#[allow(dead_code)]
pub fn annotate_source(source: &str, error_line: usize, context_lines: usize) -> String {
    let lines: Vec<&str> = source.lines().collect();
    let total = lines.len();
    if error_line == 0 || error_line > total {
        return format!("<source line {} not found>", error_line);
    }
    let start = error_line.saturating_sub(context_lines + 1);
    let end = (error_line + context_lines).min(total);
    let mut result = String::new();
    for (i, line) in lines[start..end].iter().enumerate() {
        let lineno = start + i + 1;
        let marker = if lineno == error_line { ">" } else { " " };
        result.push_str(&format!("{} {:4} | {}\n", marker, lineno, line));
    }
    result
}
/// Annotate a source line with a caret pointing to `column`.
#[allow(dead_code)]
pub fn caret_annotation(line: &str, column: usize, length: usize) -> String {
    let _line_display = line;
    let spaces = " ".repeat(column);
    let carets = "^".repeat(length.max(1));
    format!("{}{}", spaces, carets)
}
/// Heuristic recovery hint for a given error code.
#[allow(dead_code)]
pub fn recovery_hint(code: ErrorCode) -> &'static str {
    match code {
        ErrorCode::E1000 => "Remove or replace the unexpected token.",
        ErrorCode::E1001 => "Insert an identifier at this position.",
        ErrorCode::E1002 => "Add the missing closing bracket or parenthesis.",
        ErrorCode::E1003 => "Check the syntax of the surrounding expression.",
        ErrorCode::E1004 => "Rename one of the duplicate definitions.",
        ErrorCode::E1005 => "Use a valid escape sequence such as \\n, \\t, or \\\\.",
        ErrorCode::E1006 => "Ensure the file is complete and not truncated.",
        ErrorCode::E1007 => "Replace with a valid operator for this context.",
        ErrorCode::E1008 => "Add a semicolon after the statement.",
        ErrorCode::E1009 => "Use a valid integer or floating-point literal.",
        ErrorCode::E1010 => "Close the string with a matching quote.",
        ErrorCode::E2000 => "Add a type annotation or fix the function argument.",
        ErrorCode::E2001 => "Apply the term to its argument or fix the type.",
        ErrorCode::E2002 => "Use a pair or sigma type instead.",
        ErrorCode::E2003 => "Provide the correct number of arguments.",
        ErrorCode::E2004 => "Add an explicit type annotation: (x : T).",
        ErrorCode::E2005 => "Check that the two types are compatible.",
        ErrorCode::E2006 => "Remove the recursive occurrence in the type.",
        ErrorCode::E2007 => "Add enough type annotations to disambiguate.",
        ErrorCode::E2008 => "Use a valid type constructor.",
        ErrorCode::E2009 => "Add a termination proof or use a well-founded measure.",
        ErrorCode::E2010 => "Increase the universe level or use polymorphism.",
        ErrorCode::E2011 => "Fix the kind of the type expression.",
        ErrorCode::E2012 => "Avoid instantiating an impredicative type at a polymorphic level.",
        ErrorCode::E2013 => "Check the type constraint for a solution.",
        ErrorCode::E2014 => "Add type annotations to disambiguate.",
        ErrorCode::E2015 => "Ensure both sides use the same type constructor.",
        ErrorCode::E2016 => "Add an explicit coercion or fix the source type.",
        ErrorCode::E2017 => "Bring the type parameter into scope.",
        ErrorCode::E2018 => "Use a supported type feature.",
        ErrorCode::E2019 => "Specify which type you intend.",
        ErrorCode::E2020 => "Use a fresh type variable name.",
        ErrorCode::E3000 => "Check spelling or import the required module.",
        ErrorCode::E3001 => "Define or import the missing type.",
        ErrorCode::E3002 => "Import the module before using it.",
        ErrorCode::E3003 => "Check the field name and record type.",
        ErrorCode::E3004 => "Qualify the name with its module prefix.",
        ErrorCode::E3005 => "Rename the inner binding to avoid shadowing.",
        ErrorCode::E3006 => "Make the name public or use a re-export.",
        ErrorCode::E3007 => "Resolve the import conflict explicitly.",
        ErrorCode::E3008 => "Break the circular import cycle.",
        ErrorCode::E3009 => "Open the namespace or qualify the name.",
        ErrorCode::E3010 => "Fix the namespace path.",
        ErrorCode::E4000 => "Use a higher universe level.",
        ErrorCode::E4001 => "Fix the universe constraint.",
        ErrorCode::E4002 => "Use a valid universe variable name.",
        ErrorCode::E4003 => "Ensure universe constraints are consistent.",
        ErrorCode::E4004 => "Avoid impredicative usage in a predicative universe.",
        ErrorCode::E4005 => "Provide explicit universe levels.",
        ErrorCode::E4006 => "Check universe polymorphism annotations.",
        ErrorCode::E4007 => "Use distinct universe variable names.",
        ErrorCode::E4008 => "Use Prop or Type 0 appropriately.",
        ErrorCode::E4009 => "Remove the cyclic universe dependency.",
        ErrorCode::E4010 => "Use a valid universe elimination rule.",
        ErrorCode::E5000 => "Add missing cases or a wildcard pattern.",
        ErrorCode::E5001 => "Remove the unreachable pattern.",
        ErrorCode::E5002 => "Use a valid pattern for this type.",
        ErrorCode::E5003 => "Fix the pattern to match the scrutinee type.",
        ErrorCode::E5004 => "Remove the duplicate pattern.",
        ErrorCode::E5005 => "Bind the pattern variable on the left-hand side.",
        ErrorCode::E5006 => "Fix the as-pattern binding.",
        ErrorCode::E5007 => "Move the wildcard to the last position.",
        ErrorCode::E5008 => "Match on the function result rather than the function.",
        ErrorCode::E5009 => "Fix the guard expression to be a Bool.",
        ErrorCode::E5010 => "Use a valid guard expression.",
    }
}
#[cfg(test)]
mod extended_error_msg_tests {
    use super::*;
    use crate::error_msg::*;
    #[test]
    fn test_ext_error_code_number() {
        assert_eq!(ExtErrorCode::E3100.code_number(), 3100);
        assert_eq!(ExtErrorCode::E4100.code_number(), 4100);
        assert_eq!(ExtErrorCode::E5100.code_number(), 5100);
    }
    #[test]
    fn test_ext_error_code_description() {
        let d = ExtErrorCode::E4101.description();
        assert!(d.contains("Orphan"));
    }
    #[test]
    fn test_ext_error_code_is_hard_error() {
        assert!(ExtErrorCode::E5100.is_hard_error());
        assert!(!ExtErrorCode::E3105.is_hard_error());
    }
    #[test]
    fn test_ext_error_code_display() {
        assert_eq!(format!("{}", ExtErrorCode::E4102), "E4102");
    }
    #[test]
    fn test_span_len() {
        let s = Span::new(10, 20, 3, 5);
        assert_eq!(s.len(), 10);
    }
    #[test]
    fn test_span_is_empty() {
        let s = Span::point(1, 0, 5);
        assert!(s.is_empty());
    }
    #[test]
    fn test_span_contains_offset() {
        let s = Span::new(10, 20, 1, 0);
        assert!(s.contains_offset(15));
        assert!(!s.contains_offset(5));
        assert!(!s.contains_offset(20));
    }
    #[test]
    fn test_span_merge() {
        let a = Span::new(5, 10, 1, 5);
        let b = Span::new(8, 15, 1, 8);
        let merged = a.merge(b);
        assert_eq!(merged.start, 5);
        assert_eq!(merged.end, 15);
    }
    #[test]
    fn test_span_format_location() {
        let s = Span::new(0, 5, 3, 7);
        assert_eq!(s.format_location(), "3:7");
    }
    #[test]
    fn test_span_label_primary() {
        let s = Span::point(1, 0, 0);
        let label = SpanLabel::primary(s, "here");
        assert!(label.is_primary);
        assert_eq!(label.label, "here");
    }
    #[test]
    fn test_span_label_secondary() {
        let s = Span::point(2, 0, 5);
        let label = SpanLabel::secondary(s, "context");
        assert!(!label.is_primary);
    }
    #[test]
    fn test_multi_span_diagnostic_creation() {
        let diag = MultiSpanDiagnostic::new(ErrorCode::E2000, MsgSeverity::Error, "bad type");
        assert_eq!(diag.code, ErrorCode::E2000);
        assert!(diag.spans.is_empty());
        assert!(diag.notes.is_empty());
    }
    #[test]
    fn test_multi_span_diagnostic_with_primary() {
        let s = Span::point(5, 10, 20);
        let diag = MultiSpanDiagnostic::new(ErrorCode::E2000, MsgSeverity::Error, "err")
            .with_primary(s, "type error here");
        assert_eq!(diag.spans.len(), 1);
        assert!(diag.primary_span().is_some());
    }
    #[test]
    fn test_multi_span_diagnostic_secondary_spans() {
        let s1 = Span::point(1, 0, 0);
        let s2 = Span::point(2, 0, 10);
        let diag = MultiSpanDiagnostic::new(ErrorCode::E2000, MsgSeverity::Error, "x")
            .with_primary(s1, "main")
            .with_secondary(s2, "note");
        assert_eq!(diag.secondary_spans().len(), 1);
    }
    #[test]
    fn test_multi_span_diagnostic_format_compact() {
        let s = Span::point(3, 7, 30);
        let diag = MultiSpanDiagnostic::new(ErrorCode::E2000, MsgSeverity::Error, "type mismatch")
            .with_primary(s, "here");
        let formatted = diag.format_compact();
        assert!(formatted.contains("E2000"));
        assert!(formatted.contains("type mismatch"));
        assert!(formatted.contains("3:7"));
    }
    #[test]
    fn test_multi_span_diagnostic_with_note() {
        let diag = MultiSpanDiagnostic::new(ErrorCode::E2000, MsgSeverity::Error, "x")
            .with_note("consider adding a type annotation");
        assert_eq!(diag.notes.len(), 1);
    }
    #[test]
    fn test_fix_suggestion_creation() {
        let s = Span::point(1, 0, 0);
        let fix = FixSuggestion::new("add annotation", "x : Nat", s, 0.9);
        assert!(fix.is_confident());
        assert!(fix.format().contains("90%"));
    }
    #[test]
    fn test_fix_suggestion_not_confident() {
        let s = Span::point(1, 0, 0);
        let fix = FixSuggestion::new("maybe", "x", s, 0.5);
        assert!(!fix.is_confident());
    }
    #[test]
    fn test_deduplicate_errors() {
        let e1 = ErrorContext::new(ErrorCode::E2000, "bad type".to_string(), 1, 1);
        let e2 = ErrorContext::new(ErrorCode::E2000, "bad type".to_string(), 1, 1);
        let e3 = ErrorContext::new(ErrorCode::E3000, "not found".to_string(), 2, 1);
        let result = deduplicate_errors(vec![e1, e2, e3]);
        assert_eq!(result.len(), 2);
    }
    #[test]
    fn test_deduplicate_messages() {
        let m1 = ElabMessage::error(ErrorCode::E2000, "x");
        let m2 = ElabMessage::error(ErrorCode::E2000, "x");
        let m3 = ElabMessage::info("different");
        let result = deduplicate_messages(vec![m1, m2, m3]);
        assert_eq!(result.len(), 2);
    }
    #[test]
    fn test_error_stats_from_errors() {
        let e1 = ErrorContext::new(ErrorCode::E2000, "x".to_string(), 1, 1);
        let e2 = ErrorContext::new(ErrorCode::E3000, "y".to_string(), 2, 1);
        let stats = ErrorStats::from_errors(&[e1, e2]);
        assert_eq!(stats.total, 2);
        assert_eq!(stats.count_category(2), 1);
        assert_eq!(stats.count_category(3), 1);
    }
    #[test]
    fn test_error_stats_summary() {
        let stats = ErrorStats::new();
        let s = stats.summary();
        assert!(s.contains("total=0"));
    }
    #[test]
    fn test_error_stats_from_batch() {
        let mut batch = MessageBatch::new();
        batch.add(ElabMessage::error(ErrorCode::E2000, "x"));
        batch.add(ElabMessage::warning(ErrorCode::E1000, "y"));
        let stats = ErrorStats::from_batch(&batch);
        assert_eq!(stats.total, 2);
    }
    #[test]
    fn test_group_errors_by_code() {
        let e1 = ErrorContext::new(ErrorCode::E2000, "x".to_string(), 1, 1);
        let e2 = ErrorContext::new(ErrorCode::E2000, "y".to_string(), 2, 1);
        let e3 = ErrorContext::new(ErrorCode::E3000, "z".to_string(), 3, 1);
        let binding = [e1, e2, e3];
        let groups = group_errors_by_code(&binding);
        assert_eq!(groups.get(&2000).expect("key should exist").len(), 2);
        assert_eq!(groups.get(&3000).expect("key should exist").len(), 1);
    }
    #[test]
    fn test_group_errors_by_line() {
        let e1 = ErrorContext::new(ErrorCode::E2000, "x".to_string(), 5, 1);
        let e2 = ErrorContext::new(ErrorCode::E3000, "y".to_string(), 5, 2);
        let e3 = ErrorContext::new(ErrorCode::E2000, "z".to_string(), 7, 1);
        let binding = [e1, e2, e3];
        let groups = group_errors_by_line(&binding);
        assert_eq!(groups.get(&5).expect("key should exist").len(), 2);
        assert_eq!(groups.get(&7).expect("key should exist").len(), 1);
    }
    #[test]
    fn test_filter_by_severity() {
        let messages = vec![
            ElabMessage::info("info"),
            ElabMessage::warning(ErrorCode::E1000, "warn"),
            ElabMessage::error(ErrorCode::E2000, "err"),
        ];
        let filtered = filter_by_severity(&messages, MsgSeverity::Warning);
        assert_eq!(filtered.len(), 2);
    }
    #[test]
    fn test_to_sarif_json_empty() {
        let sarif = to_sarif_json(&[], "OxiLean");
        assert!(sarif.contains("OxiLean"));
        assert!(sarif.contains("results"));
    }
    #[test]
    fn test_to_sarif_json_with_messages() {
        let msgs = vec![ElabMessage::error(ErrorCode::E2000, "type mismatch").at(3, 7)];
        let sarif = to_sarif_json(&msgs, "OxiLean");
        assert!(sarif.contains("E2000"));
        assert!(sarif.contains("type mismatch"));
    }
    #[test]
    fn test_localise_severity_english() {
        assert_eq!(
            localise_severity(MsgSeverity::Error, Language::English),
            "error"
        );
    }
    #[test]
    fn test_localise_severity_japanese() {
        assert_eq!(
            localise_severity(MsgSeverity::Error, Language::Japanese),
            "エラー"
        );
        assert_eq!(
            localise_severity(MsgSeverity::Warning, Language::Japanese),
            "警告"
        );
    }
    #[test]
    fn test_localise_severity_german() {
        assert_eq!(
            localise_severity(MsgSeverity::Error, Language::German),
            "Fehler"
        );
    }
    #[test]
    fn test_pretty_print_options_default() {
        let opts = PrettyPrintOptions::default();
        assert_eq!(opts.max_width, 120);
        assert!(!opts.use_colour);
        assert!(opts.show_suggestions);
    }
    #[test]
    fn test_pretty_print_options_minimal() {
        let opts = PrettyPrintOptions::minimal();
        assert!(!opts.show_suggestions);
        assert!(!opts.show_help);
    }
    #[test]
    fn test_pretty_print_error() {
        let err = ErrorContext::new(ErrorCode::E2000, "type mismatch".to_string(), 5, 10);
        let opts = PrettyPrintOptions::default();
        let output = pretty_print_error(&err, &opts);
        assert!(output.contains("E2000"));
        assert!(output.contains("5:10"));
    }
    #[test]
    fn test_pretty_print_batch() {
        let mut batch = MessageBatch::new();
        batch.add(ElabMessage::error(ErrorCode::E2000, "bad type").at(1, 1));
        let opts = PrettyPrintOptions::default();
        let output = pretty_print_batch(&batch, &opts);
        assert!(output.contains("E2000"));
        assert!(output.contains("bad type"));
    }
    #[test]
    fn test_name_database_add_contains() {
        let mut db = NameDatabase::new();
        db.add("Nat.add");
        db.add("Nat.mul");
        assert!(db.contains("Nat.add"));
        assert!(!db.contains("Nat.sub"));
    }
    #[test]
    fn test_name_database_completions() {
        let mut db = NameDatabase::new();
        db.add_all(&["List.map", "List.filter", "List.foldl"]);
        let completions = db.completions("List");
        assert_eq!(completions.len(), 3);
    }
    #[test]
    fn test_name_database_len() {
        let mut db = NameDatabase::new();
        assert!(db.is_empty());
        db.add("a");
        db.add("b");
        assert_eq!(db.len(), 2);
    }
    #[test]
    fn test_name_database_suggest() {
        let mut db = NameDatabase::new();
        db.add_all(&["append", "apply", "approve"]);
        let suggestions = db.suggest("appli", 2);
        assert!(!suggestions.is_empty());
    }
    #[test]
    fn test_annotate_source() {
        let source = "line one\nline two\nline three\nline four\nline five";
        let annotated = annotate_source(source, 3, 1);
        assert!(annotated.contains("line three"));
        assert!(annotated.contains(">"));
    }
    #[test]
    fn test_annotate_source_out_of_bounds() {
        let annotated = annotate_source("a\nb", 99, 1);
        assert!(annotated.contains("not found"));
    }
    #[test]
    fn test_caret_annotation() {
        let ann = caret_annotation("let x := 42", 4, 1);
        assert!(ann.contains("^"));
        assert_eq!(ann, "    ^");
    }
    #[test]
    fn test_error_chain_push_len() {
        let mut chain = ErrorChain::new();
        assert!(chain.is_empty());
        chain.push(ElabMessage::error(ErrorCode::E2000, "outer error"));
        chain.push(ElabMessage::error(ErrorCode::E3000, "inner error"));
        assert_eq!(chain.len(), 2);
    }
    #[test]
    fn test_error_chain_root_immediate() {
        let mut chain = ErrorChain::new();
        chain.push(ElabMessage::error(ErrorCode::E2000, "root"));
        chain.push(ElabMessage::error(ErrorCode::E3000, "leaf"));
        assert!(chain
            .root_cause()
            .expect("test operation should succeed")
            .text
            .contains("root"));
        assert!(chain
            .immediate()
            .expect("test operation should succeed")
            .text
            .contains("leaf"));
    }
    #[test]
    fn test_error_chain_format() {
        let mut chain = ErrorChain::new();
        chain.push(ElabMessage::error(ErrorCode::E2000, "bad type"));
        let formatted = chain.format_chain();
        assert!(formatted.contains("#0"));
        assert!(formatted.contains("bad type"));
    }
    #[test]
    fn test_error_chain_clear() {
        let mut chain = ErrorChain::new();
        chain.push(ElabMessage::info("x"));
        chain.clear();
        assert!(chain.is_empty());
    }
    #[test]
    fn test_recovery_hint_e1000() {
        let hint = recovery_hint(ErrorCode::E1000);
        assert!(!hint.is_empty());
    }
    #[test]
    fn test_recovery_hint_e2000() {
        let hint = recovery_hint(ErrorCode::E2000);
        assert!(hint.contains("type"));
    }
    #[test]
    fn test_recovery_hint_e5000() {
        let hint = recovery_hint(ErrorCode::E5000);
        assert!(hint.contains("pattern") || hint.contains("wildcard"));
    }
    #[test]
    fn test_highlight_code_valid() {
        let code = "def foo := 42\ndef bar := 0";
        let highlighted = highlight_code(code, 1, 4);
        assert!(highlighted.contains("def foo"));
        assert!(highlighted.contains("^"));
    }
    #[test]
    fn test_highlight_code_invalid_line() {
        let code = "one line";
        let highlighted = highlight_code(code, 99, 0);
        assert!(highlighted.is_empty());
    }
}
