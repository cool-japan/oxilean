//! `MacroExpander::expand` when zero, one or several rules match.

use super::*;

fn tok(kind: TokenKind) -> Token {
    Token::new(kind, Span::new(0, 0, 1, 1))
}

fn hygiene() -> HygieneInfo {
    HygieneInfo::new(0, Span::new(0, 0, 1, 1))
}

fn plus_to(replacement: &str) -> MacroRule {
    MacroRule {
        pattern: vec![MacroToken::Literal(TokenKind::Plus)],
        template: vec![MacroToken::Literal(TokenKind::Ident(
            replacement.to_string(),
        ))],
    }
}

fn expander_with(rules: Vec<MacroRule>) -> MacroExpander {
    let mut expander = MacroExpander::new();
    expander.register_macro(MacroDef::new("m".to_string(), rules, hygiene()));
    expander
}

#[test]
fn no_matching_rule_is_a_pattern_mismatch() {
    let mut expander = expander_with(vec![plus_to("a")]);
    match expander.expand("m", &[tok(TokenKind::Star)]) {
        Err(err) => {
            assert_eq!(err.kind, MacroErrorKind::PatternMismatch);
            assert!(err
                .message
                .contains("no rule of macro 'm' matches the input (1 tokens)"));
        }
        Ok(_) => panic!("a mismatching input must not expand"),
    }
}

#[test]
fn a_macro_without_rules_is_a_pattern_mismatch() {
    let mut expander = expander_with(vec![]);
    match expander.expand("m", &[]) {
        Err(err) => assert_eq!(err.kind, MacroErrorKind::PatternMismatch),
        Ok(_) => panic!("a macro without rules must not expand"),
    }
}

#[test]
fn exactly_one_matching_rule_expands() {
    let mut expander = expander_with(vec![plus_to("a"), {
        MacroRule {
            pattern: vec![MacroToken::Literal(TokenKind::Minus)],
            template: vec![MacroToken::Literal(TokenKind::Ident("b".to_string()))],
        }
    }]);
    match expander.expand("m", &[tok(TokenKind::Plus)]) {
        Ok(out) => {
            assert_eq!(out.len(), 1);
            assert_eq!(out[0].kind, TokenKind::Ident("a".to_string()));
        }
        Err(err) => panic!("one matching rule must expand, got {err:?}"),
    }
}

#[test]
fn several_matching_rules_are_ambiguous() {
    let mut expander = expander_with(vec![plus_to("a"), plus_to("b"), plus_to("c")]);
    match expander.expand("m", &[tok(TokenKind::Plus)]) {
        Err(err) => {
            assert_eq!(err.kind, MacroErrorKind::AmbiguousMatch);
            assert!(err.message.contains("3 rules of macro 'm' match the input"));
        }
        Ok(_) => panic!("an ambiguous input must not expand"),
    }
}
