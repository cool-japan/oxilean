//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, VecDeque};

/// Bellman-Ford single-source shortest path algorithm.
///
/// Handles negative edge weights and detects negative cycles.
#[derive(Debug, Clone)]
pub struct BellmanFord {
    /// Number of vertices.
    pub n: usize,
    /// Edge list: (from, to, weight).
    pub edges: Vec<(usize, usize, i64)>,
}
impl BellmanFord {
    /// Create a new graph with `n` vertices.
    pub fn new(n: usize) -> Self {
        Self {
            n,
            edges: Vec::new(),
        }
    }
    /// Add a directed edge.
    pub fn add_edge(&mut self, from: usize, to: usize, weight: i64) {
        self.edges.push((from, to, weight));
    }
    /// Run Bellman-Ford from `source`.
    ///
    /// Returns `Ok(dist)` where `dist\[v\]` is the shortest distance to `v`
    /// (`i64::MAX` means unreachable), or `Err(())` if a negative cycle exists.
    #[allow(clippy::result_unit_err)]
    pub fn shortest_paths(&self, source: usize) -> Result<Vec<i64>, ()> {
        let inf = i64::MAX / 2;
        let mut dist = vec![inf; self.n];
        dist[source] = 0;
        for _ in 0..self.n - 1 {
            for &(u, v, w) in &self.edges {
                if dist[u] != inf && dist[u] + w < dist[v] {
                    dist[v] = dist[u] + w;
                }
            }
        }
        for &(u, v, w) in &self.edges {
            if dist[u] != inf && dist[u] + w < dist[v] {
                return Err(());
            }
        }
        Ok(dist)
    }
}
/// Discrete-state, discrete-action MDP solver.
///
/// Implements value iteration and policy iteration for discounted MDPs.
#[allow(dead_code)]
pub struct MdpSolver {
    /// Number of states.
    pub num_states: usize,
    /// Number of actions.
    pub num_actions: usize,
    /// Discount factor γ ∈ [0, 1).
    pub discount: f64,
    /// Reward matrix R\[s\]\[a\].
    pub rewards: Vec<Vec<f64>>,
    /// Transition probabilities P\[s\]\[a\][s'].
    pub transitions: Vec<Vec<Vec<f64>>>,
}
impl MdpSolver {
    /// Create a new MDP solver.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        num_states: usize,
        num_actions: usize,
        discount: f64,
        rewards: Vec<Vec<f64>>,
        transitions: Vec<Vec<Vec<f64>>>,
    ) -> Self {
        Self {
            num_states,
            num_actions,
            discount,
            rewards,
            transitions,
        }
    }
    /// Apply the Bellman operator once: T V(s) = max_a [R(s,a) + γ Σ P(s'|s,a) V(s')]
    pub fn bellman_update(&self, v: &[f64]) -> Vec<f64> {
        (0..self.num_states)
            .map(|s| {
                (0..self.num_actions)
                    .map(|a| {
                        let future: f64 = (0..self.num_states)
                            .map(|sp| self.transitions[s][a][sp] * v[sp])
                            .sum();
                        self.rewards[s][a] + self.discount * future
                    })
                    .fold(f64::NEG_INFINITY, f64::max)
            })
            .collect()
    }
    /// Run value iteration until convergence.
    ///
    /// Returns (V*, iterations) where V*\[s\] is the optimal value of state s.
    pub fn value_iteration(&self, tol: f64, max_iter: usize) -> (Vec<f64>, usize) {
        let mut v = vec![0.0_f64; self.num_states];
        for iter in 0..max_iter {
            let v_new = self.bellman_update(&v);
            let delta = v_new
                .iter()
                .zip(v.iter())
                .map(|(a, b)| (a - b).abs())
                .fold(0.0_f64, f64::max);
            v = v_new;
            if delta < tol {
                return (v, iter + 1);
            }
        }
        (v, max_iter)
    }
    /// Extract greedy policy from value function V.
    ///
    /// Returns policy\[s\] = argmax_a [R(s,a) + γ Σ P(s'|s,a) V(s')].
    pub fn extract_policy(&self, v: &[f64]) -> Vec<usize> {
        (0..self.num_states)
            .map(|s| {
                (0..self.num_actions)
                    .map(|a| {
                        let future: f64 = (0..self.num_states)
                            .map(|sp| self.transitions[s][a][sp] * v[sp])
                            .sum();
                        (a, self.rewards[s][a] + self.discount * future)
                    })
                    .max_by(|x, y| x.1.partial_cmp(&y.1).unwrap_or(std::cmp::Ordering::Equal))
                    .map(|(a, _)| a)
                    .unwrap_or(0)
            })
            .collect()
    }
    /// Evaluate a fixed policy: V^π(s) = R(s,π(s)) + γ Σ P(s'|s,π(s)) V^π(s')
    ///
    /// Solved by iteration (similar to value iteration for fixed policy).
    pub fn policy_evaluation(&self, policy: &[usize], tol: f64, max_iter: usize) -> Vec<f64> {
        let mut v = vec![0.0_f64; self.num_states];
        for _ in 0..max_iter {
            let v_new: Vec<f64> = (0..self.num_states)
                .map(|s| {
                    let a = policy[s];
                    let future: f64 = (0..self.num_states)
                        .map(|sp| self.transitions[s][a][sp] * v[sp])
                        .sum();
                    self.rewards[s][a] + self.discount * future
                })
                .collect();
            let delta = v_new
                .iter()
                .zip(v.iter())
                .map(|(a, b)| (a - b).abs())
                .fold(0.0_f64, f64::max);
            v = v_new;
            if delta < tol {
                break;
            }
        }
        v
    }
    /// Policy iteration: alternate between evaluation and improvement.
    ///
    /// Returns (optimal_policy, optimal_value, iterations).
    pub fn policy_iteration(&self, tol: f64, max_iter: usize) -> (Vec<usize>, Vec<f64>, usize) {
        let mut policy: Vec<usize> = vec![0; self.num_states];
        for iter in 0..max_iter {
            let v = self.policy_evaluation(&policy, tol / 10.0, 1000);
            let new_policy = self.extract_policy(&v);
            if new_policy == policy {
                return (policy, v, iter + 1);
            }
            policy = new_policy;
        }
        let v = self.policy_evaluation(&policy, tol, 1000);
        (policy, v, max_iter)
    }
}
/// A directed graph for max-flow computation.
#[derive(Debug, Clone)]
pub struct NetworkFlowGraph {
    /// Number of nodes.
    pub n: usize,
    /// Capacity matrix (n × n).
    pub capacities: Vec<Vec<i64>>,
}
impl NetworkFlowGraph {
    /// Create a new flow graph with `n` nodes and all-zero capacities.
    pub fn new(n: usize) -> Self {
        Self {
            n,
            capacities: vec![vec![0_i64; n]; n],
        }
    }
    /// Add a directed edge u → v with capacity `cap`.
    pub fn add_edge(&mut self, u: usize, v: usize, cap: i64) {
        self.capacities[u][v] += cap;
    }
    /// Compute the maximum flow from `source` to `sink` using Edmonds-Karp (BFS augmentation).
    pub fn max_flow_bfs(&self, source: usize, sink: usize) -> i64 {
        let n = self.n;
        let mut residual = self.capacities.clone();
        let mut total_flow = 0_i64;
        loop {
            let mut parent = vec![usize::MAX; n];
            parent[source] = source;
            let mut queue = VecDeque::new();
            queue.push_back(source);
            while let Some(u) = queue.pop_front() {
                for v in 0..n {
                    if parent[v] == usize::MAX && residual[u][v] > 0 {
                        parent[v] = u;
                        if v == sink {
                            break;
                        }
                        queue.push_back(v);
                    }
                }
                if parent[sink] != usize::MAX {
                    break;
                }
            }
            if parent[sink] == usize::MAX {
                break;
            }
            let mut path_flow = i64::MAX;
            let mut v = sink;
            while v != source {
                let u = parent[v];
                path_flow = path_flow.min(residual[u][v]);
                v = u;
            }
            v = sink;
            while v != source {
                let u = parent[v];
                residual[u][v] -= path_flow;
                residual[v][u] += path_flow;
                v = u;
            }
            total_flow += path_flow;
        }
        total_flow
    }
    /// The minimum cut value equals the maximum flow (max-flow min-cut theorem).
    pub fn min_cut_value(&self, source: usize, sink: usize) -> i64 {
        self.max_flow_bfs(source, sink)
    }
}
/// A collection of classic dynamic programming algorithms.
pub struct DynamicProgramming;
impl DynamicProgramming {
    /// 0/1 knapsack: maximum value achievable with given `capacity`.
    pub fn knapsack(capacity: usize, weights: &[usize], values: &[usize]) -> usize {
        let n = weights.len();
        let mut dp = vec![vec![0_usize; capacity + 1]; n + 1];
        for i in 1..=n {
            for w in 0..=capacity {
                dp[i][w] = dp[i - 1][w];
                if weights[i - 1] <= w {
                    let with_item = dp[i - 1][w - weights[i - 1]] + values[i - 1];
                    if with_item > dp[i][w] {
                        dp[i][w] = with_item;
                    }
                }
            }
        }
        dp[n][capacity]
    }
    /// Length of the longest common subsequence of byte slices `s1` and `s2`.
    pub fn longest_common_subseq(s1: &[u8], s2: &[u8]) -> usize {
        let (m, n) = (s1.len(), s2.len());
        let mut dp = vec![vec![0_usize; n + 1]; m + 1];
        for i in 1..=m {
            for j in 1..=n {
                dp[i][j] = if s1[i - 1] == s2[j - 1] {
                    dp[i - 1][j - 1] + 1
                } else {
                    dp[i - 1][j].max(dp[i][j - 1])
                };
            }
        }
        dp[m][n]
    }
    /// Minimum number of scalar multiplications to compute a chain of matrices.
    ///
    /// `dims` has length n+1 where the i-th matrix is dims\[i\] × dims[i+1].
    pub fn matrix_chain_order(dims: &[usize]) -> usize {
        let n = dims.len().saturating_sub(1);
        if n == 0 {
            return 0;
        }
        let mut dp = vec![vec![0_usize; n]; n];
        for len in 2..=n {
            for i in 0..=(n - len) {
                let j = i + len - 1;
                dp[i][j] = usize::MAX;
                for k in i..j {
                    let cost = dp[i][k]
                        .saturating_add(dp[k + 1][j])
                        .saturating_add(dims[i] * dims[k + 1] * dims[j + 1]);
                    if cost < dp[i][j] {
                        dp[i][j] = cost;
                    }
                }
            }
        }
        dp[0][n - 1]
    }
    /// Minimum number of coins from `coins` that sum to `amount`.
    ///
    /// Returns `None` if no solution exists.
    pub fn coin_change(coins: &[usize], amount: usize) -> Option<usize> {
        let inf = usize::MAX / 2;
        let mut dp = vec![inf; amount + 1];
        dp[0] = 0;
        for a in 1..=amount {
            for &c in coins {
                if c <= a && dp[a - c] != inf {
                    let cand = dp[a - c] + 1;
                    if cand < dp[a] {
                        dp[a] = cand;
                    }
                }
            }
        }
        if dp[amount] == inf {
            None
        } else {
            Some(dp[amount])
        }
    }
}
/// An M/M/c queueing model.
#[derive(Debug, Clone)]
pub struct QueueingSystem {
    /// Arrival rate λ (customers per unit time).
    pub arrival_rate: f64,
    /// Service rate μ per server (customers per unit time).
    pub service_rate: f64,
    /// Number of servers c.
    pub num_servers: usize,
}
impl QueueingSystem {
    /// Create a new M/M/c queueing system.
    pub fn new(lambda: f64, mu: f64, c: usize) -> Self {
        Self {
            arrival_rate: lambda,
            service_rate: mu,
            num_servers: c,
        }
    }
    /// Traffic intensity ρ = λ / (c μ).
    pub fn utilization(&self) -> f64 {
        self.arrival_rate / (self.num_servers as f64 * self.service_rate)
    }
    /// The system is stable when ρ < 1.
    pub fn is_stable(&self) -> bool {
        self.utilization() < 1.0
    }
    /// Mean number of customers in the M/M/1 queue: L = ρ / (1 − ρ).
    ///
    /// Returns `None` if the system is unstable or has more than one server.
    pub fn mean_queue_length_m_m_1(&self) -> Option<f64> {
        if self.num_servers != 1 || !self.is_stable() {
            return None;
        }
        let rho = self.utilization();
        Some(rho / (1.0 - rho))
    }
    /// Mean sojourn time W = L / λ (Little's law), for M/M/1.
    ///
    /// Returns `None` if the system is unstable or has more than one server.
    pub fn mean_waiting_time(&self) -> Option<f64> {
        let l = self.mean_queue_length_m_m_1()?;
        Some(l / self.arrival_rate)
    }
}
/// A collection of jobs with processing times and deadlines.
#[derive(Debug, Clone)]
pub struct JobScheduler {
    /// Jobs: (name, processing_time, deadline).
    pub jobs: Vec<(String, u64, u64)>,
}
impl JobScheduler {
    /// Create an empty job scheduler.
    pub fn new() -> Self {
        Self { jobs: Vec::new() }
    }
    /// Add a job with the given name, processing time and deadline.
    pub fn add_job(&mut self, name: &str, proc_time: u64, deadline: u64) {
        self.jobs.push((name.to_owned(), proc_time, deadline));
    }
    /// Earliest Deadline First (EDF) schedule — returns job names in EDF order.
    pub fn earliest_deadline_first(&self) -> Vec<&str> {
        let mut indices: Vec<usize> = (0..self.jobs.len()).collect();
        indices.sort_by_key(|&i| self.jobs[i].2);
        indices.iter().map(|&i| self.jobs[i].0.as_str()).collect()
    }
    /// Shortest Job First (SJF) schedule — returns job names in SJF order.
    pub fn shortest_job_first(&self) -> Vec<&str> {
        let mut indices: Vec<usize> = (0..self.jobs.len()).collect();
        indices.sort_by_key(|&i| self.jobs[i].1);
        indices.iter().map(|&i| self.jobs[i].0.as_str()).collect()
    }
    /// Compute the total completion time for a given schedule (by job name).
    pub fn total_completion_time(&self, schedule: &[&str]) -> u64 {
        let mut time = 0_u64;
        let mut total = 0_u64;
        for name in schedule {
            if let Some(job) = self.jobs.iter().find(|(n, _, _)| n == name) {
                time += job.1;
                total += time;
            }
        }
        total
    }
    /// Compute the makespan (total elapsed time) for a given schedule.
    pub fn makespan(&self, schedule: &[&str]) -> u64 {
        schedule
            .iter()
            .filter_map(|name| self.jobs.iter().find(|(n, _, _)| n == name))
            .map(|(_, p, _)| p)
            .sum()
    }
}
/// Prim's algorithm for Minimum Spanning Tree on a dense graph.
#[derive(Debug, Clone)]
pub struct PrimMst {
    /// Number of vertices.
    pub n: usize,
    /// Adjacency matrix; `cost\[i\]\[j\] = i64::MAX` means no edge.
    pub cost: Vec<Vec<i64>>,
}
impl PrimMst {
    /// Create a new MST solver with `n` vertices (no edges initially).
    pub fn new(n: usize) -> Self {
        Self {
            n,
            cost: vec![vec![i64::MAX; n]; n],
        }
    }
    /// Add an undirected edge between u and v with weight w.
    pub fn add_edge(&mut self, u: usize, v: usize, w: i64) {
        self.cost[u][v] = w;
        self.cost[v][u] = w;
    }
    /// Run Prim's algorithm starting from vertex 0.
    ///
    /// Returns `(total_weight, edges)` where each edge is `(u, v, weight)`.
    pub fn run(&self) -> (i64, Vec<(usize, usize, i64)>) {
        let n = self.n;
        let mut in_mst = vec![false; n];
        let mut key = vec![i64::MAX; n];
        let mut parent = vec![usize::MAX; n];
        key[0] = 0;
        let mut mst_edges = Vec::new();
        let mut total = 0_i64;
        for _ in 0..n {
            let u = (0..n)
                .filter(|&v| !in_mst[v])
                .min_by_key(|&v| key[v])
                .expect("Prim's algorithm: there is always an unvisited vertex in 0..n iterations");
            in_mst[u] = true;
            if parent[u] != usize::MAX {
                let w = key[u];
                mst_edges.push((parent[u], u, w));
                total += w;
            }
            for v in 0..n {
                if !in_mst[v] && self.cost[u][v] < key[v] {
                    key[v] = self.cost[u][v];
                    parent[v] = u;
                }
            }
        }
        (total, mst_edges)
    }
}
/// Reliability calculations for series and parallel systems.
#[derive(Debug, Clone)]
pub struct ReliabilitySystem {
    /// Component reliabilities (probabilities of success in \[0, 1\]).
    pub components: Vec<f64>,
}
impl ReliabilitySystem {
    /// Create a reliability system from a list of component reliabilities.
    pub fn new(components: Vec<f64>) -> Self {
        Self { components }
    }
    /// Series system reliability: R = ∏ R_i.
    pub fn series_reliability(&self) -> f64 {
        self.components.iter().product()
    }
    /// Parallel system reliability: R = 1 - ∏ (1 - R_i).
    pub fn parallel_reliability(&self) -> f64 {
        1.0 - self.components.iter().map(|&r| 1.0 - r).product::<f64>()
    }
    /// k-out-of-n system reliability (exact binomial sum).
    ///
    /// Returns the probability that at least `k` out of `n` components succeed,
    /// assuming all components have the same reliability `p = components\[0\]`.
    pub fn k_out_of_n_reliability(&self, k: usize) -> f64 {
        let n = self.components.len();
        let p = if n == 0 {
            return 0.0;
        } else {
            self.components[0]
        };
        let q = 1.0 - p;
        (k..=n)
            .map(|j| {
                let binom = binomial_coeff(n, j) as f64;
                binom * p.powi(j as i32) * q.powi((n - j) as i32)
            })
            .sum()
    }
}
/// Simulate a multi-armed bandit environment with known true means.
#[allow(dead_code)]
pub struct BanditEnvironment {
    /// True mean rewards for each arm.
    pub true_means: Vec<f64>,
    /// Pseudo-random state.
    rng_state: u64,
}
impl BanditEnvironment {
    /// Create a bandit environment with given true means.
    pub fn new(true_means: Vec<f64>) -> Self {
        Self {
            true_means,
            rng_state: 42,
        }
    }
    /// Sample a reward from arm i (Gaussian noise with σ=1).
    pub fn sample(&mut self, arm: usize) -> f64 {
        let u1 = self.lcg_next();
        let u2 = self.lcg_next();
        let z = (-2.0 * u1.ln()).sqrt() * (2.0 * std::f64::consts::PI * u2).cos();
        self.true_means[arm] + z
    }
    /// LCG pseudo-random number in (0, 1).
    fn lcg_next(&mut self) -> f64 {
        self.rng_state = self
            .rng_state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        let val = (self.rng_state >> 33) as f64 / (u32::MAX as f64 + 1.0);
        val.max(1e-10)
    }
    /// Optimal arm (highest true mean).
    pub fn optimal_arm(&self) -> usize {
        self.true_means
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(i, _)| i)
            .unwrap_or(0)
    }
    /// Run UCB1 for T rounds; return cumulative regret.
    pub fn run_ucb1(&mut self, rounds: usize) -> f64 {
        let k = self.true_means.len();
        let optimal_mean = self.true_means[self.optimal_arm()];
        let mut bandit = MultiArmedBanditUcb::new(k);
        let mut cumulative_regret = 0.0_f64;
        for _ in 0..rounds {
            let arm = bandit.select_arm();
            let reward = self.sample(arm);
            bandit.update(arm, reward);
            cumulative_regret += optimal_mean - self.true_means[arm];
        }
        cumulative_regret
    }
}
/// Multi-armed bandit solver using the UCB1 algorithm.
///
/// UCB1 index: Q̄_i + √(2 ln T / n_i)
/// where Q̄_i = average reward of arm i, T = total plays, n_i = plays of arm i.
#[allow(dead_code)]
pub struct MultiArmedBanditUcb {
    /// Number of arms.
    pub num_arms: usize,
    /// Cumulative rewards per arm.
    pub total_rewards: Vec<f64>,
    /// Number of pulls per arm.
    pub pull_counts: Vec<u64>,
    /// Total number of rounds played.
    pub total_rounds: u64,
}
impl MultiArmedBanditUcb {
    /// Create a new UCB1 bandit with k arms.
    pub fn new(num_arms: usize) -> Self {
        Self {
            num_arms,
            total_rewards: vec![0.0; num_arms],
            pull_counts: vec![0; num_arms],
            total_rounds: 0,
        }
    }
    /// UCB1 index for arm i.
    pub fn ucb_index(&self, arm: usize) -> f64 {
        if self.pull_counts[arm] == 0 {
            return f64::INFINITY;
        }
        let mean = self.total_rewards[arm] / self.pull_counts[arm] as f64;
        let t = self.total_rounds as f64;
        let n = self.pull_counts[arm] as f64;
        mean + (2.0 * t.ln() / n).sqrt()
    }
    /// Select the arm with the highest UCB index (ties broken by smallest index).
    pub fn select_arm(&self) -> usize {
        (0..self.num_arms)
            .max_by(|&a, &b| {
                let cmp = self
                    .ucb_index(a)
                    .partial_cmp(&self.ucb_index(b))
                    .unwrap_or(std::cmp::Ordering::Equal);
                // Break ties by preferring smaller arm index
                cmp.then(b.cmp(&a))
            })
            .unwrap_or(0)
    }
    /// Update statistics after pulling arm with observed reward.
    pub fn update(&mut self, arm: usize, reward: f64) {
        self.total_rewards[arm] += reward;
        self.pull_counts[arm] += 1;
        self.total_rounds += 1;
    }
    /// Average reward for arm i.
    pub fn average_reward(&self, arm: usize) -> f64 {
        if self.pull_counts[arm] == 0 {
            return 0.0;
        }
        self.total_rewards[arm] / self.pull_counts[arm] as f64
    }
    /// Empirically best arm (highest average reward).
    pub fn best_arm(&self) -> usize {
        (0..self.num_arms)
            .filter(|&i| self.pull_counts[i] > 0)
            .max_by(|&a, &b| {
                self.average_reward(a)
                    .partial_cmp(&self.average_reward(b))
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .unwrap_or(0)
    }
}
/// Two-stage stochastic linear program.
///
/// Stage 1: min cᵀx + E_ξ\[Q(x, ξ)\]  s.t. Ax = b, x ≥ 0
/// Stage 2: Q(x, ξ) = min qᵀy  s.t. Wy = h - Tx, y ≥ 0
#[allow(dead_code)]
pub struct TwoStageStochasticLP {
    /// Stage 1 objective coefficient c.
    pub stage1_obj: Vec<f64>,
    /// Stage 2 objective coefficient q (same for all scenarios here).
    pub stage2_obj: Vec<f64>,
    /// Scenario probabilities.
    pub probabilities: Vec<f64>,
    /// Recourse right-hand sides h_s for each scenario s.
    pub scenario_rhs: Vec<Vec<f64>>,
}
impl TwoStageStochasticLP {
    /// Create a two-stage stochastic LP.
    pub fn new(
        stage1_obj: Vec<f64>,
        stage2_obj: Vec<f64>,
        probabilities: Vec<f64>,
        scenario_rhs: Vec<Vec<f64>>,
    ) -> Self {
        Self {
            stage1_obj,
            stage2_obj,
            probabilities,
            scenario_rhs,
        }
    }
    /// Number of scenarios.
    pub fn num_scenarios(&self) -> usize {
        self.probabilities.len()
    }
    /// Expected recourse cost given stage-2 decisions y_s per scenario.
    ///
    /// E\[qᵀy\] = Σ_s p_s * qᵀy_s
    pub fn expected_recourse_cost(&self, stage2_decisions: &[Vec<f64>]) -> f64 {
        self.probabilities
            .iter()
            .zip(stage2_decisions.iter())
            .map(|(&p, y)| {
                let cost: f64 = self
                    .stage2_obj
                    .iter()
                    .zip(y.iter())
                    .map(|(q, v)| q * v)
                    .sum();
                p * cost
            })
            .sum()
    }
    /// Total cost given stage-1 decisions x and stage-2 decisions y_s.
    pub fn total_cost(&self, stage1_x: &[f64], stage2_decisions: &[Vec<f64>]) -> f64 {
        let c1: f64 = self
            .stage1_obj
            .iter()
            .zip(stage1_x.iter())
            .map(|(c, x)| c * x)
            .sum();
        c1 + self.expected_recourse_cost(stage2_decisions)
    }
    /// Check if stage-1 solution is non-negative.
    pub fn is_stage1_feasible(&self, x: &[f64]) -> bool {
        x.iter().all(|&v| v >= -1e-9)
    }
}
/// 0/1 knapsack solver with item selection tracking.
#[derive(Debug, Clone)]
pub struct KnapsackDP {
    /// Item weights.
    pub weights: Vec<usize>,
    /// Item values.
    pub values: Vec<usize>,
    /// Knapsack capacity.
    pub capacity: usize,
}
impl KnapsackDP {
    /// Create a new knapsack instance.
    pub fn new(capacity: usize, weights: Vec<usize>, values: Vec<usize>) -> Self {
        Self {
            weights,
            values,
            capacity,
        }
    }
    /// Solve and return `(optimal_value, selected_item_indices)`.
    pub fn solve(&self) -> (usize, Vec<usize>) {
        let n = self.weights.len();
        let cap = self.capacity;
        let mut dp = vec![vec![0_usize; cap + 1]; n + 1];
        for i in 1..=n {
            for w in 0..=cap {
                dp[i][w] = dp[i - 1][w];
                if self.weights[i - 1] <= w {
                    let with_item = dp[i - 1][w - self.weights[i - 1]] + self.values[i - 1];
                    if with_item > dp[i][w] {
                        dp[i][w] = with_item;
                    }
                }
            }
        }
        let mut selected = Vec::new();
        let mut w = cap;
        for i in (1..=n).rev() {
            if dp[i][w] != dp[i - 1][w] {
                selected.push(i - 1);
                w -= self.weights[i - 1];
            }
        }
        selected.reverse();
        (dp[n][cap], selected)
    }
}
/// Classical Economic Order Quantity (EOQ) inventory model.
#[derive(Debug, Clone)]
pub struct InventoryModel {
    /// Annual demand rate D.
    pub demand: f64,
    /// Fixed order cost per order S.
    pub order_cost: f64,
    /// Holding cost per unit per year H.
    pub holding_cost: f64,
    /// Lead time L (in the same time unit as demand).
    pub lead_time: f64,
}
impl InventoryModel {
    /// Create a new inventory model.
    pub fn new(d: f64, s: f64, h: f64, l: f64) -> Self {
        Self {
            demand: d,
            order_cost: s,
            holding_cost: h,
            lead_time: l,
        }
    }
    /// Economic Order Quantity: Q* = sqrt(2 D S / H).
    pub fn eoq(&self) -> f64 {
        (2.0 * self.demand * self.order_cost / self.holding_cost).sqrt()
    }
    /// Reorder point: R = D * L + safety_stock.
    pub fn reorder_point(&self, safety_stock: f64) -> f64 {
        self.demand * self.lead_time + safety_stock
    }
    /// Total annual cost for order quantity q: (D/q)S + (q/2)H.
    pub fn total_cost(&self, q: f64) -> f64 {
        (self.demand / q) * self.order_cost + (q / 2.0) * self.holding_cost
    }
}
/// Floyd-Warshall all-pairs shortest paths.
#[derive(Debug, Clone)]
pub struct FloydWarshall {
    /// Number of vertices.
    pub n: usize,
    /// Distance matrix; `dist\[i\]\[j\] = i64::MAX/2` means no direct edge.
    pub dist: Vec<Vec<i64>>,
}
impl FloydWarshall {
    /// Create a new instance with `n` vertices (self-loops = 0, rest = infinity).
    pub fn new(n: usize) -> Self {
        let inf = i64::MAX / 2;
        let mut dist = vec![vec![inf; n]; n];
        for i in 0..n {
            dist[i][i] = 0;
        }
        Self { n, dist }
    }
    /// Add a directed edge u → v with weight `w`.
    pub fn add_edge(&mut self, u: usize, v: usize, w: i64) {
        if w < self.dist[u][v] {
            self.dist[u][v] = w;
        }
    }
    /// Run Floyd-Warshall and return the all-pairs distance matrix.
    ///
    /// Returns `Err(())` if a negative cycle is detected.
    #[allow(clippy::result_unit_err)]
    pub fn run(&mut self) -> Result<Vec<Vec<i64>>, ()> {
        let n = self.n;
        for k in 0..n {
            for i in 0..n {
                for j in 0..n {
                    if self.dist[i][k] != i64::MAX / 2 && self.dist[k][j] != i64::MAX / 2 {
                        let through_k = self.dist[i][k] + self.dist[k][j];
                        if through_k < self.dist[i][j] {
                            self.dist[i][j] = through_k;
                        }
                    }
                }
            }
        }
        for i in 0..n {
            if self.dist[i][i] < 0 {
                return Err(());
            }
        }
        Ok(self.dist.clone())
    }
}
/// Hungarian algorithm for the linear assignment problem.
///
/// Given an n×n cost matrix, find an assignment of agents to tasks with
/// minimum total cost. Runs in O(n³).
#[derive(Debug, Clone)]
pub struct HungarianSolver {
    /// Cost matrix (n × n).
    pub cost: Vec<Vec<i64>>,
}
impl HungarianSolver {
    /// Create a new solver with the given square cost matrix.
    pub fn new(cost: Vec<Vec<i64>>) -> Self {
        Self { cost }
    }
    /// Solve the assignment problem.
    ///
    /// Returns `(min_cost, assignment)` where `assignment\[i\]` is the task
    /// assigned to agent `i`.
    pub fn solve(&self) -> (i64, Vec<usize>) {
        let n = self.cost.len();
        if n == 0 {
            return (0, vec![]);
        }
        let inf = i64::MAX / 2;
        let mut u = vec![0_i64; n + 1];
        let mut v = vec![0_i64; n + 1];
        let mut p = vec![0_usize; n + 1];
        let mut way = vec![0_usize; n + 1];
        for i in 1..=n {
            p[0] = i;
            let mut j0 = 0_usize;
            let mut minv = vec![inf; n + 1];
            let mut used = vec![false; n + 1];
            loop {
                used[j0] = true;
                let i0 = p[j0];
                let mut delta = inf;
                let mut j1 = 0_usize;
                for j in 1..=n {
                    if !used[j] {
                        let cur = self.cost[i0 - 1][j - 1] - u[i0] - v[j];
                        if cur < minv[j] {
                            minv[j] = cur;
                            way[j] = j0;
                        }
                        if minv[j] < delta {
                            delta = minv[j];
                            j1 = j;
                        }
                    }
                }
                for j in 0..=n {
                    if used[j] {
                        u[p[j]] += delta;
                        v[j] -= delta;
                    } else {
                        minv[j] -= delta;
                    }
                }
                j0 = j1;
                if p[j0] == 0 {
                    break;
                }
            }
            loop {
                p[j0] = p[way[j0]];
                j0 = way[j0];
                if j0 == 0 {
                    break;
                }
            }
        }
        let mut assignment = vec![0_usize; n];
        for j in 1..=n {
            if p[j] != 0 {
                assignment[p[j] - 1] = j - 1;
            }
        }
        let min_cost: i64 = (0..n).map(|i| self.cost[i][assignment[i]]).sum();
        (min_cost, assignment)
    }
}
/// The newsvendor (critical fractile) inventory model.
///
/// Demand is assumed uniform on \[demand_lo, demand_hi\].
#[derive(Debug, Clone)]
pub struct NewsvendorModel {
    /// Lower bound of uniform demand.
    pub demand_lo: f64,
    /// Upper bound of uniform demand.
    pub demand_hi: f64,
    /// Unit purchase (production) cost c_u.
    pub unit_cost: f64,
    /// Unit selling price p.
    pub selling_price: f64,
    /// Unit salvage value s (for unsold units).
    pub salvage_value: f64,
}
impl NewsvendorModel {
    /// Create a new newsvendor model.
    pub fn new(lo: f64, hi: f64, cost: f64, price: f64, salvage: f64) -> Self {
        Self {
            demand_lo: lo,
            demand_hi: hi,
            unit_cost: cost,
            selling_price: price,
            salvage_value: salvage,
        }
    }
    /// Critical fractile: c_r / (c_r + c_e) where c_r = underage cost, c_e = overage cost.
    pub fn critical_fractile(&self) -> f64 {
        let c_e = self.unit_cost - self.salvage_value;
        let c_r = self.selling_price - self.unit_cost;
        c_r / (c_r + c_e)
    }
    /// Optimal order quantity Q* (using inverse CDF of uniform distribution).
    pub fn optimal_quantity(&self) -> f64 {
        let cf = self.critical_fractile();
        self.demand_lo + cf * (self.demand_hi - self.demand_lo)
    }
    /// Expected profit at order quantity q.
    pub fn expected_profit(&self, q: f64) -> f64 {
        let lo = self.demand_lo;
        let hi = self.demand_hi;
        let range = hi - lo;
        let e_min = if q <= lo {
            q
        } else if q >= hi {
            (lo + hi) / 2.0
        } else {
            let below = (q * (q - lo) - (q * q - lo * lo) / 2.0) / range;
            let above = q * (hi - q) / range;
            below + above
        };
        let e_over = q - e_min;
        self.selling_price * e_min + self.salvage_value * e_over - self.unit_cost * q
    }
}
/// Ford-Fulkerson max-flow using DFS (depth-first) augmentation.
#[derive(Debug, Clone)]
pub struct FordFulkerson {
    /// Number of nodes.
    pub n: usize,
    /// Residual capacity matrix.
    residual: Vec<Vec<i64>>,
}
impl FordFulkerson {
    /// Create a new graph with `n` nodes.
    pub fn new(n: usize) -> Self {
        Self {
            n,
            residual: vec![vec![0_i64; n]; n],
        }
    }
    /// Add directed edge u → v with capacity `cap`.
    pub fn add_edge(&mut self, u: usize, v: usize, cap: i64) {
        self.residual[u][v] += cap;
    }
    /// DFS to find an augmenting path; returns bottleneck flow or 0.
    fn dfs(&mut self, u: usize, sink: usize, flow: i64, visited: &mut Vec<bool>) -> i64 {
        if u == sink {
            return flow;
        }
        visited[u] = true;
        for v in 0..self.n {
            if !visited[v] && self.residual[u][v] > 0 {
                let pushed = self.dfs(v, sink, flow.min(self.residual[u][v]), visited);
                if pushed > 0 {
                    self.residual[u][v] -= pushed;
                    self.residual[v][u] += pushed;
                    return pushed;
                }
            }
        }
        0
    }
    /// Compute max-flow from `source` to `sink`.
    pub fn max_flow(&mut self, source: usize, sink: usize) -> i64 {
        let mut total = 0_i64;
        loop {
            let mut visited = vec![false; self.n];
            let f = self.dfs(source, sink, i64::MAX, &mut visited);
            if f == 0 {
                break;
            }
            total += f;
        }
        total
    }
}
/// Lagrangian relaxation solver using the subgradient method.
///
/// For a problem: min f(x) s.t. g(x) ≤ 0, x ∈ X
/// The Lagrangian is: L(x, λ) = f(x) + λᵀg(x)
/// The dual is: max_{λ ≥ 0} min_{x ∈ X} L(x, λ)
#[allow(dead_code)]
pub struct LagrangianRelaxationSolver {
    /// Current Lagrange multipliers (one per constraint).
    pub multipliers: Vec<f64>,
    /// Step size rule parameter.
    pub step_size: f64,
    /// Best lower bound found.
    pub best_lower_bound: f64,
    /// Iteration count.
    pub iterations: usize,
}
impl LagrangianRelaxationSolver {
    /// Create a new solver with initial multipliers all zero.
    pub fn new(num_constraints: usize, initial_step: f64) -> Self {
        Self {
            multipliers: vec![0.0; num_constraints],
            step_size: initial_step,
            best_lower_bound: f64::NEG_INFINITY,
            iterations: 0,
        }
    }
    /// Subgradient update: λ_{k+1} = max(0, λ_k + t_k * g(x_k))
    ///
    /// `subgradient\[i\]` = g_i(x_k) (constraint violation of current solution).
    /// `step_size` is the current step t_k.
    pub fn subgradient_update(&mut self, subgradient: &[f64], step: f64) {
        for (lambda, &sg) in self.multipliers.iter_mut().zip(subgradient.iter()) {
            *lambda = (*lambda + step * sg).max(0.0);
        }
        self.iterations += 1;
    }
    /// Polyak step size: t_k = (UB - L_k) / ||g||²
    ///
    /// `upper_bound` = known upper bound (primal feasible solution value).
    /// `lagrangian_value` = current Lagrangian value L_k.
    /// `subgradient` = constraint violations.
    pub fn polyak_step(&self, upper_bound: f64, lagrangian_value: f64, subgradient: &[f64]) -> f64 {
        let sg_norm_sq: f64 = subgradient.iter().map(|&g| g * g).sum();
        if sg_norm_sq < 1e-10 {
            return 0.0;
        }
        (upper_bound - lagrangian_value) / sg_norm_sq * self.step_size
    }
    /// Update best lower bound if current is better.
    pub fn update_lower_bound(&mut self, lb: f64) {
        if lb > self.best_lower_bound {
            self.best_lower_bound = lb;
        }
    }
    /// Current duality gap estimate given an upper bound.
    pub fn duality_gap(&self, upper_bound: f64) -> f64 {
        upper_bound - self.best_lower_bound
    }
}
/// A tableau-based simplex solver for LP: minimize cᵀx subject to Ax ≤ b, x ≥ 0.
///
/// Uses the standard two-phase / big-M approach in the augmented tableau form.
/// For simplicity this implementation handles only the bounded feasible case.
#[derive(Debug, Clone)]
pub struct SimplexSolver {
    /// Number of decision variables.
    pub num_vars: usize,
    /// Number of constraints (rows in A).
    pub num_constraints: usize,
    /// Objective coefficients c (length = num_vars).
    pub obj: Vec<f64>,
    /// Constraint matrix A (num_constraints × num_vars).
    pub a_matrix: Vec<Vec<f64>>,
    /// Right-hand side b (length = num_constraints).
    pub rhs: Vec<f64>,
}
impl SimplexSolver {
    /// Create a new simplex solver instance.
    pub fn new(obj: Vec<f64>, a_matrix: Vec<Vec<f64>>, rhs: Vec<f64>) -> Self {
        let num_vars = obj.len();
        let num_constraints = rhs.len();
        Self {
            num_vars,
            num_constraints,
            obj,
            a_matrix,
            rhs,
        }
    }
    /// Solve the LP using the simplex method (minimization).
    ///
    /// Uses the standard full-tableau form.  The objective row stores
    /// the current reduced costs; a negative reduced cost means the
    /// variable can improve the objective.
    ///
    /// Returns `Some(optimal_value)` if an optimal solution is found,
    /// or `None` if the problem is unbounded or infeasible.
    pub fn solve(&self) -> Option<f64> {
        let m = self.num_constraints;
        let n = self.num_vars;
        let total_cols = n + m + 1;
        let rhs_col = n + m;
        let mut tab = vec![vec![0.0_f64; total_cols]; m + 1];
        for i in 0..m {
            for j in 0..n {
                tab[i][j] = self.a_matrix[i][j];
            }
            tab[i][n + i] = 1.0;
            tab[i][rhs_col] = self.rhs[i];
        }
        for j in 0..n {
            tab[m][j] = self.obj[j];
        }
        let max_iter = 1000;
        for _ in 0..max_iter {
            let pivot_col = (0..n + m).filter(|&j| tab[m][j] < -1e-9).min_by(|&a, &b| {
                tab[m][a]
                    .partial_cmp(&tab[m][b])
                    .unwrap_or(std::cmp::Ordering::Equal)
            });
            let pivot_col = match pivot_col {
                Some(c) => c,
                None => break,
            };
            let pivot_row = (0..m)
                .filter(|&i| tab[i][pivot_col] > 1e-9)
                .min_by(|&a, &b| {
                    let ra = tab[a][rhs_col] / tab[a][pivot_col];
                    let rb = tab[b][rhs_col] / tab[b][pivot_col];
                    ra.partial_cmp(&rb).unwrap_or(std::cmp::Ordering::Equal)
                });
            let pivot_row = pivot_row?;
            let pval = tab[pivot_row][pivot_col];
            for j in 0..total_cols {
                tab[pivot_row][j] /= pval;
            }
            for i in 0..=m {
                if i != pivot_row {
                    let factor = tab[i][pivot_col];
                    if factor.abs() > 1e-15 {
                        for j in 0..total_cols {
                            let delta = factor * tab[pivot_row][j];
                            tab[i][j] -= delta;
                        }
                    }
                }
            }
        }
        Some(-tab[m][rhs_col])
    }
}
/// Dijkstra's algorithm for single-source shortest path (non-negative weights).
#[derive(Debug, Clone)]
pub struct Dijkstra {
    /// Number of vertices.
    pub n: usize,
    /// Adjacency list: adj\[u\] = `Vec<(v, weight)>`.
    pub adj: Vec<Vec<(usize, u64)>>,
}
impl Dijkstra {
    /// Create a new graph with `n` vertices.
    pub fn new(n: usize) -> Self {
        Self {
            n,
            adj: vec![Vec::new(); n],
        }
    }
    /// Add a directed edge u → v with non-negative weight.
    pub fn add_edge(&mut self, u: usize, v: usize, w: u64) {
        self.adj[u].push((v, w));
    }
    /// Compute shortest distances from `source` to all vertices.
    ///
    /// Returns `dist` where `dist\[v\] = u64::MAX` means unreachable.
    pub fn shortest_paths(&self, source: usize) -> Vec<u64> {
        let mut dist = vec![u64::MAX; self.n];
        dist[source] = 0;
        let mut heap: BinaryHeap<Reverse<(u64, usize)>> = BinaryHeap::new();
        heap.push(Reverse((0, source)));
        while let Some(Reverse((d, u))) = heap.pop() {
            if d > dist[u] {
                continue;
            }
            for &(v, w) in &self.adj[u] {
                let nd = d + w;
                if nd < dist[v] {
                    dist[v] = nd;
                    heap.push(Reverse((nd, v)));
                }
            }
        }
        dist
    }
}
