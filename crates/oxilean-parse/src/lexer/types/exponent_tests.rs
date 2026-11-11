//! Numeric literals with an exponent, with and without a sign.

use super::*;

fn kinds(src: &str) -> Vec<TokenKind> {
    Lexer::new(src)
        .tokenize()
        .into_iter()
        .map(|t| t.kind)
        .collect()
}

fn float_of(src: &str) -> f64 {
    match kinds(src).first() {
        Some(TokenKind::Float(v)) => *v,
        other => panic!("expected a float token for {src:?}, got {other:?}"),
    }
}

#[test]
fn integer_with_exponent() {
    assert_eq!(float_of("2e3"), 2000.0);
    assert_eq!(float_of("2E3"), 2000.0);
    assert_eq!(float_of("1e+5"), 100000.0);
    assert_eq!(float_of("1e-2"), 0.01);
}

#[test]
fn fraction_with_exponent() {
    assert_eq!(float_of("1.5e2"), 150.0);
    assert_eq!(float_of("1.5e+2"), 150.0);
    assert_eq!(float_of("1.5e-1"), 0.15);
    assert_eq!(float_of("1.5E-1"), 0.15);
}

#[test]
fn sign_is_consumed_only_right_after_the_exponent_marker() {
    let ks = kinds("1e+5 + 3");
    assert_eq!(ks.first(), Some(&TokenKind::Float(100000.0)));
    assert!(ks.contains(&TokenKind::Plus));
    assert!(ks.contains(&TokenKind::Nat(3)));
}

#[test]
fn exponent_marker_without_digits_keeps_the_existing_fallback() {
    assert_eq!(float_of("1e+"), 0.0);
    assert_eq!(float_of("1e"), 0.0);
}
