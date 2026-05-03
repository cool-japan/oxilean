//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::functions::*;
use super::types::{
    CliffordAlgebra, CoalgebraImpl, ExteriorAlgebra, FormalGroupLawImpl, FrobeniusAlgebraImpl,
    GradedAlgebraImpl, GradedModuleImpl, HeckeAlgebraElem, HopfAlgebraElem, KoszulComplex,
    NoetherianRing, ProfiniteGroup,
};

/// Register all new advanced algebra axioms (Sections 14–25) into the environment.
pub fn register_abstract_algebra_advanced_ext(env: &mut Environment) {
    let axioms: &[(&str, Expr)] = &[
        ("TensorAlgebra", tensor_algebra_ty()),
        ("TensorAlgebraUniversal", tensor_algebra_universal_ty()),
        ("SymmetricAlgebra", symmetric_algebra_ty()),
        (
            "SymmetricAlgebraUniversal",
            symmetric_algebra_universal_ty(),
        ),
        ("ExteriorAlgebra", exterior_algebra_ty()),
        ("ExteriorAlgebraUniversal", exterior_algebra_universal_ty()),
        ("WedgeProduct", wedge_product_ty()),
        ("CliffordAlgebra", clifford_algebra_ty()),
        ("CliffordAlgebraUniversal", clifford_algebra_universal_ty()),
        ("SpinGroup", spin_group_ty()),
        ("HopfAlgebra", hopf_algebra_ty()),
        ("Comultiplication", comultiplication_ty()),
        ("Counit", counit_ty()),
        ("Antipode", antipode_ty()),
        ("HopfAntipodeAntiHom", hopf_antipode_anti_hom_ty()),
        ("HopfAntipodeInvolutive", hopf_antipode_involutive_ty()),
        ("CStarAlgebra", c_star_algebra_ty()),
        ("Spectrum", spectrum_ty()),
        ("GelfandNaimark", gelfand_naimark_ty()),
        ("GNSConstruction", gns_construction_ty()),
        ("VonNeumannAlgebra", von_neumann_algebra_ty()),
        ("DoubleCommutant", double_commutant_ty()),
        ("TypeClassificationVNA", type_classification_vna_ty()),
        ("LieAlgebra", lie_algebra_ty()),
        (
            "UniversalEnvelopingAlgebra",
            universal_enveloping_algebra_ty(),
        ),
        ("PBWTheorem", pbw_theorem_ty()),
        ("UEAUniversalProperty", uea_universal_property_ty()),
        ("WeylAlgebra", weyl_algebra_ty()),
        ("WeylSimple", weyl_simple_ty()),
        ("OreExtension", ore_extension_ty()),
        ("OreCondition", ore_condition_ty()),
        ("SkewPolynomialRing", skew_polynomial_ring_ty()),
        ("MoritaEquivalent", morita_equivalent_ty()),
        ("MoritaContext", morita_context_ty()),
        ("MatrixMoritaEquiv", matrix_morita_equiv_ty()),
        ("MoritaInvariant", morita_invariant_ty()),
        ("GroupAlgebra", group_algebra_ty()),
        ("GroupRepresentation", group_representation_ty()),
        ("MaschkeTheorem", maschke_theorem_ty()),
        ("CharacterTheory", character_theory_ty()),
        ("KoszulAlgebra", koszul_algebra_ty()),
        ("KoszulDual", koszul_dual_ty()),
        ("AssociatedGradedAlgebra", associated_graded_algebra_ty()),
        ("GradedCommutativity", graded_commutativity_ty()),
        ("AzumayaAlgebra", azumaya_algebra_ty()),
        ("BrauerGroup", brauer_group_ty()),
        ("CentralSimpleAlgebra", central_simple_algebra_ty()),
        ("BrauerPeriod", brauer_period_ty()),
        ("DGAlgebra", dg_algebra_ty()),
        ("DGAlgebraMorphism", dg_algebra_morphism_ty()),
        ("AInfinityAlgebra", a_infinity_algebra_ty()),
        ("Operad", operad_ty()),
        ("OperadAlgebra", operad_algebra_ty()),
        ("BarConstruction", bar_construction_ty()),
        ("CobarConstruction", cobar_construction_ty()),
        ("EnrichedCategory", enriched_category_ty()),
        ("InfinityCategory", infinity_category_ty()),
        ("StableInfinityCategory", stable_infinity_category_ty()),
        (
            "SymmetricMonoidalInftyCategory",
            symmetric_monoidal_infty_cat_ty(),
        ),
        ("EInfinityAlgebra", e_infinity_algebra_ty()),
        ("SymplecticForm", symplectic_form_ty()),
        ("SymplecticModule", symplectic_module_ty()),
        ("SymplecticGroup", symplectic_group_ty()),
        ("WittGroup", witt_group_ty()),
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
/// `Coalgebra : Type → Prop`
///
/// A coalgebra over a field k: a vector space C with comultiplication Δ: C → C ⊗ C
/// and counit ε: C → k satisfying coassociativity and counit axioms.
pub fn coalgebra_ty() -> Expr {
    arrow(type0(), prop())
}
/// `CoalgebraMorphism : ∀ (C D : Type), Coalgebra C → Coalgebra D → Type`
///
/// A coalgebra morphism f : C → D that commutes with Δ and ε.
pub fn coalgebra_morphism_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "C",
        type0(),
        pi(
            BinderInfo::Default,
            "D",
            type0(),
            arrow(
                app(cst("Coalgebra"), bvar(1)),
                arrow(app(cst("Coalgebra"), bvar(1)), type0()),
            ),
        ),
    )
}
/// `Bialgebra : Type → Prop`
///
/// A bialgebra: both an algebra and a coalgebra with compatibility:
/// Δ is an algebra map, and ε is an algebra map.
/// A Hopf algebra is a bialgebra with antipode.
pub fn bialgebra_ty() -> Expr {
    arrow(type0(), prop())
}
/// `CoalgebraComodule : ∀ (C M : Type), Coalgebra C → Prop`
///
/// A right C-comodule M: a vector space M with a coaction ρ: M → M ⊗ C
/// satisfying coassociativity and counit compatibility.
pub fn coalgebra_comodule_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "C",
        type0(),
        pi(
            BinderInfo::Default,
            "M",
            type0(),
            arrow(app(cst("Coalgebra"), bvar(1)), prop()),
        ),
    )
}
/// `CoFreeCoalgebra : Type → Type`
///
/// The cofree coalgebra on a vector space V: the largest coalgebra
/// with a vector space map to V. Dual to the tensor algebra.
pub fn cofree_coalgebra_ty() -> Expr {
    arrow(type0(), type0())
}
/// `FrobeniusAlgebra : Type → Prop`
///
/// A Frobenius algebra: a finite-dimensional algebra (A, μ, η) with a
/// non-degenerate symmetric bilinear form ε: A → k such that
/// ε(ab, c) = ε(a, bc). Equivalently: A is both an algebra and a coalgebra
/// with Frobenius compatibility μ ∘ (id ⊗ Δ) = Δ ∘ μ = (Δ ⊗ id) ∘ μ.
pub fn frobenius_algebra_ty() -> Expr {
    arrow(type0(), prop())
}
/// `FrobeniusAlgebraComultiplication : ∀ (A : Type), FrobeniusAlgebra A → A → A ⊗ A`
///
/// The Frobenius comultiplication Δ_F : A → A ⊗ A determined by the form.
pub fn frobenius_comultiplication_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "A",
        type0(),
        arrow(
            app(cst("FrobeniusAlgebra"), bvar(0)),
            arrow(bvar(1), app2(cst("TensorProduct"), bvar(2), bvar(2))),
        ),
    )
}
/// `CommutativeFrobeniusAlgebra : Type → Prop`
///
/// A commutative Frobenius algebra: a 2D TQFT in the sense of Atiyah is
/// equivalent to a commutative Frobenius algebra.
pub fn commutative_frobenius_algebra_ty() -> Expr {
    arrow(type0(), prop())
}
/// `FrobeniusHommorphism : ∀ (A B : Type), FrobeniusAlgebra A → FrobeniusAlgebra B → Type`
///
/// A Frobenius homomorphism: a linear map that is simultaneously an algebra
/// and coalgebra morphism.
pub fn frobenius_homomorphism_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "A",
        type0(),
        pi(
            BinderInfo::Default,
            "B",
            type0(),
            arrow(
                app(cst("FrobeniusAlgebra"), bvar(1)),
                arrow(app(cst("FrobeniusAlgebra"), bvar(1)), type0()),
            ),
        ),
    )
}
/// `QuantumGroup : Type → Prop`
///
/// A quantum group: a Hopf algebra that is a q-deformation of the
/// universal enveloping algebra U(g) of a semisimple Lie algebra g,
/// denoted U_q(g) for q a formal parameter.
pub fn quantum_group_ty() -> Expr {
    arrow(type0(), prop())
}
/// `RMatrix : ∀ (H : Type), QuantumGroup H → H → H → H ⊗ H`
///
/// The R-matrix of a quantum group H: the universal R-matrix
/// R ∈ H ⊗ H satisfying the quantum Yang-Baxter equation (QYBE).
pub fn r_matrix_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "H",
        type0(),
        arrow(
            app(cst("QuantumGroup"), bvar(0)),
            arrow(
                bvar(1),
                arrow(bvar(2), app2(cst("TensorProduct"), bvar(3), bvar(3))),
            ),
        ),
    )
}
/// `QuantumYangBaxterEquation : ∀ (H : Type), QuantumGroup H → Prop`
///
/// The QYBE: R₁₂ R₁₃ R₂₃ = R₂₃ R₁₃ R₁₂ in H ⊗ H ⊗ H.
/// Solutions give representations of the braid group.
pub fn quantum_yang_baxter_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "H",
        type0(),
        arrow(app(cst("QuantumGroup"), bvar(0)), prop()),
    )
}
/// `DrinfeldDouble : ∀ (H : Type), HopfAlgebra H → Type`
///
/// The Drinfeld double D(H) of a finite-dimensional Hopf algebra H:
/// a quasitriangular Hopf algebra D(H) = H ⊗ H^{op,cop} with canonical R-matrix.
pub fn drinfeld_double_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "H",
        type0(),
        arrow(app(cst("HopfAlgebra"), bvar(0)), type0()),
    )
}
/// `RibbonCategory : Type → Prop`
///
/// A ribbon (tortile) category: a braided monoidal category with a twist θ,
/// satisfying the ribbon condition θ_{X ⊗ Y} = c_{Y,X} ∘ c_{X,Y} ∘ (θ_X ⊗ θ_Y).
/// Representations of quantum groups at roots of unity form ribbon categories.
pub fn ribbon_category_ty() -> Expr {
    arrow(type0(), prop())
}
/// `QuasitriangularHopfAlgebra : Type → Prop`
///
/// A quasitriangular Hopf algebra: a Hopf algebra H with a universal R-matrix
/// satisfying Δ^{op}(h) = R·Δ(h)·R⁻¹. The representation category is braided.
pub fn quasitriangular_hopf_algebra_ty() -> Expr {
    arrow(type0(), prop())
}
/// `HallAlgebra : Type → Type`
///
/// The Hall algebra of an abelian category A (e.g., modules over a finite field):
/// a free abelian group on iso-classes of objects with multiplication
/// counting extensions \[M\] ∗ \[N\] = Σ c^P_{M,N} \[P\].
pub fn hall_algebra_ty() -> Expr {
    arrow(type0(), type0())
}
/// `HallPolynomial : ∀ (R : Type) (M N P : Type), Nat`
///
/// The Hall polynomial g^P_{M,N}: the number of submodules Q ≤ P
/// with Q ≅ N and P/Q ≅ M, for R-modules over a finite ring.
pub fn hall_polynomial_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "R",
        type0(),
        pi(
            BinderInfo::Default,
            "M",
            type0(),
            pi(BinderInfo::Default, "N", type0(), arrow(type0(), nat_ty())),
        ),
    )
}
/// `RingelHallTheorem : ∀ (g : Type), LieAlgebra g → Prop`
///
/// Ringel's theorem: the Hall algebra of Rep(Q, 𝔽_q) for a Dynkin quiver Q
/// is isomorphic (after base change) to the positive part of U_q(g)
/// where g is the corresponding Kac-Moody algebra.
pub fn ringel_hall_theorem_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "g",
        type0(),
        arrow(app(cst("LieAlgebra"), bvar(0)), prop()),
    )
}
/// `SchurAlgebra : Nat → Nat → Type`
///
/// The Schur algebra S(n, r): the endomorphism algebra of the r-th tensor
/// power of the natural GL_n(k)-module. Controls polynomial representations of GL_n.
pub fn schur_algebra_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `SchurFunctor : ∀ (λ : Type), YoungDiagram λ → Type → Type`
///
/// The Schur functor S^λ associated to a partition λ: maps a vector space V
/// to the Schur module S^λ(V) — the image of the Young symmetrizer.
pub fn schur_functor_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "lam",
        type0(),
        arrow(app(cst("YoungDiagram"), bvar(0)), arrow(type0(), type0())),
    )
}
/// `SchurWeylDuality : ∀ (n r : Nat), Prop`
///
/// Schur-Weyl duality: the actions of GL_n(k) and the symmetric group S_r
/// on V^{⊗r} generate each other's centraliser algebras.
/// The Schur algebra S(n, r) is isomorphic to End_{S_r}(V^{⊗r}).
pub fn schur_weyl_duality_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), prop()))
}
/// `LieSuperalgebra : Type → Prop`
///
/// A Lie superalgebra: a ℤ/2-graded vector space g = g₀ ⊕ g₁ with a
/// graded bracket \[·,·\] satisfying graded antisymmetry and the graded Jacobi identity.
pub fn lie_superalgebra_ty() -> Expr {
    arrow(type0(), prop())
}
/// `LieSuperalgebraUEA : ∀ (g : Type), LieSuperalgebra g → Type`
///
/// Universal enveloping algebra of a Lie superalgebra:
/// U(g) = T(g) / ⟨xy − (−1)^{|x||y|}yx − \[x,y\]⟩.
pub fn lie_superalgebra_uea_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "g",
        type0(),
        arrow(app(cst("LieSuperalgebra"), bvar(0)), type0()),
    )
}
/// `SuperPBW : ∀ (g : Type), LieSuperalgebra g → Prop`
///
/// The PBW theorem for Lie superalgebras: an ordered basis of g gives
/// an explicit basis for U(g) as a superalgebra.
pub fn super_pbw_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "g",
        type0(),
        arrow(app(cst("LieSuperalgebra"), bvar(0)), prop()),
    )
}
/// `LieSupermodule : ∀ (g : Type), LieSuperalgebra g → Type → Prop`
///
/// A representation (module) for a Lie superalgebra g.
pub fn lie_supermodule_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "g",
        type0(),
        pi(
            BinderInfo::Default,
            "M",
            type0(),
            arrow(app(cst("LieSuperalgebra"), bvar(1)), prop()),
        ),
    )
}
/// `StarProduct : Type → Prop`
///
/// A star product on a Poisson manifold (M, {·,·}):
/// an associative ℝ[\[ℏ\]]-bilinear deformation of C^∞(M) with
/// f ⋆ g = fg + (iℏ/2){f,g} + O(ℏ²).
pub fn star_product_ty() -> Expr {
    arrow(type0(), prop())
}
/// `KontsevichFormality : ∀ (M : Type), PoissonManifold M → Prop`
///
/// Kontsevich's formality theorem: there exists a star product
/// quantizing any Poisson manifold, constructed via a sum over graphs.
pub fn kontsevich_formality_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "M",
        type0(),
        arrow(app(cst("PoissonManifold"), bvar(0)), prop()),
    )
}
/// `DeformationQuantization : ∀ (A : Type), PoissonAlgebra A → Type`
///
/// A deformation quantization of a Poisson algebra (A, {·,·}):
/// a flat ℝ[\[ℏ\]]-algebra deforming A with the semi-classical limit
/// (A ⊗ ℝ[\[ℏ\]]/ℏ, {f,g} = (fg − gf)/ℏ mod ℏ).
pub fn deformation_quantization_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "A",
        type0(),
        arrow(app(cst("PoissonAlgebra"), bvar(0)), type0()),
    )
}
/// `FormalGroupLaw : Type → Prop`
///
/// A formal group law over a ring R: a formal power series F(X, Y) ∈ R[\[X,Y\]]
/// satisfying F(X,0) = X, F(0,Y) = Y (unit), F(X, F(Y,Z)) = F(F(X,Y), Z) (associativity).
pub fn formal_group_law_ty() -> Expr {
    arrow(type0(), prop())
}
/// `FormalGroupLawAdditive : Type → Prop`
///
/// The additive formal group law G_a: F(X, Y) = X + Y.
/// Corresponds to the additive group scheme.
pub fn formal_group_law_additive_ty() -> Expr {
    arrow(type0(), prop())
}
/// `FormalGroupLawMultiplicative : Type → Prop`
///
/// The multiplicative formal group law G_m: F(X, Y) = X + Y + XY.
/// Corresponds to the multiplicative group scheme (1 + X)(1 + Y) = 1 + F(X,Y).
pub fn formal_group_law_multiplicative_ty() -> Expr {
    arrow(type0(), prop())
}
/// `FormalGroupLawHeight : ∀ (F : Type), FormalGroupLaw F → Nat`
///
/// The height of a formal group law over a field of characteristic p:
/// the unique n such that \[p\](X) = u·X^{p^n} + (higher terms), u ≠ 0.
/// Height 1 = ordinary, height n = supersingular (for elliptic curves).
pub fn formal_group_law_height_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "F",
        type0(),
        arrow(app(cst("FormalGroupLaw"), bvar(0)), nat_ty()),
    )
}
/// `LazardRing : Type`
///
/// The Lazard ring L: the universal ring for formal group laws.
/// The complex cobordism ring MU_* is isomorphic to the Lazard ring.
pub fn lazard_ring_ty() -> Expr {
    type0()
}
/// `HeckeAlgebra : ∀ (G : Type) (K : Type), IsGroup G → IsGroup K → Type`
///
/// The Hecke algebra H(G, K) = C_c(K\G/K): the algebra of K-bi-invariant
/// compactly supported functions on G, with convolution.
/// For G = GL_n(Q_p), K = GL_n(Z_p): gives the unramified Hecke algebra.
pub fn hecke_algebra_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "G",
        type0(),
        pi(
            BinderInfo::Default,
            "K",
            type0(),
            arrow(
                app(cst("IsGroup"), bvar(1)),
                arrow(app(cst("IsGroup"), bvar(1)), type0()),
            ),
        ),
    )
}
/// `IwahoriHeckeAlgebra : ∀ (W : Type) (q : Type), CoxeterGroup W → Type`
///
/// The Iwahori-Hecke algebra H(W, q): a deformation of the group algebra k\[W\]
/// of a Coxeter group W parameterised by q ∈ k.
/// At q = 1: recovers k\[W\]. At q = 0: the nil-Hecke algebra.
pub fn iwahori_hecke_algebra_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "W",
        type0(),
        pi(
            BinderInfo::Default,
            "q",
            type0(),
            arrow(app(cst("CoxeterGroup"), bvar(1)), type0()),
        ),
    )
}
/// `KazhdanLusztigPolynomial : ∀ (W : Type) (x y : W), Polynomial (W)`
///
/// The Kazhdan-Lusztig polynomials P_{x,y}(q) ∈ ℤ\[q\] for a Coxeter group W:
/// encode representation-theoretic data of Hecke algebras.
pub fn kazhdan_lusztig_polynomial_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "W",
        type0(),
        pi(
            BinderInfo::Default,
            "x",
            bvar(0),
            pi(
                BinderInfo::Default,
                "y",
                bvar(1),
                app(cst("Polynomial"), bvar(3)),
            ),
        ),
    )
}
/// `GroupCohomology : ∀ (G M : Type), IsGroup G → GroupModule G M → Nat → Type`
///
/// The group cohomology H^n(G, M): computed via cochains C^n(G, M) = {functions G^n → M}
/// with the standard coboundary map.
pub fn group_cohomology_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "G",
        type0(),
        pi(
            BinderInfo::Default,
            "M",
            type0(),
            arrow(
                app(cst("IsGroup"), bvar(1)),
                arrow(
                    app2(cst("GroupModule"), bvar(2), bvar(1)),
                    arrow(nat_ty(), type0()),
                ),
            ),
        ),
    )
}
/// `GroupHomology : ∀ (G M : Type), IsGroup G → GroupModule G M → Nat → Type`
///
/// The group homology H_n(G, M): computed via the standard bar resolution.
/// H_0(G, M) = M_G (coinvariants), H_1(G, ℤ) = G^{ab}.
pub fn group_homology_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "G",
        type0(),
        pi(
            BinderInfo::Default,
            "M",
            type0(),
            arrow(
                app(cst("IsGroup"), bvar(1)),
                arrow(
                    app2(cst("GroupModule"), bvar(2), bvar(1)),
                    arrow(nat_ty(), type0()),
                ),
            ),
        ),
    )
}
/// `TateCohomology : ∀ (G M : Type), IsFiniteGroup G → GroupModule G M → Type`
///
/// Tate cohomology Ĥ^n(G, M) for a finite group G:
/// extends group cohomology to all integers n ∈ ℤ.
/// Ĥ^{-1}(G, M) = M_G/Nm(M), Ĥ^0(G, M) = M^G/Nm(M).
pub fn tate_cohomology_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "G",
        type0(),
        pi(
            BinderInfo::Default,
            "M",
            type0(),
            arrow(
                app(cst("IsFiniteGroup"), bvar(1)),
                arrow(app2(cst("GroupModule"), bvar(2), bvar(1)), type0()),
            ),
        ),
    )
}
/// `ShapiroLemma : ∀ (G H M : Type), IsGroup G → IsSubgroup H G → GroupModule H M → Prop`
///
/// Shapiro's lemma: H^n(G, Ind_H^G(M)) ≅ H^n(H, M) for a subgroup H ≤ G
/// and H-module M, where Ind_H^G(M) is the induced G-module.
pub fn shapiro_lemma_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "G",
        type0(),
        pi(
            BinderInfo::Default,
            "H",
            type0(),
            pi(
                BinderInfo::Default,
                "M",
                type0(),
                arrow(
                    app(cst("IsGroup"), bvar(2)),
                    arrow(
                        app2(cst("IsSubgroup"), bvar(2), bvar(3)),
                        arrow(app2(cst("GroupModule"), bvar(3), bvar(1)), prop()),
                    ),
                ),
            ),
        ),
    )
}
/// `IwasawaAlgebra : ∀ (G : Type), ProfiniteGroup G → Type`
///
/// The Iwasawa algebra Λ(G) = ℤ_p[\[G\]] = lim← ℤ_p[G/U]
/// for a pro-p group G. Central to Iwasawa theory in algebraic number theory.
pub fn iwasawa_algebra_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "G",
        type0(),
        arrow(app(cst("ProfiniteGroup"), bvar(0)), type0()),
    )
}
/// `IwasawaModule : ∀ (G : Type), ProfiniteGroup G → Type → Prop`
///
/// An Iwasawa module: a finitely generated Λ(G)-module M, arising e.g.
/// from Galois cohomology of p-adic representations.
pub fn iwasawa_module_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "G",
        type0(),
        pi(
            BinderInfo::Default,
            "M",
            type0(),
            arrow(app(cst("ProfiniteGroup"), bvar(1)), prop()),
        ),
    )
}
/// `CharacteristicIdeal : ∀ (G M : Type), IwasawaModule G M → Type`
///
/// The characteristic ideal of a torsion Iwasawa module M:
/// the annihilator ideal in Λ(G) up to pseudo-isomorphism.
pub fn characteristic_ideal_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "G",
        type0(),
        pi(
            BinderInfo::Default,
            "M",
            type0(),
            arrow(app2(cst("IwasawaModule"), bvar(1), bvar(0)), type0()),
        ),
    )
}
/// `ProjectiveModule : ∀ (R M : Type), IsRing R → IsModule R M → Prop`
///
/// M is a projective R-module: every surjection N → M splits,
/// equivalently M is a direct summand of a free module.
pub fn projective_module_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "R",
        type0(),
        pi(
            BinderInfo::Default,
            "M",
            type0(),
            arrow(
                app(cst("IsRing"), bvar(1)),
                arrow(app2(cst("IsModule"), bvar(2), bvar(1)), prop()),
            ),
        ),
    )
}
/// `InjectiveModule : ∀ (R M : Type), IsRing R → IsModule R M → Prop`
///
/// M is an injective R-module: every injection M → N splits,
/// equivalently M is divisible (for abelian groups: Q, Q/ℤ are injective).
pub fn injective_module_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "R",
        type0(),
        pi(
            BinderInfo::Default,
            "M",
            type0(),
            arrow(
                app(cst("IsRing"), bvar(1)),
                arrow(app2(cst("IsModule"), bvar(2), bvar(1)), prop()),
            ),
        ),
    )
}
/// `FlatModule : ∀ (R M : Type), IsRing R → IsModule R M → Prop`
///
/// M is a flat R-module: tensoring with M preserves exact sequences.
/// Projective ⇒ Flat; flat + finitely presented ⇒ projective (Lazard).
pub fn flat_module_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "R",
        type0(),
        pi(
            BinderInfo::Default,
            "M",
            type0(),
            arrow(
                app(cst("IsRing"), bvar(1)),
                arrow(app2(cst("IsModule"), bvar(2), bvar(1)), prop()),
            ),
        ),
    )
}
/// `InjectiveEnvelope : ∀ (R M : Type), IsModule R M → Type`
///
/// The injective envelope (injective hull) E(M) of an R-module M:
/// the smallest injective module containing M as an essential submodule.
pub fn injective_envelope_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "R",
        type0(),
        pi(
            BinderInfo::Default,
            "M",
            type0(),
            arrow(app2(cst("IsModule"), bvar(1), bvar(0)), type0()),
        ),
    )
}
/// `ArtinianRing : Type → Prop`
///
/// R is an Artinian ring if every descending chain of ideals stabilises.
/// Artinian ⇒ Noetherian (Hopkins-Levitzki), Artinian = finite length ring.
pub fn artinian_ring_ty() -> Expr {
    arrow(type0(), prop())
}
/// `SemisimpleRing : Type → Prop`
///
/// A ring R is semisimple (in the sense of Artin-Wedderburn)
/// if every R-module is projective (equivalently: R is a direct product of
/// simple modules, equivalently: R is Artinian with rad(R) = 0).
pub fn semisimple_ring_ty() -> Expr {
    arrow(type0(), prop())
}
/// `JacobsonRadical : Type → Type`
///
/// The Jacobson radical J(R) of a ring R: the intersection of all maximal left ideals.
/// J(R) = 0 iff R is semiprimitive; Artinian rings are semisimple iff J(R) = 0.
pub fn jacobson_radical_ty() -> Expr {
    arrow(type0(), type0())
}
/// `NakayamaLemma : ∀ (R M : Type), IsRing R → IsFinGenModule R M → Prop`
///
/// Nakayama's lemma: if M is a finitely generated R-module and J(R)M = M, then M = 0.
/// Critical tool in commutative algebra and algebraic geometry.
pub fn nakayama_lemma_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "R",
        type0(),
        pi(
            BinderInfo::Default,
            "M",
            type0(),
            arrow(
                app(cst("IsRing"), bvar(1)),
                arrow(app2(cst("IsFinGenModule"), bvar(2), bvar(1)), prop()),
            ),
        ),
    )
}
/// `HopkinsLevitzki : ∀ (R : Type), ArtinianRing R → NoetherianRing R`
///
/// The Hopkins-Levitzki theorem: every left Artinian ring is left Noetherian.
pub fn hopkins_levitzki_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "R",
        type0(),
        arrow(
            app(cst("ArtinianRing"), bvar(0)),
            app(cst("NoetherianRing"), bvar(1)),
        ),
    )
}
/// `GradedModule : ∀ (R : Type), IsRing R → Type → Prop`
///
/// A graded R-module: M = ⊕_{n∈ℤ} M_n with R_{i}·M_j ⊆ M_{i+j}
/// for a graded ring R = ⊕ R_n.
pub fn graded_module_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "R",
        type0(),
        pi(
            BinderInfo::Default,
            "M",
            type0(),
            arrow(app(cst("IsRing"), bvar(1)), prop()),
        ),
    )
}
/// `GradedModuleShift : ∀ (R M : Type) (n : Nat), GradedModule R M → GradedModule R M`
///
/// Degree shift M(n): the graded module with M(n)_k = M_{n+k}.
pub fn graded_module_shift_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "R",
        type0(),
        pi(
            BinderInfo::Default,
            "M",
            type0(),
            pi(
                BinderInfo::Default,
                "n",
                nat_ty(),
                arrow(
                    app2(cst("GradedModule"), bvar(2), bvar(1)),
                    app2(cst("GradedModule"), bvar(3), bvar(2)),
                ),
            ),
        ),
    )
}
/// `FilteredModule : ∀ (R M : Type), IsRing R → IsModule R M → Prop`
///
/// A filtered module over a filtered ring R:
/// M = F₀M ⊇ F₁M ⊇ F₂M ⊇ … with Fᵢ(R)·Fⱼ(M) ⊆ Fᵢ₊ⱼ(M).
pub fn filtered_module_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "R",
        type0(),
        pi(
            BinderInfo::Default,
            "M",
            type0(),
            arrow(
                app(cst("IsRing"), bvar(1)),
                arrow(app2(cst("IsModule"), bvar(2), bvar(1)), prop()),
            ),
        ),
    )
}
/// `AssociatedGradedModule : ∀ (R M : Type), FilteredModule R M → GradedModule R M`
///
/// The associated graded module gr(M) = ⊕ Fₙ(M)/F_{n+1}(M).
pub fn associated_graded_module_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "R",
        type0(),
        pi(
            BinderInfo::Default,
            "M",
            type0(),
            arrow(
                app2(cst("FilteredModule"), bvar(1), bvar(0)),
                app2(cst("GradedModule"), bvar(2), bvar(1)),
            ),
        ),
    )
}
/// Register all new advanced algebra axioms (Sections 29–42) into the environment.
pub fn register_abstract_algebra_advanced_ext2(env: &mut Environment) {
    let axioms: &[(&str, Expr)] = &[
        ("Coalgebra", coalgebra_ty()),
        ("CoalgebraMorphism", coalgebra_morphism_ty()),
        ("Bialgebra", bialgebra_ty()),
        ("CoalgebraComodule", coalgebra_comodule_ty()),
        ("CofreeCoalgebra", cofree_coalgebra_ty()),
        ("FrobeniusAlgebra", frobenius_algebra_ty()),
        ("FrobeniusComultiplication", frobenius_comultiplication_ty()),
        (
            "CommutativeFrobeniusAlgebra",
            commutative_frobenius_algebra_ty(),
        ),
        ("FrobeniusHomomorphism", frobenius_homomorphism_ty()),
        ("QuantumGroup", quantum_group_ty()),
        ("RMatrix", r_matrix_ty()),
        ("QuantumYangBaxterEquation", quantum_yang_baxter_ty()),
        ("DrinfeldDouble", drinfeld_double_ty()),
        ("RibbonCategory", ribbon_category_ty()),
        (
            "QuasitriangularHopfAlgebra",
            quasitriangular_hopf_algebra_ty(),
        ),
        ("HallAlgebra", hall_algebra_ty()),
        ("HallPolynomial", hall_polynomial_ty()),
        ("RingelHallTheorem", ringel_hall_theorem_ty()),
        ("SchurAlgebra", schur_algebra_ty()),
        ("SchurFunctor", schur_functor_ty()),
        ("SchurWeylDuality", schur_weyl_duality_ty()),
        ("LieSuperalgebra", lie_superalgebra_ty()),
        ("LieSuperalgebraUEA", lie_superalgebra_uea_ty()),
        ("SuperPBW", super_pbw_ty()),
        ("LieSupermodule", lie_supermodule_ty()),
        ("StarProduct", star_product_ty()),
        ("KontsevichFormality", kontsevich_formality_ty()),
        ("DeformationQuantization", deformation_quantization_ty()),
        ("FormalGroupLaw", formal_group_law_ty()),
        ("FormalGroupLawAdditive", formal_group_law_additive_ty()),
        (
            "FormalGroupLawMultiplicative",
            formal_group_law_multiplicative_ty(),
        ),
        ("FormalGroupLawHeight", formal_group_law_height_ty()),
        ("LazardRing", lazard_ring_ty()),
        ("HeckeAlgebra", hecke_algebra_ty()),
        ("IwahoriHeckeAlgebra", iwahori_hecke_algebra_ty()),
        ("KazhdanLusztigPolynomial", kazhdan_lusztig_polynomial_ty()),
        ("GroupCohomology", group_cohomology_ty()),
        ("GroupHomology", group_homology_ty()),
        ("TateCohomology", tate_cohomology_ty()),
        ("ShapiroLemma", shapiro_lemma_ty()),
        ("IwasawaAlgebra", iwasawa_algebra_ty()),
        ("IwasawaModule", iwasawa_module_ty()),
        ("CharacteristicIdeal", characteristic_ideal_ty()),
        ("ProjectiveModule", projective_module_ty()),
        ("InjectiveModule", injective_module_ty()),
        ("FlatModule", flat_module_ty()),
        ("InjectiveEnvelope", injective_envelope_ty()),
        ("ArtinianRing", artinian_ring_ty()),
        ("SemisimpleRing", semisimple_ring_ty()),
        ("JacobsonRadical", jacobson_radical_ty()),
        ("NakayamaLemma", nakayama_lemma_ty()),
        ("HopkinsLevitzki", hopkins_levitzki_ty()),
        ("GradedModule", graded_module_ty()),
        ("GradedModuleShift", graded_module_shift_ty()),
        ("FilteredModule", filtered_module_ty()),
        ("AssociatedGradedModule", associated_graded_module_ty()),
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
mod tests_ext {
    use super::*;
    fn ext_env() -> Environment {
        let mut env = Environment::new();
        register_abstract_algebra_advanced_ext(&mut env);
        env
    }
    #[test]
    fn test_tensor_exterior_clifford_registered() {
        let env = ext_env();
        assert!(env.get(&Name::str("TensorAlgebra")).is_some());
        assert!(env.get(&Name::str("SymmetricAlgebra")).is_some());
        assert!(env.get(&Name::str("ExteriorAlgebra")).is_some());
        assert!(env.get(&Name::str("WedgeProduct")).is_some());
        assert!(env.get(&Name::str("CliffordAlgebra")).is_some());
        assert!(env.get(&Name::str("SpinGroup")).is_some());
    }
    #[test]
    fn test_hopf_algebras_registered() {
        let env = ext_env();
        assert!(env.get(&Name::str("HopfAlgebra")).is_some());
        assert!(env.get(&Name::str("Comultiplication")).is_some());
        assert!(env.get(&Name::str("Counit")).is_some());
        assert!(env.get(&Name::str("Antipode")).is_some());
        assert!(env.get(&Name::str("HopfAntipodeAntiHom")).is_some());
    }
    #[test]
    fn test_cstar_von_neumann_registered() {
        let env = ext_env();
        assert!(env.get(&Name::str("CStarAlgebra")).is_some());
        assert!(env.get(&Name::str("Spectrum")).is_some());
        assert!(env.get(&Name::str("GelfandNaimark")).is_some());
        assert!(env.get(&Name::str("GNSConstruction")).is_some());
        assert!(env.get(&Name::str("VonNeumannAlgebra")).is_some());
        assert!(env.get(&Name::str("DoubleCommutant")).is_some());
    }
    #[test]
    fn test_uea_pbw_registered() {
        let env = ext_env();
        assert!(env.get(&Name::str("LieAlgebra")).is_some());
        assert!(env.get(&Name::str("UniversalEnvelopingAlgebra")).is_some());
        assert!(env.get(&Name::str("PBWTheorem")).is_some());
        assert!(env.get(&Name::str("UEAUniversalProperty")).is_some());
    }
    #[test]
    fn test_weyl_ore_registered() {
        let env = ext_env();
        assert!(env.get(&Name::str("WeylAlgebra")).is_some());
        assert!(env.get(&Name::str("WeylSimple")).is_some());
        assert!(env.get(&Name::str("OreExtension")).is_some());
        assert!(env.get(&Name::str("OreCondition")).is_some());
        assert!(env.get(&Name::str("SkewPolynomialRing")).is_some());
    }
    #[test]
    fn test_morita_registered() {
        let env = ext_env();
        assert!(env.get(&Name::str("MoritaEquivalent")).is_some());
        assert!(env.get(&Name::str("MoritaContext")).is_some());
        assert!(env.get(&Name::str("MatrixMoritaEquiv")).is_some());
        assert!(env.get(&Name::str("MoritaInvariant")).is_some());
    }
    #[test]
    fn test_group_algebras_registered() {
        let env = ext_env();
        assert!(env.get(&Name::str("GroupAlgebra")).is_some());
        assert!(env.get(&Name::str("GroupRepresentation")).is_some());
        assert!(env.get(&Name::str("MaschkeTheorem")).is_some());
        assert!(env.get(&Name::str("CharacterTheory")).is_some());
    }
    #[test]
    fn test_koszul_azumaya_registered() {
        let env = ext_env();
        assert!(env.get(&Name::str("KoszulAlgebra")).is_some());
        assert!(env.get(&Name::str("KoszulDual")).is_some());
        assert!(env.get(&Name::str("AzumayaAlgebra")).is_some());
        assert!(env.get(&Name::str("BrauerGroup")).is_some());
        assert!(env.get(&Name::str("CentralSimpleAlgebra")).is_some());
    }
    #[test]
    fn test_dga_operad_registered() {
        let env = ext_env();
        assert!(env.get(&Name::str("DGAlgebra")).is_some());
        assert!(env.get(&Name::str("AInfinityAlgebra")).is_some());
        assert!(env.get(&Name::str("Operad")).is_some());
        assert!(env.get(&Name::str("OperadAlgebra")).is_some());
        assert!(env.get(&Name::str("BarConstruction")).is_some());
        assert!(env.get(&Name::str("CobarConstruction")).is_some());
    }
    #[test]
    fn test_higher_category_registered() {
        let env = ext_env();
        assert!(env.get(&Name::str("EnrichedCategory")).is_some());
        assert!(env.get(&Name::str("InfinityCategory")).is_some());
        assert!(env.get(&Name::str("StableInfinityCategory")).is_some());
        assert!(env
            .get(&Name::str("SymmetricMonoidalInftyCategory"))
            .is_some());
        assert!(env.get(&Name::str("EInfinityAlgebra")).is_some());
    }
    #[test]
    fn test_symplectic_registered() {
        let env = ext_env();
        assert!(env.get(&Name::str("SymplecticForm")).is_some());
        assert!(env.get(&Name::str("SymplecticModule")).is_some());
        assert!(env.get(&Name::str("SymplecticGroup")).is_some());
        assert!(env.get(&Name::str("WittGroup")).is_some());
    }
    #[test]
    fn test_exterior_algebra_impl() {
        let ext = ExteriorAlgebra::new(3);
        assert_eq!(ext.total_dim(), 8);
        let desc = ext.describe();
        assert!(desc.contains("Λ(ℝ^3)"));
        let result = ExteriorAlgebra::wedge_basis(&[0], &[1]);
        assert!(result.is_some());
        let (sign, merged) = result.expect("result should be valid");
        assert_eq!(merged, vec![0, 1]);
        assert_eq!(sign, 1);
        let result2 = ExteriorAlgebra::wedge_basis(&[0, 1], &[1, 2]);
        assert!(result2.is_none());
    }
    #[test]
    fn test_clifford_algebra_impl() {
        let cl = CliffordAlgebra::new(vec![1.0, 1.0, 1.0]);
        assert_eq!(cl.dim(), 3);
        assert_eq!(cl.total_dim(), 8);
        let (p, q) = cl.signature();
        assert_eq!(p, 3);
        assert_eq!(q, 0);
        assert_eq!(cl.clifford_relation(0), 1.0);
        let cl2 = CliffordAlgebra::new(vec![1.0, -1.0]);
        let (p2, q2) = cl2.signature();
        assert_eq!(p2, 1);
        assert_eq!(q2, 1);
    }
    #[test]
    fn test_hopf_algebra_elem() {
        let g = HopfAlgebraElem::from_group("g");
        assert_eq!(g.antipode(), "g⁻¹");
        assert_eq!(g.coproduct(), "g ⊗ g");
        assert_eq!(g.counit(), 1);
        let h = HopfAlgebraElem::new("x");
        assert_eq!(h.antipode(), "S(x)");
    }
    #[test]
    fn test_graded_algebra_impl() {
        let ga = GradedAlgebraImpl::new(
            "ℕ",
            vec!["k".to_string(), "V".to_string(), "V⊗V/sym".to_string()],
        );
        assert!(ga.is_connected());
        assert_eq!(ga.top_degree(), Some(2));
        let ps = ga.poincare_series();
        assert!(ps.contains("t^0") || ps.contains("t^1") || ps.contains("t^2"));
    }
    #[test]
    fn test_koszul_complex() {
        let kc = KoszulComplex::new(vec!["x".to_string(), "y".to_string(), "z".to_string()], "R");
        assert_eq!(kc.length(), 3);
        assert_eq!(kc.num_terms(), 4);
        assert!(kc.is_acyclic_for_regular_sequence());
        assert_eq!(kc.euler_characteristic(), 0);
        let desc = kc.describe();
        assert!(desc.contains("Koszul complex"));
    }
}
#[cfg(test)]
mod tests_ext2 {
    use super::*;
    fn ext2_env() -> Environment {
        let mut env = Environment::new();
        register_abstract_algebra_advanced_ext2(&mut env);
        env
    }
    #[test]
    fn test_coalgebra_bialgebra_registered() {
        let env = ext2_env();
        assert!(env.get(&Name::str("Coalgebra")).is_some());
        assert!(env.get(&Name::str("CoalgebraMorphism")).is_some());
        assert!(env.get(&Name::str("Bialgebra")).is_some());
        assert!(env.get(&Name::str("CoalgebraComodule")).is_some());
        assert!(env.get(&Name::str("CofreeCoalgebra")).is_some());
    }
    #[test]
    fn test_frobenius_algebra_registered() {
        let env = ext2_env();
        assert!(env.get(&Name::str("FrobeniusAlgebra")).is_some());
        assert!(env.get(&Name::str("FrobeniusComultiplication")).is_some());
        assert!(env.get(&Name::str("CommutativeFrobeniusAlgebra")).is_some());
        assert!(env.get(&Name::str("FrobeniusHomomorphism")).is_some());
    }
    #[test]
    fn test_quantum_groups_registered() {
        let env = ext2_env();
        assert!(env.get(&Name::str("QuantumGroup")).is_some());
        assert!(env.get(&Name::str("RMatrix")).is_some());
        assert!(env.get(&Name::str("QuantumYangBaxterEquation")).is_some());
        assert!(env.get(&Name::str("DrinfeldDouble")).is_some());
        assert!(env.get(&Name::str("RibbonCategory")).is_some());
        assert!(env.get(&Name::str("QuasitriangularHopfAlgebra")).is_some());
    }
    #[test]
    fn test_hall_schur_registered() {
        let env = ext2_env();
        assert!(env.get(&Name::str("HallAlgebra")).is_some());
        assert!(env.get(&Name::str("HallPolynomial")).is_some());
        assert!(env.get(&Name::str("RingelHallTheorem")).is_some());
        assert!(env.get(&Name::str("SchurAlgebra")).is_some());
        assert!(env.get(&Name::str("SchurFunctor")).is_some());
        assert!(env.get(&Name::str("SchurWeylDuality")).is_some());
    }
    #[test]
    fn test_lie_superalgebra_registered() {
        let env = ext2_env();
        assert!(env.get(&Name::str("LieSuperalgebra")).is_some());
        assert!(env.get(&Name::str("LieSuperalgebraUEA")).is_some());
        assert!(env.get(&Name::str("SuperPBW")).is_some());
        assert!(env.get(&Name::str("LieSupermodule")).is_some());
    }
    #[test]
    fn test_deformation_quantization_registered() {
        let env = ext2_env();
        assert!(env.get(&Name::str("StarProduct")).is_some());
        assert!(env.get(&Name::str("KontsevichFormality")).is_some());
        assert!(env.get(&Name::str("DeformationQuantization")).is_some());
    }
    #[test]
    fn test_formal_group_laws_registered() {
        let env = ext2_env();
        assert!(env.get(&Name::str("FormalGroupLaw")).is_some());
        assert!(env.get(&Name::str("FormalGroupLawAdditive")).is_some());
        assert!(env
            .get(&Name::str("FormalGroupLawMultiplicative"))
            .is_some());
        assert!(env.get(&Name::str("FormalGroupLawHeight")).is_some());
        assert!(env.get(&Name::str("LazardRing")).is_some());
    }
    #[test]
    fn test_hecke_algebras_registered() {
        let env = ext2_env();
        assert!(env.get(&Name::str("HeckeAlgebra")).is_some());
        assert!(env.get(&Name::str("IwahoriHeckeAlgebra")).is_some());
        assert!(env.get(&Name::str("KazhdanLusztigPolynomial")).is_some());
    }
    #[test]
    fn test_group_cohomology_registered() {
        let env = ext2_env();
        assert!(env.get(&Name::str("GroupCohomology")).is_some());
        assert!(env.get(&Name::str("GroupHomology")).is_some());
        assert!(env.get(&Name::str("TateCohomology")).is_some());
        assert!(env.get(&Name::str("ShapiroLemma")).is_some());
    }
    #[test]
    fn test_iwasawa_registered() {
        let env = ext2_env();
        assert!(env.get(&Name::str("IwasawaAlgebra")).is_some());
        assert!(env.get(&Name::str("IwasawaModule")).is_some());
        assert!(env.get(&Name::str("CharacteristicIdeal")).is_some());
    }
    #[test]
    fn test_projective_injective_flat_registered() {
        let env = ext2_env();
        assert!(env.get(&Name::str("ProjectiveModule")).is_some());
        assert!(env.get(&Name::str("InjectiveModule")).is_some());
        assert!(env.get(&Name::str("FlatModule")).is_some());
        assert!(env.get(&Name::str("InjectiveEnvelope")).is_some());
    }
    #[test]
    fn test_artinian_semisimple_registered() {
        let env = ext2_env();
        assert!(env.get(&Name::str("ArtinianRing")).is_some());
        assert!(env.get(&Name::str("SemisimpleRing")).is_some());
        assert!(env.get(&Name::str("JacobsonRadical")).is_some());
        assert!(env.get(&Name::str("NakayamaLemma")).is_some());
        assert!(env.get(&Name::str("HopkinsLevitzki")).is_some());
    }
    #[test]
    fn test_graded_filtered_modules_registered() {
        let env = ext2_env();
        assert!(env.get(&Name::str("GradedModule")).is_some());
        assert!(env.get(&Name::str("GradedModuleShift")).is_some());
        assert!(env.get(&Name::str("FilteredModule")).is_some());
        assert!(env.get(&Name::str("AssociatedGradedModule")).is_some());
    }
    #[test]
    fn test_coalgebra_impl() {
        let coal = CoalgebraImpl::group_like(3);
        assert_eq!(coal.dim, 3);
        assert!(coal.is_coassociative_group_like());
        assert_eq!(coal.counit_val(0), 1.0);
        assert_eq!(coal.counit_val(2), 1.0);
        assert_eq!(coal.comult[1], vec![(1, 1, 1.0)]);
    }
    #[test]
    fn test_frobenius_algebra_impl() {
        let frob = FrobeniusAlgebraImpl::trivial();
        assert_eq!(frob.dim, 1);
        assert!(frob.is_nondegenerate());
        let desc = frob.describe();
        assert!(desc.contains("Frobenius algebra"));
        assert!(desc.contains("nondegenerate form: yes"));
    }
    #[test]
    fn test_formal_group_law_additive() {
        let fgl = FormalGroupLawImpl::additive(3);
        assert!((fgl.eval(2.0, 3.0) - 5.0).abs() < 1e-10);
        assert!(fgl.check_unit_axioms());
        let desc = fgl.describe();
        assert!(desc.contains("unit axioms hold: yes"));
    }
    #[test]
    fn test_formal_group_law_multiplicative() {
        let fgl = FormalGroupLawImpl::multiplicative(3);
        assert!((fgl.eval(1.0, 1.0) - 3.0).abs() < 1e-10);
        assert!(fgl.check_unit_axioms());
    }
    #[test]
    fn test_graded_module_impl() {
        let gm = GradedModuleImpl::new(
            "k[x]",
            vec!["k".to_string(), "k".to_string(), "k".to_string()],
        );
        assert_eq!(gm.top_degree(), Some(2));
        assert_eq!(gm.hilbert_function(2), 3);
        let shifted = gm.shift(1);
        assert_eq!(shifted.components.len(), 2);
        let desc = gm.describe();
        assert!(desc.contains("Graded module over k[x]"));
    }
    #[test]
    fn test_hecke_algebra_elem() {
        let e = HeckeAlgebraElem::identity(2.0);
        assert_eq!(e.terms.len(), 1);
        let t1 = HeckeAlgebraElem::generator(2.0, 0);
        let (a, b) = t1.quadratic_relation_lhs_coeff();
        assert!((a - 1.0).abs() < 1e-10);
        assert!((b - 2.0).abs() < 1e-10);
        let desc = t1.describe();
        assert!(desc.contains("Hecke algebra element"));
    }
}
/// `HopfComodule : ∀ (H M : Type), HopfAlgebra H → Prop`
///
/// A right H-comodule M: a vector space M with a coaction ρ: M → M ⊗ H
/// satisfying (id ⊗ Δ) ∘ ρ = (ρ ⊗ id) ∘ ρ and (id ⊗ ε) ∘ ρ = id.
pub fn hopf_comodule_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "H",
        type0(),
        pi(
            BinderInfo::Default,
            "M",
            type0(),
            arrow(app(cst("HopfAlgebra"), bvar(1)), prop()),
        ),
    )
}
/// `HopfModule : ∀ (H M : Type), HopfAlgebra H → Prop`
///
/// A (left) H-module algebra M: M is both an H-module and an algebra,
/// and the H-action is by algebra automorphisms.
pub fn hopf_module_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "H",
        type0(),
        pi(
            BinderInfo::Default,
            "M",
            type0(),
            arrow(app(cst("HopfAlgebra"), bvar(1)), prop()),
        ),
    )
}
/// `BialgebraCoassociativity : ∀ (B : Type), Bialgebra B → Prop`
///
/// The coassociativity axiom for a bialgebra:
/// (Δ ⊗ id) ∘ Δ = (id ⊗ Δ) ∘ Δ.
pub fn bialgebra_coassociativity_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "B",
        type0(),
        arrow(app(cst("Bialgebra"), bvar(0)), prop()),
    )
}
/// `HopfAntipodeAxiom : ∀ (H : Type), HopfAlgebra H → Prop`
///
/// The defining axiom of the antipode: m ∘ (S ⊗ id) ∘ Δ = η ∘ ε.
/// Equivalently: Σ S(h₁)h₂ = ε(h)·1 for all h ∈ H.
pub fn hopf_antipode_axiom_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "H",
        type0(),
        arrow(app(cst("HopfAlgebra"), bvar(0)), prop()),
    )
}
/// `SmashProduct : ∀ (H A : Type), HopfAlgebra H → HopfModule H A → Type`
///
/// The smash product A # H: as a vector space A ⊗ H,
/// with multiplication (a # h)(b # g) = Σ a·(h₁·b) # h₂·g.
pub fn smash_product_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "H",
        type0(),
        pi(
            BinderInfo::Default,
            "A",
            type0(),
            arrow(
                app(cst("HopfAlgebra"), bvar(1)),
                arrow(app2(cst("HopfModule"), bvar(2), bvar(1)), type0()),
            ),
        ),
    )
}
/// `TakeuchiEquivalence : ∀ (H : Type), HopfAlgebra H → Prop`
///
/// Takeuchi's equivalence: for a Hopf algebra H with bijective antipode,
/// the categories of H-modules and H-comodules satisfy a duality.
pub fn takeuchi_equivalence_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "H",
        type0(),
        arrow(app(cst("HopfAlgebra"), bvar(0)), prop()),
    )
}
/// `TiltingModule : ∀ (R : Type), IsRing R → Type → Prop`
///
/// A tilting module T over a ring R: T has finite projective dimension,
/// Ext^i_R(T,T) = 0 for i > 0, and T generates the derived category D^b(R).
pub fn tilting_module_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "R",
        type0(),
        pi(
            BinderInfo::Default,
            "T",
            type0(),
            arrow(app(cst("IsRing"), bvar(1)), prop()),
        ),
    )
}
/// `DerivedMoritaEquivalence : Type → Type → Prop`
///
/// Two rings R and S are derived Morita equivalent if their derived categories
/// D^b(Mod-R) and D^b(Mod-S) are equivalent as triangulated categories.
pub fn derived_morita_equivalence_ty() -> Expr {
    arrow(type0(), arrow(type0(), prop()))
}
/// `TiltingEquivalence : ∀ (R S : Type) (T : Type),
///     TiltingModule R T → S = End_R(T) → DerivedMoritaEquivalent R S`
///
/// Rickard's theorem: if T is a tilting R-module and S = End_R(T),
/// then R and S are derived Morita equivalent.
pub fn tilting_equivalence_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "R",
        type0(),
        pi(
            BinderInfo::Default,
            "S",
            type0(),
            pi(
                BinderInfo::Default,
                "T",
                type0(),
                arrow(
                    app2(cst("TiltingModule"), bvar(2), bvar(1)),
                    arrow(
                        app2(
                            cst("RingIso"),
                            bvar(2),
                            app2(cst("EndRing"), bvar(3), bvar(1)),
                        ),
                        app2(cst("DerivedMoritaEquivalence"), bvar(4), bvar(3)),
                    ),
                ),
            ),
        ),
    )
}
/// `SilentingModule : ∀ (R : Type), IsRing R → Type → Prop`
///
/// A silting module: a generalisation of tilting modules allowing
/// Ext^i_R(T,T) = 0 for i > 0 without requiring generation of D^b(R).
pub fn silting_module_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "R",
        type0(),
        pi(
            BinderInfo::Default,
            "T",
            type0(),
            arrow(app(cst("IsRing"), bvar(1)), prop()),
        ),
    )
}
/// `APR_Tilting : ∀ (A : Type), IsAlgebra A → Type → Prop`
///
/// Auslander-Platzeck-Reiten tilting: for an indecomposable non-projective
/// A-module M, the APR tilting module is T = τ⁻¹M ⊕ P where P is projective.
pub fn apr_tilting_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "A",
        type0(),
        pi(
            BinderInfo::Default,
            "T",
            type0(),
            arrow(app(cst("IsAlgebra"), bvar(1)), prop()),
        ),
    )
}
/// `GradedRing : Type → Prop`
///
/// A graded ring: R = ⊕_{n∈ℕ} R_n with R_i · R_j ⊆ R_{i+j}.
pub fn graded_ring_ty() -> Expr {
    arrow(type0(), prop())
}
/// `HilbertSeries : ∀ (R : Type), GradedRing R → Type`
///
/// The Hilbert series of a graded ring R: the formal power series
/// H_R(t) = Σ_{n≥0} dim_k(R_n) · t^n.
pub fn hilbert_series_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "R",
        type0(),
        arrow(app(cst("GradedRing"), bvar(0)), type0()),
    )
}
/// `HilbertSyzygy : ∀ (R M : Type), GradedRing R → GradedModule R M → Prop`
///
/// Hilbert's syzygy theorem: every finitely generated module over k\[x₁,…,xₙ\]
/// has a finite free resolution of length at most n.
pub fn hilbert_syzygy_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "R",
        type0(),
        pi(
            BinderInfo::Default,
            "M",
            type0(),
            arrow(
                app(cst("GradedRing"), bvar(1)),
                arrow(app2(cst("GradedModule"), bvar(2), bvar(1)), prop()),
            ),
        ),
    )
}
/// `RegularSequenceGraded : ∀ (R : Type), GradedRing R → List R → Prop`
///
/// A homogeneous regular sequence (f₁,…,fₙ) in a graded ring:
/// each fᵢ is a non-zero-divisor on R/(f₁,…,f_{i-1}).
pub fn regular_sequence_graded_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "R",
        type0(),
        arrow(
            app(cst("GradedRing"), bvar(0)),
            arrow(app(cst("List"), bvar(1)), prop()),
        ),
    )
}
/// `GKDimension : ∀ (A : Type), IsAlgebra A → Nat`
///
/// Gelfand-Kirillov dimension: GKdim(A) = lim sup_{n→∞} log dim(V^n)/log n
/// where V is any finite-dimensional generating subspace. Measures growth rate.
pub fn gk_dimension_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "A",
        type0(),
        arrow(app(cst("IsAlgebra"), bvar(0)), nat_ty()),
    )
}
/// `ProjectiveDimension : ∀ (R M : Type), IsModule R M → Nat`
///
/// The projective dimension pd_R(M): the minimum length of a projective
/// resolution of M. pd(M) = 0 iff M is projective.
pub fn projective_dimension_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "R",
        type0(),
        pi(
            BinderInfo::Default,
            "M",
            type0(),
            arrow(app2(cst("IsModule"), bvar(1), bvar(0)), nat_ty()),
        ),
    )
}
/// `InjectiveDimension : ∀ (R M : Type), IsModule R M → Nat`
///
/// The injective dimension id_R(M): the minimum length of an injective
/// resolution of M. id(M) = 0 iff M is injective.
pub fn injective_dimension_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "R",
        type0(),
        pi(
            BinderInfo::Default,
            "M",
            type0(),
            arrow(app2(cst("IsModule"), bvar(1), bvar(0)), nat_ty()),
        ),
    )
}
/// `GlobalDimension : Type → Nat`
///
/// The global dimension gl.dim(R) = sup_{M} pd_R(M).
/// For Noetherian regular local rings, gl.dim(R) = Krull dim(R).
pub fn global_dimension_ty() -> Expr {
    arrow(type0(), nat_ty())
}
/// `WeakDimension : Type → Nat`
///
/// The weak (flat) dimension w.dim(R) = sup_M fd_R(M) where fd is flat dimension.
/// w.dim(R) ≤ gl.dim(R); equality holds for Noetherian rings.
pub fn weak_dimension_ty() -> Expr {
    arrow(type0(), nat_ty())
}
/// `AuslanderBuchsbaumFormula : ∀ (R M : Type), RegularLocalRing R → IsModule R M →
///     pd_R(M) + depth_R(M) = depth(R)`
///
/// The Auslander-Buchsbaum formula: pd(M) + depth(M) = depth(R) for finitely
/// generated modules M over a regular local ring R with pd(M) < ∞.
pub fn auslander_buchsbaum_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "R",
        type0(),
        pi(
            BinderInfo::Default,
            "M",
            type0(),
            arrow(
                app(cst("RegularLocalRing"), bvar(1)),
                arrow(app2(cst("IsModule"), bvar(2), bvar(1)), prop()),
            ),
        ),
    )
}
/// `SerreRegularity : ∀ (R : Type), LocalRing R →
///     RegularLocalRing R ↔ gl.dim(R) < ∞`
///
/// Serre's theorem: a local ring is regular iff it has finite global dimension.
pub fn serre_regularity_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "R",
        type0(),
        arrow(app(cst("LocalRing"), bvar(0)), prop()),
    )
}
/// `PrimeSpectrum : Type → Type`
///
/// Spec(R): the set of prime ideals of R, equipped with the Zariski topology.
/// Points are prime ideals, closed sets are V(I) = { p | I ⊆ p }.
pub fn prime_spectrum_ty() -> Expr {
    arrow(type0(), type0())
}
/// `NoetherianRingAxiom : Type → Prop`
///
/// Axiom: R is Noetherian (every ideal is finitely generated,
/// equivalently the ascending chain condition holds for ideals).
pub fn noetherian_ring_axiom_ty() -> Expr {
    arrow(type0(), prop())
}
/// `ArtinianRingAxiom : Type → Prop`
///
/// Axiom: R is Artinian (descending chain condition on ideals).
/// Artinian ⟺ Noetherian + Krull dimension 0.
pub fn artinian_ring_axiom_ty() -> Expr {
    arrow(type0(), prop())
}
/// `PrimaryDecomposition : ∀ (R I : Type), NoetherianRing R → Ideal R I → Prop`
///
/// Lasker-Noether theorem: every ideal in a Noetherian ring decomposes as
/// a finite intersection of primary ideals I = q₁ ∩ ⋯ ∩ qₙ.
pub fn primary_decomposition_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "R",
        type0(),
        pi(
            BinderInfo::Default,
            "I",
            type0(),
            arrow(
                app(cst("NoetherianRing"), bvar(1)),
                arrow(app2(cst("Ideal"), bvar(2), bvar(1)), prop()),
            ),
        ),
    )
}
/// `KrullDimension : Type → Nat`
///
/// The Krull dimension of a ring R: the supremum of lengths of chains
/// p₀ ⊊ p₁ ⊊ … ⊊ pₙ of prime ideals. dim(k\[x₁,…,xₙ\]) = n.
pub fn krull_dimension_ty() -> Expr {
    arrow(type0(), nat_ty())
}
/// `LocalRing : Type → Prop`
///
/// A local ring: a ring R with a unique maximal ideal m_R.
/// Nakayama's lemma applies and residue field is R/m_R.
pub fn local_ring_ty() -> Expr {
    arrow(type0(), prop())
}
/// `RegularLocalRing : Type → Prop`
///
/// A regular local ring: a Noetherian local ring (R, m) such that
/// dim_k(m/m²) = Krull dim(R) (minimal generators of m equal Krull dim).
pub fn regular_local_ring_ty() -> Expr {
    arrow(type0(), prop())
}
/// `CohenMacaulay : Type → Prop`
///
/// A Cohen-Macaulay ring: a Noetherian ring where depth equals Krull dimension
/// (locally at every prime). Includes regular rings, complete intersections.
pub fn cohen_macaulay_ty() -> Expr {
    arrow(type0(), prop())
}
/// `SolvableExtension : ∀ (K L : Type), FieldExtension K L → Prop`
///
/// L/K is a solvable extension if Gal(L/K) is a solvable group.
/// Abel-Ruffini: degree ≥ 5 polynomials with non-solvable Galois group
/// cannot be solved by radicals.
pub fn solvable_extension_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "K",
        type0(),
        pi(
            BinderInfo::Default,
            "L",
            type0(),
            arrow(app2(cst("FieldExtension"), bvar(1), bvar(0)), prop()),
        ),
    )
}
/// `RadicalExtension : ∀ (K L : Type), FieldExtension K L → Prop`
///
/// L/K is a radical extension if L can be obtained from K by successively
/// adjoining n-th roots: L = K(α₁, …, αₘ) where each αᵢⁿⁱ ∈ K(α₁,…,α_{i-1}).
pub fn radical_extension_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "K",
        type0(),
        pi(
            BinderInfo::Default,
            "L",
            type0(),
            arrow(app2(cst("FieldExtension"), bvar(1), bvar(0)), prop()),
        ),
    )
}
/// `AbelRuffini : ∀ (K : Type) (p : Polynomial K), Prop`
///
/// Abel-Ruffini theorem: there is no general algebraic formula (using radicals)
/// for roots of polynomials of degree ≥ 5 over ℚ.
pub fn abel_ruffini_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "K",
        type0(),
        arrow(app(cst("Polynomial"), bvar(0)), prop()),
    )
}
/// `InfiniteGaloisGroup : ∀ (K : Type), IsField K → Type`
///
/// The absolute Galois group Gal(K̄/K) of a field K: a profinite group
/// = lim← Gal(L/K) over finite Galois extensions L/K.
pub fn infinite_galois_group_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "K",
        type0(),
        arrow(app(cst("IsField"), bvar(0)), type0()),
    )
}
/// `InverseGaloisProblem : ∀ (G : Type), IsFiniteGroup G → Prop`
///
/// The inverse Galois problem: does every finite group G occur as
/// Gal(L/ℚ) for some Galois extension L/ℚ? Open in general.
pub fn inverse_galois_problem_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "G",
        type0(),
        arrow(app(cst("IsFiniteGroup"), bvar(0)), prop()),
    )
}
/// `WedderburnLittleTheorem : ∀ (D : Type), IsFiniteDivisionRing D → IsField D`
///
/// Wedderburn's little theorem: every finite division ring is a field.
pub fn wedderburn_little_theorem_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "D",
        type0(),
        arrow(
            app(cst("IsFiniteDivisionRing"), bvar(0)),
            app(cst("IsField"), bvar(1)),
        ),
    )
}
/// `ArtinWedderburn : ∀ (R : Type), SemisimpleRing R → Prop`
///
/// Artin-Wedderburn theorem: every semisimple ring is isomorphic to
/// a finite direct product of matrix rings M_{n_i}(D_i) over division rings D_i.
pub fn artin_wedderburn_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "R",
        type0(),
        arrow(app(cst("SemisimpleRing"), bvar(0)), prop()),
    )
}
/// `BrauerEquivalence : ∀ (A B : Type), CentralSimpleAlgebra A →
///     CentralSimpleAlgebra B → Prop`
///
/// Two central simple algebras A and B are Brauer equivalent if
/// A ≅ M_n(D) and B ≅ M_m(D) for the same division algebra D.
pub fn brauer_equivalence_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "A",
        type0(),
        pi(
            BinderInfo::Default,
            "B",
            type0(),
            arrow(
                app(cst("CentralSimpleAlgebra"), bvar(1)),
                arrow(app(cst("CentralSimpleAlgebra"), bvar(1)), prop()),
            ),
        ),
    )
}
/// `BrauerGroupTsen : ∀ (k : Type), IsAlgebraicallyClosed k → Prop`
///
/// Tsen's theorem: Br(k) = 0 for any algebraically closed field k.
pub fn brauer_group_tsen_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "k",
        type0(),
        arrow(app(cst("IsAlgebraicallyClosed"), bvar(0)), prop()),
    )
}
/// `AlbertTheorem : ∀ (D : Type), IsDivisionAlgebra D → IsField (Center D) → Prop`
///
/// Albert's theorem: a central division algebra of degree 4 over a
/// number field is a cyclic algebra. (Part of the Albert-Brauer-Hasse-Noether theorem.)
pub fn albert_theorem_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "D",
        type0(),
        arrow(
            app(cst("IsDivisionAlgebra"), bvar(0)),
            arrow(app(cst("IsField"), app(cst("Center"), bvar(1))), prop()),
        ),
    )
}
/// `WittVectors : Type → Type`
///
/// The ring W(R) of big Witt vectors over R: a functorial construction
/// W: CRing → CRing satisfying W(R)/(Vₙ) ≅ Wₙ(R) (truncated Witt vectors).
pub fn witt_vectors_ty() -> Expr {
    arrow(type0(), type0())
}
/// `WittVectorsP : ∀ (p : Nat), Type → Type`
///
/// The ring W_p(R) of p-typical Witt vectors: relevant to p-adic Hodge theory.
/// W_p(𝔽_p) ≅ ℤ_p.
pub fn witt_vectors_p_ty() -> Expr {
    arrow(nat_ty(), arrow(type0(), type0()))
}
/// `LambdaRing : Type → Prop`
///
/// A λ-ring: a commutative ring R with operations λ^n: R → R for all n ≥ 0
/// satisfying the axioms of the Adams operations and Grothendieck's λ-ring structure.
pub fn lambda_ring_ty() -> Expr {
    arrow(type0(), prop())
}
/// `AdamsOperation : ∀ (R : Type) (n : Nat), LambdaRing R → R → R`
///
/// The Adams operation ψ^n: R → R in a λ-ring:
/// determined by λ-operations via Newton's identity ψ^n - λ^1 ψ^{n-1} + … = (-1)^{n-1} n λ^n.
pub fn adams_operation_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "R",
        type0(),
        pi(
            BinderInfo::Default,
            "n",
            nat_ty(),
            arrow(app(cst("LambdaRing"), bvar(1)), arrow(bvar(2), bvar(3))),
        ),
    )
}
/// `GrothendieckGroup : Type → Type`
///
/// The Grothendieck group K₀(C) of an exact category C: the abelian group
/// freely generated by isomorphism classes \[M\] with \[M\] = [M'] + [M'']
/// for every short exact sequence 0 → M' → M → M'' → 0.
pub fn grothendieck_group_ty() -> Expr {
    arrow(type0(), type0())
}
