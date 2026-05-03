//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)
#![allow(clippy::items_after_test_module)]

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{ContextFreeGrammar, PegExpr};

/// Prop: `Sort 0`.
#[allow(dead_code)]
pub fn prop() -> Expr {
    Expr::Sort(Level::zero())
}
/// Type 1: `Sort 1`.
#[allow(dead_code)]
pub(super) fn type1() -> Expr {
    Expr::Sort(Level::succ(Level::zero()))
}
/// Type 2: `Sort 2`.
#[allow(dead_code)]
pub fn type2() -> Expr {
    Expr::Sort(Level::succ(Level::succ(Level::zero())))
}
/// Nat type constant.
#[allow(dead_code)]
pub fn nat_ty() -> Expr {
    Expr::Const(Name::str("Nat"), vec![])
}
/// Int type constant.
#[allow(dead_code)]
pub fn int_ty() -> Expr {
    Expr::Const(Name::str("Int"), vec![])
}
/// Bool type constant.
#[allow(dead_code)]
pub fn bool_ty() -> Expr {
    Expr::Const(Name::str("Bool"), vec![])
}
/// Char type constant.
#[allow(dead_code)]
pub fn char_ty() -> Expr {
    Expr::Const(Name::str("Char"), vec![])
}
/// String type constant.
#[allow(dead_code)]
pub fn string_ty() -> Expr {
    Expr::Const(Name::str("String"), vec![])
}
/// Unit type constant.
#[allow(dead_code)]
pub fn unit_ty() -> Expr {
    Expr::Const(Name::str("Unit"), vec![])
}
/// `String.Pos` type constant.
#[allow(dead_code)]
pub fn string_pos_ty() -> Expr {
    Expr::Const(Name::str("String.Pos"), vec![])
}
/// `List` applied to a type argument.
#[allow(dead_code)]
pub fn list_of(elem_ty: Expr) -> Expr {
    app(Expr::Const(Name::str("List"), vec![]), elem_ty)
}
/// `Option` applied to a type argument.
#[allow(dead_code)]
pub fn option_of(elem_ty: Expr) -> Expr {
    app(Expr::Const(Name::str("Option"), vec![]), elem_ty)
}
/// `Inhabited` applied to a type argument.
#[allow(dead_code)]
pub fn inhabited_of(ty: Expr) -> Expr {
    app(Expr::Const(Name::str("Inhabited"), vec![]), ty)
}
/// Build a non-dependent arrow `A -> B`.
#[allow(dead_code)]
pub fn arrow(a: Expr, b: Expr) -> Expr {
    Expr::Pi(
        BinderInfo::Default,
        Name::str("_"),
        Box::new(a),
        Box::new(b),
    )
}
/// Function application `f a`.
#[allow(dead_code)]
pub fn app(f: Expr, a: Expr) -> Expr {
    Expr::App(Box::new(f), Box::new(a))
}
/// Function application `f a b`.
#[allow(dead_code)]
pub fn app2(f: Expr, a: Expr, b: Expr) -> Expr {
    app(app(f, a), b)
}
/// Function application `f a b c`.
#[allow(dead_code)]
pub fn app3(f: Expr, a: Expr, b: Expr, c: Expr) -> Expr {
    app(app2(f, a, b), c)
}
/// Function application `f a b c d`.
#[allow(dead_code)]
pub fn app4(f: Expr, a: Expr, b: Expr, c: Expr, d: Expr) -> Expr {
    app(app3(f, a, b, c), d)
}
/// An implicit Pi binder.
#[allow(dead_code)]
pub fn implicit_pi(name: &str, ty: Expr, body: Expr) -> Expr {
    Expr::Pi(
        BinderInfo::Implicit,
        Name::str(name),
        Box::new(ty),
        Box::new(body),
    )
}
/// A default (explicit) Pi binder.
#[allow(dead_code)]
pub fn default_pi(name: &str, ty: Expr, body: Expr) -> Expr {
    Expr::Pi(
        BinderInfo::Default,
        Name::str(name),
        Box::new(ty),
        Box::new(body),
    )
}
/// An instance-implicit Pi binder.
#[allow(dead_code)]
pub fn inst_pi(name: &str, ty: Expr, body: Expr) -> Expr {
    Expr::Pi(
        BinderInfo::InstImplicit,
        Name::str(name),
        Box::new(ty),
        Box::new(body),
    )
}
/// Build `Eq @{} ty a b`.
#[allow(dead_code)]
pub fn eq_expr(ty: Expr, a: Expr, b: Expr) -> Expr {
    app3(Expr::Const(Name::str("Eq"), vec![]), ty, a, b)
}
/// `ParseResult ε α` applied to two type arguments.
#[allow(dead_code)]
pub fn parse_result_of(err_ty: Expr, val_ty: Expr) -> Expr {
    app2(
        Expr::Const(Name::str("ParseResult"), vec![]),
        err_ty,
        val_ty,
    )
}
/// `Parsec ε α` applied to two type arguments.
#[allow(dead_code)]
pub fn parsec_of(err_ty: Expr, val_ty: Expr) -> Expr {
    app2(Expr::Const(Name::str("Parsec"), vec![]), err_ty, val_ty)
}
/// Shorthand to add an axiom to env.
#[allow(dead_code)]
pub fn add_axiom(
    env: &mut Environment,
    name: &str,
    univ_params: Vec<Name>,
    ty: Expr,
) -> Result<(), String> {
    env.add(Declaration::Axiom {
        name: Name::str(name),
        univ_params,
        ty,
    })
    .map_err(|e| e.to_string())
}
/// Build the Parsec parser combinator library in the environment.
///
/// Assumes that `String`, `Char`, `Nat`, `Int`, `Bool`, `Unit`, `List`,
/// `Option`, `Inhabited`, and `Eq` are already declared in `env` or
/// referenced by name.
pub fn build_parsec_env(env: &mut Environment) -> Result<(), String> {
    add_axiom(env, "String.Pos", vec![], type1())?;
    let parse_result_ty = default_pi("ε", type1(), default_pi("α", type1(), type1()));
    add_axiom(env, "ParseResult", vec![], parse_result_ty)?;
    let success_ty = implicit_pi(
        "ε",
        type1(),
        implicit_pi(
            "α",
            type1(),
            default_pi(
                "pos",
                string_pos_ty(),
                default_pi(
                    "val",
                    Expr::BVar(1),
                    parse_result_of(Expr::BVar(3), Expr::BVar(2)),
                ),
            ),
        ),
    );
    add_axiom(env, "ParseResult.success", vec![], success_ty)?;
    let error_ty = implicit_pi(
        "ε",
        type1(),
        implicit_pi(
            "α",
            type1(),
            default_pi(
                "pos",
                string_pos_ty(),
                default_pi(
                    "err",
                    Expr::BVar(2),
                    parse_result_of(Expr::BVar(3), Expr::BVar(2)),
                ),
            ),
        ),
    );
    add_axiom(env, "ParseResult.error", vec![], error_ty)?;
    let parsec_ty = default_pi("ε", type1(), default_pi("α", type1(), type1()));
    add_axiom(env, "Parsec", vec![], parsec_ty)?;
    let run_ty = implicit_pi(
        "ε",
        type1(),
        implicit_pi(
            "α",
            type1(),
            default_pi(
                "p",
                parsec_of(Expr::BVar(1), Expr::BVar(0)),
                default_pi(
                    "s",
                    string_ty(),
                    parse_result_of(Expr::BVar(3), Expr::BVar(2)),
                ),
            ),
        ),
    );
    add_axiom(env, "Parsec.run", vec![], run_ty)?;
    let pure_ty = implicit_pi(
        "ε",
        type1(),
        implicit_pi(
            "α",
            type1(),
            default_pi("a", Expr::BVar(0), parsec_of(Expr::BVar(2), Expr::BVar(1))),
        ),
    );
    add_axiom(env, "Parsec.pure", vec![], pure_ty)?;
    let fail_ty = implicit_pi(
        "ε",
        type1(),
        implicit_pi(
            "α",
            type1(),
            default_pi("e", Expr::BVar(1), parsec_of(Expr::BVar(2), Expr::BVar(1))),
        ),
    );
    add_axiom(env, "Parsec.fail", vec![], fail_ty)?;
    let bind_ty = implicit_pi(
        "ε",
        type1(),
        implicit_pi(
            "α",
            type1(),
            implicit_pi(
                "β",
                type1(),
                default_pi(
                    "p",
                    parsec_of(Expr::BVar(2), Expr::BVar(1)),
                    default_pi(
                        "f",
                        arrow(Expr::BVar(2), parsec_of(Expr::BVar(4), Expr::BVar(2))),
                        parsec_of(Expr::BVar(4), Expr::BVar(2)),
                    ),
                ),
            ),
        ),
    );
    add_axiom(env, "Parsec.bind", vec![], bind_ty)?;
    let map_ty = implicit_pi(
        "ε",
        type1(),
        implicit_pi(
            "α",
            type1(),
            implicit_pi(
                "β",
                type1(),
                default_pi(
                    "f",
                    arrow(Expr::BVar(1), Expr::BVar(1)),
                    default_pi(
                        "p",
                        parsec_of(Expr::BVar(3), Expr::BVar(2)),
                        parsec_of(Expr::BVar(4), Expr::BVar(2)),
                    ),
                ),
            ),
        ),
    );
    add_axiom(env, "Parsec.map", vec![], map_ty)?;
    let seq_ty = implicit_pi(
        "ε",
        type1(),
        implicit_pi(
            "α",
            type1(),
            implicit_pi(
                "β",
                type1(),
                default_pi(
                    "pf",
                    parsec_of(Expr::BVar(2), arrow(Expr::BVar(1), Expr::BVar(1))),
                    default_pi(
                        "pa",
                        parsec_of(Expr::BVar(3), Expr::BVar(2)),
                        parsec_of(Expr::BVar(4), Expr::BVar(2)),
                    ),
                ),
            ),
        ),
    );
    add_axiom(env, "Parsec.seq", vec![], seq_ty)?;
    let or_else_ty = implicit_pi(
        "ε",
        type1(),
        implicit_pi(
            "α",
            type1(),
            default_pi(
                "p1",
                parsec_of(Expr::BVar(1), Expr::BVar(0)),
                default_pi(
                    "p2",
                    parsec_of(Expr::BVar(2), Expr::BVar(1)),
                    parsec_of(Expr::BVar(3), Expr::BVar(2)),
                ),
            ),
        ),
    );
    add_axiom(env, "Parsec.orElse", vec![], or_else_ty)?;
    let any_ty = implicit_pi(
        "ε",
        type1(),
        inst_pi(
            "_inst",
            inhabited_of(Expr::BVar(0)),
            parsec_of(Expr::BVar(1), char_ty()),
        ),
    );
    add_axiom(env, "Parsec.any", vec![], any_ty)?;
    let satisfy_ty = implicit_pi(
        "ε",
        type1(),
        inst_pi(
            "_inst",
            inhabited_of(Expr::BVar(0)),
            default_pi(
                "pred",
                arrow(char_ty(), bool_ty()),
                parsec_of(Expr::BVar(2), char_ty()),
            ),
        ),
    );
    add_axiom(env, "Parsec.satisfy", vec![], satisfy_ty)?;
    let char_parser_ty = implicit_pi(
        "ε",
        type1(),
        inst_pi(
            "_inst",
            inhabited_of(Expr::BVar(0)),
            default_pi("c", char_ty(), parsec_of(Expr::BVar(2), char_ty())),
        ),
    );
    add_axiom(env, "Parsec.char", vec![], char_parser_ty)?;
    let digit_ty = implicit_pi(
        "ε",
        type1(),
        inst_pi(
            "_inst",
            inhabited_of(Expr::BVar(0)),
            parsec_of(Expr::BVar(1), char_ty()),
        ),
    );
    add_axiom(env, "Parsec.digit", vec![], digit_ty)?;
    let alpha_ty = implicit_pi(
        "ε",
        type1(),
        inst_pi(
            "_inst",
            inhabited_of(Expr::BVar(0)),
            parsec_of(Expr::BVar(1), char_ty()),
        ),
    );
    add_axiom(env, "Parsec.alpha", vec![], alpha_ty)?;
    let alpha_num_ty = implicit_pi(
        "ε",
        type1(),
        inst_pi(
            "_inst",
            inhabited_of(Expr::BVar(0)),
            parsec_of(Expr::BVar(1), char_ty()),
        ),
    );
    add_axiom(env, "Parsec.alphaNum", vec![], alpha_num_ty)?;
    let ws_ty = implicit_pi(
        "ε",
        type1(),
        inst_pi(
            "_inst",
            inhabited_of(Expr::BVar(0)),
            parsec_of(Expr::BVar(1), unit_ty()),
        ),
    );
    add_axiom(env, "Parsec.ws", vec![], ws_ty)?;
    let string_parser_ty = implicit_pi(
        "ε",
        type1(),
        inst_pi(
            "_inst",
            inhabited_of(Expr::BVar(0)),
            default_pi("s", string_ty(), parsec_of(Expr::BVar(2), string_ty())),
        ),
    );
    add_axiom(env, "Parsec.string", vec![], string_parser_ty)?;
    let eof_ty = implicit_pi(
        "ε",
        type1(),
        inst_pi(
            "_inst",
            inhabited_of(Expr::BVar(0)),
            parsec_of(Expr::BVar(1), unit_ty()),
        ),
    );
    add_axiom(env, "Parsec.eof", vec![], eof_ty)?;
    let many_ty = implicit_pi(
        "ε",
        type1(),
        implicit_pi(
            "α",
            type1(),
            default_pi(
                "p",
                parsec_of(Expr::BVar(1), Expr::BVar(0)),
                parsec_of(Expr::BVar(2), list_of(Expr::BVar(1))),
            ),
        ),
    );
    add_axiom(env, "Parsec.many", vec![], many_ty)?;
    let many1_ty = implicit_pi(
        "ε",
        type1(),
        implicit_pi(
            "α",
            type1(),
            default_pi(
                "p",
                parsec_of(Expr::BVar(1), Expr::BVar(0)),
                parsec_of(Expr::BVar(2), list_of(Expr::BVar(1))),
            ),
        ),
    );
    add_axiom(env, "Parsec.many1", vec![], many1_ty)?;
    let optional_ty = implicit_pi(
        "ε",
        type1(),
        implicit_pi(
            "α",
            type1(),
            default_pi(
                "p",
                parsec_of(Expr::BVar(1), Expr::BVar(0)),
                parsec_of(Expr::BVar(2), option_of(Expr::BVar(1))),
            ),
        ),
    );
    add_axiom(env, "Parsec.optional", vec![], optional_ty)?;
    let sep_by_ty = implicit_pi(
        "ε",
        type1(),
        implicit_pi(
            "α",
            type1(),
            implicit_pi(
                "β",
                type1(),
                default_pi(
                    "p",
                    parsec_of(Expr::BVar(2), Expr::BVar(1)),
                    default_pi(
                        "sep",
                        parsec_of(Expr::BVar(3), Expr::BVar(1)),
                        parsec_of(Expr::BVar(4), list_of(Expr::BVar(3))),
                    ),
                ),
            ),
        ),
    );
    add_axiom(env, "Parsec.sepBy", vec![], sep_by_ty)?;
    let sep_by1_ty = implicit_pi(
        "ε",
        type1(),
        implicit_pi(
            "α",
            type1(),
            implicit_pi(
                "β",
                type1(),
                default_pi(
                    "p",
                    parsec_of(Expr::BVar(2), Expr::BVar(1)),
                    default_pi(
                        "sep",
                        parsec_of(Expr::BVar(3), Expr::BVar(1)),
                        parsec_of(Expr::BVar(4), list_of(Expr::BVar(3))),
                    ),
                ),
            ),
        ),
    );
    add_axiom(env, "Parsec.sepBy1", vec![], sep_by1_ty)?;
    let between_ty = implicit_pi(
        "ε",
        type1(),
        implicit_pi(
            "α",
            type1(),
            implicit_pi(
                "β",
                type1(),
                implicit_pi(
                    "γ",
                    type1(),
                    default_pi(
                        "open",
                        parsec_of(Expr::BVar(3), Expr::BVar(2)),
                        default_pi(
                            "close",
                            parsec_of(Expr::BVar(4), Expr::BVar(2)),
                            default_pi(
                                "p",
                                parsec_of(Expr::BVar(5), Expr::BVar(2)),
                                parsec_of(Expr::BVar(6), Expr::BVar(3)),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    );
    add_axiom(env, "Parsec.between", vec![], between_ty)?;
    let not_followed_by_ty = implicit_pi(
        "ε",
        type1(),
        implicit_pi(
            "α",
            type1(),
            default_pi(
                "p",
                parsec_of(Expr::BVar(1), Expr::BVar(0)),
                parsec_of(Expr::BVar(2), unit_ty()),
            ),
        ),
    );
    add_axiom(env, "Parsec.notFollowedBy", vec![], not_followed_by_ty)?;
    let lookahead_ty = implicit_pi(
        "ε",
        type1(),
        implicit_pi(
            "α",
            type1(),
            default_pi(
                "p",
                parsec_of(Expr::BVar(1), Expr::BVar(0)),
                parsec_of(Expr::BVar(2), Expr::BVar(1)),
            ),
        ),
    );
    add_axiom(env, "Parsec.lookahead", vec![], lookahead_ty)?;
    let chainl_ty = implicit_pi(
        "ε",
        type1(),
        implicit_pi(
            "α",
            type1(),
            default_pi(
                "p",
                parsec_of(Expr::BVar(1), Expr::BVar(0)),
                default_pi(
                    "op",
                    parsec_of(
                        Expr::BVar(2),
                        arrow(Expr::BVar(1), arrow(Expr::BVar(2), Expr::BVar(3))),
                    ),
                    default_pi(
                        "def",
                        Expr::BVar(2),
                        parsec_of(Expr::BVar(4), Expr::BVar(3)),
                    ),
                ),
            ),
        ),
    );
    add_axiom(env, "Parsec.chainl", vec![], chainl_ty)?;
    let chainr_ty = implicit_pi(
        "ε",
        type1(),
        implicit_pi(
            "α",
            type1(),
            default_pi(
                "p",
                parsec_of(Expr::BVar(1), Expr::BVar(0)),
                default_pi(
                    "op",
                    parsec_of(
                        Expr::BVar(2),
                        arrow(Expr::BVar(1), arrow(Expr::BVar(2), Expr::BVar(3))),
                    ),
                    default_pi(
                        "def",
                        Expr::BVar(2),
                        parsec_of(Expr::BVar(4), Expr::BVar(3)),
                    ),
                ),
            ),
        ),
    );
    add_axiom(env, "Parsec.chainr", vec![], chainr_ty)?;
    let nat_parser_ty = implicit_pi(
        "ε",
        type1(),
        inst_pi(
            "_inst",
            inhabited_of(Expr::BVar(0)),
            parsec_of(Expr::BVar(1), nat_ty()),
        ),
    );
    add_axiom(env, "Parsec.nat", vec![], nat_parser_ty)?;
    let int_parser_ty = implicit_pi(
        "ε",
        type1(),
        inst_pi(
            "_inst",
            inhabited_of(Expr::BVar(0)),
            parsec_of(Expr::BVar(1), int_ty()),
        ),
    );
    add_axiom(env, "Parsec.int", vec![], int_parser_ty)?;
    let hex_digit_ty = implicit_pi(
        "ε",
        type1(),
        inst_pi(
            "_inst",
            inhabited_of(Expr::BVar(0)),
            parsec_of(Expr::BVar(1), char_ty()),
        ),
    );
    add_axiom(env, "Parsec.hexDigit", vec![], hex_digit_ty)?;
    let hex_nat_ty = implicit_pi(
        "ε",
        type1(),
        inst_pi(
            "_inst",
            inhabited_of(Expr::BVar(0)),
            parsec_of(Expr::BVar(1), nat_ty()),
        ),
    );
    add_axiom(env, "Parsec.hexNat", vec![], hex_nat_ty)?;
    let token_ty = implicit_pi(
        "ε",
        type1(),
        implicit_pi(
            "α",
            type1(),
            inst_pi(
                "_inst",
                inhabited_of(Expr::BVar(1)),
                default_pi(
                    "p",
                    parsec_of(Expr::BVar(2), Expr::BVar(1)),
                    parsec_of(Expr::BVar(3), Expr::BVar(2)),
                ),
            ),
        ),
    );
    add_axiom(env, "Parsec.token", vec![], token_ty)?;
    let symbol_ty = implicit_pi(
        "ε",
        type1(),
        inst_pi(
            "_inst",
            inhabited_of(Expr::BVar(0)),
            default_pi("s", string_ty(), parsec_of(Expr::BVar(2), string_ty())),
        ),
    );
    add_axiom(env, "Parsec.symbol", vec![], symbol_ty)?;
    let pure_bind_ty = implicit_pi(
        "ε",
        type1(),
        implicit_pi(
            "α",
            type1(),
            implicit_pi(
                "β",
                type1(),
                default_pi(
                    "a",
                    Expr::BVar(1),
                    default_pi(
                        "f",
                        arrow(Expr::BVar(2), parsec_of(Expr::BVar(4), Expr::BVar(2))),
                        eq_expr(
                            parsec_of(Expr::BVar(4), Expr::BVar(2)),
                            app2(
                                Expr::Const(Name::str("Parsec.bind"), vec![]),
                                app(Expr::Const(Name::str("Parsec.pure"), vec![]), Expr::BVar(1)),
                                Expr::BVar(0),
                            ),
                            app(Expr::BVar(0), Expr::BVar(1)),
                        ),
                    ),
                ),
            ),
        ),
    );
    add_axiom(env, "Parsec.pure_bind", vec![], pure_bind_ty)?;
    let bind_pure_ty = implicit_pi(
        "ε",
        type1(),
        implicit_pi(
            "α",
            type1(),
            default_pi(
                "p",
                parsec_of(Expr::BVar(1), Expr::BVar(0)),
                eq_expr(
                    parsec_of(Expr::BVar(2), Expr::BVar(1)),
                    app2(
                        Expr::Const(Name::str("Parsec.bind"), vec![]),
                        Expr::BVar(0),
                        Expr::Const(Name::str("Parsec.pure"), vec![]),
                    ),
                    Expr::BVar(0),
                ),
            ),
        ),
    );
    add_axiom(env, "Parsec.bind_pure", vec![], bind_pure_ty)?;
    let bind_assoc_ty = implicit_pi(
        "ε",
        type1(),
        implicit_pi(
            "α",
            type1(),
            implicit_pi(
                "β",
                type1(),
                implicit_pi(
                    "γ",
                    type1(),
                    default_pi(
                        "p",
                        parsec_of(Expr::BVar(3), Expr::BVar(2)),
                        default_pi(
                            "f",
                            arrow(Expr::BVar(3), parsec_of(Expr::BVar(5), Expr::BVar(3))),
                            default_pi(
                                "g",
                                arrow(Expr::BVar(3), parsec_of(Expr::BVar(6), Expr::BVar(3))),
                                eq_expr(
                                    parsec_of(Expr::BVar(6), Expr::BVar(3)),
                                    app2(
                                        Expr::Const(Name::str("Parsec.bind"), vec![]),
                                        app2(
                                            Expr::Const(Name::str("Parsec.bind"), vec![]),
                                            Expr::BVar(2),
                                            Expr::BVar(1),
                                        ),
                                        Expr::BVar(0),
                                    ),
                                    app2(
                                        Expr::Const(Name::str("Parsec.bind"), vec![]),
                                        Expr::BVar(2),
                                        Expr::Const(Name::str("Parsec.bind_assoc.rhs"), vec![]),
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    );
    add_axiom(env, "Parsec.bind_assoc", vec![], bind_assoc_ty)?;
    let map_pure_ty = implicit_pi(
        "ε",
        type1(),
        implicit_pi(
            "α",
            type1(),
            implicit_pi(
                "β",
                type1(),
                default_pi(
                    "f",
                    arrow(Expr::BVar(1), Expr::BVar(1)),
                    default_pi(
                        "a",
                        Expr::BVar(2),
                        eq_expr(
                            parsec_of(Expr::BVar(4), Expr::BVar(2)),
                            app2(
                                Expr::Const(Name::str("Parsec.map"), vec![]),
                                Expr::BVar(1),
                                app(Expr::Const(Name::str("Parsec.pure"), vec![]), Expr::BVar(0)),
                            ),
                            app(
                                Expr::Const(Name::str("Parsec.pure"), vec![]),
                                app(Expr::BVar(1), Expr::BVar(0)),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    );
    add_axiom(env, "Parsec.map_pure", vec![], map_pure_ty)?;
    let map_bind_ty = implicit_pi(
        "ε",
        type1(),
        implicit_pi(
            "α",
            type1(),
            implicit_pi(
                "β",
                type1(),
                implicit_pi(
                    "γ",
                    type1(),
                    default_pi(
                        "f",
                        arrow(Expr::BVar(1), Expr::BVar(1)),
                        default_pi(
                            "p",
                            parsec_of(Expr::BVar(4), Expr::BVar(3)),
                            default_pi(
                                "g",
                                arrow(Expr::BVar(4), parsec_of(Expr::BVar(6), Expr::BVar(4))),
                                eq_expr(
                                    parsec_of(Expr::BVar(6), Expr::BVar(3)),
                                    app2(
                                        Expr::Const(Name::str("Parsec.map"), vec![]),
                                        Expr::BVar(2),
                                        app2(
                                            Expr::Const(Name::str("Parsec.bind"), vec![]),
                                            Expr::BVar(1),
                                            Expr::BVar(0),
                                        ),
                                    ),
                                    app2(
                                        Expr::Const(Name::str("Parsec.bind"), vec![]),
                                        Expr::BVar(1),
                                        Expr::Const(Name::str("Parsec.map_bind.rhs"), vec![]),
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    );
    add_axiom(env, "Parsec.map_bind", vec![], map_bind_ty)?;
    let or_else_pure_left_ty = implicit_pi(
        "ε",
        type1(),
        implicit_pi(
            "α",
            type1(),
            default_pi(
                "a",
                Expr::BVar(0),
                default_pi(
                    "p",
                    parsec_of(Expr::BVar(2), Expr::BVar(1)),
                    eq_expr(
                        parsec_of(Expr::BVar(3), Expr::BVar(2)),
                        app2(
                            Expr::Const(Name::str("Parsec.orElse"), vec![]),
                            app(Expr::Const(Name::str("Parsec.pure"), vec![]), Expr::BVar(1)),
                            Expr::BVar(0),
                        ),
                        app(Expr::Const(Name::str("Parsec.pure"), vec![]), Expr::BVar(1)),
                    ),
                ),
            ),
        ),
    );
    add_axiom(env, "Parsec.orElse_pure_left", vec![], or_else_pure_left_ty)?;
    let or_else_fail_left_ty = implicit_pi(
        "ε",
        type1(),
        implicit_pi(
            "α",
            type1(),
            default_pi(
                "e",
                Expr::BVar(1),
                default_pi(
                    "p",
                    parsec_of(Expr::BVar(2), Expr::BVar(1)),
                    eq_expr(
                        parsec_of(Expr::BVar(3), Expr::BVar(2)),
                        app2(
                            Expr::Const(Name::str("Parsec.orElse"), vec![]),
                            app(Expr::Const(Name::str("Parsec.fail"), vec![]), Expr::BVar(1)),
                            Expr::BVar(0),
                        ),
                        Expr::BVar(0),
                    ),
                ),
            ),
        ),
    );
    add_axiom(env, "Parsec.orElse_fail_left", vec![], or_else_fail_left_ty)?;
    Ok(())
}
/// Build `ParseResult ε α` from error and value type expressions.
#[allow(dead_code)]
pub fn mk_parse_result(err_ty: Expr, val_ty: Expr) -> Expr {
    parse_result_of(err_ty, val_ty)
}
/// Build `Parsec ε α` from error and value type expressions.
#[allow(dead_code)]
pub fn mk_parsec(err_ty: Expr, val_ty: Expr) -> Expr {
    parsec_of(err_ty, val_ty)
}
/// Build `Parsec.pure a`.
#[allow(dead_code)]
pub fn mk_parsec_pure(val: Expr) -> Expr {
    app(Expr::Const(Name::str("Parsec.pure"), vec![]), val)
}
/// Build `Parsec.bind p f`.
#[allow(dead_code)]
pub fn mk_parsec_bind(p: Expr, f: Expr) -> Expr {
    app2(Expr::Const(Name::str("Parsec.bind"), vec![]), p, f)
}
/// Build `Parsec.map f p`.
#[allow(dead_code)]
pub fn mk_parsec_map(f: Expr, p: Expr) -> Expr {
    app2(Expr::Const(Name::str("Parsec.map"), vec![]), f, p)
}
/// Build `Parsec.many p`.
#[allow(dead_code)]
pub fn mk_parsec_many(p: Expr) -> Expr {
    app(Expr::Const(Name::str("Parsec.many"), vec![]), p)
}
/// Build `Parsec.char c`.
#[allow(dead_code)]
pub fn mk_parsec_char(c: Expr) -> Expr {
    app(Expr::Const(Name::str("Parsec.char"), vec![]), c)
}
/// Build `Parsec.seq pf pa`.
#[allow(dead_code)]
pub fn mk_parsec_seq(pf: Expr, pa: Expr) -> Expr {
    app2(Expr::Const(Name::str("Parsec.seq"), vec![]), pf, pa)
}
/// Build `Parsec.orElse p1 p2`.
#[allow(dead_code)]
pub fn mk_parsec_or_else(p1: Expr, p2: Expr) -> Expr {
    app2(Expr::Const(Name::str("Parsec.orElse"), vec![]), p1, p2)
}
/// Build `Parsec.many1 p`.
#[allow(dead_code)]
pub fn mk_parsec_many1(p: Expr) -> Expr {
    app(Expr::Const(Name::str("Parsec.many1"), vec![]), p)
}
/// Build `Parsec.optional p`.
#[allow(dead_code)]
pub fn mk_parsec_optional(p: Expr) -> Expr {
    app(Expr::Const(Name::str("Parsec.optional"), vec![]), p)
}
/// Build `Parsec.sepBy p sep`.
#[allow(dead_code)]
pub fn mk_parsec_sep_by(p: Expr, sep: Expr) -> Expr {
    app2(Expr::Const(Name::str("Parsec.sepBy"), vec![]), p, sep)
}
/// Build `Parsec.between open close p`.
#[allow(dead_code)]
pub fn mk_parsec_between(open: Expr, close: Expr, p: Expr) -> Expr {
    app3(
        Expr::Const(Name::str("Parsec.between"), vec![]),
        open,
        close,
        p,
    )
}
/// Build `Parsec.token p`.
#[allow(dead_code)]
pub fn mk_parsec_token(p: Expr) -> Expr {
    app(Expr::Const(Name::str("Parsec.token"), vec![]), p)
}
/// Build `Parsec.symbol s`.
#[allow(dead_code)]
pub fn mk_parsec_symbol(s: Expr) -> Expr {
    app(Expr::Const(Name::str("Parsec.symbol"), vec![]), s)
}
/// Build `Parsec.run p s`.
#[allow(dead_code)]
pub fn mk_parsec_run(p: Expr, s: Expr) -> Expr {
    app2(Expr::Const(Name::str("Parsec.run"), vec![]), p, s)
}
/// Build `Parsec.fail e`.
#[allow(dead_code)]
pub fn mk_parsec_fail(e: Expr) -> Expr {
    app(Expr::Const(Name::str("Parsec.fail"), vec![]), e)
}
/// Build `Parsec.string s`.
#[allow(dead_code)]
pub fn mk_parsec_string(s: Expr) -> Expr {
    app(Expr::Const(Name::str("Parsec.string"), vec![]), s)
}
/// Build `Parsec.satisfy pred`.
#[allow(dead_code)]
pub fn mk_parsec_satisfy(pred: Expr) -> Expr {
    app(Expr::Const(Name::str("Parsec.satisfy"), vec![]), pred)
}
/// Build `Parsec.lookahead p`.
#[allow(dead_code)]
pub fn mk_parsec_lookahead(p: Expr) -> Expr {
    app(Expr::Const(Name::str("Parsec.lookahead"), vec![]), p)
}
/// Build `Parsec.notFollowedBy p`.
#[allow(dead_code)]
pub fn mk_parsec_not_followed_by(p: Expr) -> Expr {
    app(Expr::Const(Name::str("Parsec.notFollowedBy"), vec![]), p)
}
/// Build `Parsec.chainl p op def`.
#[allow(dead_code)]
pub fn mk_parsec_chainl(p: Expr, op: Expr, def: Expr) -> Expr {
    app3(Expr::Const(Name::str("Parsec.chainl"), vec![]), p, op, def)
}
/// Build `Parsec.chainr p op def`.
#[allow(dead_code)]
pub fn mk_parsec_chainr(p: Expr, op: Expr, def: Expr) -> Expr {
    app3(Expr::Const(Name::str("Parsec.chainr"), vec![]), p, op, def)
}
#[cfg(test)]
mod tests {
    use super::*;
    fn make_env() -> Environment {
        let mut env = Environment::new();
        build_parsec_env(&mut env).expect("build_parsec_env should succeed");
        env
    }
    #[test]
    fn test_build_parsec_env_succeeds() {
        let mut env = Environment::new();
        assert!(build_parsec_env(&mut env).is_ok());
    }
    #[test]
    fn test_parse_result_exists() {
        let env = make_env();
        assert!(env.get(&Name::str("ParseResult")).is_some());
    }
    #[test]
    fn test_parse_result_success_exists() {
        let env = make_env();
        assert!(env.get(&Name::str("ParseResult.success")).is_some());
    }
    #[test]
    fn test_parse_result_error_exists() {
        let env = make_env();
        assert!(env.get(&Name::str("ParseResult.error")).is_some());
    }
    #[test]
    fn test_parsec_type_exists() {
        let env = make_env();
        assert!(env.get(&Name::str("Parsec")).is_some());
    }
    #[test]
    fn test_parsec_run_exists() {
        let env = make_env();
        assert!(env.get(&Name::str("Parsec.run")).is_some());
    }
    #[test]
    fn test_parsec_pure_type_is_pi() {
        let env = make_env();
        let decl = env
            .get(&Name::str("Parsec.pure"))
            .expect("declaration 'Parsec.pure' should exist in env");
        assert!(decl.ty().is_pi());
    }
    #[test]
    fn test_parsec_fail_type_is_pi() {
        let env = make_env();
        let decl = env
            .get(&Name::str("Parsec.fail"))
            .expect("declaration 'Parsec.fail' should exist in env");
        assert!(decl.ty().is_pi());
    }
    #[test]
    fn test_parsec_bind_type_is_pi() {
        let env = make_env();
        let decl = env
            .get(&Name::str("Parsec.bind"))
            .expect("declaration 'Parsec.bind' should exist in env");
        assert!(decl.ty().is_pi());
    }
    #[test]
    fn test_parsec_map_type_is_pi() {
        let env = make_env();
        let decl = env
            .get(&Name::str("Parsec.map"))
            .expect("declaration 'Parsec.map' should exist in env");
        assert!(decl.ty().is_pi());
    }
    #[test]
    fn test_parsec_seq_type_is_pi() {
        let env = make_env();
        let decl = env
            .get(&Name::str("Parsec.seq"))
            .expect("declaration 'Parsec.seq' should exist in env");
        assert!(decl.ty().is_pi());
    }
    #[test]
    fn test_parsec_or_else_type_is_pi() {
        let env = make_env();
        let decl = env
            .get(&Name::str("Parsec.orElse"))
            .expect("declaration 'Parsec.orElse' should exist in env");
        assert!(decl.ty().is_pi());
    }
    #[test]
    fn test_parsec_any_exists() {
        let env = make_env();
        assert!(env.get(&Name::str("Parsec.any")).is_some());
    }
    #[test]
    fn test_parsec_satisfy_exists() {
        let env = make_env();
        assert!(env.get(&Name::str("Parsec.satisfy")).is_some());
    }
    #[test]
    fn test_parsec_char_exists() {
        let env = make_env();
        assert!(env.get(&Name::str("Parsec.char")).is_some());
    }
    #[test]
    fn test_parsec_digit_exists() {
        let env = make_env();
        assert!(env.get(&Name::str("Parsec.digit")).is_some());
    }
    #[test]
    fn test_parsec_alpha_exists() {
        let env = make_env();
        assert!(env.get(&Name::str("Parsec.alpha")).is_some());
    }
    #[test]
    fn test_parsec_alpha_num_exists() {
        let env = make_env();
        assert!(env.get(&Name::str("Parsec.alphaNum")).is_some());
    }
    #[test]
    fn test_parsec_ws_exists() {
        let env = make_env();
        assert!(env.get(&Name::str("Parsec.ws")).is_some());
    }
    #[test]
    fn test_parsec_string_exists() {
        let env = make_env();
        assert!(env.get(&Name::str("Parsec.string")).is_some());
    }
    #[test]
    fn test_parsec_eof_exists() {
        let env = make_env();
        assert!(env.get(&Name::str("Parsec.eof")).is_some());
    }
    #[test]
    fn test_parsec_many_exists() {
        let env = make_env();
        assert!(env.get(&Name::str("Parsec.many")).is_some());
    }
    #[test]
    fn test_parsec_many1_exists() {
        let env = make_env();
        assert!(env.get(&Name::str("Parsec.many1")).is_some());
    }
    #[test]
    fn test_parsec_optional_exists() {
        let env = make_env();
        assert!(env.get(&Name::str("Parsec.optional")).is_some());
    }
    #[test]
    fn test_parsec_sep_by_exists() {
        let env = make_env();
        assert!(env.get(&Name::str("Parsec.sepBy")).is_some());
    }
    #[test]
    fn test_parsec_sep_by1_exists() {
        let env = make_env();
        assert!(env.get(&Name::str("Parsec.sepBy1")).is_some());
    }
    #[test]
    fn test_parsec_between_exists() {
        let env = make_env();
        assert!(env.get(&Name::str("Parsec.between")).is_some());
    }
    #[test]
    fn test_parsec_not_followed_by_exists() {
        let env = make_env();
        assert!(env.get(&Name::str("Parsec.notFollowedBy")).is_some());
    }
    #[test]
    fn test_parsec_lookahead_exists() {
        let env = make_env();
        assert!(env.get(&Name::str("Parsec.lookahead")).is_some());
    }
    #[test]
    fn test_parsec_chainl_exists() {
        let env = make_env();
        assert!(env.get(&Name::str("Parsec.chainl")).is_some());
    }
    #[test]
    fn test_parsec_chainr_exists() {
        let env = make_env();
        assert!(env.get(&Name::str("Parsec.chainr")).is_some());
    }
    #[test]
    fn test_parsec_nat_exists() {
        let env = make_env();
        assert!(env.get(&Name::str("Parsec.nat")).is_some());
    }
    #[test]
    fn test_parsec_int_exists() {
        let env = make_env();
        assert!(env.get(&Name::str("Parsec.int")).is_some());
    }
    #[test]
    fn test_parsec_hex_digit_exists() {
        let env = make_env();
        assert!(env.get(&Name::str("Parsec.hexDigit")).is_some());
    }
    #[test]
    fn test_parsec_hex_nat_exists() {
        let env = make_env();
        assert!(env.get(&Name::str("Parsec.hexNat")).is_some());
    }
    #[test]
    fn test_parsec_token_exists() {
        let env = make_env();
        assert!(env.get(&Name::str("Parsec.token")).is_some());
    }
    #[test]
    fn test_parsec_symbol_exists() {
        let env = make_env();
        assert!(env.get(&Name::str("Parsec.symbol")).is_some());
    }
    #[test]
    fn test_parsec_pure_bind_exists() {
        let env = make_env();
        assert!(env.get(&Name::str("Parsec.pure_bind")).is_some());
    }
    #[test]
    fn test_parsec_bind_pure_exists() {
        let env = make_env();
        assert!(env.get(&Name::str("Parsec.bind_pure")).is_some());
    }
    #[test]
    fn test_parsec_bind_assoc_exists() {
        let env = make_env();
        assert!(env.get(&Name::str("Parsec.bind_assoc")).is_some());
    }
    #[test]
    fn test_parsec_map_pure_exists() {
        let env = make_env();
        assert!(env.get(&Name::str("Parsec.map_pure")).is_some());
    }
    #[test]
    fn test_parsec_map_bind_exists() {
        let env = make_env();
        assert!(env.get(&Name::str("Parsec.map_bind")).is_some());
    }
    #[test]
    fn test_parsec_or_else_pure_left_exists() {
        let env = make_env();
        assert!(env.get(&Name::str("Parsec.orElse_pure_left")).is_some());
    }
    #[test]
    fn test_parsec_or_else_fail_left_exists() {
        let env = make_env();
        assert!(env.get(&Name::str("Parsec.orElse_fail_left")).is_some());
    }
    #[test]
    fn test_mk_parse_result() {
        let r = mk_parse_result(string_ty(), nat_ty());
        assert!(matches!(r, Expr::App(_, _)));
    }
    #[test]
    fn test_mk_parsec() {
        let p = mk_parsec(string_ty(), nat_ty());
        assert!(matches!(p, Expr::App(_, _)));
    }
    #[test]
    fn test_mk_parsec_pure() {
        let p = mk_parsec_pure(Expr::Const(Name::str("x"), vec![]));
        assert!(matches!(p, Expr::App(_, _)));
    }
    #[test]
    fn test_mk_parsec_bind() {
        let p = mk_parsec_bind(
            Expr::Const(Name::str("p"), vec![]),
            Expr::Const(Name::str("f"), vec![]),
        );
        assert!(matches!(p, Expr::App(_, _)));
    }
    #[test]
    fn test_mk_parsec_map() {
        let m = mk_parsec_map(
            Expr::Const(Name::str("f"), vec![]),
            Expr::Const(Name::str("p"), vec![]),
        );
        assert!(matches!(m, Expr::App(_, _)));
    }
    #[test]
    fn test_mk_parsec_many() {
        let m = mk_parsec_many(Expr::Const(Name::str("p"), vec![]));
        assert!(matches!(m, Expr::App(_, _)));
    }
    #[test]
    fn test_mk_parsec_char() {
        let c = mk_parsec_char(Expr::Const(Name::str("c"), vec![]));
        assert!(matches!(c, Expr::App(_, _)));
    }
    #[test]
    fn test_all_declarations_are_axioms() {
        let env = make_env();
        for name in [
            "ParseResult",
            "ParseResult.success",
            "ParseResult.error",
            "Parsec",
            "Parsec.run",
            "Parsec.pure",
            "Parsec.fail",
            "Parsec.bind",
            "Parsec.map",
            "Parsec.seq",
            "Parsec.orElse",
            "Parsec.any",
            "Parsec.satisfy",
            "Parsec.char",
            "Parsec.digit",
            "Parsec.alpha",
            "Parsec.alphaNum",
            "Parsec.ws",
            "Parsec.string",
            "Parsec.eof",
            "Parsec.many",
            "Parsec.many1",
            "Parsec.optional",
            "Parsec.sepBy",
            "Parsec.sepBy1",
            "Parsec.between",
            "Parsec.notFollowedBy",
            "Parsec.lookahead",
            "Parsec.chainl",
            "Parsec.chainr",
            "Parsec.nat",
            "Parsec.int",
            "Parsec.hexDigit",
            "Parsec.hexNat",
            "Parsec.token",
            "Parsec.symbol",
        ] {
            let decl = env.get(&Name::str(name)).expect("operation should succeed");
            assert!(
                matches!(decl, Declaration::Axiom { .. }),
                "{} should be an axiom",
                name
            );
        }
    }
    #[test]
    fn test_env_declaration_count() {
        let env = make_env();
        assert!(env.len() >= 40);
    }
    #[test]
    fn test_mk_parsec_seq_is_app() {
        let s = mk_parsec_seq(
            Expr::Const(Name::str("pf"), vec![]),
            Expr::Const(Name::str("pa"), vec![]),
        );
        assert!(matches!(s, Expr::App(_, _)));
    }
    #[test]
    fn test_mk_parsec_or_else_is_app() {
        let o = mk_parsec_or_else(
            Expr::Const(Name::str("p1"), vec![]),
            Expr::Const(Name::str("p2"), vec![]),
        );
        assert!(matches!(o, Expr::App(_, _)));
    }
    #[test]
    fn test_mk_parsec_run_is_app() {
        let r = mk_parsec_run(
            Expr::Const(Name::str("p"), vec![]),
            Expr::Const(Name::str("s"), vec![]),
        );
        assert!(matches!(r, Expr::App(_, _)));
    }
    #[test]
    fn test_mk_parsec_between_is_app() {
        let b = mk_parsec_between(
            Expr::Const(Name::str("open"), vec![]),
            Expr::Const(Name::str("close"), vec![]),
            Expr::Const(Name::str("p"), vec![]),
        );
        assert!(matches!(b, Expr::App(_, _)));
    }
    #[test]
    fn test_mk_parsec_chainl_is_app() {
        let c = mk_parsec_chainl(
            Expr::Const(Name::str("p"), vec![]),
            Expr::Const(Name::str("op"), vec![]),
            Expr::Const(Name::str("def"), vec![]),
        );
        assert!(matches!(c, Expr::App(_, _)));
    }
    #[test]
    fn test_mk_parsec_fail_is_app() {
        let f = mk_parsec_fail(Expr::Const(Name::str("e"), vec![]));
        assert!(matches!(f, Expr::App(_, _)));
    }
    #[test]
    fn test_mk_parsec_string_is_app() {
        let s = mk_parsec_string(Expr::Const(Name::str("s"), vec![]));
        assert!(matches!(s, Expr::App(_, _)));
    }
    #[test]
    fn test_mk_parsec_satisfy_is_app() {
        let s = mk_parsec_satisfy(Expr::Const(Name::str("pred"), vec![]));
        assert!(matches!(s, Expr::App(_, _)));
    }
    #[test]
    fn test_mk_parsec_lookahead_is_app() {
        let l = mk_parsec_lookahead(Expr::Const(Name::str("p"), vec![]));
        assert!(matches!(l, Expr::App(_, _)));
    }
}
/// ContextFreeGrammar: (N, Σ, P, S) — context-free grammar
#[allow(dead_code)]
pub fn prs_ext_cfg_ty() -> Expr {
    type1()
}
/// CfgNonterminal: a nonterminal symbol
#[allow(dead_code)]
pub fn prs_ext_cfg_nonterminal_ty() -> Expr {
    type1()
}
/// CfgTerminal: a terminal symbol
#[allow(dead_code)]
pub fn prs_ext_cfg_terminal_ty() -> Expr {
    type1()
}
/// CfgProduction: A → α, a production rule
#[allow(dead_code)]
pub fn prs_ext_cfg_production_ty() -> Expr {
    type1()
}
/// CfgDerivation: A ⇒* w — derivation in zero or more steps
#[allow(dead_code)]
pub fn prs_ext_cfg_derivation_ty() -> Expr {
    arrow(
        Expr::Const(Name::str("CfgNonterminal"), vec![]),
        arrow(
            list_of(Expr::Const(Name::str("CfgTerminal"), vec![])),
            prop(),
        ),
    )
}
/// CfgLanguage: L(G) = {w ∈ Σ* | S ⇒* w}
#[allow(dead_code)]
pub fn prs_ext_cfg_language_ty() -> Expr {
    arrow(
        Expr::Const(Name::str("ContextFreeGrammar"), vec![]),
        arrow(
            list_of(Expr::Const(Name::str("CfgTerminal"), vec![])),
            prop(),
        ),
    )
}
/// FirstSet: FIRST(A) = {a ∈ Σ | A ⇒* aα} ∪ (ε if A ⇒* ε)
#[allow(dead_code)]
pub fn prs_ext_first_set_ty() -> Expr {
    arrow(
        Expr::Const(Name::str("CfgNonterminal"), vec![]),
        list_of(option_of(Expr::Const(Name::str("CfgTerminal"), vec![]))),
    )
}
/// FollowSet: FOLLOW(A) = {a ∈ Σ ∪ {$} | S ⇒* αAaβ}
#[allow(dead_code)]
pub fn prs_ext_follow_set_ty() -> Expr {
    arrow(
        Expr::Const(Name::str("CfgNonterminal"), vec![]),
        list_of(option_of(Expr::Const(Name::str("CfgTerminal"), vec![]))),
    )
}
/// FirstSetCorrectness: a ∈ FIRST(A) ↔ A ⇒* aα for some α
#[allow(dead_code)]
pub fn prs_ext_first_set_correct_ty() -> Expr {
    arrow(
        Expr::Const(Name::str("ContextFreeGrammar"), vec![]),
        arrow(Expr::Const(Name::str("CfgNonterminal"), vec![]), prop()),
    )
}
/// FollowSetCorrectness: a ∈ FOLLOW(A) ↔ S ⇒* αAaβ for some α,β
#[allow(dead_code)]
pub fn prs_ext_follow_set_correct_ty() -> Expr {
    arrow(
        Expr::Const(Name::str("ContextFreeGrammar"), vec![]),
        arrow(Expr::Const(Name::str("CfgNonterminal"), vec![]), prop()),
    )
}
/// LlkGrammar: a grammar that is LL(k) — left-to-right, leftmost derivation, k tokens lookahead
#[allow(dead_code)]
pub fn prs_ext_llk_grammar_ty() -> Expr {
    arrow(
        nat_ty(),
        arrow(Expr::Const(Name::str("ContextFreeGrammar"), vec![]), prop()),
    )
}
/// LlkParsingTable: the LL(k) parsing table M\[A, w\] → production
#[allow(dead_code)]
pub fn prs_ext_llk_table_ty() -> Expr {
    arrow(
        Expr::Const(Name::str("ContextFreeGrammar"), vec![]),
        arrow(nat_ty(), Expr::Const(Name::str("LlkTable"), vec![])),
    )
}
/// LlkCorrectness: LL(k) parser accepts w ↔ w ∈ L(G)
#[allow(dead_code)]
pub fn prs_ext_llk_correct_ty() -> Expr {
    arrow(
        Expr::Const(Name::str("ContextFreeGrammar"), vec![]),
        arrow(
            list_of(Expr::Const(Name::str("CfgTerminal"), vec![])),
            prop(),
        ),
    )
}
/// LlkDeterminism: LL(k) grammars admit deterministic top-down parsing
#[allow(dead_code)]
pub fn prs_ext_llk_determinism_ty() -> Expr {
    arrow(
        Expr::Const(Name::str("ContextFreeGrammar"), vec![]),
        arrow(nat_ty(), prop()),
    )
}
/// EarleyItem: (A → α • β, i) — a dotted rule at position i
#[allow(dead_code)]
pub fn prs_ext_earley_item_ty() -> Expr {
    type1()
}
/// EarleyChart: array of Earley item sets S_0, ..., S_n
#[allow(dead_code)]
pub fn prs_ext_earley_chart_ty() -> Expr {
    type1()
}
/// EarleyCompleteness: Earley parser recognizes all context-free languages
#[allow(dead_code)]
pub fn prs_ext_earley_completeness_ty() -> Expr {
    arrow(
        Expr::Const(Name::str("ContextFreeGrammar"), vec![]),
        arrow(
            list_of(Expr::Const(Name::str("CfgTerminal"), vec![])),
            prop(),
        ),
    )
}
/// EarleySoundness: Earley parser accepts w ↔ w ∈ L(G)
#[allow(dead_code)]
pub fn prs_ext_earley_soundness_ty() -> Expr {
    arrow(Expr::Const(Name::str("ContextFreeGrammar"), vec![]), prop())
}
/// CykAlgorithm: CYK recognizes context-free languages in O(n³ |G|)
#[allow(dead_code)]
pub fn prs_ext_cyk_ty() -> Expr {
    arrow(
        Expr::Const(Name::str("ContextFreeGrammar"), vec![]),
        arrow(
            list_of(Expr::Const(Name::str("CfgTerminal"), vec![])),
            bool_ty(),
        ),
    )
}
/// CykCorrectness: CYK returns true ↔ w ∈ L(G)
#[allow(dead_code)]
pub fn prs_ext_cyk_correct_ty() -> Expr {
    arrow(
        Expr::Const(Name::str("ContextFreeGrammar"), vec![]),
        arrow(
            list_of(Expr::Const(Name::str("CfgTerminal"), vec![])),
            prop(),
        ),
    )
}
/// ChomskyNormalForm: every CFG can be converted to CNF
#[allow(dead_code)]
pub fn prs_ext_cnf_ty() -> Expr {
    arrow(
        Expr::Const(Name::str("ContextFreeGrammar"), vec![]),
        Expr::Const(Name::str("ContextFreeGrammar"), vec![]),
    )
}
/// CnfEquivalence: CNF(G) generates the same language as G (minus ε)
#[allow(dead_code)]
pub fn prs_ext_cnf_equiv_ty() -> Expr {
    arrow(Expr::Const(Name::str("ContextFreeGrammar"), vec![]), prop())
}
/// PegExpr: a parsing expression (ordered choice, sequence, Kleene*, etc.)
#[allow(dead_code)]
pub fn prs_ext_peg_expr_ty() -> Expr {
    type1()
}
/// PegResult: result of applying a PEG expression to a string
#[allow(dead_code)]
pub fn prs_ext_peg_result_ty() -> Expr {
    type1()
}
/// PegSemantics: ⟦e⟧(x) — denotational semantics of PEG expression e on input x
#[allow(dead_code)]
pub fn prs_ext_peg_semantics_ty() -> Expr {
    arrow(
        Expr::Const(Name::str("PegExpr"), vec![]),
        arrow(string_ty(), Expr::Const(Name::str("PegResult"), vec![])),
    )
}
/// PegOrderedChoice: e1 / e2 — try e1, if it fails try e2
#[allow(dead_code)]
pub fn prs_ext_peg_ordered_choice_ty() -> Expr {
    arrow(
        Expr::Const(Name::str("PegExpr"), vec![]),
        arrow(
            Expr::Const(Name::str("PegExpr"), vec![]),
            Expr::Const(Name::str("PegExpr"), vec![]),
        ),
    )
}
/// PegStar: e* — zero or more matches of e (greedy)
#[allow(dead_code)]
pub fn prs_ext_peg_star_ty() -> Expr {
    arrow(
        Expr::Const(Name::str("PegExpr"), vec![]),
        Expr::Const(Name::str("PegExpr"), vec![]),
    )
}
/// PegNot: !e — negative lookahead
#[allow(dead_code)]
pub fn prs_ext_peg_not_ty() -> Expr {
    arrow(
        Expr::Const(Name::str("PegExpr"), vec![]),
        Expr::Const(Name::str("PegExpr"), vec![]),
    )
}
/// PegDeterminism: PEG parsing is deterministic (no ambiguity by definition)
#[allow(dead_code)]
pub fn prs_ext_peg_determinism_ty() -> Expr {
    arrow(Expr::Const(Name::str("PegExpr"), vec![]), prop())
}
/// PackratMemo: memoization table for packrat parsing
#[allow(dead_code)]
pub fn prs_ext_packrat_memo_ty() -> Expr {
    type1()
}
/// PackratParsing: memoized recursive descent for PEGs in linear time
#[allow(dead_code)]
pub fn prs_ext_packrat_ty() -> Expr {
    arrow(
        Expr::Const(Name::str("PegExpr"), vec![]),
        arrow(
            string_ty(),
            arrow(
                Expr::Const(Name::str("PackratMemo"), vec![]),
                option_of(nat_ty()),
            ),
        ),
    )
}
/// PackratCorrectness: packrat produces the same result as naive PEG matching
#[allow(dead_code)]
pub fn prs_ext_packrat_correct_ty() -> Expr {
    arrow(
        Expr::Const(Name::str("PegExpr"), vec![]),
        arrow(string_ty(), prop()),
    )
}
/// PackratComplexity: packrat parsing is O(n) in input length
#[allow(dead_code)]
pub fn prs_ext_packrat_linear_ty() -> Expr {
    arrow(
        Expr::Const(Name::str("PegExpr"), vec![]),
        arrow(nat_ty(), prop()),
    )
}
/// LeftFactoring: transform G to remove common prefixes
#[allow(dead_code)]
pub fn prs_ext_left_factoring_ty() -> Expr {
    arrow(
        Expr::Const(Name::str("ContextFreeGrammar"), vec![]),
        Expr::Const(Name::str("ContextFreeGrammar"), vec![]),
    )
}
/// LeftRecursionElimination: remove left recursion from a CFG
#[allow(dead_code)]
pub fn prs_ext_left_rec_elim_ty() -> Expr {
    arrow(
        Expr::Const(Name::str("ContextFreeGrammar"), vec![]),
        Expr::Const(Name::str("ContextFreeGrammar"), vec![]),
    )
}
/// GreibachNormalForm: every CFG can be put in GNF (productions start with terminals)
#[allow(dead_code)]
pub fn prs_ext_gnf_ty() -> Expr {
    arrow(
        Expr::Const(Name::str("ContextFreeGrammar"), vec![]),
        Expr::Const(Name::str("ContextFreeGrammar"), vec![]),
    )
}
/// GreibachEquivalence: GNF(G) has the same language as G
#[allow(dead_code)]
pub fn prs_ext_gnf_equiv_ty() -> Expr {
    arrow(Expr::Const(Name::str("ContextFreeGrammar"), vec![]), prop())
}
/// ParserSemanticsDenotation: ⟦p⟧ : String → List (α × String)
#[allow(dead_code)]
pub fn prs_ext_parser_denotation_ty() -> Expr {
    arrow(string_ty(), list_of(type1()))
}
/// ParserFunctorLaw: map id p = p
#[allow(dead_code)]
pub fn prs_ext_functor_law_id_ty() -> Expr {
    arrow(parsec_of(type1(), type1()), prop())
}
/// ParserFunctorCompose: map (f ∘ g) p = map f (map g p)
#[allow(dead_code)]
pub fn prs_ext_functor_compose_ty() -> Expr {
    arrow(
        parsec_of(type1(), type1()),
        arrow(type1(), arrow(type1(), prop())),
    )
}
/// ParserApplicativeLaw: pure f <*> pure x = pure (f x)
#[allow(dead_code)]
pub fn prs_ext_applicative_law_ty() -> Expr {
    arrow(parsec_of(type1(), type1()), prop())
}
/// ParserAlternativeLaw: fail <|> p = p and p <|> fail = p
#[allow(dead_code)]
pub fn prs_ext_alternative_law_ty() -> Expr {
    arrow(parsec_of(type1(), type1()), prop())
}
/// ParserMonadLeftId: pure a >>= f = f a
#[allow(dead_code)]
pub fn prs_ext_monad_left_id_ty() -> Expr {
    arrow(
        type1(),
        arrow(arrow(type1(), parsec_of(type1(), type1())), prop()),
    )
}
/// ParserMonadRightId: p >>= pure = p
#[allow(dead_code)]
pub fn prs_ext_monad_right_id_ty() -> Expr {
    arrow(parsec_of(type1(), type1()), prop())
}
/// ParserMonadAssoc: (p >>= f) >>= g = p >>= (λa. f a >>= g)
#[allow(dead_code)]
pub fn prs_ext_monad_assoc_ty() -> Expr {
    arrow(
        parsec_of(type1(), type1()),
        arrow(
            arrow(type1(), parsec_of(type1(), type1())),
            arrow(arrow(type1(), parsec_of(type1(), type1())), prop()),
        ),
    )
}
/// ErrorRecovery: a parser with error recovery capabilities
#[allow(dead_code)]
pub fn prs_ext_error_recovery_ty() -> Expr {
    arrow(
        parsec_of(string_ty(), type1()),
        parsec_of(string_ty(), type1()),
    )
}
/// SynchronizationSet: set of tokens used for error recovery
#[allow(dead_code)]
pub fn prs_ext_sync_set_ty() -> Expr {
    list_of(char_ty())
}
/// PrettyPrint: pretty printer as an inverse of parsing
#[allow(dead_code)]
pub fn prs_ext_pretty_print_ty() -> Expr {
    arrow(type1(), string_ty())
}
/// PrettyPrintRoundtrip: parse (pretty_print x) = Some x
#[allow(dead_code)]
pub fn prs_ext_pretty_roundtrip_ty() -> Expr {
    arrow(type1(), prop())
}
/// IncrementalParsing: update parse result given a text edit
#[allow(dead_code)]
pub fn prs_ext_incremental_parse_ty() -> Expr {
    arrow(
        parsec_of(string_ty(), type1()),
        arrow(
            string_ty(),
            arrow(nat_ty(), parsec_of(string_ty(), type1())),
        ),
    )
}
/// TotalParser: a parser guaranteed to terminate on all inputs
#[allow(dead_code)]
pub fn prs_ext_total_parser_ty() -> Expr {
    arrow(parsec_of(string_ty(), type1()), prop())
}
/// WellFoundedInput: every recursive call is on strictly shorter input
#[allow(dead_code)]
pub fn prs_ext_well_founded_input_ty() -> Expr {
    arrow(parsec_of(string_ty(), type1()), prop())
}
/// TerminationProof: well-founded recursion ensures total parser terminates
#[allow(dead_code)]
pub fn prs_ext_termination_proof_ty() -> Expr {
    arrow(parsec_of(string_ty(), type1()), arrow(string_ty(), prop()))
}
/// Derivative: ∂_c L — Brzozowski derivative of language L w.r.t. character c
#[allow(dead_code)]
pub fn prs_ext_derivative_ty() -> Expr {
    arrow(
        parsec_of(string_ty(), type1()),
        arrow(char_ty(), parsec_of(string_ty(), type1())),
    )
}
/// DerivativeSemantics: ⟦∂_c p⟧(s) = ⟦p⟧(cs)
#[allow(dead_code)]
pub fn prs_ext_deriv_semantics_ty() -> Expr {
    arrow(parsec_of(string_ty(), type1()), arrow(char_ty(), prop()))
}
/// DerivativeCompaction: compact representation of Brzozowski derivatives
#[allow(dead_code)]
pub fn prs_ext_deriv_compact_ty() -> Expr {
    arrow(
        parsec_of(string_ty(), type1()),
        parsec_of(string_ty(), type1()),
    )
}
