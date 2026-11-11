//! `print_expr`, `print_expr_with_config`, `print_decl`,
//! `print_decl_with_config` and `print_pattern` against verbatim copies of
//! their earlier bodies.

use super::{
    print_decl, print_decl_with_config, print_expr, print_expr_with_config, print_pattern,
};
use crate::ast_impl::{Decl, Literal, Located, Pattern, SurfaceExpr};
use crate::prettyprint::types::{PrettyConfig, PrettyPrinter};
use crate::Span;

fn old_print_expr(expr: &SurfaceExpr) -> String {
    let mut pp = PrettyPrinter::new();
    pp.print_expr(expr)
        .expect("writing to String is infallible");
    pp.output()
}
fn old_print_expr_with_config(expr: &SurfaceExpr, config: PrettyConfig) -> String {
    let mut pp = PrettyPrinter::with_config(config);
    pp.print_expr(expr)
        .expect("writing to String is infallible");
    pp.output()
}
fn old_print_decl(decl: &Decl) -> String {
    let mut pp = PrettyPrinter::new();
    pp.print_decl(decl)
        .expect("writing to String is infallible");
    pp.output()
}
fn old_print_decl_with_config(decl: &Decl, config: PrettyConfig) -> String {
    let mut pp = PrettyPrinter::with_config(config);
    pp.print_decl(decl)
        .expect("writing to String is infallible");
    pp.output()
}
fn old_print_pattern(pat: &Pattern) -> String {
    let mut pp = PrettyPrinter::new();
    pp.print_pattern(pat)
        .expect("writing to String is infallible");
    pp.output()
}

const DECLS: &[&str] = &[
    "def foo := 42",
    "def double : Nat -> Nat := fun n -> n + n",
    "theorem t : forall (p : Prop), p -> p := fun p h -> h",
    "axiom ax : Nat -> Nat",
    "def f : Nat -> Nat := fun x -> if x = 0 then 1 else x * 2",
    "def g := fun (x : Nat) -> let y := x + 1 in y * y",
    "def h : List Nat -> Nat := fun xs -> match xs with | x -> x",
    "inductive Color : Type | red : Color | green : Color",
    "inductive Color : Type where | red : Color | green : Color",
    "structure Point where x : Nat y : Nat",
    "def s := \"a \\\"quoted\\\" text\"",
    "def k : Nat -> Nat -> Prop := fun a b -> forall (n : Nat), a + n = b",
    "def l := [1, 2, 3]",
    "def p := (1, 2)",
];

const EXPRS: &[&str] = &[
    "1 + 2",
    "f x y",
    "fun x -> x",
    "fun (x : Nat) (y : Nat) -> x * y + 1",
    "forall (a : Type), a -> a",
    "(a, b)",
    "[1, 2, 3]",
    "if p then a else b",
    "let z := 3 in z",
    "a = b",
    "x < y && y < z",
    "Nat.succ (Nat.succ 0)",
];

fn configs() -> Vec<PrettyConfig> {
    vec![
        PrettyConfig::default(),
        PrettyConfig::new().with_unicode(true),
        PrettyConfig::new()
            .with_max_width(20)
            .with_indent_size(4)
            .with_show_implicit(true)
            .with_show_universes(true),
    ]
}

fn loc<T>(value: T) -> Located<T> {
    Located::new(value, Span::new(0, 1, 1, 1))
}

#[test]
fn the_wrappers_print_what_the_earlier_bodies_printed() {
    for src in DECLS {
        let decl = match crate::parser::parse_decl(src) {
            Ok(decl) => decl,
            Err(e) => panic!("{src}: {e}"),
        };
        assert_eq!(print_decl(&decl.value), old_print_decl(&decl.value));
        for config in configs() {
            assert_eq!(
                print_decl_with_config(&decl.value, config.clone()),
                old_print_decl_with_config(&decl.value, config)
            );
        }
    }
    for src in EXPRS {
        let expr = match crate::parser::parse_expr(src) {
            Ok(expr) => expr,
            Err(e) => panic!("{src}: {e}"),
        };
        assert_eq!(print_expr(&expr.value), old_print_expr(&expr.value));
        for config in configs() {
            assert_eq!(
                print_expr_with_config(&expr.value, config.clone()),
                old_print_expr_with_config(&expr.value, config)
            );
        }
    }
    let patterns = [
        Pattern::Wild,
        Pattern::Var("x".to_string()),
        Pattern::Lit(Literal::Nat(7)),
        Pattern::Lit(Literal::String("s".to_string())),
        Pattern::Ctor(
            "Cons".to_string(),
            vec![loc(Pattern::Var("h".to_string())), loc(Pattern::Wild)],
        ),
        Pattern::Or(
            Box::new(loc(Pattern::Lit(Literal::Nat(0)))),
            Box::new(loc(Pattern::Ctor(
                "Succ".to_string(),
                vec![loc(Pattern::Var("n".to_string()))],
            ))),
        ),
    ];
    for pat in &patterns {
        assert_eq!(print_pattern(pat), old_print_pattern(pat));
    }
}
