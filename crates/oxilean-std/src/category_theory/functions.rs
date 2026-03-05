//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{
    Bicategory, Doctrine, DoubleCategory, EnrichedCat, EnrichedCategory, GluingConstruction,
    InftyCat, InftyCatModel, KanExtension, LocallyCartesianClosed, ModelCategory, Monad2Cat,
    MonoidalCategory, Topos,
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
pub fn cst(s: &str) -> Expr {
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
pub fn type2() -> Expr {
    Expr::Sort(Level::succ(Level::succ(Level::succ(Level::zero()))))
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
/// Build `Exists @{} alpha pred`.
#[allow(dead_code)]
pub fn mk_exists(ty: Expr, pred: Expr) -> Expr {
    app2(cst("Exists"), ty, pred)
}
/// Build `Category.Hom {C} [inst] a b`.
#[allow(dead_code)]
pub fn cat_hom(a: Expr, b: Expr) -> Expr {
    app2(cst("Category.Hom"), a, b)
}
/// Build `Category.id {C} [inst] a`.
#[allow(dead_code)]
pub fn cat_id(a: Expr) -> Expr {
    app(cst("Category.id"), a)
}
/// Build `Category.comp {C} [inst] f g`.
#[allow(dead_code)]
pub fn cat_comp(f: Expr, g: Expr) -> Expr {
    app2(cst("Category.comp"), f, g)
}
/// Build `Functor.obj {C D} [instC] [instD] F a`.
#[allow(dead_code)]
pub fn functor_obj(f: Expr, a: Expr) -> Expr {
    app2(cst("Functor.obj"), f, a)
}
/// Build `Functor.map {C D} [instC] [instD] F f`.
#[allow(dead_code)]
pub fn functor_map(funct: Expr, f: Expr) -> Expr {
    app2(cst("Functor.map"), funct, f)
}
/// Build `NatTrans.app {C D} [instC] [instD] {F G} eta a`.
#[allow(dead_code)]
pub fn nat_trans_app(eta: Expr, a: Expr) -> Expr {
    app2(cst("NatTrans.app"), eta, a)
}
/// Build `Iso {C} [inst] a b`.
#[allow(dead_code)]
pub fn cat_iso(a: Expr, b: Expr) -> Expr {
    app2(cst("Iso"), a, b)
}
/// Build `IsInitial {C} [inst] a`.
#[allow(dead_code)]
pub fn is_initial(a: Expr) -> Expr {
    app(cst("IsInitial"), a)
}
/// Build `IsTerminal {C} [inst] a`.
#[allow(dead_code)]
pub fn is_terminal(a: Expr) -> Expr {
    app(cst("IsTerminal"), a)
}
/// Build `IsMono {C} [inst] f`.
#[allow(dead_code)]
pub fn is_mono(f: Expr) -> Expr {
    app(cst("IsMono"), f)
}
/// Build `IsEpi {C} [inst] f`.
#[allow(dead_code)]
pub fn is_epi(f: Expr) -> Expr {
    app(cst("IsEpi"), f)
}
/// `{C : Type} -> [Category C] -> <body>`
#[allow(dead_code)]
pub fn mk_cat_law<F>(prop_builder: F) -> Expr
where
    F: FnOnce(u32) -> Expr,
{
    pi(
        BinderInfo::Implicit,
        "C",
        type0(),
        pi(
            BinderInfo::InstImplicit,
            "_inst",
            app(cst("Category"), bvar(0)),
            prop_builder(2),
        ),
    )
}
/// `{C : Type} -> [Category C] -> forall (a : C), <body>`
#[allow(dead_code)]
pub fn mk_cat_forall1<F>(prop_builder: F) -> Expr
where
    F: FnOnce(u32) -> Expr,
{
    pi(
        BinderInfo::Implicit,
        "C",
        type0(),
        pi(
            BinderInfo::InstImplicit,
            "_inst",
            app(cst("Category"), bvar(0)),
            pi(BinderInfo::Default, "a", bvar(1), prop_builder(3)),
        ),
    )
}
/// `{C : Type} -> [Category C] -> forall (a b : C), <body>`
#[allow(dead_code)]
pub fn mk_cat_forall2<F>(prop_builder: F) -> Expr
where
    F: FnOnce(u32) -> Expr,
{
    pi(
        BinderInfo::Implicit,
        "C",
        type0(),
        pi(
            BinderInfo::InstImplicit,
            "_inst",
            app(cst("Category"), bvar(0)),
            pi(
                BinderInfo::Default,
                "a",
                bvar(1),
                pi(BinderInfo::Default, "b", bvar(2), prop_builder(4)),
            ),
        ),
    )
}
/// `{C : Type} -> [Category C] -> forall (a b c : C), <body>`
#[allow(dead_code)]
pub fn mk_cat_forall3<F>(prop_builder: F) -> Expr
where
    F: FnOnce(u32) -> Expr,
{
    pi(
        BinderInfo::Implicit,
        "C",
        type0(),
        pi(
            BinderInfo::InstImplicit,
            "_inst",
            app(cst("Category"), bvar(0)),
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
/// `{C : Type} -> [Category C] -> forall (a b c d : C), <body>`
#[allow(dead_code)]
pub fn mk_cat_forall4<F>(prop_builder: F) -> Expr
where
    F: FnOnce(u32) -> Expr,
{
    pi(
        BinderInfo::Implicit,
        "C",
        type0(),
        pi(
            BinderInfo::InstImplicit,
            "_inst",
            app(cst("Category"), bvar(0)),
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
                        pi(BinderInfo::Default, "d", bvar(4), prop_builder(6)),
                    ),
                ),
            ),
        ),
    )
}
/// `{C D : Type} -> [Category C] -> [Category D] -> <body>`
#[allow(dead_code)]
pub fn mk_two_cat_law<F>(prop_builder: F) -> Expr
where
    F: FnOnce(u32) -> Expr,
{
    pi(
        BinderInfo::Implicit,
        "C",
        type0(),
        pi(
            BinderInfo::Implicit,
            "D",
            type0(),
            pi(
                BinderInfo::InstImplicit,
                "_instC",
                app(cst("Category"), bvar(1)),
                pi(
                    BinderInfo::InstImplicit,
                    "_instD",
                    app(cst("Category"), bvar(1)),
                    prop_builder(4),
                ),
            ),
        ),
    )
}
/// `{C D : Type} -> [Category C] -> [Category D] ->
///  forall (F : Functor C D), <body>`
#[allow(dead_code)]
pub fn mk_functor_forall1<F>(prop_builder: F) -> Expr
where
    F: FnOnce(u32) -> Expr,
{
    pi(
        BinderInfo::Implicit,
        "C",
        type0(),
        pi(
            BinderInfo::Implicit,
            "D",
            type0(),
            pi(
                BinderInfo::InstImplicit,
                "_instC",
                app(cst("Category"), bvar(1)),
                pi(
                    BinderInfo::InstImplicit,
                    "_instD",
                    app(cst("Category"), bvar(1)),
                    pi(
                        BinderInfo::Default,
                        "F",
                        app2(cst("Functor"), bvar(3), bvar(2)),
                        prop_builder(5),
                    ),
                ),
            ),
        ),
    )
}
/// `{C D : Type} -> [Category C] -> [Category D] ->
///  forall (F G : Functor C D), <body>`
#[allow(dead_code)]
pub fn mk_functor_forall2<F>(prop_builder: F) -> Expr
where
    F: FnOnce(u32) -> Expr,
{
    pi(
        BinderInfo::Implicit,
        "C",
        type0(),
        pi(
            BinderInfo::Implicit,
            "D",
            type0(),
            pi(
                BinderInfo::InstImplicit,
                "_instC",
                app(cst("Category"), bvar(1)),
                pi(
                    BinderInfo::InstImplicit,
                    "_instD",
                    app(cst("Category"), bvar(1)),
                    pi(
                        BinderInfo::Default,
                        "F",
                        app2(cst("Functor"), bvar(3), bvar(2)),
                        pi(
                            BinderInfo::Default,
                            "G",
                            app2(cst("Functor"), bvar(4), bvar(3)),
                            prop_builder(6),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// Build the category theory environment with categories, functors,
/// natural transformations, limits, and related concepts.
#[allow(clippy::too_many_lines)]
pub fn build_category_theory_env(env: &mut Environment) -> Result<(), String> {
    add_axiom(
        env,
        "Category",
        vec![],
        pi(BinderInfo::Default, "C", type0(), type2()),
    )?;
    add_axiom(
        env,
        "Category.Hom",
        vec![],
        mk_cat_law(|_depth| arrow(bvar(1), arrow(bvar(2), type0()))),
    )?;
    add_axiom(
        env,
        "Category.id",
        vec![],
        mk_cat_forall1(|_depth| {
            let a = bvar(0);
            app2(cst("Category.Hom"), a.clone(), a)
        }),
    )?;
    add_axiom(
        env,
        "Category.comp",
        vec![],
        pi(
            BinderInfo::Implicit,
            "C",
            type0(),
            pi(
                BinderInfo::InstImplicit,
                "_inst",
                app(cst("Category"), bvar(0)),
                pi(
                    BinderInfo::Implicit,
                    "a",
                    bvar(1),
                    pi(
                        BinderInfo::Implicit,
                        "b",
                        bvar(2),
                        pi(
                            BinderInfo::Implicit,
                            "c",
                            bvar(3),
                            pi(
                                BinderInfo::Default,
                                "f",
                                app2(cst("Category.Hom"), bvar(2), bvar(1)),
                                pi(
                                    BinderInfo::Default,
                                    "g",
                                    app2(cst("Category.Hom"), bvar(2), bvar(1)),
                                    app2(cst("Category.Hom"), bvar(4), bvar(2)),
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
        "Category.id_comp",
        vec![],
        mk_cat_forall2(|_depth| {
            let a = bvar(1);
            let b = bvar(0);
            pi(
                BinderInfo::Default,
                "f",
                app2(cst("Category.Hom"), a.clone(), b.clone()),
                mk_eq(
                    app2(cst("Category.Hom"), a.clone(), b),
                    app2(cst("Category.comp"), app(cst("Category.id"), a), bvar(0)),
                    bvar(0),
                ),
            )
        }),
    )?;
    add_axiom(
        env,
        "Category.comp_id",
        vec![],
        mk_cat_forall2(|_depth| {
            let a = bvar(1);
            let b = bvar(0);
            pi(
                BinderInfo::Default,
                "f",
                app2(cst("Category.Hom"), a.clone(), b.clone()),
                mk_eq(
                    app2(cst("Category.Hom"), a, b.clone()),
                    app2(cst("Category.comp"), bvar(0), app(cst("Category.id"), b)),
                    bvar(0),
                ),
            )
        }),
    )?;
    add_axiom(
        env,
        "Category.assoc",
        vec![],
        mk_cat_forall4(|_depth| {
            let a = bvar(3);
            let b = bvar(2);
            let c = bvar(1);
            let d = bvar(0);
            pi(
                BinderInfo::Default,
                "f",
                app2(cst("Category.Hom"), a.clone(), b.clone()),
                pi(
                    BinderInfo::Default,
                    "g",
                    app2(cst("Category.Hom"), b, c.clone()),
                    pi(
                        BinderInfo::Default,
                        "h",
                        app2(cst("Category.Hom"), c, d.clone()),
                        mk_eq(
                            app2(cst("Category.Hom"), a, d),
                            app2(
                                cst("Category.comp"),
                                app2(cst("Category.comp"), bvar(2), bvar(1)),
                                bvar(0),
                            ),
                            app2(
                                cst("Category.comp"),
                                bvar(2),
                                app2(cst("Category.comp"), bvar(1), bvar(0)),
                            ),
                        ),
                    ),
                ),
            )
        }),
    )?;
    add_axiom(
        env,
        "Iso",
        vec![],
        mk_cat_law(|_depth| arrow(bvar(1), arrow(bvar(2), type0()))),
    )?;
    add_axiom(
        env,
        "Iso.hom",
        vec![],
        mk_cat_forall2(|_depth| {
            let a = bvar(1);
            let b = bvar(0);
            arrow(
                app2(cst("Iso"), a.clone(), b.clone()),
                app2(cst("Category.Hom"), a, b),
            )
        }),
    )?;
    add_axiom(
        env,
        "Iso.inv",
        vec![],
        mk_cat_forall2(|_depth| {
            let a = bvar(1);
            let b = bvar(0);
            arrow(
                app2(cst("Iso"), a.clone(), b.clone()),
                app2(cst("Category.Hom"), b, a),
            )
        }),
    )?;
    add_axiom(
        env,
        "Iso.hom_inv",
        vec![],
        mk_cat_forall2(|_depth| {
            let a = bvar(1);
            let b = bvar(0);
            pi(
                BinderInfo::Default,
                "i",
                app2(cst("Iso"), a.clone(), b),
                mk_eq(
                    app2(cst("Category.Hom"), a.clone(), a.clone()),
                    app2(
                        cst("Category.comp"),
                        app(cst("Iso.hom"), bvar(0)),
                        app(cst("Iso.inv"), bvar(0)),
                    ),
                    app(cst("Category.id"), a),
                ),
            )
        }),
    )?;
    add_axiom(
        env,
        "Iso.inv_hom",
        vec![],
        mk_cat_forall2(|_depth| {
            let a = bvar(1);
            let b = bvar(0);
            pi(
                BinderInfo::Default,
                "i",
                app2(cst("Iso"), a, b.clone()),
                mk_eq(
                    app2(cst("Category.Hom"), b.clone(), b.clone()),
                    app2(
                        cst("Category.comp"),
                        app(cst("Iso.inv"), bvar(0)),
                        app(cst("Iso.hom"), bvar(0)),
                    ),
                    app(cst("Category.id"), b),
                ),
            )
        }),
    )?;
    add_axiom(
        env,
        "Iso.refl",
        vec![],
        mk_cat_forall1(|_depth| {
            let a = bvar(0);
            app2(cst("Iso"), a.clone(), a)
        }),
    )?;
    add_axiom(
        env,
        "Iso.symm",
        vec![],
        mk_cat_forall2(|_depth| {
            let a = bvar(1);
            let b = bvar(0);
            arrow(
                app2(cst("Iso"), a.clone(), b.clone()),
                app2(cst("Iso"), b, a),
            )
        }),
    )?;
    add_axiom(
        env,
        "Iso.trans",
        vec![],
        mk_cat_forall3(|_depth| {
            let a = bvar(2);
            let b = bvar(1);
            let c = bvar(0);
            arrow(
                app2(cst("Iso"), a.clone(), b.clone()),
                arrow(app2(cst("Iso"), b, c.clone()), app2(cst("Iso"), a, c)),
            )
        }),
    )?;
    add_axiom(
        env,
        "IsMono",
        vec![],
        mk_cat_forall2(|_depth| {
            let a = bvar(1);
            let b = bvar(0);
            arrow(app2(cst("Category.Hom"), a, b), prop())
        }),
    )?;
    add_axiom(
        env,
        "IsEpi",
        vec![],
        mk_cat_forall2(|_depth| {
            let a = bvar(1);
            let b = bvar(0);
            arrow(app2(cst("Category.Hom"), a, b), prop())
        }),
    )?;
    add_axiom(
        env,
        "Iso.isMono",
        vec![],
        mk_cat_forall2(|_depth| {
            let a = bvar(1);
            let b = bvar(0);
            pi(
                BinderInfo::Default,
                "i",
                app2(cst("Iso"), a, b),
                app(cst("IsMono"), app(cst("Iso.hom"), bvar(0))),
            )
        }),
    )?;
    add_axiom(
        env,
        "Iso.isEpi",
        vec![],
        mk_cat_forall2(|_depth| {
            let a = bvar(1);
            let b = bvar(0);
            pi(
                BinderInfo::Default,
                "i",
                app2(cst("Iso"), a, b),
                app(cst("IsEpi"), app(cst("Iso.hom"), bvar(0))),
            )
        }),
    )?;
    add_axiom(
        env,
        "IsMono.comp",
        vec![],
        mk_cat_forall3(|_depth| {
            let a = bvar(2);
            let b = bvar(1);
            let c = bvar(0);
            pi(
                BinderInfo::Default,
                "f",
                app2(cst("Category.Hom"), a.clone(), b.clone()),
                pi(
                    BinderInfo::Default,
                    "g",
                    app2(cst("Category.Hom"), b, c),
                    arrow(
                        app(cst("IsMono"), bvar(1)),
                        arrow(
                            app(cst("IsMono"), bvar(0)),
                            app(cst("IsMono"), app2(cst("Category.comp"), bvar(1), bvar(0))),
                        ),
                    ),
                ),
            )
        }),
    )?;
    add_axiom(env, "Functor", vec![], mk_two_cat_law(|_depth| type1()))?;
    add_axiom(
        env,
        "Functor.obj",
        vec![],
        mk_two_cat_law(|_depth| {
            let c = bvar(3);
            let d = bvar(2);
            arrow(app2(cst("Functor"), c.clone(), d.clone()), arrow(c, d))
        }),
    )?;
    add_axiom(
        env,
        "Functor.map",
        vec![],
        mk_two_cat_law(|_depth| {
            let c = bvar(3);
            let d = bvar(2);
            pi(
                BinderInfo::Default,
                "F",
                app2(cst("Functor"), c.clone(), d),
                pi(
                    BinderInfo::Implicit,
                    "a",
                    c.clone(),
                    pi(
                        BinderInfo::Implicit,
                        "b",
                        c,
                        arrow(
                            app2(cst("Category.Hom"), bvar(1), bvar(0)),
                            app2(
                                cst("Category.Hom"),
                                app2(cst("Functor.obj"), bvar(2), bvar(1)),
                                app2(cst("Functor.obj"), bvar(2), bvar(0)),
                            ),
                        ),
                    ),
                ),
            )
        }),
    )?;
    add_axiom(
        env,
        "Functor.map_id",
        vec![],
        mk_functor_forall1(|_depth| {
            let f = bvar(0);
            pi(BinderInfo::Default, "a", bvar(4), {
                let a = bvar(0);
                let fa = app2(cst("Functor.obj"), f.clone(), a.clone());
                mk_eq(
                    app2(cst("Category.Hom"), fa.clone(), fa.clone()),
                    app2(cst("Functor.map"), f, app(cst("Category.id"), a)),
                    app(cst("Category.id"), fa),
                )
            })
        }),
    )?;
    add_axiom(
        env,
        "Functor.map_comp",
        vec![],
        mk_functor_forall1(|_depth| {
            let funct = bvar(0);
            pi(
                BinderInfo::Implicit,
                "a",
                bvar(4),
                pi(
                    BinderInfo::Implicit,
                    "b",
                    bvar(5),
                    pi(
                        BinderInfo::Implicit,
                        "c",
                        bvar(6),
                        pi(
                            BinderInfo::Default,
                            "f",
                            app2(cst("Category.Hom"), bvar(2), bvar(1)),
                            pi(
                                BinderInfo::Default,
                                "g",
                                app2(cst("Category.Hom"), bvar(2), bvar(1)),
                                {
                                    let fa = app2(cst("Functor.obj"), funct.clone(), bvar(4));
                                    let fc = app2(cst("Functor.obj"), funct.clone(), bvar(2));
                                    mk_eq(
                                        app2(cst("Category.Hom"), fa, fc),
                                        app2(
                                            cst("Functor.map"),
                                            funct.clone(),
                                            app2(cst("Category.comp"), bvar(1), bvar(0)),
                                        ),
                                        app2(
                                            cst("Category.comp"),
                                            app2(cst("Functor.map"), funct.clone(), bvar(1)),
                                            app2(cst("Functor.map"), funct, bvar(0)),
                                        ),
                                    )
                                },
                            ),
                        ),
                    ),
                ),
            )
        }),
    )?;
    add_axiom(
        env,
        "Functor.id",
        vec![],
        mk_cat_law(|_depth| {
            let c = bvar(1);
            app2(cst("Functor"), c.clone(), c)
        }),
    )?;
    add_axiom(
        env,
        "Functor.comp",
        vec![],
        pi(
            BinderInfo::Implicit,
            "C",
            type0(),
            pi(
                BinderInfo::Implicit,
                "D",
                type0(),
                pi(
                    BinderInfo::Implicit,
                    "E",
                    type0(),
                    pi(
                        BinderInfo::InstImplicit,
                        "_instC",
                        app(cst("Category"), bvar(2)),
                        pi(
                            BinderInfo::InstImplicit,
                            "_instD",
                            app(cst("Category"), bvar(2)),
                            pi(
                                BinderInfo::InstImplicit,
                                "_instE",
                                app(cst("Category"), bvar(2)),
                                pi(
                                    BinderInfo::Default,
                                    "F",
                                    app2(cst("Functor"), bvar(5), bvar(4)),
                                    pi(
                                        BinderInfo::Default,
                                        "G",
                                        app2(cst("Functor"), bvar(5), bvar(4)),
                                        app2(cst("Functor"), bvar(7), bvar(4)),
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
        "NatTrans",
        vec![],
        mk_functor_forall2(|_depth| type1()),
    )?;
    add_axiom(
        env,
        "NatTrans.app",
        vec![],
        mk_functor_forall2(|_depth| {
            let f = bvar(1);
            let g = bvar(0);
            pi(
                BinderInfo::Default,
                "eta",
                app2(cst("NatTrans"), f.clone(), g.clone()),
                pi(
                    BinderInfo::Default,
                    "a",
                    bvar(6),
                    app2(
                        cst("Category.Hom"),
                        app2(cst("Functor.obj"), f, bvar(0)),
                        app2(cst("Functor.obj"), g, bvar(0)),
                    ),
                ),
            )
        }),
    )?;
    add_axiom(
        env,
        "NatTrans.naturality",
        vec![],
        mk_functor_forall2(|_depth| {
            let f_functor = bvar(1);
            let g_functor = bvar(0);
            pi(
                BinderInfo::Default,
                "eta",
                app2(cst("NatTrans"), f_functor, g_functor),
                pi(
                    BinderInfo::Default,
                    "a",
                    bvar(6),
                    pi(
                        BinderInfo::Default,
                        "b",
                        bvar(7),
                        pi(
                            BinderInfo::Default,
                            "f",
                            app2(cst("Category.Hom"), bvar(1), bvar(0)),
                            mk_eq(
                                app2(
                                    cst("Category.Hom"),
                                    app2(cst("Functor.obj"), bvar(5), bvar(2)),
                                    app2(cst("Functor.obj"), bvar(4), bvar(1)),
                                ),
                                app2(
                                    cst("Category.comp"),
                                    app2(cst("NatTrans.app"), bvar(3), bvar(2)),
                                    app2(cst("Functor.map"), bvar(4), bvar(0)),
                                ),
                                app2(
                                    cst("Category.comp"),
                                    app2(cst("Functor.map"), bvar(5), bvar(0)),
                                    app2(cst("NatTrans.app"), bvar(3), bvar(1)),
                                ),
                            ),
                        ),
                    ),
                ),
            )
        }),
    )?;
    add_axiom(
        env,
        "NatTrans.id",
        vec![],
        mk_functor_forall1(|_depth| {
            let f = bvar(0);
            app2(cst("NatTrans"), f.clone(), f)
        }),
    )?;
    add_axiom(
        env,
        "NatTrans.comp",
        vec![],
        pi(
            BinderInfo::Implicit,
            "C",
            type0(),
            pi(
                BinderInfo::Implicit,
                "D",
                type0(),
                pi(
                    BinderInfo::InstImplicit,
                    "_instC",
                    app(cst("Category"), bvar(1)),
                    pi(
                        BinderInfo::InstImplicit,
                        "_instD",
                        app(cst("Category"), bvar(1)),
                        pi(
                            BinderInfo::Implicit,
                            "F",
                            app2(cst("Functor"), bvar(3), bvar(2)),
                            pi(
                                BinderInfo::Implicit,
                                "G",
                                app2(cst("Functor"), bvar(4), bvar(3)),
                                pi(
                                    BinderInfo::Implicit,
                                    "H",
                                    app2(cst("Functor"), bvar(5), bvar(4)),
                                    pi(
                                        BinderInfo::Default,
                                        "alpha",
                                        app2(cst("NatTrans"), bvar(2), bvar(1)),
                                        pi(
                                            BinderInfo::Default,
                                            "beta",
                                            app2(cst("NatTrans"), bvar(2), bvar(1)),
                                            app2(cst("NatTrans"), bvar(4), bvar(2)),
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
    add_axiom(env, "NatIso", vec![], mk_functor_forall2(|_depth| type1()))?;
    add_axiom(
        env,
        "NatIso.toNatTrans",
        vec![],
        mk_functor_forall2(|_depth| {
            let f = bvar(1);
            let g = bvar(0);
            arrow(
                app2(cst("NatIso"), f.clone(), g.clone()),
                app2(cst("NatTrans"), f, g),
            )
        }),
    )?;
    add_axiom(
        env,
        "NatIso.symm",
        vec![],
        mk_functor_forall2(|_depth| {
            let f = bvar(1);
            let g = bvar(0);
            arrow(
                app2(cst("NatIso"), f.clone(), g.clone()),
                app2(cst("NatIso"), g, f),
            )
        }),
    )?;
    add_axiom(
        env,
        "IsInitial",
        vec![],
        mk_cat_law(|_depth| arrow(bvar(1), prop())),
    )?;
    add_axiom(
        env,
        "IsTerminal",
        vec![],
        mk_cat_law(|_depth| arrow(bvar(1), prop())),
    )?;
    add_axiom(
        env,
        "IsInitial.unique",
        vec![],
        mk_cat_forall1(|_depth| {
            let a = bvar(0);
            arrow(
                app(cst("IsInitial"), a),
                pi(BinderInfo::Default, "b", bvar(2), prop()),
            )
        }),
    )?;
    add_axiom(
        env,
        "IsTerminal.unique",
        vec![],
        mk_cat_forall1(|_depth| {
            let t = bvar(0);
            arrow(
                app(cst("IsTerminal"), t),
                pi(BinderInfo::Default, "a", bvar(2), prop()),
            )
        }),
    )?;
    add_axiom(
        env,
        "IsInitial.iso",
        vec![],
        mk_cat_forall2(|_depth| {
            let a = bvar(1);
            let b = bvar(0);
            arrow(
                app(cst("IsInitial"), a.clone()),
                arrow(app(cst("IsInitial"), b.clone()), app2(cst("Iso"), a, b)),
            )
        }),
    )?;
    add_axiom(
        env,
        "HasProduct",
        vec![],
        mk_cat_law(|_depth| arrow(bvar(1), arrow(bvar(2), prop()))),
    )?;
    add_axiom(
        env,
        "Product",
        vec![],
        mk_cat_law(|_depth| arrow(bvar(1), arrow(bvar(2), bvar(3)))),
    )?;
    add_axiom(
        env,
        "Product.fst",
        vec![],
        mk_cat_forall2(|_depth| {
            let a = bvar(1);
            let b = bvar(0);
            app2(cst("Category.Hom"), app2(cst("Product"), a.clone(), b), a)
        }),
    )?;
    add_axiom(
        env,
        "Product.snd",
        vec![],
        mk_cat_forall2(|_depth| {
            let a = bvar(1);
            let b = bvar(0);
            app2(cst("Category.Hom"), app2(cst("Product"), a, b.clone()), b)
        }),
    )?;
    add_axiom(
        env,
        "Product.lift",
        vec![],
        mk_cat_forall3(|_depth| {
            let a = bvar(2);
            let b = bvar(1);
            let x = bvar(0);
            arrow(
                app2(cst("Category.Hom"), x.clone(), a.clone()),
                arrow(
                    app2(cst("Category.Hom"), x.clone(), b.clone()),
                    app2(cst("Category.Hom"), x, app2(cst("Product"), a, b)),
                ),
            )
        }),
    )?;
    add_axiom(
        env,
        "HasCoproduct",
        vec![],
        mk_cat_law(|_depth| arrow(bvar(1), arrow(bvar(2), prop()))),
    )?;
    add_axiom(
        env,
        "Coproduct",
        vec![],
        mk_cat_law(|_depth| arrow(bvar(1), arrow(bvar(2), bvar(3)))),
    )?;
    add_axiom(
        env,
        "Coproduct.inl",
        vec![],
        mk_cat_forall2(|_depth| {
            let a = bvar(1);
            let b = bvar(0);
            app2(cst("Category.Hom"), a.clone(), app2(cst("Coproduct"), a, b))
        }),
    )?;
    add_axiom(
        env,
        "Coproduct.inr",
        vec![],
        mk_cat_forall2(|_depth| {
            let a = bvar(1);
            let b = bvar(0);
            app2(cst("Category.Hom"), b.clone(), app2(cst("Coproduct"), a, b))
        }),
    )?;
    add_axiom(
        env,
        "Coproduct.desc",
        vec![],
        mk_cat_forall3(|_depth| {
            let a = bvar(2);
            let b = bvar(1);
            let x = bvar(0);
            arrow(
                app2(cst("Category.Hom"), a.clone(), x.clone()),
                arrow(
                    app2(cst("Category.Hom"), b.clone(), x.clone()),
                    app2(cst("Category.Hom"), app2(cst("Coproduct"), a, b), x),
                ),
            )
        }),
    )?;
    add_axiom(
        env,
        "HasLimit",
        vec![],
        pi(
            BinderInfo::Implicit,
            "C",
            type0(),
            pi(
                BinderInfo::InstImplicit,
                "_instC",
                app(cst("Category"), bvar(0)),
                pi(
                    BinderInfo::Implicit,
                    "J",
                    type0(),
                    pi(
                        BinderInfo::InstImplicit,
                        "_instJ",
                        app(cst("Category"), bvar(0)),
                        pi(
                            BinderInfo::Default,
                            "F",
                            app2(cst("Functor"), bvar(1), bvar(3)),
                            prop(),
                        ),
                    ),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "Limit",
        vec![],
        pi(
            BinderInfo::Implicit,
            "C",
            type0(),
            pi(
                BinderInfo::InstImplicit,
                "_instC",
                app(cst("Category"), bvar(0)),
                pi(
                    BinderInfo::Implicit,
                    "J",
                    type0(),
                    pi(
                        BinderInfo::InstImplicit,
                        "_instJ",
                        app(cst("Category"), bvar(0)),
                        pi(
                            BinderInfo::Default,
                            "F",
                            app2(cst("Functor"), bvar(1), bvar(3)),
                            bvar(4),
                        ),
                    ),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "HasColimit",
        vec![],
        pi(
            BinderInfo::Implicit,
            "C",
            type0(),
            pi(
                BinderInfo::InstImplicit,
                "_instC",
                app(cst("Category"), bvar(0)),
                pi(
                    BinderInfo::Implicit,
                    "J",
                    type0(),
                    pi(
                        BinderInfo::InstImplicit,
                        "_instJ",
                        app(cst("Category"), bvar(0)),
                        pi(
                            BinderInfo::Default,
                            "F",
                            app2(cst("Functor"), bvar(1), bvar(3)),
                            prop(),
                        ),
                    ),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "Colimit",
        vec![],
        pi(
            BinderInfo::Implicit,
            "C",
            type0(),
            pi(
                BinderInfo::InstImplicit,
                "_instC",
                app(cst("Category"), bvar(0)),
                pi(
                    BinderInfo::Implicit,
                    "J",
                    type0(),
                    pi(
                        BinderInfo::InstImplicit,
                        "_instJ",
                        app(cst("Category"), bvar(0)),
                        pi(
                            BinderInfo::Default,
                            "F",
                            app2(cst("Functor"), bvar(1), bvar(3)),
                            bvar(4),
                        ),
                    ),
                ),
            ),
        ),
    )?;
    add_axiom(env, "HasAllLimits", vec![], mk_cat_law(|_depth| prop()))?;
    add_axiom(env, "HasAllColimits", vec![], mk_cat_law(|_depth| prop()))?;
    add_axiom(
        env,
        "Equalizer",
        vec![],
        mk_cat_forall2(|_depth| {
            let a = bvar(1);
            let b = bvar(0);
            arrow(
                app2(cst("Category.Hom"), a.clone(), b.clone()),
                arrow(app2(cst("Category.Hom"), a, b), bvar(3)),
            )
        }),
    )?;
    add_axiom(
        env,
        "Coequalizer",
        vec![],
        mk_cat_forall2(|_depth| {
            let a = bvar(1);
            let b = bvar(0);
            arrow(
                app2(cst("Category.Hom"), a.clone(), b.clone()),
                arrow(app2(cst("Category.Hom"), a, b), bvar(3)),
            )
        }),
    )?;
    add_axiom(env, "Equivalence", vec![], mk_two_cat_law(|_depth| type1()))?;
    add_axiom(
        env,
        "Equivalence.functor",
        vec![],
        mk_two_cat_law(|_depth| {
            let c = bvar(3);
            let d = bvar(2);
            arrow(
                app2(cst("Equivalence"), c.clone(), d.clone()),
                app2(cst("Functor"), c, d),
            )
        }),
    )?;
    add_axiom(
        env,
        "Equivalence.inverse",
        vec![],
        mk_two_cat_law(|_depth| {
            let c = bvar(3);
            let d = bvar(2);
            arrow(
                app2(cst("Equivalence"), c.clone(), d.clone()),
                app2(cst("Functor"), d, c),
            )
        }),
    )?;
    add_axiom(
        env,
        "Equivalence.refl",
        vec![],
        mk_cat_law(|_depth| {
            let c = bvar(1);
            app2(cst("Equivalence"), c.clone(), c)
        }),
    )?;
    add_axiom(
        env,
        "Equivalence.symm",
        vec![],
        mk_two_cat_law(|_depth| {
            let c = bvar(3);
            let d = bvar(2);
            arrow(
                app2(cst("Equivalence"), c.clone(), d.clone()),
                app2(cst("Equivalence"), d, c),
            )
        }),
    )?;
    add_axiom(
        env,
        "Op",
        vec![],
        pi(BinderInfo::Default, "C", type0(), type0()),
    )?;
    add_axiom(
        env,
        "Op.category",
        vec![],
        mk_cat_law(|_depth| app(cst("Category"), app(cst("Op"), bvar(1)))),
    )?;
    add_axiom(
        env,
        "Op.op",
        vec![],
        pi(
            BinderInfo::Implicit,
            "C",
            type0(),
            arrow(bvar(0), app(cst("Op"), bvar(0))),
        ),
    )?;
    add_axiom(
        env,
        "Op.unop",
        vec![],
        pi(
            BinderInfo::Implicit,
            "C",
            type0(),
            arrow(app(cst("Op"), bvar(0)), bvar(0)),
        ),
    )?;
    add_axiom(
        env,
        "Adjunction",
        vec![],
        mk_two_cat_law(|_depth| {
            let c = bvar(3);
            let d = bvar(2);
            arrow(
                app2(cst("Functor"), c.clone(), d.clone()),
                arrow(app2(cst("Functor"), d, c), type1()),
            )
        }),
    )?;
    add_axiom(
        env,
        "Adjunction.unit",
        vec![],
        mk_two_cat_law(|_depth| {
            let c = bvar(3);
            let d = bvar(2);
            pi(
                BinderInfo::Default,
                "F",
                app2(cst("Functor"), c.clone(), d.clone()),
                pi(
                    BinderInfo::Default,
                    "G",
                    app2(cst("Functor"), d, c),
                    arrow(app2(cst("Adjunction"), bvar(1), bvar(0)), prop()),
                ),
            )
        }),
    )?;
    add_axiom(
        env,
        "Adjunction.counit",
        vec![],
        mk_two_cat_law(|_depth| {
            let c = bvar(3);
            let d = bvar(2);
            pi(
                BinderInfo::Default,
                "F",
                app2(cst("Functor"), c.clone(), d.clone()),
                pi(
                    BinderInfo::Default,
                    "G",
                    app2(cst("Functor"), d, c),
                    arrow(app2(cst("Adjunction"), bvar(1), bvar(0)), prop()),
                ),
            )
        }),
    )?;
    add_axiom(env, "Yoneda", vec![], mk_cat_law(|_depth| type1()))?;
    add_axiom(env, "Yoneda.lemma", vec![], mk_cat_law(|_depth| prop()))?;
    add_axiom(
        env,
        "IsFullyFaithful",
        vec![],
        mk_two_cat_law(|_depth| {
            let c = bvar(3);
            let d = bvar(2);
            arrow(app2(cst("Functor"), c, d), prop())
        }),
    )?;
    add_axiom(
        env,
        "IsFull",
        vec![],
        mk_two_cat_law(|_depth| {
            let c = bvar(3);
            let d = bvar(2);
            arrow(app2(cst("Functor"), c, d), prop())
        }),
    )?;
    add_axiom(
        env,
        "IsFaithful",
        vec![],
        mk_two_cat_law(|_depth| {
            let c = bvar(3);
            let d = bvar(2);
            arrow(app2(cst("Functor"), c, d), prop())
        }),
    )?;
    add_axiom(
        env,
        "IsFullyFaithful.iff",
        vec![],
        mk_two_cat_law(|_depth| {
            let c = bvar(3);
            let d = bvar(2);
            pi(
                BinderInfo::Default,
                "F",
                app2(cst("Functor"), c, d),
                mk_iff(
                    app(cst("IsFullyFaithful"), bvar(0)),
                    mk_and(app(cst("IsFull"), bvar(0)), app(cst("IsFaithful"), bvar(0))),
                ),
            )
        }),
    )?;
    add_axiom(
        env,
        "IsFullyFaithful.reflects_iso",
        vec![],
        mk_two_cat_law(|_depth| {
            let c = bvar(3);
            let d = bvar(2);
            pi(
                BinderInfo::Default,
                "F",
                app2(cst("Functor"), c.clone(), d),
                arrow(
                    app(cst("IsFullyFaithful"), bvar(0)),
                    pi(
                        BinderInfo::Default,
                        "a",
                        c.clone(),
                        pi(
                            BinderInfo::Default,
                            "b",
                            c,
                            arrow(
                                app2(
                                    cst("Iso"),
                                    app2(cst("Functor.obj"), bvar(2), bvar(1)),
                                    app2(cst("Functor.obj"), bvar(2), bvar(0)),
                                ),
                                app2(cst("Iso"), bvar(1), bvar(0)),
                            ),
                        ),
                    ),
                ),
            )
        }),
    )?;
    add_axiom(env, "IsAbelian", vec![], mk_cat_law(|_depth| prop()))?;
    add_axiom(env, "IsPreadditive", vec![], mk_cat_law(|_depth| prop()))?;
    add_axiom(
        env,
        "IsAbelian.isPreadditive",
        vec![],
        mk_cat_law(|_depth| arrow(cst("IsAbelian"), cst("IsPreadditive"))),
    )?;
    add_axiom(env, "HasZeroObject", vec![], mk_cat_law(|_depth| prop()))?;
    add_axiom(
        env,
        "ZeroObject",
        vec![],
        pi(
            BinderInfo::Implicit,
            "C",
            type0(),
            pi(
                BinderInfo::InstImplicit,
                "_inst",
                app(cst("Category"), bvar(0)),
                pi(
                    BinderInfo::InstImplicit,
                    "_zero",
                    cst("HasZeroObject"),
                    bvar(2),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "ZeroObject.isInitial",
        vec![],
        mk_cat_law(|_depth| {
            arrow(
                cst("HasZeroObject"),
                app(cst("IsInitial"), cst("ZeroObject")),
            )
        }),
    )?;
    add_axiom(
        env,
        "ZeroObject.isTerminal",
        vec![],
        mk_cat_law(|_depth| {
            arrow(
                cst("HasZeroObject"),
                app(cst("IsTerminal"), cst("ZeroObject")),
            )
        }),
    )?;
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    use oxilean_kernel::Environment;
    #[test]
    fn test_build_category_theory_env() {
        let mut env = Environment::new();
        let _ = env.add(Declaration::Axiom {
            name: Name::str("Eq"),
            univ_params: vec![],
            ty: pi(
                BinderInfo::Implicit,
                "_",
                type0(),
                pi(
                    BinderInfo::Default,
                    "_",
                    bvar(0),
                    pi(BinderInfo::Default, "_", bvar(1), prop()),
                ),
            ),
        });
        let _ = env.add(Declaration::Axiom {
            name: Name::str("And"),
            univ_params: vec![],
            ty: arrow(prop(), arrow(prop(), prop())),
        });
        let _ = env.add(Declaration::Axiom {
            name: Name::str("Iff"),
            univ_params: vec![],
            ty: arrow(prop(), arrow(prop(), prop())),
        });
        let _ = env.add(Declaration::Axiom {
            name: Name::str("Exists"),
            univ_params: vec![],
            ty: pi(
                BinderInfo::Implicit,
                "a",
                type0(),
                arrow(arrow(bvar(0), prop()), prop()),
            ),
        });
        let result = build_category_theory_env(&mut env);
        assert!(
            result.is_ok(),
            "build_category_theory_env failed: {:?}",
            result
        );
    }
    #[test]
    fn test_category_expression_builders() {
        let a = cst("a");
        let b = cst("b");
        let f = cst("f");
        let g = cst("g");
        let hom = cat_hom(a.clone(), b.clone());
        assert!(matches!(hom, Expr::App(_, _)));
        let id = cat_id(a.clone());
        assert!(matches!(id, Expr::App(_, _)));
        let comp = cat_comp(f.clone(), g.clone());
        assert!(matches!(comp, Expr::App(_, _)));
        let obj = functor_obj(f.clone(), a.clone());
        assert!(matches!(obj, Expr::App(_, _)));
        let map = functor_map(f, g);
        assert!(matches!(map, Expr::App(_, _)));
        let iso = cat_iso(a.clone(), b);
        assert!(matches!(iso, Expr::App(_, _)));
        let init = is_initial(a.clone());
        assert!(matches!(init, Expr::App(_, _)));
        let term = is_terminal(a);
        assert!(matches!(term, Expr::App(_, _)));
    }
}
#[cfg(test)]
mod cat_ext_tests {
    use super::*;
    #[test]
    fn test_enriched_category() {
        let dg = EnrichedCategory::chain_complex_enriched();
        assert!(!dg.composition_map().is_empty());
    }
    #[test]
    fn test_kan_extension() {
        let lan = KanExtension::left("F", "K");
        assert!(lan.is_left);
        assert!(!lan.universal_property().is_empty());
        assert!(!lan.mac_lane_coend_formula().is_empty());
    }
    #[test]
    fn test_topos() {
        let set = Topos::set_topos();
        assert!(set.is_grothendieck);
        assert!(!set.subobject_classifier_description().is_empty());
        assert!(!set.internal_logic().is_empty());
    }
    #[test]
    fn test_model_category() {
        let top = ModelCategory::top_model_category();
        assert!(!top.homotopy_category_description().is_empty());
    }
    #[test]
    fn test_monad() {
        let m = Monad2Cat::new("Set", "List", false);
        assert!(!m.kleisli_category_description().is_empty());
        assert!(!m.eilenberg_moore_category_description().is_empty());
    }
}
#[cfg(test)]
mod infty_cat_tests {
    use super::*;
    #[test]
    fn test_infty_cat() {
        let qcat = InftyCat::new("Cat_infty", InftyCatModel::Quasicategory);
        assert!(qcat.limits_and_colimits_exist());
        assert!(!qcat.lurie_description().is_empty());
    }
}
/// Important functors.
#[allow(dead_code)]
pub fn important_functors() -> Vec<(&'static str, &'static str, &'static str)> {
    vec![
        ("Yoneda embedding", "C", "Set^(C^op)"),
        ("Nerve functor", "Cat", "sSet"),
        ("Free forgetful", "Monoids", "Set"),
        ("Postnikov truncation", "Spaces", "n-types"),
        ("Derived functor LF", "A (abelian)", "D(B)"),
        ("Algebraic K-theory", "Exact cats", "Spectra"),
        ("Hochschild homology", "Algebras", "Ch(k)"),
        ("Singular chains", "Top", "Ch(Z)"),
        ("Geometric realization", "sSet", "Top"),
        ("Stone duality", "Bool alg", "Stone spaces"),
    ]
}
#[cfg(test)]
mod doctrine_tests {
    use super::*;
    #[test]
    fn test_doctrine() {
        let d = Doctrine::first_order();
        assert!(!d.classifying_category().is_empty());
    }
    #[test]
    fn test_important_functors() {
        let fctrs = important_functors();
        assert!(!fctrs.is_empty());
    }
}
/// Brown representability theorem.
#[allow(dead_code)]
pub fn brown_representability_description() -> &'static str {
    "Every cohomology theory on CW complexes is representable by a spectrum (Brown, 1962)."
}
/// Seifert-van Kampen theorem (categorical).
#[allow(dead_code)]
pub fn seifert_van_kampen_theorem() -> &'static str {
    "pi_1 of a pushout of spaces = pushout of fundamental groups (under connectivity assumptions)."
}
#[cfg(test)]
mod homotopy_cat_tests {
    use super::*;
    #[test]
    fn test_brown_representability() {
        let desc = brown_representability_description();
        assert!(desc.contains("spectrum"));
    }
}
#[cfg(test)]
mod tests_cat_theory_ext {
    use super::*;
    #[test]
    fn test_enriched_category() {
        let ab = EnrichedCat::ab_cat();
        let comp = ab.composition_law();
        assert!(comp.contains("Abelian"));
        let yoneda = ab.yoneda_enriched();
        assert!(yoneda.contains("Yoneda"));
        let self_enr = EnrichedCat::cat_self_enriched();
        assert!(self_enr.is_self_enriched);
    }
    #[test]
    fn test_monoidal_category() {
        let vect = MonoidalCategory::vect_over_k();
        assert!(vect.is_symmetric && vect.is_closed);
        let mac = vect.mac_lane_coherence();
        assert!(mac.contains("Mac Lane"));
        let hom = vect.internal_hom();
        assert!(hom.contains("internal hom"));
        let not_closed = MonoidalCategory::new("C", "⊗", "I");
        let nh = not_closed.internal_hom();
        assert!(nh.contains("not closed"));
    }
    #[test]
    fn test_bicategory() {
        let bicat = Bicategory::bicat_rings();
        assert!(!bicat.is_strict);
        let coh = bicat.coherence_theorem();
        assert!(coh.contains("biequivalent"));
        let hor = bicat.horizontal_composition();
        assert!(hor.contains("bimodules"));
        let inter = bicat.interchange_law();
        assert!(inter.contains("interchange"));
        let two_cat = Bicategory::two_cat();
        assert!(two_cat.is_strict);
    }
    #[test]
    fn test_double_category() {
        let spans = DoubleCategory::spans();
        assert_eq!(spans.globular_cells_count(), 4);
        let shulman = spans.shulman_connection_to_fibrations();
        assert!(shulman.contains("Shulman"));
    }
    #[test]
    fn test_lcc_category() {
        let lcc = LocallyCartesianClosed::new("Fam(Set)").with_id_types();
        assert!(lcc.id_types);
        let seely = lcc.seely_equivalence();
        assert!(seely.contains("Seely"));
        let sigma = lcc.categorical_sigma_type();
        assert!(sigma.contains("Σ"));
        let pi = lcc.categorical_pi_type();
        assert!(pi.contains("Π"));
    }
    #[test]
    fn test_gluing_construction() {
        let gc = GluingConstruction::new("Syn", "Sem", "⟦-⟧");
        let nbe = gc.normalization_by_evaluation();
        assert!(nbe.contains("NbE"));
        let canon = gc.canonicity_via_gluing();
        assert!(canon.contains("Canonicity"));
        let sterling = gc.sterling_method_description();
        assert!(sterling.contains("Sterling"));
    }
}
