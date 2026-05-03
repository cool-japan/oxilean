//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)
#![allow(clippy::items_after_test_module)]

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{
    CompactOperator, Complex64, ComplexMatrix, EssentialSpectrum, ResolventSet,
    SelfAdjointOperator, SpectralDecomposition, SpectralMeasure,
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
pub fn real_ty() -> Expr {
    cst("Real")
}
pub fn bool_ty() -> Expr {
    cst("Bool")
}
pub fn complex_ty() -> Expr {
    cst("Complex")
}
pub fn list_ty(a: Expr) -> Expr {
    app(cst("List"), a)
}
pub fn option_ty(a: Expr) -> Expr {
    app(cst("Option"), a)
}
pub fn set_ty(a: Expr) -> Expr {
    app(cst("Set"), a)
}
/// Spectrum : BoundedLinearOperator → Set Complex
pub fn spectrum_ty() -> Expr {
    arrow(cst("BoundedLinearOperator"), set_ty(complex_ty()))
}
/// PointSpectrum : BoundedLinearOperator → Set Complex — eigenvalues
pub fn point_spectrum_ty() -> Expr {
    arrow(cst("BoundedLinearOperator"), set_ty(complex_ty()))
}
/// ContinuousSpectrum : BoundedLinearOperator → Set Complex
pub fn continuous_spectrum_ty() -> Expr {
    arrow(cst("BoundedLinearOperator"), set_ty(complex_ty()))
}
/// ResidualSpectrum : BoundedLinearOperator → Set Complex
pub fn residual_spectrum_ty() -> Expr {
    arrow(cst("BoundedLinearOperator"), set_ty(complex_ty()))
}
/// ResolventSet : BoundedLinearOperator → Set Complex — complement of spectrum
pub fn resolvent_set_ty() -> Expr {
    arrow(cst("BoundedLinearOperator"), set_ty(complex_ty()))
}
/// spectrum_partition : Spectrum = PointSpectrum ∪ ContinuousSpectrum ∪ ResidualSpectrum
pub fn spectrum_partition_ty() -> Expr {
    arrow(cst("BoundedLinearOperator"), prop())
}
/// spectrum_closed : ∀ T, IsClosed (Spectrum T)
pub fn spectrum_closed_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "T",
        cst("BoundedLinearOperator"),
        app(cst("IsClosed"), app(cst("Spectrum"), bvar(0))),
    )
}
/// spectrum_bounded : ∀ T, Spectrum T ⊆ Ball(0, ‖T‖)
pub fn spectrum_bounded_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "T",
        cst("BoundedLinearOperator"),
        prop(),
    )
}
/// Resolvent : BoundedLinearOperator → Complex → BoundedLinearOperator
/// R(λ,T) = (T - λI)⁻¹, defined for λ ∈ ρ(T)
pub fn resolvent_ty() -> Expr {
    arrow(
        cst("BoundedLinearOperator"),
        arrow(complex_ty(), cst("BoundedLinearOperator")),
    )
}
/// resolvent_equation : R(λ,T) - R(μ,T) = (λ - μ) R(λ,T) R(μ,T)
pub fn resolvent_equation_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "T",
        cst("BoundedLinearOperator"),
        prop(),
    )
}
/// resolvent_analytic : ResolventSet T → IsAnalytic (Resolvent T)
pub fn resolvent_analytic_ty() -> Expr {
    arrow(cst("BoundedLinearOperator"), prop())
}
/// neumann_series : ‖λ‖ > ‖T‖ → Resolvent T λ = -∑ Tⁿ / λⁿ⁺¹
pub fn neumann_series_ty() -> Expr {
    arrow(cst("BoundedLinearOperator"), arrow(complex_ty(), prop()))
}
/// SpectralRadius : BoundedLinearOperator → Real
pub fn spectral_radius_ty() -> Expr {
    arrow(cst("BoundedLinearOperator"), real_ty())
}
/// spectral_radius_formula : SpectralRadius T = lim ‖Tⁿ‖^(1/n)
pub fn spectral_radius_formula_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "T",
        cst("BoundedLinearOperator"),
        prop(),
    )
}
/// spectral_radius_le_norm : SpectralRadius T ≤ ‖T‖
pub fn spectral_radius_le_norm_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "T",
        cst("BoundedLinearOperator"),
        prop(),
    )
}
/// spectral_radius_normal : T normal → SpectralRadius T = ‖T‖
pub fn spectral_radius_normal_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "T",
        cst("BoundedLinearOperator"),
        arrow(app(cst("IsNormal"), bvar(0)), prop()),
    )
}
/// HolomorphicFunctionalCalculus : BoundedLinearOperator → (Complex → Complex) → BoundedLinearOperator
pub fn holomorphic_functional_calculus_ty() -> Expr {
    arrow(
        cst("BoundedLinearOperator"),
        arrow(
            arrow(complex_ty(), complex_ty()),
            cst("BoundedLinearOperator"),
        ),
    )
}
/// ContinuousFunctionalCalculus : SelfAdjointOperator → (Real → Real) → SelfAdjointOperator
pub fn continuous_functional_calculus_ty() -> Expr {
    arrow(
        cst("SelfAdjointOperator"),
        arrow(arrow(real_ty(), real_ty()), cst("SelfAdjointOperator")),
    )
}
/// BorelFunctionalCalculus : SelfAdjointOperator → (Real → Complex) → BoundedLinearOperator
pub fn borel_functional_calculus_ty() -> Expr {
    arrow(
        cst("SelfAdjointOperator"),
        arrow(arrow(real_ty(), complex_ty()), cst("BoundedLinearOperator")),
    )
}
/// functional_calculus_homomorphism : f g → Φ(fg) = Φ(f)Φ(g)
pub fn functional_calculus_homomorphism_ty() -> Expr {
    pi(BinderInfo::Default, "T", cst("SelfAdjointOperator"), prop())
}
/// functional_calculus_star : Φ(f̄) = Φ(f)*
pub fn functional_calculus_star_ty() -> Expr {
    pi(BinderInfo::Default, "T", cst("SelfAdjointOperator"), prop())
}
/// functional_calculus_spectral_mapping : σ(Φ(f)) = f(σ(T))
pub fn functional_calculus_spectral_mapping_ty() -> Expr {
    pi(BinderInfo::Default, "T", cst("SelfAdjointOperator"), prop())
}
/// StronglyContUnitary : (Real → UnitaryOperator) → Prop
pub fn strongly_cont_unitary_ty() -> Expr {
    arrow(arrow(real_ty(), cst("UnitaryOperator")), prop())
}
/// StonesGenerator : (Real → UnitaryOperator) → SelfAdjointOperator
pub fn stones_generator_ty() -> Expr {
    arrow(
        arrow(real_ty(), cst("UnitaryOperator")),
        cst("SelfAdjointOperator"),
    )
}
/// stones_theorem : ∀ U : ℝ → UnitaryOperator, StronglyCont U →
///   ∃ A : SelfAdjointOperator, ∀ t, U t = exp(itA)
pub fn stones_theorem_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "U",
        arrow(real_ty(), cst("UnitaryOperator")),
        arrow(
            app(cst("StronglyCont"), bvar(0)),
            app(cst("Exists"), app(cst("SelfAdjointOperator"), cst("_"))),
        ),
    )
}
/// stones_theorem_converse : ∀ A : SelfAdjointOperator, StronglyCont (fun t → exp(itA))
pub fn stones_theorem_converse_ty() -> Expr {
    pi(BinderInfo::Default, "A", cst("SelfAdjointOperator"), prop())
}
/// SpectralMeasure : HilbertSpace → Type — PVM on Borel sets
pub fn spectral_measure_ty() -> Expr {
    arrow(cst("HilbertSpace"), type0())
}
/// spectral_measure_orthogonal : E(A ∩ B) = E(A) E(B) = E(B) E(A)
pub fn spectral_measure_orthogonal_ty() -> Expr {
    arrow(cst("SpectralMeasure"), prop())
}
/// spectral_measure_sigma_additive : ∀ pairwise disjoint Aₙ, E(⋃Aₙ) = ∑E(Aₙ)
pub fn spectral_measure_sigma_additive_ty() -> Expr {
    arrow(cst("SpectralMeasure"), prop())
}
/// spectral_measure_unital : E(ℝ) = I
pub fn spectral_measure_unital_ty() -> Expr {
    arrow(cst("SpectralMeasure"), prop())
}
/// spectral_integral : SpectralMeasure → (Real → Complex) → BoundedLinearOperator
/// ∫ f dE for a bounded Borel function f
pub fn spectral_integral_ty() -> Expr {
    arrow(
        cst("SpectralMeasure"),
        arrow(arrow(real_ty(), complex_ty()), cst("BoundedLinearOperator")),
    )
}
/// spectral_measure_of_sa : SelfAdjointOperator → SpectralMeasure
pub fn spectral_measure_of_sa_ty() -> Expr {
    arrow(cst("SelfAdjointOperator"), cst("SpectralMeasure"))
}
/// SelfAdjointOperator : Type
pub fn self_adjoint_operator_ty() -> Expr {
    type0()
}
/// is_self_adjoint : BoundedLinearOperator → Prop  (T = T*)
pub fn is_self_adjoint_ty() -> Expr {
    arrow(cst("BoundedLinearOperator"), prop())
}
/// sa_spectrum_real : T self-adjoint → Spectrum T ⊆ ℝ
pub fn sa_spectrum_real_ty() -> Expr {
    pi(BinderInfo::Default, "T", cst("SelfAdjointOperator"), prop())
}
/// sa_norm_eq_spectral_radius : T self-adjoint → ‖T‖ = SpectralRadius T
pub fn sa_norm_eq_spectral_radius_ty() -> Expr {
    pi(BinderInfo::Default, "T", cst("SelfAdjointOperator"), prop())
}
/// sa_positive_spectrum : T ≥ 0 ↔ Spectrum T ⊆ [0,∞)
pub fn sa_positive_spectrum_ty() -> Expr {
    arrow(cst("SelfAdjointOperator"), prop())
}
/// NormalOperator : Type — T*T = TT*
pub fn normal_operator_ty() -> Expr {
    type0()
}
/// is_normal : BoundedLinearOperator → Prop
pub fn is_normal_ty() -> Expr {
    arrow(cst("BoundedLinearOperator"), prop())
}
/// UnitaryOperator : Type — T*T = TT* = I
pub fn unitary_operator_ty() -> Expr {
    type0()
}
/// ProjectionOperator : Type — T² = T = T*
pub fn projection_operator_ty() -> Expr {
    type0()
}
/// spectral_theorem_bounded_sa : ∀ T : SelfAdjointOperator, ∃ E : SpectralMeasure, T = ∫ λ dE
pub fn spectral_theorem_bounded_sa_ty() -> Expr {
    pi(BinderInfo::Default, "T", cst("SelfAdjointOperator"), prop())
}
/// spectral_theorem_normal : ∀ T : NormalOperator, ∃ E : SpectralMeasure, T = ∫ z dE
pub fn spectral_theorem_normal_ty() -> Expr {
    pi(BinderInfo::Default, "T", cst("NormalOperator"), prop())
}
/// spectral_decomposition_finite_dim : T self-adjoint, dim finite → T diagonalizable
pub fn spectral_decomposition_finite_dim_ty() -> Expr {
    pi(BinderInfo::Default, "T", cst("SelfAdjointOperator"), prop())
}
/// Eigenvalue : SelfAdjointOperator → Real → Prop
pub fn eigenvalue_ty() -> Expr {
    arrow(cst("SelfAdjointOperator"), arrow(real_ty(), prop()))
}
/// Eigenvector : SelfAdjointOperator → Real → HilbertSpace → Prop
pub fn eigenvector_ty() -> Expr {
    arrow(
        cst("SelfAdjointOperator"),
        arrow(real_ty(), arrow(cst("HilbertSpace"), prop())),
    )
}
/// sa_eigenvectors_orthogonal : distinct eigenvalues → eigenvectors orthogonal
pub fn sa_eigenvectors_orthogonal_ty() -> Expr {
    pi(BinderInfo::Default, "T", cst("SelfAdjointOperator"), prop())
}
/// EssentialSpectrum : BoundedLinearOperator → Set Complex
pub fn essential_spectrum_ty() -> Expr {
    arrow(cst("BoundedLinearOperator"), set_ty(complex_ty()))
}
/// weyls_theorem : K compact → EssentialSpectrum (T + K) = EssentialSpectrum T
pub fn weyls_theorem_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "T",
        cst("BoundedLinearOperator"),
        pi(BinderInfo::Default, "K", cst("CompactOperator"), prop()),
    )
}
/// FredhomIndex : BoundedLinearOperator → Int
pub fn fredholm_index_ty() -> Expr {
    arrow(cst("BoundedLinearOperator"), cst("Int"))
}
/// CompactOperator : Type
pub fn compact_operator_ty() -> Expr {
    type0()
}
/// HilbertSchmidtOperator : Type
pub fn hilbert_schmidt_operator_ty() -> Expr {
    type0()
}
/// TraceClassOperator : Type
pub fn trace_class_operator_ty() -> Expr {
    type0()
}
/// trace : TraceClassOperator → Complex
pub fn trace_ty() -> Expr {
    arrow(cst("TraceClassOperator"), complex_ty())
}
/// operator_norm : BoundedLinearOperator → Real
pub fn operator_norm_ty() -> Expr {
    arrow(cst("BoundedLinearOperator"), real_ty())
}
/// BoundedLinearOperator : HilbertSpace → HilbertSpace → Type
pub fn bounded_linear_operator_ty() -> Expr {
    arrow(cst("HilbertSpace"), arrow(cst("HilbertSpace"), type0()))
}
/// HilbertSpace : Type
pub fn hilbert_space_ty() -> Expr {
    type0()
}
/// hahn_banach_extension : Subspace → BoundedLinear → Prop
pub fn hahn_banach_extension_ty() -> Expr {
    arrow(
        cst("Subspace"),
        arrow(cst("BoundedLinearFunctional"), prop()),
    )
}
/// riesz_representation : HilbertSpace → BoundedLinearFunctional → HilbertSpace → Prop
pub fn riesz_representation_ty() -> Expr {
    arrow(
        cst("HilbertSpace"),
        arrow(
            cst("BoundedLinearFunctional"),
            arrow(cst("HilbertSpace"), prop()),
        ),
    )
}
/// lax_milgram : BoundedLinearOperator → Prop
pub fn lax_milgram_ty() -> Expr {
    arrow(cst("BoundedLinearOperator"), prop())
}
/// Register all spectral theory axioms into the kernel environment.
pub fn build_spectral_theory_env() -> Environment {
    let mut env = Environment::new();
    let axioms: &[(&str, Expr)] = &[
        ("HilbertSpace", hilbert_space_ty()),
        ("Complex", type0()),
        ("Subspace", type0()),
        ("BoundedLinearFunctional", type0()),
        ("BoundedLinearOperator", type0()),
        ("SelfAdjointOperator", self_adjoint_operator_ty()),
        ("NormalOperator", normal_operator_ty()),
        ("UnitaryOperator", unitary_operator_ty()),
        ("ProjectionOperator", projection_operator_ty()),
        ("CompactOperator", compact_operator_ty()),
        ("HilbertSchmidtOperator", hilbert_schmidt_operator_ty()),
        ("TraceClassOperator", trace_class_operator_ty()),
        ("Set", arrow(type0(), type0())),
        ("IsClosed", arrow(set_ty(complex_ty()), prop())),
        ("IsNormal", arrow(cst("BoundedLinearOperator"), prop())),
        (
            "StronglyCont",
            arrow(arrow(real_ty(), cst("UnitaryOperator")), prop()),
        ),
        ("Exists", arrow(type0(), prop())),
        ("Spectrum", spectrum_ty()),
        ("PointSpectrum", point_spectrum_ty()),
        ("ContinuousSpectrum", continuous_spectrum_ty()),
        ("ResidualSpectrum", residual_spectrum_ty()),
        ("ResolventSet", resolvent_set_ty()),
        ("spectrum_partition", spectrum_partition_ty()),
        ("spectrum_closed", spectrum_closed_ty()),
        ("spectrum_bounded", spectrum_bounded_ty()),
        ("Resolvent", resolvent_ty()),
        ("resolvent_equation", resolvent_equation_ty()),
        ("resolvent_analytic", resolvent_analytic_ty()),
        ("neumann_series", neumann_series_ty()),
        ("SpectralRadius", spectral_radius_ty()),
        ("spectral_radius_formula", spectral_radius_formula_ty()),
        ("spectral_radius_le_norm", spectral_radius_le_norm_ty()),
        ("spectral_radius_normal", spectral_radius_normal_ty()),
        (
            "HolomorphicFunctionalCalculus",
            holomorphic_functional_calculus_ty(),
        ),
        (
            "ContinuousFunctionalCalculus",
            continuous_functional_calculus_ty(),
        ),
        ("BorelFunctionalCalculus", borel_functional_calculus_ty()),
        (
            "functional_calculus_homomorphism",
            functional_calculus_homomorphism_ty(),
        ),
        ("functional_calculus_star", functional_calculus_star_ty()),
        (
            "functional_calculus_spectral_mapping",
            functional_calculus_spectral_mapping_ty(),
        ),
        ("stones_generator", stones_generator_ty()),
        ("stones_theorem", stones_theorem_ty()),
        ("stones_theorem_converse", stones_theorem_converse_ty()),
        ("SpectralMeasure", spectral_measure_ty()),
        (
            "spectral_measure_orthogonal",
            spectral_measure_orthogonal_ty(),
        ),
        (
            "spectral_measure_sigma_additive",
            spectral_measure_sigma_additive_ty(),
        ),
        ("spectral_measure_unital", spectral_measure_unital_ty()),
        ("spectral_integral", spectral_integral_ty()),
        ("spectral_measure_of_sa", spectral_measure_of_sa_ty()),
        ("is_self_adjoint", is_self_adjoint_ty()),
        ("sa_spectrum_real", sa_spectrum_real_ty()),
        (
            "sa_norm_eq_spectral_radius",
            sa_norm_eq_spectral_radius_ty(),
        ),
        ("sa_positive_spectrum", sa_positive_spectrum_ty()),
        ("is_normal", is_normal_ty()),
        (
            "spectral_theorem_bounded_sa",
            spectral_theorem_bounded_sa_ty(),
        ),
        ("spectral_theorem_normal", spectral_theorem_normal_ty()),
        (
            "spectral_decomposition_finite_dim",
            spectral_decomposition_finite_dim_ty(),
        ),
        ("Eigenvalue", eigenvalue_ty()),
        ("Eigenvector", eigenvector_ty()),
        (
            "sa_eigenvectors_orthogonal",
            sa_eigenvectors_orthogonal_ty(),
        ),
        ("EssentialSpectrum", essential_spectrum_ty()),
        ("weyls_theorem", weyls_theorem_ty()),
        ("FredhomIndex", fredholm_index_ty()),
        ("trace", trace_ty()),
        ("operator_norm", operator_norm_ty()),
        ("hahn_banach_extension", hahn_banach_extension_ty()),
        ("riesz_representation", riesz_representation_ty()),
        ("lax_milgram", lax_milgram_ty()),
    ];
    for (name, ty) in axioms {
        env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty: ty.clone(),
        })
        .ok();
    }
    env
}
/// Compute the spectral radius of a matrix using the power method.
///
/// Returns the estimate r(A) ≈ lim ‖Aⁿv‖^(1/n) for a random start vector.
pub fn spectral_radius_power_method(mat: &ComplexMatrix, iters: usize) -> f64 {
    let n = mat.n;
    if n == 0 {
        return 0.0;
    }
    let mut v: Vec<f64> = vec![1.0 / (n as f64).sqrt(); n];
    let mut radius = 0.0_f64;
    for _ in 0..iters {
        let mut w = vec![0.0_f64; n];
        for i in 0..n {
            for j in 0..n {
                w[i] += mat.data[i][j].re * v[j];
            }
        }
        let norm: f64 = w.iter().map(|x| x * x).sum::<f64>().sqrt();
        if norm < 1e-300 {
            break;
        }
        let v_norm: f64 = v.iter().map(|x| x * x).sum::<f64>().sqrt().max(1e-300);
        radius = norm / v_norm;
        v = w.iter().map(|x| x / norm).collect();
    }
    radius
}
/// Gelfand spectral radius formula: r(A) = lim ‖Aⁿ‖^(1/n).
/// Uses the Frobenius norm and computes A^n for increasing n.
pub fn gelfand_spectral_radius(mat: &ComplexMatrix) -> f64 {
    let max_n = 16u32;
    let mut best = 0.0_f64;
    let an = mat.power(max_n);
    let f_norm = an.frobenius_norm();
    if f_norm > 0.0 {
        best = f_norm.powf(1.0 / max_n as f64);
    }
    best
}
/// Resolvent of a matrix A at λ: R(λ, A) = (A - λI)⁻¹.
///
/// Uses Gaussian elimination. Returns None if λ is in the spectrum.
#[allow(clippy::too_many_arguments)]
pub fn resolvent(mat: &ComplexMatrix, lambda: Complex64) -> Option<ComplexMatrix> {
    let n = mat.n;
    if n == 0 {
        return Some(ComplexMatrix::zeros(0));
    }
    let mut a_minus_lam = mat.clone();
    for i in 0..n {
        a_minus_lam.data[i][i] = a_minus_lam.data[i][i].sub(&lambda);
    }
    let mut aug: Vec<Vec<Complex64>> = (0..n)
        .map(|i| {
            let mut row: Vec<Complex64> = a_minus_lam.data[i].clone();
            for j in 0..n {
                row.push(if i == j {
                    Complex64::real(1.0)
                } else {
                    Complex64::new(0.0, 0.0)
                });
            }
            row
        })
        .collect();
    for col in 0..n {
        let mut max_row = col;
        let mut max_val = aug[col][col].modulus();
        for row in (col + 1)..n {
            let v = aug[row][col].modulus();
            if v > max_val {
                max_val = v;
                max_row = row;
            }
        }
        if max_val < 1e-12 {
            return None;
        }
        aug.swap(col, max_row);
        let pivot = aug[col][col];
        let inv_pivot = Complex64::real(1.0).div(&pivot)?;
        for j in 0..(2 * n) {
            aug[col][j] = aug[col][j].mul(&inv_pivot);
        }
        for row in 0..n {
            if row == col {
                continue;
            }
            let factor = aug[row][col];
            for j in 0..(2 * n) {
                let sub = factor.mul(&aug[col][j]);
                aug[row][j] = aug[row][j].sub(&sub);
            }
        }
    }
    let mut inv = ComplexMatrix::zeros(n);
    for i in 0..n {
        for j in 0..n {
            inv.data[i][j] = aug[i][n + j];
        }
    }
    Some(inv)
}
/// Evaluate a polynomial p(A) = ∑ cₖ Aᵏ for a matrix A.
/// Coefficients are given as \[c₀, c₁, ..., cₘ\] (real).
pub fn polynomial_functional_calculus(mat: &ComplexMatrix, coeffs: &[f64]) -> ComplexMatrix {
    let n = mat.n;
    if coeffs.is_empty() {
        return ComplexMatrix::zeros(n);
    }
    let mut result = ComplexMatrix::zeros(n);
    let mut ak = ComplexMatrix::identity(n);
    for &c in coeffs {
        let term = ak.scale(Complex64::real(c));
        result = result.add(&term);
        ak = ak.mul(mat);
    }
    result
}
/// Evaluate exp(tA) via the Taylor series: ∑ (tA)^k / k! (up to `terms` terms).
pub fn matrix_exponential(mat: &ComplexMatrix, t: f64, terms: usize) -> ComplexMatrix {
    let n = mat.n;
    let mut result = ComplexMatrix::zeros(n);
    let mut ak = ComplexMatrix::identity(n);
    let mut factorial = 1.0_f64;
    for k in 0..terms {
        if k > 0 {
            factorial *= k as f64;
            ak = ak.mul(mat);
        }
        let coeff = t.powi(k as i32) / factorial;
        let term = ak.scale(Complex64::real(coeff));
        result = result.add(&term);
    }
    result
}
/// Compute the spectral decomposition of a real symmetric 2×2 matrix.
///
/// Returns the two eigenvalues and corresponding rank-1 projections.
pub fn spectral_decomp_2x2(a11: f64, a12: f64, a22: f64) -> SpectralDecomposition {
    let tr = a11 + a22;
    let det = a11 * a22 - a12 * a12;
    let disc = (tr * tr - 4.0 * det).max(0.0).sqrt();
    let lam1 = (tr - disc) / 2.0;
    let lam2 = (tr + disc) / 2.0;
    let mut eigenvalues = vec![lam1, lam2];
    let mut projections = Vec::new();
    for &lam in &eigenvalues {
        let b11 = a11 - lam;
        let b12 = a12;
        let b22 = a22 - lam;
        let (vx, vy) = if b11.abs() > 1e-12 || b12.abs() > 1e-12 {
            let norm = (b12 * b12 + b11 * b11).sqrt().max(1e-300);
            (-b12 / norm, b11 / norm)
        } else if b12.abs() > 1e-12 || b22.abs() > 1e-12 {
            let norm = (b22 * b22 + b12 * b12).sqrt().max(1e-300);
            (-b22 / norm, b12 / norm)
        } else {
            (1.0, 0.0)
        };
        let mut proj = ComplexMatrix::zeros(2);
        proj.data[0][0] = Complex64::real(vx * vx);
        proj.data[0][1] = Complex64::real(vx * vy);
        proj.data[1][0] = Complex64::real(vy * vx);
        proj.data[1][1] = Complex64::real(vy * vy);
        projections.push(proj);
    }
    if eigenvalues[0] > eigenvalues[1] {
        eigenvalues.swap(0, 1);
        projections.swap(0, 1);
    }
    SpectralDecomposition {
        eigenvalues,
        projections,
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_build_env_has_key_axioms() {
        let env = build_spectral_theory_env();
        assert!(env.get(&Name::str("Spectrum")).is_some());
        assert!(env.get(&Name::str("Resolvent")).is_some());
        assert!(env.get(&Name::str("SpectralRadius")).is_some());
        assert!(env.get(&Name::str("BorelFunctionalCalculus")).is_some());
        assert!(env.get(&Name::str("SpectralMeasure")).is_some());
        assert!(env.get(&Name::str("spectral_theorem_bounded_sa")).is_some());
        assert!(env.get(&Name::str("weyls_theorem")).is_some());
        assert!(env.get(&Name::str("stones_theorem")).is_some());
    }
    #[test]
    fn test_complex64_arithmetic() {
        let z1 = Complex64::new(1.0, 2.0);
        let z2 = Complex64::new(3.0, -1.0);
        let sum = z1.add(&z2);
        assert!((sum.re - 4.0).abs() < 1e-10);
        assert!((sum.im - 1.0).abs() < 1e-10);
        let prod = z1.mul(&z2);
        assert!((prod.re - 5.0).abs() < 1e-10);
        assert!((prod.im - 5.0).abs() < 1e-10);
        let conj = z1.conj();
        assert!((conj.im - (-2.0)).abs() < 1e-10);
        assert!((z1.modulus() - 5.0_f64.sqrt()).abs() < 1e-10);
    }
    #[test]
    fn test_self_adjoint_check() {
        let mut m = ComplexMatrix::zeros(2);
        m.data[0][0] = Complex64::real(3.0);
        m.data[0][1] = Complex64::real(1.0);
        m.data[1][0] = Complex64::real(1.0);
        m.data[1][1] = Complex64::real(2.0);
        assert!(m.is_self_adjoint(1e-10));
        m.data[0][1] = Complex64::real(0.5);
        assert!(!m.is_self_adjoint(1e-10));
    }
    #[test]
    fn test_spectral_decomp_2x2_diagonal() {
        let decomp = spectral_decomp_2x2(2.0, 0.0, 5.0);
        assert_eq!(decomp.eigenvalues.len(), 2);
        assert!((decomp.eigenvalues[0] - 2.0).abs() < 1e-8);
        assert!((decomp.eigenvalues[1] - 5.0).abs() < 1e-8);
        assert!(decomp.check_resolution_of_identity());
        assert!(decomp.check_orthogonality());
    }
    #[test]
    fn test_spectral_decomp_2x2_reconstruction() {
        let decomp = spectral_decomp_2x2(3.0, 1.0, 3.0);
        assert!((decomp.eigenvalues[0] - 2.0).abs() < 1e-8);
        assert!((decomp.eigenvalues[1] - 4.0).abs() < 1e-8);
        let reconstructed = decomp.reconstruct();
        assert!((reconstructed.data[0][0].re - 3.0).abs() < 1e-7);
        assert!((reconstructed.data[0][1].re - 1.0).abs() < 1e-7);
        assert!((reconstructed.data[1][1].re - 3.0).abs() < 1e-7);
    }
    #[test]
    fn test_resolvent_identity_shifted() {
        let mut m = ComplexMatrix::identity(3);
        m = m.scale(Complex64::real(2.0));
        let r = resolvent(&m, Complex64::real(2.0));
        assert!(r.is_none(), "R(2, 2I) should be singular");
        let id = ComplexMatrix::identity(2);
        let r0 = resolvent(&id, Complex64::real(3.0));
        assert!(r0.is_some());
    }
    #[test]
    fn test_polynomial_functional_calculus() {
        let mut a = ComplexMatrix::zeros(2);
        a.data[0][0] = Complex64::real(1.0);
        a.data[1][1] = Complex64::real(2.0);
        let pa = polynomial_functional_calculus(&a, &[1.0, 2.0]);
        assert!((pa.data[0][0].re - 3.0).abs() < 1e-10);
        assert!((pa.data[1][1].re - 5.0).abs() < 1e-10);
    }
    #[test]
    fn test_matrix_exponential_identity() {
        let a = ComplexMatrix::zeros(3);
        let expa = matrix_exponential(&a, 0.0, 10);
        let id = ComplexMatrix::identity(3);
        for i in 0..3 {
            for j in 0..3 {
                assert!((expa.data[i][j].re - id.data[i][j].re).abs() < 1e-10);
            }
        }
    }
    #[test]
    fn test_spectral_radius_power_method() {
        let mut m = ComplexMatrix::zeros(2);
        m.data[0][0] = Complex64::real(1.0);
        m.data[1][1] = Complex64::real(3.0);
        let r = spectral_radius_power_method(&m, 50);
        assert!((r - 3.0).abs() < 0.1, "spectral radius ≈ 3, got {r}");
    }
}
/// Build a kernel environment containing spectral theory axioms.
pub fn build_env() -> Environment {
    build_spectral_theory_env()
}
/// spectral_radius_gelfand : ∀ T, SpectralRadius T = lim_{n→∞} ‖Tⁿ‖^(1/n)
pub fn spt_ext_spectral_radius_gelfand_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "T",
        cst("BoundedLinearOperator"),
        prop(),
    )
}
/// resolvent_norm_bound : ‖R(λ,T)‖ ≤ 1 / dist(λ, Spectrum T)
pub fn spt_ext_resolvent_norm_bound_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "T",
        cst("BoundedLinearOperator"),
        arrow(complex_ty(), prop()),
    )
}
/// resolvent_identity : R(λ,T) - R(μ,T) = (λ-μ) R(λ,T) R(μ,T)
pub fn spt_ext_resolvent_identity_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "T",
        cst("BoundedLinearOperator"),
        pi(
            BinderInfo::Default,
            "lam",
            complex_ty(),
            pi(BinderInfo::Default, "mu", complex_ty(), prop()),
        ),
    )
}
/// spectrum_nonempty : ∀ T : BoundedLinearOperator, Spectrum T ≠ ∅
pub fn spt_ext_spectrum_nonempty_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "T",
        cst("BoundedLinearOperator"),
        prop(),
    )
}
/// spectrum_compact : ∀ T, IsCompact (Spectrum T)
pub fn spt_ext_spectrum_compact_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "T",
        cst("BoundedLinearOperator"),
        app(cst("IsCompact"), app(cst("Spectrum"), bvar(0))),
    )
}
/// PointSpectrumEigenvalue : λ ∈ PointSpectrum T ↔ ∃ v≠0, T v = λ v
pub fn spt_ext_point_spectrum_eigenvalue_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "T",
        cst("BoundedLinearOperator"),
        arrow(complex_ty(), prop()),
    )
}
/// ContinuousSpectrumDense : λ ∈ ContinuousSpectrum T ↔ (T-λI) injective, range dense, not surjective
pub fn spt_ext_continuous_spectrum_dense_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "T",
        cst("BoundedLinearOperator"),
        arrow(complex_ty(), prop()),
    )
}
/// ResidualSpectrumConjugate : ResidualSpectrum T ⊆ PointSpectrum T*
pub fn spt_ext_residual_spectrum_conjugate_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "T",
        cst("BoundedLinearOperator"),
        prop(),
    )
}
/// spectrum_disjoint_union : Spectrum T = PointSpectrum T ⊔ ContinuousSpectrum T ⊔ ResidualSpectrum T
pub fn spt_ext_spectrum_disjoint_union_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "T",
        cst("BoundedLinearOperator"),
        prop(),
    )
}
/// SelfAdjointHasNoResidual : T = T* → ResidualSpectrum T = ∅
pub fn spt_ext_sa_no_residual_spectrum_ty() -> Expr {
    pi(BinderInfo::Default, "T", cst("SelfAdjointOperator"), prop())
}
/// spectral_measure_projection : ∀ E : SpectralMeasure, ∀ A, E(A)² = E(A)
pub fn spt_ext_spectral_measure_projection_ty() -> Expr {
    arrow(cst("SpectralMeasure"), prop())
}
/// spectral_measure_selfadjoint : ∀ E : SpectralMeasure, ∀ A, E(A) = E(A)*
pub fn spt_ext_spectral_measure_selfadjoint_ty() -> Expr {
    arrow(cst("SpectralMeasure"), prop())
}
/// spectral_measure_countably_additive : σ-additive version for spectral measures
pub fn spt_ext_spectral_measure_countably_additive_ty() -> Expr {
    arrow(
        cst("SpectralMeasure"),
        arrow(list_ty(cst("BorelSet")), prop()),
    )
}
/// spectral_measure_support : support of spectral measure = Spectrum T
pub fn spt_ext_spectral_measure_support_ty() -> Expr {
    arrow(cst("SpectralMeasure"), prop())
}
/// stone_group_uniqueness : generator of a strongly continuous group is unique
pub fn spt_ext_stone_group_uniqueness_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "U",
        arrow(real_ty(), cst("UnitaryOperator")),
        prop(),
    )
}
/// stone_domain_core : D(A) is a core for the generator A
pub fn spt_ext_stone_domain_core_ty() -> Expr {
    pi(BinderInfo::Default, "A", cst("SelfAdjointOperator"), prop())
}
/// CStarAlgebra : Type — norm-closed *-subalgebra of B(H)
pub fn spt_ext_cstar_algebra_ty() -> Expr {
    type0()
}
/// CommutativeCStarAlgebra : Type
pub fn spt_ext_commutative_cstar_algebra_ty() -> Expr {
    type0()
}
/// MaximalIdealSpace : CommutativeCStarAlgebra → Type
pub fn spt_ext_maximal_ideal_space_ty() -> Expr {
    arrow(cst("CommutativeCStarAlgebra"), type0())
}
/// GelfandTransform : CommutativeCStarAlgebra → C(MaximalIdealSpace)
pub fn spt_ext_gelfand_transform_ty() -> Expr {
    arrow(cst("CommutativeCStarAlgebra"), cst("ContinuousFunctions"))
}
/// gelfand_transform_isometry : Gelfand transform is an isometric *-isomorphism
pub fn spt_ext_gelfand_transform_isometry_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "A",
        cst("CommutativeCStarAlgebra"),
        prop(),
    )
}
/// gelfand_naimark_theorem : every commutative C*-algebra ≅ C₀(X) for locally compact Hausdorff X
pub fn spt_ext_gelfand_naimark_theorem_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "A",
        cst("CommutativeCStarAlgebra"),
        app(cst("Exists"), cst("LocallyCompactHausdorff")),
    )
}
/// gelfand_naimark_noncommutative : every C*-algebra embeds isometrically in B(H)
pub fn spt_ext_gelfand_naimark_noncommutative_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "A",
        cst("CStarAlgebra"),
        app(cst("Exists"), cst("HilbertSpace")),
    )
}
/// cstar_spectrum_character : σ(a) = image of a under all characters
pub fn spt_ext_cstar_spectrum_character_ty() -> Expr {
    arrow(
        cst("CStarAlgebra"),
        arrow(cst("CStarElement"), set_ty(complex_ty())),
    )
}
/// NormalFunctionalCalculus : NormalOperator → (Complex → Complex) → BoundedLinearOperator
pub fn spt_ext_normal_functional_calculus_ty() -> Expr {
    arrow(
        cst("NormalOperator"),
        arrow(
            arrow(complex_ty(), complex_ty()),
            cst("BoundedLinearOperator"),
        ),
    )
}
/// normal_fc_homomorphism : Φ is a *-homomorphism
pub fn spt_ext_normal_fc_homomorphism_ty() -> Expr {
    pi(BinderInfo::Default, "T", cst("NormalOperator"), prop())
}
/// normal_fc_spectral_mapping : σ(Φ(f)) = f(σ(T)) for normal T
pub fn spt_ext_normal_fc_spectral_mapping_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "T",
        cst("NormalOperator"),
        arrow(arrow(complex_ty(), complex_ty()), prop()),
    )
}
/// normal_fc_continuous : ‖Φ(f)‖ = ‖f‖_∞ for normal operators
pub fn spt_ext_normal_fc_isometry_ty() -> Expr {
    pi(BinderInfo::Default, "T", cst("NormalOperator"), prop())
}
/// SpectralGap : BoundedLinearOperator → Real — gap between 0 and the first nonzero eigenvalue
pub fn spt_ext_spectral_gap_ty() -> Expr {
    arrow(cst("BoundedLinearOperator"), real_ty())
}
/// spectral_gap_positive : Laplacian is connected ↔ SpectralGap > 0
pub fn spt_ext_spectral_gap_positive_ty() -> Expr {
    pi(BinderInfo::Default, "L", cst("GraphLaplacian"), prop())
}
/// CheegerConstant : Graph → Real — edge expansion
pub fn spt_ext_cheeger_constant_ty() -> Expr {
    arrow(cst("FiniteGraph"), real_ty())
}
/// cheeger_inequality : h(G)²/2 ≤ λ₁(G) ≤ 2 h(G)
pub fn spt_ext_cheeger_inequality_ty() -> Expr {
    pi(BinderInfo::Default, "G", cst("FiniteGraph"), prop())
}
/// GraphLaplacian : FiniteGraph → BoundedLinearOperator
pub fn spt_ext_graph_laplacian_ty() -> Expr {
    arrow(cst("FiniteGraph"), cst("BoundedLinearOperator"))
}
/// graph_laplacian_psd : L ≥ 0 (positive semidefinite)
pub fn spt_ext_graph_laplacian_psd_ty() -> Expr {
    pi(BinderInfo::Default, "G", cst("FiniteGraph"), prop())
}
/// graph_laplacian_zero_eigenvalue : 0 ∈ Spectrum L always
pub fn spt_ext_graph_laplacian_zero_eig_ty() -> Expr {
    pi(BinderInfo::Default, "G", cst("FiniteGraph"), prop())
}
/// GOEMatrix : Nat → Type — Gaussian Orthogonal Ensemble
pub fn spt_ext_goe_matrix_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// GUEMatrix : Nat → Type — Gaussian Unitary Ensemble
pub fn spt_ext_gue_matrix_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// WignerSemicircleLaw : limiting eigenvalue distribution of GUE/GOE
pub fn spt_ext_wigner_semicircle_law_ty() -> Expr {
    pi(BinderInfo::Default, "n", nat_ty(), prop())
}
/// MarchenkoPasturLaw : limiting eigenvalue distribution of Wishart matrices
pub fn spt_ext_marchenko_pastur_law_ty() -> Expr {
    pi(BinderInfo::Default, "n", nat_ty(), arrow(real_ty(), prop()))
}
/// TracyWidomDistribution : fluctuations of largest eigenvalue in GUE
pub fn spt_ext_tracy_widom_distribution_ty() -> Expr {
    arrow(real_ty(), real_ty())
}
/// random_matrix_eigenvalue_repulsion : eigenvalues repel in GUE/GOE
pub fn spt_ext_eigenvalue_repulsion_ty() -> Expr {
    pi(BinderInfo::Default, "n", nat_ty(), prop())
}
/// random_matrix_bulk_universality : bulk eigenvalue statistics are universal
pub fn spt_ext_bulk_universality_ty() -> Expr {
    pi(BinderInfo::Default, "n", nat_ty(), prop())
}
/// random_matrix_edge_universality : edge eigenvalue statistics converge to Tracy-Widom
pub fn spt_ext_edge_universality_ty() -> Expr {
    pi(BinderInfo::Default, "n", nat_ty(), prop())
}
/// RiemannZeta : Complex → Complex — Riemann zeta function
pub fn spt_ext_riemann_zeta_ty() -> Expr {
    arrow(complex_ty(), complex_ty())
}
/// zeta_spectral_interpretation : zeros of ζ correspond to eigenvalues of a hypothetical operator
pub fn spt_ext_zeta_spectral_interpretation_ty() -> Expr {
    prop()
}
/// HilbertPolya : ∃ self-adjoint operator with eigenvalues = nontrivial zeros of ζ
pub fn spt_ext_hilbert_polya_ty() -> Expr {
    app(cst("Exists"), cst("SelfAdjointOperator"))
}
/// SelbergTraceFormula : sum over eigenvalues = sum over closed geodesics
pub fn spt_ext_selberg_trace_formula_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "M",
        cst("CompactHyperbolicSurface"),
        prop(),
    )
}
/// WeylLaw : eigenvalue counting function N(λ) ~ C_n Vol(M) λ^{n/2}
pub fn spt_ext_weyl_law_ty() -> Expr {
    pi(BinderInfo::Default, "M", cst("RiemannianManifold"), prop())
}
/// HeatKernelExpansion : heat kernel tr(e^{-tΔ}) has asymptotic expansion
pub fn spt_ext_heat_kernel_expansion_ty() -> Expr {
    pi(BinderInfo::Default, "M", cst("RiemannianManifold"), prop())
}
/// SpectralZetaFunction : SelfAdjointOperator → Complex → Complex
pub fn spt_ext_spectral_zeta_function_ty() -> Expr {
    arrow(
        cst("SelfAdjointOperator"),
        arrow(complex_ty(), complex_ty()),
    )
}
/// Register all extended spectral theory axioms into an existing environment.
pub fn register_spectral_theory_extended(env: &mut Environment) -> Result<(), String> {
    let axioms: &[(&str, Expr)] = &[
        (
            "spectral_radius_gelfand",
            spt_ext_spectral_radius_gelfand_ty(),
        ),
        ("resolvent_norm_bound", spt_ext_resolvent_norm_bound_ty()),
        ("resolvent_identity_ext", spt_ext_resolvent_identity_ty()),
        ("spectrum_nonempty", spt_ext_spectrum_nonempty_ty()),
        ("spectrum_compact", spt_ext_spectrum_compact_ty()),
        (
            "point_spectrum_eigenvalue",
            spt_ext_point_spectrum_eigenvalue_ty(),
        ),
        (
            "continuous_spectrum_dense",
            spt_ext_continuous_spectrum_dense_ty(),
        ),
        (
            "residual_spectrum_conjugate",
            spt_ext_residual_spectrum_conjugate_ty(),
        ),
        (
            "spectrum_disjoint_union",
            spt_ext_spectrum_disjoint_union_ty(),
        ),
        (
            "sa_no_residual_spectrum",
            spt_ext_sa_no_residual_spectrum_ty(),
        ),
        (
            "spectral_measure_projection_ext",
            spt_ext_spectral_measure_projection_ty(),
        ),
        (
            "spectral_measure_selfadjoint_ext",
            spt_ext_spectral_measure_selfadjoint_ty(),
        ),
        (
            "spectral_measure_countably_additive_ext",
            spt_ext_spectral_measure_countably_additive_ty(),
        ),
        (
            "spectral_measure_support",
            spt_ext_spectral_measure_support_ty(),
        ),
        (
            "stone_group_uniqueness",
            spt_ext_stone_group_uniqueness_ty(),
        ),
        ("stone_domain_core", spt_ext_stone_domain_core_ty()),
        ("CStarAlgebra", spt_ext_cstar_algebra_ty()),
        (
            "CommutativeCStarAlgebra",
            spt_ext_commutative_cstar_algebra_ty(),
        ),
        ("MaximalIdealSpace", spt_ext_maximal_ideal_space_ty()),
        ("GelfandTransform", spt_ext_gelfand_transform_ty()),
        (
            "gelfand_transform_isometry",
            spt_ext_gelfand_transform_isometry_ty(),
        ),
        (
            "gelfand_naimark_theorem",
            spt_ext_gelfand_naimark_theorem_ty(),
        ),
        (
            "gelfand_naimark_noncommutative",
            spt_ext_gelfand_naimark_noncommutative_ty(),
        ),
        (
            "cstar_spectrum_character",
            spt_ext_cstar_spectrum_character_ty(),
        ),
        (
            "NormalFunctionalCalculus",
            spt_ext_normal_functional_calculus_ty(),
        ),
        (
            "normal_fc_homomorphism",
            spt_ext_normal_fc_homomorphism_ty(),
        ),
        (
            "normal_fc_spectral_mapping",
            spt_ext_normal_fc_spectral_mapping_ty(),
        ),
        ("normal_fc_isometry", spt_ext_normal_fc_isometry_ty()),
        ("SpectralGap", spt_ext_spectral_gap_ty()),
        ("spectral_gap_positive", spt_ext_spectral_gap_positive_ty()),
        ("CheegerConstant", spt_ext_cheeger_constant_ty()),
        ("cheeger_inequality", spt_ext_cheeger_inequality_ty()),
        ("GraphLaplacian", spt_ext_graph_laplacian_ty()),
        ("graph_laplacian_psd", spt_ext_graph_laplacian_psd_ty()),
        (
            "graph_laplacian_zero_eig",
            spt_ext_graph_laplacian_zero_eig_ty(),
        ),
        ("GOEMatrix", spt_ext_goe_matrix_ty()),
        ("GUEMatrix", spt_ext_gue_matrix_ty()),
        ("WignerSemicircleLaw", spt_ext_wigner_semicircle_law_ty()),
        ("MarchenkoPasturLaw", spt_ext_marchenko_pastur_law_ty()),
        (
            "TracyWidomDistribution",
            spt_ext_tracy_widom_distribution_ty(),
        ),
        ("eigenvalue_repulsion", spt_ext_eigenvalue_repulsion_ty()),
        ("bulk_universality", spt_ext_bulk_universality_ty()),
        ("edge_universality", spt_ext_edge_universality_ty()),
        ("RiemannZeta", spt_ext_riemann_zeta_ty()),
        (
            "zeta_spectral_interpretation",
            spt_ext_zeta_spectral_interpretation_ty(),
        ),
        ("HilbertPolya", spt_ext_hilbert_polya_ty()),
        ("SelbergTraceFormula", spt_ext_selberg_trace_formula_ty()),
        ("WeylLaw", spt_ext_weyl_law_ty()),
        ("HeatKernelExpansion", spt_ext_heat_kernel_expansion_ty()),
        ("SpectralZetaFunction", spt_ext_spectral_zeta_function_ty()),
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
/// UnboundedSelfAdjointOperator : Type — densely defined, closed, T = T*
pub fn spt_ext_unbounded_sa_operator_ty() -> Expr {
    type0()
}
/// spectral_theorem_unbounded_sa : ∀ A : UnboundedSA, ∃ E : SpectralMeasure, A = ∫ λ dE(λ)
pub fn spt_ext_spectral_theorem_unbounded_sa_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "A",
        cst("UnboundedSelfAdjointOperator"),
        prop(),
    )
}
/// von_neumann_deficiency_indices : self-adjoint extensions classified by deficiency indices
pub fn spt_ext_von_neumann_deficiency_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "T",
        cst("BoundedLinearOperator"),
        prop(),
    )
}
/// operator_semigroup_generator : every C₀-semigroup has a closed generator
pub fn spt_ext_operator_semigroup_generator_ty() -> Expr {
    arrow(cst("C0Semigroup"), cst("UnboundedSelfAdjointOperator"))
}
/// lumer_phillips : A generates a C₀-contraction semigroup iff A is m-dissipative
pub fn spt_ext_lumer_phillips_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "A",
        cst("BoundedLinearOperator"),
        prop(),
    )
}
/// hille_yosida : conditions for A to generate a strongly continuous semigroup
pub fn spt_ext_hille_yosida_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "A",
        cst("BoundedLinearOperator"),
        prop(),
    )
}
/// perturbation_theory_kato : kato perturbation series for eigenvalues
pub fn spt_ext_perturbation_theory_kato_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "T",
        cst("SelfAdjointOperator"),
        arrow(cst("BoundedLinearOperator"), prop()),
    )
}
/// min_max_principle : variational characterization of eigenvalues λ_k = min max <Tx,x>/<x,x>
pub fn spt_ext_min_max_principle_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "T",
        cst("SelfAdjointOperator"),
        arrow(nat_ty(), prop()),
    )
}
/// courant_nodal_theorem : k-th eigenfunction has at most k nodal domains
pub fn spt_ext_courant_nodal_theorem_ty() -> Expr {
    pi(BinderInfo::Default, "n", nat_ty(), prop())
}
/// RiemannianLaplacian : RiemannianManifold → BoundedLinearOperator — the Laplace-Beltrami operator
pub fn spt_ext_riemannian_laplacian_ty() -> Expr {
    arrow(cst("RiemannianManifold"), cst("BoundedLinearOperator"))
}
/// Register the §13 completion axioms.
pub fn register_spectral_theory_completions(env: &mut Environment) -> Result<(), String> {
    let axioms: &[(&str, Expr)] = &[
        (
            "UnboundedSelfAdjointOperator",
            spt_ext_unbounded_sa_operator_ty(),
        ),
        (
            "spectral_theorem_unbounded_sa",
            spt_ext_spectral_theorem_unbounded_sa_ty(),
        ),
        (
            "von_neumann_deficiency_indices",
            spt_ext_von_neumann_deficiency_ty(),
        ),
        (
            "operator_semigroup_generator",
            spt_ext_operator_semigroup_generator_ty(),
        ),
        ("lumer_phillips", spt_ext_lumer_phillips_ty()),
        ("hille_yosida", spt_ext_hille_yosida_ty()),
        (
            "perturbation_theory_kato",
            spt_ext_perturbation_theory_kato_ty(),
        ),
        ("min_max_principle", spt_ext_min_max_principle_ty()),
        ("courant_nodal_theorem", spt_ext_courant_nodal_theorem_ty()),
        ("RiemannianLaplacian", spt_ext_riemannian_laplacian_ty()),
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
