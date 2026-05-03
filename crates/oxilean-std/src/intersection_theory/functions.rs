//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)
#![allow(clippy::items_after_test_module)]

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{
    BezoutBound, ChowRingElem, CycleClass, IntersectionMatrix, QuantumCohomologyP2, SchubertCalc,
    VectorBundleData,
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
pub fn real_ty() -> Expr {
    cst("Real")
}
pub fn bool_ty() -> Expr {
    cst("Bool")
}
pub fn list_ty(elem: Expr) -> Expr {
    app(cst("List"), elem)
}
/// `ChowGroup : Scheme → Nat → Type`
/// `ChowGroup X k` is the group of codimension-k cycles modulo rational equivalence.
pub fn chow_group_ty() -> Expr {
    arrow(cst("Scheme"), arrow(nat_ty(), type0()))
}
/// `ChowRing : Scheme → Type`
/// The graded ring ⊕_k A^k(X) with intersection product.
pub fn chow_ring_ty() -> Expr {
    arrow(cst("Scheme"), type0())
}
/// `AlgebraicCycle : Scheme → Nat → Type`
/// A formal integer-linear combination of subvarieties of codimension k.
pub fn algebraic_cycle_ty() -> Expr {
    arrow(cst("Scheme"), arrow(nat_ty(), type0()))
}
/// `RationalEquiv : AlgebraicCycle → AlgebraicCycle → Prop`
/// Two cycles are rationally equivalent if their difference is a boundary.
pub fn rational_equiv_ty() -> Expr {
    arrow(
        app2(cst("AlgebraicCycle"), cst("Scheme"), cst("k")),
        arrow(app2(cst("AlgebraicCycle"), cst("Scheme"), cst("k")), prop()),
    )
}
/// `IntersectionProduct : ChowRing X → ChowRing X → ChowRing X`
pub fn intersection_product_ty() -> Expr {
    arrow(cst("ChowRing"), arrow(cst("ChowRing"), cst("ChowRing")))
}
/// `PushforwardCycle : Morphism X Y → ChowGroup X k → ChowGroup Y k`
pub fn pushforward_cycle_ty() -> Expr {
    arrow(cst("Morphism"), arrow(cst("ChowGroup"), cst("ChowGroup")))
}
/// `PullbackCycle : Morphism X Y → ChowGroup Y k → ChowGroup X k`
/// (defined for flat morphisms and local complete intersection morphisms)
pub fn pullback_cycle_ty() -> Expr {
    arrow(cst("Morphism"), arrow(cst("ChowGroup"), cst("ChowGroup")))
}
/// `FundamentalClass : Scheme → ChowGroup X 0`
/// The fundamental class \[X\] ∈ A_d(X) (or A^0(X)).
pub fn fundamental_class_ty() -> Expr {
    arrow(cst("Scheme"), cst("ChowGroup"))
}
/// `BezoutIntersection : List Nat → Nat`
/// Given degrees d_1, …, d_r of hypersurfaces in P^r, the intersection number is ∏ d_i.
pub fn bezout_intersection_ty() -> Expr {
    arrow(list_ty(nat_ty()), nat_ty())
}
/// `bezout_theorem : ∀ (X : Scheme) (H_1 … H_r : Divisor X),
///   Transverse H_1 … H_r → #(H_1 ∩ … ∩ H_r) = ∏ deg(H_i)`
pub fn bezout_theorem_ty() -> Expr {
    arrow(
        list_ty(nat_ty()),
        arrow(prop(), app2(cst("Eq"), cst("Nat"), nat_ty())),
    )
}
/// `ExcessIntersectionFormula : ∀ (i : Morphism) (Z : Scheme),
///   i_* \[Z\] = e(E) ∩ \[Z\]` where E is the excess bundle.
pub fn excess_intersection_formula_ty() -> Expr {
    arrow(
        cst("Morphism"),
        arrow(cst("Scheme"), arrow(cst("ExcessBundle"), cst("ChowGroup"))),
    )
}
/// `ChernClass : VectorBundle → Nat → ChowGroup`
/// `c_k(E)` is the k-th Chern class of vector bundle E.
pub fn chern_class_ty() -> Expr {
    arrow(cst("VectorBundle"), arrow(nat_ty(), cst("ChowGroup")))
}
/// `TotalChernClass : VectorBundle → ChowRing`
/// `c(E) = 1 + c_1(E) + c_2(E) + …`
pub fn total_chern_class_ty() -> Expr {
    arrow(cst("VectorBundle"), cst("ChowRing"))
}
/// `ChernCharacter : VectorBundle → ChowRing ⊗ Q`
/// `ch(E) = rank(E) + c_1(E) + (c_1²(E) - 2c_2(E))/2 + …`
pub fn chern_character_ty() -> Expr {
    arrow(cst("VectorBundle"), cst("ChowRing"))
}
/// `whitney_sum_formula : c(E ⊕ F) = c(E) · c(F)`
pub fn whitney_sum_formula_ty() -> Expr {
    arrow(
        cst("VectorBundle"),
        arrow(
            cst("VectorBundle"),
            app2(
                cst("Eq"),
                cst("ChowRing"),
                app(
                    cst("TotalChernClass"),
                    app2(cst("DirectSum"), bvar(1), bvar(0)),
                ),
            ),
        ),
    )
}
/// `splitting_principle : ∀ (E : VectorBundle), ∃ (f : Morphism), f*E splits as line bundles`
pub fn splitting_principle_ty() -> Expr {
    arrow(cst("VectorBundle"), arrow(cst("Morphism"), prop()))
}
/// `SegreClass : Subscheme → Scheme → ChowGroup`
/// `s(Z, X)` is the Segre class of closed subscheme Z in X.
pub fn segre_class_ty() -> Expr {
    arrow(cst("Subscheme"), arrow(cst("Scheme"), cst("ChowGroup")))
}
/// `SelfIntersectionFormula : ∀ (i : Morphism) (Z : Scheme),
///   i* i_* \[Z\] = c_top(N_{Z/X}) ∩ \[Z\]`
/// where N_{Z/X} is the normal bundle.
pub fn self_intersection_formula_ty() -> Expr {
    arrow(
        cst("Morphism"),
        arrow(cst("Scheme"), arrow(cst("NormalBundle"), cst("ChowGroup"))),
    )
}
/// `segre_chern_relation : s(E) = c(E)^{-1}` in the Chow ring (formal inverse).
pub fn segre_chern_relation_ty() -> Expr {
    arrow(
        cst("VectorBundle"),
        app2(
            cst("Eq"),
            cst("ChowRing"),
            app(cst("InverseChow"), app(cst("TotalChernClass"), bvar(0))),
        ),
    )
}
/// `ToddClass : VectorBundle → ChowRing ⊗ Q`
/// `td(E) = ∏_i x_i / (1 - e^{-x_i})` where x_i are Chern roots.
pub fn todd_class_ty() -> Expr {
    arrow(cst("VectorBundle"), cst("ChowRing"))
}
/// `GrothendieckRiemannRoch : ∀ (f : Morphism X Y) (E : CoherentSheaf X),
///   ch(f_! E) · td(TY) = f_*(ch(E) · td(TX))`
pub fn grothendieck_riemann_roch_ty() -> Expr {
    arrow(
        cst("Morphism"),
        arrow(
            cst("CoherentSheaf"),
            app2(
                cst("Eq"),
                cst("ChowRing"),
                app2(
                    cst("ChowMul"),
                    app(cst("ChernCharacter"), cst("PushforwardSheaf")),
                    app(cst("ToddClass"), cst("TangentBundle")),
                ),
            ),
        ),
    )
}
/// `HirzebruchRiemannRoch : ∀ (X : SmoothProj) (E : VectorBundle),
///   χ(X, E) = ∫_X ch(E) · td(TX)`
pub fn hirzebruch_riemann_roch_ty() -> Expr {
    arrow(
        cst("SmoothProj"),
        arrow(
            cst("VectorBundle"),
            app2(cst("Eq"), int_ty(), app(cst("EulerChar"), cst("E"))),
        ),
    )
}
/// `ObstructionTheory : Morphism → Type`
/// A perfect obstruction theory on a scheme M is a morphism φ: E → L_M
/// where E is a two-term complex.
pub fn obstruction_theory_ty() -> Expr {
    arrow(cst("Morphism"), type0())
}
/// `VirtualFundamentalClass : ObstructionTheory → ChowGroup`
/// `[M]^{vir}` lives in A_{vd}(M) where vd = virtual dimension.
pub fn virtual_fundamental_class_ty() -> Expr {
    arrow(cst("ObstructionTheory"), cst("ChowGroup"))
}
/// `VirtualDimension : ObstructionTheory → Int`
/// `vd = rank(E^0) - rank(E^{-1})` for a perfect obstruction theory E.
pub fn virtual_dimension_ty() -> Expr {
    arrow(cst("ObstructionTheory"), int_ty())
}
/// `StableMap : Nat → Scheme → List Nat → Type`
/// `StableMap g X β` = moduli of stable genus-g maps to X in class β.
pub fn stable_map_ty() -> Expr {
    arrow(
        nat_ty(),
        arrow(cst("Scheme"), arrow(list_ty(nat_ty()), type1())),
    )
}
/// `GromovWittenInvariant : Scheme → Nat → List ChowClass → Int`
/// `GW_{g,β}(γ_1, …, γ_n)` is the GW invariant with insertions γ_i.
pub fn gromov_witten_invariant_ty() -> Expr {
    arrow(
        cst("Scheme"),
        arrow(nat_ty(), arrow(list_ty(cst("ChowClass")), int_ty())),
    )
}
/// `WDVVEquation : ∀ (X : Scheme) (a b c d : ChowClass),
///   ∑_{β} GW_{0,β}(a, b, e) · G^{ef} · GW_{0,β'}(f, c, d)
///   = ∑_{β} GW_{0,β}(a, c, e) · G^{ef} · GW_{0,β'}(f, b, d)`
/// (Witten-Dijkgraaf-Verlinde-Verlinde equation)
pub fn wdvv_equation_ty() -> Expr {
    arrow(
        cst("Scheme"),
        arrow(
            cst("ChowClass"),
            arrow(
                cst("ChowClass"),
                arrow(cst("ChowClass"), arrow(cst("ChowClass"), prop())),
            ),
        ),
    )
}
/// `EvaporationRelation : degeneration formula for GW invariants`
pub fn degeneration_formula_ty() -> Expr {
    arrow(
        cst("Morphism"),
        arrow(cst("GromovWittenInvariant"), cst("GromovWittenInvariant")),
    )
}
/// `QuantumCohomology : Scheme → Type`
/// `QH*(X)` = cohomology ring deformed by genus-0 GW invariants.
pub fn quantum_cohomology_ty() -> Expr {
    arrow(cst("Scheme"), type0())
}
/// `QuantumProduct : QH*(X) → QH*(X) → QH*(X)`
/// The small quantum product α ★ β = ∑_{β ≥ 0} ∑_{γ} GW_{0,β}(α, β, γ^∨) γ q^β.
pub fn quantum_product_ty() -> Expr {
    arrow(
        cst("QuantumCohomology"),
        arrow(cst("QuantumCohomology"), cst("QuantumCohomology")),
    )
}
/// `quantum_ring_associativity : (α ★ β) ★ γ = α ★ (β ★ γ)`
/// This is equivalent to the WDVV equations.
pub fn quantum_ring_associativity_ty() -> Expr {
    arrow(
        cst("QuantumCohomology"),
        arrow(
            cst("QuantumCohomology"),
            arrow(
                cst("QuantumCohomology"),
                app2(
                    cst("Eq"),
                    cst("QuantumCohomology"),
                    app2(
                        cst("QuantumProduct"),
                        app2(cst("QuantumProduct"), bvar(2), bvar(1)),
                        bvar(0),
                    ),
                ),
            ),
        ),
    )
}
/// `ConwayPotential : Scheme → Nat → Real`
/// Gromov-Witten potential Φ(t) = ∑_{g,β} (ℏ^{g-1} / n!) GW_{g,β}(t,…,t).
pub fn conway_potential_ty() -> Expr {
    arrow(cst("Scheme"), arrow(nat_ty(), real_ty()))
}
/// `DTInvariant : Scheme → CurveClass → Int`
/// The DT invariant counts ideal sheaves (or stable pairs) in class β.
pub fn dt_invariant_ty() -> Expr {
    arrow(cst("Scheme"), arrow(cst("CurveClass"), int_ty()))
}
/// `DT_GW_Correspondence : DTPartitionFunction X = GWPartitionFunction X (change of variables)`
/// The DT/GW correspondence relates the two partition functions under q ↔ -e^{iλ}.
pub fn dt_gw_correspondence_ty() -> Expr {
    arrow(
        cst("Scheme"),
        app2(
            cst("Eq"),
            cst("FormalPowerSeries"),
            app(cst("DTPartitionFunction"), bvar(0)),
        ),
    )
}
/// `VertexFormalism : LocalDT invariants via the topological vertex.`
pub fn vertex_formalism_ty() -> Expr {
    arrow(
        cst("ToricVariety"),
        arrow(
            cst("Partition"),
            arrow(cst("Partition"), cst("DTInvariant")),
        ),
    )
}
/// `NakajimaLectures : Hilbert scheme of points and DT theory on surfaces.`
pub fn hilbert_scheme_dt_ty() -> Expr {
    arrow(cst("Surface"), arrow(nat_ty(), cst("DTInvariant")))
}
/// `SchubertVariety : Grassmannian → Partition → Scheme`
/// The Schubert variety X_λ ⊂ G(k, n) indexed by a partition λ.
pub fn schubert_variety_ty() -> Expr {
    arrow(cst("Grassmannian"), arrow(cst("Partition"), cst("Scheme")))
}
/// `SchubertClass : Grassmannian → Partition → ChowGroup`
/// The Schubert class \[X_λ\] ∈ A^{|λ|}(G(k,n)).
pub fn schubert_class_ty() -> Expr {
    arrow(
        cst("Grassmannian"),
        arrow(cst("Partition"), cst("ChowGroup")),
    )
}
/// `PieriFormula : SchubertClass → Nat → ChowGroup`
/// Pieri's formula: σ_p · σ_λ = ∑_{μ} σ_μ over allowable μ.
pub fn pieri_formula_ty() -> Expr {
    arrow(cst("SchubertClass"), arrow(nat_ty(), cst("ChowGroup")))
}
/// `GiambellFormula : Partition → ChowGroup`
/// Giambelli's formula expresses σ_λ as a determinant of special Schubert classes.
pub fn giambelli_formula_ty() -> Expr {
    arrow(cst("Partition"), cst("ChowGroup"))
}
/// `LittlewoodRichardson : Partition → Partition → List ChowGroup`
/// LR coefficients c^μ_{λν}: σ_λ · σ_ν = ∑_μ c^μ_{λν} σ_μ.
pub fn littlewood_richardson_ty() -> Expr {
    arrow(
        cst("Partition"),
        arrow(cst("Partition"), list_ty(cst("ChowGroup"))),
    )
}
/// `GrassmannianChowRing : Nat → Nat → Type`
/// A^*(G(k,n)) = Z\[σ_1,…,σ_k\] / (Schubert relations).
pub fn grassmannian_chow_ring_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `FlagVariety : List Nat → Type`
/// The complete or partial flag variety FL(n_1, …, n_r; n).
pub fn flag_variety_ty() -> Expr {
    arrow(list_ty(nat_ty()), type0())
}
/// `SchubertPolynomial : Permutation → Polynomial`
/// Schubert polynomial S_w(x_1,…,x_n) representing \[X_w\] ∈ A*(FL(n)).
pub fn schubert_polynomial_ty() -> Expr {
    arrow(cst("Permutation"), cst("Polynomial"))
}
/// `Blowup : Scheme → Subscheme → Scheme`
/// The blow-up Bl_Z(X) of X along the closed subscheme Z.
pub fn blowup_ty() -> Expr {
    arrow(cst("Scheme"), arrow(cst("Subscheme"), cst("Scheme")))
}
/// `ExceptionalDivisor : BlowupScheme → Divisor`
/// The exceptional divisor E = π^{-1}(Z) ⊂ Bl_Z(X).
pub fn exceptional_divisor_ty() -> Expr {
    arrow(cst("BlowupScheme"), cst("Divisor"))
}
/// `BlowupChowRingMap : Scheme → Subscheme → ChowRing`
/// `A*(Bl_Z X) = A*(X)\[e\] / (relations involving e and A*(Z))`
pub fn blowup_chow_ring_map_ty() -> Expr {
    arrow(cst("Scheme"), arrow(cst("Subscheme"), cst("ChowRing")))
}
/// `StrictTransform : Scheme → BlowupScheme → Scheme`
/// The strict (proper) transform of a subvariety under blow-up.
pub fn strict_transform_ty() -> Expr {
    arrow(cst("Scheme"), arrow(cst("BlowupScheme"), cst("Scheme")))
}
/// `blowup_formula : A*(Bl_Z X) ≅ A*(X) ⊕ ⊕_{j=0}^{c-2} A*(Z) · e^j`
/// where c = codim(Z, X).
pub fn blowup_formula_ty() -> Expr {
    arrow(
        cst("Scheme"),
        arrow(
            cst("Subscheme"),
            app2(cst("Eq"), cst("ChowRing"), cst("ChowRing")),
        ),
    )
}
/// `DegeneracyLocus : BundleMap → Nat → Scheme`
/// D_k(φ) = {x : rank φ_x ≤ k} for a bundle map φ: E → F.
pub fn degeneracy_locus_ty() -> Expr {
    arrow(cst("BundleMap"), arrow(nat_ty(), cst("Scheme")))
}
/// `ThomPorteousFormula : ∀ (φ : BundleMap E F), \[D_k(φ)\] = Δ_{λ}(c(F-E))`
/// where Δ_λ is the Schur determinant in Chern classes.
pub fn thom_porteous_formula_ty() -> Expr {
    arrow(cst("BundleMap"), arrow(cst("Partition"), cst("ChowGroup")))
}
/// `EulerClass : VectorBundle → ChowGroup`
/// `e(E) = c_{rank}(E)` is the top Chern class, the Euler class of E.
pub fn euler_class_ty() -> Expr {
    arrow(cst("VectorBundle"), cst("ChowGroup"))
}
/// `euler_class_zero_locus : ∀ (E : VectorBundle) (s : BundleSection),
///   [Z(s)] = e(E) ∩ \[X\]`
pub fn euler_class_zero_locus_ty() -> Expr {
    arrow(
        cst("VectorBundle"),
        arrow(
            cst("BundleSection"),
            app2(cst("Eq"), cst("ChowGroup"), app(cst("EulerClass"), bvar(1))),
        ),
    )
}
/// `ModuliCurves : Nat → Nat → Type`
/// M_{g,n} = moduli space of smooth genus-g curves with n marked points.
pub fn moduli_curves_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type1()))
}
/// `DeligneMumford : Nat → Nat → Type`
/// The Deligne-Mumford compactification M̄_{g,n} of M_{g,n}.
pub fn deligne_mumford_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type1()))
}
/// `KontsevichSpace : Scheme → Nat → Nat → Type`
/// M̄_{g,n}(X, β) = Kontsevich's moduli of stable maps.
pub fn kontsevich_space_ty() -> Expr {
    arrow(cst("Scheme"), arrow(nat_ty(), arrow(nat_ty(), type1())))
}
/// `PsiClass : ModuliCurvesType → Nat → ChowGroup`
/// ψ_i = c_1(L_i) where L_i is the cotangent line bundle at the i-th marking.
pub fn psi_class_ty() -> Expr {
    arrow(cst("ModuliCurvesType"), arrow(nat_ty(), cst("ChowGroup")))
}
/// `LambdaClass : ModuliCurvesType → Nat → ChowGroup`
/// λ_i = c_i(E) where E = Hodge bundle over M̄_{g,n}.
pub fn lambda_class_ty() -> Expr {
    arrow(cst("ModuliCurvesType"), arrow(nat_ty(), cst("ChowGroup")))
}
/// `VirasaroConstraints : ∀ (g : Nat), GW potential satisfies Virasoro algebra.`
pub fn virasoro_constraints_ty() -> Expr {
    arrow(nat_ty(), arrow(cst("GWPotential"), prop()))
}
/// `DilationEquation : GW potential satisfies string and dilation equations.`
pub fn dilation_equation_ty() -> Expr {
    arrow(cst("GWPotential"), prop())
}
/// `MotivicCohomology : Scheme → Nat → Nat → Type`
/// H^{p,q}(X, Z) = motivic cohomology group.
pub fn motivic_cohomology_ty() -> Expr {
    arrow(cst("Scheme"), arrow(nat_ty(), arrow(nat_ty(), type0())))
}
/// `AlgebraicKTheory : Scheme → Nat → Type`
/// K_n(X) = Quillen's algebraic K-theory groups.
pub fn algebraic_k_theory_ty() -> Expr {
    arrow(cst("Scheme"), arrow(nat_ty(), type0()))
}
/// `ChernCharacterKTheory : KTheoryClass → ChowRing`
/// ch: K_0(X) ⊗ Q → A*(X) ⊗ Q is an isomorphism of rings.
pub fn chern_character_k_theory_ty() -> Expr {
    arrow(cst("KTheoryClass"), cst("ChowRing"))
}
/// `AtiyahHirzebruch : K_0(X) ≅ A*(X) after tensoring with Q (for smooth X).`
pub fn atiyah_hirzebruch_ty() -> Expr {
    arrow(
        cst("Scheme"),
        app2(cst("Eq"), type0(), app(cst("K0Ring"), bvar(0))),
    )
}
/// `MotivicIntegration : Scheme → Real`
/// Motivic volume / integral as a function on motivic cohomology.
pub fn motivic_integration_ty() -> Expr {
    arrow(cst("Scheme"), real_ty())
}
/// `HilbertSchemePoints : Scheme → Nat → Type`
/// Hilb^n(X) = Hilbert scheme of n points on X.
pub fn hilbert_scheme_points_ty() -> Expr {
    arrow(cst("Scheme"), arrow(nat_ty(), type1()))
}
/// `HilbertChow : HilbertSchemeType → SymmetricProduct`
/// The Hilbert-Chow morphism Hilb^n(X) → X^{(n)}.
pub fn hilbert_chow_ty() -> Expr {
    arrow(cst("HilbertSchemeType"), cst("SymmetricProduct"))
}
/// `NakajimaOperator : HilbertSchemeType → ChowGroup → ChowGroup`
/// Nakajima's creation/annihilation operators on H*(Hilb*(S)).
pub fn nakajima_operator_ty() -> Expr {
    arrow(
        cst("HilbertSchemeType"),
        arrow(cst("ChowGroup"), cst("ChowGroup")),
    )
}
/// `GottscheFormula : Surface → Nat → Int`
/// Göttsche's formula for χ(Hilb^n(S)) in terms of χ(S).
pub fn gottsche_formula_ty() -> Expr {
    arrow(cst("Surface"), arrow(nat_ty(), int_ty()))
}
/// `IntersectionMultiplicity : Scheme → Scheme → Scheme → Nat`
/// The local intersection multiplicity i(X; Y · Z; W) at a subvariety W.
pub fn intersection_multiplicity_ty() -> Expr {
    arrow(
        cst("Scheme"),
        arrow(cst("Scheme"), arrow(cst("Scheme"), nat_ty())),
    )
}
/// `PolynomialResultant : Polynomial → Polynomial → Polynomial`
/// The resultant Res(f, g) eliminating a variable from two polynomials.
pub fn polynomial_resultant_ty() -> Expr {
    arrow(
        cst("Polynomial"),
        arrow(cst("Polynomial"), cst("Polynomial")),
    )
}
/// `IntersectionMultiplicityFormula : ∀ transversal intersections,
///   the sum of local multiplicities equals the Bezout number.`
pub fn intersection_multiplicity_formula_ty() -> Expr {
    arrow(list_ty(cst("Polynomial")), arrow(nat_ty(), prop()))
}
/// Register all intersection theory declarations in the kernel environment.
pub fn build_intersection_theory_env(env: &mut Environment) {
    let base_types: &[(&str, Expr)] = &[
        ("Scheme", type1()),
        ("VectorBundle", type0()),
        ("CoherentSheaf", type0()),
        ("ChowGroup", type0()),
        ("ChowRing", type0()),
        ("ChowClass", type0()),
        ("AlgebraicCycle", type0()),
        ("Morphism", type0()),
        ("Subscheme", type0()),
        ("NormalBundle", type0()),
        ("ExcessBundle", type0()),
        ("ObstructionTheory", type0()),
        ("FormalPowerSeries", type0()),
        ("ToricVariety", type0()),
        ("Surface", type0()),
        ("SmoothProj", type0()),
        ("Partition", type0()),
        ("CurveClass", type0()),
        ("GromovWittenInvariant", type0()),
        ("QuantumCohomology", type0()),
        ("DTInvariant", type0()),
        ("Nat", type0()),
        ("Int", type0()),
        ("Real", type0()),
        ("Bool", type0()),
        ("Eq", arrow(type0(), arrow(type0(), prop()))),
        ("And", arrow(prop(), arrow(prop(), prop()))),
        ("Exists", arrow(type0(), arrow(type0(), prop()))),
        ("List", arrow(type0(), type0())),
        (
            "DirectSum",
            arrow(
                cst("VectorBundle"),
                arrow(cst("VectorBundle"), cst("VectorBundle")),
            ),
        ),
        (
            "ChowMul",
            arrow(cst("ChowRing"), arrow(cst("ChowRing"), cst("ChowRing"))),
        ),
        ("InverseChow", arrow(cst("ChowRing"), cst("ChowRing"))),
        ("PushforwardSheaf", cst("CoherentSheaf")),
        ("TangentBundle", cst("VectorBundle")),
        ("EulerChar", arrow(cst("CoherentSheaf"), int_ty())),
        (
            "DTPartitionFunction",
            arrow(cst("Scheme"), cst("FormalPowerSeries")),
        ),
        (
            "GWPartitionFunction",
            arrow(cst("Scheme"), cst("FormalPowerSeries")),
        ),
        ("Grassmannian", type0()),
        ("SchubertClass", type0()),
        ("Permutation", type0()),
        ("Polynomial", type0()),
        ("BlowupScheme", type0()),
        ("Divisor", type0()),
        ("BundleMap", type0()),
        ("BundleSection", type0()),
        ("ModuliCurvesType", type0()),
        ("GWPotential", type0()),
        ("KTheoryClass", type0()),
        ("K0Ring", arrow(cst("Scheme"), type0())),
        ("HilbertSchemeType", type0()),
        ("SymmetricProduct", type0()),
        ("EulerClass", arrow(cst("VectorBundle"), cst("ChowGroup"))),
    ];
    for (name, ty) in base_types {
        env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty: ty.clone(),
        })
        .ok();
    }
    let type_axioms: &[(&str, fn() -> Expr)] = &[
        ("chow_group", chow_group_ty),
        ("chow_ring", chow_ring_ty),
        ("algebraic_cycle", algebraic_cycle_ty),
        ("intersection_product", intersection_product_ty),
        ("pushforward_cycle", pushforward_cycle_ty),
        ("pullback_cycle", pullback_cycle_ty),
        ("fundamental_class", fundamental_class_ty),
        ("bezout_intersection", bezout_intersection_ty),
        ("chern_class", chern_class_ty),
        ("total_chern_class", total_chern_class_ty),
        ("chern_character", chern_character_ty),
        ("segre_class", segre_class_ty),
        ("todd_class", todd_class_ty),
        ("obstruction_theory", obstruction_theory_ty),
        ("virtual_fundamental_class", virtual_fundamental_class_ty),
        ("virtual_dimension", virtual_dimension_ty),
        ("stable_map", stable_map_ty),
        ("gromov_witten_invariant", gromov_witten_invariant_ty),
        ("quantum_cohomology", quantum_cohomology_ty),
        ("quantum_product", quantum_product_ty),
        ("dt_invariant", dt_invariant_ty),
        ("schubert_variety", schubert_variety_ty),
        ("schubert_class", schubert_class_ty),
        ("pieri_formula", pieri_formula_ty),
        ("giambelli_formula", giambelli_formula_ty),
        ("littlewood_richardson", littlewood_richardson_ty),
        ("grassmannian_chow_ring", grassmannian_chow_ring_ty),
        ("flag_variety", flag_variety_ty),
        ("schubert_polynomial", schubert_polynomial_ty),
        ("blowup", blowup_ty),
        ("exceptional_divisor", exceptional_divisor_ty),
        ("blowup_chow_ring_map", blowup_chow_ring_map_ty),
        ("strict_transform", strict_transform_ty),
        ("degeneracy_locus", degeneracy_locus_ty),
        ("euler_class", euler_class_ty),
        ("moduli_curves", moduli_curves_ty),
        ("deligne_mumford", deligne_mumford_ty),
        ("kontsevich_space", kontsevich_space_ty),
        ("psi_class", psi_class_ty),
        ("lambda_class", lambda_class_ty),
        ("motivic_cohomology", motivic_cohomology_ty),
        ("algebraic_k_theory", algebraic_k_theory_ty),
        ("chern_character_k_theory", chern_character_k_theory_ty),
        ("motivic_integration", motivic_integration_ty),
        ("hilbert_scheme_points", hilbert_scheme_points_ty),
        ("hilbert_chow", hilbert_chow_ty),
        ("nakajima_operator", nakajima_operator_ty),
        ("gottsche_formula", gottsche_formula_ty),
        ("intersection_multiplicity", intersection_multiplicity_ty),
        ("polynomial_resultant", polynomial_resultant_ty),
    ];
    for (name, mk_ty) in type_axioms {
        let ty = mk_ty();
        env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty,
        })
        .ok();
    }
    let theorem_axioms: &[(&str, fn() -> Expr)] = &[
        ("bezout_theorem", bezout_theorem_ty),
        (
            "excess_intersection_formula",
            excess_intersection_formula_ty,
        ),
        ("whitney_sum_formula", whitney_sum_formula_ty),
        ("splitting_principle", splitting_principle_ty),
        ("self_intersection_formula", self_intersection_formula_ty),
        ("segre_chern_relation", segre_chern_relation_ty),
        ("grothendieck_riemann_roch", grothendieck_riemann_roch_ty),
        ("hirzebruch_riemann_roch", hirzebruch_riemann_roch_ty),
        ("wdvv_equation", wdvv_equation_ty),
        ("degeneration_formula", degeneration_formula_ty),
        ("quantum_ring_associativity", quantum_ring_associativity_ty),
        ("dt_gw_correspondence", dt_gw_correspondence_ty),
        ("thom_porteous_formula", thom_porteous_formula_ty),
        ("euler_class_zero_locus", euler_class_zero_locus_ty),
        ("blowup_formula", blowup_formula_ty),
        ("virasoro_constraints", virasoro_constraints_ty),
        ("dilation_equation", dilation_equation_ty),
        ("atiyah_hirzebruch", atiyah_hirzebruch_ty),
        (
            "intersection_multiplicity_formula",
            intersection_multiplicity_formula_ty,
        ),
    ];
    for (name, mk_ty) in theorem_axioms {
        let ty = mk_ty();
        env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty,
        })
        .ok();
    }
}
/// Compute the Bezout intersection number: product of degrees.
///
/// Given hypersurfaces of degrees d_1, …, d_r in P^r, the number of
/// intersection points (counted with multiplicity) is ∏ d_i.
pub fn bezout_number(degrees: &[u64]) -> u64 {
    degrees.iter().product()
}
/// Compute the Euler characteristic using Betti numbers (alternating sum).
pub fn euler_characteristic_from_betti(betti: &[i64]) -> i64 {
    betti
        .iter()
        .enumerate()
        .map(|(i, &b)| if i % 2 == 0 { b } else { -b })
        .sum()
}
/// Compute the degree of a line bundle on P^n.
///
/// The Chow ring of P^n is Z\[H\]/(H^{n+1}), and a line bundle O(d) has degree d.
pub fn projective_line_bundle_degree(n: usize, power: u64) -> u64 {
    if power == n as u64 {
        1
    } else if power > n as u64 {
        0
    } else {
        1
    }
}
/// Intersection number of two cycle classes (pairing on a variety of dimension d).
/// Returns Some(product) if codimensions sum to d, None otherwise.
pub fn intersection_number(c1: &CycleClass, c2: &CycleClass, dim: usize) -> Option<i64> {
    if c1.codim + c2.codim == dim {
        Some(c1.multiplicity * c2.multiplicity)
    } else {
        None
    }
}
/// Gromov-Witten invariant (genus-0, 3-point) using a simple model.
///
/// For P^n, the invariant GW_{0,d}(H^{a_1}, H^{a_2}, H^{a_3}) = d^{n-1}
/// when a_1 + a_2 + a_3 = n + (n-3) * d (dimension constraint), else 0.
pub fn gw_invariant_pn(n: usize, d: u64, a1: usize, a2: usize, a3: usize) -> i64 {
    let lhs = a1 + a2 + a3;
    let rhs = n + (n as i64 - 3) as usize * d as usize;
    if lhs == rhs && d > 0 {
        (d as i64).pow((n as u32).saturating_sub(1))
    } else if d == 0 && lhs == n {
        1
    } else {
        0
    }
}
/// Donaldson-Thomas partition function for P^2 × P^1 (simple model).
///
/// Z_{DT}(q) = M(-q)^{χ(X)} where M is the MacMahon function (plane partitions).
pub fn dt_partition_function_coeff(euler_char: i64, degree: usize) -> f64 {
    let mut coeff = vec![0.0f64; degree + 1];
    coeff[0] = 1.0;
    for n in 1..=degree {
        for _k in 1..=n {
            for j in (n..=degree).rev() {
                coeff[j] += coeff[j - n];
            }
        }
    }
    let sign: f64 = if euler_char % 2 == 0 { 1.0 } else { -1.0 };
    sign * coeff[degree]
}
/// Todd class coefficients of P^n.
///
/// td(TP^n) = ((H)/(1 - e^{-H}))^{n+1} in the Chow ring Q\[H\]/(H^{n+1}).
/// Returns the coefficients td_0, td_1, …, td_n.
pub fn todd_class_projective_space(n: usize) -> Vec<f64> {
    let n1 = (n + 1) as f64;
    let mut td = vec![0.0f64; n + 1];
    if n + 1 > 0 {
        td[0] = 1.0;
    }
    if n + 1 > 1 {
        td[1] = n1 / 2.0;
    }
    if n + 1 > 2 {
        td[2] = n1 * (n1 + 1.0) / 12.0;
    }
    if n + 1 > 3 {
        td[3] = n1 * n1 * (n1 + 1.0) / 24.0;
    }
    td
}
/// Hirzebruch-Riemann-Roch: compute χ(P^n, O(d)) = C(n+d, n).
///
/// The Euler characteristic of O(d) on P^n is the binomial coefficient C(n+d, n).
pub fn hrr_projective_space(n: usize, d: i64) -> i64 {
    if d >= 0 {
        binomial(n as i64 + d, n as i64)
    } else {
        let sign = if n % 2 == 0 { 1 } else { -1 };
        sign * hrr_projective_space(n, -(n as i64) - 1 - d)
    }
}
/// Binomial coefficient C(n, k).
pub fn binomial(n: i64, k: i64) -> i64 {
    if k < 0 || k > n {
        return 0;
    }
    if k == 0 || k == n {
        return 1;
    }
    let k = k.min(n - k) as usize;
    let mut result = 1i64;
    for i in 0..k as i64 {
        result = result * (n - i) / (i + 1);
    }
    result
}
#[cfg(test)]
mod tests {
    use super::*;
    use oxilean_kernel::Environment;
    #[test]
    fn test_bezout_number() {
        assert_eq!(bezout_number(&[2, 2, 2]), 8);
        assert_eq!(bezout_number(&[1, 2]), 2);
        assert_eq!(bezout_number(&[]), 1);
    }
    #[test]
    fn test_vector_bundle_direct_sum() {
        let e = VectorBundleData::line_bundle(1);
        let f = VectorBundleData::line_bundle(2);
        let ef = e.direct_sum(&f);
        assert_eq!(ef.rank, 2);
        assert_eq!(ef.chern_numbers[0], 3);
        assert_eq!(ef.chern_numbers[1], 2);
    }
    #[test]
    fn test_chern_character_line_bundle() {
        let e = VectorBundleData::line_bundle(3);
        let ch = e.chern_character_coeffs();
        assert_eq!(ch.len(), 3);
        assert_eq!(ch[0], 1.0);
        assert_eq!(ch[1], 3.0);
        assert!((ch[2] - 4.5).abs() < 1e-10);
    }
    #[test]
    fn test_cycle_class_operations() {
        let h = CycleClass::new(1, 1, "H");
        let h2 = h.scale(2);
        assert_eq!(h2.multiplicity, 2);
        assert_eq!(h2.codim, 1);
        let h3 = CycleClass::new(1, 3, "3H");
        let sum = h.add(&h3).expect("add should succeed");
        assert_eq!(sum.multiplicity, 4);
        let pt = CycleClass::new(2, 1, "pt");
        assert!(h.add(&pt).is_none());
    }
    #[test]
    fn test_intersection_number() {
        let h = CycleClass::new(1, 1, "H");
        let result = intersection_number(&h, &h, 2);
        assert_eq!(result, Some(1));
        let pt = CycleClass::new(2, 1, "pt");
        assert_eq!(intersection_number(&h, &pt, 2), None);
    }
    #[test]
    fn test_hrr_projective_space() {
        assert_eq!(hrr_projective_space(2, 0), 1);
        assert_eq!(hrr_projective_space(2, 1), 3);
        assert_eq!(hrr_projective_space(2, 2), 6);
        assert_eq!(hrr_projective_space(1, 3), 4);
    }
    #[test]
    fn test_quantum_cohomology_p2() {
        let qh = QuantumCohomologyP2::new(1.0);
        let (c1, c_h, c_h2) = qh.quantum_product_h_powers(1, 2);
        assert!((c1 - 1.0).abs() < 1e-10);
        assert!((c_h).abs() < 1e-10);
        assert!((c_h2).abs() < 1e-10);
    }
    #[test]
    fn test_build_intersection_theory_env() {
        let mut env = Environment::new();
        build_intersection_theory_env(&mut env);
        assert!(env.get(&Name::str("chow_group")).is_some());
        assert!(env.get(&Name::str("chern_class")).is_some());
        assert!(env.get(&Name::str("bezout_theorem")).is_some());
        assert!(env.get(&Name::str("grothendieck_riemann_roch")).is_some());
        assert!(env.get(&Name::str("wdvv_equation")).is_some());
        assert!(env.get(&Name::str("dt_gw_correspondence")).is_some());
        assert!(env.get(&Name::str("quantum_product")).is_some());
        assert!(env.get(&Name::str("schubert_class")).is_some());
        assert!(env.get(&Name::str("pieri_formula")).is_some());
        assert!(env.get(&Name::str("giambelli_formula")).is_some());
        assert!(env.get(&Name::str("littlewood_richardson")).is_some());
        assert!(env.get(&Name::str("grassmannian_chow_ring")).is_some());
        assert!(env.get(&Name::str("flag_variety")).is_some());
        assert!(env.get(&Name::str("schubert_polynomial")).is_some());
        assert!(env.get(&Name::str("blowup")).is_some());
        assert!(env.get(&Name::str("exceptional_divisor")).is_some());
        assert!(env.get(&Name::str("strict_transform")).is_some());
        assert!(env.get(&Name::str("blowup_formula")).is_some());
        assert!(env.get(&Name::str("degeneracy_locus")).is_some());
        assert!(env.get(&Name::str("thom_porteous_formula")).is_some());
        assert!(env.get(&Name::str("euler_class")).is_some());
        assert!(env.get(&Name::str("deligne_mumford")).is_some());
        assert!(env.get(&Name::str("kontsevich_space")).is_some());
        assert!(env.get(&Name::str("psi_class")).is_some());
        assert!(env.get(&Name::str("lambda_class")).is_some());
        assert!(env.get(&Name::str("virasoro_constraints")).is_some());
        assert!(env.get(&Name::str("motivic_cohomology")).is_some());
        assert!(env.get(&Name::str("algebraic_k_theory")).is_some());
        assert!(env.get(&Name::str("atiyah_hirzebruch")).is_some());
        assert!(env.get(&Name::str("hilbert_scheme_points")).is_some());
        assert!(env.get(&Name::str("nakajima_operator")).is_some());
        assert!(env.get(&Name::str("gottsche_formula")).is_some());
        assert!(env.get(&Name::str("intersection_multiplicity")).is_some());
        assert!(env.get(&Name::str("polynomial_resultant")).is_some());
    }
    #[test]
    fn test_todd_class_projective_space() {
        let td = todd_class_projective_space(2);
        assert!((td[0] - 1.0).abs() < 1e-10);
        assert!((td[1] - 1.5).abs() < 1e-10);
    }
    #[test]
    fn test_chow_ring_elem_arithmetic() {
        let h = ChowRingElem::hyperplane(2);
        let h2 = h.mul(&h).expect("mul should succeed");
        assert_eq!(h2.coeffs, vec![0, 0, 1]);
        let h3 = h2.mul(&h).expect("mul should succeed");
        assert_eq!(h3.degree(), 0);
        let one = ChowRingElem::one(2);
        let one_plus_h = one.add(&h).expect("add should succeed");
        assert_eq!(one_plus_h.coeffs[0], 1);
        assert_eq!(one_plus_h.coeffs[1], 1);
    }
    #[test]
    fn test_intersection_matrix() {
        let mat = IntersectionMatrix::new(vec![vec![-1]]);
        assert_eq!(mat.self_intersections(), vec![-1]);
        assert_eq!(mat.determinant(), Some(-1));
        let mat2 = IntersectionMatrix::new(vec![vec![-2, 1], vec![1, -2]]);
        assert_eq!(mat2.determinant(), Some(3));
        assert!(mat2.is_negative_definite());
    }
    #[test]
    fn test_schubert_calc_grassmannian() {
        let calc = SchubertCalc::new(2, 4);
        assert_eq!(calc.dim(), 4);
        assert!(calc.is_valid_partition(&[2, 1]));
        assert!(calc.is_valid_partition(&[1]));
        assert!(!calc.is_valid_partition(&[3]));
        assert_eq!(calc.degree(), 2);
    }
    #[test]
    fn test_bezout_bound() {
        let b = BezoutBound::new(vec![2, 3, 4]);
        assert_eq!(b.bound(), 24);
        assert_eq!(b.mixed_bound(&[0, 1]), 6);
        assert!(b.is_overdetermined(2));
        assert!(!b.is_overdetermined(3));
        assert!(b.is_underdetermined(4));
    }
    #[test]
    fn test_vector_bundle_euler_and_todd() {
        let e = VectorBundleData::line_bundle(3);
        assert_eq!(e.euler_class(), 3);
        let td = e.todd_class_coeffs();
        assert!((td[0] - 1.0).abs() < 1e-10);
        assert!((td[1] - 1.5).abs() < 1e-10);
    }
}
/// `ChowGroupHomomorphism : ChowGroup X → ChowGroup Y → Prop`
/// A group homomorphism between Chow groups.
pub fn it_ext_chow_group_hom_ty() -> Expr {
    arrow(cst("ChowGroup"), arrow(cst("ChowGroup"), prop()))
}
/// `ChowRingIsomorphism : ChowRing X → ChowRing Y → Prop`
/// An isomorphism of graded rings.
pub fn it_ext_chow_ring_iso_ty() -> Expr {
    arrow(cst("ChowRing"), arrow(cst("ChowRing"), prop()))
}
/// `CycleMap : Scheme → MotivicCohomology → ChowGroup`
/// The cycle class map from motivic cohomology to Chow groups.
pub fn it_ext_cycle_map_ty() -> Expr {
    arrow(
        cst("Scheme"),
        arrow(cst("MotivicCohomology"), cst("ChowGroup")),
    )
}
/// `RationalEquivalenceClass : AlgebraicCycle → ChowGroup`
/// The rational equivalence class of a cycle.
pub fn it_ext_rational_equiv_class_ty() -> Expr {
    arrow(cst("AlgebraicCycle"), cst("ChowGroup"))
}
/// `NumericalEquivalence : AlgebraicCycle → AlgebraicCycle → Prop`
/// Two cycles are numerically equivalent if they have the same intersection numbers.
pub fn it_ext_numerical_equivalence_ty() -> Expr {
    arrow(cst("AlgebraicCycle"), arrow(cst("AlgebraicCycle"), prop()))
}
/// `HomologicalEquivalence : AlgebraicCycle → AlgebraicCycle → Prop`
/// Two cycles are homologically equivalent if they have the same image in cohomology.
pub fn it_ext_homological_equivalence_ty() -> Expr {
    arrow(cst("AlgebraicCycle"), arrow(cst("AlgebraicCycle"), prop()))
}
/// `ProperPushforward : ProperMorphism X Y → ChowGroup X → ChowGroup Y`
/// Proper push-forward: defined for all cycle classes.
pub fn it_ext_proper_pushforward_ty() -> Expr {
    arrow(cst("Morphism"), arrow(cst("ChowGroup"), cst("ChowGroup")))
}
/// `FlatPullback : FlatMorphism X Y → ChowGroup Y → ChowGroup X`
/// Flat pull-back preserves codimension.
pub fn it_ext_flat_pullback_ty() -> Expr {
    arrow(cst("Morphism"), arrow(cst("ChowGroup"), cst("ChowGroup")))
}
/// `LciPullback : LciMorphism X Y → ChowGroup Y → ChowGroup X`
/// Pull-back for local complete intersection (lci) morphisms.
pub fn it_ext_lci_pullback_ty() -> Expr {
    arrow(cst("Morphism"), arrow(cst("ChowGroup"), cst("ChowGroup")))
}
/// `ProjectionFormula : f_*(f*(α) · β) = α · f_*(β)` for proper f.
pub fn it_ext_projection_formula_ty() -> Expr {
    arrow(
        cst("Morphism"),
        arrow(
            cst("ChowGroup"),
            arrow(
                cst("ChowGroup"),
                app2(cst("Eq"), cst("ChowGroup"), cst("ChowGroup")),
            ),
        ),
    )
}
/// `DegreeOfMap : ProperMorphism X Y → Nat`
/// Degree of a proper map between equidimensional varieties.
pub fn it_ext_degree_of_map_ty() -> Expr {
    arrow(cst("Morphism"), nat_ty())
}
/// `ChernClassFunctoriality : f*(c_k(E)) = c_k(f*E)` for any morphism f.
pub fn it_ext_chern_class_funct_ty() -> Expr {
    arrow(
        cst("Morphism"),
        arrow(
            cst("VectorBundle"),
            app2(cst("Eq"), cst("ChowGroup"), cst("ChowGroup")),
        ),
    )
}
/// `ChernRootDecomposition : E = L_1 ⊕ … ⊕ L_r, c_k(E) = e_k(x_1,…,x_r)`
/// Chern roots splitting: elementary symmetric polynomials in Chern roots.
pub fn it_ext_chern_root_decomp_ty() -> Expr {
    arrow(cst("VectorBundle"), arrow(nat_ty(), cst("ChowGroup")))
}
/// `GrothendieckGroup : K_0(X) ≅ ⊕_k K_0^k(X) with Adams operations`
/// The Grothendieck group K_0(X) with filtration.
pub fn it_ext_grothendieck_group_ty() -> Expr {
    arrow(cst("Scheme"), type0())
}
/// `AdamsOperation : KTheoryClass → Nat → KTheoryClass`
/// Adams operation ψ^k: K_0(X) → K_0(X) satisfying ψ^k(L) = L^⊗k for line bundles.
pub fn it_ext_adams_operation_ty() -> Expr {
    arrow(cst("KTheoryClass"), arrow(nat_ty(), cst("KTheoryClass")))
}
/// `RiemannRochTransformation : ChernCharacter intertwines push-forwards`
/// ch ∘ f_! = f_* ∘ (ch · td(TX)) in appropriate sense.
pub fn it_ext_riemann_roch_transform_ty() -> Expr {
    arrow(
        cst("Morphism"),
        arrow(
            cst("CoherentSheaf"),
            app2(cst("Eq"), cst("ChowRing"), cst("ChowRing")),
        ),
    )
}
/// `LefschetzFixedPoint : Morphism X X → Int`
/// The Lefschetz number L(f) = ∑_k (-1)^k tr(f* | H^k(X)).
pub fn it_ext_lefschetz_number_ty() -> Expr {
    arrow(cst("Morphism"), int_ty())
}
/// `LefschetzFixedPointThm : L(f) ≠ 0 → f has a fixed point`
/// Lefschetz fixed-point theorem.
pub fn it_ext_lefschetz_fpt_ty() -> Expr {
    arrow(
        cst("Morphism"),
        arrow(app2(cst("Eq"), int_ty(), int_ty()), prop()),
    )
}
/// `RiemannHurwitz : 2g(X) - 2 = deg(f) * (2g(Y) - 2) + ∑ (e_p - 1)`
/// Riemann-Hurwitz formula for a map of smooth curves.
pub fn it_ext_riemann_hurwitz_ty() -> Expr {
    arrow(cst("Morphism"), arrow(nat_ty(), arrow(nat_ty(), prop())))
}
/// `ZetaFunction : Scheme → FormalPowerSeries`
/// The zeta function Z(X, t) of a variety over a finite field.
pub fn it_ext_zeta_function_ty() -> Expr {
    arrow(cst("Scheme"), cst("FormalPowerSeries"))
}
/// `WeilConjectures : Scheme → Prop`
/// Rationality, functional equation, Riemann hypothesis for varieties over finite fields.
pub fn it_ext_weil_conjectures_ty() -> Expr {
    arrow(cst("Scheme"), prop())
}
/// `KontsevichFormula : GW_{0,d}(line, line, line) on P^2 = N_d (number of rational curves)`
/// Kontsevich's recursive formula for rational plane curves.
pub fn it_ext_kontsevich_formula_ty() -> Expr {
    arrow(nat_ty(), nat_ty())
}
/// `QuantumKohomology : Frobenius manifold structure on QH*(X)`
pub fn it_ext_quantum_frobenius_ty() -> Expr {
    arrow(cst("QuantumCohomology"), prop())
}
/// `MirrorSymmetry : QH*(X) ≅ H*(X^∨) (A-model ↔ B-model duality)`
pub fn it_ext_mirror_symmetry_ty() -> Expr {
    arrow(cst("Scheme"), arrow(cst("Scheme"), prop()))
}
/// `TopologicalVertexFormalism : Partition → Partition → Partition → Int`
/// The topological vertex C_{λμν} in DT/GW theory.
pub fn it_ext_topological_vertex_ty() -> Expr {
    arrow(
        cst("Partition"),
        arrow(cst("Partition"), arrow(cst("Partition"), int_ty())),
    )
}
/// `GWPotentialGenus0 : Genus-0 Gromov-Witten potential F_0(t)`
pub fn it_ext_gw_potential_genus0_ty() -> Expr {
    arrow(cst("Scheme"), arrow(list_ty(real_ty()), real_ty()))
}
/// `StringEquation : ∂F_0/∂t_0 = ∑ t_a t_b GW_{a,b}` string equation for GW potential.
pub fn it_ext_string_equation_ty() -> Expr {
    arrow(cst("GWPotential"), prop())
}
/// `PerfectObstructionTheory : Scheme → Morphism → Type`
/// A two-term complex E^{-1} → E^0 → L_M constituting a POT.
pub fn it_ext_perfect_obstruction_ty() -> Expr {
    arrow(cst("Scheme"), arrow(cst("Morphism"), type0()))
}
/// `VirtualNormalBundle : ObstructionTheory → VectorBundle`
/// Virtual normal bundle from the POT: N^{vir} = E^1.
pub fn it_ext_virtual_normal_bundle_ty() -> Expr {
    arrow(cst("ObstructionTheory"), cst("VectorBundle"))
}
/// `VirtualPushforward : VirtualFundamentalClass → ChowGroup → Int`
/// Degree of a cohomology class against the virtual fundamental class.
pub fn it_ext_virtual_pushforward_ty() -> Expr {
    arrow(cst("ObstructionTheory"), arrow(cst("ChowGroup"), int_ty()))
}
/// `DeformationInvariance : Virtual class is invariant under deformation.`
pub fn it_ext_deformation_invariance_ty() -> Expr {
    arrow(cst("ObstructionTheory"), prop())
}
/// `SchurDeterminant : Partition → VectorBundle → VectorBundle → ChowGroup`
/// The Schur determinant Δ_λ(c(F - E)) appearing in the Porteous formula.
pub fn it_ext_schur_determinant_ty() -> Expr {
    arrow(
        cst("Partition"),
        arrow(
            cst("VectorBundle"),
            arrow(cst("VectorBundle"), cst("ChowGroup")),
        ),
    )
}
/// `GrassmannBundleFormula : ChowRing of Grassmann bundle over X.`
pub fn it_ext_grassmann_bundle_formula_ty() -> Expr {
    arrow(
        cst("Scheme"),
        arrow(cst("VectorBundle"), arrow(nat_ty(), cst("ChowRing"))),
    )
}
/// `ProjectiveBundleFormula : A*(P(E)) = A*(X)\[ξ\] / (∑ (-1)^k c_k(E) ξ^{r-k})`
/// Chow ring of a projective bundle P(E) over X.
pub fn it_ext_projective_bundle_formula_ty() -> Expr {
    arrow(cst("Scheme"), arrow(cst("VectorBundle"), cst("ChowRing")))
}
/// `MotivicSpectralSequence : H^{p,q}(X) → K_n(X) (Atiyah-Hirzebruch type)`
pub fn it_ext_motivic_spectral_seq_ty() -> Expr {
    arrow(cst("Scheme"), arrow(nat_ty(), arrow(nat_ty(), type0())))
}
/// `Bloch-Ogus cohomology: niveau spectral sequence for motivic cohomology.`
pub fn it_ext_bloch_ogus_ty() -> Expr {
    arrow(cst("Scheme"), arrow(nat_ty(), type0()))
}
/// `GersteenResolution : Exact sequence for K-sheaves on smooth X.`
pub fn it_ext_gersten_resolution_ty() -> Expr {
    arrow(cst("Scheme"), type0())
}
/// `MilnorKTheory : Field → Nat → Type`
/// Milnor K-theory K^M_n(F) = F* ⊗ … ⊗ F* / Steinberg relations.
pub fn it_ext_milnor_k_theory_ty() -> Expr {
    arrow(cst("Scheme"), arrow(nat_ty(), type0()))
}
/// Register all extended intersection theory axioms in the kernel environment.
pub fn register_intersection_theory_extended(env: &mut Environment) -> Result<(), String> {
    let extra_types: &[(&str, Expr)] = &[("MotivicCohomology", type0()), ("Frobenius", type0())];
    for (name, ty) in extra_types {
        env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty: ty.clone(),
        })
        .ok();
    }
    let axioms: &[(&str, fn() -> Expr)] = &[
        ("ChowGroupHom", it_ext_chow_group_hom_ty),
        ("ChowRingIso", it_ext_chow_ring_iso_ty),
        ("CycleMap", it_ext_cycle_map_ty),
        ("RationalEquivClass", it_ext_rational_equiv_class_ty),
        ("NumericalEquivalence", it_ext_numerical_equivalence_ty),
        ("HomologicalEquivalence", it_ext_homological_equivalence_ty),
        ("ProperPushforward", it_ext_proper_pushforward_ty),
        ("FlatPullback", it_ext_flat_pullback_ty),
        ("LciPullback", it_ext_lci_pullback_ty),
        ("ProjectionFormula", it_ext_projection_formula_ty),
        ("DegreeOfMap", it_ext_degree_of_map_ty),
        ("ChernClassFunct", it_ext_chern_class_funct_ty),
        ("ChernRootDecomp", it_ext_chern_root_decomp_ty),
        ("GrothendieckGroupExt", it_ext_grothendieck_group_ty),
        ("AdamsOperation", it_ext_adams_operation_ty),
        ("RiemannRochTransform", it_ext_riemann_roch_transform_ty),
        ("LefschetzNumber", it_ext_lefschetz_number_ty),
        ("LefschetzFpt", it_ext_lefschetz_fpt_ty),
        ("RiemannHurwitz", it_ext_riemann_hurwitz_ty),
        ("ZetaFunction", it_ext_zeta_function_ty),
        ("WeilConjectures", it_ext_weil_conjectures_ty),
        ("KontsevichFormula", it_ext_kontsevich_formula_ty),
        ("QuantumFrobenius", it_ext_quantum_frobenius_ty),
        ("MirrorSymmetry", it_ext_mirror_symmetry_ty),
        ("TopologicalVertex", it_ext_topological_vertex_ty),
        ("GwPotentialGenus0", it_ext_gw_potential_genus0_ty),
        ("StringEquation", it_ext_string_equation_ty),
        ("PerfectObstruction", it_ext_perfect_obstruction_ty),
        ("VirtualNormalBundle", it_ext_virtual_normal_bundle_ty),
        ("VirtualPushforward", it_ext_virtual_pushforward_ty),
        ("DeformationInvariance", it_ext_deformation_invariance_ty),
        ("SchurDeterminant", it_ext_schur_determinant_ty),
        ("GrassmannBundleFormula", it_ext_grassmann_bundle_formula_ty),
        (
            "ProjectiveBundleFormula",
            it_ext_projective_bundle_formula_ty,
        ),
        ("MotivicSpectralSeq", it_ext_motivic_spectral_seq_ty),
        ("BlochOgus", it_ext_bloch_ogus_ty),
        ("GerstenResolution", it_ext_gersten_resolution_ty),
        ("MilnorKTheory", it_ext_milnor_k_theory_ty),
    ];
    for (name, mk_ty) in axioms {
        let ty = mk_ty();
        env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty,
        })
        .ok();
    }
    Ok(())
}
/// Kontsevich recursive formula for the number of rational curves in P^2.
///
/// N_d = number of rational degree-d curves through 3d-1 generic points.
/// Kontsevich's recursion: N_d = ∑_{d1+d2=d} N_{d1} N_{d2} \[d1^2 d2^2 C(3d-4, 3d1-2) - d1^3 d2 C(3d-4, 3d1-1)\]
pub fn kontsevich_nd(d: usize) -> u64 {
    if d == 1 {
        return 1;
    }
    let mut n = vec![0u64; d + 1];
    n[1] = 1;
    for dd in 2..=d {
        let mut sum = 0u64;
        for d1 in 1..dd {
            let d2 = dd - d1;
            let d1u = d1 as u64;
            let d2u = d2 as u64;
            let top = 3 * dd - 4;
            let c1 = binomial_u64(top as i64, (3 * d1 - 2) as i64);
            let c2 = if 3 * d1 >= 1 {
                binomial_u64(top as i64, (3 * d1 - 1) as i64)
            } else {
                0
            };
            let term1 = n[d1]
                .saturating_mul(n[d2])
                .saturating_mul(d1u * d1u)
                .saturating_mul(d2u * d2u)
                .saturating_mul(c1);
            let term2 = n[d1]
                .saturating_mul(n[d2])
                .saturating_mul(d1u * d1u * d1u)
                .saturating_mul(d2u)
                .saturating_mul(c2);
            sum = sum.saturating_add(term1).saturating_sub(term2.min(sum));
        }
        n[dd] = sum;
    }
    n[d]
}
pub fn binomial_u64(n: i64, k: i64) -> u64 {
    if k < 0 || k > n || n < 0 {
        return 0;
    }
    let k = k.min(n - k) as usize;
    let mut result = 1u64;
    for i in 0..k as i64 {
        result = result.saturating_mul((n - i) as u64) / (i as u64 + 1);
    }
    result
}
/// Compute the total Chern class of a direct sum using the Whitney sum formula.
///
/// Given Chern classes c(E) = \[1, c1_E, c2_E, …\] and c(F) = \[1, c1_F, c2_F, …\],
/// returns c(E ⊕ F) = c(E) · c(F) (polynomial multiplication).
pub fn whitney_sum_chern(c_e: &[i64], c_f: &[i64]) -> Vec<i64> {
    let deg = c_e.len() + c_f.len() - 1;
    let mut result = vec![0i64; deg];
    for (i, &a) in c_e.iter().enumerate() {
        for (j, &b) in c_f.iter().enumerate() {
            result[i + j] += a * b;
        }
    }
    result
}
/// Compute the Segre class s(E) as the formal inverse of c(E).
///
/// Given the total Chern class c(E) = 1 + c_1 + c_2 + …, computes
/// s(E) = c(E)^{-1} = 1 - c_1 + (c_1^2 - c_2) + … up to `max_terms` terms.
pub fn segre_class_from_chern(c: &[i64], max_terms: usize) -> Vec<i64> {
    let mut s = vec![0i64; max_terms];
    if max_terms == 0 {
        return s;
    }
    s[0] = 1;
    for k in 1..max_terms {
        let mut val = 0i64;
        for j in 1..=k {
            if j < c.len() {
                val += c[j] * s[k - j];
            }
        }
        s[k] = -val;
    }
    s
}
/// Compute the Chern character polynomial coefficients ch_0, ch_1, ch_2, ch_3.
///
/// ch(E) = ∑_k ch_k(E) using Newton's identities from Chern classes.
pub fn chern_character_from_classes(c: &[i64]) -> Vec<f64> {
    let c1 = c.get(1).copied().unwrap_or(0) as f64;
    let c2 = c.get(2).copied().unwrap_or(0) as f64;
    let c3 = c.get(3).copied().unwrap_or(0) as f64;
    let rank = c.first().copied().unwrap_or(1) as f64;
    vec![
        rank,
        c1,
        (c1 * c1 - 2.0 * c2) / 2.0,
        (c1 * c1 * c1 - 3.0 * c1 * c2 + 3.0 * c3) / 6.0,
    ]
}
pub fn it_binomial(n: i64, k: i64) -> i64 {
    if k < 0 || k > n || n < 0 {
        return 0;
    }
    let k = k.min(n - k) as usize;
    let mut result = 1i64;
    for i in 0..k as i64 {
        result = result * (n - i) / (i + 1);
    }
    result
}
