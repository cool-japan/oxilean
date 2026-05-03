//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
/// Euler-Maruyama SDE simulator: dX_t = drift(X_t) dt + diffusion(X_t) dW_t.
///
/// This can simulate any Itô SDE with time-homogeneous coefficients.
/// The same `seed` produces the same path for reproducibility.
pub struct EulerMaruyama {
    /// Drift coefficient μ(x).
    pub drift: fn(f64) -> f64,
    /// Diffusion coefficient σ(x).
    pub diffusion: fn(f64) -> f64,
}
impl EulerMaruyama {
    /// Create a new Euler-Maruyama integrator with the given coefficients.
    pub fn new(drift: fn(f64) -> f64, diffusion: fn(f64) -> f64) -> Self {
        EulerMaruyama { drift, diffusion }
    }
    /// Simulate a path starting at `x0` over `[0, t_end]` with `n_steps` steps.
    ///
    /// Returns `(time, X_t)` pairs.
    pub fn simulate(&self, x0: f64, t_end: f64, n_steps: u32, seed: u64) -> Vec<(f64, f64)> {
        let mut lcg = Lcg::new(seed);
        let dt = t_end / n_steps as f64;
        let sqrt_dt = dt.sqrt();
        let mut path = Vec::with_capacity(n_steps as usize + 1);
        let mut t = 0.0f64;
        let mut x = x0;
        path.push((t, x));
        for _ in 0..n_steps {
            let dw = lcg.next_normal() * sqrt_dt;
            x += (self.drift)(x) * dt + (self.diffusion)(x) * dw;
            t += dt;
            path.push((t, x));
        }
        path
    }
}
/// Estimates the local time L_t^a of a Brownian motion path at level a.
///
/// By the occupation time formula, L_t^a ≈ (1/ε) * ∫₀ᵗ 1_{|B_s - a| < ε} ds.
#[allow(dead_code)]
pub struct LocalTimeEstimator {
    /// The Brownian motion path (time, value) pairs.
    pub path: Vec<(f64, f64)>,
}
impl LocalTimeEstimator {
    /// Create from a Brownian motion path.
    pub fn new(path: Vec<(f64, f64)>) -> Self {
        Self { path }
    }
    /// Estimate local time at level `a` using bandwidth `epsilon`.
    ///
    /// L_t^a ≈ (1/ε) * Σ_{k} 1_{|B_k - a| < ε} Δt_k.
    pub fn estimate_local_time(&self, a: f64, epsilon: f64) -> f64 {
        if self.path.len() < 2 || epsilon <= 0.0 {
            return 0.0;
        }
        let mut total = 0.0f64;
        for k in 1..self.path.len() {
            let (t_prev, _) = self.path[k - 1];
            let (t_curr, x_curr) = self.path[k];
            let dt = t_curr - t_prev;
            if (x_curr - a).abs() < epsilon {
                total += dt;
            }
        }
        total / epsilon
    }
    /// Compute the occupation time measure: time spent in interval \[lo, hi\].
    pub fn occupation_time(&self, lo: f64, hi: f64) -> f64 {
        if self.path.len() < 2 {
            return 0.0;
        }
        let mut total = 0.0f64;
        for k in 1..self.path.len() {
            let (t_prev, _) = self.path[k - 1];
            let (t_curr, x_curr) = self.path[k];
            let dt = t_curr - t_prev;
            if x_curr >= lo && x_curr < hi {
                total += dt;
            }
        }
        total
    }
    /// Compute the quadratic variation \[B\]_t ≈ Σ |B_{t_k} - B_{t_{k-1}}|².
    pub fn quadratic_variation(&self) -> f64 {
        self.path
            .windows(2)
            .map(|w| {
                let diff = w[1].1 - w[0].1;
                diff * diff
            })
            .sum()
    }
}
/// A continuous-time Markov chain defined by its state space and rate matrix Q.
///
/// The rate matrix satisfies: q_{ij} ≥ 0 for i ≠ j, and each row sums to 0
/// (i.e., q_{ii} = −∑_{j≠i} q_{ij}).
#[derive(Debug, Clone)]
pub struct CtMarkovChain {
    pub states: Vec<String>,
    /// Q matrix: rate_matrix\[i\]\[j\] = q_{ij}.
    pub rate_matrix: Vec<Vec<f64>>,
}
impl CtMarkovChain {
    /// Construct a new CTMC with the given states and rate matrix.
    pub fn new(states: Vec<String>, rate_matrix: Vec<Vec<f64>>) -> Self {
        CtMarkovChain {
            states,
            rate_matrix,
        }
    }
    /// Approximate the stationary distribution π by power iteration on the
    /// embedded discrete-time chain (using a uniformisation / balancing heuristic).
    ///
    /// Returns `None` if the chain has no states or the matrix is degenerate.
    pub fn stationary_distribution(&self) -> Option<Vec<f64>> {
        let n = self.states.len();
        if n == 0 {
            return None;
        }
        let q_max = self
            .rate_matrix
            .iter()
            .enumerate()
            .map(|(i, row)| row.get(i).map(|v| v.abs()).unwrap_or(0.0))
            .fold(0.0f64, f64::max);
        if q_max < 1e-15 {
            return Some(vec![1.0 / n as f64; n]);
        }
        let mut p = vec![vec![0.0f64; n]; n];
        for i in 0..n {
            for j in 0..n {
                let q_ij = self.rate_matrix[i].get(j).copied().unwrap_or(0.0);
                p[i][j] = if i == j {
                    1.0 + q_ij / q_max
                } else {
                    q_ij / q_max
                };
            }
        }
        let mut pi = vec![1.0 / n as f64; n];
        for _ in 0..10_000 {
            let mut pi_new = vec![0.0f64; n];
            for j in 0..n {
                for i in 0..n {
                    pi_new[j] += pi[i] * p[i][j];
                }
            }
            let s: f64 = pi_new.iter().sum();
            if s < 1e-15 {
                return None;
            }
            for v in &mut pi_new {
                *v /= s;
            }
            pi = pi_new;
        }
        Some(pi)
    }
    /// Crude estimate of the expected hitting time from state `from` to state `to`.
    ///
    /// Returns `1 / q_{from,to}` if a direct transition exists, otherwise `f64::INFINITY`.
    pub fn expected_hitting_time(&self, from: usize, to: usize) -> f64 {
        if from == to {
            return 0.0;
        }
        let rate = self
            .rate_matrix
            .get(from)
            .and_then(|row| row.get(to))
            .copied()
            .unwrap_or(0.0);
        if rate > 1e-15 {
            1.0 / rate
        } else {
            f64::INFINITY
        }
    }
    /// Crude irreducibility check: returns `true` iff every off-diagonal entry
    /// of the rate matrix is strictly positive.
    pub fn is_irreducible(&self) -> bool {
        let n = self.states.len();
        for i in 0..n {
            for j in 0..n {
                if i == j {
                    continue;
                }
                let q = self
                    .rate_matrix
                    .get(i)
                    .and_then(|r| r.get(j))
                    .copied()
                    .unwrap_or(0.0);
                if q <= 0.0 {
                    return false;
                }
            }
        }
        true
    }
}
/// Poisson process simulator.
///
/// A Poisson process N_t with rate λ generates random arrival times.
/// Inter-arrival times are Exponential(λ).
#[derive(Debug, Clone)]
pub struct PoissonProcessSimulator {
    /// Arrival rate λ > 0.
    pub rate: f64,
}
impl PoissonProcessSimulator {
    /// Create a Poisson process with the given rate λ.
    pub fn new(rate: f64) -> Self {
        PoissonProcessSimulator { rate }
    }
    /// Simulate arrival times up to time `t_end`.
    ///
    /// Returns a vector of arrival times in increasing order.
    /// Uses the inverse transform method for Exponential inter-arrivals.
    pub fn arrival_times(&self, t_end: f64, seed: u64) -> Vec<f64> {
        let mut lcg = Lcg::new(seed);
        let mut times = Vec::new();
        let mut t = 0.0f64;
        loop {
            let u = lcg.next_f64().max(1e-15);
            t += (-u.ln()) / self.rate;
            if t > t_end {
                break;
            }
            times.push(t);
        }
        times
    }
    /// Simulate the counting process N_t as `(time, count)` pairs.
    ///
    /// Returns a path with a jump of +1 at each arrival time, sampled at `n_steps`
    /// equally-spaced time points in `[0, t_end]`.
    pub fn counting_process(&self, t_end: f64, n_steps: u32, seed: u64) -> Vec<(f64, u64)> {
        let arrivals = self.arrival_times(t_end, seed);
        let dt = t_end / n_steps as f64;
        let mut path = Vec::with_capacity(n_steps as usize + 1);
        let mut count = 0u64;
        let mut arrival_idx = 0usize;
        for k in 0..=n_steps {
            let t = k as f64 * dt;
            while arrival_idx < arrivals.len() && arrivals[arrival_idx] <= t {
                count += 1;
                arrival_idx += 1;
            }
            path.push((t, count));
        }
        path
    }
    /// Expected number of arrivals in \[0, t\]: E\[N_t\] = λt.
    pub fn expected_count(&self, t: f64) -> f64 {
        self.rate * t
    }
}
/// Compound Poisson process Y_t = Σ_{i=1}^{N_t} Z_i.
///
/// N_t is a Poisson process with rate λ, and the jump sizes Z_i are i.i.d.
/// Here we use standard normal jump sizes for illustration.
#[derive(Debug, Clone)]
pub struct CompoundPoissonProcess {
    /// Arrival rate λ > 0.
    pub rate: f64,
    /// Mean of each jump.
    pub jump_mean: f64,
    /// Standard deviation of each jump.
    pub jump_std: f64,
}
impl CompoundPoissonProcess {
    /// Create a compound Poisson process with Gaussian jumps.
    pub fn new(rate: f64, jump_mean: f64, jump_std: f64) -> Self {
        CompoundPoissonProcess {
            rate,
            jump_mean,
            jump_std,
        }
    }
    /// Simulate a path of Y_t up to `t_end`, sampled at `n_steps` equally-spaced points.
    ///
    /// Returns `(time, Y_t)` pairs.
    pub fn simulate(&self, t_end: f64, n_steps: u32, seed: u64) -> Vec<(f64, f64)> {
        let mut lcg = Lcg::new(seed);
        let poisson = PoissonProcessSimulator::new(self.rate);
        let arrivals = poisson.arrival_times(t_end, seed.wrapping_add(99_999));
        let jumps: Vec<f64> = arrivals
            .iter()
            .map(|_| self.jump_mean + self.jump_std * lcg.next_normal())
            .collect();
        let dt = t_end / n_steps as f64;
        let mut path = Vec::with_capacity(n_steps as usize + 1);
        let mut cumulative = 0.0f64;
        let mut arrival_idx = 0usize;
        for k in 0..=n_steps {
            let t = k as f64 * dt;
            while arrival_idx < arrivals.len() && arrivals[arrival_idx] <= t {
                cumulative += jumps[arrival_idx];
                arrival_idx += 1;
            }
            path.push((t, cumulative));
        }
        path
    }
    /// Theoretical mean of Y_t: E\[Y_t\] = λ t E\[Z\].
    pub fn expected_value(&self, t: f64) -> f64 {
        self.rate * t * self.jump_mean
    }
    /// Theoretical variance of Y_t: Var\[Y_t\] = λ t E\[Z²\] = λ t (σ² + μ²).
    pub fn variance(&self, t: f64) -> f64 {
        self.rate * t * (self.jump_std * self.jump_std + self.jump_mean * self.jump_mean)
    }
}
/// Milstein scheme for SDEs: dX_t = μ(X_t) dt + σ(X_t) dW_t.
///
/// The Milstein correction adds a term ½ σ σ' ((ΔW)² - Δt) to achieve
/// strong order 1.0 convergence (vs. Euler-Maruyama's order 0.5).
#[allow(dead_code)]
pub struct MilsteinScheme {
    /// Drift coefficient μ(x).
    pub drift: fn(f64) -> f64,
    /// Diffusion coefficient σ(x).
    pub diffusion: fn(f64) -> f64,
    /// Derivative of diffusion σ'(x).
    pub diffusion_deriv: fn(f64) -> f64,
}
impl MilsteinScheme {
    /// Create a new Milstein integrator.
    pub fn new(
        drift: fn(f64) -> f64,
        diffusion: fn(f64) -> f64,
        diffusion_deriv: fn(f64) -> f64,
    ) -> Self {
        MilsteinScheme {
            drift,
            diffusion,
            diffusion_deriv,
        }
    }
    /// Simulate a path starting at `x0` over `[0, t_end]` with `n_steps` steps.
    ///
    /// Returns `(time, X_t)` pairs.
    pub fn simulate(&self, x0: f64, t_end: f64, n_steps: u32, seed: u64) -> Vec<(f64, f64)> {
        let mut lcg = Lcg::new(seed);
        let dt = t_end / n_steps as f64;
        let sqrt_dt = dt.sqrt();
        let mut path = Vec::with_capacity(n_steps as usize + 1);
        let mut t = 0.0f64;
        let mut x = x0;
        path.push((t, x));
        for _ in 0..n_steps {
            let dw = lcg.next_normal() * sqrt_dt;
            let sig = (self.diffusion)(x);
            let sig_p = (self.diffusion_deriv)(x);
            x += (self.drift)(x) * dt + sig * dw + 0.5 * sig * sig_p * (dw * dw - dt);
            t += dt;
            path.push((t, x));
        }
        path
    }
    /// Estimate the mean E\[X_T\] using Monte Carlo with `n_paths` paths.
    pub fn monte_carlo_mean(
        &self,
        x0: f64,
        t_end: f64,
        n_steps: u32,
        n_paths: u32,
        seed: u64,
    ) -> f64 {
        if n_paths == 0 {
            return 0.0;
        }
        let total: f64 = (0..n_paths)
            .map(|i| {
                let path = self.simulate(x0, t_end, n_steps, seed.wrapping_add(i as u64));
                path.last().map(|&(_, x)| x).unwrap_or(x0)
            })
            .sum();
        total / n_paths as f64
    }
}
/// The Heston stochastic volatility model:
///   dS = μ S dt + √V S dW₁
///   dV = κ(θ - V) dt + ξ √V dW₂
/// with correlation corr(dW₁, dW₂) = ρ.
///
/// Uses Euler-Maruyama discretization.
#[allow(dead_code)]
pub struct HestonModel {
    /// Drift μ of the asset.
    pub mu: f64,
    /// Mean reversion speed κ > 0.
    pub kappa: f64,
    /// Long-run variance θ > 0.
    pub theta: f64,
    /// Vol-of-vol ξ > 0.
    pub xi: f64,
    /// Correlation ρ ∈ (-1, 1).
    pub rho: f64,
}
impl HestonModel {
    /// Create a new Heston model.
    pub fn new(mu: f64, kappa: f64, theta: f64, xi: f64, rho: f64) -> Self {
        HestonModel {
            mu,
            kappa,
            theta,
            xi,
            rho,
        }
    }
    /// Simulate (S_t, V_t) paths using Euler-Maruyama.
    ///
    /// Returns `Vec<(time, S_t, V_t)>`.
    pub fn simulate(
        &self,
        s0: f64,
        v0: f64,
        t_end: f64,
        n_steps: u32,
        seed: u64,
    ) -> Vec<(f64, f64, f64)> {
        let mut lcg = Lcg::new(seed);
        let dt = t_end / n_steps as f64;
        let sqrt_dt = dt.sqrt();
        let mut path = Vec::with_capacity(n_steps as usize + 1);
        let mut t = 0.0f64;
        let mut s = s0;
        let mut v = v0.max(0.0);
        path.push((t, s, v));
        for _ in 0..n_steps {
            let z1 = lcg.next_normal();
            let z2 = lcg.next_normal();
            let w1 = z1;
            let w2 = self.rho * z1 + (1.0 - self.rho * self.rho).max(0.0).sqrt() * z2;
            let sqrt_v = v.max(0.0).sqrt();
            s *= 1.0 + self.mu * dt + sqrt_v * w1 * sqrt_dt;
            v = (v + self.kappa * (self.theta - v) * dt + self.xi * sqrt_v * w2 * sqrt_dt).max(0.0);
            t += dt;
            path.push((t, s, v));
        }
        path
    }
    /// Feller condition check: 2κθ > ξ² ensures V never hits 0.
    pub fn feller_condition_satisfied(&self) -> bool {
        2.0 * self.kappa * self.theta > self.xi * self.xi
    }
    /// Long-run mean of the variance: θ.
    pub fn variance_long_run_mean(&self) -> f64 {
        self.theta
    }
}
/// Geometric Brownian Motion simulator for asset price modelling.
///
/// Models: dS = μ S dt + σ S dW.
/// The exact solution is S_t = S_0 exp((μ - σ²/2)t + σW_t).
#[derive(Debug, Clone)]
pub struct GeometricBrownianMotionProcess {
    /// Drift (expected return) μ.
    pub mu: f64,
    /// Volatility σ ≥ 0.
    pub sigma: f64,
}
impl GeometricBrownianMotionProcess {
    /// Create a GBM with given drift and volatility.
    pub fn new(mu: f64, sigma: f64) -> Self {
        GeometricBrownianMotionProcess { mu, sigma }
    }
    /// Simulate a path starting at `s0` over `[0, t_end]` with `n_steps` steps.
    ///
    /// Returns `(time, S_t)` pairs.
    pub fn simulate(&self, s0: f64, t_end: f64, n_steps: u32, seed: u64) -> Vec<(f64, f64)> {
        geometric_brownian_motion(s0, self.mu, self.sigma, t_end, n_steps, seed)
    }
    /// Expected value E\[S_t\] = S_0 exp(μ t).
    pub fn expected_value(&self, s0: f64, t: f64) -> f64 {
        s0 * (self.mu * t).exp()
    }
    /// Variance Var\[S_t\] = S_0² e^{2μt} (e^{σ²t} - 1).
    pub fn variance(&self, s0: f64, t: f64) -> f64 {
        let s0sq = s0 * s0;
        s0sq * (2.0 * self.mu * t).exp() * ((self.sigma * self.sigma * t).exp() - 1.0)
    }
}
/// Black-Scholes option pricer for European options.
///
/// Prices calls and puts using the closed-form Black-Scholes formula and
/// also computes the standard Greeks (delta, gamma, vega, theta, rho).
#[derive(Debug, Clone)]
pub struct BlackScholesPricer {
    /// Current asset price S.
    pub spot: f64,
    /// Strike price K.
    pub strike: f64,
    /// Time to expiry T (in years).
    pub time_to_expiry: f64,
    /// Risk-free interest rate r.
    pub rate: f64,
    /// Implied volatility σ.
    pub volatility: f64,
}
impl BlackScholesPricer {
    /// Create a new pricer with the given market parameters.
    pub fn new(spot: f64, strike: f64, time_to_expiry: f64, rate: f64, volatility: f64) -> Self {
        BlackScholesPricer {
            spot,
            strike,
            time_to_expiry,
            rate,
            volatility,
        }
    }
    fn d1(&self) -> f64 {
        let sqrt_t = self.time_to_expiry.sqrt();
        ((self.spot / self.strike).ln()
            + (self.rate + 0.5 * self.volatility * self.volatility) * self.time_to_expiry)
            / (self.volatility * sqrt_t)
    }
    fn d2(&self) -> f64 {
        self.d1() - self.volatility * self.time_to_expiry.sqrt()
    }
    /// European call option price.
    pub fn call_price(&self) -> f64 {
        black_scholes_call(
            self.spot,
            self.strike,
            self.time_to_expiry,
            self.rate,
            self.volatility,
        )
    }
    /// European put option price.
    pub fn put_price(&self) -> f64 {
        black_scholes_put(
            self.spot,
            self.strike,
            self.time_to_expiry,
            self.rate,
            self.volatility,
        )
    }
    /// Delta for a call: ∂C/∂S = N(d₁).
    pub fn call_delta(&self) -> f64 {
        if self.time_to_expiry <= 0.0 {
            return if self.spot > self.strike { 1.0 } else { 0.0 };
        }
        standard_normal_cdf(self.d1())
    }
    /// Delta for a put: ∂P/∂S = N(d₁) − 1.
    pub fn put_delta(&self) -> f64 {
        self.call_delta() - 1.0
    }
    /// Gamma (same for call and put): ∂²C/∂S² = φ(d₁) / (S σ √T).
    pub fn gamma(&self) -> f64 {
        if self.time_to_expiry <= 0.0 {
            return 0.0;
        }
        let d1 = self.d1();
        let phi = (-0.5 * d1 * d1).exp() / (2.0 * std::f64::consts::PI).sqrt();
        phi / (self.spot * self.volatility * self.time_to_expiry.sqrt())
    }
    /// Vega (same for call and put): ∂C/∂σ = S φ(d₁) √T.
    pub fn vega(&self) -> f64 {
        if self.time_to_expiry <= 0.0 {
            return 0.0;
        }
        let d1 = self.d1();
        let phi = (-0.5 * d1 * d1).exp() / (2.0 * std::f64::consts::PI).sqrt();
        self.spot * phi * self.time_to_expiry.sqrt()
    }
    /// Rho for a call: ∂C/∂r = K T e^{-rT} N(d₂).
    pub fn call_rho(&self) -> f64 {
        if self.time_to_expiry <= 0.0 {
            return 0.0;
        }
        self.strike
            * self.time_to_expiry
            * (-self.rate * self.time_to_expiry).exp()
            * standard_normal_cdf(self.d2())
    }
    /// Rho for a put: ∂P/∂r = -K T e^{-rT} N(-d₂).
    pub fn put_rho(&self) -> f64 {
        if self.time_to_expiry <= 0.0 {
            return 0.0;
        }
        -self.strike
            * self.time_to_expiry
            * (-self.rate * self.time_to_expiry).exp()
            * standard_normal_cdf(-self.d2())
    }
    /// Implied volatility via bisection (Newton-Raphson seed).
    ///
    /// Given an observed call price, find σ such that BS(σ) = market_price.
    /// Returns `None` if the market price is outside the no-arbitrage bounds.
    pub fn implied_volatility_call(&self, market_price: f64) -> Option<f64> {
        let intrinsic =
            (self.spot - self.strike * (-self.rate * self.time_to_expiry).exp()).max(0.0);
        if market_price < intrinsic {
            return None;
        }
        let mut lo = 1e-6f64;
        let mut hi = 10.0f64;
        for _ in 0..200 {
            let mid = 0.5 * (lo + hi);
            let pricer = BlackScholesPricer {
                volatility: mid,
                ..*self
            };
            let price = pricer.call_price();
            if (price - market_price).abs() < 1e-8 {
                return Some(mid);
            }
            if price < market_price {
                lo = mid;
            } else {
                hi = mid;
            }
        }
        Some(0.5 * (lo + hi))
    }
}
/// The Variance-Gamma process: X_t = μ G_t + σ W_{G_t}
/// where G_t is a Gamma process (subordinator) and W is a Brownian motion.
///
/// VG is a popular model in mathematical finance (Madan-Seneta model).
#[allow(dead_code)]
pub struct VarianceGammaProcess {
    /// Drift parameter μ (asymmetry).
    pub mu: f64,
    /// Volatility parameter σ.
    pub sigma: f64,
    /// Variance rate ν of the Gamma subordinator.
    pub nu: f64,
}
impl VarianceGammaProcess {
    /// Create a VG process with parameters (μ, σ, ν).
    pub fn new(mu: f64, sigma: f64, nu: f64) -> Self {
        VarianceGammaProcess { mu, sigma, nu }
    }
    /// Simulate a VG path starting at `x0` over `[0, t_end]` with `n_steps` steps.
    ///
    /// At each step, sample dG ~ Gamma(dt/ν, 1/ν), then X += μ dG + σ √(dG) Z.
    pub fn simulate(&self, x0: f64, t_end: f64, n_steps: u32, seed: u64) -> Vec<(f64, f64)> {
        let mut lcg = Lcg::new(seed);
        let dt = t_end / n_steps as f64;
        let mut path = Vec::with_capacity(n_steps as usize + 1);
        let mut t = 0.0f64;
        let mut x = x0;
        path.push((t, x));
        for _ in 0..n_steps {
            let dg = sp_ext_gamma_sample(dt / self.nu, 1.0 / self.nu, &mut lcg);
            let z = lcg.next_normal();
            x += self.mu * dg + self.sigma * dg.sqrt() * z;
            t += dt;
            path.push((t, x));
        }
        path
    }
    /// Theoretical mean of X_t: E\[X_t\] = x0 + μ t.
    pub fn theoretical_mean(&self, x0: f64, t: f64) -> f64 {
        x0 + self.mu * t
    }
    /// Theoretical variance of X_t: Var\[X_t\] = σ² t + μ² ν t.
    pub fn theoretical_variance(&self, t: f64) -> f64 {
        self.sigma * self.sigma * t + self.mu * self.mu * self.nu * t
    }
    /// The VG characteristic exponent ψ(u) = log E[e^{iuX_1}].
    pub fn characteristic_exponent(&self, u: f64) -> f64 {
        let a = 1.0 - self.mu * self.nu * u * u + 0.5 * self.sigma * self.sigma * self.nu * u * u;
        let b = self.mu * self.nu * u;
        let modulus_sq = a * a + b * b;
        if modulus_sq <= 0.0 {
            return 0.0;
        }
        -0.5 * modulus_sq.ln() / self.nu
    }
}
/// Ornstein-Uhlenbeck process simulator.
///
/// Models mean-reverting dynamics: dX = θ(μ - X) dt + σ dW.
/// Useful for interest rate models (Vasicek) and volatility models.
#[derive(Debug, Clone)]
pub struct OrnsteinUhlenbeckProcess {
    /// Mean reversion speed θ > 0.
    pub theta: f64,
    /// Long-run mean μ.
    pub mean: f64,
    /// Volatility σ ≥ 0.
    pub sigma: f64,
}
impl OrnsteinUhlenbeckProcess {
    /// Create an OU process with given parameters.
    pub fn new(theta: f64, mean: f64, sigma: f64) -> Self {
        OrnsteinUhlenbeckProcess { theta, mean, sigma }
    }
    /// Simulate a path starting at `x0` over `[0, t_end]` with `n_steps` steps.
    ///
    /// Returns `(time, X_t)` pairs using exact conditional update.
    pub fn simulate(&self, x0: f64, t_end: f64, n_steps: u32, seed: u64) -> Vec<(f64, f64)> {
        ornstein_uhlenbeck(x0, self.theta, self.mean, self.sigma, t_end, n_steps, seed)
    }
    /// Long-run variance: σ² / (2θ).
    pub fn stationary_variance(&self) -> f64 {
        self.sigma * self.sigma / (2.0 * self.theta)
    }
    /// Long-run standard deviation: σ / √(2θ).
    pub fn stationary_std(&self) -> f64 {
        self.stationary_variance().sqrt()
    }
}
/// A subordinated process: X_{T_t} where T is a subordinator.
///
/// A subordinator is a non-decreasing Lévy process. We model it via a
/// gamma process approximation (increments ~ Gamma(a*dt, b)).
#[allow(dead_code)]
pub struct SubordinatedProcess {
    /// The base process drift.
    pub base_drift: f64,
    /// The base process volatility.
    pub base_sigma: f64,
    /// Gamma process parameter a (shape rate).
    pub gamma_a: f64,
    /// Gamma process parameter b (scale).
    pub gamma_b: f64,
}
impl SubordinatedProcess {
    /// Create a new subordinated process.
    pub fn new(base_drift: f64, base_sigma: f64, gamma_a: f64, gamma_b: f64) -> Self {
        SubordinatedProcess {
            base_drift,
            base_sigma,
            gamma_a,
            gamma_b,
        }
    }
    /// Simulate a path of the subordinated process over `[0, t_end]` with `n_steps` steps.
    ///
    /// Returns `(time, X_{T_t})` pairs.
    pub fn simulate(&self, x0: f64, t_end: f64, n_steps: u32, seed: u64) -> Vec<(f64, f64)> {
        let mut lcg = Lcg::new(seed);
        let dt = t_end / n_steps as f64;
        let mut path = Vec::with_capacity(n_steps as usize + 1);
        let mut t = 0.0f64;
        let mut x = x0;
        path.push((t, x));
        for _ in 0..n_steps {
            let d_sub = sp_ext_gamma_sample(self.gamma_a * dt, self.gamma_b, &mut lcg);
            let dw = lcg.next_normal() * d_sub.sqrt();
            x += self.base_drift * d_sub + self.base_sigma * dw;
            t += dt;
            path.push((t, x));
        }
        path
    }
    /// Theoretical mean E\[X_t\] = x0 + base_drift * gamma_a/gamma_b * t.
    pub fn theoretical_mean(&self, x0: f64, t: f64) -> f64 {
        x0 + self.base_drift * (self.gamma_a / self.gamma_b) * t
    }
    /// Theoretical variance Var\[X_t\] for subordinated Brownian motion.
    pub fn theoretical_variance(&self, t: f64) -> f64 {
        let e_sub = self.gamma_a / self.gamma_b * t;
        let var_sub = self.gamma_a / (self.gamma_b * self.gamma_b) * t;
        self.base_sigma * self.base_sigma * e_sub + self.base_drift * self.base_drift * var_sub
    }
}
/// Linear congruential generator (Knuth / glibc parameters).
/// Returns values in [0, 2^32).
pub(super) struct Lcg {
    state: u64,
}
impl Lcg {
    pub(super) fn new(seed: u64) -> Self {
        Lcg {
            state: seed.wrapping_add(1),
        }
    }
    /// Advance one step and return the next u32 value.
    pub(super) fn next_u32(&mut self) -> u32 {
        self.state = self
            .state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        (self.state >> 33) as u32
    }
    /// Return a uniform float in [0, 1).
    pub(super) fn next_f64(&mut self) -> f64 {
        self.next_u32() as f64 / (u32::MAX as f64 + 1.0)
    }
    /// Return ±1 with equal probability.
    pub(super) fn next_step(&mut self) -> f64 {
        if self.next_u32() & 1 == 0 {
            1.0
        } else {
            -1.0
        }
    }
    /// Box-Muller transform: return a standard normal sample.
    pub(super) fn next_normal(&mut self) -> f64 {
        let u1 = self.next_f64().max(1e-15);
        let u2 = self.next_f64();
        (-2.0 * u1.ln()).sqrt() * (2.0 * std::f64::consts::PI * u2).cos()
    }
}

