//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{
    CoC, DependentType, ExtensionalVsIntensional, Grade, IdentityTypeScheme, Induction,
    ModalContext, NormalizationThm, ObservationalEqualityChecker, QuantitativeTypeInference,
    RealizabilityAssembly, SetoidQuotientComputer, SyntheticDomainTheory, TwoLevelTypeChecker,
    UniversePolymorphism, VarRules, CIC,
};

pub fn prop() -> Expr {
    Expr::Sort(Level::zero())
}
pub fn type0() -> Expr {
    Expr::Sort(Level::succ(Level::zero()))
}
pub fn type1() -> Expr {
    Expr::Sort(Level::succ(Level::succ(Level::zero())))
}
pub fn bvar(n: u32) -> Expr {
    Expr::BVar(n)
}
pub fn cst(s: &str) -> Expr {
    Expr::Const(Name::str(s), vec![])
}
pub fn arrow(a: Expr, b: Expr) -> Expr {
    Expr::Pi(
        BinderInfo::Default,
        Name::str("_"),
        Box::new(a),
        Box::new(b),
    )
}
pub fn pi(name: &str, dom: Expr, body: Expr) -> Expr {
    Expr::Pi(
        BinderInfo::Default,
        Name::str(name),
        Box::new(dom),
        Box::new(body),
    )
}
pub fn app(f: Expr, a: Expr) -> Expr {
    Expr::App(Box::new(f), Box::new(a))
}
pub fn app2(f: Expr, a: Expr, b: Expr) -> Expr {
    app(app(f, a), b)
}
pub fn app3(f: Expr, a: Expr, b: Expr, c: Expr) -> Expr {
    app(app2(f, a, b), c)
}
pub fn nat_ty() -> Expr {
    cst("Nat")
}
pub fn string_ty() -> Expr {
    cst("String")
}
pub fn bool_ty() -> Expr {
    cst("Bool")
}
pub fn list_ty(t: Expr) -> Expr {
    app(cst("List"), t)
}
/// DependentType : String → String → Type
pub fn dependent_type_ty() -> Expr {
    arrow(string_ty(), arrow(string_ty(), type0()))
}
/// TotalSpace : String → String → Prop
pub fn total_space_ty() -> Expr {
    arrow(string_ty(), arrow(string_ty(), prop()))
}
/// PiType : String → String → Prop
pub fn pi_type_ty() -> Expr {
    arrow(string_ty(), arrow(string_ty(), prop()))
}
/// SigmaType : String → String → Prop
pub fn sigma_type_ty() -> Expr {
    arrow(string_ty(), arrow(string_ty(), prop()))
}
/// IdentityType : String → Bool → Type
pub fn identity_type_ty() -> Expr {
    arrow(string_ty(), arrow(bool_ty(), type0()))
}
/// JRule : String → Prop
pub fn j_rule_ty() -> Expr {
    arrow(string_ty(), prop())
}
/// InductionPrinciple : String → List String → Prop
pub fn induction_principle_ty() -> Expr {
    arrow(string_ty(), arrow(list_ty(string_ty()), prop()))
}
/// UniquenessOfNormalForm : String → Prop
pub fn uniqueness_nf_ty() -> Expr {
    arrow(string_ty(), prop())
}
/// CICSystem : List String → Bool → Prop
pub fn cic_system_ty() -> Expr {
    arrow(list_ty(string_ty()), arrow(bool_ty(), prop()))
}
/// CICConsistency : List String → Bool
pub fn cic_consistency_ty() -> Expr {
    arrow(list_ty(string_ty()), bool_ty())
}
/// CurryHoward : List String → Prop
pub fn curry_howard_ty() -> Expr {
    arrow(list_ty(string_ty()), prop())
}
/// PropositionsAsTypes : List String → Prop
pub fn pat_ty() -> Expr {
    arrow(list_ty(string_ty()), prop())
}
/// ExpressivePower : List String → Prop
pub fn expressive_power_ty() -> Expr {
    arrow(list_ty(string_ty()), prop())
}
/// VariableRule : Bool → Prop
pub fn variable_rule_ty() -> Expr {
    arrow(bool_ty(), prop())
}
/// WeakeningRule : Bool → Prop
pub fn weakening_rule_ty() -> Expr {
    arrow(bool_ty(), prop())
}
/// SubstitutionRule : Bool → Prop
pub fn substitution_rule_ty() -> Expr {
    arrow(bool_ty(), prop())
}
/// ExchangeRule : Bool → Bool → Prop
pub fn exchange_rule_ty() -> Expr {
    arrow(bool_ty(), arrow(bool_ty(), prop()))
}
/// StrongNormalization : String → Prop
pub fn strong_normalization_ty() -> Expr {
    arrow(string_ty(), prop())
}
/// Confluence : String → Prop
pub fn confluence_ty() -> Expr {
    arrow(string_ty(), prop())
}
/// DecidableTypeChecking : String → Bool
pub fn decidable_tc_ty() -> Expr {
    arrow(string_ty(), bool_ty())
}
/// UniverseLevel : Nat → Type (universe at level n)
pub fn universe_level_ty() -> Expr {
    arrow(nat_ty(), type1())
}
/// Cumulativity : Nat → Nat → Prop (Type_n ⊆ Type_m for n ≤ m)
pub fn cumulativity_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), prop()))
}
/// LevelLifting : Nat → String → Type
pub fn level_lifting_ty() -> Expr {
    arrow(nat_ty(), arrow(string_ty(), type0()))
}
/// EqualityReflection : Bool → Prop
pub fn equality_reflection_ty() -> Expr {
    arrow(bool_ty(), prop())
}
/// UndecidableTypeCheckingExt : Bool → Prop
pub fn undecidable_tc_ext_ty() -> Expr {
    arrow(bool_ty(), prop())
}
/// FunctionExtensionality : Bool → Prop
pub fn funext_ty() -> Expr {
    arrow(bool_ty(), prop())
}
/// UniquenessOfIdentityProofs : Bool → Prop
pub fn uip_ty() -> Expr {
    arrow(bool_ty(), prop())
}
/// DependentSum : String → String → Type
pub fn dependent_sum_ty() -> Expr {
    arrow(string_ty(), arrow(string_ty(), type0()))
}
/// DependentProduct : String → String → Type
pub fn dependent_product_ty() -> Expr {
    arrow(string_ty(), arrow(string_ty(), type0()))
}
/// InductiveRecursion : String → List String → Type
pub fn inductive_recursion_ty() -> Expr {
    arrow(string_ty(), arrow(list_ty(string_ty()), type0()))
}
/// WernerModel : List String → Bool → Prop
pub fn werner_model_ty() -> Expr {
    arrow(list_ty(string_ty()), arrow(bool_ty(), prop()))
}
/// ObservationalEquality : Bool → Prop
/// Heterogeneous equality in OTT; equality at a type is observational.
pub fn observational_equality_ty() -> Expr {
    arrow(bool_ty(), prop())
}
/// HeterogeneousEquality : String → String → Prop
/// Congruence across different types: a : A ≅ b : B.
pub fn heterogeneous_equality_ty() -> Expr {
    arrow(string_ty(), arrow(string_ty(), prop()))
}
/// OTTCongruence : String → Prop
/// Congruence lemma in OTT: equality is a congruence for all type formers.
pub fn ott_congruence_ty() -> Expr {
    arrow(string_ty(), prop())
}
/// FibrantType : Bool → Prop
/// A type is fibrant if it participates in path induction (HoTT layer).
pub fn fibrant_type_ty() -> Expr {
    arrow(bool_ty(), prop())
}
/// StrictFibrantType : Bool → Prop
/// A fibrant type in the strict (non-higher) layer of 2LTT.
pub fn strict_fibrant_ty() -> Expr {
    arrow(bool_ty(), prop())
}
/// TwoLevelCoherence : Bool → Bool → Prop
/// Coherence between the fibrant and non-fibrant layers of 2LTT.
pub fn two_level_coherence_ty() -> Expr {
    arrow(bool_ty(), arrow(bool_ty(), prop()))
}
/// SetoidModel : String → Prop
/// A setoid (A, ~) is a type equipped with an equivalence relation.
pub fn setoid_model_ty() -> Expr {
    arrow(string_ty(), prop())
}
/// SetoidQuotient : String → String → Type
/// The quotient type A/~ in setoid type theory.
pub fn setoid_quotient_ty() -> Expr {
    arrow(string_ty(), arrow(string_ty(), type0()))
}
/// EffectiveEquality : String → Prop
/// Every equivalence relation in a setoid model is effective (quotients are well-formed).
pub fn effective_equality_ty() -> Expr {
    arrow(string_ty(), prop())
}
/// PCARealizability : String → Prop
/// Realizability model based on a partial combinatory algebra (PCA).
pub fn pca_realizability_ty() -> Expr {
    arrow(string_ty(), prop())
}
/// KleeneSecondAlgebra : Bool → Prop
/// Kleene's second algebra K2: total functions ℕ^ℕ → ℕ as PCA.
pub fn kleene_second_algebra_ty() -> Expr {
    arrow(bool_ty(), prop())
}
/// CategoricalModel : String → Prop
/// A categorical model interprets types as objects and terms as morphisms.
pub fn categorical_model_ty() -> Expr {
    arrow(string_ty(), prop())
}
/// FiberedCategory : String → String → Prop
/// A fibered category (Grothendieck fibration) over a base category.
pub fn fibered_category_ty() -> Expr {
    arrow(string_ty(), arrow(string_ty(), prop()))
}
/// DisplayMap : String → String → Prop
/// Display maps (Hyland-Pitts): a class of morphisms modelling dependent types.
pub fn display_map_ty() -> Expr {
    arrow(string_ty(), arrow(string_ty(), prop()))
}
/// EdinburghLF : String → Prop
/// Edinburgh Logical Framework (Harper-Honsell-Plotkin 1993): typing derivations as LF terms.
pub fn edinburgh_lf_ty() -> Expr {
    arrow(string_ty(), prop())
}
/// TwelfSignature : List String → Prop
/// Twelf (Pfenning-Schürmann): dependent types for meta-reasoning about logics.
pub fn twelf_signature_ty() -> Expr {
    arrow(list_ty(string_ty()), prop())
}
/// NecessityType : String → Type
/// □A: the necessity modality (type of necessarily-true propositions).
pub fn necessity_type_ty() -> Expr {
    arrow(string_ty(), type0())
}
/// PossibilityType : String → Type
/// ◇A: the possibility modality (type of possibly-true propositions).
pub fn possibility_type_ty() -> Expr {
    arrow(string_ty(), type0())
}
/// HybridLogicType : String → String → Prop
/// Hybrid logic types combine modal and nominal features.
pub fn hybrid_logic_type_ty() -> Expr {
    arrow(string_ty(), arrow(string_ty(), prop()))
}
/// GradedType : String → Nat → Type
/// A graded type tracks resource usage via a semiring annotation.
pub fn graded_type_ty() -> Expr {
    arrow(string_ty(), arrow(nat_ty(), type0()))
}
/// CoeffectTyping : String → Nat → Prop
/// Coeffect typing: a judgment Γ ⊢_{r} e : A where r is a resource grade.
pub fn coeffect_typing_ty() -> Expr {
    arrow(string_ty(), arrow(nat_ty(), prop()))
}
/// ErasureSemantics : String → Prop
/// Erasure: terms with grade 0 are computationally irrelevant.
pub fn erasure_semantics_ty() -> Expr {
    arrow(string_ty(), prop())
}
/// DirectedUnivalence : Bool → Prop
/// Directed univalence: equivalences of categories are equivalent as types.
pub fn directed_univalence_ty() -> Expr {
    arrow(bool_ty(), prop())
}
/// TwoCategoryType : String → String → Type
/// A 2-categorical type where hom-types have additional structure.
pub fn two_category_type_ty() -> Expr {
    arrow(string_ty(), arrow(string_ty(), type0()))
}
/// DefinitionalEqualityVsPropositional : Bool → Bool → Prop
/// The distinction between definitional (=_β) and propositional (Id) equality.
pub fn def_eq_vs_prop_ty() -> Expr {
    arrow(bool_ty(), arrow(bool_ty(), prop()))
}
/// Dominance : String → Prop
/// A dominance Σ ⊆ Prop classifying partial elements in synthetic domain theory.
pub fn dominance_ty() -> Expr {
    arrow(string_ty(), prop())
}
/// LiftMonad : String → Type
/// L(A): the lift monad, i.e., the type of possibly-diverging computations of type A.
pub fn lift_monad_ty() -> Expr {
    arrow(string_ty(), type0())
}
/// PartialMap : String → String → Prop
/// A partial map A ⇀ B: a total map from A to L(B).
pub fn partial_map_ty() -> Expr {
    arrow(string_ty(), arrow(string_ty(), prop()))
}
/// Register all advanced type theory axioms into the given environment.
pub fn build_env(env: &mut Environment) {
    let axioms: &[(&str, Expr)] = &[
        ("DependentType", dependent_type_ty()),
        ("TotalSpace", total_space_ty()),
        ("PiType", pi_type_ty()),
        ("SigmaType", sigma_type_ty()),
        ("IdentityTypeScheme", identity_type_ty()),
        ("JRule", j_rule_ty()),
        ("DependentSum", dependent_sum_ty()),
        ("DependentProduct", dependent_product_ty()),
        ("InductionPrinciple", induction_principle_ty()),
        ("UniquenessOfNormalForm", uniqueness_nf_ty()),
        ("InductiveRecursion", inductive_recursion_ty()),
        ("CICSystem", cic_system_ty()),
        ("CICConsistency", cic_consistency_ty()),
        ("WernerModel", werner_model_ty()),
        ("CurryHoward", curry_howard_ty()),
        ("PropositionsAsTypes", pat_ty()),
        ("ExpressivePower", expressive_power_ty()),
        ("VariableRule", variable_rule_ty()),
        ("WeakeningRule", weakening_rule_ty()),
        ("SubstitutionRule", substitution_rule_ty()),
        ("ExchangeRule", exchange_rule_ty()),
        ("StrongNormalization", strong_normalization_ty()),
        ("Confluence", confluence_ty()),
        ("DecidableTypeChecking", decidable_tc_ty()),
        ("UniverseLevel", universe_level_ty()),
        ("Cumulativity", cumulativity_ty()),
        ("LevelLifting", level_lifting_ty()),
        ("EqualityReflection", equality_reflection_ty()),
        ("UndecidableTypeCheckingExt", undecidable_tc_ext_ty()),
        ("FunctionExtensionality", funext_ty()),
        ("UniquenessOfIdentityProofs", uip_ty()),
        ("ObservationalEquality", observational_equality_ty()),
        ("HeterogeneousEquality", heterogeneous_equality_ty()),
        ("OTTCongruence", ott_congruence_ty()),
        ("FibrantType", fibrant_type_ty()),
        ("StrictFibrantType", strict_fibrant_ty()),
        ("TwoLevelCoherence", two_level_coherence_ty()),
        ("SetoidModel", setoid_model_ty()),
        ("SetoidQuotient", setoid_quotient_ty()),
        ("EffectiveEquality", effective_equality_ty()),
        ("PCARealizability", pca_realizability_ty()),
        ("KleeneSecondAlgebra", kleene_second_algebra_ty()),
        ("CategoricalModel", categorical_model_ty()),
        ("FiberedCategory", fibered_category_ty()),
        ("DisplayMap", display_map_ty()),
        ("EdinburghLF", edinburgh_lf_ty()),
        ("TwelfSignature", twelf_signature_ty()),
        ("NecessityType", necessity_type_ty()),
        ("PossibilityType", possibility_type_ty()),
        ("HybridLogicType", hybrid_logic_type_ty()),
        ("GradedType", graded_type_ty()),
        ("CoeffectTyping", coeffect_typing_ty()),
        ("ErasureSemantics", erasure_semantics_ty()),
        ("DirectedUnivalence", directed_univalence_ty()),
        ("TwoCategoryType", two_category_type_ty()),
        ("DefinitionalEqualityVsPropositional", def_eq_vs_prop_ty()),
        ("Dominance", dominance_ty()),
        ("LiftMonad", lift_monad_ty()),
        ("PartialMap", partial_map_ty()),
    ];
    for (name, ty) in axioms {
        let _ = env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty: ty.clone(),
        });
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_dependent_type_total_space() {
        let dt = DependentType::new("Nat", "Vec");
        let ts = dt.total_space();
        assert!(ts.contains("TotalSpace"));
        assert!(ts.contains("Nat"));
        assert!(ts.contains("Vec"));
    }
    #[test]
    fn test_dependent_type_pi() {
        let dt = DependentType::new("A", "B");
        let pi = dt.pi_type();
        assert!(pi.contains("Π-type"));
        assert!(pi.contains("β-reduction"));
    }
    #[test]
    fn test_dependent_type_sigma() {
        let dt = DependentType::new("A", "B");
        let sigma = dt.sigma_type();
        assert!(sigma.contains("Σ-type"));
    }
    #[test]
    fn test_induction_nat() {
        let nat = Induction::nat();
        let ip = nat.induction_principle();
        assert!(ip.contains("Nat"));
        assert!(ip.contains("zero"));
    }
    #[test]
    fn test_induction_uniqueness() {
        let list = Induction::list();
        let up = list.uniqueness_principle();
        assert!(up.contains("List"));
        assert!(up.contains("uniqueness") || up.contains("η-rule"));
    }
    #[test]
    fn test_cic_standard() {
        let cic = CIC::standard();
        let desc = cic.calculus_of_inductive_constructions();
        assert!(desc.contains("CIC"));
        assert!(desc.contains("impredicat"));
        assert!(cic.is_consistent());
    }
    #[test]
    fn test_cic_universes() {
        let cic = CIC::new(vec!["Prop".to_string(), "Type₀".to_string()]);
        assert!(cic.has_impredicative_prop);
        assert!(cic.has_principal_typing());
    }
    #[test]
    fn test_coc_curry_howard() {
        let coc = CoC::empty();
        let ch = coc.curry_howard();
        assert!(ch.contains("Curry-Howard"));
        assert!(ch.contains("propositions = types"));
    }
    #[test]
    fn test_coc_propositions_as_types() {
        let coc = CoC::new(vec!["id".to_string(), "const".to_string()]);
        let pat = coc.propositions_as_types();
        assert!(pat.contains("propositions-as-types") || pat.contains("Propositions-as-types"));
    }
    #[test]
    fn test_coc_expressive_power() {
        let coc = CoC::empty();
        let ep = coc.expressive_power();
        assert!(ep.contains("System F"));
        assert!(coc.is_strongly_normalizing());
    }
    #[test]
    fn test_var_rules_variable() {
        let vr = VarRules::dependent();
        let rule = vr.variable_rule();
        assert!(rule.contains("Variable rule"));
        assert!(rule.contains("Var"));
    }
    #[test]
    fn test_var_rules_weakening() {
        let vr = VarRules::new();
        let w = vr.weakening();
        assert!(w.contains("Weakening"));
    }
    #[test]
    fn test_var_rules_substitution() {
        let vr = VarRules::new();
        let s = vr.substitution();
        assert!(s.contains("Substitution"));
        assert!(s.contains("β-reduction"));
    }
    #[test]
    fn test_var_rules_exchange() {
        let dep = VarRules::dependent();
        let exc = dep.exchange();
        assert!(exc.contains("Exchange"));
        assert!(exc.contains("independen"));
    }
    #[test]
    fn test_normalization_sn() {
        let norm = NormalizationThm::new("CIC");
        let sn = norm.strong_normalization();
        assert!(sn.contains("Strong Normalization"));
        assert!(sn.contains("CIC"));
    }
    #[test]
    fn test_normalization_confluence() {
        let norm = NormalizationThm::new("CoC");
        let cr = norm.confluence();
        assert!(cr.contains("Confluence") || cr.contains("Church-Rosser"));
        assert!(norm.decidable_type_checking());
    }
    #[test]
    fn test_universe_polymorphism_cumulativity() {
        let up = UniversePolymorphism::new(vec![0, 1, 2]);
        let cum = up.cumulativity();
        assert!(cum.contains("Cumulativity"));
        assert!(!up.is_monomorphic());
    }
    #[test]
    fn test_universe_polymorphism_level_lifting() {
        let up = UniversePolymorphism::single();
        let ll = up.level_lifting();
        assert!(ll.contains("Level lifting") || ll.contains("ULift"));
        assert!(up.is_monomorphic());
    }
    #[test]
    fn test_extensional_equality_reflection() {
        let ext = ExtensionalVsIntensional::extensional();
        let er = ext.equality_reflection();
        assert!(er.contains("equality reflection") || er.contains("Equality reflection"));
        assert!(ext.uip_holds());
        assert!(ext.funext_holds());
    }
    #[test]
    fn test_intensional_undecidability() {
        let int = ExtensionalVsIntensional::intensional();
        let ud = int.undecidable_type_checking_extensional();
        assert!(ud.contains("decidable"));
        assert!(!int.has_equality_reflection);
    }
    #[test]
    fn test_extensional_undecidability() {
        let ext = ExtensionalVsIntensional::extensional();
        let ud = ext.undecidable_type_checking_extensional();
        assert!(ud.contains("Undecidability") || ud.contains("undecidable"));
    }
    #[test]
    fn test_build_env() {
        let mut env = Environment::new();
        build_env(&mut env);
        assert!(env.get(&Name::str("DependentType")).is_some());
        assert!(env.get(&Name::str("PiType")).is_some());
        assert!(env.get(&Name::str("SigmaType")).is_some());
        assert!(env.get(&Name::str("CICSystem")).is_some());
        assert!(env.get(&Name::str("CurryHoward")).is_some());
        assert!(env.get(&Name::str("StrongNormalization")).is_some());
        assert!(env.get(&Name::str("Confluence")).is_some());
        assert!(env.get(&Name::str("Cumulativity")).is_some());
        assert!(env.get(&Name::str("EqualityReflection")).is_some());
    }
    #[test]
    fn test_observational_equality_checker_basic() {
        let mut chk = ObservationalEqualityChecker::new();
        chk.assert_equal("zero", "0");
        assert!(chk.is_equal("zero", "0"));
        assert!(!chk.is_equal("zero", "one"));
        assert_eq!(chk.num_equalities(), 1);
    }
    #[test]
    fn test_observational_equality_congruence() {
        let mut chk = ObservationalEqualityChecker::new();
        chk.assert_equal("a", "b");
        let result = chk.congruence("Nat.succ", "a", "b");
        assert!(result.contains("congruence"), "got: {result}");
    }
    #[test]
    fn test_observational_equality_heterogeneous() {
        let chk = ObservationalEqualityChecker::new().with_heterogeneous();
        assert!(chk.heterogeneous);
    }
    #[test]
    fn test_two_level_type_checker_fibrant() {
        let mut chk = TwoLevelTypeChecker::new();
        chk.register_fibrant("Nat");
        chk.register_fibrant("Bool");
        chk.register_strict("InternalNat");
        assert!(chk.is_fibrant("Nat"));
        assert!(!chk.is_fibrant("InternalNat"));
        assert!(chk.is_strict("InternalNat"));
    }
    #[test]
    fn test_two_level_coerce_to_strict() {
        let mut chk = TwoLevelTypeChecker::new();
        chk.register_fibrant("A");
        let s = chk.coerce_to_strict("A");
        assert!(s.contains("Strict"), "got: {s}");
    }
    #[test]
    fn test_two_level_summary() {
        let mut chk = TwoLevelTypeChecker::new();
        chk.register_fibrant("A");
        chk.register_strict("B");
        let s = chk.summary();
        assert!(s.contains("2LTT"), "got: {s}");
        assert!(s.contains("coherence"), "got: {s}");
    }
    #[test]
    fn test_setoid_quotient_single_class() {
        let mut sq =
            SetoidQuotientComputer::new(vec!["a".to_string(), "b".to_string(), "c".to_string()]);
        sq.add_equiv(0, 1);
        sq.add_equiv(1, 2);
        let cls_0 = sq.equiv_class(0);
        assert!(cls_0.contains(&1), "0 ~ 1 should hold");
    }
    #[test]
    fn test_setoid_quotient_num_classes() {
        let mut sq =
            SetoidQuotientComputer::new(vec!["x".to_string(), "y".to_string(), "z".to_string()]);
        sq.add_equiv(0, 1);
        let nc = sq.num_classes();
        assert!(nc <= 3, "at most 3 classes, got {nc}");
    }
    #[test]
    fn test_setoid_canonical() {
        let mut sq = SetoidQuotientComputer::new(vec!["p".to_string(), "q".to_string()]);
        sq.add_equiv(0, 1);
        assert_eq!(sq.canonical(1), 0, "canonical of 1 should be 0 (smallest)");
    }
    #[test]
    fn test_qtt_grade_arithmetic() {
        assert_eq!(Grade::Zero.add(Grade::Linear), Grade::Linear);
        assert_eq!(Grade::Linear.add(Grade::Linear), Grade::Unrestricted);
        assert_eq!(Grade::Zero.mul(Grade::Linear), Grade::Zero);
        assert_eq!(Grade::Linear.mul(Grade::Linear), Grade::Linear);
    }
    #[test]
    fn test_qtt_inference_context() {
        let mut qtt = QuantitativeTypeInference::new();
        qtt.bind("x", "Nat", 1);
        qtt.bind("y", "Bool", 0);
        qtt.bind("f", "Nat->Nat", 2);
        assert!(qtt.is_linear("x"));
        assert!(qtt.is_erased("y"));
        assert!(qtt.is_unrestricted("f"));
    }
    #[test]
    fn test_qtt_context_description() {
        let mut qtt = QuantitativeTypeInference::new();
        qtt.bind("a", "A", 1);
        let desc = qtt.context_description();
        assert!(desc.contains("a"), "got: {desc}");
        assert!(desc.contains("A"), "got: {desc}");
    }
    #[test]
    fn test_realizability_assembly() {
        let asm = RealizabilityAssembly::new("NatAsm", true, 100);
        assert!(asm.is_partitioned());
        assert_eq!(asm.size, 100);
        let desc = RealizabilityAssembly::effective_topos_description();
        assert!(desc.contains("topos"), "got: {desc}");
    }
    #[test]
    fn test_modal_context_standard() {
        let mc = ModalContext::standard();
        assert!(mc.has_modality("□"));
        assert!(mc.has_modality("◇"));
        assert!(mc.necessity_is_comonad);
        assert!(mc.possibility_is_monad);
        let axioms = mc.modal_axioms();
        assert!(!axioms.is_empty());
        assert!(axioms[0].contains("K:") || axioms[0].contains("K"));
    }
    #[test]
    fn test_modal_context_cohesive() {
        let mc = ModalContext::cohesive();
        assert!(mc.has_modality("♭"));
        assert!(mc.has_modality("♯"));
    }
    #[test]
    fn test_synthetic_domain_theory() {
        let sdt = SyntheticDomainTheory::standard();
        assert!(sdt.is_consistent());
        let lift = sdt.lift_description();
        assert!(lift.contains("Lift"), "got: {lift}");
        assert!(
            lift.contains("monad") || lift.contains("Monad"),
            "got: {lift}"
        );
    }
    #[test]
    fn test_synthetic_domain_partial_maps() {
        let sdt = SyntheticDomainTheory::standard();
        let pm = sdt.partial_map_description();
        assert!(
            pm.contains("partial map") || pm.contains("Partial"),
            "got: {pm}"
        );
    }
    #[test]
    fn test_build_env_new_axioms() {
        let mut env = Environment::new();
        build_env(&mut env);
        assert!(env.get(&Name::str("ObservationalEquality")).is_some());
        assert!(env.get(&Name::str("HeterogeneousEquality")).is_some());
        assert!(env.get(&Name::str("OTTCongruence")).is_some());
        assert!(env.get(&Name::str("FibrantType")).is_some());
        assert!(env.get(&Name::str("StrictFibrantType")).is_some());
        assert!(env.get(&Name::str("TwoLevelCoherence")).is_some());
        assert!(env.get(&Name::str("SetoidModel")).is_some());
        assert!(env.get(&Name::str("SetoidQuotient")).is_some());
        assert!(env.get(&Name::str("EffectiveEquality")).is_some());
        assert!(env.get(&Name::str("PCARealizability")).is_some());
        assert!(env.get(&Name::str("KleeneSecondAlgebra")).is_some());
        assert!(env.get(&Name::str("CategoricalModel")).is_some());
        assert!(env.get(&Name::str("FiberedCategory")).is_some());
        assert!(env.get(&Name::str("DisplayMap")).is_some());
        assert!(env.get(&Name::str("EdinburghLF")).is_some());
        assert!(env.get(&Name::str("TwelfSignature")).is_some());
        assert!(env.get(&Name::str("NecessityType")).is_some());
        assert!(env.get(&Name::str("PossibilityType")).is_some());
        assert!(env.get(&Name::str("GradedType")).is_some());
        assert!(env.get(&Name::str("CoeffectTyping")).is_some());
        assert!(env.get(&Name::str("ErasureSemantics")).is_some());
        assert!(env.get(&Name::str("DirectedUnivalence")).is_some());
        assert!(env.get(&Name::str("TwoCategoryType")).is_some());
        assert!(env.get(&Name::str("Dominance")).is_some());
        assert!(env.get(&Name::str("LiftMonad")).is_some());
        assert!(env.get(&Name::str("PartialMap")).is_some());
    }
}
/// Register cubical type theory primitives for OxiLean.
///
/// Cubical type theory (Cohen-Coquand-Huber-Mörtberg) provides a computational
/// interpretation of the univalence axiom using cubes and path types.
pub fn register_cubical_type_theory(env: &mut Environment) {
    let axiom_interval = Declaration::Axiom {
        name: Name::str("CubicalInterval"),
        univ_params: vec![],
        ty: type0(),
    };
    let _ = env.add(axiom_interval);
    let axiom_i0 = Declaration::Axiom {
        name: Name::str("CubicalI0"),
        univ_params: vec![],
        ty: cst("CubicalInterval"),
    };
    let _ = env.add(axiom_i0);
    let axiom_i1 = Declaration::Axiom {
        name: Name::str("CubicalI1"),
        univ_params: vec![],
        ty: cst("CubicalInterval"),
    };
    let _ = env.add(axiom_i1);
    let path_ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("A"),
        Box::new(type0()),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("_x"),
            Box::new(bvar(0)),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("_y"),
                Box::new(bvar(1)),
                Box::new(type0()),
            )),
        )),
    );
    let _ = env.add(Declaration::Axiom {
        name: Name::str("PathType"),
        univ_params: vec![],
        ty: path_ty,
    });
    let refl_path_ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("A"),
        Box::new(type0()),
        Box::new(Expr::Pi(
            BinderInfo::Implicit,
            Name::str("x"),
            Box::new(bvar(0)),
            Box::new(Expr::App(
                Box::new(Expr::App(
                    Box::new(Expr::App(Box::new(cst("PathType")), Box::new(bvar(1)))),
                    Box::new(bvar(0)),
                )),
                Box::new(bvar(0)),
            )),
        )),
    );
    let _ = env.add(Declaration::Axiom {
        name: Name::str("CubicalRefl"),
        univ_params: vec![],
        ty: refl_path_ty,
    });
    let sym_path_ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("A"),
        Box::new(type0()),
        Box::new(Expr::Pi(
            BinderInfo::Implicit,
            Name::str("x"),
            Box::new(bvar(0)),
            Box::new(Expr::Pi(
                BinderInfo::Implicit,
                Name::str("y"),
                Box::new(bvar(1)),
                Box::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("_p"),
                    Box::new(cst("Unit")),
                    Box::new(cst("Unit")),
                )),
            )),
        )),
    );
    let _ = env.add(Declaration::Axiom {
        name: Name::str("CubicalSym"),
        univ_params: vec![],
        ty: sym_path_ty,
    });
    let comp_path_ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("A"),
        Box::new(type0()),
        Box::new(Expr::Pi(
            BinderInfo::Implicit,
            Name::str("x"),
            Box::new(bvar(0)),
            Box::new(Expr::Pi(
                BinderInfo::Implicit,
                Name::str("y"),
                Box::new(bvar(1)),
                Box::new(Expr::Pi(
                    BinderInfo::Implicit,
                    Name::str("z"),
                    Box::new(bvar(2)),
                    Box::new(Expr::Pi(
                        BinderInfo::Default,
                        Name::str("_p"),
                        Box::new(cst("Unit")),
                        Box::new(Expr::Pi(
                            BinderInfo::Default,
                            Name::str("_q"),
                            Box::new(cst("Unit")),
                            Box::new(cst("Unit")),
                        )),
                    )),
                )),
            )),
        )),
    );
    let _ = env.add(Declaration::Axiom {
        name: Name::str("CubicalComp"),
        univ_params: vec![],
        ty: comp_path_ty,
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("CubicalGlue"),
        univ_params: vec![],
        ty: type0(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("CubicalUnglue"),
        univ_params: vec![],
        ty: type0(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("KanFill"),
        univ_params: vec![],
        ty: type0(),
    });
}
/// Register observational type theory primitives.
///
/// OTT (Altenkirch-McBride-Swierstra) uses a propositional equality that
/// is defined by recursion on types, making it computationally tractable.
pub fn register_observational_type_theory(env: &mut Environment) {
    let heq_ty = Expr::Pi(
        BinderInfo::Default,
        Name::str("A"),
        Box::new(type0()),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("_x"),
            Box::new(bvar(0)),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("B"),
                Box::new(type0()),
                Box::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("_y"),
                    Box::new(bvar(0)),
                    Box::new(prop()),
                )),
            )),
        )),
    );
    let _ = env.add(Declaration::Axiom {
        name: Name::str("HEq"),
        univ_params: vec![],
        ty: heq_ty,
    });
    let propeq_ty = Expr::Pi(
        BinderInfo::Default,
        Name::str("A"),
        Box::new(type0()),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("_x"),
            Box::new(bvar(0)),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("_y"),
                Box::new(bvar(1)),
                Box::new(prop()),
            )),
        )),
    );
    let _ = env.add(Declaration::Axiom {
        name: Name::str("OTTPropEq"),
        univ_params: vec![],
        ty: propeq_ty,
    });
    let coerce_ty = Expr::Pi(
        BinderInfo::Default,
        Name::str("A"),
        Box::new(type0()),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("B"),
            Box::new(type0()),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("_eq"),
                Box::new(cst("Unit")),
                Box::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("_x"),
                    Box::new(bvar(2)),
                    Box::new(bvar(3)),
                )),
            )),
        )),
    );
    let _ = env.add(Declaration::Axiom {
        name: Name::str("OTTCoerce"),
        univ_params: vec![],
        ty: coerce_ty,
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("OTTCoerceId"),
        univ_params: vec![],
        ty: Expr::Pi(
            BinderInfo::Default,
            Name::str("A"),
            Box::new(type0()),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("_x"),
                Box::new(bvar(0)),
                Box::new(prop()),
            )),
        ),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("OTTFunExt"),
        univ_params: vec![],
        ty: Expr::Pi(
            BinderInfo::Default,
            Name::str("A"),
            Box::new(type0()),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("B"),
                Box::new(type0()),
                Box::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("_f"),
                    Box::new(Expr::Pi(
                        BinderInfo::Default,
                        Name::str("_"),
                        Box::new(bvar(1)),
                        Box::new(bvar(2)),
                    )),
                    Box::new(Expr::Pi(
                        BinderInfo::Default,
                        Name::str("_g"),
                        Box::new(Expr::Pi(
                            BinderInfo::Default,
                            Name::str("_"),
                            Box::new(bvar(2)),
                            Box::new(bvar(3)),
                        )),
                        Box::new(Expr::Pi(
                            BinderInfo::Default,
                            Name::str("_h"),
                            Box::new(cst("Unit")),
                            Box::new(prop()),
                        )),
                    )),
                )),
            )),
        ),
    });
}
/// Register Higher Observational TT / Extensional XTT constructs.
pub fn register_higher_observational_tt(env: &mut Environment) {
    let id_ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("A"),
        Box::new(type0()),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("_x"),
            Box::new(bvar(0)),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("_y"),
                Box::new(bvar(1)),
                Box::new(type0()),
            )),
        )),
    );
    let _ = env.add(Declaration::Axiom {
        name: Name::str("HOTTId"),
        univ_params: vec![],
        ty: id_ty,
    });
    let j_ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("A"),
        Box::new(type0()),
        Box::new(Expr::Pi(
            BinderInfo::Implicit,
            Name::str("x"),
            Box::new(bvar(0)),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("_P"),
                Box::new(cst("Unit")),
                Box::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("_r"),
                    Box::new(cst("Unit")),
                    Box::new(Expr::Pi(
                        BinderInfo::Default,
                        Name::str("_y"),
                        Box::new(bvar(3)),
                        Box::new(Expr::Pi(
                            BinderInfo::Default,
                            Name::str("_p"),
                            Box::new(cst("Unit")),
                            Box::new(cst("Unit")),
                        )),
                    )),
                )),
            )),
        )),
    );
    let _ = env.add(Declaration::Axiom {
        name: Name::str("HOTTJElim"),
        univ_params: vec![],
        ty: j_ty,
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("HOTTJRegularity"),
        univ_params: vec![],
        ty: Expr::Pi(
            BinderInfo::Implicit,
            Name::str("A"),
            Box::new(type0()),
            Box::new(Expr::Const(Name::str("True"), vec![])),
        ),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("HOTTAp"),
        univ_params: vec![],
        ty: Expr::Pi(
            BinderInfo::Implicit,
            Name::str("A"),
            Box::new(type0()),
            Box::new(Expr::Pi(
                BinderInfo::Implicit,
                Name::str("B"),
                Box::new(type0()),
                Box::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("_f"),
                    Box::new(Expr::Pi(
                        BinderInfo::Default,
                        Name::str("_"),
                        Box::new(bvar(1)),
                        Box::new(bvar(2)),
                    )),
                    Box::new(Expr::Pi(
                        BinderInfo::Implicit,
                        Name::str("x"),
                        Box::new(bvar(2)),
                        Box::new(Expr::Pi(
                            BinderInfo::Implicit,
                            Name::str("y"),
                            Box::new(bvar(3)),
                            Box::new(Expr::Pi(
                                BinderInfo::Default,
                                Name::str("_p"),
                                Box::new(cst("Unit")),
                                Box::new(cst("Unit")),
                            )),
                        )),
                    )),
                )),
            )),
        ),
    });
    let trunc_ty = Expr::Pi(
        BinderInfo::Default,
        Name::str("A"),
        Box::new(type0()),
        Box::new(type0()),
    );
    let _ = env.add(Declaration::Axiom {
        name: Name::str("PropTrunc"),
        univ_params: vec![],
        ty: trunc_ty,
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("PropTruncIntro"),
        univ_params: vec![],
        ty: Expr::Pi(
            BinderInfo::Implicit,
            Name::str("A"),
            Box::new(type0()),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("_x"),
                Box::new(bvar(0)),
                Box::new(Expr::App(Box::new(cst("PropTrunc")), Box::new(bvar(1)))),
            )),
        ),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("PropTruncElim"),
        univ_params: vec![],
        ty: Expr::Pi(
            BinderInfo::Implicit,
            Name::str("A"),
            Box::new(type0()),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("_P"),
                Box::new(prop()),
                Box::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("_f"),
                    Box::new(cst("Unit")),
                    Box::new(Expr::Pi(
                        BinderInfo::Default,
                        Name::str("_x"),
                        Box::new(Expr::App(Box::new(cst("PropTrunc")), Box::new(bvar(3)))),
                        Box::new(bvar(3)),
                    )),
                )),
            )),
        ),
    });
}
#[cfg(test)]
mod tests_type_theory_extended {
    use super::*;
    use oxilean_kernel::{Environment, Name};
    #[test]
    fn test_cubical_type_theory_registered() {
        let mut env = Environment::new();
        register_cubical_type_theory(&mut env);
        assert!(env.get(&Name::str("CubicalInterval")).is_some());
        assert!(env.get(&Name::str("CubicalI0")).is_some());
        assert!(env.get(&Name::str("CubicalI1")).is_some());
        assert!(env.get(&Name::str("PathType")).is_some());
        assert!(env.get(&Name::str("CubicalRefl")).is_some());
        assert!(env.get(&Name::str("CubicalSym")).is_some());
        assert!(env.get(&Name::str("CubicalComp")).is_some());
        assert!(env.get(&Name::str("KanFill")).is_some());
    }
    #[test]
    fn test_observational_type_theory_registered() {
        let mut env = Environment::new();
        register_observational_type_theory(&mut env);
        assert!(env.get(&Name::str("HEq")).is_some());
        assert!(env.get(&Name::str("OTTPropEq")).is_some());
        assert!(env.get(&Name::str("OTTCoerce")).is_some());
        assert!(env.get(&Name::str("OTTCoerceId")).is_some());
        assert!(env.get(&Name::str("OTTFunExt")).is_some());
    }
    #[test]
    fn test_higher_observational_tt_registered() {
        let mut env = Environment::new();
        register_higher_observational_tt(&mut env);
        assert!(env.get(&Name::str("HOTTId")).is_some());
        assert!(env.get(&Name::str("HOTTJElim")).is_some());
        assert!(env.get(&Name::str("HOTTJRegularity")).is_some());
        assert!(env.get(&Name::str("HOTTAp")).is_some());
        assert!(env.get(&Name::str("PropTrunc")).is_some());
        assert!(env.get(&Name::str("PropTruncIntro")).is_some());
        assert!(env.get(&Name::str("PropTruncElim")).is_some());
    }
}
