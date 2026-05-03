//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{
    CompactHausdorffData, CoveringSpaceData, QuotientSpaceData, SeparationAxioms, TychonoffData,
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
#[allow(dead_code)]
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
pub fn nat_ty() -> Expr {
    Expr::Const(Name::str("Nat"), vec![])
}
pub fn real_ty() -> Expr {
    Expr::Const(Name::str("Real"), vec![])
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
/// Build `Type -> Type 1` (single-parameter type class type).
#[allow(dead_code)]
pub fn mk_class_ty() -> Expr {
    pi(BinderInfo::Default, "\u{03b1}", type0(), type1())
}
/// Build `TopologicalSpace.isOpen {X} [inst] U`.
#[allow(dead_code)]
pub fn is_open(u: Expr) -> Expr {
    app(cst("TopologicalSpace.isOpen"), u)
}
/// Build `TopologicalSpace.isClosed {X} [inst] U`.
#[allow(dead_code)]
pub fn is_closed(u: Expr) -> Expr {
    app(cst("TopologicalSpace.isClosed"), u)
}
/// Build `Continuous {X Y} [instX] [instY] f`.
#[allow(dead_code)]
pub fn continuous(f: Expr) -> Expr {
    app(cst("Continuous"), f)
}
/// Build `Homeomorphism {X Y} [instX] [instY]`.
#[allow(dead_code)]
pub fn homeomorphism(x: Expr, y: Expr) -> Expr {
    app2(cst("Homeomorphism"), x, y)
}
/// Build `IsCompact {X} [inst] S`.
#[allow(dead_code)]
pub fn is_compact(s: Expr) -> Expr {
    app(cst("IsCompact"), s)
}
/// Build `IsConnected {X} [inst] S`.
#[allow(dead_code)]
pub fn is_connected(s: Expr) -> Expr {
    app(cst("IsConnected"), s)
}
/// Build `MetricSpace.dist {X} [inst] a b`.
#[allow(dead_code)]
pub fn metric_dist(a: Expr, b: Expr) -> Expr {
    app2(cst("MetricSpace.dist"), a, b)
}
/// Build `IsHausdorff {X} [inst]`.
#[allow(dead_code)]
pub fn is_hausdorff() -> Expr {
    cst("IsHausdorff")
}
/// Build `Closure {X} [inst] S`.
#[allow(dead_code)]
pub fn closure(s: Expr) -> Expr {
    app(cst("Closure"), s)
}
/// Build `Interior {X} [inst] S`.
#[allow(dead_code)]
pub fn interior(s: Expr) -> Expr {
    app(cst("Interior"), s)
}
/// Build `Boundary {X} [inst] S`.
#[allow(dead_code)]
pub fn boundary(s: Expr) -> Expr {
    app(cst("Boundary"), s)
}
/// `{X : Type} -> \[TopologicalSpace X\] -> <body>`
#[allow(dead_code)]
pub fn mk_topo_law<F>(prop_builder: F) -> Expr
where
    F: FnOnce(u32) -> Expr,
{
    pi(
        BinderInfo::Implicit,
        "X",
        type0(),
        pi(
            BinderInfo::InstImplicit,
            "_inst",
            app(cst("TopologicalSpace"), bvar(0)),
            prop_builder(2),
        ),
    )
}
/// `{X : Type} -> \[TopologicalSpace X\] -> forall (U : X -> Prop), <body>`
#[allow(dead_code)]
pub fn mk_topo_forall_set<F>(prop_builder: F) -> Expr
where
    F: FnOnce(u32) -> Expr,
{
    pi(
        BinderInfo::Implicit,
        "X",
        type0(),
        pi(
            BinderInfo::InstImplicit,
            "_inst",
            app(cst("TopologicalSpace"), bvar(0)),
            pi(
                BinderInfo::Default,
                "U",
                arrow(bvar(1), prop()),
                prop_builder(3),
            ),
        ),
    )
}
/// `{X : Type} -> \[TopologicalSpace X\] -> forall (U V : X -> Prop), <body>`
#[allow(dead_code)]
pub fn mk_topo_forall_set2<F>(prop_builder: F) -> Expr
where
    F: FnOnce(u32) -> Expr,
{
    pi(
        BinderInfo::Implicit,
        "X",
        type0(),
        pi(
            BinderInfo::InstImplicit,
            "_inst",
            app(cst("TopologicalSpace"), bvar(0)),
            pi(
                BinderInfo::Default,
                "U",
                arrow(bvar(1), prop()),
                pi(
                    BinderInfo::Default,
                    "V",
                    arrow(bvar(2), prop()),
                    prop_builder(4),
                ),
            ),
        ),
    )
}
/// `{X Y : Type} -> \[TopologicalSpace X\] -> \[TopologicalSpace Y\] -> <body>`
#[allow(dead_code)]
pub fn mk_two_topo_law<F>(prop_builder: F) -> Expr
where
    F: FnOnce(u32) -> Expr,
{
    pi(
        BinderInfo::Implicit,
        "X",
        type0(),
        pi(
            BinderInfo::Implicit,
            "Y",
            type0(),
            pi(
                BinderInfo::InstImplicit,
                "_instX",
                app(cst("TopologicalSpace"), bvar(1)),
                pi(
                    BinderInfo::InstImplicit,
                    "_instY",
                    app(cst("TopologicalSpace"), bvar(1)),
                    prop_builder(4),
                ),
            ),
        ),
    )
}
/// `{X Y : Type} -> \[TopologicalSpace X\] -> \[TopologicalSpace Y\] ->
///  forall (f : X -> Y), <body>`
#[allow(dead_code)]
pub fn mk_two_topo_forall_fun<F>(prop_builder: F) -> Expr
where
    F: FnOnce(u32) -> Expr,
{
    pi(
        BinderInfo::Implicit,
        "X",
        type0(),
        pi(
            BinderInfo::Implicit,
            "Y",
            type0(),
            pi(
                BinderInfo::InstImplicit,
                "_instX",
                app(cst("TopologicalSpace"), bvar(1)),
                pi(
                    BinderInfo::InstImplicit,
                    "_instY",
                    app(cst("TopologicalSpace"), bvar(1)),
                    pi(
                        BinderInfo::Default,
                        "f",
                        arrow(bvar(3), bvar(3)),
                        prop_builder(5),
                    ),
                ),
            ),
        ),
    )
}
/// `{X : Type} -> \[MetricSpace X\] -> <body>`
#[allow(dead_code)]
pub fn mk_metric_law<F>(prop_builder: F) -> Expr
where
    F: FnOnce(u32) -> Expr,
{
    pi(
        BinderInfo::Implicit,
        "X",
        type0(),
        pi(
            BinderInfo::InstImplicit,
            "_inst",
            app(cst("MetricSpace"), bvar(0)),
            prop_builder(2),
        ),
    )
}
/// `{X : Type} -> \[MetricSpace X\] -> forall (a : X), <body>`
#[allow(dead_code)]
pub fn mk_metric_forall1<F>(prop_builder: F) -> Expr
where
    F: FnOnce(u32) -> Expr,
{
    pi(
        BinderInfo::Implicit,
        "X",
        type0(),
        pi(
            BinderInfo::InstImplicit,
            "_inst",
            app(cst("MetricSpace"), bvar(0)),
            pi(BinderInfo::Default, "a", bvar(1), prop_builder(3)),
        ),
    )
}
/// `{X : Type} -> \[MetricSpace X\] -> forall (a b : X), <body>`
#[allow(dead_code)]
pub fn mk_metric_forall2<F>(prop_builder: F) -> Expr
where
    F: FnOnce(u32) -> Expr,
{
    pi(
        BinderInfo::Implicit,
        "X",
        type0(),
        pi(
            BinderInfo::InstImplicit,
            "_inst",
            app(cst("MetricSpace"), bvar(0)),
            pi(
                BinderInfo::Default,
                "a",
                bvar(1),
                pi(BinderInfo::Default, "b", bvar(2), prop_builder(4)),
            ),
        ),
    )
}
/// `{X : Type} -> \[MetricSpace X\] -> forall (a b c : X), <body>`
#[allow(dead_code)]
pub fn mk_metric_forall3<F>(prop_builder: F) -> Expr
where
    F: FnOnce(u32) -> Expr,
{
    pi(
        BinderInfo::Implicit,
        "X",
        type0(),
        pi(
            BinderInfo::InstImplicit,
            "_inst",
            app(cst("MetricSpace"), bvar(0)),
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
/// Build the topology environment with topological spaces, continuity,
/// compactness, connectedness, metric spaces, and separation axioms.
#[allow(clippy::too_many_lines)]
pub fn build_topology_env(env: &mut Environment) -> Result<(), String> {
    add_axiom(env, "TopologicalSpace", vec![], mk_class_ty())?;
    add_axiom(
        env,
        "TopologicalSpace.isOpen",
        vec![],
        mk_topo_law(|_depth| arrow(arrow(bvar(1), prop()), prop())),
    )?;
    add_axiom(
        env,
        "TopologicalSpace.isOpen_univ",
        vec![],
        mk_topo_law(|_depth| {
            app(
                cst("TopologicalSpace.isOpen"),
                lam(BinderInfo::Default, "_", bvar(1), cst("True")),
            )
        }),
    )?;
    add_axiom(
        env,
        "TopologicalSpace.isOpen_empty",
        vec![],
        mk_topo_law(|_depth| {
            app(
                cst("TopologicalSpace.isOpen"),
                lam(BinderInfo::Default, "_", bvar(1), cst("False")),
            )
        }),
    )?;
    add_axiom(
        env,
        "TopologicalSpace.isOpen_inter",
        vec![],
        mk_topo_forall_set2(|_depth| {
            let u = bvar(1);
            let v = bvar(0);
            arrow(
                app(cst("TopologicalSpace.isOpen"), u.clone()),
                arrow(
                    app(cst("TopologicalSpace.isOpen"), v.clone()),
                    app(
                        cst("TopologicalSpace.isOpen"),
                        lam(
                            BinderInfo::Default,
                            "x",
                            bvar(3),
                            mk_and(app(u, bvar(0)), app(v, bvar(0))),
                        ),
                    ),
                ),
            )
        }),
    )?;
    add_axiom(
        env,
        "TopologicalSpace.isOpen_union",
        vec![],
        mk_topo_forall_set2(|_depth| {
            let u = bvar(1);
            let v = bvar(0);
            arrow(
                app(cst("TopologicalSpace.isOpen"), u.clone()),
                arrow(
                    app(cst("TopologicalSpace.isOpen"), v.clone()),
                    app(
                        cst("TopologicalSpace.isOpen"),
                        lam(
                            BinderInfo::Default,
                            "x",
                            bvar(3),
                            mk_or(app(u, bvar(0)), app(v, bvar(0))),
                        ),
                    ),
                ),
            )
        }),
    )?;
    add_axiom(
        env,
        "TopologicalSpace.isClosed",
        vec![],
        mk_topo_law(|_depth| arrow(arrow(bvar(1), prop()), prop())),
    )?;
    add_axiom(
        env,
        "TopologicalSpace.isClosed_def",
        vec![],
        mk_topo_forall_set(|_depth| {
            let s = bvar(0);
            mk_iff(
                app(cst("TopologicalSpace.isClosed"), s.clone()),
                app(
                    cst("TopologicalSpace.isOpen"),
                    lam(BinderInfo::Default, "x", bvar(2), mk_not(app(s, bvar(0)))),
                ),
            )
        }),
    )?;
    add_axiom(
        env,
        "TopologicalSpace.isClosed_empty",
        vec![],
        mk_topo_law(|_depth| {
            app(
                cst("TopologicalSpace.isClosed"),
                lam(BinderInfo::Default, "_", bvar(1), cst("False")),
            )
        }),
    )?;
    add_axiom(
        env,
        "TopologicalSpace.isClosed_univ",
        vec![],
        mk_topo_law(|_depth| {
            app(
                cst("TopologicalSpace.isClosed"),
                lam(BinderInfo::Default, "_", bvar(1), cst("True")),
            )
        }),
    )?;
    add_axiom(
        env,
        "TopologicalSpace.isClosed_inter",
        vec![],
        mk_topo_forall_set2(|_depth| {
            let u = bvar(1);
            let v = bvar(0);
            arrow(
                app(cst("TopologicalSpace.isClosed"), u.clone()),
                arrow(
                    app(cst("TopologicalSpace.isClosed"), v.clone()),
                    app(
                        cst("TopologicalSpace.isClosed"),
                        lam(
                            BinderInfo::Default,
                            "x",
                            bvar(3),
                            mk_and(app(u, bvar(0)), app(v, bvar(0))),
                        ),
                    ),
                ),
            )
        }),
    )?;
    add_axiom(
        env,
        "TopologicalSpace.isClosed_union",
        vec![],
        mk_topo_forall_set2(|_depth| {
            let u = bvar(1);
            let v = bvar(0);
            arrow(
                app(cst("TopologicalSpace.isClosed"), u.clone()),
                arrow(
                    app(cst("TopologicalSpace.isClosed"), v.clone()),
                    app(
                        cst("TopologicalSpace.isClosed"),
                        lam(
                            BinderInfo::Default,
                            "x",
                            bvar(3),
                            mk_or(app(u, bvar(0)), app(v, bvar(0))),
                        ),
                    ),
                ),
            )
        }),
    )?;
    add_axiom(
        env,
        "Closure",
        vec![],
        mk_topo_law(|_depth| arrow(arrow(bvar(1), prop()), arrow(bvar(1), prop()))),
    )?;
    add_axiom(
        env,
        "Interior",
        vec![],
        mk_topo_law(|_depth| arrow(arrow(bvar(1), prop()), arrow(bvar(1), prop()))),
    )?;
    add_axiom(
        env,
        "Boundary",
        vec![],
        mk_topo_law(|_depth| arrow(arrow(bvar(1), prop()), arrow(bvar(1), prop()))),
    )?;
    add_axiom(
        env,
        "Closure.isClosed",
        vec![],
        mk_topo_forall_set(|_depth| {
            let s = bvar(0);
            app(cst("TopologicalSpace.isClosed"), app(cst("Closure"), s))
        }),
    )?;
    add_axiom(
        env,
        "Interior.isOpen",
        vec![],
        mk_topo_forall_set(|_depth| {
            let s = bvar(0);
            app(cst("TopologicalSpace.isOpen"), app(cst("Interior"), s))
        }),
    )?;
    add_axiom(
        env,
        "Closure.idempotent",
        vec![],
        mk_topo_forall_set(|_depth| {
            let s = bvar(0);
            let cls = app(cst("Closure"), s);
            mk_eq(
                arrow(bvar(2), prop()),
                app(cst("Closure"), cls.clone()),
                cls,
            )
        }),
    )?;
    add_axiom(
        env,
        "Interior.idempotent",
        vec![],
        mk_topo_forall_set(|_depth| {
            let s = bvar(0);
            let int = app(cst("Interior"), s);
            mk_eq(
                arrow(bvar(2), prop()),
                app(cst("Interior"), int.clone()),
                int,
            )
        }),
    )?;
    add_axiom(
        env,
        "TopologicalSpace.isClosed_iff_closure_eq",
        vec![],
        mk_topo_forall_set(|_depth| {
            let s = bvar(0);
            mk_iff(
                app(cst("TopologicalSpace.isClosed"), s.clone()),
                mk_eq(arrow(bvar(2), prop()), app(cst("Closure"), s.clone()), s),
            )
        }),
    )?;
    add_axiom(
        env,
        "TopologicalSpace.isOpen_iff_interior_eq",
        vec![],
        mk_topo_forall_set(|_depth| {
            let s = bvar(0);
            mk_iff(
                app(cst("TopologicalSpace.isOpen"), s.clone()),
                mk_eq(arrow(bvar(2), prop()), app(cst("Interior"), s.clone()), s),
            )
        }),
    )?;
    add_axiom(
        env,
        "Continuous",
        vec![],
        mk_two_topo_law(|_depth| arrow(arrow(bvar(3), bvar(3)), prop())),
    )?;
    add_axiom(
        env,
        "Continuous.id",
        vec![],
        mk_topo_law(|_depth| {
            app(
                cst("Continuous"),
                lam(BinderInfo::Default, "x", bvar(1), bvar(0)),
            )
        }),
    )?;
    add_axiom(
        env,
        "Continuous.const",
        vec![],
        mk_two_topo_law(|_depth| {
            pi(
                BinderInfo::Default,
                "c",
                bvar(2),
                app(
                    cst("Continuous"),
                    lam(BinderInfo::Default, "_", bvar(4), bvar(1)),
                ),
            )
        }),
    )?;
    add_axiom(
        env,
        "Continuous.comp",
        vec![],
        pi(
            BinderInfo::Implicit,
            "X",
            type0(),
            pi(
                BinderInfo::Implicit,
                "Y",
                type0(),
                pi(
                    BinderInfo::Implicit,
                    "Z",
                    type0(),
                    pi(
                        BinderInfo::InstImplicit,
                        "_instX",
                        app(cst("TopologicalSpace"), bvar(2)),
                        pi(
                            BinderInfo::InstImplicit,
                            "_instY",
                            app(cst("TopologicalSpace"), bvar(2)),
                            pi(
                                BinderInfo::InstImplicit,
                                "_instZ",
                                app(cst("TopologicalSpace"), bvar(2)),
                                pi(
                                    BinderInfo::Default,
                                    "f",
                                    arrow(bvar(5), bvar(5)),
                                    pi(
                                        BinderInfo::Default,
                                        "g",
                                        arrow(bvar(5), bvar(5)),
                                        arrow(
                                            app(cst("Continuous"), bvar(1)),
                                            arrow(app(cst("Continuous"), bvar(0)), prop()),
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
        "Continuous.def",
        vec![],
        mk_two_topo_forall_fun(|_depth| {
            let f = bvar(0);
            let rhs = pi(
                BinderInfo::Default,
                "U",
                arrow(bvar(3), prop()),
                arrow(
                    app(cst("TopologicalSpace.isOpen"), bvar(0)),
                    app(
                        cst("TopologicalSpace.isOpen"),
                        lam(
                            BinderInfo::Default,
                            "x",
                            bvar(5),
                            app(bvar(1), app(bvar(2), bvar(0))),
                        ),
                    ),
                ),
            );
            mk_iff(app(cst("Continuous"), f), rhs)
        }),
    )?;
    add_axiom(
        env,
        "Homeomorphism",
        vec![],
        mk_two_topo_law(|_depth| type1()),
    )?;
    add_axiom(
        env,
        "Homeomorphism.toFun",
        vec![],
        mk_two_topo_law(|_depth| {
            let x = bvar(3);
            let y = bvar(2);
            arrow(
                app2(cst("Homeomorphism"), x.clone(), y.clone()),
                arrow(x, y),
            )
        }),
    )?;
    add_axiom(
        env,
        "Homeomorphism.invFun",
        vec![],
        mk_two_topo_law(|_depth| {
            let x = bvar(3);
            let y = bvar(2);
            arrow(
                app2(cst("Homeomorphism"), x.clone(), y.clone()),
                arrow(y, x),
            )
        }),
    )?;
    add_axiom(
        env,
        "Homeomorphism.continuous",
        vec![],
        mk_two_topo_law(|_depth| {
            let x = bvar(3);
            let y = bvar(2);
            pi(
                BinderInfo::Default,
                "h",
                app2(cst("Homeomorphism"), x, y),
                app(cst("Continuous"), app(cst("Homeomorphism.toFun"), bvar(0))),
            )
        }),
    )?;
    add_axiom(
        env,
        "Homeomorphism.continuous_inv",
        vec![],
        mk_two_topo_law(|_depth| {
            let x = bvar(3);
            let y = bvar(2);
            pi(
                BinderInfo::Default,
                "h",
                app2(cst("Homeomorphism"), x, y),
                app(cst("Continuous"), app(cst("Homeomorphism.invFun"), bvar(0))),
            )
        }),
    )?;
    add_axiom(
        env,
        "Homeomorphism.refl",
        vec![],
        mk_topo_law(|_depth| {
            let x = bvar(1);
            app2(cst("Homeomorphism"), x.clone(), x)
        }),
    )?;
    add_axiom(
        env,
        "Homeomorphism.symm",
        vec![],
        mk_two_topo_law(|_depth| {
            let x = bvar(3);
            let y = bvar(2);
            arrow(
                app2(cst("Homeomorphism"), x.clone(), y.clone()),
                app2(cst("Homeomorphism"), y, x),
            )
        }),
    )?;
    add_axiom(
        env,
        "IsCompact",
        vec![],
        mk_topo_law(|_depth| arrow(arrow(bvar(1), prop()), prop())),
    )?;
    add_axiom(env, "IsCompactSpace", vec![], mk_topo_law(|_depth| prop()))?;
    add_axiom(
        env,
        "IsCompactSpace.def",
        vec![],
        mk_topo_law(|_depth| {
            mk_iff(
                cst("IsCompactSpace"),
                app(
                    cst("IsCompact"),
                    lam(BinderInfo::Default, "_", bvar(1), cst("True")),
                ),
            )
        }),
    )?;
    add_axiom(
        env,
        "IsCompact.of_closed_subset",
        vec![],
        mk_topo_forall_set2(|_depth| {
            let s = bvar(1);
            let t = bvar(0);
            let subset_ts = pi(
                BinderInfo::Default,
                "x",
                bvar(3),
                arrow(app(bvar(1), bvar(0)), app(bvar(2), bvar(0))),
            );
            arrow(
                app(cst("IsCompact"), s),
                arrow(
                    app(cst("TopologicalSpace.isClosed"), t.clone()),
                    arrow(subset_ts, app(cst("IsCompact"), t)),
                ),
            )
        }),
    )?;
    add_axiom(
        env,
        "IsCompact.image",
        vec![],
        mk_two_topo_forall_fun(|_depth| {
            let f = bvar(0);
            arrow(
                app(cst("Continuous"), f),
                pi(
                    BinderInfo::Default,
                    "S",
                    arrow(bvar(4), prop()),
                    arrow(
                        app(cst("IsCompact"), bvar(0)),
                        app(
                            cst("IsCompact"),
                            lam(
                                BinderInfo::Default,
                                "y",
                                bvar(4),
                                mk_exists(
                                    bvar(6),
                                    lam(
                                        BinderInfo::Default,
                                        "x",
                                        bvar(6),
                                        mk_and(
                                            app(bvar(2), bvar(0)),
                                            mk_eq(bvar(6), app(bvar(3), bvar(0)), bvar(1)),
                                        ),
                                    ),
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
        "IsCompact.inter",
        vec![],
        mk_topo_forall_set2(|_depth| {
            let s = bvar(1);
            let t = bvar(0);
            let inter_st = lam(
                BinderInfo::Default,
                "x",
                bvar(3),
                mk_and(app(bvar(2), bvar(0)), app(bvar(1), bvar(0))),
            );
            arrow(
                app(cst("IsCompact"), s),
                arrow(
                    app(cst("TopologicalSpace.isClosed"), t),
                    app(cst("IsCompact"), inter_st),
                ),
            )
        }),
    )?;
    add_axiom(
        env,
        "IsCompact.union",
        vec![],
        mk_topo_forall_set2(|_depth| {
            let union_st = lam(
                BinderInfo::Default,
                "x",
                bvar(3),
                mk_or(app(bvar(2), bvar(0)), app(bvar(1), bvar(0))),
            );
            arrow(
                app(cst("IsCompact"), bvar(1)),
                arrow(
                    app(cst("IsCompact"), bvar(0)),
                    app(cst("IsCompact"), union_st),
                ),
            )
        }),
    )?;
    add_axiom(
        env,
        "IsConnected",
        vec![],
        mk_topo_law(|_depth| arrow(arrow(bvar(1), prop()), prop())),
    )?;
    add_axiom(
        env,
        "IsConnectedSpace",
        vec![],
        mk_topo_law(|_depth| prop()),
    )?;
    add_axiom(
        env,
        "IsConnectedSpace.def",
        vec![],
        mk_topo_law(|_depth| {
            mk_iff(
                cst("IsConnectedSpace"),
                app(
                    cst("IsConnected"),
                    lam(BinderInfo::Default, "_", bvar(1), cst("True")),
                ),
            )
        }),
    )?;
    add_axiom(
        env,
        "IsConnected.image",
        vec![],
        mk_two_topo_forall_fun(|_depth| {
            let f = bvar(0);
            arrow(
                app(cst("Continuous"), f),
                pi(
                    BinderInfo::Default,
                    "S",
                    arrow(bvar(4), prop()),
                    arrow(
                        app(cst("IsConnected"), bvar(0)),
                        app(
                            cst("IsConnected"),
                            lam(
                                BinderInfo::Default,
                                "y",
                                bvar(4),
                                mk_exists(
                                    bvar(6),
                                    lam(
                                        BinderInfo::Default,
                                        "x",
                                        bvar(6),
                                        mk_and(
                                            app(bvar(2), bvar(0)),
                                            mk_eq(bvar(6), app(bvar(3), bvar(0)), bvar(1)),
                                        ),
                                    ),
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
        "IsPathConnected",
        vec![],
        mk_topo_law(|_depth| arrow(arrow(bvar(1), prop()), prop())),
    )?;
    add_axiom(
        env,
        "IsPathConnected.isConnected",
        vec![],
        mk_topo_forall_set(|_depth| {
            let s = bvar(0);
            arrow(
                app(cst("IsPathConnected"), s.clone()),
                app(cst("IsConnected"), s),
            )
        }),
    )?;
    add_axiom(env, "IsT0", vec![], mk_topo_law(|_depth| prop()))?;
    add_axiom(env, "IsT1", vec![], mk_topo_law(|_depth| prop()))?;
    add_axiom(env, "IsHausdorff", vec![], mk_topo_law(|_depth| prop()))?;
    add_axiom(
        env,
        "IsT1.isT0",
        vec![],
        mk_topo_law(|_depth| arrow(cst("IsT1"), cst("IsT0"))),
    )?;
    add_axiom(
        env,
        "IsHausdorff.isT1",
        vec![],
        mk_topo_law(|_depth| arrow(cst("IsHausdorff"), cst("IsT1"))),
    )?;
    add_axiom(
        env,
        "IsHausdorff.def",
        vec![],
        mk_topo_law(|_depth| {
            let hausdorff_rhs = pi(
                BinderInfo::Default,
                "x",
                bvar(1),
                pi(
                    BinderInfo::Default,
                    "y",
                    bvar(2),
                    arrow(
                        mk_not(app3(cst("Eq"), bvar(3), bvar(1), bvar(0))),
                        mk_exists(
                            arrow(bvar(3), prop()),
                            lam(
                                BinderInfo::Default,
                                "U",
                                arrow(bvar(3), prop()),
                                mk_exists(
                                    arrow(bvar(4), prop()),
                                    lam(
                                        BinderInfo::Default,
                                        "V",
                                        arrow(bvar(4), prop()),
                                        mk_and(
                                            app(cst("TopologicalSpace.isOpen"), bvar(1)),
                                            mk_and(
                                                app(cst("TopologicalSpace.isOpen"), bvar(0)),
                                                mk_and(
                                                    app(bvar(1), bvar(3)),
                                                    mk_and(
                                                        app(bvar(0), bvar(2)),
                                                        mk_eq(
                                                            arrow(bvar(5), prop()),
                                                            lam(
                                                                BinderInfo::Default,
                                                                "z",
                                                                bvar(5),
                                                                mk_and(
                                                                    app(bvar(2), bvar(0)),
                                                                    app(bvar(1), bvar(0)),
                                                                ),
                                                            ),
                                                            lam(
                                                                BinderInfo::Default,
                                                                "_",
                                                                bvar(5),
                                                                cst("False"),
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
            );
            mk_iff(cst("IsHausdorff"), hausdorff_rhs)
        }),
    )?;
    add_axiom(
        env,
        "IsCompactSpace.isNormal",
        vec![],
        mk_topo_law(|_depth| arrow(cst("IsCompactSpace"), arrow(cst("IsHausdorff"), prop()))),
    )?;
    add_axiom(
        env,
        "IsHausdorff.compact_is_closed",
        vec![],
        mk_topo_law(|_depth| {
            arrow(
                cst("IsHausdorff"),
                pi(
                    BinderInfo::Default,
                    "S",
                    arrow(bvar(1), prop()),
                    arrow(
                        app(cst("IsCompact"), bvar(0)),
                        app(cst("TopologicalSpace.isClosed"), bvar(0)),
                    ),
                ),
            )
        }),
    )?;
    add_axiom(env, "MetricSpace", vec![], mk_class_ty())?;
    add_axiom(
        env,
        "MetricSpace.dist",
        vec![],
        mk_metric_law(|_depth| arrow(bvar(1), arrow(bvar(2), real_ty()))),
    )?;
    add_axiom(
        env,
        "MetricSpace.toTopologicalSpace",
        vec![],
        mk_metric_law(|_depth| app(cst("TopologicalSpace"), bvar(1))),
    )?;
    add_axiom(
        env,
        "MetricSpace.dist_self",
        vec![],
        mk_metric_forall1(|_depth| {
            let a = bvar(0);
            mk_eq(
                real_ty(),
                app2(cst("MetricSpace.dist"), a.clone(), a),
                cst("Real.zero"),
            )
        }),
    )?;
    add_axiom(
        env,
        "MetricSpace.dist_comm",
        vec![],
        mk_metric_forall2(|_depth| {
            let a = bvar(1);
            let b = bvar(0);
            mk_eq(
                real_ty(),
                app2(cst("MetricSpace.dist"), a.clone(), b.clone()),
                app2(cst("MetricSpace.dist"), b, a),
            )
        }),
    )?;
    add_axiom(
        env,
        "MetricSpace.dist_triangle",
        vec![],
        mk_metric_forall3(|_depth| {
            let a = bvar(2);
            let b = bvar(1);
            let c = bvar(0);
            app2(
                cst("Real.le"),
                app2(cst("MetricSpace.dist"), a.clone(), c),
                app2(
                    cst("Real.add"),
                    app2(cst("MetricSpace.dist"), a, b.clone()),
                    app2(cst("MetricSpace.dist"), b, bvar(0)),
                ),
            )
        }),
    )?;
    add_axiom(
        env,
        "MetricSpace.dist_nonneg",
        vec![],
        mk_metric_forall2(|_depth| {
            let a = bvar(1);
            let b = bvar(0);
            app2(
                cst("Real.le"),
                cst("Real.zero"),
                app2(cst("MetricSpace.dist"), a, b),
            )
        }),
    )?;
    add_axiom(
        env,
        "MetricSpace.eq_of_dist_eq_zero",
        vec![],
        mk_metric_forall2(|depth| {
            let x = bvar(depth - 1);
            let a = bvar(1);
            let b = bvar(0);
            arrow(
                mk_eq(
                    real_ty(),
                    app2(cst("MetricSpace.dist"), a.clone(), b.clone()),
                    cst("Real.zero"),
                ),
                mk_eq(x, a, b),
            )
        }),
    )?;
    add_axiom(
        env,
        "Ball",
        vec![],
        mk_metric_law(|_depth| arrow(bvar(1), arrow(real_ty(), arrow(bvar(1), prop())))),
    )?;
    add_axiom(
        env,
        "Ball.isOpen",
        vec![],
        mk_metric_forall1(|_depth| {
            let x = bvar(0);
            pi(
                BinderInfo::Default,
                "r",
                real_ty(),
                app(
                    cst("TopologicalSpace.isOpen"),
                    app2(cst("Ball"), x, bvar(0)),
                ),
            )
        }),
    )?;
    add_axiom(
        env,
        "Ball.center_mem",
        vec![],
        mk_metric_forall1(|_depth| {
            let x = bvar(0);
            pi(
                BinderInfo::Default,
                "r",
                real_ty(),
                arrow(
                    app2(cst("Real.lt"), cst("Real.zero"), bvar(0)),
                    app3(cst("Ball"), x, bvar(0), bvar(1)),
                ),
            )
        }),
    )?;
    add_axiom(
        env,
        "MetricSpace.convergesTo",
        vec![],
        mk_metric_law(|_depth| arrow(arrow(nat_ty(), bvar(1)), arrow(bvar(1), prop()))),
    )?;
    add_axiom(
        env,
        "MetricSpace.isCauchy",
        vec![],
        mk_metric_law(|_depth| arrow(arrow(nat_ty(), bvar(1)), prop())),
    )?;
    add_axiom(env, "IsComplete", vec![], mk_metric_law(|_depth| prop()))?;
    add_axiom(
        env,
        "IsComplete.def",
        vec![],
        mk_metric_law(|_depth| {
            mk_iff(
                cst("IsComplete"),
                pi(
                    BinderInfo::Default,
                    "s",
                    arrow(nat_ty(), bvar(1)),
                    arrow(
                        app(cst("MetricSpace.isCauchy"), bvar(0)),
                        mk_exists(
                            bvar(2),
                            lam(
                                BinderInfo::Default,
                                "x",
                                bvar(2),
                                app2(cst("MetricSpace.convergesTo"), bvar(1), bvar(0)),
                            ),
                        ),
                    ),
                ),
            )
        }),
    )?;
    add_axiom(
        env,
        "IsTopologicalBasis",
        vec![],
        mk_topo_law(|_depth| arrow(arrow(arrow(bvar(1), prop()), prop()), prop())),
    )?;
    add_axiom(
        env,
        "IsTopologicalBasis.isOpen",
        vec![],
        mk_topo_law(|_depth| {
            pi(
                BinderInfo::Default,
                "B",
                arrow(arrow(bvar(1), prop()), prop()),
                arrow(
                    app(cst("IsTopologicalBasis"), bvar(0)),
                    pi(
                        BinderInfo::Default,
                        "U",
                        arrow(bvar(2), prop()),
                        arrow(
                            app(bvar(1), bvar(0)),
                            app(cst("TopologicalSpace.isOpen"), bvar(0)),
                        ),
                    ),
                ),
            )
        }),
    )?;
    add_axiom(
        env,
        "ProductTopology",
        vec![],
        mk_two_topo_law(|_depth| {
            let x = bvar(3);
            let y = bvar(2);
            app(cst("TopologicalSpace"), app2(cst("Prod"), x, y))
        }),
    )?;
    add_axiom(
        env,
        "Prod",
        vec![],
        pi(
            BinderInfo::Default,
            "X",
            type0(),
            pi(BinderInfo::Default, "Y", type0(), type0()),
        ),
    )?;
    add_axiom(
        env,
        "Prod.fst_continuous",
        vec![],
        mk_two_topo_law(|_depth| app(cst("Continuous"), cst("Prod.fst"))),
    )?;
    add_axiom(
        env,
        "Prod.snd_continuous",
        vec![],
        mk_two_topo_law(|_depth| app(cst("Continuous"), cst("Prod.snd"))),
    )?;
    add_axiom(
        env,
        "SubspaceTopology",
        vec![],
        mk_topo_law(|_depth| arrow(arrow(bvar(1), prop()), type1())),
    )?;
    add_axiom(
        env,
        "SubspaceTopology.inclusion",
        vec![],
        mk_topo_forall_set(|_depth| {
            let s = bvar(0);
            arrow(app(cst("SubspaceTopology"), s), bvar(2))
        }),
    )?;
    add_axiom(
        env,
        "SubspaceTopology.inclusion_continuous",
        vec![],
        mk_topo_forall_set(|_depth| {
            app(
                cst("Continuous"),
                app(cst("SubspaceTopology.inclusion"), bvar(0)),
            )
        }),
    )?;
    add_axiom(
        env,
        "IsSequentiallyCompact",
        vec![],
        mk_metric_law(|_depth| arrow(arrow(bvar(1), prop()), prop())),
    )?;
    add_axiom(
        env,
        "IsTotallyBounded",
        vec![],
        mk_metric_law(|_depth| arrow(arrow(bvar(1), prop()), prop())),
    )?;
    add_axiom(
        env,
        "IsBounded",
        vec![],
        mk_metric_law(|_depth| arrow(arrow(bvar(1), prop()), prop())),
    )?;
    add_axiom(
        env,
        "MetricSpace.compact_iff_seq_compact",
        vec![],
        mk_metric_law(|_depth| {
            pi(
                BinderInfo::Default,
                "S",
                arrow(bvar(1), prop()),
                mk_iff(
                    app(cst("IsCompact"), bvar(0)),
                    app(cst("IsSequentiallyCompact"), bvar(0)),
                ),
            )
        }),
    )?;
    add_axiom(
        env,
        "MetricSpace.compact_iff_complete_totally_bounded",
        vec![],
        mk_metric_law(|_depth| {
            pi(
                BinderInfo::Default,
                "S",
                arrow(bvar(1), prop()),
                mk_iff(
                    app(cst("IsCompact"), bvar(0)),
                    mk_and(cst("IsComplete"), app(cst("IsTotallyBounded"), bvar(0))),
                ),
            )
        }),
    )?;
    add_axiom(
        env,
        "Real.heine_borel",
        vec![],
        pi(
            BinderInfo::Default,
            "S",
            arrow(real_ty(), prop()),
            mk_iff(
                app(cst("IsCompact"), bvar(0)),
                mk_and(
                    app(cst("TopologicalSpace.isClosed"), bvar(0)),
                    app(cst("IsBounded"), bvar(0)),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "Neighborhood",
        vec![],
        mk_topo_law(|_depth| arrow(bvar(1), arrow(arrow(bvar(1), prop()), prop()))),
    )?;
    add_axiom(
        env,
        "IsLimitPoint",
        vec![],
        mk_topo_law(|_depth| arrow(arrow(bvar(1), prop()), arrow(bvar(1), prop()))),
    )?;
    add_axiom(
        env,
        "IsDenseIn",
        vec![],
        mk_topo_law(|_depth| arrow(arrow(bvar(1), prop()), prop())),
    )?;
    add_axiom(
        env,
        "IsDenseIn.iff_closure_eq_univ",
        vec![],
        mk_topo_forall_set(|_depth| {
            let s = bvar(0);
            let univ = lam(BinderInfo::Default, "_", bvar(2), cst("True"));
            let set_ty = arrow(bvar(2), prop());
            mk_iff(
                app(cst("IsDenseIn"), s.clone()),
                mk_eq(set_ty, app(cst("Closure"), s), univ),
            )
        }),
    )?;
    add_axiom(
        env,
        "Tychonoff",
        vec![],
        pi(
            BinderInfo::Implicit,
            "ι",
            type0(),
            pi(
                BinderInfo::Implicit,
                "X",
                arrow(bvar(0), type0()),
                arrow(
                    pi(
                        BinderInfo::Default,
                        "i",
                        bvar(1),
                        app(cst("IsCompactSpace"), app(bvar(1), bvar(0))),
                    ),
                    cst("IsCompactSpace"),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "QuotientTopology",
        vec![],
        mk_topo_law(|_depth| arrow(arrow(bvar(1), arrow(bvar(2), prop())), type1())),
    )?;
    add_axiom(
        env,
        "QuotientTopology.mk",
        vec![],
        mk_topo_law(|_depth| {
            pi(
                BinderInfo::Default,
                "R",
                arrow(bvar(1), arrow(bvar(2), prop())),
                arrow(bvar(2), app(cst("QuotientTopology"), bvar(1))),
            )
        }),
    )?;
    add_axiom(
        env,
        "QuotientTopology.map_continuous",
        vec![],
        mk_topo_law(|_depth| {
            pi(
                BinderInfo::Default,
                "R",
                arrow(bvar(1), arrow(bvar(2), prop())),
                app(cst("Continuous"), app(cst("QuotientTopology.mk"), bvar(0))),
            )
        }),
    )?;
    add_axiom(
        env,
        "IsLocallyCompact",
        vec![],
        mk_topo_law(|_depth| prop()),
    )?;
    add_axiom(
        env,
        "IsCompactSpace.isLocallyCompact",
        vec![],
        mk_topo_law(|_depth| arrow(cst("IsCompactSpace"), cst("IsLocallyCompact"))),
    )?;
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    use oxilean_kernel::Environment;
    #[test]
    fn test_build_topology_env() {
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
            name: Name::str("Or"),
            univ_params: vec![],
            ty: arrow(prop(), arrow(prop(), prop())),
        });
        let _ = env.add(Declaration::Axiom {
            name: Name::str("Iff"),
            univ_params: vec![],
            ty: arrow(prop(), arrow(prop(), prop())),
        });
        let _ = env.add(Declaration::Axiom {
            name: Name::str("Not"),
            univ_params: vec![],
            ty: arrow(prop(), prop()),
        });
        let _ = env.add(Declaration::Axiom {
            name: Name::str("True"),
            univ_params: vec![],
            ty: prop(),
        });
        let _ = env.add(Declaration::Axiom {
            name: Name::str("False"),
            univ_params: vec![],
            ty: prop(),
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
        let _ = env.add(Declaration::Axiom {
            name: Name::str("Nat"),
            univ_params: vec![],
            ty: type0(),
        });
        let _ = env.add(Declaration::Axiom {
            name: Name::str("Real"),
            univ_params: vec![],
            ty: type0(),
        });
        let _ = env.add(Declaration::Axiom {
            name: Name::str("Real.zero"),
            univ_params: vec![],
            ty: real_ty(),
        });
        let _ = env.add(Declaration::Axiom {
            name: Name::str("Real.le"),
            univ_params: vec![],
            ty: arrow(real_ty(), arrow(real_ty(), prop())),
        });
        let _ = env.add(Declaration::Axiom {
            name: Name::str("Real.lt"),
            univ_params: vec![],
            ty: arrow(real_ty(), arrow(real_ty(), prop())),
        });
        let _ = env.add(Declaration::Axiom {
            name: Name::str("Real.add"),
            univ_params: vec![],
            ty: arrow(real_ty(), arrow(real_ty(), real_ty())),
        });
        let result = build_topology_env(&mut env);
        assert!(result.is_ok(), "build_topology_env failed: {:?}", result);
    }
    #[test]
    fn test_topology_expression_builders() {
        let s = cst("S");
        let f = cst("f");
        let a = cst("a");
        let b = cst("b");
        let open = is_open(s.clone());
        assert!(matches!(open, Expr::App(_, _)));
        let closed = is_closed(s.clone());
        assert!(matches!(closed, Expr::App(_, _)));
        let cont = continuous(f);
        assert!(matches!(cont, Expr::App(_, _)));
        let homeo = homeomorphism(cst("X"), cst("Y"));
        assert!(matches!(homeo, Expr::App(_, _)));
        let compact = is_compact(s.clone());
        assert!(matches!(compact, Expr::App(_, _)));
        let connected = is_connected(s);
        assert!(matches!(connected, Expr::App(_, _)));
        let dist = metric_dist(a, b);
        assert!(matches!(dist, Expr::App(_, _)));
        let cls = closure(cst("S2"));
        assert!(matches!(cls, Expr::App(_, _)));
        let int = interior(cst("S3"));
        assert!(matches!(int, Expr::App(_, _)));
        let bdy = boundary(cst("S4"));
        assert!(matches!(bdy, Expr::App(_, _)));
    }
}
#[cfg(test)]
mod tests_topology_ext_new {
    use super::*;
    #[test]
    fn test_compact_hausdorff() {
        let s2 = CompactHausdorffData::new("S^2")
            .metrizable()
            .connected()
            .with_dimension(2);
        assert!(s2.stone_weierstrass_applies());
        assert!(s2.urysohn_metrization());
        assert!(s2.tietze_extension().contains("Tietze"));
        assert_eq!(s2.dimension, Some(2));
    }
    #[test]
    fn test_quotient_space() {
        let qs = QuotientSpaceData::new("D^2", "x ~ -x on ∂D^2", "RP^2").with_open_map();
        assert!(qs.is_hausdorff_when_compact_and_closed());
        assert!(qs.characteristic_property().contains("continuous"));
    }
    #[test]
    fn test_covering_space() {
        let r_to_s1 = CoveringSpaceData::new("R", "S^1", None)
            .with_deck_group("Z")
            .universal();
        assert!(r_to_s1.is_universal);
        assert!(r_to_s1.monodromy_description().contains("π_1"));
        assert!(r_to_s1.lifting_theorem().contains("subgroup"));
    }
}
#[cfg(test)]
mod tests_topology_ext_new2 {
    use super::*;
    #[test]
    fn test_separation_axioms() {
        let nh = SeparationAxioms::normal_hausdorff();
        assert!(nh.t0 && nh.t1 && nh.t2 && nh.t4);
        assert!(nh.is_tychonoff());
        assert!(nh.urysohn_lemma_applies());
        assert!(nh.tietze_applies());
        let h = SeparationAxioms::hausdorff();
        assert!(!h.t4);
        assert!(!h.urysohn_lemma_applies());
    }
}
#[cfg(test)]
mod tests_topology_ext_new3 {
    use super::*;
    #[test]
    fn test_tychonoff_data() {
        let td = TychonoffData::new(vec!["[0,1]".to_string(), "[0,1]".to_string()]);
        assert!(td.product_compact);
        assert!(td.tychonoff_theorem().contains("Tychonoff"));
        assert!(td.cech_stone_connection().contains("compactification"));
    }
}
