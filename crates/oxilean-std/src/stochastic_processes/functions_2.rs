//! Auto-generated module (split from functions.rs)
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{Declaration, Environment, Expr, Name};

use super::functions::*;
use super::types::{
    AbsorptionData, BrownianMotion, HittingTime, MarkovChain, PoissonProcess, RandomWalk, State,
    StationaryDistribution, TransitionMatrix, WalkDistribution,
};

/// `burkholder_davis_gundy : StochasticProcess → Real → Prop`
/// The Burkholder-Davis-Gundy inequality: for p ≥ 1 and local martingale M,
/// E[max_{s≤t} |M_s|^p] ≤ C_p E[\[M\]_t^{p/2}].
pub fn burkholder_davis_gundy_ty() -> Expr {
    arrow(stochastic_process_ty(), arrow(real_ty(), prop()))
}
/// `exponential_martingale_bound : StochasticProcess → Real → Prop`
/// Exponential martingale inequality: P(max_{s≤t} M_s ≥ λ, \[M\]_t ≤ c) ≤ exp(-λ²/(2c)).
pub fn exponential_martingale_bound_ty() -> Expr {
    arrow(stochastic_process_ty(), arrow(real_ty(), prop()))
}
/// `azuma_hoeffding_martingale : StochasticProcess → Nat → Real → Prop`
/// Azuma-Hoeffding inequality for martingales with bounded differences.
pub fn azuma_hoeffding_martingale_ty() -> Expr {
    arrow(
        stochastic_process_ty(),
        arrow(nat_ty(), arrow(real_ty(), prop())),
    )
}
/// Register all extended stochastic process axioms into the kernel environment.
pub fn register_sp_extended(env: &mut Environment) -> Result<(), String> {
    let axioms: &[(&str, Expr)] = &[
        ("martingale_l1_convergence", martingale_l1_convergence_ty()),
        ("martingale_l2_convergence", martingale_l2_convergence_ty()),
        ("martingale_as_convergence", martingale_as_convergence_ty()),
        (
            "reverse_martingale_convergence",
            reverse_martingale_convergence_ty(),
        ),
        (
            "uniformly_integrable_martingale",
            uniformly_integrable_martingale_ty(),
        ),
        ("optional_stopping_ui", optional_stopping_ui_ty()),
        ("doob_maximal_l2", doob_maximal_l2_ty()),
        (
            "doob_upcrossing_inequality",
            doob_upcrossing_inequality_ty(),
        ),
        ("PredictableProcess", predictable_process_ty()),
        ("PredictableSigmaAlgebra", predictable_sigma_algebra_ty()),
        ("OptionalSigmaAlgebra", optional_sigma_algebra_ty()),
        ("predictable_compensator", predictable_compensator_ty()),
        ("natural_filtration", natural_filtration_ty()),
        (
            "brownian_filtration_complete",
            brownian_filtration_complete_ty(),
        ),
        (
            "brownian_increment_independence",
            brownian_increment_independence_ty(),
        ),
        ("brownian_scaling", brownian_scaling_ty()),
        ("brownian_time_inversion", brownian_time_inversion_ty()),
        ("brownian_lil", brownian_lil_ty()),
        ("ito_isometry_general", ito_isometry_general_ty()),
        ("StochasticExponential", stochastic_exponential_ty()),
        (
            "stochastic_exponential_martingale",
            stochastic_exponential_martingale_ty(),
        ),
        ("novikov_condition", novikov_condition_ty()),
        ("cameron_martin_theorem", cameron_martin_theorem_ty()),
        ("girsanov_multidim", girsanov_multidim_ty()),
        (
            "equivalent_martingale_measure",
            equivalent_martingale_measure_ty(),
        ),
        ("first_ftam", first_ftam_ty()),
        ("MalliavinDerivative", malliavin_derivative_ty()),
        ("SkorokhodIntegral", skorokhod_integral_ty()),
        (
            "malliavin_integration_by_parts",
            malliavin_integration_by_parts_ty(),
        ),
        ("clark_ocone_formula", clark_ocone_formula_ty()),
        (
            "malliavin_smooth_functional",
            malliavin_smooth_functional_ty(),
        ),
        ("RoughPath", rough_path_ty()),
        ("ControlledRoughPath", controlled_rough_path_ty()),
        ("rough_path_integral", rough_path_integral_ty()),
        ("rough_path_continuity", rough_path_continuity_ty()),
        ("brownian_rough_path", brownian_rough_path_ty()),
        ("rough_path_rde_solution", rough_path_rde_solution_ty()),
        ("FellerSemigroup", feller_semigroup_ty()),
        ("feller_process_existence", feller_process_existence_ty()),
        (
            "kolmogorov_forward_equation",
            kolmogorov_forward_equation_ty(),
        ),
        (
            "kolmogorov_backward_equation",
            kolmogorov_backward_equation_ty(),
        ),
        ("generator_feller", generator_feller_ty()),
        ("ReflectedBrownianMotion", reflected_brownian_motion_ty()),
        ("reflected_bm_existence", reflected_bm_existence_ty()),
        (
            "skorokhod_reflection_problem",
            skorokhod_reflection_problem_ty(),
        ),
        ("tanaka_formula", tanaka_formula_ty()),
        ("local_time", local_time_ty()),
        (
            "InfinitelyDivisibleDistribution",
            infinitely_divisible_distribution_ty(),
        ),
        (
            "levy_process_infinitely_divisible",
            levy_process_infinitely_divisible_ty(),
        ),
        ("PoissonRandomMeasure", poisson_random_measure_ty()),
        (
            "poisson_random_measure_levy",
            poisson_random_measure_levy_ty(),
        ),
        ("stable_process", stable_process_ty()),
        ("subordinator", subordinator_ty()),
        ("subordination", subordination_ty()),
        ("burkholder_davis_gundy", burkholder_davis_gundy_ty()),
        (
            "exponential_martingale_bound",
            exponential_martingale_bound_ty(),
        ),
        (
            "azuma_hoeffding_martingale",
            azuma_hoeffding_martingale_ty(),
        ),
    ];
    for (name, ty) in axioms {
        env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty: ty.clone(),
        })
        .map_err(|e| format!("Failed to add {name}: {e:?}"))?;
    }
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_random_walk_length() {
        for &n in &[0u32, 1, 10, 100] {
            let path = random_walk(n, 42);
            assert_eq!(
                path.len(),
                n as usize + 1,
                "random_walk({n}) should have {n}+1 points"
            );
        }
    }
    #[test]
    fn test_random_walk_deterministic() {
        let a = random_walk(50, 12345);
        let b = random_walk(50, 12345);
        assert_eq!(a, b, "same seed must produce identical paths");
    }
    #[test]
    fn test_brownian_motion_starts_at_zero() {
        let path = brownian_motion(1.0, 100, 7);
        assert_eq!(path.len(), 101);
        let (t0, x0) = path[0];
        assert!((t0 - 0.0).abs() < 1e-12, "t_0 should be 0");
        assert!((x0 - 0.0).abs() < 1e-12, "B_0 should be 0");
    }
    #[test]
    fn test_geometric_brownian_positive() {
        let path = geometric_brownian_motion(100.0, 0.05, 0.2, 1.0, 252, 99);
        for &(_, s) in &path {
            assert!(s > 0.0, "GBM must remain strictly positive, got {s}");
        }
    }
    #[test]
    fn test_black_scholes_call_positive() {
        let c = black_scholes_call(100.0, 100.0, 1.0, 0.05, 0.2);
        assert!(c > 0.0, "call price must be positive, got {c}");
        assert!(c <= 100.0, "call price must not exceed S, got {c}");
    }
    #[test]
    fn test_black_scholes_put_call_parity() {
        let (s, k, t, r, sigma) = (100.0, 95.0, 0.5, 0.04, 0.25);
        let c = black_scholes_call(s, k, t, r, sigma);
        let p = black_scholes_put(s, k, t, r, sigma);
        let parity = k * (-r * t).exp() - s;
        let diff = p - c - parity;
        assert!(
            diff.abs() < 1e-10,
            "put-call parity violated: |P - C - (Ke^{{-rT}} - S)| = {:.2e}",
            diff.abs()
        );
    }
    #[test]
    fn test_standard_normal_cdf() {
        let n0 = standard_normal_cdf(0.0);
        assert!((n0 - 0.5).abs() < 1e-6, "Φ(0) ≈ 0.5, got {n0}");
        let ninf = standard_normal_cdf(8.0);
        assert!((ninf - 1.0).abs() < 1e-6, "Φ(8) ≈ 1, got {ninf}");
        let nminf = standard_normal_cdf(-8.0);
        assert!(nminf.abs() < 1e-6, "Φ(-8) ≈ 0, got {nminf}");
        assert!(standard_normal_cdf(1.0) > standard_normal_cdf(0.0));
    }
    #[test]
    fn test_ct_markov_chain_new() {
        let states = vec!["A".to_string(), "B".to_string(), "C".to_string()];
        let q = vec![
            vec![-2.0, 1.0, 1.0],
            vec![1.0, -3.0, 2.0],
            vec![2.0, 1.0, -3.0],
        ];
        let chain = super::super::types::CtMarkovChain::new(states.clone(), q.clone());
        assert_eq!(chain.states.len(), 3);
        assert_eq!(chain.rate_matrix.len(), 3);
        let ht = chain.expected_hitting_time(0, 1);
        assert!((ht - 1.0).abs() < 1e-12, "E[T_{{AB}}] = 1, got {ht}");
        let sd = chain.stationary_distribution().expect("should converge");
        let total: f64 = sd.iter().sum();
        assert!((total - 1.0).abs() < 1e-6, "π sums to 1, got {total}");
    }
    #[test]
    fn test_euler_maruyama_brownian() {
        let em = super::super::types::EulerMaruyama::new(|_x| 0.0, |_x| 1.0);
        let path = em.simulate(0.0, 1.0, 100, 42);
        assert_eq!(path.len(), 101);
        let (t0, x0) = path[0];
        assert!((t0).abs() < 1e-12);
        assert!((x0).abs() < 1e-12);
    }
    #[test]
    fn test_ou_process_stationary_variance() {
        let ou = super::super::types::OrnsteinUhlenbeckProcess::new(2.0, 0.0, 1.0);
        assert!((ou.stationary_variance() - 0.25).abs() < 1e-12);
        assert!((ou.stationary_std() - 0.5).abs() < 1e-12);
    }
    #[test]
    fn test_ou_simulate_length() {
        let ou = super::super::types::OrnsteinUhlenbeckProcess::new(1.0, 0.0, 0.3);
        let path = ou.simulate(1.0, 5.0, 200, 7);
        assert_eq!(path.len(), 201);
    }
    #[test]
    fn test_gbm_expected_value() {
        let gbm = super::super::types::GeometricBrownianMotionProcess::new(0.1, 0.2);
        let ev = gbm.expected_value(100.0, 1.0);
        assert!((ev - 100.0 * (0.1f64).exp()).abs() < 1e-10);
    }
    #[test]
    fn test_gbm_simulate_positive() {
        let gbm = super::super::types::GeometricBrownianMotionProcess::new(0.05, 0.3);
        let path = gbm.simulate(50.0, 2.0, 500, 13);
        for &(_, s) in &path {
            assert!(s > 0.0, "GBM values must be positive, got {s}");
        }
    }
    #[test]
    fn test_poisson_arrivals_nonnegative() {
        let pp = super::super::types::PoissonProcessSimulator::new(5.0);
        let arrivals = pp.arrival_times(3.0, 77);
        for &t in &arrivals {
            assert!(t >= 0.0 && t <= 3.0, "arrival time {t} out of [0,3]");
        }
        let expected = pp.expected_count(3.0);
        assert!((expected - 15.0).abs() < 1e-10);
    }
    #[test]
    fn test_poisson_counting_process_nondecreasing() {
        let pp = super::super::types::PoissonProcessSimulator::new(3.0);
        let path = pp.counting_process(5.0, 100, 42);
        assert_eq!(path.len(), 101);
        for i in 1..path.len() {
            assert!(
                path[i].1 >= path[i - 1].1,
                "counting process must be non-decreasing"
            );
        }
    }
    #[test]
    fn test_black_scholes_pricer_greeks() {
        let pricer = super::super::types::BlackScholesPricer::new(100.0, 100.0, 1.0, 0.05, 0.2);
        let delta = pricer.call_delta();
        assert!(delta > 0.4 && delta < 0.7, "call delta = {delta}");
        let put_delta = pricer.put_delta();
        assert!(
            put_delta < 0.0,
            "put delta should be negative, got {put_delta}"
        );
        let gamma = pricer.gamma();
        assert!(gamma > 0.0, "gamma should be positive, got {gamma}");
        let vega = pricer.vega();
        assert!(vega > 0.0, "vega should be positive, got {vega}");
    }
    #[test]
    fn test_black_scholes_pricer_put_call_parity() {
        let pricer = super::super::types::BlackScholesPricer::new(100.0, 95.0, 0.5, 0.04, 0.25);
        let c = pricer.call_price();
        let p = pricer.put_price();
        let fwd = pricer.strike * (-pricer.rate * pricer.time_to_expiry).exp();
        let parity = c - p - (pricer.spot - fwd);
        assert!(parity.abs() < 1e-10, "put-call parity violated: {parity}");
    }
    #[test]
    fn test_black_scholes_pricer_implied_vol() {
        let pricer = super::super::types::BlackScholesPricer::new(100.0, 100.0, 1.0, 0.05, 0.2);
        let call = pricer.call_price();
        let iv = pricer
            .implied_volatility_call(call)
            .expect("IV should converge");
        assert!(
            (iv - 0.2).abs() < 1e-5,
            "recovered IV = {iv:.6}, expected 0.2"
        );
    }
    #[test]
    fn test_compound_poisson_statistics() {
        let cpp = super::super::types::CompoundPoissonProcess::new(2.0, 1.0, 0.5);
        let mean = cpp.expected_value(1.0);
        assert!((mean - 2.0).abs() < 1e-10);
        let var = cpp.variance(1.0);
        assert!((var - 2.5).abs() < 1e-10);
    }
    #[test]
    fn test_compound_poisson_simulate_length() {
        let cpp = super::super::types::CompoundPoissonProcess::new(1.0, 0.5, 0.2);
        let path = cpp.simulate(10.0, 100, 42);
        assert_eq!(path.len(), 101);
    }
    #[test]
    fn test_build_stochastic_processes_env() {
        let mut env = Environment::new();
        build_stochastic_processes_env(&mut env);
        assert!(
            env.get(&Name::str("brownian_motion_existence")).is_some(),
            "brownian_motion_existence should be registered"
        );
        assert!(
            env.get(&Name::str("feynman_kac_formula")).is_some(),
            "feynman_kac_formula should be registered"
        );
        assert!(
            env.get(&Name::str("levy_khintchine_formula")).is_some(),
            "levy_khintchine_formula should be registered"
        );
        assert!(
            env.get(&Name::str("poisson_superposition")).is_some(),
            "poisson_superposition should be registered"
        );
        assert!(
            env.get(&Name::str("fokker_planck_equation")).is_some(),
            "fokker_planck_equation should be registered"
        );
        assert!(
            env.get(&Name::str("semimartingale_integration")).is_some(),
            "semimartingale_integration should be registered"
        );
    }
}