// ── New Markov Chain / Stochastic Analysis Types ──────────────────────────────

/// A state index in a discrete Markov chain.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct State(pub usize);

/// Row-stochastic transition matrix P where P\[i\]\[j\] = P(next = j | current = i).
#[derive(Debug, Clone)]
pub struct TransitionMatrix {
    /// Flattened row-major storage of the matrix.
    pub data: Vec<Vec<f64>>,
    /// Number of states.
    pub size: usize,
}

impl TransitionMatrix {
    /// Create a new transition matrix.  `data\[i\]\[j\]` must give P(j | i).
    pub fn new(data: Vec<Vec<f64>>) -> Self {
        let size = data.len();
        TransitionMatrix { data, size }
    }

    /// Get the transition probability P(j | i).
    pub fn get(&self, i: usize, j: usize) -> f64 {
        self.data
            .get(i)
            .and_then(|row| row.get(j))
            .copied()
            .unwrap_or(0.0)
    }

    /// Set the transition probability P(j | i).
    pub fn set(&mut self, i: usize, j: usize, val: f64) {
        if let Some(row) = self.data.get_mut(i) {
            if let Some(cell) = row.get_mut(j) {
                *cell = val;
            }
        }
    }

    /// Create the n×n identity matrix (as a transition matrix: all self-loops).
    pub fn identity(n: usize) -> Self {
        let data = (0..n)
            .map(|i| (0..n).map(|j| if i == j { 1.0 } else { 0.0 }).collect())
            .collect();
        TransitionMatrix { data, size: n }
    }
}

