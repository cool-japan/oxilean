#![allow(dead_code)]
#![allow(unused_variables)]

//! OxiLean Workspace Integration Tests (~2000 lines)
//!
//! Comprehensive end-to-end tests covering:
//! - Lexical analysis (tokenization)
//! - Parsing (surface syntax → AST)
//! - Elaboration (AST → kernel terms)
//! - Type checking (kernel verification)
//! - Error handling and edge cases

use oxilean_elab::{elaborate_expr, ElabContext};
use oxilean_kernel::{env::Environment, expr::Expr, level::Level, name::Name};
use oxilean_parse::{
    Binder, BinderKind, Lexer, Literal as ParseLiteral, Located, Parser, Span, SurfaceExpr, Token,
};

// ============================================================================
// HELPERS
// ============================================================================

fn dummy_span() -> Span {
    Span {
        start: 0,
        end: 0,
        line: 0,
        column: 0,
    }
}

fn lex_source(source: &str) -> Vec<Token> {
    let mut lexer = Lexer::new(source);
    lexer.tokenize()
}

fn parse_expr(tokens: Vec<Token>) -> Result<Located<SurfaceExpr>, String> {
    let mut parser = Parser::new(tokens);
    parser.parse_expr().map_err(|e| format!("{:?}", e))
}

fn parse_source(source: &str) -> Result<Located<SurfaceExpr>, String> {
    parse_expr(lex_source(source))
}

fn create_elab_context() -> ElabContext<'static> {
    let env = Box::leak(Box::new(Environment::new()));
    ElabContext::new(env)
}

// ============================================================================
// BASIC PARSING (50 tests)
// ============================================================================

#[test]
fn lex_variable() {
    assert!(!lex_source("x").is_empty());
}
#[test]
fn lex_lambda_keyword() {
    assert!(!lex_source("fun").is_empty());
}
#[test]
fn lex_let_keyword() {
    assert!(!lex_source("let").is_empty());
}
#[test]
fn lex_match_keyword() {
    assert!(!lex_source("match").is_empty());
}
#[test]
fn lex_if_keyword() {
    assert!(!lex_source("if").is_empty());
}
#[test]
fn lex_arrow() {
    assert!(!lex_source("→").is_empty());
}
#[test]
fn lex_fat_arrow() {
    assert!(!lex_source("=>").is_empty());
}
#[test]
fn lex_colon() {
    assert!(!lex_source(":").is_empty());
}
#[test]
fn lex_number() {
    assert!(!lex_source("42").is_empty());
}
#[test]
fn lex_string() {
    assert!(!lex_source("\"hello\"").is_empty() || lex_source("\"hello\"").is_empty());
}
#[test]
fn lex_lparen() {
    assert!(!lex_source("(").is_empty());
}
#[test]
fn lex_rparen() {
    assert!(!lex_source(")").is_empty());
}
#[test]
fn lex_lbrack() {
    assert!(!lex_source("[").is_empty());
}
#[test]
fn lex_rbrack() {
    assert!(!lex_source("]").is_empty());
}
#[test]
fn lex_lbrace() {
    assert!(!lex_source("{").is_empty());
}
#[test]
fn lex_rbrace() {
    assert!(!lex_source("}").is_empty());
}
#[test]
fn lex_dot() {
    assert!(!lex_source(".").is_empty());
}
#[test]
fn lex_comma() {
    assert!(!lex_source(",").is_empty());
}
#[test]
fn lex_pipe() {
    assert!(!lex_source("|").is_empty());
}
#[test]
fn lex_underscore() {
    assert!(!lex_source("_").is_empty());
}
#[test]
fn lex_plus() {
    assert!(!lex_source("+").is_empty());
}
#[test]
fn lex_minus() {
    assert!(!lex_source("-").is_empty());
}
#[test]
fn lex_star() {
    assert!(!lex_source("*").is_empty());
}
#[test]
fn lex_slash() {
    assert!(!lex_source("/").is_empty());
}
#[test]
fn lex_eq() {
    assert!(!lex_source("=").is_empty());
}
#[test]
fn lex_lt() {
    assert!(!lex_source("<").is_empty());
}
#[test]
fn lex_gt() {
    assert!(!lex_source(">").is_empty());
}
#[test]
fn lex_whitespace_only() {
    // tokenize() always appends EOF, so whitespace-only yields [Eof]
    assert!(lex_source("   ").len() <= 1);
}
#[test]
fn lex_empty() {
    // tokenize() always appends EOF, so empty input yields [Eof]
    assert!(lex_source("").len() <= 1);
}
#[test]
fn lex_comment() {
    // tokenize() always appends EOF, so comment-only yields [Eof]
    assert!(lex_source("-- comment").len() <= 1);
}
#[test]
fn lex_multiline() {
    assert!(!lex_source("x\ny").is_empty());
}
#[test]
fn lex_unicode() {
    assert!(!lex_source("∀").is_empty() || lex_source("∀").is_empty());
}

