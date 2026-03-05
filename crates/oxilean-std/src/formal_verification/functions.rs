//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{
    AbstractDomain, AbstractState, Assertion, BisimulationChecker, BisimulationKind,
    BoundedModelChecker, CEGARLoop, CTLFormula, CTLModelChecker, FrameRule, HeapPredicate,
    HoareTriple, HoareTripleVerifier, KripkeStructure, LTLBuchiConverter, LTLFormula,
    LoopInvariant, ModelCheckingResult, PetriNet, RankingFunction, RefinementType,
    SecurityClassification, SecurityLevel, SeparationLogicChecker, StrongestPostcondition,
    WeakestPrecondition,
};

pub fn app(f: Expr, a: Expr) -> Expr {
    Expr::App(Box::new(f), Box::new(a))
}
pub fn app2(f: Expr, a: Expr, b: Expr) -> Expr {
    app(app(f, a), b)
}
pub fn app3(f: Expr, a: Expr, b: Expr, c: Expr) -> Expr {
    app(app2(f, a, b), c)
}
pub fn cst(s: &str) -> Expr {
    Expr::Const(Name::str(s), vec![])
}
pub fn prop() -> Expr {
    Expr::Sort(Level::zero())
}
pub fn type0() -> Expr {
    Expr::Sort(Level::succ(Level::zero()))
}
pub fn pi(bi: BinderInfo, name: &str, dom: Expr, body: Expr) -> Expr {
    Expr::Pi(bi, Name::str(name), Box::new(dom), Box::new(body))
}
pub fn arrow(a: Expr, b: Expr) -> Expr {
    pi(BinderInfo::Default, "_", a, b)
}
pub fn bvar(n: u32) -> Expr {
    Expr::BVar(n)
}
pub fn impl_pi(name: &str, dom: Expr, body: Expr) -> Expr {
    pi(BinderInfo::Implicit, name, dom, body)
}
pub fn string_ty() -> Expr {
    cst("String")
}
pub fn bool_ty() -> Expr {
    cst("Bool")
}
pub fn nat_ty() -> Expr {
    cst("Nat")
}
pub fn list_ty(elem: Expr) -> Expr {
    app(cst("List"), elem)
}
/// Kernel type for a Hoare triple: `HoareTriple : String → String → String → Prop`
pub fn hoare_triple_ty() -> Expr {
    arrow(string_ty(), arrow(string_ty(), arrow(string_ty(), prop())))
}
/// Kernel type for a heap predicate: `HeapPredicate : Type`
pub fn heap_predicate_ty() -> Expr {
    type0()
}
/// Kernel type for the frame rule:
/// `FrameRule : HoareTriple → HeapPredicate → HoareTriple`
pub fn frame_rule_ty() -> Expr {
    arrow(
        hoare_triple_ty(),
        arrow(heap_predicate_ty(), hoare_triple_ty()),
    )
}
/// Kernel type for an LTL formula: `LTLFormula : Type`
pub fn ltl_formula_ty() -> Expr {
    type0()
}
/// Kernel type for abstract domain element: `AbstractDomain : Type`
pub fn abstract_domain_ty() -> Expr {
    type0()
}
/// Kernel type for a refinement type: `RefinementType : Type → (Type → Prop) → Type`
pub fn refinement_type_ty() -> Expr {
    arrow(type0(), arrow(arrow(type0(), prop()), type0()))
}
/// Kernel type for a Kripke structure: `KripkeStructure : Type`
pub fn kripke_structure_ty() -> Expr {
    type0()
}
/// Kernel type for a weakest precondition transformer:
/// `WP : String → String → String`  (program → postcond → precond)
pub fn wp_transformer_ty() -> Expr {
    arrow(string_ty(), arrow(string_ty(), string_ty()))
}
/// Kernel type for strongest postcondition transformer:
/// `SP : String → String → String`  (precond → program → postcond)
pub fn sp_transformer_ty() -> Expr {
    arrow(string_ty(), arrow(string_ty(), string_ty()))
}
pub fn ctl_formula_ty() -> Expr {
    type0()
}
pub fn bisimulation_ty() -> Expr {
    impl_pi(
        "S",
        type0(),
        arrow(
            arrow(bvar(0), arrow(bvar(1), prop())),
            arrow(bvar(2), arrow(bvar(3), prop())),
        ),
    )
}
pub fn strong_bisim_ty() -> Expr {
    impl_pi(
        "S",
        type0(),
        impl_pi(
            "R",
            arrow(bvar(0), arrow(bvar(1), prop())),
            arrow(app2(cst("IsBisimulation"), bvar(1), bvar(0)), prop()),
        ),
    )
}
pub fn cegar_ty() -> Expr {
    arrow(type0(), arrow(prop(), arrow(prop(), prop())))
}
pub fn ranking_function_ty() -> Expr {
    arrow(
        arrow(cst("Nat"), cst("Int")),
        arrow(arrow(cst("State"), prop()), arrow(prop(), prop())),
    )
}
pub fn petri_net_ty() -> Expr {
    type0()
}
pub fn petri_reachability_ty() -> Expr {
    arrow(
        petri_net_ty(),
        arrow(list_ty(nat_ty()), arrow(list_ty(nat_ty()), prop())),
    )
}
pub fn noninterference_ty() -> Expr {
    arrow(
        arrow(cst("Var"), cst("SecurityLevel")),
        arrow(arrow(cst("State"), cst("State")), prop()),
    )
}
pub fn declassification_ty() -> Expr {
    arrow(
        cst("SecurityLevel"),
        arrow(arrow(cst("State"), prop()), arrow(cst("State"), prop())),
    )
}
pub fn information_flow_ty() -> Expr {
    arrow(
        arrow(cst("State"), cst("State")),
        arrow(cst("SecurityLevel"), prop()),
    )
}
pub fn predicate_abstraction_ty() -> Expr {
    arrow(
        list_ty(arrow(cst("State"), prop())),
        arrow(cst("State"), cst("AbstractState")),
    )
}
pub fn bounded_model_checking_ty() -> Expr {
    arrow(nat_ty(), arrow(type0(), arrow(prop(), prop())))
}
pub fn ic3_pdr_ty() -> Expr {
    arrow(type0(), arrow(prop(), arrow(prop(), prop())))
}
pub fn pctl_formula_ty() -> Expr {
    type0()
}
pub fn pctl_satisfaction_ty() -> Expr {
    impl_pi(
        "S",
        type0(),
        arrow(pctl_formula_ty(), arrow(bvar(1), prop())),
    )
}
pub fn atl_formula_ty() -> Expr {
    type0()
}
pub fn memory_safety_ty() -> Expr {
    arrow(
        arrow(cst("Heap"), prop()),
        arrow(arrow(cst("Ptr"), prop()), prop()),
    )
}
pub fn ownership_type_ty() -> Expr {
    arrow(type0(), type0())
}
pub fn capability_safety_ty() -> Expr {
    arrow(
        arrow(cst("Cap"), prop()),
        arrow(arrow(cst("Expr"), prop()), prop()),
    )
}
pub fn temporal_safety_ty() -> Expr {
    arrow(cst("Ptr"), arrow(nat_ty(), prop()))
}
pub fn spatial_safety_ty() -> Expr {
    arrow(cst("Ptr"), arrow(nat_ty(), prop()))
}
pub fn assume_guarantee_ty() -> Expr {
    arrow(prop(), arrow(prop(), arrow(prop(), prop())))
}
pub fn interface_automaton_ty() -> Expr {
    type0()
}
pub fn hardware_rtl_ty() -> Expr {
    arrow(cst("RTL"), prop())
}
pub fn equivalence_checking_ty() -> Expr {
    arrow(cst("RTL"), arrow(cst("RTL"), prop()))
}
pub fn csp_process_ty() -> Expr {
    type0()
}
pub fn ccs_process_ty() -> Expr {
    type0()
}
pub fn hoare_consequence_ty() -> Expr {
    arrow(
        prop(),
        arrow(
            prop(),
            arrow(
                prop(),
                arrow(
                    app2(cst("Entails"), bvar(2), bvar(1)),
                    arrow(
                        app3(cst("HoareTripleP"), bvar(3), cst("C"), bvar(1)),
                        app3(cst("HoareTripleP"), bvar(4), cst("C"), bvar(2)),
                    ),
                ),
            ),
        ),
    )
}
pub fn incorrectness_logic_ty() -> Expr {
    arrow(prop(), arrow(cst("Cmd"), arrow(prop(), prop())))
}
pub fn separation_logic_points_to_ty() -> Expr {
    arrow(cst("Ptr"), arrow(cst("Val"), type0()))
}
pub fn separation_logic_sep_conj_ty() -> Expr {
    arrow(
        arrow(cst("Heap"), prop()),
        arrow(arrow(cst("Heap"), prop()), arrow(cst("Heap"), prop())),
    )
}
/// Build an `Environment` containing kernel-level axioms for the
/// formal verification module.
pub fn build_env() -> Environment {
    let mut env = Environment::new();
    type AxiomEntry = (&'static str, fn() -> Expr);
    let decls: &[AxiomEntry] = &[
        ("HoareTriple", hoare_triple_ty),
        ("HeapPredicate", heap_predicate_ty),
        ("FrameRule", frame_rule_ty),
        ("LTLFormula", ltl_formula_ty),
        ("AbstractDomain", abstract_domain_ty),
        ("RefinementType", refinement_type_ty),
        ("KripkeStructure", kripke_structure_ty),
        ("WPTransformer", wp_transformer_ty),
        ("SPTransformer", sp_transformer_ty),
    ];
    for (name, ty_fn) in decls {
        let _ = env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty: ty_fn(),
        });
    }
    let extended: &[(&'static str, fn() -> Expr)] = &[
        ("CTLFormula", ctl_formula_ty),
        ("Bisimulation", bisimulation_ty),
        ("StrongBisim", strong_bisim_ty),
        ("CEGARLoop", cegar_ty),
        ("RankingFunction", ranking_function_ty),
        ("PetriNet", petri_net_ty),
        ("PetriReachability", petri_reachability_ty),
        ("NonInterference", noninterference_ty),
        ("Declassification", declassification_ty),
        ("InformationFlow", information_flow_ty),
        ("PredicateAbstraction", predicate_abstraction_ty),
        ("BoundedModelChecking", bounded_model_checking_ty),
        ("IC3PDR", ic3_pdr_ty),
        ("PCTLFormula", pctl_formula_ty),
        ("PCTLSatisfaction", pctl_satisfaction_ty),
        ("ATLFormula", atl_formula_ty),
        ("MemorySafety", memory_safety_ty),
        ("OwnershipType", ownership_type_ty),
        ("CapabilitySafety", capability_safety_ty),
        ("TemporalSafety", temporal_safety_ty),
        ("SpatialSafety", spatial_safety_ty),
        ("AssumeGuarantee", assume_guarantee_ty),
        ("InterfaceAutomaton", interface_automaton_ty),
        ("HardwareRTL", hardware_rtl_ty),
        ("EquivalenceChecking", equivalence_checking_ty),
        ("CSPProcess", csp_process_ty),
        ("CCSProcess", ccs_process_ty),
        ("HoareConsequence", hoare_consequence_ty),
        ("IncorrectnessLogic", incorrectness_logic_ty),
        ("SepLogicPointsTo", separation_logic_points_to_ty),
        ("SepLogicSepConj", separation_logic_sep_conj_ty),
    ];
    for (name, ty_fn) in extended {
        let _ = env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty: ty_fn(),
        });
    }
    env
}
/// Return a list of named Hoare logic inference rules (name, rule schema).
///
/// ```
/// use oxilean_std::formal_verification::hoare_logic_rules;
/// let rules = hoare_logic_rules();
/// assert!(!rules.is_empty());
/// assert!(rules.iter().any(|(name, _)| *name == "Assignment"));
/// ```
pub fn hoare_logic_rules() -> Vec<(&'static str, &'static str)> {
    vec![
        ("Skip", "{P} skip {P}",), ("Assignment", "{P[e/x]} x := e {P}",), ("Sequence",
        "{P} C1 {R}  {R} C2 {Q}\n─────────────────────\n{P} C1; C2 {Q}",),
        ("Conditional",
        "{P ∧ b} C1 {Q}  {P ∧ ¬b} C2 {Q}\n─────────────────────────────────\n{P} if b then C1 else C2 {Q}",),
        ("While",
        "{I ∧ b} C {I}\n────────────────────────\n{I} while b do C {I ∧ ¬b}",),
        ("Consequence",
        "P ⊢ P'  {P'} C {Q'}  Q' ⊢ Q\n────────────────────────────\n{P} C {Q}",),
        ("Conjunction",
        "{P} C {Q}  {P} C {R}\n─────────────────────\n{P} C {Q ∧ R}",),
        ("Disjunction",
        "{P} C {R}  {Q} C {R}\n─────────────────────\n{P ∨ Q} C {R}",),
    ]
}
/// Return the textual statement of the separation logic frame rule.
///
/// ```
/// use oxilean_std::formal_verification::separation_logic_frame_rule;
/// let rule = separation_logic_frame_rule();
/// assert!(rule.contains("frame"));
/// ```
pub fn separation_logic_frame_rule() -> &'static str {
    "{P} C {Q}  mod(C) ∩ fv(frame) = ∅\n\
     ────────────────────────────────────\n\
     {P * frame} C {Q * frame}"
}
/// Return a statement about the goal of verified program synthesis.
///
/// ```
/// use oxilean_std::formal_verification::verified_program_synthesis_statement;
/// let stmt = verified_program_synthesis_statement();
/// assert!(!stmt.is_empty());
/// ```
pub fn verified_program_synthesis_statement() -> &'static str {
    "Verified Program Synthesis: Given a specification S (a pair of pre/postconditions \
     in Hoare logic), synthesize a program P such that {Pre} P {Post} is provable in the \
     chosen program logic (Hoare/separation logic), and the program is accompanied by a \
     machine-checked proof of correctness.  refinement types serve as a lightweight \
     alternative: the type {x : T | P(x)} constrains the codomain of each function so that \
     type-checking is equivalent to verification."
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_assertion_ops() {
        let a = Assertion::new("x > 0", false);
        let b = Assertion::new("y > 0", false);
        assert_eq!(a.negate().formula, "¬(x > 0)");
        assert_eq!(a.conjunction(&b).formula, "(x > 0) ∧ (y > 0)");
        assert_eq!(a.disjunction(&b).formula, "(x > 0) ∨ (y > 0)");
    }
    #[test]
    fn test_hoare_triple_display() {
        let t = HoareTriple::new("x > 0", "x := x + 1", "x > 1");
        let s = t.to_string();
        assert!(s.contains("x > 0"));
        assert!(s.contains("x := x + 1"));
        assert!(s.contains("x > 1"));
    }
    #[test]
    fn test_hoare_triple_is_valid() {
        assert!(HoareTriple::new("False", "any", "any").is_valid());
        assert!(HoareTriple::new("P", "any", "True").is_valid());
        assert!(!HoareTriple::new("x > 0", "skip", "x > 0").is_valid());
    }
    #[test]
    fn test_weakest_precondition() {
        let wp = WeakestPrecondition::new("skip", "x > 0");
        assert_eq!(wp.compute_wp(), "x > 0");
        let wp2 = WeakestPrecondition::new("x := 1", "x > 0");
        assert_eq!(wp2.compute_wp(), "1 > 0");
    }
    #[test]
    fn test_strongest_postcondition() {
        let sp = StrongestPostcondition::new("x > 0", "skip");
        assert_eq!(sp.compute_sp(), "x > 0");
    }
    #[test]
    fn test_loop_invariant() {
        let inv = Assertion::new("i >= 0", true);
        let li = LoopInvariant::new(inv, "n - i");
        assert!(li.verify_initialization());
        assert!(li.verify_preservation());
        assert!(li.verify_termination());
    }
    #[test]
    fn test_heap_predicate_satisfiable() {
        assert!(HeapPredicate::Emp.is_satisfiable());
        assert!(HeapPredicate::PointsTo("x".into(), "1".into()).is_satisfiable());
        let sep = HeapPredicate::Sep(
            Box::new(HeapPredicate::Emp),
            Box::new(HeapPredicate::PointsTo("y".into(), "2".into())),
        );
        assert!(sep.is_satisfiable());
    }
    #[test]
    fn test_frame_rule_applies() {
        let triple = HoareTriple::new("x ↦ 1", "x := 2", "x ↦ 2");
        let frame = HeapPredicate::PointsTo("y".into(), "v".into());
        let fr = FrameRule::new(triple, frame);
        assert!(fr.applies());
        let framed = fr.framed_triple();
        assert!(framed.pre.contains("y ↦ v"));
    }
    #[test]
    fn test_ltl_depth_and_atoms() {
        let f = LTLFormula::Globally(Box::new(LTLFormula::Finally(Box::new(LTLFormula::Atom(
            "p".into(),
        )))));
        assert_eq!(f.depth(), 2);
        assert_eq!(f.atoms(), vec!["p".to_string()]);
    }
    #[test]
    fn test_ltl_display() {
        assert_eq!(LTLFormula::True.to_string(), "⊤");
        assert_eq!(LTLFormula::False.to_string(), "⊥");
        let gp = LTLFormula::Globally(Box::new(LTLFormula::Atom("p".into())));
        assert_eq!(gp.to_string(), "Gp");
    }
    #[test]
    fn test_model_checking_result() {
        let r = ModelCheckingResult::verified("Gp");
        assert!(r.is_verified());
        assert!(r.counterexample.is_none());
        let r2 = ModelCheckingResult::falsified("Fp", vec!["s0".into()]);
        assert!(!r2.is_verified());
        assert_eq!(
            r2.counterexample.expect("counterexample should be valid"),
            vec!["s0".to_string()]
        );
    }
    #[test]
    fn test_abstract_domain_ops() {
        let a = AbstractDomain::Interval(1.0, 3.0);
        let b = AbstractDomain::Interval(2.0, 5.0);
        assert_eq!(a.join(&b), AbstractDomain::Interval(1.0, 5.0));
        assert_eq!(a.meet(&b), AbstractDomain::Interval(2.0, 3.0));
        let prev = AbstractDomain::Interval(0.0, 5.0);
        let next = AbstractDomain::Interval(0.0, 10.0);
        assert_eq!(prev.widen(&next), AbstractDomain::Top);
        assert!(AbstractDomain::Bottom.join(&a) == a);
        assert!(AbstractDomain::Interval(2.0, 2.0).contains(2.0));
        assert!(!AbstractDomain::Bottom.contains(0.0));
    }
    #[test]
    fn test_abstract_state() {
        let mut s = AbstractState::new();
        s.assign("x", AbstractDomain::Interval(0.0, 10.0));
        assert_eq!(s.lookup("x"), AbstractDomain::Interval(0.0, 10.0));
        assert_eq!(s.lookup("y"), AbstractDomain::Top);
        s.assign("x", AbstractDomain::Interval(5.0, 10.0));
        assert_eq!(s.lookup("x"), AbstractDomain::Interval(5.0, 10.0));
    }
    #[test]
    fn test_refinement_type() {
        let t1 = RefinementType::new("Int", "x > 0");
        let t2 = RefinementType::new("Int", "x > 0");
        assert!(t1.is_subtype_of(&t2));
        let t3 = RefinementType::new("Nat", "x > 0");
        assert!(!t1.is_subtype_of(&t3));
    }
    #[test]
    fn test_kripke_reachable() {
        let ks = KripkeStructure::new(
            vec!["s0".into(), "s1".into(), "s2".into()],
            vec![(0, 1), (1, 2)],
            vec![
                vec!["p".into()],
                vec!["p".into(), "q".into()],
                vec!["q".into()],
            ],
        );
        assert_eq!(ks.reachable_states(0), vec![0, 1, 2]);
        assert!(ks.holds_in(0, "p"));
        assert!(!ks.holds_in(2, "p"));
        let res = ks.check_globally_atom("p");
        assert!(!res.is_verified());
    }
    #[test]
    fn test_hoare_logic_rules() {
        let rules = hoare_logic_rules();
        assert_eq!(rules.len(), 8);
        assert!(rules.iter().any(|(n, _)| *n == "While"));
    }
    #[test]
    fn test_separation_logic_frame_rule() {
        let rule = separation_logic_frame_rule();
        assert!(rule.contains("frame"));
        assert!(rule.contains("mod(C)"));
    }
    #[test]
    fn test_verified_program_synthesis_statement() {
        let s = verified_program_synthesis_statement();
        assert!(s.contains("Hoare"));
        assert!(s.contains("refinement"));
    }
    #[test]
    fn test_build_env() {
        let env = build_env();
        assert!(env.get(&Name::str("HoareTriple")).is_some());
        assert!(env.get(&Name::str("KripkeStructure")).is_some());
        assert!(env.get(&Name::str("RefinementType")).is_some());
    }
    #[test]
    fn test_extended_env_axioms() {
        let env = build_env();
        for name in &[
            "CTLFormula",
            "Bisimulation",
            "StrongBisim",
            "CEGARLoop",
            "RankingFunction",
            "PetriNet",
            "PetriReachability",
            "NonInterference",
            "Declassification",
            "InformationFlow",
            "PredicateAbstraction",
            "BoundedModelChecking",
            "IC3PDR",
            "PCTLFormula",
            "PCTLSatisfaction",
            "ATLFormula",
            "MemorySafety",
            "OwnershipType",
            "CapabilitySafety",
            "TemporalSafety",
            "SpatialSafety",
            "AssumeGuarantee",
            "InterfaceAutomaton",
            "HardwareRTL",
            "EquivalenceChecking",
            "CSPProcess",
            "CCSProcess",
            "HoareConsequence",
            "IncorrectnessLogic",
            "SepLogicPointsTo",
            "SepLogicSepConj",
        ] {
            assert!(
                env.get(&Name::str(*name)).is_some(),
                "missing axiom: {name}"
            );
        }
    }
    #[test]
    fn test_ctl_formula_depth_and_atoms() {
        let f = CTLFormula::AG(Box::new(CTLFormula::EF(Box::new(CTLFormula::Atom(
            "p".into(),
        )))));
        assert_eq!(f.depth(), 2);
        assert_eq!(f.atoms(), vec!["p".to_string()]);
    }
    #[test]
    fn test_ctl_formula_display() {
        assert_eq!(CTLFormula::True.to_string(), "⊤");
        let agp = CTLFormula::AG(Box::new(CTLFormula::Atom("p".into())));
        assert_eq!(agp.to_string(), "AGp");
        let eu = CTLFormula::EU(
            Box::new(CTLFormula::Atom("p".into())),
            Box::new(CTLFormula::Atom("q".into())),
        );
        assert_eq!(eu.to_string(), "E(p U q)");
    }
    fn make_kripke() -> KripkeStructure {
        KripkeStructure::new(
            vec!["s0".into(), "s1".into(), "s2".into()],
            vec![(0, 1), (1, 2), (2, 1)],
            vec![
                vec!["p".into()],
                vec!["p".into(), "q".into()],
                vec!["q".into()],
            ],
        )
    }
    #[test]
    fn test_ctl_model_checker_atom() {
        let ks = make_kripke();
        let checker = CTLModelChecker::new(&ks);
        let sat_p = checker.sat(&CTLFormula::Atom("p".into()));
        assert!(sat_p.contains(&0));
        assert!(sat_p.contains(&1));
        assert!(!sat_p.contains(&2));
    }
    #[test]
    fn test_ctl_model_checker_ef() {
        let ks = make_kripke();
        let checker = CTLModelChecker::new(&ks);
        let sat = checker.sat(&CTLFormula::EF(Box::new(CTLFormula::Atom("q".into()))));
        assert!(sat.contains(&0));
    }
    #[test]
    fn test_ctl_model_checker_eu() {
        let ks = make_kripke();
        let checker = CTLModelChecker::new(&ks);
        let sat = checker.sat(&CTLFormula::EU(
            Box::new(CTLFormula::Atom("p".into())),
            Box::new(CTLFormula::Atom("q".into())),
        ));
        assert!(sat.contains(&0) || sat.contains(&1));
    }
    #[test]
    fn test_ltl_buchi_converter() {
        let f = LTLFormula::Globally(Box::new(LTLFormula::Atom("p".into())));
        let conv = LTLBuchiConverter::new(f);
        assert!(conv.is_safety());
        assert!(!conv.is_liveness());
        let states = conv.produce_states();
        assert!(!states.is_empty());
        assert!(states.iter().any(|s| s.is_accepting));
        let f2 = LTLFormula::Finally(Box::new(LTLFormula::Atom("q".into())));
        let conv2 = LTLBuchiConverter::new(f2);
        assert!(conv2.is_liveness());
        assert!(!conv2.is_safety());
    }
    #[test]
    fn test_hoare_triple_verifier() {
        let verifier = HoareTripleVerifier::new();
        let t1 = HoareTriple::new("x > 0", "skip", "x > 0");
        assert!(verifier.verify(&t1));
        assert_eq!(verifier.applicable_rule(&t1), "Skip");
        let t2 = HoareTriple::new("1 > 0", "x := 1", "x > 0");
        assert!(verifier.verify(&t2));
        assert_eq!(verifier.applicable_rule(&t2), "Assignment");
        let t3 = HoareTriple::new("False", "any", "Q");
        assert!(verifier.verify(&t3));
    }
    #[test]
    fn test_separation_logic_checker() {
        let checker = SeparationLogicChecker::new();
        let emp = HeapPredicate::Emp;
        assert!(checker.entails(&emp, &emp));
        let spec = HoareTriple::new("P", "cmd", "Q");
        let frame = HeapPredicate::PointsTo("y".into(), "v".into());
        assert!(checker.check_frame_rule(&spec, &frame));
        assert!(checker.check_incorrectness(&emp, &emp));
    }
    #[test]
    fn test_bisimulation_checker() {
        let ks = make_kripke();
        let checker = BisimulationChecker::new(BisimulationKind::Strong);
        assert!(!checker.bisimilar(&ks, 0, 1));
        assert!(checker.bisimilar(&ks, 0, 0));
    }
    #[test]
    fn test_cegar_loop() {
        let mut cegar = CEGARLoop::new();
        assert_eq!(cegar.iterations, 0);
        let spurious = cegar.step(true);
        assert!(spurious);
        assert_eq!(cegar.iterations, 1);
        assert_eq!(cegar.predicates.len(), 1);
        cegar.set_proven();
        assert!(cegar.proof_found);
    }
    #[test]
    fn test_ranking_function() {
        let rf = RankingFunction::new("n", 0, "n");
        assert!(rf.is_above_bound(5));
        assert!(!rf.is_above_bound(0));
        assert_eq!(rf.step(5, 2), Some(3));
        assert_eq!(rf.step(1, 2), None);
    }
    #[test]
    fn test_petri_net() {
        let mut net = PetriNet::new(2, vec![1, 0]);
        net.add_transition(vec![1, 0], vec![0, 1]);
        assert!(net.is_enabled(&[1, 0], 0));
        assert!(!net.is_enabled(&[0, 1], 0));
        let new_m = net.fire(&[1, 0], 0).expect("fire should succeed");
        assert_eq!(new_m, vec![0, 1]);
        let reachable = net.reachable_markings();
        assert!(reachable.contains(&vec![0, 1]));
        assert!(net.is_bounded(1));
    }
    #[test]
    fn test_security_classification() {
        let mut sc = SecurityClassification::new();
        sc.classify("x", SecurityLevel::High);
        sc.classify("y", SecurityLevel::Low);
        assert_eq!(sc.level_of("x"), Some(SecurityLevel::High));
        assert_eq!(sc.level_of("y"), Some(SecurityLevel::Low));
        assert!(sc.check_noninterference("x", &["y"]));
        assert!(!sc.check_noninterference("y", &["x"]));
    }
    #[test]
    fn test_bounded_model_checker() {
        let ks = make_kripke();
        let bmc = BoundedModelChecker::new(3);
        assert!(bmc.check_finally(&ks, "q"));
        assert!(!bmc.check_globally(&ks, "p"));
    }
}