/// A discrete-time Markov chain.
#[derive(Debug, Clone)]
pub struct MarkovChain {
    /// The transition probability matrix.
    pub transition: TransitionMatrix,
    /// Initial distribution π_0.
    pub initial: Vec<f64>,
    /// Human-readable state names.
    pub state_names: Vec<String>,
}

impl MarkovChain {
    /// Construct a Markov chain with given transition matrix and initial distribution.
    pub fn new(transition: TransitionMatrix, initial: Vec<f64>, state_names: Vec<String>) -> Self {
        MarkovChain {
            transition,
            initial,
            state_names,
        }
    }

    /// Uniform initial distribution.
    pub fn with_uniform_start(transition: TransitionMatrix, state_names: Vec<String>) -> Self {
        let n = transition.size;
        let initial = vec![1.0 / n as f64; n];
        MarkovChain {
            transition,
            initial,
            state_names,
        }
    }
}

/// The stationary distribution π satisfying πP = π.
#[derive(Debug, Clone)]
pub struct StationaryDistribution {
    /// Probability of each state under the stationary distribution.
    pub probs: Vec<f64>,
}

impl StationaryDistribution {
    pub fn new(probs: Vec<f64>) -> Self {
        StationaryDistribution { probs }
    }

    /// Total variation distance to another distribution.
    pub fn tv_distance(&self, other: &[f64]) -> f64 {
        0.5 * self
            .probs
            .iter()
            .zip(other.iter())
            .map(|(a, b)| (a - b).abs())
            .sum::<f64>()
    }
}