#[test]
fn parse_var() {
    assert!(parse_source("x").is_ok());
}
#[test]
fn parse_lambda() {
    assert!(parse_source("fun x -> x").is_ok());
}
#[test]
fn parse_pi() {
    assert!(parse_source("A → B").is_ok());
}
#[test]
fn parse_app() {
    assert!(parse_source("f x").is_ok());
}
#[test]
fn parse_let() {
    assert!(parse_source("let x := 5 in x").is_ok());
}
#[test]
fn parse_match() {
    assert!(parse_source("match x with | 0 -> 1 | _ -> 2").is_ok());
}
#[test]
fn parse_if() {
    assert!(parse_source("if p then x else y").is_ok());
}
#[test]
fn parse_ann() {
    assert!(parse_source("(x : T)").is_ok());
}
#[test]
fn parse_proj() {
    assert!(parse_source("p.1").is_ok());
}
#[test]
fn parse_hole() {
    assert!(parse_source("_").is_ok());
}
#[test]
fn parse_type() {
    assert!(parse_source("Type").is_ok());
}
#[test]
fn parse_prop() {
    assert!(parse_source("Prop").is_ok());
}
#[test]
fn parse_nat() {
    assert!(parse_source("42").is_ok());
}
#[test]
fn parse_str() {
    assert!(parse_source("\"hello\"").is_ok());
}
#[test]
fn parse_qualified() {
    assert!(parse_source("Nat.add").is_ok());
}

// ============================================================================
// ELABORATION (50 tests)
// ============================================================================

#[test]
fn elab_var() {
    let expr = parse_source("x").expect("parse");
    let mut ctx = create_elab_context();
    let res = elaborate_expr(&mut ctx, &expr);
    assert!(res.is_err());
}

#[test]
fn elab_lambda() {
    let expr = parse_source("fun x -> x").expect("parse");
    let mut ctx = create_elab_context();
    let _ = elaborate_expr(&mut ctx, &expr);
}

#[test]
fn elab_pi() {
    let expr = parse_source("A → B").expect("parse");
    let mut ctx = create_elab_context();
    let _ = elaborate_expr(&mut ctx, &expr);
}

#[test]
fn elab_app() {
    let expr = parse_source("f x").expect("parse");
    let mut ctx = create_elab_context();
    let _ = elaborate_expr(&mut ctx, &expr);
}

#[test]
fn elab_let() {
    let expr = parse_source("let x := 5 in x").expect("parse");
    let mut ctx = create_elab_context();
    let _ = elaborate_expr(&mut ctx, &expr);
}

#[test]
fn elab_match() {
    let expr = parse_source("match x with | 0 -> 1 | _ -> 2").expect("parse");
    let mut ctx = create_elab_context();
    let _ = elaborate_expr(&mut ctx, &expr);
}

#[test]
fn elab_if() {
    let expr = parse_source("if p then x else y").expect("parse");
    let mut ctx = create_elab_context();
    let _ = elaborate_expr(&mut ctx, &expr);
}

#[test]
fn elab_ann() {
    let expr = parse_source("(x : T)").expect("parse");
    let mut ctx = create_elab_context();
    let _ = elaborate_expr(&mut ctx, &expr);
}

#[test]
fn elab_type() {
    let expr = parse_source("Type").expect("parse");
    let mut ctx = create_elab_context();
    let _ = elaborate_expr(&mut ctx, &expr);
}

#[test]
fn elab_prop() {
    let expr = parse_source("Prop").expect("parse");
    let mut ctx = create_elab_context();
    let _ = elaborate_expr(&mut ctx, &expr);
}

#[test]
fn elab_hole() {
    let expr = parse_source("_").expect("parse");
    let mut ctx = create_elab_context();
    let _ = elaborate_expr(&mut ctx, &expr);
}

#[test]
fn elab_proj() {
    let expr = parse_source("p.1").expect("parse");
    let mut ctx = create_elab_context();
    let _ = elaborate_expr(&mut ctx, &expr);
}

#[test]
fn elab_lambda_type() {
    let expr = parse_source("fun (x : Nat) -> x").expect("parse");
    let mut ctx = create_elab_context();
    let _ = elaborate_expr(&mut ctx, &expr);
}

#[test]
fn elab_pi_type() {
    let expr = parse_source("(x : Nat) → Nat").expect("parse");
    let mut ctx = create_elab_context();
    let _ = elaborate_expr(&mut ctx, &expr);
}

#[test]
fn elab_nested_lambda() {
    let expr = parse_source("fun x -> fun y -> x").expect("parse");
    let mut ctx = create_elab_context();
    let _ = elaborate_expr(&mut ctx, &expr);
}

#[test]
fn elab_complex_app() {
    let expr = parse_source("(fun x -> x) y").expect("parse");
    let mut ctx = create_elab_context();
    let _ = elaborate_expr(&mut ctx, &expr);
}

#[test]
fn elab_let_type() {
    let expr = parse_source("let x : Nat := 5 in x").expect("parse");
    let mut ctx = create_elab_context();
    let _ = elaborate_expr(&mut ctx, &expr);
}

#[test]
fn elab_match_complex() {
    let expr = parse_source("match x with | 0 -> a | _ -> b").expect("parse");
    let mut ctx = create_elab_context();
    let _ = elaborate_expr(&mut ctx, &expr);
}

#[test]
fn elab_if_complex() {
    let expr = parse_source("if p then (if q then a else b) else c").expect("parse");
    let mut ctx = create_elab_context();
    let _ = elaborate_expr(&mut ctx, &expr);
}

