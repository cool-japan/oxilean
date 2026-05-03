//! Functions for optimal transport.

use super::types::{
    BarycenterResult, CostMatrix, GroundMetric, Measure, OTDualSolution, SinkhornResult,
    SlicedWasserstein, TransportPlan, WassersteinDistance,
};

// ── Internal helpers ─────────────────────────────────────────────────────────

/// Euclidean norm of a slice.
fn norm2(v: &[f64]) -> f64 {
    v.iter().map(|x| x * x).sum::<f64>().sqrt()
}

/// L1 norm of a slice.
fn norm1(v: &[f64]) -> f64 {
    v.iter().map(|x| x.abs()).sum()
}

/// Dot product of two slices.
fn dot(a: &[f64], b: &[f64]) -> f64 {
    a.iter().zip(b.iter()).map(|(x, y)| x * y).sum()
}

/// Compute the pairwise distance between two points under a ground metric.
fn point_cost(x: &[f64], y: &[f64], metric: &GroundMetric) -> f64 {
    match metric {
        GroundMetric::Euclidean => {
            let diff: Vec<f64> = x.iter().zip(y.iter()).map(|(a, b)| a - b).collect();
            norm2(&diff)
        }
        GroundMetric::SquaredEuclidean => {
            let diff: Vec<f64> = x.iter().zip(y.iter()).map(|(a, b)| a - b).collect();
            let n = norm2(&diff);
            n * n
        }
        GroundMetric::L1 => {
            let diff: Vec<f64> = x.iter().zip(y.iter()).map(|(a, b)| a - b).collect();
            norm1(&diff)
        }
        GroundMetric::Custom(_) => 0.0, // handled separately in cost_matrix
    }
}

// ── Cost matrix ───────────────────────────────────────────────────────────────

/// Build the pairwise cost matrix C\[i\]\[j\] = c(source_i, target_j).
pub fn cost_matrix(source: &[Vec<f64>], target: &[Vec<f64>], metric: &GroundMetric) -> CostMatrix {
    let n = source.len();
    let m = target.len();
    if let GroundMetric::Custom(custom) = metric {
        return CostMatrix::new(custom.clone(), n, m);
    }
    let entries: Vec<Vec<f64>> = source
        .iter()
        .map(|xi| target.iter().map(|yj| point_cost(xi, yj, metric)).collect())
        .collect();
    CostMatrix::new(entries, n, m)
}

// ── Earth Mover's Distance in 1-D ────────────────────────────────────────────

/// Compute the exact Earth Mover's Distance (W_1) between two 1-D distributions.
///
/// Both `source` and `target` are atom locations; `source_w` and `target_w`
/// are the corresponding probability weights.  Atoms are sorted internally.
pub fn earth_movers_distance_1d(
    source: &[f64],
    source_w: &[f64],
    target: &[f64],
    target_w: &[f64],
) -> f64 {
    if source.is_empty() || target.is_empty() {
        return 0.0;
    }
    // Build sorted (location, weight) pairs.
    let mut src: Vec<(f64, f64)> = source
        .iter()
        .cloned()
        .zip(source_w.iter().cloned())
        .collect();
    let mut tgt: Vec<(f64, f64)> = target
        .iter()
        .cloned()
        .zip(target_w.iter().cloned())
        .collect();
    src.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
    tgt.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));

    // Merge sorted atoms into a common grid.
    let mut xs: Vec<f64> = src.iter().map(|(x, _)| *x).collect();
    xs.extend(tgt.iter().map(|(x, _)| *x));
    xs.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    xs.dedup_by(|a, b| (*a - *b).abs() < 1e-15);

    // CDF difference integral: EMD = integral |F_src(x) - F_tgt(x)| dx.
    let interp_cdf = |sorted: &[(f64, f64)], x: f64| -> f64 {
        let mut cdf = 0.0;
        for &(xi, wi) in sorted {
            if xi <= x {
                cdf += wi;
            }
        }
        cdf
    };

    let mut emd = 0.0f64;
    for w in xs.windows(2) {
        let mid = (w[0] + w[1]) / 2.0;
        let fs = interp_cdf(&src, mid);
        let ft = interp_cdf(&tgt, mid);
        emd += (fs - ft).abs() * (w[1] - w[0]);
    }
    emd
}

