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
#[allow(dead_code)]
pub fn pi(bi: BinderInfo, name: &str, dom: Expr, body: Expr) -> Expr {
    Expr::Pi(bi, Name::str(name), Box::new(dom), Box::new(body))
}
#[allow(dead_code)]
pub fn lam(bi: BinderInfo, name: &str, dom: Expr, body: Expr) -> Expr {
    Expr::Lam(bi, Name::str(name), Box::new(dom), Box::new(body))
}
#[allow(dead_code)]
pub(super) fn cst(s: &str) -> Expr {
    Expr::Const(Name::str(s), vec![])
}
#[allow(dead_code)]
pub fn cst_u(s: &str, levels: Vec<Level>) -> Expr {
    Expr::Const(Name::str(s), levels)
}
#[allow(dead_code)]
pub(super) fn prop() -> Expr {
    Expr::Sort(Level::zero())
}
#[allow(dead_code)]
pub fn type1() -> Expr {
    Expr::Sort(Level::succ(Level::zero()))
}
#[allow(dead_code)]
pub fn sort_u() -> Expr {
    Expr::Sort(Level::Param(Name::str("u")))
}
#[allow(dead_code)]
pub fn sort_v() -> Expr {
    Expr::Sort(Level::Param(Name::str("v")))
}
#[allow(dead_code)]
pub fn bvar(n: u32) -> Expr {
    Expr::BVar(n)
}
#[allow(dead_code)]
pub fn level_u() -> Level {
    Level::Param(Name::str("u"))
}
#[allow(dead_code)]
pub fn level_v() -> Level {
    Level::Param(Name::str("v"))
}
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
/// Build the Sigma / Subtype / Exists / PLift / ULift environment.
#[allow(clippy::too_many_lines)]
pub fn build_sigma_env(env: &mut Environment) -> Result<(), String> {
    let sigma_ty = pi(
        BinderInfo::Implicit,
        "α",
        type1(),
        pi(
            BinderInfo::Default,
            "β",
            pi(BinderInfo::Default, "_", bvar(0), type1()),
            type1(),
        ),
    );
    add_axiom(env, "Sigma", vec![], sigma_ty)?;
    let sigma_mk_ty = pi(
        BinderInfo::Implicit,
        "α",
        type1(),
        pi(
            BinderInfo::Implicit,
            "β",
            pi(BinderInfo::Default, "_", bvar(0), type1()),
            pi(
                BinderInfo::Default,
                "fst",
                bvar(1),
                pi(
                    BinderInfo::Default,
                    "snd",
                    app(bvar(1), bvar(0)),
                    app(cst("Sigma"), bvar(2)),
                ),
            ),
        ),
    );
    add_axiom(env, "Sigma.mk", vec![], sigma_mk_ty)?;
    let sigma_fst_ty = pi(
        BinderInfo::Implicit,
        "α",
        type1(),
        pi(
            BinderInfo::Implicit,
            "β",
            pi(BinderInfo::Default, "_", bvar(0), type1()),
            pi(
                BinderInfo::Default,
                "s",
                app(cst("Sigma"), bvar(0)),
                bvar(2),
            ),
        ),
    );
    add_axiom(env, "Sigma.fst", vec![], sigma_fst_ty)?;
    let sigma_snd_ty = pi(
        BinderInfo::Implicit,
        "α",
        type1(),
        pi(
            BinderInfo::Implicit,
            "β",
            pi(BinderInfo::Default, "_", bvar(0), type1()),
            pi(
                BinderInfo::Default,
                "s",
                app(cst("Sigma"), bvar(0)),
                app(bvar(1), app(cst("Sigma.fst"), bvar(0))),
            ),
        ),
    );
    add_axiom(env, "Sigma.snd", vec![], sigma_snd_ty)?;
    let psigma_ty = pi(
        BinderInfo::Implicit,
        "α",
        sort_u(),
        pi(
            BinderInfo::Default,
            "β",
            pi(BinderInfo::Default, "_", bvar(0), sort_v()),
            Expr::Sort(Level::max(level_u(), level_v())),
        ),
    );
    add_axiom(
        env,
        "PSigma",
        vec![Name::str("u"), Name::str("v")],
        psigma_ty,
    )?;
    let psigma_mk_ty = pi(
        BinderInfo::Implicit,
        "α",
        sort_u(),
        pi(
            BinderInfo::Implicit,
            "β",
            pi(BinderInfo::Default, "_", bvar(0), sort_v()),
            pi(
                BinderInfo::Default,
                "fst",
                bvar(1),
                pi(
                    BinderInfo::Default,
                    "snd",
                    app(bvar(1), bvar(0)),
                    app(cst_u("PSigma", vec![level_u(), level_v()]), bvar(2)),
                ),
            ),
        ),
    );
    add_axiom(
        env,
        "PSigma.mk",
        vec![Name::str("u"), Name::str("v")],
        psigma_mk_ty,
    )?;
    let psigma_fst_ty = pi(
        BinderInfo::Implicit,
        "α",
        sort_u(),
        pi(
            BinderInfo::Implicit,
            "β",
            pi(BinderInfo::Default, "_", bvar(0), sort_v()),
            pi(
                BinderInfo::Default,
                "s",
                app(cst_u("PSigma", vec![level_u(), level_v()]), bvar(0)),
                bvar(2),
            ),
        ),
    );
    add_axiom(
        env,
        "PSigma.fst",
        vec![Name::str("u"), Name::str("v")],
        psigma_fst_ty,
    )?;
    let psigma_snd_ty = pi(
        BinderInfo::Implicit,
        "α",
        sort_u(),
        pi(
            BinderInfo::Implicit,
            "β",
            pi(BinderInfo::Default, "_", bvar(0), sort_v()),
            pi(
                BinderInfo::Default,
                "s",
                app(cst_u("PSigma", vec![level_u(), level_v()]), bvar(0)),
                app(
                    bvar(1),
                    app(cst_u("PSigma.fst", vec![level_u(), level_v()]), bvar(0)),
                ),
            ),
        ),
    );
    add_axiom(
        env,
        "PSigma.snd",
        vec![Name::str("u"), Name::str("v")],
        psigma_snd_ty,
    )?;
    let subtype_ty = pi(
        BinderInfo::Implicit,
        "α",
        type1(),
        pi(
            BinderInfo::Default,
            "p",
            pi(BinderInfo::Default, "_", bvar(0), prop()),
            type1(),
        ),
    );
    add_axiom(env, "Subtype", vec![], subtype_ty)?;
    let subtype_mk_ty = pi(
        BinderInfo::Implicit,
        "α",
        type1(),
        pi(
            BinderInfo::Implicit,
            "p",
            pi(BinderInfo::Default, "_", bvar(0), prop()),
            pi(
                BinderInfo::Default,
                "val",
                bvar(1),
                pi(
                    BinderInfo::Default,
                    "property",
                    app(bvar(1), bvar(0)),
                    app(cst("Subtype"), bvar(2)),
                ),
            ),
        ),
    );
    add_axiom(env, "Subtype.mk", vec![], subtype_mk_ty)?;
    let subtype_val_ty = pi(
        BinderInfo::Implicit,
        "α",
        type1(),
        pi(
            BinderInfo::Implicit,
            "p",
            pi(BinderInfo::Default, "_", bvar(0), prop()),
            pi(
                BinderInfo::Default,
                "s",
                app(cst("Subtype"), bvar(0)),
                bvar(2),
            ),
        ),
    );
    add_axiom(env, "Subtype.val", vec![], subtype_val_ty)?;
    let subtype_property_ty = pi(
        BinderInfo::Implicit,
        "α",
        type1(),
        pi(
            BinderInfo::Implicit,
            "p",
            pi(BinderInfo::Default, "_", bvar(0), prop()),
            pi(
                BinderInfo::Default,
                "s",
                app(cst("Subtype"), bvar(0)),
                app(bvar(1), app(cst("Subtype.val"), bvar(0))),
            ),
        ),
    );
    add_axiom(env, "Subtype.property", vec![], subtype_property_ty)?;
    let exists_ty = pi(
        BinderInfo::Implicit,
        "α",
        type1(),
        pi(
            BinderInfo::Default,
            "p",
            pi(BinderInfo::Default, "_", bvar(0), prop()),
            prop(),
        ),
    );
    add_axiom(env, "Sigma.Exists", vec![], exists_ty)?;
    let exists_intro_ty = pi(
        BinderInfo::Implicit,
        "α",
        type1(),
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
                    app(cst("Sigma.Exists"), bvar(2)),
                ),
            ),
        ),
    );
    add_axiom(env, "Sigma.Exists.intro", vec![], exists_intro_ty)?;
    let exists_elim_ty = pi(
        BinderInfo::Implicit,
        "α",
        type1(),
        pi(
            BinderInfo::Implicit,
            "p",
            pi(BinderInfo::Default, "_", bvar(0), prop()),
            pi(
                BinderInfo::Implicit,
                "b",
                prop(),
                pi(
                    BinderInfo::Default,
                    "h",
                    app(cst("Sigma.Exists"), bvar(1)),
                    pi(
                        BinderInfo::Default,
                        "f",
                        pi(
                            BinderInfo::Default,
                            "w",
                            bvar(3),
                            pi(BinderInfo::Default, "hw", app(bvar(3), bvar(0)), bvar(3)),
                        ),
                        bvar(2),
                    ),
                ),
            ),
        ),
    );
    add_axiom(env, "Sigma.Exists.elim", vec![], exists_elim_ty)?;
    let setof_ty = pi(
        BinderInfo::Implicit,
        "α",
        type1(),
        pi(
            BinderInfo::Default,
            "p",
            pi(BinderInfo::Default, "_", bvar(0), prop()),
            type1(),
        ),
    );
    add_axiom(env, "SetOf", vec![], setof_ty)?;
    let setof_mem_ty = pi(
        BinderInfo::Implicit,
        "α",
        type1(),
        pi(
            BinderInfo::Implicit,
            "p",
            pi(BinderInfo::Default, "_", bvar(0), prop()),
            pi(
                BinderInfo::Default,
                "a",
                bvar(1),
                pi(
                    BinderInfo::Default,
                    "_s",
                    app(cst("SetOf"), bvar(1)),
                    prop(),
                ),
            ),
        ),
    );
    add_axiom(env, "SetOf.mem", vec![], setof_mem_ty)?;
    let plift_ty = pi(BinderInfo::Default, "p", prop(), type1());
    add_axiom(env, "PLift", vec![], plift_ty)?;
    let plift_up_ty = pi(
        BinderInfo::Implicit,
        "p",
        prop(),
        pi(
            BinderInfo::Default,
            "h",
            bvar(0),
            app(cst("PLift"), bvar(1)),
        ),
    );
    add_axiom(env, "PLift.up", vec![], plift_up_ty)?;
    let plift_down_ty = pi(
        BinderInfo::Implicit,
        "p",
        prop(),
        pi(
            BinderInfo::Default,
            "h",
            app(cst("PLift"), bvar(0)),
            bvar(1),
        ),
    );
    add_axiom(env, "PLift.down", vec![], plift_down_ty)?;
    let ulift_ty = pi(
        BinderInfo::Default,
        "α",
        sort_u(),
        Expr::Sort(Level::max(level_u(), level_v())),
    );
    add_axiom(env, "ULift", vec![Name::str("u"), Name::str("v")], ulift_ty)?;
    let ulift_up_ty = pi(
        BinderInfo::Implicit,
        "α",
        sort_u(),
        pi(
            BinderInfo::Default,
            "a",
            bvar(0),
            app(cst_u("ULift", vec![level_u(), level_v()]), bvar(1)),
        ),
    );
    add_axiom(
        env,
        "ULift.up",
        vec![Name::str("u"), Name::str("v")],
        ulift_up_ty,
    )?;
    let ulift_down_ty = pi(
        BinderInfo::Implicit,
        "α",
        sort_u(),
        pi(
            BinderInfo::Default,
            "a",
            app(cst_u("ULift", vec![level_u(), level_v()]), bvar(0)),
            bvar(1),
        ),
    );
    add_axiom(
        env,
        "ULift.down",
        vec![Name::str("u"), Name::str("v")],
        ulift_down_ty,
    )?;
    let sigma_eta_ty = pi(
        BinderInfo::Implicit,
        "α",
        type1(),
        pi(
            BinderInfo::Implicit,
            "β",
            pi(BinderInfo::Default, "_", bvar(0), type1()),
            pi(
                BinderInfo::Default,
                "s",
                app(cst("Sigma"), bvar(0)),
                app3(
                    cst("Eq"),
                    app2(
                        cst("Sigma.mk"),
                        app(cst("Sigma.fst"), bvar(0)),
                        app(cst("Sigma.snd"), bvar(0)),
                    ),
                    bvar(0),
                    app(cst("Sigma"), bvar(1)),
                ),
            ),
        ),
    );
    add_axiom(env, "Sigma.eta", vec![], sigma_eta_ty)?;
    let subtype_eq_ty = pi(
        BinderInfo::Implicit,
        "α",
        type1(),
        pi(
            BinderInfo::Implicit,
            "p",
            pi(BinderInfo::Default, "_", bvar(0), prop()),
            pi(
                BinderInfo::Implicit,
                "a",
                app(cst("Subtype"), bvar(0)),
                pi(
                    BinderInfo::Implicit,
                    "b",
                    app(cst("Subtype"), bvar(1)),
                    pi(
                        BinderInfo::Default,
                        "h",
                        app3(
                            cst("Eq"),
                            bvar(3),
                            app(cst("Subtype.val"), bvar(1)),
                            app(cst("Subtype.val"), bvar(0)),
                        ),
                        app3(cst("Eq"), app(cst("Subtype"), bvar(3)), bvar(2), bvar(1)),
                    ),
                ),
            ),
        ),
    );
    add_axiom(env, "Subtype.eq", vec![], subtype_eq_ty)?;
    let exists_imp_ty = pi(
        BinderInfo::Implicit,
        "α",
        type1(),
        pi(
            BinderInfo::Implicit,
            "p",
            pi(BinderInfo::Default, "_", bvar(0), prop()),
            pi(
                BinderInfo::Implicit,
                "q",
                pi(BinderInfo::Default, "_", bvar(1), prop()),
                pi(
                    BinderInfo::Default,
                    "h",
                    pi(
                        BinderInfo::Default,
                        "a",
                        bvar(2),
                        pi(
                            BinderInfo::Default,
                            "_",
                            app(bvar(2), bvar(0)),
                            app(bvar(2), bvar(1)),
                        ),
                    ),
                    pi(
                        BinderInfo::Default,
                        "hex",
                        app(cst("Sigma.Exists"), bvar(2)),
                        app(cst("Sigma.Exists"), bvar(2)),
                    ),
                ),
            ),
        ),
    );
    add_axiom(env, "Sigma.Exists.imp", vec![], exists_imp_ty)?;
    let exists_not_forall_ty = pi(
        BinderInfo::Implicit,
        "α",
        type1(),
        pi(
            BinderInfo::Implicit,
            "p",
            pi(BinderInfo::Default, "_", bvar(0), prop()),
            pi(
                BinderInfo::Default,
                "h",
                app(
                    cst("Sigma.Exists"),
                    lam(
                        BinderInfo::Default,
                        "a",
                        bvar(1),
                        app(cst("Not"), app(bvar(1), bvar(0))),
                    ),
                ),
                pi(
                    BinderInfo::Default,
                    "_",
                    pi(BinderInfo::Default, "a", bvar(2), app(bvar(2), bvar(0))),
                    cst("False"),
                ),
            ),
        ),
    );
    add_axiom(env, "Sigma.Exists.not_forall", vec![], exists_not_forall_ty)?;
    let sigma_cases_ty = pi(
        BinderInfo::Implicit,
        "α",
        type1(),
        pi(
            BinderInfo::Implicit,
            "β",
            pi(BinderInfo::Default, "_", bvar(0), type1()),
            pi(
                BinderInfo::Implicit,
                "motive",
                pi(
                    BinderInfo::Default,
                    "_",
                    app(cst("Sigma"), bvar(0)),
                    sort_u(),
                ),
                pi(
                    BinderInfo::Default,
                    "s",
                    app(cst("Sigma"), bvar(1)),
                    pi(
                        BinderInfo::Default,
                        "f",
                        pi(
                            BinderInfo::Default,
                            "fst",
                            bvar(3),
                            pi(
                                BinderInfo::Default,
                                "snd",
                                app(bvar(3), bvar(0)),
                                app(bvar(3), app2(cst("Sigma.mk"), bvar(1), bvar(0))),
                            ),
                        ),
                        app(bvar(2), bvar(1)),
                    ),
                ),
            ),
        ),
    );
    add_axiom(env, "Sigma.casesOn", vec![Name::str("u")], sigma_cases_ty)?;
    let subtype_cases_ty = pi(
        BinderInfo::Implicit,
        "α",
        type1(),
        pi(
            BinderInfo::Implicit,
            "p",
            pi(BinderInfo::Default, "_", bvar(0), prop()),
            pi(
                BinderInfo::Implicit,
                "motive",
                pi(
                    BinderInfo::Default,
                    "_",
                    app(cst("Subtype"), bvar(0)),
                    sort_u(),
                ),
                pi(
                    BinderInfo::Default,
                    "s",
                    app(cst("Subtype"), bvar(1)),
                    pi(
                        BinderInfo::Default,
                        "f",
                        pi(
                            BinderInfo::Default,
                            "val",
                            bvar(3),
                            pi(
                                BinderInfo::Default,
                                "property",
                                app(bvar(3), bvar(0)),
                                app(bvar(3), app2(cst("Subtype.mk"), bvar(1), bvar(0))),
                            ),
                        ),
                        app(bvar(2), bvar(1)),
                    ),
                ),
            ),
        ),
    );
    add_axiom(
        env,
        "Subtype.casesOn",
        vec![Name::str("u")],
        subtype_cases_ty,
    )?;
    let plift_cases_ty = pi(
        BinderInfo::Implicit,
        "p",
        prop(),
        pi(
            BinderInfo::Implicit,
            "motive",
            pi(
                BinderInfo::Default,
                "_",
                app(cst("PLift"), bvar(0)),
                sort_u(),
            ),
            pi(
                BinderInfo::Default,
                "s",
                app(cst("PLift"), bvar(1)),
                pi(
                    BinderInfo::Default,
                    "f",
                    pi(
                        BinderInfo::Default,
                        "h",
                        bvar(2),
                        app(bvar(2), app(cst("PLift.up"), bvar(0))),
                    ),
                    app(bvar(2), bvar(1)),
                ),
            ),
        ),
    );
    add_axiom(env, "PLift.casesOn", vec![Name::str("u")], plift_cases_ty)?;
    let sigma_ext_ty = pi(
        BinderInfo::Implicit,
        "α",
        type1(),
        pi(
            BinderInfo::Implicit,
            "β",
            pi(BinderInfo::Default, "_", bvar(0), type1()),
            pi(
                BinderInfo::Implicit,
                "s",
                app(cst("Sigma"), bvar(0)),
                pi(
                    BinderInfo::Implicit,
                    "t",
                    app(cst("Sigma"), bvar(1)),
                    pi(
                        BinderInfo::Default,
                        "h1",
                        app3(
                            cst("Eq"),
                            bvar(3),
                            app(cst("Sigma.fst"), bvar(1)),
                            app(cst("Sigma.fst"), bvar(0)),
                        ),
                        pi(
                            BinderInfo::Default,
                            "h2",
                            app4(
                                cst("HEq"),
                                app(bvar(4), app(cst("Sigma.fst"), bvar(2))),
                                app(cst("Sigma.snd"), bvar(2)),
                                app(bvar(4), app(cst("Sigma.fst"), bvar(1))),
                                app(cst("Sigma.snd"), bvar(1)),
                            ),
                            app3(cst("Eq"), app(cst("Sigma"), bvar(4)), bvar(3), bvar(2)),
                        ),
                    ),
                ),
            ),
        ),
    );
    add_axiom(env, "Sigma.ext", vec![], sigma_ext_ty)?;
    Ok(())
}
/// Build `Sigma β` expression given alpha and beta.
#[allow(dead_code)]
pub fn mk_sigma(alpha: Expr, beta: Expr) -> Expr {
    app2(cst("Sigma"), alpha, beta)
}
/// Build `Sigma.mk fst snd`.
#[allow(dead_code)]
pub fn mk_sigma_mk(fst: Expr, snd: Expr) -> Expr {
    app2(cst("Sigma.mk"), fst, snd)
}
/// Build `Sigma.fst pair`.
#[allow(dead_code)]
pub fn mk_sigma_fst(pair: Expr) -> Expr {
    app(cst("Sigma.fst"), pair)
}
/// Build `Sigma.snd pair`.
#[allow(dead_code)]
pub fn mk_sigma_snd(pair: Expr) -> Expr {
    app(cst("Sigma.snd"), pair)
}
/// Build `PSigma β` with universe levels.
#[allow(dead_code)]
pub fn mk_psigma(alpha: Expr, beta: Expr) -> Expr {
    app2(cst("PSigma"), alpha, beta)
}
/// Build `PSigma.mk fst snd`.
#[allow(dead_code)]
pub fn mk_psigma_mk(fst: Expr, snd: Expr) -> Expr {
    app2(cst("PSigma.mk"), fst, snd)
}
/// Build `PSigma.fst pair`.
#[allow(dead_code)]
pub fn mk_psigma_fst(pair: Expr) -> Expr {
    app(cst("PSigma.fst"), pair)
}
/// Build `Subtype p` given alpha and predicate.
#[allow(dead_code)]
pub fn mk_subtype(alpha: Expr, pred: Expr) -> Expr {
    app2(cst("Subtype"), alpha, pred)
}
/// Build `Subtype.mk val proof`.
#[allow(dead_code)]
pub fn mk_subtype_mk(val: Expr, proof: Expr) -> Expr {
    app2(cst("Subtype.mk"), val, proof)
}
/// Build `Subtype.val s`.
#[allow(dead_code)]
pub fn mk_subtype_val(s: Expr) -> Expr {
    app(cst("Subtype.val"), s)
}
/// Build `Sigma.Exists p` given alpha and predicate.
#[allow(dead_code)]
pub fn mk_exists(alpha: Expr, pred: Expr) -> Expr {
    app2(cst("Sigma.Exists"), alpha, pred)
}
/// Build `Sigma.Exists.intro witness proof`.
#[allow(dead_code)]
pub fn mk_exists_intro(witness: Expr, proof: Expr) -> Expr {
    app2(cst("Sigma.Exists.intro"), witness, proof)
}
/// Build `Sigma.Exists.elim hex f`.
#[allow(dead_code)]
pub fn mk_exists_elim(hex: Expr, f: Expr) -> Expr {
    app2(cst("Sigma.Exists.elim"), hex, f)
}
/// Build `SetOf p` given alpha and predicate.
#[allow(dead_code)]
pub fn mk_setof(alpha: Expr, pred: Expr) -> Expr {
    app2(cst("SetOf"), alpha, pred)
}
/// Build `PLift p`.
#[allow(dead_code)]
pub fn mk_plift(p: Expr) -> Expr {
    app(cst("PLift"), p)
}
/// Build `PLift.up proof`.
#[allow(dead_code)]
pub fn mk_plift_up(proof: Expr) -> Expr {
    app(cst("PLift.up"), proof)
}
/// Build `PLift.down h`.
#[allow(dead_code)]
pub fn mk_plift_down(h: Expr) -> Expr {
    app(cst("PLift.down"), h)
}
/// Build `ULift α`.
#[allow(dead_code)]
pub fn mk_ulift(alpha: Expr) -> Expr {
    app(cst("ULift"), alpha)
}
/// Build `ULift.up a`.
#[allow(dead_code)]
pub fn mk_ulift_up(a: Expr) -> Expr {
    app(cst("ULift.up"), a)
}
/// Build `ULift.down a`.
#[allow(dead_code)]
pub fn mk_ulift_down(a: Expr) -> Expr {
    app(cst("ULift.down"), a)
}
#[allow(dead_code)]
pub fn sigma_eta_exp_ty() -> Expr {
    pi(
        BinderInfo::Implicit,
        "α",
        type1(),
        pi(
            BinderInfo::Implicit,
            "β",
            pi(BinderInfo::Default, "_", bvar(0), type1()),
            pi(
                BinderInfo::Default,
                "s",
                app(cst("Sigma"), bvar(0)),
                app3(
                    cst("Eq"),
                    app(cst("Sigma"), bvar(1)),
                    app2(
                        cst("Sigma.mk"),
                        app(cst("Sigma.fst"), bvar(0)),
                        app(cst("Sigma.snd"), bvar(0)),
                    ),
                    bvar(0),
                ),
            ),
        ),
    )
}
#[allow(dead_code)]
pub fn sigma_surj_ty() -> Expr {
    pi(
        BinderInfo::Implicit,
        "α",
        type1(),
        pi(
            BinderInfo::Implicit,
            "β",
            pi(BinderInfo::Default, "_", bvar(0), type1()),
            pi(
                BinderInfo::Default,
                "s",
                app(cst("Sigma"), bvar(0)),
                app(
                    cst("Sigma.Exists"),
                    lam(
                        BinderInfo::Default,
                        "a",
                        bvar(2),
                        app(
                            cst("Sigma.Exists"),
                            lam(
                                BinderInfo::Default,
                                "b",
                                app(bvar(2), bvar(0)),
                                app3(
                                    cst("Eq"),
                                    app(cst("Sigma"), bvar(3)),
                                    app2(cst("Sigma.mk"), bvar(1), bvar(0)),
                                    bvar(2),
                                ),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    )
}
#[allow(dead_code)]
pub fn telescope_ty() -> Expr {
    pi(BinderInfo::Default, "_", type1(), type1())
}
#[allow(dead_code)]
pub fn telescope_nil_ty() -> Expr {
    pi(
        BinderInfo::Implicit,
        "α",
        type1(),
        app(cst("Telescope"), bvar(0)),
    )
}
#[allow(dead_code)]
pub fn telescope_cons_ty() -> Expr {
    pi(
        BinderInfo::Implicit,
        "α",
        type1(),
        pi(
            BinderInfo::Implicit,
            "β",
            pi(BinderInfo::Default, "_", bvar(0), type1()),
            pi(
                BinderInfo::Default,
                "_",
                app(cst("Telescope"), app(cst("Sigma"), bvar(0))),
                app(cst("Telescope"), bvar(2)),
            ),
        ),
    )
}
#[allow(dead_code)]
pub fn sigma_update_fst_ty() -> Expr {
    pi(
        BinderInfo::Implicit,
        "α",
        type1(),
        pi(
            BinderInfo::Implicit,
            "β",
            pi(BinderInfo::Default, "_", bvar(0), type1()),
            pi(
                BinderInfo::Default,
                "_s",
                app(cst("Sigma"), bvar(0)),
                pi(
                    BinderInfo::Default,
                    "a_prime",
                    bvar(2),
                    pi(
                        BinderInfo::Default,
                        "_b",
                        app(bvar(2), bvar(0)),
                        app(cst("Sigma"), bvar(3)),
                    ),
                ),
            ),
        ),
    )
}
#[allow(dead_code)]
pub fn quot_sigma_ty() -> Expr {
    pi(
        BinderInfo::Implicit,
        "α",
        type1(),
        pi(
            BinderInfo::Default,
            "_p",
            pi(BinderInfo::Default, "_", bvar(0), prop()),
            pi(
                BinderInfo::Default,
                "_r",
                pi(
                    BinderInfo::Default,
                    "_",
                    bvar(1),
                    pi(BinderInfo::Default, "_", bvar(1), prop()),
                ),
                type1(),
            ),
        ),
    )
}
#[allow(dead_code)]
pub fn exist_pack_ty() -> Expr {
    pi(
        BinderInfo::Implicit,
        "α",
        type1(),
        pi(
            BinderInfo::Default,
            "_β",
            pi(BinderInfo::Default, "_", bvar(0), type1()),
            type1(),
        ),
    )
}
#[allow(dead_code)]
pub fn exist_pack_pack_ty() -> Expr {
    pi(
        BinderInfo::Implicit,
        "α",
        type1(),
        pi(
            BinderInfo::Implicit,
            "β",
            pi(BinderInfo::Default, "_", bvar(0), type1()),
            pi(
                BinderInfo::Default,
                "a",
                bvar(1),
                pi(
                    BinderInfo::Default,
                    "_b",
                    app(bvar(1), bvar(0)),
                    app(cst("ExistPack"), bvar(2)),
                ),
            ),
        ),
    )
}
#[allow(dead_code)]
pub fn exist_pack_unpack_ty() -> Expr {
    pi(
        BinderInfo::Implicit,
        "α",
        type1(),
        pi(
            BinderInfo::Implicit,
            "β",
            pi(BinderInfo::Default, "_", bvar(0), type1()),
            pi(
                BinderInfo::Implicit,
                "γ",
                type1(),
                pi(
                    BinderInfo::Default,
                    "_ep",
                    app(cst("ExistPack"), bvar(1)),
                    pi(
                        BinderInfo::Default,
                        "_f",
                        pi(
                            BinderInfo::Default,
                            "a",
                            bvar(3),
                            pi(BinderInfo::Default, "_", app(bvar(3), bvar(0)), bvar(3)),
                        ),
                        bvar(2),
                    ),
                ),
            ),
        ),
    )
}
#[allow(dead_code)]
pub fn module_seal_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "_sig",
        type1(),
        pi(BinderInfo::Default, "_impl", type1(), type1()),
    )
}
#[allow(dead_code)]
pub fn sigma_path_space_ty() -> Expr {
    pi(
        BinderInfo::Implicit,
        "α",
        type1(),
        pi(
            BinderInfo::Implicit,
            "β",
            pi(BinderInfo::Default, "_", bvar(0), type1()),
            pi(
                BinderInfo::Default,
                "_s",
                app(cst("Sigma"), bvar(0)),
                pi(
                    BinderInfo::Default,
                    "_t",
                    app(cst("Sigma"), bvar(1)),
                    type1(),
                ),
            ),
        ),
    )
}
#[allow(dead_code)]
pub fn sigma_fibration_ty() -> Expr {
    pi(
        BinderInfo::Implicit,
        "α",
        type1(),
        pi(
            BinderInfo::Default,
            "_β",
            pi(BinderInfo::Default, "_", bvar(0), type1()),
            pi(BinderInfo::Default, "_total", type1(), type1()),
        ),
    )
}
#[allow(dead_code)]
pub fn wtype_ty() -> Expr {
    pi(
        BinderInfo::Implicit,
        "α",
        type1(),
        pi(
            BinderInfo::Default,
            "_β",
            pi(BinderInfo::Default, "_", bvar(0), type1()),
            type1(),
        ),
    )
}
#[allow(dead_code)]
pub fn wtype_sup_ty() -> Expr {
    pi(
        BinderInfo::Implicit,
        "α",
        type1(),
        pi(
            BinderInfo::Implicit,
            "β",
            pi(BinderInfo::Default, "_", bvar(0), type1()),
            pi(
                BinderInfo::Default,
                "a",
                bvar(1),
                pi(
                    BinderInfo::Default,
                    "_f",
                    pi(
                        BinderInfo::Default,
                        "_",
                        app(bvar(1), bvar(0)),
                        app(cst("WType"), bvar(2)),
                    ),
                    app(cst("WType"), bvar(2)),
                ),
            ),
        ),
    )
}
#[allow(dead_code)]
pub fn wtype_to_sigma_ty() -> Expr {
    pi(
        BinderInfo::Implicit,
        "α",
        type1(),
        pi(
            BinderInfo::Implicit,
            "β",
            pi(BinderInfo::Default, "_", bvar(0), type1()),
            pi(
                BinderInfo::Default,
                "_w",
                app(cst("WType"), bvar(0)),
                app(cst("Sigma"), bvar(1)),
            ),
        ),
    )
}
#[allow(dead_code)]
pub fn indrec_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "_D",
        type1(),
        pi(
            BinderInfo::Default,
            "_T",
            pi(BinderInfo::Default, "_", bvar(0), type1()),
            type1(),
        ),
    )
}
#[allow(dead_code)]
pub fn refine_ty() -> Expr {
    pi(
        BinderInfo::Implicit,
        "α",
        type1(),
        pi(
            BinderInfo::Default,
            "_p",
            pi(BinderInfo::Default, "_", bvar(0), prop()),
            type1(),
        ),
    )
}
#[allow(dead_code)]
pub fn refine_mk_ty() -> Expr {
    pi(
        BinderInfo::Implicit,
        "α",
        type1(),
        pi(
            BinderInfo::Implicit,
            "p",
            pi(BinderInfo::Default, "_", bvar(0), prop()),
            pi(
                BinderInfo::Default,
                "a",
                bvar(1),
                pi(
                    BinderInfo::Default,
                    "_h",
                    app(bvar(1), bvar(0)),
                    app(cst("Refine"), bvar(2)),
                ),
            ),
        ),
    )
}
#[allow(dead_code)]
pub fn refine_val_ty() -> Expr {
    pi(
        BinderInfo::Implicit,
        "α",
        type1(),
        pi(
            BinderInfo::Implicit,
            "p",
            pi(BinderInfo::Default, "_", bvar(0), prop()),
            pi(
                BinderInfo::Default,
                "_r",
                app(cst("Refine"), bvar(0)),
                bvar(2),
            ),
        ),
    )
}
#[allow(dead_code)]
pub fn sigma_u_ty() -> Expr {
    pi(
        BinderInfo::Implicit,
        "α",
        sort_u(),
        pi(
            BinderInfo::Default,
            "_β",
            pi(BinderInfo::Default, "_", bvar(0), sort_v()),
            Expr::Sort(Level::imax(level_u(), level_v())),
        ),
    )
}
#[allow(dead_code)]
pub fn row_type_ty() -> Expr {
    pi(BinderInfo::Default, "_", type1(), type1())
}
#[allow(dead_code)]
pub fn row_ext_ty() -> Expr {
    pi(
        BinderInfo::Implicit,
        "lbl",
        type1(),
        pi(
            BinderInfo::Default,
            "_l",
            bvar(0),
            pi(
                BinderInfo::Default,
                "_t",
                type1(),
                pi(
                    BinderInfo::Default,
                    "_row",
                    app(cst("RowType"), bvar(2)),
                    app(cst("RowType"), bvar(3)),
                ),
            ),
        ),
    )
}
#[allow(dead_code)]
pub fn row_sub_ty() -> Expr {
    pi(
        BinderInfo::Implicit,
        "lbl",
        type1(),
        pi(
            BinderInfo::Default,
            "_r1",
            app(cst("RowType"), bvar(0)),
            pi(
                BinderInfo::Default,
                "_r2",
                app(cst("RowType"), bvar(1)),
                prop(),
            ),
        ),
    )
}
#[allow(dead_code)]
pub fn dep_sum_ty() -> Expr {
    pi(
        BinderInfo::Implicit,
        "D",
        type1(),
        pi(
            BinderInfo::Default,
            "_f",
            pi(BinderInfo::Default, "_", bvar(0), type1()),
            type1(),
        ),
    )
}
#[allow(dead_code)]
pub fn dep_sum_le_ty() -> Expr {
    pi(
        BinderInfo::Implicit,
        "D",
        type1(),
        pi(
            BinderInfo::Implicit,
            "f",
            pi(BinderInfo::Default, "_", bvar(0), type1()),
            pi(
                BinderInfo::Default,
                "_x",
                app(cst("DepSum"), bvar(0)),
                pi(
                    BinderInfo::Default,
                    "_y",
                    app(cst("DepSum"), bvar(1)),
                    prop(),
                ),
            ),
        ),
    )
}
#[allow(dead_code)]
pub fn sigma_return_ty() -> Expr {
    pi(
        BinderInfo::Implicit,
        "α",
        type1(),
        pi(
            BinderInfo::Default,
            "_a",
            bvar(0),
            app(
                cst("Sigma"),
                lam(BinderInfo::Default, "_", type1(), bvar(1)),
            ),
        ),
    )
}
#[allow(dead_code)]
pub fn sigma_bind_ty() -> Expr {
    pi(
        BinderInfo::Implicit,
        "α",
        type1(),
        pi(
            BinderInfo::Implicit,
            "β",
            type1(),
            pi(
                BinderInfo::Default,
                "_sa",
                app(
                    cst("Sigma"),
                    lam(BinderInfo::Default, "_", type1(), bvar(1)),
                ),
                pi(
                    BinderInfo::Default,
                    "_f",
                    pi(
                        BinderInfo::Default,
                        "_",
                        bvar(2),
                        app(
                            cst("Sigma"),
                            lam(BinderInfo::Default, "_", type1(), bvar(2)),
                        ),
                    ),
                    app(
                        cst("Sigma"),
                        lam(BinderInfo::Default, "_", type1(), bvar(2)),
                    ),
                ),
            ),
        ),
    )
}
#[allow(dead_code)]
pub fn sigma_total_space_ty() -> Expr {
    pi(
        BinderInfo::Implicit,
        "α",
        type1(),
        pi(
            BinderInfo::Default,
            "_β",
            pi(BinderInfo::Default, "_", bvar(0), type1()),
            type1(),
        ),
    )
}
#[allow(dead_code)]
pub fn sigma_base_proj_ty() -> Expr {
    pi(
        BinderInfo::Implicit,
        "α",
        type1(),
        pi(
            BinderInfo::Implicit,
            "β",
            pi(BinderInfo::Default, "_", bvar(0), type1()),
            pi(
                BinderInfo::Default,
                "_ts",
                app(cst("Sigma.total_space"), bvar(0)),
                bvar(2),
            ),
        ),
    )
}
#[allow(dead_code)]
pub fn large_sigma_ty() -> Expr {
    pi(
        BinderInfo::Implicit,
        "α",
        Expr::Sort(Level::succ(Level::succ(Level::zero()))),
        pi(
            BinderInfo::Default,
            "_β",
            pi(
                BinderInfo::Default,
                "_",
                bvar(0),
                Expr::Sort(Level::succ(Level::succ(Level::zero()))),
            ),
            Expr::Sort(Level::succ(Level::succ(Level::zero()))),
        ),
    )
}
#[allow(dead_code)]
pub fn sigma_small_pred_ty() -> Expr {
    pi(
        BinderInfo::Implicit,
        "α",
        type1(),
        pi(
            BinderInfo::Implicit,
            "β",
            pi(BinderInfo::Default, "_", bvar(0), prop()),
            pi(
                BinderInfo::Default,
                "_s",
                app(cst("Sigma"), bvar(0)),
                bvar(2),
            ),
        ),
    )
}
#[allow(dead_code)]
pub fn sigma_map_fst_ty() -> Expr {
    pi(
        BinderInfo::Implicit,
        "α",
        type1(),
        pi(
            BinderInfo::Implicit,
            "β",
            type1(),
            pi(
                BinderInfo::Default,
                "_f",
                pi(BinderInfo::Default, "_", bvar(1), bvar(1)),
                pi(
                    BinderInfo::Default,
                    "_sa",
                    app(
                        cst("Sigma"),
                        lam(BinderInfo::Default, "_", type1(), bvar(2)),
                    ),
                    app(
                        cst("Sigma"),
                        lam(BinderInfo::Default, "_", type1(), bvar(2)),
                    ),
                ),
            ),
        ),
    )
}
#[allow(dead_code)]
pub fn sigma_choice_ty() -> Expr {
    pi(
        BinderInfo::Implicit,
        "α",
        type1(),
        pi(
            BinderInfo::Implicit,
            "β",
            pi(BinderInfo::Default, "_", bvar(0), type1()),
            pi(
                BinderInfo::Default,
                "_h",
                pi(BinderInfo::Default, "a", bvar(1), app(bvar(1), bvar(0))),
                app(cst("Sigma"), bvar(1)),
            ),
        ),
    )
}
#[allow(dead_code)]
pub fn sigma_prod_ty() -> Expr {
    pi(
        BinderInfo::Implicit,
        "α",
        type1(),
        pi(
            BinderInfo::Implicit,
            "β",
            type1(),
            pi(
                BinderInfo::Default,
                "_s",
                app(
                    cst("Sigma"),
                    lam(BinderInfo::Default, "_", type1(), bvar(1)),
                ),
                app2(cst("Prod"), bvar(2), bvar(1)),
            ),
        ),
    )
}
#[allow(dead_code)]
pub fn sigma_swap_ty() -> Expr {
    pi(
        BinderInfo::Implicit,
        "α",
        type1(),
        pi(
            BinderInfo::Implicit,
            "β",
            type1(),
            pi(
                BinderInfo::Default,
                "_s",
                app(
                    cst("Sigma"),
                    lam(BinderInfo::Default, "_", type1(), bvar(1)),
                ),
                app(
                    cst("Sigma"),
                    lam(BinderInfo::Default, "_", type1(), bvar(2)),
                ),
            ),
        ),
    )
}
#[allow(dead_code)]
pub fn sigma_assoc_ty() -> Expr {
    pi(
        BinderInfo::Implicit,
        "α",
        type1(),
        pi(
            BinderInfo::Implicit,
            "β",
            pi(BinderInfo::Default, "_", bvar(0), type1()),
            pi(
                BinderInfo::Implicit,
                "γ",
                pi(
                    BinderInfo::Default,
                    "_",
                    app(cst("Sigma"), bvar(0)),
                    type1(),
                ),
                pi(
                    BinderInfo::Default,
                    "_s",
                    app(cst("Sigma"), bvar(0)),
                    app(
                        cst("Sigma"),
                        lam(
                            BinderInfo::Default,
                            "a",
                            bvar(3),
                            app(
                                cst("Sigma"),
                                lam(
                                    BinderInfo::Default,
                                    "b",
                                    app(bvar(3), bvar(0)),
                                    app(bvar(3), app2(cst("Sigma.mk"), bvar(1), bvar(0))),
                                ),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    )
}
#[allow(dead_code)]
pub fn subtype_coerce_ty() -> Expr {
    pi(
        BinderInfo::Implicit,
        "α",
        type1(),
        pi(
            BinderInfo::Implicit,
            "p",
            pi(BinderInfo::Default, "_", bvar(0), prop()),
            pi(
                BinderInfo::Implicit,
                "q",
                pi(BinderInfo::Default, "_", bvar(1), prop()),
                pi(
                    BinderInfo::Default,
                    "_h",
                    pi(
                        BinderInfo::Default,
                        "a",
                        bvar(2),
                        pi(
                            BinderInfo::Default,
                            "_",
                            app(bvar(2), bvar(0)),
                            app(bvar(2), bvar(1)),
                        ),
                    ),
                    pi(
                        BinderInfo::Default,
                        "_s",
                        app(cst("Subtype"), bvar(2)),
                        app(cst("Subtype"), bvar(2)),
                    ),
                ),
            ),
        ),
    )
}
#[allow(dead_code)]
pub fn subtype_inter_ty() -> Expr {
    pi(
        BinderInfo::Implicit,
        "α",
        type1(),
        pi(
            BinderInfo::Default,
            "_p",
            pi(BinderInfo::Default, "_", bvar(0), prop()),
            pi(
                BinderInfo::Default,
                "_q",
                pi(BinderInfo::Default, "_", bvar(1), prop()),
                type1(),
            ),
        ),
    )
}
/// Register advanced Sigma-type axioms in the environment.
#[allow(clippy::too_many_lines)]
pub fn build_sigma_advanced_env(env: &mut Environment) -> Result<(), String> {
    build_sigma_env(env)?;
    add_axiom(env, "Sigma.eta_exp", vec![], sigma_eta_exp_ty())?;
    add_axiom(env, "Sigma.surj", vec![], sigma_surj_ty())?;
    add_axiom(env, "Telescope", vec![], telescope_ty())?;
    add_axiom(env, "Telescope.nil", vec![], telescope_nil_ty())?;
    add_axiom(env, "Telescope.cons", vec![], telescope_cons_ty())?;
    add_axiom(env, "Sigma.update_fst", vec![], sigma_update_fst_ty())?;
    add_axiom(env, "QuotSigma", vec![], quot_sigma_ty())?;
    add_axiom(env, "ExistPack", vec![], exist_pack_ty())?;
    add_axiom(env, "ExistPack.pack", vec![], exist_pack_pack_ty())?;
    add_axiom(env, "ExistPack.unpack", vec![], exist_pack_unpack_ty())?;
    add_axiom(env, "ModuleSeal", vec![], module_seal_ty())?;
    add_axiom(env, "Sigma.PathSpace", vec![], sigma_path_space_ty())?;
    add_axiom(env, "Sigma.Fibration", vec![], sigma_fibration_ty())?;
    add_axiom(env, "WType", vec![], wtype_ty())?;
    add_axiom(env, "WType.sup", vec![], wtype_sup_ty())?;
    add_axiom(env, "WType.to_sigma", vec![], wtype_to_sigma_ty())?;
    add_axiom(env, "IndRec", vec![], indrec_ty())?;
    add_axiom(env, "Refine", vec![], refine_ty())?;
    add_axiom(env, "Refine.mk", vec![], refine_mk_ty())?;
    add_axiom(env, "Refine.val", vec![], refine_val_ty())?;
    add_axiom(
        env,
        "SigmaU",
        vec![Name::str("u"), Name::str("v")],
        sigma_u_ty(),
    )?;
    add_axiom(env, "RowType", vec![], row_type_ty())?;
    add_axiom(env, "RowExt", vec![], row_ext_ty())?;
    add_axiom(env, "RowSub", vec![], row_sub_ty())?;
    add_axiom(env, "DepSum", vec![], dep_sum_ty())?;
    add_axiom(env, "DepSum.le", vec![], dep_sum_le_ty())?;
    add_axiom(env, "Sigma.return", vec![], sigma_return_ty())?;
    add_axiom(env, "Sigma.bind", vec![], sigma_bind_ty())?;
    add_axiom(env, "Sigma.total_space", vec![], sigma_total_space_ty())?;
    add_axiom(env, "Sigma.base_proj", vec![], sigma_base_proj_ty())?;
    add_axiom(env, "LargeSigma", vec![], large_sigma_ty())?;
    add_axiom(env, "Sigma.small_pred", vec![], sigma_small_pred_ty())?;
    add_axiom(env, "Sigma.map_fst", vec![], sigma_map_fst_ty())?;
    add_axiom(env, "Sigma.choice", vec![], sigma_choice_ty())?;
    add_axiom(env, "Sigma.prod_sigma", vec![], sigma_prod_ty())?;
    add_axiom(env, "Sigma.swap", vec![], sigma_swap_ty())?;
    add_axiom(env, "Sigma.assoc", vec![], sigma_assoc_ty())?;
    add_axiom(env, "Subtype.coerce", vec![], subtype_coerce_ty())?;
    add_axiom(env, "Subtype.inter", vec![], subtype_inter_ty())?;
    Ok(())
}