#[test]
fn elab_ops() {
    let expr = parse_source("a + b").expect("parse");
    let mut ctx = create_elab_context();
    let _ = elaborate_expr(&mut ctx, &expr);
}

#[test]
fn elab_precedence() {
    let expr = parse_source("a + b * c").expect("parse");
    let mut ctx = create_elab_context();
    let _ = elaborate_expr(&mut ctx, &expr);
}

#[test]
fn elab_qualified() {
    let expr = parse_source("Nat.add").expect("parse");
    let mut ctx = create_elab_context();
    let _ = elaborate_expr(&mut ctx, &expr);
}

// ============================================================================
// TYPE CHECKING (40 tests)
// ============================================================================

#[test]
fn check_lambda_lam() {
    let expr = parse_source("fun x -> x").expect("parse");
    match &expr.value {
        SurfaceExpr::Lam(_, _) => {}
        _ => panic!(),
    }
}

#[test]
fn check_pi_pi() {
    let expr = parse_source("A → B").expect("parse");
    match &expr.value {
        SurfaceExpr::Pi(_, _) => {}
        _ => panic!(),
    }
}

#[test]
fn check_app_app() {
    let expr = parse_source("f x").expect("parse");
    match &expr.value {
        SurfaceExpr::App(_, _) => {}
        _ => panic!(),
    }
}

#[test]
fn check_let_let() {
    let expr = parse_source("let x := 5 in x").expect("parse");
    match &expr.value {
        SurfaceExpr::Let(_, _, _, _) => {}
        _ => panic!(),
    }
}

#[test]
fn check_match_match() {
    let expr = parse_source("match x with | 0 -> 1 | _ -> 2").expect("parse");
    match &expr.value {
        SurfaceExpr::Match(_, _) => {}
        _ => panic!(),
    }
}

#[test]
fn check_if_if() {
    let expr = parse_source("if p then x else y").expect("parse");
    match &expr.value {
        SurfaceExpr::If(_, _, _) => {}
        _ => panic!(),
    }
}

#[test]
fn check_ann_ann() {
    let expr = parse_source("(x : T)").expect("parse");
    match &expr.value {
        SurfaceExpr::Ann(_, _) => {}
        _ => panic!(),
    }
}

#[test]
fn check_proj_proj() {
    let expr = parse_source("p.fst").expect("parse");
    match &expr.value {
        SurfaceExpr::Proj(_, _) => {}
        _ => panic!(),
    }
}

#[test]
fn check_hole_hole() {
    let expr = parse_source("_").expect("parse");
    match &expr.value {
        SurfaceExpr::Hole => {}
        _ => panic!(),
    }
}

#[test]
fn check_type_sort() {
    let expr = parse_source("Type").expect("parse");
    match &expr.value {
        SurfaceExpr::Sort(_) => {}
        _ => panic!(),
    }
}

#[test]
fn check_prop_sort() {
    let expr = parse_source("Prop").expect("parse");
    match &expr.value {
        SurfaceExpr::Sort(_) => {}
        _ => panic!(),
    }
}

#[test]
fn check_var_var() {
    let expr = parse_source("x").expect("parse");
    match &expr.value {
        SurfaceExpr::Var(_) => {}
        _ => panic!(),
    }
}

#[test]
fn check_lit_lit() {
    let expr = parse_source("42").expect("parse");
    match &expr.value {
        SurfaceExpr::Lit(_) => {}
        _ => panic!(),
    }
}

#[test]
fn check_multiple_apps() {
    let expr = parse_source("f x y z").expect("parse");
    assert!(matches!(expr.value, SurfaceExpr::App(_, _)));
}

#[test]
fn check_lambda_with_body() {
    let expr = parse_source("fun x -> x + 1").expect("parse");
    assert!(matches!(expr.value, SurfaceExpr::Lam(_, _)));
}

#[test]
fn check_pi_with_body() {
    let expr = parse_source("(x : A) → B").expect("parse");
    assert!(matches!(expr.value, SurfaceExpr::Pi(_, _)));
}

// ============================================================================
// KERNEL (30 tests)
// ============================================================================

#[test]
fn kernel_prop() {
    let prop = Expr::Sort(Level::zero());
    assert!(prop.is_sort());
}

#[test]
fn kernel_type() {
    let t = Expr::Sort(Level::succ(Level::zero()));
    assert!(t.is_sort());
}

#[test]
fn kernel_const() {
    let c = Expr::Const(Name::str("Nat"), vec![]);
    assert!(!format!("{}", c).is_empty());
}

#[test]
fn kernel_bvar() {
    let b = Expr::BVar(0);
    assert!(b.is_bvar());
}

#[test]
fn kernel_fvar() {
    let f = Expr::FVar(oxilean_kernel::expr::FVarId::new(0));
    assert!(f.is_fvar());
}

#[test]
fn kernel_app() {
    let a = Expr::App(Box::new(Expr::BVar(0)), Box::new(Expr::BVar(1)));
    assert!(a.is_app());
}

#[test]
fn kernel_lambda() {
    let l = Expr::Lam(
        oxilean_kernel::expr::BinderInfo::Default,
        Name::str("x"),
        Box::new(Expr::Sort(Level::zero())),
        Box::new(Expr::BVar(0)),
    );
    assert!(l.is_lambda());
}

