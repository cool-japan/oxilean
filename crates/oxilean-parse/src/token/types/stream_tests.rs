//! `TokenStream::eat_while` and `TokenStream::expect`.

use super::*;

fn tok(kind: TokenKind, start: usize) -> Token {
    Token::new(kind, Span::new(start, start + 1, 1, start + 1))
}

fn stream(kinds: Vec<TokenKind>) -> TokenStream {
    TokenStream::new(
        kinds
            .into_iter()
            .enumerate()
            .map(|(i, k)| tok(k, i))
            .collect(),
    )
}

#[test]
fn eat_while_takes_the_leading_run_only() {
    let mut s = stream(vec![
        TokenKind::Plus,
        TokenKind::Plus,
        TokenKind::Minus,
        TokenKind::Plus,
    ]);
    let eaten = s.eat_while(|t| t.kind == TokenKind::Plus);
    assert_eq!(eaten.len(), 2);
    assert_eq!(s.position(), 2);
    assert_eq!(s.peek().map(|t| &t.kind), Some(&TokenKind::Minus));
}

#[test]
fn eat_while_on_a_failing_first_token_takes_nothing() {
    let mut s = stream(vec![TokenKind::Minus, TokenKind::Plus]);
    assert!(s.eat_while(|t| t.kind == TokenKind::Plus).is_empty());
    assert_eq!(s.position(), 0);
}

#[test]
fn eat_while_can_take_everything_and_stops_at_the_end() {
    let mut s = stream(vec![TokenKind::Plus, TokenKind::Plus]);
    assert_eq!(s.eat_while(|_| true).len(), 2);
    assert!(s.is_empty());
    assert!(s.eat_while(|_| true).is_empty());
}

#[test]
fn eat_while_calls_the_predicate_once_per_token_looked_at() {
    let mut s = stream(vec![TokenKind::Plus, TokenKind::Plus, TokenKind::Minus]);
    let mut calls = 0;
    let eaten = s.eat_while(|t| {
        calls += 1;
        t.kind == TokenKind::Plus
    });
    assert_eq!(eaten.len(), 2);
    assert_eq!(calls, 3);
}

#[test]
fn expect_consumes_a_matching_token() {
    let mut s = stream(vec![TokenKind::Plus, TokenKind::Minus]);
    match s.expect(&TokenKind::Plus) {
        Ok(t) => assert_eq!(t.kind, TokenKind::Plus),
        Err(e) => panic!("a matching token must be accepted, got {e}"),
    }
    assert_eq!(s.position(), 1);
}

#[test]
fn expect_reports_a_wrong_token_with_its_position_and_does_not_consume() {
    let mut s = stream(vec![TokenKind::Minus]);
    match s.expect(&TokenKind::Plus) {
        Err(e) => assert_eq!(e, "expected Plus, got Minus at 1:1"),
        Ok(t) => panic!("a wrong token must be rejected, got {t:?}"),
    }
    assert_eq!(s.position(), 0);
}

#[test]
fn expect_at_the_end_reports_end_of_file() {
    let mut s = stream(vec![]);
    match s.expect(&TokenKind::Plus) {
        Err(e) => assert_eq!(e, "expected Plus, got end-of-file"),
        Ok(t) => panic!("an empty stream must be rejected, got {t:?}"),
    }
}
