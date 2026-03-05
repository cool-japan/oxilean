//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::functions::*;
use super::types::{
    HeterogeneousEq, OTTUniverseLevel, ObsEq, ObservationalQuotient, PERTypeModel, TarskiUniverse,
};

/// `ObsEq.typeLevel : ∀ {A B : Type}, ObsEq Type A B → ∀ (a : A), ∃ (b : B), HEq a b`
///
/// Type-level OTT: observational equality of types witnesses a bijection.
/// Every element on the left has a heterogeneously-equal counterpart on the right.
pub fn obs_eq_type_level_ty() -> Expr {
    ott_ext_ipi(
        "A",
        ott_ext_type0(),
        ott_ext_ipi(
            "B",
            ott_ext_type0(),
            ott_ext_arrow(
                ott_ext_app3(
                    ott_ext_cst("ObsEq"),
                    ott_ext_type0(),
                    ott_ext_bvar(1),
                    ott_ext_bvar(0),
                ),
                ott_ext_pi(
                    "a",
                    ott_ext_bvar(2),
                    ott_ext_app(
                        ott_ext_cst("Exists"),
                        ott_ext_pi(
                            "b",
                            ott_ext_bvar(2),
                            ott_ext_app4(
                                ott_ext_cst("HEq"),
                                ott_ext_bvar(4),
                                ott_ext_bvar(1),
                                ott_ext_bvar(3),
                                ott_ext_bvar(0),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// `ObsEq.unicity : ∀ {A : Type} {a b : A} (e1 e2 : ObsEq A a b), ObsEq (ObsEq A a b) e1 e2`
///
/// Unicity of OTT equality proofs: any two proofs of `ObsEq A a b` are themselves
/// observationally equal. This is the OTT analogue of UIP.
pub fn obs_eq_unicity_ty() -> Expr {
    ott_ext_ipi(
        "A",
        ott_ext_type0(),
        ott_ext_ipi(
            "a",
            ott_ext_bvar(0),
            ott_ext_ipi(
                "b",
                ott_ext_bvar(1),
                ott_ext_pi(
                    "e1",
                    ott_ext_app3(
                        ott_ext_cst("ObsEq"),
                        ott_ext_bvar(2),
                        ott_ext_bvar(1),
                        ott_ext_bvar(0),
                    ),
                    ott_ext_pi(
                        "e2",
                        ott_ext_app3(
                            ott_ext_cst("ObsEq"),
                            ott_ext_bvar(3),
                            ott_ext_bvar(2),
                            ott_ext_bvar(1),
                        ),
                        ott_ext_app3(
                            ott_ext_cst("ObsEq"),
                            ott_ext_app3(
                                ott_ext_cst("ObsEq"),
                                ott_ext_bvar(4),
                                ott_ext_bvar(3),
                                ott_ext_bvar(2),
                            ),
                            ott_ext_bvar(1),
                            ott_ext_bvar(0),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// `OTTFunExt : ∀ {A : Type} {B : A → Type} {f g : ∀ x : A, B x},
///     (∀ x, ObsEq (B x) (f x) (g x)) → ObsEq (∀ x : A, B x) f g`
///
/// Dependent function extensionality for OTT: pointwise observational equality
/// implies observational equality of dependent functions.
pub fn ott_fun_ext_ty() -> Expr {
    ott_ext_ipi(
        "A",
        ott_ext_type0(),
        ott_ext_ipi(
            "B",
            ott_ext_arrow(ott_ext_bvar(0), ott_ext_type0()),
            ott_ext_pi(
                "f",
                ott_ext_pi(
                    "x",
                    ott_ext_bvar(1),
                    ott_ext_app(ott_ext_bvar(1), ott_ext_bvar(0)),
                ),
                ott_ext_pi(
                    "g",
                    ott_ext_pi(
                        "x",
                        ott_ext_bvar(2),
                        ott_ext_app(ott_ext_bvar(2), ott_ext_bvar(0)),
                    ),
                    ott_ext_arrow(
                        ott_ext_pi(
                            "x",
                            ott_ext_bvar(3),
                            ott_ext_app3(
                                ott_ext_cst("ObsEq"),
                                ott_ext_app(ott_ext_bvar(3), ott_ext_bvar(0)),
                                ott_ext_app(ott_ext_bvar(2), ott_ext_bvar(0)),
                                ott_ext_app(ott_ext_bvar(1), ott_ext_bvar(0)),
                            ),
                        ),
                        ott_ext_app3(
                            ott_ext_cst("ObsEq"),
                            ott_ext_pi(
                                "x",
                                ott_ext_bvar(4),
                                ott_ext_app(ott_ext_bvar(4), ott_ext_bvar(0)),
                            ),
                            ott_ext_bvar(1),
                            ott_ext_bvar(0),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// `OTTPropExt : ∀ (P Q : Prop), (P ↔ Q) → ObsEq Prop P Q`
///
/// Propositional extensionality for OTT: logically equivalent propositions
/// are observationally equal as types. This follows from OTT's type-directed
/// definition of equality on propositions.
pub fn ott_prop_ext_ty() -> Expr {
    ott_ext_pi(
        "P",
        ott_ext_prop(),
        ott_ext_pi(
            "Q",
            ott_ext_prop(),
            ott_ext_arrow(
                ott_ext_app2(ott_ext_cst("Iff"), ott_ext_bvar(1), ott_ext_bvar(0)),
                ott_ext_app3(
                    ott_ext_cst("ObsEq"),
                    ott_ext_prop(),
                    ott_ext_bvar(2),
                    ott_ext_bvar(1),
                ),
            ),
        ),
    )
}
/// `OTTBoolObs : ∀ (b1 b2 : Bool), ObsEq Bool b1 b2 ↔ b1 = b2`
///
/// Observational equality on Bool coincides with propositional equality.
pub fn ott_bool_obs_ty() -> Expr {
    ott_ext_pi(
        "b1",
        ott_ext_cst("Bool"),
        ott_ext_pi(
            "b2",
            ott_ext_cst("Bool"),
            ott_ext_app2(
                ott_ext_cst("Iff"),
                ott_ext_app3(
                    ott_ext_cst("ObsEq"),
                    ott_ext_cst("Bool"),
                    ott_ext_bvar(1),
                    ott_ext_bvar(0),
                ),
                ott_ext_app3(
                    ott_ext_cst("Eq"),
                    ott_ext_cst("Bool"),
                    ott_ext_bvar(1),
                    ott_ext_bvar(0),
                ),
            ),
        ),
    )
}
/// `OTTNatSuccInj : ∀ (n m : Nat), ObsEq Nat (Nat.succ n) (Nat.succ m) → ObsEq Nat n m`
///
/// Injectivity of successor for observational equality on Nat.
pub fn ott_nat_succ_inj_ty() -> Expr {
    ott_ext_pi(
        "n",
        ott_ext_cst("Nat"),
        ott_ext_pi(
            "m",
            ott_ext_cst("Nat"),
            ott_ext_arrow(
                ott_ext_app3(
                    ott_ext_cst("ObsEq"),
                    ott_ext_cst("Nat"),
                    ott_ext_app(ott_ext_cst("Nat.succ"), ott_ext_bvar(1)),
                    ott_ext_app(ott_ext_cst("Nat.succ"), ott_ext_bvar(0)),
                ),
                ott_ext_app3(
                    ott_ext_cst("ObsEq"),
                    ott_ext_cst("Nat"),
                    ott_ext_bvar(1),
                    ott_ext_bvar(0),
                ),
            ),
        ),
    )
}
/// Register all extended OTT axioms (§16+) into the environment.
///
/// Call this after `build_env` to get the full set of OTT axioms.
pub fn register_ott_extended(env: &mut Environment) -> Result<(), String> {
    let axioms: &[(&str, Expr)] = &[
        ("HEq", heq_ty()),
        ("HEq.refl", heq_refl_ty()),
        ("HEq.sym", heq_sym_ty()),
        ("HEq.trans", heq_trans_ty()),
        ("HEq.fromEq", heq_from_eq_ty()),
        ("HEq.toEq", heq_to_eq_ty()),
        ("ObsEq.transport", obs_eq_transport_ty()),
        ("CoerceCoherence", coerce_coherence_ty()),
        ("ObsEq.sigmaExt", obs_eq_sigma_ext_ty()),
        ("ObsEq.natChar", obs_eq_nat_char_ty()),
        ("OTTPropTrunc", ott_prop_trunc_ty()),
        ("OTTPropTrunc.intro", ott_prop_trunc_intro_ty()),
        ("OTTPropTrunc.elim", ott_prop_trunc_elim_ty()),
        ("OTTPropTrunc.isProp", ott_prop_trunc_is_prop_ty()),
        ("OTTProofIrrelevance", ott_proof_irrel_ty()),
        ("OTTSingletonElim", ott_singleton_elim_ty()),
        ("ObsEq.etaFun", obs_eq_eta_fun_ty()),
        ("ObsEq.etaSigma", obs_eq_eta_sigma_ty()),
        ("ObsEq.definitionalEta", obs_eq_def_eta_ty()),
        ("ObsQuot", obs_quot_ty()),
        ("ObsQuot.eq", obs_quot_eq_ty()),
        ("IsPER", is_per_ty()),
        ("PERModel", per_model_ty()),
        ("PERModel.carrier", per_model_carrier_ty()),
        ("PERModel.rel", per_model_rel_ty()),
        ("SetoidInterp", setoid_interp_ty()),
        ("SetoidInterp.pi", setoid_interp_pi_ty()),
        ("UniverseRussell", universe_russell_ty()),
        ("UniverseTarski", universe_tarski_ty()),
        ("UniverseTarski.El", universe_tarski_el_ty()),
        ("UnivPolyOTT", univ_poly_ott_ty()),
        ("Realizability.valid", realizability_valid_ty()),
        (
            "Realizability.completeness",
            realizability_completeness_ty(),
        ),
        ("Realizability.soundness", realizability_soundness_ty()),
        ("Realizability.uniformity", realizability_uniformity_ty()),
        ("ObsEq.isProp", obs_eq_is_prop_ty()),
        ("ObsEq.typeLevel", obs_eq_type_level_ty()),
        ("ObsEq.unicity", obs_eq_unicity_ty()),
        ("OTTFunExt", ott_fun_ext_ty()),
        ("OTTPropExt", ott_prop_ext_ty()),
        ("OTTBoolObs", ott_bool_obs_ty()),
        ("OTTNatSuccInj", ott_nat_succ_inj_ty()),
    ];
    for (name, ty) in axioms {
        env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty: ty.clone(),
        })
        .ok();
    }
    Ok(())
}
#[cfg(test)]
mod extended_tests {
    use super::*;
    use oxilean_kernel::Environment;
    #[test]
    fn test_register_ott_extended() {
        let mut env = Environment::new();
        build_env(&mut env);
        assert!(register_ott_extended(&mut env).is_ok());
        assert!(
            env.get(&Name::str("HEq")).is_some(),
            "HEq must be registered"
        );
        assert!(
            env.get(&Name::str("HEq.refl")).is_some(),
            "HEq.refl must be registered"
        );
        assert!(
            env.get(&Name::str("OTTFunExt")).is_some(),
            "OTTFunExt must be registered"
        );
        assert!(
            env.get(&Name::str("OTTPropExt")).is_some(),
            "OTTPropExt must be registered"
        );
        assert!(
            env.get(&Name::str("PERModel")).is_some(),
            "PERModel must be registered"
        );
        assert!(
            env.get(&Name::str("SetoidInterp")).is_some(),
            "SetoidInterp must be registered"
        );
    }
    #[test]
    fn test_heterogeneous_eq_refl() {
        let h = HeterogeneousEq::refl("Nat", "0");
        assert!(h.is_homogeneous());
        assert_eq!(h.type_a, h.type_b);
        assert_eq!(h.elem_a, h.elem_b);
    }
    #[test]
    fn test_heterogeneous_eq_sym() {
        let h = HeterogeneousEq::new("Nat", "0", "Int", "zero");
        let sym_h = h.sym();
        assert_eq!(sym_h.type_a, "Int");
        assert_eq!(sym_h.elem_a, "zero");
        assert_eq!(sym_h.type_b, "Nat");
        assert_eq!(sym_h.elem_b, "0");
    }
    #[test]
    fn test_heterogeneous_eq_trans() {
        let h1 = HeterogeneousEq::new("Nat", "a", "Int", "b");
        let h2 = HeterogeneousEq::new("Int", "b", "Real", "c");
        let composed = h1.trans(h2);
        assert!(composed.is_some());
        let comp = composed.expect("composed should be valid");
        assert_eq!(comp.type_a, "Nat");
        assert_eq!(comp.type_b, "Real");
        assert_eq!(comp.elem_a, "a");
        assert_eq!(comp.elem_b, "c");
    }
    #[test]
    fn test_heterogeneous_eq_display() {
        let h = HeterogeneousEq::new("Nat", "n", "Nat", "m");
        let disp = h.display();
        assert!(disp.contains("HEq"));
        assert!(disp.contains("Nat"));
        assert!(disp.contains("n"));
        assert!(disp.contains("m"));
    }
    #[test]
    fn test_per_type_model_valid() {
        let per = PERTypeModel::new("Nat", "EvenEq", true, true);
        assert!(per.is_valid_per());
        assert!(per.is_setoid_like());
    }
    #[test]
    fn test_per_type_model_domain() {
        let per = PERTypeModel::new("Nat", "EvenEq", true, true);
        let dom = per.domain_name();
        assert!(dom.contains("Nat"));
        assert!(dom.contains("EvenEq"));
    }
    #[test]
    fn test_per_type_model_to_setoid() {
        let per = PERTypeModel::new("Nat", "EvenEq", true, true);
        let setoid = per.restriction_to_domain();
        assert!(setoid.is_some());
        assert!(setoid.expect("setoid should be valid").is_valid());
    }
    #[test]
    fn test_per_type_model_product() {
        let per1 = PERTypeModel::new("Nat", "EqNat", true, true);
        let per2 = PERTypeModel::new("Bool", "EqBool", true, true);
        let prod = per1.product(&per2);
        assert!(prod.is_valid_per());
        assert!(prod.set.contains("Nat"));
        assert!(prod.set.contains("Bool"));
    }
    #[test]
    fn test_ott_universe_level_ordering() {
        let prop = OTTUniverseLevel::prop();
        let t0 = OTTUniverseLevel::type0();
        let t1 = OTTUniverseLevel::type_n(1);
        assert!(prop.is_prop());
        assert!(prop.is_sub_universe_of(&t0));
        assert!(t0.is_sub_universe_of(&t1));
        assert!(!t1.is_sub_universe_of(&t0));
    }
    #[test]
    fn test_ott_universe_level_display() {
        assert_eq!(OTTUniverseLevel::prop().display(), "Prop");
        assert_eq!(OTTUniverseLevel::type0().display(), "Type");
        assert_eq!(OTTUniverseLevel::type_n(1).display(), "Type 1");
    }
    #[test]
    fn test_ott_universe_level_max() {
        let a = OTTUniverseLevel::type_n(1);
        let b = OTTUniverseLevel::type_n(3);
        let m = OTTUniverseLevel::max(&a, &b);
        assert_eq!(m.level, b.level);
    }
    #[test]
    fn test_observational_quotient() {
        let oq = ObservationalQuotient::new("Int", "ModEq(3)", true);
        assert!(oq.is_well_formed());
        let name = oq.quotient_name();
        assert!(name.contains("Int"));
        assert!(name.contains("ModEq"));
        let embedded = oq.embed("5");
        assert!(embedded.contains("5"));
        let eq_check = oq.are_equal("4", "7");
        assert!(eq_check.contains("ModEq"));
    }
    #[test]
    fn test_tarski_universe() {
        let mut u0 = TarskiUniverse::new(0);
        u0.register_code("Nat");
        u0.register_code("Bool");
        assert_eq!(u0.code_count(), 2);
        assert!(u0.contains_code("Nat"));
        assert!(!u0.contains_code("Real"));
        let u1 = u0.lift();
        assert_eq!(u1.level, 1);
        assert_eq!(u1.code_type_name(), "U1");
        let decoded = u0.decode("Nat");
        assert!(decoded.contains("El0"));
        assert!(decoded.contains("Nat"));
    }
}
