//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{
    AdamsOperationApplier, AlgebraicKGroups, BaumConnesData, BottPeriodicity, CStarKTheory,
    ChernCharacterComputer, K0Element, K1Element, KGroupComputation, MilnorKTheory, MilnorSymbol,
    ProjectiveModule, StablyFreeModuleChecker, TopologicalKTheory, VectorBundle,
    WhiteheadGroupEstimator,
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
pub fn ring_ty() -> Expr {
    cst("Ring")
}
pub fn abelian_group_ty() -> Expr {
    cst("AbelianGroup")
}
/// `K0 : Ring → AbelianGroup` — the Grothendieck group of projective modules.
///
/// K0(R) is defined as the group completion of the monoid of isomorphism classes
/// of finitely generated projective R-modules, under direct sum.
pub fn k0_group_ty() -> Expr {
    arrow(ring_ty(), abelian_group_ty())
}
/// `K1 : Ring → AbelianGroup` — the Whitehead group.
///
/// K1(R) = GL(R)^ab = lim_{n→∞} GL_n(R) / \[GL_n(R), GL_n(R)\]
/// where GL(R) is the infinite general linear group.
pub fn k1_group_ty() -> Expr {
    arrow(ring_ty(), abelian_group_ty())
}
/// `K2 : Ring → AbelianGroup` — the Milnor K2 group.
///
/// K2(R) = ker(St(R) → E(R)) where St(R) is the Steinberg group and
/// E(R) is the group of elementary matrices.
pub fn k2_group_ty() -> Expr {
    arrow(ring_ty(), abelian_group_ty())
}
/// `MilnorKn : Ring → Nat → AbelianGroup` — Milnor K-theory in degree n.
///
/// K^M_n(F) = F^× ⊗ ... ⊗ F^× / (Steinberg relations) for a field F.
pub fn milnor_kn_ty() -> Expr {
    arrow(ring_ty(), arrow(nat_ty(), abelian_group_ty()))
}
/// `HigherK : Ring → Nat → AbelianGroup` — higher algebraic K-groups K_n(R).
///
/// Defined via Quillen's Q-construction: K_n(R) = π_{n+1}(BQP(R))
/// where QP(R) is the Q-construction applied to the category of
/// finitely generated projective R-modules.
pub fn higher_k_ty() -> Expr {
    arrow(ring_ty(), arrow(nat_ty(), abelian_group_ty()))
}
/// `SteinbergGroup : Ring → Group` — the universal central extension of E(R).
///
/// St(R) is the group with generators x_{ij}(r) for i ≠ j, r ∈ R,
/// subject to Steinberg relations.
pub fn steinberg_group_ty() -> Expr {
    arrow(ring_ty(), cst("Group"))
}
/// `ElementaryMatrix : Ring → Nat → Nat → Ring → MatrixGroup` —
/// elementary matrix e_{ij}(r) = I + r·E_{ij} for i ≠ j.
pub fn elementary_matrix_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "R",
        ring_ty(),
        pi(
            BinderInfo::Default,
            "i",
            nat_ty(),
            pi(
                BinderInfo::Default,
                "j",
                nat_ty(),
                arrow(bvar(2), cst("MatrixGroup")),
            ),
        ),
    )
}
/// `BassQuillenConjecture : Ring → Prop` — every projective module over
/// a polynomial ring R\[x_1,...,x_n\] over a regular ring R is free.
pub fn bass_quillen_conjecture_ty() -> Expr {
    pi(BinderInfo::Default, "R", ring_ty(), prop())
}
/// `SuslinFreeness : Ring → Prop` — Suslin's theorem: projective modules
/// over polynomial rings over fields are free (proved 1976).
pub fn suslin_freeness_ty() -> Expr {
    pi(BinderInfo::Default, "F", cst("Field"), prop())
}
/// `KtheoryLongExactSeq : Ring → Ring → Ring → Prop` — the long exact sequence
/// in K-theory: ... → K_n(I) → K_n(R) → K_n(R/I) → K_{n-1}(I) → ...
pub fn k_theory_long_exact_seq_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "R",
        ring_ty(),
        pi(
            BinderInfo::Default,
            "I",
            cst("Ideal"),
            arrow(app(cst("QuotientRing"), bvar(0)), prop()),
        ),
    )
}
/// `BottPeriodicity : Ring → Prop` — Bott periodicity in the algebraic setting.
///
/// For C*-algebras: K_n(A) ≅ K_{n+2}(A). In the algebraic setting this is
/// more subtle and requires the Bass fundamental theorem.
pub fn bott_periodicity_ty() -> Expr {
    pi(BinderInfo::Default, "A", cst("CStarAlgebra"), prop())
}
/// `BassFundamental : Ring → Prop` — Bass's fundamental theorem:
/// K_{-1}(R\[x, x^{-1}\]) ≅ K_0(R) ⊕ K_{-1}(R) (for negative K-groups).
pub fn bass_fundamental_ty() -> Expr {
    pi(BinderInfo::Default, "R", ring_ty(), prop())
}
/// `KtheoryProduct : Ring → Ring → Nat → Nat → AbelianGroup` —
/// the product map K_m(R) ⊗ K_n(R) → K_{m+n}(R).
pub fn k_theory_product_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "R",
        ring_ty(),
        pi(
            BinderInfo::Default,
            "m",
            nat_ty(),
            pi(
                BinderInfo::Default,
                "n",
                nat_ty(),
                arrow(
                    app2(cst("KGroup"), bvar(2), bvar(1)),
                    arrow(
                        app2(cst("KGroup"), bvar(3), bvar(1)),
                        app2(
                            cst("KGroup"),
                            bvar(4),
                            app2(cst("NatAdd"), bvar(2), bvar(1)),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// `MilnorConjecture : Field → Prop` — Milnor's conjecture (proved by Voevodsky):
/// K^M_n(F)/2 ≅ H^n(F, Z/2) via the norm residue map.
pub fn milnor_conjecture_ty() -> Expr {
    pi(BinderInfo::Default, "F", cst("Field"), prop())
}
/// `BlochKato : Field → Prop` — the Bloch-Kato conjecture (Voevodsky's theorem):
/// K^M_n(F)/l ≅ H^n(F, Z/l) for all primes l.
pub fn bloch_kato_ty() -> Expr {
    pi(BinderInfo::Default, "F", cst("Field"), prop())
}
/// Populate an `Environment` with algebraic K-theory axioms.
pub fn build_k_theory_env() -> Environment {
    let mut env = Environment::new();
    let base_types: &[(&str, Expr)] = &[
        ("Ring", type1()),
        ("Field", type1()),
        ("AbelianGroup", type1()),
        ("Group", type1()),
        ("Ideal", type1()),
        ("MatrixGroup", type1()),
        ("CStarAlgebra", type1()),
        ("ProjectiveModule", type1()),
        ("FreeModule", type1()),
        ("StableEquiv", type1()),
        ("GrothendieckCompletion", type1()),
    ];
    for (name, ty) in base_types {
        env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty: ty.clone(),
        })
        .ok();
    }
    let helper_fns: &[(&str, Expr)] = &[
        (
            "QuotientRing",
            arrow(ring_ty(), arrow(cst("Ideal"), ring_ty())),
        ),
        (
            "KGroup",
            arrow(ring_ty(), arrow(nat_ty(), abelian_group_ty())),
        ),
        ("NatAdd", arrow(nat_ty(), arrow(nat_ty(), nat_ty()))),
        (
            "MilnorK",
            arrow(cst("Field"), arrow(nat_ty(), abelian_group_ty())),
        ),
        ("InfiniteGL", arrow(ring_ty(), cst("Group"))),
        ("PolyRing", arrow(ring_ty(), arrow(nat_ty(), ring_ty()))),
        ("LaurentPoly", arrow(ring_ty(), ring_ty())),
    ];
    for (name, ty) in helper_fns {
        env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty: ty.clone(),
        })
        .ok();
    }
    let k_type_axioms: &[(&str, fn() -> Expr)] = &[
        ("K0", k0_group_ty),
        ("K1", k1_group_ty),
        ("K2", k2_group_ty),
        ("MilnorKn", milnor_kn_ty),
        ("HigherKGroup", higher_k_ty),
        ("SteinbergGroup", steinberg_group_ty),
        ("ElementaryMatrix", elementary_matrix_ty),
        ("KtheoryProduct", k_theory_product_ty),
    ];
    for (name, mk_ty) in k_type_axioms {
        env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty: mk_ty(),
        })
        .ok();
    }
    let theorem_axioms: &[(&str, fn() -> Expr)] = &[
        ("bass_quillen_conjecture", bass_quillen_conjecture_ty),
        ("suslin_freeness", suslin_freeness_ty),
        ("k_theory_long_exact_seq", k_theory_long_exact_seq_ty),
        ("bott_periodicity_algebraic", bott_periodicity_ty),
        ("bass_fundamental_theorem", bass_fundamental_ty),
        ("milnor_conjecture", milnor_conjecture_ty),
        ("bloch_kato_theorem", bloch_kato_ty),
    ];
    for (name, mk_ty) in theorem_axioms {
        env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty: mk_ty(),
        })
        .ok();
    }
    env
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_k0_element_virtual_rank() {
        let p = K0Element::from_projective(3, "P");
        let q = K0Element::from_projective(2, "Q");
        let diff = p.add(&q.negate());
        assert_eq!(diff.virtual_rank(), 1);
        assert!(!diff.is_zero());
    }
    #[test]
    fn test_k0_element_zero() {
        let p = K0Element::from_projective(2, "P");
        let neg_p = p.negate();
        let sum = p.add(&neg_p);
        assert!(sum.is_zero());
        assert_eq!(sum.virtual_rank(), 0);
    }
    #[test]
    fn test_k1_identity_is_not_elementary() {
        let id = K1Element::identity(2);
        assert!(!id.is_elementary());
    }
    #[test]
    fn test_k1_elementary_matrix() {
        let mut elem = K1Element::identity(2);
        elem.entries[1] = 3;
        elem.label = "e_12(3)".to_string();
        assert!(elem.is_elementary());
    }
    #[test]
    fn test_k1_determinant_2x2() {
        let mat = K1Element::diagonal(&[2, 3], "diag");
        assert_eq!(mat.determinant_2x2(), Some(6));
    }
    #[test]
    fn test_milnor_symbol_steinberg() {
        let sym = MilnorSymbol::new(vec![2, -1]);
        assert!(!sym.satisfies_steinberg());
    }
    #[test]
    fn test_milnor_symbol_display() {
        let sym = MilnorSymbol::new(vec![3, 5, 7]);
        let s = sym.to_string();
        assert!(s.contains("3"));
        assert!(s.contains("5"));
        assert!(s.contains("7"));
    }
    #[test]
    fn test_projective_module_direct_sum() {
        let p = ProjectiveModule::free(2, "Z");
        let q = ProjectiveModule::free(3, "Z");
        let pq = p.direct_sum(&q);
        assert_eq!(pq.rank, 5);
        assert!(pq.is_free);
    }
    #[test]
    fn test_build_k_theory_env() {
        let env = build_k_theory_env();
        assert!(env.get(&Name::str("K0")).is_some());
        assert!(env.get(&Name::str("K1")).is_some());
        assert!(env.get(&Name::str("K2")).is_some());
        assert!(env.get(&Name::str("MilnorKn")).is_some());
        assert!(env.get(&Name::str("HigherKGroup")).is_some());
        assert!(env.get(&Name::str("bass_quillen_conjecture")).is_some());
        assert!(env.get(&Name::str("bloch_kato_theorem")).is_some());
    }
}
/// `TopologicalK0 : Space → AbelianGroup` — K⁰(X) = Grothendieck group of complex
/// vector bundles over compact Hausdorff space X.
pub fn topological_k0_ty() -> Expr {
    arrow(cst("Space"), abelian_group_ty())
}
/// `TopologicalK1 : Space → AbelianGroup` — K⁻¹(X) = K⁰(ΣX) where ΣX is the
/// (reduced) suspension of X.
pub fn topological_k1_ty() -> Expr {
    arrow(cst("Space"), abelian_group_ty())
}
/// `KOGroup : Space → Nat → AbelianGroup` — KO^n(X), real K-theory groups.
/// These have period 8 (Bott periodicity for real K-theory).
pub fn ko_group_ty() -> Expr {
    arrow(cst("Space"), arrow(nat_ty(), abelian_group_ty()))
}
/// `KSpGroup : Space → Nat → AbelianGroup` — KSp^n(X), symplectic K-theory groups.
/// KSp(X) classifies quaternionic vector bundles over X.
pub fn ksp_group_ty() -> Expr {
    arrow(cst("Space"), arrow(nat_ty(), abelian_group_ty()))
}
/// `AtiyahRealKTheory : RealSpace → AbelianGroup` — Atiyah's Real K-theory KR(X)
/// for spaces X with involution. Interpolates between K and KO.
pub fn atiyah_real_k_theory_ty() -> Expr {
    arrow(cst("RealSpace"), abelian_group_ty())
}
/// `AdamsOperation : Nat → Ring → Ring` — Adams operation ψ^k on K-theory rings.
/// ψ^k is the unique ring endomorphism with ψ^k(L) = L^⊗k for line bundles L.
pub fn adams_operation_ty() -> Expr {
    arrow(nat_ty(), arrow(cst("KRing"), cst("KRing")))
}
/// `EquivariantK0 : Group → Space → AbelianGroup` — G-equivariant K-theory K_G(X).
/// K_G(X) classifies G-equivariant complex vector bundles over X.
pub fn equivariant_k0_ty() -> Expr {
    arrow(cst("Group"), arrow(cst("Space"), abelian_group_ty()))
}
/// `RepresentationRing : Group → Ring` — the representation ring R(G) = K_G(pt).
/// R(G) is the Grothendieck group of finite-dimensional complex G-representations.
pub fn representation_ring_ty() -> Expr {
    arrow(cst("Group"), ring_ty())
}
/// `AtiyahCompletionTheorem : Group → Space → Prop` — Atiyah's completion theorem:
/// the natural map K_G(X) → K(EG ×_G X) is an isomorphism after completing
/// at the augmentation ideal of R(G).
pub fn atiyah_completion_theorem_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "G",
        cst("Group"),
        pi(BinderInfo::Default, "X", cst("Space"), prop()),
    )
}
/// `KCStarAlgebra : CStarAlgebra → Nat → AbelianGroup` — K-theory of C*-algebras.
/// K_0(A) = projective modules; K_1(A) = GL(A)/GL_0(A).
pub fn k_cstar_algebra_ty() -> Expr {
    arrow(cst("CStarAlgebra"), arrow(nat_ty(), abelian_group_ty()))
}
/// `PimsnerVoiculescu : CStarAlgebra → CStarAlgebra → Prop` — the Pimsner-Voiculescu
/// six-term exact sequence in K-theory for crossed products A ⋊_α ℤ.
pub fn pimsner_voiculescu_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "A",
        cst("CStarAlgebra"),
        arrow(cst("CStarAlgebra"), prop()),
    )
}
/// `ConnesThomIsomorphism : CStarAlgebra → Prop` — Connes-Thom isomorphism:
/// K_*(A ⋊_α ℝ) ≅ K_{*+1}(A) for any C*-dynamical system (A, ℝ, α).
pub fn connes_thom_isomorphism_ty() -> Expr {
    pi(BinderInfo::Default, "A", cst("CStarAlgebra"), prop())
}
/// `GTheory : Scheme → Nat → AbelianGroup` — G-theory (K-theory for singular varieties).
/// G_n(X) = K_n of the abelian category of coherent sheaves on X.
pub fn g_theory_ty() -> Expr {
    arrow(cst("Scheme"), arrow(nat_ty(), abelian_group_ty()))
}
/// `QuillenQConstruction : ExactCategory → TopSpace` — Quillen's Q-construction
/// turns an exact category C into a topological space QC. K_n(C) = π_{n+1}(BQC).
pub fn quillen_q_construction_ty() -> Expr {
    arrow(cst("ExactCategory"), cst("TopSpace"))
}
/// `QuillenPlusConstruction : TopSpace → TopSpace` — Quillen's plus-construction
/// X^+ kills the maximal perfect normal subgroup of π_1(X) without changing H_*.
pub fn quillen_plus_construction_ty() -> Expr {
    arrow(cst("TopSpace"), cst("TopSpace"))
}
/// `WaldhausenKTheory : WaldhausenCategory → Spectrum` — K-theory via
/// Waldhausen's S-dot construction. K(C) = |wS_•C|.
pub fn waldhausen_k_theory_ty() -> Expr {
    arrow(cst("WaldhausenCategory"), cst("Spectrum"))
}
/// `MotivicKTheory : Scheme → Nat → Nat → AbelianGroup` — motivic cohomology groups
/// H^p(X, Z(q)) and their relation to algebraic K-theory via the Atiyah-Hirzebruch ss.
pub fn motivic_k_theory_ty() -> Expr {
    arrow(
        cst("Scheme"),
        arrow(nat_ty(), arrow(nat_ty(), abelian_group_ty())),
    )
}
/// `BassQuillenPolynomial : Ring → Prop` — the Bass-Quillen conjecture for
/// polynomial rings: every projective R\[x_1,...,x_n\]-module is extended from R.
pub fn bass_quillen_polynomial_ty() -> Expr {
    pi(BinderInfo::Default, "R", ring_ty(), prop())
}
/// `GerstenConjecture : Ring → Prop` — Gersten's conjecture: the natural map
/// K_n(R) → K_n(Frac(R)) is injective for regular local rings R.
pub fn gersten_conjecture_ty() -> Expr {
    pi(BinderInfo::Default, "R", cst("RegularLocalRing"), prop())
}
/// `LocalisationSequence : Ring → Ring → Ring → Prop` — the K-theory localisation
/// sequence: … → K_n(R_S) → K_n(R) → K_n(R/I) → … for a multiplicative set S.
pub fn localisation_sequence_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "R",
        ring_ty(),
        pi(
            BinderInfo::Default,
            "S",
            cst("MultiplicativeSet"),
            arrow(cst("LocalisedRing"), prop()),
        ),
    )
}
/// `DevissageTheorem : ExactCategory → Prop` — Quillen's dévissage theorem:
/// if every object has a filtration with subquotients in a full subcategory C₀,
/// then K(C₀) → K(C) is an equivalence.
pub fn devissage_theorem_ty() -> Expr {
    pi(BinderInfo::Default, "C", cst("ExactCategory"), prop())
}
/// `ResolutionTheorem : ExactCategory → Prop` — Quillen's resolution theorem:
/// if every object in C has a finite resolution by objects in a full subcategory P,
/// then K(P) → K(C) is an equivalence.
pub fn resolution_theorem_ty() -> Expr {
    pi(BinderInfo::Default, "C", cst("ExactCategory"), prop())
}
/// `WhiteheadGroup : Group → AbelianGroup` — the Whitehead group Wh(G) = K_1(Z\[G\]) / {±G}.
/// Central in surgery theory and s-cobordism theorem.
pub fn whitehead_group_ty() -> Expr {
    arrow(cst("Group"), abelian_group_ty())
}
/// `ReducedKOne : Ring → AbelianGroup` — reduced K_1: K̃_1(R) = K_1(R) / (units).
/// Strips out the contribution of the group of units from K_1.
pub fn reduced_k_one_ty() -> Expr {
    arrow(ring_ty(), abelian_group_ty())
}
/// `MatrixStabilisation : Ring → Nat → Nat → Prop` — matrix stabilisation theorem:
/// GL_n(R) → GL_{n+1}(R) induces an isomorphism on K_1 for n ≥ sr(R) + 1
/// (sr = stable rank of R).
pub fn matrix_stabilisation_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "R",
        ring_ty(),
        pi(BinderInfo::Default, "n", nat_ty(), arrow(nat_ty(), prop())),
    )
}
/// `LambdaRingStructure : Ring → Prop` — the λ-ring structure on K_0(R):
/// λ^k\[E\] = \[∧^k E\] (k-th exterior power). Satisfies the λ-ring axioms.
pub fn lambda_ring_structure_ty() -> Expr {
    pi(BinderInfo::Default, "R", ring_ty(), prop())
}
/// `GammaFiltration : Ring → Nat → AbelianGroup` — the γ-filtration on K_0(R):
/// F^n K_0(R) generated by γ^{i_1}(x_1)·…·γ^{i_k}(x_k) with Σi_j ≥ n.
pub fn gamma_filtration_ty() -> Expr {
    arrow(ring_ty(), arrow(nat_ty(), abelian_group_ty()))
}
/// `ChernCharacter : Ring → Ring` — the Chern character ch: K_0(X) → H^{2*}(X; Q).
/// A ring homomorphism and isomorphism after tensoring with Q.
pub fn chern_character_ty() -> Expr {
    arrow(cst("KRing"), cst("CohomologyRing"))
}
/// `GrothendieckRiemannRoch : Scheme → Scheme → Prop` — GRR theorem:
/// for proper f: X → Y, ch(f_! α) · td(Y) = f_*(ch(α) · td(X)) in H^*(Y; Q).
pub fn grothendieck_riemann_roch_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        cst("Scheme"),
        arrow(cst("Scheme"), prop()),
    )
}
/// `AtiyahSingerIndex : EllipticOperator → Int` — the Atiyah-Singer index theorem:
/// ind(D) = ∫_X ch(σ(D)) · td(TX) where σ(D) is the principal symbol.
pub fn atiyah_singer_index_ty() -> Expr {
    arrow(cst("EllipticOperator"), int_ty())
}
/// `FamiliesIndexTheorem : FamilyOfOperators → KGroup` — the families index theorem
/// (Atiyah-Singer): for a family of elliptic operators over base B, ind(D) ∈ K(B).
pub fn families_index_theorem_ty() -> Expr {
    arrow(cst("FamilyOfOperators"), abelian_group_ty())
}
/// `GSignatureTheorem : Manifold → Group → Int` — the G-signature theorem:
/// for a compact oriented manifold M with G-action, sign_G(M) = L-genus integral.
pub fn g_signature_theorem_ty() -> Expr {
    arrow(cst("Manifold"), arrow(cst("Group"), int_ty()))
}
/// `KSphereComputation : Nat → AbelianGroup` — K-theory of spheres:
/// K̃⁰(S^{2n}) ≅ ℤ, K̃⁰(S^{2n+1}) ≅ 0 (complex K-theory).
pub fn k_sphere_computation_ty() -> Expr {
    arrow(nat_ty(), abelian_group_ty())
}
/// `KOSpherePattern : Nat → AbelianGroup` — real K-theory of spheres KO(S^n).
/// Follows an 8-periodic pattern: ℤ, ℤ/2, ℤ/2, 0, ℤ, 0, 0, 0 (for n=0..7).
pub fn ko_sphere_pattern_ty() -> Expr {
    arrow(nat_ty(), abelian_group_ty())
}
/// `VirtualBundle : Space → Int → KClass` — a virtual bundle \[E\] - \[F\] in K(X)
/// represented as a formal difference of isomorphism classes.
pub fn virtual_bundle_ty() -> Expr {
    arrow(cst("Space"), arrow(int_ty(), cst("KClass")))
}
/// `ThomIsomorphism : VectorBundle → Prop` — Thom isomorphism in K-theory:
/// K(X) ≅ K̃(Th(E)) where Th(E) is the Thom space of a complex vector bundle E.
pub fn thom_isomorphism_ty() -> Expr {
    pi(BinderInfo::Default, "E", cst("VectorBundle"), prop())
}
/// `SuspensionIsomorphism : Space → Prop` — K̃^n(X) ≅ K̃^{n+1}(ΣX) where ΣX
/// is the reduced suspension. Follows from Bott periodicity.
pub fn suspension_isomorphism_ty() -> Expr {
    pi(BinderInfo::Default, "X", cst("Space"), prop())
}
/// `NegativeKGroups : Ring → Int → AbelianGroup` — Bass's negative K-groups K_n(R)
/// for n < 0, defined via the Bass fundamental theorem and Karoubi-Villamayor.
pub fn negative_k_groups_ty() -> Expr {
    arrow(ring_ty(), arrow(int_ty(), abelian_group_ty()))
}
/// `HomotopyInvariance : Ring → Prop` — K_n(R) ≅ K_n(R\[t\]) for all n ≥ 0
/// when R is a regular ring (homotopy invariance of K-theory).
pub fn homotopy_invariance_ty() -> Expr {
    pi(BinderInfo::Default, "R", cst("RegularRing"), prop())
}
/// Populate an `Environment` with the extended set of K-theory axioms (all original
/// plus the 30+ new builders from Section 4).
pub fn build_k_theory_env_extended() -> Environment {
    let mut env = build_k_theory_env();
    let extended_types: &[(&str, Expr)] = &[
        ("Space", type1()),
        ("RealSpace", type1()),
        ("KRing", type1()),
        ("CohomologyRing", type1()),
        ("Scheme", type1()),
        ("ExactCategory", type1()),
        ("WaldhausenCategory", type1()),
        ("Spectrum", type1()),
        ("TopSpace", type1()),
        ("RegularLocalRing", type1()),
        ("RegularRing", type1()),
        ("MultiplicativeSet", type1()),
        ("LocalisedRing", type1()),
        ("VectorBundle", type1()),
        ("KClass", type1()),
        ("Manifold", type1()),
        ("EllipticOperator", type1()),
        ("FamilyOfOperators", type1()),
    ];
    for (name, ty) in extended_types {
        env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty: ty.clone(),
        })
        .ok();
    }
    let extended_type_axioms: &[(&str, fn() -> Expr)] = &[
        ("TopologicalK0", topological_k0_ty),
        ("TopologicalK1", topological_k1_ty),
        ("KOGroup", ko_group_ty),
        ("KSpGroup", ksp_group_ty),
        ("AtiyahRealKTheory", atiyah_real_k_theory_ty),
        ("AdamsOperation", adams_operation_ty),
        ("EquivariantK0", equivariant_k0_ty),
        ("RepresentationRing", representation_ring_ty),
        ("KCStarAlgebra", k_cstar_algebra_ty),
        ("GTheory", g_theory_ty),
        ("QuillenQConstruction", quillen_q_construction_ty),
        ("QuillenPlusConstruction", quillen_plus_construction_ty),
        ("WaldhausenKTheory", waldhausen_k_theory_ty),
        ("MotivicKTheory", motivic_k_theory_ty),
        ("GammaFiltration", gamma_filtration_ty),
        ("ChernCharacter", chern_character_ty),
        ("WhiteheadGroup", whitehead_group_ty),
        ("ReducedKOne", reduced_k_one_ty),
        ("NegativeKGroups", negative_k_groups_ty),
        ("KSphereComputation", k_sphere_computation_ty),
        ("KOSpherePattern", ko_sphere_pattern_ty),
        ("VirtualBundle", virtual_bundle_ty),
        ("AtiyahSingerIndex", atiyah_singer_index_ty),
        ("FamiliesIndexTheorem", families_index_theorem_ty),
        ("GSignatureTheorem", g_signature_theorem_ty),
    ];
    for (name, mk_ty) in extended_type_axioms {
        env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty: mk_ty(),
        })
        .ok();
    }
    let extended_theorem_axioms: &[(&str, fn() -> Expr)] = &[
        ("atiyah_completion_theorem", atiyah_completion_theorem_ty),
        ("pimsner_voiculescu", pimsner_voiculescu_ty),
        ("connes_thom_isomorphism", connes_thom_isomorphism_ty),
        ("bass_quillen_polynomial", bass_quillen_polynomial_ty),
        ("gersten_conjecture", gersten_conjecture_ty),
        ("localisation_sequence", localisation_sequence_ty),
        ("devissage_theorem", devissage_theorem_ty),
        ("resolution_theorem", resolution_theorem_ty),
        ("matrix_stabilisation", matrix_stabilisation_ty),
        ("lambda_ring_structure", lambda_ring_structure_ty),
        ("grothendieck_riemann_roch", grothendieck_riemann_roch_ty),
        ("thom_isomorphism", thom_isomorphism_ty),
        ("suspension_isomorphism", suspension_isomorphism_ty),
        ("homotopy_invariance", homotopy_invariance_ty),
    ];
    for (name, mk_ty) in extended_theorem_axioms {
        env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty: mk_ty(),
        })
        .ok();
    }
    env
}
#[cfg(test)]
mod extended_tests {
    use super::*;
    #[test]
    fn test_topological_k0_ty() {
        let ty = topological_k0_ty();
        matches!(ty, Expr::Pi(..));
    }
    #[test]
    fn test_ko_group_ty() {
        let ty = ko_group_ty();
        matches!(ty, Expr::Pi(..));
    }
    #[test]
    fn test_adams_operation_ty() {
        let ty = adams_operation_ty();
        matches!(ty, Expr::Pi(..));
    }
    #[test]
    fn test_chern_character_ty() {
        let ty = chern_character_ty();
        matches!(ty, Expr::Pi(..));
    }
    #[test]
    fn test_whitehead_group_ty() {
        let ty = whitehead_group_ty();
        matches!(ty, Expr::Pi(..));
    }
    #[test]
    fn test_atiyah_singer_index_ty() {
        let ty = atiyah_singer_index_ty();
        matches!(ty, Expr::Pi(..));
    }
    #[test]
    fn test_build_k_theory_env_extended() {
        let env = build_k_theory_env_extended();
        assert!(env.get(&Name::str("TopologicalK0")).is_some());
        assert!(env.get(&Name::str("TopologicalK1")).is_some());
        assert!(env.get(&Name::str("KOGroup")).is_some());
        assert!(env.get(&Name::str("KSpGroup")).is_some());
        assert!(env.get(&Name::str("AtiyahRealKTheory")).is_some());
        assert!(env.get(&Name::str("AdamsOperation")).is_some());
        assert!(env.get(&Name::str("EquivariantK0")).is_some());
        assert!(env.get(&Name::str("RepresentationRing")).is_some());
        assert!(env.get(&Name::str("GTheory")).is_some());
        assert!(env.get(&Name::str("QuillenQConstruction")).is_some());
        assert!(env.get(&Name::str("QuillenPlusConstruction")).is_some());
        assert!(env.get(&Name::str("WaldhausenKTheory")).is_some());
        assert!(env.get(&Name::str("MotivicKTheory")).is_some());
        assert!(env.get(&Name::str("ChernCharacter")).is_some());
        assert!(env.get(&Name::str("WhiteheadGroup")).is_some());
        assert!(env.get(&Name::str("NegativeKGroups")).is_some());
        assert!(env.get(&Name::str("AtiyahSingerIndex")).is_some());
        assert!(env.get(&Name::str("FamiliesIndexTheorem")).is_some());
        assert!(env.get(&Name::str("GSignatureTheorem")).is_some());
        assert!(env.get(&Name::str("atiyah_completion_theorem")).is_some());
        assert!(env.get(&Name::str("pimsner_voiculescu")).is_some());
        assert!(env.get(&Name::str("connes_thom_isomorphism")).is_some());
        assert!(env.get(&Name::str("gersten_conjecture")).is_some());
        assert!(env.get(&Name::str("devissage_theorem")).is_some());
        assert!(env.get(&Name::str("resolution_theorem")).is_some());
        assert!(env.get(&Name::str("lambda_ring_structure")).is_some());
        assert!(env.get(&Name::str("grothendieck_riemann_roch")).is_some());
        assert!(env.get(&Name::str("thom_isomorphism")).is_some());
        assert!(env.get(&Name::str("homotopy_invariance")).is_some());
        assert!(env.get(&Name::str("K0")).is_some());
        assert!(env.get(&Name::str("bloch_kato_theorem")).is_some());
    }
    #[test]
    fn test_stably_free_module_checker_is_stably_free() {
        let chk = StablyFreeModuleChecker::new(3, 2, 5, "Z");
        assert!(chk.is_stably_free());
        assert!(!chk.is_free());
        assert_eq!(chk.k0_class(), 1);
    }
    #[test]
    fn test_stably_free_module_checker_is_free() {
        let chk = StablyFreeModuleChecker::new(3, 0, 3, "Z");
        assert!(chk.is_stably_free());
        assert!(chk.is_free());
        assert_eq!(chk.k0_class(), 3);
    }
    #[test]
    fn test_stably_free_module_checker_not_stably_free() {
        let chk = StablyFreeModuleChecker::new(3, 2, 7, "Z");
        assert!(!chk.is_stably_free());
    }
    #[test]
    fn test_k_group_computation_z6() {
        let kg = KGroupComputation::new(6);
        assert_eq!(kg.k0_rank(), 2);
        let desc = kg.describe_k0();
        assert!(desc.contains("Z/6Z") || desc.contains("6"));
    }
    #[test]
    fn test_k_group_computation_prime() {
        let kg = KGroupComputation::new(7);
        assert_eq!(kg.k0_rank(), 1);
    }
    #[test]
    fn test_k_group_computation_prime_power() {
        let kg = KGroupComputation::new(8);
        assert_eq!(kg.k0_rank(), 1);
    }
    #[test]
    fn test_adams_operation_line_bundle() {
        let op = AdamsOperationApplier::new(4, vec![3]);
        assert_eq!(op.apply_to_line_bundle_c1(), Some(12));
    }
    #[test]
    fn test_adams_operation_composition_degree() {
        assert_eq!(AdamsOperationApplier::composition_degree(3, 5), 15);
    }
    #[test]
    fn test_adams_operation_rank_preserved() {
        let op = AdamsOperationApplier::new(3, vec![1, 2]);
        assert_eq!(op.apply_to_rank(4), 4);
    }
    #[test]
    fn test_chern_character_computer_rank1() {
        let cc = ChernCharacterComputer::new(1, vec![5]);
        assert_eq!(cc.ch0(), 1);
        assert_eq!(cc.ch1(), 5);
        assert_eq!(cc.ch2_numerator(), 25);
    }
    #[test]
    fn test_chern_character_computer_additive() {
        let e = ChernCharacterComputer::new(2, vec![3, 1]);
        let f = ChernCharacterComputer::new(1, vec![2, 0]);
        let sum = e.additive_with(&f);
        assert_eq!(sum.rank, 3);
        assert_eq!(sum.chern_classes[0], 5);
        assert_eq!(sum.chern_classes[1], 1);
    }
    #[test]
    fn test_chern_character_terms_length() {
        let cc = ChernCharacterComputer::new(3, vec![1, 2, 3]);
        let terms = cc.chern_character_terms();
        assert_eq!(terms.len(), 4);
        assert_eq!(terms[0], 3);
    }
    #[test]
    fn test_whitehead_group_cyclic_trivial() {
        let wh = WhiteheadGroupEstimator::new("Z/7", 7, 7, 7, true);
        assert!(wh.is_trivial());
        let desc = wh.describe();
        assert!(desc.contains("0") || desc.contains("trivial") || desc.contains("Bass"));
    }
    #[test]
    fn test_whitehead_group_non_cyclic() {
        let wh = WhiteheadGroupEstimator::new("S3", 6, 3, 5, false);
        assert!(!wh.is_trivial());
        assert_eq!(wh.estimated_rank(), 2);
        assert_eq!(wh.torsion_exponent(), 6);
    }
    #[test]
    fn test_whitehead_group_display() {
        let wh = WhiteheadGroupEstimator::new("Z/5", 5, 5, 5, true);
        let s = wh.to_string();
        assert!(!s.is_empty());
    }
}
#[cfg(test)]
mod tests_k_theory_ext {
    use super::*;
    #[test]
    fn test_algebraic_k_groups() {
        let kz = AlgebraicKGroups::integers();
        assert_eq!(kz.ring, "Z");
        assert_eq!(kz.k0, "Z");
        assert_eq!(kz.k1, "Z/2");
        assert!(kz.k2.is_some());
        assert!(kz.bass_conjecture_expected());
    }
    #[test]
    fn test_topological_k_theory() {
        let ks2 = TopologicalKTheory::sphere(2);
        assert_eq!(ks2.ku0_rank, 2);
        assert_eq!(ks2.ku1_rank, 0);
        assert!(ks2.bott_periodicity_holds());
        assert!(ks2.chern_character().contains("KU"));
        let ks3 = TopologicalKTheory::sphere(3);
        assert_eq!(ks3.ku0_rank, 1);
        assert_eq!(ks3.ku1_rank, 1);
    }
    #[test]
    fn test_cstar_k_theory() {
        let cx = CStarKTheory::continuous_functions("X", "K_0(X, Z)", "K_1(X, Z)");
        assert!(cx.is_nuclear);
        assert!(cx.uct_applies);
        assert!(cx.kunneth_formula_holds());
        assert!(cx.six_term_exact_sequence().contains("ideal"));
    }
    #[test]
    fn test_field_k_theory() {
        let kf = AlgebraicKGroups::field("k");
        assert_eq!(kf.k0, "Z");
        assert!(kf.k1.contains("k^×") || kf.k1.contains("k^") || kf.k1.contains("×"));
    }
}
#[cfg(test)]
mod tests_k_theory_ext2 {
    use super::*;
    #[test]
    fn test_milnor_k_theory() {
        let mk = MilnorKTheory::new("Q");
        assert_eq!(mk.groups[0], (0, "Z".to_string()));
        assert!(mk.groups[1].1.contains('×') || mk.groups[1].1.contains('^'));
        assert!(mk.vanishes_for_finite_field(2));
        assert!(!mk.vanishes_for_finite_field(1));
        assert!(mk.bloch_kato_theorem().contains("Voevodsky"));
    }
    #[test]
    fn test_k_theory_ahss() {
        let kt = TopologicalKTheory::new("BG", 3, 1);
        let ahss = kt.ahss_description();
        assert!(ahss.contains("AHSS"));
        assert!(ahss.contains("BG"));
    }
}
#[cfg(test)]
mod tests_k_theory_ext3 {
    use super::*;
    #[test]
    fn test_baum_connes() {
        let bc = BaumConnesData::new("Z^n", true).verified();
        assert!(bc.bc_verified);
        assert!(bc.novikov_implication().contains("Novikov"));
        assert!(bc.haagerup_property());
        assert!(bc.assembly_map.contains("Z^n"));
    }
}