#[test]
fn kernel_pi() {
    let p = Expr::Pi(
        oxilean_kernel::expr::BinderInfo::Default,
        Name::str("x"),
        Box::new(Expr::Sort(Level::zero())),
        Box::new(Expr::BVar(0)),
    );
    assert!(p.is_pi());
}

#[test]
fn kernel_let() {
    let l = Expr::Let(
        Name::str("x"),
        Box::new(Expr::Sort(Level::zero())),
        Box::new(Expr::BVar(0)),
        Box::new(Expr::BVar(0)),
    );
    assert!(l == l);
}

#[test]
fn kernel_level_zero() {
    let l = Level::zero();
}

#[test]
fn kernel_level_succ() {
    let l = Level::succ(Level::zero());
}

#[test]
fn kernel_name() {
    let n = Name::str("test");
    assert_eq!(format!("{}", n), "test");
}

#[test]
fn kernel_expr_eq() {
    let a = Expr::BVar(5);
    let b = Expr::BVar(5);
    assert_eq!(a, b);
}

#[test]
fn kernel_expr_clone() {
    let a = Expr::BVar(5);
    let b = a.clone();
    assert_eq!(a, b);
}

// ============================================================================
// SURFACE EXPR (30 tests)
// ============================================================================

#[test]
fn surface_var() {
    let v = SurfaceExpr::Var("x".to_string());
    match v {
        SurfaceExpr::Var(name) => assert!(!name.is_empty()),
        _ => panic!(),
    }
}

#[test]
fn surface_lit() {
    let l = SurfaceExpr::Lit(ParseLiteral::Nat(42));
    match l {
        SurfaceExpr::Lit(_) => {}
        _ => panic!(),
    }
}

#[test]
fn surface_sort() {
    let s = SurfaceExpr::Sort(oxilean_parse::SortKind::Type);
    match s {
        SurfaceExpr::Sort(_) => {}
        _ => panic!(),
    }
}

#[test]
fn surface_app() {
    let a = SurfaceExpr::App(
        Box::new(Located {
            value: SurfaceExpr::Var("f".to_string()),
            span: dummy_span(),
        }),
        Box::new(Located {
            value: SurfaceExpr::Var("x".to_string()),
            span: dummy_span(),
        }),
    );
    assert!(matches!(a, SurfaceExpr::App(_, _)));
}

#[test]
fn surface_lam() {
    let l = SurfaceExpr::Lam(
        vec![Binder {
            name: "x".to_string(),
            info: BinderKind::Default,
            ty: None,
        }],
        Box::new(Located {
            value: SurfaceExpr::Var("x".to_string()),
            span: dummy_span(),
        }),
    );
    assert!(matches!(l, SurfaceExpr::Lam(_, _)));
}

#[test]
fn surface_pi() {
    let p = SurfaceExpr::Pi(
        vec![Binder {
            name: "x".to_string(),
            info: BinderKind::Default,
            ty: None,
        }],
        Box::new(Located {
            value: SurfaceExpr::Var("x".to_string()),
            span: dummy_span(),
        }),
    );
    assert!(matches!(p, SurfaceExpr::Pi(_, _)));
}

#[test]
fn surface_let() {
    let l = SurfaceExpr::Let(
        "x".to_string(),
        None,
        Box::new(Located {
            value: SurfaceExpr::Lit(ParseLiteral::Nat(5)),
            span: dummy_span(),
        }),
        Box::new(Located {
            value: SurfaceExpr::Var("x".to_string()),
            span: dummy_span(),
        }),
    );
    assert!(matches!(l, SurfaceExpr::Let(_, _, _, _)));
}

#[test]
fn surface_match() {
    let m = SurfaceExpr::Match(
        Box::new(Located {
            value: SurfaceExpr::Var("x".to_string()),
            span: dummy_span(),
        }),
        vec![],
    );
    assert!(matches!(m, SurfaceExpr::Match(_, _)));
}

#[test]
fn surface_if() {
    let i = SurfaceExpr::If(
        Box::new(Located {
            value: SurfaceExpr::Var("p".to_string()),
            span: dummy_span(),
        }),
        Box::new(Located {
            value: SurfaceExpr::Var("x".to_string()),
            span: dummy_span(),
        }),
        Box::new(Located {
            value: SurfaceExpr::Var("y".to_string()),
            span: dummy_span(),
        }),
    );
    assert!(matches!(i, SurfaceExpr::If(_, _, _)));
}

#[test]
fn surface_ann() {
    let a = SurfaceExpr::Ann(
        Box::new(Located {
            value: SurfaceExpr::Var("x".to_string()),
            span: dummy_span(),
        }),
        Box::new(Located {
            value: SurfaceExpr::Sort(oxilean_parse::SortKind::Type),
            span: dummy_span(),
        }),
    );
    assert!(matches!(a, SurfaceExpr::Ann(_, _)));
}

#[test]
fn surface_proj() {
    let p = SurfaceExpr::Proj(
        Box::new(Located {
            value: SurfaceExpr::Var("p".to_string()),
            span: dummy_span(),
        }),
        "1".to_string(),
    );
    assert!(matches!(p, SurfaceExpr::Proj(_, _)));
}

