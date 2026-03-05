//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
/// Hegselmann-Krause (HK) opinion dynamics model.
///
/// Agents hold real-valued opinions in [0, 1] and update by averaging the opinions
/// of agents within confidence radius epsilon.
#[derive(Debug, Clone)]
pub struct HegselmannKrause {
    /// Current opinions of each agent in [0, 1].
    pub opinions: Vec<f64>,
    /// Confidence radius: agents only average with those within this distance.
    pub epsilon: f64,
}
impl HegselmannKrause {
    /// Create a new HK model.
    pub fn new(opinions: Vec<f64>, epsilon: f64) -> Self {
        HegselmannKrause { opinions, epsilon }
    }
    /// Perform one synchronous update step.
    pub fn step(&mut self) {
        let new_opinions: Vec<f64> = self
            .opinions
            .iter()
            .map(|&xi| {
                let neighbors: Vec<f64> = self
                    .opinions
                    .iter()
                    .copied()
                    .filter(|&xj| (xi - xj).abs() <= self.epsilon)
                    .collect();
                if neighbors.is_empty() {
                    xi
                } else {
                    neighbors.iter().sum::<f64>() / neighbors.len() as f64
                }
            })
            .collect();
        self.opinions = new_opinions;
    }
    /// Run the dynamics for `max_steps` iterations, stopping early if opinions converge
    /// (max change < `tol`).
    ///
    /// Returns the number of steps actually taken.
    pub fn run(&mut self, max_steps: usize, tol: f64) -> usize {
        for step in 0..max_steps {
            let old = self.opinions.clone();
            self.step();
            let max_change = old
                .iter()
                .zip(self.opinions.iter())
                .map(|(&a, &b)| (a - b).abs())
                .fold(0.0f64, f64::max);
            if max_change < tol {
                return step + 1;
            }
        }
        max_steps
    }
    /// Return the number of distinct opinion clusters after convergence.
    ///
    /// Two agents are in the same cluster if their opinions differ by less than `tol`.
    pub fn n_clusters(&self, tol: f64) -> usize {
        let mut clusters: Vec<f64> = Vec::new();
        for &op in &self.opinions {
            if clusters.iter().all(|&c| (c - op).abs() >= tol) {
                clusters.push(op);
            }
        }
        clusters.len()
    }
}
/// Social welfare function implementations (utilitarian and Rawlsian).
#[allow(dead_code)]
pub struct SocialWelfareEval {
    /// Utility matrix: `utility[voter][alt]` = utility of alt for voter.
    pub utility: Vec<Vec<f64>>,
    /// Number of voters.
    pub n_voters: usize,
    /// Number of alternatives.
    pub n_alts: usize,
}
impl SocialWelfareEval {
    /// Create a new social welfare evaluation.
    pub fn new(utility: Vec<Vec<f64>>) -> Self {
        let n_voters = utility.len();
        let n_alts = if n_voters > 0 { utility[0].len() } else { 0 };
        Self {
            utility,
            n_voters,
            n_alts,
        }
    }
    /// Utilitarian welfare of alternative `alt`: sum of individual utilities.
    pub fn utilitarian_welfare(&self, alt: usize) -> f64 {
        self.utility
            .iter()
            .map(|u| u.get(alt).copied().unwrap_or(0.0))
            .sum()
    }
    /// Rawlsian (maximin) welfare of alternative `alt`: minimum individual utility.
    pub fn rawlsian_welfare(&self, alt: usize) -> f64 {
        self.utility
            .iter()
            .map(|u| u.get(alt).copied().unwrap_or(f64::INFINITY))
            .fold(f64::INFINITY, f64::min)
    }
    /// Nash welfare of alternative `alt`: product of individual utilities.
    pub fn nash_welfare(&self, alt: usize) -> f64 {
        self.utility
            .iter()
            .map(|u| u.get(alt).copied().unwrap_or(0.0))
            .product()
    }
    /// Utilitarian optimal alternative: maximises sum of utilities.
    pub fn utilitarian_winner(&self) -> usize {
        (0..self.n_alts)
            .max_by(|&a, &b| {
                self.utilitarian_welfare(a)
                    .partial_cmp(&self.utilitarian_welfare(b))
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .unwrap_or(0)
    }
    /// Rawlsian optimal alternative: maximises minimum utility.
    pub fn rawlsian_winner(&self) -> usize {
        (0..self.n_alts)
            .max_by(|&a, &b| {
                self.rawlsian_welfare(a)
                    .partial_cmp(&self.rawlsian_welfare(b))
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .unwrap_or(0)
    }
    /// Nash optimal alternative: maximises product of utilities.
    pub fn nash_winner(&self) -> usize {
        (0..self.n_alts)
            .max_by(|&a, &b| {
                self.nash_welfare(a)
                    .partial_cmp(&self.nash_welfare(b))
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .unwrap_or(0)
    }
    /// Check if utilitarian and Rawlsian choices agree.
    pub fn utilitarian_equals_rawlsian(&self) -> bool {
        self.utilitarian_winner() == self.rawlsian_winner()
    }
}
/// Properties that majority rule satisfies (May's theorem, 1952).
#[derive(Debug, Clone)]
pub struct MayProperties {
    /// Anonymity: the outcome depends only on votes, not voter identities.
    pub anonymity: bool,
    /// Neutrality: the outcome treats all alternatives symmetrically.
    pub neutrality: bool,
    /// Decisiveness: the rule always produces a clear winner.
    pub decisiveness: bool,
    /// Positive responsiveness: if x wins or ties and one more voter switches to x, x wins.
    pub positive_responsiveness: bool,
}
impl MayProperties {
    /// All four May properties satisfied (majority rule).
    pub fn majority_rule() -> Self {
        MayProperties {
            anonymity: true,
            neutrality: true,
            decisiveness: true,
            positive_responsiveness: true,
        }
    }
    /// Check if all May properties hold (equivalent to majority rule by May's theorem).
    pub fn satisfies_may(&self) -> bool {
        self.anonymity && self.neutrality && self.decisiveness && self.positive_responsiveness
    }
}
/// A weighted voting game: each voter has a weight and there is a quota.
/// A coalition wins if the sum of its members' weights meets or exceeds the quota.
#[derive(Debug, Clone)]
pub struct WeightedVotingGame {
    /// Weight of each voter.
    pub weights: Vec<f64>,
    /// Winning quota (sum of weights must reach this threshold).
    pub quota: f64,
}
impl WeightedVotingGame {
    /// Create a new weighted voting game.
    pub fn new(weights: Vec<f64>, quota: f64) -> Self {
        WeightedVotingGame { weights, quota }
    }
    /// Check whether a coalition (given as a bitmask) is winning.
    pub fn is_winning(&self, coalition: u64) -> bool {
        let mut total = 0.0f64;
        for (i, &w) in self.weights.iter().enumerate() {
            if (coalition >> i) & 1 == 1 {
                total += w;
            }
        }
        total >= self.quota
    }
    /// Compute the Banzhaf power index for each voter.
    ///
    /// The Banzhaf index of voter i = (number of coalitions where i is a swing voter)
    /// divided by the total number of swings across all voters.
    pub fn banzhaf_indices(&self) -> Vec<f64> {
        let n = self.weights.len();
        assert!(n <= 20, "Banzhaf computation limited to 20 voters");
        let n_coalitions = 1u64 << n;
        let mut swings = vec![0u64; n];
        for coalition in 0..n_coalitions {
            let wins = self.is_winning(coalition);
            for i in 0..n {
                let without_i = coalition & !(1u64 << i);
                let wins_without = self.is_winning(without_i);
                if wins && !wins_without {
                    swings[i] += 1;
                }
            }
        }
        let total_swings: u64 = swings.iter().sum();
        if total_swings == 0 {
            return vec![0.0; n];
        }
        swings
            .iter()
            .map(|&s| s as f64 / total_swings as f64)
            .collect()
    }
    /// Compute the Shapley-Shubik power index for each voter.
    ///
    /// The Shapley-Shubik index of voter i = fraction of permutations in which i is
    /// the pivotal voter (the one whose addition to an ordered coalition makes it winning).
    pub fn shapley_shubik_indices(&self) -> Vec<f64> {
        let n = self.weights.len();
        assert!(n <= 10, "Shapley-Shubik computation limited to 10 voters");
        let mut pivots = vec![0u64; n];
        let total_perms = factorial(n);
        let mut perm: Vec<usize> = (0..n).collect();
        loop {
            let mut running_weight = 0.0f64;
            for &voter in &perm {
                running_weight += self.weights[voter];
                if running_weight >= self.quota {
                    pivots[voter] += 1;
                    break;
                }
            }
            if !next_permutation(&mut perm) {
                break;
            }
        }
        pivots
            .iter()
            .map(|&p| p as f64 / total_perms as f64)
            .collect()
    }
}
/// A preference profile: for each voter, an ordered list of alternatives
/// (index 0 = most preferred).
#[derive(Debug, Clone)]
pub struct PreferenceProfile {
    /// Number of voters.
    pub n_voters: usize,
    /// Number of alternatives.
    pub n_alts: usize,
    /// `rankings[voter][rank]` = alternative index at that rank position.
    pub rankings: Vec<Vec<usize>>,
}
impl PreferenceProfile {
    /// Create a new preference profile.
    pub fn new(rankings: Vec<Vec<usize>>) -> Self {
        let n_voters = rankings.len();
        let n_alts = if n_voters > 0 { rankings[0].len() } else { 0 };
        PreferenceProfile {
            n_voters,
            n_alts,
            rankings,
        }
    }
    /// Return the rank of alternative `alt` for voter `voter` (0 = top).
    pub fn rank_of(&self, voter: usize, alt: usize) -> usize {
        self.rankings[voter]
            .iter()
            .position(|&a| a == alt)
            .unwrap_or(self.n_alts)
    }
    /// Return true if voter `voter` prefers `alt_a` over `alt_b`.
    pub fn prefers(&self, voter: usize, alt_a: usize, alt_b: usize) -> bool {
        self.rank_of(voter, alt_a) < self.rank_of(voter, alt_b)
    }
    /// Count how many voters prefer `alt_a` over `alt_b`.
    pub fn majority_margin(&self, alt_a: usize, alt_b: usize) -> usize {
        (0..self.n_voters)
            .filter(|&v| self.prefers(v, alt_a, alt_b))
            .count()
    }
    /// Return true if `alt_a` beats `alt_b` in pairwise majority comparison.
    pub fn majority_beats(&self, alt_a: usize, alt_b: usize) -> bool {
        self.majority_margin(alt_a, alt_b) > self.n_voters / 2
            || (self.majority_margin(alt_a, alt_b) * 2 > self.n_voters)
    }
}
/// A liquid democracy network: voters can delegate their votes to others.
#[derive(Debug, Clone)]
pub struct LiquidDemocracy {
    /// Number of participants.
    pub n_participants: usize,
    /// `delegation[i]` = Some(j) if participant i delegates to j, None if i votes directly.
    pub delegation: Vec<Option<usize>>,
    /// Direct votes: `direct_vote[i]` = Some(alt) if i votes directly (no delegation).
    pub direct_vote: Vec<Option<usize>>,
}
impl LiquidDemocracy {
    /// Create a new liquid democracy instance.
    pub fn new(
        n_participants: usize,
        delegation: Vec<Option<usize>>,
        direct_vote: Vec<Option<usize>>,
    ) -> Self {
        LiquidDemocracy {
            n_participants,
            delegation,
            direct_vote,
        }
    }
    /// Resolve the effective vote of participant `i`.
    ///
    /// Follows delegation chains transitively, stopping if a cycle is detected
    /// or a direct voter is reached.
    pub fn effective_vote(&self, i: usize) -> Option<usize> {
        let mut visited = vec![false; self.n_participants];
        let mut current = i;
        loop {
            if visited[current] {
                return None;
            }
            visited[current] = true;
            match self.delegation[current] {
                None => return self.direct_vote[current],
                Some(next) => current = next,
            }
        }
    }
    /// Compute vote totals for each alternative in a liquid democracy vote.
    /// `n_alts`: number of alternatives.
    pub fn vote_totals(&self, n_alts: usize) -> Vec<usize> {
        let mut counts = vec![0usize; n_alts];
        for i in 0..self.n_participants {
            if let Some(alt) = self.effective_vote(i) {
                if alt < n_alts {
                    counts[alt] += 1;
                }
            }
        }
        counts
    }
    /// Return the winner under liquid democracy plurality rule.
    pub fn winner(&self, n_alts: usize) -> Option<usize> {
        let totals = self.vote_totals(n_alts);
        totals
            .iter()
            .enumerate()
            .max_by_key(|(_, &c)| c)
            .map(|(i, _)| i)
    }
    /// Detect any delegation cycles. Returns a participant involved in a cycle if found.
    pub fn find_cycle(&self) -> Option<usize> {
        for i in 0..self.n_participants {
            let mut visited = vec![false; self.n_participants];
            let mut current = i;
            loop {
                if visited[current] {
                    return Some(current);
                }
                visited[current] = true;
                match self.delegation[current] {
                    None => break,
                    Some(next) => current = next,
                }
            }
        }
        None
    }
}
/// A hedonic game: each agent ranks coalitions containing themselves.
///
/// Preferences are represented as a utility function:
/// `utility[agent][coalition_mask]` gives the utility of agent `i` for being in that coalition.
#[derive(Debug, Clone)]
pub struct HedonicGame {
    /// Number of agents.
    pub n_agents: usize,
    /// `utility[agent][coalition_bitmask]` = agent's utility for being in that coalition.
    pub utility: Vec<Vec<f64>>,
}
impl HedonicGame {
    /// Create a new hedonic game.
    pub fn new(utility: Vec<Vec<f64>>) -> Self {
        let n_agents = utility.len();
        HedonicGame { n_agents, utility }
    }
    /// Check whether a partition (Vec of coalition bitmasks, one per agent) is
    /// individually rational: every agent weakly prefers their coalition to being alone.
    pub fn individually_rational(&self, partition: &[u64]) -> bool {
        for (i, &coalition) in partition.iter().enumerate() {
            if (coalition >> i) & 1 == 0 {
                continue;
            }
            let singleton = 1u64 << i;
            let coal_idx = coalition as usize;
            let sing_idx = singleton as usize;
            if coal_idx >= self.utility[i].len() || sing_idx >= self.utility[i].len() {
                continue;
            }
            if self.utility[i][coal_idx] < self.utility[i][sing_idx] {
                return false;
            }
        }
        true
    }
    /// Find a Nash stable partition using greedy local search.
    ///
    /// Starts from all singletons; repeatedly applies agent moves that strictly improve
    /// their utility until no further improvement is possible.
    pub fn nash_stable_partition(&self) -> Vec<u64> {
        let n = self.n_agents;
        assert!(n <= 20, "Hedonic game computation limited to 20 agents");
        let mut assignment: Vec<u64> = (0..n).map(|i| 1u64 << i).collect();
        let mut improved = true;
        while improved {
            improved = false;
            for i in 0..n {
                let current_coalition = assignment[i];
                let cur_idx = current_coalition as usize;
                let current_util = if cur_idx < self.utility[i].len() {
                    self.utility[i][cur_idx]
                } else {
                    f64::NEG_INFINITY
                };
                for j in 0..n {
                    if i == j {
                        continue;
                    }
                    let target_coalition = assignment[j] | (1u64 << i);
                    let tgt_idx = target_coalition as usize;
                    if tgt_idx >= self.utility[i].len() {
                        continue;
                    }
                    let new_util = self.utility[i][tgt_idx];
                    if new_util > current_util {
                        let new_current = current_coalition & !(1u64 << i);
                        let old_j_coal = assignment[j];
                        for k in 0..n {
                            if assignment[k] == current_coalition {
                                assignment[k] = new_current;
                            }
                        }
                        for k in 0..n {
                            if assignment[k] == old_j_coal {
                                assignment[k] = target_coalition;
                            }
                        }
                        assignment[i] = target_coalition;
                        improved = true;
                        break;
                    }
                }
            }
        }
        assignment
    }
}
/// Evaluate a proper scoring rule on a forecast and an observed outcome.
///
/// Supports Brier, Spherical, and Logarithmic scoring rules.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScoringRule {
    /// Brier score: -(sum of squared errors)
    Brier,
    /// Spherical score: p_outcome / norm(p)
    Spherical,
    /// Logarithmic score: log(p_outcome)
    Logarithmic,
}
impl ScoringRule {
    /// Compute the score for a probability forecast `probs` and observed `outcome`.
    ///
    /// Higher score = better forecast.
    pub fn score(&self, probs: &[f64], outcome: usize) -> f64 {
        assert!(!probs.is_empty(), "probability vector must be non-empty");
        assert!(outcome < probs.len(), "outcome index out of range");
        match self {
            ScoringRule::Brier => {
                let sq_err: f64 = probs
                    .iter()
                    .enumerate()
                    .map(|(i, &p)| {
                        let indicator = if i == outcome { 1.0 } else { 0.0 };
                        (p - indicator).powi(2)
                    })
                    .sum();
                -sq_err
            }
            ScoringRule::Spherical => {
                let norm: f64 = probs.iter().map(|&p| p * p).sum::<f64>().sqrt();
                if norm == 0.0 {
                    f64::NEG_INFINITY
                } else {
                    probs[outcome] / norm
                }
            }
            ScoringRule::Logarithmic => {
                if probs[outcome] <= 0.0 {
                    f64::NEG_INFINITY
                } else {
                    probs[outcome].ln()
                }
            }
        }
    }
    /// Check the properness condition empirically.
    ///
    /// Verifies that expected score under honest reporting is >= expected score under
    /// a misreported `reported_dist`, within tolerance `tol`.
    pub fn is_proper_empirical(&self, true_dist: &[f64], reported_dist: &[f64], tol: f64) -> bool {
        let n = true_dist.len();
        assert_eq!(n, reported_dist.len());
        let honest_expected: f64 = (0..n)
            .map(|i| true_dist[i] * self.score(true_dist, i))
            .sum();
        let misreport_expected: f64 = (0..n)
            .map(|i| true_dist[i] * self.score(reported_dist, i))
            .sum();
        honest_expected >= misreport_expected - tol
    }
}
/// Seat allocation methods for proportional representation.
///
/// Given a list of vote totals for parties, computes the allocation of seats
/// according to either D'Hondt or Sainte-Laguë divisor methods.
#[allow(dead_code)]
pub struct SeatAllocation {
    /// Total number of seats to allocate.
    pub n_seats: usize,
    /// Vote totals for each party.
    pub votes: Vec<f64>,
    /// Allocated seats per party (filled after running a method).
    pub seats: Vec<usize>,
}
impl SeatAllocation {
    /// Create a seat allocation problem.
    pub fn new(votes: Vec<f64>, n_seats: usize) -> Self {
        let n = votes.len();
        Self {
            n_seats,
            votes,
            seats: vec![0; n],
        }
    }
    /// D'Hondt method: highest averages method with divisors 1, 2, 3, ...
    ///
    /// Each party's quotient = votes / (seats_so_far + 1).
    /// Assign the next seat to the party with the highest quotient.
    pub fn dhondt(&mut self) {
        let n = self.votes.len();
        self.seats = vec![0; n];
        for _ in 0..self.n_seats {
            let next_party = (0..n)
                .max_by(|&a, &b| {
                    let qa = self.votes[a] / (self.seats[a] + 1) as f64;
                    let qb = self.votes[b] / (self.seats[b] + 1) as f64;
                    qa.partial_cmp(&qb).unwrap_or(std::cmp::Ordering::Equal)
                })
                .unwrap_or(0);
            self.seats[next_party] += 1;
        }
    }
    /// Sainte-Laguë method: highest averages method with divisors 1, 3, 5, ...
    ///
    /// Each party's quotient = votes / (2 * seats_so_far + 1).
    pub fn sainte_lague(&mut self) {
        let n = self.votes.len();
        self.seats = vec![0; n];
        for _ in 0..self.n_seats {
            let next_party = (0..n)
                .max_by(|&a, &b| {
                    let qa = self.votes[a] / (2 * self.seats[a] + 1) as f64;
                    let qb = self.votes[b] / (2 * self.seats[b] + 1) as f64;
                    qa.partial_cmp(&qb).unwrap_or(std::cmp::Ordering::Equal)
                })
                .unwrap_or(0);
            self.seats[next_party] += 1;
        }
    }
    /// Largest remainder (Hamilton) method.
    ///
    /// Quota = total_votes / n_seats. Each party first gets floor(votes / quota).
    /// Remaining seats go to parties with the largest remainders.
    pub fn largest_remainder(&mut self) {
        let n = self.votes.len();
        let total: f64 = self.votes.iter().sum();
        let quota = total / self.n_seats as f64;
        let mut base: Vec<usize> = self.votes.iter().map(|&v| (v / quota) as usize).collect();
        let allocated: usize = base.iter().sum();
        let remainder_seats = self.n_seats.saturating_sub(allocated);
        let mut remainders: Vec<(usize, f64)> = self
            .votes
            .iter()
            .enumerate()
            .map(|(i, &v)| (i, v / quota - base[i] as f64))
            .collect();
        remainders.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        for i in 0..remainder_seats.min(n) {
            base[remainders[i].0] += 1;
        }
        self.seats = base;
    }
    /// Compute the Hare quota.
    pub fn hare_quota(&self) -> f64 {
        let total: f64 = self.votes.iter().sum();
        total / self.n_seats as f64
    }
    /// Compute the Droop quota.
    pub fn droop_quota(&self) -> f64 {
        let total: f64 = self.votes.iter().sum();
        (total / (self.n_seats + 1) as f64).floor() + 1.0
    }
    /// Check if D'Hondt is more favourable to larger parties than Sainte-Laguë.
    ///
    /// Returns true if the largest party gets strictly more seats under D'Hondt.
    pub fn dhondt_favours_large(&self) -> bool {
        let mut dh = self.votes.clone();
        let mut sl = self.votes.clone();
        let _ = dh.iter_mut().fold(0, |_acc, _v| 0);
        let _ = sl.iter_mut().fold(0, |_acc, _v| 0);
        let mut dhondt_alloc = SeatAllocation::new(self.votes.clone(), self.n_seats);
        let mut sainte_lague_alloc = SeatAllocation::new(self.votes.clone(), self.n_seats);
        dhondt_alloc.dhondt();
        sainte_lague_alloc.sainte_lague();
        if let Some(max_party) = self
            .votes
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(i, _)| i)
        {
            dhondt_alloc.seats[max_party] >= sainte_lague_alloc.seats[max_party]
        } else {
            false
        }
    }
}
/// A judgment aggregation problem: a set of propositions with logical constraints,
/// and individual judgment sets from each voter.
#[derive(Debug, Clone)]
pub struct JudgmentProfile {
    /// Number of voters.
    pub n_voters: usize,
    /// Number of propositions.
    pub n_props: usize,
    /// `judgments[voter][prop]` = true if voter accepts the proposition.
    pub judgments: Vec<Vec<bool>>,
}
impl JudgmentProfile {
    /// Create a new judgment profile.
    pub fn new(judgments: Vec<Vec<bool>>) -> Self {
        let n_voters = judgments.len();
        let n_props = if n_voters > 0 { judgments[0].len() } else { 0 };
        JudgmentProfile {
            n_voters,
            n_props,
            judgments,
        }
    }
    /// Majority judgment rule: accept proposition p iff a strict majority accepts it.
    pub fn majority_judgment(&self) -> Vec<bool> {
        (0..self.n_props)
            .map(|p| {
                let count = (0..self.n_voters).filter(|&v| self.judgments[v][p]).count();
                count * 2 > self.n_voters
            })
            .collect()
    }
    /// Quota rule with threshold `q` in (0, 1]: accept iff fraction accepting >= q.
    pub fn quota_judgment(&self, q: f64) -> Vec<bool> {
        (0..self.n_props)
            .map(|p| {
                let count = (0..self.n_voters).filter(|&v| self.judgments[v][p]).count();
                (count as f64) / (self.n_voters as f64) >= q
            })
            .collect()
    }
    /// Check whether the majority outcome satisfies a given constraint.
    ///
    /// The constraint is expressed as a closure over the aggregate judgment vector.
    pub fn majority_consistent<F>(&self, constraint: F) -> bool
    where
        F: Fn(&[bool]) -> bool,
    {
        let majority = self.majority_judgment();
        constraint(&majority)
    }
    /// Detect the doctrinal paradox: majority outcome is inconsistent with the constraint,
    /// even though each voter's individual judgment is consistent.
    ///
    /// Returns true if the paradox is present.
    pub fn doctrinal_paradox_present<F>(&self, constraint: F) -> bool
    where
        F: Fn(&[bool]) -> bool + Copy,
    {
        let all_consistent = (0..self.n_voters).all(|v| constraint(&self.judgments[v]));
        let majority_inconsistent = !self.majority_consistent(constraint);
        all_consistent && majority_inconsistent
    }
}
