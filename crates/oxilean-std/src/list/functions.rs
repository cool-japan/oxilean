//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{
    BinderInfo, Declaration, Environment, Expr, InductiveEnv, InductiveType, IntroRule, Level, Name,
};

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
/// `Prod` applied to two type arguments.
#[allow(dead_code)]
pub fn prod_of(a: Expr, b: Expr) -> Expr {
    app2(Expr::Const(Name::str("Prod"), vec![]), a, b)
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
/// Build `Eq @{} ty a b`.
#[allow(dead_code)]
pub fn eq_expr(ty: Expr, a: Expr, b: Expr) -> Expr {
    app(app(app(Expr::Const(Name::str("Eq"), vec![]), ty), a), b)
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
/// Build the List type and all standard operations, adding them to the
/// environment.
///
/// Assumes that `Nat`, `Bool`, `Option`, `Prod`, and `Eq` are already in
/// the environment (or will be referenced by name).
pub fn build_list_env(env: &mut Environment, ind_env: &mut InductiveEnv) -> Result<(), String> {
    let list_ty = arrow(type1(), type1());
    let nil_ty = implicit_pi("alpha", type1(), list_of(Expr::BVar(0)));
    let cons_ty = implicit_pi(
        "alpha",
        type1(),
        default_pi(
            "hd",
            Expr::BVar(0),
            default_pi("tl", list_of(Expr::BVar(1)), list_of(Expr::BVar(2))),
        ),
    );
    let list_ind = InductiveType::new(
        Name::str("List"),
        vec![],
        1,
        0,
        list_ty.clone(),
        vec![
            IntroRule {
                name: Name::str("List.nil"),
                ty: nil_ty.clone(),
            },
            IntroRule {
                name: Name::str("List.cons"),
                ty: cons_ty.clone(),
            },
        ],
    );
    ind_env.add(list_ind).map_err(|e| format!("{}", e))?;
    add_axiom(env, "List", vec![], list_ty)?;
    add_axiom(env, "List.nil", vec![], nil_ty)?;
    add_axiom(env, "List.cons", vec![], cons_ty)?;
    let u = Name::str("u");
    let sort_u = Expr::Sort(Level::Param(u.clone()));
    let c_ty = arrow(list_of(Expr::BVar(0)), sort_u);
    let nil_case_ty = app(
        Expr::BVar(1),
        app(Expr::Const(Name::str("List.nil"), vec![]), Expr::BVar(2)),
    );
    let cons_case_ty = default_pi(
        "hd",
        Expr::BVar(2),
        default_pi(
            "tl",
            list_of(Expr::BVar(3)),
            default_pi(
                "ih",
                app(Expr::BVar(3), Expr::BVar(0)),
                app(
                    Expr::BVar(4),
                    app2(
                        Expr::Const(Name::str("List.cons"), vec![]),
                        Expr::BVar(2),
                        Expr::BVar(1),
                    ),
                ),
            ),
        ),
    );
    let target = default_pi(
        "l",
        list_of(Expr::BVar(3)),
        app(Expr::BVar(4), Expr::BVar(0)),
    );
    let rec_ty = implicit_pi(
        "alpha",
        type1(),
        implicit_pi(
            "C",
            c_ty,
            default_pi(
                "nil_case",
                nil_case_ty,
                default_pi("cons_case", cons_case_ty, target),
            ),
        ),
    );
    add_axiom(env, "List.rec", vec![u], rec_ty)?;
    add_axiom(
        env,
        "List.map",
        vec![],
        implicit_pi(
            "alpha",
            type1(),
            implicit_pi(
                "beta",
                type1(),
                default_pi(
                    "f",
                    arrow(Expr::BVar(1), Expr::BVar(0)),
                    default_pi("l", list_of(Expr::BVar(2)), list_of(Expr::BVar(2))),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "List.filter",
        vec![],
        implicit_pi(
            "alpha",
            type1(),
            default_pi(
                "p",
                arrow(Expr::BVar(0), bool_ty()),
                default_pi("l", list_of(Expr::BVar(1)), list_of(Expr::BVar(2))),
            ),
        ),
    )?;
    {
        let f_ty = Expr::Pi(
            BinderInfo::Default,
            Name::str("_"),
            Box::new(Expr::BVar(1)),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("_"),
                Box::new(Expr::BVar(1)),
                Box::new(Expr::BVar(2)),
            )),
        );
        add_axiom(
            env,
            "List.foldr",
            vec![],
            implicit_pi(
                "alpha",
                type1(),
                implicit_pi(
                    "beta",
                    type1(),
                    default_pi(
                        "f",
                        f_ty,
                        default_pi(
                            "init",
                            Expr::BVar(1),
                            default_pi("l", list_of(Expr::BVar(3)), Expr::BVar(3)),
                        ),
                    ),
                ),
            ),
        )?;
    }
    {
        let f_ty = Expr::Pi(
            BinderInfo::Default,
            Name::str("_"),
            Box::new(Expr::BVar(0)),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("_"),
                Box::new(Expr::BVar(2)),
                Box::new(Expr::BVar(2)),
            )),
        );
        add_axiom(
            env,
            "List.foldl",
            vec![],
            implicit_pi(
                "alpha",
                type1(),
                implicit_pi(
                    "beta",
                    type1(),
                    default_pi(
                        "f",
                        f_ty,
                        default_pi(
                            "init",
                            Expr::BVar(1),
                            default_pi("l", list_of(Expr::BVar(3)), Expr::BVar(3)),
                        ),
                    ),
                ),
            ),
        )?;
    }
    add_axiom(
        env,
        "List.reverse",
        vec![],
        implicit_pi(
            "alpha",
            type1(),
            default_pi("l", list_of(Expr::BVar(0)), list_of(Expr::BVar(1))),
        ),
    )?;
    add_axiom(
        env,
        "List.length",
        vec![],
        implicit_pi(
            "alpha",
            type1(),
            default_pi("l", list_of(Expr::BVar(0)), nat_ty()),
        ),
    )?;
    add_axiom(
        env,
        "List.append",
        vec![],
        implicit_pi(
            "alpha",
            type1(),
            default_pi(
                "l1",
                list_of(Expr::BVar(0)),
                default_pi("l2", list_of(Expr::BVar(1)), list_of(Expr::BVar(2))),
            ),
        ),
    )?;
    add_axiom(
        env,
        "List.head?",
        vec![],
        implicit_pi(
            "alpha",
            type1(),
            default_pi("l", list_of(Expr::BVar(0)), option_of(Expr::BVar(1))),
        ),
    )?;
    add_axiom(
        env,
        "List.tail",
        vec![],
        implicit_pi(
            "alpha",
            type1(),
            default_pi("l", list_of(Expr::BVar(0)), list_of(Expr::BVar(1))),
        ),
    )?;
    add_axiom(
        env,
        "List.nth?",
        vec![],
        implicit_pi(
            "alpha",
            type1(),
            default_pi(
                "l",
                list_of(Expr::BVar(0)),
                default_pi("n", nat_ty(), option_of(Expr::BVar(2))),
            ),
        ),
    )?;
    add_axiom(
        env,
        "List.zip",
        vec![],
        implicit_pi(
            "alpha",
            type1(),
            implicit_pi(
                "beta",
                type1(),
                default_pi(
                    "l1",
                    list_of(Expr::BVar(1)),
                    default_pi(
                        "l2",
                        list_of(Expr::BVar(1)),
                        list_of(prod_of(Expr::BVar(3), Expr::BVar(2))),
                    ),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "List.take",
        vec![],
        implicit_pi(
            "alpha",
            type1(),
            default_pi(
                "n",
                nat_ty(),
                default_pi("l", list_of(Expr::BVar(1)), list_of(Expr::BVar(2))),
            ),
        ),
    )?;
    add_axiom(
        env,
        "List.drop",
        vec![],
        implicit_pi(
            "alpha",
            type1(),
            default_pi(
                "n",
                nat_ty(),
                default_pi("l", list_of(Expr::BVar(1)), list_of(Expr::BVar(2))),
            ),
        ),
    )?;
    add_axiom(
        env,
        "List.any",
        vec![],
        implicit_pi(
            "alpha",
            type1(),
            default_pi(
                "p",
                arrow(Expr::BVar(0), bool_ty()),
                default_pi("l", list_of(Expr::BVar(1)), bool_ty()),
            ),
        ),
    )?;
    add_axiom(
        env,
        "List.all",
        vec![],
        implicit_pi(
            "alpha",
            type1(),
            default_pi(
                "p",
                arrow(Expr::BVar(0), bool_ty()),
                default_pi("l", list_of(Expr::BVar(1)), bool_ty()),
            ),
        ),
    )?;
    add_axiom(
        env,
        "List.replicate",
        vec![],
        implicit_pi(
            "alpha",
            type1(),
            default_pi(
                "n",
                nat_ty(),
                default_pi("x", Expr::BVar(1), list_of(Expr::BVar(2))),
            ),
        ),
    )?;
    add_axiom(
        env,
        "List.join",
        vec![],
        implicit_pi(
            "alpha",
            type1(),
            default_pi(
                "ll",
                list_of(list_of(Expr::BVar(0))),
                list_of(Expr::BVar(1)),
            ),
        ),
    )?;
    add_axiom(env, "List.iota", vec![], arrow(nat_ty(), list_of(nat_ty())))?;
    add_axiom(
        env,
        "List.range",
        vec![],
        arrow(nat_ty(), list_of(nat_ty())),
    )?;
    add_axiom(
        env,
        "List.enumFrom",
        vec![],
        implicit_pi(
            "alpha",
            type1(),
            default_pi(
                "start",
                nat_ty(),
                default_pi(
                    "l",
                    list_of(Expr::BVar(1)),
                    list_of(prod_of(nat_ty(), Expr::BVar(2))),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "List.nil_append",
        vec![],
        implicit_pi(
            "alpha",
            type1(),
            default_pi(
                "l",
                list_of(Expr::BVar(0)),
                eq_expr(
                    list_of(Expr::BVar(1)),
                    app2(
                        Expr::Const(Name::str("List.append"), vec![]),
                        app(Expr::Const(Name::str("List.nil"), vec![]), Expr::BVar(1)),
                        Expr::BVar(0),
                    ),
                    Expr::BVar(0),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "List.append_nil",
        vec![],
        implicit_pi(
            "alpha",
            type1(),
            default_pi(
                "l",
                list_of(Expr::BVar(0)),
                eq_expr(
                    list_of(Expr::BVar(1)),
                    app2(
                        Expr::Const(Name::str("List.append"), vec![]),
                        Expr::BVar(0),
                        app(Expr::Const(Name::str("List.nil"), vec![]), Expr::BVar(1)),
                    ),
                    Expr::BVar(0),
                ),
            ),
        ),
    )?;
    {
        let append = Expr::Const(Name::str("List.append"), vec![]);
        add_axiom(
            env,
            "List.append_assoc",
            vec![],
            implicit_pi(
                "alpha",
                type1(),
                default_pi(
                    "l1",
                    list_of(Expr::BVar(0)),
                    default_pi(
                        "l2",
                        list_of(Expr::BVar(1)),
                        default_pi(
                            "l3",
                            list_of(Expr::BVar(2)),
                            eq_expr(
                                list_of(Expr::BVar(3)),
                                app2(
                                    append.clone(),
                                    app2(append.clone(), Expr::BVar(2), Expr::BVar(1)),
                                    Expr::BVar(0),
                                ),
                                app2(
                                    append.clone(),
                                    Expr::BVar(2),
                                    app2(append, Expr::BVar(1), Expr::BVar(0)),
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
        "List.length_nil",
        vec![],
        implicit_pi(
            "alpha",
            type1(),
            eq_expr(
                nat_ty(),
                app(
                    Expr::Const(Name::str("List.length"), vec![]),
                    app(Expr::Const(Name::str("List.nil"), vec![]), Expr::BVar(0)),
                ),
                Expr::Const(Name::str("Nat.zero"), vec![]),
            ),
        ),
    )?;
    add_axiom(
        env,
        "List.length_cons",
        vec![],
        implicit_pi(
            "alpha",
            type1(),
            default_pi(
                "a",
                Expr::BVar(0),
                default_pi(
                    "l",
                    list_of(Expr::BVar(1)),
                    eq_expr(
                        nat_ty(),
                        app(
                            Expr::Const(Name::str("List.length"), vec![]),
                            app2(
                                Expr::Const(Name::str("List.cons"), vec![]),
                                Expr::BVar(1),
                                Expr::BVar(0),
                            ),
                        ),
                        app(
                            Expr::Const(Name::str("Nat.succ"), vec![]),
                            app(Expr::Const(Name::str("List.length"), vec![]), Expr::BVar(0)),
                        ),
                    ),
                ),
            ),
        ),
    )?;
    {
        let length = Expr::Const(Name::str("List.length"), vec![]);
        let append_c = Expr::Const(Name::str("List.append"), vec![]);
        let add_c = Expr::Const(Name::str("Nat.add"), vec![]);
        add_axiom(
            env,
            "List.length_append",
            vec![],
            implicit_pi(
                "alpha",
                type1(),
                default_pi(
                    "l1",
                    list_of(Expr::BVar(0)),
                    default_pi(
                        "l2",
                        list_of(Expr::BVar(1)),
                        eq_expr(
                            nat_ty(),
                            app(length.clone(), app2(append_c, Expr::BVar(1), Expr::BVar(0))),
                            app2(
                                add_c,
                                app(length.clone(), Expr::BVar(1)),
                                app(length, Expr::BVar(0)),
                            ),
                        ),
                    ),
                ),
            ),
        )?;
    }
    add_axiom(
        env,
        "List.map_nil",
        vec![],
        implicit_pi(
            "alpha",
            type1(),
            implicit_pi(
                "beta",
                type1(),
                default_pi(
                    "f",
                    arrow(Expr::BVar(1), Expr::BVar(0)),
                    eq_expr(
                        list_of(Expr::BVar(1)),
                        app2(
                            Expr::Const(Name::str("List.map"), vec![]),
                            Expr::BVar(0),
                            app(Expr::Const(Name::str("List.nil"), vec![]), Expr::BVar(2)),
                        ),
                        app(Expr::Const(Name::str("List.nil"), vec![]), Expr::BVar(1)),
                    ),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "List.map_cons",
        vec![],
        implicit_pi(
            "alpha",
            type1(),
            implicit_pi(
                "beta",
                type1(),
                default_pi(
                    "f",
                    arrow(Expr::BVar(1), Expr::BVar(0)),
                    default_pi(
                        "a",
                        Expr::BVar(2),
                        default_pi(
                            "l",
                            list_of(Expr::BVar(3)),
                            eq_expr(
                                list_of(Expr::BVar(3)),
                                app2(
                                    Expr::Const(Name::str("List.map"), vec![]),
                                    Expr::BVar(2),
                                    app2(
                                        Expr::Const(Name::str("List.cons"), vec![]),
                                        Expr::BVar(1),
                                        Expr::BVar(0),
                                    ),
                                ),
                                app2(
                                    Expr::Const(Name::str("List.cons"), vec![]),
                                    app(Expr::BVar(2), Expr::BVar(1)),
                                    app2(
                                        Expr::Const(Name::str("List.map"), vec![]),
                                        Expr::BVar(2),
                                        Expr::BVar(0),
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "List.map_map",
        vec![],
        implicit_pi(
            "alpha",
            type1(),
            implicit_pi(
                "beta",
                type1(),
                implicit_pi(
                    "gamma",
                    type1(),
                    default_pi(
                        "f",
                        arrow(Expr::BVar(2), Expr::BVar(1)),
                        default_pi(
                            "g",
                            arrow(Expr::BVar(2), Expr::BVar(1)),
                            default_pi(
                                "l",
                                list_of(Expr::BVar(4)),
                                eq_expr(
                                    list_of(Expr::BVar(3)),
                                    app2(
                                        Expr::Const(Name::str("List.map"), vec![]),
                                        Expr::BVar(1),
                                        app2(
                                            Expr::Const(Name::str("List.map"), vec![]),
                                            Expr::BVar(2),
                                            Expr::BVar(0),
                                        ),
                                    ),
                                    app2(
                                        Expr::Const(Name::str("List.map"), vec![]),
                                        Expr::Lam(
                                            BinderInfo::Default,
                                            Name::str("x"),
                                            Box::new(Expr::BVar(5)),
                                            Box::new(app(
                                                Expr::BVar(2),
                                                app(Expr::BVar(3), Expr::BVar(0)),
                                            )),
                                        ),
                                        Expr::BVar(0),
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "List.map_id",
        vec![],
        implicit_pi(
            "alpha",
            type1(),
            default_pi(
                "l",
                list_of(Expr::BVar(0)),
                eq_expr(
                    list_of(Expr::BVar(1)),
                    app2(
                        Expr::Const(Name::str("List.map"), vec![]),
                        Expr::Lam(
                            BinderInfo::Default,
                            Name::str("x"),
                            Box::new(Expr::BVar(1)),
                            Box::new(Expr::BVar(0)),
                        ),
                        Expr::BVar(0),
                    ),
                    Expr::BVar(0),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "List.reverse_nil",
        vec![],
        implicit_pi(
            "alpha",
            type1(),
            eq_expr(
                list_of(Expr::BVar(0)),
                app(
                    Expr::Const(Name::str("List.reverse"), vec![]),
                    app(Expr::Const(Name::str("List.nil"), vec![]), Expr::BVar(0)),
                ),
                app(Expr::Const(Name::str("List.nil"), vec![]), Expr::BVar(0)),
            ),
        ),
    )?;
    add_axiom(
        env,
        "List.reverse_cons",
        vec![],
        implicit_pi(
            "alpha",
            type1(),
            default_pi(
                "a",
                Expr::BVar(0),
                default_pi(
                    "l",
                    list_of(Expr::BVar(1)),
                    eq_expr(
                        list_of(Expr::BVar(2)),
                        app(
                            Expr::Const(Name::str("List.reverse"), vec![]),
                            app2(
                                Expr::Const(Name::str("List.cons"), vec![]),
                                Expr::BVar(1),
                                Expr::BVar(0),
                            ),
                        ),
                        app2(
                            Expr::Const(Name::str("List.append"), vec![]),
                            app(
                                Expr::Const(Name::str("List.reverse"), vec![]),
                                Expr::BVar(0),
                            ),
                            app2(
                                Expr::Const(Name::str("List.cons"), vec![]),
                                Expr::BVar(1),
                                app(Expr::Const(Name::str("List.nil"), vec![]), Expr::BVar(2)),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "List.reverse_reverse",
        vec![],
        implicit_pi(
            "alpha",
            type1(),
            default_pi(
                "l",
                list_of(Expr::BVar(0)),
                eq_expr(
                    list_of(Expr::BVar(1)),
                    app(
                        Expr::Const(Name::str("List.reverse"), vec![]),
                        app(
                            Expr::Const(Name::str("List.reverse"), vec![]),
                            Expr::BVar(0),
                        ),
                    ),
                    Expr::BVar(0),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "List.filter_nil",
        vec![],
        implicit_pi(
            "alpha",
            type1(),
            default_pi(
                "p",
                arrow(Expr::BVar(0), bool_ty()),
                eq_expr(
                    list_of(Expr::BVar(1)),
                    app2(
                        Expr::Const(Name::str("List.filter"), vec![]),
                        Expr::BVar(0),
                        app(Expr::Const(Name::str("List.nil"), vec![]), Expr::BVar(1)),
                    ),
                    app(Expr::Const(Name::str("List.nil"), vec![]), Expr::BVar(1)),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "List.foldr_nil",
        vec![],
        implicit_pi(
            "alpha",
            type1(),
            implicit_pi(
                "beta",
                type1(),
                default_pi(
                    "f",
                    arrow(Expr::BVar(1), arrow(Expr::BVar(1), Expr::BVar(1))),
                    default_pi(
                        "b",
                        Expr::BVar(1),
                        eq_expr(
                            Expr::BVar(2),
                            app3(
                                Expr::Const(Name::str("List.foldr"), vec![]),
                                Expr::BVar(1),
                                Expr::BVar(0),
                                app(Expr::Const(Name::str("List.nil"), vec![]), Expr::BVar(3)),
                            ),
                            Expr::BVar(0),
                        ),
                    ),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "List.foldl_nil",
        vec![],
        implicit_pi(
            "alpha",
            type1(),
            implicit_pi(
                "beta",
                type1(),
                default_pi(
                    "f",
                    arrow(Expr::BVar(0), arrow(Expr::BVar(2), Expr::BVar(1))),
                    default_pi(
                        "b",
                        Expr::BVar(1),
                        eq_expr(
                            Expr::BVar(2),
                            app3(
                                Expr::Const(Name::str("List.foldl"), vec![]),
                                Expr::BVar(1),
                                Expr::BVar(0),
                                app(Expr::Const(Name::str("List.nil"), vec![]), Expr::BVar(3)),
                            ),
                            Expr::BVar(0),
                        ),
                    ),
                ),
            ),
        ),
    )?;
    {
        let length = Expr::Const(Name::str("List.length"), vec![]);
        let reverse = Expr::Const(Name::str("List.reverse"), vec![]);
        add_axiom(
            env,
            "List.length_reverse",
            vec![],
            implicit_pi(
                "alpha",
                type1(),
                default_pi(
                    "l",
                    list_of(Expr::BVar(0)),
                    eq_expr(
                        nat_ty(),
                        app(length.clone(), app(reverse, Expr::BVar(0))),
                        app(length, Expr::BVar(0)),
                    ),
                ),
            ),
        )?;
    }
    {
        let length = Expr::Const(Name::str("List.length"), vec![]);
        let map_c = Expr::Const(Name::str("List.map"), vec![]);
        add_axiom(
            env,
            "List.length_map",
            vec![],
            implicit_pi(
                "alpha",
                type1(),
                implicit_pi(
                    "beta",
                    type1(),
                    default_pi(
                        "f",
                        arrow(Expr::BVar(1), Expr::BVar(0)),
                        default_pi(
                            "l",
                            list_of(Expr::BVar(2)),
                            eq_expr(
                                nat_ty(),
                                app(length.clone(), app2(map_c, Expr::BVar(1), Expr::BVar(0))),
                                app(length, Expr::BVar(0)),
                            ),
                        ),
                    ),
                ),
            ),
        )?;
    }
    {
        let length = Expr::Const(Name::str("List.length"), vec![]);
        let filter_c = Expr::Const(Name::str("List.filter"), vec![]);
        let le_c = Expr::Const(Name::str("Nat.le"), vec![]);
        add_axiom(
            env,
            "List.length_filter_le",
            vec![],
            implicit_pi(
                "alpha",
                type1(),
                default_pi(
                    "p",
                    arrow(Expr::BVar(0), bool_ty()),
                    default_pi(
                        "l",
                        list_of(Expr::BVar(1)),
                        app2(
                            le_c,
                            app(length.clone(), app2(filter_c, Expr::BVar(1), Expr::BVar(0))),
                            app(length, Expr::BVar(0)),
                        ),
                    ),
                ),
            ),
        )?;
    }
    add_axiom(
        env,
        "List.mem_nil",
        vec![],
        implicit_pi(
            "alpha",
            type1(),
            default_pi(
                "a",
                Expr::BVar(0),
                arrow(
                    app2(
                        Expr::Const(Name::str("List.Mem"), vec![]),
                        Expr::BVar(0),
                        app(Expr::Const(Name::str("List.nil"), vec![]), Expr::BVar(1)),
                    ),
                    Expr::Const(Name::str("False"), vec![]),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "List.mem_cons",
        vec![],
        implicit_pi(
            "alpha",
            type1(),
            default_pi(
                "a",
                Expr::BVar(0),
                default_pi(
                    "b",
                    Expr::BVar(1),
                    default_pi(
                        "l",
                        list_of(Expr::BVar(2)),
                        app2(
                            Expr::Const(Name::str("Iff"), vec![]),
                            app2(
                                Expr::Const(Name::str("List.Mem"), vec![]),
                                Expr::BVar(2),
                                app2(
                                    Expr::Const(Name::str("List.cons"), vec![]),
                                    Expr::BVar(1),
                                    Expr::BVar(0),
                                ),
                            ),
                            app2(
                                Expr::Const(Name::str("Or"), vec![]),
                                eq_expr(Expr::BVar(3), Expr::BVar(2), Expr::BVar(1)),
                                app2(
                                    Expr::Const(Name::str("List.Mem"), vec![]),
                                    Expr::BVar(2),
                                    Expr::BVar(0),
                                ),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "List.unzip",
        vec![],
        implicit_pi(
            "alpha",
            type1(),
            implicit_pi(
                "beta",
                type1(),
                default_pi(
                    "l",
                    list_of(prod_of(Expr::BVar(1), Expr::BVar(0))),
                    prod_of(list_of(Expr::BVar(2)), list_of(Expr::BVar(1))),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "List.partition",
        vec![],
        implicit_pi(
            "alpha",
            type1(),
            default_pi(
                "p",
                arrow(Expr::BVar(0), bool_ty()),
                default_pi(
                    "l",
                    list_of(Expr::BVar(1)),
                    prod_of(list_of(Expr::BVar(2)), list_of(Expr::BVar(2))),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "List.span",
        vec![],
        implicit_pi(
            "alpha",
            type1(),
            default_pi(
                "p",
                arrow(Expr::BVar(0), bool_ty()),
                default_pi(
                    "l",
                    list_of(Expr::BVar(1)),
                    prod_of(list_of(Expr::BVar(2)), list_of(Expr::BVar(2))),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "List.find?",
        vec![],
        implicit_pi(
            "alpha",
            type1(),
            default_pi(
                "p",
                arrow(Expr::BVar(0), bool_ty()),
                default_pi("l", list_of(Expr::BVar(1)), option_of(Expr::BVar(2))),
            ),
        ),
    )?;
    add_axiom(
        env,
        "List.count",
        vec![],
        implicit_pi(
            "alpha",
            type1(),
            default_pi(
                "p",
                arrow(Expr::BVar(0), bool_ty()),
                default_pi("l", list_of(Expr::BVar(1)), nat_ty()),
            ),
        ),
    )?;
    add_axiom(
        env,
        "List.intersperse",
        vec![],
        implicit_pi(
            "alpha",
            type1(),
            default_pi(
                "sep",
                Expr::BVar(0),
                default_pi("l", list_of(Expr::BVar(1)), list_of(Expr::BVar(2))),
            ),
        ),
    )?;
    add_axiom(
        env,
        "List.transpose",
        vec![],
        implicit_pi(
            "alpha",
            type1(),
            default_pi(
                "ll",
                list_of(list_of(Expr::BVar(0))),
                list_of(list_of(Expr::BVar(1))),
            ),
        ),
    )?;
    add_axiom(
        env,
        "List.Perm",
        vec![],
        implicit_pi(
            "alpha",
            type1(),
            default_pi(
                "l1",
                list_of(Expr::BVar(0)),
                default_pi("l2", list_of(Expr::BVar(1)), prop()),
            ),
        ),
    )?;
    add_axiom(
        env,
        "List.Perm.refl",
        vec![],
        implicit_pi(
            "alpha",
            type1(),
            default_pi(
                "l",
                list_of(Expr::BVar(0)),
                app2(
                    Expr::Const(Name::str("List.Perm"), vec![]),
                    Expr::BVar(0),
                    Expr::BVar(0),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "List.Perm.symm",
        vec![],
        implicit_pi(
            "alpha",
            type1(),
            default_pi(
                "l1",
                list_of(Expr::BVar(0)),
                default_pi(
                    "l2",
                    list_of(Expr::BVar(1)),
                    arrow(
                        app2(
                            Expr::Const(Name::str("List.Perm"), vec![]),
                            Expr::BVar(1),
                            Expr::BVar(0),
                        ),
                        app2(
                            Expr::Const(Name::str("List.Perm"), vec![]),
                            Expr::BVar(0),
                            Expr::BVar(1),
                        ),
                    ),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "List.Perm.trans",
        vec![],
        implicit_pi(
            "alpha",
            type1(),
            default_pi(
                "l1",
                list_of(Expr::BVar(0)),
                default_pi(
                    "l2",
                    list_of(Expr::BVar(1)),
                    default_pi(
                        "l3",
                        list_of(Expr::BVar(2)),
                        arrow(
                            app2(
                                Expr::Const(Name::str("List.Perm"), vec![]),
                                Expr::BVar(2),
                                Expr::BVar(1),
                            ),
                            arrow(
                                app2(
                                    Expr::Const(Name::str("List.Perm"), vec![]),
                                    Expr::BVar(1),
                                    Expr::BVar(0),
                                ),
                                app2(
                                    Expr::Const(Name::str("List.Perm"), vec![]),
                                    Expr::BVar(2),
                                    Expr::BVar(0),
                                ),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "List.Sublist",
        vec![],
        implicit_pi(
            "alpha",
            type1(),
            default_pi(
                "l1",
                list_of(Expr::BVar(0)),
                default_pi("l2", list_of(Expr::BVar(1)), prop()),
            ),
        ),
    )?;
    add_axiom(
        env,
        "List.Sublist.refl",
        vec![],
        implicit_pi(
            "alpha",
            type1(),
            default_pi(
                "l",
                list_of(Expr::BVar(0)),
                app2(
                    Expr::Const(Name::str("List.Sublist"), vec![]),
                    Expr::BVar(0),
                    Expr::BVar(0),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "List.Sublist.trans",
        vec![],
        implicit_pi(
            "alpha",
            type1(),
            default_pi(
                "l1",
                list_of(Expr::BVar(0)),
                default_pi(
                    "l2",
                    list_of(Expr::BVar(1)),
                    default_pi(
                        "l3",
                        list_of(Expr::BVar(2)),
                        arrow(
                            app2(
                                Expr::Const(Name::str("List.Sublist"), vec![]),
                                Expr::BVar(2),
                                Expr::BVar(1),
                            ),
                            arrow(
                                app2(
                                    Expr::Const(Name::str("List.Sublist"), vec![]),
                                    Expr::BVar(1),
                                    Expr::BVar(0),
                                ),
                                app2(
                                    Expr::Const(Name::str("List.Sublist"), vec![]),
                                    Expr::BVar(2),
                                    Expr::BVar(0),
                                ),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "List.Decidable.mem",
        vec![],
        implicit_pi(
            "alpha",
            type1(),
            default_pi(
                "a",
                Expr::BVar(0),
                default_pi(
                    "l",
                    list_of(Expr::BVar(1)),
                    app(
                        Expr::Const(Name::str("Decidable"), vec![]),
                        app2(
                            Expr::Const(Name::str("List.Mem"), vec![]),
                            Expr::BVar(1),
                            Expr::BVar(0),
                        ),
                    ),
                ),
            ),
        ),
    )?;
    {
        let take_c = Expr::Const(Name::str("List.take"), vec![]);
        let append_c = Expr::Const(Name::str("List.append"), vec![]);
        let length_c = Expr::Const(Name::str("List.length"), vec![]);
        let sub_c = Expr::Const(Name::str("Nat.sub"), vec![]);
        add_axiom(
            env,
            "List.take_append",
            vec![],
            implicit_pi(
                "alpha",
                type1(),
                default_pi(
                    "n",
                    nat_ty(),
                    default_pi(
                        "l1",
                        list_of(Expr::BVar(1)),
                        default_pi(
                            "l2",
                            list_of(Expr::BVar(2)),
                            eq_expr(
                                list_of(Expr::BVar(3)),
                                app2(
                                    take_c.clone(),
                                    Expr::BVar(2),
                                    app2(append_c.clone(), Expr::BVar(1), Expr::BVar(0)),
                                ),
                                app2(
                                    append_c,
                                    app2(take_c.clone(), Expr::BVar(2), Expr::BVar(1)),
                                    app2(
                                        take_c,
                                        app2(sub_c, Expr::BVar(2), app(length_c, Expr::BVar(1))),
                                        Expr::BVar(0),
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
        let drop_c = Expr::Const(Name::str("List.drop"), vec![]);
        let append_c = Expr::Const(Name::str("List.append"), vec![]);
        let length_c = Expr::Const(Name::str("List.length"), vec![]);
        let sub_c = Expr::Const(Name::str("Nat.sub"), vec![]);
        add_axiom(
            env,
            "List.drop_append",
            vec![],
            implicit_pi(
                "alpha",
                type1(),
                default_pi(
                    "n",
                    nat_ty(),
                    default_pi(
                        "l1",
                        list_of(Expr::BVar(1)),
                        default_pi(
                            "l2",
                            list_of(Expr::BVar(2)),
                            eq_expr(
                                list_of(Expr::BVar(3)),
                                app2(
                                    drop_c.clone(),
                                    Expr::BVar(2),
                                    app2(append_c.clone(), Expr::BVar(1), Expr::BVar(0)),
                                ),
                                app2(
                                    append_c,
                                    app2(drop_c.clone(), Expr::BVar(2), Expr::BVar(1)),
                                    app2(
                                        drop_c,
                                        app2(sub_c, Expr::BVar(2), app(length_c, Expr::BVar(1))),
                                        Expr::BVar(0),
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
        let count_c = Expr::Const(Name::str("List.count"), vec![]);
        let length_c = Expr::Const(Name::str("List.length"), vec![]);
        let le_c = Expr::Const(Name::str("Nat.le"), vec![]);
        add_axiom(
            env,
            "List.count_le_length",
            vec![],
            implicit_pi(
                "alpha",
                type1(),
                default_pi(
                    "p",
                    arrow(Expr::BVar(0), bool_ty()),
                    default_pi(
                        "l",
                        list_of(Expr::BVar(1)),
                        app2(
                            le_c,
                            app2(count_c, Expr::BVar(1), Expr::BVar(0)),
                            app(length_c, Expr::BVar(0)),
                        ),
                    ),
                ),
            ),
        )?;
    }
    Ok(())
}
/// Create a `List` type expression applied to an element type.
pub fn list_type(elem_ty: Expr) -> Expr {
    app(Expr::Const(Name::str("List"), vec![]), elem_ty)
}
/// Create an empty list (nil) for a given element type.
pub fn list_nil(elem_ty: Expr) -> Expr {
    app(Expr::Const(Name::str("List.nil"), vec![]), elem_ty)
}
/// Create a cons cell: `List.cons head tail`.
pub fn list_cons(head: Expr, tail: Expr) -> Expr {
    app2(Expr::Const(Name::str("List.cons"), vec![]), head, tail)
}
/// Create a `List.map f l` expression.
#[allow(dead_code)]
pub fn list_map(f: Expr, l: Expr) -> Expr {
    app2(Expr::Const(Name::str("List.map"), vec![]), f, l)
}
/// Create a `List.filter pred l` expression.
#[allow(dead_code)]
pub fn list_filter(pred: Expr, l: Expr) -> Expr {
    app2(Expr::Const(Name::str("List.filter"), vec![]), pred, l)
}
/// Create a `List.foldr f init l` expression.
#[allow(dead_code)]
pub fn list_foldr(f: Expr, init: Expr, l: Expr) -> Expr {
    app3(Expr::Const(Name::str("List.foldr"), vec![]), f, init, l)
}
/// Create a `List.foldl f init l` expression.
#[allow(dead_code)]
pub fn list_foldl(f: Expr, init: Expr, l: Expr) -> Expr {
    app3(Expr::Const(Name::str("List.foldl"), vec![]), f, init, l)
}
/// Create a `List.reverse l` expression.
#[allow(dead_code)]
pub fn list_reverse(l: Expr) -> Expr {
    app(Expr::Const(Name::str("List.reverse"), vec![]), l)
}
/// Create a `List.length l` expression.
#[allow(dead_code)]
pub fn list_length(l: Expr) -> Expr {
    app(Expr::Const(Name::str("List.length"), vec![]), l)
}
/// Create a `List.append l1 l2` expression.
#[allow(dead_code)]
pub fn list_append(l1: Expr, l2: Expr) -> Expr {
    app2(Expr::Const(Name::str("List.append"), vec![]), l1, l2)
}
/// Create a `List.head? l` expression.
#[allow(dead_code)]
pub fn list_head(l: Expr) -> Expr {
    app(Expr::Const(Name::str("List.head?"), vec![]), l)
}
/// Create a `List.tail l` expression.
#[allow(dead_code)]
pub fn list_tail(l: Expr) -> Expr {
    app(Expr::Const(Name::str("List.tail"), vec![]), l)
}
/// Create a `List.take n l` expression.
#[allow(dead_code)]
pub fn list_take(n: Expr, l: Expr) -> Expr {
    app2(Expr::Const(Name::str("List.take"), vec![]), n, l)
}
/// Create a `List.drop n l` expression.
#[allow(dead_code)]
pub fn list_drop(n: Expr, l: Expr) -> Expr {
    app2(Expr::Const(Name::str("List.drop"), vec![]), n, l)
}
/// Create a `List.replicate n x` expression.
#[allow(dead_code)]
pub fn list_replicate(n: Expr, x: Expr) -> Expr {
    app2(Expr::Const(Name::str("List.replicate"), vec![]), n, x)
}
/// Create a `List.join ll` expression.
#[allow(dead_code)]
pub fn list_join(ll: Expr) -> Expr {
    app(Expr::Const(Name::str("List.join"), vec![]), ll)
}
/// Create a `List.range n` expression.
#[allow(dead_code)]
pub fn list_range(n: Expr) -> Expr {
    app(Expr::Const(Name::str("List.range"), vec![]), n)
}
/// Build a list expression from a Rust Vec of element expressions.
///
/// `mk_list_from_vec(elem_ty, vec![a, b, c])` produces
/// `List.cons a (List.cons b (List.cons c (List.nil elem_ty)))`.
#[allow(dead_code)]
pub fn mk_list_from_vec(elem_ty: Expr, elems: Vec<Expr>) -> Expr {
    let mut result = list_nil(elem_ty);
    for e in elems.into_iter().rev() {
        result = list_cons(e, result);
    }
    result
}
