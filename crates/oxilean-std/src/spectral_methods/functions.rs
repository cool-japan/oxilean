//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)
#![allow(clippy::items_after_test_module)]

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};
use std::f64::consts::PI;

use super::types::{Complex, ExponentialConvergence};

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
pub fn complex_ty() -> Expr {
    cst("Complex")
}
pub fn bool_ty() -> Expr {
    cst("Bool")
}
pub fn list_ty(a: Expr) -> Expr {
    app(cst("List"), a)
}
pub fn fn_ty(dom: Expr, cod: Expr) -> Expr {
    arrow(dom, cod)
}
pub fn vec_ty(a: Expr) -> Expr {
    app(cst("Vec"), a)
}
/// `FourierCoefficient : (Real → Complex) → Int → Complex`
///
/// The k-th Fourier coefficient of a periodic function f:
/// c_k = (1/2π) ∫₀²π f(x) e^{-ikx} dx.
pub fn fourier_coefficient_ty() -> Expr {
    arrow(
        fn_ty(real_ty(), complex_ty()),
        arrow(cst("Int"), complex_ty()),
    )
}
/// `TruncatedFourierSeries : (Real → Complex) → Nat → Real → Complex`
///
/// The N-th partial Fourier sum S_N f(x) = Σ_{|k|≤N} c_k e^{ikx}.
pub fn truncated_fourier_series_ty() -> Expr {
    arrow(
        fn_ty(real_ty(), complex_ty()),
        arrow(nat_ty(), fn_ty(real_ty(), complex_ty())),
    )
}
/// `SpectralDifferentiation : (Real → Complex) → Nat → Real → Complex`
///
/// Differentiation via Fourier modes: d/dx S_N f(x) = Σ_{|k|≤N} (ik) c_k e^{ikx}.
pub fn spectral_differentiation_ty() -> Expr {
    arrow(
        fn_ty(real_ty(), complex_ty()),
        arrow(nat_ty(), fn_ty(real_ty(), complex_ty())),
    )
}
/// `SpectralConvergence : (Real → Complex) → Prop`
///
/// Exponential convergence: if f ∈ C^∞ and f is periodic, then for all α > 0
/// there exists C > 0 such that ‖f - S_N f‖ ≤ C e^{-αN}.
pub fn spectral_convergence_ty() -> Expr {
    arrow(fn_ty(real_ty(), complex_ty()), prop())
}
/// `GibbsPhenomenon : (Real → Complex) → Prop`
///
/// Gibbs overshoot near jump discontinuities: the Fourier series overshoots
/// by approximately 9% of the jump height regardless of N.
pub fn gibbs_phenomenon_ty() -> Expr {
    arrow(fn_ty(real_ty(), complex_ty()), prop())
}
/// `AliasingError : Nat → (Real → Complex) → Real`
///
/// Aliasing error for N-point DFT: contributions from modes |k| > N/2 fold
/// back and contaminate the lower modes.
pub fn aliasing_error_ty() -> Expr {
    arrow(nat_ty(), arrow(fn_ty(real_ty(), complex_ty()), real_ty()))
}
/// `ChebyshevPolynomial : Nat → Real → Real`
///
/// T_n(x) = cos(n arccos x) for x ∈ \[-1, 1\], defined by the recurrence
/// T_0 = 1, T_1 = x, T_{n+1} = 2x T_n - T_{n-1}.
pub fn chebyshev_polynomial_ty() -> Expr {
    arrow(nat_ty(), fn_ty(real_ty(), real_ty()))
}
/// `ChebyshevNodes : Nat → List Real`
///
/// The N Chebyshev-Gauss-Lobatto nodes x_j = cos(jπ/N), j = 0,...,N.
pub fn chebyshev_nodes_ty() -> Expr {
    arrow(nat_ty(), list_ty(real_ty()))
}
/// `ChebyshevExpansion : (Real → Real) → Nat → List Real`
///
/// The first N Chebyshev expansion coefficients a_k of f:
/// f(x) ≈ Σ_{k=0}^{N} a_k T_k(x).
pub fn chebyshev_expansion_ty() -> Expr {
    arrow(
        fn_ty(real_ty(), real_ty()),
        arrow(nat_ty(), list_ty(real_ty())),
    )
}
/// `ChebyshevDifferentiationMatrix : Nat → (List (List Real))`
///
/// The (N+1)×(N+1) spectral differentiation matrix D for Chebyshev points
/// such that (Du)_j ≈ u'(x_j).
pub fn chebyshev_diff_matrix_ty() -> Expr {
    arrow(nat_ty(), list_ty(list_ty(real_ty())))
}
/// `ChebyshevInterpolation : List Real → List Real → Real → Real`
///
/// Barycentric Chebyshev interpolation at an arbitrary point x,
/// given function values at Chebyshev nodes.
pub fn chebyshev_interpolation_ty() -> Expr {
    arrow(
        list_ty(real_ty()),
        arrow(list_ty(real_ty()), fn_ty(real_ty(), real_ty())),
    )
}
/// `GaussLegendreNodes : Nat → List Real`
///
/// The N Gauss-Legendre quadrature nodes in \[-1, 1\]: roots of P_N(x).
pub fn gauss_legendre_nodes_ty() -> Expr {
    arrow(nat_ty(), list_ty(real_ty()))
}
/// `GaussLegendreWeights : Nat → List Real`
///
/// The N Gauss-Legendre quadrature weights w_i = 2/((1-x_i²)[P'_N(x_i)]²).
pub fn gauss_legendre_weights_ty() -> Expr {
    arrow(nat_ty(), list_ty(real_ty()))
}
/// `GaussQuadratureExact : Nat → (Real → Real) → Prop`
///
/// N-point Gauss-Legendre quadrature is exact for polynomials of degree ≤ 2N-1.
pub fn gauss_quadrature_exact_ty() -> Expr {
    arrow(nat_ty(), arrow(fn_ty(real_ty(), real_ty()), prop()))
}
/// `GaussChebyshevNodes : Nat → List Real`
///
/// The N Gauss-Chebyshev quadrature nodes x_k = cos((2k-1)π/(2N)).
pub fn gauss_chebyshev_nodes_ty() -> Expr {
    arrow(nat_ty(), list_ty(real_ty()))
}
/// `ClenshawCurtisWeights : Nat → List Real`
///
/// The Clenshaw-Curtis quadrature weights on N+1 Chebyshev nodes.
pub fn clenshaw_curtis_weights_ty() -> Expr {
    arrow(nat_ty(), list_ty(real_ty()))
}
/// `CollocationSolution : (Real → Real → Real) → Nat → Real → Real → Real`
///
/// The pseudospectral collocation solution u_N to a PDE at time t and position x.
pub fn collocation_solution_ty() -> Expr {
    arrow(
        fn_ty(real_ty(), fn_ty(real_ty(), real_ty())),
        arrow(nat_ty(), fn_ty(real_ty(), fn_ty(real_ty(), real_ty()))),
    )
}
/// `DealiasingRule : Nat → Nat`
///
/// The 3/2-rule: to avoid aliasing when computing N-mode convolutions,
/// pad to M = ⌊3N/2⌋ modes before multiplication and truncate back.
pub fn dealiasing_rule_ty() -> Expr {
    arrow(nat_ty(), nat_ty())
}
/// `SpectralElementApprox : Nat → Nat → (Real → Real) → Real → Real`
///
/// Spectral element approximation on K elements each with polynomial degree p.
pub fn spectral_element_approx_ty() -> Expr {
    arrow(
        nat_ty(),
        arrow(
            nat_ty(),
            arrow(fn_ty(real_ty(), real_ty()), fn_ty(real_ty(), real_ty())),
        ),
    )
}
/// `ExponentialIntegrator : (Real → Real) → (Real → Real) → Real → Real → Real`
///
/// The exponential integrator for u_t = Lu + N(u): u(t+h) = e^{Lh}u(t) + h φ_1(Lh) N(u(t)).
pub fn exponential_integrator_ty() -> Expr {
    arrow(
        fn_ty(real_ty(), real_ty()),
        arrow(
            fn_ty(real_ty(), real_ty()),
            fn_ty(real_ty(), fn_ty(real_ty(), real_ty())),
        ),
    )
}
/// `DFTInversion : List Complex → Prop`
///
/// The inverse DFT recovers the original sequence: IDFT(DFT(x)) = x.
pub fn dft_inversion_ty() -> Expr {
    arrow(list_ty(complex_ty()), prop())
}
/// `ParsevalTheorem : List Complex → Prop`
///
/// Parseval's theorem: Σ|x_n|² = (1/N) Σ|X_k|² where X = DFT(x).
pub fn parseval_theorem_ty() -> Expr {
    arrow(list_ty(complex_ty()), prop())
}
/// `ConvolutionTheorem : List Complex → List Complex → Prop`
///
/// DFT(x * y) = DFT(x) · DFT(y) (pointwise product).
pub fn convolution_theorem_ty() -> Expr {
    arrow(list_ty(complex_ty()), arrow(list_ty(complex_ty()), prop()))
}
/// `FFTComplexity : Nat → Nat`
///
/// The FFT runs in O(N log N) operations for N a power of 2.
pub fn fft_complexity_ty() -> Expr {
    arrow(nat_ty(), nat_ty())
}
/// Populate `env` with all spectral-methods axioms and theorem stubs.
pub fn build_spectral_methods_env(env: &mut Environment) {
    let axioms: &[(&str, Expr)] = &[
        ("FourierCoefficient", fourier_coefficient_ty()),
        ("TruncatedFourierSeries", truncated_fourier_series_ty()),
        ("SpectralDifferentiation", spectral_differentiation_ty()),
        ("SpectralConvergence", spectral_convergence_ty()),
        ("GibbsPhenomenon", gibbs_phenomenon_ty()),
        ("AliasingError", aliasing_error_ty()),
        ("ChebyshevPolynomial", chebyshev_polynomial_ty()),
        ("ChebyshevNodes", chebyshev_nodes_ty()),
        ("ChebyshevExpansion", chebyshev_expansion_ty()),
        ("ChebyshevDifferentiationMatrix", chebyshev_diff_matrix_ty()),
        ("ChebyshevInterpolation", chebyshev_interpolation_ty()),
        ("GaussLegendreNodes", gauss_legendre_nodes_ty()),
        ("GaussLegendreWeights", gauss_legendre_weights_ty()),
        ("GaussQuadratureExact", gauss_quadrature_exact_ty()),
        ("GaussChebyshevNodes", gauss_chebyshev_nodes_ty()),
        ("ClenshawCurtisWeights", clenshaw_curtis_weights_ty()),
        ("CollocationSolution", collocation_solution_ty()),
        ("DealiasingRule", dealiasing_rule_ty()),
        ("SpectralElementApprox", spectral_element_approx_ty()),
        ("ExponentialIntegrator", exponential_integrator_ty()),
        ("DFTInversion", dft_inversion_ty()),
        ("ParsevalTheorem", parseval_theorem_ty()),
        ("ConvolutionTheorem", convolution_theorem_ty()),
        ("FFTComplexity", fft_complexity_ty()),
        ("SpectralRadius_SM", real_ty()),
        ("ChebyshevWeight", fn_ty(real_ty(), real_ty())),
        ("LegendrePoly", arrow(nat_ty(), fn_ty(real_ty(), real_ty()))),
        ("PhiFunction", fn_ty(complex_ty(), complex_ty())),
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
/// Evaluate the n-th Chebyshev polynomial T_n(x) via the three-term recurrence.
///
/// T_0(x) = 1, T_1(x) = x, T_{n+1}(x) = 2x T_n(x) - T_{n-1}(x).
pub fn chebyshev_t(n: usize, x: f64) -> f64 {
    if n == 0 {
        return 1.0;
    }
    if n == 1 {
        return x;
    }
    let mut t_prev = 1.0_f64;
    let mut t_curr = x;
    for _ in 2..=n {
        let t_next = 2.0 * x * t_curr - t_prev;
        t_prev = t_curr;
        t_curr = t_next;
    }
    t_curr
}
/// Evaluate T_n(x) using the trigonometric definition cos(n arccos x).
///
/// Numerically stable for x ∈ \[-1, 1\].
pub fn chebyshev_t_trig(n: usize, x: f64) -> f64 {
    let theta = x.clamp(-1.0, 1.0).acos();
    ((n as f64) * theta).cos()
}
/// Compute the N+1 Chebyshev-Gauss-Lobatto nodes on \[-1, 1\]:
/// x_j = cos(jπ/N), j = 0, 1, ..., N.
pub fn chebyshev_gauss_lobatto_nodes(n: usize) -> Vec<f64> {
    (0..=n)
        .map(|j| ((j as f64) * PI / (n as f64)).cos())
        .collect()
}
/// Compute the N Gauss-Chebyshev nodes (interior, type-1):
/// x_k = cos((2k - 1)π / (2N)), k = 1, ..., N.
pub fn gauss_chebyshev_nodes(n: usize) -> Vec<f64> {
    (1..=n)
        .map(|k| (((2 * k - 1) as f64) * PI / (2.0 * n as f64)).cos())
        .collect()
}
/// Evaluate f at Chebyshev-Gauss-Lobatto nodes and return the values.
pub fn sample_at_chebyshev_nodes(f: &dyn Fn(f64) -> f64, n: usize) -> Vec<f64> {
    chebyshev_gauss_lobatto_nodes(n)
        .into_iter()
        .map(f)
        .collect()
}
/// Evaluate a Chebyshev series Σ_{k=0}^{N} a_k T_k(x) at x using Clenshaw's algorithm.
pub fn clenshaw_eval(coeffs: &[f64], x: f64) -> f64 {
    let n = coeffs.len();
    if n == 0 {
        return 0.0;
    }
    if n == 1 {
        return coeffs[0];
    }
    let mut b_next = 0.0_f64;
    let mut b_curr = 0.0_f64;
    for k in (1..n).rev() {
        let b_prev = coeffs[k] + 2.0 * x * b_curr - b_next;
        b_next = b_curr;
        b_curr = b_prev;
    }
    coeffs[0] + x * b_curr - b_next
}
/// Compute the Chebyshev expansion coefficients of f sampled at N+1
/// Gauss-Lobatto nodes using the DCT-I relationship.
///
/// Returns coefficients \[a_0, a_1, ..., a_N\].
pub fn chebyshev_coefficients(values: &[f64]) -> Vec<f64> {
    let n = values.len() - 1;
    let mut coeffs = vec![0.0_f64; n + 1];
    for k in 0..=n {
        let mut sum = 0.0;
        for j in 0..=n {
            let x_j = ((j as f64) * PI / (n as f64)).cos();
            sum += values[j] * chebyshev_t(k, x_j);
        }
        coeffs[k] = sum * 2.0 / (n as f64);
        if k == 0 || k == n {
            coeffs[k] /= 2.0;
        }
    }
    coeffs
}
/// Barycentric Chebyshev interpolation at point x given N+1 values at
/// Gauss-Lobatto nodes.
///
/// Uses barycentric weights w_j = (-1)^j δ_j where δ_0 = δ_N = 1/2, else 1.
pub fn chebyshev_barycentric_interp(nodes: &[f64], values: &[f64], x: f64) -> f64 {
    let n = nodes.len() - 1;
    let weights: Vec<f64> = (0..=n)
        .map(|j| {
            let sign = if j % 2 == 0 { 1.0 } else { -1.0 };
            let half = if j == 0 || j == n { 0.5 } else { 1.0 };
            sign * half
        })
        .collect();
    for j in 0..=n {
        if (x - nodes[j]).abs() < 1e-14 {
            return values[j];
        }
    }
    let mut num = 0.0;
    let mut den = 0.0;
    for j in 0..=n {
        let d = weights[j] / (x - nodes[j]);
        num += d * values[j];
        den += d;
    }
    num / den
}
/// Build the (N+1)×(N+1) Chebyshev spectral differentiation matrix D
/// for Gauss-Lobatto nodes x_j = cos(jπ/N).
///
/// Off-diagonal: D_{ij} = c_i/c_j · (-1)^{i+j} / (x_i - x_j)
/// Diagonal:     D_{ii} = -x_i / (2(1 - x_i²)) for 0 < i < N
///               D_{00} = (2N² + 1)/6, D_{NN} = -(2N² + 1)/6
#[allow(clippy::too_many_arguments)]
pub fn chebyshev_diff_matrix(n: usize) -> Vec<Vec<f64>> {
    let nodes = chebyshev_gauss_lobatto_nodes(n);
    let m = n + 1;
    let mut d = vec![vec![0.0f64; m]; m];
    let c = |j: usize| -> f64 {
        if j == 0 || j == n {
            2.0
        } else {
            1.0
        }
    };
    for i in 0..m {
        for j in 0..m {
            if i != j {
                let sign = if (i + j) % 2 == 0 { 1.0 } else { -1.0 };
                d[i][j] = c(i) / c(j) * sign / (nodes[i] - nodes[j]);
            }
        }
        let row_sum: f64 = (0..m).filter(|&j| j != i).map(|j| d[i][j]).sum();
        d[i][i] = -row_sum;
    }
    d
}
/// Apply the Chebyshev differentiation matrix D to a vector u.
pub fn apply_diff_matrix(d: &[Vec<f64>], u: &[f64]) -> Vec<f64> {
    let m = u.len();
    (0..m)
        .map(|i| (0..m).map(|j| d[i][j] * u[j]).sum())
        .collect()
}
/// Evaluate the n-th Legendre polynomial P_n(x) and its derivative P'_n(x)
/// via the Bonnet recurrence.
pub fn legendre_pn_dpn(n: usize, x: f64) -> (f64, f64) {
    if n == 0 {
        return (1.0, 0.0);
    }
    if n == 1 {
        return (x, 1.0);
    }
    let mut p_prev = 1.0;
    let mut p_curr = x;
    for k in 1..n {
        let p_next = ((2 * k + 1) as f64 * x * p_curr - k as f64 * p_prev) / (k + 1) as f64;
        p_prev = p_curr;
        p_curr = p_next;
    }
    let dp = (n as f64) * (p_prev - x * p_curr) / (1.0 - x * x);
    (p_curr, dp)
}
/// Compute N Gauss-Legendre nodes and weights on \[-1, 1\].
///
/// Uses Newton's method to find roots of P_N(x) then computes weights
/// w_i = 2 / ((1 - x_i²) [P'_N(x_i)]²).
pub fn gauss_legendre_nodes_weights(n: usize) -> (Vec<f64>, Vec<f64>) {
    let mut nodes = vec![0.0f64; n];
    let mut weights = vec![0.0f64; n];
    let half = n / 2;
    for i in 0..half {
        let mut x = -((2 * i + 1) as f64 * PI / (2 * n) as f64).cos();
        for _ in 0..100 {
            let (p, dp) = legendre_pn_dpn(n, x);
            let dx = -p / dp;
            x += dx;
            if dx.abs() < 1e-15 {
                break;
            }
        }
        let (_, dp) = legendre_pn_dpn(n, x);
        let w = 2.0 / ((1.0 - x * x) * dp * dp);
        nodes[i] = -x;
        nodes[n - 1 - i] = x;
        weights[i] = w;
        weights[n - 1 - i] = w;
    }
    if n % 2 == 1 {
        let (_, dp) = legendre_pn_dpn(n, 0.0);
        nodes[half] = 0.0;
        weights[half] = 2.0 / (dp * dp);
    }
    (nodes, weights)
}
/// Integrate f on \[-1, 1\] using N-point Gauss-Legendre quadrature.
pub fn gauss_legendre_integrate(f: &dyn Fn(f64) -> f64, n: usize) -> f64 {
    let (nodes, weights) = gauss_legendre_nodes_weights(n);
    nodes
        .iter()
        .zip(weights.iter())
        .map(|(&x, &w)| w * f(x))
        .sum()
}
/// Integrate f on \[a, b\] using N-point Gauss-Legendre quadrature
/// via the change of variables x = ((b-a)t + (b+a)) / 2.
pub fn gauss_legendre_integrate_ab(f: &dyn Fn(f64) -> f64, a: f64, b: f64, n: usize) -> f64 {
    let mid = (a + b) / 2.0;
    let half_len = (b - a) / 2.0;
    let g = |t: f64| f(mid + half_len * t);
    half_len * gauss_legendre_integrate(&g, n)
}
/// Compute Clenshaw-Curtis weights for N+1 Gauss-Lobatto nodes.
///
/// Uses the DCT-based formula for the weights.
pub fn clenshaw_curtis_weights(n: usize) -> Vec<f64> {
    let mut w = vec![0.0f64; n + 1];
    for j in 0..=n {
        let theta_j = (j as f64) * PI / (n as f64);
        let mut wj = 1.0;
        for k in 1..=(n / 2) {
            let factor = if 2 * k == n { 1.0 } else { 2.0 };
            wj -= factor / (4 * k * k - 1) as f64 * (2.0 * k as f64 * theta_j).cos();
        }
        wj *= 2.0 / (n as f64);
        if j == 0 || j == n {
            wj /= 2.0;
        }
        w[j] = wj;
    }
    w
}
/// Integrate f on \[-1, 1\] using N-point Clenshaw-Curtis quadrature.
pub fn clenshaw_curtis_integrate(f: &dyn Fn(f64) -> f64, n: usize) -> f64 {
    let nodes = chebyshev_gauss_lobatto_nodes(n);
    let weights = clenshaw_curtis_weights(n);
    nodes
        .iter()
        .zip(weights.iter())
        .map(|(&x, &w)| w * f(x))
        .sum()
}
/// Perform the in-place radix-2 DIT FFT on a power-of-2 length input.
///
/// After the call, `data\[k\]` holds the k-th DFT coefficient X_k = Σ x_n e^{-2πi kn/N}.
pub fn fft_inplace(data: &mut [Complex]) {
    let n = data.len();
    assert!(n.is_power_of_two(), "FFT length must be a power of two");
    let log2n = n.trailing_zeros() as usize;
    for i in 0..n {
        let j = bit_reverse(i, log2n);
        if i < j {
            data.swap(i, j);
        }
    }
    let mut len = 2;
    while len <= n {
        let ang = -2.0 * PI / len as f64;
        let w_len = Complex::exp_i(ang);
        for i in (0..n).step_by(len) {
            let mut w = Complex::one();
            for j in 0..(len / 2) {
                let u = data[i + j];
                let v = data[i + j + len / 2] * w;
                data[i + j] = u + v;
                data[i + j + len / 2] = u - v;
                w = w * w_len;
            }
        }
        len <<= 1;
    }
}
pub fn bit_reverse(mut x: usize, bits: usize) -> usize {
    let mut result = 0;
    for _ in 0..bits {
        result = (result << 1) | (x & 1);
        x >>= 1;
    }
    result
}
/// Forward DFT returning a new vector (pads to next power of two if needed).
pub fn fft(input: &[f64]) -> Vec<Complex> {
    let n = input.len().next_power_of_two();
    let mut data: Vec<Complex> = (0..n)
        .map(|i| {
            if i < input.len() {
                Complex::new(input[i], 0.0)
            } else {
                Complex::zero()
            }
        })
        .collect();
    fft_inplace(&mut data);
    data
}
/// Inverse FFT (IFFT): IDFT(X)\[n\] = (1/N) Σ X_k e^{2πi kn/N}.
pub fn ifft(spectrum: &mut [Complex]) {
    for x in spectrum.iter_mut() {
        *x = x.conj();
    }
    fft_inplace(spectrum);
    let n = spectrum.len() as f64;
    for x in spectrum.iter_mut() {
        *x = x.conj() / n;
    }
}
/// Dealias a spectrum with N modes by zero-padding to 3N/2 modes,
/// multiplying in physical space, then truncating back to N modes.
///
/// This eliminates aliasing errors in the quadratic nonlinearity u·u.
pub fn dealias_spectrum(spectrum: &[Complex], n_physical: usize) -> Vec<Complex> {
    let n_modes = spectrum.len();
    let n_padded = n_physical;
    let mut padded = vec![Complex::zero(); n_padded];
    let half = n_modes / 2;
    for k in 0..=half {
        if k < n_padded {
            padded[k] = spectrum[k];
        }
    }
    for k in (half + 1)..n_modes {
        let dest = n_padded - (n_modes - k);
        if dest < n_padded {
            padded[dest] = spectrum[k];
        }
    }
    padded
}
/// Evaluate φ_1(z) = (e^z - 1) / z, stable for small |z|.
///
/// This function appears in exponential Euler and Lawson-Euler methods.
pub fn phi1(z: f64) -> f64 {
    if z.abs() < 1e-8 {
        1.0 + z / 2.0 + z * z / 6.0 + z * z * z / 24.0
    } else {
        (z.exp() - 1.0) / z
    }
}
/// Exponential Euler method for u_t = L u + N(u).
///
/// One step: u_{n+1} = e^{Lh} u_n + h φ_1(Lh) N(u_n).
/// Implemented for scalar L (stiffness constant) and nonlinearity.
pub fn exponential_euler_step(l: f64, nonlin: &dyn Fn(f64) -> f64, u: f64, h: f64) -> f64 {
    let e_lh = (l * h).exp();
    let phi = phi1(l * h);
    e_lh * u + h * phi * nonlin(u)
}
/// Run the exponential Euler integrator for `n_steps` steps.
pub fn exponential_euler(
    l: f64,
    nonlin: &dyn Fn(f64) -> f64,
    u0: f64,
    h: f64,
    n_steps: usize,
) -> Vec<f64> {
    let mut traj = Vec::with_capacity(n_steps + 1);
    let mut u = u0;
    traj.push(u);
    for _ in 0..n_steps {
        u = exponential_euler_step(l, nonlin, u, h);
        traj.push(u);
    }
    traj
}
/// Solve the 1-D periodic heat equation u_t = ν u_{xx} on \[0, 2π\]
/// using Fourier pseudospectral method with implicit time integration.
///
/// Returns the solution at time T given initial data `u0` sampled at N
/// equi-spaced points.  Uses the exact Fourier solution in mode space.
pub fn heat_equation_fourier(u0: &[f64], nu: f64, t_end: f64, n_steps: usize) -> Vec<f64> {
    let n = u0.len();
    assert!(n.is_power_of_two(), "N must be a power of two");
    let mut u_hat = fft(u0);
    let dt = t_end / n_steps as f64;
    for step in 0..n_steps {
        let _ = step;
        for (j, coeff) in u_hat.iter_mut().enumerate() {
            let k = if j <= n / 2 {
                j as f64
            } else {
                j as f64 - n as f64
            };
            let decay = (-nu * k * k * dt).exp();
            *coeff = *coeff * decay;
        }
    }
    ifft(&mut u_hat);
    u_hat.iter().map(|c| c.re).collect()
}
/// Estimate the spectral convergence rate by comparing solutions at N and 2N modes.
///
/// Returns the log₂ of the ratio of L∞ errors.
pub fn estimate_spectral_convergence_rate(
    f: &dyn Fn(f64) -> f64,
    n_coarse: usize,
    n_fine: usize,
) -> f64 {
    let x_fine: Vec<f64> = (0..n_fine)
        .map(|j| 2.0 * PI * j as f64 / n_fine as f64)
        .collect();
    let u_coarse = fft(&(0..n_coarse)
        .map(|j| f(2.0 * PI * j as f64 / n_coarse as f64))
        .collect::<Vec<_>>());
    let e_coarse: f64 = u_coarse[n_coarse / 2..]
        .iter()
        .map(|c| c.abs())
        .sum::<f64>()
        / u_coarse.len() as f64;
    let u_fine = fft(&x_fine.iter().map(|&x| f(x)).collect::<Vec<_>>());
    let e_fine: f64 =
        u_fine[n_fine / 2..].iter().map(|c| c.abs()).sum::<f64>() / u_fine.len() as f64;
    if e_fine > 0.0 && e_coarse > 0.0 {
        (e_coarse / e_fine).log2()
    } else {
        f64::INFINITY
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_chebyshev_polynomial_recurrence() {
        let x = 0.5_f64;
        assert!((chebyshev_t(0, x) - 1.0).abs() < 1e-14);
        assert!((chebyshev_t(1, x) - 0.5).abs() < 1e-14);
        assert!((chebyshev_t(2, x) - (2.0 * 0.25 - 1.0)).abs() < 1e-14);
        assert!((chebyshev_t(3, x) - (4.0 * 0.125 - 1.5)).abs() < 1e-14);
    }
    #[test]
    fn test_chebyshev_trig_agrees_with_recurrence() {
        for n in 0..8 {
            for j in 0..20 {
                let x = -1.0 + 2.0 * j as f64 / 19.0;
                let t_rec = chebyshev_t(n, x);
                let t_trig = chebyshev_t_trig(n, x);
                assert!(
                    (t_rec - t_trig).abs() < 1e-12,
                    "n={n}, x={x}: recurrence={t_rec}, trig={t_trig}"
                );
            }
        }
    }
    #[test]
    fn test_gauss_legendre_quadrature_polynomials() {
        let f = |x: f64| x.powi(8);
        let result = gauss_legendre_integrate(&f, 5);
        let exact = 2.0 / 9.0;
        assert!(
            (result - exact).abs() < 1e-13,
            "5-pt GL, ∫x^8: got {result}, expected {exact}"
        );
    }
    #[test]
    fn test_gauss_legendre_integrate_ab() {
        let f = |x: f64| x.sin();
        let result = gauss_legendre_integrate_ab(&f, 0.0, PI, 20);
        assert!(
            (result - 2.0).abs() < 1e-12,
            "GL ∫sin: got {result}, expected 2.0"
        );
    }
    #[test]
    fn test_fft_roundtrip() {
        let signal: Vec<f64> = (0..8).map(|j| (2.0 * PI * j as f64 / 8.0).sin()).collect();
        let mut spectrum = fft(&signal);
        ifft(&mut spectrum);
        for (i, (&orig, rec)) in signal.iter().zip(spectrum.iter()).enumerate() {
            assert!(
                (orig - rec.re).abs() < 1e-12,
                "IFFT(FFT): index {i}, original={orig}, recovered={}",
                rec.re
            );
        }
    }
    #[test]
    fn test_heat_equation_decay() {
        let n = 32;
        let nu = 0.1;
        let t_end = 0.5;
        let n_steps = 100;
        let u0: Vec<f64> = (0..n)
            .map(|j| (2.0 * PI * j as f64 / n as f64).sin())
            .collect();
        let u_final = heat_equation_fourier(&u0, nu, t_end, n_steps);
        let expected_amp = (-nu * t_end).exp();
        let max_val = u_final.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        assert!(
            (max_val - expected_amp).abs() < 1e-6,
            "Heat eq: amplitude {max_val}, expected {expected_amp}"
        );
    }
    #[test]
    fn test_exponential_euler_linear() {
        let l = -1.0;
        let nonlin = |_u: f64| 0.0;
        let h = 0.01;
        let n_steps = 100;
        let traj = exponential_euler(l, &nonlin, 1.0, h, n_steps);
        let u_final = traj[n_steps];
        let exact = (-1.0_f64).exp();
        assert!(
            (u_final - exact).abs() < 1e-12,
            "Exp Euler linear: got {u_final}, expected {exact}"
        );
    }
    #[test]
    fn test_chebyshev_barycentric_interpolation() {
        let n = 4;
        let nodes = chebyshev_gauss_lobatto_nodes(n);
        let values: Vec<f64> = nodes.iter().map(|&x| x.powi(4)).collect();
        for j in 0..10 {
            let x = -1.0 + 2.0 * j as f64 / 9.0;
            let interp = chebyshev_barycentric_interp(&nodes, &values, x);
            let exact = x.powi(4);
            assert!(
                (interp - exact).abs() < 1e-11,
                "Chebyshev interp x^4 at x={x}: got {interp}, exact {exact}"
            );
        }
    }
    #[test]
    fn test_build_spectral_methods_env() {
        let mut env = Environment::new();
        build_spectral_methods_env(&mut env);
        assert!(env.get(&Name::str("FourierCoefficient")).is_some());
        assert!(env.get(&Name::str("ChebyshevPolynomial")).is_some());
        assert!(env.get(&Name::str("GaussLegendreNodes")).is_some());
        assert!(env.get(&Name::str("DFTInversion")).is_some());
        assert!(env.get(&Name::str("ExponentialIntegrator")).is_some());
    }
}
/// Solve the n×n linear system A x = b using Gaussian elimination with partial pivoting.
pub(super) fn solve_linear(a: &[Vec<f64>], b: &[f64]) -> Vec<f64> {
    let n = a.len();
    if n == 0 {
        return vec![];
    }
    let mut mat: Vec<Vec<f64>> = a.iter().map(|r| r.clone()).collect();
    let mut rhs: Vec<f64> = b.to_vec();
    for col in 0..n {
        let mut pivot = col;
        for row in col + 1..n {
            if mat[row][col].abs() > mat[pivot][col].abs() {
                pivot = row;
            }
        }
        mat.swap(col, pivot);
        rhs.swap(col, pivot);
        let diag = mat[col][col];
        if diag.abs() < 1e-15 {
            continue;
        }
        for row in col + 1..n {
            let factor = mat[row][col] / diag;
            for c in col..n {
                mat[row][c] -= factor * mat[col][c];
            }
            rhs[row] -= factor * rhs[col];
        }
    }
    let mut x = vec![0.0_f64; n];
    for i in (0..n).rev() {
        let mut s = rhs[i];
        for j in i + 1..n {
            s -= mat[i][j] * x[j];
        }
        let diag = mat[i][i];
        x[i] = if diag.abs() < 1e-15 { 0.0 } else { s / diag };
    }
    x
}
/// Build a kernel `Environment` with spectral-methods axioms.
pub fn build_env() -> Environment {
    let mut env = Environment::new();
    build_spectral_methods_env(&mut env);
    env
}
pub fn spec2_ext_cst(s: &str) -> Expr {
    Expr::Const(Name::str(s), vec![])
}
pub fn spec2_ext_prop() -> Expr {
    Expr::Sort(Level::zero())
}
pub fn spec2_ext_type0() -> Expr {
    Expr::Sort(Level::succ(Level::zero()))
}
pub fn spec2_ext_arrow(a: Expr, b: Expr) -> Expr {
    Expr::Pi(
        BinderInfo::Default,
        Name::str("_"),
        Box::new(a),
        Box::new(b),
    )
}
pub fn spec2_ext_nat_ty() -> Expr {
    spec2_ext_cst("Nat")
}
pub fn spec2_ext_real_ty() -> Expr {
    spec2_ext_cst("Real")
}
pub fn spec2_ext_list_ty(a: Expr) -> Expr {
    Expr::App(Box::new(spec2_ext_cst("List")), Box::new(a))
}
pub fn spec2_ext_impl_pi(name: &str, dom: Expr, body: Expr) -> Expr {
    Expr::Pi(
        BinderInfo::Implicit,
        Name::str(name),
        Box::new(dom),
        Box::new(body),
    )
}
pub fn spec2_ext_app(f: Expr, a: Expr) -> Expr {
    Expr::App(Box::new(f), Box::new(a))
}
pub fn spec2_ext_bvar(n: u32) -> Expr {
    Expr::BVar(n)
}
/// `RadauQuadrature : Nat → Prod (List Real) (List Real)`
///
/// Radau quadrature rule with N points including one fixed endpoint.
pub fn radau_quadrature_ty() -> Expr {
    spec2_ext_arrow(
        spec2_ext_nat_ty(),
        spec2_ext_app(
            spec2_ext_app(
                spec2_ext_cst("Prod"),
                spec2_ext_list_ty(spec2_ext_real_ty()),
            ),
            spec2_ext_list_ty(spec2_ext_real_ty()),
        ),
    )
}
/// `GaussLobattoExactness : ∀ (n : Nat) (p : Polynomial Real), DegBound p → Prop`
///
/// Gauss-Lobatto with N+1 points integrates polynomials of degree ≤ 2N-1 exactly.
pub fn gauss_lobatto_exactness_ty() -> Expr {
    spec2_ext_arrow(
        spec2_ext_nat_ty(),
        spec2_ext_arrow(
            spec2_ext_app(spec2_ext_cst("Polynomial"), spec2_ext_real_ty()),
            spec2_ext_arrow(
                spec2_ext_app(spec2_ext_cst("DegBound"), spec2_ext_bvar(1)),
                spec2_ext_prop(),
            ),
        ),
    )
}
/// `SpectralDiffMatrixAxiom : Nat → Matrix Real`
///
/// The spectral differentiation matrix D_N for Chebyshev collocation.
pub fn spectral_diff_matrix_ty() -> Expr {
    spec2_ext_arrow(
        spec2_ext_nat_ty(),
        spec2_ext_app(spec2_ext_cst("Matrix"), spec2_ext_real_ty()),
    )
}
/// `SpectralIntegMatrix : Nat → Matrix Real`
///
/// Spectral integration matrix: the pseudo-inverse of the differentiation matrix.
pub fn spectral_integ_matrix_ty() -> Expr {
    spec2_ext_arrow(
        spec2_ext_nat_ty(),
        spec2_ext_app(spec2_ext_cst("Matrix"), spec2_ext_real_ty()),
    )
}
/// `ExponentialConvergence : ∀ (f : Real → Real) (n : Nat), Prop`
///
/// Exponential convergence: for smooth f, spectral error decays faster than any polynomial.
pub fn exponential_convergence_ty() -> Expr {
    spec2_ext_impl_pi(
        "f",
        spec2_ext_arrow(spec2_ext_real_ty(), spec2_ext_real_ty()),
        spec2_ext_arrow(spec2_ext_nat_ty(), spec2_ext_prop()),
    )
}
/// `HpSpectralElement : ∀ (p h : Nat), Prop`
///
/// hp-adaptive spectral element method.
pub fn hp_spectral_element_ty() -> Expr {
    spec2_ext_arrow(
        spec2_ext_nat_ty(),
        spec2_ext_arrow(spec2_ext_nat_ty(), spec2_ext_prop()),
    )
}
/// `MappedJacobiPolynomial : Real → Real → Nat → Real → Real → Real → Real`
///
/// Jacobi polynomial P_n^{(α,β)} mapped to interval \[a, b\].
pub fn mapped_jacobi_polynomial_ty() -> Expr {
    spec2_ext_arrow(
        spec2_ext_real_ty(),
        spec2_ext_arrow(
            spec2_ext_real_ty(),
            spec2_ext_arrow(
                spec2_ext_nat_ty(),
                spec2_ext_arrow(
                    spec2_ext_real_ty(),
                    spec2_ext_arrow(
                        spec2_ext_real_ty(),
                        spec2_ext_arrow(spec2_ext_real_ty(), spec2_ext_real_ty()),
                    ),
                ),
            ),
        ),
    )
}
/// `BarycentricInterpolFormula : Nat → List Real → List Real → Real → Real`
///
/// Barycentric interpolation formula for polynomial interpolation.
pub fn barycentric_interp_formula_ty() -> Expr {
    spec2_ext_arrow(
        spec2_ext_nat_ty(),
        spec2_ext_arrow(
            spec2_ext_list_ty(spec2_ext_real_ty()),
            spec2_ext_arrow(
                spec2_ext_list_ty(spec2_ext_real_ty()),
                spec2_ext_arrow(spec2_ext_real_ty(), spec2_ext_real_ty()),
            ),
        ),
    )
}
/// `ClenshawCurtisStability : ∀ (n : Nat), Prop`
///
/// Clenshaw-Curtis Lebesgue constant is O(log N).
pub fn clenshaw_curtis_stability_ty() -> Expr {
    spec2_ext_arrow(spec2_ext_nat_ty(), spec2_ext_prop())
}
/// `SpectralIVP : ∀ (f : Real → Real → Real) (u0 T : Real), Real → Real`
///
/// Spectral method for initial value problems.
pub fn spectral_ivp_ty() -> Expr {
    spec2_ext_impl_pi(
        "f",
        spec2_ext_arrow(
            spec2_ext_real_ty(),
            spec2_ext_arrow(spec2_ext_real_ty(), spec2_ext_real_ty()),
        ),
        spec2_ext_arrow(
            spec2_ext_real_ty(),
            spec2_ext_arrow(
                spec2_ext_real_ty(),
                spec2_ext_arrow(spec2_ext_real_ty(), spec2_ext_real_ty()),
            ),
        ),
    )
}
/// `SpectralBVP : ∀ (L : DiffOp) (f : Real → Real), Real → Real`
///
/// Spectral method for boundary value problems.
pub fn spectral_bvp_ty() -> Expr {
    spec2_ext_impl_pi(
        "L",
        spec2_ext_arrow(
            spec2_ext_arrow(spec2_ext_real_ty(), spec2_ext_real_ty()),
            spec2_ext_arrow(spec2_ext_real_ty(), spec2_ext_real_ty()),
        ),
        spec2_ext_arrow(
            spec2_ext_arrow(spec2_ext_real_ty(), spec2_ext_real_ty()),
            spec2_ext_arrow(spec2_ext_real_ty(), spec2_ext_real_ty()),
        ),
    )
}
/// `EigenvalueSpectral : ∀ (L : DiffOp) (n : Nat), Real`
///
/// Eigenvalue approximation via spectral methods.
pub fn eigenvalue_spectral_ty() -> Expr {
    spec2_ext_impl_pi(
        "L",
        spec2_ext_arrow(
            spec2_ext_arrow(spec2_ext_real_ty(), spec2_ext_real_ty()),
            spec2_ext_arrow(spec2_ext_real_ty(), spec2_ext_real_ty()),
        ),
        spec2_ext_arrow(spec2_ext_nat_ty(), spec2_ext_real_ty()),
    )
}
/// `FastCosineTransform : Nat → List Real → List Real`
///
/// Fast cosine transform (DCT): O(N log N).
pub fn fast_cosine_transform_ty() -> Expr {
    spec2_ext_arrow(
        spec2_ext_nat_ty(),
        spec2_ext_arrow(
            spec2_ext_list_ty(spec2_ext_real_ty()),
            spec2_ext_list_ty(spec2_ext_real_ty()),
        ),
    )
}
/// `FastSineTransform : Nat → List Real → List Real`
///
/// Fast sine transform (DST): O(N log N).
pub fn fast_sine_transform_ty() -> Expr {
    spec2_ext_arrow(
        spec2_ext_nat_ty(),
        spec2_ext_arrow(
            spec2_ext_list_ty(spec2_ext_real_ty()),
            spec2_ext_list_ty(spec2_ext_real_ty()),
        ),
    )
}
/// `PadeApproximant : Nat → Nat → FormalSeries Real → RationalFunction Real`
///
/// Padé approximant [m/n]: rational function matching power series to order m+n.
pub fn pade_approximant_ty() -> Expr {
    spec2_ext_arrow(
        spec2_ext_nat_ty(),
        spec2_ext_arrow(
            spec2_ext_nat_ty(),
            spec2_ext_arrow(
                spec2_ext_app(spec2_ext_cst("FormalSeries"), spec2_ext_real_ty()),
                spec2_ext_app(spec2_ext_cst("RationalFunction"), spec2_ext_real_ty()),
            ),
        ),
    )
}
/// `SincFunction : Real → Real`
///
/// The sinc function sinc(x) = sin(πx)/(πx).
pub fn sinc_function_ty() -> Expr {
    spec2_ext_arrow(spec2_ext_real_ty(), spec2_ext_real_ty())
}
/// `SincCollocation : Nat → Real → Matrix Real`
///
/// Sinc collocation matrix for DEs on the whole real line.
pub fn sinc_collocation_ty() -> Expr {
    spec2_ext_arrow(
        spec2_ext_nat_ty(),
        spec2_ext_arrow(
            spec2_ext_real_ty(),
            spec2_ext_app(spec2_ext_cst("Matrix"), spec2_ext_real_ty()),
        ),
    )
}
/// `MultiDomainSpectral : Nat → Nat → Prop`
///
/// Multi-domain spectral method: subdomains with spectral expansion, coupled at interfaces.
pub fn multi_domain_spectral_ty() -> Expr {
    spec2_ext_arrow(
        spec2_ext_nat_ty(),
        spec2_ext_arrow(spec2_ext_nat_ty(), spec2_ext_prop()),
    )
}
/// `BoundaryPenaltyMethod : Real → Nat → Prop`
///
/// Boundary penalty (SAT) method: weak enforcement with penalty parameter τ.
pub fn boundary_penalty_method_ty() -> Expr {
    spec2_ext_arrow(
        spec2_ext_real_ty(),
        spec2_ext_arrow(spec2_ext_nat_ty(), spec2_ext_prop()),
    )
}
/// `SpectralRadiusCFL : ∀ (D : Matrix Real) (dt : Real), Prop`
///
/// CFL stability: dt ≤ C / ρ(D).
pub fn spectral_radius_cfl_ty() -> Expr {
    spec2_ext_impl_pi(
        "D",
        spec2_ext_app(spec2_ext_cst("Matrix"), spec2_ext_real_ty()),
        spec2_ext_arrow(spec2_ext_real_ty(), spec2_ext_prop()),
    )
}
/// `LebesgueConstantChebyshev : Nat → Real`
///
/// Lebesgue constant for Chebyshev nodes: Λ_n = O(log n).
pub fn lebesgue_constant_chebyshev_ty() -> Expr {
    spec2_ext_arrow(spec2_ext_nat_ty(), spec2_ext_real_ty())
}
/// `SpectralDeferredCorrection : Nat → Nat → Prop`
///
/// Spectral deferred correction (SDC): k sweeps to achieve high-order accuracy.
pub fn spectral_deferred_correction_ty() -> Expr {
    spec2_ext_arrow(
        spec2_ext_nat_ty(),
        spec2_ext_arrow(spec2_ext_nat_ty(), spec2_ext_prop()),
    )
}
/// `GaussRadauNodes : Nat → Bool → Prod (List Real) (List Real)`
///
/// Gauss-Radau nodes including one fixed endpoint.
pub fn gauss_radau_nodes_ty() -> Expr {
    spec2_ext_arrow(
        spec2_ext_nat_ty(),
        spec2_ext_arrow(
            spec2_ext_cst("Bool"),
            spec2_ext_app(
                spec2_ext_app(
                    spec2_ext_cst("Prod"),
                    spec2_ext_list_ty(spec2_ext_real_ty()),
                ),
                spec2_ext_list_ty(spec2_ext_real_ty()),
            ),
        ),
    )
}
/// `OrthogonalPolynomialFamily : ∀ (w : Real → Real) (n : Nat), Real → Real`
///
/// Orthogonal polynomial family {p_n} w.r.t. weight w.
pub fn orthogonal_polynomial_family_ty() -> Expr {
    spec2_ext_impl_pi(
        "w",
        spec2_ext_arrow(spec2_ext_real_ty(), spec2_ext_real_ty()),
        spec2_ext_arrow(
            spec2_ext_nat_ty(),
            spec2_ext_arrow(spec2_ext_real_ty(), spec2_ext_real_ty()),
        ),
    )
}
/// `ChristoffelDarbouxFormula : ∀ (w : Real → Real) (n : Nat) (x y : Real), Real`
///
/// Christoffel-Darboux formula for sums of orthogonal polynomials.
pub fn christoffel_darboux_formula_ty() -> Expr {
    spec2_ext_impl_pi(
        "w",
        spec2_ext_arrow(spec2_ext_real_ty(), spec2_ext_real_ty()),
        spec2_ext_arrow(
            spec2_ext_nat_ty(),
            spec2_ext_arrow(
                spec2_ext_real_ty(),
                spec2_ext_arrow(spec2_ext_real_ty(), spec2_ext_real_ty()),
            ),
        ),
    )
}
/// `FejersRule : Nat → Prod (List Real) (List Real)`
///
/// Fejér's rule: positive weights using Chebyshev points of the first kind.
pub fn fejers_rule_ty() -> Expr {
    spec2_ext_arrow(
        spec2_ext_nat_ty(),
        spec2_ext_app(
            spec2_ext_app(
                spec2_ext_cst("Prod"),
                spec2_ext_list_ty(spec2_ext_real_ty()),
            ),
            spec2_ext_list_ty(spec2_ext_real_ty()),
        ),
    )
}
/// `SpectralOperatorSplit : ∀ (L1 L2 : DiffOp) (dt : Real), Prop`
///
/// Operator splitting for spectral methods (Strang/Lie).
pub fn spectral_operator_split_ty() -> Expr {
    spec2_ext_impl_pi(
        "L1",
        spec2_ext_arrow(
            spec2_ext_arrow(spec2_ext_real_ty(), spec2_ext_real_ty()),
            spec2_ext_arrow(spec2_ext_real_ty(), spec2_ext_real_ty()),
        ),
        spec2_ext_impl_pi(
            "L2",
            spec2_ext_arrow(
                spec2_ext_arrow(spec2_ext_real_ty(), spec2_ext_real_ty()),
                spec2_ext_arrow(spec2_ext_real_ty(), spec2_ext_real_ty()),
            ),
            spec2_ext_arrow(spec2_ext_real_ty(), spec2_ext_prop()),
        ),
    )
}
/// `GalerkinSpectral : Nat → BasisType Real → Prop`
///
/// Galerkin spectral method: ⟨Lu_N - f, v⟩ = 0 for all v ∈ V_N.
pub fn galerkin_spectral_ty() -> Expr {
    spec2_ext_arrow(
        spec2_ext_nat_ty(),
        spec2_ext_arrow(
            spec2_ext_app(spec2_ext_cst("BasisType"), spec2_ext_real_ty()),
            spec2_ext_prop(),
        ),
    )
}
/// `TauMethod : Nat → ∀ (L : DiffOp), Prop`
///
/// Tau method (Lanczos): modify Galerkin equations to exactly impose BCs.
pub fn tau_method_ty() -> Expr {
    spec2_ext_arrow(
        spec2_ext_nat_ty(),
        spec2_ext_impl_pi(
            "L",
            spec2_ext_arrow(
                spec2_ext_arrow(spec2_ext_real_ty(), spec2_ext_real_ty()),
                spec2_ext_arrow(spec2_ext_real_ty(), spec2_ext_real_ty()),
            ),
            spec2_ext_prop(),
        ),
    )
}
/// `WaveNumberResolution : Nat → Real → Prop`
///
/// Wave number resolution: N modes resolve wavenumbers up to N/2.
pub fn wave_number_resolution_ty() -> Expr {
    spec2_ext_arrow(
        spec2_ext_nat_ty(),
        spec2_ext_arrow(spec2_ext_real_ty(), spec2_ext_prop()),
    )
}
/// `SpectralFilterStabilization : ∀ (sigma : Real → Real) (n : Nat), Prop`
///
/// Spectral filtering for stabilization: apply low-pass filter σ to coefficients.
pub fn spectral_filter_stabilization_ty() -> Expr {
    spec2_ext_impl_pi(
        "sigma",
        spec2_ext_arrow(spec2_ext_real_ty(), spec2_ext_real_ty()),
        spec2_ext_arrow(spec2_ext_nat_ty(), spec2_ext_prop()),
    )
}
/// `SincInterpolant : Real → Real`
///
/// Sinc interpolant: sum of shifted sinc functions for whittaker interpolation.
pub fn sinc_interpolant_ty() -> Expr {
    spec2_ext_arrow(spec2_ext_real_ty(), spec2_ext_real_ty())
}
/// `SpectralRadiusEstimate : ∀ (D : Matrix Real) (dt : Real), Prop`
///
/// Spectral radius estimate for stability analysis.
pub fn spectral_radius_estimate_ty() -> Expr {
    spec2_ext_impl_pi(
        "D",
        spec2_ext_app(spec2_ext_cst("Matrix"), spec2_ext_real_ty()),
        spec2_ext_arrow(spec2_ext_real_ty(), spec2_ext_prop()),
    )
}
/// `SemiDiscretePDE : Nat → BasisType Real → Prop`
///
/// Semi-discrete PDE system: spatial discretization with continuous time.
pub fn semi_discrete_pde_ty() -> Expr {
    spec2_ext_arrow(
        spec2_ext_nat_ty(),
        spec2_ext_arrow(
            spec2_ext_app(spec2_ext_cst("BasisType"), spec2_ext_real_ty()),
            spec2_ext_prop(),
        ),
    )
}
/// `RadauQuadratureWeights : Nat → List Real`
///
/// Radau quadrature weights for a given number of points.
pub fn radau_quadrature_weights_ty() -> Expr {
    spec2_ext_arrow(spec2_ext_nat_ty(), spec2_ext_list_ty(spec2_ext_real_ty()))
}
/// Register all extended spectral-methods axioms.
pub fn register_spectral_methods_ext(env: &mut Environment) {
    let axioms: &[(&str, Expr)] = &[
        ("RadauQuadrature", radau_quadrature_ty()),
        ("GaussLobattoExactness", gauss_lobatto_exactness_ty()),
        ("SpectralDiffMatrixAxiom", spectral_diff_matrix_ty()),
        ("SpectralIntegMatrix", spectral_integ_matrix_ty()),
        ("ExponentialConvergence", exponential_convergence_ty()),
        ("HpSpectralElement", hp_spectral_element_ty()),
        ("MappedJacobiPolynomial", mapped_jacobi_polynomial_ty()),
        (
            "BarycentricInterpolFormula",
            barycentric_interp_formula_ty(),
        ),
        ("ClenshawCurtisStability", clenshaw_curtis_stability_ty()),
        ("SpectralIVP", spectral_ivp_ty()),
        ("SpectralBVP", spectral_bvp_ty()),
        ("EigenvalueSpectral", eigenvalue_spectral_ty()),
        ("FastCosineTransform", fast_cosine_transform_ty()),
        ("FastSineTransform", fast_sine_transform_ty()),
        ("PadeApproximant", pade_approximant_ty()),
        ("SincFunction", sinc_function_ty()),
        ("SincCollocation", sinc_collocation_ty()),
        ("MultiDomainSpectral", multi_domain_spectral_ty()),
        ("BoundaryPenaltyMethod", boundary_penalty_method_ty()),
        ("SpectralRadiusCFL", spectral_radius_cfl_ty()),
        (
            "LebesgueConstantChebyshev",
            lebesgue_constant_chebyshev_ty(),
        ),
        (
            "SpectralDeferredCorrection",
            spectral_deferred_correction_ty(),
        ),
        ("GaussRadauNodes", gauss_radau_nodes_ty()),
        (
            "OrthogonalPolynomialFamily",
            orthogonal_polynomial_family_ty(),
        ),
        (
            "ChristoffelDarbouxFormula",
            christoffel_darboux_formula_ty(),
        ),
        ("FejersRule", fejers_rule_ty()),
        ("SpectralOperatorSplit", spectral_operator_split_ty()),
        ("GalerkinSpectral", galerkin_spectral_ty()),
        ("TauMethod", tau_method_ty()),
        ("WaveNumberResolution", wave_number_resolution_ty()),
        (
            "SpectralFilterStabilization",
            spectral_filter_stabilization_ty(),
        ),
        ("SincInterpolant", sinc_interpolant_ty()),
        ("SpectralRadiusEstimate", spectral_radius_estimate_ty()),
        ("SemiDiscretePDE", semi_discrete_pde_ty()),
        ("RadauQuadratureWeights", radau_quadrature_weights_ty()),
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
pub(super) fn spec2_legendre_p(n: usize, x: f64) -> f64 {
    if n == 0 {
        return 1.0;
    }
    if n == 1 {
        return x;
    }
    let mut p0 = 1.0;
    let mut p1 = x;
    for k in 1..n {
        let p2 = ((2 * k + 1) as f64 * x * p1 - k as f64 * p0) / (k + 1) as f64;
        p0 = p1;
        p1 = p2;
    }
    p1
}
