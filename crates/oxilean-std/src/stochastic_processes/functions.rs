//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{
    BlackScholesPricer, CompoundPoissonProcess, CtMarkovChain, EulerMaruyama,
    GeometricBrownianMotionProcess, Lcg, OrnsteinUhlenbeckProcess, PoissonProcessSimulator,
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
pub fn list_ty(elem: Expr) -> Expr {
    app(cst("List"), elem)
}
/// `StochasticProcess : (FilteredProbabilitySpace → Real)`
/// A stochastic process X_t indexed by time over a filtered probability space.
pub fn stochastic_process_ty() -> Expr {
    arrow(cst("FilteredProbabilitySpace"), real_ty())
}
/// `Martingale : StochasticProcess → Prop`
/// A martingale: E[X_t | F_s] = X_s for s ≤ t.
pub fn martingale_ty() -> Expr {
    arrow(stochastic_process_ty(), prop())
}
/// `BrownianMotion : StochasticProcess`
/// Standard Brownian motion (Wiener process): B_0 = 0, independent increments,
/// B_t - B_s ~ N(0, t-s).
pub fn brownian_motion_ty() -> Expr {
    stochastic_process_ty()
}
/// `ItoIntegral : (StochasticProcess → Real → Real) → StochasticProcess`
/// Itô stochastic integral ∫₀ᵗ H_s dW_s for adapted process H.
pub fn ito_integral_ty() -> Expr {
    arrow(
        arrow(stochastic_process_ty(), arrow(real_ty(), real_ty())),
        stochastic_process_ty(),
    )
}
/// `SDE : (Real → Real) → (Real → Real) → StochasticProcess → Prop`
/// Stochastic differential equation: dX_t = μ(X_t) dt + σ(X_t) dW_t.
pub fn sde_ty() -> Expr {
    arrow(
        arrow(real_ty(), real_ty()),
        arrow(
            arrow(real_ty(), real_ty()),
            arrow(stochastic_process_ty(), prop()),
        ),
    )
}
/// Optional stopping theorem: if τ is a bounded stopping time and X is a martingale,
/// then E\[X_τ\] = E\[X_0\].
pub fn optional_stopping_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        stochastic_process_ty(),
        arrow(nat_ty(), arrow(app(cst("Martingale"), bvar(1)), prop())),
    )
}
/// Doob's maximal inequality: P(max_{0≤k≤n} X_k ≥ λ) ≤ E[|X_n|] / λ for λ > 0.
pub fn doob_maximal_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        stochastic_process_ty(),
        pi(
            BinderInfo::Default,
            "n",
            nat_ty(),
            arrow(real_ty(), arrow(app(cst("Martingale"), bvar(2)), prop())),
        ),
    )
}
/// Itô's lemma (chain rule for SDEs): if f is C², then
/// df(X_t) = f'(X_t)dX_t + ½ f''(X_t)(dX_t)².
pub fn ito_lemma_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        stochastic_process_ty(),
        arrow(arrow(real_ty(), real_ty()), prop()),
    )
}
/// Girsanov's theorem: change of measure via the Cameron-Martin-Girsanov theorem.
/// Under a new measure Q, W̃_t = W_t - ∫₀ᵗ θ_s ds is a Q-Brownian motion.
pub fn girsanov_theorem_ty() -> Expr {
    arrow(
        arrow(real_ty(), real_ty()),
        arrow(brownian_motion_ty(), prop()),
    )
}
/// Reflection principle for standard Brownian motion:
/// P(max_{0≤s≤t} B_s ≥ a) = 2 * P(B_t ≥ a) for a > 0.
pub fn reflection_principle_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), arrow(prop(), prop())))
}
/// `brownian_motion_existence : BrownianMotionProcess`
/// Existence of standard Brownian motion on a complete probability space.
pub fn brownian_motion_existence_ty() -> Expr {
    cst("BrownianMotionProcess")
}
/// `brownian_motion_is_martingale : BrownianMotionProcess → Prop`
/// Standard Brownian motion is a martingale with respect to its natural filtration.
pub fn brownian_motion_is_martingale_ty() -> Expr {
    arrow(cst("BrownianMotionProcess"), prop())
}
/// `brownian_motion_continuous_paths : BrownianMotionProcess → Prop`
/// Brownian motion has continuous sample paths (a.s.).
pub fn brownian_motion_continuous_paths_ty() -> Expr {
    arrow(cst("BrownianMotionProcess"), prop())
}
/// `brownian_motion_quadratic_variation : BrownianMotionProcess → Real → Prop`
/// The quadratic variation of Brownian motion satisfies \[B\]_t = t.
pub fn brownian_motion_quadratic_variation_ty() -> Expr {
    arrow(cst("BrownianMotionProcess"), arrow(real_ty(), prop()))
}
/// `brownian_motion_zero_at_origin : BrownianMotionProcess → Prop`
/// Brownian motion starts at zero: B_0 = 0 a.s.
pub fn brownian_motion_zero_at_origin_ty() -> Expr {
    arrow(cst("BrownianMotionProcess"), prop())
}
/// `ito_isometry : AdaptedProcess → Real → Prop`
/// The Itô isometry: E\[(∫₀ᵀ H_s dW_s)²\] = E\[∫₀ᵀ H_s² ds\].
pub fn ito_isometry_ty() -> Expr {
    arrow(cst("AdaptedProcess"), arrow(real_ty(), prop()))
}
/// `ito_formula_multidim : Nat → StochasticProcess → (Real → Real) → Prop`
/// Multidimensional Itô formula for d-dimensional processes.
pub fn ito_formula_multidim_ty() -> Expr {
    arrow(
        nat_ty(),
        arrow(
            stochastic_process_ty(),
            arrow(arrow(real_ty(), real_ty()), prop()),
        ),
    )
}
/// `stratonovich_integral : AdaptedProcess → StochasticProcess`
/// The Stratonovich integral ∫₀ᵗ H_s ∘ dW_s, related to Itô by a correction term.
pub fn stratonovich_integral_ty() -> Expr {
    arrow(cst("AdaptedProcess"), stochastic_process_ty())
}
/// `ito_stratonovich_relation : AdaptedProcess → Prop`
/// ∫ H ∘ dW = ∫ H dW + ½ ∫ \[H, W\] dt (correction term formula).
pub fn ito_stratonovich_relation_ty() -> Expr {
    arrow(cst("AdaptedProcess"), prop())
}
/// `sde_strong_solution_existence : (Real → Real) → (Real → Real) → Real → Prop`
/// Under Lipschitz and linear growth conditions on μ and σ, a strong solution exists.
pub fn sde_strong_solution_existence_ty() -> Expr {
    arrow(
        arrow(real_ty(), real_ty()),
        arrow(arrow(real_ty(), real_ty()), arrow(real_ty(), prop())),
    )
}
/// `sde_strong_solution_uniqueness : (Real → Real) → (Real → Real) → Prop`
/// Pathwise uniqueness of strong solutions under Lipschitz conditions.
pub fn sde_strong_solution_uniqueness_ty() -> Expr {
    arrow(
        arrow(real_ty(), real_ty()),
        arrow(arrow(real_ty(), real_ty()), prop()),
    )
}
/// `sde_weak_solution_existence : (Real → Real) → (Real → Real) → Prop`
/// Existence of weak solutions under continuity conditions (Stroock-Varadhan).
pub fn sde_weak_solution_existence_ty() -> Expr {
    arrow(
        arrow(real_ty(), real_ty()),
        arrow(arrow(real_ty(), real_ty()), prop()),
    )
}
/// `martingale_representation_theorem : Martingale → AdaptedProcess → Prop`
/// Every square-integrable martingale can be represented as a stochastic integral.
pub fn martingale_representation_ty() -> Expr {
    arrow(
        arrow(stochastic_process_ty(), prop()),
        arrow(cst("AdaptedProcess"), prop()),
    )
}
/// `doob_meyer_decomposition : StochasticProcess → Prop`
/// Every submartingale X admits a unique decomposition X = M + A
/// where M is a martingale and A is a predictable increasing process.
pub fn doob_meyer_decomposition_ty() -> Expr {
    arrow(stochastic_process_ty(), prop())
}
/// `local_martingale_to_martingale : StochasticProcess → Prop`
/// A non-negative local martingale is a supermartingale (Fatou's lemma).
pub fn local_martingale_to_martingale_ty() -> Expr {
    arrow(stochastic_process_ty(), prop())
}
/// `feynman_kac_formula : (Real → Real → Real) → StochasticProcess → Prop`
/// The Feynman-Kac formula: solution to PDE ∂u/∂t + Lu = -f is given by
/// u(t,x) = E[∫ₜᵀ f(X_s) ds + g(X_T) | X_t = x].
pub fn feynman_kac_formula_ty() -> Expr {
    arrow(
        arrow(real_ty(), arrow(real_ty(), real_ty())),
        arrow(stochastic_process_ty(), prop()),
    )
}
/// `black_scholes_pde : (Real → Real → Real) → Prop`
/// The Black-Scholes PDE: ∂V/∂t + ½σ²S²∂²V/∂S² + rS∂V/∂S - rV = 0.
pub fn black_scholes_pde_ty() -> Expr {
    arrow(arrow(real_ty(), arrow(real_ty(), real_ty())), prop())
}
/// `risk_neutral_pricing : StochasticProcess → Real → Prop`
/// Under risk-neutral measure Q, the discounted asset price is a Q-martingale.
pub fn risk_neutral_pricing_ty() -> Expr {
    arrow(stochastic_process_ty(), arrow(real_ty(), prop()))
}
/// `LevyProcess : StochasticProcess`
/// A Lévy process has stationary and independent increments, is càdlàg, and X_0 = 0.
pub fn levy_process_ty() -> Expr {
    stochastic_process_ty()
}
/// `levy_khintchine_formula : LevyProcess → (Real → Real) → Prop`
/// The Lévy-Khintchine formula: the characteristic exponent ψ(u) satisfies
/// E[e^{iuX_t}] = exp(t ψ(u)), where ψ(u) = iau - σ²u²/2 + ∫(e^{iux}-1-iux1_{|x|<1})ν(dx).
pub fn levy_khintchine_formula_ty() -> Expr {
    arrow(
        stochastic_process_ty(),
        arrow(arrow(real_ty(), real_ty()), prop()),
    )
}
/// `levy_ito_decomposition : LevyProcess → Prop`
/// Every Lévy process decomposes as X_t = bt + σW_t + J_t where J is a pure-jump process.
pub fn levy_ito_decomposition_ty() -> Expr {
    arrow(stochastic_process_ty(), prop())
}
/// `levy_process_is_semimartingale : LevyProcess → Prop`
/// Every Lévy process is a semimartingale.
pub fn levy_process_is_semimartingale_ty() -> Expr {
    arrow(stochastic_process_ty(), prop())
}
/// `PoissonProcess : Real → StochasticProcess`
/// A Poisson process N_t with rate λ: N_0 = 0, stationary independent increments,
/// N_t - N_s ~ Poisson(λ(t-s)).
pub fn poisson_process_ty() -> Expr {
    arrow(real_ty(), stochastic_process_ty())
}
/// `poisson_process_mean : Real → Real → Prop`
/// E\[N_t\] = λt for a Poisson process with rate λ.
pub fn poisson_process_mean_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), prop()))
}
/// `poisson_superposition : Real → Real → Prop`
/// Superposition: the sum of independent Poisson(λ₁) and Poisson(λ₂) is Poisson(λ₁+λ₂).
pub fn poisson_superposition_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), prop()))
}
/// `compound_poisson_process : Real → (Real → Real) → StochasticProcess`
/// Compound Poisson process: Y_t = ∑_{i=1}^{N_t} Z_i where Z_i are i.i.d. jump sizes.
pub fn compound_poisson_process_ty() -> Expr {
    arrow(
        real_ty(),
        arrow(arrow(real_ty(), real_ty()), stochastic_process_ty()),
    )
}
/// `compound_poisson_mean : Real → (Real → Real) → Real → Prop`
/// E\[Y_t\] = λt·E\[Z\] for a compound Poisson process with rate λ and jump distribution Z.
pub fn compound_poisson_mean_ty() -> Expr {
    arrow(
        real_ty(),
        arrow(arrow(real_ty(), real_ty()), arrow(real_ty(), prop())),
    )
}
/// `jump_sde : (Real → Real) → (Real → Real) → (Real → Real → Real) → StochasticProcess → Prop`
/// Jump-diffusion SDE: dX_t = μ(X_t) dt + σ(X_t) dW_t + ∫ c(X_{t-}, z) N(dt, dz).
pub fn jump_sde_ty() -> Expr {
    arrow(
        arrow(real_ty(), real_ty()),
        arrow(
            arrow(real_ty(), real_ty()),
            arrow(
                arrow(real_ty(), arrow(real_ty(), real_ty())),
                arrow(stochastic_process_ty(), prop()),
            ),
        ),
    )
}
/// `jump_sde_existence : (Real → Real) → (Real → Real) → Prop`
/// Strong existence for jump SDEs under Lipschitz conditions on coefficients.
pub fn jump_sde_existence_ty() -> Expr {
    arrow(
        arrow(real_ty(), real_ty()),
        arrow(arrow(real_ty(), real_ty()), prop()),
    )
}
/// `markov_property : StochasticProcess → Prop`
/// The Markov property: P(X_t ∈ A | F_s) = P(X_t ∈ A | X_s) for s ≤ t.
pub fn markov_property_ty() -> Expr {
    arrow(stochastic_process_ty(), prop())
}
/// `strong_markov_property : StochasticProcess → Prop`
/// The strong Markov property: the Markov property holds at stopping times.
pub fn strong_markov_property_ty() -> Expr {
    arrow(stochastic_process_ty(), prop())
}
/// `markov_semigroup : StochasticProcess → (Real → Real → Real) → Prop`
/// The transition semigroup P_t f(x) = E[f(X_t) | X_0 = x] satisfies P_{t+s} = P_t ∘ P_s.
pub fn markov_semigroup_ty() -> Expr {
    arrow(
        stochastic_process_ty(),
        arrow(arrow(real_ty(), arrow(real_ty(), real_ty())), prop()),
    )
}
/// `brownian_motion_harmonic : (Real → Real) → Prop`
/// A function f is harmonic for Brownian motion iff Δf = 0 (Laplace equation).
pub fn brownian_motion_harmonic_ty() -> Expr {
    arrow(arrow(real_ty(), real_ty()), prop())
}
/// `potential_theory_green_function : Real → Real → Real`
/// The Green's function G(x, y) = |x - y|^{2-d} (up to constants) for d-dimensional BM.
pub fn potential_theory_green_function_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), real_ty()))
}
/// `brownian_motion_recurrence : Nat → Prop`
/// Brownian motion is recurrent in dimensions d ≤ 2 and transient for d ≥ 3.
pub fn brownian_motion_recurrence_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `StoppingTime : StochasticProcess → Nat`
/// A stopping time τ is measurable with respect to the natural filtration.
pub fn stopping_time_ty() -> Expr {
    arrow(stochastic_process_ty(), nat_ty())
}
/// `optional_sampling_theorem : StochasticProcess → Nat → Nat → Prop`
/// If X is a uniformly integrable martingale and σ ≤ τ are stopping times,
/// then E[X_τ | F_σ] = X_σ.
pub fn optional_sampling_theorem_ty() -> Expr {
    arrow(
        stochastic_process_ty(),
        arrow(nat_ty(), arrow(nat_ty(), prop())),
    )
}
/// `wald_identity : StochasticProcess → Nat → Prop`
/// Wald's identity: E\[X_τ\] = E\[X_1\] · E\[τ\] for random walks with finite mean stopping time.
pub fn wald_identity_ty() -> Expr {
    arrow(stochastic_process_ty(), arrow(nat_ty(), prop()))
}
/// `Semimartingale : StochasticProcess → Prop`
/// A semimartingale is a process X = M + A where M is a local martingale and A has
/// finite variation (FV) paths.
pub fn semimartingale_ty() -> Expr {
    arrow(stochastic_process_ty(), prop())
}
/// `semimartingale_integration : StochasticProcess → StochasticProcess → StochasticProcess`
/// Stochastic integral ∫ H dX for a predictable H and semimartingale X.
pub fn semimartingale_integration_ty() -> Expr {
    arrow(
        stochastic_process_ty(),
        arrow(stochastic_process_ty(), stochastic_process_ty()),
    )
}
/// `quadratic_covariation : StochasticProcess → StochasticProcess → StochasticProcess`
/// The quadratic covariation \[X, Y\]_t of two semimartingales.
pub fn quadratic_covariation_ty() -> Expr {
    arrow(
        stochastic_process_ty(),
        arrow(stochastic_process_ty(), stochastic_process_ty()),
    )
}
/// `invariant_measure : (Real → Real) → (Real → Real) → (Real → Real) → Prop`
/// An invariant (stationary) measure for the SDE dX = μ(X) dt + σ(X) dW.
/// Under ergodicity, the time average converges to the space average.
pub fn invariant_measure_ty() -> Expr {
    arrow(
        arrow(real_ty(), real_ty()),
        arrow(
            arrow(real_ty(), real_ty()),
            arrow(arrow(real_ty(), real_ty()), prop()),
        ),
    )
}
/// `ergodic_theorem_sde : StochasticProcess → (Real → Real) → Prop`
/// Ergodic theorem for SDEs: time averages converge to stationary expectations.
pub fn ergodic_theorem_sde_ty() -> Expr {
    arrow(
        stochastic_process_ty(),
        arrow(arrow(real_ty(), real_ty()), prop()),
    )
}
/// `fokker_planck_equation : (Real → Real) → (Real → Real) → Prop`
/// The Fokker-Planck equation for the probability density of X_t satisfying an SDE.
pub fn fokker_planck_equation_ty() -> Expr {
    arrow(
        arrow(real_ty(), real_ty()),
        arrow(arrow(real_ty(), real_ty()), prop()),
    )
}
pub fn build_stochastic_processes_env(env: &mut Environment) {
    let axioms: &[(&str, Expr)] = &[
        ("FilteredProbabilitySpace", type0()),
        ("BrownianMotionProcess", type0()),
        ("AdaptedProcess", type0()),
        ("LevyMeasure", type0()),
        ("StochasticProcess", stochastic_process_ty()),
        ("Martingale", martingale_ty()),
        ("BrownianMotion", brownian_motion_ty()),
        ("ItoIntegral", ito_integral_ty()),
        ("SDE", sde_ty()),
        ("GBM", stochastic_process_ty()),
        ("OUProcess", stochastic_process_ty()),
        ("StoppingTime", arrow(stochastic_process_ty(), nat_ty())),
        ("optional_stopping", optional_stopping_ty()),
        ("doob_maximal", doob_maximal_ty()),
        ("ito_lemma", ito_lemma_ty()),
        ("girsanov_theorem", girsanov_theorem_ty()),
        ("reflection_principle", reflection_principle_ty()),
        ("brownian_motion_existence", brownian_motion_existence_ty()),
        (
            "brownian_motion_is_martingale",
            brownian_motion_is_martingale_ty(),
        ),
        (
            "brownian_motion_continuous_paths",
            brownian_motion_continuous_paths_ty(),
        ),
        (
            "brownian_motion_quadratic_variation",
            brownian_motion_quadratic_variation_ty(),
        ),
        (
            "brownian_motion_zero_at_origin",
            brownian_motion_zero_at_origin_ty(),
        ),
        ("ito_isometry", ito_isometry_ty()),
        ("ito_formula_multidim", ito_formula_multidim_ty()),
        ("stratonovich_integral", stratonovich_integral_ty()),
        ("ito_stratonovich_relation", ito_stratonovich_relation_ty()),
        (
            "sde_strong_solution_existence",
            sde_strong_solution_existence_ty(),
        ),
        (
            "sde_strong_solution_uniqueness",
            sde_strong_solution_uniqueness_ty(),
        ),
        (
            "sde_weak_solution_existence",
            sde_weak_solution_existence_ty(),
        ),
        (
            "martingale_representation_theorem",
            martingale_representation_ty(),
        ),
        ("doob_meyer_decomposition", doob_meyer_decomposition_ty()),
        (
            "local_martingale_to_martingale",
            local_martingale_to_martingale_ty(),
        ),
        ("feynman_kac_formula", feynman_kac_formula_ty()),
        ("black_scholes_pde", black_scholes_pde_ty()),
        ("risk_neutral_pricing", risk_neutral_pricing_ty()),
        ("LevyProcess", levy_process_ty()),
        ("levy_khintchine_formula", levy_khintchine_formula_ty()),
        ("levy_ito_decomposition", levy_ito_decomposition_ty()),
        (
            "levy_process_is_semimartingale",
            levy_process_is_semimartingale_ty(),
        ),
        ("PoissonProcess", poisson_process_ty()),
        ("poisson_process_mean", poisson_process_mean_ty()),
        ("poisson_superposition", poisson_superposition_ty()),
        ("compound_poisson_process", compound_poisson_process_ty()),
        ("compound_poisson_mean", compound_poisson_mean_ty()),
        ("jump_sde", jump_sde_ty()),
        ("jump_sde_existence", jump_sde_existence_ty()),
        ("markov_property", markov_property_ty()),
        ("strong_markov_property", strong_markov_property_ty()),
        ("markov_semigroup", markov_semigroup_ty()),
        ("brownian_motion_harmonic", brownian_motion_harmonic_ty()),
        (
            "potential_theory_green_function",
            potential_theory_green_function_ty(),
        ),
        (
            "brownian_motion_recurrence",
            brownian_motion_recurrence_ty(),
        ),
        ("stopping_time_measurable", stopping_time_ty()),
        ("optional_sampling_theorem", optional_sampling_theorem_ty()),
        ("wald_identity", wald_identity_ty()),
        ("Semimartingale", semimartingale_ty()),
        (
            "semimartingale_integration",
            semimartingale_integration_ty(),
        ),
        ("quadratic_covariation", quadratic_covariation_ty()),
        ("invariant_measure", invariant_measure_ty()),
        ("ergodic_theorem_sde", ergodic_theorem_sde_ty()),
        ("fokker_planck_equation", fokker_planck_equation_ty()),
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
/// Simulate a discrete-time symmetric random walk starting at X_0 = 0.
///
/// At each step X_{t+1} = X_t ± 1 with probability 1/2 each.
/// The same `seed` always produces the same path (LCG-based reproducibility).
pub fn random_walk(n_steps: u32, seed: u64) -> Vec<f64> {
    let mut lcg = Lcg::new(seed);
    let mut path = Vec::with_capacity(n_steps as usize + 1);
    let mut x = 0.0f64;
    path.push(x);
    for _ in 0..n_steps {
        x += lcg.next_step();
        path.push(x);
    }
    path
}
/// Brownian motion approximation via a scaled symmetric random walk on \[0, t_end\].
///
/// Returns `(time, value)` pairs with `n_steps + 1` points.  The step size is
/// `dt = t_end / n_steps`, and each increment is scaled by `√dt` so that the
/// variance of the position at time t equals t.
pub fn brownian_motion(t_end: f64, n_steps: u32, seed: u64) -> Vec<(f64, f64)> {
    let mut lcg = Lcg::new(seed);
    let dt = t_end / n_steps as f64;
    let scale = dt.sqrt();
    let mut path = Vec::with_capacity(n_steps as usize + 1);
    let mut t = 0.0f64;
    let mut x = 0.0f64;
    path.push((t, x));
    for _ in 0..n_steps {
        x += lcg.next_normal() * scale;
        t += dt;
        path.push((t, x));
    }
    path
}
/// Simulate Geometric Brownian Motion: dS = μ S dt + σ S dW.
///
/// The exact solution is S_t = S_0 exp((μ - σ²/2) t + σ W_t).
/// We discretize using the Euler-Maruyama scheme on log(S).
///
/// Returns `(time, S_t)` pairs.
pub fn geometric_brownian_motion(
    s0: f64,
    mu: f64,
    sigma: f64,
    t_end: f64,
    n_steps: u32,
    seed: u64,
) -> Vec<(f64, f64)> {
    let mut lcg = Lcg::new(seed);
    let dt = t_end / n_steps as f64;
    let drift = (mu - 0.5 * sigma * sigma) * dt;
    let vol = sigma * dt.sqrt();
    let mut path = Vec::with_capacity(n_steps as usize + 1);
    let mut t = 0.0f64;
    let mut s = s0;
    path.push((t, s));
    for _ in 0..n_steps {
        s *= (drift + vol * lcg.next_normal()).exp();
        t += dt;
        path.push((t, s));
    }
    path
}
/// Simulate the Ornstein-Uhlenbeck process: dX = θ(μ - X) dt + σ dW.
///
/// Uses the exact conditional distribution (Euler-Maruyama approximation here).
///
/// Returns `(time, X_t)` pairs.
pub fn ornstein_uhlenbeck(
    x0: f64,
    theta: f64,
    mu: f64,
    sigma: f64,
    t_end: f64,
    n_steps: u32,
    seed: u64,
) -> Vec<(f64, f64)> {
    let mut lcg = Lcg::new(seed);
    let dt = t_end / n_steps as f64;
    let vol = sigma * dt.sqrt();
    let mut path = Vec::with_capacity(n_steps as usize + 1);
    let mut t = 0.0f64;
    let mut x = x0;
    path.push((t, x));
    for _ in 0..n_steps {
        x += theta * (mu - x) * dt + vol * lcg.next_normal();
        t += dt;
        path.push((t, x));
    }
    path
}
/// Approximation of the standard normal CDF Φ(x) using Abramowitz & Stegun 26.2.17.
///
/// Maximum error < 7.5e-8.
pub fn standard_normal_cdf(x: f64) -> f64 {
    if x > 8.0 {
        return 1.0;
    }
    if x < -8.0 {
        return 0.0;
    }
    let t = 1.0 / (1.0 + 0.2316419 * x.abs());
    let poly = t
        * (0.319381530
            + t * (-0.356563782 + t * (1.781477937 + t * (-1.821255978 + t * 1.330274429))));
    let phi = (-0.5 * x * x).exp() / (2.0 * std::f64::consts::PI).sqrt();
    let cdf = 1.0 - phi * poly;
    if x >= 0.0 {
        cdf
    } else {
        1.0 - cdf
    }
}
/// Black-Scholes call option price.
///
/// C = S·N(d₁) − K·e^{−rT}·N(d₂)
/// d₁ = \[ln(S/K) + (r + σ²/2)T\] / (σ√T)
/// d₂ = d₁ − σ√T
pub fn black_scholes_call(s: f64, k: f64, t: f64, r: f64, sigma: f64) -> f64 {
    if t <= 0.0 {
        return (s - k).max(0.0);
    }
    let sqrt_t = t.sqrt();
    let d1 = ((s / k).ln() + (r + 0.5 * sigma * sigma) * t) / (sigma * sqrt_t);
    let d2 = d1 - sigma * sqrt_t;
    s * standard_normal_cdf(d1) - k * (-r * t).exp() * standard_normal_cdf(d2)
}
/// Black-Scholes put option price via put-call parity.
///
/// P = C − S + K·e^{−rT}
pub fn black_scholes_put(s: f64, k: f64, t: f64, r: f64, sigma: f64) -> f64 {
    black_scholes_call(s, k, t, r, sigma) - s + k * (-r * t).exp()
}
/// Monte Carlo estimate of a European call option price under GBM.
///
/// Simulates `n_paths` terminal stock prices and averages the discounted payoff.
pub fn monte_carlo_call(
    s0: f64,
    k: f64,
    t: f64,
    r: f64,
    sigma: f64,
    n_paths: u32,
    seed: u64,
) -> f64 {
    if n_paths == 0 {
        return 0.0;
    }
    let mut lcg = Lcg::new(seed);
    let drift = (r - 0.5 * sigma * sigma) * t;
    let vol = sigma * t.sqrt();
    let discount = (-r * t).exp();
    let mut total = 0.0f64;
    for _ in 0..n_paths {
        let z = lcg.next_normal();
        let s_t = s0 * (drift + vol * z).exp();
        total += (s_t - k).max(0.0);
    }
    discount * total / n_paths as f64
}
/// `martingale_l1_convergence : StochasticProcess → Prop`
/// L¹ martingale convergence: a uniformly integrable martingale converges in L¹
/// and a.s. to an integrable limit X_∞.
pub fn martingale_l1_convergence_ty() -> Expr {
    arrow(stochastic_process_ty(), prop())
}
/// `martingale_l2_convergence : StochasticProcess → Prop`
/// L² martingale convergence: a bounded-in-L² martingale converges in L² and a.s.
pub fn martingale_l2_convergence_ty() -> Expr {
    arrow(stochastic_process_ty(), prop())
}
/// `martingale_as_convergence : StochasticProcess → Prop`
/// Almost sure martingale convergence theorem (Doob): any L¹-bounded martingale
/// converges almost surely.
pub fn martingale_as_convergence_ty() -> Expr {
    arrow(stochastic_process_ty(), prop())
}
/// `reverse_martingale_convergence : StochasticProcess → Prop`
/// Reverse martingale convergence: X_n → E[X_0 | ∩ F_n] a.s. and in L¹.
pub fn reverse_martingale_convergence_ty() -> Expr {
    arrow(stochastic_process_ty(), prop())
}
/// `uniformly_integrable_martingale : StochasticProcess → Prop`
/// A martingale is uniformly integrable iff it is L¹-bounded and closed.
pub fn uniformly_integrable_martingale_ty() -> Expr {
    arrow(stochastic_process_ty(), prop())
}
/// `optional_stopping_ui : StochasticProcess → Nat → Prop`
/// Optional stopping for uniformly integrable martingales: E\[X_τ\] = E\[X_0\]
/// for any stopping time τ (not necessarily bounded).
pub fn optional_stopping_ui_ty() -> Expr {
    arrow(stochastic_process_ty(), arrow(nat_ty(), prop()))
}
/// `doob_maximal_l2 : StochasticProcess → Nat → Prop`
/// Doob's L² maximal inequality: E\[max_{0≤k≤n} X_k²\] ≤ 4 E\[X_n²\].
pub fn doob_maximal_l2_ty() -> Expr {
    arrow(stochastic_process_ty(), arrow(nat_ty(), prop()))
}
/// `doob_upcrossing_inequality : StochasticProcess → Real → Real → Prop`
/// Doob's upcrossing inequality: E[U_n\[a,b\]] ≤ E\[(X_n - a)⁺\] / (b - a).
pub fn doob_upcrossing_inequality_ty() -> Expr {
    arrow(
        stochastic_process_ty(),
        arrow(real_ty(), arrow(real_ty(), prop())),
    )
}
/// `PredictableProcess : Type`
/// A predictable process is measurable with respect to the predictable σ-algebra.
pub fn predictable_process_ty() -> Expr {
    type0()
}
/// `PredictableSigmaAlgebra : Type`
/// The predictable σ-algebra P on Ω × [0, ∞): generated by left-continuous adapted processes.
pub fn predictable_sigma_algebra_ty() -> Expr {
    type0()
}
/// `OptionalSigmaAlgebra : Type`
/// The optional σ-algebra: generated by right-continuous adapted processes.
pub fn optional_sigma_algebra_ty() -> Expr {
    type0()
}
/// `predictable_compensator : StochasticProcess → PredictableProcess → Prop`
/// The predictable compensator (dual predictable projection): for increasing A,
/// there exists unique predictable Â with E\[∫ H dA\] = E\[∫ H dÂ\] for bounded predictable H.
pub fn predictable_compensator_ty() -> Expr {
    arrow(
        stochastic_process_ty(),
        arrow(cst("PredictableProcess"), prop()),
    )
}
/// `natural_filtration : StochasticProcess → Type`
/// The natural filtration F_t = σ(X_s : s ≤ t) generated by the process.
pub fn natural_filtration_ty() -> Expr {
    arrow(stochastic_process_ty(), type0())
}
/// `brownian_filtration_complete : BrownianMotionProcess → Prop`
/// The natural filtration of Brownian motion, augmented by null sets, is right-continuous.
pub fn brownian_filtration_complete_ty() -> Expr {
    arrow(cst("BrownianMotionProcess"), prop())
}
/// `brownian_increment_independence : BrownianMotionProcess → Real → Real → Prop`
/// Brownian increments B_t - B_s are independent of F_s for s ≤ t.
pub fn brownian_increment_independence_ty() -> Expr {
    arrow(
        cst("BrownianMotionProcess"),
        arrow(real_ty(), arrow(real_ty(), prop())),
    )
}
/// `brownian_scaling : BrownianMotionProcess → Real → Prop`
/// Scaling invariance: c^{-1/2} B_{ct} is also a standard Brownian motion.
pub fn brownian_scaling_ty() -> Expr {
    arrow(cst("BrownianMotionProcess"), arrow(real_ty(), prop()))
}
/// `brownian_time_inversion : BrownianMotionProcess → Prop`
/// Time inversion: t B_{1/t} (with convention 0 at t=0) is a Brownian motion.
pub fn brownian_time_inversion_ty() -> Expr {
    arrow(cst("BrownianMotionProcess"), prop())
}
/// `brownian_law_of_iterated_logarithm : BrownianMotionProcess → Prop`
/// The law of the iterated logarithm: lim sup_{t→∞} B_t / √(2t log log t) = 1 a.s.
pub fn brownian_lil_ty() -> Expr {
    arrow(cst("BrownianMotionProcess"), prop())
}
/// `ito_isometry_general : AdaptedProcess → Real → Prop`
/// The full Itô isometry: E[|∫₀ᵀ H_s dW_s|²] = E[∫₀ᵀ |H_s|² ds].
pub fn ito_isometry_general_ty() -> Expr {
    arrow(cst("AdaptedProcess"), arrow(real_ty(), prop()))
}
/// `StochasticExponential : StochasticProcess → StochasticProcess`
/// The stochastic exponential (Doléans-Dade exponential):
/// ε(X)_t = exp(X_t - X_0 - ½\[X\]_t) Π_{s≤t} (1 + ΔX_s) e^{-ΔX_s}.
pub fn stochastic_exponential_ty() -> Expr {
    arrow(stochastic_process_ty(), stochastic_process_ty())
}
/// `stochastic_exponential_martingale : StochasticProcess → Prop`
/// The stochastic exponential ε(M) of a local martingale M is a local martingale.
pub fn stochastic_exponential_martingale_ty() -> Expr {
    arrow(stochastic_process_ty(), prop())
}
/// `novikov_condition : AdaptedProcess → Prop`
/// Novikov's condition: E\[exp(½ ∫₀ᵀ θ²_s ds)\] < ∞ implies ε(∫ θ dW) is a true martingale.
pub fn novikov_condition_ty() -> Expr {
    arrow(cst("AdaptedProcess"), prop())
}
/// `cameron_martin_theorem : (Real → Real) → BrownianMotionProcess → Prop`
/// Cameron-Martin theorem: shifting Brownian motion by a Cameron-Martin function h
/// changes the measure by the Radon-Nikodym derivative exp(∫ h dW - ½ ∫ h² dt).
pub fn cameron_martin_theorem_ty() -> Expr {
    arrow(
        arrow(real_ty(), real_ty()),
        arrow(cst("BrownianMotionProcess"), prop()),
    )
}
/// `girsanov_multidim : Nat → AdaptedProcess → Prop`
/// Multidimensional Girsanov: for d-dimensional Brownian motion and adapted θ,
/// W̃_t = W_t - ∫₀ᵗ θ_s ds is a Q-Brownian motion under dQ/dP = ε(∫ θ·dW).
pub fn girsanov_multidim_ty() -> Expr {
    arrow(nat_ty(), arrow(cst("AdaptedProcess"), prop()))
}
/// `equivalent_martingale_measure : StochasticProcess → Prop`
/// Existence of an equivalent martingale measure (risk-neutral measure) via Girsanov.
pub fn equivalent_martingale_measure_ty() -> Expr {
    arrow(stochastic_process_ty(), prop())
}
/// `first_fundamental_theorem_am : StochasticProcess → Prop`
/// First fundamental theorem of asset management: no-arbitrage ↔ existence of EMM.
pub fn first_ftam_ty() -> Expr {
    arrow(stochastic_process_ty(), prop())
}
/// `MalliavinDerivative : Type`
/// The Malliavin derivative D: L²(Ω) → L²(Ω × \[0,T\]) is the gradient on Wiener space.
pub fn malliavin_derivative_ty() -> Expr {
    type0()
}
/// `SkorokhodIntegral : Type`
/// The Skorokhod integral δ (adjoint of D): extends Itô integral to non-adapted integrands.
pub fn skorokhod_integral_ty() -> Expr {
    type0()
}
/// `malliavin_integration_by_parts : MalliavinDerivative → SkorokhodIntegral → Prop`
/// Integration by parts: E\[F δ(u)\] = E\[〈DF, u〉_{L²}\] for smooth F and u in Dom(δ).
pub fn malliavin_integration_by_parts_ty() -> Expr {
    arrow(
        cst("MalliavinDerivative"),
        arrow(cst("SkorokhodIntegral"), prop()),
    )
}
/// `clark_ocone_formula : StochasticProcess → MalliavinDerivative → Prop`
/// Clark-Ocone formula: F = E\[F\] + ∫₀ᵀ E[D_t F | F_t] dW_t for F ∈ 𝔻^{1,2}.
pub fn clark_ocone_formula_ty() -> Expr {
    arrow(
        stochastic_process_ty(),
        arrow(cst("MalliavinDerivative"), prop()),
    )
}
/// `malliavin_smooth_functional : StochasticProcess → Prop`
/// A Brownian functional F is Malliavin smooth if it lies in 𝔻^{∞} = ∩_p ∩_k 𝔻^{k,p}.
pub fn malliavin_smooth_functional_ty() -> Expr {
    arrow(stochastic_process_ty(), prop())
}
/// `RoughPath : Type`
/// A rough path: a pair (X, X²) where X² is the area process ("Lévy area").
pub fn rough_path_ty() -> Expr {
    type0()
}
/// `ControlledRoughPath : Type`
/// A controlled rough path: a pair (Y, Y') such that Y_t - Y_s ≈ Y'_s (X_t - X_s).
pub fn controlled_rough_path_ty() -> Expr {
    type0()
}
/// `rough_path_integral : ControlledRoughPath → RoughPath → StochasticProcess → Prop`
/// The rough path integral ∫ Y dX is well-defined for controlled rough paths.
pub fn rough_path_integral_ty() -> Expr {
    arrow(
        cst("ControlledRoughPath"),
        arrow(cst("RoughPath"), arrow(stochastic_process_ty(), prop())),
    )
}
/// `rough_path_continuity : RoughPath → Real → Prop`
/// Continuity of rough path integrals: the integral map is continuous in the rough path topology.
pub fn rough_path_continuity_ty() -> Expr {
    arrow(cst("RoughPath"), arrow(real_ty(), prop()))
}
/// `brownian_rough_path : BrownianMotionProcess → RoughPath → Prop`
/// Enhanced Brownian motion: B can be lifted to a rough path using Lévy's area.
pub fn brownian_rough_path_ty() -> Expr {
    arrow(
        cst("BrownianMotionProcess"),
        arrow(cst("RoughPath"), prop()),
    )
}
/// `rough_path_rde_solution : RoughPath → StochasticProcess → Prop`
/// Rough differential equation dY = f(Y) dX has a unique solution for controlled Y.
pub fn rough_path_rde_solution_ty() -> Expr {
    arrow(cst("RoughPath"), arrow(stochastic_process_ty(), prop()))
}
/// `FellerSemigroup : Type`
/// A Feller semigroup (T_t)_{t≥0} on C_0(E): strongly continuous, contractive, positive.
pub fn feller_semigroup_ty() -> Expr {
    type0()
}
/// `feller_process_existence : FellerSemigroup → StochasticProcess → Prop`
/// For each Feller semigroup there exists a Feller process (Markov process with
/// càdlàg paths) with the given transition semigroup.
pub fn feller_process_existence_ty() -> Expr {
    arrow(
        cst("FellerSemigroup"),
        arrow(stochastic_process_ty(), prop()),
    )
}
/// `kolmogorov_forward_equation : (Real → Real) → (Real → Real) → (Real → Real → Real) → Prop`
/// Kolmogorov's forward (Fokker-Planck) equation for the density p(t,x) of X_t:
/// ∂_t p = -(∂_x \[μ p\]) + ½ ∂_x² \[σ² p\].
pub fn kolmogorov_forward_equation_ty() -> Expr {
    arrow(
        arrow(real_ty(), real_ty()),
        arrow(
            arrow(real_ty(), real_ty()),
            arrow(arrow(real_ty(), arrow(real_ty(), real_ty())), prop()),
        ),
    )
}
/// `kolmogorov_backward_equation : (Real → Real) → (Real → Real) → (Real → Real) → Prop`
/// Kolmogorov's backward equation: ∂_t u = μ(x) ∂_x u + ½ σ(x)² ∂_x² u.
pub fn kolmogorov_backward_equation_ty() -> Expr {
    arrow(
        arrow(real_ty(), real_ty()),
        arrow(
            arrow(real_ty(), real_ty()),
            arrow(arrow(real_ty(), real_ty()), prop()),
        ),
    )
}
/// `generator_feller : FellerSemigroup → (Real → Real → Real) → Prop`
/// The generator L of a Feller semigroup satisfies Lf = lim_{t→0} (T_t f - f)/t.
pub fn generator_feller_ty() -> Expr {
    arrow(
        cst("FellerSemigroup"),
        arrow(arrow(real_ty(), arrow(real_ty(), real_ty())), prop()),
    )
}
/// `ReflectedBrownianMotion : Type`
/// Brownian motion reflected at the boundary of a domain D.
pub fn reflected_brownian_motion_ty() -> Expr {
    type0()
}
/// `reflected_bm_existence : Real → ReflectedBrownianMotion → Prop`
/// Existence and uniqueness of reflected Brownian motion in a half-space or convex domain.
pub fn reflected_bm_existence_ty() -> Expr {
    arrow(real_ty(), arrow(cst("ReflectedBrownianMotion"), prop()))
}
/// `skorokhod_reflection_problem : StochasticProcess → ReflectedBrownianMotion → Prop`
/// The Skorokhod reflection problem: find (X, L) such that X_t = B_t + L_t ≥ 0
/// and L is increasing, continuous, and increases only when X_t = 0.
pub fn skorokhod_reflection_problem_ty() -> Expr {
    arrow(
        stochastic_process_ty(),
        arrow(cst("ReflectedBrownianMotion"), prop()),
    )
}
/// `tanaka_formula : BrownianMotionProcess → Real → Prop`
/// Tanaka's formula: |B_t - a| = |B_0 - a| + ∫₀ᵗ sgn(B_s - a) dB_s + L_t^a
/// where L^a is the local time at level a.
pub fn tanaka_formula_ty() -> Expr {
    arrow(cst("BrownianMotionProcess"), arrow(real_ty(), prop()))
}
/// `local_time : BrownianMotionProcess → Real → Real → Prop`
/// The local time L_t^a satisfies the occupation times formula ∫ f(B_s) d\[B\]_s = ∫ f(a) L_t^a da.
pub fn local_time_ty() -> Expr {
    arrow(
        cst("BrownianMotionProcess"),
        arrow(real_ty(), arrow(real_ty(), prop())),
    )
}
/// `InfinitelyDivisibleDistribution : Type`
/// A probability distribution μ is infinitely divisible if for every n, μ = μ_n^{*n}.
pub fn infinitely_divisible_distribution_ty() -> Expr {
    type0()
}
/// `levy_process_infinitely_divisible : StochasticProcess → InfinitelyDivisibleDistribution → Prop`
/// The distribution of X_1 for a Lévy process is infinitely divisible.
pub fn levy_process_infinitely_divisible_ty() -> Expr {
    arrow(
        stochastic_process_ty(),
        arrow(cst("InfinitelyDivisibleDistribution"), prop()),
    )
}
/// `PoissonRandomMeasure : Type`
/// A Poisson random measure N on (E, E) with intensity measure ν.
pub fn poisson_random_measure_ty() -> Expr {
    type0()
}
/// `poisson_random_measure_levy : LevyProcess → PoissonRandomMeasure → Prop`
/// Every Lévy process induces a Poisson random measure on its jump measure space.
pub fn poisson_random_measure_levy_ty() -> Expr {
    arrow(
        levy_process_ty(),
        arrow(cst("PoissonRandomMeasure"), prop()),
    )
}
/// `stable_process : Real → StochasticProcess`
/// A stable process with index α ∈ (0,2]: self-similar Lévy process with stable distribution.
pub fn stable_process_ty() -> Expr {
    arrow(real_ty(), stochastic_process_ty())
}
/// `subordinator : StochasticProcess → Prop`
/// A subordinator is a non-decreasing Lévy process (non-negative jumps only).
pub fn subordinator_ty() -> Expr {
    arrow(stochastic_process_ty(), prop())
}
/// `subordination : StochasticProcess → StochasticProcess → StochasticProcess`
/// Bochner's subordination: X_{T_t} where T is a subordinator produces a new Lévy process.
pub fn subordination_ty() -> Expr {
    arrow(
        stochastic_process_ty(),
        arrow(stochastic_process_ty(), stochastic_process_ty()),
    )
}
pub(super) fn sp_ext_gamma_sample(a: f64, b: f64, lcg: &mut super::types::Lcg) -> f64 {
    if a <= 0.0 || b <= 0.0 {
        return 0.0;
    }
    let n = (a.ceil() as u32).max(1);
    let mut sum = 0.0f64;
    for _ in 0..n {
        let u = lcg.next_f64().max(1e-15);
        sum += (-u.ln()) / b;
    }
    sum * (a / n as f64)
}
