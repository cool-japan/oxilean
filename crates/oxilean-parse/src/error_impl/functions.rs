//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::tokens::Span;

use super::types::{
    AnnotatedSpan, DiagnosticSeverity, FullDiagnostic, LocatedError, ParseError, ParseErrorKind,
    ParseErrors, RecoveryHint,
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error_impl::*;
    use crate::tokens::TokenKind;
    #[test]
    fn test_parse_error_display() {
        let err = ParseError::unexpected(
            vec!["identifier".to_string()],
            TokenKind::LParen,
            Span::new(10, 11, 2, 5),
        );
        let msg = format!("{}", err);
        assert!(msg.contains("line 2"));
        assert!(msg.contains("column 5"));
        assert!(msg.contains("expected identifier"));
    }
    #[test]
    fn test_unexpected_eof() {
        let err =
            ParseError::unexpected_eof(vec!["expression".to_string()], Span::new(100, 100, 10, 1));
        assert!(matches!(err.kind, ParseErrorKind::UnexpectedEof { .. }));
    }
}
/// Format a list of expected token descriptions as a human-readable string.
pub fn format_expected(expected: &[String]) -> String {
    match expected.len() {
        0 => "something".to_string(),
        1 => expected[0].clone(),
        2 => format!("{} or {}", expected[0], expected[1]),
        _ => {
            let (last, rest) = expected
                .split_last()
                .expect("slice has len >= 3 per match arm");
            format!("{}, or {}", rest.join(", "), last)
        }
    }
}
/// Suggest a correction for a common typo in keywords.
pub fn suggest_correction(got: &str) -> Option<&'static str> {
    match got {
        "Def" | "def" | "defn" => Some("definition"),
        "thm" | "Thm" | "Theorem" => Some("theorem"),
        "Axiom" => Some("axiom"),
        "fun " | "fn" => Some("fun"),
        "match " | "Match" => Some("match"),
        _ => None,
    }
}
/// Truncate source text to at most `max_len` chars, appending `…` if needed.
pub fn truncate_source(src: &str, max_len: usize) -> String {
    if src.chars().count() <= max_len {
        src.to_string()
    } else {
        let truncated: String = src.chars().take(max_len).collect();
        format!("{}…", truncated)
    }
}
/// Return a string that underlines the span within its line.
pub fn underline_span(line: &str, col: usize, len: usize) -> String {
    let col = col.saturating_sub(1);
    let spaces = " ".repeat(col);
    let len = len.max(1).min(line.len().saturating_sub(col));
    let carets = "^".repeat(len);
    format!("{}{}", spaces, carets)
}
/// Generate recovery hints for common parse errors.
pub fn recovery_hints(err: &ParseError) -> Vec<RecoveryHint> {
    let mut hints = Vec::new();
    match &err.kind {
        ParseErrorKind::UnexpectedEof { .. } => {
            hints.push(RecoveryHint::new(
                "Did you forget a closing `)`, `}`, or `]`?",
            ));
        }
        ParseErrorKind::InvalidBinder(msg) => {
            if msg.contains("type") {
                hints.push(RecoveryHint::new(
                    "Binders require a type annotation, e.g. `(x : Nat)`",
                ));
            }
        }
        ParseErrorKind::InvalidSyntax(msg) => {
            if msg.contains("=>") {
                hints.push(
                    RecoveryHint::new("OxiLean uses `->` for arrows, not `=>`")
                        .with_replacement("->"),
                );
            }
        }
        _ => {}
    }
    hints
}
#[cfg(test)]
mod error_extra_tests {
    use super::*;
    use crate::error_impl::*;
    use crate::tokens::TokenKind;
    fn make_err(kind: ParseErrorKind) -> ParseError {
        ParseError::new(kind, Span::new(0, 5, 1, 1))
    }
    #[test]
    fn test_severity_ordering() {
        assert!(Severity::Note < Severity::Warning);
        assert!(Severity::Warning < Severity::Error);
    }
    #[test]
    fn test_severity_display() {
        assert_eq!(Severity::Error.to_string(), "error");
        assert_eq!(Severity::Warning.to_string(), "warning");
        assert_eq!(Severity::Note.to_string(), "note");
    }
    #[test]
    fn test_diagnostic_report() {
        let err = ParseError::unexpected(
            vec!["identifier".to_string()],
            TokenKind::LParen,
            Span::new(0, 5, 1, 1),
        );
        let diag = Diagnostic::error(err).with_hint("try writing an identifier here");
        let report = diag.report("(hello)");
        assert!(report.contains("error"));
        assert!(report.contains("hint"));
    }
    #[test]
    fn test_parse_errors_collection() {
        let mut errs = ParseErrors::new();
        errs.add_error(make_err(ParseErrorKind::Other("oops".to_string())));
        errs.add_warning(make_err(ParseErrorKind::Other("meh".to_string())));
        assert!(errs.has_errors());
        assert!(errs.has_warnings());
        assert_eq!(errs.len(), 2);
        assert_eq!(errs.errors().count(), 1);
        assert_eq!(errs.warnings().count(), 1);
    }
    #[test]
    fn test_parse_errors_empty() {
        let errs = ParseErrors::new();
        assert!(errs.is_empty());
        assert!(!errs.has_errors());
        assert!(errs.first_error().is_none());
    }
    #[test]
    fn test_format_expected() {
        assert_eq!(format_expected(&[]), "something");
        assert_eq!(format_expected(&["expr".to_string()]), "expr");
        assert_eq!(
            format_expected(&["a".to_string(), "b".to_string()]),
            "a or b"
        );
        let three = format_expected(&["a".to_string(), "b".to_string(), "c".to_string()]);
        assert!(three.contains("a"));
        assert!(three.contains("or c"));
    }
    #[test]
    fn test_suggest_correction() {
        assert_eq!(suggest_correction("thm"), Some("theorem"));
        assert_eq!(suggest_correction("def"), Some("definition"));
        assert!(suggest_correction("totally_unknown").is_none());
    }
    #[test]
    fn test_truncate_source() {
        let s = "hello world this is a long string";
        let t = truncate_source(s, 10);
        assert!(t.len() <= 15);
        assert!(t.contains('…'));
        assert_eq!(truncate_source("short", 20), "short");
    }
    #[test]
    fn test_underline_span() {
        let u = underline_span("hello world", 1, 5);
        assert!(u.starts_with("^"));
    }
    #[test]
    fn test_recovery_hints_eof() {
        let err = ParseError::unexpected_eof(vec![], Span::new(0, 0, 1, 1));
        let hints = recovery_hints(&err);
        assert!(!hints.is_empty());
    }
    #[test]
    fn test_recovery_hints_arrow() {
        let err = make_err(ParseErrorKind::InvalidSyntax("use => not ->".to_string()));
        let hints = recovery_hints(&err);
        assert!(!hints.is_empty());
    }
    #[test]
    fn test_parse_errors_summary() {
        let mut errs = ParseErrors::new();
        errs.add_error(make_err(ParseErrorKind::Other("e".to_string())));
        let s = errs.summary();
        assert!(s.contains("1 error(s)"));
        assert!(s.contains("0 warning(s)"));
    }
    #[test]
    fn test_diagnostic_display() {
        let err = make_err(ParseErrorKind::Other("bad".to_string()));
        let diag = Diagnostic::error(err);
        let s = format!("{}", diag);
        assert!(s.contains("error"));
    }
}
/// Return the 1-indexed line number from a `ParseError`.
pub fn error_line(err: &ParseError) -> usize {
    err.span.line
}
/// Return the 1-indexed column from a `ParseError`.
pub fn error_col(err: &ParseError) -> usize {
    err.span.column
}
/// Return the byte start offset from a `ParseError`.
pub fn error_start(err: &ParseError) -> usize {
    err.span.start
}
/// Return the byte end offset from a `ParseError`.
pub fn error_end(err: &ParseError) -> usize {
    err.span.end
}
/// Extract the source text covered by the error span.
pub fn error_source_text<'a>(err: &ParseError, source: &'a str) -> &'a str {
    source.get(err.span.start..err.span.end).unwrap_or("")
}
/// Convenience: create an `InvalidSyntax` error at the given location.
pub fn syntax_error(msg: impl Into<String>, span: Span) -> ParseError {
    ParseError::new(ParseErrorKind::InvalidSyntax(msg.into()), span)
}
/// Convenience: create a `DuplicateDeclaration` error.
pub fn duplicate_error(name: impl Into<String>, span: Span) -> ParseError {
    ParseError::new(ParseErrorKind::DuplicateDeclaration(name.into()), span)
}
/// Convenience: create an `InvalidBinder` error.
pub fn binder_error(msg: impl Into<String>, span: Span) -> ParseError {
    ParseError::new(ParseErrorKind::InvalidBinder(msg.into()), span)
}
/// Convenience: create an `InvalidPattern` error.
pub fn pattern_error(msg: impl Into<String>, span: Span) -> ParseError {
    ParseError::new(ParseErrorKind::InvalidPattern(msg.into()), span)
}
/// Convenience: create an `InvalidUniverse` error.
pub fn universe_error(msg: impl Into<String>, span: Span) -> ParseError {
    ParseError::new(ParseErrorKind::InvalidUniverse(msg.into()), span)
}
/// Convenience: wrap any string into an `Other` parse error.
pub fn other_error(msg: impl Into<String>, span: Span) -> ParseError {
    ParseError::new(ParseErrorKind::Other(msg.into()), span)
}
/// Produce a compact JSON-compatible representation of a `ParseError`.
pub fn error_to_json(err: &ParseError) -> String {
    format!(
        r#"{{"kind":"{}","line":{},"col":{},"start":{},"end":{},"message":"{}"}}"#,
        error_kind_name(&err.kind),
        err.span.line,
        err.span.column,
        err.span.start,
        err.span.end,
        err.message().replace('"', "\\\"")
    )
}
/// Return a short category name for a `ParseErrorKind`.
pub fn error_kind_name(kind: &ParseErrorKind) -> &'static str {
    match kind {
        ParseErrorKind::UnexpectedToken { .. } => "unexpected_token",
        ParseErrorKind::UnexpectedEof { .. } => "unexpected_eof",
        ParseErrorKind::InvalidSyntax(_) => "invalid_syntax",
        ParseErrorKind::DuplicateDeclaration(_) => "duplicate_declaration",
        ParseErrorKind::InvalidBinder(_) => "invalid_binder",
        ParseErrorKind::InvalidPattern(_) => "invalid_pattern",
        ParseErrorKind::InvalidUniverse(_) => "invalid_universe",
        ParseErrorKind::Other(_) => "other",
    }
}
#[cfg(test)]
mod error_extra_tests2 {
    use super::*;
    use crate::error_impl::*;
    use crate::tokens::TokenKind;
    #[test]
    fn test_error_location_helpers() {
        let err = ParseError::new(
            ParseErrorKind::Other("test".to_string()),
            Span::new(10, 20, 5, 3),
        );
        assert_eq!(error_line(&err), 5);
        assert_eq!(error_col(&err), 3);
        assert_eq!(error_start(&err), 10);
        assert_eq!(error_end(&err), 20);
    }
    #[test]
    fn test_error_source_text() {
        let source = "hello world";
        let err = ParseError::new(
            ParseErrorKind::Other("t".to_string()),
            Span::new(6, 11, 1, 7),
        );
        assert_eq!(error_source_text(&err, source), "world");
    }
    #[test]
    fn test_parse_error_builder() {
        let err = ParseErrorBuilder::new()
            .kind(ParseErrorKind::Other("oops".to_string()))
            .at(Span::new(5, 10, 2, 3))
            .build();
        assert_eq!(err.span.line, 2);
        assert_eq!(err.span.column, 3);
        assert!(matches!(err.kind, ParseErrorKind::Other(_)));
    }
    #[test]
    fn test_convenience_constructors() {
        let span = Span::new(0, 5, 1, 1);
        let se = syntax_error("bad syntax", span.clone());
        assert!(matches!(se.kind, ParseErrorKind::InvalidSyntax(_)));
        let de = duplicate_error("foo", span.clone());
        assert!(matches!(de.kind, ParseErrorKind::DuplicateDeclaration(_)));
        let be = binder_error("bad binder", span.clone());
        assert!(matches!(be.kind, ParseErrorKind::InvalidBinder(_)));
        let pe = pattern_error("bad pattern", span.clone());
        assert!(matches!(pe.kind, ParseErrorKind::InvalidPattern(_)));
        let ue = universe_error("bad universe", span.clone());
        assert!(matches!(ue.kind, ParseErrorKind::InvalidUniverse(_)));
        let oe = other_error("other", span);
        assert!(matches!(oe.kind, ParseErrorKind::Other(_)));
    }
    #[test]
    fn test_error_kind_name() {
        assert_eq!(
            error_kind_name(&ParseErrorKind::Other("x".to_string())),
            "other"
        );
        assert_eq!(
            error_kind_name(&ParseErrorKind::UnexpectedEof { expected: vec![] }),
            "unexpected_eof"
        );
    }
    #[test]
    fn test_error_to_json() {
        let err = syntax_error("oops", Span::new(0, 5, 1, 1));
        let json = error_to_json(&err);
        assert!(json.contains("\"kind\""));
        assert!(json.contains("\"line\""));
        assert!(json.contains("invalid_syntax"));
    }
    #[test]
    fn test_parse_errors_first_error() {
        let mut errs = ParseErrors::new();
        let err = syntax_error("first", Span::new(0, 1, 1, 1));
        errs.add_error(err.clone());
        assert!(errs.first_error().is_some());
        assert_eq!(
            errs.first_error()
                .expect("test operation should succeed")
                .message(),
            err.message()
        );
    }
}
/// Return `true` if the error kind is a duplicate declaration.
#[allow(dead_code)]
pub fn is_duplicate_error(err: &ParseError) -> bool {
    matches!(err.kind, ParseErrorKind::DuplicateDeclaration(_))
}
/// Return `true` if the error kind is an invalid syntax error.
#[allow(dead_code)]
pub fn is_syntax_error(err: &ParseError) -> bool {
    matches!(err.kind, ParseErrorKind::InvalidSyntax(_))
}
/// Classify a `ParseError` into a human-readable category string.
#[allow(dead_code)]
pub fn classify_error(err: &ParseError) -> &'static str {
    match &err.kind {
        ParseErrorKind::UnexpectedToken { .. } => "syntax",
        ParseErrorKind::UnexpectedEof { .. } => "eof",
        ParseErrorKind::InvalidSyntax(_) => "syntax",
        ParseErrorKind::DuplicateDeclaration(_) => "semantic",
        ParseErrorKind::InvalidBinder(_) => "binder",
        ParseErrorKind::InvalidPattern(_) => "pattern",
        ParseErrorKind::InvalidUniverse(_) => "universe",
        ParseErrorKind::Other(_) => "other",
    }
}
/// Return `true` if the error is likely recoverable (parser can continue).
///
/// EOF errors and certain syntax errors are generally unrecoverable,
/// while duplicate declaration errors are recoverable.
#[allow(dead_code)]
pub fn is_recoverable(err: &ParseError) -> bool {
    match &err.kind {
        ParseErrorKind::UnexpectedEof { .. } => false,
        ParseErrorKind::DuplicateDeclaration(_) => true,
        ParseErrorKind::InvalidBinder(_) => false,
        _ => true,
    }
}
/// Return the length (in bytes) of the span.
#[allow(dead_code)]
pub fn span_len(err: &ParseError) -> usize {
    err.span.end.saturating_sub(err.span.start)
}
/// Check if an error's span is empty (zero-length).
#[allow(dead_code)]
pub fn span_is_empty(err: &ParseError) -> bool {
    span_len(err) == 0
}
/// Expand the span of an error by `n` bytes on both sides (clamped to source bounds).
#[allow(dead_code)]
pub fn expand_span(err: &ParseError, source_len: usize, n: usize) -> (usize, usize) {
    let start = err.span.start.saturating_sub(n);
    let end = (err.span.end + n).min(source_len);
    (start, end)
}
/// Render a collection of diagnostics as a multi-line string.
#[allow(dead_code)]
pub fn render_diagnostics(diags: &ParseErrors, source: &str) -> String {
    let mut out = String::new();
    for diag in diags.iter() {
        out.push_str(&diag.report(source));
        out.push('\n');
    }
    out
}
/// Return the count of each kind in a `ParseErrors` collection.
#[allow(dead_code)]
pub fn count_by_kind(errs: &ParseErrors) -> std::collections::HashMap<&'static str, usize> {
    let mut counts = std::collections::HashMap::new();
    for diag in errs.iter() {
        let category = classify_error(&diag.error);
        *counts.entry(category).or_insert(0) += 1;
    }
    counts
}
/// Deduplicate errors by span start position.
///
/// If two errors start at the same position, only the first is kept.
#[allow(dead_code)]
pub fn dedup_errors(errs: &[ParseError]) -> Vec<ParseError> {
    let mut seen = std::collections::HashSet::new();
    errs.iter()
        .filter(|e| seen.insert(e.span.start))
        .cloned()
        .collect()
}
/// Sort errors by their span start position.
#[allow(dead_code)]
pub fn sort_errors(errs: &mut [ParseError]) {
    errs.sort_by_key(|e| e.span.start);
}
/// Return the error with the earliest position.
#[allow(dead_code)]
pub fn earliest_error(errs: &[ParseError]) -> Option<&ParseError> {
    errs.iter().min_by_key(|e| e.span.start)
}
/// Return the error with the latest position.
#[allow(dead_code)]
pub fn latest_error(errs: &[ParseError]) -> Option<&ParseError> {
    errs.iter().max_by_key(|e| e.span.start)
}
/// Export all errors in a `ParseErrors` as a JSON array string.
#[allow(dead_code)]
pub fn errors_to_json(errs: &ParseErrors) -> String {
    let entries: Vec<String> = errs.errors().map(|d| error_to_json(&d.error)).collect();
    format!("[{}]", entries.join(","))
}
#[cfg(test)]
mod error_final_tests {
    use super::*;
    use crate::error_impl::*;
    use crate::tokens::TokenKind;
    fn make_err_at(start: usize, end: usize) -> ParseError {
        ParseError::new(
            ParseErrorKind::Other("test".to_string()),
            Span::new(start, end, 1, 1),
        )
    }
    #[test]
    fn test_classify_error() {
        assert_eq!(
            classify_error(&syntax_error("bad", Span::new(0, 1, 1, 1))),
            "syntax"
        );
        assert_eq!(
            classify_error(&duplicate_error("foo", Span::new(0, 1, 1, 1))),
            "semantic"
        );
        assert_eq!(
            classify_error(&binder_error("bad binder", Span::new(0, 1, 1, 1))),
            "binder"
        );
        assert_eq!(
            classify_error(&pattern_error("bad pat", Span::new(0, 1, 1, 1))),
            "pattern"
        );
        assert_eq!(
            classify_error(&universe_error("bad univ", Span::new(0, 1, 1, 1))),
            "universe"
        );
        assert_eq!(
            classify_error(&other_error("other", Span::new(0, 1, 1, 1))),
            "other"
        );
    }
    #[test]
    fn test_is_recoverable() {
        let eof = ParseError::unexpected_eof(vec![], Span::new(0, 0, 1, 1));
        assert!(!is_recoverable(&eof));
        let dup = duplicate_error("foo", Span::new(0, 1, 1, 1));
        assert!(is_recoverable(&dup));
        let syn = syntax_error("bad", Span::new(0, 1, 1, 1));
        assert!(is_recoverable(&syn));
    }
    #[test]
    fn test_span_len() {
        let err = make_err_at(5, 10);
        assert_eq!(span_len(&err), 5);
    }
    #[test]
    fn test_span_is_empty() {
        let err = make_err_at(5, 5);
        assert!(span_is_empty(&err));
        let err2 = make_err_at(5, 6);
        assert!(!span_is_empty(&err2));
    }
    #[test]
    fn test_expand_span() {
        let err = make_err_at(5, 10);
        let (start, end) = expand_span(&err, 100, 2);
        assert_eq!(start, 3);
        assert_eq!(end, 12);
    }
    #[test]
    fn test_expand_span_clamp() {
        let err = make_err_at(1, 2);
        let (start, end) = expand_span(&err, 5, 10);
        assert_eq!(start, 0);
        assert_eq!(end, 5);
    }
    #[test]
    fn test_dedup_errors() {
        let e1 = make_err_at(5, 10);
        let e2 = make_err_at(5, 12);
        let e3 = make_err_at(15, 20);
        let errs = vec![e1, e2, e3];
        let deduped = dedup_errors(&errs);
        assert_eq!(deduped.len(), 2);
    }
    #[test]
    fn test_sort_errors() {
        let mut errs = vec![make_err_at(10, 15), make_err_at(3, 5), make_err_at(7, 9)];
        sort_errors(&mut errs);
        assert_eq!(errs[0].span.start, 3);
        assert_eq!(errs[1].span.start, 7);
        assert_eq!(errs[2].span.start, 10);
    }
    #[test]
    fn test_earliest_and_latest_error() {
        let errs = vec![make_err_at(10, 15), make_err_at(3, 5), make_err_at(7, 9)];
        assert_eq!(
            earliest_error(&errs)
                .expect("span should be present")
                .span
                .start,
            3
        );
        assert_eq!(
            latest_error(&errs)
                .expect("span should be present")
                .span
                .start,
            10
        );
    }
    #[test]
    fn test_count_by_kind() {
        let mut errs = ParseErrors::new();
        errs.add_error(syntax_error("x", Span::new(0, 1, 1, 1)));
        errs.add_error(syntax_error("y", Span::new(1, 2, 1, 2)));
        errs.add_error(binder_error("z", Span::new(2, 3, 1, 3)));
        let counts = count_by_kind(&errs);
        assert_eq!(counts.get("syntax"), Some(&2));
        assert_eq!(counts.get("binder"), Some(&1));
    }
    #[test]
    fn test_errors_to_json() {
        let mut errs = ParseErrors::new();
        errs.add_error(syntax_error("bad", Span::new(0, 3, 1, 1)));
        let json = errors_to_json(&errs);
        assert!(json.starts_with('['));
        assert!(json.ends_with(']'));
        assert!(json.contains("invalid_syntax"));
    }
    #[test]
    fn test_render_diagnostics() {
        let mut errs = ParseErrors::new();
        errs.add_error(syntax_error("bad", Span::new(0, 3, 1, 1)));
        let rendered = render_diagnostics(&errs, "bad code here");
        assert!(!rendered.is_empty());
    }
    #[test]
    fn test_is_duplicate_error_fn() {
        let err = duplicate_error("foo", Span::new(0, 1, 1, 1));
        assert!(is_duplicate_error(&err));
        let err2 = syntax_error("oops", Span::new(0, 1, 1, 1));
        assert!(!is_duplicate_error(&err2));
    }
    #[test]
    fn test_is_syntax_error_fn() {
        let err = syntax_error("oops", Span::new(0, 1, 1, 1));
        assert!(is_syntax_error(&err));
        let err2 = other_error("other", Span::new(0, 1, 1, 1));
        assert!(!is_syntax_error(&err2));
    }
}
/// Parse a source location from a "line:col" string.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn parse_location(s: &str) -> Option<(usize, usize)> {
    let mut parts = s.splitn(2, ':');
    let line = parts.next()?.parse::<usize>().ok()?;
    let col = parts.next()?.parse::<usize>().ok()?;
    Some((line, col))
}
/// Formats a sequence of errors in GNU style (file:line:col: message).
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn format_gnu_errors(filename: &str, errors: &[LocatedError]) -> String {
    errors
        .iter()
        .map(|e| format!("{}:{}:{}: error: {}", filename, e.line, e.col, e.message))
        .collect::<Vec<_>>()
        .join("\n")
}
/// Computes a fingerprint for an error message (for deduplication).
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn error_fingerprint(msg: &str) -> u64 {
    let mut hash = 14695981039346656037u64;
    for b in msg.bytes() {
        hash ^= b as u64;
        hash = hash.wrapping_mul(1099511628211u64);
    }
    hash
}
/// Deduplicates a list of errors by message fingerprint.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn dedup_by_message(errors: Vec<LocatedError>) -> Vec<LocatedError> {
    let mut seen = std::collections::HashSet::new();
    errors
        .into_iter()
        .filter(|e| seen.insert(error_fingerprint(&e.message)))
        .collect()
}
/// Format source text with annotated spans underlined.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn format_annotated_source(src: &str, spans: &[AnnotatedSpan]) -> String {
    let mut out = String::from(src);
    out.push('\n');
    for span in spans {
        let start = span.start.min(src.len());
        let end = span.end.min(src.len());
        let len = end.saturating_sub(start);
        out.push_str(&" ".repeat(start));
        out.push_str(&"^".repeat(len.max(1)));
        out.push(' ');
        out.push_str(&span.label);
        out.push('\n');
    }
    out
}
/// Extract context lines around a given line number from source.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn extract_context(src: &str, line_no: usize, radius: usize) -> (Vec<String>, usize) {
    let lines: Vec<&str> = src.lines().collect();
    let idx = line_no.saturating_sub(1);
    let start = idx.saturating_sub(radius);
    let end = (idx + radius + 1).min(lines.len());
    let context: Vec<String> = lines[start..end].iter().map(|s| s.to_string()).collect();
    let error_idx = idx - start;
    (context, error_idx)
}
/// Counts the number of errors at each severity level.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn count_by_severity(
    diagnostics: &[FullDiagnostic],
) -> std::collections::HashMap<String, usize> {
    let mut counts = std::collections::HashMap::new();
    for d in diagnostics {
        *counts.entry(d.severity.to_string()).or_insert(0) += 1;
    }
    counts
}
/// Formats a compact error summary line.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn compact_error_summary(diagnostics: &[FullDiagnostic]) -> String {
    let errors = diagnostics
        .iter()
        .filter(|d| d.severity >= DiagnosticSeverity::Error)
        .count();
    let warnings = diagnostics
        .iter()
        .filter(|d| d.severity == DiagnosticSeverity::Warning)
        .count();
    format!("{} error(s), {} warning(s)", errors, warnings)
}
/// Returns the line and column for a byte offset in source.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn byte_offset_to_line_col(src: &str, offset: usize) -> (usize, usize) {
    let mut line = 1usize;
    let mut col = 1usize;
    for (i, c) in src.char_indices() {
        if i >= offset {
            break;
        }
        if c == '\n' {
            line += 1;
            col = 1;
        } else {
            col += 1;
        }
    }
    (line, col)
}
#[cfg(test)]
mod error_impl_ext_tests {
    use super::*;
    use crate::error_impl::*;
    use crate::tokens::TokenKind;
    #[test]
    fn test_located_error_format() {
        let err = LocatedError::new("unexpected token", 0, 5, 1, 3);
        assert_eq!(err.format(), "1:3: unexpected token");
    }
    #[test]
    fn test_error_sink() {
        let mut sink = ErrorSink::new();
        sink.push(LocatedError::new("err", 0, 1, 1, 1));
        assert!(sink.has_errors());
        assert_eq!(sink.len(), 1);
        sink.clear();
        assert!(sink.is_empty());
    }
    #[test]
    fn test_error_code_format() {
        let code = ErrorCode::new("E", 42);
        assert_eq!(code.format(), "E0042");
    }
    #[test]
    fn test_full_diagnostic_display() {
        let diag = FullDiagnostic::error("something went wrong").with_note("check the input");
        let s = diag.display();
        assert!(s.contains("error"));
        assert!(s.contains("something went wrong"));
        assert!(s.contains("check the input"));
    }
    #[test]
    fn test_diagnostic_bag() {
        let mut bag = DiagnosticBag::new();
        bag.add(FullDiagnostic::error("oops"));
        bag.add(FullDiagnostic::warning("careful"));
        assert!(bag.has_errors());
        assert_eq!(bag.errors().len(), 1);
        assert_eq!(bag.warnings().len(), 1);
    }
    #[test]
    fn test_parse_location() {
        assert_eq!(parse_location("10:5"), Some((10, 5)));
        assert_eq!(parse_location("bad"), None);
    }
    #[test]
    fn test_format_gnu_errors() {
        let errs = vec![LocatedError::new("unexpected", 0, 1, 2, 3)];
        let s = format_gnu_errors("main.lean", &errs);
        assert!(s.contains("main.lean:2:3: error:"));
    }
    #[test]
    fn test_dedup_by_message() {
        let errs = vec![
            LocatedError::new("duplicate", 0, 1, 1, 1),
            LocatedError::new("duplicate", 0, 1, 2, 1),
            LocatedError::new("unique", 0, 1, 3, 1),
        ];
        let deduped = dedup_by_message(errs);
        assert_eq!(deduped.len(), 2);
    }
    #[test]
    fn test_byte_offset_to_line_col() {
        let src = "hello\nworld\n";
        let (line, _col) = byte_offset_to_line_col(src, 6);
        assert_eq!(line, 2);
    }
    #[test]
    fn test_error_rate_limiter() {
        let mut limiter = ErrorRateLimiter::new(3);
        assert!(limiter.accept());
        assert!(limiter.accept());
        assert!(limiter.accept());
        assert!(!limiter.accept());
        assert!(limiter.exceeded);
    }
    #[test]
    fn test_format_annotated_source() {
        let src = "fun x -> y";
        let spans = vec![AnnotatedSpan::new(4, 5, "here")];
        let out = format_annotated_source(src, &spans);
        assert!(out.contains("fun x -> y"));
        assert!(out.contains("here"));
    }
    #[test]
    fn test_extract_context() {
        let src = "line1\nline2\nline3\nline4\nline5";
        let (ctx, idx) = extract_context(src, 3, 1);
        assert!(ctx.len() <= 3);
        assert!(idx < ctx.len());
    }
    #[test]
    fn test_error_message_filter() {
        let filter = ErrorMessageFilter::new().suppress("internal");
        let errs = vec![
            LocatedError::new("internal error", 0, 1, 1, 1),
            LocatedError::new("parse error", 0, 1, 2, 1),
        ];
        let shown = filter.filter(&errs);
        assert_eq!(shown.len(), 1);
        assert!(shown[0].message.contains("parse"));
    }
    #[test]
    fn test_compact_error_summary() {
        let diags = vec![
            FullDiagnostic::error("e1"),
            FullDiagnostic::error("e2"),
            FullDiagnostic::warning("w1"),
        ];
        let s = compact_error_summary(&diags);
        assert!(s.contains("2 error"));
        assert!(s.contains("1 warning"));
    }
}
/// Formats an error range as a caret string (e.g., "   ^^^").
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn format_caret_range(col: usize, len: usize) -> String {
    format!(
        "{}{}",
        " ".repeat(col.saturating_sub(1)),
        "^".repeat(len.max(1))
    )
}
#[cfg(test)]
mod error_impl_ext2_tests {
    use super::*;
    use crate::error_impl::*;
    use crate::tokens::TokenKind;
    #[test]
    fn test_lint_warning() {
        let w = LintWarning::new("unused-variable", "variable x is unused")
            .with_suggestion("prefix with _")
            .at_range(5, 6);
        assert_eq!(w.code, "unused-variable");
        assert!(w.suggestion.is_some());
        assert_eq!(w.start, 5);
    }
    #[test]
    fn test_lint_report() {
        let mut report = LintReport::new();
        report.add(LintWarning::new("code1", "msg1"));
        report.add(LintWarning::new("code2", "msg2"));
        assert_eq!(report.len(), 2);
        assert_eq!(report.by_code("code1").len(), 1);
        let out = report.format_all();
        assert!(out.contains("[code1]"));
    }
    #[test]
    fn test_error_with_fix_apply() {
        let e = ErrorWithFix::new("replace x", 4, 5, "y");
        let result = e.apply("fun x -> x");
        assert!(result.contains('y'));
    }
    #[test]
    fn test_multi_file_errors() {
        let mut mfe = MultiFileErrors::new();
        mfe.add("a.lean", LocatedError::new("err1", 0, 1, 1, 1));
        mfe.add("b.lean", LocatedError::new("err2", 0, 1, 2, 1));
        assert_eq!(mfe.total(), 2);
        assert_eq!(mfe.get("a.lean").len(), 1);
    }
    #[test]
    fn test_error_range() {
        let r1 = ErrorRange::new(0, 5);
        let r2 = ErrorRange::new(3, 8);
        assert!(r1.overlaps(&r2));
        let r3 = ErrorRange::new(5, 10);
        assert!(!r1.overlaps(&r3));
        assert_eq!(r1.len(), 5);
    }
    #[test]
    fn test_format_caret_range() {
        let s = format_caret_range(3, 4);
        assert_eq!(s, "  ^^^^");
    }
}
/// Checks if two errors have the same message after normalisation.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn errors_have_same_message(a: &LocatedError, b: &LocatedError) -> bool {
    let norm_a: String = a.message.split_whitespace().collect::<Vec<_>>().join(" ");
    let norm_b: String = b.message.split_whitespace().collect::<Vec<_>>().join(" ");
    norm_a == norm_b
}
#[cfg(test)]
mod error_impl_ext3_tests {
    use super::*;
    use crate::error_impl::*;
    use crate::tokens::TokenKind;
    #[test]
    fn test_error_template() {
        let t = ErrorTemplate::new("expected {0} but found {1}");
        let msg = t.format(&["Ident", "Nat"]);
        assert_eq!(msg, "expected Ident but found Nat");
    }
    #[test]
    fn test_spanned_error_overlaps() {
        let err = SpannedError::new(100, "msg", 5, 10);
        assert!(err.overlaps(7, 15));
        assert!(!err.overlaps(0, 5));
        assert!(!err.overlaps(10, 20));
    }
    #[test]
    fn test_error_batch() {
        let mut batch = ErrorBatch::new();
        batch.add(SpannedError::new(100, "msg1", 0, 5));
        batch.add(SpannedError::new(100, "msg2", 6, 10));
        batch.add(SpannedError::new(200, "msg3", 0, 3));
        assert_eq!(batch.total(), 3);
        assert_eq!(batch.get(100).len(), 2);
        assert_eq!(batch.get(200).len(), 1);
    }
    #[test]
    fn test_recoverable_error() {
        let err = RecoverableError::new("unexpected token")
            .suggest("try adding a semicolon")
            .mark_recovered();
        assert!(err.recovered);
        assert_eq!(err.suggestions.len(), 1);
    }
    #[test]
    fn test_errors_have_same_message() {
        let e1 = LocatedError::new("foo   bar", 0, 1, 1, 1);
        let e2 = LocatedError::new("foo bar", 0, 1, 2, 1);
        assert!(errors_have_same_message(&e1, &e2));
    }
}
/// Partitions errors by line number.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn partition_by_line(
    errors: &[LocatedError],
) -> std::collections::HashMap<usize, Vec<&LocatedError>> {
    let mut map: std::collections::HashMap<usize, Vec<&LocatedError>> =
        std::collections::HashMap::new();
    for err in errors {
        map.entry(err.line).or_default().push(err);
    }
    map
}
#[cfg(test)]
mod error_window_tests {
    use super::*;
    use crate::error_impl::*;
    use crate::tokens::TokenKind;
    #[test]
    fn test_error_window() {
        let mut w = ErrorWindow::new(2);
        w.push(LocatedError::new("e1", 0, 1, 1, 1));
        w.push(LocatedError::new("e2", 0, 1, 2, 1));
        w.push(LocatedError::new("e3", 0, 1, 3, 1));
        assert!(w.truncated);
        assert_eq!(w.shown.len(), 2);
        assert!(w.summary().contains("more omitted"));
    }
    #[test]
    fn test_partition_by_line() {
        let errs = vec![
            LocatedError::new("e1", 0, 1, 1, 1),
            LocatedError::new("e2", 0, 1, 1, 2),
            LocatedError::new("e3", 0, 1, 2, 1),
        ];
        let partitioned = partition_by_line(&errs);
        assert_eq!(partitioned[&1].len(), 2);
        assert_eq!(partitioned[&2].len(), 1);
    }
}
/// Format a list of errors as a numbered list.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn numbered_error_list(errors: &[LocatedError]) -> String {
    errors
        .iter()
        .enumerate()
        .map(|(i, e)| format!("{}. {}", i + 1, e.format()))
        .collect::<Vec<_>>()
        .join("\n")
}
#[cfg(test)]
mod error_chain_tests {
    use super::*;
    use crate::error_impl::*;
    use crate::tokens::TokenKind;
    #[test]
    fn test_error_chain_ext() {
        let e = ErrorChainExt::new("while parsing def", "unexpected token");
        assert_eq!(e.format(), "while parsing def: unexpected token");
    }
    #[test]
    fn test_numbered_error_list() {
        let errs = vec![
            LocatedError::new("e1", 0, 1, 1, 1),
            LocatedError::new("e2", 0, 1, 2, 1),
        ];
        let out = numbered_error_list(&errs);
        assert!(out.contains("1."));
        assert!(out.contains("2."));
    }
}
/// Returns the total byte span covered by a list of errors.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn total_span(errors: &[LocatedError]) -> usize {
    errors.iter().map(|e| e.end.saturating_sub(e.start)).sum()
}
/// Returns the error with the widest span.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn widest_error(errors: &[LocatedError]) -> Option<&LocatedError> {
    errors.iter().max_by_key(|e| e.end.saturating_sub(e.start))
}
#[cfg(test)]
mod error_impl_pad {
    use super::*;
    use crate::error_impl::*;
    use crate::tokens::TokenKind;
    #[test]
    fn test_total_span() {
        let e = vec![
            LocatedError::new("a", 0, 5, 1, 1),
            LocatedError::new("b", 5, 10, 2, 1),
        ];
        assert_eq!(total_span(&e), 10);
    }
    #[test]
    fn test_widest_error() {
        let e = vec![
            LocatedError::new("a", 0, 3, 1, 1),
            LocatedError::new("b", 0, 10, 2, 1),
        ];
        assert_eq!(
            widest_error(&e).expect("test operation should succeed").end,
            10
        );
    }
}
/// Checks if an error message matches a substring.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn error_contains(e: &LocatedError, s: &str) -> bool {
    e.message.contains(s)
}
/// Returns the line number of a located error.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn error_line_ext2(e: &LocatedError) -> usize {
    e.line
}
/// Returns the column of a located error.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn error_col_ext2(e: &LocatedError) -> usize {
    e.col
}
#[cfg(test)]
mod error_impl_pad2 {
    use super::*;
    use crate::error_impl::*;
    use crate::tokens::TokenKind;
    #[test]
    fn test_error_contains() {
        let e = LocatedError::new("unexpected token", 0, 5, 1, 1);
        assert!(error_contains(&e, "unexpected"));
        assert!(!error_contains(&e, "missing"));
    }
}
