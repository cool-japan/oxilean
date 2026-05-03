//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)
use super::functions::*;

/// Amari's α-divergence family on discrete distributions.
///
/// D^(α)(P||Q) = (4 / (1 - α²)) * (1 - Σ p_i^((1+α)/2) * q_i^((1-α)/2))
///
/// Special cases:
/// - α = -1: KL(Q||P)
/// - α =  0: 2 * Hellinger²(P, Q)
/// - α =  1: KL(P||Q)
pub struct AmariDivergence {
    /// Order α (must not be ±1 in the direct formula; use KL limits instead).
    pub alpha: f64,
}
impl AmariDivergence {
    pub fn new(alpha: f64) -> Self {
        Self { alpha }
    }
    /// Compute D^(α)(p || q).
    pub fn compute(&self, p: &[f64], q: &[f64]) -> f64 {
        assert_eq!(p.len(), q.len());
        let a = self.alpha;
        if (a - 1.0).abs() < 1e-8 {
            return p
                .iter()
                .zip(q.iter())
                .map(|(&pi, &qi)| {
                    if pi < 1e-300 {
                        0.0
                    } else if qi < 1e-300 {
                        f64::INFINITY
                    } else {
                        pi * (pi / qi).ln()
                    }
                })
                .sum();
        }
        if (a + 1.0).abs() < 1e-8 {
            return q
                .iter()
                .zip(p.iter())
                .map(|(&qi, &pi)| {
                    if qi < 1e-300 {
                        0.0
                    } else if pi < 1e-300 {
                        f64::INFINITY
                    } else {
                        qi * (qi / pi).ln()
                    }
                })
                .sum();
        }
        let e_p = (1.0 + a) / 2.0;
        let e_q = (1.0 - a) / 2.0;
        let bc: f64 = p
            .iter()
            .zip(q.iter())
            .map(|(&pi, &qi)| {
                if pi < 1e-300 || qi < 1e-300 {
                    0.0
                } else {
                    pi.powf(e_p) * qi.powf(e_q)
                }
            })
            .sum();
        let denom = 1.0 - a * a;
        if denom.abs() < 1e-12 {
            return 0.0;
        }
        (4.0 / denom) * (1.0 - bc)
    }
    /// Jeffreys symmetric divergence J(P||Q) = KL(P||Q) + KL(Q||P).
    pub fn jeffreys(p: &[f64], q: &[f64]) -> f64 {
        let kl_pq: f64 = p
            .iter()
            .zip(q.iter())
            .map(|(&pi, &qi)| {
                if pi < 1e-300 {
                    0.0
                } else if qi < 1e-300 {
                    f64::INFINITY
                } else {
                    pi * (pi / qi).ln()
                }
            })
            .sum();
        let kl_qp: f64 = q
            .iter()
            .zip(p.iter())
            .map(|(&qi, &pi)| {
                if qi < 1e-300 {
                    0.0
                } else if pi < 1e-300 {
                    f64::INFINITY
                } else {
                    qi * (qi / pi).ln()
                }
            })
            .sum();
        kl_pq + kl_qp
    }
    /// Fisher-Rao geodesic distance d_FR(P, Q) = arccos(Σ √(p_i q_i)).
    pub fn fisher_rao_distance(p: &[f64], q: &[f64]) -> f64 {
        let bc: f64 = p
            .iter()
            .zip(q.iter())
            .map(|(&pi, &qi)| (pi * qi).max(0.0).sqrt())
            .sum::<f64>()
            .clamp(-1.0, 1.0);
        bc.acos()
    }
}
/// Multi-user information theory: MAC and broadcast channels.
pub struct MultiUserInfo {
    pub num_senders: usize,
    pub capacity: f64,
}
impl MultiUserInfo {
    pub fn new(num_senders: usize, capacity: f64) -> Self {
        Self {
            num_senders,
            capacity,
        }
    }
    /// Multiple-access channel (MAC) capacity region.
    ///
    /// For a Gaussian MAC with `num_senders` senders each having power `P`
    /// and noise variance 1, the sum-rate capacity is:
    ///   C = log2(1 + num_senders * P)
    ///
    /// Returns the corner points of the MAC capacity region as (rate1, rate2)
    /// pairs (for the 2-sender case) or rates per sender (for n senders).
    pub fn mac_capacity_region(&self) -> Vec<Vec<f64>> {
        let n = self.num_senders.max(1);
        let p = self.capacity;
        let sum_capacity = (1.0 + n as f64 * p).log2();
        let per_sender = sum_capacity / n as f64;
        if n == 2 {
            let c1 = (1.0 + p).log2();
            let c2 = (1.0 + p).log2();
            vec![
                vec![0.0, sum_capacity],
                vec![c1, sum_capacity - c1],
                vec![sum_capacity - c2, c2],
                vec![sum_capacity, 0.0],
            ]
        } else {
            vec![(0..n).map(|_| per_sender).collect()]
        }
    }
    /// Broadcast channel capacity (degraded broadcast channel).
    ///
    /// For a Gaussian broadcast channel with power P and two receivers
    /// with noise variances σ1² < σ2², the superposition coding capacity is:
    ///   C1 = log2(1 + α*P / (σ1² + (1-α)*P))
    ///   C2 = log2(1 + (1-α)*P / σ2²)
    ///
    /// Returns the capacity pairs swept over power allocation α ∈ (0,1).
    pub fn broadcast_capacity(&self) -> Vec<(f64, f64)> {
        let p = self.capacity;
        let sigma1_sq = 1.0;
        let sigma2_sq = 4.0;
        let mut region = Vec::new();
        for k in 1..=19 {
            let alpha = k as f64 / 20.0;
            let c1 = (1.0 + alpha * p / (sigma1_sq + (1.0 - alpha) * p)).log2();
            let c2 = (1.0 + (1.0 - alpha) * p / sigma2_sq).log2();
            region.push((c1.max(0.0), c2.max(0.0)));
        }
        region
    }
}
/// Finite blocklength analysis using the Polyanskiy-Poor-Verdú (2010) normal
/// approximation for memoryless channels.
///
/// The maximum code size M* at blocklength n and error ε satisfies:
///   log M* ≈ n C - √(n V) Φ⁻¹(ε) + (1/2) log n + O(1)
///
/// where C is capacity, V is channel dispersion, and Φ⁻¹ is the inverse normal CDF.
pub struct FiniteBlocklengthAnalyzer {
    /// Channel capacity C (bits per channel use).
    pub capacity: f64,
    /// Channel dispersion V (bits² per channel use).
    pub dispersion: f64,
    /// Target block error probability ε ∈ (0, 1).
    pub epsilon: f64,
}
impl FiniteBlocklengthAnalyzer {
    pub fn new(capacity: f64, dispersion: f64, epsilon: f64) -> Self {
        Self {
            capacity,
            dispersion: dispersion.max(0.0),
            epsilon: epsilon.clamp(1e-10, 1.0 - 1e-10),
        }
    }
    /// Rational approximation to the inverse standard normal CDF (Beasley-Springer-Moro).
    fn inv_normal_cdf(p: f64) -> f64 {
        let p = p.clamp(1e-10, 1.0 - 1e-10);
        let q = if p < 0.5 { p } else { 1.0 - p };
        let t = (-2.0 * q.ln()).sqrt();
        let c = [2.515_517, 0.802_853, 0.010_328_f64];
        let d = [1.432_788, 0.189_269, 0.001_308_f64];
        let num = c[0] + c[1] * t + c[2] * t * t;
        let den = 1.0 + d[0] * t + d[1] * t * t + d[2] * t * t * t;
        let x = t - num / den;
        if p < 0.5 {
            -x
        } else {
            x
        }
    }
    /// Normal approximation: log M*(n, ε) ≈ n C - √(nV) Φ⁻¹(ε).
    pub fn max_log_code_size(&self, n: usize) -> f64 {
        let n = n as f64;
        let qinv = Self::inv_normal_cdf(1.0 - self.epsilon);
        (n * self.capacity - (n * self.dispersion).max(0.0).sqrt() * qinv).max(0.0)
    }
    /// Maximum code size M*(n, ε) = 2^{log M*}.
    pub fn max_code_size(&self, n: usize) -> f64 {
        (2.0_f64).powf(self.max_log_code_size(n))
    }
    /// Effective rate R*(n, ε) = (1/n) log M*(n, ε).
    pub fn effective_rate(&self, n: usize) -> f64 {
        self.max_log_code_size(n) / (n as f64)
    }
    /// Rate gap from capacity Δ(n, ε) = C - R*(n, ε).
    pub fn rate_gap(&self, n: usize) -> f64 {
        (self.capacity - self.effective_rate(n)).max(0.0)
    }
    /// Minimum blocklength n needed to achieve rate R with error ε.
    pub fn min_blocklength(&self, target_rate: f64) -> usize {
        if target_rate >= self.capacity - 1e-12 {
            return usize::MAX;
        }
        let qinv = Self::inv_normal_cdf(1.0 - self.epsilon);
        let delta = self.capacity - target_rate;
        let sqrt_v = self.dispersion.sqrt();
        let n_est = (sqrt_v * qinv / delta).powi(2);
        (n_est.max(1.0) as usize) + 1
    }
}
/// Rényi entropy of order α for a discrete probability distribution.
///
/// H_α(X) = (1/(1-α)) * log2(Σ p_i^α)
///
/// Special cases:
/// - α → 0: H_0(X) = log2(|supp X|) (Hartley / max-entropy)
/// - α → 1: H_1(X) = H(X) (Shannon entropy, computed via l'Hôpital limit)
/// - α → ∞: H_∞(X) = -log2(max p_i) (min-entropy)
pub struct RenyiEntropy {
    /// Order parameter α (must be ≥ 0, ≠ 1).
    pub alpha: f64,
}
impl RenyiEntropy {
    pub fn new(alpha: f64) -> Self {
        Self { alpha }
    }
    /// Compute H_α(probs).
    pub fn compute(&self, probs: &[f64]) -> f64 {
        let alpha = self.alpha;
        if (alpha - 1.0).abs() < 1e-10 {
            return -probs
                .iter()
                .filter(|&&p| p > 1e-300)
                .map(|&p| p * p.log2())
                .sum::<f64>();
        }
        if alpha < 1e-10 {
            let support = probs.iter().filter(|&&p| p > 1e-300).count();
            return (support as f64).log2();
        }
        let sum: f64 = probs.iter().map(|&p| p.powf(alpha)).sum();
        if sum < 1e-300 {
            return 0.0;
        }
        sum.log2() / (1.0 - alpha)
    }
    /// Compute min-entropy H_∞ = -log2(max p_i).
    pub fn min_entropy(probs: &[f64]) -> f64 {
        let p_max = probs.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        if p_max <= 0.0 {
            return 0.0;
        }
        -p_max.log2()
    }
    /// Compute max-entropy H_0 = log2(|supp X|).
    pub fn max_entropy(probs: &[f64]) -> f64 {
        let support = probs.iter().filter(|&&p| p > 1e-300).count();
        (support as f64).log2()
    }
    /// Compute smooth min-entropy H_∞^ε by trimming the largest probabilities
    /// within an ε ball (L1 ball heuristic).
    ///
    /// Removes probability mass greedily until the remaining L1 distance to the
    /// original distribution exceeds ε, then computes min-entropy.
    pub fn smooth_min_entropy(probs: &[f64], epsilon: f64) -> f64 {
        let mut sorted: Vec<f64> = probs.to_vec();
        sorted.sort_by(|a, b| b.partial_cmp(a).unwrap_or(std::cmp::Ordering::Equal));
        let mut trimmed = sorted.clone();
        let mut removed = 0.0f64;
        let eps = epsilon.max(0.0);
        for p in trimmed.iter_mut() {
            if removed + *p <= eps {
                removed += *p;
                *p = 0.0;
            } else if removed < eps {
                let cut = eps - removed;
                removed = eps;
                *p -= cut;
                break;
            } else {
                break;
            }
        }
        Self::min_entropy(&trimmed)
    }
}
/// Rate-distortion theory: finding the minimum rate required to encode a
/// source with a given average distortion constraint.
///
/// `source_dist\[i\]` = P(X = i) for a discrete memoryless source.
/// `distortion_limit` = D, the maximum allowable expected distortion.
pub struct RateDistortion {
    pub source_dist: Vec<f64>,
    pub distortion_limit: f64,
}
impl RateDistortion {
    pub fn new(source_dist: Vec<f64>, distortion_limit: f64) -> Self {
        Self {
            source_dist,
            distortion_limit,
        }
    }
    /// Compute the rate-distortion function R(D) for a binary symmetric source
    /// with Hamming distortion.
    ///
    /// For a uniform binary source: R(D) = 1 - H_b(D) for 0 ≤ D ≤ 0.5,
    /// where H_b is the binary entropy function.
    pub fn rate_distortion_function(&self) -> f64 {
        let n = self.source_dist.len();
        let d = self.distortion_limit.max(0.0);
        if n == 2 {
            let h_d = binary_entropy(d);
            (1.0 - h_d).max(0.0)
        } else {
            let h_x = entropy(&self.source_dist);
            let approx = h_x - (n as f64 * d + 1.0).log2();
            approx.max(0.0)
        }
    }
    /// Blahut-Arimoto algorithm: iterative computation of R(D).
    ///
    /// Computes the rate-distortion function via the parametric method.
    /// Uses a quadratic distortion measure d(x, x̂) = (x - x̂)².
    /// `n_reconstructions` = number of reconstruction levels.
    /// Returns (rate, achieved_distortion) pairs along the R(D) curve.
    pub fn blahut_arimoto(&self) -> Vec<(f64, f64)> {
        let n = self.source_dist.len();
        if n == 0 {
            return vec![];
        }
        let mut result = Vec::new();
        for s_exp in 0..20 {
            let s = -(s_exp as f64 + 1.0) * 0.5;
            let m = n;
            let d_mat: Vec<Vec<f64>> = (0..n)
                .map(|i| {
                    (0..m)
                        .map(|j| {
                            let diff = (i as f64) - (j as f64);
                            diff * diff
                        })
                        .collect()
                })
                .collect();
            let mut r: Vec<f64> = vec![1.0 / m as f64; m];
            for _ in 0..200 {
                let q: Vec<Vec<f64>> = (0..n)
                    .map(|i| {
                        let unnorm: Vec<f64> =
                            (0..m).map(|j| r[j] * (s * d_mat[i][j]).exp()).collect();
                        let z: f64 = unnorm.iter().sum();
                        if z < 1e-300 {
                            vec![1.0 / m as f64; m]
                        } else {
                            unnorm.iter().map(|v| v / z).collect()
                        }
                    })
                    .collect();
                let r_new: Vec<f64> = (0..m)
                    .map(|j| (0..n).map(|i| self.source_dist[i] * q[i][j]).sum())
                    .collect();
                r = r_new;
                let sum: f64 = r.iter().sum();
                if sum > 1e-300 {
                    r.iter_mut().for_each(|v| *v /= sum);
                }
            }
            let q: Vec<Vec<f64>> = (0..n)
                .map(|i| {
                    let unnorm: Vec<f64> = (0..m).map(|j| r[j] * (s * d_mat[i][j]).exp()).collect();
                    let z: f64 = unnorm.iter().sum();
                    if z < 1e-300 {
                        vec![1.0 / m as f64; m]
                    } else {
                        unnorm.iter().map(|v| v / z).collect()
                    }
                })
                .collect();
            let dist: f64 = (0..n)
                .map(|i| {
                    (0..m)
                        .map(|j| self.source_dist[i] * q[i][j] * d_mat[i][j])
                        .sum::<f64>()
                })
                .sum();
            let mut rate = 0.0f64;
            for i in 0..n {
                for j in 0..m {
                    let p_xj = q[i][j];
                    if p_xj > 1e-300 && self.source_dist[i] > 1e-300 && r[j] > 1e-300 {
                        rate += self.source_dist[i] * p_xj * (p_xj / r[j]).log2();
                    }
                }
            }
            result.push((rate.max(0.0), dist.max(0.0)));
        }
        result
    }
}
/// Chernoff information between two discrete distributions P and Q.
///
/// C(P, Q) = -min_{0 ≤ λ ≤ 1} log Σ_x p_x^λ q_x^(1-λ)
///
/// Represents the optimal symmetric error exponent: both miss and false-alarm
/// probabilities decay as exp(-n C(P,Q)).
pub struct ChernoffInformation {
    pub p: Vec<f64>,
    pub q: Vec<f64>,
}
impl ChernoffInformation {
    pub fn new(p: Vec<f64>, q: Vec<f64>) -> Self {
        Self { p, q }
    }
    /// Compute the Chernoff exponent e(λ) = -log2(Σ p_i^λ q_i^(1-λ)).
    pub fn exponent(&self, lambda: f64) -> f64 {
        let lam = lambda.clamp(0.0, 1.0);
        let s: f64 = self
            .p
            .iter()
            .zip(self.q.iter())
            .map(|(&pi, &qi)| {
                if pi <= 0.0 || qi <= 0.0 {
                    0.0
                } else {
                    pi.powf(lam) * qi.powf(1.0 - lam)
                }
            })
            .sum();
        if s <= 0.0 {
            return f64::INFINITY;
        }
        -s.log2()
    }
    /// Compute the Chernoff information C(P, Q) = max_{λ ∈ \[0,1\]} e(λ)
    /// via golden-section search.
    pub fn chernoff_information(&self) -> f64 {
        let phi = (5.0f64.sqrt() - 1.0) / 2.0;
        let mut a = 0.0f64;
        let mut b = 1.0f64;
        for _ in 0..60 {
            let c = b - phi * (b - a);
            let d = a + phi * (b - a);
            if self.exponent(c) < self.exponent(d) {
                a = c;
            } else {
                b = d;
            }
        }
        self.exponent((a + b) / 2.0).max(0.0)
    }
    /// Rényi divergence D_α(P||Q) = (1/(α-1)) log2 Σ p_i^α q_i^(1-α).
    pub fn renyi_divergence(&self, alpha: f64) -> f64 {
        if (alpha - 1.0).abs() < 1e-10 {
            return self
                .p
                .iter()
                .zip(self.q.iter())
                .filter(|(&pi, _)| pi > 1e-300)
                .map(|(&pi, &qi)| {
                    if qi <= 0.0 {
                        f64::INFINITY
                    } else {
                        pi * (pi / qi).log2()
                    }
                })
                .sum();
        }
        let s: f64 = self
            .p
            .iter()
            .zip(self.q.iter())
            .map(|(&pi, &qi)| {
                if pi <= 0.0 || qi <= 0.0 {
                    0.0
                } else {
                    pi.powf(alpha) * qi.powf(1.0 - alpha)
                }
            })
            .sum();
        if s <= 0.0 {
            return f64::INFINITY;
        }
        s.log2() / (alpha - 1.0)
    }
}
/// Natural gradient descent on the statistical manifold of a parametric family.
///
/// Given a log-likelihood gradient and the Fisher information matrix (FIM),
/// computes the natural gradient update: θ ← θ + η F⁻¹ ∇L.
///
/// The FIM is stored as a flat row-major vector of size dim × dim.
pub struct NaturalGradientDescent {
    /// Dimension of the parameter vector θ.
    pub dim: usize,
    /// Fisher information matrix (dim × dim), row-major.
    pub fim: Vec<f64>,
    /// Step size η.
    pub step_size: f64,
}
impl NaturalGradientDescent {
    pub fn new(dim: usize, fim: Vec<f64>, step_size: f64) -> Self {
        Self {
            dim,
            fim,
            step_size,
        }
    }
    /// Solve F x = g via Cholesky-like regularised inversion (LDLᵀ approximation).
    ///
    /// For simplicity, uses gradient descent on the linear system with
    /// Tikhonov regularisation: (F + λ I) x = g, λ = 1e-6.
    fn fim_solve(&self, g: &[f64]) -> Vec<f64> {
        let d = self.dim;
        let lambda = 1e-6;
        let mut x = vec![0.0f64; d];
        for _iter in 0..200 {
            let mut r = vec![0.0f64; d];
            for i in 0..d {
                let mut ax_i = lambda * x[i];
                for j in 0..d {
                    ax_i += self.fim[i * d + j] * x[j];
                }
                r[i] = g[i] - ax_i;
            }
            let r_norm_sq: f64 = r.iter().map(|&v| v * v).sum();
            if r_norm_sq < 1e-20 {
                break;
            }
            let mut fr = vec![0.0f64; d];
            for i in 0..d {
                fr[i] = lambda * r[i];
                for j in 0..d {
                    fr[i] += self.fim[i * d + j] * r[j];
                }
            }
            let r_fr: f64 = r.iter().zip(fr.iter()).map(|(&a, &b)| a * b).sum();
            if r_fr.abs() < 1e-20 {
                break;
            }
            let alpha = r_norm_sq / r_fr;
            for i in 0..d {
                x[i] += alpha * r[i];
            }
        }
        x
    }
    /// Apply one natural gradient step: returns updated parameters θ_new = θ + η F⁻¹ ∇L.
    pub fn step(&self, theta: &[f64], grad: &[f64]) -> Vec<f64> {
        assert_eq!(theta.len(), self.dim);
        assert_eq!(grad.len(), self.dim);
        let nat_grad = self.fim_solve(grad);
        theta
            .iter()
            .zip(nat_grad.iter())
            .map(|(&t, &ng)| t + self.step_size * ng)
            .collect()
    }
    /// Compute the Riemannian gradient norm ‖∇L‖_F = √(∇Lᵀ F⁻¹ ∇L).
    pub fn grad_norm(&self, grad: &[f64]) -> f64 {
        let nat_grad = self.fim_solve(grad);
        grad.iter()
            .zip(nat_grad.iter())
            .map(|(&g, &ng)| g * ng)
            .sum::<f64>()
            .max(0.0)
            .sqrt()
    }
}
/// Computes achievable rate pairs for distributed (Slepian-Wolf) source coding
/// of two correlated binary sources X and Y.
///
/// The source model: X ~ Bernoulli(p_x), Y = X XOR Z where Z ~ Bernoulli(p_z).
pub struct DistributedSourceCoder {
    /// Marginal probability of X = 1.
    pub p_x: f64,
    /// Crossover probability P(Y ≠ X).
    pub p_z: f64,
}
impl DistributedSourceCoder {
    pub fn new(p_x: f64, p_z: f64) -> Self {
        Self {
            p_x: p_x.clamp(1e-12, 1.0 - 1e-12),
            p_z: p_z.clamp(1e-12, 1.0 - 1e-12),
        }
    }
    fn binary_entropy_local(p: f64) -> f64 {
        let p = p.clamp(1e-12, 1.0 - 1e-12);
        -p * p.log2() - (1.0 - p) * (1.0 - p).log2()
    }
    /// H(X) — marginal entropy of X.
    pub fn h_x(&self) -> f64 {
        Self::binary_entropy_local(self.p_x)
    }
    /// H(Y) — marginal entropy of Y = X XOR Z.
    /// P(Y=1) = p_x (1-p_z) + (1-p_x) p_z.
    pub fn h_y(&self) -> f64 {
        let p_y = self.p_x * (1.0 - self.p_z) + (1.0 - self.p_x) * self.p_z;
        Self::binary_entropy_local(p_y)
    }
    /// H(X|Y) = H(Z) = H_b(p_z) (correlation model).
    pub fn h_x_given_y(&self) -> f64 {
        Self::binary_entropy_local(self.p_z)
    }
    /// H(Y|X) = H_b(p_z).
    pub fn h_y_given_x(&self) -> f64 {
        Self::binary_entropy_local(self.p_z)
    }
    /// H(X, Y) = H(X) + H(Y|X).
    pub fn h_xy(&self) -> f64 {
        self.h_x() + self.h_y_given_x()
    }
    /// I(X; Y) = H(X) - H(X|Y).
    pub fn mutual_information(&self) -> f64 {
        (self.h_x() - self.h_x_given_y()).max(0.0)
    }
    /// Slepian-Wolf corner points:
    /// A = (H(X), H(Y|X))  — X encoded at full rate, Y at conditional rate
    /// B = (H(X|Y), H(Y))  — X at conditional rate, Y at full rate
    pub fn slepian_wolf_corner_points(&self) -> [(f64, f64); 2] {
        [
            (self.h_x(), self.h_y_given_x()),
            (self.h_x_given_y(), self.h_y()),
        ]
    }
    /// Check whether a proposed rate pair (r_x, r_y) is in the Slepian-Wolf region.
    pub fn is_achievable(&self, r_x: f64, r_y: f64) -> bool {
        r_x >= self.h_x_given_y() - 1e-9
            && r_y >= self.h_y_given_x() - 1e-9
            && r_x + r_y >= self.h_xy() - 1e-9
    }
}
/// Quantum information theory for a noisy quantum channel.
///
/// `entanglement` represents the coherent information or entanglement
/// fidelity parameter characterising the channel noise level.
pub struct QuantumInfo {
    pub entanglement: f64,
}
impl QuantumInfo {
    pub fn new(entanglement: f64) -> Self {
        Self { entanglement }
    }
    /// Quantum channel capacity (heuristic formula).
    ///
    /// For a depolarizing channel with parameter p (entanglement parameter),
    /// the quantum capacity is approximately:
    ///   Q = max(0, 1 - H_b(p) - p * log2(3)) when p < 1/4,
    /// where H_b is the binary entropy function.
    pub fn quantum_capacity(&self) -> f64 {
        let p = self.entanglement.clamp(0.0, 1.0);
        if p >= 0.25 {
            return 0.0;
        }
        let hb = binary_entropy(p);
        let depol_term = if p > 1e-12 { p * (3.0f64).log2() } else { 0.0 };
        (1.0 - hb - depol_term).max(0.0)
    }
    /// Hashing bound on the quantum capacity.
    ///
    /// Q ≤ I_c = coherent information = S(B) - S(BE)
    /// For a depolarizing channel with error probability p:
    ///   I_c = 1 - H_b(p) - p * log2(3)    (same as quantum_capacity for depolarizing)
    pub fn hashing_bound(&self) -> f64 {
        self.quantum_capacity()
    }
    /// Coherent information of the channel.
    ///
    /// I_c(ρ, N) = S(N(ρ)) - S((N ⊗ I)(|ψ⟩⟨ψ|))
    ///
    /// For a qubit channel parameterised by entanglement fidelity F = 1 - e,
    /// where e = `entanglement` (error probability):
    ///   S(output) ≈ H_b(e)
    ///   S(environment) ≈ H_b(e) + e * log2(3)
    ///   I_c ≈ 1 - 2 * H_b(e) (simplified model)
    pub fn coherent_information(&self) -> f64 {
        let e = self.entanglement.clamp(0.0, 1.0);
        let hb = binary_entropy(e);
        (1.0 - 2.0 * hb).max(0.0)
    }
}
/// Supported f-divergence variants.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FDivergenceKind {
    /// KL divergence: f(t) = t log t
    KL,
    /// Reverse KL: f(t) = -log t
    ReverseKL,
    /// Total variation: f(t) = |t - 1| / 2
    TotalVariation,
    /// Squared Hellinger: f(t) = (√t - 1)²
    Hellinger,
    /// χ²-divergence: f(t) = (t - 1)²
    ChiSquared,
    /// α-divergence: f(t) = (t^α - 1) / (α(α-1))
    AlphaDivergence(f64),
}
/// Estimates quantum (von Neumann) entropy S(ρ) = -Tr(ρ log ρ) from a density
/// matrix represented as a flat vector of eigenvalues.
///
/// For a full density matrix you would first diagonalize it; here we accept
/// pre-computed eigenvalues or diagonal entries.
pub struct QuantumEntropyEstimator {
    /// Eigenvalues λ_i ≥ 0, Σ λ_i = 1.
    pub eigenvalues: Vec<f64>,
}
impl QuantumEntropyEstimator {
    pub fn new(eigenvalues: Vec<f64>) -> Self {
        Self { eigenvalues }
    }
    /// von Neumann entropy S(ρ) = -Σ λ_i log2(λ_i) (nats converted to bits).
    pub fn von_neumann_entropy(&self) -> f64 {
        -self
            .eigenvalues
            .iter()
            .filter(|&&l| l > 1e-300)
            .map(|&l| l * l.log2())
            .sum::<f64>()
    }
    /// Quantum Rényi entropy of order α: S_α(ρ) = (1/(1-α)) log2 Tr(ρ^α).
    pub fn renyi_entropy(&self, alpha: f64) -> f64 {
        let re = RenyiEntropy::new(alpha);
        re.compute(&self.eigenvalues)
    }
    /// Quantum min-entropy: -log2(λ_max).
    pub fn min_entropy(&self) -> f64 {
        RenyiEntropy::min_entropy(&self.eigenvalues)
    }
    /// Purity Tr(ρ²) = Σ λ_i².
    pub fn purity(&self) -> f64 {
        self.eigenvalues.iter().map(|&l| l * l).sum()
    }
    /// Check whether the state is approximately pure (purity ≥ 1 - tol).
    pub fn is_pure(&self, tol: f64) -> bool {
        (self.purity() - 1.0).abs() <= tol
    }
}
/// Simulates common single-qubit quantum channels acting on a qubit density
/// matrix represented as a 2×2 real matrix \[a, b; c, d\] stored as \[a, b, c, d\].
pub struct QuantumChannelSimulator {
    /// Noise parameter p ∈ \[0, 1\].
    pub p: f64,
}
impl QuantumChannelSimulator {
    pub fn new(p: f64) -> Self {
        Self {
            p: p.clamp(0.0, 1.0),
        }
    }
    /// Apply the depolarizing channel N_dep(ρ) = (1-p)ρ + (p/3)(XρX + YρY + ZρZ)
    ///        = (1 - 4p/3) ρ + (4p/3) * I/2.
    ///
    /// For a 2×2 density matrix \[ρ₀₀, ρ₀₁; ρ₁₀, ρ₁₁\] stored as \[ρ₀₀, ρ₀₁, ρ₁₀, ρ₁₁\].
    pub fn depolarizing(&self, rho: &[f64]) -> Vec<f64> {
        assert_eq!(
            rho.len(),
            4,
            "Expected 2×2 density matrix as 4-element slice"
        );
        let q = 1.0 - 4.0 * self.p / 3.0;
        let mixed = 0.5;
        vec![
            q * rho[0] + (1.0 - q) * mixed,
            q * rho[1],
            q * rho[2],
            q * rho[3] + (1.0 - q) * mixed,
        ]
    }
    /// Apply the amplitude-damping channel with decay parameter γ = p.
    ///
    /// Kraus operators: K₀ = [\[1, 0\], \[0, √(1-γ)\]], K₁ = [\[0, √γ\], \[0, 0\]].
    pub fn amplitude_damping(&self, rho: &[f64]) -> Vec<f64> {
        assert_eq!(
            rho.len(),
            4,
            "Expected 2×2 density matrix as 4-element slice"
        );
        let gamma = self.p;
        let sqrt_1mg = (1.0 - gamma).max(0.0).sqrt();
        vec![
            rho[0] + gamma * rho[3],
            rho[1] * sqrt_1mg,
            rho[2] * sqrt_1mg,
            rho[3] * (1.0 - gamma),
        ]
    }
    /// Fidelity between two 2×2 density matrices: F(ρ, σ) = Tr(ρ σ) for pure states.
    /// For mixed states this is a lower bound; exact fidelity needs matrix square roots.
    pub fn trace_overlap(rho: &[f64], sigma: &[f64]) -> f64 {
        assert_eq!(rho.len(), 4);
        assert_eq!(sigma.len(), 4);
        rho[0] * sigma[0] + rho[1] * sigma[2] + rho[2] * sigma[1] + rho[3] * sigma[3]
    }
    /// Compute the purity Tr(ρ²) for a 2×2 density matrix.
    pub fn purity(rho: &[f64]) -> f64 {
        Self::trace_overlap(rho, rho)
    }
}
/// Generic f-divergence D_f(P||Q) = Σ q_i f(p_i/q_i).
pub struct FDivergence {
    pub kind: FDivergenceKind,
}
impl FDivergence {
    pub fn new(kind: FDivergenceKind) -> Self {
        Self { kind }
    }
    /// Compute D_f(p || q).
    pub fn compute(&self, p: &[f64], q: &[f64]) -> f64 {
        assert_eq!(p.len(), q.len(), "p and q must have the same length");
        p.iter()
            .zip(q.iter())
            .map(|(&pi, &qi)| {
                if qi < 1e-300 {
                    if pi < 1e-300 {
                        0.0
                    } else {
                        f64::INFINITY
                    }
                } else {
                    let t = pi / qi;
                    qi * self.f(t)
                }
            })
            .fold(0.0f64, |a, x| if x.is_nan() { a } else { a + x })
    }
    fn f(&self, t: f64) -> f64 {
        match self.kind {
            FDivergenceKind::KL => {
                if t <= 0.0 {
                    0.0
                } else {
                    t * t.ln()
                }
            }
            FDivergenceKind::ReverseKL => {
                if t <= 0.0 {
                    f64::INFINITY
                } else {
                    -t.ln()
                }
            }
            FDivergenceKind::TotalVariation => (t - 1.0).abs() / 2.0,
            FDivergenceKind::Hellinger => {
                let s = t.max(0.0).sqrt();
                (s - 1.0) * (s - 1.0)
            }
            FDivergenceKind::ChiSquared => (t - 1.0) * (t - 1.0),
            FDivergenceKind::AlphaDivergence(alpha) => {
                if (alpha - 1.0).abs() < 1e-10 {
                    if t <= 0.0 {
                        0.0
                    } else {
                        t * t.ln()
                    }
                } else if alpha.abs() < 1e-10 {
                    if t <= 0.0 {
                        f64::INFINITY
                    } else {
                        -t.ln()
                    }
                } else {
                    (t.powf(alpha) - 1.0) / (alpha * (alpha - 1.0))
                }
            }
        }
    }
    /// Total variation TV(P, Q) = (1/2) Σ |p_i - q_i|.
    pub fn total_variation(p: &[f64], q: &[f64]) -> f64 {
        p.iter()
            .zip(q.iter())
            .map(|(&pi, &qi)| (pi - qi).abs())
            .sum::<f64>()
            / 2.0
    }
    /// Hellinger distance H²(P, Q) = 1 - Σ √(p_i q_i) (Bhattacharyya coefficient form).
    pub fn hellinger_squared(p: &[f64], q: &[f64]) -> f64 {
        let bc: f64 = p
            .iter()
            .zip(q.iter())
            .map(|(&pi, &qi)| (pi * qi).sqrt())
            .sum();
        (1.0 - bc).max(0.0)
    }
    /// χ²-divergence: Σ (p_i - q_i)² / q_i.
    pub fn chi_squared(p: &[f64], q: &[f64]) -> f64 {
        p.iter()
            .zip(q.iter())
            .map(|(&pi, &qi)| {
                if qi < 1e-300 {
                    if pi < 1e-300 {
                        0.0
                    } else {
                        f64::INFINITY
                    }
                } else {
                    (pi - qi) * (pi - qi) / qi
                }
            })
            .sum()
    }
}
/// Smooth min-entropy H_∞^ε(X): the min-entropy maximized over distributions
/// within an ε-ball (in purified-distance / half L1) around P.
///
/// This structure wraps the distribution and provides computation via a
/// greedy ball-trimming procedure matching the Rényi family approach.
pub struct SmoothMinEntropy {
    pub probs: Vec<f64>,
    /// Smoothing parameter ε ∈ [0, 1).
    pub epsilon: f64,
}
impl SmoothMinEntropy {
    pub fn new(probs: Vec<f64>, epsilon: f64) -> Self {
        Self { probs, epsilon }
    }
    /// Compute smooth min-entropy H_∞^ε via greedy mass removal.
    pub fn compute(&self) -> f64 {
        RenyiEntropy::smooth_min_entropy(&self.probs, self.epsilon)
    }
    /// Smooth max-entropy H_0^ε: log2 of effective support size after ε-trimming
    /// the smallest probabilities.
    pub fn smooth_max_entropy(&self) -> f64 {
        let mut sorted: Vec<f64> = self.probs.to_vec();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        let mut removed = 0.0f64;
        let eps = self.epsilon.max(0.0);
        let mut remaining_support = sorted.len();
        for &p in &sorted {
            if removed + p <= eps {
                removed += p;
                remaining_support -= 1;
            } else {
                break;
            }
        }
        (remaining_support as f64).log2().max(0.0)
    }
    /// One-shot accessible information compression rate (smooth-min-entropy bound).
    ///
    /// The ε-error compression rate equals H_∞^ε(X) bits in the one-shot regime.
    pub fn compression_rate(&self) -> f64 {
        self.compute()
    }
}
/// Network coding: max-flow and achievable rate computation.
///
/// The network is described as an adjacency list encoded in the `graph` field.
/// `rate` is the target information flow rate (bits/channel use).
pub struct NetworkCoding {
    pub graph: String,
    pub rate: f64,
    /// Capacity of each edge, parallel to edge list
    pub edge_capacities: Vec<f64>,
    pub num_nodes: usize,
    pub source: usize,
    pub sink: usize,
}
impl NetworkCoding {
    pub fn new(graph: String, rate: f64) -> Self {
        Self {
            graph,
            rate,
            edge_capacities: Vec::new(),
            num_nodes: 0,
            source: 0,
            sink: 0,
        }
    }
    pub fn with_graph(
        graph: String,
        rate: f64,
        num_nodes: usize,
        source: usize,
        sink: usize,
        edge_capacities: Vec<f64>,
    ) -> Self {
        Self {
            graph,
            rate,
            edge_capacities,
            num_nodes,
            source,
            sink,
        }
    }
    /// Max-flow / min-cut via Ford-Fulkerson (BFS augmenting paths).
    ///
    /// The graph is encoded as an edge list in `self.graph` as
    /// "u,v,cap;..." (e.g. "0,1,3;1,2,2").
    /// Returns the maximum flow value.
    pub fn max_flow_min_cut(&self) -> f64 {
        let mut adjacency: Vec<Vec<(usize, f64)>> = vec![Vec::new(); self.num_nodes.max(2)];
        for edge_str in self.graph.split(';') {
            let parts: Vec<&str> = edge_str.trim().split(',').collect();
            if parts.len() >= 3 {
                if let (Ok(u), Ok(v), Ok(cap)) = (
                    parts[0].trim().parse::<usize>(),
                    parts[1].trim().parse::<usize>(),
                    parts[2].trim().parse::<f64>(),
                ) {
                    if u < adjacency.len() && v < adjacency.len() {
                        adjacency[u].push((v, cap));
                    }
                }
            }
        }
        if self.num_nodes == 0 || adjacency.is_empty() {
            return self.rate;
        }
        let n = adjacency.len();
        let mut cap = vec![vec![0.0f64; n]; n];
        for u in 0..n {
            for &(v, c) in &adjacency[u] {
                cap[u][v] += c;
            }
        }
        let s = self.source.min(n - 1);
        let t = self.sink.min(n - 1);
        let mut flow = 0.0;
        loop {
            let mut parent = vec![usize::MAX; n];
            parent[s] = s;
            let mut queue = std::collections::VecDeque::new();
            queue.push_back(s);
            while let Some(u) = queue.pop_front() {
                for v in 0..n {
                    if parent[v] == usize::MAX && cap[u][v] > 1e-12 {
                        parent[v] = u;
                        if v == t {
                            break;
                        }
                        queue.push_back(v);
                    }
                }
            }
            if parent[t] == usize::MAX {
                break;
            }
            let mut path_flow = f64::INFINITY;
            let mut v = t;
            while v != s {
                let u = parent[v];
                path_flow = path_flow.min(cap[u][v]);
                v = u;
            }
            let mut v = t;
            while v != s {
                let u = parent[v];
                cap[u][v] -= path_flow;
                cap[v][u] += path_flow;
                v = u;
            }
            flow += path_flow;
        }
        flow
    }
    /// Check whether the target `rate` is achievable by network coding.
    ///
    /// By the max-flow min-cut theorem for network coding, a rate R is
    /// achievable if and only if R ≤ max-flow from source to sink.
    pub fn achievable_rates(&self) -> bool {
        self.rate <= self.max_flow_min_cut() + 1e-12
    }
}
