//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{
    BanachAlgebraElem, BoundedPerturbation, FiniteDimCStarAlgebra, FredholmIndexCalculator,
    FunctionAlgebraElement, NumericalRange, OperatorSemigroup, Polynomial, ResolventData,
    SpectralMeasure, SpectralRadiusComputer, SquareMatrix, StrongContSemigroup, TraceClassNorm,
};

pub fn app(f: Expr, a: Expr) -> Expr {
    Expr::App(Box::new(f), Box::new(a))
}
pub fn app2(f: Expr, a: Expr, b: Expr) -> Expr {
    app(app(f, a), b)
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
pub fn real_ty() -> Expr {
    cst("Real")
}
/// `ContinuousFunctionalCalculus : (X : Type) -> BoundedLinearOp X X -> Type`
///
/// The continuous functional calculus assigns to each continuous function on the
/// spectrum a bounded operator, extending polynomial evaluation via density of
/// polynomials in C(sigma(T)).
pub fn continuous_functional_calculus_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        type0(),
        arrow(app2(cst("BoundedLinearOp"), bvar(0), bvar(0)), type0()),
    )
}
/// `HolomorphicFunctionalCalculus : (X : Type) -> BoundedLinearOp X X -> Type`
///
/// The holomorphic functional calculus extends functional calculus to functions
/// holomorphic on a neighbourhood of the spectrum, defined via
/// f(T) = (1/2pi i) integral_Gamma f(z)(zI - T)^{-1} dz.
pub fn holomorphic_functional_calculus_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        type0(),
        arrow(app2(cst("BoundedLinearOp"), bvar(0), bvar(0)), type0()),
    )
}
/// `SpectralMappingTheorem : (X : Type) -> (T : BoundedLinearOp X X) ->
///                            (f : ContinuousFunctionalCalculus X T) -> Prop`
///
/// sigma(f(T)) = f(sigma(T)) for continuous f.
pub fn spectral_mapping_theorem_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        type0(),
        pi(
            BinderInfo::Default,
            "T",
            app2(cst("BoundedLinearOp"), bvar(0), bvar(0)),
            arrow(
                app2(cst("ContinuousFunctionalCalculus"), bvar(1), bvar(0)),
                prop(),
            ),
        ),
    )
}
/// `SpectralRadiusFormula : (X : Type) -> (T : BoundedLinearOp X X) -> Prop`
///
/// The Gelfand spectral radius formula:
/// r(T) = lim_{n -> infty} ||T^n||^{1/n} = inf_n ||T^n||^{1/n}.
pub fn spectral_radius_formula_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        type0(),
        arrow(app2(cst("BoundedLinearOp"), bvar(0), bvar(0)), prop()),
    )
}
/// `ResolventOperator : (X : Type) -> BoundedLinearOp X X -> Real -> Type`
///
/// The resolvent R(lambda, T) = (lambda I - T)^{-1}, defined for lambda
/// not in the spectrum of T.
pub fn resolvent_operator_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        type0(),
        arrow(
            app2(cst("BoundedLinearOp"), bvar(0), bvar(0)),
            arrow(real_ty(), type0()),
        ),
    )
}
/// `ResolventIdentity : (X : Type) -> (T : BoundedLinearOp X X) ->
///                       (lambda mu : Real) -> Prop`
///
/// The first resolvent identity:
/// R(lambda, T) - R(mu, T) = (mu - lambda) R(lambda, T) R(mu, T).
pub fn resolvent_identity_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        type0(),
        pi(
            BinderInfo::Default,
            "T",
            app2(cst("BoundedLinearOp"), bvar(0), bvar(0)),
            arrow(real_ty(), arrow(real_ty(), prop())),
        ),
    )
}
/// `FunctionAlgebra : (X : Type) -> Type`
///
/// The algebra of continuous functions on a compact Hausdorff space, forming
/// a commutative unital C*-algebra under pointwise operations.
pub fn function_algebra_ty() -> Expr {
    arrow(type0(), type0())
}
/// `PolynomialEvalOp : (X : Type) -> BoundedLinearOp X X -> Type`
///
/// Evaluation of a polynomial p(z) = a_0 + a_1 z + ... + a_n z^n at an
/// operator T, yielding p(T) = a_0 I + a_1 T + ... + a_n T^n.
pub fn polynomial_eval_op_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        type0(),
        arrow(app2(cst("BoundedLinearOp"), bvar(0), bvar(0)), type0()),
    )
}
/// `BanachAlgebra : Type -> Prop`
///
/// Predicate asserting that a type A carries the structure of a Banach algebra:
/// a complete normed associative unital algebra over the reals (or complexes).
pub fn banach_algebra_ty() -> Expr {
    arrow(type0(), prop())
}
/// `BanachAlgebraSpectrum : (A : Type) -> BanachAlgebra A -> A -> Type`
///
/// The spectrum sigma(a) of an element a in a Banach algebra A is the set of
/// scalars lambda for which (a - lambda * 1) is not invertible in A.
pub fn banach_algebra_spectrum_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "A",
        type0(),
        arrow(app(cst("BanachAlgebra"), bvar(0)), arrow(bvar(1), type0())),
    )
}
/// `SpectralRadiusBanach : (A : Type) -> BanachAlgebra A -> A -> Real`
///
/// The spectral radius r(a) = sup { |lambda| : lambda in sigma(a) }
/// equals lim_{n->inf} ||a^n||^{1/n} in any Banach algebra (Gelfand formula).
pub fn spectral_radius_banach_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "A",
        type0(),
        arrow(
            app(cst("BanachAlgebra"), bvar(0)),
            arrow(bvar(1), real_ty()),
        ),
    )
}
/// `GelfandTransform : (A : Type) -> BanachAlgebra A -> Type`
///
/// The Gelfand transform Gamma : A -> C(M_A) where M_A is the maximal ideal
/// space (character space) of A; it is a continuous algebra homomorphism that
/// is an isometric *-isomorphism when A is a commutative C*-algebra.
pub fn gelfand_transform_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "A",
        type0(),
        arrow(app(cst("BanachAlgebra"), bvar(0)), type0()),
    )
}
/// `GelfandMazurTheorem : (A : Type) -> BanachAlgebra A -> Prop`
///
/// Every Banach algebra that is also a division algebra (every non-zero element
/// is invertible) is isometrically isomorphic to the complex numbers.
pub fn gelfand_mazur_theorem_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "A",
        type0(),
        arrow(app(cst("BanachAlgebra"), bvar(0)), prop()),
    )
}
/// `CStarAlgebra : Type -> Prop`
///
/// A C*-algebra is a Banach algebra with an involution * satisfying the
/// C*-identity ||a* a|| = ||a||^2.  Every commutative C*-algebra is
/// isometrically *-isomorphic to C(X) for a compact Hausdorff space X.
pub fn cstar_algebra_ty() -> Expr {
    arrow(type0(), prop())
}
/// `PositiveElement : (A : Type) -> CStarAlgebra A -> A -> Prop`
///
/// An element a in a C*-algebra is positive (a >= 0) iff a = b* b for some
/// b in A, equivalently iff sigma(a) subset [0, infty).
pub fn positive_element_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "A",
        type0(),
        arrow(app(cst("CStarAlgebra"), bvar(0)), arrow(bvar(1), prop())),
    )
}
/// `State : (A : Type) -> CStarAlgebra A -> Type`
///
/// A state on a C*-algebra A is a positive linear functional phi : A -> C
/// with ||phi|| = 1, equivalently phi(1) = 1 (when A is unital).
pub fn state_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "A",
        type0(),
        arrow(app(cst("CStarAlgebra"), bvar(0)), type0()),
    )
}
/// `BorelFunctionalCalculus : (X : Type) -> (T : BoundedLinearOp X X) -> Type`
///
/// The Borel functional calculus extends the continuous functional calculus to
/// bounded Borel-measurable functions on the spectrum of a bounded self-adjoint
/// operator T, using the spectral measure associated to T.
pub fn borel_functional_calculus_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        type0(),
        arrow(app2(cst("BoundedLinearOp"), bvar(0), bvar(0)), type0()),
    )
}
/// `SpectralMeasure : (X : Type) -> (T : BoundedLinearOp X X) -> Type`
///
/// A spectral measure (projection-valued measure, PVM) E : Borel(R) -> B(X)
/// assigns to each Borel set a projection on X satisfying countable additivity
/// and E(R) = I.  The spectral theorem provides such a measure for every
/// self-adjoint operator T via T = integral lambda dE(lambda).
pub fn spectral_measure_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        type0(),
        arrow(app2(cst("BoundedLinearOp"), bvar(0), bvar(0)), type0()),
    )
}
/// `SpectralProjection : (X : Type) -> (T : BoundedLinearOp X X) -> Real -> Type`
///
/// The spectral projection P(lambda_0) = E({lambda_0}) associated with an
/// isolated eigenvalue lambda_0 in the spectrum of T.
/// For a simple isolated eigenvalue it equals the eigenprojection.
pub fn spectral_projection_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        type0(),
        arrow(
            app2(cst("BoundedLinearOp"), bvar(0), bvar(0)),
            arrow(real_ty(), type0()),
        ),
    )
}
/// `DunfordIntegral : (X : Type) -> (T : BoundedLinearOp X X) -> Type`
///
/// The Dunford (Riesz-Dunford) integral representation:
/// f(T) = (1 / 2pi i) * contour_integral_Gamma f(z) R(z, T) dz,
/// where Gamma is a contour surrounding the spectrum of T.
pub fn dunford_integral_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        type0(),
        arrow(app2(cst("BoundedLinearOp"), bvar(0), bvar(0)), type0()),
    )
}
/// `FredholmOperator : (X : Type) -> (Y : Type) -> BoundedLinearOp X Y -> Prop`
///
/// A bounded linear operator T : X -> Y is Fredholm if ker(T) and coker(T)
/// are both finite-dimensional.  The Fredholm index is ind(T) = dim(ker T) - dim(coker T).
pub fn fredholm_operator_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        type0(),
        pi(
            BinderInfo::Default,
            "Y",
            type0(),
            arrow(app2(cst("BoundedLinearOp"), bvar(1), bvar(0)), prop()),
        ),
    )
}
/// `FredholmIndex : (X : Type) -> (Y : Type) -> (T : BoundedLinearOp X Y) ->
///                   FredholmOperator X Y T -> Real`
///
/// The Fredholm index ind(T) = dim ker(T) - dim coker(T).
/// It is stable under small perturbations and compact perturbations.
pub fn fredholm_index_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        type0(),
        pi(
            BinderInfo::Default,
            "Y",
            type0(),
            pi(
                BinderInfo::Default,
                "T",
                app2(cst("BoundedLinearOp"), bvar(1), bvar(0)),
                arrow(
                    app2(
                        app2(cst("FredholmOperator"), bvar(2), bvar(1)),
                        bvar(0),
                        bvar(0),
                    ),
                    real_ty(),
                ),
            ),
        ),
    )
}
/// `EssentialSpectrum : (X : Type) -> BoundedLinearOp X X -> Type`
///
/// The essential spectrum sigma_ess(T) is the set of lambda for which
/// (T - lambda I) is not a Fredholm operator.  It is invariant under
/// compact perturbations (Weyl's theorem: sigma_ess(T) = sigma_ess(T+K)
/// for compact K).
pub fn essential_spectrum_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        type0(),
        arrow(app2(cst("BoundedLinearOp"), bvar(0), bvar(0)), type0()),
    )
}
/// `WeylsTheorem : (X : Type) -> (T : BoundedLinearOp X X) -> Prop`
///
/// Weyl's theorem: sigma(T) \ sigma_ess(T) consists of isolated eigenvalues
/// of finite multiplicity (the Weyl spectrum equals the essential spectrum
/// for self-adjoint operators).
pub fn weyls_theorem_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        type0(),
        arrow(app2(cst("BoundedLinearOp"), bvar(0), bvar(0)), prop()),
    )
}
/// `BrowdersTheorem : (X : Type) -> (T : BoundedLinearOp X X) -> Prop`
///
/// Browder's theorem: the Browder spectrum (essential spectrum union accumulation
/// points of sigma) equals the Weyl spectrum; equivalently, the set of Riesz
/// points (isolated eigenvalues of finite multiplicity not in sigma_ess) is
/// the complement of the Browder spectrum in sigma(T).
pub fn browders_theorem_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        type0(),
        arrow(app2(cst("BoundedLinearOp"), bvar(0), bvar(0)), prop()),
    )
}
/// `StronglyContinuousSemigroup : (X : Type) -> Type`
///
/// A strongly continuous (C_0) semigroup is a family {T(t)}_{t>=0} of bounded
/// linear operators on X such that T(0) = I, T(s+t) = T(s)T(t), and
/// t -> T(t)x is continuous for each x in X.
pub fn strongly_continuous_semigroup_ty() -> Expr {
    arrow(type0(), type0())
}
/// `InfinitesimalGenerator : (X : Type) -> StronglyContinuousSemigroup X -> Type`
///
/// The infinitesimal generator A of a C_0-semigroup {T(t)} is defined by
/// Ax = lim_{t->0+} (T(t)x - x)/t on the domain D(A) = { x : this limit exists }.
pub fn infinitesimal_generator_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        type0(),
        arrow(app(cst("StronglyContinuousSemigroup"), bvar(0)), type0()),
    )
}
/// `HilleYosidaTheorem : (X : Type) -> (A : BoundedLinearOp X X) -> Prop`
///
/// The Hille-Yosida theorem characterizes generators of C_0-semigroups:
/// A generates a C_0-semigroup of contractions iff A is densely defined,
/// closed, and the resolvent satisfies ||R(lambda, A)|| <= 1/lambda for
/// all lambda > 0.
pub fn hille_yosida_theorem_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        type0(),
        arrow(app2(cst("BoundedLinearOp"), bvar(0), bvar(0)), prop()),
    )
}
/// `StonesTheorem : (H : Type) -> (A : BoundedLinearOp H H) -> Prop`
///
/// Stone's theorem: every strongly continuous one-parameter unitary group
/// {U(t)}_{t in R} on a Hilbert space H has the form U(t) = exp(itA) for a
/// unique self-adjoint operator A (the generator), and conversely.
pub fn stones_theorem_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "H",
        type0(),
        arrow(app2(cst("BoundedLinearOp"), bvar(0), bvar(0)), prop()),
    )
}
/// `DissipatveOperator : (X : Type) -> (A : BoundedLinearOp X X) -> Prop`
///
/// A densely defined linear operator A on a Banach space X is dissipative if
/// Re <Ax, x*> <= 0 for all x in D(A) and all x* in the duality map of x.
/// Dissipative operators generate contraction semigroups (Lumer-Phillips theorem).
pub fn dissipative_operator_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        type0(),
        arrow(app2(cst("BoundedLinearOp"), bvar(0), bvar(0)), prop()),
    )
}
/// `AccretiveOperator : (X : Type) -> (A : BoundedLinearOp X X) -> Prop`
///
/// A densely defined operator A is accretive if -A is dissipative, i.e.,
/// Re <Ax, x*> >= 0.  A maximal accretive operator generates a C_0-semigroup
/// of contractions on the Hilbert space (Kato-Rellich theorem).
pub fn accretive_operator_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        type0(),
        arrow(app2(cst("BoundedLinearOp"), bvar(0), bvar(0)), prop()),
    )
}
/// `MSectorialOperator : (X : Type) -> (A : BoundedLinearOp X X) -> Prop`
///
/// An m-sectorial operator is a closed densely defined operator whose numerical
/// range is contained in a sector {z : |arg(z - gamma)| <= theta} for some
/// gamma in R and theta < pi/2, and the resolvent exists outside the sector
/// with standard estimates.
pub fn m_sectorial_operator_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        type0(),
        arrow(app2(cst("BoundedLinearOp"), bvar(0), bvar(0)), prop()),
    )
}
/// `HilbertSchmidtOperator : (H : Type) -> (T : BoundedLinearOp H H) -> Prop`
///
/// A bounded operator T on a Hilbert space H is Hilbert-Schmidt if for some
/// (and hence every) orthonormal basis {e_n}: sum ||T e_n||^2 < infty.
/// The Hilbert-Schmidt norm is ||T||_HS = (sum ||T e_n||^2)^{1/2}.
pub fn hilbert_schmidt_operator_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "H",
        type0(),
        arrow(app2(cst("BoundedLinearOp"), bvar(0), bvar(0)), prop()),
    )
}
/// `TraceClassOperator : (H : Type) -> (T : BoundedLinearOp H H) -> Prop`
///
/// A bounded operator T on a Hilbert space H is trace class (nuclear) if
/// sum <|T| e_n, e_n> < infty for some orthonormal basis {e_n}, where
/// |T| = (T* T)^{1/2}.  The trace norm is ||T||_1 = sum singular values.
pub fn trace_class_operator_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "H",
        type0(),
        arrow(app2(cst("BoundedLinearOp"), bvar(0), bvar(0)), prop()),
    )
}
/// `TraceOfOperator : (H : Type) -> (T : BoundedLinearOp H H) ->
///                     TraceClassOperator H T -> Real`
///
/// The trace tr(T) = sum <T e_n, e_n> for an orthonormal basis {e_n};
/// this is well-defined and basis-independent for trace class operators.
pub fn trace_of_operator_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "H",
        type0(),
        pi(
            BinderInfo::Default,
            "T",
            app2(cst("BoundedLinearOp"), bvar(0), bvar(0)),
            arrow(app2(cst("TraceClassOperator"), bvar(1), bvar(0)), real_ty()),
        ),
    )
}
/// `LidskiiTraceFormula : (H : Type) -> (T : BoundedLinearOp H H) -> Prop`
///
/// Lidskii's trace theorem: for a trace class operator T on a Hilbert space,
/// tr(T) = sum_{n} lambda_n where the lambda_n are the eigenvalues of T
/// counted with algebraic multiplicity.
pub fn lidskii_trace_formula_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "H",
        type0(),
        arrow(app2(cst("BoundedLinearOp"), bvar(0), bvar(0)), prop()),
    )
}
/// `AbstractCauchyProblem : (X : Type) -> Type`
///
/// The abstract Cauchy problem: u'(t) = A u(t), u(0) = x, where A is the
/// generator of a C_0-semigroup on X.  The unique mild solution is u(t) = T(t)x.
pub fn abstract_cauchy_problem_ty() -> Expr {
    arrow(type0(), type0())
}
/// `RieszFunctionalCalculus : (X : Type) -> (T : BoundedLinearOp X X) -> Type`
///
/// The Riesz functional calculus defines f(T) via the contour integral
/// f(T) = (1 / 2pi i) * integral_Gamma f(z)(zI - T)^{-1} dz
/// for f holomorphic on a neighbourhood of sigma(T), generalising the
/// Cauchy integral formula to operators.  It equals the holomorphic
/// functional calculus but emphasises the contour construction.
pub fn riesz_functional_calculus_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        type0(),
        arrow(app2(cst("BoundedLinearOp"), bvar(0), bvar(0)), type0()),
    )
}
/// `NormalElement : (A : Type) -> CStarAlgebra A -> A -> Prop`
///
/// An element a in a C*-algebra A is normal if a* a = a a*.
/// The continuous functional calculus applies to normal elements,
/// giving a *-homomorphism C(sigma(a)) -> A.
pub fn normal_element_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "A",
        type0(),
        arrow(app(cst("CStarAlgebra"), bvar(0)), arrow(bvar(1), prop())),
    )
}
/// `ContinuousCalcForNormalElement : (A : Type) -> (a : A) ->
///                                    NormalElement A a -> Type`
///
/// Given a normal element a in a C*-algebra with spectrum sigma(a),
/// the continuous functional calculus provides a unique unital
/// *-homomorphism phi_a : C(sigma(a)) -> A extending the polynomial
/// calculus, with ||phi_a(f)|| = ||f||_inf.
pub fn continuous_calc_normal_element_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "A",
        type0(),
        pi(
            BinderInfo::Default,
            "a",
            bvar(0),
            arrow(app2(cst("NormalElement"), bvar(1), bvar(0)), type0()),
        ),
    )
}
/// Register all functional calculus axioms in the given environment.
pub fn build_functional_calculus_env(env: &mut Environment) {
    let axioms: &[(&str, Expr)] = &[
        (
            "ContinuousFunctionalCalculus",
            continuous_functional_calculus_ty(),
        ),
        (
            "HolomorphicFunctionalCalculus",
            holomorphic_functional_calculus_ty(),
        ),
        ("ResolventOperator", resolvent_operator_ty()),
        ("FunctionAlgebra", function_algebra_ty()),
        ("PolynomialEvalOp", polynomial_eval_op_ty()),
        ("spectral_mapping_theorem", spectral_mapping_theorem_ty()),
        ("spectral_radius_formula", spectral_radius_formula_ty()),
        ("resolvent_identity", resolvent_identity_ty()),
        ("BanachAlgebra", banach_algebra_ty()),
        ("BanachAlgebraSpectrum", banach_algebra_spectrum_ty()),
        ("SpectralRadiusBanach", spectral_radius_banach_ty()),
        ("GelfandTransform", gelfand_transform_ty()),
        ("gelfand_mazur_theorem", gelfand_mazur_theorem_ty()),
        ("CStarAlgebra", cstar_algebra_ty()),
        ("PositiveElement", positive_element_ty()),
        ("State", state_ty()),
        ("NormalElement", normal_element_ty()),
        (
            "ContinuousCalcForNormalElement",
            continuous_calc_normal_element_ty(),
        ),
        ("BorelFunctionalCalculus", borel_functional_calculus_ty()),
        ("SpectralMeasure", spectral_measure_ty()),
        ("SpectralProjection", spectral_projection_ty()),
        ("DunfordIntegral", dunford_integral_ty()),
        ("RieszFunctionalCalculus", riesz_functional_calculus_ty()),
        ("FredholmOperator", fredholm_operator_ty()),
        ("FredholmIndex", fredholm_index_ty()),
        ("EssentialSpectrum", essential_spectrum_ty()),
        ("weyls_theorem", weyls_theorem_ty()),
        ("browders_theorem", browders_theorem_ty()),
        (
            "StronglyContinuousSemigroup",
            strongly_continuous_semigroup_ty(),
        ),
        ("InfinitesimalGenerator", infinitesimal_generator_ty()),
        ("hille_yosida_theorem", hille_yosida_theorem_ty()),
        ("stones_theorem", stones_theorem_ty()),
        ("AbstractCauchyProblem", abstract_cauchy_problem_ty()),
        ("DissipativeOperator", dissipative_operator_ty()),
        ("AccretiveOperator", accretive_operator_ty()),
        ("MSectorialOperator", m_sectorial_operator_ty()),
        ("HilbertSchmidtOperator", hilbert_schmidt_operator_ty()),
        ("TraceClassOperator", trace_class_operator_ty()),
        ("TraceOfOperator", trace_of_operator_ty()),
        ("lidskii_trace_formula", lidskii_trace_formula_ty()),
    ];
    for &(name, ref ty) in axioms {
        env.add(Declaration::Axiom {
            name: Name::str(name),
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
    fn test_polynomial_eval() {
        let p = Polynomial::new(vec![2.0, 3.0, 1.0]);
        assert!((p.eval(0.0) - 2.0).abs() < 1e-10);
        assert!((p.eval(1.0) - 6.0).abs() < 1e-10);
        assert!((p.eval(2.0) - 12.0).abs() < 1e-10);
    }
    #[test]
    fn test_polynomial_add_multiply() {
        let p = Polynomial::new(vec![1.0, 1.0]);
        let q = Polynomial::new(vec![1.0, -1.0]);
        let sum = p.add(&q);
        assert!((sum.eval(5.0) - 2.0).abs() < 1e-10);
        let prod = p.multiply(&q);
        assert!((prod.eval(0.0) - 1.0).abs() < 1e-10);
        assert!((prod.eval(1.0) - 0.0).abs() < 1e-10);
        assert!((prod.eval(3.0) - (-8.0)).abs() < 1e-10);
    }
    #[test]
    fn test_polynomial_compose() {
        let p = Polynomial::new(vec![0.0, 0.0, 1.0]);
        let q = Polynomial::new(vec![1.0, 2.0]);
        let c = p.compose(&q);
        assert!((c.eval(0.0) - 1.0).abs() < 1e-10);
        assert!((c.eval(1.0) - 9.0).abs() < 1e-10);
        assert!((c.eval(2.0) - 25.0).abs() < 1e-10);
    }
    #[test]
    fn test_polynomial_degree() {
        assert_eq!(Polynomial::zero().degree(), 0);
        assert_eq!(Polynomial::constant(5.0).degree(), 0);
        assert_eq!(Polynomial::identity().degree(), 1);
        assert_eq!(Polynomial::new(vec![1.0, 0.0, 3.0]).degree(), 2);
    }
    #[test]
    fn test_matrix_poly_eval() {
        let a = SquareMatrix::new(vec![1.0, 1.0, 0.0, 1.0], 2);
        let p = Polynomial::new(vec![1.0, -2.0, 1.0]);
        let pa = a.poly_eval(&p);
        for entry in &pa.entries {
            assert!(
                entry.abs() < 1e-10,
                "p(A) should be zero matrix, got {}",
                entry
            );
        }
    }
    #[test]
    fn test_matrix_pow() {
        let a = SquareMatrix::new(vec![1.0, 1.0, 0.0, 1.0], 2);
        let a3 = a.pow(3);
        assert!((a3.get(0, 0) - 1.0).abs() < 1e-10);
        assert!((a3.get(0, 1) - 3.0).abs() < 1e-10);
        assert!((a3.get(1, 0) - 0.0).abs() < 1e-10);
        assert!((a3.get(1, 1) - 1.0).abs() < 1e-10);
    }
    #[test]
    fn test_spectral_radius() {
        let a = SquareMatrix::new(vec![2.0, 0.0, 0.0, 3.0], 2);
        let r = a.spectral_radius(20);
        assert!(
            (r - 3.0).abs() < 0.1,
            "spectral radius should be ~3.0, got {}",
            r
        );
    }
    #[test]
    fn test_resolvent_2x2() {
        let a = SquareMatrix::new(vec![1.0, 0.0, 0.0, 2.0], 2);
        let r = a.resolvent_2x2(3.0).expect("lambda=3 is not in spectrum");
        assert!((r.get(0, 0) - 0.5).abs() < 1e-10);
        assert!((r.get(0, 1) - 0.0).abs() < 1e-10);
        assert!((r.get(1, 0) - 0.0).abs() < 1e-10);
        assert!((r.get(1, 1) - 1.0).abs() < 1e-10);
        assert!(a.resolvent_2x2(1.0).is_none());
    }
    #[test]
    fn test_function_algebra_add_mul() {
        let n = 100;
        let f = FunctionAlgebraElement::from_fn(|x| x, n);
        let g = FunctionAlgebraElement::from_fn(|x| 1.0 - x, n);
        let sum = f.add(&g);
        for v in &sum.values {
            assert!((v - 1.0).abs() < 1e-10, "f + g should be 1.0, got {}", v);
        }
        let prod = f.multiply(&g);
        let max_val = prod.sup_norm();
        assert!(
            (max_val - 0.25).abs() < 0.01,
            "max of x(1-x) should be ~0.25, got {}",
            max_val
        );
    }
    #[test]
    fn test_function_algebra_norms() {
        let n = 1000;
        let f = FunctionAlgebraElement::from_fn(|x| x * x, n);
        assert!(
            (f.sup_norm() - 1.0).abs() < 0.01,
            "sup norm of x^2 on [0,1] should be ~1.0"
        );
        let expected_l2 = (1.0 / 5.0_f64).sqrt();
        let actual_l2 = f.l2_norm();
        assert!(
            (actual_l2 - expected_l2).abs() < 0.01,
            "L2 norm of x^2 should be ~{}, got {}",
            expected_l2,
            actual_l2
        );
    }
    #[test]
    fn test_build_functional_calculus_env() {
        let mut env = Environment::new();
        build_functional_calculus_env(&mut env);
        let expected_names = [
            "ContinuousFunctionalCalculus",
            "HolomorphicFunctionalCalculus",
            "ResolventOperator",
            "FunctionAlgebra",
            "PolynomialEvalOp",
            "spectral_mapping_theorem",
            "spectral_radius_formula",
            "resolvent_identity",
            "BanachAlgebra",
            "BanachAlgebraSpectrum",
            "SpectralRadiusBanach",
            "GelfandTransform",
            "gelfand_mazur_theorem",
            "CStarAlgebra",
            "PositiveElement",
            "State",
            "NormalElement",
            "ContinuousCalcForNormalElement",
            "BorelFunctionalCalculus",
            "SpectralMeasure",
            "SpectralProjection",
            "DunfordIntegral",
            "RieszFunctionalCalculus",
            "FredholmOperator",
            "FredholmIndex",
            "EssentialSpectrum",
            "weyls_theorem",
            "browders_theorem",
            "StronglyContinuousSemigroup",
            "InfinitesimalGenerator",
            "hille_yosida_theorem",
            "stones_theorem",
            "AbstractCauchyProblem",
            "DissipativeOperator",
            "AccretiveOperator",
            "MSectorialOperator",
            "HilbertSchmidtOperator",
            "TraceClassOperator",
            "TraceOfOperator",
            "lidskii_trace_formula",
        ];
        for name in expected_names {
            assert!(
                env.get(&Name::str(name)).is_some(),
                "axiom '{}' should be registered in the environment",
                name
            );
        }
    }
    #[test]
    fn test_banach_algebra_elem_spectrum_2x2() {
        let mat = SquareMatrix::new(vec![2.0, 0.0, 0.0, 5.0], 2);
        let elem = BanachAlgebraElem::new(mat, "M_2(R)");
        let evs = elem.spectrum_2x2().expect("2x2 matrix");
        let mut reals: Vec<f64> = evs.iter().map(|e| e.0).collect();
        reals.sort_by(|a, b| a.partial_cmp(b).expect("sort_by should succeed"));
        assert!((reals[0] - 2.0).abs() < 1e-10, "eigenvalue should be 2");
        assert!((reals[1] - 5.0).abs() < 1e-10, "eigenvalue should be 5");
        assert_eq!(elem.is_in_spectrum_2x2(2.0), Some(true));
        assert_eq!(elem.is_in_spectrum_2x2(5.0), Some(true));
        assert_eq!(elem.is_in_spectrum_2x2(3.0), Some(false));
    }
    #[test]
    fn test_banach_algebra_elem_spectral_radius() {
        let mat = SquareMatrix::new(vec![3.0, 0.0, 0.0, 1.0], 2);
        let elem = BanachAlgebraElem::new(mat, "M_2(R)");
        let r = elem.spectral_radius_estimate(25);
        assert!(
            (r - 3.0).abs() < 0.1,
            "spectral radius should be ~3, got {r}"
        );
    }
    #[test]
    fn test_banach_algebra_elem_invertibility() {
        let invertible = SquareMatrix::new(vec![2.0, 0.0, 0.0, 3.0], 2);
        let singular = SquareMatrix::new(vec![1.0, 2.0, 2.0, 4.0], 2);
        let e1 = BanachAlgebraElem::new(invertible, "M_2");
        let e2 = BanachAlgebraElem::new(singular, "M_2");
        assert_eq!(e1.is_invertible_2x2(), Some(true));
        assert_eq!(e2.is_invertible_2x2(), Some(false));
    }
    #[test]
    fn test_spectral_radius_computer_diagonal() {
        let mat = SquareMatrix::new(vec![4.0, 0.0, 0.0, 1.0], 2);
        let computer = SpectralRadiusComputer::default();
        let r = computer.compute(&mat);
        assert!((r - 4.0).abs() < 0.1, "expected ~4.0, got {r}");
    }
    #[test]
    fn test_spectral_radius_power_vector() {
        let mat = SquareMatrix::new(vec![5.0, 0.0, 0.0, 2.0], 2);
        let computer = SpectralRadiusComputer::new(50, 1e-10);
        let r = computer.power_vector_method(&mat, &[1.0, 0.1]);
        assert!(
            (r - 5.0).abs() < 0.5,
            "power method: expected ~5.0, got {r}"
        );
    }
    #[test]
    fn test_spectral_radius_zero_matrix() {
        let mat = SquareMatrix::zero(3);
        let computer = SpectralRadiusComputer::default();
        let r = computer.compute(&mat);
        assert!(
            r < 1e-10,
            "zero matrix spectral radius should be 0, got {r}"
        );
    }
    #[test]
    fn test_operator_semigroup_identity_generator() {
        let gen = SquareMatrix::zero(2);
        let sg = OperatorSemigroup::new(gen, 100);
        let t1 = sg.eval(1.0);
        let id = SquareMatrix::identity(2);
        let diff = t1.sub(&id);
        assert!(
            diff.frobenius_norm() < 1e-10,
            "T(t) should be I for zero generator"
        );
    }
    #[test]
    fn test_operator_semigroup_property() {
        let gen = SquareMatrix::new(vec![0.0, -1.0, 1.0, 0.0], 2);
        let sg = OperatorSemigroup::new(gen, 1000);
        let err = sg.check_semigroup_property(0.1, 0.2);
        assert!(
            err < 0.01,
            "semigroup property relative error should be small, got {err}"
        );
    }
    #[test]
    fn test_operator_semigroup_apply() {
        let gen = SquareMatrix::new(vec![0.0], 1);
        let sg = OperatorSemigroup::new(gen, 100);
        let result = sg.apply(1.0, &[2.5]);
        assert!(
            (result[0] - 2.5).abs() < 1e-10,
            "expected 2.5, got {}",
            result[0]
        );
    }
    #[test]
    fn test_fredholm_index_square_full_rank() {
        let calc = FredholmIndexCalculator::new(2, 2, vec![1.0, 0.0, 0.0, 1.0]);
        assert_eq!(calc.numerical_rank(1e-10), 2);
        assert_eq!(calc.kernel_dim(1e-10), 0);
        assert_eq!(calc.cokernel_dim(1e-10), 0);
        assert_eq!(calc.fredholm_index(1e-10), 0);
    }
    #[test]
    fn test_fredholm_index_rectangular() {
        let calc = FredholmIndexCalculator::new(2, 3, vec![1.0, 0.0, 0.0, 0.0, 1.0, 0.0]);
        assert_eq!(calc.fredholm_index(1e-10), 1);
    }
    #[test]
    fn test_fredholm_index_rank_deficient() {
        let calc = FredholmIndexCalculator::new(2, 2, vec![1.0, 2.0, 2.0, 4.0]);
        assert_eq!(calc.numerical_rank(1e-8), 1);
        assert_eq!(calc.kernel_dim(1e-8), 1);
        assert_eq!(calc.cokernel_dim(1e-8), 1);
        assert_eq!(calc.fredholm_index(1e-8), 0);
    }
    #[test]
    fn test_trace_class_norm_identity() {
        let mat = SquareMatrix::identity(2);
        let tcn = TraceClassNorm::new(mat);
        let tn = tcn.trace_norm_2x2().expect("2x2");
        assert!(
            (tn - 2.0).abs() < 1e-10,
            "trace norm of I_2 should be 2, got {tn}"
        );
    }
    #[test]
    fn test_trace_class_norm_diagonal() {
        let mat = SquareMatrix::new(vec![3.0, 0.0, 0.0, 4.0], 2);
        let tcn = TraceClassNorm::new(mat);
        let tn = tcn.trace_norm_2x2().expect("2x2");
        assert!((tn - 7.0).abs() < 1e-8, "trace norm should be 7, got {tn}");
    }
    #[test]
    fn test_hilbert_schmidt_norm() {
        let mat = SquareMatrix::new(vec![1.0, 2.0, 3.0, 4.0], 2);
        let tcn = TraceClassNorm::new(mat);
        let expected = 30.0_f64.sqrt();
        assert!(
            (tcn.hilbert_schmidt_norm() - expected).abs() < 1e-10,
            "HS norm should be sqrt(30)"
        );
    }
    #[test]
    fn test_lidskii_error_diagonal() {
        let mat = SquareMatrix::new(vec![2.0, 0.0, 0.0, 5.0], 2);
        let tcn = TraceClassNorm::new(mat);
        let err = tcn.lidskii_error_2x2().expect("2x2");
        assert!(
            err < 1e-10,
            "Lidskii error for diagonal matrix should be 0, got {err}"
        );
    }
    #[test]
    fn test_largest_singular_value() {
        let mat = SquareMatrix::new(vec![5.0, 0.0, 0.0, 2.0], 2);
        let tcn = TraceClassNorm::new(mat);
        let sv = tcn.largest_singular_value(30);
        assert!((sv - 5.0).abs() < 0.1, "largest SV should be ~5, got {sv}");
    }
}
#[cfg(test)]
mod tests_functional_calculus_ext {
    use super::*;
    #[test]
    fn test_spectral_measure_diagonal() {
        let sm = SpectralMeasure::diagonal(vec![1.0, 4.0, 9.0]);
        assert!((sm.spectral_radius() - 9.0).abs() < 1e-10);
        assert!(sm.is_positive_definite());
        let sq = sm
            .sqrt_eigenvalues()
            .expect("sqrt_eigenvalues should succeed");
        assert!((sq[0] - 1.0).abs() < 1e-10);
        assert!((sq[1] - 2.0).abs() < 1e-10);
        assert!((sq[2] - 3.0).abs() < 1e-10);
    }
    #[test]
    fn test_spectral_measure_trace() {
        let sm = SpectralMeasure::diagonal(vec![2.0, 3.0]);
        let tr = sm.trace_of_function(|x| x * x);
        assert!((tr - 13.0).abs() < 1e-10);
    }
    #[test]
    fn test_cstar_algebra() {
        let m2 = FiniteDimCStarAlgebra::matrix_algebra(2);
        assert_eq!(m2.dimension(), 4);
        assert!(m2.is_simple());
        assert!(!m2.is_commutative());
        assert_eq!(m2.k0_rank(), 1);
        let comm = FiniteDimCStarAlgebra::commutative(3);
        assert!(comm.is_commutative());
        assert_eq!(comm.num_irreps(), 3);
    }
    #[test]
    fn test_resolvent_data() {
        let rd = ResolventData::new(vec![1.0, 2.0, 3.0], 3.0);
        assert!(rd.in_resolvent_set(0.0, 0.5));
        assert!(!rd.in_resolvent_set(2.0, 0.5));
        assert!(rd.is_invertible());
    }
    #[test]
    fn test_semigroup() {
        let sg = StrongContSemigroup::new(vec![-1.0, -2.0], -1.0);
        assert!(sg.is_contractive);
        let v = sg.apply_at_time(1.0, &[1.0, 1.0]);
        assert!((v[0] - (-1.0f64).exp()).abs() < 1e-10);
        assert!(sg.check_hille_yosida());
        assert!((sg.spectral_bound() - (-1.0)).abs() < 1e-10);
    }
    #[test]
    fn test_resolvent_norm_estimate() {
        let mut rd = ResolventData::new(vec![0.0, 1.0], 1.0);
        rd.is_self_adjoint = true;
        let norm = rd
            .resolvent_norm_estimate(3.0)
            .expect("resolvent_norm_estimate should succeed");
        assert!((norm - 0.5).abs() < 1e-10);
    }
}
#[cfg(test)]
mod tests_functional_calculus_ext2 {
    use super::*;
    #[test]
    fn test_numerical_range() {
        let nr = NumericalRange::from_eigenvalues(&[1.0, 4.0]);
        assert!((nr.numerical_radius() - 4.0).abs() < 1e-10);
        assert!(!nr.contains_zero(1e-10));
        let nr2 = NumericalRange::from_eigenvalues(&[-1.0, 2.0]);
        assert!(nr2.contains_zero(1e-10));
    }
    #[test]
    fn test_bounded_perturbation() {
        let bp = BoundedPerturbation::new(0.5, -2.0, 1.0);
        assert!((bp.perturbed_growth_bound() - (-1.5)).abs() < 1e-10);
        assert!(bp.preserves_contractivity());
        let bp2 = BoundedPerturbation::new(3.0, -1.0, 1.0);
        assert!(!bp2.preserves_contractivity());
    }
}
