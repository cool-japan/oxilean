//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;

fn hs_ext_cardinality_axioms(
    add: &mut impl FnMut(&str, oxilean_kernel::Expr) -> Result<(), String>,
) -> Result<(), String> {
    use oxilean_kernel::{BinderInfo as Bi, Expr, Level, Name};
    let cst = |s: &str| -> Expr { Expr::Const(Name::str(s), vec![]) };
    let app = |f: Expr, a: Expr| -> Expr { Expr::App(Box::new(f), Box::new(a)) };
    let arr = |a: Expr, b: Expr| -> Expr {
        Expr::Pi(Bi::Default, Name::Anonymous, Box::new(a), Box::new(b))
    };
    let type1 = || -> Expr { Expr::Sort(Level::succ(Level::zero())) };
    let nat_ty = || -> Expr { cst("Nat") };
    let hashset_of = |ty: Expr| -> Expr { app(cst("HashSet"), ty) };
    let card_empty_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(app(
            app(cst("Eq"), app(cst("HashSet.size"), cst("HashSet.empty"))),
            cst("Nat.zero"),
        )),
    );
    add("HashSet.card_empty", card_empty_ty)?;
    let card_singleton_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(arr(
            Expr::BVar(0),
            app(
                app(
                    cst("Eq"),
                    app(
                        cst("HashSet.size"),
                        app(
                            app(cst("HashSet.insert"), Expr::BVar(0)),
                            cst("HashSet.empty"),
                        ),
                    ),
                ),
                app(cst("Nat.succ"), cst("Nat.zero")),
            ),
        )),
    );
    add("HashSet.card_singleton", card_singleton_ty)?;
    let inclusion_exclusion_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(arr(
            hashset_of(Expr::BVar(0)),
            arr(
                hashset_of(Expr::BVar(1)),
                app(
                    app(
                        cst("Eq"),
                        app(
                            cst("HashSet.size"),
                            app(app(cst("HashSet.union"), Expr::BVar(0)), Expr::BVar(1)),
                        ),
                    ),
                    app(
                        app(
                            cst("Nat.sub"),
                            app(
                                app(cst("Nat.add"), app(cst("HashSet.size"), Expr::BVar(0))),
                                app(cst("HashSet.size"), Expr::BVar(1)),
                            ),
                        ),
                        app(
                            cst("HashSet.size"),
                            app(app(cst("HashSet.inter"), Expr::BVar(0)), Expr::BVar(1)),
                        ),
                    ),
                ),
            ),
        )),
    );
    add(
        "HashSet.card_union_inclusion_exclusion",
        inclusion_exclusion_ty,
    )?;
    let _ = nat_ty();
    let disjoint_card_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(arr(
            hashset_of(Expr::BVar(0)),
            arr(
                hashset_of(Expr::BVar(1)),
                arr(
                    app(
                        app(
                            cst("Eq"),
                            app(app(cst("HashSet.inter"), Expr::BVar(0)), Expr::BVar(1)),
                        ),
                        cst("HashSet.empty"),
                    ),
                    app(
                        app(
                            cst("Eq"),
                            app(
                                cst("HashSet.size"),
                                app(app(cst("HashSet.union"), Expr::BVar(0)), Expr::BVar(1)),
                            ),
                        ),
                        app(
                            app(cst("Nat.add"), app(cst("HashSet.size"), Expr::BVar(0))),
                            app(cst("HashSet.size"), Expr::BVar(1)),
                        ),
                    ),
                ),
            ),
        )),
    );
    add("HashSet.card_union_disjoint", disjoint_card_ty)?;
    Ok(())
}
fn hs_ext_boolean_algebra(
    add: &mut impl FnMut(&str, oxilean_kernel::Expr) -> Result<(), String>,
) -> Result<(), String> {
    use oxilean_kernel::{BinderInfo as Bi, Expr, Level, Name};
    let cst = |s: &str| -> Expr { Expr::Const(Name::str(s), vec![]) };
    let app = |f: Expr, a: Expr| -> Expr { Expr::App(Box::new(f), Box::new(a)) };
    let arr = |a: Expr, b: Expr| -> Expr {
        Expr::Pi(Bi::Default, Name::Anonymous, Box::new(a), Box::new(b))
    };
    let type1 = || -> Expr { Expr::Sort(Level::succ(Level::zero())) };
    let hashset_of = |ty: Expr| -> Expr { app(cst("HashSet"), ty) };
    let compl_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(arr(
            hashset_of(Expr::BVar(0)),
            arr(hashset_of(Expr::BVar(1)), hashset_of(Expr::BVar(2))),
        )),
    );
    add("HashSet.compl", compl_ty)?;
    let demorgan1_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(arr(
            hashset_of(Expr::BVar(0)),
            arr(
                hashset_of(Expr::BVar(1)),
                arr(
                    hashset_of(Expr::BVar(2)),
                    app(
                        app(
                            cst("Eq"),
                            app(
                                app(
                                    cst("HashSet.compl"),
                                    app(app(cst("HashSet.union"), Expr::BVar(0)), Expr::BVar(1)),
                                ),
                                Expr::BVar(2),
                            ),
                        ),
                        app(
                            app(
                                cst("HashSet.inter"),
                                app(app(cst("HashSet.compl"), Expr::BVar(0)), Expr::BVar(2)),
                            ),
                            app(app(cst("HashSet.compl"), Expr::BVar(1)), Expr::BVar(2)),
                        ),
                    ),
                ),
            ),
        )),
    );
    add("HashSet.demorgan_compl_union", demorgan1_ty)?;
    let demorgan2_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(arr(
            hashset_of(Expr::BVar(0)),
            arr(
                hashset_of(Expr::BVar(1)),
                arr(
                    hashset_of(Expr::BVar(2)),
                    app(
                        app(
                            cst("Eq"),
                            app(
                                app(
                                    cst("HashSet.compl"),
                                    app(app(cst("HashSet.inter"), Expr::BVar(0)), Expr::BVar(1)),
                                ),
                                Expr::BVar(2),
                            ),
                        ),
                        app(
                            app(
                                cst("HashSet.union"),
                                app(app(cst("HashSet.compl"), Expr::BVar(0)), Expr::BVar(2)),
                            ),
                            app(app(cst("HashSet.compl"), Expr::BVar(1)), Expr::BVar(2)),
                        ),
                    ),
                ),
            ),
        )),
    );
    add("HashSet.demorgan_compl_inter", demorgan2_ty)?;
    let compl_compl_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(arr(
            hashset_of(Expr::BVar(0)),
            arr(
                hashset_of(Expr::BVar(1)),
                app(
                    app(
                        cst("Eq"),
                        app(
                            app(
                                cst("HashSet.compl"),
                                app(app(cst("HashSet.compl"), Expr::BVar(0)), Expr::BVar(1)),
                            ),
                            Expr::BVar(1),
                        ),
                    ),
                    Expr::BVar(0),
                ),
            ),
        )),
    );
    add("HashSet.compl_compl", compl_compl_ty)?;
    let union_compl_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(arr(
            hashset_of(Expr::BVar(0)),
            arr(
                hashset_of(Expr::BVar(1)),
                app(
                    app(
                        cst("Eq"),
                        app(
                            app(cst("HashSet.union"), Expr::BVar(0)),
                            app(app(cst("HashSet.compl"), Expr::BVar(0)), Expr::BVar(1)),
                        ),
                    ),
                    Expr::BVar(1),
                ),
            ),
        )),
    );
    add("HashSet.union_compl_eq_univ", union_compl_ty)?;
    let inter_compl_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(arr(
            hashset_of(Expr::BVar(0)),
            arr(
                hashset_of(Expr::BVar(1)),
                app(
                    app(
                        cst("Eq"),
                        app(
                            app(cst("HashSet.inter"), Expr::BVar(0)),
                            app(app(cst("HashSet.compl"), Expr::BVar(0)), Expr::BVar(1)),
                        ),
                    ),
                    cst("HashSet.empty"),
                ),
            ),
        )),
    );
    add("HashSet.inter_compl_eq_empty", inter_compl_ty)?;
    Ok(())
}
fn hs_ext_list_conversion(
    add: &mut impl FnMut(&str, oxilean_kernel::Expr) -> Result<(), String>,
) -> Result<(), String> {
    use oxilean_kernel::{BinderInfo as Bi, Expr, Level, Name};
    let cst = |s: &str| -> Expr { Expr::Const(Name::str(s), vec![]) };
    let app = |f: Expr, a: Expr| -> Expr { Expr::App(Box::new(f), Box::new(a)) };
    let arr = |a: Expr, b: Expr| -> Expr {
        Expr::Pi(Bi::Default, Name::Anonymous, Box::new(a), Box::new(b))
    };
    let type1 = || -> Expr { Expr::Sort(Level::succ(Level::zero())) };
    let list_of = |ty: Expr| -> Expr { app(cst("List"), ty) };
    let hashset_of = |ty: Expr| -> Expr { app(cst("HashSet"), ty) };
    let to_list_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(arr(hashset_of(Expr::BVar(0)), list_of(Expr::BVar(1)))),
    );
    add("HashSet.toList", to_list_ty)?;
    let from_list_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(arr(list_of(Expr::BVar(0)), hashset_of(Expr::BVar(1)))),
    );
    add("HashSet.fromList", from_list_ty)?;
    let from_to_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(arr(
            hashset_of(Expr::BVar(0)),
            app(
                app(
                    cst("Eq"),
                    app(
                        cst("HashSet.fromList"),
                        app(cst("HashSet.toList"), Expr::BVar(0)),
                    ),
                ),
                Expr::BVar(0),
            ),
        )),
    );
    add("HashSet.fromList_toList", from_to_ty)?;
    let mem_from_list_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(arr(
            Expr::BVar(0),
            arr(
                list_of(Expr::BVar(1)),
                app(
                    app(
                        cst("Iff"),
                        app(
                            app(cst("HashSet.mem"), Expr::BVar(0)),
                            app(cst("HashSet.fromList"), Expr::BVar(1)),
                        ),
                    ),
                    app(app(cst("List.mem"), Expr::BVar(0)), Expr::BVar(1)),
                ),
            ),
        )),
    );
    add("HashSet.mem_fromList", mem_from_list_ty)?;
    let card_length_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(arr(
            hashset_of(Expr::BVar(0)),
            app(
                app(cst("Eq"), app(cst("HashSet.size"), Expr::BVar(0))),
                app(
                    cst("List.length"),
                    app(cst("HashSet.toList"), Expr::BVar(0)),
                ),
            ),
        )),
    );
    add("HashSet.card_eq_length_toList", card_length_ty)?;
    Ok(())
}
fn hs_ext_extensionality(
    add: &mut impl FnMut(&str, oxilean_kernel::Expr) -> Result<(), String>,
) -> Result<(), String> {
    use oxilean_kernel::{BinderInfo as Bi, Expr, Level, Name};
    let cst = |s: &str| -> Expr { Expr::Const(Name::str(s), vec![]) };
    let app = |f: Expr, a: Expr| -> Expr { Expr::App(Box::new(f), Box::new(a)) };
    let arr = |a: Expr, b: Expr| -> Expr {
        Expr::Pi(Bi::Default, Name::Anonymous, Box::new(a), Box::new(b))
    };
    let type1 = || -> Expr { Expr::Sort(Level::succ(Level::zero())) };
    let prop = || -> Expr { Expr::Sort(Level::zero()) };
    let hashset_of = |ty: Expr| -> Expr { app(cst("HashSet"), ty) };
    let ext_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(arr(
            hashset_of(Expr::BVar(0)),
            arr(
                hashset_of(Expr::BVar(1)),
                arr(
                    Expr::Pi(
                        Bi::Default,
                        Name::str("x"),
                        Box::new(Expr::BVar(0)),
                        Box::new(app(
                            app(
                                cst("Iff"),
                                app(app(cst("HashSet.mem"), Expr::BVar(0)), Expr::BVar(3)),
                            ),
                            app(app(cst("HashSet.mem"), Expr::BVar(0)), Expr::BVar(2)),
                        )),
                    ),
                    app(app(cst("Eq"), Expr::BVar(0)), Expr::BVar(1)),
                ),
            ),
        )),
    );
    add("HashSet.ext", ext_ty)?;
    let ext_iff_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(arr(
            hashset_of(Expr::BVar(0)),
            arr(
                hashset_of(Expr::BVar(1)),
                app(
                    app(
                        cst("Iff"),
                        app(app(cst("Eq"), Expr::BVar(0)), Expr::BVar(1)),
                    ),
                    Expr::Pi(
                        Bi::Default,
                        Name::str("x"),
                        Box::new(Expr::BVar(0)),
                        Box::new(app(
                            app(
                                cst("Iff"),
                                app(app(cst("HashSet.mem"), Expr::BVar(0)), Expr::BVar(3)),
                            ),
                            app(app(cst("HashSet.mem"), Expr::BVar(0)), Expr::BVar(2)),
                        )),
                    ),
                ),
            ),
        )),
    );
    add("HashSet.ext_iff", ext_iff_ty)?;
    let _ = prop();
    Ok(())
}
fn hs_ext_map_filter_axioms(
    add: &mut impl FnMut(&str, oxilean_kernel::Expr) -> Result<(), String>,
) -> Result<(), String> {
    use oxilean_kernel::{BinderInfo as Bi, Expr, Level, Name};
    let cst = |s: &str| -> Expr { Expr::Const(Name::str(s), vec![]) };
    let app = |f: Expr, a: Expr| -> Expr { Expr::App(Box::new(f), Box::new(a)) };
    let arr = |a: Expr, b: Expr| -> Expr {
        Expr::Pi(Bi::Default, Name::Anonymous, Box::new(a), Box::new(b))
    };
    let type1 = || -> Expr { Expr::Sort(Level::succ(Level::zero())) };
    let bool_ty = || -> Expr { cst("Bool") };
    let hashset_of = |ty: Expr| -> Expr { app(cst("HashSet"), ty) };
    let image_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(Expr::Pi(
            Bi::Implicit,
            Name::str("β"),
            Box::new(type1()),
            Box::new(arr(
                arr(Expr::BVar(1), Expr::BVar(0)),
                arr(hashset_of(Expr::BVar(2)), hashset_of(Expr::BVar(1))),
            )),
        )),
    );
    add("HashSet.image", image_ty)?;
    let mem_image_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(Expr::Pi(
            Bi::Implicit,
            Name::str("β"),
            Box::new(type1()),
            Box::new(arr(
                arr(Expr::BVar(1), Expr::BVar(0)),
                arr(
                    hashset_of(Expr::BVar(2)),
                    arr(
                        Expr::BVar(1),
                        app(
                            app(
                                cst("Iff"),
                                app(
                                    app(cst("HashSet.mem"), Expr::BVar(0)),
                                    app(app(cst("HashSet.image"), Expr::BVar(1)), Expr::BVar(2)),
                                ),
                            ),
                            app(
                                app(cst("Exists"), Expr::BVar(3)),
                                app(app(cst("HashSet.mem"), Expr::BVar(0)), Expr::BVar(2)),
                            ),
                        ),
                    ),
                ),
            )),
        )),
    );
    add("HashSet.mem_image", mem_image_ty)?;
    let filter_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(arr(
            arr(Expr::BVar(0), bool_ty()),
            arr(hashset_of(Expr::BVar(1)), hashset_of(Expr::BVar(2))),
        )),
    );
    add("HashSet.filter", filter_ty)?;
    let mem_filter_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(arr(
            arr(Expr::BVar(0), bool_ty()),
            arr(
                hashset_of(Expr::BVar(1)),
                arr(
                    Expr::BVar(2),
                    app(
                        app(
                            cst("Iff"),
                            app(
                                app(cst("HashSet.mem"), Expr::BVar(0)),
                                app(app(cst("HashSet.filter"), Expr::BVar(1)), Expr::BVar(2)),
                            ),
                        ),
                        app(
                            app(
                                cst("And"),
                                app(app(cst("HashSet.mem"), Expr::BVar(0)), Expr::BVar(2)),
                            ),
                            app(
                                app(cst("Eq"), app(Expr::BVar(1), Expr::BVar(0))),
                                cst("Bool.true"),
                            ),
                        ),
                    ),
                ),
            ),
        )),
    );
    add("HashSet.mem_filter", mem_filter_ty)?;
    let filter_subset_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(arr(
            arr(Expr::BVar(0), bool_ty()),
            arr(
                hashset_of(Expr::BVar(1)),
                app(
                    app(
                        cst("HashSet.subset"),
                        app(app(cst("HashSet.filter"), Expr::BVar(0)), Expr::BVar(1)),
                    ),
                    Expr::BVar(1),
                ),
            ),
        )),
    );
    add("HashSet.filter_subset", filter_subset_ty)?;
    Ok(())
}
fn hs_ext_partition_fold(
    add: &mut impl FnMut(&str, oxilean_kernel::Expr) -> Result<(), String>,
) -> Result<(), String> {
    use oxilean_kernel::{BinderInfo as Bi, Expr, Level, Name};
    let cst = |s: &str| -> Expr { Expr::Const(Name::str(s), vec![]) };
    let app = |f: Expr, a: Expr| -> Expr { Expr::App(Box::new(f), Box::new(a)) };
    let arr = |a: Expr, b: Expr| -> Expr {
        Expr::Pi(Bi::Default, Name::Anonymous, Box::new(a), Box::new(b))
    };
    let type1 = || -> Expr { Expr::Sort(Level::succ(Level::zero())) };
    let bool_ty = || -> Expr { cst("Bool") };
    let hashset_of = |ty: Expr| -> Expr { app(cst("HashSet"), ty) };
    let partition_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(arr(
            arr(Expr::BVar(0), bool_ty()),
            arr(
                hashset_of(Expr::BVar(1)),
                app(
                    app(cst("Prod"), hashset_of(Expr::BVar(2))),
                    hashset_of(Expr::BVar(3)),
                ),
            ),
        )),
    );
    add("HashSet.partition", partition_ty)?;
    let partition_union_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(arr(
            arr(Expr::BVar(0), bool_ty()),
            arr(
                hashset_of(Expr::BVar(1)),
                app(
                    app(
                        cst("Eq"),
                        app(
                            app(
                                cst("HashSet.union"),
                                app(
                                    cst("Prod.fst"),
                                    app(
                                        app(cst("HashSet.partition"), Expr::BVar(0)),
                                        Expr::BVar(1),
                                    ),
                                ),
                            ),
                            app(
                                cst("Prod.snd"),
                                app(app(cst("HashSet.partition"), Expr::BVar(0)), Expr::BVar(1)),
                            ),
                        ),
                    ),
                    Expr::BVar(1),
                ),
            ),
        )),
    );
    add("HashSet.partition_fst_union_snd", partition_union_ty)?;
    let fold_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(Expr::Pi(
            Bi::Implicit,
            Name::str("β"),
            Box::new(type1()),
            Box::new(arr(
                arr(Expr::BVar(0), arr(Expr::BVar(2), Expr::BVar(1))),
                arr(Expr::BVar(1), arr(hashset_of(Expr::BVar(3)), Expr::BVar(2))),
            )),
        )),
    );
    add("HashSet.fold", fold_ty)?;
    let fold_empty_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(Expr::Pi(
            Bi::Implicit,
            Name::str("β"),
            Box::new(type1()),
            Box::new(arr(
                arr(Expr::BVar(0), arr(Expr::BVar(2), Expr::BVar(1))),
                arr(
                    Expr::BVar(1),
                    app(
                        app(
                            cst("Eq"),
                            app(
                                app(app(cst("HashSet.fold"), Expr::BVar(1)), Expr::BVar(0)),
                                cst("HashSet.empty"),
                            ),
                        ),
                        Expr::BVar(0),
                    ),
                ),
            )),
        )),
    );
    add("HashSet.fold_empty", fold_empty_ty)?;
    Ok(())
}
fn hs_ext_disjointness_covering(
    add: &mut impl FnMut(&str, oxilean_kernel::Expr) -> Result<(), String>,
) -> Result<(), String> {
    use oxilean_kernel::{BinderInfo as Bi, Expr, Level, Name};
    let cst = |s: &str| -> Expr { Expr::Const(Name::str(s), vec![]) };
    let app = |f: Expr, a: Expr| -> Expr { Expr::App(Box::new(f), Box::new(a)) };
    let arr = |a: Expr, b: Expr| -> Expr {
        Expr::Pi(Bi::Default, Name::Anonymous, Box::new(a), Box::new(b))
    };
    let type1 = || -> Expr { Expr::Sort(Level::succ(Level::zero())) };
    let hashset_of = |ty: Expr| -> Expr { app(cst("HashSet"), ty) };
    let disjoint_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(arr(
            hashset_of(Expr::BVar(0)),
            arr(hashset_of(Expr::BVar(1)), cst("Prop")),
        )),
    );
    add("HashSet.Disjoint", disjoint_ty)?;
    let disjoint_iff_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(arr(
            hashset_of(Expr::BVar(0)),
            arr(
                hashset_of(Expr::BVar(1)),
                app(
                    app(
                        cst("Iff"),
                        app(app(cst("HashSet.Disjoint"), Expr::BVar(0)), Expr::BVar(1)),
                    ),
                    app(
                        app(
                            cst("Eq"),
                            app(app(cst("HashSet.inter"), Expr::BVar(0)), Expr::BVar(1)),
                        ),
                        cst("HashSet.empty"),
                    ),
                ),
            ),
        )),
    );
    add("HashSet.disjoint_iff", disjoint_iff_ty)?;
    let disjoint_symm_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(arr(
            hashset_of(Expr::BVar(0)),
            arr(
                hashset_of(Expr::BVar(1)),
                arr(
                    app(app(cst("HashSet.Disjoint"), Expr::BVar(0)), Expr::BVar(1)),
                    app(app(cst("HashSet.Disjoint"), Expr::BVar(1)), Expr::BVar(0)),
                ),
            ),
        )),
    );
    add("HashSet.disjoint_symm", disjoint_symm_ty)?;
    let covering_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(arr(
            hashset_of(hashset_of(Expr::BVar(0))),
            arr(hashset_of(Expr::BVar(1)), cst("Prop")),
        )),
    );
    add("HashSet.IsCovering", covering_ty)?;
    let disj_empty_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(arr(
            hashset_of(Expr::BVar(0)),
            app(
                app(cst("HashSet.Disjoint"), cst("HashSet.empty")),
                Expr::BVar(0),
            ),
        )),
    );
    add("HashSet.disjoint_empty_left", disj_empty_ty)?;
    Ok(())
}
fn hs_ext_lattice_axioms(
    add: &mut impl FnMut(&str, oxilean_kernel::Expr) -> Result<(), String>,
) -> Result<(), String> {
    use oxilean_kernel::{BinderInfo as Bi, Expr, Level, Name};
    let cst = |s: &str| -> Expr { Expr::Const(Name::str(s), vec![]) };
    let app = |f: Expr, a: Expr| -> Expr { Expr::App(Box::new(f), Box::new(a)) };
    let arr = |a: Expr, b: Expr| -> Expr {
        Expr::Pi(Bi::Default, Name::Anonymous, Box::new(a), Box::new(b))
    };
    let type1 = || -> Expr { Expr::Sort(Level::succ(Level::zero())) };
    let hashset_of = |ty: Expr| -> Expr { app(cst("HashSet"), ty) };
    let join_union_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(arr(
            hashset_of(Expr::BVar(0)),
            arr(
                hashset_of(Expr::BVar(1)),
                app(
                    app(
                        cst("Eq"),
                        app(app(cst("HashSet.join"), Expr::BVar(0)), Expr::BVar(1)),
                    ),
                    app(app(cst("HashSet.union"), Expr::BVar(0)), Expr::BVar(1)),
                ),
            ),
        )),
    );
    add("HashSet.join_eq_union", join_union_ty)?;
    let meet_inter_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(arr(
            hashset_of(Expr::BVar(0)),
            arr(
                hashset_of(Expr::BVar(1)),
                app(
                    app(
                        cst("Eq"),
                        app(app(cst("HashSet.meet"), Expr::BVar(0)), Expr::BVar(1)),
                    ),
                    app(app(cst("HashSet.inter"), Expr::BVar(0)), Expr::BVar(1)),
                ),
            ),
        )),
    );
    add("HashSet.meet_eq_inter", meet_inter_ty)?;
    let le_join_left_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(arr(
            hashset_of(Expr::BVar(0)),
            arr(
                hashset_of(Expr::BVar(1)),
                app(
                    app(cst("HashSet.subset"), Expr::BVar(0)),
                    app(app(cst("HashSet.union"), Expr::BVar(0)), Expr::BVar(1)),
                ),
            ),
        )),
    );
    add("HashSet.le_join_left", le_join_left_ty)?;
    let meet_le_left_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(arr(
            hashset_of(Expr::BVar(0)),
            arr(
                hashset_of(Expr::BVar(1)),
                app(
                    app(
                        cst("HashSet.subset"),
                        app(app(cst("HashSet.inter"), Expr::BVar(0)), Expr::BVar(1)),
                    ),
                    Expr::BVar(0),
                ),
            ),
        )),
    );
    add("HashSet.meet_le_left", meet_le_left_ty)?;
    let absorb_union_inter_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(arr(
            hashset_of(Expr::BVar(0)),
            arr(
                hashset_of(Expr::BVar(1)),
                app(
                    app(
                        cst("Eq"),
                        app(
                            app(cst("HashSet.union"), Expr::BVar(0)),
                            app(app(cst("HashSet.inter"), Expr::BVar(0)), Expr::BVar(1)),
                        ),
                    ),
                    Expr::BVar(0),
                ),
            ),
        )),
    );
    add("HashSet.absorption_union_inter", absorb_union_inter_ty)?;
    let absorb_inter_union_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(arr(
            hashset_of(Expr::BVar(0)),
            arr(
                hashset_of(Expr::BVar(1)),
                app(
                    app(
                        cst("Eq"),
                        app(
                            app(cst("HashSet.inter"), Expr::BVar(0)),
                            app(app(cst("HashSet.union"), Expr::BVar(0)), Expr::BVar(1)),
                        ),
                    ),
                    Expr::BVar(0),
                ),
            ),
        )),
    );
    add("HashSet.absorption_inter_union", absorb_inter_union_ty)?;
    Ok(())
}
fn hs_ext_cartesian_product(
    add: &mut impl FnMut(&str, oxilean_kernel::Expr) -> Result<(), String>,
) -> Result<(), String> {
    use oxilean_kernel::{BinderInfo as Bi, Expr, Level, Name};
    let cst = |s: &str| -> Expr { Expr::Const(Name::str(s), vec![]) };
    let app = |f: Expr, a: Expr| -> Expr { Expr::App(Box::new(f), Box::new(a)) };
    let arr = |a: Expr, b: Expr| -> Expr {
        Expr::Pi(Bi::Default, Name::Anonymous, Box::new(a), Box::new(b))
    };
    let type1 = || -> Expr { Expr::Sort(Level::succ(Level::zero())) };
    let hashset_of = |ty: Expr| -> Expr { app(cst("HashSet"), ty) };
    let product_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(Expr::Pi(
            Bi::Implicit,
            Name::str("β"),
            Box::new(type1()),
            Box::new(arr(
                hashset_of(Expr::BVar(1)),
                arr(
                    hashset_of(Expr::BVar(1)),
                    hashset_of(app(app(cst("Prod"), Expr::BVar(3)), Expr::BVar(2))),
                ),
            )),
        )),
    );
    add("HashSet.product", product_ty)?;
    let card_product_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(Expr::Pi(
            Bi::Implicit,
            Name::str("β"),
            Box::new(type1()),
            Box::new(arr(
                hashset_of(Expr::BVar(1)),
                arr(
                    hashset_of(Expr::BVar(1)),
                    app(
                        app(
                            cst("Eq"),
                            app(
                                cst("HashSet.size"),
                                app(app(cst("HashSet.product"), Expr::BVar(0)), Expr::BVar(1)),
                            ),
                        ),
                        app(
                            app(cst("Nat.mul"), app(cst("HashSet.size"), Expr::BVar(0))),
                            app(cst("HashSet.size"), Expr::BVar(1)),
                        ),
                    ),
                ),
            )),
        )),
    );
    add("HashSet.card_product", card_product_ty)?;
    let mem_product_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(Expr::Pi(
            Bi::Implicit,
            Name::str("β"),
            Box::new(type1()),
            Box::new(arr(
                hashset_of(Expr::BVar(1)),
                arr(
                    hashset_of(Expr::BVar(1)),
                    arr(
                        app(app(cst("Prod"), Expr::BVar(3)), Expr::BVar(2)),
                        app(
                            app(
                                cst("Iff"),
                                app(
                                    app(cst("HashSet.mem"), Expr::BVar(0)),
                                    app(app(cst("HashSet.product"), Expr::BVar(1)), Expr::BVar(2)),
                                ),
                            ),
                            app(
                                app(
                                    cst("And"),
                                    app(
                                        app(
                                            cst("HashSet.mem"),
                                            app(cst("Prod.fst"), Expr::BVar(0)),
                                        ),
                                        Expr::BVar(1),
                                    ),
                                ),
                                app(
                                    app(cst("HashSet.mem"), app(cst("Prod.snd"), Expr::BVar(0))),
                                    Expr::BVar(2),
                                ),
                            ),
                        ),
                    ),
                ),
            )),
        )),
    );
    add("HashSet.mem_product", mem_product_ty)?;
    Ok(())
}
fn hs_ext_bloom_filter_axioms(
    add: &mut impl FnMut(&str, oxilean_kernel::Expr) -> Result<(), String>,
) -> Result<(), String> {
    use oxilean_kernel::{BinderInfo as Bi, Expr, Level, Name};
    let cst = |s: &str| -> Expr { Expr::Const(Name::str(s), vec![]) };
    let app = |f: Expr, a: Expr| -> Expr { Expr::App(Box::new(f), Box::new(a)) };
    let arr = |a: Expr, b: Expr| -> Expr {
        Expr::Pi(Bi::Default, Name::Anonymous, Box::new(a), Box::new(b))
    };
    let type1 = || -> Expr { Expr::Sort(Level::succ(Level::zero())) };
    let bool_ty = || -> Expr { cst("Bool") };
    let nat_ty = || -> Expr { cst("Nat") };
    let hashset_of = |ty: Expr| -> Expr { app(cst("HashSet"), ty) };
    add("BloomFilter", arr(type1(), type1()))?;
    let bf_insert_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(arr(
            Expr::BVar(0),
            arr(
                app(cst("BloomFilter"), Expr::BVar(1)),
                app(cst("BloomFilter"), Expr::BVar(2)),
            ),
        )),
    );
    add("BloomFilter.insert", bf_insert_ty)?;
    let bf_query_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(arr(
            app(cst("BloomFilter"), Expr::BVar(0)),
            arr(Expr::BVar(1), bool_ty()),
        )),
    );
    add("BloomFilter.query", bf_query_ty)?;
    let no_fn_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(arr(
            app(cst("BloomFilter"), Expr::BVar(0)),
            arr(
                hashset_of(Expr::BVar(1)),
                Expr::Pi(
                    Bi::Default,
                    Name::str("x"),
                    Box::new(Expr::BVar(1)),
                    Box::new(arr(
                        app(app(cst("HashSet.mem"), Expr::BVar(0)), Expr::BVar(2)),
                        app(
                            app(
                                cst("Eq"),
                                app(app(cst("BloomFilter.query"), Expr::BVar(3)), Expr::BVar(0)),
                            ),
                            cst("Bool.true"),
                        ),
                    )),
                ),
            ),
        )),
    );
    add("BloomFilter.no_false_negatives", no_fn_ty)?;
    let fpr_ty = arr(nat_ty(), arr(nat_ty(), arr(nat_ty(), cst("Prop"))));
    add("BloomFilter.false_positive_rate_bound", fpr_ty)?;
    Ok(())
}
/// Register all extended HashSet axioms into the kernel environment.
pub fn register_hashset_extended_axioms(env: &mut oxilean_kernel::Environment) {
    use oxilean_kernel::{BinderInfo as Bi, Declaration, Expr, Level, Name};
    let mut add = |name: &str, ty: Expr| {
        let _ = env.add(Declaration::Axiom {
            name: Name::str(name),
            univ_params: vec![],
            ty,
        });
    };
    let cst = |s: &str| -> Expr { Expr::Const(Name::str(s), vec![]) };
    let app = |f: Expr, a: Expr| -> Expr { Expr::App(Box::new(f), Box::new(a)) };
    let arr = |a: Expr, b: Expr| -> Expr {
        Expr::Pi(Bi::Default, Name::Anonymous, Box::new(a), Box::new(b))
    };
    let type1 = || -> Expr { Expr::Sort(Level::succ(Level::zero())) };
    let hashset_of = |ty: Expr| -> Expr { app(cst("HashSet"), ty) };
    add(
        "HashSet.subset",
        Expr::Pi(
            Bi::Implicit,
            Name::str("α"),
            Box::new(type1()),
            Box::new(arr(
                hashset_of(Expr::BVar(0)),
                arr(hashset_of(Expr::BVar(1)), cst("Prop")),
            )),
        ),
    );
    add(
        "HashSet.union",
        Expr::Pi(
            Bi::Implicit,
            Name::str("α"),
            Box::new(type1()),
            Box::new(arr(
                hashset_of(Expr::BVar(0)),
                arr(hashset_of(Expr::BVar(1)), hashset_of(Expr::BVar(2))),
            )),
        ),
    );
    add(
        "HashSet.inter",
        Expr::Pi(
            Bi::Implicit,
            Name::str("α"),
            Box::new(type1()),
            Box::new(arr(
                hashset_of(Expr::BVar(0)),
                arr(hashset_of(Expr::BVar(1)), hashset_of(Expr::BVar(2))),
            )),
        ),
    );
    add(
        "HashSet.sdiff",
        Expr::Pi(
            Bi::Implicit,
            Name::str("α"),
            Box::new(type1()),
            Box::new(arr(
                hashset_of(Expr::BVar(0)),
                arr(hashset_of(Expr::BVar(1)), hashset_of(Expr::BVar(2))),
            )),
        ),
    );
    add(
        "HashSet.size",
        Expr::Pi(
            Bi::Implicit,
            Name::str("α"),
            Box::new(type1()),
            Box::new(arr(hashset_of(Expr::BVar(0)), cst("Nat"))),
        ),
    );
    add(
        "HashSet.join",
        Expr::Pi(
            Bi::Implicit,
            Name::str("α"),
            Box::new(type1()),
            Box::new(arr(
                hashset_of(Expr::BVar(0)),
                arr(hashset_of(Expr::BVar(1)), hashset_of(Expr::BVar(2))),
            )),
        ),
    );
    add(
        "HashSet.meet",
        Expr::Pi(
            Bi::Implicit,
            Name::str("α"),
            Box::new(type1()),
            Box::new(arr(
                hashset_of(Expr::BVar(0)),
                arr(hashset_of(Expr::BVar(1)), hashset_of(Expr::BVar(2))),
            )),
        ),
    );
    let _ = hs_ext_finite_set_type(&mut |n, t| {
        add(n, t);
        Ok(())
    });
    let _ = hs_ext_membership_axioms(&mut |n, t| {
        add(n, t);
        Ok(())
    });
    let _ = hs_ext_insert_delete_axioms(&mut |n, t| {
        add(n, t);
        Ok(())
    });
    let _ = hs_ext_union_laws(&mut |n, t| {
        add(n, t);
        Ok(())
    });
    let _ = hs_ext_intersection_laws(&mut |n, t| {
        add(n, t);
        Ok(())
    });
    let _ = hs_ext_difference_laws(&mut |n, t| {
        add(n, t);
        Ok(())
    });
    let _ = hs_ext_subset_partial_order(&mut |n, t| {
        add(n, t);
        Ok(())
    });
    let _ = hs_ext_power_set_axioms(&mut |n, t| {
        add(n, t);
        Ok(())
    });
    let _ = hs_ext_cardinality_axioms(&mut |n, t| {
        add(n, t);
        Ok(())
    });
    let _ = hs_ext_boolean_algebra(&mut |n, t| {
        add(n, t);
        Ok(())
    });
    let _ = hs_ext_list_conversion(&mut |n, t| {
        add(n, t);
        Ok(())
    });
    let _ = hs_ext_extensionality(&mut |n, t| {
        add(n, t);
        Ok(())
    });
    let _ = hs_ext_map_filter_axioms(&mut |n, t| {
        add(n, t);
        Ok(())
    });
    let _ = hs_ext_partition_fold(&mut |n, t| {
        add(n, t);
        Ok(())
    });
    let _ = hs_ext_disjointness_covering(&mut |n, t| {
        add(n, t);
        Ok(())
    });
    let _ = hs_ext_lattice_axioms(&mut |n, t| {
        add(n, t);
        Ok(())
    });
    let _ = hs_ext_cartesian_product(&mut |n, t| {
        add(n, t);
        Ok(())
    });
    let _ = hs_ext_bloom_filter_axioms(&mut |n, t| {
        add(n, t);
        Ok(())
    });
}
