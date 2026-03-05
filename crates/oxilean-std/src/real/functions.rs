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
pub(super) fn prop() -> Expr {
    Expr::Sort(Level::zero())
}
pub(super) fn type0() -> Expr {
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
pub(super) fn bvar(n: u32) -> Expr {
    Expr::BVar(n)
}
pub(super) fn arrow(a: Expr, b: Expr) -> Expr {
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
pub fn real_ty() -> Expr {
    Expr::Const(Name::str("Real"), vec![])
}
pub fn rat_ty() -> Expr {
    Expr::Const(Name::str("Rat"), vec![])
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
/// Build `Eq @{} ty a b`.
#[allow(dead_code)]
pub fn mk_eq(ty: Expr, lhs: Expr, rhs: Expr) -> Expr {
    app3(cst("Eq"), ty, lhs, rhs)
}
/// Build `And p q`.
#[allow(dead_code)]
pub fn mk_and(p: Expr, q: Expr) -> Expr {
    app2(cst("And"), p, q)
}
/// Build `Iff p q`.
#[allow(dead_code)]
pub fn mk_iff(p: Expr, q: Expr) -> Expr {
    app2(cst("Iff"), p, q)
}
/// Build `Or p q`.
#[allow(dead_code)]
pub fn mk_or(p: Expr, q: Expr) -> Expr {
    app2(cst("Or"), p, q)
}
/// Build `Not p`.
#[allow(dead_code)]
pub fn mk_not(p: Expr) -> Expr {
    app(cst("Not"), p)
}
/// Build `Exists @{} alpha pred`.
#[allow(dead_code)]
pub fn mk_exists(ty: Expr, pred: Expr) -> Expr {
    app2(cst("Exists"), ty, pred)
}
/// Build `Real.add a b`.
#[allow(dead_code)]
pub fn real_add(a: Expr, b: Expr) -> Expr {
    app2(cst("Real.add"), a, b)
}
/// Build `Real.mul a b`.
#[allow(dead_code)]
pub fn real_mul(a: Expr, b: Expr) -> Expr {
    app2(cst("Real.mul"), a, b)
}
/// Build `Real.neg a`.
#[allow(dead_code)]
pub fn real_neg(a: Expr) -> Expr {
    app(cst("Real.neg"), a)
}
/// Build `Real.inv a`.
#[allow(dead_code)]
pub fn real_inv(a: Expr) -> Expr {
    app(cst("Real.inv"), a)
}
/// Build `Real.sub a b`.
#[allow(dead_code)]
pub fn real_sub(a: Expr, b: Expr) -> Expr {
    app2(cst("Real.sub"), a, b)
}
/// Build `Real.div a b`.
#[allow(dead_code)]
pub fn real_div(a: Expr, b: Expr) -> Expr {
    app2(cst("Real.div"), a, b)
}
/// Build `Real.abs a`.
#[allow(dead_code)]
pub fn real_abs(a: Expr) -> Expr {
    app(cst("Real.abs"), a)
}
/// Build `Real.le a b`.
#[allow(dead_code)]
pub fn real_le(a: Expr, b: Expr) -> Expr {
    app2(cst("Real.le"), a, b)
}
/// Build `Real.lt a b`.
#[allow(dead_code)]
pub fn real_lt(a: Expr, b: Expr) -> Expr {
    app2(cst("Real.lt"), a, b)
}
/// Build `Real.dist a b`.
#[allow(dead_code)]
pub fn real_dist(a: Expr, b: Expr) -> Expr {
    app(cst("Real.abs"), app2(cst("Real.sub"), a, b))
}
/// Build `Real.zero`.
#[allow(dead_code)]
pub fn real_zero() -> Expr {
    cst("Real.zero")
}
/// Build `Real.one`.
#[allow(dead_code)]
pub fn real_one() -> Expr {
    cst("Real.one")
}
/// Build `Real.ofNat n`.
#[allow(dead_code)]
pub fn real_of_nat(n: Expr) -> Expr {
    app(cst("Real.ofNat"), n)
}
/// Build `Real.ofRat q`.
#[allow(dead_code)]
pub fn real_of_rat(q: Expr) -> Expr {
    app(cst("Real.ofRat"), q)
}
/// Build `Seq.lim s`.
#[allow(dead_code)]
pub fn seq_lim(s: Expr) -> Expr {
    app(cst("Seq.lim"), s)
}
/// Build `UpperBound S x`.
#[allow(dead_code)]
pub fn upper_bound(s: Expr, x: Expr) -> Expr {
    app2(cst("UpperBound"), s, x)
}
/// Build `IsLUB S x`.
#[allow(dead_code)]
pub fn is_lub(s: Expr, x: Expr) -> Expr {
    app2(cst("IsLUB"), s, x)
}
/// `forall (a : Real), <body>`
#[allow(dead_code)]
pub fn real_forall1<F>(prop_builder: F) -> Expr
where
    F: FnOnce(u32) -> Expr,
{
    pi(BinderInfo::Default, "a", real_ty(), prop_builder(1))
}
/// `forall (a b : Real), <body>`
#[allow(dead_code)]
pub fn real_forall2<F>(prop_builder: F) -> Expr
where
    F: FnOnce(u32) -> Expr,
{
    pi(
        BinderInfo::Default,
        "a",
        real_ty(),
        pi(BinderInfo::Default, "b", real_ty(), prop_builder(2)),
    )
}
/// `forall (a b c : Real), <body>`
#[allow(dead_code)]
pub fn real_forall3<F>(prop_builder: F) -> Expr
where
    F: FnOnce(u32) -> Expr,
{
    pi(
        BinderInfo::Default,
        "a",
        real_ty(),
        pi(
            BinderInfo::Default,
            "b",
            real_ty(),
            pi(BinderInfo::Default, "c", real_ty(), prop_builder(3)),
        ),
    )
}
/// `forall (eps : Real), Real.lt Real.zero eps -> <body>`
#[allow(dead_code)]
pub fn for_all_eps<F>(prop_builder: F) -> Expr
where
    F: FnOnce(u32) -> Expr,
{
    pi(
        BinderInfo::Default,
        "eps",
        real_ty(),
        arrow(
            app2(cst("Real.lt"), cst("Real.zero"), bvar(0)),
            prop_builder(1),
        ),
    )
}
/// Build the real number environment with Cauchy sequences, arithmetic,
/// completeness, convergence, and basic analysis.
#[allow(clippy::too_many_lines)]
pub fn build_real_env(env: &mut Environment) -> Result<(), String> {
    add_axiom(env, "Rat", vec![], type0())?;
    add_axiom(
        env,
        "Rat.add",
        vec![],
        arrow(rat_ty(), arrow(rat_ty(), rat_ty())),
    )?;
    add_axiom(
        env,
        "Rat.mul",
        vec![],
        arrow(rat_ty(), arrow(rat_ty(), rat_ty())),
    )?;
    add_axiom(env, "Rat.neg", vec![], arrow(rat_ty(), rat_ty()))?;
    add_axiom(env, "Rat.inv", vec![], arrow(rat_ty(), rat_ty()))?;
    add_axiom(env, "Rat.zero", vec![], rat_ty())?;
    add_axiom(env, "Rat.one", vec![], rat_ty())?;
    add_axiom(
        env,
        "Rat.le",
        vec![],
        arrow(rat_ty(), arrow(rat_ty(), prop())),
    )?;
    add_axiom(
        env,
        "Rat.lt",
        vec![],
        arrow(rat_ty(), arrow(rat_ty(), prop())),
    )?;
    add_axiom(env, "Rat.abs", vec![], arrow(rat_ty(), rat_ty()))?;
    add_axiom(env, "Rat.ofNat", vec![], arrow(nat_ty(), rat_ty()))?;
    add_axiom(env, "CauchySeq", vec![], type0())?;
    add_axiom(
        env,
        "CauchySeq.mk",
        vec![],
        arrow(arrow(nat_ty(), rat_ty()), cst("CauchySeq")),
    )?;
    add_axiom(
        env,
        "CauchySeq.get",
        vec![],
        arrow(cst("CauchySeq"), arrow(nat_ty(), rat_ty())),
    )?;
    add_axiom(
        env,
        "CauchySeq.cauchy",
        vec![],
        pi(
            BinderInfo::Default,
            "s",
            cst("CauchySeq"),
            pi(
                BinderInfo::Default,
                "eps",
                rat_ty(),
                arrow(app2(cst("Rat.lt"), cst("Rat.zero"), bvar(0)), prop()),
            ),
        ),
    )?;
    add_axiom(
        env,
        "CauchySeq.equiv",
        vec![],
        arrow(cst("CauchySeq"), arrow(cst("CauchySeq"), prop())),
    )?;
    add_axiom(
        env,
        "CauchySeq.equiv_refl",
        vec![],
        pi(
            BinderInfo::Default,
            "s",
            cst("CauchySeq"),
            app2(cst("CauchySeq.equiv"), bvar(0), bvar(0)),
        ),
    )?;
    add_axiom(
        env,
        "CauchySeq.equiv_symm",
        vec![],
        pi(
            BinderInfo::Default,
            "s",
            cst("CauchySeq"),
            pi(
                BinderInfo::Default,
                "t",
                cst("CauchySeq"),
                arrow(
                    app2(cst("CauchySeq.equiv"), bvar(1), bvar(0)),
                    app2(cst("CauchySeq.equiv"), bvar(0), bvar(1)),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "CauchySeq.equiv_trans",
        vec![],
        pi(
            BinderInfo::Default,
            "s",
            cst("CauchySeq"),
            pi(
                BinderInfo::Default,
                "t",
                cst("CauchySeq"),
                pi(
                    BinderInfo::Default,
                    "u",
                    cst("CauchySeq"),
                    arrow(
                        app2(cst("CauchySeq.equiv"), bvar(2), bvar(1)),
                        arrow(
                            app2(cst("CauchySeq.equiv"), bvar(1), bvar(0)),
                            app2(cst("CauchySeq.equiv"), bvar(2), bvar(0)),
                        ),
                    ),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "CauchySeq.add",
        vec![],
        arrow(cst("CauchySeq"), arrow(cst("CauchySeq"), cst("CauchySeq"))),
    )?;
    add_axiom(
        env,
        "CauchySeq.mul",
        vec![],
        arrow(cst("CauchySeq"), arrow(cst("CauchySeq"), cst("CauchySeq"))),
    )?;
    add_axiom(
        env,
        "CauchySeq.neg",
        vec![],
        arrow(cst("CauchySeq"), cst("CauchySeq")),
    )?;
    add_axiom(env, "Real", vec![], type0())?;
    add_axiom(env, "Real.mk", vec![], arrow(cst("CauchySeq"), real_ty()))?;
    add_axiom(
        env,
        "Real.mk_equiv",
        vec![],
        pi(
            BinderInfo::Default,
            "s",
            cst("CauchySeq"),
            pi(
                BinderInfo::Default,
                "t",
                cst("CauchySeq"),
                arrow(
                    app2(cst("CauchySeq.equiv"), bvar(1), bvar(0)),
                    mk_eq(
                        real_ty(),
                        app(cst("Real.mk"), bvar(1)),
                        app(cst("Real.mk"), bvar(0)),
                    ),
                ),
            ),
        ),
    )?;
    add_axiom(env, "Real.zero", vec![], real_ty())?;
    add_axiom(env, "Real.one", vec![], real_ty())?;
    add_axiom(
        env,
        "Real.add",
        vec![],
        arrow(real_ty(), arrow(real_ty(), real_ty())),
    )?;
    add_axiom(
        env,
        "Real.mul",
        vec![],
        arrow(real_ty(), arrow(real_ty(), real_ty())),
    )?;
    add_axiom(env, "Real.neg", vec![], arrow(real_ty(), real_ty()))?;
    add_axiom(env, "Real.inv", vec![], arrow(real_ty(), real_ty()))?;
    add_axiom(
        env,
        "Real.sub",
        vec![],
        arrow(real_ty(), arrow(real_ty(), real_ty())),
    )?;
    add_axiom(
        env,
        "Real.div",
        vec![],
        arrow(real_ty(), arrow(real_ty(), real_ty())),
    )?;
    add_axiom(
        env,
        "Real.pow",
        vec![],
        arrow(real_ty(), arrow(nat_ty(), real_ty())),
    )?;
    add_axiom(env, "Real.ofNat", vec![], arrow(nat_ty(), real_ty()))?;
    add_axiom(env, "Real.ofRat", vec![], arrow(rat_ty(), real_ty()))?;
    add_axiom(
        env,
        "Real.add_comm",
        vec![],
        real_forall2(|_depth| {
            let a = bvar(1);
            let b = bvar(0);
            mk_eq(
                real_ty(),
                app2(cst("Real.add"), a.clone(), b.clone()),
                app2(cst("Real.add"), b, a),
            )
        }),
    )?;
    add_axiom(
        env,
        "Real.add_assoc",
        vec![],
        real_forall3(|_depth| {
            let a = bvar(2);
            let b = bvar(1);
            let c = bvar(0);
            mk_eq(
                real_ty(),
                app2(
                    cst("Real.add"),
                    app2(cst("Real.add"), a.clone(), b.clone()),
                    c.clone(),
                ),
                app2(cst("Real.add"), a, app2(cst("Real.add"), b, c)),
            )
        }),
    )?;
    add_axiom(
        env,
        "Real.add_zero",
        vec![],
        real_forall1(|_depth| {
            let a = bvar(0);
            mk_eq(
                real_ty(),
                app2(cst("Real.add"), a.clone(), cst("Real.zero")),
                a,
            )
        }),
    )?;
    add_axiom(
        env,
        "Real.zero_add",
        vec![],
        real_forall1(|_depth| {
            let a = bvar(0);
            mk_eq(
                real_ty(),
                app2(cst("Real.add"), cst("Real.zero"), a.clone()),
                a,
            )
        }),
    )?;
    add_axiom(
        env,
        "Real.add_neg",
        vec![],
        real_forall1(|_depth| {
            let a = bvar(0);
            mk_eq(
                real_ty(),
                app2(cst("Real.add"), a.clone(), app(cst("Real.neg"), a)),
                cst("Real.zero"),
            )
        }),
    )?;
    add_axiom(
        env,
        "Real.neg_add",
        vec![],
        real_forall1(|_depth| {
            let a = bvar(0);
            mk_eq(
                real_ty(),
                app2(cst("Real.add"), app(cst("Real.neg"), a.clone()), a),
                cst("Real.zero"),
            )
        }),
    )?;
    add_axiom(
        env,
        "Real.sub_def",
        vec![],
        real_forall2(|_depth| {
            let a = bvar(1);
            let b = bvar(0);
            mk_eq(
                real_ty(),
                app2(cst("Real.sub"), a.clone(), b.clone()),
                app2(cst("Real.add"), a, app(cst("Real.neg"), b)),
            )
        }),
    )?;
    add_axiom(
        env,
        "Real.neg_neg",
        vec![],
        real_forall1(|_depth| {
            let a = bvar(0);
            mk_eq(
                real_ty(),
                app(cst("Real.neg"), app(cst("Real.neg"), a.clone())),
                a,
            )
        }),
    )?;
    add_axiom(
        env,
        "Real.neg_zero",
        vec![],
        mk_eq(
            real_ty(),
            app(cst("Real.neg"), cst("Real.zero")),
            cst("Real.zero"),
        ),
    )?;
    add_axiom(
        env,
        "Real.mul_comm",
        vec![],
        real_forall2(|_depth| {
            let a = bvar(1);
            let b = bvar(0);
            mk_eq(
                real_ty(),
                app2(cst("Real.mul"), a.clone(), b.clone()),
                app2(cst("Real.mul"), b, a),
            )
        }),
    )?;
    add_axiom(
        env,
        "Real.mul_assoc",
        vec![],
        real_forall3(|_depth| {
            let a = bvar(2);
            let b = bvar(1);
            let c = bvar(0);
            mk_eq(
                real_ty(),
                app2(
                    cst("Real.mul"),
                    app2(cst("Real.mul"), a.clone(), b.clone()),
                    c.clone(),
                ),
                app2(cst("Real.mul"), a, app2(cst("Real.mul"), b, c)),
            )
        }),
    )?;
    add_axiom(
        env,
        "Real.mul_one",
        vec![],
        real_forall1(|_depth| {
            let a = bvar(0);
            mk_eq(
                real_ty(),
                app2(cst("Real.mul"), a.clone(), cst("Real.one")),
                a,
            )
        }),
    )?;
    add_axiom(
        env,
        "Real.one_mul",
        vec![],
        real_forall1(|_depth| {
            let a = bvar(0);
            mk_eq(
                real_ty(),
                app2(cst("Real.mul"), cst("Real.one"), a.clone()),
                a,
            )
        }),
    )?;
    add_axiom(
        env,
        "Real.mul_zero",
        vec![],
        real_forall1(|_depth| {
            let a = bvar(0);
            mk_eq(
                real_ty(),
                app2(cst("Real.mul"), a, cst("Real.zero")),
                cst("Real.zero"),
            )
        }),
    )?;
    add_axiom(
        env,
        "Real.zero_mul",
        vec![],
        real_forall1(|_depth| {
            let a = bvar(0);
            mk_eq(
                real_ty(),
                app2(cst("Real.mul"), cst("Real.zero"), a),
                cst("Real.zero"),
            )
        }),
    )?;
    add_axiom(
        env,
        "Real.mul_inv",
        vec![],
        real_forall1(|_depth| {
            let a = bvar(0);
            arrow(
                mk_not(mk_eq(real_ty(), a.clone(), cst("Real.zero"))),
                mk_eq(
                    real_ty(),
                    app2(cst("Real.mul"), a.clone(), app(cst("Real.inv"), a)),
                    cst("Real.one"),
                ),
            )
        }),
    )?;
    add_axiom(
        env,
        "Real.inv_mul",
        vec![],
        real_forall1(|_depth| {
            let a = bvar(0);
            arrow(
                mk_not(mk_eq(real_ty(), a.clone(), cst("Real.zero"))),
                mk_eq(
                    real_ty(),
                    app2(cst("Real.mul"), app(cst("Real.inv"), a.clone()), a),
                    cst("Real.one"),
                ),
            )
        }),
    )?;
    add_axiom(
        env,
        "Real.div_def",
        vec![],
        real_forall2(|_depth| {
            let a = bvar(1);
            let b = bvar(0);
            mk_eq(
                real_ty(),
                app2(cst("Real.div"), a.clone(), b.clone()),
                app2(cst("Real.mul"), a, app(cst("Real.inv"), b)),
            )
        }),
    )?;
    add_axiom(
        env,
        "Real.left_distrib",
        vec![],
        real_forall3(|_depth| {
            let a = bvar(2);
            let b = bvar(1);
            let c = bvar(0);
            mk_eq(
                real_ty(),
                app2(
                    cst("Real.mul"),
                    a.clone(),
                    app2(cst("Real.add"), b.clone(), c.clone()),
                ),
                app2(
                    cst("Real.add"),
                    app2(cst("Real.mul"), a.clone(), b),
                    app2(cst("Real.mul"), a, c),
                ),
            )
        }),
    )?;
    add_axiom(
        env,
        "Real.right_distrib",
        vec![],
        real_forall3(|_depth| {
            let a = bvar(2);
            let b = bvar(1);
            let c = bvar(0);
            mk_eq(
                real_ty(),
                app2(
                    cst("Real.mul"),
                    app2(cst("Real.add"), a.clone(), b.clone()),
                    c.clone(),
                ),
                app2(
                    cst("Real.add"),
                    app2(cst("Real.mul"), a, c.clone()),
                    app2(cst("Real.mul"), b, c),
                ),
            )
        }),
    )?;
    add_axiom(
        env,
        "Real.neg_mul",
        vec![],
        real_forall2(|_depth| {
            let a = bvar(1);
            let b = bvar(0);
            mk_eq(
                real_ty(),
                app2(cst("Real.mul"), app(cst("Real.neg"), a.clone()), b.clone()),
                app(cst("Real.neg"), app2(cst("Real.mul"), a, b)),
            )
        }),
    )?;
    add_axiom(
        env,
        "Real.mul_neg",
        vec![],
        real_forall2(|_depth| {
            let a = bvar(1);
            let b = bvar(0);
            mk_eq(
                real_ty(),
                app2(cst("Real.mul"), a.clone(), app(cst("Real.neg"), b.clone())),
                app(cst("Real.neg"), app2(cst("Real.mul"), a, b)),
            )
        }),
    )?;
    add_axiom(
        env,
        "Real.le",
        vec![],
        arrow(real_ty(), arrow(real_ty(), prop())),
    )?;
    add_axiom(
        env,
        "Real.lt",
        vec![],
        arrow(real_ty(), arrow(real_ty(), prop())),
    )?;
    add_axiom(
        env,
        "Real.lt_def",
        vec![],
        real_forall2(|_depth| {
            let a = bvar(1);
            let b = bvar(0);
            mk_iff(
                app2(cst("Real.lt"), a.clone(), b.clone()),
                mk_and(
                    app2(cst("Real.le"), a.clone(), b.clone()),
                    mk_not(mk_eq(real_ty(), a, b)),
                ),
            )
        }),
    )?;
    add_axiom(
        env,
        "Real.le_refl",
        vec![],
        real_forall1(|_depth| {
            let a = bvar(0);
            app2(cst("Real.le"), a.clone(), a)
        }),
    )?;
    add_axiom(
        env,
        "Real.le_trans",
        vec![],
        real_forall3(|_depth| {
            let a = bvar(2);
            let b = bvar(1);
            let c = bvar(0);
            arrow(
                app2(cst("Real.le"), a.clone(), b.clone()),
                arrow(
                    app2(cst("Real.le"), b, c.clone()),
                    app2(cst("Real.le"), a, c),
                ),
            )
        }),
    )?;
    add_axiom(
        env,
        "Real.le_antisymm",
        vec![],
        real_forall2(|_depth| {
            let a = bvar(1);
            let b = bvar(0);
            arrow(
                app2(cst("Real.le"), a.clone(), b.clone()),
                arrow(
                    app2(cst("Real.le"), b.clone(), a.clone()),
                    mk_eq(real_ty(), a, b),
                ),
            )
        }),
    )?;
    add_axiom(
        env,
        "Real.le_total",
        vec![],
        real_forall2(|_depth| {
            let a = bvar(1);
            let b = bvar(0);
            mk_or(
                app2(cst("Real.le"), a.clone(), b.clone()),
                app2(cst("Real.le"), b, a),
            )
        }),
    )?;
    add_axiom(
        env,
        "Real.zero_le_one",
        vec![],
        app2(cst("Real.le"), cst("Real.zero"), cst("Real.one")),
    )?;
    add_axiom(
        env,
        "Real.zero_lt_one",
        vec![],
        app2(cst("Real.lt"), cst("Real.zero"), cst("Real.one")),
    )?;
    add_axiom(
        env,
        "Real.add_le_add_left",
        vec![],
        real_forall3(|_depth| {
            let a = bvar(2);
            let b = bvar(1);
            let c = bvar(0);
            arrow(
                app2(cst("Real.le"), a.clone(), b.clone()),
                app2(
                    cst("Real.le"),
                    app2(cst("Real.add"), c.clone(), a),
                    app2(cst("Real.add"), c, b),
                ),
            )
        }),
    )?;
    add_axiom(
        env,
        "Real.mul_pos",
        vec![],
        real_forall2(|_depth| {
            let a = bvar(1);
            let b = bvar(0);
            arrow(
                app2(cst("Real.lt"), cst("Real.zero"), a.clone()),
                arrow(
                    app2(cst("Real.lt"), cst("Real.zero"), b.clone()),
                    app2(
                        cst("Real.lt"),
                        cst("Real.zero"),
                        app2(cst("Real.mul"), a, b),
                    ),
                ),
            )
        }),
    )?;
    add_axiom(env, "Real.abs", vec![], arrow(real_ty(), real_ty()))?;
    add_axiom(
        env,
        "Real.abs_nonneg",
        vec![],
        real_forall1(|_depth| {
            let a = bvar(0);
            app2(cst("Real.le"), cst("Real.zero"), app(cst("Real.abs"), a))
        }),
    )?;
    add_axiom(
        env,
        "Real.abs_zero",
        vec![],
        mk_eq(
            real_ty(),
            app(cst("Real.abs"), cst("Real.zero")),
            cst("Real.zero"),
        ),
    )?;
    add_axiom(
        env,
        "Real.abs_pos",
        vec![],
        real_forall1(|_depth| {
            let a = bvar(0);
            arrow(
                mk_not(mk_eq(real_ty(), a.clone(), cst("Real.zero"))),
                app2(cst("Real.lt"), cst("Real.zero"), app(cst("Real.abs"), a)),
            )
        }),
    )?;
    add_axiom(
        env,
        "Real.abs_neg",
        vec![],
        real_forall1(|_depth| {
            let a = bvar(0);
            mk_eq(
                real_ty(),
                app(cst("Real.abs"), app(cst("Real.neg"), a.clone())),
                app(cst("Real.abs"), a),
            )
        }),
    )?;
    add_axiom(
        env,
        "Real.abs_mul",
        vec![],
        real_forall2(|_depth| {
            let a = bvar(1);
            let b = bvar(0);
            mk_eq(
                real_ty(),
                app(cst("Real.abs"), app2(cst("Real.mul"), a.clone(), b.clone())),
                app2(
                    cst("Real.mul"),
                    app(cst("Real.abs"), a),
                    app(cst("Real.abs"), b),
                ),
            )
        }),
    )?;
    add_axiom(
        env,
        "Real.triangle_inequality",
        vec![],
        real_forall2(|_depth| {
            let a = bvar(1);
            let b = bvar(0);
            app2(
                cst("Real.le"),
                app(cst("Real.abs"), app2(cst("Real.add"), a.clone(), b.clone())),
                app2(
                    cst("Real.add"),
                    app(cst("Real.abs"), a),
                    app(cst("Real.abs"), b),
                ),
            )
        }),
    )?;
    add_axiom(
        env,
        "Real.abs_eq_zero_iff",
        vec![],
        real_forall1(|_depth| {
            let a = bvar(0);
            mk_iff(
                mk_eq(real_ty(), app(cst("Real.abs"), a.clone()), cst("Real.zero")),
                mk_eq(real_ty(), a, cst("Real.zero")),
            )
        }),
    )?;
    add_axiom(
        env,
        "Real.dist",
        vec![],
        arrow(real_ty(), arrow(real_ty(), real_ty())),
    )?;
    add_axiom(
        env,
        "Real.dist_def",
        vec![],
        real_forall2(|_depth| {
            let a = bvar(1);
            let b = bvar(0);
            mk_eq(
                real_ty(),
                app2(cst("Real.dist"), a.clone(), b.clone()),
                app(cst("Real.abs"), app2(cst("Real.sub"), a, b)),
            )
        }),
    )?;
    add_axiom(
        env,
        "Real.dist_nonneg",
        vec![],
        real_forall2(|_depth| {
            let a = bvar(1);
            let b = bvar(0);
            app2(
                cst("Real.le"),
                cst("Real.zero"),
                app2(cst("Real.dist"), a, b),
            )
        }),
    )?;
    add_axiom(
        env,
        "Real.dist_self",
        vec![],
        real_forall1(|_depth| {
            let a = bvar(0);
            mk_eq(
                real_ty(),
                app2(cst("Real.dist"), a.clone(), a),
                cst("Real.zero"),
            )
        }),
    )?;
    add_axiom(
        env,
        "Real.dist_comm",
        vec![],
        real_forall2(|_depth| {
            let a = bvar(1);
            let b = bvar(0);
            mk_eq(
                real_ty(),
                app2(cst("Real.dist"), a.clone(), b.clone()),
                app2(cst("Real.dist"), b, a),
            )
        }),
    )?;
    add_axiom(
        env,
        "Real.dist_triangle",
        vec![],
        real_forall3(|_depth| {
            let a = bvar(2);
            let b = bvar(1);
            let c = bvar(0);
            app2(
                cst("Real.le"),
                app2(cst("Real.dist"), a.clone(), c),
                app2(
                    cst("Real.add"),
                    app2(cst("Real.dist"), a, b.clone()),
                    app2(cst("Real.dist"), b, bvar(0)),
                ),
            )
        }),
    )?;
    add_axiom(
        env,
        "UpperBound",
        vec![],
        arrow(arrow(real_ty(), prop()), arrow(real_ty(), prop())),
    )?;
    add_axiom(
        env,
        "LowerBound",
        vec![],
        arrow(arrow(real_ty(), prop()), arrow(real_ty(), prop())),
    )?;
    add_axiom(
        env,
        "IsLUB",
        vec![],
        arrow(arrow(real_ty(), prop()), arrow(real_ty(), prop())),
    )?;
    add_axiom(
        env,
        "IsGLB",
        vec![],
        arrow(arrow(real_ty(), prop()), arrow(real_ty(), prop())),
    )?;
    add_axiom(
        env,
        "Real.sup",
        vec![],
        arrow(arrow(real_ty(), prop()), real_ty()),
    )?;
    add_axiom(
        env,
        "Real.inf",
        vec![],
        arrow(arrow(real_ty(), prop()), real_ty()),
    )?;
    add_axiom(
        env,
        "Real.completeness",
        vec![],
        pi(
            BinderInfo::Default,
            "S",
            arrow(real_ty(), prop()),
            arrow(
                app2(cst("Exists"), real_ty(), bvar(0)),
                arrow(
                    app2(cst("Exists"), real_ty(), app(cst("UpperBound"), bvar(0))),
                    app2(cst("IsLUB"), bvar(0), app(cst("Real.sup"), bvar(0))),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "Real.completeness_glb",
        vec![],
        pi(
            BinderInfo::Default,
            "S",
            arrow(real_ty(), prop()),
            arrow(
                app2(cst("Exists"), real_ty(), bvar(0)),
                arrow(
                    app2(cst("Exists"), real_ty(), app(cst("LowerBound"), bvar(0))),
                    app2(cst("IsGLB"), bvar(0), app(cst("Real.inf"), bvar(0))),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "Real.archimedean",
        vec![],
        pi(
            BinderInfo::Default,
            "x",
            real_ty(),
            arrow(
                app2(cst("Real.lt"), cst("Real.zero"), bvar(0)),
                pi(
                    BinderInfo::Default,
                    "y",
                    real_ty(),
                    mk_exists(
                        nat_ty(),
                        lam(
                            BinderInfo::Default,
                            "n",
                            nat_ty(),
                            app2(
                                cst("Real.lt"),
                                bvar(1),
                                app2(cst("Real.mul"), app(cst("Real.ofNat"), bvar(0)), bvar(3)),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "Real.archimedean_nat",
        vec![],
        pi(
            BinderInfo::Default,
            "x",
            real_ty(),
            mk_exists(
                nat_ty(),
                lam(
                    BinderInfo::Default,
                    "n",
                    nat_ty(),
                    app2(cst("Real.lt"), bvar(1), app(cst("Real.ofNat"), bvar(0))),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "Real.rat_dense",
        vec![],
        real_forall2(|_depth| {
            let a = bvar(1);
            let b = bvar(0);
            arrow(
                app2(cst("Real.lt"), a, b),
                mk_exists(
                    rat_ty(),
                    lam(
                        BinderInfo::Default,
                        "q",
                        rat_ty(),
                        mk_and(
                            app2(cst("Real.lt"), bvar(3), app(cst("Real.ofRat"), bvar(0))),
                            app2(cst("Real.lt"), app(cst("Real.ofRat"), bvar(0)), bvar(2)),
                        ),
                    ),
                ),
            )
        }),
    )?;
    add_axiom(
        env,
        "Seq.convergesTo",
        vec![],
        arrow(arrow(nat_ty(), real_ty()), arrow(real_ty(), prop())),
    )?;
    add_axiom(
        env,
        "Seq.converges",
        vec![],
        arrow(arrow(nat_ty(), real_ty()), prop()),
    )?;
    add_axiom(
        env,
        "Seq.lim",
        vec![],
        arrow(arrow(nat_ty(), real_ty()), real_ty()),
    )?;
    add_axiom(
        env,
        "Seq.isCauchy",
        vec![],
        arrow(arrow(nat_ty(), real_ty()), prop()),
    )?;
    add_axiom(
        env,
        "Seq.convergent_is_cauchy",
        vec![],
        pi(
            BinderInfo::Default,
            "s",
            arrow(nat_ty(), real_ty()),
            arrow(
                app(cst("Seq.converges"), bvar(0)),
                app(cst("Seq.isCauchy"), bvar(0)),
            ),
        ),
    )?;
    add_axiom(
        env,
        "Seq.cauchy_is_convergent",
        vec![],
        pi(
            BinderInfo::Default,
            "s",
            arrow(nat_ty(), real_ty()),
            arrow(
                app(cst("Seq.isCauchy"), bvar(0)),
                app(cst("Seq.converges"), bvar(0)),
            ),
        ),
    )?;
    add_axiom(
        env,
        "Seq.lim_unique",
        vec![],
        pi(
            BinderInfo::Default,
            "s",
            arrow(nat_ty(), real_ty()),
            pi(
                BinderInfo::Default,
                "a",
                real_ty(),
                pi(
                    BinderInfo::Default,
                    "b",
                    real_ty(),
                    arrow(
                        app2(cst("Seq.convergesTo"), bvar(2), bvar(1)),
                        arrow(
                            app2(cst("Seq.convergesTo"), bvar(2), bvar(0)),
                            mk_eq(real_ty(), bvar(1), bvar(0)),
                        ),
                    ),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "Seq.lim_spec",
        vec![],
        pi(
            BinderInfo::Default,
            "s",
            arrow(nat_ty(), real_ty()),
            arrow(
                app(cst("Seq.converges"), bvar(0)),
                app2(
                    cst("Seq.convergesTo"),
                    bvar(0),
                    app(cst("Seq.lim"), bvar(0)),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "Seq.lim_add",
        vec![],
        pi(
            BinderInfo::Default,
            "s",
            arrow(nat_ty(), real_ty()),
            pi(
                BinderInfo::Default,
                "t",
                arrow(nat_ty(), real_ty()),
                arrow(
                    app(cst("Seq.converges"), bvar(1)),
                    arrow(
                        app(cst("Seq.converges"), bvar(0)),
                        mk_eq(
                            real_ty(),
                            app(
                                cst("Seq.lim"),
                                lam(
                                    BinderInfo::Default,
                                    "n",
                                    nat_ty(),
                                    app2(
                                        cst("Real.add"),
                                        app(bvar(2), bvar(0)),
                                        app(bvar(1), bvar(0)),
                                    ),
                                ),
                            ),
                            app2(
                                cst("Real.add"),
                                app(cst("Seq.lim"), bvar(1)),
                                app(cst("Seq.lim"), bvar(0)),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "Seq.lim_mul",
        vec![],
        pi(
            BinderInfo::Default,
            "s",
            arrow(nat_ty(), real_ty()),
            pi(
                BinderInfo::Default,
                "t",
                arrow(nat_ty(), real_ty()),
                arrow(
                    app(cst("Seq.converges"), bvar(1)),
                    arrow(
                        app(cst("Seq.converges"), bvar(0)),
                        mk_eq(
                            real_ty(),
                            app(
                                cst("Seq.lim"),
                                lam(
                                    BinderInfo::Default,
                                    "n",
                                    nat_ty(),
                                    app2(
                                        cst("Real.mul"),
                                        app(bvar(2), bvar(0)),
                                        app(bvar(1), bvar(0)),
                                    ),
                                ),
                            ),
                            app2(
                                cst("Real.mul"),
                                app(cst("Seq.lim"), bvar(1)),
                                app(cst("Seq.lim"), bvar(0)),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "Seq.lim_neg",
        vec![],
        pi(
            BinderInfo::Default,
            "s",
            arrow(nat_ty(), real_ty()),
            arrow(
                app(cst("Seq.converges"), bvar(0)),
                mk_eq(
                    real_ty(),
                    app(
                        cst("Seq.lim"),
                        lam(
                            BinderInfo::Default,
                            "n",
                            nat_ty(),
                            app(cst("Real.neg"), app(bvar(1), bvar(0))),
                        ),
                    ),
                    app(cst("Real.neg"), app(cst("Seq.lim"), bvar(0))),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "Seq.lim_const",
        vec![],
        pi(
            BinderInfo::Default,
            "c",
            real_ty(),
            app2(
                cst("Seq.convergesTo"),
                lam(BinderInfo::Default, "_", nat_ty(), bvar(1)),
                bvar(0),
            ),
        ),
    )?;
    add_axiom(
        env,
        "Seq.isMonotone",
        vec![],
        arrow(arrow(nat_ty(), real_ty()), prop()),
    )?;
    add_axiom(
        env,
        "Seq.isBounded",
        vec![],
        arrow(arrow(nat_ty(), real_ty()), prop()),
    )?;
    add_axiom(
        env,
        "Seq.monotone_convergence",
        vec![],
        pi(
            BinderInfo::Default,
            "s",
            arrow(nat_ty(), real_ty()),
            arrow(
                app(cst("Seq.isMonotone"), bvar(0)),
                arrow(
                    app(cst("Seq.isBounded"), bvar(0)),
                    app(cst("Seq.converges"), bvar(0)),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "Seq.hasSubsequence",
        vec![],
        arrow(
            arrow(nat_ty(), real_ty()),
            arrow(arrow(nat_ty(), real_ty()), prop()),
        ),
    )?;
    add_axiom(
        env,
        "Seq.bolzano_weierstrass",
        vec![],
        pi(
            BinderInfo::Default,
            "s",
            arrow(nat_ty(), real_ty()),
            arrow(
                app(cst("Seq.isBounded"), bvar(0)),
                mk_exists(
                    arrow(nat_ty(), real_ty()),
                    lam(
                        BinderInfo::Default,
                        "sub",
                        arrow(nat_ty(), real_ty()),
                        mk_and(
                            app2(cst("Seq.hasSubsequence"), bvar(1), bvar(0)),
                            app(cst("Seq.converges"), bvar(0)),
                        ),
                    ),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "Real.isContinuousAt",
        vec![],
        arrow(arrow(real_ty(), real_ty()), arrow(real_ty(), prop())),
    )?;
    add_axiom(
        env,
        "Real.isContinuousOn",
        vec![],
        arrow(
            arrow(real_ty(), real_ty()),
            arrow(arrow(real_ty(), prop()), prop()),
        ),
    )?;
    add_axiom(
        env,
        "Real.intermediate_value_theorem",
        vec![],
        pi(
            BinderInfo::Default,
            "f",
            arrow(real_ty(), real_ty()),
            pi(
                BinderInfo::Default,
                "a",
                real_ty(),
                pi(
                    BinderInfo::Default,
                    "b",
                    real_ty(),
                    arrow(
                        app2(cst("Real.le"), bvar(1), bvar(0)),
                        arrow(
                            app2(
                                cst("Real.isContinuousOn"),
                                bvar(3),
                                lam(
                                    BinderInfo::Default,
                                    "x",
                                    real_ty(),
                                    mk_and(
                                        app2(cst("Real.le"), bvar(3), bvar(0)),
                                        app2(cst("Real.le"), bvar(0), bvar(2)),
                                    ),
                                ),
                            ),
                            pi(
                                BinderInfo::Default,
                                "y",
                                real_ty(),
                                arrow(
                                    app2(cst("Real.le"), app(bvar(5), bvar(4)), bvar(0)),
                                    arrow(
                                        app2(cst("Real.le"), bvar(1), app(bvar(6), bvar(4))),
                                        mk_exists(
                                            real_ty(),
                                            lam(
                                                BinderInfo::Default,
                                                "c",
                                                real_ty(),
                                                mk_and(
                                                    mk_and(
                                                        app2(cst("Real.le"), bvar(7), bvar(0)),
                                                        app2(cst("Real.le"), bvar(0), bvar(6)),
                                                    ),
                                                    mk_eq(
                                                        real_ty(),
                                                        app(bvar(8), bvar(0)),
                                                        bvar(3),
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
    add_axiom(
        env,
        "Real.pow_zero",
        vec![],
        real_forall1(|_depth| {
            let a = bvar(0);
            mk_eq(
                real_ty(),
                app2(cst("Real.pow"), a, cst("Nat.zero")),
                cst("Real.one"),
            )
        }),
    )?;
    add_axiom(
        env,
        "Real.pow_succ",
        vec![],
        pi(
            BinderInfo::Default,
            "a",
            real_ty(),
            pi(
                BinderInfo::Default,
                "n",
                nat_ty(),
                mk_eq(
                    real_ty(),
                    app2(cst("Real.pow"), bvar(1), app(cst("Nat.succ"), bvar(0))),
                    app2(
                        cst("Real.mul"),
                        bvar(1),
                        app2(cst("Real.pow"), bvar(1), bvar(0)),
                    ),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "Real.max",
        vec![],
        arrow(real_ty(), arrow(real_ty(), real_ty())),
    )?;
    add_axiom(
        env,
        "Real.min",
        vec![],
        arrow(real_ty(), arrow(real_ty(), real_ty())),
    )?;
    add_axiom(
        env,
        "Real.max_comm",
        vec![],
        real_forall2(|_depth| {
            let a = bvar(1);
            let b = bvar(0);
            mk_eq(
                real_ty(),
                app2(cst("Real.max"), a.clone(), b.clone()),
                app2(cst("Real.max"), b, a),
            )
        }),
    )?;
    add_axiom(
        env,
        "Real.min_comm",
        vec![],
        real_forall2(|_depth| {
            let a = bvar(1);
            let b = bvar(0);
            mk_eq(
                real_ty(),
                app2(cst("Real.min"), a.clone(), b.clone()),
                app2(cst("Real.min"), b, a),
            )
        }),
    )?;
    add_axiom(
        env,
        "Real.le_max_left",
        vec![],
        real_forall2(|_depth| {
            let a = bvar(1);
            let b = bvar(0);
            app2(cst("Real.le"), a.clone(), app2(cst("Real.max"), a, b))
        }),
    )?;
    add_axiom(
        env,
        "Real.le_max_right",
        vec![],
        real_forall2(|_depth| {
            let a = bvar(1);
            let b = bvar(0);
            app2(cst("Real.le"), b.clone(), app2(cst("Real.max"), a, b))
        }),
    )?;
    add_axiom(
        env,
        "Real.min_le_left",
        vec![],
        real_forall2(|_depth| {
            let a = bvar(1);
            let b = bvar(0);
            app2(cst("Real.le"), app2(cst("Real.min"), a.clone(), b), a)
        }),
    )?;
    add_axiom(
        env,
        "Real.min_le_right",
        vec![],
        real_forall2(|_depth| {
            let a = bvar(1);
            let b = bvar(0);
            app2(cst("Real.le"), app2(cst("Real.min"), a, b.clone()), b)
        }),
    )?;
    add_axiom(
        env,
        "Seq.squeeze",
        vec![],
        pi(
            BinderInfo::Default,
            "s",
            arrow(nat_ty(), real_ty()),
            pi(
                BinderInfo::Default,
                "t",
                arrow(nat_ty(), real_ty()),
                pi(
                    BinderInfo::Default,
                    "u",
                    arrow(nat_ty(), real_ty()),
                    pi(
                        BinderInfo::Default,
                        "L",
                        real_ty(),
                        arrow(
                            app2(cst("Seq.convergesTo"), bvar(3), bvar(0)),
                            arrow(
                                app2(cst("Seq.convergesTo"), bvar(1), bvar(0)),
                                arrow(
                                    pi(
                                        BinderInfo::Default,
                                        "n",
                                        nat_ty(),
                                        mk_and(
                                            app2(
                                                cst("Real.le"),
                                                app(bvar(4), bvar(0)),
                                                app(bvar(3), bvar(0)),
                                            ),
                                            app2(
                                                cst("Real.le"),
                                                app(bvar(3), bvar(0)),
                                                app(bvar(2), bvar(0)),
                                            ),
                                        ),
                                    ),
                                    app2(cst("Seq.convergesTo"), bvar(2), bvar(0)),
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
        "Series.partialSum",
        vec![],
        arrow(arrow(nat_ty(), real_ty()), arrow(nat_ty(), real_ty())),
    )?;
    add_axiom(
        env,
        "Series.converges",
        vec![],
        arrow(arrow(nat_ty(), real_ty()), prop()),
    )?;
    add_axiom(
        env,
        "Series.sum",
        vec![],
        arrow(arrow(nat_ty(), real_ty()), real_ty()),
    )?;
    add_axiom(
        env,
        "Series.geometric_converges",
        vec![],
        pi(
            BinderInfo::Default,
            "r",
            real_ty(),
            arrow(
                app2(
                    cst("Real.lt"),
                    app(cst("Real.abs"), bvar(0)),
                    cst("Real.one"),
                ),
                app(
                    cst("Series.converges"),
                    lam(
                        BinderInfo::Default,
                        "n",
                        nat_ty(),
                        app2(cst("Real.pow"), bvar(1), bvar(0)),
                    ),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "Real.ofNat_add",
        vec![],
        pi(
            BinderInfo::Default,
            "m",
            nat_ty(),
            pi(
                BinderInfo::Default,
                "n",
                nat_ty(),
                mk_eq(
                    real_ty(),
                    app(cst("Real.ofNat"), app2(cst("Nat.add"), bvar(1), bvar(0))),
                    app2(
                        cst("Real.add"),
                        app(cst("Real.ofNat"), bvar(1)),
                        app(cst("Real.ofNat"), bvar(0)),
                    ),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "Real.ofNat_mul",
        vec![],
        pi(
            BinderInfo::Default,
            "m",
            nat_ty(),
            pi(
                BinderInfo::Default,
                "n",
                nat_ty(),
                mk_eq(
                    real_ty(),
                    app(cst("Real.ofNat"), app2(cst("Nat.mul"), bvar(1), bvar(0))),
                    app2(
                        cst("Real.mul"),
                        app(cst("Real.ofNat"), bvar(1)),
                        app(cst("Real.ofNat"), bvar(0)),
                    ),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "Real.ofRat_add",
        vec![],
        pi(
            BinderInfo::Default,
            "p",
            rat_ty(),
            pi(
                BinderInfo::Default,
                "q",
                rat_ty(),
                mk_eq(
                    real_ty(),
                    app(cst("Real.ofRat"), app2(cst("Rat.add"), bvar(1), bvar(0))),
                    app2(
                        cst("Real.add"),
                        app(cst("Real.ofRat"), bvar(1)),
                        app(cst("Real.ofRat"), bvar(0)),
                    ),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "Real.ofRat_mul",
        vec![],
        pi(
            BinderInfo::Default,
            "p",
            rat_ty(),
            pi(
                BinderInfo::Default,
                "q",
                rat_ty(),
                mk_eq(
                    real_ty(),
                    app(cst("Real.ofRat"), app2(cst("Rat.mul"), bvar(1), bvar(0))),
                    app2(
                        cst("Real.mul"),
                        app(cst("Real.ofRat"), bvar(1)),
                        app(cst("Real.ofRat"), bvar(0)),
                    ),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "Real.ofRat_injective",
        vec![],
        pi(
            BinderInfo::Default,
            "p",
            rat_ty(),
            pi(
                BinderInfo::Default,
                "q",
                rat_ty(),
                arrow(
                    mk_eq(
                        real_ty(),
                        app(cst("Real.ofRat"), bvar(1)),
                        app(cst("Real.ofRat"), bvar(0)),
                    ),
                    mk_eq(rat_ty(), bvar(1), bvar(0)),
                ),
            ),
        ),
    )?;
    Ok(())
}
