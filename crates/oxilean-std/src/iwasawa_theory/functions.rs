//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)
#![allow(clippy::items_after_test_module)]

use oxilean_kernel::{Declaration, Environment, Expr, Name};

use super::types::{
    AnticyclotomicExtension, BlochKato, BlochKatoConjecture, CharacteristicIdealApprox,
    CharacteristicIdealElement, ClassGroupTower, ColemanMap, ColemanPAdicL, CyclotomicField,
    CyclotomicUnit, EllipticCurveMainConjecture, EquivariantLFunction, EquivariantTamagawa,
    EulerCharacteristicFormula, EulerSystemValidator, FittingIdeal, GaloisRepresentation,
    GeometricMainConjecture, GreenbergConjecture, GreenbergSelmer, IwasawaAlgebra,
    IwasawaMainConjecture, IwasawaMainConjectureStatement, IwasawaModule, IwasawaModuleComputer,
    KatoEulerSystemData, KatzPAdicLFunction, KolyvaginEulerSystem, KubotaLeopoldt, MazurTeitelbaum,
    NoncommutativeIwasawa, NormCompatibleSystem, PAdicLFunction, PAdicLieExtension, PerrinRiouExp,
    RegulatorMap, RegulatorPowerSeries, RubinStarkConjecture, SelmerGroup, SelmerGroupInTower,
    SelmerTowerGrowth, StructureTheorem, SyntomicRegulator, TamagawaNumber, WilesProof,
};