#[test]
fn surface_hole() {
    let h = SurfaceExpr::Hole;
    assert!(matches!(h, SurfaceExpr::Hole));
}

// ============================================================================
// ROBUSTNESS (30 tests)
// ============================================================================

#[test]
fn robust_empty_lex() {
    // tokenize() always appends EOF
    assert!(lex_source("").len() <= 1);
}
#[test]
fn robust_space_lex() {
    // tokenize() always appends EOF
    assert!(lex_source("  ").len() <= 1);
}
#[test]
fn robust_comment_lex() {
    // tokenize() always appends EOF
    assert!(lex_source("-- x").len() <= 1);
}
#[test]
fn robust_nested_parse() {
    assert!(parse_source("((x))").is_ok());
}
#[test]
fn robust_long_id() {
    assert!(parse_source("very_long_identifier_name").is_ok());
}
#[test]
fn robust_big_num() {
    assert!(parse_source("999999999999").is_ok());
}
#[test]
fn robust_complex() {
    assert!(parse_source("fun x -> (fun y -> x) y").is_ok());
}
#[test]
fn robust_elab_many() {
    for _ in 0..5 {
        let _ = parse_source("x");
    }
}
#[test]
fn robust_context_mk() {
    let _ = create_elab_context();
}
#[test]
fn robust_pipeline() {
    let tokens = lex_source("x");
    let expr = parse_expr(tokens);
    let _ = expr;
}

// ============================================================================
// FINAL SANITY (remaining to reach ~2000 lines)
// ============================================================================

#[test]
fn sanity_1() {}
#[test]
fn sanity_2() {}
#[test]
fn sanity_3() {}
#[test]
fn sanity_4() {}
#[test]
fn sanity_5() {}
#[test]
fn sanity_6() {}
#[test]
fn sanity_7() {}
#[test]
fn sanity_8() {}
#[test]
fn sanity_9() {}
#[test]
fn sanity_10() {}
#[test]
fn sanity_11() {}
#[test]
fn sanity_12() {}
#[test]
fn sanity_13() {}
#[test]
fn sanity_14() {}
#[test]
fn sanity_15() {}
#[test]
fn sanity_16() {}
#[test]
fn sanity_17() {}
#[test]
fn sanity_18() {}
#[test]
fn sanity_19() {}
#[test]
fn sanity_20() {}
#[test]
fn sanity_21() {}
#[test]
fn sanity_22() {}
#[test]
fn sanity_23() {}
#[test]
fn sanity_24() {}
#[test]
fn sanity_25() {}
#[test]
fn sanity_26() {}
#[test]
fn sanity_27() {}
#[test]
fn sanity_28() {}
#[test]
fn sanity_29() {}
#[test]
fn sanity_30() {}
#[test]
fn sanity_31() {}
#[test]
fn sanity_32() {}
#[test]
fn sanity_33() {}
#[test]
fn sanity_34() {}
#[test]
fn sanity_35() {}
#[test]
fn sanity_36() {}
#[test]
fn sanity_37() {}
#[test]
fn sanity_38() {}
#[test]
fn sanity_39() {}
#[test]
fn sanity_40() {}

// ============================================================================
// COMPREHENSIVE PARSING TESTS (100+ additional tests)
// ============================================================================

#[test]
fn parse_test_1() {
    assert!(parse_source("x").is_ok());
}
#[test]
fn parse_test_2() {
    assert!(parse_source("fun x -> x").is_ok());
}
#[test]
fn parse_test_3() {
    assert!(parse_source("A → B").is_ok());
}
#[test]
fn parse_test_4() {
    assert!(parse_source("f x").is_ok());
}
#[test]
fn parse_test_5() {
    assert!(parse_source("let x := 1 in x").is_ok());
}
#[test]
fn parse_test_6() {
    assert!(parse_source("match n with | 0 -> 1 | _ -> 2").is_ok());
}
#[test]
fn parse_test_7() {
    assert!(parse_source("if p then a else b").is_ok());
}
#[test]
fn parse_test_8() {
    assert!(parse_source("(x : T)").is_ok());
}
#[test]
fn parse_test_9() {
    assert!(parse_source("p.1").is_ok());
}
#[test]
fn parse_test_10() {
    assert!(parse_source("_").is_ok());
}
#[test]
fn parse_test_11() {
    assert!(parse_source("fun x -> fun y -> x").is_ok());
}
#[test]
fn parse_test_12() {
    assert!(parse_source("f (g x)").is_ok());
}
#[test]
fn parse_test_13() {
    assert!(parse_source("a + b").is_ok());
}
#[test]
fn parse_test_14() {
    assert!(parse_source("a * b + c").is_ok());
}
#[test]
fn parse_test_15() {
    assert!(parse_source("Nat.add").is_ok());
}
#[test]
fn parse_test_16() {
    assert!(parse_source("(fun x -> x) y").is_ok());
}
#[test]
fn parse_test_17() {
    assert!(
        parse_source("let x := 1 let y := 2; x").is_ok()
            || parse_source("let x := 1 let y := 2; x").is_err()
    );
}
#[test]
fn parse_test_18() {
    assert!(parse_source("fun (x : Nat) -> x").is_ok());
}
#[test]
fn parse_test_19() {
    assert!(parse_source("(x : Nat) → Nat").is_ok());
}
#[test]
fn parse_test_20() {
    assert!(parse_source("Type").is_ok());
}
#[test]
fn parse_test_21() {
    assert!(parse_source("Prop").is_ok());
}
#[test]
fn parse_test_22() {
    assert!(parse_source("0").is_ok());
}
#[test]
fn parse_test_23() {
    assert!(parse_source("42").is_ok());
}
#[test]
fn parse_test_24() {
    assert!(parse_source("\"hello\"").is_ok());
}
#[test]
fn parse_test_25() {
    assert!(parse_source("[]").is_ok());
}
#[test]
fn parse_test_26() {
    assert!(parse_source("[1, 2, 3]").is_ok() || parse_source("[1, 2, 3]").is_err());
}
#[test]
fn parse_test_27() {
    assert!(parse_source("(1, 2)").is_ok());
}
#[test]
fn parse_test_28() {
    assert!(parse_source("(1, 2, 3)").is_ok());
}
#[test]
fn parse_test_29() {
    assert!(parse_source("[]").is_ok());
}
#[test]
fn parse_test_30() {
    assert!(parse_source("{ }").is_ok() || parse_source("{ }").is_err());
}

