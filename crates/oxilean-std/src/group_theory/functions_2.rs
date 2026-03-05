//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::functions::*;

/// Build the group theory environment with group type class, subgroups,
/// homomorphisms, normal subgroups, quotient groups, cyclic groups, and
/// Lagrange's theorem.
#[allow(clippy::too_many_lines)]
pub fn build_group_theory_env(env: &mut Environment) -> Result<(), String> {
    add_axiom(env, "Group", vec![], mk_class_ty())?;
    add_axiom(
        env,
        "Group.mul",
        vec![],
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
                    pi(BinderInfo::Default, "b", bvar(2), bvar(3)),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "Group.one",
        vec![],
        pi(
            BinderInfo::Implicit,
            "G",
            type0(),
            pi(
                BinderInfo::InstImplicit,
                "_inst",
                app(cst("Group"), bvar(0)),
                bvar(1),
            ),
        ),
    )?;
    add_axiom(
        env,
        "Group.inv",
        vec![],
        pi(
            BinderInfo::Implicit,
            "G",
            type0(),
            pi(
                BinderInfo::InstImplicit,
                "_inst",
                app(cst("Group"), bvar(0)),
                pi(BinderInfo::Default, "a", bvar(1), bvar(2)),
            ),
        ),
    )?;
    add_axiom(
        env,
        "Group.mul_assoc",
        vec![],
        mk_group_forall3(|depth| {
            let g = bvar(depth - 1);
            let a = bvar(2);
            let b = bvar(1);
            let c = bvar(0);
            mk_eq(
                g,
                app2(
                    cst("Group.mul"),
                    app2(cst("Group.mul"), a.clone(), b.clone()),
                    c.clone(),
                ),
                app2(cst("Group.mul"), a, app2(cst("Group.mul"), b, c)),
            )
        }),
    )?;
    add_axiom(
        env,
        "Group.one_mul",
        vec![],
        mk_group_forall1(|depth| {
            let g = bvar(depth - 1);
            let a = bvar(0);
            mk_eq(g, app2(cst("Group.mul"), cst("Group.one"), a.clone()), a)
        }),
    )?;
    add_axiom(
        env,
        "Group.mul_one",
        vec![],
        mk_group_forall1(|depth| {
            let g = bvar(depth - 1);
            let a = bvar(0);
            mk_eq(g, app2(cst("Group.mul"), a.clone(), cst("Group.one")), a)
        }),
    )?;
    add_axiom(
        env,
        "Group.inv_mul_cancel",
        vec![],
        mk_group_forall1(|depth| {
            let g = bvar(depth - 1);
            let a = bvar(0);
            mk_eq(
                g,
                app2(cst("Group.mul"), app(cst("Group.inv"), a), cst("Group.one")),
                cst("Group.one"),
            )
        }),
    )?;
    add_axiom(
        env,
        "Group.mul_inv_cancel",
        vec![],
        mk_group_forall1(|depth| {
            let g = bvar(depth - 1);
            let a = bvar(0);
            mk_eq(
                g,
                app2(cst("Group.mul"), a.clone(), app(cst("Group.inv"), a)),
                cst("Group.one"),
            )
        }),
    )?;
    add_axiom(
        env,
        "Group.inv_inv",
        vec![],
        mk_group_forall1(|depth| {
            let g = bvar(depth - 1);
            let a = bvar(0);
            mk_eq(
                g,
                app(cst("Group.inv"), app(cst("Group.inv"), a.clone())),
                a,
            )
        }),
    )?;
    add_axiom(
        env,
        "Group.inv_one",
        vec![],
        mk_group_law(|depth| {
            let g = bvar(depth - 1);
            mk_eq(g, app(cst("Group.inv"), cst("Group.one")), cst("Group.one"))
        }),
    )?;
    add_axiom(
        env,
        "Group.mul_left_cancel",
        vec![],
        mk_group_forall3(|depth| {
            let g = bvar(depth - 1);
            let a = bvar(2);
            let b = bvar(1);
            let c = bvar(0);
            arrow(
                mk_eq(
                    g.clone(),
                    app2(cst("Group.mul"), a.clone(), b.clone()),
                    app2(cst("Group.mul"), a, c.clone()),
                ),
                mk_eq(g, b, c),
            )
        }),
    )?;
    add_axiom(
        env,
        "Group.mul_right_cancel",
        vec![],
        mk_group_forall3(|depth| {
            let g = bvar(depth - 1);
            let a = bvar(2);
            let b = bvar(1);
            let c = bvar(0);
            arrow(
                mk_eq(
                    g.clone(),
                    app2(cst("Group.mul"), a.clone(), c.clone()),
                    app2(cst("Group.mul"), b.clone(), c),
                ),
                mk_eq(g, a, b),
            )
        }),
    )?;
    add_axiom(
        env,
        "Group.inv_mul_rev",
        vec![],
        mk_group_forall2(|depth| {
            let g = bvar(depth - 1);
            let a = bvar(1);
            let b = bvar(0);
            mk_eq(
                g,
                app(
                    cst("Group.inv"),
                    app2(cst("Group.mul"), a.clone(), b.clone()),
                ),
                app2(
                    cst("Group.mul"),
                    app(cst("Group.inv"), b),
                    app(cst("Group.inv"), a),
                ),
            )
        }),
    )?;
    add_axiom(
        env,
        "Group.mul_eq_one_iff_eq_inv",
        vec![],
        mk_group_forall2(|depth| {
            let g = bvar(depth - 1);
            let a = bvar(1);
            let b = bvar(0);
            mk_iff(
                mk_eq(
                    g.clone(),
                    app2(cst("Group.mul"), a.clone(), b.clone()),
                    cst("Group.one"),
                ),
                mk_eq(g, a, app(cst("Group.inv"), b)),
            )
        }),
    )?;
    add_axiom(
        env,
        "Group.eq_inv_iff_mul_eq_one",
        vec![],
        mk_group_forall2(|depth| {
            let g = bvar(depth - 1);
            let a = bvar(1);
            let b = bvar(0);
            mk_iff(
                mk_eq(g.clone(), a.clone(), app(cst("Group.inv"), b.clone())),
                mk_eq(g, app2(cst("Group.mul"), a, b), cst("Group.one")),
            )
        }),
    )?;
    add_axiom(
        env,
        "Group.inv_eq_of_mul_eq_one",
        vec![],
        mk_group_forall2(|depth| {
            let g = bvar(depth - 1);
            let a = bvar(1);
            let b = bvar(0);
            arrow(
                mk_eq(
                    g.clone(),
                    app2(cst("Group.mul"), a.clone(), b.clone()),
                    cst("Group.one"),
                ),
                mk_eq(g, app(cst("Group.inv"), a), b),
            )
        }),
    )?;
    add_axiom(env, "CommGroup", vec![], mk_class_ty())?;
    add_axiom(
        env,
        "CommGroup.toGroup",
        vec![],
        pi(
            BinderInfo::Implicit,
            "G",
            type0(),
            pi(
                BinderInfo::InstImplicit,
                "_inst",
                app(cst("CommGroup"), bvar(0)),
                app(cst("Group"), bvar(1)),
            ),
        ),
    )?;
    add_axiom(
        env,
        "CommGroup.mul_comm",
        vec![],
        pi(
            BinderInfo::Implicit,
            "G",
            type0(),
            pi(
                BinderInfo::InstImplicit,
                "_inst",
                app(cst("CommGroup"), bvar(0)),
                pi(
                    BinderInfo::Default,
                    "a",
                    bvar(1),
                    pi(BinderInfo::Default, "b", bvar(2), {
                        let g = bvar(3);
                        let a = bvar(1);
                        let b = bvar(0);
                        mk_eq(
                            g,
                            app2(cst("Group.mul"), a.clone(), b.clone()),
                            app2(cst("Group.mul"), b, a),
                        )
                    }),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "Subgroup",
        vec![],
        pi(
            BinderInfo::Implicit,
            "G",
            type0(),
            pi(
                BinderInfo::InstImplicit,
                "_inst",
                app(cst("Group"), bvar(0)),
                type1(),
            ),
        ),
    )?;
    add_axiom(
        env,
        "Subgroup.mem",
        vec![],
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
                    pi(BinderInfo::Default, "a", bvar(2), prop()),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "Subgroup.one_mem",
        vec![],
        mk_subgroup_law(|_depth| app2(cst("Subgroup.mem"), bvar(0), cst("Group.one"))),
    )?;
    add_axiom(
        env,
        "Subgroup.mul_mem",
        vec![],
        mk_subgroup_forall2(|_depth| {
            let a = bvar(1);
            let b = bvar(0);
            let s = bvar(2);
            arrow(
                app2(cst("Subgroup.mem"), s.clone(), a.clone()),
                arrow(
                    app2(cst("Subgroup.mem"), s.clone(), b.clone()),
                    app2(cst("Subgroup.mem"), s, app2(cst("Group.mul"), a, b)),
                ),
            )
        }),
    )?;
    add_axiom(
        env,
        "Subgroup.inv_mem",
        vec![],
        mk_subgroup_forall1(|_depth| {
            let a = bvar(0);
            let s = bvar(1);
            arrow(
                app2(cst("Subgroup.mem"), s.clone(), a.clone()),
                app2(cst("Subgroup.mem"), s, app(cst("Group.inv"), a)),
            )
        }),
    )?;
    add_axiom(
        env,
        "Subgroup.trivial",
        vec![],
        pi(
            BinderInfo::Implicit,
            "G",
            type0(),
            pi(
                BinderInfo::InstImplicit,
                "_inst",
                app(cst("Group"), bvar(0)),
                app(cst("Subgroup"), bvar(1)),
            ),
        ),
    )?;
    add_axiom(
        env,
        "Subgroup.whole",
        vec![],
        pi(
            BinderInfo::Implicit,
            "G",
            type0(),
            pi(
                BinderInfo::InstImplicit,
                "_inst",
                app(cst("Group"), bvar(0)),
                app(cst("Subgroup"), bvar(1)),
            ),
        ),
    )?;
    add_axiom(
        env,
        "Subgroup.inter",
        vec![],
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
                    "S1",
                    app(cst("Subgroup"), bvar(1)),
                    pi(
                        BinderInfo::Default,
                        "S2",
                        app(cst("Subgroup"), bvar(2)),
                        app(cst("Subgroup"), bvar(3)),
                    ),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "Subgroup.le",
        vec![],
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
                    "S1",
                    app(cst("Subgroup"), bvar(1)),
                    pi(
                        BinderInfo::Default,
                        "S2",
                        app(cst("Subgroup"), bvar(2)),
                        prop(),
                    ),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "GroupHom",
        vec![],
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
                        type1(),
                    ),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "GroupHom.toFun",
        vec![],
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
                            pi(BinderInfo::Default, "x", bvar(4), bvar(3)),
                        ),
                    ),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "GroupHom.map_mul",
        vec![],
        mk_hom_forall3(|_depth| {
            let h = bvar(5);
            let f = bvar(2);
            let a = bvar(1);
            let b = bvar(0);
            mk_eq(
                h,
                app2(
                    cst("GroupHom.toFun"),
                    f.clone(),
                    app2(cst("Group.mul"), a.clone(), b.clone()),
                ),
                app2(
                    cst("Group.mul"),
                    app2(cst("GroupHom.toFun"), f.clone(), a),
                    app2(cst("GroupHom.toFun"), f, b),
                ),
            )
        }),
    )?;
    add_axiom(
        env,
        "GroupHom.map_one",
        vec![],
        mk_hom_forall1(|_depth| {
            let h = bvar(4);
            let f = bvar(0);
            mk_eq(
                h,
                app2(cst("GroupHom.toFun"), f, cst("Group.one")),
                cst("Group.one"),
            )
        }),
    )?;
    add_axiom(
        env,
        "GroupHom.map_inv",
        vec![],
        mk_hom_forall2(|_depth| {
            let h = bvar(5);
            let f = bvar(1);
            let a = bvar(0);
            mk_eq(
                h,
                app2(
                    cst("GroupHom.toFun"),
                    f.clone(),
                    app(cst("Group.inv"), a.clone()),
                ),
                app(cst("Group.inv"), app2(cst("GroupHom.toFun"), f, a)),
            )
        }),
    )?;
    add_axiom(
        env,
        "GroupHom.id",
        vec![],
        pi(
            BinderInfo::Implicit,
            "G",
            type0(),
            pi(
                BinderInfo::InstImplicit,
                "_inst",
                app(cst("Group"), bvar(0)),
                app2(cst("GroupHom"), bvar(1), bvar(1)),
            ),
        ),
    )?;
    add_axiom(
        env,
        "GroupHom.comp",
        vec![],
        pi(
            BinderInfo::Implicit,
            "G",
            type0(),
            pi(
                BinderInfo::Implicit,
                "H",
                type0(),
                pi(
                    BinderInfo::Implicit,
                    "K",
                    type0(),
                    pi(
                        BinderInfo::InstImplicit,
                        "_instG",
                        app(cst("Group"), bvar(2)),
                        pi(
                            BinderInfo::InstImplicit,
                            "_instH",
                            app(cst("Group"), bvar(2)),
                            pi(
                                BinderInfo::InstImplicit,
                                "_instK",
                                app(cst("Group"), bvar(2)),
                                pi(
                                    BinderInfo::Default,
                                    "g",
                                    app2(cst("GroupHom"), bvar(3), bvar(2)),
                                    pi(
                                        BinderInfo::Default,
                                        "f",
                                        app2(cst("GroupHom"), bvar(5), bvar(4)),
                                        app2(cst("GroupHom"), bvar(6), bvar(4)),
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    )?;
    add_axiom(env, "GroupIso", vec![], mk_two_group_law(|_depth| type1()))?;
    add_axiom(
        env,
        "GroupIso.toGroupHom",
        vec![],
        mk_two_group_law(|_depth| {
            let g = bvar(3);
            let h = bvar(2);
            arrow(
                app2(cst("GroupIso"), g.clone(), h.clone()),
                app2(cst("GroupHom"), g, h),
            )
        }),
    )?;
    add_axiom(
        env,
        "GroupIso.symm",
        vec![],
        mk_two_group_law(|_depth| {
            let g = bvar(3);
            let h = bvar(2);
            arrow(
                app2(cst("GroupIso"), g.clone(), h.clone()),
                app2(cst("GroupIso"), h, g),
            )
        }),
    )?;
    add_axiom(
        env,
        "GroupIso.refl",
        vec![],
        pi(
            BinderInfo::Implicit,
            "G",
            type0(),
            pi(
                BinderInfo::InstImplicit,
                "_inst",
                app(cst("Group"), bvar(0)),
                app2(cst("GroupIso"), bvar(1), bvar(1)),
            ),
        ),
    )?;
    add_axiom(
        env,
        "GroupHom.ker",
        vec![],
        mk_two_group_law(|_depth| {
            let g = bvar(3);
            let h = bvar(2);
            arrow(app2(cst("GroupHom"), g.clone(), h), app(cst("Subgroup"), g))
        }),
    )?;
    add_axiom(
        env,
        "GroupHom.range",
        vec![],
        mk_two_group_law(|_depth| {
            let g = bvar(3);
            let h = bvar(2);
            arrow(app2(cst("GroupHom"), g, h.clone()), app(cst("Subgroup"), h))
        }),
    )?;
    add_axiom(
        env,
        "GroupHom.injective",
        vec![],
        mk_two_group_law(|_depth| {
            let g = bvar(3);
            let h = bvar(2);
            arrow(app2(cst("GroupHom"), g, h), prop())
        }),
    )?;
    add_axiom(
        env,
        "GroupHom.surjective",
        vec![],
        mk_two_group_law(|_depth| {
            let g = bvar(3);
            let h = bvar(2);
            arrow(app2(cst("GroupHom"), g, h), prop())
        }),
    )?;
    add_axiom(
        env,
        "GroupHom.ker_trivial_iff_injective",
        vec![],
        mk_hom_forall1(|_depth| {
            let f = bvar(0);
            mk_iff(
                mk_eq(
                    app(cst("Subgroup"), bvar(4)),
                    app(cst("GroupHom.ker"), f.clone()),
                    cst("Subgroup.trivial"),
                ),
                app(cst("GroupHom.injective"), f),
            )
        }),
    )?;
    add_axiom(
        env,
        "IsNormal",
        vec![],
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
                    prop(),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "IsNormal.conj_mem",
        vec![],
        mk_normal_forall2(|_depth| {
            let n_elem = bvar(1);
            let g_elem = bvar(0);
            let n_sub = bvar(2);
            arrow(
                app2(cst("Subgroup.mem"), n_sub.clone(), n_elem.clone()),
                app2(
                    cst("Subgroup.mem"),
                    n_sub,
                    app2(
                        cst("Group.mul"),
                        app2(cst("Group.mul"), g_elem.clone(), n_elem),
                        app(cst("Group.inv"), g_elem),
                    ),
                ),
            )
        }),
    )?;
    add_axiom(
        env,
        "GroupHom.ker_is_normal",
        vec![],
        mk_hom_forall1(|_depth| {
            let f = bvar(0);
            app(cst("IsNormal"), app(cst("GroupHom.ker"), f))
        }),
    )?;
    add_axiom(
        env,
        "LeftCoset",
        vec![],
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
                        "H",
                        app(cst("Subgroup"), bvar(2)),
                        type0(),
                    ),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "RightCoset",
        vec![],
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
                    "H",
                    app(cst("Subgroup"), bvar(1)),
                    pi(BinderInfo::Default, "a", bvar(2), type0()),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "LeftCoset.eq_iff",
        vec![],
        mk_subgroup_forall2(|_depth| {
            let h = bvar(2);
            let a = bvar(1);
            let b = bvar(0);
            mk_iff(
                mk_eq(
                    type0(),
                    app2(cst("LeftCoset"), a.clone(), h.clone()),
                    app2(cst("LeftCoset"), b.clone(), h.clone()),
                ),
                app2(
                    cst("Subgroup.mem"),
                    h,
                    app2(cst("Group.mul"), app(cst("Group.inv"), a), b),
                ),
            )
        }),
    )?;
    add_axiom(
        env,
        "QuotientGroup",
        vec![],
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
                        type0(),
                    ),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "QuotientGroup.mk",
        vec![],
        mk_normal_forall1(|_depth| app(cst("QuotientGroup"), bvar(2))),
    )?;
    add_axiom(
        env,
        "QuotientGroup.group",
        vec![],
        mk_normal_law(|_depth| app(cst("Group"), app(cst("QuotientGroup"), bvar(1)))),
    )?;
    add_axiom(
        env,
        "QuotientGroup.surjection",
        vec![],
        mk_normal_law(|_depth| {
            let g = bvar(3);
            app2(cst("GroupHom"), g, app(cst("QuotientGroup"), bvar(1)))
        }),
    )?;
    add_axiom(
        env,
        "GroupHom.first_isomorphism_theorem",
        vec![],
        mk_hom_forall1(|_depth| {
            let f = bvar(0);
            app2(
                cst("GroupIso"),
                app(cst("QuotientGroup"), app(cst("GroupHom.ker"), f.clone())),
                app(cst("GroupHom.range"), f),
            )
        }),
    )?;
    add_axiom(
        env,
        "GroupOrder",
        vec![],
        pi(
            BinderInfo::Implicit,
            "G",
            type0(),
            pi(
                BinderInfo::InstImplicit,
                "_inst",
                app(cst("Group"), bvar(0)),
                pi(BinderInfo::Default, "a", bvar(1), nat_ty()),
            ),
        ),
    )?;
    add_axiom(
        env,
        "GroupOrder.spec",
        vec![],
        mk_group_forall1(|_depth| {
            let g = bvar(2);
            let a = bvar(0);
            arrow(
                app2(
                    cst("Nat.lt"),
                    cst("Nat.zero"),
                    app(cst("GroupOrder"), a.clone()),
                ),
                mk_eq(
                    g,
                    app2(cst("Group.pow"), a.clone(), app(cst("GroupOrder"), a)),
                    cst("Group.one"),
                ),
            )
        }),
    )?;
    add_axiom(
        env,
        "Group.pow",
        vec![],
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
                    pi(BinderInfo::Default, "n", nat_ty(), bvar(3)),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "Group.pow_zero",
        vec![],
        mk_group_forall1(|depth| {
            let g = bvar(depth - 1);
            let a = bvar(0);
            mk_eq(
                g,
                app2(cst("Group.pow"), a, cst("Nat.zero")),
                cst("Group.one"),
            )
        }),
    )?;
    add_axiom(
        env,
        "Group.pow_succ",
        vec![],
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
                    pi(BinderInfo::Default, "n", nat_ty(), {
                        let g = bvar(3);
                        let a = bvar(1);
                        let n = bvar(0);
                        mk_eq(
                            g,
                            app2(cst("Group.pow"), a.clone(), app(cst("Nat.succ"), n.clone())),
                            app2(cst("Group.mul"), a.clone(), app2(cst("Group.pow"), a, n)),
                        )
                    }),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "IsCyclic",
        vec![],
        pi(
            BinderInfo::Implicit,
            "G",
            type0(),
            pi(
                BinderInfo::InstImplicit,
                "_inst",
                app(cst("Group"), bvar(0)),
                prop(),
            ),
        ),
    )?;
    add_axiom(
        env,
        "IsCyclic.generator",
        vec![],
        pi(
            BinderInfo::Implicit,
            "G",
            type0(),
            pi(
                BinderInfo::InstImplicit,
                "_inst",
                app(cst("Group"), bvar(0)),
                pi(BinderInfo::InstImplicit, "_cyc", cst("IsCyclic"), bvar(2)),
            ),
        ),
    )?;
    add_axiom(
        env,
        "IsCyclic.spec",
        vec![],
        pi(
            BinderInfo::Implicit,
            "G",
            type0(),
            pi(
                BinderInfo::InstImplicit,
                "_inst",
                app(cst("Group"), bvar(0)),
                pi(
                    BinderInfo::InstImplicit,
                    "_cyc",
                    cst("IsCyclic"),
                    pi(BinderInfo::Default, "a", bvar(2), prop()),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "IsCyclic.subgroup_cyclic",
        vec![],
        pi(
            BinderInfo::Implicit,
            "G",
            type0(),
            pi(
                BinderInfo::InstImplicit,
                "_inst",
                app(cst("Group"), bvar(0)),
                pi(
                    BinderInfo::InstImplicit,
                    "_cyc",
                    cst("IsCyclic"),
                    pi(
                        BinderInfo::Default,
                        "S",
                        app(cst("Subgroup"), bvar(2)),
                        prop(),
                    ),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "Fintype",
        vec![],
        pi(BinderInfo::Default, "G", type0(), prop()),
    )?;
    add_axiom(
        env,
        "Fintype.card",
        vec![],
        pi(
            BinderInfo::Implicit,
            "G",
            type0(),
            pi(
                BinderInfo::InstImplicit,
                "_fin",
                app(cst("Fintype"), bvar(0)),
                nat_ty(),
            ),
        ),
    )?;
    add_axiom(
        env,
        "Subgroup.index",
        vec![],
        pi(
            BinderInfo::Implicit,
            "G",
            type0(),
            pi(
                BinderInfo::InstImplicit,
                "_inst",
                app(cst("Group"), bvar(0)),
                pi(
                    BinderInfo::InstImplicit,
                    "_fin",
                    app(cst("Fintype"), bvar(1)),
                    pi(
                        BinderInfo::Default,
                        "H",
                        app(cst("Subgroup"), bvar(2)),
                        nat_ty(),
                    ),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "Subgroup.card",
        vec![],
        pi(
            BinderInfo::Implicit,
            "G",
            type0(),
            pi(
                BinderInfo::InstImplicit,
                "_inst",
                app(cst("Group"), bvar(0)),
                pi(
                    BinderInfo::InstImplicit,
                    "_fin",
                    app(cst("Fintype"), bvar(1)),
                    pi(
                        BinderInfo::Default,
                        "H",
                        app(cst("Subgroup"), bvar(2)),
                        nat_ty(),
                    ),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "Subgroup.lagrange",
        vec![],
        pi(
            BinderInfo::Implicit,
            "G",
            type0(),
            pi(
                BinderInfo::InstImplicit,
                "_inst",
                app(cst("Group"), bvar(0)),
                pi(
                    BinderInfo::InstImplicit,
                    "_fin",
                    app(cst("Fintype"), bvar(1)),
                    pi(
                        BinderInfo::Default,
                        "H",
                        app(cst("Subgroup"), bvar(2)),
                        mk_eq(
                            nat_ty(),
                            app(cst("Fintype.card"), bvar(3)),
                            app2(
                                cst("Nat.mul"),
                                app(cst("Subgroup.card"), bvar(0)),
                                app(cst("Subgroup.index"), bvar(0)),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "Group.order_dvd_card",
        vec![],
        pi(
            BinderInfo::Implicit,
            "G",
            type0(),
            pi(
                BinderInfo::InstImplicit,
                "_inst",
                app(cst("Group"), bvar(0)),
                pi(
                    BinderInfo::InstImplicit,
                    "_fin",
                    app(cst("Fintype"), bvar(1)),
                    pi(
                        BinderInfo::Default,
                        "a",
                        bvar(2),
                        app2(
                            cst("Nat.dvd"),
                            app(cst("GroupOrder"), bvar(0)),
                            app(cst("Fintype.card"), bvar(3)),
                        ),
                    ),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "Nat.dvd",
        vec![],
        arrow(nat_ty(), arrow(nat_ty(), prop())),
    )?;
    add_axiom(
        env,
        "Center",
        vec![],
        pi(
            BinderInfo::Implicit,
            "G",
            type0(),
            pi(
                BinderInfo::InstImplicit,
                "_inst",
                app(cst("Group"), bvar(0)),
                app(cst("Subgroup"), bvar(1)),
            ),
        ),
    )?;
    add_axiom(
        env,
        "Center.is_normal",
        vec![],
        mk_group_law(|_depth| app(cst("IsNormal"), cst("Center"))),
    )?;
    add_axiom(
        env,
        "Center.mem_iff",
        vec![],
        mk_group_forall1(|depth| {
            let g_ty = bvar(depth - 1);
            let a = bvar(0);
            mk_iff(
                app2(cst("Subgroup.mem"), cst("Center"), a.clone()),
                pi(
                    BinderInfo::Default,
                    "g",
                    g_ty.clone(),
                    mk_eq(
                        g_ty,
                        app2(cst("Group.mul"), a.clone(), bvar(0)),
                        app2(cst("Group.mul"), bvar(0), a),
                    ),
                ),
            )
        }),
    )?;
    add_axiom(
        env,
        "Centralizer",
        vec![],
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
                    app(cst("Subgroup"), bvar(2)),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "Conjugate",
        vec![],
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
                    "g",
                    bvar(1),
                    pi(BinderInfo::Default, "h", bvar(2), bvar(3)),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "Conjugate.def",
        vec![],
        mk_group_forall2(|depth| {
            let g_ty = bvar(depth - 1);
            let g = bvar(1);
            let h = bvar(0);
            mk_eq(
                g_ty,
                app2(cst("Conjugate"), g.clone(), h.clone()),
                app2(
                    cst("Group.mul"),
                    app2(cst("Group.mul"), g.clone(), h),
                    app(cst("Group.inv"), g),
                ),
            )
        }),
    )?;
    add_axiom(
        env,
        "IsConjugate",
        vec![],
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
                    pi(BinderInfo::Default, "b", bvar(2), prop()),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "IsConjugate.refl",
        vec![],
        mk_group_forall1(|_depth| {
            let a = bvar(0);
            app2(cst("IsConjugate"), a.clone(), a)
        }),
    )?;
    add_axiom(
        env,
        "IsConjugate.symm",
        vec![],
        mk_group_forall2(|_depth| {
            let a = bvar(1);
            let b = bvar(0);
            arrow(
                app2(cst("IsConjugate"), a.clone(), b.clone()),
                app2(cst("IsConjugate"), b, a),
            )
        }),
    )?;
    add_axiom(
        env,
        "IsConjugate.trans",
        vec![],
        mk_group_forall3(|_depth| {
            let a = bvar(2);
            let b = bvar(1);
            let c = bvar(0);
            arrow(
                app2(cst("IsConjugate"), a.clone(), b.clone()),
                arrow(
                    app2(cst("IsConjugate"), b, c.clone()),
                    app2(cst("IsConjugate"), a, c),
                ),
            )
        }),
    )?;
    add_axiom(
        env,
        "GroupAction",
        vec![],
        pi(
            BinderInfo::Default,
            "G",
            type0(),
            pi(BinderInfo::Default, "X", type0(), type1()),
        ),
    )?;
    add_axiom(
        env,
        "GroupAction.smul",
        vec![],
        pi(
            BinderInfo::Implicit,
            "G",
            type0(),
            pi(
                BinderInfo::Implicit,
                "X",
                type0(),
                pi(
                    BinderInfo::InstImplicit,
                    "_instG",
                    app(cst("Group"), bvar(1)),
                    pi(
                        BinderInfo::InstImplicit,
                        "_instA",
                        app2(cst("GroupAction"), bvar(2), bvar(1)),
                        pi(
                            BinderInfo::Default,
                            "g",
                            bvar(3),
                            pi(BinderInfo::Default, "x", bvar(3), bvar(4)),
                        ),
                    ),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "GroupAction.one_smul",
        vec![],
        pi(
            BinderInfo::Implicit,
            "G",
            type0(),
            pi(
                BinderInfo::Implicit,
                "X",
                type0(),
                pi(
                    BinderInfo::InstImplicit,
                    "_instG",
                    app(cst("Group"), bvar(1)),
                    pi(
                        BinderInfo::InstImplicit,
                        "_instA",
                        app2(cst("GroupAction"), bvar(2), bvar(1)),
                        pi(
                            BinderInfo::Default,
                            "x",
                            bvar(2),
                            mk_eq(
                                bvar(3),
                                app2(cst("GroupAction.smul"), cst("Group.one"), bvar(0)),
                                bvar(0),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "GroupAction.mul_smul",
        vec![],
        pi(
            BinderInfo::Implicit,
            "G",
            type0(),
            pi(
                BinderInfo::Implicit,
                "X",
                type0(),
                pi(
                    BinderInfo::InstImplicit,
                    "_instG",
                    app(cst("Group"), bvar(1)),
                    pi(
                        BinderInfo::InstImplicit,
                        "_instA",
                        app2(cst("GroupAction"), bvar(2), bvar(1)),
                        pi(
                            BinderInfo::Default,
                            "g",
                            bvar(3),
                            pi(
                                BinderInfo::Default,
                                "h",
                                bvar(4),
                                pi(
                                    BinderInfo::Default,
                                    "x",
                                    bvar(4),
                                    mk_eq(
                                        bvar(5),
                                        app2(
                                            cst("GroupAction.smul"),
                                            app2(cst("Group.mul"), bvar(2), bvar(1)),
                                            bvar(0),
                                        ),
                                        app2(
                                            cst("GroupAction.smul"),
                                            bvar(2),
                                            app2(cst("GroupAction.smul"), bvar(1), bvar(0)),
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
        "Orbit",
        vec![],
        pi(
            BinderInfo::Implicit,
            "G",
            type0(),
            pi(
                BinderInfo::Implicit,
                "X",
                type0(),
                pi(
                    BinderInfo::InstImplicit,
                    "_instG",
                    app(cst("Group"), bvar(1)),
                    pi(
                        BinderInfo::InstImplicit,
                        "_instA",
                        app2(cst("GroupAction"), bvar(2), bvar(1)),
                        pi(BinderInfo::Default, "x", bvar(2), type0()),
                    ),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "Stabilizer",
        vec![],
        pi(
            BinderInfo::Implicit,
            "G",
            type0(),
            pi(
                BinderInfo::Implicit,
                "X",
                type0(),
                pi(
                    BinderInfo::InstImplicit,
                    "_instG",
                    app(cst("Group"), bvar(1)),
                    pi(
                        BinderInfo::InstImplicit,
                        "_instA",
                        app2(cst("GroupAction"), bvar(2), bvar(1)),
                        pi(
                            BinderInfo::Default,
                            "x",
                            bvar(2),
                            app(cst("Subgroup"), bvar(4)),
                        ),
                    ),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "IsSimple",
        vec![],
        pi(
            BinderInfo::Implicit,
            "G",
            type0(),
            pi(
                BinderInfo::InstImplicit,
                "_inst",
                app(cst("Group"), bvar(0)),
                prop(),
            ),
        ),
    )?;
    add_axiom(
        env,
        "IsSimple.def",
        vec![],
        mk_group_law(|_depth| {
            mk_iff(
                cst("IsSimple"),
                pi(
                    BinderInfo::Default,
                    "N",
                    app(cst("Subgroup"), bvar(1)),
                    arrow(
                        app(cst("IsNormal"), bvar(0)),
                        mk_or(
                            mk_eq(
                                app(cst("Subgroup"), bvar(2)),
                                bvar(0),
                                cst("Subgroup.trivial"),
                            ),
                            mk_eq(
                                app(cst("Subgroup"), bvar(2)),
                                bvar(0),
                                cst("Subgroup.whole"),
                            ),
                        ),
                    ),
                ),
            )
        }),
    )?;
    add_axiom(
        env,
        "GroupProd",
        vec![],
        pi(
            BinderInfo::Default,
            "G",
            type0(),
            pi(BinderInfo::Default, "H", type0(), type0()),
        ),
    )?;
    add_axiom(
        env,
        "GroupProd.group",
        vec![],
        mk_two_group_law(|_depth| {
            let g = bvar(3);
            let h = bvar(2);
            app(cst("Group"), app2(cst("GroupProd"), g, h))
        }),
    )?;
    add_axiom(
        env,
        "GroupProd.fst",
        vec![],
        mk_two_group_law(|_depth| {
            let g = bvar(3);
            let h = bvar(2);
            app2(cst("GroupHom"), app2(cst("GroupProd"), g.clone(), h), g)
        }),
    )?;
    add_axiom(
        env,
        "GroupProd.snd",
        vec![],
        mk_two_group_law(|_depth| {
            let g = bvar(3);
            let h = bvar(2);
            app2(cst("GroupHom"), app2(cst("GroupProd"), g, h.clone()), h)
        }),
    )?;
    add_axiom(
        env,
        "IsSolvable",
        vec![],
        pi(
            BinderInfo::Implicit,
            "G",
            type0(),
            pi(
                BinderInfo::InstImplicit,
                "_inst",
                app(cst("Group"), bvar(0)),
                prop(),
            ),
        ),
    )?;
    add_axiom(
        env,
        "CommGroup.is_solvable",
        vec![],
        pi(
            BinderInfo::Implicit,
            "G",
            type0(),
            pi(
                BinderInfo::InstImplicit,
                "_inst",
                app(cst("CommGroup"), bvar(0)),
                cst("IsSolvable"),
            ),
        ),
    )?;
    add_axiom(
        env,
        "CommutatorSubgroup",
        vec![],
        pi(
            BinderInfo::Implicit,
            "G",
            type0(),
            pi(
                BinderInfo::InstImplicit,
                "_inst",
                app(cst("Group"), bvar(0)),
                app(cst("Subgroup"), bvar(1)),
            ),
        ),
    )?;
    add_axiom(
        env,
        "CommutatorSubgroup.is_normal",
        vec![],
        mk_group_law(|_depth| app(cst("IsNormal"), cst("CommutatorSubgroup"))),
    )?;
    add_axiom(
        env,
        "Abelianization.comm",
        vec![],
        mk_group_law(|_depth| {
            app(
                cst("CommGroup"),
                app(cst("QuotientGroup"), cst("CommutatorSubgroup")),
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
    fn test_build_group_theory_env() {
        let mut env = Environment::new();
        let _ = env.add(Declaration::Axiom {
            name: Name::str("Eq"),
            univ_params: vec![],
            ty: Expr::Pi(
                BinderInfo::Implicit,
                Name::str("_"),
                Box::new(type0()),
                Box::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("_"),
                    Box::new(Expr::BVar(0)),
                    Box::new(Expr::Pi(
                        BinderInfo::Default,
                        Name::str("_"),
                        Box::new(Expr::BVar(1)),
                        Box::new(prop()),
                    )),
                )),
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
            name: Name::str("Nat.zero"),
            univ_params: vec![],
            ty: nat_ty(),
        });
        let _ = env.add(Declaration::Axiom {
            name: Name::str("Nat.succ"),
            univ_params: vec![],
            ty: arrow(nat_ty(), nat_ty()),
        });
        let _ = env.add(Declaration::Axiom {
            name: Name::str("Nat.mul"),
            univ_params: vec![],
            ty: arrow(nat_ty(), arrow(nat_ty(), nat_ty())),
        });
        let _ = env.add(Declaration::Axiom {
            name: Name::str("Nat.lt"),
            univ_params: vec![],
            ty: arrow(nat_ty(), arrow(nat_ty(), prop())),
        });
        let result = build_group_theory_env(&mut env);
        assert!(
            result.is_ok(),
            "build_group_theory_env failed: {:?}",
            result
        );
    }
    #[test]
    fn test_group_expression_builders() {
        let a = cst("a");
        let b = cst("b");
        let mul = group_mul(a.clone(), b.clone());
        assert!(matches!(mul, Expr::App(_, _)));
        let inv = group_inv(a);
        assert!(matches!(inv, Expr::App(_, _)));
        let one = group_one();
        assert!(matches!(one, Expr::Const(_, _)));
        let lc = left_coset(b.clone(), cst("N"));
        assert!(matches!(lc, Expr::App(_, _)));
        let rc = right_coset(cst("N"), b);
        assert!(matches!(rc, Expr::App(_, _)));
    }
}
