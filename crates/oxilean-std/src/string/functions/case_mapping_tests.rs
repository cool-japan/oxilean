//! `camel_to_snake`, `snake_to_camel` and `is_valid_ident` against verbatim
//! copies of the previous code, which took the first char of a case mapping
//! with `expect`.

use super::{camel_to_snake, is_valid_ident, snake_to_camel};

fn old_camel_to_snake(s: &str) -> String {
    let mut result = String::with_capacity(s.len() + 4);
    for c in s.chars() {
        if c.is_uppercase() {
            if !result.is_empty() {
                result.push('_');
            }
            result.push(c.to_lowercase().next().expect("never empty"));
        } else {
            result.push(c);
        }
    }
    result
}
fn old_snake_to_camel(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut capitalise_next = false;
    for c in s.chars() {
        if c == '_' {
            capitalise_next = true;
        } else if capitalise_next {
            result.push(c.to_uppercase().next().expect("never empty"));
            capitalise_next = false;
        } else {
            result.push(c);
        }
    }
    result
}
fn old_is_valid_ident(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    let mut chars = s.chars();
    let first = chars.next().expect("s is non-empty");
    if !first.is_alphabetic() && first != '_' {
        return false;
    }
    chars.all(|c| c.is_alphanumeric() || c == '_' || c == '\'')
}

#[test]
fn case_conversions_are_unchanged() {
    // Mappings to several chars ('İ' lowercases to "i\u{307}", 'ß' and 'ŉ'
    // uppercase to two chars) keep only their first char, as before.
    let words = [
        "",
        "a",
        "_",
        "camelCase",
        "PascalCase",
        "snake_case_word",
        "__x",
        "a_",
        "İstanbulCity",
        "straße_ß",
        "ŉ_ŉ",
        "ǅemal_ǆ",
        "Ωmega_ω",
        "x1_y2'",
        "1abc",
        "_ok",
    ];
    for w in words {
        assert_eq!(camel_to_snake(w), old_camel_to_snake(w), "{w:?}");
        assert_eq!(snake_to_camel(w), old_snake_to_camel(w), "{w:?}");
        assert_eq!(is_valid_ident(w), old_is_valid_ident(w), "{w:?}");
    }
    assert_eq!(snake_to_camel("a_ß"), "aS");
    assert_eq!(camel_to_snake("aİ"), "a_i");
}
