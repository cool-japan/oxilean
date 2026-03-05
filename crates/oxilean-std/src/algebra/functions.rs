//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{
    AbelianCategory, DerivedCategory, ExtTorGroups, GaloisExtension, GradedRing, LieAlgebra,
    Localization, Module, NoetherianRing, ProjectiveResolution, ShortExactSequence,
};

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
#[allow(dead_code)]
pub fn prop() -> Expr {
    Expr::Sort(Level::zero())
}
pub fn type0() -> Expr {
    Expr::Sort(Level::succ(Level::zero()))
}
pub fn type1() -> Expr {
    Expr::Sort(Level::succ(Level::succ(Level::zero())))
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
/// Build `Type → Type 1` (the type of a type class that takes a Type).
#[allow(dead_code)]
pub fn mk_class_ty() -> Expr {
    pi(BinderInfo::Default, "α", type0(), type1())
}
/// Build `Const(name, [])`.
#[allow(dead_code)]
pub fn mk_typeclass(name: &str) -> Expr {
    cst(name)
}
/// Build a nullary method type: `{α : Type} → [Class α] → α`
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
/// Build a unary method type: `{α : Type} → [Class α] → α → α`
#[allow(dead_code)]
pub fn mk_unop_method(class: &str) -> Expr {
    pi(
        BinderInfo::Implicit,
        "α",
        type0(),
        pi(
            BinderInfo::InstImplicit,
            "_inst",
            app(cst(class), bvar(0)),
            pi(BinderInfo::Default, "a", bvar(1), bvar(2)),
        ),
    )
}
/// Build a binary method type: `{α : Type} → [Class α] → α → α → α`
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
/// Build a law type (proposition with class instance):
/// `{α : Type} → [Class α] → <prop>`
/// The `prop_builder` receives the de Bruijn depth (number of binders above it).
#[allow(dead_code)]
pub fn mk_law<F>(class: &str, prop_builder: F) -> Expr
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
            prop_builder(2),
        ),
    )
}
/// Build a law with universally quantified element(s):
/// `{α : Type} → [Class α] → ∀ (a : α), <prop(a)>`
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
/// `{α : Type} → [Class α] → ∀ (a b : α), <prop(a, b)>`
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
/// `{α : Type} → [Class α] → ∀ (a b c : α), <prop(a, b, c)>`
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
/// Build `Eq @{} α a b`.
#[allow(dead_code)]
pub fn mk_eq(ty: Expr, lhs: Expr, rhs: Expr) -> Expr {
    app3(cst("Eq"), ty, lhs, rhs)
}
/// Build `Add.add {α} [inst] a b`.
#[allow(dead_code)]
pub fn algebra_add(a: Expr, b: Expr) -> Expr {
    app2(cst("Add.add"), a, b)
}
/// Build `Mul.mul {α} [inst] a b`.
#[allow(dead_code)]
pub fn algebra_mul(a: Expr, b: Expr) -> Expr {
    app2(cst("Mul.mul"), a, b)
}
/// Build `Neg.neg {α} [inst] a`.
#[allow(dead_code)]
pub fn algebra_neg(a: Expr) -> Expr {
    app(cst("Neg.neg"), a)
}
/// Build `Inv.inv {α} [inst] a`.
#[allow(dead_code)]
pub fn algebra_inv(a: Expr) -> Expr {
    app(cst("Inv.inv"), a)
}
/// Build `Zero.zero {α} [inst]` — the zero element for type `ty`.
#[allow(dead_code)]
pub fn algebra_zero(ty: Expr) -> Expr {
    app(cst("Zero.zero"), ty)
}
/// Build `One.one {α} [inst]` — the one element for type `ty`.
#[allow(dead_code)]
pub fn algebra_one(ty: Expr) -> Expr {
    app(cst("One.one"), ty)
}
/// Build `Sub.sub {α} [inst] a b`.
#[allow(dead_code)]
pub fn algebra_sub(a: Expr, b: Expr) -> Expr {
    app2(cst("Sub.sub"), a, b)
}
/// Build `Div.div {α} [inst] a b`.
#[allow(dead_code)]
pub fn algebra_div(a: Expr, b: Expr) -> Expr {
    app2(cst("Div.div"), a, b)
}
/// Build the algebra environment with all algebraic type classes, methods, and laws.
#[allow(clippy::too_many_lines)]
pub fn build_algebra_env(env: &mut Environment) -> Result<(), String> {
    add_axiom(env, "Zero", vec![], mk_class_ty())?;
    add_axiom(env, "Zero.zero", vec![], mk_nullary_method("Zero"))?;
    add_axiom(env, "One", vec![], mk_class_ty())?;
    add_axiom(env, "One.one", vec![], mk_nullary_method("One"))?;
    add_axiom(env, "Add", vec![], mk_class_ty())?;
    add_axiom(env, "Add.add", vec![], mk_binop_method("Add"))?;
    add_axiom(env, "Mul", vec![], mk_class_ty())?;
    add_axiom(env, "Mul.mul", vec![], mk_binop_method("Mul"))?;
    add_axiom(env, "Neg", vec![], mk_class_ty())?;
    add_axiom(env, "Neg.neg", vec![], mk_unop_method("Neg"))?;
    add_axiom(env, "Inv", vec![], mk_class_ty())?;
    add_axiom(env, "Inv.inv", vec![], mk_unop_method("Inv"))?;
    add_axiom(env, "Sub", vec![], mk_class_ty())?;
    add_axiom(env, "Sub.sub", vec![], mk_binop_method("Sub"))?;
    add_axiom(env, "Div", vec![], mk_class_ty())?;
    add_axiom(env, "Div.div", vec![], mk_binop_method("Div"))?;
    add_axiom(env, "AddMonoid", vec![], mk_class_ty())?;
    add_axiom(
        env,
        "AddMonoid.toZero",
        vec![],
        pi(
            BinderInfo::Implicit,
            "α",
            type0(),
            pi(
                BinderInfo::InstImplicit,
                "_inst",
                app(cst("AddMonoid"), bvar(0)),
                app(cst("Zero"), bvar(1)),
            ),
        ),
    )?;
    add_axiom(
        env,
        "AddMonoid.toAdd",
        vec![],
        pi(
            BinderInfo::Implicit,
            "α",
            type0(),
            pi(
                BinderInfo::InstImplicit,
                "_inst",
                app(cst("AddMonoid"), bvar(0)),
                app(cst("Add"), bvar(1)),
            ),
        ),
    )?;
    add_axiom(
        env,
        "add_zero",
        vec![],
        mk_law_forall1("AddMonoid", |depth| {
            let alpha = bvar(depth - 1);
            let a = bvar(0);
            mk_eq(alpha, app2(cst("Add.add"), a.clone(), cst("Zero.zero")), a)
        }),
    )?;
    add_axiom(
        env,
        "zero_add",
        vec![],
        mk_law_forall1("AddMonoid", |depth| {
            let alpha = bvar(depth - 1);
            let a = bvar(0);
            mk_eq(alpha, app2(cst("Add.add"), cst("Zero.zero"), a.clone()), a)
        }),
    )?;
    add_axiom(
        env,
        "add_assoc",
        vec![],
        mk_law_forall3("AddMonoid", |depth| {
            let alpha = bvar(depth - 1);
            let a = bvar(2);
            let b = bvar(1);
            let c = bvar(0);
            mk_eq(
                alpha,
                app2(
                    cst("Add.add"),
                    app2(cst("Add.add"), a.clone(), b.clone()),
                    c.clone(),
                ),
                app2(cst("Add.add"), a, app2(cst("Add.add"), b, c)),
            )
        }),
    )?;
    add_axiom(env, "MulMonoid", vec![], mk_class_ty())?;
    add_axiom(
        env,
        "MulMonoid.toOne",
        vec![],
        pi(
            BinderInfo::Implicit,
            "α",
            type0(),
            pi(
                BinderInfo::InstImplicit,
                "_inst",
                app(cst("MulMonoid"), bvar(0)),
                app(cst("One"), bvar(1)),
            ),
        ),
    )?;
    add_axiom(
        env,
        "MulMonoid.toMul",
        vec![],
        pi(
            BinderInfo::Implicit,
            "α",
            type0(),
            pi(
                BinderInfo::InstImplicit,
                "_inst",
                app(cst("MulMonoid"), bvar(0)),
                app(cst("Mul"), bvar(1)),
            ),
        ),
    )?;
    add_axiom(
        env,
        "mul_one",
        vec![],
        mk_law_forall1("MulMonoid", |depth| {
            let alpha = bvar(depth - 1);
            let a = bvar(0);
            mk_eq(alpha, app2(cst("Mul.mul"), a.clone(), cst("One.one")), a)
        }),
    )?;
    add_axiom(
        env,
        "one_mul",
        vec![],
        mk_law_forall1("MulMonoid", |depth| {
            let alpha = bvar(depth - 1);
            let a = bvar(0);
            mk_eq(alpha, app2(cst("Mul.mul"), cst("One.one"), a.clone()), a)
        }),
    )?;
    add_axiom(
        env,
        "mul_assoc",
        vec![],
        mk_law_forall3("MulMonoid", |depth| {
            let alpha = bvar(depth - 1);
            let a = bvar(2);
            let b = bvar(1);
            let c = bvar(0);
            mk_eq(
                alpha,
                app2(
                    cst("Mul.mul"),
                    app2(cst("Mul.mul"), a.clone(), b.clone()),
                    c.clone(),
                ),
                app2(cst("Mul.mul"), a, app2(cst("Mul.mul"), b, c)),
            )
        }),
    )?;
    add_axiom(env, "AddGroup", vec![], mk_class_ty())?;
    add_axiom(
        env,
        "AddGroup.toAddMonoid",
        vec![],
        pi(
            BinderInfo::Implicit,
            "α",
            type0(),
            pi(
                BinderInfo::InstImplicit,
                "_inst",
                app(cst("AddGroup"), bvar(0)),
                app(cst("AddMonoid"), bvar(1)),
            ),
        ),
    )?;
    add_axiom(
        env,
        "AddGroup.toNeg",
        vec![],
        pi(
            BinderInfo::Implicit,
            "α",
            type0(),
            pi(
                BinderInfo::InstImplicit,
                "_inst",
                app(cst("AddGroup"), bvar(0)),
                app(cst("Neg"), bvar(1)),
            ),
        ),
    )?;
    add_axiom(
        env,
        "add_neg_cancel",
        vec![],
        mk_law_forall1("AddGroup", |depth| {
            let alpha = bvar(depth - 1);
            let a = bvar(0);
            mk_eq(
                alpha,
                app2(cst("Add.add"), a.clone(), app(cst("Neg.neg"), a)),
                cst("Zero.zero"),
            )
        }),
    )?;
    add_axiom(
        env,
        "neg_add_cancel",
        vec![],
        mk_law_forall1("AddGroup", |depth| {
            let alpha = bvar(depth - 1);
            let a = bvar(0);
            mk_eq(
                alpha,
                app2(cst("Add.add"), app(cst("Neg.neg"), a.clone()), a),
                cst("Zero.zero"),
            )
        }),
    )?;
    add_axiom(env, "MulGroup", vec![], mk_class_ty())?;
    add_axiom(
        env,
        "MulGroup.toMulMonoid",
        vec![],
        pi(
            BinderInfo::Implicit,
            "α",
            type0(),
            pi(
                BinderInfo::InstImplicit,
                "_inst",
                app(cst("MulGroup"), bvar(0)),
                app(cst("MulMonoid"), bvar(1)),
            ),
        ),
    )?;
    add_axiom(
        env,
        "MulGroup.toInv",
        vec![],
        pi(
            BinderInfo::Implicit,
            "α",
            type0(),
            pi(
                BinderInfo::InstImplicit,
                "_inst",
                app(cst("MulGroup"), bvar(0)),
                app(cst("Inv"), bvar(1)),
            ),
        ),
    )?;
    add_axiom(
        env,
        "mul_inv_cancel",
        vec![],
        mk_law_forall1("MulGroup", |depth| {
            let alpha = bvar(depth - 1);
            let a = bvar(0);
            mk_eq(
                alpha,
                app2(cst("Mul.mul"), a.clone(), app(cst("Inv.inv"), a)),
                cst("One.one"),
            )
        }),
    )?;
    add_axiom(
        env,
        "inv_mul_cancel",
        vec![],
        mk_law_forall1("MulGroup", |depth| {
            let alpha = bvar(depth - 1);
            let a = bvar(0);
            mk_eq(
                alpha,
                app2(cst("Mul.mul"), app(cst("Inv.inv"), a.clone()), a),
                cst("One.one"),
            )
        }),
    )?;
    add_axiom(env, "AddCommMonoid", vec![], mk_class_ty())?;
    add_axiom(
        env,
        "AddCommMonoid.toAddMonoid",
        vec![],
        pi(
            BinderInfo::Implicit,
            "α",
            type0(),
            pi(
                BinderInfo::InstImplicit,
                "_inst",
                app(cst("AddCommMonoid"), bvar(0)),
                app(cst("AddMonoid"), bvar(1)),
            ),
        ),
    )?;
    add_axiom(
        env,
        "add_comm",
        vec![],
        mk_law_forall2("AddCommMonoid", |depth| {
            let alpha = bvar(depth - 1);
            let a = bvar(1);
            let b = bvar(0);
            mk_eq(
                alpha,
                app2(cst("Add.add"), a.clone(), b.clone()),
                app2(cst("Add.add"), b, a),
            )
        }),
    )?;
    add_axiom(env, "MulCommMonoid", vec![], mk_class_ty())?;
    add_axiom(
        env,
        "MulCommMonoid.toMulMonoid",
        vec![],
        pi(
            BinderInfo::Implicit,
            "α",
            type0(),
            pi(
                BinderInfo::InstImplicit,
                "_inst",
                app(cst("MulCommMonoid"), bvar(0)),
                app(cst("MulMonoid"), bvar(1)),
            ),
        ),
    )?;
    add_axiom(
        env,
        "mul_comm",
        vec![],
        mk_law_forall2("MulCommMonoid", |depth| {
            let alpha = bvar(depth - 1);
            let a = bvar(1);
            let b = bvar(0);
            mk_eq(
                alpha,
                app2(cst("Mul.mul"), a.clone(), b.clone()),
                app2(cst("Mul.mul"), b, a),
            )
        }),
    )?;
    add_axiom(env, "AddCommGroup", vec![], mk_class_ty())?;
    add_axiom(
        env,
        "AddCommGroup.toAddGroup",
        vec![],
        pi(
            BinderInfo::Implicit,
            "α",
            type0(),
            pi(
                BinderInfo::InstImplicit,
                "_inst",
                app(cst("AddCommGroup"), bvar(0)),
                app(cst("AddGroup"), bvar(1)),
            ),
        ),
    )?;
    add_axiom(
        env,
        "AddCommGroup.toAddCommMonoid",
        vec![],
        pi(
            BinderInfo::Implicit,
            "α",
            type0(),
            pi(
                BinderInfo::InstImplicit,
                "_inst",
                app(cst("AddCommGroup"), bvar(0)),
                app(cst("AddCommMonoid"), bvar(1)),
            ),
        ),
    )?;
    add_axiom(env, "MulCommGroup", vec![], mk_class_ty())?;
    add_axiom(
        env,
        "MulCommGroup.toMulGroup",
        vec![],
        pi(
            BinderInfo::Implicit,
            "α",
            type0(),
            pi(
                BinderInfo::InstImplicit,
                "_inst",
                app(cst("MulCommGroup"), bvar(0)),
                app(cst("MulGroup"), bvar(1)),
            ),
        ),
    )?;
    add_axiom(
        env,
        "MulCommGroup.toMulCommMonoid",
        vec![],
        pi(
            BinderInfo::Implicit,
            "α",
            type0(),
            pi(
                BinderInfo::InstImplicit,
                "_inst",
                app(cst("MulCommGroup"), bvar(0)),
                app(cst("MulCommMonoid"), bvar(1)),
            ),
        ),
    )?;
    add_axiom(env, "Semiring", vec![], mk_class_ty())?;
    add_axiom(
        env,
        "Semiring.toAddCommMonoid",
        vec![],
        pi(
            BinderInfo::Implicit,
            "α",
            type0(),
            pi(
                BinderInfo::InstImplicit,
                "_inst",
                app(cst("Semiring"), bvar(0)),
                app(cst("AddCommMonoid"), bvar(1)),
            ),
        ),
    )?;
    add_axiom(
        env,
        "Semiring.toMulMonoid",
        vec![],
        pi(
            BinderInfo::Implicit,
            "α",
            type0(),
            pi(
                BinderInfo::InstImplicit,
                "_inst",
                app(cst("Semiring"), bvar(0)),
                app(cst("MulMonoid"), bvar(1)),
            ),
        ),
    )?;
    add_axiom(
        env,
        "left_distrib",
        vec![],
        mk_law_forall3("Semiring", |depth| {
            let alpha = bvar(depth - 1);
            let a = bvar(2);
            let b = bvar(1);
            let c = bvar(0);
            mk_eq(
                alpha,
                app2(
                    cst("Mul.mul"),
                    a.clone(),
                    app2(cst("Add.add"), b.clone(), c.clone()),
                ),
                app2(
                    cst("Add.add"),
                    app2(cst("Mul.mul"), a.clone(), b),
                    app2(cst("Mul.mul"), a, c),
                ),
            )
        }),
    )?;
    add_axiom(
        env,
        "right_distrib",
        vec![],
        mk_law_forall3("Semiring", |depth| {
            let alpha = bvar(depth - 1);
            let a = bvar(2);
            let b = bvar(1);
            let c = bvar(0);
            mk_eq(
                alpha,
                app2(
                    cst("Mul.mul"),
                    app2(cst("Add.add"), a.clone(), b.clone()),
                    c.clone(),
                ),
                app2(
                    cst("Add.add"),
                    app2(cst("Mul.mul"), a, c.clone()),
                    app2(cst("Mul.mul"), b, c),
                ),
            )
        }),
    )?;
    add_axiom(
        env,
        "mul_zero",
        vec![],
        mk_law_forall1("Semiring", |depth| {
            let alpha = bvar(depth - 1);
            let a = bvar(0);
            mk_eq(
                alpha,
                app2(cst("Mul.mul"), a, cst("Zero.zero")),
                cst("Zero.zero"),
            )
        }),
    )?;
    add_axiom(
        env,
        "zero_mul",
        vec![],
        mk_law_forall1("Semiring", |depth| {
            let alpha = bvar(depth - 1);
            let a = bvar(0);
            mk_eq(
                alpha,
                app2(cst("Mul.mul"), cst("Zero.zero"), a),
                cst("Zero.zero"),
            )
        }),
    )?;
    add_axiom(env, "CommSemiring", vec![], mk_class_ty())?;
    add_axiom(
        env,
        "CommSemiring.toSemiring",
        vec![],
        pi(
            BinderInfo::Implicit,
            "α",
            type0(),
            pi(
                BinderInfo::InstImplicit,
                "_inst",
                app(cst("CommSemiring"), bvar(0)),
                app(cst("Semiring"), bvar(1)),
            ),
        ),
    )?;
    add_axiom(
        env,
        "CommSemiring.toMulCommMonoid",
        vec![],
        pi(
            BinderInfo::Implicit,
            "α",
            type0(),
            pi(
                BinderInfo::InstImplicit,
                "_inst",
                app(cst("CommSemiring"), bvar(0)),
                app(cst("MulCommMonoid"), bvar(1)),
            ),
        ),
    )?;
    add_axiom(env, "Ring", vec![], mk_class_ty())?;
    add_axiom(
        env,
        "Ring.toAddCommGroup",
        vec![],
        pi(
            BinderInfo::Implicit,
            "α",
            type0(),
            pi(
                BinderInfo::InstImplicit,
                "_inst",
                app(cst("Ring"), bvar(0)),
                app(cst("AddCommGroup"), bvar(1)),
            ),
        ),
    )?;
    add_axiom(
        env,
        "Ring.toSemiring",
        vec![],
        pi(
            BinderInfo::Implicit,
            "α",
            type0(),
            pi(
                BinderInfo::InstImplicit,
                "_inst",
                app(cst("Ring"), bvar(0)),
                app(cst("Semiring"), bvar(1)),
            ),
        ),
    )?;
    add_axiom(
        env,
        "Ring.toMulMonoid",
        vec![],
        pi(
            BinderInfo::Implicit,
            "α",
            type0(),
            pi(
                BinderInfo::InstImplicit,
                "_inst",
                app(cst("Ring"), bvar(0)),
                app(cst("MulMonoid"), bvar(1)),
            ),
        ),
    )?;
    add_axiom(
        env,
        "ring_left_distrib",
        vec![],
        mk_law_forall3("Ring", |depth| {
            let alpha = bvar(depth - 1);
            let a = bvar(2);
            let b = bvar(1);
            let c = bvar(0);
            mk_eq(
                alpha,
                app2(
                    cst("Mul.mul"),
                    a.clone(),
                    app2(cst("Add.add"), b.clone(), c.clone()),
                ),
                app2(
                    cst("Add.add"),
                    app2(cst("Mul.mul"), a.clone(), b),
                    app2(cst("Mul.mul"), a, c),
                ),
            )
        }),
    )?;
    add_axiom(
        env,
        "ring_right_distrib",
        vec![],
        mk_law_forall3("Ring", |depth| {
            let alpha = bvar(depth - 1);
            let a = bvar(2);
            let b = bvar(1);
            let c = bvar(0);
            mk_eq(
                alpha,
                app2(
                    cst("Mul.mul"),
                    app2(cst("Add.add"), a.clone(), b.clone()),
                    c.clone(),
                ),
                app2(
                    cst("Add.add"),
                    app2(cst("Mul.mul"), a, c.clone()),
                    app2(cst("Mul.mul"), b, c),
                ),
            )
        }),
    )?;
    add_axiom(
        env,
        "ring_sub_def",
        vec![],
        mk_law_forall2("Ring", |depth| {
            let alpha = bvar(depth - 1);
            let a = bvar(1);
            let b = bvar(0);
            mk_eq(
                alpha,
                app2(cst("Sub.sub"), a.clone(), b.clone()),
                app2(cst("Add.add"), a, app(cst("Neg.neg"), b)),
            )
        }),
    )?;
    add_axiom(env, "CommRing", vec![], mk_class_ty())?;
    add_axiom(
        env,
        "CommRing.toRing",
        vec![],
        pi(
            BinderInfo::Implicit,
            "α",
            type0(),
            pi(
                BinderInfo::InstImplicit,
                "_inst",
                app(cst("CommRing"), bvar(0)),
                app(cst("Ring"), bvar(1)),
            ),
        ),
    )?;
    add_axiom(
        env,
        "CommRing.toCommSemiring",
        vec![],
        pi(
            BinderInfo::Implicit,
            "α",
            type0(),
            pi(
                BinderInfo::InstImplicit,
                "_inst",
                app(cst("CommRing"), bvar(0)),
                app(cst("CommSemiring"), bvar(1)),
            ),
        ),
    )?;
    add_axiom(
        env,
        "comm_ring_mul_comm",
        vec![],
        mk_law_forall2("CommRing", |depth| {
            let alpha = bvar(depth - 1);
            let a = bvar(1);
            let b = bvar(0);
            mk_eq(
                alpha,
                app2(cst("Mul.mul"), a.clone(), b.clone()),
                app2(cst("Mul.mul"), b, a),
            )
        }),
    )?;
    add_axiom(env, "Field", vec![], mk_class_ty())?;
    add_axiom(
        env,
        "Field.toCommRing",
        vec![],
        pi(
            BinderInfo::Implicit,
            "α",
            type0(),
            pi(
                BinderInfo::InstImplicit,
                "_inst",
                app(cst("Field"), bvar(0)),
                app(cst("CommRing"), bvar(1)),
            ),
        ),
    )?;
    add_axiom(
        env,
        "Field.toInv",
        vec![],
        pi(
            BinderInfo::Implicit,
            "α",
            type0(),
            pi(
                BinderInfo::InstImplicit,
                "_inst",
                app(cst("Field"), bvar(0)),
                app(cst("Inv"), bvar(1)),
            ),
        ),
    )?;
    add_axiom(
        env,
        "Field.toDiv",
        vec![],
        pi(
            BinderInfo::Implicit,
            "α",
            type0(),
            pi(
                BinderInfo::InstImplicit,
                "_inst",
                app(cst("Field"), bvar(0)),
                app(cst("Div"), bvar(1)),
            ),
        ),
    )?;
    add_axiom(
        env,
        "field_div_def",
        vec![],
        mk_law_forall2("Field", |depth| {
            let alpha = bvar(depth - 1);
            let a = bvar(1);
            let b = bvar(0);
            mk_eq(
                alpha,
                app2(cst("Div.div"), a.clone(), b.clone()),
                app2(cst("Mul.mul"), a, app(cst("Inv.inv"), b)),
            )
        }),
    )?;
    add_axiom(
        env,
        "field_mul_inv_cancel",
        vec![],
        pi(
            BinderInfo::Implicit,
            "α",
            type0(),
            pi(
                BinderInfo::InstImplicit,
                "_inst",
                app(cst("Field"), bvar(0)),
                pi(
                    BinderInfo::Default,
                    "a",
                    bvar(1),
                    pi(
                        BinderInfo::Default,
                        "hne",
                        app(cst("Not"), mk_eq(bvar(2), bvar(0), cst("Zero.zero"))),
                        mk_eq(
                            bvar(3),
                            app2(cst("Mul.mul"), bvar(1), app(cst("Inv.inv"), bvar(1))),
                            cst("One.one"),
                        ),
                    ),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "field_inv_mul_cancel",
        vec![],
        pi(
            BinderInfo::Implicit,
            "α",
            type0(),
            pi(
                BinderInfo::InstImplicit,
                "_inst",
                app(cst("Field"), bvar(0)),
                pi(
                    BinderInfo::Default,
                    "a",
                    bvar(1),
                    pi(
                        BinderInfo::Default,
                        "hne",
                        app(cst("Not"), mk_eq(bvar(2), bvar(0), cst("Zero.zero"))),
                        mk_eq(
                            bvar(3),
                            app2(cst("Mul.mul"), app(cst("Inv.inv"), bvar(1)), bvar(1)),
                            cst("One.one"),
                        ),
                    ),
                ),
            ),
        ),
    )?;
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
        let not_ty = pi(BinderInfo::Default, "p", prop(), prop());
        add_axiom(&mut env, "Not", vec![], not_ty).expect("operation should succeed");
        build_algebra_env(&mut env).expect("build_algebra_env should succeed");
        env
    }
    #[test]
    fn test_build_algebra_env_succeeds() {
        let _env = setup_env();
    }
    #[test]
    fn test_zero_class() {
        let env = setup_env();
        assert!(env.get(&Name::str("Zero")).is_some());
        assert!(env.get(&Name::str("Zero.zero")).is_some());
    }
    #[test]
    fn test_one_class() {
        let env = setup_env();
        assert!(env.get(&Name::str("One")).is_some());
        assert!(env.get(&Name::str("One.one")).is_some());
    }
    #[test]
    fn test_add_class() {
        let env = setup_env();
        assert!(env.get(&Name::str("Add")).is_some());
        assert!(env.get(&Name::str("Add.add")).is_some());
    }
    #[test]
    fn test_mul_class() {
        let env = setup_env();
        assert!(env.get(&Name::str("Mul")).is_some());
        assert!(env.get(&Name::str("Mul.mul")).is_some());
    }
    #[test]
    fn test_neg_class() {
        let env = setup_env();
        assert!(env.get(&Name::str("Neg")).is_some());
        assert!(env.get(&Name::str("Neg.neg")).is_some());
    }
    #[test]
    fn test_inv_class() {
        let env = setup_env();
        assert!(env.get(&Name::str("Inv")).is_some());
        assert!(env.get(&Name::str("Inv.inv")).is_some());
    }
    #[test]
    fn test_sub_class() {
        let env = setup_env();
        assert!(env.get(&Name::str("Sub")).is_some());
        assert!(env.get(&Name::str("Sub.sub")).is_some());
    }
    #[test]
    fn test_div_class() {
        let env = setup_env();
        assert!(env.get(&Name::str("Div")).is_some());
        assert!(env.get(&Name::str("Div.div")).is_some());
    }
    #[test]
    fn test_add_monoid_class() {
        let env = setup_env();
        assert!(env.get(&Name::str("AddMonoid")).is_some());
        assert!(env.get(&Name::str("AddMonoid.toZero")).is_some());
        assert!(env.get(&Name::str("AddMonoid.toAdd")).is_some());
    }
    #[test]
    fn test_add_monoid_laws() {
        let env = setup_env();
        assert!(env.get(&Name::str("add_zero")).is_some());
        assert!(env.get(&Name::str("zero_add")).is_some());
        assert!(env.get(&Name::str("add_assoc")).is_some());
    }
    #[test]
    fn test_mul_monoid_class() {
        let env = setup_env();
        assert!(env.get(&Name::str("MulMonoid")).is_some());
        assert!(env.get(&Name::str("MulMonoid.toOne")).is_some());
        assert!(env.get(&Name::str("MulMonoid.toMul")).is_some());
    }
    #[test]
    fn test_mul_monoid_laws() {
        let env = setup_env();
        assert!(env.get(&Name::str("mul_one")).is_some());
        assert!(env.get(&Name::str("one_mul")).is_some());
        assert!(env.get(&Name::str("mul_assoc")).is_some());
    }
    #[test]
    fn test_add_group_class() {
        let env = setup_env();
        assert!(env.get(&Name::str("AddGroup")).is_some());
        assert!(env.get(&Name::str("AddGroup.toAddMonoid")).is_some());
        assert!(env.get(&Name::str("AddGroup.toNeg")).is_some());
    }
    #[test]
    fn test_add_group_laws() {
        let env = setup_env();
        assert!(env.get(&Name::str("add_neg_cancel")).is_some());
        assert!(env.get(&Name::str("neg_add_cancel")).is_some());
    }
    #[test]
    fn test_mul_group_class() {
        let env = setup_env();
        assert!(env.get(&Name::str("MulGroup")).is_some());
        assert!(env.get(&Name::str("MulGroup.toMulMonoid")).is_some());
        assert!(env.get(&Name::str("MulGroup.toInv")).is_some());
    }
    #[test]
    fn test_mul_group_laws() {
        let env = setup_env();
        assert!(env.get(&Name::str("mul_inv_cancel")).is_some());
        assert!(env.get(&Name::str("inv_mul_cancel")).is_some());
    }
    #[test]
    fn test_add_comm_monoid() {
        let env = setup_env();
        assert!(env.get(&Name::str("AddCommMonoid")).is_some());
        assert!(env.get(&Name::str("AddCommMonoid.toAddMonoid")).is_some());
        assert!(env.get(&Name::str("add_comm")).is_some());
    }
    #[test]
    fn test_mul_comm_monoid() {
        let env = setup_env();
        assert!(env.get(&Name::str("MulCommMonoid")).is_some());
        assert!(env.get(&Name::str("MulCommMonoid.toMulMonoid")).is_some());
        assert!(env.get(&Name::str("mul_comm")).is_some());
    }
    #[test]
    fn test_add_comm_group() {
        let env = setup_env();
        assert!(env.get(&Name::str("AddCommGroup")).is_some());
        assert!(env.get(&Name::str("AddCommGroup.toAddGroup")).is_some());
        assert!(env
            .get(&Name::str("AddCommGroup.toAddCommMonoid"))
            .is_some());
    }
    #[test]
    fn test_mul_comm_group() {
        let env = setup_env();
        assert!(env.get(&Name::str("MulCommGroup")).is_some());
        assert!(env.get(&Name::str("MulCommGroup.toMulGroup")).is_some());
        assert!(env
            .get(&Name::str("MulCommGroup.toMulCommMonoid"))
            .is_some());
    }
    #[test]
    fn test_semiring_class() {
        let env = setup_env();
        assert!(env.get(&Name::str("Semiring")).is_some());
        assert!(env.get(&Name::str("Semiring.toAddCommMonoid")).is_some());
        assert!(env.get(&Name::str("Semiring.toMulMonoid")).is_some());
    }
    #[test]
    fn test_semiring_laws() {
        let env = setup_env();
        assert!(env.get(&Name::str("left_distrib")).is_some());
        assert!(env.get(&Name::str("right_distrib")).is_some());
        assert!(env.get(&Name::str("mul_zero")).is_some());
        assert!(env.get(&Name::str("zero_mul")).is_some());
    }
    #[test]
    fn test_comm_semiring() {
        let env = setup_env();
        assert!(env.get(&Name::str("CommSemiring")).is_some());
        assert!(env.get(&Name::str("CommSemiring.toSemiring")).is_some());
        assert!(env
            .get(&Name::str("CommSemiring.toMulCommMonoid"))
            .is_some());
    }
    #[test]
    fn test_ring_class() {
        let env = setup_env();
        assert!(env.get(&Name::str("Ring")).is_some());
        assert!(env.get(&Name::str("Ring.toAddCommGroup")).is_some());
        assert!(env.get(&Name::str("Ring.toSemiring")).is_some());
        assert!(env.get(&Name::str("Ring.toMulMonoid")).is_some());
    }
    #[test]
    fn test_ring_laws() {
        let env = setup_env();
        assert!(env.get(&Name::str("ring_left_distrib")).is_some());
        assert!(env.get(&Name::str("ring_right_distrib")).is_some());
        assert!(env.get(&Name::str("ring_sub_def")).is_some());
    }
    #[test]
    fn test_comm_ring() {
        let env = setup_env();
        assert!(env.get(&Name::str("CommRing")).is_some());
        assert!(env.get(&Name::str("CommRing.toRing")).is_some());
        assert!(env.get(&Name::str("CommRing.toCommSemiring")).is_some());
        assert!(env.get(&Name::str("comm_ring_mul_comm")).is_some());
    }
    #[test]
    fn test_field_class() {
        let env = setup_env();
        assert!(env.get(&Name::str("Field")).is_some());
        assert!(env.get(&Name::str("Field.toCommRing")).is_some());
        assert!(env.get(&Name::str("Field.toInv")).is_some());
        assert!(env.get(&Name::str("Field.toDiv")).is_some());
    }
    #[test]
    fn test_field_laws() {
        let env = setup_env();
        assert!(env.get(&Name::str("field_div_def")).is_some());
        assert!(env.get(&Name::str("field_mul_inv_cancel")).is_some());
        assert!(env.get(&Name::str("field_inv_mul_cancel")).is_some());
    }
    #[test]
    fn test_class_types_are_type_to_type1() {
        let env = setup_env();
        let classes = [
            "Zero",
            "One",
            "Add",
            "Mul",
            "Neg",
            "Inv",
            "Sub",
            "Div",
            "AddMonoid",
            "MulMonoid",
            "AddGroup",
            "MulGroup",
            "AddCommMonoid",
            "MulCommMonoid",
            "AddCommGroup",
            "MulCommGroup",
            "Semiring",
            "CommSemiring",
            "Ring",
            "CommRing",
            "Field",
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
    fn test_binop_method_types() {
        let env = setup_env();
        let methods = ["Add.add", "Mul.mul", "Sub.sub", "Div.div"];
        for method_name in &methods {
            let decl = env.get(&Name::str(*method_name));
            assert!(decl.is_some(), "Missing method: {}", method_name);
            if let Some(Declaration::Axiom { ty, .. }) = decl {
                assert!(
                    matches!(ty, Expr::Pi(BinderInfo::Implicit, _, _, _)),
                    "Method {} should start with implicit binder, got {:?}",
                    method_name,
                    ty
                );
            }
        }
    }
    #[test]
    fn test_unop_method_types() {
        let env = setup_env();
        let methods = ["Neg.neg", "Inv.inv"];
        for method_name in &methods {
            let decl = env.get(&Name::str(*method_name));
            assert!(decl.is_some(), "Missing method: {}", method_name);
            if let Some(Declaration::Axiom { ty, .. }) = decl {
                assert!(
                    matches!(ty, Expr::Pi(BinderInfo::Implicit, _, _, _)),
                    "Method {} should start with implicit binder, got {:?}",
                    method_name,
                    ty
                );
            }
        }
    }
    #[test]
    fn test_nullary_method_types() {
        let env = setup_env();
        let methods = ["Zero.zero", "One.one"];
        for method_name in &methods {
            let decl = env.get(&Name::str(*method_name));
            assert!(decl.is_some(), "Missing method: {}", method_name);
            if let Some(Declaration::Axiom { ty, .. }) = decl {
                assert!(
                    matches!(ty, Expr::Pi(BinderInfo::Implicit, _, _, _)),
                    "Method {} should start with implicit binder, got {:?}",
                    method_name,
                    ty
                );
            }
        }
    }
    #[test]
    fn test_algebra_add_produces_app() {
        let a = bvar(0);
        let b = bvar(1);
        let result = algebra_add(a, b);
        assert!(matches!(result, Expr::App(_, _)));
    }
    #[test]
    fn test_algebra_mul_produces_app() {
        let a = bvar(0);
        let b = bvar(1);
        let result = algebra_mul(a, b);
        assert!(matches!(result, Expr::App(_, _)));
    }
    #[test]
    fn test_algebra_neg_produces_app() {
        let a = bvar(0);
        let result = algebra_neg(a);
        assert!(matches!(result, Expr::App(_, _)));
    }
    #[test]
    fn test_algebra_inv_produces_app() {
        let a = bvar(0);
        let result = algebra_inv(a);
        assert!(matches!(result, Expr::App(_, _)));
    }
    #[test]
    fn test_algebra_zero_produces_app() {
        let ty = type0();
        let result = algebra_zero(ty);
        assert!(matches!(result, Expr::App(_, _)));
    }
    #[test]
    fn test_algebra_one_produces_app() {
        let ty = type0();
        let result = algebra_one(ty);
        assert!(matches!(result, Expr::App(_, _)));
    }
    #[test]
    fn test_algebra_sub_produces_app() {
        let a = bvar(0);
        let b = bvar(1);
        let result = algebra_sub(a, b);
        assert!(matches!(result, Expr::App(_, _)));
    }
    #[test]
    fn test_algebra_div_produces_app() {
        let a = bvar(0);
        let b = bvar(1);
        let result = algebra_div(a, b);
        assert!(matches!(result, Expr::App(_, _)));
    }
    #[test]
    fn test_add_zero_type_structure() {
        let env = setup_env();
        let decl = env
            .get(&Name::str("add_zero"))
            .expect("declaration 'add_zero' should exist in env");
        if let Declaration::Axiom { ty, .. } = decl {
            assert!(matches!(ty, Expr::Pi(BinderInfo::Implicit, _, _, _)));
        } else {
            panic!("add_zero should be an Axiom");
        }
    }
    #[test]
    fn test_left_distrib_type_structure() {
        let env = setup_env();
        let decl = env
            .get(&Name::str("left_distrib"))
            .expect("declaration 'left_distrib' should exist in env");
        if let Declaration::Axiom { ty, .. } = decl {
            assert!(matches!(ty, Expr::Pi(BinderInfo::Implicit, _, _, _)));
        } else {
            panic!("left_distrib should be an Axiom");
        }
    }
    #[test]
    fn test_field_mul_inv_cancel_type_structure() {
        let env = setup_env();
        let decl = env
            .get(&Name::str("field_mul_inv_cancel"))
            .expect("declaration 'field_mul_inv_cancel' should exist in env");
        if let Declaration::Axiom { ty, .. } = decl {
            assert!(matches!(ty, Expr::Pi(BinderInfo::Implicit, _, _, _)));
        } else {
            panic!("field_mul_inv_cancel should be an Axiom");
        }
    }
    #[test]
    fn test_mk_class_ty() {
        let ty = mk_class_ty();
        assert!(matches!(ty, Expr::Pi(BinderInfo::Default, _, _, _)));
    }
    #[test]
    fn test_mk_typeclass() {
        let tc = mk_typeclass("Add");
        assert!(matches!(tc, Expr::Const(_, _)));
    }
    #[test]
    fn test_mk_nullary_method_structure() {
        let m = mk_nullary_method("Zero");
        if let Expr::Pi(BinderInfo::Implicit, _, dom, body) = m {
            assert!(matches!(*dom, Expr::Sort(_)));
            assert!(matches!(*body, Expr::Pi(BinderInfo::InstImplicit, _, _, _)));
        } else {
            panic!("Unexpected structure");
        }
    }
    #[test]
    fn test_mk_binop_method_structure() {
        let m = mk_binop_method("Add");
        if let Expr::Pi(BinderInfo::Implicit, _, _, body) = m {
            if let Expr::Pi(BinderInfo::InstImplicit, _, _, body2) = *body {
                assert!(matches!(*body2, Expr::Pi(BinderInfo::Default, _, _, _)));
            } else {
                panic!("Expected InstImplicit");
            }
        } else {
            panic!("Expected Implicit");
        }
    }
    #[test]
    fn test_mk_unop_method_structure() {
        let m = mk_unop_method("Neg");
        if let Expr::Pi(BinderInfo::Implicit, _, _, body) = m {
            if let Expr::Pi(BinderInfo::InstImplicit, _, _, body2) = *body {
                assert!(matches!(*body2, Expr::Pi(BinderInfo::Default, _, _, _)));
            } else {
                panic!("Expected InstImplicit");
            }
        } else {
            panic!("Expected Implicit");
        }
    }
    #[test]
    fn test_mk_eq_structure() {
        let eq_expr = mk_eq(type0(), bvar(0), bvar(1));
        assert!(matches!(eq_expr, Expr::App(_, _)));
    }
    #[test]
    fn test_total_declarations_count() {
        let env = setup_env();
        let all_names = [
            "Zero",
            "Zero.zero",
            "One",
            "One.one",
            "Add",
            "Add.add",
            "Mul",
            "Mul.mul",
            "Neg",
            "Neg.neg",
            "Inv",
            "Inv.inv",
            "Sub",
            "Sub.sub",
            "Div",
            "Div.div",
            "AddMonoid",
            "AddMonoid.toZero",
            "AddMonoid.toAdd",
            "add_zero",
            "zero_add",
            "add_assoc",
            "MulMonoid",
            "MulMonoid.toOne",
            "MulMonoid.toMul",
            "mul_one",
            "one_mul",
            "mul_assoc",
            "AddGroup",
            "AddGroup.toAddMonoid",
            "AddGroup.toNeg",
            "add_neg_cancel",
            "neg_add_cancel",
            "MulGroup",
            "MulGroup.toMulMonoid",
            "MulGroup.toInv",
            "mul_inv_cancel",
            "inv_mul_cancel",
            "AddCommMonoid",
            "AddCommMonoid.toAddMonoid",
            "add_comm",
            "MulCommMonoid",
            "MulCommMonoid.toMulMonoid",
            "mul_comm",
            "AddCommGroup",
            "AddCommGroup.toAddGroup",
            "AddCommGroup.toAddCommMonoid",
            "MulCommGroup",
            "MulCommGroup.toMulGroup",
            "MulCommGroup.toMulCommMonoid",
            "Semiring",
            "Semiring.toAddCommMonoid",
            "Semiring.toMulMonoid",
            "left_distrib",
            "right_distrib",
            "mul_zero",
            "zero_mul",
            "CommSemiring",
            "CommSemiring.toSemiring",
            "CommSemiring.toMulCommMonoid",
            "Ring",
            "Ring.toAddCommGroup",
            "Ring.toSemiring",
            "Ring.toMulMonoid",
            "ring_left_distrib",
            "ring_right_distrib",
            "ring_sub_def",
            "CommRing",
            "CommRing.toRing",
            "CommRing.toCommSemiring",
            "comm_ring_mul_comm",
            "Field",
            "Field.toCommRing",
            "Field.toInv",
            "Field.toDiv",
            "field_div_def",
            "field_mul_inv_cancel",
            "field_inv_mul_cancel",
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
#[allow(dead_code)]
pub fn euler_phi_algebra(n: u64) -> u64 {
    let mut result = n;
    let mut temp = n;
    let mut p = 2u64;
    while p * p <= temp {
        if temp % p == 0 {
            while temp % p == 0 {
                temp /= p;
            }
            result -= result / p;
        }
        p += 1;
    }
    if temp > 1 {
        result -= result / temp;
    }
    result
}
#[cfg(test)]
mod algebra_ext_tests {
    use super::*;
    #[test]
    fn test_module_over_pid() {
        let m = Module::free("Z", "Z^3", 3);
        assert!(m.is_free);
        assert!(m.is_projective());
        assert!(!m.over_pid_structure_theorem().is_empty());
    }
    #[test]
    fn test_graded_ring() {
        let poly = GradedRing::polynomial_ring(&["x", "y", "z"]);
        assert!(poly.is_commutative);
        assert!(poly.is_noetherian());
    }
    #[test]
    fn test_localization() {
        let loc = Localization::at_prime("Z", "(5)");
        assert!(!loc.universal_property().is_empty());
    }
    #[test]
    fn test_galois_extension() {
        let cyc = GaloisExtension::cyclotomic(8);
        assert_eq!(cyc.extension_field, "Q(zeta_8)");
        assert!(!cyc.fundamental_theorem().is_empty());
    }
    #[test]
    fn test_noetherian_ring() {
        let r = NoetherianRing::new("Z[x,y]", Some(2));
        assert!(r.primary_decomposition_exists());
        assert!(!r.hilbert_basis_theorem().is_empty());
    }
    #[test]
    fn test_ext_tor() {
        let et = ExtTorGroups::new("Z", "Z/2Z", "Z/3Z");
        assert!(!et.ext_description().is_empty());
        assert!(!et.tor_description().is_empty());
    }
}
#[cfg(test)]
mod algebra_cat_tests {
    use super::*;
    #[test]
    fn test_abelian_category() {
        let r_mod = AbelianCategory::of_modules("Z");
        assert!(r_mod.has_zero_object);
        assert!(!r_mod.snake_lemma().is_empty());
    }
    #[test]
    fn test_ses() {
        let ses = ShortExactSequence::new("Z", "Z", "Z/2Z");
        assert!(!ses.display().is_empty());
    }
}
#[cfg(test)]
mod homological_algebra_tests {
    use super::*;
    #[test]
    fn test_projective_resolution() {
        let pr = ProjectiveResolution::new("Z/pZ", Some(1), 1);
        assert!(pr.is_finite());
    }
    #[test]
    fn test_derived_category() {
        let dc = DerivedCategory::new("Ab");
        assert!(dc.objects_are_complexes());
        assert!(!dc.localization_description().is_empty());
    }
}
#[cfg(test)]
mod lie_algebra_tests {
    use super::*;
    #[test]
    fn test_sl2() {
        let sl2 = LieAlgebra::sl_n(2);
        assert_eq!(sl2.dimension, 3);
        assert!(sl2.is_semisimple);
        assert!(!sl2.is_solvable);
    }
    #[test]
    fn test_heisenberg() {
        let h = LieAlgebra::heisenberg(1);
        assert_eq!(h.dimension, 3);
        assert!(h.is_nilpotent);
        assert!(h.is_solvable);
    }
}
