//! `is_pascal_case` / `is_camel_case` on the empty name and on the first
//! character of a name.

use super::*;

#[test]
fn empty_name_is_accepted_by_both_case_checks() {
    assert!(is_pascal_case(""));
    assert!(is_camel_case(""));
}

#[test]
fn first_character_decides_pascal_and_camel_case() {
    assert!(is_pascal_case("Foo"));
    assert!(!is_pascal_case("foo"));
    assert!(!is_pascal_case("Foo_bar"));
    assert!(is_camel_case("fooBar"));
    assert!(!is_camel_case("FooBar"));
    assert!(!is_camel_case("foo_bar"));
}

#[test]
fn single_character_names() {
    assert!(is_pascal_case("A"));
    assert!(!is_pascal_case("a"));
    assert!(is_camel_case("a"));
    assert!(!is_camel_case("A"));
}