// ── New Markov Chain / Stochastic Analysis Functions ─────────────────────────
const ROW_SUM_EPS: f64 = 1e-9;

/// Returns `true` if every row of `m` sums to 1 within `ROW_SUM_EPS`.
pub fn is_stochastic(m: &TransitionMatrix) -> bool {
    for row in &m.data {
        let s: f64 = row.iter().sum();
        if (s - 1.0).abs() > ROW_SUM_EPS {
            return false;
        }
        if row.iter().any(|&v| v < -ROW_SUM_EPS) {
            return false;
        }
    }
    true
}

/// Multiply distribution vector `dist` by transition matrix `m` one step: dist' = dist·P.
fn step_distribution(dist: &[f64], m: &TransitionMatrix) -> Vec<f64> {
    let n = m.size;
    let mut out = vec![0.0f64; n];
    for (i, &p) in dist.iter().enumerate() {
        for j in 0..n {
            out[j] += p * m.get(i, j);
        }
    }
    out
}

/// Apply the transition matrix `steps` times to the initial distribution.
///
/// Returns the distribution after `steps` steps: π_0 · P^steps.
pub fn power_iteration(m: &TransitionMatrix, steps: usize) -> Vec<f64> {
    let n = m.size;
    let mut dist: Vec<f64> = vec![1.0 / n as f64; n];
    for _ in 0..steps {
        dist = step_distribution(&dist, m);
    }
    dist
}

