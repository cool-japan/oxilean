//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::functions::*;
use super::types::*;

/// Register extended order-theory axioms (35+ declarations) into `env`.
#[allow(clippy::too_many_lines)]
pub fn register_order_extended_axioms(env: &mut Environment) -> Result<(), String> {
    add_axiom(env, "StrictOrder", vec![], ord2_ext_class_ty())?;
    add_axiom(
        env,
        "StrictOrder.toPreorder",
        vec![],
        ord2_ext_extends("StrictOrder", "Preorder"),
    )?;
    add_axiom(
        env,
        "lt_irrefl",
        vec![],
        ord2_ext_law1("StrictOrder", |_d| {
            let a = ord2_ext_bvar(0);
            ord2_ext_not(ord2_ext_app2(ord2_ext_cst("LT.lt"), a.clone(), a))
        }),
    )?;
    add_axiom(
        env,
        "lt_trans_strict",
        vec![],
        ord2_ext_law3("StrictOrder", |_d| {
            let a = ord2_ext_bvar(2);
            let b = ord2_ext_bvar(1);
            let c = ord2_ext_bvar(0);
            let lt = |x: Expr, y: Expr| ord2_ext_app2(ord2_ext_cst("LT.lt"), x, y);
            ord2_ext_pi(
                BinderInfo::Default,
                "hab",
                lt(a.clone(), b.clone()),
                ord2_ext_pi(BinderInfo::Default, "hbc", lt(b, c.clone()), lt(a, c)),
            )
        }),
    )?;
    add_axiom(env, "TotalOrder", vec![], ord2_ext_class_ty())?;
    add_axiom(
        env,
        "TotalOrder.toLinearOrder",
        vec![],
        ord2_ext_extends("TotalOrder", "LinearOrder"),
    )?;
    add_axiom(
        env,
        "lt_trichotomy",
        vec![],
        ord2_ext_law2("TotalOrder", |depth| {
            let alpha = ord2_ext_bvar(depth - 1);
            let a = ord2_ext_bvar(1);
            let b = ord2_ext_bvar(0);
            let lt = |x: Expr, y: Expr| ord2_ext_app2(ord2_ext_cst("LT.lt"), x, y);
            ord2_ext_or(
                lt(a.clone(), b.clone()),
                ord2_ext_or(ord2_ext_eq(alpha, a.clone(), b.clone()), lt(b, a)),
            )
        }),
    )?;
    add_axiom(
        env,
        "sup_inf_absorb",
        vec![],
        ord2_ext_law2("Lattice", |depth| {
            let alpha = ord2_ext_bvar(depth - 1);
            let a = ord2_ext_bvar(1);
            let b = ord2_ext_bvar(0);
            ord2_ext_eq(
                alpha,
                ord2_ext_sup(a.clone(), ord2_ext_inf(a.clone(), b)),
                a,
            )
        }),
    )?;
    add_axiom(
        env,
        "inf_sup_absorb",
        vec![],
        ord2_ext_law2("Lattice", |depth| {
            let alpha = ord2_ext_bvar(depth - 1);
            let a = ord2_ext_bvar(1);
            let b = ord2_ext_bvar(0);
            ord2_ext_eq(
                alpha,
                ord2_ext_inf(a.clone(), ord2_ext_sup(a.clone(), b)),
                a,
            )
        }),
    )?;
    add_axiom(
        env,
        "sup_idem",
        vec![],
        ord2_ext_law1("Lattice", |depth| {
            let alpha = ord2_ext_bvar(depth - 1);
            let a = ord2_ext_bvar(0);
            ord2_ext_eq(alpha, ord2_ext_sup(a.clone(), a.clone()), a)
        }),
    )?;
    add_axiom(
        env,
        "inf_idem",
        vec![],
        ord2_ext_law1("Lattice", |depth| {
            let alpha = ord2_ext_bvar(depth - 1);
            let a = ord2_ext_bvar(0);
            ord2_ext_eq(alpha, ord2_ext_inf(a.clone(), a.clone()), a)
        }),
    )?;
    add_axiom(env, "BoolAlg", vec![], ord2_ext_class_ty())?;
    add_axiom(
        env,
        "BoolAlg.toLattice",
        vec![],
        ord2_ext_extends("BoolAlg", "Lattice"),
    )?;
    add_axiom(
        env,
        "BoolAlg.toBoundedOrder",
        vec![],
        ord2_ext_extends("BoolAlg", "BoundedOrder"),
    )?;
    add_axiom(
        env,
        "BoolAlg.compl",
        vec![],
        ord2_ext_pi(
            BinderInfo::Implicit,
            "α",
            ord2_ext_type0(),
            ord2_ext_pi(
                BinderInfo::InstImplicit,
                "_inst",
                ord2_ext_app(ord2_ext_cst("BoolAlg"), ord2_ext_bvar(0)),
                ord2_ext_pi(BinderInfo::Default, "a", ord2_ext_bvar(1), ord2_ext_bvar(2)),
            ),
        ),
    )?;
    add_axiom(
        env,
        "sup_compl_top",
        vec![],
        ord2_ext_law1("BoolAlg", |depth| {
            let alpha = ord2_ext_bvar(depth - 1);
            let a = ord2_ext_bvar(0);
            ord2_ext_eq(
                alpha,
                ord2_ext_sup(a.clone(), ord2_ext_app(ord2_ext_cst("BoolAlg.compl"), a)),
                ord2_ext_cst("Top.top"),
            )
        }),
    )?;
    add_axiom(
        env,
        "inf_compl_bot",
        vec![],
        ord2_ext_law1("BoolAlg", |depth| {
            let alpha = ord2_ext_bvar(depth - 1);
            let a = ord2_ext_bvar(0);
            ord2_ext_eq(
                alpha,
                ord2_ext_inf(a.clone(), ord2_ext_app(ord2_ext_cst("BoolAlg.compl"), a)),
                ord2_ext_cst("Bot.bot"),
            )
        }),
    )?;
    add_axiom(
        env,
        "de_morgan_sup",
        vec![],
        ord2_ext_law2("BoolAlg", |depth| {
            let alpha = ord2_ext_bvar(depth - 1);
            let a = ord2_ext_bvar(1);
            let b = ord2_ext_bvar(0);
            let compl = |x: Expr| ord2_ext_app(ord2_ext_cst("BoolAlg.compl"), x);
            ord2_ext_eq(
                alpha,
                compl(ord2_ext_sup(a.clone(), b.clone())),
                ord2_ext_inf(compl(a), compl(b)),
            )
        }),
    )?;
    add_axiom(
        env,
        "de_morgan_inf",
        vec![],
        ord2_ext_law2("BoolAlg", |depth| {
            let alpha = ord2_ext_bvar(depth - 1);
            let a = ord2_ext_bvar(1);
            let b = ord2_ext_bvar(0);
            let compl = |x: Expr| ord2_ext_app(ord2_ext_cst("BoolAlg.compl"), x);
            ord2_ext_eq(
                alpha,
                compl(ord2_ext_inf(a.clone(), b.clone())),
                ord2_ext_sup(compl(a), compl(b)),
            )
        }),
    )?;
    add_axiom(
        env,
        "bool_alg_double_neg",
        vec![],
        ord2_ext_law1("BoolAlg", |depth| {
            let alpha = ord2_ext_bvar(depth - 1);
            let a = ord2_ext_bvar(0);
            let compl = |x: Expr| ord2_ext_app(ord2_ext_cst("BoolAlg.compl"), x);
            ord2_ext_eq(alpha, compl(compl(a.clone())), a)
        }),
    )?;
    add_axiom(env, "HeytingAlg", vec![], ord2_ext_class_ty())?;
    add_axiom(
        env,
        "HeytingAlg.toLattice",
        vec![],
        ord2_ext_extends("HeytingAlg", "Lattice"),
    )?;
    add_axiom(
        env,
        "HeytingAlg.toBoundedOrder",
        vec![],
        ord2_ext_extends("HeytingAlg", "BoundedOrder"),
    )?;
    add_axiom(
        env,
        "HeytingAlg.himp",
        vec![],
        ord2_ext_binop_method("HeytingAlg"),
    )?;
    add_axiom(
        env,
        "himp_adjoint",
        vec![],
        ord2_ext_law3("HeytingAlg", |_d| {
            let a = ord2_ext_bvar(2);
            let b = ord2_ext_bvar(1);
            let c = ord2_ext_bvar(0);
            let himp = |x: Expr, y: Expr| ord2_ext_app2(ord2_ext_cst("HeytingAlg.himp"), x, y);
            ord2_ext_pi(
                BinderInfo::Default,
                "h1",
                ord2_ext_le(c.clone(), himp(a.clone(), b.clone())),
                ord2_ext_le(ord2_ext_inf(c, a), b),
            )
        }),
    )?;
    add_axiom(
        env,
        "IsChain",
        vec![],
        ord2_ext_pi(
            BinderInfo::Implicit,
            "α",
            ord2_ext_type0(),
            ord2_ext_pi(
                BinderInfo::InstImplicit,
                "_inst",
                ord2_ext_app(ord2_ext_cst("Preorder"), ord2_ext_bvar(0)),
                ord2_ext_pi(
                    BinderInfo::Default,
                    "s",
                    ord2_ext_arrow(ord2_ext_bvar(1), ord2_ext_prop()),
                    ord2_ext_prop(),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "IsAntichain",
        vec![],
        ord2_ext_pi(
            BinderInfo::Implicit,
            "α",
            ord2_ext_type0(),
            ord2_ext_pi(
                BinderInfo::InstImplicit,
                "_inst",
                ord2_ext_app(ord2_ext_cst("Preorder"), ord2_ext_bvar(0)),
                ord2_ext_pi(
                    BinderInfo::Default,
                    "s",
                    ord2_ext_arrow(ord2_ext_bvar(1), ord2_ext_prop()),
                    ord2_ext_prop(),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "IsMaximal",
        vec![],
        ord2_ext_pi(
            BinderInfo::Implicit,
            "α",
            ord2_ext_type0(),
            ord2_ext_pi(
                BinderInfo::InstImplicit,
                "_inst",
                ord2_ext_app(ord2_ext_cst("PartialOrder"), ord2_ext_bvar(0)),
                ord2_ext_pi(BinderInfo::Default, "a", ord2_ext_bvar(1), ord2_ext_prop()),
            ),
        ),
    )?;
    add_axiom(
        env,
        "IsMinimal",
        vec![],
        ord2_ext_pi(
            BinderInfo::Implicit,
            "α",
            ord2_ext_type0(),
            ord2_ext_pi(
                BinderInfo::InstImplicit,
                "_inst",
                ord2_ext_app(ord2_ext_cst("PartialOrder"), ord2_ext_bvar(0)),
                ord2_ext_pi(BinderInfo::Default, "a", ord2_ext_bvar(1), ord2_ext_prop()),
            ),
        ),
    )?;
    add_axiom(
        env,
        "IsGreatest",
        vec![],
        ord2_ext_pi(
            BinderInfo::Implicit,
            "α",
            ord2_ext_type0(),
            ord2_ext_pi(
                BinderInfo::InstImplicit,
                "_inst",
                ord2_ext_app(ord2_ext_cst("Preorder"), ord2_ext_bvar(0)),
                ord2_ext_pi(
                    BinderInfo::Default,
                    "s",
                    ord2_ext_arrow(ord2_ext_bvar(1), ord2_ext_prop()),
                    ord2_ext_pi(BinderInfo::Default, "a", ord2_ext_bvar(2), ord2_ext_prop()),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "IsLeast",
        vec![],
        ord2_ext_pi(
            BinderInfo::Implicit,
            "α",
            ord2_ext_type0(),
            ord2_ext_pi(
                BinderInfo::InstImplicit,
                "_inst",
                ord2_ext_app(ord2_ext_cst("Preorder"), ord2_ext_bvar(0)),
                ord2_ext_pi(
                    BinderInfo::Default,
                    "s",
                    ord2_ext_arrow(ord2_ext_bvar(1), ord2_ext_prop()),
                    ord2_ext_pi(BinderInfo::Default, "a", ord2_ext_bvar(2), ord2_ext_prop()),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "upperBounds",
        vec![],
        ord2_ext_pi(
            BinderInfo::Implicit,
            "α",
            ord2_ext_type0(),
            ord2_ext_pi(
                BinderInfo::InstImplicit,
                "_inst",
                ord2_ext_app(ord2_ext_cst("Preorder"), ord2_ext_bvar(0)),
                ord2_ext_pi(
                    BinderInfo::Default,
                    "s",
                    ord2_ext_arrow(ord2_ext_bvar(1), ord2_ext_prop()),
                    ord2_ext_arrow(ord2_ext_bvar(2), ord2_ext_prop()),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "lowerBounds",
        vec![],
        ord2_ext_pi(
            BinderInfo::Implicit,
            "α",
            ord2_ext_type0(),
            ord2_ext_pi(
                BinderInfo::InstImplicit,
                "_inst",
                ord2_ext_app(ord2_ext_cst("Preorder"), ord2_ext_bvar(0)),
                ord2_ext_pi(
                    BinderInfo::Default,
                    "s",
                    ord2_ext_arrow(ord2_ext_bvar(1), ord2_ext_prop()),
                    ord2_ext_arrow(ord2_ext_bvar(2), ord2_ext_prop()),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "BddAbove",
        vec![],
        ord2_ext_pi(
            BinderInfo::Implicit,
            "α",
            ord2_ext_type0(),
            ord2_ext_pi(
                BinderInfo::InstImplicit,
                "_inst",
                ord2_ext_app(ord2_ext_cst("Preorder"), ord2_ext_bvar(0)),
                ord2_ext_pi(
                    BinderInfo::Default,
                    "s",
                    ord2_ext_arrow(ord2_ext_bvar(1), ord2_ext_prop()),
                    ord2_ext_prop(),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "BddBelow",
        vec![],
        ord2_ext_pi(
            BinderInfo::Implicit,
            "α",
            ord2_ext_type0(),
            ord2_ext_pi(
                BinderInfo::InstImplicit,
                "_inst",
                ord2_ext_app(ord2_ext_cst("Preorder"), ord2_ext_bvar(0)),
                ord2_ext_pi(
                    BinderInfo::Default,
                    "s",
                    ord2_ext_arrow(ord2_ext_bvar(1), ord2_ext_prop()),
                    ord2_ext_prop(),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "IsLUB",
        vec![],
        ord2_ext_pi(
            BinderInfo::Implicit,
            "α",
            ord2_ext_type0(),
            ord2_ext_pi(
                BinderInfo::InstImplicit,
                "_inst",
                ord2_ext_app(ord2_ext_cst("Preorder"), ord2_ext_bvar(0)),
                ord2_ext_pi(
                    BinderInfo::Default,
                    "s",
                    ord2_ext_arrow(ord2_ext_bvar(1), ord2_ext_prop()),
                    ord2_ext_pi(BinderInfo::Default, "a", ord2_ext_bvar(2), ord2_ext_prop()),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "IsGLB",
        vec![],
        ord2_ext_pi(
            BinderInfo::Implicit,
            "α",
            ord2_ext_type0(),
            ord2_ext_pi(
                BinderInfo::InstImplicit,
                "_inst",
                ord2_ext_app(ord2_ext_cst("Preorder"), ord2_ext_bvar(0)),
                ord2_ext_pi(
                    BinderInfo::Default,
                    "s",
                    ord2_ext_arrow(ord2_ext_bvar(1), ord2_ext_prop()),
                    ord2_ext_pi(BinderInfo::Default, "a", ord2_ext_bvar(2), ord2_ext_prop()),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "OrderHom",
        vec![],
        ord2_ext_pi(
            BinderInfo::Default,
            "α",
            ord2_ext_type0(),
            ord2_ext_pi(BinderInfo::Default, "β", ord2_ext_type0(), ord2_ext_type0()),
        ),
    )?;
    add_axiom(
        env,
        "OrderHom.toFun",
        vec![],
        ord2_ext_pi(
            BinderInfo::Implicit,
            "α",
            ord2_ext_type0(),
            ord2_ext_pi(
                BinderInfo::Implicit,
                "β",
                ord2_ext_type0(),
                ord2_ext_pi(
                    BinderInfo::Default,
                    "f",
                    ord2_ext_app2(ord2_ext_cst("OrderHom"), ord2_ext_bvar(1), ord2_ext_bvar(0)),
                    ord2_ext_arrow(ord2_ext_bvar(2), ord2_ext_bvar(2)),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "OrderHom.monotone",
        vec![],
        ord2_ext_pi(
            BinderInfo::Implicit,
            "α",
            ord2_ext_type0(),
            ord2_ext_pi(
                BinderInfo::Implicit,
                "β",
                ord2_ext_type0(),
                ord2_ext_pi(
                    BinderInfo::Default,
                    "f",
                    ord2_ext_app2(ord2_ext_cst("OrderHom"), ord2_ext_bvar(1), ord2_ext_bvar(0)),
                    ord2_ext_prop(),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "OrderIso",
        vec![],
        ord2_ext_pi(
            BinderInfo::Default,
            "α",
            ord2_ext_type0(),
            ord2_ext_pi(BinderInfo::Default, "β", ord2_ext_type0(), ord2_ext_type0()),
        ),
    )?;
    add_axiom(
        env,
        "OrderIso.toOrderHom",
        vec![],
        ord2_ext_pi(
            BinderInfo::Implicit,
            "α",
            ord2_ext_type0(),
            ord2_ext_pi(
                BinderInfo::Implicit,
                "β",
                ord2_ext_type0(),
                ord2_ext_pi(
                    BinderInfo::Default,
                    "e",
                    ord2_ext_app2(ord2_ext_cst("OrderIso"), ord2_ext_bvar(1), ord2_ext_bvar(0)),
                    ord2_ext_app2(ord2_ext_cst("OrderHom"), ord2_ext_bvar(2), ord2_ext_bvar(1)),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "GaloisConnection",
        vec![],
        ord2_ext_pi(
            BinderInfo::Implicit,
            "α",
            ord2_ext_type0(),
            ord2_ext_pi(
                BinderInfo::Implicit,
                "β",
                ord2_ext_type0(),
                ord2_ext_pi(
                    BinderInfo::Default,
                    "l",
                    ord2_ext_arrow(ord2_ext_bvar(1), ord2_ext_bvar(0)),
                    ord2_ext_pi(
                        BinderInfo::Default,
                        "u",
                        ord2_ext_arrow(ord2_ext_bvar(1), ord2_ext_bvar(2)),
                        ord2_ext_prop(),
                    ),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "galois_connection_iff",
        vec![],
        ord2_ext_pi(
            BinderInfo::Implicit,
            "α",
            ord2_ext_type0(),
            ord2_ext_pi(
                BinderInfo::Implicit,
                "β",
                ord2_ext_type0(),
                ord2_ext_pi(
                    BinderInfo::Default,
                    "l",
                    ord2_ext_arrow(ord2_ext_bvar(1), ord2_ext_bvar(0)),
                    ord2_ext_pi(
                        BinderInfo::Default,
                        "u",
                        ord2_ext_arrow(ord2_ext_bvar(1), ord2_ext_bvar(2)),
                        ord2_ext_prop(),
                    ),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "gc_l_monotone",
        vec![],
        ord2_ext_pi(
            BinderInfo::Implicit,
            "α",
            ord2_ext_type0(),
            ord2_ext_pi(
                BinderInfo::Implicit,
                "β",
                ord2_ext_type0(),
                ord2_ext_pi(
                    BinderInfo::Default,
                    "l",
                    ord2_ext_arrow(ord2_ext_bvar(1), ord2_ext_bvar(0)),
                    ord2_ext_pi(
                        BinderInfo::Default,
                        "u",
                        ord2_ext_arrow(ord2_ext_bvar(1), ord2_ext_bvar(2)),
                        ord2_ext_pi(
                            BinderInfo::Default,
                            "h",
                            ord2_ext_app2(
                                ord2_ext_cst("GaloisConnection"),
                                ord2_ext_bvar(1),
                                ord2_ext_bvar(0),
                            ),
                            ord2_ext_app(ord2_ext_cst("Monotone"), ord2_ext_bvar(2)),
                        ),
                    ),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "IsFixedPoint",
        vec![],
        ord2_ext_pi(
            BinderInfo::Implicit,
            "α",
            ord2_ext_type0(),
            ord2_ext_pi(
                BinderInfo::Default,
                "f",
                ord2_ext_arrow(ord2_ext_bvar(0), ord2_ext_bvar(0)),
                ord2_ext_pi(BinderInfo::Default, "x", ord2_ext_bvar(1), ord2_ext_prop()),
            ),
        ),
    )?;
    add_axiom(
        env,
        "tarski_fixed_point",
        vec![],
        ord2_ext_pi(
            BinderInfo::Implicit,
            "α",
            ord2_ext_type0(),
            ord2_ext_pi(
                BinderInfo::InstImplicit,
                "_inst",
                ord2_ext_app(ord2_ext_cst("CompleteLattice"), ord2_ext_bvar(0)),
                ord2_ext_pi(
                    BinderInfo::Default,
                    "f",
                    ord2_ext_arrow(ord2_ext_bvar(1), ord2_ext_bvar(1)),
                    ord2_ext_pi(
                        BinderInfo::Default,
                        "_hm",
                        ord2_ext_app(ord2_ext_cst("Monotone"), ord2_ext_bvar(0)),
                        ord2_ext_app2(
                            ord2_ext_cst("Exists"),
                            ord2_ext_bvar(2),
                            ord2_ext_app(ord2_ext_cst("IsFixedPoint"), ord2_ext_bvar(1)),
                        ),
                    ),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "knaster_tarski_lfp",
        vec![],
        ord2_ext_pi(
            BinderInfo::Implicit,
            "α",
            ord2_ext_type0(),
            ord2_ext_pi(
                BinderInfo::InstImplicit,
                "_inst",
                ord2_ext_app(ord2_ext_cst("CompleteLattice"), ord2_ext_bvar(0)),
                ord2_ext_pi(
                    BinderInfo::Default,
                    "f",
                    ord2_ext_arrow(ord2_ext_bvar(1), ord2_ext_bvar(1)),
                    ord2_ext_prop(),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "WellFounded",
        vec![],
        ord2_ext_pi(
            BinderInfo::Implicit,
            "α",
            ord2_ext_type0(),
            ord2_ext_pi(
                BinderInfo::Default,
                "r",
                ord2_ext_pi(
                    BinderInfo::Default,
                    "_",
                    ord2_ext_bvar(0),
                    ord2_ext_pi(BinderInfo::Default, "_", ord2_ext_bvar(1), ord2_ext_prop()),
                ),
                ord2_ext_prop(),
            ),
        ),
    )?;
    add_axiom(env, "WellOrder", vec![], ord2_ext_class_ty())?;
    add_axiom(
        env,
        "WellOrder.toLinearOrder",
        vec![],
        ord2_ext_extends("WellOrder", "LinearOrder"),
    )?;
    add_axiom(
        env,
        "WellOrder.wf",
        vec![],
        ord2_ext_pi(
            BinderInfo::Implicit,
            "α",
            ord2_ext_type0(),
            ord2_ext_pi(
                BinderInfo::InstImplicit,
                "_inst",
                ord2_ext_app(ord2_ext_cst("WellOrder"), ord2_ext_bvar(0)),
                ord2_ext_app(ord2_ext_cst("WellFounded"), ord2_ext_cst("LT.lt")),
            ),
        ),
    )?;
    add_axiom(
        env,
        "IsOrderIdeal",
        vec![],
        ord2_ext_pi(
            BinderInfo::Implicit,
            "α",
            ord2_ext_type0(),
            ord2_ext_pi(
                BinderInfo::InstImplicit,
                "_inst",
                ord2_ext_app(ord2_ext_cst("PartialOrder"), ord2_ext_bvar(0)),
                ord2_ext_pi(
                    BinderInfo::Default,
                    "I",
                    ord2_ext_arrow(ord2_ext_bvar(1), ord2_ext_prop()),
                    ord2_ext_prop(),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "IsFilter",
        vec![],
        ord2_ext_pi(
            BinderInfo::Implicit,
            "α",
            ord2_ext_type0(),
            ord2_ext_pi(
                BinderInfo::InstImplicit,
                "_inst",
                ord2_ext_app(ord2_ext_cst("PartialOrder"), ord2_ext_bvar(0)),
                ord2_ext_pi(
                    BinderInfo::Default,
                    "F",
                    ord2_ext_arrow(ord2_ext_bvar(1), ord2_ext_prop()),
                    ord2_ext_prop(),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "zorn_lemma",
        vec![],
        ord2_ext_pi(
            BinderInfo::Implicit,
            "α",
            ord2_ext_type0(),
            ord2_ext_pi(
                BinderInfo::InstImplicit,
                "_inst",
                ord2_ext_app(ord2_ext_cst("PartialOrder"), ord2_ext_bvar(0)),
                ord2_ext_pi(
                    BinderInfo::Default,
                    "hc",
                    ord2_ext_prop(),
                    ord2_ext_app2(
                        ord2_ext_cst("Exists"),
                        ord2_ext_bvar(1),
                        ord2_ext_app(ord2_ext_cst("IsMaximal"), ord2_ext_cst("id")),
                    ),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "IsDirected",
        vec![],
        ord2_ext_pi(
            BinderInfo::Implicit,
            "α",
            ord2_ext_type0(),
            ord2_ext_pi(
                BinderInfo::InstImplicit,
                "_inst",
                ord2_ext_app(ord2_ext_cst("Preorder"), ord2_ext_bvar(0)),
                ord2_ext_pi(
                    BinderInfo::Default,
                    "d",
                    ord2_ext_arrow(ord2_ext_bvar(1), ord2_ext_prop()),
                    ord2_ext_prop(),
                ),
            ),
        ),
    )?;
    add_axiom(env, "DCPO", vec![], ord2_ext_class_ty())?;
    add_axiom(
        env,
        "DCPO.toPartialOrder",
        vec![],
        ord2_ext_extends("DCPO", "PartialOrder"),
    )?;
    add_axiom(
        env,
        "DCPO.directedSup",
        vec![],
        ord2_ext_pi(
            BinderInfo::Implicit,
            "α",
            ord2_ext_type0(),
            ord2_ext_pi(
                BinderInfo::InstImplicit,
                "_inst",
                ord2_ext_app(ord2_ext_cst("DCPO"), ord2_ext_bvar(0)),
                ord2_ext_pi(
                    BinderInfo::Default,
                    "d",
                    ord2_ext_arrow(ord2_ext_bvar(1), ord2_ext_prop()),
                    ord2_ext_bvar(2),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "dcpo_directed_sup_is_ub",
        vec![],
        ord2_ext_pi(
            BinderInfo::Implicit,
            "α",
            ord2_ext_type0(),
            ord2_ext_pi(
                BinderInfo::InstImplicit,
                "_inst",
                ord2_ext_app(ord2_ext_cst("DCPO"), ord2_ext_bvar(0)),
                ord2_ext_prop(),
            ),
        ),
    )?;
    add_axiom(
        env,
        "ScottOpen",
        vec![],
        ord2_ext_pi(
            BinderInfo::Implicit,
            "α",
            ord2_ext_type0(),
            ord2_ext_pi(
                BinderInfo::InstImplicit,
                "_inst",
                ord2_ext_app(ord2_ext_cst("DCPO"), ord2_ext_bvar(0)),
                ord2_ext_pi(
                    BinderInfo::Default,
                    "u",
                    ord2_ext_arrow(ord2_ext_bvar(1), ord2_ext_prop()),
                    ord2_ext_prop(),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "AlexandrovOpen",
        vec![],
        ord2_ext_pi(
            BinderInfo::Implicit,
            "α",
            ord2_ext_type0(),
            ord2_ext_pi(
                BinderInfo::InstImplicit,
                "_inst",
                ord2_ext_app(ord2_ext_cst("Preorder"), ord2_ext_bvar(0)),
                ord2_ext_pi(
                    BinderInfo::Default,
                    "u",
                    ord2_ext_arrow(ord2_ext_bvar(1), ord2_ext_prop()),
                    ord2_ext_prop(),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "scott_open_upward_closed",
        vec![],
        ord2_ext_pi(
            BinderInfo::Implicit,
            "α",
            ord2_ext_type0(),
            ord2_ext_pi(
                BinderInfo::InstImplicit,
                "_inst",
                ord2_ext_app(ord2_ext_cst("DCPO"), ord2_ext_bvar(0)),
                ord2_ext_pi(
                    BinderInfo::Default,
                    "u",
                    ord2_ext_arrow(ord2_ext_bvar(1), ord2_ext_prop()),
                    ord2_ext_pi(
                        BinderInfo::Default,
                        "_h",
                        ord2_ext_app(ord2_ext_cst("ScottOpen"), ord2_ext_bvar(0)),
                        ord2_ext_prop(),
                    ),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "IsMooreFamily",
        vec![],
        ord2_ext_pi(
            BinderInfo::Implicit,
            "α",
            ord2_ext_type0(),
            ord2_ext_pi(
                BinderInfo::InstImplicit,
                "_inst",
                ord2_ext_app(ord2_ext_cst("CompleteLattice"), ord2_ext_bvar(0)),
                ord2_ext_pi(
                    BinderInfo::Default,
                    "M",
                    ord2_ext_arrow(
                        ord2_ext_arrow(ord2_ext_bvar(1), ord2_ext_prop()),
                        ord2_ext_prop(),
                    ),
                    ord2_ext_prop(),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "ClosureOp",
        vec![],
        ord2_ext_pi(
            BinderInfo::Implicit,
            "α",
            ord2_ext_type0(),
            ord2_ext_pi(
                BinderInfo::InstImplicit,
                "_inst",
                ord2_ext_app(ord2_ext_cst("Preorder"), ord2_ext_bvar(0)),
                ord2_ext_pi(
                    BinderInfo::Default,
                    "c",
                    ord2_ext_arrow(ord2_ext_bvar(1), ord2_ext_bvar(1)),
                    ord2_ext_prop(),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "closure_op_extensive",
        vec![],
        ord2_ext_pi(
            BinderInfo::Implicit,
            "α",
            ord2_ext_type0(),
            ord2_ext_pi(
                BinderInfo::InstImplicit,
                "_inst",
                ord2_ext_app(ord2_ext_cst("Preorder"), ord2_ext_bvar(0)),
                ord2_ext_pi(
                    BinderInfo::Default,
                    "c",
                    ord2_ext_arrow(ord2_ext_bvar(1), ord2_ext_bvar(1)),
                    ord2_ext_pi(
                        BinderInfo::Default,
                        "_hc",
                        ord2_ext_app(ord2_ext_cst("ClosureOp"), ord2_ext_bvar(0)),
                        ord2_ext_pi(
                            BinderInfo::Default,
                            "a",
                            ord2_ext_bvar(2),
                            ord2_ext_le(
                                ord2_ext_bvar(0),
                                ord2_ext_app(ord2_ext_bvar(2), ord2_ext_bvar(0)),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "closure_op_idempotent",
        vec![],
        ord2_ext_pi(
            BinderInfo::Implicit,
            "α",
            ord2_ext_type0(),
            ord2_ext_pi(
                BinderInfo::InstImplicit,
                "_inst",
                ord2_ext_app(ord2_ext_cst("Preorder"), ord2_ext_bvar(0)),
                ord2_ext_pi(
                    BinderInfo::Default,
                    "c",
                    ord2_ext_arrow(ord2_ext_bvar(1), ord2_ext_bvar(1)),
                    ord2_ext_pi(
                        BinderInfo::Default,
                        "_hc",
                        ord2_ext_app(ord2_ext_cst("ClosureOp"), ord2_ext_bvar(0)),
                        ord2_ext_pi(BinderInfo::Default, "a", ord2_ext_bvar(2), ord2_ext_prop()),
                    ),
                ),
            ),
        ),
    )?;
    add_axiom(env, "ContinuousLattice", vec![], ord2_ext_class_ty())?;
    add_axiom(
        env,
        "ContinuousLattice.toCompleteLattice",
        vec![],
        ord2_ext_extends("ContinuousLattice", "CompleteLattice"),
    )?;
    add_axiom(
        env,
        "ContinuousLattice.toDCPO",
        vec![],
        ord2_ext_extends("ContinuousLattice", "DCPO"),
    )?;
    add_axiom(
        env,
        "continuous_lattice_interpolation",
        vec![],
        ord2_ext_pi(
            BinderInfo::Implicit,
            "α",
            ord2_ext_type0(),
            ord2_ext_pi(
                BinderInfo::InstImplicit,
                "_inst",
                ord2_ext_app(ord2_ext_cst("ContinuousLattice"), ord2_ext_bvar(0)),
                ord2_ext_prop(),
            ),
        ),
    )?;
    add_axiom(env, "DistribLattice", vec![], ord2_ext_class_ty())?;
    add_axiom(
        env,
        "DistribLattice.toLattice",
        vec![],
        ord2_ext_extends("DistribLattice", "Lattice"),
    )?;
    add_axiom(
        env,
        "distrib_sup_inf",
        vec![],
        ord2_ext_law3("DistribLattice", |depth| {
            let alpha = ord2_ext_bvar(depth - 1);
            let a = ord2_ext_bvar(2);
            let b = ord2_ext_bvar(1);
            let c = ord2_ext_bvar(0);
            ord2_ext_eq(
                alpha,
                ord2_ext_sup(a.clone(), ord2_ext_inf(b.clone(), c.clone())),
                ord2_ext_inf(ord2_ext_sup(a.clone(), b), ord2_ext_sup(a, c)),
            )
        }),
    )?;
    if env.get(&Name::str("And")).is_none() {
        let and_ty = ord2_ext_pi(
            BinderInfo::Default,
            "a",
            ord2_ext_prop(),
            ord2_ext_pi(BinderInfo::Default, "b", ord2_ext_prop(), ord2_ext_prop()),
        );
        add_axiom(env, "And", vec![], and_ty)?;
    }
    if env.get(&Name::str("Not")).is_none() {
        let not_ty = ord2_ext_pi(BinderInfo::Default, "a", ord2_ext_prop(), ord2_ext_prop());
        add_axiom(env, "Not", vec![], not_ty)?;
    }
    if env.get(&Name::str("Exists")).is_none() {
        let exists_ty = ord2_ext_pi(
            BinderInfo::Implicit,
            "α",
            ord2_ext_type0(),
            ord2_ext_pi(
                BinderInfo::Default,
                "p",
                ord2_ext_arrow(ord2_ext_bvar(0), ord2_ext_prop()),
                ord2_ext_prop(),
            ),
        );
        add_axiom(env, "Exists", vec![], exists_ty)?;
    }
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    fn setup_env() -> Environment {
        let mut env = Environment::new();
        let eq_ty = pi(
            BinderInfo::Implicit,
            "α",
            type0(),
            pi(
                BinderInfo::Default,
                "a",
                bvar(0),
                pi(BinderInfo::Default, "b", bvar(1), prop()),
            ),
        );
        add_axiom(&mut env, "Eq", vec![], eq_ty).expect("operation should succeed");
        let or_ty = pi(
            BinderInfo::Default,
            "a",
            prop(),
            pi(BinderInfo::Default, "b", prop(), prop()),
        );
        add_axiom(&mut env, "Or", vec![], or_ty).expect("operation should succeed");
        build_order_env(&mut env).expect("build_order_env should succeed");
        env
    }
    #[test]
    fn test_build_order_env_succeeds() {
        let _env = setup_env();
    }
    #[test]
    fn test_le_class() {
        let env = setup_env();
        assert!(env.get(&Name::str("LE")).is_some());
        assert!(env.get(&Name::str("LE.le")).is_some());
    }
    #[test]
    fn test_lt_class() {
        let env = setup_env();
        assert!(env.get(&Name::str("LT")).is_some());
        assert!(env.get(&Name::str("LT.lt")).is_some());
    }
    #[test]
    fn test_preorder() {
        let env = setup_env();
        assert!(env.get(&Name::str("Preorder")).is_some());
        assert!(env.get(&Name::str("Preorder.toLE")).is_some());
        assert!(env.get(&Name::str("Preorder.toLT")).is_some());
        assert!(env.get(&Name::str("le_refl")).is_some());
        assert!(env.get(&Name::str("le_trans")).is_some());
    }
    #[test]
    fn test_partial_order() {
        let env = setup_env();
        assert!(env.get(&Name::str("PartialOrder")).is_some());
        assert!(env.get(&Name::str("PartialOrder.toPreorder")).is_some());
        assert!(env.get(&Name::str("le_antisymm")).is_some());
    }
    #[test]
    fn test_linear_order() {
        let env = setup_env();
        assert!(env.get(&Name::str("LinearOrder")).is_some());
        assert!(env.get(&Name::str("LinearOrder.toPartialOrder")).is_some());
        assert!(env.get(&Name::str("le_total")).is_some());
    }
    #[test]
    fn test_decidable_linear_order() {
        let env = setup_env();
        assert!(env.get(&Name::str("DecidableLinearOrder")).is_some());
        assert!(env
            .get(&Name::str("DecidableLinearOrder.toLinearOrder"))
            .is_some());
        assert!(env
            .get(&Name::str("DecidableLinearOrder.decidableEq"))
            .is_some());
        assert!(env
            .get(&Name::str("DecidableLinearOrder.decidableLe"))
            .is_some());
        assert!(env
            .get(&Name::str("DecidableLinearOrder.decidableLt"))
            .is_some());
    }
    #[test]
    fn test_sup_class() {
        let env = setup_env();
        assert!(env.get(&Name::str("Sup")).is_some());
        assert!(env.get(&Name::str("Sup.sup")).is_some());
    }
    #[test]
    fn test_inf_class() {
        let env = setup_env();
        assert!(env.get(&Name::str("Inf")).is_some());
        assert!(env.get(&Name::str("Inf.inf")).is_some());
    }
    #[test]
    fn test_semilattice_sup() {
        let env = setup_env();
        assert!(env.get(&Name::str("SemilatticeSup")).is_some());
        assert!(env
            .get(&Name::str("SemilatticeSup.toPartialOrder"))
            .is_some());
        assert!(env.get(&Name::str("SemilatticeSup.toSup")).is_some());
        assert!(env.get(&Name::str("le_sup_left")).is_some());
        assert!(env.get(&Name::str("le_sup_right")).is_some());
        assert!(env.get(&Name::str("sup_le")).is_some());
    }
    #[test]
    fn test_semilattice_inf() {
        let env = setup_env();
        assert!(env.get(&Name::str("SemilatticeInf")).is_some());
        assert!(env
            .get(&Name::str("SemilatticeInf.toPartialOrder"))
            .is_some());
        assert!(env.get(&Name::str("SemilatticeInf.toInf")).is_some());
        assert!(env.get(&Name::str("inf_le_left")).is_some());
        assert!(env.get(&Name::str("inf_le_right")).is_some());
        assert!(env.get(&Name::str("le_inf")).is_some());
    }
    #[test]
    fn test_lattice() {
        let env = setup_env();
        assert!(env.get(&Name::str("Lattice")).is_some());
        assert!(env.get(&Name::str("Lattice.toSemilatticeSup")).is_some());
        assert!(env.get(&Name::str("Lattice.toSemilatticeInf")).is_some());
    }
    #[test]
    fn test_lattice_laws() {
        let env = setup_env();
        assert!(env.get(&Name::str("sup_comm")).is_some());
        assert!(env.get(&Name::str("inf_comm")).is_some());
        assert!(env.get(&Name::str("sup_assoc")).is_some());
        assert!(env.get(&Name::str("inf_assoc")).is_some());
        assert!(env.get(&Name::str("sup_inf_distrib")).is_some());
        assert!(env.get(&Name::str("inf_sup_distrib")).is_some());
    }
    #[test]
    fn test_bounded_order() {
        let env = setup_env();
        assert!(env.get(&Name::str("BoundedOrder")).is_some());
        assert!(env.get(&Name::str("BoundedOrder.toPartialOrder")).is_some());
        assert!(env.get(&Name::str("Top")).is_some());
        assert!(env.get(&Name::str("Top.top")).is_some());
        assert!(env.get(&Name::str("Bot")).is_some());
        assert!(env.get(&Name::str("Bot.bot")).is_some());
        assert!(env.get(&Name::str("BoundedOrder.toTop")).is_some());
        assert!(env.get(&Name::str("BoundedOrder.toBot")).is_some());
        assert!(env.get(&Name::str("le_top")).is_some());
        assert!(env.get(&Name::str("bot_le")).is_some());
    }
    #[test]
    fn test_complete_lattice() {
        let env = setup_env();
        assert!(env.get(&Name::str("CompleteLattice")).is_some());
        assert!(env.get(&Name::str("CompleteLattice.toLattice")).is_some());
        assert!(env
            .get(&Name::str("CompleteLattice.toBoundedOrder"))
            .is_some());
        assert!(env.get(&Name::str("CompleteLattice.sSup")).is_some());
        assert!(env.get(&Name::str("CompleteLattice.sInf")).is_some());
    }
    #[test]
    fn test_min_max() {
        let env = setup_env();
        assert!(env.get(&Name::str("Min")).is_some());
        assert!(env.get(&Name::str("Min.min")).is_some());
        assert!(env.get(&Name::str("Max")).is_some());
        assert!(env.get(&Name::str("Max.max")).is_some());
    }
    #[test]
    fn test_min_max_laws() {
        let env = setup_env();
        assert!(env.get(&Name::str("min_le_left")).is_some());
        assert!(env.get(&Name::str("min_le_right")).is_some());
        assert!(env.get(&Name::str("le_max_left")).is_some());
        assert!(env.get(&Name::str("le_max_right")).is_some());
    }
    #[test]
    fn test_monotone_antitone() {
        let env = setup_env();
        assert!(env.get(&Name::str("Monotone")).is_some());
        assert!(env.get(&Name::str("Antitone")).is_some());
        assert!(env.get(&Name::str("monotone_id")).is_some());
        assert!(env.get(&Name::str("monotone_comp")).is_some());
    }
    #[test]
    fn test_order_class_types() {
        let env = setup_env();
        let classes = [
            "LE",
            "LT",
            "Preorder",
            "PartialOrder",
            "LinearOrder",
            "DecidableLinearOrder",
            "Sup",
            "Inf",
            "SemilatticeSup",
            "SemilatticeInf",
            "Lattice",
            "BoundedOrder",
            "CompleteLattice",
            "Min",
            "Max",
            "Top",
            "Bot",
        ];
        for class_name in &classes {
            let decl = env.get(&Name::str(*class_name));
            assert!(decl.is_some(), "Missing class: {}", class_name);
            if let Some(Declaration::Axiom { ty, .. }) = decl {
                assert!(
                    matches!(ty, Expr::Pi(BinderInfo::Default, _, _, _)),
                    "Class {} should be Pi type, got {:?}",
                    class_name,
                    ty
                );
            }
        }
    }
    #[test]
    fn test_relation_method_types() {
        let env = setup_env();
        let methods = ["LE.le", "LT.lt"];
        for method_name in &methods {
            let decl = env.get(&Name::str(*method_name));
            assert!(decl.is_some(), "Missing method: {}", method_name);
            if let Some(Declaration::Axiom { ty, .. }) = decl {
                assert!(
                    matches!(ty, Expr::Pi(BinderInfo::Implicit, _, _, _)),
                    "Method {} should start with implicit binder",
                    method_name,
                );
            }
        }
    }
    #[test]
    fn test_binop_methods() {
        let env = setup_env();
        let methods = ["Sup.sup", "Inf.inf", "Min.min", "Max.max"];
        for method_name in &methods {
            let decl = env.get(&Name::str(*method_name));
            assert!(decl.is_some(), "Missing method: {}", method_name);
            if let Some(Declaration::Axiom { ty, .. }) = decl {
                assert!(
                    matches!(ty, Expr::Pi(BinderInfo::Implicit, _, _, _)),
                    "Method {} should start with implicit binder",
                    method_name,
                );
            }
        }
    }
    #[test]
    fn test_mk_le_produces_app() {
        let a = bvar(0);
        let b = bvar(1);
        let result = mk_le(a, b);
        assert!(matches!(result, Expr::App(_, _)));
    }
    #[test]
    fn test_mk_lt_produces_app() {
        let a = bvar(0);
        let b = bvar(1);
        let result = mk_lt(a, b);
        assert!(matches!(result, Expr::App(_, _)));
    }
    #[test]
    fn test_mk_sup_produces_app() {
        let a = bvar(0);
        let b = bvar(1);
        let result = mk_sup(a, b);
        assert!(matches!(result, Expr::App(_, _)));
    }
    #[test]
    fn test_mk_inf_produces_app() {
        let a = bvar(0);
        let b = bvar(1);
        let result = mk_inf(a, b);
        assert!(matches!(result, Expr::App(_, _)));
    }
    #[test]
    fn test_mk_min_produces_app() {
        let a = bvar(0);
        let b = bvar(1);
        let result = mk_min(a, b);
        assert!(matches!(result, Expr::App(_, _)));
    }
    #[test]
    fn test_mk_max_produces_app() {
        let a = bvar(0);
        let b = bvar(1);
        let result = mk_max(a, b);
        assert!(matches!(result, Expr::App(_, _)));
    }
    #[test]
    fn test_mk_monotone_produces_app() {
        let f = bvar(0);
        let result = mk_monotone(f);
        assert!(matches!(result, Expr::App(_, _)));
    }
    #[test]
    fn test_le_refl_type_structure() {
        let env = setup_env();
        let decl = env
            .get(&Name::str("le_refl"))
            .expect("declaration 'le_refl' should exist in env");
        if let Declaration::Axiom { ty, .. } = decl {
            assert!(matches!(ty, Expr::Pi(BinderInfo::Implicit, _, _, _)));
        } else {
            panic!("le_refl should be an Axiom");
        }
    }
    #[test]
    fn test_le_trans_type_structure() {
        let env = setup_env();
        let decl = env
            .get(&Name::str("le_trans"))
            .expect("declaration 'le_trans' should exist in env");
        if let Declaration::Axiom { ty, .. } = decl {
            assert!(matches!(ty, Expr::Pi(BinderInfo::Implicit, _, _, _)));
        } else {
            panic!("le_trans should be an Axiom");
        }
    }
    #[test]
    fn test_le_antisymm_type_structure() {
        let env = setup_env();
        let decl = env
            .get(&Name::str("le_antisymm"))
            .expect("declaration 'le_antisymm' should exist in env");
        if let Declaration::Axiom { ty, .. } = decl {
            assert!(matches!(ty, Expr::Pi(BinderInfo::Implicit, _, _, _)));
        } else {
            panic!("le_antisymm should be an Axiom");
        }
    }
    #[test]
    fn test_monotone_type_structure() {
        let env = setup_env();
        let decl = env
            .get(&Name::str("Monotone"))
            .expect("declaration 'Monotone' should exist in env");
        if let Declaration::Axiom { ty, .. } = decl {
            assert!(matches!(ty, Expr::Pi(BinderInfo::Implicit, _, _, _)));
        } else {
            panic!("Monotone should be an Axiom");
        }
    }
    #[test]
    fn test_total_order_declarations() {
        let env = setup_env();
        let all_names = [
            "LE",
            "LE.le",
            "LT",
            "LT.lt",
            "Preorder",
            "Preorder.toLE",
            "Preorder.toLT",
            "le_refl",
            "le_trans",
            "PartialOrder",
            "PartialOrder.toPreorder",
            "le_antisymm",
            "LinearOrder",
            "LinearOrder.toPartialOrder",
            "le_total",
            "DecidableLinearOrder",
            "DecidableLinearOrder.toLinearOrder",
            "DecidableLinearOrder.decidableEq",
            "DecidableLinearOrder.decidableLe",
            "DecidableLinearOrder.decidableLt",
            "Sup",
            "Sup.sup",
            "Inf",
            "Inf.inf",
            "SemilatticeSup",
            "SemilatticeSup.toPartialOrder",
            "SemilatticeSup.toSup",
            "le_sup_left",
            "le_sup_right",
            "sup_le",
            "SemilatticeInf",
            "SemilatticeInf.toPartialOrder",
            "SemilatticeInf.toInf",
            "inf_le_left",
            "inf_le_right",
            "le_inf",
            "Lattice",
            "Lattice.toSemilatticeSup",
            "Lattice.toSemilatticeInf",
            "sup_comm",
            "inf_comm",
            "sup_assoc",
            "inf_assoc",
            "sup_inf_distrib",
            "inf_sup_distrib",
            "BoundedOrder",
            "BoundedOrder.toPartialOrder",
            "Top",
            "Top.top",
            "Bot",
            "Bot.bot",
            "BoundedOrder.toTop",
            "BoundedOrder.toBot",
            "le_top",
            "bot_le",
            "CompleteLattice",
            "CompleteLattice.toLattice",
            "CompleteLattice.toBoundedOrder",
            "CompleteLattice.sSup",
            "CompleteLattice.sInf",
            "Min",
            "Min.min",
            "Max",
            "Max.max",
            "min_le_left",
            "min_le_right",
            "le_max_left",
            "le_max_right",
            "Monotone",
            "Antitone",
            "monotone_id",
            "monotone_comp",
        ];
        for name in &all_names {
            assert!(
                env.get(&Name::str(*name)).is_some(),
                "Missing declaration: {}",
                name
            );
        }
    }
}
