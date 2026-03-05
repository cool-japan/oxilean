//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{
    ABCTriple, ArakelovData, ChabautyBound, EllipticCurveArithmetic, FaltingsData, HeightFunction,
    MordellWeilData, MordellWeilGroup, NaiveHeightComputer, ProjectiveCurve, SelmerGroupEstimator,
    ThueSolver, WeilHeight,
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
pub fn nat_ty() -> Expr {
    cst("Nat")
}
pub fn real_ty() -> Expr {
    cst("Real")
}
pub fn int_ty() -> Expr {
    cst("Int")
}
pub fn bool_ty() -> Expr {
    cst("Bool")
}
pub fn add_axiom(
    env: &mut Environment,
    name: &str,
    univ_params: Vec<Name>,
    ty: Expr,
) -> Result<(), String> {
    env.add(Declaration::Axiom {
        name: Name::str(name),
        univ_params,
        ty,
    })
    .map_err(|e| format!("add_axiom({name}): {e:?}"))
}
/// `HeightFunction : Type → Real`
///
/// A height function h: V(Q̄) → ℝ on a variety V assigns a real-valued
/// "arithmetic complexity" to each rational point.
pub fn height_function_ty() -> Expr {
    arrow(type0(), real_ty())
}
/// `WeilHeightMachine : Type → Type`
///
/// The Weil height machine: a functorial assignment of height functions
/// to line bundles on a variety, compatible with tensor products and pullbacks.
pub fn weil_height_machine_ty() -> Expr {
    arrow(type0(), type0())
}
/// `CanonicalHeight : Type → Real`
///
/// The canonical (Néron-Tate) height ĥ on an abelian variety A,
/// characterized by ĥ = lim_{n→∞} h([2^n]P) / 4^n.
pub fn canonical_height_ty() -> Expr {
    arrow(type0(), real_ty())
}
/// `NorthcottProperty : Type → Prop`
///
/// A height function satisfies the Northcott property if for every B > 0
/// and every number field K of bounded degree, {P ∈ V(K) | h(P) ≤ B} is finite.
pub fn northcott_property_ty() -> Expr {
    arrow(type0(), prop())
}
/// `LogarithmicHeight : Nat → Real`
///
/// The logarithmic height of a rational number p/q (in lowest terms): log max(|p|, |q|).
pub fn logarithmic_height_ty() -> Expr {
    arrow(nat_ty(), real_ty())
}
/// `AbsoluteLogarithmicHeight : Type → Real`
///
/// The absolute logarithmic Weil height on projective space P^n(Q̄):
/// h(x) = (1/[K:Q]) ∑_v max_i log |x_i|_v for a number field K.
pub fn absolute_logarithmic_height_ty() -> Expr {
    arrow(type0(), real_ty())
}
/// `HeightPairing : Type → Real`
///
/// The height pairing ⟨·,·⟩: A(K) × A(K) → ℝ on an abelian variety,
/// derived from the canonical height via the parallelogram law.
pub fn height_pairing_ty() -> Expr {
    arrow(type0(), real_ty())
}
/// `SzpiroInequality : Prop`
///
/// Szpiro's conjecture: for an elliptic curve E/Q with conductor N and
/// minimal discriminant Δ, we have |Δ| ≤ C_ε · N^{6+ε} for all ε > 0.
pub fn szpiro_inequality_ty() -> Expr {
    prop()
}
/// `MordellWeilGroup : Type → Type`
///
/// The Mordell-Weil group A(K) of rational points on an abelian variety A
/// over a number field K; a finitely generated abelian group.
pub fn mordell_weil_group_ty() -> Expr {
    arrow(type0(), type0())
}
/// `MordellWeilTheorem : Prop`
///
/// The Mordell-Weil theorem: for any abelian variety A over a number field K,
/// the group A(K) is finitely generated.
pub fn mordell_weil_theorem_ty() -> Expr {
    prop()
}
/// `MordellWeilRank : Type → Nat`
///
/// The rank of the Mordell-Weil group A(K): the number of independent generators
/// of the free part of A(K) ≅ Z^r ⊕ A(K)_tors.
pub fn mordell_weil_rank_ty() -> Expr {
    arrow(type0(), nat_ty())
}
/// `TorsionSubgroup : Type → Type`
///
/// The torsion subgroup A(K)_tors of the Mordell-Weil group.
pub fn torsion_subgroup_ty() -> Expr {
    arrow(type0(), type0())
}
/// `WeakMordellWeil : Prop`
///
/// The weak Mordell-Weil theorem: A(K) / nA(K) is finite for any n ≥ 2,
/// which is the key step in the proof of the full theorem.
pub fn weak_mordell_weil_ty() -> Expr {
    prop()
}
/// `DescentProcedure : Type → Type`
///
/// The descent procedure: a method for bounding generators of A(K)
/// using 2-isogenies or n-descent via Selmer groups.
pub fn descent_procedure_ty() -> Expr {
    arrow(type0(), type0())
}
/// `SelmerGroup : Type → Type`
///
/// The n-Selmer group Sel^(n)(A/K) ⊆ H^1(K, A[n]): fits into
/// 0 → A(K)/nA(K) → Sel^(n)(A/K) → Ш(A/K)[n] → 0.
pub fn selmer_group_ty() -> Expr {
    arrow(type0(), type0())
}
/// `ShafarevichTateGroup : Type → Type`
///
/// The Shafarevich-Tate group Ш(A/K): the obstruction to local-global
/// principles for the abelian variety A over K.
pub fn shafarevich_tate_group_ty() -> Expr {
    arrow(type0(), type0())
}
/// `FaltingsTheorem : Prop`
///
/// Faltings' theorem (Mordell conjecture): a smooth projective curve C of genus
/// g ≥ 2 over a number field K has only finitely many K-rational points.
pub fn faltings_theorem_ty() -> Expr {
    prop()
}
/// `MordellConjecture : Prop`
///
/// The Mordell conjecture (1922), proven by Faltings (1983):
/// C(K) is finite for curves C/K of genus g ≥ 2.
pub fn mordell_conjecture_ty() -> Expr {
    prop()
}
/// `CurveGenus : Type → Nat`
///
/// The geometric genus g of a smooth projective curve C.
pub fn curve_genus_ty() -> Expr {
    arrow(type0(), nat_ty())
}
/// `RiemannRochCurve : Prop`
///
/// The Riemann-Roch theorem for curves: dim L(D) - dim L(K_C - D) = deg(D) - g + 1.
pub fn riemann_roch_curve_ty() -> Expr {
    prop()
}
/// `JacobianVariety : Type → Type`
///
/// The Jacobian variety J(C) of a curve C: an abelian variety of dimension g
/// with C ↪ J(C) (for a base point P₀).
pub fn jacobian_variety_ty() -> Expr {
    arrow(type0(), type0())
}
/// `TorellisTheorem : Prop`
///
/// Torelli's theorem: the Jacobian J(C) together with its principal polarization
/// determines C up to isomorphism.
pub fn torellis_theorem_ty() -> Expr {
    prop()
}
/// `EffectiveMordell : Prop`
///
/// The effective Mordell problem: can one compute an explicit bound on |C(K)|
/// or a finite set containing C(K)? Still open in general.
pub fn effective_mordell_ty() -> Expr {
    prop()
}
/// `ABCTriple : Nat → Nat → Nat → Prop`
///
/// An abc triple: coprime positive integers a, b, c with a + b = c.
pub fn abc_triple_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), arrow(nat_ty(), prop())))
}
/// `RadicalOfInteger : Nat → Nat`
///
/// The radical rad(n) = ∏_{p | n} p: the product of distinct prime factors of n.
pub fn radical_of_integer_ty() -> Expr {
    arrow(nat_ty(), nat_ty())
}
/// `ABCConjecture : Prop`
///
/// The abc conjecture (Masser-Oesterlé, 1985): for every ε > 0, there are
/// finitely many abc triples with c > rad(abc)^{1+ε}.
pub fn abc_conjecture_ty() -> Expr {
    prop()
}
/// `ABCQuality : Nat → Nat → Nat → Real`
///
/// The quality of an abc triple: q(a,b,c) = log(c) / log(rad(abc)).
/// The abc conjecture asserts q < 1 + ε for all but finitely many triples.
pub fn abc_quality_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), arrow(nat_ty(), real_ty())))
}
/// `MaserOesterlaeABC : Prop`
///
/// The Masser-Oesterlé form of the abc conjecture: equivalent to many deep
/// Diophantine results including Fermat's Last Theorem.
pub fn masser_oesterle_abc_ty() -> Expr {
    prop()
}
/// `ABCImpliesFermat : Prop`
///
/// Implication: abc conjecture (with exponent 1) implies Fermat's Last Theorem
/// (and more generally Beal's conjecture).
pub fn abc_implies_fermat_ty() -> Expr {
    prop()
}
/// `LangsConjecture : Prop`
///
/// Lang's conjecture: if A is an abelian variety over a number field K and
/// X ⊆ A is a subvariety not containing any translate of a positive-dimensional
/// abelian subvariety, then X(K) is finite.
pub fn langs_conjecture_ty() -> Expr {
    prop()
}
/// `ManinMumfordConjecture : Prop`
///
/// The Manin-Mumford conjecture (Raynaud's theorem): if X is a curve in its
/// Jacobian J, then X contains only finitely many torsion points of J.
pub fn manin_mumford_conjecture_ty() -> Expr {
    prop()
}
/// `RaynaudTheorem : Prop`
///
/// Raynaud's theorem (1983) proving Manin-Mumford: subvarieties of abelian
/// varieties containing infinitely many torsion points are unions of torsion cosets.
pub fn raynaud_theorem_ty() -> Expr {
    prop()
}
/// `BogomolovConjecture : Prop`
///
/// Bogomolov's conjecture (Zhang's theorem): for a curve X in J(X), there is
/// ε > 0 such that {P ∈ X(Q̄) | ĥ(P) ≤ ε} is finite (ĥ = Néron-Tate height).
pub fn bogomolov_conjecture_ty() -> Expr {
    prop()
}
/// `Equidistribution : Prop`
///
/// Equidistribution theorem (Szpiro-Ullmo-Zhang): points of small height on
/// abelian varieties are equidistributed with respect to Haar measure.
pub fn equidistribution_ty() -> Expr {
    prop()
}
/// `BombieriLangConjecture : Prop`
///
/// The Bombieri-Lang conjecture: if X is a variety of general type over a
/// number field K, then X(K) is not Zariski dense in X.
pub fn bombieri_lang_conjecture_ty() -> Expr {
    prop()
}
/// `VarietyOfGeneralType : Type → Prop`
///
/// A variety X is of general type if its Kodaira dimension κ(X) = dim(X).
pub fn variety_of_general_type_ty() -> Expr {
    arrow(type0(), prop())
}
/// `KodairaDimension : Type → Int`
///
/// The Kodaira dimension κ(X) ∈ {-∞, 0, 1, ..., dim X} of a variety X.
pub fn kodaira_dimension_ty() -> Expr {
    arrow(type0(), int_ty())
}
/// `ZariskiDensity : Type → Type → Prop`
///
/// A subset S of a variety X is Zariski dense if every polynomial vanishing
/// on S vanishes on all of X.
pub fn zariski_density_ty() -> Expr {
    arrow(type0(), arrow(type0(), prop()))
}
/// `CanonicalBundle : Type → Type`
///
/// The canonical bundle (or canonical divisor class) K_X of a smooth variety X.
pub fn canonical_bundle_ty() -> Expr {
    arrow(type0(), type0())
}
/// `PluricanonicalMap : Type → Nat → Type`
///
/// The m-canonical map φ_{mK_X}: X → P(H^0(X, mK_X)^*).
pub fn pluricanonical_map_ty() -> Expr {
    arrow(type0(), arrow(nat_ty(), type0()))
}
/// `LineBundle : Type → Type`
///
/// A line bundle (invertible sheaf) L on a variety X.
pub fn line_bundle_ty() -> Expr {
    arrow(type0(), type0())
}
/// `AmpleLineBundleAxiom : Type → Prop`
///
/// A line bundle L on X is ample if some power L^⊗m gives an embedding X ↪ P^N.
pub fn ample_line_bundle_ty() -> Expr {
    arrow(type0(), prop())
}
/// `NakaiMoishezonCriterion : Prop`
///
/// Nakai-Moishezon criterion: L is ample iff L^dim(Z) · Z > 0 for every
/// irreducible subvariety Z ⊆ X (including Z = X).
pub fn nakai_moishezon_criterion_ty() -> Expr {
    prop()
}
/// `KleimanCriterion : Prop`
///
/// Kleiman's criterion: L is ample iff L lies in the interior of the
/// nef (numerically effective) cone in N^1(X)_ℝ.
pub fn kleiman_criterion_ty() -> Expr {
    prop()
}
/// `SeshadriConstant : Type → Real`
///
/// The Seshadri constant ε(L; x) at a point x measures the local positivity
/// of L: ε(L; x) = inf_{x ∈ C} (L · C) / mult_x(C).
pub fn seshadri_constant_ty() -> Expr {
    arrow(type0(), real_ty())
}
/// `NefDivisor : Type → Prop`
///
/// A divisor D on X is nef (numerically effective) if D · C ≥ 0 for every
/// curve C ⊆ X.
pub fn nef_divisor_ty() -> Expr {
    arrow(type0(), prop())
}
/// `BigLineBundleAxiom : Type → Prop`
///
/// A line bundle L is big if h^0(X, L^m) ~ m^{dim X} (maximum growth rate).
pub fn big_line_bundle_ty() -> Expr {
    arrow(type0(), prop())
}
/// `VolumeOfLineBundleAxiom : Type → Real`
///
/// The volume vol(L) = lim sup_{m→∞} h^0(X, L^m) / (m^n / n!) of a line bundle.
pub fn volume_of_line_bundle_ty() -> Expr {
    arrow(type0(), real_ty())
}
/// `ArakelovDivisor : Type → Type`
///
/// An Arakelov divisor on an arithmetic surface S: a finite combination of
/// horizontal divisors + vertical divisors at finite primes + real-valued
/// "divisors at infinity" (one per Archimedean place).
pub fn arakelov_divisor_ty() -> Expr {
    arrow(type0(), type0())
}
/// `ArithmeticIntersection : Type → Real`
///
/// The Arakelov intersection pairing ⟨D₁, D₂⟩: extends the algebraic
/// intersection to include contributions from Archimedean places via
/// Green's functions.
pub fn arithmetic_intersection_ty() -> Expr {
    arrow(type0(), real_ty())
}
/// `ArakelovRiemannRoch : Prop`
///
/// The arithmetic Riemann-Roch theorem (Faltings, Hriljac, Gillet-Soulé):
/// relates arithmetic degree, height, and analytic torsion.
pub fn arakelov_riemann_roch_ty() -> Expr {
    prop()
}
/// `ArithmeticSurface : Type`
///
/// An arithmetic surface S → Spec(O_K): a flat projective scheme over the
/// ring of integers of a number field K, with generic fiber a smooth curve.
pub fn arithmetic_surface_ty() -> Expr {
    type0()
}
/// `GreensFunction : Type → Real`
///
/// A Green's function g_μ(P, Q) on a Riemann surface Σ with respect to a
/// measure μ: satisfies ∂∂̄ g_μ = δ_P - μ.
pub fn greens_function_ty() -> Expr {
    arrow(type0(), real_ty())
}
/// `AnalyticTorsion : Type → Real`
///
/// The Ray-Singer analytic torsion T(X, E) of a vector bundle E on a
/// compact Riemannian manifold X: a spectral invariant.
pub fn analytic_torsion_ty() -> Expr {
    arrow(type0(), real_ty())
}
/// `FaltingsHeight : Type → Real`
///
/// The Faltings height h_F(A) of an abelian variety A: defined via the
/// Hodge bundle on the moduli space of abelian varieties.
pub fn faltings_height_ty() -> Expr {
    arrow(type0(), real_ty())
}
/// `GilletSouleRiemannRoch : Prop`
///
/// The Gillet-Soulé arithmetic Riemann-Roch theorem in higher dimensions:
/// extends Arakelov's theory to higher-dimensional arithmetic varieties.
pub fn gillet_soule_riemann_roch_ty() -> Expr {
    prop()
}
/// `AdmissibleMetric : Type → Prop`
///
/// An admissible metric on a line bundle L over an arithmetic surface:
/// a smooth Hermitian metric at infinite places satisfying certain curvature conditions.
pub fn admissible_metric_ty() -> Expr {
    arrow(type0(), prop())
}
/// `NevanlinnaFirstMainThm : Prop`
///
/// Nevanlinna's first main theorem: for a meromorphic function f and a value a,
/// T(r, f) = N(r, a, f) + m(r, a, f) + O(1),
/// where T is the characteristic, N counts poles, m is the proximity function.
pub fn nevanlinna_first_main_thm_ty() -> Expr {
    prop()
}
/// `NevanlinnaSecondMainThm : Prop`
///
/// Nevanlinna's second main theorem: for distinct values a₁, ..., aₙ,
/// (n-2) T(r,f) ≤ ∑ N(r, aᵢ, f) + S(r, f),
/// where S(r, f) is an error term. Generalizes Picard's theorem.
pub fn nevanlinna_second_main_thm_ty() -> Expr {
    prop()
}
/// `NevanlinnaCharacteristic : Type → Real`
///
/// The Nevanlinna characteristic T(r, f) = m(r, ∞, f) + N(r, ∞, f)
/// measuring the "growth" of a meromorphic function f.
pub fn nevanlinna_characteristic_ty() -> Expr {
    arrow(type0(), real_ty())
}
/// `DeficiencyFunction : Type → Real`
///
/// The deficiency δ(a, f) = 1 - lim sup N(r, a, f)/T(r, f) ∈ [0, 1];
/// the deficiency relation ∑_a δ(a, f) ≤ 2 follows from the second main theorem.
pub fn deficiency_function_ty() -> Expr {
    arrow(type0(), real_ty())
}
/// `BlochOchiaiTheorem : Prop`
///
/// The Bloch-Ochiai theorem: a holomorphic map f: ℂ → X into a variety X
/// with irregularity q > dim X cannot be Zariski dense in X.
pub fn bloch_ochiai_theorem_ty() -> Expr {
    prop()
}
/// `PicardTheorem : Prop`
///
/// Picard's (little) theorem: a non-constant entire function omits at most
/// one value in ℂ. (Follows from Nevanlinna's second main theorem with n=3.)
pub fn picard_theorem_ty() -> Expr {
    prop()
}
/// `ProximityFunction : Type → Real`
///
/// The proximity function m(r, a, f) = (1/2π) ∫ log⁺|1/(f(re^{iθ}) - a)| dθ
/// measuring how closely f approximates the value a on average.
pub fn proximity_function_ty() -> Expr {
    arrow(type_0_alias(), real_ty())
}
pub fn type_0_alias() -> Expr {
    type0()
}
/// `CountingFunction : Type → Real`
///
/// The counting function N(r, a, f) = ∫₀ʳ (n(t, a) - n(0, a))/t dt + n(0, a) log r
/// counting the number of a-points of f weighted by distance.
pub fn counting_function_ty() -> Expr {
    arrow(type0(), real_ty())
}
/// `VojtaConjecture : Prop`
///
/// Vojta's conjecture: a vast generalization of both Nevanlinna theory and
/// Faltings' theorem, predicting that rational points on a variety of log
/// general type avoid a proper closed subset.
pub fn vojta_conjecture_ty() -> Expr {
    prop()
}
/// `DiophantineApproximation : Nat → Nat → Real`
///
/// Diophantine approximation quality: how well an algebraic number α can be
/// approximated by rationals p/q (Roth's theorem: exponent is exactly 2).
pub fn diophantine_approximation_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), real_ty()))
}
/// `RothTheorem : Prop`
///
/// Roth's theorem (1955): an algebraic number of degree ≥ 2 cannot be
/// approximated by rationals to order > 2; i.e., |α - p/q| > 1/q^{2+ε}
/// for all but finitely many p/q.
pub fn roth_theorem_ty() -> Expr {
    prop()
}
/// `NeronTateHeight : Type → Real`
///
/// The canonical Néron-Tate height ĥ: A(K) → ℝ on an abelian variety,
/// a positive semi-definite quadratic form on A(K) ⊗ ℝ.
pub fn neron_tate_height_ty() -> Expr {
    arrow(type0(), real_ty())
}
/// `LehmerConjecture : Prop`
///
/// Lehmer's conjecture (1933): the Mahler measure M(α) of a non-zero non-root-of-unity
/// algebraic integer α satisfies M(α) ≥ λ₀ ≈ 1.1762... (Lehmer's number).
pub fn lehmer_conjecture_ty() -> Expr {
    prop()
}
/// `MahlerMeasure : Nat → Real`
///
/// The Mahler measure M(P) = exp((1/2π) ∫₀²π log |P(e^{iθ})| dθ) of a polynomial P.
pub fn mahler_measure_ty() -> Expr {
    arrow(nat_ty(), real_ty())
}
/// `HeightBound : Type → Nat → Prop`
///
/// A height bound: #{P ∈ V(K) | H(P) ≤ B} ≤ f(B) for an explicit function f
/// (Northcott property quantified).
pub fn height_bound_ty() -> Expr {
    arrow(type0(), arrow(nat_ty(), prop()))
}
/// `HassePrinciple : Type → Prop`
///
/// The Hasse-Minkowski principle: a quadratic form has a rational point iff
/// it has p-adic and real points for all primes p.
pub fn hasse_principle_ty() -> Expr {
    arrow(type0(), prop())
}
/// `WeakApproximation : Type → Prop`
///
/// Weak approximation: V(ℚ) is dense in ∏_v V(ℚ_v) (product of local fields).
pub fn weak_approximation_ty() -> Expr {
    arrow(type0(), prop())
}
/// `StrongApproximation : Type → Prop`
///
/// Strong approximation: the adelic points V(𝔸) → ∏'_v V(ℚ_v) is dense
/// away from one place (holds for simply connected groups, not curves in general).
pub fn strong_approximation_ty() -> Expr {
    arrow(type0(), prop())
}
/// `LocalGlobalPrinciple : Type → Prop`
///
/// Local-global principle: V has a global rational point iff V has
/// local (p-adic and real) points for all primes and at infinity.
pub fn local_global_principle_ty() -> Expr {
    arrow(type0(), prop())
}
/// `ObstructionToHasse : Type → Type`
///
/// Obstruction to the Hasse principle: an element of Br(V) or
/// a Brauer-Manin obstruction accounting for failure of local-global.
pub fn obstruction_to_hasse_ty() -> Expr {
    arrow(type0(), type0())
}
/// `ChabautyColeman : Type → Nat → Prop`
///
/// The Chabauty-Coleman method: for a curve C/ℚ of genus g with rank r < g,
/// |C(ℚ)| ≤ 2g - 2 + #{bad primes} (Coleman's explicit bound, 1985).
pub fn chabauty_coleman_ty() -> Expr {
    arrow(type0(), arrow(nat_ty(), prop()))
}
/// `PAdicIntegration : Type → Complex → Complex`
///
/// Coleman's p-adic line integrals ∫ ω on a curve C, used to bound rational points.
pub fn p_adic_integration_ty() -> Expr {
    arrow(type0(), arrow(cst("Complex"), cst("Complex")))
}
/// `ChabautyBoundValue : Nat → Nat → Nat`
///
/// The explicit Chabauty bound: for genus g, rank r < g, the number of rational
/// points is bounded by a computable function of g and the prime p.
pub fn chabauty_bound_value_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), nat_ty()))
}
/// `TwistingMethod : Type → Nat → Type`
///
/// The twisting / Selmer descent method: computing twists of curves to reduce
/// the rank below the genus for the Chabauty-Coleman method.
pub fn twisting_method_ty() -> Expr {
    arrow(type0(), arrow(nat_ty(), type0()))
}
/// `TwoDescent : Type → Type`
///
/// The 2-descent on an elliptic curve E/ℚ: computes an upper bound on the rank
/// via the 2-Selmer group Sel^(2)(E/ℚ).
pub fn two_descent_ty() -> Expr {
    arrow(type0(), type0())
}
/// `SelmerGroupExplicit : Type → Nat → Type`
///
/// An explicit n-Selmer group computation (2-descent or n-descent),
/// bounding the Mordell-Weil rank from above.
pub fn selmer_group_explicit_ty() -> Expr {
    arrow(type0(), arrow(nat_ty(), type0()))
}
/// `VisibilityOfSelmer : Type → Prop`
///
/// Cremona-Mazur visibility: a Selmer element can be "explained" by a
/// corresponding element in the Shafarevich-Tate group of an isogenous curve.
pub fn visibility_of_selmer_ty() -> Expr {
    arrow(type0(), prop())
}
/// `IsogenyDescent : Type → Type → Type`
///
/// Descent via an isogeny φ: A → B, giving a short exact sequence
/// 0 → B(K)/φA(K) → Sel^(φ)(A/K) → Ш(A/K)[φ̂] → 0.
pub fn isogeny_descent_ty() -> Expr {
    arrow(type0(), arrow(type0(), type0()))
}
/// `BrauerGroup : Type → Type`
///
/// The Brauer group Br(X) = H^2(X, G_m) of a variety X,
/// containing Azumaya algebras up to Morita equivalence.
pub fn brauer_group_ty() -> Expr {
    arrow(type0(), type0())
}
/// `BrauerManinObstruction : Type → Prop`
///
/// The Brauer-Manin obstruction: X(𝔸)^{Br} = ∅ implies X(ℚ) = ∅
/// (sufficient to explain many failures of the Hasse principle).
pub fn brauer_manin_obstruction_ty() -> Expr {
    arrow(type0(), prop())
}
/// `HasseNoetherBrauer : Prop`
///
/// Hasse-Brauer-Noether theorem: Br(K) = ⊕_v Br(K_v) for a global field K,
/// with exact sequence 0 → Br(K) → ⊕ Br(K_v) → ℚ/ℤ → 0.
pub fn hasse_noether_brauer_ty() -> Expr {
    prop()
}
/// `AzumayaAlgebra : Type → Type`
///
/// An Azumaya algebra over a scheme X: a locally free O_X-algebra A
/// with A ⊗ A^{op} ≅ End(A).
pub fn azumaya_algebra_ty() -> Expr {
    arrow(type0(), type0())
}
/// `VojtaDictionary : Prop`
///
/// Vojta's dictionary: rational/integer points ↔ values of meromorphic functions,
/// heights ↔ Nevanlinna characteristics, ramification ↔ discriminant.
pub fn vojta_dictionary_ty() -> Expr {
    prop()
}
/// `FunctionFieldAnalogy : Prop`
///
/// The function field analogy: arithmetic over number fields mirrors geometry over
/// function fields; e.g., Faltings ↔ de Franchis, abc ↔ Mason-Stothers.
pub fn function_field_analogy_ty() -> Expr {
    prop()
}
/// `MasonStothers : Prop`
///
/// Mason-Stothers theorem (function field abc): for coprime polynomials a+b=c
/// over ℂ[t], max(deg a, deg b, deg c) ≤ deg(rad(abc)) - 1.
pub fn mason_stothers_ty() -> Expr {
    prop()
}
/// `BakerMethod : Nat → Prop`
///
/// Baker's method: effective lower bounds for linear forms in logarithms,
/// giving effective bounds for solutions of Thue equations and S-unit equations.
pub fn baker_method_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `ThueEquation : Nat → Nat → Prop`
///
/// Thue equation: F(x, y) = m for a degree ≥ 3 homogeneous polynomial F
/// has only finitely many integer solutions (Thue 1909; effective via Baker).
pub fn thue_equation_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), prop()))
}
/// `SUnitEquation : Nat → Prop`
///
/// S-unit equation: for a finite set S of primes, the equation x + y = 1
/// with x, y ∈ O_{K,S}× has only finitely many solutions (Evertse 1984).
pub fn s_unit_equation_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `LiouvilleApproximation : Nat → Real`
///
/// Liouville's approximation bound: |α - p/q| > c / q^n for algebraic α of degree n.
pub fn liouville_approximation_ty() -> Expr {
    arrow(nat_ty(), real_ty())
}
/// `SubspaceTheorem : Prop`
///
/// The Subspace theorem (Schmidt 1972): a generalization of Roth's theorem to
/// linear forms in several variables; implies finiteness of integer points on
/// many varieties.
pub fn subspace_theorem_ty() -> Expr {
    prop()
}
/// `RationallyConnected : Type → Prop`
///
/// A variety X is rationally connected if any two points can be connected by
/// a chain of rational curves (Mori-Miyaoka-Mori 1992 for dimension ≤ 3).
pub fn rationally_connected_ty() -> Expr {
    arrow(type0(), prop())
}
/// `BendAndBreak : Type → Prop`
///
/// Mori's bend-and-break technique: through any point of a Fano variety
/// there passes a rational curve of bounded degree.
pub fn bend_and_break_ty() -> Expr {
    arrow(type0(), prop())
}
/// `MinimalModel : Type → Type`
///
/// The minimal model of a surface: the end result of the minimal model program (MMP),
/// obtained by contracting (-1)-curves.
pub fn minimal_model_ty() -> Expr {
    arrow(type0(), type0())
}
/// `FanoVariety : Type → Prop`
///
/// A Fano variety: a smooth projective variety X with -K_X ample
/// (anticanonical class is positive).
pub fn fano_variety_ty() -> Expr {
    arrow(type0(), prop())
}
/// `CampanaManinConjecture : Type → Prop`
///
/// The Campana-Manin conjecture: #{P ∈ X(ℚ) | H(P) ≤ B} ~ C · B^a (log B)^{b-1}
/// for Fano varieties X, with explicit constants a, b predicted by geometry.
pub fn campana_manin_conjecture_ty() -> Expr {
    arrow(type0(), prop())
}
/// `HermitianLineBundleArakelov : Type → Type`
///
/// A Hermitian line bundle (L, ‖·‖) on an arithmetic variety: a line bundle L
/// equipped with a smooth Hermitian metric at each Archimedean place.
pub fn hermitian_line_bundle_arakelov_ty() -> Expr {
    arrow(type0(), type0())
}
/// `ArithmeticChowGroup : Type → Type`
///
/// The arithmetic Chow group CH^p_ℚ(X) of an arithmetic variety X,
/// generated by arithmetic cycles (Z, g_Z) modulo arithmetic rational equivalence.
pub fn arithmetic_chow_group_ty() -> Expr {
    arrow(type0(), type0())
}
/// `ZhangAdmissiblePairing : Type → Real`
///
/// Zhang's admissible pairing on a curve: an extension of the Arakelov
/// intersection pairing defined using the Arakelov-Green's function.
pub fn zhang_admissible_pairing_ty() -> Expr {
    arrow(type0(), real_ty())
}
/// `BerkovichSpace : Type → Type`
///
/// The Berkovich analytification X^an of an algebraic variety X over
/// a non-Archimedean field: a compact Hausdorff topological space.
pub fn berkovich_space_ty() -> Expr {
    arrow(type0(), type0())
}
/// `BerkovichAnalytification : Type → Type`
///
/// The analytification functor (-)^an from algebraic varieties over K
/// to Berkovich analytic spaces (Berkovich 1990).
pub fn berkovich_analytification_ty() -> Expr {
    arrow(type0(), type0())
}
/// `GAGA_NonArchimedean : Type → Prop`
///
/// Non-Archimedean GAGA: coherent sheaves on X^an correspond to coherent
/// algebraic sheaves on X (for proper X over a non-Archimedean field).
pub fn gaga_non_archimedean_ty() -> Expr {
    arrow(type0(), prop())
}
/// `BerkovichRetract : Type → Type`
///
/// The skeleton retract of a Berkovich space: the Berkovich space retracts
/// onto a canonical simplicial complex (the skeleton Sk(X^an)).
pub fn berkovich_retract_ty() -> Expr {
    arrow(type0(), type0())
}
/// `AnabelianVariety : Type → Prop`
///
/// An anabelian variety: a variety X/K whose isomorphism class is determined
/// by its geometric fundamental group π₁(X_K̄) together with the Galois action.
pub fn anabelian_variety_ty() -> Expr {
    arrow(type0(), prop())
}
/// `GrothendieckAnabelianConjecture : Prop`
///
/// Grothendieck's section conjecture: for a hyperbolic curve C/K over a number field,
/// C(K) bijects with sections of π₁(C) → Gal(K̄/K) up to conjugation.
pub fn grothendieck_anabelian_conjecture_ty() -> Expr {
    prop()
}
/// `MochizukiTheorem : Prop`
///
/// Mochizuki's theorem (1996): the anabelian conjecture for hyperbolic curves over
/// sub-p-adic fields (a class containing number fields) is true.
pub fn mochizuki_theorem_ty() -> Expr {
    prop()
}
/// `EtaleFundamentalGroup : Type → Type`
///
/// The etale fundamental group π₁^et(X, x̄) of a connected scheme X with
/// geometric base point x̄: classifies finite etale covers of X.
pub fn etale_fundamental_group_ty() -> Expr {
    arrow(type0(), type0())
}
/// `GaloisAction : Type → Type`
///
/// The outer Galois action Gal(K̄/K) → Out(π₁^et(X_K̄)) on the geometric
/// fundamental group of a variety X/K.
pub fn galois_action_ty() -> Expr {
    arrow(type0(), type0())
}
/// Populate `env` with all Diophantine geometry axioms.
pub fn build_diophantine_geometry_env(env: &mut Environment) -> Result<(), String> {
    add_axiom(env, "HeightFunction", vec![], height_function_ty())?;
    add_axiom(env, "WeilHeightMachine", vec![], weil_height_machine_ty())?;
    add_axiom(env, "CanonicalHeight", vec![], canonical_height_ty())?;
    add_axiom(env, "NorthcottProperty", vec![], northcott_property_ty())?;
    add_axiom(env, "LogarithmicHeight", vec![], logarithmic_height_ty())?;
    add_axiom(
        env,
        "AbsoluteLogarithmicHeight",
        vec![],
        absolute_logarithmic_height_ty(),
    )?;
    add_axiom(env, "HeightPairing", vec![], height_pairing_ty())?;
    add_axiom(env, "SzpiroInequality", vec![], szpiro_inequality_ty())?;
    add_axiom(env, "MordellWeilGroup", vec![], mordell_weil_group_ty())?;
    add_axiom(env, "MordellWeilTheorem", vec![], mordell_weil_theorem_ty())?;
    add_axiom(env, "MordellWeilRank", vec![], mordell_weil_rank_ty())?;
    add_axiom(env, "TorsionSubgroup", vec![], torsion_subgroup_ty())?;
    add_axiom(env, "WeakMordellWeil", vec![], weak_mordell_weil_ty())?;
    add_axiom(env, "DescentProcedure", vec![], descent_procedure_ty())?;
    add_axiom(env, "SelmerGroup", vec![], selmer_group_ty())?;
    add_axiom(
        env,
        "ShafarevichTateGroup",
        vec![],
        shafarevich_tate_group_ty(),
    )?;
    add_axiom(env, "FaltingsTheorem", vec![], faltings_theorem_ty())?;
    add_axiom(env, "MordellConjecture", vec![], mordell_conjecture_ty())?;
    add_axiom(env, "CurveGenus", vec![], curve_genus_ty())?;
    add_axiom(env, "RiemannRochCurve", vec![], riemann_roch_curve_ty())?;
    add_axiom(env, "JacobianVariety", vec![], jacobian_variety_ty())?;
    add_axiom(env, "TorellisTheorem", vec![], torellis_theorem_ty())?;
    add_axiom(env, "EffectiveMordell", vec![], effective_mordell_ty())?;
    add_axiom(env, "ABCTriple", vec![], abc_triple_ty())?;
    add_axiom(env, "RadicalOfInteger", vec![], radical_of_integer_ty())?;
    add_axiom(env, "ABCConjecture", vec![], abc_conjecture_ty())?;
    add_axiom(env, "ABCQuality", vec![], abc_quality_ty())?;
    add_axiom(env, "MasserOesterlaeABC", vec![], masser_oesterle_abc_ty())?;
    add_axiom(env, "ABCImpliesFermat", vec![], abc_implies_fermat_ty())?;
    add_axiom(env, "LangsConjecture", vec![], langs_conjecture_ty())?;
    add_axiom(
        env,
        "ManinMumfordConjecture",
        vec![],
        manin_mumford_conjecture_ty(),
    )?;
    add_axiom(env, "RaynaudTheorem", vec![], raynaud_theorem_ty())?;
    add_axiom(
        env,
        "BogomolovConjecture",
        vec![],
        bogomolov_conjecture_ty(),
    )?;
    add_axiom(env, "Equidistribution", vec![], equidistribution_ty())?;
    add_axiom(
        env,
        "BombieriLangConjecture",
        vec![],
        bombieri_lang_conjecture_ty(),
    )?;
    add_axiom(
        env,
        "VarietyOfGeneralType",
        vec![],
        variety_of_general_type_ty(),
    )?;
    add_axiom(env, "KodairaDimension", vec![], kodaira_dimension_ty())?;
    add_axiom(env, "ZariskiDensity", vec![], zariski_density_ty())?;
    add_axiom(env, "CanonicalBundle", vec![], canonical_bundle_ty())?;
    add_axiom(env, "PluricanonicalMap", vec![], pluricanonical_map_ty())?;
    add_axiom(env, "LineBundle", vec![], line_bundle_ty())?;
    add_axiom(env, "AmpleLineBundle", vec![], ample_line_bundle_ty())?;
    add_axiom(
        env,
        "NakaiMoishezonCriterion",
        vec![],
        nakai_moishezon_criterion_ty(),
    )?;
    add_axiom(env, "KleimanCriterion", vec![], kleiman_criterion_ty())?;
    add_axiom(env, "SeshadriConstant", vec![], seshadri_constant_ty())?;
    add_axiom(env, "NefDivisor", vec![], nef_divisor_ty())?;
    add_axiom(env, "BigLineBundle", vec![], big_line_bundle_ty())?;
    add_axiom(
        env,
        "VolumeOfLineBundle",
        vec![],
        volume_of_line_bundle_ty(),
    )?;
    add_axiom(env, "ArakelovDivisor", vec![], arakelov_divisor_ty())?;
    add_axiom(
        env,
        "ArithmeticIntersection",
        vec![],
        arithmetic_intersection_ty(),
    )?;
    add_axiom(
        env,
        "ArakelovRiemannRoch",
        vec![],
        arakelov_riemann_roch_ty(),
    )?;
    add_axiom(env, "ArithmeticSurface", vec![], arithmetic_surface_ty())?;
    add_axiom(env, "GreensFunction", vec![], greens_function_ty())?;
    add_axiom(env, "AnalyticTorsion", vec![], analytic_torsion_ty())?;
    add_axiom(env, "FaltingsHeight", vec![], faltings_height_ty())?;
    add_axiom(
        env,
        "GilletSouleRiemannRoch",
        vec![],
        gillet_soule_riemann_roch_ty(),
    )?;
    add_axiom(env, "AdmissibleMetric", vec![], admissible_metric_ty())?;
    add_axiom(
        env,
        "NevanlinnaFirstMainThm",
        vec![],
        nevanlinna_first_main_thm_ty(),
    )?;
    add_axiom(
        env,
        "NevanlinnaSecondMainThm",
        vec![],
        nevanlinna_second_main_thm_ty(),
    )?;
    add_axiom(
        env,
        "NevanlinnaCharacteristic",
        vec![],
        nevanlinna_characteristic_ty(),
    )?;
    add_axiom(env, "DeficiencyFunction", vec![], deficiency_function_ty())?;
    add_axiom(env, "BlochOchiaiTheorem", vec![], bloch_ochiai_theorem_ty())?;
    add_axiom(env, "PicardTheorem", vec![], picard_theorem_ty())?;
    add_axiom(env, "ProximityFunction", vec![], proximity_function_ty())?;
    add_axiom(env, "CountingFunction", vec![], counting_function_ty())?;
    add_axiom(env, "VojtaConjecture", vec![], vojta_conjecture_ty())?;
    add_axiom(
        env,
        "DiophantineApproximation",
        vec![],
        diophantine_approximation_ty(),
    )?;
    add_axiom(env, "RothTheorem", vec![], roth_theorem_ty())?;
    add_axiom(env, "NeronTateHeight", vec![], neron_tate_height_ty())?;
    add_axiom(env, "LehmerConjecture", vec![], lehmer_conjecture_ty())?;
    add_axiom(env, "MahlerMeasure", vec![], mahler_measure_ty())?;
    add_axiom(env, "HeightBound", vec![], height_bound_ty())?;
    add_axiom(env, "HassePrinciple", vec![], hasse_principle_ty())?;
    add_axiom(env, "WeakApproximation", vec![], weak_approximation_ty())?;
    add_axiom(
        env,
        "StrongApproximation",
        vec![],
        strong_approximation_ty(),
    )?;
    add_axiom(
        env,
        "LocalGlobalPrinciple",
        vec![],
        local_global_principle_ty(),
    )?;
    add_axiom(env, "ObstructionToHasse", vec![], obstruction_to_hasse_ty())?;
    add_axiom(env, "ChabautyColeman", vec![], chabauty_coleman_ty())?;
    add_axiom(env, "PAdicIntegration", vec![], p_adic_integration_ty())?;
    add_axiom(env, "ChabautyBoundValue", vec![], chabauty_bound_value_ty())?;
    add_axiom(env, "TwistingMethod", vec![], twisting_method_ty())?;
    add_axiom(env, "TwoDescent", vec![], two_descent_ty())?;
    add_axiom(
        env,
        "SelmerGroupExplicit",
        vec![],
        selmer_group_explicit_ty(),
    )?;
    add_axiom(env, "VisibilityOfSelmer", vec![], visibility_of_selmer_ty())?;
    add_axiom(env, "IsogenyDescent", vec![], isogeny_descent_ty())?;
    add_axiom(env, "BrauerGroup", vec![], brauer_group_ty())?;
    add_axiom(
        env,
        "BrauerManinObstruction",
        vec![],
        brauer_manin_obstruction_ty(),
    )?;
    add_axiom(env, "HasseNoetherBrauer", vec![], hasse_noether_brauer_ty())?;
    add_axiom(env, "AzumayaAlgebra", vec![], azumaya_algebra_ty())?;
    add_axiom(env, "VojtaDictionary", vec![], vojta_dictionary_ty())?;
    add_axiom(
        env,
        "FunctionFieldAnalogy",
        vec![],
        function_field_analogy_ty(),
    )?;
    add_axiom(env, "MasonStothers", vec![], mason_stothers_ty())?;
    add_axiom(env, "BakerMethod", vec![], baker_method_ty())?;
    add_axiom(env, "ThueEquation", vec![], thue_equation_ty())?;
    add_axiom(env, "SUnitEquation", vec![], s_unit_equation_ty())?;
    add_axiom(
        env,
        "LiouvilleApproximation",
        vec![],
        liouville_approximation_ty(),
    )?;
    add_axiom(env, "SubspaceTheorem", vec![], subspace_theorem_ty())?;
    add_axiom(
        env,
        "RationallyConnected",
        vec![],
        rationally_connected_ty(),
    )?;
    add_axiom(env, "BendAndBreak", vec![], bend_and_break_ty())?;
    add_axiom(env, "MinimalModel", vec![], minimal_model_ty())?;
    add_axiom(env, "FanoVariety", vec![], fano_variety_ty())?;
    add_axiom(
        env,
        "CampanaManinConjecture",
        vec![],
        campana_manin_conjecture_ty(),
    )?;
    add_axiom(
        env,
        "HermitianLineBundleArakelov",
        vec![],
        hermitian_line_bundle_arakelov_ty(),
    )?;
    add_axiom(
        env,
        "ArithmeticChowGroup",
        vec![],
        arithmetic_chow_group_ty(),
    )?;
    add_axiom(
        env,
        "ZhangAdmissiblePairing",
        vec![],
        zhang_admissible_pairing_ty(),
    )?;
    add_axiom(env, "BerkovichSpace", vec![], berkovich_space_ty())?;
    add_axiom(
        env,
        "BerkovichAnalytification",
        vec![],
        berkovich_analytification_ty(),
    )?;
    add_axiom(
        env,
        "GAGA_NonArchimedean",
        vec![],
        gaga_non_archimedean_ty(),
    )?;
    add_axiom(env, "BerkovichRetract", vec![], berkovich_retract_ty())?;
    add_axiom(env, "AnabelianVariety", vec![], anabelian_variety_ty())?;
    add_axiom(
        env,
        "GrothendieckAnabelianConjecture",
        vec![],
        grothendieck_anabelian_conjecture_ty(),
    )?;
    add_axiom(env, "MochizukiTheorem", vec![], mochizuki_theorem_ty())?;
    add_axiom(
        env,
        "EtaleFundamentalGroup",
        vec![],
        etale_fundamental_group_ty(),
    )?;
    add_axiom(env, "GaloisAction", vec![], galois_action_ty())?;
    Ok(())
}
/// Compute gcd(a, b) via Euclidean algorithm.
pub fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}
/// Compute the radical of n: product of distinct prime divisors.
pub fn radical(mut n: u64) -> u64 {
    if n == 0 {
        return 0;
    }
    let mut result = 1u64;
    let mut d = 2u64;
    while d * d <= n {
        if n % d == 0 {
            result *= d;
            while n % d == 0 {
                n /= d;
            }
        }
        d += 1;
    }
    if n > 1 {
        result *= n;
    }
    result
}
#[cfg(test)]
mod tests {
    use super::*;
    fn test_env() -> Environment {
        let mut env = Environment::new();
        build_diophantine_geometry_env(&mut env).expect("build_diophantine_geometry_env failed");
        env
    }
    #[test]
    fn test_height_functions_registered() {
        let env = test_env();
        assert!(env.get(&Name::str("HeightFunction")).is_some());
        assert!(env.get(&Name::str("WeilHeightMachine")).is_some());
        assert!(env.get(&Name::str("CanonicalHeight")).is_some());
        assert!(env.get(&Name::str("NorthcottProperty")).is_some());
        assert!(env.get(&Name::str("LogarithmicHeight")).is_some());
        assert!(env.get(&Name::str("AbsoluteLogarithmicHeight")).is_some());
        assert!(env.get(&Name::str("HeightPairing")).is_some());
        assert!(env.get(&Name::str("SzpiroInequality")).is_some());
    }
    #[test]
    fn test_mordell_weil_registered() {
        let env = test_env();
        assert!(env.get(&Name::str("MordellWeilGroup")).is_some());
        assert!(env.get(&Name::str("MordellWeilTheorem")).is_some());
        assert!(env.get(&Name::str("MordellWeilRank")).is_some());
        assert!(env.get(&Name::str("SelmerGroup")).is_some());
        assert!(env.get(&Name::str("ShafarevichTateGroup")).is_some());
        assert!(env.get(&Name::str("WeakMordellWeil")).is_some());
    }
    #[test]
    fn test_faltings_registered() {
        let env = test_env();
        assert!(env.get(&Name::str("FaltingsTheorem")).is_some());
        assert!(env.get(&Name::str("MordellConjecture")).is_some());
        assert!(env.get(&Name::str("CurveGenus")).is_some());
        assert!(env.get(&Name::str("JacobianVariety")).is_some());
        assert!(env.get(&Name::str("TorellisTheorem")).is_some());
    }
    #[test]
    fn test_abc_conjecture_registered() {
        let env = test_env();
        assert!(env.get(&Name::str("ABCTriple")).is_some());
        assert!(env.get(&Name::str("RadicalOfInteger")).is_some());
        assert!(env.get(&Name::str("ABCConjecture")).is_some());
        assert!(env.get(&Name::str("ABCQuality")).is_some());
    }
    #[test]
    fn test_arakelov_and_nevanlinna_registered() {
        let env = test_env();
        assert!(env.get(&Name::str("ArakelovDivisor")).is_some());
        assert!(env.get(&Name::str("ArithmeticIntersection")).is_some());
        assert!(env.get(&Name::str("FaltingsHeight")).is_some());
        assert!(env.get(&Name::str("NevanlinnaFirstMainThm")).is_some());
        assert!(env.get(&Name::str("NevanlinnaSecondMainThm")).is_some());
        assert!(env.get(&Name::str("VojtaConjecture")).is_some());
        assert!(env.get(&Name::str("RothTheorem")).is_some());
    }
    #[test]
    fn test_weil_height_computation() {
        let h = WeilHeight::new(vec![3, -4, 0, 5]);
        assert_eq!(h.naive_height(), 5);
        assert!((h.logarithmic_height() - (5.0_f64).ln()).abs() < 1e-10);
        assert!(h.northcott_property_holds());
    }
    #[test]
    fn test_mordell_weil_group_structure() {
        let mw = MordellWeilGroup::new(2, vec![2, 6]);
        assert_eq!(mw.rank, 2);
        assert_eq!(mw.torsion_size(), 12);
        assert!(!mw.is_finite());
        assert_eq!(mw.num_generators(), 4);
        let finite = MordellWeilGroup::new(0, vec![5]);
        assert!(finite.is_finite());
        assert_eq!(finite.torsion_size(), 5);
    }
    #[test]
    fn test_curve_genus_and_faltings() {
        let genus0 = ProjectiveCurve::new(0, "P^1");
        let genus1 = ProjectiveCurve::new(1, "Elliptic curve");
        let genus2 = ProjectiveCurve::new(2, "Genus-2 curve");
        assert!(!genus0.has_finitely_many_rational_points());
        assert!(!genus1.has_finitely_many_rational_points());
        assert!(genus2.has_finitely_many_rational_points());
        assert!(genus0.is_rational());
        assert!(genus1.is_elliptic());
        assert_eq!(genus2.riemann_roch_dim(4), 3);
    }
    #[test]
    fn test_heights_extended_registered() {
        let env = test_env();
        assert!(env.get(&Name::str("NeronTateHeight")).is_some());
        assert!(env.get(&Name::str("LehmerConjecture")).is_some());
        assert!(env.get(&Name::str("MahlerMeasure")).is_some());
        assert!(env.get(&Name::str("HeightBound")).is_some());
    }
    #[test]
    fn test_hasse_principle_registered() {
        let env = test_env();
        assert!(env.get(&Name::str("HassePrinciple")).is_some());
        assert!(env.get(&Name::str("WeakApproximation")).is_some());
        assert!(env.get(&Name::str("StrongApproximation")).is_some());
        assert!(env.get(&Name::str("LocalGlobalPrinciple")).is_some());
        assert!(env.get(&Name::str("ObstructionToHasse")).is_some());
    }
    #[test]
    fn test_chabauty_coleman_registered() {
        let env = test_env();
        assert!(env.get(&Name::str("ChabautyColeman")).is_some());
        assert!(env.get(&Name::str("PAdicIntegration")).is_some());
        assert!(env.get(&Name::str("ChabautyBoundValue")).is_some());
        assert!(env.get(&Name::str("TwistingMethod")).is_some());
    }
    #[test]
    fn test_descent_methods_registered() {
        let env = test_env();
        assert!(env.get(&Name::str("TwoDescent")).is_some());
        assert!(env.get(&Name::str("SelmerGroupExplicit")).is_some());
        assert!(env.get(&Name::str("VisibilityOfSelmer")).is_some());
        assert!(env.get(&Name::str("IsogenyDescent")).is_some());
    }
    #[test]
    fn test_brauer_manin_registered() {
        let env = test_env();
        assert!(env.get(&Name::str("BrauerGroup")).is_some());
        assert!(env.get(&Name::str("BrauerManinObstruction")).is_some());
        assert!(env.get(&Name::str("HasseNoetherBrauer")).is_some());
        assert!(env.get(&Name::str("AzumayaAlgebra")).is_some());
    }
    #[test]
    fn test_vojta_function_field_registered() {
        let env = test_env();
        assert!(env.get(&Name::str("VojtaDictionary")).is_some());
        assert!(env.get(&Name::str("FunctionFieldAnalogy")).is_some());
        assert!(env.get(&Name::str("MasonStothers")).is_some());
    }
    #[test]
    fn test_integral_points_registered() {
        let env = test_env();
        assert!(env.get(&Name::str("BakerMethod")).is_some());
        assert!(env.get(&Name::str("ThueEquation")).is_some());
        assert!(env.get(&Name::str("SUnitEquation")).is_some());
        assert!(env.get(&Name::str("LiouvilleApproximation")).is_some());
        assert!(env.get(&Name::str("SubspaceTheorem")).is_some());
    }
    #[test]
    fn test_mori_theory_registered() {
        let env = test_env();
        assert!(env.get(&Name::str("RationallyConnected")).is_some());
        assert!(env.get(&Name::str("BendAndBreak")).is_some());
        assert!(env.get(&Name::str("MinimalModel")).is_some());
        assert!(env.get(&Name::str("FanoVariety")).is_some());
        assert!(env.get(&Name::str("CampanaManinConjecture")).is_some());
    }
    #[test]
    fn test_arakelov_extended_registered() {
        let env = test_env();
        assert!(env.get(&Name::str("HermitianLineBundleArakelov")).is_some());
        assert!(env.get(&Name::str("ArithmeticChowGroup")).is_some());
        assert!(env.get(&Name::str("ZhangAdmissiblePairing")).is_some());
    }
    #[test]
    fn test_berkovich_registered() {
        let env = test_env();
        assert!(env.get(&Name::str("BerkovichSpace")).is_some());
        assert!(env.get(&Name::str("BerkovichAnalytification")).is_some());
        assert!(env.get(&Name::str("GAGA_NonArchimedean")).is_some());
        assert!(env.get(&Name::str("BerkovichRetract")).is_some());
    }
    #[test]
    fn test_anabelian_registered() {
        let env = test_env();
        assert!(env.get(&Name::str("AnabelianVariety")).is_some());
        assert!(env
            .get(&Name::str("GrothendieckAnabelianConjecture"))
            .is_some());
        assert!(env.get(&Name::str("MochizukiTheorem")).is_some());
        assert!(env.get(&Name::str("EtaleFundamentalGroup")).is_some());
        assert!(env.get(&Name::str("GaloisAction")).is_some());
    }
    #[test]
    fn test_naive_height_computer_rust() {
        let nh = NaiveHeightComputer::new(vec![3, -4, 5]);
        assert_eq!(nh.naive_height(), 5);
        assert_eq!(nh.number_of_coords(), 3);
        assert!(nh.log_height() > 0.0);
    }
    #[test]
    fn test_chabauty_bound_rust() {
        let cb = ChabautyBound::new(3, 1, 5);
        assert!(cb.is_applicable());
        assert!(cb.point_bound() > 0);
    }
    #[test]
    fn test_thue_solver_rust() {
        let ts = ThueSolver::new(vec![1, 0, 0, -2], 6);
        assert_eq!(ts.degree(), 3);
        let sols = ts.small_solutions(5);
        assert!(sols.len() <= 22);
    }
    #[test]
    fn test_selmer_group_estimator_rust() {
        let se = SelmerGroupEstimator::new(1, 2);
        let bound = se.rank_upper_bound();
        assert_eq!(bound, 4);
        assert!(!se.is_trivially_trivial());
        let trivial = SelmerGroupEstimator::new(0, 0);
        assert!(trivial.is_trivially_trivial());
        assert_eq!(trivial.rank_upper_bound(), 1);
    }
}
#[cfg(test)]
mod tests_diophantine_ext {
    use super::*;
    #[test]
    fn test_height_function() {
        let h = HeightFunction::new("Weil height", "Q").with_northcott();
        assert!(h.northcott);
        let wh = HeightFunction::weil_height_rational(3, 4);
        assert!((wh - 4.0f64.ln()).abs() < 1e-10);
        let ph = HeightFunction::projective_height(&[3, -5, 2]);
        assert!((ph - 5.0).abs() < 1e-10);
    }
    #[test]
    fn test_elliptic_curve_arithmetic() {
        let mut ec = EllipticCurveArithmetic::new(-1, 0);
        assert!(ec.is_smooth());
        assert!(ec.is_on_curve(0, 0));
        assert!(ec.is_on_curve(1, 0));
        ec.add_rational_point(0, 0);
        assert_eq!(ec.rational_points.len(), 1);
    }
    #[test]
    fn test_elliptic_j_invariant() {
        let ec = EllipticCurveArithmetic::new(1, 0);
        let j = ec.j_invariant().expect("j_invariant should succeed");
        assert!((j - (-1728.0)).abs() < 1.0, "j should be -1728, got {j}");
    }
    #[test]
    fn test_mordell_weil() {
        let mut mw = MordellWeilData::new("E", "Q")
            .with_rank(2)
            .with_torsion("Z/2Z");
        mw.add_generator("P1");
        mw.add_generator("P2");
        assert_eq!(mw.mw_rank(), 2);
        assert!(mw.structure_theorem().contains("Z^2"));
    }
    #[test]
    fn test_faltings() {
        let f = FaltingsData::new("C_4", 4).with_point_count(5);
        assert!(f.applies());
        assert!(f.finitely_many);
        assert!(f.faltings_statement().contains("finitely many"));
        let g1 = FaltingsData::new("E", 1);
        assert!(!g1.applies());
    }
}
#[cfg(test)]
mod tests_diophantine_ext2 {
    use super::*;
    #[test]
    fn test_arakelov_data() {
        let mut ad = ArakelovData::new("X/Z").with_arith_degree(2.5);
        ad.add_green_value(0.0, 1.0);
        ad.add_green_value(1.0, 0.5);
        assert!((ad.faltings_height_estimate() - 1.25).abs() < 1e-10);
        assert!(ad.noether_formula().contains("Noether"));
    }
    #[test]
    fn test_double_point() {
        let ec = EllipticCurveArithmetic::new(-1, 0);
        let res = ec.double_point(0, 0);
        assert!(res.is_none());
        let ec2 = EllipticCurveArithmetic::new(-5, 4);
        let res2 = ec2.double_point(1, 0);
        assert!(res2.is_none());
    }
}
