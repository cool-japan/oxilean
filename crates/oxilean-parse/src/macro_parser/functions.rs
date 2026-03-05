//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::tokens::{Span, Token, TokenKind};

use super::types::{MacroRule, MacroSubst, MacroTemplateNodeExt, MacroToken, MacroVarExt};

/// Match a macro pattern against input tokens.
///
/// Returns a map from variable names to the tokens they matched,
/// or `None` if the pattern does not match.
pub fn match_pattern(pattern: &[MacroToken], input: &[Token]) -> Option<Vec<(String, Vec<Token>)>> {
    let mut bindings: Vec<(String, Vec<Token>)> = Vec::new();
    let mut input_pos = 0;
    for pat_tok in pattern {
        match pat_tok {
            MacroToken::Literal(expected_kind) => {
                if input_pos >= input.len() {
                    return None;
                }
                if &input[input_pos].kind != expected_kind {
                    return None;
                }
                input_pos += 1;
            }
            MacroToken::Var(name) => {
                if input_pos >= input.len() {
                    return None;
                }
                let bound = collect_var_tokens(pattern, pat_tok, input, input_pos);
                let count = bound.len();
                if count == 0 {
                    return None;
                }
                bindings.push((name.clone(), bound));
                input_pos += count;
            }
            MacroToken::Repeat(sub_pattern) => {
                if sub_pattern.is_empty() {
                    let rest: Vec<Token> = input[input_pos..].to_vec();
                    if !rest.is_empty() {
                        bindings.push(("_repeat".to_string(), rest));
                    }
                    input_pos = input.len();
                } else {
                    loop {
                        if input_pos >= input.len() {
                            break;
                        }
                        let sub_input = &input[input_pos..];
                        if let Some(sub_bindings) = match_pattern(sub_pattern, sub_input) {
                            let consumed: usize =
                                sub_bindings.iter().map(|(_, toks)| toks.len()).sum();
                            if consumed == 0 {
                                break;
                            }
                            for (k, v) in sub_bindings {
                                if let Some(existing) = bindings.iter_mut().find(|(n, _)| n == &k) {
                                    existing.1.extend(v);
                                } else {
                                    bindings.push((k, v));
                                }
                            }
                            input_pos += consumed;
                        } else {
                            break;
                        }
                    }
                }
            }
            MacroToken::Optional(sub_pattern) => {
                if !sub_pattern.is_empty() && input_pos < input.len() {
                    let sub_input = &input[input_pos..];
                    if let Some(sub_bindings) = match_pattern(sub_pattern, sub_input) {
                        let consumed: usize = sub_bindings.iter().map(|(_, toks)| toks.len()).sum();
                        for (k, v) in sub_bindings {
                            bindings.push((k, v));
                        }
                        input_pos += consumed;
                    }
                }
            }
            MacroToken::Antiquote(name) => {
                if input_pos >= input.len() {
                    return None;
                }
                bindings.push((name.clone(), vec![input[input_pos].clone()]));
                input_pos += 1;
            }
            MacroToken::Quote(_) | MacroToken::SpliceArray(_) => {}
        }
    }
    Some(bindings)
}
/// Helper: determine how many tokens a `Var` should consume.
///
/// If the next pattern element is a `Literal`, we consume tokens from `input`
/// starting at `start` until we find that literal. Otherwise we consume exactly one.
pub(super) fn collect_var_tokens(
    _pattern: &[MacroToken],
    _current_pat: &MacroToken,
    input: &[Token],
    start: usize,
) -> Vec<Token> {
    if start < input.len() {
        vec![input[start].clone()]
    } else {
        Vec::new()
    }
}
/// Expand a macro template with bindings.
///
/// Replaces variable references (`$name`) in the template with the tokens
/// bound during pattern matching.
pub fn expand_template(template: &[MacroToken], bindings: &[(String, Vec<Token>)]) -> Vec<Token> {
    let mut result = Vec::new();
    for tok in template {
        match tok {
            MacroToken::Literal(kind) => {
                result.push(Token::new(kind.clone(), dummy_span()));
            }
            MacroToken::Var(name) | MacroToken::Antiquote(name) => {
                if let Some((_, bound)) = bindings.iter().find(|(n, _)| n == name) {
                    result.extend(bound.iter().cloned());
                }
            }
            MacroToken::Repeat(sub_template) => {
                if sub_template.is_empty() {
                    continue;
                }
                let first_var = sub_template.iter().find_map(|t| match t {
                    MacroToken::Var(n) | MacroToken::Antiquote(n) => Some(n.clone()),
                    _ => None,
                });
                if let Some(var_name) = first_var {
                    if let Some((_, bound)) = bindings.iter().find(|(n, _)| n == &var_name) {
                        for tok_item in bound {
                            let iter_bindings = vec![(var_name.clone(), vec![tok_item.clone()])];
                            result.extend(expand_template(sub_template, &iter_bindings));
                        }
                    }
                }
            }
            MacroToken::Optional(sub_template) => {
                let has_any = sub_template.iter().any(|t| match t {
                    MacroToken::Var(n) | MacroToken::Antiquote(n) => {
                        bindings.iter().any(|(bn, bv)| bn == n && !bv.is_empty())
                    }
                    _ => false,
                });
                if has_any {
                    result.extend(expand_template(sub_template, bindings));
                }
            }
            MacroToken::SpliceArray(name) => {
                if let Some((_, bound)) = bindings.iter().find(|(n, _)| n == name) {
                    result.extend(bound.iter().cloned());
                }
            }
            MacroToken::Quote(inner) => {
                result.extend(expand_template(inner, bindings));
            }
        }
    }
    result
}
/// Create a dummy span for generated tokens.
pub(super) fn dummy_span() -> Span {
    Span::new(0, 0, 0, 0)
}
/// Try to match a macro rule's pattern against input tokens.
///
/// Returns the variable bindings if the pattern matches.
pub fn try_match_rule(rule: &MacroRule, input: &[Token]) -> Option<Vec<(String, Vec<Token>)>> {
    match_pattern(&rule.pattern, input)
}
/// Substitute bindings into a template to produce expanded tokens.
///
/// This is a convenience wrapper around [`expand_template`].
pub fn substitute(template: &[MacroToken], bindings: &[(String, Vec<Token>)]) -> Vec<Token> {
    expand_template(template, bindings)
}
/// Create a token with a dummy span (for testing).
#[cfg(test)]
pub(super) fn make_token(kind: TokenKind) -> Token {
    Token::new(kind, dummy_span())
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::macro_parser::*;
    fn mk(kind: TokenKind) -> Token {
        make_token(kind)
    }
    fn mk_ident(s: &str) -> Token {
        mk(TokenKind::Ident(s.to_string()))
    }
    fn mk_eof() -> Token {
        mk(TokenKind::Eof)
    }
    fn dummy_hygiene() -> HygieneInfo {
        HygieneInfo::new(0, Span::new(0, 0, 1, 1))
    }
    #[test]
    fn test_macro_token_var() {
        let token = MacroToken::Var("x".to_string());
        assert_eq!(token, MacroToken::Var("x".to_string()));
    }
    #[test]
    fn test_macro_token_literal() {
        let token = MacroToken::Literal(TokenKind::Plus);
        assert!(matches!(token, MacroToken::Literal(TokenKind::Plus)));
    }
    #[test]
    fn test_macro_token_repeat() {
        let token = MacroToken::Repeat(vec![]);
        assert!(matches!(token, MacroToken::Repeat(_)));
    }
    #[test]
    fn test_macro_token_quote() {
        let q = MacroToken::Quote(vec![MacroToken::Var("x".into())]);
        assert!(matches!(q, MacroToken::Quote(_)));
    }
    #[test]
    fn test_macro_token_antiquote() {
        let a = MacroToken::Antiquote("y".into());
        assert!(matches!(a, MacroToken::Antiquote(_)));
    }
    #[test]
    fn test_macro_token_splice_array() {
        let s = MacroToken::SpliceArray("xs".into());
        assert!(matches!(s, MacroToken::SpliceArray(_)));
    }
    #[test]
    fn test_macro_token_display() {
        assert_eq!(format!("{}", MacroToken::Var("x".into())), "$x");
        assert_eq!(format!("{}", MacroToken::Literal(TokenKind::Plus)), "+");
        assert_eq!(format!("{}", MacroToken::Antiquote("y".into())), "$y");
        assert_eq!(
            format!("{}", MacroToken::SpliceArray("xs".into())),
            "$[xs]*"
        );
    }
    #[test]
    fn test_macro_parser_create() {
        let tokens = vec![];
        let parser = MacroParser::new(tokens);
        assert_eq!(parser.pos, 0);
    }
    #[test]
    fn test_macro_parser_parse_simple_rule() {
        let tokens = vec![
            mk_ident("$x"),
            mk(TokenKind::Plus),
            mk_ident("$y"),
            mk(TokenKind::Arrow),
            mk_ident("add"),
            mk_ident("$x"),
            mk_ident("$y"),
            mk_eof(),
        ];
        let mut parser = MacroParser::new(tokens);
        let rule = parser.parse_rule().expect("test operation should succeed");
        assert_eq!(rule.pattern.len(), 3);
        assert_eq!(rule.template.len(), 3);
    }
    #[test]
    fn test_macro_parser_parse_multiple_rules() {
        let tokens = vec![
            mk_ident("$x"),
            mk(TokenKind::Arrow),
            mk_ident("foo"),
            mk_ident("$x"),
            mk(TokenKind::Bar),
            mk_ident("$y"),
            mk(TokenKind::Plus),
            mk_ident("$z"),
            mk(TokenKind::Arrow),
            mk_ident("bar"),
            mk_ident("$y"),
            mk_ident("$z"),
            mk_eof(),
        ];
        let mut parser = MacroParser::new(tokens);
        let rules = parser.parse_rules().expect("test operation should succeed");
        assert_eq!(rules.len(), 2);
    }
    #[test]
    fn test_hygiene_info() {
        let h = HygieneInfo::new(42, Span::new(10, 20, 3, 5));
        assert_eq!(h.scope_id, 42);
        assert_eq!(h.def_site.start, 10);
    }
    #[test]
    fn test_macro_def_new() {
        let def = MacroDef::new("myMacro".into(), vec![], dummy_hygiene());
        assert_eq!(def.name, "myMacro");
        assert_eq!(def.rule_count(), 0);
        assert!(def.doc.is_none());
    }
    #[test]
    fn test_macro_def_with_doc() {
        let def = MacroDef::new("myMacro".into(), vec![], dummy_hygiene())
            .with_doc("A test macro".into());
        assert_eq!(def.doc.as_deref(), Some("A test macro"));
    }
    #[test]
    fn test_syntax_kind_display() {
        assert_eq!(format!("{}", SyntaxKind::Term), "term");
        assert_eq!(format!("{}", SyntaxKind::Command), "command");
        assert_eq!(format!("{}", SyntaxKind::Tactic), "tactic");
        assert_eq!(format!("{}", SyntaxKind::Level), "level");
        assert_eq!(format!("{}", SyntaxKind::Attr), "attr");
    }
    #[test]
    fn test_syntax_item_display() {
        let item = SyntaxItem::Token(TokenKind::Plus);
        assert_eq!(format!("{}", item), "+");
        let cat = SyntaxItem::Category("term".into());
        assert_eq!(format!("{}", cat), "term");
        let opt = SyntaxItem::Optional(Box::new(SyntaxItem::Category("ident".into())));
        assert_eq!(format!("{}", opt), "(ident)?");
        let many = SyntaxItem::Many(Box::new(SyntaxItem::Token(TokenKind::Comma)));
        assert_eq!(format!("{}", many), "(,)*");
        let group = SyntaxItem::Group(vec![
            SyntaxItem::Token(TokenKind::LParen),
            SyntaxItem::Category("term".into()),
            SyntaxItem::Token(TokenKind::RParen),
        ]);
        assert_eq!(format!("{}", group), "(( term ))");
    }
    #[test]
    fn test_syntax_def_new() {
        let def = SyntaxDef::new(
            "myIf".into(),
            SyntaxKind::Term,
            vec![
                SyntaxItem::Token(TokenKind::If),
                SyntaxItem::Category("term".into()),
                SyntaxItem::Token(TokenKind::Then),
                SyntaxItem::Category("term".into()),
                SyntaxItem::Token(TokenKind::Else),
                SyntaxItem::Category("term".into()),
            ],
        );
        assert_eq!(def.name, "myIf");
        assert_eq!(def.kind, SyntaxKind::Term);
        assert_eq!(def.item_count(), 6);
    }
    #[test]
    fn test_macro_error_display() {
        let err = MacroError::new(MacroErrorKind::UnknownMacro, "not found".into());
        let msg = format!("{}", err);
        assert!(msg.contains("unknown macro"));
        assert!(msg.contains("not found"));
    }
    #[test]
    fn test_macro_error_with_span() {
        let err = MacroError::new(MacroErrorKind::PatternMismatch, "oops".into())
            .with_span(Span::new(5, 10, 2, 3));
        assert!(err.span.is_some());
        let sp = err.span.expect("span should be present");
        assert_eq!(sp.start, 5);
    }
    #[test]
    fn test_macro_error_kind_display() {
        assert_eq!(format!("{}", MacroErrorKind::UnknownMacro), "unknown macro");
        assert_eq!(
            format!("{}", MacroErrorKind::PatternMismatch),
            "pattern mismatch"
        );
        assert_eq!(
            format!("{}", MacroErrorKind::HygieneViolation),
            "hygiene violation"
        );
        assert_eq!(
            format!("{}", MacroErrorKind::AmbiguousMatch),
            "ambiguous match"
        );
        assert_eq!(
            format!("{}", MacroErrorKind::ExpansionError),
            "expansion error"
        );
    }
    #[test]
    fn test_match_pattern_empty() {
        let result = match_pattern(&[], &[]);
        assert!(result.is_some());
        assert!(result.expect("test operation should succeed").is_empty());
    }
    #[test]
    fn test_match_pattern_literal_match() {
        let pattern = vec![MacroToken::Literal(TokenKind::Plus)];
        let input = vec![mk(TokenKind::Plus)];
        let result = match_pattern(&pattern, &input);
        assert!(result.is_some());
    }
    #[test]
    fn test_match_pattern_literal_mismatch() {
        let pattern = vec![MacroToken::Literal(TokenKind::Plus)];
        let input = vec![mk(TokenKind::Minus)];
        let result = match_pattern(&pattern, &input);
        assert!(result.is_none());
    }
    #[test]
    fn test_match_pattern_var_binding() {
        let pattern = vec![
            MacroToken::Var("x".into()),
            MacroToken::Literal(TokenKind::Plus),
            MacroToken::Var("y".into()),
        ];
        let input = vec![mk_ident("a"), mk(TokenKind::Plus), mk_ident("b")];
        let result = match_pattern(&pattern, &input);
        assert!(result.is_some());
        let bindings = result.expect("test operation should succeed");
        assert_eq!(bindings.len(), 2);
        assert_eq!(bindings[0].0, "x");
        assert_eq!(bindings[1].0, "y");
    }
    #[test]
    fn test_match_pattern_too_short_input() {
        let pattern = vec![
            MacroToken::Var("x".into()),
            MacroToken::Literal(TokenKind::Plus),
        ];
        let input = vec![mk_ident("a")];
        let result = match_pattern(&pattern, &input);
        assert!(result.is_none());
    }
    #[test]
    fn test_match_pattern_optional_present() {
        let pattern = vec![
            MacroToken::Var("x".into()),
            MacroToken::Optional(vec![MacroToken::Literal(TokenKind::Plus)]),
        ];
        let input = vec![mk_ident("a"), mk(TokenKind::Plus)];
        let result = match_pattern(&pattern, &input);
        assert!(result.is_some());
    }
    #[test]
    fn test_match_pattern_optional_absent() {
        let pattern = vec![
            MacroToken::Var("x".into()),
            MacroToken::Optional(vec![MacroToken::Literal(TokenKind::Plus)]),
        ];
        let input = vec![mk_ident("a")];
        let result = match_pattern(&pattern, &input);
        assert!(result.is_some());
    }
    #[test]
    fn test_expand_template_empty() {
        let result = expand_template(&[], &[]);
        assert!(result.is_empty());
    }
    #[test]
    fn test_expand_template_literal() {
        let template = vec![
            MacroToken::Literal(TokenKind::LParen),
            MacroToken::Literal(TokenKind::RParen),
        ];
        let result = expand_template(&template, &[]);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].kind, TokenKind::LParen);
        assert_eq!(result[1].kind, TokenKind::RParen);
    }
    #[test]
    fn test_expand_template_var_substitution() {
        let template = vec![
            MacroToken::Literal(TokenKind::Ident("add".into())),
            MacroToken::Var("x".into()),
            MacroToken::Var("y".into()),
        ];
        let bindings = vec![
            ("x".into(), vec![mk_ident("a")]),
            ("y".into(), vec![mk_ident("b")]),
        ];
        let result = expand_template(&template, &bindings);
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].kind, TokenKind::Ident("add".into()));
        assert_eq!(result[1].kind, TokenKind::Ident("a".into()));
        assert_eq!(result[2].kind, TokenKind::Ident("b".into()));
    }
    #[test]
    fn test_expand_template_missing_var() {
        let template = vec![MacroToken::Var("missing".into())];
        let result = expand_template(&template, &[]);
        assert!(result.is_empty());
    }
    #[test]
    fn test_expand_template_splice_array() {
        let template = vec![
            MacroToken::Literal(TokenKind::LBracket),
            MacroToken::SpliceArray("xs".into()),
            MacroToken::Literal(TokenKind::RBracket),
        ];
        let bindings = vec![(
            "xs".into(),
            vec![mk_ident("a"), mk(TokenKind::Comma), mk_ident("b")],
        )];
        let result = expand_template(&template, &bindings);
        assert_eq!(result.len(), 5);
    }
    #[test]
    fn test_expander_new() {
        let exp = MacroExpander::new();
        assert_eq!(exp.macro_count(), 0);
    }
    #[test]
    fn test_expander_default() {
        let exp = MacroExpander::default();
        assert_eq!(exp.macro_count(), 0);
    }
    #[test]
    fn test_expander_register_and_lookup() {
        let mut exp = MacroExpander::new();
        let def = MacroDef::new("test_mac".into(), vec![], dummy_hygiene());
        exp.register_macro(def);
        assert!(exp.has_macro("test_mac"));
        assert!(!exp.has_macro("other"));
        assert_eq!(exp.macro_count(), 1);
    }
    #[test]
    fn test_expander_unregister() {
        let mut exp = MacroExpander::new();
        exp.register_macro(MacroDef::new("test_mac".into(), vec![], dummy_hygiene()));
        let removed = exp.unregister_macro("test_mac");
        assert!(removed.is_some());
        assert!(!exp.has_macro("test_mac"));
    }
    #[test]
    fn test_expander_fresh_scope() {
        let mut exp = MacroExpander::new();
        let s1 = exp.fresh_scope();
        let s2 = exp.fresh_scope();
        assert_ne!(s1, s2);
        assert_eq!(s2, s1 + 1);
    }
    #[test]
    fn test_expander_unknown_macro() {
        let mut exp = MacroExpander::new();
        let result = exp.expand("nonexistent", &[]);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.kind, MacroErrorKind::UnknownMacro);
    }
    #[test]
    fn test_expander_pattern_mismatch() {
        let mut exp = MacroExpander::new();
        let rule = MacroRule {
            pattern: vec![MacroToken::Literal(TokenKind::Plus)],
            template: vec![MacroToken::Literal(TokenKind::Minus)],
        };
        exp.register_macro(MacroDef::new("myMac".into(), vec![rule], dummy_hygiene()));
        let result = exp.expand("myMac", &[mk(TokenKind::Star)]);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind, MacroErrorKind::PatternMismatch);
    }
    #[test]
    fn test_expander_successful_expansion() {
        let mut exp = MacroExpander::new();
        let rule = MacroRule {
            pattern: vec![
                MacroToken::Var("x".into()),
                MacroToken::Literal(TokenKind::Plus),
                MacroToken::Var("y".into()),
            ],
            template: vec![
                MacroToken::Literal(TokenKind::Ident("add".into())),
                MacroToken::Var("x".into()),
                MacroToken::Var("y".into()),
            ],
        };
        exp.register_macro(MacroDef::new(
            "addMacro".into(),
            vec![rule],
            dummy_hygiene(),
        ));
        let input = vec![mk_ident("a"), mk(TokenKind::Plus), mk_ident("b")];
        let result = exp
            .expand("addMacro", &input)
            .expect("test operation should succeed");
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].kind, TokenKind::Ident("add".into()));
        assert_eq!(result[1].kind, TokenKind::Ident("a".into()));
        assert_eq!(result[2].kind, TokenKind::Ident("b".into()));
    }
    #[test]
    fn test_expander_macro_names() {
        let mut exp = MacroExpander::new();
        exp.register_macro(MacroDef::new("beta".into(), vec![], dummy_hygiene()));
        exp.register_macro(MacroDef::new("alpha".into(), vec![], dummy_hygiene()));
        let names = exp.macro_names();
        assert_eq!(names, vec!["alpha", "beta"]);
    }
    #[test]
    fn test_expander_syntax_defs() {
        let mut exp = MacroExpander::new();
        exp.register_syntax(SyntaxDef::new(
            "myIf".into(),
            SyntaxKind::Term,
            vec![SyntaxItem::Token(TokenKind::If)],
        ));
        exp.register_syntax(SyntaxDef::new(
            "myTac".into(),
            SyntaxKind::Tactic,
            vec![SyntaxItem::Category("tactic".into())],
        ));
        let term_defs = exp.syntax_defs_for(&SyntaxKind::Term);
        assert_eq!(term_defs.len(), 1);
        assert_eq!(term_defs[0].name, "myIf");
        let tactic_defs = exp.syntax_defs_for(&SyntaxKind::Tactic);
        assert_eq!(tactic_defs.len(), 1);
    }
    #[test]
    fn test_expander_max_depth() {
        let mut exp = MacroExpander::new();
        exp.set_max_depth(5);
        assert_eq!(exp.max_depth, 5);
    }
    #[test]
    fn test_parse_repeat_group_star() {
        let tokens = vec![
            mk(TokenKind::Ident("$".to_string())),
            mk(TokenKind::LParen),
            mk(TokenKind::Ident("$x".to_string())),
            mk(TokenKind::RParen),
            mk(TokenKind::Star),
            mk(TokenKind::Eof),
        ];
        let mut parser = MacroParser::new(tokens);
        let tok = parser
            .parse_macro_token()
            .expect("test operation should succeed");
        assert!(matches!(tok, MacroToken::Repeat(ref inner) if inner.len() == 1));
        if let MacroToken::Repeat(inner) = tok {
            assert!(matches!(inner[0], MacroToken::Var(ref n) if n == "x"));
        }
    }
    #[test]
    fn test_parse_repeat_group_question() {
        let tokens = vec![
            mk(TokenKind::Ident("$".to_string())),
            mk(TokenKind::LParen),
            mk(TokenKind::Ident("$x".to_string())),
            mk(TokenKind::RParen),
            mk(TokenKind::Question),
            mk(TokenKind::Eof),
        ];
        let mut parser = MacroParser::new(tokens);
        let tok = parser
            .parse_macro_token()
            .expect("test operation should succeed");
        assert!(matches!(tok, MacroToken::Optional(ref inner) if inner.len() == 1));
    }
    #[test]
    fn test_parse_repeat_group_with_separator() {
        let tokens = vec![
            mk(TokenKind::Ident("$".to_string())),
            mk(TokenKind::LParen),
            mk(TokenKind::Ident("$x".to_string())),
            mk(TokenKind::Comma),
            mk(TokenKind::RParen),
            mk(TokenKind::Star),
            mk(TokenKind::Eof),
        ];
        let mut parser = MacroParser::new(tokens);
        let tok = parser
            .parse_macro_token()
            .expect("test operation should succeed");
        if let MacroToken::Repeat(inner) = tok {
            assert_eq!(inner.len(), 2);
            assert!(matches!(inner[0], MacroToken::Var(_)));
            assert!(matches!(inner[1], MacroToken::Literal(TokenKind::Comma)));
        } else {
            panic!("expected Repeat");
        }
    }
    #[test]
    fn test_match_pattern_with_repeat() {
        let pattern = vec![MacroToken::Repeat(vec![MacroToken::Var("x".into())])];
        let input = vec![mk_ident("a"), mk_ident("b"), mk_ident("c")];
        let result = match_pattern(&pattern, &input);
        assert!(result.is_some());
        let bindings = result.expect("test operation should succeed");
        assert!(bindings.iter().any(|(k, _)| k == "x"));
    }
    #[test]
    fn test_match_pattern_with_repeat_empty() {
        let pattern = vec![MacroToken::Repeat(vec![MacroToken::Var("x".into())])];
        let input = vec![];
        let result = match_pattern(&pattern, &input);
        assert!(result.is_some());
    }
    #[test]
    fn test_try_match_rule_success() {
        let rule = MacroRule {
            pattern: vec![MacroToken::Var("x".into())],
            template: vec![MacroToken::Var("x".into())],
        };
        let input = vec![mk_ident("hello")];
        let result = try_match_rule(&rule, &input);
        assert!(result.is_some());
    }
    #[test]
    fn test_substitute_identity() {
        let template = vec![MacroToken::Var("x".into())];
        let bindings = vec![("x".into(), vec![mk_ident("hello")])];
        let result = substitute(&template, &bindings);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].kind, TokenKind::Ident("hello".into()));
    }
}
/// Parse a simple macro template string into nodes.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn parse_simple_template_ext(s: &str) -> Vec<MacroTemplateNodeExt> {
    s.split_whitespace()
        .map(|tok| {
            if let Some(name) = tok.strip_prefix('$') {
                MacroTemplateNodeExt::Var(MacroVarExt::simple(name))
            } else {
                MacroTemplateNodeExt::Literal(tok.to_string())
            }
        })
        .collect()
}
/// Expand a simple macro template with variable substitutions.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn expand_template_ext(
    nodes: &[MacroTemplateNodeExt],
    env: &std::collections::HashMap<String, String>,
) -> String {
    let mut result = Vec::new();
    for node in nodes {
        match node {
            MacroTemplateNodeExt::Literal(s) => result.push(s.clone()),
            MacroTemplateNodeExt::Var(v) => {
                if let Some(val) = env.get(&v.name) {
                    result.push(val.clone());
                } else {
                    result.push(format!("${}?", v.name));
                }
            }
            MacroTemplateNodeExt::Rep { .. } => result.push("(...)".to_string()),
            MacroTemplateNodeExt::Group(inner) => result.push(expand_template_ext(inner, env)),
        }
    }
    result.join(" ")
}
#[cfg(test)]
mod macro_parser_ext_tests {
    use super::*;
    use crate::macro_parser::*;
    #[test]
    fn test_macro_var() {
        let v = MacroVarExt::simple("x");
        assert_eq!(v.name, "x");
        assert!(!v.is_rep);
        let rv = MacroVarExt::rep("xs");
        assert!(rv.is_rep);
    }
    #[test]
    fn test_parse_simple_template() {
        let nodes = parse_simple_template_ext("if $cond then $body else $alt");
        assert_eq!(nodes.len(), 6);
    }
    #[test]
    fn test_expand_template() {
        let nodes = parse_simple_template_ext("add $x $y");
        let mut env = std::collections::HashMap::new();
        env.insert("x".to_string(), "1".to_string());
        env.insert("y".to_string(), "2".to_string());
        let result = expand_template_ext(&nodes, &env);
        assert_eq!(result, "add 1 2");
    }
    #[test]
    fn test_macro_definition_expand() {
        let def = MacroDefinitionExt::new("myMacro", vec!["a", "b"], "$a + $b");
        let result = def.expand(&["x", "y"]);
        assert_eq!(result, "x + y");
    }
    #[test]
    fn test_macro_definition_wrong_arity() {
        let def = MacroDefinitionExt::new("myMacro", vec!["a"], "$a");
        let result = def.expand(&["x", "y"]);
        assert!(result.contains("error"));
    }
    #[test]
    fn test_macro_environment() {
        let mut env = MacroEnvironmentExt::new();
        env.define(MacroDefinitionExt::new("add", vec!["a", "b"], "$a + $b"));
        assert_eq!(env.len(), 1);
        let result = env
            .expand_call("add", &["1", "2"])
            .expect("test operation should succeed");
        assert_eq!(result, "1 + 2");
        assert_eq!(env.expand_call("unknown", &[]), None);
    }
    #[test]
    fn test_macro_expansion_trace() {
        let mut trace = MacroExpansionTraceExt::new();
        trace.record("step 1");
        trace.record("step 2");
        let out = trace.format();
        assert!(out.contains("0: step 1"));
        assert!(out.contains("1: step 2"));
    }
    #[test]
    fn test_macro_call_site() {
        let site = MacroCallSiteExt::new("myMacro", vec!["a", "b"], 10);
        assert_eq!(site.macro_name, "myMacro");
        assert_eq!(site.args.len(), 2);
        assert_eq!(site.offset, 10);
    }
}
#[cfg(test)]
mod macro_parser_ext2_tests {
    use super::*;
    use crate::macro_parser::*;
    #[test]
    fn test_hygiene_context() {
        let ctx = HygieneContext::new(42);
        let nested = ctx.nested();
        assert_eq!(nested.depth, 1);
        let name = nested.hygienic_name("x");
        assert!(name.starts_with("x__42"));
    }
    #[test]
    fn test_macro_expansion_error() {
        let err = MacroExpansionError::new("myMacro", "arity mismatch", 2);
        let formatted = err.format();
        assert!(formatted.contains("myMacro"));
        assert!(formatted.contains("arity mismatch"));
        assert!(formatted.contains("depth 2"));
    }
    #[test]
    fn test_macro_expansion_result() {
        let ok = MacroExpansionResult::Ok("result".to_string());
        assert!(ok.is_ok());
        let ok2 = MacroExpansionResult::Ok("value".to_string());
        match ok2 {
            MacroExpansionResult::Ok(v) => assert_eq!(v, "value"),
            _ => panic!("expected Ok variant"),
        };
        let err = MacroExpansionResult::Err(MacroExpansionError::new("m", "e", 0));
        assert!(!err.is_ok());
        assert_eq!(err.unwrap_or("default"), "default");
    }
    #[test]
    fn test_macro_stats() {
        let mut stats = MacroStats::new();
        stats.record_success(3);
        stats.record_success(5);
        stats.record_error();
        assert_eq!(stats.expansions, 2);
        assert_eq!(stats.errors, 1);
        assert_eq!(stats.max_depth, 5);
    }
}
#[cfg(test)]
mod macro_library_tests {
    use super::*;
    use crate::macro_parser::*;
    #[test]
    fn test_macro_matcher() {
        let m = MacroMatcher::from_str("if $cond then $body");
        assert_eq!(m.hole_count(), 2);
        assert_eq!(m.literal_count(), 2);
    }
    #[test]
    fn test_macro_library() {
        let mut lib = MacroLibrary::new();
        lib.add_to_group("logic", MacroDefinitionExt::new("myMacro", vec!["x"], "$x"));
        assert_eq!(lib.total_macros(), 1);
        assert_eq!(lib.group("logic").len(), 1);
        assert_eq!(lib.group("missing").len(), 0);
    }
}
/// Apply a list of substitutions to a template string.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn apply_substitutions(template: &str, substs: &[MacroSubst]) -> String {
    let mut result = template.to_string();
    for s in substs {
        let placeholder = format!("${}", s.var);
        result = result.replace(&placeholder, &s.value);
    }
    result
}
#[cfg(test)]
mod macro_final_tests {
    use super::*;
    use crate::macro_parser::*;
    #[test]
    fn test_depth_limited_expander() {
        let mut exp = DepthLimitedExpander::new(3);
        assert!(exp.try_expand());
        assert!(exp.try_expand());
        assert!(exp.try_expand());
        assert!(!exp.try_expand());
        exp.exit();
        assert!(exp.try_expand());
    }
    #[test]
    fn test_apply_substitutions() {
        let substs = vec![MacroSubst::new("x", "1"), MacroSubst::new("y", "2")];
        let result = apply_substitutions("$x + $y", &substs);
        assert_eq!(result, "1 + 2");
    }
}
/// Strips macro invocations from a source string (very simplified).
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn strip_macro_invocations(src: &str) -> String {
    src.lines()
        .map(|line| {
            if line.contains('!') {
                line.split('!')
                    .next()
                    .unwrap_or(line)
                    .trim_end()
                    .to_string()
            } else {
                line.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}
/// Count macro invocations in a source string.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn count_macro_invocations(src: &str) -> usize {
    src.matches('!').count()
}
#[cfg(test)]
mod macro_pad {
    use super::*;
    use crate::macro_parser::*;
    #[test]
    fn test_count_macro_invocations() {
        assert_eq!(count_macro_invocations("dbg!(x) + dbg!(y)"), 2);
        assert_eq!(count_macro_invocations("no macros here"), 0);
    }
    #[test]
    fn test_macro_signature() {
        let sig = MacroSignature::new("myMacro", 3);
        assert_eq!(sig.format(), "myMacro/3");
    }
}
#[cfg(test)]
mod macro_pad2 {
    use super::*;
    use crate::macro_parser::*;
    #[test]
    fn test_hygiene_context() {
        let mut ctx = HygieneContextExt2::new("_x");
        assert_eq!(ctx.fresh(), "_x0");
        assert_eq!(ctx.fresh(), "_x1");
        assert_eq!(ctx.generated_count(), 2);
    }
    #[test]
    fn test_macro_expansion_error() {
        let e = MacroExpansionErrorExt2::new("myMacro", "arity mismatch", 3);
        assert!(e.format().contains("myMacro"));
        assert!(e.format().contains("depth 3"));
    }
    #[test]
    fn test_macro_expansion_result() {
        let r = MacroExpansionResultExt2::Success("x + 1".to_string());
        assert!(r.is_success());
        assert_eq!(r.as_str(), Some("x + 1"));
        let e = MacroExpansionResultExt2::NoMatch;
        assert!(!e.is_success());
    }
    #[test]
    fn test_macro_stats() {
        let mut s = MacroStatsExt2::default();
        s.record_success(3);
        s.record_failure();
        assert_eq!(s.attempts, 2);
        assert!((s.success_rate() - 0.5).abs() < 1e-9);
    }
}
