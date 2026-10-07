//! `tokens_cover_source` and `is_valid_lean_ident`.

use super::*;

fn raw(start: usize, end: usize) -> RawToken {
    RawToken {
        kind: "X".to_string(),
        text: String::new(),
        start,
        end,
        line: 1,
        col: 1,
    }
}

#[test]
fn no_tokens_cover_only_the_empty_source() {
    assert!(tokens_cover_source("", &[]));
    assert!(!tokens_cover_source("a", &[]));
}

#[test]
fn contiguous_tokens_from_zero_to_the_end_cover_the_source() {
    assert!(tokens_cover_source("abcd", &[raw(0, 2), raw(2, 4)]));
    assert!(tokens_cover_source("ab", &[raw(0, 2)]));
}

#[test]
fn a_gap_a_late_start_or_an_early_end_do_not_cover() {
    assert!(!tokens_cover_source("abcd", &[raw(0, 1), raw(2, 4)]));
    assert!(!tokens_cover_source("abcd", &[raw(1, 4)]));
    assert!(!tokens_cover_source("abcd", &[raw(0, 3)]));
}

#[test]
fn identifier_rules() {
    assert!(is_valid_lean_ident("x"));
    assert!(is_valid_lean_ident("_x1'"));
    assert!(is_valid_lean_ident("foo_bar"));
    assert!(!is_valid_lean_ident(""));
    assert!(!is_valid_lean_ident("1x"));
    assert!(!is_valid_lean_ident("a-b"));
    assert!(!is_valid_lean_ident("'a"));
}
