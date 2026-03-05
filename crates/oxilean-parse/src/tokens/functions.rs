//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{Span, Token, TokenKind, TokenPairMatcher, TokenRole};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tokens::*;
    #[test]
    fn test_span_merge() {
        let s1 = Span::new(0, 5, 1, 1);
        let s2 = Span::new(10, 15, 1, 11);
        let merged = s1.merge(&s2);
        assert_eq!(merged.start, 0);
        assert_eq!(merged.end, 15);
    }
    #[test]
    fn test_token_is_ident() {
        let tok = Token::new(TokenKind::Ident("foo".to_string()), Span::new(0, 3, 1, 1));
        assert!(tok.is_ident());
        assert_eq!(tok.as_ident(), Some("foo"));
    }
    #[test]
    fn test_token_is_keyword() {
        let tok = Token::new(TokenKind::Axiom, Span::new(0, 5, 1, 1));
        assert!(tok.is_keyword());
    }
    #[test]
    fn test_new_keywords_are_keywords() {
        let keywords = vec![
            TokenKind::If,
            TokenKind::Then,
            TokenKind::Else,
            TokenKind::Match,
            TokenKind::With,
            TokenKind::Do,
            TokenKind::Have,
            TokenKind::Suffices,
            TokenKind::Show,
            TokenKind::Where,
            TokenKind::Return,
        ];
        for kw in keywords {
            let tok = Token::new(kw, Span::new(0, 1, 1, 1));
            assert!(tok.is_keyword());
        }
    }
    #[test]
    fn test_new_token_display() {
        assert_eq!(format!("{}", TokenKind::FatArrow), "=>");
        assert_eq!(format!("{}", TokenKind::DotDot), "..");
        assert_eq!(format!("{}", TokenKind::Return), "return");
        assert_eq!(format!("{}", TokenKind::Ge), ">=");
        #[allow(clippy::approx_constant)]
        let val = 3.14;
        assert_eq!(format!("{}", TokenKind::Float(val)), "3.14");
        assert_eq!(format!("{}", TokenKind::Char('x')), "'x'");
        assert_eq!(
            format!("{}", TokenKind::DocComment("a doc".to_string())),
            "/-- a doc -/"
        );
    }
    #[test]
    fn test_string_part() {
        let parts = vec![
            StringPart::Literal("hello ".to_string()),
            StringPart::Interpolation(vec![Token::new(
                TokenKind::Ident("name".to_string()),
                Span::new(0, 4, 1, 1),
            )]),
        ];
        let tok = TokenKind::InterpolatedString(parts);
        let display = format!("{}", tok);
        assert!(display.contains("hello"));
    }
}
/// Check if a token kind represents an arithmetic operator.
pub fn is_arithmetic_op(kind: &TokenKind) -> bool {
    matches!(
        kind,
        TokenKind::Plus
            | TokenKind::Minus
            | TokenKind::Star
            | TokenKind::Slash
            | TokenKind::Percent
            | TokenKind::Caret
    )
}
/// Check if a token kind represents a comparison operator.
pub fn is_comparison_op(kind: &TokenKind) -> bool {
    matches!(
        kind,
        TokenKind::Eq
            | TokenKind::Ne
            | TokenKind::Lt
            | TokenKind::Le
            | TokenKind::Gt
            | TokenKind::Ge
            | TokenKind::BangEq
    )
}
/// Check if a token kind represents a logical operator.
pub fn is_logical_op(kind: &TokenKind) -> bool {
    matches!(
        kind,
        TokenKind::And
            | TokenKind::Or
            | TokenKind::Not
            | TokenKind::Iff
            | TokenKind::AndAnd
            | TokenKind::OrOr
    )
}
/// Check if a token kind is a delimiter (bracket, brace, paren).
pub fn is_delimiter(kind: &TokenKind) -> bool {
    matches!(
        kind,
        TokenKind::LParen
            | TokenKind::RParen
            | TokenKind::LBrace
            | TokenKind::RBrace
            | TokenKind::LBracket
            | TokenKind::RBracket
            | TokenKind::LAngle
            | TokenKind::RAngle
    )
}
/// Get the closing delimiter for an opening delimiter.
pub fn matching_delimiter(open: &TokenKind) -> Option<TokenKind> {
    match open {
        TokenKind::LParen => Some(TokenKind::RParen),
        TokenKind::LBrace => Some(TokenKind::RBrace),
        TokenKind::LBracket => Some(TokenKind::RBracket),
        TokenKind::LAngle => Some(TokenKind::RAngle),
        _ => None,
    }
}
/// Check if a token kind is a declaration keyword.
pub fn is_decl_keyword(kind: &TokenKind) -> bool {
    matches!(
        kind,
        TokenKind::Axiom
            | TokenKind::Definition
            | TokenKind::Theorem
            | TokenKind::Lemma
            | TokenKind::Opaque
            | TokenKind::Inductive
            | TokenKind::Structure
            | TokenKind::Class
            | TokenKind::Instance
    )
}
/// Check if a token kind is a universe keyword.
pub fn is_universe_keyword(kind: &TokenKind) -> bool {
    matches!(kind, TokenKind::Type | TokenKind::Prop | TokenKind::Sort)
}
/// Compute the span covering a list of tokens.
///
/// Returns `None` if the list is empty.
pub fn span_of_tokens(tokens: &[Token]) -> Option<Span> {
    let first = tokens.first()?;
    let last = tokens.last()?;
    Some(first.span.merge(&last.span))
}
/// Check if a span is on a specific line.
pub fn span_on_line(span: &Span, line: usize) -> bool {
    span.line == line
}
/// Check if a string is a reserved identifier in OxiLean.
pub fn is_reserved(s: &str) -> bool {
    matches!(
        s,
        "axiom"
            | "def"
            | "definition"
            | "theorem"
            | "lemma"
            | "fun"
            | "forall"
            | "let"
            | "in"
            | "if"
            | "then"
            | "else"
            | "match"
            | "with"
            | "do"
            | "have"
            | "show"
            | "by"
            | "where"
            | "return"
            | "Type"
            | "Prop"
            | "Sort"
    )
}
/// Check if a string looks like a tactic name.
pub fn is_tactic_name(s: &str) -> bool {
    matches!(
        s,
        "intro"
            | "intros"
            | "exact"
            | "apply"
            | "assumption"
            | "refl"
            | "rfl"
            | "simp"
            | "ring"
            | "linarith"
            | "cases"
            | "induction"
            | "constructor"
            | "split"
            | "left"
            | "right"
            | "exists"
            | "use"
            | "have"
            | "show"
            | "rw"
            | "rewrite"
            | "push_neg"
            | "by_contra"
            | "contrapose"
            | "sorry"
            | "trivial"
            | "tauto"
    )
}
#[cfg(test)]
mod tokens_extra_tests {
    use super::*;
    use crate::tokens::*;
    fn make_tok(kind: TokenKind) -> Token {
        Token::new(kind, Span::new(0, 1, 1, 1))
    }
    fn sample_stream() -> TokenStream {
        TokenStream::new(vec![
            make_tok(TokenKind::Let),
            make_tok(TokenKind::Ident("x".to_string())),
            make_tok(TokenKind::Colon),
            make_tok(TokenKind::Ident("Nat".to_string())),
            make_tok(TokenKind::Assign),
            make_tok(TokenKind::Nat(42)),
            make_tok(TokenKind::Eof),
        ])
    }
    #[test]
    fn test_stream_peek() {
        let s = sample_stream();
        assert!(s.peek().is_some());
        assert!(matches!(
            s.peek().expect("test operation should succeed").kind,
            TokenKind::Let
        ));
    }
    #[test]
    fn test_stream_next() {
        let mut s = sample_stream();
        let t = s.next().expect("iterator should have next element");
        assert!(matches!(t.kind, TokenKind::Let));
    }
    #[test]
    fn test_stream_snapshot_restore() {
        let mut s = sample_stream();
        let snap = s.snapshot();
        s.next();
        assert!(matches!(
            s.peek().expect("test operation should succeed").kind,
            TokenKind::Ident(_)
        ));
        s.restore(snap);
        assert!(matches!(
            s.peek().expect("test operation should succeed").kind,
            TokenKind::Let
        ));
    }
    #[test]
    fn test_stream_expect_ok() {
        let mut s = sample_stream();
        assert!(s.expect(&TokenKind::Let).is_ok());
    }
    #[test]
    fn test_stream_expect_err() {
        let mut s = sample_stream();
        assert!(s.expect(&TokenKind::Comma).is_err());
    }
    #[test]
    fn test_stream_expect_ident() {
        let mut s = sample_stream();
        s.next();
        let name = s.expect_ident().expect("test operation should succeed");
        assert_eq!(name, "x");
    }
    #[test]
    fn test_stream_consume_if() {
        let mut s = sample_stream();
        assert!(s.consume_if(&TokenKind::Let));
        assert!(!s.consume_if(&TokenKind::Let));
    }
    #[test]
    fn test_stream_at_ident() {
        let mut s = sample_stream();
        s.next();
        assert!(s.at_ident("x"));
        assert!(!s.at_ident("y"));
    }
    #[test]
    fn test_stream_is_eof() {
        let mut s = sample_stream();
        assert!(!s.is_eof());
        for _ in 0..6 {
            s.next();
        }
        assert!(s.is_eof());
    }
    #[test]
    fn test_is_arithmetic_op() {
        assert!(is_arithmetic_op(&TokenKind::Plus));
        assert!(is_arithmetic_op(&TokenKind::Star));
        assert!(!is_arithmetic_op(&TokenKind::Colon));
    }
    #[test]
    fn test_is_comparison_op() {
        assert!(is_comparison_op(&TokenKind::Lt));
        assert!(is_comparison_op(&TokenKind::Ge));
        assert!(!is_comparison_op(&TokenKind::Plus));
    }
    #[test]
    fn test_is_logical_op() {
        assert!(is_logical_op(&TokenKind::And));
        assert!(is_logical_op(&TokenKind::OrOr));
        assert!(!is_logical_op(&TokenKind::Plus));
    }
    #[test]
    fn test_is_delimiter() {
        assert!(is_delimiter(&TokenKind::LParen));
        assert!(is_delimiter(&TokenKind::RBrace));
        assert!(!is_delimiter(&TokenKind::Plus));
    }
    #[test]
    fn test_matching_delimiter() {
        assert_eq!(
            matching_delimiter(&TokenKind::LParen),
            Some(TokenKind::RParen)
        );
        assert_eq!(
            matching_delimiter(&TokenKind::LBrace),
            Some(TokenKind::RBrace)
        );
        assert_eq!(matching_delimiter(&TokenKind::Plus), None);
    }
    #[test]
    fn test_is_decl_keyword() {
        assert!(is_decl_keyword(&TokenKind::Theorem));
        assert!(is_decl_keyword(&TokenKind::Inductive));
        assert!(!is_decl_keyword(&TokenKind::Let));
    }
    #[test]
    fn test_is_universe_keyword() {
        assert!(is_universe_keyword(&TokenKind::Type));
        assert!(is_universe_keyword(&TokenKind::Prop));
        assert!(!is_universe_keyword(&TokenKind::Let));
    }
    #[test]
    fn test_token_stats() {
        let tokens = vec![
            make_tok(TokenKind::Ident("x".to_string())),
            make_tok(TokenKind::Nat(1)),
            make_tok(TokenKind::Let),
            make_tok(TokenKind::Error("oops".to_string())),
        ];
        let stats = TokenStats::compute(&tokens);
        assert_eq!(stats.total, 4);
        assert_eq!(stats.identifiers, 1);
        assert_eq!(stats.nat_literals, 1);
        assert_eq!(stats.errors, 1);
        assert!(stats.has_errors());
    }
    #[test]
    fn test_is_reserved() {
        assert!(is_reserved("theorem"));
        assert!(is_reserved("Type"));
        assert!(!is_reserved("foo"));
    }
    #[test]
    fn test_is_tactic_name() {
        assert!(is_tactic_name("intro"));
        assert!(is_tactic_name("simp"));
        assert!(!is_tactic_name("fun"));
    }
    #[test]
    fn test_span_of_tokens() {
        let tokens = vec![
            Token::new(TokenKind::Let, Span::new(0, 3, 1, 1)),
            Token::new(TokenKind::Ident("x".to_string()), Span::new(4, 5, 1, 5)),
        ];
        let span = span_of_tokens(&tokens).expect("span should be present");
        assert_eq!(span.start, 0);
        assert_eq!(span.end, 5);
    }
    #[test]
    fn test_span_of_empty_tokens() {
        assert!(span_of_tokens(&[]).is_none());
    }
}
/// Returns `true` if the token kind represents a declaration keyword (extended check).
#[allow(dead_code)]
pub fn is_decl_keyword_ext(kind: &TokenKind) -> bool {
    matches!(
        kind,
        TokenKind::Axiom
            | TokenKind::Definition
            | TokenKind::Theorem
            | TokenKind::Lemma
            | TokenKind::Opaque
            | TokenKind::Inductive
            | TokenKind::Structure
            | TokenKind::Class
            | TokenKind::Instance
    )
}
/// Returns `true` if the token kind represents a binder keyword.
#[allow(dead_code)]
pub fn is_binder_keyword(kind: &TokenKind) -> bool {
    matches!(
        kind,
        TokenKind::Fun
            | TokenKind::Forall
            | TokenKind::Let
            | TokenKind::Have
            | TokenKind::Suffices
            | TokenKind::Show
    )
}
/// Returns `true` if the token kind represents a type-level keyword.
#[allow(dead_code)]
pub fn is_type_keyword(kind: &TokenKind) -> bool {
    matches!(kind, TokenKind::Type | TokenKind::Prop | TokenKind::Sort)
}
/// Returns `true` if the token kind represents a control-flow keyword.
#[allow(dead_code)]
pub fn is_control_flow(kind: &TokenKind) -> bool {
    matches!(
        kind,
        TokenKind::If
            | TokenKind::Then
            | TokenKind::Else
            | TokenKind::Match
            | TokenKind::With
            | TokenKind::Do
            | TokenKind::Return
    )
}
/// Returns `true` if the token represents a literal value.
#[allow(dead_code)]
pub fn is_literal(kind: &TokenKind) -> bool {
    matches!(
        kind,
        TokenKind::Nat(_)
            | TokenKind::Float(_)
            | TokenKind::String(_)
            | TokenKind::InterpolatedString(_)
            | TokenKind::Char(_)
    )
}
/// Returns `true` if the token is a punctuation character used for grouping.
#[allow(dead_code)]
pub fn is_grouping(kind: &TokenKind) -> bool {
    matches!(
        kind,
        TokenKind::LParen
            | TokenKind::RParen
            | TokenKind::LBrace
            | TokenKind::RBrace
            | TokenKind::LBracket
            | TokenKind::RBracket
            | TokenKind::LAngle
            | TokenKind::RAngle
    )
}
/// Returns `true` if the token is a relational comparison operator (extended).
#[allow(dead_code)]
pub fn is_comparison_op_ext(kind: &TokenKind) -> bool {
    matches!(
        kind,
        TokenKind::Lt
            | TokenKind::Le
            | TokenKind::Gt
            | TokenKind::Ge
            | TokenKind::Eq
            | TokenKind::Ne
    )
}
/// Returns `true` if the token is an arithmetic operator (extended).
#[allow(dead_code)]
pub fn is_arithmetic_op_ext(kind: &TokenKind) -> bool {
    matches!(
        kind,
        TokenKind::Plus
            | TokenKind::Minus
            | TokenKind::Star
            | TokenKind::Slash
            | TokenKind::Percent
            | TokenKind::Caret
    )
}
/// Returns `true` if the token is a logical operator (extended).
#[allow(dead_code)]
pub fn is_logical_op_ext(kind: &TokenKind) -> bool {
    matches!(
        kind,
        TokenKind::AndAnd
            | TokenKind::OrOr
            | TokenKind::Not
            | TokenKind::And
            | TokenKind::Or
            | TokenKind::Iff
    )
}
/// Returns `true` if the token is a namespace/module keyword.
#[allow(dead_code)]
pub fn is_scope_keyword(kind: &TokenKind) -> bool {
    matches!(
        kind,
        TokenKind::Namespace
            | TokenKind::Section
            | TokenKind::End
            | TokenKind::Open
            | TokenKind::Export
            | TokenKind::Import
    )
}
/// Filter a token slice to only include tokens of the given kind.
#[allow(dead_code)]
pub fn filter_tokens<'a>(tokens: &'a [Token], kind: &TokenKind) -> Vec<&'a Token> {
    tokens
        .iter()
        .filter(|t| std::mem::discriminant(&t.kind) == std::mem::discriminant(kind))
        .collect()
}
/// Count how many tokens match the given kind.
#[allow(dead_code)]
pub fn count_tokens(tokens: &[Token], kind: &TokenKind) -> usize {
    tokens
        .iter()
        .filter(|t| std::mem::discriminant(&t.kind) == std::mem::discriminant(kind))
        .count()
}
/// Extract all identifier names from a token slice (in order).
#[allow(dead_code)]
pub fn extract_ident_names(tokens: &[Token]) -> Vec<&str> {
    tokens
        .iter()
        .filter_map(|t| {
            if let TokenKind::Ident(s) = &t.kind {
                Some(s.as_str())
            } else {
                None
            }
        })
        .collect()
}
/// Collect spans of all tokens that match a predicate.
#[allow(dead_code)]
pub fn spans_matching<F>(tokens: &[Token], pred: F) -> Vec<Span>
where
    F: Fn(&TokenKind) -> bool,
{
    tokens
        .iter()
        .filter(|t| pred(&t.kind))
        .map(|t| t.span.clone())
        .collect()
}
/// Returns a human-readable name for a `TokenKind`.
#[allow(dead_code)]
pub fn token_kind_name(kind: &TokenKind) -> &'static str {
    match kind {
        TokenKind::Axiom => "axiom",
        TokenKind::Definition => "def",
        TokenKind::Theorem => "theorem",
        TokenKind::Lemma => "lemma",
        TokenKind::Opaque => "opaque",
        TokenKind::Inductive => "inductive",
        TokenKind::Structure => "structure",
        TokenKind::Class => "class",
        TokenKind::Instance => "instance",
        TokenKind::Namespace => "namespace",
        TokenKind::Section => "section",
        TokenKind::Variable => "variable",
        TokenKind::Variables => "variables",
        TokenKind::Parameter => "parameter",
        TokenKind::Parameters => "parameters",
        TokenKind::Constant => "constant",
        TokenKind::Constants => "constants",
        TokenKind::End => "end",
        TokenKind::Import => "import",
        TokenKind::Export => "export",
        TokenKind::Open => "open",
        TokenKind::Attribute => "attribute",
        TokenKind::Return => "return",
        TokenKind::Type => "Type",
        TokenKind::Prop => "Prop",
        TokenKind::Sort => "Sort",
        TokenKind::Fun => "fun",
        TokenKind::Forall => "forall",
        TokenKind::Let => "let",
        TokenKind::In => "in",
        TokenKind::If => "if",
        TokenKind::Then => "then",
        TokenKind::Else => "else",
        TokenKind::Match => "match",
        TokenKind::With => "with",
        TokenKind::Do => "do",
        TokenKind::Have => "have",
        TokenKind::Suffices => "suffices",
        TokenKind::Show => "show",
        TokenKind::Where => "where",
        TokenKind::From => "from",
        TokenKind::By => "by",
        TokenKind::Exists => "exists",
        TokenKind::Ident(_) => "identifier",
        TokenKind::Nat(_) => "nat-literal",
        TokenKind::Float(_) => "float-literal",
        TokenKind::String(_) => "string-literal",
        TokenKind::InterpolatedString(_) => "interpolated-string",
        TokenKind::Char(_) => "char-literal",
        TokenKind::DocComment(_) => "doc-comment",
        TokenKind::Eof => "EOF",
        TokenKind::Error(_) => "error",
        TokenKind::Underscore => "_",
        _ => "token",
    }
}
/// Returns `true` if two `TokenKind` values have the same discriminant.
#[allow(dead_code)]
pub fn same_kind(a: &TokenKind, b: &TokenKind) -> bool {
    std::mem::discriminant(a) == std::mem::discriminant(b)
}
/// The list of all reserved keyword strings.
#[allow(dead_code)]
pub const KEYWORDS: &[&str] = &[
    "axiom",
    "def",
    "definition",
    "theorem",
    "lemma",
    "opaque",
    "inductive",
    "structure",
    "class",
    "instance",
    "namespace",
    "section",
    "variable",
    "variables",
    "parameter",
    "parameters",
    "constant",
    "constants",
    "end",
    "import",
    "export",
    "open",
    "attribute",
    "return",
    "Type",
    "Prop",
    "Sort",
    "fun",
    "forall",
    "let",
    "in",
    "if",
    "then",
    "else",
    "match",
    "with",
    "do",
    "have",
    "suffices",
    "show",
    "where",
    "from",
    "by",
    "exists",
];
/// Check if a string is a reserved keyword.
#[allow(dead_code)]
pub fn is_keyword(s: &str) -> bool {
    KEYWORDS.contains(&s)
}
/// Convert a token slice to a compact debug string.
#[allow(dead_code)]
pub fn tokens_to_debug_str(tokens: &[Token]) -> String {
    tokens
        .iter()
        .map(|t| format!("{}", t.kind))
        .collect::<Vec<_>>()
        .join(" ")
}
/// Check whether a token sequence is balanced (all brackets match).
#[allow(dead_code)]
pub fn is_balanced(tokens: &[Token]) -> bool {
    let mut stack: Vec<TokenKind> = Vec::new();
    for tok in tokens {
        if let Some(closer) = TokenPairMatcher::closing_of(&tok.kind) {
            stack.push(closer);
        } else if TokenPairMatcher::opening_of(&tok.kind).is_some() {
            match stack.pop() {
                Some(expected) if expected == tok.kind => {}
                _ => return false,
            }
        }
    }
    stack.is_empty()
}
/// Find all positions where the given kind appears at bracket depth 0.
#[allow(dead_code)]
pub fn top_level_positions(tokens: &[Token], kind: &TokenKind) -> Vec<usize> {
    let mut depth: i32 = 0;
    let mut result = Vec::new();
    for (i, tok) in tokens.iter().enumerate() {
        if TokenPairMatcher::closing_of(&tok.kind).is_some() {
            depth += 1;
        } else if TokenPairMatcher::opening_of(&tok.kind).is_some() {
            depth -= 1;
        } else if depth == 0 && std::mem::discriminant(&tok.kind) == std::mem::discriminant(kind) {
            result.push(i);
        }
    }
    result
}
/// Split a token slice at each occurrence of `sep` at depth 0.
#[allow(dead_code)]
pub fn split_at_depth_zero<'a>(tokens: &'a [Token], sep: &TokenKind) -> Vec<&'a [Token]> {
    let positions = top_level_positions(tokens, sep);
    if positions.is_empty() {
        return vec![tokens];
    }
    let mut parts = Vec::new();
    let mut prev = 0;
    for pos in positions {
        parts.push(&tokens[prev..pos]);
        prev = pos + 1;
    }
    parts.push(&tokens[prev..]);
    parts
}
/// Check if two tokens have exactly the same kind (value equality).
#[allow(dead_code)]
pub fn tokens_equal(a: &Token, b: &Token) -> bool {
    a.kind == b.kind
}
/// Check if a token slice is a prefix of another.
#[allow(dead_code)]
pub fn tokens_have_prefix(tokens: &[Token], prefix: &[Token]) -> bool {
    if tokens.len() < prefix.len() {
        return false;
    }
    tokens
        .iter()
        .zip(prefix.iter())
        .all(|(a, b)| same_kind(&a.kind, &b.kind))
}
/// Reconstruct a source-like string from a token sequence.
///
/// This is not the original source text but a human-readable representation.
#[allow(dead_code)]
pub fn reconstruct_source(tokens: &[Token]) -> String {
    let mut out = String::new();
    for tok in tokens {
        if !out.is_empty() {
            out.push(' ');
        }
        out.push_str(&tok.kind.to_string());
    }
    out
}
/// Group tokens by source line number.
#[allow(dead_code)]
pub fn group_by_line(tokens: &[Token]) -> std::collections::HashMap<usize, Vec<&Token>> {
    let mut map: std::collections::HashMap<usize, Vec<&Token>> = std::collections::HashMap::new();
    for tok in tokens {
        map.entry(tok.span.line).or_default().push(tok);
    }
    map
}
#[cfg(test)]
mod extended_tokens_tests {
    use super::*;
    use crate::tokens::*;
    fn make_tok(kind: TokenKind) -> Token {
        Token::new(kind, Span::new(0, 1, 1, 1))
    }
    #[test]
    fn test_is_decl_keyword() {
        assert!(is_decl_keyword(&TokenKind::Theorem));
        assert!(is_decl_keyword(&TokenKind::Definition));
        assert!(!is_decl_keyword(&TokenKind::Let));
    }
    #[test]
    fn test_is_binder_keyword() {
        assert!(is_binder_keyword(&TokenKind::Fun));
        assert!(is_binder_keyword(&TokenKind::Forall));
        assert!(!is_binder_keyword(&TokenKind::If));
    }
    #[test]
    fn test_is_type_keyword() {
        assert!(is_type_keyword(&TokenKind::Type));
        assert!(is_type_keyword(&TokenKind::Prop));
        assert!(!is_type_keyword(&TokenKind::Let));
    }
    #[test]
    fn test_is_control_flow() {
        assert!(is_control_flow(&TokenKind::If));
        assert!(is_control_flow(&TokenKind::Match));
        assert!(!is_control_flow(&TokenKind::Type));
    }
    #[test]
    fn test_is_literal() {
        assert!(is_literal(&TokenKind::Nat(42)));
        assert!(is_literal(&TokenKind::Float(std::f64::consts::PI)));
        assert!(is_literal(&TokenKind::Char('a')));
        assert!(!is_literal(&TokenKind::Let));
    }
    #[test]
    fn test_is_grouping() {
        assert!(is_grouping(&TokenKind::LParen));
        assert!(is_grouping(&TokenKind::RBrace));
        assert!(!is_grouping(&TokenKind::Plus));
    }
    #[test]
    fn test_filter_tokens() {
        let tokens = vec![
            make_tok(TokenKind::Nat(1)),
            make_tok(TokenKind::Let),
            make_tok(TokenKind::Nat(2)),
        ];
        let result = filter_tokens(&tokens, &TokenKind::Nat(0));
        assert_eq!(result.len(), 2);
    }
    #[test]
    fn test_count_tokens() {
        let tokens = vec![
            make_tok(TokenKind::Ident("x".to_string())),
            make_tok(TokenKind::Ident("y".to_string())),
            make_tok(TokenKind::Let),
        ];
        assert_eq!(count_tokens(&tokens, &TokenKind::Ident("z".to_string())), 2);
        assert_eq!(count_tokens(&tokens, &TokenKind::Let), 1);
    }
    #[test]
    fn test_extract_ident_names() {
        let tokens = vec![
            make_tok(TokenKind::Ident("foo".to_string())),
            make_tok(TokenKind::Let),
            make_tok(TokenKind::Ident("bar".to_string())),
        ];
        let names = extract_ident_names(&tokens);
        assert_eq!(names, vec!["foo", "bar"]);
    }
    #[test]
    fn test_token_buffer_basic() {
        let mut buf = TokenBuffer::new();
        buf.push(make_tok(TokenKind::Let));
        buf.push(make_tok(TokenKind::Ident("x".to_string())));
        assert_eq!(buf.len(), 2);
        assert!(!buf.is_empty());
    }
    #[test]
    fn test_token_buffer_into_stream() {
        let mut buf = TokenBuffer::new();
        buf.push(make_tok(TokenKind::Nat(10)));
        let stream = buf.into_stream();
        assert!(!stream.is_eof());
    }
    #[test]
    fn test_token_cursor_basic() {
        let tokens = vec![
            make_tok(TokenKind::Let),
            make_tok(TokenKind::Ident("x".to_string())),
            make_tok(TokenKind::Eq),
        ];
        let mut cursor = TokenCursor::new(&tokens);
        assert!(!cursor.is_eof());
        cursor.advance();
        assert_eq!(cursor.position(), 1);
        assert_eq!(cursor.remaining(), 2);
    }
    #[test]
    fn test_token_cursor_save_restore() {
        let tokens = vec![
            make_tok(TokenKind::Let),
            make_tok(TokenKind::Ident("x".to_string())),
        ];
        let mut cursor = TokenCursor::new(&tokens);
        cursor.save();
        cursor.advance();
        assert_eq!(cursor.position(), 1);
        cursor.restore();
        assert_eq!(cursor.position(), 0);
    }
    #[test]
    fn test_token_cursor_collect_until() {
        let tokens = vec![
            make_tok(TokenKind::Ident("a".to_string())),
            make_tok(TokenKind::Ident("b".to_string())),
            make_tok(TokenKind::Comma),
            make_tok(TokenKind::Ident("c".to_string())),
        ];
        let mut cursor = TokenCursor::new(&tokens);
        let collected = cursor.collect_until(|t| t.kind == TokenKind::Comma);
        assert_eq!(collected.len(), 2);
        assert_eq!(cursor.position(), 2);
    }
    #[test]
    fn test_pair_matcher_closing_of() {
        assert_eq!(
            TokenPairMatcher::closing_of(&TokenKind::LParen),
            Some(TokenKind::RParen)
        );
        assert_eq!(
            TokenPairMatcher::closing_of(&TokenKind::LBrace),
            Some(TokenKind::RBrace)
        );
        assert_eq!(TokenPairMatcher::closing_of(&TokenKind::Let), None);
    }
    #[test]
    fn test_pair_matcher_find_closing() {
        let tokens = vec![
            make_tok(TokenKind::LParen),
            make_tok(TokenKind::Nat(1)),
            make_tok(TokenKind::LParen),
            make_tok(TokenKind::Nat(2)),
            make_tok(TokenKind::RParen),
            make_tok(TokenKind::RParen),
        ];
        assert_eq!(TokenPairMatcher::find_closing(&tokens, 0), Some(5));
        assert_eq!(TokenPairMatcher::find_closing(&tokens, 2), Some(4));
    }
    #[test]
    fn test_is_balanced_true() {
        let tokens = vec![
            make_tok(TokenKind::LParen),
            make_tok(TokenKind::Nat(1)),
            make_tok(TokenKind::RParen),
        ];
        assert!(is_balanced(&tokens));
    }
    #[test]
    fn test_is_balanced_false() {
        let tokens = vec![make_tok(TokenKind::LParen), make_tok(TokenKind::Nat(1))];
        assert!(!is_balanced(&tokens));
    }
    #[test]
    fn test_reconstruct_source() {
        let tokens = vec![
            make_tok(TokenKind::Let),
            make_tok(TokenKind::Ident("x".to_string())),
        ];
        let src = reconstruct_source(&tokens);
        assert!(src.contains("let"));
        assert!(src.contains("x"));
    }
    #[test]
    fn test_top_level_positions() {
        let tokens = vec![
            make_tok(TokenKind::LParen),
            make_tok(TokenKind::Comma),
            make_tok(TokenKind::RParen),
            make_tok(TokenKind::Comma),
        ];
        let positions = top_level_positions(&tokens, &TokenKind::Comma);
        assert_eq!(positions, vec![3]);
    }
    #[test]
    fn test_split_at_depth_zero() {
        let tokens = vec![
            make_tok(TokenKind::Nat(1)),
            make_tok(TokenKind::Comma),
            make_tok(TokenKind::Nat(2)),
            make_tok(TokenKind::Comma),
            make_tok(TokenKind::Nat(3)),
        ];
        let parts = split_at_depth_zero(&tokens, &TokenKind::Comma);
        assert_eq!(parts.len(), 3);
    }
    #[test]
    fn test_token_kind_name() {
        assert_eq!(token_kind_name(&TokenKind::Theorem), "theorem");
        assert_eq!(
            token_kind_name(&TokenKind::Ident("x".to_string())),
            "identifier"
        );
        assert_eq!(token_kind_name(&TokenKind::Nat(0)), "nat-literal");
    }
    #[test]
    fn test_annotated_token_basic() {
        let tok = make_tok(TokenKind::Ident("x".to_string()));
        let mut ann = AnnotatedToken::plain(tok);
        assert!(!ann.is_binding_site());
        ann.annotate(TokenAnnotation::BindingSite("x".to_string()));
        assert!(ann.is_binding_site());
    }
    #[test]
    fn test_annotated_token_resolved_name() {
        let tok = make_tok(TokenKind::Ident("add".to_string()));
        let mut ann = AnnotatedToken::plain(tok);
        ann.annotate(TokenAnnotation::ResolvedName("Nat.add".to_string()));
        assert_eq!(ann.resolved_name(), Some("Nat.add"));
    }
    #[test]
    fn test_group_by_line() {
        let tokens = vec![
            Token::new(TokenKind::Let, Span::new(0, 3, 1, 1)),
            Token::new(TokenKind::Ident("x".to_string()), Span::new(4, 5, 1, 5)),
            Token::new(TokenKind::Nat(1), Span::new(10, 11, 2, 1)),
        ];
        let groups = group_by_line(&tokens);
        assert_eq!(groups[&1].len(), 2);
        assert_eq!(groups[&2].len(), 1);
    }
    #[test]
    fn test_is_keyword() {
        assert!(is_keyword("theorem"));
        assert!(is_keyword("fun"));
        assert!(!is_keyword("myFunc"));
    }
    #[test]
    fn test_same_kind() {
        assert!(same_kind(&TokenKind::Nat(1), &TokenKind::Nat(2)));
        assert!(!same_kind(&TokenKind::Nat(1), &TokenKind::Float(1.0)));
    }
    #[test]
    fn test_tokens_have_prefix() {
        let tokens = vec![
            make_tok(TokenKind::Let),
            make_tok(TokenKind::Ident("x".to_string())),
            make_tok(TokenKind::Eq),
        ];
        let prefix = vec![make_tok(TokenKind::Let)];
        assert!(tokens_have_prefix(&tokens, &prefix));
    }
    #[test]
    fn test_token_buffer_split_off() {
        let mut buf = TokenBuffer::new();
        for i in 0..5 {
            buf.push(make_tok(TokenKind::Nat(i as u64)));
        }
        let second = buf.split_off(3);
        assert_eq!(buf.len(), 3);
        assert_eq!(second.len(), 2);
    }
    #[test]
    fn test_token_range_from_empty() {
        let result = TokenRange::from_tokens(vec![]);
        assert!(result.is_none());
    }
    #[test]
    fn test_token_range_from_tokens() {
        let tokens = vec![
            Token::new(TokenKind::Let, Span::new(0, 3, 1, 1)),
            Token::new(TokenKind::Ident("x".to_string()), Span::new(4, 5, 1, 5)),
        ];
        let range = TokenRange::from_tokens(tokens).expect("test operation should succeed");
        assert_eq!(range.len(), 2);
        assert_eq!(range.span.start, 0);
        assert_eq!(range.span.end, 5);
    }
}
/// Format a token kind as it would appear in a Lean 4-style error message.
#[allow(dead_code)]
pub fn format_token_for_error(kind: &TokenKind) -> String {
    match kind {
        TokenKind::Ident(s) => format!("identifier `{}`", s),
        TokenKind::Nat(n) => format!("nat literal `{}`", n),
        TokenKind::Float(f) => format!("float literal `{}`", f),
        TokenKind::String(s) => format!("string literal `\"{}\"`", s),
        TokenKind::Char(c) => format!("char literal `'{}'`", c),
        TokenKind::DocComment(s) => format!("doc comment `{}`", s),
        TokenKind::Error(e) => format!("lexer error: {}", e),
        TokenKind::Eof => "end of file".to_string(),
        other => format!("token `{}`", other),
    }
}
/// A compact single-line representation of the next few tokens in a stream.
#[allow(dead_code)]
pub fn stream_preview(tokens: &[Token], max_tokens: usize) -> String {
    let count = max_tokens.min(tokens.len());
    let parts: Vec<String> = tokens[..count].iter().map(|t| t.kind.to_string()).collect();
    if tokens.len() > max_tokens {
        format!("{} ...", parts.join(" "))
    } else {
        parts.join(" ")
    }
}
#[cfg(test)]
mod more_token_tests {
    use super::*;
    use crate::tokens::*;
    fn make_tok(kind: TokenKind) -> Token {
        Token::new(kind, Span::new(0, 1, 1, 1))
    }
    #[test]
    fn test_format_token_for_error_ident() {
        let s = format_token_for_error(&TokenKind::Ident("foo".to_string()));
        assert!(s.contains("foo"));
    }
    #[test]
    fn test_format_token_for_error_eof() {
        let s = format_token_for_error(&TokenKind::Eof);
        assert!(s.contains("end of file"));
    }
    #[test]
    fn test_stream_preview_short() {
        let tokens = vec![
            make_tok(TokenKind::Let),
            make_tok(TokenKind::Ident("x".to_string())),
        ];
        let preview = stream_preview(&tokens, 5);
        assert!(preview.contains("let"));
        assert!(preview.contains("x"));
    }
    #[test]
    fn test_stream_preview_truncated() {
        let tokens: Vec<Token> = (0..10).map(|i| make_tok(TokenKind::Nat(i))).collect();
        let preview = stream_preview(&tokens, 3);
        assert!(preview.ends_with("..."));
    }
    #[test]
    fn test_token_kind_set_basic() {
        let mut set = TokenKindSet::new();
        set.add(&TokenKind::Let);
        set.add(&TokenKind::Theorem);
        assert!(set.contains(&TokenKind::Let));
        assert!(set.contains(&TokenKind::Theorem));
        assert!(!set.contains(&TokenKind::Fun));
        assert_eq!(set.len(), 2);
    }
    #[test]
    fn test_token_kind_set_no_duplicate() {
        let mut set = TokenKindSet::new();
        set.add(&TokenKind::Let);
        set.add(&TokenKind::Let);
        assert_eq!(set.len(), 1);
    }
    #[test]
    fn test_contextual_token_from_slice() {
        let tokens = vec![
            make_tok(TokenKind::Let),
            make_tok(TokenKind::Ident("x".to_string())),
            make_tok(TokenKind::Eq),
        ];
        let ctx = ContextualToken::from_slice(&tokens);
        assert_eq!(ctx.len(), 3);
        assert!(ctx[0].prev.is_none());
        assert!(ctx[0].next.is_some());
        assert!(ctx[2].next.is_none());
    }
    #[test]
    fn test_contextual_token_same_line() {
        let tokens = vec![
            Token::new(TokenKind::Let, Span::new(0, 3, 1, 1)),
            Token::new(TokenKind::Ident("x".to_string()), Span::new(4, 5, 1, 5)),
        ];
        let ctx = ContextualToken::from_slice(&tokens);
        assert!(ctx[1].same_line_as_prev());
    }
}
/// Classify a `TokenKind` into a broad role.
#[allow(dead_code)]
pub fn classify_role(kind: &TokenKind) -> TokenRole {
    match kind {
        TokenKind::Axiom
        | TokenKind::Definition
        | TokenKind::Theorem
        | TokenKind::Lemma
        | TokenKind::Opaque
        | TokenKind::Inductive
        | TokenKind::Structure
        | TokenKind::Class
        | TokenKind::Instance => TokenRole::DeclStart,
        TokenKind::By | TokenKind::Have | TokenKind::Show | TokenKind::Suffices => {
            TokenRole::TacticKeyword
        }
        TokenKind::Type | TokenKind::Prop | TokenKind::Sort | TokenKind::Forall => {
            TokenRole::TypeLevel
        }
        TokenKind::Fun
        | TokenKind::Let
        | TokenKind::In
        | TokenKind::If
        | TokenKind::Then
        | TokenKind::Else
        | TokenKind::Match
        | TokenKind::With
        | TokenKind::Do
        | TokenKind::Return => TokenRole::TermLevel,
        TokenKind::Ident(_) => TokenRole::Name,
        TokenKind::Nat(_)
        | TokenKind::Float(_)
        | TokenKind::String(_)
        | TokenKind::Char(_)
        | TokenKind::InterpolatedString(_) => TokenRole::Literal,
        TokenKind::Eof => TokenRole::Eof,
        TokenKind::Error(_) => TokenRole::Error,
        _ => TokenRole::Punctuation,
    }
}
#[cfg(test)]
mod role_tests {
    use super::*;
    use crate::tokens::*;
    #[test]
    fn test_classify_role_decl() {
        assert_eq!(classify_role(&TokenKind::Theorem), TokenRole::DeclStart);
        assert_eq!(classify_role(&TokenKind::Inductive), TokenRole::DeclStart);
    }
    #[test]
    fn test_classify_role_name() {
        assert_eq!(
            classify_role(&TokenKind::Ident("foo".to_string())),
            TokenRole::Name
        );
    }
    #[test]
    fn test_classify_role_literal() {
        assert_eq!(classify_role(&TokenKind::Nat(0)), TokenRole::Literal);
        assert_eq!(classify_role(&TokenKind::Float(0.0)), TokenRole::Literal);
    }
    #[test]
    fn test_classify_role_eof() {
        assert_eq!(classify_role(&TokenKind::Eof), TokenRole::Eof);
    }
}
