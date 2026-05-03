//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)
use super::functions::*;

/// Vickrey-Clarke-Groves (VCG) mechanism implementation.
/// Allocates a divisible good or selects a discrete allocation to maximize
/// social welfare, with payments making truthful reporting dominant.
#[allow(dead_code)]
pub struct VCGMechanism {
    pub n_agents: usize,
}
impl VCGMechanism {
    /// Create a new VCG mechanism with n agents.
    pub fn new(n_agents: usize) -> Self {
        Self { n_agents }
    }
    /// Given reported valuations, compute the efficient allocation.
    /// For single-item auction: returns index of highest-valuation agent.
    pub fn efficient_allocation(&self, valuations: &[f64]) -> usize {
        valuations
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(i, _)| i)
            .unwrap_or(0)
    }
    /// Compute VCG payments for single-item auction.
    /// Each agent pays the externality they impose: second-highest value.
    /// Winner pays: max_{j ≠ winner} v_j.
    /// Losers pay: 0.
    pub fn vcg_payments(&self, valuations: &[f64]) -> Vec<f64> {
        if valuations.len() < 2 {
            return vec![0.0; valuations.len()];
        }
        let winner = self.efficient_allocation(valuations);
        let mut payments = vec![0.0; valuations.len()];
        let second_highest = valuations
            .iter()
            .enumerate()
            .filter(|&(i, _)| i != winner)
            .map(|(_, &v)| v)
            .fold(f64::NEG_INFINITY, f64::max);
        payments[winner] = second_highest.max(0.0);
        payments
    }
    /// Social welfare = sum of valuations of allocated agents.
    pub fn social_welfare(&self, valuations: &[f64]) -> f64 {
        valuations.iter().cloned().fold(f64::NEG_INFINITY, f64::max)
    }
    /// Surplus for each agent: v_i(allocation) - payment_i.
    pub fn agent_surplus(&self, valuations: &[f64]) -> Vec<f64> {
        let winner = self.efficient_allocation(valuations);
        let payments = self.vcg_payments(valuations);
        valuations
            .iter()
            .enumerate()
            .map(|(i, &v)| {
                if i == winner {
                    v - payments[i]
                } else {
                    -payments[i]
                }
            })
            .collect()
    }
    /// Check dominant strategy incentive compatibility: truthful bidding is optimal.
    /// Simplified check for two agents.
    pub fn is_dsic_for_two(&self, true_v0: f64, true_v1: f64) -> bool {
        let payments_truth = self.vcg_payments(&[true_v0, true_v1]);
        let winner_truth = self.efficient_allocation(&[true_v0, true_v1]);
        let payoff_truth0 = if winner_truth == 0 {
            true_v0 - payments_truth[0]
        } else {
            0.0
        };
        let payments_misrep = self.vcg_payments(&[0.0, true_v1]);
        let winner_misrep = self.efficient_allocation(&[0.0, true_v1]);
        let payoff_misrep0 = if winner_misrep == 0 {
            true_v0 - payments_misrep[0]
        } else {
            0.0
        };
        payoff_truth0 >= payoff_misrep0 - 1e-10
    }
}
pub struct CooperativeGameImpl {
    pub n_players: usize,
    pub coalition_values: Vec<f64>,
}
impl CooperativeGameImpl {
    pub fn new(n_players: usize, coalition_values: Vec<f64>) -> Self {
        assert_eq!(coalition_values.len(), 1 << n_players);
        CooperativeGameImpl {
            n_players,
            coalition_values,
        }
    }
    pub fn grand_coalition_value(&self) -> f64 {
        self.coalition_values[(1 << self.n_players) - 1]
    }
    /// Compute the Shapley value for each player.
    pub fn shapley_value(&self) -> Vec<f64> {
        let n = self.n_players;
        let n_fact = factorial(n);
        let mut phi = vec![0.0; n];
        for i in 0..n {
            let mut sum = 0.0;
            for mask in 0..(1u32 << n) {
                if mask & (1 << i) != 0 {
                    continue;
                }
                let s_size = mask.count_ones() as usize;
                let marginal = self.coalition_values[(mask | (1 << i)) as usize]
                    - self.coalition_values[mask as usize];
                sum += factorial(s_size) as f64 * factorial(n - s_size - 1) as f64 * marginal;
            }
            phi[i] = sum / n_fact as f64;
        }
        phi
    }
    /// Check superadditivity.
    pub fn is_superadditive(&self) -> bool {
        let n = self.n_players;
        for s in 0..(1u32 << n) {
            for t in 0..(1u32 << n) {
                if s & t != 0 {
                    continue;
                }
                if self.coalition_values[(s | t) as usize]
                    < self.coalition_values[s as usize] + self.coalition_values[t as usize] - 1e-10
                {
                    return false;
                }
            }
        }
        true
    }
    /// Check if a payoff vector is in the core.
    pub fn is_in_core(&self, x: &[f64]) -> bool {
        if x.len() != self.n_players {
            return false;
        }
        let n = self.n_players;
        let total: f64 = x.iter().sum();
        if (total - self.grand_coalition_value()).abs() > 1e-8 {
            return false;
        }
        for mask in 1..(1u32 << n) {
            let cs: f64 = (0..n).filter(|&i| mask & (1 << i) != 0).map(|i| x[i]).sum();
            if cs < self.coalition_values[mask as usize] - 1e-8 {
                return false;
            }
        }
        true
    }
    pub fn shapley_in_core(&self) -> bool {
        let phi = self.shapley_value();
        self.is_in_core(&phi)
    }
}
/// Banzhaf power index for weighted voting games.
/// The Banzhaf value of player i measures their average marginal contribution
/// across all 2^{n-1} coalitions not containing i.
#[allow(dead_code)]
pub struct BanzhafIndex {
    pub n_players: usize,
    /// coalition_value\[mask\] = value of coalition encoded by bitmask.
    pub coalition_value: Vec<f64>,
}
impl BanzhafIndex {
    /// Create from coalition value function (2^n values).
    pub fn new(n_players: usize, coalition_value: Vec<f64>) -> Self {
        assert_eq!(coalition_value.len(), 1 << n_players);
        Self {
            n_players,
            coalition_value,
        }
    }
    /// Create from a weighted voting game with quota q and weights w.
    pub fn from_weighted_voting(weights: &[f64], quota: f64) -> Self {
        let n = weights.len();
        let mut vals = vec![0.0; 1 << n];
        for mask in 0..(1u32 << n) {
            let total: f64 = (0..n)
                .filter(|&i| (mask >> i) & 1 == 1)
                .map(|i| weights[i])
                .sum();
            vals[mask as usize] = if total >= quota { 1.0 } else { 0.0 };
        }
        Self::new(n, vals)
    }
    /// Compute the (normalized) Banzhaf value for each player.
    pub fn banzhaf_values(&self) -> Vec<f64> {
        let n = self.n_players;
        let mut raw = vec![0.0; n];
        for i in 0..n {
            for mask in 0..(1u32 << n) {
                if (mask >> i) & 1 != 0 {
                    continue;
                }
                let with = self.coalition_value[(mask | (1 << i)) as usize];
                let without = self.coalition_value[mask as usize];
                raw[i] += (with - without).abs();
            }
        }
        raw
    }
    /// Normalize Banzhaf values so they sum to 1.
    pub fn normalized_banzhaf(&self) -> Vec<f64> {
        let raw = self.banzhaf_values();
        let total: f64 = raw.iter().sum();
        if total < 1e-15 {
            let n = raw.len();
            return vec![1.0 / n as f64; n];
        }
        raw.iter().map(|&v| v / total).collect()
    }
    /// Count number of "swing" coalitions for player i (marginal contributor).
    pub fn swing_count(&self, player: usize) -> usize {
        let n = self.n_players;
        let mut count = 0;
        for mask in 0..(1u32 << n) {
            if (mask >> player) & 1 != 0 {
                continue;
            }
            let with = self.coalition_value[(mask | (1 << player)) as usize];
            let without = self.coalition_value[mask as usize];
            if (with - without).abs() > 1e-10 {
                count += 1;
            }
        }
        count
    }
    /// Check if a player is a dictator (single player with all swing power).
    pub fn is_dictator(&self, player: usize) -> bool {
        let vals = self.banzhaf_values();
        let total: f64 = vals.iter().sum();
        if total < 1e-15 {
            return false;
        }
        (vals[player] / total - 1.0).abs() < 1e-10
    }
}
#[derive(Debug, Clone)]
pub struct AuctionResult {
    pub winner: usize,
    pub price: f64,
}
/// An N-player game where each player chooses from a set of strategies.
pub struct NPlayerGame {
    pub n_players: usize,
    pub n_strategies: Vec<usize>,
    /// Payoffs indexed by strategy profile (flattened).
    /// For a profile (s_0, s_1, ..., s_{n-1}), the flat index is computed
    /// as s_0 * prod_{k>0} n_strategies\[k\] + s_1 * prod_{k>1} n_strategies\[k\] + ...
    pub payoffs: Vec<Vec<f64>>,
}
impl NPlayerGame {
    pub fn new(n_players: usize, n_strategies: Vec<usize>, payoffs: Vec<Vec<f64>>) -> Self {
        NPlayerGame {
            n_players,
            n_strategies,
            payoffs,
        }
    }
    /// Total number of strategy profiles.
    pub fn total_profiles(&self) -> usize {
        self.n_strategies.iter().product()
    }
    /// Convert a strategy profile to a flat index.
    pub fn profile_to_index(&self, profile: &[usize]) -> usize {
        let mut idx = 0;
        let mut stride = 1;
        for i in (0..self.n_players).rev() {
            idx += profile[i] * stride;
            stride *= self.n_strategies[i];
        }
        idx
    }
    /// Convert flat index to strategy profile.
    pub fn index_to_profile(&self, mut idx: usize) -> Vec<usize> {
        let mut profile = vec![0; self.n_players];
        for i in (0..self.n_players).rev() {
            profile[i] = idx % self.n_strategies[i];
            idx /= self.n_strategies[i];
        }
        profile
    }
    /// Get payoff for a given strategy profile.
    pub fn payoff(&self, profile: &[usize]) -> Vec<f64> {
        let idx = self.profile_to_index(profile);
        if idx < self.payoffs.len() {
            self.payoffs[idx].clone()
        } else {
            vec![0.0; self.n_players]
        }
    }
    /// Check if a profile is a pure Nash equilibrium.
    pub fn is_pure_nash_profile(&self, profile: &[usize]) -> bool {
        let current_payoffs = self.payoff(profile);
        for player in 0..self.n_players {
            for alt in 0..self.n_strategies[player] {
                if alt == profile[player] {
                    continue;
                }
                let mut alt_profile = profile.to_vec();
                alt_profile[player] = alt;
                let alt_payoffs = self.payoff(&alt_profile);
                if alt_payoffs[player] > current_payoffs[player] + 1e-10 {
                    return false;
                }
            }
        }
        true
    }
    /// Find all pure Nash equilibria.
    pub fn all_pure_nash_profiles(&self) -> Vec<Vec<usize>> {
        let total = self.total_profiles();
        let mut result = Vec::new();
        for idx in 0..total {
            let profile = self.index_to_profile(idx);
            if self.is_pure_nash_profile(&profile) {
                result.push(profile);
            }
        }
        result
    }
}
/// A two-player finite normal form game represented by payoff matrices.
pub struct TwoPlayerGame {
    pub n_strategies_a: usize,
    pub n_strategies_b: usize,
    pub payoffs_a: Vec<Vec<f64>>,
    pub payoffs_b: Vec<Vec<f64>>,
}
impl TwoPlayerGame {
    pub fn new(payoffs_a: Vec<Vec<f64>>, payoffs_b: Vec<Vec<f64>>) -> Self {
        let n_strategies_a = payoffs_a.len();
        let n_strategies_b = if n_strategies_a > 0 {
            payoffs_a[0].len()
        } else {
            0
        };
        TwoPlayerGame {
            n_strategies_a,
            n_strategies_b,
            payoffs_a,
            payoffs_b,
        }
    }
    /// Create a zero-sum game from A's payoff matrix.
    pub fn zero_sum(payoffs_a: Vec<Vec<f64>>) -> Self {
        let payoffs_b: Vec<Vec<f64>> = payoffs_a
            .iter()
            .map(|row| row.iter().map(|x| -x).collect())
            .collect();
        Self::new(payoffs_a, payoffs_b)
    }
    pub fn is_zero_sum(&self) -> bool {
        for i in 0..self.n_strategies_a {
            for j in 0..self.n_strategies_b {
                if (self.payoffs_a[i][j] + self.payoffs_b[i][j]).abs() > 1e-9 {
                    return false;
                }
            }
        }
        true
    }
    pub fn dominant_strategy_a(&self) -> Option<usize> {
        'outer: for s in 0..self.n_strategies_a {
            for t in 0..self.n_strategies_a {
                if s == t {
                    continue;
                }
                for j in 0..self.n_strategies_b {
                    if self.payoffs_a[s][j] <= self.payoffs_a[t][j] {
                        continue 'outer;
                    }
                }
            }
            return Some(s);
        }
        None
    }
    pub fn dominant_strategy_b(&self) -> Option<usize> {
        'outer: for s in 0..self.n_strategies_b {
            for t in 0..self.n_strategies_b {
                if s == t {
                    continue;
                }
                for i in 0..self.n_strategies_a {
                    if self.payoffs_b[i][s] <= self.payoffs_b[i][t] {
                        continue 'outer;
                    }
                }
            }
            return Some(s);
        }
        None
    }
    /// Return index of weakly dominant strategy for player A.
    pub fn weakly_dominant_strategy_a(&self) -> Option<usize> {
        'outer: for s in 0..self.n_strategies_a {
            let mut strictly_better = false;
            for t in 0..self.n_strategies_a {
                if s == t {
                    continue;
                }
                for j in 0..self.n_strategies_b {
                    if self.payoffs_a[s][j] < self.payoffs_a[t][j] {
                        continue 'outer;
                    }
                    if self.payoffs_a[s][j] > self.payoffs_a[t][j] {
                        strictly_better = true;
                    }
                }
            }
            if strictly_better {
                return Some(s);
            }
        }
        None
    }
    pub fn minimax_row(&self) -> usize {
        let mut best_row = 0;
        let mut best_val = f64::NEG_INFINITY;
        for i in 0..self.n_strategies_a {
            let row_min = self.payoffs_a[i]
                .iter()
                .cloned()
                .fold(f64::INFINITY, f64::min);
            if row_min > best_val {
                best_val = row_min;
                best_row = i;
            }
        }
        best_row
    }
    pub fn minimax_col(&self) -> usize {
        let mut best_col = 0;
        let mut best_val = f64::INFINITY;
        for j in 0..self.n_strategies_b {
            let col_max = (0..self.n_strategies_a)
                .map(|i| self.payoffs_a[i][j])
                .fold(f64::NEG_INFINITY, f64::max);
            if col_max < best_val {
                best_val = col_max;
                best_col = j;
            }
        }
        best_col
    }
    /// Maximin value for the row player.
    pub fn maximin_value(&self) -> f64 {
        let i = self.minimax_row();
        self.payoffs_a[i]
            .iter()
            .cloned()
            .fold(f64::INFINITY, f64::min)
    }
    /// Minimax value for the column player.
    pub fn minimax_value(&self) -> f64 {
        let j = self.minimax_col();
        (0..self.n_strategies_a)
            .map(|i| self.payoffs_a[i][j])
            .fold(f64::NEG_INFINITY, f64::max)
    }
    pub fn saddle_point(&self) -> Option<(usize, usize)> {
        for i in 0..self.n_strategies_a {
            for j in 0..self.n_strategies_b {
                let val = self.payoffs_a[i][j];
                let is_row_min = self.payoffs_a[i].iter().all(|&v| val <= v);
                let is_col_max = (0..self.n_strategies_a).all(|k| self.payoffs_a[k][j] <= val);
                if is_row_min && is_col_max {
                    return Some((i, j));
                }
            }
        }
        None
    }
    pub fn best_response_a(&self, b_strategy: usize) -> usize {
        let mut best = 0;
        let mut best_val = f64::NEG_INFINITY;
        for i in 0..self.n_strategies_a {
            if self.payoffs_a[i][b_strategy] > best_val {
                best_val = self.payoffs_a[i][b_strategy];
                best = i;
            }
        }
        best
    }
    pub fn best_response_b(&self, a_strategy: usize) -> usize {
        let mut best = 0;
        let mut best_val = f64::NEG_INFINITY;
        for j in 0..self.n_strategies_b {
            if self.payoffs_b[a_strategy][j] > best_val {
                best_val = self.payoffs_b[a_strategy][j];
                best = j;
            }
        }
        best
    }
    pub fn is_pure_nash(&self, i: usize, j: usize) -> bool {
        let a_best = self.best_response_a(j);
        if self.payoffs_a[a_best][j] > self.payoffs_a[i][j] {
            return false;
        }
        let b_best = self.best_response_b(i);
        if self.payoffs_b[i][b_best] > self.payoffs_b[i][j] {
            return false;
        }
        true
    }
    pub fn all_pure_nash(&self) -> Vec<(usize, usize)> {
        let mut result = Vec::new();
        for i in 0..self.n_strategies_a {
            for j in 0..self.n_strategies_b {
                if self.is_pure_nash(i, j) {
                    result.push((i, j));
                }
            }
        }
        result
    }
    /// Check if (i,j) is Pareto optimal.
    pub fn is_pareto_optimal(&self, i: usize, j: usize) -> bool {
        for ii in 0..self.n_strategies_a {
            for jj in 0..self.n_strategies_b {
                if ii == i && jj == j {
                    continue;
                }
                let a_ge = self.payoffs_a[ii][jj] >= self.payoffs_a[i][j];
                let b_ge = self.payoffs_b[ii][jj] >= self.payoffs_b[i][j];
                let a_gt = self.payoffs_a[ii][jj] > self.payoffs_a[i][j];
                let b_gt = self.payoffs_b[ii][jj] > self.payoffs_b[i][j];
                if a_ge && b_ge && (a_gt || b_gt) {
                    return false;
                }
            }
        }
        true
    }
    /// Expected payoff for player A given mixed strategies.
    pub fn expected_payoff_a(&self, p: &[f64], q: &[f64]) -> f64 {
        let mut total = 0.0;
        for i in 0..self.n_strategies_a {
            for j in 0..self.n_strategies_b {
                total += p[i] * q[j] * self.payoffs_a[i][j];
            }
        }
        total
    }
    /// Expected payoff for player B given mixed strategies.
    pub fn expected_payoff_b(&self, p: &[f64], q: &[f64]) -> f64 {
        let mut total = 0.0;
        for i in 0..self.n_strategies_a {
            for j in 0..self.n_strategies_b {
                total += p[i] * q[j] * self.payoffs_b[i][j];
            }
        }
        total
    }
    /// For a 2x2 game, compute the mixed strategy Nash equilibrium.
    pub fn mixed_nash_2x2(&self) -> Option<(f64, f64)> {
        if self.n_strategies_a != 2 || self.n_strategies_b != 2 {
            return None;
        }
        let dq = self.payoffs_a[0][0] - self.payoffs_a[0][1] - self.payoffs_a[1][0]
            + self.payoffs_a[1][1];
        if dq.abs() < 1e-12 {
            return None;
        }
        let q = (self.payoffs_a[1][1] - self.payoffs_a[0][1]) / dq;
        let dp = self.payoffs_b[0][0] - self.payoffs_b[1][0] - self.payoffs_b[0][1]
            + self.payoffs_b[1][1];
        if dp.abs() < 1e-12 {
            return None;
        }
        let p = (self.payoffs_b[1][1] - self.payoffs_b[1][0]) / dp;
        if p >= 0.0 && p <= 1.0 && q >= 0.0 && q <= 1.0 {
            Some((p, q))
        } else {
            None
        }
    }
    /// Iterated elimination of strictly dominated strategies.
    pub fn iterated_elimination(&self) -> (Vec<usize>, Vec<usize>) {
        let mut a_surv: Vec<usize> = (0..self.n_strategies_a).collect();
        let mut b_surv: Vec<usize> = (0..self.n_strategies_b).collect();
        let mut changed = true;
        while changed {
            changed = false;
            let mut new_a = Vec::new();
            'ca: for &s in &a_surv {
                for &t in &a_surv {
                    if s == t {
                        continue;
                    }
                    if b_surv
                        .iter()
                        .all(|&j| self.payoffs_a[s][j] < self.payoffs_a[t][j])
                    {
                        changed = true;
                        continue 'ca;
                    }
                }
                new_a.push(s);
            }
            a_surv = new_a;
            let mut new_b = Vec::new();
            'cb: for &s in &b_surv {
                for &t in &b_surv {
                    if s == t {
                        continue;
                    }
                    if a_surv
                        .iter()
                        .all(|&i| self.payoffs_b[i][s] < self.payoffs_b[i][t])
                    {
                        changed = true;
                        continue 'cb;
                    }
                }
                new_b.push(s);
            }
            b_surv = new_b;
        }
        (a_surv, b_surv)
    }
}
/// Quantal Response Equilibrium (QRE) solver for two-player games.
/// Under QRE, players best-respond with logistic noise (rationality λ).
/// As λ → ∞, QRE → Nash; as λ → 0, QRE → uniform mix.
#[allow(dead_code)]
pub struct QuantalResponseEquilibriumSolver {
    pub game: TwoPlayerGame,
    pub lambda: f64,
}
impl QuantalResponseEquilibriumSolver {
    /// Create a new QRE solver with rationality parameter lambda.
    pub fn new(game: TwoPlayerGame, lambda: f64) -> Self {
        Self { game, lambda }
    }
    /// Logistic quantal response for player A given player B's mixed strategy q.
    pub fn qre_a(&self, q: &[f64]) -> Vec<f64> {
        let m = self.game.n_strategies_a;
        let expected: Vec<f64> = (0..m)
            .map(|i| {
                (0..self.game.n_strategies_b)
                    .map(|j| {
                        if j < q.len() {
                            q[j] * self.game.payoffs_a[i][j]
                        } else {
                            0.0
                        }
                    })
                    .sum::<f64>()
            })
            .collect();
        softmax(&expected, self.lambda)
    }
    /// Logistic quantal response for player B given player A's mixed strategy p.
    pub fn qre_b(&self, p: &[f64]) -> Vec<f64> {
        let n = self.game.n_strategies_b;
        let expected: Vec<f64> = (0..n)
            .map(|j| {
                (0..self.game.n_strategies_a)
                    .map(|i| {
                        if i < p.len() {
                            p[i] * self.game.payoffs_b[i][j]
                        } else {
                            0.0
                        }
                    })
                    .sum::<f64>()
            })
            .collect();
        softmax(&expected, self.lambda)
    }
    /// Fixed-point iteration to find the QRE.
    /// Returns (p*, q*) after `max_iter` iterations.
    pub fn solve(&self, max_iter: usize) -> (Vec<f64>, Vec<f64>) {
        let m = self.game.n_strategies_a;
        let n = self.game.n_strategies_b;
        let mut p = vec![1.0 / m as f64; m];
        let mut q = vec![1.0 / n as f64; n];
        for _ in 0..max_iter {
            let new_p = self.qre_a(&q);
            let new_q = self.qre_b(&p);
            p = new_p;
            q = new_q;
        }
        (p, q)
    }
    /// Compute the total variation distance between two distributions.
    pub fn tv_distance(p: &[f64], q: &[f64]) -> f64 {
        p.iter()
            .zip(q.iter())
            .map(|(a, b)| (a - b).abs())
            .sum::<f64>()
            / 2.0
    }
}
#[derive(Debug, Clone)]
pub enum GameNode {
    Decision {
        player: usize,
        children: Vec<(String, GameNode)>,
    },
    Chance {
        outcomes: Vec<(f64, GameNode)>,
    },
    Terminal {
        payoffs: Vec<f64>,
    },
}
impl GameNode {
    pub fn terminal(payoffs: Vec<f64>) -> Self {
        GameNode::Terminal { payoffs }
    }
    pub fn decision(player: usize, children: Vec<(String, GameNode)>) -> Self {
        GameNode::Decision { player, children }
    }
    pub fn chance(outcomes: Vec<(f64, GameNode)>) -> Self {
        GameNode::Chance { outcomes }
    }
    pub fn node_count(&self) -> usize {
        match self {
            GameNode::Terminal { .. } => 1,
            GameNode::Decision { children, .. } => {
                1 + children.iter().map(|(_, c)| c.node_count()).sum::<usize>()
            }
            GameNode::Chance { outcomes, .. } => {
                1 + outcomes.iter().map(|(_, c)| c.node_count()).sum::<usize>()
            }
        }
    }
    pub fn terminal_count(&self) -> usize {
        match self {
            GameNode::Terminal { .. } => 1,
            GameNode::Decision { children, .. } => {
                children.iter().map(|(_, c)| c.terminal_count()).sum()
            }
            GameNode::Chance { outcomes, .. } => {
                outcomes.iter().map(|(_, c)| c.terminal_count()).sum()
            }
        }
    }
    /// Backward induction: returns (payoff vector, action name).
    pub fn backward_induction(&self) -> (Vec<f64>, String) {
        match self {
            GameNode::Terminal { payoffs } => (payoffs.clone(), String::new()),
            GameNode::Decision { player, children } => {
                if children.is_empty() {
                    return (vec![], String::new());
                }
                let sub: Vec<(Vec<f64>, String)> = children
                    .iter()
                    .map(|(act, ch)| {
                        let (p, _) = ch.backward_induction();
                        (p, act.clone())
                    })
                    .collect();
                let mut best_idx = 0;
                let mut best_val = f64::NEG_INFINITY;
                for (idx, (payoffs, _)) in sub.iter().enumerate() {
                    if *player < payoffs.len() && payoffs[*player] > best_val {
                        best_val = payoffs[*player];
                        best_idx = idx;
                    }
                }
                (sub[best_idx].0.clone(), sub[best_idx].1.clone())
            }
            GameNode::Chance { outcomes } => {
                let sub: Vec<(f64, Vec<f64>)> = outcomes
                    .iter()
                    .map(|(prob, ch)| {
                        let (p, _) = ch.backward_induction();
                        (*prob, p)
                    })
                    .collect();
                if sub.is_empty() {
                    return (vec![], String::new());
                }
                let n_players = sub[0].1.len();
                let mut expected = vec![0.0; n_players];
                for (prob, payoffs) in &sub {
                    for (k, p) in payoffs.iter().enumerate() {
                        if k < n_players {
                            expected[k] += prob * p;
                        }
                    }
                }
                (expected, "chance".to_string())
            }
        }
    }
}
pub struct EvolutionaryGame {
    pub strategies: Vec<String>,
    pub fitness_matrix: Vec<Vec<f64>>,
}
impl EvolutionaryGame {
    pub fn new(strategies: Vec<String>, fitness_matrix: Vec<Vec<f64>>) -> Self {
        EvolutionaryGame {
            strategies,
            fitness_matrix,
        }
    }
    pub fn avg_fitness(&self, population: &[f64]) -> f64 {
        let n = self.strategies.len();
        (0..n)
            .map(|i| population[i] * self.strategy_fitness(population, i))
            .sum()
    }
    pub fn strategy_fitness(&self, population: &[f64], strategy: usize) -> f64 {
        let n = self.strategies.len();
        (0..n)
            .map(|j| self.fitness_matrix[strategy][j] * population[j])
            .sum()
    }
    pub fn replicator_step(&self, population: &[f64], dt: f64) -> Vec<f64> {
        let n = self.strategies.len();
        let avg = self.avg_fitness(population);
        let mut new_pop: Vec<f64> = (0..n)
            .map(|i| {
                population[i] + dt * population[i] * (self.strategy_fitness(population, i) - avg)
            })
            .collect();
        let total: f64 = new_pop.iter().sum();
        if total > 1e-15 {
            for xi in &mut new_pop {
                *xi /= total;
            }
        }
        new_pop
    }
    pub fn simulate(&self, initial: Vec<f64>, steps: u32, dt: f64) -> Vec<Vec<f64>> {
        let mut history = vec![initial.clone()];
        let mut current = initial;
        for _ in 0..steps {
            current = self.replicator_step(&current, dt);
            history.push(current.clone());
        }
        history
    }
    /// Check if a pure strategy is an ESS.
    pub fn is_ess(&self, strategy: usize) -> bool {
        let n = self.strategies.len();
        for j in 0..n {
            if j == strategy {
                continue;
            }
            if self.fitness_matrix[strategy][strategy] > self.fitness_matrix[j][strategy] {
                continue;
            }
            if (self.fitness_matrix[strategy][strategy] - self.fitness_matrix[j][strategy]).abs()
                < 1e-10
            {
                if self.fitness_matrix[strategy][j] <= self.fitness_matrix[j][j] {
                    return false;
                }
            } else {
                return false;
            }
        }
        true
    }
    pub fn find_all_ess(&self) -> Vec<usize> {
        (0..self.strategies.len())
            .filter(|&i| self.is_ess(i))
            .collect()
    }
}
/// Nash bargaining problem and solution.
/// The feasible set is represented as a finite list of Pareto-efficient payoff pairs.
#[allow(dead_code)]
pub struct NashBargainingSolver {
    /// Feasible payoff vectors (u1, u2).
    pub feasible: Vec<(f64, f64)>,
    /// Disagreement point (d1, d2).
    pub disagreement: (f64, f64),
}
impl NashBargainingSolver {
    /// Create a new Nash bargaining problem.
    pub fn new(feasible: Vec<(f64, f64)>, disagreement: (f64, f64)) -> Self {
        Self {
            feasible,
            disagreement,
        }
    }
    /// Compute the Nash bargaining solution: arg max (u1 - d1)(u2 - d2).
    pub fn nash_solution(&self) -> Option<(f64, f64)> {
        let (d1, d2) = self.disagreement;
        self.feasible
            .iter()
            .filter(|&&(u1, u2)| u1 >= d1 && u2 >= d2)
            .max_by(|&&(u1a, u2a), &&(u1b, u2b)| {
                let na = (u1a - d1) * (u2a - d2);
                let nb = (u1b - d1) * (u2b - d2);
                na.partial_cmp(&nb).unwrap_or(std::cmp::Ordering::Equal)
            })
            .copied()
    }
    /// Compute the Kalai-Smorodinsky solution: proportional to ideal payoffs.
    pub fn kalai_smorodinsky_solution(&self) -> Option<(f64, f64)> {
        let (d1, d2) = self.disagreement;
        let a1 = self
            .feasible
            .iter()
            .filter(|&&(_, u2)| u2 >= d2)
            .map(|&(u1, _)| u1)
            .fold(f64::NEG_INFINITY, f64::max);
        let a2 = self
            .feasible
            .iter()
            .filter(|&&(u1, _)| u1 >= d1)
            .map(|&(_, u2)| u2)
            .fold(f64::NEG_INFINITY, f64::max);
        if a1 <= d1 || a2 <= d2 {
            return None;
        }
        let ratio = (a2 - d2) / (a1 - d1);
        self.feasible
            .iter()
            .filter(|&&(u1, u2)| u1 >= d1 && u2 >= d2)
            .min_by(|&&(u1a, u2a), &&(u1b, u2b)| {
                let ra = ((u2a - d2) / (u1a - d1 + 1e-15) - ratio).abs();
                let rb = ((u2b - d2) / (u1b - d1 + 1e-15) - ratio).abs();
                ra.partial_cmp(&rb).unwrap_or(std::cmp::Ordering::Equal)
            })
            .copied()
    }
    /// Symmetric solution: find point where u1 - d1 = u2 - d2.
    pub fn symmetric_solution(&self) -> Option<(f64, f64)> {
        let (d1, d2) = self.disagreement;
        self.feasible
            .iter()
            .filter(|&&(u1, u2)| u1 >= d1 && u2 >= d2)
            .min_by(|&&(u1a, u2a), &&(u1b, u2b)| {
                let da = ((u1a - d1) - (u2a - d2)).abs();
                let db = ((u1b - d1) - (u2b - d2)).abs();
                da.partial_cmp(&db).unwrap_or(std::cmp::Ordering::Equal)
            })
            .copied()
    }
}
/// Fictitious play simulator for finite normal-form games.
/// Each player maintains empirical frequency counts and best-responds.
#[allow(dead_code)]
pub struct FictitiousPlaySimulator {
    pub game: TwoPlayerGame,
    /// Empirical frequency counts for player A's strategies.
    pub counts_a: Vec<u64>,
    /// Empirical frequency counts for player B's strategies.
    pub counts_b: Vec<u64>,
    pub round: u64,
}
impl FictitiousPlaySimulator {
    /// Initialize with a starting strategy profile.
    pub fn new(game: TwoPlayerGame, init_a: usize, init_b: usize) -> Self {
        let m = game.n_strategies_a;
        let n = game.n_strategies_b;
        let mut counts_a = vec![0u64; m];
        let mut counts_b = vec![0u64; n];
        if init_a < m {
            counts_a[init_a] = 1;
        }
        if init_b < n {
            counts_b[init_b] = 1;
        }
        Self {
            game,
            counts_a,
            counts_b,
            round: 1,
        }
    }
    /// Compute empirical mixed strategy for player A.
    pub fn empirical_a(&self) -> Vec<f64> {
        let total: u64 = self.counts_a.iter().sum();
        if total == 0 {
            let m = self.counts_a.len();
            return vec![1.0 / m as f64; m];
        }
        self.counts_a
            .iter()
            .map(|&c| c as f64 / total as f64)
            .collect()
    }
    /// Compute empirical mixed strategy for player B.
    pub fn empirical_b(&self) -> Vec<f64> {
        let total: u64 = self.counts_b.iter().sum();
        if total == 0 {
            let n = self.counts_b.len();
            return vec![1.0 / n as f64; n];
        }
        self.counts_b
            .iter()
            .map(|&c| c as f64 / total as f64)
            .collect()
    }
    /// Best response for player A against empirical distribution of B.
    pub fn best_response_a_fp(&self) -> usize {
        let q = self.empirical_b();
        let m = self.game.n_strategies_a;
        let (mut best, mut best_val) = (0, f64::NEG_INFINITY);
        for i in 0..m {
            let val: f64 = q
                .iter()
                .enumerate()
                .map(|(j, &qj)| qj * self.game.payoffs_a[i][j])
                .sum();
            if val > best_val {
                best_val = val;
                best = i;
            }
        }
        best
    }
    /// Best response for player B against empirical distribution of A.
    pub fn best_response_b_fp(&self) -> usize {
        let p = self.empirical_a();
        let n = self.game.n_strategies_b;
        let (mut best, mut best_val) = (0, f64::NEG_INFINITY);
        for j in 0..n {
            let val: f64 = p
                .iter()
                .enumerate()
                .map(|(i, &pi)| pi * self.game.payoffs_b[i][j])
                .sum();
            if val > best_val {
                best_val = val;
                best = j;
            }
        }
        best
    }
    /// Run one round of fictitious play.
    pub fn step(&mut self) {
        let a = self.best_response_a_fp();
        let b = self.best_response_b_fp();
        self.counts_a[a] += 1;
        self.counts_b[b] += 1;
        self.round += 1;
    }
    /// Run T rounds of fictitious play.
    pub fn run(&mut self, rounds: u64) {
        for _ in 0..rounds {
            self.step();
        }
    }
    /// Check convergence: empirical distributions are close to Nash.
    pub fn has_converged(&self, tol: f64) -> bool {
        let p = self.empirical_a();
        let q = self.empirical_b();
        let m = self.game.n_strategies_a;
        let n = self.game.n_strategies_b;
        let best_a = (0..m)
            .map(|i| {
                (0..n)
                    .map(|j| q[j] * self.game.payoffs_a[i][j])
                    .sum::<f64>()
            })
            .fold(f64::NEG_INFINITY, f64::max);
        let exp_a: f64 = (0..m)
            .map(|i| {
                p[i] * (0..n)
                    .map(|j| q[j] * self.game.payoffs_a[i][j])
                    .sum::<f64>()
            })
            .sum();
        let best_b = (0..n)
            .map(|j| {
                (0..m)
                    .map(|i| p[i] * self.game.payoffs_b[i][j])
                    .sum::<f64>()
            })
            .fold(f64::NEG_INFINITY, f64::max);
        let exp_b: f64 = (0..n)
            .map(|j| {
                q[j] * (0..m)
                    .map(|i| p[i] * self.game.payoffs_b[i][j])
                    .sum::<f64>()
            })
            .sum();
        (best_a - exp_a).abs() < tol && (best_b - exp_b).abs() < tol
    }
}
