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
#[allow(dead_code)]
pub fn type1() -> Expr {
    Expr::Sort(Level::succ(Level::succ(Level::zero())))
}
#[allow(dead_code)]
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
pub(super) fn nat_ty() -> Expr {
    Expr::Const(Name::str("Nat"), vec![])
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
/// Build `Set α` — which is `α → Prop`.
/// `elem_ty` is the expression for `α`.
#[allow(dead_code)]
pub(super) fn set_ty_of(elem_ty: Expr) -> Expr {
    arrow(elem_ty, prop())
}
/// Build `Eq @{} T a b`.
#[allow(dead_code)]
pub(super) fn mk_eq_expr(ty: Expr, lhs: Expr, rhs: Expr) -> Expr {
    app3(cst("Eq"), ty, lhs, rhs)
}
/// Build the set theory environment containing set types, operations, and theorems.
#[allow(clippy::too_many_lines)]
pub fn build_set_env(env: &mut Environment) -> Result<(), String> {
    add_axiom(env, "Set", vec![], arrow(type0(), type0()))?;
    let set_mem_ty = pi(
        BinderInfo::Implicit,
        "α",
        type0(),
        pi(
            BinderInfo::Default,
            "a",
            bvar(0),
            pi(BinderInfo::Default, "s", app(cst("Set"), bvar(1)), prop()),
        ),
    );
    add_axiom(env, "Set.mem", vec![], set_mem_ty)?;
    let set_empty_ty = pi(BinderInfo::Implicit, "α", type0(), app(cst("Set"), bvar(0)));
    add_axiom(env, "Set.empty", vec![], set_empty_ty)?;
    let set_univ_ty = pi(BinderInfo::Implicit, "α", type0(), app(cst("Set"), bvar(0)));
    add_axiom(env, "Set.univ", vec![], set_univ_ty)?;
    let set_singleton_ty = pi(
        BinderInfo::Implicit,
        "α",
        type0(),
        pi(BinderInfo::Default, "a", bvar(0), app(cst("Set"), bvar(1))),
    );
    add_axiom(env, "Set.singleton", vec![], set_singleton_ty)?;
    let set_union_ty = pi(
        BinderInfo::Implicit,
        "α",
        type0(),
        pi(
            BinderInfo::Default,
            "s",
            app(cst("Set"), bvar(0)),
            pi(
                BinderInfo::Default,
                "t",
                app(cst("Set"), bvar(1)),
                app(cst("Set"), bvar(2)),
            ),
        ),
    );
    add_axiom(env, "Set.union", vec![], set_union_ty)?;
    let set_inter_ty = pi(
        BinderInfo::Implicit,
        "α",
        type0(),
        pi(
            BinderInfo::Default,
            "s",
            app(cst("Set"), bvar(0)),
            pi(
                BinderInfo::Default,
                "t",
                app(cst("Set"), bvar(1)),
                app(cst("Set"), bvar(2)),
            ),
        ),
    );
    add_axiom(env, "Set.inter", vec![], set_inter_ty)?;
    let set_diff_ty = pi(
        BinderInfo::Implicit,
        "α",
        type0(),
        pi(
            BinderInfo::Default,
            "s",
            app(cst("Set"), bvar(0)),
            pi(
                BinderInfo::Default,
                "t",
                app(cst("Set"), bvar(1)),
                app(cst("Set"), bvar(2)),
            ),
        ),
    );
    add_axiom(env, "Set.diff", vec![], set_diff_ty)?;
    let set_compl_ty = pi(
        BinderInfo::Implicit,
        "α",
        type0(),
        pi(
            BinderInfo::Default,
            "s",
            app(cst("Set"), bvar(0)),
            app(cst("Set"), bvar(1)),
        ),
    );
    add_axiom(env, "Set.compl", vec![], set_compl_ty)?;
    let set_subset_ty = pi(
        BinderInfo::Implicit,
        "α",
        type0(),
        pi(
            BinderInfo::Default,
            "s",
            app(cst("Set"), bvar(0)),
            pi(BinderInfo::Default, "t", app(cst("Set"), bvar(1)), prop()),
        ),
    );
    add_axiom(env, "Set.subset", vec![], set_subset_ty)?;
    let set_ssubset_ty = pi(
        BinderInfo::Implicit,
        "α",
        type0(),
        pi(
            BinderInfo::Default,
            "s",
            app(cst("Set"), bvar(0)),
            pi(BinderInfo::Default, "t", app(cst("Set"), bvar(1)), prop()),
        ),
    );
    add_axiom(env, "Set.ssubset", vec![], set_ssubset_ty)?;
    let set_image_ty = pi(
        BinderInfo::Implicit,
        "α",
        type0(),
        pi(
            BinderInfo::Implicit,
            "β",
            type0(),
            pi(
                BinderInfo::Default,
                "f",
                arrow(bvar(1), bvar(1)),
                pi(
                    BinderInfo::Default,
                    "s",
                    app(cst("Set"), bvar(2)),
                    app(cst("Set"), bvar(2)),
                ),
            ),
        ),
    );
    add_axiom(env, "Set.image", vec![], set_image_ty)?;
    let set_preimage_ty = pi(
        BinderInfo::Implicit,
        "α",
        type0(),
        pi(
            BinderInfo::Implicit,
            "β",
            type0(),
            pi(
                BinderInfo::Default,
                "f",
                arrow(bvar(1), bvar(1)),
                pi(
                    BinderInfo::Default,
                    "s",
                    app(cst("Set"), bvar(1)),
                    app(cst("Set"), bvar(3)),
                ),
            ),
        ),
    );
    add_axiom(env, "Set.preimage", vec![], set_preimage_ty)?;
    let set_range_ty = pi(
        BinderInfo::Implicit,
        "α",
        type0(),
        pi(
            BinderInfo::Implicit,
            "β",
            type0(),
            pi(
                BinderInfo::Default,
                "f",
                arrow(bvar(1), bvar(1)),
                app(cst("Set"), bvar(1)),
            ),
        ),
    );
    add_axiom(env, "Set.range", vec![], set_range_ty)?;
    let set_sep_ty = pi(
        BinderInfo::Implicit,
        "α",
        type0(),
        pi(
            BinderInfo::Default,
            "s",
            app(cst("Set"), bvar(0)),
            pi(
                BinderInfo::Default,
                "p",
                arrow(bvar(1), prop()),
                app(cst("Set"), bvar(2)),
            ),
        ),
    );
    add_axiom(env, "Set.sep", vec![], set_sep_ty)?;
    let set_insert_ty = pi(
        BinderInfo::Implicit,
        "α",
        type0(),
        pi(
            BinderInfo::Default,
            "a",
            bvar(0),
            pi(
                BinderInfo::Default,
                "s",
                app(cst("Set"), bvar(1)),
                app(cst("Set"), bvar(2)),
            ),
        ),
    );
    add_axiom(env, "Set.insert", vec![], set_insert_ty)?;
    let set_erase_ty = pi(
        BinderInfo::Implicit,
        "α",
        type0(),
        pi(
            BinderInfo::InstImplicit,
            "_inst",
            app(cst("DecidableEq"), bvar(0)),
            pi(
                BinderInfo::Default,
                "s",
                app(cst("Set"), bvar(1)),
                pi(BinderInfo::Default, "a", bvar(2), app(cst("Set"), bvar(3))),
            ),
        ),
    );
    add_axiom(env, "Set.erase", vec![], set_erase_ty)?;
    let mem_union_ty = pi(
        BinderInfo::Implicit,
        "α",
        type0(),
        pi(
            BinderInfo::Implicit,
            "a",
            bvar(0),
            pi(
                BinderInfo::Implicit,
                "s",
                app(cst("Set"), bvar(1)),
                pi(
                    BinderInfo::Implicit,
                    "t",
                    app(cst("Set"), bvar(2)),
                    app2(
                        cst("Iff"),
                        app2(
                            cst("Set.mem"),
                            bvar(2),
                            app2(cst("Set.union"), bvar(1), bvar(0)),
                        ),
                        app2(
                            cst("Or"),
                            app2(cst("Set.mem"), bvar(2), bvar(1)),
                            app2(cst("Set.mem"), bvar(2), bvar(0)),
                        ),
                    ),
                ),
            ),
        ),
    );
    add_axiom(env, "Set.mem_union", vec![], mem_union_ty)?;
    let mem_inter_ty = pi(
        BinderInfo::Implicit,
        "α",
        type0(),
        pi(
            BinderInfo::Implicit,
            "a",
            bvar(0),
            pi(
                BinderInfo::Implicit,
                "s",
                app(cst("Set"), bvar(1)),
                pi(
                    BinderInfo::Implicit,
                    "t",
                    app(cst("Set"), bvar(2)),
                    app2(
                        cst("Iff"),
                        app2(
                            cst("Set.mem"),
                            bvar(2),
                            app2(cst("Set.inter"), bvar(1), bvar(0)),
                        ),
                        app2(
                            cst("And"),
                            app2(cst("Set.mem"), bvar(2), bvar(1)),
                            app2(cst("Set.mem"), bvar(2), bvar(0)),
                        ),
                    ),
                ),
            ),
        ),
    );
    add_axiom(env, "Set.mem_inter", vec![], mem_inter_ty)?;
    let mem_diff_ty = pi(
        BinderInfo::Implicit,
        "α",
        type0(),
        pi(
            BinderInfo::Implicit,
            "a",
            bvar(0),
            pi(
                BinderInfo::Implicit,
                "s",
                app(cst("Set"), bvar(1)),
                pi(
                    BinderInfo::Implicit,
                    "t",
                    app(cst("Set"), bvar(2)),
                    app2(
                        cst("Iff"),
                        app2(
                            cst("Set.mem"),
                            bvar(2),
                            app2(cst("Set.diff"), bvar(1), bvar(0)),
                        ),
                        app2(
                            cst("And"),
                            app2(cst("Set.mem"), bvar(2), bvar(1)),
                            app(cst("Not"), app2(cst("Set.mem"), bvar(2), bvar(0))),
                        ),
                    ),
                ),
            ),
        ),
    );
    add_axiom(env, "Set.mem_diff", vec![], mem_diff_ty)?;
    let mem_compl_ty = pi(
        BinderInfo::Implicit,
        "α",
        type0(),
        pi(
            BinderInfo::Implicit,
            "a",
            bvar(0),
            pi(
                BinderInfo::Implicit,
                "s",
                app(cst("Set"), bvar(1)),
                app2(
                    cst("Iff"),
                    app2(cst("Set.mem"), bvar(1), app(cst("Set.compl"), bvar(0))),
                    app(cst("Not"), app2(cst("Set.mem"), bvar(1), bvar(0))),
                ),
            ),
        ),
    );
    add_axiom(env, "Set.mem_compl", vec![], mem_compl_ty)?;
    let union_comm_ty = pi(
        BinderInfo::Implicit,
        "α",
        type0(),
        pi(
            BinderInfo::Default,
            "s",
            app(cst("Set"), bvar(0)),
            pi(
                BinderInfo::Default,
                "t",
                app(cst("Set"), bvar(1)),
                mk_eq_expr(
                    app(cst("Set"), bvar(2)),
                    app2(cst("Set.union"), bvar(1), bvar(0)),
                    app2(cst("Set.union"), bvar(0), bvar(1)),
                ),
            ),
        ),
    );
    add_axiom(env, "Set.union_comm", vec![], union_comm_ty)?;
    let union_assoc_ty = pi(
        BinderInfo::Implicit,
        "α",
        type0(),
        pi(
            BinderInfo::Default,
            "s",
            app(cst("Set"), bvar(0)),
            pi(
                BinderInfo::Default,
                "t",
                app(cst("Set"), bvar(1)),
                pi(
                    BinderInfo::Default,
                    "u",
                    app(cst("Set"), bvar(2)),
                    mk_eq_expr(
                        app(cst("Set"), bvar(3)),
                        app2(
                            cst("Set.union"),
                            app2(cst("Set.union"), bvar(2), bvar(1)),
                            bvar(0),
                        ),
                        app2(
                            cst("Set.union"),
                            bvar(2),
                            app2(cst("Set.union"), bvar(1), bvar(0)),
                        ),
                    ),
                ),
            ),
        ),
    );
    add_axiom(env, "Set.union_assoc", vec![], union_assoc_ty)?;
    let inter_comm_ty = pi(
        BinderInfo::Implicit,
        "α",
        type0(),
        pi(
            BinderInfo::Default,
            "s",
            app(cst("Set"), bvar(0)),
            pi(
                BinderInfo::Default,
                "t",
                app(cst("Set"), bvar(1)),
                mk_eq_expr(
                    app(cst("Set"), bvar(2)),
                    app2(cst("Set.inter"), bvar(1), bvar(0)),
                    app2(cst("Set.inter"), bvar(0), bvar(1)),
                ),
            ),
        ),
    );
    add_axiom(env, "Set.inter_comm", vec![], inter_comm_ty)?;
    let inter_assoc_ty = pi(
        BinderInfo::Implicit,
        "α",
        type0(),
        pi(
            BinderInfo::Default,
            "s",
            app(cst("Set"), bvar(0)),
            pi(
                BinderInfo::Default,
                "t",
                app(cst("Set"), bvar(1)),
                pi(
                    BinderInfo::Default,
                    "u",
                    app(cst("Set"), bvar(2)),
                    mk_eq_expr(
                        app(cst("Set"), bvar(3)),
                        app2(
                            cst("Set.inter"),
                            app2(cst("Set.inter"), bvar(2), bvar(1)),
                            bvar(0),
                        ),
                        app2(
                            cst("Set.inter"),
                            bvar(2),
                            app2(cst("Set.inter"), bvar(1), bvar(0)),
                        ),
                    ),
                ),
            ),
        ),
    );
    add_axiom(env, "Set.inter_assoc", vec![], inter_assoc_ty)?;
    let union_empty_ty = pi(
        BinderInfo::Implicit,
        "α",
        type0(),
        pi(
            BinderInfo::Default,
            "s",
            app(cst("Set"), bvar(0)),
            mk_eq_expr(
                app(cst("Set"), bvar(1)),
                app2(cst("Set.union"), bvar(0), cst("Set.empty")),
                bvar(0),
            ),
        ),
    );
    add_axiom(env, "Set.union_empty", vec![], union_empty_ty)?;
    let empty_union_ty = pi(
        BinderInfo::Implicit,
        "α",
        type0(),
        pi(
            BinderInfo::Default,
            "s",
            app(cst("Set"), bvar(0)),
            mk_eq_expr(
                app(cst("Set"), bvar(1)),
                app2(cst("Set.union"), cst("Set.empty"), bvar(0)),
                bvar(0),
            ),
        ),
    );
    add_axiom(env, "Set.empty_union", vec![], empty_union_ty)?;
    let inter_univ_ty = pi(
        BinderInfo::Implicit,
        "α",
        type0(),
        pi(
            BinderInfo::Default,
            "s",
            app(cst("Set"), bvar(0)),
            mk_eq_expr(
                app(cst("Set"), bvar(1)),
                app2(cst("Set.inter"), bvar(0), cst("Set.univ")),
                bvar(0),
            ),
        ),
    );
    add_axiom(env, "Set.inter_univ", vec![], inter_univ_ty)?;
    let univ_inter_ty = pi(
        BinderInfo::Implicit,
        "α",
        type0(),
        pi(
            BinderInfo::Default,
            "s",
            app(cst("Set"), bvar(0)),
            mk_eq_expr(
                app(cst("Set"), bvar(1)),
                app2(cst("Set.inter"), cst("Set.univ"), bvar(0)),
                bvar(0),
            ),
        ),
    );
    add_axiom(env, "Set.univ_inter", vec![], univ_inter_ty)?;
    let subset_refl_ty = pi(
        BinderInfo::Implicit,
        "α",
        type0(),
        pi(
            BinderInfo::Default,
            "s",
            app(cst("Set"), bvar(0)),
            app2(cst("Set.subset"), bvar(0), bvar(0)),
        ),
    );
    add_axiom(env, "Set.subset_refl", vec![], subset_refl_ty)?;
    let subset_trans_ty = pi(
        BinderInfo::Implicit,
        "α",
        type0(),
        pi(
            BinderInfo::Implicit,
            "s",
            app(cst("Set"), bvar(0)),
            pi(
                BinderInfo::Implicit,
                "t",
                app(cst("Set"), bvar(1)),
                pi(
                    BinderInfo::Implicit,
                    "u",
                    app(cst("Set"), bvar(2)),
                    pi(
                        BinderInfo::Default,
                        "h1",
                        app2(cst("Set.subset"), bvar(2), bvar(1)),
                        pi(
                            BinderInfo::Default,
                            "h2",
                            app2(cst("Set.subset"), bvar(2), bvar(1)),
                            app2(cst("Set.subset"), bvar(4), bvar(2)),
                        ),
                    ),
                ),
            ),
        ),
    );
    add_axiom(env, "Set.subset_trans", vec![], subset_trans_ty)?;
    let subset_antisymm_ty = pi(
        BinderInfo::Implicit,
        "α",
        type0(),
        pi(
            BinderInfo::Implicit,
            "s",
            app(cst("Set"), bvar(0)),
            pi(
                BinderInfo::Implicit,
                "t",
                app(cst("Set"), bvar(1)),
                pi(
                    BinderInfo::Default,
                    "h1",
                    app2(cst("Set.subset"), bvar(1), bvar(0)),
                    pi(
                        BinderInfo::Default,
                        "h2",
                        app2(cst("Set.subset"), bvar(1), bvar(2)),
                        mk_eq_expr(app(cst("Set"), bvar(4)), bvar(3), bvar(2)),
                    ),
                ),
            ),
        ),
    );
    add_axiom(env, "Set.subset_antisymm", vec![], subset_antisymm_ty)?;
    let uid_ty = pi(
        BinderInfo::Implicit,
        "α",
        type0(),
        pi(
            BinderInfo::Default,
            "s",
            app(cst("Set"), bvar(0)),
            pi(
                BinderInfo::Default,
                "t",
                app(cst("Set"), bvar(1)),
                pi(
                    BinderInfo::Default,
                    "u",
                    app(cst("Set"), bvar(2)),
                    mk_eq_expr(
                        app(cst("Set"), bvar(3)),
                        app2(
                            cst("Set.union"),
                            bvar(2),
                            app2(cst("Set.inter"), bvar(1), bvar(0)),
                        ),
                        app2(
                            cst("Set.inter"),
                            app2(cst("Set.union"), bvar(2), bvar(1)),
                            app2(cst("Set.union"), bvar(2), bvar(0)),
                        ),
                    ),
                ),
            ),
        ),
    );
    add_axiom(env, "Set.union_inter_distrib", vec![], uid_ty)?;
    let iud_ty = pi(
        BinderInfo::Implicit,
        "α",
        type0(),
        pi(
            BinderInfo::Default,
            "s",
            app(cst("Set"), bvar(0)),
            pi(
                BinderInfo::Default,
                "t",
                app(cst("Set"), bvar(1)),
                pi(
                    BinderInfo::Default,
                    "u",
                    app(cst("Set"), bvar(2)),
                    mk_eq_expr(
                        app(cst("Set"), bvar(3)),
                        app2(
                            cst("Set.inter"),
                            bvar(2),
                            app2(cst("Set.union"), bvar(1), bvar(0)),
                        ),
                        app2(
                            cst("Set.union"),
                            app2(cst("Set.inter"), bvar(2), bvar(1)),
                            app2(cst("Set.inter"), bvar(2), bvar(0)),
                        ),
                    ),
                ),
            ),
        ),
    );
    add_axiom(env, "Set.inter_union_distrib", vec![], iud_ty)?;
    let compl_compl_ty = pi(
        BinderInfo::Implicit,
        "α",
        type0(),
        pi(
            BinderInfo::Default,
            "s",
            app(cst("Set"), bvar(0)),
            mk_eq_expr(
                app(cst("Set"), bvar(1)),
                app(cst("Set.compl"), app(cst("Set.compl"), bvar(0))),
                bvar(0),
            ),
        ),
    );
    add_axiom(env, "Set.compl_compl", vec![], compl_compl_ty)?;
    let compl_union_ty = pi(
        BinderInfo::Implicit,
        "α",
        type0(),
        pi(
            BinderInfo::Default,
            "s",
            app(cst("Set"), bvar(0)),
            pi(
                BinderInfo::Default,
                "t",
                app(cst("Set"), bvar(1)),
                mk_eq_expr(
                    app(cst("Set"), bvar(2)),
                    app(cst("Set.compl"), app2(cst("Set.union"), bvar(1), bvar(0))),
                    app2(
                        cst("Set.inter"),
                        app(cst("Set.compl"), bvar(1)),
                        app(cst("Set.compl"), bvar(0)),
                    ),
                ),
            ),
        ),
    );
    add_axiom(env, "Set.compl_union", vec![], compl_union_ty)?;
    let compl_inter_ty = pi(
        BinderInfo::Implicit,
        "α",
        type0(),
        pi(
            BinderInfo::Default,
            "s",
            app(cst("Set"), bvar(0)),
            pi(
                BinderInfo::Default,
                "t",
                app(cst("Set"), bvar(1)),
                mk_eq_expr(
                    app(cst("Set"), bvar(2)),
                    app(cst("Set.compl"), app2(cst("Set.inter"), bvar(1), bvar(0))),
                    app2(
                        cst("Set.union"),
                        app(cst("Set.compl"), bvar(1)),
                        app(cst("Set.compl"), bvar(0)),
                    ),
                ),
            ),
        ),
    );
    add_axiom(env, "Set.compl_inter", vec![], compl_inter_ty)?;
    let image_comp_ty = pi(
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
                    BinderInfo::Default,
                    "g",
                    arrow(bvar(1), bvar(1)),
                    pi(
                        BinderInfo::Default,
                        "f",
                        arrow(bvar(3), bvar(3)),
                        pi(
                            BinderInfo::Default,
                            "s",
                            app(cst("Set"), bvar(4)),
                            mk_eq_expr(
                                app(cst("Set"), bvar(3)),
                                app2(
                                    cst("Set.image"),
                                    bvar(2),
                                    app2(cst("Set.image"), bvar(1), bvar(0)),
                                ),
                                app2(
                                    cst("Set.image"),
                                    lam(
                                        BinderInfo::Default,
                                        "x",
                                        bvar(5),
                                        app(bvar(3), app(bvar(2), bvar(0))),
                                    ),
                                    bvar(0),
                                ),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    );
    add_axiom(env, "Set.image_comp", vec![], image_comp_ty)?;
    let preimage_comp_ty = pi(
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
                    BinderInfo::Default,
                    "g",
                    arrow(bvar(1), bvar(1)),
                    pi(
                        BinderInfo::Default,
                        "f",
                        arrow(bvar(3), bvar(3)),
                        pi(
                            BinderInfo::Default,
                            "s",
                            app(cst("Set"), bvar(2)),
                            mk_eq_expr(
                                app(cst("Set"), bvar(5)),
                                app2(
                                    cst("Set.preimage"),
                                    bvar(1),
                                    app2(cst("Set.preimage"), bvar(2), bvar(0)),
                                ),
                                app2(
                                    cst("Set.preimage"),
                                    lam(
                                        BinderInfo::Default,
                                        "x",
                                        bvar(5),
                                        app(bvar(3), app(bvar(2), bvar(0))),
                                    ),
                                    bvar(0),
                                ),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    );
    add_axiom(env, "Set.preimage_comp", vec![], preimage_comp_ty)?;
    let image_union_ty = pi(
        BinderInfo::Implicit,
        "α",
        type0(),
        pi(
            BinderInfo::Implicit,
            "β",
            type0(),
            pi(
                BinderInfo::Default,
                "f",
                arrow(bvar(1), bvar(1)),
                pi(
                    BinderInfo::Default,
                    "s",
                    app(cst("Set"), bvar(2)),
                    pi(
                        BinderInfo::Default,
                        "t",
                        app(cst("Set"), bvar(3)),
                        mk_eq_expr(
                            app(cst("Set"), bvar(3)),
                            app2(
                                cst("Set.image"),
                                bvar(2),
                                app2(cst("Set.union"), bvar(1), bvar(0)),
                            ),
                            app2(
                                cst("Set.union"),
                                app2(cst("Set.image"), bvar(2), bvar(1)),
                                app2(cst("Set.image"), bvar(2), bvar(0)),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    );
    add_axiom(env, "Set.image_union", vec![], image_union_ty)?;
    let image_inter_subset_ty = pi(
        BinderInfo::Implicit,
        "α",
        type0(),
        pi(
            BinderInfo::Implicit,
            "β",
            type0(),
            pi(
                BinderInfo::Default,
                "f",
                arrow(bvar(1), bvar(1)),
                pi(
                    BinderInfo::Default,
                    "s",
                    app(cst("Set"), bvar(2)),
                    pi(
                        BinderInfo::Default,
                        "t",
                        app(cst("Set"), bvar(3)),
                        app2(
                            cst("Set.subset"),
                            app2(
                                cst("Set.image"),
                                bvar(2),
                                app2(cst("Set.inter"), bvar(1), bvar(0)),
                            ),
                            app2(
                                cst("Set.inter"),
                                app2(cst("Set.image"), bvar(2), bvar(1)),
                                app2(cst("Set.image"), bvar(2), bvar(0)),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    );
    add_axiom(env, "Set.image_inter_subset", vec![], image_inter_subset_ty)?;
    add_axiom(env, "Finset", vec![], arrow(type0(), type0()))?;
    let finset_empty_ty = pi(
        BinderInfo::Implicit,
        "α",
        type0(),
        app(cst("Finset"), bvar(0)),
    );
    add_axiom(env, "Finset.empty", vec![], finset_empty_ty)?;
    let finset_insert_ty = pi(
        BinderInfo::Implicit,
        "α",
        type0(),
        pi(
            BinderInfo::InstImplicit,
            "_inst",
            app(cst("DecidableEq"), bvar(0)),
            pi(
                BinderInfo::Default,
                "a",
                bvar(1),
                pi(
                    BinderInfo::Default,
                    "s",
                    app(cst("Finset"), bvar(2)),
                    app(cst("Finset"), bvar(3)),
                ),
            ),
        ),
    );
    add_axiom(env, "Finset.insert", vec![], finset_insert_ty)?;
    let finset_card_ty = pi(
        BinderInfo::Implicit,
        "α",
        type0(),
        pi(
            BinderInfo::Default,
            "s",
            app(cst("Finset"), bvar(0)),
            nat_ty(),
        ),
    );
    add_axiom(env, "Finset.card", vec![], finset_card_ty)?;
    let finset_union_ty = pi(
        BinderInfo::Implicit,
        "α",
        type0(),
        pi(
            BinderInfo::InstImplicit,
            "_inst",
            app(cst("DecidableEq"), bvar(0)),
            pi(
                BinderInfo::Default,
                "s",
                app(cst("Finset"), bvar(1)),
                pi(
                    BinderInfo::Default,
                    "t",
                    app(cst("Finset"), bvar(2)),
                    app(cst("Finset"), bvar(3)),
                ),
            ),
        ),
    );
    add_axiom(env, "Finset.union", vec![], finset_union_ty)?;
    let finset_inter_ty = pi(
        BinderInfo::Implicit,
        "α",
        type0(),
        pi(
            BinderInfo::InstImplicit,
            "_inst",
            app(cst("DecidableEq"), bvar(0)),
            pi(
                BinderInfo::Default,
                "s",
                app(cst("Finset"), bvar(1)),
                pi(
                    BinderInfo::Default,
                    "t",
                    app(cst("Finset"), bvar(2)),
                    app(cst("Finset"), bvar(3)),
                ),
            ),
        ),
    );
    add_axiom(env, "Finset.inter", vec![], finset_inter_ty)?;
    let finset_to_set_ty = pi(
        BinderInfo::Implicit,
        "α",
        type0(),
        pi(
            BinderInfo::Default,
            "s",
            app(cst("Finset"), bvar(0)),
            app(cst("Set"), bvar(1)),
        ),
    );
    add_axiom(env, "Finset.toSet", vec![], finset_to_set_ty)?;
    let finset_sum_ty = pi(
        BinderInfo::Implicit,
        "α",
        type0(),
        pi(
            BinderInfo::Implicit,
            "β",
            type0(),
            pi(
                BinderInfo::InstImplicit,
                "_inst",
                app(cst("AddCommMonoid"), bvar(0)),
                pi(
                    BinderInfo::Default,
                    "s",
                    app(cst("Finset"), bvar(2)),
                    pi(BinderInfo::Default, "f", arrow(bvar(3), bvar(3)), bvar(3)),
                ),
            ),
        ),
    );
    add_axiom(env, "Finset.sum", vec![], finset_sum_ty)?;
    let card_empty_ty = pi(
        BinderInfo::Implicit,
        "α",
        type0(),
        mk_eq_expr(
            nat_ty(),
            app(cst("Finset.card"), cst("Finset.empty")),
            Expr::Lit(oxilean_kernel::Literal::Nat(0)),
        ),
    );
    add_axiom(env, "Finset.card_empty", vec![], card_empty_ty)?;
    let card_insert_ty = pi(
        BinderInfo::Implicit,
        "α",
        type0(),
        pi(
            BinderInfo::InstImplicit,
            "_inst",
            app(cst("DecidableEq"), bvar(0)),
            pi(
                BinderInfo::Implicit,
                "a",
                bvar(1),
                pi(
                    BinderInfo::Implicit,
                    "s",
                    app(cst("Finset"), bvar(2)),
                    pi(
                        BinderInfo::Default,
                        "h",
                        app(
                            cst("Not"),
                            app2(cst("Set.mem"), bvar(1), app(cst("Finset.toSet"), bvar(0))),
                        ),
                        mk_eq_expr(
                            nat_ty(),
                            app(
                                cst("Finset.card"),
                                app2(cst("Finset.insert"), bvar(2), bvar(1)),
                            ),
                            app(cst("Nat.succ"), app(cst("Finset.card"), bvar(1))),
                        ),
                    ),
                ),
            ),
        ),
    );
    add_axiom(env, "Finset.card_insert", vec![], card_insert_ty)?;
    let card_union_le_ty = pi(
        BinderInfo::Implicit,
        "α",
        type0(),
        pi(
            BinderInfo::InstImplicit,
            "_inst",
            app(cst("DecidableEq"), bvar(0)),
            pi(
                BinderInfo::Default,
                "s",
                app(cst("Finset"), bvar(1)),
                pi(
                    BinderInfo::Default,
                    "t",
                    app(cst("Finset"), bvar(2)),
                    app2(
                        cst("Nat.le"),
                        app(
                            cst("Finset.card"),
                            app2(cst("Finset.union"), bvar(1), bvar(0)),
                        ),
                        app2(
                            cst("Nat.add"),
                            app(cst("Finset.card"), bvar(1)),
                            app(cst("Finset.card"), bvar(0)),
                        ),
                    ),
                ),
            ),
        ),
    );
    add_axiom(env, "Finset.card_union_le", vec![], card_union_le_ty)?;
    Ok(())
}
/// Build `Set α` type expression.
#[allow(dead_code)]
pub fn mk_set(elem_ty: Expr) -> Expr {
    app(cst("Set"), elem_ty)
}
/// Build `Set.mem a s` expression.
#[allow(dead_code)]
pub fn mk_set_mem(elem: Expr, set: Expr) -> Expr {
    app2(cst("Set.mem"), elem, set)
}
/// Build `Set.union s t` expression.
#[allow(dead_code)]
pub fn mk_set_union(s: Expr, t: Expr) -> Expr {
    app2(cst("Set.union"), s, t)
}
/// Build `Set.inter s t` expression.
#[allow(dead_code)]
pub fn mk_set_inter(s: Expr, t: Expr) -> Expr {
    app2(cst("Set.inter"), s, t)
}
/// Build `Set.subset s t` expression.
#[allow(dead_code)]
pub fn mk_set_subset(s: Expr, t: Expr) -> Expr {
    app2(cst("Set.subset"), s, t)
}
/// Build `Set.diff s t` expression.
#[allow(dead_code)]
pub fn mk_set_diff(s: Expr, t: Expr) -> Expr {
    app2(cst("Set.diff"), s, t)
}
/// Build `Set.compl s` expression.
#[allow(dead_code)]
pub fn mk_set_compl(s: Expr) -> Expr {
    app(cst("Set.compl"), s)
}
/// Build `Set.image f s` expression.
#[allow(dead_code)]
pub fn mk_set_image(f: Expr, s: Expr) -> Expr {
    app2(cst("Set.image"), f, s)
}
/// Build `Set.preimage f s` expression.
#[allow(dead_code)]
pub fn mk_set_preimage(f: Expr, s: Expr) -> Expr {
    app2(cst("Set.preimage"), f, s)
}
/// Build `Set.range f` expression.
#[allow(dead_code)]
pub fn mk_set_range(f: Expr) -> Expr {
    app(cst("Set.range"), f)
}
/// Build `Set.singleton a` expression.
#[allow(dead_code)]
pub fn mk_set_singleton(a: Expr) -> Expr {
    app(cst("Set.singleton"), a)
}
/// Build `Set.insert a s` expression.
#[allow(dead_code)]
pub fn mk_set_insert(a: Expr, s: Expr) -> Expr {
    app2(cst("Set.insert"), a, s)
}
/// Build `Set.sep s p` expression.
#[allow(dead_code)]
pub fn mk_set_sep(s: Expr, p: Expr) -> Expr {
    app2(cst("Set.sep"), s, p)
}
/// Build `Set.empty` expression.
#[allow(dead_code)]
pub fn mk_set_empty() -> Expr {
    cst("Set.empty")
}
/// Build `Set.univ` expression.
#[allow(dead_code)]
pub fn mk_set_univ() -> Expr {
    cst("Set.univ")
}
/// Build `Finset α` type expression.
#[allow(dead_code)]
pub fn mk_finset(elem_ty: Expr) -> Expr {
    app(cst("Finset"), elem_ty)
}
/// Build `Finset.empty` expression.
#[allow(dead_code)]
pub fn mk_finset_empty() -> Expr {
    cst("Finset.empty")
}
/// Build `Finset.insert a s` expression.
#[allow(dead_code)]
pub fn mk_finset_insert(a: Expr, s: Expr) -> Expr {
    app2(cst("Finset.insert"), a, s)
}
/// Build `Finset.card s` expression.
#[allow(dead_code)]
pub fn mk_finset_card(s: Expr) -> Expr {
    app(cst("Finset.card"), s)
}
/// Build `Finset.union s t` expression.
#[allow(dead_code)]
pub fn mk_finset_union(s: Expr, t: Expr) -> Expr {
    app2(cst("Finset.union"), s, t)
}
/// Build `Finset.inter s t` expression.
#[allow(dead_code)]
pub fn mk_finset_inter(s: Expr, t: Expr) -> Expr {
    app2(cst("Finset.inter"), s, t)
}
/// Build `Finset.toSet s` expression.
#[allow(dead_code)]
pub fn mk_finset_to_set(s: Expr) -> Expr {
    app(cst("Finset.toSet"), s)
}
/// Build `Finset.sum s f` expression.
#[allow(dead_code)]
pub fn mk_finset_sum(s: Expr, f: Expr) -> Expr {
    app2(cst("Finset.sum"), s, f)
}
