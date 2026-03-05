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
/// Build `Type -> Type 1` (the type of a type class taking a Type).
#[allow(dead_code)]
pub fn mk_class_ty() -> Expr {
    pi(BinderInfo::Default, "\u{03b1}", type0(), type1())
}
/// Build `Eq @{} alpha a b`.
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
/// Build `Not p`.
#[allow(dead_code)]
pub fn mk_not(p: Expr) -> Expr {
    app(cst("Not"), p)
}
/// Build `Or p q`.
#[allow(dead_code)]
pub fn mk_or(p: Expr, q: Expr) -> Expr {
    app2(cst("Or"), p, q)
}
/// Build `Group.mul {G} [inst] a b`.
#[allow(dead_code)]
pub fn group_mul(a: Expr, b: Expr) -> Expr {
    app2(cst("Group.mul"), a, b)
}
/// Build `Group.inv {G} [inst] a`.
#[allow(dead_code)]
pub fn group_inv(a: Expr) -> Expr {
    app(cst("Group.inv"), a)
}
/// Build `Group.one {G} [inst]`.
#[allow(dead_code)]
pub fn group_one() -> Expr {
    cst("Group.one")
}
/// Build `Subgroup.mem {G} [inst] s a`.
#[allow(dead_code)]
pub fn subgroup_mem(s: Expr, a: Expr) -> Expr {
    app2(cst("Subgroup.mem"), s, a)
}
/// Build `GroupHom.toFun {G H} [instG] [instH] f a`.
#[allow(dead_code)]
pub fn group_hom_apply(f: Expr, a: Expr) -> Expr {
    app2(cst("GroupHom.toFun"), f, a)
}
/// Build `IsNormal {G} [inst] N`.
#[allow(dead_code)]
pub fn is_normal(n: Expr) -> Expr {
    app(cst("IsNormal"), n)
}
/// Build `LeftCoset {G} [inst] a N`.
#[allow(dead_code)]
pub fn left_coset(a: Expr, n: Expr) -> Expr {
    app2(cst("LeftCoset"), a, n)
}
/// Build `RightCoset {G} [inst] N a`.
#[allow(dead_code)]
pub fn right_coset(n: Expr, a: Expr) -> Expr {
    app2(cst("RightCoset"), n, a)
}
/// Build `GroupOrder {G} [inst] a`.
#[allow(dead_code)]
pub fn group_order(a: Expr) -> Expr {
    app(cst("GroupOrder"), a)
}
/// Build `IsCyclic {G} [inst]`.
#[allow(dead_code)]
pub fn is_cyclic() -> Expr {
    cst("IsCyclic")
}
/// Build `Center {G} [inst]`.
#[allow(dead_code)]
pub fn center() -> Expr {
    cst("Center")
}
/// Build `GroupAction.smul {G X} [instG] [instAct] g x`.
#[allow(dead_code)]
pub fn group_action_smul(g: Expr, x: Expr) -> Expr {
    app2(cst("GroupAction.smul"), g, x)
}
/// Build `Conjugate {G} [inst] g h`.
#[allow(dead_code)]
pub fn conjugate(g: Expr, h: Expr) -> Expr {
    app2(cst("Conjugate"), g, h)
}
/// `{G : Type} -> [Group G] -> <body>` where body builder gets de Bruijn depth.
#[allow(dead_code)]
pub fn mk_group_law<F>(prop_builder: F) -> Expr
where
    F: FnOnce(u32) -> Expr,
{
    pi(
        BinderInfo::Implicit,
        "G",
        type0(),
        pi(
            BinderInfo::InstImplicit,
            "_inst",
            app(cst("Group"), bvar(0)),
            prop_builder(2),
        ),
    )
}
/// `{G : Type} -> [Group G] -> forall (a : G), <body>`
#[allow(dead_code)]
pub fn mk_group_forall1<F>(prop_builder: F) -> Expr
where
    F: FnOnce(u32) -> Expr,
{
    pi(
        BinderInfo::Implicit,
        "G",
        type0(),
        pi(
            BinderInfo::InstImplicit,
            "_inst",
            app(cst("Group"), bvar(0)),
            pi(BinderInfo::Default, "a", bvar(1), prop_builder(3)),
        ),
    )
}
/// `{G : Type} -> [Group G] -> forall (a b : G), <body>`
#[allow(dead_code)]
pub fn mk_group_forall2<F>(prop_builder: F) -> Expr
where
    F: FnOnce(u32) -> Expr,
{
    pi(
        BinderInfo::Implicit,
        "G",
        type0(),
        pi(
            BinderInfo::InstImplicit,
            "_inst",
            app(cst("Group"), bvar(0)),
            pi(
                BinderInfo::Default,
                "a",
                bvar(1),
                pi(BinderInfo::Default, "b", bvar(2), prop_builder(4)),
            ),
        ),
    )
}
/// `{G : Type} -> [Group G] -> forall (a b c : G), <body>`
#[allow(dead_code)]
pub fn mk_group_forall3<F>(prop_builder: F) -> Expr
where
    F: FnOnce(u32) -> Expr,
{
    pi(
        BinderInfo::Implicit,
        "G",
        type0(),
        pi(
            BinderInfo::InstImplicit,
            "_inst",
            app(cst("Group"), bvar(0)),
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
/// `{G H : Type} -> [Group G] -> [Group H] -> <body>`
#[allow(dead_code)]
pub fn mk_two_group_law<F>(prop_builder: F) -> Expr
where
    F: FnOnce(u32) -> Expr,
{
    pi(
        BinderInfo::Implicit,
        "G",
        type0(),
        pi(
            BinderInfo::Implicit,
            "H",
            type0(),
            pi(
                BinderInfo::InstImplicit,
                "_instG",
                app(cst("Group"), bvar(1)),
                pi(
                    BinderInfo::InstImplicit,
                    "_instH",
                    app(cst("Group"), bvar(1)),
                    prop_builder(4),
                ),
            ),
        ),
    )
}
/// `{G H : Type} -> [Group G] -> [Group H] -> forall (f : GroupHom G H), <body>`
#[allow(dead_code)]
pub fn mk_hom_forall1<F>(prop_builder: F) -> Expr
where
    F: FnOnce(u32) -> Expr,
{
    pi(
        BinderInfo::Implicit,
        "G",
        type0(),
        pi(
            BinderInfo::Implicit,
            "H",
            type0(),
            pi(
                BinderInfo::InstImplicit,
                "_instG",
                app(cst("Group"), bvar(1)),
                pi(
                    BinderInfo::InstImplicit,
                    "_instH",
                    app(cst("Group"), bvar(1)),
                    pi(
                        BinderInfo::Default,
                        "f",
                        app2(cst("GroupHom"), bvar(3), bvar(2)),
                        prop_builder(5),
                    ),
                ),
            ),
        ),
    )
}
/// `{G H : Type} -> [Group G] -> [Group H] -> forall (f : GroupHom G H) (a : G), <body>`
#[allow(dead_code)]
pub fn mk_hom_forall2<F>(prop_builder: F) -> Expr
where
    F: FnOnce(u32) -> Expr,
{
    pi(
        BinderInfo::Implicit,
        "G",
        type0(),
        pi(
            BinderInfo::Implicit,
            "H",
            type0(),
            pi(
                BinderInfo::InstImplicit,
                "_instG",
                app(cst("Group"), bvar(1)),
                pi(
                    BinderInfo::InstImplicit,
                    "_instH",
                    app(cst("Group"), bvar(1)),
                    pi(
                        BinderInfo::Default,
                        "f",
                        app2(cst("GroupHom"), bvar(3), bvar(2)),
                        pi(BinderInfo::Default, "a", bvar(4), prop_builder(6)),
                    ),
                ),
            ),
        ),
    )
}
/// `{G H : Type} -> [Group G] -> [Group H] -> forall (f : GroupHom G H) (a b : G), <body>`
#[allow(dead_code)]
#[allow(clippy::too_many_arguments)]
pub fn mk_hom_forall3<F>(prop_builder: F) -> Expr
where
    F: FnOnce(u32) -> Expr,
{
    pi(
        BinderInfo::Implicit,
        "G",
        type0(),
        pi(
            BinderInfo::Implicit,
            "H",
            type0(),
            pi(
                BinderInfo::InstImplicit,
                "_instG",
                app(cst("Group"), bvar(1)),
                pi(
                    BinderInfo::InstImplicit,
                    "_instH",
                    app(cst("Group"), bvar(1)),
                    pi(
                        BinderInfo::Default,
                        "f",
                        app2(cst("GroupHom"), bvar(3), bvar(2)),
                        pi(
                            BinderInfo::Default,
                            "a",
                            bvar(4),
                            pi(BinderInfo::Default, "b", bvar(5), prop_builder(7)),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// Build `{G : Type} -> [Group G] -> (S : Subgroup G) -> <body>`.
#[allow(dead_code)]
pub fn mk_subgroup_law<F>(prop_builder: F) -> Expr
where
    F: FnOnce(u32) -> Expr,
{
    pi(
        BinderInfo::Implicit,
        "G",
        type0(),
        pi(
            BinderInfo::InstImplicit,
            "_inst",
            app(cst("Group"), bvar(0)),
            pi(
                BinderInfo::Default,
                "S",
                app(cst("Subgroup"), bvar(1)),
                prop_builder(3),
            ),
        ),
    )
}
/// Build `{G : Type} -> [Group G] -> (S : Subgroup G) -> forall (a : G), <body>`.
#[allow(dead_code)]
pub fn mk_subgroup_forall1<F>(prop_builder: F) -> Expr
where
    F: FnOnce(u32) -> Expr,
{
    pi(
        BinderInfo::Implicit,
        "G",
        type0(),
        pi(
            BinderInfo::InstImplicit,
            "_inst",
            app(cst("Group"), bvar(0)),
            pi(
                BinderInfo::Default,
                "S",
                app(cst("Subgroup"), bvar(1)),
                pi(BinderInfo::Default, "a", bvar(2), prop_builder(4)),
            ),
        ),
    )
}
/// Build `{G : Type} -> [Group G] -> (S : Subgroup G) -> forall (a b : G), <body>`.
#[allow(dead_code)]
pub fn mk_subgroup_forall2<F>(prop_builder: F) -> Expr
where
    F: FnOnce(u32) -> Expr,
{
    pi(
        BinderInfo::Implicit,
        "G",
        type0(),
        pi(
            BinderInfo::InstImplicit,
            "_inst",
            app(cst("Group"), bvar(0)),
            pi(
                BinderInfo::Default,
                "S",
                app(cst("Subgroup"), bvar(1)),
                pi(
                    BinderInfo::Default,
                    "a",
                    bvar(2),
                    pi(BinderInfo::Default, "b", bvar(3), prop_builder(5)),
                ),
            ),
        ),
    )
}
/// Build `{G : Type} -> [Group G] -> (N : Subgroup G) -> [IsNormal N] -> <body>`.
#[allow(dead_code)]
pub fn mk_normal_law<F>(prop_builder: F) -> Expr
where
    F: FnOnce(u32) -> Expr,
{
    pi(
        BinderInfo::Implicit,
        "G",
        type0(),
        pi(
            BinderInfo::InstImplicit,
            "_inst",
            app(cst("Group"), bvar(0)),
            pi(
                BinderInfo::Default,
                "N",
                app(cst("Subgroup"), bvar(1)),
                pi(
                    BinderInfo::InstImplicit,
                    "_normal",
                    app(cst("IsNormal"), bvar(0)),
                    prop_builder(4),
                ),
            ),
        ),
    )
}
/// Build `{G : Type} -> [Group G] -> (N : Subgroup G) -> [IsNormal N] -> forall (a : G), <body>`.
#[allow(dead_code)]
pub fn mk_normal_forall1<F>(prop_builder: F) -> Expr
where
    F: FnOnce(u32) -> Expr,
{
    pi(
        BinderInfo::Implicit,
        "G",
        type0(),
        pi(
            BinderInfo::InstImplicit,
            "_inst",
            app(cst("Group"), bvar(0)),
            pi(
                BinderInfo::Default,
                "N",
                app(cst("Subgroup"), bvar(1)),
                pi(
                    BinderInfo::InstImplicit,
                    "_normal",
                    app(cst("IsNormal"), bvar(0)),
                    pi(BinderInfo::Default, "a", bvar(3), prop_builder(5)),
                ),
            ),
        ),
    )
}
/// Build `{G : Type} -> [Group G] -> (N : Subgroup G) -> [IsNormal N] -> forall (a b : G), <body>`.
#[allow(dead_code)]
pub fn mk_normal_forall2<F>(prop_builder: F) -> Expr
where
    F: FnOnce(u32) -> Expr,
{
    pi(
        BinderInfo::Implicit,
        "G",
        type0(),
        pi(
            BinderInfo::InstImplicit,
            "_inst",
            app(cst("Group"), bvar(0)),
            pi(
                BinderInfo::Default,
                "N",
                app(cst("Subgroup"), bvar(1)),
                pi(
                    BinderInfo::InstImplicit,
                    "_normal",
                    app(cst("IsNormal"), bvar(0)),
                    pi(
                        BinderInfo::Default,
                        "a",
                        bvar(3),
                        pi(BinderInfo::Default, "b", bvar(4), prop_builder(6)),
                    ),
                ),
            ),
        ),
    )
}
