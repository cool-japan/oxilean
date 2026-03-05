//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{
    ADMMSolver, AdamConfig, AdamOptimizer, BinaryIntegerProgram, FrankWolfeOptimizer,
    GradientDescentConfig, GradientDescentOptimizer, LBFGSState, RegretTracker,
    RobustOptimizationProblem, SGDConfig, TwoStageStochasticProgram,
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
pub fn bvar(i: u32) -> Expr {
    Expr::BVar(i)
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
pub fn fn_ty(dom: Expr, cod: Expr) -> Expr {
    arrow(dom, cod)
}
pub fn rn_to_r() -> Expr {
    fn_ty(list_ty(real_ty()), real_ty())
}
pub fn rn_to_rn() -> Expr {
    fn_ty(list_ty(real_ty()), list_ty(real_ty()))
}
/// `FirstOrderOptimal : (Rⁿ → ℝ) → List ℝ → Prop`
/// x* satisfies ∇f(x*) = 0 (unconstrained first-order necessary condition).
pub fn first_order_optimal_ty() -> Expr {
    arrow(rn_to_r(), arrow(list_ty(real_ty()), prop()))
}
/// `SecondOrderOptimal : (Rⁿ → ℝ) → List ℝ → Prop`
/// x* satisfies ∇f(x*) = 0 and ∇²f(x*) ≻ 0.
pub fn second_order_optimal_ty() -> Expr {
    arrow(rn_to_r(), arrow(list_ty(real_ty()), prop()))
}
/// `LocalMinimum : (Rⁿ → ℝ) → List ℝ → Prop`
/// x* is a local minimiser of f.
pub fn local_minimum_ty() -> Expr {
    arrow(rn_to_r(), arrow(list_ty(real_ty()), prop()))
}
/// `GlobalMinimum : (Rⁿ → ℝ) → List ℝ → Prop`
/// x* is a global minimiser of f.
pub fn global_minimum_ty() -> Expr {
    arrow(rn_to_r(), arrow(list_ty(real_ty()), prop()))
}
/// `KKTPoint : (Rⁿ→ℝ) → List(Rⁿ→ℝ) → List(Rⁿ→ℝ) → List ℝ → List ℝ → List ℝ → Prop`
/// (f, g_eq, g_ineq, x, λ_eq, λ_ineq) satisfy KKT:
///   ∇f + Σλᵢ∇gᵢ = 0, complementary slackness, feasibility.
pub fn kkt_point_ty() -> Expr {
    let list_rn_to_r = list_ty(rn_to_r());
    let list_r = list_ty(real_ty());
    arrow(
        rn_to_r(),
        arrow(
            list_rn_to_r.clone(),
            arrow(
                list_rn_to_r,
                arrow(list_r.clone(), arrow(list_r.clone(), arrow(list_r, prop()))),
            ),
        ),
    )
}
/// `ComplementarySlackness : List (Rⁿ→ℝ) → List ℝ → List ℝ → Prop`
/// λ_i g_i(x) = 0 for all i.
pub fn complementary_slackness_ty() -> Expr {
    let list_rn_to_r = list_ty(rn_to_r());
    let list_r = list_ty(real_ty());
    arrow(list_rn_to_r, arrow(list_r.clone(), arrow(list_r, prop())))
}
/// `DualFeasible : List ℝ → Prop`
/// λ ≥ 0 (dual feasibility for inequality constraints).
pub fn dual_feasible_ty() -> Expr {
    arrow(list_ty(real_ty()), prop())
}
/// `LICQ : List (Rⁿ→ℝ) → List ℝ → Prop`
/// Linear Independence Constraint Qualification.
pub fn licq_ty() -> Expr {
    arrow(list_ty(rn_to_r()), arrow(list_ty(real_ty()), prop()))
}
/// `SlaterCondition : List (Rⁿ→ℝ) → Prop`
/// There exists strictly feasible x: g_i(x) < 0 for all i.
pub fn slater_condition_ty() -> Expr {
    arrow(list_ty(rn_to_r()), prop())
}
/// `MangasarianFromovitz : List (Rⁿ→ℝ) → List ℝ → Prop`
/// MFCQ holds at x.
pub fn mfcq_ty() -> Expr {
    arrow(list_ty(rn_to_r()), arrow(list_ty(real_ty()), prop()))
}
/// `WeakDuality : (Rⁿ→ℝ) → List(Rⁿ→ℝ) → Prop`
/// Dual objective ≤ primal objective for all feasible primal/dual pairs.
pub fn weak_duality_ty() -> Expr {
    arrow(rn_to_r(), arrow(list_ty(rn_to_r()), prop()))
}
/// `StrongDuality : (Rⁿ→ℝ) → List(Rⁿ→ℝ) → Prop`
/// Optimal primal value = optimal dual value (gap = 0).
pub fn strong_duality_ty() -> Expr {
    arrow(rn_to_r(), arrow(list_ty(rn_to_r()), prop()))
}
/// `Lagrangian : (Rⁿ→ℝ) → List(Rⁿ→ℝ) → List ℝ → List ℝ → ℝ`
/// L(x, λ) = f(x) + Σ λᵢ gᵢ(x).
pub fn lagrangian_ty() -> Expr {
    let list_rn_to_r = list_ty(rn_to_r());
    let list_r = list_ty(real_ty());
    arrow(
        rn_to_r(),
        arrow(
            list_rn_to_r,
            arrow(list_r.clone(), arrow(list_r, real_ty())),
        ),
    )
}
/// `DualFunction : (Rⁿ→ℝ) → List(Rⁿ→ℝ) → List ℝ → ℝ`
/// g(λ) = inf_x L(x, λ).
pub fn dual_function_ty() -> Expr {
    let list_rn_to_r = list_ty(rn_to_r());
    let list_r = list_ty(real_ty());
    arrow(rn_to_r(), arrow(list_rn_to_r, arrow(list_r, real_ty())))
}
/// `DualityGap : (Rⁿ→ℝ) → List(Rⁿ→ℝ) → List ℝ → List ℝ → ℝ → Prop`
/// Duality gap at (x, λ) equals ε.
pub fn duality_gap_ty() -> Expr {
    let list_rn_to_r = list_ty(rn_to_r());
    let list_r = list_ty(real_ty());
    arrow(
        rn_to_r(),
        arrow(
            list_rn_to_r,
            arrow(list_r.clone(), arrow(list_r, arrow(real_ty(), prop()))),
        ),
    )
}
/// `PenaltyObjective : (Rⁿ→ℝ) → List(Rⁿ→ℝ) → ℝ → Rⁿ→ℝ`
/// f_ρ(x) = f(x) + ρ/2 Σ max(0, g_i(x))².
pub fn penalty_objective_ty() -> Expr {
    let list_rn_to_r = list_ty(rn_to_r());
    arrow(rn_to_r(), arrow(list_rn_to_r, arrow(real_ty(), rn_to_r())))
}
/// `AugmentedLagrangian : (Rⁿ→ℝ) → List(Rⁿ→ℝ) → List ℝ → ℝ → Rⁿ→ℝ`
/// L_ρ(x, λ) = f(x) + Σλᵢgᵢ(x) + ρ/2 Σmax(0, gᵢ(x))².
pub fn augmented_lagrangian_ty() -> Expr {
    let list_rn_to_r = list_ty(rn_to_r());
    let list_r = list_ty(real_ty());
    arrow(
        rn_to_r(),
        arrow(list_rn_to_r, arrow(list_r, arrow(real_ty(), rn_to_r()))),
    )
}
/// `RegretBound : (List (List ℝ) → ℝ) → Nat → ℝ → Prop`
/// Online algorithm on T rounds achieves cumulative regret ≤ bound.
pub fn regret_bound_ty() -> Expr {
    let seq_to_r = fn_ty(list_ty(list_ty(real_ty())), real_ty());
    arrow(seq_to_r, arrow(nat_ty(), arrow(real_ty(), prop())))
}
/// `NoRegretAlgorithm : (List (List ℝ) → ℝ) → Prop`
/// Average regret → 0 as T → ∞.
pub fn no_regret_ty() -> Expr {
    let seq_to_r = fn_ty(list_ty(list_ty(real_ty())), real_ty());
    arrow(seq_to_r, prop())
}
/// `StochasticConvergence : (Rⁿ→ℝ) → Rⁿ→ℝ → Nat → ℝ → Prop`
/// SGD converges to ε-neighbourhood of minimum after T iterations.
pub fn stochastic_convergence_ty() -> Expr {
    arrow(
        rn_to_r(),
        arrow(rn_to_r(), arrow(nat_ty(), arrow(real_ty(), prop()))),
    )
}
/// `GradDescentConvergence : (Rⁿ→ℝ) → ℝ → ℝ → Nat → ℝ → Prop`
/// Gradient descent with step size α on L-smooth μ-strongly-convex f
/// achieves ‖x_k − x*‖ ≤ ε after k steps.
pub fn grad_descent_convergence_ty() -> Expr {
    arrow(
        rn_to_r(),
        arrow(
            real_ty(),
            arrow(real_ty(), arrow(nat_ty(), arrow(real_ty(), prop()))),
        ),
    )
}
/// `NesterovAcceleration : (Rⁿ→ℝ) → ℝ → Nat → ℝ → Prop`
/// Nesterov accelerated gradient achieves f(x_k) - f* ≤ O(1/k²) rate.
pub fn nesterov_acceleration_ty() -> Expr {
    arrow(
        rn_to_r(),
        arrow(real_ty(), arrow(nat_ty(), arrow(real_ty(), prop()))),
    )
}
/// `SGDConvergenceConvex : (Rⁿ→ℝ) → ℝ → ℝ → Nat → ℝ → Prop`
/// SGD on convex L-Lipschitz f with step η achieves E[f(x̄_T) - f*] ≤ ε after T steps.
pub fn sgd_convergence_convex_ty() -> Expr {
    arrow(
        rn_to_r(),
        arrow(
            real_ty(),
            arrow(real_ty(), arrow(nat_ty(), arrow(real_ty(), prop()))),
        ),
    )
}
/// `SGDConvergenceStronglyConvex : (Rⁿ→ℝ) → ℝ → ℝ → Nat → ℝ → Prop`
/// SGD on μ-strongly-convex f achieves geometric convergence rate.
pub fn sgd_convergence_sc_ty() -> Expr {
    arrow(
        rn_to_r(),
        arrow(
            real_ty(),
            arrow(real_ty(), arrow(nat_ty(), arrow(real_ty(), prop()))),
        ),
    )
}
/// `AdaGradConvergence : (Rⁿ→ℝ) → ℝ → Nat → ℝ → Prop`
/// AdaGrad achieves regret O(√T) on convex functions.
pub fn adagrad_convergence_ty() -> Expr {
    arrow(
        rn_to_r(),
        arrow(real_ty(), arrow(nat_ty(), arrow(real_ty(), prop()))),
    )
}
/// `RMSPropConvergence : (Rⁿ→ℝ) → ℝ → ℝ → Nat → ℝ → Prop`
/// RMSProp with decay ρ and learning rate α converges on smooth objectives.
pub fn rmsprop_convergence_ty() -> Expr {
    arrow(
        rn_to_r(),
        arrow(
            real_ty(),
            arrow(real_ty(), arrow(nat_ty(), arrow(real_ty(), prop()))),
        ),
    )
}
/// `AdamConvergence : (Rⁿ→ℝ) → ℝ → ℝ → ℝ → Nat → ℝ → Prop`
/// Adam optimizer achieves O(√T) regret on convex online problems.
pub fn adam_convergence_ty() -> Expr {
    arrow(
        rn_to_r(),
        arrow(
            real_ty(),
            arrow(
                real_ty(),
                arrow(real_ty(), arrow(nat_ty(), arrow(real_ty(), prop()))),
            ),
        ),
    )
}
/// `FrankWolfeConvergence : (Rⁿ→ℝ) → ℝ → Nat → ℝ → Prop`
/// Frank-Wolfe (conditional gradient) on smooth f achieves f(x_k)-f* ≤ O(1/k).
pub fn frank_wolfe_convergence_ty() -> Expr {
    arrow(
        rn_to_r(),
        arrow(real_ty(), arrow(nat_ty(), arrow(real_ty(), prop()))),
    )
}
/// `FrankWolfeFeasible : (Rⁿ→ℝ) → List(Rⁿ→ℝ) → List ℝ → Nat → Prop`
/// Frank-Wolfe iterates remain in the feasible set at each step.
pub fn frank_wolfe_feasible_ty() -> Expr {
    arrow(
        rn_to_r(),
        arrow(
            list_ty(rn_to_r()),
            arrow(list_ty(real_ty()), arrow(nat_ty(), prop())),
        ),
    )
}
/// `BregmanDivergence : (Rⁿ→ℝ) → List ℝ → List ℝ → ℝ`
/// D_h(x, y) = h(x) - h(y) - ⟨∇h(y), x-y⟩.
pub fn bregman_divergence_ty() -> Expr {
    arrow(
        rn_to_r(),
        arrow(list_ty(real_ty()), arrow(list_ty(real_ty()), real_ty())),
    )
}
/// `MirrorDescentConvergence : (Rⁿ→ℝ) → (Rⁿ→ℝ) → ℝ → Nat → ℝ → Prop`
/// Mirror descent with mirror map h and step η achieves O(1/√T) regret.
pub fn mirror_descent_convergence_ty() -> Expr {
    arrow(
        rn_to_r(),
        arrow(
            rn_to_r(),
            arrow(real_ty(), arrow(nat_ty(), arrow(real_ty(), prop()))),
        ),
    )
}
/// `UCBRegretBound : Nat → Nat → ℝ → Prop`
/// UCB1 on K arms after T rounds achieves regret O(√(KT log T)).
pub fn ucb_regret_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), arrow(real_ty(), prop())))
}
/// `ThompsonSamplingRegret : Nat → Nat → ℝ → Prop`
/// Thompson sampling achieves Bayes-optimal regret O(√(KT log K)).
pub fn thompson_sampling_regret_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), arrow(real_ty(), prop())))
}
/// `ExpThreeRegret : Nat → Nat → ℝ → Prop`
/// Exp3 on K actions after T rounds achieves regret O(√(KT log K)).
pub fn exp3_regret_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), arrow(real_ty(), prop())))
}
/// `ADMMConvergence : (Rⁿ→ℝ) → (Rⁿ→ℝ) → ℝ → Nat → ℝ → Prop`
/// ADMM for min f(x)+g(z) s.t. Ax+Bz=c converges to (x*, z*, λ*).
pub fn admm_convergence_ty() -> Expr {
    arrow(
        rn_to_r(),
        arrow(
            rn_to_r(),
            arrow(real_ty(), arrow(nat_ty(), arrow(real_ty(), prop()))),
        ),
    )
}
/// `DouglasRachfordConvergence : (Rⁿ→ℝ) → (Rⁿ→ℝ) → Nat → ℝ → Prop`
/// Douglas-Rachford splitting converges weakly to a fixed point.
pub fn douglas_rachford_convergence_ty() -> Expr {
    arrow(
        rn_to_r(),
        arrow(rn_to_r(), arrow(nat_ty(), arrow(real_ty(), prop()))),
    )
}
/// `ChambollePockConvergence : (Rⁿ→ℝ) → (Rⁿ→ℝ) → ℝ → ℝ → Nat → ℝ → Prop`
/// Chambolle-Pock primal-dual achieves O(1/N) ergodic convergence.
pub fn chambolle_pock_convergence_ty() -> Expr {
    arrow(
        rn_to_r(),
        arrow(
            rn_to_r(),
            arrow(
                real_ty(),
                arrow(real_ty(), arrow(nat_ty(), arrow(real_ty(), prop()))),
            ),
        ),
    )
}
/// `DykstraConvergence : List(Rⁿ→ℝ) → List ℝ → Nat → ℝ → Prop`
/// Dykstra's alternating projections onto convex sets converges to the projection.
pub fn dykstra_convergence_ty() -> Expr {
    arrow(
        list_ty(rn_to_r()),
        arrow(
            list_ty(real_ty()),
            arrow(nat_ty(), arrow(real_ty(), prop())),
        ),
    )
}
/// `CoordinateDescentConvergence : (Rⁿ→ℝ) → Nat → ℝ → Prop`
/// Coordinate descent on smooth convex f achieves linear convergence rate.
pub fn coordinate_descent_convergence_ty() -> Expr {
    arrow(rn_to_r(), arrow(nat_ty(), arrow(real_ty(), prop())))
}
/// `BlockCoordinateDescentConvergence : (Rⁿ→ℝ) → Nat → Nat → ℝ → Prop`
/// Block coordinate descent with B blocks on smooth convex f converges.
pub fn block_cd_convergence_ty() -> Expr {
    arrow(
        rn_to_r(),
        arrow(nat_ty(), arrow(nat_ty(), arrow(real_ty(), prop()))),
    )
}
/// `TrustRegionConvergence : (Rⁿ→ℝ) → ℝ → ℝ → Nat → Prop`
/// Trust region method achieves ‖∇f(x_k)‖ ≤ ε in O(ε^(-3/2)) iterations.
pub fn trust_region_convergence_ty() -> Expr {
    arrow(
        rn_to_r(),
        arrow(real_ty(), arrow(real_ty(), arrow(nat_ty(), prop()))),
    )
}
/// `LevenbergMarquardtConvergence : (Rⁿ→ℝ) → ℝ → Nat → ℝ → Prop`
/// Levenberg-Marquardt for nonlinear least squares achieves quadratic local convergence.
pub fn levenberg_marquardt_convergence_ty() -> Expr {
    arrow(
        rn_to_r(),
        arrow(real_ty(), arrow(nat_ty(), arrow(real_ty(), prop()))),
    )
}
/// `LBFGSConvergence : (Rⁿ→ℝ) → Nat → Nat → ℝ → Prop`
/// L-BFGS with memory m achieves superlinear convergence on strongly convex f.
pub fn lbfgs_convergence_ty() -> Expr {
    arrow(
        rn_to_r(),
        arrow(nat_ty(), arrow(nat_ty(), arrow(real_ty(), prop()))),
    )
}
/// `ConjugateGradientConvergence : (Rⁿ→ℝ) → ℝ → ℝ → Nat → ℝ → Prop`
/// Conjugate gradient for quadratic f with condition number κ achieves ε-accuracy
/// in O(√κ log(1/ε)) steps.
pub fn conjugate_gradient_convergence_ty() -> Expr {
    arrow(
        rn_to_r(),
        arrow(
            real_ty(),
            arrow(real_ty(), arrow(nat_ty(), arrow(real_ty(), prop()))),
        ),
    )
}
/// `SuccessiveConvexApprox : (Rⁿ→ℝ) → List(Rⁿ→ℝ) → Nat → ℝ → Prop`
/// SCA (MM algorithm) converges to a KKT point of the original problem.
pub fn successive_convex_approx_ty() -> Expr {
    arrow(
        rn_to_r(),
        arrow(
            list_ty(rn_to_r()),
            arrow(nat_ty(), arrow(real_ty(), prop())),
        ),
    )
}
/// `SDPWeakDuality : (Rⁿ→ℝ) → List(Rⁿ→ℝ) → Prop`
/// Weak duality holds for semidefinite programs: dual ≤ primal.
pub fn sdp_weak_duality_ty() -> Expr {
    arrow(rn_to_r(), arrow(list_ty(rn_to_r()), prop()))
}
/// `SDPStrongDuality : (Rⁿ→ℝ) → List(Rⁿ→ℝ) → Prop`
/// Strong duality holds for SDPs under Slater's condition.
pub fn sdp_strong_duality_ty() -> Expr {
    arrow(rn_to_r(), arrow(list_ty(rn_to_r()), prop()))
}
/// `SDPRankBound : Nat → Nat → ℝ → Prop`
/// A feasible SDP solution of rank r satisfies r(r+1)/2 ≤ m constraints.
pub fn sdp_rank_bound_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), arrow(real_ty(), prop())))
}
/// Build an [`Environment`] with optimization theory axioms.
pub fn build_optimization_theory_env() -> Environment {
    let mut env = Environment::new();
    let axioms: &[(&str, Expr)] = &[
        ("FirstOrderOptimal", first_order_optimal_ty()),
        ("SecondOrderOptimal", second_order_optimal_ty()),
        ("LocalMinimum", local_minimum_ty()),
        ("GlobalMinimum", global_minimum_ty()),
        ("KKTPoint", kkt_point_ty()),
        ("ComplementarySlackness", complementary_slackness_ty()),
        ("DualFeasible", dual_feasible_ty()),
        ("LICQ", licq_ty()),
        ("SlaterCondition", slater_condition_ty()),
        ("MangasarianFromovitz", mfcq_ty()),
        ("WeakDuality", weak_duality_ty()),
        ("StrongDuality", strong_duality_ty()),
        ("Lagrangian", lagrangian_ty()),
        ("DualFunction", dual_function_ty()),
        ("DualityGap", duality_gap_ty()),
        ("PenaltyObjective", penalty_objective_ty()),
        ("AugmentedLagrangian", augmented_lagrangian_ty()),
        ("RegretBound", regret_bound_ty()),
        ("NoRegretAlgorithm", no_regret_ty()),
        ("StochasticConvergence", stochastic_convergence_ty()),
        ("GradDescentConvergence", grad_descent_convergence_ty()),
        ("NesterovAcceleration", nesterov_acceleration_ty()),
        ("SGDConvergenceConvex", sgd_convergence_convex_ty()),
        ("SGDConvergenceStronglyConvex", sgd_convergence_sc_ty()),
        ("AdaGradConvergence", adagrad_convergence_ty()),
        ("RMSPropConvergence", rmsprop_convergence_ty()),
        ("AdamConvergence", adam_convergence_ty()),
        ("FrankWolfeConvergence", frank_wolfe_convergence_ty()),
        ("FrankWolfeFeasible", frank_wolfe_feasible_ty()),
        ("BregmanDivergence", bregman_divergence_ty()),
        ("MirrorDescentConvergence", mirror_descent_convergence_ty()),
        ("UCBRegretBound", ucb_regret_ty()),
        ("ThompsonSamplingRegret", thompson_sampling_regret_ty()),
        ("ExpThreeRegret", exp3_regret_ty()),
        ("ADMMConvergence", admm_convergence_ty()),
        (
            "DouglasRachfordConvergence",
            douglas_rachford_convergence_ty(),
        ),
        ("ChambollePockConvergence", chambolle_pock_convergence_ty()),
        ("DykstraConvergence", dykstra_convergence_ty()),
        (
            "CoordinateDescentConvergence",
            coordinate_descent_convergence_ty(),
        ),
        (
            "BlockCoordinateDescentConvergence",
            block_cd_convergence_ty(),
        ),
        ("TrustRegionConvergence", trust_region_convergence_ty()),
        (
            "LevenbergMarquardtConvergence",
            levenberg_marquardt_convergence_ty(),
        ),
        ("LBFGSConvergence", lbfgs_convergence_ty()),
        (
            "ConjugateGradientConvergence",
            conjugate_gradient_convergence_ty(),
        ),
        ("SuccessiveConvexApprox", successive_convex_approx_ty()),
        ("SDPWeakDuality", sdp_weak_duality_ty()),
        ("SDPStrongDuality", sdp_strong_duality_ty()),
        ("SDPRankBound", sdp_rank_bound_ty()),
        ("kkt_necessary_licq", prop()),
        ("kkt_sufficient_convex", prop()),
        ("weak_duality_theorem", prop()),
        ("strong_duality_slater", prop()),
        ("penalty_exact_kkt", prop()),
        ("sqp_superlinear_convergence", prop()),
        ("sgd_convergence_convex_smooth", prop()),
        ("ogd_regret_sqrt_t", prop()),
        ("mirror_descent_regret", prop()),
        ("interior_point_barrier_convergence", prop()),
        ("nesterov_optimal_rate", prop()),
        ("admm_linear_convergence", prop()),
        ("frank_wolfe_away_steps", prop()),
        ("lbfgs_superlinear_convergence", prop()),
        ("coordinate_descent_linear_sc", prop()),
    ];
    for (name, ty) in axioms {
        env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty: ty.clone(),
        })
        .ok();
    }
    env
}
/// Compute finite-difference gradient of f at x with step h.
pub fn finite_diff_gradient(f: &dyn Fn(&[f64]) -> f64, x: &[f64], h: f64) -> Vec<f64> {
    let n = x.len();
    let mut grad = vec![0.0; n];
    let mut xp = x.to_vec();
    let mut xm = x.to_vec();
    for i in 0..n {
        xp[i] += h;
        xm[i] -= h;
        grad[i] = (f(&xp) - f(&xm)) / (2.0 * h);
        xp[i] = x[i];
        xm[i] = x[i];
    }
    grad
}
/// Compute finite-difference Hessian of f at x.
pub fn finite_diff_hessian(f: &dyn Fn(&[f64]) -> f64, x: &[f64], h: f64) -> Vec<Vec<f64>> {
    let n = x.len();
    let mut hess = vec![vec![0.0; n]; n];
    let f0 = f(x);
    let mut xph = x.to_vec();
    let mut xmh = x.to_vec();
    let mut xphk = x.to_vec();
    let mut xmhk = x.to_vec();
    let mut xphk_ph = x.to_vec();
    for i in 0..n {
        xph[i] += h;
        xmh[i] -= h;
        hess[i][i] = (f(&xph) - 2.0 * f0 + f(&xmh)) / (h * h);
        xph[i] = x[i];
        xmh[i] = x[i];
        for j in (i + 1)..n {
            xphk[i] += h;
            xmhk[i] -= h;
            xphk_ph[i] += h;
            xphk_ph[j] += h;
            let mut xph_mh = x.to_vec();
            xph_mh[i] += h;
            xph_mh[j] -= h;
            let mut xmh_ph = x.to_vec();
            xmh_ph[i] -= h;
            xmh_ph[j] += h;
            let mut xmh_mh = x.to_vec();
            xmh_mh[i] -= h;
            xmh_mh[j] -= h;
            hess[i][j] = (f(&xphk_ph) - f(&xph_mh) - f(&xmh_ph) + f(&xmh_mh)) / (4.0 * h * h);
            hess[j][i] = hess[i][j];
            xphk[i] = x[i];
            xmhk[i] = x[i];
            xphk_ph[i] = x[i];
            xphk_ph[j] = x[i];
        }
    }
    hess
}
/// Run SGD on a differentiable objective.
///
/// `grad_f` returns the (sub)gradient at the current point.
/// Returns `(solution, final_value, iterations)`.
pub fn sgd(
    f: &dyn Fn(&[f64]) -> f64,
    grad_f: &dyn Fn(&[f64]) -> Vec<f64>,
    x0: &[f64],
    cfg: &SGDConfig,
) -> (Vec<f64>, f64, usize) {
    let n = x0.len();
    let mut x = x0.to_vec();
    let mut iters = 0;
    for t in 0..cfg.max_iter {
        let g = grad_f(&x);
        let gnorm: f64 = g.iter().map(|gi| gi * gi).sum::<f64>().sqrt();
        if gnorm < cfg.tol {
            iters = t;
            break;
        }
        let lr_t = if cfg.decay {
            cfg.lr / ((t as f64 + 1.0).sqrt())
        } else {
            cfg.lr
        };
        for i in 0..n {
            x[i] -= lr_t * g[i];
        }
        iters = t + 1;
    }
    (x.clone(), f(&x), iters)
}
/// Run Adam on a differentiable objective.
///
/// Returns `(solution, final_value, iterations)`.
pub fn adam(
    f: &dyn Fn(&[f64]) -> f64,
    grad_f: &dyn Fn(&[f64]) -> Vec<f64>,
    x0: &[f64],
    cfg: &AdamConfig,
) -> (Vec<f64>, f64, usize) {
    let n = x0.len();
    let mut x = x0.to_vec();
    let mut m = vec![0.0; n];
    let mut v = vec![0.0; n];
    let mut iters = 0;
    for t in 1..=cfg.max_iter {
        let g = grad_f(&x);
        let gnorm: f64 = g.iter().map(|gi| gi * gi).sum::<f64>().sqrt();
        if gnorm < cfg.tol {
            iters = t - 1;
            break;
        }
        for i in 0..n {
            m[i] = cfg.beta1 * m[i] + (1.0 - cfg.beta1) * g[i];
            v[i] = cfg.beta2 * v[i] + (1.0 - cfg.beta2) * g[i] * g[i];
            let m_hat = m[i] / (1.0 - cfg.beta1.powi(t as i32));
            let v_hat = v[i] / (1.0 - cfg.beta2.powi(t as i32));
            x[i] -= cfg.lr * m_hat / (v_hat.sqrt() + cfg.eps);
        }
        iters = t;
    }
    (x.clone(), f(&x), iters)
}
/// Augmented Lagrangian method for minimise f(x) s.t. g(x) = 0.
///
/// Updates:
///   x_{k+1} = argmin_x { f(x) + λ^T g(x) + ρ/2 ‖g(x)‖² }  (gradient step)
///   λ_{k+1} = λ_k + ρ g(x_{k+1})
///
/// Returns `(solution, dual_variable, iterations)`.
#[allow(clippy::too_many_arguments)]
pub fn augmented_lagrangian_method(
    _f: &dyn Fn(&[f64]) -> f64,
    grad_f: &dyn Fn(&[f64]) -> Vec<f64>,
    g: &dyn Fn(&[f64]) -> Vec<f64>,
    jac_g: &dyn Fn(&[f64]) -> Vec<Vec<f64>>,
    x0: &[f64],
    rho: f64,
    max_outer: usize,
    max_inner: usize,
    tol: f64,
) -> (Vec<f64>, Vec<f64>, usize) {
    let n = x0.len();
    let m_c = g(x0).len();
    let mut x = x0.to_vec();
    let mut lam = vec![0.0; m_c];
    let mut total_iters = 0;
    for outer in 0..max_outer {
        let aug_grad = |xk: &[f64]| -> Vec<f64> {
            let gval = g(xk);
            let jg = jac_g(xk);
            let mut grad = grad_f(xk);
            for c in 0..m_c {
                let scale = lam[c] + rho * gval[c];
                for i in 0..n {
                    grad[i] += scale * jg[c][i];
                }
            }
            grad
        };
        let lr = 0.01 / (1.0 + outer as f64);
        for _ in 0..max_inner {
            let gr = aug_grad(&x);
            let gnorm: f64 = gr.iter().map(|gi| gi * gi).sum::<f64>().sqrt();
            if gnorm < tol * 0.1 {
                break;
            }
            for i in 0..n {
                x[i] -= lr * gr[i];
            }
            total_iters += 1;
        }
        let gval = g(&x);
        for c in 0..m_c {
            lam[c] += rho * gval[c];
        }
        let feas: f64 = gval.iter().map(|gi| gi * gi).sum::<f64>().sqrt();
        if feas < tol {
            break;
        }
        let _ = outer;
    }
    (x, lam, total_iters)
}
/// Interior point method for minimise f(x) s.t. g_i(x) ≤ 0.
///
/// Uses a log-barrier: minimise f(x) − t·Σ ln(−g_i(x)).
/// The barrier parameter t is decreased geometrically.
///
/// Returns `(solution, iterations)`.
#[allow(clippy::too_many_arguments)]
pub fn interior_point(
    _f: &dyn Fn(&[f64]) -> f64,
    grad_f: &dyn Fn(&[f64]) -> Vec<f64>,
    g: &dyn Fn(&[f64]) -> Vec<f64>,
    jac_g: &dyn Fn(&[f64]) -> Vec<Vec<f64>>,
    x0: &[f64],
    mut t: f64,
    mu: f64,
    max_outer: usize,
    inner_tol: f64,
) -> (Vec<f64>, usize) {
    let n = x0.len();
    let mut x = x0.to_vec();
    let mut total_iters = 0;
    for _ in 0..max_outer {
        let barrier_grad = |xk: &[f64]| -> Option<Vec<f64>> {
            let gval = g(xk);
            let jg = jac_g(xk);
            for &gv in &gval {
                if gv >= 0.0 {
                    return None;
                }
            }
            let mut grad = grad_f(xk);
            for (c, &gv) in gval.iter().enumerate() {
                let scale = -t / gv;
                for i in 0..n {
                    grad[i] += scale * jg[c][i];
                }
            }
            Some(grad)
        };
        let lr = 0.1 * t;
        for _ in 0..100 {
            match barrier_grad(&x) {
                None => break,
                Some(gr) => {
                    let gnorm: f64 = gr.iter().map(|gi| gi * gi).sum::<f64>().sqrt();
                    if gnorm < inner_tol {
                        break;
                    }
                    let mut step = lr;
                    for _ in 0..20 {
                        let xnew: Vec<f64> =
                            x.iter().zip(&gr).map(|(xi, gi)| xi - step * gi).collect();
                        let gnew = g(&xnew);
                        if gnew.iter().all(|&gv| gv < 0.0) {
                            x = xnew;
                            break;
                        }
                        step *= 0.5;
                    }
                    total_iters += 1;
                }
            }
        }
        t /= mu;
        if t < inner_tol {
            break;
        }
    }
    (x, total_iters)
}
/// One SQP step: solve a quadratic approximation to the NLP.
///
/// Approximates f by its quadratic model and g by its linear model,
/// then solves the resulting QP using projected gradient.
///
/// Returns the updated x and multiplier estimates λ.
pub fn sqp_step(
    grad_f: &dyn Fn(&[f64]) -> Vec<f64>,
    hess_f: &dyn Fn(&[f64]) -> Vec<Vec<f64>>,
    g: &dyn Fn(&[f64]) -> Vec<f64>,
    jac_g: &dyn Fn(&[f64]) -> Vec<Vec<f64>>,
    x: &[f64],
    lam: &[f64],
    lr: f64,
) -> (Vec<f64>, Vec<f64>) {
    let n = x.len();
    let gval = g(x);
    let jg = jac_g(x);
    let gf = grad_f(x);
    let hf = hess_f(x);
    let mut lag_grad = gf.clone();
    for (c, lc) in lam.iter().enumerate() {
        for i in 0..n {
            lag_grad[i] += lc * jg[c][i];
        }
    }
    let mut dx = lag_grad.iter().map(|gi| -gi).collect::<Vec<f64>>();
    for i in 0..n {
        let hii = hf[i][i].abs().max(1e-8);
        dx[i] /= hii;
    }
    let xnew: Vec<f64> = x.iter().zip(&dx).map(|(xi, dxi)| xi + dxi).collect();
    let lam_new: Vec<f64> = lam
        .iter()
        .zip(&gval)
        .map(|(li, gi)| (li + lr * gi).max(0.0))
        .collect();
    (xnew, lam_new)
}
/// Run SQP for equality-constrained NLP.
///
/// Returns `(solution, multipliers, iterations)`.
#[allow(clippy::too_many_arguments)]
pub fn sqp(
    f: &dyn Fn(&[f64]) -> f64,
    grad_f: &dyn Fn(&[f64]) -> Vec<f64>,
    hess_f: &dyn Fn(&[f64]) -> Vec<Vec<f64>>,
    g: &dyn Fn(&[f64]) -> Vec<f64>,
    jac_g: &dyn Fn(&[f64]) -> Vec<Vec<f64>>,
    x0: &[f64],
    max_iter: usize,
    tol: f64,
) -> (Vec<f64>, Vec<f64>, usize) {
    let m_c = g(x0).len();
    let mut x = x0.to_vec();
    let mut lam = vec![0.0; m_c];
    for iter in 0..max_iter {
        let (xnew, lam_new) = sqp_step(grad_f, hess_f, g, jac_g, &x, &lam, 0.1);
        let gval = g(&xnew);
        let feas: f64 = gval.iter().map(|gi| gi * gi).sum::<f64>().sqrt();
        let gf = grad_f(&xnew);
        let opt: f64 = gf.iter().map(|gi| gi * gi).sum::<f64>().sqrt();
        x = xnew;
        lam = lam_new;
        if feas < tol && opt < tol {
            return (x, lam, iter + 1);
        }
        let _ = f;
    }
    (x, lam, max_iter)
}
/// Penalty method for minimise f(x) s.t. g(x) = 0.
///
/// Solves a sequence of unconstrained problems:
///   min_x { f(x) + ρ_k/2 ‖g(x)‖² }
/// with ρ_k → ∞.
///
/// Returns `(solution, iterations)`.
pub fn penalty_method(
    f: &dyn Fn(&[f64]) -> f64,
    grad_f: &dyn Fn(&[f64]) -> Vec<f64>,
    g: &dyn Fn(&[f64]) -> Vec<f64>,
    jac_g: &dyn Fn(&[f64]) -> Vec<Vec<f64>>,
    x0: &[f64],
    rho0: f64,
    rho_factor: f64,
    max_outer: usize,
    tol: f64,
) -> (Vec<f64>, usize) {
    let n = x0.len();
    let mut x = x0.to_vec();
    let mut rho = rho0;
    let mut total_iters = 0;
    for _ in 0..max_outer {
        let pen_grad = |xk: &[f64]| -> Vec<f64> {
            let gval = g(xk);
            let jg = jac_g(xk);
            let mut grad = grad_f(xk);
            for (c, &gv) in gval.iter().enumerate() {
                for i in 0..n {
                    grad[i] += rho * gv * jg[c][i];
                }
            }
            grad
        };
        let lr = 1.0 / (rho + 1.0);
        for _ in 0..200 {
            let gr = pen_grad(&x);
            let gnorm: f64 = gr.iter().map(|gi| gi * gi).sum::<f64>().sqrt();
            if gnorm < tol * 0.1 {
                break;
            }
            for i in 0..n {
                x[i] -= lr * gr[i];
            }
            total_iters += 1;
        }
        let gval = g(&x);
        let feas: f64 = gval.iter().map(|gi| gi * gi).sum::<f64>().sqrt();
        if feas < tol {
            break;
        }
        rho *= rho_factor;
        let _ = f;
    }
    (x, total_iters)
}
/// Online Gradient Descent algorithm.
///
/// At each round t, the adversary reveals loss function f_t; the learner
/// plays x_t and suffers f_t(x_t), then updates x_{t+1} = Π_C(x_t - η∇f_t(x_t)).
///
/// `losses`: sequence of (f_t, grad_f_t) pairs.
/// `project`: project onto feasible set.
/// Returns `(trajectory, cumulative_regret_against_best_fixed_point)`.
pub fn online_gradient_descent(
    losses: &[(Box<dyn Fn(&[f64]) -> f64>, Box<dyn Fn(&[f64]) -> Vec<f64>>)],
    x0: &[f64],
    eta: f64,
    project: &dyn Fn(Vec<f64>) -> Vec<f64>,
) -> (Vec<Vec<f64>>, Vec<f64>) {
    let n = x0.len();
    let t_max = losses.len();
    let mut x = x0.to_vec();
    let mut trajectory: Vec<Vec<f64>> = Vec::with_capacity(t_max + 1);
    trajectory.push(x.clone());
    let mut cumulative_loss = vec![0.0; t_max];
    for (t, (ft, grad_ft)) in losses.iter().enumerate() {
        cumulative_loss[t] = ft(&x);
        let g = grad_ft(&x);
        let xnew: Vec<f64> = (0..n).map(|i| x[i] - eta * g[i]).collect();
        x = project(xnew);
        trajectory.push(x.clone());
    }
    (trajectory, cumulative_loss)
}
/// Compute regret of a trajectory against a fixed comparator x*.
///
/// Regret_T = Σ_{t=1}^T (f_t(x_t) - f_t(x*)).
pub fn compute_regret(
    losses: &[Box<dyn Fn(&[f64]) -> f64>],
    trajectory: &[Vec<f64>],
    comparator: &[f64],
) -> f64 {
    losses
        .iter()
        .zip(trajectory.iter())
        .map(|(ft, xt)| ft(xt) - ft(comparator))
        .sum()
}
/// Robbins-Monro stochastic approximation.
///
/// Finds root of E[H(x, ξ)] = 0 using noisy observations H(x_t, ξ_t).
///
/// `h_oracle(x, t)` returns a noisy sample of H(x, ξ_t).
/// Step sizes: a_t = a / (t + A)^α (Polyak-Ruppert schedule).
///
/// Returns `(iterates, final_estimate)`.
#[allow(clippy::too_many_arguments)]
pub fn robbins_monro(
    h_oracle: &dyn Fn(&[f64], usize) -> Vec<f64>,
    x0: &[f64],
    a: f64,
    big_a: f64,
    alpha: f64,
    max_iter: usize,
) -> (Vec<Vec<f64>>, Vec<f64>) {
    let n = x0.len();
    let mut x = x0.to_vec();
    let mut iterates = Vec::with_capacity(max_iter);
    iterates.push(x.clone());
    for t in 0..max_iter {
        let h = h_oracle(&x, t);
        let step = a / (t as f64 + big_a).powf(alpha);
        for i in 0..n {
            x[i] -= step * h[i];
        }
        iterates.push(x.clone());
    }
    (iterates, x)
}
/// Check approximate KKT conditions for minimise f(x) s.t. g_i(x) ≤ 0.
///
/// Returns `(stationarity_error, primal_feasibility, complementary_slackness)`.
pub fn check_kkt(
    grad_f: &dyn Fn(&[f64]) -> Vec<f64>,
    g: &dyn Fn(&[f64]) -> Vec<f64>,
    jac_g: &dyn Fn(&[f64]) -> Vec<Vec<f64>>,
    x: &[f64],
    lam: &[f64],
) -> (f64, f64, f64) {
    let n = x.len();
    let gval = g(x);
    let jg = jac_g(x);
    let gf = grad_f(x);
    let mut stat = gf.clone();
    for (c, &lc) in lam.iter().enumerate() {
        for i in 0..n {
            stat[i] += lc * jg[c][i];
        }
    }
    let stat_err: f64 = stat.iter().map(|s| s * s).sum::<f64>().sqrt();
    let prim_feas: f64 = gval
        .iter()
        .map(|&gv| gv.max(0.0).powi(2))
        .sum::<f64>()
        .sqrt();
    let comp: f64 = lam
        .iter()
        .zip(&gval)
        .map(|(&lc, &gv)| (lc * gv).powi(2))
        .sum::<f64>()
        .sqrt();
    (stat_err, prim_feas, comp)
}
/// Nesterov accelerated gradient descent for smooth convex functions.
///
/// Uses the classical momentum sequence:
///   y_{k+1} = x_k - α ∇f(x_k)
///   x_{k+1} = y_{k+1} + (t_k - 1)/t_{k+1} (y_{k+1} - y_k)
///   t_{k+1} = (1 + √(1 + 4 t_k²)) / 2
///
/// Returns `(solution, final_value, iterations)`.
pub fn nesterov_gradient(
    f: &dyn Fn(&[f64]) -> f64,
    grad_f: &dyn Fn(&[f64]) -> Vec<f64>,
    x0: &[f64],
    alpha: f64,
    max_iter: usize,
    tol: f64,
) -> (Vec<f64>, f64, usize) {
    let _n = x0.len();
    let mut x = x0.to_vec();
    let mut y = x0.to_vec();
    let mut t = 1.0_f64;
    let mut iters = 0;
    for k in 0..max_iter {
        let grad = grad_f(&x);
        let gnorm: f64 = grad.iter().map(|g| g * g).sum::<f64>().sqrt();
        if gnorm < tol {
            iters = k;
            break;
        }
        let y_new: Vec<f64> = x
            .iter()
            .zip(&grad)
            .map(|(xi, gi)| xi - alpha * gi)
            .collect();
        let t_new = (1.0 + (1.0 + 4.0 * t * t).sqrt()) / 2.0;
        let momentum = (t - 1.0) / t_new;
        x = y_new
            .iter()
            .zip(&y)
            .map(|(yn, yo)| yn + momentum * (yn - yo))
            .collect();
        y = y_new;
        t = t_new;
        iters = k + 1;
    }
    (x.clone(), f(&x), iters)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sgd_minimises_quadratic() {
        let f = |x: &[f64]| x[0] * x[0] + x[1] * x[1];
        let grad_f = |x: &[f64]| vec![2.0 * x[0], 2.0 * x[1]];
        let cfg = SGDConfig::new(0.1, 1000, 1e-6);
        let (x, fval, _iters) = sgd(&f, &grad_f, &[3.0, -2.0], &cfg);
        assert!(fval < 1e-6, "SGD fval={fval}");
        assert!(x[0].abs() < 1e-3, "x[0]={}", x[0]);
        assert!(x[1].abs() < 1e-3, "x[1]={}", x[1]);
    }
    #[test]
    fn test_adam_minimises_quadratic() {
        let f = |x: &[f64]| x[0] * x[0] + 4.0 * x[1] * x[1];
        let grad_f = |x: &[f64]| vec![2.0 * x[0], 8.0 * x[1]];
        let cfg = AdamConfig::default_params(0.1, 2000);
        let (x, fval, _) = adam(&f, &grad_f, &[5.0, 3.0], &cfg);
        assert!(fval < 1e-4, "Adam fval={fval}");
        assert!(x[0].abs() < 0.02, "x[0]={}", x[0]);
        assert!(x[1].abs() < 0.02, "x[1]={}", x[1]);
    }
    #[test]
    fn test_finite_diff_gradient() {
        let f = |x: &[f64]| x[0] * x[0] + 2.0 * x[1] * x[1];
        let x = vec![3.0, -1.0];
        let g = finite_diff_gradient(&f, &x, 1e-5);
        assert!((g[0] - 6.0).abs() < 1e-6, "g[0]={}", g[0]);
        assert!((g[1] + 4.0).abs() < 1e-6, "g[1]={}", g[1]);
    }
    #[test]
    fn test_finite_diff_hessian() {
        let f = |x: &[f64]| x[0] * x[0] + 3.0 * x[1] * x[1];
        let x = vec![1.0, 1.0];
        let h = finite_diff_hessian(&f, &x, 1e-4);
        assert!((h[0][0] - 2.0).abs() < 1e-4, "H[0,0]={}", h[0][0]);
        assert!((h[1][1] - 6.0).abs() < 1e-4, "H[1,1]={}", h[1][1]);
        assert!(h[0][1].abs() < 1e-4, "H[0,1]={}", h[0][1]);
    }
    #[test]
    fn test_check_kkt_unconstrained_minimum() {
        let grad_f = |x: &[f64]| vec![2.0 * x[0]];
        let g = |_x: &[f64]| -> Vec<f64> { vec![] };
        let jac_g = |_x: &[f64]| -> Vec<Vec<f64>> { vec![] };
        let (stat_err, prim_feas, comp) = check_kkt(&grad_f, &g, &jac_g, &[0.0], &[]);
        assert!(stat_err < 1e-12, "stat_err={stat_err}");
        assert!(prim_feas < 1e-12, "prim_feas={prim_feas}");
        assert!(comp < 1e-12, "comp={comp}");
    }
    #[test]
    fn test_penalty_method_equality() {
        let f = |x: &[f64]| x[0] * x[0] + x[1] * x[1];
        let grad_f = |x: &[f64]| vec![2.0 * x[0], 2.0 * x[1]];
        let g = |x: &[f64]| vec![x[0] + x[1] - 1.0];
        let jac_g = |_x: &[f64]| vec![vec![1.0, 1.0]];
        let (x, _iters) = penalty_method(&f, &grad_f, &g, &jac_g, &[0.5, 0.5], 1.0, 3.0, 15, 1e-5);
        assert!((x[0] - 0.5).abs() < 0.05, "x[0]={}", x[0]);
        assert!((x[1] - 0.5).abs() < 0.05, "x[1]={}", x[1]);
    }
    #[test]
    fn test_ogd_cumulative_loss() {
        let t_max = 20;
        let losses: Vec<(Box<dyn Fn(&[f64]) -> f64>, Box<dyn Fn(&[f64]) -> Vec<f64>>)> = (0..t_max)
            .map(|_| {
                let f: Box<dyn Fn(&[f64]) -> f64> = Box::new(|x: &[f64]| (x[0] - 1.0).powi(2));
                let g: Box<dyn Fn(&[f64]) -> Vec<f64>> =
                    Box::new(|x: &[f64]| vec![2.0 * (x[0] - 1.0)]);
                (f, g)
            })
            .collect();
        let project = |x: Vec<f64>| x;
        let (_traj, cum_loss) = online_gradient_descent(&losses, &[0.0], 0.1, &project);
        let avg_loss: f64 = cum_loss.iter().sum::<f64>() / t_max as f64;
        assert!(avg_loss < 1.0, "avg_loss={avg_loss}");
    }
    #[test]
    fn test_build_optimization_theory_env() {
        let env = build_optimization_theory_env();
        assert!(env.get(&Name::str("FirstOrderOptimal")).is_some());
        assert!(env.get(&Name::str("KKTPoint")).is_some());
        assert!(env.get(&Name::str("WeakDuality")).is_some());
        assert!(env.get(&Name::str("AugmentedLagrangian")).is_some());
        assert!(env.get(&Name::str("RegretBound")).is_some());
        assert!(env.get(&Name::str("StochasticConvergence")).is_some());
        assert!(env.get(&Name::str("GradDescentConvergence")).is_some());
        assert!(env.get(&Name::str("NesterovAcceleration")).is_some());
        assert!(env.get(&Name::str("AdamConvergence")).is_some());
        assert!(env.get(&Name::str("FrankWolfeConvergence")).is_some());
        assert!(env.get(&Name::str("BregmanDivergence")).is_some());
        assert!(env.get(&Name::str("MirrorDescentConvergence")).is_some());
        assert!(env.get(&Name::str("UCBRegretBound")).is_some());
        assert!(env.get(&Name::str("ADMMConvergence")).is_some());
        assert!(env.get(&Name::str("DouglasRachfordConvergence")).is_some());
        assert!(env.get(&Name::str("ChambollePockConvergence")).is_some());
        assert!(env.get(&Name::str("DykstraConvergence")).is_some());
        assert!(env
            .get(&Name::str("CoordinateDescentConvergence"))
            .is_some());
        assert!(env.get(&Name::str("TrustRegionConvergence")).is_some());
        assert!(env.get(&Name::str("LBFGSConvergence")).is_some());
        assert!(env.get(&Name::str("SDPStrongDuality")).is_some());
    }
    #[test]
    fn test_gradient_descent_armijo() {
        let f = |x: &[f64]| x[0] * x[0] + 2.0 * x[1] * x[1];
        let grad_f = |x: &[f64]| vec![2.0 * x[0], 4.0 * x[1]];
        let cfg = GradientDescentConfig::new(500, 1e-6);
        let mut opt = GradientDescentOptimizer::new(vec![4.0, -3.0], cfg);
        let (x, fval, _iters) = opt.run(&f, &grad_f);
        assert!(fval < 1e-6, "GD fval={fval}");
        assert!(x[0].abs() < 1e-3, "x[0]={}", x[0]);
        assert!(x[1].abs() < 1e-3, "x[1]={}", x[1]);
    }
    #[test]
    fn test_adam_optimizer_struct() {
        let f = |x: &[f64]| (x[0] - 2.0).powi(2) + (x[1] + 1.0).powi(2);
        let grad_f = |x: &[f64]| vec![2.0 * (x[0] - 2.0), 2.0 * (x[1] + 1.0)];
        let mut opt = AdamOptimizer::new(vec![0.0, 0.0], 0.05, 0.9, 0.999, 1e-8);
        let (x, fval, _steps) = opt.run(&f, &grad_f, 3000, 1e-6);
        assert!(fval < 0.01, "Adam struct fval={fval}");
        assert!((x[0] - 2.0).abs() < 0.05, "x[0]={}", x[0]);
        assert!((x[1] + 1.0).abs() < 0.05, "x[1]={}", x[1]);
    }
    #[test]
    fn test_frank_wolfe_simplex() {
        let f = |x: &[f64]| (x[0] - 0.3).powi(2) + (x[1] - 0.7).powi(2);
        let grad_f = |x: &[f64]| vec![2.0 * (x[0] - 0.3), 2.0 * (x[1] - 0.7)];
        let lmo = |g: &[f64]| {
            let imin = if g[0] < g[1] { 0 } else { 1 };
            let mut s = vec![0.0; g.len()];
            s[imin] = 1.0;
            s
        };
        let mut fw = FrankWolfeOptimizer::new(vec![0.5, 0.5]);
        let (_x, fval, _iters) = fw.run(&f, &grad_f, &lmo, 200, 1e-6);
        assert!(fval < 0.01, "FW fval={fval}");
    }
    #[test]
    fn test_admm_consensus() {
        let rho = 1.0_f64;
        let x_update = move |z: &[f64], u: &[f64]| vec![rho * (z[0] - u[0]) / (2.0 + rho)];
        let z_update = move |x: &[f64], u: &[f64]| vec![rho * (x[0] + u[0]) / (2.0 + rho)];
        let constraint = |x: &[f64], z: &[f64]| vec![x[0] - z[0]];
        let mut admm = ADMMSolver::new(rho, vec![1.0], vec![1.0], vec![0.0]);
        let (x, z, prim, _iters) = admm.run(&x_update, &z_update, &constraint, 200, 1e-8, 1e-4);
        assert!(prim < 1e-6, "ADMM primal residual={prim}");
        assert!(x[0].abs() < 0.01, "x={}", x[0]);
        assert!(z[0].abs() < 0.01, "z={}", z[0]);
    }
    #[test]
    fn test_nesterov_gradient_quadratic() {
        let f = |x: &[f64]| x[0] * x[0] + 4.0 * x[1] * x[1];
        let grad_f = |x: &[f64]| vec![2.0 * x[0], 8.0 * x[1]];
        let (x, fval, _iters) = nesterov_gradient(&f, &grad_f, &[3.0, 2.0], 0.1, 500, 1e-7);
        assert!(fval < 1e-6, "Nesterov fval={fval}");
        assert!(x[0].abs() < 1e-3, "x[0]={}", x[0]);
    }
    #[test]
    fn test_regret_tracker() {
        let mut tracker = RegretTracker::new();
        for _ in 0..10 {
            let ft = |x: &[f64]| (x[0] - 1.0).powi(2);
            tracker.record(&ft, &[0.0], &[1.0]);
        }
        assert_eq!(tracker.rounds, 10);
        assert!((tracker.total_regret() - 10.0).abs() < 1e-10);
        assert_eq!(tracker.average_regret(), 1.0);
        assert!(!tracker.is_no_regret(0.5));
    }
    #[test]
    fn test_lbfgs_quadratic() {
        let f = |x: &[f64]| x[0].powi(2) + 4.0 * x[1].powi(2) + x[2].powi(2);
        let grad_f = |x: &[f64]| vec![2.0 * x[0], 8.0 * x[1], 2.0 * x[2]];
        let mut lbfgs = LBFGSState::new(vec![2.0, 1.0, -3.0], 5);
        let (x, fval, _iters) = lbfgs.run(&f, &grad_f, 200, 1e-8);
        assert!(fval < 1e-8, "L-BFGS fval={fval}");
        assert!(x[0].abs() < 1e-4, "x[0]={}", x[0]);
        assert!(x[1].abs() < 1e-4, "x[1]={}", x[1]);
        assert!(x[2].abs() < 1e-4, "x[2]={}", x[2]);
    }
}
#[cfg(test)]
mod tests_optimization_extended {
    use super::*;
    #[test]
    fn test_robust_worst_case_cost() {
        let prob = RobustOptimizationProblem::new(2, 2, 0.5, vec![1.0, 2.0]);
        let x = vec![1.0, 1.0];
        let wc = prob.worst_case_cost(&x);
        assert!((wc - 4.0).abs() < 1e-10);
    }
    #[test]
    fn test_ellipsoidal_worst_case() {
        let prob = RobustOptimizationProblem::new(2, 2, 1.0, vec![0.0, 0.0]);
        let x = vec![3.0, 4.0];
        let wc = prob.ellipsoidal_worst_case(&x);
        assert!((wc - 5.0).abs() < 1e-10);
    }
    #[test]
    fn test_bip_feasibility() {
        let bip = BinaryIntegerProgram::new(vec![1.0, 2.0], vec![vec![1.0, 1.0]], vec![1.0]);
        assert!(bip.is_feasible(&[true, false]));
        assert!(!bip.is_feasible(&[true, true]));
    }
    #[test]
    fn test_bip_greedy_solution() {
        let bip = BinaryIntegerProgram::new(vec![-3.0, -1.0], vec![vec![1.0, 1.0]], vec![1.0]);
        let sol = bip.greedy_solution();
        assert!(sol[0]);
    }
    #[test]
    fn test_two_stage_expected_cost() {
        let prog = TwoStageStochasticProgram::new(vec![1.0], vec![0.3, 0.7], vec![10.0, 5.0]);
        let eq = prog.expected_second_stage(&[1.0]);
        assert!((eq - 6.5).abs() < 1e-10);
    }
    #[test]
    fn test_value_of_perfect_information() {
        let prog = TwoStageStochasticProgram::new(vec![1.0], vec![0.5, 0.5], vec![10.0, 2.0]);
        let vpi = prog.value_of_perfect_information();
        assert!(vpi >= 0.0);
    }
}