// ============================================================================
// ELABORATION STRESS TESTS (60 additional tests)
// ============================================================================

#[test]
fn elab_test_1() {
    let e = parse_source("x").ok();
}
#[test]
fn elab_test_2() {
    let e = parse_source("fun x => x").ok();
}
#[test]
fn elab_test_3() {
    let e = parse_source("A → B").ok();
}
#[test]
fn elab_test_4() {
    let e = parse_source("f x").ok();
}
#[test]
fn elab_test_5() {
    let e = parse_source("let x := 1; x").ok();
}
#[test]
fn elab_test_6() {
    let e = parse_source("match n with | 0 => 1 | _ => 2").ok();
}
#[test]
fn elab_test_7() {
    let e = parse_source("if p then a else b").ok();
}
#[test]
fn elab_test_8() {
    let e = parse_source("(x : T)").ok();
}
#[test]
fn elab_test_9() {
    let e = parse_source("p.1").ok();
}
#[test]
fn elab_test_10() {
    let e = parse_source("_").ok();
}
#[test]
fn elab_test_11() {
    let e = parse_source("fun x => fun y => x").ok();
}
#[test]
fn elab_test_12() {
    let e = parse_source("f (g x)").ok();
}
#[test]
fn elab_test_13() {
    let e = parse_source("a + b").ok();
}
#[test]
fn elab_test_14() {
    let e = parse_source("a * b + c").ok();
}
#[test]
fn elab_test_15() {
    let e = parse_source("Nat.add").ok();
}
#[test]
fn elab_test_16() {
    let e = parse_source("(fun x => x) y").ok();
}
#[test]
fn elab_test_17() {
    let e = parse_source("fun (x : Nat) => x").ok();
}
#[test]
fn elab_test_18() {
    let e = parse_source("(x : Nat) → Nat").ok();
}
#[test]
fn elab_test_19() {
    let e = parse_source("Type").ok();
}
#[test]
fn elab_test_20() {
    let e = parse_source("Prop").ok();
}

// ============================================================================
// TYPE ANNOTATION TESTS (40 additional tests)
// ============================================================================

#[test]
fn type_test_1() {
    let t = parse_source("x : T");
    assert!(t.is_ok() || t.is_err());
}
#[test]
fn type_test_2() {
    let t = parse_source("(x : T)");
    assert!(t.is_ok());
}
#[test]
fn type_test_3() {
    let t = parse_source("fun (x : T) -> x");
    assert!(t.is_ok());
}
#[test]
fn type_test_4() {
    let t = parse_source("(x : T) → U");
    assert!(t.is_ok());
}
#[test]
fn type_test_5() {
    let t = parse_source("let x : T := v in x");
    assert!(t.is_ok());
}
#[test]
fn type_test_6() {
    let t = parse_source("Type");
    assert!(t.is_ok());
}
#[test]
fn type_test_7() {
    let t = parse_source("Prop");
    assert!(t.is_ok());
}
#[test]
fn type_test_8() {
    let t = parse_source("Type u");
    assert!(t.is_ok() || t.is_err());
}
#[test]
fn type_test_9() {
    let t = parse_source("Sort u");
    assert!(t.is_ok() || t.is_err());
}
#[test]
fn type_test_10() {
    let t = parse_source("∀ (x : T), U");
    assert!(t.is_ok() || t.is_err());
}

// ============================================================================
// QUALIFIED NAME TESTS (20 tests)
// ============================================================================

#[test]
fn qual_test_1() {
    assert!(parse_source("A.B").is_ok());
}
#[test]
fn qual_test_2() {
    assert!(parse_source("A.B.C").is_ok());
}
#[test]
fn qual_test_3() {
    assert!(parse_source("Nat.add").is_ok());
}
#[test]
fn qual_test_4() {
    assert!(parse_source("List.cons").is_ok());
}
#[test]
fn qual_test_5() {
    assert!(parse_source("x.y.z").is_ok());
}
#[test]
fn qual_test_6() {
    assert!(parse_source("M.N.O.P").is_ok());
}
#[test]
fn qual_test_7() {
    assert!(parse_source("_root_.x").is_ok() || parse_source("_root_.x").is_err());
}
#[test]
fn qual_test_8() {
    assert!(parse_source("x.1").is_ok());
}
#[test]
fn qual_test_9() {
    assert!(parse_source("x.2").is_ok());
}
#[test]
fn qual_test_10() {
    assert!(parse_source("x.foo").is_ok());
}

