//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)
#![allow(clippy::items_after_test_module)]

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

/// Prop: `Sort 0`.
#[allow(dead_code)]
pub fn prop() -> Expr {
    Expr::Sort(Level::zero())
}
/// Type 1: `Sort 1`.
#[allow(dead_code)]
pub fn type1() -> Expr {
    Expr::Sort(Level::succ(Level::zero()))
}
/// Nat type constant.
#[allow(dead_code)]
pub fn nat_ty() -> Expr {
    Expr::Const(Name::str("Nat"), vec![])
}
/// Bool type constant.
#[allow(dead_code)]
pub fn bool_ty() -> Expr {
    Expr::Const(Name::str("Bool"), vec![])
}
/// `Fin` applied to a size argument.
#[allow(dead_code)]
pub fn fin_of(n: Expr) -> Expr {
    app(Expr::Const(Name::str("Fin"), vec![]), n)
}
/// `Array` applied to element type and size.
#[allow(dead_code)]
pub fn array_of(elem_ty: Expr, size: Expr) -> Expr {
    app2(Expr::Const(Name::str("Array"), vec![]), elem_ty, size)
}
/// `Option` applied to a type argument.
#[allow(dead_code)]
pub fn option_of(ty: Expr) -> Expr {
    app(Expr::Const(Name::str("Option"), vec![]), ty)
}
/// `List` applied to a type argument.
#[allow(dead_code)]
pub fn list_of(ty: Expr) -> Expr {
    app(Expr::Const(Name::str("List"), vec![]), ty)
}
/// `Prod` applied to two type arguments.
#[allow(dead_code)]
pub fn prod_of(a: Expr, b: Expr) -> Expr {
    app2(Expr::Const(Name::str("Prod"), vec![]), a, b)
}
/// `Nat.succ n` — successor of a Nat expression.
#[allow(dead_code)]
pub fn nat_succ(n: Expr) -> Expr {
    app(Expr::Const(Name::str("Nat.succ"), vec![]), n)
}
/// `Nat.add a b`.
#[allow(dead_code)]
pub fn nat_add(a: Expr, b: Expr) -> Expr {
    app2(Expr::Const(Name::str("Nat.add"), vec![]), a, b)
}
/// `Nat.sub a b`.
#[allow(dead_code)]
pub fn nat_sub(a: Expr, b: Expr) -> Expr {
    app2(Expr::Const(Name::str("Nat.sub"), vec![]), a, b)
}
/// `Nat.min a b`.
#[allow(dead_code)]
pub fn nat_min(a: Expr, b: Expr) -> Expr {
    app2(Expr::Const(Name::str("Nat.min"), vec![]), a, b)
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
/// An instance Pi binder `[inst : ty]`.
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
/// `Ord` type class applied to a type.
#[allow(dead_code)]
pub fn ord_of(ty: Expr) -> Expr {
    app(Expr::Const(Name::str("Ord"), vec![]), ty)
}
/// `BEq` type class applied to a type.
#[allow(dead_code)]
pub fn beq_of(ty: Expr) -> Expr {
    app(Expr::Const(Name::str("BEq"), vec![]), ty)
}
/// Build the `Array α n` type expression.
#[allow(dead_code)]
pub fn mk_array_ty(elem_ty: Expr, size: Expr) -> Expr {
    array_of(elem_ty, size)
}
/// Build `Array.empty` for a given element type (returns `Array α 0`).
#[allow(dead_code)]
pub fn mk_array_empty(elem_ty: Expr) -> Expr {
    app(Expr::Const(Name::str("Array.empty"), vec![]), elem_ty)
}
/// Build `Array.push arr elem`.
#[allow(dead_code)]
pub fn mk_array_push(arr: Expr, elem: Expr) -> Expr {
    app2(Expr::Const(Name::str("Array.push"), vec![]), arr, elem)
}
/// Build `Array.get arr idx`.
#[allow(dead_code)]
pub fn mk_array_get(arr: Expr, idx: Expr) -> Expr {
    app2(Expr::Const(Name::str("Array.get"), vec![]), arr, idx)
}
/// Build `Array.set arr idx val`.
#[allow(dead_code)]
pub fn mk_array_set(arr: Expr, idx: Expr, val: Expr) -> Expr {
    app3(Expr::Const(Name::str("Array.set"), vec![]), arr, idx, val)
}
/// Build `Array.map f arr`.
#[allow(dead_code)]
pub fn mk_array_map(f: Expr, arr: Expr) -> Expr {
    app2(Expr::Const(Name::str("Array.map"), vec![]), f, arr)
}
/// Build `Array.foldl f init arr`.
#[allow(dead_code)]
pub fn mk_array_foldl(f: Expr, init: Expr, arr: Expr) -> Expr {
    app3(Expr::Const(Name::str("Array.foldl"), vec![]), f, init, arr)
}
/// Build `Array.toList arr`.
#[allow(dead_code)]
pub fn mk_array_tolist(arr: Expr) -> Expr {
    app(Expr::Const(Name::str("Array.toList"), vec![]), arr)
}
/// Build Array type and all standard declarations, adding them to the
/// environment.
///
/// Assumes that `Nat`, `Fin`, `Bool`, `Option`, `List`, `Prod`, `Eq`,
/// `Ord`, `BEq`, `Nat.succ`, `Nat.add`, `Nat.sub`, `Nat.min` are
/// already declared (or referenced by name).
pub fn build_array_env(env: &mut Environment) -> Result<(), String> {
    let array_type = Expr::Pi(
        BinderInfo::Default,
        Name::str("α"),
        Box::new(type1()),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("n"),
            Box::new(nat_ty()),
            Box::new(type1()),
        )),
    );
    add_axiom(env, "Array", vec![], array_type)?;
    add_axiom(
        env,
        "Array.get",
        vec![],
        implicit_pi(
            "α",
            type1(),
            implicit_pi(
                "n",
                nat_ty(),
                default_pi(
                    "arr",
                    array_of(Expr::BVar(1), Expr::BVar(0)),
                    default_pi("i", fin_of(Expr::BVar(1)), Expr::BVar(3)),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "Array.set",
        vec![],
        implicit_pi(
            "α",
            type1(),
            implicit_pi(
                "n",
                nat_ty(),
                default_pi(
                    "arr",
                    array_of(Expr::BVar(1), Expr::BVar(0)),
                    default_pi(
                        "i",
                        fin_of(Expr::BVar(1)),
                        default_pi("val", Expr::BVar(3), array_of(Expr::BVar(4), Expr::BVar(3))),
                    ),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "Array.empty",
        vec![],
        implicit_pi(
            "α",
            type1(),
            array_of(Expr::BVar(0), Expr::Const(Name::str("Nat.zero"), vec![])),
        ),
    )?;
    add_axiom(
        env,
        "Array.mk",
        vec![],
        implicit_pi(
            "α",
            type1(),
            default_pi(
                "data",
                list_of(Expr::BVar(0)),
                array_of(
                    Expr::BVar(1),
                    app(Expr::Const(Name::str("List.length"), vec![]), Expr::BVar(0)),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "Array.mkEmpty",
        vec![],
        implicit_pi(
            "α",
            type1(),
            default_pi(
                "capacity",
                nat_ty(),
                array_of(Expr::BVar(1), Expr::Const(Name::str("Nat.zero"), vec![])),
            ),
        ),
    )?;
    add_axiom(
        env,
        "Array.size",
        vec![],
        implicit_pi(
            "α",
            type1(),
            implicit_pi(
                "n",
                nat_ty(),
                default_pi("arr", array_of(Expr::BVar(1), Expr::BVar(0)), nat_ty()),
            ),
        ),
    )?;
    add_axiom(
        env,
        "Array.push",
        vec![],
        implicit_pi(
            "α",
            type1(),
            implicit_pi(
                "n",
                nat_ty(),
                default_pi(
                    "arr",
                    array_of(Expr::BVar(1), Expr::BVar(0)),
                    default_pi(
                        "x",
                        Expr::BVar(2),
                        array_of(Expr::BVar(3), nat_succ(Expr::BVar(2))),
                    ),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "Array.pop",
        vec![],
        implicit_pi(
            "α",
            type1(),
            implicit_pi(
                "n",
                nat_ty(),
                default_pi(
                    "arr",
                    array_of(Expr::BVar(1), nat_succ(Expr::BVar(0))),
                    prod_of(array_of(Expr::BVar(2), Expr::BVar(1)), Expr::BVar(2)),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "Array.swap",
        vec![],
        implicit_pi(
            "α",
            type1(),
            implicit_pi(
                "n",
                nat_ty(),
                default_pi(
                    "arr",
                    array_of(Expr::BVar(1), Expr::BVar(0)),
                    default_pi(
                        "i",
                        fin_of(Expr::BVar(1)),
                        default_pi(
                            "j",
                            fin_of(Expr::BVar(2)),
                            array_of(Expr::BVar(4), Expr::BVar(3)),
                        ),
                    ),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "Array.map",
        vec![],
        implicit_pi(
            "α",
            type1(),
            implicit_pi(
                "β",
                type1(),
                implicit_pi(
                    "n",
                    nat_ty(),
                    default_pi(
                        "f",
                        arrow(Expr::BVar(2), Expr::BVar(1)),
                        default_pi(
                            "arr",
                            array_of(Expr::BVar(3), Expr::BVar(1)),
                            array_of(Expr::BVar(3), Expr::BVar(2)),
                        ),
                    ),
                ),
            ),
        ),
    )?;
    {
        let f_ty = arrow(Expr::BVar(1), arrow(Expr::BVar(3), Expr::BVar(3)));
        add_axiom(
            env,
            "Array.foldl",
            vec![],
            implicit_pi(
                "α",
                type1(),
                implicit_pi(
                    "β",
                    type1(),
                    implicit_pi(
                        "n",
                        nat_ty(),
                        default_pi(
                            "f",
                            f_ty,
                            default_pi(
                                "init",
                                Expr::BVar(2),
                                default_pi(
                                    "arr",
                                    array_of(Expr::BVar(4), Expr::BVar(2)),
                                    Expr::BVar(4),
                                ),
                            ),
                        ),
                    ),
                ),
            ),
        )?;
    }
    {
        let f_ty = arrow(Expr::BVar(2), arrow(Expr::BVar(2), Expr::BVar(3)));
        add_axiom(
            env,
            "Array.foldr",
            vec![],
            implicit_pi(
                "α",
                type1(),
                implicit_pi(
                    "β",
                    type1(),
                    implicit_pi(
                        "n",
                        nat_ty(),
                        default_pi(
                            "f",
                            f_ty,
                            default_pi(
                                "init",
                                Expr::BVar(2),
                                default_pi(
                                    "arr",
                                    array_of(Expr::BVar(4), Expr::BVar(2)),
                                    Expr::BVar(4),
                                ),
                            ),
                        ),
                    ),
                ),
            ),
        )?;
    }
    add_axiom(
        env,
        "Array.filter",
        vec![],
        implicit_pi(
            "α",
            type1(),
            implicit_pi(
                "n",
                nat_ty(),
                default_pi(
                    "p",
                    arrow(Expr::BVar(1), bool_ty()),
                    default_pi(
                        "arr",
                        array_of(Expr::BVar(2), Expr::BVar(1)),
                        list_of(Expr::BVar(3)),
                    ),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "Array.append",
        vec![],
        implicit_pi(
            "α",
            type1(),
            implicit_pi(
                "n",
                nat_ty(),
                implicit_pi(
                    "m",
                    nat_ty(),
                    default_pi(
                        "a1",
                        array_of(Expr::BVar(2), Expr::BVar(1)),
                        default_pi(
                            "a2",
                            array_of(Expr::BVar(3), Expr::BVar(1)),
                            array_of(Expr::BVar(4), nat_add(Expr::BVar(3), Expr::BVar(2))),
                        ),
                    ),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "Array.reverse",
        vec![],
        implicit_pi(
            "α",
            type1(),
            implicit_pi(
                "n",
                nat_ty(),
                default_pi(
                    "arr",
                    array_of(Expr::BVar(1), Expr::BVar(0)),
                    array_of(Expr::BVar(2), Expr::BVar(1)),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "Array.zip",
        vec![],
        implicit_pi(
            "α",
            type1(),
            implicit_pi(
                "β",
                type1(),
                implicit_pi(
                    "n",
                    nat_ty(),
                    default_pi(
                        "a1",
                        array_of(Expr::BVar(2), Expr::BVar(0)),
                        default_pi(
                            "a2",
                            array_of(Expr::BVar(2), Expr::BVar(1)),
                            array_of(prod_of(Expr::BVar(4), Expr::BVar(3)), Expr::BVar(2)),
                        ),
                    ),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "Array.enumerate",
        vec![],
        implicit_pi(
            "α",
            type1(),
            implicit_pi(
                "n",
                nat_ty(),
                default_pi(
                    "arr",
                    array_of(Expr::BVar(1), Expr::BVar(0)),
                    array_of(prod_of(nat_ty(), Expr::BVar(2)), Expr::BVar(1)),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "Array.take",
        vec![],
        implicit_pi(
            "α",
            type1(),
            implicit_pi(
                "n",
                nat_ty(),
                default_pi(
                    "k",
                    nat_ty(),
                    default_pi(
                        "arr",
                        array_of(Expr::BVar(2), Expr::BVar(1)),
                        array_of(Expr::BVar(3), nat_min(Expr::BVar(1), Expr::BVar(2))),
                    ),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "Array.drop",
        vec![],
        implicit_pi(
            "α",
            type1(),
            implicit_pi(
                "n",
                nat_ty(),
                default_pi(
                    "k",
                    nat_ty(),
                    default_pi(
                        "arr",
                        array_of(Expr::BVar(2), Expr::BVar(1)),
                        array_of(Expr::BVar(3), nat_sub(Expr::BVar(2), Expr::BVar(1))),
                    ),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "Array.any",
        vec![],
        implicit_pi(
            "α",
            type1(),
            implicit_pi(
                "n",
                nat_ty(),
                default_pi(
                    "p",
                    arrow(Expr::BVar(1), bool_ty()),
                    default_pi("arr", array_of(Expr::BVar(2), Expr::BVar(1)), bool_ty()),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "Array.all",
        vec![],
        implicit_pi(
            "α",
            type1(),
            implicit_pi(
                "n",
                nat_ty(),
                default_pi(
                    "p",
                    arrow(Expr::BVar(1), bool_ty()),
                    default_pi("arr", array_of(Expr::BVar(2), Expr::BVar(1)), bool_ty()),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "Array.contains",
        vec![],
        implicit_pi(
            "α",
            type1(),
            implicit_pi(
                "n",
                nat_ty(),
                inst_pi(
                    "inst",
                    beq_of(Expr::BVar(1)),
                    default_pi(
                        "arr",
                        array_of(Expr::BVar(2), Expr::BVar(1)),
                        default_pi("a", Expr::BVar(3), bool_ty()),
                    ),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "Array.indexOf?",
        vec![],
        implicit_pi(
            "α",
            type1(),
            implicit_pi(
                "n",
                nat_ty(),
                inst_pi(
                    "inst",
                    beq_of(Expr::BVar(1)),
                    default_pi(
                        "arr",
                        array_of(Expr::BVar(2), Expr::BVar(1)),
                        default_pi("a", Expr::BVar(3), option_of(fin_of(Expr::BVar(3)))),
                    ),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "Array.toList",
        vec![],
        implicit_pi(
            "α",
            type1(),
            implicit_pi(
                "n",
                nat_ty(),
                default_pi(
                    "arr",
                    array_of(Expr::BVar(1), Expr::BVar(0)),
                    list_of(Expr::BVar(2)),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "Array.findSome?",
        vec![],
        implicit_pi(
            "α",
            type1(),
            implicit_pi(
                "β",
                type1(),
                implicit_pi(
                    "n",
                    nat_ty(),
                    default_pi(
                        "f",
                        arrow(Expr::BVar(2), option_of(Expr::BVar(1))),
                        default_pi(
                            "arr",
                            array_of(Expr::BVar(3), Expr::BVar(1)),
                            option_of(Expr::BVar(3)),
                        ),
                    ),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "Array.qsort",
        vec![],
        implicit_pi(
            "α",
            type1(),
            implicit_pi(
                "n",
                nat_ty(),
                inst_pi(
                    "inst",
                    ord_of(Expr::BVar(1)),
                    default_pi(
                        "arr",
                        array_of(Expr::BVar(2), Expr::BVar(1)),
                        array_of(Expr::BVar(3), Expr::BVar(2)),
                    ),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "Array.binSearch",
        vec![],
        implicit_pi(
            "α",
            type1(),
            implicit_pi(
                "n",
                nat_ty(),
                inst_pi(
                    "inst",
                    ord_of(Expr::BVar(1)),
                    default_pi(
                        "arr",
                        array_of(Expr::BVar(2), Expr::BVar(1)),
                        default_pi("a", Expr::BVar(3), option_of(fin_of(Expr::BVar(3)))),
                    ),
                ),
            ),
        ),
    )?;
    {
        let push_expr = app2(
            Expr::Const(Name::str("Array.push"), vec![]),
            Expr::BVar(1),
            Expr::BVar(0),
        );
        let size_push = app(Expr::Const(Name::str("Array.size"), vec![]), push_expr);
        let size_a = app(Expr::Const(Name::str("Array.size"), vec![]), Expr::BVar(1));
        let succ_size_a = nat_succ(size_a);
        add_axiom(
            env,
            "Array.size_push",
            vec![],
            implicit_pi(
                "α",
                type1(),
                implicit_pi(
                    "n",
                    nat_ty(),
                    default_pi(
                        "a",
                        array_of(Expr::BVar(1), Expr::BVar(0)),
                        default_pi(
                            "x",
                            Expr::BVar(2),
                            eq_expr(nat_ty(), size_push, succ_size_a),
                        ),
                    ),
                ),
            ),
        )?;
    }
    {
        let set_expr = app3(
            Expr::Const(Name::str("Array.set"), vec![]),
            Expr::BVar(2),
            Expr::BVar(1),
            Expr::BVar(0),
        );
        let get_set = app2(
            Expr::Const(Name::str("Array.get"), vec![]),
            set_expr,
            Expr::BVar(1),
        );
        add_axiom(
            env,
            "Array.get_set_same",
            vec![],
            implicit_pi(
                "α",
                type1(),
                implicit_pi(
                    "n",
                    nat_ty(),
                    default_pi(
                        "a",
                        array_of(Expr::BVar(1), Expr::BVar(0)),
                        default_pi(
                            "i",
                            fin_of(Expr::BVar(1)),
                            default_pi(
                                "v",
                                Expr::BVar(3),
                                eq_expr(Expr::BVar(4), get_set, Expr::BVar(0)),
                            ),
                        ),
                    ),
                ),
            ),
        )?;
    }
    {
        let eq_ij = eq_expr(fin_of(Expr::BVar(4)), Expr::BVar(2), Expr::BVar(1));
        let not_eq = arrow(eq_ij, Expr::Const(Name::str("False"), vec![]));
        let set_expr = app3(
            Expr::Const(Name::str("Array.set"), vec![]),
            Expr::BVar(4),
            Expr::BVar(3),
            Expr::BVar(1),
        );
        let get_set_j = app2(
            Expr::Const(Name::str("Array.get"), vec![]),
            set_expr,
            Expr::BVar(2),
        );
        let get_a_j = app2(
            Expr::Const(Name::str("Array.get"), vec![]),
            Expr::BVar(4),
            Expr::BVar(2),
        );
        add_axiom(
            env,
            "Array.get_set_diff",
            vec![],
            implicit_pi(
                "α",
                type1(),
                implicit_pi(
                    "n",
                    nat_ty(),
                    default_pi(
                        "a",
                        array_of(Expr::BVar(1), Expr::BVar(0)),
                        default_pi(
                            "i",
                            fin_of(Expr::BVar(1)),
                            default_pi(
                                "j",
                                fin_of(Expr::BVar(2)),
                                default_pi(
                                    "v",
                                    Expr::BVar(4),
                                    default_pi(
                                        "h",
                                        not_eq,
                                        eq_expr(Expr::BVar(6), get_set_j, get_a_j),
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
            ),
        )?;
    }
    {
        let map_fa = app2(
            Expr::Const(Name::str("Array.map"), vec![]),
            Expr::BVar(1),
            Expr::BVar(0),
        );
        let size_map = app(Expr::Const(Name::str("Array.size"), vec![]), map_fa);
        let size_a = app(Expr::Const(Name::str("Array.size"), vec![]), Expr::BVar(0));
        add_axiom(
            env,
            "Array.map_size",
            vec![],
            implicit_pi(
                "α",
                type1(),
                implicit_pi(
                    "β",
                    type1(),
                    implicit_pi(
                        "n",
                        nat_ty(),
                        default_pi(
                            "f",
                            arrow(Expr::BVar(2), Expr::BVar(1)),
                            default_pi(
                                "a",
                                array_of(Expr::BVar(3), Expr::BVar(1)),
                                eq_expr(nat_ty(), size_map, size_a),
                            ),
                        ),
                    ),
                ),
            ),
        )?;
    }
    {
        let to_list_a = app(
            Expr::Const(Name::str("Array.toList"), vec![]),
            Expr::BVar(0),
        );
        let length_tolist = app(Expr::Const(Name::str("List.length"), vec![]), to_list_a);
        let size_a = app(Expr::Const(Name::str("Array.size"), vec![]), Expr::BVar(0));
        add_axiom(
            env,
            "Array.toList_length",
            vec![],
            implicit_pi(
                "α",
                type1(),
                implicit_pi(
                    "n",
                    nat_ty(),
                    default_pi(
                        "a",
                        array_of(Expr::BVar(1), Expr::BVar(0)),
                        eq_expr(nat_ty(), length_tolist, size_a),
                    ),
                ),
            ),
        )?;
    }
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    /// Set up a minimal environment with all prerequisites.
    fn setup_env() -> Environment {
        let mut env = Environment::new();
        for name in &[
            "Nat", "Bool", "Nat.zero", "Nat.succ", "Nat.add", "Nat.sub", "Nat.min", "Ord", "BEq",
            "Eq", "False",
        ] {
            env.add(Declaration::Axiom {
                name: Name::str(*name),
                univ_params: vec![],
                ty: type1(),
            })
            .expect("operation should succeed");
        }
        env.add(Declaration::Axiom {
            name: Name::str("Fin"),
            univ_params: vec![],
            ty: arrow(nat_ty(), type1()),
        })
        .expect("operation should succeed");
        env.add(Declaration::Axiom {
            name: Name::str("Option"),
            univ_params: vec![],
            ty: arrow(type1(), type1()),
        })
        .expect("operation should succeed");
        env.add(Declaration::Axiom {
            name: Name::str("List"),
            univ_params: vec![],
            ty: arrow(type1(), type1()),
        })
        .expect("operation should succeed");
        env.add(Declaration::Axiom {
            name: Name::str("List.length"),
            univ_params: vec![],
            ty: implicit_pi(
                "α",
                type1(),
                default_pi("l", list_of(Expr::BVar(0)), nat_ty()),
            ),
        })
        .expect("operation should succeed");
        env.add(Declaration::Axiom {
            name: Name::str("Prod"),
            univ_params: vec![],
            ty: arrow(type1(), arrow(type1(), type1())),
        })
        .expect("operation should succeed");
        env
    }
    #[test]
    fn test_build_array_env() {
        let mut env = setup_env();
        assert!(build_array_env(&mut env).is_ok());
        assert!(env.get(&Name::str("Array")).is_some());
        assert!(env.get(&Name::str("Array.get")).is_some());
        assert!(env.get(&Name::str("Array.set")).is_some());
    }
    #[test]
    fn test_array_empty() {
        let mut env = setup_env();
        build_array_env(&mut env).expect("build_array_env should succeed");
        assert!(env.get(&Name::str("Array.empty")).is_some());
    }
    #[test]
    fn test_array_mk() {
        let mut env = setup_env();
        build_array_env(&mut env).expect("build_array_env should succeed");
        assert!(env.get(&Name::str("Array.mk")).is_some());
    }
    #[test]
    fn test_array_mk_empty() {
        let mut env = setup_env();
        build_array_env(&mut env).expect("build_array_env should succeed");
        assert!(env.get(&Name::str("Array.mkEmpty")).is_some());
    }
    #[test]
    fn test_array_size() {
        let mut env = setup_env();
        build_array_env(&mut env).expect("build_array_env should succeed");
        let decl = env
            .get(&Name::str("Array.size"))
            .expect("declaration 'Array.size' should exist in env");
        assert!(decl.ty().is_pi());
    }
    #[test]
    fn test_array_push() {
        let mut env = setup_env();
        build_array_env(&mut env).expect("build_array_env should succeed");
        let decl = env
            .get(&Name::str("Array.push"))
            .expect("declaration 'Array.push' should exist in env");
        assert!(decl.ty().is_pi());
    }
    #[test]
    fn test_array_pop() {
        let mut env = setup_env();
        build_array_env(&mut env).expect("build_array_env should succeed");
        let decl = env
            .get(&Name::str("Array.pop"))
            .expect("declaration 'Array.pop' should exist in env");
        assert!(decl.ty().is_pi());
    }
    #[test]
    fn test_array_swap() {
        let mut env = setup_env();
        build_array_env(&mut env).expect("build_array_env should succeed");
        let decl = env
            .get(&Name::str("Array.swap"))
            .expect("declaration 'Array.swap' should exist in env");
        assert!(decl.ty().is_pi());
    }
    #[test]
    fn test_array_map() {
        let mut env = setup_env();
        build_array_env(&mut env).expect("build_array_env should succeed");
        let decl = env
            .get(&Name::str("Array.map"))
            .expect("declaration 'Array.map' should exist in env");
        assert!(decl.ty().is_pi());
    }
    #[test]
    fn test_array_foldl() {
        let mut env = setup_env();
        build_array_env(&mut env).expect("build_array_env should succeed");
        let decl = env
            .get(&Name::str("Array.foldl"))
            .expect("declaration 'Array.foldl' should exist in env");
        assert!(decl.ty().is_pi());
    }
    #[test]
    fn test_array_foldr() {
        let mut env = setup_env();
        build_array_env(&mut env).expect("build_array_env should succeed");
        let decl = env
            .get(&Name::str("Array.foldr"))
            .expect("declaration 'Array.foldr' should exist in env");
        assert!(decl.ty().is_pi());
    }
    #[test]
    fn test_array_filter() {
        let mut env = setup_env();
        build_array_env(&mut env).expect("build_array_env should succeed");
        assert!(env.get(&Name::str("Array.filter")).is_some());
    }
    #[test]
    fn test_array_append() {
        let mut env = setup_env();
        build_array_env(&mut env).expect("build_array_env should succeed");
        let decl = env
            .get(&Name::str("Array.append"))
            .expect("declaration 'Array.append' should exist in env");
        assert!(decl.ty().is_pi());
    }
    #[test]
    fn test_array_reverse() {
        let mut env = setup_env();
        build_array_env(&mut env).expect("build_array_env should succeed");
        assert!(env.get(&Name::str("Array.reverse")).is_some());
    }
    #[test]
    fn test_array_zip() {
        let mut env = setup_env();
        build_array_env(&mut env).expect("build_array_env should succeed");
        let decl = env
            .get(&Name::str("Array.zip"))
            .expect("declaration 'Array.zip' should exist in env");
        assert!(decl.ty().is_pi());
    }
    #[test]
    fn test_array_enumerate() {
        let mut env = setup_env();
        build_array_env(&mut env).expect("build_array_env should succeed");
        assert!(env.get(&Name::str("Array.enumerate")).is_some());
    }
    #[test]
    fn test_array_take() {
        let mut env = setup_env();
        build_array_env(&mut env).expect("build_array_env should succeed");
        let decl = env
            .get(&Name::str("Array.take"))
            .expect("declaration 'Array.take' should exist in env");
        assert!(decl.ty().is_pi());
    }
    #[test]
    fn test_array_drop() {
        let mut env = setup_env();
        build_array_env(&mut env).expect("build_array_env should succeed");
        assert!(env.get(&Name::str("Array.drop")).is_some());
    }
    #[test]
    fn test_array_any() {
        let mut env = setup_env();
        build_array_env(&mut env).expect("build_array_env should succeed");
        assert!(env.get(&Name::str("Array.any")).is_some());
    }
    #[test]
    fn test_array_all() {
        let mut env = setup_env();
        build_array_env(&mut env).expect("build_array_env should succeed");
        assert!(env.get(&Name::str("Array.all")).is_some());
    }
    #[test]
    fn test_array_contains() {
        let mut env = setup_env();
        build_array_env(&mut env).expect("build_array_env should succeed");
        let decl = env
            .get(&Name::str("Array.contains"))
            .expect("declaration 'Array.contains' should exist in env");
        assert!(decl.ty().is_pi());
    }
    #[test]
    fn test_array_indexof() {
        let mut env = setup_env();
        build_array_env(&mut env).expect("build_array_env should succeed");
        let decl = env
            .get(&Name::str("Array.indexOf?"))
            .expect("declaration 'Array.indexOf?' should exist in env");
        assert!(decl.ty().is_pi());
    }
    #[test]
    fn test_array_tolist() {
        let mut env = setup_env();
        build_array_env(&mut env).expect("build_array_env should succeed");
        assert!(env.get(&Name::str("Array.toList")).is_some());
    }
    #[test]
    fn test_array_findsome() {
        let mut env = setup_env();
        build_array_env(&mut env).expect("build_array_env should succeed");
        let decl = env
            .get(&Name::str("Array.findSome?"))
            .expect("declaration 'Array.findSome?' should exist in env");
        assert!(decl.ty().is_pi());
    }
    #[test]
    fn test_array_qsort() {
        let mut env = setup_env();
        build_array_env(&mut env).expect("build_array_env should succeed");
        let decl = env
            .get(&Name::str("Array.qsort"))
            .expect("declaration 'Array.qsort' should exist in env");
        assert!(decl.ty().is_pi());
    }
    #[test]
    fn test_array_binsearch() {
        let mut env = setup_env();
        build_array_env(&mut env).expect("build_array_env should succeed");
        let decl = env
            .get(&Name::str("Array.binSearch"))
            .expect("declaration 'Array.binSearch' should exist in env");
        assert!(decl.ty().is_pi());
    }
    #[test]
    fn test_size_push_theorem() {
        let mut env = setup_env();
        build_array_env(&mut env).expect("build_array_env should succeed");
        let decl = env
            .get(&Name::str("Array.size_push"))
            .expect("declaration 'Array.size_push' should exist in env");
        assert!(decl.ty().is_pi());
    }
    #[test]
    fn test_get_set_same_theorem() {
        let mut env = setup_env();
        build_array_env(&mut env).expect("build_array_env should succeed");
        let decl = env
            .get(&Name::str("Array.get_set_same"))
            .expect("declaration 'Array.get_set_same' should exist in env");
        assert!(decl.ty().is_pi());
    }
    #[test]
    fn test_get_set_diff_theorem() {
        let mut env = setup_env();
        build_array_env(&mut env).expect("build_array_env should succeed");
        let decl = env
            .get(&Name::str("Array.get_set_diff"))
            .expect("declaration 'Array.get_set_diff' should exist in env");
        assert!(decl.ty().is_pi());
    }
    #[test]
    fn test_map_size_theorem() {
        let mut env = setup_env();
        build_array_env(&mut env).expect("build_array_env should succeed");
        let decl = env
            .get(&Name::str("Array.map_size"))
            .expect("declaration 'Array.map_size' should exist in env");
        assert!(decl.ty().is_pi());
    }
    #[test]
    fn test_tolist_length_theorem() {
        let mut env = setup_env();
        build_array_env(&mut env).expect("build_array_env should succeed");
        let decl = env
            .get(&Name::str("Array.toList_length"))
            .expect("declaration 'Array.toList_length' should exist in env");
        assert!(decl.ty().is_pi());
    }
    #[test]
    fn test_mk_array_ty_expr() {
        let t = mk_array_ty(nat_ty(), Expr::Const(Name::str("n"), vec![]));
        assert!(matches!(t, Expr::App(_, _)));
    }
    #[test]
    fn test_mk_array_empty_expr() {
        let e = mk_array_empty(nat_ty());
        assert!(matches!(e, Expr::App(_, _)));
    }
    #[test]
    fn test_mk_array_push_expr() {
        let arr = Expr::Const(Name::str("a"), vec![]);
        let elem = Expr::Const(Name::str("x"), vec![]);
        let expr = mk_array_push(arr, elem);
        assert!(matches!(expr, Expr::App(_, _)));
    }
    #[test]
    fn test_mk_array_get_expr() {
        let arr = Expr::Const(Name::str("a"), vec![]);
        let idx = Expr::Const(Name::str("i"), vec![]);
        let expr = mk_array_get(arr, idx);
        assert!(matches!(expr, Expr::App(_, _)));
    }
    #[test]
    fn test_mk_array_set_expr() {
        let arr = Expr::Const(Name::str("a"), vec![]);
        let idx = Expr::Const(Name::str("i"), vec![]);
        let val = Expr::Const(Name::str("v"), vec![]);
        let expr = mk_array_set(arr, idx, val);
        assert!(matches!(expr, Expr::App(_, _)));
    }
    #[test]
    fn test_mk_array_map_expr() {
        let f = Expr::Const(Name::str("f"), vec![]);
        let arr = Expr::Const(Name::str("a"), vec![]);
        let expr = mk_array_map(f, arr);
        assert!(matches!(expr, Expr::App(_, _)));
    }
    #[test]
    fn test_mk_array_foldl_expr() {
        let f = Expr::Const(Name::str("f"), vec![]);
        let init = Expr::Const(Name::str("init"), vec![]);
        let arr = Expr::Const(Name::str("a"), vec![]);
        let expr = mk_array_foldl(f, init, arr);
        assert!(matches!(expr, Expr::App(_, _)));
    }
    #[test]
    fn test_mk_array_tolist_expr() {
        let arr = Expr::Const(Name::str("a"), vec![]);
        let expr = mk_array_tolist(arr);
        assert!(matches!(expr, Expr::App(_, _)));
    }
    #[test]
    fn test_all_array_decls_present() {
        let mut env = setup_env();
        build_array_env(&mut env).expect("build_array_env should succeed");
        let names = [
            "Array",
            "Array.get",
            "Array.set",
            "Array.empty",
            "Array.mk",
            "Array.mkEmpty",
            "Array.size",
            "Array.push",
            "Array.pop",
            "Array.swap",
            "Array.map",
            "Array.foldl",
            "Array.foldr",
            "Array.filter",
            "Array.append",
            "Array.reverse",
            "Array.zip",
            "Array.enumerate",
            "Array.take",
            "Array.drop",
            "Array.any",
            "Array.all",
            "Array.contains",
            "Array.indexOf?",
            "Array.toList",
            "Array.findSome?",
            "Array.qsort",
            "Array.binSearch",
            "Array.size_push",
            "Array.get_set_same",
            "Array.get_set_diff",
            "Array.map_size",
            "Array.toList_length",
        ];
        for name in &names {
            assert!(
                env.get(&Name::str(*name)).is_some(),
                "missing declaration: {}",
                name
            );
        }
    }
    #[test]
    fn test_all_array_decls_are_axioms() {
        let mut env = setup_env();
        build_array_env(&mut env).expect("build_array_env should succeed");
        let names = [
            "Array",
            "Array.get",
            "Array.set",
            "Array.empty",
            "Array.mk",
            "Array.mkEmpty",
            "Array.size",
            "Array.push",
            "Array.pop",
            "Array.swap",
            "Array.map",
            "Array.foldl",
            "Array.foldr",
            "Array.filter",
            "Array.append",
            "Array.reverse",
            "Array.zip",
            "Array.enumerate",
            "Array.take",
            "Array.drop",
            "Array.any",
            "Array.all",
            "Array.contains",
            "Array.indexOf?",
            "Array.toList",
            "Array.findSome?",
            "Array.qsort",
            "Array.binSearch",
            "Array.size_push",
            "Array.get_set_same",
            "Array.get_set_diff",
            "Array.map_size",
            "Array.toList_length",
        ];
        for name in &names {
            let decl = env
                .get(&Name::str(*name))
                .expect("operation should succeed");
            assert!(
                matches!(decl, Declaration::Axiom { .. }),
                "{} should be an axiom",
                name
            );
        }
    }
    #[test]
    fn test_array_declaration_count() {
        let mut env = setup_env();
        let pre = env.len();
        build_array_env(&mut env).expect("build_array_env should succeed");
        let added = env.len() - pre;
        assert!(added >= 30, "expected >= 30 declarations, got {}", added);
    }
    #[test]
    fn test_array_push_type_depth() {
        let mut env = setup_env();
        build_array_env(&mut env).expect("build_array_env should succeed");
        let decl = env
            .get(&Name::str("Array.push"))
            .expect("declaration 'Array.push' should exist in env");
        let mut ty = decl.ty().clone();
        let mut depth = 0;
        while let Expr::Pi(_, _, _, body) = ty {
            depth += 1;
            ty = *body;
        }
        assert!(depth >= 4, "push should have >= 4 Pi levels, got {}", depth);
    }
    #[test]
    fn test_array_foldl_type_depth() {
        let mut env = setup_env();
        build_array_env(&mut env).expect("build_array_env should succeed");
        let decl = env
            .get(&Name::str("Array.foldl"))
            .expect("declaration 'Array.foldl' should exist in env");
        let mut ty = decl.ty().clone();
        let mut depth = 0;
        while let Expr::Pi(_, _, _, body) = ty {
            depth += 1;
            ty = *body;
        }
        assert!(
            depth >= 6,
            "foldl should have >= 6 Pi levels, got {}",
            depth
        );
    }
}
/// `Array.map_id : ∀ {α n}, Array α n → Prop`
///
/// Functor identity law: mapping the identity function over an array returns
/// the same array. `map id a = a`.
#[allow(dead_code)]
pub fn arr_ext_map_id_ty() -> Expr {
    implicit_pi(
        "α",
        type1(),
        implicit_pi(
            "n",
            nat_ty(),
            default_pi("a", array_of(Expr::BVar(1), Expr::BVar(0)), prop()),
        ),
    )
}
/// `Array.map_comp : ∀ {α β γ n}, (β → γ) → (α → β) → Array α n → Prop`
///
/// Functor composition law: mapping (g ∘ f) over an array equals mapping g
/// after mapping f. `map (g ∘ f) a = map g (map f a)`.
#[allow(dead_code)]
pub fn arr_ext_map_comp_ty() -> Expr {
    implicit_pi(
        "α",
        type1(),
        implicit_pi(
            "β",
            type1(),
            implicit_pi(
                "γ",
                type1(),
                implicit_pi(
                    "n",
                    nat_ty(),
                    default_pi(
                        "g",
                        arrow(Expr::BVar(2), Expr::BVar(1)),
                        default_pi(
                            "f",
                            arrow(Expr::BVar(4), Expr::BVar(3)),
                            default_pi("a", array_of(Expr::BVar(5), Expr::BVar(2)), prop()),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// `Array.pure_map_size : ∀ {α β n}, (α → β) → Array α n → Prop`
///
/// Applicative functor preservation of size under pure/map:
/// `size (map f a) = size a`.
#[allow(dead_code)]
pub fn arr_ext_pure_map_size_ty() -> Expr {
    implicit_pi(
        "α",
        type1(),
        implicit_pi(
            "β",
            type1(),
            implicit_pi(
                "n",
                nat_ty(),
                default_pi(
                    "f",
                    arrow(Expr::BVar(2), Expr::BVar(1)),
                    default_pi("a", array_of(Expr::BVar(3), Expr::BVar(1)), prop()),
                ),
            ),
        ),
    )
}
/// `Array.bind_assoc : ∀ {α β γ n}, Array α n → (α → Array β n) → (β → Array γ n) → Prop`
///
/// Monad associativity (kleisli composition): `(a >>= f) >>= g = a >>= (λx, f x >>= g)`.
#[allow(dead_code)]
pub fn arr_ext_bind_assoc_ty() -> Expr {
    implicit_pi(
        "α",
        type1(),
        implicit_pi(
            "β",
            type1(),
            implicit_pi(
                "γ",
                type1(),
                implicit_pi(
                    "n",
                    nat_ty(),
                    default_pi(
                        "a",
                        array_of(Expr::BVar(3), Expr::BVar(0)),
                        default_pi(
                            "f",
                            arrow(Expr::BVar(4), array_of(Expr::BVar(3), Expr::BVar(2))),
                            default_pi(
                                "g",
                                arrow(Expr::BVar(4), array_of(Expr::BVar(3), Expr::BVar(1))),
                                prop(),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// `Array.mergesort : {α : Type} → {n : Nat} → \[Ord α\] → Array α n → Array α n`
///
/// Merge sort: a stable O(n log n) sorting algorithm that returns a sorted
/// permutation of the input array.
#[allow(dead_code)]
pub fn arr_ext_mergesort_ty() -> Expr {
    implicit_pi(
        "α",
        type1(),
        implicit_pi(
            "n",
            nat_ty(),
            inst_pi(
                "inst",
                ord_of(Expr::BVar(1)),
                default_pi(
                    "arr",
                    array_of(Expr::BVar(2), Expr::BVar(1)),
                    array_of(Expr::BVar(3), Expr::BVar(2)),
                ),
            ),
        ),
    )
}
/// `Array.sort_stable : ∀ {α n}, \[Ord α\] → Array α n → Prop`
///
/// Stability of merge sort: elements with equal keys retain their original
/// relative order. A stable sort preserves the original ordering for equal elements.
#[allow(dead_code)]
pub fn arr_ext_sort_stable_ty() -> Expr {
    implicit_pi(
        "α",
        type1(),
        implicit_pi(
            "n",
            nat_ty(),
            inst_pi(
                "inst",
                ord_of(Expr::BVar(1)),
                default_pi("arr", array_of(Expr::BVar(2), Expr::BVar(1)), prop()),
            ),
        ),
    )
}
/// `Array.sort_perm : ∀ {α n}, \[Ord α\] → Array α n → Prop`
///
/// Correctness of sort as a permutation: the sorted result is a permutation
/// of the input (no elements added or dropped).
#[allow(dead_code)]
pub fn arr_ext_sort_perm_ty() -> Expr {
    implicit_pi(
        "α",
        type1(),
        implicit_pi(
            "n",
            nat_ty(),
            inst_pi(
                "inst",
                ord_of(Expr::BVar(1)),
                default_pi("arr", array_of(Expr::BVar(2), Expr::BVar(1)), prop()),
            ),
        ),
    )
}
/// `Array.sort_sorted : ∀ {α n}, \[Ord α\] → Array α n → Prop`
///
/// Output of sort satisfies the sorted predicate: for all i < j,
/// `get (sort a) i ≤ get (sort a) j`.
#[allow(dead_code)]
pub fn arr_ext_sort_sorted_ty() -> Expr {
    implicit_pi(
        "α",
        type1(),
        implicit_pi(
            "n",
            nat_ty(),
            inst_pi(
                "inst",
                ord_of(Expr::BVar(1)),
                default_pi("arr", array_of(Expr::BVar(2), Expr::BVar(1)), prop()),
            ),
        ),
    )
}
/// `Array.qsort_average_case : ∀ {α n}, \[Ord α\] → Array α n → Prop`
///
/// Quicksort average-case complexity: for a random permutation of n elements,
/// expected O(n log n) comparisons with randomized pivot selection.
#[allow(dead_code)]
pub fn arr_ext_qsort_avg_ty() -> Expr {
    implicit_pi(
        "α",
        type1(),
        implicit_pi(
            "n",
            nat_ty(),
            inst_pi(
                "inst",
                ord_of(Expr::BVar(1)),
                default_pi("arr", array_of(Expr::BVar(2), Expr::BVar(1)), prop()),
            ),
        ),
    )
}
/// `Array.reverse_involution : ∀ {α n}, Array α n → Prop`
///
/// Reversal is an involution: `reverse (reverse a) = a`.
#[allow(dead_code)]
pub fn arr_ext_reverse_involution_ty() -> Expr {
    implicit_pi(
        "α",
        type1(),
        implicit_pi(
            "n",
            nat_ty(),
            default_pi("a", array_of(Expr::BVar(1), Expr::BVar(0)), prop()),
        ),
    )
}
/// `Array.reverse_size : ∀ {α n}, Array α n → Prop`
///
/// Reversal preserves size: `size (reverse a) = size a`.
#[allow(dead_code)]
pub fn arr_ext_reverse_size_ty() -> Expr {
    implicit_pi(
        "α",
        type1(),
        implicit_pi(
            "n",
            nat_ty(),
            default_pi("a", array_of(Expr::BVar(1), Expr::BVar(0)), prop()),
        ),
    )
}
/// `Array.append_assoc : ∀ {α n m k}, Array α n → Array α m → Array α k → Prop`
///
/// Append is associative: `(a ++ b) ++ c = a ++ (b ++ c)`.
#[allow(dead_code)]
pub fn arr_ext_append_assoc_ty() -> Expr {
    implicit_pi(
        "α",
        type1(),
        implicit_pi(
            "n",
            nat_ty(),
            implicit_pi(
                "m",
                nat_ty(),
                implicit_pi(
                    "k",
                    nat_ty(),
                    default_pi(
                        "a",
                        array_of(Expr::BVar(3), Expr::BVar(2)),
                        default_pi(
                            "b",
                            array_of(Expr::BVar(4), Expr::BVar(2)),
                            default_pi("c", array_of(Expr::BVar(5), Expr::BVar(2)), prop()),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// `Array.append_empty_left : ∀ {α n}, Array α n → Prop`
///
/// Left identity of append: `empty ++ a = a`.
#[allow(dead_code)]
pub fn arr_ext_append_empty_left_ty() -> Expr {
    implicit_pi(
        "α",
        type1(),
        implicit_pi(
            "n",
            nat_ty(),
            default_pi("a", array_of(Expr::BVar(1), Expr::BVar(0)), prop()),
        ),
    )
}
/// `Array.append_empty_right : ∀ {α n}, Array α n → Prop`
///
/// Right identity of append: `a ++ empty = a`.
#[allow(dead_code)]
pub fn arr_ext_append_empty_right_ty() -> Expr {
    implicit_pi(
        "α",
        type1(),
        implicit_pi(
            "n",
            nat_ty(),
            default_pi("a", array_of(Expr::BVar(1), Expr::BVar(0)), prop()),
        ),
    )
}
/// `Array.append_size : ∀ {α n m}, Array α n → Array α m → Prop`
///
/// Size of append equals sum: `size (a ++ b) = size a + size b`.
#[allow(dead_code)]
pub fn arr_ext_append_size_ty() -> Expr {
    implicit_pi(
        "α",
        type1(),
        implicit_pi(
            "n",
            nat_ty(),
            implicit_pi(
                "m",
                nat_ty(),
                default_pi(
                    "a",
                    array_of(Expr::BVar(2), Expr::BVar(1)),
                    default_pi("b", array_of(Expr::BVar(3), Expr::BVar(1)), prop()),
                ),
            ),
        ),
    )
}
/// `Array.slice : {α : Type} → {n : Nat} → Array α n → Nat → Nat → List α`
///
/// Array slice: extract a sub-array from index `lo` to `hi` (exclusive),
/// returning it as a list. Used in range queries and substring operations.
#[allow(dead_code)]
pub fn arr_ext_slice_ty() -> Expr {
    implicit_pi(
        "α",
        type1(),
        implicit_pi(
            "n",
            nat_ty(),
            default_pi(
                "arr",
                array_of(Expr::BVar(1), Expr::BVar(0)),
                default_pi(
                    "lo",
                    nat_ty(),
                    default_pi("hi", nat_ty(), list_of(Expr::BVar(4))),
                ),
            ),
        ),
    )
}
/// `Array.prefix_sum : {α : Type} → {n : Nat} → Array α n → Array α n`
///
/// Prefix sum (cumulative sum): given an array a, compute b where
/// b\[i\] = a\[0\] + a\[1\] + ... + a\[i\]. Enables O(1) range sum queries.
#[allow(dead_code)]
pub fn arr_ext_prefix_sum_ty() -> Expr {
    implicit_pi(
        "α",
        type1(),
        implicit_pi(
            "n",
            nat_ty(),
            default_pi(
                "arr",
                array_of(Expr::BVar(1), Expr::BVar(0)),
                array_of(Expr::BVar(2), Expr::BVar(1)),
            ),
        ),
    )
}
