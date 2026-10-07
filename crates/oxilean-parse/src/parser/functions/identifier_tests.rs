//! `is_valid_identifier`.

use super::*;

#[test]
fn identifier_rules() {
    assert!(is_valid_identifier("x"));
    assert!(is_valid_identifier("_x1'"));
    assert!(is_valid_identifier("foo_bar"));
    assert!(!is_valid_identifier(""));
    assert!(!is_valid_identifier("1x"));
    assert!(!is_valid_identifier("a-b"));
    assert!(!is_valid_identifier("'a"));
}