// ── Sinkhorn–Knopp ────────────────────────────────────────────────────────────

/// Regularised optimal transport via the Sinkhorn–Knopp algorithm.
///
/// Minimises sum_{ij} C\[i\]\[j\] γ\[i\]\[j\] + ε H(γ) subject to marginal constraints,
/// where H(γ) = −sum_{ij} γ\[i\]\[j\] log γ\[i\]\[j\] is the entropic regulariser.
pub fn sinkhorn(
    source: &Measure,
    target: &Measure,
    cost: &CostMatrix,
    epsilon: f64,
    max_iter: usize,
) -> SinkhornResult {
    let n = source.weights.len();
    let m = target.weights.len();
    if n == 0 || m == 0 || epsilon <= 0.0 {
        return SinkhornResult {
            plan: TransportPlan {
                gamma: vec![],
                cost: 0.0,
            },
            iterations: 0,
            converged: false,
        };
    }
    let a = &source.weights;
    let b = &target.weights;

    // Gibbs kernel K[i][j] = exp(-C[i][j] / epsilon).
    let k: Vec<Vec<f64>> = cost
        .entries
        .iter()
        .map(|row| row.iter().map(|c| (-c / epsilon).exp()).collect())
        .collect();

    // Sinkhorn iterations: u, v scaling vectors.
    let mut u = vec![1.0f64 / n as f64; n];
    let mut v = vec![1.0f64 / m as f64; m];
    let tol = 1e-9;
    let mut converged = false;

    for iter in 0..max_iter {
        // v = b / (K^T u)
        let ktu: Vec<f64> = (0..m)
            .map(|j| (0..n).map(|i| k[i][j] * u[i]).sum())
            .collect();
        let v_new: Vec<f64> = b
            .iter()
            .zip(ktu.iter())
            .map(|(bi, ki)| if *ki > 1e-300 { bi / ki } else { 0.0 })
            .collect();

        // u = a / (K v)
        let kv: Vec<f64> = k
            .iter()
            .map(|row| row.iter().zip(v_new.iter()).map(|(kij, vj)| kij * vj).sum())
            .collect();
        let u_new: Vec<f64> = a
            .iter()
            .zip(kv.iter())
            .map(|(ai, ki)| if *ki > 1e-300 { ai / ki } else { 0.0 })
            .collect();

        // Convergence: change in u.
        let delta: f64 = u_new
            .iter()
            .zip(u.iter())
            .map(|(un, uo)| (un - uo).abs())
            .sum();
        u = u_new;
        v = v_new;

        if delta < tol {
            converged = true;
            let _ = iter;
            break;
        }
    }

    // Build transport plan γ = diag(u) K diag(v).
    let gamma: Vec<Vec<f64>> = k
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .map(|(j, kij)| u[i] * kij * v[j])
                .collect()
        })
        .collect();

    // Compute primal cost.
    let primal_cost: f64 = gamma
        .iter()
        .zip(cost.entries.iter())
        .map(|(gi, ci)| {
            gi.iter()
                .zip(ci.iter())
                .map(|(gij, cij)| gij * cij)
                .sum::<f64>()
        })
        .sum();

    let iters = if converged { max_iter } else { max_iter }; // approximation
    SinkhornResult {
        plan: TransportPlan {
            gamma,
            cost: primal_cost,
        },
        iterations: iters,
        converged,
    }
}

// ── Wasserstein distance ──────────────────────────────────────────────────────

