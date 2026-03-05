//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{
    BisectionSolver, CrankNicolsonSolver, GradientDescentOptimizer, Interval, MonteCarloIntegrator,
    NewtonRaphsonSolver, PowerIterationSolver, RungeKutta4Solver, SparseMatrix, TikhonovSolver,
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
pub fn bool_ty() -> Expr {
    cst("Bool")
}
pub fn fn_ty(dom: Expr, cod: Expr) -> Expr {
    arrow(dom, cod)
}
/// `ConvergentSequence : (Nat → Real) → Real → Prop`
/// f converges to L.
pub fn convergent_sequence_ty() -> Expr {
    arrow(fn_ty(nat_ty(), real_ty()), arrow(real_ty(), prop()))
}
/// `CauchySequence : (Nat → Real) → Prop`
/// Cauchy completeness: every Cauchy sequence converges.
pub fn cauchy_sequence_ty() -> Expr {
    arrow(fn_ty(nat_ty(), real_ty()), prop())
}
/// `LipschitzContinuous : (Real → Real) → Real → Prop`
/// f is Lipschitz continuous with constant K.
pub fn lipschitz_ty() -> Expr {
    arrow(fn_ty(real_ty(), real_ty()), arrow(real_ty(), prop()))
}
/// `ContractionMapping : (Real → Real) → Real → Prop`
/// f is a contraction mapping with factor k < 1.
pub fn contraction_ty() -> Expr {
    arrow(fn_ty(real_ty(), real_ty()), arrow(real_ty(), prop()))
}
/// `FixedPoint : (Real → Real) → Real → Prop`
/// x* is a fixed point: f(x*) = x*.
pub fn fixed_point_ty() -> Expr {
    arrow(fn_ty(real_ty(), real_ty()), arrow(real_ty(), prop()))
}
/// `NewtonConvergence : (Real → Real) → (Real → Real) → Real → Prop`
/// Newton's method exhibits quadratic convergence near a simple root.
pub fn newton_convergence_ty() -> Expr {
    arrow(
        fn_ty(real_ty(), real_ty()),
        arrow(fn_ty(real_ty(), real_ty()), arrow(real_ty(), prop())),
    )
}
/// `RungeKuttaError : (Real → Real → Real) → Real → Prop`
/// RK4 global truncation error is O(h^4).
pub fn runge_kutta_error_ty() -> Expr {
    arrow(
        fn_ty(real_ty(), fn_ty(real_ty(), real_ty())),
        arrow(real_ty(), prop()),
    )
}
/// `BisectionConvergence : (Real → Real) → Real → Real → Prop`
/// Bisection halves the interval at each step.
pub fn bisection_convergence_ty() -> Expr {
    arrow(
        fn_ty(real_ty(), real_ty()),
        arrow(real_ty(), arrow(real_ty(), prop())),
    )
}
/// `IEEE754Rounding : (Real → Real) → Prop`
/// A function models IEEE 754 rounding to nearest.
pub fn ieee754_rounding_ty() -> Expr {
    arrow(fn_ty(real_ty(), real_ty()), prop())
}
/// `FloatCancellation : (Real → Real → Real) → Real → Real → Prop`
/// Catastrophic cancellation: subtracting two nearly-equal floats.
pub fn float_cancellation_ty() -> Expr {
    arrow(
        fn_ty(real_ty(), fn_ty(real_ty(), real_ty())),
        arrow(real_ty(), arrow(real_ty(), prop())),
    )
}
/// `ConditionNumber : (Real → Real) → Real → Real → Prop`
/// The condition number of f at x is κ.
pub fn condition_number_ty() -> Expr {
    arrow(
        fn_ty(real_ty(), real_ty()),
        arrow(real_ty(), arrow(real_ty(), prop())),
    )
}
/// `NumericalStability : (Real → Real) → Prop`
/// An algorithm is numerically stable.
pub fn numerical_stability_ty() -> Expr {
    arrow(fn_ty(real_ty(), real_ty()), prop())
}
/// `BackwardStability : (Real → Real) → Real → Prop`
/// An algorithm has backward stability constant ε.
pub fn backward_stability_ty() -> Expr {
    arrow(fn_ty(real_ty(), real_ty()), arrow(real_ty(), prop()))
}
/// `BisectionRate : (Real → Real) → Prop`
/// The bisection method converges linearly with rate 1/2.
pub fn bisection_rate_ty() -> Expr {
    arrow(fn_ty(real_ty(), real_ty()), prop())
}
/// `SecantConvergence : (Real → Real) → Real → Prop`
/// The secant method converges with superlinear order ~1.618.
pub fn secant_convergence_ty() -> Expr {
    arrow(fn_ty(real_ty(), real_ty()), arrow(real_ty(), prop()))
}
/// `LagrangeInterpolantError : (Real → Real) → Nat → Real → Prop`
/// Error bound for Lagrange interpolation with n+1 nodes at x.
pub fn lagrange_interpolant_error_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "f",
        fn_ty(real_ty(), real_ty()),
        pi(BinderInfo::Default, "n", nat_ty(), arrow(real_ty(), prop())),
    )
}
/// `NewtonDividedDifference : (Real → Real) → Nat → Real → Real → Prop`
/// Newton divided-difference interpolation formula.
pub fn newton_divided_difference_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "f",
        fn_ty(real_ty(), real_ty()),
        pi(
            BinderInfo::Default,
            "n",
            nat_ty(),
            arrow(real_ty(), arrow(real_ty(), prop())),
        ),
    )
}
/// `CubicSplineError : (Real → Real) → Real → Prop`
/// Cubic spline interpolation error bound O(h^4).
pub fn cubic_spline_error_ty() -> Expr {
    arrow(fn_ty(real_ty(), real_ty()), arrow(real_ty(), prop()))
}
/// `GaussianQuadratureExact : Nat → Nat → Prop`
/// Gaussian quadrature with n points is exact for polynomials of degree ≤ 2n-1.
pub fn gaussian_quadrature_exact_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), prop()))
}
/// `AdaptiveQuadratureConvergence : (Real → Real) → Real → Prop`
/// Adaptive quadrature converges to given tolerance.
pub fn adaptive_quadrature_convergence_ty() -> Expr {
    arrow(fn_ty(real_ty(), real_ty()), arrow(real_ty(), prop()))
}
/// `AdamsBashforthError : Nat → Real → Prop`
/// Adams-Bashforth p-step method has global error O(h^p).
pub fn adams_bashforth_error_ty() -> Expr {
    arrow(nat_ty(), arrow(real_ty(), prop()))
}
/// `StiffODE : (Real → Real → Real) → Real → Prop`
/// An ODE is stiff with stiffness ratio r.
pub fn stiff_ode_ty() -> Expr {
    arrow(
        fn_ty(real_ty(), fn_ty(real_ty(), real_ty())),
        arrow(real_ty(), prop()),
    )
}
/// `ImplicitMethodStability : (Real → Real → Real) → Prop`
/// An implicit method is A-stable for the given ODE.
pub fn implicit_method_stability_ty() -> Expr {
    arrow(fn_ty(real_ty(), fn_ty(real_ty(), real_ty())), prop())
}
/// `FiniteDifferenceConsistency : (Real → Real → Real) → Real → Prop`
/// A finite difference scheme is consistent with truncation error O(h^p).
pub fn finite_difference_consistency_ty() -> Expr {
    arrow(
        fn_ty(real_ty(), fn_ty(real_ty(), real_ty())),
        arrow(real_ty(), prop()),
    )
}
/// `GalerkinOrthogonality : (Real → Real) → Prop`
/// Galerkin method satisfies the orthogonality condition.
pub fn galerkin_orthogonality_ty() -> Expr {
    arrow(fn_ty(real_ty(), real_ty()), prop())
}
/// `FEMConvergence : (Real → Real) → Real → Real → Prop`
/// Finite element method converges with rate O(h^k) in H^1 norm.
pub fn fem_convergence_ty() -> Expr {
    arrow(
        fn_ty(real_ty(), real_ty()),
        arrow(real_ty(), arrow(real_ty(), prop())),
    )
}
/// `ConjugateGradientConvergence : Nat → Real → Prop`
/// CG method converges in at most n steps; rate depends on condition number.
pub fn conjugate_gradient_convergence_ty() -> Expr {
    arrow(nat_ty(), arrow(real_ty(), prop()))
}
/// `KrylovDimension : Nat → Nat → Prop`
/// Krylov subspace of dimension k for an n×n system.
pub fn krylov_dimension_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), prop()))
}
/// `PreconditionedSystem : (Real → Real) → Real → Prop`
/// A preconditioned system has condition number κ.
pub fn preconditioned_system_ty() -> Expr {
    arrow(fn_ty(real_ty(), real_ty()), arrow(real_ty(), prop()))
}
/// `SVDDecomposition : Nat → Nat → Prop`
/// An m×n matrix has a singular value decomposition.
pub fn svd_decomposition_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), prop()))
}
/// `QRAlgorithmConvergence : Nat → Real → Prop`
/// The QR algorithm converges to eigenvalues for an n×n matrix.
pub fn qr_algorithm_convergence_ty() -> Expr {
    arrow(nat_ty(), arrow(real_ty(), prop()))
}
/// `PowerIterationConvergence : Nat → Real → Prop`
/// Power iteration converges with ratio |λ₂/λ₁|.
pub fn power_iteration_convergence_ty() -> Expr {
    arrow(nat_ty(), arrow(real_ty(), prop()))
}
/// `RichardsonExtrapolation : (Real → Real) → Real → Nat → Prop`
/// Richardson extrapolation improves convergence order by p.
pub fn richardson_extrapolation_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "f",
        fn_ty(real_ty(), real_ty()),
        arrow(real_ty(), arrow(nat_ty(), prop())),
    )
}
/// `MultigridConvergence : Nat → Real → Prop`
/// Multigrid method converges with mesh-independent rate.
pub fn multigrid_convergence_ty() -> Expr {
    arrow(nat_ty(), arrow(real_ty(), prop()))
}
/// `FastMultipoleComplexity : Nat → Real → Prop`
/// FMM computes N-body interactions in O(N) with precision ε.
pub fn fast_multipole_complexity_ty() -> Expr {
    arrow(nat_ty(), arrow(real_ty(), prop()))
}
/// `SparseMatrixDensity : Nat → Real → Prop`
/// An n×n matrix has sparsity density ρ (fraction of nonzeros).
pub fn sparse_matrix_density_ty() -> Expr {
    arrow(nat_ty(), arrow(real_ty(), prop()))
}
/// `LaxEquivalence : (Real → Real → Real) → Prop`
/// Lax equivalence: consistency + stability ↔ convergence.
pub fn lax_equivalence_ty() -> Expr {
    arrow(fn_ty(real_ty(), fn_ty(real_ty(), real_ty())), prop())
}
/// `RungeKuttaOrder : Nat → (Real → Real → Real) → Real → Prop`
/// An s-stage RK method has global error order p for the given ODE and step h.
pub fn runge_kutta_order_ty() -> Expr {
    arrow(
        nat_ty(),
        arrow(
            fn_ty(real_ty(), fn_ty(real_ty(), real_ty())),
            arrow(real_ty(), prop()),
        ),
    )
}
/// `AdamsBashforthStability : Nat → Real → Prop`
/// The p-step Adams-Bashforth method has stability region containing disk of radius r.
pub fn adams_bashforth_stability_ty() -> Expr {
    arrow(nat_ty(), arrow(real_ty(), prop()))
}
/// `AdamsMoultonError : Nat → Real → Prop`
/// Adams-Moulton p-step implicit method has global error O(h^(p+1)).
pub fn adams_moulton_error_ty() -> Expr {
    arrow(nat_ty(), arrow(real_ty(), prop()))
}
/// `StiffnessRatio : (Real → Real → Real) → Real → Real → Prop`
/// The ODE has stiffness ratio σ = |λ_max|/|λ_min| at (t, y).
pub fn stiffness_ratio_ty() -> Expr {
    arrow(
        fn_ty(real_ty(), fn_ty(real_ty(), real_ty())),
        arrow(real_ty(), arrow(real_ty(), prop())),
    )
}
/// `CrankNicolsonStability : (Real → Real → Real) → Prop`
/// The Crank-Nicolson scheme is unconditionally stable (A-stable).
pub fn crank_nicolson_stability_ty() -> Expr {
    arrow(fn_ty(real_ty(), fn_ty(real_ty(), real_ty())), prop())
}
/// `FDMStabilityCondition : (Real → Real → Real) → Real → Real → Prop`
/// Von Neumann stability: amplification factor satisfies |g| ≤ 1 for given Δt, Δx.
pub fn fdm_stability_condition_ty() -> Expr {
    arrow(
        fn_ty(real_ty(), fn_ty(real_ty(), real_ty())),
        arrow(real_ty(), arrow(real_ty(), prop())),
    )
}
/// `FDMTruncationError : (Real → Real → Real) → Real → Nat → Prop`
/// The FD scheme has truncation error O(h^p) for the given PDE.
pub fn fdm_truncation_error_ty() -> Expr {
    arrow(
        fn_ty(real_ty(), fn_ty(real_ty(), real_ty())),
        arrow(real_ty(), arrow(nat_ty(), prop())),
    )
}
/// `LaxFriedrichsScheme : (Real → Real → Real) → Real → Prop`
/// The Lax-Friedrichs scheme satisfies the CFL condition with ratio r.
pub fn lax_friedrichs_scheme_ty() -> Expr {
    arrow(
        fn_ty(real_ty(), fn_ty(real_ty(), real_ty())),
        arrow(real_ty(), prop()),
    )
}
/// `ChebyshevSpectralAccuracy : (Real → Real) → Nat → Prop`
/// Chebyshev spectral approximation of f with N modes achieves spectral accuracy.
pub fn chebyshev_spectral_accuracy_ty() -> Expr {
    arrow(fn_ty(real_ty(), real_ty()), arrow(nat_ty(), prop()))
}
/// `FourierSpectralConvergence : (Real → Real) → Nat → Real → Prop`
/// Fourier spectral method with N modes has error bounded by ε for smooth f.
pub fn fourier_spectral_convergence_ty() -> Expr {
    arrow(
        fn_ty(real_ty(), real_ty()),
        arrow(nat_ty(), arrow(real_ty(), prop())),
    )
}
/// `SpectralElementOrder : Nat → Nat → Real → Prop`
/// A spectral element method of polynomial order p with K elements achieves error ε.
pub fn spectral_element_order_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), arrow(real_ty(), prop())))
}
/// `FredholmIntegralEquation : (Real → Real → Real) → (Real → Real) → Prop`
/// The Fredholm integral equation of the second kind with kernel K and rhs f is well-posed.
pub fn fredholm_integral_equation_ty() -> Expr {
    arrow(
        fn_ty(real_ty(), fn_ty(real_ty(), real_ty())),
        arrow(fn_ty(real_ty(), real_ty()), prop()),
    )
}
/// `VolterraIntegralEquation : (Real → Real → Real) → (Real → Real) → Prop`
/// The Volterra integral equation with kernel K and rhs f has a unique solution.
pub fn volterra_integral_equation_ty() -> Expr {
    arrow(
        fn_ty(real_ty(), fn_ty(real_ty(), real_ty())),
        arrow(fn_ty(real_ty(), real_ty()), prop()),
    )
}
/// `NyströmMethodConvergence : (Real → Real → Real) → Nat → Real → Prop`
/// Nyström discretization of a Fredholm equation with n nodes achieves error ε.
pub fn nystrom_method_convergence_ty() -> Expr {
    arrow(
        fn_ty(real_ty(), fn_ty(real_ty(), real_ty())),
        arrow(nat_ty(), arrow(real_ty(), prop())),
    )
}
/// `GradientDescentConvergence : (Real → Real) → Real → Nat → Prop`
/// Gradient descent with step size α converges in at most k iterations for L-smooth f.
pub fn gradient_descent_convergence_ty() -> Expr {
    arrow(
        fn_ty(real_ty(), real_ty()),
        arrow(real_ty(), arrow(nat_ty(), prop())),
    )
}
/// `NewtonMethodLocalConvergence : (Real → Real) → (Real → Real) → Real → Prop`
/// Newton's method converges quadratically from initial point x0 with radius r.
pub fn newton_method_local_convergence_ty() -> Expr {
    arrow(
        fn_ty(real_ty(), real_ty()),
        arrow(fn_ty(real_ty(), real_ty()), arrow(real_ty(), prop())),
    )
}
/// `SteepestDescentRate : (Real → Real) → Real → Real → Prop`
/// Steepest descent converges at rate (κ-1)/(κ+1) for function with condition number κ.
pub fn steepest_descent_rate_ty() -> Expr {
    arrow(
        fn_ty(real_ty(), real_ty()),
        arrow(real_ty(), arrow(real_ty(), prop())),
    )
}
/// `MonteCarloConvergence : (Real → Real) → Nat → Real → Prop`
/// Monte Carlo integration converges as O(1/√N) for N samples.
pub fn monte_carlo_convergence_ty() -> Expr {
    arrow(
        fn_ty(real_ty(), real_ty()),
        arrow(nat_ty(), arrow(real_ty(), prop())),
    )
}
/// `VarianceReduction : (Real → Real) → (Real → Real) → Real → Prop`
/// Control variate g reduces variance of f by factor r compared to crude MC.
pub fn variance_reduction_ty() -> Expr {
    arrow(
        fn_ty(real_ty(), real_ty()),
        arrow(fn_ty(real_ty(), real_ty()), arrow(real_ty(), prop())),
    )
}
/// `MCMCErgodicity : (Real → Real → Real) → Prop`
/// A Markov chain with transition kernel K is ergodic (mixes to stationary distribution).
pub fn mcmc_ergodicity_ty() -> Expr {
    arrow(fn_ty(real_ty(), fn_ty(real_ty(), real_ty())), prop())
}
/// `MCMCConvergenceRate : (Real → Real → Real) → Real → Prop`
/// The MCMC chain has spectral gap δ and converges exponentially at rate (1-δ)^n.
pub fn mcmc_convergence_rate_ty() -> Expr {
    arrow(
        fn_ty(real_ty(), fn_ty(real_ty(), real_ty())),
        arrow(real_ty(), prop()),
    )
}
/// `NumericalContinuation : (Real → Real → Real) → Real → Real → Prop`
/// Pseudo-arclength continuation follows a solution branch from λ0 to λ1.
pub fn numerical_continuation_ty() -> Expr {
    arrow(
        fn_ty(real_ty(), fn_ty(real_ty(), real_ty())),
        arrow(real_ty(), arrow(real_ty(), prop())),
    )
}
/// `BifurcationPoint : (Real → Real → Real) → Real → Prop`
/// The parameter value λ* is a bifurcation point of the parameterized system F(u,λ)=0.
pub fn bifurcation_point_ty() -> Expr {
    arrow(
        fn_ty(real_ty(), fn_ty(real_ty(), real_ty())),
        arrow(real_ty(), prop()),
    )
}
/// `IntervalArithmetic : (Real → Real) → Real → Real → Real → Real → Prop`
/// Interval arithmetic evaluates f([a,b]) ⊆ [c,d] rigorously.
pub fn interval_arithmetic_ty() -> Expr {
    arrow(
        fn_ty(real_ty(), real_ty()),
        arrow(
            real_ty(),
            arrow(real_ty(), arrow(real_ty(), arrow(real_ty(), prop()))),
        ),
    )
}
/// `VerifiedRoot : (Real → Real) → Real → Real → Prop`
/// There exists a unique root of f in interval [a,b] (interval Newton method certificate).
pub fn verified_root_ty() -> Expr {
    arrow(
        fn_ty(real_ty(), real_ty()),
        arrow(real_ty(), arrow(real_ty(), prop())),
    )
}
/// `FloatingPointRoundingError : (Real → Real) → Real → Real → Prop`
/// The floating-point evaluation of f(x) has absolute error bounded by ε.
pub fn floating_point_rounding_error_ty() -> Expr {
    arrow(
        fn_ty(real_ty(), real_ty()),
        arrow(real_ty(), arrow(real_ty(), prop())),
    )
}
/// `CancellationError : Real → Real → Real → Prop`
/// Subtracting nearly equal x ≈ y produces relative error bounded by ε.
pub fn cancellation_error_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), arrow(real_ty(), prop())))
}
/// `TikhonovRegularization : (Real → Real) → Real → Real → Prop`
/// Tikhonov regularization with parameter λ produces solution with error ε.
pub fn tikhonov_regularization_ty() -> Expr {
    arrow(
        fn_ty(real_ty(), real_ty()),
        arrow(real_ty(), arrow(real_ty(), prop())),
    )
}
/// `TruncatedSVDApproximation : Nat → Nat → Nat → Real → Prop`
/// Truncated SVD of an m×n matrix with rank k approximation has error bounded by σ_{k+1}.
pub fn truncated_svd_approximation_ty() -> Expr {
    arrow(
        nat_ty(),
        arrow(nat_ty(), arrow(nat_ty(), arrow(real_ty(), prop()))),
    )
}
/// `LASSORegularization : (Real → Real) → Real → Real → Prop`
/// LASSO with penalty λ produces a sparse solution with reconstruction error ε.
pub fn lasso_regularization_ty() -> Expr {
    arrow(
        fn_ty(real_ty(), real_ty()),
        arrow(real_ty(), arrow(real_ty(), prop())),
    )
}
/// `MultigridOptimalComplexity : Nat → Nat → Prop`
/// Multigrid solves an n-dof system to tolerance ε in O(n) operations.
pub fn multigrid_optimal_complexity_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), prop()))
}
/// `MultigridSmoothing : (Real → Real) → Nat → Real → Prop`
/// The smoother reduces high-frequency error by factor ρ after ν sweeps on mesh n.
pub fn multigrid_smoothing_ty() -> Expr {
    arrow(
        fn_ty(real_ty(), real_ty()),
        arrow(nat_ty(), arrow(real_ty(), prop())),
    )
}
/// `DomainDecompositionConvergence : Nat → Real → Prop`
/// Schwarz domain decomposition with K subdomains converges with spectral radius ρ.
pub fn domain_decomposition_convergence_ty() -> Expr {
    arrow(nat_ty(), arrow(real_ty(), prop()))
}
/// `SchurComplementCondition : Nat → Real → Prop`
/// The Schur complement of a K-subdomain decomposition has condition number κ.
pub fn schur_complement_condition_ty() -> Expr {
    arrow(nat_ty(), arrow(real_ty(), prop()))
}
/// `HpFEMExponentialConvergence : Nat → Nat → Real → Prop`
/// The hp-FEM method with p degrees and h mesh size achieves exponential convergence rate β.
pub fn hp_fem_exponential_convergence_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), arrow(real_ty(), prop())))
}
/// `TuckerDecomposition : Nat → Nat → Real → Prop`
/// A tensor of order d and mode sizes n has Tucker rank-r approximation with error ε.
pub fn tucker_decomposition_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), arrow(real_ty(), prop())))
}
/// `TensorTrainApproximation : Nat → Nat → Real → Prop`
/// A tensor of order d admits a tensor-train decomposition of rank r with error ε.
pub fn tensor_train_approximation_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), arrow(real_ty(), prop())))
}
/// `CertifiedNumericalComputation : (Real → Real) → Real → Real → Prop`
/// A computation certifies that f(x) lies in interval [lo, hi] with machine precision.
pub fn certified_numerical_computation_ty() -> Expr {
    arrow(
        fn_ty(real_ty(), real_ty()),
        arrow(real_ty(), arrow(real_ty(), prop())),
    )
}
/// `ValidatedNumericsEnclosure : (Real → Real) → Real → Real → Real → Prop`
/// The enclosure algorithm provides verified bounds [a,b] on f(x) with radius ε.
pub fn validated_numerics_enclosure_ty() -> Expr {
    arrow(
        fn_ty(real_ty(), real_ty()),
        arrow(real_ty(), arrow(real_ty(), arrow(real_ty(), prop()))),
    )
}
/// `BanachFixedPoint : ∀ (f : Real → Real) (k : Real), ContractionMapping f k → ∃ x, FixedPoint f x`
pub fn banach_fixed_point_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "f",
        fn_ty(real_ty(), real_ty()),
        pi(
            BinderInfo::Default,
            "k",
            real_ty(),
            arrow(app2(cst("ContractionMapping"), bvar(1), bvar(0)), prop()),
        ),
    )
}
/// `IntermediateValue : ∀ (f : Real → Real) (a b c : Real), f(a) ≤ c ≤ f(b) → ∃ x, f(x) = c`
pub fn intermediate_value_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "f",
        fn_ty(real_ty(), real_ty()),
        pi(
            BinderInfo::Default,
            "a",
            real_ty(),
            pi(BinderInfo::Default, "b", real_ty(), arrow(prop(), prop())),
        ),
    )
}
/// `TaylorErrorBound : ∀ (f : Real → Real) (n : Nat) (x a : Real), Prop`
pub fn taylor_error_bound_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "f",
        fn_ty(real_ty(), real_ty()),
        pi(
            BinderInfo::Default,
            "n",
            nat_ty(),
            arrow(real_ty(), arrow(real_ty(), prop())),
        ),
    )
}
/// `EulerMethodError : ∀ (f : Real → Real → Real) (h : Real), Prop`
/// Euler method global truncation error is O(h).
pub fn euler_method_error_ty() -> Expr {
    arrow(
        fn_ty(real_ty(), fn_ty(real_ty(), real_ty())),
        arrow(real_ty(), prop()),
    )
}
/// Build the numerical analysis environment: register all axioms as opaque constants.
pub fn build_numerical_analysis_env(env: &mut Environment) {
    let axioms: &[(&str, Expr)] = &[
        ("ConvergentSequence", convergent_sequence_ty()),
        ("CauchySequence", cauchy_sequence_ty()),
        ("LipschitzContinuous", lipschitz_ty()),
        ("ContractionMapping", contraction_ty()),
        ("FixedPoint", fixed_point_ty()),
        ("NewtonConvergence", newton_convergence_ty()),
        ("RungeKuttaError", runge_kutta_error_ty()),
        ("BisectionConvergence", bisection_convergence_ty()),
        ("IEEE754Rounding", ieee754_rounding_ty()),
        ("FloatCancellation", float_cancellation_ty()),
        ("ConditionNumber", condition_number_ty()),
        ("NumericalStability", numerical_stability_ty()),
        ("BackwardStability", backward_stability_ty()),
        ("BisectionRate", bisection_rate_ty()),
        ("SecantConvergence", secant_convergence_ty()),
        ("LagrangeInterpolantError", lagrange_interpolant_error_ty()),
        ("NewtonDividedDifference", newton_divided_difference_ty()),
        ("CubicSplineError", cubic_spline_error_ty()),
        ("GaussianQuadratureExact", gaussian_quadrature_exact_ty()),
        (
            "AdaptiveQuadratureConvergence",
            adaptive_quadrature_convergence_ty(),
        ),
        ("AdamsBashforthError", adams_bashforth_error_ty()),
        ("StiffODE", stiff_ode_ty()),
        ("ImplicitMethodStability", implicit_method_stability_ty()),
        (
            "FiniteDifferenceConsistency",
            finite_difference_consistency_ty(),
        ),
        ("GalerkinOrthogonality", galerkin_orthogonality_ty()),
        ("FEMConvergence", fem_convergence_ty()),
        (
            "ConjugateGradientConvergence",
            conjugate_gradient_convergence_ty(),
        ),
        ("KrylovDimension", krylov_dimension_ty()),
        ("PreconditionedSystem", preconditioned_system_ty()),
        ("SVDDecomposition", svd_decomposition_ty()),
        ("QRAlgorithmConvergence", qr_algorithm_convergence_ty()),
        (
            "PowerIterationConvergence",
            power_iteration_convergence_ty(),
        ),
        ("RichardsonExtrapolation", richardson_extrapolation_ty()),
        ("MultigridConvergence", multigrid_convergence_ty()),
        ("FastMultipoleComplexity", fast_multipole_complexity_ty()),
        ("SparseMatrixDensity", sparse_matrix_density_ty()),
        ("LaxEquivalence", lax_equivalence_ty()),
        ("banach_fixed_point", banach_fixed_point_ty()),
        ("intermediate_value", intermediate_value_ty()),
        ("taylor_error_bound", taylor_error_bound_ty()),
        ("euler_method_error", euler_method_error_ty()),
        (
            "NumericalSolution",
            arrow(fn_ty(real_ty(), real_ty()), real_ty()),
        ),
        (
            "ODESolution",
            arrow(
                fn_ty(real_ty(), fn_ty(real_ty(), real_ty())),
                fn_ty(real_ty(), real_ty()),
            ),
        ),
        ("QuadraticConvergence", prop()),
        ("SecondOrderConvergence", prop()),
        ("RungeKuttaOrder", runge_kutta_order_ty()),
        ("AdamsBashforthStability", adams_bashforth_stability_ty()),
        ("AdamsMoultonError", adams_moulton_error_ty()),
        ("StiffnessRatio", stiffness_ratio_ty()),
        ("CrankNicolsonStability", crank_nicolson_stability_ty()),
        ("FDMStabilityCondition", fdm_stability_condition_ty()),
        ("FDMTruncationError", fdm_truncation_error_ty()),
        ("LaxFriedrichsScheme", lax_friedrichs_scheme_ty()),
        (
            "ChebyshevSpectralAccuracy",
            chebyshev_spectral_accuracy_ty(),
        ),
        (
            "FourierSpectralConvergence",
            fourier_spectral_convergence_ty(),
        ),
        ("SpectralElementOrder", spectral_element_order_ty()),
        ("FredholmIntegralEquation", fredholm_integral_equation_ty()),
        ("VolterraIntegralEquation", volterra_integral_equation_ty()),
        ("NystromMethodConvergence", nystrom_method_convergence_ty()),
        (
            "GradientDescentConvergence",
            gradient_descent_convergence_ty(),
        ),
        (
            "NewtonMethodLocalConvergence",
            newton_method_local_convergence_ty(),
        ),
        ("SteepestDescentRate", steepest_descent_rate_ty()),
        ("MonteCarloConvergence", monte_carlo_convergence_ty()),
        ("VarianceReduction", variance_reduction_ty()),
        ("MCMCErgodicity", mcmc_ergodicity_ty()),
        ("MCMCConvergenceRate", mcmc_convergence_rate_ty()),
        ("NumericalContinuation", numerical_continuation_ty()),
        ("BifurcationPoint", bifurcation_point_ty()),
        ("IntervalArithmetic", interval_arithmetic_ty()),
        ("VerifiedRoot", verified_root_ty()),
        (
            "FloatingPointRoundingError",
            floating_point_rounding_error_ty(),
        ),
        ("CancellationError", cancellation_error_ty()),
        ("TikhonovRegularization", tikhonov_regularization_ty()),
        (
            "TruncatedSVDApproximation",
            truncated_svd_approximation_ty(),
        ),
        ("LASSORegularization", lasso_regularization_ty()),
        (
            "MultigridOptimalComplexity",
            multigrid_optimal_complexity_ty(),
        ),
        ("MultigridSmoothing", multigrid_smoothing_ty()),
        (
            "DomainDecompositionConvergence",
            domain_decomposition_convergence_ty(),
        ),
        ("SchurComplementCondition", schur_complement_condition_ty()),
        (
            "HpFEMExponentialConvergence",
            hp_fem_exponential_convergence_ty(),
        ),
        ("TuckerDecomposition", tucker_decomposition_ty()),
        ("TensorTrainApproximation", tensor_train_approximation_ty()),
        (
            "CertifiedNumericalComputation",
            certified_numerical_computation_ty(),
        ),
        (
            "ValidatedNumericsEnclosure",
            validated_numerics_enclosure_ty(),
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
}
/// Find a root of f in [a, b] using the bisection method.
///
/// Requires f(a) and f(b) to have opposite signs.
/// Returns `None` if the sign condition is not met or max_iter is exceeded.
pub fn bisection(
    f: &dyn Fn(f64) -> f64,
    mut a: f64,
    mut b: f64,
    tol: f64,
    max_iter: u32,
) -> Option<f64> {
    if f(a) * f(b) > 0.0 {
        return None;
    }
    for _ in 0..max_iter {
        let mid = (a + b) / 2.0;
        let fm = f(mid);
        if fm.abs() < tol || (b - a) / 2.0 < tol {
            return Some(mid);
        }
        if f(a) * fm < 0.0 {
            b = mid;
        } else {
            a = mid;
        }
    }
    Some((a + b) / 2.0)
}
/// Find a root of f using Newton-Raphson iteration.
///
/// `df` is the derivative of f.
pub fn newton_raphson(
    f: &dyn Fn(f64) -> f64,
    df: &dyn Fn(f64) -> f64,
    mut x: f64,
    tol: f64,
    max_iter: u32,
) -> Option<f64> {
    for _ in 0..max_iter {
        let fx = f(x);
        if fx.abs() < tol {
            return Some(x);
        }
        let dfx = df(x);
        if dfx.abs() < 1e-15 {
            return None;
        }
        x -= fx / dfx;
    }
    if f(x).abs() < tol {
        Some(x)
    } else {
        None
    }
}
/// Find a root of f using the secant method (no derivative required).
pub fn secant_method(
    f: &dyn Fn(f64) -> f64,
    mut x0: f64,
    mut x1: f64,
    tol: f64,
    max_iter: u32,
) -> Option<f64> {
    for _ in 0..max_iter {
        let f0 = f(x0);
        let f1 = f(x1);
        if f1.abs() < tol {
            return Some(x1);
        }
        let denom = f1 - f0;
        if denom.abs() < 1e-15 {
            return None;
        }
        let x2 = x1 - f1 * (x1 - x0) / denom;
        x0 = x1;
        x1 = x2;
    }
    if f(x1).abs() < tol {
        Some(x1)
    } else {
        None
    }
}
/// Perform a single Euler step: y_{n+1} = y_n + h * f(t_n, y_n).
pub fn euler_step(f: &dyn Fn(f64, f64) -> f64, t: f64, y: f64, h: f64) -> f64 {
    y + h * f(t, y)
}
/// Solve dy/dt = f(t, y) from t0 to t_end using the Euler method.
///
/// Returns a vector of (t, y) pairs.
pub fn euler_method(
    f: &dyn Fn(f64, f64) -> f64,
    t0: f64,
    y0: f64,
    t_end: f64,
    steps: u32,
) -> Vec<(f64, f64)> {
    let h = (t_end - t0) / steps as f64;
    let mut result = Vec::with_capacity(steps as usize + 1);
    let mut t = t0;
    let mut y = y0;
    result.push((t, y));
    for _ in 0..steps {
        y = euler_step(f, t, y, h);
        t += h;
        result.push((t, y));
    }
    result
}
/// Solve dy/dt = f(t, y) from t0 to t_end using the classic Runge-Kutta 4th order method.
///
/// Returns a vector of (t, y) pairs.
pub fn runge_kutta_4(
    f: &dyn Fn(f64, f64) -> f64,
    t0: f64,
    y0: f64,
    t_end: f64,
    steps: u32,
) -> Vec<(f64, f64)> {
    let h = (t_end - t0) / steps as f64;
    let mut result = Vec::with_capacity(steps as usize + 1);
    let mut t = t0;
    let mut y = y0;
    result.push((t, y));
    for _ in 0..steps {
        let k1 = f(t, y);
        let k2 = f(t + h / 2.0, y + h / 2.0 * k1);
        let k3 = f(t + h / 2.0, y + h / 2.0 * k2);
        let k4 = f(t + h, y + h * k3);
        y += h / 6.0 * (k1 + 2.0 * k2 + 2.0 * k3 + k4);
        t += h;
        result.push((t, y));
    }
    result
}
/// Approximate ∫_a^b f(x) dx using the trapezoidal rule with n subintervals.
pub fn trapezoidal_rule(f: &dyn Fn(f64) -> f64, a: f64, b: f64, n: u32) -> f64 {
    let h = (b - a) / n as f64;
    let mut sum = (f(a) + f(b)) / 2.0;
    for i in 1..n {
        sum += f(a + i as f64 * h);
    }
    h * sum
}
/// Approximate ∫_a^b f(x) dx using Simpson's rule with n subintervals (n must be even).
///
/// If n is odd, it is incremented by 1.
pub fn simpson_rule(f: &dyn Fn(f64) -> f64, a: f64, b: f64, n: u32) -> f64 {
    let n = if n % 2 == 0 { n } else { n + 1 };
    let h = (b - a) / n as f64;
    let mut sum = f(a) + f(b);
    for i in 1..n {
        let x = a + i as f64 * h;
        if i % 2 == 0 {
            sum += 2.0 * f(x);
        } else {
            sum += 4.0 * f(x);
        }
    }
    h / 3.0 * sum
}
/// Solve the linear system Ax = b using Gaussian elimination with partial pivoting.
///
/// Returns `None` if the system is singular.
pub fn gaussian_elimination(mut a: Vec<Vec<f64>>, mut b: Vec<f64>) -> Option<Vec<f64>> {
    let n = b.len();
    for col in 0..n {
        let mut max_row = col;
        let mut max_val = a[col][col].abs();
        for row in (col + 1)..n {
            if a[row][col].abs() > max_val {
                max_val = a[row][col].abs();
                max_row = row;
            }
        }
        if max_val < 1e-12 {
            return None;
        }
        a.swap(col, max_row);
        b.swap(col, max_row);
        let pivot = a[col][col];
        for row in (col + 1)..n {
            let factor = a[row][col] / pivot;
            b[row] -= factor * b[col];
            for k in col..n {
                a[row][k] -= factor * a[col][k];
            }
        }
    }
    let mut x = vec![0.0f64; n];
    for i in (0..n).rev() {
        let mut s = b[i];
        for j in (i + 1)..n {
            s -= a[i][j] * x[j];
        }
        if a[i][i].abs() < 1e-12 {
            return None;
        }
        x[i] = s / a[i][i];
    }
    Some(x)
}
/// Solve Ax = b iteratively using the Jacobi method.
///
/// Returns `None` if the method does not converge within `max_iter` iterations.
pub fn jacobi_iteration(
    a: &Vec<Vec<f64>>,
    b: &Vec<f64>,
    max_iter: u32,
    tol: f64,
) -> Option<Vec<f64>> {
    let n = b.len();
    let mut x = vec![0.0f64; n];
    for _ in 0..max_iter {
        let mut x_new = vec![0.0f64; n];
        for i in 0..n {
            let mut s = b[i];
            for j in 0..n {
                if j != i {
                    s -= a[i][j] * x[j];
                }
            }
            if a[i][i].abs() < 1e-15 {
                return None;
            }
            x_new[i] = s / a[i][i];
        }
        let diff: f64 = x_new
            .iter()
            .zip(x.iter())
            .map(|(a, b)| (a - b).abs())
            .fold(0.0_f64, f64::max);
        x = x_new;
        if diff < tol {
            return Some(x);
        }
    }
    None
}
/// Approximate f'(x) using the central difference formula: (f(x+h) - f(x-h)) / (2h).
pub fn numerical_derivative(f: &dyn Fn(f64) -> f64, x: f64, h: f64) -> f64 {
    (f(x + h) - f(x - h)) / (2.0 * h)
}
/// Evaluate the Lagrange interpolating polynomial at `x` given nodes `xs` and
/// values `ys`.
///
/// Panics if `xs` and `ys` have different lengths or are empty.
pub fn lagrange_interpolation(xs: &[f64], ys: &[f64], x: f64) -> f64 {
    assert_eq!(xs.len(), ys.len(), "xs and ys must have equal length");
    assert!(!xs.is_empty(), "xs must not be empty");
    let n = xs.len();
    let mut result = 0.0;
    for i in 0..n {
        let mut li = 1.0;
        for j in 0..n {
            if i != j {
                li *= (x - xs[j]) / (xs[i] - xs[j]);
            }
        }
        result += ys[i] * li;
    }
    result
}
/// Compute Newton divided differences table for nodes `xs` and values `ys`.
///
/// Returns the vector of divided-difference coefficients (diagonal of the table).
pub fn newton_divided_differences(xs: &[f64], ys: &[f64]) -> Vec<f64> {
    assert_eq!(xs.len(), ys.len());
    let n = xs.len();
    let mut d = ys.to_vec();
    for j in 1..n {
        for i in (j..n).rev() {
            d[i] = (d[i] - d[i - 1]) / (xs[i] - xs[i - j]);
        }
    }
    d
}
/// Evaluate the Newton interpolating polynomial at `x` using the divided-difference
/// coefficients `coeffs` computed from `xs`.
pub fn newton_interpolation_eval(xs: &[f64], coeffs: &[f64], x: f64) -> f64 {
    let n = xs.len();
    let mut result = coeffs[n - 1];
    for i in (0..n - 1).rev() {
        result = result * (x - xs[i]) + coeffs[i];
    }
    result
}
/// Fixed-order Gaussian quadrature using Gauss-Legendre nodes on `[a, b]`.
///
/// Supports orders 2, 3, 4, and 5; falls back to Simpson's rule for other orders.
pub fn gaussian_quadrature(f: &dyn Fn(f64) -> f64, a: f64, b: f64, order: usize) -> f64 {
    let (nodes, weights): (&[f64], &[f64]) = match order {
        2 => (
            &[-0.577_350_269_189_626, 0.577_350_269_189_626],
            &[1.0, 1.0],
        ),
        3 => (
            &[-0.774_596_669_241_483, 0.0, 0.774_596_669_241_483],
            &[
                0.555_555_555_555_556,
                0.888_888_888_888_889,
                0.555_555_555_555_556,
            ],
        ),
        4 => (
            &[
                -0.861_136_311_594_953,
                -0.339_981_043_584_856,
                0.339_981_043_584_856,
                0.861_136_311_594_953,
            ],
            &[
                0.347_854_845_137_454,
                0.652_145_154_862_546,
                0.652_145_154_862_546,
                0.347_854_845_137_454,
            ],
        ),
        5 => (
            &[
                -0.906_179_845_938_664,
                -0.538_469_310_105_683,
                0.0,
                0.538_469_310_105_683,
                0.906_179_845_938_664,
            ],
            &[
                0.236_926_885_056_189,
                0.478_628_670_499_366,
                0.568_888_888_888_889,
                0.478_628_670_499_366,
                0.236_926_885_056_189,
            ],
        ),
        _ => return simpson_rule(f, a, b, 100),
    };
    let mid = (a + b) / 2.0;
    let half = (b - a) / 2.0;
    nodes
        .iter()
        .zip(weights.iter())
        .map(|(&t, &w)| w * f(mid + half * t))
        .sum::<f64>()
        * half
}
/// Advance one step of Adams-Bashforth 2-step explicit method.
///
/// `f_prev` = f(t_{n-1}, y_{n-1}),  `f_curr` = f(t_n, y_n).
pub fn adams_bashforth_2_step(y: f64, h: f64, f_prev: f64, f_curr: f64) -> f64 {
    y + h / 2.0 * (3.0 * f_curr - f_prev)
}
/// Solve the symmetric positive definite system `A x = b` using the conjugate
/// gradient method.
///
/// Returns `(solution, num_iters)` or `None` if not converged.
pub fn conjugate_gradient(
    a: &SparseMatrix,
    b: &[f64],
    tol: f64,
    max_iter: u32,
) -> Option<(Vec<f64>, u32)> {
    let n = b.len();
    let mut x = vec![0.0f64; n];
    let mut r: Vec<f64> = b.to_vec();
    let mut p = r.clone();
    let mut rs_old: f64 = r.iter().map(|v| v * v).sum();
    for iter in 0..max_iter {
        if rs_old.sqrt() < tol {
            return Some((x, iter));
        }
        let ap = a.matvec(&p);
        let pap: f64 = p.iter().zip(ap.iter()).map(|(pi, api)| pi * api).sum();
        if pap.abs() < 1e-15 {
            return None;
        }
        let alpha = rs_old / pap;
        for i in 0..n {
            x[i] += alpha * p[i];
            r[i] -= alpha * ap[i];
        }
        let rs_new: f64 = r.iter().map(|v| v * v).sum();
        if rs_new.sqrt() < tol {
            return Some((x, iter + 1));
        }
        let beta = rs_new / rs_old;
        for i in 0..n {
            p[i] = r[i] + beta * p[i];
        }
        rs_old = rs_new;
    }
    None
}
/// Apply Richardson extrapolation to improve accuracy.
///
/// Given approximations `f_h` (step h) and `f_h2` (step h/2) for order `p`,
/// returns the extrapolated value: (2^p * f_h2 - f_h) / (2^p - 1).
pub fn richardson_extrapolation(f_h: f64, f_h2: f64, p: u32) -> f64 {
    let r = (2.0_f64).powi(p as i32);
    (r * f_h2 - f_h) / (r - 1.0)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_bisection_sqrt2() {
        let f = |x: f64| x * x - 2.0;
        let root = bisection(&f, 1.0, 2.0, 1e-9, 100).expect("operation should succeed");
        let expected = 2.0_f64.sqrt();
        assert!(
            (root - expected).abs() < 1e-6,
            "bisection sqrt2: got {root}, expected {expected}"
        );
    }
    #[test]
    fn test_newton_raphson_sqrt2() {
        let f = |x: f64| x * x - 2.0;
        let df = |x: f64| 2.0 * x;
        let root = newton_raphson(&f, &df, 1.5, 1e-10, 50).expect("operation should succeed");
        let expected = 2.0_f64.sqrt();
        assert!(
            (root - expected).abs() < 1e-9,
            "Newton sqrt2: got {root}, expected {expected}"
        );
    }
    #[test]
    fn test_euler_method() {
        let f = |_t: f64, y: f64| y;
        let result = euler_method(&f, 0.0, 1.0, 1.0, 1000);
        let (_, y_final) = result[result.len() - 1];
        let expected = 1.0_f64.exp();
        assert!(
            (y_final - expected).abs() / expected < 0.01,
            "Euler method: got {y_final}, expected {expected}"
        );
    }
    #[test]
    fn test_runge_kutta_4() {
        let f = |_t: f64, y: f64| y;
        let result = runge_kutta_4(&f, 0.0, 1.0, 1.0, 100);
        let (_, y_final) = result[result.len() - 1];
        let expected = 1.0_f64.exp();
        assert!(
            (y_final - expected).abs() < 1e-8,
            "RK4: got {y_final}, expected {expected}"
        );
    }
    #[test]
    fn test_trapezoidal_rule() {
        let f = |x: f64| x;
        let result = trapezoidal_rule(&f, 0.0, 1.0, 1000);
        assert!(
            (result - 0.5).abs() < 1e-6,
            "Trapezoidal: got {result}, expected 0.5"
        );
    }
    #[test]
    fn test_simpson_rule() {
        let f = |x: f64| x * x;
        let result = simpson_rule(&f, 0.0, 1.0, 100);
        let expected = 1.0 / 3.0;
        assert!(
            (result - expected).abs() < 1e-10,
            "Simpson: got {result}, expected {expected}"
        );
    }
    #[test]
    fn test_gaussian_elimination() {
        let a = vec![vec![2.0, 1.0], vec![1.0, 3.0]];
        let b = vec![5.0, 8.0];
        let x = gaussian_elimination(a, b).expect("operation should succeed");
        assert!((x[0] - 1.4).abs() < 1e-10, "x = {} (expected 1.4)", x[0]);
        assert!((x[1] - 2.2).abs() < 1e-10, "y = {} (expected 2.2)", x[1]);
    }
    #[test]
    fn test_numerical_derivative() {
        let f = |x: f64| x * x;
        let deriv = numerical_derivative(&f, 3.0, 1e-5);
        assert!(
            (deriv - 6.0).abs() < 1e-8,
            "Derivative: got {deriv}, expected 6.0"
        );
    }
    #[test]
    fn test_bisection_solver_struct() {
        let solver = BisectionSolver::new(1e-9, 100);
        let f = |x: f64| x * x - 3.0;
        let root = solver.solve(&f, 1.0, 2.0).expect("solve should succeed");
        let expected = 3.0_f64.sqrt();
        assert!((root - expected).abs() < 1e-6, "BisectionSolver: {root}");
    }
    #[test]
    fn test_newton_raphson_solver_struct() {
        let solver = NewtonRaphsonSolver::new(1e-10, 50);
        let f = |x: f64| x * x - 5.0;
        let df = |x: f64| 2.0 * x;
        let (root, converged) = solver.solve(&f, &df, 2.0);
        assert!(converged, "Newton-Raphson should converge");
        let expected = 5.0_f64.sqrt();
        assert!((root - expected).abs() < 1e-9, "NR root: {root}");
    }
    #[test]
    fn test_lagrange_interpolation_linear() {
        let xs = [0.0, 1.0];
        let ys = [1.0, 3.0];
        let val = lagrange_interpolation(&xs, &ys, 0.5);
        assert!((val - 2.0).abs() < 1e-12, "Lagrange linear: {val}");
    }
    #[test]
    fn test_lagrange_interpolation_quadratic() {
        let xs = [0.0, 1.0, 2.0];
        let ys = [0.0, 1.0, 4.0];
        let val = lagrange_interpolation(&xs, &ys, 1.5);
        assert!((val - 2.25).abs() < 1e-12, "Lagrange quadratic: {val}");
    }
    #[test]
    fn test_newton_divided_differences() {
        let xs = [0.0, 1.0, 2.0];
        let ys = [0.0, 1.0, 4.0];
        let coeffs = newton_divided_differences(&xs, &ys);
        let val = newton_interpolation_eval(&xs, &coeffs, 1.5);
        assert!((val - 2.25).abs() < 1e-12, "Newton DD: {val}");
    }
    #[test]
    fn test_gaussian_quadrature_order3() {
        let f = |x: f64| x * x;
        let result = gaussian_quadrature(&f, 0.0, 1.0, 3);
        let expected = 1.0 / 3.0;
        assert!(
            (result - expected).abs() < 1e-13,
            "GL order 3: got {result}, expected {expected}"
        );
    }
    #[test]
    fn test_gaussian_quadrature_order5() {
        let f = |x: f64| x * x * x * x;
        let result = gaussian_quadrature(&f, -1.0, 1.0, 5);
        let expected = 2.0 / 5.0;
        assert!(
            (result - expected).abs() < 1e-12,
            "GL order 5: got {result}, expected {expected}"
        );
    }
    #[test]
    fn test_rk4_solver_struct() {
        let solver = RungeKutta4Solver::new(0.01);
        let f = |_t: f64, y: f64| -y;
        let result = solver.integrate(&f, 0.0, 1.0, 1.0);
        let (_, y_final) = result[result.len() - 1];
        let expected = (-1.0_f64).exp();
        assert!(
            (y_final - expected).abs() < 1e-8,
            "RK4 struct: got {y_final}, expected {expected}"
        );
    }
    #[test]
    fn test_adams_bashforth_2() {
        let f = |_t: f64, y: f64| y;
        let h = 0.01;
        let y0 = 1.0;
        let y1 = euler_step(&f, 0.0, y0, h);
        let f0 = f(0.0, y0);
        let f1 = f(h, y1);
        let y2 = adams_bashforth_2_step(y1, h, f0, f1);
        let expected = (2.0 * h).exp();
        assert!((y2 - expected).abs() < 1e-4, "AB2: {y2} vs {expected}");
    }
    #[test]
    fn test_power_iteration_2x2() {
        let a = vec![vec![2.0, 1.0], vec![1.0, 2.0]];
        let solver = PowerIterationSolver::new(1e-9, 200);
        let (eig, _vec) = solver.solve(&a).expect("solve should succeed");
        assert!(
            (eig - 3.0).abs() < 1e-7,
            "Power iteration eigenvalue: {eig}"
        );
    }
    #[test]
    fn test_sparse_matrix_matvec() {
        let triplets = vec![(0, 0, 1.0), (0, 1, 2.0), (1, 0, 3.0), (1, 1, 4.0)];
        let mat = SparseMatrix::from_triplets(2, 2, &triplets);
        let x = vec![1.0, 1.0];
        let y = mat.matvec(&x);
        assert_eq!(y.len(), 2);
        assert!((y[0] - 3.0).abs() < 1e-12, "y[0] = {}", y[0]);
        assert!((y[1] - 7.0).abs() < 1e-12, "y[1] = {}", y[1]);
    }
    #[test]
    fn test_sparse_matrix_nnz() {
        let triplets = vec![(0, 0, 1.0), (1, 1, 2.0)];
        let mat = SparseMatrix::from_triplets(3, 3, &triplets);
        assert_eq!(mat.nnz(), 2);
    }
    #[test]
    fn test_conjugate_gradient_simple() {
        let triplets = vec![(0, 0, 4.0), (0, 1, 1.0), (1, 0, 1.0), (1, 1, 3.0)];
        let a = SparseMatrix::from_triplets(2, 2, &triplets);
        let b = vec![1.0, 2.0];
        let (x, _iters) = conjugate_gradient(&a, &b, 1e-12, 100).expect("operation should succeed");
        let expected_x0 = 1.0 / 11.0;
        let expected_x1 = 7.0 / 11.0;
        assert!(
            (x[0] - expected_x0).abs() < 1e-9,
            "CG x[0] = {} expected {}",
            x[0],
            expected_x0
        );
        assert!(
            (x[1] - expected_x1).abs() < 1e-9,
            "CG x[1] = {} expected {}",
            x[1],
            expected_x1
        );
    }
    #[test]
    fn test_richardson_extrapolation() {
        let f = |x: f64| x * x;
        let h = 1.0;
        let f_h = trapezoidal_rule(&f, 0.0, 1.0, 4);
        let f_h2 = trapezoidal_rule(&f, 0.0, 1.0, 8);
        let extrap = richardson_extrapolation(f_h, f_h2, 2);
        let expected = 1.0 / 3.0;
        let _ = h;
        assert!(
            (extrap - expected).abs() < 1e-10,
            "Richardson extrapolation: {extrap}"
        );
    }
    #[test]
    fn test_build_numerical_analysis_env() {
        use oxilean_kernel::Environment;
        let mut env = Environment::new();
        build_numerical_analysis_env(&mut env);
        assert!(env
            .get(&oxilean_kernel::Name::str("ConvergentSequence"))
            .is_some());
        assert!(env
            .get(&oxilean_kernel::Name::str("IEEE754Rounding"))
            .is_some());
        assert!(env
            .get(&oxilean_kernel::Name::str("SVDDecomposition"))
            .is_some());
        assert!(env
            .get(&oxilean_kernel::Name::str("LaxEquivalence"))
            .is_some());
        assert!(env
            .get(&oxilean_kernel::Name::str("SparseMatrixDensity"))
            .is_some());
    }
    #[test]
    fn test_new_axiom_builders_registered() {
        use oxilean_kernel::Environment;
        let mut env = Environment::new();
        build_numerical_analysis_env(&mut env);
        let new_axioms = [
            "RungeKuttaOrder",
            "AdamsBashforthStability",
            "AdamsMoultonError",
            "StiffnessRatio",
            "CrankNicolsonStability",
            "FDMStabilityCondition",
            "FDMTruncationError",
            "LaxFriedrichsScheme",
            "ChebyshevSpectralAccuracy",
            "FourierSpectralConvergence",
            "SpectralElementOrder",
            "FredholmIntegralEquation",
            "VolterraIntegralEquation",
            "NystromMethodConvergence",
            "GradientDescentConvergence",
            "NewtonMethodLocalConvergence",
            "SteepestDescentRate",
            "MonteCarloConvergence",
            "VarianceReduction",
            "MCMCErgodicity",
            "MCMCConvergenceRate",
            "NumericalContinuation",
            "BifurcationPoint",
            "IntervalArithmetic",
            "VerifiedRoot",
            "FloatingPointRoundingError",
            "CancellationError",
            "TikhonovRegularization",
            "TruncatedSVDApproximation",
            "LASSORegularization",
            "MultigridOptimalComplexity",
            "MultigridSmoothing",
            "DomainDecompositionConvergence",
            "SchurComplementCondition",
            "HpFEMExponentialConvergence",
            "TuckerDecomposition",
            "TensorTrainApproximation",
            "CertifiedNumericalComputation",
            "ValidatedNumericsEnclosure",
        ];
        for name in &new_axioms {
            assert!(
                env.get(&oxilean_kernel::Name::str(*name)).is_some(),
                "Axiom not registered: {name}"
            );
        }
    }
    #[test]
    fn test_interval_add() {
        let a = Interval::new(1.0, 2.0);
        let b = Interval::new(3.0, 4.0);
        let c = a.add(b);
        assert!((c.lo - 4.0).abs() < 1e-14);
        assert!((c.hi - 6.0).abs() < 1e-14);
    }
    #[test]
    fn test_interval_sub() {
        let a = Interval::new(3.0, 5.0);
        let b = Interval::new(1.0, 2.0);
        let c = a.sub(b);
        assert!((c.lo - 1.0).abs() < 1e-14, "lo = {}", c.lo);
        assert!((c.hi - 4.0).abs() < 1e-14, "hi = {}", c.hi);
    }
    #[test]
    fn test_interval_mul_positive() {
        let a = Interval::new(2.0, 3.0);
        let b = Interval::new(4.0, 5.0);
        let c = a.mul(b);
        assert!((c.lo - 8.0).abs() < 1e-14, "lo = {}", c.lo);
        assert!((c.hi - 15.0).abs() < 1e-14, "hi = {}", c.hi);
    }
    #[test]
    fn test_interval_contains() {
        let iv = Interval::new(1.0, 3.0);
        assert!(iv.contains(2.0));
        assert!(!iv.contains(4.0));
        assert!(iv.contains(1.0));
        assert!(iv.contains(3.0));
    }
    #[test]
    fn test_interval_sqrt() {
        let iv = Interval::new(4.0, 9.0);
        let s = iv.sqrt();
        assert!((s.lo - 2.0).abs() < 1e-14);
        assert!((s.hi - 3.0).abs() < 1e-14);
    }
    #[test]
    fn test_interval_mid_and_width() {
        let iv = Interval::new(1.0, 3.0);
        assert!((iv.mid() - 2.0).abs() < 1e-14);
        assert!((iv.width() - 2.0).abs() < 1e-14);
    }
    #[test]
    fn test_tikhonov_solver_identity() {
        let a = vec![vec![1.0, 0.0], vec![0.0, 1.0]];
        let b = vec![1.0, 2.0];
        let solver = TikhonovSolver::new(0.0);
        let x = solver.solve(&a, &b).expect("solve should succeed");
        assert!((x[0] - 1.0).abs() < 1e-9, "x[0] = {}", x[0]);
        assert!((x[1] - 2.0).abs() < 1e-9, "x[1] = {}", x[1]);
    }
    #[test]
    fn test_tikhonov_solver_regularized() {
        let a = vec![vec![1.0], vec![1.0]];
        let b = vec![1.0, 1.0];
        let solver = TikhonovSolver::new(1.0);
        let x = solver.solve(&a, &b).expect("solve should succeed");
        let expected = 2.0 / 3.0;
        assert!(
            (x[0] - expected).abs() < 1e-9,
            "x[0] = {} expected {expected}",
            x[0]
        );
    }
    #[test]
    fn test_gradient_descent_quadratic() {
        let grad = |v: &[f64]| vec![2.0 * v[0], 2.0 * v[1]];
        let optimizer = GradientDescentOptimizer::new(0.1, 1e-8, 500);
        let (x, _iters, converged) = optimizer.minimize(&grad, &[1.0, 1.0]);
        assert!(converged, "Gradient descent should converge");
        assert!(x[0].abs() < 1e-5, "x[0] = {}", x[0]);
        assert!(x[1].abs() < 1e-5, "x[1] = {}", x[1]);
    }
    #[test]
    fn test_crank_nicolson_steady_state() {
        let solver = CrankNicolsonSolver::new(1.0, 1.0, 4, 0.01);
        let u0 = vec![1.0; 4];
        let history = solver.integrate(&u0, 5.0);
        let u_final = &history[history.len() - 1];
        for &v in u_final {
            assert!(
                v.abs() < 0.1,
                "heat equation: value {v} should decay toward 0"
            );
        }
    }
    #[test]
    fn test_crank_nicolson_step_preserves_size() {
        let solver = CrankNicolsonSolver::new(0.5, 2.0, 5, 0.1);
        let u0 = vec![0.1, 0.2, 0.3, 0.2, 0.1];
        let u1 = solver.step(&u0).expect("step should succeed");
        assert_eq!(u1.len(), 5);
    }
    #[test]
    fn test_monte_carlo_integrate_constant() {
        let integrator = MonteCarloIntegrator::new(100_000, 42);
        let result = integrator.integrate(&|_| 1.0, 0.0, 1.0);
        assert!((result - 1.0).abs() < 0.01, "MC constant: {result}");
    }
    #[test]
    fn test_monte_carlo_integrate_linear() {
        let integrator = MonteCarloIntegrator::new(500_000, 123);
        let result = integrator.integrate(&|x| x, 0.0, 1.0);
        assert!((result - 0.5).abs() < 0.01, "MC linear: {result}");
    }
    #[test]
    fn test_monte_carlo_control_variate() {
        let integrator = MonteCarloIntegrator::new(200_000, 7);
        let result = integrator.integrate_with_control_variate(&|x| x * x, &|x| x, 0.5, 0.0, 1.0);
        let expected = 1.0 / 3.0;
        assert!(
            (result - expected).abs() < 0.01,
            "MC CV: {result}, expected {expected}"
        );
    }
}