pub fn cst(s: &str) -> Expr {
    Expr::Const(Name::str(s), vec![])
}
pub fn prop() -> Expr {
    Expr::Sort(oxilean_kernel::Level::zero())
}
pub fn type0() -> Expr {
    Expr::Sort(oxilean_kernel::Level::succ(oxilean_kernel::Level::zero()))
}
pub fn nat_ty() -> Expr {
    cst("Nat")
}
pub fn int_ty() -> Expr {
    cst("Int")
}
pub fn real_ty() -> Expr {
    cst("Real")
}
pub fn bool_ty() -> Expr {
    cst("Bool")
}
pub fn arrow(a: Expr, b: Expr) -> Expr {
    Expr::Pi(
        oxilean_kernel::BinderInfo::Default,
        Name::str("_"),
        Box::new(a),
        Box::new(b),
    )
}
/// `IwasawaAlgebraTy : Nat → Type` — Λ = ℤ_p[\[Γ\]] for a given prime p.
pub fn iwasawa_algebra_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `IwasawaModuleTy : Nat → Type` — finitely generated torsion Λ-module.
pub fn iwasawa_module_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `StructureTheoremTy : Prop` — the Iwasawa structure theorem.
pub fn structure_theorem_ty() -> Expr {
    prop()
}
/// `LambdaInvariantTy : Nat → Nat` — λ-invariant of a module.
pub fn lambda_invariant_ty() -> Expr {
    arrow(nat_ty(), nat_ty())
}
/// `MuInvariantTy : Nat → Nat` — μ-invariant of a module.
pub fn mu_invariant_ty() -> Expr {
    arrow(nat_ty(), nat_ty())
}
/// `CharacteristicIdealTy : Nat → Type` — characteristic ideal in Λ.
pub fn characteristic_ideal_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `CyclotomicFieldTy : Nat → Nat → Type` — ℚ(ζ_{p^n}).
pub fn cyclotomic_field_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `ConductorTy : Nat → Nat → Nat` — conductor of ℚ(ζ_{p^n}).
pub fn conductor_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), nat_ty()))
}
/// `DiscriminantTy : Nat → Nat → Int` — discriminant of cyclotomic field.
pub fn discriminant_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), int_ty()))
}
/// `ClassNumberTy : Nat → Nat → Nat` — class number h(ℚ(ζ_{p^n})).
pub fn class_number_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), nat_ty()))
}
/// `CyclotomicUnitTy : Nat → Type` — group of cyclotomic units.
pub fn cyclotomic_unit_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `RegulatorPowerSeriesTy : Nat → Type` — p-adic regulator power series.
pub fn regulator_power_series_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `ClassGroupTowerTy : Nat → Type` — inverse limit of class groups.
pub fn class_group_tower_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `IwasawaMainConjectureTy : Prop` — char ideal equals p-adic L-function.
pub fn iwasawa_main_conjecture_ty() -> Expr {
    prop()
}
/// `KolyvaginEulerSystemTy : Prop` — Kolyvagin's Euler system bound.
pub fn kolyvagin_euler_system_ty() -> Expr {
    prop()
}
/// `AnticyclotomicExtensionTy : Int → Nat → Type` — K_∞^- over ℚ(√-D).
pub fn anticyclotomic_extension_ty() -> Expr {
    arrow(int_ty(), arrow(nat_ty(), type0()))
}
/// `PAdicLFunctionTy : Nat → Type` — p-adic L-function.
pub fn padic_l_function_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `InterpolationPropertyTy : Prop` — L_p interpolates classical L-values.
pub fn interpolation_property_ty() -> Expr {
    prop()
}
/// `FunctionalEquationTy : Prop` — functional equation for L_p.
pub fn functional_equation_ty() -> Expr {
    prop()
}
/// `TrivialZerosTy : Nat → Type` — trivial zeros of L_p.
pub fn trivial_zeros_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `MazurTeitelbaumTy : Prop` — exceptional zero formula.
pub fn mazur_teitelbaum_ty() -> Expr {
    prop()
}
/// `KatzPAdicLFunctionTy : Int → Nat → Type` — Katz L-function for CM fields.
pub fn katz_padic_l_function_ty() -> Expr {
    arrow(int_ty(), arrow(nat_ty(), type0()))
}
/// `SelmerGroupTy : Nat → Type` — Bloch–Kato Selmer group H¹_f.
pub fn selmer_group_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `SelmerRankTy : Nat → Nat` — rank of Selmer group.
pub fn selmer_rank_ty() -> Expr {
    arrow(nat_ty(), nat_ty())
}
/// `SelmerLambdaRankTy : Nat → Nat` — λ-rank of Selmer over Λ.
pub fn selmer_lambda_rank_ty() -> Expr {
    arrow(nat_ty(), nat_ty())
}
/// `GreenbergSelmerTy : Nat → Type` — Greenberg's Selmer group.
pub fn greenberg_selmer_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `BlochKatoTy : Nat → Type` — Bloch–Kato Selmer via period rings.
pub fn bloch_kato_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `IwasawaMainConjectureStatementTy : Prop` — char(Sel^∨) = (L_p(f)).
pub fn iwasawa_main_conjecture_statement_ty() -> Expr {
    prop()
}
/// `WilesProofTy : Prop` — Wiles's theorem (cyclotomic IMC).
pub fn wiles_proof_ty() -> Expr {
    prop()
}
/// `GreenbergConjectureTy : Prop` — μ = 0 for cyclotomic ℤ_p-extension.
pub fn greenberg_conjecture_ty() -> Expr {
    prop()
}
/// `SkinnerUrbanTy : Prop` — Skinner–Urban IMC for modular forms.
pub fn skinner_urban_ty() -> Expr {
    prop()
}
/// `KatoEulerSystemTy : Prop` — Kato's Euler system for modular forms.
pub fn kato_euler_system_ty() -> Expr {
    prop()
}
/// `FittingIdealTy : Nat → Type` — 0th Fitting ideal of a Λ-module.
pub fn fitting_ideal_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `CharacteristicIdealElementTy : Nat → Type` — characteristic series element.
pub fn characteristic_ideal_element_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `KubotaLeopoldt : Nat → Type` — Kubota-Leopoldt p-adic L-function.
pub fn kubota_leopoldt_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `ColemanPAdicL : Nat → Type` — Coleman p-adic L-function.
pub fn coleman_padic_l_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `GeometricMainConjecture : Prop` — geometric IMC over function fields.
pub fn geometric_main_conjecture_ty() -> Expr {
    prop()
}
/// `EllipticCurveMainConjecture : Nat → Prop` — elliptic curve IMC.
pub fn elliptic_curve_main_conjecture_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `SelmerTowerGrowth : Nat → Type` — Selmer group growth in tower.
pub fn selmer_tower_growth_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `EulerCharacteristicFormula : Prop` — Euler characteristic formula.
pub fn euler_characteristic_formula_ty() -> Expr {
    prop()
}
/// `ColemanMap : Nat → Type` — Coleman map H¹ → Λ.
pub fn coleman_map_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `RegulatorMap : Nat → Type` — p-adic regulator map.
pub fn regulator_map_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `NoncommutativeIwasawa : Nat → Type` — non-comm Iwasawa theory.
pub fn noncommutative_iwasawa_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `PAdicLieExtension : Nat → Type` — p-adic Lie extension.
pub fn padic_lie_extension_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `EquivariantLFunction : Nat → Type` — equivariant L-function.
pub fn equivariant_l_function_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `EquivariantTamagawa : Prop` — equivariant Tamagawa number conjecture.
pub fn equivariant_tamagawa_ty() -> Expr {
    prop()
}
/// `PerrinRiouExp : Nat → Type` — Perrin-Riou big exponential map.
pub fn perrin_riou_exp_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `SyntomicRegulator : Nat → Type` — syntomic regulator.
pub fn syntomic_regulator_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `KatoEulerSystemData : Nat → Type` — Kato's Euler system.
pub fn kato_euler_system_data_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `NormCompatibleSystem : Nat → Type` — norm-compatible Euler system.
pub fn norm_compatible_system_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `BlochKatoConjecture : Prop` — Bloch-Kato conjecture.
pub fn bloch_kato_conjecture_ty() -> Expr {
    prop()
}
/// `TamagawaNumber : Nat → Nat` — Tamagawa number at l.
pub fn tamagawa_number_ty() -> Expr {
    arrow(nat_ty(), nat_ty())
}
/// `GaloisRepresentation : Nat → Type` — l-adic Galois representation.
pub fn galois_representation_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `RubinStarkConjecture : Prop` — Rubin-Stark conjecture.
pub fn rubin_stark_conjecture_ty() -> Expr {
    prop()
}
/// `IwasawaModuleComputer : Nat → Type` — algorithmic module computer.
pub fn iwasawa_module_computer_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `CharacteristicIdealApprox : Nat → Type` — approximate char ideal.
pub fn characteristic_ideal_approx_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `EulerSystemValidator : Nat → Type` — Euler system norm checker.
pub fn euler_system_validator_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `SelmerGroupInTower : Nat → Type` — Selmer group growth computation.
pub fn selmer_group_in_tower_ty() -> Expr {
    arrow(nat_ty(), type0())
}
#[cfg(test)]
mod extended_tests {
    use super::*;
    #[test]
    fn test_fitting_ideal() {
        let fi = FittingIdeal::new("Sel^∨", 5);
        assert!(fi.fitting_divides_char().contains("Fitt_0"));
        assert!(fi.cyclic_equality().contains("cyclic"));
    }
    #[test]
    fn test_characteristic_ideal_element() {
        let ci = CharacteristicIdealElement::new(5, 3, 0);
        assert_eq!(ci.lambda_degree(), 3);
        assert_eq!(ci.mu_valuation(), 0);
        assert!(ci.characteristic_series().contains("T^3"));
    }
    #[test]
    fn test_kubota_leopoldt() {
        let kl = KubotaLeopoldt::new(5, "χ", true);
        assert!(kl.iwasawa_power_series().contains("g_χ"));
        assert!(kl.interpolation_at_negative_integer(3).contains("L_p(1-3"));
    }
    #[test]
    fn test_coleman_padic_l() {
        let col = ColemanPAdicL::new(5, "f", false);
        let signed = col.signed_l_functions();
        assert_eq!(signed.len(), 2);
        assert!(signed[0].contains("L_p^+"));
    }
    #[test]
    fn test_geometric_main_conjecture() {
        let gmc = GeometricMainConjecture::new("F_q(t)", true);
        assert!(gmc.statement().contains("char_Λ"));
        assert!(gmc.is_proven);
    }
    #[test]
    fn test_elliptic_curve_main_conjecture() {
        let ec = EllipticCurveMainConjecture::new("37A", 5);
        assert!(ec.mazur_conjecture().contains("char_Λ"));
        assert!(ec.analytic_rank().contains("37A"));
    }
    #[test]
    fn test_selmer_tower_growth() {
        let stg = SelmerTowerGrowth::new("E", 5, 0, 2, 3);
        assert!(stg.has_bounded_rank());
        let f = stg.growth_formula(2);
        assert!(f.contains("n=2"));
    }
    #[test]
    fn test_euler_characteristic_formula() {
        let ec = EulerCharacteristicFormula::new("37A", 5, 4);
        let s = ec.euler_characteristic();
        assert!(s.contains("37A"));
    }
    #[test]
    fn test_coleman_map() {
        let cm = ColemanMap::new("T_p(E)", 5);
        assert!(cm.map_description().contains("Coleman"));
        assert!(cm.perrin_riou_compatibility().contains("Perrin-Riou"));
    }
    #[test]
    fn test_regulator_map() {
        let rm = RegulatorMap::new("ℚ(√5)", 5, 2);
        assert!(rm.map_description().contains("Reg"));
    }
    #[test]
    fn test_noncommutative_iwasawa() {
        let nc = NoncommutativeIwasawa::new("GL_2(ℤ_5)", 5);
        assert!(nc.characteristic_element_nc().contains("K_1"));
        assert!(nc.noncommutative_main_conjecture().contains("Non-comm"));
    }
    #[test]
    fn test_padic_lie_extension() {
        let ple = PAdicLieExtension::new("GL_2(ℤ_p)", 5, 4);
        assert!(ple.iwasawa_algebra().contains("Λ"));
        assert!(ple.lazard_isomorphism().contains("Lazard"));
    }
    #[test]
    fn test_equivariant_l_function() {
        let eq = EquivariantLFunction::new("ℚ(√5)", 5, 2);
        assert!(eq.deligne_ribet_lfunction().contains("Deligne"));
        assert!(eq.equivariant_main_conjecture().contains("equivariant"));
    }
    #[test]
    fn test_equivariant_tamagawa() {
        let et = EquivariantTamagawa::new("h^{1,0}(E)", "ℤ_5", 5);
        assert!(et.etnc_statement().contains("eTNC"));
    }
    #[test]
    fn test_perrin_riou_exp() {
        let pr = PerrinRiouExp::new("T_p(E)", 5, 1);
        assert!(pr.map_description().contains("Exp_5"));
        assert!(pr.specialization_bk_exp(1).contains("Bloch-Kato"));
    }
    #[test]
    fn test_syntomic_regulator() {
        let sr = SyntomicRegulator::new("E", 5);
        assert!(sr.regulator_map(2, 1, 1).contains("syntomic"));
    }
    #[test]
    fn test_kato_euler_system_data() {
        let kes = KatoEulerSystemData::new("37A", 5);
        assert!(kes.beilinson_element(2).contains("Kato"));
        assert!(kes.norm_compatibility(2).contains("Kato norm"));
        assert!(kes.one_sided_divisibility().contains("Kato 2004"));
    }
    #[test]
    fn test_norm_compatible_system() {
        let ncs = NormCompatibleSystem::new("ℚ(ζ_{p^∞})", 5, "T_p(E)");
        assert!(ncs.norm_relation_at_l(7).contains("P_7"));
    }
    #[test]
    fn test_bloch_kato_conjecture() {
        let bkc = BlochKatoConjecture::new("h^1(E)", 1, 0);
        assert!(bkc.rank_formula().contains("BK rank formula"));
        assert!(bkc
            .leading_coefficient_formula()
            .contains("BK leading term"));
    }
    #[test]
    fn test_tamagawa_number() {
        let tn = TamagawaNumber::new("E", 7, 4);
        assert!(tn.description().contains("Tamagawa"));
        assert_eq!(tn.value, 4);
    }
    #[test]
    fn test_galois_representation() {
        let gr = GaloisRepresentation::new("E", 5, 2);
        assert!(gr.description().contains("G_ℚ"));
        assert_eq!(gr.dimension, 2);
    }
    #[test]
    fn test_rubin_stark_conjecture() {
        let rs = RubinStarkConjecture::new("ℚ(√5)", 1);
        assert!(rs.rubin_stark_element().contains("Rubin–Stark element"));
        assert!(rs.leading_term_formula().contains("Rubin–Stark"));
        assert!(rs.vanishing_order_formula().contains("ord"));
        assert!(rs.is_proven);
    }
    #[test]
    fn test_iwasawa_module_computer() {
        let imc = IwasawaModuleComputer::new(5, vec![0, 0, 1, 2, 3]);
        assert_eq!(imc.mu_invariant(), 2);
        let summary = imc.invariant_summary();
        assert!(summary.contains("p=5"));
    }
    #[test]
    fn test_characteristic_ideal_approx() {
        let cia = CharacteristicIdealApprox::new(5, 10, vec![1, 0, 1]);
        assert_eq!(cia.evaluate_at(2), 1 + 1 * 4);
        assert!(cia.is_distinguished());
        let cia_not = CharacteristicIdealApprox::new(5, 10, vec![1, 0, 3]);
        assert!(!cia_not.is_distinguished());
        let cia2 = CharacteristicIdealApprox::new(5, 10, vec![0, 0, 1]);
        assert!(cia2.is_distinguished());
    }
    #[test]
    fn test_euler_system_validator() {
        let mut esv = EulerSystemValidator::new(5, "T_p(E)");
        esv.add_class(0, "c_0");
        esv.add_class(1, "c_1");
        esv.add_class(2, "c_2");
        esv.add_euler_factor(7, "1 - 7^{-1}·Frob_7");
        assert!(esv.check_norm_relation(0));
        assert!(esv.check_norm_relation(1));
        assert_eq!(esv.verified_relations(), 2);
        assert!(esv.description().contains("Euler system"));
    }
    #[test]
    fn test_selmer_group_in_tower() {
        let sgt = SelmerGroupInTower::new("37A", 5, 0, 2, 1);
        assert!(sgt.is_bounded_growth());
        let seq = sgt.growth_sequence();
        assert_eq!(seq.len(), 11);
        assert_eq!(seq[0], 1);
        assert_eq!(seq[1], 3);
        assert!(sgt.description().contains("bounded=true"));
    }
    #[test]
    fn test_new_kernel_types() {
        let mut env = Environment::new();
        let new_axioms: &[(&str, fn() -> Expr)] = &[
            ("FittingIdeal", fitting_ideal_ty),
            (
                "CharacteristicIdealElement",
                characteristic_ideal_element_ty,
            ),
            ("KubotaLeopoldt", kubota_leopoldt_ty),
            ("ColemanPAdicL", coleman_padic_l_ty),
            ("GeometricMainConjecture", geometric_main_conjecture_ty),
            (
                "EllipticCurveMainConjecture",
                elliptic_curve_main_conjecture_ty,
            ),
            ("SelmerTowerGrowth", selmer_tower_growth_ty),
            (
                "EulerCharacteristicFormula",
                euler_characteristic_formula_ty,
            ),
            ("ColemanMap", coleman_map_ty),
            ("RegulatorMap", regulator_map_ty),
            ("NoncommutativeIwasawa", noncommutative_iwasawa_ty),
            ("PAdicLieExtension", padic_lie_extension_ty),
            ("EquivariantLFunction", equivariant_l_function_ty),
            ("EquivariantTamagawa", equivariant_tamagawa_ty),
            ("PerrinRiouExp", perrin_riou_exp_ty),
            ("SyntomicRegulator", syntomic_regulator_ty),
            ("KatoEulerSystemData", kato_euler_system_data_ty),
            ("NormCompatibleSystem", norm_compatible_system_ty),
            ("BlochKatoConjectureAxiom", bloch_kato_conjecture_ty),
            ("TamagawaNumber", tamagawa_number_ty),
            ("GaloisRepresentation", galois_representation_ty),
            ("RubinStarkConjecture", rubin_stark_conjecture_ty),
            ("IwasawaModuleComputerAxiom", iwasawa_module_computer_ty),
            (
                "CharacteristicIdealApproxAxiom",
                characteristic_ideal_approx_ty,
            ),
            ("EulerSystemValidatorAxiom", euler_system_validator_ty),
            ("SelmerGroupInTowerAxiom", selmer_group_in_tower_ty),
        ];
        for (name, ty_fn) in new_axioms {
            env.add(Declaration::Axiom {
                name: Name::str(*name),
                univ_params: vec![],
                ty: ty_fn(),
            })
            .ok();
        }
        assert!(env.get(&Name::str("FittingIdeal")).is_some());
        assert!(env.get(&Name::str("RubinStarkConjecture")).is_some());
        assert!(env.get(&Name::str("SelmerGroupInTowerAxiom")).is_some());
    }
}
type AxiomEntry = (&'static str, fn() -> Expr);
pub fn build_env(env: &mut Environment) {
    let axioms: &[AxiomEntry] = &[
        ("IwasawaAlgebra", iwasawa_algebra_ty),
        ("IwasawaModule", iwasawa_module_ty),
        ("StructureTheorem", structure_theorem_ty),
        ("LambdaInvariant", lambda_invariant_ty),
        ("MuInvariant", mu_invariant_ty),
        ("CharacteristicIdeal", characteristic_ideal_ty),
        ("CyclotomicField", cyclotomic_field_ty),
        ("Conductor", conductor_ty),
        ("Discriminant", discriminant_ty),
        ("ClassNumber", class_number_ty),
        ("CyclotomicUnit", cyclotomic_unit_ty),
        ("RegulatorPowerSeries", regulator_power_series_ty),
        ("ClassGroupTower", class_group_tower_ty),
        ("IwasawaMainConjecture", iwasawa_main_conjecture_ty),
        ("KolyvaginEulerSystem", kolyvagin_euler_system_ty),
        ("AnticyclotomicExtension", anticyclotomic_extension_ty),
        ("PAdicLFunction", padic_l_function_ty),
        ("InterpolationProperty", interpolation_property_ty),
        ("FunctionalEquation", functional_equation_ty),
        ("TrivialZeros", trivial_zeros_ty),
        ("MazurTeitelbaum", mazur_teitelbaum_ty),
        ("KatzPAdicLFunction", katz_padic_l_function_ty),
        ("SelmerGroup", selmer_group_ty),
        ("SelmerRank", selmer_rank_ty),
        ("SelmerLambdaRank", selmer_lambda_rank_ty),
        ("GreenbergSelmer", greenberg_selmer_ty),
        ("BlochKato", bloch_kato_ty),
        (
            "IwasawaMainConjectureStatement",
            iwasawa_main_conjecture_statement_ty,
        ),
        ("WilesProof", wiles_proof_ty),
        ("GreenbergConjecture", greenberg_conjecture_ty),
        ("SkinnnerUrban", skinner_urban_ty),
        ("KatoEulerSystem", kato_euler_system_ty),
        ("FittingIdeal", fitting_ideal_ty),
        (
            "CharacteristicIdealElement",
            characteristic_ideal_element_ty,
        ),
        ("KubotaLeopoldt", kubota_leopoldt_ty),
        ("ColemanPAdicL", coleman_padic_l_ty),
        ("GeometricMainConjecture", geometric_main_conjecture_ty),
        (
            "EllipticCurveMainConjecture",
            elliptic_curve_main_conjecture_ty,
        ),
        ("SelmerTowerGrowth", selmer_tower_growth_ty),
        (
            "EulerCharacteristicFormula",
            euler_characteristic_formula_ty,
        ),
        ("ColemanMap", coleman_map_ty),
        ("RegulatorMap", regulator_map_ty),
        ("NoncommutativeIwasawa", noncommutative_iwasawa_ty),
        ("PAdicLieExtension", padic_lie_extension_ty),
        ("EquivariantLFunction", equivariant_l_function_ty),
        ("EquivariantTamagawa", equivariant_tamagawa_ty),
        ("PerrinRiouExp", perrin_riou_exp_ty),
        ("SyntomicRegulator", syntomic_regulator_ty),
        ("KatoEulerSystemData", kato_euler_system_data_ty),
        ("NormCompatibleSystem", norm_compatible_system_ty),
        ("BlochKatoConjectureAxiom", bloch_kato_conjecture_ty),
        ("TamagawaNumber", tamagawa_number_ty),
        ("GaloisRepresentation", galois_representation_ty),
        ("RubinStarkConjecture", rubin_stark_conjecture_ty),
        ("IwasawaModuleComputerAxiom", iwasawa_module_computer_ty),
        (
            "CharacteristicIdealApproxAxiom",
            characteristic_ideal_approx_ty,
        ),
        ("EulerSystemValidatorAxiom", euler_system_validator_ty),
        ("SelmerGroupInTowerAxiom", selmer_group_in_tower_ty),
    ];
    for (name, ty_fn) in axioms {
        env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty: ty_fn(),
        })
        .ok();
    }
}