/// Estimate the stationary distribution of `chain` via power method.
///
/// Returns `None` if the chain has no states.
pub fn stationary_distribution(chain: &MarkovChain) -> Option<StationaryDistribution> {
    let n = chain.transition.size;
    if n == 0 {
        return None;
    }
    let mut dist = chain.initial.clone();
    // Normalise initial distribution.
    let s: f64 = dist.iter().sum();
    if s < ROW_SUM_EPS {
        dist = vec![1.0 / n as f64; n];
    } else {
        dist.iter_mut().for_each(|v| *v /= s);
    }
    for _ in 0..20_000 {
        let next = step_distribution(&dist, &chain.transition);
        // Check convergence.
        let diff: f64 = dist
            .iter()
            .zip(next.iter())
            .map(|(a, b)| (a - b).abs())
            .sum();
        dist = next;
        if diff < 1e-12 {
            break;
        }
    }
    Some(StationaryDistribution::new(dist))
}

/// Half the L1 distance between distributions p and q.
pub fn total_variation_distance(p: &[f64], q: &[f64]) -> f64 {
    0.5 * p
        .iter()
        .zip(q.iter())
        .map(|(a, b)| (a - b).abs())
        .sum::<f64>()
}

/// Return `true` if the chain appears ergodic (irreducible + aperiodic).
///
/// A chain is ergodic if P^k has all strictly positive entries for some k.
/// We check this by checking that P + P^2 has all positive entries (heuristic).
pub fn is_ergodic(chain: &MarkovChain) -> bool {
    let n = chain.transition.size;
    if n == 0 {
        return false;
    }
    // Compute P + P^2 entry-wise via matrix multiplication.
    let p = &chain.transition;
    let mut p2 = vec![vec![0.0f64; n]; n];
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                p2[i][j] += p.get(i, k) * p.get(k, j);
            }
        }
    }
    // Check all entries of P + P² are positive.
    for i in 0..n {
        for j in 0..n {
            if p.get(i, j) + p2[i][j] <= 0.0 {
                return false;
            }
        }
    }
    true
}

