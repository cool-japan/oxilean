//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

pub use crate::ast_impl::{Decl, Located, SurfaceExpr};
pub use crate::error_impl::{ParseError, ParseErrorKind};
pub use crate::lexer::Lexer;
pub use crate::parser_impl::Parser;
pub use crate::tokens::{Span, Token, TokenKind};

/// Parse a single expression from source text.
///
/// This is the primary entry point for parsing an isolated expression.
///
/// # Example
/// ```ignore
/// let expr = parse_expr("1 + 2").expect("valid expression");
/// ```
#[allow(missing_docs)]
pub fn parse_expr(src: &str) -> Result<Located<SurfaceExpr>, ParseError> {
    let tokens = Lexer::new(src).tokenize();
    Parser::new(tokens).parse_expr()
}
/// Parse a single declaration from source text.
#[allow(missing_docs)]
pub fn parse_decl(src: &str) -> Result<Located<Decl>, ParseError> {
    let tokens = Lexer::new(src).tokenize();
    Parser::new(tokens).parse_decl()
}
/// Parse all declarations in a source file.
///
/// Returns a vector of successfully parsed declarations; stops on the
/// first parse error.
#[allow(missing_docs)]
pub fn parse_decls(src: &str) -> Result<Vec<Located<Decl>>, ParseError> {
    let tokens = Lexer::new(src).tokenize();
    let mut parser = Parser::new(tokens);
    let mut decls = Vec::new();
    loop {
        match parser.parse_decl() {
            Ok(d) => decls.push(d),
            Err(e) if e.is_eof() => break,
            Err(e) => return Err(e),
        }
    }
    Ok(decls)
}
/// Check whether a source string is parseable as an expression.
#[allow(missing_docs)]
pub fn is_valid_expr(src: &str) -> bool {
    parse_expr(src).is_ok()
}
/// Check whether a source string is parseable as a declaration.
#[allow(missing_docs)]
pub fn is_valid_decl(src: &str) -> bool {
    parse_decl(src).is_ok()
}
/// FNV-1a 64-bit hash.
pub(super) fn fnv1a(data: &[u8]) -> u64 {
    let mut hash: u64 = 14_695_981_039_346_656_037;
    for &b in data {
        hash ^= b as u64;
        hash = hash.wrapping_mul(1_099_511_628_211);
    }
    hash
}
/// Collect a sequence of Results, stopping at the first error.
#[allow(missing_docs)]
pub fn collect_results<T, E>(iter: impl IntoIterator<Item = Result<T, E>>) -> Result<Vec<T>, E> {
    let mut out = Vec::new();
    for item in iter {
        out.push(item?);
    }
    Ok(out)
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::*;
    #[test]
    fn test_parse_expr_nat_literal() {
        let result = parse_expr("42");
        assert!(result.is_ok(), "Expected Ok but got {:?}", result);
    }
    #[test]
    fn test_parse_expr_var() {
        let result = parse_expr("x");
        assert!(result.is_ok());
    }
    #[test]
    fn test_parse_expr_lambda() {
        let result = parse_expr("fun x -> x");
        assert!(result.is_ok(), "lambda parse failed: {:?}", result);
    }
    #[test]
    fn test_parse_expr_empty_fails() {
        let result = parse_expr("");
        let _ = result;
    }
    #[test]
    fn test_is_valid_expr_true() {
        assert!(is_valid_expr("1"));
    }
    #[test]
    fn test_is_valid_expr_false() {
        assert!(!is_valid_expr("fun"));
    }
    #[test]
    fn test_parse_decl_def() {
        let result = parse_decl("def x : Nat := 0");
        assert!(result.is_ok(), "def parse failed: {:?}", result);
    }
    #[test]
    fn test_parse_decls_multiple() {
        let result = parse_decls("def a : Nat := 0");
        let _ = result;
    }
    #[test]
    fn test_token_stream_from_src() {
        let ts = TokenStream::from_src("x y z");
        assert!(!ts.is_empty());
    }
    #[test]
    fn test_token_stream_peek() {
        let ts = TokenStream::from_src("42");
        assert!(ts.peek().is_some());
    }
    #[test]
    fn test_token_stream_next() {
        let mut ts = TokenStream::from_src("a b");
        let first = ts.next();
        assert!(first.is_some());
    }
    #[test]
    fn test_token_stream_remaining() {
        let ts = TokenStream::from_src("a b c");
        let total = ts.total_len();
        assert!(total >= 3);
    }
    #[test]
    fn test_token_stream_reset() {
        let mut ts = TokenStream::from_src("a b");
        ts.next();
        ts.reset();
        assert_eq!(ts.pos, 0);
    }
    #[test]
    fn test_token_stream_collect_remaining() {
        let ts = TokenStream::from_src("x");
        let rem = ts.collect_remaining();
        assert!(!rem.is_empty());
    }
    #[test]
    fn test_parse_stats_default() {
        let s = ParseStats::new();
        assert_eq!(s.files_parsed, 0);
        assert!(s.is_clean());
    }
    #[test]
    fn test_parse_stats_avg_decls() {
        let s = ParseStats {
            files_parsed: 2,
            decls_parsed: 10,
            ..ParseStats::default()
        };
        assert_eq!(s.avg_decls_per_file(), 5.0);
    }
    #[test]
    fn test_parse_stats_avg_decls_zero_files() {
        let s = ParseStats::new();
        assert_eq!(s.avg_decls_per_file(), 0.0);
    }
    #[test]
    fn test_parse_stats_error_rate() {
        let s = ParseStats {
            decls_parsed: 10,
            errors_total: 2,
            ..ParseStats::default()
        };
        assert!((s.error_rate() - 0.2).abs() < 1e-10);
    }
    #[test]
    fn test_parse_stats_display() {
        let s = ParseStats {
            files_parsed: 3,
            decls_parsed: 5,
            errors_total: 0,
            ..ParseStats::default()
        };
        let t = format!("{}", s);
        assert!(t.contains("files: 3"));
    }
    #[test]
    fn test_parse_cache_key_from_src() {
        let k1 = ParseCacheKey::from_src("hello");
        let k2 = ParseCacheKey::from_src("hello");
        assert_eq!(k1, k2);
    }
    #[test]
    fn test_parse_cache_key_different() {
        let k1 = ParseCacheKey::from_src("a");
        let k2 = ParseCacheKey::from_src("b");
        assert_ne!(k1, k2);
    }
    #[test]
    fn test_parse_quality_rate_clean() {
        assert_eq!(ParseQuality::rate(0, 0), ParseQuality::Clean);
    }
    #[test]
    fn test_parse_quality_rate_failed() {
        assert_eq!(ParseQuality::rate(1, 0), ParseQuality::Failed);
    }
    #[test]
    fn test_parse_quality_rate_warnings() {
        assert_eq!(ParseQuality::rate(0, 2), ParseQuality::WithWarnings);
    }
    #[test]
    fn test_parse_quality_is_usable() {
        assert!(ParseQuality::Clean.is_usable());
        assert!(ParseQuality::WithWarnings.is_usable());
        assert!(!ParseQuality::Failed.is_usable());
    }
    #[test]
    fn test_parse_quality_display() {
        assert_eq!(format!("{}", ParseQuality::Clean), "clean");
        assert_eq!(format!("{}", ParseQuality::Failed), "failed");
    }
    #[test]
    fn test_parse_batch_add() {
        let mut b = ParseBatch::new();
        b.add("foo.ox", "def x : Nat := 0");
        assert_eq!(b.len(), 1);
    }
    #[test]
    fn test_parse_batch_execute() {
        let mut b = ParseBatch::new();
        b.add("a.ox", "def x : Nat := 0");
        b.add("b.ox", "def y : Nat := 1");
        let session = b.execute();
        assert_eq!(session.file_count(), 2);
    }
    #[test]
    fn test_parse_session_total_decls() {
        let mut session = ParseSession::new();
        session.parse_file("f.ox", "def x : Nat := 0");
        assert!(session.total_decls() >= 1);
    }
    #[test]
    fn test_parse_session_all_ok() {
        let session = ParseSession::new();
        assert!(session.all_ok());
    }
    #[test]
    fn test_parse_error_summary_clean() {
        let session = ParseSession::new();
        let summary = ParseErrorSummary::from_session(&session);
        assert!(summary.is_clean());
    }
    #[test]
    fn test_collect_results_ok() {
        let items: Vec<Result<i32, &str>> = vec![Ok(1), Ok(2), Ok(3)];
        let r = collect_results(items);
        assert_eq!(r.expect("test operation should succeed"), vec![1, 2, 3]);
    }
    #[test]
    fn test_collect_results_err() {
        let items: Vec<Result<i32, &str>> = vec![Ok(1), Err("oops"), Ok(3)];
        let r = collect_results(items);
        assert_eq!(r.unwrap_err(), "oops");
    }
    #[test]
    fn test_fnv1a_deterministic() {
        let h1 = fnv1a(b"hello world");
        let h2 = fnv1a(b"hello world");
        assert_eq!(h1, h2);
    }
    #[test]
    fn test_parse_file_result_decl_count() {
        let mut session = ParseSession::new();
        session.parse_file("g.ox", "def a : Nat := 0\ndef b : Nat := 0");
        let r = &session.results[0];
        assert!(r.decl_count() >= 1);
    }
}
#[cfg(test)]
mod extra_tests {
    use super::*;
    use crate::parser::*;
    #[test]
    fn test_parse_mode_strict() {
        let m = ParseMode::strict();
        assert!(!m.allow_tactics);
        assert!(!m.recover_on_error);
    }
    #[test]
    fn test_parse_mode_lenient() {
        let m = ParseMode::lenient();
        assert!(m.recover_on_error);
        assert!(m.lenient);
    }
    #[test]
    fn test_parse_mode_display() {
        let m = ParseMode::default();
        let s = format!("{}", m);
        assert!(s.contains("ParseMode"));
    }
    #[test]
    fn test_source_map_basic() {
        let src = "line1\nline2\nline3";
        let sm = SourceMap::new(src);
        assert_eq!(sm.num_lines(), 3);
    }
    #[test]
    fn test_source_map_offset_to_line_col_start() {
        let src = "abc\ndef";
        let sm = SourceMap::new(src);
        let (line, col) = sm.offset_to_line_col(0);
        assert_eq!(line, 1);
        assert_eq!(col, 1);
    }
    #[test]
    fn test_source_map_offset_to_line_col_second_line() {
        let src = "abc\ndef";
        let sm = SourceMap::new(src);
        let (line, col) = sm.offset_to_line_col(4);
        assert_eq!(line, 2);
        assert_eq!(col, 1);
    }
    #[test]
    fn test_source_map_source_len() {
        let src = "hello";
        let sm = SourceMap::new(src);
        assert_eq!(sm.source_len(), 5);
    }
    #[test]
    fn test_token_kind_set_empty() {
        let s = TokenKindSet::empty();
        assert!(s.is_empty());
    }
    #[test]
    fn test_token_kind_set_insert_contains() {
        let mut s = TokenKindSet::empty();
        s.insert(3);
        assert!(s.contains(3));
        assert!(!s.contains(4));
    }
    #[test]
    fn test_token_kind_set_union() {
        let mut a = TokenKindSet::empty();
        let mut b = TokenKindSet::empty();
        a.insert(1);
        b.insert(2);
        let u = a.union(b);
        assert!(u.contains(1));
        assert!(u.contains(2));
    }
    #[test]
    fn test_token_kind_set_intersect() {
        let mut a = TokenKindSet::empty();
        let mut b = TokenKindSet::empty();
        a.insert(1);
        a.insert(2);
        b.insert(2);
        b.insert(3);
        let i = a.intersect(b);
        assert!(i.contains(2));
        assert!(!i.contains(1));
        assert!(!i.contains(3));
    }
    #[test]
    fn test_token_kind_set_out_of_range() {
        let mut s = TokenKindSet::empty();
        s.insert(64);
        assert!(!s.contains(64));
    }
}
#[cfg(test)]
mod extra_parser_tests {
    use super::*;
    use crate::parser::*;
    #[test]
    fn test_parse_pipeline_new() {
        let p = ParsePipeline::new();
        assert_eq!(p.stage_count(), 0);
    }
    #[test]
    fn test_parse_pipeline_add_stage() {
        let mut p = ParsePipeline::new();
        p.add_stage("lex");
        p.add_stage("parse");
        assert_eq!(p.stage_count(), 2);
    }
    #[test]
    fn test_parse_pipeline_execute() {
        let p = ParsePipeline::new();
        let ts = p.execute("x y z");
        assert!(!ts.is_empty());
    }
    #[test]
    fn test_parse_pipeline_display() {
        let mut p = ParsePipeline::new();
        p.add_stage("lex");
        let s = format!("{}", p);
        assert!(s.contains("1 stages"));
    }
    #[test]
    fn test_annotation_info() {
        let span = Span::new(0, 1, 1, 1);
        let a = ParseAnnotation::info(span, "test note");
        assert_eq!(a.kind, AnnotationKind::Info);
        assert_eq!(a.message, "test note");
    }
    #[test]
    fn test_annotation_deprecated() {
        let span = Span::new(0, 1, 1, 1);
        let a = ParseAnnotation::deprecated(span, "use X instead");
        assert_eq!(a.kind, AnnotationKind::Deprecated);
    }
    #[test]
    fn test_annotation_display() {
        let span = Span::new(0, 1, 1, 1);
        let a = ParseAnnotation::info(span, "hello");
        let s = format!("{}", a);
        assert!(s.contains("info"));
        assert!(s.contains("hello"));
    }
    #[test]
    fn test_annotation_kind_display() {
        assert_eq!(format!("{}", AnnotationKind::Suggestion), "suggestion");
        assert_eq!(format!("{}", AnnotationKind::Deprecated), "deprecated");
    }
    #[test]
    fn test_parse_buffer_new() {
        let buf = ParseBuffer::new(10);
        assert!(buf.is_empty());
    }
    #[test]
    fn test_parse_buffer_push_pop() {
        let mut buf = ParseBuffer::new(10);
        let ts = TokenStream::from_src("x");
        if let Some(tok) = ts.collect_remaining().into_iter().next() {
            buf.push(tok.clone());
            assert_eq!(buf.len(), 1);
            let popped = buf.pop();
            assert!(popped.is_some());
        }
    }
    #[test]
    fn test_parse_buffer_clear() {
        let mut buf = ParseBuffer::new(10);
        let ts = TokenStream::from_src("a b c");
        for tok in ts.collect_remaining() {
            buf.push(tok.clone());
        }
        buf.clear();
        assert!(buf.is_empty());
    }
    #[test]
    fn test_parse_buffer_front() {
        let mut buf = ParseBuffer::new(10);
        let ts = TokenStream::from_src("hello");
        for tok in ts.collect_remaining() {
            buf.push(tok.clone());
        }
        assert!(buf.front().is_some());
    }
}
#[cfg(test)]
mod extended_parser_tests {
    use super::*;
    use crate::parser::*;
    #[test]
    fn test_parse_config() {
        let cfg = ParseConfig::default_config();
        assert!(cfg.allow_holes);
        assert!(cfg.recover_from_errors);
        let strict = ParseConfig::strict();
        assert!(strict.strict_mode);
        assert!(!strict.recover_from_errors);
    }
    #[test]
    fn test_parse_stats() {
        let mut s = ParseStatsExt::new();
        s.tokens_consumed = 100;
        s.backtrack_count = 10;
        s.nodes_created = 50;
        s.error_count = 5;
        assert!((s.efficiency() - 0.9).abs() < 1e-9);
        assert!((s.error_rate() - 0.1).abs() < 1e-9);
        let sum = s.summary();
        assert!(sum.contains("tokens=100"));
    }
    #[test]
    fn test_token_cursor() {
        let mut cur = TokenCursor::new(10);
        cur.advance();
        cur.advance();
        assert_eq!(cur.position, 2);
        assert_eq!(cur.remaining(), 8);
        cur.enter_scope();
        assert_eq!(cur.depth, 1);
        cur.exit_scope();
        assert!(cur.is_at_root());
    }
    #[test]
    fn test_checkpoint_stack() {
        let mut cur = TokenCursor::new(10);
        cur.advance();
        cur.advance();
        let mut stack = CheckpointStack::new();
        let cp = ParseCheckpoint::save(&cur, 0);
        stack.push(cp);
        cur.advance();
        assert_eq!(cur.position, 3);
        let cp2 = stack.pop().expect("collection should not be empty");
        cp2.restore(&mut cur);
        assert_eq!(cur.position, 2);
    }
    #[test]
    fn test_parse_trace() {
        let mut trace = ParseTrace::new(100);
        let idx = trace.enter("expr", 0);
        trace.exit(idx, 5, true);
        let idx2 = trace.enter("ty", 5);
        trace.exit(idx2, 5, false);
        assert_eq!(trace.success_count(), 1);
        assert_eq!(trace.fail_count(), 1);
        assert_eq!(trace.most_failing_rule(), Some("ty"));
    }
    #[test]
    fn test_packrat_table() {
        let mut table = PackratTable::new();
        let entry = PackratEntry {
            end_pos: 5,
            success: true,
            result_repr: "Nat".into(),
        };
        table.store(0, "ty", entry);
        let found = table.lookup(0, "ty");
        assert!(found.is_some());
        assert_eq!(found.expect("test operation should succeed").end_pos, 5);
        assert_eq!(table.size(), 1);
    }
    #[test]
    fn test_recovery_log() {
        let mut log = RecoveryLog::new();
        log.record(10, RecoveryDecision::skip(1, "skip bad token"));
        log.record(20, RecoveryDecision::abandon("unrecoverable"));
        assert_eq!(log.count(), 2);
        assert_eq!(log.abandon_count(), 1);
    }
    #[test]
    fn test_parse_ambiguity() {
        let mut amb = ParseAmbiguity::new(5, vec!["expr".into(), "ty".into()]);
        assert!(!amb.is_resolved());
        amb.resolve("expr");
        assert!(amb.is_resolved());
    }
    #[test]
    fn test_ambiguity_registry() {
        let mut reg = AmbiguityRegistry::new();
        let mut amb = ParseAmbiguity::new(0, vec!["a".into(), "b".into()]);
        amb.resolve("a");
        reg.report(amb);
        reg.report(ParseAmbiguity::new(5, vec!["c".into()]));
        assert_eq!(reg.count(), 2);
        assert_eq!(reg.unresolved(), 1);
    }
    #[test]
    fn test_fixity() {
        let f = Fixity::InfixLeft(65);
        assert_eq!(f.precedence(), 65);
        assert!(f.is_infix());
        assert!(!f.is_right_assoc());
        let rf = Fixity::InfixRight(75);
        assert!(rf.is_right_assoc());
    }
    #[test]
    fn test_fixity_registry() {
        let reg = FixityRegistry::new();
        let plus = reg.lookup("+").expect("lookup should succeed");
        assert_eq!(plus.precedence(), 65);
        assert!(plus.is_infix());
        let star = reg.lookup("*").expect("lookup should succeed");
        assert!(star.precedence() > plus.precedence());
    }
    #[test]
    fn test_lookahead_result() {
        let r = LookaheadResult::Matches(3);
        assert_ne!(r, LookaheadResult::NoMatch);
        assert_ne!(r, LookaheadResult::Ambiguous);
    }
}
/// Computes line and column from byte offset in source.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn offset_to_line_col(source: &str, offset: usize) -> (usize, usize) {
    let offset = offset.min(source.len());
    let before = &source[..offset];
    let line = before.chars().filter(|&c| c == '\n').count();
    let col = before.rfind('\n').map_or(offset, |pos| offset - pos - 1);
    (line, col)
}
/// Computes byte offset from line and column in source.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn line_col_to_offset(source: &str, line: usize, col: usize) -> usize {
    let mut cur_line = 0;
    let mut offset = 0;
    for ch in source.chars() {
        if cur_line == line {
            break;
        }
        if ch == '\n' {
            cur_line += 1;
        }
        offset += ch.len_utf8();
    }
    (offset + col).min(source.len())
}
/// Checks if a string is a valid OxiLean identifier.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn is_valid_identifier(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    let mut chars = s.chars();
    let first = chars
        .next()
        .expect("string is non-empty per is_empty check above");
    if !first.is_alphabetic() && first != '_' {
        return false;
    }
    chars.all(|c| c.is_alphanumeric() || c == '_' || c == '\'')
}
/// Checks if a string is a keyword in OxiLean.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn is_keyword(s: &str) -> bool {
    matches!(
        s,
        "def"
            | "theorem"
            | "lemma"
            | "axiom"
            | "import"
            | "open"
            | "namespace"
            | "end"
            | "let"
            | "fun"
            | "match"
            | "with"
            | "if"
            | "then"
            | "else"
            | "forall"
            | "exists"
            | "have"
            | "show"
            | "from"
            | "do"
            | "return"
            | "Type"
            | "Prop"
            | "Sort"
            | "true"
            | "false"
            | "sorry"
            | "by"
            | "inductive"
            | "structure"
            | "class"
            | "instance"
            | "where"
    )
}
/// Checks if a character is a valid start of an operator.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn is_operator_start(c: char) -> bool {
    matches!(
        c,
        '+' | '-' | '*' | '/' | '<' | '>' | '=' | '!' | '&' | '|' | '^' | '~' | '%' | '@'
    )
}
/// Checks if a string is a multi-char operator.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn is_multi_char_op(s: &str) -> bool {
    matches!(
        s,
        "->" | "=>" | "<=" | ">=" | "!=" | "==" | "&&" | "||" | ":=" | "::"
    )
}
/// Computes the nesting depth at a given position in source.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn nesting_depth_at(source: &str, pos: usize) -> usize {
    let end = (pos + 1).min(source.len());
    let prefix = &source[..end];
    let opens = prefix
        .chars()
        .filter(|&c| c == '(' || c == '[' || c == '{')
        .count();
    let closes = prefix
        .chars()
        .filter(|&c| c == ')' || c == ']' || c == '}')
        .count();
    opens.saturating_sub(closes)
}
/// Extracts the identifier at a given byte position.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn ident_at(source: &str, pos: usize) -> Option<&str> {
    let pos = pos.min(source.len());
    let start = source[..pos]
        .rfind(|c: char| !c.is_alphanumeric() && c != '_')
        .map_or(0, |i| i + 1);
    let end = source[pos..]
        .find(|c: char| !c.is_alphanumeric() && c != '_')
        .map_or(source.len(), |i| pos + i);
    if start < end && is_valid_identifier(&source[start..end]) {
        Some(&source[start..end])
    } else {
        None
    }
}
#[cfg(test)]
mod extended_parser_tests_2 {
    use super::*;
    use crate::parser::*;
    #[test]
    fn test_parse_fuel() {
        let mut fuel = ParseFuel::new(10);
        assert!(fuel.consume(5));
        assert_eq!(fuel.remaining(), 5);
        assert!(fuel.consume(5));
        assert!(!fuel.has_fuel());
        assert!(!fuel.consume(1));
    }
    #[test]
    fn test_expected_set() {
        let mut es = ExpectedSet::new();
        es.add("'('");
        es.add("identifier");
        es.add("literal");
        assert_eq!(es.count(), 3);
        let msg = es.to_message();
        assert!(msg.contains("expected"));
        es.clear();
        assert!(es.is_empty());
    }
    #[test]
    fn test_expected_set_single() {
        let mut es = ExpectedSet::new();
        es.add("')'");
        assert_eq!(es.to_message(), "expected ')'");
    }
    #[test]
    fn test_pratt_context() {
        let ctx = PrattContext::new(0);
        let ctx2 = ctx.with_min_prec(65);
        assert_eq!(ctx2.min_prec, 65);
        assert_eq!(ctx2.depth, 1);
        assert!(!ctx.is_too_deep());
    }
    #[test]
    fn test_parse_stack() {
        let mut stack = ParseStack::new();
        stack.push(ParseFrame::new("expr", 0, 1));
        stack.push(ParseFrame::new("ty", 5, 2).for_type());
        assert_eq!(stack.depth(), 2);
        assert_eq!(stack.current_rule(), Some("ty"));
        assert!(stack.in_type());
        assert!(!stack.in_pattern());
        let rules = stack.rules_string();
        assert!(rules.contains("expr > ty"));
    }
    #[test]
    fn test_source_pos() {
        let pos = SourcePos::new("foo.ox", 3, 10, 50);
        assert_eq!(pos.display(), "foo.ox:4:11");
    }
    #[test]
    fn test_offset_to_line_col() {
        let src = "hello\nworld\nfoo";
        let (line, col) = offset_to_line_col(src, 7);
        assert_eq!(line, 1);
        assert_eq!(col, 1);
    }
    #[test]
    fn test_line_col_to_offset() {
        let src = "hello\nworld";
        let off = line_col_to_offset(src, 1, 0);
        assert_eq!(off, 6);
    }
    #[test]
    fn test_is_valid_identifier() {
        assert!(is_valid_identifier("foo"));
        assert!(is_valid_identifier("_bar123"));
        assert!(is_valid_identifier("Nat"));
        assert!(!is_valid_identifier(""));
        assert!(!is_valid_identifier("123abc"));
        assert!(!is_valid_identifier("a b"));
    }
    #[test]
    fn test_is_keyword() {
        assert!(is_keyword("def"));
        assert!(is_keyword("theorem"));
        assert!(is_keyword("fun"));
        assert!(!is_keyword("foo"));
        assert!(!is_keyword("Nat"));
    }
    #[test]
    fn test_is_operator_start() {
        assert!(is_operator_start('+'));
        assert!(is_operator_start('<'));
        assert!(!is_operator_start('a'));
        assert!(!is_operator_start('_'));
    }
    #[test]
    fn test_is_multi_char_op() {
        assert!(is_multi_char_op("->"));
        assert!(is_multi_char_op(":="));
        assert!(is_multi_char_op(">="));
        assert!(!is_multi_char_op("+"));
        assert!(!is_multi_char_op("x"));
    }
    #[test]
    fn test_nesting_depth() {
        let src = "f (g (x + y))";
        assert_eq!(nesting_depth_at(src, 8), 2);
        assert_eq!(nesting_depth_at(src, 2), 1);
    }
    #[test]
    fn test_ident_at() {
        let src = "hello world";
        assert_eq!(ident_at(src, 0), Some("hello"));
        assert_eq!(ident_at(src, 6), Some("world"));
    }
    #[test]
    fn test_comb_result() {
        let r: CombResult<i32> = CombResult::Ok(42, 5);
        assert!(r.is_ok());
        assert!(!r.is_err());
        assert_eq!(r.position(), 5);
        let e: CombResult<i32> = CombResult::Err("fail".into(), 3);
        assert!(e.is_err());
        assert_eq!(e.position(), 3);
    }
}
/// A simple parser combinator: optional.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn parse_optional<T, F>(f: F, tokens: &[String], pos: usize) -> (Option<T>, usize)
where
    F: Fn(&[String], usize) -> Option<(T, usize)>,
{
    match f(tokens, pos) {
        Some((val, new_pos)) => (Some(val), new_pos),
        None => (None, pos),
    }
}
/// A simple parser combinator: many0 (zero or more).
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn parse_many0<T, F>(f: F, tokens: &[String], mut pos: usize) -> (Vec<T>, usize)
where
    F: Fn(&[String], usize) -> Option<(T, usize)>,
{
    let mut results = Vec::new();
    loop {
        match f(tokens, pos) {
            Some((val, new_pos)) if new_pos > pos => {
                results.push(val);
                pos = new_pos;
            }
            _ => break,
        }
    }
    (results, pos)
}
/// A simple parser combinator: many1 (one or more).
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn parse_many1<T, F>(f: F, tokens: &[String], pos: usize) -> Option<(Vec<T>, usize)>
where
    F: Fn(&[String], usize) -> Option<(T, usize)>,
{
    let (results, new_pos) = parse_many0(f, tokens, pos);
    if results.is_empty() {
        None
    } else {
        Some((results, new_pos))
    }
}
/// A simple parser combinator: separated list.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn parse_sep_by<T, F>(item: F, sep: &str, tokens: &[String], pos: usize) -> (Vec<T>, usize)
where
    F: Fn(&[String], usize) -> Option<(T, usize)>,
{
    let mut results = Vec::new();
    let mut cur = pos;
    match item(tokens, cur) {
        Some((v, new_pos)) => {
            results.push(v);
            cur = new_pos;
        }
        None => return (results, cur),
    }
    loop {
        if tokens.get(cur).is_some_and(|t| t == sep) {
            let next = cur + 1;
            match item(tokens, next) {
                Some((v, new_pos)) => {
                    results.push(v);
                    cur = new_pos;
                }
                None => break,
            }
        } else {
            break;
        }
    }
    (results, cur)
}
/// Parse a parenthesised sequence.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn parse_parens<T, F>(inner: F, tokens: &[String], pos: usize) -> Option<(T, usize)>
where
    F: Fn(&[String], usize) -> Option<(T, usize)>,
{
    if tokens.get(pos).is_some_and(|t| t == "(") {
        match inner(tokens, pos + 1) {
            Some((val, new_pos)) if tokens.get(new_pos).is_some_and(|t| t == ")") => {
                Some((val, new_pos + 1))
            }
            _ => None,
        }
    } else {
        None
    }
}
/// A utility to peek at token `pos` without consuming it.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn peek(tokens: &[String], pos: usize) -> Option<&str> {
    tokens.get(pos).map(|s| s.as_str())
}
/// Consume a specific token.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn consume(tokens: &[String], pos: usize, expected: &str) -> Option<usize> {
    if tokens.get(pos).is_some_and(|t| t == expected) {
        Some(pos + 1)
    } else {
        None
    }
}
/// Consume any identifier.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn consume_ident(tokens: &[String], pos: usize) -> Option<(String, usize)> {
    tokens.get(pos).and_then(|t| {
        if is_valid_identifier(t) && !is_keyword(t) {
            Some((t.clone(), pos + 1))
        } else {
            None
        }
    })
}
/// Count tokens from `pos` until `end_token` (exclusive).
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn count_until(tokens: &[String], pos: usize, end_token: &str) -> usize {
    tokens[pos..]
        .iter()
        .take_while(|t| t.as_str() != end_token)
        .count()
}
/// Find the matching closing token for an open bracket at `pos`.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn find_matching_close(
    tokens: &[String],
    pos: usize,
    open: &str,
    close: &str,
) -> Option<usize> {
    if tokens.get(pos).is_some_and(|t| t != open) {
        return None;
    }
    let mut depth = 0;
    for (i, tok) in tokens[pos..].iter().enumerate() {
        if tok == open {
            depth += 1;
        } else if tok == close {
            depth -= 1;
            if depth == 0 {
                return Some(pos + i);
            }
        }
    }
    None
}
/// A simple token-based identifier parser (returns name + new pos).
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn parse_ident(tokens: &[String], pos: usize) -> Option<(String, usize)> {
    consume_ident(tokens, pos)
}
/// A simple token-based number parser.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn parse_number(tokens: &[String], pos: usize) -> Option<(u64, usize)> {
    tokens
        .get(pos)
        .and_then(|t| t.parse::<u64>().ok())
        .map(|n| (n, pos + 1))
}
/// A simple token-based string literal parser.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn parse_string_lit(tokens: &[String], pos: usize) -> Option<(String, usize)> {
    tokens.get(pos).and_then(|t| {
        if t.starts_with('"') && t.ends_with('"') && t.len() >= 2 {
            Some((t[1..t.len() - 1].to_string(), pos + 1))
        } else {
            None
        }
    })
}
#[cfg(test)]
mod extended_parser_tests_3 {
    use super::*;
    use crate::parser::*;
    fn tok(s: &[&str]) -> Vec<String> {
        s.iter().map(|&x| x.to_string()).collect()
    }
    #[test]
    fn test_parse_optional() {
        let tokens = tok(&["foo", "bar"]);
        let result = parse_optional(
            |ts, p| {
                if ts.get(p).is_some_and(|t| t == "foo") {
                    Some(("foo", p + 1))
                } else {
                    None
                }
            },
            &tokens,
            0,
        );
        assert_eq!(result, (Some("foo"), 1));
        let result2 = parse_optional(
            |ts, p| {
                if ts.get(p).is_some_and(|t| t == "baz") {
                    Some(("baz", p + 1))
                } else {
                    None
                }
            },
            &tokens,
            0,
        );
        assert_eq!(result2, (None, 0));
    }
    #[test]
    fn test_parse_many0() {
        let tokens = tok(&["x", "x", "x", "y"]);
        let (results, pos) = parse_many0(
            |ts, p| {
                if ts.get(p).is_some_and(|t| t == "x") {
                    Some(("x", p + 1))
                } else {
                    None
                }
            },
            &tokens,
            0,
        );
        assert_eq!(results.len(), 3);
        assert_eq!(pos, 3);
    }
    #[test]
    fn test_parse_many1() {
        let tokens = tok(&["x", "y"]);
        assert!(parse_many1(
            |ts, p| {
                if ts.get(p).is_some_and(|t| t == "x") {
                    Some(("x", p + 1))
                } else {
                    None
                }
            },
            &tokens,
            0
        )
        .is_some());
        assert!(parse_many1(
            |ts, p| {
                if ts.get(p).is_some_and(|t| t == "z") {
                    Some(("z", p + 1))
                } else {
                    None
                }
            },
            &tokens,
            0
        )
        .is_none());
    }
    #[test]
    fn test_parse_sep_by() {
        let tokens = tok(&["a", ",", "b", ",", "c"]);
        let (results, pos) = parse_sep_by(
            |ts, p| {
                ts.get(p).and_then(|t| {
                    if t != "," {
                        Some((t.clone(), p + 1))
                    } else {
                        None
                    }
                })
            },
            ",",
            &tokens,
            0,
        );
        assert_eq!(
            results,
            vec!["a".to_string(), "b".to_string(), "c".to_string()]
        );
        assert_eq!(pos, 5);
    }
    #[test]
    fn test_parse_parens() {
        let tokens = tok(&["(", "x", ")"]);
        let result = parse_parens(|ts, p| ts.get(p).map(|t| (t.clone(), p + 1)), &tokens, 0);
        assert_eq!(result, Some(("x".to_string(), 3)));
    }
    #[test]
    fn test_peek_and_consume() {
        let tokens = tok(&["hello", "world"]);
        assert_eq!(peek(&tokens, 0), Some("hello"));
        assert_eq!(consume(&tokens, 0, "hello"), Some(1));
        assert_eq!(consume(&tokens, 0, "world"), None);
    }
    #[test]
    fn test_consume_ident() {
        let tokens = tok(&["foo", "def", "bar"]);
        assert_eq!(consume_ident(&tokens, 0), Some(("foo".to_string(), 1)));
        assert_eq!(consume_ident(&tokens, 1), None);
        assert_eq!(consume_ident(&tokens, 2), Some(("bar".to_string(), 3)));
    }
    #[test]
    fn test_count_until() {
        let tokens = tok(&["a", "b", "c", "end", "d"]);
        assert_eq!(count_until(&tokens, 0, "end"), 3);
    }
    #[test]
    fn test_find_matching_close() {
        let tokens = tok(&["(", "a", "(", "b", ")", "c", ")"]);
        assert_eq!(find_matching_close(&tokens, 0, "(", ")"), Some(6));
        assert_eq!(find_matching_close(&tokens, 2, "(", ")"), Some(4));
    }
    #[test]
    fn test_parse_number() {
        let tokens = tok(&["42", "abc"]);
        assert_eq!(parse_number(&tokens, 0), Some((42, 1)));
        assert_eq!(parse_number(&tokens, 1), None);
    }
    #[test]
    fn test_parse_string_lit() {
        let tokens = tok(&["\"hello\"", "world"]);
        assert_eq!(parse_string_lit(&tokens, 0), Some(("hello".to_string(), 1)));
        assert_eq!(parse_string_lit(&tokens, 1), None);
    }
    #[test]
    fn test_parse_error_simple() {
        let e = ParseErrorSimple::new(5, "unexpected token");
        assert_eq!(e.pos, 5);
        assert!(!e.recovered);
        let e2 = e.recovered();
        assert!(e2.recovered);
        let disp = format!("{}", e2);
        assert!(disp.contains("parse error at 5"));
    }
    #[test]
    fn test_parse_result_with_errors() {
        let r = ParseResultWithErrors::ok(42);
        assert!(r.is_ok());
        assert!(!r.has_errors());
        let e = ParseResultWithErrors::<i32>::err(ParseErrorSimple::new(0, "bad"));
        assert!(!e.is_ok());
        assert!(e.has_errors());
    }
    #[test]
    fn test_depth_limiter() {
        let mut lim = DepthLimiter::new(3);
        assert!(lim.enter());
        assert!(lim.enter());
        assert!(lim.enter());
        assert!(!lim.enter());
        lim.exit();
        assert!(lim.enter());
    }
}