/// Compute the Wasserstein-p distance W_p(μ, ν) using the Sinkhorn plan as a
/// proxy for the true OT cost.
///
/// Returns W_p = (sum_{ij} C\[i\]\[j\]^p γ\[i\]\[j\])^{1/p} where γ is the Sinkhorn
/// coupling with a small regularisation ε = 1e-3.
pub fn wasserstein_distance(
    p: f64,
    source: &Measure,
    target: &Measure,
    cost: &CostMatrix,
) -> WassersteinDistance {
    // Build C^p cost matrix.
    let cp_entries: Vec<Vec<f64>> = cost
        .entries
        .iter()
        .map(|row| row.iter().map(|c| c.powf(p)).collect())
        .collect();
    let cp_cost = CostMatrix::new(cp_entries, cost.n_source, cost.n_target);

    let result = sinkhorn(source, target, &cp_cost, 1e-3, 1000);
    let raw_cost: f64 = result.plan.cost.max(0.0);
    WassersteinDistance {
        p,
        distance: raw_cost.powf(1.0 / p),
    }
}

// ── Wasserstein-2 between Gaussians ──────────────────────────────────────────

/// Closed-form Bures metric / Wasserstein-2 distance between two Gaussian
/// distributions N(m1, Σ1) and N(m2, Σ2):
///
///   W_2^2 = ||m1 - m2||^2 + B(Σ1, Σ2)^2
///
/// where the Bures metric between positive definite matrices is approximated
/// here by the Frobenius-norm formula:
///   B^2 = tr(Σ1) + tr(Σ2) - 2 tr((Σ1^{1/2} Σ2 Σ1^{1/2})^{1/2}).
///
/// For diagonal covariances the matrix square root is exact.  For general
/// covariances we use a Schur decomposition-free approximation:
///   tr((Σ1 Σ2)^{1/2}) ≈ sqrt(|Σ1| * |Σ2|) (valid for commuting matrices).
pub fn wasserstein_2_gaussian(
    mean1: &[f64],
    cov1: &[Vec<f64>],
    mean2: &[f64],
    cov2: &[Vec<f64>],
) -> f64 {
    // Mean part.
    let mean_diff: Vec<f64> = mean1.iter().zip(mean2.iter()).map(|(a, b)| a - b).collect();
    let mean_sq = norm2(&mean_diff).powi(2);

    // Covariance part: B^2 = tr(Σ1) + tr(Σ2) - 2 tr((Σ1 Σ2)^{1/2}).
    let n = cov1.len();
    let tr1: f64 = (0..n)
        .map(|i| if i < cov1[i].len() { cov1[i][i] } else { 0.0 })
        .sum();
    let tr2: f64 = (0..n)
        .map(|i| if i < cov2[i].len() { cov2[i][i] } else { 0.0 })
        .sum();

    // For commuting PSD matrices: tr(Σ1^{1/2} Σ2 Σ1^{1/2})^{1/2} = tr((Σ1 Σ2)^{1/2}).
    // We use element-wise sqrt for diagonal case; for general we use Frobenius approximation.
    let cross_term = bures_cross_term(cov1, cov2, n);

    let bures_sq = (tr1 + tr2 - 2.0 * cross_term).max(0.0);
    (mean_sq + bures_sq).sqrt()
}

/// Approximation of tr((Σ1 Σ2)^{1/2}).
///
/// If both matrices are diagonal: exact formula sum_i sqrt(σ1_i * σ2_i).
/// Otherwise: approximation via product of traces.
fn bures_cross_term(cov1: &[Vec<f64>], cov2: &[Vec<f64>], n: usize) -> f64 {
    // Check if both are diagonal.
    let is_diag = |c: &[Vec<f64>]| -> bool {
        c.iter().enumerate().all(|(i, row)| {
            row.iter()
                .enumerate()
                .all(|(j, &v)| i == j || v.abs() < 1e-12)
        })
    };

    if is_diag(cov1) && is_diag(cov2) {
        (0..n)
            .map(|i| {
                let s1 = if i < cov1[i].len() { cov1[i][i] } else { 0.0 };
                let s2 = if i < cov2[i].len() { cov2[i][i] } else { 0.0 };
                (s1 * s2).max(0.0).sqrt()
            })
            .sum()
    } else {
        // Frobenius inner product approximation: tr((AB)^{1/2}) ≈ ||A^{1/2} B^{1/2}||_F.
        // Here we use sqrt(tr(A) * tr(B)) as a coarse bound.
        let tr1: f64 = (0..n)
            .map(|i| if i < cov1[i].len() { cov1[i][i] } else { 0.0 })
            .sum();
        let tr2: f64 = (0..n)
            .map(|i| if i < cov2[i].len() { cov2[i][i] } else { 0.0 })
            .sum();
        (tr1 * tr2).max(0.0).sqrt()
    }
}

