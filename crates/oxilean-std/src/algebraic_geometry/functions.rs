//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{
    AffineScheme, AffineSchemeData, AffineVariety, ChernClass, Divisor, DivisorClass,
    EllipticCurveF, EllipticCurvePoint, ModuliProblem, Morphism, ProjectivePoint, ProjectiveSpace,
    ProjectiveVariety, Sheaf,
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
pub fn app4(f: Expr, a: Expr, b: Expr, c: Expr, d: Expr) -> Expr {
    app(app3(f, a, b, c), d)
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
pub fn bool_ty() -> Expr {
    cst("Bool")
}
pub fn list_ty(elem: Expr) -> Expr {
    app(cst("List"), elem)
}
pub fn option_ty(elem: Expr) -> Expr {
    app(cst("Option"), elem)
}
/// `Scheme : Type 1` — the type of all schemes (Grothendieck's definition).
pub fn scheme_ty() -> Expr {
    type1()
}
/// `RingOfFunctions : Scheme → CommRing` — sheaf of regular functions on a scheme.
pub fn ring_of_functions_ty() -> Expr {
    arrow(cst("Scheme"), cst("CommRing"))
}
/// `Sheaf : Scheme → Type → Type 1` — sheaves of modules on a scheme.
/// `Sheaf X T` is the type of sheaves of `T`-modules on `X`.
pub fn sheaf_ty() -> Expr {
    arrow(cst("Scheme"), arrow(type0(), type1()))
}
/// `AffineScheme : CommRing → Scheme` — Spec construction sending rings to affine schemes.
pub fn affine_scheme_ty() -> Expr {
    arrow(cst("CommRing"), cst("Scheme"))
}
/// `ProjectiveSpace : Nat → Scheme` — n-dimensional projective space P^n over a base.
pub fn projective_space_ty() -> Expr {
    arrow(nat_ty(), cst("Scheme"))
}
/// `MorphismOfSchemes : Scheme → Scheme → Type` — morphisms between schemes.
pub fn morphism_of_schemes_ty() -> Expr {
    arrow(cst("Scheme"), arrow(cst("Scheme"), type0()))
}
/// `CoherentSheaf : Scheme → Type 1` — coherent sheaves on a scheme.
pub fn coherent_sheaf_ty() -> Expr {
    arrow(cst("Scheme"), type1())
}
/// `Divisor : Scheme → Type` — Weil divisors on a scheme.
pub fn divisor_ty() -> Expr {
    arrow(cst("Scheme"), type0())
}
/// `LineBundle : Scheme → Type` — invertible sheaves (line bundles) on a scheme.
pub fn line_bundle_ty() -> Expr {
    arrow(cst("Scheme"), type0())
}
/// `AffineVariety : CommRing → Nat → Type` — an affine variety defined by a
/// ring quotient embedded in affine n-space.
pub fn affine_variety_ty() -> Expr {
    arrow(cst("CommRing"), arrow(nat_ty(), type0()))
}
/// `ZariskiOpen : Scheme → Type` — Zariski open subsets of a scheme.
pub fn zariski_open_ty() -> Expr {
    arrow(cst("Scheme"), type0())
}
/// `ZariskiClosed : Scheme → Type` — Zariski closed subsets (varieties) of a scheme.
pub fn zariski_closed_ty() -> Expr {
    arrow(cst("Scheme"), type0())
}
/// `HomogeneousIdeal : CommRing → Type` — homogeneous ideals in a graded ring.
pub fn homogeneous_ideal_ty() -> Expr {
    arrow(cst("CommRing"), type0())
}
/// `ProjectiveVariety : CommRing → Nat → Type` — projective variety as a closed
/// subscheme of projective n-space defined by homogeneous polynomials.
pub fn projective_variety_ty() -> Expr {
    arrow(cst("CommRing"), arrow(nat_ty(), type0()))
}
/// `RationalMap : Scheme → Scheme → Type` — rational maps between varieties
/// (defined on a dense open subset).
pub fn rational_map_ty() -> Expr {
    arrow(cst("Scheme"), arrow(cst("Scheme"), type0()))
}
/// `StructureSheaf : Scheme → Type` — the structure sheaf O_X on a scheme X.
pub fn structure_sheaf_ty() -> Expr {
    arrow(cst("Scheme"), type0())
}
/// `QuasiCoherentSheaf : Scheme → Type 1` — quasi-coherent sheaves (larger class
/// than coherent; closed under arbitrary direct limits).
pub fn quasi_coherent_sheaf_ty() -> Expr {
    arrow(cst("Scheme"), type1())
}
/// `AmpLeLineBundle : Scheme → Type` — ample line bundles on a projective scheme,
/// providing the data needed for projective embedding.
pub fn ample_line_bundle_ty() -> Expr {
    arrow(cst("Scheme"), type0())
}
/// `PicardGroup : Scheme → Type` — the Picard group Pic(X) = line bundles / ≅.
pub fn picard_group_ty() -> Expr {
    arrow(cst("Scheme"), type0())
}
/// `CartierDivisor : Scheme → Type` — Cartier divisors (locally principal Weil divisors).
pub fn cartier_divisor_ty() -> Expr {
    arrow(cst("Scheme"), type0())
}
/// `LinearSystem : Scheme → Divisor Scheme → Type` — the complete linear system |D|
/// associated to a divisor D on a scheme X.
pub fn linear_system_ty() -> Expr {
    arrow(
        cst("Scheme"),
        arrow(app(cst("Divisor"), cst("Scheme")), type0()),
    )
}
/// `BaseLocus : Scheme → Divisor Scheme → Type` — the base locus of a linear system,
/// the intersection of all divisors in |D|.
pub fn base_locus_ty() -> Expr {
    arrow(
        cst("Scheme"),
        arrow(app(cst("Divisor"), cst("Scheme")), type0()),
    )
}
/// `JacobianVariety : Scheme → Scheme` — the Jacobian J(C) of a smooth projective
/// curve C, parametrising degree-0 line bundles.
pub fn jacobian_variety_ty() -> Expr {
    arrow(cst("Scheme"), cst("Scheme"))
}
/// `EllipticCurve : CommRing → Type` — an elliptic curve over a ring (smooth genus-1
/// projective curve with a marked point).
pub fn elliptic_curve_ty() -> Expr {
    arrow(cst("CommRing"), type0())
}
/// `EllipticCurvePoint : CommRing → EllipticCurve CommRing → Type` — a point on an
/// elliptic curve, including the point at infinity.
pub fn elliptic_curve_point_ty() -> Expr {
    arrow(
        cst("CommRing"),
        arrow(app(cst("EllipticCurve"), cst("CommRing")), type0()),
    )
}
/// `TorsionSubgroup : CommRing → EllipticCurve CommRing → Nat → Type` — the n-torsion
/// subgroup E\[n\] of an elliptic curve.
pub fn torsion_subgroup_ty() -> Expr {
    arrow(
        cst("CommRing"),
        arrow(
            app(cst("EllipticCurve"), cst("CommRing")),
            arrow(nat_ty(), type0()),
        ),
    )
}
/// `AbelianVariety : CommRing → Nat → Type` — an abelian variety of dimension g over
/// a ring (a projective group scheme).
pub fn abelian_variety_ty() -> Expr {
    arrow(cst("CommRing"), arrow(nat_ty(), type0()))
}
/// `HodgeNumber : Scheme → Nat → Nat → Nat` — the Hodge number h^{p,q}(X).
pub fn hodge_number_ty() -> Expr {
    arrow(cst("Scheme"), arrow(nat_ty(), arrow(nat_ty(), nat_ty())))
}
/// `EtaleSite : Scheme → Type 1` — the étale site of a scheme (the category of
/// étale covers used in étale cohomology).
pub fn etale_site_ty() -> Expr {
    arrow(cst("Scheme"), type1())
}
/// `EtaleCohomology : Scheme → Nat → CommRing → Type` — the étale cohomology group
/// H^i_ét(X, ℓ) for prime ℓ.
pub fn etale_cohomology_ty() -> Expr {
    arrow(
        cst("Scheme"),
        arrow(nat_ty(), arrow(cst("CommRing"), type0())),
    )
}
/// `WeilConjectureData : Scheme → Type` — the zeta function data for a variety over
/// a finite field, as formulated by Weil.
pub fn weil_conjecture_data_ty() -> Expr {
    arrow(cst("Scheme"), type0())
}
/// `ZetaFunction : Scheme → Type` — the Hasse-Weil zeta function Z(X, t) of a
/// variety over a finite field.
pub fn zeta_function_ty() -> Expr {
    arrow(cst("Scheme"), type0())
}
/// `SheafCohomology : Scheme → Nat → CoherentSheaf Scheme → Type` — cohomology of
/// a coherent sheaf: H^i(X, F).
pub fn sheaf_cohomology_ty() -> Expr {
    arrow(
        cst("Scheme"),
        arrow(
            nat_ty(),
            arrow(app(cst("CoherentSheaf"), cst("Scheme")), type0()),
        ),
    )
}
/// `spec_functor_ty` — Spec is a contravariant functor from CommRing to Schemes.
///
/// `∀ (R S : CommRing), RingHom R S → MorphismOfSchemes (Spec S) (Spec R)`
pub fn spec_functor_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "R",
        cst("CommRing"),
        pi(
            BinderInfo::Default,
            "S",
            cst("CommRing"),
            arrow(
                app2(cst("RingHom"), bvar(1), bvar(0)),
                app2(
                    cst("MorphismOfSchemes"),
                    app(cst("Spec"), bvar(0)),
                    app(cst("Spec"), bvar(1)),
                ),
            ),
        ),
    )
}
/// `affine_cover_ty` — every scheme has an open cover by affine schemes.
///
/// `∀ (X : Scheme), ∃ (cover : List Scheme), AllAffine cover ∧ CoversScheme cover X`
pub fn affine_cover_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        cst("Scheme"),
        app2(
            cst("Exists"),
            list_ty(cst("Scheme")),
            app2(
                cst("And"),
                app(cst("AllAffine"), bvar(0)),
                app2(cst("CoversScheme"), bvar(0), bvar(1)),
            ),
        ),
    )
}
/// `yoneda_schemes_ty` — a scheme is determined by its functor of points.
///
/// `∀ (X Y : Scheme), (∀ R : CommRing, MorphismOfSchemes (Spec R) X ≃ MorphismOfSchemes (Spec R) Y) → X = Y`
pub fn yoneda_schemes_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        cst("Scheme"),
        pi(
            BinderInfo::Default,
            "Y",
            cst("Scheme"),
            arrow(
                pi(
                    BinderInfo::Default,
                    "R",
                    cst("CommRing"),
                    app2(
                        cst("Equiv"),
                        app2(cst("MorphismOfSchemes"), app(cst("Spec"), bvar(0)), bvar(2)),
                        app2(cst("MorphismOfSchemes"), app(cst("Spec"), bvar(0)), bvar(1)),
                    ),
                ),
                app2(cst("Eq"), bvar(1), bvar(0)),
            ),
        ),
    )
}
/// `riemann_roch_ty` — the Riemann-Roch theorem for curves.
///
/// `∀ (C : Scheme) (D : Divisor C), h0(D) - h1(D) = deg(D) - genus(C) + 1`
pub fn riemann_roch_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "C",
        cst("Scheme"),
        pi(
            BinderInfo::Default,
            "D",
            app(cst("Divisor"), bvar(0)),
            app2(
                cst("Eq"),
                app2(
                    cst("Int.sub"),
                    app(cst("H0"), bvar(0)),
                    app(cst("H1"), bvar(0)),
                ),
                app2(
                    cst("Int.sub"),
                    app2(
                        cst("Int.sub"),
                        app(cst("DivisorDeg"), bvar(0)),
                        app(cst("Genus"), bvar(1)),
                    ),
                    cst("Int.one"),
                ),
            ),
        ),
    )
}
/// `serre_duality_ty` — Serre duality for coherent sheaves.
///
/// `∀ (X : Scheme) (F : CoherentSheaf X) (n : Nat),
///   H^i(X, F) ≃ Dual(H^{n-i}(X, ω ⊗ F))`
pub fn serre_duality_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        cst("Scheme"),
        pi(
            BinderInfo::Default,
            "F",
            app(cst("CoherentSheaf"), bvar(0)),
            pi(
                BinderInfo::Default,
                "i",
                nat_ty(),
                app2(
                    cst("Equiv"),
                    app3(cst("Cohomology"), bvar(2), bvar(0), bvar(1)),
                    app(
                        cst("Dual"),
                        app3(
                            cst("Cohomology"),
                            bvar(2),
                            app2(cst("DualizeIndex"), app(cst("Dim"), bvar(2)), bvar(0)),
                            app2(
                                cst("TensorProduct"),
                                app(cst("DualSheaf"), bvar(2)),
                                bvar(1),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// `cohomology_long_exact_sequence_ty` — short exact sequence of sheaves induces long exact sequence.
///
/// `∀ (X : Scheme) (A B C : CoherentSheaf X),
///   ShortExact A B C → LongExactCohomology X A B C`
pub fn cohomology_long_exact_sequence_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        cst("Scheme"),
        pi(
            BinderInfo::Default,
            "A",
            app(cst("CoherentSheaf"), bvar(0)),
            pi(
                BinderInfo::Default,
                "B",
                app(cst("CoherentSheaf"), bvar(1)),
                pi(
                    BinderInfo::Default,
                    "C",
                    app(cst("CoherentSheaf"), bvar(2)),
                    arrow(
                        app3(cst("ShortExact"), bvar(2), bvar(1), bvar(0)),
                        app3(cst("LongExactCohomology"), bvar(3), bvar(2), bvar(1)),
                    ),
                ),
            ),
        ),
    )
}
/// `hilbert_nullstellensatz_ty` — Hilbert's Nullstellensatz: the radical ideal
/// of the variety corresponds to the vanishing ideal.
///
/// `∀ (k : Field) (I : Ideal k\[x₁,..,xₙ\]), Ideal(Variety(I)) = Radical(I)`
pub fn hilbert_nullstellensatz_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "k",
        cst("Field"),
        pi(
            BinderInfo::Default,
            "I",
            app(cst("Ideal"), bvar(0)),
            app2(
                cst("Eq"),
                app(cst("VanishingIdeal"), app(cst("AffineVarietyOf"), bvar(0))),
                app(cst("Radical"), bvar(0)),
            ),
        ),
    )
}
/// `weak_nullstellensatz_ty` — the weak Nullstellensatz: a system of polynomials
/// with no common zero has 1 in its ideal.
///
/// `∀ (k : AlgClosedField) (I : Ideal k\[x\]), Variety(I) = ∅ → 1 ∈ I`
pub fn weak_nullstellensatz_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "k",
        cst("AlgClosedField"),
        pi(
            BinderInfo::Default,
            "I",
            app(cst("Ideal"), bvar(0)),
            arrow(
                app2(
                    cst("Eq"),
                    app(cst("AffineVarietyOf"), bvar(0)),
                    cst("EmptySet"),
                ),
                app2(cst("IdealMem"), cst("One"), bvar(0)),
            ),
        ),
    )
}
/// `variety_ideal_correspondence_ty` — anti-isomorphism between radical ideals and
/// affine varieties over an algebraically closed field.
///
/// `∀ (k : AlgClosedField), RadicalIdeals k ≃ᵒᵖ AffineVarieties k`
pub fn variety_ideal_correspondence_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "k",
        cst("AlgClosedField"),
        app2(
            cst("AntiEquiv"),
            app(cst("RadicalIdeals"), bvar(0)),
            app(cst("AffineVarieties"), bvar(0)),
        ),
    )
}
/// `zariski_topology_ty` — Zariski closed sets form a topology.
///
/// `∀ (n : Nat) (k : AlgClosedField), IsTopology (ZariskiClosed (AffineN n k))`
pub fn zariski_topology_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "k",
            cst("AlgClosedField"),
            app(
                cst("IsTopology"),
                app(cst("ZariskiClosed"), app2(cst("AffineN"), bvar(1), bvar(0))),
            ),
        ),
    )
}
/// `projective_variety_irreducible_ty` — a projective variety is irreducible iff
/// its homogeneous ideal is prime.
///
/// `∀ (k : AlgClosedField) (I : HomogeneousIdeal k), Irreducible (Proj I) ↔ IsPrime I`
pub fn projective_variety_irreducible_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "k",
        cst("AlgClosedField"),
        pi(
            BinderInfo::Default,
            "I",
            app(cst("HomogeneousIdeal"), bvar(0)),
            app2(
                cst("Iff"),
                app(cst("Irreducible"), app(cst("Proj"), bvar(0))),
                app(cst("IsPrime"), bvar(0)),
            ),
        ),
    )
}
/// `rational_map_dominant_ty` — a rational map between irreducible varieties induces
/// a field extension of function fields.
///
/// `∀ (X Y : Scheme), DominantRationalMap X Y → FieldExt (FunctionField Y) (FunctionField X)`
pub fn rational_map_dominant_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        cst("Scheme"),
        pi(
            BinderInfo::Default,
            "Y",
            cst("Scheme"),
            arrow(
                app2(cst("DominantRationalMap"), bvar(1), bvar(0)),
                app2(
                    cst("FieldExt"),
                    app(cst("FunctionField"), bvar(0)),
                    app(cst("FunctionField"), bvar(1)),
                ),
            ),
        ),
    )
}
/// `ample_embedding_ty` — an ample line bundle gives a projective embedding.
///
/// `∀ (X : Scheme) (L : AmpLeLineBundle X), ∃ n : Nat, ClosedEmbedding X (ProjectiveSpace n)`
pub fn ample_embedding_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        cst("Scheme"),
        arrow(
            app(cst("AmpLeLineBundle"), bvar(0)),
            app2(
                cst("Exists"),
                nat_ty(),
                app2(
                    cst("ClosedEmbedding"),
                    bvar(1),
                    app(cst("ProjectiveSpace"), bvar(0)),
                ),
            ),
        ),
    )
}
/// `picard_group_is_group_ty` — the Picard group Pic(X) is an abelian group under
/// tensor product of line bundles.
///
/// `∀ (X : Scheme), IsAbelianGroup (PicardGroup X)`
pub fn picard_group_is_group_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        cst("Scheme"),
        app(cst("IsAbelianGroup"), app(cst("PicardGroup"), bvar(0))),
    )
}
/// `weil_cartier_correspondence_ty` — on a smooth variety, Weil divisors and
/// Cartier divisors coincide.
///
/// `∀ (X : Scheme), IsSmooth X → WeilDivisors X ≃ CartierDivisors X`
pub fn weil_cartier_correspondence_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        cst("Scheme"),
        arrow(
            app(cst("IsSmooth"), bvar(0)),
            app2(
                cst("Equiv"),
                app(cst("WeilDivisors"), bvar(0)),
                app(cst("CartierDivisors"), bvar(0)),
            ),
        ),
    )
}
/// `divisor_linear_system_dim_ty` — dimension of H^0(X, O(D)) equals the projective
/// dimension of the linear system |D| plus one.
///
/// `∀ (X : Scheme) (D : Divisor X), DimH0 (O X D) = LinearSystemDim (LS X D) + 1`
pub fn divisor_linear_system_dim_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        cst("Scheme"),
        pi(
            BinderInfo::Default,
            "D",
            app(cst("Divisor"), bvar(0)),
            app2(
                cst("Eq"),
                app(cst("DimH0"), app2(cst("TwistedSheaf"), bvar(1), bvar(0))),
                app2(
                    cst("Nat.add"),
                    app(
                        cst("LinearSystemDim"),
                        app2(cst("LinearSystem"), bvar(1), bvar(0)),
                    ),
                    cst("Nat.one"),
                ),
            ),
        ),
    )
}
/// `jacobian_abel_jacobi_ty` — the Abel-Jacobi map from the curve to its Jacobian.
///
/// `∀ (C : Scheme), IsSmooth C → IsCurve C → MorphismOfSchemes C (JacobianVariety C)`
pub fn jacobian_abel_jacobi_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "C",
        cst("Scheme"),
        arrow(
            app(cst("IsSmooth"), bvar(0)),
            arrow(
                app(cst("IsCurve"), bvar(0)),
                app2(
                    cst("MorphismOfSchemes"),
                    bvar(0),
                    app(cst("JacobianVariety"), bvar(0)),
                ),
            ),
        ),
    )
}
/// `elliptic_curve_group_law_ty` — the group law on an elliptic curve.
///
/// `∀ (k : Field) (E : EllipticCurve k), IsAbelianGroup (EllipticCurvePoint k E)`
pub fn elliptic_curve_group_law_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "k",
        cst("Field"),
        pi(
            BinderInfo::Default,
            "E",
            app(cst("EllipticCurve"), bvar(0)),
            app(
                cst("IsAbelianGroup"),
                app2(cst("EllipticCurvePoint"), bvar(1), bvar(0)),
            ),
        ),
    )
}
/// `mordell_weil_ty` — the Mordell-Weil theorem: E(K) is finitely generated for a
/// number field K.
///
/// `∀ (K : NumberField) (E : EllipticCurve K), IsFinitelyGenerated (EllipticCurvePoint K E)`
pub fn mordell_weil_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "K",
        cst("NumberField"),
        pi(
            BinderInfo::Default,
            "E",
            app(cst("EllipticCurve"), bvar(0)),
            app(
                cst("IsFinitelyGenerated"),
                app2(cst("EllipticCurvePoint"), bvar(1), bvar(0)),
            ),
        ),
    )
}
/// `torsion_subgroup_finite_ty` — the torsion subgroup of E(K) is finite.
///
/// `∀ (K : NumberField) (E : EllipticCurve K), IsFinite (TorsionPoints K E)`
pub fn torsion_subgroup_finite_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "K",
        cst("NumberField"),
        pi(
            BinderInfo::Default,
            "E",
            app(cst("EllipticCurve"), bvar(0)),
            app(
                cst("IsFinite"),
                app2(cst("TorsionPoints"), bvar(1), bvar(0)),
            ),
        ),
    )
}
/// `mazur_torsion_ty` — Mazur's theorem: the torsion subgroup of E(ℚ) is one of 15
/// possible groups.
///
/// `∀ (E : EllipticCurve Q), TorsionPoints Q E ∈ MazurList`
pub fn mazur_torsion_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "E",
        app(cst("EllipticCurve"), cst("Q")),
        app2(
            cst("ListMem"),
            app2(cst("TorsionPoints"), cst("Q"), bvar(0)),
            cst("MazurList"),
        ),
    )
}
/// `abelian_variety_group_ty` — every abelian variety is a commutative algebraic group.
///
/// `∀ (k : Field) (n : Nat) (A : AbelianVariety k n), IsCommAlgGroup A`
pub fn abelian_variety_group_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "k",
        cst("Field"),
        pi(
            BinderInfo::Default,
            "n",
            nat_ty(),
            pi(
                BinderInfo::Default,
                "A",
                app2(cst("AbelianVariety"), bvar(1), bvar(0)),
                app(cst("IsCommAlgGroup"), bvar(0)),
            ),
        ),
    )
}
/// `hodge_decomposition_ty` — the Hodge decomposition of de Rham cohomology.
///
/// `∀ (X : Scheme) (n : Nat), IsSmooth X → IsProjective X →
///   H^n_dR(X) ≅ ⊕_{p+q=n} H^{p,q}(X)`
pub fn hodge_decomposition_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        cst("Scheme"),
        pi(
            BinderInfo::Default,
            "n",
            nat_ty(),
            arrow(
                app(cst("IsSmooth"), bvar(1)),
                arrow(
                    app(cst("IsProjective"), bvar(1)),
                    app2(
                        cst("Equiv"),
                        app2(cst("DeRhamCohomology"), bvar(2), bvar(1)),
                        app2(cst("HodgeDecomposition"), bvar(2), bvar(1)),
                    ),
                ),
            ),
        ),
    )
}
/// `lefschetz_hyperplane_ty` — the Lefschetz hyperplane theorem.
///
/// `∀ (X : Scheme) (H : Hyperplane X), IsSmooth X → Dim X ≥ 3 →
///   π₁(H) ≅ π₁(X)`
pub fn lefschetz_hyperplane_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        cst("Scheme"),
        pi(
            BinderInfo::Default,
            "H",
            app(cst("Hyperplane"), bvar(0)),
            arrow(
                app(cst("IsSmooth"), bvar(1)),
                arrow(
                    app2(cst("Nat.le"), cst("Three"), app(cst("Dim"), bvar(1))),
                    app2(
                        cst("Equiv"),
                        app(cst("FundamentalGroup"), bvar(0)),
                        app(cst("FundamentalGroup"), bvar(1)),
                    ),
                ),
            ),
        ),
    )
}
/// `etale_cohomology_comparison_ty` — étale cohomology with ℚ_ℓ coefficients
/// agrees with Betti cohomology over ℂ.
///
/// `∀ (X : Scheme) (n : Nat), IsSmooth X → IsProjective X →
///   EtaleCohomology X n QEll ≅ BettiCohomology (X(ℂ)) n`
pub fn etale_cohomology_comparison_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        cst("Scheme"),
        pi(
            BinderInfo::Default,
            "n",
            nat_ty(),
            arrow(
                app(cst("IsSmooth"), bvar(1)),
                arrow(
                    app(cst("IsProjective"), bvar(1)),
                    app2(
                        cst("Equiv"),
                        app3(cst("EtaleCohomology"), bvar(2), bvar(1), cst("QEll")),
                        app2(
                            cst("BettiCohomology"),
                            app(cst("ComplexPoints"), bvar(2)),
                            bvar(1),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// `weil_conjectures_ty` — Deligne's proof of the Weil conjectures: the zeta function
/// of a smooth projective variety over 𝔽_q is rational, satisfies a functional equation,
/// and its roots have absolute value q^{w/2}.
///
/// `∀ (X : Scheme) (q : PrimePower), IsSmooth X → IsProjective X → IsOverFiniteField X q →
///   WeilConjecturesHold X q`
pub fn weil_conjectures_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        cst("Scheme"),
        pi(
            BinderInfo::Default,
            "q",
            cst("PrimePower"),
            arrow(
                app(cst("IsSmooth"), bvar(1)),
                arrow(
                    app(cst("IsProjective"), bvar(1)),
                    arrow(
                        app2(cst("IsOverFiniteField"), bvar(1), bvar(0)),
                        app2(cst("WeilConjecturesHold"), bvar(2), bvar(1)),
                    ),
                ),
            ),
        ),
    )
}
/// `zeta_function_rational_ty` — the zeta function of a smooth projective variety
/// over a finite field is a rational function in t.
///
/// `∀ (X : Scheme) (q : PrimePower), IsOverFiniteField X q →
///   IsRational (ZetaFunction X)`
pub fn zeta_function_rational_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        cst("Scheme"),
        pi(
            BinderInfo::Default,
            "q",
            cst("PrimePower"),
            arrow(
                app2(cst("IsOverFiniteField"), bvar(1), bvar(0)),
                app(cst("IsRational"), app(cst("ZetaFunction"), bvar(1))),
            ),
        ),
    )
}
/// `structure_sheaf_is_sheaf_ty` — the structure sheaf O_X satisfies the sheaf axioms.
///
/// `∀ (X : Scheme), IsSheaf (StructureSheaf X)`
pub fn structure_sheaf_is_sheaf_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        cst("Scheme"),
        app(cst("IsSheaf"), app(cst("StructureSheaf"), bvar(0))),
    )
}
/// `quasi_coherent_localization_ty` — quasi-coherent sheaves on Spec(R) correspond
/// to R-modules.
///
/// `∀ (R : CommRing), QuasiCoherentSheaf (Spec R) ≃ RMod R`
pub fn quasi_coherent_localization_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "R",
        cst("CommRing"),
        app2(
            cst("Equiv"),
            app(cst("QuasiCoherentSheaf"), app(cst("Spec"), bvar(0))),
            app(cst("RMod"), bvar(0)),
        ),
    )
}
/// `coherent_sheaf_noetherian_ty` — on a Noetherian scheme, every quasi-coherent
/// sheaf of finite type is coherent.
///
/// `∀ (X : Scheme), IsNoetherian X → FiniteTypeQCoh X ⊆ CoherentSheaf X`
pub fn coherent_sheaf_noetherian_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        cst("Scheme"),
        arrow(
            app(cst("IsNoetherian"), bvar(0)),
            app2(
                cst("Subset"),
                app(cst("FiniteTypeQCoh"), bvar(0)),
                app(cst("CoherentSheaf"), bvar(0)),
            ),
        ),
    )
}
/// `base_change_cohomology_ty` — flat base change for cohomology of quasi-coherent sheaves.
///
/// `∀ (X Y : Scheme) (f : MorphismOfSchemes X Y) (F : QuasiCoherentSheaf X),
///   IsFlat f → Cohomology Y (Pushforward f F) ≃ Cohomology X F`
pub fn base_change_cohomology_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        cst("Scheme"),
        pi(
            BinderInfo::Default,
            "Y",
            cst("Scheme"),
            pi(
                BinderInfo::Default,
                "f",
                app2(cst("MorphismOfSchemes"), bvar(1), bvar(0)),
                pi(
                    BinderInfo::Default,
                    "F",
                    app(cst("QuasiCoherentSheaf"), bvar(2)),
                    arrow(
                        app(cst("IsFlat"), bvar(1)),
                        app2(
                            cst("Equiv"),
                            app2(
                                cst("CohomologyOf"),
                                bvar(3),
                                app2(cst("Pushforward"), bvar(1), bvar(0)),
                            ),
                            app2(cst("CohomologyOf"), bvar(4), bvar(0)),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// `grothendieck_vanishing_ty` — Grothendieck's vanishing theorem: H^i(X, F) = 0 for i > dim X.
///
/// `∀ (X : Scheme) (F : CoherentSheaf X) (i : Nat),
///   Nat.lt (Dim X) i → Cohomology X i F = 0`
pub fn grothendieck_vanishing_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        cst("Scheme"),
        pi(
            BinderInfo::Default,
            "F",
            app(cst("CoherentSheaf"), bvar(0)),
            pi(
                BinderInfo::Default,
                "i",
                nat_ty(),
                arrow(
                    app2(cst("Nat.lt"), app(cst("Dim"), bvar(2)), bvar(0)),
                    app2(
                        cst("Eq"),
                        app3(cst("Cohomology"), bvar(3), bvar(1), bvar(2)),
                        cst("ZeroModule"),
                    ),
                ),
            ),
        ),
    )
}
/// Register all algebraic geometry type builders and theorem types in the kernel environment.
pub fn build_algebraic_geometry_env(env: &mut Environment) {
    let base_types: &[(&str, Expr)] = &[
        ("Scheme", type1()),
        ("CommRing", type0()),
        (
            "RingHom",
            arrow(cst("CommRing"), arrow(cst("CommRing"), type0())),
        ),
        ("Spec", affine_scheme_ty()),
        ("AllAffine", arrow(list_ty(cst("Scheme")), prop())),
        (
            "CoversScheme",
            arrow(list_ty(cst("Scheme")), arrow(cst("Scheme"), prop())),
        ),
        ("Equiv", arrow(type0(), arrow(type0(), prop()))),
        ("Eq", arrow(type0(), arrow(type0(), prop()))),
        ("And", arrow(prop(), arrow(prop(), prop()))),
        ("Exists", arrow(type0(), arrow(type0(), prop()))),
        ("Dim", arrow(cst("Scheme"), nat_ty())),
        ("Genus", arrow(cst("Scheme"), int_ty())),
        ("H0", arrow(type0(), int_ty())),
        ("H1", arrow(type0(), int_ty())),
        ("DivisorDeg", arrow(type0(), int_ty())),
        ("Int.sub", arrow(int_ty(), arrow(int_ty(), int_ty()))),
        ("Int.one", int_ty()),
        ("Dual", arrow(type0(), type0())),
        ("DualSheaf", arrow(cst("Scheme"), type0())),
        ("DualizeIndex", arrow(nat_ty(), arrow(nat_ty(), nat_ty()))),
        (
            "Cohomology",
            arrow(cst("Scheme"), arrow(nat_ty(), arrow(type0(), type0()))),
        ),
        ("TensorProduct", arrow(type0(), arrow(type0(), type0()))),
        (
            "ShortExact",
            arrow(type0(), arrow(type0(), arrow(type0(), prop()))),
        ),
        (
            "LongExactCohomology",
            arrow(cst("Scheme"), arrow(type0(), arrow(type0(), prop()))),
        ),
        ("Field", type0()),
        ("AlgClosedField", type0()),
        ("Ideal", arrow(type0(), type0())),
        ("AffineVarietyOf", arrow(type0(), type0())),
        ("VanishingIdeal", arrow(type0(), type0())),
        ("Radical", arrow(type0(), type0())),
        ("EmptySet", type0()),
        ("One", type0()),
        ("IdealMem", arrow(type0(), arrow(type0(), prop()))),
        ("AntiEquiv", arrow(type0(), arrow(type0(), prop()))),
        ("RadicalIdeals", arrow(type0(), type0())),
        ("AffineVarieties", arrow(type0(), type0())),
        ("IsTopology", arrow(type0(), prop())),
        (
            "AffineN",
            arrow(nat_ty(), arrow(cst("AlgClosedField"), cst("Scheme"))),
        ),
        ("Proj", arrow(type0(), cst("Scheme"))),
        ("Irreducible", arrow(cst("Scheme"), prop())),
        ("IsPrime", arrow(type0(), prop())),
        ("Iff", arrow(prop(), arrow(prop(), prop()))),
        (
            "DominantRationalMap",
            arrow(cst("Scheme"), arrow(cst("Scheme"), prop())),
        ),
        ("FunctionField", arrow(cst("Scheme"), type0())),
        ("FieldExt", arrow(type0(), arrow(type0(), prop()))),
        (
            "ClosedEmbedding",
            arrow(cst("Scheme"), arrow(cst("Scheme"), prop())),
        ),
        ("IsAbelianGroup", arrow(type0(), prop())),
        ("WeilDivisors", arrow(cst("Scheme"), type0())),
        ("CartierDivisors", arrow(cst("Scheme"), type0())),
        ("IsSmooth", arrow(cst("Scheme"), prop())),
        ("DimH0", arrow(type0(), nat_ty())),
        (
            "TwistedSheaf",
            arrow(
                cst("Scheme"),
                arrow(app(cst("Divisor"), cst("Scheme")), type0()),
            ),
        ),
        ("LinearSystemDim", arrow(type0(), nat_ty())),
        ("Nat.add", arrow(nat_ty(), arrow(nat_ty(), nat_ty()))),
        ("Nat.one", nat_ty()),
        ("IsCurve", arrow(cst("Scheme"), prop())),
        ("IsFinitelyGenerated", arrow(type0(), prop())),
        ("NumberField", type0()),
        ("IsFinite", arrow(type0(), prop())),
        ("TorsionPoints", arrow(type0(), arrow(type0(), type0()))),
        ("Q", type0()),
        ("MazurList", list_ty(type0())),
        ("ListMem", arrow(type0(), arrow(list_ty(type0()), prop()))),
        ("IsCommAlgGroup", arrow(type0(), prop())),
        (
            "DeRhamCohomology",
            arrow(cst("Scheme"), arrow(nat_ty(), type0())),
        ),
        (
            "HodgeDecomposition",
            arrow(cst("Scheme"), arrow(nat_ty(), type0())),
        ),
        ("IsProjective", arrow(cst("Scheme"), prop())),
        ("Hyperplane", arrow(cst("Scheme"), type0())),
        ("FundamentalGroup", arrow(cst("Scheme"), type0())),
        ("Nat.le", arrow(nat_ty(), arrow(nat_ty(), prop()))),
        ("Three", nat_ty()),
        ("QEll", cst("CommRing")),
        ("BettiCohomology", arrow(type0(), arrow(nat_ty(), type0()))),
        ("ComplexPoints", arrow(cst("Scheme"), type0())),
        ("PrimePower", type0()),
        (
            "IsOverFiniteField",
            arrow(cst("Scheme"), arrow(cst("PrimePower"), prop())),
        ),
        (
            "WeilConjecturesHold",
            arrow(cst("Scheme"), arrow(cst("PrimePower"), prop())),
        ),
        ("IsRational", arrow(type0(), prop())),
        ("IsSheaf", arrow(type0(), prop())),
        ("RMod", arrow(cst("CommRing"), type0())),
        ("IsNoetherian", arrow(cst("Scheme"), prop())),
        ("FiniteTypeQCoh", arrow(cst("Scheme"), type0())),
        ("Subset", arrow(type0(), arrow(type0(), prop()))),
        ("IsFlat", arrow(type0(), prop())),
        ("Pushforward", arrow(type0(), arrow(type0(), type0()))),
        (
            "CohomologyOf",
            arrow(cst("Scheme"), arrow(type0(), type0())),
        ),
        ("Nat.lt", arrow(nat_ty(), arrow(nat_ty(), prop()))),
        ("ZeroModule", type0()),
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
        ("RingOfFunctions", ring_of_functions_ty),
        ("Sheaf", sheaf_ty),
        ("AffineScheme", affine_scheme_ty),
        ("ProjectiveSpace", projective_space_ty),
        ("MorphismOfSchemes", morphism_of_schemes_ty),
        ("CoherentSheaf", coherent_sheaf_ty),
        ("Divisor", divisor_ty),
        ("LineBundle", line_bundle_ty),
        ("AffineVariety", affine_variety_ty),
        ("ZariskiOpen", zariski_open_ty),
        ("ZariskiClosed", zariski_closed_ty),
        ("HomogeneousIdeal", homogeneous_ideal_ty),
        ("ProjectiveVariety", projective_variety_ty),
        ("RationalMap", rational_map_ty),
        ("StructureSheaf", structure_sheaf_ty),
        ("QuasiCoherentSheaf", quasi_coherent_sheaf_ty),
        ("AmpLeLineBundle", ample_line_bundle_ty),
        ("PicardGroup", picard_group_ty),
        ("CartierDivisor", cartier_divisor_ty),
        ("LinearSystem", linear_system_ty),
        ("BaseLocus", base_locus_ty),
        ("JacobianVariety", jacobian_variety_ty),
        ("EllipticCurve", elliptic_curve_ty),
        ("EllipticCurvePoint", elliptic_curve_point_ty),
        ("TorsionSubgroup", torsion_subgroup_ty),
        ("AbelianVariety", abelian_variety_ty),
        ("HodgeNumber", hodge_number_ty),
        ("EtaleSite", etale_site_ty),
        ("EtaleCohomology", etale_cohomology_ty),
        ("WeilConjectureData", weil_conjecture_data_ty),
        ("ZetaFunction", zeta_function_ty),
        ("SheafCohomology", sheaf_cohomology_ty),
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
        ("spec_functor", spec_functor_ty),
        ("affine_cover", affine_cover_ty),
        ("yoneda_schemes", yoneda_schemes_ty),
        ("riemann_roch", riemann_roch_ty),
        ("serre_duality", serre_duality_ty),
        (
            "cohomology_long_exact_sequence",
            cohomology_long_exact_sequence_ty,
        ),
        ("hilbert_nullstellensatz", hilbert_nullstellensatz_ty),
        ("weak_nullstellensatz", weak_nullstellensatz_ty),
        (
            "variety_ideal_correspondence",
            variety_ideal_correspondence_ty,
        ),
        ("zariski_topology", zariski_topology_ty),
        (
            "projective_variety_irreducible",
            projective_variety_irreducible_ty,
        ),
        ("rational_map_dominant", rational_map_dominant_ty),
        ("ample_embedding", ample_embedding_ty),
        ("picard_group_is_group", picard_group_is_group_ty),
        (
            "weil_cartier_correspondence",
            weil_cartier_correspondence_ty,
        ),
        ("divisor_linear_system_dim", divisor_linear_system_dim_ty),
        ("jacobian_abel_jacobi", jacobian_abel_jacobi_ty),
        ("elliptic_curve_group_law", elliptic_curve_group_law_ty),
        ("mordell_weil", mordell_weil_ty),
        ("torsion_subgroup_finite", torsion_subgroup_finite_ty),
        ("mazur_torsion", mazur_torsion_ty),
        ("abelian_variety_group", abelian_variety_group_ty),
        ("hodge_decomposition", hodge_decomposition_ty),
        ("lefschetz_hyperplane", lefschetz_hyperplane_ty),
        (
            "etale_cohomology_comparison",
            etale_cohomology_comparison_ty,
        ),
        ("weil_conjectures", weil_conjectures_ty),
        ("zeta_function_rational", zeta_function_rational_ty),
        ("structure_sheaf_is_sheaf", structure_sheaf_is_sheaf_ty),
        (
            "quasi_coherent_localization",
            quasi_coherent_localization_ty,
        ),
        ("coherent_sheaf_noetherian", coherent_sheaf_noetherian_ty),
        ("base_change_cohomology", base_change_cohomology_ty),
        ("grothendieck_vanishing", grothendieck_vanishing_ty),
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
/// Compute the rank of a coherent sheaf given by a list of local ranks.
///
/// For a locally free sheaf, all local ranks are equal; this function
/// returns `None` if the ranks are inconsistent (not all equal).
pub fn coherent_sheaf_rank(local_ranks: &[usize]) -> Option<usize> {
    if local_ranks.is_empty() {
        return None;
    }
    let first = local_ranks[0];
    if local_ranks.iter().all(|&r| r == first) {
        Some(first)
    } else {
        None
    }
}
/// Compute the degree of a divisor given by a list of (prime divisor, multiplicity) pairs.
///
/// The degree is the sum of all multiplicities.
pub fn divisor_degree(components: &[(&str, i64)]) -> i64 {
    components.iter().map(|(_, mult)| mult).sum()
}
/// Compute the topological Euler characteristic χ(X) = Σ (-1)^i dim H^i(X, ℤ).
///
/// Input: `betti_numbers\[i\]` = dim H^i(X, ℤ).
pub fn euler_characteristic(betti_numbers: &[u64]) -> i64 {
    betti_numbers
        .iter()
        .enumerate()
        .map(|(i, &b)| if i % 2 == 0 { b as i64 } else { -(b as i64) })
        .sum()
}
/// Riemann-Roch formula for curves: h^0(D) - h^1(D) = deg(D) - g + 1.
///
/// Given `deg_d` (degree of divisor) and `genus` (geometric genus of the curve),
/// returns the right-hand side `deg(D) - g + 1`.
pub fn riemann_roch_rhs(deg_d: i64, genus: i64) -> i64 {
    deg_d - genus + 1
}
/// Check if a divisor of given degree satisfies the conditions for global sections.
///
/// By Riemann-Roch, `h^0(D) ≥ deg(D) - g + 1` when `deg(D) > 2g - 2`.
pub fn has_global_sections(deg_d: i64, genus: i64) -> bool {
    deg_d > 2 * genus - 2
}
/// Euclidean GCD helper.
pub fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}
/// Compute the Riemann-Roch dimension h^0(D) for a divisor of degree d on a curve
/// of genus g, using the Riemann-Roch theorem and Serre duality.
///
/// - If deg(D) > 2g - 2: h^0(D) = deg(D) - g + 1 (no h^1 contribution).
/// - If deg(D) < 0:       h^0(D) = 0.
/// - Otherwise:           returns the RR lower bound (actual value may be higher).
pub fn riemann_roch_dim(deg_d: i64, genus: i64) -> i64 {
    if deg_d < 0 {
        0
    } else if deg_d > 2 * genus - 2 {
        deg_d - genus + 1
    } else {
        (deg_d - genus + 1).max(0)
    }
}
/// Count lattice points in a closed interval \[a, b\].
///
/// Used as a building block for Ehrhart polynomial computations in the context of
/// toric varieties and Newton-Okounkov bodies.
pub fn count_lattice_points_interval(a: i64, b: i64) -> i64 {
    if b < a {
        0
    } else {
        b - a + 1
    }
}
/// Compute the Hasse-Weil zeta function values at small integers for an elliptic curve
/// over F_p.
///
/// The zeta function is Z(E/F_p, T) = exp(Σ_{n≥1} #E(F_{p^n}) T^n / n).
/// Here we return the count #E(F_p) = p + 1 - a_p where a_p is the trace of Frobenius.
pub fn hasse_weil_trace_of_frobenius(curve: &EllipticCurveF) -> i64 {
    let np = curve.point_count() as i64;
    curve.p + 1 - np
}
/// Hasse's theorem bound: |a_p| ≤ 2√p.
/// Returns `true` if the trace of Frobenius satisfies Hasse's bound.
pub fn satisfies_hasse_bound(curve: &EllipticCurveF) -> bool {
    let ap = hasse_weil_trace_of_frobenius(curve).abs();
    let bound_sq = 4 * curve.p;
    ap * ap <= bound_sq
}
pub fn mod_reduce(x: i64, p: i64) -> i64 {
    ((x % p) + p) % p
}
pub fn mod_mul(a: i64, b: i64, p: i64) -> i64 {
    mod_reduce(a.wrapping_mul(b), p)
}
pub fn pow_mod(mut base: i64, mut exp: i64, modulus: i64) -> i64 {
    let mut result = 1i64;
    base = mod_reduce(base, modulus);
    while exp > 0 {
        if exp & 1 == 1 {
            result = mod_mul(result, base, modulus);
        }
        base = mod_mul(base, base, modulus);
        exp >>= 1;
    }
    result
}
/// Modular inverse via Fermat's little theorem (p must be prime).
pub fn mod_inv(a: i64, p: i64) -> i64 {
    pow_mod(a, p - 2, p)
}
#[cfg(test)]
mod tests {
    use super::*;
    use oxilean_kernel::Environment;
    #[test]
    fn test_affine_scheme_display() {
        let s = AffineScheme::new("k[x, y]");
        assert_eq!(s.to_string(), "Spec(k[x, y])");
    }
    #[test]
    fn test_affine_scheme_krull_dim() {
        let an = AffineScheme::affine_n_space("k", 3);
        assert_eq!(an.krull_dim_estimate(), Some(3));
        assert_eq!(AffineScheme::spec_z().krull_dim_estimate(), Some(1));
    }
    #[test]
    fn test_projective_space_properties() {
        let p2 = ProjectiveSpace::projective_plane();
        assert_eq!(p2.dim, 2);
        assert_eq!(p2.num_coordinates(), 3);
        assert_eq!(p2.euler_characteristic(), 3);
    }
    #[test]
    fn test_betti_numbers_projective_space() {
        let p3 = ProjectiveSpace::new(3);
        let betti = p3.betti_numbers();
        assert_eq!(betti, vec![1, 0, 1, 0, 1, 0, 1]);
        assert_eq!(euler_characteristic(&betti), 4);
    }
    #[test]
    fn test_morphism_compose() {
        let f = Morphism::new("A", "B");
        let g = Morphism::new("B", "C");
        let gf = f.compose(&g).expect("compose should succeed");
        assert_eq!(gf.source, "A");
        assert_eq!(gf.target, "C");
        let h = Morphism::new("D", "E");
        assert!(f.compose(&h).is_none());
    }
    #[test]
    fn test_divisor_degree() {
        let d = [("P", 2i64), ("Q", -1i64), ("R", 3i64)];
        assert_eq!(divisor_degree(&d), 4);
    }
    #[test]
    fn test_riemann_roch_rhs() {
        assert_eq!(riemann_roch_rhs(3, 1), 3);
        assert_eq!(riemann_roch_rhs(2, 0), 3);
    }
    #[test]
    fn test_build_algebraic_geometry_env() {
        let mut env = Environment::new();
        build_algebraic_geometry_env(&mut env);
        assert!(env.get(&Name::str("Scheme")).is_some());
        assert!(env.get(&Name::str("Divisor")).is_some());
        assert!(env.get(&Name::str("riemann_roch")).is_some());
        assert!(env.get(&Name::str("spec_functor")).is_some());
        assert!(env.get(&Name::str("serre_duality")).is_some());
        assert!(env
            .get(&Name::str("cohomology_long_exact_sequence"))
            .is_some());
    }
    #[test]
    fn test_build_env_new_axioms() {
        let mut env = Environment::new();
        build_algebraic_geometry_env(&mut env);
        assert!(env.get(&Name::str("AffineVariety")).is_some());
        assert!(env.get(&Name::str("ZariskiOpen")).is_some());
        assert!(env.get(&Name::str("HomogeneousIdeal")).is_some());
        assert!(env.get(&Name::str("ProjectiveVariety")).is_some());
        assert!(env.get(&Name::str("RationalMap")).is_some());
        assert!(env.get(&Name::str("StructureSheaf")).is_some());
        assert!(env.get(&Name::str("QuasiCoherentSheaf")).is_some());
        assert!(env.get(&Name::str("AmpLeLineBundle")).is_some());
        assert!(env.get(&Name::str("PicardGroup")).is_some());
        assert!(env.get(&Name::str("CartierDivisor")).is_some());
        assert!(env.get(&Name::str("LinearSystem")).is_some());
        assert!(env.get(&Name::str("JacobianVariety")).is_some());
        assert!(env.get(&Name::str("EllipticCurve")).is_some());
        assert!(env.get(&Name::str("AbelianVariety")).is_some());
        assert!(env.get(&Name::str("EtaleCohomology")).is_some());
        assert!(env.get(&Name::str("ZetaFunction")).is_some());
        assert!(env.get(&Name::str("hilbert_nullstellensatz")).is_some());
        assert!(env.get(&Name::str("weak_nullstellensatz")).is_some());
        assert!(env
            .get(&Name::str("variety_ideal_correspondence"))
            .is_some());
        assert!(env.get(&Name::str("zariski_topology")).is_some());
        assert!(env.get(&Name::str("elliptic_curve_group_law")).is_some());
        assert!(env.get(&Name::str("mordell_weil")).is_some());
        assert!(env.get(&Name::str("hodge_decomposition")).is_some());
        assert!(env.get(&Name::str("weil_conjectures")).is_some());
        assert!(env.get(&Name::str("grothendieck_vanishing")).is_some());
    }
    #[test]
    fn test_affine_variety() {
        let v = AffineVariety::new(2, vec!["x^2 + y^2 - 1".to_string()]);
        assert_eq!(v.ambient_dim, 2);
        assert_eq!(v.num_equations(), 1);
        assert_eq!(v.dimension_estimate(), 1);
        assert!(!v.is_empty_variety());
        assert!(!v.is_full_space());
        let empty = AffineVariety::empty(3);
        assert!(empty.is_empty_variety());
        let full = AffineVariety::affine_space(4);
        assert!(full.is_full_space());
    }
    #[test]
    fn test_projective_point() {
        let p = ProjectivePoint::new(vec![1, 2, 3]).expect("ProjectivePoint::new should succeed");
        assert_eq!(p.dim(), 2);
        let q = ProjectivePoint::new(vec![2, 4, 6]).expect("ProjectivePoint::new should succeed");
        assert!(p.equiv(&q));
        let r = ProjectivePoint::new(vec![1, 0, 0]).expect("ProjectivePoint::new should succeed");
        let s = ProjectivePoint::new(vec![0, 1, 0]).expect("ProjectivePoint::new should succeed");
        assert!(!r.equiv(&s));
        assert!(ProjectivePoint::new(vec![0, 0, 0]).is_none());
    }
    #[test]
    fn test_projective_point_normalize() {
        let p = ProjectivePoint::new(vec![4, -6, 2]).expect("ProjectivePoint::new should succeed");
        let n = p.normalize();
        assert_eq!(n.coords, vec![2, -3, 1]);
    }
    #[test]
    fn test_elliptic_curve_on_curve() {
        let e = EllipticCurveF::new(6, 0, 7);
        assert!(e.on_curve(0, 0));
        assert!(e.on_curve(1, 0));
        assert!(!e.on_curve(2, 1));
    }
    #[test]
    fn test_elliptic_curve_group_law() {
        let e = EllipticCurveF::new(1, 1, 5);
        let pts = e.points();
        let p = &pts[1];
        let sum = e.add(&EllipticCurvePoint::Infinity, p);
        assert_eq!(&sum, p);
        if let Some(ord) = e.point_order(p, 100) {
            let result = e.scalar_mul(ord, p);
            assert_eq!(result, EllipticCurvePoint::Infinity);
        }
    }
    #[test]
    fn test_hasse_bound() {
        let e = EllipticCurveF::new(1, 6, 7);
        assert!(satisfies_hasse_bound(&e));
    }
    #[test]
    fn test_divisor_class_arithmetic() {
        let d1 = DivisorClass::new(3, 2, "D1");
        let d2 = DivisorClass::new(-1, 2, "D2");
        let sum = d1.add(&d2);
        assert_eq!(sum.degree, 2);
        let canonical = DivisorClass::canonical(2);
        assert_eq!(canonical.degree, 2);
        assert!(canonical.is_canonical());
        let zero = DivisorClass::zero(2);
        assert_eq!(zero.degree, 0);
        assert!(zero.is_effective());
    }
    #[test]
    fn test_riemann_roch_dim() {
        assert_eq!(riemann_roch_dim(3, 1), 3);
        assert_eq!(riemann_roch_dim(-1, 1), 0);
        assert_eq!(riemann_roch_dim(0, 1), 0);
        assert_eq!(riemann_roch_dim(2, 0), 3);
        assert_eq!(riemann_roch_dim(5, 3), 3);
    }
    #[test]
    fn test_count_lattice_points() {
        assert_eq!(count_lattice_points_interval(0, 5), 6);
        assert_eq!(count_lattice_points_interval(-3, 3), 7);
        assert_eq!(count_lattice_points_interval(5, 3), 0);
    }
}
#[cfg(test)]
mod extended_alggeom_tests {
    use super::*;
    #[test]
    fn test_projective_variety() {
        let p2 = ProjectiveVariety::projective_space(2);
        assert_eq!(p2.degree, 1);
        assert!(p2.is_smooth);
        let h = ProjectiveVariety::hypersurface(3, 4);
        assert_eq!(h.bezout_bound(3), 12);
    }
    #[test]
    fn test_affine_scheme() {
        let a3 = AffineSchemeData::affine_space(3);
        assert_eq!(a3.dimension, Some(3));
        assert!(a3.is_variety());
    }
    #[test]
    fn test_divisor() {
        let d = Divisor::prime("H");
        assert_eq!(d.degree(), 1);
        assert!(d.is_prime);
        assert!(d.linear_equiv_description().contains("degree 1"));
    }
    #[test]
    fn test_chern_class() {
        let c = ChernClass::new("E", 3);
        assert_eq!(c.classes.len(), 4);
        assert!(c.grr_applies());
        assert!(c.total_chern_class().contains("c_0"));
    }
    #[test]
    fn test_moduli() {
        let m = ModuliProblem::elliptic_curves();
        assert!(!m.has_fine_moduli);
        assert!(m.fine_moduli_description().contains("coarse"));
    }
}
