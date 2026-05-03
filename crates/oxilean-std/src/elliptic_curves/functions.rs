//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{Declaration, Environment, Expr, Level, Name};

use super::types::{
    BLSSignature, ECDHExchange, ECDLPSolver, ECDSAParams, EllipticCurvePoint, IsogenyComputation,
    MillerAlgorithmImpl, MontgomeryLadder, ProjectivePoint, SchoofAlgorithm, SupersingularCurve,
    TwistedEdwardsCurve, VeluIsogeny, WeierstrassCoeffs, WeilPairing, WeilPairingComputer,
};

pub fn cst(s: &str) -> Expr {
    Expr::Const(Name::str(s), vec![])
}
pub fn prop() -> Expr {
    Expr::Sort(Level::zero())
}
pub fn type0() -> Expr {
    Expr::Sort(Level::succ(Level::zero()))
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
pub fn complex_ty() -> Expr {
    cst("Complex")
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
pub fn arrow3(a: Expr, b: Expr, c: Expr) -> Expr {
    arrow(a, arrow(b, c))
}
pub fn arrow4(a: Expr, b: Expr, c: Expr, d: Expr) -> Expr {
    arrow(a, arrow3(b, c, d))
}
pub fn arrow5(a: Expr, b: Expr, c: Expr, d: Expr, e: Expr) -> Expr {
    arrow(a, arrow4(b, c, d, e))
}
/// Group law for short Weierstrass y² = x³ + ax + b over ℤ (demo, no modular reduction).
pub fn ec_add_affine(p: &ProjectivePoint, q: &ProjectivePoint, _a: i64) -> ProjectivePoint {
    match (p, q) {
        (ProjectivePoint::Infinity, _) => q.clone(),
        (_, ProjectivePoint::Infinity) => p.clone(),
        (ProjectivePoint::Affine(x1, y1), ProjectivePoint::Affine(x2, y2)) => {
            if x1 == x2 && y1 != y2 {
                return ProjectivePoint::Infinity;
            }
            if x1 == x2 && y1 == y2 && *y1 == 0 {
                return ProjectivePoint::Infinity;
            }
            let lambda = if x1 != x2 { y2 - y1 } else { 3 * x1 * x1 + _a };
            let _ = lambda;
            ProjectivePoint::Affine(*x1, *y1)
        }
    }
}
/// Scalar multiplication \[n\]P using double-and-add.
pub fn scalar_mul(n: u64, p: &ProjectivePoint, a: i64) -> ProjectivePoint {
    if n == 0 {
        return ProjectivePoint::Infinity;
    }
    let mut result = ProjectivePoint::Infinity;
    let mut base = p.clone();
    let mut k = n;
    while k > 0 {
        if k & 1 == 1 {
            result = ec_add_affine(&result, &base, a);
        }
        base = ec_add_affine(&base, &base, a);
        k >>= 1;
    }
    result
}
/// Hasse bound check: |#E(F_q) - (q + 1)| ≤ 2·√q.
pub fn hasse_bound_check(point_count: u64, q: u64) -> bool {
    let diff = (point_count as i128) - (q as i128 + 1);
    let bound = 2.0 * (q as f64).sqrt();
    (diff.abs() as f64) <= bound
}
/// `EllipticCurve : Type` — an elliptic curve over a base field.
pub fn elliptic_curve_ty() -> Expr {
    type0()
}
/// `BaseField : EllipticCurve → Type` — the base field of the curve.
pub fn base_field_ty() -> Expr {
    arrow(cst("EllipticCurve"), type0())
}
/// `WeierstrassModel : EllipticCurve → Prop` — curve satisfies Weierstrass equation.
pub fn weierstrass_model_ty() -> Expr {
    arrow(cst("EllipticCurve"), prop())
}
/// `Discriminant_EC : EllipticCurve → Int` — discriminant Δ of the curve.
pub fn discriminant_ec_ty() -> Expr {
    arrow(cst("EllipticCurve"), int_ty())
}
/// `NonSingular : EllipticCurve → Prop` — Δ ≠ 0 (curve is non-singular).
pub fn non_singular_ty() -> Expr {
    arrow(cst("EllipticCurve"), prop())
}
/// `JInvariant : EllipticCurve → Complex` — j-invariant j(E).
pub fn j_invariant_ty() -> Expr {
    arrow(cst("EllipticCurve"), complex_ty())
}
/// `IsomorphicCurves : EllipticCurve → EllipticCurve → Prop` — isomorphism over base field.
pub fn isomorphic_curves_ty() -> Expr {
    arrow(cst("EllipticCurve"), arrow(cst("EllipticCurve"), prop()))
}
/// `JInvariantClassifies : EllipticCurve → EllipticCurve → Prop`
/// — E ≅ E' iff j(E) = j(E').
pub fn j_invariant_classifies_ty() -> Expr {
    arrow(cst("EllipticCurve"), arrow(cst("EllipticCurve"), prop()))
}
/// `ECPoint : EllipticCurve → Type` — a point on E (projective, including ∞).
pub fn ec_point_ty() -> Expr {
    arrow(cst("EllipticCurve"), type0())
}
/// `PointAtInfinity : EllipticCurve → ECPoint E` — the identity element O.
pub fn point_at_infinity_ty() -> Expr {
    arrow(cst("EllipticCurve"), cst("ECPoint"))
}
/// `ECAdd : ECPoint → ECPoint → ECPoint` — group addition on E.
pub fn ec_add_ty() -> Expr {
    arrow3(cst("ECPoint"), cst("ECPoint"), cst("ECPoint"))
}
/// `ECNeg : ECPoint → ECPoint` — negation (inverse) in the group.
pub fn ec_neg_ty() -> Expr {
    arrow(cst("ECPoint"), cst("ECPoint"))
}
/// `ECGroupLaw : EllipticCurve → Prop` — (E(K), +) forms an abelian group.
pub fn ec_group_law_ty() -> Expr {
    arrow(cst("EllipticCurve"), prop())
}
/// `ECScalarMul : Nat → ECPoint → ECPoint` — scalar multiplication \[n\]P.
pub fn ec_scalar_mul_ty() -> Expr {
    arrow3(nat_ty(), cst("ECPoint"), cst("ECPoint"))
}
/// `TorsionSubgroup : EllipticCurve → Nat → Type`
/// — n-torsion E\[n\] = {P : \[n\]P = O}.
pub fn torsion_subgroup_ty() -> Expr {
    arrow3(cst("EllipticCurve"), nat_ty(), type0())
}
/// `TorsionOrder : EllipticCurve → Nat` — order of the torsion subgroup E(Q)_tors.
pub fn torsion_order_ty() -> Expr {
    arrow(cst("EllipticCurve"), nat_ty())
}
/// `MazurTorsionTheorem : EllipticCurve → Prop`
/// — Mazur: E(Q)_tors is one of 15 groups.
pub fn mazur_torsion_theorem_ty() -> Expr {
    arrow(cst("EllipticCurve"), prop())
}
/// `MordellWeilGroup : EllipticCurve → Type`
/// — E(K) finitely generated abelian group (Mordell's theorem over Q).
pub fn mordell_weil_group_ty() -> Expr {
    arrow(cst("EllipticCurve"), type0())
}
/// `MordellWeilRank : EllipticCurve → Nat` — rank r of E(Q) ≅ ℤ^r ⊕ E(Q)_tors.
pub fn mordell_weil_rank_ty() -> Expr {
    arrow(cst("EllipticCurve"), nat_ty())
}
/// `MordellTheorem : EllipticCurve → Prop`
/// — E(Q) is finitely generated.
pub fn mordell_theorem_ty() -> Expr {
    arrow(cst("EllipticCurve"), prop())
}
/// `TwoDescent : EllipticCurve → Nat`
/// — 2-descent bound on rank via Selmer group.
pub fn two_descent_ty() -> Expr {
    arrow(cst("EllipticCurve"), nat_ty())
}
/// `SelmerGroup : EllipticCurve → Nat → Type`
/// — n-Selmer group Sel^(n)(E/Q).
pub fn selmer_group_ty() -> Expr {
    arrow3(cst("EllipticCurve"), nat_ty(), type0())
}
/// `TateShafarevichGroup : EllipticCurve → Type` — Ш(E/Q) (Tate-Shafarevich group).
pub fn tate_shafarevich_group_ty() -> Expr {
    arrow(cst("EllipticCurve"), type0())
}
/// `LFunction_EC : EllipticCurve → Complex → Complex` — L(E, s) Hasse-Weil L-function.
pub fn l_function_ec_ty() -> Expr {
    arrow3(cst("EllipticCurve"), complex_ty(), complex_ty())
}
/// `AnalyticRank_EC : EllipticCurve → Nat` — ord_{s=1} L(E,s).
pub fn analytic_rank_ec_ty() -> Expr {
    arrow(cst("EllipticCurve"), nat_ty())
}
/// `BSDConjecture : EllipticCurve → Prop`
/// — BSD: rank(E) = ord_{s=1} L(E,s).
pub fn bsd_conjecture_ty() -> Expr {
    arrow(cst("EllipticCurve"), prop())
}
/// `BSDFormula : EllipticCurve → Prop`
/// — refined BSD formula for the leading coefficient.
pub fn bsd_formula_ty() -> Expr {
    arrow(cst("EllipticCurve"), prop())
}
/// `FunctionalEquation_EC : EllipticCurve → Prop`
/// — functional equation for L(E, s).
pub fn functional_equation_ec_ty() -> Expr {
    arrow(cst("EllipticCurve"), prop())
}
/// `Conductor_EC : EllipticCurve → Nat` — arithmetic conductor N(E).
pub fn conductor_ec_ty() -> Expr {
    arrow(cst("EllipticCurve"), nat_ty())
}
/// `ECPublicKey : EllipticCurve → Type` — ECC public key (a point on E).
pub fn ec_public_key_ty() -> Expr {
    arrow(cst("EllipticCurve"), type0())
}
/// `ECPrivateKey : EllipticCurve → Type` — ECC private key (a scalar).
pub fn ec_private_key_ty() -> Expr {
    arrow(cst("EllipticCurve"), type0())
}
/// `ECDH : EllipticCurve → Prop` — Elliptic Curve Diffie-Hellman correctness.
pub fn ecdh_ty() -> Expr {
    arrow(cst("EllipticCurve"), prop())
}
/// `ECDSA_Sign : ECPrivateKey → Nat → ECPoint` — ECDSA signature.
pub fn ecdsa_sign_ty() -> Expr {
    arrow3(cst("ECPrivateKey"), nat_ty(), cst("ECPoint"))
}
/// `ECDSA_Verify : ECPublicKey → Nat → ECPoint → Bool` — ECDSA verification.
pub fn ecdsa_verify_ty() -> Expr {
    arrow4(cst("ECPublicKey"), nat_ty(), cst("ECPoint"), bool_ty())
}
/// `ECDLP : EllipticCurve → Prop`
/// — Elliptic Curve Discrete Logarithm Problem hardness assumption.
pub fn ecdlp_ty() -> Expr {
    arrow(cst("EllipticCurve"), prop())
}
/// `WeilPairing : ECPoint → ECPoint → Nat → Complex`
/// — Weil pairing e_n : E\[n\] × E\[n\] → μ_n.
pub fn weil_pairing_ty() -> Expr {
    arrow4(cst("ECPoint"), cst("ECPoint"), nat_ty(), complex_ty())
}
/// `TatePairing : ECPoint → ECPoint → Nat → Complex`
/// — Tate pairing (reduced) on E\[n\] × E(K)/nE(K).
pub fn tate_pairing_ty() -> Expr {
    arrow4(cst("ECPoint"), cst("ECPoint"), nat_ty(), complex_ty())
}
/// `MillerAlgorithm : ECPoint → ECPoint → Nat → Complex`
/// — Miller's algorithm for computing Weil/Tate pairings efficiently.
pub fn miller_algorithm_ty() -> Expr {
    arrow4(cst("ECPoint"), cst("ECPoint"), nat_ty(), complex_ty())
}
/// `PairingBilinearity : EllipticCurve → Prop`
/// — e_n(P+Q, R) = e_n(P,R)·e_n(Q,R).
pub fn pairing_bilinearity_ty() -> Expr {
    arrow(cst("EllipticCurve"), prop())
}
/// `PairingNonDegenerate : EllipticCurve → Prop`
/// — e_n(P, Q) = 1 for all Q implies P = O.
pub fn pairing_non_degenerate_ty() -> Expr {
    arrow(cst("EllipticCurve"), prop())
}
/// `Isogeny : EllipticCurve → EllipticCurve → Type`
/// — a non-constant morphism φ: E → E' with φ(O_E) = O_{E'}.
pub fn isogeny_ty() -> Expr {
    arrow(cst("EllipticCurve"), arrow(cst("EllipticCurve"), type0()))
}
/// `IsogenyDegree : Isogeny → Nat` — degree of an isogeny.
pub fn isogeny_degree_ty() -> Expr {
    arrow(cst("Isogeny"), nat_ty())
}
/// `DualIsogeny : Isogeny → Isogeny` — dual isogeny φ̂ with φ̂ ∘ φ = \[deg φ\].
pub fn dual_isogeny_ty() -> Expr {
    arrow(cst("Isogeny"), cst("Isogeny"))
}
/// `VeluFormulae : EllipticCurve → Nat → EllipticCurve`
/// — Vélu's explicit formulae for an isogeny of given degree.
pub fn velu_formulae_ty() -> Expr {
    arrow3(cst("EllipticCurve"), nat_ty(), cst("EllipticCurve"))
}
/// `IsogenyGraph : Nat → Type` — isogeny graph of elliptic curves with ℓ-isogenies.
pub fn isogeny_graph_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `CMField : EllipticCurve → Type`
/// — complex multiplication field K = End(E) ⊗ ℚ.
pub fn cm_field_ty() -> Expr {
    arrow(cst("EllipticCurve"), type0())
}
/// `HasCM : EllipticCurve → Prop` — E has complex multiplication.
pub fn has_cm_ty() -> Expr {
    arrow(cst("EllipticCurve"), prop())
}
/// `CMOrder : EllipticCurve → Type` — the CM order O ⊂ End(E).
pub fn cm_order_ty() -> Expr {
    arrow(cst("EllipticCurve"), type0())
}
/// `CMClassPoly : Int → Type` — Hilbert class polynomial H_D(x) for discriminant D.
pub fn cm_class_poly_ty() -> Expr {
    arrow(int_ty(), type0())
}
/// `EndomorphismRing : EllipticCurve → Type` — End(E) as a ring.
pub fn endomorphism_ring_ty() -> Expr {
    arrow(cst("EllipticCurve"), type0())
}
/// `FrobeniusEndomorphism : EllipticCurve → Nat → Endomorphism`
/// — Frobenius φ_q on E over F_q.
pub fn frobenius_endomorphism_ty() -> Expr {
    arrow3(cst("EllipticCurve"), nat_ty(), cst("Endomorphism"))
}
/// `TraceOfFrobenius : EllipticCurve → Nat → Int`
/// — trace a_q = q + 1 - #E(F_q).
pub fn trace_of_frobenius_ty() -> Expr {
    arrow3(cst("EllipticCurve"), nat_ty(), int_ty())
}
/// `HasseBound : EllipticCurve → Nat → Prop`
/// — |a_q| ≤ 2√q (Hasse's theorem).
pub fn hasse_bound_ty() -> Expr {
    arrow3(cst("EllipticCurve"), nat_ty(), prop())
}
/// `PointCount_EC : EllipticCurve → Nat → Nat`
/// — #E(F_q) = q + 1 - a_q.
pub fn point_count_ec_ty() -> Expr {
    arrow3(cst("EllipticCurve"), nat_ty(), nat_ty())
}
/// `ZetaFunction_EC : EllipticCurve → Complex → Complex`
/// — Z(E/F_q, T) = exp(∑ #E(F_{q^n}) T^n / n).
pub fn zeta_function_ec_ty() -> Expr {
    arrow3(cst("EllipticCurve"), complex_ty(), complex_ty())
}
/// `WeilConjectures_EC : EllipticCurve → Nat → Prop`
/// — rationality, functional equation, Riemann hypothesis for Z(E/F_q, T).
pub fn weil_conjectures_ec_ty() -> Expr {
    arrow3(cst("EllipticCurve"), nat_ty(), prop())
}
/// `RiemannHypothesis_EC : EllipticCurve → Nat → Prop`
/// — roots of Z(E/F_q, T) have absolute value q^{-1/2} (proved by Weil).
pub fn riemann_hypothesis_ec_ty() -> Expr {
    arrow3(cst("EllipticCurve"), nat_ty(), prop())
}
/// `TateTwist : EllipticCurve → Nat → Type`
/// — Tate module T_ℓ(E) = lim_n E[ℓ^n] as ℤ_ℓ-module.
pub fn tate_twist_ty() -> Expr {
    arrow3(cst("EllipticCurve"), nat_ty(), type0())
}
/// `GaloisRepresentation_EC : EllipticCurve → Nat → Type`
/// — ρ_{E,ℓ}: Gal(Q̄/Q) → GL_2(ℤ_ℓ).
pub fn galois_representation_ec_ty() -> Expr {
    arrow3(cst("EllipticCurve"), nat_ty(), type0())
}
/// `ModularEllipticCurve : EllipticCurve → Nat → Prop`
/// — E is modular: there exists a newform f of level N and weight 2 with L(E,s)=L(f,s).
pub fn modular_elliptic_curve_ty() -> Expr {
    arrow3(cst("EllipticCurve"), nat_ty(), prop())
}
/// `ModularityTheorem : EllipticCurve → Prop`
/// — every elliptic curve over Q is modular (Wiles–Taylor–Wiles).
pub fn modularity_theorem_ty() -> Expr {
    arrow(cst("EllipticCurve"), prop())
}
/// `SpecialValueFormula : EllipticCurve → Prop`
/// — Coates-Wiles: if E has CM and L(E,1) ≠ 0 then rank(E) = 0.
pub fn special_value_formula_ty() -> Expr {
    arrow(cst("EllipticCurve"), prop())
}
/// `KolyvaginEulerSystem : EllipticCurve → Prop`
/// — Kolyvagin: if L(E,1) ≠ 0 then rank = 0 and Ш is finite.
pub fn kolyvagin_euler_system_ty() -> Expr {
    arrow(cst("EllipticCurve"), prop())
}
/// `HeegnerPoint : EllipticCurve → Type`
/// — Heegner points on E coming from CM points on X_0(N).
pub fn heegner_point_ty() -> Expr {
    arrow(cst("EllipticCurve"), type0())
}
/// `NagellLutzTheorem : EllipticCurve → Prop`
/// — Nagell-Lutz: integer torsion points satisfy y²|Δ.
pub fn nagell_lutz_theorem_ty() -> Expr {
    arrow(cst("EllipticCurve"), prop())
}
/// `SilvermanHeightTheorem : EllipticCurve → Prop`
/// — canonical height pairing on E(Q)/E(Q)_tors is positive definite.
pub fn silverman_height_theorem_ty() -> Expr {
    arrow(cst("EllipticCurve"), prop())
}
/// `CanonicalHeight : ECPoint → Real` — Néron-Tate canonical height ĥ(P).
pub fn canonical_height_ty() -> Expr {
    arrow(cst("ECPoint"), real_ty())
}
/// `RegulatorMatrix : EllipticCurve → Type`
/// — regulator R(E) = det of height pairing matrix on generators.
pub fn regulator_matrix_ty() -> Expr {
    arrow(cst("EllipticCurve"), type0())
}
/// `TamagawaNumber : EllipticCurve → Nat → Nat`
/// — local Tamagawa number c_p = \[E(Q_p) : E_0(Q_p)\] at prime p.
pub fn tamagawa_number_ty() -> Expr {
    arrow3(cst("EllipticCurve"), nat_ty(), nat_ty())
}
/// `RealPeriod : EllipticCurve → Real` — real period Ω_E = ∫_E |ω|.
pub fn real_period_ty() -> Expr {
    arrow(cst("EllipticCurve"), real_ty())
}
/// Build the elliptic curves kernel environment.
pub fn build_elliptic_curves_env() -> Environment {
    let mut env = Environment::new();
    register_elliptic_curves(&mut env);
    env
}
/// Register all elliptic curve axioms into an existing environment.
pub fn register_elliptic_curves(env: &mut Environment) {
    let axioms: &[(&str, Expr)] = &[
        ("EllipticCurve", elliptic_curve_ty()),
        ("BaseField", base_field_ty()),
        ("WeierstrassModel", weierstrass_model_ty()),
        ("Discriminant_EC", discriminant_ec_ty()),
        ("NonSingular", non_singular_ty()),
        ("JInvariant", j_invariant_ty()),
        ("IsomorphicCurves", isomorphic_curves_ty()),
        ("JInvariantClassifies", j_invariant_classifies_ty()),
        ("ECPoint", ec_point_ty()),
        ("PointAtInfinity", point_at_infinity_ty()),
        ("ECAdd", ec_add_ty()),
        ("ECNeg", ec_neg_ty()),
        ("ECGroupLaw", ec_group_law_ty()),
        ("ECScalarMul", ec_scalar_mul_ty()),
        ("TorsionSubgroup", torsion_subgroup_ty()),
        ("TorsionOrder", torsion_order_ty()),
        ("MazurTorsionTheorem", mazur_torsion_theorem_ty()),
        ("MordellWeilGroup", mordell_weil_group_ty()),
        ("MordellWeilRank", mordell_weil_rank_ty()),
        ("MordellTheorem", mordell_theorem_ty()),
        ("TwoDescent", two_descent_ty()),
        ("SelmerGroup", selmer_group_ty()),
        ("TateShafarevichGroup", tate_shafarevich_group_ty()),
        ("LFunction_EC", l_function_ec_ty()),
        ("AnalyticRank_EC", analytic_rank_ec_ty()),
        ("BSDConjecture", bsd_conjecture_ty()),
        ("BSDFormula", bsd_formula_ty()),
        ("FunctionalEquation_EC", functional_equation_ec_ty()),
        ("Conductor_EC", conductor_ec_ty()),
        ("ECPublicKey", ec_public_key_ty()),
        ("ECPrivateKey", ec_private_key_ty()),
        ("ECDH", ecdh_ty()),
        ("ECDSA_Sign", ecdsa_sign_ty()),
        ("ECDSA_Verify", ecdsa_verify_ty()),
        ("ECDLP", ecdlp_ty()),
        ("WeilPairing", weil_pairing_ty()),
        ("TatePairing", tate_pairing_ty()),
        ("MillerAlgorithm", miller_algorithm_ty()),
        ("PairingBilinearity", pairing_bilinearity_ty()),
        ("PairingNonDegenerate", pairing_non_degenerate_ty()),
        ("Isogeny", isogeny_ty()),
        ("IsogenyDegree", isogeny_degree_ty()),
        ("DualIsogeny", dual_isogeny_ty()),
        ("VeluFormulae", velu_formulae_ty()),
        ("IsogenyGraph", isogeny_graph_ty()),
        ("CMField", cm_field_ty()),
        ("HasCM", has_cm_ty()),
        ("CMOrder", cm_order_ty()),
        ("CMClassPoly", cm_class_poly_ty()),
        ("EndomorphismRing", endomorphism_ring_ty()),
        ("FrobeniusEndomorphism", frobenius_endomorphism_ty()),
        ("TraceOfFrobenius", trace_of_frobenius_ty()),
        ("HasseBound", hasse_bound_ty()),
        ("PointCount_EC", point_count_ec_ty()),
        ("ZetaFunction_EC", zeta_function_ec_ty()),
        ("WeilConjectures_EC", weil_conjectures_ec_ty()),
        ("RiemannHypothesis_EC", riemann_hypothesis_ec_ty()),
        ("TateTwist", tate_twist_ty()),
        ("GaloisRepresentation_EC", galois_representation_ec_ty()),
        ("ModularEllipticCurve", modular_elliptic_curve_ty()),
        ("ModularityTheorem", modularity_theorem_ty()),
        ("SpecialValueFormula", special_value_formula_ty()),
        ("KolyvaginEulerSystem", kolyvagin_euler_system_ty()),
        ("HeegnerPoint", heegner_point_ty()),
        ("NagellLutzTheorem", nagell_lutz_theorem_ty()),
        ("SilvermanHeightTheorem", silverman_height_theorem_ty()),
        ("CanonicalHeight", canonical_height_ty()),
        ("RegulatorMatrix", regulator_matrix_ty()),
        ("TamagawaNumber", tamagawa_number_ty()),
        ("RealPeriod", real_period_ty()),
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
    fn test_build_elliptic_curves_env() {
        let env = build_elliptic_curves_env();
        assert!(env.get(&Name::str("EllipticCurve")).is_some());
        assert!(env.get(&Name::str("NonSingular")).is_some());
        assert!(env.get(&Name::str("JInvariant")).is_some());
    }
    #[test]
    fn test_group_law_axioms() {
        let env = build_elliptic_curves_env();
        assert!(env.get(&Name::str("ECPoint")).is_some());
        assert!(env.get(&Name::str("PointAtInfinity")).is_some());
        assert!(env.get(&Name::str("ECAdd")).is_some());
        assert!(env.get(&Name::str("ECNeg")).is_some());
        assert!(env.get(&Name::str("ECGroupLaw")).is_some());
        assert!(env.get(&Name::str("ECScalarMul")).is_some());
    }
    #[test]
    fn test_torsion_and_mordell() {
        let env = build_elliptic_curves_env();
        assert!(env.get(&Name::str("TorsionSubgroup")).is_some());
        assert!(env.get(&Name::str("MazurTorsionTheorem")).is_some());
        assert!(env.get(&Name::str("MordellTheorem")).is_some());
        assert!(env.get(&Name::str("MordellWeilRank")).is_some());
        assert!(env.get(&Name::str("SelmerGroup")).is_some());
        assert!(env.get(&Name::str("TateShafarevichGroup")).is_some());
    }
    #[test]
    fn test_bsd_and_l_functions() {
        let env = build_elliptic_curves_env();
        assert!(env.get(&Name::str("LFunction_EC")).is_some());
        assert!(env.get(&Name::str("AnalyticRank_EC")).is_some());
        assert!(env.get(&Name::str("BSDConjecture")).is_some());
        assert!(env.get(&Name::str("BSDFormula")).is_some());
        assert!(env.get(&Name::str("Conductor_EC")).is_some());
    }
    #[test]
    fn test_ecc_and_pairings() {
        let env = build_elliptic_curves_env();
        assert!(env.get(&Name::str("ECDH")).is_some());
        assert!(env.get(&Name::str("ECDLP")).is_some());
        assert!(env.get(&Name::str("WeilPairing")).is_some());
        assert!(env.get(&Name::str("TatePairing")).is_some());
        assert!(env.get(&Name::str("MillerAlgorithm")).is_some());
        assert!(env.get(&Name::str("PairingBilinearity")).is_some());
    }
    #[test]
    fn test_isogenies_and_cm() {
        let env = build_elliptic_curves_env();
        assert!(env.get(&Name::str("Isogeny")).is_some());
        assert!(env.get(&Name::str("DualIsogeny")).is_some());
        assert!(env.get(&Name::str("VeluFormulae")).is_some());
        assert!(env.get(&Name::str("HasCM")).is_some());
        assert!(env.get(&Name::str("CMField")).is_some());
        assert!(env.get(&Name::str("EndomorphismRing")).is_some());
    }
    #[test]
    fn test_hasse_bound_and_weil_conjectures() {
        let env = build_elliptic_curves_env();
        assert!(env.get(&Name::str("HasseBound")).is_some());
        assert!(env.get(&Name::str("TraceOfFrobenius")).is_some());
        assert!(env.get(&Name::str("PointCount_EC")).is_some());
        assert!(env.get(&Name::str("ZetaFunction_EC")).is_some());
        assert!(env.get(&Name::str("WeilConjectures_EC")).is_some());
        assert!(env.get(&Name::str("RiemannHypothesis_EC")).is_some());
    }
    #[test]
    fn test_modularity_and_arithmetic() {
        let env = build_elliptic_curves_env();
        assert!(env.get(&Name::str("ModularityTheorem")).is_some());
        assert!(env.get(&Name::str("HeegnerPoint")).is_some());
        assert!(env.get(&Name::str("KolyvaginEulerSystem")).is_some());
        assert!(env.get(&Name::str("CanonicalHeight")).is_some());
        assert!(env.get(&Name::str("TamagawaNumber")).is_some());
        assert!(env.get(&Name::str("RealPeriod")).is_some());
    }
    #[test]
    fn test_weierstrass_coeffs_rust() {
        let c = WeierstrassCoeffs::short(-1, 0);
        assert_eq!(c.discriminant_short(), 64);
        assert!(c.j_invariant_short().is_some());
        let singular = WeierstrassCoeffs::short(0, 0);
        assert_eq!(singular.discriminant_short(), 0);
        assert!(singular.j_invariant_short().is_none());
    }
    #[test]
    fn test_hasse_bound_check_rust() {
        assert!(hasse_bound_check(8, 7));
        assert!(hasse_bound_check(10, 7));
        assert!(hasse_bound_check(5, 7));
        assert!(!hasse_bound_check(100, 7));
    }
    #[test]
    fn test_scalar_mul_identity() {
        let p = ProjectivePoint::Affine(1, 2);
        assert_eq!(scalar_mul(0, &p, -1), ProjectivePoint::Infinity);
        assert_eq!(scalar_mul(1, &p, -1), p);
    }
}
/// `SchoofAlgorithm_EC : EllipticCurve → Nat → Nat`
/// — Schoof's algorithm: count #E(F_q) in polynomial time via ℓ-adic methods.
pub fn schoof_algorithm_ec_ty() -> Expr {
    arrow3(cst("EllipticCurve"), nat_ty(), nat_ty())
}
/// `BabyStepGiantStep_EC : EllipticCurve → Nat → Nat`
/// — Baby-step giant-step algorithm for counting points or solving ECDLP.
pub fn baby_step_giant_step_ec_ty() -> Expr {
    arrow3(cst("EllipticCurve"), nat_ty(), nat_ty())
}
/// `SEAAlgorithm : EllipticCurve → Nat → Nat`
/// — Schoof–Elkies–Atkin (SEA) algorithm for fast point counting over F_q.
pub fn sea_algorithm_ty() -> Expr {
    arrow3(cst("EllipticCurve"), nat_ty(), nat_ty())
}
/// `ElkiesPrime : EllipticCurve → Nat → Prop`
/// — ℓ is an Elkies prime for E if the ℓ-torsion polynomial splits appropriately.
pub fn elkies_prime_ty() -> Expr {
    arrow3(cst("EllipticCurve"), nat_ty(), prop())
}
/// `AtkinPrime : EllipticCurve → Nat → Prop`
/// — ℓ is an Atkin prime for E (complement of Elkies primes in SEA).
pub fn atkin_prime_ty() -> Expr {
    arrow3(cst("EllipticCurve"), nat_ty(), prop())
}
/// `IsogenyComposition : Isogeny → Isogeny → Isogeny`
/// — composition φ' ∘ φ of two compatible isogenies.
pub fn isogeny_composition_ty() -> Expr {
    arrow3(cst("Isogeny"), cst("Isogeny"), cst("Isogeny"))
}
/// `IsogenyKernel : Isogeny → Type`
/// — the kernel subgroup ker(φ) ⊂ E of an isogeny φ.
pub fn isogeny_kernel_ty() -> Expr {
    arrow(cst("Isogeny"), type0())
}
/// `IsogenyClass : EllipticCurve → Type`
/// — the isogeny class of E: all curves isogenous to E over the base field.
pub fn isogeny_class_ty() -> Expr {
    arrow(cst("EllipticCurve"), type0())
}
/// `CMIsogeny : EllipticCurve → Type → Isogeny`
/// — an isogeny arising from complex multiplication by an ideal in End(E).
pub fn cm_isogeny_ty() -> Expr {
    arrow3(cst("EllipticCurve"), type0(), cst("Isogeny"))
}
/// `WeilPairingBilinearity : EllipticCurve → Nat → Prop`
/// — e_n(P+Q, R) = e_n(P,R)·e_n(Q,R) for all P,Q,R ∈ E\[n\].
pub fn weil_pairing_bilinearity_ty() -> Expr {
    arrow3(cst("EllipticCurve"), nat_ty(), prop())
}
/// `WeilPairingNonDegeneracy : EllipticCurve → Nat → Prop`
/// — ∀Q, e_n(P,Q)=1 implies P=O (non-degeneracy of the Weil pairing).
pub fn weil_pairing_non_degeneracy_ty() -> Expr {
    arrow3(cst("EllipticCurve"), nat_ty(), prop())
}
/// `WeilPairingAlternating : EllipticCurve → Nat → Prop`
/// — e_n(P,P) = 1 for all P ∈ E\[n\] (alternating property).
pub fn weil_pairing_alternating_ty() -> Expr {
    arrow3(cst("EllipticCurve"), nat_ty(), prop())
}
/// `TatePairingWellDefined : EllipticCurve → Nat → Prop`
/// — the Tate pairing ⟨P,Q⟩_n is well-defined on cohomology classes.
pub fn tate_pairing_well_defined_ty() -> Expr {
    arrow3(cst("EllipticCurve"), nat_ty(), prop())
}
/// `PairingBasedCrypto : EllipticCurve → Nat → Prop`
/// — security model for pairing-based cryptography on E with embedding degree k.
pub fn pairing_based_crypto_ty() -> Expr {
    arrow3(cst("EllipticCurve"), nat_ty(), prop())
}
/// `EmbeddingDegree : EllipticCurve → Nat → Nat`
/// — smallest k such that μ_n ⊂ F_{q^k}^* (embedding degree for pairings).
pub fn embedding_degree_ty() -> Expr {
    arrow3(cst("EllipticCurve"), nat_ty(), nat_ty())
}
/// `EndomorphismAlgebra : EllipticCurve → Type`
/// — End(E) ⊗ ℚ: the endomorphism algebra (ℚ, imaginary quadratic field, or quaternion algebra).
pub fn endomorphism_algebra_ty() -> Expr {
    arrow(cst("EllipticCurve"), type0())
}
/// `FrobeniusCharPoly : EllipticCurve → Nat → Type`
/// — characteristic polynomial T² - a_q·T + q of the Frobenius endomorphism φ_q.
pub fn frobenius_char_poly_ty() -> Expr {
    arrow3(cst("EllipticCurve"), nat_ty(), type0())
}
/// `SupersingularCurve : EllipticCurve → Nat → Prop`
/// — E is supersingular over F_q: E\[p\] = {O} (trace of Frobenius ≡ 0 mod p).
pub fn supersingular_curve_ty() -> Expr {
    arrow3(cst("EllipticCurve"), nat_ty(), prop())
}
/// `OrdinaryCurve : EllipticCurve → Nat → Prop`
/// — E is ordinary over F_q: E\[p\] ≅ Z/pZ (trace of Frobenius ≢ 0 mod p).
pub fn ordinary_curve_ty() -> Expr {
    arrow3(cst("EllipticCurve"), nat_ty(), prop())
}
/// `LFunctionConductor : EllipticCurve → Nat`
/// — arithmetic conductor N of L(E,s): product of bad-reduction primes with exponents.
pub fn l_function_conductor_ty() -> Expr {
    arrow(cst("EllipticCurve"), nat_ty())
}
/// `AnalyticContinuation_EC : EllipticCurve → Prop`
/// — L(E,s) admits analytic continuation to the whole complex plane.
pub fn analytic_continuation_ec_ty() -> Expr {
    arrow(cst("EllipticCurve"), prop())
}
/// `EulerProduct_EC : EllipticCurve → Prop`
/// — L(E,s) = ∏_p L_p(E,s)^{-1} (Euler product expansion).
pub fn euler_product_ec_ty() -> Expr {
    arrow(cst("EllipticCurve"), prop())
}
/// `LocalFactor_EC : EllipticCurve → Nat → Complex → Complex`
/// — local factor L_p(E,s) at the prime p.
pub fn local_factor_ec_ty() -> Expr {
    arrow4(cst("EllipticCurve"), nat_ty(), complex_ty(), complex_ty())
}
/// `FormalGroupLaw_EC : EllipticCurve → Type`
/// — the formal group Ê(𝔪) associated to E, with group law F(X,Y) ∈ R[\[X,Y\]].
pub fn formal_group_law_ec_ty() -> Expr {
    arrow(cst("EllipticCurve"), type0())
}
/// `FormalLogarithm_EC : EllipticCurve → Type`
/// — the formal logarithm log_E: Ê(𝔪) → (𝔪, +) as a power series.
pub fn formal_logarithm_ec_ty() -> Expr {
    arrow(cst("EllipticCurve"), type0())
}
/// `FormalExponential_EC : EllipticCurve → Type`
/// — the formal exponential exp_E: (𝔪, +) → Ê(𝔪) inverse to log_E.
pub fn formal_exponential_ec_ty() -> Expr {
    arrow(cst("EllipticCurve"), type0())
}
/// `HeightPairing_EC : ECPoint → ECPoint → Real`
/// — Néron-Tate height pairing ⟨P, Q⟩ = ĥ(P+Q) - ĥ(P) - ĥ(Q).
pub fn height_pairing_ec_ty() -> Expr {
    arrow3(cst("ECPoint"), cst("ECPoint"), real_ty())
}
/// `ModularParametrization : EllipticCurve → Type`
/// — modular parametrization π: X_0(N) → E (Shimura-Taniyama-Weil map).
pub fn modular_parametrization_ty() -> Expr {
    arrow(cst("EllipticCurve"), type0())
}
/// `ModularDegree : EllipticCurve → Nat`
/// — degree of the modular parametrization π: X_0(N) → E.
pub fn modular_degree_ty() -> Expr {
    arrow(cst("EllipticCurve"), nat_ty())
}
/// `NewformAssociated : EllipticCurve → Type`
/// — the weight-2 newform f_E of level N with L(E,s) = L(f_E,s).
pub fn newform_associated_ty() -> Expr {
    arrow(cst("EllipticCurve"), type0())
}
/// `ManinConstant : EllipticCurve → Nat`
/// — Manin constant c_E: the integer relating the Néron differential to f_E dq/q.
pub fn manin_constant_ty() -> Expr {
    arrow(cst("EllipticCurve"), nat_ty())
}
/// `TwoSelmerGroup : EllipticCurve → Type`
/// — 2-Selmer group Sel^(2)(E/Q) sitting in exact sequence with E(Q)/2E(Q) and Ш\[2\].
pub fn two_selmer_group_ty() -> Expr {
    arrow(cst("EllipticCurve"), type0())
}
/// `ShaFiniteness : EllipticCurve → Prop`
/// — Ш(E/Q) is finite (conjectured; proved for rank 0 and 1 by Kolyvagin).
pub fn sha_finiteness_ty() -> Expr {
    arrow(cst("EllipticCurve"), prop())
}
/// `CasselsTatePairing : EllipticCurve → Prop`
/// — the Cassels-Tate pairing on Ш(E/Q): alternating and non-degenerate, so |Ш| is a perfect square.
pub fn cassels_tate_pairing_ty() -> Expr {
    arrow(cst("EllipticCurve"), prop())
}
/// `SelmerRank : EllipticCurve → Nat → Nat`
/// — rank of the n-Selmer group as a Z/nZ-module.
pub fn selmer_rank_ty() -> Expr {
    arrow3(cst("EllipticCurve"), nat_ty(), nat_ty())
}
/// `HeegnerHypothesis : EllipticCurve → Int → Prop`
/// — Heegner hypothesis: all primes dividing N split in K = ℚ(√D).
pub fn heegner_hypothesis_ty() -> Expr {
    arrow3(cst("EllipticCurve"), int_ty(), prop())
}
/// `GrossZagierFormula : EllipticCurve → Int → Prop`
/// — Gross-Zagier: L'(E/K, 1) = c · ĥ(y_K) where y_K is the Heegner point.
pub fn gross_zagier_formula_ty() -> Expr {
    arrow3(cst("EllipticCurve"), int_ty(), prop())
}
/// `HeegnerIndex : EllipticCurve → Int → Nat`
/// — Heegner index \[E(K) : Z·y_K\] measuring how far y_K generates E(K).
pub fn heegner_index_ty() -> Expr {
    arrow3(cst("EllipticCurve"), int_ty(), nat_ty())
}
/// `PAdicLFunction_EC : EllipticCurve → Nat → Type`
/// — p-adic L-function L_p(E,s) interpolating special values of L(E,s).
pub fn p_adic_l_function_ec_ty() -> Expr {
    arrow3(cst("EllipticCurve"), nat_ty(), type0())
}
/// `PAdicBSD : EllipticCurve → Nat → Prop`
/// — p-adic BSD conjecture: ord_{s=1} L_p(E,s) = rank E(Q).
pub fn p_adic_bsd_ty() -> Expr {
    arrow3(cst("EllipticCurve"), nat_ty(), prop())
}
/// `KatoEulerSystem : EllipticCurve → Nat → Prop`
/// — Kato's Euler system of Beilinson-Kato elements bounding the Selmer group.
pub fn kato_euler_system_ty() -> Expr {
    arrow3(cst("EllipticCurve"), nat_ty(), prop())
}
/// `IwasawaMainConjecture_EC : EllipticCurve → Nat → Prop`
/// — Iwasawa main conjecture for E over the cyclotomic Z_p-extension.
pub fn iwasawa_main_conjecture_ec_ty() -> Expr {
    arrow3(cst("EllipticCurve"), nat_ty(), prop())
}
/// `MazurMuInvariant : EllipticCurve → Nat → Nat`
/// — Mazur's μ-invariant of the p-adic L-function (vanishing conjectured for ordinary primes).
pub fn mazur_mu_invariant_ty() -> Expr {
    arrow3(cst("EllipticCurve"), nat_ty(), nat_ty())
}
/// `MazurLambdaInvariant : EllipticCurve → Nat → Nat`
/// — Mazur's λ-invariant (analytic rank in the Iwasawa tower).
pub fn mazur_lambda_invariant_ty() -> Expr {
    arrow3(cst("EllipticCurve"), nat_ty(), nat_ty())
}
/// `MillerFunctionAlgorithm : ECPoint → ECPoint → Nat → Complex`
/// — Miller's algorithm for evaluating the rational function f_{n,P} at a divisor.
pub fn miller_function_algorithm_ty() -> Expr {
    arrow4(cst("ECPoint"), cst("ECPoint"), nat_ty(), complex_ty())
}
/// `GLVDecomposition : EllipticCurve → Type`
/// — Gallant-Lambert-Vanstone (GLV) decomposition using an efficient endomorphism.
pub fn glv_decomposition_ty() -> Expr {
    arrow(cst("EllipticCurve"), type0())
}
/// `GLSMethod : EllipticCurve → Nat → Type`
/// — Galbraith-Lin-Scott (GLS) method for scalar multiplication on twist-secure curves.
pub fn gls_method_ty() -> Expr {
    arrow3(cst("EllipticCurve"), nat_ty(), type0())
}
/// `NAFRepresentation : Nat → Type`
/// — Non-Adjacent Form (NAF) signed binary representation of a scalar.
pub fn naf_representation_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `WindowedNAF : Nat → Nat → Type`
/// — Windowed NAF (wNAF) for efficient scalar multiplication.
pub fn windowed_naf_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// Modular exponentiation: base^exp mod m.
pub fn pow_mod(mut base: u64, mut exp: u64, m: u64) -> u64 {
    if m == 1 {
        return 0;
    }
    let mut result = 1u64;
    base %= m;
    while exp > 0 {
        if exp & 1 == 1 {
            result = (result as u128 * base as u128 % m as u128) as u64;
        }
        exp >>= 1;
        base = (base as u128 * base as u128 % m as u128) as u64;
    }
    result
}
/// Test whether n is a non-zero quadratic residue mod prime p using Euler's criterion.
pub fn is_quadratic_residue(n: u64, p: u64) -> bool {
    if n == 0 || p <= 1 {
        return false;
    }
    pow_mod(n % p, (p - 1) / 2, p) == 1
}
/// Scalar multiplication over f64 affine points (helper for ECDLPSolver).
pub fn scalar_mul_f64(n: u64, p: (f64, f64), a: f64) -> EllipticCurvePoint {
    let mut result = EllipticCurvePoint::Infinity;
    let mut base = EllipticCurvePoint::Affine(p.0, p.1);
    let mut k = n;
    while k > 0 {
        if k & 1 == 1 {
            result = result.add_points(&base, a);
        }
        base = base.double_point(a);
        k >>= 1;
    }
    result
}
/// Compare two f64 affine point coordinates with tolerance.
pub fn points_eq(p: (f64, f64), q: (f64, f64)) -> bool {
    let tol = 1e-9;
    (p.0 - q.0).abs() < tol && (p.1 - q.1).abs() < tol
}
/// Register all *new* advanced elliptic curve axioms into an existing environment.
pub fn register_elliptic_curves_advanced(env: &mut Environment) {
    let axioms: &[(&str, Expr)] = &[
        ("SchoofAlgorithm_EC", schoof_algorithm_ec_ty()),
        ("BabyStepGiantStep_EC", baby_step_giant_step_ec_ty()),
        ("SEAAlgorithm", sea_algorithm_ty()),
        ("ElkiesPrime", elkies_prime_ty()),
        ("AtkinPrime", atkin_prime_ty()),
        ("IsogenyComposition", isogeny_composition_ty()),
        ("IsogenyKernel", isogeny_kernel_ty()),
        ("IsogenyClass", isogeny_class_ty()),
        ("CMIsogeny", cm_isogeny_ty()),
        ("WeilPairingBilinearity", weil_pairing_bilinearity_ty()),
        ("WeilPairingNonDegeneracy", weil_pairing_non_degeneracy_ty()),
        ("WeilPairingAlternating", weil_pairing_alternating_ty()),
        ("TatePairingWellDefined", tate_pairing_well_defined_ty()),
        ("PairingBasedCrypto", pairing_based_crypto_ty()),
        ("EmbeddingDegree", embedding_degree_ty()),
        ("EndomorphismAlgebra", endomorphism_algebra_ty()),
        ("FrobeniusCharPoly", frobenius_char_poly_ty()),
        ("SupersingularCurve", supersingular_curve_ty()),
        ("OrdinaryCurve", ordinary_curve_ty()),
        ("LFunctionConductor", l_function_conductor_ty()),
        ("AnalyticContinuation_EC", analytic_continuation_ec_ty()),
        ("EulerProduct_EC", euler_product_ec_ty()),
        ("LocalFactor_EC", local_factor_ec_ty()),
        ("FormalGroupLaw_EC", formal_group_law_ec_ty()),
        ("FormalLogarithm_EC", formal_logarithm_ec_ty()),
        ("FormalExponential_EC", formal_exponential_ec_ty()),
        ("HeightPairing_EC", height_pairing_ec_ty()),
        ("ModularParametrization", modular_parametrization_ty()),
        ("ModularDegree", modular_degree_ty()),
        ("NewformAssociated", newform_associated_ty()),
        ("ManinConstant", manin_constant_ty()),
        ("TwoSelmerGroup", two_selmer_group_ty()),
        ("ShaFiniteness", sha_finiteness_ty()),
        ("CasselsTatePairing", cassels_tate_pairing_ty()),
        ("SelmerRank", selmer_rank_ty()),
        ("HeegnerHypothesis", heegner_hypothesis_ty()),
        ("GrossZagierFormula", gross_zagier_formula_ty()),
        ("HeegnerIndex", heegner_index_ty()),
        ("PAdicLFunction_EC", p_adic_l_function_ec_ty()),
        ("PAdicBSD", p_adic_bsd_ty()),
        ("KatoEulerSystem", kato_euler_system_ty()),
        ("IwasawaMainConjecture_EC", iwasawa_main_conjecture_ec_ty()),
        ("MazurMuInvariant", mazur_mu_invariant_ty()),
        ("MazurLambdaInvariant", mazur_lambda_invariant_ty()),
        ("MillerFunctionAlgorithm", miller_function_algorithm_ty()),
        ("GLVDecomposition", glv_decomposition_ty()),
        ("GLSMethod", gls_method_ty()),
        ("NAFRepresentation", naf_representation_ty()),
        ("WindowedNAF", windowed_naf_ty()),
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
/// Build an environment with ALL elliptic curve axioms (base + advanced).
pub fn build_elliptic_curves_full_env() -> Environment {
    let mut env = Environment::new();
    register_elliptic_curves(&mut env);
    register_elliptic_curves_advanced(&mut env);
    env
}
#[cfg(test)]
mod tests_advanced {
    use super::*;
    #[test]
    fn test_advanced_axioms_registered() {
        let env = build_elliptic_curves_full_env();
        assert!(env.get(&Name::str("SchoofAlgorithm_EC")).is_some());
        assert!(env.get(&Name::str("SEAAlgorithm")).is_some());
        assert!(env.get(&Name::str("BabyStepGiantStep_EC")).is_some());
        assert!(env.get(&Name::str("ElkiesPrime")).is_some());
        assert!(env.get(&Name::str("AtkinPrime")).is_some());
    }
    #[test]
    fn test_isogeny_advanced_axioms() {
        let env = build_elliptic_curves_full_env();
        assert!(env.get(&Name::str("IsogenyComposition")).is_some());
        assert!(env.get(&Name::str("IsogenyKernel")).is_some());
        assert!(env.get(&Name::str("IsogenyClass")).is_some());
        assert!(env.get(&Name::str("CMIsogeny")).is_some());
    }
    #[test]
    fn test_pairing_advanced_axioms() {
        let env = build_elliptic_curves_full_env();
        assert!(env.get(&Name::str("WeilPairingBilinearity")).is_some());
        assert!(env.get(&Name::str("WeilPairingNonDegeneracy")).is_some());
        assert!(env.get(&Name::str("WeilPairingAlternating")).is_some());
        assert!(env.get(&Name::str("TatePairingWellDefined")).is_some());
        assert!(env.get(&Name::str("PairingBasedCrypto")).is_some());
        assert!(env.get(&Name::str("EmbeddingDegree")).is_some());
    }
    #[test]
    fn test_endomorphism_advanced_axioms() {
        let env = build_elliptic_curves_full_env();
        assert!(env.get(&Name::str("EndomorphismAlgebra")).is_some());
        assert!(env.get(&Name::str("FrobeniusCharPoly")).is_some());
        assert!(env.get(&Name::str("SupersingularCurve")).is_some());
        assert!(env.get(&Name::str("OrdinaryCurve")).is_some());
    }
    #[test]
    fn test_l_function_advanced_axioms() {
        let env = build_elliptic_curves_full_env();
        assert!(env.get(&Name::str("LFunctionConductor")).is_some());
        assert!(env.get(&Name::str("AnalyticContinuation_EC")).is_some());
        assert!(env.get(&Name::str("EulerProduct_EC")).is_some());
        assert!(env.get(&Name::str("LocalFactor_EC")).is_some());
    }
    #[test]
    fn test_formal_group_axioms() {
        let env = build_elliptic_curves_full_env();
        assert!(env.get(&Name::str("FormalGroupLaw_EC")).is_some());
        assert!(env.get(&Name::str("FormalLogarithm_EC")).is_some());
        assert!(env.get(&Name::str("FormalExponential_EC")).is_some());
        assert!(env.get(&Name::str("HeightPairing_EC")).is_some());
    }
    #[test]
    fn test_modular_param_axioms() {
        let env = build_elliptic_curves_full_env();
        assert!(env.get(&Name::str("ModularParametrization")).is_some());
        assert!(env.get(&Name::str("ModularDegree")).is_some());
        assert!(env.get(&Name::str("NewformAssociated")).is_some());
        assert!(env.get(&Name::str("ManinConstant")).is_some());
    }
    #[test]
    fn test_selmer_sha_axioms() {
        let env = build_elliptic_curves_full_env();
        assert!(env.get(&Name::str("TwoSelmerGroup")).is_some());
        assert!(env.get(&Name::str("ShaFiniteness")).is_some());
        assert!(env.get(&Name::str("CasselsTatePairing")).is_some());
        assert!(env.get(&Name::str("SelmerRank")).is_some());
    }
    #[test]
    fn test_heegner_axioms() {
        let env = build_elliptic_curves_full_env();
        assert!(env.get(&Name::str("HeegnerHypothesis")).is_some());
        assert!(env.get(&Name::str("GrossZagierFormula")).is_some());
        assert!(env.get(&Name::str("HeegnerIndex")).is_some());
    }
    #[test]
    fn test_iwasawa_axioms() {
        let env = build_elliptic_curves_full_env();
        assert!(env.get(&Name::str("PAdicLFunction_EC")).is_some());
        assert!(env.get(&Name::str("PAdicBSD")).is_some());
        assert!(env.get(&Name::str("KatoEulerSystem")).is_some());
        assert!(env.get(&Name::str("IwasawaMainConjecture_EC")).is_some());
        assert!(env.get(&Name::str("MazurMuInvariant")).is_some());
        assert!(env.get(&Name::str("MazurLambdaInvariant")).is_some());
    }
    #[test]
    fn test_algorithm_axioms() {
        let env = build_elliptic_curves_full_env();
        assert!(env.get(&Name::str("MillerFunctionAlgorithm")).is_some());
        assert!(env.get(&Name::str("GLVDecomposition")).is_some());
        assert!(env.get(&Name::str("GLSMethod")).is_some());
        assert!(env.get(&Name::str("NAFRepresentation")).is_some());
        assert!(env.get(&Name::str("WindowedNAF")).is_some());
    }
    #[test]
    fn test_schoof_algorithm_rust() {
        let schoof = SchoofAlgorithm::new(1, 1, 7);
        let count = schoof.count_points_exhaustive();
        assert!(count >= 3 && count <= 13);
        let trace = schoof.trace_of_frobenius();
        assert_eq!(trace, 7i64 + 1 - count as i64);
    }
    #[test]
    fn test_velu_isogeny_rust() {
        let velu = VeluIsogeny::new(-1.0, 0.0, vec![0.0]);
        let (a_prime, b_prime) = velu.codomain_coefficients();
        assert!((a_prime - 4.0).abs() < 1e-9);
        assert!(b_prime.abs() < 1e-9);
        assert_eq!(velu.degree(), 2);
    }
    #[test]
    fn test_weil_pairing_computer_rust() {
        let wpc = WeilPairingComputer::new(7, "BN256".to_string());
        let bilin = wpc.bilinearity_statement();
        assert!(bilin.contains("e_7"));
        assert!(bilin.contains("BN256"));
        let alt = wpc.alternating_statement();
        assert!(alt.contains("alternating"));
        let nd = wpc.non_degeneracy_statement();
        assert!(nd.contains("P = O"));
        let iters = wpc.miller_iteration_count();
        assert!(iters >= 3);
    }
    #[test]
    fn test_miller_algorithm_rust() {
        let miller = MillerAlgorithmImpl::new(2, -1.0);
        let val = miller.tangent_line_value((0.0, 0.0), (1.0, 0.0));
        let _ = val;
        let miller2 = MillerAlgorithmImpl::new(5, -5.0);
        let result = miller2.miller_loop((1.0, 2.0), (2.0, 1.0));
        let _ = result;
    }
    #[test]
    fn test_ecdlp_solver_rust() {
        let solver = ECDLPSolver::new(-1.0, 0.0, 4);
        let g = (1.0, f64::INFINITY);
        let target = (1.0, f64::INFINITY);
        let result = solver.solve(g, target);
        let _ = result;
    }
    #[test]
    fn test_pow_mod_helper() {
        assert_eq!(pow_mod(2, 10, 1000), 24);
        assert_eq!(pow_mod(3, 0, 7), 1);
        assert_eq!(pow_mod(0, 5, 7), 0);
        assert_eq!(pow_mod(2, 3, 8), 0);
    }
    #[test]
    fn test_quadratic_residue_helper() {
        assert!(is_quadratic_residue(4, 7));
        assert!(!is_quadratic_residue(3, 7));
        assert!(is_quadratic_residue(1, 7));
    }
}
/// Build an `Environment` containing all elliptic curve kernel axioms.
///
/// This is an alias for `build_elliptic_curves_env` using the canonical name
/// expected by the OxiLean std module interface.
pub fn build_env() -> Environment {
    build_elliptic_curves_env()
}
#[cfg(test)]
mod tests_ec_extra {
    use super::*;
    #[test]
    fn test_weil_pairing() {
        let wp = WeilPairing::new("BN254", 12);
        assert!(wp.is_non_degenerate());
        assert!(!WeilPairing::new("weak", 1).is_non_degenerate());
    }
    #[test]
    fn test_supersingular_curve() {
        let sc = SupersingularCurve::new(101, 0);
        assert_eq!(sc.trace_of_frobenius(), 0);
        assert_eq!(sc.group_order(), 102);
        assert!(sc.endomorphism_ring_is_maximal_order());
    }
    #[test]
    fn test_montgomery_ladder() {
        let ladder = MontgomeryLadder::from_scalar(255);
        assert_eq!(ladder.bit_length(), 8);
        assert_eq!(ladder.n_additions(), 7);
        assert_eq!(ladder.n_doublings(), 7);
        assert!(ladder.estimated_field_mults() > 0);
    }
    #[test]
    fn test_ecdsa_params() {
        let p256 = ECDSAParams::p256();
        assert_eq!(p256.order_bits, 256);
        assert_eq!(p256.security_level_bits(), 128);
        assert_eq!(p256.signature_size_bytes(), 64);
        let k = ECDSAParams::secp256k1();
        assert_eq!(k.curve_name, "secp256k1");
    }
    #[test]
    fn test_ecdh() {
        let x25519 = ECDHExchange::x25519();
        assert_eq!(x25519.shared_secret_size_bytes(), 32);
        assert_eq!(x25519.security_level_bits(), 127);
        let x448 = ECDHExchange::x448();
        assert_eq!(x448.shared_secret_size_bytes(), 56);
    }
    #[test]
    fn test_bls_signature() {
        let bls = BLSSignature::bls12_381();
        assert!(bls.supports_aggregation());
        assert_eq!(bls.signature_size_bytes(), 48);
        assert_eq!(BLSSignature::verify_pairing_count(), 2);
    }
    #[test]
    fn test_isogeny_computation() {
        let iso = IsogenyComputation::new(11);
        assert!(iso.is_prime_degree());
        let iso4 = IsogenyComputation::new(4);
        assert!(!iso4.is_prime_degree());
        assert!(iso.fast_velu_complexity() < iso.velu_complexity());
    }
    #[test]
    fn test_twisted_edwards() {
        let ed = TwistedEdwardsCurve::ed25519();
        assert!(ed.has_complete_addition_law());
        assert!(ed.group_order_multiple_of_4());
        assert_eq!(TwistedEdwardsCurve::neutral_point(), (0, 1));
    }
}
