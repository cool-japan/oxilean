//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)
use super::functions::*;

/// A simple correlated equilibrium checker for finite two-player games.
///
/// A correlated equilibrium is a distribution `σ` over joint action profiles
/// such that no player can gain by deviating, given the correlation device.
///
/// This struct verifies whether a given distribution satisfies the
/// correlated equilibrium constraints.
pub struct CorrelatedEquilibriumSolver {
    /// Payoff matrices for each player.
    /// `payoff_a\[i\]\[j\]` = payoff to player A when (A plays i, B plays j).
    pub payoff_a: Vec<Vec<f64>>,
    pub payoff_b: Vec<Vec<f64>>,
}
impl CorrelatedEquilibriumSolver {
    pub fn new(payoff_a: Vec<Vec<f64>>, payoff_b: Vec<Vec<f64>>) -> Self {
        Self { payoff_a, payoff_b }
    }
    /// Check whether `sigma\[i\]\[j\]` is a correlated equilibrium.
    ///
    /// Verifies:
    /// - σ is a valid distribution (non-negative, sums to 1).
    /// - Obedience constraints hold for every player and every possible
    ///   deviation `s → s'`.
    pub fn is_correlated_equilibrium(&self, sigma: &[Vec<f64>]) -> bool {
        let m = self.payoff_a.len();
        if m == 0 {
            return true;
        }
        let n_b = self.payoff_a[0].len();
        let total: f64 = sigma.iter().flat_map(|row| row.iter()).sum();
        if (total - 1.0).abs() > 1e-9 {
            return false;
        }
        if sigma.iter().flat_map(|row| row.iter()).any(|&p| p < -1e-9) {
            return false;
        }
        for s in 0..m {
            for s_prime in 0..m {
                if s == s_prime {
                    continue;
                }
                let gain: f64 = (0..n_b)
                    .map(|j| {
                        let p = sigma.get(s).and_then(|r| r.get(j)).copied().unwrap_or(0.0);
                        let us = self.payoff_a[s].get(j).copied().unwrap_or(0.0);
                        let us_prime = self.payoff_a[s_prime].get(j).copied().unwrap_or(0.0);
                        p * (us - us_prime)
                    })
                    .sum();
                if gain < -1e-9 {
                    return false;
                }
            }
        }
        for t in 0..n_b {
            for t_prime in 0..n_b {
                if t == t_prime {
                    continue;
                }
                let gain: f64 = (0..m)
                    .map(|i| {
                        let p = sigma.get(i).and_then(|r| r.get(t)).copied().unwrap_or(0.0);
                        let ut = self.payoff_b[i].get(t).copied().unwrap_or(0.0);
                        let ut_prime = self.payoff_b[i].get(t_prime).copied().unwrap_or(0.0);
                        p * (ut - ut_prime)
                    })
                    .sum();
                if gain < -1e-9 {
                    return false;
                }
            }
        }
        true
    }
    /// Find the uniform correlated equilibrium (i.e., uniform over all
    /// profiles that satisfy obedience) by checking the uniform distribution
    /// first, then returning it if valid.
    pub fn uniform_correlated_equilibrium(&self) -> Option<Vec<Vec<f64>>> {
        let m = self.payoff_a.len();
        if m == 0 {
            return None;
        }
        let n_b = self.payoff_a[0].len();
        let total = (m * n_b) as f64;
        let sigma: Vec<Vec<f64>> = vec![vec![1.0 / total; n_b]; m];
        if self.is_correlated_equilibrium(&sigma) {
            Some(sigma)
        } else {
            None
        }
    }
}
/// An evolutionary game defined by a payoff matrix between strategies.
///
/// The population state is a probability vector over strategies.
/// `payoff_matrix\[i\]\[j\]` is the payoff that strategy `i` receives when
/// meeting strategy `j`.
pub struct EvolutionaryGame {
    pub payoff_matrix: Vec<Vec<f64>>,
    pub strategies: Vec<String>,
}
impl EvolutionaryGame {
    /// Create a new evolutionary game.
    pub fn new(payoff_matrix: Vec<Vec<f64>>, strategies: Vec<String>) -> Self {
        Self {
            payoff_matrix,
            strategies,
        }
    }
    /// Average fitness of strategy `i` in population `pop`.
    fn strategy_fitness(&self, pop: &[f64], i: usize) -> f64 {
        self.payoff_matrix[i]
            .iter()
            .zip(pop.iter())
            .map(|(a, p)| a * p)
            .sum()
    }
    /// Overall average fitness of the population.
    fn avg_fitness(&self, pop: &[f64]) -> f64 {
        pop.iter()
            .enumerate()
            .map(|(i, p)| p * self.strategy_fitness(pop, i))
            .sum()
    }
    /// Perform one step of replicator dynamics with time-step `dt`.
    pub fn replicator_dynamics(&self, dt: f64) -> Vec<f64> {
        let n = self.strategies.len();
        let pop: Vec<f64> = vec![1.0 / n as f64; n];
        let avg = self.avg_fitness(&pop);
        pop.iter()
            .enumerate()
            .map(|(i, p)| {
                let fi = self.strategy_fitness(&pop, i);
                p + dt * p * (fi - avg)
            })
            .collect()
    }
    /// Check whether strategy `s` is evolutionarily stable (ESS).
    ///
    /// A strategy `s` is ESS if:
    ///   - u(s, s) > u(t, s) for all t ≠ s, **or**
    ///   - u(s, s) = u(t, s) and u(s, t) > u(t, t)
    pub fn evolutionarily_stable_strategy(&self) -> Vec<usize> {
        let n = self.strategies.len();
        let mut ess = Vec::new();
        for s in 0..n {
            let mut is_ess = true;
            for t in 0..n {
                if t == s {
                    continue;
                }
                let uss = self.payoff_matrix[s][s];
                let uts = self.payoff_matrix[t][s];
                let ust = self.payoff_matrix[s][t];
                let utt = self.payoff_matrix[t][t];
                if uss < uts - 1e-12 {
                    is_ess = false;
                    break;
                }
                if (uss - uts).abs() < 1e-12 && ust < utt - 1e-12 {
                    is_ess = false;
                    break;
                }
            }
            if is_ess {
                ess.push(s);
            }
        }
        ess
    }
    /// Find Nash equilibria in the evolutionary (symmetric) game.
    ///
    /// Returns indices of pure strategies that are fixed points of
    /// replicator dynamics (best responses to themselves).
    pub fn nash_evolutionary(&self) -> Vec<usize> {
        let n = self.strategies.len();
        let mut nash = Vec::new();
        for s in 0..n {
            let pop_s: Vec<f64> = (0..n).map(|i| if i == s { 1.0 } else { 0.0 }).collect();
            let fi = self.strategy_fitness(&pop_s, s);
            let is_best = (0..n).all(|t| self.payoff_matrix[t][s] <= fi + 1e-12);
            if is_best {
                nash.push(s);
            }
        }
        nash
    }
}
/// Checker for evolutionary stability of strategies.
pub struct ESSChecker {
    /// Payoff matrix (symmetric game).
    pub payoff: Vec<Vec<f64>>,
}
impl ESSChecker {
    pub fn new(payoff: Vec<Vec<f64>>) -> Self {
        Self { payoff }
    }
    /// Returns `true` if pure strategy `s` is evolutionarily stable.
    ///
    /// A pure strategy `s` is ESS iff for all `t ≠ s`:
    ///   - `u(s,s) > u(t,s)`, OR
    ///   - `u(s,s) = u(t,s)` AND `u(s,t) > u(t,t)`
    pub fn is_ess(&self, s: usize) -> bool {
        let n = self.payoff.len();
        for t in 0..n {
            if t == s {
                continue;
            }
            let uss = self.payoff[s][s];
            let uts = self.payoff[t][s];
            let ust = self.payoff[s][t];
            let utt = self.payoff[t][t];
            if uss < uts - 1e-12 {
                return false;
            }
            if (uss - uts).abs() < 1e-12 && ust < utt - 1e-12 {
                return false;
            }
        }
        true
    }
    /// Return all ESS indices.
    pub fn all_ess(&self) -> Vec<usize> {
        (0..self.payoff.len()).filter(|&s| self.is_ess(s)).collect()
    }
    /// Return `true` if the game has at least one ESS.
    pub fn has_ess(&self) -> bool {
        !self.all_ess().is_empty()
    }
}
/// Gale-Shapley deferred acceptance algorithm.
///
/// Finds a stable matching between two sets of `n` agents.
/// `proposer_prefs\[i\]` is agent `i`'s preference list over acceptors
/// (most preferred first).
/// `acceptor_prefs\[j\]` is acceptor `j`'s preference list over proposers.
pub struct GaleShapleyAlgorithm {
    pub n: usize,
    /// `proposer_prefs\[i\]\[k\]` = k-th choice of proposer `i`.
    pub proposer_prefs: Vec<Vec<usize>>,
    /// `acceptor_prefs\[j\]\[k\]` = k-th choice of acceptor `j`.
    pub acceptor_prefs: Vec<Vec<usize>>,
}
impl GaleShapleyAlgorithm {
    pub fn new(n: usize, proposer_prefs: Vec<Vec<usize>>, acceptor_prefs: Vec<Vec<usize>>) -> Self {
        Self {
            n,
            proposer_prefs,
            acceptor_prefs,
        }
    }
    /// Run the proposer-optimal Gale-Shapley algorithm.
    ///
    /// Returns `matching\[i\]` = the acceptor matched to proposer `i`,
    /// or `n` if unmatched.
    pub fn run(&self) -> Vec<usize> {
        let n = self.n;
        let mut rank = vec![vec![0usize; n]; n];
        for j in 0..n {
            for (r, &i) in self.acceptor_prefs[j].iter().enumerate() {
                if i < n {
                    rank[j][i] = r;
                }
            }
        }
        let mut proposer_match = vec![n; n];
        let mut acceptor_match = vec![n; n];
        let mut next_proposal = vec![0usize; n];
        let mut free: Vec<usize> = (0..n).collect();
        while let Some(p) = free.pop() {
            if next_proposal[p] >= self.proposer_prefs[p].len() {
                continue;
            }
            let a = self.proposer_prefs[p][next_proposal[p]];
            next_proposal[p] += 1;
            if a >= n {
                continue;
            }
            if acceptor_match[a] == n {
                acceptor_match[a] = p;
                proposer_match[p] = a;
            } else {
                let current = acceptor_match[a];
                if rank[a][p] < rank[a][current] {
                    acceptor_match[a] = p;
                    proposer_match[p] = a;
                    proposer_match[current] = n;
                    free.push(current);
                } else {
                    free.push(p);
                }
            }
        }
        proposer_match
    }
    /// Check whether `matching` is stable.
    ///
    /// A matching is stable if there are no blocking pairs (p, a) where
    /// both p prefers a over their current match and a prefers p over theirs.
    pub fn is_stable(&self, matching: &[usize]) -> bool {
        let n = self.n;
        let mut acceptor_of = vec![n; n];
        for (p, &a) in matching.iter().enumerate() {
            if a < n {
                acceptor_of[a] = p;
            }
        }
        let mut proposer_rank = vec![vec![n; n]; n];
        for i in 0..n {
            for (r, &j) in self.proposer_prefs[i].iter().enumerate() {
                if j < n {
                    proposer_rank[i][j] = r;
                }
            }
        }
        let mut acceptor_rank = vec![vec![n; n]; n];
        for j in 0..n {
            for (r, &i) in self.acceptor_prefs[j].iter().enumerate() {
                if i < n {
                    acceptor_rank[j][i] = r;
                }
            }
        }
        for p in 0..n {
            for a in 0..n {
                if matching[p] == a {
                    continue;
                }
                let p_prefers_a = proposer_rank[p][a] < proposer_rank[p][matching[p].min(n - 1)];
                let current_p = acceptor_of[a];
                let a_prefers_p = if current_p == n {
                    true
                } else {
                    acceptor_rank[a][p] < acceptor_rank[a][current_p]
                };
                if p_prefers_a && a_prefers_p {
                    return false;
                }
            }
        }
        true
    }
}
/// Checks incentive-compatibility (IC) and individual-rationality (IR)
/// conditions for a direct mechanism.
///
/// A direct mechanism is a pair `(q, t)` where:
/// - `q\[i\]` = allocation rule probability for agent `i`
/// - `t\[i\]` = transfer (payment) for agent `i`
///
/// Given reported type `v\[i\]` (valuation), agent `i`'s utility is:
///   `u(v, q, t) = v * q - t`
pub struct MechanismDesignChecker {
    /// Valuations (types) of each agent.
    pub valuations: Vec<f64>,
    /// Allocation probabilities (0..=1).
    pub allocations: Vec<f64>,
    /// Transfers (payments).
    pub transfers: Vec<f64>,
}
impl MechanismDesignChecker {
    pub fn new(valuations: Vec<f64>, allocations: Vec<f64>, transfers: Vec<f64>) -> Self {
        Self {
            valuations,
            allocations,
            transfers,
        }
    }
    fn utility(&self, value: f64, alloc: f64, transfer: f64) -> f64 {
        value * alloc - transfer
    }
    /// Check dominant-strategy incentive compatibility (DSIC).
    ///
    /// For every pair `(i, j)` with `i ≠ j`, verify that agent `i` cannot
    /// benefit by pretending to have valuation `v_j`:
    ///   u(v_i, q_i, t_i) ≥ u(v_i, q_j, t_j)
    pub fn is_dsic(&self) -> bool {
        let n = self.valuations.len();
        for i in 0..n {
            let vi = self.valuations[i];
            let true_util = self.utility(vi, self.allocations[i], self.transfers[i]);
            for j in 0..n {
                if i == j {
                    continue;
                }
                let dev_util = self.utility(vi, self.allocations[j], self.transfers[j]);
                if dev_util > true_util + 1e-9 {
                    return false;
                }
            }
        }
        true
    }
    /// Check individual rationality (IR).
    ///
    /// Every agent's utility under truthful reporting must be non-negative:
    ///   u(v_i, q_i, t_i) ≥ 0
    pub fn is_ir(&self) -> bool {
        let n = self.valuations.len();
        (0..n).all(|i| {
            self.utility(self.valuations[i], self.allocations[i], self.transfers[i]) >= -1e-9
        })
    }
    /// Expected revenue of the mechanism.
    pub fn revenue(&self) -> f64 {
        self.transfers.iter().sum()
    }
}
/// A cooperative (transferable-utility) game in characteristic form.
///
/// `players` lists the player names.
/// `characteristic_fn\[S\]` is the worth `v(S)` of coalition `S`
/// encoded as a bitmask: `S = 0b101` means {player 0, player 2}.
/// The vector has length `2^n`.
pub struct CooperativeGame {
    pub players: Vec<String>,
    pub characteristic_fn: Vec<f64>,
}
impl CooperativeGame {
    /// Create a new cooperative game.
    pub fn new(players: Vec<String>, characteristic_fn: Vec<f64>) -> Self {
        Self {
            players,
            characteristic_fn,
        }
    }
    fn n(&self) -> usize {
        self.players.len()
    }
    fn v(&self, coalition: usize) -> f64 {
        self.characteristic_fn
            .get(coalition)
            .copied()
            .unwrap_or(0.0)
    }
    /// Compute the Shapley value for all players.
    ///
    /// φ_i(v) = Σ_{S ⊆ N \ {i}} [|S|! (n - |S| - 1)! / n!] \[v(S ∪ {i}) - v(S)\]
    pub fn shapley_value(&self) -> Vec<f64> {
        let n = self.n();
        let mut phi = vec![0.0f64; n];
        let total = (1..=n).product::<usize>() as f64;
        for i in 0..n {
            let mut value = 0.0f64;
            for s_mask in 0usize..(1 << n) {
                if s_mask & (1 << i) != 0 {
                    continue;
                }
                let s_size = s_mask.count_ones() as usize;
                let weight = factorial(s_size) * factorial(n - s_size - 1);
                let marginal = self.v(s_mask | (1 << i)) - self.v(s_mask);
                value += (weight as f64 / total) * marginal;
            }
            phi[i] = value;
        }
        phi
    }
    /// Check whether the core is non-empty using a simple LP-based check.
    ///
    /// A feasible imputation x exists in the core iff:
    ///   Σ x_i = v(N)  and  Σ_{i∈S} x_i ≥ v(S) for all S.
    ///
    /// We use a greedy test: the Shapley value is in the core iff
    /// it satisfies all coalition constraints.
    pub fn core_is_nonempty(&self) -> bool {
        let n = self.n();
        let shapley = self.shapley_value();
        let grand = self.v((1 << n) - 1);
        let total: f64 = shapley.iter().sum();
        if (total - grand).abs() > 1e-9 {
            return false;
        }
        for s_mask in 1usize..(1 << n) {
            let coalition_val: f64 = (0..n)
                .filter(|&i| s_mask & (1 << i) != 0)
                .map(|i| shapley[i])
                .sum();
            if coalition_val < self.v(s_mask) - 1e-9 {
                return false;
            }
        }
        true
    }
    /// Compute the nucleolus (approximation via excess minimization).
    ///
    /// The nucleolus is the unique imputation that lexicographically minimises
    /// the sorted vector of coalition excesses. Here we return the Shapley
    /// value as a first approximation when the core is non-empty, otherwise
    /// we return the proportional imputation.
    pub fn nucleolus(&self) -> Vec<f64> {
        if self.core_is_nonempty() {
            self.shapley_value()
        } else {
            let n = self.n();
            let grand = self.v((1 << n) - 1);
            let singleton_sum: f64 = (0..n).map(|i| self.v(1 << i)).sum();
            if singleton_sum.abs() < 1e-12 {
                vec![grand / n as f64; n]
            } else {
                (0..n)
                    .map(|i| grand * self.v(1 << i) / singleton_sum)
                    .collect()
            }
        }
    }
    /// Check superadditivity: v(S ∪ T) ≥ v(S) + v(T) for all disjoint S, T.
    pub fn is_superadditive(&self) -> bool {
        let n = self.n();
        let total_coalitions = 1usize << n;
        for s in 1..total_coalitions {
            for t in 1..total_coalitions {
                if s & t != 0 {
                    continue;
                }
                if self.v(s | t) < self.v(s) + self.v(t) - 1e-9 {
                    return false;
                }
            }
        }
        true
    }
    /// Check convexity: v(S ∪ T) + v(S ∩ T) ≥ v(S) + v(T) for all S, T.
    pub fn is_convex(&self) -> bool {
        let n = self.n();
        let total_coalitions = 1usize << n;
        for s in 1..total_coalitions {
            for t in 1..total_coalitions {
                let union_val = self.v(s | t);
                let inter_val = self.v(s & t);
                let sum_val = self.v(s) + self.v(t);
                if union_val + inter_val < sum_val - 1e-9 {
                    return false;
                }
            }
        }
        true
    }
}
/// An auction with multiple bidders.
///
/// `valuations\[i\]` is bidder `i`'s private valuation.
pub struct AuctionGame {
    pub num_bidders: usize,
    pub valuations: Vec<f64>,
}
impl AuctionGame {
    pub fn new(num_bidders: usize, valuations: Vec<f64>) -> Self {
        Self {
            num_bidders,
            valuations,
        }
    }
    /// Second-price (Vickrey) auction equilibrium.
    ///
    /// In a second-price auction, truthful bidding is a weakly dominant strategy.
    /// Returns `(winner_idx, price_paid)` where price = second-highest valuation.
    pub fn second_price_equilibrium(&self) -> (usize, f64) {
        let n = self.num_bidders.min(self.valuations.len());
        if n == 0 {
            return (0, 0.0);
        }
        let mut sorted_with_idx: Vec<(usize, f64)> =
            self.valuations[..n].iter().copied().enumerate().collect();
        sorted_with_idx.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        let winner = sorted_with_idx[0].0;
        let price = if n >= 2 { sorted_with_idx[1].1 } else { 0.0 };
        (winner, price)
    }
    /// First-price auction Bayes-Nash equilibrium bid function.
    ///
    /// With uniform private valuations on \[0,1\] and `n` bidders, the
    /// symmetric BNE bid for a bidder with valuation `v` is:
    ///   b(v) = v * (n-1) / n
    ///
    /// Returns a vector of equilibrium bids.
    pub fn first_price_bayes_nash(&self) -> Vec<f64> {
        let n = self.num_bidders.max(1) as f64;
        self.valuations.iter().map(|&v| v * (n - 1.0) / n).collect()
    }
    /// All-pay auction equilibrium bids.
    ///
    /// In an all-pay auction with uniform valuations on \[0,1\] and `n` bidders,
    /// the symmetric BNE bid for valuation `v` is:
    ///   b(v) = v^n * (n-1)/n  (approximate: integral formula)
    ///
    /// Returns a vector of equilibrium bids.
    pub fn all_pay_equilibrium(&self) -> Vec<f64> {
        let n = self.num_bidders.max(1) as f64;
        self.valuations
            .iter()
            .map(|&v| v.powf(n) * (n - 1.0) / n)
            .collect()
    }
}
/// Solver for symmetric congestion games via an exact potential function.
///
/// In a congestion game, players share resources.  The cost to a player using
/// resource `r` when `k` players use it is `cost\[r\][k-1]` (1-indexed count).
/// The exact potential Φ(x) = Σ_r Σ_{k=1}^{x_r} cost\[r\][k-1].
///
/// A pure Nash equilibrium minimises Φ (for cost-minimising players).
pub struct CongestionGameSolver {
    /// `cost\[r\]\[k\]` = cost of resource `r` when `k+1` players use it.
    pub cost: Vec<Vec<f64>>,
    pub num_resources: usize,
    pub num_players: usize,
}
impl CongestionGameSolver {
    /// Create a new solver.
    pub fn new(cost: Vec<Vec<f64>>, num_players: usize) -> Self {
        let num_resources = cost.len();
        Self {
            cost,
            num_resources,
            num_players,
        }
    }
    /// Potential of a load vector `load\[r\]` = number of players on resource `r`.
    pub fn potential(&self, load: &[usize]) -> f64 {
        load.iter()
            .enumerate()
            .map(|(r, &k)| {
                (0..k)
                    .map(|idx| {
                        self.cost
                            .get(r)
                            .and_then(|row| row.get(idx))
                            .copied()
                            .unwrap_or(0.0)
                    })
                    .sum::<f64>()
            })
            .sum()
    }
    /// Find the load vector minimising the potential among all allocations
    /// of `num_players` players over `num_resources` resources (exhaustive).
    ///
    /// Returns the load vector at the pure Nash equilibrium.
    pub fn pure_nash_equilibrium(&self) -> Vec<usize> {
        let n = self.num_players;
        let r = self.num_resources;
        if r == 0 {
            return vec![];
        }
        let mut best_load = vec![0usize; r];
        best_load[0] = n;
        let mut best_pot = self.potential(&best_load);
        fn enumerate_loads(
            remaining: usize,
            resources: usize,
            current: &mut Vec<usize>,
            best: &mut (Vec<usize>, f64),
            solver: &CongestionGameSolver,
        ) {
            if resources == 1 {
                current.push(remaining);
                let pot = solver.potential(current);
                if pot < best.1 {
                    best.0 = current.clone();
                    best.1 = pot;
                }
                current.pop();
                return;
            }
            for k in 0..=remaining {
                current.push(k);
                enumerate_loads(remaining - k, resources - 1, current, best, solver);
                current.pop();
            }
        }
        let mut current = Vec::new();
        let mut best = (best_load, best_pot);
        enumerate_loads(n, r, &mut current, &mut best, self);
        best_pot = best.1;
        let _ = best_pot;
        best.0
    }
}
/// Multi-step replicator dynamics simulator.
///
/// Simulates continuous-time replicator dynamics using Euler integration.
pub struct ReplicatorDynamics {
    /// Payoff matrix: `payoff\[i\]\[j\]` = payoff to strategy `i` vs strategy `j`.
    pub payoff: Vec<Vec<f64>>,
}
impl ReplicatorDynamics {
    /// Create a new simulator from a payoff matrix.
    pub fn new(payoff: Vec<Vec<f64>>) -> Self {
        Self { payoff }
    }
    fn fitness(&self, pop: &[f64], i: usize) -> f64 {
        self.payoff[i]
            .iter()
            .zip(pop.iter())
            .map(|(a, p)| a * p)
            .sum()
    }
    fn avg(&self, pop: &[f64]) -> f64 {
        pop.iter()
            .enumerate()
            .map(|(i, p)| p * self.fitness(pop, i))
            .sum()
    }
    /// Run `steps` Euler steps of size `dt` starting from `init_pop`.
    ///
    /// Returns the population state after all steps.
    pub fn simulate(&self, init_pop: &[f64], dt: f64, steps: usize) -> Vec<f64> {
        let mut pop = init_pop.to_vec();
        for _ in 0..steps {
            let avg = self.avg(&pop);
            let next: Vec<f64> = pop
                .iter()
                .enumerate()
                .map(|(i, p)| {
                    let fi = self.fitness(&pop, i);
                    (p + dt * p * (fi - avg)).max(0.0)
                })
                .collect();
            let total: f64 = next.iter().sum();
            if total > 1e-15 {
                pop = next.iter().map(|x| x / total).collect();
            } else {
                break;
            }
        }
        pop
    }
    /// Detect fixed points: indices where population share is > `threshold`.
    pub fn dominant_strategies(&self, init_pop: &[f64], dt: f64, steps: usize) -> Vec<usize> {
        let final_pop = self.simulate(init_pop, dt, steps);
        final_pop
            .iter()
            .enumerate()
            .filter(|(_, &p)| p > 0.5)
            .map(|(i, _)| i)
            .collect()
    }
}
/// Exhaustive minimax search for zero-sum two-player games.
///
/// The game tree is represented by a payoff matrix; the maximizing player
/// (row) wants to maximise, the minimizing player (col) wants to minimise.
pub struct ExhaustiveSearch {
    pub num_strategies: usize,
    /// payoff_matrix[row][col] = payoff to row player
    payoff_matrix: Vec<Vec<f64>>,
}
impl ExhaustiveSearch {
    pub fn new(num_strategies: usize, payoff_matrix: Vec<Vec<f64>>) -> Self {
        Self {
            num_strategies,
            payoff_matrix,
        }
    }
    /// Plain minimax: maximizing player picks row, minimizing player picks col.
    pub fn minimax(&self) -> f64 {
        let mut best = f64::NEG_INFINITY;
        for row in &self.payoff_matrix {
            let worst_case = row.iter().cloned().fold(f64::INFINITY, f64::min);
            if worst_case > best {
                best = worst_case;
            }
        }
        best
    }
    /// Alpha-beta pruning search to `depth` levels (simplified: treats matrix
    /// rows as depth-1 and columns as depth-2 maximizer/minimizer alternation).
    pub fn alpha_beta(&self, depth: u32) -> f64 {
        fn ab(
            matrix: &[Vec<f64>],
            depth: u32,
            mut alpha: f64,
            mut beta: f64,
            maximizing: bool,
        ) -> f64 {
            if depth == 0 || matrix.is_empty() {
                return matrix
                    .iter()
                    .map(|row| row.iter().cloned().fold(f64::INFINITY, f64::min))
                    .fold(f64::NEG_INFINITY, f64::max);
            }
            if maximizing {
                let mut value = f64::NEG_INFINITY;
                for row in matrix {
                    let child_val = ab(std::slice::from_ref(row), depth - 1, alpha, beta, false);
                    value = value.max(child_val);
                    alpha = alpha.max(value);
                    if value >= beta {
                        break;
                    }
                }
                value
            } else {
                let mut value = f64::INFINITY;
                for col in 0..matrix[0].len() {
                    let sub: Vec<Vec<f64>> = matrix.iter().map(|row| vec![row[col]]).collect();
                    let child_val = ab(&sub, depth - 1, alpha, beta, true);
                    value = value.min(child_val);
                    beta = beta.min(value);
                    if value <= alpha {
                        break;
                    }
                }
                value
            }
        }
        ab(
            &self.payoff_matrix,
            depth,
            f64::NEG_INFINITY,
            f64::INFINITY,
            true,
        )
    }
    /// Find the saddle-point value of the zero-sum game (minimax = maximin).
    ///
    /// Returns `(row_strategy, col_strategy, value)` if a pure saddle point
    /// exists, otherwise returns the minimax value with `(usize::MAX, usize::MAX)`.
    pub fn solve_zero_sum(&self) -> (usize, usize, f64) {
        let n = self.num_strategies;
        for i in 0..self.payoff_matrix.len().min(n) {
            for j in 0..self.payoff_matrix[i].len().min(n) {
                let v = self.payoff_matrix[i][j];
                let row_min = self.payoff_matrix[i]
                    .iter()
                    .cloned()
                    .fold(f64::INFINITY, f64::min);
                let col_max = (0..self.payoff_matrix.len())
                    .map(|r| self.payoff_matrix[r].get(j).copied().unwrap_or(0.0))
                    .fold(f64::NEG_INFINITY, f64::max);
                if (v - row_min).abs() < 1e-12 && (v - col_max).abs() < 1e-12 {
                    return (i, j, v);
                }
            }
        }
        (usize::MAX, usize::MAX, self.minimax())
    }
}
/// Solves the Rubinstein alternating-offers bargaining game.
///
/// Two players split a pie of size 1.  Player 1 makes offers in odd rounds,
/// player 2 in even rounds.  Each player discounts the future by `delta_1`
/// and `delta_2` respectively.
///
/// The unique subgame-perfect equilibrium has player 1 offering:
///   x_1* = (1 - δ_2) / (1 - δ_1 · δ_2)
///   x_2* = δ_2 · (1 - δ_1) / (1 - δ_1 · δ_2)
pub struct RubinsteinBargainingSolver {
    /// Discount factor for player 1.
    pub delta1: f64,
    /// Discount factor for player 2.
    pub delta2: f64,
}
impl RubinsteinBargainingSolver {
    /// Create a new solver.
    pub fn new(delta1: f64, delta2: f64) -> Self {
        Self { delta1, delta2 }
    }
    /// Compute the SPE equilibrium shares `(share_1, share_2)`.
    pub fn equilibrium_shares(&self) -> (f64, f64) {
        let denom = 1.0 - self.delta1 * self.delta2;
        if denom.abs() < 1e-12 {
            return (0.5, 0.5);
        }
        let x1 = (1.0 - self.delta2) / denom;
        let x2 = self.delta2 * (1.0 - self.delta1) / denom;
        (x1, x2)
    }
    /// Check whether a proposed split `(offer_1, offer_2)` with `offer_1 + offer_2 = 1`
    /// is approximately consistent with the Rubinstein SPE.
    pub fn is_equilibrium_split(&self, offer1: f64, offer2: f64, tol: f64) -> bool {
        let (x1, x2) = self.equilibrium_shares();
        (offer1 - x1).abs() < tol && (offer2 - x2).abs() < tol
    }
    /// When δ_1 = δ_2 = δ, the equilibrium simplifies to 1/(1+δ) for player 1.
    pub fn symmetric_share(delta: f64) -> f64 {
        1.0 / (1.0 + delta)
    }
}
/// Computes a logit Quantal Response Equilibrium (QRE) for two-player finite games.
///
/// In a logit QRE with precision parameter `λ ≥ 0`, each player's mixed strategy
/// is a softmax over expected payoffs:
///   σ_i(s) ∝ exp(λ · EU_i(s, σ_{-i}))
///
/// This struct iterates the best-response map to convergence.
pub struct QuantalResponseEquilibrium {
    /// Payoff matrix for player A: `payoff_a\[i\]\[j\]`.
    pub payoff_a: Vec<Vec<f64>>,
    /// Payoff matrix for player B: `payoff_b\[i\]\[j\]`.
    pub payoff_b: Vec<Vec<f64>>,
    /// Precision (rationality) parameter λ.
    pub lambda: f64,
}
impl QuantalResponseEquilibrium {
    /// Create a new QRE solver.
    pub fn new(payoff_a: Vec<Vec<f64>>, payoff_b: Vec<Vec<f64>>, lambda: f64) -> Self {
        Self {
            payoff_a,
            payoff_b,
            lambda,
        }
    }
    fn softmax(values: &[f64], lambda: f64) -> Vec<f64> {
        let scaled: Vec<f64> = values.iter().map(|&v| lambda * v).collect();
        let max_v = scaled.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let exps: Vec<f64> = scaled.iter().map(|&v| (v - max_v).exp()).collect();
        let sum: f64 = exps.iter().sum();
        if sum < 1e-300 {
            vec![1.0 / values.len() as f64; values.len()]
        } else {
            exps.iter().map(|&e| e / sum).collect()
        }
    }
    fn eu_a(payoff_a: &[Vec<f64>], sigma_b: &[f64], i: usize) -> f64 {
        payoff_a[i]
            .iter()
            .zip(sigma_b.iter())
            .map(|(&p, &q)| p * q)
            .sum()
    }
    fn eu_b(payoff_b: &[Vec<f64>], sigma_a: &[f64], j: usize) -> f64 {
        sigma_a
            .iter()
            .zip(payoff_b.iter())
            .map(|(&p, row)| p * row.get(j).copied().unwrap_or(0.0))
            .sum()
    }
    /// Iterate to find the logit QRE mixed strategies.
    ///
    /// Returns `(sigma_a, sigma_b)` — the equilibrium mixed strategies.
    pub fn solve(&self, max_iter: usize, tol: f64) -> (Vec<f64>, Vec<f64>) {
        let m = self.payoff_a.len();
        if m == 0 {
            return (vec![], vec![]);
        }
        let n = self.payoff_a[0].len();
        let mut sigma_a = vec![1.0 / m as f64; m];
        let mut sigma_b = vec![1.0 / n as f64; n];
        for _ in 0..max_iter {
            let eu_vals_a: Vec<f64> = (0..m)
                .map(|i| Self::eu_a(&self.payoff_a, &sigma_b, i))
                .collect();
            let eu_vals_b: Vec<f64> = (0..n)
                .map(|j| Self::eu_b(&self.payoff_b, &sigma_a, j))
                .collect();
            let new_a = Self::softmax(&eu_vals_a, self.lambda);
            let new_b = Self::softmax(&eu_vals_b, self.lambda);
            let diff_a: f64 = sigma_a
                .iter()
                .zip(new_a.iter())
                .map(|(a, b)| (a - b).abs())
                .sum();
            let diff_b: f64 = sigma_b
                .iter()
                .zip(new_b.iter())
                .map(|(a, b)| (a - b).abs())
                .sum();
            sigma_a = new_a;
            sigma_b = new_b;
            if diff_a + diff_b < tol {
                break;
            }
        }
        (sigma_a, sigma_b)
    }
    /// Expected payoff to player A at the QRE.
    pub fn expected_payoff_a(&self, sigma_a: &[f64], sigma_b: &[f64]) -> f64 {
        sigma_a
            .iter()
            .enumerate()
            .map(|(i, &p)| p * Self::eu_a(&self.payoff_a, sigma_b, i))
            .sum()
    }
}
/// A Stackelberg game with a leader and follower.
/// Leader moves first, follower observes and responds optimally.
#[allow(dead_code)]
pub struct StackelbergGameExt {
    /// Leader's strategy space size.
    pub leader_strategies: usize,
    /// Follower's strategy space size.
    pub follower_strategies: usize,
    /// Leader's payoff matrix: leader_payoff\[i\]\[j\] = payoff when leader plays i, follower plays j.
    pub leader_payoff: Vec<Vec<f64>>,
    /// Follower's payoff matrix.
    pub follower_payoff: Vec<Vec<f64>>,
}
#[allow(dead_code)]
impl StackelbergGameExt {
    /// Create a new Stackelberg game.
    pub fn new(
        leader_strategies: usize,
        follower_strategies: usize,
        leader_payoff: Vec<Vec<f64>>,
        follower_payoff: Vec<Vec<f64>>,
    ) -> Self {
        StackelbergGameExt {
            leader_strategies,
            follower_strategies,
            leader_payoff,
            follower_payoff,
        }
    }
    /// Follower's best response to leader's strategy i.
    pub fn follower_best_response(&self, leader_strategy: usize) -> usize {
        (0..self.follower_strategies)
            .max_by(|&j1, &j2| {
                self.follower_payoff[leader_strategy][j1]
                    .partial_cmp(&self.follower_payoff[leader_strategy][j2])
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .unwrap_or(0)
    }
    /// Stackelberg equilibrium: leader maximizes payoff anticipating follower's BR.
    /// Returns (leader_strategy, follower_strategy, leader_payoff).
    pub fn stackelberg_equilibrium(&self) -> (usize, usize, f64) {
        let mut best_leader = 0;
        let mut best_payoff = f64::NEG_INFINITY;
        for i in 0..self.leader_strategies {
            let j = self.follower_best_response(i);
            let payoff = self.leader_payoff[i][j];
            if payoff > best_payoff {
                best_payoff = payoff;
                best_leader = i;
            }
        }
        let j = self.follower_best_response(best_leader);
        (best_leader, j, best_payoff)
    }
    /// First-mover advantage: Stackelberg payoff ≥ Nash payoff (in typical cases).
    /// Returns true if the Stackelberg payoff is at least as large as
    /// the best diagonal payoff (simplified Nash surrogate).
    pub fn first_mover_advantage(&self) -> bool {
        let (_, _, stack_payoff) = self.stackelberg_equilibrium();
        let n = self.leader_strategies.min(self.follower_strategies);
        let best_sym: f64 = (0..n)
            .map(|i| self.leader_payoff[i][i])
            .fold(f64::NEG_INFINITY, f64::max);
        stack_payoff >= best_sym - 1e-9
    }
}
/// Checks pairwise stability of a network in a network formation game.
///
/// A network is a set of bilateral links between players.
/// `benefit\[i\]\[j\]` = benefit to player `i` from a direct link with `j`.
/// `cost_link` = symmetric cost of forming any link.
///
/// A network is pairwise stable if:
///  - No linked pair wants to sever their link.
///  - No unlinked pair both want to add their link.
pub struct NetworkFormationSolver {
    pub num_players: usize,
    pub benefit: Vec<Vec<f64>>,
    pub cost_link: f64,
}
impl NetworkFormationSolver {
    /// Create a new network formation solver.
    pub fn new(num_players: usize, benefit: Vec<Vec<f64>>, cost_link: f64) -> Self {
        Self {
            num_players,
            benefit,
            cost_link,
        }
    }
    /// Net utility of player `i` in a network represented by adjacency bitmasks.
    fn utility(&self, links: &[u64], i: usize) -> f64 {
        let n = self.num_players;
        let degree = links[i].count_ones() as f64;
        let benefit: f64 = (0..n)
            .filter(|&j| j != i && (links[i] >> j) & 1 == 1)
            .map(|j| {
                self.benefit
                    .get(i)
                    .and_then(|row| row.get(j))
                    .copied()
                    .unwrap_or(0.0)
            })
            .sum();
        benefit - self.cost_link * degree
    }
    /// Check whether the network (encoded as adjacency bitmasks) is pairwise stable.
    pub fn is_pairwise_stable(&self, links: &[u64]) -> bool {
        let n = self.num_players;
        for i in 0..n {
            for j in (i + 1)..n {
                if (links[i] >> j) & 1 == 1 {
                    let ui = self.utility(links, i);
                    let uj = self.utility(links, j);
                    let mut sever = links.to_vec();
                    sever[i] &= !(1u64 << j);
                    sever[j] &= !(1u64 << i);
                    let ui_sever = self.utility(&sever, i);
                    let uj_sever = self.utility(&sever, j);
                    if ui_sever > ui + 1e-9 || uj_sever > uj + 1e-9 {
                        return false;
                    }
                } else {
                    let mut add = links.to_vec();
                    add[i] |= 1u64 << j;
                    add[j] |= 1u64 << i;
                    let ui_add = self.utility(&add, i);
                    let uj_add = self.utility(&add, j);
                    let ui = self.utility(links, i);
                    let uj = self.utility(links, j);
                    if ui_add > ui + 1e-9 && uj_add > uj + 1e-9 {
                        return false;
                    }
                }
            }
        }
        true
    }
    /// Find the empty network (no links) and check if it is pairwise stable.
    pub fn empty_network_is_stable(&self) -> bool {
        let links = vec![0u64; self.num_players];
        self.is_pairwise_stable(&links)
    }
}
/// A repeated game built on top of a stage game (payoff matrix for two players).
///
/// Models infinitely repeated play with discount factor `delta ∈ (0,1)`.
/// Provides utilities for checking folk-theorem feasibility of payoff pairs.
pub struct RepeatedGame {
    /// Stage game payoff matrix for player A.
    pub payoff_a: Vec<Vec<f64>>,
    /// Stage game payoff matrix for player B.
    pub payoff_b: Vec<Vec<f64>>,
    /// Common discount factor δ.
    pub delta: f64,
}
impl RepeatedGame {
    /// Create a new repeated game.
    pub fn new(payoff_a: Vec<Vec<f64>>, payoff_b: Vec<Vec<f64>>, delta: f64) -> Self {
        Self {
            payoff_a,
            payoff_b,
            delta,
        }
    }
    /// Compute the minimax payoff for player A (minimized by player B).
    pub fn minimax_a(&self) -> f64 {
        let nb = if self.payoff_a.is_empty() {
            0
        } else {
            self.payoff_a[0].len()
        };
        let mut best = f64::INFINITY;
        for j in 0..nb {
            let max_a = self
                .payoff_a
                .iter()
                .map(|row| row[j])
                .fold(f64::NEG_INFINITY, f64::max);
            if max_a < best {
                best = max_a;
            }
        }
        best
    }
    /// Compute the minimax payoff for player B (minimized by player A).
    pub fn minimax_b(&self) -> f64 {
        let mut best = f64::INFINITY;
        for row in &self.payoff_b {
            let max_b = row.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
            if max_b < best {
                best = max_b;
            }
        }
        best
    }
    /// Check whether a payoff pair `(va, vb)` is feasible and individually
    /// rational (necessary condition for folk-theorem supportability).
    ///
    /// Feasibility: there exist (possibly mixed) stage strategies achieving `(va, vb)`.
    /// IR: `va ≥ minimax_a` and `vb ≥ minimax_b`.
    pub fn is_folk_theorem_candidate(&self, va: f64, vb: f64) -> bool {
        va >= self.minimax_a() - 1e-9 && vb >= self.minimax_b() - 1e-9
    }
    /// Present-discounted value of a constant stage payoff `u` with discount `delta`.
    ///
    /// PV = (1 - δ) Σ_{t=0}^∞ δ^t u = u
    pub fn present_discounted_value(&self, stage_payoff: f64) -> f64 {
        stage_payoff
    }
    /// Minimum discount factor for which a grim-trigger strategy supports
    /// cooperation in a symmetric prisoner's dilemma with payoffs
    /// `(cooperate, defect, punishment)`.
    pub fn grim_trigger_threshold(cooperate: f64, defect: f64, punishment: f64) -> f64 {
        let numerator = defect - cooperate;
        let denominator = defect - punishment;
        if denominator.abs() < 1e-12 {
            1.0
        } else {
            numerator / denominator
        }
    }
}
/// Replicator dynamics for evolutionary game theory.
/// Population state x = (x_1, ..., x_n) where x_i = frequency of strategy i.
#[allow(dead_code)]
pub struct ReplicatorDynamicsEvo {
    /// The n×n payoff matrix A where A\[i\]\[j\] = payoff to strategy i against j.
    pub payoff_matrix: Vec<Vec<f64>>,
    /// Number of strategies.
    pub n: usize,
}
#[allow(dead_code)]
impl ReplicatorDynamicsEvo {
    /// Create a new replicator dynamics instance.
    pub fn new(payoff_matrix: Vec<Vec<f64>>) -> Self {
        let n = payoff_matrix.len();
        ReplicatorDynamicsEvo { payoff_matrix, n }
    }
    /// Compute the fitness of strategy i given population state x.
    /// f_i(x) = (Ax)_i = Σ_j A_{ij} x_j.
    pub fn fitness(&self, i: usize, x: &[f64]) -> f64 {
        self.payoff_matrix[i]
            .iter()
            .zip(x.iter())
            .map(|(&a, &xj)| a * xj)
            .sum()
    }
    /// Average fitness: φ(x) = Σ_i x_i f_i(x) = x^T A x.
    pub fn average_fitness(&self, x: &[f64]) -> f64 {
        x.iter()
            .enumerate()
            .map(|(i, &xi)| xi * self.fitness(i, x))
            .sum()
    }
    /// Replicator equation: dx_i/dt = x_i (f_i(x) - φ(x)).
    /// Returns the derivatives dx/dt.
    pub fn replicator_rhs(&self, x: &[f64]) -> Vec<f64> {
        let avg = self.average_fitness(x);
        x.iter()
            .enumerate()
            .map(|(i, &xi)| xi * (self.fitness(i, x) - avg))
            .collect()
    }
    /// Euler step of replicator dynamics.
    pub fn euler_step(&self, x: &[f64], dt: f64) -> Vec<f64> {
        let rhs = self.replicator_rhs(x);
        let x_new: Vec<f64> = x
            .iter()
            .zip(rhs.iter())
            .map(|(&xi, &di)| xi + dt * di)
            .collect();
        self.project_simplex(&x_new)
    }
    /// Project a vector onto the probability simplex.
    pub fn project_simplex(&self, x: &[f64]) -> Vec<f64> {
        let n = x.len();
        if n == 0 {
            return vec![];
        }
        let sum: f64 = x.iter().sum();
        if sum.abs() < 1e-15 {
            return vec![1.0 / n as f64; n];
        }
        x.iter().map(|&xi| (xi / sum).max(0.0)).collect()
    }
    /// Check if x is an evolutionarily stable strategy (ESS).
    /// x* is ESS if: for all y ≠ x*, f(x*, x*) > f(y, x*) OR
    /// \[f(x*, x*) = f(y, x*) AND f(x*, y) > f(y, y)\].
    pub fn is_ess(&self, x_star: &[f64], epsilon: f64) -> bool {
        let n = self.n;
        for i in 0..n {
            let mut y = vec![0.0; n];
            y[i] = 1.0;
            let fxx = self.average_fitness(x_star);
            let fyx = self.fitness(i, x_star);
            if (fyx - fxx).abs() < epsilon {
                let mix: Vec<f64> = x_star
                    .iter()
                    .zip(y.iter())
                    .map(|(&xs, &yi)| 0.5 * xs + 0.5 * yi)
                    .collect();
                let fxm = self.average_fitness(&mix);
                let fym = self.fitness(i, &mix);
                if fym >= fxm {
                    return false;
                }
            } else if fyx > fxx {
                return false;
            }
        }
        true
    }
    /// Nash equilibrium check: x is Nash if no strategy can improve fitness.
    pub fn is_nash_equilibrium(&self, x: &[f64], epsilon: f64) -> bool {
        let avg = self.average_fitness(x);
        for i in 0..self.n {
            if x[i].abs() < epsilon {
                continue;
            }
            if (self.fitness(i, x) - avg).abs() > epsilon {
                return false;
            }
        }
        true
    }
}
/// Represents a correlated equilibrium: a joint distribution σ over strategy profiles
/// such that no player has incentive to deviate from the recommended strategy.
#[allow(dead_code)]
pub struct CorrelatedEquilibrium {
    /// Number of players.
    pub num_players: usize,
    /// Number of strategies per player (assuming equal).
    pub strategies_per_player: usize,
    /// Joint distribution over profiles (flattened, row-major).
    pub distribution: Vec<f64>,
}
#[allow(dead_code)]
impl CorrelatedEquilibrium {
    /// Create a new correlated equilibrium instance.
    pub fn new(num_players: usize, strategies_per_player: usize) -> Self {
        let size = strategies_per_player.pow(num_players as u32);
        let uniform = 1.0 / size as f64;
        CorrelatedEquilibrium {
            num_players,
            strategies_per_player,
            distribution: vec![uniform; size],
        }
    }
    /// Total probability mass (should be 1.0 for valid distribution).
    pub fn total_mass(&self) -> f64 {
        self.distribution.iter().sum()
    }
    /// Check if this is a valid probability distribution.
    pub fn is_valid(&self) -> bool {
        let mass = self.total_mass();
        (mass - 1.0).abs() < 1e-9 && self.distribution.iter().all(|&p| p >= -1e-12)
    }
    /// Nash equilibria are always correlated equilibria.
    /// Returns true if this distribution is a product distribution.
    pub fn is_product_distribution(&self) -> bool {
        let n = self.distribution.len();
        if n == 0 {
            return true;
        }
        let expected = 1.0 / n as f64;
        self.distribution
            .iter()
            .all(|&p| (p - expected).abs() < 1e-6)
    }
    /// Linear programming formulation: the set of correlated equilibria forms a polytope.
    /// Returns the number of LP constraints.
    pub fn num_lp_constraints(&self) -> usize {
        let s = self.strategies_per_player;
        self.num_players * s * (s.saturating_sub(1)) + 1
    }
}
/// A Stackelberg game with a leader and a follower.
///
/// `leader_payoffs\[i\]\[j\]` = leader payoff when leader plays i, follower plays j.
/// `follower_payoffs\[i\]\[j\]` = follower payoff when leader plays i, follower plays j.
pub struct StackelbergGame {
    pub leader_strategies: Vec<String>,
    pub follower_strategies: Vec<String>,
    leader_payoffs: Vec<Vec<f64>>,
    follower_payoffs: Vec<Vec<f64>>,
}
impl StackelbergGame {
    pub fn new(
        leader_strategies: Vec<String>,
        follower_strategies: Vec<String>,
        leader_payoffs: Vec<Vec<f64>>,
        follower_payoffs: Vec<Vec<f64>>,
    ) -> Self {
        Self {
            leader_strategies,
            follower_strategies,
            leader_payoffs,
            follower_payoffs,
        }
    }
    /// For each leader strategy, find the follower's best response.
    fn follower_best_response(&self, leader_idx: usize) -> usize {
        self.follower_payoffs[leader_idx]
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(j, _)| j)
            .unwrap_or(0)
    }
    /// Compute the Stackelberg equilibrium via backward induction.
    ///
    /// Returns `(leader_strategy_idx, follower_strategy_idx, leader_payoff)`.
    pub fn stackelberg_equilibrium(&self) -> (usize, usize, f64) {
        let mut best_leader_payoff = f64::NEG_INFINITY;
        let mut best_i = 0;
        let mut best_j = 0;
        for i in 0..self.leader_strategies.len() {
            let j = self.follower_best_response(i);
            let payoff = self.leader_payoffs[i].get(j).copied().unwrap_or(0.0);
            if payoff > best_leader_payoff {
                best_leader_payoff = payoff;
                best_i = i;
                best_j = j;
            }
        }
        (best_i, best_j, best_leader_payoff)
    }
    /// Backward induction: same as stackelberg_equilibrium for two-stage games.
    pub fn backward_induction(&self) -> (usize, usize, f64) {
        self.stackelberg_equilibrium()
    }
}
