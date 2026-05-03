//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::tokens::{Span, Token, TokenKind};

use super::types::{
    AnnotatedToken, BracketKind, OperatorArity, OperatorPriority, ReformatOptions, RichToken,
    TokenCategory, TokenMeta, TokenNgramIter,
};

/// Assign a `TokenCategory` to a `TokenKind`.
#[allow(missing_docs)]
pub fn categorise(kind: &TokenKind) -> TokenCategory {
    match kind {
        TokenKind::Axiom
        | TokenKind::Definition
        | TokenKind::Theorem
        | TokenKind::Lemma
        | TokenKind::Opaque
        | TokenKind::Inductive
        | TokenKind::Structure
        | TokenKind::Class
        | TokenKind::Instance
        | TokenKind::Namespace
        | TokenKind::Section
        | TokenKind::Variable
        | TokenKind::Variables
        | TokenKind::Parameter
        | TokenKind::Parameters
        | TokenKind::Constant
        | TokenKind::Constants
        | TokenKind::End
        | TokenKind::Import
        | TokenKind::Export
        | TokenKind::Open
        | TokenKind::Attribute
        | TokenKind::Return
        | TokenKind::Fun
        | TokenKind::Do
        | TokenKind::Let
        | TokenKind::In
        | TokenKind::If
        | TokenKind::Then
        | TokenKind::Else
        | TokenKind::Match
        | TokenKind::With
        | TokenKind::Where
        | TokenKind::Have
        | TokenKind::Show
        | TokenKind::From
        | TokenKind::By
        | TokenKind::Forall
        | TokenKind::Exists
        | TokenKind::Type => TokenCategory::Keyword,
        TokenKind::Ident(_) => TokenCategory::Identifier,
        TokenKind::Nat(_) | TokenKind::Float(_) | TokenKind::String(_) | TokenKind::Char(_) => {
            TokenCategory::Literal
        }
        TokenKind::DocComment(_) => TokenCategory::Comment,
        TokenKind::LParen
        | TokenKind::RParen
        | TokenKind::LBrace
        | TokenKind::RBrace
        | TokenKind::LBracket
        | TokenKind::RBracket
        | TokenKind::Comma
        | TokenKind::Semicolon
        | TokenKind::Colon
        | TokenKind::Dot
        | TokenKind::DotDot => TokenCategory::Punctuation,
        TokenKind::Arrow
        | TokenKind::FatArrow
        | TokenKind::Eq
        | TokenKind::Assign
        | TokenKind::Plus
        | TokenKind::Minus
        | TokenKind::Star
        | TokenKind::Slash
        | TokenKind::Percent
        | TokenKind::Caret
        | TokenKind::Bar
        | TokenKind::Bang
        | TokenKind::BangEq
        | TokenKind::Lt
        | TokenKind::Gt
        | TokenKind::Le
        | TokenKind::Ge
        | TokenKind::Ne
        | TokenKind::AndAnd
        | TokenKind::OrOr
        | TokenKind::And
        | TokenKind::Or
        | TokenKind::Not
        | TokenKind::Iff
        | TokenKind::Underscore
        | TokenKind::At => TokenCategory::Operator,
        TokenKind::Eof => TokenCategory::Eof,
        _ => TokenCategory::Other,
    }
}
/// Return `true` if `kind` can begin a term / expression.
#[allow(missing_docs)]
pub fn can_start_expr(kind: &TokenKind) -> bool {
    matches!(
        kind,
        TokenKind::Ident(_)
            | TokenKind::Nat(_)
            | TokenKind::Float(_)
            | TokenKind::String(_)
            | TokenKind::Char(_)
            | TokenKind::LParen
            | TokenKind::LBrace
            | TokenKind::LBracket
            | TokenKind::Fun
            | TokenKind::Let
            | TokenKind::If
            | TokenKind::Match
            | TokenKind::Forall
            | TokenKind::Exists
            | TokenKind::Not
            | TokenKind::Bang
            | TokenKind::Minus
            | TokenKind::Type
            | TokenKind::Underscore
    )
}
/// Return `true` if `kind` can begin a declaration.
#[allow(missing_docs)]
pub fn can_start_decl(kind: &TokenKind) -> bool {
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
            | TokenKind::Namespace
            | TokenKind::Section
            | TokenKind::Variable
            | TokenKind::Variables
            | TokenKind::Parameter
            | TokenKind::Parameters
            | TokenKind::Constant
            | TokenKind::Constants
            | TokenKind::Open
            | TokenKind::Import
            | TokenKind::End
    )
}
/// Return `true` if `kind` represents an infix binary operator.
#[allow(missing_docs)]
pub fn is_infix_op(kind: &TokenKind) -> bool {
    matches!(
        kind,
        TokenKind::Plus
            | TokenKind::Minus
            | TokenKind::Star
            | TokenKind::Slash
            | TokenKind::Percent
            | TokenKind::Eq
            | TokenKind::Ne
            | TokenKind::BangEq
            | TokenKind::Lt
            | TokenKind::Le
            | TokenKind::Gt
            | TokenKind::Ge
            | TokenKind::AndAnd
            | TokenKind::OrOr
            | TokenKind::And
            | TokenKind::Or
            | TokenKind::Iff
            | TokenKind::Arrow
    )
}
/// Return the precedence of an infix operator, or `None` if not an operator.
#[allow(missing_docs)]
pub fn infix_precedence(kind: &TokenKind) -> Option<u32> {
    match kind {
        TokenKind::Iff => Some(10),
        TokenKind::Arrow => Some(20),
        TokenKind::Or | TokenKind::OrOr => Some(30),
        TokenKind::And | TokenKind::AndAnd => Some(40),
        TokenKind::Eq
        | TokenKind::Ne
        | TokenKind::BangEq
        | TokenKind::Lt
        | TokenKind::Le
        | TokenKind::Gt
        | TokenKind::Ge => Some(50),
        TokenKind::Plus | TokenKind::Minus => Some(65),
        TokenKind::Star | TokenKind::Slash | TokenKind::Percent => Some(70),
        TokenKind::Caret => Some(75),
        _ => None,
    }
}
/// Return `true` if `kind` is a right-associative operator.
#[allow(missing_docs)]
pub fn is_right_assoc(kind: &TokenKind) -> bool {
    matches!(kind, TokenKind::Arrow | TokenKind::Caret)
}
/// Format a token kind for use in error messages.
#[allow(missing_docs)]
pub fn token_kind_display(kind: &TokenKind) -> String {
    match kind {
        TokenKind::Ident(s) => format!("identifier `{}`", s),
        TokenKind::Nat(n) => format!("number `{}`", n),
        TokenKind::Float(f) => format!("float `{}`", f),
        TokenKind::String(s) => format!("string \"{}\"", s),
        TokenKind::Char(c) => format!("char '{}'", c),
        TokenKind::Eof => "end of file".to_string(),
        other => format!("`{:?}`", other),
    }
}
/// Extract the identifier string from a token, if it is one.
#[allow(missing_docs)]
pub fn ident_of(token: &Token) -> Option<&str> {
    token.as_ident()
}
/// Extract a natural-number literal value from a token, if it is one.
#[allow(missing_docs)]
pub fn nat_lit_of(token: &Token) -> Option<u64> {
    if let TokenKind::Nat(n) = &token.kind {
        Some(*n)
    } else {
        None
    }
}
/// Return the opening bracket kind for a `TokenKind`, if any.
#[allow(missing_docs)]
pub fn opening_bracket(kind: &TokenKind) -> Option<BracketKind> {
    match kind {
        TokenKind::LParen => Some(BracketKind::Paren),
        TokenKind::LBrace => Some(BracketKind::Brace),
        TokenKind::LBracket => Some(BracketKind::Bracket),
        _ => None,
    }
}
/// Return the closing bracket kind for a `TokenKind`, if any.
#[allow(missing_docs)]
pub fn closing_bracket(kind: &TokenKind) -> Option<BracketKind> {
    match kind {
        TokenKind::RParen => Some(BracketKind::Paren),
        TokenKind::RBrace => Some(BracketKind::Brace),
        TokenKind::RBracket => Some(BracketKind::Bracket),
        _ => None,
    }
}
/// Given an opening bracket kind, return the matching closing `TokenKind`.
#[allow(missing_docs)]
pub fn closing_for(open: BracketKind) -> TokenKind {
    match open {
        BracketKind::Paren => TokenKind::RParen,
        BracketKind::Brace => TokenKind::RBrace,
        BracketKind::Bracket => TokenKind::RBracket,
    }
}
/// Given a closing bracket kind, return the matching opening `TokenKind`.
#[allow(missing_docs)]
pub fn opening_for(close: BracketKind) -> TokenKind {
    match close {
        BracketKind::Paren => TokenKind::LParen,
        BracketKind::Brace => TokenKind::LBrace,
        BracketKind::Bracket => TokenKind::LBracket,
    }
}
/// Verify that all brackets in `tokens` are balanced.
///
/// Returns `Ok(())` if balanced, or an `Err` with the position of the first
/// mismatch.
#[allow(missing_docs)]
pub fn check_bracket_balance(tokens: &[Token]) -> Result<(), (usize, String)> {
    let mut stack: Vec<(BracketKind, usize)> = Vec::new();
    for (i, tok) in tokens.iter().enumerate() {
        if let Some(kind) = opening_bracket(&tok.kind) {
            stack.push((kind, i));
        } else if let Some(kind) = closing_bracket(&tok.kind) {
            match stack.pop() {
                Some((open, _)) if open == kind => {}
                Some((open, pos)) => {
                    return Err((
                        i,
                        format!(
                            "bracket mismatch: opened {:?} at index {}, closed {:?} at index {}",
                            open, pos, kind, i
                        ),
                    ));
                }
                None => {
                    return Err((
                        i,
                        format!("unexpected closing bracket {:?} at index {}", kind, i),
                    ));
                }
            }
        }
    }
    if let Some((kind, pos)) = stack.pop() {
        return Err((pos, format!("unclosed bracket {:?} at index {}", kind, pos)));
    }
    Ok(())
}
/// Strip all comment tokens from a token list.
#[allow(missing_docs)]
pub fn strip_comments(tokens: Vec<Token>) -> Vec<Token> {
    tokens
        .into_iter()
        .filter(|t| !matches!(t.kind, TokenKind::DocComment(_)))
        .collect()
}
/// Count tokens with the given kind.
#[allow(missing_docs)]
pub fn count_kind(tokens: &[Token], kind: &TokenKind) -> usize {
    tokens.iter().filter(|t| &t.kind == kind).count()
}
/// Return the span covering the entire slice of tokens, or a dummy span if empty.
#[allow(missing_docs)]
pub fn covering_span(tokens: &[Token]) -> Span {
    if tokens.is_empty() {
        return Span::new(0, 0, 1, 1);
    }
    let first = &tokens[0].span;
    let last = &tokens[tokens.len() - 1].span;
    first.merge(last)
}
/// Return `true` if the token list contains any identifier with the given name.
#[allow(missing_docs)]
pub fn contains_ident(tokens: &[Token], name: &str) -> bool {
    tokens
        .iter()
        .any(|t| matches!(& t.kind, TokenKind::Ident(s) if s == name))
}
/// Collect all identifier names from a token slice.
#[allow(missing_docs)]
pub fn collect_idents(tokens: &[Token]) -> Vec<String> {
    tokens
        .iter()
        .filter_map(|t| {
            if let TokenKind::Ident(s) = &t.kind {
                Some(s.clone())
            } else {
                None
            }
        })
        .collect()
}
/// Split a token slice at every occurrence of `sep`, returning the groups.
///
/// The separator token itself is not included in any group.
#[allow(missing_docs)]
pub fn split_at_kind(tokens: &[Token], sep: &TokenKind) -> Vec<Vec<Token>> {
    let mut groups: Vec<Vec<Token>> = vec![Vec::new()];
    for tok in tokens {
        if &tok.kind == sep {
            groups.push(Vec::new());
        } else {
            groups
                .last_mut()
                .expect("groups initialized with one element and only grows")
                .push(tok.clone());
        }
    }
    groups
}
/// Check whether a token is a `:=` (assign) token.
#[allow(missing_docs)]
pub fn is_assign(tok: &Token) -> bool {
    matches!(tok.kind, TokenKind::Assign)
}
/// Check whether a token is a `:` (colon) token.
#[allow(missing_docs)]
pub fn is_colon(tok: &Token) -> bool {
    matches!(tok.kind, TokenKind::Colon)
}
/// Check whether a token is an identifier.
#[allow(missing_docs)]
pub fn is_ident_token(tok: &Token) -> bool {
    tok.is_ident()
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::*;
    fn make_token(kind: TokenKind) -> Token {
        Token::new(kind, Span::new(0, 1, 1, 1))
    }
    #[test]
    fn test_categorise_keyword() {
        assert_eq!(categorise(&TokenKind::Theorem), TokenCategory::Keyword);
        assert_eq!(categorise(&TokenKind::Fun), TokenCategory::Keyword);
        assert_eq!(categorise(&TokenKind::Let), TokenCategory::Keyword);
    }
    #[test]
    fn test_categorise_identifier() {
        assert_eq!(
            categorise(&TokenKind::Ident("foo".to_string())),
            TokenCategory::Identifier
        );
    }
    #[test]
    fn test_categorise_literal() {
        assert_eq!(categorise(&TokenKind::Nat(42)), TokenCategory::Literal);
        assert_eq!(
            categorise(&TokenKind::String("hello".to_string())),
            TokenCategory::Literal
        );
    }
    #[test]
    fn test_categorise_operator() {
        assert_eq!(categorise(&TokenKind::Arrow), TokenCategory::Operator);
        assert_eq!(categorise(&TokenKind::Plus), TokenCategory::Operator);
    }
    #[test]
    fn test_token_stream_peek_and_next() {
        let tokens = vec![
            make_token(TokenKind::Ident("x".to_string())),
            make_token(TokenKind::Eq),
            make_token(TokenKind::Nat(1)),
        ];
        let mut stream = TokenStream::new(tokens);
        assert_eq!(
            stream.peek().map(|t| &t.kind),
            Some(&TokenKind::Ident("x".to_string()))
        );
        let _ = stream.next();
        assert_eq!(stream.peek().map(|t| &t.kind), Some(&TokenKind::Eq));
    }
    #[test]
    fn test_token_stream_eat() {
        let tokens = vec![make_token(TokenKind::Colon), make_token(TokenKind::Eq)];
        let mut stream = TokenStream::new(tokens);
        assert!(stream.eat(&TokenKind::Eq).is_none());
        assert!(stream.eat(&TokenKind::Colon).is_some());
        assert!(stream.eat(&TokenKind::Eq).is_some());
    }
    #[test]
    fn test_infix_precedence_ordering() {
        let plus = infix_precedence(&TokenKind::Plus).expect("test operation should succeed");
        let times = infix_precedence(&TokenKind::Star).expect("test operation should succeed");
        let eq = infix_precedence(&TokenKind::Eq).expect("test operation should succeed");
        assert!(times > plus);
        assert!(plus > eq);
    }
    #[test]
    fn test_check_bracket_balance_ok() {
        let tokens = vec![
            make_token(TokenKind::LParen),
            make_token(TokenKind::Ident("x".to_string())),
            make_token(TokenKind::RParen),
        ];
        assert!(check_bracket_balance(&tokens).is_ok());
    }
    #[test]
    fn test_check_bracket_balance_mismatch() {
        let tokens = vec![
            make_token(TokenKind::LParen),
            make_token(TokenKind::RBracket),
        ];
        assert!(check_bracket_balance(&tokens).is_err());
    }
    #[test]
    fn test_check_bracket_balance_unclosed() {
        let tokens = vec![make_token(TokenKind::LBrace)];
        assert!(check_bracket_balance(&tokens).is_err());
    }
    #[test]
    fn test_covering_span() {
        let t1 = Token::new(TokenKind::Ident("a".to_string()), Span::new(0, 1, 1, 1));
        let t2 = Token::new(TokenKind::Nat(5), Span::new(5, 6, 1, 6));
        let span = covering_span(&[t1, t2]);
        assert_eq!(span.start, 0);
        assert_eq!(span.end, 6);
    }
    #[test]
    fn test_can_start_expr() {
        assert!(can_start_expr(&TokenKind::Ident("x".to_string())));
        assert!(can_start_expr(&TokenKind::Nat(0)));
        assert!(can_start_expr(&TokenKind::LParen));
        assert!(!can_start_expr(&TokenKind::Comma));
    }
    #[test]
    fn test_can_start_decl() {
        assert!(can_start_decl(&TokenKind::Theorem));
        assert!(can_start_decl(&TokenKind::Definition));
        assert!(!can_start_decl(&TokenKind::Plus));
    }
    #[test]
    fn test_count_kind() {
        let tokens = vec![
            make_token(TokenKind::Comma),
            make_token(TokenKind::Ident("a".to_string())),
            make_token(TokenKind::Comma),
        ];
        assert_eq!(count_kind(&tokens, &TokenKind::Comma), 2);
    }
    #[test]
    fn test_is_right_assoc() {
        assert!(is_right_assoc(&TokenKind::Arrow));
        assert!(is_right_assoc(&TokenKind::Caret));
        assert!(!is_right_assoc(&TokenKind::Plus));
    }
    #[test]
    fn test_token_stream_save_rewind() {
        let tokens = vec![
            make_token(TokenKind::Nat(1)),
            make_token(TokenKind::Nat(2)),
            make_token(TokenKind::Nat(3)),
        ];
        let mut stream = TokenStream::new(tokens);
        let saved = stream.save();
        let _ = stream.next();
        let _ = stream.next();
        assert_eq!(stream.position(), 2);
        stream.rewind(saved);
        assert_eq!(stream.position(), 0);
    }
    #[test]
    fn test_split_at_kind() {
        let tokens = vec![
            make_token(TokenKind::Nat(1)),
            make_token(TokenKind::Comma),
            make_token(TokenKind::Nat(2)),
            make_token(TokenKind::Comma),
            make_token(TokenKind::Nat(3)),
        ];
        let groups = split_at_kind(&tokens, &TokenKind::Comma);
        assert_eq!(groups.len(), 3);
        assert_eq!(groups[0].len(), 1);
        assert_eq!(groups[1].len(), 1);
        assert_eq!(groups[2].len(), 1);
    }
    #[test]
    fn test_collect_idents() {
        let tokens = vec![
            make_token(TokenKind::Ident("foo".to_string())),
            make_token(TokenKind::Plus),
            make_token(TokenKind::Ident("bar".to_string())),
        ];
        let idents = collect_idents(&tokens);
        assert_eq!(idents, vec!["foo", "bar"]);
    }
    #[test]
    fn test_contains_ident() {
        let tokens = vec![
            make_token(TokenKind::Ident("alpha".to_string())),
            make_token(TokenKind::Comma),
        ];
        assert!(contains_ident(&tokens, "alpha"));
        assert!(!contains_ident(&tokens, "beta"));
    }
}
/// Look up the binding priority of an operator token.
#[allow(missing_docs)]
pub fn operator_priority(kind: &TokenKind) -> OperatorPriority {
    match kind {
        TokenKind::Plus | TokenKind::Minus => OperatorPriority(60),
        TokenKind::Star | TokenKind::Slash => OperatorPriority(70),
        TokenKind::Caret => OperatorPriority(80),
        TokenKind::Eq
        | TokenKind::Ne
        | TokenKind::Lt
        | TokenKind::Gt
        | TokenKind::Le
        | TokenKind::Ge => OperatorPriority(50),
        TokenKind::AndAnd => OperatorPriority(40),
        TokenKind::OrOr => OperatorPriority(30),
        TokenKind::Arrow => OperatorPriority(20),
        _ => OperatorPriority(0),
    }
}
/// Determine the arity of an operator token.
#[allow(missing_docs)]
pub fn operator_arity(kind: &TokenKind) -> OperatorArity {
    match kind {
        TokenKind::Not | TokenKind::Minus => OperatorArity::Unary,
        TokenKind::Plus
        | TokenKind::Star
        | TokenKind::Slash
        | TokenKind::Eq
        | TokenKind::Ne
        | TokenKind::Lt
        | TokenKind::Gt
        | TokenKind::Le
        | TokenKind::Ge
        | TokenKind::Arrow
        | TokenKind::AndAnd
        | TokenKind::OrOr
        | TokenKind::Caret
        | TokenKind::Iff => OperatorArity::Binary,
        _ => OperatorArity::None,
    }
}
/// Convert a slice of tokens to a `Vec<RichToken>`.
#[allow(missing_docs)]
pub fn enrich_tokens(tokens: &[Token]) -> Vec<RichToken> {
    tokens
        .iter()
        .map(|t| RichToken::from_token(t.clone()))
        .collect()
}
/// Find the first token with the given category.
#[allow(missing_docs)]
pub fn find_by_category(tokens: &[Token], cat: TokenCategory) -> Option<&Token> {
    tokens.iter().find(|t| categorise(&t.kind) == cat)
}
/// Return all tokens matching a predicate.
#[allow(missing_docs)]
pub fn filter_tokens<F: Fn(&Token) -> bool>(tokens: &[Token], pred: F) -> Vec<&Token> {
    tokens.iter().filter(|t| pred(t)).collect()
}
/// Check if the token slice contains any operator.
#[allow(missing_docs)]
pub fn has_operator(tokens: &[Token]) -> bool {
    tokens
        .iter()
        .any(|t| operator_arity(&t.kind) != OperatorArity::None)
}
/// Strip leading and trailing EOF tokens.
#[allow(missing_docs)]
pub fn strip_eof(tokens: &[Token]) -> &[Token] {
    let start = tokens
        .iter()
        .position(|t| !matches!(t.kind, TokenKind::Eof))
        .unwrap_or(0);
    let end = tokens
        .iter()
        .rposition(|t| !matches!(t.kind, TokenKind::Eof))
        .map(|i| i + 1)
        .unwrap_or(start);
    &tokens[start..end]
}
/// Count the total number of characters spanned by a token slice.
#[allow(missing_docs)]
pub fn span_char_count(tokens: &[Token]) -> usize {
    tokens
        .iter()
        .map(|t| t.span.end.saturating_sub(t.span.start))
        .sum()
}
/// Return the maximum depth of bracket nesting in a token slice.
#[allow(missing_docs)]
pub fn max_bracket_depth(tokens: &[Token]) -> u32 {
    let mut depth = 0u32;
    let mut max_depth = 0u32;
    for tok in tokens {
        match &tok.kind {
            TokenKind::LParen | TokenKind::LBracket | TokenKind::LBrace => {
                depth += 1;
                if depth > max_depth {
                    max_depth = depth;
                }
            }
            TokenKind::RParen | TokenKind::RBracket | TokenKind::RBrace => {
                depth = depth.saturating_sub(1);
            }
            _ => {}
        }
    }
    max_depth
}
#[cfg(test)]
mod token_extended_tests {
    use super::*;
    use crate::token::*;
    fn make_tok(kind: TokenKind) -> Token {
        Token::new(kind, Span::new(0, 1, 1, 1))
    }
    #[test]
    fn test_operator_priority_ordering() {
        let star = operator_priority(&TokenKind::Star);
        let plus = operator_priority(&TokenKind::Plus);
        assert!(star > plus);
    }
    #[test]
    fn test_operator_arity_plus() {
        assert_eq!(operator_arity(&TokenKind::Plus), OperatorArity::Binary);
    }
    #[test]
    fn test_operator_arity_not() {
        assert_eq!(operator_arity(&TokenKind::Not), OperatorArity::Unary);
    }
    #[test]
    fn test_operator_arity_comma() {
        assert_eq!(operator_arity(&TokenKind::Comma), OperatorArity::None);
    }
    #[test]
    fn test_rich_token_infix() {
        let t = make_tok(TokenKind::Plus);
        let rt = RichToken::from_token(t);
        assert!(rt.is_infix());
        assert!(!rt.is_prefix());
    }
    #[test]
    fn test_enrich_tokens() {
        let tokens = vec![make_tok(TokenKind::Plus), make_tok(TokenKind::Nat(1))];
        let rich = enrich_tokens(&tokens);
        assert_eq!(rich.len(), 2);
        assert!(rich[0].is_infix());
    }
    #[test]
    fn test_find_by_category() {
        let tokens = vec![make_tok(TokenKind::Plus), make_tok(TokenKind::Nat(5))];
        let found = find_by_category(&tokens, TokenCategory::Literal);
        assert!(found.is_some());
    }
    #[test]
    fn test_filter_tokens() {
        let tokens = vec![
            make_tok(TokenKind::Plus),
            make_tok(TokenKind::Minus),
            make_tok(TokenKind::Nat(1)),
        ];
        let ops = filter_tokens(&tokens, |t| operator_arity(&t.kind) != OperatorArity::None);
        assert_eq!(ops.len(), 2);
    }
    #[test]
    fn test_has_operator_true() {
        let tokens = vec![make_tok(TokenKind::Plus)];
        assert!(has_operator(&tokens));
    }
    #[test]
    fn test_has_operator_false() {
        let tokens = vec![make_tok(TokenKind::Comma)];
        assert!(!has_operator(&tokens));
    }
    #[test]
    fn test_strip_eof() {
        let tokens = vec![
            make_tok(TokenKind::Eof),
            make_tok(TokenKind::Nat(1)),
            make_tok(TokenKind::Eof),
        ];
        let stripped = strip_eof(&tokens);
        assert_eq!(stripped.len(), 1);
    }
    #[test]
    fn test_max_bracket_depth() {
        let tokens = vec![
            make_tok(TokenKind::LParen),
            make_tok(TokenKind::LParen),
            make_tok(TokenKind::Nat(1)),
            make_tok(TokenKind::RParen),
            make_tok(TokenKind::RParen),
        ];
        assert_eq!(max_bracket_depth(&tokens), 2);
    }
    #[test]
    fn test_max_bracket_depth_flat() {
        let tokens = vec![make_tok(TokenKind::Nat(1)), make_tok(TokenKind::Plus)];
        assert_eq!(max_bracket_depth(&tokens), 0);
    }
    #[test]
    fn test_operator_priority_min_max() {
        assert!(OperatorPriority::MIN < OperatorPriority::MAX);
    }
}
/// Produce N-grams of size `n` from a token slice.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn token_ngrams(tokens: &[Token], n: usize) -> Vec<&[Token]> {
    TokenNgramIter::new(tokens, n).collect()
}
/// Count bigrams (pairs of consecutive tokens) in a slice.
/// Returns counts keyed by (debug_repr_a, debug_repr_b) string pairs.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn count_bigrams(tokens: &[Token]) -> std::collections::HashMap<(String, String), usize> {
    let mut map = std::collections::HashMap::new();
    for w in tokens.windows(2) {
        let key = (format!("{:?}", w[0].kind), format!("{:?}", w[1].kind));
        *map.entry(key).or_insert(0) += 1;
    }
    map
}
/// Return the longest run of consecutive tokens with the same kind.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn longest_run(tokens: &[Token]) -> usize {
    let mut max = 0usize;
    let mut current = 0usize;
    let mut last_kind: Option<&TokenKind> = None;
    for tok in tokens {
        if Some(&tok.kind) == last_kind {
            current += 1;
        } else {
            current = 1;
            last_kind = Some(&tok.kind);
        }
        if current > max {
            max = current;
        }
    }
    max
}
/// Compute the vocabulary (set of distinct token kinds) in a slice.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn vocabulary(tokens: &[Token]) -> std::collections::HashSet<String> {
    tokens.iter().map(|t| format!("{:?}", t.kind)).collect()
}
/// Compute the type-token ratio (distinct kinds / total tokens).
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn type_token_ratio(tokens: &[Token]) -> f64 {
    if tokens.is_empty() {
        return 0.0;
    }
    let distinct = vocabulary(tokens).len();
    distinct as f64 / tokens.len() as f64
}
/// Tokenize frequencies: return (kind_string, count) pairs sorted by count.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn token_frequencies(tokens: &[Token]) -> Vec<(String, usize)> {
    let mut freq: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
    for tok in tokens {
        *freq.entry(format!("{:?}", tok.kind)).or_insert(0) += 1;
    }
    let mut pairs: Vec<(String, usize)> = freq.into_iter().collect();
    pairs.sort_by_key(|b| std::cmp::Reverse(b.1));
    pairs
}
/// ANSI color codes for syntax highlighting.
#[allow(dead_code)]
#[allow(missing_docs)]
pub mod ansi {
    #[allow(missing_docs)]
    pub const RESET: &str = "\x1b[0m";
    pub const BOLD: &str = "\x1b[1m";
    pub const BLUE: &str = "\x1b[34m";
    pub const CYAN: &str = "\x1b[36m";
    pub const GREEN: &str = "\x1b[32m";
    #[allow(missing_docs)]
    pub const YELLOW: &str = "\x1b[33m";
    pub const RED: &str = "\x1b[31m";
    pub const MAGENTA: &str = "\x1b[35m";
    pub const BOLD_BLUE: &str = "\x1b[1;34m";
    pub const BOLD_CYAN: &str = "\x1b[1;36m";
    #[allow(missing_docs)]
    pub const BOLD_GREEN: &str = "\x1b[1;32m";
}
/// Apply ANSI coloring to a token based on its category.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn colorize_token(kind: &TokenKind, text: &str) -> String {
    let cat = categorise(kind);
    match cat {
        TokenCategory::Keyword => format!("{}{}{}", ansi::BOLD_BLUE, text, ansi::RESET),
        TokenCategory::Identifier => text.to_string(),
        TokenCategory::Literal => format!("{}{}{}", ansi::BOLD_GREEN, text, ansi::RESET),
        TokenCategory::Operator => format!("{}{}{}", ansi::CYAN, text, ansi::RESET),
        TokenCategory::Punctuation => format!("{}{}{}", ansi::YELLOW, text, ansi::RESET),
        TokenCategory::Comment => format!("{}{}{}", ansi::GREEN, text, ansi::RESET),
        TokenCategory::Eof => String::new(),
        TokenCategory::Other => text.to_string(),
    }
}
/// Render a token slice as a colorized string.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn render_colored(tokens: &[Token], source: &str) -> String {
    let mut out = String::new();
    for tok in tokens {
        let span = &tok.span;
        let text = source.get(span.start..span.end).unwrap_or("?");
        out.push_str(&colorize_token(&tok.kind, text));
    }
    out
}
/// Reformat a token stream to a normalized string.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn reformat(tokens: &[Token], source: &str, opts: &ReformatOptions) -> String {
    let mut out = String::new();
    for (i, tok) in tokens.iter().enumerate() {
        let span = &tok.span;
        let text = source.get(span.start..span.end).unwrap_or("?");
        let is_op = is_infix_op(&tok.kind);
        let is_comma = matches!(tok.kind, TokenKind::Comma);
        let is_close = matches!(
            tok.kind,
            TokenKind::RParen | TokenKind::RBrace | TokenKind::RBracket
        );
        if i > 0 {
            let prev = &tokens[i - 1];
            let prev_is_open = matches!(
                prev.kind,
                TokenKind::LParen | TokenKind::LBrace | TokenKind::LBracket
            );
            let need_space = if is_close && opts.no_space_before_close {
                false
            } else if is_op && opts.space_before_op {
                true
            } else {
                !is_comma && !prev_is_open
            };
            if need_space {
                out.push(' ');
            }
        }
        out.push_str(text);
        if is_comma && opts.space_after_comma {}
    }
    out
}
/// Validate that a token slice represents a well-formed expression head.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn starts_with_valid_expr_head(tokens: &[Token]) -> bool {
    tokens
        .first()
        .map(|t| can_start_expr(&t.kind))
        .unwrap_or(false)
}
/// Validate that a token slice represents a well-formed declaration head.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn starts_with_valid_decl_head(tokens: &[Token]) -> bool {
    tokens
        .first()
        .map(|t| can_start_decl(&t.kind))
        .unwrap_or(false)
}
/// Count the nesting depth at each token position.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn compute_depths(tokens: &[Token]) -> Vec<i32> {
    let mut depths = Vec::with_capacity(tokens.len());
    let mut depth = 0i32;
    for tok in tokens {
        match &tok.kind {
            TokenKind::LParen | TokenKind::LBracket | TokenKind::LBrace => {
                depths.push(depth);
                depth += 1;
            }
            TokenKind::RParen | TokenKind::RBracket | TokenKind::RBrace => {
                depth -= 1;
                depths.push(depth.max(0));
            }
            _ => depths.push(depth),
        }
    }
    depths
}
/// Find the index of the matching closing bracket for an opening bracket at `open_idx`.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn find_matching_close(tokens: &[Token], open_idx: usize) -> Option<usize> {
    let open_kind = opening_bracket(&tokens.get(open_idx)?.kind)?;
    let close_kind = closing_for(open_kind);
    let mut depth = 0i32;
    for (i, tok) in tokens[open_idx..].iter().enumerate() {
        if opening_bracket(&tok.kind).is_some() {
            depth += 1;
        } else if tok.kind == close_kind {
            depth -= 1;
            if depth == 0 {
                return Some(open_idx + i);
            }
        }
    }
    None
}
/// Extract the content between matched brackets (exclusive).
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn extract_bracketed(tokens: &[Token], open_idx: usize) -> Option<&[Token]> {
    let close_idx = find_matching_close(tokens, open_idx)?;
    Some(&tokens[open_idx + 1..close_idx])
}
/// Compare two token slices for structural equality (same kinds in same order).
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn structurally_equal(a: &[Token], b: &[Token]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    a.iter()
        .zip(b.iter())
        .all(|(ta, tb)| std::mem::discriminant(&ta.kind) == std::mem::discriminant(&tb.kind))
}
/// Compute the edit distance between two token sequences (by kind).
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn token_edit_distance(a: &[Token], b: &[Token]) -> usize {
    let m = a.len();
    let n = b.len();
    let mut dp = vec![vec![0usize; n + 1]; m + 1];
    for (i, row) in dp.iter_mut().enumerate().take(m + 1) {
        row[0] = i;
    }
    for (j, cell) in dp[0].iter_mut().enumerate().take(n + 1) {
        *cell = j;
    }
    for i in 1..=m {
        for j in 1..=n {
            if a[i - 1].kind == b[j - 1].kind {
                dp[i][j] = dp[i - 1][j - 1];
            } else {
                dp[i][j] = 1 + dp[i - 1][j].min(dp[i][j - 1]).min(dp[i - 1][j - 1]);
            }
        }
    }
    dp[m][n]
}
/// Find the longest common subsequence (by kind) of two token slices.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn token_lcs_length(a: &[Token], b: &[Token]) -> usize {
    let m = a.len();
    let n = b.len();
    let mut dp = vec![vec![0usize; n + 1]; m + 1];
    for i in 1..=m {
        for j in 1..=n {
            if a[i - 1].kind == b[j - 1].kind {
                dp[i][j] = dp[i - 1][j - 1] + 1;
            } else {
                dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]);
            }
        }
    }
    dp[m][n]
}
/// Annotate a token slice with depth and category information.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn annotate_tokens(tokens: &[Token]) -> Vec<AnnotatedToken> {
    let depths = compute_depths(tokens);
    tokens
        .iter()
        .enumerate()
        .map(|(i, tok)| AnnotatedToken::new(tok.clone(), depths[i], i))
        .collect()
}
/// Compute a simple hash of a token sequence (by kind only).
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn token_hash(tokens: &[Token]) -> u64 {
    let mut hash: u64 = 14_695_981_039_346_656_037;
    for tok in tokens {
        let kind_str = format!("{:?}", tok.kind);
        for b in kind_str.bytes() {
            hash ^= b as u64;
            hash = hash.wrapping_mul(1_099_511_628_211);
        }
    }
    hash
}
/// Serialize a token slice to a compact string representation (for debugging).
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn serialize_tokens(tokens: &[Token]) -> String {
    tokens
        .iter()
        .map(|t| format!("{:?}@{}:{}", t.kind, t.span.line, t.span.column))
        .collect::<Vec<_>>()
        .join(" ")
}
/// Deserialize a debug token description (best-effort, for tests only).
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn describe_token(tok: &Token) -> String {
    format!(
        "Token({:?}, line={}, col={})",
        tok.kind, tok.span.line, tok.span.column
    )
}
/// Reconstruct source text from tokens and original source.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn reconstruct_source(tokens: &[Token], source: &str) -> String {
    let mut out = String::new();
    let mut last_end = 0usize;
    for tok in tokens {
        let start = tok.span.start;
        let end = tok.span.end;
        if start >= last_end && end <= source.len() {
            out.push_str(&source[last_end..start]);
            out.push_str(&source[start..end]);
            last_end = end;
        }
    }
    out
}
/// Check if two token spans are adjacent (no gap between them in source).
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn are_adjacent(a: &Token, b: &Token) -> bool {
    a.span.end == b.span.start
}
/// Find all pairs of adjacent tokens.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn adjacent_pairs(tokens: &[Token]) -> Vec<(&Token, &Token)> {
    tokens
        .windows(2)
        .filter(|w| are_adjacent(&w[0], &w[1]))
        .map(|w| (&w[0], &w[1]))
        .collect()
}
/// Annotate a token slice with metadata from source text.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn annotate_with_meta(tokens: &[Token], source: &str) -> Vec<TokenMeta> {
    let mut result = Vec::with_capacity(tokens.len());
    for (i, tok) in tokens.iter().enumerate() {
        let mut meta = TokenMeta::from_token(tok.clone(), source);
        if i > 0 {
            let prev = &tokens[i - 1];
            let prev_end = prev.span.end;
            let cur_start = tok.span.start;
            if prev_end < cur_start && cur_start <= source.len() {
                let gap = &source[prev_end..cur_start];
                meta.preceded_by_newline = gap.contains('\n');
                meta.preceded_by_space = gap.contains(' ') || gap.contains('\t');
            }
        }
        result.push(meta);
    }
    result
}
#[cfg(test)]
mod token_analysis_tests {
    use super::*;
    use crate::token::*;
    fn make(kind: TokenKind) -> Token {
        Token::new(kind, Span::new(0, 1, 1, 1))
    }
    fn make_at(kind: TokenKind, start: usize, end: usize) -> Token {
        Token::new(kind, Span::new(start, end, 1, start + 1))
    }
    #[test]
    fn test_token_ngrams_size2() {
        let tokens = vec![
            make(TokenKind::Nat(1)),
            make(TokenKind::Plus),
            make(TokenKind::Nat(2)),
        ];
        let grams = token_ngrams(&tokens, 2);
        assert_eq!(grams.len(), 2);
    }
    #[test]
    fn test_token_ngrams_empty() {
        let tokens: Vec<Token> = vec![];
        let grams = token_ngrams(&tokens, 2);
        assert!(grams.is_empty());
    }
    #[test]
    fn test_longest_run_same_kind() {
        let tokens = vec![
            make(TokenKind::Plus),
            make(TokenKind::Plus),
            make(TokenKind::Nat(1)),
        ];
        assert_eq!(longest_run(&tokens), 2);
    }
    #[test]
    fn test_type_token_ratio_all_distinct() {
        let tokens = vec![
            make(TokenKind::Plus),
            make(TokenKind::Minus),
            make(TokenKind::Star),
        ];
        let r = type_token_ratio(&tokens);
        assert!((r - 1.0).abs() < 1e-10);
    }
    #[test]
    fn test_type_token_ratio_empty() {
        let r = type_token_ratio(&[]);
        assert_eq!(r, 0.0);
    }
    #[test]
    fn test_token_frequencies() {
        let tokens = vec![
            make(TokenKind::Plus),
            make(TokenKind::Plus),
            make(TokenKind::Nat(1)),
        ];
        let freq = token_frequencies(&tokens);
        assert!(!freq.is_empty());
        assert_eq!(freq[0].1, 2);
    }
    #[test]
    fn test_structurally_equal_true() {
        let a = vec![make(TokenKind::Plus), make(TokenKind::Nat(1))];
        let b = vec![make(TokenKind::Plus), make(TokenKind::Nat(99))];
        assert!(structurally_equal(&a, &b));
    }
    #[test]
    fn test_structurally_equal_false_length() {
        let a = vec![make(TokenKind::Plus)];
        let b = vec![make(TokenKind::Plus), make(TokenKind::Plus)];
        assert!(!structurally_equal(&a, &b));
    }
    #[test]
    fn test_token_edit_distance_equal() {
        let tokens = vec![make(TokenKind::Plus), make(TokenKind::Nat(1))];
        assert_eq!(token_edit_distance(&tokens, &tokens), 0);
    }
    #[test]
    fn test_token_edit_distance_insert() {
        let a: Vec<Token> = vec![];
        let b = vec![make(TokenKind::Plus)];
        assert_eq!(token_edit_distance(&a, &b), 1);
    }
    #[test]
    fn test_token_lcs_length() {
        let a = vec![
            make(TokenKind::Plus),
            make(TokenKind::Nat(1)),
            make(TokenKind::Minus),
        ];
        let b = vec![make(TokenKind::Plus), make(TokenKind::Minus)];
        assert_eq!(token_lcs_length(&a, &b), 2);
    }
    #[test]
    fn test_compute_depths() {
        let tokens = vec![
            make(TokenKind::LParen),
            make(TokenKind::Nat(1)),
            make(TokenKind::RParen),
        ];
        let depths = compute_depths(&tokens);
        assert_eq!(depths.len(), 3);
        assert_eq!(depths[0], 0);
        assert_eq!(depths[1], 1);
    }
    #[test]
    fn test_find_matching_close() {
        let tokens = vec![
            make(TokenKind::LParen),
            make(TokenKind::Nat(1)),
            make(TokenKind::RParen),
        ];
        assert_eq!(find_matching_close(&tokens, 0), Some(2));
    }
    #[test]
    fn test_find_matching_close_nested() {
        let tokens = vec![
            make(TokenKind::LParen),
            make(TokenKind::LParen),
            make(TokenKind::RParen),
            make(TokenKind::RParen),
        ];
        assert_eq!(find_matching_close(&tokens, 0), Some(3));
    }
    #[test]
    fn test_extract_bracketed() {
        let tokens = vec![
            make(TokenKind::LParen),
            make(TokenKind::Nat(42)),
            make(TokenKind::RParen),
        ];
        let inner = extract_bracketed(&tokens, 0);
        assert!(inner.is_some());
        assert_eq!(inner.expect("test operation should succeed").len(), 1);
    }
    #[test]
    fn test_token_hash_deterministic() {
        let tokens = vec![make(TokenKind::Plus), make(TokenKind::Nat(1))];
        assert_eq!(token_hash(&tokens), token_hash(&tokens));
    }
    #[test]
    fn test_token_hash_different() {
        let a = vec![make(TokenKind::Plus)];
        let b = vec![make(TokenKind::Minus)];
        assert_ne!(token_hash(&a), token_hash(&b));
    }
    #[test]
    fn test_annotate_tokens() {
        let tokens = vec![
            make(TokenKind::LParen),
            make(TokenKind::Nat(1)),
            make(TokenKind::RParen),
        ];
        let ann = annotate_tokens(&tokens);
        assert_eq!(ann.len(), 3);
        assert_eq!(ann[0].depth, 0);
        assert_eq!(ann[1].depth, 1);
    }
    #[test]
    fn test_serialize_tokens() {
        let tokens = vec![make(TokenKind::Plus)];
        let s = serialize_tokens(&tokens);
        assert!(s.contains("Plus"));
    }
    #[test]
    fn test_describe_token() {
        let tok = make(TokenKind::Nat(7));
        let s = describe_token(&tok);
        assert!(s.contains("Nat"));
    }
    #[test]
    fn test_token_category_all() {
        let all = TokenCategory::all();
        assert_eq!(all.len(), 8);
    }
    #[test]
    fn test_token_category_ansi_color() {
        let col = TokenCategory::Keyword.ansi_color();
        assert!(!col.is_empty());
    }
    #[test]
    fn test_token_category_is_meaningful() {
        assert!(TokenCategory::Keyword.is_meaningful());
        assert!(!TokenCategory::Eof.is_meaningful());
    }
    #[test]
    fn test_token_meta_is_numeric() {
        let tok = make(TokenKind::Nat(5));
        let meta = TokenMeta::from_token(tok, "5");
        assert!(meta.is_numeric());
    }
    #[test]
    fn test_token_meta_is_string() {
        let tok = make(TokenKind::String("hi".to_string()));
        let meta = TokenMeta::from_token(tok, "\"hi\"");
        assert!(meta.is_string());
    }
    #[test]
    fn test_token_pattern_exact() {
        let tok = make(TokenKind::Plus);
        let pat = TokenPattern::Exact(TokenKind::Plus);
        assert!(pat.matches_single(&tok));
    }
    #[test]
    fn test_token_pattern_category() {
        let tok = make(TokenKind::Plus);
        let pat = TokenPattern::Category(TokenCategory::Operator);
        assert!(pat.matches_single(&tok));
    }
    #[test]
    fn test_token_pattern_any() {
        let tok = make(TokenKind::Comma);
        let pat = TokenPattern::Any;
        assert!(pat.matches_single(&tok));
    }
    #[test]
    fn test_token_pattern_alternatives() {
        let tok = make(TokenKind::Minus);
        let pat = TokenPattern::Alternatives(vec![
            TokenPattern::Exact(TokenKind::Plus),
            TokenPattern::Exact(TokenKind::Minus),
        ]);
        assert!(pat.matches_single(&tok));
    }
    #[test]
    fn test_token_pattern_sequence_match() {
        let tokens = vec![make(TokenKind::Plus), make(TokenKind::Nat(1))];
        let pat = TokenPattern::Sequence(vec![
            TokenPattern::Exact(TokenKind::Plus),
            TokenPattern::Category(TokenCategory::Literal),
        ]);
        assert_eq!(pat.try_match(&tokens), Some(2));
    }
    #[test]
    fn test_token_pattern_sequence_no_match() {
        let tokens = vec![make(TokenKind::Minus), make(TokenKind::Nat(1))];
        let pat = TokenPattern::Sequence(vec![TokenPattern::Exact(TokenKind::Plus)]);
        assert!(pat.try_match(&tokens).is_none());
    }
    #[test]
    fn test_token_pattern_find_all() {
        let tokens = vec![
            make(TokenKind::Plus),
            make(TokenKind::Nat(1)),
            make(TokenKind::Plus),
            make(TokenKind::Nat(2)),
        ];
        let pat = TokenPattern::Exact(TokenKind::Plus);
        let matches = pat.find_all(&tokens);
        assert_eq!(matches.len(), 2);
    }
    #[test]
    fn test_stream_inject() {
        let mut stream = TokenStream::new(vec![make(TokenKind::Nat(1)), make(TokenKind::Nat(2))]);
        stream.inject(vec![make(TokenKind::Plus)]);
        assert_eq!(stream.len(), 3);
        let first = stream.next().expect("iterator should have next element");
        assert_eq!(first.kind, TokenKind::Plus);
    }
    #[test]
    fn test_stream_peek_all() {
        let stream = TokenStream::new(vec![make(TokenKind::Plus), make(TokenKind::Nat(1))]);
        assert_eq!(stream.peek_all().len(), 2);
    }
    #[test]
    fn test_stream_peek_slice() {
        let stream = TokenStream::new(vec![
            make(TokenKind::Nat(1)),
            make(TokenKind::Nat(2)),
            make(TokenKind::Nat(3)),
        ]);
        let sl = stream.peek_slice(2);
        assert_eq!(sl.len(), 2);
    }
    #[test]
    fn test_stream_matches_sequence() {
        let stream = TokenStream::new(vec![
            make(TokenKind::LParen),
            make(TokenKind::Nat(1)),
            make(TokenKind::RParen),
        ]);
        assert!(stream.matches_sequence(&[&TokenKind::LParen, &TokenKind::Nat(1)]));
        assert!(!stream.matches_sequence(&[&TokenKind::Nat(1)]));
    }
    #[test]
    fn test_stream_consume_n() {
        let mut stream = TokenStream::new(vec![
            make(TokenKind::Nat(1)),
            make(TokenKind::Nat(2)),
            make(TokenKind::Nat(3)),
        ]);
        let consumed = stream.consume_n(2);
        assert_eq!(consumed.len(), 2);
        assert_eq!(stream.remaining(), 1);
    }
    #[test]
    fn test_stream_skip_to() {
        let mut stream = TokenStream::new(vec![
            make(TokenKind::Nat(1)),
            make(TokenKind::Plus),
            make(TokenKind::Nat(2)),
        ]);
        stream.skip_to(&TokenKind::Plus);
        assert_eq!(stream.peek().map(|t| &t.kind), Some(&TokenKind::Plus));
    }
    #[test]
    fn test_are_adjacent() {
        let a = make_at(TokenKind::Nat(1), 0, 1);
        let b = make_at(TokenKind::Plus, 1, 2);
        assert!(are_adjacent(&a, &b));
    }
    #[test]
    fn test_are_not_adjacent() {
        let a = make_at(TokenKind::Nat(1), 0, 1);
        let b = make_at(TokenKind::Plus, 3, 4);
        assert!(!are_adjacent(&a, &b));
    }
    #[test]
    fn test_adjacent_pairs() {
        let tokens = vec![
            make_at(TokenKind::Nat(1), 0, 1),
            make_at(TokenKind::Plus, 1, 2),
            make_at(TokenKind::Nat(2), 4, 5),
        ];
        let pairs = adjacent_pairs(&tokens);
        assert_eq!(pairs.len(), 1);
    }
    #[test]
    fn test_starts_with_valid_expr_head() {
        let tokens = vec![make(TokenKind::Nat(1))];
        assert!(starts_with_valid_expr_head(&tokens));
    }
    #[test]
    fn test_starts_with_valid_decl_head() {
        let tokens = vec![make(TokenKind::Theorem)];
        assert!(starts_with_valid_decl_head(&tokens));
    }
    #[test]
    fn test_vocabulary() {
        let tokens = vec![
            make(TokenKind::Plus),
            make(TokenKind::Plus),
            make(TokenKind::Nat(1)),
        ];
        let v = vocabulary(&tokens);
        assert_eq!(v.len(), 2);
    }
}
/// Returns the display name for a token kind.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn token_kind_display_name(kind_str: &str) -> &'static str {
    match kind_str {
        "Ident" => "identifier",
        "Nat" => "natural number literal",
        "String" => "string literal",
        "Eof" => "end of file",
        "LParen" => "(",
        "RParen" => ")",
        "LBrack" => "[",
        "RBrack" => "]",
        "LBrace" => "{",
        "RBrace" => "}",
        "Comma" => ",",
        "Colon" => ":",
        "ColonColon" => "::",
        "Semi" => ";",
        "Arrow" => "->",
        _ => "token",
    }
}
#[cfg(test)]
mod token_display_tests {
    use super::*;
    use crate::token::*;
    #[test]
    fn test_token_kind_display_name() {
        assert_eq!(token_kind_display_name("Ident"), "identifier");
        assert_eq!(token_kind_display_name("Arrow"), "->");
        assert_eq!(token_kind_display_name("unknown"), "token");
    }
}
/// Returns true if the token kind string represents a keyword.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn is_keyword_token(kind_str: &str) -> bool {
    matches!(
        kind_str,
        "Def"
            | "Theorem"
            | "Lemma"
            | "Fun"
            | "Let"
            | "Have"
            | "Show"
            | "Match"
            | "Do"
            | "If"
            | "Then"
            | "Else"
            | "Forall"
            | "Return"
            | "In"
            | "End"
    )
}
#[cfg(test)]
mod token_keyword_tests {
    use super::*;
    use crate::token::*;
    #[test]
    fn test_is_keyword_token() {
        assert!(is_keyword_token("Def"));
        assert!(!is_keyword_token("Ident"));
    }
}
