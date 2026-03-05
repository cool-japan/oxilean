//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{
    ChainComplex, Cochain, CwComplex, HomologyGroup, HomotopyGroupData, SimplicialComplex,
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
pub fn bvar(n: u32) -> Expr {
    Expr::BVar(n)
}
pub fn nat_ty() -> Expr {
    cst("Nat")
}
pub fn group_ty() -> Expr {
    cst("Group")
}
pub fn ring_ty() -> Expr {
    cst("Ring")
}
pub fn space_ty() -> Expr {
    cst("TopologicalSpace")
}
/// `FundamentalGroup : TopologicalSpace → Group`
/// π₁(X, x₀): the fundamental group based at a point x₀.
pub fn fundamental_group_ty() -> Expr {
    arrow(space_ty(), group_ty())
}
/// `HigherHomotopyGroup : Nat → TopologicalSpace → Group`
/// πₙ(X, x₀) for n ≥ 2 (abelian groups).
pub fn higher_homotopy_group_ty() -> Expr {
    arrow(nat_ty(), arrow(space_ty(), group_ty()))
}
/// `SingularHomology : Nat → TopologicalSpace → Ring → Group`
/// H_n(X; R) with coefficients in R.
pub fn singular_homology_ty() -> Expr {
    arrow(nat_ty(), arrow(space_ty(), arrow(ring_ty(), group_ty())))
}
/// `CohomologyRing : TopologicalSpace → Ring → Ring`
/// H*(X; R) as a graded ring with the cup product.
pub fn cohomology_ring_ty() -> Expr {
    arrow(space_ty(), arrow(ring_ty(), ring_ty()))
}
/// `CoveringSpace : TopologicalSpace → TopologicalSpace → Prop`
/// p : E → B is a covering map.
pub fn covering_space_ty() -> Expr {
    arrow(space_ty(), arrow(space_ty(), prop()))
}
/// `CWComplex : Type`
/// A CW complex with cells and attaching maps.
pub fn cw_complex_ty() -> Expr {
    type0()
}
/// `SimplicialComplex : Type`
/// A simplicial complex (abstract or geometric).
pub fn simplicial_complex_ty() -> Expr {
    type0()
}
/// `SimplicialHomology : Nat → SimplicialComplex → Ring → Group`
/// H_n(K; R) — simplicial homology of a simplicial complex K.
pub fn simplicial_homology_ty() -> Expr {
    arrow(
        nat_ty(),
        arrow(cst("SimplicialComplex"), arrow(ring_ty(), group_ty())),
    )
}
/// `BoundaryOperator : Nat → SimplicialComplex → Group → Group`
/// ∂_n : C_n(K) → C_{n-1}(K), the boundary map of the simplicial chain complex.
pub fn boundary_operator_ty() -> Expr {
    arrow(
        nat_ty(),
        arrow(cst("SimplicialComplex"), arrow(group_ty(), group_ty())),
    )
}
/// `CellularHomology : Nat → CWComplex → Ring → Group`
/// H_n^{CW}(X; R) — cellular homology (agrees with singular homology for CW complexes).
pub fn cellular_homology_ty() -> Expr {
    arrow(
        nat_ty(),
        arrow(cst("CWComplex"), arrow(ring_ty(), group_ty())),
    )
}
/// `SingularCohomology : Nat → TopologicalSpace → Ring → Group`
/// H^n(X; R) — singular cohomology in degree n with coefficients in R.
pub fn singular_cohomology_ty() -> Expr {
    arrow(nat_ty(), arrow(space_ty(), arrow(ring_ty(), group_ty())))
}
/// `CupProduct : Nat → Nat → TopologicalSpace → Ring → Group`
/// ∪ : H^p(X;R) ⊗ H^q(X;R) → H^{p+q}(X;R).
pub fn cup_product_ty() -> Expr {
    arrow(
        nat_ty(),
        arrow(nat_ty(), arrow(space_ty(), arrow(ring_ty(), group_ty()))),
    )
}
/// `CapProduct : Nat → Nat → TopologicalSpace → Ring → Group`
/// ∩ : H^p(X;R) ⊗ H_n(X;R) → H_{n-p}(X;R).
pub fn cap_product_ty() -> Expr {
    arrow(
        nat_ty(),
        arrow(nat_ty(), arrow(space_ty(), arrow(ring_ty(), group_ty()))),
    )
}
/// `KunnethFormula : TopologicalSpace → TopologicalSpace → Ring → Prop`
/// H_n(X × Y; R) ≅ ⊕_{p+q=n} H_p(X;R) ⊗ H_q(Y;R)  (when R is a field).
pub fn kunneth_formula_ty() -> Expr {
    arrow(space_ty(), arrow(space_ty(), arrow(ring_ty(), prop())))
}
/// `LefschetzFixedPoint : TopologicalSpace → Prop`
/// If the Lefschetz number L(f) ≠ 0 then every continuous self-map has a fixed point.
pub fn lefschetz_fixed_point_ty() -> Expr {
    arrow(space_ty(), arrow(space_ty(), prop()))
}
/// `AlexanderDuality : Nat → TopologicalSpace → Ring → Prop`
/// For a compact subset K ⊂ S^n:
/// H̃^q(K;R) ≅ H̃_{n-q-1}(S^n ∖ K; R).
pub fn alexander_duality_ty() -> Expr {
    arrow(nat_ty(), arrow(space_ty(), arrow(ring_ty(), prop())))
}
/// `CechCohomology : Nat → TopologicalSpace → Ring → Group`
/// Ȟ^n(X; R) — Čech cohomology agreeing with singular cohomology for good spaces.
pub fn cech_cohomology_ty() -> Expr {
    arrow(nat_ty(), arrow(space_ty(), arrow(ring_ty(), group_ty())))
}
/// `SheafCohomology : Nat → TopologicalSpace → Type → Group`
/// H^n(X; F) — sheaf cohomology of a sheaf F on X.
pub fn sheaf_cohomology_ty() -> Expr {
    arrow(nat_ty(), arrow(space_ty(), arrow(type0(), group_ty())))
}
/// `SteenrodSquare : Nat → Nat → TopologicalSpace → Prop`
/// Sq^i : H^n(X; Z/2) → H^{n+i}(X; Z/2), the i-th Steenrod square.
pub fn steenrod_square_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), arrow(space_ty(), prop())))
}
/// `KTheory : TopologicalSpace → Ring`
/// K(X) — complex K-theory of a compact Hausdorff space X.
pub fn k_theory_ty() -> Expr {
    arrow(space_ty(), ring_ty())
}
/// `BottPeriodicity : TopologicalSpace → Prop`
/// K̃(Σ²X) ≅ K̃(X) — Bott periodicity for complex K-theory.
pub fn bott_periodicity_ty() -> Expr {
    arrow(space_ty(), prop())
}
/// `ClutchingConstruction : Nat → TopologicalSpace → Ring → Group`
/// Vector bundles on S^n classified via clutching functions on the equator S^{n-1}.
pub fn clutching_construction_ty() -> Expr {
    arrow(nat_ty(), arrow(space_ty(), arrow(ring_ty(), group_ty())))
}
/// `CobordismRing : Ring`
/// The cobordism ring Ω_* classifying smooth manifolds up to cobordism.
pub fn cobordism_ring_ty() -> Expr {
    ring_ty()
}
/// `StiefelWhitneyClass : Nat → TopologicalSpace → Ring → Group`
/// w_i(E) ∈ H^i(X; Z/2) for a vector bundle E → X.
pub fn stiefel_whitney_class_ty() -> Expr {
    arrow(nat_ty(), arrow(space_ty(), arrow(ring_ty(), group_ty())))
}
/// `PontryaginClass : Nat → TopologicalSpace → Ring → Group`
/// p_i(E) ∈ H^{4i}(X; Z) for a real vector bundle E → X.
pub fn pontryagin_class_ty() -> Expr {
    arrow(nat_ty(), arrow(space_ty(), arrow(ring_ty(), group_ty())))
}
/// `SerreSpectralSequence : TopologicalSpace → TopologicalSpace → Ring → Prop`
/// E_2^{p,q} ≅ H^p(B; H^q(F; R)) ⟹ H^{p+q}(E; R)
/// for a fibration F → E → B.
pub fn serre_spectral_sequence_ty() -> Expr {
    arrow(space_ty(), arrow(space_ty(), arrow(ring_ty(), prop())))
}
/// `EilenbergMaclaneSpace : Nat → Group → TopologicalSpace`
/// K(G, n): a space with π_n = G and all other homotopy groups trivial.
pub fn eilenberg_maclane_space_ty() -> Expr {
    arrow(nat_ty(), arrow(group_ty(), space_ty()))
}
/// `PostnikovTower : TopologicalSpace → Nat → TopologicalSpace`
/// P_n(X): the n-th Postnikov section of X, killing homotopy above degree n.
pub fn postnikov_tower_ty() -> Expr {
    arrow(space_ty(), arrow(nat_ty(), space_ty()))
}
/// `WhiteheadProduct : Nat → Nat → TopologicalSpace → Group`
/// [α, β] : π_m(X) × π_n(X) → π_{m+n-1}(X), the Whitehead product.
pub fn whitehead_product_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), arrow(space_ty(), group_ty())))
}
/// `JamesConstruction : TopologicalSpace → TopologicalSpace`
/// J(X): the free topological monoid on X; J(S^n) ≃ ΩΣS^n.
pub fn james_construction_ty() -> Expr {
    arrow(space_ty(), space_ty())
}
/// `EHPSequence : Nat → TopologicalSpace → Prop`
/// The EHP long exact sequence:
/// … → π_{n+1}(S^{2n+1}) →^H π_n(S^n) →^E π_{n+1}(S^{n+1}) →^P π_{n-1}(S^{2n-1}) → …
pub fn ehp_sequence_ty() -> Expr {
    arrow(nat_ty(), arrow(space_ty(), prop()))
}
/// `FibrationLongExactSeq : TopologicalSpace → TopologicalSpace → TopologicalSpace → Prop`
/// … → π_n(F) → π_n(E) → π_n(B) → π_{n-1}(F) → … for F → E → B a fibration.
pub fn fibration_long_exact_seq_ty() -> Expr {
    arrow(space_ty(), arrow(space_ty(), arrow(space_ty(), prop())))
}
/// `MayerVietorisSequence : TopologicalSpace → TopologicalSpace → TopologicalSpace → Prop`
/// … → H_n(A∩B) → H_n(A) ⊕ H_n(B) → H_n(A∪B) → H_{n-1}(A∩B) → …
pub fn mayer_vietoris_sequence_ty() -> Expr {
    arrow(space_ty(), arrow(space_ty(), arrow(space_ty(), prop())))
}
/// `SuspensionIsomorphism : Nat → TopologicalSpace → Ring → Prop`
/// H̃_n(X; R) ≅ H̃_{n+1}(ΣX; R), the reduced homology suspension isomorphism.
pub fn suspension_isomorphism_ty() -> Expr {
    arrow(nat_ty(), arrow(space_ty(), arrow(ring_ty(), prop())))
}
/// `HomologyFunctoriality : TopologicalSpace → TopologicalSpace → Nat → Prop`
/// A continuous map f: X → Y induces f_* : H_n(X;R) → H_n(Y;R).
pub fn homology_functoriality_ty() -> Expr {
    arrow(space_ty(), arrow(space_ty(), arrow(nat_ty(), prop())))
}
/// `HomotopyInvariance : TopologicalSpace → TopologicalSpace → Nat → Prop`
/// If f ≃ g (homotopic) then f_* = g_* on H_n.
pub fn homotopy_invariance_ty() -> Expr {
    arrow(space_ty(), arrow(space_ty(), arrow(nat_ty(), prop())))
}
/// `PersistentHomology : Nat → TopologicalSpace → Ring → Group`
/// PH_n(X; R): persistent homology in degree n tracking topological features
/// across a filtration.
pub fn persistent_homology_ty() -> Expr {
    arrow(nat_ty(), arrow(space_ty(), arrow(ring_ty(), group_ty())))
}
/// `BarcodeDecomposition : Nat → TopologicalSpace → Ring → Type`
/// The barcode (multiset of intervals [b, d)) representing PH_n(X; R).
pub fn barcode_decomposition_ty() -> Expr {
    arrow(nat_ty(), arrow(space_ty(), arrow(ring_ty(), type0())))
}
/// `PersistenceDiagram : Nat → TopologicalSpace → Type`
/// The persistence diagram: multiset of points (b, d) in the extended plane.
pub fn persistence_diagram_ty() -> Expr {
    arrow(nat_ty(), arrow(space_ty(), type0()))
}
/// `WassersteinDistance : Nat → TopologicalSpace → TopologicalSpace → Type`
/// d_W^p(D₁, D₂): the p-Wasserstein distance between two persistence diagrams.
pub fn wasserstein_distance_ty() -> Expr {
    arrow(nat_ty(), arrow(space_ty(), arrow(space_ty(), type0())))
}
/// `StabilityTheorem : Nat → TopologicalSpace → TopologicalSpace → Prop`
/// The stability theorem: d_W(PH(X), PH(Y)) ≤ d_GH(X, Y)
/// (bottleneck distance ≤ Gromov–Hausdorff distance).
pub fn stability_theorem_ty() -> Expr {
    arrow(nat_ty(), arrow(space_ty(), arrow(space_ty(), prop())))
}
/// `Sheaf : TopologicalSpace → Type → Type`
/// A sheaf F on a topological space X with values in a category C.
pub fn sheaf_ty() -> Expr {
    arrow(space_ty(), arrow(type0(), type0()))
}
/// `SheafMorphism : TopologicalSpace → Type → Type → Prop`
/// A morphism of sheaves F → G on X.
pub fn sheaf_morphism_ty() -> Expr {
    arrow(space_ty(), arrow(type0(), arrow(type0(), prop())))
}
/// `DerivedFunctor : Nat → TopologicalSpace → Type → Group`
/// R^n F: the n-th right derived functor of a left-exact functor F.
pub fn derived_functor_ty() -> Expr {
    arrow(nat_ty(), arrow(space_ty(), arrow(type0(), group_ty())))
}
/// `GrothendieckTopos : Type`
/// A Grothendieck topos: a category equivalent to sheaves on a site.
pub fn grothendieck_topos_ty() -> Expr {
    type0()
}
/// `LocallyConstantSheaf : TopologicalSpace → Ring → Type`
/// A locally constant sheaf (local coefficient system) on X with stalk R.
pub fn locally_constant_sheaf_ty() -> Expr {
    arrow(space_ty(), arrow(ring_ty(), type0()))
}
/// `SpectralSequence : Nat → Ring → Type`
/// A spectral sequence E_r^{p,q} converging to the associated graded of some filtered complex.
pub fn spectral_sequence_ty() -> Expr {
    arrow(nat_ty(), arrow(ring_ty(), type0()))
}
/// `AdemsRelations : Nat → Nat → TopologicalSpace → Prop`
/// The Adem relations for Steenrod operations:
/// Sq^a Sq^b = Σ Sq^{a+b-c} Sq^c when a < 2b.
pub fn adem_relations_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), arrow(space_ty(), prop())))
}
/// `AtiyahHirzebruchSS : TopologicalSpace → Ring → Prop`
/// The Atiyah–Hirzebruch spectral sequence:
/// E_2^{p,q} = H^p(X; K^q(pt)) ⟹ K^{p+q}(X).
pub fn atiyah_hirzebruch_ss_ty() -> Expr {
    arrow(space_ty(), arrow(ring_ty(), prop()))
}
/// `AdamsSpectralSequence : TopologicalSpace → TopologicalSpace → Prop`
/// The Adams spectral sequence converging to stable homotopy classes:
/// E_2^{s,t} = Ext_{A}^{s,t}(H*(Y), H*(X)) ⟹ [X, Y]_*.
pub fn adams_spectral_sequence_ty() -> Expr {
    arrow(space_ty(), arrow(space_ty(), prop()))
}
/// `ChernClass : Nat → TopologicalSpace → Ring → Group`
/// c_i(E) ∈ H^{2i}(X; Z) for a complex vector bundle E → X.
pub fn chern_class_ty() -> Expr {
    arrow(nat_ty(), arrow(space_ty(), arrow(ring_ty(), group_ty())))
}
/// `EulerClass : TopologicalSpace → Ring → Group`
/// e(E) ∈ H^n(X; Z) for an oriented rank-n vector bundle E → X.
pub fn euler_class_ty() -> Expr {
    arrow(space_ty(), arrow(ring_ty(), group_ty()))
}
/// `ThomIsomorphism : Nat → TopologicalSpace → Ring → Prop`
/// Φ : H^k(X; R) ≅ H^{k+n}(E, E₀; R) — the Thom isomorphism for an
/// oriented rank-n bundle.
pub fn thom_isomorphism_ty() -> Expr {
    arrow(nat_ty(), arrow(space_ty(), arrow(ring_ty(), prop())))
}
/// `CharacteristicNumber : TopologicalSpace → Ring → Type`
/// A characteristic number: evaluation of a characteristic class on the
/// fundamental class of an oriented manifold.
pub fn characteristic_number_ty() -> Expr {
    arrow(space_ty(), arrow(ring_ty(), type0()))
}
/// `RealKTheory : TopologicalSpace → Ring`
/// KO(X) — real K-theory of a compact Hausdorff space X.
pub fn real_k_theory_ty() -> Expr {
    arrow(space_ty(), ring_ty())
}
/// `EquivariantKTheory : TopologicalSpace → Group → Ring`
/// K_G(X) — equivariant K-theory with respect to a group action G ↷ X.
pub fn equivariant_k_theory_ty() -> Expr {
    arrow(space_ty(), arrow(group_ty(), ring_ty()))
}
/// `KGroupExactSeq : TopologicalSpace → TopologicalSpace → Prop`
/// The six-term exact sequence in K-theory for a pair (X, A):
/// K^0(X, A) → K^0(X) → K^0(A) → K^1(A) → K^1(X) → K^1(X, A).
pub fn k_group_exact_seq_ty() -> Expr {
    arrow(space_ty(), arrow(space_ty(), prop()))
}
/// `OrientedCobordism : TopologicalSpace → TopologicalSpace → Prop`
/// M and N are oriented cobordant (there exists an oriented manifold W
/// with ∂W = M ⊔ N̄).
pub fn oriented_cobordism_ty() -> Expr {
    arrow(space_ty(), arrow(space_ty(), prop()))
}
/// `CobordismInvariant : TopologicalSpace → Ring → Type`
/// A cobordism invariant of a closed manifold (e.g., signature, Todd genus).
pub fn cobordism_invariant_ty() -> Expr {
    arrow(space_ty(), arrow(ring_ty(), type0()))
}
/// `ThomTransversality : TopologicalSpace → TopologicalSpace → Prop`
/// Thom's transversality theorem: any smooth map is homotopic to a
/// transverse map.
pub fn thom_transversality_ty() -> Expr {
    arrow(space_ty(), arrow(space_ty(), prop()))
}
/// `FiberBundle : TopologicalSpace → TopologicalSpace → TopologicalSpace → Prop`
/// p : E → B with fiber F is a fiber bundle.
pub fn fiber_bundle_ty() -> Expr {
    arrow(space_ty(), arrow(space_ty(), arrow(space_ty(), prop())))
}
/// `VectorBundle : Nat → TopologicalSpace → Type`
/// A rank-n real vector bundle over a base space B.
pub fn vector_bundle_ty() -> Expr {
    arrow(nat_ty(), arrow(space_ty(), type0()))
}
/// `PrincipalBundle : Group → TopologicalSpace → Type`
/// A principal G-bundle over a base space B.
pub fn principal_bundle_ty() -> Expr {
    arrow(group_ty(), arrow(space_ty(), type0()))
}
/// `Connection : TopologicalSpace → Type → Type`
/// A connection (covariant derivative) on a vector bundle over X.
pub fn connection_ty() -> Expr {
    arrow(space_ty(), arrow(type0(), type0()))
}
/// `CurvatureForm : TopologicalSpace → Type → Type`
/// The curvature 2-form F_∇ of a connection ∇ on a bundle.
pub fn curvature_form_ty() -> Expr {
    arrow(space_ty(), arrow(type0(), type0()))
}
/// `GaugeTransformation : TopologicalSpace → Group → Type`
/// A gauge transformation: a section of the automorphism bundle of a
/// principal G-bundle.
pub fn gauge_transformation_ty() -> Expr {
    arrow(space_ty(), arrow(group_ty(), type0()))
}
/// `FredholmOperator : TopologicalSpace → Type`
/// A Fredholm operator: bounded linear operator with finite-dimensional
/// kernel and cokernel.
pub fn fredholm_operator_ty() -> Expr {
    arrow(space_ty(), type0())
}
/// `AnalyticIndex : TopologicalSpace → Type → Type`
/// The analytic index of an elliptic differential operator D:
/// ind(D) = dim ker D − dim coker D.
pub fn analytic_index_ty() -> Expr {
    arrow(space_ty(), arrow(type0(), type0()))
}
/// `TopologicalIndex : TopologicalSpace → Type → Ring`
/// The topological index of an elliptic operator, defined via K-theory
/// and characteristic classes.
pub fn topological_index_ty() -> Expr {
    arrow(space_ty(), arrow(type0(), ring_ty()))
}
/// `AtiyahSingerIndexTheorem : TopologicalSpace → Type → Prop`
/// The Atiyah–Singer index theorem: analytic index = topological index
/// for an elliptic operator on a closed manifold.
pub fn atiyah_singer_index_ty() -> Expr {
    arrow(space_ty(), arrow(type0(), prop()))
}
/// `DiracOperator : TopologicalSpace → Type`
/// The Dirac operator on a Riemannian spin manifold.
pub fn dirac_operator_ty() -> Expr {
    arrow(space_ty(), type0())
}
/// `SurgeryObstruction : Nat → TopologicalSpace → Group`
/// The Wall surgery obstruction in L_n(π₁(X)) for a degree-1 normal map.
pub fn surgery_obstruction_ty() -> Expr {
    arrow(nat_ty(), arrow(space_ty(), group_ty()))
}
/// `NormalMap : TopologicalSpace → TopologicalSpace → Prop`
/// A degree-1 normal map (M, ν, f) → (X, ξ) covering a map f : M → X.
pub fn normal_map_ty() -> Expr {
    arrow(space_ty(), arrow(space_ty(), prop()))
}
/// `SurgeryExactSequence : Nat → TopologicalSpace → Prop`
/// The Browder–Novikov–Sullivan–Wall surgery exact sequence:
/// … → L_{n+1}(π₁) → S(X) → [X, G/O] → L_n(π₁) → …
pub fn surgery_exact_sequence_ty() -> Expr {
    arrow(nat_ty(), arrow(space_ty(), prop()))
}
/// `SmoothManifold : Nat → Type`
/// A smooth n-manifold (without boundary).
pub fn smooth_manifold_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `IntersectionForm : TopologicalSpace → Ring → Type`
/// The intersection form on H_2(M; Z) for a 4-manifold M.
pub fn intersection_form_ty() -> Expr {
    arrow(space_ty(), arrow(ring_ty(), type0()))
}
/// `IntersectionHomology : Nat → TopologicalSpace → Ring → Group`
/// IH^p_n(X; R): intersection homology with perversity p for a
/// stratified pseudomanifold X.
pub fn intersection_homology_ty() -> Expr {
    arrow(nat_ty(), arrow(space_ty(), arrow(ring_ty(), group_ty())))
}
/// `IntersectionCohomology : Nat → TopologicalSpace → Ring → Group`
/// IH_p^n(X; R): intersection cohomology satisfying Poincaré duality
/// for singular spaces.
pub fn intersection_cohomology_ty() -> Expr {
    arrow(nat_ty(), arrow(space_ty(), arrow(ring_ty(), group_ty())))
}
/// `DeRhamCohomology : Nat → TopologicalSpace → Ring → Group`
/// H^n_{dR}(M): de Rham cohomology via differential forms on a smooth manifold.
pub fn de_rham_cohomology_ty() -> Expr {
    arrow(nat_ty(), arrow(space_ty(), arrow(ring_ty(), group_ty())))
}
/// `DeRhamIsomorphism : Nat → TopologicalSpace → Prop`
/// The de Rham theorem: H^n_{dR}(M) ≅ H^n(M; R) for smooth manifolds.
pub fn de_rham_isomorphism_ty() -> Expr {
    arrow(nat_ty(), arrow(space_ty(), prop()))
}
/// `Operad : Type`
/// An operad O: a collection O(n) of operations with composition and
/// symmetric group actions.
pub fn operad_ty() -> Expr {
    type0()
}
/// `LittleDisksOperad : Nat → Type`
/// E_n: the little n-disks operad encoding n-fold loop spaces.
pub fn little_disks_operad_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `AlgebraOverOperad : Type → Type → Type`
/// An algebra over an operad O: a space A with structure maps O(n) × A^n → A.
pub fn algebra_over_operad_ty() -> Expr {
    arrow(type0(), arrow(type0(), type0()))
}
/// `FactorizationHomology : Nat → TopologicalSpace → Type → Group`
/// ∫_M A: the factorization homology of an E_n-algebra A over an n-manifold M.
pub fn factorization_homology_ty() -> Expr {
    arrow(nat_ty(), arrow(space_ty(), arrow(type0(), group_ty())))
}
/// `NonabelianPoincareDuality : Nat → TopologicalSpace → Type → Prop`
/// ∫_M A ≃ Map_c(M, BA): the nonabelian Poincaré duality theorem.
pub fn nonabelian_poincare_duality_ty() -> Expr {
    arrow(nat_ty(), arrow(space_ty(), arrow(type0(), prop())))
}
/// `ConfigurationSpace : Nat → TopologicalSpace → TopologicalSpace`
/// Conf_n(X): the ordered configuration space of n distinct points in X.
pub fn configuration_space_ty() -> Expr {
    arrow(nat_ty(), arrow(space_ty(), space_ty()))
}
/// `UnorderedConfigSpace : Nat → TopologicalSpace → TopologicalSpace`
/// UConf_n(X): the unordered configuration space (= Conf_n(X)/S_n).
pub fn unordered_config_space_ty() -> Expr {
    arrow(nat_ty(), arrow(space_ty(), space_ty()))
}
/// `BraidGroup : Nat → Group`
/// B_n: the braid group on n strands (= π₁(UConf_n(ℝ²))).
pub fn braid_group_ty() -> Expr {
    arrow(nat_ty(), group_ty())
}
/// `PureBraidGroup : Nat → Group`
/// P_n: the pure braid group (kernel of B_n → S_n).
pub fn pure_braid_group_ty() -> Expr {
    arrow(nat_ty(), group_ty())
}
/// `ArfInvariant : TopologicalSpace → Ring → Type`
/// The Arf invariant of a quadratic form on H_1(M; Z/2) for a surface M.
pub fn arf_invariant_ty() -> Expr {
    arrow(space_ty(), arrow(ring_ty(), type0()))
}
/// `MappingClassGroup : TopologicalSpace → Group`
/// MCG(Σ) = π₀(Diff⁺(Σ)): the mapping class group of an oriented surface.
pub fn mapping_class_group_ty() -> Expr {
    arrow(space_ty(), group_ty())
}
/// `TeichmullerSpace : Nat → Nat → TopologicalSpace`
/// T(g, n): the Teichmüller space of genus-g surfaces with n punctures.
pub fn teichmuller_space_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), space_ty()))
}
/// `ModuliSpaceCurves : Nat → Nat → TopologicalSpace`
/// M(g, n) = T(g, n) / MCG(g, n): the moduli space of Riemann surfaces.
pub fn moduli_space_curves_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), space_ty()))
}
/// `DehnTwist : TopologicalSpace → Group → Group`
/// A Dehn twist along a simple closed curve c on a surface Σ; these
/// generate the mapping class group.
pub fn dehn_twist_ty() -> Expr {
    arrow(space_ty(), arrow(group_ty(), group_ty()))
}
/// `WeilPeterssonMetric : Nat → Nat → Type`
/// The Weil–Petersson Kähler metric on Teichmüller space T(g, n).
pub fn weil_petersson_metric_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// Seifert–van Kampen theorem: if X = U ∪ V with U, V, U∩V path-connected, then
/// π₁(X) ≅ π₁(U) *_{π₁(U∩V)} π₁(V)  (amalgamated free product).
pub fn van_kampen_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        space_ty(),
        pi(
            BinderInfo::Default,
            "U",
            space_ty(),
            arrow(space_ty(), prop()),
        ),
    )
}
/// Hurewicz theorem: for a simply-connected space X,
/// the Hurewicz map π_n(X) → H_n(X; Z) is an isomorphism for the first n ≥ 2
/// with π_n(X) ≠ 0.
pub fn hurewicz_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        arrow(space_ty(), arrow(prop(), prop())),
    )
}
/// Whitehead's theorem: a weak homotopy equivalence between CW complexes is a
/// (strong) homotopy equivalence.
pub fn whitehead_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        space_ty(),
        pi(
            BinderInfo::Default,
            "Y",
            space_ty(),
            arrow(prop(), arrow(prop(), arrow(prop(), prop()))),
        ),
    )
}
/// Universal coefficient theorem: there is a (non-naturally split) short exact sequence
/// 0 → H_n(X;Z) ⊗ R → H_n(X;R) → Tor(H_{n-1}(X;Z), R) → 0.
pub fn universal_coefficient_ty() -> Expr {
    arrow(nat_ty(), arrow(space_ty(), arrow(ring_ty(), prop())))
}
/// Poincaré duality: for a compact oriented n-manifold M,
/// H^k(M; R) ≅ H_{n-k}(M; R).
pub fn poincare_duality_hy_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "k",
            nat_ty(),
            arrow(space_ty(), arrow(ring_ty(), arrow(prop(), prop()))),
        ),
    )
}
pub fn build_algebraic_topology_env(env: &mut Environment) {
    let axioms: &[(&str, Expr)] = &[
        ("TopologicalSpace", type0()),
        ("Group", type0()),
        ("Ring", type0()),
        ("FundamentalGroup", fundamental_group_ty()),
        ("HigherHomotopyGroup", higher_homotopy_group_ty()),
        ("SingularHomology", singular_homology_ty()),
        ("CohomologyRing", cohomology_ring_ty()),
        ("CoveringSpace", covering_space_ty()),
        ("CWComplex", cw_complex_ty()),
        ("SimplicialComplex", type0()),
        ("SimplicialHomology", simplicial_homology_ty()),
        ("BoundaryOperator", boundary_operator_ty()),
        ("CellularHomology", cellular_homology_ty()),
        ("SingularCohomology", singular_cohomology_ty()),
        ("CupProduct", cup_product_ty()),
        ("CapProduct", cap_product_ty()),
        ("CechCohomology", cech_cohomology_ty()),
        ("SheafCohomology", sheaf_cohomology_ty()),
        ("KTheory", k_theory_ty()),
        ("ClutchingConstruction", clutching_construction_ty()),
        ("StiefelWhitneyClass", stiefel_whitney_class_ty()),
        ("PontryaginClass", pontryagin_class_ty()),
        ("EilenbergMaclaneSpace", eilenberg_maclane_space_ty()),
        ("PostnikovTower", postnikov_tower_ty()),
        ("WhiteheadProduct", whitehead_product_ty()),
        ("JamesConstruction", james_construction_ty()),
        ("PathConnected", arrow(space_ty(), prop())),
        ("SimplyConnected", arrow(space_ty(), prop())),
        (
            "CompactOriented",
            arrow(nat_ty(), arrow(space_ty(), prop())),
        ),
        ("WeakEquiv", arrow(space_ty(), arrow(space_ty(), prop()))),
        (
            "HomotopyEquiv",
            arrow(space_ty(), arrow(space_ty(), prop())),
        ),
        ("van_kampen", van_kampen_ty()),
        ("hurewicz", hurewicz_ty()),
        ("whitehead", whitehead_ty()),
        ("universal_coefficient", universal_coefficient_ty()),
        ("poincare_duality", poincare_duality_hy_ty()),
        ("kunneth_formula", kunneth_formula_ty()),
        ("lefschetz_fixed_point", lefschetz_fixed_point_ty()),
        ("alexander_duality", alexander_duality_ty()),
        ("steenrod_square", steenrod_square_ty()),
        ("bott_periodicity", bott_periodicity_ty()),
        ("cobordism_ring", cobordism_ring_ty()),
        ("serre_spectral_sequence", serre_spectral_sequence_ty()),
        ("ehp_sequence", ehp_sequence_ty()),
        ("fibration_long_exact_seq", fibration_long_exact_seq_ty()),
        ("mayer_vietoris_sequence", mayer_vietoris_sequence_ty()),
        ("suspension_isomorphism", suspension_isomorphism_ty()),
        ("homology_functoriality", homology_functoriality_ty()),
        ("homotopy_invariance", homotopy_invariance_ty()),
        ("PersistentHomology", persistent_homology_ty()),
        ("BarcodeDecomposition", barcode_decomposition_ty()),
        ("PersistenceDiagram", persistence_diagram_ty()),
        ("WassersteinDistance", wasserstein_distance_ty()),
        ("stability_theorem", stability_theorem_ty()),
        ("Sheaf", sheaf_ty()),
        ("SheafMorphism", sheaf_morphism_ty()),
        ("DerivedFunctor", derived_functor_ty()),
        ("GrothendieckTopos", grothendieck_topos_ty()),
        ("LocallyConstantSheaf", locally_constant_sheaf_ty()),
        ("SpectralSequence", spectral_sequence_ty()),
        ("adem_relations", adem_relations_ty()),
        ("atiyah_hirzebruch_ss", atiyah_hirzebruch_ss_ty()),
        ("adams_spectral_sequence", adams_spectral_sequence_ty()),
        ("ChernClass", chern_class_ty()),
        ("EulerClass", euler_class_ty()),
        ("thom_isomorphism", thom_isomorphism_ty()),
        ("CharacteristicNumber", characteristic_number_ty()),
        ("RealKTheory", real_k_theory_ty()),
        ("EquivariantKTheory", equivariant_k_theory_ty()),
        ("k_group_exact_seq", k_group_exact_seq_ty()),
        ("oriented_cobordism", oriented_cobordism_ty()),
        ("CobordismInvariant", cobordism_invariant_ty()),
        ("thom_transversality", thom_transversality_ty()),
        ("fiber_bundle", fiber_bundle_ty()),
        ("VectorBundle", vector_bundle_ty()),
        ("PrincipalBundle", principal_bundle_ty()),
        ("Connection", connection_ty()),
        ("CurvatureForm", curvature_form_ty()),
        ("GaugeTransformation", gauge_transformation_ty()),
        ("FredholmOperator", fredholm_operator_ty()),
        ("AnalyticIndex", analytic_index_ty()),
        ("TopologicalIndex", topological_index_ty()),
        ("atiyah_singer_index", atiyah_singer_index_ty()),
        ("DiracOperator", dirac_operator_ty()),
        ("SurgeryObstruction", surgery_obstruction_ty()),
        ("NormalMap", normal_map_ty()),
        ("surgery_exact_sequence", surgery_exact_sequence_ty()),
        ("SmoothManifold", smooth_manifold_ty()),
        ("IntersectionForm", intersection_form_ty()),
        ("IntersectionHomology", intersection_homology_ty()),
        ("IntersectionCohomology", intersection_cohomology_ty()),
        ("DeRhamCohomology", de_rham_cohomology_ty()),
        ("de_rham_isomorphism", de_rham_isomorphism_ty()),
        ("Operad", operad_ty()),
        ("LittleDisksOperad", little_disks_operad_ty()),
        ("AlgebraOverOperad", algebra_over_operad_ty()),
        ("FactorizationHomology", factorization_homology_ty()),
        (
            "nonabelian_poincare_duality",
            nonabelian_poincare_duality_ty(),
        ),
        ("ConfigurationSpace", configuration_space_ty()),
        ("UnorderedConfigSpace", unordered_config_space_ty()),
        ("BraidGroup", braid_group_ty()),
        ("PureBraidGroup", pure_braid_group_ty()),
        ("ArfInvariant", arf_invariant_ty()),
        ("MappingClassGroup", mapping_class_group_ty()),
        ("TeichmullerSpace", teichmuller_space_ty()),
        ("ModuliSpaceCurves", moduli_space_curves_ty()),
        ("DehnTwist", dehn_twist_ty()),
        ("WeilPeterssonMetric", weil_petersson_metric_ty()),
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
/// Compute the rank of an integer matrix via Gaussian elimination over the rationals.
pub fn matrix_rank(mat: &[Vec<i32>]) -> usize {
    if mat.is_empty() {
        return 0;
    }
    let rows = mat.len();
    let cols = mat[0].len();
    if cols == 0 {
        return 0;
    }
    let mut a: Vec<Vec<f64>> = mat
        .iter()
        .map(|row| row.iter().map(|&v| v as f64).collect())
        .collect();
    let mut rank = 0usize;
    let mut pivot_col = 0usize;
    for r in 0..rows {
        if pivot_col >= cols {
            break;
        }
        let mut found = None;
        for i in r..rows {
            if a[i][pivot_col].abs() > 1e-9 {
                found = Some(i);
                break;
            }
        }
        if found.is_none() {
            pivot_col += 1;
            continue;
        }
        let pivot_row = found.expect("found is Some: checked by is_none guard above");
        a.swap(r, pivot_row);
        let pivot = a[r][pivot_col];
        for j in pivot_col..cols {
            a[r][j] /= pivot;
        }
        for i in 0..rows {
            if i != r && a[i][pivot_col].abs() > 1e-9 {
                let factor = a[i][pivot_col];
                for j in pivot_col..cols {
                    let sub = factor * a[r][j];
                    a[i][j] -= sub;
                }
            }
        }
        rank += 1;
        pivot_col += 1;
    }
    rank
}
/// Known homotopy groups π_n(S^k) of spheres.
///
/// Uses the classical table up to what is well-known.
/// Returns "unknown" for undetermined cases.
pub fn pi_n_sphere(n: u32, sphere_dim: u32) -> HomotopyGroupData {
    let description = match (n, sphere_dim) {
        (0, _) => "trivial",
        (k, m) if k < m => "trivial",
        (k, m) if k == m => "Z",
        (2, 1) => "trivial",
        (3, 2) => "Z",
        (4, 2) => "Z/2Z",
        (5, 2) => "Z/2Z",
        (4, 3) => "Z/2Z",
        (5, 3) => "Z/2Z",
        (6, 3) => "Z/12Z",
        (7, 3) => "Z/2Z",
        (5, 4) => "Z/2Z",
        (6, 4) => "Z/2Z",
        (7, 4) => "Z x Z/12Z",
        _ => "unknown",
    };
    let is_abelian = !(n == sphere_dim && sphere_dim == 1);
    HomotopyGroupData {
        space: format!("S^{sphere_dim}"),
        base_point: "pt".to_string(),
        degree: n,
        description: description.to_string(),
        is_abelian: is_abelian || n >= 2,
    }
}
/// π₁(T²) = ℤ × ℤ (the torus has abelian fundamental group).
pub fn pi1_torus() -> HomotopyGroupData {
    HomotopyGroupData {
        space: "T^2".to_string(),
        base_point: "pt".to_string(),
        degree: 1,
        description: "Z x Z".to_string(),
        is_abelian: true,
    }
}
/// π₁(S^n):
/// - n = 1: Z (the circle)
/// - n ≥ 2: trivial (higher spheres are simply connected)
pub fn pi1_sphere(n: u32) -> HomotopyGroupData {
    let (description, is_abelian) = if n == 1 {
        ("Z", true)
    } else {
        ("trivial", true)
    };
    HomotopyGroupData {
        space: format!("S^{n}"),
        base_point: "pt".to_string(),
        degree: 1,
        description: description.to_string(),
        is_abelian,
    }
}
/// Helper: generate all k-element combinations from `items`.
pub fn combinations(items: &[usize], k: usize) -> Vec<Vec<usize>> {
    if k == 0 {
        return vec![vec![]];
    }
    if k > items.len() {
        return vec![];
    }
    let mut result = Vec::new();
    for i in 0..=items.len() - k {
        let head = items[i];
        for mut tail in combinations(&items[i + 1..], k - 1) {
            let mut combo = vec![head];
            combo.append(&mut tail);
            result.push(combo);
        }
    }
    result
}
/// Compute the Smith Normal Form of an integer matrix.
/// Returns the diagonal entries (the invariant factors), ignoring zeros.
pub fn smith_normal_form(mat: &[Vec<i32>]) -> Vec<i64> {
    if mat.is_empty() {
        return vec![];
    }
    let rows = mat.len();
    let cols = mat[0].len();
    let mut a: Vec<Vec<i64>> = mat
        .iter()
        .map(|r| r.iter().map(|&v| v as i64).collect())
        .collect();
    let mut diagonal: Vec<i64> = Vec::new();
    let mut pivot_row = 0usize;
    let mut pivot_col = 0usize;
    while pivot_row < rows && pivot_col < cols {
        let mut min_val: Option<i64> = None;
        let mut min_pos = (pivot_row, pivot_col);
        for r in pivot_row..rows {
            for c in pivot_col..cols {
                if a[r][c] != 0 {
                    let v = a[r][c].unsigned_abs();
                    match min_val {
                        None => {
                            min_val = Some(v as i64);
                            min_pos = (r, c);
                        }
                        Some(m) if (v as i64) < m => {
                            min_val = Some(v as i64);
                            min_pos = (r, c);
                        }
                        _ => {}
                    }
                }
            }
        }
        if min_val.is_none() {
            break;
        }
        a.swap(pivot_row, min_pos.0);
        for r in 0..rows {
            a[r].swap(pivot_col, min_pos.1);
        }
        if a[pivot_row][pivot_col] < 0 {
            for c in pivot_col..cols {
                a[pivot_row][c] = -a[pivot_row][c];
            }
        }
        let d = a[pivot_row][pivot_col];
        let mut changed = true;
        while changed {
            changed = false;
            for r in pivot_row + 1..rows {
                if a[r][pivot_col] != 0 {
                    let q = a[r][pivot_col] / a[pivot_row][pivot_col];
                    for c in pivot_col..cols {
                        let sub = q * a[pivot_row][c];
                        a[r][c] -= sub;
                    }
                    if a[r][pivot_col] != 0 {
                        changed = true;
                    }
                }
            }
            for c in pivot_col + 1..cols {
                if a[pivot_row][c] != 0 {
                    let q = a[pivot_row][c] / a[pivot_row][pivot_col];
                    for r in pivot_row..rows {
                        let sub = q * a[r][pivot_col];
                        a[r][c] -= sub;
                    }
                    if a[pivot_row][c] != 0 {
                        changed = true;
                    }
                }
            }
            if !changed {
                'outer: for r in pivot_row + 1..rows {
                    for c in pivot_col + 1..cols {
                        if a[r][c] % d != 0 {
                            for cc in pivot_col..cols {
                                let v = a[r][cc];
                                a[pivot_row][cc] += v;
                            }
                            changed = true;
                            break 'outer;
                        }
                    }
                }
            }
        }
        diagonal.push(a[pivot_row][pivot_col].abs());
        pivot_row += 1;
        pivot_col += 1;
    }
    diagonal
}
/// Compute the k-th homology group H_k of a simplicial complex over ℤ.
pub fn compute_homology(sc: &SimplicialComplex, k: usize) -> HomologyGroup {
    let max_dim = sc.simplices.iter().map(|s| s.len()).max().unwrap_or(0);
    if k >= max_dim {
        return HomologyGroup::trivial();
    }
    let rank_ck = sc.k_simplices(k).len();
    if rank_ck == 0 {
        return HomologyGroup::trivial();
    }
    let bm_k = sc.boundary_matrix(k);
    let bm_k1 = sc.boundary_matrix(k + 1);
    let snf_k1 = smith_normal_form(&bm_k1);
    let image_free: usize = snf_k1.iter().filter(|&&v| v != 0).count();
    let snf_k = smith_normal_form(&bm_k);
    let rank_dk: usize = snf_k.iter().filter(|&&v| v != 0).count();
    let ker_dim = rank_ck.saturating_sub(rank_dk);
    let free_rank = ker_dim.saturating_sub(image_free) as u32;
    let torsion: Vec<u64> = snf_k1
        .iter()
        .filter(|&&v| v > 1)
        .map(|&v| v as u64)
        .collect();
    HomologyGroup { free_rank, torsion }
}
/// Compute the cup product α ∪ β of two cochains.
///
/// For a p-cochain α and q-cochain β on a simplicial complex, the cup product
/// (α ∪ β)(σ) = α(σ|_{[0..p]}) · β(σ|_{[p..p+q]}) for each (p+q)-simplex σ.
pub fn cup_product(sc: &SimplicialComplex, alpha: &Cochain, beta: &Cochain) -> Cochain {
    let p = alpha.degree;
    let q = beta.degree;
    let pq = p + q;
    let pq_sims = sc.k_simplices(pq);
    let p_sims = sc.k_simplices(p);
    let q_sims = sc.k_simplices(q);
    let mut result = Cochain {
        degree: pq,
        values: vec![0; pq_sims.len()],
    };
    for (idx, sigma) in pq_sims.iter().enumerate() {
        let front: Vec<usize> = sigma[..=p].to_vec();
        let back: Vec<usize> = sigma[p..].to_vec();
        let alpha_val = if let Some(pi) = p_sims.iter().position(|s| **s == front) {
            alpha.eval(pi)
        } else {
            0
        };
        let beta_val = if let Some(qi) = q_sims.iter().position(|s| **s == back) {
            beta.eval(qi)
        } else {
            0
        };
        result.values[idx] = alpha_val * beta_val;
    }
    result
}
/// Betti numbers of the n-sphere S^n.
/// b_0 = b_n = 1, all others 0.
pub fn betti_sphere(n: u32) -> Vec<u32> {
    let mut b = vec![0u32; n as usize + 1];
    b[0] = 1;
    if n > 0 {
        b[n as usize] = 1;
    }
    b
}
/// Betti numbers of the torus T^2.
/// b_0 = 1, b_1 = 2, b_2 = 1.
pub fn betti_torus() -> Vec<u32> {
    vec![1, 2, 1]
}
/// Betti numbers of RP^n (real projective space) over ℤ.
/// H_0 = ℤ, H_k = ℤ/2ℤ for k odd 0 < k < n,
/// H_n = ℤ if n odd, 0 if n even.
/// Free ranks only (ignoring torsion):
/// b_0 = 1, b_n = 1 if n odd, 0 otherwise; all others 0.
pub fn betti_rpn(n: u32) -> Vec<u32> {
    let mut b = vec![0u32; n as usize + 1];
    b[0] = 1;
    if n > 0 && n % 2 == 1 {
        b[n as usize] = 1;
    }
    b
}
/// Betti numbers of the product S^m × S^n.
/// b_{0} = 1, b_m = 1, b_n = 1, b_{m+n} = 1 (with overlaps if m=n: b_m=2).
pub fn betti_sphere_product(m: u32, n: u32) -> Vec<u32> {
    let dim = (m + n) as usize;
    let mut b = vec![0u32; dim + 1];
    b[0] += 1;
    b[m as usize] += 1;
    b[n as usize] += 1;
    b[dim] += 1;
    b
}
/// Compute the Lefschetz number L(f) = Σ_k (-1)^k · tr(f_k)
/// where f_k is the induced map on H_k with ℚ coefficients.
///
/// `traces[k]` should contain tr(f_* : H_k(X;ℚ) → H_k(X;ℚ)).
pub fn lefschetz_number(traces: &[i64]) -> i64 {
    traces
        .iter()
        .enumerate()
        .map(|(k, &tr)| {
            let sign: i64 = if k % 2 == 0 { 1 } else { -1 };
            sign * tr
        })
        .sum()
}
/// Compute the Lefschetz number of the identity map on a space with given Betti numbers.
/// L(id) = Σ_k (-1)^k b_k = χ(X).
pub fn lefschetz_number_identity(betti: &[u32]) -> i64 {
    let traces: Vec<i64> = betti.iter().map(|&b| b as i64).collect();
    lefschetz_number(&traces)
}
/// Compute the Lefschetz number of the antipodal map on S^n.
/// The antipodal map acts as (-1)^{n+1} on H_n and +1 on H_0.
/// L(A) = 1 + (-1)^n · (-1)^{n+1} = 1 + (-1)^{2n+1} = 1 - 1 = 0 (for n > 0).
pub fn lefschetz_antipodal_sphere(n: u32) -> i64 {
    if n == 0 {
        return 2;
    }
    let trace_n: i64 = if (n + 1) % 2 == 0 { 1 } else { -1 };
    let sign_n: i64 = if n % 2 == 0 { 1 } else { -1 };
    1 + sign_n * trace_n
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sphere_cw_structure() {
        for n in [0u32, 1, 2, 3, 5] {
            let s = CwComplex::sphere(n);
            assert_eq!(s.n_cells(0), 1, "S^{n} has 1 zero-cell");
            if n > 0 {
                assert_eq!(s.n_cells(n), 1, "S^{n} has 1 top cell");
                for k in 1..n {
                    assert_eq!(s.n_cells(k), 0, "S^{n} has no {k}-cells");
                }
            }
        }
    }
    #[test]
    fn test_torus_euler_char() {
        let t2 = CwComplex::torus();
        assert_eq!(t2.euler_characteristic(), 0, "χ(T²) = 0");
    }
    #[test]
    fn test_sphere_euler_char_s2() {
        let s2 = CwComplex::sphere(2);
        assert_eq!(s2.euler_characteristic(), 2, "χ(S²) = 2");
    }
    #[test]
    fn test_rp2_euler_char() {
        let rp2 = CwComplex::real_projective_plane();
        assert_eq!(rp2.euler_characteristic(), 1, "χ(RP²) = 1");
    }
    #[test]
    fn test_cw_add_cell() {
        let mut cw = CwComplex::new("test");
        assert_eq!(cw.cells.len(), 0);
        cw.add_cell(0, "v", vec![]);
        assert_eq!(cw.cells.len(), 1);
        cw.add_cell(1, "e", vec![(0, 0)]);
        assert_eq!(cw.cells.len(), 2);
        assert_eq!(cw.n_cells(0), 1);
        assert_eq!(cw.n_cells(1), 1);
    }
    #[test]
    fn test_pi1_sphere_s1() {
        let pi1 = pi1_sphere(1);
        assert_eq!(pi1.description, "Z");
        assert!(pi1.is_abelian);
    }
    #[test]
    fn test_pi1_sphere_s2() {
        let pi1 = pi1_sphere(2);
        assert_eq!(pi1.description, "trivial");
    }
    #[test]
    fn test_betti_numbers_s2() {
        let s2 = CwComplex::sphere(2);
        let cc = ChainComplex::from_cw(&s2);
        let b = cc.betti_numbers();
        assert_eq!(b.len(), 3, "should have 3 Betti numbers, got {}", b.len());
        assert_eq!(b[0], 1, "b_0(S²) = 1");
        assert_eq!(b[1], 0, "b_1(S²) = 0");
        assert_eq!(b[2], 1, "b_2(S²) = 1");
    }
    #[test]
    fn test_simplicial_complex_triangle() {
        let mut sc = SimplicialComplex::new("triangle");
        sc.add_simplex(vec![0, 1, 2]);
        assert_eq!(sc.k_simplices(0).len(), 3, "3 vertices");
        assert_eq!(sc.k_simplices(1).len(), 3, "3 edges");
        assert_eq!(sc.k_simplices(2).len(), 1, "1 face");
    }
    #[test]
    fn test_sphere_triangulation_s1() {
        let sc = SimplicialComplex::sphere_triangulation(2);
        assert_eq!(sc.k_simplices(0).len(), 3, "3 vertices in ∂Δ²");
        assert_eq!(sc.k_simplices(1).len(), 3, "3 edges in ∂Δ²");
    }
    #[test]
    fn test_boundary_matrix_edge() {
        let mut sc = SimplicialComplex::new("edge");
        sc.add_simplex(vec![0, 1]);
        let bm = sc.boundary_matrix(1);
        assert_eq!(bm.len(), 2, "2 rows");
        assert_eq!(bm[0].len(), 1, "1 column");
        let sum: i32 = bm.iter().map(|r| r[0]).sum();
        assert_eq!(sum, 0, "boundary of an edge sums to zero");
    }
    #[test]
    fn test_homology_circle() {
        let sc = SimplicialComplex::sphere_triangulation(2);
        let h0 = compute_homology(&sc, 0);
        let h1 = compute_homology(&sc, 1);
        assert_eq!(h0.free_rank, 1, "H_0(S^1) = Z");
        assert_eq!(h1.free_rank, 1, "H_1(S^1) = Z");
        assert!(h1.torsion.is_empty(), "H_1(S^1) has no torsion");
    }
    #[test]
    fn test_betti_sphere_function() {
        let b2 = betti_sphere(2);
        assert_eq!(b2, vec![1, 0, 1]);
        let b0 = betti_sphere(0);
        assert_eq!(b0, vec![1]);
        let b1 = betti_sphere(1);
        assert_eq!(b1, vec![1, 1]);
    }
    #[test]
    fn test_betti_torus() {
        let b = betti_torus();
        assert_eq!(b, vec![1, 2, 1]);
    }
    #[test]
    fn test_betti_rpn() {
        assert_eq!(betti_rpn(1), vec![1, 1]);
        assert_eq!(betti_rpn(2), vec![1, 0, 0]);
        assert_eq!(betti_rpn(3), vec![1, 0, 0, 1]);
    }
    #[test]
    fn test_lefschetz_identity() {
        let b = betti_sphere(2);
        assert_eq!(lefschetz_number_identity(&b), 2);
        let bt = betti_torus();
        assert_eq!(lefschetz_number_identity(&bt), 0);
    }
    #[test]
    fn test_lefschetz_antipodal() {
        assert_eq!(lefschetz_antipodal_sphere(1), 0);
        assert_eq!(lefschetz_antipodal_sphere(2), 0);
    }
    #[test]
    fn test_smith_normal_form_trivial() {
        let mat: Vec<Vec<i32>> = vec![];
        let snf = smith_normal_form(&mat);
        assert!(snf.is_empty());
    }
    #[test]
    fn test_smith_normal_form_identity_2x2() {
        let mat = vec![vec![1, 0], vec![0, 1]];
        let snf = smith_normal_form(&mat);
        assert_eq!(snf, vec![1, 1]);
    }
    #[test]
    fn test_homology_group_description() {
        assert_eq!(HomologyGroup::trivial().description(), "0");
        assert_eq!(HomologyGroup::integers().description(), "Z");
        assert_eq!(HomologyGroup::free(2).description(), "Z^2");
        assert_eq!(HomologyGroup::cyclic(2).description(), "Z/2Z");
    }
    #[test]
    fn test_cup_product_trivial() {
        let mut sc = SimplicialComplex::new("triangle");
        sc.add_simplex(vec![0, 1, 2]);
        let alpha = Cochain::zero(&sc, 0);
        let beta = Cochain::zero(&sc, 0);
        let gamma = cup_product(&sc, &alpha, &beta);
        assert_eq!(gamma.degree, 0);
        assert!(gamma.values.iter().all(|&v| v == 0));
    }
    #[test]
    fn test_build_algebraic_topology_env() {
        use oxilean_kernel::Environment;
        let mut env = Environment::new();
        build_algebraic_topology_env(&mut env);
        let check_names = [
            "SimplicialHomology",
            "CupProduct",
            "KTheory",
            "bott_periodicity",
            "serre_spectral_sequence",
            "EilenbergMaclaneSpace",
            "StiefelWhitneyClass",
            "PontryaginClass",
            "JamesConstruction",
            "mayer_vietoris_sequence",
            "homotopy_invariance",
            "suspension_isomorphism",
        ];
        for name in check_names {
            assert!(
                env.get(&oxilean_kernel::Name::str(name)).is_some(),
                "axiom '{name}' should be registered"
            );
        }
    }
}
