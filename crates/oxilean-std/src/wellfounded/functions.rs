//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::Node;
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
        Node::new(a),
        Node::new(b),
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
        Node::new(Expr::Const(Name::str("WellFounded"), vec![])),
        Node::new(rel),
    )
}
/// Create `Acc rel x`.
#[allow(dead_code)]
pub fn mk_acc(rel: Expr, x: Expr) -> Expr {
    Expr::App(
        Node::new(Expr::App(
            Node::new(Expr::Const(Name::str("Acc"), vec![])),
            Node::new(rel),
        )),
        Node::new(x),
    )
}
/// Create `Acc.intro x h`.
#[allow(dead_code)]
pub fn mk_acc_intro(x: Expr, h: Expr) -> Expr {
    Expr::App(
        Node::new(Expr::App(
            Node::new(Expr::Const(Name::str("Acc.intro"), vec![])),
            Node::new(x),
        )),
        Node::new(h),
    )
}
/// Create `WellFounded.fix wf f a`.
#[allow(dead_code)]
pub fn mk_wf_fix(wf: Expr, f: Expr, a: Expr) -> Expr {
    Expr::App(
        Node::new(Expr::App(
            Node::new(Expr::App(
                Node::new(Expr::Const(Name::str("WellFounded.fix"), vec![])),
                Node::new(wf),
            )),
            Node::new(f),
        )),
        Node::new(a),
    )
}
/// Create `Measure f`.
#[allow(dead_code)]
pub fn mk_measure(f: Expr) -> Expr {
    Expr::App(
        Node::new(Expr::Const(Name::str("Measure"), vec![])),
        Node::new(f),
    )
}
/// Create `InvImage r f`.
#[allow(dead_code)]
pub fn mk_inv_image(r: Expr, f: Expr) -> Expr {
    Expr::App(
        Node::new(Expr::App(
            Node::new(Expr::Const(Name::str("InvImage"), vec![])),
            Node::new(r),
        )),
        Node::new(f),
    )
}
/// Create `Prod.Lex ra rb`.
#[allow(dead_code)]
pub fn mk_prod_lex(ra: Expr, rb: Expr) -> Expr {
    Expr::App(
        Node::new(Expr::App(
            Node::new(Expr::Const(Name::str("Prod.Lex"), vec![])),
            Node::new(ra),
        )),
        Node::new(rb),
    )
}
/// Create `@sizeOf ty a`.
#[allow(dead_code)]
pub fn mk_sizeof(ty: Expr, a: Expr) -> Expr {
    Expr::App(
        Node::new(Expr::App(
            Node::new(Expr::Const(Name::str("sizeOf"), vec![])),
            Node::new(ty),
        )),
        Node::new(a),
    )
}
/// Create `PSigma beta`.
#[allow(dead_code)]
pub fn mk_psigma(alpha: Expr, beta: Expr) -> Expr {
    Expr::App(
        Node::new(Expr::App(
            Node::new(Expr::Const(Name::str("PSigma"), vec![])),
            Node::new(alpha),
        )),
        Node::new(beta),
    )
}
/// Create `PSigma.mk fst snd`.
#[allow(dead_code)]
pub fn mk_psigma_mk(fst: Expr, snd: Expr) -> Expr {
    Expr::App(
        Node::new(Expr::App(
            Node::new(Expr::Const(Name::str("PSigma.mk"), vec![])),
            Node::new(fst),
        )),
        Node::new(snd),
    )
}
/// Create `Decreasing rel x y`.
#[allow(dead_code)]
pub fn mk_decreasing(rel: Expr, x: Expr, y: Expr) -> Expr {
    Expr::App(
        Node::new(Expr::App(
            Node::new(Expr::App(
                Node::new(Expr::Const(Name::str("Decreasing"), vec![])),
                Node::new(rel),
            )),
            Node::new(x),
        )),
        Node::new(y),
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
            Node::new(sort_u()),
            Node::new(arrow(rel_ty(0), prop())),
        ),
    })
    .map_err(|e| e.to_string())?;
    env.add(Declaration::Axiom {
        name: Name::str("Acc"),
        univ_params: vec![u_param()],
        ty: Expr::Pi(
            BinderInfo::Implicit,
            Name::str("alpha"),
            Node::new(sort_u()),
            Node::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("r"),
                Node::new(rel_ty(0)),
                Node::new(arrow(Expr::BVar(1), prop())),
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
            Node::new(sort_u()),
            Node::new(Expr::Pi(
                BinderInfo::Implicit,
                Name::str("r"),
                Node::new(rel_ty(0)),
                Node::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("x"),
                    Node::new(Expr::BVar(1)),
                    Node::new(Expr::Pi(
                        BinderInfo::Default,
                        Name::str("h"),
                        Node::new(Expr::Pi(
                            BinderInfo::Default,
                            Name::str("y"),
                            Node::new(Expr::BVar(2)),
                            Node::new(arrow(
                                Expr::App(
                                    Node::new(Expr::App(
                                        Node::new(Expr::BVar(2)),
                                        Node::new(Expr::BVar(0)),
                                    )),
                                    Node::new(Expr::BVar(1)),
                                ),
                                Expr::App(
                                    Node::new(Expr::App(
                                        Node::new(Expr::Const(Name::str("Acc"), vec![])),
                                        Node::new(Expr::BVar(3)),
                                    )),
                                    Node::new(Expr::BVar(1)),
                                ),
                            )),
                        )),
                        Node::new(Expr::App(
                            Node::new(Expr::App(
                                Node::new(Expr::Const(Name::str("Acc"), vec![])),
                                Node::new(Expr::BVar(2)),
                            )),
                            Node::new(Expr::BVar(1)),
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
            Node::new(sort_u()),
            Node::new(Expr::Pi(
                BinderInfo::Implicit,
                Name::str("r"),
                Node::new(rel_ty(0)),
                Node::new(Expr::Pi(
                    BinderInfo::Implicit,
                    Name::str("C"),
                    Node::new(arrow(Expr::BVar(1), sort_v())),
                    Node::new(Expr::Pi(
                        BinderInfo::Default,
                        Name::str("step"),
                        Node::new(Expr::Pi(
                            BinderInfo::Default,
                            Name::str("x"),
                            Node::new(Expr::BVar(2)),
                            Node::new(arrow(
                                Expr::Pi(
                                    BinderInfo::Default,
                                    Name::str("y"),
                                    Node::new(Expr::BVar(3)),
                                    Node::new(arrow(
                                        Expr::App(
                                            Node::new(Expr::App(
                                                Node::new(Expr::BVar(3)),
                                                Node::new(Expr::BVar(0)),
                                            )),
                                            Node::new(Expr::BVar(1)),
                                        ),
                                        Expr::App(
                                            Node::new(Expr::App(
                                                Node::new(Expr::Const(Name::str("Acc"), vec![])),
                                                Node::new(Expr::BVar(3)),
                                            )),
                                            Node::new(Expr::BVar(0)),
                                        ),
                                    )),
                                ),
                                arrow(
                                    Expr::Pi(
                                        BinderInfo::Default,
                                        Name::str("y"),
                                        Node::new(Expr::BVar(4)),
                                        Node::new(arrow(
                                            Expr::App(
                                                Node::new(Expr::App(
                                                    Node::new(Expr::BVar(4)),
                                                    Node::new(Expr::BVar(0)),
                                                )),
                                                Node::new(Expr::BVar(2)),
                                            ),
                                            Expr::App(
                                                Node::new(Expr::BVar(3)),
                                                Node::new(Expr::BVar(0)),
                                            ),
                                        )),
                                    ),
                                    Expr::App(Node::new(Expr::BVar(3)), Node::new(Expr::BVar(2))),
                                ),
                            )),
                        )),
                        Node::new(Expr::Pi(
                            BinderInfo::Implicit,
                            Name::str("a"),
                            Node::new(Expr::BVar(3)),
                            Node::new(arrow(
                                Expr::App(
                                    Node::new(Expr::App(
                                        Node::new(Expr::Const(Name::str("Acc"), vec![])),
                                        Node::new(Expr::BVar(3)),
                                    )),
                                    Node::new(Expr::BVar(0)),
                                ),
                                Expr::App(Node::new(Expr::BVar(2)), Node::new(Expr::BVar(1))),
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
            Node::new(sort_u()),
            Node::new(Expr::Pi(
                BinderInfo::Implicit,
                Name::str("r"),
                Node::new(rel_ty(0)),
                Node::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("h"),
                    Node::new(Expr::Pi(
                        BinderInfo::Default,
                        Name::str("a"),
                        Node::new(Expr::BVar(1)),
                        Node::new(Expr::App(
                            Node::new(Expr::App(
                                Node::new(Expr::Const(Name::str("Acc"), vec![])),
                                Node::new(Expr::BVar(1)),
                            )),
                            Node::new(Expr::BVar(0)),
                        )),
                    )),
                    Node::new(Expr::App(
                        Node::new(Expr::Const(Name::str("WellFounded"), vec![])),
                        Node::new(Expr::BVar(1)),
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
            Node::new(sort_u()),
            Node::new(Expr::Pi(
                BinderInfo::Implicit,
                Name::str("r"),
                Node::new(rel_ty(0)),
                Node::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("wf"),
                    Node::new(Expr::App(
                        Node::new(Expr::Const(Name::str("WellFounded"), vec![])),
                        Node::new(Expr::BVar(0)),
                    )),
                    Node::new(Expr::Pi(
                        BinderInfo::Default,
                        Name::str("a"),
                        Node::new(Expr::BVar(2)),
                        Node::new(Expr::App(
                            Node::new(Expr::App(
                                Node::new(Expr::Const(Name::str("Acc"), vec![])),
                                Node::new(Expr::BVar(2)),
                            )),
                            Node::new(Expr::BVar(0)),
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
            Node::new(sort_u()),
            Node::new(Expr::Pi(
                BinderInfo::Implicit,
                Name::str("C"),
                Node::new(arrow(Expr::BVar(0), sort_v())),
                Node::new(Expr::Pi(
                    BinderInfo::Implicit,
                    Name::str("r"),
                    Node::new(rel_ty(1)),
                    Node::new(Expr::Pi(
                        BinderInfo::Default,
                        Name::str("wf"),
                        Node::new(Expr::App(
                            Node::new(Expr::Const(Name::str("WellFounded"), vec![])),
                            Node::new(Expr::BVar(0)),
                        )),
                        Node::new(Expr::Pi(
                            BinderInfo::Default,
                            Name::str("F"),
                            Node::new(Expr::Pi(
                                BinderInfo::Default,
                                Name::str("x"),
                                Node::new(Expr::BVar(3)),
                                Node::new(arrow(
                                    Expr::Pi(
                                        BinderInfo::Default,
                                        Name::str("y"),
                                        Node::new(Expr::BVar(4)),
                                        Node::new(arrow(
                                            Expr::App(
                                                Node::new(Expr::App(
                                                    Node::new(Expr::BVar(3)),
                                                    Node::new(Expr::BVar(0)),
                                                )),
                                                Node::new(Expr::BVar(1)),
                                            ),
                                            Expr::App(
                                                Node::new(Expr::BVar(5)),
                                                Node::new(Expr::BVar(1)),
                                            ),
                                        )),
                                    ),
                                    Expr::App(Node::new(Expr::BVar(4)), Node::new(Expr::BVar(1))),
                                )),
                            )),
                            Node::new(Expr::Pi(
                                BinderInfo::Default,
                                Name::str("a"),
                                Node::new(Expr::BVar(4)),
                                Node::new(Expr::App(
                                    Node::new(Expr::BVar(4)),
                                    Node::new(Expr::BVar(0)),
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
            Node::new(sort_u()),
            Node::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("f"),
                Node::new(arrow(Expr::BVar(0), nat_const())),
                Node::new(arrow(Expr::BVar(1), arrow(Expr::BVar(2), prop()))),
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
            Node::new(sort_u()),
            Node::new(Expr::Pi(
                BinderInfo::Implicit,
                Name::str("beta"),
                Node::new(sort_u()),
                Node::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("r"),
                    Node::new(arrow(Expr::BVar(0), arrow(Expr::BVar(1), prop()))),
                    Node::new(Expr::Pi(
                        BinderInfo::Default,
                        Name::str("f"),
                        Node::new(arrow(Expr::BVar(2), Expr::BVar(1))),
                        Node::new(arrow(Expr::BVar(3), arrow(Expr::BVar(4), prop()))),
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
            Node::new(sort_u()),
            Node::new(Expr::Pi(
                BinderInfo::Implicit,
                Name::str("beta"),
                Node::new(sort_u()),
                Node::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("ra"),
                    Node::new(arrow(Expr::BVar(1), arrow(Expr::BVar(2), prop()))),
                    Node::new(Expr::Pi(
                        BinderInfo::Default,
                        Name::str("rb"),
                        Node::new(arrow(Expr::BVar(1), arrow(Expr::BVar(2), prop()))),
                        Node::new(arrow(
                            Expr::App(
                                Node::new(Expr::App(
                                    Node::new(Expr::Const(Name::str("Prod"), vec![])),
                                    Node::new(Expr::BVar(3)),
                                )),
                                Node::new(Expr::BVar(2)),
                            ),
                            arrow(
                                Expr::App(
                                    Node::new(Expr::App(
                                        Node::new(Expr::Const(Name::str("Prod"), vec![])),
                                        Node::new(Expr::BVar(4)),
                                    )),
                                    Node::new(Expr::BVar(3)),
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
            Node::new(sort_u()),
            Node::new(Expr::Pi(
                BinderInfo::InstImplicit,
                Name::str("inst"),
                Node::new(Expr::App(
                    Node::new(Expr::Const(Name::str("SizeOf"), vec![])),
                    Node::new(Expr::BVar(0)),
                )),
                Node::new(arrow(Expr::BVar(1), nat_const())),
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
            Node::new(Expr::Const(Name::str("WellFounded"), vec![])),
            Node::new(Expr::Const(Name::str("Nat.lt"), vec![])),
        ),
    })
    .map_err(|e| e.to_string())?;
    env.add(Declaration::Axiom {
        name: Name::str("measure_wf"),
        univ_params: vec![u_param()],
        ty: Expr::Pi(
            BinderInfo::Implicit,
            Name::str("alpha"),
            Node::new(sort_u()),
            Node::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("f"),
                Node::new(arrow(Expr::BVar(0), nat_const())),
                Node::new(Expr::App(
                    Node::new(Expr::Const(Name::str("WellFounded"), vec![])),
                    Node::new(Expr::App(
                        Node::new(Expr::Const(Name::str("Measure"), vec![])),
                        Node::new(Expr::BVar(0)),
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
            Node::new(sort_u()),
            Node::new(Expr::Pi(
                BinderInfo::Implicit,
                Name::str("beta"),
                Node::new(sort_u()),
                Node::new(Expr::Pi(
                    BinderInfo::Implicit,
                    Name::str("r"),
                    Node::new(arrow(Expr::BVar(0), arrow(Expr::BVar(1), prop()))),
                    Node::new(Expr::Pi(
                        BinderInfo::Default,
                        Name::str("wf"),
                        Node::new(Expr::App(
                            Node::new(Expr::Const(Name::str("WellFounded"), vec![])),
                            Node::new(Expr::BVar(0)),
                        )),
                        Node::new(Expr::Pi(
                            BinderInfo::Default,
                            Name::str("f"),
                            Node::new(arrow(Expr::BVar(3), Expr::BVar(2))),
                            Node::new(Expr::App(
                                Node::new(Expr::Const(Name::str("WellFounded"), vec![])),
                                Node::new(Expr::App(
                                    Node::new(Expr::App(
                                        Node::new(Expr::Const(Name::str("InvImage"), vec![])),
                                        Node::new(Expr::BVar(2)),
                                    )),
                                    Node::new(Expr::BVar(0)),
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
            Node::new(sort_u()),
            Node::new(Expr::Pi(
                BinderInfo::Implicit,
                Name::str("beta"),
                Node::new(sort_u()),
                Node::new(Expr::Pi(
                    BinderInfo::Implicit,
                    Name::str("ra"),
                    Node::new(arrow(Expr::BVar(1), arrow(Expr::BVar(2), prop()))),
                    Node::new(Expr::Pi(
                        BinderInfo::Implicit,
                        Name::str("rb"),
                        Node::new(arrow(Expr::BVar(1), arrow(Expr::BVar(2), prop()))),
                        Node::new(Expr::Pi(
                            BinderInfo::Default,
                            Name::str("wfa"),
                            Node::new(Expr::App(
                                Node::new(Expr::Const(Name::str("WellFounded"), vec![])),
                                Node::new(Expr::BVar(1)),
                            )),
                            Node::new(Expr::Pi(
                                BinderInfo::Default,
                                Name::str("wfb"),
                                Node::new(Expr::App(
                                    Node::new(Expr::Const(Name::str("WellFounded"), vec![])),
                                    Node::new(Expr::BVar(1)),
                                )),
                                Node::new(Expr::App(
                                    Node::new(Expr::Const(Name::str("WellFounded"), vec![])),
                                    Node::new(Expr::App(
                                        Node::new(Expr::App(
                                            Node::new(Expr::Const(Name::str("Prod.Lex"), vec![])),
                                            Node::new(Expr::BVar(3)),
                                        )),
                                        Node::new(Expr::BVar(2)),
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
            Node::new(sort_u()),
            Node::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("r"),
                Node::new(rel_ty(0)),
                Node::new(arrow(Expr::BVar(1), arrow(Expr::BVar(2), prop()))),
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
            Node::new(sort_u()),
            Node::new(arrow(
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
            Node::new(sort_u()),
            Node::new(Expr::Pi(
                BinderInfo::Implicit,
                Name::str("beta"),
                Node::new(arrow(Expr::BVar(0), sort_v())),
                Node::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("fst"),
                    Node::new(Expr::BVar(1)),
                    Node::new(Expr::Pi(
                        BinderInfo::Default,
                        Name::str("snd"),
                        Node::new(Expr::App(
                            Node::new(Expr::BVar(1)),
                            Node::new(Expr::BVar(0)),
                        )),
                        Node::new(Expr::App(
                            Node::new(Expr::Const(Name::str("PSigma"), vec![])),
                            Node::new(Expr::BVar(2)),
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
            Node::new(sort_u()),
            Node::new(Expr::Pi(
                BinderInfo::Implicit,
                Name::str("beta"),
                Node::new(arrow(Expr::BVar(0), sort_v())),
                Node::new(arrow(
                    Expr::App(
                        Node::new(Expr::Const(Name::str("PSigma"), vec![])),
                        Node::new(Expr::BVar(0)),
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
            Node::new(sort_u()),
            Node::new(Expr::Pi(
                BinderInfo::Implicit,
                Name::str("beta"),
                Node::new(arrow(Expr::BVar(0), sort_v())),
                Node::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("p"),
                    Node::new(Expr::App(
                        Node::new(Expr::Const(Name::str("PSigma"), vec![])),
                        Node::new(Expr::BVar(0)),
                    )),
                    Node::new(Expr::App(
                        Node::new(Expr::BVar(1)),
                        Node::new(Expr::App(
                            Node::new(Expr::App(
                                Node::new(Expr::App(
                                    Node::new(Expr::Const(Name::str("PSigma.fst"), vec![])),
                                    Node::new(Expr::BVar(2)),
                                )),
                                Node::new(Expr::BVar(1)),
                            )),
                            Node::new(Expr::BVar(0)),
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
            Node::new(sort_u()),
            Node::new(Expr::Pi(
                BinderInfo::Implicit,
                Name::str("r"),
                Node::new(rel_ty(0)),
                Node::new(Expr::Pi(
                    BinderInfo::Implicit,
                    Name::str("C"),
                    Node::new(arrow(Expr::BVar(1), sort_v())),
                    Node::new(Expr::Pi(
                        BinderInfo::Implicit,
                        Name::str("a"),
                        Node::new(Expr::BVar(2)),
                        Node::new(Expr::Pi(
                            BinderInfo::Default,
                            Name::str("acc"),
                            Node::new(Expr::App(
                                Node::new(Expr::App(
                                    Node::new(Expr::Const(Name::str("Acc"), vec![])),
                                    Node::new(Expr::BVar(3)),
                                )),
                                Node::new(Expr::BVar(0)),
                            )),
                            Node::new(Expr::Pi(
                                BinderInfo::Default,
                                Name::str("step"),
                                Node::new(arrow(Expr::BVar(3), arrow(prop(), sort_v()))),
                                Node::new(Expr::App(
                                    Node::new(Expr::BVar(3)),
                                    Node::new(Expr::BVar(2)),
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
        Node::new(sort_u()),
        Node::new(Expr::Pi(
            BinderInfo::Implicit,
            Name::str("r"),
            Node::new(rel_ty(0)),
            Node::new(Expr::Pi(
                BinderInfo::Implicit,
                Name::str("C"),
                Node::new(arrow(Expr::BVar(1), sort_v())),
                Node::new(arrow(
                    Expr::App(
                        Node::new(Expr::Const(Name::str("WellFounded"), vec![])),
                        Node::new(Expr::BVar(1)),
                    ),
                    arrow(
                        Expr::Pi(
                            BinderInfo::Default,
                            Name::str("x"),
                            Node::new(Expr::BVar(3)),
                            Node::new(arrow(
                                Expr::Pi(
                                    BinderInfo::Default,
                                    Name::str("y"),
                                    Node::new(Expr::BVar(4)),
                                    Node::new(arrow(
                                        Expr::App(
                                            Node::new(Expr::App(
                                                Node::new(Expr::BVar(4)),
                                                Node::new(Expr::BVar(0)),
                                            )),
                                            Node::new(Expr::BVar(1)),
                                        ),
                                        Expr::App(
                                            Node::new(Expr::BVar(3)),
                                            Node::new(Expr::BVar(0)),
                                        ),
                                    )),
                                ),
                                Expr::App(Node::new(Expr::BVar(3)), Node::new(Expr::BVar(1))),
                            )),
                        ),
                        Expr::Pi(
                            BinderInfo::Default,
                            Name::str("a"),
                            Node::new(Expr::BVar(4)),
                            Node::new(Expr::App(
                                Node::new(Expr::App(
                                    Node::new(Expr::App(
                                        Node::new(Expr::Const(Name::str("Eq"), vec![])),
                                        Node::new(Expr::App(
                                            Node::new(Expr::BVar(3)),
                                            Node::new(Expr::BVar(0)),
                                        )),
                                    )),
                                    Node::new(Expr::App(
                                        Node::new(Expr::App(
                                            Node::new(Expr::App(
                                                Node::new(Expr::Const(
                                                    Name::str("WellFounded.fix"),
                                                    vec![],
                                                )),
                                                Node::new(Expr::BVar(2)),
                                            )),
                                            Node::new(Expr::BVar(1)),
                                        )),
                                        Node::new(Expr::BVar(0)),
                                    )),
                                )),
                                Node::new(Expr::App(
                                    Node::new(Expr::App(
                                        Node::new(Expr::BVar(1)),
                                        Node::new(Expr::BVar(0)),
                                    )),
                                    Node::new(Expr::Lam(
                                        BinderInfo::Default,
                                        Name::str("y"),
                                        Node::new(Expr::BVar(5)),
                                        Node::new(Expr::Lam(
                                            BinderInfo::Default,
                                            Name::str("hy"),
                                            Node::new(Expr::App(
                                                Node::new(Expr::App(
                                                    Node::new(Expr::BVar(6)),
                                                    Node::new(Expr::BVar(1)),
                                                )),
                                                Node::new(Expr::BVar(2)),
                                            )),
                                            Node::new(Expr::App(
                                                Node::new(Expr::App(
                                                    Node::new(Expr::App(
                                                        Node::new(Expr::Const(
                                                            Name::str("WellFounded.fix"),
                                                            vec![],
                                                        )),
                                                        Node::new(Expr::BVar(4)),
                                                    )),
                                                    Node::new(Expr::BVar(3)),
                                                )),
                                                Node::new(Expr::BVar(1)),
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
        Node::new(sort_u()),
        Node::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("f"),
            Node::new(arrow(Expr::BVar(0), nat_const())),
            Node::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("x"),
                Node::new(Expr::BVar(1)),
                Node::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("y"),
                    Node::new(Expr::BVar(2)),
                    Node::new(arrow(
                        Expr::App(
                            Node::new(Expr::App(
                                Node::new(Expr::Const(Name::str("Nat.lt"), vec![])),
                                Node::new(Expr::App(
                                    Node::new(Expr::BVar(2)),
                                    Node::new(Expr::BVar(1)),
                                )),
                            )),
                            Node::new(Expr::App(
                                Node::new(Expr::BVar(2)),
                                Node::new(Expr::BVar(0)),
                            )),
                        ),
                        Expr::App(
                            Node::new(Expr::App(
                                Node::new(Expr::App(
                                    Node::new(Expr::Const(Name::str("Measure"), vec![])),
                                    Node::new(Expr::BVar(3)),
                                )),
                                Node::new(Expr::BVar(2)),
                            )),
                            Node::new(Expr::BVar(1)),
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
        Node::new(nat_const()),
        Node::new(Expr::App(
            Node::new(Expr::App(
                Node::new(Expr::Const(Name::str("Acc"), vec![])),
                Node::new(Expr::Const(Name::str("Nat.lt"), vec![])),
            )),
            Node::new(Expr::BVar(0)),
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
        Node::new(nat_const()),
        Node::new(Expr::App(
            Node::new(Expr::App(
                Node::new(Expr::Const(Name::str("Eq"), vec![])),
                Node::new(Expr::App(
                    Node::new(Expr::App(
                        Node::new(Expr::Const(Name::str("sizeOf"), vec![])),
                        Node::new(nat_const()),
                    )),
                    Node::new(Expr::BVar(0)),
                )),
            )),
            Node::new(Expr::BVar(0)),
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
        Node::new(sort_u()),
        Node::new(Expr::Pi(
            BinderInfo::Implicit,
            Name::str("beta"),
            Node::new(sort_v()),
            Node::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("a"),
                Node::new(Expr::BVar(1)),
                Node::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("b"),
                    Node::new(Expr::BVar(1)),
                    Node::new(Expr::App(
                        Node::new(Expr::App(
                            Node::new(Expr::App(
                                Node::new(Expr::Const(Name::str("Eq"), vec![])),
                                Node::new(nat_const()),
                            )),
                            Node::new(Expr::App(
                                Node::new(Expr::Const(Name::str("sizeOf"), vec![])),
                                Node::new(Expr::App(
                                    Node::new(Expr::App(
                                        Node::new(Expr::Const(Name::str("Prod.mk"), vec![])),
                                        Node::new(Expr::BVar(1)),
                                    )),
                                    Node::new(Expr::BVar(0)),
                                )),
                            )),
                        )),
                        Node::new(Expr::App(
                            Node::new(Expr::App(
                                Node::new(Expr::Const(Name::str("Nat.add"), vec![])),
                                Node::new(Expr::App(
                                    Node::new(Expr::App(
                                        Node::new(Expr::Const(Name::str("Nat.add"), vec![])),
                                        Node::new(Expr::App(
                                            Node::new(Expr::Const(Name::str("Nat.succ"), vec![])),
                                            Node::new(Expr::Const(Name::str("Nat.zero"), vec![])),
                                        )),
                                    )),
                                    Node::new(Expr::App(
                                        Node::new(Expr::Const(Name::str("sizeOf"), vec![])),
                                        Node::new(Expr::BVar(1)),
                                    )),
                                )),
                            )),
                            Node::new(Expr::App(
                                Node::new(Expr::Const(Name::str("sizeOf"), vec![])),
                                Node::new(Expr::BVar(0)),
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
            Node::new(type1()),
            Node::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("x"),
                Node::new(Expr::BVar(0)),
                Node::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("y"),
                    Node::new(Expr::BVar(1)),
                    Node::new(prop()),
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
