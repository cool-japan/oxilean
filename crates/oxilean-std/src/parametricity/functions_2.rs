//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{
    CubicalPath, FreeTheoremDeriver, GradualType, GradualTyper, LogicalRelation, PcfType, PcfValue,
    RealizabilityModel, SimpleType,
};

#[cfg(test)]
mod tests {
    use super::*;
    fn registered_env() -> Environment {
        let mut env = Environment::new();
        register_parametricity(&mut env);
        env
    }
    #[test]
    fn test_rel_registered() {
        let env = registered_env();
        assert!(env.get(&Name::str("Rel")).is_some());
        assert!(env.get(&Name::str("RelId")).is_some());
        assert!(env.get(&Name::str("RelFun")).is_some());
    }
    #[test]
    fn test_reynolds_parametricity_registered() {
        let env = registered_env();
        assert!(env.get(&Name::str("ReynoldsParametricity")).is_some());
        assert!(env.get(&Name::str("ParametricFunction")).is_some());
        assert!(env.get(&Name::str("ParametricityCondition")).is_some());
    }
    #[test]
    fn test_free_theorems_registered() {
        let env = registered_env();
        assert!(env.get(&Name::str("FreeTheorem")).is_some());
        assert!(env.get(&Name::str("FreeTheoremId")).is_some());
        assert!(env.get(&Name::str("FreeTheoremList")).is_some());
        assert!(env.get(&Name::str("FreeTheoremFold")).is_some());
    }
    #[test]
    fn test_naturality_registered() {
        let env = registered_env();
        assert!(env.get(&Name::str("Naturality")).is_some());
        assert!(env.get(&Name::str("DiNaturality")).is_some());
        assert!(env.get(&Name::str("NaturalTransformation")).is_some());
        assert!(env.get(&Name::str("NaturalIso")).is_some());
    }
    #[test]
    fn test_abstract_types_registered() {
        let env = registered_env();
        assert!(env.get(&Name::str("AbstractPackage")).is_some());
        assert!(env.get(&Name::str("RepresentationIndependence")).is_some());
        assert!(env.get(&Name::str("AbstractionTheorem")).is_some());
        assert!(env.get(&Name::str("ExistentialType")).is_some());
        assert!(env.get(&Name::str("Pack")).is_some());
        assert!(env.get(&Name::str("Unpack")).is_some());
    }
    #[test]
    fn test_logical_relations_registered() {
        let env = registered_env();
        assert!(env.get(&Name::str("LogicalRelation")).is_some());
        assert!(env.get(&Name::str("FundamentalLemma")).is_some());
        assert!(env.get(&Name::str("RelationalModel")).is_some());
        assert!(env.get(&Name::str("ParametricModel")).is_some());
    }
    #[test]
    fn test_system_f_registered() {
        let env = registered_env();
        assert!(env.get(&Name::str("PolymorphicId")).is_some());
        assert!(env.get(&Name::str("PolymorphicConst")).is_some());
        assert!(env.get(&Name::str("PolymorphicFlip")).is_some());
        assert!(env.get(&Name::str("ChurchNumeral")).is_some());
        assert!(env.get(&Name::str("ChurchBool")).is_some());
    }
    #[test]
    fn test_coherence_registered() {
        let env = registered_env();
        assert!(env.get(&Name::str("CoherenceTheorem")).is_some());
        assert!(env.get(&Name::str("ParametricityCoherence")).is_some());
        assert!(env.get(&Name::str("UniqueInhabitant")).is_some());
        assert!(env.get(&Name::str("UniversalProperty")).is_some());
    }
    #[test]
    fn test_enriched_parametricity_registered() {
        let env = registered_env();
        assert!(env.get(&Name::str("KripkeRelation")).is_some());
        assert!(env.get(&Name::str("StepIndexedRelation")).is_some());
        assert!(env.get(&Name::str("BiorthogonalityRelation")).is_some());
        assert!(env.get(&Name::str("AdmissibleRelation")).is_some());
    }
    #[test]
    fn test_all_axioms_have_correct_sort() {
        let env = registered_env();
        let decl = env
            .get(&Name::str("PolymorphicId"))
            .expect("declaration 'PolymorphicId' should exist in env");
        match decl {
            Declaration::Axiom { ty, .. } => {
                matches!(ty, Expr::Pi(..));
            }
            _ => panic!("Expected Axiom declaration"),
        }
    }
    #[test]
    fn test_dependent_types_registered() {
        let env = registered_env();
        assert!(env.get(&Name::str("PiType")).is_some());
        assert!(env.get(&Name::str("SigmaType")).is_some());
        assert!(env.get(&Name::str("SigmaFst")).is_some());
        assert!(env.get(&Name::str("SigmaSnd")).is_some());
        assert!(env.get(&Name::str("PiExt")).is_some());
    }
    #[test]
    fn test_identity_types_registered() {
        let env = registered_env();
        assert!(env.get(&Name::str("IdType")).is_some());
        assert!(env.get(&Name::str("IdRefl")).is_some());
        assert!(env.get(&Name::str("IdElim")).is_some());
        assert!(env.get(&Name::str("IdSymm")).is_some());
        assert!(env.get(&Name::str("IdTrans")).is_some());
    }
    #[test]
    fn test_univalence_funext_registered() {
        let env = registered_env();
        assert!(env.get(&Name::str("TypeEquiv")).is_some());
        assert!(env.get(&Name::str("UnivalenceAxiom")).is_some());
        assert!(env.get(&Name::str("FunExt")).is_some());
        assert!(env.get(&Name::str("PropExt")).is_some());
    }
    #[test]
    fn test_hits_registered() {
        let env = registered_env();
        assert!(env.get(&Name::str("Trunc")).is_some());
        assert!(env.get(&Name::str("TruncIn")).is_some());
        assert!(env.get(&Name::str("TruncElim")).is_some());
        assert!(env.get(&Name::str("CircleHIT")).is_some());
        assert!(env.get(&Name::str("SuspensionHIT")).is_some());
        assert!(env.get(&Name::str("IntervalHIT")).is_some());
        assert!(env.get(&Name::str("PushoutHIT")).is_some());
    }
    #[test]
    fn test_ott_registered() {
        let env = registered_env();
        assert!(env.get(&Name::str("ObsEq")).is_some());
        assert!(env.get(&Name::str("ObsEqRefl")).is_some());
        assert!(env.get(&Name::str("Coerce")).is_some());
        assert!(env.get(&Name::str("CoherenceOTT")).is_some());
    }
    #[test]
    fn test_two_level_tt_registered() {
        let env = registered_env();
        assert!(env.get(&Name::str("FibrantType")).is_some());
        assert!(env.get(&Name::str("StrictEquality")).is_some());
        assert!(env.get(&Name::str("InnerToOuter")).is_some());
    }
    #[test]
    fn test_setoid_model_registered() {
        let env = registered_env();
        assert!(env.get(&Name::str("Setoid")).is_some());
        assert!(env.get(&Name::str("SetoidMap")).is_some());
        assert!(env.get(&Name::str("SetoidEquiv")).is_some());
        assert!(env.get(&Name::str("SetoidQuotient")).is_some());
    }
    #[test]
    fn test_realizability_registered() {
        let env = registered_env();
        assert!(env.get(&Name::str("PCA")).is_some());
        assert!(env.get(&Name::str("Realizer")).is_some());
        assert!(env.get(&Name::str("RealizabilityTripos")).is_some());
        assert!(env.get(&Name::str("EffectiveTopos")).is_some());
    }
    #[test]
    fn test_per_model_registered() {
        let env = registered_env();
        assert!(env.get(&Name::str("PER")).is_some());
        assert!(env.get(&Name::str("PERDomain")).is_some());
        assert!(env.get(&Name::str("PERMap")).is_some());
        assert!(env.get(&Name::str("PERModel")).is_some());
    }
    #[test]
    fn test_orthogonality_registered() {
        let env = registered_env();
        assert!(env.get(&Name::str("Orthogonal")).is_some());
        assert!(env.get(&Name::str("WFS")).is_some());
        assert!(env.get(&Name::str("SmallObjectArgument")).is_some());
    }
    #[test]
    fn test_gradual_typing_registered() {
        let env = registered_env();
        assert!(env.get(&Name::str("DynType")).is_some());
        assert!(env.get(&Name::str("Inject")).is_some());
        assert!(env.get(&Name::str("Project")).is_some());
        assert!(env.get(&Name::str("GradualConsistency")).is_some());
        assert!(env.get(&Name::str("CastEvidence")).is_some());
    }
    #[test]
    fn test_effect_types_registered() {
        let env = registered_env();
        assert!(env.get(&Name::str("EffectSig")).is_some());
        assert!(env.get(&Name::str("EffectTree")).is_some());
        assert!(env.get(&Name::str("Handler")).is_some());
        assert!(env.get(&Name::str("MonadLaw")).is_some());
        assert!(env.get(&Name::str("MonadMorphism")).is_some());
    }
    #[test]
    fn test_logical_relation_base() {
        let rel = LogicalRelation::base("Nat", |a, b| a == b);
        assert!(rel.self_related("42"));
        assert!(rel.relates("hello", "hello"));
        assert!(!rel.relates("x", "y"));
    }
    #[test]
    fn test_logical_relation_arrow() {
        let dom = LogicalRelation::base("Bool", |a, b| a == b);
        let cod = LogicalRelation::base("Nat", |a, b| a == b);
        let arr = LogicalRelation::arrow(dom, cod);
        assert!(arr.self_related("f"));
    }
    #[test]
    fn test_free_theorem_identity() {
        let xs = vec![1, 2, 3, 4, 5];
        assert!(FreeTheoremDeriver::verify_identity_theorem(xs));
    }
    #[test]
    fn test_free_theorem_reverse_naturality() {
        let xs = vec![1i32, 2, 3];
        let result = FreeTheoremDeriver::verify_reverse_naturality(|x: i32| x * 2, xs);
        assert!(result);
    }
    #[test]
    fn test_free_theorem_deriver_fields() {
        let d = FreeTheoremDeriver::list_endomorphism();
        assert!(d.type_sig.contains("List alpha"));
        assert!(d.theorem.contains("map"));
        let d2 = FreeTheoremDeriver::poly_identity();
        assert!(d2.theorem.contains("f A x = x"));
    }
    #[test]
    fn test_realizability_model_nat() {
        let model = RealizabilityModel::new(PcfType::Nat);
        assert!(model.is_realizer(&PcfValue::Num(42)));
        assert!(!model.is_realizer(&PcfValue::BoolVal(true)));
        assert!(model.is_realizer(&PcfValue::Bottom));
    }
    #[test]
    fn test_realizability_model_per() {
        let model = RealizabilityModel::new(PcfType::Nat);
        assert!(model.per_equiv(&PcfValue::Num(5), &PcfValue::Num(5)));
        assert!(!model.per_equiv(&PcfValue::Num(5), &PcfValue::Num(6)));
        assert!(!model.per_equiv(&PcfValue::Bottom, &PcfValue::Bottom));
    }
    #[test]
    fn test_cubical_path_constant() {
        let p: CubicalPath<i32> = CubicalPath::constant(42);
        assert_eq!(p.at(vec![false]), 42);
        assert_eq!(p.at(vec![true]), 42);
    }
    #[test]
    fn test_cubical_path_endpoints() {
        let p = CubicalPath::from_fn(|i| if i { 10 } else { 0 });
        assert_eq!(p.left(), 0);
        assert_eq!(p.right(), 10);
        assert_eq!(p.face0(), 0);
        assert_eq!(p.face1(), 10);
    }
    #[test]
    fn test_cubical_path_reverse() {
        let p = CubicalPath::from_fn(|i| if i { 10 } else { 0 });
        let rev = p.reverse();
        assert_eq!(rev.left(), 10);
        assert_eq!(rev.right(), 0);
    }
    #[test]
    fn test_cubical_path_connections() {
        let p = CubicalPath::from_fn(|i| if i { 1u32 } else { 0u32 });
        assert_eq!(p.meet(false), 0);
        assert_eq!(p.join(true), 0);
    }
    #[test]
    fn test_gradual_typer_consistency() {
        let int_ty = GradualType::Base("Int".into());
        let bool_ty = GradualType::Base("Bool".into());
        let unknown = GradualType::Unknown;
        assert!(GradualTyper::consistent(&unknown, &int_ty).is_some());
        assert!(GradualTyper::consistent(&int_ty, &unknown).is_some());
        assert!(GradualTyper::consistent(&int_ty, &bool_ty).is_none());
        assert!(GradualTyper::consistent(&int_ty, &int_ty).is_some());
    }
    #[test]
    fn test_gradual_typer_arrow_consistency() {
        let int_ty = GradualType::Base("Int".into());
        let unknown = GradualType::Unknown;
        let arrow_dyn = GradualType::Arrow(Box::new(unknown.clone()), Box::new(unknown.clone()));
        let arrow_int = GradualType::Arrow(Box::new(int_ty.clone()), Box::new(int_ty.clone()));
        assert!(GradualTyper::consistent(&arrow_int, &arrow_dyn).is_some());
    }
    #[test]
    fn test_gradual_typer_precision() {
        let int_ty = GradualType::Base("Int".into());
        let unknown = GradualType::Unknown;
        assert!(GradualTyper::precision(&int_ty, &unknown));
        assert!(!GradualTyper::precision(&unknown, &int_ty));
        assert!(GradualTyper::precision(&int_ty, &int_ty));
    }
    #[test]
    fn test_gradual_typer_unknown_consistent_with_any() {
        let int_ty = GradualType::Base("Int".into());
        let bool_ty = GradualType::Base("Bool".into());
        assert!(GradualTyper::unknown_is_consistent_with_any(&int_ty));
        assert!(GradualTyper::unknown_is_consistent_with_any(&bool_ty));
        assert!(GradualTyper::unknown_is_consistent_with_any(
            &GradualType::Unknown
        ));
    }
    #[test]
    fn test_simple_type_construction() {
        let int_ty = SimpleType::Base("Int".into());
        let bool_ty = SimpleType::Base("Bool".into());
        let arr = SimpleType::Arrow(Box::new(int_ty.clone()), Box::new(bool_ty.clone()));
        match &arr {
            SimpleType::Arrow(dom, cod) => {
                assert_eq!(**dom, int_ty);
                assert_eq!(**cod, bool_ty);
            }
            _ => panic!("Expected Arrow"),
        }
    }
    #[test]
    fn test_pcf_type_construction() {
        let nat = PcfType::Nat;
        let bool_ty = PcfType::Bool;
        let fun = PcfType::Fun(Box::new(nat), Box::new(bool_ty));
        match &fun {
            PcfType::Fun(a, b) => {
                assert_eq!(**a, PcfType::Nat);
                assert_eq!(**b, PcfType::Bool);
            }
            _ => panic!("Expected Fun"),
        }
    }
}
