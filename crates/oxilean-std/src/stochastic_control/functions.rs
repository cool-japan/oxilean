//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{
    ActionValueFunction, ActorCritic, AlmostSureStability, BeliefMDP, ErgodicControl,
    ExponentialMSStability, HInfinityControl, MeanFieldGame, MeanFieldGameSolver,
    MeanSquareStability, NashEquilibrium, PathwiseSDE, Policy, PolicyGradient, PursuitEvasionGame,
    QLearning, QLearningSolver, RiccatiEquation, RiskSensitiveControl, RiskSensitiveCost, SDGame,
    StochasticLyapunov, ValueIteration, ZeroSumSDG, MDP, SARSA,
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
pub fn bool_ty() -> Expr {
    cst("Bool")
}
pub fn fn_ty(dom: Expr, cod: Expr) -> Expr {
    arrow(dom, cod)
}
/// `MDP : (Nat -> Nat -> Real -> Nat -> Real -> Prop) -> Prop`
/// Markov Decision Process: (S, A, P, R, γ) = (states, actions, transitions, rewards, discount).
pub fn mdp_ty() -> Expr {
    let rel = fn_ty(
        nat_ty(),
        fn_ty(
            nat_ty(),
            fn_ty(real_ty(), fn_ty(nat_ty(), fn_ty(real_ty(), prop()))),
        ),
    );
    arrow(rel, prop())
}
/// `Policy : Nat -> Nat`
/// Deterministic policy: maps each state to an action.
pub fn policy_ty() -> Expr {
    fn_ty(nat_ty(), nat_ty())
}
/// `StochasticPolicy : Nat -> (Nat -> Real)`
/// Stochastic policy: maps each state to a probability distribution over actions.
pub fn stochastic_policy_ty() -> Expr {
    fn_ty(nat_ty(), fn_ty(nat_ty(), real_ty()))
}
/// `ValueFunction : (Nat -> Nat) -> Nat -> Real`
/// V^π(s) = E[∑ γ^t r_t | s_0=s, π]. Maps policy and state to a real value.
pub fn value_function_ty() -> Expr {
    fn_ty(fn_ty(nat_ty(), nat_ty()), fn_ty(nat_ty(), real_ty()))
}
/// `ActionValueFunction : (Nat -> Nat) -> Nat -> Nat -> Real`
/// Q^π(s,a) = expected return from state s, taking action a, then following π.
pub fn action_value_function_ty() -> Expr {
    fn_ty(
        fn_ty(nat_ty(), nat_ty()),
        fn_ty(nat_ty(), fn_ty(nat_ty(), real_ty())),
    )
}
/// `BellmanOperator : Prop`
/// The Bellman operator T^π V(s) = R(s,π(s)) + γ Σ P(s'|s,π(s)) V(s') is a contraction.
pub fn bellman_operator_ty() -> Expr {
    prop()
}
/// `PolicyEvaluation : Prop`
/// Policy evaluation computes V^π as the unique fixed-point of T^π.
pub fn policy_evaluation_ty() -> Expr {
    prop()
}
/// `PolicyImprovement : Prop`
/// The greedy policy w.r.t. V^π is at least as good as π (policy improvement theorem).
pub fn policy_improvement_ty() -> Expr {
    prop()
}
/// `ValueIteration : Prop`
/// Value iteration converges to V* = V^{π*} when γ < 1.
pub fn value_iteration_ty() -> Expr {
    prop()
}
/// `HamiltonJacobiBellman : Prop`
/// HJB PDE: V_t + inf_u{L(x,u) + ∇V·f(x,u)} = 0 characterises optimal value.
pub fn hjb_ty() -> Expr {
    prop()
}
/// `StochasticOptimalControl : Prop`
/// Itô dynamics: dx = f(x,u)dt + σ(x,u)dW; optimal cost via HJB.
pub fn stochastic_optimal_control_ty() -> Expr {
    prop()
}
/// `LinearQuadraticRegulator : (List (List Real)) -> (List (List Real)) -> (List (List Real)) -> (List (List Real)) -> Prop`
/// LQR/LQG: minimise E\[∫(x^T Q x + u^T R u)dt\] subject to linear stochastic dynamics.
pub fn lqr_ty() -> Expr {
    let mat = list_ty(list_ty(real_ty()));
    arrow(
        mat.clone(),
        arrow(mat.clone(), arrow(mat.clone(), arrow(mat, prop()))),
    )
}
/// `RiccatiEquation : Prop`
/// P_t = A^T P + P A - P B R^{-1} B^T P + Q: differential Riccati equation for LQR.
pub fn riccati_equation_ty() -> Expr {
    prop()
}
/// `solve_riccati_ty : Prop`
/// The algebraic Riccati equation has a unique positive-definite solution under stabilisability.
pub fn solve_riccati_ty() -> Expr {
    prop()
}
/// `optimal_gain_matrix_ty : Prop`
/// Optimal LQR gain K* = R^{-1} B^T P where P solves the Riccati equation.
pub fn optimal_gain_matrix_ty() -> Expr {
    prop()
}
/// `infinite_horizon_lqr_ty : Prop`
/// Infinite-horizon LQR: steady-state optimal gain stabilises the system.
pub fn infinite_horizon_lqr_ty() -> Expr {
    prop()
}
/// `QLearning : Real -> Real -> Nat -> Nat -> Real -> Nat -> Prop`
/// Q(s,a) ← Q(s,a) + α(r + γ max Q(s',a') − Q(s,a)) off-policy TD control.
pub fn q_learning_ty() -> Expr {
    arrow(
        real_ty(),
        arrow(
            real_ty(),
            arrow(
                nat_ty(),
                arrow(nat_ty(), arrow(real_ty(), arrow(nat_ty(), prop()))),
            ),
        ),
    )
}
/// `SARSA : Real -> Real -> Nat -> Nat -> Real -> Nat -> Nat -> Prop`
/// On-policy TD(0): Q(s,a) ← Q(s,a) + α(r + γ Q(s',a') − Q(s,a)).
pub fn sarsa_ty() -> Expr {
    arrow(
        real_ty(),
        arrow(
            real_ty(),
            arrow(
                nat_ty(),
                arrow(
                    nat_ty(),
                    arrow(real_ty(), arrow(nat_ty(), arrow(nat_ty(), prop()))),
                ),
            ),
        ),
    )
}
/// `PolicyGradient : Prop`
/// ∇_θ J(π_θ) = E[∇_θ log π_θ(a|s) · Q^π(s,a)] (REINFORCE theorem).
pub fn policy_gradient_ty() -> Expr {
    prop()
}
/// `ActorCritic : Prop`
/// Actor-critic: policy (actor) updated by gradient, value (critic) updated by TD error.
pub fn actor_critic_ty() -> Expr {
    prop()
}
/// `convergence_rate_rl_ty : Prop`
/// Q-learning converges to Q* under appropriate step-size conditions (Watkins & Dayan 1992).
pub fn convergence_rate_rl_ty() -> Expr {
    prop()
}
/// `ZeroSumSDG : Prop`
/// J = E\[∫ L(x,u,v)dt + g(x_T)\]: zero-sum stochastic differential game objective.
pub fn zero_sum_sdg_ty() -> Expr {
    prop()
}
/// `NashEquilibrium : Prop`
/// (u*, v*) such that neither player can benefit from unilateral deviation.
pub fn nash_equilibrium_ty() -> Expr {
    prop()
}
/// `IsaacEquation : Prop`
/// Hamilton-Jacobi-Isaacs PDE: saddle-point condition for zero-sum SDG.
pub fn isaac_equation_ty() -> Expr {
    prop()
}
/// `PursuitEvasionGame : Prop`
/// Classical pursuit-evasion differential game with pursuit and evader dynamics.
pub fn pursuit_evasion_game_ty() -> Expr {
    prop()
}
/// `StochasticLyapunov : Prop`
/// V with LV ≤ -αV + β: Foster-Lyapunov condition for stability.
pub fn stochastic_lyapunov_ty() -> Expr {
    prop()
}
/// `MeanSquareStability : Prop`
/// E[||x_t||²] → 0 as t → ∞.
pub fn mean_square_stability_ty() -> Expr {
    prop()
}
/// `AlmostSureStability : Prop`
/// ||x_t|| → 0 almost surely.
pub fn almost_sure_stability_ty() -> Expr {
    prop()
}
/// `ExponentialMSStability : Prop`
/// E[||x_t||²] ≤ C e^{-λt} for some C,λ > 0.
pub fn exponential_ms_stability_ty() -> Expr {
    prop()
}
/// `POMDP : Prop`
/// POMDP: (S, A, O, T, Z, R, γ) — states, actions, observations, transition,
/// observation, reward, discount.
pub fn pomdp_ty() -> Expr {
    prop()
}
/// `BeliefState : (Nat -> Real) -> Prop`
/// Belief b: Δ(S) — probability distribution over states.
pub fn belief_state_ty() -> Expr {
    arrow(fn_ty(nat_ty(), real_ty()), prop())
}
/// `BeliefUpdate : (Nat -> Real) -> Nat -> Nat -> (Nat -> Real) -> Prop`
/// Bayesian belief update: b'(s') ∝ Z(o|s',a) Σ_s T(s'|s,a) b(s).
pub fn belief_update_ty() -> Expr {
    arrow(
        fn_ty(nat_ty(), real_ty()),
        arrow(
            nat_ty(),
            arrow(nat_ty(), arrow(fn_ty(nat_ty(), real_ty()), prop())),
        ),
    )
}
/// `BeliefMDP : Prop`
/// Belief-space MDP induced by a POMDP: states are belief distributions.
pub fn belief_mdp_ty() -> Expr {
    prop()
}
/// `QMDPApproximation : Prop`
/// QMDP approximation: V(b) ≈ max_a Σ_s b(s) Q*(s, a) — ignores future observations.
pub fn qmdp_approximation_ty() -> Expr {
    prop()
}
/// `EntropicRisk : Real -> (Nat -> Real) -> Real`
/// Entropic risk measure: ρ_θ(X) = (1/θ) log E\[e^{θX}\].
pub fn entropic_risk_ty() -> Expr {
    arrow(real_ty(), arrow(fn_ty(nat_ty(), real_ty()), real_ty()))
}
/// `CVaROptimization : Real -> Prop`
/// CVaR (Conditional Value-at-Risk) at level α: minimise E[X | X ≥ VaR_α(X)].
pub fn cvar_optimization_ty() -> Expr {
    arrow(real_ty(), prop())
}
/// `CoherentRiskMeasure : ((Nat -> Real) -> Real) -> Prop`
/// A coherent risk measure satisfies monotonicity, sub-additivity, positive homogeneity,
/// and translation invariance.
pub fn coherent_risk_measure_ty() -> Expr {
    arrow(fn_ty(fn_ty(nat_ty(), real_ty()), real_ty()), prop())
}
/// `RiskSensitiveBellman : Real -> Prop`
/// Risk-sensitive Bellman equation: V(s) = min_a {l(s,a) + (1/θ) log E\[e^{θ V(s')}\]}.
pub fn risk_sensitive_bellman_ty() -> Expr {
    arrow(real_ty(), prop())
}
/// `MinimaxControl : Prop`
/// Minimax (H∞) control: min_u max_{w ∈ W} J(u, w) — worst-case disturbance.
pub fn minimax_control_ty() -> Expr {
    prop()
}
/// `HInfinityStochastic : Prop`
/// H∞ stochastic control: robust to stochastic uncertainty bounded by an ambiguity set.
pub fn h_infinity_stochastic_ty() -> Expr {
    prop()
}
/// `AmbiguitySet : ((Nat -> Real) -> Prop) -> Prop`
/// Ambiguity set: collection of distributions consistent with observed data.
pub fn ambiguity_set_ty() -> Expr {
    arrow(fn_ty(fn_ty(nat_ty(), real_ty()), prop()), prop())
}
/// `DistributionallyRobustMDP : Prop`
/// Distributionally robust MDP: optimise worst-case expected return over an ambiguity set.
pub fn distributionally_robust_mdp_ty() -> Expr {
    prop()
}
/// `MeanFieldGame : Prop`
/// MFG: large-population game where each agent interacts with the population distribution.
pub fn mean_field_game_ty() -> Expr {
    prop()
}
/// `McKeanVlasovSDE : Prop`
/// McKean-Vlasov SDE: dX = b(X, μ_t)dt + σ(X, μ_t)dW, μ_t = Law(X_t).
pub fn mckean_vlasov_sde_ty() -> Expr {
    prop()
}
/// `MFGNashEquilibrium : (Nat -> Real) -> Prop`
/// MFG Nash equilibrium: (u*, μ*) such that u* is optimal given μ* and μ* = Law(X^{u*}).
pub fn mfg_nash_equilibrium_ty() -> Expr {
    arrow(fn_ty(nat_ty(), real_ty()), prop())
}
/// `MFGConsistencyCondition : Prop`
/// Consistency: the distribution induced by optimal play equals the equilibrium distribution.
pub fn mfg_consistency_ty() -> Expr {
    prop()
}
/// `TwoPlayerZeroSumGame : (Nat -> Nat -> Nat -> Real) -> Prop`
/// Two-player zero-sum stochastic game: reward kernel r(s, a1, a2).
pub fn two_player_zero_sum_game_ty() -> Expr {
    let reward_kernel = fn_ty(nat_ty(), fn_ty(nat_ty(), fn_ty(nat_ty(), real_ty())));
    arrow(reward_kernel, prop())
}
/// `ShapleyOperator : Prop`
/// Shapley (dynamic programming) operator for stochastic games.
pub fn shapley_operator_ty() -> Expr {
    prop()
}
/// `AverageCostMDP : Prop`
/// Ergodic/average-cost MDP: min_{π} lim_{T→∞} (1/T) E\[Σ_{t=0}^{T-1} r_t\].
pub fn average_cost_mdp_ty() -> Expr {
    prop()
}
/// `PoissonEquation : Prop`
/// Poisson equation for ergodic MDP: λ + h(s) = min_a {r(s,a) + Σ P(s'|s,a) h(s')}.
pub fn poisson_equation_ty() -> Expr {
    prop()
}
/// `BiasFunction : (Nat -> Real) -> Prop`
/// Bias function h in the Poisson equation: captures relative costs across states.
pub fn bias_function_ty() -> Expr {
    arrow(fn_ty(nat_ty(), real_ty()), prop())
}
/// `OptimalStopping : Prop`
/// Optimal stopping: choose stopping time τ to maximise E\[g(X_τ)\] — free boundary problem.
pub fn optimal_stopping_ty() -> Expr {
    prop()
}
/// `DynkinFormula : Prop`
/// Dynkin's formula: E\[f(X_τ)\] = f(x) + E\[∫_0^τ Lf(X_t)dt\] for Markov processes.
pub fn dynkin_formula_ty() -> Expr {
    prop()
}
/// `QuasiVariationalInequality : Prop`
/// QVI for impulse control: min{Lv + f, Mv - v} = 0.
pub fn quasi_variational_inequality_ty() -> Expr {
    prop()
}
/// `ImpulseControlPolicy : (Nat -> Real) -> Nat -> Prop`
/// Impulse control policy: specifies intervention times and sizes.
pub fn impulse_control_policy_ty() -> Expr {
    arrow(fn_ty(nat_ty(), real_ty()), arrow(nat_ty(), prop()))
}
/// `CertaintyEquivalence : Prop`
/// Certainty equivalence principle: replace unknown parameters with estimates in the control law.
pub fn certainty_equivalence_ty() -> Expr {
    prop()
}
/// `SelfTuningRegulator : Prop`
/// Self-tuning regulator: online parameter estimation + adaptive LQR.
pub fn self_tuning_regulator_ty() -> Expr {
    prop()
}
/// `QlearningConvergence : Real -> Real -> Prop`
/// Q-learning convergence theorem (Watkins-Dayan): Q_n → Q* a.s. under suitable step sizes.
pub fn q_learning_convergence_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), prop()))
}
/// `FittedValueIteration : Prop`
/// Fitted value iteration: approximate V* using a function approximator class.
pub fn fitted_value_iteration_ty() -> Expr {
    prop()
}
/// `JointPolicyConvergence : Nat -> Prop`
/// Joint policy convergence in multi-agent RL: policies converge to correlated equilibrium.
pub fn joint_policy_convergence_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `CorrelatedEquilibrium : ((Nat -> Nat -> Real) -> Prop) -> Prop`
/// Correlated equilibrium: joint distribution σ over action profiles such that no agent
/// benefits from deviating given σ.
pub fn correlated_equilibrium_ty() -> Expr {
    let joint_dist = fn_ty(fn_ty(nat_ty(), fn_ty(nat_ty(), real_ty())), prop());
    arrow(joint_dist, prop())
}
/// `RegretMinimisation : Real -> Prop`
/// Regret minimisation: no-regret algorithm guarantees cumulative regret = o(T).
pub fn regret_minimisation_ty() -> Expr {
    arrow(real_ty(), prop())
}
/// Populate an [`Environment`] with all stochastic-control axioms.
pub fn build_env(env: &mut Environment) {
    let axioms: &[(&str, Expr)] = &[
        ("MDP", mdp_ty()),
        ("Policy", policy_ty()),
        ("StochasticPolicy", stochastic_policy_ty()),
        ("ValueFunction", value_function_ty()),
        ("ActionValueFunction", action_value_function_ty()),
        ("BellmanOperator", bellman_operator_ty()),
        ("PolicyEvaluation", policy_evaluation_ty()),
        ("PolicyImprovement", policy_improvement_ty()),
        ("ValueIteration", value_iteration_ty()),
        ("HamiltonJacobiBellman", hjb_ty()),
        ("StochasticOptimalControl", stochastic_optimal_control_ty()),
        ("LinearQuadraticRegulator", lqr_ty()),
        ("RiccatiEquation", riccati_equation_ty()),
        ("SolveRiccati", solve_riccati_ty()),
        ("OptimalGainMatrix", optimal_gain_matrix_ty()),
        ("InfiniteHorizonLqr", infinite_horizon_lqr_ty()),
        ("QLearning", q_learning_ty()),
        ("SARSA", sarsa_ty()),
        ("PolicyGradient", policy_gradient_ty()),
        ("ActorCritic", actor_critic_ty()),
        ("ConvergenceRateRL", convergence_rate_rl_ty()),
        ("ZeroSumSDG", zero_sum_sdg_ty()),
        ("NashEquilibrium", nash_equilibrium_ty()),
        ("IsaacEquation", isaac_equation_ty()),
        ("PursuitEvasionGame", pursuit_evasion_game_ty()),
        ("StochasticLyapunov", stochastic_lyapunov_ty()),
        ("MeanSquareStability", mean_square_stability_ty()),
        ("AlmostSureStability", almost_sure_stability_ty()),
        ("ExponentialMSStability", exponential_ms_stability_ty()),
        ("POMDP", pomdp_ty()),
        ("BeliefState", belief_state_ty()),
        ("BeliefUpdate", belief_update_ty()),
        ("BeliefMDP", belief_mdp_ty()),
        ("QMDPApproximation", qmdp_approximation_ty()),
        ("EntropicRisk", entropic_risk_ty()),
        ("CVaROptimization", cvar_optimization_ty()),
        ("CoherentRiskMeasure", coherent_risk_measure_ty()),
        ("RiskSensitiveBellman", risk_sensitive_bellman_ty()),
        ("MinimaxControl", minimax_control_ty()),
        ("HInfinityStochastic", h_infinity_stochastic_ty()),
        ("AmbiguitySet", ambiguity_set_ty()),
        (
            "DistributionallyRobustMDP",
            distributionally_robust_mdp_ty(),
        ),
        ("MeanFieldGame", mean_field_game_ty()),
        ("McKeanVlasovSDE", mckean_vlasov_sde_ty()),
        ("MFGNashEquilibrium", mfg_nash_equilibrium_ty()),
        ("MFGConsistencyCondition", mfg_consistency_ty()),
        ("TwoPlayerZeroSumGame", two_player_zero_sum_game_ty()),
        ("ShapleyOperator", shapley_operator_ty()),
        ("AverageCostMDP", average_cost_mdp_ty()),
        ("PoissonEquation", poisson_equation_ty()),
        ("BiasFunction", bias_function_ty()),
        ("OptimalStopping", optimal_stopping_ty()),
        ("DynkinFormula", dynkin_formula_ty()),
        (
            "QuasiVariationalInequality",
            quasi_variational_inequality_ty(),
        ),
        ("ImpulseControlPolicy", impulse_control_policy_ty()),
        ("CertaintyEquivalence", certainty_equivalence_ty()),
        ("SelfTuningRegulator", self_tuning_regulator_ty()),
        ("QlearningConvergence", q_learning_convergence_ty()),
        ("FittedValueIteration", fitted_value_iteration_ty()),
        ("JointPolicyConvergence", joint_policy_convergence_ty()),
        ("CorrelatedEquilibrium", correlated_equilibrium_ty()),
        ("RegretMinimisation", regret_minimisation_ty()),
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
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_build_env() {
        let mut env = Environment::new();
        build_env(&mut env);
        assert!(env.get(&Name::str("MDP")).is_some());
        assert!(env.get(&Name::str("ValueFunction")).is_some());
        assert!(env.get(&Name::str("QLearning")).is_some());
        assert!(env.get(&Name::str("NashEquilibrium")).is_some());
        assert!(env.get(&Name::str("StochasticLyapunov")).is_some());
    }
    #[test]
    fn test_mdp_value_iteration() {
        let transitions = vec![
            vec![vec![1.0, 0.0], vec![0.0, 1.0]],
            vec![vec![0.0, 1.0], vec![0.0, 1.0]],
        ];
        let rewards = vec![vec![0.0, 0.0], vec![1.0, 1.0]];
        let mdp = MDP::new(2, 2, transitions, rewards, 0.9);
        let v = mdp.value_iteration(1e-8, 1000);
        assert!((v[1] - 10.0).abs() < 0.01, "V*(1) ≈ 10, got {}", v[1]);
        assert!((v[0] - 9.0).abs() < 0.01, "V*(0) ≈ 9, got {}", v[0]);
    }
    #[test]
    fn test_policy_evaluation() {
        let transitions = vec![
            vec![vec![1.0, 0.0], vec![0.0, 1.0]],
            vec![vec![0.0, 1.0], vec![0.0, 1.0]],
        ];
        let rewards = vec![vec![0.0, 0.0], vec![1.0, 1.0]];
        let mdp = MDP::new(2, 2, transitions, rewards, 0.9);
        let policy = vec![1, 0];
        let v = mdp.policy_evaluation(&policy, 1e-8, 1000);
        assert!(v[1] > v[0], "Good state should have higher value");
    }
    #[test]
    fn test_q_learning_update() {
        let mut agent = QLearning::new(3, 2, 0.5, 0.9);
        agent.update(0, 1, 1.0, 1);
        assert!((agent.q[0][1] - 0.5).abs() < 1e-12, "Q(0,1) should be 0.5");
    }
    #[test]
    fn test_sarsa_update() {
        let mut agent = SARSA::new(3, 2, 0.5, 0.9);
        agent.update(0, 1, 1.0, 1, 0);
        assert!(
            (agent.q[0][1] - 0.5).abs() < 1e-12,
            "SARSA Q(0,1) should be 0.5"
        );
    }
    #[test]
    fn test_policy_gradient_softmax() {
        let agent = PolicyGradient::new(2, 3, 0.01, 0.99);
        let pi = agent.softmax(0);
        let total: f64 = pi.iter().sum();
        assert!((total - 1.0).abs() < 1e-12, "softmax should sum to 1");
        assert!((pi[0] - 1.0 / 3.0).abs() < 1e-12);
    }
    #[test]
    fn test_actor_critic_update() {
        let mut ac = ActorCritic::new(2, 2, 0.1, 0.1, 0.9);
        let v0 = ac.expected_return(0);
        ac.update(0, 1, 1.0, 1);
        assert!(
            ac.expected_return(0) > v0,
            "critic value should increase after positive reward"
        );
    }
    #[test]
    fn test_pursuit_evasion() {
        let game = PursuitEvasionGame::new([0.0, 0.0], [3.0, 4.0], 2.0, 1.0);
        assert!((game.distance() - 5.0).abs() < 1e-12);
        assert!(game.pursuer_wins());
        assert!((game.capture_time_estimate() - 5.0).abs() < 1e-12);
    }
    #[test]
    fn test_stochastic_lyapunov_bound() {
        let lv = StochasticLyapunov::new(0.5, 0.1);
        assert!(lv.check(-0.9, 2.0));
        let v0 = 10.0;
        assert!((lv.ev_upper_bound(v0, 0.0) - v0).abs() < 1e-12);
    }
    #[test]
    fn test_exponential_ms_stability() {
        let stab = ExponentialMSStability::new(5.0, 0.5);
        assert!((stab.bound(0.0) - 5.0).abs() < 1e-12);
        assert!(stab.bound(10.0) < 0.1);
        assert!(stab.check(4.9, 0.0));
        assert!(!stab.check(5.1, 0.0));
    }
    #[test]
    fn test_riccati_solve() {
        let riccati = RiccatiEquation::new(
            vec![vec![-1.0]],
            vec![vec![1.0]],
            vec![vec![1.0]],
            vec![vec![1.0]],
        );
        let (p, k) = riccati.infinite_horizon_lqr();
        assert!(p[0][0] > 0.0, "P should be positive");
        assert!(k[0][0] > 0.0, "K should be positive");
    }
    #[test]
    fn test_build_env_new_axioms() {
        let mut env = Environment::new();
        build_env(&mut env);
        assert!(env.get(&Name::str("POMDP")).is_some());
        assert!(env.get(&Name::str("BeliefState")).is_some());
        assert!(env.get(&Name::str("BeliefUpdate")).is_some());
        assert!(env.get(&Name::str("BeliefMDP")).is_some());
        assert!(env.get(&Name::str("QMDPApproximation")).is_some());
        assert!(env.get(&Name::str("EntropicRisk")).is_some());
        assert!(env.get(&Name::str("CVaROptimization")).is_some());
        assert!(env.get(&Name::str("CoherentRiskMeasure")).is_some());
        assert!(env.get(&Name::str("RiskSensitiveBellman")).is_some());
        assert!(env.get(&Name::str("MinimaxControl")).is_some());
        assert!(env.get(&Name::str("HInfinityStochastic")).is_some());
        assert!(env.get(&Name::str("AmbiguitySet")).is_some());
        assert!(env.get(&Name::str("DistributionallyRobustMDP")).is_some());
        assert!(env.get(&Name::str("MeanFieldGame")).is_some());
        assert!(env.get(&Name::str("McKeanVlasovSDE")).is_some());
        assert!(env.get(&Name::str("MFGNashEquilibrium")).is_some());
        assert!(env.get(&Name::str("MFGConsistencyCondition")).is_some());
        assert!(env.get(&Name::str("TwoPlayerZeroSumGame")).is_some());
        assert!(env.get(&Name::str("ShapleyOperator")).is_some());
        assert!(env.get(&Name::str("AverageCostMDP")).is_some());
        assert!(env.get(&Name::str("PoissonEquation")).is_some());
        assert!(env.get(&Name::str("BiasFunction")).is_some());
        assert!(env.get(&Name::str("OptimalStopping")).is_some());
        assert!(env.get(&Name::str("DynkinFormula")).is_some());
        assert!(env.get(&Name::str("QuasiVariationalInequality")).is_some());
        assert!(env.get(&Name::str("ImpulseControlPolicy")).is_some());
        assert!(env.get(&Name::str("CertaintyEquivalence")).is_some());
        assert!(env.get(&Name::str("SelfTuningRegulator")).is_some());
        assert!(env.get(&Name::str("QlearningConvergence")).is_some());
        assert!(env.get(&Name::str("FittedValueIteration")).is_some());
        assert!(env.get(&Name::str("JointPolicyConvergence")).is_some());
        assert!(env.get(&Name::str("CorrelatedEquilibrium")).is_some());
        assert!(env.get(&Name::str("RegretMinimisation")).is_some());
    }
    #[test]
    fn test_belief_update_normalises() {
        let transitions = vec![vec![vec![1.0, 0.0]], vec![vec![0.0, 1.0]]];
        let observations = vec![vec![vec![0.9, 0.1]], vec![vec![0.2, 0.8]]];
        let pomdp = BeliefMDP::new(2, 1, 2, transitions, observations);
        let belief = vec![0.5, 0.5];
        let new_belief = pomdp.belief_update(&belief, 0, 0);
        let total: f64 = new_belief.iter().sum();
        assert!((total - 1.0).abs() < 1e-10, "belief should sum to 1");
        assert!((new_belief[0] - 0.45 / 0.55).abs() < 1e-10, "b'(0) ≈ 0.818");
    }
    #[test]
    fn test_qmdp_value() {
        let transitions = vec![vec![vec![1.0, 0.0]], vec![vec![0.0, 1.0]]];
        let observations = vec![vec![vec![1.0, 0.0]], vec![vec![0.0, 1.0]]];
        let pomdp = BeliefMDP::new(2, 1, 2, transitions, observations);
        let q_star = vec![vec![1.0], vec![5.0]];
        let belief = vec![0.5, 0.5];
        let v = pomdp.qmdp_value(&belief, &q_star);
        assert!((v - 3.0).abs() < 1e-12, "QMDP value should be 3.0");
    }
    #[test]
    fn test_cvar_basic() {
        let rsc = RiskSensitiveCost::new(0.5, 1.0);
        let samples: Vec<f64> = (1..=10).map(|x| x as f64).collect();
        let cvar = rsc.cvar(&samples);
        assert!(cvar >= 6.0, "CVaR_0.5 should be ≥ 6.0, got {cvar}");
    }
    #[test]
    fn test_entropic_risk() {
        let rsc = RiskSensitiveCost::new(0.5, 2.0);
        let samples = vec![1.0_f64; 100];
        let er = rsc.entropic_risk(&samples);
        assert!((er - 1.0).abs() < 1e-10, "ρ_θ(1) = 1, got {er}");
    }
    #[test]
    fn test_mean_field_game_solver() {
        let transitions = vec![
            vec![vec![1.0, 0.0], vec![0.0, 1.0]],
            vec![vec![0.0, 1.0], vec![0.0, 1.0]],
        ];
        let rewards = vec![vec![0.0, 0.0], vec![1.0, 1.0]];
        let solver = MeanFieldGameSolver::new(2, 2, transitions, rewards, 0.9, 1e-6, 1000);
        let (policy, mu) = solver.solve();
        assert_eq!(policy.len(), 2);
        let total: f64 = mu.iter().sum();
        assert!((total - 1.0).abs() < 1e-6, "MFG distribution sums to 1");
    }
    #[test]
    fn test_value_iteration_solver() {
        let transitions = vec![
            vec![vec![1.0, 0.0], vec![0.0, 1.0]],
            vec![vec![0.0, 1.0], vec![0.0, 1.0]],
        ];
        let rewards = vec![vec![0.0, 0.0], vec![1.0, 1.0]];
        let vi = ValueIteration::new(2, 2, transitions, rewards, 0.9);
        let (v, pi) = vi.run(1e-8, 1000);
        assert!((v[1] - 10.0).abs() < 0.01, "V*(1) ≈ 10");
        assert!((v[0] - 9.0).abs() < 0.01, "V*(0) ≈ 9");
        assert_eq!(pi.len(), 2);
    }
    #[test]
    fn test_q_from_v() {
        let transitions = vec![
            vec![vec![1.0, 0.0], vec![0.0, 1.0]],
            vec![vec![0.0, 1.0], vec![0.0, 1.0]],
        ];
        let rewards = vec![vec![0.0, 0.0], vec![1.0, 1.0]];
        let vi = ValueIteration::new(2, 2, transitions.clone(), rewards.clone(), 0.9);
        let (v, _) = vi.run(1e-8, 1000);
        let q = vi.q_from_v(&v);
        assert!((q[1][0] - 10.0).abs() < 0.1, "Q*(1,0) ≈ 10");
    }
    #[test]
    fn test_span_convergence() {
        let transitions = vec![
            vec![vec![1.0, 0.0], vec![0.0, 1.0]],
            vec![vec![0.0, 1.0], vec![0.0, 1.0]],
        ];
        let rewards = vec![vec![0.0, 0.0], vec![1.0, 1.0]];
        let vi = ValueIteration::new(2, 2, transitions, rewards, 0.9);
        let (v, _) = vi.run(1e-8, 1000);
        let span = vi.span(&v);
        assert!(
            span < 1e-6,
            "span should be near 0 at convergence, got {span}"
        );
    }
    #[test]
    fn test_q_learning_solver_update() {
        let mut solver = QLearningSolver::new(3, 2, 1.0, 0.9, 0.1);
        let prev_q = solver.q.clone();
        solver.update(0, 1, 1.0, 1);
        assert!(
            (solver.q[0][1] - prev_q[0][1]).abs() > 1e-12,
            "Q(0,1) should have been updated"
        );
    }
    #[test]
    fn test_q_learning_solver_greedy_policy() {
        let mut solver = QLearningSolver::new(2, 3, 0.5, 0.9, 0.0);
        solver.q[0][2] = 5.0;
        solver.q[1][1] = 3.0;
        let policy = solver.greedy_policy();
        assert_eq!(policy[0], 2, "state 0 should choose action 2");
        assert_eq!(policy[1], 1, "state 1 should choose action 1");
    }
    #[test]
    fn test_q_learning_solver_convergence_check() {
        let solver = QLearningSolver::new(2, 2, 0.5, 0.9, 0.1);
        let same_q = solver.q.clone();
        assert!(
            solver.has_converged(&same_q, 1e-10),
            "identical Q tables should be converged"
        );
        let mut diff_q = same_q.clone();
        diff_q[0][0] += 1.0;
        assert!(
            !solver.has_converged(&diff_q, 1e-10),
            "different Q tables should not be converged"
        );
    }
}
#[cfg(test)]
mod tests_stoch_control_ext {
    use super::*;
    #[test]
    fn test_sd_game_zero_sum() {
        let mut game = SDGame::zero_sum(1.0);
        assert!(game.is_zero_sum);
        assert!(game.saddle_point_exists());
        let isaacs = game.isaacs_equation();
        assert!(isaacs.contains("Isaacs"));
        game.set_value(2.5);
        assert_eq!(game.value_function, Some(2.5));
    }
    #[test]
    fn test_mean_field_game() {
        let mfg = MeanFieldGame::new(1000, 0.5);
        assert!((mfg.convergence_rate - 1.0 / (1000_f64).sqrt()).abs() < 1e-10);
        let desc = mfg.mfg_system_description();
        assert!(desc.contains("1000"));
        let poa = mfg.price_of_anarchy();
        assert!(poa > 1.0);
        let master = mfg.master_equation();
        assert!(master.contains("FP") || master.contains("∂_t"));
    }
    #[test]
    fn test_risk_sensitive_control() {
        let rsc = RiskSensitiveControl::risk_averse(0.5, 1.0);
        assert!(rsc.risk_parameter > 0.0);
        let ce = rsc.certainty_equivalent(1.0, 0.5);
        assert!((ce - 1.125).abs() < 1e-10);
        assert!(rsc.is_robust_control_connection());
        let crit = rsc.exponential_criterion();
        assert!(crit.contains("Risk-sensitive"));
    }
    #[test]
    fn test_hinf_control() {
        let hinf = HInfinityControl::new(2.0, 3, 2, 2);
        assert!(!hinf.is_feasible());
        let minimax = hinf.minimax_criterion();
        assert!(minimax.contains("H∞"));
        let riccati = hinf.game_riccati_equation();
        assert!(riccati.contains("ARE"));
    }
    #[test]
    fn test_ergodic_control() {
        let mut ec = ErgodicControl::new(3);
        ec.set_eigenvalue(1.5);
        assert_eq!(ec.long_run_cost, Some(1.5));
        let hjb = ec.ergodic_hjb();
        assert!(hjb.contains("ergodic"));
        let tp = ec.turnpike_property();
        assert!(tp.contains("Turnpike"));
    }
    #[test]
    fn test_pathwise_sde_euler() {
        let sde = PathwiseSDE::euler_maruyama("ax", "bx", 1.0, 10, 0.01);
        assert!((sde.strong_order() - 0.5).abs() < 1e-10);
        assert!((sde.weak_order() - 1.0).abs() < 1e-10);
        let path = sde.simulate_one_path();
        assert_eq!(path.len(), 11);
    }
    #[test]
    fn test_pathwise_sde_milstein() {
        let sde = PathwiseSDE::milstein("ax", "bx", 1.0, 5, 0.01);
        assert!((sde.strong_order() - 1.0).abs() < 1e-10);
    }
}
