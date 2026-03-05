//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tactic_parser::*;
    use crate::tokens::{Span, Token, TokenKind};
    fn make_token(kind: TokenKind) -> Token {
        Token {
            kind,
            span: Span::new(0, 0, 0, 0),
        }
    }
    fn ident(s: &str) -> Token {
        make_token(TokenKind::Ident(s.to_string()))
    }
    #[test]
    fn test_parse_basic_tactic() {
        let tokens = vec![make_token(TokenKind::Ident("intro".to_string()))];
        let mut parser = TacticParser::new(&tokens);
        let result = parser.parse().expect("parsing should succeed");
        assert_eq!(result, TacticExpr::Intro(Vec::new()));
    }
    #[test]
    fn test_parse_tactic_with_args() {
        let tokens = vec![
            make_token(TokenKind::Ident("custom".to_string())),
            make_token(TokenKind::LParen),
            make_token(TokenKind::Ident("foo".to_string())),
            make_token(TokenKind::RParen),
        ];
        let mut parser = TacticParser::new(&tokens);
        let result = parser.parse().expect("parsing should succeed");
        assert!(matches!(result, TacticExpr::WithArgs(..)));
    }
    #[test]
    fn test_parse_repeat() {
        let tokens = vec![
            make_token(TokenKind::Ident("repeat".to_string())),
            make_token(TokenKind::Ident("assumption".to_string())),
        ];
        let mut parser = TacticParser::new(&tokens);
        let result = parser.parse().expect("parsing should succeed");
        match result {
            TacticExpr::Repeat(inner) => {
                assert_eq!(*inner, TacticExpr::Assumption);
            }
            _ => panic!("expected Repeat"),
        }
    }
    #[test]
    fn test_parse_try() {
        let tokens = vec![
            make_token(TokenKind::Ident("try".to_string())),
            make_token(TokenKind::Ident("trivial".to_string())),
        ];
        let mut parser = TacticParser::new(&tokens);
        let result = parser.parse().expect("parsing should succeed");
        match result {
            TacticExpr::Try(inner) => {
                assert_eq!(*inner, TacticExpr::Trivial);
            }
            _ => panic!("expected Try"),
        }
    }
    #[test]
    fn test_parse_seq() {
        let tokens = vec![
            ident("omega"),
            make_token(TokenKind::Semicolon),
            ident("ring"),
        ];
        let mut parser = TacticParser::new(&tokens);
        let result = parser.parse().expect("parsing should succeed");
        match result {
            TacticExpr::Seq(left, right) => {
                assert_eq!(*left, TacticExpr::Omega);
                assert_eq!(*right, TacticExpr::Ring);
            }
            _ => panic!("expected Seq"),
        }
    }
    #[test]
    fn test_parse_intro_with_names() {
        let tokens = vec![ident("intro"), ident("x"), ident("y"), ident("z")];
        let mut parser = TacticParser::new(&tokens);
        let result = parser.parse().expect("parsing should succeed");
        assert_eq!(
            result,
            TacticExpr::Intro(vec!["x".to_string(), "y".to_string(), "z".to_string()])
        );
    }
    #[test]
    fn test_parse_intro_no_names() {
        let tokens = vec![ident("intro")];
        let mut parser = TacticParser::new(&tokens);
        let result = parser.parse().expect("parsing should succeed");
        assert_eq!(result, TacticExpr::Intro(vec![]));
    }
    #[test]
    fn test_parse_apply() {
        let tokens = vec![ident("apply"), ident("Nat.add_comm")];
        let mut parser = TacticParser::new(&tokens);
        let result = parser.parse().expect("parsing should succeed");
        assert_eq!(result, TacticExpr::Apply("Nat.add_comm".to_string()));
    }
    #[test]
    fn test_parse_exact() {
        let tokens = vec![ident("exact"), ident("rfl")];
        let mut parser = TacticParser::new(&tokens);
        let result = parser.parse().expect("parsing should succeed");
        assert_eq!(result, TacticExpr::Exact("rfl".to_string()));
    }
    #[test]
    fn test_parse_rewrite_single() {
        let tokens = vec![
            ident("rewrite"),
            make_token(TokenKind::LBracket),
            ident("add_comm"),
            make_token(TokenKind::RBracket),
        ];
        let mut parser = TacticParser::new(&tokens);
        let result = parser.parse().expect("parsing should succeed");
        match result {
            TacticExpr::Rewrite(rules) => {
                assert_eq!(rules.len(), 1);
                assert_eq!(rules[0].lemma, "add_comm");
                assert!(!rules[0].reverse);
            }
            _ => panic!("expected Rewrite"),
        }
    }
    #[test]
    fn test_parse_rewrite_reverse() {
        let tokens = vec![
            ident("rw"),
            make_token(TokenKind::LBracket),
            ident("<-"),
            ident("mul_comm"),
            make_token(TokenKind::RBracket),
        ];
        let mut parser = TacticParser::new(&tokens);
        let result = parser.parse().expect("parsing should succeed");
        match result {
            TacticExpr::Rewrite(rules) => {
                assert_eq!(rules.len(), 1);
                assert_eq!(rules[0].lemma, "mul_comm");
                assert!(rules[0].reverse);
            }
            _ => panic!("expected Rewrite"),
        }
    }
    #[test]
    fn test_parse_rewrite_multiple() {
        let tokens = vec![
            ident("rw"),
            make_token(TokenKind::LBracket),
            ident("add_comm"),
            make_token(TokenKind::Comma),
            ident("<-"),
            ident("mul_assoc"),
            make_token(TokenKind::RBracket),
        ];
        let mut parser = TacticParser::new(&tokens);
        let result = parser.parse().expect("parsing should succeed");
        match result {
            TacticExpr::Rewrite(rules) => {
                assert_eq!(rules.len(), 2);
                assert_eq!(rules[0].lemma, "add_comm");
                assert!(!rules[0].reverse);
                assert_eq!(rules[1].lemma, "mul_assoc");
                assert!(rules[1].reverse);
            }
            _ => panic!("expected Rewrite"),
        }
    }
    #[test]
    fn test_parse_simp_basic() {
        let tokens = vec![ident("simp")];
        let mut parser = TacticParser::new(&tokens);
        let result = parser.parse().expect("parsing should succeed");
        assert_eq!(
            result,
            TacticExpr::Simp(SimpArgs {
                only: false,
                lemmas: vec![],
                config: vec![],
            })
        );
    }
    #[test]
    fn test_parse_simp_only_with_lemmas() {
        let tokens = vec![
            ident("simp"),
            ident("only"),
            make_token(TokenKind::LBracket),
            ident("add_zero"),
            make_token(TokenKind::Comma),
            ident("mul_one"),
            make_token(TokenKind::RBracket),
        ];
        let mut parser = TacticParser::new(&tokens);
        let result = parser.parse().expect("parsing should succeed");
        match result {
            TacticExpr::Simp(args) => {
                assert!(args.only);
                assert_eq!(
                    args.lemmas,
                    vec!["add_zero".to_string(), "mul_one".to_string()]
                );
            }
            _ => panic!("expected Simp"),
        }
    }
    #[test]
    fn test_parse_simp_star() {
        let tokens = vec![
            ident("simp"),
            make_token(TokenKind::LBracket),
            make_token(TokenKind::Star),
            make_token(TokenKind::RBracket),
        ];
        let mut parser = TacticParser::new(&tokens);
        let result = parser.parse().expect("parsing should succeed");
        match result {
            TacticExpr::Simp(args) => {
                assert!(!args.only);
                assert_eq!(args.lemmas, vec!["*".to_string()]);
            }
            _ => panic!("expected Simp"),
        }
    }
    #[test]
    fn test_parse_cases_no_arms() {
        let tokens = vec![ident("cases"), ident("h")];
        let mut parser = TacticParser::new(&tokens);
        let result = parser.parse().expect("parsing should succeed");
        assert_eq!(result, TacticExpr::Cases("h".to_string(), vec![]));
    }
    #[test]
    fn test_parse_cases_with_arms() {
        let tokens = vec![
            ident("cases"),
            ident("h"),
            ident("with"),
            make_token(TokenKind::Bar),
            ident("zero"),
            make_token(TokenKind::Arrow),
            ident("rfl"),
            make_token(TokenKind::Bar),
            ident("succ"),
            ident("n"),
            make_token(TokenKind::Arrow),
            ident("omega"),
        ];
        let mut parser = TacticParser::new(&tokens);
        let result = parser.parse().expect("parsing should succeed");
        match result {
            TacticExpr::Cases(target, arms) => {
                assert_eq!(target, "h");
                assert_eq!(arms.len(), 2);
                assert_eq!(arms[0].name, "zero");
                assert!(arms[0].bindings.is_empty());
                assert_eq!(arms[0].tactic, TacticExpr::Rfl);
                assert_eq!(arms[1].name, "succ");
                assert_eq!(arms[1].bindings, vec!["n".to_string()]);
                assert_eq!(arms[1].tactic, TacticExpr::Omega);
            }
            _ => panic!("expected Cases"),
        }
    }
    #[test]
    fn test_parse_induction_with_arms() {
        let tokens = vec![
            ident("induction"),
            ident("n"),
            ident("with"),
            make_token(TokenKind::Bar),
            ident("zero"),
            make_token(TokenKind::Arrow),
            ident("rfl"),
            make_token(TokenKind::Bar),
            ident("succ"),
            ident("k"),
            ident("ih"),
            make_token(TokenKind::Arrow),
            ident("assumption"),
        ];
        let mut parser = TacticParser::new(&tokens);
        let result = parser.parse().expect("parsing should succeed");
        match result {
            TacticExpr::Induction(target, arms) => {
                assert_eq!(target, "n");
                assert_eq!(arms.len(), 2);
                assert_eq!(arms[1].name, "succ");
                assert_eq!(arms[1].bindings, vec!["k".to_string(), "ih".to_string()]);
                assert_eq!(arms[1].tactic, TacticExpr::Assumption);
            }
            _ => panic!("expected Induction"),
        }
    }
    #[test]
    fn test_parse_have() {
        let tokens = vec![
            ident("have"),
            ident("h"),
            make_token(TokenKind::Colon),
            ident("Nat"),
            make_token(TokenKind::Assign),
            ident("trivial"),
        ];
        let mut parser = TacticParser::new(&tokens);
        let result = parser.parse().expect("parsing should succeed");
        match result {
            TacticExpr::Have(name, ty, body) => {
                assert_eq!(name, "h");
                assert_eq!(ty, Some("Nat".to_string()));
                assert_eq!(*body, TacticExpr::Trivial);
            }
            _ => panic!("expected Have"),
        }
    }
    #[test]
    fn test_parse_let() {
        let tokens = vec![
            ident("let"),
            ident("x"),
            make_token(TokenKind::Assign),
            ident("foo"),
        ];
        let mut parser = TacticParser::new(&tokens);
        let result = parser.parse().expect("parsing should succeed");
        assert_eq!(result, TacticExpr::Let("x".to_string(), "foo".to_string()));
    }
    #[test]
    fn test_parse_show() {
        let tokens = vec![ident("show"), ident("Nat")];
        let mut parser = TacticParser::new(&tokens);
        let result = parser.parse().expect("parsing should succeed");
        assert_eq!(result, TacticExpr::Show("Nat".to_string()));
    }
    #[test]
    fn test_parse_omega() {
        let tokens = vec![ident("omega")];
        let mut parser = TacticParser::new(&tokens);
        assert_eq!(
            parser.parse().expect("parsing should succeed"),
            TacticExpr::Omega
        );
    }
    #[test]
    fn test_parse_ring() {
        let tokens = vec![ident("ring")];
        let mut parser = TacticParser::new(&tokens);
        assert_eq!(
            parser.parse().expect("parsing should succeed"),
            TacticExpr::Ring
        );
    }
    #[test]
    fn test_parse_decide() {
        let tokens = vec![ident("decide")];
        let mut parser = TacticParser::new(&tokens);
        assert_eq!(
            parser.parse().expect("parsing should succeed"),
            TacticExpr::Decide
        );
    }
    #[test]
    fn test_parse_norm_num() {
        let tokens = vec![ident("norm_num")];
        let mut parser = TacticParser::new(&tokens);
        assert_eq!(
            parser.parse().expect("parsing should succeed"),
            TacticExpr::NormNum
        );
    }
    #[test]
    fn test_parse_constructor() {
        let tokens = vec![ident("constructor")];
        let mut parser = TacticParser::new(&tokens);
        assert_eq!(
            parser.parse().expect("parsing should succeed"),
            TacticExpr::Constructor
        );
    }
    #[test]
    fn test_parse_left_right() {
        let tokens_l = vec![ident("left")];
        let tokens_r = vec![ident("right")];
        let mut parser_l = TacticParser::new(&tokens_l);
        let mut parser_r = TacticParser::new(&tokens_r);
        assert_eq!(
            parser_l.parse().expect("parsing should succeed"),
            TacticExpr::Left
        );
        assert_eq!(
            parser_r.parse().expect("parsing should succeed"),
            TacticExpr::Right
        );
    }
    #[test]
    fn test_parse_existsi() {
        let tokens = vec![ident("existsi"), ident("42")];
        let mut parser = TacticParser::new(&tokens);
        assert_eq!(
            parser.parse().expect("parsing should succeed"),
            TacticExpr::Existsi("42".to_string())
        );
    }
    #[test]
    fn test_parse_clear() {
        let tokens = vec![ident("clear"), ident("h1"), ident("h2")];
        let mut parser = TacticParser::new(&tokens);
        assert_eq!(
            parser.parse().expect("parsing should succeed"),
            TacticExpr::Clear(vec!["h1".to_string(), "h2".to_string()])
        );
    }
    #[test]
    fn test_parse_revert() {
        let tokens = vec![ident("revert"), ident("x"), ident("y")];
        let mut parser = TacticParser::new(&tokens);
        assert_eq!(
            parser.parse().expect("parsing should succeed"),
            TacticExpr::Revert(vec!["x".to_string(), "y".to_string()])
        );
    }
    #[test]
    fn test_parse_subst() {
        let tokens = vec![ident("subst"), ident("h")];
        let mut parser = TacticParser::new(&tokens);
        assert_eq!(
            parser.parse().expect("parsing should succeed"),
            TacticExpr::Subst("h".to_string())
        );
    }
    #[test]
    fn test_parse_contradiction() {
        let tokens = vec![ident("contradiction")];
        let mut parser = TacticParser::new(&tokens);
        assert_eq!(
            parser.parse().expect("parsing should succeed"),
            TacticExpr::Contradiction
        );
    }
    #[test]
    fn test_parse_exfalso() {
        let tokens = vec![ident("exfalso")];
        let mut parser = TacticParser::new(&tokens);
        assert_eq!(
            parser.parse().expect("parsing should succeed"),
            TacticExpr::Exfalso
        );
    }
    #[test]
    fn test_parse_by_contra_with_name() {
        let tokens = vec![ident("by_contra"), ident("h")];
        let mut parser = TacticParser::new(&tokens);
        assert_eq!(
            parser.parse().expect("parsing should succeed"),
            TacticExpr::ByContra(Some("h".to_string()))
        );
    }
    #[test]
    fn test_parse_by_contra_no_name() {
        let tokens = vec![ident("by_contra")];
        let mut parser = TacticParser::new(&tokens);
        assert_eq!(
            parser.parse().expect("parsing should succeed"),
            TacticExpr::ByContra(None)
        );
    }
    #[test]
    fn test_parse_assumption() {
        let tokens = vec![ident("assumption")];
        let mut parser = TacticParser::new(&tokens);
        assert_eq!(
            parser.parse().expect("parsing should succeed"),
            TacticExpr::Assumption
        );
    }
    #[test]
    fn test_parse_trivial() {
        let tokens = vec![ident("trivial")];
        let mut parser = TacticParser::new(&tokens);
        assert_eq!(
            parser.parse().expect("parsing should succeed"),
            TacticExpr::Trivial
        );
    }
    #[test]
    fn test_parse_rfl() {
        let tokens = vec![ident("rfl")];
        let mut parser = TacticParser::new(&tokens);
        assert_eq!(
            parser.parse().expect("parsing should succeed"),
            TacticExpr::Rfl
        );
    }
    #[test]
    fn test_parse_block() {
        let tokens = vec![
            make_token(TokenKind::LBrace),
            ident("omega"),
            make_token(TokenKind::Semicolon),
            ident("ring"),
            make_token(TokenKind::RBrace),
        ];
        let mut parser = TacticParser::new(&tokens);
        let result = parser.parse().expect("parsing should succeed");
        match result {
            TacticExpr::Block(tactics) => {
                assert_eq!(tactics.len(), 1);
                assert!(matches!(tactics[0], TacticExpr::Seq(..)));
            }
            _ => panic!("expected Block"),
        }
    }
    #[test]
    fn test_parse_by_block() {
        let tokens = vec![
            ident("by"),
            make_token(TokenKind::LBrace),
            ident("rfl"),
            make_token(TokenKind::RBrace),
        ];
        let mut parser = TacticParser::new(&tokens);
        let result = parser
            .parse_by_block()
            .expect("mutex should not be poisoned");
        match result {
            TacticExpr::Block(tactics) => {
                assert_eq!(tactics.len(), 1);
                assert_eq!(tactics[0], TacticExpr::Rfl);
            }
            _ => panic!("expected Block"),
        }
    }
    #[test]
    fn test_parse_by_single() {
        let tokens = vec![ident("by"), ident("rfl")];
        let mut parser = TacticParser::new(&tokens);
        let result = parser
            .parse_by_block()
            .expect("mutex should not be poisoned");
        assert_eq!(result, TacticExpr::Rfl);
    }
    #[test]
    fn test_parse_conv_lhs() {
        let tokens = vec![
            ident("conv_lhs"),
            make_token(TokenKind::Arrow),
            ident("ring"),
        ];
        let mut parser = TacticParser::new(&tokens);
        let result = parser.parse().expect("parsing should succeed");
        match result {
            TacticExpr::Conv(side, inner) => {
                assert_eq!(side, ConvSide::Lhs);
                assert_eq!(*inner, TacticExpr::Ring);
            }
            _ => panic!("expected Conv"),
        }
    }
    #[test]
    fn test_parse_conv_rhs() {
        let tokens = vec![
            ident("conv_rhs"),
            make_token(TokenKind::Arrow),
            ident("ring"),
        ];
        let mut parser = TacticParser::new(&tokens);
        let result = parser.parse().expect("parsing should succeed");
        match result {
            TacticExpr::Conv(side, inner) => {
                assert_eq!(side, ConvSide::Rhs);
                assert_eq!(*inner, TacticExpr::Ring);
            }
            _ => panic!("expected Conv"),
        }
    }
    #[test]
    fn test_parse_conv_with_side() {
        let tokens = vec![
            ident("conv"),
            ident("lhs"),
            make_token(TokenKind::Arrow),
            ident("ring"),
        ];
        let mut parser = TacticParser::new(&tokens);
        let result = parser.parse().expect("parsing should succeed");
        match result {
            TacticExpr::Conv(side, inner) => {
                assert_eq!(side, ConvSide::Lhs);
                assert_eq!(*inner, TacticExpr::Ring);
            }
            _ => panic!("expected Conv"),
        }
    }
    #[test]
    fn test_parse_seq_of_structured() {
        let tokens = vec![
            ident("intro"),
            ident("x"),
            make_token(TokenKind::Semicolon),
            ident("apply"),
            ident("foo"),
            make_token(TokenKind::Semicolon),
            ident("rfl"),
        ];
        let mut parser = TacticParser::new(&tokens);
        let result = parser.parse().expect("parsing should succeed");
        match result {
            TacticExpr::Seq(left, right) => {
                assert_eq!(*right, TacticExpr::Rfl);
                match *left {
                    TacticExpr::Seq(left2, right2) => {
                        assert_eq!(*left2, TacticExpr::Intro(vec!["x".to_string()]));
                        assert_eq!(*right2, TacticExpr::Apply("foo".to_string()));
                    }
                    _ => panic!("expected inner Seq"),
                }
            }
            _ => panic!("expected Seq"),
        }
    }
    #[test]
    fn test_parse_try_structured() {
        let tokens = vec![ident("try"), ident("omega")];
        let mut parser = TacticParser::new(&tokens);
        let result = parser.parse().expect("parsing should succeed");
        match result {
            TacticExpr::Try(inner) => assert_eq!(*inner, TacticExpr::Omega),
            _ => panic!("expected Try"),
        }
    }
    #[test]
    fn test_parse_repeat_structured() {
        let tokens = vec![ident("repeat"), ident("constructor")];
        let mut parser = TacticParser::new(&tokens);
        let result = parser.parse().expect("parsing should succeed");
        match result {
            TacticExpr::Repeat(inner) => assert_eq!(*inner, TacticExpr::Constructor),
            _ => panic!("expected Repeat"),
        }
    }
    #[test]
    fn test_parse_focus() {
        let tokens = vec![ident("focus"), make_token(TokenKind::Nat(2)), ident("ring")];
        let mut parser = TacticParser::new(&tokens);
        let result = parser.parse().expect("parsing should succeed");
        match result {
            TacticExpr::Focus(n, inner) => {
                assert_eq!(n, 2);
                assert_eq!(*inner, TacticExpr::Ring);
            }
            _ => panic!("expected Focus"),
        }
    }
    #[test]
    fn test_parse_suffices() {
        let tokens = vec![
            ident("suffices"),
            ident("h"),
            ident("by"),
            ident("assumption"),
        ];
        let mut parser = TacticParser::new(&tokens);
        let result = parser.parse().expect("parsing should succeed");
        match result {
            TacticExpr::Suffices(name, body) => {
                assert_eq!(name, "h");
                assert_eq!(*body, TacticExpr::Assumption);
            }
            _ => panic!("expected Suffices"),
        }
    }
    #[test]
    fn test_parse_empty_rewrite() {
        let tokens = vec![
            ident("rw"),
            make_token(TokenKind::LBracket),
            make_token(TokenKind::RBracket),
        ];
        let mut parser = TacticParser::new(&tokens);
        let result = parser.parse().expect("parsing should succeed");
        match result {
            TacticExpr::Rewrite(rules) => assert!(rules.is_empty()),
            _ => panic!("expected Rewrite"),
        }
    }
    #[test]
    fn test_parse_use_alias() {
        let tokens = vec![ident("use"), ident("witness")];
        let mut parser = TacticParser::new(&tokens);
        let result = parser.parse().expect("parsing should succeed");
        assert_eq!(result, TacticExpr::Existsi("witness".to_string()));
    }
    #[test]
    fn test_rewrite_rule_equality() {
        let r1 = RewriteRule {
            lemma: "foo".to_string(),
            reverse: false,
        };
        let r2 = RewriteRule {
            lemma: "foo".to_string(),
            reverse: true,
        };
        assert_ne!(r1, r2);
    }
    #[test]
    fn test_simp_args_equality() {
        let a1 = SimpArgs {
            only: true,
            lemmas: vec!["a".to_string()],
            config: vec![],
        };
        let a2 = SimpArgs {
            only: false,
            lemmas: vec!["a".to_string()],
            config: vec![],
        };
        assert_ne!(a1, a2);
    }
    #[test]
    fn test_case_arm_equality() {
        let arm = CaseArm {
            name: "zero".to_string(),
            bindings: vec![],
            tactic: TacticExpr::Rfl,
        };
        let arm2 = CaseArm {
            name: "zero".to_string(),
            bindings: vec![],
            tactic: TacticExpr::Rfl,
        };
        assert_eq!(arm, arm2);
    }
    #[test]
    fn test_conv_side_equality() {
        assert_ne!(ConvSide::Lhs, ConvSide::Rhs);
        assert_eq!(ConvSide::Lhs, ConvSide::Lhs);
    }
    #[test]
    fn test_parse_all_goals_combinator() {
        let tokens = vec![
            ident("intro"),
            make_token(TokenKind::Ident("<;>".to_string())),
            ident("rfl"),
        ];
        let mut parser = TacticParser::new(&tokens);
        let result = parser.parse().expect("parsing should succeed");
        assert!(!matches!(result, TacticExpr::Rfl));
    }
    #[test]
    fn test_parse_intros() {
        let tokens = vec![ident("intros"), ident("x"), ident("y"), ident("z")];
        let mut parser = TacticParser::new(&tokens);
        let result = parser.parse().expect("parsing should succeed");
        assert_eq!(
            result,
            TacticExpr::Intros(vec!["x".to_string(), "y".to_string(), "z".to_string()])
        );
    }
    #[test]
    fn test_parse_intros_empty() {
        let tokens = vec![ident("intros")];
        let mut parser = TacticParser::new(&tokens);
        let result = parser.parse().expect("parsing should succeed");
        assert_eq!(result, TacticExpr::Intros(vec![]));
    }
    #[test]
    fn test_parse_generalize() {
        let tokens = vec![
            ident("generalize"),
            ident("h"),
            make_token(TokenKind::Colon),
            ident("n"),
        ];
        let mut parser = TacticParser::new(&tokens);
        let result = parser.parse().expect("parsing should succeed");
        assert_eq!(
            result,
            TacticExpr::Generalize("h".to_string(), "n".to_string())
        );
    }
    #[test]
    fn test_parse_obtain() {
        let tokens = vec![
            ident("obtain"),
            ident("h"),
            make_token(TokenKind::Comma),
            ident("rfl"),
        ];
        let mut parser = TacticParser::new(&tokens);
        let result = parser.parse().expect("parsing should succeed");
        match result {
            TacticExpr::Obtain(name, body) => {
                assert_eq!(name, "h");
                assert_eq!(*body, TacticExpr::Rfl);
            }
            _ => panic!("expected Obtain"),
        }
    }
    #[test]
    fn test_parse_rcases() {
        let tokens = vec![
            ident("rcases"),
            ident("h"),
            ident("with"),
            make_token(TokenKind::Bar),
            ident("zero"),
            make_token(TokenKind::Arrow),
            ident("rfl"),
        ];
        let mut parser = TacticParser::new(&tokens);
        let result = parser.parse().expect("parsing should succeed");
        match result {
            TacticExpr::Rcases(target, patterns) => {
                assert_eq!(target, "h");
                assert_eq!(patterns.len(), 1);
                assert_eq!(patterns[0], "zero");
            }
            _ => panic!("expected Rcases"),
        }
    }
    #[test]
    fn test_parse_tauto() {
        let tokens = vec![ident("tauto")];
        let mut parser = TacticParser::new(&tokens);
        assert_eq!(
            parser.parse().expect("parsing should succeed"),
            TacticExpr::Tauto
        );
    }
    #[test]
    fn test_parse_ac_rfl() {
        let tokens = vec![ident("ac_rfl")];
        let mut parser = TacticParser::new(&tokens);
        assert_eq!(
            parser.parse().expect("parsing should succeed"),
            TacticExpr::AcRfl
        );
    }
    #[test]
    fn test_first_tactic_single() {
        let tokens = vec![ident("first"), make_token(TokenKind::Bar), ident("omega")];
        let mut parser = TacticParser::new(&tokens);
        let result = parser.parse().expect("parsing should succeed");
        match result {
            TacticExpr::First(tactics) => {
                assert!(!tactics.is_empty());
            }
            _ => panic!("expected First"),
        }
    }
    #[test]
    fn test_custom_tactic_structure() {
        let custom = CustomTactic {
            name: "my_tactic".to_string(),
            params: vec!["x".to_string(), "y".to_string()],
            body: "simp".to_string(),
        };
        assert_eq!(custom.name, "my_tactic");
        assert_eq!(custom.params.len(), 2);
    }
    #[test]
    fn test_tactic_location_structure() {
        let loc = TacticLocation {
            line: 42,
            column: 10,
            name: "omega".to_string(),
        };
        assert_eq!(loc.line, 42);
        assert_eq!(loc.column, 10);
        assert_eq!(loc.name, "omega");
    }
    #[test]
    fn test_parse_seq_with_all_goals() {
        let tokens = vec![
            ident("omega"),
            make_token(TokenKind::Semicolon),
            ident("ring"),
        ];
        let mut parser = TacticParser::new(&tokens);
        let result = parser.parse().expect("parsing should succeed");
        match result {
            TacticExpr::Seq(_, _) => {}
            _ => panic!("expected Seq"),
        }
    }
    #[test]
    fn test_parse_apply_with_multiple_names() {
        let tokens = vec![ident("apply"), ident("Nat"), ident("add_comm")];
        let mut parser = TacticParser::new(&tokens);
        let result = parser.parse().expect("parsing should succeed");
        match result {
            TacticExpr::Apply(name) => {
                assert_eq!(name, "Nat");
            }
            _ => panic!("expected Apply"),
        }
    }
    #[test]
    fn test_parse_repeat_with_apply() {
        let tokens = vec![ident("repeat"), ident("apply"), ident("Nat.succ")];
        let mut parser = TacticParser::new(&tokens);
        let result = parser.parse().expect("parsing should succeed");
        match result {
            TacticExpr::Repeat(inner) => {
                assert!(matches!(*inner, TacticExpr::Apply(_)));
            }
            _ => panic!("expected Repeat"),
        }
    }
    #[test]
    fn test_parse_try_with_omega() {
        let tokens = vec![ident("try"), ident("omega")];
        let mut parser = TacticParser::new(&tokens);
        let result = parser.parse().expect("parsing should succeed");
        match result {
            TacticExpr::Try(inner) => {
                assert_eq!(*inner, TacticExpr::Omega);
            }
            _ => panic!("expected Try"),
        }
    }
    #[test]
    fn test_parse_rewrite_with_multiple_lemmas() {
        let tokens = vec![
            ident("rw"),
            make_token(TokenKind::LBracket),
            ident("lem1"),
            make_token(TokenKind::Comma),
            ident("lem2"),
            make_token(TokenKind::Comma),
            ident("lem3"),
            make_token(TokenKind::RBracket),
        ];
        let mut parser = TacticParser::new(&tokens);
        let result = parser.parse().expect("parsing should succeed");
        match result {
            TacticExpr::Rewrite(rules) => {
                assert_eq!(rules.len(), 3);
            }
            _ => panic!("expected Rewrite"),
        }
    }
    #[test]
    fn test_parse_simp_with_negation() {
        let tokens = vec![
            ident("simp"),
            make_token(TokenKind::LBracket),
            make_token(TokenKind::Minus),
            ident("lemma"),
            make_token(TokenKind::RBracket),
        ];
        let mut parser = TacticParser::new(&tokens);
        let result = parser.parse().expect("parsing should succeed");
        match result {
            TacticExpr::Simp(args) => {
                assert_eq!(args.lemmas.len(), 1);
                assert_eq!(args.lemmas[0], "-lemma");
            }
            _ => panic!("expected Simp"),
        }
    }
    #[test]
    fn test_parse_cases_multiple_arms() {
        let tokens = vec![
            ident("cases"),
            ident("n"),
            ident("with"),
            make_token(TokenKind::Bar),
            ident("zero"),
            make_token(TokenKind::Arrow),
            ident("rfl"),
            make_token(TokenKind::Bar),
            ident("succ"),
            ident("k"),
            make_token(TokenKind::Arrow),
            ident("assumption"),
            make_token(TokenKind::Bar),
            ident("other"),
            make_token(TokenKind::Arrow),
            ident("trivial"),
        ];
        let mut parser = TacticParser::new(&tokens);
        let result = parser.parse().expect("parsing should succeed");
        match result {
            TacticExpr::Cases(_, arms) => {
                assert_eq!(arms.len(), 3);
            }
            _ => panic!("expected Cases"),
        }
    }
    #[test]
    fn test_parse_induction_base_and_step() {
        let tokens = vec![
            ident("induction"),
            ident("n"),
            ident("with"),
            make_token(TokenKind::Bar),
            ident("zero"),
            make_token(TokenKind::Arrow),
            ident("rfl"),
            make_token(TokenKind::Bar),
            ident("succ"),
            ident("k"),
            ident("ih"),
            make_token(TokenKind::Arrow),
            ident("simp"),
        ];
        let mut parser = TacticParser::new(&tokens);
        let result = parser.parse().expect("parsing should succeed");
        match result {
            TacticExpr::Induction(target, arms) => {
                assert_eq!(target, "n");
                assert_eq!(arms.len(), 2);
                assert_eq!(arms[1].bindings.len(), 2);
            }
            _ => panic!("expected Induction"),
        }
    }
    #[test]
    fn test_parse_have_with_by() {
        let tokens = vec![
            ident("have"),
            ident("h"),
            make_token(TokenKind::Colon),
            ident("P"),
            make_token(TokenKind::Assign),
            ident("by"),
            ident("assumption"),
        ];
        let mut parser = TacticParser::new(&tokens);
        let result = parser.parse().expect("parsing should succeed");
        match result {
            TacticExpr::Have(name, ty, body) => {
                assert_eq!(name, "h");
                assert_eq!(ty, Some("P".to_string()));
                assert_eq!(*body, TacticExpr::Assumption);
            }
            _ => panic!("expected Have"),
        }
    }
    #[test]
    fn test_parse_calc_multiple_steps() {
        let tokens = vec![
            ident("calc"),
            make_token(TokenKind::Underscore),
            ident("="),
            ident("e1"),
            make_token(TokenKind::Assign),
            ident("by"),
            ident("rfl"),
            make_token(TokenKind::Underscore),
            ident("="),
            ident("e2"),
            make_token(TokenKind::Assign),
            ident("by"),
            ident("simp"),
        ];
        let mut parser = TacticParser::new(&tokens);
        let result = parser.parse().expect("parsing should succeed");
        match result {
            TacticExpr::Calc(steps) => {
                assert_eq!(steps.len(), 2);
            }
            _ => panic!("expected Calc"),
        }
    }
    #[test]
    fn test_parse_conv_complex() {
        let tokens = vec![
            ident("conv"),
            ident("lhs"),
            make_token(TokenKind::Arrow),
            ident("simp"),
            make_token(TokenKind::Semicolon),
            ident("ring"),
        ];
        let mut parser = TacticParser::new(&tokens);
        let result = parser.parse().expect("parsing should succeed");
        match result {
            TacticExpr::Seq(left, right) => {
                match *left {
                    TacticExpr::Conv(side, ref inner) => {
                        assert_eq!(side, ConvSide::Lhs);
                        assert!(matches!(**inner, TacticExpr::Simp(_)));
                    }
                    _ => panic!("expected Conv in left of Seq"),
                }
                assert_eq!(*right, TacticExpr::Ring);
            }
            _ => panic!("expected Seq(Conv(...), Ring)"),
        }
    }
    #[test]
    fn test_parse_clear_multiple() {
        let tokens = vec![ident("clear"), ident("h1"), ident("h2"), ident("h3")];
        let mut parser = TacticParser::new(&tokens);
        let result = parser.parse().expect("parsing should succeed");
        match result {
            TacticExpr::Clear(names) => {
                assert_eq!(names.len(), 3);
            }
            _ => panic!("expected Clear"),
        }
    }
    #[test]
    fn test_parse_revert_multiple() {
        let tokens = vec![ident("revert"), ident("x"), ident("y"), ident("z")];
        let mut parser = TacticParser::new(&tokens);
        let result = parser.parse().expect("parsing should succeed");
        match result {
            TacticExpr::Revert(names) => {
                assert_eq!(names.len(), 3);
            }
            _ => panic!("expected Revert"),
        }
    }
    #[test]
    fn test_parse_by_contra_full() {
        let tokens = vec![ident("by_contra"), ident("h")];
        let mut parser = TacticParser::new(&tokens);
        let result = parser.parse().expect("parsing should succeed");
        match result {
            TacticExpr::ByContra(Some(name)) => {
                assert_eq!(name, "h");
            }
            _ => panic!("expected ByContra with name"),
        }
    }
    #[test]
    fn test_parse_block_nested() {
        let tokens = vec![
            make_token(TokenKind::LBrace),
            ident("intro"),
            ident("x"),
            make_token(TokenKind::Semicolon),
            make_token(TokenKind::LBrace),
            ident("apply"),
            ident("foo"),
            make_token(TokenKind::Semicolon),
            ident("rfl"),
            make_token(TokenKind::RBrace),
            make_token(TokenKind::RBrace),
        ];
        let mut parser = TacticParser::new(&tokens);
        let result = parser.parse().expect("parsing should succeed");
        assert!(matches!(result, TacticExpr::Block(_)));
    }
    #[test]
    fn test_parse_intros_with_underscore() {
        let tokens = vec![
            ident("intros"),
            ident("x"),
            make_token(TokenKind::Underscore),
            ident("y"),
        ];
        let mut parser = TacticParser::new(&tokens);
        let result = parser.parse().expect("parsing should succeed");
        match result {
            TacticExpr::Intros(names) => {
                assert_eq!(names.len(), 3);
                assert_eq!(names[1], "_");
            }
            _ => panic!("expected Intros"),
        }
    }
    #[test]
    fn test_parse_constructor_nullary() {
        let tokens = vec![ident("constructor")];
        let mut parser = TacticParser::new(&tokens);
        assert_eq!(
            parser.parse().expect("parsing should succeed"),
            TacticExpr::Constructor
        );
    }
    #[test]
    fn test_parse_left_right_pair() {
        let left_tokens = vec![ident("left")];
        let right_tokens = vec![ident("right")];
        let mut left_parser = TacticParser::new(&left_tokens);
        let mut right_parser = TacticParser::new(&right_tokens);
        assert_eq!(
            left_parser.parse().expect("parsing should succeed"),
            TacticExpr::Left
        );
        assert_eq!(
            right_parser.parse().expect("parsing should succeed"),
            TacticExpr::Right
        );
    }
    #[test]
    fn test_rewrite_rule_reverse_flag() {
        let rule_fwd = RewriteRule {
            lemma: "add_comm".to_string(),
            reverse: false,
        };
        let rule_rev = RewriteRule {
            lemma: "add_comm".to_string(),
            reverse: true,
        };
        assert_ne!(rule_fwd, rule_rev);
    }
    #[test]
    fn test_simp_args_only_flag() {
        let simp1 = SimpArgs {
            only: true,
            lemmas: vec![],
            config: vec![],
        };
        let simp2 = SimpArgs {
            only: false,
            lemmas: vec![],
            config: vec![],
        };
        assert_ne!(simp1, simp2);
    }
    #[test]
    fn test_case_arm_with_bindings() {
        let arm = CaseArm {
            name: "succ".to_string(),
            bindings: vec!["n".to_string(), "ih".to_string()],
            tactic: TacticExpr::Assumption,
        };
        assert_eq!(arm.bindings.len(), 2);
    }
    #[test]
    fn test_calc_step_relation_symbols() {
        let step_eq = CalcStep {
            relation: "=".to_string(),
            rhs: "e2".to_string(),
            justification: TacticExpr::Rfl,
        };
        let step_lt = CalcStep {
            relation: "<".to_string(),
            rhs: "n".to_string(),
            justification: TacticExpr::Omega,
        };
        assert_eq!(step_eq.relation, "=");
        assert_eq!(step_lt.relation, "<");
    }
    #[test]
    fn test_parse_focus_with_goal_number() {
        let tokens = vec![
            ident("focus"),
            make_token(TokenKind::Nat(3)),
            ident("assumption"),
        ];
        let mut parser = TacticParser::new(&tokens);
        let result = parser.parse().expect("parsing should succeed");
        match result {
            TacticExpr::Focus(n, inner) => {
                assert_eq!(n, 3);
                assert_eq!(*inner, TacticExpr::Assumption);
            }
            _ => panic!("expected Focus"),
        }
    }
    #[test]
    fn test_parse_all_combinator() {
        let tokens = vec![ident("all"), ident("rfl")];
        let mut parser = TacticParser::new(&tokens);
        let result = parser.parse().expect("parsing should succeed");
        match result {
            TacticExpr::All(inner) => {
                assert_eq!(*inner, TacticExpr::Rfl);
            }
            _ => panic!("expected All"),
        }
    }
    #[test]
    fn test_parse_any_combinator() {
        let tokens = vec![ident("any"), ident("trivial")];
        let mut parser = TacticParser::new(&tokens);
        let result = parser.parse().expect("parsing should succeed");
        match result {
            TacticExpr::Any(inner) => {
                assert_eq!(*inner, TacticExpr::Trivial);
            }
            _ => panic!("expected Any"),
        }
    }
    #[test]
    fn test_parse_suffices_with_type() {
        let tokens = vec![
            ident("suffices"),
            ident("h"),
            make_token(TokenKind::Colon),
            ident("Q"),
            ident("by"),
            ident("assumption"),
        ];
        let mut parser = TacticParser::new(&tokens);
        let result = parser.parse().expect("parsing should succeed");
        match result {
            TacticExpr::Suffices(name, body) => {
                assert_eq!(name, "h");
                assert_eq!(*body, TacticExpr::Assumption);
            }
            _ => panic!("expected Suffices"),
        }
    }
    #[test]
    fn test_parse_show_goal() {
        let tokens = vec![ident("show"), ident("Nat")];
        let mut parser = TacticParser::new(&tokens);
        let result = parser.parse().expect("parsing should succeed");
        match result {
            TacticExpr::Show(ty) => {
                assert_eq!(ty, "Nat");
            }
            _ => panic!("expected Show"),
        }
    }
    #[test]
    fn test_parse_decision_tactics() {
        let decide_tokens = vec![ident("decide")];
        let norm_tokens = vec![ident("norm_num")];
        let mut decide_parser = TacticParser::new(&decide_tokens);
        let mut norm_parser = TacticParser::new(&norm_tokens);
        assert_eq!(
            decide_parser.parse().expect("parsing should succeed"),
            TacticExpr::Decide
        );
        assert_eq!(
            norm_parser.parse().expect("parsing should succeed"),
            TacticExpr::NormNum
        );
    }
    #[test]
    fn test_parse_let_in_tactic() {
        let tokens = vec![
            ident("let"),
            ident("x"),
            make_token(TokenKind::Assign),
            ident("42"),
        ];
        let mut parser = TacticParser::new(&tokens);
        let result = parser.parse().expect("parsing should succeed");
        assert_eq!(result, TacticExpr::Let("x".to_string(), "42".to_string()));
    }
    #[test]
    fn test_parse_existsi_witness() {
        let tokens = vec![ident("existsi"), ident("witness_expr")];
        let mut parser = TacticParser::new(&tokens);
        let result = parser.parse().expect("parsing should succeed");
        match result {
            TacticExpr::Existsi(witness) => {
                assert_eq!(witness, "witness_expr");
            }
            _ => panic!("expected Existsi"),
        }
    }
    #[test]
    fn test_generalize_expr_simple() {
        let tokens = vec![
            ident("generalize"),
            ident("h"),
            make_token(TokenKind::Colon),
            ident("m"),
        ];
        let mut parser = TacticParser::new(&tokens);
        let result = parser.parse().expect("parsing should succeed");
        match result {
            TacticExpr::Generalize(name, expr) => {
                assert_eq!(name, "h");
                assert_eq!(expr, "m");
            }
            _ => panic!("expected Generalize"),
        }
    }
    #[test]
    fn test_obtain_simple_pattern() {
        let tokens = vec![
            ident("obtain"),
            ident("h"),
            make_token(TokenKind::Comma),
            ident("trivial"),
        ];
        let mut parser = TacticParser::new(&tokens);
        let result = parser.parse().expect("parsing should succeed");
        assert!(matches!(result, TacticExpr::Obtain(_, _)));
    }
    #[test]
    #[allow(dead_code)]
    fn test_rcases_pattern_destructuring() {
        let tokens = vec![
            ident("rcases"),
            ident("h"),
            ident("with"),
            make_token(TokenKind::Bar),
            ident("case"),
        ];
        let mut parser = TacticParser::new(&tokens);
        let result = parser.parse().expect("parsing should succeed");
        assert!(matches!(result, TacticExpr::Rcases(_, _)));
    }
    #[test]
    fn test_first_list_alternatives() {
        let tokens = vec![
            ident("first"),
            make_token(TokenKind::Bar),
            ident("omega"),
            make_token(TokenKind::Bar),
            ident("ring"),
        ];
        let mut parser = TacticParser::new(&tokens);
        let result = parser.parse().expect("parsing should succeed");
        match result {
            TacticExpr::First(tactics) => {
                assert!(!tactics.is_empty());
            }
            _ => panic!("expected First"),
        }
    }
    #[test]
    fn test_tauto_propositional_solver() {
        let tokens = vec![ident("tauto")];
        let mut parser = TacticParser::new(&tokens);
        assert_eq!(
            parser.parse().expect("parsing should succeed"),
            TacticExpr::Tauto
        );
    }
    #[test]
    fn test_ac_rfl_associative_commutative() {
        let tokens = vec![ident("ac_rfl")];
        let mut parser = TacticParser::new(&tokens);
        assert_eq!(
            parser.parse().expect("parsing should succeed"),
            TacticExpr::AcRfl
        );
    }
    #[test]
    fn test_custom_tactic_parameters() {
        let custom = CustomTactic {
            name: "my_solver".to_string(),
            params: vec!["strategy".to_string(), "depth".to_string()],
            body: "omega".to_string(),
        };
        assert_eq!(custom.params.len(), 2);
        assert_eq!(custom.params[0], "strategy");
    }
    #[test]
    fn test_tactic_location_line_column() {
        let loc = TacticLocation {
            line: 100,
            column: 15,
            name: "simp".to_string(),
        };
        assert_eq!(loc.line, 100);
        assert_eq!(loc.column, 15);
    }
    #[test]
    fn test_parse_alternative_combinator() {
        let tokens = vec![
            ident("omega"),
            make_token(TokenKind::Ident("<|>".to_string())),
            ident("ring"),
        ];
        let mut parser = TacticParser::new(&tokens);
        let result = parser.parse().expect("parsing should succeed");
        match result {
            TacticExpr::Alt(_, _) => {}
            _ => panic!("expected Alt"),
        }
    }
    #[test]
    fn test_parse_simp_with_config() {
        let tokens = vec![
            ident("simp"),
            make_token(TokenKind::LBracket),
            ident("lem"),
            make_token(TokenKind::RBracket),
            make_token(TokenKind::LBrace),
            ident("arith"),
            make_token(TokenKind::Assign),
            ident("true"),
            make_token(TokenKind::RBrace),
        ];
        let mut parser = TacticParser::new(&tokens);
        let result = parser.parse().expect("parsing should succeed");
        match result {
            TacticExpr::Simp(args) => {
                assert!(!args.config.is_empty());
            }
            _ => panic!("expected Simp"),
        }
    }
}
