//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

#[allow(dead_code)]
pub fn app(f: Expr, a: Expr) -> Expr {
    Expr::App(Box::new(f), Box::new(a))
}
#[allow(dead_code)]
pub fn app2(f: Expr, a: Expr, b: Expr) -> Expr {
    app(app(f, a), b)
}
#[allow(dead_code)]
pub fn app3(f: Expr, a: Expr, b: Expr, c: Expr) -> Expr {
    app(app2(f, a, b), c)
}
#[allow(dead_code)]
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
pub(super) fn cst(s: &str) -> Expr {
    Expr::Const(Name::str(s), vec![])
}
#[allow(dead_code)]
pub fn cst_u(s: &str, levels: Vec<Level>) -> Expr {
    Expr::Const(Name::str(s), levels)
}
pub fn prop() -> Expr {
    Expr::Sort(Level::zero())
}
pub fn type0() -> Expr {
    Expr::Sort(Level::succ(Level::zero()))
}
pub fn type1() -> Expr {
    Expr::Sort(Level::succ(Level::succ(Level::zero())))
}
pub fn sort_u() -> Expr {
    Expr::Sort(Level::Param(Name::str("u")))
}
#[allow(dead_code)]
pub fn sort_v() -> Expr {
    Expr::Sort(Level::Param(Name::str("v")))
}
pub fn bvar(n: u32) -> Expr {
    Expr::BVar(n)
}
pub fn arrow(a: Expr, b: Expr) -> Expr {
    Expr::Pi(
        BinderInfo::Default,
        Name::Anonymous,
        Box::new(a),
        Box::new(b),
    )
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
/// Build `Eq @{} T a b`.
#[allow(dead_code)]
pub(super) fn mk_eq_expr(ty: Expr, lhs: Expr, rhs: Expr) -> Expr {
    app3(cst("Eq"), ty, lhs, rhs)
}
/// Build `Iff a b`.
#[allow(dead_code)]
pub fn mk_iff(a: Expr, b: Expr) -> Expr {
    app2(cst("Iff"), a, b)
}
/// Build the quotient types environment.
#[allow(clippy::too_many_lines)]
pub fn build_quotient_types_env(env: &mut Environment) -> Result<(), String> {
    add_axiom(env, "Setoid", vec![], arrow(type0(), type1()))?;
    let setoid_mk_ty = pi(
        BinderInfo::Implicit,
        "α",
        type0(),
        pi(
            BinderInfo::Default,
            "r",
            arrow(bvar(0), arrow(bvar(1), prop())),
            app(cst("Setoid"), bvar(1)),
        ),
    );
    add_axiom(env, "Setoid.mk", vec![], setoid_mk_ty)?;
    let setoid_r_ty = pi(
        BinderInfo::Implicit,
        "α",
        type0(),
        pi(
            BinderInfo::Default,
            "s",
            app(cst("Setoid"), bvar(0)),
            pi(
                BinderInfo::Default,
                "a",
                bvar(1),
                pi(BinderInfo::Default, "b", bvar(2), prop()),
            ),
        ),
    );
    add_axiom(env, "Setoid.r", vec![], setoid_r_ty)?;
    let setoid_refl_ty = pi(
        BinderInfo::Implicit,
        "α",
        type0(),
        pi(
            BinderInfo::InstImplicit,
            "s",
            app(cst("Setoid"), bvar(0)),
            pi(
                BinderInfo::Default,
                "a",
                bvar(1),
                app2(cst("Setoid.r"), bvar(1), bvar(0)),
            ),
        ),
    );
    add_axiom(env, "Setoid.refl", vec![], setoid_refl_ty)?;
    let setoid_symm_ty = pi(
        BinderInfo::Implicit,
        "α",
        type0(),
        pi(
            BinderInfo::InstImplicit,
            "s",
            app(cst("Setoid"), bvar(0)),
            pi(
                BinderInfo::Default,
                "a",
                bvar(1),
                pi(
                    BinderInfo::Default,
                    "b",
                    bvar(2),
                    pi(
                        BinderInfo::Default,
                        "h",
                        app3(cst("Setoid.r"), bvar(2), bvar(1), bvar(0)),
                        app3(cst("Setoid.r"), bvar(3), bvar(1), bvar(2)),
                    ),
                ),
            ),
        ),
    );
    add_axiom(env, "Setoid.symm", vec![], setoid_symm_ty)?;
    let setoid_trans_ty = pi(
        BinderInfo::Implicit,
        "α",
        type0(),
        pi(
            BinderInfo::InstImplicit,
            "s",
            app(cst("Setoid"), bvar(0)),
            pi(
                BinderInfo::Default,
                "a",
                bvar(1),
                pi(
                    BinderInfo::Default,
                    "b",
                    bvar(2),
                    pi(
                        BinderInfo::Default,
                        "c",
                        bvar(3),
                        pi(
                            BinderInfo::Default,
                            "h1",
                            app3(cst("Setoid.r"), bvar(3), bvar(2), bvar(1)),
                            pi(
                                BinderInfo::Default,
                                "h2",
                                app3(cst("Setoid.r"), bvar(4), bvar(2), bvar(1)),
                                app3(cst("Setoid.r"), bvar(5), bvar(4), bvar(2)),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    );
    add_axiom(env, "Setoid.trans", vec![], setoid_trans_ty)?;
    let quotient_ty = pi(
        BinderInfo::Implicit,
        "α",
        type0(),
        pi(
            BinderInfo::InstImplicit,
            "s",
            app(cst("Setoid"), bvar(0)),
            type0(),
        ),
    );
    add_axiom(env, "Quotient", vec![], quotient_ty)?;
    let quotient_mk_ty = pi(
        BinderInfo::Implicit,
        "α",
        type0(),
        pi(
            BinderInfo::InstImplicit,
            "s",
            app(cst("Setoid"), bvar(0)),
            pi(BinderInfo::Default, "a", bvar(1), cst("Quotient")),
        ),
    );
    add_axiom(env, "Quotient.mk", vec![], quotient_mk_ty)?;
    let quotient_lift_ty = pi(
        BinderInfo::Implicit,
        "α",
        type0(),
        pi(
            BinderInfo::Implicit,
            "β",
            sort_u(),
            pi(
                BinderInfo::InstImplicit,
                "s",
                app(cst("Setoid"), bvar(1)),
                pi(
                    BinderInfo::Default,
                    "f",
                    arrow(bvar(2), bvar(2)),
                    pi(
                        BinderInfo::Default,
                        "h",
                        pi(
                            BinderInfo::Default,
                            "a",
                            bvar(3),
                            pi(
                                BinderInfo::Default,
                                "b",
                                bvar(4),
                                arrow(
                                    app3(cst("Setoid.r"), bvar(4), bvar(1), bvar(0)),
                                    mk_eq_expr(
                                        bvar(5),
                                        app(bvar(3), bvar(1)),
                                        app(bvar(3), bvar(0)),
                                    ),
                                ),
                            ),
                        ),
                        pi(BinderInfo::Default, "q", cst("Quotient"), bvar(4)),
                    ),
                ),
            ),
        ),
    );
    add_axiom(env, "Quotient.lift", vec![Name::str("u")], quotient_lift_ty)?;
    let quotient_ind_ty = pi(
        BinderInfo::Implicit,
        "α",
        type0(),
        pi(
            BinderInfo::InstImplicit,
            "s",
            app(cst("Setoid"), bvar(0)),
            pi(
                BinderInfo::Implicit,
                "p",
                arrow(cst("Quotient"), prop()),
                pi(
                    BinderInfo::Default,
                    "h",
                    pi(
                        BinderInfo::Default,
                        "a",
                        bvar(2),
                        app(bvar(1), app(cst("Quotient.mk"), bvar(0))),
                    ),
                    pi(
                        BinderInfo::Default,
                        "q",
                        cst("Quotient"),
                        app(bvar(2), bvar(0)),
                    ),
                ),
            ),
        ),
    );
    add_axiom(env, "Quotient.ind", vec![], quotient_ind_ty)?;
    let quotient_sound_ty = pi(
        BinderInfo::Implicit,
        "α",
        type0(),
        pi(
            BinderInfo::InstImplicit,
            "s",
            app(cst("Setoid"), bvar(0)),
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
                        "h",
                        app3(cst("Setoid.r"), bvar(2), bvar(1), bvar(0)),
                        mk_eq_expr(
                            cst("Quotient"),
                            app(cst("Quotient.mk"), bvar(2)),
                            app(cst("Quotient.mk"), bvar(1)),
                        ),
                    ),
                ),
            ),
        ),
    );
    add_axiom(env, "Quotient.sound", vec![], quotient_sound_ty)?;
    let quotient_exact_ty = pi(
        BinderInfo::Implicit,
        "α",
        type0(),
        pi(
            BinderInfo::InstImplicit,
            "s",
            app(cst("Setoid"), bvar(0)),
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
                        "h",
                        mk_eq_expr(
                            cst("Quotient"),
                            app(cst("Quotient.mk"), bvar(1)),
                            app(cst("Quotient.mk"), bvar(0)),
                        ),
                        app3(cst("Setoid.r"), bvar(3), bvar(2), bvar(1)),
                    ),
                ),
            ),
        ),
    );
    add_axiom(env, "Quotient.exact", vec![], quotient_exact_ty)?;
    let quotient_map_ty = pi(
        BinderInfo::Implicit,
        "α",
        type0(),
        pi(
            BinderInfo::Implicit,
            "β",
            type0(),
            pi(
                BinderInfo::InstImplicit,
                "sa",
                app(cst("Setoid"), bvar(1)),
                pi(
                    BinderInfo::InstImplicit,
                    "sb",
                    app(cst("Setoid"), bvar(1)),
                    pi(
                        BinderInfo::Default,
                        "f",
                        arrow(bvar(3), bvar(3)),
                        pi(
                            BinderInfo::Default,
                            "h",
                            pi(
                                BinderInfo::Default,
                                "a1",
                                bvar(4),
                                pi(
                                    BinderInfo::Default,
                                    "a2",
                                    bvar(5),
                                    arrow(
                                        app3(cst("Setoid.r"), bvar(4), bvar(1), bvar(0)),
                                        app3(
                                            cst("Setoid.r"),
                                            bvar(4),
                                            app(bvar(3), bvar(1)),
                                            app(bvar(3), bvar(0)),
                                        ),
                                    ),
                                ),
                            ),
                            pi(BinderInfo::Default, "q", cst("Quotient"), cst("Quotient")),
                        ),
                    ),
                ),
            ),
        ),
    );
    add_axiom(env, "Quotient.map", vec![], quotient_map_ty)?;
    let quotient_map2_ty = pi(
        BinderInfo::Implicit,
        "α",
        type0(),
        pi(
            BinderInfo::Implicit,
            "β",
            type0(),
            pi(
                BinderInfo::Implicit,
                "γ",
                type0(),
                pi(
                    BinderInfo::InstImplicit,
                    "sa",
                    app(cst("Setoid"), bvar(2)),
                    pi(
                        BinderInfo::InstImplicit,
                        "sb",
                        app(cst("Setoid"), bvar(2)),
                        pi(
                            BinderInfo::InstImplicit,
                            "sc",
                            app(cst("Setoid"), bvar(2)),
                            pi(
                                BinderInfo::Default,
                                "f",
                                arrow(bvar(5), arrow(bvar(5), bvar(5))),
                                pi(
                                    BinderInfo::Default,
                                    "h",
                                    pi(
                                        BinderInfo::Default,
                                        "a1",
                                        bvar(6),
                                        pi(
                                            BinderInfo::Default,
                                            "a2",
                                            bvar(7),
                                            pi(
                                                BinderInfo::Default,
                                                "b1",
                                                bvar(7),
                                                pi(
                                                    BinderInfo::Default,
                                                    "b2",
                                                    bvar(8),
                                                    arrow(
                                                        app3(
                                                            cst("Setoid.r"),
                                                            bvar(7),
                                                            bvar(3),
                                                            bvar(2),
                                                        ),
                                                        arrow(
                                                            app3(
                                                                cst("Setoid.r"),
                                                                bvar(6),
                                                                bvar(1),
                                                                bvar(0),
                                                            ),
                                                            app3(
                                                                cst("Setoid.r"),
                                                                bvar(5),
                                                                app(app(bvar(4), bvar(3)), bvar(1)),
                                                                app(app(bvar(4), bvar(2)), bvar(0)),
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        ),
                                    ),
                                    pi(
                                        BinderInfo::Default,
                                        "q1",
                                        cst("Quotient"),
                                        pi(
                                            BinderInfo::Default,
                                            "q2",
                                            cst("Quotient"),
                                            cst("Quotient"),
                                        ),
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    );
    add_axiom(env, "Quotient.map2", vec![], quotient_map2_ty)?;
    let quotient_lift_on_ty = pi(
        BinderInfo::Implicit,
        "α",
        type0(),
        pi(
            BinderInfo::Implicit,
            "β",
            sort_u(),
            pi(
                BinderInfo::InstImplicit,
                "s",
                app(cst("Setoid"), bvar(1)),
                pi(
                    BinderInfo::Default,
                    "q",
                    cst("Quotient"),
                    pi(
                        BinderInfo::Default,
                        "f",
                        arrow(bvar(3), bvar(3)),
                        pi(
                            BinderInfo::Default,
                            "h",
                            pi(
                                BinderInfo::Default,
                                "a",
                                bvar(4),
                                pi(
                                    BinderInfo::Default,
                                    "b",
                                    bvar(5),
                                    arrow(
                                        app3(cst("Setoid.r"), bvar(5), bvar(1), bvar(0)),
                                        mk_eq_expr(
                                            bvar(6),
                                            app(bvar(3), bvar(1)),
                                            app(bvar(3), bvar(0)),
                                        ),
                                    ),
                                ),
                            ),
                            bvar(4),
                        ),
                    ),
                ),
            ),
        ),
    );
    add_axiom(
        env,
        "Quotient.liftOn",
        vec![Name::str("u")],
        quotient_lift_on_ty,
    )?;
    let quotient_rec_ty = pi(
        BinderInfo::Implicit,
        "α",
        type0(),
        pi(
            BinderInfo::InstImplicit,
            "s",
            app(cst("Setoid"), bvar(0)),
            pi(
                BinderInfo::Implicit,
                "C",
                arrow(cst("Quotient"), type0()),
                pi(
                    BinderInfo::Default,
                    "f",
                    pi(
                        BinderInfo::Default,
                        "a",
                        bvar(2),
                        app(bvar(1), app(cst("Quotient.mk"), bvar(0))),
                    ),
                    pi(
                        BinderInfo::Default,
                        "h",
                        pi(
                            BinderInfo::Default,
                            "a",
                            bvar(3),
                            pi(
                                BinderInfo::Default,
                                "b",
                                bvar(4),
                                arrow(
                                    app3(cst("Setoid.r"), bvar(4), bvar(1), bvar(0)),
                                    mk_eq_expr(
                                        app(bvar(3), app(cst("Quotient.mk"), bvar(1))),
                                        app(bvar(2), bvar(1)),
                                        app(bvar(2), bvar(0)),
                                    ),
                                ),
                            ),
                        ),
                        pi(
                            BinderInfo::Default,
                            "q",
                            cst("Quotient"),
                            app(bvar(3), bvar(0)),
                        ),
                    ),
                ),
            ),
        ),
    );
    add_axiom(env, "Quotient.rec", vec![], quotient_rec_ty)?;
    add_axiom(env, "HasEquiv", vec![], arrow(type0(), type1()))?;
    let has_equiv_equiv_ty = pi(
        BinderInfo::Implicit,
        "α",
        type0(),
        pi(
            BinderInfo::InstImplicit,
            "_inst",
            app(cst("HasEquiv"), bvar(0)),
            pi(
                BinderInfo::Default,
                "a",
                bvar(1),
                pi(BinderInfo::Default, "b", bvar(2), prop()),
            ),
        ),
    );
    add_axiom(env, "HasEquiv.equiv", vec![], has_equiv_equiv_ty)?;
    let inst_has_equiv_ty = pi(
        BinderInfo::Implicit,
        "α",
        type0(),
        pi(
            BinderInfo::InstImplicit,
            "_inst",
            app(cst("Setoid"), bvar(0)),
            app(cst("HasEquiv"), bvar(1)),
        ),
    );
    add_axiom(env, "instHasEquivOfSetoid", vec![], inst_has_equiv_ty)?;
    let quot_ty = pi(
        BinderInfo::Implicit,
        "α",
        type0(),
        pi(
            BinderInfo::Default,
            "r",
            arrow(bvar(0), arrow(bvar(1), prop())),
            type0(),
        ),
    );
    add_axiom(env, "Quot", vec![], quot_ty)?;
    let quot_mk_ty = pi(
        BinderInfo::Implicit,
        "α",
        type0(),
        pi(
            BinderInfo::Implicit,
            "r",
            arrow(bvar(0), arrow(bvar(1), prop())),
            pi(BinderInfo::Default, "a", bvar(1), app(cst("Quot"), bvar(1))),
        ),
    );
    add_axiom(env, "Quot.mk", vec![], quot_mk_ty)?;
    let quot_lift_ty = pi(
        BinderInfo::Implicit,
        "α",
        type0(),
        pi(
            BinderInfo::Implicit,
            "r",
            arrow(bvar(0), arrow(bvar(1), prop())),
            pi(
                BinderInfo::Implicit,
                "β",
                sort_u(),
                pi(
                    BinderInfo::Default,
                    "f",
                    arrow(bvar(2), bvar(1)),
                    pi(
                        BinderInfo::Default,
                        "h",
                        pi(
                            BinderInfo::Default,
                            "a",
                            bvar(3),
                            pi(
                                BinderInfo::Default,
                                "b",
                                bvar(4),
                                arrow(
                                    app2(bvar(4), bvar(1), bvar(0)),
                                    mk_eq_expr(
                                        bvar(4),
                                        app(bvar(3), bvar(1)),
                                        app(bvar(3), bvar(0)),
                                    ),
                                ),
                            ),
                        ),
                        pi(BinderInfo::Default, "q", app(cst("Quot"), bvar(3)), bvar(3)),
                    ),
                ),
            ),
        ),
    );
    add_axiom(env, "Quot.lift", vec![Name::str("u")], quot_lift_ty)?;
    let quot_ind_ty = pi(
        BinderInfo::Implicit,
        "α",
        type0(),
        pi(
            BinderInfo::Implicit,
            "r",
            arrow(bvar(0), arrow(bvar(1), prop())),
            pi(
                BinderInfo::Implicit,
                "p",
                arrow(app(cst("Quot"), bvar(0)), prop()),
                pi(
                    BinderInfo::Default,
                    "h",
                    pi(
                        BinderInfo::Default,
                        "a",
                        bvar(2),
                        app(bvar(1), app(cst("Quot.mk"), bvar(0))),
                    ),
                    pi(
                        BinderInfo::Default,
                        "q",
                        app(cst("Quot"), bvar(2)),
                        app(bvar(2), bvar(0)),
                    ),
                ),
            ),
        ),
    );
    add_axiom(env, "Quot.ind", vec![], quot_ind_ty)?;
    let quot_sound_ty = pi(
        BinderInfo::Implicit,
        "α",
        type0(),
        pi(
            BinderInfo::Implicit,
            "r",
            arrow(bvar(0), arrow(bvar(1), prop())),
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
                        "h",
                        app2(bvar(2), bvar(1), bvar(0)),
                        mk_eq_expr(
                            app(cst("Quot"), bvar(3)),
                            app(cst("Quot.mk"), bvar(2)),
                            app(cst("Quot.mk"), bvar(1)),
                        ),
                    ),
                ),
            ),
        ),
    );
    add_axiom(env, "Quot.sound", vec![], quot_sound_ty)?;
    let lift_mk_ty = pi(
        BinderInfo::Implicit,
        "α",
        type0(),
        pi(
            BinderInfo::Implicit,
            "β",
            sort_u(),
            pi(
                BinderInfo::InstImplicit,
                "s",
                app(cst("Setoid"), bvar(1)),
                pi(
                    BinderInfo::Default,
                    "f",
                    arrow(bvar(2), bvar(2)),
                    pi(
                        BinderInfo::Default,
                        "h",
                        pi(
                            BinderInfo::Default,
                            "a",
                            bvar(3),
                            pi(
                                BinderInfo::Default,
                                "b",
                                bvar(4),
                                arrow(
                                    app3(cst("Setoid.r"), bvar(3), bvar(1), bvar(0)),
                                    mk_eq_expr(
                                        bvar(4),
                                        app(bvar(2), bvar(1)),
                                        app(bvar(2), bvar(0)),
                                    ),
                                ),
                            ),
                        ),
                        pi(
                            BinderInfo::Default,
                            "a",
                            bvar(4),
                            mk_eq_expr(
                                bvar(4),
                                app3(
                                    cst("Quotient.lift"),
                                    bvar(2),
                                    bvar(1),
                                    app(cst("Quotient.mk"), bvar(0)),
                                ),
                                app(bvar(2), bvar(0)),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    );
    add_axiom(env, "Quotient.lift_mk", vec![Name::str("u")], lift_mk_ty)?;
    let map_mk_ty = pi(
        BinderInfo::Implicit,
        "α",
        type0(),
        pi(
            BinderInfo::Implicit,
            "β",
            type0(),
            pi(
                BinderInfo::InstImplicit,
                "sa",
                app(cst("Setoid"), bvar(1)),
                pi(
                    BinderInfo::InstImplicit,
                    "sb",
                    app(cst("Setoid"), bvar(1)),
                    pi(
                        BinderInfo::Default,
                        "f",
                        arrow(bvar(3), bvar(3)),
                        pi(
                            BinderInfo::Default,
                            "h",
                            pi(
                                BinderInfo::Default,
                                "a",
                                bvar(4),
                                pi(
                                    BinderInfo::Default,
                                    "b",
                                    bvar(5),
                                    arrow(
                                        app3(cst("Setoid.r"), bvar(4), bvar(1), bvar(0)),
                                        app3(
                                            cst("Setoid.r"),
                                            bvar(3),
                                            app(bvar(2), bvar(1)),
                                            app(bvar(2), bvar(0)),
                                        ),
                                    ),
                                ),
                            ),
                            pi(
                                BinderInfo::Default,
                                "a",
                                bvar(5),
                                mk_eq_expr(
                                    cst("Quotient"),
                                    app3(
                                        cst("Quotient.map"),
                                        bvar(2),
                                        bvar(1),
                                        app(cst("Quotient.mk"), bvar(0)),
                                    ),
                                    app(cst("Quotient.mk"), app(bvar(2), bvar(0))),
                                ),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    );
    add_axiom(env, "Quotient.map_mk", vec![], map_mk_ty)?;
    let ind_mk_ty = pi(
        BinderInfo::Implicit,
        "α",
        type0(),
        pi(
            BinderInfo::InstImplicit,
            "s",
            app(cst("Setoid"), bvar(0)),
            pi(
                BinderInfo::Implicit,
                "p",
                arrow(cst("Quotient"), prop()),
                pi(
                    BinderInfo::Default,
                    "h",
                    pi(
                        BinderInfo::Default,
                        "a",
                        bvar(2),
                        app(bvar(1), app(cst("Quotient.mk"), bvar(0))),
                    ),
                    pi(
                        BinderInfo::Default,
                        "a",
                        bvar(3),
                        mk_eq_expr(
                            app(bvar(2), app(cst("Quotient.mk"), bvar(0))),
                            app2(
                                cst("Quotient.ind"),
                                bvar(1),
                                app(cst("Quotient.mk"), bvar(0)),
                            ),
                            app(bvar(1), bvar(0)),
                        ),
                    ),
                ),
            ),
        ),
    );
    add_axiom(env, "Quotient.ind_mk", vec![], ind_mk_ty)?;
    let sound_iff_ty = pi(
        BinderInfo::Implicit,
        "α",
        type0(),
        pi(
            BinderInfo::InstImplicit,
            "s",
            app(cst("Setoid"), bvar(0)),
            pi(
                BinderInfo::Implicit,
                "a",
                bvar(1),
                pi(
                    BinderInfo::Implicit,
                    "b",
                    bvar(2),
                    mk_iff(
                        mk_eq_expr(
                            cst("Quotient"),
                            app(cst("Quotient.mk"), bvar(1)),
                            app(cst("Quotient.mk"), bvar(0)),
                        ),
                        app3(cst("Setoid.r"), bvar(2), bvar(1), bvar(0)),
                    ),
                ),
            ),
        ),
    );
    add_axiom(env, "Quotient.sound_iff", vec![], sound_iff_ty)?;
    let quot_lift_mk_ty = pi(
        BinderInfo::Implicit,
        "α",
        type0(),
        pi(
            BinderInfo::Implicit,
            "r",
            arrow(bvar(0), arrow(bvar(1), prop())),
            pi(
                BinderInfo::Implicit,
                "β",
                sort_u(),
                pi(
                    BinderInfo::Default,
                    "f",
                    arrow(bvar(2), bvar(1)),
                    pi(
                        BinderInfo::Default,
                        "h",
                        pi(
                            BinderInfo::Default,
                            "a",
                            bvar(3),
                            pi(
                                BinderInfo::Default,
                                "b",
                                bvar(4),
                                arrow(
                                    app(app(bvar(4), bvar(1)), bvar(0)),
                                    mk_eq_expr(
                                        bvar(3),
                                        app(bvar(2), bvar(1)),
                                        app(bvar(2), bvar(0)),
                                    ),
                                ),
                            ),
                        ),
                        pi(
                            BinderInfo::Default,
                            "a",
                            bvar(4),
                            mk_eq_expr(
                                bvar(3),
                                app3(
                                    cst("Quot.lift"),
                                    bvar(2),
                                    bvar(1),
                                    app(cst("Quot.mk"), bvar(0)),
                                ),
                                app(bvar(2), bvar(0)),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    );
    add_axiom(env, "Quot.lift_mk", vec![Name::str("u")], quot_lift_mk_ty)?;
    Ok(())
}
/// Build `Setoid α` type expression.
#[allow(dead_code)]
pub fn mk_setoid(alpha: Expr) -> Expr {
    app(cst("Setoid"), alpha)
}
/// Build `Quotient` type expression (with implicit setoid).
#[allow(dead_code)]
pub fn mk_quotient() -> Expr {
    cst("Quotient")
}
/// Build `Quotient.mk a` expression.
#[allow(dead_code)]
pub fn mk_quotient_mk(val: Expr) -> Expr {
    app(cst("Quotient.mk"), val)
}
/// Build `Quotient.lift f h q` expression.
#[allow(dead_code)]
pub fn mk_quotient_lift(f: Expr, proof: Expr, q: Expr) -> Expr {
    app3(cst("Quotient.lift"), f, proof, q)
}
/// Build `Quotient.map f h q` expression.
#[allow(dead_code)]
pub fn mk_quotient_map(f: Expr, proof: Expr, q: Expr) -> Expr {
    app3(cst("Quotient.map"), f, proof, q)
}
/// Build `Quotient.ind h q` expression.
#[allow(dead_code)]
pub fn mk_quotient_ind(h: Expr, q: Expr) -> Expr {
    app2(cst("Quotient.ind"), h, q)
}
/// Build `Quotient.sound h` expression.
#[allow(dead_code)]
pub fn mk_quotient_sound(h: Expr) -> Expr {
    app(cst("Quotient.sound"), h)
}
/// Build `Setoid.r s a b` expression.
#[allow(dead_code)]
pub fn mk_setoid_r(s: Expr, a: Expr, b: Expr) -> Expr {
    app3(cst("Setoid.r"), s, a, b)
}
/// Build `Quot r` type expression.
#[allow(dead_code)]
pub fn mk_quot(rel: Expr) -> Expr {
    app(cst("Quot"), rel)
}
/// Build `Quot.mk a` expression.
#[allow(dead_code)]
pub fn mk_quot_mk(val: Expr) -> Expr {
    app(cst("Quot.mk"), val)
}
/// Build `Quot.lift f h q` expression.
#[allow(dead_code)]
pub fn mk_quot_lift(f: Expr, proof: Expr, q: Expr) -> Expr {
    app3(cst("Quot.lift"), f, proof, q)
}
/// Build `Quot.sound h` expression.
#[allow(dead_code)]
pub fn mk_quot_sound(h: Expr) -> Expr {
    app(cst("Quot.sound"), h)
}
/// Build `HasEquiv α` type expression.
#[allow(dead_code)]
pub fn mk_has_equiv(alpha: Expr) -> Expr {
    app(cst("HasEquiv"), alpha)
}
/// Quotient.elimType: the type of the eliminator for quotient types.
/// QuotientElimType : (α : Type) → \[Setoid α\] → (C : Quotient → Type) → Type
pub fn qt_ext_quotient_elim_type_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "α",
        type0(),
        pi(
            BinderInfo::InstImplicit,
            "s",
            app(cst("Setoid"), bvar(0)),
            pi(
                BinderInfo::Default,
                "C",
                arrow(cst("Quotient"), type0()),
                type0(),
            ),
        ),
    )
}
/// Quotient.inductionOn: induction principle stated in "on" form.
/// InductionOn : {α : Type} → \[Setoid α\] → (q : Quotient) →
///   (motive : Quotient → Prop) → (∀ a, motive (Quotient.mk a)) → motive q
pub fn qt_ext_quotient_induction_on_ty() -> Expr {
    pi(
        BinderInfo::Implicit,
        "α",
        type0(),
        pi(
            BinderInfo::InstImplicit,
            "s",
            app(cst("Setoid"), bvar(0)),
            pi(
                BinderInfo::Default,
                "q",
                cst("Quotient"),
                pi(
                    BinderInfo::Default,
                    "motive",
                    arrow(cst("Quotient"), prop()),
                    pi(
                        BinderInfo::Default,
                        "ih",
                        pi(
                            BinderInfo::Default,
                            "a",
                            bvar(3),
                            app(bvar(1), app(cst("Quotient.mk"), bvar(0))),
                        ),
                        app(bvar(1), bvar(2)),
                    ),
                ),
            ),
        ),
    )
}
/// Quotient.surjective: every element of Quotient is of the form Quotient.mk a.
/// QuotientSurjective : {α : Type} → \[Setoid α\] → ∀ q : Quotient, ∃ a : α, Quotient.mk a = q
pub fn qt_ext_quotient_surjective_ty() -> Expr {
    pi(
        BinderInfo::Implicit,
        "α",
        type0(),
        pi(
            BinderInfo::InstImplicit,
            "s",
            app(cst("Setoid"), bvar(0)),
            pi(
                BinderInfo::Default,
                "q",
                cst("Quotient"),
                app(cst("ExistsSatisfying"), bvar(1)),
            ),
        ),
    )
}
/// Quotient.functor: functorial action on quotient types.
/// QuotientFunctor : (f : α → β) → (α/~) → (β/~')
pub fn qt_ext_quotient_functor_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "α",
        type0(),
        pi(
            BinderInfo::Default,
            "β",
            type0(),
            arrow(
                arrow(bvar(1), bvar(1)),
                arrow(cst("Quotient"), cst("Quotient")),
            ),
        ),
    )
}
/// Quotient.monad_pure: monad unit for the quotient monad.
/// QuotientPure : (α : Type) → \[Setoid α\] → α → Quotient
pub fn qt_ext_quotient_monad_pure_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "α",
        type0(),
        pi(
            BinderInfo::InstImplicit,
            "s",
            app(cst("Setoid"), bvar(0)),
            arrow(bvar(1), cst("Quotient")),
        ),
    )
}
/// Quotient.monad_bind: monadic bind for the quotient monad.
/// QuotientBind : (α β : Type) → \[Setoid α\] → \[Setoid β\] →
///   Quotient → (α → Quotient) → Quotient
pub fn qt_ext_quotient_monad_bind_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "α",
        type0(),
        pi(
            BinderInfo::Default,
            "β",
            type0(),
            pi(
                BinderInfo::InstImplicit,
                "sa",
                app(cst("Setoid"), bvar(1)),
                pi(
                    BinderInfo::InstImplicit,
                    "sb",
                    app(cst("Setoid"), bvar(1)),
                    arrow(
                        cst("Quotient"),
                        arrow(arrow(bvar(3), cst("Quotient")), cst("Quotient")),
                    ),
                ),
            ),
        ),
    )
}
/// SetoidModel: the setoid model interprets type theory in Setoids.
/// SetoidModel : Type → Type 1
pub fn qt_ext_setoid_model_ty() -> Expr {
    arrow(type0(), type1())
}
/// EffectiveQuotient: a quotient is effective if the kernel pair equals the relation.
/// EffectiveQuotient : (α : Type) → \[Setoid α\] → Prop
pub fn qt_ext_effective_quotient_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "α",
        type0(),
        pi(
            BinderInfo::InstImplicit,
            "s",
            app(cst("Setoid"), bvar(0)),
            prop(),
        ),
    )
}
/// SetoidCategory: the category of setoids and setoid-respecting functions.
/// SetoidCategory : Type 1
pub fn qt_ext_setoid_category_ty() -> Expr {
    type1()
}
/// SetoidFunctor: a functor between setoid categories.
/// SetoidFunctor : SetoidCategory → SetoidCategory → Type 1
pub fn qt_ext_setoid_functor_ty() -> Expr {
    arrow(type1(), arrow(type1(), type1()))
}
/// SetoidProduct: the product of two setoids.
/// SetoidProduct : (α β : Type) → \[Setoid α\] → \[Setoid β\] → Type
pub fn qt_ext_setoid_product_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "α",
        type0(),
        pi(
            BinderInfo::Default,
            "β",
            type0(),
            pi(
                BinderInfo::InstImplicit,
                "sa",
                app(cst("Setoid"), bvar(1)),
                pi(
                    BinderInfo::InstImplicit,
                    "sb",
                    app(cst("Setoid"), bvar(1)),
                    type0(),
                ),
            ),
        ),
    )
}
/// SetoidCoproduct: the coproduct (disjoint union) of two setoids.
/// SetoidCoproduct : (α β : Type) → \[Setoid α\] → \[Setoid β\] → Type
pub fn qt_ext_setoid_coproduct_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "α",
        type0(),
        pi(
            BinderInfo::Default,
            "β",
            type0(),
            pi(
                BinderInfo::InstImplicit,
                "sa",
                app(cst("Setoid"), bvar(1)),
                pi(
                    BinderInfo::InstImplicit,
                    "sb",
                    app(cst("Setoid"), bvar(1)),
                    type0(),
                ),
            ),
        ),
    )
}
/// SetoidExponential: the function-setoid (exponential object).
/// SetoidExponential : (α β : Type) → \[Setoid α\] → \[Setoid β\] → Type
pub fn qt_ext_setoid_exponential_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "α",
        type0(),
        pi(
            BinderInfo::Default,
            "β",
            type0(),
            pi(
                BinderInfo::InstImplicit,
                "sa",
                app(cst("Setoid"), bvar(1)),
                pi(
                    BinderInfo::InstImplicit,
                    "sb",
                    app(cst("Setoid"), bvar(1)),
                    type0(),
                ),
            ),
        ),
    )
}
/// FreeGroupWords: the type of reduced words over a generator set.
/// FreeGroupWords : (α : Type) → Type
pub fn qt_ext_free_group_words_ty() -> Expr {
    pi(BinderInfo::Default, "α", type0(), type0())
}
/// FreeGroupEquiv: the equivalence relation on words for the free group.
/// FreeGroupEquiv : (α : Type) → FreeGroupWords α → FreeGroupWords α → Prop
pub fn qt_ext_free_group_equiv_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "α",
        type0(),
        arrow(
            app(cst("FreeGroupWords"), bvar(0)),
            arrow(app(cst("FreeGroupWords"), bvar(1)), prop()),
        ),
    )
}
/// FreeGroup: the free group as a quotient of words.
/// FreeGroup : (α : Type) → Type
pub fn qt_ext_free_group_ty() -> Expr {
    pi(BinderInfo::Default, "α", type0(), type0())
}
/// FreeGroup.mul: group multiplication in the free group.
/// FreeGroup.mul : (α : Type) → FreeGroup α → FreeGroup α → FreeGroup α
pub fn qt_ext_free_group_mul_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "α",
        type0(),
        arrow(
            app(cst("FreeGroup"), bvar(0)),
            arrow(
                app(cst("FreeGroup"), bvar(1)),
                app(cst("FreeGroup"), bvar(2)),
            ),
        ),
    )
}
/// FreeGroup.inv: group inversion in the free group.
/// FreeGroup.inv : (α : Type) → FreeGroup α → FreeGroup α
pub fn qt_ext_free_group_inv_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "α",
        type0(),
        arrow(
            app(cst("FreeGroup"), bvar(0)),
            app(cst("FreeGroup"), bvar(1)),
        ),
    )
}
/// IntEquiv: the equivalence relation (m,n) ~ (m',n') iff m+n' = m'+n.
/// IntEquiv : (ℕ × ℕ) → (ℕ × ℕ) → Prop
pub fn qt_ext_int_equiv_ty() -> Expr {
    arrow(
        app2(cst("Prod"), cst("Nat"), cst("Nat")),
        arrow(app2(cst("Prod"), cst("Nat"), cst("Nat")), prop()),
    )
}
/// IntAsQuotient: the integers constructed as a quotient of ℕ×ℕ.
/// IntAsQuotient : Type
pub fn qt_ext_int_as_quotient_ty() -> Expr {
    type0()
}
/// IntAsQuotient.add: addition on the integer quotient.
/// IntAdd : IntAsQuotient → IntAsQuotient → IntAsQuotient
pub fn qt_ext_int_add_ty() -> Expr {
    arrow(
        cst("IntAsQuotient"),
        arrow(cst("IntAsQuotient"), cst("IntAsQuotient")),
    )
}
/// IntAsQuotient.neg: negation on the integer quotient.
/// IntNeg : IntAsQuotient → IntAsQuotient
pub fn qt_ext_int_neg_ty() -> Expr {
    arrow(cst("IntAsQuotient"), cst("IntAsQuotient"))
}
/// RatEquiv: the equivalence relation (p,q) ~ (p',q') iff p*q' = p'*q.
/// RatEquiv : (ℤ × ℤ*) → (ℤ × ℤ*) → Prop
pub fn qt_ext_rat_equiv_ty() -> Expr {
    arrow(
        app2(cst("Prod"), cst("Int"), cst("NonzeroInt")),
        arrow(app2(cst("Prod"), cst("Int"), cst("NonzeroInt")), prop()),
    )
}
/// RatAsQuotient: the rationals constructed as a quotient.
/// RatAsQuotient : Type
pub fn qt_ext_rat_as_quotient_ty() -> Expr {
    type0()
}
/// RatAsQuotient.add: addition on rationals.
/// RatAdd : RatAsQuotient → RatAsQuotient → RatAsQuotient
pub fn qt_ext_rat_add_ty() -> Expr {
    arrow(
        cst("RatAsQuotient"),
        arrow(cst("RatAsQuotient"), cst("RatAsQuotient")),
    )
}
/// RatAsQuotient.mul: multiplication on rationals.
/// RatMul : RatAsQuotient → RatAsQuotient → RatAsQuotient
pub fn qt_ext_rat_mul_ty() -> Expr {
    arrow(
        cst("RatAsQuotient"),
        arrow(cst("RatAsQuotient"), cst("RatAsQuotient")),
    )
}
/// OrbitType: the orbit type G\X for a group action.
/// OrbitType : (G : Group) → (X : Type) → GroupAction G X → Type
pub fn qt_ext_orbit_type_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "G",
        cst("Group"),
        pi(
            BinderInfo::Default,
            "X",
            type0(),
            arrow(app2(cst("GroupAction"), bvar(1), bvar(0)), type0()),
        ),
    )
}
/// OrbitEquiv: the equivalence relation for the orbit decomposition.
/// OrbitEquiv : (G : Group) → (X : Type) → GroupAction G X → X → X → Prop
pub fn qt_ext_orbit_equiv_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "G",
        cst("Group"),
        pi(
            BinderInfo::Default,
            "X",
            type0(),
            pi(
                BinderInfo::Default,
                "act",
                app2(cst("GroupAction"), bvar(1), bvar(0)),
                arrow(bvar(1), arrow(bvar(2), prop())),
            ),
        ),
    )
}
/// OrbitStabilizer: the orbit-stabilizer theorem.
/// |G| = |Orbit x| * |Stabilizer x|
pub fn qt_ext_orbit_stabilizer_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "G",
        cst("Group"),
        pi(
            BinderInfo::Default,
            "X",
            type0(),
            pi(
                BinderInfo::Default,
                "x",
                bvar(0),
                app(cst("OrbitStabilizerThm"), bvar(0)),
            ),
        ),
    )
}
/// GroupoidQuotient: the quotient of a type by a groupoid action.
/// GroupoidQuotient : (α : Type) → Groupoid α → Type
pub fn qt_ext_groupoid_quotient_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "α",
        type0(),
        arrow(app(cst("Groupoid"), bvar(0)), type0()),
    )
}
/// GroupoidQuotient.mk: constructor for the groupoid quotient.
/// GroupoidQuotient.mk : {α : Type} → {G : Groupoid α} → α → GroupoidQuotient α G
pub fn qt_ext_groupoid_quotient_mk_ty() -> Expr {
    pi(
        BinderInfo::Implicit,
        "α",
        type0(),
        pi(
            BinderInfo::Implicit,
            "G",
            app(cst("Groupoid"), bvar(0)),
            arrow(bvar(1), app2(cst("GroupoidQuotient"), bvar(2), bvar(1))),
        ),
    )
}
/// RezkCompletion: the Rezk completion of a precategory.
/// RezkCompletion : Precategory → Category
pub fn qt_ext_rezk_completion_ty() -> Expr {
    arrow(cst("Precategory"), cst("Category"))
}
/// RezkCompletionFunctor: the canonical functor into the Rezk completion.
/// RezkCompletionFunctor : (C : Precategory) → Functor C (RezkCompletion C)
pub fn qt_ext_rezk_completion_functor_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "C",
        cst("Precategory"),
        app2(cst("Functor"), bvar(0), app(cst("RezkCompletion"), bvar(1))),
    )
}
/// RezkUniversalProperty: the Rezk completion is initial among univalent completions.
/// RezkUniversal : (C : Precategory) → (D : Category) → IsUnivalent D →
///   Equiv (Functor (RezkCompletion C) D) (Functor C D)
pub fn qt_ext_rezk_universal_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "C",
        cst("Precategory"),
        pi(
            BinderInfo::Default,
            "D",
            cst("Category"),
            arrow(
                app(cst("IsUnivalent"), bvar(0)),
                app2(
                    cst("Equiv"),
                    app2(cst("Functor"), app(cst("RezkCompletion"), bvar(2)), bvar(1)),
                    app2(cst("Functor"), bvar(2), bvar(1)),
                ),
            ),
        ),
    )
}
/// PartialityMonad: the partiality monad as a quotient of computation trees.
/// PartialityMonad : Type → Type
pub fn qt_ext_partiality_monad_ty() -> Expr {
    arrow(type0(), type0())
}
/// PartialityMonad.now: the return of the partiality monad.
/// PartialityNow : {α : Type} → α → PartialityMonad α
pub fn qt_ext_partiality_now_ty() -> Expr {
    pi(
        BinderInfo::Implicit,
        "α",
        type0(),
        arrow(bvar(0), app(cst("PartialityMonad"), bvar(1))),
    )
}
/// PartialityMonad.later: the delay constructor.
/// PartialityLater : {α : Type} → PartialityMonad α → PartialityMonad α
pub fn qt_ext_partiality_later_ty() -> Expr {
    pi(
        BinderInfo::Implicit,
        "α",
        type0(),
        arrow(
            app(cst("PartialityMonad"), bvar(0)),
            app(cst("PartialityMonad"), bvar(1)),
        ),
    )
}
/// PartialityMonad.bind: monadic bind.
/// PartialityBind : {α β : Type} → PartialityMonad α → (α → PartialityMonad β) → PartialityMonad β
pub fn qt_ext_partiality_bind_ty() -> Expr {
    pi(
        BinderInfo::Implicit,
        "α",
        type0(),
        pi(
            BinderInfo::Implicit,
            "β",
            type0(),
            arrow(
                app(cst("PartialityMonad"), bvar(1)),
                arrow(
                    arrow(bvar(2), app(cst("PartialityMonad"), bvar(1))),
                    app(cst("PartialityMonad"), bvar(1)),
                ),
            ),
        ),
    )
}
/// SetoidMorphism: a function respecting setoid relations.
/// SetoidMorphism : (α β : Type) → \[Setoid α\] → \[Setoid β\] → Type
pub fn qt_ext_setoid_morphism_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "α",
        type0(),
        pi(
            BinderInfo::Default,
            "β",
            type0(),
            pi(
                BinderInfo::InstImplicit,
                "sa",
                app(cst("Setoid"), bvar(1)),
                pi(
                    BinderInfo::InstImplicit,
                    "sb",
                    app(cst("Setoid"), bvar(1)),
                    type0(),
                ),
            ),
        ),
    )
}
/// SetoidIso: an isomorphism of setoids.
/// SetoidIso : (α β : Type) → \[Setoid α\] → \[Setoid β\] → Prop
pub fn qt_ext_setoid_iso_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "α",
        type0(),
        pi(
            BinderInfo::Default,
            "β",
            type0(),
            pi(
                BinderInfo::InstImplicit,
                "sa",
                app(cst("Setoid"), bvar(1)),
                pi(
                    BinderInfo::InstImplicit,
                    "sb",
                    app(cst("Setoid"), bvar(1)),
                    prop(),
                ),
            ),
        ),
    )
}
/// Setoid.quotientSetoid: the quotient of a setoid by a sub-relation.
/// SetoidQuotientSetoid : (α : Type) → \[Setoid α\] → (α → α → Prop) → Setoid (Quotient)
pub fn qt_ext_quotient_setoid_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "α",
        type0(),
        pi(
            BinderInfo::InstImplicit,
            "s",
            app(cst("Setoid"), bvar(0)),
            arrow(
                arrow(bvar(1), arrow(bvar(2), prop())),
                app(cst("Setoid"), cst("Quotient")),
            ),
        ),
    )
}
/// Setoid.congruence: the smallest congruence relation.
/// SetoidCongruence : (α : Type) → (α → α → Prop) → (α → α → Prop)
pub fn qt_ext_setoid_congruence_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "α",
        type0(),
        arrow(
            arrow(bvar(0), arrow(bvar(1), prop())),
            arrow(bvar(1), arrow(bvar(2), prop())),
        ),
    )
}
/// Register all extended quotient-type axioms in an environment.
pub fn register_quotient_types_extended(env: &mut Environment) -> Result<(), String> {
    let axioms: &[(&str, Expr)] = &[
        ("QuotientElimType", qt_ext_quotient_elim_type_ty()),
        ("QuotientInductionOn", qt_ext_quotient_induction_on_ty()),
        ("QuotientSurjective", qt_ext_quotient_surjective_ty()),
        ("QuotientFunctor", qt_ext_quotient_functor_ty()),
        ("QuotientMonadPure", qt_ext_quotient_monad_pure_ty()),
        ("QuotientMonadBind", qt_ext_quotient_monad_bind_ty()),
        ("SetoidModel", qt_ext_setoid_model_ty()),
        ("EffectiveQuotient", qt_ext_effective_quotient_ty()),
        ("SetoidCategory", qt_ext_setoid_category_ty()),
        ("SetoidFunctor", qt_ext_setoid_functor_ty()),
        ("SetoidProduct", qt_ext_setoid_product_ty()),
        ("SetoidCoproduct", qt_ext_setoid_coproduct_ty()),
        ("SetoidExponential", qt_ext_setoid_exponential_ty()),
        ("FreeGroupWords", qt_ext_free_group_words_ty()),
        ("FreeGroupEquiv", qt_ext_free_group_equiv_ty()),
        ("FreeGroup", qt_ext_free_group_ty()),
        ("FreeGroup.mul", qt_ext_free_group_mul_ty()),
        ("FreeGroup.inv", qt_ext_free_group_inv_ty()),
        ("IntEquiv", qt_ext_int_equiv_ty()),
        ("IntAsQuotient", qt_ext_int_as_quotient_ty()),
        ("IntAdd", qt_ext_int_add_ty()),
        ("IntNeg", qt_ext_int_neg_ty()),
        ("RatEquiv", qt_ext_rat_equiv_ty()),
        ("RatAsQuotient", qt_ext_rat_as_quotient_ty()),
        ("RatAdd", qt_ext_rat_add_ty()),
        ("RatMul", qt_ext_rat_mul_ty()),
        ("OrbitType", qt_ext_orbit_type_ty()),
        ("OrbitEquiv", qt_ext_orbit_equiv_ty()),
        ("OrbitStabilizerThm", qt_ext_orbit_stabilizer_ty()),
        ("GroupoidQuotient", qt_ext_groupoid_quotient_ty()),
        ("GroupoidQuotient.mk", qt_ext_groupoid_quotient_mk_ty()),
        ("RezkCompletion", qt_ext_rezk_completion_ty()),
        ("RezkCompletionFunctor", qt_ext_rezk_completion_functor_ty()),
        ("RezkUniversal", qt_ext_rezk_universal_ty()),
        ("PartialityMonad", qt_ext_partiality_monad_ty()),
        ("PartialityNow", qt_ext_partiality_now_ty()),
        ("PartialityLater", qt_ext_partiality_later_ty()),
        ("PartialityBind", qt_ext_partiality_bind_ty()),
        ("SetoidMorphism", qt_ext_setoid_morphism_ty()),
        ("SetoidIso", qt_ext_setoid_iso_ty()),
        ("QuotientSetoid", qt_ext_quotient_setoid_ty()),
        ("SetoidCongruence", qt_ext_setoid_congruence_ty()),
        ("ExistsSatisfying", arrow(type0(), prop())),
        ("Group", type0()),
        ("GroupAction", arrow(type0(), arrow(type0(), type0()))),
        ("Groupoid", arrow(type0(), type0())),
        ("Precategory", type0()),
        ("Category", type0()),
        ("IsUnivalent", arrow(type0(), prop())),
        ("Equiv", arrow(type0(), arrow(type0(), prop()))),
        ("FreeGroupWords", arrow(type0(), type0())),
        ("NonzeroInt", type0()),
        ("Nat", type0()),
        ("Int", type0()),
        ("Prod", arrow(type0(), arrow(type0(), type0()))),
        ("OrbitStabilizerThm", arrow(type0(), prop())),
    ];
    for (name, ty) in axioms {
        env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty: ty.clone(),
        })
        .ok();
    }
    Ok(())
}
