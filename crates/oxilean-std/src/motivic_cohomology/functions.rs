//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{
    AdamsOperation, AlgebraicCycle, AlgebraicKGroup, BlochFormula, BlochKatoConjecture, ChowGroup,
    ChowRing, FormalGroupLaw, GerstenResolution, HigherChowGroup, KTheorySpectrum, LAdicCohomology,
    MilnorConjecture, MilnorKTheory, MixedMotive, MotivicCohomology, MotivicComplex,
    MotivicFunctor, MotivicSphere, PureMotive, RationalEquivalence, RealizationFunctor,
    RealizationType, ReducedPowerOperation, VoevodskysProof, WeilConjectures,
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
pub fn bool_ty() -> Expr {
    cst("Bool")
}
pub fn list_ty(elem: Expr) -> Expr {
    app(cst("List"), elem)
}
pub fn option_ty(elem: Expr) -> Expr {
    app(cst("Option"), elem)
}
pub fn pair_ty(a: Expr, b: Expr) -> Expr {
    app2(cst("Prod"), a, b)
}
/// `AlgebraicKGroup : CommRing → Nat → AbGroup`
/// K_n(R) is the n-th algebraic K-group of the ring R.
pub fn algebraic_k_group_ty() -> Expr {
    arrow(cst("CommRing"), arrow(nat_ty(), cst("AbGroup")))
}
/// `KTheorySpectrum : CommRing → Spectrum`
/// K(R) is the connective K-theory spectrum of R.
pub fn k_theory_spectrum_ty() -> Expr {
    arrow(cst("CommRing"), cst("Spectrum"))
}
/// `GerstenResolution : Scheme → Nat → Type`
/// The Gersten complex 0 → K_n(R) → ⊕_{x∈X^0} K_n(k(x)) → ⊕_{x∈X^1} K_{n-1}(k(x)) → ...
pub fn gersten_resolution_ty() -> Expr {
    arrow(cst("Scheme"), arrow(nat_ty(), type0()))
}
/// `BassNegativeKGroup : CommRing → Int → AbGroup`
/// Bass's negative K-groups K_{-n}(R) extending Quillen's theory.
pub fn bass_negative_k_group_ty() -> Expr {
    arrow(cst("CommRing"), arrow(int_ty(), cst("AbGroup")))
}
/// Gersten's conjecture: the Gersten complex is exact for regular local rings.
///
/// `∀ (X : Scheme), IsRegular X → IsExact (GerstenComplex X)`
pub fn gersten_conjecture_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        cst("Scheme"),
        arrow(
            app(cst("IsRegular"), bvar(0)),
            app(cst("IsExact"), app(cst("GerstenComplex"), bvar(1))),
        ),
    )
}
/// Quillen's localization sequence for K-theory.
///
/// `∀ (X : Scheme) (Z : ClosedSubscheme X), LongExactSeq (K_*(Z)) (K_*(X)) (K_*(X \ Z))`
pub fn quillen_localization_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        cst("Scheme"),
        pi(
            BinderInfo::Default,
            "Z",
            app(cst("ClosedSubscheme"), bvar(0)),
            app3(
                cst("LongExactSeq"),
                app(cst("KTheory"), bvar(1)),
                app(cst("KTheory"), bvar(0)),
                app2(
                    cst("KTheory"),
                    app2(cst("SchemeComplement"), bvar(1), bvar(0)),
                    cst("Z"),
                ),
            ),
        ),
    )
}
/// Bass's fundamental theorem: K_n(R\[t, t^{-1}\]) ≅ K_n(R) ⊕ K_{n-1}(R).
///
/// `∀ (R : CommRing) (n : Nat), Iso (K R\[t,t^{-1}\] n) (Prod (K R n) (K R (n-1)))`
pub fn bass_fundamental_theorem_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "R",
        cst("CommRing"),
        pi(
            BinderInfo::Default,
            "n",
            nat_ty(),
            app2(
                cst("Iso"),
                app2(
                    cst("AlgebraicKGroup"),
                    app(cst("LaurentPoly"), bvar(1)),
                    bvar(0),
                ),
                app2(
                    cst("Prod"),
                    app2(cst("AlgebraicKGroup"), bvar(2), bvar(1)),
                    app2(
                        cst("AlgebraicKGroup"),
                        bvar(2),
                        app(cst("Nat.pred"), bvar(1)),
                    ),
                ),
            ),
        ),
    )
}
/// `ChowGroup : Scheme → Nat → AbGroup`
/// CH^p(X) = codimension p cycles modulo rational equivalence.
pub fn chow_group_ty() -> Expr {
    arrow(cst("Scheme"), arrow(nat_ty(), cst("AbGroup")))
}
/// `ChowRing : Scheme → CommRing`
/// CH^*(X) = ⊕_p CH^p(X) with the intersection product (for smooth X).
pub fn chow_ring_ty() -> Expr {
    arrow(cst("Scheme"), cst("CommRing"))
}
/// `AlgebraicCycle : Scheme → Nat → Type`
/// A formal ℤ-linear combination of irreducible subvarieties of codimension p.
pub fn algebraic_cycle_ty() -> Expr {
    arrow(cst("Scheme"), arrow(nat_ty(), type0()))
}
/// `RationalEquivalence : Scheme → Nat → AlgebraicCycle → AlgebraicCycle → Prop`
/// Two cycles are rationally equivalent if their difference is a divisor of a rational function on ℙ¹.
pub fn rational_equivalence_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        cst("Scheme"),
        pi(
            BinderInfo::Default,
            "p",
            nat_ty(),
            arrow(
                app2(cst("AlgebraicCycle"), bvar(1), bvar(0)),
                arrow(app2(cst("AlgebraicCycle"), bvar(2), bvar(1)), prop()),
            ),
        ),
    )
}
/// Push-forward of Chow groups along a proper morphism.
///
/// `∀ (X Y : Scheme) (f : ProperMorphism X Y) (p : Nat), CH^p(X) → CH^p(Y)`
pub fn chow_push_forward_ty() -> Expr {
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
                app2(cst("ProperMorphism"), bvar(1), bvar(0)),
                pi(
                    BinderInfo::Default,
                    "p",
                    nat_ty(),
                    arrow(
                        app2(cst("ChowGroup"), bvar(3), bvar(0)),
                        app2(cst("ChowGroup"), bvar(2), bvar(1)),
                    ),
                ),
            ),
        ),
    )
}
/// Pull-back of Chow groups along a flat morphism of relative dimension d.
///
/// `∀ (X Y : Scheme) (f : FlatMorphism X Y) (p : Nat), CH^p(Y) → CH^{p+d}(X)`
pub fn chow_pull_back_ty() -> Expr {
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
                app2(cst("FlatMorphism"), bvar(1), bvar(0)),
                pi(
                    BinderInfo::Default,
                    "p",
                    nat_ty(),
                    arrow(
                        app2(cst("ChowGroup"), bvar(2), bvar(0)),
                        app2(
                            cst("ChowGroup"),
                            bvar(3),
                            app(cst("ChowCodimShift"), bvar(1)),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// `MotivicCohomology : Scheme → Nat → Nat → AbGroup`
/// H^{p,q}(X, ℤ) — motivic cohomology with bigrading (p = cohom degree, q = weight).
pub fn motivic_cohomology_ty() -> Expr {
    arrow(
        cst("Scheme"),
        arrow(nat_ty(), arrow(nat_ty(), cst("AbGroup"))),
    )
}
/// `MotivicComplex : Scheme → Nat → ChainComplex`
/// ℤ(n) is Bloch's cycle complex on X, whose cohomology is motivic cohomology.
pub fn motivic_complex_ty() -> Expr {
    arrow(cst("Scheme"), arrow(nat_ty(), cst("ChainComplex")))
}
/// `MilnorKTheory : Field → Nat → AbGroup`
/// K_n^M(F) = (F^×)^{⊗n} / Steinberg relations.
pub fn milnor_k_theory_ty() -> Expr {
    arrow(cst("Field"), arrow(nat_ty(), cst("AbGroup")))
}
/// `BlochFormula : Scheme → Nat → Prop`
/// CH^n(X) ≅ H^{2n,n}(X, ℤ) for smooth X over a field (Bloch's formula).
pub fn bloch_formula_ty() -> Expr {
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
                app2(
                    cst("Iso"),
                    app2(cst("ChowGroup"), bvar(2), bvar(1)),
                    app3(
                        cst("MotivicCohomology"),
                        bvar(2),
                        app(cst("NatMul2"), bvar(1)),
                        bvar(1),
                    ),
                ),
            ),
        ),
    )
}
/// Nesterenko-Suslin / Totaro theorem: H^{n,n}(Spec F, ℤ) ≅ K_n^M(F).
///
/// `∀ (F : Field) (n : Nat), Iso (MotivicCohomology (Spec F) n n) (MilnorKTheory F n)`
pub fn nesterenko_suslin_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "F",
        cst("Field"),
        pi(
            BinderInfo::Default,
            "n",
            nat_ty(),
            app2(
                cst("Iso"),
                app3(
                    cst("MotivicCohomology"),
                    app(cst("Spec"), bvar(1)),
                    bvar(0),
                    bvar(0),
                ),
                app2(cst("MilnorKTheory"), bvar(1), bvar(0)),
            ),
        ),
    )
}
/// `MixedMotive : SchemeBase → Type 1`
/// An object in the triangulated category DM(S, ℤ) of mixed motives over S.
pub fn mixed_motive_ty() -> Expr {
    arrow(cst("SchemeBase"), type1())
}
/// `PureMotive : Scheme → Nat → Type`
/// A Chow motive (X, p, n): X a smooth projective variety,
/// p an idempotent correspondence, n a Tate twist.
pub fn pure_motive_ty() -> Expr {
    arrow(cst("Scheme"), arrow(nat_ty(), type0()))
}
/// `MotivicFunctor : Scheme → MixedMotive`
/// X ↦ M(X), the motivic functor assigning to each scheme its motive.
pub fn motivic_functor_ty() -> Expr {
    arrow(cst("Scheme"), cst("MixedMotiveObj"))
}
/// `RealizationFunctor : RealizationType → MixedMotiveObj → GradedModule`
/// Betti, de Rham, or ℓ-adic realization of a mixed motive.
pub fn realization_functor_ty() -> Expr {
    arrow(
        cst("RealizationType"),
        arrow(cst("MixedMotiveObj"), cst("GradedModule")),
    )
}
/// Weight filtration on mixed motives.
///
/// `∀ (M : MixedMotiveObj), ∃ (W : WeightFiltration M), IsStrictlyCompatible W`
pub fn weight_filtration_existence_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "M",
        cst("MixedMotiveObj"),
        app2(
            cst("Exists"),
            app(cst("WeightFiltration"), bvar(0)),
            app(cst("IsStrictlyCompatible"), bvar(0)),
        ),
    )
}
/// Effective motives: M(X)(n) is effective iff n ≥ 0.
///
/// `∀ (X : Scheme) (n : Nat), IsEffective (TateTwist (MotivicFunctor X) n)`
pub fn effective_motive_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        cst("Scheme"),
        pi(
            BinderInfo::Default,
            "n",
            nat_ty(),
            app(
                cst("IsEffective"),
                app2(
                    cst("TateTwist"),
                    app(cst("MotivicFunctor"), bvar(1)),
                    bvar(0),
                ),
            ),
        ),
    )
}
/// Künneth formula for motives: M(X × Y) ≅ M(X) ⊗ M(Y).
///
/// `∀ (X Y : Scheme), Iso (MotivicFunctor (Prod X Y)) (TensorMotive (MotivicFunctor X) (MotivicFunctor Y))`
pub fn kunneth_motives_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        cst("Scheme"),
        pi(
            BinderInfo::Default,
            "Y",
            cst("Scheme"),
            app2(
                cst("Iso"),
                app(
                    cst("MotivicFunctor"),
                    app2(cst("SchemeProd"), bvar(1), bvar(0)),
                ),
                app2(
                    cst("TensorMotive"),
                    app(cst("MotivicFunctor"), bvar(1)),
                    app(cst("MotivicFunctor"), bvar(0)),
                ),
            ),
        ),
    )
}
/// `BlochKatoConjecture : Field → Nat → Prime → Prop`
/// The Bloch-Kato conjecture: H^n_ét(Spec k, μ_ℓ^⊗n) ≅ K_n^M(k) / ℓ.
/// Proved by Voevodsky (Fields Medal 2002) and Rost.
pub fn bloch_kato_conjecture_ty() -> Expr {
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
                "ell",
                cst("Prime"),
                app2(
                    cst("Iso"),
                    app3(
                        cst("EtaleCohomology"),
                        app(cst("Spec"), bvar(2)),
                        bvar(1),
                        app2(cst("RootsOfUnity"), bvar(1), bvar(0)),
                    ),
                    app2(
                        cst("QuotientGroup"),
                        app2(cst("MilnorKTheory"), bvar(2), bvar(1)),
                        bvar(0),
                    ),
                ),
            ),
        ),
    )
}
/// `MilnorConjecture : Field → Prop`
/// The Milnor conjecture: special case of Bloch-Kato for ℓ = 2,
/// relating K_n^M(k)/2 to the Witt group W(k) and étale cohomology with ℤ/2.
pub fn milnor_conjecture_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "k",
        cst("Field"),
        arrow(
            app(cst("CharNot2"), bvar(0)),
            app2(
                cst("Iso"),
                app2(cst("MilnorKTheoryMod2"), bvar(1), cst("AllDegrees")),
                app(cst("GradedWittGroup"), bvar(1)),
            ),
        ),
    )
}
/// `VoevodskysProof : Prop`
/// Voevodsky's proof uses motivic cohomology, the Steenrod algebra,
/// and the Milnor conjecture as a special case.
pub fn voevodsky_proof_ty() -> Expr {
    app2(
        cst("And"),
        cst("MilnorConjectureHolds"),
        app2(
            cst("And"),
            cst("BlochKatoConjectureHolds"),
            cst("MotivicSteenrodAlgebraWellDefined"),
        ),
    )
}
/// `ReducedPowerOperation : Nat → MotivicCohomologyClass → MotivicCohomologyClass`
/// The motivic Steenrod operations Sq^i (for ℓ=2) and P^i (for odd ℓ).
pub fn reduced_power_operation_ty() -> Expr {
    arrow(
        nat_ty(),
        arrow(cst("MotivicCohomologyClass"), cst("MotivicCohomologyClass")),
    )
}
/// `LAdicCohomology : Scheme → Nat → Prime → GaloisModule`
/// H^i_ét(X, ℤ_ℓ) as a continuous Galois module (ℓ-adic cohomology).
pub fn l_adic_cohomology_ty() -> Expr {
    arrow(
        cst("Scheme"),
        arrow(nat_ty(), arrow(cst("Prime"), cst("GaloisModule"))),
    )
}
/// `EtaleSheafLAdicAlgebra : Scheme → Prime → Type`
/// The algebra of ℓ-adic sheaves on X.
pub fn etale_sheaf_l_adic_algebra_ty() -> Expr {
    arrow(cst("Scheme"), arrow(cst("Prime"), type0()))
}
/// Proper base change theorem.
///
/// `∀ (f : ProperMorphism X Y) (F : EtaleSheaveLAdicAlgebra Y ell),
///    Iso (Pullback f (DirectImageSheaf f F)) F`
pub fn proper_base_change_ty() -> Expr {
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
                app2(cst("ProperMorphism"), bvar(1), bvar(0)),
                pi(
                    BinderInfo::Default,
                    "F",
                    app2(cst("EtaleSheaveLAdicAlgebra"), bvar(1), cst("ell")),
                    app2(
                        cst("Iso"),
                        app2(
                            cst("SheafPullback"),
                            bvar(1),
                            app2(cst("DirectImage"), bvar(1), bvar(0)),
                        ),
                        bvar(0),
                    ),
                ),
            ),
        ),
    )
}
/// Purity theorem (cohomological purity).
///
/// `∀ (X : Scheme) (Z : SmoothClosedSubscheme X) (c : Nat),
///    Iso (LAdicCohomologySupport Z ell i) (LAdicCohomology Z ell (i - 2c)(-c))`
pub fn purity_theorem_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        cst("Scheme"),
        pi(
            BinderInfo::Default,
            "Z",
            app(cst("SmoothClosedSubscheme"), bvar(0)),
            pi(
                BinderInfo::Default,
                "c",
                nat_ty(),
                app2(
                    cst("Iso"),
                    app3(cst("LAdicCohomologySupport"), bvar(2), bvar(1), bvar(0)),
                    app3(cst("TateTwistCohomology"), bvar(2), bvar(1), bvar(0)),
                ),
            ),
        ),
    )
}
/// `WeilConjectures : Scheme → Prime → Prop`
/// Weil's conjectures (proved by Deligne, 1974):
/// (1) Rationality of the zeta function,
/// (2) Functional equation,
/// (3) Riemann hypothesis (eigenvalues of Frobenius have absolute value q^{i/2}).
pub fn weil_conjectures_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        cst("Scheme"),
        pi(
            BinderInfo::Default,
            "ell",
            cst("Prime"),
            arrow(
                app(cst("IsProperSmoothOverFiniteField"), bvar(1)),
                app3(
                    cst("And3"),
                    app2(cst("ZetaFunctionRational"), bvar(2), bvar(1)),
                    app2(cst("ZetaFunctionFunctionalEqn"), bvar(2), bvar(1)),
                    app2(cst("ZetaFunctionRiemannHypothesis"), bvar(2), bvar(1)),
                ),
            ),
        ),
    )
}
/// Deligne's theorem: the Weil conjectures hold.
///
/// `∀ (X : Scheme) (ell : Prime), IsProperSmoothOverFiniteField X → WeilConjectures X ell`
pub fn deligne_weil_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        cst("Scheme"),
        pi(
            BinderInfo::Default,
            "ell",
            cst("Prime"),
            arrow(
                app(cst("IsProperSmoothOverFiniteField"), bvar(1)),
                app2(cst("WeilConjecturesHold"), bvar(1), bvar(0)),
            ),
        ),
    )
}
/// Register all motivic cohomology axioms and theorem types in the environment.
pub fn build_env(env: &mut Environment) {
    let base_types: &[(&str, Expr)] = &[
        ("CommRing", type0()),
        ("AbGroup", type0()),
        ("Spectrum", type0()),
        ("Scheme", type1()),
        ("SchemeBase", type0()),
        ("Field", type0()),
        ("Prime", nat_ty()),
        ("ChainComplex", type0()),
        ("GradedModule", type0()),
        ("GaloisModule", type0()),
        ("MixedMotiveObj", type1()),
        ("RealizationType", type0()),
        ("WeightFiltration", arrow(cst("MixedMotiveObj"), type0())),
        ("MotivicCohomologyClass", type0()),
        (
            "AlgebraicKGroup",
            arrow(cst("CommRing"), arrow(nat_ty(), cst("AbGroup"))),
        ),
        (
            "MilnorKTheory",
            arrow(cst("Field"), arrow(nat_ty(), cst("AbGroup"))),
        ),
        (
            "ChowGroup",
            arrow(cst("Scheme"), arrow(nat_ty(), cst("AbGroup"))),
        ),
        (
            "MotivicCohomology",
            arrow(
                cst("Scheme"),
                arrow(nat_ty(), arrow(nat_ty(), cst("AbGroup"))),
            ),
        ),
        (
            "EtaleCohomology",
            arrow(
                cst("Scheme"),
                arrow(nat_ty(), arrow(type0(), cst("AbGroup"))),
            ),
        ),
        (
            "LAdicCohomology",
            arrow(
                cst("Scheme"),
                arrow(nat_ty(), arrow(cst("Prime"), cst("GaloisModule"))),
            ),
        ),
        (
            "EtaleSheaveLAdicAlgebra",
            arrow(cst("Scheme"), arrow(cst("Prime"), type0())),
        ),
        ("KTheory", arrow(cst("Scheme"), cst("Spectrum"))),
        ("Spec", arrow(cst("CommRing"), cst("Scheme"))),
        ("Iso", arrow(type0(), arrow(type0(), prop()))),
        ("And", arrow(prop(), arrow(prop(), prop()))),
        ("And3", arrow(prop(), arrow(prop(), arrow(prop(), prop())))),
        ("Exists", arrow(type0(), arrow(type0(), prop()))),
        ("IsRegular", arrow(cst("Scheme"), prop())),
        ("IsSmooth", arrow(cst("Scheme"), prop())),
        ("IsEffective", arrow(cst("MixedMotiveObj"), prop())),
        ("IsExact", arrow(type0(), prop())),
        ("IsStrictlyCompatible", arrow(cst("MixedMotiveObj"), prop())),
        ("GerstenComplex", arrow(cst("Scheme"), type0())),
        ("ClosedSubscheme", arrow(cst("Scheme"), type0())),
        ("SmoothClosedSubscheme", arrow(cst("Scheme"), type0())),
        (
            "ProperMorphism",
            arrow(cst("Scheme"), arrow(cst("Scheme"), type0())),
        ),
        (
            "FlatMorphism",
            arrow(cst("Scheme"), arrow(cst("Scheme"), type0())),
        ),
        (
            "SchemeProd",
            arrow(cst("Scheme"), arrow(cst("Scheme"), cst("Scheme"))),
        ),
        (
            "SchemeComplement",
            arrow(cst("Scheme"), arrow(cst("ClosedSubscheme"), cst("Scheme"))),
        ),
        (
            "LongExactSeq",
            arrow(
                cst("Spectrum"),
                arrow(cst("Spectrum"), arrow(cst("Spectrum"), prop())),
            ),
        ),
        (
            "AlgebraicCycle",
            arrow(cst("Scheme"), arrow(nat_ty(), type0())),
        ),
        ("RationalEquivalence", rational_equivalence_ty()),
        (
            "TateTwist",
            arrow(
                cst("MixedMotiveObj"),
                arrow(nat_ty(), cst("MixedMotiveObj")),
            ),
        ),
        (
            "TensorMotive",
            arrow(
                cst("MixedMotiveObj"),
                arrow(cst("MixedMotiveObj"), cst("MixedMotiveObj")),
            ),
        ),
        (
            "MotivicFunctor",
            arrow(cst("Scheme"), cst("MixedMotiveObj")),
        ),
        (
            "RealizationFunctor",
            arrow(
                cst("RealizationType"),
                arrow(cst("MixedMotiveObj"), cst("GradedModule")),
            ),
        ),
        (
            "RootsOfUnity",
            arrow(nat_ty(), arrow(cst("Prime"), type0())),
        ),
        (
            "QuotientGroup",
            arrow(cst("AbGroup"), arrow(cst("Prime"), cst("AbGroup"))),
        ),
        (
            "MilnorKTheoryMod2",
            arrow(cst("Field"), arrow(type0(), cst("AbGroup"))),
        ),
        ("GradedWittGroup", arrow(cst("Field"), cst("AbGroup"))),
        ("CharNot2", arrow(cst("Field"), prop())),
        ("CharOf", arrow(cst("Field"), nat_ty())),
        ("AllDegrees", type0()),
        ("MilnorConjectureHolds", prop()),
        ("BlochKatoConjectureHolds", prop()),
        ("MotivicSteenrodAlgebraWellDefined", prop()),
        ("SheafPullback", arrow(type0(), arrow(type0(), type0()))),
        ("DirectImage", arrow(type0(), arrow(type0(), type0()))),
        (
            "LAdicCohomologySupport",
            arrow(
                cst("Scheme"),
                arrow(type0(), arrow(nat_ty(), cst("AbGroup"))),
            ),
        ),
        (
            "TateTwistCohomology",
            arrow(
                cst("Scheme"),
                arrow(type0(), arrow(nat_ty(), cst("AbGroup"))),
            ),
        ),
        (
            "ZetaFunctionRational",
            arrow(cst("Scheme"), arrow(cst("Prime"), prop())),
        ),
        (
            "ZetaFunctionFunctionalEqn",
            arrow(cst("Scheme"), arrow(cst("Prime"), prop())),
        ),
        (
            "ZetaFunctionRiemannHypothesis",
            arrow(cst("Scheme"), arrow(cst("Prime"), prop())),
        ),
        (
            "WeilConjecturesHold",
            arrow(cst("Scheme"), arrow(cst("Prime"), prop())),
        ),
        (
            "IsProperSmoothOverFiniteField",
            arrow(cst("Scheme"), prop()),
        ),
        ("LaurentPoly", arrow(cst("CommRing"), cst("CommRing"))),
        ("Nat.pred", arrow(nat_ty(), nat_ty())),
        ("Prod", arrow(type0(), arrow(type0(), type0()))),
        ("NatMul2", arrow(nat_ty(), nat_ty())),
        ("ChowCodimShift", arrow(nat_ty(), nat_ty())),
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
        ("AlgebraicKGroupType", algebraic_k_group_ty),
        ("KTheorySpectrumType", k_theory_spectrum_ty),
        ("GerstenResolutionType", gersten_resolution_ty),
        ("BassNegativeKGroupType", bass_negative_k_group_ty),
        ("ChowGroupType", chow_group_ty),
        ("ChowRingType", chow_ring_ty),
        ("AlgebraicCycleType", algebraic_cycle_ty),
        ("MotivicCohomologyType", motivic_cohomology_ty),
        ("MotivicComplexType", motivic_complex_ty),
        ("MilnorKTheoryType", milnor_k_theory_ty),
        ("MixedMotiveType", mixed_motive_ty),
        ("PureMotiveType", pure_motive_ty),
        ("LAdicCohomologyType", l_adic_cohomology_ty),
        ("EtaleSheaveLAdicAlgebraType", etale_sheaf_l_adic_algebra_ty),
        ("ReducedPowerOperationType", reduced_power_operation_ty),
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
        ("gersten_conjecture", gersten_conjecture_ty),
        ("quillen_localization", quillen_localization_ty),
        ("bass_fundamental_theorem", bass_fundamental_theorem_ty),
        ("chow_push_forward", chow_push_forward_ty),
        ("chow_pull_back", chow_pull_back_ty),
        ("bloch_formula", bloch_formula_ty),
        ("nesterenko_suslin", nesterenko_suslin_ty),
        (
            "weight_filtration_existence",
            weight_filtration_existence_ty,
        ),
        ("effective_motive", effective_motive_ty),
        ("kunneth_motives", kunneth_motives_ty),
        ("bloch_kato_conjecture", bloch_kato_conjecture_ty),
        ("milnor_conjecture", milnor_conjecture_ty),
        ("voevodsky_proof", voevodsky_proof_ty),
        ("proper_base_change", proper_base_change_ty),
        ("purity_theorem", purity_theorem_ty),
        ("weil_conjectures", weil_conjectures_ty),
        ("deligne_weil", deligne_weil_ty),
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
/// Compute the rank of K_0(k\[x_1,...,x_n\]) for a field k.
///
/// K_0 of a polynomial ring over a field equals ℤ (rank 1).
pub fn k0_polynomial_ring_rank() -> usize {
    1
}
/// Degree of the algebraic cycle \[V\] of a hypersurface V ⊂ P^n of degree d.
pub fn hypersurface_cycle_degree(d: i64) -> i64 {
    d
}
/// Euler characteristic of motivic cohomology H^{*,*}(P^n, ℤ).
///
/// The Chow ring of P^n is ℤ\[H\]/(H^{n+1}), so Σ rank CH^p(P^n) = n+1.
pub fn projective_space_chow_rank(n: usize) -> usize {
    n + 1
}
/// `HigherChowGroup : Scheme → Nat → Nat → AbGroup`
/// z^p(X, n) = Bloch's higher Chow group CH^p(X, n).
pub fn mc_ext_higher_chow_group_ty() -> Expr {
    arrow(
        cst("Scheme"),
        arrow(nat_ty(), arrow(nat_ty(), cst("AbGroup"))),
    )
}
/// `HigherChowComplexity : Scheme → Nat → ChainComplex`
/// The simplicial set z^p(X, *) defining higher Chow groups via normalization.
pub fn mc_ext_higher_chow_complex_ty() -> Expr {
    arrow(cst("Scheme"), arrow(nat_ty(), cst("ChainComplex")))
}
/// Bloch's formula for higher Chow groups: CH^p(X, n) ≅ H^{2p-n, p}(X, ℤ).
///
/// `∀ (X : Scheme) (p n : Nat), IsSmooth X →
///    Iso (HigherChowGroup X p n) (MotivicCohomology X (2*p - n) p)`
pub fn mc_ext_higher_chow_bloch_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        cst("Scheme"),
        pi(
            BinderInfo::Default,
            "p",
            nat_ty(),
            pi(
                BinderInfo::Default,
                "n",
                nat_ty(),
                arrow(
                    app(cst("IsSmooth"), bvar(2)),
                    app2(
                        cst("Iso"),
                        app3(cst("HigherChowGroup"), bvar(3), bvar(2), bvar(1)),
                        app3(
                            cst("MotivicCohomology"),
                            bvar(3),
                            cst("TwoPMinusN"),
                            bvar(2),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// `RigidityTheorem : Field → Nat → Prime → Prop`
/// Suslin's rigidity theorem: motivic cohomology H^{n,n}(X, ℤ/ℓ) is rigid.
pub fn mc_ext_rigidity_theorem_ty() -> Expr {
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
                "ell",
                cst("Prime"),
                app3(cst("IsRigid"), bvar(2), bvar(1), bvar(0)),
            ),
        ),
    )
}
/// `KTheoryMotivicRelation : Scheme → Nat → Prop`
/// Adams spectral sequence relating motivic cohomology to algebraic K-theory:
/// E_2^{p,q} = H^{p-q,−q}(X, ℤ) ⟹ K_{-p-q}(X).
pub fn mc_ext_k_theory_motivic_relation_ty() -> Expr {
    arrow(cst("Scheme"), arrow(nat_ty(), prop()))
}
/// `AtiyahHirzebruchSpectralSeq : Scheme → Prop`
/// The motivic Atiyah-Hirzebruch spectral sequence converging to algebraic K-theory.
pub fn mc_ext_atiyah_hirzebruch_ss_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        cst("Scheme"),
        arrow(
            app(cst("IsSmooth"), bvar(0)),
            app(cst("HasSpectralSeq"), app(cst("KTheory"), bvar(1))),
        ),
    )
}
/// `GammaFiltration : CommRing → Nat → AbGroup`
/// The γ-filtration on K_0(R) refining the dimension filtration.
pub fn mc_ext_gamma_filtration_ty() -> Expr {
    arrow(cst("CommRing"), arrow(nat_ty(), cst("AbGroup")))
}
/// `AdamsOperation : CommRing → Nat → AbGroup → AbGroup`
/// Adams operation ψ^k on algebraic K-theory: ring endomorphism of K_*(R).
pub fn mc_ext_adams_operation_ty() -> Expr {
    arrow(
        cst("CommRing"),
        arrow(nat_ty(), arrow(cst("AbGroup"), cst("AbGroup"))),
    )
}
/// Eigenspace decomposition of K_n(R) under Adams operations.
///
/// `∀ (R : CommRing) (n k : Nat), GammaFiltration R k ≅ EigenspaceAdams ψ^n k`
pub fn mc_ext_adams_eigenspace_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "R",
        cst("CommRing"),
        pi(
            BinderInfo::Default,
            "n",
            nat_ty(),
            pi(
                BinderInfo::Default,
                "k",
                nat_ty(),
                app2(
                    cst("Iso"),
                    app2(cst("GammaFiltration"), bvar(2), bvar(0)),
                    app3(cst("EigenspaceAdams"), bvar(2), bvar(1), bvar(0)),
                ),
            ),
        ),
    )
}
/// `LambdaRingStructure : CommRing → Prop`
/// K_0(R) carries a λ-ring structure via exterior power operations.
pub fn mc_ext_lambda_ring_structure_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "R",
        cst("CommRing"),
        app(cst("IsLambdaRing"), app(cst("AlgebraicKGroup"), bvar(0))),
    )
}
/// `BeilinsonLichtenbaumConj : Field → Nat → Prime → Prop`
/// Beilinson-Lichtenbaum: the motivic-to-étale map is an isomorphism in degrees ≤ weight.
pub fn mc_ext_beilinson_lichtenbaum_ty() -> Expr {
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
                "ell",
                cst("Prime"),
                app3(cst("MotivicEtaleIso"), bvar(2), bvar(1), bvar(0)),
            ),
        ),
    )
}
/// `MotivicToEtaleMap : Field → Nat → Nat → Prime → Prop`
/// The natural map H^{p,q}(k, ℤ/ℓ) → H^p_ét(k, μ_ℓ^⊗q).
pub fn mc_ext_motivic_to_etale_map_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "k",
        cst("Field"),
        pi(
            BinderInfo::Default,
            "p",
            nat_ty(),
            pi(
                BinderInfo::Default,
                "q",
                nat_ty(),
                pi(
                    BinderInfo::Default,
                    "ell",
                    cst("Prime"),
                    arrow(
                        app3(cst("MotivicCohomologyMod"), bvar(3), bvar(2), bvar(1)),
                        app3(
                            cst("EtaleCohomology"),
                            app(cst("Spec"), bvar(3)),
                            bvar(2),
                            cst("MuEll"),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// `VoevodskyMotivatorsThm : Prop`
/// Voevodsky's theorem: Beilinson-Lichtenbaum is equivalent to Bloch-Kato.
pub fn mc_ext_voevodsky_motivators_ty() -> Expr {
    app2(
        cst("Iff"),
        cst("BeilinsonLichtenbaumHolds"),
        cst("BlochKatoConjectureHolds"),
    )
}
/// `A1HomotopyCategory : Scheme → Type 1`
/// The A¹-homotopy category H(k) of pointed motivic spaces over a field.
pub fn mc_ext_a1_homotopy_category_ty() -> Expr {
    arrow(cst("Field"), type1())
}
/// `A1WeakEquivalence : Scheme → Scheme → Prop`
/// An A¹-weak equivalence between motivic spaces.
pub fn mc_ext_a1_weak_equivalence_ty() -> Expr {
    arrow(cst("Scheme"), arrow(cst("Scheme"), prop()))
}
/// `MotivicFundamentalGroup : Scheme → AbGroup`
/// The motivic fundamental group π_1^A¹(X) in the A¹-homotopy category.
pub fn mc_ext_motivic_fundamental_group_ty() -> Expr {
    arrow(cst("Scheme"), cst("AbGroup"))
}
/// `MotivicHomotopyGroup : Scheme → Nat → AbGroup`
/// The n-th motivic homotopy group π_n^A¹(X).
pub fn mc_ext_motivic_homotopy_group_ty() -> Expr {
    arrow(cst("Scheme"), arrow(nat_ty(), cst("AbGroup")))
}
/// Morel's theorem: π_1^A¹(A² \ {0}) ≅ K_2^{MW}.
///
/// `Iso (MotivicFundamentalGroup (Punctured A^2)) (MilnorWittK2)`
pub fn mc_ext_morel_theorem_ty() -> Expr {
    app2(
        cst("Iso"),
        app(cst("MotivicFundamentalGroup"), cst("PuncturedA2")),
        cst("MilnorWittK2"),
    )
}
/// `MotivicSphere : Nat → Nat → Scheme`
/// The motivic sphere S^{p,q} = ℙ^1 smashed with T^∧(q), Σ^p.
pub fn mc_ext_motivic_sphere_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), cst("Scheme")))
}
/// `NisnevichSheaf : Scheme → Type 0`
/// A sheaf in the Nisnevich topology on Sch/S.
pub fn mc_ext_nisnevich_sheaf_ty() -> Expr {
    arrow(cst("Scheme"), type0())
}
/// `NisnevichCohomology : Scheme → Nat → AbGroup → AbGroup`
/// Nisnevich cohomology H^n_{Nis}(X, F).
pub fn mc_ext_nisnevich_cohomology_ty() -> Expr {
    arrow(
        cst("Scheme"),
        arrow(nat_ty(), arrow(cst("AbGroup"), cst("AbGroup"))),
    )
}
/// `CdhSheaf : Scheme → Type 0`
/// A sheaf in the cdh topology (completely decomposed h-topology).
pub fn mc_ext_cdh_sheaf_ty() -> Expr {
    arrow(cst("Scheme"), type0())
}
/// `CdhDescent : Scheme → Prop`
/// cdh-descent for algebraic K-theory: K-theory satisfies descent for cdh squares.
pub fn mc_ext_cdh_descent_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        cst("Scheme"),
        arrow(
            app(cst("IsCdhSquare"), bvar(0)),
            app(cst("KTheoryDescentHolds"), bvar(1)),
        ),
    )
}
/// Nisnevich excision for K-theory.
///
/// `∀ (X : Scheme) (U : NisnevichCover X), IsExcisive (KTheory U)`
pub fn mc_ext_nisnevich_excision_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        cst("Scheme"),
        pi(
            BinderInfo::Default,
            "U",
            app(cst("NisnevichCover"), bvar(0)),
            app(cst("IsExcisive"), app(cst("KTheory"), bvar(0))),
        ),
    )
}
/// `OrientedCohomologyTheory : Scheme → Type 1`
/// An oriented cohomology theory on Sm/k: Chow groups, K-theory, cobordism, etc.
pub fn mc_ext_oriented_cohomology_theory_ty() -> Expr {
    arrow(cst("Scheme"), type1())
}
/// `FormalGroupLaw : CommRing → Type 0`
/// The formal group law of an oriented cohomology theory h:
/// captures the first Chern class of a tensor product of line bundles.
pub fn mc_ext_formal_group_law_ty() -> Expr {
    arrow(cst("CommRing"), type0())
}
/// `UniversalFormalGroupLaw : Prop`
/// Lazard's theorem: the universal formal group law is represented by the Lazard ring L.
pub fn mc_ext_universal_formal_group_law_ty() -> Expr {
    app2(cst("IsUniversal"), cst("LazardRing"), cst("FormalGroupLaw"))
}
/// `ChernClassMap : OrientedCohomologyTheory → LineBundleClass → CohomClass`
/// The first Chern class c_1 : Pic(X) → h^{1,1}(X).
pub fn mc_ext_chern_class_map_ty() -> Expr {
    arrow(
        cst("OrientedCohomology"),
        arrow(cst("LineBundleClass"), cst("CohomClass")),
    )
}
/// Projective bundle formula for oriented cohomology.
///
/// `∀ (E : VectorBundle X r), h(P(E)) ≅ h(X)\[ξ\]/(ξ^r + c_1(E)ξ^{r-1} + ... + c_r(E))`
pub fn mc_ext_projective_bundle_formula_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        cst("Scheme"),
        pi(
            BinderInfo::Default,
            "E",
            app(cst("VectorBundle"), bvar(0)),
            pi(
                BinderInfo::Default,
                "r",
                nat_ty(),
                app2(
                    cst("Iso"),
                    app(
                        cst("OrientedCohomology"),
                        app(cst("ProjectiveBundle"), bvar(1)),
                    ),
                    app2(
                        cst("QuotientRing"),
                        app(cst("OrientedCohomology"), bvar(2)),
                        cst("CharPolyChern"),
                    ),
                ),
            ),
        ),
    )
}
/// `AlgebraicCobordism : Scheme → AbGroup`
/// Levine-Morel algebraic cobordism Ω^*(X): the universal oriented cohomology theory.
pub fn mc_ext_algebraic_cobordism_ty() -> Expr {
    arrow(cst("Scheme"), cst("AbGroup"))
}
/// `CobordismFormalGroupLaw : CommRing → Prop`
/// The formal group law of algebraic cobordism is the universal one over the Lazard ring.
pub fn mc_ext_cobordism_formal_group_law_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "R",
        cst("CommRing"),
        app2(cst("IsUniversalFGL"), cst("AlgebraicCobordismFGL"), bvar(0)),
    )
}
/// Levine-Morel theorem: Ω^*(X) is the universal oriented cohomology theory.
///
/// `∀ (h : OrientedCohomologyTheory), ∃! (φ : NaturalTransformation Ω h), IsOrientedMap φ`
pub fn mc_ext_levine_morel_theorem_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "h",
        cst("OrientedCohomology"),
        app2(
            cst("ExistsUnique"),
            app2(cst("NatTrans"), cst("AlgebraicCobordism"), bvar(0)),
            cst("IsOrientedMap"),
        ),
    )
}
/// `CobordismRingCoefficients : CommRing`
/// The cobordism ring Ω^*(pt) = MGL_{2*,*}(pt) ≅ Lazard ring.
pub fn mc_ext_cobordism_ring_coefficients_ty() -> Expr {
    cst("CommRing")
}
/// `SliceFiltration : MixedMotiveObj → Nat → MixedMotiveObj`
/// The slice filtration f_n(M) in SH(k): the n-th effective slice.
pub fn mc_ext_slice_filtration_ty() -> Expr {
    arrow(
        cst("MixedMotiveObj"),
        arrow(nat_ty(), cst("MixedMotiveObj")),
    )
}
/// `SliceCofiber : MixedMotiveObj → Nat → MixedMotiveObj`
/// The n-th slice s_n(M) = cofiber of f_{n+1}(M) → f_n(M).
pub fn mc_ext_slice_cofiber_ty() -> Expr {
    arrow(
        cst("MixedMotiveObj"),
        arrow(nat_ty(), cst("MixedMotiveObj")),
    )
}
/// Voevodsky's conjecture on slices of the sphere spectrum.
///
/// `s_n(S) ≅ MZ\[n\] ∧ T^∧n` where MZ is the motivic Eilenberg-MacLane spectrum.
pub fn mc_ext_voevodsky_slices_conjecture_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        app2(
            cst("Iso"),
            app2(cst("SliceCofiber"), cst("SphereSpectrum"), bvar(0)),
            app2(
                cst("MotivicSmash"),
                app(cst("MotivicEMLSpectrum"), cst("ZCoeffs")),
                app(cst("TateTwist"), bvar(0)),
            ),
        ),
    )
}
/// `EffectiveCoverFunctor : MixedMotiveObj → MixedMotiveObj`
/// The effective cover functor f_0: SH(k) → SH^eff(k).
pub fn mc_ext_effective_cover_functor_ty() -> Expr {
    arrow(cst("MixedMotiveObj"), cst("MixedMotiveObj"))
}
/// `EffectiveMotivesCategory : Field → Type 1`
/// The ∞-category of effective motives SH^eff(k).
pub fn mc_ext_effective_motives_category_ty() -> Expr {
    arrow(cst("Field"), type1())
}
/// `MotivicEML : AbGroup → Nat → Nat → Scheme`
/// The motivic Eilenberg-MacLane space K(A, p, q) representing H^{p,q}(-, A).
pub fn mc_ext_motivic_eml_ty() -> Expr {
    arrow(
        cst("AbGroup"),
        arrow(nat_ty(), arrow(nat_ty(), cst("Scheme"))),
    )
}
/// `MotivicEMLSpectrum : CommRing → Spectrum`
/// The motivic Eilenberg-MacLane spectrum MR for a ring R.
pub fn mc_ext_motivic_eml_spectrum_ty() -> Expr {
    arrow(cst("CommRing"), cst("Spectrum"))
}
/// Representability of motivic cohomology.
///
/// `∀ (X : Scheme) (p q : Nat), H^{p,q}(X, A) ≅ \[X, K(A, p, q)\]_A1`
pub fn mc_ext_motivic_cohomology_representability_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        cst("Scheme"),
        pi(
            BinderInfo::Default,
            "p",
            nat_ty(),
            pi(
                BinderInfo::Default,
                "q",
                nat_ty(),
                app2(
                    cst("Iso"),
                    app3(cst("MotivicCohomology"), bvar(2), bvar(1), bvar(0)),
                    app2(
                        cst("A1HomSets"),
                        bvar(2),
                        app3(cst("MotivicEML"), cst("ZCoeffs"), bvar(1), bvar(0)),
                    ),
                ),
            ),
        ),
    )
}
/// `MotivicSteenrodAlgebra : Prime → Type 0`
/// The motivic Steenrod algebra A^{*,*}(k; ℓ) acting on motivic cohomology mod ℓ.
pub fn mc_ext_motivic_steenrod_algebra_ty() -> Expr {
    arrow(cst("Prime"), type0())
}
/// Milnor basis for motivic Steenrod algebra.
///
/// `∀ (ell : Prime), HasMilnorBasis (MotivicSteenrodAlgebra ell)`
pub fn mc_ext_milnor_basis_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "ell",
        cst("Prime"),
        app(
            cst("HasMilnorBasis"),
            app(cst("MotivicSteenrodAlgebra"), bvar(0)),
        ),
    )
}
/// `WeilCohomologyTheory : Field → Type 1`
/// A Weil cohomology theory on smooth projective varieties over a field.
pub fn mc_ext_weil_cohomology_theory_ty() -> Expr {
    arrow(cst("Field"), type1())
}
/// `WeilCohomologyAxioms : WeilCohomologyTheory → Prop`
/// The axioms for a Weil cohomology theory: finiteness, Poincaré duality, Künneth, cycle map.
pub fn mc_ext_weil_cohomology_axioms_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "H",
        cst("WeilCohomology"),
        app2(
            cst("And"),
            app(cst("HasFiniteDimensionality"), bvar(0)),
            app2(
                cst("And"),
                app(cst("HasPoincareDuality"), bvar(1)),
                app(cst("HasCycleClassMap"), bvar(2)),
            ),
        ),
    )
}
/// `PoincareDuality : Scheme → Nat → Nat → Prop`
/// Poincaré duality for Weil cohomology: H^i ≅ (H^{2d-i})^∨(d) where d = dim X.
pub fn mc_ext_poincare_duality_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        cst("Scheme"),
        pi(
            BinderInfo::Default,
            "i",
            nat_ty(),
            pi(
                BinderInfo::Default,
                "d",
                nat_ty(),
                arrow(
                    app(cst("IsSmooth"), bvar(2)),
                    app2(
                        cst("Iso"),
                        app2(cst("WeilCohomology"), bvar(3), bvar(2)),
                        app(
                            cst("TateTwistDual"),
                            app2(
                                cst("WeilCohomology"),
                                bvar(3),
                                app2(cst("NatSub"), app(cst("NatMul2"), bvar(1)), bvar(2)),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// `CycleClassMap : Scheme → Nat → AbGroup → AbGroup`
/// The cycle class map cl: CH^p(X) → H^{2p}(X) to a Weil cohomology theory.
pub fn mc_ext_cycle_class_map_ty() -> Expr {
    arrow(
        cst("Scheme"),
        arrow(nat_ty(), arrow(cst("AbGroup"), cst("AbGroup"))),
    )
}
/// `KunnethFormula : Scheme → Scheme → Prop`
/// The Künneth formula for Weil cohomology: H^*(X×Y) ≅ H^*(X) ⊗ H^*(Y).
pub fn mc_ext_kunneth_formula_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        cst("Scheme"),
        pi(
            BinderInfo::Default,
            "Y",
            cst("Scheme"),
            app2(
                cst("Iso"),
                app(
                    cst("WeilCohomGraded"),
                    app2(cst("SchemeProd"), bvar(1), bvar(0)),
                ),
                app2(
                    cst("TensorGradedModule"),
                    app(cst("WeilCohomGraded"), bvar(1)),
                    app(cst("WeilCohomGraded"), bvar(0)),
                ),
            ),
        ),
    )
}
/// `ChowMotiveCategory : Field → Type 1`
/// The category of Chow motives M(k) over a field k.
pub fn mc_ext_chow_motive_category_ty() -> Expr {
    arrow(cst("Field"), type1())
}
/// `SmithTheory : Prime → Scheme → Prop`
/// Smith theory for fixed-point schemes under a prime-order group action.
pub fn mc_ext_smith_theory_ty() -> Expr {
    arrow(cst("Prime"), arrow(cst("Scheme"), prop()))
}
/// `MotivicDecomposition : Scheme → MixedMotiveObj → Prop`
/// Motivic decomposition: a smooth projective variety decomposes as a direct sum of motives.
pub fn mc_ext_motivic_decomposition_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        cst("Scheme"),
        arrow(
            app(cst("IsSmooth"), bvar(0)),
            app2(
                cst("HasMotivicDecomposition"),
                bvar(1),
                app(cst("MotivicFunctor"), bvar(1)),
            ),
        ),
    )
}
/// `TateConjecture : Scheme → Prime → Prop`
/// Tate conjecture: the cycle class map CH^p(X) ⊗ ℚ_ℓ → H^{2p}_ét(X, ℚ_ℓ)(p)^{Gal} is surjective.
pub fn mc_ext_tate_conjecture_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        cst("Scheme"),
        pi(
            BinderInfo::Default,
            "ell",
            cst("Prime"),
            arrow(
                app(cst("IsProperSmoothOverFiniteField"), bvar(1)),
                app2(cst("CycleClassIsSurjective"), bvar(2), bvar(1)),
            ),
        ),
    )
}
/// `HodgeConjecture : Scheme → Prop`
/// Hodge conjecture: Hodge classes on a complex projective variety are algebraic.
pub fn mc_ext_hodge_conjecture_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        cst("Scheme"),
        arrow(
            app(cst("IsProjectiveComplex"), bvar(0)),
            app(cst("HodgeClassesAreAlgebraic"), bvar(1)),
        ),
    )
}
/// `StandardConjecture : Nat → Scheme → Prop`
/// Grothendieck's standard conjectures: A (algebraicity of Lefschetz classes), B (Lefschetz), C (Künneth), D (numerical = homological).
pub fn mc_ext_standard_conjecture_ty() -> Expr {
    arrow(nat_ty(), arrow(cst("Scheme"), prop()))
}
/// Register all extended motivic cohomology axioms in the environment.
pub fn register_motivic_cohomology_extended(env: &mut Environment) -> Result<(), String> {
    let axioms: &[(&str, Expr)] = &[
        ("HigherChowGroup", mc_ext_higher_chow_group_ty()),
        ("HigherChowComplex", mc_ext_higher_chow_complex_ty()),
        ("higher_chow_bloch", mc_ext_higher_chow_bloch_ty()),
        ("rigidity_theorem", mc_ext_rigidity_theorem_ty()),
        (
            "KTheoryMotivicRelation",
            mc_ext_k_theory_motivic_relation_ty(),
        ),
        ("AtiyahHirzebruchSS", mc_ext_atiyah_hirzebruch_ss_ty()),
        ("GammaFiltration", mc_ext_gamma_filtration_ty()),
        ("AdamsOperation", mc_ext_adams_operation_ty()),
        ("adams_eigenspace", mc_ext_adams_eigenspace_ty()),
        ("lambda_ring_structure", mc_ext_lambda_ring_structure_ty()),
        ("beilinson_lichtenbaum", mc_ext_beilinson_lichtenbaum_ty()),
        ("MotivicToEtaleMap", mc_ext_motivic_to_etale_map_ty()),
        ("voevodsky_motivators", mc_ext_voevodsky_motivators_ty()),
        ("A1HomotopyCategory", mc_ext_a1_homotopy_category_ty()),
        ("A1WeakEquivalence", mc_ext_a1_weak_equivalence_ty()),
        (
            "MotivicFundamentalGroup",
            mc_ext_motivic_fundamental_group_ty(),
        ),
        ("MotivicHomotopyGroup", mc_ext_motivic_homotopy_group_ty()),
        ("morel_theorem", mc_ext_morel_theorem_ty()),
        ("MotivicSphere", mc_ext_motivic_sphere_ty()),
        ("NisnevichSheaf", mc_ext_nisnevich_sheaf_ty()),
        ("NisnevichCohomology", mc_ext_nisnevich_cohomology_ty()),
        ("CdhSheaf", mc_ext_cdh_sheaf_ty()),
        ("cdh_descent", mc_ext_cdh_descent_ty()),
        ("nisnevich_excision", mc_ext_nisnevich_excision_ty()),
        (
            "OrientedCohomologyTheory",
            mc_ext_oriented_cohomology_theory_ty(),
        ),
        ("FormalGroupLaw", mc_ext_formal_group_law_ty()),
        (
            "universal_formal_group_law",
            mc_ext_universal_formal_group_law_ty(),
        ),
        ("ChernClassMap", mc_ext_chern_class_map_ty()),
        (
            "projective_bundle_formula",
            mc_ext_projective_bundle_formula_ty(),
        ),
        ("AlgebraicCobordism", mc_ext_algebraic_cobordism_ty()),
        (
            "cobordism_formal_group_law",
            mc_ext_cobordism_formal_group_law_ty(),
        ),
        ("levine_morel_theorem", mc_ext_levine_morel_theorem_ty()),
        (
            "CobordismRingCoefficients",
            mc_ext_cobordism_ring_coefficients_ty(),
        ),
        ("SliceFiltration", mc_ext_slice_filtration_ty()),
        ("SliceCofiber", mc_ext_slice_cofiber_ty()),
        (
            "voevodsky_slices_conjecture",
            mc_ext_voevodsky_slices_conjecture_ty(),
        ),
        ("EffectiveCoverFunctor", mc_ext_effective_cover_functor_ty()),
        (
            "EffectiveMotivesCategory",
            mc_ext_effective_motives_category_ty(),
        ),
        ("MotivicEML", mc_ext_motivic_eml_ty()),
        ("MotivicEMLSpectrum", mc_ext_motivic_eml_spectrum_ty()),
        (
            "motivic_cohomology_representability",
            mc_ext_motivic_cohomology_representability_ty(),
        ),
        (
            "MotivicSteenrodAlgebra",
            mc_ext_motivic_steenrod_algebra_ty(),
        ),
        ("milnor_basis", mc_ext_milnor_basis_ty()),
        ("WeilCohomologyTheory", mc_ext_weil_cohomology_theory_ty()),
        ("weil_cohomology_axioms", mc_ext_weil_cohomology_axioms_ty()),
        ("PoincareDuality", mc_ext_poincare_duality_ty()),
        ("CycleClassMap", mc_ext_cycle_class_map_ty()),
        ("kunneth_formula", mc_ext_kunneth_formula_ty()),
        ("ChowMotiveCategory", mc_ext_chow_motive_category_ty()),
        ("SmithTheory", mc_ext_smith_theory_ty()),
        ("motivic_decomposition", mc_ext_motivic_decomposition_ty()),
        ("tate_conjecture", mc_ext_tate_conjecture_ty()),
        ("hodge_conjecture", mc_ext_hodge_conjecture_ty()),
        ("StandardConjecture", mc_ext_standard_conjecture_ty()),
        (
            "IsRigid",
            arrow(cst("Field"), arrow(nat_ty(), arrow(cst("Prime"), prop()))),
        ),
        ("HasSpectralSeq", arrow(cst("Spectrum"), prop())),
        ("IsLambdaRing", arrow(cst("AbGroup"), prop())),
        (
            "MotivicEtaleIso",
            arrow(cst("Field"), arrow(nat_ty(), arrow(cst("Prime"), prop()))),
        ),
        (
            "MotivicCohomologyMod",
            arrow(
                cst("Field"),
                arrow(nat_ty(), arrow(nat_ty(), cst("AbGroup"))),
            ),
        ),
        ("MuEll", type0()),
        ("Iff", arrow(prop(), arrow(prop(), prop()))),
        ("BeilinsonLichtenbaumHolds", prop()),
        ("HasSpectralSeqConv", arrow(cst("Spectrum"), prop())),
        (
            "IsUniversal",
            arrow(cst("CommRing"), arrow(type0(), prop())),
        ),
        ("LazardRing", cst("CommRing")),
        (
            "IsUniversalFGL",
            arrow(type0(), arrow(cst("CommRing"), prop())),
        ),
        ("AlgebraicCobordismFGL", type0()),
        ("ExistsUnique", arrow(type0(), arrow(type0(), prop()))),
        ("NatTrans", arrow(type0(), arrow(type0(), type0()))),
        ("IsOrientedMap", type0()),
        ("OrientedCohomology", arrow(cst("Scheme"), type0())),
        ("LineBundleClass", type0()),
        ("CohomClass", type0()),
        ("VectorBundle", arrow(cst("Scheme"), type0())),
        ("ProjectiveBundle", arrow(type0(), cst("Scheme"))),
        ("QuotientRing", arrow(type0(), arrow(type0(), type0()))),
        ("CharPolyChern", type0()),
        ("IsCdhSquare", arrow(cst("Scheme"), prop())),
        ("KTheoryDescentHolds", arrow(cst("Scheme"), prop())),
        ("NisnevichCover", arrow(cst("Scheme"), type0())),
        ("IsExcisive", arrow(cst("Spectrum"), prop())),
        ("PuncturedA2", cst("Scheme")),
        ("MilnorWittK2", cst("AbGroup")),
        (
            "MotivicSmash",
            arrow(
                cst("MixedMotiveObj"),
                arrow(cst("MixedMotiveObj"), cst("MixedMotiveObj")),
            ),
        ),
        ("MotivicEMLSpect", arrow(cst("CommRing"), cst("Spectrum"))),
        ("ZCoeffs", cst("CommRing")),
        ("SphereSpectrum", cst("MixedMotiveObj")),
        (
            "A1HomSets",
            arrow(cst("Scheme"), arrow(cst("Scheme"), cst("AbGroup"))),
        ),
        ("HasMilnorBasis", arrow(type0(), prop())),
        (
            "WeilCohomology",
            arrow(cst("Scheme"), arrow(nat_ty(), cst("AbGroup"))),
        ),
        ("WeilCohomGraded", arrow(cst("Scheme"), cst("GradedModule"))),
        (
            "TensorGradedModule",
            arrow(
                cst("GradedModule"),
                arrow(cst("GradedModule"), cst("GradedModule")),
            ),
        ),
        ("TateTwistDual", arrow(cst("AbGroup"), cst("AbGroup"))),
        ("NatSub", arrow(nat_ty(), arrow(nat_ty(), nat_ty()))),
        ("HasFiniteDimensionality", arrow(type0(), prop())),
        ("HasPoincareDuality", arrow(type0(), prop())),
        ("HasCycleClassMap", arrow(type0(), prop())),
        ("IsProjectiveComplex", arrow(cst("Scheme"), prop())),
        ("HodgeClassesAreAlgebraic", arrow(cst("Scheme"), prop())),
        (
            "CycleClassIsSurjective",
            arrow(cst("Scheme"), arrow(cst("Prime"), prop())),
        ),
        (
            "HasMotivicDecomposition",
            arrow(cst("Scheme"), arrow(cst("MixedMotiveObj"), prop())),
        ),
        ("TwoPMinusN", nat_ty()),
        (
            "EigenspaceAdams",
            arrow(
                cst("CommRing"),
                arrow(nat_ty(), arrow(nat_ty(), cst("AbGroup"))),
            ),
        ),
    ];
    for (name, ty) in axioms {
        env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty: ty.clone(),
        })
        .ok();
    }
    Ok(())
}