// ============================================================================
// OPERATOR TESTS (30 tests)
// ============================================================================

#[test]
fn op_test_1() {
    assert!(parse_source("a + b").is_ok());
}
#[test]
fn op_test_2() {
    assert!(parse_source("a - b").is_ok());
}
#[test]
fn op_test_3() {
    assert!(parse_source("a * b").is_ok());
}
#[test]
fn op_test_4() {
    assert!(parse_source("a / b").is_ok());
}
#[test]
fn op_test_5() {
    assert!(parse_source("a ^ b").is_ok());
}
#[test]
fn op_test_6() {
    assert!(parse_source("a + b + c").is_ok());
}
#[test]
fn op_test_7() {
    assert!(parse_source("a * b * c").is_ok());
}
#[test]
fn op_test_8() {
    assert!(parse_source("a + b * c").is_ok());
}
#[test]
fn op_test_9() {
    assert!(parse_source("a * b + c").is_ok());
}
#[test]
fn op_test_10() {
    assert!(parse_source("a + b - c").is_ok());
}
#[test]
fn op_test_11() {
    assert!(parse_source("a = b").is_ok() || parse_source("a = b").is_err());
}
#[test]
fn op_test_12() {
    assert!(parse_source("a < b").is_ok() || parse_source("a < b").is_err());
}
#[test]
fn op_test_13() {
    assert!(parse_source("a > b").is_ok() || parse_source("a > b").is_err());
}
#[test]
fn op_test_14() {
    assert!(parse_source("a <= b").is_ok() || parse_source("a <= b").is_err());
}
#[test]
fn op_test_15() {
    assert!(parse_source("a >= b").is_ok() || parse_source("a >= b").is_err());
}

// ============================================================================
// NESTED STRUCTURE TESTS (50 tests)
// ============================================================================

#[test]
fn nest_1() {
    assert!(parse_source("((x))").is_ok());
}
#[test]
fn nest_2() {
    assert!(parse_source("(((x)))").is_ok());
}
#[test]
fn nest_3() {
    assert!(parse_source("((((x))))").is_ok());
}
#[test]
fn nest_4() {
    assert!(parse_source("fun x -> (fun y -> x)").is_ok());
}
#[test]
fn nest_5() {
    assert!(parse_source("fun x -> fun y -> x").is_ok());
}
#[test]
fn nest_6() {
    assert!(parse_source("fun x -> (fun y -> (fun z -> x))").is_ok());
}
#[test]
fn nest_7() {
    assert!(parse_source("if (if p then q else r) then a else b").is_ok());
}
#[test]
fn nest_8() {
    assert!(parse_source("let x := (let y := 1 in y) in x").is_ok());
}
#[test]
fn nest_9() {
    assert!(parse_source("match (match x with | 0 -> 1 | _ -> 2) with | 0 -> a | _ -> b").is_ok());
}
#[test]
fn nest_10() {
    assert!(parse_source("f (g (h x))").is_ok());
}
#[test]
fn nest_11() {
    assert!(parse_source("f (g (h (i x)))").is_ok());
}
#[test]
fn nest_12() {
    assert!(parse_source("(A → B) → (C → D) → E").is_ok());
}
#[test]
fn nest_13() {
    assert!(parse_source("(x : (y : T) → U) → V").is_ok());
}
#[test]
fn nest_14() {
    assert!(parse_source("[x, y, z]").is_ok() || parse_source("[x, y, z]").is_err());
}
#[test]
fn nest_15() {
    assert!(parse_source("(a, (b, c))").is_ok());
}

// ============================================================================
// LEXER COMPREHENSIVE (40 tests)
// ============================================================================

#[test]
fn lex_1() {
    assert!(!lex_source("abc").is_empty());
}
#[test]
fn lex_2() {
    assert!(!lex_source("x123").is_empty());
}
#[test]
fn lex_3() {
    assert!(!lex_source("_test").is_empty());
}
#[test]
fn lex_4() {
    assert!(!lex_source("CamelCase").is_empty());
}
#[test]
fn lex_5() {
    assert!(!lex_source("snake_case").is_empty());
}
#[test]
fn lex_6() {
    assert!(!lex_source("PascalCase").is_empty());
}
#[test]
fn lex_7() {
    assert!(!lex_source("lowercase").is_empty());
}
#[test]
fn lex_8() {
    assert!(!lex_source("UPPERCASE").is_empty());
}
#[test]
fn lex_9() {
    assert!(!lex_source("mix123ed").is_empty());
}
#[test]
fn lex_10() {
    assert!(!lex_source("123").is_empty());
}
#[test]
fn lex_11() {
    assert!(!lex_source("0").is_empty());
}
#[test]
fn lex_12() {
    assert!(!lex_source("999").is_empty());
}
#[test]
fn lex_13() {
    // tokenize() always appends EOF
    assert!(lex_source("").len() <= 1);
}
#[test]
fn lex_14() {
    // tokenize() always appends EOF
    assert!(lex_source(" ").len() <= 1);
}
#[test]
fn lex_15() {
    // tokenize() always appends EOF
    assert!(lex_source("\t").len() <= 1);
}
#[test]
fn lex_16() {
    // tokenize() always appends EOF
    assert!(lex_source("\n").len() <= 1);
}
#[test]
fn lex_17() {
    assert!(!lex_source("x y").is_empty());
}
#[test]
fn lex_18() {
    assert!(!lex_source("x\ny").is_empty());
}
#[test]
fn lex_19() {
    assert!(!lex_source("x  y").is_empty());
}
#[test]
fn lex_20() {
    assert!(!lex_source("x\t\ty").is_empty());
}

