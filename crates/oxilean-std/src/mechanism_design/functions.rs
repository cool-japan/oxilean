//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};
use std::collections::HashMap;

use super::types::{
    AuctionResult, BipartiteMatching, CombAuctionSolver, CombBid, DSICMechanism,
    DynamicVCGMechanism, InformationDesignSolver, MyersonMechanism, RevelationPrincipleVerifier,
    SocialWelfareFunction, VCGMechanism, VickreyAuction,
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
pub fn list_ty(elem: Expr) -> Expr {
    app(cst("List"), elem)
}
/// `Mechanism : Nat → Nat → Prop`
/// A mechanism for n agents over m outcomes.
pub fn mechanism_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), prop()))
}
/// `IncentiveCompatible : Prop → Prop`
/// A mechanism is incentive-compatible if truthful reporting is a dominant strategy.
pub fn incentive_compatible_ty() -> Expr {
    arrow(prop(), prop())
}
/// `DominantStrategyIC : Prop → Prop`
/// DSIC: truthful reporting is weakly dominant regardless of others' reports.
pub fn dominant_strategy_ic_ty() -> Expr {
    arrow(prop(), prop())
}
/// `BayesianIC : Prop → Prop`
/// BIC: truthful reporting maximizes expected utility in Bayesian Nash equilibrium.
pub fn bayesian_ic_ty() -> Expr {
    arrow(prop(), prop())
}
/// `IndividuallyRational : Prop → Prop`
/// IR: every agent weakly prefers participating to not participating.
pub fn individually_rational_ty() -> Expr {
    arrow(prop(), prop())
}
/// `VCGMechanism : Nat → Prop`
/// VCG: Vickrey-Clarke-Groves mechanism for n agents.
pub fn vcg_mechanism_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `SecondPriceAuction : Prop`
/// Vickrey second-price sealed-bid auction is DSIC and efficient.
pub fn second_price_auction_ty() -> Expr {
    prop()
}
/// `OptimalAuction : Nat → Prop`
/// Myerson's optimal auction maximizes expected revenue with n bidders.
pub fn optimal_auction_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `GaleShapleyMatch : Nat → Nat → Prop`
/// Gale-Shapley stable matching between n proposers and n reviewers.
pub fn gale_shapley_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), prop()))
}
/// `StableMatching : Prop → Prop`
/// A matching is stable if no blocking pair exists.
pub fn stable_matching_ty() -> Expr {
    arrow(prop(), prop())
}
/// `RevenueEquivalence : Prop`
/// Revenue equivalence theorem: all efficient DSIC auctions yield the same expected revenue.
pub fn revenue_equivalence_ty() -> Expr {
    prop()
}
/// `RevelationPrinciple : Prop → Prop`
/// Any BIC mechanism can be implemented as a truthful direct mechanism.
pub fn revelation_principle_ty() -> Expr {
    arrow(prop(), prop())
}
/// `VCGDominant : ∀ n, VCGMechanism n → DominantStrategyIC`
/// VCG mechanism is DSIC.
pub fn vcg_dominant_ty() -> Expr {
    arrow(nat_ty(), arrow(app(cst("VCGMechanism"), bvar(0)), prop()))
}
/// `MyersonOptimality : ∀ n, OptimalAuction n maximizes revenue`
pub fn myerson_optimality_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `GaleShapleyStability : the deferred acceptance algorithm produces a stable matching`
pub fn gale_shapley_stability_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `RevelationPrincipleThm : every BIC mechanism has an equivalent direct truthful mechanism`
pub fn revelation_principle_thm_ty() -> Expr {
    arrow(prop(), prop())
}
/// Build the mechanism design environment: register all axioms as opaque constants.
pub fn build_mechanism_design_env(env: &mut Environment) {
    let axioms: &[(&str, Expr)] = &[
        ("Mechanism", mechanism_ty()),
        ("IncentiveCompatible", incentive_compatible_ty()),
        ("DominantStrategyIC", dominant_strategy_ic_ty()),
        ("BayesianIC", bayesian_ic_ty()),
        ("IndividuallyRational", individually_rational_ty()),
        ("VCGMechanism", vcg_mechanism_ty()),
        ("SecondPriceAuction", second_price_auction_ty()),
        ("OptimalAuction", optimal_auction_ty()),
        ("GaleShapleyMatch", gale_shapley_ty()),
        ("StableMatching", stable_matching_ty()),
        ("RevenueEquivalence", revenue_equivalence_ty()),
        ("RevelationPrinciple", revelation_principle_ty()),
        ("vcg_dominant", vcg_dominant_ty()),
        ("myerson_optimality", myerson_optimality_ty()),
        ("gale_shapley_stability", gale_shapley_stability_ty()),
        ("revelation_principle_thm", revelation_principle_thm_ty()),
        (
            "AllocationRule",
            arrow(list_ty(real_ty()), list_ty(real_ty())),
        ),
        ("PaymentRule", arrow(list_ty(real_ty()), list_ty(real_ty()))),
        (
            "VirtualValue",
            arrow(real_ty(), arrow(real_ty(), real_ty())),
        ),
        ("IronedVirtualValue", arrow(real_ty(), real_ty())),
        (
            "WinnerDetermination",
            arrow(nat_ty(), arrow(list_ty(real_ty()), nat_ty())),
        ),
        ("PostedPrice", arrow(real_ty(), bool_ty())),
        (
            "CombinatorialAuction",
            arrow(nat_ty(), arrow(nat_ty(), prop())),
        ),
        ("Deferred Acceptance", arrow(nat_ty(), prop())),
        ("BlockingPair", arrow(nat_ty(), arrow(nat_ty(), bool_ty()))),
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
/// Run a sealed-bid second-price (Vickrey) auction.
///
/// The highest bidder wins and pays the second-highest bid.
/// If there are fewer than 2 bidders, the winner pays 0.
///
/// VCG mechanism for single-item allocation: DSIC and efficient.
pub fn second_price_auction(bids: &[f64]) -> AuctionResult {
    if bids.is_empty() {
        return AuctionResult {
            winner: None,
            payments: vec![],
        };
    }
    let mut payments = vec![0.0f64; bids.len()];
    let mut highest_idx = 0;
    let mut highest = f64::NEG_INFINITY;
    let mut second_highest = f64::NEG_INFINITY;
    for (i, &bid) in bids.iter().enumerate() {
        if bid > highest {
            second_highest = highest;
            highest = bid;
            highest_idx = i;
        } else if bid > second_highest {
            second_highest = bid;
        }
    }
    let payment = if second_highest == f64::NEG_INFINITY {
        0.0
    } else {
        second_highest
    };
    payments[highest_idx] = payment;
    AuctionResult {
        winner: Some(highest_idx),
        payments,
    }
}
/// Run a sealed-bid first-price auction.
///
/// The highest bidder wins and pays their own bid.
/// Note: this is NOT DSIC (bidders shade their bids below true value).
pub fn first_price_auction(bids: &[f64]) -> AuctionResult {
    if bids.is_empty() {
        return AuctionResult {
            winner: None,
            payments: vec![],
        };
    }
    let mut payments = vec![0.0f64; bids.len()];
    let highest_idx = bids
        .iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
        .map(|(i, _)| i)
        .unwrap_or(0);
    payments[highest_idx] = bids[highest_idx];
    AuctionResult {
        winner: Some(highest_idx),
        payments,
    }
}
/// Run an all-pay auction (e.g., war of attrition, lobbying).
///
/// The highest bidder wins, but ALL bidders pay their bids.
pub fn all_pay_auction(bids: &[f64]) -> AuctionResult {
    if bids.is_empty() {
        return AuctionResult {
            winner: None,
            payments: vec![],
        };
    }
    let payments = bids.to_vec();
    let winner_idx = bids
        .iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
        .map(|(i, _)| i)
        .unwrap_or(0);
    AuctionResult {
        winner: Some(winner_idx),
        payments,
    }
}
/// Compute Myerson's virtual value for a bidder with uniform valuation.
///
/// For uniform distribution on \[0, 1\]: ψ(v) = v - (1 - F(v)) / f(v) = 2v - 1.
pub fn myerson_virtual_value_uniform(v: f64) -> f64 {
    2.0 * v - 1.0
}
/// Myerson's optimal auction for n bidders with i.i.d. uniform values on \[0,1\].
///
/// Allocates the item to the bidder with the highest non-negative virtual value.
/// Returns (winner_index, payment).
pub fn myerson_optimal_auction(values: &[f64]) -> AuctionResult {
    let virtual_values: Vec<f64> = values
        .iter()
        .map(|&v| myerson_virtual_value_uniform(v))
        .collect();
    let mut payments = vec![0.0f64; values.len()];
    let best = virtual_values
        .iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    match best {
        Some((idx, &vv)) if vv >= 0.0 => {
            let second_vv = virtual_values
                .iter()
                .enumerate()
                .filter(|(i, _)| *i != idx)
                .map(|(_, &vv)| vv)
                .fold(f64::NEG_INFINITY, f64::max);
            let threshold_vv = second_vv.max(0.0);
            payments[idx] = (threshold_vv + 1.0) / 2.0;
            AuctionResult {
                winner: Some(idx),
                payments,
            }
        }
        _ => AuctionResult {
            winner: None,
            payments,
        },
    }
}
/// A posted-price mechanism: the seller posts a take-it-or-leave-it price.
/// Returns (sold, revenue).
pub fn posted_price(price: f64, bidder_values: &[f64]) -> (bool, f64) {
    for &v in bidder_values {
        if v >= price {
            return (true, price);
        }
    }
    (false, 0.0)
}
/// Compute the optimal posted price for i.i.d. bidders with known value distribution.
/// Evaluates a range of price candidates and picks the one maximizing expected revenue.
///
/// `values`: samples from the value distribution.
/// Returns the revenue-maximizing posted price and its expected revenue.
pub fn optimal_posted_price(values: &[f64], n_bidders: usize) -> (f64, f64) {
    if values.is_empty() {
        return (0.0, 0.0);
    }
    let mut best_price = 0.0;
    let mut best_revenue = 0.0;
    for &candidate_price in values {
        let prob_accept =
            values.iter().filter(|&&v| v >= candidate_price).count() as f64 / values.len() as f64;
        let prob_sale = 1.0 - (1.0 - prob_accept).powi(n_bidders as i32);
        let revenue = candidate_price * prob_sale;
        if revenue > best_revenue {
            best_revenue = revenue;
            best_price = candidate_price;
        }
    }
    (best_price, best_revenue)
}
/// Check if a direct mechanism is dominant-strategy incentive compatible (DSIC).
///
/// A mechanism (allocation_fn, payment_fn) is DSIC if, for every agent i,
/// reporting truthfully maximizes utility regardless of others' reports.
///
/// This simplified check verifies IC for a single agent given fixed reports from others.
/// Returns true if truthful reporting yields weakly higher utility than any deviation.
pub fn check_dsic_single_agent(
    true_value: f64,
    candidate_reports: &[f64],
    allocation_fn: impl Fn(f64) -> f64,
    payment_fn: impl Fn(f64) -> f64,
) -> bool {
    let truth_alloc = allocation_fn(true_value);
    let truth_payment = payment_fn(true_value);
    let truth_utility = true_value * truth_alloc - truth_payment;
    for &report in candidate_reports {
        let alloc = allocation_fn(report);
        let payment = payment_fn(report);
        let utility = true_value * alloc - payment;
        if utility > truth_utility + 1e-10 {
            return false;
        }
    }
    true
}
/// Check individual rationality: participation is weakly preferred to abstaining.
///
/// Returns true if the agent's utility from participating (with truthful report) ≥ 0.
pub fn check_individual_rationality(
    true_value: f64,
    allocation_fn: impl Fn(f64) -> f64,
    payment_fn: impl Fn(f64) -> f64,
) -> bool {
    let alloc = allocation_fn(true_value);
    let payment = payment_fn(true_value);
    true_value * alloc - payment >= -1e-10
}
/// Run the Gale-Shapley deferred acceptance algorithm (proposer-optimal).
///
/// `proposer_prefs\[i\]` = list of reviewers in order of preference for proposer i.
/// `reviewer_prefs\[j\]` = list of proposers in order of preference for reviewer j.
///
/// Returns `matching[proposer]` = reviewer matched to that proposer (or None if unmatched).
pub fn gale_shapley(
    proposer_prefs: &[Vec<usize>],
    reviewer_prefs: &[Vec<usize>],
) -> Vec<Option<usize>> {
    let n_proposers = proposer_prefs.len();
    let n_reviewers = reviewer_prefs.len();
    let mut reviewer_rank: Vec<Vec<usize>> = vec![vec![0; n_proposers]; n_reviewers];
    for j in 0..n_reviewers {
        for (rank, &proposer) in reviewer_prefs[j].iter().enumerate() {
            if proposer < n_proposers {
                reviewer_rank[j][proposer] = rank;
            }
        }
    }
    let mut proposer_match: Vec<Option<usize>> = vec![None; n_proposers];
    let mut reviewer_match: Vec<Option<usize>> = vec![None; n_reviewers];
    let mut next_proposal: Vec<usize> = vec![0; n_proposers];
    let mut free: Vec<usize> = (0..n_proposers).collect();
    while !free.is_empty() {
        let proposer = free.remove(0);
        if next_proposal[proposer] >= proposer_prefs[proposer].len() {
            continue;
        }
        let reviewer = proposer_prefs[proposer][next_proposal[proposer]];
        next_proposal[proposer] += 1;
        if reviewer >= n_reviewers {
            free.push(proposer);
            continue;
        }
        match reviewer_match[reviewer] {
            None => {
                reviewer_match[reviewer] = Some(proposer);
                proposer_match[proposer] = Some(reviewer);
            }
            Some(current_proposer) => {
                let current_rank = reviewer_rank[reviewer][current_proposer];
                let new_rank = reviewer_rank[reviewer][proposer];
                if new_rank < current_rank {
                    reviewer_match[reviewer] = Some(proposer);
                    proposer_match[proposer] = Some(reviewer);
                    proposer_match[current_proposer] = None;
                    free.push(current_proposer);
                } else {
                    free.push(proposer);
                }
            }
        }
    }
    proposer_match
}
/// Verify that a matching is stable: no blocking pair (i, j) exists where
/// both proposer i and reviewer j prefer each other to their current matches.
pub fn is_stable_matching(
    matching: &[Option<usize>],
    proposer_prefs: &[Vec<usize>],
    reviewer_prefs: &[Vec<usize>],
) -> bool {
    let n_proposers = proposer_prefs.len();
    let n_reviewers = reviewer_prefs.len();
    let mut reviewer_match: Vec<Option<usize>> = vec![None; n_reviewers];
    for (p, &r_opt) in matching.iter().enumerate() {
        if let Some(r) = r_opt {
            if r < n_reviewers {
                reviewer_match[r] = Some(p);
            }
        }
    }
    let mut proposer_rank: Vec<HashMap<usize, usize>> = vec![HashMap::new(); n_proposers];
    for i in 0..n_proposers {
        for (rank, &r) in proposer_prefs[i].iter().enumerate() {
            proposer_rank[i].insert(r, rank);
        }
    }
    let mut reviewer_rank: Vec<HashMap<usize, usize>> = vec![HashMap::new(); n_reviewers];
    for j in 0..n_reviewers {
        for (rank, &p) in reviewer_prefs[j].iter().enumerate() {
            reviewer_rank[j].insert(p, rank);
        }
    }
    for i in 0..n_proposers {
        for j in 0..n_reviewers {
            if matching[i] == Some(j) {
                continue;
            }
            let i_prefers_j = match matching[i] {
                None => true,
                Some(current_j) => {
                    let rank_j = proposer_rank[i].get(&j).copied().unwrap_or(usize::MAX);
                    let rank_cur = proposer_rank[i]
                        .get(&current_j)
                        .copied()
                        .unwrap_or(usize::MAX);
                    rank_j < rank_cur
                }
            };
            if !i_prefers_j {
                continue;
            }
            let j_prefers_i = match reviewer_match[j] {
                None => true,
                Some(current_i) => {
                    let rank_i = reviewer_rank[j].get(&i).copied().unwrap_or(usize::MAX);
                    let rank_cur = reviewer_rank[j]
                        .get(&current_i)
                        .copied()
                        .unwrap_or(usize::MAX);
                    rank_i < rank_cur
                }
            };
            if j_prefers_i {
                return false;
            }
        }
    }
    true
}
/// Greedy winner determination for combinatorial auctions.
///
/// Sorts bids by value-to-bundle-size ratio and greedily assigns non-overlapping bundles.
/// Returns the winning bids (as indices into the input slice).
pub fn greedy_winner_determination(bids: &[CombBid], n_items: usize) -> Vec<usize> {
    let _ = n_items;
    let mut sorted_indices: Vec<usize> = (0..bids.len()).collect();
    sorted_indices.sort_by(|&a, &b| {
        bids[b]
            .value
            .partial_cmp(&bids[a].value)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    let mut allocated_items = 0usize;
    let mut winners = Vec::new();
    for idx in sorted_indices {
        let bid = &bids[idx];
        if bid.bundle & allocated_items == 0 {
            winners.push(idx);
            allocated_items |= bid.bundle;
        }
    }
    winners
}
/// Verify revenue equivalence numerically: for i.i.d. bidders with uniform values,
/// the expected revenue of the second-price auction and the first-price auction
/// should be approximately equal.
///
/// For n bidders with values uniform on \[0,1\]:
/// Expected revenue of SPA = n/(n+1) * E\[max value\] = (n-1)/(n+1)
/// Expected revenue of FPA (symmetric equilibrium bid = v*(n-1)/n) = (n-1)/(n+1)
///
/// Returns the empirical expected revenues of SPA and FPA respectively.
pub fn revenue_equivalence_check(values_samples: &[Vec<f64>]) -> (f64, f64) {
    let mut spa_revenue = 0.0;
    let mut fpa_revenue = 0.0;
    let n_samples = values_samples.len() as f64;
    for sample in values_samples {
        let spa = second_price_auction(sample);
        let spa_rev: f64 = spa.payments.iter().sum();
        spa_revenue += spa_rev;
        let fpa = first_price_auction(sample);
        let fpa_rev: f64 = fpa.payments.iter().sum();
        fpa_revenue += fpa_rev;
    }
    (spa_revenue / n_samples, fpa_revenue / n_samples)
}
/// `AdverseSelection : Prop → Prop`
/// Adverse selection: the uninformed party cannot distinguish types before contracting.
pub fn adverse_selection_ty() -> Expr {
    arrow(prop(), prop())
}
/// `MoralHazard : Prop → Prop`
/// Moral hazard: the agent's effort/action is unobservable after contracting.
pub fn moral_hazard_ty() -> Expr {
    arrow(prop(), prop())
}
/// `ScreeningContract : Nat → Prop`
/// A screening contract for n types that satisfies IC and IR across types.
pub fn screening_contract_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `DelegationProblem : Prop → Prop → Prop`
/// The delegation problem: principal delegates decision to better-informed agent.
pub fn delegation_problem_ty() -> Expr {
    arrow(prop(), arrow(prop(), prop()))
}
/// `BundlingEffect : Nat → Nat → Prop`
/// Bundling n goods with m types: whether bundling increases revenue.
pub fn bundling_effect_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), prop()))
}
/// `DynamicRevelationPrinciple : Prop → Prop`
/// Dynamic mechanism design: the revelation principle extends to multi-period settings.
pub fn dynamic_revelation_principle_ty() -> Expr {
    arrow(prop(), prop())
}
/// `MirrleesApproach : Prop`
/// The Mirrlees approach to optimal income taxation: characterizes optimal contracts
/// using local IC constraints (envelope conditions).
pub fn mirrlees_approach_ty() -> Expr {
    prop()
}
/// `RobustMechanism : Prop → Prop`
/// Robust mechanism design: maximizes worst-case welfare/revenue without a prior.
pub fn robust_mechanism_ty() -> Expr {
    arrow(prop(), prop())
}
/// `MaxMinMechanism : Prop → Prop`
/// Max-min mechanism: maximizes minimum possible payoff over all type profiles.
pub fn max_min_mechanism_ty() -> Expr {
    arrow(prop(), prop())
}
/// `PriorFreeMechanism : Prop → Prop`
/// Prior-free (detail-free) mechanism: performance guarantee independent of distribution.
pub fn prior_free_mechanism_ty() -> Expr {
    arrow(prop(), prop())
}
/// `RegretMinimization : Prop → Prop`
/// Regret minimization in mechanism design: bound on worst-case regret.
pub fn regret_minimization_ty() -> Expr {
    arrow(prop(), prop())
}
/// `WinnerDeterminationProblem : Nat → Nat → Prop`
/// The winner determination problem (WDP) for n bidders and m items: NP-hard in general.
pub fn winner_determination_problem_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), prop()))
}
/// `IterativeAuction : Nat → Prop`
/// Iterative (ascending) combinatorial auction for n items.
pub fn iterative_auction_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `ShapleyValueCostSharing : Nat → Prop`
/// Shapley value cost sharing: unique cost allocation satisfying efficiency, symmetry, dummy, additivity.
pub fn shapley_value_cost_sharing_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `CrossMonotonicity : Prop → Prop`
/// Cross-monotone cost sharing: a player's share decreases as the set grows.
pub fn cross_monotonicity_ty() -> Expr {
    arrow(prop(), prop())
}
/// `StrategyproofMatching : Nat → Prop`
/// A matching mechanism is strategyproof if truthful reporting is dominant for all agents.
pub fn strategyproof_matching_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `MatchingEfficiency : Prop → Prop`
/// A matching is Pareto efficient (no blocking coalition or improving trade).
pub fn matching_efficiency_ty() -> Expr {
    arrow(prop(), prop())
}
/// `BayesianPersuasion : Prop → Prop`
/// Bayesian persuasion (Kamenica-Gentzkow): sender chooses a signal to maximize payoff.
pub fn bayesian_persuasion_ty() -> Expr {
    arrow(prop(), prop())
}
/// `ObedientStrategies : Prop → Prop`
/// Information design obedience: receivers act on their recommended action.
pub fn obedient_strategies_ty() -> Expr {
    arrow(prop(), prop())
}
/// `SinglePeakedPreferences : Nat → Prop`
/// Single-peaked preferences: each voter has a unique peak and prefers alternatives closer to it.
pub fn single_peaked_preferences_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `MedianVoterTheorem : Prop`
/// The median voter theorem: the median voter's preferred policy wins under majority rule with single-peaked prefs.
pub fn median_voter_theorem_ty() -> Expr {
    prop()
}
/// `SocialWelfareFunction : Nat → Prop`
/// A social welfare function mapping preference profiles to a social ordering.
pub fn social_welfare_function_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `StrongBudgetBalance : Prop → Prop`
/// Strong budget balance: total payments exactly equal zero (no money left/burned).
pub fn strong_budget_balance_ty() -> Expr {
    arrow(prop(), prop())
}
/// `ExPostBudgetBalance : Prop → Prop`
/// Ex post budget balance: payments sum to zero for every realized type profile.
pub fn ex_post_budget_balance_ty() -> Expr {
    arrow(prop(), prop())
}
/// `RevenueApproximation : Real → Prop`
/// α-approximation to optimal revenue: mechanism achieves at least α fraction of optimal revenue.
pub fn revenue_approximation_ty() -> Expr {
    arrow(real_ty(), prop())
}
/// `ProphetInequality : Prop`
/// The prophet inequality: a single-threshold policy achieves at least 1/2 of the prophet's expected value.
pub fn prophet_inequality_ty() -> Expr {
    prop()
}
/// `TruthfulApproximation : Real → Prop`
/// Truthful mechanism achieving a (1/α)-approximation to social welfare.
pub fn truthful_approximation_ty() -> Expr {
    arrow(real_ty(), prop())
}
pub fn build_mechanism_design_env_extended(env: &mut Environment) {
    build_mechanism_design_env(env);
    let axioms: &[(&str, Expr)] = &[
        ("AdverseSelection", adverse_selection_ty()),
        ("MoralHazard", moral_hazard_ty()),
        ("ScreeningContract", screening_contract_ty()),
        ("DelegationProblem", delegation_problem_ty()),
        ("BundlingEffect", bundling_effect_ty()),
        (
            "DynamicRevelationPrinciple",
            dynamic_revelation_principle_ty(),
        ),
        ("MirrleesApproach", mirrlees_approach_ty()),
        ("RobustMechanism", robust_mechanism_ty()),
        ("MaxMinMechanism", max_min_mechanism_ty()),
        ("PriorFreeMechanism", prior_free_mechanism_ty()),
        ("RegretMinimization", regret_minimization_ty()),
        (
            "WinnerDeterminationProblem",
            winner_determination_problem_ty(),
        ),
        ("IterativeAuction", iterative_auction_ty()),
        ("ShapleyValueCostSharing", shapley_value_cost_sharing_ty()),
        ("CrossMonotonicity", cross_monotonicity_ty()),
        ("StrategyproofMatching", strategyproof_matching_ty()),
        ("MatchingEfficiency", matching_efficiency_ty()),
        ("BayesianPersuasion", bayesian_persuasion_ty()),
        ("ObedientStrategies", obedient_strategies_ty()),
        ("SinglePeakedPreferences", single_peaked_preferences_ty()),
        ("median_voter_theorem", median_voter_theorem_ty()),
        ("SocialWelfareFunction", social_welfare_function_ty()),
        ("StrongBudgetBalance", strong_budget_balance_ty()),
        ("ExPostBudgetBalance", ex_post_budget_balance_ty()),
        ("RevenueApproximation", revenue_approximation_ty()),
        ("prophet_inequality", prophet_inequality_ty()),
        ("TruthfulApproximation", truthful_approximation_ty()),
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
/// Compute the Shapley value for a coalitional cost-sharing game.
///
/// `cost_fn(S)` = cost of serving coalition S (given as a bitmask).
/// Returns the Shapley value for each agent.
pub fn shapley_value_cost(n_agents: usize, cost_fn: impl Fn(usize) -> f64) -> Vec<f64> {
    let mut values = vec![0.0f64; n_agents];
    let n_coalitions = 1usize << n_agents;
    let mut fact = vec![1u64; n_agents + 1];
    for i in 1..=n_agents {
        fact[i] = fact[i - 1] * i as u64;
    }
    for i in 0..n_agents {
        for s_mask in 0..n_coalitions {
            if s_mask & (1 << i) == 0 {
                continue;
            }
            let s_without_i = s_mask & !(1 << i);
            let s_size = (s_mask.count_ones() - 1) as usize;
            let weight = fact[s_size] * fact[n_agents - s_size - 1];
            let marginal = cost_fn(s_mask) - cost_fn(s_without_i);
            values[i] += weight as f64 * marginal;
        }
        values[i] /= fact[n_agents] as f64;
    }
    values
}
/// Check if a cost-sharing method is cross-monotone:
/// agent i's share does not increase when new agents join.
///
/// `shares\[S\]\[i\]` = share of agent i in coalition S (bitmask).
/// Returns true if for all i ∈ S ⊆ T, shares\[S\]\[i\] >= shares\[T\]\[i\].
pub fn is_cross_monotone(n_agents: usize, shares: &[Vec<f64>]) -> bool {
    let n_coalitions = 1usize << n_agents;
    for s in 0..n_coalitions {
        for t in 0..n_coalitions {
            if s & t != s {
                continue;
            }
            for i in 0..n_agents {
                if s & (1 << i) == 0 {
                    continue;
                }
                if shares[s][i] < shares[t][i] - 1e-10 {
                    return false;
                }
            }
        }
    }
    true
}
/// Compute the expected value of the optimal offline algorithm (prophet) for a sequence
/// of independent random variables with given distributions (given as samples).
///
/// The prophet knows all values in advance and picks the maximum.
/// Returns the empirical expected maximum.
pub fn prophet_expected_value(value_samples: &[Vec<f64>]) -> f64 {
    if value_samples.is_empty() {
        return 0.0;
    }
    let sum: f64 = value_samples
        .iter()
        .map(|sample| sample.iter().cloned().fold(f64::NEG_INFINITY, f64::max))
        .sum();
    sum / value_samples.len() as f64
}
/// Single-threshold policy for the secretary/prophet problem.
///
/// Sets a threshold τ and accepts the first value ≥ τ.
/// Returns the expected value achieved by this policy.
pub fn single_threshold_policy(value_samples: &[Vec<f64>], threshold: f64) -> f64 {
    if value_samples.is_empty() {
        return 0.0;
    }
    let sum: f64 = value_samples
        .iter()
        .map(|sample| {
            for &v in sample {
                if v >= threshold {
                    return v;
                }
            }
            0.0
        })
        .sum();
    sum / value_samples.len() as f64
}
/// Find the approximately optimal single threshold for the prophet inequality.
///
/// Returns the threshold achieving ≥ 1/2 of the prophet's expected value.
pub fn find_prophet_threshold(value_samples: &[Vec<f64>]) -> f64 {
    if value_samples.is_empty() {
        return 0.0;
    }
    let mut candidates: Vec<f64> = value_samples.iter().flatten().cloned().collect();
    candidates.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    candidates.dedup_by(|a, b| (*a - *b).abs() < 1e-12);
    let prophet = prophet_expected_value(value_samples);
    let mut best_thresh = 0.0f64;
    let mut best_val = 0.0f64;
    for &t in &candidates {
        let v = single_threshold_policy(value_samples, t);
        if v > best_val {
            best_val = v;
            best_thresh = t;
        }
    }
    let _ = prophet;
    best_thresh
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_second_price_auction_basic() {
        let bids = vec![10.0, 7.0, 5.0];
        let result = second_price_auction(&bids);
        assert_eq!(result.winner, Some(0), "Highest bidder wins");
        assert!(
            (result.payments[0] - 7.0).abs() < 1e-10,
            "Winner pays second-price (7.0), got {}",
            result.payments[0]
        );
        assert_eq!(result.payments[1], 0.0, "Losers pay 0");
        assert_eq!(result.payments[2], 0.0, "Losers pay 0");
    }
    #[test]
    fn test_second_price_dsic() {
        let bids_truthful = vec![10.0, 7.0, 5.0];
        let res_truth = second_price_auction(&bids_truthful);
        let utility_truth = 10.0 - res_truth.payments[0];
        let bids_overbid = vec![15.0, 7.0, 5.0];
        let res_over = second_price_auction(&bids_overbid);
        let utility_over = 10.0 - res_over.payments[0];
        let bids_underbid = vec![6.0, 7.0, 5.0];
        let res_under = second_price_auction(&bids_underbid);
        let utility_under = if res_under.winner == Some(0) {
            10.0 - res_under.payments[0]
        } else {
            0.0
        };
        assert!(
            utility_truth >= utility_over - 1e-10,
            "Truthful utility ({utility_truth}) should be >= overbid utility ({utility_over})"
        );
        assert!(
            utility_truth >= utility_under - 1e-10,
            "Truthful utility ({utility_truth}) should be >= underbid utility ({utility_under})"
        );
    }
    #[test]
    fn test_myerson_optimal_auction() {
        let result_below = myerson_optimal_auction(&[0.4]);
        assert_eq!(
            result_below.winner, None,
            "Value below reserve should not win"
        );
        let result_above = myerson_optimal_auction(&[0.6]);
        assert_eq!(
            result_above.winner,
            Some(0),
            "Value above reserve should win"
        );
        let result_two = myerson_optimal_auction(&[0.8, 0.7]);
        assert_eq!(result_two.winner, Some(0), "Higher value bidder wins");
    }
    #[test]
    fn test_gale_shapley_stability() {
        let proposer_prefs = vec![vec![0, 1, 2], vec![0, 1, 2], vec![0, 1, 2]];
        let reviewer_prefs = vec![vec![0, 1, 2], vec![0, 1, 2], vec![0, 1, 2]];
        let matching = gale_shapley(&proposer_prefs, &reviewer_prefs);
        assert_eq!(matching[0], Some(0), "P0 matches R0");
        assert_eq!(matching[1], Some(1), "P1 matches R1");
        assert_eq!(matching[2], Some(2), "P2 matches R2");
        assert!(
            is_stable_matching(&matching, &proposer_prefs, &reviewer_prefs),
            "Matching should be stable"
        );
    }
    #[test]
    fn test_gale_shapley_proposer_optimal() {
        let proposer_prefs = vec![vec![0, 1], vec![0, 1]];
        let reviewer_prefs = vec![vec![1, 0], vec![0, 1]];
        let matching = gale_shapley(&proposer_prefs, &reviewer_prefs);
        assert_eq!(matching[0], Some(1), "P0 matches R1");
        assert_eq!(matching[1], Some(0), "P1 matches R0");
        assert!(
            is_stable_matching(&matching, &proposer_prefs, &reviewer_prefs),
            "Matching should be stable"
        );
    }
    #[test]
    fn test_greedy_combinatorial_auction() {
        let bids = vec![
            CombBid {
                bidder: 0,
                bundle: 0b01,
                value: 10.0,
            },
            CombBid {
                bidder: 1,
                bundle: 0b10,
                value: 8.0,
            },
            CombBid {
                bidder: 2,
                bundle: 0b11,
                value: 15.0,
            },
        ];
        let winners = greedy_winner_determination(&bids, 2);
        assert_eq!(winners.len(), 1, "One winner for the full bundle");
        assert_eq!(winners[0], 2, "Bidder 2 wins with the full bundle");
    }
    #[test]
    fn test_posted_price() {
        let values = vec![3.0, 5.0, 8.0, 2.0];
        let (sold, revenue) = posted_price(6.0, &values);
        assert!(sold, "Item should sell at price 6");
        assert!((revenue - 6.0).abs() < 1e-10, "Revenue should be 6");
        let (sold_10, revenue_10) = posted_price(10.0, &values);
        assert!(!sold_10, "Item should not sell at price 10");
        assert_eq!(revenue_10, 0.0);
    }
    #[test]
    fn test_check_dsic_single_agent() {
        let threshold = 5.0;
        let alloc_fn = |r: f64| if r >= threshold { 1.0 } else { 0.0 };
        let pay_fn = |r: f64| if r >= threshold { threshold } else { 0.0 };
        let candidates: Vec<f64> = (0..10).map(|i| i as f64).collect();
        assert!(
            check_dsic_single_agent(7.0, &candidates, alloc_fn, pay_fn),
            "Second-price mechanism should be DSIC for value 7"
        );
        let alloc_fn2 = |r: f64| if r >= threshold { 1.0 } else { 0.0 };
        let pay_fn2 = |r: f64| if r >= threshold { threshold } else { 0.0 };
        assert!(
            check_dsic_single_agent(3.0, &candidates, alloc_fn2, pay_fn2),
            "Second-price mechanism should be DSIC for value 3"
        );
    }
    #[test]
    fn test_build_mechanism_design_env() {
        let mut env = oxilean_kernel::Environment::new();
        build_mechanism_design_env(&mut env);
        assert!(
            env.get(&oxilean_kernel::Name::str("VCGMechanism"))
                .is_some(),
            "VCGMechanism should be in environment"
        );
        assert!(
            env.get(&oxilean_kernel::Name::str("GaleShapleyMatch"))
                .is_some(),
            "GaleShapleyMatch should be in environment"
        );
        assert!(
            env.get(&oxilean_kernel::Name::str("OptimalAuction"))
                .is_some(),
            "OptimalAuction should be in environment"
        );
    }
    #[test]
    fn test_build_mechanism_design_env_extended() {
        let mut env = oxilean_kernel::Environment::new();
        build_mechanism_design_env_extended(&mut env);
        assert!(
            env.get(&oxilean_kernel::Name::str("AdverseSelection"))
                .is_some(),
            "AdverseSelection should be in extended environment"
        );
        assert!(
            env.get(&oxilean_kernel::Name::str("BayesianPersuasion"))
                .is_some(),
            "BayesianPersuasion should be in extended environment"
        );
        assert!(
            env.get(&oxilean_kernel::Name::str("prophet_inequality"))
                .is_some(),
            "prophet_inequality should be in extended environment"
        );
        assert!(
            env.get(&oxilean_kernel::Name::str("StrongBudgetBalance"))
                .is_some(),
            "StrongBudgetBalance should be in extended environment"
        );
    }
    #[test]
    fn test_revelation_principle_verifier() {
        let type_values: Vec<f64> = (0..5).map(|i| (i as f64 + 1.0) / 5.0).collect();
        let n_types = type_values.len();
        let alloc: Vec<f64> = type_values
            .iter()
            .map(|&v| if v > 0.5 { 1.0 } else { 0.0 })
            .collect();
        let pay: Vec<f64> = type_values
            .iter()
            .map(|&v| if v > 0.5 { 0.5 } else { 0.0 })
            .collect();
        let verifier = RevelationPrincipleVerifier::new(n_types, alloc, pay);
        assert!(
            verifier.verify(&type_values),
            "Monotone mechanism should satisfy revelation principle"
        );
    }
    #[test]
    fn test_comb_auction_solver_exact() {
        let bids = vec![
            CombBid {
                bidder: 0,
                bundle: 0b01,
                value: 10.0,
            },
            CombBid {
                bidder: 1,
                bundle: 0b10,
                value: 8.0,
            },
            CombBid {
                bidder: 2,
                bundle: 0b11,
                value: 15.0,
            },
        ];
        let solver = CombAuctionSolver::new(bids, 2);
        let (winners, total) = solver.solve();
        assert!(
            (total - 18.0).abs() < 1e-10,
            "Optimal welfare should be 18.0, got {total}"
        );
        assert_eq!(winners.len(), 2, "Two winners for two disjoint bundles");
    }
    #[test]
    fn test_comb_auction_vcg_payments() {
        let bids = vec![
            CombBid {
                bidder: 0,
                bundle: 0b01,
                value: 10.0,
            },
            CombBid {
                bidder: 1,
                bundle: 0b01,
                value: 7.0,
            },
        ];
        let solver = CombAuctionSolver::new(bids, 1);
        let (winners, _) = solver.solve();
        let payments = solver.vcg_payments_for_winners(&winners);
        assert_eq!(winners.len(), 1);
        assert!(
            (payments[winners[0]] - 7.0).abs() < 1e-10,
            "VCG payment should be 7.0, got {}",
            payments[winners[0]]
        );
    }
    #[test]
    fn test_information_design_obedience() {
        let sender_payoff = vec![vec![0.0, 1.0], vec![0.0, 1.0]];
        let receiver_payoff = vec![vec![1.0, 0.0], vec![0.0, 1.0]];
        let prior = vec![0.5, 0.5];
        let solver = InformationDesignSolver::new(2, 2, sender_payoff, receiver_payoff, prior);
        let signal = solver
            .fully_revealing_signal()
            .expect("fully_revealing_signal should succeed");
        assert!(
            solver.check_obedience(&signal),
            "Fully revealing signal should be obedient"
        );
        let _ = solver.babbling_signal();
    }
    #[test]
    fn test_dynamic_vcg_mechanism() {
        let mut dvcg = DynamicVCGMechanism::new(3, 3);
        dvcg.run_period(&[5.0, 8.0, 3.0]);
        dvcg.run_period(&[10.0, 2.0, 7.0]);
        assert_eq!(dvcg.alloc_history.len(), 2);
        assert!(
            (dvcg.alloc_history[0][1] - 1.0).abs() < 1e-10,
            "Period 1: bidder 1 wins"
        );
        assert!(
            (dvcg.alloc_history[1][0] - 1.0).abs() < 1e-10,
            "Period 2: bidder 0 wins"
        );
        assert!(
            (dvcg.payment_history[0][1] - 5.0).abs() < 1e-10,
            "Period 1: winner pays 5"
        );
        assert!(
            (dvcg.payment_history[1][0] - 7.0).abs() < 1e-10,
            "Period 2: winner pays 7"
        );
        assert!(
            dvcg.is_budget_feasible(),
            "All payments should be non-negative"
        );
        let total_rev = dvcg.total_revenue();
        assert!(
            (total_rev - 12.0).abs() < 1e-10,
            "Total revenue should be 12.0, got {total_rev}"
        );
    }
    #[test]
    fn test_shapley_value_cost_sharing() {
        let cost_fn = |mask: usize| -> f64 {
            match mask {
                0 => 0.0,
                1 => 2.0,
                2 => 3.0,
                3 => 3.0,
                _ => 0.0,
            }
        };
        let sv = shapley_value_cost(2, cost_fn);
        assert!(
            (sv[0] - 1.0).abs() < 1e-10,
            "Shapley(0) should be 1.0, got {}",
            sv[0]
        );
        assert!(
            (sv[1] - 2.0).abs() < 1e-10,
            "Shapley(1) should be 2.0, got {}",
            sv[1]
        );
    }
    #[test]
    fn test_prophet_inequality() {
        let samples = vec![
            vec![1.0, 3.0],
            vec![2.0, 2.0],
            vec![3.0, 1.0],
            vec![0.0, 4.0],
        ];
        let prophet = prophet_expected_value(&samples);
        assert!(
            (prophet - 3.0).abs() < 1e-10,
            "Prophet value should be 3.0, got {prophet}"
        );
        let threshold_val = single_threshold_policy(&samples, 2.5);
        assert!(
            (threshold_val - 2.5).abs() < 1e-10,
            "Threshold 2.5 policy value should be 2.5, got {threshold_val}"
        );
        assert!(
            threshold_val >= prophet / 2.0 - 1e-10,
            "Threshold policy should achieve >= 1/2 of prophet"
        );
    }
    #[test]
    fn test_extended_axiom_types_wellformed() {
        let ty1 = adverse_selection_ty();
        assert!(matches!(ty1, Expr::Pi(_, _, _, _)));
        let ty2 = bayesian_persuasion_ty();
        assert!(matches!(ty2, Expr::Pi(_, _, _, _)));
        let ty3 = dynamic_revelation_principle_ty();
        assert!(matches!(ty3, Expr::Pi(_, _, _, _)));
        let ty4 = prophet_inequality_ty();
        assert!(matches!(ty4, Expr::Sort(_)));
        let ty5 = shapley_value_cost_sharing_ty();
        assert!(matches!(ty5, Expr::Pi(_, _, _, _)));
    }
}
#[cfg(test)]
mod tests_mechanism_design_ext {
    use super::*;
    #[test]
    fn test_myerson_mechanism() {
        let m = MyersonMechanism::new("Uniform[0,1]", 0.5);
        let psi = MyersonMechanism::virtual_value_uniform(0.8, 1.0);
        assert!((psi - 0.6).abs() < 1e-10);
        let reserve = MyersonMechanism::optimal_reserve_uniform(1.0);
        assert!((reserve - 0.5).abs() < 1e-10);
        assert!(m.revenue_equivalence().contains("Revenue Equivalence"));
    }
    #[test]
    fn test_myerson_allocation() {
        let m = MyersonMechanism::new("Uniform[0,1]", 0.5);
        let bids = vec![0.8, 0.4, 0.9];
        let winner = m.optimal_allocation(&bids, 1.0);
        assert_eq!(winner, Some(2));
    }
    #[test]
    fn test_vickrey_auction() {
        let auction = VickreyAuction::new(vec![100.0, 80.0, 60.0], 50.0);
        assert_eq!(auction.winner(), Some(0));
        assert!((auction.payment() - 80.0).abs() < 1e-10);
        assert!(auction.is_dsic());
        assert!((auction.expected_revenue() - 80.0).abs() < 1e-10);
    }
    #[test]
    fn test_vickrey_no_winner() {
        let auction = VickreyAuction::new(vec![30.0, 20.0], 50.0);
        assert!(auction.winner().is_none());
        assert!((auction.expected_revenue() - 0.0).abs() < 1e-10);
    }
    #[test]
    fn test_social_welfare_function() {
        let utils = vec![1.0, 2.0, 3.0];
        let u = SocialWelfareFunction::Utilitarian;
        assert!((u.evaluate(&utils) - 6.0).abs() < 1e-10);
        let r = SocialWelfareFunction::Rawlsian;
        assert!((r.evaluate(&utils) - 1.0).abs() < 1e-10);
        let n = SocialWelfareFunction::Nash;
        assert!((n.evaluate(&utils) - 6.0).abs() < 1e-5);
        assert!(u.arrow_impossibility_applies());
    }
}
#[cfg(test)]
mod tests_mechanism_design_ext2 {
    use super::*;
    #[test]
    fn test_bipartite_matching() {
        let mut bm = BipartiteMatching::new(
            vec!["a".to_string(), "b".to_string()],
            vec!["x".to_string(), "y".to_string()],
        );
        bm.add_edge(0, 0);
        bm.add_edge(0, 1);
        bm.add_edge(1, 1);
        bm.greedy_match();
        assert_eq!(bm.matching_size(), 2);
        assert!(bm.is_perfect_left());
        assert!(bm.halls_condition_description().contains("Hall"));
    }
    #[test]
    fn test_bipartite_no_perfect() {
        let mut bm = BipartiteMatching::new(
            vec!["a".to_string(), "b".to_string()],
            vec!["x".to_string()],
        );
        bm.add_edge(0, 0);
        bm.add_edge(1, 0);
        bm.greedy_match();
        assert_eq!(bm.matching_size(), 1);
        assert!(!bm.is_perfect_left());
    }
}
#[cfg(test)]
mod tests_mechanism_design_ext3 {
    use super::*;
    #[test]
    fn test_dsic_mechanism() {
        let m = DSICMechanism::new("Vickrey", true, true, "second-price");
        assert!(m.myersons_lemma_satisfied());
        assert!(m.individually_rational());
        assert!(m.description().contains("Vickrey"));
    }
}
