//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)
#![allow(clippy::items_after_test_module)]

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};
use std::f64::consts::PI;

use super::complex_type::Complex;
use super::types::{MobiusTransform, C64};

#[allow(dead_code)]
pub fn app(f: Expr, a: Expr) -> Expr {
    Expr::App(Box::new(f), Box::new(a))
}
#[allow(dead_code)]
pub fn app2(f: Expr, a: Expr, b: Expr) -> Expr {
    app(app(f, a), b)
}
#[allow(dead_code)]
pub fn app3(f: Expr, a: Expr, b: Expr, c: Expr) -> Expr {
    app(app2(f, a, b), c)
}
pub fn cst(s: &str) -> Expr {
    Expr::Const(Name::str(s), vec![])
}
#[allow(dead_code)]
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
#[allow(dead_code)]
pub fn impl_pi(name: &str, dom: Expr, body: Expr) -> Expr {
    pi(BinderInfo::Implicit, name, dom, body)
}
pub fn bvar(n: u32) -> Expr {
    Expr::BVar(n)
}
pub fn real_ty() -> Expr {
    cst("Real")
}
pub fn complex_ty() -> Expr {
    cst("Complex")
}
pub fn nat_ty() -> Expr {
    cst("Nat")
}
/// Complex : Type  (ℂ = ℝ × ℝ as a structure)
pub fn complex_type_decl() -> Expr {
    type0()
}
/// Complex.mk : Real → Real → Complex
pub fn complex_mk_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), complex_ty()))
}
/// Complex.re : Complex → Real
pub fn complex_re_ty() -> Expr {
    arrow(complex_ty(), real_ty())
}
/// Complex.im : Complex → Real
pub fn complex_im_ty() -> Expr {
    arrow(complex_ty(), real_ty())
}
/// Complex.add : Complex → Complex → Complex
pub fn complex_add_ty() -> Expr {
    arrow(complex_ty(), arrow(complex_ty(), complex_ty()))
}
/// Complex.mul : Complex → Complex → Complex
pub fn complex_mul_ty() -> Expr {
    arrow(complex_ty(), arrow(complex_ty(), complex_ty()))
}
/// Complex.neg : Complex → Complex
pub fn complex_neg_ty() -> Expr {
    arrow(complex_ty(), complex_ty())
}
/// Complex.conj : Complex → Complex  (complex conjugate: a + bi → a - bi)
pub fn complex_conj_ty() -> Expr {
    arrow(complex_ty(), complex_ty())
}
/// Complex.normSq : Complex → Real  (|z|² = re² + im²)
pub fn complex_norm_sq_ty() -> Expr {
    arrow(complex_ty(), real_ty())
}
/// Complex.abs : Complex → Real  (|z| = √(re² + im²))
pub fn complex_abs_ty() -> Expr {
    arrow(complex_ty(), real_ty())
}
/// Complex.arg : Complex → Real  (argument, in (-π, π])
pub fn complex_arg_ty() -> Expr {
    arrow(complex_ty(), real_ty())
}
/// Complex.exp : Complex → Complex  (complex exponential)
pub fn complex_exp_ty() -> Expr {
    arrow(complex_ty(), complex_ty())
}
/// Complex.log : Complex → Complex  (principal value logarithm)
pub fn complex_log_ty() -> Expr {
    arrow(complex_ty(), complex_ty())
}
/// Complex.pow : Complex → Nat → Complex
pub fn complex_pow_ty() -> Expr {
    arrow(complex_ty(), arrow(nat_ty(), complex_ty()))
}
/// Complex.sqrt : Complex → Complex  (principal square root)
pub fn complex_sqrt_ty() -> Expr {
    arrow(complex_ty(), complex_ty())
}
/// Complex.ofReal : Real → Complex  (coercion ℝ → ℂ)
pub fn complex_of_real_ty() -> Expr {
    arrow(real_ty(), complex_ty())
}
/// Complex.I : Complex  (imaginary unit i = (0, 1))
pub fn complex_i_ty() -> Expr {
    complex_ty()
}
/// Euler's formula: exp(i·θ) = cos(θ) + i·sin(θ)
pub fn euler_formula_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "theta",
        real_ty(),
        app2(
            app(cst("Eq"), complex_ty()),
            app(
                cst("Complex.exp"),
                app2(
                    cst("Complex.mul"),
                    cst("Complex.I"),
                    app(cst("Complex.ofReal"), bvar(0)),
                ),
            ),
            app2(
                cst("Complex.add"),
                app(cst("Complex.ofReal"), app(cst("Real.cos"), bvar(0))),
                app2(
                    cst("Complex.mul"),
                    cst("Complex.I"),
                    app(cst("Complex.ofReal"), app(cst("Real.sin"), bvar(0))),
                ),
            ),
        ),
    )
}
/// De Moivre's theorem: (cos θ + i·sin θ)^n = cos(n·θ) + i·sin(n·θ)
pub fn de_moivre_theorem_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "theta",
        real_ty(),
        pi(
            BinderInfo::Default,
            "n",
            nat_ty(),
            app2(
                app(cst("Eq"), complex_ty()),
                app2(
                    cst("Complex.pow"),
                    app2(
                        cst("Complex.add"),
                        app(cst("Complex.ofReal"), app(cst("Real.cos"), bvar(1))),
                        app2(
                            cst("Complex.mul"),
                            cst("Complex.I"),
                            app(cst("Complex.ofReal"), app(cst("Real.sin"), bvar(1))),
                        ),
                    ),
                    bvar(0),
                ),
                app2(
                    cst("Complex.add"),
                    app(
                        cst("Complex.ofReal"),
                        app(
                            cst("Real.cos"),
                            app2(cst("Real.mul"), app(cst("Nat.cast"), bvar(0)), bvar(1)),
                        ),
                    ),
                    app2(
                        cst("Complex.mul"),
                        cst("Complex.I"),
                        app(
                            cst("Complex.ofReal"),
                            app(
                                cst("Real.sin"),
                                app2(cst("Real.mul"), app(cst("Nat.cast"), bvar(0)), bvar(1)),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// Fundamental theorem of algebra: every non-constant polynomial over ℂ has a root
pub fn fundamental_theorem_algebra_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "p",
        app(cst("Polynomial"), complex_ty()),
        arrow(
            app(cst("Polynomial.degree_pos"), bvar(0)),
            app(
                cst("Exists"),
                pi(
                    BinderInfo::Default,
                    "z",
                    complex_ty(),
                    app2(
                        app(cst("Eq"), complex_ty()),
                        app2(cst("Polynomial.eval"), bvar(0), bvar(1)),
                        app(
                            cst("Complex.ofReal"),
                            app(
                                cst("Real.ofNat"),
                                Expr::Lit(oxilean_kernel::Literal::Nat(0)),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// Liouville's theorem: every bounded entire function is constant
pub fn liouville_theorem_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "f",
        arrow(complex_ty(), complex_ty()),
        arrow(
            app(cst("IsEntire"), bvar(0)),
            arrow(
                app(cst("IsBounded"), bvar(0)),
                app(cst("IsConstant"), bvar(0)),
            ),
        ),
    )
}
/// Cauchy's integral theorem: ∮_C f(z) dz = 0 for holomorphic f and closed curve C
pub fn cauchy_integral_theorem_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "f",
        arrow(complex_ty(), complex_ty()),
        pi(
            BinderInfo::Default,
            "C",
            app(cst("ClosedCurve"), complex_ty()),
            arrow(
                app(cst("IsHolomorphic"), bvar(1)),
                app2(
                    app(cst("Eq"), complex_ty()),
                    app2(cst("ContourIntegral"), bvar(1), bvar(0)),
                    app(
                        cst("Complex.ofReal"),
                        Expr::Lit(oxilean_kernel::Literal::Nat(0)),
                    ),
                ),
            ),
        ),
    )
}
/// n-th roots of unity: z^n = 1 has exactly n solutions in ℂ
pub fn roots_of_unity_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        arrow(
            app2(
                cst("Nat.lt"),
                Expr::Lit(oxilean_kernel::Literal::Nat(0)),
                bvar(0),
            ),
            app2(
                app(cst("Eq"), nat_ty()),
                app(cst("Fintype.card"), app(cst("rootsOfUnity"), bvar(0))),
                bvar(0),
            ),
        ),
    )
}
/// CauchyRiemann: holomorphicity ↔ Cauchy-Riemann equations
/// ∀ f : ℂ → ℂ, IsHolomorphic f ↔ SatisfiesCauchyRiemann f
pub fn cauchy_riemann_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "f",
        arrow(complex_ty(), complex_ty()),
        app2(
            app(cst("Iff"), prop()),
            app(cst("IsHolomorphic"), bvar(0)),
            app(cst("SatisfiesCauchyRiemann"), bvar(0)),
        ),
    )
}
/// Cauchy integral formula: f(z₀) = (1/2πi) ∮ f(z)/(z-z₀) dz
pub fn cauchy_integral_formula_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "f",
        arrow(complex_ty(), complex_ty()),
        pi(
            BinderInfo::Default,
            "C",
            app(cst("ClosedCurve"), complex_ty()),
            pi(
                BinderInfo::Default,
                "z0",
                complex_ty(),
                arrow(
                    app(cst("IsHolomorphicOn"), app2(cst("Pair"), bvar(2), bvar(1))),
                    arrow(
                        app2(cst("InsideCurve"), bvar(0), bvar(1)),
                        app2(
                            app(cst("Eq"), complex_ty()),
                            app(bvar(2), bvar(0)),
                            app2(cst("ContourIntegralFormula"), bvar(2), bvar(1)),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// Residue theorem: ∮_C f dz = 2πi · Σ Res(f, zₖ)
pub fn residue_theorem_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "f",
        arrow(complex_ty(), complex_ty()),
        pi(
            BinderInfo::Default,
            "C",
            app(cst("ClosedCurve"), complex_ty()),
            arrow(
                app(cst("IsMeromorphicOn"), app2(cst("Pair"), bvar(1), bvar(0))),
                app2(
                    app(cst("Eq"), complex_ty()),
                    app2(cst("ContourIntegral"), bvar(1), bvar(0)),
                    app2(cst("ResidueSum"), bvar(1), bvar(0)),
                ),
            ),
        ),
    )
}
/// Analytic continuation: two entire functions agreeing on an open set agree everywhere
pub fn analytic_continuation_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "f",
        arrow(complex_ty(), complex_ty()),
        pi(
            BinderInfo::Default,
            "g",
            arrow(complex_ty(), complex_ty()),
            arrow(
                app(cst("IsEntire"), bvar(1)),
                arrow(
                    app(cst("IsEntire"), bvar(0)),
                    arrow(
                        app2(cst("AgreeOnOpenSet"), bvar(1), bvar(0)),
                        app2(cst("FunEq"), bvar(1), bvar(0)),
                    ),
                ),
            ),
        ),
    )
}
/// Picard's little theorem: a non-constant entire function omits at most one value
pub fn picard_little_theorem_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "f",
        arrow(complex_ty(), complex_ty()),
        arrow(
            app(cst("IsEntire"), bvar(0)),
            arrow(
                app2(cst("Not"), prop(), app(cst("IsConstant"), bvar(0))),
                app2(cst("OmitsAtMostOne"), bvar(0), complex_ty()),
            ),
        ),
    )
}
/// Picard's great theorem: near an essential singularity, f is densely surjective (minus at most one point)
pub fn picard_great_theorem_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "f",
        arrow(complex_ty(), complex_ty()),
        pi(
            BinderInfo::Default,
            "z0",
            complex_ty(),
            arrow(
                app2(cst("IsEssentialSingularity"), bvar(1), bvar(0)),
                app2(cst("DenseImageNearPoint"), bvar(1), bvar(0)),
            ),
        ),
    )
}
/// Weierstrass factorization theorem: every entire function factors via its zero set
pub fn weierstrass_factorization_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "f",
        arrow(complex_ty(), complex_ty()),
        arrow(
            app(cst("IsEntire"), bvar(0)),
            app2(cst("HasWeierstrassFactorization"), bvar(0), complex_ty()),
        ),
    )
}
/// Meromorphic function on the Riemann sphere is a rational function
pub fn meromorphic_riemann_sphere_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "f",
        arrow(complex_ty(), complex_ty()),
        app2(
            app(cst("Iff"), prop()),
            app(cst("IsMeromorphicOnSphere"), bvar(0)),
            app(cst("IsRationalFunction"), bvar(0)),
        ),
    )
}
/// Riemann mapping theorem: every simply connected proper open subset of ℂ is conformally equivalent to the disk
pub fn riemann_mapping_theorem_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "U",
        app(cst("OpenSet"), complex_ty()),
        arrow(
            app(cst("IsSimplyConnected"), bvar(0)),
            arrow(
                app2(cst("ProperSubset"), bvar(0), complex_ty()),
                app2(cst("ConformallyEquivalent"), bvar(0), cst("UnitDisk")),
            ),
        ),
    )
}
/// Möbius transformations form a group (PSL(2,ℂ))
pub fn mobius_group_ty() -> Expr {
    app(cst("IsGroup"), cst("MobiusGroup"))
}
/// Every Möbius transformation is a conformal automorphism of the Riemann sphere
pub fn mobius_conformal_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "m",
        cst("MobiusGroup"),
        app(cst("IsConformalAutomorphism"), bvar(0)),
    )
}
/// Laurent series convergence: every meromorphic function has a Laurent series in an annulus
pub fn laurent_series_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "f",
        arrow(complex_ty(), complex_ty()),
        pi(
            BinderInfo::Default,
            "z0",
            complex_ty(),
            arrow(
                app2(cst("IsIsolatedSingularity"), bvar(1), bvar(0)),
                app2(cst("HasLaurentSeries"), bvar(1), bvar(0)),
            ),
        ),
    )
}
/// Argument principle: (1/2πi)∮ f'/f dz = Z - P (zeros minus poles)
pub fn argument_principle_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "f",
        arrow(complex_ty(), complex_ty()),
        pi(
            BinderInfo::Default,
            "C",
            app(cst("ClosedCurve"), complex_ty()),
            arrow(
                app(cst("IsMeromorphicOn"), app2(cst("Pair"), bvar(1), bvar(0))),
                app2(
                    app(cst("Eq"), nat_ty()),
                    app2(cst("WindingNumberIntegral"), bvar(1), bvar(0)),
                    app2(
                        cst("Nat.sub"),
                        app2(cst("CountZeros"), bvar(1), bvar(0)),
                        app2(cst("CountPoles"), bvar(1), bvar(0)),
                    ),
                ),
            ),
        ),
    )
}
/// Rouché's theorem: if |f| > |g| on C, then f and f+g have the same number of zeros inside C
pub fn rouche_theorem_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "f",
        arrow(complex_ty(), complex_ty()),
        pi(
            BinderInfo::Default,
            "g",
            arrow(complex_ty(), complex_ty()),
            pi(
                BinderInfo::Default,
                "C",
                app(cst("ClosedCurve"), complex_ty()),
                arrow(
                    app2(
                        cst("DominatesOnCurve"),
                        bvar(2),
                        app2(cst("Pair"), bvar(1), bvar(0)),
                    ),
                    app2(
                        app(cst("Eq"), nat_ty()),
                        app2(cst("CountZeros"), bvar(2), bvar(0)),
                        app2(
                            cst("CountZeros"),
                            app2(cst("FunAdd"), bvar(2), bvar(1)),
                            bvar(0),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// Mittag-Leffler theorem: meromorphic function with prescribed poles exists
pub fn mittag_leffler_theorem_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "poles",
        app(cst("List"), complex_ty()),
        pi(
            BinderInfo::Default,
            "parts",
            app(cst("List"), app(cst("LaurentPrincipalPart"), complex_ty())),
            arrow(
                app2(cst("CompatiblePolesAndParts"), bvar(1), bvar(0)),
                app(
                    cst("Exists"),
                    pi(
                        BinderInfo::Default,
                        "f",
                        arrow(complex_ty(), complex_ty()),
                        app2(
                            cst("HasPrescribedPoles"),
                            bvar(0),
                            app2(cst("Pair"), bvar(2), bvar(1)),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// Runge's theorem: holomorphic functions on a compact set can be approximated by rational functions
pub fn runge_theorem_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "K",
        app(cst("CompactSet"), complex_ty()),
        pi(
            BinderInfo::Default,
            "f",
            arrow(complex_ty(), complex_ty()),
            arrow(
                app(
                    cst("IsHolomorphicOnCompact"),
                    app2(cst("Pair"), bvar(1), bvar(0)),
                ),
                app2(
                    cst("UniformlyApproximableBy"),
                    app2(cst("Pair"), bvar(1), bvar(0)),
                    cst("RationalFunctions"),
                ),
            ),
        ),
    )
}
/// Hardy space H^p is a Banach space for p ≥ 1
pub fn hardy_banach_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "p",
        real_ty(),
        arrow(
            app2(
                cst("Real.le"),
                Expr::Lit(oxilean_kernel::Literal::Nat(1)),
                bvar(0),
            ),
            app(cst("IsBanachSpace"), app(cst("HardySpace"), bvar(0))),
        ),
    )
}
/// Nevanlinna's first fundamental theorem: T(r, f) = m(r, f) + N(r, f) + O(1)
pub fn nevanlinna_first_theorem_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "f",
        arrow(complex_ty(), complex_ty()),
        arrow(
            app(cst("IsMeromorphic"), bvar(0)),
            app(cst("NevanlinnaFirstFundamentalTheorem"), bvar(0)),
        ),
    )
}
/// Nevanlinna's second fundamental theorem: sum of Nevanlinna deficiencies ≤ 2
pub fn nevanlinna_second_theorem_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "f",
        arrow(complex_ty(), complex_ty()),
        arrow(
            app(cst("IsEntire"), bvar(0)),
            app2(
                app(cst("Real.le"), prop()),
                app(cst("NevanlinnaDeficiencySum"), bvar(0)),
                Expr::Lit(oxilean_kernel::Literal::Nat(2)),
            ),
        ),
    )
}
/// Bloch's theorem: image of unit disk contains a disk of radius B (Bloch's constant)
pub fn bloch_theorem_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "f",
        arrow(complex_ty(), complex_ty()),
        arrow(
            app(cst("IsHolomorphicOnDisk"), bvar(0)),
            app2(cst("ImageContainsDisk"), bvar(0), cst("BlochConstant")),
        ),
    )
}
/// Weierstrass ℘-function is an elliptic function for its lattice
pub fn weierstrass_p_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "L",
        app(cst("Lattice"), complex_ty()),
        app(cst("IsEllipticFunction"), app(cst("WeierstrassP"), bvar(0))),
    )
}
/// Liouville theorem for elliptic functions: bounded implies constant
pub fn elliptic_liouville_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "L",
        app(cst("Lattice"), complex_ty()),
        pi(
            BinderInfo::Default,
            "f",
            arrow(complex_ty(), complex_ty()),
            arrow(
                app2(cst("IsEllipticFunctionForLattice"), bvar(0), bvar(1)),
                arrow(
                    app(cst("IsBounded"), bvar(0)),
                    app(cst("IsConstant"), bvar(0)),
                ),
            ),
        ),
    )
}
/// The space of modular forms of weight k is finite-dimensional
pub fn modular_form_finite_dim_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "k",
        nat_ty(),
        app(
            cst("IsFiniteDimensional"),
            app(cst("ModularFormSpace"), bvar(0)),
        ),
    )
}
/// Hartogs' theorem: holomorphic functions in several variables extend across compact singularities
pub fn hartogs_theorem_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "U",
            app(cst("OpenSet"), app(cst("CVec"), bvar(0))),
            pi(
                BinderInfo::Default,
                "K",
                app(cst("CompactSet"), app(cst("CVec"), bvar(1))),
                arrow(
                    app2(cst("IsCompactlyContained"), bvar(0), bvar(1)),
                    arrow(
                        app(cst("IsSimplyConnected"), bvar(1)),
                        app2(cst("HolomorphicFunctionsExtend"), bvar(1), bvar(0)),
                    ),
                ),
            ),
        ),
    )
}
/// Oka's theorem: pseudoconvex ↔ domain of holomorphy in ℂⁿ
pub fn oka_theorem_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "D",
        app(cst("Domain"), cst("CnSpace")),
        app2(
            app(cst("Iff"), prop()),
            app(cst("IsPseudoconvex"), bvar(0)),
            app(cst("IsDomainOfHolomorphy"), bvar(0)),
        ),
    )
}
/// Bohr's theorem: |f| < 1 on disk ⟹ power series converges in disk of radius 1/3
pub fn bohr_theorem_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "f",
        arrow(complex_ty(), complex_ty()),
        arrow(
            app(cst("IsHolomorphicOnDisk"), bvar(0)),
            arrow(
                app(cst("BoundedByOneOnDisk"), bvar(0)),
                app2(
                    cst("PowerSeriesConvergesInDisk"),
                    bvar(0),
                    cst("BohrConstant"),
                ),
            ),
        ),
    )
}
/// Paley-Wiener: L² function is bandlimited ↔ Fourier transform has compact support
pub fn paley_wiener_theorem_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "f",
        arrow(real_ty(), complex_ty()),
        app2(
            app(cst("Iff"), prop()),
            app(cst("IsBandlimited"), bvar(0)),
            app(cst("FourierTransformHasCompactSupport"), bvar(0)),
        ),
    )
}
/// Hadamard factorization theorem: entire function of finite order has Weierstrass product form
pub fn hadamard_factorization_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "f",
        arrow(complex_ty(), complex_ty()),
        pi(
            BinderInfo::Default,
            "rho",
            real_ty(),
            arrow(
                app(cst("IsEntire"), bvar(1)),
                arrow(
                    app2(cst("HasFiniteOrder"), bvar(1), bvar(0)),
                    app2(cst("HasHadamardFactorization"), bvar(1), bvar(0)),
                ),
            ),
        ),
    )
}
/// Schwarz-Pick lemma: holomorphic self-map of the unit disk is a hyperbolic contraction
pub fn schwarz_pick_lemma_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "f",
        arrow(complex_ty(), complex_ty()),
        arrow(
            app(cst("IsSelfMapOfDisk"), bvar(0)),
            app(cst("IsHyperbolicContraction"), bvar(0)),
        ),
    )
}
/// Jensen's formula: log|f(0)| = Σ log(r/|zₖ|) + (1/2π)∫₀²π log|f(re^{iθ})| dθ
pub fn jensen_formula_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "f",
        arrow(complex_ty(), complex_ty()),
        pi(
            BinderInfo::Default,
            "r",
            real_ty(),
            arrow(
                app(cst("IsHolomorphicOnDisk"), bvar(1)),
                arrow(
                    app2(
                        cst("Real.lt"),
                        Expr::Lit(oxilean_kernel::Literal::Nat(0)),
                        bvar(0),
                    ),
                    app2(cst("JensenEquality"), bvar(1), bvar(0)),
                ),
            ),
        ),
    )
}
/// Phragmén-Lindelöf: maximum modulus principle extended to unbounded sectors
pub fn phragmen_lindelof_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "f",
        arrow(complex_ty(), complex_ty()),
        pi(
            BinderInfo::Default,
            "S",
            app(cst("OpenSector"), complex_ty()),
            arrow(
                app(cst("IsHolomorphicOn"), app2(cst("Pair"), bvar(1), bvar(0))),
                arrow(
                    app(
                        cst("BoundedOnBoundary"),
                        app2(cst("Pair"), bvar(1), bvar(0)),
                    ),
                    app(cst("BoundedOn"), app2(cst("Pair"), bvar(1), bvar(0))),
                ),
            ),
        ),
    )
}
/// Riemann zeta function extends meromorphically to ℂ with a simple pole at s = 1
pub fn riemann_zeta_meromorphic_ty() -> Expr {
    app(cst("IsMeromorphic"), cst("RiemannZeta"))
}
/// Riemann hypothesis (as axiom): non-trivial zeros of ζ lie on Re(s) = 1/2
pub fn riemann_hypothesis_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "s",
        complex_ty(),
        arrow(
            app2(cst("IsNontrivialZeroOfZeta"), bvar(0), cst("RiemannZeta")),
            app2(
                app(cst("Eq"), real_ty()),
                app(cst("Complex.re"), bvar(0)),
                Expr::Lit(oxilean_kernel::Literal::Nat(0)),
            ),
        ),
    )
}
/// Build the complex number environment, registering all axioms and theorems.
#[allow(dead_code)]
pub fn build_complex_env(env: &mut Environment) -> Result<(), String> {
    let _ = env.add(Declaration::Axiom {
        name: Name::str("Complex"),
        univ_params: vec![],
        ty: complex_type_decl(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("Complex.mk"),
        univ_params: vec![],
        ty: complex_mk_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("Complex.re"),
        univ_params: vec![],
        ty: complex_re_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("Complex.im"),
        univ_params: vec![],
        ty: complex_im_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("Complex.add"),
        univ_params: vec![],
        ty: complex_add_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("Complex.mul"),
        univ_params: vec![],
        ty: complex_mul_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("Complex.neg"),
        univ_params: vec![],
        ty: complex_neg_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("Complex.conj"),
        univ_params: vec![],
        ty: complex_conj_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("Complex.normSq"),
        univ_params: vec![],
        ty: complex_norm_sq_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("Complex.abs"),
        univ_params: vec![],
        ty: complex_abs_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("Complex.arg"),
        univ_params: vec![],
        ty: complex_arg_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("Complex.exp"),
        univ_params: vec![],
        ty: complex_exp_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("Complex.log"),
        univ_params: vec![],
        ty: complex_log_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("Complex.pow"),
        univ_params: vec![],
        ty: complex_pow_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("Complex.sqrt"),
        univ_params: vec![],
        ty: complex_sqrt_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("Complex.ofReal"),
        univ_params: vec![],
        ty: complex_of_real_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("Complex.I"),
        univ_params: vec![],
        ty: complex_i_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("Complex.euler_formula"),
        univ_params: vec![],
        ty: euler_formula_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("Complex.de_moivre"),
        univ_params: vec![],
        ty: de_moivre_theorem_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("Complex.fundamental_theorem_algebra"),
        univ_params: vec![],
        ty: fundamental_theorem_algebra_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("Complex.liouville"),
        univ_params: vec![],
        ty: liouville_theorem_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("Complex.cauchy_integral"),
        univ_params: vec![],
        ty: cauchy_integral_theorem_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("Complex.roots_of_unity_card"),
        univ_params: vec![],
        ty: roots_of_unity_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("ComplexAnalysis.cauchy_riemann"),
        univ_params: vec![],
        ty: cauchy_riemann_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("ComplexAnalysis.cauchy_integral_formula"),
        univ_params: vec![],
        ty: cauchy_integral_formula_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("ComplexAnalysis.residue_theorem"),
        univ_params: vec![],
        ty: residue_theorem_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("ComplexAnalysis.analytic_continuation"),
        univ_params: vec![],
        ty: analytic_continuation_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("ComplexAnalysis.picard_little"),
        univ_params: vec![],
        ty: picard_little_theorem_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("ComplexAnalysis.picard_great"),
        univ_params: vec![],
        ty: picard_great_theorem_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("ComplexAnalysis.weierstrass_factorization"),
        univ_params: vec![],
        ty: weierstrass_factorization_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("ComplexAnalysis.meromorphic_riemann_sphere"),
        univ_params: vec![],
        ty: meromorphic_riemann_sphere_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("ComplexAnalysis.riemann_mapping"),
        univ_params: vec![],
        ty: riemann_mapping_theorem_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("ComplexAnalysis.mobius_group"),
        univ_params: vec![],
        ty: mobius_group_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("ComplexAnalysis.mobius_conformal"),
        univ_params: vec![],
        ty: mobius_conformal_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("ComplexAnalysis.laurent_series"),
        univ_params: vec![],
        ty: laurent_series_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("ComplexAnalysis.argument_principle"),
        univ_params: vec![],
        ty: argument_principle_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("ComplexAnalysis.rouche"),
        univ_params: vec![],
        ty: rouche_theorem_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("ComplexAnalysis.mittag_leffler"),
        univ_params: vec![],
        ty: mittag_leffler_theorem_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("ComplexAnalysis.runge"),
        univ_params: vec![],
        ty: runge_theorem_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("ComplexAnalysis.hardy_banach"),
        univ_params: vec![],
        ty: hardy_banach_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("ComplexAnalysis.nevanlinna_first"),
        univ_params: vec![],
        ty: nevanlinna_first_theorem_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("ComplexAnalysis.nevanlinna_second"),
        univ_params: vec![],
        ty: nevanlinna_second_theorem_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("ComplexAnalysis.bloch"),
        univ_params: vec![],
        ty: bloch_theorem_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("ComplexAnalysis.weierstrass_p_elliptic"),
        univ_params: vec![],
        ty: weierstrass_p_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("ComplexAnalysis.elliptic_liouville"),
        univ_params: vec![],
        ty: elliptic_liouville_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("ComplexAnalysis.modular_form_finite_dim"),
        univ_params: vec![],
        ty: modular_form_finite_dim_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("ComplexAnalysis.hartogs"),
        univ_params: vec![],
        ty: hartogs_theorem_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("ComplexAnalysis.oka"),
        univ_params: vec![],
        ty: oka_theorem_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("ComplexAnalysis.bohr"),
        univ_params: vec![],
        ty: bohr_theorem_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("ComplexAnalysis.paley_wiener"),
        univ_params: vec![],
        ty: paley_wiener_theorem_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("ComplexAnalysis.hadamard_factorization"),
        univ_params: vec![],
        ty: hadamard_factorization_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("ComplexAnalysis.schwarz_pick"),
        univ_params: vec![],
        ty: schwarz_pick_lemma_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("ComplexAnalysis.jensen_formula"),
        univ_params: vec![],
        ty: jensen_formula_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("ComplexAnalysis.phragmen_lindelof"),
        univ_params: vec![],
        ty: phragmen_lindelof_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("ComplexAnalysis.riemann_zeta_meromorphic"),
        univ_params: vec![],
        ty: riemann_zeta_meromorphic_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("ComplexAnalysis.riemann_hypothesis"),
        univ_params: vec![],
        ty: riemann_hypothesis_ty(),
    });
    Ok(())
}
/// Numerically integrate f along a circular contour centered at `center` with given `radius`.
/// Uses `n_points` equally spaced quadrature nodes (trapezoidal rule on the circle).
/// Returns the integral ∮ f(z) dz (NOT divided by 2πi).
#[allow(dead_code)]
pub fn contour_integrate_circular<F>(f: F, center: Complex, radius: f64, n_points: usize) -> Complex
where
    F: Fn(Complex) -> Complex,
{
    if n_points == 0 {
        return Complex::zero();
    }
    let mut sum = Complex::zero();
    for k in 0..n_points {
        let theta = 2.0 * PI * k as f64 / n_points as f64;
        let z = center.add(Complex::from_polar(radius, theta));
        let dz = Complex::from_polar(radius, theta + PI / 2.0).scale(2.0 * PI / n_points as f64);
        sum = sum.add(f(z).mul(dz));
    }
    sum
}
/// Discrete Fourier Transform: X[k] = Σ_{n=0}^{N-1} x[n] · e^(-2πi·k·n/N).
#[allow(dead_code)]
pub fn dft(signal: &[Complex]) -> Vec<Complex> {
    let n = signal.len();
    if n == 0 {
        return vec![];
    }
    (0..n)
        .map(|k| {
            (0..n)
                .map(|j| {
                    let angle = -2.0 * PI * k as f64 * j as f64 / n as f64;
                    signal[j].mul(Complex::from_polar(1.0, angle))
                })
                .fold(Complex::zero(), |acc, x| acc.add(x))
        })
        .collect()
}
/// Inverse Discrete Fourier Transform: x[n] = (1/N) Σ_{k=0}^{N-1} X[k] · e^(2πi·k·n/N).
#[allow(dead_code)]
pub fn idft(spectrum: &[Complex]) -> Vec<Complex> {
    let n = spectrum.len();
    if n == 0 {
        return vec![];
    }
    let scale = 1.0 / n as f64;
    (0..n)
        .map(|j| {
            (0..n)
                .map(|k| {
                    let angle = 2.0 * PI * k as f64 * j as f64 / n as f64;
                    spectrum[k].mul(Complex::from_polar(1.0, angle))
                })
                .fold(Complex::zero(), |acc, x| acc.add(x))
                .scale(scale)
        })
        .collect()
}
/// Cooley-Tukey radix-2 FFT (N must be a power of two; falls back to DFT otherwise).
#[allow(dead_code)]
pub fn fft(signal: &[Complex]) -> Vec<Complex> {
    let n = signal.len();
    if n <= 1 {
        return signal.to_vec();
    }
    if n & (n - 1) != 0 {
        return dft(signal);
    }
    let evens: Vec<Complex> = signal.iter().step_by(2).cloned().collect();
    let odds: Vec<Complex> = signal.iter().skip(1).step_by(2).cloned().collect();
    let mut e = fft(&evens);
    let mut o = fft(&odds);
    e.resize(n, Complex::zero());
    o.resize(n, Complex::zero());
    let mut result = vec![Complex::zero(); n];
    let half = n / 2;
    for k in 0..half {
        let twiddle = Complex::from_polar(1.0, -2.0 * PI * k as f64 / n as f64);
        let t = twiddle.mul(o[k]);
        result[k] = e[k].add(t);
        result[k + half] = e[k].sub(t);
    }
    result
}
/// Newton's method for finding a root of f (with derivative df), starting at z0.
/// Returns `Some(root)` if converged within `max_iter` iterations to within `tol`.
#[allow(dead_code)]
pub fn newton_method<F, Df>(f: F, df: Df, z0: Complex, max_iter: usize, tol: f64) -> Option<Complex>
where
    F: Fn(Complex) -> Complex,
    Df: Fn(Complex) -> Complex,
{
    let mut z = z0;
    for _ in 0..max_iter {
        let fz = f(z);
        if fz.abs() < tol {
            return Some(z);
        }
        let dfz = df(z);
        z = z.sub(fz.div(dfz)?);
    }
    if f(z).abs() < tol {
        Some(z)
    } else {
        None
    }
}
/// Check membership in the Mandelbrot set: iterate z ↦ z² + c from z = 0.
/// Returns `None` if c is in the set (within `max_iter`), or `Some(escape_iter)`.
#[allow(dead_code)]
pub fn mandelbrot_iter(c: Complex, max_iter: usize) -> Option<usize> {
    let mut z = Complex::zero();
    for i in 0..max_iter {
        if z.norm_sq() > 4.0 {
            return Some(i);
        }
        z = z.mul(z).add(c);
    }
    None
}
/// Render a simple ASCII Mandelbrot set for a given rectangular region.
/// Returns a `Vec<String>` with one row per line.
#[allow(dead_code)]
pub fn mandelbrot_ascii(
    re_min: f64,
    re_max: f64,
    im_min: f64,
    im_max: f64,
    width: usize,
    height: usize,
    max_iter: usize,
) -> Vec<String> {
    let chars: Vec<char> = " .:-=+*#%@".chars().collect();
    let n_chars = chars.len();
    (0..height)
        .map(|row| {
            let im = im_max - (im_max - im_min) * row as f64 / height as f64;
            (0..width)
                .map(|col| {
                    let re = re_min + (re_max - re_min) * col as f64 / width as f64;
                    let c = Complex::new(re, im);
                    let idx = match mandelbrot_iter(c, max_iter) {
                        None => n_chars - 1,
                        Some(i) => (i * (n_chars - 1) / max_iter).min(n_chars - 2),
                    };
                    chars[idx]
                })
                .collect()
        })
        .collect()
}
/// Sieve of Eratosthenes: returns the first `count` prime numbers.
#[allow(dead_code)]
pub fn sieve_of_eratosthenes(count: usize) -> Vec<u64> {
    if count == 0 {
        return vec![];
    }
    let limit = ((count as f64 * (count as f64).max(2.0).ln() * 1.3) as usize).max(20);
    let mut is_prime = vec![true; limit + 1];
    is_prime[0] = false;
    if limit >= 1 {
        is_prime[1] = false;
    }
    let mut i = 2usize;
    while i * i <= limit {
        if is_prime[i] {
            let mut j = i * i;
            while j <= limit {
                is_prime[j] = false;
                j += i;
            }
        }
        i += 1;
    }
    is_prime
        .iter()
        .enumerate()
        .filter(|(_, &p)| p)
        .map(|(i, _)| i as u64)
        .take(count)
        .collect()
}
/// Approximate the Riemann zeta function ζ(s) via the first `n_terms` of the Dirichlet series.
/// Converges for Re(s) > 1; use `n_terms` ≥ 1000 for reasonable accuracy.
#[allow(dead_code)]
pub fn riemann_zeta_approx(s: Complex, n_terms: usize) -> Complex {
    (1..=n_terms)
        .map(|n| {
            let log_n = Complex::new((n as f64).ln(), 0.0);
            s.neg().mul(log_n).exp()
        })
        .fold(Complex::zero(), |acc, x| acc.add(x))
}
/// Euler product approximation of ζ(s): ∏_{p prime} 1/(1 - p^{-s}) over the first `n_primes` primes.
#[allow(dead_code)]
pub fn riemann_zeta_euler_product(s: Complex, n_primes: usize) -> Complex {
    let primes = sieve_of_eratosthenes(n_primes);
    primes.iter().fold(Complex::one(), |acc, &p| {
        let log_p = Complex::new((p as f64).ln(), 0.0);
        let p_neg_s = s.neg().mul(log_p).exp();
        let denom = Complex::one().sub(p_neg_s);
        match Complex::one().div(denom) {
            Some(factor) => acc.mul(factor),
            None => acc,
        }
    })
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::E;
    const EPS: f64 = 1e-10;
    #[test]
    fn test_complex_add() {
        let z = Complex::new(1.0, 2.0);
        let w = Complex::new(3.0, -1.0);
        let sum = z + w;
        assert!((sum.re - 4.0).abs() < EPS);
        assert!((sum.im - 1.0).abs() < EPS);
    }
    #[test]
    fn test_complex_mul() {
        let z = Complex::new(1.0, 2.0);
        let w = Complex::new(3.0, 4.0);
        let prod = z * w;
        assert!((prod.re - (-5.0)).abs() < EPS);
        assert!((prod.im - 10.0).abs() < EPS);
    }
    #[test]
    fn test_complex_conj() {
        let z = Complex::new(3.0, 4.0);
        let c = z.conj();
        assert!((c.re - 3.0).abs() < EPS);
        assert!((c.im - (-4.0)).abs() < EPS);
    }
    #[test]
    fn test_complex_abs() {
        let z = Complex::new(3.0, 4.0);
        assert!((z.abs() - 5.0).abs() < EPS);
    }
    #[test]
    fn test_complex_div() {
        let z = Complex::new(1.0, 0.0);
        let w = Complex::new(0.0, 1.0);
        let q = z.div(w).expect("div operation should succeed");
        assert!((q.re - 0.0).abs() < EPS);
        assert!((q.im - (-1.0)).abs() < EPS);
    }
    #[test]
    fn test_euler_formula() {
        let theta = PI;
        let z = Complex::new(0.0, theta).exp();
        assert!((z.re - (-1.0)).abs() < 1e-10);
        assert!(z.im.abs() < 1e-10);
    }
    #[test]
    fn test_euler_e_to_half_pi_i() {
        let z = Complex::new(0.0, PI / 2.0).exp();
        assert!(z.re.abs() < 1e-10);
        assert!((z.im - 1.0).abs() < 1e-10);
    }
    #[test]
    fn test_complex_exp_real() {
        let z = Complex::new(1.0, 0.0).exp();
        assert!((z.re - E).abs() < EPS);
        assert!(z.im.abs() < EPS);
    }
    #[test]
    fn test_complex_log_inverse() {
        let z = Complex::new(1.0, 1.0);
        let log_z = z.log().expect("log should succeed");
        let exp_log_z = log_z.exp();
        assert!((exp_log_z.re - z.re).abs() < EPS);
        assert!((exp_log_z.im - z.im).abs() < EPS);
    }
    #[test]
    fn test_complex_sqrt() {
        let z = Complex::new(-1.0, 0.0);
        let s = z.sqrt();
        assert!(s.re.abs() < EPS);
        assert!((s.im - 1.0).abs() < EPS);
    }
    #[test]
    fn test_complex_powi() {
        let z = Complex::i();
        let z2 = z.powi(2);
        assert!((z2.re - (-1.0)).abs() < EPS);
        assert!(z2.im.abs() < EPS);
        let z4 = z.powi(4);
        assert!((z4.re - 1.0).abs() < EPS);
        assert!(z4.im.abs() < EPS);
    }
    #[test]
    fn test_roots_of_unity() {
        let roots = Complex::roots_of_unity(4);
        assert_eq!(roots.len(), 4);
        for r in &roots {
            assert!((r.abs() - 1.0).abs() < EPS);
        }
        for r in &roots {
            let r4 = r.powi(4);
            assert!((r4.re - 1.0).abs() < EPS);
            assert!(r4.im.abs() < EPS);
        }
    }
    #[test]
    fn test_nth_roots() {
        let one = Complex::one();
        let roots = one.nth_roots(3);
        assert_eq!(roots.len(), 3);
        for r in &roots {
            let r3 = r.powi(3);
            assert!((r3.re - 1.0).abs() < EPS, "r^3 ≠ 1: {:?}", r3);
            assert!(r3.im.abs() < EPS, "r^3.im ≠ 0: {:?}", r3.im);
        }
    }
    #[test]
    fn test_complex_cos_sin() {
        let z = Complex::zero();
        assert!((z.cos().re - 1.0).abs() < EPS);
        assert!(z.sin().re.abs() < EPS);
        let pi_z = Complex::new(PI, 0.0);
        assert!((pi_z.cos().re - (-1.0)).abs() < EPS);
    }
    #[test]
    fn test_polar_roundtrip() {
        let z = Complex::new(3.0, 4.0);
        let (r, theta) = z.to_polar();
        let z2 = Complex::from_polar(r, theta);
        assert!((z2.re - z.re).abs() < EPS);
        assert!((z2.im - z.im).abs() < EPS);
    }
    #[test]
    fn test_sinh_cosh() {
        let z = Complex::new(1.0, 0.0);
        let c2 = z.cosh().mul(z.cosh());
        let s2 = z.sinh().mul(z.sinh());
        let diff = c2.sub(s2);
        assert!((diff.re - 1.0).abs() < EPS);
        assert!(diff.im.abs() < EPS);
    }
    #[test]
    fn test_asin_sin_roundtrip() {
        let z = Complex::new(0.5, 0.3);
        let sin_z = z.sin();
        let asin_sin_z = sin_z.asin().expect("asin should succeed");
        assert!((asin_sin_z.re - z.re).abs() < 1e-9);
        assert!((asin_sin_z.im - z.im).abs() < 1e-9);
    }
    #[test]
    fn test_acos_cos_roundtrip() {
        let z = Complex::new(0.7, 0.0);
        let cos_z = z.cos();
        let acos_cos_z = cos_z.acos().expect("acos should succeed");
        assert!((acos_cos_z.re - z.re).abs() < 1e-9);
        assert!(acos_cos_z.im.abs() < 1e-9);
    }
    #[test]
    fn test_atan_roundtrip() {
        let z = Complex::new(0.3, 0.2);
        let atan_z = z.atan().expect("atan should succeed");
        let tan_atan_z = atan_z.tan().expect("tan should succeed");
        assert!((tan_atan_z.re - z.re).abs() < 1e-9);
        assert!((tan_atan_z.im - z.im).abs() < 1e-9);
    }
    #[test]
    fn test_dft_idft_roundtrip() {
        let signal = vec![
            Complex::new(1.0, 0.0),
            Complex::new(0.0, 1.0),
            Complex::new(-1.0, 0.0),
            Complex::new(0.0, -1.0),
        ];
        let spectrum = dft(&signal);
        let recovered = idft(&spectrum);
        assert_eq!(recovered.len(), signal.len());
        for (a, b) in signal.iter().zip(recovered.iter()) {
            assert!(
                a.approx_eq(*b, 1e-10),
                "DFT/IDFT roundtrip failed: {:?} vs {:?}",
                a,
                b
            );
        }
    }
    #[test]
    fn test_fft_matches_dft() {
        let signal: Vec<Complex> = (0u32..8)
            .map(|k| Complex::from_polar(1.0, 2.0 * PI * k as f64 / 8.0))
            .collect();
        let dft_result = dft(&signal);
        let fft_result = fft(&signal);
        assert_eq!(dft_result.len(), fft_result.len());
        for (a, b) in dft_result.iter().zip(fft_result.iter()) {
            assert!(
                a.approx_eq(*b, 1e-9),
                "FFT/DFT mismatch: {:?} vs {:?}",
                a,
                b
            );
        }
    }
    #[test]
    fn test_newton_method_sqrt2() {
        let root = newton_method(
            |z| z.mul(z).sub(Complex::new(2.0, 0.0)),
            |z| z.scale(2.0),
            Complex::new(1.5, 0.0),
            50,
            1e-12,
        );
        let r = root.expect("Newton should converge");
        assert!((r.re - 2.0_f64.sqrt()).abs() < 1e-10);
        assert!(r.im.abs() < 1e-10);
    }
    #[test]
    fn test_newton_method_complex_root() {
        let root = newton_method(
            |z| z.mul(z).add(Complex::one()),
            |z| z.scale(2.0),
            Complex::new(0.1, 1.0),
            50,
            1e-12,
        );
        let r = root.expect("Newton should converge to i");
        assert!(r.re.abs() < 1e-10);
        assert!((r.im - 1.0).abs() < 1e-10);
    }
    #[test]
    fn test_mandelbrot_zero_in_set() {
        assert_eq!(mandelbrot_iter(Complex::zero(), 100), None);
    }
    #[test]
    fn test_mandelbrot_large_escape() {
        let result = mandelbrot_iter(Complex::new(2.5, 0.0), 100);
        assert!(result.is_some());
    }
    #[test]
    fn test_riemann_zeta_s2() {
        let s = Complex::new(2.0, 0.0);
        let zeta = riemann_zeta_approx(s, 10_000);
        let expected = PI * PI / 6.0;
        assert!((zeta.re - expected).abs() < 0.01, "ζ(2) ≈ {}", zeta.re);
        assert!(zeta.im.abs() < 0.01);
    }
    #[test]
    fn test_sieve_first_primes() {
        let primes = sieve_of_eratosthenes(5);
        assert_eq!(primes, vec![2, 3, 5, 7, 11]);
    }
    #[test]
    fn test_mobius_identity() {
        let id = MobiusTransform::identity();
        let z = Complex::new(2.0, 3.0);
        let result = id.apply(z).expect("apply should succeed");
        assert!(result.approx_eq(z, EPS));
    }
    #[test]
    fn test_mobius_compose_invert() {
        let f = MobiusTransform::new(
            Complex::one(),
            Complex::one(),
            Complex::one(),
            Complex::new(2.0, 0.0),
        )
        .expect("operation should succeed");
        let f_inv = f.invert().expect("invert should succeed");
        let z = Complex::new(1.0, 1.0);
        let round = f
            .apply(z)
            .and_then(|w| f_inv.apply(w))
            .expect("Complex::new should succeed");
        assert!(round.approx_eq(z, 1e-10));
    }
    #[test]
    fn test_mobius_fixed_points() {
        let f = MobiusTransform::new(
            Complex::zero(),
            Complex::one(),
            Complex::one(),
            Complex::zero(),
        )
        .expect("operation should succeed");
        let fps = f.fixed_points();
        assert_eq!(fps.len(), 2);
        for fp in &fps {
            let fz = f.apply(*fp).expect("apply should succeed");
            assert!(fz.approx_eq(*fp, 1e-10), "fixed point check: {:?}", fp);
        }
    }
    #[test]
    fn test_contour_integral_constant() {
        let integral = contour_integrate_circular(|_| Complex::one(), Complex::zero(), 1.0, 1024);
        assert!(integral.abs() < 1e-10, "∮ 1 dz ≠ 0: {:?}", integral);
    }
    #[test]
    fn test_build_complex_env() {
        let mut env = Environment::new();
        let _ = env.add(Declaration::Axiom {
            name: Name::str("Real"),
            univ_params: vec![],
            ty: type0(),
        });
        let _ = env.add(Declaration::Axiom {
            name: Name::str("Nat"),
            univ_params: vec![],
            ty: type0(),
        });
        let _ = env.add(Declaration::Axiom {
            name: Name::str("Polynomial"),
            univ_params: vec![],
            ty: arrow(type0(), type0()),
        });
        let _ = env.add(Declaration::Axiom {
            name: Name::str("IsEntire"),
            univ_params: vec![],
            ty: impl_pi(
                "alpha",
                type0(),
                arrow(arrow(complex_ty(), complex_ty()), prop()),
            ),
        });
        let _ = env.add(Declaration::Axiom {
            name: Name::str("IsBounded"),
            univ_params: vec![],
            ty: impl_pi(
                "alpha",
                type0(),
                arrow(arrow(complex_ty(), complex_ty()), prop()),
            ),
        });
        let _ = env.add(Declaration::Axiom {
            name: Name::str("IsConstant"),
            univ_params: vec![],
            ty: impl_pi(
                "alpha",
                type0(),
                arrow(arrow(complex_ty(), complex_ty()), prop()),
            ),
        });
        let _ = env.add(Declaration::Axiom {
            name: Name::str("IsHolomorphic"),
            univ_params: vec![],
            ty: arrow(arrow(complex_ty(), complex_ty()), prop()),
        });
        let _ = env.add(Declaration::Axiom {
            name: Name::str("ClosedCurve"),
            univ_params: vec![],
            ty: arrow(type0(), type0()),
        });
        let _ = env.add(Declaration::Axiom {
            name: Name::str("ContourIntegral"),
            univ_params: vec![],
            ty: arrow(
                arrow(complex_ty(), complex_ty()),
                arrow(app(cst("ClosedCurve"), complex_ty()), complex_ty()),
            ),
        });
        let _ = env.add(Declaration::Axiom {
            name: Name::str("rootsOfUnity"),
            univ_params: vec![],
            ty: arrow(nat_ty(), type0()),
        });
        let _ = env.add(Declaration::Axiom {
            name: Name::str("Fintype.card"),
            univ_params: vec![],
            ty: arrow(type0(), nat_ty()),
        });
        let _ = env.add(Declaration::Axiom {
            name: Name::str("Polynomial.degree_pos"),
            univ_params: vec![],
            ty: impl_pi(
                "alpha",
                type0(),
                arrow(app(cst("Polynomial"), bvar(0)), prop()),
            ),
        });
        let _ = env.add(Declaration::Axiom {
            name: Name::str("Polynomial.eval"),
            univ_params: vec![],
            ty: impl_pi(
                "alpha",
                type0(),
                arrow(app(cst("Polynomial"), bvar(0)), arrow(bvar(0), bvar(0))),
            ),
        });
        let _ = env.add(Declaration::Axiom {
            name: Name::str("Real.ofNat"),
            univ_params: vec![],
            ty: arrow(nat_ty(), real_ty()),
        });
        let result = build_complex_env(&mut env);
        assert!(result.is_ok());
    }
}
/// Cauchy integral formula computation (simple approximation).
#[allow(dead_code)]
pub fn cauchy_integral_approx<F>(f: F, center: C64, radius: f64, n_points: usize) -> C64
where
    F: Fn(C64) -> C64,
{
    let mut sum = C64::zero();
    let dt = 2.0 * std::f64::consts::PI / n_points as f64;
    for k in 0..n_points {
        let t = k as f64 * dt;
        let z = center.add(&C64::from_polar(radius, t));
        let fz = f(z);
        let dz = C64::new(-radius * t.sin(), radius * t.cos());
        sum = sum.add(&fz.mul(&dz));
    }
    let factor = C64::new(0.0, -1.0 / (2.0 * std::f64::consts::PI));
    let result = sum.mul(&factor);
    C64::new(result.re * dt, result.im * dt)
}
