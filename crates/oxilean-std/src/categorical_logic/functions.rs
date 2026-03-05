//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{
    AssemblyCategory, BeckChevalleyChecker, CoherenceTheorem, ComprehensionCategory,
    DialecticaCategory, DisplayMapCategory, DoctrineType, EnrichedCategory, GameSemanticsCategory,
    GrothendieckFibration, GrothendieckFibrationImpl, HigherToposType, HyperdoctrineModel,
    HyperdoctrineType, IndexedCategory, InstitutionMorphism, InstitutionType, InternalLogic,
    LawvereTheory, ModalLogicCategory, Morphism, ParametricityModel, PolymorphismCategory,
    RealizabilityInterpreter, StarAutonomousCategory, StringDiagramComposer,
    TracedMonoidalCategory, TriposType, TypeTheoryInterpretation,
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
/// Lawvere's completeness theorem for hyperdoctrines.
pub fn lawvere_completeness_theorem() -> &'static str {
    "Lawvere completeness: A formula φ is provable in first-order logic \
     if and only if it is valid in all hyperdoctrines (with values in sets). \
     This is the categorical version of Gödel completeness, using the \
     syntactic hyperdoctrine as a universal model."
}
/// The Morita equivalence of Lawvere theories.
///
/// Two Lawvere theories T and T' are Morita equivalent if their categories
/// of models in Set are equivalent. This is the categorical analogue of
/// Morita equivalence for rings.
pub fn morita_equivalence_theorem() -> &'static str {
    "Morita equivalence for Lawvere theories: Two Lawvere theories T and T' \
     are Morita equivalent if and only if their categories of models Mod(T, Set) \
     and Mod(T', Set) are equivalent as categories. \
     This generalises ring Morita equivalence to arbitrary algebraic theories."
}
/// The effective topos (Hyland, 1982).
pub fn effective_topos_existence() -> &'static str {
    "Hyland's effective topos Eff: The effective topos is the topos \
     constructed from the realizability tripos over Kleene's first PCA (ℕ, ·). \
     In Eff: (1) every function ℕ → ℕ is computable, \
     (2) Church's thesis holds (CT), \
     (3) the axiom of choice fails, \
     (4) there exist non-classical truth values. \
     Eff is the canonical model for constructive mathematics + Church's thesis."
}
/// Lurie's ∞-topos descent theorem.
pub fn lurie_descent_theorem() -> &'static str {
    "Lurie descent theorem (HTT §6.1): An ∞-category X is an ∞-topos if and only if \
     (1) X is a presentable ∞-category, \
     (2) colimits in X are universal (stable under arbitrary base change), \
     (3) groupoid objects in X are effective. \
     In an ∞-topos every descent datum is effective: sheaves on X satisfy \
     all higher homotopy coherences automatically."
}
/// GrothendieckFibration : String → String → Prop
pub fn grothendieck_fibration_ty() -> Expr {
    arrow(string_ty(), arrow(string_ty(), prop()))
}
/// CartesianLifting : String → String → String → Prop
pub fn cartesian_lifting_ty() -> Expr {
    arrow(string_ty(), arrow(string_ty(), arrow(string_ty(), prop())))
}
/// IndexedCategory : String → Prop
pub fn indexed_category_ty() -> Expr {
    arrow(string_ty(), prop())
}
/// ReindexingFunctor : String → String → String → Prop
pub fn reindexing_functor_ty() -> Expr {
    arrow(string_ty(), arrow(string_ty(), arrow(string_ty(), prop())))
}
/// StarAutonomousCategory : String → Prop
pub fn star_autonomous_ty() -> Expr {
    arrow(string_ty(), prop())
}
/// LinearLogicDuality : String → String → Prop (A, B models de Morgan duality)
pub fn linear_logic_duality_ty() -> Expr {
    arrow(string_ty(), arrow(string_ty(), prop()))
}
/// DialecticaObject : String → String → Prop (U, X pair)
pub fn dialectica_object_ty() -> Expr {
    arrow(string_ty(), arrow(string_ty(), prop()))
}
/// DialecticaTensor : String → String → String → Prop
pub fn dialectica_tensor_ty() -> Expr {
    arrow(string_ty(), arrow(string_ty(), arrow(string_ty(), prop())))
}
/// GameSemanticsCategory : String → Bool → Prop
pub fn game_semantics_ty() -> Expr {
    arrow(string_ty(), arrow(bool_ty(), prop()))
}
/// StrategyComposition : String → String → String → Prop
pub fn strategy_composition_ty() -> Expr {
    arrow(string_ty(), arrow(string_ty(), arrow(string_ty(), prop())))
}
/// AssemblyObject : String → Prop (PCA name → Prop)
pub fn assembly_object_ty() -> Expr {
    arrow(string_ty(), prop())
}
/// TrackedMorphism : String → String → String → Prop
pub fn tracked_morphism_ty() -> Expr {
    arrow(string_ty(), arrow(string_ty(), arrow(string_ty(), prop())))
}
/// DisplayMapType : String → String → Prop (context, type → Prop)
pub fn display_map_ty() -> Expr {
    arrow(string_ty(), arrow(string_ty(), prop()))
}
/// DependentProduct : String → String → String → Prop
pub fn dependent_product_ty() -> Expr {
    arrow(string_ty(), arrow(string_ty(), arrow(string_ty(), prop())))
}
/// ComprehensionExtension : String → String → String → Prop
pub fn comprehension_extension_ty() -> Expr {
    arrow(string_ty(), arrow(string_ty(), arrow(string_ty(), prop())))
}
/// SeelyIsomorphism : String → String → String → Prop
pub fn seely_iso_ty() -> Expr {
    arrow(string_ty(), arrow(string_ty(), arrow(string_ty(), prop())))
}
/// SystemFType : String → Prop
pub fn system_f_type_ty() -> Expr {
    arrow(string_ty(), prop())
}
/// CoherenceTheorem : String → Bool → Prop
pub fn coherence_theorem_ty() -> Expr {
    arrow(string_ty(), arrow(bool_ty(), prop()))
}
/// BraidedCoherence : Nat → Prop (braid group B_n)
pub fn braided_coherence_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// EnrichedCategory : String → String → Prop (category, enriching)
pub fn enriched_category_ty() -> Expr {
    arrow(string_ty(), arrow(string_ty(), prop()))
}
/// VProfunctor : String → String → String → Prop
pub fn v_profunctor_ty() -> Expr {
    arrow(string_ty(), arrow(string_ty(), arrow(string_ty(), prop())))
}
/// DayConvolution : String → Prop
pub fn day_convolution_ty() -> Expr {
    arrow(string_ty(), prop())
}
/// TracedMonoidalCategory : String → Prop
pub fn traced_monoidal_ty() -> Expr {
    arrow(string_ty(), prop())
}
/// TraceOperation : String → String → String → Prop
pub fn trace_operation_ty() -> Expr {
    arrow(string_ty(), arrow(string_ty(), arrow(string_ty(), prop())))
}
/// CompactClosedCategory : String → Prop
pub fn compact_closed_ty() -> Expr {
    arrow(string_ty(), prop())
}
/// ParametricityModel : String → Prop
pub fn parametricity_model_ty() -> Expr {
    arrow(string_ty(), prop())
}
/// AbstractionTheorem : String → Prop
pub fn abstraction_theorem_ty() -> Expr {
    arrow(string_ty(), prop())
}
/// FreeTheorem : String → String → Prop
pub fn free_theorem_ty() -> Expr {
    arrow(string_ty(), arrow(string_ty(), prop()))
}
/// ModalComonad : String → String → Prop (category, modal logic)
pub fn modal_comonad_ty() -> Expr {
    arrow(string_ty(), arrow(string_ty(), prop()))
}
/// NecessityAxioms : String → Prop
pub fn necessity_axioms_ty() -> Expr {
    arrow(string_ty(), prop())
}
/// CurryHowardModal : String → String → Prop
pub fn curry_howard_modal_ty() -> Expr {
    arrow(string_ty(), arrow(string_ty(), prop()))
}
/// GrothendieckConstruction : String → Prop
pub fn grothendieck_construction_ty() -> Expr {
    arrow(string_ty(), prop())
}
/// IntConstruction : String → Prop (traced → compact closed)
pub fn int_construction_ty() -> Expr {
    arrow(string_ty(), prop())
}
/// EnrichedYoneda : String → Prop
pub fn enriched_yoneda_ty() -> Expr {
    arrow(string_ty(), prop())
}
/// RealizabilityInterpretation : String → String → Prop
pub fn realizability_interpretation_ty() -> Expr {
    arrow(string_ty(), arrow(string_ty(), prop()))
}
/// PartitionedAssembly : String → Prop
pub fn partitioned_assembly_ty() -> Expr {
    arrow(string_ty(), prop())
}
/// Type of the Hyperdoctrine axiom: (String × String) → Prop
pub fn hyperdoctrine_ty() -> Expr {
    arrow(string_ty(), arrow(string_ty(), prop()))
}
/// SubstitutionFunctor : String → String → String → Prop
pub fn substitution_functor_ty() -> Expr {
    arrow(string_ty(), arrow(string_ty(), arrow(string_ty(), prop())))
}
/// InternalLogicType : String → Prop
pub fn internal_logic_ty() -> Expr {
    arrow(string_ty(), prop())
}
/// LawvereTheory : List String → Prop (takes a list of op-names)
pub fn lawvere_theory_ty() -> Expr {
    arrow(list_ty(string_ty()), prop())
}
/// FreeModel : List String → Nat → Prop
pub fn free_model_ty() -> Expr {
    arrow(list_ty(string_ty()), arrow(nat_ty(), prop()))
}
/// DoctrineIsCartesian : Bool
pub fn doctrine_cartesian_ty() -> Expr {
    bool_ty()
}
/// DoctrineHasQuantifiers : Bool
pub fn doctrine_quantifiers_ty() -> Expr {
    bool_ty()
}
/// TriposType : Bool → Prop
pub fn tripos_ty() -> Expr {
    arrow(bool_ty(), prop())
}
/// TriposToTopos : Bool → String (returns topos name)
pub fn tripos_to_topos_ty() -> Expr {
    arrow(bool_ty(), string_ty())
}
/// InstitutionSignMorphisms : String → Prop
pub fn institution_sign_morphisms_ty() -> Expr {
    arrow(string_ty(), prop())
}
/// SatisfactionCondition : String → Prop
pub fn satisfaction_condition_ty() -> Expr {
    arrow(string_ty(), prop())
}
/// GrothendieckInstitution : String → Prop
pub fn grothendieck_institution_ty() -> Expr {
    arrow(string_ty(), prop())
}
/// LCCCCorrespondence : String → String → Prop
pub fn lccc_correspondence_ty() -> Expr {
    arrow(string_ty(), arrow(string_ty(), prop()))
}
/// InitialityConjecture : String → String → Prop
pub fn initiality_conjecture_ty() -> Expr {
    arrow(string_ty(), arrow(string_ty(), prop()))
}
/// ObjectClassifier : Bool → Prop
pub fn object_classifier_ty() -> Expr {
    arrow(bool_ty(), prop())
}
/// Descent : Bool → Bool → Prop
pub fn descent_ty() -> Expr {
    arrow(bool_ty(), arrow(bool_ty(), prop()))
}
/// ComprehensionCategory : String → String → Prop
pub fn comprehension_category_ty() -> Expr {
    arrow(string_ty(), arrow(string_ty(), prop()))
}
/// MitchellBenabou : String → Prop
pub fn mitchell_benabou_ty() -> Expr {
    arrow(string_ty(), prop())
}
/// AlgebraicTheory : List String → Nat → Prop (ops × arity → Prop)
pub fn algebraic_theory_ty() -> Expr {
    arrow(list_ty(string_ty()), arrow(nat_ty(), prop()))
}
/// RealizabilityTripos : String → Prop
pub fn realizability_tripos_ty() -> Expr {
    arrow(string_ty(), prop())
}
/// InstitutionMorphism : String → String → Prop
pub fn institution_morphism_ty() -> Expr {
    arrow(string_ty(), arrow(string_ty(), prop()))
}
/// UnivalenceHolds : Bool → Prop
pub fn univalence_holds_ty() -> Expr {
    arrow(bool_ty(), prop())
}
/// HigherToposGiraudAxioms : Bool → Nat → Prop (topos_flag × axiom_index)
pub fn higher_topos_axiom_ty() -> Expr {
    arrow(bool_ty(), arrow(nat_ty(), prop()))
}
/// LawvereCompletenessThm : String → String → Prop
pub fn lawvere_completeness_ty() -> Expr {
    arrow(string_ty(), arrow(string_ty(), prop()))
}
/// EffectiveTopos : Prop (constant)
pub fn effective_topos_ty() -> Expr {
    prop()
}
/// BeckChevalley : String → String → Prop
pub fn beck_chevalley_ty() -> Expr {
    arrow(string_ty(), arrow(string_ty(), prop()))
}
/// FrobeniusReciprocity : String → String → Prop
pub fn frobenius_ty() -> Expr {
    arrow(string_ty(), arrow(string_ty(), prop()))
}
/// DoctrineType : Prop (a universe of doctrine types)
pub fn doctrine_type_ty() -> Expr {
    type0()
}
/// InstitutionType : Prop (a universe of institution types)
pub fn institution_type_ty() -> Expr {
    type0()
}
/// HigherToposType : Prop (a universe of higher topos types)
pub fn higher_topos_type_ty() -> Expr {
    type0()
}
/// TypeTheoryInterp : String → String → Bool → Prop
pub fn type_theory_interp_ty() -> Expr {
    arrow(string_ty(), arrow(string_ty(), arrow(bool_ty(), prop())))
}
/// Register all categorical logic axioms into the given environment.
pub fn build_env(env: &mut Environment) {
    let axioms: &[(&str, Expr)] = &[
        ("Hyperdoctrine", hyperdoctrine_ty()),
        ("SubstitutionFunctor", substitution_functor_ty()),
        ("BeckChevalleyCondition", beck_chevalley_ty()),
        ("FrobeniusReciprocity", frobenius_ty()),
        ("ComprehensionCategory", comprehension_category_ty()),
        ("LawvereCompletenessThm", lawvere_completeness_ty()),
        ("InternalLogic", internal_logic_ty()),
        ("MitchellBenabouThm", mitchell_benabou_ty()),
        ("EffectiveTopos", effective_topos_ty()),
        ("LawvereTheory", lawvere_theory_ty()),
        ("FreeModelOnGenerators", free_model_ty()),
        ("AlgebraicTheoryPresentation", algebraic_theory_ty()),
        ("DoctrineType", doctrine_type_ty()),
        ("DoctrineIsCartesian", doctrine_cartesian_ty()),
        ("DoctrineHasQuantifiers", doctrine_quantifiers_ty()),
        ("TriposType", tripos_ty()),
        ("TriposToToposConstruction", tripos_to_topos_ty()),
        ("RealizabilityTripos", realizability_tripos_ty()),
        ("InstitutionType", institution_type_ty()),
        ("InstitutionSignMorphisms", institution_sign_morphisms_ty()),
        ("SatisfactionCondition", satisfaction_condition_ty()),
        ("GrothendieckInstitution", grothendieck_institution_ty()),
        ("InstitutionMorphism", institution_morphism_ty()),
        ("LCCCCorrespondence", lccc_correspondence_ty()),
        ("InitialityConjecture", initiality_conjecture_ty()),
        ("TypeTheoryInterpretation", type_theory_interp_ty()),
        ("HigherToposType", higher_topos_type_ty()),
        ("ObjectClassifier", object_classifier_ty()),
        ("DescentCondition", descent_ty()),
        ("UnivalenceHolds", univalence_holds_ty()),
        ("HigherToposGiraudAxiom", higher_topos_axiom_ty()),
        ("GrothendieckFibration", grothendieck_fibration_ty()),
        ("CartesianLifting", cartesian_lifting_ty()),
        ("IndexedCategory", indexed_category_ty()),
        ("ReindexingFunctor", reindexing_functor_ty()),
        ("GrothendieckConstruction", grothendieck_construction_ty()),
        ("StarAutonomousCategory", star_autonomous_ty()),
        ("LinearLogicDuality", linear_logic_duality_ty()),
        ("DialecticaObject", dialectica_object_ty()),
        ("DialecticaTensor", dialectica_tensor_ty()),
        ("GameSemanticsCategory", game_semantics_ty()),
        ("StrategyComposition", strategy_composition_ty()),
        ("AssemblyObject", assembly_object_ty()),
        ("TrackedMorphism", tracked_morphism_ty()),
        (
            "RealizabilityInterpretation",
            realizability_interpretation_ty(),
        ),
        ("PartitionedAssembly", partitioned_assembly_ty()),
        ("DisplayMapType", display_map_ty()),
        ("DependentProduct", dependent_product_ty()),
        ("ComprehensionExtension", comprehension_extension_ty()),
        ("SeelyIsomorphism", seely_iso_ty()),
        ("SystemFType", system_f_type_ty()),
        ("CoherenceTheorem", coherence_theorem_ty()),
        ("BraidedCoherence", braided_coherence_ty()),
        ("EnrichedCategory", enriched_category_ty()),
        ("VProfunctor", v_profunctor_ty()),
        ("DayConvolution", day_convolution_ty()),
        ("EnrichedYoneda", enriched_yoneda_ty()),
        ("TracedMonoidalCategory", traced_monoidal_ty()),
        ("TraceOperation", trace_operation_ty()),
        ("CompactClosedCategory", compact_closed_ty()),
        ("IntConstruction", int_construction_ty()),
        ("ParametricityModel", parametricity_model_ty()),
        ("AbstractionTheorem", abstraction_theorem_ty()),
        ("FreeTheorem", free_theorem_ty()),
        ("ModalComonad", modal_comonad_ty()),
        ("NecessityAxioms", necessity_axioms_ty()),
        ("CurryHowardModal", curry_howard_modal_ty()),
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
    fn test_hyperdoctrine_substitution_functor() {
        let h = HyperdoctrineType::new("Set", "HeytAlg");
        let sf = h.substitution_functor();
        assert!(sf.contains("SubstitutionFunctor"));
        assert!(sf.contains("Set"));
    }
    #[test]
    fn test_hyperdoctrine_comprehension_category() {
        let h = HyperdoctrineType::new("C", "Heyting");
        let cc = h.comprehension_category();
        assert!(cc.contains("Comprehension category"));
        assert!(cc.contains("C"));
    }
    #[test]
    fn test_internal_logic_mitchell_benabou() {
        let il = InternalLogic::new("Sh(X)");
        let mb = il.mitchell_benabou();
        assert!(mb.contains("Mitchell-Bénabou"));
        assert!(mb.contains("Sh(X)"));
    }
    #[test]
    fn test_internal_logic_language() {
        let il = InternalLogic::for_sets();
        let lang = il.internal_language();
        assert!(lang.contains("classical"));
        assert!(il.excluded_middle_holds());
    }
    #[test]
    fn test_lawvere_theory_algebraic() {
        let lt = LawvereTheory::groups();
        let desc = lt.algebraic_theory();
        assert!(desc.contains("Group"));
        assert!(desc.contains("mul/2"));
    }
    #[test]
    fn test_lawvere_theory_free_model() {
        let lt = LawvereTheory::rings();
        let fm = lt.free_model(3);
        assert!(fm.contains("Ring"));
        assert!(fm.contains("3"));
    }
    #[test]
    fn test_doctrine_type() {
        assert!(DoctrineType::Predicate.is_cartesian());
        assert!(DoctrineType::Fibered.has_quantifiers());
        assert!(DoctrineType::Indexed.is_hyperdoctrine());
    }
    #[test]
    fn test_tripos_realizability() {
        let t = TriposType::realizability("Kleene");
        assert!(t.pca_based());
        assert!(t.is_effective);
        let rt = t.realizability_tripos();
        assert!(rt.contains("Kleene"));
    }
    #[test]
    fn test_tripos_to_topos() {
        let t = TriposType::realizability("K1");
        let topos = t.induced_topos();
        assert!(topos.contains("Eff"));
    }
    #[test]
    fn test_institution_satisfaction() {
        let inst = InstitutionType::fol();
        let sc = inst.satisfaction_condition();
        assert!(sc.contains("Satisfaction condition"));
        assert!(sc.contains("FOL"));
    }
    #[test]
    fn test_institution_grothendieck() {
        let inst = InstitutionType::new("CASL");
        let gi = inst.grothendieck_institution();
        assert!(gi.contains("Grothendieck institution"));
        assert!(gi.contains("CASL"));
    }
    #[test]
    fn test_institution_sign_morphisms() {
        let inst = InstitutionType::fol();
        let sm = inst.sign_morphisms();
        assert!(sm.contains("Signature morphisms"));
    }
    #[test]
    fn test_type_theory_lccc() {
        let tti = TypeTheoryInterpretation::new("MLTT", "CwF");
        let corr = tti.lccc_correspondence();
        assert!(corr.contains("LCCC"));
        assert!(corr.contains("MLTT"));
    }
    #[test]
    fn test_type_theory_initiality() {
        let tti = TypeTheoryInterpretation::new("MLTT", "LCC");
        let ic = tti.initiality_conjecture();
        assert!(ic.contains("Initiality conjecture"));
    }
    #[test]
    fn test_higher_topos_object_classifier() {
        let ht = HigherToposType::new(true);
        let oc = ht.object_classifier();
        assert!(oc.contains("Object classifier"));
        assert!(ht.univalence_holds());
    }
    #[test]
    fn test_higher_topos_descent() {
        let ht = HigherToposType::spaces();
        let desc = ht.descent();
        assert!(desc.contains("Descent holds"));
        assert!(ht.has_descent);
    }
    #[test]
    fn test_build_env() {
        let mut env = Environment::new();
        build_env(&mut env);
        assert!(env.get(&Name::str("Hyperdoctrine")).is_some());
        assert!(env.get(&Name::str("LCCCCorrespondence")).is_some());
        assert!(env.get(&Name::str("TriposType")).is_some());
        assert!(env.get(&Name::str("ObjectClassifier")).is_some());
        assert!(env.get(&Name::str("SatisfactionCondition")).is_some());
        assert!(env.get(&Name::str("GrothendieckFibration")).is_some());
        assert!(env.get(&Name::str("StarAutonomousCategory")).is_some());
        assert!(env.get(&Name::str("DialecticaObject")).is_some());
        assert!(env.get(&Name::str("GameSemanticsCategory")).is_some());
        assert!(env.get(&Name::str("AssemblyObject")).is_some());
        assert!(env.get(&Name::str("DisplayMapType")).is_some());
        assert!(env.get(&Name::str("SeelyIsomorphism")).is_some());
        assert!(env.get(&Name::str("CoherenceTheorem")).is_some());
        assert!(env.get(&Name::str("EnrichedCategory")).is_some());
        assert!(env.get(&Name::str("TracedMonoidalCategory")).is_some());
        assert!(env.get(&Name::str("ParametricityModel")).is_some());
        assert!(env.get(&Name::str("ModalComonad")).is_some());
        assert!(env.get(&Name::str("IntConstruction")).is_some());
        assert!(env.get(&Name::str("FreeTheorem")).is_some());
        assert!(env.get(&Name::str("RealizabilityInterpretation")).is_some());
    }
    #[test]
    fn test_grothendieck_fibration() {
        let fib = GrothendieckFibration::new("E", "B");
        let lift = fib.cartesian_lifting("f");
        assert!(lift.contains("Cartesian lifting"));
        assert!(lift.contains("f"));
        let fiber = fib.fiber_over("I");
        assert!(fiber.contains("Fiber"));
        assert!(fiber.contains("I"));
    }
    #[test]
    fn test_grothendieck_fibration_split() {
        let fib = GrothendieckFibration::split("Fam(Set)", "Set");
        assert!(fib.is_split);
        assert!(fib.satisfies_beck_chevalley());
    }
    #[test]
    fn test_grothendieck_construction() {
        let gc = GrothendieckFibration::grothendieck_construction("F : B^op → Cat");
        assert!(gc.contains("Grothendieck construction"));
        assert!(gc.contains("∫F"));
    }
    #[test]
    fn test_indexed_category() {
        let ic = IndexedCategory::new("Set");
        let rf = ic.reindexing_functor("f", "J", "I");
        assert!(rf.contains("Reindexing functor"));
        assert!(rf.contains("f*"));
        let ge = ic.grothendieck_equivalence();
        assert!(ge.contains("Grothendieck equivalence"));
    }
    #[test]
    fn test_star_autonomous_category() {
        let sa = StarAutonomousCategory::new("Rel");
        let dn = sa.double_negation("A");
        assert!(dn.contains("Double-negation"));
        assert!(dn.contains("A"));
        let dm = sa.de_morgan_duality("A", "B");
        assert!(dm.contains("De Morgan"));
        assert!(sa.models_mll());
        let g = StarAutonomousCategory::girard_translation();
        assert!(g.contains("Girard"));
    }
    #[test]
    fn test_dialectica_category() {
        let dc = DialecticaCategory::new("Set");
        let obj = dc.objects_description();
        assert!(obj.contains("witnesses"));
        let t = dc.tensor_product("A", "B");
        assert!(t.contains("Tensor"));
        let interp = dc.dialectica_interpretation_ha();
        assert!(interp.contains("Dialectica interpretation"));
    }
    #[test]
    fn test_game_semantics_category() {
        let g = GameSemanticsCategory::hyland_ong();
        assert!(g.innocent);
        assert!(g.pcf_full_abstraction());
        let bg = g.base_game("Int");
        assert!(bg.contains("Int"));
        let sc = g.strategy_composition("s", "t");
        assert!(sc.contains("Composition s ; t"));
    }
    #[test]
    fn test_assembly_category() {
        let asm = AssemblyCategory::new("K1");
        let od = asm.objects_description();
        assert!(od.contains("K1"));
        let md = asm.morphisms_description();
        assert!(md.contains("tracks"));
        let et = asm.effective_topos_description();
        assert!(et.contains("Eff(K1)"));
    }
    #[test]
    fn test_assembly_partitioned() {
        let asm = AssemblyCategory::partitioned("K2");
        assert!(asm.partitioned);
        let od = asm.objects_description();
        assert!(od.contains("Partitioned"));
    }
    #[test]
    fn test_display_map_category() {
        let dm = DisplayMapCategory::new("C", "fibrations");
        let t = dm.types_as_display_maps();
        assert!(t.contains("Types in display map category"));
        let dp = dm.dependent_products();
        assert!(dp.contains("Dependent products"));
    }
    #[test]
    fn test_comprehension_category() {
        let cc = ComprehensionCategory::new("Fam", "Ctx");
        let op = cc.comprehension_operation("A");
        assert!(op.contains("Comprehension"));
        assert!(op.contains("A"));
    }
    #[test]
    fn test_polymorphism_category() {
        let pc = PolymorphismCategory::new("CCC");
        let si = pc.seely_isomorphism("A", "τ");
        assert!(si.contains("Seely"));
        assert!(si.contains("τ"));
        let ft = pc.system_f_types();
        assert!(ft.contains("System F"));
    }
    #[test]
    fn test_coherence_theorems() {
        let ct = CoherenceTheorem::mac_lane_monoidal();
        assert!(ct.is_strict);
        let ac = ct.apply_coherence("associativity pentagon");
        assert!(ac.contains("commutes"));
        let ks = CoherenceTheorem::kelly_symmetric();
        assert!(!ks.is_strict);
        let bc = CoherenceTheorem::braided_coherence();
        assert!(bc.statement.contains("braid"));
    }
    #[test]
    fn test_enriched_category() {
        let ec = EnrichedCategory::new("Ab-Cat", "Ab");
        let prof = ec.profunctor("C", "D");
        assert!(prof.contains("V-profunctor"));
        let ey = ec.enriched_yoneda();
        assert!(ey.contains("Yoneda"));
        let dc = ec.day_convolution();
        assert!(dc.contains("Day convolution"));
    }
    #[test]
    fn test_traced_monoidal_category() {
        let tmc = TracedMonoidalCategory::new("VecK");
        let tr = tmc.trace_operation("f", "U");
        assert!(tr.contains("Trace"));
        let ic = tmc.int_construction();
        assert!(ic.contains("Int("));
        let sdr = tmc.string_diagram_rewriting("sliding");
        assert!(sdr.contains("sliding"));
    }
    #[test]
    fn test_compact_closed_category() {
        let ccc = TracedMonoidalCategory::compact_closed("FdVec");
        assert!(ccc.compact_closed);
        let tr = ccc.trace_operation("M", "V");
        assert!(tr.contains("Trace"));
    }
    #[test]
    fn test_parametricity_model() {
        let pm = ParametricityModel::new("SystemF");
        assert!(pm.parametricity_holds);
        let at = pm.abstraction_theorem();
        assert!(at.contains("abstraction theorem"));
        let ft = pm.free_theorem("∀X. X → X");
        assert!(ft.contains("Free theorem"));
        let din = pm.dinaturality_description();
        assert!(din.contains("Dinaturality"));
    }
    #[test]
    fn test_modal_logic_category() {
        let ml = ModalLogicCategory::s4_model();
        assert_eq!(ml.modal_logic, "S4");
        let na = ml.necessity_axioms();
        assert!(na.contains("Counit"));
        let ch = ml.curry_howard_s4();
        assert!(ch.contains("Curry-Howard"));
        let pw = ml.possible_worlds();
        assert!(pw.contains("Kripke"));
    }
    #[test]
    fn test_beck_chevalley_checker() {
        let mut bc = BeckChevalleyChecker::new(true);
        let result = bc.check_square("f", "g", "f'", "g'");
        assert!(result);
        assert_eq!(bc.num_checked(), 1);
        let results = bc.results();
        assert!(results[0].contains("PASS"));
        let frobenius = bc.check_frobenius("p", "phi", "psi");
        assert!(frobenius);
    }
    #[test]
    fn test_hyperdoctrine_model() {
        let mut hm = HyperdoctrineModel::new("LindenbaummTarski");
        hm.add_predicate("Even");
        hm.add_predicate("Prime");
        assert!(hm.has_predicate("Even"));
        assert!(!hm.has_predicate("Odd"));
        assert_eq!(hm.num_predicates(), 2);
        let es = hm.eval_substitution("Even(x)", "succ");
        assert!(es.contains("Substitution"));
        let ee = hm.eval_exists("x", "Even(x)");
        assert!(ee.contains("∃x"));
    }
    #[test]
    fn test_string_diagram_composer() {
        let mut sdc = StringDiagramComposer::new("Vect");
        let f = Morphism::new("f", "A", "B");
        let g = Morphism::new("g", "B", "C");
        sdc.add_morphism(f.clone());
        sdc.add_morphism(g.clone());
        assert_eq!(sdc.num_morphisms(), 2);
        let fg = sdc.sequential(&f, &g);
        assert!(fg.is_some());
        let fg = fg.expect("fg should be valid");
        assert_eq!(fg.domain, "A");
        assert_eq!(fg.codomain, "C");
        let par = sdc.parallel(&f, &g);
        assert!(par.domain.contains("A ⊗ B"));
        let id = StringDiagramComposer::identity("X");
        assert_eq!(id.name, "id_X");
        let rendered = sdc.render();
        assert!(rendered.contains("Vect"));
    }
    #[test]
    fn test_realizability_interpreter() {
        let mut ri = RealizabilityInterpreter::new("Kleene");
        ri.realize("True", 0);
        assert!(ri.is_realized("True"));
        assert_eq!(ri.realizer_of("True"), Some(0));
        assert!(!ri.is_realized("False"));
        ri.realize_and("A", 2, "B", 3);
        assert!(ri.is_realized("A ∧ B"));
        ri.realize_implication("A", "B", 42);
        assert!(ri.is_realized("A → B"));
        ri.realize_exists("∃x.P(x)", 1, 5);
        assert!(ri.is_realized("∃x.P(x)"));
        assert_eq!(ri.num_realized(), 4);
    }
    #[test]
    fn test_grothendieck_fibration_impl() {
        let mut gf = GrothendieckFibrationImpl::new("E", "B");
        gf.add_cartesian_lift("e'", "f", "e");
        gf.add_cartesian_lift("x'", "g", "x");
        assert_eq!(gf.num_lifts(), 2);
        let lift = gf.cartesian_lift_over("f");
        assert!(lift.is_some());
        let (src, tgt) = lift.expect("lift should be valid");
        assert_eq!(src, "e'");
        assert_eq!(tgt, "e");
        assert!(gf.is_bifibration());
        let desc = gf.describe();
        assert!(desc.contains("E"));
        assert!(desc.contains("B"));
        assert!(desc.contains("2"));
    }
}
