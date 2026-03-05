//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{
    DehnSurgery, DehnSurgeryComputer, FourManifold, HeegaardDiagramSimplifier,
    HeegaardSplitting2V2, HeegaardSplittingData, IntersectionFormData, KhovanovHomologyComputer,
    KirbyDiagramData, KnotConcordanceChecker, KnotDiagram, ReidemeisterMove, Surface, SurfaceType,
    SurgerySpec, ThurstonGeometrizationData, ThurstonGeometry, ThurstonGeometryKind,
    WRTInvariantComputer,
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
pub fn int_ty() -> Expr {
    cst("Int")
}
pub fn bool_ty() -> Expr {
    cst("Bool")
}
pub fn ring_ty() -> Expr {
    cst("Ring")
}
pub fn group_ty() -> Expr {
    cst("Group")
}
pub fn manifold_ty() -> Expr {
    cst("Manifold")
}
pub fn surface_ty() -> Expr {
    cst("Surface")
}
pub fn knot_ty() -> Expr {
    cst("Knot")
}
/// `ClosedSurface : Type` — a compact connected surface without boundary.
pub fn closed_surface_ty() -> Expr {
    type0()
}
/// `SurfaceGenus : Surface → Nat`
/// The topological genus g (number of handles for orientable surfaces).
pub fn surface_genus_ty() -> Expr {
    arrow(surface_ty(), nat_ty())
}
/// `IsOrientable : Surface → Prop`
pub fn is_orientable_ty() -> Expr {
    arrow(surface_ty(), prop())
}
/// `EulerCharacteristic : Surface → Int`
/// χ(Σ) = 2 − 2g for orientable genus-g surfaces; χ = 2 − k for non-orientable.
pub fn euler_characteristic_surface_ty() -> Expr {
    arrow(surface_ty(), int_ty())
}
/// Classification theorem for compact surfaces:
/// Every compact connected surface is homeomorphic to a sphere, a connected sum of tori,
/// or a connected sum of projective planes.
/// ∀ (S : Surface), IsOrientable S ↔ ∃ g : Nat, S ≅ Σ_g   (orientable case)
pub fn surface_classification_ty() -> Expr {
    pi(BinderInfo::Default, "S", surface_ty(), prop())
}
/// `ConnectedSum : Surface → Surface → Surface` — the connected sum operation #.
pub fn connected_sum_surface_ty() -> Expr {
    arrow(surface_ty(), arrow(surface_ty(), surface_ty()))
}
/// `MappingTorus : Surface → (Surface → Surface) → Manifold`
/// The mapping torus M_φ of a homeomorphism φ : Σ → Σ.
pub fn mapping_torus_ty() -> Expr {
    arrow(
        surface_ty(),
        arrow(arrow(surface_ty(), surface_ty()), manifold_ty()),
    )
}
/// `HandlebodyGenus : Nat → Manifold`
/// A genus-g handlebody (compact orientable 3-manifold with boundary Σ_g).
pub fn handlebody_genus_ty() -> Expr {
    arrow(nat_ty(), manifold_ty())
}
/// `HeegaardSplitting : Manifold → Nat → Prop`
/// M admits a Heegaard splitting of genus g.
pub fn heegaard_splitting_ty() -> Expr {
    arrow(manifold_ty(), arrow(nat_ty(), prop()))
}
/// `HeegaardGenus : Manifold → Nat`
/// Minimum Heegaard genus of M.
pub fn heegaard_genus_ty() -> Expr {
    arrow(manifold_ty(), nat_ty())
}
/// Reidemeister-Singer theorem: any two Heegaard splittings of M become isotopic
/// after finitely many stabilizations.
pub fn reidemeister_singer_ty() -> Expr {
    pi(BinderInfo::Default, "M", manifold_ty(), prop())
}
/// `Irreducible3Manifold : Manifold → Prop`
/// Every embedded 2-sphere bounds a 3-ball.
pub fn irreducible_3manifold_ty() -> Expr {
    arrow(manifold_ty(), prop())
}
/// Kneser-Milnor prime decomposition theorem:
/// Every compact orientable 3-manifold decomposes uniquely (up to order) as a connected
/// sum of prime 3-manifolds.
pub fn kneser_milnor_ty() -> Expr {
    pi(BinderInfo::Default, "M", manifold_ty(), prop())
}
/// `JSJDecomposition : Manifold → Type`
/// The Jaco-Shalen-Johannsen decomposition along incompressible tori.
pub fn jsj_decomposition_ty() -> Expr {
    arrow(manifold_ty(), type0())
}
/// `DehnSurgery : Manifold → Knot → Nat → Nat → Manifold`
/// p/q-surgery on K ⊂ M yields a new 3-manifold.
pub fn dehn_surgery_ty() -> Expr {
    arrow(
        manifold_ty(),
        arrow(knot_ty(), arrow(nat_ty(), arrow(nat_ty(), manifold_ty()))),
    )
}
/// `SurgeryCoefficient : Type` — the rational number p/q labeling a surgery.
pub fn surgery_coefficient_ty() -> Expr {
    type0()
}
/// Lickorish-Wallace theorem: every closed orientable 3-manifold can be obtained by
/// integral Dehn surgery on a link in S³.
pub fn lickorish_wallace_ty() -> Expr {
    pi(BinderInfo::Default, "M", manifold_ty(), prop())
}
/// `DehnFillingCusp : Manifold → Manifold`
/// Filling a cusp of a hyperbolic manifold to get a closed manifold.
pub fn dehn_filling_ty() -> Expr {
    arrow(manifold_ty(), manifold_ty())
}
/// Thurston's Dehn surgery theorem: all but finitely many Dehn fillings of a
/// cusped hyperbolic 3-manifold yield hyperbolic 3-manifolds.
pub fn thurston_dehn_surgery_ty() -> Expr {
    pi(BinderInfo::Default, "M", manifold_ty(), prop())
}
/// `MCG : Surface → Group`
/// Mapping class group Mod(Σ) = Homeo(Σ) / Homeo₀(Σ).
pub fn mcg_ty() -> Expr {
    arrow(surface_ty(), group_ty())
}
/// `DehnTwist : Surface → Knot → MCG`
/// The Dehn twist T_c along a simple closed curve c ∈ Σ.
pub fn dehn_twist_ty() -> Expr {
    arrow(
        surface_ty(),
        arrow(knot_ty(), arrow(surface_ty(), group_ty())),
    )
}
/// `NielsenThurston : MCG → Type`
/// Nielsen-Thurston classification: periodic, reducible, or pseudo-Anosov.
pub fn nielsen_thurston_ty() -> Expr {
    arrow(group_ty(), type0())
}
/// `PseudoAnosov : MCG → Prop`
/// φ is pseudo-Anosov: preserves a pair of transverse measured foliations.
pub fn pseudo_anosov_ty() -> Expr {
    arrow(group_ty(), prop())
}
/// `DilationFactor : MCG → Nat`  (rationalised as integer × 1000)
/// The dilatation λ > 1 of a pseudo-Anosov mapping class.
pub fn dilation_factor_ty() -> Expr {
    arrow(group_ty(), nat_ty())
}
/// Birman-Hilden theorem: the hyperelliptic involution acts on MCG(Σ_g).
pub fn birman_hilden_ty() -> Expr {
    pi(BinderInfo::Default, "g", nat_ty(), prop())
}
/// `TorellGroup : Surface → Group`
/// The Torelli group I(Σ): elements acting trivially on H_1(Σ; Z).
pub fn torelli_group_ty() -> Expr {
    arrow(surface_ty(), group_ty())
}
/// `ThurstonGeometry : Type`
/// One of the eight 3-dimensional model geometries: S³, E³, H³, S²×R, H²×R,
/// SL₂(R)~, Nil, Sol.
pub fn thurston_geometry_ty() -> Expr {
    type0()
}
/// `GeometricStructure : Manifold → ThurstonGeometry → Prop`
/// M admits a complete locally homogeneous geometric structure modelled on G.
pub fn geometric_structure_ty() -> Expr {
    arrow(manifold_ty(), arrow(thurston_geometry_ty(), prop()))
}
/// Geometrization conjecture (Perelman's theorem):
/// Every closed orientable prime 3-manifold decomposes along incompressible tori
/// into geometric pieces.
pub fn geometrization_theorem_ty() -> Expr {
    pi(BinderInfo::Default, "M", manifold_ty(), prop())
}
/// Poincaré conjecture (corollary of geometrization):
/// Every closed simply-connected 3-manifold is homeomorphic to S³.
pub fn poincare_conjecture_ty() -> Expr {
    pi(BinderInfo::Default, "M", manifold_ty(), prop())
}
/// `HyperbolicVolume : Manifold → Nat`  (rational approximation × 10^6)
/// The hyperbolic volume Vol(M) of a finite-volume hyperbolic 3-manifold.
pub fn hyperbolic_volume_ty() -> Expr {
    arrow(manifold_ty(), nat_ty())
}
/// `IsHyperbolic : Manifold → Prop`
/// M admits a complete hyperbolic metric of finite volume.
pub fn is_hyperbolic_ty() -> Expr {
    arrow(manifold_ty(), prop())
}
/// Mostow-Prasad rigidity:
/// If M and N are finite-volume hyperbolic 3-manifolds with π₁(M) ≅ π₁(N),
/// then M and N are isometric.
pub fn mostow_rigidity_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "M",
        manifold_ty(),
        pi(BinderInfo::Default, "N", manifold_ty(), prop()),
    )
}
/// Wang's finiteness theorem: for any V > 0 there are finitely many
/// hyperbolic 3-manifolds of volume ≤ V.
pub fn wang_finiteness_ty() -> Expr {
    pi(BinderInfo::Default, "V", nat_ty(), prop())
}
/// `CuspedManifold : Manifold → Nat → Prop`
/// M is hyperbolic with k cusps.
pub fn cusped_manifold_ty() -> Expr {
    arrow(manifold_ty(), arrow(nat_ty(), prop()))
}
/// `FourManifold : Type`
/// A compact smooth 4-manifold.
pub fn four_manifold_ty() -> Expr {
    type0()
}
/// `IntersectionForm : FourManifold → Type`
/// The symmetric unimodular bilinear form on H₂(M; Z).
pub fn intersection_form_ty() -> Expr {
    arrow(four_manifold_ty(), type0())
}
/// `Signature : FourManifold → Int`
/// σ(M) = b₂⁺ − b₂⁻: the signature of the intersection form.
pub fn signature_4mfld_ty() -> Expr {
    arrow(four_manifold_ty(), int_ty())
}
/// `SecondBetti : FourManifold → Nat`
/// b₂(M): rank of H₂(M; Z).
pub fn second_betti_ty() -> Expr {
    arrow(four_manifold_ty(), nat_ty())
}
/// `EvenIntersectionForm : FourManifold → Prop`
/// The intersection form Q_M satisfies Q_M(x, x) ≡ 0 (mod 2) for all x.
pub fn even_intersection_form_ty() -> Expr {
    arrow(four_manifold_ty(), prop())
}
/// Donaldson's diagonalisation theorem:
/// If M is a compact simply-connected smooth 4-manifold with definite intersection form,
/// then Q_M ≅ ±I (diagonal).
pub fn donaldson_diagonalisation_ty() -> Expr {
    pi(BinderInfo::Default, "M", four_manifold_ty(), prop())
}
/// Rokhlin's theorem:
/// If M is a closed smooth spin 4-manifold, then σ(M) ≡ 0 (mod 16).
pub fn rokhlin_theorem_ty() -> Expr {
    pi(BinderInfo::Default, "M", four_manifold_ty(), prop())
}
/// `SpinManifold : FourManifold → Prop`
/// M admits a spin structure (w₂(M) = 0).
pub fn spin_manifold_ty() -> Expr {
    arrow(four_manifold_ty(), prop())
}
/// `KirbyDiagram : Type`
/// A framed link diagram in S³ encoding a 4-manifold via surgery.
pub fn kirby_diagram_ty() -> Expr {
    type0()
}
/// `KirbyMove1 : KirbyDiagram → KirbyDiagram → Prop`
/// Blow-up / blow-down: add or remove an isolated ±1-framed unknot.
pub fn kirby_move1_ty() -> Expr {
    arrow(kirby_diagram_ty(), arrow(kirby_diagram_ty(), prop()))
}
/// `KirbyMove2 : KirbyDiagram → KirbyDiagram → Prop`
/// Handle sliding: slide one 2-handle over another.
pub fn kirby_move2_ty() -> Expr {
    arrow(kirby_diagram_ty(), arrow(kirby_diagram_ty(), prop()))
}
/// Kirby's theorem: two Kirby diagrams represent diffeomorphic 4-manifolds iff
/// they are related by Kirby moves.
pub fn kirby_theorem_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "D1",
        kirby_diagram_ty(),
        pi(BinderInfo::Default, "D2", kirby_diagram_ty(), prop()),
    )
}
/// `FramedLink : Type`
/// A framed link in S³: each component has an integer framing.
pub fn framed_link_ty() -> Expr {
    type0()
}
/// `BoundaryManifold : KirbyDiagram → FourManifold`
/// The 4-manifold with boundary described by a Kirby diagram.
pub fn boundary_manifold_ty() -> Expr {
    arrow(kirby_diagram_ty(), four_manifold_ty())
}
/// `SpinCStructure : FourManifold → Type`
/// A Spin^c structure on M.
pub fn spinc_structure_ty() -> Expr {
    arrow(four_manifold_ty(), type0())
}
/// `SWBasicClass : FourManifold → Type`
/// A Spin^c structure is a SW basic class if the SW invariant is non-zero.
pub fn sw_basic_class_ty() -> Expr {
    arrow(four_manifold_ty(), type0())
}
/// `SeibergWittenInvariant : FourManifold → SpinCStructure → Int`
/// SW(M, s) ∈ Z: counts solutions to the SW equations modulo gauge.
pub fn sw_invariant_ty() -> Expr {
    arrow(four_manifold_ty(), arrow(type0(), int_ty()))
}
/// Taubes's theorem: SW(X, K_X) = ±1 for a symplectic 4-manifold X
/// (the canonical class is always a basic class).
pub fn taubes_theorem_ty() -> Expr {
    pi(BinderInfo::Default, "X", four_manifold_ty(), prop())
}
/// Witten conjecture / Donaldson-SW equivalence:
/// Donaldson invariants are equivalent to Seiberg-Witten invariants
/// (via wall-crossing formulas).
pub fn donaldson_sw_equivalence_ty() -> Expr {
    pi(BinderInfo::Default, "M", four_manifold_ty(), prop())
}
/// `HeegaardDiagram : Type`
/// A Heegaard diagram: a surface with two sets of attaching curves (α, β).
pub fn heegaard_diagram_ty() -> Expr {
    type0()
}
/// `DiagramGenus : HeegaardDiagram → Nat`
/// The genus of the Heegaard surface in a diagram.
pub fn diagram_genus_ty() -> Expr {
    arrow(heegaard_diagram_ty(), nat_ty())
}
/// `HeegaardStabilization : HeegaardDiagram → HeegaardDiagram`
/// Stabilization increases genus by 1 and adds one canceling pair of α/β curves.
pub fn heegaard_stabilization_ty() -> Expr {
    arrow(heegaard_diagram_ty(), heegaard_diagram_ty())
}
/// `DiagramReidemeisterMove : HeegaardDiagram → HeegaardDiagram → Prop`
/// Two Heegaard diagrams represent the same 3-manifold iff related by diagram
/// isotopies and handle slides (Reidemeister moves on the diagram).
pub fn diagram_reidemeister_move_ty() -> Expr {
    arrow(heegaard_diagram_ty(), arrow(heegaard_diagram_ty(), prop()))
}
/// `SurgeryExactTriangle : Manifold → Manifold → Manifold → Prop`
/// The surgery exact triangle in Heegaard Floer homology:
/// HF(M) → HF(M_0(K)) → HF(M_1(K)) → …
pub fn surgery_exact_triangle_ty() -> Expr {
    arrow(
        manifold_ty(),
        arrow(manifold_ty(), arrow(manifold_ty(), prop())),
    )
}
/// `SurgeryDescription : Manifold → Type`
/// A surgery description of a 3-manifold as surgery on a framed link in S³.
pub fn surgery_description_ty() -> Expr {
    arrow(manifold_ty(), type0())
}
/// `LinkSurgery : Nat → Manifold`
/// The 3-manifold obtained by surgery on an n-component framed link.
pub fn link_surgery_ty() -> Expr {
    arrow(nat_ty(), manifold_ty())
}
/// `KnotConcordance : Knot → Knot → Prop`
/// Two knots K₀ and K₁ are concordant if there is a smoothly embedded annulus
/// in S³ × [0,1] with boundary K₀ × {0} ∪ K₁ × {1}.
pub fn knot_concordance_ty() -> Expr {
    arrow(knot_ty(), arrow(knot_ty(), prop()))
}
/// `ConcordanceGroup : Type`
/// The smooth concordance group C of knots modulo concordance, under connected sum.
pub fn concordance_group_ty() -> Expr {
    group_ty()
}
/// `SliceKnot : Knot → Prop`
/// K is slice if it bounds a smoothly embedded disk in B⁴.
pub fn slice_knot_ty() -> Expr {
    arrow(knot_ty(), prop())
}
/// `SmoothSliceKnot : Knot → Prop`
/// K bounds a smoothly embedded disk in B⁴.
pub fn smooth_slice_knot_ty() -> Expr {
    arrow(knot_ty(), prop())
}
/// `TopologicalSliceKnot : Knot → Prop`
/// K bounds a locally flat topologically embedded disk in B⁴.
pub fn topological_slice_knot_ty() -> Expr {
    arrow(knot_ty(), prop())
}
/// Slice-ribbon conjecture: every slice knot is ribbon.
/// (Fox's conjecture, still open in general.)
pub fn slice_ribbon_conjecture_ty() -> Expr {
    pi(BinderInfo::Default, "K", knot_ty(), prop())
}
/// `KhovanovHomology : Knot → Nat → Int → Type`
/// Kh^{i,j}(K): the (i,j)-bigraded Khovanov chain group.
pub fn khovanov_homology_ty() -> Expr {
    arrow(knot_ty(), arrow(nat_ty(), arrow(int_ty(), type0())))
}
/// `CubeOfResolutions : Knot → Type`
/// The cube of resolutions: for each crossing choose 0- or 1-smoothing.
pub fn cube_of_resolutions_ty() -> Expr {
    arrow(knot_ty(), type0())
}
/// `RasmussenInvariant : Knot → Int`
/// The Rasmussen s-invariant: a concordance homomorphism derived from
/// Khovanov homology; |s(K)| ≤ 2 g_4(K).
pub fn rasmussen_invariant_ty() -> Expr {
    arrow(knot_ty(), int_ty())
}
/// `KnotFloerComplex : Knot → Type`
/// The knot Floer chain complex CFK^∞(K) over F_2.
pub fn knot_floer_complex_ty() -> Expr {
    arrow(knot_ty(), type0())
}
/// `TauInvariant : Knot → Int`
/// The tau invariant τ(K): a concordance homomorphism from knot Floer homology;
/// τ(K) ≤ g_4(K).
pub fn tau_invariant_ty() -> Expr {
    arrow(knot_ty(), int_ty())
}
/// `AlexanderPolynomial : Knot → Type`
/// Δ_K(t) ∈ Z[t, t⁻¹]: the Alexander polynomial of K, computed from HFK.
pub fn alexander_polynomial_ty() -> Expr {
    arrow(knot_ty(), type0())
}
/// `KnotFloerHomology : Knot → Int → Int → Type`
/// HFK^(K, s): the knot Floer homology group in Alexander grading s.
pub fn knot_floer_homology_ty() -> Expr {
    arrow(knot_ty(), arrow(int_ty(), arrow(int_ty(), type0())))
}
/// `HFHat : Manifold → Type`
/// HF_hat(Y): the hat version of Heegaard Floer homology.
pub fn hf_hat_ty() -> Expr {
    arrow(manifold_ty(), type0())
}
/// `HFPlus : Manifold → Type`
/// HF⁺(Y): the plus version (uses chain complex over F_2[U]).
pub fn hf_plus_ty() -> Expr {
    arrow(manifold_ty(), type0())
}
/// `HFMinus : Manifold → Type`
/// HF⁻(Y): the minus version.
pub fn hf_minus_ty() -> Expr {
    arrow(manifold_ty(), type0())
}
/// `HFInfinity : Manifold → Type`
/// HF∞(Y): the infinity version.
pub fn hf_infinity_ty() -> Expr {
    arrow(manifold_ty(), type0())
}
/// `HFExactTriangle : Manifold → Manifold → Manifold → Prop`
/// The Heegaard Floer surgery exact triangle relating three manifolds obtained
/// by 0, +1, −1 surgeries.
pub fn hf_exact_triangle_ty() -> Expr {
    arrow(
        manifold_ty(),
        arrow(manifold_ty(), arrow(manifold_ty(), prop())),
    )
}
/// Freedman's classification theorem (1982):
/// Closed simply-connected topological 4-manifolds are classified up to homeomorphism
/// by their intersection form and Kirby-Siebenmann invariant.
pub fn freedman_classification_ty() -> Expr {
    pi(BinderInfo::Default, "M", four_manifold_ty(), prop())
}
/// `DonaldsonPolynomial : FourManifold → Int → Int`
/// Donaldson polynomial invariant D_k(M) counting ASD instantons of charge k.
pub fn donaldson_polynomial_ty() -> Expr {
    arrow(four_manifold_ty(), arrow(int_ty(), int_ty()))
}
/// `ExoticR4 : Type`
/// Exotic smooth structures on R⁴: uncountably many pairwise non-diffeomorphic
/// smooth structures on the underlying topological space R⁴.
pub fn exotic_r4_ty() -> Expr {
    type0()
}
/// `HyperbolicStructure : Manifold → Prop`
/// M admits a complete hyperbolic structure (i.e., is locally isometric to H³).
pub fn hyperbolic_structure_ty() -> Expr {
    arrow(manifold_ty(), prop())
}
/// Thurston's geometrization theorem (proved by Perelman via Ricci flow):
/// Every compact orientable 3-manifold admits a geometric decomposition.
pub fn thurston_geometrization_full_ty() -> Expr {
    pi(BinderInfo::Default, "M", manifold_ty(), prop())
}
/// Volume conjecture:
/// lim_{N→∞} (2π/N) log |J_N(K; e^{2πi/N})| = Vol(S³ \ K)
/// where J_N is the N-th colored Jones polynomial.
pub fn vol_conj_ty() -> Expr {
    pi(BinderInfo::Default, "K", knot_ty(), prop())
}
/// `MappingClassGroupPresentation : Surface → Type`
/// A finite presentation of MCG(Σ_g) by Dehn twists with Humphries generators.
pub fn mcg_presentation_ty() -> Expr {
    arrow(surface_ty(), type0())
}
/// `DehnTwistRelation : Surface → Knot → Knot → Prop`
/// Braid relation between Dehn twists: T_a T_b T_a = T_b T_a T_b when i(a,b)=1.
pub fn dehn_twist_relation_ty() -> Expr {
    arrow(surface_ty(), arrow(knot_ty(), arrow(knot_ty(), prop())))
}
/// `NielsenThurstonExtended : Group → Nat → Prop`
/// Extended Nielsen-Thurston: the dilatation of a pseudo-Anosov element of
/// MCG(Σ_g) is an algebraic integer of degree ≤ 6g−6.
pub fn nielsen_thurston_extended_ty() -> Expr {
    arrow(group_ty(), arrow(nat_ty(), prop()))
}
/// `RibbonSurface : Knot → Type`
/// A ribbon surface bounded by K: an immersed disk in S³ with only ribbon singularities.
pub fn ribbon_surface_ty() -> Expr {
    arrow(knot_ty(), type0())
}
/// `RibbonSingularity : Type`
/// A ribbon singularity: a self-intersection arc whose preimage consists of
/// an interior arc and a boundary arc (the ribbon move).
pub fn ribbon_singularity_ty() -> Expr {
    type0()
}
/// `ReshetikhinTuraevInvariant : Manifold → Nat → Int`
/// The Reshetikhin-Turaev invariant RT_r(M) at level r, derived from the
/// quantum group U_q(sl_2) at q = e^{2πi/r}.
pub fn reshetikhin_turaev_ty() -> Expr {
    arrow(manifold_ty(), arrow(nat_ty(), int_ty()))
}
/// `WRTInvariant : Manifold → Nat → Int`
/// The Witten-Reshetikhin-Turaev invariant WRT_r(M): the r-th WRT invariant
/// of the closed 3-manifold M.
pub fn wrt_invariant_ty() -> Expr {
    arrow(manifold_ty(), arrow(nat_ty(), int_ty()))
}
/// `TQFTFunctor : Type`
/// A (2+1)-dimensional TQFT: a symmetric monoidal functor from the category
/// of cobordisms to the category of vector spaces.
pub fn tqft_functor_ty() -> Expr {
    type0()
}
/// `VirtualKnot : Type`
/// A virtual knot: an equivalence class of Gauss diagrams under virtual
/// Reidemeister moves (classical + virtual + mixed).
pub fn virtual_knot_ty() -> Expr {
    type0()
}
/// `VirtualCrossing : Type`
/// A virtual crossing: a marked self-intersection that is not a classical
/// over/under crossing (drawn as a circled crossing).
pub fn virtual_crossing_ty() -> Expr {
    type0()
}
/// `GaussDiagram : VirtualKnot → Type`
/// The Gauss diagram of a virtual knot: a circle with signed chords
/// corresponding to classical crossings.
pub fn gauss_diagram_ty() -> Expr {
    arrow(virtual_knot_ty(), type0())
}
/// `KauffmanVirtualPolynomial : VirtualKnot → Type`
/// The Kauffman bracket polynomial extended to virtual knots via the
/// state-sum model with virtual crossings.
pub fn kauffman_virtual_ty() -> Expr {
    arrow(virtual_knot_ty(), type0())
}
/// Populate an `Environment` with all low-dimensional topology axioms.
pub fn build_low_dimensional_topology_env(env: &mut Environment) {
    let axioms: &[(&str, Expr)] = &[
        ("Manifold", type0()),
        ("Surface", type0()),
        ("Knot", type0()),
        ("Ring", type0()),
        ("Group", type0()),
        ("ClosedSurface", closed_surface_ty()),
        ("SurfaceGenus", surface_genus_ty()),
        ("IsOrientable", is_orientable_ty()),
        (
            "EulerCharacteristicSurface",
            euler_characteristic_surface_ty(),
        ),
        ("SurfaceClassification", surface_classification_ty()),
        ("ConnectedSumSurface", connected_sum_surface_ty()),
        ("MappingTorus", mapping_torus_ty()),
        ("HandlebodyGenus", handlebody_genus_ty()),
        ("HeegaardSplitting", heegaard_splitting_ty()),
        ("HeegaardGenus", heegaard_genus_ty()),
        ("ReidemeisterSinger", reidemeister_singer_ty()),
        ("Irreducible3Manifold", irreducible_3manifold_ty()),
        ("KneserMilnor", kneser_milnor_ty()),
        ("JSJDecomposition", jsj_decomposition_ty()),
        ("DehnSurgery", dehn_surgery_ty()),
        ("SurgeryCoefficient", surgery_coefficient_ty()),
        ("LickorishWallace", lickorish_wallace_ty()),
        ("DehnFilling", dehn_filling_ty()),
        ("ThurstonDehnSurgery", thurston_dehn_surgery_ty()),
        ("MCG", mcg_ty()),
        ("DehnTwist", dehn_twist_ty()),
        ("NielsenThurston", nielsen_thurston_ty()),
        ("PseudoAnosov", pseudo_anosov_ty()),
        ("DilationFactor", dilation_factor_ty()),
        ("BirmanHilden", birman_hilden_ty()),
        ("TorelliGroup", torelli_group_ty()),
        ("ThurstonGeometry", thurston_geometry_ty()),
        ("GeometricStructure", geometric_structure_ty()),
        ("GeometrizationTheorem", geometrization_theorem_ty()),
        ("PoincareConjecture", poincare_conjecture_ty()),
        ("HyperbolicVolume", hyperbolic_volume_ty()),
        ("IsHyperbolic", is_hyperbolic_ty()),
        ("MostowRigidity", mostow_rigidity_ty()),
        ("WangFiniteness", wang_finiteness_ty()),
        ("CuspedManifold", cusped_manifold_ty()),
        ("FourManifold", four_manifold_ty()),
        ("IntersectionForm", intersection_form_ty()),
        ("Signature4Mfld", signature_4mfld_ty()),
        ("SecondBetti", second_betti_ty()),
        ("EvenIntersectionForm", even_intersection_form_ty()),
        ("DonaldsonDiagonalisation", donaldson_diagonalisation_ty()),
        ("RokhlinTheorem", rokhlin_theorem_ty()),
        ("SpinManifold", spin_manifold_ty()),
        ("KirbyDiagram", kirby_diagram_ty()),
        ("KirbyMove1", kirby_move1_ty()),
        ("KirbyMove2", kirby_move2_ty()),
        ("KirbyTheorem", kirby_theorem_ty()),
        ("FramedLink", framed_link_ty()),
        ("BoundaryManifold", boundary_manifold_ty()),
        ("SpinCStructure", spinc_structure_ty()),
        ("SWBasicClass", sw_basic_class_ty()),
        ("SeibergWittenInvariant", sw_invariant_ty()),
        ("TaubesTheorem", taubes_theorem_ty()),
        ("DonaldsonSWEquivalence", donaldson_sw_equivalence_ty()),
        ("HeegaardDiagram", heegaard_diagram_ty()),
        ("DiagramGenus", diagram_genus_ty()),
        ("HeegaardStabilization", heegaard_stabilization_ty()),
        ("DiagramReidemeisterMove", diagram_reidemeister_move_ty()),
        ("SurgeryExactTriangle", surgery_exact_triangle_ty()),
        ("SurgeryDescription", surgery_description_ty()),
        ("LinkSurgery", link_surgery_ty()),
        ("KnotConcordance", knot_concordance_ty()),
        ("ConcordanceGroup", concordance_group_ty()),
        ("SliceKnot", slice_knot_ty()),
        ("SmoothSliceKnot", smooth_slice_knot_ty()),
        ("TopologicalSliceKnot", topological_slice_knot_ty()),
        ("SliceRibbonConjecture", slice_ribbon_conjecture_ty()),
        ("KhovanovHomology", khovanov_homology_ty()),
        ("CubeOfResolutions", cube_of_resolutions_ty()),
        ("RasmussenInvariant", rasmussen_invariant_ty()),
        ("KnotFloerComplex", knot_floer_complex_ty()),
        ("TauInvariant", tau_invariant_ty()),
        ("AlexanderPolynomial", alexander_polynomial_ty()),
        ("KnotFloerHomology", knot_floer_homology_ty()),
        ("HFHat", hf_hat_ty()),
        ("HFPlus", hf_plus_ty()),
        ("HFMinus", hf_minus_ty()),
        ("HFInfinity", hf_infinity_ty()),
        ("HFExactTriangle", hf_exact_triangle_ty()),
        ("FreedmanClassification", freedman_classification_ty()),
        ("DonaldsonPolynomial", donaldson_polynomial_ty()),
        ("ExoticR4", exotic_r4_ty()),
        ("HyperbolicStructure", hyperbolic_structure_ty()),
        (
            "ThurstonGeometrizationFull",
            thurston_geometrization_full_ty(),
        ),
        ("VolConj", vol_conj_ty()),
        ("MappingClassGroupPresentation", mcg_presentation_ty()),
        ("DehnTwistRelation", dehn_twist_relation_ty()),
        ("NielsenThurstonExtended", nielsen_thurston_extended_ty()),
        ("RibbonSurface", ribbon_surface_ty()),
        ("RibbonSingularity", ribbon_singularity_ty()),
        ("ReshetikhinTuraevInvariant", reshetikhin_turaev_ty()),
        ("WRTInvariant", wrt_invariant_ty()),
        ("TQFTFunctor", tqft_functor_ty()),
        ("VirtualKnot", virtual_knot_ty()),
        ("VirtualCrossing", virtual_crossing_ty()),
        ("GaussDiagram", gauss_diagram_ty()),
        ("KauffmanVirtualPolynomial", kauffman_virtual_ty()),
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
mod tests {
    use super::*;
    #[test]
    fn test_build_env_nonempty() {
        let mut env = Environment::new();
        build_low_dimensional_topology_env(&mut env);
        assert!(env.get(&Name::str("Manifold")).is_some());
        assert!(env.get(&Name::str("HeegaardSplitting")).is_some());
        assert!(env.get(&Name::str("GeometrizationTheorem")).is_some());
        assert!(env.get(&Name::str("KirbyTheorem")).is_some());
        assert!(env.get(&Name::str("DonaldsonDiagonalisation")).is_some());
    }
    #[test]
    fn test_surface_euler_characteristic() {
        assert_eq!(SurfaceType::Sphere.euler_characteristic(), 2);
        assert_eq!(SurfaceType::OrientableSurface(1).euler_characteristic(), 0);
        assert_eq!(SurfaceType::OrientableSurface(2).euler_characteristic(), -2);
        assert_eq!(
            SurfaceType::NonOrientableSurface(1).euler_characteristic(),
            1
        );
        assert_eq!(
            SurfaceType::NonOrientableSurface(2).euler_characteristic(),
            0
        );
    }
    #[test]
    fn test_surface_connected_sum() {
        let t2 = SurfaceType::OrientableSurface(1);
        let t2b = SurfaceType::OrientableSurface(1);
        let s2g2 = t2.connected_sum(&t2b);
        assert_eq!(s2g2, SurfaceType::OrientableSurface(2));
        let sphere = SurfaceType::Sphere;
        assert_eq!(sphere.connected_sum(&t2), SurfaceType::OrientableSurface(1));
    }
    #[test]
    fn test_thurston_geometry_names() {
        let geoms = [
            ThurstonGeometryKind::Spherical,
            ThurstonGeometryKind::Euclidean,
            ThurstonGeometryKind::Hyperbolic,
            ThurstonGeometryKind::S2xR,
            ThurstonGeometryKind::H2xR,
            ThurstonGeometryKind::SL2R,
            ThurstonGeometryKind::Nil,
            ThurstonGeometryKind::Sol,
        ];
        assert_eq!(geoms.len(), 8);
        assert!(ThurstonGeometryKind::Hyperbolic.is_constant_curvature());
        assert!(!ThurstonGeometryKind::Nil.is_constant_curvature());
    }
    #[test]
    fn test_intersection_form_e8() {
        let e8 = IntersectionFormData::e8();
        assert_eq!(e8.rank, 8);
        assert!(e8.is_even());
        assert_eq!(e8.matrix[0][0], 2);
    }
    #[test]
    fn test_intersection_form_hyperbolic() {
        let h = IntersectionFormData::hyperbolic();
        assert_eq!(h.rank, 2);
        assert!(h.is_even());
        assert_eq!(h.signature_diagonal(), 0);
        assert!(!h.is_definite());
    }
    #[test]
    fn test_kirby_diagram_cp2() {
        let d = KirbyDiagramData::cp2();
        assert_eq!(d.components.len(), 1);
        assert_eq!(d.components[0].framing, 1);
        let m = d.intersection_matrix();
        assert_eq!(m[0][0], 1);
    }
    #[test]
    fn test_heegaard_genus_lens_space() {
        let ls = HeegaardSplittingData::lens_space(5, 2);
        assert_eq!(ls.genus, 1);
        assert!(ls.strongly_irreducible);
    }
    #[test]
    fn test_surgery_spec_label() {
        let s = SurgerySpec::integer("trefoil", 1);
        let label = s.label();
        assert!(label.contains("trefoil"));
        assert!(label.contains("1/1"));
    }
}
#[cfg(test)]
mod new_impl_tests {
    use super::*;
    #[test]
    fn test_heegaard_diagram_stabilize() {
        let d = HeegaardDiagramSimplifier::new(2);
        assert_eq!(d.genus, 2);
        let d2 = d.stabilize();
        assert_eq!(d2.genus, 3);
        assert!(d2.stabilized);
        let d3 = d2.destabilize().expect("destabilize should succeed");
        assert_eq!(d3.genus, 2);
    }
    #[test]
    fn test_heegaard_diagram_destabilize_genus_zero() {
        let d = HeegaardDiagramSimplifier::new(0);
        assert!(d.destabilize().is_none());
    }
    #[test]
    fn test_khovanov_cube_dimension() {
        let kc = KhovanovHomologyComputer::new(3, -3);
        assert_eq!(kc.cube_dimension(), 8);
        assert_eq!(KhovanovHomologyComputer::unknot_rank(), 2);
        let ranks = KhovanovHomologyComputer::trefoil_ranks();
        assert!(!ranks.is_empty());
    }
    #[test]
    fn test_dehn_surgery_computer() {
        let s = DehnSurgeryComputer::integer_surgery("unknot", 5);
        let name = s.result_manifold_name();
        assert!(name.contains("lens"));
        assert_eq!(s.slope_label(), "5");
        assert_eq!(s.first_homology(), "Z/5");
    }
    #[test]
    fn test_knot_concordance_checker() {
        let checker = KnotConcordanceChecker::new("3_1", -1, -2, -2);
        assert!(!checker.is_potentially_slice());
        assert!(checker.four_ball_genus_lower_bound() >= 1);
        let unknot = KnotConcordanceChecker::new("unknot", 0, 0, 0);
        assert!(unknot.is_potentially_slice());
    }
    #[test]
    fn test_wrt_invariant_computer() {
        let wrt = WRTInvariantComputer::new("S^3", 5);
        assert_eq!(wrt.modular_category_rank(), 4);
        let desc = wrt.wrt_invariant_description();
        assert!(desc.contains("WRT_5"));
        let poincare = WRTInvariantComputer::new("Poincare", 5);
        assert!(poincare.poincare_sphere_note().is_some());
    }
    #[test]
    fn test_build_env_new_axioms() {
        let mut env = Environment::new();
        build_low_dimensional_topology_env(&mut env);
        assert!(env.get(&Name::str("HeegaardDiagram")).is_some());
        assert!(env.get(&Name::str("KhovanovHomology")).is_some());
        assert!(env.get(&Name::str("TauInvariant")).is_some());
        assert!(env.get(&Name::str("HFHat")).is_some());
        assert!(env.get(&Name::str("WRTInvariant")).is_some());
        assert!(env.get(&Name::str("VirtualKnot")).is_some());
        assert!(env.get(&Name::str("RasmussenInvariant")).is_some());
        assert!(env.get(&Name::str("SliceKnot")).is_some());
        assert!(env.get(&Name::str("FreedmanClassification")).is_some());
    }
}
/// Alias for `build_low_dimensional_topology_env` so callers can use `build_env`.
pub fn build_env(env: &mut Environment) {
    build_low_dimensional_topology_env(env);
}
#[cfg(test)]
mod tests_low_dim_topo_ext {
    use super::*;
    #[test]
    fn test_thurston_geometrization() {
        let mut tg = ThurstonGeometrizationData::new("M");
        tg.add_piece(ThurstonGeometry::Hyperbolic, "M is hyperbolic");
        assert!(tg.is_hyperbolic);
        assert!(tg.hyperbolic_volume_lower_bound() > 2.0);
        assert!(tg.perelman_theorem().contains("Perelman"));
    }
    #[test]
    fn test_heegaard_splitting() {
        let hs = HeegaardSplitting2V2::new("S^3", 0);
        assert!(hs.waldhausen_s3());
        assert!(hs.genus1_classification().is_none());
        let lens = HeegaardSplitting2V2::new("L(p,q)", 1).strongly_irreducible();
        assert!(lens.strongly_irreducible);
        assert!(lens.genus1_classification().is_some());
    }
    #[test]
    fn test_dehn_surgery() {
        let ds = DehnSurgery::new("trefoil", 1, 0, "S^3");
        assert!(ds.coefficient().is_infinite());
        let ds2 = DehnSurgery::new("trefoil", 5, 1, "lens space");
        assert!((ds2.coefficient() - 5.0).abs() < 1e-10);
        assert!(ds2.is_integral());
        assert!(ds2.lickorish_wallace().contains("Lickorish"));
    }
}
#[cfg(test)]
mod tests_low_dim_topo_ext2 {
    use super::*;
    #[test]
    fn test_reidemeister_moves() {
        let r1 = ReidemeisterMove::R1;
        assert!(r1.preserves_knot_type());
        assert!(!r1.preserves_writhe());
        let r2 = ReidemeisterMove::R2;
        assert!(r2.preserves_writhe());
        let r3 = ReidemeisterMove::R3;
        assert!(r3.description().contains("triangle"));
    }
    #[test]
    fn test_knot_diagram() {
        let tr = KnotDiagram::trefoil_right();
        assert_eq!(tr.crossing_number, 3);
        assert_eq!(tr.writhe, 3);
        assert!(tr.is_alternating);
        assert!((tr.jones_at_one() - 1.0).abs() < 1e-10);
        let fig8 = KnotDiagram::figure_eight();
        assert_eq!(fig8.writhe, 0);
        assert!((fig8.kauffman_bracket_approx() - 1.0).abs() < 1e-10);
    }
}
