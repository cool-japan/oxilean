//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{AvlRotation, BTreeNodeData, OrderStatisticsTree, SplayAnalysis, TreapNode};

/// Prop: `Sort 0`.
#[allow(dead_code)]
pub fn prop() -> Expr {
    Expr::Sort(Level::zero())
}
/// Type 1: `Sort 1`.
#[allow(dead_code)]
pub fn type1() -> Expr {
    Expr::Sort(Level::succ(Level::zero()))
}
/// Type 2: `Sort 2`.
#[allow(dead_code)]
pub fn type2() -> Expr {
    Expr::Sort(Level::succ(Level::succ(Level::zero())))
}
/// Nat type constant.
#[allow(dead_code)]
pub fn nat_ty() -> Expr {
    Expr::Const(Name::str("Nat"), vec![])
}
/// Bool type constant.
#[allow(dead_code)]
pub fn bool_ty() -> Expr {
    Expr::Const(Name::str("Bool"), vec![])
}
/// Unit type constant.
#[allow(dead_code)]
pub fn unit_ty() -> Expr {
    Expr::Const(Name::str("Unit"), vec![])
}
/// `RBColor` type constant.
#[allow(dead_code)]
pub fn rbcolor_ty() -> Expr {
    Expr::Const(Name::str("RBColor"), vec![])
}
/// `RBNode` applied to key and value types.
#[allow(dead_code)]
pub fn rbnode_of(key_ty: Expr, val_ty: Expr) -> Expr {
    app2(Expr::Const(Name::str("RBNode"), vec![]), key_ty, val_ty)
}
/// `RBMap` applied to key and value types.
#[allow(dead_code)]
pub fn rbmap_of(key_ty: Expr, val_ty: Expr) -> Expr {
    app2(Expr::Const(Name::str("RBMap"), vec![]), key_ty, val_ty)
}
/// `RBSet` applied to an element type.
#[allow(dead_code)]
pub fn rbset_of(elem_ty: Expr) -> Expr {
    app(Expr::Const(Name::str("RBSet"), vec![]), elem_ty)
}
/// `Option` applied to a type argument.
#[allow(dead_code)]
pub fn option_of(ty: Expr) -> Expr {
    app(Expr::Const(Name::str("Option"), vec![]), ty)
}
/// `List` applied to a type argument.
#[allow(dead_code)]
pub fn list_of(ty: Expr) -> Expr {
    app(Expr::Const(Name::str("List"), vec![]), ty)
}
/// `Prod` applied to two type arguments.
#[allow(dead_code)]
pub fn prod_of(a: Expr, b: Expr) -> Expr {
    app2(Expr::Const(Name::str("Prod"), vec![]), a, b)
}
/// Build a non-dependent arrow `A -> B`.
#[allow(dead_code)]
pub fn arrow(a: Expr, b: Expr) -> Expr {
    Expr::Pi(
        BinderInfo::Default,
        Name::str("_"),
        Box::new(a),
        Box::new(b),
    )
}
/// Function application `f a`.
#[allow(dead_code)]
pub fn app(f: Expr, a: Expr) -> Expr {
    Expr::App(Box::new(f), Box::new(a))
}
/// Function application `f a b`.
#[allow(dead_code)]
pub fn app2(f: Expr, a: Expr, b: Expr) -> Expr {
    app(app(f, a), b)
}
/// Function application `f a b c`.
#[allow(dead_code)]
pub fn app3(f: Expr, a: Expr, b: Expr, c: Expr) -> Expr {
    app(app2(f, a, b), c)
}
/// Function application `f a b c d`.
#[allow(dead_code)]
pub fn app4(f: Expr, a: Expr, b: Expr, c: Expr, d: Expr) -> Expr {
    app(app3(f, a, b, c), d)
}
/// Function application `f a b c d e`.
#[allow(dead_code)]
pub fn app5(f: Expr, a: Expr, b: Expr, c: Expr, d: Expr, e: Expr) -> Expr {
    app(app4(f, a, b, c, d), e)
}
/// An implicit Pi binder.
#[allow(dead_code)]
pub fn implicit_pi(name: &str, ty: Expr, body: Expr) -> Expr {
    Expr::Pi(
        BinderInfo::Implicit,
        Name::str(name),
        Box::new(ty),
        Box::new(body),
    )
}
/// A default (explicit) Pi binder.
#[allow(dead_code)]
pub fn default_pi(name: &str, ty: Expr, body: Expr) -> Expr {
    Expr::Pi(
        BinderInfo::Default,
        Name::str(name),
        Box::new(ty),
        Box::new(body),
    )
}
/// An instance Pi binder `[inst : ty]`.
#[allow(dead_code)]
pub fn inst_pi(name: &str, ty: Expr, body: Expr) -> Expr {
    Expr::Pi(
        BinderInfo::InstImplicit,
        Name::str(name),
        Box::new(ty),
        Box::new(body),
    )
}
/// Build `Eq @{} ty a b`.
#[allow(dead_code)]
pub fn eq_expr(ty: Expr, a: Expr, b: Expr) -> Expr {
    app3(Expr::Const(Name::str("Eq"), vec![]), ty, a, b)
}
/// Shorthand to add an axiom to env.
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
/// `Ord` type class applied to a type.
#[allow(dead_code)]
pub fn ord_of(ty: Expr) -> Expr {
    app(Expr::Const(Name::str("Ord"), vec![]), ty)
}
/// `BEq` type class applied to a type.
#[allow(dead_code)]
pub fn beq_of(ty: Expr) -> Expr {
    app(Expr::Const(Name::str("BEq"), vec![]), ty)
}
/// Build the `RBNode α β` type expression.
#[allow(dead_code)]
pub fn mk_rbnode(key_ty: Expr, val_ty: Expr) -> Expr {
    rbnode_of(key_ty, val_ty)
}
/// Build the `RBMap α β` type expression.
#[allow(dead_code)]
pub fn mk_rbmap(key_ty: Expr, val_ty: Expr) -> Expr {
    rbmap_of(key_ty, val_ty)
}
/// Build the `RBSet α` type expression.
#[allow(dead_code)]
pub fn mk_rbset(key_ty: Expr) -> Expr {
    rbset_of(key_ty)
}
/// Build the `RBColor.red` constructor expression.
#[allow(dead_code)]
pub fn mk_rbcolor_red() -> Expr {
    Expr::Const(Name::str("RBColor.red"), vec![])
}
/// Build the `RBColor.black` constructor expression.
#[allow(dead_code)]
pub fn mk_rbcolor_black() -> Expr {
    Expr::Const(Name::str("RBColor.black"), vec![])
}
/// Build `RBNode.leaf` (the empty node) for given key/value types.
#[allow(dead_code)]
pub fn mk_rbnode_leaf(key_ty: Expr, val_ty: Expr) -> Expr {
    app2(
        Expr::Const(Name::str("RBNode.leaf"), vec![]),
        key_ty,
        val_ty,
    )
}
/// Build `RBNode.node color left key value right`.
#[allow(dead_code)]
pub fn mk_rbnode_node(color: Expr, left: Expr, key: Expr, value: Expr, right: Expr) -> Expr {
    app5(
        Expr::Const(Name::str("RBNode.node"), vec![]),
        color,
        left,
        key,
        value,
        right,
    )
}
/// Build `RBNode.insert t k v`.
#[allow(dead_code)]
pub fn mk_rbnode_insert(tree: Expr, key: Expr, value: Expr) -> Expr {
    app3(
        Expr::Const(Name::str("RBNode.insert"), vec![]),
        tree,
        key,
        value,
    )
}
/// Build `RBNode.find t k`.
#[allow(dead_code)]
pub fn mk_rbnode_find(tree: Expr, key: Expr) -> Expr {
    app2(Expr::Const(Name::str("RBNode.find"), vec![]), tree, key)
}
/// Build `RBMap.empty`.
#[allow(dead_code)]
pub fn mk_rbmap_empty(key_ty: Expr, val_ty: Expr) -> Expr {
    app2(
        Expr::Const(Name::str("RBMap.empty"), vec![]),
        key_ty,
        val_ty,
    )
}
/// Build `RBMap.insert m k v`.
#[allow(dead_code)]
pub fn mk_rbmap_insert(m: Expr, key: Expr, value: Expr) -> Expr {
    app3(
        Expr::Const(Name::str("RBMap.insert"), vec![]),
        m,
        key,
        value,
    )
}
/// Build `RBMap.find m k`.
#[allow(dead_code)]
pub fn mk_rbmap_find(m: Expr, key: Expr) -> Expr {
    app2(Expr::Const(Name::str("RBMap.find"), vec![]), m, key)
}
/// Build `RBSet.insert s a`.
#[allow(dead_code)]
pub fn mk_rbset_insert(s: Expr, elem: Expr) -> Expr {
    app2(Expr::Const(Name::str("RBSet.insert"), vec![]), s, elem)
}
/// Build `RBSet.contains s a`.
#[allow(dead_code)]
pub fn mk_rbset_contains(s: Expr, elem: Expr) -> Expr {
    app2(Expr::Const(Name::str("RBSet.contains"), vec![]), s, elem)
}
/// Build all red-black tree declarations and add them to `env`.
///
/// Declares `RBColor`, `RBNode`, `RBMap`, `RBSet`, core operations,
/// balance helpers, map/set wrappers, and invariant theorems.
///
/// Assumes that `Nat`, `Bool`, `Unit`, `Option`, `List`, `Prod`, `Ord`,
/// `BEq`, and `Eq` are already declared (or referenced by name).
pub fn build_rbtree_env(env: &mut Environment) -> Result<(), String> {
    add_axiom(env, "RBColor", vec![], type1())?;
    add_axiom(env, "RBColor.red", vec![], rbcolor_ty())?;
    add_axiom(env, "RBColor.black", vec![], rbcolor_ty())?;
    let rbnode_ty = arrow(type1(), arrow(type1(), type1()));
    add_axiom(env, "RBNode", vec![], rbnode_ty)?;
    add_axiom(
        env,
        "RBNode.leaf",
        vec![],
        implicit_pi(
            "α",
            type1(),
            implicit_pi("β", type1(), rbnode_of(Expr::BVar(1), Expr::BVar(0))),
        ),
    )?;
    add_axiom(
        env,
        "RBNode.node",
        vec![],
        implicit_pi(
            "α",
            type1(),
            implicit_pi(
                "β",
                type1(),
                default_pi(
                    "color",
                    rbcolor_ty(),
                    default_pi(
                        "left",
                        rbnode_of(Expr::BVar(2), Expr::BVar(1)),
                        default_pi(
                            "key",
                            Expr::BVar(3),
                            default_pi(
                                "value",
                                Expr::BVar(3),
                                default_pi(
                                    "right",
                                    rbnode_of(Expr::BVar(5), Expr::BVar(4)),
                                    rbnode_of(Expr::BVar(6), Expr::BVar(5)),
                                ),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    )?;
    let rbmap_ty = arrow(type1(), arrow(type1(), type1()));
    add_axiom(env, "RBMap", vec![], rbmap_ty)?;
    let rbset_ty = arrow(type1(), type1());
    add_axiom(env, "RBSet", vec![], rbset_ty)?;
    add_axiom(
        env,
        "RBNode.insert",
        vec![],
        implicit_pi(
            "α",
            type1(),
            implicit_pi(
                "β",
                type1(),
                inst_pi(
                    "inst",
                    ord_of(Expr::BVar(1)),
                    default_pi(
                        "t",
                        rbnode_of(Expr::BVar(2), Expr::BVar(1)),
                        default_pi(
                            "k",
                            Expr::BVar(3),
                            default_pi("v", Expr::BVar(3), rbnode_of(Expr::BVar(5), Expr::BVar(4))),
                        ),
                    ),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "RBNode.find",
        vec![],
        implicit_pi(
            "α",
            type1(),
            implicit_pi(
                "β",
                type1(),
                inst_pi(
                    "inst",
                    ord_of(Expr::BVar(1)),
                    default_pi(
                        "t",
                        rbnode_of(Expr::BVar(2), Expr::BVar(1)),
                        default_pi("k", Expr::BVar(3), option_of(Expr::BVar(3))),
                    ),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "RBNode.erase",
        vec![],
        implicit_pi(
            "α",
            type1(),
            implicit_pi(
                "β",
                type1(),
                inst_pi(
                    "inst",
                    ord_of(Expr::BVar(1)),
                    default_pi(
                        "t",
                        rbnode_of(Expr::BVar(2), Expr::BVar(1)),
                        default_pi("k", Expr::BVar(3), rbnode_of(Expr::BVar(4), Expr::BVar(3))),
                    ),
                ),
            ),
        ),
    )?;
    {
        let f_ty = arrow(
            Expr::BVar(0),
            arrow(Expr::BVar(3), arrow(Expr::BVar(3), Expr::BVar(3))),
        );
        add_axiom(
            env,
            "RBNode.fold",
            vec![],
            implicit_pi(
                "α",
                type1(),
                implicit_pi(
                    "β",
                    type1(),
                    implicit_pi(
                        "σ",
                        type1(),
                        default_pi(
                            "f",
                            f_ty,
                            default_pi(
                                "init",
                                Expr::BVar(1),
                                default_pi(
                                    "t",
                                    rbnode_of(Expr::BVar(4), Expr::BVar(3)),
                                    Expr::BVar(3),
                                ),
                            ),
                        ),
                    ),
                ),
            ),
        )?;
    }
    add_axiom(
        env,
        "RBNode.toList",
        vec![],
        implicit_pi(
            "α",
            type1(),
            implicit_pi(
                "β",
                type1(),
                default_pi(
                    "t",
                    rbnode_of(Expr::BVar(1), Expr::BVar(0)),
                    list_of(prod_of(Expr::BVar(2), Expr::BVar(1))),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "RBNode.size",
        vec![],
        implicit_pi(
            "α",
            type1(),
            implicit_pi(
                "β",
                type1(),
                default_pi("t", rbnode_of(Expr::BVar(1), Expr::BVar(0)), nat_ty()),
            ),
        ),
    )?;
    add_axiom(
        env,
        "RBNode.contains",
        vec![],
        implicit_pi(
            "α",
            type1(),
            implicit_pi(
                "β",
                type1(),
                inst_pi(
                    "inst",
                    ord_of(Expr::BVar(1)),
                    default_pi(
                        "t",
                        rbnode_of(Expr::BVar(2), Expr::BVar(1)),
                        default_pi("k", Expr::BVar(3), bool_ty()),
                    ),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "RBNode.isEmpty",
        vec![],
        implicit_pi(
            "α",
            type1(),
            implicit_pi(
                "β",
                type1(),
                default_pi("t", rbnode_of(Expr::BVar(1), Expr::BVar(0)), bool_ty()),
            ),
        ),
    )?;
    add_axiom(
        env,
        "RBNode.balance1",
        vec![],
        implicit_pi(
            "α",
            type1(),
            implicit_pi(
                "β",
                type1(),
                default_pi(
                    "c",
                    rbcolor_ty(),
                    default_pi(
                        "left",
                        rbnode_of(Expr::BVar(2), Expr::BVar(1)),
                        default_pi(
                            "k",
                            Expr::BVar(3),
                            default_pi(
                                "v",
                                Expr::BVar(3),
                                default_pi(
                                    "right",
                                    rbnode_of(Expr::BVar(5), Expr::BVar(4)),
                                    rbnode_of(Expr::BVar(6), Expr::BVar(5)),
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
        "RBNode.balance2",
        vec![],
        implicit_pi(
            "α",
            type1(),
            implicit_pi(
                "β",
                type1(),
                default_pi(
                    "c",
                    rbcolor_ty(),
                    default_pi(
                        "left",
                        rbnode_of(Expr::BVar(2), Expr::BVar(1)),
                        default_pi(
                            "k",
                            Expr::BVar(3),
                            default_pi(
                                "v",
                                Expr::BVar(3),
                                default_pi(
                                    "right",
                                    rbnode_of(Expr::BVar(5), Expr::BVar(4)),
                                    rbnode_of(Expr::BVar(6), Expr::BVar(5)),
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
        "RBNode.ins",
        vec![],
        implicit_pi(
            "α",
            type1(),
            implicit_pi(
                "β",
                type1(),
                inst_pi(
                    "inst",
                    ord_of(Expr::BVar(1)),
                    default_pi(
                        "t",
                        rbnode_of(Expr::BVar(2), Expr::BVar(1)),
                        default_pi(
                            "k",
                            Expr::BVar(3),
                            default_pi("v", Expr::BVar(3), rbnode_of(Expr::BVar(5), Expr::BVar(4))),
                        ),
                    ),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "RBNode.setBlack",
        vec![],
        implicit_pi(
            "α",
            type1(),
            implicit_pi(
                "β",
                type1(),
                default_pi(
                    "t",
                    rbnode_of(Expr::BVar(1), Expr::BVar(0)),
                    rbnode_of(Expr::BVar(2), Expr::BVar(1)),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "RBNode.isRed",
        vec![],
        implicit_pi(
            "α",
            type1(),
            implicit_pi(
                "β",
                type1(),
                default_pi("t", rbnode_of(Expr::BVar(1), Expr::BVar(0)), bool_ty()),
            ),
        ),
    )?;
    add_axiom(
        env,
        "RBMap.empty",
        vec![],
        implicit_pi(
            "α",
            type1(),
            implicit_pi("β", type1(), rbmap_of(Expr::BVar(1), Expr::BVar(0))),
        ),
    )?;
    add_axiom(
        env,
        "RBMap.insert",
        vec![],
        implicit_pi(
            "α",
            type1(),
            implicit_pi(
                "β",
                type1(),
                inst_pi(
                    "inst",
                    ord_of(Expr::BVar(1)),
                    default_pi(
                        "m",
                        rbmap_of(Expr::BVar(2), Expr::BVar(1)),
                        default_pi(
                            "k",
                            Expr::BVar(3),
                            default_pi("v", Expr::BVar(3), rbmap_of(Expr::BVar(5), Expr::BVar(4))),
                        ),
                    ),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "RBMap.find",
        vec![],
        implicit_pi(
            "α",
            type1(),
            implicit_pi(
                "β",
                type1(),
                inst_pi(
                    "inst",
                    ord_of(Expr::BVar(1)),
                    default_pi(
                        "m",
                        rbmap_of(Expr::BVar(2), Expr::BVar(1)),
                        default_pi("k", Expr::BVar(3), option_of(Expr::BVar(3))),
                    ),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "RBMap.erase",
        vec![],
        implicit_pi(
            "α",
            type1(),
            implicit_pi(
                "β",
                type1(),
                inst_pi(
                    "inst",
                    ord_of(Expr::BVar(1)),
                    default_pi(
                        "m",
                        rbmap_of(Expr::BVar(2), Expr::BVar(1)),
                        default_pi("k", Expr::BVar(3), rbmap_of(Expr::BVar(4), Expr::BVar(3))),
                    ),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "RBMap.toList",
        vec![],
        implicit_pi(
            "α",
            type1(),
            implicit_pi(
                "β",
                type1(),
                default_pi(
                    "m",
                    rbmap_of(Expr::BVar(1), Expr::BVar(0)),
                    list_of(prod_of(Expr::BVar(2), Expr::BVar(1))),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "RBMap.ofList",
        vec![],
        implicit_pi(
            "α",
            type1(),
            implicit_pi(
                "β",
                type1(),
                inst_pi(
                    "inst",
                    ord_of(Expr::BVar(1)),
                    default_pi(
                        "l",
                        list_of(prod_of(Expr::BVar(2), Expr::BVar(1))),
                        rbmap_of(Expr::BVar(3), Expr::BVar(2)),
                    ),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "RBMap.size",
        vec![],
        implicit_pi(
            "α",
            type1(),
            implicit_pi(
                "β",
                type1(),
                default_pi("m", rbmap_of(Expr::BVar(1), Expr::BVar(0)), nat_ty()),
            ),
        ),
    )?;
    add_axiom(
        env,
        "RBMap.contains",
        vec![],
        implicit_pi(
            "α",
            type1(),
            implicit_pi(
                "β",
                type1(),
                inst_pi(
                    "inst",
                    ord_of(Expr::BVar(1)),
                    default_pi(
                        "m",
                        rbmap_of(Expr::BVar(2), Expr::BVar(1)),
                        default_pi("k", Expr::BVar(3), bool_ty()),
                    ),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "RBMap.isEmpty",
        vec![],
        implicit_pi(
            "α",
            type1(),
            implicit_pi(
                "β",
                type1(),
                default_pi("m", rbmap_of(Expr::BVar(1), Expr::BVar(0)), bool_ty()),
            ),
        ),
    )?;
    add_axiom(
        env,
        "RBMap.keys",
        vec![],
        implicit_pi(
            "α",
            type1(),
            implicit_pi(
                "β",
                type1(),
                default_pi(
                    "m",
                    rbmap_of(Expr::BVar(1), Expr::BVar(0)),
                    list_of(Expr::BVar(2)),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "RBMap.values",
        vec![],
        implicit_pi(
            "α",
            type1(),
            implicit_pi(
                "β",
                type1(),
                default_pi(
                    "m",
                    rbmap_of(Expr::BVar(1), Expr::BVar(0)),
                    list_of(Expr::BVar(1)),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "RBMap.mapValues",
        vec![],
        implicit_pi(
            "α",
            type1(),
            implicit_pi(
                "β",
                type1(),
                implicit_pi(
                    "γ",
                    type1(),
                    default_pi(
                        "f",
                        arrow(Expr::BVar(1), Expr::BVar(0)),
                        default_pi(
                            "m",
                            rbmap_of(Expr::BVar(3), Expr::BVar(2)),
                            rbmap_of(Expr::BVar(4), Expr::BVar(2)),
                        ),
                    ),
                ),
            ),
        ),
    )?;
    {
        let f_ty = arrow(
            Expr::BVar(3),
            arrow(Expr::BVar(3), option_of(Expr::BVar(3))),
        );
        add_axiom(
            env,
            "RBMap.filterMap",
            vec![],
            implicit_pi(
                "α",
                type1(),
                implicit_pi(
                    "β",
                    type1(),
                    implicit_pi(
                        "γ",
                        type1(),
                        inst_pi(
                            "inst",
                            ord_of(Expr::BVar(2)),
                            default_pi(
                                "f",
                                f_ty,
                                default_pi(
                                    "m",
                                    rbmap_of(Expr::BVar(4), Expr::BVar(3)),
                                    rbmap_of(Expr::BVar(5), Expr::BVar(3)),
                                ),
                            ),
                        ),
                    ),
                ),
            ),
        )?;
    }
    add_axiom(
        env,
        "RBSet.empty",
        vec![],
        implicit_pi("α", type1(), rbset_of(Expr::BVar(0))),
    )?;
    add_axiom(
        env,
        "RBSet.insert",
        vec![],
        implicit_pi(
            "α",
            type1(),
            inst_pi(
                "inst",
                ord_of(Expr::BVar(0)),
                default_pi(
                    "s",
                    rbset_of(Expr::BVar(1)),
                    default_pi("a", Expr::BVar(2), rbset_of(Expr::BVar(3))),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "RBSet.contains",
        vec![],
        implicit_pi(
            "α",
            type1(),
            inst_pi(
                "inst",
                ord_of(Expr::BVar(0)),
                default_pi(
                    "s",
                    rbset_of(Expr::BVar(1)),
                    default_pi("a", Expr::BVar(2), bool_ty()),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "RBSet.erase",
        vec![],
        implicit_pi(
            "α",
            type1(),
            inst_pi(
                "inst",
                ord_of(Expr::BVar(0)),
                default_pi(
                    "s",
                    rbset_of(Expr::BVar(1)),
                    default_pi("a", Expr::BVar(2), rbset_of(Expr::BVar(3))),
                ),
            ),
        ),
    )?;
    for name in &["RBSet.union", "RBSet.inter", "RBSet.diff"] {
        add_axiom(
            env,
            name,
            vec![],
            implicit_pi(
                "α",
                type1(),
                inst_pi(
                    "inst",
                    ord_of(Expr::BVar(0)),
                    default_pi(
                        "s1",
                        rbset_of(Expr::BVar(1)),
                        default_pi("s2", rbset_of(Expr::BVar(2)), rbset_of(Expr::BVar(3))),
                    ),
                ),
            ),
        )?;
    }
    add_axiom(
        env,
        "RBSet.toList",
        vec![],
        implicit_pi(
            "α",
            type1(),
            default_pi("s", rbset_of(Expr::BVar(0)), list_of(Expr::BVar(1))),
        ),
    )?;
    add_axiom(
        env,
        "RBSet.ofList",
        vec![],
        implicit_pi(
            "α",
            type1(),
            inst_pi(
                "inst",
                ord_of(Expr::BVar(0)),
                default_pi("l", list_of(Expr::BVar(1)), rbset_of(Expr::BVar(2))),
            ),
        ),
    )?;
    add_axiom(
        env,
        "RBSet.size",
        vec![],
        implicit_pi(
            "α",
            type1(),
            default_pi("s", rbset_of(Expr::BVar(0)), nat_ty()),
        ),
    )?;
    add_axiom(
        env,
        "RBNode.Balanced",
        vec![],
        implicit_pi(
            "α",
            type1(),
            implicit_pi(
                "β",
                type1(),
                default_pi("t", rbnode_of(Expr::BVar(1), Expr::BVar(0)), prop()),
            ),
        ),
    )?;
    add_axiom(
        env,
        "RBNode.Ordered",
        vec![],
        implicit_pi(
            "α",
            type1(),
            implicit_pi(
                "β",
                type1(),
                inst_pi(
                    "inst",
                    ord_of(Expr::BVar(1)),
                    default_pi("t", rbnode_of(Expr::BVar(2), Expr::BVar(1)), prop()),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "RBNode.insert_balanced",
        vec![],
        implicit_pi(
            "α",
            type1(),
            implicit_pi(
                "β",
                type1(),
                inst_pi(
                    "inst",
                    ord_of(Expr::BVar(1)),
                    default_pi(
                        "t",
                        rbnode_of(Expr::BVar(2), Expr::BVar(1)),
                        default_pi(
                            "k",
                            Expr::BVar(3),
                            default_pi(
                                "v",
                                Expr::BVar(3),
                                default_pi(
                                    "h",
                                    app(
                                        Expr::Const(Name::str("RBNode.Balanced"), vec![]),
                                        Expr::BVar(2),
                                    ),
                                    app(
                                        Expr::Const(Name::str("RBNode.Balanced"), vec![]),
                                        app3(
                                            Expr::Const(Name::str("RBNode.insert"), vec![]),
                                            Expr::BVar(3),
                                            Expr::BVar(2),
                                            Expr::BVar(1),
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
    {
        let insert_expr = app3(
            Expr::Const(Name::str("RBNode.insert"), vec![]),
            Expr::BVar(2),
            Expr::BVar(1),
            Expr::BVar(0),
        );
        let find_expr = app2(
            Expr::Const(Name::str("RBNode.find"), vec![]),
            insert_expr,
            Expr::BVar(1),
        );
        let some_v = app(Expr::Const(Name::str("Option.some"), vec![]), Expr::BVar(0));
        let result_ty = option_of(Expr::BVar(4));
        add_axiom(
            env,
            "RBNode.find_insert_same",
            vec![],
            implicit_pi(
                "α",
                type1(),
                implicit_pi(
                    "β",
                    type1(),
                    inst_pi(
                        "inst",
                        ord_of(Expr::BVar(1)),
                        default_pi(
                            "t",
                            rbnode_of(Expr::BVar(2), Expr::BVar(1)),
                            default_pi(
                                "k",
                                Expr::BVar(3),
                                default_pi(
                                    "v",
                                    Expr::BVar(3),
                                    eq_expr(result_ty, find_expr, some_v),
                                ),
                            ),
                        ),
                    ),
                ),
            ),
        )?;
    }
    add_axiom(
        env,
        "RBNode.ordered_insert",
        vec![],
        implicit_pi(
            "α",
            type1(),
            implicit_pi(
                "β",
                type1(),
                inst_pi(
                    "inst",
                    ord_of(Expr::BVar(1)),
                    default_pi(
                        "t",
                        rbnode_of(Expr::BVar(2), Expr::BVar(1)),
                        default_pi(
                            "k",
                            Expr::BVar(3),
                            default_pi(
                                "v",
                                Expr::BVar(3),
                                default_pi(
                                    "h",
                                    app(
                                        Expr::Const(Name::str("RBNode.Ordered"), vec![]),
                                        Expr::BVar(2),
                                    ),
                                    app(
                                        Expr::Const(Name::str("RBNode.Ordered"), vec![]),
                                        app3(
                                            Expr::Const(Name::str("RBNode.insert"), vec![]),
                                            Expr::BVar(3),
                                            Expr::BVar(2),
                                            Expr::BVar(1),
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
    {
        let insert_expr = app3(
            Expr::Const(Name::str("RBNode.insert"), vec![]),
            Expr::BVar(2),
            Expr::BVar(1),
            Expr::BVar(0),
        );
        let size_insert = app(Expr::Const(Name::str("RBNode.size"), vec![]), insert_expr);
        let size_t = app(Expr::Const(Name::str("RBNode.size"), vec![]), Expr::BVar(2));
        let succ_size_t = app(Expr::Const(Name::str("Nat.succ"), vec![]), size_t);
        let le_prop = app2(
            Expr::Const(Name::str("Nat.le"), vec![]),
            size_insert,
            succ_size_t,
        );
        add_axiom(
            env,
            "RBNode.size_insert_le",
            vec![],
            implicit_pi(
                "α",
                type1(),
                implicit_pi(
                    "β",
                    type1(),
                    inst_pi(
                        "inst",
                        ord_of(Expr::BVar(1)),
                        default_pi(
                            "t",
                            rbnode_of(Expr::BVar(2), Expr::BVar(1)),
                            default_pi("k", Expr::BVar(3), default_pi("v", Expr::BVar(3), le_prop)),
                        ),
                    ),
                ),
            ),
        )?;
    }
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    /// Set up a minimal environment with all prerequisites.
    fn setup_env() -> Environment {
        let mut env = Environment::new();
        for name in &[
            "Nat", "Bool", "Unit", "Ord", "BEq", "Eq", "Nat.le", "Nat.succ",
        ] {
            env.add(Declaration::Axiom {
                name: Name::str(*name),
                univ_params: vec![],
                ty: type1(),
            })
            .expect("operation should succeed");
        }
        env.add(Declaration::Axiom {
            name: Name::str("Option"),
            univ_params: vec![],
            ty: arrow(type1(), type1()),
        })
        .expect("operation should succeed");
        env.add(Declaration::Axiom {
            name: Name::str("Option.some"),
            univ_params: vec![],
            ty: implicit_pi(
                "α",
                type1(),
                default_pi("a", Expr::BVar(0), option_of(Expr::BVar(1))),
            ),
        })
        .expect("operation should succeed");
        env.add(Declaration::Axiom {
            name: Name::str("List"),
            univ_params: vec![],
            ty: arrow(type1(), type1()),
        })
        .expect("operation should succeed");
        env.add(Declaration::Axiom {
            name: Name::str("Prod"),
            univ_params: vec![],
            ty: arrow(type1(), arrow(type1(), type1())),
        })
        .expect("operation should succeed");
        env
    }
    #[test]
    fn test_build_rbtree_env() {
        let mut env = setup_env();
        assert!(build_rbtree_env(&mut env).is_ok());
    }
    #[test]
    fn test_rbcolor_type() {
        let mut env = setup_env();
        build_rbtree_env(&mut env).expect("build_rbtree_env should succeed");
        assert!(env.get(&Name::str("RBColor")).is_some());
        assert!(env.get(&Name::str("RBColor.red")).is_some());
        assert!(env.get(&Name::str("RBColor.black")).is_some());
    }
    #[test]
    fn test_rbnode_type() {
        let mut env = setup_env();
        build_rbtree_env(&mut env).expect("build_rbtree_env should succeed");
        assert!(env.get(&Name::str("RBNode")).is_some());
    }
    #[test]
    fn test_rbnode_leaf() {
        let mut env = setup_env();
        build_rbtree_env(&mut env).expect("build_rbtree_env should succeed");
        let decl = env
            .get(&Name::str("RBNode.leaf"))
            .expect("declaration 'RBNode.leaf' should exist in env");
        assert!(matches!(decl, Declaration::Axiom { .. }));
    }
    #[test]
    fn test_rbnode_node() {
        let mut env = setup_env();
        build_rbtree_env(&mut env).expect("build_rbtree_env should succeed");
        let decl = env
            .get(&Name::str("RBNode.node"))
            .expect("declaration 'RBNode.node' should exist in env");
        assert!(decl.ty().is_pi());
    }
    #[test]
    fn test_rbnode_insert() {
        let mut env = setup_env();
        build_rbtree_env(&mut env).expect("build_rbtree_env should succeed");
        let decl = env
            .get(&Name::str("RBNode.insert"))
            .expect("declaration 'RBNode.insert' should exist in env");
        assert!(decl.ty().is_pi());
    }
    #[test]
    fn test_rbnode_find() {
        let mut env = setup_env();
        build_rbtree_env(&mut env).expect("build_rbtree_env should succeed");
        let decl = env
            .get(&Name::str("RBNode.find"))
            .expect("declaration 'RBNode.find' should exist in env");
        assert!(decl.ty().is_pi());
    }
    #[test]
    fn test_rbnode_erase() {
        let mut env = setup_env();
        build_rbtree_env(&mut env).expect("build_rbtree_env should succeed");
        let decl = env
            .get(&Name::str("RBNode.erase"))
            .expect("declaration 'RBNode.erase' should exist in env");
        assert!(decl.ty().is_pi());
    }
    #[test]
    fn test_rbnode_fold() {
        let mut env = setup_env();
        build_rbtree_env(&mut env).expect("build_rbtree_env should succeed");
        let decl = env
            .get(&Name::str("RBNode.fold"))
            .expect("declaration 'RBNode.fold' should exist in env");
        assert!(decl.ty().is_pi());
    }
    #[test]
    fn test_rbnode_tolist() {
        let mut env = setup_env();
        build_rbtree_env(&mut env).expect("build_rbtree_env should succeed");
        let decl = env
            .get(&Name::str("RBNode.toList"))
            .expect("declaration 'RBNode.toList' should exist in env");
        assert!(decl.ty().is_pi());
    }
    #[test]
    fn test_rbnode_size() {
        let mut env = setup_env();
        build_rbtree_env(&mut env).expect("build_rbtree_env should succeed");
        let decl = env
            .get(&Name::str("RBNode.size"))
            .expect("declaration 'RBNode.size' should exist in env");
        assert!(decl.ty().is_pi());
    }
    #[test]
    fn test_rbnode_contains() {
        let mut env = setup_env();
        build_rbtree_env(&mut env).expect("build_rbtree_env should succeed");
        let decl = env
            .get(&Name::str("RBNode.contains"))
            .expect("declaration 'RBNode.contains' should exist in env");
        assert!(decl.ty().is_pi());
    }
    #[test]
    fn test_rbnode_isempty() {
        let mut env = setup_env();
        build_rbtree_env(&mut env).expect("build_rbtree_env should succeed");
        let decl = env
            .get(&Name::str("RBNode.isEmpty"))
            .expect("declaration 'RBNode.isEmpty' should exist in env");
        assert!(decl.ty().is_pi());
    }
    #[test]
    fn test_rbnode_balance1() {
        let mut env = setup_env();
        build_rbtree_env(&mut env).expect("build_rbtree_env should succeed");
        assert!(env.get(&Name::str("RBNode.balance1")).is_some());
    }
    #[test]
    fn test_rbnode_balance2() {
        let mut env = setup_env();
        build_rbtree_env(&mut env).expect("build_rbtree_env should succeed");
        assert!(env.get(&Name::str("RBNode.balance2")).is_some());
    }
    #[test]
    fn test_rbnode_ins() {
        let mut env = setup_env();
        build_rbtree_env(&mut env).expect("build_rbtree_env should succeed");
        assert!(env.get(&Name::str("RBNode.ins")).is_some());
    }
    #[test]
    fn test_rbnode_setblack() {
        let mut env = setup_env();
        build_rbtree_env(&mut env).expect("build_rbtree_env should succeed");
        assert!(env.get(&Name::str("RBNode.setBlack")).is_some());
    }
    #[test]
    fn test_rbnode_isred() {
        let mut env = setup_env();
        build_rbtree_env(&mut env).expect("build_rbtree_env should succeed");
        assert!(env.get(&Name::str("RBNode.isRed")).is_some());
    }
    #[test]
    fn test_rbmap_type() {
        let mut env = setup_env();
        build_rbtree_env(&mut env).expect("build_rbtree_env should succeed");
        assert!(env.get(&Name::str("RBMap")).is_some());
    }
    #[test]
    fn test_rbmap_empty() {
        let mut env = setup_env();
        build_rbtree_env(&mut env).expect("build_rbtree_env should succeed");
        let decl = env
            .get(&Name::str("RBMap.empty"))
            .expect("declaration 'RBMap.empty' should exist in env");
        assert!(decl.ty().is_pi());
    }
    #[test]
    fn test_rbmap_insert() {
        let mut env = setup_env();
        build_rbtree_env(&mut env).expect("build_rbtree_env should succeed");
        let decl = env
            .get(&Name::str("RBMap.insert"))
            .expect("declaration 'RBMap.insert' should exist in env");
        assert!(decl.ty().is_pi());
    }
    #[test]
    fn test_rbmap_find() {
        let mut env = setup_env();
        build_rbtree_env(&mut env).expect("build_rbtree_env should succeed");
        let decl = env
            .get(&Name::str("RBMap.find"))
            .expect("declaration 'RBMap.find' should exist in env");
        assert!(decl.ty().is_pi());
    }
    #[test]
    fn test_rbmap_erase() {
        let mut env = setup_env();
        build_rbtree_env(&mut env).expect("build_rbtree_env should succeed");
        assert!(env.get(&Name::str("RBMap.erase")).is_some());
    }
    #[test]
    fn test_rbmap_tolist() {
        let mut env = setup_env();
        build_rbtree_env(&mut env).expect("build_rbtree_env should succeed");
        assert!(env.get(&Name::str("RBMap.toList")).is_some());
    }
    #[test]
    fn test_rbmap_oflist() {
        let mut env = setup_env();
        build_rbtree_env(&mut env).expect("build_rbtree_env should succeed");
        assert!(env.get(&Name::str("RBMap.ofList")).is_some());
    }
    #[test]
    fn test_rbmap_size() {
        let mut env = setup_env();
        build_rbtree_env(&mut env).expect("build_rbtree_env should succeed");
        assert!(env.get(&Name::str("RBMap.size")).is_some());
    }
    #[test]
    fn test_rbmap_contains() {
        let mut env = setup_env();
        build_rbtree_env(&mut env).expect("build_rbtree_env should succeed");
        assert!(env.get(&Name::str("RBMap.contains")).is_some());
    }
    #[test]
    fn test_rbmap_isempty() {
        let mut env = setup_env();
        build_rbtree_env(&mut env).expect("build_rbtree_env should succeed");
        assert!(env.get(&Name::str("RBMap.isEmpty")).is_some());
    }
    #[test]
    fn test_rbmap_keys() {
        let mut env = setup_env();
        build_rbtree_env(&mut env).expect("build_rbtree_env should succeed");
        assert!(env.get(&Name::str("RBMap.keys")).is_some());
    }
    #[test]
    fn test_rbmap_values() {
        let mut env = setup_env();
        build_rbtree_env(&mut env).expect("build_rbtree_env should succeed");
        assert!(env.get(&Name::str("RBMap.values")).is_some());
    }
    #[test]
    fn test_rbmap_mapvalues() {
        let mut env = setup_env();
        build_rbtree_env(&mut env).expect("build_rbtree_env should succeed");
        let decl = env
            .get(&Name::str("RBMap.mapValues"))
            .expect("declaration 'RBMap.mapValues' should exist in env");
        assert!(decl.ty().is_pi());
    }
    #[test]
    fn test_rbmap_filtermap() {
        let mut env = setup_env();
        build_rbtree_env(&mut env).expect("build_rbtree_env should succeed");
        let decl = env
            .get(&Name::str("RBMap.filterMap"))
            .expect("declaration 'RBMap.filterMap' should exist in env");
        assert!(decl.ty().is_pi());
    }
    #[test]
    fn test_rbset_type() {
        let mut env = setup_env();
        build_rbtree_env(&mut env).expect("build_rbtree_env should succeed");
        assert!(env.get(&Name::str("RBSet")).is_some());
    }
    #[test]
    fn test_rbset_empty() {
        let mut env = setup_env();
        build_rbtree_env(&mut env).expect("build_rbtree_env should succeed");
        assert!(env.get(&Name::str("RBSet.empty")).is_some());
    }
    #[test]
    fn test_rbset_insert() {
        let mut env = setup_env();
        build_rbtree_env(&mut env).expect("build_rbtree_env should succeed");
        let decl = env
            .get(&Name::str("RBSet.insert"))
            .expect("declaration 'RBSet.insert' should exist in env");
        assert!(decl.ty().is_pi());
    }
    #[test]
    fn test_rbset_contains() {
        let mut env = setup_env();
        build_rbtree_env(&mut env).expect("build_rbtree_env should succeed");
        assert!(env.get(&Name::str("RBSet.contains")).is_some());
    }
    #[test]
    fn test_rbset_erase() {
        let mut env = setup_env();
        build_rbtree_env(&mut env).expect("build_rbtree_env should succeed");
        assert!(env.get(&Name::str("RBSet.erase")).is_some());
    }
    #[test]
    fn test_rbset_union() {
        let mut env = setup_env();
        build_rbtree_env(&mut env).expect("build_rbtree_env should succeed");
        assert!(env.get(&Name::str("RBSet.union")).is_some());
    }
    #[test]
    fn test_rbset_inter() {
        let mut env = setup_env();
        build_rbtree_env(&mut env).expect("build_rbtree_env should succeed");
        assert!(env.get(&Name::str("RBSet.inter")).is_some());
    }
    #[test]
    fn test_rbset_diff() {
        let mut env = setup_env();
        build_rbtree_env(&mut env).expect("build_rbtree_env should succeed");
        assert!(env.get(&Name::str("RBSet.diff")).is_some());
    }
    #[test]
    fn test_rbset_tolist() {
        let mut env = setup_env();
        build_rbtree_env(&mut env).expect("build_rbtree_env should succeed");
        assert!(env.get(&Name::str("RBSet.toList")).is_some());
    }
    #[test]
    fn test_rbset_oflist() {
        let mut env = setup_env();
        build_rbtree_env(&mut env).expect("build_rbtree_env should succeed");
        assert!(env.get(&Name::str("RBSet.ofList")).is_some());
    }
    #[test]
    fn test_rbset_size() {
        let mut env = setup_env();
        build_rbtree_env(&mut env).expect("build_rbtree_env should succeed");
        assert!(env.get(&Name::str("RBSet.size")).is_some());
    }
    #[test]
    fn test_rbnode_balanced_predicate() {
        let mut env = setup_env();
        build_rbtree_env(&mut env).expect("build_rbtree_env should succeed");
        assert!(env.get(&Name::str("RBNode.Balanced")).is_some());
    }
    #[test]
    fn test_rbnode_ordered_predicate() {
        let mut env = setup_env();
        build_rbtree_env(&mut env).expect("build_rbtree_env should succeed");
        assert!(env.get(&Name::str("RBNode.Ordered")).is_some());
    }
    #[test]
    fn test_insert_balanced_theorem() {
        let mut env = setup_env();
        build_rbtree_env(&mut env).expect("build_rbtree_env should succeed");
        let decl = env
            .get(&Name::str("RBNode.insert_balanced"))
            .expect("declaration 'RBNode.insert_balanced' should exist in env");
        assert!(decl.ty().is_pi());
    }
    #[test]
    fn test_find_insert_same_theorem() {
        let mut env = setup_env();
        build_rbtree_env(&mut env).expect("build_rbtree_env should succeed");
        let decl = env
            .get(&Name::str("RBNode.find_insert_same"))
            .expect("declaration 'RBNode.find_insert_same' should exist in env");
        assert!(decl.ty().is_pi());
    }
    #[test]
    fn test_ordered_insert_theorem() {
        let mut env = setup_env();
        build_rbtree_env(&mut env).expect("build_rbtree_env should succeed");
        let decl = env
            .get(&Name::str("RBNode.ordered_insert"))
            .expect("declaration 'RBNode.ordered_insert' should exist in env");
        assert!(decl.ty().is_pi());
    }
    #[test]
    fn test_size_insert_le_theorem() {
        let mut env = setup_env();
        build_rbtree_env(&mut env).expect("build_rbtree_env should succeed");
        let decl = env
            .get(&Name::str("RBNode.size_insert_le"))
            .expect("declaration 'RBNode.size_insert_le' should exist in env");
        assert!(decl.ty().is_pi());
    }
    #[test]
    fn test_mk_rbnode_expr() {
        let t = mk_rbnode(nat_ty(), bool_ty());
        assert!(matches!(t, Expr::App(_, _)));
    }
    #[test]
    fn test_mk_rbmap_expr() {
        let t = mk_rbmap(nat_ty(), bool_ty());
        assert!(matches!(t, Expr::App(_, _)));
    }
    #[test]
    fn test_mk_rbset_expr() {
        let t = mk_rbset(nat_ty());
        assert!(matches!(t, Expr::App(_, _)));
    }
    #[test]
    fn test_mk_rbcolor_red_expr() {
        let c = mk_rbcolor_red();
        assert!(matches!(c, Expr::Const(_, _)));
    }
    #[test]
    fn test_mk_rbcolor_black_expr() {
        let c = mk_rbcolor_black();
        assert!(matches!(c, Expr::Const(_, _)));
    }
    #[test]
    fn test_mk_rbnode_leaf_expr() {
        let leaf = mk_rbnode_leaf(nat_ty(), bool_ty());
        assert!(matches!(leaf, Expr::App(_, _)));
    }
    #[test]
    fn test_mk_rbnode_node_expr() {
        let leaf = mk_rbnode_leaf(nat_ty(), bool_ty());
        let node = mk_rbnode_node(
            mk_rbcolor_red(),
            leaf.clone(),
            Expr::Const(Name::str("k"), vec![]),
            Expr::Const(Name::str("v"), vec![]),
            leaf,
        );
        assert!(matches!(node, Expr::App(_, _)));
    }
    #[test]
    fn test_mk_rbnode_insert_expr() {
        let leaf = mk_rbnode_leaf(nat_ty(), bool_ty());
        let expr = mk_rbnode_insert(
            leaf,
            Expr::Const(Name::str("k"), vec![]),
            Expr::Const(Name::str("v"), vec![]),
        );
        assert!(matches!(expr, Expr::App(_, _)));
    }
    #[test]
    fn test_mk_rbnode_find_expr() {
        let leaf = mk_rbnode_leaf(nat_ty(), bool_ty());
        let expr = mk_rbnode_find(leaf, Expr::Const(Name::str("k"), vec![]));
        assert!(matches!(expr, Expr::App(_, _)));
    }
    #[test]
    fn test_mk_rbmap_empty_expr() {
        let m = mk_rbmap_empty(nat_ty(), bool_ty());
        assert!(matches!(m, Expr::App(_, _)));
    }
    #[test]
    fn test_mk_rbmap_insert_expr() {
        let m = mk_rbmap_empty(nat_ty(), bool_ty());
        let expr = mk_rbmap_insert(
            m,
            Expr::Const(Name::str("k"), vec![]),
            Expr::Const(Name::str("v"), vec![]),
        );
        assert!(matches!(expr, Expr::App(_, _)));
    }
    #[test]
    fn test_mk_rbmap_find_expr() {
        let m = mk_rbmap_empty(nat_ty(), bool_ty());
        let expr = mk_rbmap_find(m, Expr::Const(Name::str("k"), vec![]));
        assert!(matches!(expr, Expr::App(_, _)));
    }
    #[test]
    fn test_mk_rbset_insert_expr() {
        let s = Expr::Const(Name::str("s"), vec![]);
        let expr = mk_rbset_insert(s, Expr::Const(Name::str("a"), vec![]));
        assert!(matches!(expr, Expr::App(_, _)));
    }
    #[test]
    fn test_mk_rbset_contains_expr() {
        let s = Expr::Const(Name::str("s"), vec![]);
        let expr = mk_rbset_contains(s, Expr::Const(Name::str("a"), vec![]));
        assert!(matches!(expr, Expr::App(_, _)));
    }
    #[test]
    fn test_all_rbtree_decls_are_axioms() {
        let mut env = setup_env();
        build_rbtree_env(&mut env).expect("build_rbtree_env should succeed");
        let names = [
            "RBColor",
            "RBColor.red",
            "RBColor.black",
            "RBNode",
            "RBNode.leaf",
            "RBNode.node",
            "RBNode.insert",
            "RBNode.find",
            "RBNode.erase",
            "RBNode.fold",
            "RBNode.toList",
            "RBNode.size",
            "RBNode.contains",
            "RBNode.isEmpty",
            "RBNode.balance1",
            "RBNode.balance2",
            "RBNode.ins",
            "RBNode.setBlack",
            "RBNode.isRed",
            "RBMap",
            "RBMap.empty",
            "RBMap.insert",
            "RBMap.find",
            "RBMap.erase",
            "RBMap.toList",
            "RBMap.ofList",
            "RBMap.size",
            "RBMap.contains",
            "RBMap.isEmpty",
            "RBMap.keys",
            "RBMap.values",
            "RBMap.mapValues",
            "RBMap.filterMap",
            "RBSet",
            "RBSet.empty",
            "RBSet.insert",
            "RBSet.contains",
            "RBSet.erase",
            "RBSet.union",
            "RBSet.inter",
            "RBSet.diff",
            "RBSet.toList",
            "RBSet.ofList",
            "RBSet.size",
            "RBNode.Balanced",
            "RBNode.Ordered",
            "RBNode.insert_balanced",
            "RBNode.find_insert_same",
            "RBNode.ordered_insert",
            "RBNode.size_insert_le",
        ];
        for name in &names {
            let decl = env.get(&Name::str(*name));
            assert!(decl.is_some(), "missing declaration: {}", name);
            assert!(
                matches!(
                    decl.expect("declaration should exist"),
                    Declaration::Axiom { .. }
                ),
                "{} should be an axiom",
                name
            );
        }
    }
    #[test]
    fn test_rbtree_declaration_count() {
        let mut env = setup_env();
        let pre = env.len();
        build_rbtree_env(&mut env).expect("build_rbtree_env should succeed");
        let added = env.len() - pre;
        assert!(added >= 45, "expected >= 45 declarations, got {}", added);
    }
    #[test]
    fn test_rbnode_insert_type_is_deeply_nested_pi() {
        let mut env = setup_env();
        build_rbtree_env(&mut env).expect("build_rbtree_env should succeed");
        let decl = env
            .get(&Name::str("RBNode.insert"))
            .expect("declaration 'RBNode.insert' should exist in env");
        let mut ty = decl.ty().clone();
        let mut depth = 0;
        while let Expr::Pi(_, _, _, body) = ty {
            depth += 1;
            ty = *body;
        }
        assert!(
            depth >= 6,
            "insert should have >= 6 Pi levels, got {}",
            depth
        );
    }
    #[test]
    fn test_rbmap_filtermap_type_is_deeply_nested_pi() {
        let mut env = setup_env();
        build_rbtree_env(&mut env).expect("build_rbtree_env should succeed");
        let decl = env
            .get(&Name::str("RBMap.filterMap"))
            .expect("declaration 'RBMap.filterMap' should exist in env");
        let mut ty = decl.ty().clone();
        let mut depth = 0;
        while let Expr::Pi(_, _, _, body) = ty {
            depth += 1;
            ty = *body;
        }
        assert!(
            depth >= 6,
            "filterMap should have >= 6 Pi levels, got {}",
            depth
        );
    }
}
#[cfg(test)]
mod extended_rbtree_tests {
    use super::*;
    #[test]
    fn test_btree_node() {
        let leaf = BTreeNodeData::leaf(3, vec![1, 2, 3, 4, 5]);
        assert!(leaf.needs_split());
        let small = BTreeNodeData::leaf(3, vec![1]);
        assert!(!small.needs_split());
    }
    #[test]
    fn test_avl_rotation() {
        let ll = AvlRotation::LeftLeft;
        assert_eq!(ll.num_rotations(), 1);
        let lr = AvlRotation::LeftRight;
        assert_eq!(lr.num_rotations(), 2);
        assert!(lr.description().contains("double"));
    }
    #[test]
    fn test_treap_node() {
        let _t = TreapNode::new(42, 12345);
        let desc = TreapNode::expected_height_description(100);
        assert!(desc.contains("n=100"));
    }
    #[test]
    fn test_splay_analysis() {
        let sa = SplayAnalysis::new(vec![1, 2, 3, 4, 5, 6, 7, 8]);
        assert_eq!(sa.sequence_length, 8);
        assert!(sa.avg_cost() > 0.0);
    }
    #[test]
    fn test_order_statistics_tree() {
        let ost = OrderStatisticsTree::new(vec![5, 3, 1, 4, 2]);
        assert_eq!(ost.kth_smallest(1), Some(1));
        assert_eq!(ost.kth_smallest(3), Some(3));
        assert_eq!(ost.kth_smallest(0), None);
        assert_eq!(ost.rank(3), 3);
    }
}
