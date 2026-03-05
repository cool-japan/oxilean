//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::tokens::{Token, TokenKind};

use super::types::{DiagnosticCode, SyncToken};

/// Find the next synchronization token starting from position `from`.
///
/// Scans the token slice for a token that matches a common synchronization
/// point (semicolon, end keyword, declaration keyword, closing brackets, or EOF).
/// Returns the index of the found sync token, or `tokens.len()` if none found.
#[allow(dead_code)]
pub fn find_sync_token(tokens: &[Token], from: usize) -> usize {
    for (i, token) in tokens.iter().enumerate().skip(from) {
        if is_sync_kind(&token.kind) {
            return i;
        }
    }
    tokens.len()
}
/// Skip tokens until a specific sync token type is found.
///
/// Returns the index of the found sync token, or `tokens.len()` if not found.
#[allow(dead_code)]
pub fn skip_to_sync(tokens: &[Token], from: usize, sync: SyncToken) -> usize {
    for (i, token) in tokens.iter().enumerate().skip(from) {
        if matches_sync(&token.kind, sync) {
            return i;
        }
    }
    tokens.len()
}
/// Check if a token kind is any synchronization point.
pub(super) fn is_sync_kind(kind: &TokenKind) -> bool {
    matches!(
        kind,
        TokenKind::Semicolon
            | TokenKind::End
            | TokenKind::Eof
            | TokenKind::RBrace
            | TokenKind::RParen
            | TokenKind::Definition
            | TokenKind::Theorem
            | TokenKind::Lemma
            | TokenKind::Axiom
            | TokenKind::Inductive
            | TokenKind::Structure
            | TokenKind::Class
            | TokenKind::Instance
    )
}
/// Check if a token kind matches a specific sync token.
pub(super) fn matches_sync(kind: &TokenKind, sync: SyncToken) -> bool {
    match sync {
        SyncToken::Semicolon => matches!(kind, TokenKind::Semicolon),
        SyncToken::End => matches!(kind, TokenKind::End),
        SyncToken::Declaration => {
            matches!(
                kind,
                TokenKind::Definition
                    | TokenKind::Theorem
                    | TokenKind::Lemma
                    | TokenKind::Axiom
                    | TokenKind::Inductive
                    | TokenKind::Structure
                    | TokenKind::Class
                    | TokenKind::Instance
            )
        }
        SyncToken::RightBrace => matches!(kind, TokenKind::RBrace),
        SyncToken::RightParen => matches!(kind, TokenKind::RParen),
        SyncToken::Eof => matches!(kind, TokenKind::Eof),
    }
}
/// Suggest a correction when the wrong token is found.
///
/// Returns a human-readable suggestion string if a common mistake is detected.
#[allow(dead_code)]
pub fn suggest_token(expected: &TokenKind, found: &TokenKind) -> Option<String> {
    match (expected, found) {
        (TokenKind::RParen, TokenKind::RBracket) => {
            Some("did you mean `)` instead of `]`?".to_string())
        }
        (TokenKind::RParen, TokenKind::RBrace) => {
            Some("did you mean `)` instead of `}`?".to_string())
        }
        (TokenKind::RBrace, TokenKind::RParen) => {
            Some("did you mean `}` instead of `)`?".to_string())
        }
        (TokenKind::RBrace, TokenKind::RBracket) => {
            Some("did you mean `}` instead of `]`?".to_string())
        }
        (TokenKind::RBracket, TokenKind::RParen) => {
            Some("did you mean `]` instead of `)`?".to_string())
        }
        (TokenKind::RBracket, TokenKind::RBrace) => {
            Some("did you mean `]` instead of `}`?".to_string())
        }
        (TokenKind::Colon, TokenKind::Assign) => {
            Some("did you mean `:` instead of `:=`?".to_string())
        }
        (TokenKind::Assign, TokenKind::Colon) => {
            Some("did you mean `:=` instead of `:`?".to_string())
        }
        (TokenKind::Assign, TokenKind::Eq) => Some("did you mean `:=` instead of `=`?".to_string()),
        (TokenKind::Arrow, TokenKind::Eq) => Some("did you mean `->` instead of `=`?".to_string()),
        (TokenKind::Semicolon, TokenKind::Comma) => {
            Some("did you mean `;` instead of `,`?".to_string())
        }
        (TokenKind::Comma, TokenKind::Semicolon) => {
            Some("did you mean `,` instead of `;`?".to_string())
        }
        (TokenKind::Semicolon, _) => Some("missing `;`".to_string()),
        _ => None,
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::diagnostic::*;
    use crate::tokens::Span;
    fn dummy_span() -> Span {
        Span::new(0, 1, 1, 1)
    }
    fn span_at(line: usize, col: usize, start: usize, end: usize) -> Span {
        Span::new(start, end, line, col)
    }
    #[test]
    fn test_error_diagnostic() {
        let diag = Diagnostic::error("test error".to_string(), dummy_span());
        assert!(diag.is_error());
        assert!(!diag.is_warning());
    }
    #[test]
    fn test_warning_diagnostic() {
        let diag = Diagnostic::warning("test warning".to_string(), dummy_span());
        assert!(diag.is_warning());
        assert!(!diag.is_error());
    }
    #[test]
    fn test_with_label() {
        let diag = Diagnostic::error("test".to_string(), dummy_span())
            .with_label("label".to_string(), dummy_span());
        assert_eq!(diag.labels.len(), 1);
    }
    #[test]
    fn test_with_help() {
        let diag =
            Diagnostic::error("test".to_string(), dummy_span()).with_help("help text".to_string());
        assert!(diag.help.is_some());
    }
    #[test]
    fn test_collector_create() {
        let collector = DiagnosticCollector::new();
        assert_eq!(collector.error_count(), 0);
        assert!(!collector.has_errors());
    }
    #[test]
    fn test_collector_add_error() {
        let mut collector = DiagnosticCollector::new();
        collector.add(Diagnostic::error("test".to_string(), dummy_span()));
        assert_eq!(collector.error_count(), 1);
        assert!(collector.has_errors());
    }
    #[test]
    fn test_collector_add_warning() {
        let mut collector = DiagnosticCollector::new();
        collector.add(Diagnostic::warning("test".to_string(), dummy_span()));
        assert_eq!(collector.warning_count(), 1);
        assert!(!collector.has_errors());
    }
    #[test]
    fn test_collector_clear() {
        let mut collector = DiagnosticCollector::new();
        collector.add(Diagnostic::error("test".to_string(), dummy_span()));
        collector.clear();
        assert_eq!(collector.error_count(), 0);
    }
    #[test]
    fn test_with_code() {
        let diag =
            Diagnostic::error("test".to_string(), dummy_span()).with_code(DiagnosticCode::E0001);
        assert_eq!(diag.code, Some(DiagnosticCode::E0001));
    }
    #[test]
    fn test_with_fix() {
        let fix = CodeFix {
            message: "add semicolon".to_string(),
            span: dummy_span(),
            replacement: ";".to_string(),
        };
        let diag = Diagnostic::error("test".to_string(), dummy_span()).with_fix(fix);
        assert_eq!(diag.fixes.len(), 1);
        assert_eq!(diag.fixes[0].replacement, ";");
    }
    #[test]
    fn test_note_diagnostic() {
        let diag = Diagnostic::note("a note".to_string(), dummy_span());
        assert_eq!(diag.severity, Severity::Info);
        assert_eq!(diag.message, "a note");
    }
    #[test]
    fn test_diagnostic_code_display() {
        assert_eq!(format!("{}", DiagnosticCode::E0001), "E0001");
        assert_eq!(format!("{}", DiagnosticCode::E0100), "E0100");
        assert_eq!(format!("{}", DiagnosticCode::E0901), "E0901");
    }
    #[test]
    fn test_diagnostic_display_with_code() {
        let diag = Diagnostic::error("bad token".to_string(), dummy_span())
            .with_code(DiagnosticCode::E0001);
        let s = format!("{}", diag);
        assert!(s.contains("E0001"));
        assert!(s.contains("bad token"));
    }
    #[test]
    fn test_diagnostic_display_without_code() {
        let diag = Diagnostic::warning("unused var".to_string(), dummy_span());
        let s = format!("{}", diag);
        assert!(s.contains("warning"));
        assert!(s.contains("unused var"));
        assert!(!s.contains("["));
    }
    #[test]
    fn test_format_line_highlight() {
        let source = "let x := 42";
        let span = span_at(1, 5, 4, 5);
        let output = Diagnostic::format_line_highlight(source, &span);
        assert!(output.contains("let x := 42"));
        assert!(output.contains("^"));
    }
    #[test]
    fn test_format_line_highlight_out_of_range() {
        let source = "hello";
        let span = span_at(5, 1, 0, 1);
        let output = Diagnostic::format_line_highlight(source, &span);
        assert!(output.is_empty());
    }
    #[test]
    fn test_format_rich() {
        let source = "let x := 42\nlet y := true";
        let span = span_at(1, 5, 4, 5);
        let diag = Diagnostic::error("type mismatch".to_string(), span)
            .with_code(DiagnosticCode::E0100)
            .with_help("expected Nat".to_string());
        let output = diag.format_rich(source);
        assert!(output.contains("error[E0100]"));
        assert!(output.contains("type mismatch"));
        assert!(output.contains("let x := 42"));
        assert!(output.contains("expected Nat"));
    }
    #[test]
    fn test_format_rich_with_fix() {
        let source = "let x = 42";
        let span = span_at(1, 7, 6, 7);
        let fix = CodeFix {
            message: "use `:=` for assignment".to_string(),
            span: span_at(1, 7, 6, 7),
            replacement: ":=".to_string(),
        };
        let diag = Diagnostic::error("unexpected `=`".to_string(), span).with_fix(fix);
        let output = diag.format_rich(source);
        assert!(output.contains("fix:"));
        assert!(output.contains(":="));
    }
    #[test]
    fn test_diagnostics_at() {
        let mut collector = DiagnosticCollector::new();
        collector.add(Diagnostic::error("err1".to_string(), span_at(1, 1, 0, 1)));
        collector.add(Diagnostic::error("err2".to_string(), span_at(2, 1, 10, 11)));
        collector.add(Diagnostic::warning(
            "warn1".to_string(),
            span_at(1, 5, 4, 5),
        ));
        let line1 = collector.diagnostics_at(1);
        assert_eq!(line1.len(), 2);
        let line2 = collector.diagnostics_at(2);
        assert_eq!(line2.len(), 1);
        let line3 = collector.diagnostics_at(3);
        assert!(line3.is_empty());
    }
    #[test]
    fn test_info_count() {
        let mut collector = DiagnosticCollector::new();
        collector.add(Diagnostic::error("e".to_string(), dummy_span()));
        collector.add(Diagnostic::info("i1".to_string(), dummy_span()));
        collector.add(Diagnostic::info("i2".to_string(), dummy_span()));
        assert_eq!(collector.info_count(), 2);
    }
    #[test]
    fn test_sort_by_severity() {
        let mut collector = DiagnosticCollector::new();
        collector.add(Diagnostic::warning("w".to_string(), dummy_span()));
        collector.add(Diagnostic::error("e".to_string(), dummy_span()));
        collector.add(Diagnostic::info("i".to_string(), dummy_span()));
        collector.sort_by_severity();
        let diags = collector.diagnostics();
        assert_eq!(diags[0].severity, Severity::Error);
        assert_eq!(diags[1].severity, Severity::Warning);
        assert_eq!(diags[2].severity, Severity::Info);
    }
    #[test]
    fn test_sort_by_position() {
        let mut collector = DiagnosticCollector::new();
        collector.add(Diagnostic::error(
            "second".to_string(),
            span_at(2, 1, 10, 11),
        ));
        collector.add(Diagnostic::error("first".to_string(), span_at(1, 1, 0, 1)));
        collector.sort_by_position();
        let diags = collector.diagnostics();
        assert_eq!(diags[0].message, "first");
        assert_eq!(diags[1].message, "second");
    }
    #[test]
    fn test_filter_severity() {
        let mut collector = DiagnosticCollector::new();
        collector.add(Diagnostic::error("e1".to_string(), dummy_span()));
        collector.add(Diagnostic::warning("w1".to_string(), dummy_span()));
        collector.add(Diagnostic::error("e2".to_string(), dummy_span()));
        let errors = collector.filter_severity(Severity::Error);
        assert_eq!(errors.len(), 2);
        let warnings = collector.filter_severity(Severity::Warning);
        assert_eq!(warnings.len(), 1);
    }
    #[test]
    fn test_merge() {
        let mut collector1 = DiagnosticCollector::new();
        collector1.add(Diagnostic::error("e1".to_string(), dummy_span()));
        let mut collector2 = DiagnosticCollector::new();
        collector2.add(Diagnostic::warning("w1".to_string(), dummy_span()));
        collector2.add(Diagnostic::error("e2".to_string(), dummy_span()));
        collector1.merge(&collector2);
        assert_eq!(collector1.error_count(), 2);
        assert_eq!(collector1.warning_count(), 1);
        assert_eq!(collector1.diagnostics().len(), 3);
    }
    #[test]
    fn test_summary_empty() {
        let collector = DiagnosticCollector::new();
        assert_eq!(collector.summary(), "no diagnostics");
    }
    #[test]
    fn test_summary_errors_only() {
        let mut collector = DiagnosticCollector::new();
        collector.add(Diagnostic::error("e1".to_string(), dummy_span()));
        assert_eq!(collector.summary(), "1 error");
    }
    #[test]
    fn test_summary_mixed() {
        let mut collector = DiagnosticCollector::new();
        collector.add(Diagnostic::error("e1".to_string(), dummy_span()));
        collector.add(Diagnostic::error("e2".to_string(), dummy_span()));
        collector.add(Diagnostic::error("e3".to_string(), dummy_span()));
        collector.add(Diagnostic::warning("w1".to_string(), dummy_span()));
        collector.add(Diagnostic::warning("w2".to_string(), dummy_span()));
        assert_eq!(collector.summary(), "3 errors, 2 warnings");
    }
    #[test]
    fn test_summary_with_info() {
        let mut collector = DiagnosticCollector::new();
        collector.add(Diagnostic::info("i1".to_string(), dummy_span()));
        assert_eq!(collector.summary(), "1 info");
    }
    #[test]
    fn test_find_sync_token_semicolon() {
        let tokens = vec![
            Token::new(TokenKind::Ident("x".to_string()), dummy_span()),
            Token::new(TokenKind::Eq, dummy_span()),
            Token::new(TokenKind::Nat(42), dummy_span()),
            Token::new(TokenKind::Semicolon, dummy_span()),
        ];
        assert_eq!(find_sync_token(&tokens, 0), 3);
    }
    #[test]
    fn test_find_sync_token_not_found() {
        let tokens = vec![
            Token::new(TokenKind::Ident("x".to_string()), dummy_span()),
            Token::new(TokenKind::Eq, dummy_span()),
        ];
        assert_eq!(find_sync_token(&tokens, 0), tokens.len());
    }
    #[test]
    fn test_find_sync_token_eof() {
        let tokens = vec![
            Token::new(TokenKind::Ident("x".to_string()), dummy_span()),
            Token::new(TokenKind::Eof, dummy_span()),
        ];
        assert_eq!(find_sync_token(&tokens, 0), 1);
    }
    #[test]
    fn test_skip_to_sync_semicolon() {
        let tokens = vec![
            Token::new(TokenKind::Ident("a".to_string()), dummy_span()),
            Token::new(TokenKind::Ident("b".to_string()), dummy_span()),
            Token::new(TokenKind::Semicolon, dummy_span()),
            Token::new(TokenKind::Ident("c".to_string()), dummy_span()),
        ];
        assert_eq!(skip_to_sync(&tokens, 0, SyncToken::Semicolon), 2);
    }
    #[test]
    fn test_skip_to_sync_declaration() {
        let tokens = vec![
            Token::new(TokenKind::Ident("garbage".to_string()), dummy_span()),
            Token::new(TokenKind::Definition, dummy_span()),
        ];
        assert_eq!(skip_to_sync(&tokens, 0, SyncToken::Declaration), 1);
    }
    #[test]
    fn test_skip_to_sync_not_found() {
        let tokens = vec![Token::new(TokenKind::Ident("a".to_string()), dummy_span())];
        assert_eq!(skip_to_sync(&tokens, 0, SyncToken::Semicolon), tokens.len());
    }
    #[test]
    fn test_suggest_token_paren_bracket() {
        let suggestion = suggest_token(&TokenKind::RParen, &TokenKind::RBracket);
        assert!(suggestion.is_some());
        assert!(suggestion
            .expect("test operation should succeed")
            .contains(")"));
    }
    #[test]
    fn test_suggest_token_assign_eq() {
        let suggestion = suggest_token(&TokenKind::Assign, &TokenKind::Eq);
        assert!(suggestion.is_some());
        assert!(suggestion
            .expect("test operation should succeed")
            .contains(":="));
    }
    #[test]
    fn test_suggest_token_no_suggestion() {
        let suggestion = suggest_token(&TokenKind::Ident("x".to_string()), &TokenKind::Nat(42));
        assert!(suggestion.is_none());
    }
    #[test]
    fn test_suggest_token_missing_semicolon() {
        let suggestion = suggest_token(&TokenKind::Semicolon, &TokenKind::Ident("x".to_string()));
        assert!(suggestion.is_some());
        assert!(suggestion
            .expect("test operation should succeed")
            .contains(";"));
    }
    #[test]
    fn test_severity_ordering() {
        assert!(Severity::Error < Severity::Warning);
        assert!(Severity::Warning < Severity::Info);
        assert!(Severity::Info < Severity::Hint);
    }
    #[test]
    fn test_multiple_fixes() {
        let fix1 = CodeFix {
            message: "fix1".to_string(),
            span: dummy_span(),
            replacement: "a".to_string(),
        };
        let fix2 = CodeFix {
            message: "fix2".to_string(),
            span: dummy_span(),
            replacement: "b".to_string(),
        };
        let diag = Diagnostic::error("test".to_string(), dummy_span())
            .with_fix(fix1)
            .with_fix(fix2);
        assert_eq!(diag.fixes.len(), 2);
    }
    #[test]
    fn test_sort_by_position_same_line() {
        let mut collector = DiagnosticCollector::new();
        collector.add(Diagnostic::error(
            "later".to_string(),
            span_at(1, 10, 9, 10),
        ));
        collector.add(Diagnostic::error(
            "earlier".to_string(),
            span_at(1, 1, 0, 1),
        ));
        collector.sort_by_position();
        let diags = collector.diagnostics();
        assert_eq!(diags[0].message, "earlier");
        assert_eq!(diags[1].message, "later");
    }
}
/// Return a short human-readable description for each `DiagnosticCode`.
#[allow(dead_code)]
pub fn code_description(code: DiagnosticCode) -> &'static str {
    match code {
        DiagnosticCode::E0001 => "unexpected token",
        DiagnosticCode::E0002 => "unterminated string literal",
        DiagnosticCode::E0003 => "unmatched bracket",
        DiagnosticCode::E0004 => "missing semicolon",
        DiagnosticCode::E0005 => "invalid number literal",
        DiagnosticCode::E0100 => "type mismatch",
        DiagnosticCode::E0101 => "undeclared variable",
        DiagnosticCode::E0102 => "cannot infer type",
        DiagnosticCode::E0103 => "too many arguments",
        DiagnosticCode::E0104 => "too few arguments",
        DiagnosticCode::E0200 => "no goals to solve",
        DiagnosticCode::E0201 => "tactic failed",
        DiagnosticCode::E0202 => "unsolved goals",
        DiagnosticCode::E0900 => "internal error",
        DiagnosticCode::E0901 => "not implemented",
    }
}
/// Return a suggested fix hint for each `DiagnosticCode`.
#[allow(dead_code)]
pub fn code_hint(code: DiagnosticCode) -> Option<&'static str> {
    match code {
        DiagnosticCode::E0001 => Some("check that the token is valid in this position"),
        DiagnosticCode::E0002 => Some("add a closing `\"` to terminate the string"),
        DiagnosticCode::E0003 => Some("ensure brackets are properly matched and closed"),
        DiagnosticCode::E0004 => Some("add a `;` after the statement"),
        DiagnosticCode::E0005 => Some("only decimal digits are allowed in number literals"),
        DiagnosticCode::E0100 => Some("check the expected type of this expression"),
        DiagnosticCode::E0101 => Some("declare the variable or check spelling"),
        DiagnosticCode::E0102 => Some("add a type annotation, e.g. `(x : Nat)`"),
        DiagnosticCode::E0103 => Some("remove extra arguments"),
        DiagnosticCode::E0104 => Some("provide missing arguments"),
        DiagnosticCode::E0200 => Some("remove the extra tactic step"),
        DiagnosticCode::E0201 => Some("check tactic preconditions and goal state"),
        DiagnosticCode::E0202 => Some("close all goals before finishing the proof"),
        DiagnosticCode::E0900 => None,
        DiagnosticCode::E0901 => Some("this feature is not yet implemented"),
    }
}
/// Return all defined diagnostic codes.
#[allow(dead_code)]
pub fn all_codes() -> &'static [DiagnosticCode] {
    &[
        DiagnosticCode::E0001,
        DiagnosticCode::E0002,
        DiagnosticCode::E0003,
        DiagnosticCode::E0004,
        DiagnosticCode::E0005,
        DiagnosticCode::E0100,
        DiagnosticCode::E0101,
        DiagnosticCode::E0102,
        DiagnosticCode::E0103,
        DiagnosticCode::E0104,
        DiagnosticCode::E0200,
        DiagnosticCode::E0201,
        DiagnosticCode::E0202,
        DiagnosticCode::E0900,
        DiagnosticCode::E0901,
    ]
}
#[cfg(test)]
mod extra_tests {
    use super::*;
    use crate::diagnostic::*;
    use crate::tokens::Span;
    fn dummy_span() -> Span {
        Span::new(0, 1, 1, 1)
    }
    fn span_at(line: usize, col: usize, start: usize, end: usize) -> Span {
        Span::new(start, end, line, col)
    }
    #[test]
    fn test_renderer_renders_error() {
        let source = "let x := bad";
        let renderer = DiagnosticRenderer::new(source);
        let diag = Diagnostic::error("test error".to_string(), span_at(1, 1, 0, 3));
        let output = renderer.render(&diag);
        assert!(output.contains("error"));
        assert!(output.contains("test error"));
    }
    #[test]
    fn test_renderer_renders_all() {
        let renderer = DiagnosticRenderer::new("code here");
        let diags = vec![
            Diagnostic::error("e1".to_string(), dummy_span()),
            Diagnostic::warning("w1".to_string(), dummy_span()),
        ];
        let output = renderer.render_all(&diags);
        assert!(output.contains("error"));
        assert!(output.contains("warning"));
    }
    #[test]
    fn test_renderer_errors_only() {
        let renderer = DiagnosticRenderer::new("code");
        let mut c = DiagnosticCollector::new();
        c.add(Diagnostic::error("err".to_string(), dummy_span()));
        c.add(Diagnostic::warning("warn".to_string(), dummy_span()));
        let output = renderer.render_errors(&c);
        assert!(output.contains("error"));
        assert!(!output.contains("warning"));
    }
    #[test]
    fn test_filter_with_code() {
        let mut c = DiagnosticCollector::new();
        c.add(Diagnostic::error("e".to_string(), dummy_span()).with_code(DiagnosticCode::E0001));
        c.add(Diagnostic::warning("w".to_string(), dummy_span()));
        let f = DiagnosticFilter::new(&c);
        assert_eq!(f.with_code(DiagnosticCode::E0001).len(), 1);
        assert_eq!(f.with_code(DiagnosticCode::E0002).len(), 0);
    }
    #[test]
    fn test_filter_message_contains() {
        let mut c = DiagnosticCollector::new();
        c.add(Diagnostic::error(
            "type mismatch here".to_string(),
            dummy_span(),
        ));
        c.add(Diagnostic::error("bad token".to_string(), dummy_span()));
        let f = DiagnosticFilter::new(&c);
        assert_eq!(f.message_contains("mismatch").len(), 1);
    }
    #[test]
    fn test_filter_in_line_range() {
        let mut c = DiagnosticCollector::new();
        c.add(Diagnostic::error("e1".to_string(), span_at(1, 1, 0, 1)));
        c.add(Diagnostic::error("e2".to_string(), span_at(5, 1, 10, 11)));
        let f = DiagnosticFilter::new(&c);
        assert_eq!(f.in_line_range(1, 3).len(), 1);
        assert_eq!(f.in_line_range(1, 10).len(), 2);
    }
    #[test]
    fn test_filter_with_fixes() {
        let mut c = DiagnosticCollector::new();
        let fix = CodeFix {
            message: "fix".to_string(),
            span: dummy_span(),
            replacement: ";".to_string(),
        };
        c.add(Diagnostic::error("e".to_string(), dummy_span()).with_fix(fix));
        c.add(Diagnostic::warning("w".to_string(), dummy_span()));
        let f = DiagnosticFilter::new(&c);
        assert_eq!(f.with_fixes().len(), 1);
    }
    #[test]
    fn test_aggregator_totals() {
        let mut agg = DiagnosticAggregator::new("test");
        let mut c1 = DiagnosticCollector::new();
        c1.add(Diagnostic::error("e1".to_string(), dummy_span()));
        let mut c2 = DiagnosticCollector::new();
        c2.add(Diagnostic::warning("w1".to_string(), dummy_span()));
        c2.add(Diagnostic::warning("w2".to_string(), dummy_span()));
        agg.add_collector(c1);
        agg.add_collector(c2);
        assert_eq!(agg.total_errors(), 1);
        assert_eq!(agg.total_warnings(), 2);
        assert_eq!(agg.total_count(), 3);
        assert!(agg.has_errors());
    }
    #[test]
    fn test_aggregator_flat_sorted() {
        let mut agg = DiagnosticAggregator::new("sort_test");
        let mut c = DiagnosticCollector::new();
        c.add(Diagnostic::error("e2".to_string(), span_at(3, 1, 10, 11)));
        c.add(Diagnostic::error("e1".to_string(), span_at(1, 1, 0, 1)));
        agg.add_collector(c);
        let sorted = agg.flat_sorted();
        assert_eq!(sorted[0].message, "e1");
        assert_eq!(sorted[1].message, "e2");
    }
    #[test]
    fn test_builder_error() {
        let d = DiagnosticBuilder::error("err msg", dummy_span())
            .code(DiagnosticCode::E0001)
            .help("try this")
            .build();
        assert!(d.is_error());
        assert_eq!(d.code, Some(DiagnosticCode::E0001));
        assert!(d.help.is_some());
    }
    #[test]
    fn test_builder_warning_with_fix() {
        let d = DiagnosticBuilder::warning("warn", dummy_span())
            .fix("add semicolon", dummy_span(), ";")
            .build();
        assert!(d.is_warning());
        assert_eq!(d.fixes.len(), 1);
    }
    #[test]
    fn test_builder_label() {
        let d = DiagnosticBuilder::info("info", dummy_span())
            .label("here", dummy_span())
            .build();
        assert_eq!(d.labels.len(), 1);
    }
    #[test]
    fn test_group_counts() {
        let mut g = DiagnosticGroup::new("file.ox");
        g.add(Diagnostic::error("e".to_string(), dummy_span()));
        g.add(Diagnostic::warning("w".to_string(), dummy_span()));
        assert_eq!(g.error_count(), 1);
        assert_eq!(g.warning_count(), 1);
        assert_eq!(g.len(), 2);
        assert!(g.has_errors());
    }
    #[test]
    fn test_group_sort() {
        let mut g = DiagnosticGroup::new("g");
        g.add(Diagnostic::error("b".to_string(), span_at(3, 1, 10, 11)));
        g.add(Diagnostic::error("a".to_string(), span_at(1, 1, 0, 1)));
        g.sort_by_position();
        assert_eq!(g.diagnostics[0].message, "a");
    }
    #[test]
    fn test_span_contains() {
        let outer = span_at(1, 1, 0, 10);
        let inner = span_at(1, 3, 2, 5);
        assert!(SpanUtils::contains(&outer, &inner));
        assert!(!SpanUtils::contains(&inner, &outer));
    }
    #[test]
    fn test_span_overlaps() {
        let a = span_at(1, 1, 0, 5);
        let b = span_at(1, 4, 3, 8);
        assert!(SpanUtils::overlaps(&a, &b));
        let c = span_at(1, 9, 8, 10);
        assert!(!SpanUtils::overlaps(&a, &c));
    }
    #[test]
    fn test_span_merge() {
        let a = span_at(1, 1, 0, 5);
        let b = span_at(1, 4, 3, 10);
        let merged = SpanUtils::merge(&a, &b);
        assert_eq!(merged.start, 0);
        assert_eq!(merged.end, 10);
    }
    #[test]
    fn test_span_byte_len() {
        let s = span_at(1, 1, 5, 10);
        assert_eq!(SpanUtils::byte_len(&s), 5);
    }
    #[test]
    fn test_span_is_empty() {
        assert!(SpanUtils::is_empty(&span_at(1, 1, 5, 5)));
        assert!(!SpanUtils::is_empty(&span_at(1, 1, 5, 6)));
    }
    #[test]
    fn test_span_extract() {
        let src = "hello world";
        let s = span_at(1, 7, 6, 11);
        assert_eq!(SpanUtils::extract(&s, src), "world");
    }
    #[test]
    fn test_stats_from_collector() {
        let mut c = DiagnosticCollector::new();
        c.add(Diagnostic::error("e".to_string(), dummy_span()));
        c.add(Diagnostic::warning("w".to_string(), dummy_span()).with_help("h".to_string()));
        let stats = DiagnosticStats::from_collector(&c);
        assert_eq!(stats.errors, 1);
        assert_eq!(stats.warnings, 1);
        assert_eq!(stats.with_help, 1);
    }
    #[test]
    fn test_stats_total_and_has_errors() {
        let mut c = DiagnosticCollector::new();
        c.add(Diagnostic::error("e".to_string(), dummy_span()));
        let s = DiagnosticStats::from_collector(&c);
        assert_eq!(s.total(), 1);
        assert!(s.has_errors());
    }
    #[test]
    fn test_code_description() {
        assert!(!code_description(DiagnosticCode::E0001).is_empty());
        assert!(!code_description(DiagnosticCode::E0100).is_empty());
    }
    #[test]
    fn test_code_hint_some() {
        assert!(code_hint(DiagnosticCode::E0001).is_some());
    }
    #[test]
    fn test_code_hint_none() {
        assert!(code_hint(DiagnosticCode::E0900).is_none());
    }
    #[test]
    fn test_all_codes_non_empty() {
        assert!(!all_codes().is_empty());
    }
    #[test]
    fn test_exporter_to_json() {
        let d = Diagnostic::error("bad token".to_string(), dummy_span())
            .with_code(DiagnosticCode::E0001);
        let json = DiagnosticExporter::to_json(&d);
        assert!(json.contains("error"));
        assert!(json.contains("E0001"));
    }
    #[test]
    fn test_exporter_collector_to_json() {
        let mut c = DiagnosticCollector::new();
        c.add(Diagnostic::error("e".to_string(), dummy_span()));
        let json = DiagnosticExporter::collector_to_json(&c);
        assert!(json.starts_with('['));
        assert!(json.ends_with(']'));
    }
    #[test]
    fn test_exporter_to_oneliner() {
        let d = Diagnostic::warning("unused".to_string(), span_at(3, 7, 0, 1));
        let s = DiagnosticExporter::to_oneliner(&d);
        assert!(s.contains("3:7"));
        assert!(s.contains("warning"));
    }
    #[test]
    fn test_exporter_to_csv() {
        let d = Diagnostic::error("bad".to_string(), span_at(2, 5, 0, 1));
        let csv = DiagnosticExporter::to_csv(&d);
        assert!(csv.contains("2,5,error"));
    }
    #[test]
    fn test_exporter_collector_to_csv() {
        let mut c = DiagnosticCollector::new();
        c.add(Diagnostic::error("e".to_string(), dummy_span()));
        let csv = DiagnosticExporter::collector_to_csv(&c);
        assert!(csv.starts_with("line,col,severity,message"));
    }
    #[test]
    fn test_suppressor_code() {
        let sup = DiagnosticSuppressor::new().suppress_code(DiagnosticCode::E0001);
        let d = Diagnostic::error("e".to_string(), dummy_span()).with_code(DiagnosticCode::E0001);
        assert!(sup.should_suppress(&d));
        let d2 = Diagnostic::error("e".to_string(), dummy_span()).with_code(DiagnosticCode::E0002);
        assert!(!sup.should_suppress(&d2));
    }
    #[test]
    fn test_suppressor_warnings() {
        let sup = DiagnosticSuppressor::new().suppress_all_warnings();
        let d = Diagnostic::warning("w".to_string(), dummy_span());
        assert!(sup.should_suppress(&d));
        let d2 = Diagnostic::error("e".to_string(), dummy_span());
        assert!(!sup.should_suppress(&d2));
    }
    #[test]
    fn test_suppressor_filter_collector() {
        let mut c = DiagnosticCollector::new();
        c.add(Diagnostic::error("e".to_string(), dummy_span()).with_code(DiagnosticCode::E0001));
        c.add(Diagnostic::warning("w".to_string(), dummy_span()));
        let sup = DiagnosticSuppressor::new().suppress_code(DiagnosticCode::E0001);
        let filtered = sup.filter_collector(&c);
        assert_eq!(filtered.diagnostics().len(), 1);
        assert!(filtered.diagnostics()[0].is_warning());
    }
    #[test]
    fn test_policy_fail_fast() {
        let mut c = DiagnosticCollector::new();
        c.add(Diagnostic::error("e".to_string(), dummy_span()));
        assert!(DiagnosticPolicy::FailFast.should_fail(&c));
    }
    #[test]
    fn test_policy_permissive() {
        let mut c = DiagnosticCollector::new();
        c.add(Diagnostic::error("e".to_string(), dummy_span()));
        assert!(!DiagnosticPolicy::Permissive.should_fail(&c));
    }
    #[test]
    fn test_policy_warnings_as_errors() {
        let mut c = DiagnosticCollector::new();
        c.add(Diagnostic::warning("w".to_string(), dummy_span()));
        assert!(DiagnosticPolicy::WarningsAsErrors.should_fail(&c));
    }
    #[test]
    fn test_policy_name() {
        assert_eq!(DiagnosticPolicy::FailFast.name(), "fail-fast");
        assert_eq!(DiagnosticPolicy::Permissive.name(), "permissive");
    }
    #[test]
    fn test_printer_output_non_empty() {
        let mut c = DiagnosticCollector::new();
        c.add(Diagnostic::error("e".to_string(), dummy_span()));
        let printer = DiagnosticPrinter::new(DiagnosticPolicy::CollectAll);
        let out = printer.print(&c);
        assert!(!out.is_empty());
    }
    #[test]
    fn test_printer_should_fail() {
        let mut c = DiagnosticCollector::new();
        c.add(Diagnostic::error("e".to_string(), dummy_span()));
        let printer = DiagnosticPrinter::new(DiagnosticPolicy::FailFast);
        assert!(printer.should_fail(&c));
    }
    #[test]
    fn test_sync_token_info_all_non_empty() {
        assert!(!SyncTokenInfo::all().is_empty());
    }
    #[test]
    fn test_sync_token_info_semicolon_is_statement_end() {
        let all = SyncTokenInfo::all();
        let semi = all
            .iter()
            .find(|i| i.kind == SyncToken::Semicolon)
            .expect("lookup should succeed");
        assert!(semi.is_statement_end);
    }
    #[test]
    fn test_sync_token_info_right_paren_not_statement_end() {
        let all = SyncTokenInfo::all();
        let rp = all
            .iter()
            .find(|i| i.kind == SyncToken::RightParen)
            .expect("test operation should succeed");
        assert!(!rp.is_statement_end);
    }
}
#[cfg(test)]
mod diagnostic_filter_tests {
    use super::*;
    use crate::diagnostic::*;
    use crate::tokens::Span;
    #[test]
    fn test_severity_filter() {
        let f = SeverityFilter::all();
        assert_eq!(f.min_severity, 0);
        let e = SeverityFilter::errors_only();
        assert_eq!(e.min_severity, 2);
    }
}
/// Returns whether a diagnostic message matches a given pattern.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn diagnostic_matches_pattern(message: &str, pattern: &str) -> bool {
    message.contains(pattern)
}
#[cfg(test)]
mod diagnostic_event_tests {
    use super::*;
    use crate::diagnostic::*;
    #[test]
    fn test_diagnostic_event() {
        let e = DiagnosticEvent::new(1, "test message");
        assert_eq!(e.id, 1);
        assert_eq!(e.message, "test message");
    }
    #[test]
    fn test_matches_pattern() {
        assert!(diagnostic_matches_pattern(
            "unexpected token 'foo'",
            "token"
        ));
        assert!(!diagnostic_matches_pattern("unexpected token", "missing"));
    }
}