/// Absorption data for a Markov chain with absorbing states.
#[derive(Debug, Clone)]
pub struct AbsorptionData {
    /// The absorbing states (where the chain gets trapped).
    pub absorbing_states: Vec<State>,
    /// The transient states.
    pub transient_states: Vec<State>,
    /// absorption_probs\[t\]\[a\] = P(absorbed into state a | start in transient t).
    pub absorption_probs: Vec<Vec<f64>>,
    /// expected_steps\[t\] = expected number of steps to absorption from transient t.
    pub expected_steps: Vec<f64>,
}

/// Expected hitting time from one state to another.
#[derive(Debug, Clone)]
pub struct HittingTime {
    pub from: State,
    pub to: State,
    /// E\[T_{from→to}\].
    pub expected_steps: f64,
    /// Var\[T_{from→to}\] if computable.
    pub variance: Option<f64>,
}

/// Distribution of steps in a random walk.
#[derive(Debug, Clone)]
pub enum WalkDistribution {
    /// Simple symmetric random walk: ±1 with probability 1/2 each.
    Simple,
    /// Lazy random walk: stays with probability `stay_prob`, otherwise ±1.
    Lazy { stay_prob: f64 },
    /// Biased random walk with given direction probabilities (must sum to 1).
    Biased { bias: Vec<f64> },
}

