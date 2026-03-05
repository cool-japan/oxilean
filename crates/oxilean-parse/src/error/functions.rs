//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

pub use crate::error_impl::{ParseError, ParseErrorKind};
pub use crate::tokens::Span;

use super::types::{
    ErrorLocationResolver, ErrorSeverity, ErrorSeverityLevel, ParseDiagnostic, RichError,
};

/// Convenience alias for results that may fail with a `ParseError`.
#[allow(missing_docs)]
pub type ParseResult<T> = Result<T, ParseError>;
/// Extension trait providing factory methods on `ParseError`.
#[allow(clippy::new_ret_no_self)]
#[allow(missing_docs)]
pub trait ParseErrorFactory {
    /// Create a generic parse error with a message.
    fn new(msg: &str) -> ParseError;
    /// Create an error at a specific source location.
    fn at(msg: &str, line: u32, col: u32) -> ParseError;
    /// Create an "unexpected token" error.
    fn unexpected_token(found: &str, expected: &str, line: u32, col: u32) -> ParseError;
    /// Create an "unexpected end of file" error.
    fn unexpected_eof() -> ParseError;
    /// Create an "invalid syntax" error.
    fn invalid_syntax(msg: &str, line: u32, col: u32) -> ParseError;
    /// Create an "unterminated string literal" error.
    fn unterminated_string(line: u32, col: u32) -> ParseError;
    /// Create a "reserved keyword used as identifier" error.
    fn reserved_keyword(kw: &str, line: u32, col: u32) -> ParseError;
    /// Create a "duplicate binder name" error.
    fn duplicate_binder(name: &str, line: u32, col: u32) -> ParseError;
}
impl ParseErrorFactory for ParseError {
    fn new(msg: &str) -> ParseError {
        ParseError::new(
            ParseErrorKind::InvalidSyntax(msg.to_string()),
            Span::new(0, 0, 0, 0),
        )
    }
    fn at(msg: &str, line: u32, col: u32) -> ParseError {
        ParseError::new(
            ParseErrorKind::InvalidSyntax(msg.to_string()),
            Span::new(0, 0, line as usize, col as usize),
        )
    }
    fn unexpected_token(found: &str, expected: &str, _line: u32, _col: u32) -> ParseError {
        let msg = format!("unexpected token '{}', expected {}", found, expected);
        ParseError::new(ParseErrorKind::InvalidSyntax(msg), Span::new(0, 0, 0, 0))
    }
    fn unexpected_eof() -> ParseError {
        ParseError::new(
            ParseErrorKind::UnexpectedEof { expected: vec![] },
            Span::new(0, 0, 0, 0),
        )
    }
    fn invalid_syntax(msg: &str, _line: u32, _col: u32) -> ParseError {
        ParseError::new(
            ParseErrorKind::InvalidSyntax(format!("invalid syntax: {}", msg)),
            Span::new(0, 0, 0, 0),
        )
    }
    fn unterminated_string(_line: u32, _col: u32) -> ParseError {
        ParseError::new(
            ParseErrorKind::InvalidSyntax("unterminated string literal".to_string()),
            Span::new(0, 0, 0, 0),
        )
    }
    fn reserved_keyword(kw: &str, _line: u32, _col: u32) -> ParseError {
        let msg = format!(
            "'{}' is a reserved keyword and cannot be used as an identifier",
            kw
        );
        ParseError::new(ParseErrorKind::Other(msg), Span::new(0, 0, 0, 0))
    }
    fn duplicate_binder(name: &str, _line: u32, _col: u32) -> ParseError {
        let msg = format!("duplicate binder name '{}'", name);
        ParseError::new(ParseErrorKind::Other(msg), Span::new(0, 0, 0, 0))
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::*;
    fn mk_err(msg: &str) -> ParseError {
        ParseError::from_msg(msg, 1, 1)
    }
    #[test]
    fn test_factory_new() {
        let e = <ParseError as ParseErrorFactory>::new("test");
        assert!(!e.message().is_empty());
    }
    #[test]
    fn test_factory_at() {
        let e = <ParseError as ParseErrorFactory>::at("test", 3, 5);
        assert_eq!(e.line(), 3);
        assert_eq!(e.col(), 5);
    }
    #[test]
    fn test_factory_unexpected_token() {
        let e = <ParseError as ParseErrorFactory>::unexpected_token("foo", "bar", 1, 1);
        assert!(e.message().contains("foo"));
        assert!(e.message().contains("bar"));
    }
    #[test]
    fn test_factory_unexpected_eof() {
        let e = <ParseError as ParseErrorFactory>::unexpected_eof();
        assert!(matches!(e.kind, ParseErrorKind::UnexpectedEof { .. }));
    }
    #[test]
    fn test_factory_invalid_syntax() {
        let e = <ParseError as ParseErrorFactory>::invalid_syntax("missing `:=`", 2, 3);
        assert!(e.message().contains("invalid syntax"));
    }
    #[test]
    fn test_factory_unterminated_string() {
        let e = <ParseError as ParseErrorFactory>::unterminated_string(1, 10);
        assert!(e.message().contains("unterminated"));
    }
    #[test]
    fn test_factory_reserved_keyword() {
        let e = <ParseError as ParseErrorFactory>::reserved_keyword("def", 1, 1);
        assert!(e.message().contains("def"));
        assert!(e.message().contains("reserved"));
    }
    #[test]
    fn test_factory_duplicate_binder() {
        let e = <ParseError as ParseErrorFactory>::duplicate_binder("x", 5, 5);
        assert!(e.message().contains("x"));
        assert!(e.message().contains("duplicate"));
    }
    #[test]
    fn test_collector_add_and_len() {
        let mut c = ParseErrorCollector::new();
        c.add(mk_err("one"));
        c.add(mk_err("two"));
        assert_eq!(c.len(), 2);
        assert!(c.has_errors());
    }
    #[test]
    fn test_collector_limit() {
        let mut c = ParseErrorCollector::with_limit(2);
        c.add(mk_err("one"));
        c.add(mk_err("two"));
        c.add(mk_err("three"));
        assert_eq!(c.len(), 2);
        assert!(c.is_full());
    }
    #[test]
    fn test_collector_clear() {
        let mut c = ParseErrorCollector::new();
        c.add(mk_err("a"));
        c.clear();
        assert!(c.is_empty());
    }
    #[test]
    fn test_collector_first_error() {
        let mut c = ParseErrorCollector::new();
        assert!(c.first_error().is_none());
        c.add(mk_err("first"));
        c.add(mk_err("second"));
        assert!(c
            .first_error()
            .expect("test operation should succeed")
            .message()
            .contains("first"));
    }
    #[test]
    fn test_collector_merge() {
        let mut c1 = ParseErrorCollector::new();
        let mut c2 = ParseErrorCollector::new();
        c1.add(mk_err("a"));
        c2.add(mk_err("b"));
        c1.merge(c2);
        assert_eq!(c1.len(), 2);
    }
    #[test]
    fn test_collector_display() {
        let mut c = ParseErrorCollector::new();
        c.add(mk_err("x"));
        let s = format!("{}", c);
        assert!(s.contains("1 errors"));
    }
    #[test]
    fn test_recovery_strategy_continues() {
        assert!(!RecoveryStrategy::Abort.continues());
        assert!(RecoveryStrategy::SkipToSync.continues());
        assert!(RecoveryStrategy::InsertToken.continues());
        assert!(RecoveryStrategy::Replace.continues());
    }
    #[test]
    fn test_recovery_strategy_display() {
        assert_eq!(format!("{}", RecoveryStrategy::Abort), "abort");
        assert_eq!(format!("{}", RecoveryStrategy::SkipToSync), "skip-to-sync");
    }
    #[test]
    fn test_error_severity_ordering() {
        assert!(ErrorSeverity::Error > ErrorSeverity::Warning);
        assert!(ErrorSeverity::Warning > ErrorSeverity::Note);
    }
    #[test]
    fn test_error_severity_is_error() {
        assert!(ErrorSeverity::Error.is_error());
        assert!(!ErrorSeverity::Warning.is_error());
    }
    #[test]
    fn test_error_severity_is_recoverable() {
        assert!(ErrorSeverity::Warning.is_recoverable());
        assert!(ErrorSeverity::Note.is_recoverable());
        assert!(!ErrorSeverity::Error.is_recoverable());
    }
    #[test]
    fn test_error_severity_display() {
        assert_eq!(format!("{}", ErrorSeverity::Error), "error");
        assert_eq!(format!("{}", ErrorSeverity::Warning), "warning");
        assert_eq!(format!("{}", ErrorSeverity::Note), "note");
    }
    #[test]
    fn test_parse_diagnostic_error() {
        let d = ParseDiagnostic::error("foo.ox", 3, 5, "something went wrong");
        assert!(d.is_error());
        assert_eq!(d.line, 3);
    }
    #[test]
    fn test_parse_diagnostic_warning() {
        let d = ParseDiagnostic::warning("foo.ox", 1, 1, "unused import");
        assert!(!d.is_error());
    }
    #[test]
    fn test_parse_diagnostic_with_hint() {
        let d = ParseDiagnostic::error("foo.ox", 1, 1, "oops").with_hint("try this");
        assert_eq!(d.hint.as_deref(), Some("try this"));
    }
    #[test]
    fn test_parse_diagnostic_with_code() {
        let d = ParseDiagnostic::error("foo.ox", 1, 1, "oops").with_code("def foo := 1");
        assert!(d.code.is_some());
    }
    #[test]
    fn test_parse_diagnostic_display() {
        let d = ParseDiagnostic::error("foo.ox", 2, 4, "msg");
        let s = format!("{}", d);
        assert!(s.contains("foo.ox"));
        assert!(s.contains("2:4"));
        assert!(s.contains("msg"));
    }
    #[test]
    fn test_formatter_format() {
        let src = "line 1\nline 2 with error\nline 3\n";
        let fmt = ParseErrorFormatter::new(src, "test.ox");
        let err = ParseError::from_msg("test error", 2, 8);
        let s = fmt.format(&err);
        assert!(s.contains("test error"));
    }
    #[test]
    fn test_formatter_format_all() {
        let src = "def x := 1\n";
        let fmt = ParseErrorFormatter::new(src, "f.ox");
        let mut c = ParseErrorCollector::new();
        c.add(mk_err("e1"));
        c.add(mk_err("e2"));
        let s = fmt.format_all(&c);
        assert!(s.contains("e1"));
        assert!(s.contains("e2"));
    }
    #[test]
    fn test_parse_error_stats_record() {
        let mut s = ParseErrorStats::new();
        s.record(&ParseError::from_msg("eof", 0, 0));
        s.record(&ParseError::from_msg("loc", 1, 5));
        assert_eq!(s.total, 2);
        assert_eq!(s.eof_errors, 1);
        assert_eq!(s.located_errors, 1);
    }
    #[test]
    fn test_parse_error_stats_display() {
        let s = ParseErrorStats {
            total: 5,
            eof_errors: 1,
            located_errors: 4,
        };
        let txt = format!("{}", s);
        assert!(txt.contains("total: 5"));
    }
    #[test]
    fn test_parse_error_budget_consume() {
        let mut b = ParseErrorBudget::new(3);
        assert!(b.consume());
        assert!(b.consume());
        assert!(b.consume());
        assert!(!b.consume());
        assert!(b.is_exhausted());
    }
    #[test]
    fn test_parse_error_budget_consumed() {
        let mut b = ParseErrorBudget::new(5);
        b.consume();
        b.consume();
        assert_eq!(b.consumed(), 2);
    }
    #[test]
    fn test_parse_error_budget_reset() {
        let mut b = ParseErrorBudget::new(3);
        b.consume();
        b.reset();
        assert_eq!(b.remaining, 3);
        assert!(!b.is_exhausted());
    }
    #[test]
    fn test_parse_result_ok() {
        let r: ParseResult<i32> = Ok(42);
        assert_eq!(r, Ok(42));
    }
    #[test]
    fn test_parse_result_err() {
        let r: ParseResult<i32> = Err(ParseError::from_msg("oops", 1, 1));
        assert!(r.is_err());
    }
    #[test]
    fn test_collector_into_errors() {
        let mut c = ParseErrorCollector::new();
        c.add(mk_err("a"));
        let v = c.into_errors();
        assert_eq!(v.len(), 1);
    }
}
/// Return a short label for a `ParseErrorKind`.
#[allow(missing_docs)]
pub fn error_kind_label(kind: &ParseErrorKind) -> &'static str {
    match kind {
        ParseErrorKind::UnexpectedToken { .. } => "unexpected-token",
        ParseErrorKind::UnexpectedEof { .. } => "unexpected-eof",
        ParseErrorKind::InvalidSyntax(_) => "invalid-syntax",
        ParseErrorKind::DuplicateDeclaration(_) => "duplicate-declaration",
        ParseErrorKind::InvalidBinder(_) => "invalid-binder",
        ParseErrorKind::InvalidPattern(_) => "invalid-pattern",
        ParseErrorKind::InvalidUniverse(_) => "invalid-universe",
        ParseErrorKind::Other(_) => "other",
    }
}
#[cfg(test)]
mod extra_tests {
    use super::*;
    use crate::error::*;
    #[test]
    fn test_error_kind_label_eof() {
        assert_eq!(
            error_kind_label(&ParseErrorKind::UnexpectedEof { expected: vec![] }),
            "unexpected-eof"
        );
    }
    #[test]
    fn test_error_kind_label_token() {
        assert_eq!(
            error_kind_label(&ParseErrorKind::UnexpectedToken {
                expected: vec![],
                got: crate::tokens::TokenKind::Eof
            }),
            "unexpected-token"
        );
    }
    #[test]
    fn test_error_kind_label_syntax() {
        assert_eq!(
            error_kind_label(&ParseErrorKind::InvalidSyntax("".to_string())),
            "invalid-syntax"
        );
    }
    #[test]
    fn test_error_kind_label_other() {
        assert_eq!(
            error_kind_label(&ParseErrorKind::Other("".to_string())),
            "other"
        );
    }
    #[test]
    fn test_parse_warning_new() {
        let w = ParseWarning::new("unused import", 5, 3);
        assert_eq!(w.line, 5);
        assert_eq!(w.col, 3);
    }
    #[test]
    fn test_parse_warning_display() {
        let w = ParseWarning::new("test warning", 2, 4);
        let s = format!("{}", w);
        assert!(s.contains("warning"));
        assert!(s.contains("test warning"));
    }
    #[test]
    fn test_parse_error_group_new() {
        let g = ParseErrorGroup::new("syntax");
        assert_eq!(g.label, "syntax");
        assert!(g.is_empty());
    }
    #[test]
    fn test_parse_error_group_add() {
        let mut g = ParseErrorGroup::new("g");
        g.add(ParseError::from_msg("err", 1, 1));
        assert_eq!(g.len(), 1);
    }
    #[test]
    fn test_parse_error_group_display() {
        let mut g = ParseErrorGroup::new("syntax");
        g.add(ParseError::from_msg("e", 1, 1));
        let s = format!("{}", g);
        assert!(s.contains("syntax"));
        assert!(s.contains("1 errors"));
    }
    #[test]
    fn test_recovery_strategy_replace() {
        assert!(RecoveryStrategy::Replace.continues());
    }
    #[test]
    fn test_error_severity_note() {
        assert!(!ErrorSeverity::Note.is_error());
        assert!(ErrorSeverity::Note.is_recoverable());
    }
    #[test]
    fn test_parse_error_budget_initial_not_exhausted() {
        let b = ParseErrorBudget::new(10);
        assert!(!b.is_exhausted());
        assert_eq!(b.consumed(), 0);
    }
}
/// Filters a list of diagnostics by severity.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn filter_by_severity(
    diagnostics: &[ParseDiagnostic],
    min_severity: ErrorSeverity,
) -> Vec<&ParseDiagnostic> {
    diagnostics
        .iter()
        .filter(|d| d.severity >= min_severity)
        .collect()
}
/// Filters diagnostics to only include hard errors.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn errors_only(diagnostics: &[ParseDiagnostic]) -> Vec<&ParseDiagnostic> {
    filter_by_severity(diagnostics, ErrorSeverity::Error)
}
/// Filters diagnostics to only include warnings.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn warnings_only(diagnostics: &[ParseDiagnostic]) -> Vec<&ParseDiagnostic> {
    diagnostics
        .iter()
        .filter(|d| d.severity == ErrorSeverity::Warning)
        .collect()
}
#[cfg(test)]
mod error_report_tests {
    use super::*;
    use crate::error::*;
    fn mk_diag(sev: ErrorSeverity) -> ParseDiagnostic {
        ParseDiagnostic::new(sev, "test.ox", 1, 1, "msg")
    }
    #[test]
    fn test_filter_by_severity_error_only() {
        let diags = vec![
            mk_diag(ErrorSeverity::Error),
            mk_diag(ErrorSeverity::Warning),
            mk_diag(ErrorSeverity::Note),
        ];
        let errs = filter_by_severity(&diags, ErrorSeverity::Error);
        assert_eq!(errs.len(), 1);
    }
    #[test]
    fn test_filter_by_severity_warning_up() {
        let diags = vec![
            mk_diag(ErrorSeverity::Error),
            mk_diag(ErrorSeverity::Warning),
            mk_diag(ErrorSeverity::Note),
        ];
        let result = filter_by_severity(&diags, ErrorSeverity::Warning);
        assert_eq!(result.len(), 2);
    }
    #[test]
    fn test_errors_only() {
        let diags = vec![
            mk_diag(ErrorSeverity::Error),
            mk_diag(ErrorSeverity::Warning),
        ];
        let errs = errors_only(&diags);
        assert_eq!(errs.len(), 1);
    }
    #[test]
    fn test_warnings_only() {
        let diags = vec![
            mk_diag(ErrorSeverity::Error),
            mk_diag(ErrorSeverity::Warning),
            mk_diag(ErrorSeverity::Warning),
        ];
        let warns = warnings_only(&diags);
        assert_eq!(warns.len(), 2);
    }
    #[test]
    fn test_parse_error_context_new() {
        let err = ParseError::from_msg("oops", 1, 1);
        let ctx = ParseErrorContext::new(err);
        assert!(ctx.decl_name.is_none());
        assert!(ctx.phase.is_none());
    }
    #[test]
    fn test_parse_error_context_with_decl() {
        let err = ParseError::from_msg("oops", 1, 1);
        let ctx = ParseErrorContext::new(err).with_decl("foo");
        assert_eq!(ctx.decl_name.as_deref(), Some("foo"));
    }
    #[test]
    fn test_parse_error_context_with_phase() {
        let err = ParseError::from_msg("oops", 1, 1);
        let ctx = ParseErrorContext::new(err).with_phase("binder");
        assert_eq!(ctx.phase.as_deref(), Some("binder"));
    }
    #[test]
    fn test_parse_error_context_display() {
        let err = ParseError::from_msg("test", 1, 1);
        let ctx = ParseErrorContext::new(err)
            .with_decl("myDef")
            .with_phase("expr");
        let s = format!("{}", ctx);
        assert!(s.contains("myDef"));
        assert!(s.contains("expr"));
    }
    #[test]
    fn test_parse_error_report_new() {
        let r = ParseErrorReport::new("foo.ox");
        assert!(r.is_clean());
        assert_eq!(r.error_count(), 0);
    }
    #[test]
    fn test_parse_error_report_add_error() {
        let mut r = ParseErrorReport::new("foo.ox");
        r.add(mk_diag(ErrorSeverity::Error));
        assert_eq!(r.error_count(), 1);
        assert!(!r.is_clean());
    }
    #[test]
    fn test_parse_error_report_warnings() {
        let mut r = ParseErrorReport::new("foo.ox");
        r.add(mk_diag(ErrorSeverity::Warning));
        r.add(mk_diag(ErrorSeverity::Warning));
        assert_eq!(r.warning_count(), 2);
        assert!(r.is_clean());
    }
    #[test]
    fn test_parse_error_report_display() {
        let r = ParseErrorReport::new("test.ox");
        let s = format!("{}", r);
        assert!(s.contains("test.ox"));
    }
}
/// A "try" wrapper: runs a fallible operation, collecting any errors.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn try_collect<T, E: Clone>(results: Vec<Result<T, E>>) -> (Vec<T>, Vec<E>) {
    let mut oks = Vec::new();
    let mut errs = Vec::new();
    for r in results {
        match r {
            Ok(v) => oks.push(v),
            Err(e) => errs.push(e),
        }
    }
    (oks, errs)
}
/// Formats a caret pointer under a line at `col`.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn format_caret(col: usize, len: usize) -> String {
    format!("{}{}", " ".repeat(col), "^".repeat(len.max(1)))
}
/// Formats an error at a given source position.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn format_error_at(source: &str, byte_offset: usize, message: &str) -> String {
    let resolver = ErrorLocationResolver::new(source);
    let (line, col) = resolver.resolve(byte_offset);
    let line_text = resolver.line_text(line);
    let caret = format_caret(col, 1);
    format!(
        "{}\n{:4} | {}\n     | {}\n     {}",
        message,
        line + 1,
        line_text,
        caret,
        ""
    )
}
#[cfg(test)]
mod extended_error_tests {
    use super::*;
    use crate::error::*;
    #[test]
    fn test_rich_error_format() {
        let e = RichError::error("unexpected '+'", 10, 11)
            .with_code("E0001")
            .with_suggestion("remove the '+'")
            .with_note("operators must be binary");
        assert_eq!(e.span_len(), 1);
        let fmt = e.format();
        assert!(fmt.contains("[E0001]"));
        assert!(fmt.contains("suggestion: remove the '+'"));
        assert!(fmt.contains("note: operators must be binary"));
    }
    #[test]
    fn test_error_severity_ordering() {
        assert!(ErrorSeverityLevel::Fatal > ErrorSeverityLevel::Error);
        assert!(ErrorSeverityLevel::Error > ErrorSeverityLevel::Warning);
        assert!(ErrorSeverityLevel::Warning > ErrorSeverityLevel::Note);
    }
    #[test]
    fn test_error_accumulator2() {
        let mut acc = ErrorAccumulator2::new(10);
        acc.add(RichError::error("err1", 0, 1));
        acc.add(RichError::warning("warn1", 5, 6));
        assert_eq!(acc.error_count(), 1);
        assert_eq!(acc.warning_count(), 1);
        assert!(!acc.has_fatal());
        assert!(!acc.is_clean());
    }
    #[test]
    fn test_error_deduplicator() {
        let mut dedup = ErrorDeduplicator::new();
        assert!(dedup.should_emit("E0001 at 5"));
        assert!(!dedup.should_emit("E0001 at 5"));
        assert!(dedup.should_emit("E0002 at 10"));
        assert_eq!(dedup.suppressed_count(), 1);
        assert_eq!(dedup.unique_count(), 2);
    }
    #[test]
    fn test_error_filter() {
        let mut filter = ErrorFilter::new(ErrorSeverityLevel::Warning);
        filter.suppress_code("E0001");
        let e1 = RichError::error("err", 0, 1).with_code("E0001");
        let e2 = RichError::error("err2", 0, 1).with_code("E0002");
        let e3 = RichError::warning("warn", 0, 1);
        let errors = vec![e1, e2, e3];
        let shown = filter.filter(&errors);
        assert_eq!(shown.len(), 2);
    }
    #[test]
    fn test_error_location_resolver() {
        let src = "hello\nworld\nfoo";
        let resolver = ErrorLocationResolver::new(src);
        let (line, col) = resolver.resolve(7);
        assert_eq!(line, 1);
        assert_eq!(col, 1);
        assert_eq!(resolver.line_text(0), "hello");
        assert_eq!(resolver.line_text(1), "world");
        assert_eq!(resolver.line_count(), 3);
    }
    #[test]
    fn test_error_location_snippet() {
        let src = "line1\nline2\nline3\nline4\nline5";
        let resolver = ErrorLocationResolver::new(src);
        let snippet = resolver.snippet(6, 1);
        assert!(snippet.contains("line2"));
    }
    #[test]
    fn test_batch_error_report() {
        let errors = vec![RichError::error("e1", 0, 1), RichError::warning("w1", 2, 3)];
        let report = BatchErrorReport::new("foo.ox", errors, 1000);
        assert_eq!(report.error_count(), 1);
        assert_eq!(report.warning_count(), 1);
        assert!(!report.is_success());
        let summary = report.summary_line();
        assert!(summary.contains("foo.ox"));
        assert!(summary.contains("1 error(s)"));
    }
    #[test]
    fn test_error_code_catalogue() {
        let cat = ErrorCodeCatalogue::new();
        assert_eq!(cat.description("E0001"), Some("unexpected token"));
        assert_eq!(cat.description("E9999"), None);
        assert!(cat.count() >= 10);
    }
    #[test]
    fn test_try_collect() {
        let results: Vec<Result<i32, &str>> = vec![Ok(1), Err("e1"), Ok(2), Err("e2")];
        let (oks, errs) = try_collect(results);
        assert_eq!(oks, vec![1, 2]);
        assert_eq!(errs, vec!["e1", "e2"]);
    }
    #[test]
    fn test_format_caret() {
        assert_eq!(format_caret(3, 2), "   ^^");
        assert_eq!(format_caret(0, 1), "^");
    }
    #[test]
    fn test_format_error_at() {
        let src = "hello world";
        let msg = format_error_at(src, 6, "unexpected word");
        assert!(msg.contains("unexpected word"));
        assert!(msg.contains("hello world"));
    }
    #[test]
    fn test_rich_error_warning() {
        let w = RichError::warning("unused var", 0, 5);
        assert_eq!(w.severity, ErrorSeverityLevel::Warning);
        assert!(!w.is_fatal());
    }
    #[test]
    fn test_error_accumulator_max() {
        let mut acc = ErrorAccumulator2::new(2);
        acc.add(RichError::error("e1", 0, 1));
        acc.add(RichError::error("e2", 0, 1));
        let added = acc.add(RichError::error("e3", 0, 1));
        assert!(!added);
    }
}
/// An error formatter that outputs machine-readable JSON-like text.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn format_error_json(e: &RichError) -> String {
    let code = e.code.as_deref().unwrap_or("null");
    format!(
        r#"{{"severity":"{}", "code":"{}", "message":"{}", "span":[{},{}]}}"#,
        e.severity, code, e.message, e.span_start, e.span_end
    )
}
/// An error formatter that outputs UNIX-style error messages.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn format_error_unix(file: &str, line: usize, col: usize, e: &RichError) -> String {
    format!(
        "{}:{}:{}: {}: {}",
        file,
        line + 1,
        col + 1,
        e.severity,
        e.message
    )
}
/// Checks if a collection of errors is below a threshold.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn errors_within_budget(errors: &[RichError], max_errors: usize) -> bool {
    let error_count = errors
        .iter()
        .filter(|e| e.severity >= ErrorSeverityLevel::Error)
        .count();
    error_count <= max_errors
}
/// Sorts errors by severity (most severe first), then by position.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn sort_errors_by_severity(errors: &mut [RichError]) {
    errors.sort_by(|a, b| {
        b.severity
            .cmp(&a.severity)
            .then(a.span_start.cmp(&b.span_start))
    });
}
/// Deduplicates errors by (code, span_start) key.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn dedup_errors(errors: Vec<RichError>) -> Vec<RichError> {
    let mut seen = std::collections::HashSet::new();
    errors
        .into_iter()
        .filter(|e| {
            let key = (e.code.clone().unwrap_or_default(), e.span_start);
            seen.insert(key)
        })
        .collect()
}
/// Converts a vector of errors to a short summary string.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn error_summary(errors: &[RichError]) -> String {
    let errs = errors
        .iter()
        .filter(|e| e.severity >= ErrorSeverityLevel::Error)
        .count();
    let warns = errors
        .iter()
        .filter(|e| e.severity == ErrorSeverityLevel::Warning)
        .count();
    format!("{} error(s), {} warning(s)", errs, warns)
}
#[cfg(test)]
mod extended_error_tests_2 {
    use super::*;
    use crate::error::*;
    #[test]
    fn test_error_chain() {
        let root = RichError::error("root error", 0, 5);
        let cause = RichError::error("underlying cause", 0, 3);
        let chain = ErrorChain::new(root).caused_by(cause);
        assert_eq!(chain.len(), 2);
        let fmt = chain.format_chain();
        assert!(fmt.contains("root error"));
        assert!(fmt.contains("caused by: underlying cause"));
    }
    #[test]
    fn test_string_error_sink() {
        let mut sink = StringErrorSink::new();
        sink.emit(&RichError::error("e1", 0, 1));
        sink.emit(&RichError::warning("w1", 2, 3));
        assert_eq!(sink.count(), 2);
        assert!(sink.contents().contains("e1"));
        sink.clear();
        assert_eq!(sink.count(), 0);
    }
    #[test]
    fn test_error_budget() {
        let mut budget = ErrorBudget::new(3);
        assert!(budget.spend());
        assert!(budget.spend());
        assert!(budget.spend());
        assert!(!budget.spend());
        assert!(budget.is_exhausted());
        assert!((budget.fraction_used() - 1.0).abs() < 1e-9);
    }
    #[test]
    fn test_error_grouper() {
        let mut grouper = ErrorGrouper::new();
        grouper.add(RichError::error("e1", 0, 1).with_code("E0001"));
        grouper.add(RichError::error("e2", 0, 1).with_code("E0001"));
        grouper.add(RichError::error("e3", 0, 1).with_code("E0002"));
        assert_eq!(grouper.group_count(), 2);
        assert_eq!(grouper.errors_in_group("E0001").len(), 2);
        assert_eq!(grouper.most_common_code(), Some("E0001"));
        assert_eq!(grouper.total_error_count(), 3);
    }
    #[test]
    fn test_format_error_json() {
        let e = RichError::error("unexpected '+'", 5, 6).with_code("E0001");
        let json = format_error_json(&e);
        assert!(json.contains("\"severity\":\"error\""));
        assert!(json.contains("\"code\":\"E0001\""));
        assert!(json.contains("\"message\":\"unexpected '+'\""));
    }
    #[test]
    fn test_format_error_unix() {
        let e = RichError::error("bad token", 0, 1);
        let s = format_error_unix("foo.ox", 2, 5, &e);
        assert_eq!(s, "foo.ox:3:6: error: bad token");
    }
    #[test]
    fn test_errors_within_budget() {
        let errors = vec![RichError::error("e1", 0, 1), RichError::warning("w1", 0, 1)];
        assert!(errors_within_budget(&errors, 1));
        assert!(!errors_within_budget(&errors, 0));
    }
    #[test]
    fn test_sort_errors_by_severity() {
        let mut errors = vec![RichError::warning("w", 10, 11), RichError::error("e", 5, 6)];
        sort_errors_by_severity(&mut errors);
        assert_eq!(errors[0].severity, ErrorSeverityLevel::Error);
    }
    #[test]
    fn test_dedup_errors() {
        let errors = vec![
            RichError::error("e1", 5, 6).with_code("E0001"),
            RichError::error("e1 dup", 5, 6).with_code("E0001"),
            RichError::error("e2", 10, 11).with_code("E0002"),
        ];
        let deduped = dedup_errors(errors);
        assert_eq!(deduped.len(), 2);
    }
    #[test]
    fn test_error_summary() {
        let errors = vec![
            RichError::error("e1", 0, 1),
            RichError::error("e2", 0, 1),
            RichError::warning("w1", 0, 1),
        ];
        let s = error_summary(&errors);
        assert_eq!(s, "2 error(s), 1 warning(s)");
    }
    #[test]
    fn test_recovery_hint() {
        let h = RecoveryHint::insert_before(":=");
        assert!(h.description().contains("insert ':=' before"));
        let d = RecoveryHint::delete("+");
        assert!(d.description().contains("delete '+'"));
        let r = RecoveryHint::replace("->".to_string());
        assert!(r.description().contains("replace with '->'"));
    }
    #[test]
    fn test_tagged_error() {
        let e = RichError::error("syntax error", 0, 5);
        let te = TaggedError::new(e)
            .with_tag(ErrorTag::Syntax)
            .with_hint(RecoveryHint::insert_before(";"));
        assert!(te.has_tag(ErrorTag::Syntax));
        assert!(!te.has_tag(ErrorTag::Type));
        let fmt = te.format_full();
        assert!(fmt.contains("help: insert ';' before"));
    }
}
/// Writes a batch of errors to a formatted report string.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn write_error_report(errors: &[RichError], source_name: &str) -> String {
    let mut out = format!("=== Error Report: {} ===\n", source_name);
    out.push_str(&format!("{}\n", error_summary(errors)));
    out.push_str(&"─".repeat(50));
    out.push('\n');
    for (i, e) in errors.iter().enumerate() {
        out.push_str(&format!("[{}] {}\n", i + 1, e.format()));
    }
    out
}
/// Check if source contains common error patterns.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn detect_common_mistakes(source: &str) -> Vec<(&'static str, usize)> {
    let mut issues = Vec::new();
    for (i, line) in source.lines().enumerate() {
        if line.contains("->") && line.contains("=>") {
            issues.push(("mixed arrow styles", i));
        }
        if line.trim_start().starts_with("def") && !line.contains(":=") && !line.contains("where") {
            issues.push(("def without assignment", i));
        }
    }
    issues
}
/// Compute an "error density" metric: errors per 100 lines.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn error_density(errors: &[RichError], source_line_count: usize) -> f64 {
    if source_line_count == 0 {
        return 0.0;
    }
    let err_count = errors
        .iter()
        .filter(|e| e.severity >= ErrorSeverityLevel::Error)
        .count();
    err_count as f64 / source_line_count as f64 * 100.0
}
/// Format all errors as a table.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn format_error_table(errors: &[RichError]) -> String {
    let mut out = format!(
        "{:>4}  {:<10}  {:<8}  {}\n",
        "N", "Code", "Severity", "Message"
    );
    out.push_str(&"─".repeat(60));
    out.push('\n');
    for (i, e) in errors.iter().enumerate() {
        let code = e.code.as_deref().unwrap_or("-");
        out.push_str(&format!(
            "{:>4}  {:<10}  {:<8}  {}\n",
            i + 1,
            code,
            format!("{}", e.severity),
            e.message
        ));
    }
    out
}
#[cfg(test)]
mod extended_error_tests_3 {
    use super::*;
    use crate::error::*;
    #[test]
    fn test_error_rate_tracker() {
        let mut tracker = ErrorRateTracker::new(5);
        tracker.record(3);
        tracker.commit_window();
        tracker.record(1);
        tracker.commit_window();
        assert!((tracker.average() - 2.0).abs() < 1e-9);
        assert!((tracker.trend() - (-2.0)).abs() < 1e-9);
    }
    #[test]
    fn test_quick_fix_registry() {
        let mut reg = QuickFixRegistry::new();
        reg.register("E0001", "remove the unexpected token");
        reg.register("E0001", "wrap in parentheses");
        reg.register("E0002", "add missing '}'");
        assert_eq!(reg.fixes_for("E0001").len(), 2);
        assert!(reg.has_fixes("E0002"));
        assert!(!reg.has_fixes("E9999"));
        assert_eq!(reg.total_codes(), 2);
    }
    #[test]
    fn test_contextual_rich_error() {
        let src = "def foo := 1\ndef bar := bad\n";
        let e = RichError::error("bad identifier", 19, 22);
        let ce = ContextualRichError::new(e, src, "test.ox");
        assert_eq!(ce.file, "test.ox");
        let fmt = ce.format_full();
        assert!(fmt.contains("test.ox"));
        assert!(fmt.contains("bad identifier"));
    }
    #[test]
    fn test_write_error_report() {
        let errors = vec![RichError::error("e1", 0, 1), RichError::warning("w1", 5, 6)];
        let report = write_error_report(&errors, "main.ox");
        assert!(report.contains("=== Error Report: main.ox ==="));
        assert!(report.contains("e1"));
        assert!(report.contains("w1"));
    }
    #[test]
    fn test_error_explanation() {
        let exp = ErrorExplanation::new(
            "E0001",
            "Unexpected token",
            "You used a token that is not valid here.",
            "def x 1",
            "def x := 1",
        );
        let rendered = exp.render();
        assert!(rendered.contains("[E0001] Unexpected token"));
        assert!(rendered.contains("Bad:"));
        assert!(rendered.contains("Good:"));
    }
    #[test]
    fn test_error_explanation_book() {
        let mut book = ErrorExplanationBook::new();
        book.add(ErrorExplanation::new("E0001", "T", "D", "B", "G"));
        book.add(ErrorExplanation::new("E0002", "T2", "D2", "B2", "G2"));
        assert_eq!(book.count(), 2);
        assert!(book.lookup("E0001").is_some());
        assert!(book.lookup("E9999").is_none());
    }
    #[test]
    fn test_detect_common_mistakes() {
        let src = "def foo where\ndef bar";
        let issues = detect_common_mistakes(src);
        assert!(issues
            .iter()
            .any(|(msg, _)| *msg == "def without assignment"));
    }
    #[test]
    fn test_error_density() {
        let errors = vec![
            RichError::error("e1", 0, 1),
            RichError::error("e2", 0, 1),
            RichError::warning("w1", 0, 1),
        ];
        let density = error_density(&errors, 100);
        assert!((density - 2.0).abs() < 1e-9);
    }
    #[test]
    fn test_format_error_table() {
        let errors = vec![
            RichError::error("bad token", 0, 1).with_code("E0001"),
            RichError::warning("unused var", 5, 6),
        ];
        let table = format_error_table(&errors);
        assert!(table.contains("E0001"));
        assert!(table.contains("bad token"));
        assert!(table.contains("warning"));
    }
}
