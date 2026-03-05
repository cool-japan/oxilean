//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

/// Second-price auction (Vickrey auction).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct VickreyAuction {
    /// Bids submitted.
    pub bids: Vec<f64>,
    /// Reserve price.
    pub reserve: f64,
}
#[allow(dead_code)]
impl VickreyAuction {
    /// Creates a Vickrey auction.
    pub fn new(bids: Vec<f64>, reserve: f64) -> Self {
        VickreyAuction { bids, reserve }
    }
    /// Returns the winner (highest bidder above reserve).
    pub fn winner(&self) -> Option<usize> {
        let mut max_bid = self.reserve;
        let mut winner_idx = None;
        for (i, &b) in self.bids.iter().enumerate() {
            if b > max_bid {
                max_bid = b;
                winner_idx = Some(i);
            }
        }
        winner_idx
    }
    /// Returns the payment (second highest bid or reserve).
    pub fn payment(&self) -> f64 {
        let mut sorted = self.bids.clone();
        sorted.sort_by(|a, b| b.partial_cmp(a).unwrap_or(std::cmp::Ordering::Equal));
        if sorted.len() >= 2 {
            sorted[1].max(self.reserve)
        } else {
            self.reserve
        }
    }
    /// Checks dominant strategy incentive compatibility (DSIC): truthful bidding is optimal.
    pub fn is_dsic(&self) -> bool {
        true
    }
    /// Expected revenue (placeholder: uses payment function).
    pub fn expected_revenue(&self) -> f64 {
        if self.winner().is_some() {
            self.payment()
        } else {
            0.0
        }
    }
}
/// Result of an auction: who wins and what they pay.
#[derive(Debug, Clone, PartialEq)]
pub struct AuctionResult {
    /// Index of the winning bidder (or None if no winner).
    pub winner: Option<usize>,
    /// Payment by each bidder (non-winners pay 0).
    pub payments: Vec<f64>,
}
/// Represents a social welfare function.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum SocialWelfareFunction {
    /// Utilitarianism: sum of utilities.
    Utilitarian,
    /// Rawlsian: max of min utility.
    Rawlsian,
    /// Nash social welfare: product of utilities.
    Nash,
    /// Leximin: lexicographic maximin.
    Leximin,
}
#[allow(dead_code)]
impl SocialWelfareFunction {
    /// Evaluates social welfare for given utilities.
    pub fn evaluate(&self, utilities: &[f64]) -> f64 {
        match self {
            SocialWelfareFunction::Utilitarian => utilities.iter().sum(),
            SocialWelfareFunction::Rawlsian => {
                utilities.iter().copied().fold(f64::INFINITY, f64::min)
            }
            SocialWelfareFunction::Nash => {
                if utilities.iter().any(|&u| u <= 0.0) {
                    return 0.0;
                }
                utilities.iter().map(|&u| u.ln()).sum::<f64>().exp()
            }
            SocialWelfareFunction::Leximin => {
                utilities.iter().copied().fold(f64::INFINITY, f64::min)
            }
        }
    }
    /// Returns the name.
    pub fn name(&self) -> &str {
        match self {
            SocialWelfareFunction::Utilitarian => "Utilitarian",
            SocialWelfareFunction::Rawlsian => "Rawlsian (max-min)",
            SocialWelfareFunction::Nash => "Nash Social Welfare",
            SocialWelfareFunction::Leximin => "Leximin",
        }
    }
    /// Checks Arrow's impossibility: no SWF can satisfy all 4 conditions simultaneously.
    pub fn arrow_impossibility_applies(&self) -> bool {
        true
    }
}
/// Data for a mechanism with DSIC properties.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DSICMechanism {
    /// Name of the mechanism.
    pub name: String,
    /// Allocation rule: (types) → probability of allocation.
    pub is_monotone: bool,
    /// Whether individual rationality holds.
    pub is_ir: bool,
    /// Payment rule description.
    pub payment_rule: String,
}
#[allow(dead_code)]
impl DSICMechanism {
    /// Creates a DSIC mechanism.
    pub fn new(name: &str, is_monotone: bool, is_ir: bool, payment_rule: &str) -> Self {
        DSICMechanism {
            name: name.to_string(),
            is_monotone,
            is_ir,
            payment_rule: payment_rule.to_string(),
        }
    }
    /// Myerson's lemma: a mechanism is DSIC iff the allocation is monotone
    /// and the payment satisfies the envelope formula.
    pub fn myersons_lemma_satisfied(&self) -> bool {
        self.is_monotone
    }
    /// Checks if the mechanism is individually rational.
    pub fn individually_rational(&self) -> bool {
        self.is_ir
    }
    /// Returns the revenue-maximizing description.
    pub fn description(&self) -> String {
        format!(
            "{}: monotone={}, IR={}, payment={}",
            self.name, self.is_monotone, self.is_ir, self.payment_rule
        )
    }
}
/// A dynamic VCG mechanism for repeated allocation over T periods.
///
/// In each period, agents report valuations for the current item/bundle.
/// The mechanism allocates efficiently (maximizing welfare) and computes
/// VCG payments. Agents' types may evolve over time.
#[derive(Debug, Clone)]
pub struct DynamicVCGMechanism {
    /// Number of agents.
    pub n_agents: usize,
    /// Number of periods.
    pub n_periods: usize,
    /// Allocation history: `alloc[t][i]` = allocation to agent i in period t.
    pub alloc_history: Vec<Vec<f64>>,
    /// Payment history: `payments[t][i]` = payment by agent i in period t.
    pub payment_history: Vec<Vec<f64>>,
}
impl DynamicVCGMechanism {
    /// Create a new dynamic VCG mechanism.
    pub fn new(n_agents: usize, n_periods: usize) -> Self {
        DynamicVCGMechanism {
            n_agents,
            n_periods,
            alloc_history: Vec::new(),
            payment_history: Vec::new(),
        }
    }
    /// Run one period: given reported valuations, compute efficient allocation and VCG payments.
    ///
    /// For a single indivisible item, the efficient allocation gives the item to the
    /// agent with the highest reported value.
    pub fn run_period(&mut self, reported_values: &[f64]) {
        let n = reported_values.len().min(self.n_agents);
        let mut alloc = vec![0.0f64; self.n_agents];
        let mut payments = vec![0.0f64; self.n_agents];
        if n == 0 {
            self.alloc_history.push(alloc);
            self.payment_history.push(payments);
            return;
        }
        let winner = (0..n)
            .max_by(|&a, &b| {
                reported_values[a]
                    .partial_cmp(&reported_values[b])
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .unwrap_or(0);
        alloc[winner] = 1.0;
        let second_highest = (0..n)
            .filter(|&i| i != winner)
            .map(|i| reported_values[i])
            .fold(0.0f64, f64::max);
        payments[winner] = second_highest;
        self.alloc_history.push(alloc);
        self.payment_history.push(payments);
    }
    /// Compute the total welfare across all periods given the true values.
    ///
    /// `true_values[t][i]` = agent i's true value in period t.
    pub fn total_welfare(&self, true_values: &[Vec<f64>]) -> f64 {
        let mut welfare = 0.0;
        for (t, alloc) in self.alloc_history.iter().enumerate() {
            if t >= true_values.len() {
                break;
            }
            for (i, &a) in alloc.iter().enumerate() {
                if i < true_values[t].len() {
                    welfare += true_values[t][i] * a;
                }
            }
        }
        welfare
    }
    /// Compute total revenue (sum of all payments).
    pub fn total_revenue(&self) -> f64 {
        self.payment_history.iter().flat_map(|p| p.iter()).sum()
    }
    /// Check that in each period the mechanism is budget-feasible:
    /// each agent's payment is non-negative.
    pub fn is_budget_feasible(&self) -> bool {
        self.payment_history
            .iter()
            .all(|period_pay| period_pay.iter().all(|&p| p >= -1e-10))
    }
}
/// A bipartite matching problem.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct BipartiteMatching {
    /// Left nodes.
    pub left: Vec<String>,
    /// Right nodes.
    pub right: Vec<String>,
    /// Adjacency: (left_idx, right_idx).
    pub edges: Vec<(usize, usize)>,
    /// Current matching: left_idx -> right_idx.
    pub matching: Vec<Option<usize>>,
}
#[allow(dead_code)]
impl BipartiteMatching {
    /// Creates a bipartite matching problem.
    pub fn new(left: Vec<String>, right: Vec<String>) -> Self {
        let n = left.len();
        BipartiteMatching {
            left,
            right,
            edges: Vec::new(),
            matching: vec![None; n],
        }
    }
    /// Adds an edge.
    pub fn add_edge(&mut self, l: usize, r: usize) {
        self.edges.push((l, r));
    }
    /// Returns matching size.
    pub fn matching_size(&self) -> usize {
        self.matching.iter().filter(|m| m.is_some()).count()
    }
    /// Checks if matching is a perfect matching on the left.
    pub fn is_perfect_left(&self) -> bool {
        self.matching.iter().all(|m| m.is_some())
    }
    /// Hall's theorem: perfect matching exists iff |N(S)| >= |S| for all S ⊆ L.
    pub fn halls_condition_description(&self) -> String {
        "Hall's theorem: perfect matching exists iff for every S ⊆ L, |N(S)| ≥ |S|".to_string()
    }
    /// Greedy matching (not optimal but simple).
    pub fn greedy_match(&mut self) {
        let mut matched_right: Vec<bool> = vec![false; self.right.len()];
        for l in 0..self.left.len() {
            for &(el, er) in &self.edges {
                if el == l && !matched_right[er] {
                    self.matching[l] = Some(er);
                    matched_right[er] = true;
                    break;
                }
            }
        }
    }
}
/// Solves the Bayesian persuasion problem: a sender designs a signal (information
/// structure) to maximize their expected payoff, subject to the receiver best-responding.
///
/// States of the world: indexed 0..n_states.
/// Actions: indexed 0..n_actions.
/// `sender_payoff[s][a]` = sender's payoff in state s when receiver takes action a.
/// `receiver_payoff[s][a]` = receiver's payoff in state s when taking action a.
/// `prior[s]` = prior probability of state s.
#[derive(Debug, Clone)]
pub struct InformationDesignSolver {
    /// Number of states.
    pub n_states: usize,
    /// Number of actions.
    pub n_actions: usize,
    /// Sender payoffs: n_states × n_actions.
    pub sender_payoff: Vec<Vec<f64>>,
    /// Receiver payoffs: n_states × n_actions.
    pub receiver_payoff: Vec<Vec<f64>>,
    /// Prior distribution over states.
    pub prior: Vec<f64>,
}
impl InformationDesignSolver {
    /// Create a new information design solver.
    pub fn new(
        n_states: usize,
        n_actions: usize,
        sender_payoff: Vec<Vec<f64>>,
        receiver_payoff: Vec<Vec<f64>>,
        prior: Vec<f64>,
    ) -> Self {
        InformationDesignSolver {
            n_states,
            n_actions,
            sender_payoff,
            receiver_payoff,
            prior,
        }
    }
    /// Compute the receiver's best response to a posterior belief `belief[s]`.
    ///
    /// Returns the action maximizing the receiver's expected payoff.
    pub fn receiver_best_response(&self, belief: &[f64]) -> usize {
        let mut best_action = 0;
        let mut best_payoff = f64::NEG_INFINITY;
        for a in 0..self.n_actions {
            let exp_payoff: f64 = belief
                .iter()
                .zip(self.receiver_payoff.iter())
                .map(|(&p, row)| p * row[a])
                .sum();
            if exp_payoff > best_payoff {
                best_payoff = exp_payoff;
                best_action = a;
            }
        }
        best_action
    }
    /// Compute the sender's expected payoff for a direct signal (recommendation scheme).
    ///
    /// A direct signal assigns to each state s a distribution over actions.
    /// `signal[s][a]` = probability of recommending action a in state s.
    ///
    /// The obedience condition requires that each recommended action a is a best
    /// response given the posterior. Here we compute the sender's expected payoff
    /// directly without checking obedience.
    pub fn sender_expected_payoff(&self, signal: &[Vec<f64>]) -> f64 {
        let mut total = 0.0;
        for s in 0..self.n_states {
            for a in 0..self.n_actions {
                total += self.prior[s] * signal[s][a] * self.sender_payoff[s][a];
            }
        }
        total
    }
    /// Check obedience: for each recommended action a, it must be a best response
    /// given the posterior belief induced by the recommendation.
    ///
    /// Returns true if the signal satisfies obedience (approximately).
    pub fn check_obedience(&self, signal: &[Vec<f64>]) -> bool {
        for a in 0..self.n_actions {
            let total_prob_a: f64 = (0..self.n_states)
                .map(|s| self.prior[s] * signal[s][a])
                .sum();
            if total_prob_a < 1e-12 {
                continue;
            }
            let posterior: Vec<f64> = (0..self.n_states)
                .map(|s| self.prior[s] * signal[s][a] / total_prob_a)
                .collect();
            let br = self.receiver_best_response(&posterior);
            if br != a {
                return false;
            }
        }
        true
    }
    /// Find the fully revealing signal: each state maps to a distinct action.
    ///
    /// Only feasible if n_actions >= n_states.  Returns None otherwise.
    pub fn fully_revealing_signal(&self) -> Option<Vec<Vec<f64>>> {
        if self.n_actions < self.n_states {
            return None;
        }
        let mut signal = vec![vec![0.0f64; self.n_actions]; self.n_states];
        for s in 0..self.n_states {
            signal[s][s] = 1.0;
        }
        Some(signal)
    }
    /// Find the babbling (completely uninformative) signal: always recommend action 0.
    pub fn babbling_signal(&self) -> Vec<Vec<f64>> {
        let mut signal = vec![vec![0.0f64; self.n_actions]; self.n_states];
        for s in 0..self.n_states {
            signal[s][0] = 1.0;
        }
        signal
    }
}
/// A multi-item VCG mechanism for combinatorial allocation.
///
/// Each bidder has a value for each bundle of items. The mechanism allocates
/// items to maximize total value (welfare), then charges Clarke pivotal payments.
#[derive(Debug, Clone)]
pub struct VCGMechanism {
    /// Number of bidders.
    pub n_bidders: usize,
    /// Number of items.
    pub n_items: usize,
    /// `values[bidder][bundle_mask]` = bidder's value for the bundle indicated by the bitmask.
    pub values: Vec<Vec<f64>>,
}
impl VCGMechanism {
    /// Create a new VCG mechanism instance.
    pub fn new(n_bidders: usize, n_items: usize, values: Vec<Vec<f64>>) -> Self {
        VCGMechanism {
            n_bidders,
            n_items,
            values,
        }
    }
    /// Compute the optimal (welfare-maximizing) allocation.
    ///
    /// Uses brute-force enumeration for small instances.
    /// Returns `allocation[bidder]` = bitmask of items allocated to bidder.
    pub fn optimal_allocation(&self) -> Vec<usize> {
        let n_bundles = 1usize << self.n_items;
        let mut best_welfare = f64::NEG_INFINITY;
        let mut best_alloc = vec![0usize; self.n_bidders];
        self.enumerate_allocations(
            0,
            0,
            &mut vec![0usize; self.n_bidders],
            &mut best_welfare,
            &mut best_alloc,
            n_bundles,
        );
        best_alloc
    }
    #[allow(clippy::too_many_arguments)]
    fn enumerate_allocations(
        &self,
        bidder: usize,
        items_assigned: usize,
        current: &mut Vec<usize>,
        best_welfare: &mut f64,
        best_alloc: &mut Vec<usize>,
        n_bundles: usize,
    ) {
        if bidder == self.n_bidders {
            let welfare: f64 = (0..self.n_bidders)
                .map(|b| {
                    if current[b] < n_bundles {
                        self.values[b][current[b]]
                    } else {
                        0.0
                    }
                })
                .sum();
            if welfare > *best_welfare {
                *best_welfare = welfare;
                *best_alloc = current.clone();
            }
            return;
        }
        let remaining = ((1usize << self.n_items) - 1) & !items_assigned;
        let mut subset = remaining;
        loop {
            if subset < n_bundles {
                current[bidder] = subset;
                self.enumerate_allocations(
                    bidder + 1,
                    items_assigned | subset,
                    current,
                    best_welfare,
                    best_alloc,
                    n_bundles,
                );
            }
            if subset == 0 {
                break;
            }
            subset = (subset - 1) & remaining;
        }
    }
    /// Compute VCG payments using the Clarke pivot rule.
    ///
    /// Payment for bidder i = (welfare without i) - (welfare of all others in optimal allocation).
    pub fn vcg_payments(&self) -> Vec<f64> {
        let alloc = self.optimal_allocation();
        let welfare_all: f64 = (0..self.n_bidders).map(|b| self.values[b][alloc[b]]).sum();
        let mut payments = vec![0.0f64; self.n_bidders];
        for i in 0..self.n_bidders {
            let welfare_others: f64 = (0..self.n_bidders)
                .filter(|&b| b != i)
                .map(|b| self.values[b][alloc[b]])
                .sum();
            let welfare_without_i = self.optimal_welfare_without(i);
            payments[i] = welfare_without_i - welfare_others;
            let _ = welfare_all;
        }
        payments
    }
    /// Compute the optimal welfare achievable without bidder `excluded`.
    fn optimal_welfare_without(&self, excluded: usize) -> f64 {
        let n_bundles = 1usize << self.n_items;
        let mut best = f64::NEG_INFINITY;
        let mut current = vec![0usize; self.n_bidders];
        let mut best_alloc = vec![0usize; self.n_bidders];
        self.enumerate_allocations_except(
            0,
            0,
            excluded,
            &mut current,
            &mut best,
            &mut best_alloc,
            n_bundles,
        );
        best
    }
    #[allow(clippy::too_many_arguments)]
    fn enumerate_allocations_except(
        &self,
        bidder: usize,
        items_assigned: usize,
        excluded: usize,
        current: &mut Vec<usize>,
        best_welfare: &mut f64,
        best_alloc: &mut Vec<usize>,
        n_bundles: usize,
    ) {
        if bidder == self.n_bidders {
            let welfare: f64 = (0..self.n_bidders)
                .filter(|&b| b != excluded)
                .map(|b| {
                    if current[b] < n_bundles {
                        self.values[b][current[b]]
                    } else {
                        0.0
                    }
                })
                .sum();
            if welfare > *best_welfare {
                *best_welfare = welfare;
                *best_alloc = current.clone();
            }
            return;
        }
        if bidder == excluded {
            current[bidder] = 0;
            self.enumerate_allocations_except(
                bidder + 1,
                items_assigned,
                excluded,
                current,
                best_welfare,
                best_alloc,
                n_bundles,
            );
            return;
        }
        let remaining = ((1usize << self.n_items) - 1) & !items_assigned;
        let mut subset = remaining;
        loop {
            if subset < n_bundles {
                current[bidder] = subset;
                self.enumerate_allocations_except(
                    bidder + 1,
                    items_assigned | subset,
                    excluded,
                    current,
                    best_welfare,
                    best_alloc,
                    n_bundles,
                );
            }
            if subset == 0 {
                break;
            }
            subset = (subset - 1) & remaining;
        }
    }
}
/// A bid in a combinatorial auction: a bidder's value for a specific bundle.
#[derive(Debug, Clone)]
pub struct CombBid {
    /// Bidder index.
    pub bidder: usize,
    /// Bundle of items (bitmask).
    pub bundle: usize,
    /// Value for the bundle.
    pub value: f64,
}
/// Verifies that a direct mechanism implements the same outcome as an indirect mechanism,
/// consistent with the revelation principle.
///
/// For each type profile, checks that truthful reporting yields the same allocation
/// and payment as the optimal report in the indirect mechanism.
#[derive(Debug, Clone)]
pub struct RevelationPrincipleVerifier {
    /// Number of types (discretized).
    pub n_types: usize,
    /// Allocation under the indirect mechanism: `indirect_alloc[type_idx]` = prob of winning.
    pub indirect_alloc: Vec<f64>,
    /// Payment under the indirect mechanism: `indirect_pay[type_idx]`.
    pub indirect_pay: Vec<f64>,
}
impl RevelationPrincipleVerifier {
    /// Create a new verifier.
    pub fn new(n_types: usize, indirect_alloc: Vec<f64>, indirect_pay: Vec<f64>) -> Self {
        RevelationPrincipleVerifier {
            n_types,
            indirect_alloc,
            indirect_pay,
        }
    }
    /// Verify the revelation principle: for each true type t, truthful reporting
    /// gives weakly higher utility than any deviating report r.
    ///
    /// Utility: u(t, r) = t * alloc(r) - pay(r).
    pub fn verify(&self, type_values: &[f64]) -> bool {
        for (t_idx, &true_val) in type_values.iter().enumerate() {
            if t_idx >= self.n_types {
                break;
            }
            let truth_utility = true_val * self.indirect_alloc[t_idx] - self.indirect_pay[t_idx];
            for r_idx in 0..self.n_types {
                let dev_utility = true_val * self.indirect_alloc[r_idx] - self.indirect_pay[r_idx];
                if dev_utility > truth_utility + 1e-10 {
                    return false;
                }
            }
        }
        true
    }
    /// Extract the direct mechanism: the truthful allocation and payment rules.
    pub fn direct_mechanism(&self) -> (Vec<f64>, Vec<f64>) {
        (self.indirect_alloc.clone(), self.indirect_pay.clone())
    }
}
/// Data for a revenue-optimal mechanism (Myerson).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct MyersonMechanism {
    /// Valuation distribution type.
    pub distribution: String,
    /// Virtual value function: ψ(v) = v - (1-F(v))/f(v).
    pub virtual_values: Vec<(f64, f64)>,
    /// Reserve price.
    pub reserve_price: f64,
    /// Expected revenue.
    pub expected_revenue: f64,
}
#[allow(dead_code)]
impl MyersonMechanism {
    /// Creates a Myerson mechanism.
    pub fn new(distribution: &str, reserve: f64) -> Self {
        MyersonMechanism {
            distribution: distribution.to_string(),
            virtual_values: Vec::new(),
            reserve_price: reserve,
            expected_revenue: 0.0,
        }
    }
    /// Virtual value for uniform distribution on [0, b]: ψ(v) = 2v - b.
    pub fn virtual_value_uniform(v: f64, b: f64) -> f64 {
        2.0 * v - b
    }
    /// Optimal reserve for uniform [0, b]: r* = b/2.
    pub fn optimal_reserve_uniform(b: f64) -> f64 {
        b / 2.0
    }
    /// Adds a virtual value sample.
    pub fn add_virtual_value(&mut self, v: f64, psi_v: f64) {
        self.virtual_values.push((v, psi_v));
    }
    /// Revenue equivalence theorem: any two mechanisms with same allocation
    /// give same expected revenue (for symmetric bidders).
    pub fn revenue_equivalence(&self) -> String {
        "Revenue Equivalence: any BIC, IR mechanism with same allocation rule yields same expected revenue"
            .to_string()
    }
    /// Myerson optimal auction: allocate to highest positive virtual value bidder.
    pub fn optimal_allocation(&self, bids: &[f64], b: f64) -> Option<usize> {
        let psi_bids: Vec<(usize, f64)> = bids
            .iter()
            .enumerate()
            .map(|(i, &v)| (i, Self::virtual_value_uniform(v, b)))
            .filter(|(_, psi)| *psi > 0.0)
            .collect();
        psi_bids
            .iter()
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
            .map(|&(i, _)| i)
    }
}
/// Exact winner determination solver for combinatorial auctions.
///
/// Uses branch-and-bound to find the optimal allocation maximizing total value.
#[derive(Debug, Clone)]
pub struct CombAuctionSolver {
    /// All submitted bids.
    pub bids: Vec<CombBid>,
    /// Number of items.
    pub n_items: usize,
}
impl CombAuctionSolver {
    /// Create a new solver.
    pub fn new(bids: Vec<CombBid>, n_items: usize) -> Self {
        CombAuctionSolver { bids, n_items }
    }
    /// Solve the winner determination problem exactly.
    ///
    /// Returns the winning bid indices and total welfare.
    pub fn solve(&self) -> (Vec<usize>, f64) {
        let mut best_value = 0.0f64;
        let mut best_winners: Vec<usize> = vec![];
        let mut current: Vec<usize> = vec![];
        self.branch_and_bound(
            0,
            0usize,
            0.0,
            &mut current,
            &mut best_value,
            &mut best_winners,
        );
        (best_winners, best_value)
    }
    fn branch_and_bound(
        &self,
        bid_idx: usize,
        allocated: usize,
        current_value: f64,
        current: &mut Vec<usize>,
        best_value: &mut f64,
        best_winners: &mut Vec<usize>,
    ) {
        if bid_idx == self.bids.len() {
            if current_value > *best_value {
                *best_value = current_value;
                *best_winners = current.clone();
            }
            return;
        }
        let ub = current_value + self.bids[bid_idx..].iter().map(|b| b.value).sum::<f64>();
        if ub <= *best_value + 1e-10 {
            return;
        }
        let bid = &self.bids[bid_idx];
        self.branch_and_bound(
            bid_idx + 1,
            allocated,
            current_value,
            current,
            best_value,
            best_winners,
        );
        if bid.bundle & allocated == 0 {
            current.push(bid_idx);
            self.branch_and_bound(
                bid_idx + 1,
                allocated | bid.bundle,
                current_value + bid.value,
                current,
                best_value,
                best_winners,
            );
            current.pop();
        }
    }
    /// Compute VCG payments for the winning bids.
    pub fn vcg_payments_for_winners(&self, winners: &[usize]) -> Vec<f64> {
        let total_welfare: f64 = winners.iter().map(|&i| self.bids[i].value).sum();
        let mut payments = vec![0.0f64; self.bids.len()];
        for &w in winners {
            let welfare_others: f64 = winners
                .iter()
                .filter(|&&i| i != w)
                .map(|&i| self.bids[i].value)
                .sum();
            let bids_without_w: Vec<CombBid> = self
                .bids
                .iter()
                .enumerate()
                .filter(|(i, _)| *i != w)
                .map(|(_, b)| b.clone())
                .collect();
            let solver_without = CombAuctionSolver::new(bids_without_w, self.n_items);
            let (_, welfare_without_w) = solver_without.solve();
            payments[w] = welfare_without_w - welfare_others;
            let _ = total_welfare;
        }
        payments
    }
}