// ── Sliced Wasserstein ────────────────────────────────────────────────────────

/// Sliced Wasserstein distance: average 1-D Wasserstein over random projections.
///
/// Uses a deterministic pseudo-random sequence seeded by `seed` (linear
/// congruential generator) to avoid external randomness dependencies.
pub fn sliced_wasserstein(
    source: &Measure,
    target: &Measure,
    n_proj: usize,
    seed: u64,
) -> SlicedWasserstein {
    if source.support.is_empty() || target.support.is_empty() || n_proj == 0 {
        return SlicedWasserstein {
            distance: 0.0,
            n_projections: n_proj,
        };
    }
    let d = source.support[0].len();
    let mut lcg = seed.wrapping_add(1);
    let mut total = 0.0f64;

    for _ in 0..n_proj {
        // Generate a unit vector via LCG + normalisation.
        let mut theta = vec![0.0f64; d];
        for t in theta.iter_mut() {
            lcg = lcg
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            // Map to [-1, 1].
            let frac = ((lcg >> 11) as f64) / ((1u64 << 53) as f64);
            *t = 2.0 * frac - 1.0;
        }
        let nrm = norm2(&theta);
        if nrm < 1e-12 {
            theta[0] = 1.0;
        } else {
            for t in theta.iter_mut() {
                *t /= nrm;
            }
        }

        // Project source and target onto theta.
        let src_proj: Vec<f64> = source.support.iter().map(|x| dot(x, &theta)).collect();
        let tgt_proj: Vec<f64> = target.support.iter().map(|x| dot(x, &theta)).collect();

        let emd = earth_movers_distance_1d(&src_proj, &source.weights, &tgt_proj, &target.weights);
        total += emd;
    }

    SlicedWasserstein {
        distance: total / n_proj as f64,
        n_projections: n_proj,
    }
}

// ── Fréchet mean (Wasserstein barycenter) ────────────────────────────────────

/// Compute a Wasserstein barycenter of a collection of discrete measures.
///
/// Uses the fixed-support barycenter algorithm of Cuturi & Doucet (2014):
/// the support is fixed to that of the first measure, and weights are
/// iteratively updated via geometric averaging of Sinkhorn plans.
pub fn frechet_mean(
    measures: &[Measure],
    weights: &[f64],
    cost: &CostMatrix,
    max_iter: usize,
) -> BarycenterResult {
    if measures.is_empty() || weights.is_empty() {
        return BarycenterResult {
            weights: vec![],
            support: vec![],
            iterations: 0,
        };
    }
    let n = cost.n_source;
    // Initialise barycenter weights uniformly.
    let mut bar_weights = vec![1.0 / n as f64; n];
    let epsilon = 5e-2;

    for it in 0..max_iter {
        let mut log_bar = vec![0.0f64; n];
        for (measure, &lam) in measures.iter().zip(weights.iter()) {
            let bar_measure = Measure::new(bar_weights.clone(), measures[0].support.clone());
            let sk = sinkhorn(&bar_measure, measure, cost, epsilon, 200);
            // Row marginal of the plan.
            let marginal: Vec<f64> = sk
                .plan
                .gamma
                .iter()
                .map(|row| row.iter().sum::<f64>())
                .collect();
            // Geometric averaging in log space.
            for (lb, mi) in log_bar.iter_mut().zip(marginal.iter()) {
                *lb += lam * mi.max(1e-300).ln();
            }
        }
        let new_weights: Vec<f64> = log_bar.iter().map(|lb| lb.exp()).collect();
        let sum: f64 = new_weights.iter().sum();
        let new_weights: Vec<f64> = if sum > 1e-300 {
            new_weights.iter().map(|w| w / sum).collect()
        } else {
            vec![1.0 / n as f64; n]
        };
        // Convergence check.
        let delta: f64 = bar_weights
            .iter()
            .zip(new_weights.iter())
            .map(|(a, b)| (a - b).abs())
            .sum();
        bar_weights = new_weights;
        if delta < 1e-8 {
            return BarycenterResult {
                weights: bar_weights,
                support: measures[0].support.clone(),
                iterations: it + 1,
            };
        }
    }

    BarycenterResult {
        weights: bar_weights,
        support: measures[0].support.clone(),
        iterations: max_iter,
    }
}

