//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{Distribution, ParticleFilter};

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
pub fn type1() -> Expr {
    Expr::Sort(Level::succ(Level::succ(Level::zero())))
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
pub fn bool_ty() -> Expr {
    cst("Bool")
}
pub fn list_ty(elem: Expr) -> Expr {
    app(cst("List"), elem)
}
/// `Measure : Type → Type` — a σ-finite measure on a measurable space.
pub fn measure_ty() -> Expr {
    arrow(type0(), type0())
}
/// `SigmaAlgebra : Type → Type` — a σ-algebra of subsets.
pub fn sigma_algebra_ty() -> Expr {
    arrow(type0(), type0())
}
/// `MeasurableSpace : Type` — a type equipped with a σ-algebra.
pub fn measurable_space_ty() -> Expr {
    type0()
}
/// `ProbabilityMonad : (Type → Type)` — the distribution / Giry monad.
pub fn probability_monad_ty() -> Expr {
    arrow(type0(), type0())
}
/// `Kernel : Type → Type → Type` — a Markov kernel k(x, A).
pub fn kernel_ty() -> Expr {
    arrow(type0(), arrow(type0(), type0()))
}
/// `Sampler : Type → Type` — a procedure that draws samples from a distribution.
pub fn sampler_ty() -> Expr {
    arrow(type0(), type0())
}
/// `Density : Type → Type` — a density/pmf function on a type.
pub fn density_ty() -> Expr {
    arrow(type0(), arrow(type0(), real_ty()))
}
/// `PPLProgram : Type → Type` — a probabilistic program returning values of type A.
pub fn ppl_program_ty() -> Expr {
    arrow(type0(), type0())
}
/// `ELBO : (Type → Type) → Type → Real` — evidence lower bound for variational inference.
pub fn elbo_ty() -> Expr {
    arrow(arrow(type0(), type0()), arrow(type0(), real_ty()))
}
/// `ImportanceWeight : Type → Real` — self-normalised importance weight.
pub fn importance_weight_ty() -> Expr {
    arrow(type0(), real_ty())
}
/// `ParticleFilter : Type → Type` — sequential Monte Carlo state estimator.
pub fn particle_filter_ty() -> Expr {
    arrow(type0(), type0())
}
/// `GradientEstimator : Type → Type` — a Monte Carlo gradient estimator.
pub fn gradient_estimator_ty() -> Expr {
    arrow(type0(), type0())
}
/// **Measure-Theoretic Bayes**: the posterior is the Radon-Nikodym derivative
/// of the joint w.r.t. the marginal likelihood.
///
/// `bayes_measure_theory : ∀ (prior : Measure X) (likelihood : Kernel X Y),
///   Posterior prior likelihood = RadonNikodym (Joint prior likelihood) (Marginal likelihood)`
pub fn bayes_measure_theory_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        type0(),
        pi(
            BinderInfo::Default,
            "Y",
            type0(),
            pi(
                BinderInfo::Default,
                "prior",
                app(cst("Measure"), bvar(1)),
                pi(
                    BinderInfo::Default,
                    "likelihood",
                    app2(cst("Kernel"), bvar(2), bvar(1)),
                    app2(
                        cst("Eq"),
                        app2(cst("Posterior"), bvar(1), bvar(0)),
                        app2(
                            cst("RadonNikodym"),
                            app2(cst("Joint"), bvar(1), bvar(0)),
                            app(cst("Marginal"), bvar(0)),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// **Giry Monad Laws**: the distribution monad satisfies monad laws.
///
/// `giry_monad_laws : MonadLaws ProbabilityMonad`
pub fn giry_monad_laws_ty() -> Expr {
    app(cst("MonadLaws"), cst("ProbabilityMonad"))
}
/// **Importance Sampling Consistency**: the IS estimator is consistent as N→∞.
///
/// `is_consistency : ∀ (f : X → Real) (q p : Measure X),
///   AbsContinuous p q → ConsistentEstimator (ISSampler f q p)`
pub fn is_consistency_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        type0(),
        pi(
            BinderInfo::Default,
            "f",
            arrow(bvar(0), real_ty()),
            pi(
                BinderInfo::Default,
                "q",
                app(cst("Measure"), bvar(1)),
                pi(
                    BinderInfo::Default,
                    "p",
                    app(cst("Measure"), bvar(2)),
                    arrow(
                        app2(cst("AbsContinuous"), bvar(0), bvar(1)),
                        app(
                            cst("ConsistentEstimator"),
                            app3(cst("ISSampler"), bvar(3), bvar(1), bvar(0)),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// **ELBO Lower Bound**: ELBO(q) ≤ log p(x) for all variational families q.
///
/// `elbo_lower_bound : ∀ (q : Measure Z) (p : Joint Z X) (x : X),
///   ELBO q p x ≤ LogMarginalLikelihood p x`
pub fn elbo_lower_bound_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "Z",
        type0(),
        pi(
            BinderInfo::Default,
            "X",
            type0(),
            pi(
                BinderInfo::Default,
                "q",
                app(cst("Measure"), bvar(1)),
                pi(
                    BinderInfo::Default,
                    "p",
                    app2(cst("JointMeasure"), bvar(2), bvar(1)),
                    pi(
                        BinderInfo::Default,
                        "x",
                        bvar(2),
                        app2(
                            cst("Real.le"),
                            app3(cst("ELBO"), bvar(2), bvar(1), bvar(0)),
                            app2(cst("LogMarginalLikelihood"), bvar(1), bvar(0)),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// **Reparameterisation Gradient**: the reparameterised gradient estimator is
/// an unbiased estimator of ∇_φ E_{z~q_φ}[f(z)].
///
/// `reparam_unbiased : ∀ (φ : Params) (f : Z → Real),
///   Unbiased (ReparamGradient f) (GradExpectation f φ)`
pub fn reparam_unbiased_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "Z",
        type0(),
        pi(
            BinderInfo::Default,
            "Params",
            type0(),
            pi(
                BinderInfo::Default,
                "phi",
                bvar(0),
                pi(
                    BinderInfo::Default,
                    "f",
                    arrow(bvar(2), real_ty()),
                    app2(
                        cst("Unbiased"),
                        app(cst("ReparamGradient"), bvar(0)),
                        app2(cst("GradExpectation"), bvar(0), bvar(1)),
                    ),
                ),
            ),
        ),
    )
}
/// **HMC Correctness**: Hamiltonian Monte Carlo leaves the target distribution invariant.
///
/// `hmc_invariant : ∀ (target : Measure X), InvariantUnder (HMCKernel target) target`
pub fn hmc_invariant_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        type0(),
        pi(
            BinderInfo::Default,
            "target",
            app(cst("Measure"), bvar(0)),
            app2(
                cst("InvariantUnder"),
                app(cst("HMCKernel"), bvar(0)),
                bvar(0),
            ),
        ),
    )
}
/// **SMC Consistency**: sequential Monte Carlo converges to the true filtering distribution.
pub fn smc_consistency_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        type0(),
        pi(
            BinderInfo::Default,
            "ssm",
            app(cst("StateSpaceModel"), bvar(0)),
            app(cst("ConsistentFilteringDist"), bvar(0)),
        ),
    )
}
/// **Stochastic Variational Inference (SVI) Convergence**: SVI converges to a local ELBO maximum.
///
/// `svi_convergence : ∀ (q_family : VariationalFamily) (lr_schedule : LRSchedule),
///   ConvergesToLocalOptimum (SVIOptimizer q_family lr_schedule)`
pub fn svi_convergence_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "VF",
        type0(),
        pi(
            BinderInfo::Default,
            "lr",
            type0(),
            app(
                cst("ConvergesToLocalOptimum"),
                app2(cst("SVIOptimizer"), bvar(1), bvar(0)),
            ),
        ),
    )
}
/// **Normalizing Flow Change of Variables**: the pushforward density satisfies the
/// change-of-variables formula.
///
/// `normalizing_flow_cov : ∀ (f : X → Y) (p : Measure X),
///   Bijective f → Density (Pushforward f p) y = Density p (f_inv y) * AbsDetJac f_inv y`
pub fn normalizing_flow_cov_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        type0(),
        pi(
            BinderInfo::Default,
            "Y",
            type0(),
            pi(
                BinderInfo::Default,
                "f",
                arrow(bvar(1), bvar(0)),
                pi(
                    BinderInfo::Default,
                    "p",
                    app(cst("Measure"), bvar(2)),
                    arrow(
                        app(cst("Bijective"), bvar(1)),
                        app(cst("FlowDensityEq"), bvar(2)),
                    ),
                ),
            ),
        ),
    )
}
/// **Score Function Estimator Unbiasedness**: the REINFORCE estimator is an unbiased
/// gradient estimator under mild regularity conditions.
///
/// `score_fn_unbiased : ∀ (q_phi : ParametricMeasure) (f : Z → Real),
///   RegularFamily q_phi → Unbiased (ScoreFnGrad q_phi f) (GradELBO q_phi f)`
pub fn score_fn_unbiased_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "Z",
        type0(),
        pi(
            BinderInfo::Default,
            "q_phi",
            app(cst("ParametricMeasure"), bvar(0)),
            pi(
                BinderInfo::Default,
                "f",
                arrow(bvar(1), real_ty()),
                arrow(
                    app(cst("RegularFamily"), bvar(1)),
                    app2(
                        cst("Unbiased"),
                        app2(cst("ScoreFnGrad"), bvar(1), bvar(0)),
                        app2(cst("GradELBO"), bvar(1), bvar(0)),
                    ),
                ),
            ),
        ),
    )
}
/// **Pathwise Gradient Unbiasedness**: the reparameterised (pathwise) gradient is
/// an unbiased estimator when the reparameterisation is differentiable.
///
/// `pathwise_gradient_unbiased : ∀ (g : Eps → Phi → Z) (f : Z → Real),
///   DiffReparameterisation g → Unbiased (PathwiseGrad g f) (GradELBO (Reparam g) f)`
pub fn pathwise_gradient_unbiased_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "Eps",
        type0(),
        pi(
            BinderInfo::Default,
            "Phi",
            type0(),
            pi(
                BinderInfo::Default,
                "Z",
                type0(),
                pi(
                    BinderInfo::Default,
                    "g",
                    arrow(bvar(2), arrow(bvar(1), bvar(0))),
                    pi(
                        BinderInfo::Default,
                        "f",
                        arrow(bvar(1), real_ty()),
                        arrow(
                            app(cst("DiffReparameterisation"), bvar(1)),
                            app2(
                                cst("Unbiased"),
                                app2(cst("PathwiseGrad"), bvar(1), bvar(0)),
                                app2(cst("GradELBO"), app(cst("Reparam"), bvar(1)), bvar(0)),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// **Measure Transport Existence**: for any two probability measures with the same
/// total mass there exists a measurable transport map.
///
/// `measure_transport_exists : ∀ (mu nu : ProbMeasure X),
///   ∃ (T : X → X), Pushforward T mu = nu`
pub fn measure_transport_exists_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        type0(),
        pi(
            BinderInfo::Default,
            "mu",
            app(cst("ProbMeasure"), bvar(0)),
            pi(
                BinderInfo::Default,
                "nu",
                app(cst("ProbMeasure"), bvar(1)),
                app(
                    cst("Exists"),
                    pi(
                        BinderInfo::Default,
                        "T",
                        arrow(bvar(2), bvar(2)),
                        app2(
                            cst("Eq"),
                            app2(cst("Pushforward"), bvar(0), bvar(2)),
                            bvar(1),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// **Optimal Transport Kantorovich Duality**: the Wasserstein-1 distance equals
/// the supremum of Lipschitz-1 functions.
///
/// `ot_kantorovich : ∀ (mu nu : ProbMeasure X),
///   W1 mu nu = sup_{f : 1-Lip} (E_mu\[f\] - E_nu\[f\])`
pub fn ot_kantorovich_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        type0(),
        pi(
            BinderInfo::Default,
            "mu",
            app(cst("ProbMeasure"), bvar(0)),
            pi(
                BinderInfo::Default,
                "nu",
                app(cst("ProbMeasure"), bvar(1)),
                app2(
                    cst("Eq"),
                    app2(cst("W1Dist"), bvar(1), bvar(0)),
                    app2(cst("KantorovichDual"), bvar(1), bvar(0)),
                ),
            ),
        ),
    )
}
/// **Stein Identity**: for any smooth function h and score function s_p = ∇ log p,
/// E_p\[∇ h(x) + h(x) s_p(x)\] = 0.
///
/// `stein_identity : ∀ (p : SmoothMeasure X) (h : X → Real),
///   SmoothTestFn h → E_p\[SteinOp p h\] = 0`
pub fn stein_identity_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        type0(),
        pi(
            BinderInfo::Default,
            "p",
            app(cst("SmoothMeasure"), bvar(0)),
            pi(
                BinderInfo::Default,
                "h",
                arrow(bvar(1), real_ty()),
                arrow(
                    app(cst("SmoothTestFn"), bvar(0)),
                    app2(
                        cst("Eq"),
                        app2(
                            cst("Expectation"),
                            bvar(1),
                            app2(cst("SteinOp"), bvar(1), bvar(0)),
                        ),
                        cst("Real.zero"),
                    ),
                ),
            ),
        ),
    )
}
/// **Stein Variational Gradient Descent Convergence**: SVGD converges to the target
/// distribution in the Stein discrepancy sense.
///
/// `svgd_convergence : ∀ (target : SmoothMeasure X) (n : Nat),
///   SteinDiscrepancy (SVGDUpdate target n) target ≤ SVGDBound n`
pub fn svgd_convergence_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        type0(),
        pi(
            BinderInfo::Default,
            "target",
            app(cst("SmoothMeasure"), bvar(0)),
            pi(
                BinderInfo::Default,
                "n",
                nat_ty(),
                app2(
                    cst("Real.le"),
                    app2(
                        cst("SteinDiscrepancy"),
                        app2(cst("SVGDUpdate"), bvar(1), bvar(0)),
                        bvar(1),
                    ),
                    app(cst("SVGDBound"), bvar(0)),
                ),
            ),
        ),
    )
}
/// **Population Monte Carlo Consistency**: PMC estimators are consistent as
/// population size and iterations grow.
///
/// `pmc_consistency : ∀ (N : Nat) (T : Nat) (target : Measure X),
///   ConsistentEstimator (PMCEstimator target N T)`
pub fn pmc_consistency_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        type0(),
        pi(
            BinderInfo::Default,
            "N",
            nat_ty(),
            pi(
                BinderInfo::Default,
                "T",
                nat_ty(),
                pi(
                    BinderInfo::Default,
                    "target",
                    app(cst("Measure"), bvar(2)),
                    app(
                        cst("ConsistentEstimator"),
                        app3(cst("PMCEstimator"), bvar(0), bvar(2), bvar(1)),
                    ),
                ),
            ),
        ),
    )
}
/// **Evolutionary MCMC Detailed Balance**: evolutionary MCMC satisfies detailed
/// balance with respect to a product-form invariant distribution.
///
/// `evol_mcmc_detailed_balance : ∀ (target : Measure X) (temp : Tempering),
///   DetailedBalance (EvolMCMCKernel target temp) (TemperedTarget target temp)`
pub fn evol_mcmc_detailed_balance_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        type0(),
        pi(
            BinderInfo::Default,
            "target",
            app(cst("Measure"), bvar(0)),
            pi(
                BinderInfo::Default,
                "temp",
                cst("Tempering"),
                app2(
                    cst("DetailedBalance"),
                    app2(cst("EvolMCMCKernel"), bvar(1), bvar(0)),
                    app2(cst("TemperedTarget"), bvar(1), bvar(0)),
                ),
            ),
        ),
    )
}
/// **Parallel Tempering Exchange Correctness**: the swap move in parallel tempering
/// preserves the joint invariant distribution.
///
/// `parallel_tempering_exchange : ∀ (temps : List Real) (joint : Measure (ProductSpace temps)),
///   InvariantUnder (SwapKernel temps) joint`
pub fn parallel_tempering_exchange_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "temps",
        list_ty(real_ty()),
        pi(
            BinderInfo::Default,
            "joint",
            app(cst("Measure"), app(cst("ProductSpace"), bvar(0))),
            app2(
                cst("InvariantUnder"),
                app(cst("SwapKernel"), bvar(1)),
                bvar(0),
            ),
        ),
    )
}
/// **Simulated Annealing Convergence**: simulated annealing converges to a global
/// optimum under a logarithmic cooling schedule.
///
/// `simulated_annealing_convergence : ∀ (f : X → Real) (T : CoolingSchedule),
///   LogarithmicSchedule T → ConvergesToGlobalOpt (SAChain f T)`
pub fn simulated_annealing_convergence_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        type0(),
        pi(
            BinderInfo::Default,
            "f",
            arrow(bvar(0), real_ty()),
            pi(
                BinderInfo::Default,
                "T",
                cst("CoolingSchedule"),
                arrow(
                    app(cst("LogarithmicSchedule"), bvar(0)),
                    app(
                        cst("ConvergesToGlobalOpt"),
                        app2(cst("SAChain"), bvar(2), bvar(0)),
                    ),
                ),
            ),
        ),
    )
}
/// **VAE ELBO Decomposition**: the VAE objective decomposes as reconstruction term
/// minus KL divergence.
///
/// `vae_elbo_decomp : ∀ (encoder decoder : NeuralNet) (x : X),
///   VAELBO encoder decoder x = Reconstruction encoder decoder x - KLDivQ encoder x`
pub fn vae_elbo_decomp_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        type0(),
        pi(
            BinderInfo::Default,
            "encoder",
            cst("NeuralNet"),
            pi(
                BinderInfo::Default,
                "decoder",
                cst("NeuralNet"),
                pi(
                    BinderInfo::Default,
                    "x",
                    bvar(2),
                    app2(
                        cst("Eq"),
                        app3(cst("VAELBO"), bvar(2), bvar(1), bvar(0)),
                        app2(
                            cst("Real.sub"),
                            app3(cst("Reconstruction"), bvar(2), bvar(1), bvar(0)),
                            app2(cst("KLDivQ"), bvar(2), bvar(0)),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// **Diffusion Model Score Matching**: the reverse diffusion score function
/// minimises the denoising score matching objective.
///
/// `diffusion_score_matching : ∀ (p_data : Measure X) (sigma : Real),
///   OptimScore (DSMObjective p_data sigma) = ScoreFunction (GaussianSmooth p_data sigma)`
pub fn diffusion_score_matching_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        type0(),
        pi(
            BinderInfo::Default,
            "p_data",
            app(cst("Measure"), bvar(0)),
            pi(
                BinderInfo::Default,
                "sigma",
                real_ty(),
                app2(
                    cst("Eq"),
                    app(
                        cst("OptimScore"),
                        app2(cst("DSMObjective"), bvar(1), bvar(0)),
                    ),
                    app(
                        cst("ScoreFunction"),
                        app2(cst("GaussianSmooth"), bvar(1), bvar(0)),
                    ),
                ),
            ),
        ),
    )
}
/// **Flow Matching ODE Correctness**: the conditional flow matching ODE generates
/// the correct marginal distribution at time t=1.
///
/// `flow_matching_ode : ∀ (p_0 p_1 : Measure X) (vt : VectorField),
///   CondFlowMatchingField vt p_0 p_1 → PushforwardODE vt p_0 1 = p_1`
pub fn flow_matching_ode_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        type0(),
        pi(
            BinderInfo::Default,
            "p_0",
            app(cst("Measure"), bvar(0)),
            pi(
                BinderInfo::Default,
                "p_1",
                app(cst("Measure"), bvar(1)),
                pi(
                    BinderInfo::Default,
                    "vt",
                    cst("VectorField"),
                    arrow(
                        app3(cst("CondFlowMatchingField"), bvar(0), bvar(2), bvar(1)),
                        app2(
                            cst("Eq"),
                            app3(cst("PushforwardODE"), bvar(0), bvar(2), cst("Real.one")),
                            bvar(1),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// **Gaussian Process Posterior**: the posterior of a GP given observations is also a GP.
///
/// `gp_posterior_is_gp : ∀ (prior : GaussianProcess X) (obs : Observations),
///   IsGaussianProcess (GPPosterior prior obs)`
pub fn gp_posterior_is_gp_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        type0(),
        pi(
            BinderInfo::Default,
            "prior",
            app(cst("GaussianProcess"), bvar(0)),
            pi(
                BinderInfo::Default,
                "obs",
                cst("Observations"),
                app(
                    cst("IsGaussianProcess"),
                    app2(cst("GPPosterior"), bvar(1), bvar(0)),
                ),
            ),
        ),
    )
}
/// **GP Marginal Likelihood**: the marginal likelihood of a GP is Gaussian.
///
/// `gp_marginal_gaussian : ∀ (gp : GaussianProcess X) (X_train : List X),
///   MarginalLikelihood gp X_train = MultivariateGaussian (GPMean gp X_train) (GPKernelMatrix gp X_train)`
pub fn gp_marginal_gaussian_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        type0(),
        pi(
            BinderInfo::Default,
            "gp",
            app(cst("GaussianProcess"), bvar(0)),
            pi(
                BinderInfo::Default,
                "X_train",
                list_ty(bvar(1)),
                app2(
                    cst("Eq"),
                    app2(cst("MarginalLikelihood"), bvar(1), bvar(0)),
                    app2(
                        cst("MultivariateGaussian"),
                        app2(cst("GPMean"), bvar(1), bvar(0)),
                        app2(cst("GPKernelMatrix"), bvar(1), bvar(0)),
                    ),
                ),
            ),
        ),
    )
}
/// **Probabilistic Numerics Integration**: Bayesian quadrature produces a posterior
/// over integrals.
///
/// `pn_integration : ∀ (f : X → Real) (prior_gp : GaussianProcess X),
///   BQPosterior prior_gp f = GaussianMeasureOver Real`
pub fn pn_integration_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        type0(),
        pi(
            BinderInfo::Default,
            "f",
            arrow(bvar(0), real_ty()),
            pi(
                BinderInfo::Default,
                "prior_gp",
                app(cst("GaussianProcess"), bvar(1)),
                app2(
                    cst("Eq"),
                    app2(cst("BQPosterior"), bvar(0), bvar(1)),
                    app(cst("GaussianMeasureOver"), real_ty()),
                ),
            ),
        ),
    )
}
/// **Stein Discrepancy Zero Iff Same Distribution**: the kernel Stein discrepancy
/// between two measures is zero if and only if they are equal.
///
/// `stein_disc_zero_iff : ∀ (p q : Measure X) (k : SteinKernel X),
///   KSD p q k = 0 ↔ p = q`
pub fn stein_disc_zero_iff_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        type0(),
        pi(
            BinderInfo::Default,
            "p",
            app(cst("Measure"), bvar(0)),
            pi(
                BinderInfo::Default,
                "q",
                app(cst("Measure"), bvar(1)),
                pi(
                    BinderInfo::Default,
                    "k",
                    app(cst("SteinKernel"), bvar(2)),
                    app2(
                        cst("Iff"),
                        app2(
                            cst("Eq"),
                            app3(cst("KSD"), bvar(2), bvar(1), bvar(0)),
                            cst("Real.zero"),
                        ),
                        app2(cst("Eq"), bvar(2), bvar(1)),
                    ),
                ),
            ),
        ),
    )
}
/// **SMC Feynman-Kac**: SMC computes the Feynman-Kac normalising constant exactly
/// in expectation.
///
/// `smc_feynman_kac : ∀ (fk : FeynmanKacModel X) (N : Nat),
///   E\[SMCNormConst fk N\] = FeynmanKacNormConst fk`
pub fn smc_feynman_kac_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        type0(),
        pi(
            BinderInfo::Default,
            "fk",
            app(cst("FeynmanKacModel"), bvar(0)),
            pi(
                BinderInfo::Default,
                "N",
                nat_ty(),
                app2(
                    cst("Eq"),
                    app(
                        cst("Expectation"),
                        app2(cst("SMCNormConst"), bvar(1), bvar(0)),
                    ),
                    app(cst("FeynmanKacNormConst"), bvar(1)),
                ),
            ),
        ),
    )
}
/// **Particle Marginal Metropolis-Hastings Correctness**: PMMH targeting the exact
/// posterior is asymptotically exact.
///
/// `pmmh_correctness : ∀ (model : LatentModel X Y) (obs : Y),
///   TargetsExactPosterior (PMMHKernel model obs)`
pub fn pmmh_correctness_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        type0(),
        pi(
            BinderInfo::Default,
            "Y",
            type0(),
            pi(
                BinderInfo::Default,
                "model",
                app2(cst("LatentModel"), bvar(1), bvar(0)),
                pi(
                    BinderInfo::Default,
                    "obs",
                    bvar(1),
                    app(
                        cst("TargetsExactPosterior"),
                        app2(cst("PMMHKernel"), bvar(1), bvar(0)),
                    ),
                ),
            ),
        ),
    )
}
/// **Annealed Importance Sampling Unbiasedness**: AIS produces unbiased estimates
/// of the normalising constant.
///
/// `ais_unbiased : ∀ (p_0 p_T : Measure X) (beta_sched : AnnealingSchedule),
///   Unbiased (AISEstimator p_0 p_T beta_sched) (NormConst p_T)`
pub fn ais_unbiased_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        type0(),
        pi(
            BinderInfo::Default,
            "p_0",
            app(cst("Measure"), bvar(0)),
            pi(
                BinderInfo::Default,
                "p_T",
                app(cst("Measure"), bvar(1)),
                pi(
                    BinderInfo::Default,
                    "beta_sched",
                    cst("AnnealingSchedule"),
                    app2(
                        cst("Unbiased"),
                        app3(cst("AISEstimator"), bvar(2), bvar(1), bvar(0)),
                        app(cst("NormConst"), bvar(1)),
                    ),
                ),
            ),
        ),
    )
}
/// **Denoising Score Matching Objective**: DSM objective equals implicit score matching
/// objective under Gaussian noise.
///
/// `dsm_equals_sm : ∀ (p_data : Measure X) (sigma : Real),
///   DSMObjective p_data sigma = ImplicitSMObjective (GaussianConvolution p_data sigma)`
pub fn dsm_equals_sm_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        type0(),
        pi(
            BinderInfo::Default,
            "p_data",
            app(cst("Measure"), bvar(0)),
            pi(
                BinderInfo::Default,
                "sigma",
                real_ty(),
                app2(
                    cst("Eq"),
                    app2(cst("DSMObjective"), bvar(1), bvar(0)),
                    app(
                        cst("ImplicitSMObjective"),
                        app2(cst("GaussianConvolution"), bvar(1), bvar(0)),
                    ),
                ),
            ),
        ),
    )
}
/// **Langevin Dynamics Convergence**: the unadjusted Langevin algorithm (ULA) converges
/// to the target in 2-Wasserstein under strong convexity.
///
/// `langevin_convergence : ∀ (target : LogConcaveMeasure X) (eps : Real),
///   W2 (ULADist target eps n) target ≤ LangevinBound target eps n`
pub fn langevin_convergence_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        type0(),
        pi(
            BinderInfo::Default,
            "target",
            app(cst("LogConcaveMeasure"), bvar(0)),
            pi(
                BinderInfo::Default,
                "eps",
                real_ty(),
                pi(
                    BinderInfo::Default,
                    "n",
                    nat_ty(),
                    app2(
                        cst("Real.le"),
                        app3(
                            cst("W2"),
                            app3(cst("ULADist"), bvar(2), bvar(1), bvar(0)),
                            bvar(2),
                            bvar(1),
                        ),
                        app3(cst("LangevinBound"), bvar(2), bvar(1), bvar(0)),
                    ),
                ),
            ),
        ),
    )
}
/// **Metropolis-Hastings Detailed Balance**: MH kernel satisfies detailed balance
/// w.r.t. the target distribution.
///
/// `mh_detailed_balance : ∀ (target : Measure X) (proposal : Kernel X X),
///   DetailedBalance (MHKernel target proposal) target`
pub fn mh_detailed_balance_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        type0(),
        pi(
            BinderInfo::Default,
            "target",
            app(cst("Measure"), bvar(0)),
            pi(
                BinderInfo::Default,
                "proposal",
                app2(cst("Kernel"), bvar(1), bvar(1)),
                app2(
                    cst("DetailedBalance"),
                    app2(cst("MHKernel"), bvar(1), bvar(0)),
                    bvar(1),
                ),
            ),
        ),
    )
}
/// **Gibbs Sampling Invariance**: the Gibbs sampler leaves the joint distribution invariant.
///
/// `gibbs_invariant : ∀ (joint : Measure (X × Y)),
///   InvariantUnder (GibbsKernel joint) joint`
pub fn gibbs_invariant_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        type0(),
        pi(
            BinderInfo::Default,
            "Y",
            type0(),
            pi(
                BinderInfo::Default,
                "joint",
                app(cst("Measure"), app2(cst("Prod"), bvar(1), bvar(0))),
                app2(
                    cst("InvariantUnder"),
                    app(cst("GibbsKernel"), bvar(0)),
                    bvar(0),
                ),
            ),
        ),
    )
}
/// **Variational Autoencoder Posterior Collapse**: with a sufficiently expressive decoder
/// there exists a risk of posterior collapse.
///
/// `vae_posterior_collapse_risk : ∀ (decoder : ExpressiveDecoder) (beta : Real),
///   beta < 1 → AvoidsCollapse (BetaVAE decoder beta)`
pub fn vae_posterior_collapse_risk_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "decoder",
        cst("ExpressiveDecoder"),
        pi(
            BinderInfo::Default,
            "beta",
            real_ty(),
            arrow(
                app2(cst("Real.lt"), bvar(0), cst("Real.one")),
                app(
                    cst("AvoidsCollapse"),
                    app2(cst("BetaVAE"), bvar(1), bvar(0)),
                ),
            ),
        ),
    )
}
/// **Gradient of Log Normalizer**: the gradient of the log normalizer of an exponential
/// family equals the mean of the sufficient statistics.
///
/// `grad_log_normalizer : ∀ (eta : NaturalParams) (T : SuffStat),
///   Gradient (LogNormalizer T) eta = MeanSuffStat T (ExpFamilyDist T eta)`
pub fn grad_log_normalizer_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "T",
        type0(),
        pi(
            BinderInfo::Default,
            "eta",
            app(cst("NaturalParams"), bvar(0)),
            app2(
                cst("Eq"),
                app(cst("Gradient"), app(cst("LogNormalizer"), bvar(0))),
                app2(
                    cst("MeanSuffStat"),
                    bvar(0),
                    app2(cst("ExpFamilyDist"), bvar(0), bvar(1)),
                ),
            ),
        ),
    )
}
/// **Sequential Monte Carlo Genealogy**: the ancestral lineage in SMC traces back
/// through the resampling steps.
///
/// `smc_genealogy : ∀ (pf : ParticleSystem X T),
///   AncestralLineage pf = CoalescentProcess (ResamplingTimes pf)`
pub fn smc_genealogy_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        type0(),
        pi(
            BinderInfo::Default,
            "T",
            nat_ty(),
            pi(
                BinderInfo::Default,
                "pf",
                app2(cst("ParticleSystem"), bvar(1), bvar(0)),
                app2(
                    cst("Eq"),
                    app(cst("AncestralLineage"), bvar(0)),
                    app(
                        cst("CoalescentProcess"),
                        app(cst("ResamplingTimes"), bvar(0)),
                    ),
                ),
            ),
        ),
    )
}
/// **Kernel Density Estimation Consistency**: the KDE converges to the true density
/// in L2 as n → ∞ with optimal bandwidth.
///
/// `kde_consistency : ∀ (p : SmoothMeasure X) (h : BandwidthSeq),
///   OptimalBandwidth h → L2Convergence (KDEn p h)`
pub fn kde_consistency_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        type0(),
        pi(
            BinderInfo::Default,
            "p",
            app(cst("SmoothMeasure"), bvar(0)),
            pi(
                BinderInfo::Default,
                "h",
                cst("BandwidthSeq"),
                arrow(
                    app(cst("OptimalBandwidth"), bvar(0)),
                    app(cst("L2Convergence"), app2(cst("KDEn"), bvar(1), bvar(0))),
                ),
            ),
        ),
    )
}
/// **Variational Inference Mean-Field Factorization**: the mean-field approximation
/// optimises each factor holding others fixed via coordinate ascent.
///
/// `mean_field_cavi : ∀ (joint : Measure Z) (q_factors : List (Measure Z)),
///   CAVIStep joint q_factors = UpdatedFactors joint q_factors`
pub fn mean_field_cavi_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "Z",
        type0(),
        pi(
            BinderInfo::Default,
            "joint",
            app(cst("Measure"), bvar(0)),
            pi(
                BinderInfo::Default,
                "q_factors",
                list_ty(app(cst("Measure"), bvar(1))),
                app2(
                    cst("Eq"),
                    app2(cst("CAVIStep"), bvar(1), bvar(0)),
                    app2(cst("UpdatedFactors"), bvar(1), bvar(0)),
                ),
            ),
        ),
    )
}
/// **Probabilistic Backpropagation Gaussian Propagation**: PBP propagates a
/// Gaussian approximation through each layer of a neural network.
///
/// `pbp_gaussian_propagation : ∀ (net : BayesianNeuralNet) (x : Input),
///   GaussianApproxActivations (PBP net x) = PBPActivations net x`
pub fn pbp_gaussian_propagation_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "net",
        cst("BayesianNeuralNet"),
        pi(
            BinderInfo::Default,
            "x",
            cst("Input"),
            app2(
                cst("Eq"),
                app(
                    cst("GaussianApproxActivations"),
                    app2(cst("PBP"), bvar(1), bvar(0)),
                ),
                app2(cst("PBPActivations"), bvar(1), bvar(0)),
            ),
        ),
    )
}
/// **Expectation Propagation Fixed Point**: EP converges when the cavity distribution
/// and tilted distribution agree.
///
/// `ep_fixed_point : ∀ (model : FactorGraph) (approx : GaussianApprox),
///   EPFixedPoint approx model ↔ CavityAgreement approx model`
pub fn ep_fixed_point_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "model",
        cst("FactorGraph"),
        pi(
            BinderInfo::Default,
            "approx",
            cst("GaussianApprox"),
            app2(
                cst("Iff"),
                app2(cst("EPFixedPoint"), bvar(0), bvar(1)),
                app2(cst("CavityAgreement"), bvar(0), bvar(1)),
            ),
        ),
    )
}
/// **Nested Monte Carlo Estimator Bias**: nested MC estimators are biased but
/// consistent as the inner sample size grows.
///
/// `nested_mc_bias : ∀ (outer inner : Nat) (f : X → Real),
///   Bias (NestedMCEstimator f outer inner) ≤ NestedMCBiasRate inner`
pub fn nested_mc_bias_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        type0(),
        pi(
            BinderInfo::Default,
            "outer",
            nat_ty(),
            pi(
                BinderInfo::Default,
                "inner",
                nat_ty(),
                pi(
                    BinderInfo::Default,
                    "f",
                    arrow(bvar(2), real_ty()),
                    app2(
                        cst("Real.le"),
                        app(
                            cst("Bias"),
                            app3(cst("NestedMCEstimator"), bvar(0), bvar(2), bvar(1)),
                        ),
                        app(cst("NestedMCBiasRate"), bvar(1)),
                    ),
                ),
            ),
        ),
    )
}
/// **Approximate Bayesian Computation Consistency**: ABC-SMC converges to the
/// correct posterior as the tolerance ε → 0.
///
/// `abc_smc_consistency : ∀ (prior : Measure Theta) (sim : Theta → Measure Y) (eps : Real),
///   eps > 0 → ApproxPosterior (ABCSMC prior sim eps) eps (TruePosterior prior sim)`
pub fn abc_smc_consistency_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "Theta",
        type0(),
        pi(
            BinderInfo::Default,
            "Y",
            type0(),
            pi(
                BinderInfo::Default,
                "prior",
                app(cst("Measure"), bvar(1)),
                pi(
                    BinderInfo::Default,
                    "sim",
                    arrow(bvar(2), app(cst("Measure"), bvar(1))),
                    pi(
                        BinderInfo::Default,
                        "eps",
                        real_ty(),
                        arrow(
                            app2(cst("Real.gt"), bvar(0), cst("Real.zero")),
                            app3(
                                cst("ApproxPosterior"),
                                app3(cst("ABCSMC"), bvar(3), bvar(1), bvar(0)),
                                bvar(0),
                                app2(cst("TruePosterior"), bvar(3), bvar(1)),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// Populate an `Environment` with all probabilistic-programming axiom declarations.
pub fn build_probabilistic_programming_env(
    env: &mut Environment,
) -> Result<(), Box<dyn std::error::Error>> {
    let axioms: &[(&str, Expr)] = &[
        ("Measure", measure_ty()),
        ("SigmaAlgebra", sigma_algebra_ty()),
        ("MeasurableSpace", measurable_space_ty()),
        ("ProbabilityMonad", probability_monad_ty()),
        ("Kernel", kernel_ty()),
        ("Sampler", sampler_ty()),
        ("PPLProgram", ppl_program_ty()),
        ("GradientEstimator", gradient_estimator_ty()),
        ("ParticleFilter", particle_filter_ty()),
        (
            "Joint",
            arrow(
                app(cst("Measure"), type0()),
                arrow(app2(cst("Kernel"), type0(), type0()), type0()),
            ),
        ),
        ("JointMeasure", arrow(type0(), arrow(type0(), type0()))),
        (
            "Marginal",
            arrow(app2(cst("Kernel"), type0(), type0()), type0()),
        ),
        (
            "Posterior",
            arrow(
                app(cst("Measure"), type0()),
                arrow(
                    app2(cst("Kernel"), type0(), type0()),
                    app(cst("Measure"), type0()),
                ),
            ),
        ),
        ("RadonNikodym", arrow(type0(), arrow(type0(), type0()))),
        (
            "AbsContinuous",
            arrow(
                app(cst("Measure"), type0()),
                arrow(app(cst("Measure"), type0()), prop()),
            ),
        ),
        ("ConsistentEstimator", arrow(type0(), prop())),
        (
            "ISSampler",
            arrow(
                arrow(type0(), real_ty()),
                arrow(
                    app(cst("Measure"), type0()),
                    arrow(app(cst("Measure"), type0()), type0()),
                ),
            ),
        ),
        (
            "ELBO",
            arrow(
                app(cst("Measure"), type0()),
                arrow(type0(), arrow(type0(), real_ty())),
            ),
        ),
        (
            "LogMarginalLikelihood",
            arrow(type0(), arrow(type0(), real_ty())),
        ),
        ("MonadLaws", arrow(arrow(type0(), type0()), prop())),
        ("Unbiased", arrow(type0(), arrow(type0(), prop()))),
        ("ReparamGradient", arrow(arrow(type0(), real_ty()), type0())),
        (
            "GradExpectation",
            arrow(arrow(type0(), real_ty()), arrow(type0(), type0())),
        ),
        (
            "HMCKernel",
            arrow(
                app(cst("Measure"), type0()),
                app2(cst("Kernel"), type0(), type0()),
            ),
        ),
        (
            "InvariantUnder",
            arrow(
                app2(cst("Kernel"), type0(), type0()),
                arrow(app(cst("Measure"), type0()), prop()),
            ),
        ),
        ("StateSpaceModel", arrow(type0(), type0())),
        ("ConsistentFilteringDist", arrow(type0(), prop())),
        ("bayes_measure_theory", bayes_measure_theory_ty()),
        ("giry_monad_laws", giry_monad_laws_ty()),
        ("is_consistency", is_consistency_ty()),
        ("elbo_lower_bound", elbo_lower_bound_ty()),
        ("reparam_unbiased", reparam_unbiased_ty()),
        ("hmc_invariant", hmc_invariant_ty()),
        ("smc_consistency", smc_consistency_ty()),
        ("VariationalFamily", arrow(type0(), type0())),
        ("LRSchedule", type0()),
        ("SVIOptimizer", arrow(type0(), arrow(type0(), type0()))),
        ("ConvergesToLocalOptimum", arrow(type0(), prop())),
        ("Bijective", arrow(arrow(type0(), type0()), prop())),
        ("FlowDensityEq", arrow(type0(), prop())),
        ("ParametricMeasure", arrow(type0(), type0())),
        ("RegularFamily", arrow(type0(), prop())),
        (
            "ScoreFnGrad",
            arrow(type0(), arrow(arrow(type0(), real_ty()), type0())),
        ),
        (
            "GradELBO",
            arrow(type0(), arrow(arrow(type0(), real_ty()), type0())),
        ),
        (
            "DiffReparameterisation",
            arrow(arrow(type0(), arrow(type0(), type0())), prop()),
        ),
        (
            "PathwiseGrad",
            arrow(
                arrow(type0(), arrow(type0(), type0())),
                arrow(arrow(type0(), real_ty()), type0()),
            ),
        ),
        (
            "Reparam",
            arrow(arrow(type0(), arrow(type0(), type0())), type0()),
        ),
        ("ProbMeasure", arrow(type0(), type0())),
        (
            "Pushforward",
            arrow(
                arrow(type0(), type0()),
                arrow(app(cst("Measure"), type0()), app(cst("Measure"), type0())),
            ),
        ),
        (
            "W1Dist",
            arrow(
                app(cst("Measure"), type0()),
                arrow(app(cst("Measure"), type0()), real_ty()),
            ),
        ),
        (
            "KantorovichDual",
            arrow(
                app(cst("Measure"), type0()),
                arrow(app(cst("Measure"), type0()), real_ty()),
            ),
        ),
        ("SmoothMeasure", arrow(type0(), type0())),
        ("SmoothTestFn", arrow(arrow(type0(), real_ty()), prop())),
        (
            "Expectation",
            arrow(app(cst("Measure"), type0()), arrow(type0(), real_ty())),
        ),
        (
            "SteinOp",
            arrow(
                app(cst("SmoothMeasure"), type0()),
                arrow(arrow(type0(), real_ty()), type0()),
            ),
        ),
        ("Real.zero", real_ty()),
        ("Real.one", real_ty()),
        (
            "SteinDiscrepancy",
            arrow(
                app(cst("Measure"), type0()),
                arrow(app(cst("Measure"), type0()), real_ty()),
            ),
        ),
        (
            "SVGDUpdate",
            arrow(
                app(cst("SmoothMeasure"), type0()),
                arrow(nat_ty(), app(cst("Measure"), type0())),
            ),
        ),
        ("SVGDBound", arrow(nat_ty(), real_ty())),
        ("SteinKernel", arrow(type0(), type0())),
        (
            "KSD",
            arrow(
                app(cst("Measure"), type0()),
                arrow(
                    app(cst("Measure"), type0()),
                    arrow(app(cst("SteinKernel"), type0()), real_ty()),
                ),
            ),
        ),
        ("Iff", arrow(prop(), arrow(prop(), prop()))),
        (
            "PMCEstimator",
            arrow(
                app(cst("Measure"), type0()),
                arrow(nat_ty(), arrow(nat_ty(), type0())),
            ),
        ),
        ("Tempering", type0()),
        (
            "EvolMCMCKernel",
            arrow(
                app(cst("Measure"), type0()),
                arrow(cst("Tempering"), app2(cst("Kernel"), type0(), type0())),
            ),
        ),
        (
            "TemperedTarget",
            arrow(
                app(cst("Measure"), type0()),
                arrow(cst("Tempering"), app(cst("Measure"), type0())),
            ),
        ),
        (
            "DetailedBalance",
            arrow(
                app2(cst("Kernel"), type0(), type0()),
                arrow(app(cst("Measure"), type0()), prop()),
            ),
        ),
        ("ProductSpace", arrow(list_ty(real_ty()), type0())),
        (
            "SwapKernel",
            arrow(list_ty(real_ty()), app2(cst("Kernel"), type0(), type0())),
        ),
        ("CoolingSchedule", type0()),
        ("LogarithmicSchedule", arrow(cst("CoolingSchedule"), prop())),
        (
            "SAChain",
            arrow(
                arrow(type0(), real_ty()),
                arrow(cst("CoolingSchedule"), type0()),
            ),
        ),
        ("ConvergesToGlobalOpt", arrow(type0(), prop())),
        ("NeuralNet", type0()),
        (
            "VAELBO",
            arrow(
                cst("NeuralNet"),
                arrow(cst("NeuralNet"), arrow(type0(), real_ty())),
            ),
        ),
        (
            "Reconstruction",
            arrow(
                cst("NeuralNet"),
                arrow(cst("NeuralNet"), arrow(type0(), real_ty())),
            ),
        ),
        ("KLDivQ", arrow(cst("NeuralNet"), arrow(type0(), real_ty()))),
        ("Real.sub", arrow(real_ty(), arrow(real_ty(), real_ty()))),
        (
            "DSMObjective",
            arrow(app(cst("Measure"), type0()), arrow(real_ty(), type0())),
        ),
        (
            "GaussianSmooth",
            arrow(
                app(cst("Measure"), type0()),
                arrow(real_ty(), app(cst("Measure"), type0())),
            ),
        ),
        ("OptimScore", arrow(type0(), type0())),
        (
            "ScoreFunction",
            arrow(app(cst("Measure"), type0()), type0()),
        ),
        ("VectorField", type0()),
        (
            "CondFlowMatchingField",
            arrow(
                cst("VectorField"),
                arrow(
                    app(cst("Measure"), type0()),
                    arrow(app(cst("Measure"), type0()), prop()),
                ),
            ),
        ),
        (
            "PushforwardODE",
            arrow(
                cst("VectorField"),
                arrow(
                    app(cst("Measure"), type0()),
                    arrow(real_ty(), app(cst("Measure"), type0())),
                ),
            ),
        ),
        ("GaussianProcess", arrow(type0(), type0())),
        ("Observations", type0()),
        ("IsGaussianProcess", arrow(type0(), prop())),
        (
            "GPPosterior",
            arrow(
                app(cst("GaussianProcess"), type0()),
                arrow(cst("Observations"), type0()),
            ),
        ),
        (
            "MarginalLikelihood",
            arrow(
                app(cst("GaussianProcess"), type0()),
                arrow(list_ty(type0()), app(cst("Measure"), type0())),
            ),
        ),
        (
            "MultivariateGaussian",
            arrow(type0(), arrow(type0(), app(cst("Measure"), type0()))),
        ),
        (
            "GPMean",
            arrow(
                app(cst("GaussianProcess"), type0()),
                arrow(list_ty(type0()), type0()),
            ),
        ),
        (
            "GPKernelMatrix",
            arrow(
                app(cst("GaussianProcess"), type0()),
                arrow(list_ty(type0()), type0()),
            ),
        ),
        (
            "BQPosterior",
            arrow(
                app(cst("GaussianProcess"), type0()),
                arrow(arrow(type0(), real_ty()), app(cst("Measure"), real_ty())),
            ),
        ),
        (
            "GaussianMeasureOver",
            arrow(type0(), app(cst("Measure"), real_ty())),
        ),
        ("FeynmanKacModel", arrow(type0(), type0())),
        (
            "SMCNormConst",
            arrow(
                app(cst("FeynmanKacModel"), type0()),
                arrow(nat_ty(), real_ty()),
            ),
        ),
        (
            "FeynmanKacNormConst",
            arrow(app(cst("FeynmanKacModel"), type0()), real_ty()),
        ),
        ("LatentModel", arrow(type0(), arrow(type0(), type0()))),
        (
            "PMMHKernel",
            arrow(
                app2(cst("LatentModel"), type0(), type0()),
                arrow(type0(), app2(cst("Kernel"), type0(), type0())),
            ),
        ),
        (
            "TargetsExactPosterior",
            arrow(app2(cst("Kernel"), type0(), type0()), prop()),
        ),
        ("AnnealingSchedule", type0()),
        (
            "AISEstimator",
            arrow(
                app(cst("Measure"), type0()),
                arrow(
                    app(cst("Measure"), type0()),
                    arrow(cst("AnnealingSchedule"), type0()),
                ),
            ),
        ),
        ("NormConst", arrow(app(cst("Measure"), type0()), real_ty())),
        (
            "ImplicitSMObjective",
            arrow(app(cst("Measure"), type0()), type0()),
        ),
        (
            "GaussianConvolution",
            arrow(
                app(cst("Measure"), type0()),
                arrow(real_ty(), app(cst("Measure"), type0())),
            ),
        ),
        ("LogConcaveMeasure", arrow(type0(), type0())),
        (
            "ULADist",
            arrow(
                app(cst("LogConcaveMeasure"), type0()),
                arrow(real_ty(), arrow(nat_ty(), app(cst("Measure"), type0()))),
            ),
        ),
        (
            "W2",
            arrow(
                app(cst("Measure"), type0()),
                arrow(app(cst("Measure"), type0()), real_ty()),
            ),
        ),
        (
            "LangevinBound",
            arrow(
                app(cst("LogConcaveMeasure"), type0()),
                arrow(real_ty(), arrow(nat_ty(), real_ty())),
            ),
        ),
        (
            "MHKernel",
            arrow(
                app(cst("Measure"), type0()),
                arrow(
                    app2(cst("Kernel"), type0(), type0()),
                    app2(cst("Kernel"), type0(), type0()),
                ),
            ),
        ),
        ("Prod", arrow(type0(), arrow(type0(), type0()))),
        (
            "GibbsKernel",
            arrow(
                app(cst("Measure"), app2(cst("Prod"), type0(), type0())),
                app2(
                    cst("Kernel"),
                    app2(cst("Prod"), type0(), type0()),
                    app2(cst("Prod"), type0(), type0()),
                ),
            ),
        ),
        ("ExpressiveDecoder", type0()),
        (
            "BetaVAE",
            arrow(cst("ExpressiveDecoder"), arrow(real_ty(), type0())),
        ),
        ("AvoidsCollapse", arrow(type0(), prop())),
        ("Real.lt", arrow(real_ty(), arrow(real_ty(), prop()))),
        ("NaturalParams", arrow(type0(), type0())),
        ("LogNormalizer", arrow(type0(), arrow(type0(), real_ty()))),
        ("Gradient", arrow(arrow(type0(), real_ty()), type0())),
        (
            "MeanSuffStat",
            arrow(type0(), arrow(app(cst("Measure"), type0()), type0())),
        ),
        (
            "ExpFamilyDist",
            arrow(type0(), arrow(type0(), app(cst("Measure"), type0()))),
        ),
        ("ParticleSystem", arrow(type0(), arrow(nat_ty(), type0()))),
        ("AncestralLineage", arrow(type0(), type0())),
        ("CoalescentProcess", arrow(type0(), type0())),
        ("ResamplingTimes", arrow(type0(), type0())),
        ("BandwidthSeq", type0()),
        (
            "KDEn",
            arrow(
                app(cst("SmoothMeasure"), type0()),
                arrow(cst("BandwidthSeq"), type0()),
            ),
        ),
        ("OptimalBandwidth", arrow(cst("BandwidthSeq"), prop())),
        ("L2Convergence", arrow(type0(), prop())),
        (
            "CAVIStep",
            arrow(
                app(cst("Measure"), type0()),
                arrow(
                    list_ty(app(cst("Measure"), type0())),
                    list_ty(app(cst("Measure"), type0())),
                ),
            ),
        ),
        (
            "UpdatedFactors",
            arrow(
                app(cst("Measure"), type0()),
                arrow(
                    list_ty(app(cst("Measure"), type0())),
                    list_ty(app(cst("Measure"), type0())),
                ),
            ),
        ),
        ("BayesianNeuralNet", type0()),
        ("Input", type0()),
        (
            "PBP",
            arrow(cst("BayesianNeuralNet"), arrow(cst("Input"), type0())),
        ),
        ("GaussianApproxActivations", arrow(type0(), type0())),
        (
            "PBPActivations",
            arrow(cst("BayesianNeuralNet"), arrow(cst("Input"), type0())),
        ),
        ("FactorGraph", type0()),
        ("GaussianApprox", type0()),
        (
            "EPFixedPoint",
            arrow(cst("GaussianApprox"), arrow(cst("FactorGraph"), prop())),
        ),
        (
            "CavityAgreement",
            arrow(cst("GaussianApprox"), arrow(cst("FactorGraph"), prop())),
        ),
        (
            "NestedMCEstimator",
            arrow(
                arrow(type0(), real_ty()),
                arrow(nat_ty(), arrow(nat_ty(), type0())),
            ),
        ),
        ("Bias", arrow(type0(), real_ty())),
        ("NestedMCBiasRate", arrow(nat_ty(), real_ty())),
        (
            "ABCSMC",
            arrow(
                app(cst("Measure"), type0()),
                arrow(
                    arrow(type0(), app(cst("Measure"), type0())),
                    arrow(real_ty(), app(cst("Measure"), type0())),
                ),
            ),
        ),
        (
            "TruePosterior",
            arrow(
                app(cst("Measure"), type0()),
                arrow(
                    arrow(type0(), app(cst("Measure"), type0())),
                    app(cst("Measure"), type0()),
                ),
            ),
        ),
        (
            "ApproxPosterior",
            arrow(
                app(cst("Measure"), type0()),
                arrow(real_ty(), arrow(app(cst("Measure"), type0()), prop())),
            ),
        ),
        ("Real.gt", arrow(real_ty(), arrow(real_ty(), prop()))),
        ("svi_convergence", svi_convergence_ty()),
        ("normalizing_flow_cov", normalizing_flow_cov_ty()),
        ("score_fn_unbiased", score_fn_unbiased_ty()),
        (
            "pathwise_gradient_unbiased",
            pathwise_gradient_unbiased_ty(),
        ),
        ("measure_transport_exists", measure_transport_exists_ty()),
        ("ot_kantorovich", ot_kantorovich_ty()),
        ("stein_identity", stein_identity_ty()),
        ("svgd_convergence", svgd_convergence_ty()),
        ("pmc_consistency", pmc_consistency_ty()),
        (
            "evol_mcmc_detailed_balance",
            evol_mcmc_detailed_balance_ty(),
        ),
        (
            "parallel_tempering_exchange",
            parallel_tempering_exchange_ty(),
        ),
        (
            "simulated_annealing_convergence",
            simulated_annealing_convergence_ty(),
        ),
        ("vae_elbo_decomp", vae_elbo_decomp_ty()),
        ("diffusion_score_matching", diffusion_score_matching_ty()),
        ("flow_matching_ode", flow_matching_ode_ty()),
        ("gp_posterior_is_gp", gp_posterior_is_gp_ty()),
        ("gp_marginal_gaussian", gp_marginal_gaussian_ty()),
        ("pn_integration", pn_integration_ty()),
        ("stein_disc_zero_iff", stein_disc_zero_iff_ty()),
        ("smc_feynman_kac", smc_feynman_kac_ty()),
        ("pmmh_correctness", pmmh_correctness_ty()),
        ("ais_unbiased", ais_unbiased_ty()),
        ("dsm_equals_sm", dsm_equals_sm_ty()),
        ("langevin_convergence", langevin_convergence_ty()),
        ("mh_detailed_balance", mh_detailed_balance_ty()),
        ("gibbs_invariant", gibbs_invariant_ty()),
        (
            "vae_posterior_collapse_risk",
            vae_posterior_collapse_risk_ty(),
        ),
        ("grad_log_normalizer", grad_log_normalizer_ty()),
        ("smc_genealogy", smc_genealogy_ty()),
        ("kde_consistency", kde_consistency_ty()),
        ("mean_field_cavi", mean_field_cavi_ty()),
        ("pbp_gaussian_propagation", pbp_gaussian_propagation_ty()),
        ("ep_fixed_point", ep_fixed_point_ty()),
        ("nested_mc_bias", nested_mc_bias_ty()),
        ("abc_smc_consistency", abc_smc_consistency_ty()),
    ];
    for (name, ty) in axioms {
        env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty: ty.clone(),
        })
        .ok();
    }
    Ok(())
}