/// Estimate the mixing time: smallest t such that TV(π_0 · P^t, π) < epsilon.
pub fn mixing_time(chain: &MarkovChain, epsilon: f64) -> usize {
    let n = chain.transition.size;
    if n == 0 {
        return 0;
    }
    let stat = match stationary_distribution(chain) {
        Some(s) => s,
        None => return 0,
    };
    let mut dist = chain.initial.clone();
    let s: f64 = dist.iter().sum();
    if s < ROW_SUM_EPS {
        dist = vec![1.0 / n as f64; n];
    } else {
        dist.iter_mut().for_each(|v| *v /= s);
    }
    for t in 0..100_000usize {
        if total_variation_distance(&dist, &stat.probs) < epsilon {
            return t;
        }
        dist = step_distribution(&dist, &chain.transition);
    }
    100_000
}

/// Identify absorbing states (self-loop with probability 1).
pub fn identify_absorbing_states(chain: &MarkovChain) -> Vec<State> {
    (0..chain.transition.size)
        .filter(|&i| (chain.transition.get(i, i) - 1.0).abs() < ROW_SUM_EPS)
        .map(State)
        .collect()
}

/// Full absorption analysis for chains with absorbing states.
///
/// Uses the fundamental matrix N = (I − Q)^{−1} where Q is the transient sub-matrix.
pub fn absorption_analysis(chain: &MarkovChain) -> AbsorptionData {
    let abs = identify_absorbing_states(chain);
    let abs_set: std::collections::HashSet<usize> = abs.iter().map(|s| s.0).collect();
    let trans: Vec<State> = (0..chain.transition.size)
        .filter(|i| !abs_set.contains(i))
        .map(State)
        .collect();
    let t = trans.len();
    let a = abs.len();
    if t == 0 {
        return AbsorptionData {
            absorbing_states: abs,
            transient_states: trans,
            absorption_probs: vec![],
            expected_steps: vec![],
        };
    }
    // Build Q (t×t) and R (t×a) sub-matrices.
    let mut q = vec![vec![0.0f64; t]; t];
    let mut r = vec![vec![0.0f64; a]; t];
    for (ti, &State(si)) in trans.iter().enumerate() {
        for (tj, &State(sj)) in trans.iter().enumerate() {
            q[ti][tj] = chain.transition.get(si, sj);
        }
        for (ai, &State(sa)) in abs.iter().enumerate() {
            r[ti][ai] = chain.transition.get(si, sa);
        }
    }
    // Fundamental matrix N = (I − Q)^{−1} via Gauss-Jordan.
    let n_mat = fundamental_matrix_inv(&q, t);
    // Expected steps to absorption: t_i = (N · 1)_i.
    let expected_steps: Vec<f64> = n_mat.iter().map(|row| row.iter().sum()).collect();
    // Absorption probabilities: B = N · R.
    let mut absorption_probs = vec![vec![0.0f64; a]; t];
    for i in 0..t {
        for j in 0..a {
            for k in 0..t {
                absorption_probs[i][j] += n_mat[i][k] * r[k][j];
            }
        }
    }
    AbsorptionData {
        absorbing_states: abs,
        transient_states: trans,
        absorption_probs,
        expected_steps,
    }
}

