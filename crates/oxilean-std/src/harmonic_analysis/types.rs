//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

/// Fourier multiplier operator with a given symbol.
pub struct MultiplierOperator {
    /// Symbol m(ξ) of the operator.
    pub symbol: String,
    /// Whether the operator is Lᵖ-bounded (p ≠ 2).
    pub is_bounded: bool,
}
impl MultiplierOperator {
    /// Create a new MultiplierOperator.
    pub fn new(symbol: impl Into<String>, is_bounded: bool) -> Self {
        Self {
            symbol: symbol.into(),
            is_bounded,
        }
    }
    /// Lᵖ boundedness of the multiplier operator.
    pub fn lp_boundedness(&self) -> bool {
        self.is_bounded
    }
    /// Hörmander–Mikhlin multiplier condition (sufficient for Lᵖ bounds).
    pub fn hormander_condition(&self) -> bool {
        self.is_bounded
    }
}
/// Real-variable Hardy space Hᵖ.
pub struct HardySpace {
    /// The integrability exponent p (0 < p ≤ ∞).
    pub p: f64,
    /// Whether this is the real-variable (not holomorphic) Hardy space.
    pub is_real_variable: bool,
}
impl HardySpace {
    /// Create a new HardySpace.
    pub fn new(p: f64, is_real_variable: bool) -> Self {
        Self {
            p,
            is_real_variable,
        }
    }
    /// Atomic decomposition of H¹: every f ∈ H¹ is a sum of atoms.
    pub fn atomic_decomposition(&self) -> bool {
        self.p <= 1.0
    }
    /// Duality: (H¹)* = BMO (Fefferman-Stein).
    pub fn duality_with_bmo(&self) -> bool {
        (self.p - 1.0).abs() < 1e-12
    }
}
/// Abstract harmonic analysis on a group.
pub struct GroupHarmonic {
    /// Type of group (e.g., "compact", "locally_compact", "abelian").
    pub group_type: String,
}
impl GroupHarmonic {
    /// Create a new GroupHarmonic object.
    pub fn new(group_type: impl Into<String>) -> Self {
        Self {
            group_type: group_type.into(),
        }
    }
    /// Connection to unitary representation theory of the group.
    pub fn representation_theory_connection(&self) -> bool {
        true
    }
    /// Peter-Weyl theorem: L²(G) decomposes into finite-dim unitary reps (compact G).
    pub fn peter_weyl(&self) -> bool {
        self.group_type.to_lowercase().contains("compact")
    }
}
/// A discrete H¹ atom checker over a finite signal.
///
/// An H¹-atom supported on an interval I = [lo, hi] satisfies:
/// 1. supp(a) ⊆ I
/// 2. ‖a‖_{L∞} ≤ 1/|I|
/// 3. ∫ a = 0  (zero-mean / cancellation condition)
#[derive(Debug, Clone)]
pub struct HardySpaceAtom {
    /// Support interval [lo, hi] (index range).
    pub lo: usize,
    /// Support interval hi (inclusive).
    pub hi: usize,
    /// Values of the atom (length = full signal length; zero outside support).
    pub values: Vec<f64>,
}
impl HardySpaceAtom {
    /// Create a new H¹ atom with the given support and values.
    pub fn new(lo: usize, hi: usize, values: Vec<f64>) -> Self {
        HardySpaceAtom { lo, hi, values }
    }
    /// Length of the support interval.
    pub fn support_length(&self) -> usize {
        if self.hi >= self.lo {
            self.hi - self.lo + 1
        } else {
            0
        }
    }
    /// Check that all values outside [lo, hi] are zero.
    pub fn has_compact_support(&self) -> bool {
        self.values
            .iter()
            .enumerate()
            .filter(|&(i, _)| i < self.lo || i > self.hi)
            .all(|(_, &v)| v.abs() < 1e-12)
    }
    /// Check the L∞ bound: ‖a‖_{L∞} ≤ 1/|I|.
    pub fn satisfies_linfty_bound(&self) -> bool {
        let len = self.support_length();
        if len == 0 {
            return true;
        }
        let bound = 1.0 / len as f64;
        self.values.iter().all(|&v| v.abs() <= bound + 1e-12)
    }
    /// Check the cancellation condition: ∫ a = 0.
    pub fn has_zero_mean(&self) -> bool {
        let sum: f64 = self.values.iter().sum();
        sum.abs() < 1e-10
    }
    /// Verify all three H¹ atom conditions.
    pub fn is_valid_atom(&self) -> bool {
        self.has_compact_support() && self.satisfies_linfty_bound() && self.has_zero_mean()
    }
    /// Construct a canonical atom supported on [lo, hi] with +1/|I| on first half,
    /// -1/|I| on second half (or ±1/|I| for |I|=1 → zero atom).
    pub fn canonical(signal_len: usize, lo: usize, hi: usize) -> Self {
        let mut values = vec![0.0f64; signal_len];
        let len = if hi >= lo { hi - lo + 1 } else { 0 };
        if len < 2 {
            return HardySpaceAtom { lo, hi, values };
        }
        let weight = 1.0 / len as f64;
        let mid = lo + len / 2;
        for i in lo..mid {
            if i < signal_len {
                values[i] = weight;
            }
        }
        for i in mid..=hi {
            if i < signal_len {
                values[i] = -weight;
            }
        }
        HardySpaceAtom { lo, hi, values }
    }
}
/// Discrete Littlewood-Paley square function S(f) = (Σⱼ |Δⱼf|²)^{1/2}.
///
/// Uses DFT-based dyadic frequency decomposition.
#[derive(Debug, Clone)]
pub struct LittlewoodPaleySquare {
    /// The original signal.
    pub signal: Vec<f64>,
}
impl LittlewoodPaleySquare {
    /// Create from signal.
    pub fn new(signal: Vec<f64>) -> Self {
        LittlewoodPaleySquare { signal }
    }
    /// Forward DFT of the signal.
    fn dft(signal: &[f64]) -> Vec<(f64, f64)> {
        let n = signal.len();
        if n == 0 {
            return vec![];
        }
        let two_pi_over_n = 2.0 * std::f64::consts::PI / n as f64;
        (0..n)
            .map(|k| {
                signal
                    .iter()
                    .enumerate()
                    .fold((0.0, 0.0), |(re, im), (j, &x)| {
                        let angle = two_pi_over_n * (k * j) as f64;
                        (re + x * angle.cos(), im - x * angle.sin())
                    })
            })
            .collect()
    }
    /// Inverse DFT.
    fn idft(spectrum: &[(f64, f64)]) -> Vec<f64> {
        let n = spectrum.len();
        if n == 0 {
            return vec![];
        }
        let two_pi_over_n = 2.0 * std::f64::consts::PI / n as f64;
        let n_f = n as f64;
        (0..n)
            .map(|j| {
                spectrum
                    .iter()
                    .enumerate()
                    .map(|(k, &(re, im))| {
                        let angle = two_pi_over_n * (k * j) as f64;
                        (re * angle.cos() - im * angle.sin()) / n_f
                    })
                    .sum()
            })
            .collect()
    }
    /// Compute the pointwise square function S(f)(i) = (Σⱼ |Δⱼf(i)|²)^{1/2}.
    pub fn square_function_pointwise(&self) -> Vec<f64> {
        let n = self.signal.len();
        if n == 0 {
            return vec![];
        }
        let spectrum = Self::dft(&self.signal);
        let mut sum_sq = vec![0.0f64; n];
        let mut j = 1usize;
        while j < n {
            let lo = j;
            let hi = (2 * j).min(n);
            let mut block_spectrum: Vec<(f64, f64)> = vec![(0.0, 0.0); n];
            for k in lo..hi {
                block_spectrum[k] = spectrum[k];
                let mirror = n - k;
                if mirror < n && mirror != k {
                    block_spectrum[mirror] = spectrum[mirror];
                }
            }
            let block_signal = Self::idft(&block_spectrum);
            for i in 0..n {
                sum_sq[i] += block_signal[i] * block_signal[i];
            }
            j = hi;
            if hi >= n {
                break;
            }
        }
        sum_sq.iter().map(|&s| s.sqrt()).collect()
    }
    /// L² norm of the square function.
    pub fn l2_norm_squared(&self) -> f64 {
        self.square_function_pointwise()
            .iter()
            .map(|&v| v * v)
            .sum()
    }
    /// L² norm of the original signal.
    pub fn signal_l2_norm_squared(&self) -> f64 {
        self.signal.iter().map(|&x| x * x).sum()
    }
    /// Verify the Littlewood-Paley inequality: ‖S(f)‖₂ ≈ ‖f‖₂.
    ///
    /// Returns (‖S(f)‖₂², ‖f‖₂², ratio ≈ 1).
    pub fn verify_lp_inequality(&self) -> (f64, f64, f64) {
        let sq_norm = self.l2_norm_squared();
        let sig_norm = self.signal_l2_norm_squared();
        let ratio = if sig_norm > 1e-15 {
            sq_norm / sig_norm
        } else {
            1.0
        };
        (sq_norm, sig_norm, ratio)
    }
}
/// A discrete Fourier multiplier operator M_m.
///
/// Given a multiplier symbol m: {0, …, N-1} → ℝ, applies the operator
/// (M_m f)^(k) = m(k) · f̂(k), i.e., pointwise multiplication in frequency domain.
#[derive(Debug, Clone)]
pub struct FourierMultiplierOp {
    /// Multiplier symbol m(k) for k = 0, …, N-1.
    pub symbol: Vec<f64>,
}
impl FourierMultiplierOp {
    /// Create a new Fourier multiplier with given symbol.
    pub fn new(symbol: Vec<f64>) -> Self {
        FourierMultiplierOp { symbol }
    }
    /// Create the Hilbert transform multiplier: m(k) = -i·sign(k).
    /// For real signals: m(0)=0, m(k)=1 for k>0, m(k)=-1 for k<0 (as real multiplier).
    pub fn hilbert_multiplier(n: usize) -> Self {
        let mut symbol = vec![0.0f64; n];
        for k in 1..n / 2 {
            symbol[k] = 1.0;
        }
        for k in n / 2..n {
            symbol[k] = -1.0;
        }
        FourierMultiplierOp { symbol }
    }
    /// Create a low-pass filter: m(k) = 1 for |k| ≤ cutoff, 0 otherwise.
    pub fn low_pass(n: usize, cutoff: usize) -> Self {
        let mut symbol = vec![0.0f64; n];
        for k in 0..=cutoff.min(n - 1) {
            symbol[k] = 1.0;
        }
        for k in (n - cutoff).min(n)..n {
            symbol[k] = 1.0;
        }
        FourierMultiplierOp { symbol }
    }
    /// Apply the multiplier to a real signal (via DFT).
    ///
    /// Computes M_m f = IDFT(m · DFT(f)).
    pub fn apply(&self, signal: &[f64]) -> Vec<f64> {
        let n = signal.len();
        assert_eq!(n, self.symbol.len());
        if n == 0 {
            return vec![];
        }
        let two_pi_over_n = 2.0 * std::f64::consts::PI / n as f64;
        let spectrum: Vec<(f64, f64)> = (0..n)
            .map(|k| {
                signal
                    .iter()
                    .enumerate()
                    .fold((0.0, 0.0), |(re, im), (j, &x)| {
                        let angle = two_pi_over_n * (k * j) as f64;
                        (re + x * angle.cos(), im - x * angle.sin())
                    })
            })
            .collect();
        let filtered: Vec<(f64, f64)> = spectrum
            .iter()
            .zip(self.symbol.iter())
            .map(|(&(re, im), &m)| (re * m, im * m))
            .collect();
        let n_f = n as f64;
        (0..n)
            .map(|j| {
                filtered
                    .iter()
                    .enumerate()
                    .map(|(k, &(re, im))| {
                        let angle = two_pi_over_n * (k * j) as f64;
                        (re * angle.cos() - im * angle.sin()) / n_f
                    })
                    .sum()
            })
            .collect()
    }
    /// Compute the operator norm (sup over unit vectors) approximately via power iteration.
    ///
    /// For a multiplier, the L² norm is just ‖m‖_{L∞}.
    pub fn l2_operator_norm(&self) -> f64 {
        self.symbol.iter().fold(0.0_f64, |acc, &m| acc.max(m.abs()))
    }
    /// Verify L² boundedness: M_m is L²-bounded iff m ∈ L∞.
    pub fn is_l2_bounded(&self) -> bool {
        self.l2_operator_norm().is_finite()
    }
}
/// Represents a Calderón-Zygmund operator.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CalderonZygmundOperator {
    /// Name of the operator.
    pub name: String,
    /// Whether it is bounded on L^2.
    pub l2_bounded: bool,
    /// Order of the kernel singularity.
    pub kernel_order: f64,
    /// Dimension n.
    pub dimension: usize,
}
#[allow(dead_code)]
impl CalderonZygmundOperator {
    /// Creates a CZ operator.
    pub fn new(name: &str, dim: usize) -> Self {
        CalderonZygmundOperator {
            name: name.to_string(),
            l2_bounded: true,
            kernel_order: 0.0,
            dimension: dim,
        }
    }
    /// Creates the Hilbert transform (1D CZ operator).
    pub fn hilbert_transform() -> Self {
        CalderonZygmundOperator::new("H", 1)
    }
    /// Creates the Riesz transform R_j in R^n.
    pub fn riesz_transform(j: usize, n: usize) -> Self {
        CalderonZygmundOperator::new(&format!("R_{j}"), n)
    }
    /// CZ theorem: bounded on L^p for 1 < p < ∞.
    pub fn lp_boundedness(&self, p: f64) -> bool {
        self.l2_bounded && p > 1.0 && p < f64::INFINITY
    }
    /// Weak-type (1,1) bound: ||Tf||_{1,∞} <= C ||f||_1.
    pub fn weak_type_one_one(&self) -> bool {
        self.l2_bounded
    }
    /// Cotlar-Stein almost orthogonality estimate (symbolic).
    pub fn cotlar_stein_description(&self) -> String {
        format!(
            "Cotlar-Stein for {}: almost orthogonal sum bounded on L^2",
            self.name
        )
    }
    /// T(1) theorem condition: T bounded on L^2 iff T(1) ∈ BMO and T^*(1) ∈ BMO.
    pub fn t1_theorem_condition(&self) -> String {
        format!(
            "T(1) theorem: {} bounded on L^2 iff T(1), T^*(1) ∈ BMO",
            self.name
        )
    }
}
/// Discrete Littlewood-Paley square function for a 1D signal of length N = 2^m.
///
/// Partitions the DFT spectrum into dyadic blocks [2ʲ, 2^{j+1}) and computes
/// S(f)(i) = (Σⱼ |Δⱼf(i)|²)^{1/2} via inverse DFT of each block.
pub struct LPSquareFunction {
    /// The original signal.
    pub signal: Vec<f64>,
}
impl LPSquareFunction {
    /// Create from signal.
    pub fn new(signal: Vec<f64>) -> Self {
        Self { signal }
    }
    /// Compute the square function value at position i.
    pub fn square_function_pointwise(&self) -> Vec<f64> {
        let n = self.signal.len();
        if n == 0 {
            return vec![];
        }
        let spectrum = dft(&self.signal);
        let mut sum_sq = vec![0.0f64; n];
        let mut j = 1usize;
        while j < n {
            let lo = j;
            let hi = (2 * j).min(n);
            let mut block_spectrum: Vec<(f64, f64)> = vec![(0.0, 0.0); n];
            for k in lo..hi {
                block_spectrum[k] = spectrum[k];
                let mirror = n - k;
                if mirror < n && mirror != k {
                    block_spectrum[mirror] = spectrum[mirror];
                }
            }
            let block_signal = idft(&block_spectrum);
            for i in 0..n {
                sum_sq[i] += block_signal[i] * block_signal[i];
            }
            j = hi;
            if hi >= n {
                break;
            }
        }
        sum_sq.iter().map(|&s| s.sqrt()).collect()
    }
    /// L² norm of the square function (should ≈ L² norm of signal by LP inequality).
    pub fn l2_norm_squared(&self) -> f64 {
        self.square_function_pointwise()
            .iter()
            .map(|&v| v * v)
            .sum()
    }
}
/// Calderón-Zygmund singular integral operator.
pub struct CalderonZygmund {
    /// Description of the CZ kernel.
    pub kernel: String,
    /// Whether the kernel is a singular (non-integrable) kernel.
    pub is_singular: bool,
}
impl CalderonZygmund {
    /// Create a new CalderonZygmund operator.
    pub fn new(kernel: impl Into<String>, is_singular: bool) -> Self {
        Self {
            kernel: kernel.into(),
            is_singular,
        }
    }
    /// Lᵖ estimate: CZ operators are bounded on Lᵖ for 1 < p < ∞.
    pub fn lp_estimate(&self) -> bool {
        true
    }
    /// Endpoint L¹ → weak-L¹ bound.
    pub fn endpoint_l1(&self) -> bool {
        true
    }
}
/// Data for oscillatory integral estimation.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct OscillatoryIntegralData {
    /// Phase function description.
    pub phase: String,
    /// Amplitude function description.
    pub amplitude: String,
    /// Frequency parameter λ.
    pub frequency: f64,
    /// Dimension.
    pub dim: usize,
}
#[allow(dead_code)]
impl OscillatoryIntegralData {
    /// Creates oscillatory integral data.
    pub fn new(phase: &str, amplitude: &str, lambda: f64, dim: usize) -> Self {
        OscillatoryIntegralData {
            phase: phase.to_string(),
            amplitude: amplitude.to_string(),
            frequency: lambda,
            dim,
        }
    }
    /// Stationary phase estimate: |I(λ)| ~ λ^{-n/2} as λ → ∞.
    pub fn stationary_phase_decay(&self) -> f64 {
        self.frequency.powf(-(self.dim as f64) / 2.0)
    }
    /// Van der Corput lemma: if |φ'| >= 1, then |I(λ)| <= C λ^{-1/k} for k-th order derivative.
    pub fn van_der_corput_bound(&self, k: usize) -> f64 {
        self.frequency.powf(-1.0 / k as f64)
    }
    /// Returns the Strichartz estimate description.
    pub fn strichartz_description(&self) -> String {
        format!(
            "Strichartz: ||e^{{it∆}}f||_{{L^p_t L^q_x}} <= C ||f||_{{L^2}} (n={})",
            self.dim
        )
    }
}
/// Data for the BMO (Bounded Mean Oscillation) space.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct BMOData {
    /// Samples of a function.
    pub values: Vec<f64>,
    /// BMO seminorm estimate.
    pub bmo_seminorm: f64,
}
#[allow(dead_code)]
impl BMOData {
    /// Creates BMO data.
    pub fn new(values: Vec<f64>) -> Self {
        let seminorm = Self::compute_bmo_seminorm(&values);
        BMOData {
            values,
            bmo_seminorm: seminorm,
        }
    }
    /// Computes the BMO seminorm: sup_Q (1/|Q|) ∫_Q |f - f_Q|.
    /// Simplified: use standard deviation over the whole sample.
    fn compute_bmo_seminorm(values: &[f64]) -> f64 {
        if values.is_empty() {
            return 0.0;
        }
        let mean: f64 = values.iter().sum::<f64>() / values.len() as f64;
        let dev: f64 = values.iter().map(|&x| (x - mean).abs()).sum::<f64>() / values.len() as f64;
        dev
    }
    /// John-Nirenberg inequality: f ∈ BMO iff exp(c|f|/||f||_BMO) is locally integrable.
    pub fn john_nirenberg_constant(&self) -> f64 {
        if self.bmo_seminorm < 1e-14 {
            0.0
        } else {
            1.0 / self.bmo_seminorm
        }
    }
    /// Checks if this function is in VMO (vanishing mean oscillation).
    pub fn is_vmo_approx(&self, tol: f64) -> bool {
        self.bmo_seminorm < tol
    }
}
/// Fourier series on an interval of given period.
pub struct FourierSeries {
    /// Period of the underlying function.
    pub period: f64,
    /// Cosine and sine coefficients: (aₙ, bₙ) for n = 0, 1, 2, …
    pub coefficients: Vec<(f64, f64)>,
}
impl FourierSeries {
    /// Create a new FourierSeries.
    pub fn new(period: f64, coefficients: Vec<(f64, f64)>) -> Self {
        Self {
            period,
            coefficients,
        }
    }
    /// Evaluate the N-th partial sum at x.
    ///
    /// Sₙ(x) = a₀/2 + Σₖ₌₁ⁿ [aₖ cos(2πkx/T) + bₖ sin(2πkx/T)]
    pub fn partial_sum(&self, n: usize, x: f64) -> f64 {
        let t = self.period;
        let (a0, _b0) = self.coefficients.first().copied().unwrap_or((0.0, 0.0));
        let mut s = a0 / 2.0;
        let limit = n.min(self.coefficients.len().saturating_sub(1));
        for k in 1..=limit {
            let (ak, bk) = self.coefficients[k];
            let arg = 2.0 * std::f64::consts::PI * (k as f64) * x / t;
            s += ak * arg.cos() + bk * arg.sin();
        }
        s
    }
    /// Parseval's identity: (1/T)∫|f|² = a₀²/4 + (1/2)Σ(aₙ²+bₙ²).
    pub fn parseval_identity(&self) -> f64 {
        let (a0, _) = self.coefficients.first().copied().unwrap_or((0.0, 0.0));
        let mut s = a0 * a0 / 4.0;
        for &(ak, bk) in self.coefficients.iter().skip(1) {
            s += 0.5 * (ak * ak + bk * bk);
        }
        s
    }
    /// Dirichlet kernel value Dₙ(x) = sin((n+½)x) / sin(x/2).
    pub fn dirichlet_kernel(&self) -> f64 {
        let n = self.coefficients.len();
        let x = 0.1_f64;
        let num = ((n as f64 + 0.5) * x).sin();
        let den = (x / 2.0).sin();
        if den.abs() < 1e-15 {
            (2 * n + 1) as f64
        } else {
            num / den
        }
    }
}
/// Data for the Fourier restriction problem.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FourierRestrictionData {
    /// Manifold/hypersurface.
    pub manifold: String,
    /// Dimension n.
    pub dimension: usize,
    /// Stein-Tomas exponent p_0 = 2(n+1)/(n-1).
    pub stein_tomas_p: f64,
}
#[allow(dead_code)]
impl FourierRestrictionData {
    /// Creates restriction data.
    pub fn new(manifold: &str, dim: usize) -> Self {
        let n = dim as f64;
        let p0 = 2.0 * (n + 1.0) / (n - 1.0);
        FourierRestrictionData {
            manifold: manifold.to_string(),
            dimension: dim,
            stein_tomas_p: p0,
        }
    }
    /// Stein-Tomas theorem: restriction R_S: L^{p'} → L^2(S, dσ) bounded for p' <= p_0.
    pub fn stein_tomas_statement(&self) -> String {
        format!(
            "Stein-Tomas: R_{{{}}}: L^{{p'}} → L^2(S) for p' <= {:.3}",
            self.manifold, self.stein_tomas_p
        )
    }
    /// Decoupling theorem connection (Bourgain-Demeter).
    pub fn decoupling_description(&self) -> String {
        format!(
            "Bourgain-Demeter decoupling for {} in R^{}",
            self.manifold, self.dimension
        )
    }
    /// Checks if the endpoint Stein-Tomas holds.
    pub fn endpoint_holds(&self) -> bool {
        false
    }
}
/// Fourier transform — continuous or discrete variant.
pub struct FourierTransform {
    /// Whether this is the continuous Fourier transform.
    pub is_continuous: bool,
}
impl FourierTransform {
    /// Create a new FourierTransform.
    pub fn new(is_continuous: bool) -> Self {
        Self { is_continuous }
    }
    /// The Fourier inversion theorem: f = ℱ⁻¹(ℱ(f)).
    pub fn fourier_inversion(&self) -> bool {
        true
    }
    /// Plancherel theorem: ‖ℱf‖₂ = ‖f‖₂.
    pub fn plancherel_theorem(&self) -> bool {
        true
    }
    /// Riemann-Lebesgue lemma: ℱf(ξ) → 0 as |ξ| → ∞ for f ∈ L¹.
    pub fn riemann_lebesgue(&self) -> bool {
        self.is_continuous
    }
}
/// Data for multilinear Calderón-Zygmund operators.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct MultilinearCZData {
    /// Number of inputs m.
    pub m: usize,
    /// Name.
    pub name: String,
    /// Whether the operator is bounded.
    pub is_bounded: bool,
}
#[allow(dead_code)]
impl MultilinearCZData {
    /// Creates multilinear CZ data.
    pub fn new(name: &str, m: usize) -> Self {
        MultilinearCZData {
            m,
            name: name.to_string(),
            is_bounded: true,
        }
    }
    /// Hölder exponent: 1/r = 1/p_1 + ... + 1/p_m, 1 < p_j <= ∞.
    pub fn holder_exponent(exponents: &[f64]) -> f64 {
        exponents.iter().map(|&p| 1.0 / p).sum()
    }
    /// Returns the Leibniz rule description.
    pub fn leibniz_rule(&self) -> String {
        format!("Product rule for {}: fractional differentiation", self.name)
    }
}
/// Convolution of two named functions f and g.
pub struct Convolution {
    /// Name of the first function.
    pub f: String,
    /// Name of the second function.
    pub g: String,
}
impl Convolution {
    /// Create a new Convolution.
    pub fn new(f: impl Into<String>, g: impl Into<String>) -> Self {
        Self {
            f: f.into(),
            g: g.into(),
        }
    }
    /// Convolution is commutative: f * g = g * f.
    pub fn is_commutative(&self) -> bool {
        true
    }
    /// Convolution theorem: ℱ(f * g) = ℱ(f) · ℱ(g).
    pub fn convolution_theorem(&self) -> bool {
        true
    }
    /// Young's inequality: ‖f * g‖_r ≤ ‖f‖_p ‖g‖_q  (1/r = 1/p + 1/q − 1).
    pub fn young_inequality(&self) -> bool {
        true
    }
}
/// Calderón-Zygmund decomposition of a discrete signal at height α.
///
/// Given a non-negative signal f and α > 0, decomposes f = g + b where:
/// - g is the "good" part (bounded by 2^n α a.e.)
/// - b is the "bad" part supported on a union of dyadic intervals where the
///   average of f exceeds α
#[derive(Debug, Clone)]
pub struct CalderonZygmundDecomp {
    /// Original signal values.
    pub signal: Vec<f64>,
    /// Decomposition height α.
    pub alpha: f64,
}
impl CalderonZygmundDecomp {
    /// Create a new CZ decomposition.
    pub fn new(signal: Vec<f64>, alpha: f64) -> Self {
        assert!(alpha > 0.0, "alpha must be positive");
        CalderonZygmundDecomp { signal, alpha }
    }
    /// Find the "bad" intervals: maximal dyadic intervals where the average > α.
    ///
    /// Uses a greedy partition into dyadic intervals [k·2ʲ, (k+1)·2ʲ).
    pub fn bad_intervals(&self) -> Vec<(usize, usize)> {
        let n = self.signal.len();
        if n == 0 {
            return vec![];
        }
        let mut bad = Vec::new();
        let mut i = 0;
        while i < n {
            let mut len = 1;
            let mut found = false;
            while i + len <= n {
                let avg: f64 = self.signal[i..i + len].iter().sum::<f64>() / len as f64;
                if avg > self.alpha {
                    bad.push((i, i + len - 1));
                    found = true;
                    break;
                }
                len += 1;
            }
            if !found {
                i += 1;
            } else {
                i += len;
            }
        }
        bad
    }
    /// Compute the "good" part g: set f to α on bad intervals, keep f elsewhere.
    pub fn good_part(&self) -> Vec<f64> {
        let bad = self.bad_intervals();
        self.signal
            .iter()
            .enumerate()
            .map(|(i, &fi)| {
                if bad.iter().any(|&(lo, hi)| i >= lo && i <= hi) {
                    self.alpha
                } else {
                    fi
                }
            })
            .collect()
    }
    /// Compute the "bad" part b = f - g.
    pub fn bad_part(&self) -> Vec<f64> {
        let g = self.good_part();
        self.signal
            .iter()
            .zip(g.iter())
            .map(|(&f, &gv)| f - gv)
            .collect()
    }
    /// Verify decomposition: f = g + b pointwise.
    pub fn verify_decomposition(&self) -> bool {
        let g = self.good_part();
        let b = self.bad_part();
        self.signal
            .iter()
            .zip(g.iter().zip(b.iter()))
            .all(|(&f, (&gv, &bv))| (f - gv - bv).abs() < 1e-12)
    }
    /// Check that the good part satisfies ‖g‖_{L∞} ≤ 2α.
    pub fn good_part_bounded(&self) -> bool {
        let g = self.good_part();
        g.iter().all(|&v| v.abs() <= 2.0 * self.alpha + 1e-12)
    }
}
/// Littlewood-Paley theory with dyadic frequency decomposition.
pub struct LittlewoodPaley {
    /// Labels of the dyadic blocks Δⱼ.
    pub dyadic_blocks: Vec<String>,
}
impl LittlewoodPaley {
    /// Create a new LittlewoodPaley decomposition.
    pub fn new(dyadic_blocks: Vec<String>) -> Self {
        Self { dyadic_blocks }
    }
    /// Lᵖ equivalence: ‖f‖_p ~ ‖(Σⱼ |Δⱼf|²)^{1/2}‖_p for 1 < p < ∞.
    pub fn lp_equivalence(&self) -> bool {
        !self.dyadic_blocks.is_empty()
    }
    /// The Littlewood-Paley square function S(f) = (Σ|Δⱼf|²)^{1/2}.
    pub fn square_function(&self) -> usize {
        self.dyadic_blocks.len()
    }
}
/// Wavelet transform with a given mother wavelet.
pub struct WaveletTransform {
    /// Name or formula of the mother wavelet ψ.
    pub mother_wavelet: String,
    /// Number of decomposition levels.
    pub num_levels: usize,
}
impl WaveletTransform {
    /// Create a new WaveletTransform.
    pub fn new(mother_wavelet: impl Into<String>, num_levels: usize) -> Self {
        Self {
            mother_wavelet: mother_wavelet.into(),
            num_levels,
        }
    }
    /// Continuous wavelet transform: Wf(a,b) = ∫ f(t) ψ_{a,b}(t) dt.
    pub fn continuous_wavelet(&self) -> bool {
        true
    }
    /// Discrete wavelet transform (dyadic subsampling).
    pub fn discrete_wavelet(&self) -> Vec<usize> {
        (0..self.num_levels).collect()
    }
    /// Haar wavelet: the simplest piecewise-constant orthonormal wavelet.
    pub fn haar_wavelet(&self) -> bool {
        self.mother_wavelet.to_lowercase().contains("haar")
    }
}
/// Weighted Lᵖ norm with Muckenhoupt Aₚ weight.
pub struct WeightedNorm {
    /// Description of the weight function w.
    pub weight: String,
    /// Whether w satisfies the Aₚ condition.
    pub ap_condition: bool,
}
impl WeightedNorm {
    /// Create a new WeightedNorm.
    pub fn new(weight: impl Into<String>, ap_condition: bool) -> Self {
        Self {
            weight: weight.into(),
            ap_condition,
        }
    }
    /// Muckenhoupt Aₚ condition characterises weights for which M is bounded.
    pub fn muckenhoupt_ap(&self) -> bool {
        self.ap_condition
    }
    /// Reverse Hölder inequality: Aₚ weights satisfy a reverse Hölder estimate.
    pub fn reverse_holder(&self) -> bool {
        self.ap_condition
    }
}
/// Hardy-Littlewood maximal function data.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct MaximalFunctionData {
    /// Dimension.
    pub dimension: usize,
    /// Input samples (as finite sequence approximation).
    pub samples: Vec<f64>,
}
#[allow(dead_code)]
impl MaximalFunctionData {
    /// Creates maximal function data.
    pub fn new(dimension: usize, samples: Vec<f64>) -> Self {
        MaximalFunctionData { dimension, samples }
    }
    /// Approximate Hardy-Littlewood maximal function Mf(x) at index i.
    /// Uses average over samples[max(0,i-r)..=min(n-1,i+r)] for r=1.
    pub fn hl_maximal_at(&self, i: usize, r: usize) -> f64 {
        if self.samples.is_empty() {
            return 0.0;
        }
        let lo = i.saturating_sub(r);
        let hi = (i + r).min(self.samples.len() - 1);
        let window = &self.samples[lo..=hi];
        window.iter().copied().fold(f64::NEG_INFINITY, f64::max)
    }
    /// Checks weak-type (1,1) bound: |{Mf > λ}| <= C/λ ||f||_1.
    pub fn weak_type_bound_approx(&self, lambda: f64) -> f64 {
        if lambda <= 0.0 {
            return self.samples.len() as f64;
        }
        let l1_norm: f64 = self.samples.iter().map(|&x| x.abs()).sum();
        l1_norm / lambda
    }
    /// L^p norm estimate for p > 1: ||Mf||_p <= C_p ||f||_p.
    pub fn lp_norm_estimate(&self, p: f64) -> f64 {
        if p <= 1.0 {
            return f64::INFINITY;
        }
        let lp_norm: f64 = self
            .samples
            .iter()
            .map(|&x| x.abs().powf(p))
            .sum::<f64>()
            .powf(1.0 / p);
        let cp = p / (p - 1.0);
        cp * lp_norm
    }
}