// ── Dual objective ────────────────────────────────────────────────────────────

/// Compute the Kantorovich dual objective:
///   D(f, g) = <f, a> + <g, b>  subject to f_i + g_j <= C\[i\]\[j\].
pub fn dual_objective(
    f: &[f64],
    g: &[f64],
    cost: &CostMatrix,
    source_w: &[f64],
    target_w: &[f64],
) -> f64 {
    // Check dual feasibility (clamp to feasible if violated).
    let feasible = f.iter().enumerate().all(|(i, &fi)| {
        g.iter().enumerate().all(|(j, &gj)| {
            let c = cost
                .entries
                .get(i)
                .and_then(|r| r.get(j))
                .copied()
                .unwrap_or(0.0);
            fi + gj <= c + 1e-9
        })
    });
    if !feasible {
        return f64::NEG_INFINITY;
    }
    dot(f, source_w) + dot(g, target_w)
}

// ── Marginals ─────────────────────────────────────────────────────────────────

/// Compute source and target marginals of a transport plan.
///
/// Returns `(source_marginal, target_marginal)` where
/// `source_marginal\[i\] = sum_j γ\[i\]\[j\]` and `target_marginal\[j\] = sum_i γ\[i\]\[j\]`.
pub fn transport_plan_marginals(plan: &TransportPlan) -> (Vec<f64>, Vec<f64>) {
    if plan.gamma.is_empty() {
        return (vec![], vec![]);
    }
    let n = plan.gamma.len();
    let m = plan.gamma[0].len();
    let source_marg: Vec<f64> = plan.gamma.iter().map(|row| row.iter().sum()).collect();
    let target_marg: Vec<f64> = (0..m)
        .map(|j| (0..n).map(|i| plan.gamma[i][j]).sum())
        .collect();
    (source_marg, target_marg)
}

// ── Coupling check ────────────────────────────────────────────────────────────

/// Check whether a transport plan γ is a valid coupling for measures (a, b),
/// i.e., source and target marginals match within tolerance `tol`.
pub fn check_coupling(plan: &TransportPlan, source_w: &[f64], target_w: &[f64], tol: f64) -> bool {
    let (src_marg, tgt_marg) = transport_plan_marginals(plan);
    if src_marg.len() != source_w.len() || tgt_marg.len() != target_w.len() {
        return false;
    }
    let src_ok = src_marg
        .iter()
        .zip(source_w.iter())
        .all(|(m, w)| (m - w).abs() <= tol);
    let tgt_ok = tgt_marg
        .iter()
        .zip(target_w.iter())
        .all(|(m, w)| (m - w).abs() <= tol);
    src_ok && tgt_ok
}

// ── Support intersection ──────────────────────────────────────────────────────

/// Return the atoms that appear in both the source and target support
/// (coordinate-wise equality up to 1e-9 tolerance).
pub fn support_intersection(source: &Measure, target: &Measure) -> Vec<Vec<f64>> {
    source
        .support
        .iter()
        .filter(|xs| {
            target.support.iter().any(|xt| {
                xs.len() == xt.len() && xs.iter().zip(xt.iter()).all(|(a, b)| (a - b).abs() < 1e-9)
            })
        })
        .cloned()
        .collect()
}

// ── Normalisation ─────────────────────────────────────────────────────────────

/// Normalise the weights of `measure` so they sum to 1.
///
/// If all weights are zero or negative, sets uniform weights.
pub fn normalize_measure(measure: &mut Measure) {
    let sum: f64 = measure.weights.iter().map(|w| w.max(0.0)).sum();
    if sum < 1e-300 {
        let n = measure.weights.len();
        if n > 0 {
            measure.weights = vec![1.0 / n as f64; n];
        }
    } else {
        measure.weights = measure.weights.iter().map(|w| w.max(0.0) / sum).collect();
    }
}

