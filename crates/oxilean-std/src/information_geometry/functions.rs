//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{
    AlphaDivMid, AlphaDivergence, BayesianEstimation, BeliefPropagation, BregmanDivergence,
    ConstantCurvatureManifold, DualConnection, ExpectationPropagation, ExponentialFamily,
    ExponentialFamilyDistrib, FisherInformationMetric, GaussianProcess, GeodesicOfDistributions,
    JeffreysPrior, LegendreTransform, MirrorDescent, MomentParameter, NatGradExt, NatGradMid,
    NaturalParameter, QuantumInfoGeometry, ReferenceAnalysis, SchroedingerBridge,
    SlicedWasserstein, StatManiExt, StatManiMid, StatisticalManifold, WassersteinGeometry,
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
/// `StatisticalManifold`: smooth manifold of probability distributions parametrized by θ ∈ Θ ⊆ ℝ^n
/// Type: Nat → Type (dimension n → manifold)
pub fn statistical_manifold_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `FisherInformationMetric`: g_{ij}(θ) = E\[∂_i log p · ∂_j log p\]
/// Type: Nat → Type (dim n → n×n metric tensor field)
pub fn fisher_information_metric_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `RiemannianMetric`: general Riemannian metric on the probability simplex
/// Type: Nat → Type
pub fn riemannian_metric_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `GeodesicOfDistributions`: shortest path between two distributions on the manifold
/// Type: Type → Type → Type (start → end → geodesic path)
pub fn geodesic_of_distributions_ty() -> Expr {
    arrow(type0(), arrow(type0(), type0()))
}
/// Chentsov's theorem: the Fisher information metric is the unique (up to scale)
/// Riemannian metric invariant under sufficient statistics
/// Type: Prop
pub fn chentsov_theorem_ty() -> Expr {
    prop()
}
/// Geodesic distance formula: d(p,q) = 2 arccos(∫ √(p q) dμ) (Bhattacharyya arc length)
/// Type: ∀ (n : Nat), Prop
pub fn geodesic_distance_formula_ty() -> Expr {
    pi(BinderInfo::Default, "n", nat_ty(), prop())
}
/// Sectional curvature of the statistical manifold under Fisher metric
/// Type: ∀ (n : Nat), Real (returns curvature)
pub fn sectional_curvature_ty() -> Expr {
    pi(BinderInfo::Default, "n", nat_ty(), real_ty())
}
/// Christoffel symbols Γ^k_{ij} for the Fisher information metric
/// Type: Nat → Nat → Type
pub fn christoffel_symbols_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `ExponentialFamily`: p(x|θ) = exp(⟨θ, T(x)⟩ - A(θ)) h(x)
/// Type: Nat → Type (sufficient statistic dimension → family)
pub fn exponential_family_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `NaturalParameter`: θ ∈ Θ ⊆ ℝ^d (canonical/natural parameters)
/// Type: Nat → Type
pub fn natural_parameter_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `MomentParameter`: η = E_θ[T(x)] ∈ ℝ^d (mean/moment parameters)
/// Type: Nat → Type
pub fn moment_parameter_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `LegendreTransform`: A*(η) = sup_θ {⟨θ,η⟩ - A(θ)} (convex conjugate of log-partition)
/// Type: (List Real → Real) → List Real → Real
pub fn legendre_transform_ty() -> Expr {
    arrow(
        arrow(list_ty(real_ty()), real_ty()),
        arrow(list_ty(real_ty()), real_ty()),
    )
}
/// `LogPartitionFunction`: A(θ) = log ∫ exp(⟨θ, T(x)⟩) h(x) dμ(x)
/// Type: List Real → Real
pub fn log_partition_function_ty() -> Expr {
    arrow(list_ty(real_ty()), real_ty())
}
/// Natural-to-moment parameter conversion: η = ∇A(θ)
/// Type: ∀ (d : Nat), Prop
pub fn natural_to_moment_ty() -> Expr {
    pi(BinderInfo::Default, "d", nat_ty(), prop())
}
/// Bregman divergence from log-partition: D_A(η ‖ η') = A*(η) - A*(η') - ⟨∇A*(η'), η - η'⟩
/// Type: ∀ (d : Nat), Prop
pub fn bregman_divergence_ty() -> Expr {
    pi(BinderInfo::Default, "d", nat_ty(), prop())
}
/// Fisher information as Hessian of log-partition: I(θ) = ∇²A(θ)
/// Type: ∀ (d : Nat), Prop
pub fn fisher_as_hessian_ty() -> Expr {
    pi(BinderInfo::Default, "d", nat_ty(), prop())
}
/// KL divergence equals Bregman divergence for exponential families:
/// D_KL(p_θ ‖ p_θ') = D_A(η ‖ η')
/// Type: Prop
pub fn kl_equals_bregman_ty() -> Expr {
    prop()
}
/// `AlphaConnection`: Γ^(α)_{ij,k} = Γ^(0)_{ij,k} - (α/2) T_{ijk}
/// (mixture of e-connection and m-connection)
/// Type: Real → Nat → Type (α parameter → dimension → connection)
pub fn alpha_connection_ty() -> Expr {
    arrow(real_ty(), arrow(nat_ty(), type0()))
}
/// `AlphaDivergence`: D^(α)(P‖Q) = 4/(1-α²)(1 - ∫p^{(1+α)/2} q^{(1-α)/2} dμ)
/// Type: Real → List Real → List Real → Real (α, P-dist, Q-dist → divergence)
pub fn alpha_divergence_ty() -> Expr {
    arrow(
        real_ty(),
        arrow(list_ty(real_ty()), arrow(list_ty(real_ty()), real_ty())),
    )
}
/// `DualConnection`: (∇, ∇*) dual affine connections satisfying X⟨Y,Z⟩ = ⟨∇_X Y, Z⟩ + ⟨Y, ∇*_X Z⟩
/// Type: Nat → Type (dimension → dual connection pair)
pub fn dual_connection_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `ConstantCurvatureManifold`: statistical manifold with constant α-curvature
/// (α = ±1 gives exponential/mixture families)
/// Type: Real → Nat → Type (curvature α → dimension → manifold)
pub fn constant_curvature_manifold_ty() -> Expr {
    arrow(real_ty(), arrow(nat_ty(), type0()))
}
/// Duality theorem: (∇^(α))* = ∇^(-α)
/// Type: ∀ (α : Real) (n : Nat), Prop
pub fn alpha_duality_theorem_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "alpha",
        real_ty(),
        pi(BinderInfo::Default, "n", nat_ty(), prop()),
    )
}
/// Generalized Pythagorean theorem for α-divergences on flat manifolds
/// Type: ∀ (n : Nat), Prop
pub fn generalized_pythagoras_ty() -> Expr {
    pi(BinderInfo::Default, "n", nat_ty(), prop())
}
/// α-divergence reduction: α=1 gives KL, α=-1 gives reverse KL, α=0 gives Hellinger
/// Type: Prop
pub fn alpha_divergence_limits_ty() -> Expr {
    prop()
}
/// Curvature formula: constant curvature = -1/4 for e/m-families
/// Type: ∀ (α : Real), Real
pub fn curvature_formula_ty() -> Expr {
    pi(BinderInfo::Default, "alpha", real_ty(), real_ty())
}
/// `BayesianEstimation`: posterior p(θ|x) ∝ L(θ|x) · π(θ)
/// Type: (Real → Real) → (Real → Real) → Real → Real (likelihood, prior, x → posterior)
pub fn bayesian_estimation_ty() -> Expr {
    arrow(
        arrow(real_ty(), real_ty()),
        arrow(arrow(real_ty(), real_ty()), arrow(real_ty(), real_ty())),
    )
}
/// `JeffreysPrior`: π(θ) ∝ √det(I(θ)) — invariant under reparametrization
/// Type: (Real → Real) → Real → Real (log-density → θ → prior density)
pub fn jeffreys_prior_ty() -> Expr {
    arrow(arrow(real_ty(), real_ty()), arrow(real_ty(), real_ty()))
}
/// `ReferenceAnalysis`: Bernardo's reference prior maximizing expected KL divergence
/// Type: (Real → Real) → Real → Real
pub fn reference_analysis_ty() -> Expr {
    arrow(arrow(real_ty(), real_ty()), arrow(real_ty(), real_ty()))
}
/// `ExpectationPropagation`: EP approximation — project tilted distribution onto exponential family
/// Type: Nat → Type (number of factors → EP state)
pub fn expectation_propagation_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// Jeffreys prior invariance: π̃(φ) = π(θ)|dθ/dφ| for reparametrization φ = g(θ)
/// Type: Prop
pub fn jeffreys_invariance_ty() -> Expr {
    prop()
}
/// Bernstein-von Mises theorem: posterior concentrates at MLE as n → ∞
/// Type: ∀ (n : Nat), Prop
pub fn bernstein_von_mises_ty() -> Expr {
    pi(BinderInfo::Default, "n", nat_ty(), prop())
}
/// EP fixed point: at convergence, q(θ) is the closest exponential family member to p(θ|x)
/// Type: Prop
pub fn ep_fixed_point_ty() -> Expr {
    prop()
}
/// Laplace approximation: posterior ≈ N(θ_MAP, I(θ_MAP)^{-1}/n)
/// Type: ∀ (n : Nat), Prop
pub fn laplace_approximation_ty() -> Expr {
    pi(BinderInfo::Default, "n", nat_ty(), prop())
}
/// `FisherRaoMetric`: Riemannian metric on the probability simplex induced by
/// the Fisher information: ds² = Σ_{ij} g_{ij}(θ) dθ^i dθ^j
/// Type: Nat → Type (dimension → metric)
pub fn fisher_rao_metric_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `EConnection`: the (-1)-connection (exponential connection ∇^{(-1)}) on a
/// statistical manifold; flat in exponential coordinates
/// Type: Nat → Type
pub fn e_connection_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `MConnection`: the (+1)-connection (mixture connection ∇^{(+1)}) on a
/// statistical manifold; flat in mixture coordinates
/// Type: Nat → Type
pub fn m_connection_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `EProjection`: projection of a distribution onto an e-flat (exponential family) submanifold
/// minimizing KL divergence: π_e(p) = argmin_{q ∈ E} D_KL(q ‖ p)
/// Type: Nat → Type → Type (dim → family → projected dist)
pub fn e_projection_ty() -> Expr {
    arrow(nat_ty(), arrow(type0(), type0()))
}
/// `MProjection`: projection of a distribution onto an m-flat (mixture family) submanifold
/// minimizing KL divergence: π_m(p) = argmin_{q ∈ M} D_KL(p ‖ q)
/// Type: Nat → Type → Type (dim → family → projected dist)
pub fn m_projection_ty() -> Expr {
    arrow(nat_ty(), arrow(type0(), type0()))
}
/// Pythagorean theorem in information geometry:
/// for e-geodesic p,r with m-projection q onto e-flat family:
/// D_KL(p ‖ r) = D_KL(p ‖ q) + D_KL(q ‖ r)
/// Type: ∀ (n : Nat), Prop
pub fn pythagorean_theorem_ig_ty() -> Expr {
    pi(BinderInfo::Default, "n", nat_ty(), prop())
}
/// e-geodesic closure: exponential families are e-flat (e-geodesically complete)
/// Type: ∀ (d : Nat), Prop
pub fn e_flat_exponential_family_ty() -> Expr {
    pi(BinderInfo::Default, "d", nat_ty(), prop())
}
/// m-geodesic closure: mixture families are m-flat (m-geodesically complete)
/// Type: ∀ (d : Nat), Prop
pub fn m_flat_mixture_family_ty() -> Expr {
    pi(BinderInfo::Default, "d", nat_ty(), prop())
}
/// Legendre duality: θ ↦ η is a bijection, and A**(θ) = A(θ) (double Legendre)
/// Type: ∀ (d : Nat), Prop
pub fn legendre_duality_ty() -> Expr {
    pi(BinderInfo::Default, "d", nat_ty(), prop())
}
/// `FDivergence`: D_f(P ‖ Q) = ∫ f(dP/dQ) dQ for a convex f with f(1)=0
/// Type: (Real → Real) → List Real → List Real → Real (generator f, P, Q → divergence)
pub fn f_divergence_ty() -> Expr {
    arrow(
        arrow(real_ty(), real_ty()),
        arrow(list_ty(real_ty()), arrow(list_ty(real_ty()), real_ty())),
    )
}
/// `BregmanDivergenceGen`: generalized Bregman divergence D_φ(x ‖ y) = φ(x) - φ(y) - ⟨∇φ(y), x-y⟩
/// for a strictly convex differentiable φ
/// Type: (List Real → Real) → List Real → List Real → Real
pub fn bregman_divergence_gen_ty() -> Expr {
    arrow(
        arrow(list_ty(real_ty()), real_ty()),
        arrow(list_ty(real_ty()), arrow(list_ty(real_ty()), real_ty())),
    )
}
/// `WassersteinMetric`: optimal transport distance W_p(μ,ν) = (inf_γ ∫|x-y|^p dγ)^{1/p}
/// Type: Real → Nat → Type (p-parameter → dim → metric)
pub fn wasserstein_metric_ty() -> Expr {
    arrow(real_ty(), arrow(nat_ty(), type0()))
}
/// Every f-divergence is a Bregman divergence on exponential families
/// Type: Prop
pub fn f_div_is_bregman_on_exp_ty() -> Expr {
    prop()
}
/// Chentsov's uniqueness theorem for f-divergences:
/// Up to scaling, KL is the unique f-divergence invariant under sufficient statistics
/// Type: Prop
pub fn chentsov_uniqueness_f_div_ty() -> Expr {
    prop()
}
/// Wasserstein vs Fisher-Rao: they induce different geodesics;
/// Fisher-Rao is intrinsic, Wasserstein is extrinsic/optimal-transport
/// Type: ∀ (n : Nat), Prop
pub fn wasserstein_vs_fisher_rao_ty() -> Expr {
    pi(BinderInfo::Default, "n", nat_ty(), prop())
}
/// Pinsker's inequality: D_KL(P ‖ Q) ≥ (1/2) ‖P - Q‖²_TV
/// Type: Prop
pub fn pinsker_inequality_ty() -> Expr {
    prop()
}
/// `NaturalGradientDescent`: update rule θ ← θ - ε · G(θ)^{-1} ∇L(θ)
/// where G(θ) is the Fisher information matrix
/// Type: Nat → Type (dim → optimizer state)
pub fn natural_gradient_descent_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `MirrorDescent`: generalized gradient descent using Bregman divergence:
/// θ_{t+1} = argmin_{θ} {⟨∇L(θ_t), θ⟩ + (1/ε) D_φ(θ ‖ θ_t)}
/// Type: Nat → Type
pub fn mirror_descent_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `EMAlgorithm`: Expectation-Maximization as alternating m/e-projections:
/// E-step: m-project posterior onto simplex; M-step: e-project onto exponential family
/// Type: Nat → Nat → Type (latent dim → obs dim → EM state)
pub fn em_algorithm_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// Natural gradient descent converges to Fisher-efficient estimator:
/// θ_t → θ_MLE at rate O(1/t) in Fisher metric
/// Type: ∀ (d : Nat), Prop
pub fn natural_gradient_convergence_ty() -> Expr {
    pi(BinderInfo::Default, "d", nat_ty(), prop())
}
/// Mirror descent equals natural gradient for exponential family loss:
/// Bregman mirror descent with φ=A* is equivalent to natural gradient on exp family
/// Type: Prop
pub fn mirror_descent_eq_natural_gradient_ty() -> Expr {
    prop()
}
/// EM monotone convergence: log-likelihood L(θ^{(t+1)}) ≥ L(θ^{(t)})
/// Type: ∀ (n : Nat), Prop
pub fn em_monotone_convergence_ty() -> Expr {
    pi(BinderInfo::Default, "n", nat_ty(), prop())
}
/// EM as alternating projection: E-step is m-projection, M-step is e-projection
/// Type: Prop
pub fn em_as_alternating_projection_ty() -> Expr {
    prop()
}
/// `BeliefPropagation`: sum-product message passing on a factor graph
/// corresponds to iterated e-projections onto local exponential families
/// Type: Nat → Nat → Type (nodes → factors → BP state)
pub fn belief_propagation_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `TreeReweightedBP`: TRW-BP minimizes a variational Bethe free energy
/// Type: Nat → Type
pub fn tree_reweighted_bp_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// Belief propagation fixed point: BP fixed points are stationary points of Bethe free energy
/// Type: ∀ (n : Nat), Prop
pub fn bp_fixed_point_bethe_ty() -> Expr {
    pi(BinderInfo::Default, "n", nat_ty(), prop())
}
/// On a tree, BP converges to exact marginals (equals e-projection)
/// Type: ∀ (n : Nat), Prop
pub fn bp_exact_on_tree_ty() -> Expr {
    pi(BinderInfo::Default, "n", nat_ty(), prop())
}
/// `SanovTheorem`: rate function for empirical distribution is the KL divergence
/// P(L_n ∈ E) ≈ exp(-n · inf_{q ∈ E} D_KL(q ‖ p))
/// Type: Nat → Type (sample size → large-deviation event)
pub fn sanov_theorem_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `RateFunction`: I(q) = D_KL(q ‖ p₀) for the Sanov rate
/// Type: List Real → Real
pub fn rate_function_ty() -> Expr {
    arrow(list_ty(real_ty()), real_ty())
}
/// Sanov's theorem: D_KL is the unique rate function for empirical distributions
/// Type: ∀ (n : Nat), Prop
pub fn sanov_kl_rate_function_ty() -> Expr {
    pi(BinderInfo::Default, "n", nat_ty(), prop())
}
/// Contraction principle: rate function of a smooth map φ is I ∘ φ^{-1}
/// Type: ∀ (n : Nat), Prop
pub fn contraction_principle_ty() -> Expr {
    pi(BinderInfo::Default, "n", nat_ty(), prop())
}
/// `QuantumStatisticalManifold`: manifold of density matrices ρ(θ) on a Hilbert space H
/// Type: Nat → Nat → Type (dim-H → param-dim → manifold)
pub fn quantum_statistical_manifold_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `SLDMetric`: Symmetric Logarithmic Derivative (SLD) Fisher metric on quantum states;
/// the quantum analogue of Fisher-Rao: g_{ij}^{SLD} = (1/2) Tr\[ρ {L_i, L_j}\]
/// Type: Nat → Nat → Type (Hilbert-dim → param-dim → metric)
pub fn sld_metric_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `RLDMetric`: Right Logarithmic Derivative metric on quantum states
/// Type: Nat → Nat → Type
pub fn rld_metric_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `QuantumRelativeEntropy`: S(ρ ‖ σ) = Tr\[ρ (log ρ - log σ)\] (von Neumann relative entropy)
/// Type: Nat → Type (dim → relative-entropy operator)
pub fn quantum_relative_entropy_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// Quantum Cramér-Rao bound: Var(θ̂) ≥ 1 / (n · g^{SLD}(θ))
/// Type: ∀ (d : Nat), Prop
pub fn quantum_cramer_rao_ty() -> Expr {
    pi(BinderInfo::Default, "d", nat_ty(), prop())
}
/// SLD metric contracts under quantum channels (monotonicity under CPTP maps)
/// Type: Prop
pub fn sld_monotonicity_ty() -> Expr {
    prop()
}
/// Uhlmann's theorem: geometric phase = arc cos of fidelity F(ρ,σ) = Tr\[√(√ρ σ √ρ)\]
/// Type: ∀ (n : Nat), Prop
pub fn uhlmann_theorem_ty() -> Expr {
    pi(BinderInfo::Default, "n", nat_ty(), prop())
}
/// Quantum Stein's lemma: optimal exponent for quantum hypothesis testing is D_KL(ρ ‖ σ)
/// Type: Prop
pub fn quantum_stein_lemma_ty() -> Expr {
    prop()
}
/// `ItoGirsanovIG`: Girsanov's theorem viewed as a change of measure in IG:
/// the Radon-Nikodym derivative exp(∫ h dW - (1/2) ∫ h² dt) is a path-space exponential family
/// Type: Nat → Type (dim → process)
pub fn ito_girsanov_ig_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `FokkerPlanckIG`: Fokker-Planck equation as a gradient flow on the manifold of densities
/// under the Fisher-Rao metric
/// Type: Nat → Type
pub fn fokker_planck_ig_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// Girsanov change-of-measure as e-geodesic in path space:
/// p^h(x) = exp(∫ h dx - A(h)) p^0(x) is an e-family parametrized by h
/// Type: ∀ (d : Nat), Prop
pub fn girsanov_e_geodesic_ty() -> Expr {
    pi(BinderInfo::Default, "d", nat_ty(), prop())
}
/// Otto calculus: Fokker-Planck is gradient flow of KL divergence in Wasserstein geometry
/// Type: ∀ (d : Nat), Prop
pub fn otto_calculus_gradient_flow_ty() -> Expr {
    pi(BinderInfo::Default, "d", nat_ty(), prop())
}
/// Register all information geometry axioms and theorems in the kernel environment.
pub fn build_env(env: &mut Environment) -> Result<(), String> {
    let axioms: &[(&str, Expr)] = &[
        ("StatisticalManifold", statistical_manifold_ty()),
        ("FisherInformationMetric", fisher_information_metric_ty()),
        ("RiemannianMetric", riemannian_metric_ty()),
        ("GeodesicOfDistributions", geodesic_of_distributions_ty()),
        ("chentsov_theorem", chentsov_theorem_ty()),
        ("geodesic_distance_formula", geodesic_distance_formula_ty()),
        ("sectional_curvature", sectional_curvature_ty()),
        ("christoffel_symbols", christoffel_symbols_ty()),
        ("ExponentialFamily", exponential_family_ty()),
        ("NaturalParameter", natural_parameter_ty()),
        ("MomentParameter", moment_parameter_ty()),
        ("LegendreTransform", legendre_transform_ty()),
        ("LogPartitionFunction", log_partition_function_ty()),
        ("natural_to_moment", natural_to_moment_ty()),
        ("bregman_divergence", bregman_divergence_ty()),
        ("fisher_as_hessian", fisher_as_hessian_ty()),
        ("kl_equals_bregman", kl_equals_bregman_ty()),
        ("AlphaConnection", alpha_connection_ty()),
        ("AlphaDivergence", alpha_divergence_ty()),
        ("DualConnection", dual_connection_ty()),
        (
            "ConstantCurvatureManifold",
            constant_curvature_manifold_ty(),
        ),
        ("alpha_duality_theorem", alpha_duality_theorem_ty()),
        ("generalized_pythagoras", generalized_pythagoras_ty()),
        ("alpha_divergence_limits", alpha_divergence_limits_ty()),
        ("curvature_formula", curvature_formula_ty()),
        ("BayesianEstimation", bayesian_estimation_ty()),
        ("JeffreysPrior", jeffreys_prior_ty()),
        ("ReferenceAnalysis", reference_analysis_ty()),
        ("ExpectationPropagation", expectation_propagation_ty()),
        ("jeffreys_invariance", jeffreys_invariance_ty()),
        ("bernstein_von_mises", bernstein_von_mises_ty()),
        ("ep_fixed_point", ep_fixed_point_ty()),
        ("laplace_approximation", laplace_approximation_ty()),
        ("FisherRaoMetric", fisher_rao_metric_ty()),
        ("EConnection", e_connection_ty()),
        ("MConnection", m_connection_ty()),
        ("EProjection", e_projection_ty()),
        ("MProjection", m_projection_ty()),
        ("pythagorean_theorem_ig", pythagorean_theorem_ig_ty()),
        ("e_flat_exponential_family", e_flat_exponential_family_ty()),
        ("m_flat_mixture_family", m_flat_mixture_family_ty()),
        ("legendre_duality", legendre_duality_ty()),
        ("FDivergence", f_divergence_ty()),
        ("BregmanDivergenceGen", bregman_divergence_gen_ty()),
        ("WassersteinMetric", wasserstein_metric_ty()),
        ("f_div_is_bregman_on_exp", f_div_is_bregman_on_exp_ty()),
        ("chentsov_uniqueness_f_div", chentsov_uniqueness_f_div_ty()),
        ("wasserstein_vs_fisher_rao", wasserstein_vs_fisher_rao_ty()),
        ("pinsker_inequality", pinsker_inequality_ty()),
        ("NaturalGradientDescent", natural_gradient_descent_ty()),
        ("MirrorDescent", mirror_descent_ty()),
        ("EMAlgorithm", em_algorithm_ty()),
        (
            "natural_gradient_convergence",
            natural_gradient_convergence_ty(),
        ),
        (
            "mirror_descent_eq_natural_gradient",
            mirror_descent_eq_natural_gradient_ty(),
        ),
        ("em_monotone_convergence", em_monotone_convergence_ty()),
        (
            "em_as_alternating_projection",
            em_as_alternating_projection_ty(),
        ),
        ("BeliefPropagation", belief_propagation_ty()),
        ("TreeReweightedBP", tree_reweighted_bp_ty()),
        ("bp_fixed_point_bethe", bp_fixed_point_bethe_ty()),
        ("bp_exact_on_tree", bp_exact_on_tree_ty()),
        ("SanovTheorem", sanov_theorem_ty()),
        ("RateFunction", rate_function_ty()),
        ("sanov_kl_rate_function", sanov_kl_rate_function_ty()),
        ("contraction_principle", contraction_principle_ty()),
        (
            "QuantumStatisticalManifold",
            quantum_statistical_manifold_ty(),
        ),
        ("SLDMetric", sld_metric_ty()),
        ("RLDMetric", rld_metric_ty()),
        ("QuantumRelativeEntropy", quantum_relative_entropy_ty()),
        ("quantum_cramer_rao", quantum_cramer_rao_ty()),
        ("sld_monotonicity", sld_monotonicity_ty()),
        ("uhlmann_theorem", uhlmann_theorem_ty()),
        ("quantum_stein_lemma", quantum_stein_lemma_ty()),
        ("ItoGirsanovIG", ito_girsanov_ig_ty()),
        ("FokkerPlanckIG", fokker_planck_ig_ty()),
        ("girsanov_e_geodesic", girsanov_e_geodesic_ty()),
        (
            "otto_calculus_gradient_flow",
            otto_calculus_gradient_flow_ty(),
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
    Ok(())
}
/// Dot product of two equal-length slices.
pub fn dot_product(a: &[f64], b: &[f64]) -> f64 {
    a.iter().zip(b.iter()).map(|(ai, bi)| ai * bi).sum()
}
/// Matrix-vector product: returns A * v where A is d×d (row-major) and v is d.
pub fn mat_vec(a: &[Vec<f64>], v: &[f64]) -> Vec<f64> {
    a.iter().map(|row| dot_product(row, v)).collect()
}
/// Solve a d×d linear system Ax = b using Gaussian elimination with partial pivoting.
pub fn solve_linear_system(a: &[Vec<f64>], b: &[f64]) -> Vec<f64> {
    let d = b.len();
    let mut mat: Vec<Vec<f64>> = a.to_vec();
    let mut rhs: Vec<f64> = b.to_vec();
    for col in 0..d {
        let pivot = (col..d)
            .max_by(|&i, &j| {
                mat[i][col]
                    .abs()
                    .partial_cmp(&mat[j][col].abs())
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .unwrap_or(col);
        mat.swap(col, pivot);
        rhs.swap(col, pivot);
        let diag = mat[col][col];
        if diag.abs() < 1e-14 {
            continue;
        }
        for row in (col + 1)..d {
            let factor = mat[row][col] / diag;
            for k in col..d {
                let val = mat[col][k];
                mat[row][k] -= factor * val;
            }
            rhs[row] -= factor * rhs[col];
        }
    }
    let mut x = vec![0.0f64; d];
    for i in (0..d).rev() {
        let mut s = rhs[i];
        for j in (i + 1)..d {
            s -= mat[i][j] * x[j];
        }
        x[i] = if mat[i][i].abs() < 1e-14 {
            0.0
        } else {
            s / mat[i][i]
        };
    }
    x
}
#[cfg(test)]
mod ig_ext_tests {
    use super::*;
    #[test]
    fn test_statistical_manifold() {
        let exp = StatManiMid::exponential_family("Normal", 2);
        assert!(exp.is_dually_flat());
        assert!(!exp.alpha_divergence_description().is_empty());
    }
    #[test]
    fn test_natural_gradient() {
        let ng = NatGradMid::new(10, 0.01);
        assert!(!ng.update_rule().is_empty());
        assert!(!ng.invariance_property().is_empty());
    }
    #[test]
    fn test_alpha_divergence() {
        let kl = AlphaDivMid::kl_divergence("p", "q");
        assert!(kl.is_kl());
    }
    #[test]
    fn test_bregman_divergence() {
        let bd = BregmanDivergence::squared_euclidean();
        assert!(!bd.definition().is_empty());
        assert!(!bd.three_point_property().is_empty());
    }
    #[test]
    fn test_wasserstein() {
        let w = WassersteinGeometry::new(2, "R^d");
        assert!(!w.w2_distance_description().is_empty());
        assert!(!w.benamou_brenier_description().is_empty());
    }
}
#[cfg(test)]
mod gp_expfam_tests {
    use super::*;
    #[test]
    fn test_gaussian_process() {
        let gp = GaussianProcess::rbf(1.0);
        assert!(gp.is_stationary);
        assert!(!gp.posterior_description().is_empty());
    }
    #[test]
    fn test_exponential_family() {
        let gauss = ExponentialFamilyDistrib::gaussian(2);
        assert!(gauss.mle_equals_moment_matching());
        assert!(!gauss.natural_to_moment_params().is_empty());
    }
}
#[cfg(test)]
mod tests_info_geom_ext {
    use super::*;
    #[test]
    fn test_natural_gradient() {
        let ng = NatGradExt::new(10);
        let update = ng.update_rule(0.01);
        assert!(update.contains("Natural gradient"));
        let fr = ng.fisher_rao_distance();
        assert!(fr.contains("Fisher-Rao"));
        let amari = ng.amari_dual_connection();
        assert!(amari.contains("α-connection"));
        let inv = ng.invariance_property();
        assert!(inv.contains("Fisher-Rao"));
    }
    #[test]
    fn test_statistical_manifold() {
        let gauss = StatManiExt::gaussian_family();
        assert!(gauss.is_dually_flat);
        assert_eq!(gauss.dimension, 2);
        let pyth = gauss.pythagorean_theorem();
        assert!(pyth.contains("Pythagoras"));
        let bregman = gauss.bregman_divergence_connection();
        assert!(bregman.contains("Bregman"));
    }
    #[test]
    fn test_sliced_wasserstein() {
        let sw = SlicedWasserstein::new(10, 100);
        let desc = sw.complexity_description();
        assert!(desc.contains("Sliced"));
        let bonneel = sw.bonneel_et_al_description();
        assert!(bonneel.contains("sliced Wasserstein"));
    }
    #[test]
    fn test_schroedinger_bridge() {
        let sb = SchroedingerBridge::new("P", "Q", "BM", 0.01);
        let sink = sb.sinkhorn_algorithm();
        assert!(sink.contains("Sinkhorn"));
        let ipfp = sb.ipfp_iteration();
        assert!(ipfp.contains("IPFP"));
        let diff = sb.connection_to_diffusion_models();
        assert!(diff.contains("diffusion"));
    }
    #[test]
    fn test_quantum_info_geom() {
        let bures = QuantumInfoGeometry::bures_metric(4);
        assert!(bures.is_monotone_metric);
        let petz = bures.petz_classification();
        assert!(petz.contains("Petz"));
        let qcr = bures.quantum_cramer_rao();
        assert!(qcr.contains("Cramér-Rao"));
        let holevo = bures.holevo_bound();
        assert!(holevo.contains("Holevo"));
        let bures_dist = bures.bures_distance(1.0);
        assert!((bures_dist - 0.0).abs() < 1e-10);
        let bures_dist2 = bures.bures_distance(0.0);
        assert!((bures_dist2 - 2.0_f64.sqrt()).abs() < 1e-10);
    }
}
