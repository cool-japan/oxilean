//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{
    NewtonPolytope, NewtonPolytopeExt, RegularSubdivision, TropicalAbelianVariety,
    TropicalCurveExt2, TropicalFan, TropicalGrassmannianExt, TropicalHypersurface,
    TropicalHypersurfaceExt, TropicalLinearProgram, TropicalModuliM0n, TropicalPolynomial,
    TropicalVariety, ValuatedMatroid, Valuation,
};

/// Returns the Minkowski sum of two Newton polytopes.
///
/// The Newton polytope of a product `f · g` equals the Minkowski sum of
/// the Newton polytopes of `f` and `g`.
pub fn newton_polytope_of_product(p: &NewtonPolytope, q: &NewtonPolytope) -> NewtonPolytope {
    debug_assert_eq!(
        p.dimension, q.dimension,
        "dimension mismatch in Minkowski sum"
    );
    let dim = p.dimension;
    let mut result = NewtonPolytope::new(dim);
    for vp in &p.vertices {
        for vq in &q.vertices {
            let v: Vec<i32> = vp.iter().zip(vq.iter()).map(|(a, b)| a + b).collect();
            result.vertices.push(v);
        }
    }
    result
}
/// Greatest common divisor of two `usize` values.
pub(super) fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a.max(1)
}
/// Computes the binomial coefficient C(n, k).
pub(super) fn binomial(n: usize, k: usize) -> usize {
    if k > n {
        return 0;
    }
    let k = k.min(n - k);
    let mut result = 1usize;
    for i in 0..k {
        result = result * (n - i) / (i + 1);
    }
    result
}
/// Returns the standard list of valuation axioms.
///
/// A valuation `v : K* → Γ` satisfies:
/// 1. `v(ab) = v(a) + v(b)` (multiplicativity)
/// 2. `v(a + b) ≥ min(v(a), v(b))` (ultrametric triangle inequality)
/// 3. `v(0) = +∞` and `v(1) = 0` (normalisation)
pub fn valuation_axioms() -> Vec<&'static str> {
    vec![
        "v(a·b) = v(a) + v(b)  (multiplicativity / homomorphism to value group)",
        "v(a + b) ≥ min(v(a), v(b))  (ultrametric triangle inequality)",
        "v(0) = +∞  (zero maps to +∞)",
        "v(1) = 0   (one maps to group identity)",
    ]
}
pub fn tg_app(f: Expr, a: Expr) -> Expr {
    Expr::App(Box::new(f), Box::new(a))
}
pub fn tg_app2(f: Expr, a: Expr, b: Expr) -> Expr {
    tg_app(tg_app(f, a), b)
}
pub fn tg_cst(s: &str) -> Expr {
    Expr::Const(Name::str(s), vec![])
}
pub fn tg_prop() -> Expr {
    Expr::Sort(Level::zero())
}
pub fn tg_type0() -> Expr {
    Expr::Sort(Level::succ(Level::zero()))
}
pub fn tg_pi(bi: BinderInfo, name: &str, dom: Expr, body: Expr) -> Expr {
    Expr::Pi(bi, Name::str(name), Box::new(dom), Box::new(body))
}
pub fn tg_arrow(a: Expr, b: Expr) -> Expr {
    tg_pi(BinderInfo::Default, "_", a, b)
}
pub fn tg_nat() -> Expr {
    tg_cst("Nat")
}
pub fn tg_real() -> Expr {
    tg_cst("Real")
}
pub fn tg_int() -> Expr {
    tg_cst("Int")
}
/// MaxPlusSemiring type: (ℝ ∪ {−∞}, max, +).
/// Dual to the min-plus semiring via negation.
pub fn max_plus_semiring_ty() -> Expr {
    tg_type0()
}
/// Tropical semiring homomorphism: TropicalHom (S T : TropSemiring) : Type.
/// A semiring map preserving ⊕ and ⊗.
pub fn tropical_hom_ty() -> Expr {
    tg_arrow(tg_type0(), tg_arrow(tg_type0(), tg_type0()))
}
/// Tropical idempotent semiring law: tropical_idemp_law : Prop.
/// In any tropical semiring a ⊕ a = a.
pub fn tropical_idemp_law_ty() -> Expr {
    tg_prop()
}
/// TropicalPolynomial type: TropPoly (n : Nat) : Type.
/// A tropical polynomial in n variables.
pub fn trop_poly_ty() -> Expr {
    tg_arrow(tg_nat(), tg_type0())
}
/// TropicalHypersurface type: TropHypersurface (f : TropPoly n) : Type.
/// The non-smooth locus of f — a pure polyhedral complex of codimension 1.
pub fn trop_hypersurface_ty() -> Expr {
    tg_arrow(tg_type0(), tg_type0())
}
/// Tropical hypersurface duality: trop_dual_subdivision : Prop.
/// V(f) is combinatorially dual to a regular subdivision of the Newton polytope of f.
pub fn trop_dual_subdivision_ty() -> Expr {
    tg_prop()
}
/// Tropical Nullstellensatz: trop_nullstellensatz : Prop.
/// A tropical polynomial system has no solution iff a certificate exists in the ideal.
pub fn trop_nullstellensatz_ty() -> Expr {
    tg_prop()
}
/// TropicalVariety type: TropVariety (I : Ideal) : Type.
pub fn trop_variety_ty() -> Expr {
    tg_arrow(tg_type0(), tg_type0())
}
/// Fundamental theorem of tropical geometry: fund_thm_trop_geom : Prop.
/// Trop(V(I)) equals the tropical variety of the tropicalized ideal.
pub fn fund_thm_trop_geom_ty() -> Expr {
    tg_prop()
}
/// Tropical Bezout theorem: trop_bezout : (d1 d2 : Nat) → Prop.
/// Two tropical plane curves of degrees d1 and d2 intersect in d1 * d2 points (counted with multiplicity).
pub fn trop_bezout_ty() -> Expr {
    tg_arrow(tg_nat(), tg_arrow(tg_nat(), tg_prop()))
}
/// Balancing condition: trop_balancing : Prop.
/// Every tropical variety satisfies the balancing condition at each codimension-1 cell.
pub fn trop_balancing_ty() -> Expr {
    tg_prop()
}
/// SpeyerSturmfelsTropGr type: TropGr (k n : Nat) : Type.
/// The tropical Grassmannian of k-planes in n-space, a polyhedral fan in ℝ^C(n,k).
pub fn speyer_sturmfels_trop_gr_ty() -> Expr {
    tg_arrow(tg_nat(), tg_arrow(tg_nat(), tg_type0()))
}
/// Tropical Plücker relations: trop_plucker : Prop.
/// Points of TropGr(k,n) satisfy the tropical three-term Plücker relations.
pub fn trop_plucker_ty() -> Expr {
    tg_prop()
}
/// Tropical flag variety: TropFlagVariety (dims : List Nat) : Type.
/// Parameterises partial flags 0 ⊂ V₁ ⊂ … ⊂ Vₙ in tropical projective space.
pub fn trop_flag_variety_ty() -> Expr {
    tg_arrow(tg_type0(), tg_type0())
}
/// BergmanFan type: BergmanFan (M : Matroid) : Type.
/// The Bergman fan of M — the tropical linear space associated with M.
pub fn bergman_fan_ty() -> Expr {
    tg_arrow(tg_type0(), tg_type0())
}
/// Tropical linear space axiom: trop_linear_space_matroid : Prop.
/// Every tropical linear space is the Bergman fan of a matroid.
pub fn trop_linear_space_matroid_ty() -> Expr {
    tg_prop()
}
/// TropConvexSet type: TropConvexSet (n : Nat) : Type.
/// A tropically convex subset of ℝⁿ.
pub fn trop_convex_set_ty() -> Expr {
    tg_arrow(tg_nat(), tg_type0())
}
/// Tropical convex hull type: TropConvHull (S : Set ℝⁿ) : Type.
pub fn trop_conv_hull_ty() -> Expr {
    tg_arrow(tg_type0(), tg_type0())
}
/// Tropical convexity Helly theorem: trop_helly : Prop.
/// For n+1 tropically convex sets in ℝⁿ, if every n sets have a common point so do all n+1.
pub fn trop_helly_ty() -> Expr {
    tg_prop()
}
/// TropEigenvalue type: TropEigenvalue (A : TropMatrix n) : Real.
/// The largest max-plus eigenvalue of A, equal to the max-weight cycle mean.
pub fn trop_eigenvalue_ty() -> Expr {
    tg_arrow(tg_type0(), tg_real())
}
/// Tropical Perron–Frobenius theorem: trop_perron_frobenius : Prop.
/// An irreducible tropical matrix has a unique max-plus eigenvalue equal to the max-cycle-mean.
pub fn trop_perron_frobenius_ty() -> Expr {
    tg_prop()
}
/// Tropical matrix powers: TropMatPow (A : TropMatrix n) (k : Nat) : TropMatrix n.
pub fn trop_mat_pow_ty() -> Expr {
    tg_arrow(tg_type0(), tg_arrow(tg_nat(), tg_type0()))
}
/// NewtonOkounkovBody type: NOBody (X : Variety) (L : LineBundle) : Type.
/// A convex body in ℝ^{dim X} encoding asymptotic properties of sections of L.
pub fn newton_okounkov_body_ty() -> Expr {
    tg_arrow(tg_type0(), tg_arrow(tg_type0(), tg_type0()))
}
/// Newton–Okounkov body volume: no_body_volume : Prop.
/// The Euclidean volume of the NO-body of L equals (1/n!) times the degree of L.
pub fn no_body_volume_ty() -> Expr {
    tg_prop()
}
/// GroebnerFan type: GroebnerFan (I : Ideal) : Type.
/// The normal fan of the state polytope; parameterises initial ideals of I.
pub fn groebner_fan_ty() -> Expr {
    tg_arrow(tg_type0(), tg_type0())
}
/// InitialDegeneration type: InitialDegeneration (X : Variety) (w : Weight) : Type.
/// The flat limit of X along a one-parameter subgroup determined by w.
pub fn initial_degeneration_ty() -> Expr {
    tg_arrow(tg_type0(), tg_arrow(tg_type0(), tg_type0()))
}
/// Tropical compactification: trop_compactification : Prop.
/// A variety X in a torus has a tropical compactification whose boundary is simple normal crossing.
pub fn trop_compactification_ty() -> Expr {
    tg_prop()
}
/// Amoeba type: Amoeba (V : Variety) : Type.
/// The image of V ⊆ (ℂ*)ⁿ under the coordinate-wise log-absolute-value map.
pub fn amoeba_ty() -> Expr {
    tg_arrow(tg_type0(), tg_type0())
}
/// Amoeba spine: AmoebaSpine (A : Amoeba) : Type.
/// The deformation retract of the amoeba onto a tropical variety (the spine).
pub fn amoeba_spine_ty() -> Expr {
    tg_arrow(tg_type0(), tg_type0())
}
/// Tropical limit of amoeba: trop_amoeba_limit : Prop.
/// As t → 0 the amoeba of Vₜ under Log_t converges to the tropical variety Trop(V).
pub fn trop_amoeba_limit_ty() -> Expr {
    tg_prop()
}
/// TropicalizationMap type: TropMap (K : NonArchField) : Type.
/// The valuation map val : (K*)ⁿ → ℝⁿ sending x ↦ (−val(x₁), …, −val(xₙ)).
pub fn tropicalization_map_ty() -> Expr {
    tg_arrow(tg_type0(), tg_type0())
}
/// Tropicalization functor: trop_functor : Prop.
/// Tropicalization is functorial: morphisms of varieties induce piecewise-linear maps.
pub fn trop_functor_ty() -> Expr {
    tg_prop()
}
/// BerkovichSpace type: BerkovichSpace (X : AlgVariety) : Type.
/// The Berkovich analytification of X over a non-archimedean field.
pub fn berkovich_space_ty() -> Expr {
    tg_arrow(tg_type0(), tg_type0())
}
/// BerkovichSkeleton type: BerkovichSkeleton (X : BerkovichSpace) : Type.
/// The canonical piecewise-linear subspace (skeleton) of the Berkovich space.
pub fn berkovich_skeleton_ty() -> Expr {
    tg_arrow(tg_type0(), tg_type0())
}
/// Berkovich retraction: berkovich_retraction : Prop.
/// There is a strong deformation retraction of Xᵃⁿ onto its skeleton.
pub fn berkovich_retraction_ty() -> Expr {
    tg_prop()
}
/// NonArchimedeanField type: NonArchField : Type.
pub fn non_arch_field_ty() -> Expr {
    tg_type0()
}
/// Ultrametric inequality: ultrametric_ineq : Prop.
/// In a non-archimedean field |a + b| ≤ max(|a|, |b|).
pub fn ultrametric_ineq_ty() -> Expr {
    tg_prop()
}
/// Non-archimedean absolute value: NonArchAbsValue (K : Field) : Type.
pub fn non_arch_abs_value_ty() -> Expr {
    tg_arrow(tg_type0(), tg_type0())
}
/// TropicalCurveType type: TropCurveType (d g : Nat) : Type.
/// A tropical curve of degree d and genus g in ℝ².
pub fn trop_curve_type_ty() -> Expr {
    tg_arrow(tg_nat(), tg_arrow(tg_nat(), tg_type0()))
}
/// Tropical Riemann–Hurwitz formula: trop_riemann_hurwitz : Prop.
/// For a tropical morphism of degree d, 2g − 2 = d(2h − 2) + Σ (eₚ − 1).
pub fn trop_riemann_hurwitz_ty() -> Expr {
    tg_prop()
}
/// Tropical Jacobian: TropJacobian (C : TropCurve) : Type.
/// The Jacobian of a tropical curve — a real torus of dimension g.
pub fn trop_jacobian_ty() -> Expr {
    tg_arrow(tg_type0(), tg_type0())
}
/// Mikhalkin correspondence theorem: mikhalkin_correspondence : Prop.
/// The number of genus-g tropical curves of degree d through 3d + g − 1 points equals
/// the number of complex curves (counted with multiplicity).
pub fn mikhalkin_correspondence_ty() -> Expr {
    tg_prop()
}
/// Tropical Gromov–Witten invariant: TropGW (d g : Nat) : Int.
pub fn trop_gw_invariant_ty() -> Expr {
    tg_arrow(tg_nat(), tg_arrow(tg_nat(), tg_int()))
}
/// Tropical Hurwitz number: TropHurwitzNumber (profile : List Nat) : Int.
/// Counts tropical covers of ℝ with given ramification profile.
pub fn trop_hurwitz_number_ty() -> Expr {
    tg_arrow(tg_type0(), tg_int())
}
/// Tropical ELSV formula: trop_elsv : Prop.
/// Expresses tropical Hurwitz numbers in terms of intersection numbers on tropical moduli.
pub fn trop_elsv_ty() -> Expr {
    tg_prop()
}
/// Registers all tropical geometry kernel axioms into an OxiLean `Environment`.
pub fn build_tropical_geometry_env(env: &mut Environment) {
    let axioms: &[(&str, Expr)] = &[
        ("MaxPlusSemiring", max_plus_semiring_ty()),
        ("TropicalHom", tropical_hom_ty()),
        ("tropical_idemp_law", tropical_idemp_law_ty()),
        ("TropPoly", trop_poly_ty()),
        ("TropHypersurface", trop_hypersurface_ty()),
        ("trop_dual_subdivision", trop_dual_subdivision_ty()),
        ("trop_nullstellensatz", trop_nullstellensatz_ty()),
        ("TropVariety", trop_variety_ty()),
        ("fund_thm_trop_geom", fund_thm_trop_geom_ty()),
        ("trop_bezout", trop_bezout_ty()),
        ("trop_balancing", trop_balancing_ty()),
        ("SpeyerSturmfelsTropGr", speyer_sturmfels_trop_gr_ty()),
        ("trop_plucker", trop_plucker_ty()),
        ("TropFlagVariety", trop_flag_variety_ty()),
        ("BergmanFan", bergman_fan_ty()),
        ("trop_linear_space_matroid", trop_linear_space_matroid_ty()),
        ("TropConvexSet", trop_convex_set_ty()),
        ("TropConvHull", trop_conv_hull_ty()),
        ("trop_helly", trop_helly_ty()),
        ("TropEigenvalue", trop_eigenvalue_ty()),
        ("trop_perron_frobenius", trop_perron_frobenius_ty()),
        ("TropMatPow", trop_mat_pow_ty()),
        ("NOBody", newton_okounkov_body_ty()),
        ("no_body_volume", no_body_volume_ty()),
        ("GroebnerFan", groebner_fan_ty()),
        ("InitialDegeneration", initial_degeneration_ty()),
        ("trop_compactification", trop_compactification_ty()),
        ("Amoeba", amoeba_ty()),
        ("AmoebaSpine", amoeba_spine_ty()),
        ("trop_amoeba_limit", trop_amoeba_limit_ty()),
        ("TropicalizationMap", tropicalization_map_ty()),
        ("trop_functor", trop_functor_ty()),
        ("BerkovichSpace", berkovich_space_ty()),
        ("BerkovichSkeleton", berkovich_skeleton_ty()),
        ("berkovich_retraction", berkovich_retraction_ty()),
        ("NonArchField", non_arch_field_ty()),
        ("ultrametric_ineq", ultrametric_ineq_ty()),
        ("NonArchAbsValue", non_arch_abs_value_ty()),
        ("TropCurveType", trop_curve_type_ty()),
        ("trop_riemann_hurwitz", trop_riemann_hurwitz_ty()),
        ("TropJacobian", trop_jacobian_ty()),
        ("mikhalkin_correspondence", mikhalkin_correspondence_ty()),
        ("TropGW", trop_gw_invariant_ty()),
        ("TropHurwitzNumber", trop_hurwitz_number_ty()),
        ("trop_elsv", trop_elsv_ty()),
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
/// Returns a string listing the core axioms of tropical geometry.
///
/// These axioms characterise the tropical semiring (ℝ ∪ {−∞}, min, +) and
/// form the basis for formal verification in OxiLean.
pub fn build_env() -> Vec<(&'static str, &'static str)> {
    vec![
        ("tropical_add_comm",
        "forall (a b : Real), tropical_add a b = tropical_add b a",),
        ("tropical_add_assoc",
        "forall (a b c : Real), tropical_add a (tropical_add b c) = tropical_add (tropical_add a b) c",),
        ("tropical_add_idemp", "forall (a : Real), tropical_add a a = a",),
        ("tropical_mul_comm",
        "forall (a b : Real), tropical_mul a b = tropical_mul b a",),
        ("tropical_mul_assoc",
        "forall (a b c : Real), tropical_mul a (tropical_mul b c) = tropical_mul (tropical_mul a b) c",),
        ("tropical_distrib",
        "forall (a b c : Real), tropical_mul a (tropical_add b c) = tropical_add (tropical_mul a b) (tropical_mul a c)",),
        ("tropical_zero", "forall (a : Real), tropical_add NegInf a = a",),
        ("tropical_one", "forall (a : Real), tropical_mul Zero a = a",),
        ("valuation_mult",
        "forall (v : Valuation) (a b : NonZero), v (mul a b) = add (v a) (v b)",),
        ("valuation_ultrametric",
        "forall (v : Valuation) (a b : NonZero), le (v (add a b)) (min (v a) (v b))",),
        ("max_plus_semiring", "exists (S : Type), isSemiring S",), ("tropical_idemp",
        "forall (a : TropElem), tropical_add a a = a",), ("trop_poly_eval",
        "forall (f : TropPoly) (x : Real), le (trop_eval f x) (trop_eval f x)",),
        ("trop_hypersurface_codim",
        "forall (f : TropPoly), codim (TropHypersurface f) = 1",), ("trop_dual_subdiv",
        "forall (f : TropPoly), isDualSubdivision (NewtonPolytope f) (TropHypersurface f)",),
        ("trop_nullstell",
        "forall (I : TropIdeal), isEmpty (TropVariety I) = hasCert I",),
        ("fund_thm_tropical",
        "forall (I : Ideal), Trop (classicalVariety I) = TropVariety (tropIdeal I)",),
        ("trop_bezout_law",
        "forall (d1 d2 : Nat) (f g : TropPoly), intersectionCount f g = mul d1 d2",),
        ("trop_balance_cond", "forall (V : TropVariety), isBalanced V",),
        ("speyer_sturmfels_gr", "forall (k n : Nat), isFan (TropGr k n)",),
        ("trop_plucker_rels", "forall (p : TropGr k n), satisfiesTropPlucker p",),
        ("bergman_fan_matroid",
        "forall (M : Matroid), isBergmanFan M (TropLinSpace M)",), ("trop_helly_thm",
        "forall (n : Nat) (C : Fin (add n 1) -> TropConvexSet n), hasCommonPoint C",),
        ("trop_perron_frob",
        "forall (A : IrredTropMatrix), existsUnique (TropEigenvalue A)",),
        ("trop_mat_pow_conv", "forall (A : IrredTropMatrix), converges (TropMatPow A)",),
        ("no_body_vol_eq",
        "forall (X : Variety) (L : LineBundle), vol (NOBody X L) = div (deg L) (factorial (dim X))",),
        ("groebner_fan_normal",
        "forall (I : Ideal), isNormalFan (StatePolytope I) (GroebnerFan I)",),
        ("initial_degen_flat",
        "forall (X : Variety) (w : Weight), isFlatLimit X w (InitialDegen X w)",),
        ("trop_compact_snc",
        "forall (X : TorusVariety), hasSNCBoundary (TropCompact X)",),
        ("amoeba_log_image",
        "forall (V : ComplexVariety), Amoeba V = image (logAbsMap) V",),
        ("amoeba_spine_retract",
        "forall (A : Amoeba), isDeformRetract (AmoebaSpine A) A",), ("trop_limit_amoeba",
        "forall (V : Family), limAsZero (scaledAmoeba V) = TropVariety V",),
        ("trop_map_val",
        "forall (K : NonArchField) (x : Torus K), TropMap K x = negVal x",),
        ("berkovich_skeleton_retract",
        "forall (X : AlgVariety) (K : NonArchField), isRetract (Skeleton (Berkovich X K)) (Berkovich X K)",),
        ("ultrametric_triangle",
        "forall (K : NonArchField) (a b : K), le (absVal (add a b)) (max (absVal a) (absVal b))",),
        ("mikhalkin_corr",
        "forall (d g : Nat), tropCurveCount d g = complexCurveCount d g",),
        ("trop_riemann_hurwitz_formula",
        "forall (f : TropMorphism), eq (sub (mul 2 (genus Source f)) 2) (add (mul deg f (sub (mul 2 (genus Target f)) 2)) (ramification f))",),
        ("trop_hurwitz_positive",
        "forall (mu : Partition), le 0 (TropHurwitzNumber mu)",),
    ]
}
/// Tropical enumerative geometry.
#[allow(dead_code)]
pub fn tropical_enumerative_results() -> Vec<(&'static str, &'static str)> {
    vec![
        (
            "Mikhalkin correspondence",
            "Tropical curves <-> complex curves via Welschinger's count",
        ),
        ("BKK theorem", "Mixed volume = number of isolated roots"),
        (
            "Nishinou-Siebert",
            "Tropical-classical curve correspondence for toric surfaces",
        ),
        (
            "Speyer-Sturmfels",
            "Tropical Grassmannian parametrizes tropical linear spaces",
        ),
        (
            "Gathmann-Markwig",
            "Tropical Hurwitz numbers via graph enumeration",
        ),
        (
            "Allermann-Rau",
            "Tropical intersection theory via stable intersection",
        ),
    ]
}
#[cfg(test)]
mod trop_geom_ext_tests {
    use super::*;
    #[test]
    fn test_tropical_curve() {
        let c = TropicalCurveExt2::in_tropical_plane(3);
        assert_eq!(c.genus, 1);
        assert_eq!(c.euler_characteristic(), 0);
    }
    #[test]
    fn test_tropical_grassmannian() {
        let tg = TropicalGrassmannianExt::new(2, 4);
        assert_eq!(tg.dimension(), 4);
    }
    #[test]
    fn test_newton_polytope() {
        let np = NewtonPolytopeExt::new(vec![vec![0, 0], vec![1, 0], vec![0, 1]]);
        assert_eq!(np.num_vertices(), 3);
    }
    #[test]
    fn test_tropical_hypersurface() {
        let th = TropicalHypersurfaceExt::new(
            vec![vec![1, 0], vec![0, 1], vec![0, 0]],
            vec![0.0, 0.0, 0.0],
        );
        assert!(th.is_on_hypersurface(&[0.0, 0.0]));
    }
    #[test]
    fn test_enumerative_nonempty() {
        let results = tropical_enumerative_results();
        assert!(!results.is_empty());
    }
}
#[cfg(test)]
mod trop_extra_tests {
    use super::*;
    #[test]
    fn test_tropical_abelian_variety() {
        let jac = TropicalAbelianVariety::jacobian(3);
        assert_eq!(jac.dimension(), 3);
    }
    #[test]
    fn test_valuated_matroid() {
        let vm = ValuatedMatroid::new(4, 2, "U24");
        assert!(!vm.tropical_linear_space_description().is_empty());
    }
}
#[cfg(test)]
mod trop_fan_tests {
    use super::*;
    #[test]
    fn test_tropical_fan() {
        let fan = TropicalFan::new("V(x+y+1)", 2, 1);
        assert!(fan.represents_tropical_variety());
    }
    #[test]
    fn test_moduli_m0n() {
        let m0n = TropicalModuliM0n::new(5);
        assert_eq!(m0n.dimension(), 2);
    }
    #[test]
    fn test_regular_subdivision() {
        let pts = vec![vec![0i64], vec![1], vec![2]];
        let hs = vec![0.0, 0.0, 0.0];
        let sd = RegularSubdivision::new(pts, hs);
        assert_eq!(sd.num_cells, 2);
    }
}
#[cfg(test)]
mod trop_lp_tests {
    use super::*;
    #[test]
    fn test_tropical_lp() {
        let tlp = TropicalLinearProgram::new(
            2,
            vec![1.0, 2.0],
            vec![vec![1.0, 0.0], vec![0.0, 1.0]],
            vec![0.0, 0.0],
        );
        assert_eq!(tlp.num_variables, 2);
        assert!(tlp.optimal_value_lower_bound() > 0.0);
    }
}