/// A multi-dimensional random walk.
#[derive(Debug, Clone)]
pub struct RandomWalk {
    /// Dimension of the walk.
    pub dimension: usize,
    /// Steps taken so far: `steps\[t\]\[d\]` is the position at time t in dimension d.
    pub steps: Vec<Vec<i64>>,
    /// The step distribution.
    pub step_distribution: WalkDistribution,
}

impl RandomWalk {
    pub fn new(dimension: usize, step_distribution: WalkDistribution) -> Self {
        let initial = vec![0i64; dimension];
        RandomWalk {
            dimension,
            steps: vec![initial],
            step_distribution,
        }
    }
}

/// A Poisson process with given rate and arrival times.
#[derive(Debug, Clone)]
pub struct PoissonProcess {
    /// Arrival rate λ > 0.
    pub rate: f64,
    /// Sorted arrival times.
    pub arrivals: Vec<f64>,
}

impl PoissonProcess {
    pub fn new(rate: f64, arrivals: Vec<f64>) -> Self {
        PoissonProcess { rate, arrivals }
    }

    /// Number of arrivals in \[0, t\].
    pub fn count_by(&self, t: f64) -> usize {
        self.arrivals.iter().filter(|&&s| s <= t).count()
    }
}

/// Discrete approximation of a Brownian motion path B_0, B_{dt}, B_{2dt}, ...
#[derive(Debug, Clone)]
pub struct BrownianMotion {
    /// Time step dt.
    pub dt: f64,
    /// B_0 = 0, B_{k·dt} = path\[k\].
    pub path: Vec<f64>,
}

impl BrownianMotion {
    pub fn new(dt: f64, path: Vec<f64>) -> Self {
        BrownianMotion { dt, path }
    }

    /// Quadratic variation \[B\]_T ≈ n·dt (should ≈ T).
    pub fn quadratic_variation(&self) -> f64 {
        self.path.windows(2).map(|w| (w[1] - w[0]).powi(2)).sum()
    }

    /// Terminal time T = n·dt.
    pub fn terminal_time(&self) -> f64 {
        (self.path.len().saturating_sub(1)) as f64 * self.dt
    }
}
