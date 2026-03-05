//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)
#![allow(clippy::items_after_test_module)]

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::functions::*;
use super::types::{Distribution, Hmc, ImportanceSampler, MeanFieldVI, ParticleFilter, Rng};

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_distribution_normal_log_density() {
        let d = Distribution::Normal {
            mean: 0.0,
            std: 1.0,
        };
        let lp = d.log_density(0.0);
        assert!((lp - (-0.5 * (2.0 * std::f64::consts::PI).ln())).abs() < 1e-8);
    }
    #[test]
    fn test_distribution_sample_bernoulli() {
        let mut rng = Rng::new(42);
        let d = Distribution::Bernoulli { p: 0.7 };
        let n = 2000;
        let ones = (0..n).filter(|_| d.sample(&mut rng) == 1.0).count();
        let frac = ones as f64 / n as f64;
        assert!((frac - 0.7).abs() < 0.05, "Bernoulli(0.7) fraction: {frac}");
    }
    #[test]
    fn test_importance_sampling_mean() {
        let mut is = ImportanceSampler::new(5000, 99);
        let est = is.estimate(
            |x| x * x,
            |rng| rng.normal_mv(0.0, 2.0),
            |x| {
                let lp = -0.5 * x * x - 0.5 * (2.0 * std::f64::consts::PI).ln();
                let lq = -0.5 * (x / 2.0).powi(2)
                    - (2.0_f64).ln()
                    - 0.5 * (2.0 * std::f64::consts::PI).ln();
                lp - lq
            },
        );
        assert!(
            (est - 1.0).abs() < 0.1,
            "IS estimate of E[x^2] should be near 1.0, got {est}"
        );
    }
    #[test]
    fn test_particle_filter_constant_state() {
        let mut pf = ParticleFilter::new(500, 7);
        let obs: Vec<f64> = vec![3.0; 10];
        let means = pf.filter_mean(
            &obs,
            |rng| rng.normal_mv(3.0, 1.0),
            |x, rng| rng.normal_mv(x, 0.1),
            |x, y| {
                let z = (x - y) / 0.5;
                -0.5 * z * z
            },
        );
        let last = *means.last().expect("last should succeed");
        assert!(
            (last - 3.0).abs() < 1.0,
            "PF mean should be near 3.0, got {last}"
        );
    }
    #[test]
    fn test_hmc_samples_normal() {
        let mut hmc = Hmc::new(0.2, 5, 1337);
        let samples = hmc.sample(
            vec![0.0],
            1000,
            |q| -0.5 * (q[0] - 2.0).powi(2),
            |q| vec![-(q[0] - 2.0)],
        );
        let mean = samples.iter().map(|s| s[0]).sum::<f64>() / samples.len() as f64;
        assert!(
            (mean - 2.0).abs() < 0.3,
            "HMC mean should be near 2.0, got {mean}"
        );
    }
    #[test]
    fn test_mean_field_vi_converges() {
        let mut vi = MeanFieldVI::new(1, 0.1, 10, 42);
        let elbo_hist = vi.fit(|z| -0.5 * (z[0] - 5.0).powi(2), 200);
        assert!(
            (vi.mu[0] - 5.0).abs() < 1.5,
            "VI mean should converge near 5.0, got {}",
            vi.mu[0]
        );
        assert!(elbo_hist.last().expect("last should succeed").is_finite());
    }
    #[test]
    fn test_build_probabilistic_programming_env() {
        let mut env = Environment::new();
        build_probabilistic_programming_env(&mut env).expect("env build failed");
        assert!(env.get(&Name::str("Measure")).is_some());
        assert!(env.get(&Name::str("ProbabilityMonad")).is_some());
        assert!(env.get(&Name::str("elbo_lower_bound")).is_some());
        assert!(env.get(&Name::str("hmc_invariant")).is_some());
    }
    #[test]
    fn test_effective_sample_size() {
        let n = 100;
        let log_weights = vec![0.0f64; n];
        let ess = ImportanceSampler::effective_sample_size(&log_weights);
        assert!(
            (ess - n as f64).abs() < 1.0,
            "ESS for uniform weights should be {n}, got {ess}"
        );
        let mut lw = vec![f64::NEG_INFINITY; n];
        lw[0] = 0.0;
        let ess2 = ImportanceSampler::effective_sample_size(&lw);
        assert!(ess2 < 2.0, "degenerate ESS should be ≈ 1, got {ess2}");
    }
}
/// Natural log of the Beta function: ln B(a,b) = lgamma(a) + lgamma(b) - lgamma(a+b).
pub(super) fn ln_beta(a: f64, b: f64) -> f64 {
    lgamma(a) + lgamma(b) - lgamma(a + b)
}
/// Stirling approximation of ln Γ(x).
pub(super) fn lgamma(x: f64) -> f64 {
    if x < 0.5 {
        std::f64::consts::PI.ln() - (std::f64::consts::PI * x).sin().ln() - lgamma(1.0 - x)
    } else {
        let g = 7.0_f64;
        let c = [
            0.999_999_999_999_809_9_f64,
            676.5203681218851,
            -1259.1392167224028,
            771.323_428_777_653_1,
            -176.615_029_162_140_6,
            12.507343278686905,
            -0.13857109526572012,
            9.984_369_578_019_572e-6,
            1.5056327351493116e-7,
        ];
        let x = x - 1.0;
        let t = x + g + 0.5;
        let mut s = c[0];
        for i in 1..9 {
            s += c[i] / (x + i as f64);
        }
        0.5 * (2.0 * std::f64::consts::PI).ln() + (x + 0.5) * t.ln() - t + s.ln()
    }
}
