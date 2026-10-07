//! `TokenRange::from_tokens`.

use super::*;

fn tok(start: usize, end: usize) -> Token {
    Token::new(TokenKind::Plus, Span::new(start, end, 1, start + 1))
}

#[test]
fn an_empty_vector_has_no_range() {
    assert!(TokenRange::from_tokens(Vec::new()).is_none());
}

#[test]
fn a_single_token_spans_itself() {
    match TokenRange::from_tokens(vec![tok(3, 5)]) {
        Some(range) => {
            assert_eq!(range.len(), 1);
            assert_eq!((range.span.start, range.span.end), (3, 5));
        }
        None => panic!("a non-empty vector has a range"),
    }
}

#[test]
fn several_tokens_span_from_the_first_to_the_last() {
    match TokenRange::from_tokens(vec![tok(1, 2), tok(4, 6), tok(8, 9)]) {
        Some(range) => {
            assert_eq!(range.len(), 3);
            assert_eq!((range.span.start, range.span.end), (1, 9));
        }
        None => panic!("a non-empty vector has a range"),
    }
}
