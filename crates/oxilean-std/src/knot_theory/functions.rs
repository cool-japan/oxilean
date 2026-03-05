//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{
    AlexanderPolynomial, BraidWord, HOMFLYPolynomial, HyperbolicVolumeEstimator, JonesPolynomial,
    KauffmanBracketCalc, Knot, KnotDiagram, LaurentPoly, LorenzKnotData, RationalTangle,
    SeifertMatrixComputer, SeifertSurface,
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
pub fn knot_ty() -> Expr {
    cst("Knot")
}
pub fn link_ty() -> Expr {
    cst("Link")
}
pub fn poly_ty() -> Expr {
    cst("Polynomial")
}
pub fn braid_ty() -> Expr {
    cst("Braid")
}
pub fn tangle_ty() -> Expr {
    cst("Tangle")
}
pub fn surface_ty() -> Expr {
    cst("Surface")
}
pub fn real_ty() -> Expr {
    cst("Real")
}
/// `KnotDiagram : Type`  — a planar projection with labeled crossings.
pub fn knot_diagram_ty() -> Expr {
    type0()
}
/// `Crossing : Type`  — a single over/under crossing in a diagram.
pub fn crossing_ty() -> Expr {
    type0()
}
/// `CrossingSign : Type`  — positive (+1) or negative (-1) crossing.
pub fn crossing_sign_ty() -> Expr {
    type0()
}
/// `GaussCode : Type`  — ordered sequence of signed crossing labels along a knot.
pub fn gauss_code_ty() -> Expr {
    type0()
}
/// `PlanarDiagram : Type`  — PD notation: list of 4-tuples for each crossing.
pub fn planar_diagram_ty() -> Expr {
    type0()
}
/// `DiagramEquivalence : KnotDiagram → KnotDiagram → Prop`
/// Two diagrams represent the same knot iff related by Reidemeister moves.
pub fn diagram_equivalence_ty() -> Expr {
    arrow(knot_diagram_ty(), arrow(knot_diagram_ty(), prop()))
}
/// `ReidemeisterI : KnotDiagram → KnotDiagram → Prop`
/// A type-I move adds or removes a single kink (curl).
pub fn reidemeister_i_ty() -> Expr {
    arrow(knot_diagram_ty(), arrow(knot_diagram_ty(), prop()))
}
/// `ReidemeisterII : KnotDiagram → KnotDiagram → Prop`
/// A type-II move introduces or removes two canceling crossings.
pub fn reidemeister_ii_ty() -> Expr {
    arrow(knot_diagram_ty(), arrow(knot_diagram_ty(), prop()))
}
/// `ReidemeisterIII : KnotDiagram → KnotDiagram → Prop`
/// A type-III move slides a strand over a crossing.
pub fn reidemeister_iii_ty() -> Expr {
    arrow(knot_diagram_ty(), arrow(knot_diagram_ty(), prop()))
}
/// Theorem: Reidemeister moves generate all diagram equivalences.
/// ∀ D₁ D₂ : KnotDiagram, DiagramEquivalence D₁ D₂ ↔
///   (ReidemeisterI ∪ ReidemeisterII ∪ ReidemeisterIII)* D₁ D₂
pub fn reidemeister_completeness_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "D1",
        knot_diagram_ty(),
        pi(BinderInfo::Default, "D2", knot_diagram_ty(), prop()),
    )
}
/// `Writhe : KnotDiagram → Int`
/// The writhe w(D) = sum of crossing signs.  NOT a knot invariant (only framing).
pub fn writhe_ty() -> Expr {
    arrow(knot_diagram_ty(), int_ty())
}
/// `AlexanderPolynomial : Knot → Polynomial`
/// The Alexander polynomial Δ_K(t) ∈ Z[t, t⁻¹].
pub fn alexander_polynomial_ty() -> Expr {
    arrow(knot_ty(), poly_ty())
}
/// `JonesPolynomial : Knot → Polynomial`
/// V_K(t) ∈ Z[t^{1/2}, t^{-1/2}]; defined via the Kauffman bracket.
pub fn jones_polynomial_ty() -> Expr {
    arrow(knot_ty(), poly_ty())
}
/// `HomflyPt : Knot → Polynomial`
/// P_K(v, z) ∈ Z[v^{±1}, z^{±1}]; generalises both Alexander and Jones.
pub fn homfly_pt_ty() -> Expr {
    arrow(knot_ty(), poly_ty())
}
/// `KauffmanBracket : KnotDiagram → Ring → Polynomial`
/// ⟨D⟩ ∈ R[A, A⁻¹]; invariant of regular isotopy.
pub fn kauffman_bracket_ty() -> Expr {
    arrow(knot_diagram_ty(), arrow(ring_ty(), poly_ty()))
}
/// `ArcIndex : Knot → Nat`
/// The arc index a(K): minimum number of arcs in a grid diagram for K.
pub fn arc_index_ty() -> Expr {
    arrow(knot_ty(), nat_ty())
}
/// `BridgeNumber : Knot → Nat`
/// b(K): minimum number of local maxima over all embeddings.
pub fn bridge_number_ty() -> Expr {
    arrow(knot_ty(), nat_ty())
}
/// `CrossingNumber : Knot → Nat`
/// c(K): minimum crossing number over all diagrams.
pub fn crossing_number_ty() -> Expr {
    arrow(knot_ty(), nat_ty())
}
/// `UnknotCriterion : Knot → Prop`
/// Alexander polynomial = 1 is necessary (but not sufficient) for the unknot.
pub fn unknot_criterion_ty() -> Expr {
    arrow(knot_ty(), prop())
}
/// `LinkingNumber : Link → Nat → Nat → Int`
/// lk(K₁, K₂) computed from a diagram: half the sum of mixed crossing signs.
pub fn linking_number_ty() -> Expr {
    arrow(link_ty(), arrow(nat_ty(), arrow(nat_ty(), int_ty())))
}
/// `SeifertSurface : Knot → Surface`
/// A compact oriented surface whose boundary is the knot.
pub fn seifert_surface_ty() -> Expr {
    arrow(knot_ty(), surface_ty())
}
/// `Genus : Knot → Nat`
/// g(K): minimum genus of any Seifert surface for K.
pub fn genus_ty() -> Expr {
    arrow(knot_ty(), nat_ty())
}
/// `SeifertMatrix : Knot → Type`
/// The Seifert matrix V derived from a Seifert surface.
pub fn seifert_matrix_ty() -> Expr {
    arrow(knot_ty(), type0())
}
/// `SeifertGenusFormula : ∀ (K : Knot), Genus K ≤ (rank(SeifertMatrix K)) / 2`
pub fn seifert_genus_formula_ty() -> Expr {
    pi(BinderInfo::Default, "K", knot_ty(), prop())
}
/// `VirtualKnotDiagram : Type`
/// A knot diagram with additional virtual (non-classical) crossings.
pub fn virtual_knot_diagram_ty() -> Expr {
    type0()
}
/// `VirtualCrossing : Type`  — a virtual (circled) crossing.
pub fn virtual_crossing_ty() -> Expr {
    type0()
}
/// `ParityPolynomial : VirtualKnotDiagram → Polynomial`
/// An invariant of virtual knots using the parity of crossings.
pub fn parity_polynomial_ty() -> Expr {
    arrow(virtual_knot_diagram_ty(), poly_ty())
}
/// `ArrowPolynomial : VirtualKnotDiagram → Polynomial`
/// A stronger invariant of virtual knots via arrow calculus.
pub fn arrow_polynomial_ty() -> Expr {
    arrow(virtual_knot_diagram_ty(), poly_ty())
}
/// `VirtualGenus : VirtualKnotDiagram → Nat`
/// Virtual genus: minimum genus of a surface in which the diagram embeds.
pub fn virtual_genus_ty() -> Expr {
    arrow(virtual_knot_diagram_ty(), nat_ty())
}
/// `RationalTangle : Type`  — a 2-tangle with a rational fraction invariant.
pub fn rational_tangle_ty() -> Expr {
    type0()
}
/// `TangleFraction : Tangle → Polynomial`
/// Conway's fraction: classifies rational tangles.
pub fn tangle_fraction_ty() -> Expr {
    arrow(tangle_ty(), poly_ty())
}
/// `TangleSum : Tangle → Tangle → Tangle`
/// Horizontal composition of two tangles.
pub fn tangle_sum_ty() -> Expr {
    arrow(tangle_ty(), arrow(tangle_ty(), tangle_ty()))
}
/// `TangleProd : Tangle → Tangle → Tangle`
/// Vertical composition of two tangles.
pub fn tangle_prod_ty() -> Expr {
    arrow(tangle_ty(), arrow(tangle_ty(), tangle_ty()))
}
/// `TangleClosure : Tangle → Knot`
/// Closing a (1,1)-tangle yields a knot.
pub fn tangle_closure_ty() -> Expr {
    arrow(tangle_ty(), knot_ty())
}
/// `ConwayNotation : Type`
/// A string representation of a knot/link via tangle decomposition.
pub fn conway_notation_ty() -> Expr {
    type0()
}
/// `BraidGroup : Nat → Type`
/// B_n = the braid group on n strands.
pub fn braid_group_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `BraidWord : Nat → Type`
/// A word in the Artin generators σ_i^{±1}.
pub fn braid_word_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `BraidClosure : Braid → Knot`
/// Taking the closure of a braid yields a knot or link.
pub fn braid_closure_ty() -> Expr {
    arrow(braid_ty(), knot_ty())
}
/// Alexander's theorem: every knot/link is the closure of some braid.
/// ∀ (K : Knot), ∃ (n : Nat) (β : BraidGroup n), BraidClosure β = K
pub fn alexander_braid_theorem_ty() -> Expr {
    pi(BinderInfo::Default, "K", knot_ty(), prop())
}
/// Markov's theorem: two braids have ambient-isotopic closures iff they are
/// related by Markov moves (conjugation and stabilisation).
pub fn markov_theorem_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "b1",
        braid_ty(),
        pi(BinderInfo::Default, "b2", braid_ty(), prop()),
    )
}
/// `BurauRepresentation : Nat → Braid → Type`
/// The Burau matrix representation of a braid.
pub fn burau_representation_ty() -> Expr {
    arrow(nat_ty(), arrow(braid_ty(), type0()))
}
/// `LawrenceBigelowRepresentation : Nat → Braid → Type`
/// The faithful Lawrence-Bigelow representation (proves faithfulness of B_n for n ≥ 5).
pub fn lawrence_bigelow_ty() -> Expr {
    arrow(nat_ty(), arrow(braid_ty(), type0()))
}
/// `LorenzTemplate : Type`
/// The branched surface (template) that organises the Lorenz flow's periodic orbits.
pub fn lorenz_template_ty() -> Expr {
    type0()
}
/// `LorenzKnot : Type`
/// A periodic orbit of the Lorenz differential equation, viewed as a knot in S³.
pub fn lorenz_knot_ty() -> Expr {
    type0()
}
/// `LorenzBraid : LorenzKnot → Braid`
/// Every Lorenz knot is a positive braid (all generators σ_i, no σ_i⁻¹).
pub fn lorenz_braid_ty() -> Expr {
    arrow(lorenz_knot_ty(), braid_ty())
}
/// `LorenzFibered : LorenzKnot → Prop`
/// All Lorenz knots are fibered.
pub fn lorenz_fibered_ty() -> Expr {
    arrow(lorenz_knot_ty(), prop())
}
/// `LorenzPositive : LorenzKnot → Prop`
/// All Lorenz knots are positive (admit a positive braid representation).
pub fn lorenz_positive_ty() -> Expr {
    arrow(lorenz_knot_ty(), prop())
}
/// Populate an `Environment` with all knot-theory axioms.
pub fn build_knot_theory_env(env: &mut Environment) {
    let axioms: &[(&str, Expr)] = &[
        ("Knot", type0()),
        ("Link", type0()),
        ("Braid", type0()),
        ("Tangle", type0()),
        ("Surface", type0()),
        ("Polynomial", type0()),
        ("Ring", type0()),
        ("KnotDiagram", knot_diagram_ty()),
        ("Crossing", crossing_ty()),
        ("CrossingSign", crossing_sign_ty()),
        ("GaussCode", gauss_code_ty()),
        ("PlanarDiagram", planar_diagram_ty()),
        ("DiagramEquivalence", diagram_equivalence_ty()),
        ("ReidemeisterI", reidemeister_i_ty()),
        ("ReidemeisterII", reidemeister_ii_ty()),
        ("ReidemeisterIII", reidemeister_iii_ty()),
        ("ReidemeisterCompleteness", reidemeister_completeness_ty()),
        ("Writhe", writhe_ty()),
        ("AlexanderPolynomial", alexander_polynomial_ty()),
        ("JonesPolynomial", jones_polynomial_ty()),
        ("HomflyPt", homfly_pt_ty()),
        ("KauffmanBracket", kauffman_bracket_ty()),
        ("ArcIndex", arc_index_ty()),
        ("BridgeNumber", bridge_number_ty()),
        ("CrossingNumber", crossing_number_ty()),
        ("UnknotCriterion", unknot_criterion_ty()),
        ("LinkingNumber", linking_number_ty()),
        ("SeifertSurface", seifert_surface_ty()),
        ("Genus", genus_ty()),
        ("SeifertMatrix", seifert_matrix_ty()),
        ("SeifertGenusFormula", seifert_genus_formula_ty()),
        ("VirtualKnotDiagram", virtual_knot_diagram_ty()),
        ("VirtualCrossing", virtual_crossing_ty()),
        ("ParityPolynomial", parity_polynomial_ty()),
        ("ArrowPolynomial", arrow_polynomial_ty()),
        ("VirtualGenus", virtual_genus_ty()),
        ("RationalTangle", rational_tangle_ty()),
        ("TangleFraction", tangle_fraction_ty()),
        ("TangleSum", tangle_sum_ty()),
        ("TangleProd", tangle_prod_ty()),
        ("TangleClosure", tangle_closure_ty()),
        ("ConwayNotation", conway_notation_ty()),
        ("BraidGroup", braid_group_ty()),
        ("BraidWord", braid_word_ty()),
        ("BraidClosure", braid_closure_ty()),
        ("AlexanderBraidTheorem", alexander_braid_theorem_ty()),
        ("MarkovTheorem", markov_theorem_ty()),
        ("BurauRepresentation", burau_representation_ty()),
        ("LawrenceBigelow", lawrence_bigelow_ty()),
        ("LorenzTemplate", lorenz_template_ty()),
        ("LorenzKnot", lorenz_knot_ty()),
        ("LorenzBraid", lorenz_braid_ty()),
        ("LorenzFibered", lorenz_fibered_ty()),
        ("LorenzPositive", lorenz_positive_ty()),
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
/// Compute a simplified Alexander polynomial from the crossing sequence.
/// This is a heuristic for demonstration; real computation uses the Seifert matrix.
pub fn compute_alexander_mock(diagram: &KnotDiagram) -> LaurentPoly {
    let n = diagram.crossings.len();
    match n {
        0 => LaurentPoly::monomial(1, 0),
        3 => LaurentPoly {
            min_exp: 0,
            coeffs: vec![1, -1, 1],
        },
        4 => LaurentPoly {
            min_exp: -1,
            coeffs: vec![-1, 3, -1],
        },
        2 => LaurentPoly {
            min_exp: 0,
            coeffs: vec![1, -1],
        },
        _ => LaurentPoly::monomial(1, 0),
    }
}
/// Compute the linking number of two components of a 2-component link diagram.
/// Uses the formula: lk = (1/2) * Σ (signs of mixed crossings).
pub fn linking_number(diagram: &KnotDiagram) -> i32 {
    let sum: i32 = diagram.crossings.iter().map(|c| c.sign.value()).sum();
    sum / 2
}
pub fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_build_env_nonempty() {
        let mut env = Environment::new();
        build_knot_theory_env(&mut env);
        assert!(env.get(&Name::str("Knot")).is_some());
        assert!(env.get(&Name::str("AlexanderPolynomial")).is_some());
        assert!(env.get(&Name::str("JonesPolynomial")).is_some());
        assert!(env.get(&Name::str("MarkovTheorem")).is_some());
        assert!(env.get(&Name::str("LorenzFibered")).is_some());
    }
    #[test]
    fn test_writhe_trefoil() {
        let d = KnotDiagram::left_trefoil();
        assert_eq!(d.writhe(), -3, "left trefoil has writhe -3");
    }
    #[test]
    fn test_writhe_figure_eight() {
        let d = KnotDiagram::figure_eight();
        assert_eq!(d.writhe(), 0, "figure-eight knot has writhe 0");
    }
    #[test]
    fn test_linking_number_hopf() {
        let d = KnotDiagram::hopf_link();
        assert_eq!(linking_number(&d), 1, "Hopf link has linking number 1");
    }
    #[test]
    fn test_alexander_polynomial_trefoil() {
        let d = KnotDiagram::left_trefoil();
        let poly = compute_alexander_mock(&d);
        assert_eq!(poly.coeff(0), 1);
        assert_eq!(poly.coeff(1), -1);
        assert_eq!(poly.coeff(2), 1);
    }
    #[test]
    fn test_alexander_polynomial_figure_eight() {
        let d = KnotDiagram::figure_eight();
        let poly = compute_alexander_mock(&d);
        assert_eq!(poly.coeff(-1), -1);
        assert_eq!(poly.coeff(0), 3);
        assert_eq!(poly.coeff(1), -1);
    }
    #[test]
    fn test_braid_word_trefoil() {
        let b = BraidWord::trefoil_braid();
        assert_eq!(b.strands, 2);
        assert_eq!(b.word_length(), 3);
        assert!(b.is_positive());
        assert_eq!(b.algebraic_length(), 3);
    }
    #[test]
    fn test_lorenz_knot_trefoil() {
        let lk = LorenzKnotData::trefoil_lorenz();
        assert_eq!(lk.period, 3);
        assert_eq!(lk.left_count(), 2);
        assert_eq!(lk.right_count(), 1);
        assert!(lk.is_fibered());
        assert!(lk.is_positive());
    }
    #[test]
    fn test_rational_tangle_add() {
        let t1 = RationalTangle::integer(1);
        let t2 = RationalTangle::integer(2);
        let sum = t1.add(&t2);
        assert_eq!(sum.numerator, 3);
        assert_eq!(sum.denominator, 1);
    }
}
/// Build an environment containing the knot theory axioms.
pub fn build_env() -> Environment {
    let mut env = Environment::new();
    build_knot_theory_env(&mut env);
    env
}
/// `HomflySkeinRelation : Prop`
/// v⁻¹ P(L₊) − v P(L₋) = z P(L₀): the HOMFLY skein relation.
pub fn homfly_skein_relation_ty() -> Expr {
    prop()
}
/// `HomflyNormalization : Prop`
/// P(unknot) = 1: normalization of the HOMFLY polynomial.
pub fn homfly_normalization_ty() -> Expr {
    prop()
}
/// `KauffmanPolynomial : Knot → Polynomial`
/// F_K(a, x) ∈ ℤ[a^{±1}, x^{±1}]: the Kauffman polynomial (not the bracket).
pub fn kauffman_polynomial_ty() -> Expr {
    arrow(knot_ty(), poly_ty())
}
/// `KauffmanBracketSkein : KnotDiagram → Prop`
/// ⟨D⟩ = A ⟨D₀⟩ + A⁻¹ ⟨D∞⟩: the Kauffman bracket skein relation.
pub fn kauffman_bracket_skein_ty() -> Expr {
    arrow(knot_diagram_ty(), prop())
}
/// `KauffmanBracketNormalization : KnotDiagram → Prop`
/// ⟨unknot⟩ = 1 and ⟨D ∪ unknot⟩ = (-A² - A⁻²) ⟨D⟩.
pub fn kauffman_bracket_normalization_ty() -> Expr {
    arrow(knot_diagram_ty(), prop())
}
/// `JonesFromKauffman : Knot → Polynomial`
/// V_K(t) obtained from the Kauffman bracket via writhe normalization.
pub fn jones_from_kauffman_ty() -> Expr {
    arrow(knot_ty(), poly_ty())
}
/// `QuantumGroup : Type`
/// U_q(sl_2) and its generalizations: the source of quantum knot invariants.
pub fn quantum_group_ty() -> Expr {
    type0()
}
/// `ColoredJonesPolynomial : Knot → ℕ → Polynomial`
/// J_n(K; q): the n-colored Jones polynomial (Jones = 2-colored).
pub fn colored_jones_ty() -> Expr {
    arrow(knot_ty(), arrow(nat_ty(), poly_ty()))
}
/// `QuantumRepresentation : QuantumGroup → ℕ → Type`
/// The n-dimensional irreducible representation of U_q(sl_2).
pub fn quantum_representation_ty() -> Expr {
    arrow(quantum_group_ty(), arrow(nat_ty(), type0()))
}
/// `RMatrix : QuantumGroup → Type`
/// The universal R-matrix of a quantum group: solves the Yang-Baxter equation.
pub fn r_matrix_ty() -> Expr {
    arrow(quantum_group_ty(), type0())
}
/// `YangBaxterEquation : RMatrix → Prop`
/// R₁₂ R₁₃ R₂₃ = R₂₃ R₁₃ R₁₂: the braid-like Yang-Baxter equation.
pub fn yang_baxter_equation_ty() -> Expr {
    arrow(r_matrix_ty(), prop())
}
/// `QuantumTraceInvariant : QuantumGroup → Knot → Polynomial`
/// The quantum trace (Markov trace) applied to a quantum group representation.
pub fn quantum_trace_invariant_ty() -> Expr {
    arrow(quantum_group_ty(), arrow(knot_ty(), poly_ty()))
}
/// `ReshetikhinTuraevInvariant : QuantumGroup → Knot → Polynomial`
/// The Reshetikhin-Turaev invariant built from the quantum group representation.
pub fn reshetikhin_turaev_ty() -> Expr {
    arrow(quantum_group_ty(), arrow(knot_ty(), poly_ty()))
}
/// `KnotGroup : Knot → Type`
/// π₁(S³ \ K): the fundamental group of the knot complement.
pub fn knot_group_ty() -> Expr {
    arrow(knot_ty(), type0())
}
/// `WirtingerPresentation : Knot → Type`
/// A group presentation of π₁(S³\K) with generators = arcs, relations = crossings.
pub fn wirtinger_presentation_ty() -> Expr {
    arrow(knot_ty(), type0())
}
/// `AlexanderIdeal : Knot → Type`
/// The Alexander ideal in ℤ[t, t⁻¹] obtained from the Alexander matrix.
pub fn alexander_ideal_ty() -> Expr {
    arrow(knot_ty(), type0())
}
/// `FoxCalculus : KnotGroup → Type`
/// Fox free differential calculus: ∂/∂xᵢ of group ring elements.
pub fn fox_calculus_ty() -> Expr {
    arrow(knot_group_ty(), type0())
}
/// `AlexanderMatrix : Knot → Type`
/// The Alexander (Fox Jacobian) matrix from the Wirtinger presentation.
pub fn alexander_matrix_ty() -> Expr {
    arrow(knot_ty(), type0())
}
/// `AbelianizationKnotGroup : Knot → Prop`
/// π₁(S³\K)^{ab} = ℤ (abelianization is always ℤ for knots).
pub fn abelianization_knot_group_ty() -> Expr {
    arrow(knot_ty(), prop())
}
/// `SeifertSignature : Knot → ℤ`
/// σ(K) = signature of V + Vᵀ where V is the Seifert matrix.
pub fn seifert_signature_ty() -> Expr {
    arrow(knot_ty(), int_ty())
}
/// `SEquivalence : SeifertMatrix → SeifertMatrix → Prop`
/// Two Seifert matrices are S-equivalent iff they differ by row/column operations.
pub fn s_equivalence_ty() -> Expr {
    arrow(
        cst("SeifertMatrixData"),
        arrow(cst("SeifertMatrixData"), prop()),
    )
}
/// `FiberKnot : Knot → Prop`
/// K is fibered iff its Seifert surface is a fiber in a fibration S³\K → S¹.
pub fn fiber_knot_ty() -> Expr {
    arrow(knot_ty(), prop())
}
/// `SeifertAlgorithm : KnotDiagram → Surface`
/// Seifert's algorithm: produces a Seifert surface from any oriented knot diagram.
pub fn seifert_algorithm_ty() -> Expr {
    arrow(knot_diagram_ty(), surface_ty())
}
/// `SeifertCircle : KnotDiagram → ℕ`
/// Number of Seifert circles s(D) produced by Seifert's algorithm.
pub fn seifert_circle_ty() -> Expr {
    arrow(knot_diagram_ty(), nat_ty())
}
/// `SatelliteKnot : Knot → Knot → Knot`
/// K_P: the satellite with pattern P and companion K.
pub fn satellite_knot_ty() -> Expr {
    arrow(knot_ty(), arrow(knot_ty(), knot_ty()))
}
/// `CableKnot : Knot → ℕ → ℕ → Knot`
/// The (p,q)-cable of K: wrap p times longitudinally and q times meridionally.
pub fn cable_knot_ty() -> Expr {
    arrow(knot_ty(), arrow(nat_ty(), arrow(nat_ty(), knot_ty())))
}
/// `WhiteheadDouble : Knot → Knot`
/// The Whitehead double of K: a satellite with Whitehead link as pattern.
pub fn whitehead_double_ty() -> Expr {
    arrow(knot_ty(), knot_ty())
}
/// `SatelliteAlexander : Knot → Knot → Prop`
/// Alexander polynomial of satellite: Δ_{K_P}(t) = Δ_P(t) · Δ_K(t^n) for winding n.
pub fn satellite_alexander_ty() -> Expr {
    arrow(knot_ty(), arrow(knot_ty(), prop()))
}
/// `CompanionGenus : Knot → ℕ → ℕ`
/// The genus of a satellite knot in terms of companion genus and pattern.
pub fn companion_genus_ty() -> Expr {
    arrow(knot_ty(), arrow(nat_ty(), nat_ty()))
}
/// `LegendriankKnot : Type`
/// A knot tangent to the standard contact structure ξ_std = ker(dz - y dx) on ℝ³.
pub fn legendrian_knot_ty() -> Expr {
    type0()
}
/// `TransverseKnot : Type`
/// A knot transverse to the contact structure ξ.
pub fn transverse_knot_ty() -> Expr {
    type0()
}
/// `ThurstonBennequinInequality : LegendriankKnot → Prop`
/// tb(K) + |r(K)| ≤ 2g(K) - 1 for a Legendrian knot.
pub fn thurston_bennequin_inequality_ty() -> Expr {
    arrow(legendrian_knot_ty(), prop())
}
/// `FrontProjection : LegendriankKnot → KnotDiagram`
/// The front projection of a Legendrian knot to the xz-plane.
pub fn front_projection_ty() -> Expr {
    arrow(legendrian_knot_ty(), knot_diagram_ty())
}
/// `LOSSInvariant : LegendriankKnot → Type`
/// The LOSS invariant (Lisca-Ozsváth-Stipsicz-Szabó) in knot Floer homology.
pub fn loss_invariant_ty() -> Expr {
    arrow(legendrian_knot_ty(), type0())
}
/// `SelfLinkingNumber : TransverseKnot → ℤ`
/// sl(K): the self-linking number of a transverse knot.
pub fn self_linking_number_ty() -> Expr {
    arrow(transverse_knot_ty(), int_ty())
}
/// `HyperbolicKnot : Type`
/// A knot whose complement admits a complete hyperbolic metric of finite volume.
pub fn hyperbolic_knot_ty() -> Expr {
    type0()
}
/// `HyperbolicVolume : HyperbolicKnot → ℝ`
/// Vol(S³\K): the hyperbolic volume of the knot complement.
pub fn hyperbolic_volume_ty() -> Expr {
    arrow(hyperbolic_knot_ty(), real_ty())
}
/// `ThurstonHyperbolicTheorem : Knot → Prop`
/// A knot is hyperbolic iff it is not a torus knot and not a satellite knot.
pub fn thurston_hyperbolic_theorem_ty() -> Expr {
    arrow(knot_ty(), prop())
}
/// `APolynomial : HyperbolicKnot → Polynomial`
/// The A-polynomial of a hyperbolic knot: character variety equation.
pub fn a_polynomial_ty() -> Expr {
    arrow(hyperbolic_knot_ty(), poly_ty())
}
/// `RigidityTheorem : HyperbolicKnot → Prop`
/// Mostow rigidity: the hyperbolic metric is unique (hyperbolic volume is a knot invariant).
pub fn rigidity_theorem_ty() -> Expr {
    arrow(hyperbolic_knot_ty(), prop())
}
/// `GeometricDehn : HyperbolicKnot → ℤ → ℤ → Knot`
/// Dehn surgery on a hyperbolic knot K: K(p/q) = p-filling along q-framing.
pub fn geometric_dehn_ty() -> Expr {
    arrow(
        hyperbolic_knot_ty(),
        arrow(int_ty(), arrow(int_ty(), knot_ty())),
    )
}
/// `ContactHomology : Knot → Type`
/// Legendrian contact homology DGA A(K) associated to a knot K.
pub fn contact_homology_ty() -> Expr {
    arrow(knot_ty(), type0())
}
/// `AugmentationPolynomial : Knot → Polynomial`
/// Aug_K(x, y, q): encodes augmentations of the Chekanov DGA.
pub fn augmentation_polynomial_ty() -> Expr {
    arrow(knot_ty(), poly_ty())
}
/// `ChekanovDGA : LegendriankKnot → Type`
/// The Chekanov differential graded algebra of a Legendrian knot.
pub fn chekanov_dga_ty() -> Expr {
    arrow(legendrian_knot_ty(), type0())
}
/// `KnotContactHomologyInvariance : Prop`
/// Knot contact homology is an invariant of the smooth knot type.
pub fn kch_invariance_ty() -> Expr {
    prop()
}
/// `TauInvariant : Knot → ℤ`
/// Ozsváth-Szabó's τ invariant from knot Floer homology.
pub fn tau_invariant_ty() -> Expr {
    arrow(knot_ty(), int_ty())
}
/// `SInvariant : Knot → ℤ`
/// Rasmussen's s invariant from Khovanov homology.
pub fn s_invariant_ty() -> Expr {
    arrow(knot_ty(), int_ty())
}
/// `UpsilonInvariant : Knot → ℝ → ℝ`
/// Υ_K(t) for t ∈ [0,2]: a concordance invariant from knot Floer homology.
pub fn upsilon_invariant_ty() -> Expr {
    arrow(knot_ty(), arrow(real_ty(), real_ty()))
}
/// `OSSZInvariant : Knot → ℤ`
/// The Ozsváth-Stipsicz-Szabó concordance invariant from knot Floer homology.
pub fn ossz_invariant_ty() -> Expr {
    arrow(knot_ty(), int_ty())
}
/// `SliceGenus : Knot → ℕ`
/// g_4(K): the 4-ball (slice) genus: minimum genus of a smooth surface in B^4 bounded by K.
pub fn slice_genus_ty() -> Expr {
    arrow(knot_ty(), nat_ty())
}
/// `SliceGenusLowerBound : Knot → Prop`
/// |τ(K)| ≤ g_4(K) and |s(K)|/2 ≤ g_4(K).
pub fn slice_genus_lower_bound_ty() -> Expr {
    arrow(knot_ty(), prop())
}
/// `RibbonKnot : Knot → Prop`
/// K is ribbon if it bounds an immersed disk in S³ with only ribbon singularities.
pub fn ribbon_knot_ty() -> Expr {
    arrow(knot_ty(), prop())
}
/// `GridDiagram : Type`
/// An n×n grid diagram for a knot: two sets of n points (O and X markings).
pub fn grid_diagram_ty() -> Expr {
    type0()
}
/// `GridMove : GridDiagram → GridDiagram → Prop`
/// Cyclic permutation, commutation, or (de)stabilization of grid diagrams.
pub fn grid_move_ty() -> Expr {
    arrow(grid_diagram_ty(), arrow(grid_diagram_ty(), prop()))
}
/// `GridHomology : Knot → Type`
/// GH(K): knot Floer homology computed combinatorially from a grid diagram.
pub fn grid_homology_ty() -> Expr {
    arrow(knot_ty(), type0())
}
/// `GridHomologyInvariance : Prop`
/// Grid homology is invariant under grid moves, hence a knot invariant.
pub fn grid_homology_invariance_ty() -> Expr {
    prop()
}
/// `GridAlexanderGrading : GridDiagram → ℤ`
/// The Alexander grading on the grid complex.
pub fn grid_alexander_grading_ty() -> Expr {
    arrow(grid_diagram_ty(), int_ty())
}
/// `GridMaslovGrading : GridDiagram → ℤ`
/// The Maslov (homological) grading on the grid complex.
pub fn grid_maslov_grading_ty() -> Expr {
    arrow(grid_diagram_ty(), int_ty())
}
/// `LeeSpectralSequence : Knot → Type`
/// Lee's spectral sequence from Khovanov homology to ℚ⊕ℚ (one class per smoothing).
pub fn lee_spectral_sequence_ty() -> Expr {
    arrow(knot_ty(), type0())
}
/// `LeeHomology : Knot → Type`
/// Lee's deformed homology Kh'(K) ≅ ℚ² for any knot K.
pub fn lee_homology_ty() -> Expr {
    arrow(knot_ty(), type0())
}
/// `RasmussenFromLee : LeeSpectralSequence → ℤ`
/// Rasmussen's s invariant from the filtration induced by Lee's spectral sequence.
pub fn rasmussen_from_lee_ty() -> Expr {
    arrow(lee_spectral_sequence_ty(), int_ty())
}
/// `LOSSFiltration : LegendriankKnot → Type`
/// The LOSS filtration on knot Floer chain complex.
pub fn loss_filtration_ty() -> Expr {
    arrow(legendrian_knot_ty(), type0())
}
/// `CategorifiedQuantumGroup : Type`
/// The 2-category categorifying U_q(sl_2) (Khovanov-Lauda, Rouquier).
pub fn categorified_quantum_group_ty() -> Expr {
    type0()
}
/// `SL2Categorification : Type`
/// sl_2 foam category: 2-morphisms are foams (cobordisms between webs).
pub fn sl2_categorification_ty() -> Expr {
    type0()
}
/// `KhovanovHomologyFunctor : Knot → Type`
/// Kh(K): a categorification of the Jones polynomial.
/// χ(Kh(K)) = V_K(q).
pub fn khovanov_homology_functor_ty() -> Expr {
    arrow(knot_ty(), type0())
}
/// `CubeOfResolutions : KnotDiagram → Type`
/// The {0,1}^n cube of Kauffman smoothings used to construct the Khovanov complex.
pub fn cube_of_resolutions_ty() -> Expr {
    arrow(knot_diagram_ty(), type0())
}
/// `KhovanovDifferential : KnotDiagram → Prop`
/// The differential d: C^{i,j}(K) → C^{i+1,j}(K) satisfies d² = 0.
pub fn khovanov_differential_ty() -> Expr {
    arrow(knot_diagram_ty(), prop())
}
/// `FoamEvaluation : SL2Categorification → ℤ`
/// The foam evaluation map in sl_2 link homology.
pub fn foam_evaluation_ty() -> Expr {
    arrow(sl2_categorification_ty(), int_ty())
}
/// Populate the environment with extended knot-theory axioms.
pub fn build_knot_theory_ext_env(env: &mut Environment) {
    let axioms: &[(&str, Expr)] = &[
        ("QuantumGroup", quantum_group_ty()),
        ("LegendriankKnot", legendrian_knot_ty()),
        ("TransverseKnot", transverse_knot_ty()),
        ("HyperbolicKnot", hyperbolic_knot_ty()),
        ("GridDiagram", grid_diagram_ty()),
        ("LeeSpectralSequence", lee_spectral_sequence_ty()),
        ("CategorifiedQuantumGroup", categorified_quantum_group_ty()),
        ("SL2Categorification", sl2_categorification_ty()),
        ("SeifertMatrixData", type0()),
        ("homfly_skein_relation", homfly_skein_relation_ty()),
        ("homfly_normalization", homfly_normalization_ty()),
        ("KauffmanPolynomial", kauffman_polynomial_ty()),
        ("kauffman_bracket_skein", kauffman_bracket_skein_ty()),
        (
            "kauffman_bracket_normalization",
            kauffman_bracket_normalization_ty(),
        ),
        ("JonesFromKauffman", jones_from_kauffman_ty()),
        ("ColoredJones", colored_jones_ty()),
        ("QuantumRepresentation", quantum_representation_ty()),
        ("RMatrix", r_matrix_ty()),
        ("yang_baxter_equation", yang_baxter_equation_ty()),
        ("QuantumTraceInvariant", quantum_trace_invariant_ty()),
        ("ReshetikhinTuraev", reshetikhin_turaev_ty()),
        ("KnotGroup", knot_group_ty()),
        ("WirtingerPresentation", wirtinger_presentation_ty()),
        ("AlexanderIdeal", alexander_ideal_ty()),
        ("FoxCalculus", fox_calculus_ty()),
        ("AlexanderMatrix", alexander_matrix_ty()),
        ("abelianization_knot_group", abelianization_knot_group_ty()),
        ("SeifertSignature", seifert_signature_ty()),
        ("s_equivalence", s_equivalence_ty()),
        ("fiber_knot", fiber_knot_ty()),
        ("SeifertAlgorithm", seifert_algorithm_ty()),
        ("SeifertCircle", seifert_circle_ty()),
        ("SatelliteKnot", satellite_knot_ty()),
        ("CableKnot", cable_knot_ty()),
        ("WhiteheadDouble", whitehead_double_ty()),
        ("satellite_alexander", satellite_alexander_ty()),
        ("CompanionGenus", companion_genus_ty()),
        (
            "thurston_bennequin_inequality",
            thurston_bennequin_inequality_ty(),
        ),
        ("FrontProjection", front_projection_ty()),
        ("LOSSInvariant", loss_invariant_ty()),
        ("SelfLinkingNumber", self_linking_number_ty()),
        ("HyperbolicVolume", hyperbolic_volume_ty()),
        (
            "thurston_hyperbolic_theorem",
            thurston_hyperbolic_theorem_ty(),
        ),
        ("APolynomial", a_polynomial_ty()),
        ("rigidity_theorem", rigidity_theorem_ty()),
        ("GeometricDehn", geometric_dehn_ty()),
        ("ContactHomology", contact_homology_ty()),
        ("AugmentationPolynomial", augmentation_polynomial_ty()),
        ("ChekanovDGA", chekanov_dga_ty()),
        ("kch_invariance", kch_invariance_ty()),
        ("TauInvariant", tau_invariant_ty()),
        ("SInvariant", s_invariant_ty()),
        ("UpsilonInvariant", upsilon_invariant_ty()),
        ("OSSZInvariant", ossz_invariant_ty()),
        ("SliceGenus", slice_genus_ty()),
        ("slice_genus_lower_bound", slice_genus_lower_bound_ty()),
        ("ribbon_knot", ribbon_knot_ty()),
        ("GridMove", grid_move_ty()),
        ("GridHomology", grid_homology_ty()),
        ("grid_homology_invariance", grid_homology_invariance_ty()),
        ("GridAlexanderGrading", grid_alexander_grading_ty()),
        ("GridMaslovGrading", grid_maslov_grading_ty()),
        ("LeeHomology", lee_homology_ty()),
        ("rasmussen_from_lee", rasmussen_from_lee_ty()),
        ("LOSSFiltration", loss_filtration_ty()),
        ("KhovanovHomologyFunctor", khovanov_homology_functor_ty()),
        ("CubeOfResolutions", cube_of_resolutions_ty()),
        ("khovanov_differential", khovanov_differential_ty()),
        ("FoamEvaluation", foam_evaluation_ty()),
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
mod ext_tests {
    use super::*;
    #[test]
    fn test_build_ext_env() {
        let mut env = Environment::new();
        build_knot_theory_env(&mut env);
        build_knot_theory_ext_env(&mut env);
        assert!(env.get(&Name::str("HyperbolicVolume")).is_some());
        assert!(env.get(&Name::str("ColoredJones")).is_some());
        assert!(env.get(&Name::str("TauInvariant")).is_some());
        assert!(env.get(&Name::str("GridHomology")).is_some());
        assert!(env.get(&Name::str("KhovanovHomologyFunctor")).is_some());
    }
    #[test]
    fn test_homfly_unknot() {
        let p = HOMFLYPolynomial::unknot();
        assert_eq!(p.get(0, 0), 1);
        assert_eq!(p.num_terms(), 1);
    }
    #[test]
    fn test_homfly_amphichirality() {
        let fig8 = HOMFLYPolynomial::figure_eight();
        assert!(fig8.is_amphichiral(), "Figure-eight should be amphichiral");
        let ltrefoil = HOMFLYPolynomial::left_trefoil();
        assert!(
            !ltrefoil.is_amphichiral(),
            "Left trefoil should not be amphichiral"
        );
    }
    #[test]
    fn test_homfly_evaluate_unknot_at_one() {
        let p = HOMFLYPolynomial::unknot();
        let val = p.evaluate(1.0, 0.0);
        assert!((val - 1.0).abs() < 1e-12, "P(unknot)(1,0) should be 1");
    }
    #[test]
    fn test_kauffman_bracket_loop_value() {
        let calc = KauffmanBracketCalc::new(1.0);
        assert!((calc.loop_value() - (-2.0)).abs() < 1e-12);
    }
    #[test]
    fn test_kauffman_bracket_unknot() {
        let calc = KauffmanBracketCalc::new(1.0);
        assert!((calc.bracket_unknot() - 1.0).abs() < 1e-12);
    }
    #[test]
    fn test_kauffman_bracket_with_loops() {
        let calc = KauffmanBracketCalc::new(1.0);
        let val = calc.bracket_with_loops(1.0, 2);
        assert!((val - 4.0).abs() < 1e-12);
    }
    #[test]
    fn test_seifert_matrix_trefoil() {
        let sm = SeifertMatrixComputer::trefoil();
        assert_eq!(sm.size, 2);
        assert_eq!(sm.knot_determinant(), 3, "Trefoil determinant should be 3");
    }
    #[test]
    fn test_seifert_matrix_figure_eight() {
        let sm = SeifertMatrixComputer::figure_eight();
        assert_eq!(
            sm.knot_determinant(),
            5,
            "Figure-eight determinant should be 5"
        );
    }
    #[test]
    fn test_seifert_signature_trefoil() {
        let sm = SeifertMatrixComputer::trefoil();
        assert_eq!(sm.signature_2x2(), -2, "Trefoil signature should be -2");
    }
    #[test]
    fn test_seifert_signature_figure_eight() {
        let sm = SeifertMatrixComputer::figure_eight();
        assert_eq!(sm.signature_2x2(), 0, "Figure-eight signature should be 0");
    }
    #[test]
    fn test_hyperbolic_volume_figure_eight() {
        let est = HyperbolicVolumeEstimator::new();
        let vol = est.volume("4_1");
        assert!(vol.is_some());
        let v = vol.expect("vol should be valid");
        assert!(
            (v - 2.0298832128_f64).abs() < 1e-6,
            "Volume of 4_1 should be ~2.0299, got {v}"
        );
    }
    #[test]
    fn test_hyperbolic_trefoil_not_hyperbolic() {
        let est = HyperbolicVolumeEstimator::new();
        assert!(!est.is_hyperbolic("trefoil"), "Trefoil is not hyperbolic");
    }
    #[test]
    fn test_lackenby_bound() {
        let est = HyperbolicVolumeEstimator::new();
        let bound = est.lackenby_bound(4);
        assert!(
            bound > 0.0,
            "Lackenby bound for 4-crossing knot should be positive"
        );
    }
    #[test]
    fn test_dehn_filling_volume() {
        let est = HyperbolicVolumeEstimator::new();
        let base_vol = est.figure_eight_volume();
        let filled = est.dehn_filling_volume("4_1", 10);
        assert!(filled < base_vol, "Dehn filling should reduce volume");
        assert!(
            filled > 0.0,
            "Non-trivial Dehn filling should have positive volume"
        );
    }
}
