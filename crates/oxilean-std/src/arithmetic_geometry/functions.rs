//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{
    AbelianVariety, AbsoluteHeight, AutomorphicRepresentation, BerkovichSpace, CanonicalModel,
    ChowGroup, DualAbelianVariety, EllipticCurve, FaltingsThm, GaloisRepresentation,
    HeightFunction, Isogeny, LanglandsCorrespondence, LogarithmicHeight,
    NearlyOrdinaryRepresentation, NeronModel, NorthcottProperty, PolarizedAbelianVariety,
    ShimuraDatum, ShimuraVariety, TateModule, TolimaniConjecture, TorsionPoint,
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
/// `AbelianVariety : Field → Nat → Type`
/// An abelian variety over a field k of dimension g is a smooth projective group variety.
pub fn abelian_variety_ty() -> Expr {
    arrow(cst("Field"), arrow(nat_ty(), type0()))
}
/// `PolarizedAbelianVariety : Field → Nat → Type`
/// An abelian variety together with an ample line bundle (polarization).
pub fn polarized_abelian_variety_ty() -> Expr {
    arrow(cst("Field"), arrow(nat_ty(), type0()))
}
/// `TateModule : AbelianVarietyObj → Prime → ZpModule`
/// T_p(A) = lim_{n} A[p^n], the p-adic Tate module of A, free of rank 2g over ℤ_p.
pub fn tate_module_ty() -> Expr {
    arrow(
        cst("AbelianVarietyObj"),
        arrow(cst("Prime"), cst("ZpModule")),
    )
}
/// `DualAbelianVariety : AbelianVarietyObj → AbelianVarietyObj`
/// A^ = Pic^0(A), the dual abelian variety (variety of degree-0 line bundles).
pub fn dual_abelian_variety_ty() -> Expr {
    arrow(cst("AbelianVarietyObj"), cst("AbelianVarietyObj"))
}
/// Poincaré reducibility: every abelian variety is isogenous to a product of simple abelian varieties.
///
/// `∀ (A : AbelianVarietyObj), ∃ (simples : List AbelianVarietyObj), IsogenousToProduct A simples`
pub fn poincare_reducibility_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "A",
        cst("AbelianVarietyObj"),
        app2(
            cst("Exists"),
            list_ty(cst("AbelianVarietyObj")),
            app2(cst("IsogenousToProduct"), bvar(1), bvar(0)),
        ),
    )
}
/// Tate module rank theorem.
///
/// `∀ (A : AbelianVarietyObj) (p : Prime), Rank (TateModule A p) = 2 * Dimension A`
pub fn tate_module_rank_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "A",
        cst("AbelianVarietyObj"),
        pi(
            BinderInfo::Default,
            "p",
            cst("Prime"),
            app2(
                cst("NatEq"),
                app(
                    cst("ZpModuleRank"),
                    app2(cst("TateModule"), bvar(1), bvar(0)),
                ),
                app(cst("NatMul2"), app(cst("AbelianVarietyDim"), bvar(1))),
            ),
        ),
    )
}
/// Isogeny theorem: for abelian varieties over finite fields, Hom(A, B) ⊗ ℤ_ℓ ≅ Hom(T_ℓ A, T_ℓ B).
///
/// `∀ (A B : AbelianVarietyObj) (ell : Prime), IsFiniteField k →
///    Iso (HomTensor A B ell) (TateModuleHom A B ell)`
pub fn isogeny_theorem_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "A",
        cst("AbelianVarietyObj"),
        pi(
            BinderInfo::Default,
            "B",
            cst("AbelianVarietyObj"),
            pi(
                BinderInfo::Default,
                "ell",
                cst("Prime"),
                arrow(
                    cst("IsFiniteField"),
                    app2(
                        cst("Iso"),
                        app3(cst("HomTensor"), bvar(2), bvar(1), bvar(0)),
                        app3(cst("TateModuleHom"), bvar(2), bvar(1), bvar(0)),
                    ),
                ),
            ),
        ),
    )
}
/// `EllipticCurve : Field → Type`
/// An elliptic curve E/k given by a Weierstrass equation y² = x³ + ax + b.
pub fn elliptic_curve_ty() -> Expr {
    arrow(cst("Field"), type0())
}
/// `Isogeny : EllipticCurveObj → EllipticCurveObj → Type`
/// A group homomorphism φ: E → E' with finite kernel.
pub fn isogeny_ty() -> Expr {
    arrow(
        cst("EllipticCurveObj"),
        arrow(cst("EllipticCurveObj"), type0()),
    )
}
/// `TorsionPoint : EllipticCurveObj → Nat → Type`
/// E\[n\] = {P ∈ E : nP = O}, the n-torsion subgroup.
pub fn torsion_point_ty() -> Expr {
    arrow(cst("EllipticCurveObj"), arrow(nat_ty(), type0()))
}
/// `HeightFunction : EllipticCurveObj → EllipticPointObj → Real`
/// The canonical Néron-Tate height ĥ: E(K̄) → ℝ.
pub fn height_function_ty() -> Expr {
    arrow(
        cst("EllipticCurveObj"),
        arrow(cst("EllipticPointObj"), real_ty()),
    )
}
/// Mordell-Weil theorem: E(K) is a finitely generated abelian group.
///
/// `∀ (E : EllipticCurveObj) (K : NumberField), IsFinitelyGenerated (EllipticPoints E K)`
pub fn mordell_weil_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "E",
        cst("EllipticCurveObj"),
        pi(
            BinderInfo::Default,
            "K",
            cst("NumberField"),
            app(
                cst("IsFinitelyGenerated"),
                app2(cst("EllipticPoints"), bvar(1), bvar(0)),
            ),
        ),
    )
}
/// Torsion structure: E\[n\] ≅ (ℤ/nℤ)² over an algebraically closed field of characteristic 0.
///
/// `∀ (E : EllipticCurveObj) (n : Nat), IsAlgClosedChar0 k →
///    Iso (TorsionPoint E n) (DirectSum (ZMod n) (ZMod n))`
pub fn torsion_structure_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "E",
        cst("EllipticCurveObj"),
        pi(
            BinderInfo::Default,
            "n",
            nat_ty(),
            arrow(
                cst("IsAlgClosedChar0"),
                app2(
                    cst("Iso"),
                    app2(cst("TorsionPoint"), bvar(1), bvar(0)),
                    app2(
                        cst("DirectSum"),
                        app(cst("ZMod"), bvar(1)),
                        app(cst("ZMod"), bvar(1)),
                    ),
                ),
            ),
        ),
    )
}
/// Weil pairing: E\[n\] × E^\[n\] → μ_n (alternating, non-degenerate).
///
/// `∀ (E : EllipticCurveObj) (n : Nat), WeilPairing E n : TorsionPoint E n × TorsionPoint E^ n → RootsUnity n`
pub fn weil_pairing_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "E",
        cst("EllipticCurveObj"),
        pi(
            BinderInfo::Default,
            "n",
            nat_ty(),
            arrow(
                app2(
                    cst("ProductGroup"),
                    app2(cst("TorsionPoint"), bvar(1), bvar(0)),
                    app2(cst("TorsionPoint"), app(cst("DualEC"), bvar(1)), bvar(0)),
                ),
                app(cst("RootsUnityGroup"), bvar(0)),
            ),
        ),
    )
}
/// BSD conjecture data: rank of E(ℚ) equals order of vanishing of L(E, s) at s=1.
///
/// `∀ (E : EllipticCurveObj), ord_{s=1} L(E, s) = rank E(Q)`
pub fn bsd_conjecture_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "E",
        cst("EllipticCurveObj"),
        app2(
            cst("NatEq"),
            app(cst("LFunctionOrder"), bvar(0)),
            app(cst("MordellWeilRank"), bvar(0)),
        ),
    )
}
/// `ShimuraDatum : ReductiveGroup → HermitianDomain → Type`
/// A Shimura datum (G, X) where G is a reductive group and X a hermitian symmetric domain.
pub fn shimura_datum_ty() -> Expr {
    arrow(
        cst("ReductiveGroup"),
        arrow(cst("HermitianDomain"), type0()),
    )
}
/// `ShimuraVariety : ShimuraDatumObj → CompactOpenSubgroup → Type`
/// Sh_K(G, X) = G(ℚ) \ X × G(𝔸_f) / K, a moduli space of abelian varieties with extra structure.
pub fn shimura_variety_ty() -> Expr {
    arrow(
        cst("ShimuraDatumObj"),
        arrow(cst("CompactOpenSubgroup"), type0()),
    )
}
/// `CanonicalModel : ShimuraVarietyObj → ReflexField → AlgebraicVariety`
/// The canonical model of a Shimura variety over its reflex field E(G, X).
pub fn canonical_model_ty() -> Expr {
    arrow(
        cst("ShimuraVarietyObj"),
        arrow(cst("ReflexField"), cst("AlgebraicVariety")),
    )
}
/// `TolimaniConjecture : ShimuraVarietyObj → Prop`
/// The André-Oort conjecture: special subvarieties are Zariski closures of special points.
pub fn andre_oort_conjecture_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "Sh",
        cst("ShimuraVarietyObj"),
        pi(
            BinderInfo::Default,
            "Z",
            app(cst("IrreducibleSubvariety"), bvar(0)),
            arrow(
                app2(cst("IsZariskiClosureSpecialPoints"), bvar(1), bvar(0)),
                app2(cst("IsSpecialSubvariety"), bvar(1), bvar(0)),
            ),
        ),
    )
}
/// Reciprocity law for Shimura varieties: Frobenius acts on special points via the reflex norm.
///
/// `∀ (Sh : ShimuraVarietyObj) (x : SpecialPoint Sh), FrobeniusAction Sh x = ReflexNormOf x`
pub fn shimura_reciprocity_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "Sh",
        cst("ShimuraVarietyObj"),
        pi(
            BinderInfo::Default,
            "x",
            app(cst("SpecialPoint"), bvar(0)),
            app2(
                cst("PointEq"),
                app2(cst("FrobeniusAction"), bvar(1), bvar(0)),
                app(cst("ReflexNormOf"), bvar(0)),
            ),
        ),
    )
}
/// `GaloisRepresentation : GaloisGroup → Nat → Ring → Type`
/// A continuous homomorphism ρ: G_K → GL_n(R).
pub fn galois_representation_ty() -> Expr {
    arrow(
        cst("GaloisGroup"),
        arrow(nat_ty(), arrow(cst("Ring"), type0())),
    )
}
/// `NearlyOrdinaryRepresentation : GaloisGroup → Prime → Type`
/// A p-adic Galois representation admitting a Borel reduction at p (ordinary at p).
pub fn nearly_ordinary_representation_ty() -> Expr {
    arrow(cst("GaloisGroup"), arrow(cst("Prime"), type0()))
}
/// `AutomorphicRepresentation : ReductiveGroup → Type`
/// An automorphic representation π = ⊗_v π_v of a reductive group G over a number field.
pub fn automorphic_representation_ty() -> Expr {
    arrow(cst("ReductiveGroup"), type0())
}
/// `LanglandsCorrespondence : GaloisRepresentationObj → AutomorphicRepresentationObj → Prop`
/// The local/global Langlands correspondence: ρ_π ↔ π.
pub fn langlands_correspondence_ty() -> Expr {
    arrow(
        cst("GaloisRepresentationObj"),
        arrow(cst("AutomorphicRepresentationObj"), prop()),
    )
}
/// Local Langlands correspondence for GL_n.
///
/// `∀ (K : LocalField) (n : Nat), Bijection (GaloisReps K n) (AutomorphicReps (GL n K))`
pub fn local_langlands_gl_n_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "K",
        cst("LocalField"),
        pi(
            BinderInfo::Default,
            "n",
            nat_ty(),
            app2(
                cst("Bijection"),
                app2(cst("GaloisReps"), bvar(1), bvar(0)),
                app(cst("AutomorphicReps"), app2(cst("GL"), bvar(0), bvar(1))),
            ),
        ),
    )
}
/// Global Langlands conjecture: correspondence between automorphic forms and Galois representations.
///
/// `∀ (π : AutomorphicRepresentationObj), ∃ (ρ : GaloisRepresentationObj), IsAssociated ρ π`
pub fn global_langlands_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "pi_rep",
        cst("AutomorphicRepresentationObj"),
        app2(
            cst("Exists"),
            cst("GaloisRepresentationObj"),
            app2(cst("IsAssociated"), bvar(0), bvar(1)),
        ),
    )
}
/// Sato-Tate conjecture: for a non-CM elliptic curve E/ℚ, the distribution of
/// a_p / 2√p is equidistributed with respect to the Sato-Tate measure on \[-1, 1\].
///
/// `∀ (E : EllipticCurveObj), IsNonCM E → SatoTateEquidistributed E`
pub fn sato_tate_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "E",
        cst("EllipticCurveObj"),
        arrow(
            app(cst("IsNonCM"), bvar(0)),
            app(cst("SatoTateEquidistributed"), bvar(0)),
        ),
    )
}
/// `AbsoluteHeight : ProjectivePoint → Real`
/// H(P) = max|x_i|_v, the absolute Weil height on projective space.
pub fn absolute_height_ty() -> Expr {
    arrow(cst("ProjectivePoint"), real_ty())
}
/// `LogarithmicHeight : ProjectivePoint → Real`
/// h(P) = log H(P), the logarithmic Weil height.
pub fn logarithmic_height_ty() -> Expr {
    arrow(cst("ProjectivePoint"), real_ty())
}
/// `NorthcottProperty : Type → Prop`
/// A set S has the Northcott property if for all B, {P ∈ S : H(P) ≤ B} is finite.
pub fn northcott_property_ty() -> Expr {
    arrow(type0(), prop())
}
/// `FaltingsThm : AlgebraicCurve → Prop`
/// Faltings's theorem (Mordell conjecture): a curve of genus ≥ 2 over ℚ has finitely many rational points.
pub fn faltings_thm_ty() -> Expr {
    arrow(cst("AlgebraicCurve"), prop())
}
/// Northcott property for projective space.
///
/// `∀ (n : Nat) (B : Real), Finite {P : P^n(Q) | H(P) ≤ B}`
pub fn northcott_projective_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "B",
            real_ty(),
            app(
                cst("IsFinite"),
                app2(cst("ProjectivePointsBoundedHeight"), bvar(1), bvar(0)),
            ),
        ),
    )
}
/// Faltings's theorem (Mordell conjecture, proved by Faltings 1983).
///
/// `∀ (C : AlgebraicCurve) (K : NumberField), Genus C ≥ 2 → IsFinite (RationalPoints C K)`
pub fn faltings_mordell_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "C",
        cst("AlgebraicCurve"),
        pi(
            BinderInfo::Default,
            "K",
            cst("NumberField"),
            arrow(
                app(cst("GeqTwo"), app(cst("CurveGenus"), bvar(1))),
                app(
                    cst("IsFinite"),
                    app2(cst("RationalPoints"), bvar(1), bvar(0)),
                ),
            ),
        ),
    )
}
/// Néron-Tate height satisfies the parallelogram law.
///
/// `∀ (E : EllipticCurveObj) (P Q : EllipticPointObj),
///    ĥ(P + Q) + ĥ(P - Q) = 2 ĥ(P) + 2 ĥ(Q)`
pub fn neron_tate_parallelogram_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "E",
        cst("EllipticCurveObj"),
        pi(
            BinderInfo::Default,
            "P",
            cst("EllipticPointObj"),
            pi(
                BinderInfo::Default,
                "Q",
                cst("EllipticPointObj"),
                app2(
                    cst("RealEq"),
                    app2(
                        cst("Real.add"),
                        app(
                            cst("NeronTateHeight"),
                            app3(cst("EllipticAdd"), bvar(2), bvar(1), bvar(0)),
                        ),
                        app(
                            cst("NeronTateHeight"),
                            app3(cst("EllipticSub"), bvar(2), bvar(1), bvar(0)),
                        ),
                    ),
                    app2(
                        cst("Real.add"),
                        app2(
                            cst("Real.mul"),
                            cst("Two"),
                            app(cst("NeronTateHeight"), bvar(1)),
                        ),
                        app2(
                            cst("Real.mul"),
                            cst("Two"),
                            app(cst("NeronTateHeight"), bvar(0)),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// `PerfectoidSpace : Type`
/// A perfectoid space is a topological space with a perfectoid algebra structure,
/// used to transfer problems between characteristic 0 and characteristic p.
pub fn perfectoid_space_ty() -> Expr {
    type0()
}
/// `Diamond : PerfectoidSpaceObj → Type`
/// X^◇ = X / (Frobenius equivalence), the diamond associated to a perfectoid space.
pub fn diamond_ty() -> Expr {
    arrow(cst("PerfectoidSpaceObj"), type0())
}
/// `VStack : Site → Type`
/// A v-stack (sheaf on the v-topology), generalizing perfectoid spaces and diamonds.
pub fn v_stack_ty() -> Expr {
    arrow(cst("Site"), type0())
}
/// Tilting equivalence: there is an equivalence between perfectoid spaces of characteristic 0
/// and characteristic p via the tilt functor X ↦ X^♭.
///
/// `∀ (X : PerfectoidSpaceObj), IsPerfectoid X → IsEquiv (TiltFunctor X) (TiltedSpace X)`
pub fn tilting_equivalence_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        cst("PerfectoidSpaceObj"),
        arrow(
            app(cst("IsPerfectoid"), bvar(0)),
            app2(
                cst("IsEquiv"),
                app(cst("TiltFunctor"), bvar(0)),
                app(cst("TiltedSpace"), bvar(0)),
            ),
        ),
    )
}
/// `PrismaticCohomology : Scheme → Prism → AbelianGroup`
/// The prismatic cohomology H^*_{Δ}(X/A) of a p-adic formal scheme X over a prism (A, I).
pub fn prismatic_cohomology_ty() -> Expr {
    arrow(cst("Scheme"), arrow(cst("Prism"), cst("AbelianGroup")))
}
/// Prismatic comparison: prismatic cohomology specializes to de Rham, étale, and crystalline cohomology.
///
/// `∀ (X : Scheme) (A : Prism), PrismaticSpec X A → CohomologyComparisonTriangle X A`
pub fn prismatic_comparison_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        cst("Scheme"),
        pi(
            BinderInfo::Default,
            "A",
            cst("Prism"),
            arrow(
                app2(cst("PrismaticSpec"), bvar(1), bvar(0)),
                app2(cst("CohomologyComparisonTriangle"), bvar(1), bvar(0)),
            ),
        ),
    )
}
/// `Syntomic Cohomology : Scheme → Nat → AbelianGroup`
/// Syntomic cohomology H^i_{syn}(X, Z_p(n)), interpolating between crystalline and étale.
pub fn syntomic_cohomology_ty() -> Expr {
    arrow(cst("Scheme"), arrow(nat_ty(), cst("AbelianGroup")))
}
/// `FontaineTheory : PAdicField → GaloisRepresentationObj → HodgeTateDecomp`
/// Fontaine's theory associates a Hodge-Tate decomposition to a de Rham Galois representation.
pub fn fontaine_theory_ty() -> Expr {
    arrow(
        cst("PAdicField"),
        arrow(cst("GaloisRepresentationObj"), cst("HodgeTateDecomp")),
    )
}
/// Fontaine's C_dR ⊗ D_dR(V) ≅ C_dR ⊗ V: de Rham comparison for p-adic representations.
///
/// `∀ (K : PAdicField) (V : GaloisRepresentationObj), IsDeRham V →
///    Iso (TensorCdR (DdRModule V)) (TensorCdR V)`
pub fn fontaine_de_rham_comparison_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "K",
        cst("PAdicField"),
        pi(
            BinderInfo::Default,
            "V",
            cst("GaloisRepresentationObj"),
            arrow(
                app(cst("IsDeRham"), bvar(0)),
                app2(
                    cst("Iso"),
                    app(cst("TensorCdR"), app(cst("DdRModule"), bvar(0))),
                    app(cst("TensorCdR"), bvar(0)),
                ),
            ),
        ),
    )
}
/// `CondensedSet : Type`
/// A condensed set is a sheaf of sets on the category of profinite sets (Clausen-Scholze).
pub fn condensed_set_ty() -> Expr {
    type0()
}
/// `SolidAbelianGroup : CondensedAbelianGroupObj → Prop`
/// A condensed abelian group is solid if it satisfies the solid module condition (Clausen-Scholze).
pub fn solid_abelian_group_ty() -> Expr {
    arrow(cst("CondensedAbelianGroupObj"), prop())
}
/// `LiquidVectorSpace : Real → CondensedVectorSpaceObj → Prop`
/// A p-liquid vector space V is a condensed vector space satisfying the p-liquid condition.
pub fn liquid_vector_space_ty() -> Expr {
    arrow(real_ty(), arrow(cst("CondensedVectorSpaceObj"), prop()))
}
/// Analytic ring structure: every solid abelian group admits a natural analytic ring structure.
///
/// `∀ (G : CondensedAbelianGroupObj), IsSolid G → ExistsAnalyticRingStr G`
pub fn analytic_ring_structure_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "G",
        cst("CondensedAbelianGroupObj"),
        arrow(
            app(cst("IsSolid"), bvar(0)),
            app(cst("ExistsAnalyticRingStr"), bvar(0)),
        ),
    )
}
/// Liquid tensor product: the p-liquid tensor product is exact for 0 < p ≤ 1.
///
/// `∀ (p : Real) (V W : CondensedVectorSpaceObj), IsPLiquid p V → IsExact (LiquidTensor p V W)`
pub fn liquid_tensor_exact_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "p",
        real_ty(),
        pi(
            BinderInfo::Default,
            "V",
            cst("CondensedVectorSpaceObj"),
            pi(
                BinderInfo::Default,
                "W",
                cst("CondensedVectorSpaceObj"),
                arrow(
                    app2(cst("IsPLiquid"), bvar(2), bvar(1)),
                    app(
                        cst("IsExact"),
                        app3(cst("LiquidTensor"), bvar(2), bvar(1), bvar(0)),
                    ),
                ),
            ),
        ),
    )
}
/// `MotivicCohomology : Scheme → Int → Int → AbelianGroup`
/// Motivic cohomology H^{p,q}(X, Z), the bigraded cohomology theory of algebraic varieties.
pub fn motivic_cohomology_ty() -> Expr {
    arrow(
        cst("Scheme"),
        arrow(int_ty(), arrow(int_ty(), cst("AbelianGroup"))),
    )
}
/// `MixedMotive : NumberField → Type`
/// A mixed motive over a number field K: an object in Voevodsky's triangulated category of motives.
pub fn mixed_motive_ty() -> Expr {
    arrow(cst("NumberField"), type0())
}
/// `SliceFiltration : MotiveObj → Nat → MotiveObj`
/// Voevodsky's slice filtration s_n(M): the n-th slice of a motive M.
pub fn slice_filtration_ty() -> Expr {
    arrow(cst("MotiveObj"), arrow(nat_ty(), cst("MotiveObj")))
}
/// `A1HomotopyType : Scheme → A1SpaceObj`
/// The A¹-homotopy type Sing^{A¹}(X) of an algebraic variety X.
pub fn a1_homotopy_type_ty() -> Expr {
    arrow(cst("Scheme"), cst("A1SpaceObj"))
}
/// Motivic cohomology comparison: H^{2n,n}(X, Z) ≅ CH^n(X) (Chow groups).
///
/// `∀ (X : Scheme) (n : Nat), Iso (MotivicCohomology X (2*n) n) (ChowGroup X n)`
pub fn motivic_chow_comparison_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        cst("Scheme"),
        pi(
            BinderInfo::Default,
            "n",
            nat_ty(),
            app2(
                cst("Iso"),
                app3(cst("MotivicCohomologyGrp"), bvar(1), bvar(0), bvar(0)),
                app2(cst("ChowGroup"), bvar(1), bvar(0)),
            ),
        ),
    )
}
/// Beilinson-Lichtenbaum conjecture (proved): motivic cohomology agrees with étale cohomology
/// in the range p ≤ q (mod n).
///
/// `∀ (X : Scheme) (n : Nat), BeilinsonLichtenbaumIso X n`
pub fn beilinson_lichtenbaum_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        cst("Scheme"),
        pi(
            BinderInfo::Default,
            "n",
            nat_ty(),
            app2(cst("BeilinsonLichtenbaumIso"), bvar(1), bvar(0)),
        ),
    )
}
/// Voevodsky's cancellation theorem: ⊗ A¹ is invertible in the motivic stable homotopy category.
///
/// `∀ (M N : MotiveObj), Iso (TwistedMotive M) (TwistedMotive N) → Iso M N`
pub fn voevodsky_cancellation_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "M",
        cst("MotiveObj"),
        pi(
            BinderInfo::Default,
            "N",
            cst("MotiveObj"),
            arrow(
                app2(
                    cst("Iso"),
                    app(cst("TwistedMotive"), bvar(1)),
                    app(cst("TwistedMotive"), bvar(0)),
                ),
                app2(cst("Iso"), bvar(1), bvar(0)),
            ),
        ),
    )
}
/// `BerkovichSpace : AffinoidAlgebra → Type`
/// The Berkovich space M(A) of an affinoid algebra A: the space of bounded multiplicative seminorms.
pub fn berkovich_space_ty() -> Expr {
    arrow(cst("AffinoidAlgebra"), type0())
}
/// `AffinoidAlgebra : NonArchField → Type`
/// An affinoid algebra T_n/I over a non-archimedean field K.
pub fn affinoid_algebra_ty() -> Expr {
    arrow(cst("NonArchField"), type0())
}
/// `AdicSpace : HuberRing → Type`
/// An adic space Spa(A, A+) for a Huber ring A, generalizing rigid analytic spaces.
pub fn adic_space_ty() -> Expr {
    arrow(cst("HuberRing"), type0())
}
/// `RigidAnalyticSpace : NonArchField → Type`
/// A rigid analytic space (Tate, 1971) over a non-archimedean field.
pub fn rigid_analytic_space_ty() -> Expr {
    arrow(cst("NonArchField"), type0())
}
/// GAGA for rigid analytic spaces: coherent analytic sheaves correspond to algebraic coherent sheaves.
///
/// `∀ (X : ProjectiveScheme) (K : NonArchField), Equiv (AlgCohSheaves X) (AnCohSheaves (Analytify X K))`
pub fn rigid_gaga_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        cst("ProjectiveScheme"),
        pi(
            BinderInfo::Default,
            "K",
            cst("NonArchField"),
            app2(
                cst("Equiv"),
                app(cst("AlgCohSheaves"), bvar(1)),
                app(
                    cst("AnCohSheaves"),
                    app2(cst("Analytify"), bvar(1), bvar(0)),
                ),
            ),
        ),
    )
}
/// Berkovich analytification is a retract onto the skeleton.
///
/// `∀ (X : BerkovichSpaceObj), ∃ (S : Skeleton X), IsDeformRetract X S`
pub fn berkovich_skeleton_retract_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        cst("BerkovichSpaceObj"),
        app2(
            cst("Exists"),
            app(cst("Skeleton"), bvar(0)),
            app2(cst("IsDeformRetract"), bvar(1), bvar(0)),
        ),
    )
}
/// `ProEtaleTopos : Scheme → Topos`
/// The pro-étale topos X_{pro-ét} of a scheme X (Bhatt-Scholze).
pub fn pro_etale_topos_ty() -> Expr {
    arrow(cst("Scheme"), cst("Topos"))
}
/// Pro-étale cohomology: the pro-étale cohomology of X with coefficients in Z_ℓ.
///
/// `∀ (X : Scheme) (ell : Prime), Iso (ProEtCohomology X ell) (EtCohomology X ell)`
pub fn pro_etale_comparison_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        cst("Scheme"),
        pi(
            BinderInfo::Default,
            "ell",
            cst("Prime"),
            app2(
                cst("Iso"),
                app2(cst("ProEtCohomology"), bvar(1), bvar(0)),
                app2(cst("EtCohomology"), bvar(1), bvar(0)),
            ),
        ),
    )
}
/// `LogScheme : Scheme → LogStructure → Type`
/// A log scheme (X, M_X) where M_X is a sheaf of monoids on X (Fontaine-Illusie-Kato).
pub fn log_scheme_ty() -> Expr {
    arrow(cst("Scheme"), arrow(cst("LogStructure"), type0()))
}
/// `LogEtaleCohomology : LogSchemeObj → Nat → AbelianGroup`
/// Log-étale cohomology H^i_{log-et}(X, Z/nZ) of a log scheme X.
pub fn log_etale_cohomology_ty() -> Expr {
    arrow(cst("LogSchemeObj"), arrow(nat_ty(), cst("AbelianGroup")))
}
/// `LogCrystallineCohomology : LogSchemeObj → Ring → AbelianGroup`
/// Log-crystalline cohomology H^i_{log-cris}(X/W) with W the Witt vectors.
pub fn log_crystalline_cohomology_ty() -> Expr {
    arrow(cst("LogSchemeObj"), arrow(cst("Ring"), cst("AbelianGroup")))
}
/// `NeronModel : AbelianVarietyObj → DVRing → GroupScheme`
/// The Néron model N(A) of an abelian variety A over the fraction field of a DVR.
pub fn neron_model_ty() -> Expr {
    arrow(
        cst("AbelianVarietyObj"),
        arrow(cst("DVRing"), cst("GroupScheme")),
    )
}
/// Néron mapping property: every rational map from a smooth scheme to A extends to N(A).
///
/// `∀ (A : AbelianVarietyObj) (R : DVRing) (S : SmoothSchemeObj),
///    ∀ (f : RationalMap S A), UniqueExtension (NeronModel A R) f`
pub fn neron_mapping_property_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "A",
        cst("AbelianVarietyObj"),
        pi(
            BinderInfo::Default,
            "R",
            cst("DVRing"),
            pi(
                BinderInfo::Default,
                "S",
                cst("SmoothSchemeObj"),
                pi(
                    BinderInfo::Default,
                    "f",
                    app2(cst("RationalMap"), bvar(0), bvar(2)),
                    app2(
                        cst("UniqueExtension"),
                        app2(cst("NeronModel"), bvar(3), bvar(2)),
                        bvar(0),
                    ),
                ),
            ),
        ),
    )
}
/// Semi-stable reduction theorem: after a finite base change, any abelian variety becomes semi-stable.
///
/// `∀ (A : AbelianVarietyObj) (K : NumberField), ∃ (L : NumberField), IsSemiStable (BaseChange A L)`
pub fn semi_stable_reduction_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "A",
        cst("AbelianVarietyObj"),
        pi(
            BinderInfo::Default,
            "K",
            cst("NumberField"),
            app2(
                cst("Exists"),
                cst("NumberField"),
                app(
                    cst("IsSemiStable"),
                    app2(cst("BaseChange"), bvar(1), bvar(0)),
                ),
            ),
        ),
    )
}
/// `FaltingsHeight : AbelianVarietyObj → Real`
/// The Faltings height h_F(A) of a principally polarized abelian variety A.
pub fn faltings_height_ty() -> Expr {
    arrow(cst("AbelianVarietyObj"), real_ty())
}
/// Northcott for Faltings heights: there are finitely many isomorphism classes of
/// principally polarized abelian varieties over K with bounded Faltings height and fixed dimension.
///
/// `∀ (g : Nat) (K : NumberField) (B : Real), IsFinite (PPAVBoundedFaltingsHeight g K B)`
pub fn northcott_faltings_height_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "g",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "K",
            cst("NumberField"),
            pi(
                BinderInfo::Default,
                "B",
                real_ty(),
                app(
                    cst("IsFinite"),
                    app3(cst("PPAVBoundedFaltingsHeight"), bvar(2), bvar(1), bvar(0)),
                ),
            ),
        ),
    )
}
/// `CrystallineCohomology : Scheme → Ring → AbelianGroup`
/// Crystalline cohomology H^i_{cris}(X/W) of a smooth proper variety in characteristic p.
pub fn crystalline_cohomology_ty() -> Expr {
    arrow(cst("Scheme"), arrow(cst("Ring"), cst("AbelianGroup")))
}
/// Crystalline comparison theorem (Berthelot-Ogus): for a smooth proper scheme over W(k),
/// H^i_{cris}(X_k/W(k)) ≅ H^i_{dR}(X/W(k)).
///
/// `∀ (X : SmoothProperScheme) (k : PerfectField), CrystallineDeRhamIso X k`
pub fn crystalline_de_rham_iso_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        cst("SmoothProperScheme"),
        pi(
            BinderInfo::Default,
            "k",
            cst("PerfectField"),
            app2(cst("CrystallineDeRhamIso"), bvar(1), bvar(0)),
        ),
    )
}
/// Register all arithmetic geometry axioms and theorem types in the environment.
pub fn build_env(env: &mut Environment) {
    let base_types: &[(&str, Expr)] = &[
        ("Field", type0()),
        ("NumberField", type0()),
        ("LocalField", type0()),
        ("FiniteField", type0()),
        ("Ring", type0()),
        ("Prime", nat_ty()),
        ("Real", type0()),
        ("AlgebraicVariety", type0()),
        ("AlgebraicCurve", type0()),
        ("ProjectivePoint", type0()),
        ("EllipticPointObj", type0()),
        ("AbelianVarietyObj", type0()),
        ("EllipticCurveObj", type0()),
        ("ShimuraDatumObj", type0()),
        ("ShimuraVarietyObj", type0()),
        ("GaloisRepresentationObj", type0()),
        ("AutomorphicRepresentationObj", type0()),
        ("ReductiveGroup", type0()),
        ("HermitianDomain", type0()),
        ("CompactOpenSubgroup", type0()),
        ("ReflexField", type0()),
        ("GaloisGroup", type0()),
        ("ZpModule", type0()),
        (
            "TateModule",
            arrow(
                cst("AbelianVarietyObj"),
                arrow(cst("Prime"), cst("ZpModule")),
            ),
        ),
        (
            "DualAbelianVariety",
            arrow(cst("AbelianVarietyObj"), cst("AbelianVarietyObj")),
        ),
        (
            "AbelianVarietyDim",
            arrow(cst("AbelianVarietyObj"), nat_ty()),
        ),
        ("ZpModuleRank", arrow(cst("ZpModule"), nat_ty())),
        ("NatMul2", arrow(nat_ty(), nat_ty())),
        ("NatEq", arrow(nat_ty(), arrow(nat_ty(), prop()))),
        ("Iso", arrow(type0(), arrow(type0(), prop()))),
        ("And", arrow(prop(), arrow(prop(), prop()))),
        ("Exists", arrow(type0(), arrow(type0(), prop()))),
        ("IsFinitelyGenerated", arrow(type0(), prop())),
        ("IsFinite", arrow(type0(), prop())),
        (
            "IsogenyMap",
            arrow(
                cst("EllipticCurveObj"),
                arrow(cst("EllipticCurveObj"), type0()),
            ),
        ),
        (
            "TorsionPoint",
            arrow(cst("EllipticCurveObj"), arrow(nat_ty(), type0())),
        ),
        (
            "EllipticPoints",
            arrow(cst("EllipticCurveObj"), arrow(cst("NumberField"), type0())),
        ),
        (
            "IsogenousToProduct",
            arrow(
                cst("AbelianVarietyObj"),
                arrow(list_ty(cst("AbelianVarietyObj")), prop()),
            ),
        ),
        (
            "HomTensor",
            arrow(
                cst("AbelianVarietyObj"),
                arrow(cst("AbelianVarietyObj"), arrow(cst("Prime"), type0())),
            ),
        ),
        (
            "TateModuleHom",
            arrow(
                cst("AbelianVarietyObj"),
                arrow(cst("AbelianVarietyObj"), arrow(cst("Prime"), type0())),
            ),
        ),
        ("IsFiniteField", prop()),
        ("ZMod", arrow(nat_ty(), type0())),
        ("DirectSum", arrow(type0(), arrow(type0(), type0()))),
        ("IsAlgClosedChar0", prop()),
        (
            "DualEC",
            arrow(cst("EllipticCurveObj"), cst("EllipticCurveObj")),
        ),
        ("ProductGroup", arrow(type0(), arrow(type0(), type0()))),
        ("RootsUnityGroup", arrow(nat_ty(), type0())),
        ("LFunctionOrder", arrow(cst("EllipticCurveObj"), nat_ty())),
        ("MordellWeilRank", arrow(cst("EllipticCurveObj"), nat_ty())),
        ("Bijection", arrow(type0(), arrow(type0(), prop()))),
        ("GL", arrow(nat_ty(), arrow(cst("LocalField"), type0()))),
        (
            "GaloisReps",
            arrow(cst("LocalField"), arrow(nat_ty(), type0())),
        ),
        ("AutomorphicReps", arrow(type0(), type0())),
        (
            "IsAssociated",
            arrow(
                cst("GaloisRepresentationObj"),
                arrow(cst("AutomorphicRepresentationObj"), prop()),
            ),
        ),
        ("IsNonCM", arrow(cst("EllipticCurveObj"), prop())),
        (
            "SatoTateEquidistributed",
            arrow(cst("EllipticCurveObj"), prop()),
        ),
        (
            "IrreducibleSubvariety",
            arrow(cst("ShimuraVarietyObj"), type0()),
        ),
        (
            "IsZariskiClosureSpecialPoints",
            arrow(cst("ShimuraVarietyObj"), arrow(type0(), prop())),
        ),
        (
            "IsSpecialSubvariety",
            arrow(cst("ShimuraVarietyObj"), arrow(type0(), prop())),
        ),
        ("SpecialPoint", arrow(cst("ShimuraVarietyObj"), type0())),
        ("PointEq", arrow(type0(), arrow(type0(), prop()))),
        (
            "FrobeniusAction",
            arrow(cst("ShimuraVarietyObj"), arrow(type0(), type0())),
        ),
        ("ReflexNormOf", arrow(type0(), type0())),
        (
            "ProjectivePointsBoundedHeight",
            arrow(nat_ty(), arrow(real_ty(), type0())),
        ),
        ("CurveGenus", arrow(cst("AlgebraicCurve"), nat_ty())),
        ("GeqTwo", arrow(nat_ty(), prop())),
        (
            "RationalPoints",
            arrow(cst("AlgebraicCurve"), arrow(cst("NumberField"), type0())),
        ),
        ("NeronTateHeight", arrow(cst("EllipticPointObj"), real_ty())),
        ("RealEq", arrow(real_ty(), arrow(real_ty(), prop()))),
        ("Real.add", arrow(real_ty(), arrow(real_ty(), real_ty()))),
        ("Real.mul", arrow(real_ty(), arrow(real_ty(), real_ty()))),
        ("Two", real_ty()),
        (
            "EllipticAdd",
            arrow(
                cst("EllipticCurveObj"),
                arrow(
                    cst("EllipticPointObj"),
                    arrow(cst("EllipticPointObj"), cst("EllipticPointObj")),
                ),
            ),
        ),
        (
            "EllipticSub",
            arrow(
                cst("EllipticCurveObj"),
                arrow(
                    cst("EllipticPointObj"),
                    arrow(cst("EllipticPointObj"), cst("EllipticPointObj")),
                ),
            ),
        ),
        ("Spec", arrow(cst("Field"), type0())),
        (
            "LongExactSeq",
            arrow(type0(), arrow(type0(), arrow(type0(), prop()))),
        ),
        ("PerfectoidSpaceObj", type0()),
        ("Prism", type0()),
        ("PAdicField", type0()),
        ("AbelianGroup", type0()),
        ("HodgeTateDecomp", type0()),
        ("Site", type0()),
        ("IsPerfectoid", arrow(cst("PerfectoidSpaceObj"), prop())),
        ("TiltFunctor", arrow(cst("PerfectoidSpaceObj"), type0())),
        ("TiltedSpace", arrow(cst("PerfectoidSpaceObj"), type0())),
        ("IsEquiv", arrow(type0(), arrow(type0(), prop()))),
        (
            "PrismaticSpec",
            arrow(cst("Scheme"), arrow(cst("Prism"), prop())),
        ),
        (
            "CohomologyComparisonTriangle",
            arrow(cst("Scheme"), arrow(cst("Prism"), prop())),
        ),
        ("IsDeRham", arrow(cst("GaloisRepresentationObj"), prop())),
        ("TensorCdR", arrow(type0(), type0())),
        ("DdRModule", arrow(cst("GaloisRepresentationObj"), type0())),
        ("CondensedAbelianGroupObj", type0()),
        ("CondensedVectorSpaceObj", type0()),
        ("IsSolid", arrow(cst("CondensedAbelianGroupObj"), prop())),
        (
            "ExistsAnalyticRingStr",
            arrow(cst("CondensedAbelianGroupObj"), prop()),
        ),
        (
            "IsPLiquid",
            arrow(real_ty(), arrow(cst("CondensedVectorSpaceObj"), prop())),
        ),
        ("IsExact", arrow(type0(), prop())),
        (
            "LiquidTensor",
            arrow(
                real_ty(),
                arrow(
                    cst("CondensedVectorSpaceObj"),
                    arrow(cst("CondensedVectorSpaceObj"), type0()),
                ),
            ),
        ),
        ("Scheme", type0()),
        ("MotiveObj", type0()),
        ("A1SpaceObj", type0()),
        (
            "MotivicCohomologyGrp",
            arrow(
                cst("Scheme"),
                arrow(nat_ty(), arrow(nat_ty(), cst("AbelianGroup"))),
            ),
        ),
        ("ChowGroup", arrow(cst("Scheme"), arrow(nat_ty(), type0()))),
        (
            "BeilinsonLichtenbaumIso",
            arrow(cst("Scheme"), arrow(nat_ty(), prop())),
        ),
        ("TwistedMotive", arrow(cst("MotiveObj"), cst("MotiveObj"))),
        ("AffinoidAlgebra", type0()),
        ("NonArchField", type0()),
        ("HuberRing", type0()),
        ("Topos", type0()),
        ("ProjectiveScheme", type0()),
        ("BerkovichSpaceObj", type0()),
        ("AlgCohSheaves", arrow(cst("ProjectiveScheme"), type0())),
        ("AnCohSheaves", arrow(type0(), type0())),
        (
            "Analytify",
            arrow(cst("ProjectiveScheme"), arrow(cst("NonArchField"), type0())),
        ),
        ("Equiv", arrow(type0(), arrow(type0(), prop()))),
        ("Skeleton", arrow(cst("BerkovichSpaceObj"), type0())),
        (
            "IsDeformRetract",
            arrow(cst("BerkovichSpaceObj"), arrow(type0(), prop())),
        ),
        (
            "ProEtCohomology",
            arrow(cst("Scheme"), arrow(cst("Prime"), type0())),
        ),
        (
            "EtCohomology",
            arrow(cst("Scheme"), arrow(cst("Prime"), type0())),
        ),
        ("LogStructure", type0()),
        ("LogSchemeObj", type0()),
        ("DVRing", type0()),
        ("GroupScheme", type0()),
        ("SmoothSchemeObj", type0()),
        ("SmoothProperScheme", type0()),
        ("PerfectField", type0()),
        (
            "RationalMap",
            arrow(
                cst("SmoothSchemeObj"),
                arrow(cst("AbelianVarietyObj"), type0()),
            ),
        ),
        (
            "UniqueExtension",
            arrow(cst("GroupScheme"), arrow(type0(), prop())),
        ),
        (
            "NeronModel",
            arrow(
                cst("AbelianVarietyObj"),
                arrow(cst("DVRing"), cst("GroupScheme")),
            ),
        ),
        ("IsSemiStable", arrow(type0(), prop())),
        (
            "BaseChange",
            arrow(cst("AbelianVarietyObj"), arrow(cst("NumberField"), type0())),
        ),
        (
            "PPAVBoundedFaltingsHeight",
            arrow(
                nat_ty(),
                arrow(cst("NumberField"), arrow(real_ty(), type0())),
            ),
        ),
        (
            "CrystallineDeRhamIso",
            arrow(
                cst("SmoothProperScheme"),
                arrow(cst("PerfectField"), prop()),
            ),
        ),
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
        ("AbelianVarietyType", abelian_variety_ty),
        ("PolarizedAbelianVarietyType", polarized_abelian_variety_ty),
        ("TateModuleType", tate_module_ty),
        ("DualAbelianVarietyType", dual_abelian_variety_ty),
        ("EllipticCurveType", elliptic_curve_ty),
        ("IsogenyType", isogeny_ty),
        ("TorsionPointType", torsion_point_ty),
        ("HeightFunctionType", height_function_ty),
        ("ShimuraDatumType", shimura_datum_ty),
        ("ShimuraVarietyType", shimura_variety_ty),
        ("CanonicalModelType", canonical_model_ty),
        ("GaloisRepresentationType", galois_representation_ty),
        ("NearlyOrdinaryRepType", nearly_ordinary_representation_ty),
        (
            "AutomorphicRepresentationType",
            automorphic_representation_ty,
        ),
        ("LanglandsCorrespondenceType", langlands_correspondence_ty),
        ("AbsoluteHeightType", absolute_height_ty),
        ("LogarithmicHeightType", logarithmic_height_ty),
        ("NorthcottPropertyType", northcott_property_ty),
        ("FaltingsThmType", faltings_thm_ty),
        ("PerfectoidSpaceType", perfectoid_space_ty),
        ("DiamondType", diamond_ty),
        ("VStackType", v_stack_ty),
        ("PrismaticCohomologyType", prismatic_cohomology_ty),
        ("SyntomicCohomologyType", syntomic_cohomology_ty),
        ("FontaineTheoryType", fontaine_theory_ty),
        ("CondensedSetType", condensed_set_ty),
        ("SolidAbelianGroupType", solid_abelian_group_ty),
        ("LiquidVectorSpaceType", liquid_vector_space_ty),
        ("MotivicCohomologyType", motivic_cohomology_ty),
        ("MixedMotiveType", mixed_motive_ty),
        ("SliceFiltrationType", slice_filtration_ty),
        ("A1HomotopyTypeType", a1_homotopy_type_ty),
        ("BerkovichSpaceType", berkovich_space_ty),
        ("AffinoidAlgebraType", affinoid_algebra_ty),
        ("AdicSpaceType", adic_space_ty),
        ("RigidAnalyticSpaceType", rigid_analytic_space_ty),
        ("ProEtaleToposType", pro_etale_topos_ty),
        ("LogSchemeType", log_scheme_ty),
        ("LogEtaleCohomologyType", log_etale_cohomology_ty),
        (
            "LogCrystallineCohomologyType",
            log_crystalline_cohomology_ty,
        ),
        ("NeronModelType", neron_model_ty),
        ("FaltingsHeightType", faltings_height_ty),
        ("CrystallineCohomologyType", crystalline_cohomology_ty),
    ];
    for (name, mk_ty) in type_axioms {
        env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty: mk_ty(),
        })
        .ok();
    }
    let theorem_axioms: &[(&str, fn() -> Expr)] = &[
        ("poincare_reducibility", poincare_reducibility_ty),
        ("tate_module_rank", tate_module_rank_ty),
        ("isogeny_theorem", isogeny_theorem_ty),
        ("mordell_weil", mordell_weil_ty),
        ("torsion_structure", torsion_structure_ty),
        ("weil_pairing", weil_pairing_ty),
        ("bsd_conjecture", bsd_conjecture_ty),
        ("andre_oort_conjecture", andre_oort_conjecture_ty),
        ("shimura_reciprocity", shimura_reciprocity_ty),
        ("local_langlands_gl_n", local_langlands_gl_n_ty),
        ("global_langlands", global_langlands_ty),
        ("sato_tate", sato_tate_ty),
        ("northcott_projective", northcott_projective_ty),
        ("faltings_mordell", faltings_mordell_ty),
        ("neron_tate_parallelogram", neron_tate_parallelogram_ty),
        ("tilting_equivalence", tilting_equivalence_ty),
        ("prismatic_comparison", prismatic_comparison_ty),
        (
            "fontaine_de_rham_comparison",
            fontaine_de_rham_comparison_ty,
        ),
        ("analytic_ring_structure", analytic_ring_structure_ty),
        ("liquid_tensor_exact", liquid_tensor_exact_ty),
        ("motivic_chow_comparison", motivic_chow_comparison_ty),
        ("beilinson_lichtenbaum", beilinson_lichtenbaum_ty),
        ("voevodsky_cancellation", voevodsky_cancellation_ty),
        ("rigid_gaga", rigid_gaga_ty),
        ("berkovich_skeleton_retract", berkovich_skeleton_retract_ty),
        ("pro_etale_comparison", pro_etale_comparison_ty),
        ("neron_mapping_property", neron_mapping_property_ty),
        ("semi_stable_reduction", semi_stable_reduction_ty),
        ("northcott_faltings_height", northcott_faltings_height_ty),
        ("crystalline_de_rham_iso", crystalline_de_rham_iso_ty),
    ];
    for (name, mk_ty) in theorem_axioms {
        env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty: mk_ty(),
        })
        .ok();
    }
}
/// Discriminant of a Weierstrass cubic y² = x³ + ax + b.
pub fn weierstrass_discriminant(a: i64, b: i64) -> i64 {
    -16 * (4 * a.pow(3) + 27 * b.pow(2))
}
/// j-invariant of y² = x³ + ax + b (returns None if singular).
pub fn j_invariant(a: i64, b: i64) -> Option<f64> {
    let delta = weierstrass_discriminant(a, b);
    if delta == 0 {
        return None;
    }
    Some(-1728.0 * (4.0 * a as f64).powi(3) / (delta as f64))
}
/// Estimate the number of rational points on E(𝔽_q) using Hasse's bound.
///
/// |#E(𝔽_q) - (q + 1)| ≤ 2√q.
/// Returns (lower bound, upper bound).
pub fn hasse_bound(q: u64) -> (i64, i64) {
    let two_sqrt_q = 2.0 * (q as f64).sqrt();
    let center = (q as i64) + 1;
    (center - two_sqrt_q as i64, center + two_sqrt_q as i64)
}
/// Compute the rank of an elliptic curve E/ℚ via naive 2-descent lower bound.
///
/// This is a placeholder returning 0; full 2-descent requires more data.
pub fn rank_lower_bound_2descent(_a: i64, _b: i64) -> usize {
    0
}
