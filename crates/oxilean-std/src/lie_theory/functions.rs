//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{
    CartanSubalgebra, CartanSubalgebraData, DynkinDiagram, ExponentialMap, HeckeAlgebra,
    InvariantTheory, KillingForm, LeviDecomposition, LieAlgebra, LieAlgebraElement, LieBracket,
    LieGroup, LieGroupElement, LieGroupHom, LieRepresentation, NilpotentOrbit, NilpotentOrbitData,
    QuantumGroup, Root, RootSystem, SolvableLieAlgebra, StructureConstants, VermaModule,
    WeightLattice, WeylCharacterFormula,
};

pub fn app(f: Expr, a: Expr) -> Expr {
    Expr::App(Box::new(f), Box::new(a))
}
pub fn app2(f: Expr, a: Expr, b: Expr) -> Expr {
    app(app(f, a), b)
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
pub fn nat_ty() -> Expr {
    cst("Nat")
}
pub fn real_ty() -> Expr {
    cst("Real")
}
pub fn bool_ty() -> Expr {
    cst("Bool")
}
pub fn int_ty() -> Expr {
    cst("Int")
}
/// Lie algebra type: a vector space with Lie bracket
pub fn lie_algebra_ty() -> Expr {
    type0()
}
/// Lie bracket type: [X, Y] — antisymmetric bilinear map
pub fn lie_bracket_ty() -> Expr {
    arrow(type0(), arrow(type0(), type0()))
}
/// Lie algebra element type
pub fn lie_algebra_element_ty() -> Expr {
    arrow(type0(), type0())
}
/// Cartan subalgebra type: maximal abelian self-normalizing subalgebra
pub fn cartan_subalgebra_ty() -> Expr {
    arrow(type0(), type0())
}
/// Structure constants type: f^k_{ij} for [e_i, e_j] = f^k_{ij} e_k
pub fn structure_constants_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// Lie group type: smooth manifold with group structure
pub fn lie_group_ty() -> Expr {
    type0()
}
/// Lie group element type
pub fn lie_group_element_ty() -> Expr {
    arrow(type0(), type0())
}
/// Exponential map type: exp : g → G
pub fn exp_map_ty() -> Expr {
    arrow(type0(), type0())
}
/// Adjoint representation type: Ad : G → GL(g)
pub fn adjoint_rep_ty() -> Expr {
    arrow(type0(), arrow(type0(), type0()))
}
/// Root system type
pub fn root_system_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// Root type: element of the dual of a Cartan subalgebra
pub fn root_ty() -> Expr {
    arrow(type0(), type0())
}
/// Dynkin diagram type
pub fn dynkin_diagram_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// Weight lattice type
pub fn weight_lattice_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// Lie algebra representation type
pub fn lie_representation_ty() -> Expr {
    arrow(type0(), arrow(nat_ty(), type0()))
}
/// Weyl character formula type
pub fn weyl_character_formula_ty() -> Expr {
    arrow(type0(), type0())
}
/// Killing form type: symmetric bilinear form K(X,Y) = Tr(ad X ∘ ad Y)
pub fn killing_form_ty() -> Expr {
    arrow(type0(), type0())
}
/// Jacobi identity: [X,[Y,Z]] + [Y,[Z,X]] + [Z,[X,Y]] = 0
pub fn jacobi_identity_ty() -> Expr {
    let g = type0();
    arrow(
        g,
        arrow(real_ty(), arrow(real_ty(), arrow(real_ty(), prop()))),
    )
}
/// Cartan criterion: g semisimple ↔ Killing form is nondegenerate
pub fn cartan_criterion_ty() -> Expr {
    arrow(type0(), prop())
}
/// Ado's theorem: every finite-dimensional Lie algebra has a faithful linear representation
pub fn ado_theorem_ty() -> Expr {
    arrow(type0(), arrow(nat_ty(), prop()))
}
/// Lie's theorem: solvable Lie algebras over ℂ have upper-triangular representations
pub fn lie_theorem_ty() -> Expr {
    arrow(type0(), prop())
}
/// Engel's theorem: nilpotent iff every element is ad-nilpotent
pub fn engel_theorem_ty() -> Expr {
    arrow(type0(), prop())
}
/// Weyl's theorem: representations of semisimple Lie algebras are completely reducible
pub fn weyl_theorem_ty() -> Expr {
    arrow(type0(), arrow(type0(), prop()))
}
/// Weyl character formula: character of irreducible representation
pub fn weyl_character_formula_ty_theorem() -> Expr {
    arrow(type0(), arrow(type0(), real_ty()))
}
/// Baker-Campbell-Hausdorff formula: log(exp(X)exp(Y)) = X + Y + [X,Y]/2 + ...
pub fn bch_formula_ty() -> Expr {
    arrow(type0(), arrow(type0(), type0()))
}
/// Killing form signature for real semisimple Lie algebras (Sylvester's law)
pub fn killing_form_signature_ty() -> Expr {
    arrow(type0(), arrow(int_ty(), prop()))
}
/// Complete reducibility (Weyl): every representation of a semisimple Lie algebra
/// over char 0 is completely reducible.
pub fn complete_reducibility_ty() -> Expr {
    arrow(type0(), arrow(type0(), prop()))
}
/// Character of a representation: ch : Rep(g) → Z[P]
pub fn character_ty() -> Expr {
    arrow(type0(), arrow(type0(), type0()))
}
/// Schur's lemma: Hom_g(V, W) = 0 if V ≇ W irreducible, = ℂ if V ≅ W.
pub fn schur_lemma_ty() -> Expr {
    arrow(type0(), arrow(type0(), prop()))
}
/// Verma module M(λ) associated to a weight λ.
pub fn verma_module_ty() -> Expr {
    arrow(type0(), type0())
}
/// BGG category O: full subcategory of g-modules with weight decomposition.
pub fn bgg_category_o_ty() -> Expr {
    arrow(type0(), type0())
}
/// BGG resolution: projective resolution of L(λ) by Verma modules.
pub fn bgg_resolution_ty() -> Expr {
    arrow(type0(), arrow(nat_ty(), type0()))
}
/// Kazhdan-Lusztig polynomial P_{x,y}(q) ∈ ℤ[q] for x ≤ y in Weyl group W.
pub fn kl_polynomial_ty() -> Expr {
    arrow(type0(), arrow(type0(), arrow(nat_ty(), int_ty())))
}
/// Canonical basis (KL basis) of the Hecke algebra H(W, S).
pub fn canonical_basis_ty() -> Expr {
    arrow(type0(), type0())
}
/// KL conjecture (Beilinson-Bernstein-Deligne-Gabber theorem):
/// multiplicities of composition factors equal KL polynomial values.
pub fn kl_conjecture_ty() -> Expr {
    arrow(type0(), arrow(type0(), prop()))
}
/// Quantum group U_q(g): q-deformation of the universal enveloping algebra.
pub fn quantum_group_ty() -> Expr {
    arrow(real_ty(), arrow(type0(), type0()))
}
/// Quantum universal enveloping algebra U_q(g).
pub fn quantum_uea_ty() -> Expr {
    arrow(real_ty(), arrow(type0(), type0()))
}
/// R-matrix: universal R-matrix for a quasi-triangular Hopf algebra.
pub fn r_matrix_ty() -> Expr {
    arrow(type0(), arrow(type0(), arrow(type0(), type0())))
}
/// Hopf algebra: algebra with comultiplication, counit, and antipode.
pub fn hopf_algebra_ty() -> Expr {
    arrow(type0(), type0())
}
/// Drinfeld-Jimbo quantum group: quantization of a semisimple Lie algebra.
pub fn drinfeld_jimbo_ty() -> Expr {
    arrow(real_ty(), arrow(type0(), type0()))
}
/// Affine Lie algebra g_hat: central extension of the loop algebra Lg.
pub fn affine_lie_algebra_ty() -> Expr {
    arrow(type0(), type0())
}
/// Kac-Moody algebra associated to a generalized Cartan matrix.
pub fn kac_moody_algebra_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// Generalized Cartan matrix: integer matrix satisfying Kac-Moody axioms.
pub fn generalized_cartan_matrix_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), int_ty()))
}
/// Level of a representation of an affine Lie algebra.
pub fn affine_level_ty() -> Expr {
    arrow(type0(), int_ty())
}
/// Integrable highest weight module for an affine Lie algebra.
pub fn integrable_hwm_ty() -> Expr {
    arrow(type0(), arrow(nat_ty(), type0()))
}
/// Virasoro algebra: central extension of the Witt algebra (derivations of ℂ[t,t⁻¹]).
pub fn virasoro_algebra_ty() -> Expr {
    type0()
}
/// Central charge c of the Virasoro algebra.
pub fn central_charge_ty() -> Expr {
    arrow(type0(), real_ty())
}
/// Highest weight representation of the Virasoro algebra L(c, h).
pub fn virasoro_hw_module_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), type0()))
}
/// D-module on an algebraic variety X.
pub fn d_module_ty() -> Expr {
    arrow(type0(), type0())
}
/// Perverse sheaf: middle perversity intersection complex.
pub fn perverse_sheaf_ty() -> Expr {
    arrow(type0(), type0())
}
/// Beilinson-Bernstein localization: D-modules on flag variety ↔ g-modules.
pub fn bb_localization_ty() -> Expr {
    arrow(type0(), prop())
}
/// Intersection cohomology IH*(X, L) of a variety.
pub fn intersection_cohomology_ty() -> Expr {
    arrow(type0(), arrow(nat_ty(), type0()))
}
/// Lie superalgebra: Z/2-graded vector space with super-Lie bracket.
pub fn lie_superalgebra_ty() -> Expr {
    arrow(type0(), type0())
}
/// Super-Lie bracket: graded bilinear map respecting parity.
pub fn super_lie_bracket_ty() -> Expr {
    arrow(type0(), arrow(type0(), arrow(type0(), type0())))
}
/// Representation of a Lie superalgebra.
pub fn lie_super_rep_ty() -> Expr {
    arrow(type0(), arrow(type0(), type0()))
}
/// Iwahori-Hecke algebra H(W, q) with parameter q.
pub fn hecke_algebra_ty() -> Expr {
    arrow(real_ty(), arrow(type0(), type0()))
}
/// Hecke algebra module (representation of Hecke algebra).
pub fn hecke_module_ty() -> Expr {
    arrow(type0(), type0())
}
/// Kashiwara crystal basis B(∞) for the quantum group U_q(g).
pub fn crystal_basis_ty() -> Expr {
    arrow(type0(), type0())
}
/// Crystal graph: colored directed graph encoding crystal basis operators.
pub fn crystal_graph_ty() -> Expr {
    arrow(type0(), type0())
}
/// Crystal operators ẽ_i, f̃_i on a crystal basis.
pub fn crystal_operators_ty() -> Expr {
    arrow(nat_ty(), arrow(type0(), type0()))
}
/// Geometric Satake correspondence: Sat(G) ≅ Rep(G∨).
pub fn geometric_satake_ty() -> Expr {
    arrow(type0(), prop())
}
/// Langlands dual group G∨ associated to a reductive group G.
pub fn langlands_dual_ty() -> Expr {
    arrow(type0(), type0())
}
/// Langlands duality for Lie algebras: g and g∨ have dual root data.
pub fn langlands_duality_ty() -> Expr {
    arrow(type0(), prop())
}
/// Nilpotent orbit: G-orbit in N(g) (nilpotent cone of g).
pub fn nilpotent_orbit_ty() -> Expr {
    arrow(type0(), type0())
}
/// Springer resolution: T*(G/B) → N(g), cotangent bundle of flag variety.
pub fn springer_resolution_ty() -> Expr {
    arrow(type0(), type0())
}
/// Springer correspondence: bijection nilpotent orbits ↔ pairs (σ, local system).
pub fn springer_correspondence_ty() -> Expr {
    arrow(type0(), prop())
}
/// Slodowy slice: transversal slice to a nilpotent orbit.
pub fn slodowy_slice_ty() -> Expr {
    arrow(type0(), arrow(type0(), type0()))
}
/// Kirillov's orbit method: bijection between coadjoint orbits and unitary reps.
pub fn kirillov_orbit_method_ty() -> Expr {
    arrow(type0(), prop())
}
/// Coadjoint orbit O_f = G·f ⊂ g* for f ∈ g*.
pub fn coadjoint_orbit_ty() -> Expr {
    arrow(type0(), type0())
}
/// Kirillov-Kostant-Souriau symplectic form on a coadjoint orbit.
pub fn kks_symplectic_form_ty() -> Expr {
    arrow(type0(), type0())
}
/// Loop group LG = Map(S¹, G): infinite-dimensional Lie group.
pub fn loop_group_ty() -> Expr {
    arrow(type0(), type0())
}
/// Central extension of a loop group: LG_hat → LG → 1.
pub fn central_extension_ty() -> Expr {
    arrow(type0(), type0())
}
/// 2-cocycle defining a central extension.
pub fn two_cocycle_ty() -> Expr {
    arrow(type0(), arrow(type0(), arrow(type0(), type0())))
}
/// Algebraic group: group variety over an algebraically closed field.
pub fn algebraic_group_ty() -> Expr {
    type0()
}
/// Borel subgroup B ⊂ G: maximal connected solvable subgroup.
pub fn borel_subgroup_ty() -> Expr {
    arrow(type0(), type0())
}
/// Flag variety G/B: homogeneous space for a reductive group G.
pub fn flag_variety_ty() -> Expr {
    arrow(type0(), type0())
}
/// Schubert variety: closure of Bruhat cell B·wB/B ⊂ G/B.
pub fn schubert_variety_ty() -> Expr {
    arrow(type0(), arrow(type0(), type0()))
}
/// Rational representation of an algebraic group.
pub fn algebraic_group_rep_ty() -> Expr {
    arrow(type0(), arrow(nat_ty(), type0()))
}
/// Register all Lie theory axioms into the environment.
pub fn build_lie_theory_env(env: &mut Environment) {
    let axioms: &[(&str, Expr)] = &[
        ("LieAlgebra", lie_algebra_ty()),
        ("LieBracket", lie_bracket_ty()),
        ("LieAlgebraElement", lie_algebra_element_ty()),
        ("CartanSubalgebra", cartan_subalgebra_ty()),
        ("StructureConstants", structure_constants_ty()),
        ("LieGroup", lie_group_ty()),
        ("LieGroupElement", lie_group_element_ty()),
        ("ExponentialMap", exp_map_ty()),
        ("AdjointRep", adjoint_rep_ty()),
        ("RootSystem", root_system_ty()),
        ("Root", root_ty()),
        ("DynkinDiagram", dynkin_diagram_ty()),
        ("WeightLattice", weight_lattice_ty()),
        ("LieRepresentation", lie_representation_ty()),
        ("WeylCharacterFormula", weyl_character_formula_ty()),
        ("KillingForm", killing_form_ty()),
        ("AlgebraAn", arrow(nat_ty(), type0())),
        ("AlgebraBn", arrow(nat_ty(), type0())),
        ("AlgebraCn", arrow(nat_ty(), type0())),
        ("AlgebraDn", arrow(nat_ty(), type0())),
        ("AlgebraG2", type0()),
        ("AlgebraF4", type0()),
        ("AlgebraE6", type0()),
        ("AlgebraE7", type0()),
        ("AlgebraE8", type0()),
        ("GroupGL", arrow(nat_ty(), type0())),
        ("GroupSL", arrow(nat_ty(), type0())),
        ("GroupO", arrow(nat_ty(), type0())),
        ("GroupSO", arrow(nat_ty(), type0())),
        ("GroupU", arrow(nat_ty(), type0())),
        ("GroupSU", arrow(nat_ty(), type0())),
        ("GroupSp", arrow(nat_ty(), type0())),
        ("IsSimple", arrow(type0(), prop())),
        ("IsSemisimple", arrow(type0(), prop())),
        ("IsAbelian", arrow(type0(), prop())),
        ("IsNilpotent", arrow(type0(), prop())),
        ("IsSolvable", arrow(type0(), prop())),
        ("IsCompact", arrow(type0(), prop())),
        ("IsConnected", arrow(type0(), prop())),
        ("IsSimplyConnected", arrow(type0(), prop())),
        ("Rank", arrow(type0(), nat_ty())),
        ("Dimension", arrow(type0(), nat_ty())),
        ("HighestRoot", arrow(type0(), type0())),
        ("WeylGroup", arrow(type0(), type0())),
        ("WeylDimFormula", arrow(type0(), arrow(type0(), nat_ty()))),
        ("jacobi_identity", jacobi_identity_ty()),
        ("cartan_criterion", cartan_criterion_ty()),
        ("ado_theorem", ado_theorem_ty()),
        ("lie_theorem", lie_theorem_ty()),
        ("engel_theorem", engel_theorem_ty()),
        ("weyl_theorem", weyl_theorem_ty()),
        (
            "weyl_character_formula",
            weyl_character_formula_ty_theorem(),
        ),
        ("bch_formula", bch_formula_ty()),
        ("killing_form_signature", killing_form_signature_ty()),
        ("LieGroupLieAlgebra", arrow(type0(), type0())),
        ("CenterOf", arrow(type0(), type0())),
        ("DerivedAlgebra", arrow(type0(), type0())),
        (
            "LowerCentralSeries",
            arrow(type0(), arrow(nat_ty(), type0())),
        ),
        (
            "UpperCentralSeries",
            arrow(type0(), arrow(nat_ty(), type0())),
        ),
        ("RootOfUnity", arrow(nat_ty(), type0())),
        ("CoxeterNumber", arrow(type0(), nat_ty())),
        ("DualCoxeterNumber", arrow(type0(), nat_ty())),
        ("CompleteReducibility", complete_reducibility_ty()),
        ("RepCharacter", character_ty()),
        ("SchurLemma", schur_lemma_ty()),
        ("VermaModule", verma_module_ty()),
        ("BggCategoryO", bgg_category_o_ty()),
        ("BggResolution", bgg_resolution_ty()),
        ("KLPolynomial", kl_polynomial_ty()),
        ("CanonicalBasis", canonical_basis_ty()),
        ("KLConjecture", kl_conjecture_ty()),
        ("QuantumGroup", quantum_group_ty()),
        ("QuantumUEA", quantum_uea_ty()),
        ("RMatrix", r_matrix_ty()),
        ("HopfAlgebra", hopf_algebra_ty()),
        ("DrinfeldJimbo", drinfeld_jimbo_ty()),
        ("AffineLieAlgebra", affine_lie_algebra_ty()),
        ("KacMoodyAlgebra", kac_moody_algebra_ty()),
        ("GeneralizedCartanMatrix", generalized_cartan_matrix_ty()),
        ("AffineLevel", affine_level_ty()),
        ("IntegrableHWM", integrable_hwm_ty()),
        ("VirasoroAlgebra", virasoro_algebra_ty()),
        ("CentralCharge", central_charge_ty()),
        ("VirasoroHWModule", virasoro_hw_module_ty()),
        ("DModule", d_module_ty()),
        ("PerverseSheaf", perverse_sheaf_ty()),
        ("BBLocalization", bb_localization_ty()),
        ("IntersectionCohomology", intersection_cohomology_ty()),
        ("LieSuperalgebra", lie_superalgebra_ty()),
        ("SuperLieBracket", super_lie_bracket_ty()),
        ("LieSuperRep", lie_super_rep_ty()),
        ("HeckeAlgebra", hecke_algebra_ty()),
        ("HeckeModule", hecke_module_ty()),
        ("CrystalBasis", crystal_basis_ty()),
        ("CrystalGraph", crystal_graph_ty()),
        ("CrystalOperators", crystal_operators_ty()),
        ("GeometricSatake", geometric_satake_ty()),
        ("LanglandsDual", langlands_dual_ty()),
        ("LanglandsDuality", langlands_duality_ty()),
        ("NilpotentOrbit", nilpotent_orbit_ty()),
        ("SpringerResolution", springer_resolution_ty()),
        ("SpringerCorrespondence", springer_correspondence_ty()),
        ("SlodowlySlice", slodowy_slice_ty()),
        ("KirillovOrbitMethod", kirillov_orbit_method_ty()),
        ("CoadjointOrbit", coadjoint_orbit_ty()),
        ("KKSSymplecticForm", kks_symplectic_form_ty()),
        ("LoopGroup", loop_group_ty()),
        ("CentralExtension", central_extension_ty()),
        ("TwoCocycle", two_cocycle_ty()),
        ("AlgebraicGroup", algebraic_group_ty()),
        ("BorelSubgroup", borel_subgroup_ty()),
        ("FlagVariety", flag_variety_ty()),
        ("SchubertVariety", schubert_variety_ty()),
        ("AlgebraicGroupRep", algebraic_group_rep_ty()),
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
/// SU(2) generators: iσ_k / 2 where σ_k are Pauli matrices.
///
/// The generators satisfy [T_i, T_j] = ε_{ijk} T_k.
pub fn su2_generators() -> [Vec<Vec<f64>>; 3] {
    let half = 0.5_f64;
    let t1 = vec![vec![0.0, half], vec![-half, 0.0]];
    let t2 = vec![vec![0.0, half], vec![half, 0.0]];
    let t3 = vec![vec![half, 0.0], vec![0.0, -half]];
    [t1, t2, t3]
}
/// SO(3) generators: (L_k)_{ij} = -ε_{kij}.
///
/// The generators satisfy [L_i, L_j] = ε_{ijk} L_k.
pub fn so3_generators() -> [Vec<Vec<f64>>; 3] {
    let l1 = vec![
        vec![0.0, 0.0, 0.0],
        vec![0.0, 0.0, -1.0],
        vec![0.0, 1.0, 0.0],
    ];
    let l2 = vec![
        vec![0.0, 0.0, 1.0],
        vec![0.0, 0.0, 0.0],
        vec![-1.0, 0.0, 0.0],
    ];
    let l3 = vec![
        vec![0.0, -1.0, 0.0],
        vec![1.0, 0.0, 0.0],
        vec![0.0, 0.0, 0.0],
    ];
    [l1, l2, l3]
}
/// The Weyl character formula statement as a string.
pub fn weyl_character_formula_statement() -> &'static str {
    "ch(V_λ) = Σ_{w ∈ W} sgn(w) e^{w(λ+ρ)} / Σ_{w ∈ W} sgn(w) e^{w(ρ)}, \
     where ρ = (1/2) Σ_{α>0} α is the Weyl vector."
}
/// First N terms of the Baker-Campbell-Hausdorff series:
///   log(exp(X) exp(Y)) = X + Y + [X,Y]/2 + [X,[X,Y]]/12 - [Y,[X,Y]]/12 + ...
pub fn baker_campbell_hausdorff_series(terms: usize) -> Vec<String> {
    let all_terms = vec![
        "X".to_string(),
        "Y".to_string(),
        "(1/2)[X,Y]".to_string(),
        "(1/12)[X,[X,Y]]".to_string(),
        "-(1/12)[Y,[X,Y]]".to_string(),
        "-(1/24)[Y,[X,[X,Y]]]".to_string(),
        "(1/360)[X,[X,[X,[X,Y]]]]".to_string(),
        "(1/360)[Y,[Y,[Y,[Y,X]]]]".to_string(),
        "-(1/120)[Y,[X,[X,[X,Y]]]]".to_string(),
        "-(1/120)[X,[Y,[Y,[Y,X]]]]".to_string(),
    ];
    all_terms.into_iter().take(terms).collect()
}
/// Structure constants for su(2) in the basis {T₁, T₂, T₃}.
///
/// [T_i, T_j] = ε_{ijk} T_k  (Levi-Civita symbol).
pub fn su2_structure_constants() -> StructureConstants {
    let mut sc = StructureConstants::new(3);
    sc.algebra = "su(2)".to_string();
    sc.f[0][1][2] = 1.0;
    sc.f[1][2][0] = 1.0;
    sc.f[2][0][1] = 1.0;
    sc.f[1][0][2] = -1.0;
    sc.f[2][1][0] = -1.0;
    sc.f[0][2][1] = -1.0;
    sc
}
/// Structure constants for su(3) (Gell-Mann basis): f^{abc}.
///
/// The 8 generators λ_a satisfy [λ_a/2, λ_b/2] = i f^{abc} λ_c/2.
/// Non-zero independent values (others by antisymmetry):
pub fn su3_structure_constants() -> StructureConstants {
    let mut sc = StructureConstants::new(8);
    sc.algebra = "su(3)".to_string();
    let mut set = |a: usize, b: usize, c: usize, v: f64| {
        sc.f[a][b][c] = v;
        sc.f[b][a][c] = -v;
    };
    set(0, 1, 2, 1.0);
    set(0, 3, 6, 0.5);
    set(0, 5, 4, 0.5);
    set(1, 3, 5, 0.5);
    set(1, 4, 6, 0.5);
    set(2, 3, 4, 0.5);
    set(2, 6, 5, 0.5);
    let s3h = 3.0_f64.sqrt() / 2.0;
    set(3, 4, 7, s3h);
    set(5, 6, 7, s3h);
    sc
}
/// Cartan's criterion statement.
pub fn cartan_criterion_statement() -> &'static str {
    "Cartan's Criterion: A Lie algebra g is semisimple if and only if its \
     Killing form B(X,Y) = Tr(ad X ∘ ad Y) is non-degenerate. \
     Moreover g is solvable if and only if B(X,[Y,Z]) = 0 for all X ∈ g, \
     Y,Z ∈ [g,g] (i.e. B vanishes on [g,g] × g)."
}
#[cfg(test)]
mod extended_lie_tests {
    use super::*;
    #[test]
    fn test_cartan_subalgebra() {
        let h = CartanSubalgebraData::semisimple("sl_3", 2);
        assert!(h.is_abelian);
        assert_eq!(h.rank, 2);
        assert!(h.weight_space_description().contains("Cartan"));
    }
    #[test]
    fn test_lie_group_hom() {
        let adj = LieGroupHom::adjoint("G");
        assert!(adj.lie_algebra_map().contains("d phi"));
    }
    #[test]
    fn test_solvable() {
        let heis = SolvableLieAlgebra::heisenberg();
        assert!(heis.is_nilpotent);
        assert!(!heis.is_abelian);
        assert!(heis.lies_theorem_applies(true));
        assert!(!heis.lies_theorem_applies(false));
    }
    #[test]
    fn test_levi() {
        let ld = LeviDecomposition::new("g", "rad(g)", "s");
        assert!(ld.levi_is_semisimple());
        assert!(ld.description().contains("Levi"));
    }
    #[test]
    fn test_invariant_theory() {
        let inv = InvariantTheory::sl_n_invariants(3);
        assert_eq!(inv.num_invariants(), 2);
        assert!(inv.chevalley_description().contains("freely"));
    }
    #[test]
    fn test_nilpotent_orbit() {
        let reg = NilpotentOrbitData::regular("sl_3", 6);
        let zero = NilpotentOrbitData::zero("sl_3");
        assert!(zero.in_closure_of(&reg));
        assert!(!reg.in_closure_of(&zero));
    }
}
