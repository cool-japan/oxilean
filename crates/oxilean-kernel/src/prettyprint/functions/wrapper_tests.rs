//! `print_expr`, `print_expr_ascii`, `print_expr_with_config` and
//! `print_level` against verbatim copies of their earlier bodies.

use super::{print_expr, print_expr_ascii, print_expr_with_config, print_level};
use crate::prettyprint::types::{ExprPrinter, PrintConfig};
use crate::{BinderInfo, Expr, FVarId, Level, Literal, Name, Node};

fn old_print_expr(expr: &Expr) -> String {
    let mut printer = ExprPrinter::new();
    printer
        .print(expr)
        .expect("pretty-printer must succeed on valid expression");
    printer.output()
}
fn old_print_expr_ascii(expr: &Expr) -> String {
    let mut printer = ExprPrinter::new().with_unicode(false);
    printer
        .print(expr)
        .expect("pretty-printer must succeed on valid expression");
    printer.output()
}
fn old_print_expr_with_config(expr: &Expr, config: PrintConfig) -> String {
    let mut printer = ExprPrinter::with_config(config);
    printer
        .print(expr)
        .expect("pretty-printer must succeed on valid expression");
    printer.output()
}
fn old_print_level(level: &Level) -> String {
    let mut printer = ExprPrinter::new();
    printer
        .print_level(level)
        .expect("pretty-printer must succeed on valid level");
    printer.output()
}

fn levels() -> Vec<Level> {
    let u = Level::param(Name::str("u"));
    let v = Level::param(Name::str("v"));
    vec![
        Level::zero(),
        Level::succ(Level::zero()),
        Level::succ(Level::succ(u.clone())),
        Level::max(u.clone(), v.clone()),
        Level::imax(u.clone(), Level::succ(v.clone())),
        Level::max(Level::succ(Level::zero()), Level::imax(v, u)),
    ]
}

fn exprs() -> Vec<Expr> {
    let nat = Expr::Const(Name::str("Nat"), vec![]);
    let f = Expr::Const(Name::str("f"), vec![]);
    let mut out: Vec<Expr> = levels().into_iter().map(Expr::Sort).collect();
    out.extend([
        Expr::BVar(0),
        Expr::BVar(7),
        Expr::FVar(FVarId(3)),
        nat.clone(),
        Expr::Const(Name::str("List"), vec![Level::succ(Level::zero())]),
        Expr::Lit(Literal::nat(42)),
        Expr::Lit(Literal::string("a \"quoted\" text")),
        Expr::App(Node::new(f.clone()), Node::new(Expr::Lit(Literal::nat(1)))),
        Expr::App(
            Node::new(Expr::App(Node::new(f), Node::new(Expr::BVar(0)))),
            Node::new(nat.clone()),
        ),
    ]);
    for bi in [
        BinderInfo::Default,
        BinderInfo::Implicit,
        BinderInfo::StrictImplicit,
        BinderInfo::InstImplicit,
    ] {
        out.push(Expr::Lam(
            bi,
            Name::str("x"),
            Node::new(nat.clone()),
            Node::new(Expr::BVar(0)),
        ));
        out.push(Expr::Pi(
            bi,
            Name::str("α"),
            Node::new(Expr::Sort(Level::succ(Level::zero()))),
            Node::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("_"),
                Node::new(Expr::BVar(0)),
                Node::new(Expr::BVar(1)),
            )),
        ));
    }
    out.push(Expr::Let(
        Name::str("y"),
        Node::new(nat.clone()),
        Node::new(Expr::Lit(Literal::nat(5))),
        Node::new(Expr::BVar(0)),
    ));
    out.push(Expr::Proj(Name::str("Prod"), 1, Node::new(Expr::BVar(2))));
    out
}

#[test]
fn the_wrappers_print_what_the_earlier_bodies_printed() {
    for expr in exprs() {
        assert_eq!(print_expr(&expr), old_print_expr(&expr));
        assert_eq!(print_expr_ascii(&expr), old_print_expr_ascii(&expr));
        for config in [
            PrintConfig::default(),
            PrintConfig::verbose(),
            PrintConfig::ascii(),
        ] {
            assert_eq!(
                print_expr_with_config(&expr, config.clone()),
                old_print_expr_with_config(&expr, config)
            );
        }
    }
    for level in levels() {
        assert_eq!(print_level(&level), old_print_level(&level));
    }
}