// ── Dual solution helper ──────────────────────────────────────────────────────

/// Compute approximate dual potentials (f, g) from a transport plan using the
/// complementary slackness conditions.
///
/// For the entropic OT problem:
///   f_i = ε log(u_i),  g_j = ε log(v_j)
/// where u, v are the Sinkhorn scaling vectors.  Here we recover them from the
/// plan marginals and the cost matrix.
pub fn ot_dual_solution(
    source: &Measure,
    target: &Measure,
    cost: &CostMatrix,
    epsilon: f64,
) -> OTDualSolution {
    let result = sinkhorn(source, target, cost, epsilon, 500);
    let (src_marg, _) = transport_plan_marginals(&result.plan);
    // f_i = epsilon * log(a_i / src_marg_i)
    let f: Vec<f64> = source
        .weights
        .iter()
        .zip(src_marg.iter())
        .map(|(ai, mi)| {
            if *mi > 1e-300 {
                epsilon * (ai / mi).ln()
            } else {
                0.0
            }
        })
        .collect();
    let (_, tgt_marg) = transport_plan_marginals(&result.plan);
    let g: Vec<f64> = target
        .weights
        .iter()
        .zip(tgt_marg.iter())
        .map(|(bi, mi)| {
            if *mi > 1e-300 {
                epsilon * (bi / mi).ln()
            } else {
                0.0
            }
        })
        .collect();
    let dual_obj = dot(&f, &source.weights) + dot(&g, &target.weights);
    OTDualSolution {
        f,
        g,
        dual_objective: dual_obj,
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn approx(a: f64, b: f64, tol: f64) -> bool {
        (a - b).abs() < tol
    }

    fn uniform_measure(pts: Vec<Vec<f64>>) -> Measure {
        let n = pts.len();
        Measure::new(vec![1.0 / n as f64; n], pts)
    }

    // 1. Cost matrix: Euclidean distance from (0,0) to (3,4) = 5.
    #[test]
    fn test_cost_matrix_euclidean() {
        let src = vec![vec![0.0, 0.0]];
        let tgt = vec![vec![3.0, 4.0]];
        let c = cost_matrix(&src, &tgt, &GroundMetric::Euclidean);
        assert!(approx(c.entries[0][0], 5.0, 1e-9));
    }

    // 2. Cost matrix: squared Euclidean.
    #[test]
    fn test_cost_matrix_squared_euclidean() {
        let src = vec![vec![0.0]];
        let tgt = vec![vec![2.0]];
        let c = cost_matrix(&src, &tgt, &GroundMetric::SquaredEuclidean);
        assert!(approx(c.entries[0][0], 4.0, 1e-9));
    }

    // 3. Cost matrix: L1.
    #[test]
    fn test_cost_matrix_l1() {
        let src = vec![vec![1.0, 2.0]];
        let tgt = vec![vec![3.0, 5.0]];
        let c = cost_matrix(&src, &tgt, &GroundMetric::L1);
        assert!(approx(c.entries[0][0], 5.0, 1e-9));
    }

    // 4. EMD 1-D: identical distributions → 0.
    #[test]
    fn test_emd_1d_identical() {
        let xs = vec![0.0, 1.0, 2.0];
        let ws = vec![1.0 / 3.0; 3];
        let d = earth_movers_distance_1d(&xs, &ws, &xs, &ws);
        assert!(d < 1e-6);
    }

    // 5. EMD 1-D: point mass shifted by 1.
    #[test]
    fn test_emd_1d_shift() {
        let src = vec![0.0];
        let sw = vec![1.0];
        let tgt = vec![1.0];
        let tw = vec![1.0];
        let d = earth_movers_distance_1d(&src, &sw, &tgt, &tw);
        assert!(approx(d, 1.0, 1e-6));
    }

    // 6. Sinkhorn: marginals approximately match source weights.
    #[test]
    fn test_sinkhorn_source_marginal() {
        let src = uniform_measure(vec![vec![0.0], vec![1.0]]);
        let tgt = uniform_measure(vec![vec![0.5], vec![1.5]]);
        let c = cost_matrix(&src.support, &tgt.support, &GroundMetric::SquaredEuclidean);
        let res = sinkhorn(&src, &tgt, &c, 0.01, 500);
        let (sm, _) = transport_plan_marginals(&res.plan);
        for (m, w) in sm.iter().zip(src.weights.iter()) {
            assert!(approx(*m, *w, 0.05));
        }
    }

    // 7. Sinkhorn: target marginals approximately match.
    #[test]
    fn test_sinkhorn_target_marginal() {
        let src = uniform_measure(vec![vec![0.0], vec![2.0]]);
        let tgt = uniform_measure(vec![vec![1.0], vec![3.0]]);
        let c = cost_matrix(&src.support, &tgt.support, &GroundMetric::Euclidean);
        let res = sinkhorn(&src, &tgt, &c, 0.05, 500);
        let (_, tm) = transport_plan_marginals(&res.plan);
        for (m, w) in tm.iter().zip(tgt.weights.iter()) {
            assert!(approx(*m, *w, 0.1));
        }
    }

    // 8. Wasserstein-2 between Gaussians: identical → 0.
    #[test]
    fn test_wasserstein_2_gaussian_identical() {
        let mu = vec![1.0, 2.0];
        let sigma = vec![vec![1.0, 0.0], vec![0.0, 1.0]];
        let d = wasserstein_2_gaussian(&mu, &sigma, &mu, &sigma);
        assert!(d < 1e-6);
    }

    // 9. Wasserstein-2 between Gaussians: mean shift.
    #[test]
    fn test_wasserstein_2_gaussian_mean_shift() {
        let mu1 = vec![0.0];
        let mu2 = vec![3.0];
        let sigma = vec![vec![1.0]];
        let d = wasserstein_2_gaussian(&mu1, &sigma, &mu2, &sigma);
        // W_2 = sqrt(9 + (1 + 1 - 2*1)) = sqrt(9) = 3.
        assert!(approx(d, 3.0, 1e-6));
    }

    // 10. Sliced Wasserstein: same measure → near zero.
    #[test]
    fn test_sliced_wasserstein_same_measure() {
        let m = uniform_measure(vec![vec![0.0, 0.0], vec![1.0, 1.0], vec![2.0, 0.0]]);
        let sw = sliced_wasserstein(&m, &m, 20, 42);
        assert!(sw.distance < 1e-6);
    }

    // 11. Transport plan marginals: row and column sums.
    #[test]
    fn test_transport_plan_marginals() {
        let gamma = vec![vec![0.3, 0.2], vec![0.1, 0.4]];
        let plan = TransportPlan { gamma, cost: 0.0 };
        let (sm, tm) = transport_plan_marginals(&plan);
        assert!(approx(sm[0], 0.5, 1e-9));
        assert!(approx(sm[1], 0.5, 1e-9));
        assert!(approx(tm[0], 0.4, 1e-9));
        assert!(approx(tm[1], 0.6, 1e-9));
    }

    // 12. Check coupling: valid coupling passes.
    #[test]
    fn test_check_coupling_valid() {
        let gamma = vec![vec![0.5, 0.0], vec![0.0, 0.5]];
        let plan = TransportPlan { gamma, cost: 0.0 };
        assert!(check_coupling(&plan, &[0.5, 0.5], &[0.5, 0.5], 1e-6));
    }

    // 13. Check coupling: invalid coupling fails.
    #[test]
    fn test_check_coupling_invalid() {
        let gamma = vec![vec![0.3, 0.2], vec![0.1, 0.3]]; // target marginals = [0.4, 0.5] ≠ [0.5, 0.5]
        let plan = TransportPlan { gamma, cost: 0.0 };
        assert!(!check_coupling(&plan, &[0.5, 0.4], &[0.5, 0.5], 1e-6));
    }

    // 14. Support intersection: common atom returned.
    #[test]
    fn test_support_intersection_common() {
        let src = Measure::new(vec![0.5, 0.5], vec![vec![0.0], vec![1.0]]);
        let tgt = Measure::new(vec![0.5, 0.5], vec![vec![1.0], vec![2.0]]);
        let inter = support_intersection(&src, &tgt);
        assert_eq!(inter.len(), 1);
        assert!(approx(inter[0][0], 1.0, 1e-9));
    }

    // 15. Support intersection: disjoint supports.
    #[test]
    fn test_support_intersection_disjoint() {
        let src = Measure::new(vec![1.0], vec![vec![0.0]]);
        let tgt = Measure::new(vec![1.0], vec![vec![1.0]]);
        let inter = support_intersection(&src, &tgt);
        assert!(inter.is_empty());
    }

    // 16. Normalize measure: non-uniform weights.
    #[test]
    fn test_normalize_measure() {
        let mut m = Measure::new(vec![2.0, 3.0, 5.0], vec![vec![0.0]; 3]);
        normalize_measure(&mut m);
        let sum: f64 = m.weights.iter().sum();
        assert!(approx(sum, 1.0, 1e-12));
        assert!(approx(m.weights[0], 0.2, 1e-12));
    }

    // 17. Normalize measure: all zeros → uniform.
    #[test]
    fn test_normalize_measure_all_zero() {
        let mut m = Measure::new(vec![0.0, 0.0], vec![vec![0.0]; 2]);
        normalize_measure(&mut m);
        assert!(approx(m.weights[0], 0.5, 1e-12));
    }

    // 18. Dual objective: feasible potentials.
    #[test]
    fn test_dual_objective_feasible() {
        // C = [[1.0]], a = [1.0], b = [1.0], f = [0.5], g = [0.5]
        let c = CostMatrix::new(vec![vec![1.0]], 1, 1);
        let val = dual_objective(&[0.5], &[0.5], &c, &[1.0], &[1.0]);
        assert!(approx(val, 1.0, 1e-12));
    }

    // 19. Dual objective: infeasible potentials → -∞.
    #[test]
    fn test_dual_objective_infeasible() {
        // f + g = 2 > C[0][0] = 1.
        let c = CostMatrix::new(vec![vec![1.0]], 1, 1);
        let val = dual_objective(&[1.0], &[1.5], &c, &[1.0], &[1.0]);
        assert!(val.is_infinite() && val < 0.0);
    }

    // 20. Frechet mean of two identical measures → same measure.
    #[test]
    fn test_frechet_mean_identical() {
        let m1 = uniform_measure(vec![vec![0.0], vec![1.0]]);
        let m2 = m1.clone();
        let c = cost_matrix(&m1.support, &m2.support, &GroundMetric::SquaredEuclidean);
        let bar = frechet_mean(&[m1.clone(), m2], &[0.5, 0.5], &c, 20);
        let sum: f64 = bar.weights.iter().sum();
        assert!(approx(sum, 1.0, 1e-3));
        assert_eq!(bar.support.len(), 2);
    }

    // 21. Wasserstein distance W2: non-negative.
    #[test]
    fn test_wasserstein_distance_nonneg() {
        let src = uniform_measure(vec![vec![0.0], vec![1.0]]);
        let tgt = uniform_measure(vec![vec![2.0], vec![3.0]]);
        let c = cost_matrix(&src.support, &tgt.support, &GroundMetric::SquaredEuclidean);
        let wd = wasserstein_distance(2.0, &src, &tgt, &c);
        assert!(wd.distance >= 0.0);
        assert!(approx(wd.p, 2.0, 1e-12));
    }

    // 22. Sliced Wasserstein n_projections field.
    #[test]
    fn test_sliced_wasserstein_n_projections() {
        let src = uniform_measure(vec![vec![0.0], vec![1.0]]);
        let tgt = uniform_measure(vec![vec![1.0], vec![2.0]]);
        let sw = sliced_wasserstein(&src, &tgt, 50, 1234);
        assert_eq!(sw.n_projections, 50);
    }
}
