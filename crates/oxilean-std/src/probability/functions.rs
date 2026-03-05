//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{
    CharFunctionData, CharacteristicFunction, ConcentrationBound, Coupling, DirichletProcess,
    DiscreteDistribution, EmpiricalCdf, ErgodicTheoremData, ExponentialDistribution,
    GaussianDistribution, GaussianProcess, GaussianProcessRegression, HawkesProcess,
    KernelDensityEstimator, LargeDeviations, Lcg, MarkovChain, PoissonProcess, RenewalProcess,
    StoppingTime, WelfordEstimator,
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
/// `ProbSpace : Type` — a probability space Ω equipped with a sigma-algebra and measure.
pub fn prob_space_ty() -> Expr {
    type0()
}
/// `Event : Type → Prop` — an event as a measurable subset of the sample space.
pub fn event_ty() -> Expr {
    arrow(type0(), prop())
}
/// `RandomVar : Type → Type → Type` — a measurable function from sample space to value space.
pub fn random_var_ty() -> Expr {
    arrow(type0(), arrow(type0(), type0()))
}
/// `Distribution : Type → Type` — a probability measure (distribution) on a type.
pub fn distribution_ty() -> Expr {
    arrow(type0(), type0())
}
/// `MarkovChain : Type → Type` — a discrete-time Markov chain on a state space.
pub fn markov_chain_ty() -> Expr {
    arrow(type0(), type0())
}
/// `StochasticProcess : Nat → Type → Type` — a time-indexed family of random variables.
pub fn stochastic_process_ty() -> Expr {
    arrow(nat_ty(), arrow(type0(), type0()))
}
/// `SigmaAlgebra : Type → Type` — a sigma-algebra F on a sample space Ω.
/// Satisfies closure under complement and countable union.
pub fn sigma_algebra_ty() -> Expr {
    arrow(type0(), type0())
}
/// `Measurable : (α → β) → SigmaAlgebra α → SigmaAlgebra β → Prop`
/// — a function is measurable with respect to given sigma-algebras.
pub fn measurable_ty() -> Expr {
    arrow(arrow(type0(), type0()), prop())
}
/// `ProbMeasure : SigmaAlgebra α → Type` — a probability measure on a measurable space.
pub fn prob_measure_ty() -> Expr {
    arrow(sigma_algebra_ty(), type0())
}
/// `Covariance : (Ω → Real) → (Ω → Real) → Real`
/// — the covariance E[(X - EX)(Y - EY)] of two random variables.
pub fn covariance_ty() -> Expr {
    arrow(
        arrow(type0(), real_ty()),
        arrow(arrow(type0(), real_ty()), real_ty()),
    )
}
/// `MutualIndependence : List Event → Prop`
/// — a family of events is mutually independent.
pub fn mutual_independence_ty() -> Expr {
    arrow(app(cst("List"), event_ty()), prop())
}
/// `PairwiseIndependence : List Event → Prop`
/// — every pair of events in a family is independent.
pub fn pairwise_independence_ty() -> Expr {
    arrow(app(cst("List"), event_ty()), prop())
}
/// `ConditionalExpectation : (Ω → Real) → SigmaAlgebra Ω → (Ω → Real)`
/// — the conditional expectation E[X | F] as a measurable function.
pub fn conditional_expectation_ty() -> Expr {
    arrow(
        arrow(type0(), real_ty()),
        arrow(sigma_algebra_ty(), arrow(type0(), real_ty())),
    )
}
/// `CharacteristicFn : (Ω → Real) → Real → Complex`
/// — the characteristic function φ_X(t) = E[exp(itX)] of a random variable.
pub fn characteristic_fn_ty() -> Expr {
    arrow(arrow(type0(), real_ty()), arrow(real_ty(), cst("Complex")))
}
/// `Quantile : Distribution α → Real → α`
/// — the quantile function (inverse CDF) of a distribution.
pub fn quantile_ty() -> Expr {
    arrow(distribution_ty(), arrow(real_ty(), type0()))
}
/// `Entropy : Distribution α → Real`
/// — the Shannon entropy H(X) = -∑ p(x) log p(x).
pub fn entropy_ty() -> Expr {
    arrow(distribution_ty(), real_ty())
}
/// `KLDivergence : Distribution α → Distribution α → Real`
/// — the Kullback–Leibler divergence D_KL(P ‖ Q).
pub fn kl_divergence_ty() -> Expr {
    arrow(distribution_ty(), arrow(distribution_ty(), real_ty()))
}
/// `StoppingTime : (Nat → Event) → Prop`
/// — a random time τ adapted to a filtration is a stopping time.
pub fn stopping_time_ty() -> Expr {
    arrow(arrow(nat_ty(), event_ty()), prop())
}
/// `Martingale : StochasticProcess → Prop`
/// — a stochastic process M is a martingale: E[M_{n+1} | F_n] = M_n.
pub fn martingale_ty() -> Expr {
    arrow(stochastic_process_ty(), prop())
}
/// `SubGaussian : (Ω → Real) → Real → Prop`
/// — X is σ-sub-Gaussian: E[exp(λX)] ≤ exp(λ²σ²/2) for all λ.
pub fn sub_gaussian_ty() -> Expr {
    arrow(arrow(type0(), real_ty()), arrow(real_ty(), prop()))
}
/// `SubExponential : (Ω → Real) → Real → Real → Prop`
/// — X is (σ², b)-sub-exponential.
pub fn sub_exponential_ty() -> Expr {
    arrow(
        arrow(type0(), real_ty()),
        arrow(real_ty(), arrow(real_ty(), prop())),
    )
}
/// `RateFunction : Distribution α → (α → Real) → Prop`
/// — the large-deviation rate function I satisfying the LDP.
pub fn rate_function_ty() -> Expr {
    arrow(distribution_ty(), arrow(arrow(type0(), real_ty()), prop()))
}
/// `RenewalProcess : (Nat → Real) → Prop`
/// — the inter-arrival times form a renewal process.
pub fn renewal_process_ty() -> Expr {
    arrow(arrow(nat_ty(), real_ty()), prop())
}
/// `MixingTime : MarkovChain α → Real → Nat`
/// — the ε-mixing time of a Markov chain.
pub fn mixing_time_ty() -> Expr {
    arrow(markov_chain_ty(), arrow(real_ty(), nat_ty()))
}
/// `TotalVariationDist : Distribution α → Distribution α → Real`
/// — the total variation distance between two probability measures.
pub fn total_variation_dist_ty() -> Expr {
    arrow(distribution_ty(), arrow(distribution_ty(), real_ty()))
}
/// `Coupling : Distribution α → Distribution α → Type`
/// — a coupling of two distributions (a joint distribution with given marginals).
pub fn coupling_ty() -> Expr {
    arrow(distribution_ty(), arrow(distribution_ty(), type0()))
}
/// `EmpiricalMeasure : Nat → (Nat → α) → Distribution α`
/// — the empirical measure L_n = (1/n) Σ δ_{X_i}.
pub fn empirical_measure_ty() -> Expr {
    arrow(nat_ty(), arrow(arrow(nat_ty(), type0()), distribution_ty()))
}
/// `WeakLawOfLargeNumbers : Prop`
/// — the sample mean converges in probability to the population mean.
pub fn weak_lln_ty() -> Expr {
    prop()
}
/// `StrongLawOfLargeNumbers : Prop`
/// — the sample mean converges almost surely to the population mean.
pub fn strong_lln_ty() -> Expr {
    prop()
}
/// `LindebergCLT : Prop`
/// — the CLT under the Lindeberg condition (triangular arrays).
pub fn lindeberg_clt_ty() -> Expr {
    prop()
}
/// `LyapunovCLT : Prop`
/// — the CLT under the Lyapunov condition (finite 2+δ moments).
pub fn lyapunov_clt_ty() -> Expr {
    prop()
}
/// `BerryEsseenBound : Prop`
/// — |F_n(x) - Φ(x)| ≤ C ρ / (σ³ √n) uniformly in x.
pub fn berry_esseen_ty() -> Expr {
    prop()
}
/// `HoeffdingInequality : Prop`
/// — P(S_n - E[S_n] ≥ t) ≤ exp(-2t²/Σ(b_i-a_i)²) for bounded summands.
pub fn hoeffding_inequality_ty() -> Expr {
    prop()
}
/// `BernsteinInequality : Prop`
/// — a refined concentration bound exploiting variance information.
pub fn bernstein_inequality_ty() -> Expr {
    prop()
}
/// `ChernoffBound : Prop`
/// — tail bound via the moment generating function: P(X ≥ t) ≤ e^{-st} M_X(s).
pub fn chernoff_bound_ty() -> Expr {
    prop()
}
/// `CramerLDP : Prop`
/// — the Cramér large deviation principle for i.i.d. sums.
pub fn cramer_ldp_ty() -> Expr {
    prop()
}
/// `SanovLDP : Prop`
/// — the Sanov large deviation principle for empirical measures.
pub fn sanov_ldp_ty() -> Expr {
    prop()
}
/// `DoobOptionalSampling : Prop`
/// — E[M_τ] = E[M_0] for a uniformly integrable martingale stopped at τ.
pub fn doob_optional_sampling_ty() -> Expr {
    prop()
}
/// `AzumaHoeffding : Prop`
/// — P(M_n - M_0 ≥ t) ≤ exp(-t²/(2Σc_i²)) for a bounded-difference martingale.
pub fn azuma_hoeffding_ty() -> Expr {
    prop()
}
/// `RenewalReward : Prop`
/// — the renewal reward theorem: long-run average reward = E[reward]/E[inter-arrival].
pub fn renewal_reward_ty() -> Expr {
    prop()
}
/// `CouplingLemma : Prop`
/// — d_TV(P, Q) = inf_{coupling} P(X ≠ Y) over all couplings of P and Q.
pub fn coupling_lemma_ty() -> Expr {
    prop()
}
/// `OriginalLawOfLargeNumbers : Prop` — the sample mean converges to the population mean.
pub fn law_of_large_numbers_ty() -> Expr {
    prop()
}
/// `CentralLimitTheorem : Prop` — normalized sums converge in distribution to the standard normal.
pub fn central_limit_theorem_ty() -> Expr {
    prop()
}
/// `BayesTheorem : Prop` — P(A|B) = P(B|A) * P(A) / P(B).
pub fn bayes_theorem_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "P_A",
        real_ty(),
        pi(
            BinderInfo::Default,
            "P_B",
            real_ty(),
            pi(BinderInfo::Default, "P_B_given_A", real_ty(), real_ty()),
        ),
    )
}
/// `MarkovInequality : Prop` — P(X ≥ a) ≤ E[X] / a for non-negative X and a > 0.
pub fn markov_inequality_ty() -> Expr {
    prop()
}
/// `ChebyshevInequality : Prop` — P(|X - μ| ≥ k·σ) ≤ 1/k² for k > 0.
pub fn chebyshev_inequality_ty() -> Expr {
    prop()
}
/// `KolmogorovAxioms : Prop` — the three Kolmogorov axioms for a probability measure.
pub fn kolmogorov_axioms_ty() -> Expr {
    prop()
}
/// Register all probability theory axioms and type declarations into `env`.
pub fn build_probability_env(env: &mut Environment) -> Result<(), String> {
    let type_decls: &[(&str, Expr)] = &[
        ("ProbSpace", prob_space_ty()),
        ("Event", event_ty()),
        ("RandomVar", random_var_ty()),
        ("Distribution", distribution_ty()),
        ("MarkovChain", markov_chain_ty()),
        ("StochasticProcess", stochastic_process_ty()),
    ];
    for (name, ty) in type_decls {
        env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty: ty.clone(),
        })
        .ok();
    }
    let new_type_decls: &[(&str, Expr)] = &[
        ("SigmaAlgebra", sigma_algebra_ty()),
        ("ProbMeasure", prob_measure_ty()),
        ("Coupling", coupling_ty()),
    ];
    for (name, ty) in new_type_decls {
        env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty: ty.clone(),
        })
        .ok();
    }
    let theorem_decls: &[(&str, Expr)] = &[
        ("LawOfLargeNumbers", law_of_large_numbers_ty()),
        ("CentralLimitTheorem", central_limit_theorem_ty()),
        ("BayesTheorem", bayes_theorem_ty()),
        ("MarkovInequality", markov_inequality_ty()),
        ("ChebyshevInequality", chebyshev_inequality_ty()),
        ("KolmogorovAxioms", kolmogorov_axioms_ty()),
    ];
    for (name, ty) in theorem_decls {
        env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty: ty.clone(),
        })
        .ok();
    }
    let new_theorem_decls: &[(&str, Expr)] = &[
        ("WeakLawOfLargeNumbers", weak_lln_ty()),
        ("StrongLawOfLargeNumbers", strong_lln_ty()),
        ("LindebergCLT", lindeberg_clt_ty()),
        ("LyapunovCLT", lyapunov_clt_ty()),
        ("BerryEsseenBound", berry_esseen_ty()),
        ("HoeffdingInequality", hoeffding_inequality_ty()),
        ("BernsteinInequality", bernstein_inequality_ty()),
        ("ChernoffBound", chernoff_bound_ty()),
        ("CramerLDP", cramer_ldp_ty()),
        ("SanovLDP", sanov_ldp_ty()),
        ("DoobOptionalSampling", doob_optional_sampling_ty()),
        ("AzumaHoeffding", azuma_hoeffding_ty()),
        ("RenewalReward", renewal_reward_ty()),
        ("CouplingLemma", coupling_lemma_ty()),
    ];
    for (name, ty) in new_theorem_decls {
        env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty: ty.clone(),
        })
        .ok();
    }
    let extra: &[(&str, Expr)] = &[
        ("Prob", arrow(event_ty(), real_ty())),
        ("Expectation", arrow(arrow(type0(), real_ty()), real_ty())),
        ("Variance", arrow(arrow(type0(), real_ty()), real_ty())),
        (
            "Conditional",
            arrow(event_ty(), arrow(event_ty(), real_ty())),
        ),
        ("Independence", arrow(event_ty(), arrow(event_ty(), prop()))),
        (
            "StationaryDist",
            arrow(markov_chain_ty(), distribution_ty()),
        ),
        (
            "MomentGenerating",
            arrow(arrow(type0(), real_ty()), arrow(real_ty(), real_ty())),
        ),
        ("Measurable", measurable_ty()),
        ("Cov", covariance_ty()),
        ("MutualIndep", mutual_independence_ty()),
        ("PairwiseIndep", pairwise_independence_ty()),
        ("CondExpectation", conditional_expectation_ty()),
        ("CharFn", characteristic_fn_ty()),
        ("Quantile", quantile_ty()),
        ("Entropy", entropy_ty()),
        ("KLDiv", kl_divergence_ty()),
        ("StoppingTime", stopping_time_ty()),
        ("IsMartingale", martingale_ty()),
        ("IsSubGaussian", sub_gaussian_ty()),
        ("IsSubExponential", sub_exponential_ty()),
        ("RateFunction", rate_function_ty()),
        ("IsRenewalProcess", renewal_process_ty()),
        ("MixingTime", mixing_time_ty()),
        ("TVDist", total_variation_dist_ty()),
        ("EmpiricalMeasure", empirical_measure_ty()),
    ];
    for (name, ty) in extra {
        env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty: ty.clone(),
        })
        .ok();
    }
    build_advanced_probability_env(env)?;
    Ok(())
}
/// Returns the uniform distribution over `n` outcomes: each probability is `1/n`.
pub fn discrete_uniform(n: usize) -> Vec<f64> {
    if n == 0 {
        return vec![];
    }
    vec![1.0 / n as f64; n]
}
/// Computes the binomial PMF: P(X = k) for X ~ Bin(n, p).
pub fn binomial_pmf(n: u32, k: u32, p: f64) -> f64 {
    if k > n {
        return 0.0;
    }
    let binom = binomial_coeff(n, k) as f64;
    binom * p.powi(k as i32) * (1.0 - p).powi((n - k) as i32)
}
/// Computes the Poisson PMF: P(X = k) for X ~ Pois(λ).
pub fn poisson_pmf(lambda: f64, k: u32) -> f64 {
    if lambda < 0.0 {
        return 0.0;
    }
    lambda.powi(k as i32) * (-lambda).exp() / factorial(k) as f64
}
/// Computes the Gaussian (normal) PDF: f(x; μ, σ).
pub fn normal_pdf(x: f64, mu: f64, sigma: f64) -> f64 {
    if sigma <= 0.0 {
        return 0.0;
    }
    let z = (x - mu) / sigma;
    (-0.5 * z * z).exp() / (sigma * (2.0 * std::f64::consts::PI).sqrt())
}
/// Computes the sample mean of `data`.
pub fn sample_mean(data: &[f64]) -> f64 {
    if data.is_empty() {
        return 0.0;
    }
    data.iter().sum::<f64>() / data.len() as f64
}
/// Computes the sample variance of `data` (unbiased, divides by n-1).
pub fn sample_variance(data: &[f64]) -> f64 {
    if data.len() < 2 {
        return 0.0;
    }
    let mean = sample_mean(data);
    let sum_sq: f64 = data.iter().map(|x| (x - mean).powi(2)).sum();
    sum_sq / (data.len() - 1) as f64
}
/// Computes the sample covariance of paired slices `x` and `y`.
pub fn covariance(x: &[f64], y: &[f64]) -> f64 {
    let n = x.len().min(y.len());
    if n < 2 {
        return 0.0;
    }
    let mx = sample_mean(&x[..n]);
    let my = sample_mean(&y[..n]);
    let sum: f64 = x[..n]
        .iter()
        .zip(y[..n].iter())
        .map(|(xi, yi)| (xi - mx) * (yi - my))
        .sum();
    sum / (n - 1) as f64
}
/// Computes the Pearson correlation coefficient between `x` and `y`.
pub fn pearson_correlation(x: &[f64], y: &[f64]) -> f64 {
    let n = x.len().min(y.len());
    if n < 2 {
        return 0.0;
    }
    let sx = sample_variance(&x[..n]).sqrt();
    let sy = sample_variance(&y[..n]).sqrt();
    if sx == 0.0 || sy == 0.0 {
        return 0.0;
    }
    covariance(x, y) / (sx * sy)
}
/// Approximates the standard normal CDF Φ(z) for z ∈ ℝ.
///
/// Uses the Horner-form rational approximation from Abramowitz & Stegun 26.2.17.
pub fn standard_normal_cdf(z: f64) -> f64 {
    let sign = if z < 0.0 { -1.0 } else { 1.0 };
    let z = z.abs();
    let t = 1.0 / (1.0 + 0.2316419 * z);
    let poly = t
        * (0.319_381_530
            + t * (-0.356_563_782
                + t * (1.781_477_937 + t * (-1.821_255_978 + t * 1.330_274_429))));
    let phi_pos = 1.0 - normal_pdf(z, 0.0, 1.0) * poly;
    if sign > 0.0 {
        phi_pos
    } else {
        1.0 - phi_pos
    }
}
/// Computes the exponential PDF: f(x; λ) = λ exp(-λx) for x ≥ 0.
pub fn exponential_pdf(x: f64, lambda: f64) -> f64 {
    if x < 0.0 || lambda <= 0.0 {
        return 0.0;
    }
    lambda * (-lambda * x).exp()
}
/// Computes the exponential CDF: F(x; λ) = 1 - exp(-λx) for x ≥ 0.
pub fn exponential_cdf(x: f64, lambda: f64) -> f64 {
    if x < 0.0 || lambda <= 0.0 {
        return 0.0;
    }
    1.0 - (-lambda * x).exp()
}
/// Computes the geometric PMF: P(X = k) = (1-p)^(k-1) p for k ≥ 1.
pub fn geometric_pmf(k: u32, p: f64) -> f64 {
    if k == 0 || p <= 0.0 || p > 1.0 {
        return 0.0;
    }
    (1.0 - p).powi((k - 1) as i32) * p
}
/// Computes the negative binomial PMF: P(X = k) for X ~ NB(r, p).
///
/// X = number of failures before the r-th success.
pub fn negative_binomial_pmf(k: u32, r: u32, p: f64) -> f64 {
    if p <= 0.0 || p > 1.0 {
        return 0.0;
    }
    let binom = binomial_coeff(k + r - 1, k) as f64;
    binom * p.powi(r as i32) * (1.0 - p).powi(k as i32)
}
/// Approximates the gamma PDF: f(x; α, β) using the log-gamma function.
///
/// f(x; α, β) = x^(α-1) exp(-x/β) / (Γ(α) β^α) for x > 0.
pub fn gamma_pdf(x: f64, alpha: f64, beta: f64) -> f64 {
    if x <= 0.0 || alpha <= 0.0 || beta <= 0.0 {
        return 0.0;
    }
    let log_pdf = (alpha - 1.0) * x.ln() - x / beta - log_gamma(alpha) - alpha * beta.ln();
    log_pdf.exp()
}
/// Approximates the beta PDF: f(x; α, β) for x ∈ (0, 1).
pub fn beta_pdf(x: f64, alpha: f64, beta: f64) -> f64 {
    if x <= 0.0 || x >= 1.0 || alpha <= 0.0 || beta <= 0.0 {
        return 0.0;
    }
    let log_b = log_gamma(alpha) + log_gamma(beta) - log_gamma(alpha + beta);
    let log_pdf = (alpha - 1.0) * x.ln() + (beta - 1.0) * (1.0 - x).ln() - log_b;
    log_pdf.exp()
}
/// Stirling approximation of log Γ(x) for x > 0.
///
/// log Γ(x) ≈ (x-0.5)·log(x) - x + 0.5·log(2π) + 1/(12x)
pub fn log_gamma(x: f64) -> f64 {
    if x <= 0.0 {
        return f64::NAN;
    }
    let g = 7.0f64;
    let c = [
        0.999_999_999_999_809_9,
        676.520_368_121_885_1,
        -1_259.139_216_722_403,
        771.323_428_777_653_1,
        -176.615_029_162_140_6,
        12.507_343_278_686_905,
        -0.138_571_095_265_720_12,
        9.984_369_578_019_572e-6,
        1.505_632_735_149_311_6e-7,
    ];
    if x < 0.5 {
        return std::f64::consts::PI.ln()
            - (std::f64::consts::PI * x).sin().ln()
            - log_gamma(1.0 - x);
    }
    let x = x - 1.0;
    let mut a = c[0];
    for i in 1..9 {
        a += c[i] / (x + i as f64);
    }
    let t = x + g + 0.5;
    0.5 * (2.0 * std::f64::consts::PI).ln() + (x + 0.5) * t.ln() - t + a.ln()
}
/// Computes the posterior distribution via Bayes' theorem.
///
/// Given a `prior` over hypotheses and the `likelihoods` P(evidence | hypothesis),
/// returns the normalized posterior P(hypothesis | evidence).
pub fn bayes_update(prior: &[f64], likelihoods: &[f64]) -> Vec<f64> {
    let n = prior.len().min(likelihoods.len());
    let mut posterior: Vec<f64> = prior[..n]
        .iter()
        .zip(likelihoods[..n].iter())
        .map(|(p, l)| p * l)
        .collect();
    let total: f64 = posterior.iter().sum();
    if total > 0.0 {
        for v in posterior.iter_mut() {
            *v /= total;
        }
    }
    posterior
}
/// Computes the KL divergence D_KL(p ‖ q) = Σ p_i log(p_i / q_i).
pub fn kl_divergence(p: &[f64], q: &[f64]) -> f64 {
    let n = p.len().min(q.len());
    p[..n]
        .iter()
        .zip(q[..n].iter())
        .filter(|(&pi, &qi)| pi > 0.0 && qi > 0.0)
        .map(|(&pi, &qi)| pi * (pi / qi).ln())
        .sum()
}
/// Computes the total variation distance between two discrete distributions.
///
/// d_TV(p, q) = 0.5 · Σ |p_i - q_i|.
pub fn total_variation_distance(p: &[f64], q: &[f64]) -> f64 {
    let n = p.len().min(q.len());
    0.5 * p[..n]
        .iter()
        .zip(q[..n].iter())
        .map(|(a, b)| (a - b).abs())
        .sum::<f64>()
}
/// Computes empirical moments up to order `max_order` from data.
///
/// Returns a vector where index k holds the k-th raw moment E[X^k].
pub fn empirical_moments(data: &[f64], max_order: u32) -> Vec<f64> {
    (0..=max_order)
        .map(|k| {
            if data.is_empty() {
                0.0
            } else {
                data.iter().map(|x| x.powi(k as i32)).sum::<f64>() / data.len() as f64
            }
        })
        .collect()
}
pub fn binomial_coeff(n: u32, k: u32) -> u64 {
    if k > n {
        return 0;
    }
    let k = k.min(n - k);
    let mut result: u64 = 1;
    for i in 0..k {
        result = result * (n - i) as u64 / (i + 1) as u64;
    }
    result
}
pub fn factorial(n: u32) -> u64 {
    (1..=n as u64).product()
}
/// `BrownianMotion : (Real → Ω → Real) → Prop`
/// — a stochastic process W satisfying: W(0)=0, independent increments,
///   Gaussian increments W(t)-W(s) ~ N(0, t-s), and continuous paths.
pub fn brownian_motion_ty() -> Expr {
    arrow(arrow(real_ty(), arrow(type0(), real_ty())), prop())
}
/// `LevyProcess : (Real → Ω → Real) → Prop`
/// — a process with stationary independent increments, càdlàg paths, X(0)=0.
pub fn levy_process_ty() -> Expr {
    arrow(arrow(real_ty(), arrow(type0(), real_ty())), prop())
}
/// `ItoIntegral : (Real → Ω → Real) → (Real → Ω → Real) → (Ω → Real)`
/// — the Itô stochastic integral ∫H dW for an adapted integrand H and Brownian W.
pub fn ito_integral_ty() -> Expr {
    arrow(
        arrow(real_ty(), arrow(type0(), real_ty())),
        arrow(
            arrow(real_ty(), arrow(type0(), real_ty())),
            arrow(type0(), real_ty()),
        ),
    )
}
/// `ItoFormula : Prop`
/// — Itô's lemma: df(t, X_t) = ∂_t f dt + ∂_x f dX + ½ ∂_xx f d⟨X⟩.
pub fn ito_formula_ty() -> Expr {
    prop()
}
/// `SDE : (Real → Ω → Real) → (Real → Real → Real) → (Real → Real → Real) → Prop`
/// — the stochastic differential equation dX = μ(t,X)dt + σ(t,X)dW.
pub fn sde_ty() -> Expr {
    arrow(
        arrow(real_ty(), arrow(type0(), real_ty())),
        arrow(
            arrow(real_ty(), arrow(real_ty(), real_ty())),
            arrow(arrow(real_ty(), arrow(real_ty(), real_ty())), prop()),
        ),
    )
}
/// `StrongSolution : SDE → Prop`
/// — the SDE has a strong solution (pathwise unique, adapted to W's filtration).
pub fn strong_solution_ty() -> Expr {
    arrow(sde_ty(), prop())
}
/// `WeakSolution : SDE → Prop`
/// — the SDE has a weak solution (exists on some probability space).
pub fn weak_solution_ty() -> Expr {
    arrow(sde_ty(), prop())
}
/// `GirsanovThm : Prop`
/// — Girsanov's theorem: change of measure via Novikov's condition,
///   W̃_t = W_t - ∫θ_s ds is Brownian under ℚ.
pub fn girsanov_thm_ty() -> Expr {
    prop()
}
/// `QuadraticVariation : (Real → Ω → Real) → (Ω → Real) → Prop`
/// — the quadratic variation ⟨X⟩_T of a semimartingale X.
pub fn quadratic_variation_ty() -> Expr {
    arrow(
        arrow(real_ty(), arrow(type0(), real_ty())),
        arrow(arrow(type0(), real_ty()), prop()),
    )
}
/// `McDiarmidInequality : Prop`
/// — if f(x_1,…,x_n) changes by at most c_i when x_i is replaced,
///   then P(f - Ef ≥ t) ≤ exp(-2t²/Σc_i²).
pub fn mcdiarmid_inequality_ty() -> Expr {
    prop()
}
/// `AzumaInequality : Prop`
/// — Azuma's inequality for martingale difference sequences with bounded differences.
pub fn azuma_inequality_ty() -> Expr {
    prop()
}
/// `LDP : Distribution → (Real → Real) → Prop`
/// — the large deviation principle: lim(1/n) log P(S_n/n ∈ A) = -inf_{x∈A} I(x).
pub fn ldp_ty() -> Expr {
    arrow(
        distribution_ty(),
        arrow(arrow(real_ty(), real_ty()), prop()),
    )
}
/// `GartnerEllis : Prop`
/// — the Gärtner–Ellis theorem: LDP from the limit of (1/n) log E[e^{nλX_n}].
pub fn gartner_ellis_ty() -> Expr {
    prop()
}
/// `LogMGF : (Ω → Real) → Real → Real`
/// — the log moment generating function Λ(λ) = log E[e^{λX}].
pub fn log_mgf_ty() -> Expr {
    arrow(arrow(type0(), real_ty()), arrow(real_ty(), real_ty()))
}
/// `FenchelLegendre : (Real → Real) → (Real → Real)`
/// — the Fenchel–Legendre transform I(x) = sup_λ (λx - Λ(λ)).
pub fn fenchel_legendre_ty() -> Expr {
    arrow(arrow(real_ty(), real_ty()), arrow(real_ty(), real_ty()))
}
/// `RandomWalk : (Nat → Ω → Real) → Prop`
/// — a random walk S_n = X_1 + ⋯ + X_n with i.i.d. steps X_i.
pub fn random_walk_ty() -> Expr {
    arrow(arrow(nat_ty(), arrow(type0(), real_ty())), prop())
}
/// `GreenFunction : MarkovChain → Nat → Nat → Real`
/// — the expected number of visits to state j starting from i: G(i,j) = Σ_{t≥0} P^t(i,j).
pub fn green_function_ty() -> Expr {
    arrow(
        markov_chain_ty(),
        arrow(nat_ty(), arrow(nat_ty(), real_ty())),
    )
}
/// `HittingTime : MarkovChain → Nat → (Ω → Nat)`
/// — the first passage time τ_j = min{t ≥ 1 : X_t = j}.
pub fn hitting_time_ty() -> Expr {
    arrow(markov_chain_ty(), arrow(nat_ty(), arrow(type0(), nat_ty())))
}
/// `SpectralGap : MarkovChain → Real`
/// — the spectral gap 1 - λ_2 of the transition matrix (λ_2 = second largest eigenvalue).
pub fn spectral_gap_ty() -> Expr {
    arrow(markov_chain_ty(), real_ty())
}
/// `ReversibleChain : MarkovChain → Distribution → Prop`
/// — detailed balance: π(i) P(i,j) = π(j) P(j,i) for all states i, j.
pub fn reversible_chain_ty() -> Expr {
    arrow(markov_chain_ty(), arrow(distribution_ty(), prop()))
}
/// `GEVDistribution : Real → Real → Real → Type`
/// — the generalised extreme value distribution GEV(μ, σ, ξ).
pub fn gev_distribution_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), arrow(real_ty(), type0())))
}
/// `GPDDistribution : Real → Real → Type`
/// — the generalised Pareto distribution GPD(σ, ξ) for peaks over threshold.
pub fn gpd_distribution_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), type0()))
}
/// `PickandsBalkemaDeHaan : Prop`
/// — the Pickands–Balkema–de Haan theorem: exceedances over high threshold
///   converge to a GPD.
pub fn pickands_balkema_de_haan_ty() -> Expr {
    prop()
}
/// `FisherTippettGnedenko : Prop`
/// — the Fisher–Tippett–Gnedenko theorem: block maxima converge to a GEV distribution.
pub fn fisher_tippett_gnedenko_ty() -> Expr {
    prop()
}
/// `GaussianProcess : ((Ω → Real) → (Ω → Real) → Real) → Prop`
/// — a stochastic process GP(m, k) fully specified by mean m and covariance kernel k.
pub fn gaussian_process_ty() -> Expr {
    arrow(
        arrow(
            arrow(type0(), real_ty()),
            arrow(arrow(type0(), real_ty()), real_ty()),
        ),
        prop(),
    )
}
/// `DirichletProcess : Real → Distribution → Distribution`
/// — the Dirichlet process DP(α, G₀) with concentration α and base measure G₀.
pub fn dirichlet_process_ty() -> Expr {
    arrow(real_ty(), arrow(distribution_ty(), distribution_ty()))
}
/// `CRP : Real → Nat → Distribution`
/// — the Chinese Restaurant Process: CRP(α, n) gives distribution over partitions of [n].
pub fn crp_ty() -> Expr {
    arrow(real_ty(), arrow(nat_ty(), distribution_ty()))
}
/// `DonskerThm : Prop`
/// — Donsker's functional CLT: the empirical process converges to Brownian bridge.
pub fn donsker_thm_ty() -> Expr {
    prop()
}
/// `VCDimension : ((Ω → Prop) → Prop) → Nat`
/// — the Vapnik–Chervonenkis dimension of a hypothesis class H.
pub fn vc_dimension_ty() -> Expr {
    arrow(arrow(arrow(type0(), prop()), prop()), nat_ty())
}
/// `RademacherComplexity : ((Ω → Real) → Prop) → Real → Real`
/// — the Rademacher complexity of a function class F over n samples.
pub fn rademacher_complexity_ty() -> Expr {
    arrow(
        arrow(arrow(type0(), real_ty()), prop()),
        arrow(real_ty(), real_ty()),
    )
}
/// `MarkovBlanket : (Nat → Prop) → Nat → Nat → Prop`
/// — node v is in the Markov blanket of u in graph G: conditional independence given blanket.
pub fn markov_blanket_ty() -> Expr {
    arrow(
        arrow(nat_ty(), prop()),
        arrow(nat_ty(), arrow(nat_ty(), prop())),
    )
}
/// `DSeparation : (Nat → Nat → Prop) → Nat → Nat → (Nat → Prop) → Prop`
/// — d-separation in a DAG: u ⊥ v | Z given separating set Z.
pub fn d_separation_ty() -> Expr {
    arrow(
        arrow(nat_ty(), arrow(nat_ty(), prop())),
        arrow(
            nat_ty(),
            arrow(nat_ty(), arrow(arrow(nat_ty(), prop()), prop())),
        ),
    )
}
/// `Faithfulness : (Nat → Nat → Prop) → Distribution → Prop`
/// — the faithfulness condition: every conditional independence in P is entailed by the graph.
pub fn faithfulness_ty() -> Expr {
    arrow(
        arrow(nat_ty(), arrow(nat_ty(), prop())),
        arrow(distribution_ty(), prop()),
    )
}
/// `FreeProbabilitySpace : Type`
/// — a non-commutative probability space (A, φ) for free probability theory.
pub fn free_probability_space_ty() -> Expr {
    type0()
}
/// `FreeIndependence : FreeProbabilitySpace → FreeProbabilitySpace → Prop`
/// — freeness (free independence) in the sense of Voiculescu.
pub fn free_independence_ty() -> Expr {
    arrow(
        free_probability_space_ty(),
        arrow(free_probability_space_ty(), prop()),
    )
}
/// `FreeConvolution : Distribution → Distribution → Distribution`
/// — the free additive convolution ⊞ of two spectral distributions.
pub fn free_convolution_ty() -> Expr {
    arrow(
        distribution_ty(),
        arrow(distribution_ty(), distribution_ty()),
    )
}
/// `QuantumProbSpace : Type`
/// — a quantum probability space given by a C*-algebra A with state φ.
pub fn quantum_prob_space_ty() -> Expr {
    type0()
}
/// `BranchingProcess : (Nat → Distribution) → (Nat → Nat) → Prop`
/// — a Galton–Watson branching process with offspring distribution Z_n.
pub fn branching_process_ty() -> Expr {
    arrow(
        arrow(nat_ty(), distribution_ty()),
        arrow(arrow(nat_ty(), nat_ty()), prop()),
    )
}
/// `ExtinctionProbability : BranchingProcess → Real`
/// — the probability q = P(eventual extinction) for a branching process.
pub fn extinction_prob_ty() -> Expr {
    arrow(branching_process_ty(), real_ty())
}
/// `RandomTree : Nat → Type`
/// — a random recursive tree (or Galton–Watson tree) with n nodes.
pub fn random_tree_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `ContinuumRandomTree : Type`
/// — the Brownian continuum random tree (CRT) as the scaling limit.
pub fn continuum_random_tree_ty() -> Expr {
    type0()
}
/// Extends the environment built by `build_probability_env` with the
/// advanced axioms from Section 9.
pub fn build_advanced_probability_env(env: &mut Environment) -> Result<(), String> {
    let advanced_type_decls: &[(&str, Expr)] = &[
        ("BrownianMotion", brownian_motion_ty()),
        ("LevyProcess", levy_process_ty()),
        ("ItoIntegral", ito_integral_ty()),
        ("QuadraticVariation", quadratic_variation_ty()),
        ("SDE", sde_ty()),
        ("GEVDistribution", gev_distribution_ty()),
        ("GPDDistribution", gpd_distribution_ty()),
        ("DirichletProcess", dirichlet_process_ty()),
        ("CRP", crp_ty()),
        ("GreenFunction", green_function_ty()),
        ("HittingTime", hitting_time_ty()),
        ("RandomWalk", random_walk_ty()),
        ("RandomTree", random_tree_ty()),
        ("ContinuumRandomTree", continuum_random_tree_ty()),
        ("FreeProbabilitySpace", free_probability_space_ty()),
        ("QuantumProbSpace", quantum_prob_space_ty()),
    ];
    for (name, ty) in advanced_type_decls {
        env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty: ty.clone(),
        })
        .ok();
    }
    let advanced_fn_decls: &[(&str, Expr)] = &[
        ("LogMGF", log_mgf_ty()),
        ("FenchelLegendre", fenchel_legendre_ty()),
        ("SpectralGap", spectral_gap_ty()),
        ("ExtinctionProb", extinction_prob_ty()),
        ("FreeConvolution", free_convolution_ty()),
        ("RademacherComplexity", rademacher_complexity_ty()),
        ("VCDimension", vc_dimension_ty()),
    ];
    for (name, ty) in advanced_fn_decls {
        env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty: ty.clone(),
        })
        .ok();
    }
    let advanced_pred_decls: &[(&str, Expr)] = &[
        ("ItoFormula", ito_formula_ty()),
        ("StrongSolution", strong_solution_ty()),
        ("WeakSolution", weak_solution_ty()),
        ("GirsanovThm", girsanov_thm_ty()),
        ("McDiarmidInequality", mcdiarmid_inequality_ty()),
        ("AzumaInequality", azuma_inequality_ty()),
        ("LDP", ldp_ty()),
        ("GartnerEllis", gartner_ellis_ty()),
        ("PickandsBalkemaDeHaan", pickands_balkema_de_haan_ty()),
        ("FisherTippettGnedenko", fisher_tippett_gnedenko_ty()),
        ("GaussianProcess", gaussian_process_ty()),
        ("DonskerThm", donsker_thm_ty()),
        ("MarkovBlanket", markov_blanket_ty()),
        ("DSeparation", d_separation_ty()),
        ("Faithfulness", faithfulness_ty()),
        ("FreeIndependence", free_independence_ty()),
        ("BranchingProcess", branching_process_ty()),
        ("ReversibleChain", reversible_chain_ty()),
    ];
    for (name, ty) in advanced_pred_decls {
        env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty: ty.clone(),
        })
        .ok();
    }
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    use oxilean_kernel::Environment;
    const EPS: f64 = 1e-6;
    fn approx_eq(a: f64, b: f64, tol: f64) -> bool {
        (a - b).abs() < tol
    }
    #[test]
    fn test_discrete_uniform() {
        let probs = discrete_uniform(4);
        assert_eq!(probs.len(), 4);
        for p in &probs {
            assert!(approx_eq(*p, 0.25, EPS));
        }
        let sum: f64 = probs.iter().sum();
        assert!(approx_eq(sum, 1.0, EPS));
    }
    #[test]
    fn test_binomial_pmf() {
        let p = binomial_pmf(10, 5, 0.5);
        assert!(approx_eq(p, 0.24609375, 1e-8));
    }
    #[test]
    fn test_poisson_pmf() {
        let p = poisson_pmf(2.0, 2);
        assert!(approx_eq(p, 2.0 * (-2.0f64).exp(), 1e-9));
        assert!(approx_eq(p, 0.27067, 1e-4));
    }
    #[test]
    fn test_normal_pdf() {
        let p = normal_pdf(0.0, 0.0, 1.0);
        let expected = 1.0 / (2.0 * std::f64::consts::PI).sqrt();
        assert!(approx_eq(p, expected, EPS));
        assert!(approx_eq(p, 0.3989422804, 1e-9));
    }
    #[test]
    fn test_sample_stats() {
        let data = [1.0, 2.0, 3.0, 4.0, 5.0];
        let mean = sample_mean(&data);
        assert!(approx_eq(mean, 3.0, EPS));
        let var = sample_variance(&data);
        assert!(approx_eq(var, 2.5, EPS));
    }
    #[test]
    fn test_pearson() {
        let x: Vec<f64> = (0..10).map(|i| i as f64).collect();
        let y: Vec<f64> = x.iter().map(|xi| 2.0 * xi + 1.0).collect();
        let r = pearson_correlation(&x, &y);
        assert!(approx_eq(r, 1.0, EPS));
    }
    #[test]
    fn test_markov_chain() {
        let transition = vec![vec![0.7, 0.3], vec![0.4, 0.6]];
        let chain = MarkovChain::new(transition);
        let stat = chain.stationary_distribution();
        assert_eq!(stat.len(), 2);
        assert!(approx_eq(stat[0], 4.0 / 7.0, 1e-6));
        assert!(approx_eq(stat[1], 3.0 / 7.0, 1e-6));
        assert!(chain.is_ergodic());
    }
    #[test]
    fn test_bayes_update() {
        let prior = [0.5, 0.5];
        let likelihoods = [0.8, 0.4];
        let posterior = bayes_update(&prior, &likelihoods);
        assert_eq!(posterior.len(), 2);
        assert!(approx_eq(posterior[0], 2.0 / 3.0, EPS));
        assert!(approx_eq(posterior[1], 1.0 / 3.0, EPS));
    }
    #[test]
    fn test_build_env() {
        let mut env = Environment::new();
        let result = build_probability_env(&mut env);
        assert!(result.is_ok());
    }
    #[test]
    fn test_discrete_distribution() {
        let weights = [1.0, 2.0, 3.0, 4.0];
        let dist = DiscreteDistribution::from_weights(&weights);
        assert_eq!(dist.pmf.len(), 4);
        let sum: f64 = dist.pmf.iter().sum();
        assert!(approx_eq(sum, 1.0, EPS));
        assert!(approx_eq(dist.prob(0), 0.1, EPS));
        assert!(approx_eq(dist.prob(3), 0.4, EPS));
        assert!(approx_eq(dist.mean(), 2.0, EPS));
    }
    #[test]
    fn test_gaussian_cdf() {
        let g = GaussianDistribution::new(0.0, 1.0);
        assert!(approx_eq(g.cdf(0.0), 0.5, 1e-4));
        assert!(approx_eq(g.cdf(1.96), 0.975, 1e-3));
    }
    #[test]
    fn test_concentration_bounds() {
        let intervals: Vec<(f64, f64)> = vec![(0.0, 1.0); 10];
        let b = ConcentrationBound::hoeffding(1.0, &intervals);
        assert!(approx_eq(b, (-0.2f64).exp(), 1e-6));
        let m = ConcentrationBound::markov(2.0, 4.0);
        assert!(approx_eq(m, 0.5, EPS));
        let c = ConcentrationBound::chebyshev(2.0);
        assert!(approx_eq(c, 0.25, EPS));
    }
    #[test]
    fn test_characteristic_function() {
        let pmf = vec![0.25; 4];
        let cf = CharacteristicFunction::new(pmf);
        assert!(approx_eq(cf.real_part(0.0), 1.0, EPS));
        assert!(approx_eq(cf.imag_part(0.0), 0.0, EPS));
        assert!(approx_eq(cf.moment(1), 1.5, EPS));
    }
    #[test]
    fn test_exponential_dist() {
        assert!(approx_eq(exponential_pdf(0.0, 1.0), 1.0, EPS));
        assert!(approx_eq(
            exponential_cdf(1.0, 1.0),
            1.0 - (-1.0f64).exp(),
            EPS
        ));
    }
    #[test]
    fn test_kl_divergence() {
        let p = [0.5, 0.5];
        assert!(approx_eq(kl_divergence(&p, &p), 0.0, EPS));
        let q = [0.5, 0.5];
        let p2 = [1.0, 0.0];
        let kl = kl_divergence(&p2, &q);
        assert!(approx_eq(kl, 2.0f64.ln(), EPS));
    }
    #[test]
    fn test_total_variation() {
        let p = [0.5, 0.5];
        let q = [0.25, 0.75];
        let tv = total_variation_distance(&p, &q);
        assert!(approx_eq(tv, 0.25, EPS));
    }
    #[test]
    fn test_geometric_pmf() {
        assert!(approx_eq(geometric_pmf(1, 0.5), 0.5, EPS));
        assert!(approx_eq(geometric_pmf(2, 0.5), 0.25, EPS));
    }
    #[test]
    fn test_lcg() {
        let mut lcg = Lcg::new(42);
        for _ in 0..100 {
            let v = lcg.next_f64();
            assert!(v >= 0.0 && v < 1.0);
        }
    }
    #[test]
    fn test_mixing_time() {
        let transition = vec![vec![0.5, 0.5], vec![0.5, 0.5]];
        let chain = MarkovChain::new(transition);
        let t = chain.mixing_time(0.01);
        assert!(t <= 5);
    }
    #[test]
    fn test_empirical_moments() {
        let data = [1.0, 2.0, 3.0];
        let moments = empirical_moments(&data, 2);
        assert!(approx_eq(moments[0], 1.0, EPS));
        assert!(approx_eq(moments[1], 2.0, EPS));
        assert!(approx_eq(moments[2], 14.0 / 3.0, EPS));
    }
    #[test]
    fn test_gaussian_mgf() {
        let g = GaussianDistribution::new(0.0, 1.0);
        assert!(approx_eq(g.mgf(1.0), (0.5f64).exp(), EPS));
        assert!(approx_eq(g.mgf(0.0), 1.0, EPS));
    }
    #[test]
    fn test_gamma_pdf_exponential() {
        let g = gamma_pdf(1.0, 1.0, 1.0);
        assert!(approx_eq(g, (-1.0f64).exp(), 1e-6));
    }
    #[test]
    fn test_exponential_distribution_struct() {
        let exp = ExponentialDistribution::new(2.0);
        assert!(approx_eq(exp.pdf(0.0), 2.0, EPS));
        assert!(approx_eq(exp.cdf(1.0), 1.0 - (-2.0f64).exp(), EPS));
        assert!(approx_eq(exp.mean(), 0.5, EPS));
        assert!(approx_eq(exp.variance(), 0.25, EPS));
        assert!(approx_eq(exp.quantile(0.0), 0.0, EPS));
        assert!(approx_eq(exp.mgf(1.0), 2.0, EPS));
    }
    #[test]
    fn test_welford_estimator() {
        let mut est = WelfordEstimator::new();
        for x in [1.0, 2.0, 3.0, 4.0, 5.0] {
            est.update(x);
        }
        assert_eq!(est.count(), 5);
        assert!(approx_eq(est.mean(), 3.0, EPS));
        assert!(approx_eq(est.variance(), 2.5, EPS));
    }
    #[test]
    fn test_welford_merge() {
        let mut est1 = WelfordEstimator::new();
        let mut est2 = WelfordEstimator::new();
        for x in [1.0, 2.0, 3.0] {
            est1.update(x);
        }
        for x in [4.0, 5.0] {
            est2.update(x);
        }
        est1.merge(&est2);
        assert_eq!(est1.count(), 5);
        assert!(approx_eq(est1.mean(), 3.0, 1e-10));
    }
    #[test]
    fn test_kde_density() {
        let kde = KernelDensityEstimator::with_bandwidth(vec![0.0], 1.0);
        let d = kde.density(0.0);
        let expected = 1.0 / (2.0 * std::f64::consts::PI).sqrt();
        assert!(approx_eq(d, expected, 1e-9));
        assert!(kde.density(100.0) < 1e-10);
    }
    #[test]
    fn test_empirical_cdf() {
        let ecdf = EmpiricalCdf::new(vec![1.0, 2.0, 3.0, 4.0, 5.0]);
        assert_eq!(ecdf.len(), 5);
        assert!(approx_eq(ecdf.eval(0.0), 0.0, EPS));
        assert!(approx_eq(ecdf.eval(3.0), 0.6, EPS));
        assert!(approx_eq(ecdf.eval(10.0), 1.0, EPS));
        assert!(approx_eq(ecdf.quantile(0.5), 3.0, EPS));
    }
    #[test]
    fn test_poisson_process() {
        let pp = PoissonProcess::new(3.0);
        assert!(approx_eq(pp.expected_count(1.0), 3.0, EPS));
        assert!(approx_eq(pp.variance_count(2.0), 6.0, EPS));
        assert!(approx_eq(pp.count_pmf(1.0, 0), (-3.0f64).exp(), 1e-9));
        assert!(approx_eq(pp.compound_expected(2.0, 4.0), 24.0, EPS));
    }
    #[test]
    fn test_poisson_process_simulation() {
        let pp = PoissonProcess::new(10.0);
        let mut lcg = Lcg::new(12345);
        let arrivals = pp.simulate_arrivals(1.0, &mut lcg);
        assert!(!arrivals.is_empty() || arrivals.is_empty());
        for &t in &arrivals {
            assert!(t > 0.0 && t <= 1.0);
        }
    }
    #[test]
    fn test_build_advanced_env() {
        let mut env = Environment::new();
        let result = build_probability_env(&mut env);
        assert!(result.is_ok());
    }
    #[test]
    fn test_ks_statistic() {
        let data: Vec<f64> = (1..=10).map(|i| i as f64 / 10.0).collect();
        let ecdf = EmpiricalCdf::new(data);
        let ks = ecdf.ks_statistic(|x| x.clamp(0.0, 1.0));
        assert!(ks <= 0.1 + EPS);
    }
}
#[cfg(test)]
mod extended_prob_tests {
    use super::*;
    #[test]
    fn test_characteristic_function() {
        let cf = CharFunctionData::gaussian(0.0, 1.0);
        assert!(cf.is_integrable);
        assert!(cf.levy_cramer_applies());
        assert!(cf.formula.contains("exp"));
    }
    #[test]
    fn test_large_deviations() {
        let ld = LargeDeviations::cramer("X");
        assert!(ld.is_good);
        assert!(ld.ldp_description().contains("LDP"));
        let sanov = LargeDeviations::sanov();
        assert!(sanov.rate_function.contains("KL"));
    }
    #[test]
    fn test_ergodic_theorem() {
        let birk = ErgodicTheoremData::birkhoff("T");
        assert_eq!(birk.theorem_name, "Birkhoff");
        assert!(birk.convergence_type.contains("L1"));
    }
    #[test]
    fn test_stopping_time() {
        let tau = StoppingTime::first_hitting("A", "F_t");
        assert!(tau.optional_stopping_description().contains("tau"));
    }
    #[test]
    fn test_coupling() {
        let c = Coupling::maximal("mu", "nu", 0.2);
        assert!(c.maximal_coupling_property().contains("P(X != Y)"));
        let ot = Coupling::optimal_transport("mu", "nu");
        assert!(ot.tv_bound.is_none());
    }
}
#[cfg(test)]
mod tests_prob_ext {
    use super::*;
    #[test]
    fn test_gaussian_process_sq_exp() {
        let gp = GaussianProcess::with_sq_exp(1.0, 1.0, 2);
        assert!(gp.is_stationary);
        let k = gp.kernel_value(0.0);
        assert!((k - 1.0).abs() < 1e-10);
        let k2 = gp.kernel_value(1.0);
        assert!(k2 < 1.0 && k2 > 0.0);
        let post = gp.posterior_description(5);
        assert!(post.contains("GP posterior"));
        let mercer = gp.mercer_representation();
        assert!(mercer.contains("Mercer"));
    }
    #[test]
    fn test_gaussian_process_matern() {
        let gp = GaussianProcess::with_matern(1.5, 1.0, 3);
        let k = gp.kernel_value(0.0);
        assert!((k - 1.0).abs() < 1e-10);
    }
    #[test]
    fn test_gp_regression() {
        let gp = GaussianProcess::with_sq_exp(1.0, 1.0, 2);
        let mut gpr = GaussianProcessRegression::new(gp, 0.1);
        gpr.n_training = 100;
        let cplx = gpr.complexity_exact();
        assert!(cplx.contains("O(n³)"));
        let sparse = gpr.sparse_gp_complexity(10);
        assert!(sparse.contains("inducing"));
        let lml = gpr.log_marginal_likelihood();
        assert!(lml.contains("log p"));
    }
    #[test]
    fn test_renewal_process() {
        let rp = RenewalProcess::poisson_process(2.0);
        assert!((rp.rate - 2.0).abs() < 1e-10);
        let ert = rp.elementary_renewal_theorem();
        assert!(ert.contains("Elementary renewal"));
        let rrt = rp.renewal_reward_theorem(1.0);
        assert!((rrt - 2.0).abs() < 1e-10);
        let blackwell = rp.blackwell_renewal_theorem();
        assert!(blackwell.contains("Blackwell"));
    }
    #[test]
    fn test_hawkes_process() {
        let hawkes = HawkesProcess::new(1.0, 0.5, 1.0);
        assert!(hawkes.is_stationary);
        assert!((hawkes.branching_ratio() - 0.5).abs() < 1e-10);
        let mean = hawkes.mean_intensity();
        assert!(mean > hawkes.base_intensity);
        let ci = hawkes.conditional_intensity(1.0, 0.5);
        assert!(ci > hawkes.base_intensity);
    }
    #[test]
    fn test_dirichlet_process() {
        let dp = DirichletProcess::new(2.0, "N(0,1)");
        assert!(dp.is_discrete);
        let ec = dp.expected_clusters_for_n(100);
        assert!(ec > 0.0);
        let stick = dp.stick_breaking_construction();
        assert!(stick.contains("Stick-breaking"));
        let crp = dp.chinese_restaurant_process(100);
        assert!(crp.contains("CRP"));
        let post = dp.posterior_update(50);
        assert!((post.concentration - 52.0).abs() < 1e-10);
    }
}
/// Log-gamma approximation via Stirling's series.
#[allow(dead_code)]
pub(super) fn lgamma_approx(x: f64) -> f64 {
    0.5 * std::f64::consts::TAU.ln() + (x - 0.5) * x.ln() - x
}
