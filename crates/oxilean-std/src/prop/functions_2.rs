//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::functions::*;
use super::types::{KripkeModel, NnfFormula, Sequent, TruthTableChecker};

/// Type of `IPC.hypotheticalSyllogism`: (p → q) → (q → r) → (p → r).
/// `IPC.hypotheticalSyllogism : ∀ p q r : Prop, (p → q) → (q → r) → (p → r)`
#[allow(dead_code)]
pub fn axiom_hypothetical_syllogism_ty() -> Expr {
    Expr::Pi(
        BinderInfo::Default,
        Name::str("p"),
        Box::new(prop()),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("q"),
            Box::new(prop()),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("r"),
                Box::new(prop()),
                Box::new(prp_ext_arrow(
                    prp_ext_arrow(Expr::BVar(2), Expr::BVar(1)),
                    prp_ext_arrow(
                        prp_ext_arrow(Expr::BVar(1), Expr::BVar(0)),
                        prp_ext_arrow(Expr::BVar(2), Expr::BVar(0)),
                    ),
                )),
            )),
        )),
    )
}
/// Type of `PropLogic.conjunctiveNormalForm`: every proposition can be converted to CNF.
/// `PropLogic.conjunctiveNormalForm : Prop → Prop`
#[allow(dead_code)]
pub fn axiom_cnf_ty() -> Expr {
    prp_ext_arrow(prop(), prop())
}
/// Type of `PropLogic.disjunctiveNormalForm`: every proposition can be converted to DNF.
/// `PropLogic.disjunctiveNormalForm : Prop → Prop`
#[allow(dead_code)]
pub fn axiom_dnf_ty() -> Expr {
    prp_ext_arrow(prop(), prop())
}
/// Type of `PropLogic.resolution`: the resolution inference rule.
/// `PropLogic.resolution : ∀ p q r : Prop, (Or p q) → (Or (Not p) r) → (Or q r)`
#[allow(dead_code)]
pub fn axiom_resolution_ty() -> Expr {
    Expr::Pi(
        BinderInfo::Default,
        Name::str("p"),
        Box::new(prop()),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("q"),
            Box::new(prop()),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("r"),
                Box::new(prop()),
                Box::new(prp_ext_arrow(
                    Expr::App(
                        Box::new(Expr::App(
                            Box::new(Expr::Const(Name::str("Or"), vec![])),
                            Box::new(Expr::BVar(2)),
                        )),
                        Box::new(Expr::BVar(1)),
                    ),
                    prp_ext_arrow(
                        Expr::App(
                            Box::new(Expr::App(
                                Box::new(Expr::Const(Name::str("Or"), vec![])),
                                Box::new(Expr::App(
                                    Box::new(Expr::Const(Name::str("Not"), vec![])),
                                    Box::new(Expr::BVar(2)),
                                )),
                            )),
                            Box::new(Expr::BVar(0)),
                        ),
                        Expr::App(
                            Box::new(Expr::App(
                                Box::new(Expr::Const(Name::str("Or"), vec![])),
                                Box::new(Expr::BVar(1)),
                            )),
                            Box::new(Expr::BVar(0)),
                        ),
                    ),
                )),
            )),
        )),
    )
}
/// Register all extended propositional logic axioms in the environment.
#[allow(dead_code)]
pub fn register_prop_extended(env: &mut Environment) -> Result<(), String> {
    add_prereqs_if_missing(env)?;
    let modal_prereqs: &[(&str, Expr)] = &[
        ("Box", prp_ext_arrow(prop(), prop())),
        ("Diamond", prp_ext_arrow(prop(), prop())),
        (
            "KripkeFrame",
            Expr::Sort(Level::succ(Level::succ(Level::zero()))),
        ),
        ("BHK.proof", prp_ext_arrow(prop(), type1())),
        (
            "HeytingAlgebra",
            prp_ext_arrow(type1(), Expr::Sort(Level::succ(Level::succ(Level::zero())))),
        ),
        ("LTL.next", prp_ext_arrow(prop(), prop())),
        ("LTL.eventually", prp_ext_arrow(prop(), prop())),
        ("LTL.globally", prp_ext_arrow(prop(), prop())),
        ("Godel.gentzenTranslation", prp_ext_arrow(prop(), prop())),
        (
            "PropLogic.conjunctiveNormalForm",
            prp_ext_arrow(prop(), prop()),
        ),
        (
            "PropLogic.disjunctiveNormalForm",
            prp_ext_arrow(prop(), prop()),
        ),
    ];
    for (name, ty) in modal_prereqs {
        if !env.contains(&Name::str(*name)) {
            env.add(Declaration::Axiom {
                name: Name::str(*name),
                univ_params: vec![],
                ty: ty.clone(),
            })
            .map_err(|e| e.to_string())?;
        }
    }
    if !env.contains(&Name::str("KripkeFrame.worlds")) {
        env.add(Declaration::Axiom {
            name: Name::str("KripkeFrame.worlds"),
            univ_params: vec![],
            ty: axiom_kripke_frame_worlds_ty(),
        })
        .map_err(|e| e.to_string())?;
    }
    let axioms: &[(&str, Expr)] = &[
        ("ClassicalLogic.excludedMiddle", axiom_excluded_middle_ty()),
        (
            "ClassicalLogic.doubleNegationElim",
            axiom_double_negation_elim_ty(),
        ),
        ("ClassicalLogic.peirce", axiom_peirce_law_ty()),
        ("Completeness.propLogic", axiom_prop_logic_completeness_ty()),
        ("Lindstrom.theorem", axiom_lindstrom_theorem_ty()),
        ("Godel.completeness", axiom_godel_completeness_ty()),
        ("Craig.interpolation", axiom_craig_interpolation_ty()),
        ("Beth.definability", axiom_beth_definability_ty()),
        ("KripkeFrame.accessibility", axiom_kripke_accessibility_ty()),
        ("ModalLogic.necessitation", axiom_modal_necessitation_ty()),
        ("ModalLogic.K", axiom_modal_k_ty()),
        ("ModalLogic.T", axiom_modal_t_ty()),
        ("ModalLogic.S4", axiom_modal_s4_ty()),
        ("ModalLogic.S5", axiom_modal_s5_ty()),
        ("LTL.until", axiom_ltl_until_ty()),
        ("LTL.unfolding", axiom_ltl_unfolding_ty()),
        ("IntuitionisticLogic.exFalso", axiom_ex_falso_ty()),
        (
            "IntuitionisticLogic.noExcludedMiddle",
            axiom_no_excluded_middle_ty(),
        ),
        ("BHK.andIntro", axiom_bhk_and_intro_ty()),
        ("HeytingAlgebra.implication", axiom_heyting_implication_ty()),
        ("DeMorgan.classical1", axiom_de_morgan_classical1_ty()),
        ("DeMorgan.classical2", axiom_de_morgan_classical2_ty()),
        (
            "Godel.gentzenCorrectness",
            axiom_godel_gentzen_correctness_ty(),
        ),
        ("PropLogic.soundness", axiom_prop_soundness_ty()),
        ("PropLogic.consistencyLem", axiom_prop_consistency_ty()),
        ("IPC.andElimLeft", axiom_ipc_and_elim_left_ty()),
        ("IPC.andElimRight", axiom_ipc_and_elim_right_ty()),
        ("IPC.orIntroLeft", axiom_ipc_or_intro_left_ty()),
        ("IPC.orIntroRight", axiom_ipc_or_intro_right_ty()),
        ("IPC.modusPonens", axiom_modus_ponens_ty()),
        (
            "IPC.hypotheticalSyllogism",
            axiom_hypothetical_syllogism_ty(),
        ),
        ("PropLogic.resolution", axiom_resolution_ty()),
    ];
    for (name, ty) in axioms {
        env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty: ty.clone(),
        })
        .map_err(|e| e.to_string())?;
    }
    Ok(())
}
#[cfg(test)]
mod extended_prop_tests {
    use super::*;
    #[test]
    fn test_nnf_formula_atom_count() {
        let f = NnfFormula::And(
            Box::new(NnfFormula::Atom(0)),
            Box::new(NnfFormula::Or(
                Box::new(NnfFormula::Atom(1)),
                Box::new(NnfFormula::NegAtom(0)),
            )),
        );
        assert_eq!(f.atom_count(), 3);
        assert_eq!(f.connective_count(), 2);
    }
    #[test]
    fn test_nnf_formula_eval() {
        let f = NnfFormula::And(
            Box::new(NnfFormula::Atom(0)),
            Box::new(NnfFormula::NegAtom(1)),
        );
        assert!(f.eval(&[true, false]));
        assert!(!f.eval(&[true, true]));
        assert!(!f.eval(&[false, false]));
    }
    #[test]
    fn test_nnf_formula_variables() {
        let f = NnfFormula::Or(
            Box::new(NnfFormula::Atom(2)),
            Box::new(NnfFormula::NegAtom(0)),
        );
        assert_eq!(f.variables(), vec![0, 2]);
    }
    #[test]
    fn test_nnf_formula_simplify_bot() {
        let f = NnfFormula::And(Box::new(NnfFormula::Bot), Box::new(NnfFormula::Atom(0)));
        assert_eq!(f.simplify(), NnfFormula::Bot);
    }
    #[test]
    fn test_nnf_formula_simplify_top() {
        let f = NnfFormula::Or(Box::new(NnfFormula::Top), Box::new(NnfFormula::Atom(0)));
        assert_eq!(f.simplify(), NnfFormula::Top);
    }
    #[test]
    fn test_nnf_formula_simplify_and_top() {
        let f = NnfFormula::And(Box::new(NnfFormula::Top), Box::new(NnfFormula::Atom(0)));
        assert_eq!(f.simplify(), NnfFormula::Atom(0));
    }
    #[test]
    fn test_truth_table_tautology() {
        let checker = TruthTableChecker::new(1);
        let f = NnfFormula::Or(
            Box::new(NnfFormula::Atom(0)),
            Box::new(NnfFormula::NegAtom(0)),
        );
        assert!(checker.is_tautology(&f));
    }
    #[test]
    fn test_truth_table_contradiction() {
        let checker = TruthTableChecker::new(1);
        let f = NnfFormula::And(
            Box::new(NnfFormula::Atom(0)),
            Box::new(NnfFormula::NegAtom(0)),
        );
        assert!(checker.is_contradiction(&f));
    }
    #[test]
    fn test_truth_table_satisfiable() {
        let checker = TruthTableChecker::new(2);
        let f = NnfFormula::And(Box::new(NnfFormula::Atom(0)), Box::new(NnfFormula::Atom(1)));
        assert!(checker.is_satisfiable(&f));
        assert!(!checker.is_tautology(&f));
    }
    #[test]
    fn test_truth_table_count_satisfying() {
        let checker = TruthTableChecker::new(2);
        let f = NnfFormula::And(Box::new(NnfFormula::Atom(0)), Box::new(NnfFormula::Atom(1)));
        assert_eq!(checker.count_satisfying(&f), 1);
    }
    #[test]
    fn test_kripke_model_reflexive() {
        let m = KripkeModel::reflexive(3, 2);
        assert!(m.is_reflexive());
    }
    #[test]
    fn test_kripke_model_accessible_count() {
        let m = KripkeModel::reflexive(3, 2);
        assert_eq!(m.accessible_count(0), 1);
    }
    #[test]
    fn test_kripke_model_set_access() {
        let mut m = KripkeModel::reflexive(3, 2);
        m.set_access(0, 1);
        m.set_access(1, 2);
        assert!(!m.is_transitive());
        m.set_access(0, 2);
        assert!(m.is_transitive());
    }
    #[test]
    fn test_kripke_model_symmetric() {
        let mut m = KripkeModel::reflexive(2, 1);
        m.set_access(0, 1);
        assert!(!m.is_symmetric());
        m.set_access(1, 0);
        assert!(m.is_symmetric());
    }
    #[test]
    fn test_sequent_is_initial() {
        let p = NnfFormula::Atom(0);
        let s = Sequent::axiom(p);
        assert!(s.is_initial());
    }
    #[test]
    fn test_sequent_is_not_initial() {
        let p = NnfFormula::Atom(0);
        let q = NnfFormula::Atom(1);
        let s = Sequent::new(vec![p], vec![q]);
        assert!(!s.is_initial());
    }
    #[test]
    fn test_sequent_has_false_antecedent() {
        let s = Sequent::new(vec![NnfFormula::Bot], vec![NnfFormula::Atom(0)]);
        assert!(s.has_false_in_antecedent());
    }
    #[test]
    fn test_sequent_has_true_succedent() {
        let s = Sequent::new(vec![NnfFormula::Atom(0)], vec![NnfFormula::Top]);
        assert!(s.has_true_in_succedent());
    }
    #[test]
    fn test_sequent_classically_valid() {
        let p = NnfFormula::Atom(0);
        let s = Sequent::axiom(p);
        assert!(s.is_classically_valid(1));
    }
    #[test]
    fn test_axiom_excluded_middle_ty() {
        let ty = axiom_excluded_middle_ty();
        assert!(matches!(ty, Expr::Pi(_, _, _, _)));
    }
    #[test]
    fn test_axiom_double_negation_elim_ty() {
        let ty = axiom_double_negation_elim_ty();
        assert!(matches!(ty, Expr::Pi(_, _, _, _)));
    }
    #[test]
    fn test_axiom_modal_k_ty() {
        let ty = axiom_modal_k_ty();
        assert!(matches!(ty, Expr::Pi(_, _, _, _)));
    }
    #[test]
    fn test_axiom_de_morgan_classical1_ty() {
        let ty = axiom_de_morgan_classical1_ty();
        assert!(matches!(ty, Expr::Pi(_, _, _, _)));
    }
    #[test]
    fn test_register_prop_extended() {
        let mut env = Environment::new();
        assert!(register_prop_extended(&mut env).is_ok());
        assert!(env.contains(&Name::str("ClassicalLogic.excludedMiddle")));
        assert!(env.contains(&Name::str("ClassicalLogic.doubleNegationElim")));
        assert!(env.contains(&Name::str("ModalLogic.K")));
        assert!(env.contains(&Name::str("ModalLogic.T")));
        assert!(env.contains(&Name::str("ModalLogic.S4")));
        assert!(env.contains(&Name::str("Craig.interpolation")));
        assert!(env.contains(&Name::str("DeMorgan.classical1")));
        assert!(env.contains(&Name::str("IPC.modusPonens")));
    }
}
