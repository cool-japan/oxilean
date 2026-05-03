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
pub(super) fn pi(bi: BinderInfo, name: &str, dom: Expr, body: Expr) -> Expr {
    Expr::Pi(bi, Name::str(name), Box::new(dom), Box::new(body))
}
#[allow(dead_code)]
pub fn lam(bi: BinderInfo, name: &str, dom: Expr, body: Expr) -> Expr {
    Expr::Lam(bi, Name::str(name), Box::new(dom), Box::new(body))
}
pub fn cst(s: &str) -> Expr {
    Expr::Const(Name::str(s), vec![])
}
pub(super) fn prop() -> Expr {
    Expr::Sort(Level::zero())
}
pub(super) fn type0() -> Expr {
    Expr::Sort(Level::succ(Level::zero()))
}
pub fn type1() -> Expr {
    Expr::Sort(Level::succ(Level::succ(Level::zero())))
}
pub(super) fn bvar(n: u32) -> Expr {
    Expr::BVar(n)
}
pub(super) fn add_axiom(
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
/// Build `Type → Type 1` (the type of a type class).
#[allow(dead_code)]
pub fn mk_class_ty() -> Expr {
    pi(BinderInfo::Default, "α", type0(), type1())
}
/// Build a binary relation method type:
/// `{α : Type} → \[Class α\] → α → α → Prop`
#[allow(dead_code)]
pub fn mk_relation_method(class: &str) -> Expr {
    pi(
        BinderInfo::Implicit,
        "α",
        type0(),
        pi(
            BinderInfo::InstImplicit,
            "_inst",
            app(cst(class), bvar(0)),
            pi(
                BinderInfo::Default,
                "a",
                bvar(1),
                pi(BinderInfo::Default, "b", bvar(2), prop()),
            ),
        ),
    )
}
/// Build a binary operation method type:
/// `{α : Type} → \[Class α\] → α → α → α`
#[allow(dead_code)]
pub fn mk_binop_method(class: &str) -> Expr {
    pi(
        BinderInfo::Implicit,
        "α",
        type0(),
        pi(
            BinderInfo::InstImplicit,
            "_inst",
            app(cst(class), bvar(0)),
            pi(
                BinderInfo::Default,
                "a",
                bvar(1),
                pi(BinderInfo::Default, "b", bvar(2), bvar(3)),
            ),
        ),
    )
}
/// Build a nullary method type: `{α : Type} → \[Class α\] → α`
#[allow(dead_code)]
pub fn mk_nullary_method(class: &str) -> Expr {
    pi(
        BinderInfo::Implicit,
        "α",
        type0(),
        pi(
            BinderInfo::InstImplicit,
            "_inst",
            app(cst(class), bvar(0)),
            bvar(1),
        ),
    )
}
/// Build `Eq @{} α a b`.
#[allow(dead_code)]
pub fn mk_eq(ty: Expr, lhs: Expr, rhs: Expr) -> Expr {
    app3(cst("Eq"), ty, lhs, rhs)
}
/// Build a law with ∀ (a : α):
/// `{α : Type} → \[Class α\] → ∀ (a : α), <prop>`
#[allow(dead_code)]
pub fn mk_law_forall1<F>(class: &str, prop_builder: F) -> Expr
where
    F: FnOnce(u32) -> Expr,
{
    pi(
        BinderInfo::Implicit,
        "α",
        type0(),
        pi(
            BinderInfo::InstImplicit,
            "_inst",
            app(cst(class), bvar(0)),
            pi(BinderInfo::Default, "a", bvar(1), prop_builder(3)),
        ),
    )
}
/// Build a law with ∀ (a b : α):
/// `{α : Type} → \[Class α\] → ∀ (a b : α), <prop>`
#[allow(dead_code)]
pub fn mk_law_forall2<F>(class: &str, prop_builder: F) -> Expr
where
    F: FnOnce(u32) -> Expr,
{
    pi(
        BinderInfo::Implicit,
        "α",
        type0(),
        pi(
            BinderInfo::InstImplicit,
            "_inst",
            app(cst(class), bvar(0)),
            pi(
                BinderInfo::Default,
                "a",
                bvar(1),
                pi(BinderInfo::Default, "b", bvar(2), prop_builder(4)),
            ),
        ),
    )
}
/// Build a law with ∀ (a b c : α):
/// `{α : Type} → \[Class α\] → ∀ (a b c : α), <prop>`
#[allow(dead_code)]
pub fn mk_law_forall3<F>(class: &str, prop_builder: F) -> Expr
where
    F: FnOnce(u32) -> Expr,
{
    pi(
        BinderInfo::Implicit,
        "α",
        type0(),
        pi(
            BinderInfo::InstImplicit,
            "_inst",
            app(cst(class), bvar(0)),
            pi(
                BinderInfo::Default,
                "a",
                bvar(1),
                pi(
                    BinderInfo::Default,
                    "b",
                    bvar(2),
                    pi(BinderInfo::Default, "c", bvar(3), prop_builder(5)),
                ),
            ),
        ),
    )
}
/// Build an "extends" projection:
/// `{α : Type} → \[Class α\] → ParentClass α`
#[allow(dead_code)]
pub fn mk_extends(class: &str, parent: &str) -> Expr {
    pi(
        BinderInfo::Implicit,
        "α",
        type0(),
        pi(
            BinderInfo::InstImplicit,
            "_inst",
            app(cst(class), bvar(0)),
            app(cst(parent), bvar(1)),
        ),
    )
}
/// Build `LE.le a b`.
#[allow(dead_code)]
pub fn mk_le(a: Expr, b: Expr) -> Expr {
    app2(cst("LE.le"), a, b)
}
/// Build `LT.lt a b`.
#[allow(dead_code)]
pub fn mk_lt(a: Expr, b: Expr) -> Expr {
    app2(cst("LT.lt"), a, b)
}
/// Build `Sup.sup a b`.
#[allow(dead_code)]
pub fn mk_sup(a: Expr, b: Expr) -> Expr {
    app2(cst("Sup.sup"), a, b)
}
/// Build `Inf.inf a b`.
#[allow(dead_code)]
pub fn mk_inf(a: Expr, b: Expr) -> Expr {
    app2(cst("Inf.inf"), a, b)
}
/// Build `Min.min a b`.
#[allow(dead_code)]
pub fn mk_min(a: Expr, b: Expr) -> Expr {
    app2(cst("Min.min"), a, b)
}
/// Build `Max.max a b`.
#[allow(dead_code)]
pub fn mk_max(a: Expr, b: Expr) -> Expr {
    app2(cst("Max.max"), a, b)
}
/// Build `Monotone f` as an expression application.
#[allow(dead_code)]
pub fn mk_monotone(f: Expr) -> Expr {
    app(cst("Monotone"), f)
}
/// Build the order environment with all order-theoretic type classes and laws.
#[allow(clippy::too_many_lines)]
pub fn build_order_env(env: &mut Environment) -> Result<(), String> {
    add_axiom(env, "LE", vec![], mk_class_ty())?;
    add_axiom(env, "LE.le", vec![], mk_relation_method("LE"))?;
    add_axiom(env, "LT", vec![], mk_class_ty())?;
    add_axiom(env, "LT.lt", vec![], mk_relation_method("LT"))?;
    add_axiom(env, "Preorder", vec![], mk_class_ty())?;
    add_axiom(env, "Preorder.toLE", vec![], mk_extends("Preorder", "LE"))?;
    add_axiom(env, "Preorder.toLT", vec![], mk_extends("Preorder", "LT"))?;
    add_axiom(
        env,
        "le_refl",
        vec![],
        mk_law_forall1("Preorder", |_depth| {
            let a = bvar(0);
            app2(cst("LE.le"), a.clone(), a)
        }),
    )?;
    add_axiom(
        env,
        "le_trans",
        vec![],
        mk_law_forall3("Preorder", |_depth| {
            let a = bvar(2);
            let b = bvar(1);
            let c = bvar(0);
            pi(
                BinderInfo::Default,
                "hab",
                app2(cst("LE.le"), a.clone(), b.clone()),
                pi(
                    BinderInfo::Default,
                    "hbc",
                    app2(cst("LE.le"), b, c.clone()),
                    app2(cst("LE.le"), a, c),
                ),
            )
        }),
    )?;
    add_axiom(env, "PartialOrder", vec![], mk_class_ty())?;
    add_axiom(
        env,
        "PartialOrder.toPreorder",
        vec![],
        mk_extends("PartialOrder", "Preorder"),
    )?;
    add_axiom(
        env,
        "le_antisymm",
        vec![],
        mk_law_forall2("PartialOrder", |depth| {
            let alpha = bvar(depth - 1);
            let a = bvar(1);
            let b = bvar(0);
            pi(
                BinderInfo::Default,
                "hab",
                app2(cst("LE.le"), a.clone(), b.clone()),
                pi(
                    BinderInfo::Default,
                    "hba",
                    app2(cst("LE.le"), b.clone(), a.clone()),
                    mk_eq(alpha, a, b),
                ),
            )
        }),
    )?;
    add_axiom(env, "LinearOrder", vec![], mk_class_ty())?;
    add_axiom(
        env,
        "LinearOrder.toPartialOrder",
        vec![],
        mk_extends("LinearOrder", "PartialOrder"),
    )?;
    add_axiom(
        env,
        "le_total",
        vec![],
        mk_law_forall2("LinearOrder", |_depth| {
            let a = bvar(1);
            let b = bvar(0);
            app2(
                cst("Or"),
                app2(cst("LE.le"), a.clone(), b.clone()),
                app2(cst("LE.le"), b, a),
            )
        }),
    )?;
    add_axiom(env, "DecidableLinearOrder", vec![], mk_class_ty())?;
    add_axiom(
        env,
        "DecidableLinearOrder.toLinearOrder",
        vec![],
        mk_extends("DecidableLinearOrder", "LinearOrder"),
    )?;
    add_axiom(
        env,
        "DecidableLinearOrder.decidableEq",
        vec![],
        mk_relation_method("DecidableLinearOrder"),
    )?;
    add_axiom(
        env,
        "DecidableLinearOrder.decidableLe",
        vec![],
        mk_relation_method("DecidableLinearOrder"),
    )?;
    add_axiom(
        env,
        "DecidableLinearOrder.decidableLt",
        vec![],
        mk_relation_method("DecidableLinearOrder"),
    )?;
    add_axiom(env, "Sup", vec![], mk_class_ty())?;
    add_axiom(env, "Sup.sup", vec![], mk_binop_method("Sup"))?;
    add_axiom(env, "Inf", vec![], mk_class_ty())?;
    add_axiom(env, "Inf.inf", vec![], mk_binop_method("Inf"))?;
    add_axiom(env, "SemilatticeSup", vec![], mk_class_ty())?;
    add_axiom(
        env,
        "SemilatticeSup.toPartialOrder",
        vec![],
        mk_extends("SemilatticeSup", "PartialOrder"),
    )?;
    add_axiom(
        env,
        "SemilatticeSup.toSup",
        vec![],
        mk_extends("SemilatticeSup", "Sup"),
    )?;
    add_axiom(
        env,
        "le_sup_left",
        vec![],
        mk_law_forall2("SemilatticeSup", |_depth| {
            let a = bvar(1);
            let b = bvar(0);
            app2(cst("LE.le"), a.clone(), app2(cst("Sup.sup"), a, b))
        }),
    )?;
    add_axiom(
        env,
        "le_sup_right",
        vec![],
        mk_law_forall2("SemilatticeSup", |_depth| {
            let a = bvar(1);
            let b = bvar(0);
            app2(cst("LE.le"), b.clone(), app2(cst("Sup.sup"), a, b))
        }),
    )?;
    add_axiom(
        env,
        "sup_le",
        vec![],
        mk_law_forall3("SemilatticeSup", |_depth| {
            let a = bvar(2);
            let b = bvar(1);
            let c = bvar(0);
            pi(
                BinderInfo::Default,
                "hac",
                app2(cst("LE.le"), a.clone(), c.clone()),
                pi(
                    BinderInfo::Default,
                    "hbc",
                    app2(cst("LE.le"), b.clone(), c.clone()),
                    app2(cst("LE.le"), app2(cst("Sup.sup"), a, b), c),
                ),
            )
        }),
    )?;
    add_axiom(env, "SemilatticeInf", vec![], mk_class_ty())?;
    add_axiom(
        env,
        "SemilatticeInf.toPartialOrder",
        vec![],
        mk_extends("SemilatticeInf", "PartialOrder"),
    )?;
    add_axiom(
        env,
        "SemilatticeInf.toInf",
        vec![],
        mk_extends("SemilatticeInf", "Inf"),
    )?;
    add_axiom(
        env,
        "inf_le_left",
        vec![],
        mk_law_forall2("SemilatticeInf", |_depth| {
            let a = bvar(1);
            let b = bvar(0);
            app2(cst("LE.le"), app2(cst("Inf.inf"), a.clone(), b), a)
        }),
    )?;
    add_axiom(
        env,
        "inf_le_right",
        vec![],
        mk_law_forall2("SemilatticeInf", |_depth| {
            let a = bvar(1);
            let b = bvar(0);
            app2(cst("LE.le"), app2(cst("Inf.inf"), a, b.clone()), b)
        }),
    )?;
    add_axiom(
        env,
        "le_inf",
        vec![],
        mk_law_forall3("SemilatticeInf", |_depth| {
            let a = bvar(2);
            let b = bvar(1);
            let c = bvar(0);
            pi(
                BinderInfo::Default,
                "hab",
                app2(cst("LE.le"), a.clone(), b.clone()),
                pi(
                    BinderInfo::Default,
                    "hac",
                    app2(cst("LE.le"), a.clone(), c.clone()),
                    app2(cst("LE.le"), a, app2(cst("Inf.inf"), b, c)),
                ),
            )
        }),
    )?;
    add_axiom(env, "Lattice", vec![], mk_class_ty())?;
    add_axiom(
        env,
        "Lattice.toSemilatticeSup",
        vec![],
        mk_extends("Lattice", "SemilatticeSup"),
    )?;
    add_axiom(
        env,
        "Lattice.toSemilatticeInf",
        vec![],
        mk_extends("Lattice", "SemilatticeInf"),
    )?;
    add_axiom(
        env,
        "sup_comm",
        vec![],
        mk_law_forall2("Lattice", |depth| {
            let alpha = bvar(depth - 1);
            let a = bvar(1);
            let b = bvar(0);
            mk_eq(
                alpha,
                app2(cst("Sup.sup"), a.clone(), b.clone()),
                app2(cst("Sup.sup"), b, a),
            )
        }),
    )?;
    add_axiom(
        env,
        "inf_comm",
        vec![],
        mk_law_forall2("Lattice", |depth| {
            let alpha = bvar(depth - 1);
            let a = bvar(1);
            let b = bvar(0);
            mk_eq(
                alpha,
                app2(cst("Inf.inf"), a.clone(), b.clone()),
                app2(cst("Inf.inf"), b, a),
            )
        }),
    )?;
    add_axiom(
        env,
        "sup_assoc",
        vec![],
        mk_law_forall3("Lattice", |depth| {
            let alpha = bvar(depth - 1);
            let a = bvar(2);
            let b = bvar(1);
            let c = bvar(0);
            mk_eq(
                alpha,
                app2(
                    cst("Sup.sup"),
                    app2(cst("Sup.sup"), a.clone(), b.clone()),
                    c.clone(),
                ),
                app2(cst("Sup.sup"), a, app2(cst("Sup.sup"), b, c)),
            )
        }),
    )?;
    add_axiom(
        env,
        "inf_assoc",
        vec![],
        mk_law_forall3("Lattice", |depth| {
            let alpha = bvar(depth - 1);
            let a = bvar(2);
            let b = bvar(1);
            let c = bvar(0);
            mk_eq(
                alpha,
                app2(
                    cst("Inf.inf"),
                    app2(cst("Inf.inf"), a.clone(), b.clone()),
                    c.clone(),
                ),
                app2(cst("Inf.inf"), a, app2(cst("Inf.inf"), b, c)),
            )
        }),
    )?;
    add_axiom(
        env,
        "sup_inf_distrib",
        vec![],
        mk_law_forall3("Lattice", |depth| {
            let alpha = bvar(depth - 1);
            let a = bvar(2);
            let b = bvar(1);
            let c = bvar(0);
            mk_eq(
                alpha,
                app2(
                    cst("Sup.sup"),
                    a.clone(),
                    app2(cst("Inf.inf"), b.clone(), c.clone()),
                ),
                app2(
                    cst("Inf.inf"),
                    app2(cst("Sup.sup"), a.clone(), b),
                    app2(cst("Sup.sup"), a, c),
                ),
            )
        }),
    )?;
    add_axiom(
        env,
        "inf_sup_distrib",
        vec![],
        mk_law_forall3("Lattice", |depth| {
            let alpha = bvar(depth - 1);
            let a = bvar(2);
            let b = bvar(1);
            let c = bvar(0);
            mk_eq(
                alpha,
                app2(
                    cst("Inf.inf"),
                    a.clone(),
                    app2(cst("Sup.sup"), b.clone(), c.clone()),
                ),
                app2(
                    cst("Sup.sup"),
                    app2(cst("Inf.inf"), a.clone(), b),
                    app2(cst("Inf.inf"), a, c),
                ),
            )
        }),
    )?;
    add_axiom(env, "BoundedOrder", vec![], mk_class_ty())?;
    add_axiom(
        env,
        "BoundedOrder.toPartialOrder",
        vec![],
        mk_extends("BoundedOrder", "PartialOrder"),
    )?;
    add_axiom(env, "Top", vec![], mk_class_ty())?;
    add_axiom(env, "Top.top", vec![], mk_nullary_method("Top"))?;
    add_axiom(env, "Bot", vec![], mk_class_ty())?;
    add_axiom(env, "Bot.bot", vec![], mk_nullary_method("Bot"))?;
    add_axiom(
        env,
        "BoundedOrder.toTop",
        vec![],
        mk_extends("BoundedOrder", "Top"),
    )?;
    add_axiom(
        env,
        "BoundedOrder.toBot",
        vec![],
        mk_extends("BoundedOrder", "Bot"),
    )?;
    add_axiom(
        env,
        "le_top",
        vec![],
        mk_law_forall1("BoundedOrder", |_depth| {
            let a = bvar(0);
            app2(cst("LE.le"), a, cst("Top.top"))
        }),
    )?;
    add_axiom(
        env,
        "bot_le",
        vec![],
        mk_law_forall1("BoundedOrder", |_depth| {
            let a = bvar(0);
            app2(cst("LE.le"), cst("Bot.bot"), a)
        }),
    )?;
    add_axiom(env, "CompleteLattice", vec![], mk_class_ty())?;
    add_axiom(
        env,
        "CompleteLattice.toLattice",
        vec![],
        mk_extends("CompleteLattice", "Lattice"),
    )?;
    add_axiom(
        env,
        "CompleteLattice.toBoundedOrder",
        vec![],
        mk_extends("CompleteLattice", "BoundedOrder"),
    )?;
    add_axiom(
        env,
        "CompleteLattice.sSup",
        vec![],
        pi(
            BinderInfo::Implicit,
            "α",
            type0(),
            pi(
                BinderInfo::InstImplicit,
                "_inst",
                app(cst("CompleteLattice"), bvar(0)),
                pi(
                    BinderInfo::Default,
                    "s",
                    pi(BinderInfo::Default, "_", bvar(1), prop()),
                    bvar(2),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "CompleteLattice.sInf",
        vec![],
        pi(
            BinderInfo::Implicit,
            "α",
            type0(),
            pi(
                BinderInfo::InstImplicit,
                "_inst",
                app(cst("CompleteLattice"), bvar(0)),
                pi(
                    BinderInfo::Default,
                    "s",
                    pi(BinderInfo::Default, "_", bvar(1), prop()),
                    bvar(2),
                ),
            ),
        ),
    )?;
    add_axiom(env, "Min", vec![], mk_class_ty())?;
    add_axiom(env, "Min.min", vec![], mk_binop_method("Min"))?;
    add_axiom(env, "Max", vec![], mk_class_ty())?;
    add_axiom(env, "Max.max", vec![], mk_binop_method("Max"))?;
    add_axiom(
        env,
        "min_le_left",
        vec![],
        mk_law_forall2("Min", |_depth| {
            let a = bvar(1);
            let b = bvar(0);
            app2(cst("LE.le"), app2(cst("Min.min"), a.clone(), b), a)
        }),
    )?;
    add_axiom(
        env,
        "min_le_right",
        vec![],
        mk_law_forall2("Min", |_depth| {
            let a = bvar(1);
            let b = bvar(0);
            app2(cst("LE.le"), app2(cst("Min.min"), a, b.clone()), b)
        }),
    )?;
    add_axiom(
        env,
        "le_max_left",
        vec![],
        mk_law_forall2("Max", |_depth| {
            let a = bvar(1);
            let b = bvar(0);
            app2(cst("LE.le"), a.clone(), app2(cst("Max.max"), a, b))
        }),
    )?;
    add_axiom(
        env,
        "le_max_right",
        vec![],
        mk_law_forall2("Max", |_depth| {
            let a = bvar(1);
            let b = bvar(0);
            app2(cst("LE.le"), b.clone(), app2(cst("Max.max"), a, b))
        }),
    )?;
    add_axiom(
        env,
        "Monotone",
        vec![],
        pi(
            BinderInfo::Implicit,
            "α",
            type0(),
            pi(
                BinderInfo::Implicit,
                "β",
                type0(),
                pi(
                    BinderInfo::InstImplicit,
                    "_inst1",
                    app(cst("Preorder"), bvar(1)),
                    pi(
                        BinderInfo::InstImplicit,
                        "_inst2",
                        app(cst("Preorder"), bvar(1)),
                        pi(
                            BinderInfo::Default,
                            "f",
                            pi(BinderInfo::Default, "_", bvar(3), bvar(2)),
                            prop(),
                        ),
                    ),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "Antitone",
        vec![],
        pi(
            BinderInfo::Implicit,
            "α",
            type0(),
            pi(
                BinderInfo::Implicit,
                "β",
                type0(),
                pi(
                    BinderInfo::InstImplicit,
                    "_inst1",
                    app(cst("Preorder"), bvar(1)),
                    pi(
                        BinderInfo::InstImplicit,
                        "_inst2",
                        app(cst("Preorder"), bvar(1)),
                        pi(
                            BinderInfo::Default,
                            "f",
                            pi(BinderInfo::Default, "_", bvar(3), bvar(2)),
                            prop(),
                        ),
                    ),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "monotone_id",
        vec![],
        pi(
            BinderInfo::Implicit,
            "α",
            type0(),
            pi(
                BinderInfo::InstImplicit,
                "_inst",
                app(cst("Preorder"), bvar(0)),
                app(
                    cst("Monotone"),
                    lam(BinderInfo::Default, "x", bvar(1), bvar(0)),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "monotone_comp",
        vec![],
        pi(
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
                        "_inst1",
                        app(cst("Preorder"), bvar(2)),
                        pi(
                            BinderInfo::InstImplicit,
                            "_inst2",
                            app(cst("Preorder"), bvar(2)),
                            pi(
                                BinderInfo::InstImplicit,
                                "_inst3",
                                app(cst("Preorder"), bvar(2)),
                                pi(
                                    BinderInfo::Implicit,
                                    "f",
                                    pi(BinderInfo::Default, "_", bvar(5), bvar(5)),
                                    pi(
                                        BinderInfo::Implicit,
                                        "g",
                                        pi(BinderInfo::Default, "_", bvar(5), bvar(5)),
                                        pi(
                                            BinderInfo::Default,
                                            "hf",
                                            app(cst("Monotone"), bvar(1)),
                                            pi(
                                                BinderInfo::Default,
                                                "hg",
                                                app(cst("Monotone"), bvar(1)),
                                                app(
                                                    cst("Monotone"),
                                                    lam(
                                                        BinderInfo::Default,
                                                        "x",
                                                        bvar(9),
                                                        app(bvar(3), app(bvar(4), bvar(0))),
                                                    ),
                                                ),
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    )?;
    Ok(())
}
#[allow(dead_code)]
pub(super) fn ord2_ext_app(f: Expr, a: Expr) -> Expr {
    Expr::App(Box::new(f), Box::new(a))
}
#[allow(dead_code)]
pub(super) fn ord2_ext_app2(f: Expr, a: Expr, b: Expr) -> Expr {
    ord2_ext_app(ord2_ext_app(f, a), b)
}
#[allow(dead_code)]
pub fn ord2_ext_app3(f: Expr, a: Expr, b: Expr, c: Expr) -> Expr {
    ord2_ext_app(ord2_ext_app2(f, a, b), c)
}
#[allow(dead_code)]
pub(super) fn ord2_ext_cst(s: &str) -> Expr {
    Expr::Const(Name::str(s), vec![])
}
#[allow(dead_code)]
pub(super) fn ord2_ext_prop() -> Expr {
    Expr::Sort(Level::zero())
}
#[allow(dead_code)]
pub(super) fn ord2_ext_type0() -> Expr {
    Expr::Sort(Level::succ(Level::zero()))
}
#[allow(dead_code)]
pub(super) fn ord2_ext_bvar(n: u32) -> Expr {
    Expr::BVar(n)
}
#[allow(dead_code)]
pub fn ord2_ext_nat_ty() -> Expr {
    ord2_ext_cst("Nat")
}
#[allow(dead_code)]
pub(super) fn ord2_ext_arrow(dom: Expr, cod: Expr) -> Expr {
    Expr::Pi(
        BinderInfo::Default,
        Name::Anonymous,
        Box::new(dom),
        Box::new(cod),
    )
}
#[allow(dead_code)]
pub(super) fn ord2_ext_pi(bi: BinderInfo, name: &str, dom: Expr, body: Expr) -> Expr {
    Expr::Pi(bi, Name::str(name), Box::new(dom), Box::new(body))
}
/// `{α : Type} → \[Class α\] → α → α → Prop`
#[allow(dead_code)]
pub fn ord2_ext_rel_method(class: &str) -> Expr {
    ord2_ext_pi(
        BinderInfo::Implicit,
        "α",
        ord2_ext_type0(),
        ord2_ext_pi(
            BinderInfo::InstImplicit,
            "_inst",
            ord2_ext_app(ord2_ext_cst(class), ord2_ext_bvar(0)),
            ord2_ext_pi(
                BinderInfo::Default,
                "a",
                ord2_ext_bvar(1),
                ord2_ext_pi(BinderInfo::Default, "b", ord2_ext_bvar(2), ord2_ext_prop()),
            ),
        ),
    )
}
/// `{α : Type} → \[Class α\] → α → α → α`
#[allow(dead_code)]
pub(super) fn ord2_ext_binop_method(class: &str) -> Expr {
    ord2_ext_pi(
        BinderInfo::Implicit,
        "α",
        ord2_ext_type0(),
        ord2_ext_pi(
            BinderInfo::InstImplicit,
            "_inst",
            ord2_ext_app(ord2_ext_cst(class), ord2_ext_bvar(0)),
            ord2_ext_pi(
                BinderInfo::Default,
                "a",
                ord2_ext_bvar(1),
                ord2_ext_pi(BinderInfo::Default, "b", ord2_ext_bvar(2), ord2_ext_bvar(3)),
            ),
        ),
    )
}
/// `{α : Type} → \[Class α\] → α`
#[allow(dead_code)]
pub fn ord2_ext_nullary_method(class: &str) -> Expr {
    ord2_ext_pi(
        BinderInfo::Implicit,
        "α",
        ord2_ext_type0(),
        ord2_ext_pi(
            BinderInfo::InstImplicit,
            "_inst",
            ord2_ext_app(ord2_ext_cst(class), ord2_ext_bvar(0)),
            ord2_ext_bvar(1),
        ),
    )
}
/// `Type → Type 1`
#[allow(dead_code)]
pub(super) fn ord2_ext_class_ty() -> Expr {
    ord2_ext_pi(
        BinderInfo::Default,
        "α",
        ord2_ext_type0(),
        Expr::Sort(Level::succ(Level::succ(Level::zero()))),
    )
}
/// `{α : Type} → \[C α\] → ParentClass α`
#[allow(dead_code)]
pub(super) fn ord2_ext_extends(class: &str, parent: &str) -> Expr {
    ord2_ext_pi(
        BinderInfo::Implicit,
        "α",
        ord2_ext_type0(),
        ord2_ext_pi(
            BinderInfo::InstImplicit,
            "_inst",
            ord2_ext_app(ord2_ext_cst(class), ord2_ext_bvar(0)),
            ord2_ext_app(ord2_ext_cst(parent), ord2_ext_bvar(1)),
        ),
    )
}
/// Law with ∀ (a : α).
#[allow(dead_code)]
pub(super) fn ord2_ext_law1<F>(class: &str, f: F) -> Expr
where
    F: FnOnce(u32) -> Expr,
{
    ord2_ext_pi(
        BinderInfo::Implicit,
        "α",
        ord2_ext_type0(),
        ord2_ext_pi(
            BinderInfo::InstImplicit,
            "_inst",
            ord2_ext_app(ord2_ext_cst(class), ord2_ext_bvar(0)),
            ord2_ext_pi(BinderInfo::Default, "a", ord2_ext_bvar(1), f(3)),
        ),
    )
}
/// Law with ∀ (a b : α).
#[allow(dead_code)]
pub(super) fn ord2_ext_law2<F>(class: &str, f: F) -> Expr
where
    F: FnOnce(u32) -> Expr,
{
    ord2_ext_pi(
        BinderInfo::Implicit,
        "α",
        ord2_ext_type0(),
        ord2_ext_pi(
            BinderInfo::InstImplicit,
            "_inst",
            ord2_ext_app(ord2_ext_cst(class), ord2_ext_bvar(0)),
            ord2_ext_pi(
                BinderInfo::Default,
                "a",
                ord2_ext_bvar(1),
                ord2_ext_pi(BinderInfo::Default, "b", ord2_ext_bvar(2), f(4)),
            ),
        ),
    )
}
/// Law with ∀ (a b c : α).
#[allow(dead_code)]
pub(super) fn ord2_ext_law3<F>(class: &str, f: F) -> Expr
where
    F: FnOnce(u32) -> Expr,
{
    ord2_ext_pi(
        BinderInfo::Implicit,
        "α",
        ord2_ext_type0(),
        ord2_ext_pi(
            BinderInfo::InstImplicit,
            "_inst",
            ord2_ext_app(ord2_ext_cst(class), ord2_ext_bvar(0)),
            ord2_ext_pi(
                BinderInfo::Default,
                "a",
                ord2_ext_bvar(1),
                ord2_ext_pi(
                    BinderInfo::Default,
                    "b",
                    ord2_ext_bvar(2),
                    ord2_ext_pi(BinderInfo::Default, "c", ord2_ext_bvar(3), f(5)),
                ),
            ),
        ),
    )
}
#[allow(dead_code)]
pub(super) fn ord2_ext_eq(ty: Expr, lhs: Expr, rhs: Expr) -> Expr {
    ord2_ext_app3(ord2_ext_cst("Eq"), ty, lhs, rhs)
}
#[allow(dead_code)]
pub fn ord2_ext_and(a: Expr, b: Expr) -> Expr {
    ord2_ext_app2(ord2_ext_cst("And"), a, b)
}
#[allow(dead_code)]
pub(super) fn ord2_ext_or(a: Expr, b: Expr) -> Expr {
    ord2_ext_app2(ord2_ext_cst("Or"), a, b)
}
#[allow(dead_code)]
pub(super) fn ord2_ext_not(p: Expr) -> Expr {
    ord2_ext_app(ord2_ext_cst("Not"), p)
}
#[allow(dead_code)]
pub(super) fn ord2_ext_le(a: Expr, b: Expr) -> Expr {
    ord2_ext_app2(ord2_ext_cst("LE.le"), a, b)
}
#[allow(dead_code)]
pub(super) fn ord2_ext_sup(a: Expr, b: Expr) -> Expr {
    ord2_ext_app2(ord2_ext_cst("Sup.sup"), a, b)
}
#[allow(dead_code)]
pub(super) fn ord2_ext_inf(a: Expr, b: Expr) -> Expr {
    ord2_ext_app2(ord2_ext_cst("Inf.inf"), a, b)
}
