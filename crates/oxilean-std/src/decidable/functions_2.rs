//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;

fn dcs_ext_finset_membership(
    add: &mut impl FnMut(&str, oxilean_kernel::Expr) -> Result<(), String>,
) -> Result<(), String> {
    use oxilean_kernel::{BinderInfo as Bi, Expr, Level, Name};
    let cst = |s: &str| -> Expr { Expr::Const(Name::str(s), vec![]) };
    let app = |f: Expr, a: Expr| -> Expr { Expr::App(Box::new(f), Box::new(a)) };
    let arr = |a: Expr, b: Expr| -> Expr {
        Expr::Pi(Bi::Default, Name::Anonymous, Box::new(a), Box::new(b))
    };
    let type1 = || -> Expr { Expr::Sort(Level::succ(Level::zero())) };
    let dec_of = |p: Expr| -> Expr { app(cst("Decidable"), p) };
    add("Finset", arr(type1(), type1()))?;
    let finset_mem_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(Expr::Pi(
            Bi::InstImplicit,
            Name::str("inst"),
            Box::new(app(cst("DecidableEq"), Expr::BVar(0))),
            Box::new(arr(
                Expr::BVar(1),
                arr(app(cst("Finset"), Expr::BVar(2)), cst("Prop")),
            )),
        )),
    );
    add("Finset.mem", finset_mem_ty)?;
    let finset_dec_mem_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(Expr::Pi(
            Bi::InstImplicit,
            Name::str("inst"),
            Box::new(app(cst("DecidableEq"), Expr::BVar(0))),
            Box::new(arr(
                Expr::BVar(1),
                arr(
                    app(cst("Finset"), Expr::BVar(2)),
                    dec_of(app(app(cst("Finset.mem"), Expr::BVar(1)), Expr::BVar(0))),
                ),
            )),
        )),
    );
    add("Finset.decidableMem", finset_dec_mem_ty)?;
    let finset_dec_fa_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(arr(
            arr(Expr::BVar(0), cst("Prop")),
            arr(
                app(cst("Finset"), Expr::BVar(1)),
                dec_of(Expr::Pi(
                    Bi::Default,
                    Name::str("x"),
                    Box::new(Expr::BVar(1)),
                    Box::new(arr(
                        app(app(cst("Finset.mem"), Expr::BVar(0)), Expr::BVar(2)),
                        app(Expr::BVar(2), Expr::BVar(0)),
                    )),
                )),
            ),
        )),
    );
    add("Finset.decidableForall", finset_dec_fa_ty)?;
    let finset_dec_ex_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(arr(
            arr(Expr::BVar(0), cst("Prop")),
            arr(
                app(cst("Finset"), Expr::BVar(1)),
                dec_of(app(
                    app(cst("Exists"), Expr::BVar(1)),
                    app(
                        app(cst("And"), app(cst("Finset.mem"), Expr::BVar(0))),
                        app(Expr::BVar(1), Expr::BVar(0)),
                    ),
                )),
            ),
        )),
    );
    add("Finset.decidableExists", finset_dec_ex_ty)?;
    let finset_card_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(arr(app(cst("Finset"), Expr::BVar(0)), cst("Nat"))),
    );
    add("Finset.card", finset_card_ty)?;
    let finset_empty_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(app(cst("Finset"), Expr::BVar(0))),
    );
    add("Finset.empty", finset_empty_ty)?;
    let finset_insert_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(Expr::Pi(
            Bi::InstImplicit,
            Name::str("inst"),
            Box::new(app(cst("DecidableEq"), Expr::BVar(0))),
            Box::new(arr(
                Expr::BVar(1),
                arr(
                    app(cst("Finset"), Expr::BVar(2)),
                    app(cst("Finset"), Expr::BVar(3)),
                ),
            )),
        )),
    );
    add("Finset.insert", finset_insert_ty)?;
    Ok(())
}
fn dcs_ext_witness_extraction(
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
    let dec_of = |p: Expr| -> Expr { app(cst("Decidable"), p) };
    let extract_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(Expr::Pi(
            Bi::Implicit,
            Name::str("p"),
            Box::new(arr(Expr::BVar(0), prop())),
            Box::new(arr(
                app(
                    app(cst("Exists"), Expr::BVar(1)),
                    app(Expr::BVar(0), Expr::BVar(0)),
                ),
                app(
                    app(cst("Sigma"), Expr::BVar(1)),
                    app(Expr::BVar(0), Expr::BVar(0)),
                ),
            )),
        )),
    );
    add("DecWitness.extract", extract_ty)?;
    let search_ty = Expr::Pi(
        Bi::Default,
        Name::str("p"),
        Box::new(arr(cst("Nat"), prop())),
        Box::new(arr(
            app(
                app(cst("Exists"), cst("Nat")),
                app(Expr::BVar(1), Expr::BVar(0)),
            ),
            app(
                app(cst("Sigma"), cst("Nat")),
                app(Expr::BVar(1), Expr::BVar(0)),
            ),
        )),
    );
    add("DecWitness.search", search_ty)?;
    let min_witness_ty = Expr::Pi(
        Bi::Default,
        Name::str("p"),
        Box::new(arr(cst("Nat"), prop())),
        Box::new(arr(
            app(
                app(cst("Exists"), cst("Nat")),
                app(Expr::BVar(1), Expr::BVar(0)),
            ),
            app(
                app(cst("Sigma"), cst("Nat")),
                app(
                    app(cst("And"), app(Expr::BVar(1), Expr::BVar(0))),
                    Expr::Pi(
                        Bi::Default,
                        Name::str("m"),
                        Box::new(cst("Nat")),
                        Box::new(arr(
                            app(app(cst("Nat.lt"), Expr::BVar(0)), Expr::BVar(2)),
                            arr(app(Expr::BVar(3), Expr::BVar(0)), cst("False")),
                        )),
                    ),
                ),
            ),
        )),
    );
    add("DecWitness.min_witness", min_witness_ty)?;
    let exists_find_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(arr(
            arr(Expr::BVar(0), prop()),
            arr(
                app(cst("Finset"), Expr::BVar(1)),
                app(
                    app(
                        cst("Iff"),
                        app(
                            app(cst("Exists"), Expr::BVar(1)),
                            app(
                                app(cst("And"), app(cst("Finset.mem"), Expr::BVar(0))),
                                app(Expr::BVar(1), Expr::BVar(0)),
                            ),
                        ),
                    ),
                    app(
                        app(cst("Exists"), Expr::BVar(1)),
                        app(
                            app(cst("And"), app(cst("Finset.mem"), Expr::BVar(0))),
                            app(Expr::BVar(1), Expr::BVar(0)),
                        ),
                    ),
                ),
            ),
        )),
    );
    add("Decidable.exists_iff_find", exists_find_ty)?;
    let _ = dec_of(cst("P"));
    Ok(())
}
fn dcs_ext_predicates_membership(
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
    let dec_of = |p: Expr| -> Expr { app(cst("Decidable"), p) };
    let dec_mem_list_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(Expr::Pi(
            Bi::InstImplicit,
            Name::str("inst"),
            Box::new(app(cst("DecidableEq"), Expr::BVar(0))),
            Box::new(arr(
                Expr::BVar(1),
                arr(
                    app(cst("List"), Expr::BVar(2)),
                    dec_of(app(app(cst("List.mem"), Expr::BVar(1)), Expr::BVar(0))),
                ),
            )),
        )),
    );
    add("Decidable.mem_list", dec_mem_list_ty)?;
    let beq_iff_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(arr(
            Expr::BVar(0),
            arr(
                Expr::BVar(1),
                app(
                    app(
                        cst("Iff"),
                        app(
                            app(
                                cst("Eq"),
                                app(app(cst("BEq.beq"), Expr::BVar(1)), Expr::BVar(0)),
                            ),
                            cst("Bool.true"),
                        ),
                    ),
                    app(app(cst("Eq"), Expr::BVar(1)), Expr::BVar(0)),
                ),
            ),
        )),
    );
    add("Decidable.beq_iff_eq", beq_iff_ty)?;
    let decide_pred_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(arr(
            arr(Expr::BVar(0), prop()),
            arr(
                arr(Expr::BVar(1), cst("Bool")),
                Expr::Pi(
                    Bi::Default,
                    Name::str("h"),
                    Box::new(Expr::Pi(
                        Bi::Default,
                        Name::str("x"),
                        Box::new(Expr::BVar(2)),
                        Box::new(app(
                            app(
                                cst("Iff"),
                                app(
                                    app(cst("Eq"), app(Expr::BVar(2), Expr::BVar(0))),
                                    cst("Bool.true"),
                                ),
                            ),
                            app(Expr::BVar(3), Expr::BVar(0)),
                        )),
                    )),
                    Box::new(Expr::Pi(
                        Bi::Default,
                        Name::str("x"),
                        Box::new(Expr::BVar(3)),
                        Box::new(dec_of(app(Expr::BVar(4), Expr::BVar(0)))),
                    )),
                ),
            ),
        )),
    );
    add("Decidable.decide_pred", decide_pred_ty)?;
    Ok(())
}
/// Register all extended Decidable axioms into the kernel environment.
pub fn register_decidable_extended_axioms(env: &mut oxilean_kernel::Environment) {
    use oxilean_kernel::{BinderInfo as Bi, Declaration, Expr, Level, Name};
    let mut add = |name: &str, ty: Expr| {
        let _ = env.add(Declaration::Axiom {
            name: Name::str(name),
            univ_params: vec![],
            ty,
        });
    };
    let cst = |s: &str| -> Expr { Expr::Const(Name::str(s), vec![]) };
    let _app = |f: Expr, a: Expr| -> Expr { Expr::App(Box::new(f), Box::new(a)) };
    let arr = |a: Expr, b: Expr| -> Expr {
        Expr::Pi(Bi::Default, Name::Anonymous, Box::new(a), Box::new(b))
    };
    let type1 = || -> Expr { Expr::Sort(Level::succ(Level::zero())) };
    let prop = || -> Expr { Expr::Sort(Level::zero()) };
    add(
        "Computes",
        arr(cst("Nat"), arr(arr(cst("Nat"), cst("Nat")), prop())),
    );
    add("Ordering", type1());
    add(
        "PresburgerFormula.holds",
        arr(cst("PresburgerFormula"), prop()),
    );
    add("CNFFormula.satisfiable", arr(cst("CNFFormula"), prop()));
    add(
        "Sigma",
        Expr::Pi(
            Bi::Implicit,
            Name::str("α"),
            Box::new(type1()),
            Box::new(arr(arr(Expr::BVar(0), prop()), type1())),
        ),
    );
    add(
        "BEq.beq",
        Expr::Pi(
            Bi::Implicit,
            Name::str("α"),
            Box::new(type1()),
            Box::new(arr(Expr::BVar(0), arr(Expr::BVar(1), cst("Bool")))),
        ),
    );
    let _ = dcs_ext_decidable_typeclass(&mut |n, t| {
        add(n, t);
        Ok(())
    });
    let _ = dcs_ext_logical_connective_closure(&mut |n, t| {
        add(n, t);
        Ok(())
    });
    let _ = dcs_ext_decidable_eq_basic(&mut |n, t| {
        add(n, t);
        Ok(())
    });
    let _ = dcs_ext_decidable_eq_compound(&mut |n, t| {
        add(n, t);
        Ok(())
    });
    let _ = dcs_ext_linear_ordering(&mut |n, t| {
        add(n, t);
        Ok(())
    });
    let _ = dcs_ext_bounded_quantifiers(&mut |n, t| {
        add(n, t);
        Ok(())
    });
    let _ = dcs_ext_lem_boolean_reflection(&mut |n, t| {
        add(n, t);
        Ok(())
    });
    let _ = dcs_ext_semi_decidability(&mut |n, t| {
        add(n, t);
        Ok(())
    });
    let _ = dcs_ext_undecidability_halting(&mut |n, t| {
        add(n, t);
        Ok(())
    });
    let _ = dcs_ext_presburger_arithmetic(&mut |n, t| {
        add(n, t);
        Ok(())
    });
    let _ = dcs_ext_dpll_procedure(&mut |n, t| {
        add(n, t);
        Ok(())
    });
    let _ = dcs_ext_constructive_markov(&mut |n, t| {
        add(n, t);
        Ok(())
    });
    let _ = dcs_ext_finset_membership(&mut |n, t| {
        add(n, t);
        Ok(())
    });
    let _ = dcs_ext_witness_extraction(&mut |n, t| {
        add(n, t);
        Ok(())
    });
    let _ = dcs_ext_predicates_membership(&mut |n, t| {
        add(n, t);
        Ok(())
    });
}
