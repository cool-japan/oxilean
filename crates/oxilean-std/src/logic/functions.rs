//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

pub fn app(f: Expr, a: Expr) -> Expr {
    Expr::App(Box::new(f), Box::new(a))
}
pub fn app2(f: Expr, a: Expr, b: Expr) -> Expr {
    app(app(f, a), b)
}
pub fn app3(f: Expr, a: Expr, b: Expr, c: Expr) -> Expr {
    app(app2(f, a, b), c)
}
pub fn app4(f: Expr, a: Expr, b: Expr, c: Expr, d: Expr) -> Expr {
    app(app3(f, a, b, c), d)
}
pub fn pi(bi: BinderInfo, name: &str, dom: Expr, body: Expr) -> Expr {
    Expr::Pi(bi, Name::str(name), Box::new(dom), Box::new(body))
}
#[allow(dead_code)]
pub fn lam(bi: BinderInfo, name: &str, dom: Expr, body: Expr) -> Expr {
    Expr::Lam(bi, Name::str(name), Box::new(dom), Box::new(body))
}
pub fn cst(s: &str) -> Expr {
    Expr::Const(Name::str(s), vec![])
}
pub fn cst_u(s: &str, levels: Vec<Level>) -> Expr {
    Expr::Const(Name::str(s), levels)
}
pub fn prop() -> Expr {
    Expr::Sort(Level::zero())
}
pub fn sort_u() -> Expr {
    Expr::Sort(Level::Param(Name::str("u")))
}
pub fn sort_v() -> Expr {
    Expr::Sort(Level::Param(Name::str("v")))
}
pub fn bvar(n: u32) -> Expr {
    Expr::BVar(n)
}
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
/// Build the logic environment containing all core logical types and axioms.
#[allow(clippy::too_many_lines)]
pub fn build_logic_env(env: &mut Environment) -> Result<(), String> {
    add_axiom(env, "True", vec![], prop())?;
    add_axiom(env, "True.intro", vec![], cst("True"))?;
    add_axiom(env, "False", vec![], prop())?;
    let false_elim_ty = pi(
        BinderInfo::Implicit,
        "α",
        sort_u(),
        pi(BinderInfo::Default, "_", cst("False"), bvar(1)),
    );
    add_axiom(env, "False.elim", vec![Name::str("u")], false_elim_ty)?;
    let not_ty = pi(BinderInfo::Default, "p", prop(), prop());
    add_axiom(env, "Not", vec![], not_ty)?;
    let and_ty = pi(
        BinderInfo::Default,
        "a",
        prop(),
        pi(BinderInfo::Default, "b", prop(), prop()),
    );
    add_axiom(env, "And", vec![], and_ty)?;
    let and_intro_ty = pi(
        BinderInfo::Implicit,
        "a",
        prop(),
        pi(
            BinderInfo::Implicit,
            "b",
            prop(),
            pi(
                BinderInfo::Default,
                "ha",
                bvar(1),
                pi(
                    BinderInfo::Default,
                    "hb",
                    bvar(1),
                    app2(cst("And"), bvar(3), bvar(2)),
                ),
            ),
        ),
    );
    add_axiom(env, "And.intro", vec![], and_intro_ty)?;
    let and_left_ty = pi(
        BinderInfo::Implicit,
        "a",
        prop(),
        pi(
            BinderInfo::Implicit,
            "b",
            prop(),
            pi(
                BinderInfo::Default,
                "h",
                app2(cst("And"), bvar(1), bvar(0)),
                bvar(2),
            ),
        ),
    );
    add_axiom(env, "And.left", vec![], and_left_ty)?;
    let and_right_ty = pi(
        BinderInfo::Implicit,
        "a",
        prop(),
        pi(
            BinderInfo::Implicit,
            "b",
            prop(),
            pi(
                BinderInfo::Default,
                "h",
                app2(cst("And"), bvar(1), bvar(0)),
                bvar(1),
            ),
        ),
    );
    add_axiom(env, "And.right", vec![], and_right_ty)?;
    let or_ty = pi(
        BinderInfo::Default,
        "a",
        prop(),
        pi(BinderInfo::Default, "b", prop(), prop()),
    );
    add_axiom(env, "Or", vec![], or_ty)?;
    let or_inl_ty = pi(
        BinderInfo::Implicit,
        "a",
        prop(),
        pi(
            BinderInfo::Implicit,
            "b",
            prop(),
            pi(
                BinderInfo::Default,
                "ha",
                bvar(1),
                app2(cst("Or"), bvar(2), bvar(1)),
            ),
        ),
    );
    add_axiom(env, "Or.inl", vec![], or_inl_ty)?;
    let or_inr_ty = pi(
        BinderInfo::Implicit,
        "a",
        prop(),
        pi(
            BinderInfo::Implicit,
            "b",
            prop(),
            pi(
                BinderInfo::Default,
                "hb",
                bvar(0),
                app2(cst("Or"), bvar(2), bvar(1)),
            ),
        ),
    );
    add_axiom(env, "Or.inr", vec![], or_inr_ty)?;
    let or_elim_ty = pi(
        BinderInfo::Implicit,
        "a",
        prop(),
        pi(
            BinderInfo::Implicit,
            "b",
            prop(),
            pi(
                BinderInfo::Implicit,
                "c",
                prop(),
                pi(
                    BinderInfo::Default,
                    "h",
                    app2(cst("Or"), bvar(2), bvar(1)),
                    pi(
                        BinderInfo::Default,
                        "f",
                        pi(BinderInfo::Default, "_", bvar(3), bvar(1)),
                        pi(
                            BinderInfo::Default,
                            "g",
                            pi(BinderInfo::Default, "_", bvar(3), bvar(2)),
                            bvar(3),
                        ),
                    ),
                ),
            ),
        ),
    );
    add_axiom(env, "Or.elim", vec![], or_elim_ty)?;
    let iff_ty = pi(
        BinderInfo::Default,
        "a",
        prop(),
        pi(BinderInfo::Default, "b", prop(), prop()),
    );
    add_axiom(env, "Iff", vec![], iff_ty)?;
    let iff_intro_ty = pi(
        BinderInfo::Implicit,
        "a",
        prop(),
        pi(
            BinderInfo::Implicit,
            "b",
            prop(),
            pi(
                BinderInfo::Default,
                "f",
                pi(BinderInfo::Default, "_", bvar(1), bvar(0)),
                pi(
                    BinderInfo::Default,
                    "g",
                    pi(BinderInfo::Default, "_", bvar(1), bvar(2)),
                    app2(cst("Iff"), bvar(3), bvar(2)),
                ),
            ),
        ),
    );
    add_axiom(env, "Iff.intro", vec![], iff_intro_ty)?;
    let iff_mp_ty = pi(
        BinderInfo::Implicit,
        "a",
        prop(),
        pi(
            BinderInfo::Implicit,
            "b",
            prop(),
            pi(
                BinderInfo::Default,
                "h",
                app2(cst("Iff"), bvar(1), bvar(0)),
                pi(BinderInfo::Default, "ha", bvar(2), bvar(1)),
            ),
        ),
    );
    add_axiom(env, "Iff.mp", vec![], iff_mp_ty)?;
    let iff_mpr_ty = pi(
        BinderInfo::Implicit,
        "a",
        prop(),
        pi(
            BinderInfo::Implicit,
            "b",
            prop(),
            pi(
                BinderInfo::Default,
                "h",
                app2(cst("Iff"), bvar(1), bvar(0)),
                pi(BinderInfo::Default, "hb", bvar(1), bvar(2)),
            ),
        ),
    );
    add_axiom(env, "Iff.mpr", vec![], iff_mpr_ty)?;
    let exists_ty = pi(
        BinderInfo::Implicit,
        "α",
        sort_u(),
        pi(
            BinderInfo::Default,
            "p",
            pi(BinderInfo::Default, "_", bvar(0), prop()),
            prop(),
        ),
    );
    add_axiom(env, "Exists", vec![Name::str("u")], exists_ty)?;
    let exists_intro_ty = pi(
        BinderInfo::Implicit,
        "α",
        sort_u(),
        pi(
            BinderInfo::Implicit,
            "p",
            pi(BinderInfo::Default, "_", bvar(0), prop()),
            pi(
                BinderInfo::Default,
                "w",
                bvar(1),
                pi(
                    BinderInfo::Default,
                    "h",
                    app(bvar(1), bvar(0)),
                    app(cst_u("Exists", vec![Level::Param(Name::str("u"))]), bvar(2)),
                ),
            ),
        ),
    );
    add_axiom(env, "Exists.intro", vec![Name::str("u")], exists_intro_ty)?;
    let eq_ty = pi(
        BinderInfo::Implicit,
        "α",
        sort_u(),
        pi(
            BinderInfo::Default,
            "a",
            bvar(0),
            pi(BinderInfo::Default, "b", bvar(1), prop()),
        ),
    );
    add_axiom(env, "Eq", vec![Name::str("u")], eq_ty)?;
    let eq_refl_ty = pi(
        BinderInfo::Implicit,
        "α",
        sort_u(),
        pi(
            BinderInfo::Default,
            "a",
            bvar(0),
            app2(
                cst_u("Eq", vec![Level::Param(Name::str("u"))]),
                bvar(0),
                bvar(0),
            ),
        ),
    );
    add_axiom(env, "Eq.refl", vec![Name::str("u")], eq_refl_ty)?;
    let eq_symm_ty = pi(
        BinderInfo::Implicit,
        "α",
        sort_u(),
        pi(
            BinderInfo::Implicit,
            "a",
            bvar(0),
            pi(
                BinderInfo::Implicit,
                "b",
                bvar(1),
                pi(
                    BinderInfo::Default,
                    "h",
                    app2(
                        cst_u("Eq", vec![Level::Param(Name::str("u"))]),
                        bvar(1),
                        bvar(0),
                    ),
                    app2(
                        cst_u("Eq", vec![Level::Param(Name::str("u"))]),
                        bvar(1),
                        bvar(2),
                    ),
                ),
            ),
        ),
    );
    add_axiom(env, "Eq.symm", vec![Name::str("u")], eq_symm_ty)?;
    let eq_trans_ty = pi(
        BinderInfo::Implicit,
        "α",
        sort_u(),
        pi(
            BinderInfo::Implicit,
            "a",
            bvar(0),
            pi(
                BinderInfo::Implicit,
                "b",
                bvar(1),
                pi(
                    BinderInfo::Implicit,
                    "c",
                    bvar(2),
                    pi(
                        BinderInfo::Default,
                        "h1",
                        app2(
                            cst_u("Eq", vec![Level::Param(Name::str("u"))]),
                            bvar(2),
                            bvar(1),
                        ),
                        pi(
                            BinderInfo::Default,
                            "h2",
                            app2(
                                cst_u("Eq", vec![Level::Param(Name::str("u"))]),
                                bvar(2),
                                bvar(1),
                            ),
                            app2(
                                cst_u("Eq", vec![Level::Param(Name::str("u"))]),
                                bvar(4),
                                bvar(2),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    );
    add_axiom(env, "Eq.trans", vec![Name::str("u")], eq_trans_ty)?;
    let eq_subst_ty = pi(
        BinderInfo::Implicit,
        "α",
        sort_u(),
        pi(
            BinderInfo::Implicit,
            "a",
            bvar(0),
            pi(
                BinderInfo::Implicit,
                "b",
                bvar(1),
                pi(
                    BinderInfo::Implicit,
                    "p",
                    pi(BinderInfo::Default, "_", bvar(2), prop()),
                    pi(
                        BinderInfo::Default,
                        "h",
                        app2(
                            cst_u("Eq", vec![Level::Param(Name::str("u"))]),
                            bvar(2),
                            bvar(1),
                        ),
                        pi(
                            BinderInfo::Default,
                            "hp",
                            app(bvar(1), bvar(3)),
                            app(bvar(2), bvar(3)),
                        ),
                    ),
                ),
            ),
        ),
    );
    add_axiom(env, "Eq.subst", vec![Name::str("u")], eq_subst_ty)?;
    let eq_rec_ty = pi(
        BinderInfo::Implicit,
        "α",
        sort_u(),
        pi(
            BinderInfo::Implicit,
            "a",
            bvar(0),
            pi(
                BinderInfo::Implicit,
                "motive",
                pi(
                    BinderInfo::Default,
                    "b",
                    bvar(1),
                    pi(
                        BinderInfo::Default,
                        "_",
                        app2(
                            cst_u("Eq", vec![Level::Param(Name::str("u"))]),
                            bvar(2),
                            bvar(0),
                        ),
                        sort_v(),
                    ),
                ),
                pi(
                    BinderInfo::Default,
                    "h_refl",
                    app2(
                        bvar(0),
                        bvar(1),
                        app(
                            cst_u("Eq.refl", vec![Level::Param(Name::str("u"))]),
                            bvar(1),
                        ),
                    ),
                    pi(
                        BinderInfo::Implicit,
                        "b",
                        bvar(3),
                        pi(
                            BinderInfo::Default,
                            "h",
                            app2(
                                cst_u("Eq", vec![Level::Param(Name::str("u"))]),
                                bvar(3),
                                bvar(0),
                            ),
                            app2(bvar(3), bvar(1), bvar(0)),
                        ),
                    ),
                ),
            ),
        ),
    );
    add_axiom(
        env,
        "Eq.rec",
        vec![Name::str("u"), Name::str("v")],
        eq_rec_ty,
    )?;
    let heq_ty = pi(
        BinderInfo::Implicit,
        "α",
        sort_u(),
        pi(
            BinderInfo::Default,
            "a",
            bvar(0),
            pi(
                BinderInfo::Implicit,
                "β",
                sort_u(),
                pi(BinderInfo::Default, "b", bvar(0), prop()),
            ),
        ),
    );
    add_axiom(env, "HEq", vec![Name::str("u")], heq_ty)?;
    let heq_refl_ty = pi(
        BinderInfo::Implicit,
        "α",
        sort_u(),
        pi(
            BinderInfo::Default,
            "a",
            bvar(0),
            app4(
                cst_u("HEq", vec![Level::Param(Name::str("u"))]),
                bvar(0),
                bvar(0),
                bvar(0),
                bvar(0),
            ),
        ),
    );
    add_axiom(env, "HEq.refl", vec![Name::str("u")], heq_refl_ty)?;
    let cast_ty = pi(
        BinderInfo::Implicit,
        "α",
        sort_u(),
        pi(
            BinderInfo::Implicit,
            "β",
            sort_u(),
            pi(
                BinderInfo::Default,
                "h",
                app2(
                    cst_u("Eq", vec![Level::succ(Level::Param(Name::str("u")))]),
                    bvar(1),
                    bvar(0),
                ),
                pi(BinderInfo::Default, "a", bvar(2), bvar(1)),
            ),
        ),
    );
    add_axiom(env, "cast", vec![Name::str("u")], cast_ty)?;
    let congr_ty = pi(
        BinderInfo::Implicit,
        "α",
        sort_u(),
        pi(
            BinderInfo::Implicit,
            "β",
            sort_u(),
            pi(
                BinderInfo::Implicit,
                "f",
                pi(BinderInfo::Default, "_", bvar(1), bvar(0)),
                pi(
                    BinderInfo::Implicit,
                    "g",
                    pi(BinderInfo::Default, "_", bvar(2), bvar(1)),
                    pi(
                        BinderInfo::Implicit,
                        "a",
                        bvar(3),
                        pi(
                            BinderInfo::Implicit,
                            "b",
                            bvar(4),
                            pi(
                                BinderInfo::Default,
                                "hfg",
                                app2(
                                    cst_u("Eq", vec![Level::Param(Name::str("u"))]),
                                    bvar(3),
                                    bvar(2),
                                ),
                                pi(
                                    BinderInfo::Default,
                                    "hab",
                                    app2(
                                        cst_u("Eq", vec![Level::Param(Name::str("u"))]),
                                        bvar(2),
                                        bvar(1),
                                    ),
                                    app2(
                                        cst_u("Eq", vec![Level::Param(Name::str("u"))]),
                                        app(bvar(5), bvar(3)),
                                        app(bvar(4), bvar(2)),
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    );
    add_axiom(env, "congr", vec![Name::str("u")], congr_ty)?;
    let congr_arg_ty = pi(
        BinderInfo::Implicit,
        "α",
        sort_u(),
        pi(
            BinderInfo::Implicit,
            "β",
            sort_u(),
            pi(
                BinderInfo::Implicit,
                "a",
                bvar(1),
                pi(
                    BinderInfo::Implicit,
                    "b",
                    bvar(2),
                    pi(
                        BinderInfo::Default,
                        "f",
                        pi(BinderInfo::Default, "_", bvar(3), bvar(2)),
                        pi(
                            BinderInfo::Default,
                            "h",
                            app2(
                                cst_u("Eq", vec![Level::Param(Name::str("u"))]),
                                bvar(2),
                                bvar(1),
                            ),
                            app2(
                                cst_u("Eq", vec![Level::Param(Name::str("u"))]),
                                app(bvar(1), bvar(3)),
                                app(bvar(1), bvar(2)),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    );
    add_axiom(env, "congrArg", vec![Name::str("u")], congr_arg_ty)?;
    let congr_fun_ty = pi(
        BinderInfo::Implicit,
        "α",
        sort_u(),
        pi(
            BinderInfo::Implicit,
            "β",
            sort_u(),
            pi(
                BinderInfo::Implicit,
                "f",
                pi(BinderInfo::Default, "_", bvar(1), bvar(0)),
                pi(
                    BinderInfo::Implicit,
                    "g",
                    pi(BinderInfo::Default, "_", bvar(2), bvar(1)),
                    pi(
                        BinderInfo::Default,
                        "h",
                        app2(
                            cst_u("Eq", vec![Level::Param(Name::str("u"))]),
                            bvar(1),
                            bvar(0),
                        ),
                        pi(
                            BinderInfo::Default,
                            "a",
                            bvar(4),
                            app2(
                                cst_u("Eq", vec![Level::Param(Name::str("u"))]),
                                app(bvar(3), bvar(0)),
                                app(bvar(2), bvar(0)),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    );
    add_axiom(env, "congrFun", vec![Name::str("u")], congr_fun_ty)?;
    let funext_ty = pi(
        BinderInfo::Implicit,
        "α",
        sort_u(),
        pi(
            BinderInfo::Implicit,
            "β",
            pi(BinderInfo::Default, "_", bvar(0), sort_v()),
            pi(
                BinderInfo::Implicit,
                "f",
                pi(BinderInfo::Default, "a", bvar(1), app(bvar(1), bvar(0))),
                pi(
                    BinderInfo::Implicit,
                    "g",
                    pi(BinderInfo::Default, "a", bvar(2), app(bvar(2), bvar(0))),
                    pi(
                        BinderInfo::Default,
                        "h",
                        pi(
                            BinderInfo::Default,
                            "a",
                            bvar(3),
                            app2(
                                cst_u("Eq", vec![Level::Param(Name::str("v"))]),
                                app(bvar(2), bvar(0)),
                                app(bvar(1), bvar(0)),
                            ),
                        ),
                        app2(
                            cst_u("Eq", vec![Level::Param(Name::str("u"))]),
                            bvar(2),
                            bvar(1),
                        ),
                    ),
                ),
            ),
        ),
    );
    add_axiom(
        env,
        "funext",
        vec![Name::str("u"), Name::str("v")],
        funext_ty,
    )?;
    let propext_ty = pi(
        BinderInfo::Implicit,
        "a",
        prop(),
        pi(
            BinderInfo::Implicit,
            "b",
            prop(),
            pi(
                BinderInfo::Default,
                "h",
                app2(cst("Iff"), bvar(1), bvar(0)),
                app2(
                    cst_u("Eq", vec![Level::succ(Level::zero())]),
                    bvar(2),
                    bvar(1),
                ),
            ),
        ),
    );
    add_axiom(env, "propext", vec![], propext_ty)?;
    let nonempty_ty = pi(BinderInfo::Default, "α", sort_u(), prop());
    add_axiom(env, "Nonempty", vec![Name::str("u")], nonempty_ty)?;
    let nonempty_intro_ty = pi(
        BinderInfo::Implicit,
        "α",
        sort_u(),
        pi(
            BinderInfo::Default,
            "a",
            bvar(0),
            app(
                cst_u("Nonempty", vec![Level::Param(Name::str("u"))]),
                bvar(1),
            ),
        ),
    );
    add_axiom(
        env,
        "Nonempty.intro",
        vec![Name::str("u")],
        nonempty_intro_ty,
    )?;
    let em_ty = pi(
        BinderInfo::Default,
        "p",
        prop(),
        app2(cst("Or"), bvar(0), app(cst("Not"), bvar(0))),
    );
    add_axiom(env, "Classical.em", vec![], em_ty)?;
    let choice_ty = pi(
        BinderInfo::Implicit,
        "α",
        sort_u(),
        pi(
            BinderInfo::Default,
            "h",
            app(
                cst_u("Nonempty", vec![Level::Param(Name::str("u"))]),
                bvar(0),
            ),
            bvar(1),
        ),
    );
    add_axiom(env, "Classical.choice", vec![Name::str("u")], choice_ty)?;
    let absurd_ty = pi(
        BinderInfo::Implicit,
        "a",
        prop(),
        pi(
            BinderInfo::Implicit,
            "b",
            prop(),
            pi(
                BinderInfo::Default,
                "ha",
                bvar(1),
                pi(
                    BinderInfo::Default,
                    "hna",
                    app(cst("Not"), bvar(2)),
                    bvar(2),
                ),
            ),
        ),
    );
    add_axiom(env, "absurd", vec![], absurd_ty)?;
    let not_not_ty = pi(
        BinderInfo::Implicit,
        "a",
        prop(),
        pi(
            BinderInfo::Default,
            "h",
            app(cst("Not"), app(cst("Not"), bvar(0))),
            bvar(1),
        ),
    );
    add_axiom(env, "not_not", vec![], not_not_ty)?;
    let and_comm_ty = pi(
        BinderInfo::Implicit,
        "a",
        prop(),
        pi(
            BinderInfo::Implicit,
            "b",
            prop(),
            app2(
                cst("Iff"),
                app2(cst("And"), bvar(1), bvar(0)),
                app2(cst("And"), bvar(0), bvar(1)),
            ),
        ),
    );
    add_axiom(env, "And.comm", vec![], and_comm_ty)?;
    let or_comm_ty = pi(
        BinderInfo::Implicit,
        "a",
        prop(),
        pi(
            BinderInfo::Implicit,
            "b",
            prop(),
            app2(
                cst("Iff"),
                app2(cst("Or"), bvar(1), bvar(0)),
                app2(cst("Or"), bvar(0), bvar(1)),
            ),
        ),
    );
    add_axiom(env, "Or.comm", vec![], or_comm_ty)?;
    let and_assoc_ty = pi(
        BinderInfo::Implicit,
        "a",
        prop(),
        pi(
            BinderInfo::Implicit,
            "b",
            prop(),
            pi(
                BinderInfo::Implicit,
                "c",
                prop(),
                app2(
                    cst("Iff"),
                    app2(cst("And"), app2(cst("And"), bvar(2), bvar(1)), bvar(0)),
                    app2(cst("And"), bvar(2), app2(cst("And"), bvar(1), bvar(0))),
                ),
            ),
        ),
    );
    add_axiom(env, "And.assoc", vec![], and_assoc_ty)?;
    let or_assoc_ty = pi(
        BinderInfo::Implicit,
        "a",
        prop(),
        pi(
            BinderInfo::Implicit,
            "b",
            prop(),
            pi(
                BinderInfo::Implicit,
                "c",
                prop(),
                app2(
                    cst("Iff"),
                    app2(cst("Or"), app2(cst("Or"), bvar(2), bvar(1)), bvar(0)),
                    app2(cst("Or"), bvar(2), app2(cst("Or"), bvar(1), bvar(0))),
                ),
            ),
        ),
    );
    add_axiom(env, "Or.assoc", vec![], or_assoc_ty)?;
    let iff_refl_ty = pi(
        BinderInfo::Implicit,
        "a",
        prop(),
        app2(cst("Iff"), bvar(0), bvar(0)),
    );
    add_axiom(env, "iff_refl", vec![], iff_refl_ty)?;
    let iff_comm_ty = pi(
        BinderInfo::Implicit,
        "a",
        prop(),
        pi(
            BinderInfo::Implicit,
            "b",
            prop(),
            app2(
                cst("Iff"),
                app2(cst("Iff"), bvar(1), bvar(0)),
                app2(cst("Iff"), bvar(0), bvar(1)),
            ),
        ),
    );
    add_axiom(env, "iff_comm", vec![], iff_comm_ty)?;
    let iff_trans_ty = pi(
        BinderInfo::Implicit,
        "a",
        prop(),
        pi(
            BinderInfo::Implicit,
            "b",
            prop(),
            pi(
                BinderInfo::Implicit,
                "c",
                prop(),
                pi(
                    BinderInfo::Default,
                    "h1",
                    app2(cst("Iff"), bvar(2), bvar(1)),
                    pi(
                        BinderInfo::Default,
                        "h2",
                        app2(cst("Iff"), bvar(2), bvar(1)),
                        app2(cst("Iff"), bvar(4), bvar(2)),
                    ),
                ),
            ),
        ),
    );
    add_axiom(env, "iff_trans", vec![], iff_trans_ty)?;
    let dne_ty = pi(
        BinderInfo::Implicit,
        "a",
        prop(),
        pi(
            BinderInfo::Default,
            "h",
            app(cst("Not"), app(cst("Not"), bvar(0))),
            bvar(1),
        ),
    );
    add_axiom(env, "dne", vec![], dne_ty)?;
    let contrapositive_ty = pi(
        BinderInfo::Implicit,
        "a",
        prop(),
        pi(
            BinderInfo::Implicit,
            "b",
            prop(),
            pi(
                BinderInfo::Default,
                "f",
                pi(BinderInfo::Default, "_", bvar(1), bvar(0)),
                pi(
                    BinderInfo::Default,
                    "hnb",
                    app(cst("Not"), bvar(1)),
                    app(cst("Not"), bvar(3)),
                ),
            ),
        ),
    );
    add_axiom(env, "contrapositive", vec![], contrapositive_ty)?;
    let by_cases_ty = pi(
        BinderInfo::Implicit,
        "p",
        prop(),
        pi(
            BinderInfo::Implicit,
            "q",
            prop(),
            pi(
                BinderInfo::Default,
                "hp",
                pi(BinderInfo::Default, "_", bvar(1), bvar(0)),
                pi(
                    BinderInfo::Default,
                    "hnp",
                    pi(BinderInfo::Default, "_", app(cst("Not"), bvar(2)), bvar(1)),
                    bvar(2),
                ),
            ),
        ),
    );
    add_axiom(env, "by_cases", vec![], by_cases_ty)?;
    let not_and_ty = pi(
        BinderInfo::Implicit,
        "a",
        prop(),
        pi(
            BinderInfo::Implicit,
            "b",
            prop(),
            app2(
                cst("Iff"),
                app(cst("Not"), app2(cst("And"), bvar(1), bvar(0))),
                app2(
                    cst("Or"),
                    app(cst("Not"), bvar(1)),
                    app(cst("Not"), bvar(0)),
                ),
            ),
        ),
    );
    add_axiom(env, "not_and", vec![], not_and_ty)?;
    let not_or_ty = pi(
        BinderInfo::Implicit,
        "a",
        prop(),
        pi(
            BinderInfo::Implicit,
            "b",
            prop(),
            app2(
                cst("Iff"),
                app(cst("Not"), app2(cst("Or"), bvar(1), bvar(0))),
                app2(
                    cst("And"),
                    app(cst("Not"), bvar(1)),
                    app(cst("Not"), bvar(0)),
                ),
            ),
        ),
    );
    add_axiom(env, "not_or", vec![], not_or_ty)?;
    let not_implies_ty = pi(
        BinderInfo::Implicit,
        "a",
        prop(),
        pi(
            BinderInfo::Implicit,
            "b",
            prop(),
            app2(
                cst("Iff"),
                app(cst("Not"), pi(BinderInfo::Default, "_", bvar(1), bvar(0))),
                app2(cst("And"), bvar(1), app(cst("Not"), bvar(0))),
            ),
        ),
    );
    add_axiom(env, "not_implies", vec![], not_implies_ty)?;
    let forall_and_ty = pi(
        BinderInfo::Implicit,
        "α",
        sort_u(),
        pi(
            BinderInfo::Implicit,
            "p",
            pi(BinderInfo::Default, "_", bvar(0), prop()),
            pi(
                BinderInfo::Implicit,
                "q",
                pi(BinderInfo::Default, "_", bvar(1), prop()),
                app2(
                    cst("Iff"),
                    pi(
                        BinderInfo::Default,
                        "x",
                        bvar(2),
                        app2(cst("And"), app(bvar(1), bvar(0)), app(bvar(0), bvar(0))),
                    ),
                    app2(
                        cst("And"),
                        pi(BinderInfo::Default, "x", bvar(3), app(bvar(2), bvar(0))),
                        pi(BinderInfo::Default, "x", bvar(3), app(bvar(1), bvar(0))),
                    ),
                ),
            ),
        ),
    );
    add_axiom(env, "forall_and", vec![Name::str("u")], forall_and_ty)?;
    let exists_or_ty = pi(
        BinderInfo::Implicit,
        "α",
        sort_u(),
        pi(
            BinderInfo::Implicit,
            "p",
            pi(BinderInfo::Default, "_", bvar(0), prop()),
            pi(
                BinderInfo::Implicit,
                "q",
                pi(BinderInfo::Default, "_", bvar(1), prop()),
                app2(
                    cst("Iff"),
                    app(
                        cst_u("Exists", vec![Level::Param(Name::str("u"))]),
                        pi(
                            BinderInfo::Default,
                            "x",
                            bvar(2),
                            app2(cst("Or"), app(bvar(1), bvar(0)), app(bvar(0), bvar(0))),
                        ),
                    ),
                    app2(
                        cst("Or"),
                        app(
                            cst_u("Exists", vec![Level::Param(Name::str("u"))]),
                            pi(BinderInfo::Default, "x", bvar(3), app(bvar(2), bvar(0))),
                        ),
                        app(
                            cst_u("Exists", vec![Level::Param(Name::str("u"))]),
                            pi(BinderInfo::Default, "x", bvar(3), app(bvar(1), bvar(0))),
                        ),
                    ),
                ),
            ),
        ),
    );
    add_axiom(env, "exists_or", vec![Name::str("u")], exists_or_ty)?;
    let forall_or_exists_not_ty = pi(
        BinderInfo::Implicit,
        "α",
        sort_u(),
        pi(
            BinderInfo::Implicit,
            "p",
            pi(BinderInfo::Default, "_", bvar(0), prop()),
            app2(
                cst("Or"),
                pi(BinderInfo::Default, "x", bvar(1), app(bvar(1), bvar(0))),
                app(
                    cst_u("Exists", vec![Level::Param(Name::str("u"))]),
                    pi(
                        BinderInfo::Default,
                        "x",
                        bvar(1),
                        app(cst("Not"), app(bvar(1), bvar(0))),
                    ),
                ),
            ),
        ),
    );
    add_axiom(
        env,
        "forall_or_exists_not",
        vec![Name::str("u")],
        forall_or_exists_not_ty,
    )?;
    env.add(Declaration::Axiom {
        name: Name::str("Decidable"),
        univ_params: vec![],
        ty: pi(BinderInfo::Default, "p", prop(), prop()),
    })
    .map_err(|e| e.to_string())?;
    add_axiom(
        env,
        "is_true",
        vec![],
        pi(BinderInfo::Default, "b", cst("Bool"), prop()),
    )?;
    let decidable_true_ty = pi(
        BinderInfo::Implicit,
        "p",
        prop(),
        pi(
            BinderInfo::Default,
            "h",
            app(cst("Decidable"), bvar(0)),
            app2(cst("Or"), bvar(1), app(cst("Not"), bvar(1))),
        ),
    );
    add_axiom(env, "decidable_true", vec![], decidable_true_ty)?;
    let proof_by_contradiction_ty = pi(
        BinderInfo::Implicit,
        "p",
        prop(),
        pi(
            BinderInfo::Default,
            "h",
            pi(
                BinderInfo::Default,
                "_",
                app(cst("Not"), bvar(0)),
                cst("False"),
            ),
            bvar(1),
        ),
    );
    add_axiom(
        env,
        "proof_by_contradiction",
        vec![],
        proof_by_contradiction_ty,
    )?;
    let iff_of_true_ty = pi(
        BinderInfo::Implicit,
        "a",
        prop(),
        pi(
            BinderInfo::Implicit,
            "b",
            prop(),
            pi(
                BinderInfo::Default,
                "ha",
                bvar(1),
                pi(
                    BinderInfo::Default,
                    "hb",
                    bvar(1),
                    app2(cst("Iff"), bvar(3), bvar(2)),
                ),
            ),
        ),
    );
    add_axiom(env, "iff_of_true", vec![], iff_of_true_ty)?;
    let iff_of_false_ty = pi(
        BinderInfo::Implicit,
        "a",
        prop(),
        pi(
            BinderInfo::Implicit,
            "b",
            prop(),
            pi(
                BinderInfo::Default,
                "ha",
                app(cst("Not"), bvar(1)),
                pi(
                    BinderInfo::Default,
                    "hb",
                    app(cst("Not"), bvar(1)),
                    app2(cst("Iff"), bvar(3), bvar(2)),
                ),
            ),
        ),
    );
    add_axiom(env, "iff_of_false", vec![], iff_of_false_ty)?;
    let and_iff_left_ty = pi(
        BinderInfo::Implicit,
        "a",
        prop(),
        pi(
            BinderInfo::Implicit,
            "b",
            prop(),
            pi(
                BinderInfo::Default,
                "ha",
                bvar(1),
                app2(cst("Iff"), app2(cst("And"), bvar(2), bvar(1)), bvar(1)),
            ),
        ),
    );
    add_axiom(env, "and_iff_left", vec![], and_iff_left_ty)?;
    let and_iff_right_ty = pi(
        BinderInfo::Implicit,
        "a",
        prop(),
        pi(
            BinderInfo::Implicit,
            "b",
            prop(),
            pi(
                BinderInfo::Default,
                "hb",
                bvar(0),
                app2(cst("Iff"), app2(cst("And"), bvar(2), bvar(1)), bvar(2)),
            ),
        ),
    );
    add_axiom(env, "and_iff_right", vec![], and_iff_right_ty)?;
    let or_iff_left_ty = pi(
        BinderInfo::Implicit,
        "a",
        prop(),
        pi(
            BinderInfo::Implicit,
            "b",
            prop(),
            pi(
                BinderInfo::Default,
                "hnb",
                app(cst("Not"), bvar(0)),
                app2(cst("Iff"), app2(cst("Or"), bvar(2), bvar(1)), bvar(2)),
            ),
        ),
    );
    add_axiom(env, "or_iff_left", vec![], or_iff_left_ty)?;
    let or_iff_right_ty = pi(
        BinderInfo::Implicit,
        "a",
        prop(),
        pi(
            BinderInfo::Implicit,
            "b",
            prop(),
            pi(
                BinderInfo::Default,
                "hna",
                app(cst("Not"), bvar(1)),
                app2(cst("Iff"), app2(cst("Or"), bvar(2), bvar(1)), bvar(1)),
            ),
        ),
    );
    add_axiom(env, "or_iff_right", vec![], or_iff_right_ty)?;
    Ok(())
}
/// Create the True proposition.
#[allow(dead_code)]
pub fn mk_true() -> Expr {
    cst("True")
}
/// Create the False proposition.
#[allow(dead_code)]
pub fn mk_false() -> Expr {
    cst("False")
}
/// Create Not p.
#[allow(dead_code)]
pub fn mk_not(p: Expr) -> Expr {
    app(cst("Not"), p)
}
/// Create And a b.
#[allow(dead_code)]
pub fn mk_and(a: Expr, b: Expr) -> Expr {
    app2(cst("And"), a, b)
}
/// Create Or a b.
#[allow(dead_code)]
pub fn mk_or(a: Expr, b: Expr) -> Expr {
    app2(cst("Or"), a, b)
}
/// Create Iff a b.
#[allow(dead_code)]
pub fn mk_iff(a: Expr, b: Expr) -> Expr {
    app2(cst("Iff"), a, b)
}
/// Create Exists alpha_ty pred.
#[allow(dead_code)]
pub fn mk_exists(alpha_ty: Expr, pred: Expr) -> Expr {
    app2(cst("Exists"), alpha_ty, pred)
}
/// Create Eq ty a b (with implicit type argument).
#[allow(dead_code)]
pub fn mk_eq(ty: Expr, a: Expr, b: Expr) -> Expr {
    app3(cst("Eq"), ty, a, b)
}
/// Create HEq alpha a beta b.
#[allow(dead_code)]
pub fn mk_heq(alpha: Expr, a: Expr, beta: Expr, b: Expr) -> Expr {
    app4(cst("HEq"), alpha, a, beta, b)
}
/// Create an implication a → b (as Pi Default "_" a b).
#[allow(dead_code)]
pub fn mk_implies(a: Expr, b: Expr) -> Expr {
    pi(BinderInfo::Default, "_", a, b)
}
/// Create a universal quantification ∀ (name : ty), body.
#[allow(dead_code)]
pub fn mk_forall(name: &str, ty: Expr, body: Expr) -> Expr {
    pi(BinderInfo::Default, name, ty, body)
}
/// Create Prop (Sort 0).
#[allow(dead_code)]
pub fn mk_prop() -> Expr {
    prop()
}
/// Create Type u (Sort (succ u)).
#[allow(dead_code)]
pub fn mk_type(u: Level) -> Expr {
    Expr::Sort(Level::succ(u))
}
/// Create True.intro.
#[allow(dead_code)]
pub fn true_intro() -> Expr {
    cst("True.intro")
}
/// Create False.elim target_ty proof.
#[allow(dead_code)]
pub fn false_elim(target_ty: Expr, proof: Expr) -> Expr {
    app2(cst("False.elim"), target_ty, proof)
}
/// Create And.intro ha hb.
#[allow(dead_code)]
pub fn and_intro(ha: Expr, hb: Expr) -> Expr {
    app2(cst("And.intro"), ha, hb)
}
/// Create And.left hab.
#[allow(dead_code)]
pub fn and_left(hab: Expr) -> Expr {
    app(cst("And.left"), hab)
}
/// Create And.right hab.
#[allow(dead_code)]
pub fn and_right(hab: Expr) -> Expr {
    app(cst("And.right"), hab)
}
/// Create Or.inl ha.
#[allow(dead_code)]
pub fn or_inl(ha: Expr) -> Expr {
    app(cst("Or.inl"), ha)
}
/// Create Or.inr hb.
#[allow(dead_code)]
pub fn or_inr(hb: Expr) -> Expr {
    app(cst("Or.inr"), hb)
}
/// Create Eq.refl a.
#[allow(dead_code)]
pub fn eq_refl(a: Expr) -> Expr {
    app(cst("Eq.refl"), a)
}
/// Create Eq.symm h.
#[allow(dead_code)]
pub fn eq_symm(h: Expr) -> Expr {
    app(cst("Eq.symm"), h)
}
/// Create Eq.trans h1 h2.
#[allow(dead_code)]
pub fn eq_trans(h1: Expr, h2: Expr) -> Expr {
    app2(cst("Eq.trans"), h1, h2)
}
/// Create Exists.intro witness proof.
#[allow(dead_code)]
pub fn exists_intro(witness: Expr, proof: Expr) -> Expr {
    app2(cst("Exists.intro"), witness, proof)
}
