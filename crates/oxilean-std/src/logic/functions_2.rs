//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::functions::*;
use super::types::*;

#[cfg(test)]
mod tests {
    use super::*;
    use oxilean_kernel::Literal;
    fn nat_ty() -> Expr {
        Expr::Const(Name::str("Nat"), vec![])
    }
    fn nat_one() -> Expr {
        Expr::Lit(Literal::Nat(1))
    }
    fn nat_two() -> Expr {
        Expr::Lit(Literal::Nat(2))
    }
    #[test]
    fn test_build_logic_env() {
        let mut env = Environment::new();
        assert!(build_logic_env(&mut env).is_ok());
        assert!(env.contains(&Name::str("True")));
        assert!(env.contains(&Name::str("True.intro")));
        assert!(env.contains(&Name::str("False")));
        assert!(env.contains(&Name::str("False.elim")));
        assert!(env.contains(&Name::str("Not")));
        assert!(env.contains(&Name::str("And")));
        assert!(env.contains(&Name::str("And.intro")));
        assert!(env.contains(&Name::str("And.left")));
        assert!(env.contains(&Name::str("And.right")));
        assert!(env.contains(&Name::str("Or")));
        assert!(env.contains(&Name::str("Or.inl")));
        assert!(env.contains(&Name::str("Or.inr")));
        assert!(env.contains(&Name::str("Or.elim")));
        assert!(env.contains(&Name::str("Iff")));
        assert!(env.contains(&Name::str("Iff.intro")));
        assert!(env.contains(&Name::str("Iff.mp")));
        assert!(env.contains(&Name::str("Iff.mpr")));
        assert!(env.contains(&Name::str("Exists")));
        assert!(env.contains(&Name::str("Exists.intro")));
        assert!(env.contains(&Name::str("Eq")));
        assert!(env.contains(&Name::str("Eq.refl")));
        assert!(env.contains(&Name::str("Eq.symm")));
        assert!(env.contains(&Name::str("Eq.trans")));
        assert!(env.contains(&Name::str("Eq.subst")));
        assert!(env.contains(&Name::str("Eq.rec")));
        assert!(env.contains(&Name::str("HEq")));
        assert!(env.contains(&Name::str("HEq.refl")));
        assert!(env.contains(&Name::str("cast")));
        assert!(env.contains(&Name::str("congr")));
        assert!(env.contains(&Name::str("congrArg")));
        assert!(env.contains(&Name::str("congrFun")));
        assert!(env.contains(&Name::str("funext")));
        assert!(env.contains(&Name::str("propext")));
        assert!(env.contains(&Name::str("Classical.em")));
        assert!(env.contains(&Name::str("Classical.choice")));
        assert!(env.contains(&Name::str("Nonempty")));
        assert!(env.contains(&Name::str("Nonempty.intro")));
        assert!(env.contains(&Name::str("absurd")));
        assert!(env.contains(&Name::str("not_not")));
        assert!(env.contains(&Name::str("And.comm")));
        assert!(env.contains(&Name::str("Or.comm")));
        assert!(env.contains(&Name::str("And.assoc")));
        assert!(env.contains(&Name::str("Or.assoc")));
        assert!(env.contains(&Name::str("iff_refl")));
        assert!(env.contains(&Name::str("iff_comm")));
        assert!(env.contains(&Name::str("iff_trans")));
    }
    #[test]
    fn test_mk_true() {
        let e = mk_true();
        assert!(matches!(e, Expr::Const(_, _)));
        if let Expr::Const(n, _) = &e {
            assert_eq!(*n, Name::str("True"));
        }
    }
    #[test]
    fn test_mk_false() {
        let e = mk_false();
        if let Expr::Const(n, _) = &e {
            assert_eq!(*n, Name::str("False"));
        } else {
            panic!("expected Const");
        }
    }
    #[test]
    fn test_mk_not() {
        let p = mk_true();
        let e = mk_not(p);
        assert!(matches!(e, Expr::App(_, _)));
    }
    #[test]
    fn test_mk_and() {
        let a = mk_true();
        let b = mk_false();
        let e = mk_and(a, b);
        assert!(matches!(e, Expr::App(_, _)));
    }
    #[test]
    fn test_mk_or() {
        let a = mk_true();
        let b = mk_false();
        let e = mk_or(a, b);
        assert!(matches!(e, Expr::App(_, _)));
    }
    #[test]
    fn test_mk_iff() {
        let a = mk_true();
        let b = mk_false();
        let e = mk_iff(a, b);
        assert!(matches!(e, Expr::App(_, _)));
    }
    #[test]
    fn test_mk_exists() {
        let alpha = nat_ty();
        let pred = lam(
            BinderInfo::Default,
            "x",
            nat_ty(),
            mk_eq(nat_ty(), bvar(0), nat_one()),
        );
        let e = mk_exists(alpha, pred);
        assert!(matches!(e, Expr::App(_, _)));
    }
    #[test]
    fn test_mk_eq() {
        let e = mk_eq(nat_ty(), nat_one(), nat_two());
        assert!(matches!(e, Expr::App(_, _)));
    }
    #[test]
    fn test_mk_heq() {
        let e = mk_heq(nat_ty(), nat_one(), nat_ty(), nat_two());
        assert!(matches!(e, Expr::App(_, _)));
    }
    #[test]
    fn test_mk_implies() {
        let a = mk_true();
        let b = mk_false();
        let e = mk_implies(a, b);
        assert!(matches!(e, Expr::Pi(BinderInfo::Default, _, _, _)));
    }
    #[test]
    fn test_mk_forall() {
        let e = mk_forall("x", nat_ty(), mk_eq(nat_ty(), bvar(0), bvar(0)));
        assert!(matches!(e, Expr::Pi(BinderInfo::Default, _, _, _)));
    }
    #[test]
    fn test_mk_prop() {
        let e = mk_prop();
        assert!(matches!(e, Expr::Sort(_)));
        if let Expr::Sort(l) = &e {
            assert_eq!(*l, Level::zero());
        }
    }
    #[test]
    fn test_mk_type() {
        let e = mk_type(Level::zero());
        assert!(matches!(e, Expr::Sort(_)));
        if let Expr::Sort(l) = &e {
            assert_eq!(*l, Level::succ(Level::zero()));
        }
    }
    #[test]
    fn test_true_intro() {
        let e = true_intro();
        if let Expr::Const(n, _) = &e {
            assert_eq!(*n, Name::str("True.intro"));
        } else {
            panic!("expected Const");
        }
    }
    #[test]
    fn test_false_elim() {
        let target = nat_ty();
        let proof = cst("some_false_proof");
        let e = false_elim(target, proof);
        assert!(matches!(e, Expr::App(_, _)));
    }
    #[test]
    fn test_and_intro() {
        let ha = cst("proof_a");
        let hb = cst("proof_b");
        let e = and_intro(ha, hb);
        assert!(matches!(e, Expr::App(_, _)));
    }
    #[test]
    fn test_and_left() {
        let hab = cst("proof_and");
        let e = and_left(hab);
        assert!(matches!(e, Expr::App(_, _)));
    }
    #[test]
    fn test_and_right() {
        let hab = cst("proof_and");
        let e = and_right(hab);
        assert!(matches!(e, Expr::App(_, _)));
    }
    #[test]
    fn test_or_inl() {
        let ha = cst("proof_a");
        let e = or_inl(ha);
        assert!(matches!(e, Expr::App(_, _)));
    }
    #[test]
    fn test_or_inr() {
        let hb = cst("proof_b");
        let e = or_inr(hb);
        assert!(matches!(e, Expr::App(_, _)));
    }
    #[test]
    fn test_eq_refl() {
        let a = nat_one();
        let e = eq_refl(a);
        assert!(matches!(e, Expr::App(_, _)));
    }
    #[test]
    fn test_eq_symm() {
        let h = cst("h_eq");
        let e = eq_symm(h);
        assert!(matches!(e, Expr::App(_, _)));
    }
    #[test]
    fn test_eq_trans() {
        let h1 = cst("h_ab");
        let h2 = cst("h_bc");
        let e = eq_trans(h1, h2);
        assert!(matches!(e, Expr::App(_, _)));
    }
    #[test]
    fn test_exists_intro() {
        let witness = nat_one();
        let proof = cst("proof_pred_witness");
        let e = exists_intro(witness, proof);
        assert!(matches!(e, Expr::App(_, _)));
    }
    #[test]
    fn test_mk_not_structure() {
        let p = cst("P");
        let e = mk_not(p);
        if let Expr::App(f, arg) = &e {
            assert!(matches!(f.as_ref(), Expr::Const(_, _)));
            assert!(matches!(arg.as_ref(), Expr::Const(_, _)));
        } else {
            panic!("expected App");
        }
    }
    #[test]
    fn test_mk_and_structure() {
        let a = cst("A");
        let b = cst("B");
        let e = mk_and(a, b);
        if let Expr::App(f, rhs) = &e {
            assert!(matches!(rhs.as_ref(), Expr::Const(_, _)));
            if let Expr::App(g, lhs) = f.as_ref() {
                assert!(matches!(g.as_ref(), Expr::Const(_, _)));
                assert!(matches!(lhs.as_ref(), Expr::Const(_, _)));
            } else {
                panic!("expected nested App");
            }
        } else {
            panic!("expected App");
        }
    }
    #[test]
    fn test_mk_implies_is_pi() {
        let a = cst("A");
        let b = cst("B");
        let e = mk_implies(a, b);
        if let Expr::Pi(bi, name, _, _) = &e {
            assert_eq!(*bi, BinderInfo::Default);
            assert_eq!(*name, Name::str("_"));
        } else {
            panic!("expected Pi");
        }
    }
    #[test]
    fn test_mk_forall_name() {
        let e = mk_forall("n", nat_ty(), mk_prop());
        if let Expr::Pi(_, name, _, _) = &e {
            assert_eq!(*name, Name::str("n"));
        } else {
            panic!("expected Pi");
        }
    }
    #[test]
    fn test_mk_type_level_param() {
        let u = Level::Param(Name::str("u"));
        let e = mk_type(u.clone());
        if let Expr::Sort(l) = &e {
            assert_eq!(*l, Level::succ(u));
        } else {
            panic!("expected Sort");
        }
    }
    #[test]
    fn test_and_intro_structure() {
        let ha = cst("ha");
        let hb = cst("hb");
        let e = and_intro(ha, hb);
        if let Expr::App(f, arg_b) = &e {
            assert!(matches!(arg_b.as_ref(), Expr::Const(_, _)));
            if let Expr::App(g, arg_a) = f.as_ref() {
                assert!(matches!(arg_a.as_ref(), Expr::Const(_, _)));
                if let Expr::Const(n, _) = g.as_ref() {
                    assert_eq!(*n, Name::str("And.intro"));
                }
            }
        } else {
            panic!("expected App");
        }
    }
    #[test]
    fn test_eq_trans_structure() {
        let h1 = cst("h1");
        let h2 = cst("h2");
        let e = eq_trans(h1, h2);
        if let Expr::App(f, _) = &e {
            if let Expr::App(g, _) = f.as_ref() {
                if let Expr::Const(n, _) = g.as_ref() {
                    assert_eq!(*n, Name::str("Eq.trans"));
                }
            }
        } else {
            panic!("expected App");
        }
    }
    #[test]
    fn test_false_elim_structure() {
        let ty = nat_ty();
        let pf = cst("false_pf");
        let e = false_elim(ty, pf);
        if let Expr::App(f, _) = &e {
            if let Expr::App(g, _) = f.as_ref() {
                if let Expr::Const(n, _) = g.as_ref() {
                    assert_eq!(*n, Name::str("False.elim"));
                }
            }
        } else {
            panic!("expected App");
        }
    }
    #[test]
    fn test_exists_intro_structure() {
        let w = nat_one();
        let pf = cst("pf");
        let e = exists_intro(w, pf);
        if let Expr::App(f, _) = &e {
            if let Expr::App(g, _) = f.as_ref() {
                if let Expr::Const(n, _) = g.as_ref() {
                    assert_eq!(*n, Name::str("Exists.intro"));
                }
            }
        } else {
            panic!("expected App");
        }
    }
    #[test]
    fn test_mk_eq_structure() {
        let e = mk_eq(nat_ty(), nat_one(), nat_two());
        if let Expr::App(f, rhs) = &e {
            assert!(matches!(rhs.as_ref(), Expr::Lit(Literal::Nat(2))));
            if let Expr::App(g, lhs) = f.as_ref() {
                assert!(matches!(lhs.as_ref(), Expr::Lit(Literal::Nat(1))));
                if let Expr::App(h, ty) = g.as_ref() {
                    assert!(matches!(ty.as_ref(), Expr::Const(_, _)));
                    if let Expr::Const(n, _) = h.as_ref() {
                        assert_eq!(*n, Name::str("Eq"));
                    }
                }
            }
        } else {
            panic!("expected App");
        }
    }
    #[test]
    fn test_mk_heq_structure() {
        let e = mk_heq(nat_ty(), nat_one(), nat_ty(), nat_two());
        if let Expr::App(_, b_expr) = &e {
            assert!(matches!(b_expr.as_ref(), Expr::Lit(Literal::Nat(2))));
        } else {
            panic!("expected App");
        }
    }
    #[test]
    fn test_mk_iff_structure() {
        let a = cst("P");
        let b = cst("Q");
        let e = mk_iff(a, b);
        if let Expr::App(f, rhs) = &e {
            if let Expr::Const(n, _) = rhs.as_ref() {
                assert_eq!(*n, Name::str("Q"));
            }
            if let Expr::App(g, lhs) = f.as_ref() {
                if let Expr::Const(n, _) = lhs.as_ref() {
                    assert_eq!(*n, Name::str("P"));
                }
                if let Expr::Const(n, _) = g.as_ref() {
                    assert_eq!(*n, Name::str("Iff"));
                }
            }
        } else {
            panic!("expected App");
        }
    }
    #[test]
    fn test_mk_or_structure() {
        let a = cst("P");
        let b = cst("Q");
        let e = mk_or(a, b);
        if let Expr::App(f, _) = &e {
            if let Expr::App(g, _) = f.as_ref() {
                if let Expr::Const(n, _) = g.as_ref() {
                    assert_eq!(*n, Name::str("Or"));
                }
            }
        } else {
            panic!("expected App");
        }
    }
    #[test]
    fn test_mk_exists_structure() {
        let alpha = nat_ty();
        let pred = cst("is_even");
        let e = mk_exists(alpha, pred);
        if let Expr::App(f, p) = &e {
            if let Expr::Const(n, _) = p.as_ref() {
                assert_eq!(*n, Name::str("is_even"));
            }
            if let Expr::App(g, _) = f.as_ref() {
                if let Expr::Const(n, _) = g.as_ref() {
                    assert_eq!(*n, Name::str("Exists"));
                }
            }
        } else {
            panic!("expected App");
        }
    }
    #[test]
    fn test_dne_present() {
        let mut env = Environment::new();
        assert!(build_logic_env(&mut env).is_ok());
        assert!(env.contains(&Name::str("dne")));
    }
    #[test]
    fn test_contrapositive_present() {
        let mut env = Environment::new();
        assert!(build_logic_env(&mut env).is_ok());
        assert!(env.contains(&Name::str("contrapositive")));
    }
    #[test]
    fn test_by_cases_present() {
        let mut env = Environment::new();
        assert!(build_logic_env(&mut env).is_ok());
        assert!(env.contains(&Name::str("by_cases")));
    }
    #[test]
    fn test_not_and_present() {
        let mut env = Environment::new();
        assert!(build_logic_env(&mut env).is_ok());
        assert!(env.contains(&Name::str("not_and")));
    }
    #[test]
    fn test_not_or_present() {
        let mut env = Environment::new();
        assert!(build_logic_env(&mut env).is_ok());
        assert!(env.contains(&Name::str("not_or")));
    }
    #[test]
    fn test_not_implies_present() {
        let mut env = Environment::new();
        assert!(build_logic_env(&mut env).is_ok());
        assert!(env.contains(&Name::str("not_implies")));
    }
    #[test]
    fn test_forall_and_present() {
        let mut env = Environment::new();
        assert!(build_logic_env(&mut env).is_ok());
        assert!(env.contains(&Name::str("forall_and")));
    }
    #[test]
    fn test_exists_or_present() {
        let mut env = Environment::new();
        assert!(build_logic_env(&mut env).is_ok());
        assert!(env.contains(&Name::str("exists_or")));
    }
    #[test]
    fn test_forall_or_exists_not_present() {
        let mut env = Environment::new();
        assert!(build_logic_env(&mut env).is_ok());
        assert!(env.contains(&Name::str("forall_or_exists_not")));
    }
    #[test]
    fn test_decidable_present() {
        let mut env = Environment::new();
        assert!(build_logic_env(&mut env).is_ok());
        assert!(env.contains(&Name::str("Decidable")));
    }
    #[test]
    fn test_is_true_present() {
        let mut env = Environment::new();
        assert!(build_logic_env(&mut env).is_ok());
        assert!(env.contains(&Name::str("is_true")));
    }
    #[test]
    fn test_decidable_true_present() {
        let mut env = Environment::new();
        assert!(build_logic_env(&mut env).is_ok());
        assert!(env.contains(&Name::str("decidable_true")));
    }
    #[test]
    fn test_proof_by_contradiction_present() {
        let mut env = Environment::new();
        assert!(build_logic_env(&mut env).is_ok());
        assert!(env.contains(&Name::str("proof_by_contradiction")));
    }
    #[test]
    fn test_iff_of_true_present() {
        let mut env = Environment::new();
        assert!(build_logic_env(&mut env).is_ok());
        assert!(env.contains(&Name::str("iff_of_true")));
    }
    #[test]
    fn test_iff_of_false_present() {
        let mut env = Environment::new();
        assert!(build_logic_env(&mut env).is_ok());
        assert!(env.contains(&Name::str("iff_of_false")));
    }
    #[test]
    fn test_and_iff_left_present() {
        let mut env = Environment::new();
        assert!(build_logic_env(&mut env).is_ok());
        assert!(env.contains(&Name::str("and_iff_left")));
    }
    #[test]
    fn test_and_iff_right_present() {
        let mut env = Environment::new();
        assert!(build_logic_env(&mut env).is_ok());
        assert!(env.contains(&Name::str("and_iff_right")));
    }
    #[test]
    fn test_or_iff_left_present() {
        let mut env = Environment::new();
        assert!(build_logic_env(&mut env).is_ok());
        assert!(env.contains(&Name::str("or_iff_left")));
    }
    #[test]
    fn test_or_iff_right_present() {
        let mut env = Environment::new();
        assert!(build_logic_env(&mut env).is_ok());
        assert!(env.contains(&Name::str("or_iff_right")));
    }
    #[test]
    fn test_dne_type_is_pi() {
        let mut env = Environment::new();
        assert!(build_logic_env(&mut env).is_ok());
        let decl = env
            .get(&Name::str("dne"))
            .expect("declaration 'dne' should exist in env");
        assert!(decl.ty().is_pi());
    }
    #[test]
    fn test_contrapositive_type_is_pi() {
        let mut env = Environment::new();
        assert!(build_logic_env(&mut env).is_ok());
        let decl = env
            .get(&Name::str("contrapositive"))
            .expect("declaration 'contrapositive' should exist in env");
        assert!(decl.ty().is_pi());
    }
    #[test]
    fn test_forall_and_type_is_pi() {
        let mut env = Environment::new();
        assert!(build_logic_env(&mut env).is_ok());
        let decl = env
            .get(&Name::str("forall_and"))
            .expect("declaration 'forall_and' should exist in env");
        assert!(decl.ty().is_pi());
    }
    #[test]
    fn test_all_new_logic_theorems_present() {
        let mut env = Environment::new();
        assert!(build_logic_env(&mut env).is_ok());
        let new_theorems = [
            "dne",
            "contrapositive",
            "by_cases",
            "not_and",
            "not_or",
            "not_implies",
            "forall_and",
            "exists_or",
            "forall_or_exists_not",
            "Decidable",
            "is_true",
            "decidable_true",
            "proof_by_contradiction",
            "iff_of_true",
            "iff_of_false",
            "and_iff_left",
            "and_iff_right",
            "or_iff_left",
            "or_iff_right",
        ];
        for name in &new_theorems {
            assert!(env.contains(&Name::str(*name)), "missing theorem: {}", name);
        }
    }
}
#[cfg(test)]
mod tests_logic_extended {
    use super::*;
    #[test]
    fn test_paraconsistent_lp() {
        let lp = ParaconsistentSystem::LP;
        assert_eq!(lp.num_truth_values(), 3);
        assert!(!lp.explosion_holds());
        assert!(lp.handles_liar_paradox());
    }
    #[test]
    fn test_fde_four_valued() {
        let fde = ParaconsistentSystem::FDE;
        assert_eq!(fde.num_truth_values(), 4);
    }
    #[test]
    fn test_kleene_and() {
        use ManyValuedTruth::*;
        assert_eq!(ManyValuedTruth::kleene_and(&True, &True), True);
        assert_eq!(ManyValuedTruth::kleene_and(&True, &False), False);
        assert_eq!(ManyValuedTruth::kleene_and(&Neither, &True), Neither);
    }
    #[test]
    fn test_kleene_not() {
        use ManyValuedTruth::*;
        assert_eq!(ManyValuedTruth::kleene_not(&True), False);
        assert_eq!(ManyValuedTruth::kleene_not(&False), True);
        assert_eq!(ManyValuedTruth::kleene_not(&Neither), Neither);
    }
    #[test]
    fn test_many_valued_designated() {
        use ManyValuedTruth::*;
        assert!(True.is_designated());
        assert!(Both.is_designated());
        assert!(!False.is_designated());
        assert!(!Neither.is_designated());
    }
    #[test]
    fn test_second_order_z2_categorical() {
        let z2 = SecondOrderLogic::z2();
        assert!(z2.is_categorical());
        assert!(!z2.is_complete());
    }
    #[test]
    fn test_aca0_interprets_pa() {
        let aca = SecondOrderLogic::aca0();
        assert!(aca.interprets_pa());
        assert!(!aca.is_categorical());
    }
    #[test]
    fn test_resolution_php() {
        let res = ProofComplexitySystem::resolution();
        let php = res.php_complexity();
        assert!(php.contains("Exponential"));
    }
    #[test]
    fn test_extended_frege_simulates() {
        let ef = ProofComplexitySystem::extended_frege();
        assert!(ef.has_p_simulations);
        assert!(ef.simulates_resolution);
    }
}
#[cfg(test)]
mod tests_logic_extra {
    use super::*;
    #[test]
    fn test_kripke_frame_s5() {
        let mut frame = KripkeFrame::new(vec!["w1", "w2"]);
        frame.add_access(0, 0);
        frame.add_access(1, 1);
        frame.add_access(0, 1);
        frame.add_access(1, 0);
        assert!(frame.is_reflexive());
        assert!(frame.is_symmetric());
        assert!(frame.is_transitive());
        assert_eq!(frame.modal_logic_name(), "S5 (equivalence relation)");
    }
    #[test]
    fn test_ltl_formula() {
        let p = LTLFormula::atom("p");
        assert!(!p.is_temporal());
        assert_eq!(p.depth(), 0);
        let gp = LTLFormula::safety(LTLFormula::atom("p"));
        assert!(gp.is_temporal());
        assert_eq!(gp.depth(), 1);
    }
    #[test]
    fn test_sequent_proof() {
        let ax = SequentProof::axiom("p");
        assert_eq!(ax.height(), 0);
        assert_eq!(ax.n_leaves(), 1);
        let proof = SequentProof::new(
            vec!["p", "p->q"],
            "q",
            "->E",
            vec![ax.clone(), SequentProof::axiom("p->q")],
        );
        assert_eq!(proof.height(), 1);
        assert_eq!(proof.n_leaves(), 2);
    }
}
