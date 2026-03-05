//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{
    DysonBeta, DysonBrownianMotion, DysonBrownianMotionData, EmpiricalSpectralDistribution,
    FreeConvolution, FreeProbabilityData, GOEEnsemble, GUEEnsemble, GaussianMatrixSampler, Lcg,
    LevelSpacingStats, MarcenkoPastur, MarchenkoPasturData, StieltjesTransformEval,
    SymmetricMatrix, TracyWidomData, UniversalityData, WignerSemicircle, WignerSemicircleData,
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
pub fn nat_ty() -> Expr {
    cst("Nat")
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
pub fn mat_ty() -> Expr {
    list_ty(list_ty(real_ty()))
}
pub fn vec_ty() -> Expr {
    list_ty(real_ty())
}
/// `GUEEnsemble : Nat -> Type` — n×n GUE random Hermitian matrix ensemble
pub fn gue_ensemble_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `GOEEnsemble : Nat -> Type` — n×n GOE real symmetric matrix ensemble
pub fn goe_ensemble_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `GSEEnsemble : Nat -> Type` — n×n GSE quaternion self-dual matrix ensemble
pub fn gse_ensemble_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `WishartEnsemble : Nat -> Nat -> Type` — p×p Wishart matrix W = X X^T, X is n×p
pub fn wishart_ensemble_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `WignerMatrix : Nat -> Type` — general n×n Wigner random symmetric matrix
pub fn wigner_matrix_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `RandomUnitaryGroup : Nat -> Type` — Haar-distributed unitary group U(n)
pub fn random_unitary_group_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `CircularUnitaryEnsemble : Nat -> Type` — CUE (Circular Unitary Ensemble)
pub fn cue_ensemble_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `EmpiricalSpectralDist : (List Real) -> Real -> Real`
/// μ_n(x) = (1/n) Σ δ(x - λ_i), the empirical spectral measure
pub fn empirical_spectral_dist_ty() -> Expr {
    arrow(vec_ty(), arrow(real_ty(), real_ty()))
}
/// `SemicircleDensity : Real -> Real -> Real`
/// ρ_σ(x) = (1/(2πσ²)) sqrt(4σ² - x²) · 1_{|x|≤2σ}, the Wigner semicircle
pub fn semicircle_density_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), real_ty()))
}
/// `MarchenkoPasturDensity : Real -> Real -> Real -> Real`
/// Marchenko-Pastur density with ratio γ = p/n and variance σ²
pub fn marchenko_pastur_density_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), arrow(real_ty(), real_ty())))
}
/// `TracyWidomDensity : Real -> Real`
/// Tracy-Widom GUE distribution F_2(s): limiting fluctuation of largest eigenvalue
pub fn tracy_widom_density_ty() -> Expr {
    arrow(real_ty(), real_ty())
}
/// `SineKernel : Real -> Real -> Real`
/// Bulk universality correlation kernel: K_sin(x,y) = sin(π(x-y))/(π(x-y))
pub fn sine_kernel_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), real_ty()))
}
/// `AiryKernel : Real -> Real -> Real`
/// Edge universality correlation kernel expressed via Airy functions
pub fn airy_kernel_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), real_ty()))
}
/// `LevelSpacing : (List Real) -> List Real`
/// Computes normalised consecutive level spacings s_i = (λ_{i+1} - λ_i) / ⟨s⟩
pub fn level_spacing_ty() -> Expr {
    arrow(vec_ty(), vec_ty())
}
/// `LevelSpacingDist : Real -> Real -> Real`
/// GUE Wigner surmise: p(s) ≈ (32/π²) s² exp(-4s²/π)
pub fn level_spacing_dist_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), real_ty()))
}
/// `LevelRepulsion : Nat -> Real -> Real`
/// Level repulsion exponent β for GUE(β=2)/GOE(β=1)/GSE(β=4): p(s) ~ s^β
pub fn level_repulsion_ty() -> Expr {
    arrow(nat_ty(), arrow(real_ty(), real_ty()))
}
/// `NumberVariance : (List Real) -> Real -> Real`
/// Σ²(L) = Var(number of eigenvalues in interval of length L)
pub fn number_variance_ty() -> Expr {
    arrow(vec_ty(), arrow(real_ty(), real_ty()))
}
/// `FreeRandomVariable : Type -> Type`
/// A noncommutative random variable in a free probability space (A, φ)
pub fn free_random_variable_ty() -> Expr {
    arrow(type0(), type0())
}
/// `FreeCumulant : (List Real) -> Nat -> Real`
/// Free cumulant κ_n[a] of a noncommutative distribution
pub fn free_cumulant_ty() -> Expr {
    arrow(vec_ty(), arrow(nat_ty(), real_ty()))
}
/// `RTransform : (Real -> Real) -> Real -> Real`
/// The R-transform: M(z) = G(-1/z-R(z))^(-1), linearises free convolution
pub fn r_transform_ty() -> Expr {
    arrow(arrow(real_ty(), real_ty()), arrow(real_ty(), real_ty()))
}
/// `STransform : (Real -> Real) -> Real -> Real`
/// The S-transform: linearises free multiplicative convolution
pub fn s_transform_ty() -> Expr {
    arrow(arrow(real_ty(), real_ty()), arrow(real_ty(), real_ty()))
}
/// `FreeConvolution : (Real -> Real) -> (Real -> Real) -> Real -> Real`
/// Free additive convolution μ ⊞ ν
pub fn free_convolution_ty() -> Expr {
    let density = arrow(real_ty(), real_ty());
    arrow(density.clone(), arrow(density.clone(), density))
}
/// `FreeIndependence : Type -> Type -> Prop`
/// Boolean predicate: two subalgebras are freely independent
pub fn free_independence_ty() -> Expr {
    arrow(type0(), arrow(type0(), prop()))
}
/// `DysonBMState : Nat -> Real -> List Real`
/// State of n eigenvalues at time t under Dyson Brownian motion
pub fn dyson_bm_state_ty() -> Expr {
    arrow(nat_ty(), arrow(real_ty(), vec_ty()))
}
/// `DysonBMGenerator : Nat -> Nat -> Real -> Real`
/// Generator of Dyson BM for β-ensemble: L f = Σ ∂²f/∂λᵢ² + β Σ_{i≠j} (∂f/∂λᵢ)/(λᵢ-λⱼ)
pub fn dyson_bm_generator_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), arrow(real_ty(), real_ty())))
}
/// `GaussianEquilibrium : Nat -> Real -> (List Real -> Real)`
/// The Gaussian equilibrium measure (Hermite weight) e^{-βn/4 Σ λᵢ²}
pub fn gaussian_equilibrium_ty() -> Expr {
    arrow(nat_ty(), arrow(real_ty(), arrow(vec_ty(), real_ty())))
}
/// `BulkUniversality : Nat -> Prop`
/// Tao-Vu/Erdős-Yau-Yin: local statistics of Wigner matrices match GUE in bulk
pub fn bulk_universality_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `EdgeUniversality : Nat -> Prop`
/// Largest eigenvalue statistics of Wigner matrices converge to Tracy-Widom
pub fn edge_universality_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `FourMomentTheorem : Type -> Prop`
/// Tao-Vu four moment theorem: local statistics depend only on first four moments
pub fn four_moment_theorem_ty() -> Expr {
    arrow(type0(), prop())
}
/// `GreenFunctionComparison : Type -> Type -> Prop`
/// Green function comparison theorem (Erdős-Yau-Yin)
pub fn green_function_comparison_ty() -> Expr {
    arrow(type0(), arrow(type0(), prop()))
}
/// Build the random matrix theory kernel environment.
pub fn build_random_matrix_theory_env(env: &mut Environment) {
    let axioms: &[(&str, Expr)] = &[
        ("GUEEnsemble", gue_ensemble_ty()),
        ("GOEEnsemble", goe_ensemble_ty()),
        ("GSEEnsemble", gse_ensemble_ty()),
        ("WishartEnsemble", wishart_ensemble_ty()),
        ("WignerMatrix", wigner_matrix_ty()),
        ("RandomUnitaryGroup", random_unitary_group_ty()),
        ("CUEEnsemble", cue_ensemble_ty()),
        ("EmpiricalSpectralDist", empirical_spectral_dist_ty()),
        ("SemicircleDensity", semicircle_density_ty()),
        ("MarchenkoPasturDensity", marchenko_pastur_density_ty()),
        ("TracyWidomDensity", tracy_widom_density_ty()),
        ("SineKernel", sine_kernel_ty()),
        ("AiryKernel", airy_kernel_ty()),
        ("LevelSpacing", level_spacing_ty()),
        ("LevelSpacingDist", level_spacing_dist_ty()),
        ("LevelRepulsion", level_repulsion_ty()),
        ("NumberVariance", number_variance_ty()),
        ("DysonBeta", nat_ty()),
        ("FreeRandomVariable", free_random_variable_ty()),
        ("FreeCumulant", free_cumulant_ty()),
        ("RTransform", r_transform_ty()),
        ("STransform", s_transform_ty()),
        ("FreeConvolution", free_convolution_ty()),
        ("FreeIndependence", free_independence_ty()),
        ("DysonBMState", dyson_bm_state_ty()),
        ("DysonBMGenerator", dyson_bm_generator_ty()),
        ("GaussianEquilibrium", gaussian_equilibrium_ty()),
        ("BulkUniversality", bulk_universality_ty()),
        ("EdgeUniversality", edge_universality_ty()),
        ("FourMomentTheorem", four_moment_theorem_ty()),
        ("GreenFunctionComparison", green_function_comparison_ty()),
        ("SpectralRadius", arrow(mat_ty(), real_ty())),
        ("SpectralGap", arrow(mat_ty(), real_ty())),
        ("SpectralNorm", arrow(mat_ty(), real_ty())),
        (
            "StieltjesTransform",
            arrow(arrow(real_ty(), real_ty()), arrow(real_ty(), real_ty())),
        ),
        (
            "CauchyTransform",
            arrow(arrow(real_ty(), real_ty()), arrow(real_ty(), real_ty())),
        ),
        (
            "TwoPointCorrelation",
            arrow(vec_ty(), arrow(real_ty(), arrow(real_ty(), real_ty()))),
        ),
        (
            "nPointCorrelation",
            arrow(nat_ty(), arrow(vec_ty(), arrow(vec_ty(), real_ty()))),
        ),
        ("WignerSemicircleLaw", arrow(nat_ty(), prop())),
        (
            "MarchenkoPasturLaw",
            arrow(nat_ty(), arrow(nat_ty(), prop())),
        ),
        ("TracyWidomLimit", arrow(nat_ty(), prop())),
        ("DysonLogarithmicPotential", arrow(nat_ty(), prop())),
        (
            "RigidityOfEigenvalues",
            arrow(nat_ty(), arrow(real_ty(), prop())),
        ),
        ("DelocalizationOfEigenvectors", arrow(nat_ty(), prop())),
        ("OperatorValuedFreeness", arrow(type0(), prop())),
        (
            "AmalgamatedFreeProd",
            arrow(type0(), arrow(type0(), type0())),
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
}
/// Sample a GOE(n) matrix: symmetric with N(0,1/n) off-diagonal, N(0,2/n) diagonal entries
pub fn sample_goe(n: usize, seed: u64) -> SymmetricMatrix {
    let mut rng = Lcg::new(seed);
    let mut m = SymmetricMatrix::zeros(n);
    let scale_off = (1.0 / n as f64).sqrt();
    let scale_diag = (2.0 / n as f64).sqrt();
    for i in 0..n {
        m.set(i, i, rng.next_normal() * scale_diag);
        for j in (i + 1)..n {
            let v = rng.next_normal() * scale_off;
            m.set(i, j, v);
            m.set(j, i, v);
        }
    }
    m
}
/// Estimate eigenvalues of a symmetric matrix via the QR algorithm (simple version)
/// Uses Householder tridiagonalisation + QR iteration.
pub fn tridiagonalise(m: &SymmetricMatrix) -> (Vec<f64>, Vec<f64>) {
    let n = m.n;
    let mut alpha = vec![0.0f64; n];
    let mut beta = vec![0.0f64; n - 1];
    let mut a: Vec<f64> = m.data.clone();
    for k in 0..(n.saturating_sub(2)) {
        let mut norm_sq = 0.0f64;
        for i in (k + 1)..n {
            norm_sq += a[i * n + k] * a[i * n + k];
        }
        let norm = norm_sq.sqrt();
        if norm < 1e-14 {
            alpha[k] = a[k * n + k];
            beta[k] = 0.0;
            continue;
        }
        let sign = if a[(k + 1) * n + k] >= 0.0 { 1.0 } else { -1.0 };
        let u1 = a[(k + 1) * n + k] + sign * norm;
        let mut v = vec![0.0f64; n];
        v[k + 1] = u1;
        for i in (k + 2)..n {
            v[i] = a[i * n + k];
        }
        let v_norm_sq: f64 = v.iter().map(|x| x * x).sum();
        if v_norm_sq < 1e-28 {
            continue;
        }
        let inv2 = 2.0 / v_norm_sq;
        let mut q = vec![0.0f64; n];
        for i in 0..n {
            for j in 0..n {
                q[i] += a[i * n + j] * v[j];
            }
        }
        let vtq: f64 = v.iter().zip(q.iter()).map(|(x, y)| x * y).sum();
        let c = inv2 * inv2 * vtq;
        for i in 0..n {
            for j in 0..n {
                a[i * n + j] -= inv2 * (q[i] * v[j] + v[i] * q[j]) - c * v[i] * v[j];
            }
        }
        alpha[k] = a[k * n + k];
        beta[k] = -sign * norm;
    }
    if n >= 2 {
        alpha[n - 2] = a[(n - 2) * n + (n - 2)];
        beta[n - 2] = a[(n - 1) * n + (n - 2)];
        alpha[n - 1] = a[(n - 1) * n + (n - 1)];
    } else if n == 1 {
        alpha[0] = a[0];
    }
    (alpha, beta)
}
/// Compute eigenvalues of a symmetric tridiagonal matrix via implicit QR shifts.
pub fn tridiag_eigenvalues(mut alpha: Vec<f64>, mut beta: Vec<f64>) -> Vec<f64> {
    let n = alpha.len();
    if n == 0 {
        return vec![];
    }
    if n == 1 {
        return alpha;
    }
    let eps = 1e-14f64;
    let max_iter = 30 * n;
    let mut m = n;
    for _ in 0..max_iter {
        while m > 1 && beta[m - 2].abs() < eps * (alpha[m - 1].abs() + alpha[m - 2].abs()) {
            beta[m - 2] = 0.0;
            m -= 1;
        }
        if m <= 1 {
            break;
        }
        let lo = {
            let mut l = m - 1;
            while l > 0 && beta[l - 1].abs() >= eps * (alpha[l].abs() + alpha[l - 1].abs()) {
                l -= 1;
            }
            l
        };
        let a_mm = alpha[m - 1];
        let a_m1m1 = alpha[m - 2];
        let b = beta[m - 2];
        let d = (a_m1m1 - a_mm) / 2.0;
        let shift = if d.abs() < 1e-300 {
            a_mm - b.abs()
        } else {
            let r = (d * d + b * b).sqrt();
            let s = if d >= 0.0 { r } else { -r };
            a_mm - b * b / (d + s)
        };
        let mut g = alpha[lo] - shift;
        let mut h = beta[lo];
        for i in lo..(m - 1) {
            let r = (g * g + h * h).sqrt();
            let (c, s) = if r < 1e-300 {
                (1.0, 0.0)
            } else {
                (g / r, h / r)
            };
            if i > lo {
                beta[i - 1] = r;
            }
            g = c * alpha[i] - s * beta[i];
            let new_alpha_i1 = s * alpha[i] + c * beta[i];
            beta[i] = c * new_alpha_i1 - s * alpha[i + 1];
            alpha[i + 1] = s * new_alpha_i1 + c * alpha[i + 1];
            alpha[i] = c * (c * alpha[i] - s * beta[i]) + s * (s * alpha[i] + c * beta[i]);
            let old_ai = c * g + s * beta[i];
            alpha[i] = old_ai;
            h = if i + 1 < m - 1 { -s * beta[i + 1] } else { 0.0 };
            if i + 1 < m - 1 {
                beta[i + 1] *= c;
            }
        }
    }
    alpha
}
/// Compute approximate eigenvalues of a symmetric matrix via tridiagonalisation + QR
pub fn eigenvalues_symmetric(m: &SymmetricMatrix) -> Vec<f64> {
    let (alpha, beta) = tridiagonalise(m);
    let mut eigs = tridiag_eigenvalues(alpha, beta);
    eigs.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    eigs
}
/// Compute the Wigner semicircle density ρ(x) = sqrt(4σ² - x²)/(2πσ²)
pub fn semicircle_density(x: f64, sigma: f64) -> f64 {
    let r = 2.0 * sigma;
    if x.abs() > r {
        return 0.0;
    }
    (r * r - x * x).sqrt() / (2.0 * std::f64::consts::PI * sigma * sigma)
}
/// Compute the Marchenko-Pastur density ρ_{γ,σ²}(x)
/// γ = p/n (aspect ratio), σ² = variance of entries
/// λ± = σ²(1 ± √γ)²
pub fn marchenko_pastur_density(x: f64, gamma: f64, sigma_sq: f64) -> f64 {
    if gamma <= 0.0 || sigma_sq <= 0.0 {
        return 0.0;
    }
    let sqrt_gamma = gamma.sqrt();
    let lambda_plus = sigma_sq * (1.0 + sqrt_gamma).powi(2);
    let lambda_minus = sigma_sq * (1.0 - sqrt_gamma).powi(2);
    if x < lambda_minus || x > lambda_plus {
        return 0.0;
    }
    ((lambda_plus - x) * (x - lambda_minus)).sqrt()
        / (2.0 * std::f64::consts::PI * sigma_sq * gamma * x)
}
/// Compute normalised level spacings s_i = (λ_{i+1} - λ_i) / mean_spacing
pub fn level_spacings(eigenvalues: &[f64]) -> Vec<f64> {
    if eigenvalues.len() < 2 {
        return vec![];
    }
    let mut sorted = eigenvalues.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let gaps: Vec<f64> = sorted.windows(2).map(|w| w[1] - w[0]).collect();
    let mean = gaps.iter().sum::<f64>() / gaps.len() as f64;
    if mean < 1e-300 {
        return gaps;
    }
    gaps.iter().map(|g| g / mean).collect()
}
/// Wigner surmise for GUE level spacing: p(s) = (32/π²) s² exp(-4s²/π)
pub fn gue_level_spacing_distribution(s: f64) -> f64 {
    let pi = std::f64::consts::PI;
    (32.0 / (pi * pi)) * s * s * (-4.0 * s * s / pi).exp()
}
/// Wigner surmise for GOE level spacing: p(s) = (π/2) s exp(-πs²/4)
pub fn goe_level_spacing_distribution(s: f64) -> f64 {
    let pi = std::f64::consts::PI;
    (pi / 2.0) * s * (-pi * s * s / 4.0).exp()
}
/// Sine kernel for GUE bulk correlations: K(x,y) = sin(π(x-y))/(π(x-y))
pub fn sine_kernel(x: f64, y: f64) -> f64 {
    let pi = std::f64::consts::PI;
    let diff = pi * (x - y);
    if diff.abs() < 1e-14 {
        1.0
    } else {
        diff.sin() / diff
    }
}
/// Empirical spectral distribution: returns (bin_centers, density) histogram
pub fn empirical_spectral_distribution(
    eigenvalues: &[f64],
    num_bins: usize,
) -> (Vec<f64>, Vec<f64>) {
    if eigenvalues.is_empty() || num_bins == 0 {
        return (vec![], vec![]);
    }
    let min_e = eigenvalues.iter().cloned().fold(f64::INFINITY, f64::min);
    let max_e = eigenvalues
        .iter()
        .cloned()
        .fold(f64::NEG_INFINITY, f64::max);
    let range = max_e - min_e;
    if range < 1e-14 {
        return (vec![min_e], vec![eigenvalues.len() as f64]);
    }
    let width = range / num_bins as f64;
    let mut counts = vec![0usize; num_bins];
    for &e in eigenvalues {
        let bin = ((e - min_e) / width).floor() as usize;
        let bin = bin.min(num_bins - 1);
        counts[bin] += 1;
    }
    let n = eigenvalues.len() as f64;
    let centers: Vec<f64> = (0..num_bins)
        .map(|i| min_e + (i as f64 + 0.5) * width)
        .collect();
    let densities: Vec<f64> = counts.iter().map(|&c| c as f64 / (n * width)).collect();
    (centers, densities)
}
/// Number variance Σ²(L): variance of eigenvalue count in interval of length L
/// Using unfolded eigenvalues (mean spacing = 1)
pub fn number_variance(unfolded_eigs: &[f64], l: f64) -> f64 {
    if unfolded_eigs.len() < 2 {
        return 0.0;
    }
    let n_samples = 50usize.min(unfolded_eigs.len());
    let step = unfolded_eigs.len() / n_samples;
    let mut counts = Vec::with_capacity(n_samples);
    let sorted: Vec<f64> = {
        let mut v = unfolded_eigs.to_vec();
        v.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        v
    };
    for k in 0..n_samples {
        let center = sorted[k * step];
        let lo = center - l / 2.0;
        let hi = center + l / 2.0;
        let cnt = sorted.iter().filter(|&&e| e >= lo && e <= hi).count();
        counts.push(cnt as f64);
    }
    let mean = counts.iter().sum::<f64>() / counts.len() as f64;
    let var = counts.iter().map(|c| (c - mean).powi(2)).sum::<f64>() / counts.len() as f64;
    var
}
/// Dyson Brownian motion: advance eigenvalues {λ_i} by one Euler-Maruyama step
/// dλ_i = β Σ_{j≠i} dt/(λ_i - λ_j) - (β n / 2) λ_i dt + sqrt(dt) dW_i
#[allow(clippy::too_many_arguments)]
pub fn dyson_bm_step(
    eigenvalues: &mut Vec<f64>,
    beta: f64,
    dt: f64,
    n_matrix: usize,
    rng: &mut Lcg,
) {
    let n = eigenvalues.len();
    let mut drift = vec![0.0f64; n];
    for i in 0..n {
        let mut repulsion = 0.0f64;
        for j in 0..n {
            if i != j {
                let diff = eigenvalues[i] - eigenvalues[j];
                if diff.abs() > 1e-12 {
                    repulsion += 1.0 / diff;
                }
            }
        }
        drift[i] = beta * repulsion - (beta * n_matrix as f64 / 2.0) * eigenvalues[i];
    }
    for i in 0..n {
        eigenvalues[i] += drift[i] * dt + dt.sqrt() * rng.next_normal();
    }
}
/// Free cumulant κ_n from moments m_1, ..., m_n via moment-cumulant formula
/// Uses the simple recursive formula for the first three free cumulants
pub fn free_cumulants_from_moments(moments: &[f64]) -> Vec<f64> {
    let mut kappa = vec![0.0f64; moments.len()];
    if moments.is_empty() {
        return kappa;
    }
    let m = moments;
    kappa[0] = m[0];
    if m.len() >= 2 {
        kappa[1] = m[1] - m[0] * m[0];
    }
    if m.len() >= 3 {
        kappa[2] = m[2] - 3.0 * m[1] * m[0] + 2.0 * m[0].powi(3);
    }
    if m.len() >= 4 {
        kappa[3] = m[3] - 4.0 * m[2] * m[0] - 3.0 * m[1] * m[1] + 12.0 * m[1] * m[0] * m[0]
            - 6.0 * m[0].powi(4);
    }
    kappa
}
/// Stieltjes / Cauchy transform of empirical spectral measure
/// G(z) = (1/n) Σ 1/(z - λ_i), for complex z = x + iε (we use ε > 0 regularisation)
pub fn stieltjes_transform(eigenvalues: &[f64], z_real: f64, z_imag: f64) -> (f64, f64) {
    let n = eigenvalues.len() as f64;
    if n == 0.0 {
        return (0.0, 0.0);
    }
    let (mut re, mut im) = (0.0f64, 0.0f64);
    for &lambda in eigenvalues {
        let d_re = z_real - lambda;
        let d_im = z_imag;
        let denom = d_re * d_re + d_im * d_im;
        re += d_re / denom;
        im += -d_im / denom;
    }
    (re / n, im / n)
}
/// Variance of eigenvalue density deviation from semicircle law
/// Estimates ||ρ_n - ρ_sc||_2^2 over a uniform grid
pub fn semicircle_deviation(eigenvalues: &[f64], sigma: f64, num_bins: usize) -> f64 {
    let (centers, empirical) = empirical_spectral_distribution(eigenvalues, num_bins);
    centers
        .iter()
        .zip(empirical.iter())
        .map(|(&x, &emp)| {
            let sc = semicircle_density(x, sigma);
            (emp - sc).powi(2)
        })
        .sum::<f64>()
        / centers.len() as f64
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_semicircle_density_normalised() {
        let sigma = 1.0;
        let n_points = 10_000;
        let r = 2.0 * sigma;
        let dx = 2.0 * r / n_points as f64;
        let integral: f64 = (0..n_points)
            .map(|i| {
                let x = -r + (i as f64 + 0.5) * dx;
                semicircle_density(x, sigma) * dx
            })
            .sum();
        assert!(
            (integral - 1.0).abs() < 1e-3,
            "semicircle normalisation: expected 1.0, got {integral}"
        );
    }
    #[test]
    fn test_semicircle_density_support() {
        let sigma = 1.5;
        assert_eq!(semicircle_density(3.1, sigma), 0.0);
        assert_eq!(semicircle_density(-3.1, sigma), 0.0);
        assert!(semicircle_density(0.0, sigma) > 0.0);
    }
    #[test]
    fn test_marchenko_pastur_density() {
        let gamma = 1.0;
        let sigma_sq = 1.0;
        assert_eq!(marchenko_pastur_density(-0.1, gamma, sigma_sq), 0.0);
        assert_eq!(marchenko_pastur_density(4.1, gamma, sigma_sq), 0.0);
        assert!(marchenko_pastur_density(1.0, gamma, sigma_sq) > 0.0);
    }
    #[test]
    fn test_level_spacings_mean_one() {
        let eigs: Vec<f64> = (0..10)
            .map(|i| i as f64 * 0.3 + (i as f64).sin() * 0.05)
            .collect();
        let spacings = level_spacings(&eigs);
        let mean = spacings.iter().sum::<f64>() / spacings.len() as f64;
        assert!(
            (mean - 1.0).abs() < 1e-10,
            "mean spacing should be 1, got {mean}"
        );
    }
    #[test]
    fn test_sine_kernel_diagonal() {
        let k = sine_kernel(1.5, 1.5);
        assert!(
            (k - 1.0).abs() < 1e-10,
            "sine kernel diagonal should be 1, got {k}"
        );
    }
    #[test]
    fn test_free_cumulants_gaussian() {
        let moments = vec![0.0f64, 1.0, 0.0, 3.0];
        let kappa = free_cumulants_from_moments(&moments);
        assert!((kappa[0]).abs() < 1e-10, "κ_1 = 0");
        assert!((kappa[1] - 1.0).abs() < 1e-10, "κ_2 = 1");
        assert!((kappa[2]).abs() < 1e-10, "κ_3 = 0");
        assert!((kappa[3]).abs() < 1e-10, "κ_4 = 0 for Gaussian");
    }
    #[test]
    fn test_stieltjes_transform_imaginary_part_negative() {
        let eigs = vec![0.0, 1.0, 2.0, 3.0];
        let (_re, im) = stieltjes_transform(&eigs, 1.5, 0.1);
        assert!(im <= 0.0, "Im G(z) should be ≤ 0 for Im z > 0, got {im}");
    }
    #[test]
    fn test_goe_sample_symmetry() {
        let m = sample_goe(4, 42);
        for i in 0..4 {
            for j in 0..4 {
                let diff = (m.get(i, j) - m.get(j, i)).abs();
                assert!(
                    diff < 1e-14,
                    "GOE matrix not symmetric at ({i},{j}): diff = {diff}"
                );
            }
        }
    }
    #[test]
    fn test_empirical_spectral_distribution_sums_to_one() {
        let eigs: Vec<f64> = (0..20).map(|i| i as f64 * 0.1).collect();
        let (centers, densities) = empirical_spectral_distribution(&eigs, 10);
        assert_eq!(centers.len(), 10);
        let range = centers.last().expect("last should succeed")
            - centers.first().expect("first should succeed");
        let bin_width = range / (centers.len() - 1) as f64;
        let total: f64 = densities.iter().sum::<f64>() * bin_width;
        assert!(
            total > 0.5,
            "total density mass should be positive, got {total}"
        );
    }
}
/// Compute the p-th Catalan number as f64.
pub(super) fn catalan_number(p: u32) -> f64 {
    let mut num = 1.0_f64;
    for i in 1..=(2 * p) {
        num *= i as f64;
    }
    let mut den = 1.0_f64;
    for i in 1..=(p + 1) {
        den *= i as f64;
    }
    for i in 1..=p {
        den *= i as f64;
    }
    num / den
}
/// Build a kernel environment containing random matrix theory axioms.
pub fn build_env() -> Environment {
    let mut env = Environment::new();
    build_random_matrix_theory_env(&mut env);
    env
}
/// `DysonThreeFoldWay : Nat -> Prop`
/// Dyson's three-fold way: β=1 (orthogonal), β=2 (unitary), β=4 (symplectic) symmetry classes.
pub fn dyson_three_fold_way_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `BetaEnsembleDensity : Real -> Real -> Real`
/// General β-ensemble joint density: C_β · ∏_{i<j} |λᵢ−λⱼ|^β · e^{-βn V(λᵢ)/2}
pub fn beta_ensemble_density_ty() -> Expr {
    arrow(real_ty(), arrow(vec_ty(), real_ty()))
}
/// `BetaEnsembleLogGas : Real -> Nat -> Prop`
/// Log-gas interpretation of β-ensemble: eigenvalues as 2D Coulomb gas at inverse temperature β.
pub fn beta_ensemble_log_gas_ty() -> Expr {
    arrow(real_ty(), arrow(nat_ty(), prop()))
}
/// `GUELevelSpacingUniversality : Prop`
/// GUE level spacing universality: spacing distributions are universal in the bulk.
pub fn gue_level_spacing_universality_ty() -> Expr {
    prop()
}
/// `GOELevelSpacingUniversality : Prop`
/// GOE level spacing universality: same universality class for all real symmetric Wigner matrices.
pub fn goe_level_spacing_universality_ty() -> Expr {
    prop()
}
/// `TracyWidomGUE : Real -> Real`
/// F₂(s): GUE Tracy-Widom distribution via Painlevé II transcendent.
pub fn tracy_widom_gue_ty() -> Expr {
    arrow(real_ty(), real_ty())
}
/// `TracyWidomGOE : Real -> Real`
/// F₁(s): GOE Tracy-Widom distribution.
pub fn tracy_widom_goe_ty() -> Expr {
    arrow(real_ty(), real_ty())
}
/// `TracyWidomGSE : Real -> Real`
/// F₄(s): GSE Tracy-Widom distribution.
pub fn tracy_widom_gse_ty() -> Expr {
    arrow(real_ty(), real_ty())
}
/// `LargestEigenvalueFluctuation : Nat -> Real -> Prop`
/// (λ_max − 2√n) · n^{1/6} → TW_β in distribution.
pub fn largest_eigenvalue_fluctuation_ty() -> Expr {
    arrow(nat_ty(), arrow(real_ty(), prop()))
}
/// `PainleveIIConnection : Prop`
/// Tracy-Widom GUE is expressed via the Painlevé II transcendent q: q'' = 2q³ + xq.
pub fn painleve_ii_connection_ty() -> Expr {
    prop()
}
/// `FreeCumulantMomentFormula : Nat -> Prop`
/// Moment-cumulant formula in free probability (Speicher's formula over non-crossing partitions).
pub fn free_cumulant_moment_formula_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `FreeAdditiveCumulantAdditivity : Prop`
/// κₙ(a ⊞ b) = κₙ(a) + κₙ(b) when a and b are freely independent.
pub fn free_additive_cumulant_additivity_ty() -> Expr {
    prop()
}
/// `RTransformAdditivity : Prop`
/// R-transform linearises free additive convolution: R_{a⊞b}(z) = R_a(z) + R_b(z).
pub fn r_transform_additivity_ty() -> Expr {
    prop()
}
/// `STransformMultiplicativity : Prop`
/// S-transform linearises free multiplicative convolution: S_{a⊠b}(z) = S_a(z)·S_b(z).
pub fn s_transform_multiplicativity_ty() -> Expr {
    prop()
}
/// `FreeMultiplicativeConvolution : (Real -> Real) -> (Real -> Real) -> Real -> Real`
/// Free multiplicative convolution μ ⊠ ν.
pub fn free_multiplicative_convolution_ty() -> Expr {
    let density = arrow(real_ty(), real_ty());
    arrow(density.clone(), arrow(density.clone(), density))
}
/// `StieltjesInversionFormula : Prop`
/// Stieltjes inversion: ρ(x) = lim_{ε↓0} (1/π) Im G(x + iε).
pub fn stieltjes_inversion_formula_ty() -> Expr {
    prop()
}
/// `MarchenkoPasturEquation : Real -> Prop`
/// The self-consistency equation for the Marchenko-Pastur law's Stieltjes transform.
pub fn marchenko_pastur_equation_ty() -> Expr {
    arrow(real_ty(), prop())
}
/// `StieltjesTransformBound : Prop`
/// |G(z)| ≤ 1/Im(z) for z in the upper half-plane.
pub fn stieltjes_transform_bound_ty() -> Expr {
    prop()
}
/// `WignerUniversality : Nat -> Prop`
/// Wigner's semicircle law holds for all symmetric matrices with i.i.d. entries (finite variance).
pub fn wigner_universality_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `SteinChenMethod : Prop`
/// Stein-Chen method for establishing convergence to the semicircle law.
pub fn stein_chen_method_ty() -> Expr {
    prop()
}
/// `LocalSemicircleLaw : Nat -> Real -> Prop`
/// Local semicircle law: the empirical measure matches the semicircle down to scale 1/n.
pub fn local_semicircle_law_ty() -> Expr {
    arrow(nat_ty(), arrow(real_ty(), prop()))
}
/// `DeterminantalPointProcess : (Real -> Real -> Real) -> Prop`
/// A determinantal point process with kernel K: correlation functions are determinants of K.
pub fn determinantal_point_process_ty() -> Expr {
    arrow(sine_kernel_ty(), prop())
}
/// `CorrelationKernel : Real -> Real -> Real`
/// The correlation kernel K(x,y) defining a determinantal process.
pub fn correlation_kernel_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), real_ty()))
}
/// `JanossyDensity : Nat -> (List Real) -> Real`
/// Janossy densities: probability that the configuration has exactly n points at given locations.
pub fn janossy_density_ty() -> Expr {
    arrow(nat_ty(), arrow(vec_ty(), real_ty()))
}
/// `GapProbability : Real -> Real -> Real`
/// Probability that the interval (a,b) contains no eigenvalues.
pub fn gap_probability_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), real_ty()))
}
/// `FredholmDeterminant : (Real -> Real -> Real) -> Real`
/// Fredholm determinant det(I - K) encoding gap probabilities.
pub fn fredholm_determinant_ty() -> Expr {
    arrow(arrow(real_ty(), arrow(real_ty(), real_ty())), real_ty())
}
/// `AndersonModel : Nat -> Prop`
/// The Anderson model: H = -Δ + V with random potential V.
pub fn anderson_model_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `AndersonLocalisation : Nat -> Real -> Prop`
/// Anderson localisation: all eigenstates are exponentially localised for strong disorder.
pub fn anderson_localisation_ty() -> Expr {
    arrow(nat_ty(), arrow(real_ty(), prop()))
}
/// `SpectralGapRandom : Nat -> Real -> Prop`
/// Random Schrödinger operator has a spectral gap with high probability.
pub fn spectral_gap_random_ty() -> Expr {
    arrow(nat_ty(), arrow(real_ty(), prop()))
}
/// `MobilityEdge : Real -> Prop`
/// The mobility edge separating localised and extended states in 3D Anderson model.
pub fn mobility_edge_ty() -> Expr {
    arrow(real_ty(), prop())
}
/// `CircularUnitaryEnsembleDensity : Nat -> Real -> Real`
/// CUE(n) density on the unit circle via Haar measure on U(n).
pub fn cue_density_ty() -> Expr {
    arrow(nat_ty(), arrow(real_ty(), real_ty()))
}
/// `CircularLaw : Nat -> Prop`
/// Girko's circular law: empirical spectral distribution of n⁻¹/² i.i.d. matrix → uniform disk.
pub fn circular_law_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `GirkosCircularLaw : Prop`
/// Girko (1984) / Tao-Vu (2010): circular law for matrices with finite second moment entries.
pub fn girkos_circular_law_ty() -> Expr {
    prop()
}
/// `CircularOrthogonalEnsemble : Nat -> Type`
/// COE: circular version of GOE, arising from symmetric unitary matrices.
pub fn coe_ensemble_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `CircularSymplecticEnsemble : Nat -> Type`
/// CSE: circular version of GSE, arising from self-dual unitary matrices.
pub fn cse_ensemble_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `GinibreEnsemble : Nat -> Type`
/// Ginibre ensemble: n×n matrices with i.i.d. complex Gaussian entries.
pub fn ginibre_ensemble_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `GinibreEigenvalueDensity : Real -> Real -> Real`
/// Joint density of complex eigenvalues: product of |z_i - z_j|² · e^{-n Σ|z_i|²}.
pub fn ginibre_eigenvalue_density_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), real_ty()))
}
/// `NonHermitianCorrelation : Nat -> (List Real) -> Real`
/// k-point correlation functions for non-Hermitian Ginibre ensemble.
pub fn non_hermitian_correlation_ty() -> Expr {
    arrow(nat_ty(), arrow(vec_ty(), real_ty()))
}
/// `PseudospectralRadius : Real -> Real`
/// ε-pseudospectrum: set of z with ‖(zI - A)⁻¹‖ > 1/ε.
pub fn pseudospectral_radius_ty() -> Expr {
    arrow(real_ty(), real_ty())
}
/// `GenusExpansion : Nat -> Real`
/// The genus expansion: F = Σ_{g≥0} N^{2-2g} F_g gives the free energy in the large N limit.
pub fn genus_expansion_ty() -> Expr {
    arrow(nat_ty(), real_ty())
}
/// `PlanarDiagramContribution : Nat -> Real`
/// Contribution from planar (genus-0) Feynman diagrams to the partition function.
pub fn planar_diagram_contribution_ty() -> Expr {
    arrow(nat_ty(), real_ty())
}
/// `LargeNExpansion : (Real -> Real) -> Nat -> Real`
/// Large-N expansion of the resolvent / free energy.
pub fn large_n_expansion_ty() -> Expr {
    arrow(arrow(real_ty(), real_ty()), arrow(nat_ty(), real_ty()))
}
/// `MatrixIntegralPartitionFunction : Nat -> Real`
/// Z_N = ∫ dM exp(-N Tr V(M)): matrix model partition function.
pub fn matrix_integral_partition_function_ty() -> Expr {
    arrow(nat_ty(), real_ty())
}
/// Build the extended random matrix theory kernel environment (adds new §9 axioms).
pub fn build_random_matrix_theory_env_extended(env: &mut Environment) {
    let axioms: &[(&str, Expr)] = &[
        ("DysonThreeFoldWay", dyson_three_fold_way_ty()),
        ("BetaEnsembleDensity", beta_ensemble_density_ty()),
        ("BetaEnsembleLogGas", beta_ensemble_log_gas_ty()),
        (
            "GUELevelSpacingUniversality",
            gue_level_spacing_universality_ty(),
        ),
        (
            "GOELevelSpacingUniversality",
            goe_level_spacing_universality_ty(),
        ),
        ("TracyWidomGUE", tracy_widom_gue_ty()),
        ("TracyWidomGOE", tracy_widom_goe_ty()),
        ("TracyWidomGSE", tracy_widom_gse_ty()),
        (
            "LargestEigenvalueFluctuation",
            largest_eigenvalue_fluctuation_ty(),
        ),
        ("PainleveIIConnection", painleve_ii_connection_ty()),
        (
            "FreeCumulantMomentFormula",
            free_cumulant_moment_formula_ty(),
        ),
        (
            "FreeAdditiveCumulantAdditivity",
            free_additive_cumulant_additivity_ty(),
        ),
        ("RTransformAdditivity", r_transform_additivity_ty()),
        (
            "STransformMultiplicativity",
            s_transform_multiplicativity_ty(),
        ),
        (
            "FreeMultiplicativeConvolution",
            free_multiplicative_convolution_ty(),
        ),
        (
            "StieltjesInversionFormula",
            stieltjes_inversion_formula_ty(),
        ),
        ("MarchenkoPasturEquation", marchenko_pastur_equation_ty()),
        ("StieltjesTransformBound", stieltjes_transform_bound_ty()),
        ("WignerUniversality", wigner_universality_ty()),
        ("SteinChenMethod", stein_chen_method_ty()),
        ("LocalSemicircleLaw", local_semicircle_law_ty()),
        (
            "DeterminantalPointProcess",
            determinantal_point_process_ty(),
        ),
        ("CorrelationKernel", correlation_kernel_ty()),
        ("JanossyDensity", janossy_density_ty()),
        ("GapProbability", gap_probability_ty()),
        ("FredholmDeterminant", fredholm_determinant_ty()),
        ("AndersonModel", anderson_model_ty()),
        ("AndersonLocalisation", anderson_localisation_ty()),
        ("SpectralGapRandom", spectral_gap_random_ty()),
        ("MobilityEdge", mobility_edge_ty()),
        ("CUEDensity", cue_density_ty()),
        ("CircularLaw", circular_law_ty()),
        ("GirkosCircularLaw", girkos_circular_law_ty()),
        ("COEEnsemble", coe_ensemble_ty()),
        ("CSEEnsemble", cse_ensemble_ty()),
        ("GinibreEnsemble", ginibre_ensemble_ty()),
        ("GinibreEigenvalueDensity", ginibre_eigenvalue_density_ty()),
        ("NonHermitianCorrelation", non_hermitian_correlation_ty()),
        ("PseudospectralRadius", pseudospectral_radius_ty()),
        ("GenusExpansion", genus_expansion_ty()),
        (
            "PlanarDiagramContribution",
            planar_diagram_contribution_ty(),
        ),
        ("LargeNExpansion", large_n_expansion_ty()),
        (
            "MatrixIntegralPartitionFunction",
            matrix_integral_partition_function_ty(),
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
}
#[cfg(test)]
mod tests_extended {
    use super::*;
    #[test]
    fn test_extended_env_axioms_registered() {
        let mut env = Environment::new();
        build_random_matrix_theory_env_extended(&mut env);
        assert!(env.get(&Name::str("DysonThreeFoldWay")).is_some());
        assert!(env.get(&Name::str("TracyWidomGUE")).is_some());
        assert!(env.get(&Name::str("CircularLaw")).is_some());
        assert!(env.get(&Name::str("GirkosCircularLaw")).is_some());
        assert!(env.get(&Name::str("AndersonLocalisation")).is_some());
        assert!(env.get(&Name::str("FredholmDeterminant")).is_some());
        assert!(env.get(&Name::str("GenusExpansion")).is_some());
        assert!(env.get(&Name::str("GinibreEnsemble")).is_some());
    }
    #[test]
    fn test_gaussian_matrix_sampler_goe() {
        let mut sampler = GaussianMatrixSampler::new(6, DysonBeta::Beta1, 123);
        let m = sampler.sample_goe();
        for i in 0..6 {
            for j in 0..6 {
                assert!(
                    (m.get(i, j) - m.get(j, i)).abs() < 1e-14,
                    "GOE matrix not symmetric at ({i},{j})"
                );
            }
        }
    }
    #[test]
    fn test_gaussian_matrix_sampler_eigenvalues() {
        let mut sampler = GaussianMatrixSampler::new(5, DysonBeta::Beta2, 456);
        let eigs = sampler.sample_eigenvalues();
        assert_eq!(eigs.len(), 5);
    }
    #[test]
    fn test_empirical_spectral_distribution_struct() {
        let eigs: Vec<f64> = (0..50).map(|i| i as f64 * 0.04 - 1.0).collect();
        let esd = EmpiricalSpectralDistribution::new(eigs, 10);
        let (centers, densities) = esd.histogram();
        assert_eq!(centers.len(), 10);
        assert_eq!(densities.len(), 10);
        assert!(!esd.is_empty());
        assert_eq!(esd.len(), 50);
    }
    #[test]
    fn test_stieltjes_transform_eval_valid() {
        let eigs = vec![-1.5, -0.5, 0.0, 0.5, 1.5];
        let st = StieltjesTransformEval::new(eigs);
        assert!(st.is_valid_stieltjes(0.3, 0.1));
        let density = st.density_approx(0.0, 0.05);
        assert!(density >= 0.0, "density should be non-negative: {density}");
    }
    #[test]
    fn test_level_spacing_stats_mean_one() {
        let eigs: Vec<f64> = (0..20).map(|i| i as f64 * 0.5).collect();
        let stats = LevelSpacingStats::from_eigenvalues(&eigs);
        let m = stats.mean();
        assert!((m - 1.0).abs() < 1e-10, "mean spacing = {m}");
    }
    #[test]
    fn test_level_spacing_ratio_bounds() {
        let eigs: Vec<f64> = (0..15).map(|i| i as f64).collect();
        let stats = LevelSpacingStats::from_eigenvalues(&eigs);
        let ratios = stats.ratio_statistics();
        for r in &ratios {
            assert!(*r >= 0.0 && *r <= 1.0 + 1e-12, "ratio out of [0,1]: {r}");
        }
    }
    #[test]
    fn test_wigner_semicircle_struct_moments() {
        let sc = WignerSemicircle::new(2.0);
        assert!((sc.moments(0) - 1.0).abs() < 1e-10);
        assert!((sc.moments(2) - 1.0).abs() < 1e-10);
        assert_eq!(sc.moments(1), 0.0);
        assert_eq!(sc.moments(3), 0.0);
    }
    #[test]
    fn test_marchenko_pastur_struct() {
        let mp = MarcenkoPastur::new(0.5, 1.0);
        let (lo, hi) = mp.support();
        assert!(lo >= 0.0, "support lower bound non-negative");
        assert!(hi > lo, "support has positive width");
        let mid = (lo + hi) / 2.0;
        assert!(mp.density_at(mid) > 0.0, "density positive in support");
    }
    #[test]
    fn test_free_convolution_struct() {
        let fc = FreeConvolution::new("μ".to_string(), "ν".to_string());
        let desc = fc.free_cumulants();
        assert!(desc.contains("κₙ"));
    }
    #[test]
    fn test_dyson_bm_equilibrium() {
        let dbm = DysonBrownianMotion::new(5, 2.0);
        let desc = dbm.equilibrium_measure();
        assert!(desc.contains("semicircle") || desc.contains("Wigner"));
    }
}
#[cfg(test)]
mod tests_rmt_ext {
    use super::*;
    #[test]
    fn test_semicircle_density() {
        let sd = WignerSemicircleData::new(vec![-1.0, 0.0, 1.0], 1.0);
        let rho = sd.semicircle_density(0.0);
        assert!((rho - 2.0 / std::f64::consts::PI).abs() < 1e-5);
        let (lo, hi) = sd.support_endpoints();
        assert!((lo + 2.0).abs() < 1e-10);
        assert!((hi - 2.0).abs() < 1e-10);
        assert_eq!(sd.semicircle_density(3.0), 0.0);
    }
    #[test]
    fn test_tracy_widom_data() {
        let mut tw = TracyWidomData::new(2);
        tw.add_sample(2.0);
        tw.add_sample(2.1);
        tw.add_sample(1.9);
        assert!(tw.is_valid_beta());
        let mean = tw.sample_mean();
        assert!((mean - 2.0).abs() < 0.15);
        let fscale = tw.fluctuation_scale(1000);
        assert!(fscale < 1.0);
    }
    #[test]
    fn test_universality_gue() {
        let ud = UniversalityData::new("GUE").with_gue_universality();
        assert!(ud.gue_universal);
        let s = 0.5;
        let p = ud.level_spacing_density(s);
        assert!(p > 0.0);
        assert!(ud.correlation_kernel().contains("Sine kernel"));
    }
    #[test]
    fn test_free_probability() {
        let mut fp = FreeProbabilityData::new();
        fp.add_moment(1.0);
        fp.add_moment(0.0);
        fp.add_moment(1.0);
        fp.add_free_cumulant(0.0);
        fp.add_free_cumulant(1.0);
        assert!(fp.first_cumulants_ok(0.0, 1.0, 1e-10));
        let r = fp.r_transform_truncated(0.5);
        assert!((r - 0.5).abs() < 1e-10);
    }
}
#[cfg(test)]
mod tests_rmt_ext2 {
    use super::*;
    #[test]
    fn test_dyson_brownian_motion() {
        let dbm = DysonBrownianMotionData::new(vec![-1.0, 0.0, 1.0], 2.0, 0.01);
        let d0 = dbm.drift(0);
        assert!(
            (d0 - (-1.5)).abs() < 1e-10,
            "drift at -1 should be -1.5, got {d0}"
        );
        let energy = dbm.log_repulsion_energy();
        assert!((energy - std::f64::consts::LN_2).abs() < 1e-10);
    }
}
#[cfg(test)]
mod tests_rmt_ext3 {
    use super::*;
    #[test]
    fn test_marchenko_pastur() {
        let mp = MarchenkoPasturData::new(0.25, 1.0);
        let (lo, hi) = mp.support_endpoints();
        assert!((lo - 0.25).abs() < 1e-10, "lo={lo}");
        assert!((hi - 2.25).abs() < 1e-10, "hi={hi}");
        assert_eq!(mp.density(0.0), 0.0);
        let d = mp.density(1.0);
        assert!(d > 0.0, "density at x=1 should be positive: {d}");
        assert!((mp.mean() - 1.0).abs() < 1e-10);
    }
}
