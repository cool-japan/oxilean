//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{
    AfAlgebra, CStarAlgebraData, CStarElem, CompletelyPositiveMap, CrossedProduct, CuntzSemigroup,
    FactorType, FiniteMatrix, FiniteVonNeumann, FredholmData, GNSRepresentationSim, GNSTripleData,
    HaagerupProperty, KTheoryData, ModularTheoryData, NuclearityData, OperatorAlgebraRegistry,
    OperatorInequality, OperatorSpaceData, OperatorSpectrum, OperatorSystem, SpectralTripleData,
    StateData,
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
pub fn type1() -> Expr {
    Expr::Sort(Level::succ(Level::succ(Level::zero())))
}
pub(super) fn pi(bi: BinderInfo, name: &str, dom: Expr, body: Expr) -> Expr {
    Expr::Pi(bi, Name::str(name), Box::new(dom), Box::new(body))
}
pub fn arrow(a: Expr, b: Expr) -> Expr {
    pi(BinderInfo::Default, "_", a, b)
}
pub fn bvar(n: u32) -> Expr {
    Expr::BVar(n)
}
pub fn nat_ty() -> Expr {
    cst("Nat")
}
pub fn real_ty() -> Expr {
    cst("Real")
}
pub fn complex_ty() -> Expr {
    cst("Complex")
}
pub fn hilbert_ty() -> Expr {
    cst("HilbertSpace")
}
/// `CStarAlgebra : Type` — a C*-algebra is a Banach algebra A with involution *
/// satisfying the C*-identity: ||a* a|| = ||a||^2.
pub fn c_star_algebra_ty() -> Expr {
    type1()
}
/// `VonNeumannAlgebra : Type` — a von Neumann algebra (W*-algebra) is a
/// C*-algebra M in B(H) that is closed in the weak operator topology,
/// equivalently M = M'' (bicommutant theorem).
pub fn von_neumann_algebra_ty() -> Expr {
    type1()
}
/// `Commutant : CStarAlgebra -> CStarAlgebra` — the commutant M' of an algebra M.
pub fn commutant_ty() -> Expr {
    arrow(cst("CStarAlgebra"), cst("CStarAlgebra"))
}
/// `BicommutantTheorem : VonNeumannAlgebra -> Prop`.
pub fn bicommutant_theorem_ty() -> Expr {
    pi(BinderInfo::Default, "M", cst("VonNeumannAlgebra"), prop())
}
/// `GNSConstruction : CStarAlgebra -> State -> HilbertSpace`.
pub fn gns_construction_ty() -> Expr {
    arrow(cst("CStarAlgebra"), arrow(cst("State"), hilbert_ty()))
}
/// `GNSRepresentation : CStarAlgebra -> State -> StarHomomorphism`.
pub fn gns_representation_ty() -> Expr {
    arrow(
        cst("CStarAlgebra"),
        arrow(cst("State"), cst("StarHomomorphism")),
    )
}
/// `KMSState : CStarAlgebra -> Real -> State`.
pub fn kms_state_ty() -> Expr {
    arrow(cst("CStarAlgebra"), arrow(real_ty(), cst("State")))
}
/// `ModularAutomorphismGroup : VonNeumannAlgebra -> State -> Real -> StarAutomorphism`.
pub fn modular_automorphism_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "M",
        cst("VonNeumannAlgebra"),
        pi(
            BinderInfo::Default,
            "phi",
            cst("State"),
            arrow(real_ty(), cst("StarAutomorphism")),
        ),
    )
}
/// `TomitaTakesakiTheorem : VonNeumannAlgebra -> State -> Prop`.
pub fn tomita_takesaki_theorem_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "M",
        cst("VonNeumannAlgebra"),
        pi(BinderInfo::Default, "phi", cst("State"), prop()),
    )
}
/// `ModularConjugation : VonNeumannAlgebra -> State -> BoundedOperator`.
pub fn modular_conjugation_ty() -> Expr {
    arrow(
        cst("VonNeumannAlgebra"),
        arrow(cst("State"), cst("BoundedOperator")),
    )
}
/// `ModularOperator : VonNeumannAlgebra -> State -> PositiveOperator`.
pub fn modular_operator_ty() -> Expr {
    arrow(
        cst("VonNeumannAlgebra"),
        arrow(cst("State"), cst("PositiveOperator")),
    )
}
/// `Spectrum : CStarAlgebra -> AlgebraElement -> SubsetComplex`.
pub fn spectrum_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "A",
        cst("CStarAlgebra"),
        pi(
            BinderInfo::Default,
            "a",
            cst("AlgebraElement"),
            cst("SubsetComplex"),
        ),
    )
}
/// `SpectralRadius : CStarAlgebra -> AlgebraElement -> Real`.
pub fn spectral_radius_ty() -> Expr {
    arrow(cst("CStarAlgebra"), arrow(cst("AlgebraElement"), real_ty()))
}
/// `SpectralTheorem : HilbertSpace -> BoundedOperator -> Prop`.
pub fn spectral_theorem_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "H",
        hilbert_ty(),
        pi(BinderInfo::Default, "N", cst("BoundedOperator"), prop()),
    )
}
/// `ContinuousFunctionalCalculus : CStarAlgebra -> AlgebraElement -> Prop`.
pub fn continuous_functional_calculus_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "A",
        cst("CStarAlgebra"),
        pi(BinderInfo::Default, "a", cst("AlgebraElement"), prop()),
    )
}
/// `GelfandTransform : CommutativeCStarAlgebra -> ContinuousFunctions`.
pub fn gelfand_transform_ty() -> Expr {
    arrow(cst("CommutativeCStarAlgebra"), cst("ContinuousFunctions"))
}
/// `GelfandNaimarkTheorem : CStarAlgebra -> Prop`.
pub fn gelfand_naimark_theorem_ty() -> Expr {
    pi(BinderInfo::Default, "A", cst("CStarAlgebra"), prop())
}
/// `ConnesClassification : VonNeumannAlgebra -> VonNeumannAlgebraType -> Prop`.
pub fn connes_classification_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "M",
        cst("VonNeumannAlgebra"),
        arrow(cst("VonNeumannAlgebraType"), prop()),
    )
}
/// `HaagerupProperty : CStarAlgebra -> Prop`.
pub fn haagerup_property_ty() -> Expr {
    pi(BinderInfo::Default, "A", cst("CStarAlgebra"), prop())
}
/// `NuclearCStarAlgebra : CStarAlgebra -> Prop`.
pub fn nuclear_c_star_ty() -> Expr {
    pi(BinderInfo::Default, "A", cst("CStarAlgebra"), prop())
}
/// `InjectiveFactor : VonNeumannAlgebra -> Prop`.
pub fn injective_factor_ty() -> Expr {
    pi(BinderInfo::Default, "M", cst("VonNeumannAlgebra"), prop())
}
/// `PureState : CStarAlgebra -> State -> Prop` -- pure state (extremal in state space).
pub fn pure_state_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "A",
        cst("CStarAlgebra"),
        arrow(cst("State"), prop()),
    )
}
/// `StateSpace : CStarAlgebra -> ConvexSet` -- weak*-compact convex set of states.
pub fn state_space_ty() -> Expr {
    arrow(cst("CStarAlgebra"), cst("ConvexSet"))
}
/// `GNSCyclicVector : CStarAlgebra -> State -> HilbertSpace -> AlgebraElement`.
pub fn gns_cyclic_vector_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "A",
        cst("CStarAlgebra"),
        pi(
            BinderInfo::Default,
            "phi",
            cst("State"),
            arrow(hilbert_ty(), cst("AlgebraElement")),
        ),
    )
}
/// `TypeI_Factor : VonNeumannAlgebra -> Prop` -- has a minimal projection.
pub fn type_i_factor_ty() -> Expr {
    pi(BinderInfo::Default, "M", cst("VonNeumannAlgebra"), prop())
}
/// `TypeII1_Factor : VonNeumannAlgebra -> Prop` -- finite with unique tracial state.
pub fn type_ii1_factor_ty() -> Expr {
    pi(BinderInfo::Default, "M", cst("VonNeumannAlgebra"), prop())
}
/// `TypeIII_Factor : VonNeumannAlgebra -> Prop` -- no semifinite normal trace.
pub fn type_iii_factor_ty() -> Expr {
    pi(BinderInfo::Default, "M", cst("VonNeumannAlgebra"), prop())
}
/// `HyperfiniteII1Factor : VonNeumannAlgebra -> Prop` -- unique hyperfinite II_1 factor R.
pub fn hyperfinite_ii1_factor_ty() -> Expr {
    pi(BinderInfo::Default, "M", cst("VonNeumannAlgebra"), prop())
}
/// `FredholmOperator : BoundedOperator -> Prop` -- finite-dimensional kernel and cokernel.
pub fn fredholm_operator_ty() -> Expr {
    pi(BinderInfo::Default, "T", cst("BoundedOperator"), prop())
}
/// `FredholmIndex : BoundedOperator -> Int` -- ind(T) = dim(ker T) - dim(coker T).
pub fn fredholm_index_ty() -> Expr {
    arrow(cst("BoundedOperator"), cst("Int"))
}
/// `CalkinAlgebra : HilbertSpace -> CStarAlgebra` -- Q(H) = B(H)/K(H).
pub fn calkin_algebra_ty() -> Expr {
    arrow(hilbert_ty(), cst("CStarAlgebra"))
}
/// `SpectralTripleAxiom : CStarAlgebra -> HilbertSpace -> BoundedOperator -> Prop`.
pub fn spectral_triple_axiom_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "A",
        cst("CStarAlgebra"),
        pi(
            BinderInfo::Default,
            "H",
            hilbert_ty(),
            arrow(cst("BoundedOperator"), prop()),
        ),
    )
}
/// `DiracOperator : HilbertSpace -> BoundedOperator` -- self-adjoint, compact resolvent.
pub fn dirac_operator_ty() -> Expr {
    arrow(hilbert_ty(), cst("BoundedOperator"))
}
/// `KGroupK0 : CStarAlgebra -> AbelianGroup` -- Grothendieck group of projections.
pub fn k_group_k0_ty() -> Expr {
    arrow(cst("CStarAlgebra"), cst("AbelianGroup"))
}
/// `KGroupK1 : CStarAlgebra -> AbelianGroup` -- connected components of invertibles.
pub fn k_group_k1_ty() -> Expr {
    arrow(cst("CStarAlgebra"), cst("AbelianGroup"))
}
/// `SixTermExactSequence : CStarAlgebra -> CStarAlgebra -> CStarAlgebra -> Prop`.
pub fn six_term_exact_sequence_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "I",
        cst("CStarAlgebra"),
        pi(
            BinderInfo::Default,
            "A",
            cst("CStarAlgebra"),
            arrow(cst("CStarAlgebra"), prop()),
        ),
    )
}
/// `KasparovKK : CStarAlgebra -> CStarAlgebra -> AbelianGroup` -- bivariant K-theory.
pub fn kasparov_kk_ty() -> Expr {
    arrow(
        cst("CStarAlgebra"),
        arrow(cst("CStarAlgebra"), cst("AbelianGroup")),
    )
}
/// `KasparovProduct : KKElement -> KKElement -> KKElement` -- intersection product.
pub fn kasparov_product_ty() -> Expr {
    arrow(cst("KKElement"), arrow(cst("KKElement"), cst("KKElement")))
}
/// `GroupCStarAlgebra : Group -> CStarAlgebra` -- full group C*-algebra C*(G).
pub fn group_c_star_algebra_ty() -> Expr {
    arrow(cst("Group"), cst("CStarAlgebra"))
}
/// `ReducedGroupCStarAlgebra : Group -> CStarAlgebra` -- reduced group C*-algebra C*_r(G).
pub fn reduced_group_c_star_ty() -> Expr {
    arrow(cst("Group"), cst("CStarAlgebra"))
}
/// `AmenableGroup : Group -> Prop` -- G amenable iff C*(G) = C*_r(G).
pub fn amenable_group_ty() -> Expr {
    pi(BinderInfo::Default, "G", cst("Group"), prop())
}
/// `CrossedProduct : CStarAlgebra -> Group -> GroupAction -> CStarAlgebra`.
pub fn crossed_product_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "A",
        cst("CStarAlgebra"),
        pi(
            BinderInfo::Default,
            "G",
            cst("Group"),
            arrow(cst("GroupAction"), cst("CStarAlgebra")),
        ),
    )
}
/// `CompletelyBoundedMap : OperatorSpace -> OperatorSpace -> Real -> Prop`.
pub fn completely_bounded_map_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "V",
        cst("OperatorSpace"),
        pi(
            BinderInfo::Default,
            "W",
            cst("OperatorSpace"),
            arrow(real_ty(), prop()),
        ),
    )
}
/// `HaagerupTensorProduct : OperatorSpace -> OperatorSpace -> OperatorSpace`.
pub fn haagerup_tensor_product_ty() -> Expr {
    arrow(
        cst("OperatorSpace"),
        arrow(cst("OperatorSpace"), cst("OperatorSpace")),
    )
}
/// `InjectiveOperatorSystem : CStarAlgebra -> Prop` -- completely positive extensions exist.
pub fn injective_operator_system_ty() -> Expr {
    pi(BinderInfo::Default, "A", cst("CStarAlgebra"), prop())
}
/// `ExtGroup : CStarAlgebra -> CStarAlgebra -> AbelianGroup` -- BDF Ext group.
pub fn ext_group_ty() -> Expr {
    arrow(
        cst("CStarAlgebra"),
        arrow(cst("CStarAlgebra"), cst("AbelianGroup")),
    )
}
/// `RieszFunctionalCalculus : BoundedOperator -> ContinuousFunctions -> BoundedOperator`.
pub fn riesz_functional_calculus_ty() -> Expr {
    arrow(
        cst("BoundedOperator"),
        arrow(cst("ContinuousFunctions"), cst("BoundedOperator")),
    )
}
/// `BorelFunctionalCalculus : BoundedOperator -> BorelFunction -> BoundedOperator`.
pub fn borel_functional_calculus_ty() -> Expr {
    arrow(
        cst("BoundedOperator"),
        arrow(cst("BorelFunction"), cst("BoundedOperator")),
    )
}
/// `AtiyahSingerIndex : SpectralTriple -> Int`.
pub fn atiyah_singer_index_ty() -> Expr {
    arrow(cst("SpectralTriple"), cst("Int"))
}
/// `ConnesChernCharacter : SpectralTriple -> CyclicCohomology`.
pub fn connes_chern_character_ty() -> Expr {
    arrow(cst("SpectralTriple"), cst("CyclicCohomology"))
}
/// `CyclicCohomology : CStarAlgebra -> Nat -> AbelianGroup`.
pub fn cyclic_cohomology_ty() -> Expr {
    arrow(cst("CStarAlgebra"), arrow(nat_ty(), cst("AbelianGroup")))
}
/// `MoritaEquivalence : CStarAlgebra -> CStarAlgebra -> Prop`.
pub fn morita_equivalence_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "A",
        cst("CStarAlgebra"),
        arrow(cst("CStarAlgebra"), prop()),
    )
}
/// `ConditionalExpectation : VonNeumannAlgebra -> VonNeumannAlgebra -> BoundedOperator`.
pub fn conditional_expectation_ty() -> Expr {
    arrow(
        cst("VonNeumannAlgebra"),
        arrow(cst("VonNeumannAlgebra"), cst("BoundedOperator")),
    )
}
/// Populate an `Environment` with operator algebra axioms.
pub fn build_operator_algebras_env() -> Environment {
    let mut env = Environment::new();
    let base_types: &[(&str, Expr)] = &[
        ("CStarAlgebra", type1()),
        ("VonNeumannAlgebra", type1()),
        ("HilbertSpace", type1()),
        ("BoundedOperator", type1()),
        ("PositiveOperator", type1()),
        ("State", type1()),
        ("StarHomomorphism", type1()),
        ("StarAutomorphism", type1()),
        ("AlgebraElement", type1()),
        ("SubsetComplex", type1()),
        ("ContinuousFunctions", type1()),
        ("CommutativeCStarAlgebra", type1()),
        ("VonNeumannAlgebraType", type1()),
        ("Projection", type1()),
        ("SpectralMeasure", type1()),
        ("NormalOperator", type1()),
        ("TracialState", type1()),
        ("FaithfulState", type1()),
        ("GNSTriple", type1()),
        ("ModularTheory", type1()),
        ("ConvexSet", type1()),
        ("AbelianGroup", type1()),
        ("KKElement", type1()),
        ("Group", type1()),
        ("GroupAction", type1()),
        ("OperatorSpace", type1()),
        ("BorelFunction", type1()),
        ("SpectralTriple", type1()),
        ("CyclicCohomology", type1()),
        ("Int", type1()),
    ];
    for (name, ty) in base_types {
        env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty: ty.clone(),
        })
        .ok();
    }
    let helper_fns: &[(&str, Expr)] = &[
        (
            "TensorProduct",
            arrow(
                cst("CStarAlgebra"),
                arrow(cst("CStarAlgebra"), cst("CStarAlgebra")),
            ),
        ),
        (
            "DirectSum",
            arrow(
                cst("CStarAlgebra"),
                arrow(cst("CStarAlgebra"), cst("CStarAlgebra")),
            ),
        ),
        ("Commutant", commutant_ty()),
        (
            "Bicommutant",
            arrow(cst("CStarAlgebra"), cst("CStarAlgebra")),
        ),
        ("Norm", arrow(cst("AlgebraElement"), real_ty())),
        (
            "Involution",
            arrow(cst("AlgebraElement"), cst("AlgebraElement")),
        ),
        (
            "ReducedCrossedProduct",
            arrow(
                cst("CStarAlgebra"),
                arrow(cst("Group"), cst("CStarAlgebra")),
            ),
        ),
        (
            "FullCrossedProduct",
            arrow(
                cst("CStarAlgebra"),
                arrow(cst("Group"), cst("CStarAlgebra")),
            ),
        ),
        ("UHFAlgebra", arrow(nat_ty(), cst("CStarAlgebra"))),
        ("AFAlgebra", type1()),
    ];
    for (name, ty) in helper_fns {
        env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty: ty.clone(),
        })
        .ok();
    }
    let type_axioms: &[(&str, fn() -> Expr)] = &[
        ("GNSConstruction", gns_construction_ty),
        ("GNSRepresentation", gns_representation_ty),
        ("KMSState", kms_state_ty),
        ("ModularAutomorphismGroup", modular_automorphism_ty),
        ("ModularConjugation", modular_conjugation_ty),
        ("ModularOperator", modular_operator_ty),
        ("Spectrum", spectrum_ty),
        ("SpectralRadius", spectral_radius_ty),
        ("GelfandTransform", gelfand_transform_ty),
    ];
    for (name, mk_ty) in type_axioms {
        env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty: mk_ty(),
        })
        .ok();
    }
    let theorem_axioms: &[(&str, fn() -> Expr)] = &[
        ("bicommutant_theorem", bicommutant_theorem_ty),
        ("tomita_takesaki_theorem", tomita_takesaki_theorem_ty),
        ("spectral_theorem", spectral_theorem_ty),
        (
            "continuous_functional_calculus",
            continuous_functional_calculus_ty,
        ),
        ("gelfand_naimark_theorem", gelfand_naimark_theorem_ty),
        ("connes_classification", connes_classification_ty),
        ("haagerup_property", haagerup_property_ty),
        ("nuclear_c_star_algebra", nuclear_c_star_ty),
        ("injective_factor", injective_factor_ty),
    ];
    for (name, mk_ty) in theorem_axioms {
        env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty: mk_ty(),
        })
        .ok();
    }
    let new_prop_axioms: &[(&str, fn() -> Expr)] = &[
        ("pure_state", pure_state_ty),
        ("type_i_factor", type_i_factor_ty),
        ("type_ii1_factor", type_ii1_factor_ty),
        ("type_iii_factor", type_iii_factor_ty),
        ("hyperfinite_ii1_factor", hyperfinite_ii1_factor_ty),
        ("fredholm_operator", fredholm_operator_ty),
        ("spectral_triple_axiom", spectral_triple_axiom_ty),
        ("six_term_exact_sequence", six_term_exact_sequence_ty),
        ("amenable_group", amenable_group_ty),
        ("completely_bounded_map", completely_bounded_map_ty),
        ("injective_operator_system", injective_operator_system_ty),
        ("morita_equivalence", morita_equivalence_ty),
    ];
    for (name, mk_ty) in new_prop_axioms {
        env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty: mk_ty(),
        })
        .ok();
    }
    let new_fn_axioms: &[(&str, fn() -> Expr)] = &[
        ("state_space", state_space_ty),
        ("gns_cyclic_vector", gns_cyclic_vector_ty),
        ("fredholm_index", fredholm_index_ty),
        ("calkin_algebra", calkin_algebra_ty),
        ("dirac_operator", dirac_operator_ty),
        ("k_group_k0", k_group_k0_ty),
        ("k_group_k1", k_group_k1_ty),
        ("kasparov_kk", kasparov_kk_ty),
        ("kasparov_product", kasparov_product_ty),
        ("group_c_star_algebra", group_c_star_algebra_ty),
        ("reduced_group_c_star", reduced_group_c_star_ty),
        ("crossed_product", crossed_product_ty),
        ("haagerup_tensor_product", haagerup_tensor_product_ty),
        ("ext_group", ext_group_ty),
        ("riesz_functional_calculus", riesz_functional_calculus_ty),
        ("borel_functional_calculus", borel_functional_calculus_ty),
        ("atiyah_singer_index", atiyah_singer_index_ty),
        ("connes_chern_character", connes_chern_character_ty),
        ("cyclic_cohomology", cyclic_cohomology_ty),
        ("conditional_expectation", conditional_expectation_ty),
    ];
    for (name, mk_ty) in new_fn_axioms {
        env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty: mk_ty(),
        })
        .ok();
    }
    env
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_factor_type_display() {
        assert_eq!(FactorType::TypeII1.to_string(), "II_1");
        assert_eq!(FactorType::TypeI(Some(3)).to_string(), "I_3");
        assert_eq!(FactorType::TypeIII1.to_string(), "III_1");
    }
    #[test]
    fn test_c_star_algebra_matrix() {
        let m2 = CStarAlgebraData::matrix_algebra(2);
        assert_eq!(m2.dimension, Some(4));
        assert!(m2.is_nuclear);
        assert!(m2.is_simple);
        assert!(!m2.is_commutative);
        assert!(m2.satisfies_uct());
    }
    #[test]
    fn test_c_star_algebra_commutative() {
        let cx = CStarAlgebraData::continuous_functions("S^1", 2);
        assert!(cx.is_commutative);
        assert!(cx.is_nuclear);
        assert_eq!(cx.k0_rank, 2);
        assert!(cx.is_af());
    }
    #[test]
    fn test_state_kms_condition() {
        let state = StateData::kms("phi_beta", 2.5);
        assert!(state.check_kms_condition());
        assert!(state.is_faithful);
        assert!(!state.is_tracial);
        assert_eq!(state.beta, Some(2.5));
    }
    #[test]
    fn test_gns_triple_tomita_takesaki() {
        let alg = CStarAlgebraData::matrix_algebra(3);
        let state = StateData::tracial("tr");
        let gns = GNSTripleData::build(&alg, &state);
        assert!(gns.omega_cyclic);
        assert!(gns.omega_separating);
        assert!(gns.tomita_takesaki_applies());
    }
    #[test]
    fn test_modular_theory_ii1() {
        let mod_data = ModularTheoryData::for_ii1_factor("R");
        assert!(mod_data.modular_is_inner());
        assert_eq!(mod_data.modular_automorphism_at(1.0), 0.0);
    }
    #[test]
    fn test_spectrum_self_adjoint() {
        let spec = OperatorSpectrum::self_adjoint("T", vec![-2.0, 0.0, 3.0]);
        assert!(spec.is_self_adjoint);
        assert!(spec.is_normal);
        assert_eq!(spec.compute_radius(), 3.0);
        assert!(!spec.is_positive);
    }
    #[test]
    fn test_operator_algebra_registry() {
        let reg = OperatorAlgebraRegistry::with_standard_examples();
        assert!(reg.c_star_algebras.contains_key("M_2(C)"));
        assert!(reg.c_star_algebras.contains_key("K(H)"));
        assert!(reg.von_neumann_algebras.contains_key("R"));
        assert!(reg.gns_triples.contains_key("M_2(C)::tr"));
        assert!(reg.total_count() >= 6);
    }
    #[test]
    fn test_build_operator_algebras_env() {
        let env = build_operator_algebras_env();
        assert!(env.get(&Name::str("CStarAlgebra")).is_some());
        assert!(env.get(&Name::str("VonNeumannAlgebra")).is_some());
        assert!(env.get(&Name::str("GNSConstruction")).is_some());
        assert!(env.get(&Name::str("KMSState")).is_some());
        assert!(env.get(&Name::str("tomita_takesaki_theorem")).is_some());
        assert!(env.get(&Name::str("gelfand_naimark_theorem")).is_some());
        assert!(env.get(&Name::str("connes_classification")).is_some());
    }
    #[test]
    fn test_new_axioms_registered() {
        let env = build_operator_algebras_env();
        assert!(env.get(&Name::str("ConvexSet")).is_some());
        assert!(env.get(&Name::str("AbelianGroup")).is_some());
        assert!(env.get(&Name::str("KKElement")).is_some());
        assert!(env.get(&Name::str("OperatorSpace")).is_some());
        assert!(env.get(&Name::str("pure_state")).is_some());
        assert!(env.get(&Name::str("type_i_factor")).is_some());
        assert!(env.get(&Name::str("type_ii1_factor")).is_some());
        assert!(env.get(&Name::str("type_iii_factor")).is_some());
        assert!(env.get(&Name::str("hyperfinite_ii1_factor")).is_some());
        assert!(env.get(&Name::str("fredholm_operator")).is_some());
        assert!(env.get(&Name::str("spectral_triple_axiom")).is_some());
        assert!(env.get(&Name::str("six_term_exact_sequence")).is_some());
        assert!(env.get(&Name::str("amenable_group")).is_some());
        assert!(env.get(&Name::str("completely_bounded_map")).is_some());
        assert!(env.get(&Name::str("injective_operator_system")).is_some());
        assert!(env.get(&Name::str("morita_equivalence")).is_some());
        assert!(env.get(&Name::str("state_space")).is_some());
        assert!(env.get(&Name::str("fredholm_index")).is_some());
        assert!(env.get(&Name::str("calkin_algebra")).is_some());
        assert!(env.get(&Name::str("k_group_k0")).is_some());
        assert!(env.get(&Name::str("k_group_k1")).is_some());
        assert!(env.get(&Name::str("kasparov_kk")).is_some());
        assert!(env.get(&Name::str("kasparov_product")).is_some());
        assert!(env.get(&Name::str("group_c_star_algebra")).is_some());
        assert!(env.get(&Name::str("reduced_group_c_star")).is_some());
        assert!(env.get(&Name::str("crossed_product")).is_some());
        assert!(env.get(&Name::str("haagerup_tensor_product")).is_some());
        assert!(env.get(&Name::str("ext_group")).is_some());
        assert!(env.get(&Name::str("riesz_functional_calculus")).is_some());
        assert!(env.get(&Name::str("borel_functional_calculus")).is_some());
        assert!(env.get(&Name::str("atiyah_singer_index")).is_some());
        assert!(env.get(&Name::str("connes_chern_character")).is_some());
        assert!(env.get(&Name::str("cyclic_cohomology")).is_some());
        assert!(env.get(&Name::str("conditional_expectation")).is_some());
    }
    #[test]
    fn test_finite_matrix_identity() {
        let id = FiniteMatrix::identity(3);
        assert_eq!(id.trace(), 3.0);
        assert!(id.is_self_adjoint());
        let id2 = id.matmul(&id).expect("matmul should succeed");
        assert_eq!(id2.data, id.data);
    }
    #[test]
    fn test_finite_matrix_matmul() {
        let a = FiniteMatrix::from_data(2, vec![1.0, 2.0, 3.0, 4.0])
            .expect("FiniteMatrix::from_data should succeed");
        let b = FiniteMatrix::from_data(2, vec![5.0, 6.0, 7.0, 8.0])
            .expect("FiniteMatrix::from_data should succeed");
        let c = a.matmul(&b).expect("matmul should succeed");
        assert_eq!(c.get(0, 0), 19.0);
        assert_eq!(c.get(0, 1), 22.0);
        assert_eq!(c.get(1, 0), 43.0);
        assert_eq!(c.get(1, 1), 50.0);
    }
    #[test]
    fn test_finite_matrix_trace() {
        let m = FiniteMatrix::from_data(2, vec![3.0, 1.0, 1.0, 5.0])
            .expect("FiniteMatrix::from_data should succeed");
        assert_eq!(m.trace(), 8.0);
    }
    #[test]
    fn test_finite_matrix_adjoint() {
        let m = FiniteMatrix::from_data(2, vec![1.0, 2.0, 3.0, 4.0])
            .expect("FiniteMatrix::from_data should succeed");
        let adj = m.adjoint();
        assert_eq!(adj.get(0, 1), 3.0);
        assert_eq!(adj.get(1, 0), 2.0);
    }
    #[test]
    fn test_commutator_norm() {
        let a = FiniteMatrix::from_data(2, vec![1.0, 0.0, 0.0, 2.0])
            .expect("FiniteMatrix::from_data should succeed");
        let b = FiniteMatrix::from_data(2, vec![0.0, 1.0, 1.0, 0.0])
            .expect("FiniteMatrix::from_data should succeed");
        let cn = a
            .commutator_norm(&b)
            .expect("commutator_norm should succeed");
        assert!(cn > 0.0);
    }
    #[test]
    fn test_spectral_decompose_2x2() {
        let m = FiniteMatrix::from_data(2, vec![5.0, 2.0, 2.0, 2.0])
            .expect("FiniteMatrix::from_data should succeed");
        let (l1, l2, _v1, _v2) = m
            .spectral_decompose_2x2()
            .expect("spectral_decompose_2x2 should succeed");
        let sum = l1 + l2;
        let prod = l1 * l2;
        assert!((sum - 7.0).abs() < 1e-9);
        assert!((prod - 6.0).abs() < 1e-9);
    }
    #[test]
    fn test_c_star_elem_identity() {
        let id = FiniteMatrix::identity(2);
        let elem = CStarElem::from_matrix("I", id);
        assert!(elem.is_self_adjoint);
        assert!(elem.is_normal);
        assert!(elem.is_unitary);
        assert!(elem.is_projection);
        assert!(elem.involution().is_some());
    }
    #[test]
    fn test_gns_sim_tracial_state() {
        let gns = GNSRepresentationSim::for_matrix_algebra(2);
        let id = FiniteMatrix::identity(2);
        let val = gns
            .evaluate_state(&id)
            .expect("evaluate_state should succeed");
        assert!((val - 1.0).abs() < 1e-10);
        assert!(gns.verify_gns_property(&id));
    }
    #[test]
    fn test_fredholm_index() {
        let d_plus = FredholmData::new("D+", 0, 3);
        assert_eq!(d_plus.index(), -3);
        let d_minus = FredholmData::new("D-", 3, 0);
        assert_eq!(d_minus.index(), 3);
        let d_zero = FredholmData::new("D0", 2, 2);
        assert_eq!(d_zero.index(), 0);
        assert!(d_zero.index_stable_under_compact_perturbation(&FredholmData::new("D0p", 1, 1)));
        assert!(d_plus.calkin_invertible());
    }
    #[test]
    fn test_k_theory_matrix_algebra() {
        let k = KTheoryData::matrix_algebra(3);
        assert_eq!(k.k0_rank(), 1);
        assert_eq!(k.k1_rank(), 0);
        assert!(k.k0_torsion_free());
        assert_eq!(k.unit_class, 3);
        assert_eq!(k.total_betti(), 1);
    }
    #[test]
    fn test_k_theory_cuntz() {
        let k = KTheoryData::cuntz_algebra(2);
        assert_eq!(k.k0_summands, vec![1]);
        assert_eq!(k.k0_rank(), 0);
    }
    #[test]
    fn test_k_theory_display() {
        let k = KTheoryData::circle();
        let s = k.to_string();
        assert!(s.contains("K0=Z"));
        assert!(s.contains("K1=Z"));
    }
    #[test]
    fn test_operator_space_data() {
        let col = OperatorSpaceData::column_hilbert(4);
        assert!(col.is_column_hilbert_space);
        assert!(!col.is_self_dual);
        assert!(col.haagerup_inequality_holds());
        let oh = OperatorSpaceData::operator_hilbert(3);
        assert!(oh.is_self_dual);
        assert!(oh.haagerup_inequality_holds());
    }
}
#[cfg(test)]
mod extended_operator_tests {
    use super::*;
    #[test]
    fn test_cp_map() {
        let qc = CompletelyPositiveMap::quantum_channel("M_n", "M_m");
        assert!(qc.is_unital);
        assert!(qc.is_trace_preserving);
        assert!(qc.stinespring_applies());
    }
    #[test]
    fn test_operator_system() {
        let os = OperatorSystem::finite("S", 4);
        assert!(os.arveson_extension_applies());
        assert!(os.cb_norm_description().contains("CB norm"));
    }
    #[test]
    fn test_crossed_product() {
        let cp = CrossedProduct::full("A", "Z");
        assert!(cp.is_full);
        assert!(cp.amenable_coincidence(true));
        assert!(!cp.amenable_coincidence(false));
    }
    #[test]
    fn test_finite_von_neumann() {
        let fvn = FiniteVonNeumann::hyperfinite();
        assert!(fvn.is_factor);
        assert_eq!(fvn.murray_von_neumann_type, "II_1");
        assert!(fvn.l2_space().contains("L²"));
    }
    #[test]
    fn test_spectral_triple() {
        let st = SpectralTripleData::torus(2);
        assert!(st.is_even);
        assert_eq!(st.spectral_dim, 2.0);
        assert!(st.distance_formula().contains("d(x,y)"));
    }
    #[test]
    fn test_haagerup() {
        let h = HaagerupProperty::free_group(2);
        assert!(h.has_haagerup);
        assert!(h.baum_connes_holds());
        assert!(h.group.contains("F_2"));
    }
}
#[cfg(test)]
mod extended_operator_tests2 {
    use super::*;
    #[test]
    fn test_nuclearity() {
        let n = NuclearityData::nuclear("A");
        assert!(n.is_nuclear);
        assert!(n.kirchberg_phillips_applies(true));
        assert!(!n.kirchberg_phillips_applies(false));
    }
    #[test]
    fn test_cuntz_semigroup() {
        let cs = CuntzSemigroup::z_stable("A ⊗ Z");
        assert!(cs.toms_winter_regularity());
    }
    #[test]
    fn test_af_algebra() {
        let car = AfAlgebra::car();
        assert!(car.elliott_invariant().contains("K0"));
        let uhf = AfAlgebra::uhf(2);
        assert!(uhf.name.contains("M_2"));
    }
    #[test]
    fn test_operator_inequalities() {
        let ks = OperatorInequality::kadison_schwarz();
        assert!(ks.description.contains("CP"));
        let gt = OperatorInequality::golden_thompson();
        assert!(
            gt.description.contains("Golden-Thompson") || gt.inequality_type.contains("Golden")
        );
    }
}