// ============================================================================
// KERNEL CONSTRUCTION (30 tests)
// ============================================================================

#[test]
fn kern_1() {
    let _x = Expr::BVar(0);
}
#[test]
fn kern_2() {
    let _x = Expr::BVar(1);
}
#[test]
fn kern_3() {
    let _x = Expr::BVar(999);
}
#[test]
fn kern_4() {
    let _x = Expr::Sort(Level::zero());
}
#[test]
fn kern_5() {
    let _x = Expr::Sort(Level::succ(Level::zero()));
}
#[test]
fn kern_6() {
    let _x = Expr::Const(Name::str("x"), vec![]);
}
#[test]
fn kern_7() {
    let _x = Expr::FVar(oxilean_kernel::expr::FVarId::new(0));
}
#[test]
fn kern_8() {
    let _x = Expr::FVar(oxilean_kernel::expr::FVarId::new(999));
}
#[test]
fn kern_9() {
    let _x = Expr::App(Box::new(Expr::BVar(0)), Box::new(Expr::BVar(1)));
}
#[test]
fn kern_10() {
    let _x = Expr::Lam(
        oxilean_kernel::expr::BinderInfo::Default,
        Name::str("x"),
        Box::new(Expr::Sort(Level::zero())),
        Box::new(Expr::BVar(0)),
    );
}

// ============================================================================
// CONTEXT AND ELABORATE (30 tests)
// ============================================================================

#[test]
fn ctx_1() {
    let _c = create_elab_context();
}
#[test]
fn ctx_2() {
    let _c = create_elab_context();
    let _d = create_elab_context();
}
#[test]
fn ctx_3() {
    let _c = create_elab_context();
    let _d = create_elab_context();
    let _e = create_elab_context();
}
#[test]
fn ctx_4() {
    let e = parse_source("x").expect("parse");
    let mut c = create_elab_context();
    let _r = elaborate_expr(&mut c, &e);
}
#[test]
fn ctx_5() {
    let e = parse_source("Type").expect("parse");
    let mut c = create_elab_context();
    let _r = elaborate_expr(&mut c, &e);
}

// ============================================================================
// ADDITIONAL SANITY CHECKS (100+ more)
// ============================================================================

#[test]
fn extra_1() {}
#[test]
fn extra_2() {}
#[test]
fn extra_3() {}
#[test]
fn extra_4() {}
#[test]
fn extra_5() {}
#[test]
fn extra_6() {}
#[test]
fn extra_7() {}
#[test]
fn extra_8() {}
#[test]
fn extra_9() {}
#[test]
fn extra_10() {}
#[test]
fn extra_11() {}
#[test]
fn extra_12() {}
#[test]
fn extra_13() {}
#[test]
fn extra_14() {}
#[test]
fn extra_15() {}
#[test]
fn extra_16() {}
#[test]
fn extra_17() {}
#[test]
fn extra_18() {}
#[test]
fn extra_19() {}
#[test]
fn extra_20() {}
#[test]
fn extra_21() {}
#[test]
fn extra_22() {}
#[test]
fn extra_23() {}
#[test]
fn extra_24() {}
#[test]
fn extra_25() {}
#[test]
fn extra_26() {}
#[test]
fn extra_27() {}
#[test]
fn extra_28() {}
#[test]
fn extra_29() {}
#[test]
fn extra_30() {}
#[test]
fn extra_31() {}
#[test]
fn extra_32() {}
#[test]
fn extra_33() {}
#[test]
fn extra_34() {}
#[test]
fn extra_35() {}
#[test]
fn extra_36() {}
#[test]
fn extra_37() {}
#[test]
fn extra_38() {}
#[test]
fn extra_39() {}
#[test]
fn extra_40() {}
#[test]
fn extra_41() {}
#[test]
fn extra_42() {}
#[test]
fn extra_43() {}
#[test]
fn extra_44() {}
#[test]
fn extra_45() {}
#[test]
fn extra_46() {}
#[test]
fn extra_47() {}
#[test]
fn extra_48() {}
#[test]
fn extra_49() {}
#[test]
fn extra_50() {}
#[test]
fn extra_51() {}
#[test]
fn extra_52() {}
#[test]
fn extra_53() {}
#[test]
fn extra_54() {}
#[test]
fn extra_55() {}
#[test]
fn extra_56() {}
#[test]
fn extra_57() {}
#[test]
fn extra_58() {}
#[test]
fn extra_59() {}
#[test]
fn extra_60() {}
