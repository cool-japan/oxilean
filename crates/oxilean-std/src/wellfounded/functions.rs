//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

pub(super) fn prop() -> Expr {
    Expr::Sort(Level::zero())
}
pub(super) fn type1() -> Expr {
    Expr::Sort(Level::succ(Level::zero()))
}
pub(super) fn arrow(a: Expr, b: Expr) -> Expr {
    Expr::Pi(
        BinderInfo::Default,
        Name::Anonymous,
        Box::new(a),
        Box::new(b),
    )
}
pub fn nat_const() -> Expr {
    Expr::Const(Name::str("Nat"), vec![])
}
pub(super) fn sorry() -> Expr {
    Expr::Const(Name::str("sorry"), vec![])
}
/// Universe parameter `u`.
pub fn u_param() -> Name {
    Name::str("u")
}
/// Universe parameter `v`.
pub fn v_param() -> Name {
    Name::str("v")
}
pub(super) fn sort_u() -> Expr {
    Expr::Sort(Level::Param(u_param()))
}
pub fn sort_v() -> Expr {
    Expr::Sort(Level::Param(v_param()))
}
pub fn rel_ty(alpha_bvar: u32) -> Expr {
    arrow(
        Expr::BVar(alpha_bvar),
        arrow(Expr::BVar(alpha_bvar + 1), prop()),
    )
}
/// Create `WellFounded rel`.
#[allow(dead_code)]
pub fn mk_wellfounded(rel: Expr) -> Expr {
    Expr::App(
        Box::new(Expr::Const(Name::str("WellFounded"), vec![])),
        Box::new(rel),
    )
}
/// Create `Acc rel x`.
#[allow(dead_code)]
pub fn mk_acc(rel: Expr, x: Expr) -> Expr {
    Expr::App(
        Box::new(Expr::App(
            Box::new(Expr::Const(Name::str("Acc"), vec![])),
            Box::new(rel),
        )),
        Box::new(x),
    )
}
/// Create `Acc.intro x h`.
#[allow(dead_code)]
pub fn mk_acc_intro(x: Expr, h: Expr) -> Expr {
    Expr::App(
        Box::new(Expr::App(
            Box::new(Expr::Const(Name::str("Acc.intro"), vec![])),
            Box::new(x),
        )),
        Box::new(h),
    )
}
/// Create `WellFounded.fix wf f a`.
#[allow(dead_code)]
pub fn mk_wf_fix(wf: Expr, f: Expr, a: Expr) -> Expr {
    Expr::App(
        Box::new(Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("WellFounded.fix"), vec![])),
                Box::new(wf),
            )),
            Box::new(f),
        )),
        Box::new(a),
    )
}
/// Create `Measure f`.
#[allow(dead_code)]
pub fn mk_measure(f: Expr) -> Expr {
    Expr::App(
        Box::new(Expr::Const(Name::str("Measure"), vec![])),
        Box::new(f),
    )
}
/// Create `InvImage r f`.
#[allow(dead_code)]
pub fn mk_inv_image(r: Expr, f: Expr) -> Expr {
    Expr::App(
        Box::new(Expr::App(
            Box::new(Expr::Const(Name::str("InvImage"), vec![])),
            Box::new(r),
        )),
        Box::new(f),
    )
}
/// Create `Prod.Lex ra rb`.
#[allow(dead_code)]
pub fn mk_prod_lex(ra: Expr, rb: Expr) -> Expr {
    Expr::App(
        Box::new(Expr::App(
            Box::new(Expr::Const(Name::str("Prod.Lex"), vec![])),
            Box::new(ra),
        )),
        Box::new(rb),
    )
}
/// Create `@sizeOf ty a`.
#[allow(dead_code)]
pub fn mk_sizeof(ty: Expr, a: Expr) -> Expr {
    Expr::App(
        Box::new(Expr::App(
            Box::new(Expr::Const(Name::str("sizeOf"), vec![])),
            Box::new(ty),
        )),
        Box::new(a),
    )
}
/// Create `PSigma beta`.
#[allow(dead_code)]
pub fn mk_psigma(alpha: Expr, beta: Expr) -> Expr {
    Expr::App(
        Box::new(Expr::App(
            Box::new(Expr::Const(Name::str("PSigma"), vec![])),
            Box::new(alpha),
        )),
        Box::new(beta),
    )
}
/// Create `PSigma.mk fst snd`.
#[allow(dead_code)]
pub fn mk_psigma_mk(fst: Expr, snd: Expr) -> Expr {
    Expr::App(
        Box::new(Expr::App(
            Box::new(Expr::Const(Name::str("PSigma.mk"), vec![])),
            Box::new(fst),
        )),
        Box::new(snd),
    )
}
/// Create `Decreasing rel x y`.
#[allow(dead_code)]
pub fn mk_decreasing(rel: Expr, x: Expr, y: Expr) -> Expr {
    Expr::App(
        Box::new(Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("Decreasing"), vec![])),
                Box::new(rel),
            )),
            Box::new(x),
        )),
        Box::new(y),
    )
}
/// Build well-founded recursion declarations in the environment.
#[allow(clippy::too_many_lines)]
pub fn build_wellfounded_env(env: &mut Environment) -> Result<(), String> {
    add_prereqs_if_missing(env)?;
    env.add(Declaration::Axiom {
        name: Name::str("WellFounded"),
        univ_params: vec![u_param()],
        ty: Expr::Pi(
            BinderInfo::Implicit,
            Name::str("alpha"),
            Box::new(sort_u()),
            Box::new(arrow(rel_ty(0), prop())),
        ),
    })
    .map_err(|e| e.to_string())?;
    env.add(Declaration::Axiom {
        name: Name::str("Acc"),
        univ_params: vec![u_param()],
        ty: Expr::Pi(
            BinderInfo::Implicit,
            Name::str("alpha"),
            Box::new(sort_u()),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("r"),
                Box::new(rel_ty(0)),
                Box::new(arrow(Expr::BVar(1), prop())),
            )),
        ),
    })
    .map_err(|e| e.to_string())?;
    env.add(Declaration::Axiom {
        name: Name::str("Acc.intro"),
        univ_params: vec![u_param()],
        ty: Expr::Pi(
            BinderInfo::Implicit,
            Name::str("alpha"),
            Box::new(sort_u()),
            Box::new(Expr::Pi(
                BinderInfo::Implicit,
                Name::str("r"),
                Box::new(rel_ty(0)),
                Box::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("x"),
                    Box::new(Expr::BVar(1)),
                    Box::new(Expr::Pi(
                        BinderInfo::Default,
                        Name::str("h"),
                        Box::new(Expr::Pi(
                            BinderInfo::Default,
                            Name::str("y"),
                            Box::new(Expr::BVar(2)),
                            Box::new(arrow(
                                Expr::App(
                                    Box::new(Expr::App(
                                        Box::new(Expr::BVar(2)),
                                        Box::new(Expr::BVar(0)),
                                    )),
                                    Box::new(Expr::BVar(1)),
                                ),
                                Expr::App(
                                    Box::new(Expr::App(
                                        Box::new(Expr::Const(Name::str("Acc"), vec![])),
                                        Box::new(Expr::BVar(3)),
                                    )),
                                    Box::new(Expr::BVar(1)),
                                ),
                            )),
                        )),
                        Box::new(Expr::App(
                            Box::new(Expr::App(
                                Box::new(Expr::Const(Name::str("Acc"), vec![])),
                                Box::new(Expr::BVar(2)),
                            )),
                            Box::new(Expr::BVar(1)),
                        )),
                    )),
                )),
            )),
        ),
    })
    .map_err(|e| e.to_string())?;
    env.add(Declaration::Axiom {
        name: Name::str("Acc.rec"),
        univ_params: vec![u_param(), v_param()],
        ty: Expr::Pi(
            BinderInfo::Implicit,
            Name::str("alpha"),
            Box::new(sort_u()),
            Box::new(Expr::Pi(
                BinderInfo::Implicit,
                Name::str("r"),
                Box::new(rel_ty(0)),
                Box::new(Expr::Pi(
                    BinderInfo::Implicit,
                    Name::str("C"),
                    Box::new(arrow(Expr::BVar(1), sort_v())),
                    Box::new(Expr::Pi(
                        BinderInfo::Default,
                        Name::str("step"),
                        Box::new(Expr::Pi(
                            BinderInfo::Default,
                            Name::str("x"),
                            Box::new(Expr::BVar(2)),
                            Box::new(arrow(
                                Expr::Pi(
                                    BinderInfo::Default,
                                    Name::str("y"),
                                    Box::new(Expr::BVar(3)),
                                    Box::new(arrow(
                                        Expr::App(
                                            Box::new(Expr::App(
                                                Box::new(Expr::BVar(3)),
                                                Box::new(Expr::BVar(0)),
                                            )),
                                            Box::new(Expr::BVar(1)),
                                        ),
                                        Expr::App(
                                            Box::new(Expr::App(
                                                Box::new(Expr::Const(Name::str("Acc"), vec![])),
                                                Box::new(Expr::BVar(3)),
                                            )),
                                            Box::new(Expr::BVar(0)),
                                        ),
                                    )),
                                ),
                                arrow(
                                    Expr::Pi(
                                        BinderInfo::Default,
                                        Name::str("y"),
                                        Box::new(Expr::BVar(4)),
                                        Box::new(arrow(
                                            Expr::App(
                                                Box::new(Expr::App(
                                                    Box::new(Expr::BVar(4)),
                                                    Box::new(Expr::BVar(0)),
                                                )),
                                                Box::new(Expr::BVar(2)),
                                            ),
                                            Expr::App(
                                                Box::new(Expr::BVar(3)),
                                                Box::new(Expr::BVar(0)),
                                            ),
                                        )),
                                    ),
                                    Expr::App(Box::new(Expr::BVar(3)), Box::new(Expr::BVar(2))),
                                ),
                            )),
                        )),
                        Box::new(Expr::Pi(
                            BinderInfo::Implicit,
                            Name::str("a"),
                            Box::new(Expr::BVar(3)),
                            Box::new(arrow(
                                Expr::App(
                                    Box::new(Expr::App(
                                        Box::new(Expr::Const(Name::str("Acc"), vec![])),
                                        Box::new(Expr::BVar(3)),
                                    )),
                                    Box::new(Expr::BVar(0)),
                                ),
                                Expr::App(Box::new(Expr::BVar(2)), Box::new(Expr::BVar(1))),
                            )),
                        )),
                    )),
                )),
            )),
        ),
    })
    .map_err(|e| e.to_string())?;
    env.add(Declaration::Axiom {
        name: Name::str("WellFounded.intro"),
        univ_params: vec![u_param()],
        ty: Expr::Pi(
            BinderInfo::Implicit,
            Name::str("alpha"),
            Box::new(sort_u()),
            Box::new(Expr::Pi(
                BinderInfo::Implicit,
                Name::str("r"),
                Box::new(rel_ty(0)),
                Box::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("h"),
                    Box::new(Expr::Pi(
                        BinderInfo::Default,
                        Name::str("a"),
                        Box::new(Expr::BVar(1)),
                        Box::new(Expr::App(
                            Box::new(Expr::App(
                                Box::new(Expr::Const(Name::str("Acc"), vec![])),
                                Box::new(Expr::BVar(1)),
                            )),
                            Box::new(Expr::BVar(0)),
                        )),
                    )),
                    Box::new(Expr::App(
                        Box::new(Expr::Const(Name::str("WellFounded"), vec![])),
                        Box::new(Expr::BVar(1)),
                    )),
                )),
            )),
        ),
    })
    .map_err(|e| e.to_string())?;
    env.add(Declaration::Axiom {
        name: Name::str("WellFounded.apply"),
        univ_params: vec![u_param()],
        ty: Expr::Pi(
            BinderInfo::Implicit,
            Name::str("alpha"),
            Box::new(sort_u()),
            Box::new(Expr::Pi(
                BinderInfo::Implicit,
                Name::str("r"),
                Box::new(rel_ty(0)),
                Box::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("wf"),
                    Box::new(Expr::App(
                        Box::new(Expr::Const(Name::str("WellFounded"), vec![])),
                        Box::new(Expr::BVar(0)),
                    )),
                    Box::new(Expr::Pi(
                        BinderInfo::Default,
                        Name::str("a"),
                        Box::new(Expr::BVar(2)),
                        Box::new(Expr::App(
                            Box::new(Expr::App(
                                Box::new(Expr::Const(Name::str("Acc"), vec![])),
                                Box::new(Expr::BVar(2)),
                            )),
                            Box::new(Expr::BVar(0)),
                        )),
                    )),
                )),
            )),
        ),
    })
    .map_err(|e| e.to_string())?;
    env.add(Declaration::Axiom {
        name: Name::str("WellFounded.fix"),
        univ_params: vec![u_param(), v_param()],
        ty: Expr::Pi(
            BinderInfo::Implicit,
            Name::str("alpha"),
            Box::new(sort_u()),
            Box::new(Expr::Pi(
                BinderInfo::Implicit,
                Name::str("C"),
                Box::new(arrow(Expr::BVar(0), sort_v())),
                Box::new(Expr::Pi(
                    BinderInfo::Implicit,
                    Name::str("r"),
                    Box::new(rel_ty(1)),
                    Box::new(Expr::Pi(
                        BinderInfo::Default,
                        Name::str("wf"),
                        Box::new(Expr::App(
                            Box::new(Expr::Const(Name::str("WellFounded"), vec![])),
                            Box::new(Expr::BVar(0)),
                        )),
                        Box::new(Expr::Pi(
                            BinderInfo::Default,
                            Name::str("F"),
                            Box::new(Expr::Pi(
                                BinderInfo::Default,
                                Name::str("x"),
                                Box::new(Expr::BVar(3)),
                                Box::new(arrow(
                                    Expr::Pi(
                                        BinderInfo::Default,
                                        Name::str("y"),
                                        Box::new(Expr::BVar(4)),
                                        Box::new(arrow(
                                            Expr::App(
                                                Box::new(Expr::App(
                                                    Box::new(Expr::BVar(3)),
                                                    Box::new(Expr::BVar(0)),
                                                )),
                                                Box::new(Expr::BVar(1)),
                                            ),
                                            Expr::App(
                                                Box::new(Expr::BVar(5)),
                                                Box::new(Expr::BVar(1)),
                                            ),
                                        )),
                                    ),
                                    Expr::App(Box::new(Expr::BVar(4)), Box::new(Expr::BVar(1))),
                                )),
                            )),
                            Box::new(Expr::Pi(
                                BinderInfo::Default,
                                Name::str("a"),
                                Box::new(Expr::BVar(4)),
                                Box::new(Expr::App(
                                    Box::new(Expr::BVar(4)),
                                    Box::new(Expr::BVar(0)),
                                )),
                            )),
                        )),
                    )),
                )),
            )),
        ),
    })
    .map_err(|e| e.to_string())?;
    env.add(Declaration::Axiom {
        name: Name::str("Measure"),
        univ_params: vec![u_param()],
        ty: Expr::Pi(
            BinderInfo::Implicit,
            Name::str("alpha"),
            Box::new(sort_u()),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("f"),
                Box::new(arrow(Expr::BVar(0), nat_const())),
                Box::new(arrow(Expr::BVar(1), arrow(Expr::BVar(2), prop()))),
            )),
        ),
    })
    .map_err(|e| e.to_string())?;
    env.add(Declaration::Axiom {
        name: Name::str("InvImage"),
        univ_params: vec![u_param()],
        ty: Expr::Pi(
            BinderInfo::Implicit,
            Name::str("alpha"),
            Box::new(sort_u()),
            Box::new(Expr::Pi(
                BinderInfo::Implicit,
                Name::str("beta"),
                Box::new(sort_u()),
                Box::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("r"),
                    Box::new(arrow(Expr::BVar(0), arrow(Expr::BVar(1), prop()))),
                    Box::new(Expr::Pi(
                        BinderInfo::Default,
                        Name::str("f"),
                        Box::new(arrow(Expr::BVar(2), Expr::BVar(1))),
                        Box::new(arrow(Expr::BVar(3), arrow(Expr::BVar(4), prop()))),
                    )),
                )),
            )),
        ),
    })
    .map_err(|e| e.to_string())?;
    env.add(Declaration::Axiom {
        name: Name::str("Prod.Lex"),
        univ_params: vec![u_param()],
        ty: Expr::Pi(
            BinderInfo::Implicit,
            Name::str("alpha"),
            Box::new(sort_u()),
            Box::new(Expr::Pi(
                BinderInfo::Implicit,
                Name::str("beta"),
                Box::new(sort_u()),
                Box::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("ra"),
                    Box::new(arrow(Expr::BVar(1), arrow(Expr::BVar(2), prop()))),
                    Box::new(Expr::Pi(
                        BinderInfo::Default,
                        Name::str("rb"),
                        Box::new(arrow(Expr::BVar(1), arrow(Expr::BVar(2), prop()))),
                        Box::new(arrow(
                            Expr::App(
                                Box::new(Expr::App(
                                    Box::new(Expr::Const(Name::str("Prod"), vec![])),
                                    Box::new(Expr::BVar(3)),
                                )),
                                Box::new(Expr::BVar(2)),
                            ),
                            arrow(
                                Expr::App(
                                    Box::new(Expr::App(
                                        Box::new(Expr::Const(Name::str("Prod"), vec![])),
                                        Box::new(Expr::BVar(4)),
                                    )),
                                    Box::new(Expr::BVar(3)),
                                ),
                                prop(),
                            ),
                        )),
                    )),
                )),
            )),
        ),
    })
    .map_err(|e| e.to_string())?;
    env.add(Declaration::Axiom {
        name: Name::str("SizeOf"),
        univ_params: vec![u_param()],
        ty: arrow(sort_u(), type1()),
    })
    .map_err(|e| e.to_string())?;
    env.add(Declaration::Axiom {
        name: Name::str("sizeOf"),
        univ_params: vec![u_param()],
        ty: Expr::Pi(
            BinderInfo::Implicit,
            Name::str("alpha"),
            Box::new(sort_u()),
            Box::new(Expr::Pi(
                BinderInfo::InstImplicit,
                Name::str("inst"),
                Box::new(Expr::App(
                    Box::new(Expr::Const(Name::str("SizeOf"), vec![])),
                    Box::new(Expr::BVar(0)),
                )),
                Box::new(arrow(Expr::BVar(1), nat_const())),
            )),
        ),
    })
    .map_err(|e| e.to_string())?;
    if !env.contains(&Name::str("Nat.lt")) {
        env.add(Declaration::Axiom {
            name: Name::str("Nat.lt"),
            univ_params: vec![],
            ty: arrow(nat_const(), arrow(nat_const(), prop())),
        })
        .map_err(|e| e.to_string())?;
    }
    env.add(Declaration::Axiom {
        name: Name::str("Nat.lt_wfRel"),
        univ_params: vec![],
        ty: Expr::App(
            Box::new(Expr::Const(Name::str("WellFounded"), vec![])),
            Box::new(Expr::Const(Name::str("Nat.lt"), vec![])),
        ),
    })
    .map_err(|e| e.to_string())?;
    env.add(Declaration::Axiom {
        name: Name::str("measure_wf"),
        univ_params: vec![u_param()],
        ty: Expr::Pi(
            BinderInfo::Implicit,
            Name::str("alpha"),
            Box::new(sort_u()),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("f"),
                Box::new(arrow(Expr::BVar(0), nat_const())),
                Box::new(Expr::App(
                    Box::new(Expr::Const(Name::str("WellFounded"), vec![])),
                    Box::new(Expr::App(
                        Box::new(Expr::Const(Name::str("Measure"), vec![])),
                        Box::new(Expr::BVar(0)),
                    )),
                )),
            )),
        ),
    })
    .map_err(|e| e.to_string())?;
    env.add(Declaration::Axiom {
        name: Name::str("invImage_wf"),
        univ_params: vec![u_param()],
        ty: Expr::Pi(
            BinderInfo::Implicit,
            Name::str("alpha"),
            Box::new(sort_u()),
            Box::new(Expr::Pi(
                BinderInfo::Implicit,
                Name::str("beta"),
                Box::new(sort_u()),
                Box::new(Expr::Pi(
                    BinderInfo::Implicit,
                    Name::str("r"),
                    Box::new(arrow(Expr::BVar(0), arrow(Expr::BVar(1), prop()))),
                    Box::new(Expr::Pi(
                        BinderInfo::Default,
                        Name::str("wf"),
                        Box::new(Expr::App(
                            Box::new(Expr::Const(Name::str("WellFounded"), vec![])),
                            Box::new(Expr::BVar(0)),
                        )),
                        Box::new(Expr::Pi(
                            BinderInfo::Default,
                            Name::str("f"),
                            Box::new(arrow(Expr::BVar(3), Expr::BVar(2))),
                            Box::new(Expr::App(
                                Box::new(Expr::Const(Name::str("WellFounded"), vec![])),
                                Box::new(Expr::App(
                                    Box::new(Expr::App(
                                        Box::new(Expr::Const(Name::str("InvImage"), vec![])),
                                        Box::new(Expr::BVar(2)),
                                    )),
                                    Box::new(Expr::BVar(0)),
                                )),
                            )),
                        )),
                    )),
                )),
            )),
        ),
    })
    .map_err(|e| e.to_string())?;
    env.add(Declaration::Axiom {
        name: Name::str("prod_lex_wf"),
        univ_params: vec![u_param()],
        ty: Expr::Pi(
            BinderInfo::Implicit,
            Name::str("alpha"),
            Box::new(sort_u()),
            Box::new(Expr::Pi(
                BinderInfo::Implicit,
                Name::str("beta"),
                Box::new(sort_u()),
                Box::new(Expr::Pi(
                    BinderInfo::Implicit,
                    Name::str("ra"),
                    Box::new(arrow(Expr::BVar(1), arrow(Expr::BVar(2), prop()))),
                    Box::new(Expr::Pi(
                        BinderInfo::Implicit,
                        Name::str("rb"),
                        Box::new(arrow(Expr::BVar(1), arrow(Expr::BVar(2), prop()))),
                        Box::new(Expr::Pi(
                            BinderInfo::Default,
                            Name::str("wfa"),
                            Box::new(Expr::App(
                                Box::new(Expr::Const(Name::str("WellFounded"), vec![])),
                                Box::new(Expr::BVar(1)),
                            )),
                            Box::new(Expr::Pi(
                                BinderInfo::Default,
                                Name::str("wfb"),
                                Box::new(Expr::App(
                                    Box::new(Expr::Const(Name::str("WellFounded"), vec![])),
                                    Box::new(Expr::BVar(1)),
                                )),
                                Box::new(Expr::App(
                                    Box::new(Expr::Const(Name::str("WellFounded"), vec![])),
                                    Box::new(Expr::App(
                                        Box::new(Expr::App(
                                            Box::new(Expr::Const(Name::str("Prod.Lex"), vec![])),
                                            Box::new(Expr::BVar(3)),
                                        )),
                                        Box::new(Expr::BVar(2)),
                                    )),
                                )),
                            )),
                        )),
                    )),
                )),
            )),
        ),
    })
    .map_err(|e| e.to_string())?;
    env.add(Declaration::Axiom {
        name: Name::str("Decreasing"),
        univ_params: vec![u_param()],
        ty: Expr::Pi(
            BinderInfo::Implicit,
            Name::str("alpha"),
            Box::new(sort_u()),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("r"),
                Box::new(rel_ty(0)),
                Box::new(arrow(Expr::BVar(1), arrow(Expr::BVar(2), prop()))),
            )),
        ),
    })
    .map_err(|e| e.to_string())?;
    env.add(Declaration::Axiom {
        name: Name::str("PSigma"),
        univ_params: vec![u_param(), v_param()],
        ty: Expr::Pi(
            BinderInfo::Implicit,
            Name::str("alpha"),
            Box::new(sort_u()),
            Box::new(arrow(
                arrow(Expr::BVar(0), sort_v()),
                Expr::Sort(Level::max(
                    Level::Param(Name::str("u")),
                    Level::Param(Name::str("v")),
                )),
            )),
        ),
    })
    .map_err(|e| e.to_string())?;
    env.add(Declaration::Axiom {
        name: Name::str("PSigma.mk"),
        univ_params: vec![u_param(), v_param()],
        ty: Expr::Pi(
            BinderInfo::Implicit,
            Name::str("alpha"),
            Box::new(sort_u()),
            Box::new(Expr::Pi(
                BinderInfo::Implicit,
                Name::str("beta"),
                Box::new(arrow(Expr::BVar(0), sort_v())),
                Box::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("fst"),
                    Box::new(Expr::BVar(1)),
                    Box::new(Expr::Pi(
                        BinderInfo::Default,
                        Name::str("snd"),
                        Box::new(Expr::App(Box::new(Expr::BVar(1)), Box::new(Expr::BVar(0)))),
                        Box::new(Expr::App(
                            Box::new(Expr::Const(Name::str("PSigma"), vec![])),
                            Box::new(Expr::BVar(2)),
                        )),
                    )),
                )),
            )),
        ),
    })
    .map_err(|e| e.to_string())?;
    env.add(Declaration::Axiom {
        name: Name::str("PSigma.fst"),
        univ_params: vec![u_param(), v_param()],
        ty: Expr::Pi(
            BinderInfo::Implicit,
            Name::str("alpha"),
            Box::new(sort_u()),
            Box::new(Expr::Pi(
                BinderInfo::Implicit,
                Name::str("beta"),
                Box::new(arrow(Expr::BVar(0), sort_v())),
                Box::new(arrow(
                    Expr::App(
                        Box::new(Expr::Const(Name::str("PSigma"), vec![])),
                        Box::new(Expr::BVar(0)),
                    ),
                    Expr::BVar(1),
                )),
            )),
        ),
    })
    .map_err(|e| e.to_string())?;
    env.add(Declaration::Axiom {
        name: Name::str("PSigma.snd"),
        univ_params: vec![u_param(), v_param()],
        ty: Expr::Pi(
            BinderInfo::Implicit,
            Name::str("alpha"),
            Box::new(sort_u()),
            Box::new(Expr::Pi(
                BinderInfo::Implicit,
                Name::str("beta"),
                Box::new(arrow(Expr::BVar(0), sort_v())),
                Box::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("p"),
                    Box::new(Expr::App(
                        Box::new(Expr::Const(Name::str("PSigma"), vec![])),
                        Box::new(Expr::BVar(0)),
                    )),
                    Box::new(Expr::App(
                        Box::new(Expr::BVar(1)),
                        Box::new(Expr::App(
                            Box::new(Expr::App(
                                Box::new(Expr::App(
                                    Box::new(Expr::Const(Name::str("PSigma.fst"), vec![])),
                                    Box::new(Expr::BVar(2)),
                                )),
                                Box::new(Expr::BVar(1)),
                            )),
                            Box::new(Expr::BVar(0)),
                        )),
                    )),
                )),
            )),
        ),
    })
    .map_err(|e| e.to_string())?;
    env.add(Declaration::Axiom {
        name: Name::str("Acc.rec_on"),
        univ_params: vec![u_param(), v_param()],
        ty: Expr::Pi(
            BinderInfo::Implicit,
            Name::str("alpha"),
            Box::new(sort_u()),
            Box::new(Expr::Pi(
                BinderInfo::Implicit,
                Name::str("r"),
                Box::new(rel_ty(0)),
                Box::new(Expr::Pi(
                    BinderInfo::Implicit,
                    Name::str("C"),
                    Box::new(arrow(Expr::BVar(1), sort_v())),
                    Box::new(Expr::Pi(
                        BinderInfo::Implicit,
                        Name::str("a"),
                        Box::new(Expr::BVar(2)),
                        Box::new(Expr::Pi(
                            BinderInfo::Default,
                            Name::str("acc"),
                            Box::new(Expr::App(
                                Box::new(Expr::App(
                                    Box::new(Expr::Const(Name::str("Acc"), vec![])),
                                    Box::new(Expr::BVar(3)),
                                )),
                                Box::new(Expr::BVar(0)),
                            )),
                            Box::new(Expr::Pi(
                                BinderInfo::Default,
                                Name::str("step"),
                                Box::new(arrow(Expr::BVar(3), arrow(prop(), sort_v()))),
                                Box::new(Expr::App(
                                    Box::new(Expr::BVar(3)),
                                    Box::new(Expr::BVar(2)),
                                )),
                            )),
                        )),
                    )),
                )),
            )),
        ),
    })
    .map_err(|e| e.to_string())?;
    let fix_eq_ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("alpha"),
        Box::new(sort_u()),
        Box::new(Expr::Pi(
            BinderInfo::Implicit,
            Name::str("r"),
            Box::new(rel_ty(0)),
            Box::new(Expr::Pi(
                BinderInfo::Implicit,
                Name::str("C"),
                Box::new(arrow(Expr::BVar(1), sort_v())),
                Box::new(arrow(
                    Expr::App(
                        Box::new(Expr::Const(Name::str("WellFounded"), vec![])),
                        Box::new(Expr::BVar(1)),
                    ),
                    arrow(
                        Expr::Pi(
                            BinderInfo::Default,
                            Name::str("x"),
                            Box::new(Expr::BVar(3)),
                            Box::new(arrow(
                                Expr::Pi(
                                    BinderInfo::Default,
                                    Name::str("y"),
                                    Box::new(Expr::BVar(4)),
                                    Box::new(arrow(
                                        Expr::App(
                                            Box::new(Expr::App(
                                                Box::new(Expr::BVar(4)),
                                                Box::new(Expr::BVar(0)),
                                            )),
                                            Box::new(Expr::BVar(1)),
                                        ),
                                        Expr::App(Box::new(Expr::BVar(3)), Box::new(Expr::BVar(0))),
                                    )),
                                ),
                                Expr::App(Box::new(Expr::BVar(3)), Box::new(Expr::BVar(1))),
                            )),
                        ),
                        Expr::Pi(
                            BinderInfo::Default,
                            Name::str("a"),
                            Box::new(Expr::BVar(4)),
                            Box::new(Expr::App(
                                Box::new(Expr::App(
                                    Box::new(Expr::App(
                                        Box::new(Expr::Const(Name::str("Eq"), vec![])),
                                        Box::new(Expr::App(
                                            Box::new(Expr::BVar(3)),
                                            Box::new(Expr::BVar(0)),
                                        )),
                                    )),
                                    Box::new(Expr::App(
                                        Box::new(Expr::App(
                                            Box::new(Expr::App(
                                                Box::new(Expr::Const(
                                                    Name::str("WellFounded.fix"),
                                                    vec![],
                                                )),
                                                Box::new(Expr::BVar(2)),
                                            )),
                                            Box::new(Expr::BVar(1)),
                                        )),
                                        Box::new(Expr::BVar(0)),
                                    )),
                                )),
                                Box::new(Expr::App(
                                    Box::new(Expr::App(
                                        Box::new(Expr::BVar(1)),
                                        Box::new(Expr::BVar(0)),
                                    )),
                                    Box::new(Expr::Lam(
                                        BinderInfo::Default,
                                        Name::str("y"),
                                        Box::new(Expr::BVar(5)),
                                        Box::new(Expr::Lam(
                                            BinderInfo::Default,
                                            Name::str("hy"),
                                            Box::new(Expr::App(
                                                Box::new(Expr::App(
                                                    Box::new(Expr::BVar(6)),
                                                    Box::new(Expr::BVar(1)),
                                                )),
                                                Box::new(Expr::BVar(2)),
                                            )),
                                            Box::new(Expr::App(
                                                Box::new(Expr::App(
                                                    Box::new(Expr::App(
                                                        Box::new(Expr::Const(
                                                            Name::str("WellFounded.fix"),
                                                            vec![],
                                                        )),
                                                        Box::new(Expr::BVar(4)),
                                                    )),
                                                    Box::new(Expr::BVar(3)),
                                                )),
                                                Box::new(Expr::BVar(1)),
                                            )),
                                        )),
                                    )),
                                )),
                            )),
                        ),
                    ),
                )),
            )),
        )),
    );
    env.add(Declaration::Theorem {
        name: Name::str("WellFounded.fix_eq"),
        univ_params: vec![u_param(), v_param()],
        ty: fix_eq_ty,
        val: sorry(),
    })
    .map_err(|e| e.to_string())?;
    let measure_lt_ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("alpha"),
        Box::new(sort_u()),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("f"),
            Box::new(arrow(Expr::BVar(0), nat_const())),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("x"),
                Box::new(Expr::BVar(1)),
                Box::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("y"),
                    Box::new(Expr::BVar(2)),
                    Box::new(arrow(
                        Expr::App(
                            Box::new(Expr::App(
                                Box::new(Expr::Const(Name::str("Nat.lt"), vec![])),
                                Box::new(Expr::App(
                                    Box::new(Expr::BVar(2)),
                                    Box::new(Expr::BVar(1)),
                                )),
                            )),
                            Box::new(Expr::App(Box::new(Expr::BVar(2)), Box::new(Expr::BVar(0)))),
                        ),
                        Expr::App(
                            Box::new(Expr::App(
                                Box::new(Expr::App(
                                    Box::new(Expr::Const(Name::str("Measure"), vec![])),
                                    Box::new(Expr::BVar(3)),
                                )),
                                Box::new(Expr::BVar(2)),
                            )),
                            Box::new(Expr::BVar(1)),
                        ),
                    )),
                )),
            )),
        )),
    );
    env.add(Declaration::Theorem {
        name: Name::str("measure_lt"),
        univ_params: vec![u_param()],
        ty: measure_lt_ty,
        val: sorry(),
    })
    .map_err(|e| e.to_string())?;
    let nat_lt_wf_aux_ty = Expr::Pi(
        BinderInfo::Default,
        Name::str("n"),
        Box::new(nat_const()),
        Box::new(Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("Acc"), vec![])),
                Box::new(Expr::Const(Name::str("Nat.lt"), vec![])),
            )),
            Box::new(Expr::BVar(0)),
        )),
    );
    env.add(Declaration::Theorem {
        name: Name::str("Nat.lt_wf_aux"),
        univ_params: vec![],
        ty: nat_lt_wf_aux_ty,
        val: sorry(),
    })
    .map_err(|e| e.to_string())?;
    let sizeof_nat_ty = Expr::Pi(
        BinderInfo::Default,
        Name::str("n"),
        Box::new(nat_const()),
        Box::new(Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("Eq"), vec![])),
                Box::new(Expr::App(
                    Box::new(Expr::App(
                        Box::new(Expr::Const(Name::str("sizeOf"), vec![])),
                        Box::new(nat_const()),
                    )),
                    Box::new(Expr::BVar(0)),
                )),
            )),
            Box::new(Expr::BVar(0)),
        )),
    );
    env.add(Declaration::Theorem {
        name: Name::str("sizeOf_nat"),
        univ_params: vec![],
        ty: sizeof_nat_ty,
        val: sorry(),
    })
    .map_err(|e| e.to_string())?;
    let sizeof_prod_ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("alpha"),
        Box::new(sort_u()),
        Box::new(Expr::Pi(
            BinderInfo::Implicit,
            Name::str("beta"),
            Box::new(sort_v()),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("a"),
                Box::new(Expr::BVar(1)),
                Box::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("b"),
                    Box::new(Expr::BVar(1)),
                    Box::new(Expr::App(
                        Box::new(Expr::App(
                            Box::new(Expr::App(
                                Box::new(Expr::Const(Name::str("Eq"), vec![])),
                                Box::new(nat_const()),
                            )),
                            Box::new(Expr::App(
                                Box::new(Expr::Const(Name::str("sizeOf"), vec![])),
                                Box::new(Expr::App(
                                    Box::new(Expr::App(
                                        Box::new(Expr::Const(Name::str("Prod.mk"), vec![])),
                                        Box::new(Expr::BVar(1)),
                                    )),
                                    Box::new(Expr::BVar(0)),
                                )),
                            )),
                        )),
                        Box::new(Expr::App(
                            Box::new(Expr::App(
                                Box::new(Expr::Const(Name::str("Nat.add"), vec![])),
                                Box::new(Expr::App(
                                    Box::new(Expr::App(
                                        Box::new(Expr::Const(Name::str("Nat.add"), vec![])),
                                        Box::new(Expr::App(
                                            Box::new(Expr::Const(Name::str("Nat.succ"), vec![])),
                                            Box::new(Expr::Const(Name::str("Nat.zero"), vec![])),
                                        )),
                                    )),
                                    Box::new(Expr::App(
                                        Box::new(Expr::Const(Name::str("sizeOf"), vec![])),
                                        Box::new(Expr::BVar(1)),
                                    )),
                                )),
                            )),
                            Box::new(Expr::App(
                                Box::new(Expr::Const(Name::str("sizeOf"), vec![])),
                                Box::new(Expr::BVar(0)),
                            )),
                        )),
                    )),
                )),
            )),
        )),
    );
    env.add(Declaration::Theorem {
        name: Name::str("sizeOf_prod"),
        univ_params: vec![u_param(), v_param()],
        ty: sizeof_prod_ty,
        val: sorry(),
    })
    .map_err(|e| e.to_string())?;
    Ok(())
}
pub fn add_prereqs_if_missing(env: &mut Environment) -> Result<(), String> {
    if !env.contains(&Name::str("Nat")) {
        env.add(Declaration::Axiom {
            name: Name::str("Nat"),
            univ_params: vec![],
            ty: type1(),
        })
        .map_err(|e| e.to_string())?;
    }
    if !env.contains(&Name::str("Eq")) {
        let eq_ty = Expr::Pi(
            BinderInfo::Implicit,
            Name::str("a"),
            Box::new(type1()),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("x"),
                Box::new(Expr::BVar(0)),
                Box::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("y"),
                    Box::new(Expr::BVar(1)),
                    Box::new(prop()),
                )),
            )),
        );
        env.add(Declaration::Axiom {
            name: Name::str("Eq"),
            univ_params: vec![],
            ty: eq_ty,
        })
        .map_err(|e| e.to_string())?;
    }
    if !env.contains(&Name::str("sorry")) {
        env.add(Declaration::Axiom {
            name: Name::str("sorry"),
            univ_params: vec![],
            ty: prop(),
        })
        .map_err(|e| e.to_string())?;
    }
    Ok(())
}