/// Gauss-Jordan inversion of (I − Q) where Q is t×t.
fn fundamental_matrix_inv(q: &[Vec<f64>], t: usize) -> Vec<Vec<f64>> {
    // Build augmented matrix [I−Q | I].
    let mut aug = vec![vec![0.0f64; 2 * t]; t];
    for i in 0..t {
        for j in 0..t {
            aug[i][j] = if i == j { 1.0 - q[i][j] } else { -q[i][j] };
        }
        aug[i][t + i] = 1.0;
    }
    // Forward elimination.
    for col in 0..t {
        // Find pivot.
        let pivot = (col..t)
            .max_by(|&a, &b| {
                aug[a][col]
                    .abs()
                    .partial_cmp(&aug[b][col].abs())
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .unwrap_or(col);
        aug.swap(col, pivot);
        let diag = aug[col][col];
        if diag.abs() < 1e-15 {
            continue;
        }
        let inv_diag = 1.0 / diag;
        for j in 0..(2 * t) {
            aug[col][j] *= inv_diag;
        }
        for row in 0..t {
            if row == col {
                continue;
            }
            let factor = aug[row][col];
            for j in 0..(2 * t) {
                let sub = factor * aug[col][j];
                aug[row][j] -= sub;
            }
        }
    }
    // Extract right half.
    aug.iter().map(|row| row[t..].to_vec()).collect()
}

/// Compute the expected hitting time from `from` to `to` via linear system.
///
/// Returns `None` if `from == to` or the system is degenerate.
pub fn expected_hitting_time(chain: &MarkovChain, from: State, to: State) -> Option<f64> {
    if from == to {
        return Some(0.0);
    }
    let n = chain.transition.size;
    if from.0 >= n || to.0 >= n {
        return None;
    }
    // Solve system: h[i] = 1 + ∑_{j≠to} P[i][j] h[j].
    // States other than `to` form the linear system.
    let states: Vec<usize> = (0..n).filter(|&i| i != to.0).collect();
    let m = states.len();
    if m == 0 {
        return Some(0.0);
    }
    // Build [A | b] where A[i][j] = (i==j) - P[states[i]][states[j]], b[i] = 1.
    let mut aug = vec![vec![0.0f64; m + 1]; m];
    for (i, &si) in states.iter().enumerate() {
        for (j, &sj) in states.iter().enumerate() {
            aug[i][j] = if i == j { 1.0 } else { 0.0 } - chain.transition.get(si, sj);
        }
        aug[i][m] = 1.0;
    }
    // Gauss-Jordan solve.
    for col in 0..m {
        let pivot = (col..m)
            .max_by(|&a, &b| {
                aug[a][col]
                    .abs()
                    .partial_cmp(&aug[b][col].abs())
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .unwrap_or(col);
        aug.swap(col, pivot);
        let diag = aug[col][col];
        if diag.abs() < 1e-15 {
            continue;
        }
        let inv = 1.0 / diag;
        for j in 0..=m {
            aug[col][j] *= inv;
        }
        for row in 0..m {
            if row == col {
                continue;
            }
            let f = aug[row][col];
            for j in 0..=m {
                let sub = f * aug[col][j];
                aug[row][j] -= sub;
            }
        }
    }
    // Find index of `from` in states.
    let idx = states.iter().position(|&s| s == from.0)?;
    Some(aug[idx][m])
}

/// Simulate a random walk for `steps` additional steps using the given `seed`.
///
/// Returns the updated walk with positions appended.
pub fn simulate_walk(walk: &RandomWalk, steps: usize, seed: u64) -> Vec<Vec<i64>> {
    let d = walk.dimension;
    let mut lcg = super::types::Lcg::new(seed);
    let last = walk.steps.last().cloned().unwrap_or_else(|| vec![0i64; d]);
    let mut positions = walk.steps.clone();
    let mut cur = last;
    for _ in 0..steps {
        let mut next = cur.clone();
        match &walk.step_distribution {
            WalkDistribution::Simple => {
                for dim_val in next.iter_mut() {
                    *dim_val += lcg.next_step() as i64;
                }
            }
            WalkDistribution::Lazy { stay_prob } => {
                if lcg.next_f64() >= *stay_prob {
                    for dim_val in next.iter_mut() {
                        *dim_val += lcg.next_step() as i64;
                    }
                }
            }
            WalkDistribution::Biased { bias } => {
                let u = lcg.next_f64();
                let mut cumulative = 0.0;
                for (dim_idx, &p) in bias.iter().enumerate() {
                    cumulative += p;
                    if u < cumulative {
                        if dim_idx < d {
                            next[dim_idx] += 1;
                        }
                        break;
                    }
                    if dim_idx + 1 < bias.len() {
                        let p2 = bias[dim_idx + 1];
                        cumulative += p2;
                        if u < cumulative {
                            if dim_idx < d {
                                next[dim_idx] -= 1;
                            }
                            break;
                        }
                    }
                }
            }
        }
        positions.push(next.clone());
        cur = next;
    }
    positions
}

/// Simulate Poisson arrivals with rate `rate` in `[0, duration]` using `seed`.
pub fn poisson_arrivals(rate: f64, duration: f64, seed: u64) -> PoissonProcess {
    let mut lcg = super::types::Lcg::new(seed);
    let mut arrivals = Vec::new();
    let mut t = 0.0f64;
    loop {
        let u = lcg.next_f64().max(1e-15);
        t += (-u.ln()) / rate;
        if t > duration {
            break;
        }
        arrivals.push(t);
    }
    PoissonProcess::new(rate, arrivals)
}

/// Simulate a Brownian motion path B_0, B_{dt}, ..., B_{steps·dt} with given variance per step.
pub fn brownian_motion_path(variance: f64, steps: usize, seed: u64) -> BrownianMotion {
    let dt = 1.0;
    let std_dev = variance.sqrt();
    let mut lcg = super::types::Lcg::new(seed);
    let mut path = Vec::with_capacity(steps + 1);
    let mut b = 0.0f64;
    path.push(b);
    for _ in 0..steps {
        b += lcg.next_normal() * std_dev;
        path.push(b);
    }
    BrownianMotion::new(dt, path)
}

#[cfg(test)]
mod markov_tests {
    use super::*;

    fn two_state_chain(p: f64, q: f64) -> MarkovChain {
        let data = vec![vec![1.0 - p, p], vec![q, 1.0 - q]];
        let tm = TransitionMatrix::new(data);
        MarkovChain::new(tm, vec![0.5, 0.5], vec!["A".into(), "B".into()])
    }

    #[test]
    fn test_is_stochastic_valid() {
        let chain = two_state_chain(0.3, 0.4);
        assert!(is_stochastic(&chain.transition));
    }

    #[test]
    fn test_is_stochastic_invalid() {
        let data = vec![vec![0.5, 0.3], vec![0.4, 0.6]];
        let tm = TransitionMatrix::new(data);
        assert!(!is_stochastic(&tm));
    }

    #[test]
    fn test_stationary_distribution_two_state() {
        let chain = two_state_chain(0.3, 0.4);
        let stat = stationary_distribution(&chain).expect("should converge");
        // Exact: π_A = q/(p+q), π_B = p/(p+q).
        let pi_a = 0.4 / (0.3 + 0.4);
        let pi_b = 0.3 / (0.3 + 0.4);
        assert!(
            (stat.probs[0] - pi_a).abs() < 1e-6,
            "π_A = {}",
            stat.probs[0]
        );
        assert!(
            (stat.probs[1] - pi_b).abs() < 1e-6,
            "π_B = {}",
            stat.probs[1]
        );
    }

    #[test]
    fn test_stationary_sums_to_one() {
        let chain = two_state_chain(0.2, 0.5);
        let stat = stationary_distribution(&chain).expect("should converge");
        let s: f64 = stat.probs.iter().sum();
        assert!((s - 1.0).abs() < 1e-9);
    }

    #[test]
    fn test_is_ergodic_true() {
        let chain = two_state_chain(0.3, 0.4);
        assert!(is_ergodic(&chain));
    }

    #[test]
    fn test_is_ergodic_absorbing_chain() {
        let data = vec![vec![1.0, 0.0], vec![0.5, 0.5]];
        let tm = TransitionMatrix::new(data);
        let chain = MarkovChain::new(tm, vec![0.5, 0.5], vec!["Abs".into(), "T".into()]);
        assert!(!is_ergodic(&chain));
    }

    #[test]
    fn test_mixing_time_fast_mixer() {
        let chain = two_state_chain(0.4, 0.4);
        let t = mixing_time(&chain, 0.05);
        assert!(t < 200, "mixing time = {t}");
    }

    #[test]
    fn test_identify_absorbing_states() {
        let data = vec![
            vec![1.0, 0.0, 0.0],
            vec![0.5, 0.0, 0.5],
            vec![0.0, 0.0, 1.0],
        ];
        let tm = TransitionMatrix::new(data);
        let chain = MarkovChain::new(
            tm,
            vec![0.0, 1.0, 0.0],
            vec!["A".into(), "T".into(), "B".into()],
        );
        let abs = identify_absorbing_states(&chain);
        assert_eq!(abs.len(), 2);
        assert_eq!(abs[0], State(0));
        assert_eq!(abs[1], State(2));
    }

    #[test]
    fn test_absorption_analysis_basic() {
        let data = vec![
            vec![1.0, 0.0, 0.0],
            vec![0.5, 0.0, 0.5],
            vec![0.0, 0.0, 1.0],
        ];
        let tm = TransitionMatrix::new(data);
        let chain = MarkovChain::new(
            tm,
            vec![0.0, 1.0, 0.0],
            vec!["A".into(), "T".into(), "B".into()],
        );
        let data_res = absorption_analysis(&chain);
        assert_eq!(data_res.absorbing_states.len(), 2);
        assert_eq!(data_res.transient_states.len(), 1);
        // From transient state T: P(T→A) = P(T→B) = 0.5, P(T→T) = 0.
        // E[steps to absorption from T] = 1 step.
        assert!(
            (data_res.expected_steps[0] - 1.0).abs() < 1e-6,
            "expected 1, got {}",
            data_res.expected_steps[0]
        );
        // Equal probability to each absorber.
        assert!((data_res.absorption_probs[0][0] - 0.5).abs() < 1e-6);
        assert!((data_res.absorption_probs[0][1] - 0.5).abs() < 1e-6);
    }

    #[test]
    fn test_expected_hitting_time_self() {
        let chain = two_state_chain(0.3, 0.4);
        let h = expected_hitting_time(&chain, State(0), State(0));
        assert_eq!(h, Some(0.0));
    }

    #[test]
    fn test_expected_hitting_time_two_state() {
        // h(0→1): p hits 1 from 0, and q returns. E[T] = 1/p for simple cases.
        let p = 0.5;
        let chain = two_state_chain(p, p);
        let h = expected_hitting_time(&chain, State(0), State(1)).expect("should compute");
        assert!(h > 0.0, "h = {h}");
    }

    #[test]
    fn test_power_iteration_converges() {
        let chain = two_state_chain(0.3, 0.4);
        let dist = power_iteration(&chain.transition, 1000);
        let stat = stationary_distribution(&chain).expect("converge");
        assert!(total_variation_distance(&dist, &stat.probs) < 1e-5);
    }

    #[test]
    fn test_total_variation_distance_identical() {
        let p = vec![0.3, 0.3, 0.4];
        assert!(total_variation_distance(&p, &p) < 1e-15);
    }

    #[test]
    fn test_total_variation_distance_orthogonal() {
        let p = vec![1.0, 0.0];
        let q = vec![0.0, 1.0];
        assert!((total_variation_distance(&p, &q) - 1.0).abs() < 1e-15);
    }

    #[test]
    fn test_simulate_walk_simple() {
        let walk = RandomWalk::new(2, WalkDistribution::Simple);
        let result = simulate_walk(&walk, 100, 42);
        assert_eq!(result.len(), 101); // initial + 100 steps
    }

    #[test]
    fn test_simulate_walk_lazy() {
        let walk = RandomWalk::new(1, WalkDistribution::Lazy { stay_prob: 0.5 });
        let result = simulate_walk(&walk, 50, 99);
        assert_eq!(result.len(), 51);
    }

    #[test]
    fn test_poisson_arrivals_sorted() {
        let pp = poisson_arrivals(3.0, 10.0, 7);
        for i in 1..pp.arrivals.len() {
            assert!(pp.arrivals[i] > pp.arrivals[i - 1]);
        }
    }

    #[test]
    fn test_poisson_arrivals_within_duration() {
        let pp = poisson_arrivals(2.0, 5.0, 13);
        for &t in &pp.arrivals {
            assert!(t <= 5.0, "arrival {t} exceeds duration");
        }
    }

    #[test]
    fn test_brownian_motion_path_starts_at_zero() {
        let bm = brownian_motion_path(1.0, 200, 42);
        assert_eq!(bm.path.len(), 201);
        assert_eq!(bm.path[0], 0.0);
    }

    #[test]
    fn test_brownian_motion_path_quadratic_variation() {
        let bm = brownian_motion_path(1.0, 1000, 42);
        let qv = bm.quadratic_variation();
        // Quadratic variation ≈ n * variance = 1000.
        assert!(qv > 500.0 && qv < 2000.0, "quadratic variation = {qv}");
    }

    #[test]
    fn test_transition_matrix_get_set() {
        let mut tm = TransitionMatrix::identity(3);
        tm.set(0, 1, 0.3);
        tm.set(0, 0, 0.7);
        assert!((tm.get(0, 0) - 0.7).abs() < 1e-15);
        assert!((tm.get(0, 1) - 0.3).abs() < 1e-15);
        assert!((tm.get(1, 1) - 1.0).abs() < 1e-15);
    }

    #[test]
    fn test_state_ordering() {
        let s0 = State(0);
        let s1 = State(1);
        assert!(s0 < s1);
        assert_eq!(s0, State(0));
    }

    #[test]
    fn test_poisson_process_count_by() {
        let pp = poisson_arrivals(10.0, 5.0, 1);
        let total = pp.count_by(5.0);
        assert_eq!(total, pp.arrivals.len());
        let half = pp.count_by(2.5);
        assert!(half <= total);
    }

    #[test]
    fn test_markov_chain_uniform_start() {
        let data = vec![vec![0.5, 0.5], vec![0.5, 0.5]];
        let tm = TransitionMatrix::new(data);
        let chain = MarkovChain::with_uniform_start(tm, vec!["X".into(), "Y".into()]);
        assert!((chain.initial[0] - 0.5).abs() < 1e-15);
    }

    #[test]
    fn test_stationary_distribution_empty() {
        let tm = TransitionMatrix::new(vec![]);
        let chain = MarkovChain::new(tm, vec![], vec![]);
        assert!(stationary_distribution(&chain).is_none());
    }
}
