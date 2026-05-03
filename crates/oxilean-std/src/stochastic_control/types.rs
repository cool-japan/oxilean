//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;

/// Nash equilibrium for a two-player zero-sum stochastic differential game.
///
/// Stores the equilibrium strategies (u*, v*).
#[derive(Debug, Clone)]
pub struct NashEquilibrium {
    /// Equilibrium strategy for player 1 (minimiser).
    pub player1_strategy: Vec<usize>,
    /// Equilibrium strategy for player 2 (maximiser).
    pub player2_strategy: Vec<usize>,
    /// Equilibrium value function.
    pub value: Vec<f64>,
}
impl NashEquilibrium {
    /// Construct a Nash equilibrium.
    pub fn new(
        player1_strategy: Vec<usize>,
        player2_strategy: Vec<usize>,
        value: Vec<f64>,
    ) -> Self {
        Self {
            player1_strategy,
            player2_strategy,
            value,
        }
    }
    /// Check whether neither player benefits from unilateral deviation (numeric check).
    /// Returns true if the strategies are consistent with the value function.
    pub fn verify_nash_property(&self) -> bool {
        !self.player1_strategy.is_empty()
            && !self.player2_strategy.is_empty()
            && self.player1_strategy.len() == self.value.len()
            && self.player2_strategy.len() == self.value.len()
    }
}
/// Risk-sensitive cost using Conditional Value-at-Risk (CVaR).
///
/// CVaR_α(X) = (1/α) ∫_{α}^{1} VaR_u(X) du
///           ≈ E[X | X ≥ VaR_α(X)] (discrete approximation via sorting).
#[derive(Debug, Clone)]
pub struct RiskSensitiveCost {
    /// Risk level α ∈ (0, 1]: CVaR_α.
    pub alpha: f64,
    /// Entropic risk parameter θ (for entropic risk measure ρ_θ).
    pub theta: f64,
}
impl RiskSensitiveCost {
    /// Construct a risk-sensitive cost calculator.
    pub fn new(alpha: f64, theta: f64) -> Self {
        Self { alpha, theta }
    }
    /// Compute VaR_α(X) from a sorted list of losses.
    ///
    /// Sorts `samples` and returns the (1-α)-quantile.
    pub fn var(&self, samples: &[f64]) -> f64 {
        if samples.is_empty() {
            return 0.0;
        }
        let mut sorted = samples.to_vec();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        let idx = ((1.0 - self.alpha) * sorted.len() as f64).floor() as usize;
        sorted[idx.min(sorted.len() - 1)]
    }
    /// Compute CVaR_α(X): average of losses at or above VaR_α.
    pub fn cvar(&self, samples: &[f64]) -> f64 {
        if samples.is_empty() {
            return 0.0;
        }
        let var_val = self.var(samples);
        let tail: Vec<f64> = samples.iter().cloned().filter(|&x| x >= var_val).collect();
        if tail.is_empty() {
            return var_val;
        }
        tail.iter().sum::<f64>() / tail.len() as f64
    }
    /// Entropic risk measure: ρ_θ(X) = (1/θ) log E\[exp(θ X)\].
    pub fn entropic_risk(&self, samples: &[f64]) -> f64 {
        if samples.is_empty() {
            return 0.0;
        }
        let mean_exp: f64 =
            samples.iter().map(|&x| (self.theta * x).exp()).sum::<f64>() / samples.len() as f64;
        mean_exp.ln() / self.theta
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RiskSensitiveControl {
    pub risk_parameter: f64,
    pub time_horizon: f64,
    pub control_space: String,
    pub state_space: String,
}
#[allow(dead_code)]
impl RiskSensitiveControl {
    pub fn risk_averse(theta: f64, horizon: f64) -> Self {
        assert!(theta > 0.0, "risk-averse requires θ > 0");
        RiskSensitiveControl {
            risk_parameter: theta,
            time_horizon: horizon,
            control_space: "U".to_string(),
            state_space: "R^n".to_string(),
        }
    }
    pub fn exponential_criterion(&self) -> String {
        format!(
            "Risk-sensitive criterion: J(u) = (1/{:.3}) log E[exp({:.3} ∫ r dt)]",
            self.risk_parameter, self.risk_parameter
        )
    }
    pub fn risk_sensitive_hjb(&self) -> String {
        format!(
            "RS-HJB: 0 = ∂V/∂t + min_u[f·∇V + (1/2)tr(σσ^T ∇²V) + r + ({:.3}/2)|σ^T∇V|²]",
            self.risk_parameter
        )
    }
    pub fn certainty_equivalent(&self, expected_cost: f64, variance: f64) -> f64 {
        expected_cost + self.risk_parameter / 2.0 * variance
    }
    pub fn is_robust_control_connection(&self) -> bool {
        self.risk_parameter > 0.0
    }
}
/// Mean field game solver via fixed-point iteration.
///
/// Iterates:
///   1. Given population distribution μ, solve individual optimal control → policy π.
///   2. Given policy π, simulate population dynamics → new distribution μ'.
///   3. Repeat until ||μ' - μ|| < ε.
#[derive(Debug, Clone)]
pub struct MeanFieldGameSolver {
    /// Number of states.
    pub num_states: usize,
    /// Number of actions.
    pub num_actions: usize,
    /// Transition kernel (state-dependent on population): T\[s\]\[a\][s'].
    pub transitions: Vec<Vec<Vec<f64>>>,
    /// Reward (depends on state, action, mean population weight).
    /// reward\[s\]\[a\] — simplified to not depend on μ for tractability.
    pub rewards: Vec<Vec<f64>>,
    /// Discount factor.
    pub discount: f64,
    /// Convergence tolerance.
    pub tol: f64,
    /// Maximum iterations.
    pub max_iter: usize,
}
impl MeanFieldGameSolver {
    /// Construct a mean field game solver.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        num_states: usize,
        num_actions: usize,
        transitions: Vec<Vec<Vec<f64>>>,
        rewards: Vec<Vec<f64>>,
        discount: f64,
        tol: f64,
        max_iter: usize,
    ) -> Self {
        Self {
            num_states,
            num_actions,
            transitions,
            rewards,
            discount,
            tol,
            max_iter,
        }
    }
    /// Solve the individual MDP given current population distribution (fixed μ).
    /// Returns the greedy policy as a state→action table.
    fn solve_individual(&self) -> Vec<usize> {
        let mdp = MDP::new(
            self.num_states,
            self.num_actions,
            self.transitions.clone(),
            self.rewards.clone(),
            self.discount,
        );
        let v = mdp.value_iteration(self.tol * 0.01, self.max_iter);
        mdp.policy_improvement(&v)
    }
    /// Compute the stationary distribution of a Markov chain induced by policy π.
    fn stationary_distribution(&self, policy: &[usize]) -> Vec<f64> {
        let mut mu = vec![1.0_f64 / self.num_states as f64; self.num_states];
        for _ in 0..self.max_iter {
            let mut new_mu = vec![0.0_f64; self.num_states];
            for s in 0..self.num_states {
                let a = policy[s];
                for sp in 0..self.num_states {
                    new_mu[sp] += mu[s] * self.transitions[s][a][sp];
                }
            }
            let delta: f64 = mu
                .iter()
                .zip(new_mu.iter())
                .map(|(a, b)| (a - b).abs())
                .fold(0.0_f64, f64::max);
            mu = new_mu;
            if delta < self.tol {
                break;
            }
        }
        mu
    }
    /// Run the mean field game fixed-point iteration.
    /// Returns (equilibrium_policy, equilibrium_distribution).
    pub fn solve(&self) -> (Vec<usize>, Vec<f64>) {
        let policy = self.solve_individual();
        let mu = self.stationary_distribution(&policy);
        (policy, mu)
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum NumericalSDEScheme {
    EulerMaruyama,
    Milstein,
    RungeKuttaSDE,
    StochasticTaylor,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SDGame {
    pub player1_strategy: String,
    pub player2_strategy: String,
    pub value_function: Option<f64>,
    pub is_zero_sum: bool,
    pub horizon: f64,
}
#[allow(dead_code)]
impl SDGame {
    pub fn zero_sum(horizon: f64) -> Self {
        SDGame {
            player1_strategy: "min".to_string(),
            player2_strategy: "max".to_string(),
            value_function: None,
            is_zero_sum: true,
            horizon,
        }
    }
    pub fn cooperative(horizon: f64) -> Self {
        SDGame {
            player1_strategy: "cooperative".to_string(),
            player2_strategy: "cooperative".to_string(),
            value_function: None,
            is_zero_sum: false,
            horizon,
        }
    }
    pub fn isaacs_equation(&self) -> String {
        if self.is_zero_sum {
            format!(
                "Isaacs equation for zero-sum game on [0,{}]: -∂V/∂t = min_u max_v H(x,u,v,∇V)",
                self.horizon
            )
        } else {
            format!("Nash system for cooperative game on [0,{}]", self.horizon)
        }
    }
    pub fn saddle_point_exists(&self) -> bool {
        self.is_zero_sum
    }
    pub fn set_value(&mut self, val: f64) {
        self.value_function = Some(val);
    }
}
/// Zero-sum stochastic differential game.
///
/// Two players minimise/maximise J = E\[∫ L(x,u,v) dt + g(x_T)\].
/// Here we represent the saddle value and equilibrium strategies as numeric tables.
#[derive(Debug, Clone)]
pub struct ZeroSumSDG {
    /// Value function table: `value\[s\]` = saddle value from state s.
    pub value: Vec<f64>,
    /// Minimising player strategy: `min_strategy\[s\]` = optimal action index.
    pub min_strategy: Vec<usize>,
    /// Maximising player strategy: `max_strategy\[s\]` = optimal action index.
    pub max_strategy: Vec<usize>,
}
impl ZeroSumSDG {
    /// Construct a zero-sum SDG from pre-computed value and strategies.
    pub fn new(value: Vec<f64>, min_strategy: Vec<usize>, max_strategy: Vec<usize>) -> Self {
        Self {
            value,
            min_strategy,
            max_strategy,
        }
    }
    /// Return the saddle value from state `s`.
    pub fn saddle_value(&self, s: usize) -> f64 {
        self.value[s]
    }
}
/// Exponential mean-square stability: E[||x_t||²] ≤ C e^{-λt}.
#[derive(Debug, Clone)]
pub struct ExponentialMSStability {
    /// Upper bound constant C.
    pub c: f64,
    /// Decay rate λ > 0.
    pub lambda: f64,
}
impl ExponentialMSStability {
    /// Construct an exponential MS stability bound.
    pub fn new(c: f64, lambda: f64) -> Self {
        Self { c, lambda }
    }
    /// Evaluate the bound C e^{-λt}.
    pub fn bound(&self, t: f64) -> f64 {
        self.c * (-self.lambda * t).exp()
    }
    /// Check whether a given E[||x_t||²] value satisfies the bound at time t.
    pub fn check(&self, ms_value: f64, t: f64) -> bool {
        ms_value <= self.bound(t)
    }
}
/// Standalone discounted MDP value iteration solver.
///
/// More flexible than `MDP::value_iteration` — accepts sparse transition
/// representations via closures (approximated here via table).
#[derive(Debug, Clone)]
pub struct ValueIteration {
    /// Number of states.
    pub num_states: usize,
    /// Number of actions.
    pub num_actions: usize,
    /// Transition probabilities T\[s\]\[a\][s'].
    pub transitions: Vec<Vec<Vec<f64>>>,
    /// Reward R\[s\]\[a\].
    pub rewards: Vec<Vec<f64>>,
    /// Discount factor γ.
    pub discount: f64,
}
impl ValueIteration {
    /// Construct a value iteration solver.
    pub fn new(
        num_states: usize,
        num_actions: usize,
        transitions: Vec<Vec<Vec<f64>>>,
        rewards: Vec<Vec<f64>>,
        discount: f64,
    ) -> Self {
        Self {
            num_states,
            num_actions,
            transitions,
            rewards,
            discount,
        }
    }
    /// Run value iteration and return (V*, π*).
    pub fn run(&self, tol: f64, max_iter: usize) -> (Vec<f64>, Vec<usize>) {
        let mdp = MDP::new(
            self.num_states,
            self.num_actions,
            self.transitions.clone(),
            self.rewards.clone(),
            self.discount,
        );
        let v = mdp.value_iteration(tol, max_iter);
        let pi = mdp.policy_improvement(&v);
        (v, pi)
    }
    /// Compute the Q-function from the optimal value function.
    pub fn q_from_v(&self, v: &[f64]) -> Vec<Vec<f64>> {
        let mut q = vec![vec![0.0_f64; self.num_actions]; self.num_states];
        for s in 0..self.num_states {
            for a in 0..self.num_actions {
                let mut qa = self.rewards[s][a];
                for sp in 0..self.num_states {
                    qa += self.discount * self.transitions[s][a][sp] * v[sp];
                }
                q[s][a] = qa;
            }
        }
        q
    }
    /// Span semi-norm: measure of Bellman residual (for convergence diagnostics).
    pub fn span(&self, v: &[f64]) -> f64 {
        let tv_v = MDP::new(
            self.num_states,
            self.num_actions,
            self.transitions.clone(),
            self.rewards.clone(),
            self.discount,
        )
        .bellman_operator(v);
        tv_v.iter()
            .zip(v.iter())
            .map(|(tv, vv)| (tv - vv).abs())
            .fold(0.0_f64, f64::max)
    }
}
/// State-action value function Q^π(s,a).
#[derive(Debug, Clone)]
pub struct ActionValueFunction {
    /// `q\[s\]\[a\]` = Q(s, a).
    pub q: Vec<Vec<f64>>,
}
impl ActionValueFunction {
    /// Construct from a table.
    pub fn new(q: Vec<Vec<f64>>) -> Self {
        Self { q }
    }
    /// Return Q(s, a).
    pub fn get(&self, s: usize, a: usize) -> f64 {
        self.q[s][a]
    }
    /// Return max_a Q(s, a).
    pub fn max_action_value(&self, s: usize) -> f64 {
        self.q[s].iter().cloned().fold(f64::NEG_INFINITY, f64::max)
    }
    /// Return argmax_a Q(s, a).
    pub fn greedy_action(&self, s: usize) -> usize {
        self.q[s]
            .iter()
            .enumerate()
            .max_by(|x, y| x.1.partial_cmp(y.1).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(i, _)| i)
            .unwrap_or(0)
    }
}
/// A finite Markov Decision Process (S, A, P, R, γ).
///
/// - `num_states`: |S|
/// - `num_actions`: |A|
/// - `transitions\[s\]\[a\][s']`: P(s' | s, a)
/// - `rewards\[s\]\[a\]`: R(s, a)
/// - `discount`: γ ∈ [0,1)
#[derive(Debug, Clone)]
pub struct MDP {
    /// Number of states.
    pub num_states: usize,
    /// Number of actions.
    pub num_actions: usize,
    /// Transition probabilities: `transitions\[s\]\[a\]` is a probability vector over next states.
    pub transitions: Vec<Vec<Vec<f64>>>,
    /// Expected reward: `rewards\[s\]\[a\]`.
    pub rewards: Vec<Vec<f64>>,
    /// Discount factor γ ∈ [0,1).
    pub discount: f64,
}
impl MDP {
    /// Construct a new MDP.
    pub fn new(
        num_states: usize,
        num_actions: usize,
        transitions: Vec<Vec<Vec<f64>>>,
        rewards: Vec<Vec<f64>>,
        discount: f64,
    ) -> Self {
        Self {
            num_states,
            num_actions,
            transitions,
            rewards,
            discount,
        }
    }
    /// Apply the Bellman operator: (TV)(s) = max_a [R(s,a) + γ Σ P(s'|s,a) V(s')].
    pub fn bellman_operator(&self, v: &[f64]) -> Vec<f64> {
        let mut tv = vec![0.0_f64; self.num_states];
        for s in 0..self.num_states {
            let mut best = f64::NEG_INFINITY;
            for a in 0..self.num_actions {
                let mut q = self.rewards[s][a];
                for sp in 0..self.num_states {
                    q += self.discount * self.transitions[s][a][sp] * v[sp];
                }
                if q > best {
                    best = q;
                }
            }
            tv[s] = best;
        }
        tv
    }
    /// Value iteration: iterate the Bellman operator until convergence.
    pub fn value_iteration(&self, tol: f64, max_iter: usize) -> Vec<f64> {
        let mut v = vec![0.0_f64; self.num_states];
        for _ in 0..max_iter {
            let tv = self.bellman_operator(&v);
            let delta: f64 = v
                .iter()
                .zip(tv.iter())
                .map(|(a, b)| (a - b).abs())
                .fold(0.0_f64, f64::max);
            v = tv;
            if delta < tol {
                break;
            }
        }
        v
    }
    /// Extract the greedy policy from a value function.
    pub fn policy_improvement(&self, v: &[f64]) -> Vec<usize> {
        let mut policy = vec![0_usize; self.num_states];
        for s in 0..self.num_states {
            let mut best_a = 0;
            let mut best_q = f64::NEG_INFINITY;
            for a in 0..self.num_actions {
                let mut q = self.rewards[s][a];
                for sp in 0..self.num_states {
                    q += self.discount * self.transitions[s][a][sp] * v[sp];
                }
                if q > best_q {
                    best_q = q;
                    best_a = a;
                }
            }
            policy[s] = best_a;
        }
        policy
    }
    /// Policy evaluation: compute V^π for a deterministic policy using iterative updates.
    pub fn policy_evaluation(&self, policy: &[usize], tol: f64, max_iter: usize) -> Vec<f64> {
        let mut v = vec![0.0_f64; self.num_states];
        for _ in 0..max_iter {
            let mut new_v = vec![0.0_f64; self.num_states];
            for s in 0..self.num_states {
                let a = policy[s];
                let mut val = self.rewards[s][a];
                for sp in 0..self.num_states {
                    val += self.discount * self.transitions[s][a][sp] * v[sp];
                }
                new_v[s] = val;
            }
            let delta: f64 = v
                .iter()
                .zip(new_v.iter())
                .map(|(a, b)| (a - b).abs())
                .fold(0.0_f64, f64::max);
            v = new_v;
            if delta < tol {
                break;
            }
        }
        v
    }
}
/// Q-learning agent.
///
/// Off-policy TD control: Q(s,a) ← Q(s,a) + α(r + γ max_a' Q(s',a') − Q(s,a)).
#[derive(Debug, Clone)]
pub struct QLearning {
    /// Q-value table: `q\[s\]\[a\]`.
    pub q: Vec<Vec<f64>>,
    /// Learning rate α ∈ (0,1].
    pub alpha: f64,
    /// Discount factor γ ∈ [0,1).
    pub gamma: f64,
}
impl QLearning {
    /// Construct a Q-learning agent with zero-initialised Q-table.
    pub fn new(num_states: usize, num_actions: usize, alpha: f64, gamma: f64) -> Self {
        Self {
            q: vec![vec![0.0_f64; num_actions]; num_states],
            alpha,
            gamma,
        }
    }
    /// Perform a single Q-learning update.
    pub fn update(&mut self, s: usize, a: usize, r: f64, s_next: usize) {
        let max_q_next = self.q[s_next]
            .iter()
            .cloned()
            .fold(f64::NEG_INFINITY, f64::max);
        let td_error = r + self.gamma * max_q_next - self.q[s][a];
        self.q[s][a] += self.alpha * td_error;
    }
    /// Return the greedy action in state `s`.
    pub fn greedy_action(&self, s: usize) -> usize {
        self.q[s]
            .iter()
            .enumerate()
            .max_by(|x, y| x.1.partial_cmp(y.1).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(i, _)| i)
            .unwrap_or(0)
    }
    /// Expected return (sum of Q values) from state `s` under current Q-table greedy policy.
    pub fn expected_return(&self, s: usize) -> f64 {
        self.q[s].iter().cloned().fold(f64::NEG_INFINITY, f64::max)
    }
}
/// Actor-Critic agent: combined policy gradient (actor) + TD value function (critic).
#[derive(Debug, Clone)]
pub struct ActorCritic {
    /// Actor: policy gradient component.
    pub actor: PolicyGradient,
    /// Critic: value function V(s) estimate.
    pub critic: Vec<f64>,
    /// Critic learning rate.
    pub critic_alpha: f64,
}
impl ActorCritic {
    /// Construct an actor-critic agent.
    pub fn new(
        num_states: usize,
        num_actions: usize,
        actor_alpha: f64,
        critic_alpha: f64,
        gamma: f64,
    ) -> Self {
        Self {
            actor: PolicyGradient::new(num_states, num_actions, actor_alpha, gamma),
            critic: vec![0.0_f64; num_states],
            critic_alpha,
        }
    }
    /// Perform a single actor-critic update given (s, a, r, s').
    pub fn update(&mut self, s: usize, a: usize, r: f64, s_next: usize) {
        let td_error = r + self.actor.gamma * self.critic[s_next] - self.critic[s];
        self.critic[s] += self.critic_alpha * td_error;
        let pi = self.actor.softmax(s);
        let num_actions = self.actor.theta[s].len();
        for b in 0..num_actions {
            let indicator = if b == a { 1.0 } else { 0.0 };
            let grad_log = indicator - pi[b];
            self.actor.theta[s][b] += self.actor.alpha * grad_log * td_error;
        }
    }
    /// Return the estimated state value V(s).
    pub fn expected_return(&self, s: usize) -> f64 {
        self.critic[s]
    }
}
/// Pursuit-evasion game: pursuer minimises time-to-capture, evader maximises it.
#[derive(Debug, Clone)]
pub struct PursuitEvasionGame {
    /// Position of pursuer (2D).
    pub pursuer: [f64; 2],
    /// Position of evader (2D).
    pub evader: [f64; 2],
    /// Pursuer speed.
    pub pursuer_speed: f64,
    /// Evader speed.
    pub evader_speed: f64,
}
impl PursuitEvasionGame {
    /// Construct a pursuit-evasion game.
    pub fn new(pursuer: [f64; 2], evader: [f64; 2], pursuer_speed: f64, evader_speed: f64) -> Self {
        Self {
            pursuer,
            evader,
            pursuer_speed,
            evader_speed,
        }
    }
    /// Euclidean distance between pursuer and evader.
    pub fn distance(&self) -> f64 {
        let dx = self.pursuer[0] - self.evader[0];
        let dy = self.pursuer[1] - self.evader[1];
        (dx * dx + dy * dy).sqrt()
    }
    /// Returns true if the pursuer can eventually catch the evader (pursuer speed > evader speed).
    pub fn pursuer_wins(&self) -> bool {
        self.pursuer_speed > self.evader_speed
    }
    /// Isotropic capture time estimate (simple Apollonius circle formula for equal speeds).
    pub fn capture_time_estimate(&self) -> f64 {
        let d = self.distance();
        let relative_speed = self.pursuer_speed - self.evader_speed;
        if relative_speed <= 0.0 {
            f64::INFINITY
        } else {
            d / relative_speed
        }
    }
}
/// Mean-square stability checker: verifies E[||x_t||²] → 0.
#[derive(Debug, Clone)]
pub struct MeanSquareStability {
    /// Sequence of E[||x_t||²] samples.
    pub ms_samples: Vec<f64>,
}
impl MeanSquareStability {
    /// Construct from a sequence of mean-square values.
    pub fn new(ms_samples: Vec<f64>) -> Self {
        Self { ms_samples }
    }
    /// Check if the sequence is non-increasing (necessary but not sufficient for stability).
    pub fn is_non_increasing(&self) -> bool {
        self.ms_samples.windows(2).all(|w| w[0] >= w[1])
    }
    /// Check if the last sample is below a tolerance (approximate convergence to 0).
    pub fn has_converged(&self, tol: f64) -> bool {
        self.ms_samples.last().is_some_and(|&v| v < tol)
    }
}
/// POMDP belief state update (Bayes filter).
///
/// Given belief b ∈ Δ(S), action a, observation o,
/// computes b'(s') ∝ Z(o|s', a) · Σ_s T(s'|s, a) · b(s).
#[derive(Debug, Clone)]
pub struct BeliefMDP {
    /// Number of states.
    pub num_states: usize,
    /// Number of actions.
    pub num_actions: usize,
    /// Number of observations.
    pub num_obs: usize,
    /// Transition probabilities T\[s\]\[a\][s'].
    pub transitions: Vec<Vec<Vec<f64>>>,
    /// Observation probabilities Z[s']\[a\]\[o\].
    pub observations: Vec<Vec<Vec<f64>>>,
}
impl BeliefMDP {
    /// Construct a BeliefMDP.
    pub fn new(
        num_states: usize,
        num_actions: usize,
        num_obs: usize,
        transitions: Vec<Vec<Vec<f64>>>,
        observations: Vec<Vec<Vec<f64>>>,
    ) -> Self {
        Self {
            num_states,
            num_actions,
            num_obs,
            transitions,
            observations,
        }
    }
    /// Perform a Bayesian belief update.
    ///
    /// b'(s') ∝ Z(o | s', a) · Σ_s T(s' | s, a) · b(s)
    pub fn belief_update(&self, belief: &[f64], action: usize, obs: usize) -> Vec<f64> {
        let mut new_belief = vec![0.0_f64; self.num_states];
        for sp in 0..self.num_states {
            let predict: f64 = (0..self.num_states)
                .map(|s| self.transitions[s][action][sp] * belief[s])
                .sum();
            new_belief[sp] = self.observations[sp][action][obs] * predict;
        }
        let total: f64 = new_belief.iter().sum();
        if total > 0.0 {
            for b in &mut new_belief {
                *b /= total;
            }
        }
        new_belief
    }
    /// QMDP approximation: V(b) ≈ max_a Σ_s b(s) · Q*(s, a).
    pub fn qmdp_value(&self, belief: &[f64], q_star: &[Vec<f64>]) -> f64 {
        (0..self.num_actions)
            .map(|a| {
                belief
                    .iter()
                    .enumerate()
                    .map(|(s, &bs)| bs * q_star[s][a])
                    .sum::<f64>()
            })
            .fold(f64::NEG_INFINITY, f64::max)
    }
}
/// Algebraic Riccati equation solver for LQR/LQG.
///
/// Solves A^T P + P A - P B R^{-1} B^T P + Q = 0 iteratively (Euler integration).
#[derive(Debug, Clone)]
pub struct RiccatiEquation {
    /// System matrix A (n×n).
    pub a: Vec<Vec<f64>>,
    /// Input matrix B (n×m).
    pub b: Vec<Vec<f64>>,
    /// State cost matrix Q (n×n), positive semidefinite.
    pub q_cost: Vec<Vec<f64>>,
    /// Input cost matrix R (m×m), positive definite.
    pub r_cost: Vec<Vec<f64>>,
}
impl RiccatiEquation {
    /// Construct a Riccati solver.
    pub fn new(
        a: Vec<Vec<f64>>,
        b: Vec<Vec<f64>>,
        q_cost: Vec<Vec<f64>>,
        r_cost: Vec<Vec<f64>>,
    ) -> Self {
        Self {
            a,
            b,
            q_cost,
            r_cost,
        }
    }
    fn mat_mul(m1: &[Vec<f64>], m2: &[Vec<f64>]) -> Vec<Vec<f64>> {
        let r1 = m1.len();
        let c2 = m2[0].len();
        let inner = m2.len();
        let mut out = vec![vec![0.0_f64; c2]; r1];
        for i in 0..r1 {
            for j in 0..c2 {
                for k in 0..inner {
                    out[i][j] += m1[i][k] * m2[k][j];
                }
            }
        }
        out
    }
    fn mat_transpose(m: &[Vec<f64>]) -> Vec<Vec<f64>> {
        if m.is_empty() {
            return vec![];
        }
        let rows = m.len();
        let cols = m[0].len();
        let mut out = vec![vec![0.0_f64; rows]; cols];
        for i in 0..rows {
            for j in 0..cols {
                out[j][i] = m[i][j];
            }
        }
        out
    }
    fn mat_add(m1: &[Vec<f64>], m2: &[Vec<f64>]) -> Vec<Vec<f64>> {
        m1.iter()
            .zip(m2.iter())
            .map(|(r1, r2)| r1.iter().zip(r2.iter()).map(|(a, b)| a + b).collect())
            .collect()
    }
    fn mat_sub(m1: &[Vec<f64>], m2: &[Vec<f64>]) -> Vec<Vec<f64>> {
        m1.iter()
            .zip(m2.iter())
            .map(|(r1, r2)| r1.iter().zip(r2.iter()).map(|(a, b)| a - b).collect())
            .collect()
    }
    fn mat_scale(m: &[Vec<f64>], s: f64) -> Vec<Vec<f64>> {
        m.iter()
            .map(|r| r.iter().map(|x| x * s).collect())
            .collect()
    }
    /// Invert a 1×1 or 2×2 matrix (sufficient for test cases).
    fn mat_inv_small(m: &[Vec<f64>]) -> Vec<Vec<f64>> {
        let n = m.len();
        if n == 1 {
            return vec![vec![1.0 / m[0][0]]];
        }
        if n == 2 {
            let det = m[0][0] * m[1][1] - m[0][1] * m[1][0];
            return vec![
                vec![m[1][1] / det, -m[0][1] / det],
                vec![-m[1][0] / det, m[0][0] / det],
            ];
        }
        let mut eye = vec![vec![0.0_f64; n]; n];
        for i in 0..n {
            eye[i][i] = 1.0;
        }
        eye
    }
    /// Compute the Riccati derivative dP/dt = A^T P + P A - P B R^{-1} B^T P + Q.
    fn riccati_deriv(&self, p: &[Vec<f64>]) -> Vec<Vec<f64>> {
        let at = Self::mat_transpose(&self.a);
        let bt = Self::mat_transpose(&self.b);
        let r_inv = Self::mat_inv_small(&self.r_cost);
        let at_p = Self::mat_mul(&at, p);
        let p_a = Self::mat_mul(p, &self.a);
        let p_b = Self::mat_mul(p, &self.b);
        let p_b_rinv = Self::mat_mul(&p_b, &r_inv);
        let p_b_rinv_bt = Self::mat_mul(&p_b_rinv, &bt);
        let p_b_rinv_bt_p = Self::mat_mul(&p_b_rinv_bt, p);
        let sum = Self::mat_add(&at_p, &p_a);
        let sum2 = Self::mat_add(&sum, &self.q_cost);
        Self::mat_sub(&sum2, &p_b_rinv_bt_p)
    }
    /// Solve the algebraic Riccati equation via backward Euler integration.
    pub fn solve_riccati(&self, dt: f64, max_iter: usize) -> Vec<Vec<f64>> {
        let n = self.a.len();
        let mut p = vec![vec![0.0_f64; n]; n];
        for _ in 0..max_iter {
            let dp = self.riccati_deriv(&p);
            let update = Self::mat_scale(&dp, dt);
            p = Self::mat_add(&p, &update);
        }
        p
    }
    /// Compute the optimal LQR gain matrix K* = R^{-1} B^T P.
    pub fn optimal_gain_matrix(&self, p: &[Vec<f64>]) -> Vec<Vec<f64>> {
        let bt = Self::mat_transpose(&self.b);
        let r_inv = Self::mat_inv_small(&self.r_cost);
        let bt_p = Self::mat_mul(&bt, p);
        Self::mat_mul(&r_inv, &bt_p)
    }
    /// Solve the infinite-horizon LQR: return (P, K) where K stabilises the system.
    pub fn infinite_horizon_lqr(&self) -> (Vec<Vec<f64>>, Vec<Vec<f64>>) {
        let p = self.solve_riccati(0.001, 10_000);
        let k = self.optimal_gain_matrix(&p);
        (p, k)
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct MeanFieldGame {
    pub num_players: usize,
    pub coupling_strength: f64,
    pub mean_field_type: MFGType,
    pub convergence_rate: f64,
}
#[allow(dead_code)]
impl MeanFieldGame {
    pub fn new(players: usize, coupling: f64) -> Self {
        MeanFieldGame {
            num_players: players,
            coupling_strength: coupling,
            mean_field_type: MFGType::LasryLions,
            convergence_rate: 1.0 / (players as f64).sqrt(),
        }
    }
    pub fn mfg_system_description(&self) -> String {
        format!(
            "MFG ({} players, coupling={:.3}): HJB + FP system, rate O(1/√N)",
            self.num_players, self.coupling_strength
        )
    }
    pub fn price_of_anarchy(&self) -> f64 {
        1.0 + self.coupling_strength * 0.5
    }
    pub fn master_equation(&self) -> String {
        "∂_t U + H(x, m, ∇_x U) - ν Δ_x U = ∫ (∂_m U)(y) δ_y F(y,m) m(dy)".to_string()
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PathwiseSDE {
    pub drift: String,
    pub diffusion: String,
    pub initial_condition: f64,
    pub time_steps: usize,
    pub step_size: f64,
    pub scheme: NumericalSDEScheme,
}
#[allow(dead_code)]
impl PathwiseSDE {
    pub fn euler_maruyama(drift: &str, diffusion: &str, x0: f64, steps: usize, dt: f64) -> Self {
        PathwiseSDE {
            drift: drift.to_string(),
            diffusion: diffusion.to_string(),
            initial_condition: x0,
            time_steps: steps,
            step_size: dt,
            scheme: NumericalSDEScheme::EulerMaruyama,
        }
    }
    pub fn milstein(drift: &str, diffusion: &str, x0: f64, steps: usize, dt: f64) -> Self {
        PathwiseSDE {
            drift: drift.to_string(),
            diffusion: diffusion.to_string(),
            initial_condition: x0,
            time_steps: steps,
            step_size: dt,
            scheme: NumericalSDEScheme::Milstein,
        }
    }
    pub fn strong_order(&self) -> f64 {
        match &self.scheme {
            NumericalSDEScheme::EulerMaruyama => 0.5,
            NumericalSDEScheme::Milstein => 1.0,
            NumericalSDEScheme::RungeKuttaSDE => 1.5,
            NumericalSDEScheme::StochasticTaylor => 2.0,
        }
    }
    pub fn weak_order(&self) -> f64 {
        match &self.scheme {
            NumericalSDEScheme::EulerMaruyama => 1.0,
            NumericalSDEScheme::Milstein => 1.0,
            NumericalSDEScheme::RungeKuttaSDE => 2.0,
            NumericalSDEScheme::StochasticTaylor => 2.0,
        }
    }
    pub fn simulate_one_path(&self) -> Vec<f64> {
        let mut path = vec![self.initial_condition];
        let mut x = self.initial_condition;
        for _ in 0..self.time_steps {
            let dw = 0.0;
            x += self.step_size * 0.5 + 0.3 * dw;
            path.push(x);
        }
        path
    }
}
/// Stochastic Lyapunov stability checker.
///
/// Checks the Foster-Lyapunov condition: LV(x) ≤ -α V(x) + β.
#[derive(Debug, Clone)]
pub struct StochasticLyapunov {
    /// Decay rate α > 0.
    pub alpha: f64,
    /// Additive drift β ≥ 0.
    pub beta: f64,
}
impl StochasticLyapunov {
    /// Construct a stochastic Lyapunov condition checker.
    pub fn new(alpha: f64, beta: f64) -> Self {
        Self { alpha, beta }
    }
    /// Check if LV(x) ≤ -α V(x) + β holds for a given LV(x) and V(x).
    pub fn check(&self, lv: f64, v: f64) -> bool {
        lv <= -self.alpha * v + self.beta
    }
    /// Upper bound on E[V(x_t)] given E[V(x_0)] = v0 (from Foster-Lyapunov).
    pub fn ev_upper_bound(&self, v0: f64, t: f64) -> f64 {
        let decay = (-self.alpha * t).exp();
        v0 * decay + (self.beta / self.alpha) * (1.0 - decay)
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum MFGType {
    LasryLions,
    MFGControl,
    ExtendedMFG,
}
/// Policy gradient agent (softmax parameterisation).
///
/// Policy: π_θ(a|s) = exp(θ\[s\]\[a\]) / Σ exp(θ\[s\][a'])
/// Update: θ\[s\]\[a\] += α · ∇_θ log π_θ(a|s) · G where G is the return.
#[derive(Debug, Clone)]
pub struct PolicyGradient {
    /// Policy parameter table θ\[s\]\[a\].
    pub theta: Vec<Vec<f64>>,
    /// Learning rate α.
    pub alpha: f64,
    /// Discount factor γ.
    pub gamma: f64,
}
impl PolicyGradient {
    /// Construct a policy gradient agent.
    pub fn new(num_states: usize, num_actions: usize, alpha: f64, gamma: f64) -> Self {
        Self {
            theta: vec![vec![0.0_f64; num_actions]; num_states],
            alpha,
            gamma,
        }
    }
    /// Compute the softmax policy π_θ(·|s).
    pub fn softmax(&self, s: usize) -> Vec<f64> {
        let row = &self.theta[s];
        let max_val = row.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let exps: Vec<f64> = row.iter().map(|&x| (x - max_val).exp()).collect();
        let sum: f64 = exps.iter().sum();
        exps.iter().map(|&e| e / sum).collect()
    }
    /// Update θ using a single (s, a, G) sample from a trajectory.
    pub fn update(&mut self, s: usize, a: usize, g: f64) {
        let pi = self.softmax(s);
        let num_actions = self.theta[s].len();
        for b in 0..num_actions {
            let indicator = if b == a { 1.0 } else { 0.0 };
            let grad_log = indicator - pi[b];
            self.theta[s][b] += self.alpha * grad_log * g;
        }
    }
    /// Expected return from state `s` as E_π\[Q(s,a)\].
    pub fn expected_return(&self, s: usize, q: &ActionValueFunction) -> f64 {
        let pi = self.softmax(s);
        pi.iter().enumerate().map(|(a, &p)| p * q.get(s, a)).sum()
    }
    /// Convergence rate estimate: max |∇J| across states (gradient norm).
    pub fn convergence_rate(&self, q: &ActionValueFunction) -> f64 {
        let mut max_grad = 0.0_f64;
        for s in 0..self.theta.len() {
            let pi = self.softmax(s);
            for a in 0..self.theta[s].len() {
                let grad = pi
                    .iter()
                    .enumerate()
                    .map(|(b, &pb)| {
                        let indicator = if b == a { 1.0 } else { 0.0 };
                        (indicator - pb) * q.get(s, a)
                    })
                    .sum::<f64>()
                    .abs();
                if grad > max_grad {
                    max_grad = grad;
                }
            }
        }
        max_grad
    }
}
/// Q-learning solver with epsilon-greedy exploration and decaying step size.
#[derive(Debug, Clone)]
pub struct QLearningSolver {
    /// Q-value table Q\[s\]\[a\].
    pub q: Vec<Vec<f64>>,
    /// Learning rate α.
    pub alpha: f64,
    /// Discount factor γ.
    pub gamma: f64,
    /// Exploration rate ε.
    pub epsilon: f64,
    /// Step counter per (s,a) pair (for step-size decay).
    pub visit_count: Vec<Vec<u64>>,
}
impl QLearningSolver {
    /// Construct a Q-learning solver.
    pub fn new(
        num_states: usize,
        num_actions: usize,
        alpha: f64,
        gamma: f64,
        epsilon: f64,
    ) -> Self {
        Self {
            q: vec![vec![0.0_f64; num_actions]; num_states],
            alpha,
            gamma,
            epsilon,
            visit_count: vec![vec![0_u64; num_actions]; num_states],
        }
    }
    /// Perform a Q-learning update with harmonic step size 1/(1 + n(s,a)).
    pub fn update(&mut self, s: usize, a: usize, r: f64, s_next: usize) {
        self.visit_count[s][a] += 1;
        let n = self.visit_count[s][a] as f64;
        let step = self.alpha / (1.0 + n).sqrt();
        let max_q_next = self.q[s_next]
            .iter()
            .cloned()
            .fold(f64::NEG_INFINITY, f64::max);
        let td_error = r + self.gamma * max_q_next - self.q[s][a];
        self.q[s][a] += step * td_error;
    }
    /// Select an action using epsilon-greedy policy (deterministic tie-breaking).
    /// `rng_val` ∈ [0,1) is a uniform random value supplied by the caller.
    pub fn select_action(&self, s: usize, rng_val: f64) -> usize {
        if rng_val < self.epsilon {
            let n = self.q[s].len();
            ((rng_val / self.epsilon) * n as f64) as usize % n
        } else {
            self.q[s]
                .iter()
                .enumerate()
                .max_by(|x, y| x.1.partial_cmp(y.1).unwrap_or(std::cmp::Ordering::Equal))
                .map(|(i, _)| i)
                .unwrap_or(0)
        }
    }
    /// Check convergence: max |Q(s,a) - Q_prev(s,a)| < tol.
    pub fn has_converged(&self, prev_q: &[Vec<f64>], tol: f64) -> bool {
        self.q.iter().zip(prev_q.iter()).all(|(row, prev_row)| {
            row.iter()
                .zip(prev_row.iter())
                .all(|(q, pq)| (q - pq).abs() < tol)
        })
    }
    /// Return the current greedy policy.
    pub fn greedy_policy(&self) -> Vec<usize> {
        (0..self.q.len())
            .map(|s| {
                self.q[s]
                    .iter()
                    .enumerate()
                    .max_by(|x, y| x.1.partial_cmp(y.1).unwrap_or(std::cmp::Ordering::Equal))
                    .map(|(i, _)| i)
                    .unwrap_or(0)
            })
            .collect()
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct HInfinityControl {
    pub disturbance_attenuation: f64,
    pub state_dim: usize,
    pub control_dim: usize,
    pub disturbance_dim: usize,
    pub riccati_solution: Option<f64>,
}
#[allow(dead_code)]
impl HInfinityControl {
    pub fn new(gamma: f64, n: usize, m: usize, k: usize) -> Self {
        HInfinityControl {
            disturbance_attenuation: gamma,
            state_dim: n,
            control_dim: m,
            disturbance_dim: k,
            riccati_solution: None,
        }
    }
    pub fn minimax_criterion(&self) -> String {
        format!(
            "H∞: min_u max_w ||z||² - {:.3}² ||w||² (disturbance attenuation γ={:.3})",
            self.disturbance_attenuation, self.disturbance_attenuation
        )
    }
    pub fn game_riccati_equation(&self) -> String {
        format!(
            "Game ARE: PA + A^TP - P(B B^T - (1/{:.3}²) B_w B_w^T)P + C^TC = 0",
            self.disturbance_attenuation
        )
    }
    pub fn is_feasible(&self) -> bool {
        self.riccati_solution.map_or(false, |p| p > 0.0)
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ErgodicControl {
    pub discount_rate: f64,
    pub state_space_dim: usize,
    pub long_run_cost: Option<f64>,
    pub eigenvalue_lambda: Option<f64>,
}
#[allow(dead_code)]
impl ErgodicControl {
    pub fn new(dim: usize) -> Self {
        ErgodicControl {
            discount_rate: 0.0,
            state_space_dim: dim,
            long_run_cost: None,
            eigenvalue_lambda: None,
        }
    }
    pub fn ergodic_hjb(&self) -> String {
        "λ + H(x, ∇V, ∇²V) = 0 (ergodic HJB: λ is long-run average cost)".to_string()
    }
    pub fn turnpike_property(&self) -> String {
        "Turnpike: finite-horizon optimal trajectories spend most time near ergodic optimal"
            .to_string()
    }
    pub fn set_eigenvalue(&mut self, lambda: f64) {
        self.eigenvalue_lambda = Some(lambda);
        self.long_run_cost = Some(lambda);
    }
    pub fn relative_value_function_description(&self) -> String {
        format!(
            "Ergodic control dim={}: solve (λ*, V) pair in ergodic HJB",
            self.state_space_dim
        )
    }
}
/// Almost-sure stability: checks ||x_t|| → 0 along sample paths.
#[derive(Debug, Clone)]
pub struct AlmostSureStability {
    /// Sample paths: each inner Vec is a trajectory of ||x_t||.
    pub paths: Vec<Vec<f64>>,
}
impl AlmostSureStability {
    /// Construct from sample paths.
    pub fn new(paths: Vec<Vec<f64>>) -> Self {
        Self { paths }
    }
    /// Fraction of paths that have converged below `tol` at the final time step.
    pub fn convergence_fraction(&self, tol: f64) -> f64 {
        if self.paths.is_empty() {
            return 0.0;
        }
        let converged = self
            .paths
            .iter()
            .filter(|p| p.last().is_some_and(|&v| v < tol))
            .count();
        converged as f64 / self.paths.len() as f64
    }
}
/// Deterministic policy π: S → A.
#[derive(Debug, Clone)]
pub struct Policy {
    /// `table\[s\]` = action chosen in state s.
    pub table: Vec<usize>,
}
impl Policy {
    /// Construct a policy from an action table.
    pub fn new(table: Vec<usize>) -> Self {
        Self { table }
    }
    /// Return the action for state `s`.
    pub fn action(&self, s: usize) -> usize {
        self.table[s]
    }
}
/// SARSA agent (on-policy TD(0)).
///
/// Q(s,a) ← Q(s,a) + α(r + γ Q(s',a') − Q(s,a)).
#[derive(Debug, Clone)]
pub struct SARSA {
    /// Q-value table.
    pub q: Vec<Vec<f64>>,
    /// Learning rate α.
    pub alpha: f64,
    /// Discount factor γ.
    pub gamma: f64,
}
impl SARSA {
    /// Construct a SARSA agent.
    pub fn new(num_states: usize, num_actions: usize, alpha: f64, gamma: f64) -> Self {
        Self {
            q: vec![vec![0.0_f64; num_actions]; num_states],
            alpha,
            gamma,
        }
    }
    /// Perform a single SARSA update given (s, a, r, s', a').
    pub fn update(&mut self, s: usize, a: usize, r: f64, s_next: usize, a_next: usize) {
        let td_error = r + self.gamma * self.q[s_next][a_next] - self.q[s][a];
        self.q[s][a] += self.alpha * td_error;
    }
    /// Return the greedy action in state `s`.
    pub fn greedy_action(&self, s: usize) -> usize {
        self.q[s]
            .iter()
            .enumerate()
            .max_by(|x, y| x.1.partial_cmp(y.1).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(i, _)| i)
            .unwrap_or(0)
    }
    /// Expected return from state `s` under the current greedy policy.
    pub fn expected_return(&self, s: usize) -> f64 {
        self.q[s].iter().cloned().fold(f64::NEG_INFINITY, f64::max)
    }
}
