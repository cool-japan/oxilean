//! `split_at_kind`.

use super::*;

fn toks(kinds: &[TokenKind]) -> Vec<Token> {
    kinds
        .iter()
        .enumerate()
        .map(|(i, k)| Token::new(k.clone(), Span::new(i, i + 1, 1, i + 1)))
        .collect()
}

fn lens(groups: &[Vec<Token>]) -> Vec<usize> {
    groups.iter().map(|g| g.len()).collect()
}

#[test]
fn no_tokens_give_one_empty_group() {
    assert_eq!(lens(&split_at_kind(&[], &TokenKind::Comma)), vec![0]);
}

#[test]
fn no_separator_gives_one_group_with_everything() {
    let tokens = toks(&[TokenKind::Plus, TokenKind::Minus]);
    assert_eq!(lens(&split_at_kind(&tokens, &TokenKind::Comma)), vec![2]);
}

#[test]
fn separators_split_and_are_dropped() {
    let tokens = toks(&[
        TokenKind::Plus,
        TokenKind::Comma,
        TokenKind::Minus,
        TokenKind::Star,
        TokenKind::Comma,
        TokenKind::Slash,
    ]);
    let groups = split_at_kind(&tokens, &TokenKind::Comma);
    assert_eq!(lens(&groups), vec![1, 2, 1]);
    assert_eq!(groups[1][0].kind, TokenKind::Minus);
    assert_eq!(groups[1][1].kind, TokenKind::Star);
}

#[test]
fn leading_trailing_and_adjacent_separators_give_empty_groups() {
    let tokens = toks(&[
        TokenKind::Comma,
        TokenKind::Plus,
        TokenKind::Comma,
        TokenKind::Comma,
        TokenKind::Comma,
    ]);
    assert_eq!(
        lens(&split_at_kind(&tokens, &TokenKind::Comma)),
        vec![0, 1, 0, 0, 0]
    );
}
