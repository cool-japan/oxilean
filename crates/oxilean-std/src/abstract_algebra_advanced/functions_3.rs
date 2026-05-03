//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::functions::*;
use super::functions_2::*;
use super::types::{
    BrauerGroupElem, HilbertFunctionImpl, LatticeOrderedGroupImpl, TiltingModuleImpl,
    WittVectorImpl,
};

/// `WittGhostMap : ∀ (R : Type), WittVectors R → ∏ₙ R`
///
/// The ghost map (Witt components): W(R) → R^ℕ sending (a₀, a₁, …)
/// to the ghost components w_n = Σ_{d|n} d · a_d^{n/d}.
pub fn witt_ghost_map_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "R",
        type0(),
        arrow(
            app(cst("WittVectors"), bvar(0)),
            app(cst("ProdSeq"), bvar(1)),
        ),
    )
}
/// `OrderedGroup : Type → Prop`
///
/// An ordered (abelian) group: a group G with a total order compatible
/// with the group operation: a ≤ b ⇒ a+c ≤ b+c for all c.
pub fn ordered_group_ty() -> Expr {
    arrow(type0(), prop())
}
/// `LatticeOrderedRing : Type → Prop`
///
/// A lattice-ordered ring (l-ring): a ring R that is also a lattice
/// with a² ≥ 0 for all a and the lattice operations distributing over addition.
pub fn lattice_ordered_ring_ty() -> Expr {
    arrow(type0(), prop())
}
/// `ArchimedeanOrderedField : Type → Prop`
///
/// An Archimedean ordered field: an ordered field where for all a > 0
/// there exists n ∈ ℕ with n·1 > a. ℝ is the unique complete Archimedean ordered field.
pub fn archimedean_ordered_field_ty() -> Expr {
    arrow(type0(), prop())
}
/// `PositiveCone : ∀ (G : Type), OrderedGroup G → Type`
///
/// The positive cone P = { g ∈ G | g ≥ 0 } of an ordered group:
/// determines the order via a ≤ b ↔ b − a ∈ P.
pub fn positive_cone_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "G",
        type0(),
        arrow(app(cst("OrderedGroup"), bvar(0)), type0()),
    )
}
/// `HolderTheorem : ∀ (G : Type), OrderedGroup G → Prop`
///
/// Hölder's theorem: every Archimedean ordered group embeds into (ℝ, +).
pub fn holder_theorem_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "G",
        type0(),
        arrow(app(cst("OrderedGroup"), bvar(0)), prop()),
    )
}
/// `DerivedCategory : Type → Type`
///
/// The derived category D(A) of an abelian category A: the localisation of
/// the category of chain complexes by quasi-isomorphisms.
pub fn derived_category_ty() -> Expr {
    arrow(type0(), type0())
}
/// `BoundedDerivedCategory : Type → Type`
///
/// The bounded derived category D^b(A): full subcategory of D(A)
/// on complexes with bounded cohomology (H^n = 0 for |n| ≫ 0).
pub fn bounded_derived_category_ty() -> Expr {
    arrow(type0(), type0())
}
/// `TStructure : ∀ (D : Type), TriangulatedCategory D → Prop`
///
/// A t-structure on a triangulated category D: a pair (D^{≤0}, D^{≥0})
/// of full subcategories satisfying the axioms: Hom(D^{≤0}, D^{≥1}) = 0,
/// and every object fits in a truncation triangle.
pub fn t_structure_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "D",
        type0(),
        arrow(app(cst("TriangulatedCategory"), bvar(0)), prop()),
    )
}
/// `Heart : ∀ (D : Type), TStructure D → Type`
///
/// The heart of a t-structure: the abelian category D^{≤0} ∩ D^{≥0}.
/// The heart of the standard t-structure on D(A) is A itself.
pub fn heart_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "D",
        type0(),
        arrow(app(cst("TStructure"), bvar(0)), type0()),
    )
}
/// `Perversity : ∀ (X : Type), TopologicalSpace X → Type`
///
/// A perversity function: assigns to each stratum an integer controlling
/// the t-structure for perverse sheaves (intersection cohomology).
pub fn perversity_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        type0(),
        arrow(app(cst("TopologicalSpace"), bvar(0)), type0()),
    )
}
/// `PerverseSheaves : ∀ (X : Type), TopologicalSpace X → Type`
///
/// The category of perverse sheaves Perv(X): objects in D^b(Sh(X))
/// satisfying the perversity conditions with respect to a stratification.
pub fn perverse_sheaves_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        type0(),
        arrow(app(cst("TopologicalSpace"), bvar(0)), type0()),
    )
}
/// `RiemannHilbert : ∀ (X : Type), ComplexManifold X → Prop`
///
/// The Riemann-Hilbert correspondence: an equivalence between
/// regular holonomic D-modules and perverse sheaves on a complex manifold X.
pub fn riemann_hilbert_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        type0(),
        arrow(app(cst("ComplexManifold"), bvar(0)), prop()),
    )
}
/// `KoszulDualityForOperads : ∀ (P : Type), Operad P → Type`
///
/// Koszul duality for operads: the Koszul dual operad P^! such that
/// there is a quasi-isomorphism of bar constructions B(P^!) ≃ P^¡.
pub fn koszul_duality_operads_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "P",
        type0(),
        arrow(app(cst("Operad"), bvar(0)), type0()),
    )
}
/// `AInfinityMorphism : ∀ (A B : Type), AInfinityAlgebra A → AInfinityAlgebra B → Type`
///
/// A morphism of A_∞-algebras: a collection of maps fₙ: A^{⊗n} → B
/// of degree 1-n satisfying the A_∞-morphism equations.
pub fn a_infinity_morphism_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "A",
        type0(),
        pi(
            BinderInfo::Default,
            "B",
            type0(),
            arrow(
                app(cst("AInfinityAlgebra"), bvar(1)),
                arrow(app(cst("AInfinityAlgebra"), bvar(1)), type0()),
            ),
        ),
    )
}
/// `MinimalModel : ∀ (A : Type), DGAlgebra A → Type`
///
/// The minimal A_∞-model of a DGA: a minimal A_∞-algebra quasi-isomorphic
/// to A. Unique up to A_∞-isomorphism (Kadeishvili's theorem).
pub fn minimal_model_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "A",
        type0(),
        arrow(app(cst("DGAlgebra"), bvar(0)), type0()),
    )
}
/// `Formality : ∀ (A : Type), DGAlgebra A → Prop`
///
/// A DGA A is formal if it is quasi-isomorphic as an A_∞-algebra to H(A)
/// with zero differentials. Deligne-Griffiths-Morgan-Sullivan: compact Kähler
/// manifolds are formal.
pub fn formality_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "A",
        type0(),
        arrow(app(cst("DGAlgebra"), bvar(0)), prop()),
    )
}
/// `MaurerCartanEquation : ∀ (g : Type), DGLieAlgebra g → g → Prop`
///
/// The Maurer-Cartan equation in a dg Lie algebra g: dα + (1/2)\[α, α\] = 0.
/// Solutions α are Maurer-Cartan elements controlling deformation problems.
pub fn maurer_cartan_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "g",
        type0(),
        pi(
            BinderInfo::Default,
            "alpha",
            bvar(0),
            arrow(app(cst("DGLieAlgebra"), bvar(1)), prop()),
        ),
    )
}
/// Register all new advanced algebra axioms (Sections 46–56) into the environment.
pub fn register_abstract_algebra_advanced_ext3(env: &mut Environment) {
    let axioms: &[(&str, Expr)] = &[
        ("HopfComodule", hopf_comodule_ty()),
        ("HopfModule", hopf_module_ty()),
        ("BialgebraCoassociativity", bialgebra_coassociativity_ty()),
        ("HopfAntipodeAxiom", hopf_antipode_axiom_ty()),
        ("SmashProduct", smash_product_ty()),
        ("TakeuchiEquivalence", takeuchi_equivalence_ty()),
        ("TiltingModule", tilting_module_ty()),
        ("DerivedMoritaEquivalence", derived_morita_equivalence_ty()),
        ("TiltingEquivalence", tilting_equivalence_ty()),
        ("SiltingModule", silting_module_ty()),
        ("APRTilting", apr_tilting_ty()),
        ("GradedRing", graded_ring_ty()),
        ("HilbertSeries", hilbert_series_ty()),
        ("HilbertSyzygy", hilbert_syzygy_ty()),
        ("RegularSequenceGraded", regular_sequence_graded_ty()),
        ("GKDimension", gk_dimension_ty()),
        ("ProjectiveDimension", projective_dimension_ty()),
        ("InjectiveDimension", injective_dimension_ty()),
        ("GlobalDimension", global_dimension_ty()),
        ("WeakDimension", weak_dimension_ty()),
        ("AuslanderBuchsbaum", auslander_buchsbaum_ty()),
        ("SerreRegularity", serre_regularity_ty()),
        ("PrimeSpectrum", prime_spectrum_ty()),
        ("NoetherianRingAxiom", noetherian_ring_axiom_ty()),
        ("ArtinianRingAxiom", artinian_ring_axiom_ty()),
        ("PrimaryDecomposition", primary_decomposition_ty()),
        ("KrullDimension", krull_dimension_ty()),
        ("LocalRing", local_ring_ty()),
        ("RegularLocalRing", regular_local_ring_ty()),
        ("CohenMacaulay", cohen_macaulay_ty()),
        ("SolvableExtension", solvable_extension_ty()),
        ("RadicalExtension", radical_extension_ty()),
        ("AbelRuffini", abel_ruffini_ty()),
        ("InfiniteGaloisGroup", infinite_galois_group_ty()),
        ("InverseGaloisProblem", inverse_galois_problem_ty()),
        ("WedderburnLittleTheorem", wedderburn_little_theorem_ty()),
        ("ArtinWedderburn", artin_wedderburn_ty()),
        ("BrauerEquivalence", brauer_equivalence_ty()),
        ("BrauerGroupTsen", brauer_group_tsen_ty()),
        ("AlbertTheorem", albert_theorem_ty()),
        ("WittVectors", witt_vectors_ty()),
        ("WittVectorsP", witt_vectors_p_ty()),
        ("LambdaRing", lambda_ring_ty()),
        ("AdamsOperation", adams_operation_ty()),
        ("GrothendieckGroup", grothendieck_group_ty()),
        ("WittGhostMap", witt_ghost_map_ty()),
        ("OrderedGroup", ordered_group_ty()),
        ("LatticeOrderedRing", lattice_ordered_ring_ty()),
        ("ArchimedeanOrderedField", archimedean_ordered_field_ty()),
        ("PositiveCone", positive_cone_ty()),
        ("HolderTheorem", holder_theorem_ty()),
        ("DerivedCategory", derived_category_ty()),
        ("BoundedDerivedCategory", bounded_derived_category_ty()),
        ("TStructure", t_structure_ty()),
        ("Heart", heart_ty()),
        ("Perversity", perversity_ty()),
        ("PerverseSheaves", perverse_sheaves_ty()),
        ("RiemannHilbert", riemann_hilbert_ty()),
        ("KoszulDualityOperads", koszul_duality_operads_ty()),
        ("AInfinityMorphism", a_infinity_morphism_ty()),
        ("MinimalModel", minimal_model_ty()),
        ("Formality", formality_ty()),
        ("MaurerCartan", maurer_cartan_ty()),
    ];
    for (name, ty) in axioms {
        env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty: ty.clone(),
        })
        .ok();
    }
}
#[cfg(test)]
mod tests_ext3 {
    use super::*;
    fn ext3_env() -> Environment {
        let mut env = Environment::new();
        register_abstract_algebra_advanced_ext3(&mut env);
        env
    }
    #[test]
    fn test_hopf_comodule_smash_registered() {
        let env = ext3_env();
        assert!(env.get(&Name::str("HopfComodule")).is_some());
        assert!(env.get(&Name::str("HopfModule")).is_some());
        assert!(env.get(&Name::str("BialgebraCoassociativity")).is_some());
        assert!(env.get(&Name::str("HopfAntipodeAxiom")).is_some());
        assert!(env.get(&Name::str("SmashProduct")).is_some());
        assert!(env.get(&Name::str("TakeuchiEquivalence")).is_some());
    }
    #[test]
    fn test_tilting_morita_registered() {
        let env = ext3_env();
        assert!(env.get(&Name::str("TiltingModule")).is_some());
        assert!(env.get(&Name::str("DerivedMoritaEquivalence")).is_some());
        assert!(env.get(&Name::str("TiltingEquivalence")).is_some());
        assert!(env.get(&Name::str("SiltingModule")).is_some());
        assert!(env.get(&Name::str("APRTilting")).is_some());
    }
    #[test]
    fn test_graded_hilbert_registered() {
        let env = ext3_env();
        assert!(env.get(&Name::str("GradedRing")).is_some());
        assert!(env.get(&Name::str("HilbertSeries")).is_some());
        assert!(env.get(&Name::str("HilbertSyzygy")).is_some());
        assert!(env.get(&Name::str("RegularSequenceGraded")).is_some());
        assert!(env.get(&Name::str("GKDimension")).is_some());
    }
    #[test]
    fn test_homological_dimensions_registered() {
        let env = ext3_env();
        assert!(env.get(&Name::str("ProjectiveDimension")).is_some());
        assert!(env.get(&Name::str("InjectiveDimension")).is_some());
        assert!(env.get(&Name::str("GlobalDimension")).is_some());
        assert!(env.get(&Name::str("WeakDimension")).is_some());
        assert!(env.get(&Name::str("AuslanderBuchsbaum")).is_some());
        assert!(env.get(&Name::str("SerreRegularity")).is_some());
    }
    #[test]
    fn test_prime_spectrum_noetherian_registered() {
        let env = ext3_env();
        assert!(env.get(&Name::str("PrimeSpectrum")).is_some());
        assert!(env.get(&Name::str("NoetherianRingAxiom")).is_some());
        assert!(env.get(&Name::str("ArtinianRingAxiom")).is_some());
        assert!(env.get(&Name::str("PrimaryDecomposition")).is_some());
        assert!(env.get(&Name::str("KrullDimension")).is_some());
        assert!(env.get(&Name::str("LocalRing")).is_some());
        assert!(env.get(&Name::str("RegularLocalRing")).is_some());
        assert!(env.get(&Name::str("CohenMacaulay")).is_some());
    }
    #[test]
    fn test_galois_extensions_registered() {
        let env = ext3_env();
        assert!(env.get(&Name::str("SolvableExtension")).is_some());
        assert!(env.get(&Name::str("RadicalExtension")).is_some());
        assert!(env.get(&Name::str("AbelRuffini")).is_some());
        assert!(env.get(&Name::str("InfiniteGaloisGroup")).is_some());
        assert!(env.get(&Name::str("InverseGaloisProblem")).is_some());
    }
    #[test]
    fn test_brauer_wedderburn_registered() {
        let env = ext3_env();
        assert!(env.get(&Name::str("WedderburnLittleTheorem")).is_some());
        assert!(env.get(&Name::str("ArtinWedderburn")).is_some());
        assert!(env.get(&Name::str("BrauerEquivalence")).is_some());
        assert!(env.get(&Name::str("BrauerGroupTsen")).is_some());
        assert!(env.get(&Name::str("AlbertTheorem")).is_some());
    }
    #[test]
    fn test_witt_lambda_registered() {
        let env = ext3_env();
        assert!(env.get(&Name::str("WittVectors")).is_some());
        assert!(env.get(&Name::str("WittVectorsP")).is_some());
        assert!(env.get(&Name::str("LambdaRing")).is_some());
        assert!(env.get(&Name::str("AdamsOperation")).is_some());
        assert!(env.get(&Name::str("GrothendieckGroup")).is_some());
        assert!(env.get(&Name::str("WittGhostMap")).is_some());
    }
    #[test]
    fn test_ordered_groups_registered() {
        let env = ext3_env();
        assert!(env.get(&Name::str("OrderedGroup")).is_some());
        assert!(env.get(&Name::str("LatticeOrderedRing")).is_some());
        assert!(env.get(&Name::str("ArchimedeanOrderedField")).is_some());
        assert!(env.get(&Name::str("PositiveCone")).is_some());
        assert!(env.get(&Name::str("HolderTheorem")).is_some());
    }
    #[test]
    fn test_derived_categories_registered() {
        let env = ext3_env();
        assert!(env.get(&Name::str("DerivedCategory")).is_some());
        assert!(env.get(&Name::str("BoundedDerivedCategory")).is_some());
        assert!(env.get(&Name::str("TStructure")).is_some());
        assert!(env.get(&Name::str("Heart")).is_some());
        assert!(env.get(&Name::str("Perversity")).is_some());
        assert!(env.get(&Name::str("PerverseSheaves")).is_some());
        assert!(env.get(&Name::str("RiemannHilbert")).is_some());
    }
    #[test]
    fn test_koszul_a_infinity_registered() {
        let env = ext3_env();
        assert!(env.get(&Name::str("KoszulDualityOperads")).is_some());
        assert!(env.get(&Name::str("AInfinityMorphism")).is_some());
        assert!(env.get(&Name::str("MinimalModel")).is_some());
        assert!(env.get(&Name::str("Formality")).is_some());
        assert!(env.get(&Name::str("MaurerCartan")).is_some());
    }
    #[test]
    fn test_tilting_module_impl() {
        let tm = TiltingModuleImpl::new(
            vec!["P₁".to_string(), "τ⁻¹S₁".to_string()],
            vec![0, 1],
            true,
        );
        assert_eq!(tm.num_summands(), 2);
        assert_eq!(tm.projective_dimension(), 1);
        assert!(tm.is_classical());
        assert!(tm.ext_vanishing);
        let desc = tm.describe();
        assert!(desc.contains("Tilting module"));
        assert!(desc.contains("pd = 1"));
    }
    #[test]
    fn test_hilbert_function_impl() {
        let hf = HilbertFunctionImpl::polynomial_ring(2, 4);
        assert_eq!(hf.hilbert_fn[0], 1);
        assert_eq!(hf.hilbert_fn[1], 2);
        assert_eq!(hf.hilbert_fn[2], 3);
        assert_eq!(hf.hilbert_fn[3], 4);
        let desc = hf.describe();
        assert!(desc.contains("Hilbert function"));
    }
    #[test]
    fn test_witt_vector_impl() {
        let w = WittVectorImpl::teichmuller(3, 2, 3);
        assert_eq!(w.components[0], 2);
        assert_eq!(w.components[1], 0);
        assert_eq!(w.ghost_component(0), 2);
        let desc = w.describe();
        assert!(desc.contains("W_3"));
    }
    #[test]
    fn test_lattice_ordered_group_impl() {
        let log = LatticeOrderedGroupImpl::new("ℤ", vec![-3, -1, 0, 1, 3]);
        assert_eq!(LatticeOrderedGroupImpl::meet(-1, 2), -1);
        assert_eq!(LatticeOrderedGroupImpl::join(-1, 2), 2);
        assert_eq!(LatticeOrderedGroupImpl::positive_part(-3), 0);
        assert_eq!(LatticeOrderedGroupImpl::positive_part(3), 3);
        assert_eq!(LatticeOrderedGroupImpl::abs_val(-5), 5);
        assert!(log.check_decomposition());
        let desc = log.describe();
        assert!(desc.contains("Lattice-ordered group ℤ"));
    }
    #[test]
    fn test_brauer_group_elem() {
        let trivial = BrauerGroupElem::trivial("ℚ");
        assert!(trivial.is_split());
        assert_eq!(trivial.degree, 1);
        assert_eq!(trivial.schur_index(), 1);
        let quaternions = BrauerGroupElem::new("ℝ", 2, 2, "ℍ (quaternions)");
        assert!(!quaternions.is_split());
        assert_eq!(quaternions.schur_index(), 2);
        let desc = quaternions.describe();
        assert!(desc.contains("Br(ℝ)"));
        assert!(desc.contains("split: false"));
    }
}
