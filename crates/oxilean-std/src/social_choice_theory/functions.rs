//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)
#![allow(clippy::items_after_test_module)]

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{HedonicGame, LiquidDemocracy, PreferenceProfile, WeightedVotingGame};

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
/// `PreferenceProfile : Nat → Nat → Prop`
/// A preference profile for n voters over m alternatives.
pub fn preference_profile_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), prop()))
}
/// `SocialWelfareFunction : Nat → Nat → Prop`
/// A social welfare function mapping preference profiles to a social ranking.
pub fn social_welfare_function_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), prop()))
}
/// `SocialChoiceFunction : Nat → Nat → Prop`
/// A social choice function selecting a winner from a preference profile.
pub fn social_choice_function_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), prop()))
}
/// `IndependenceOfIrrelevantAlternatives : Prop → Prop`
/// IIA: the social ranking of x vs y depends only on individual rankings of x vs y.
pub fn iia_ty() -> Expr {
    arrow(prop(), prop())
}
/// `ParetoOptimality : Prop → Prop`
/// Pareto: if everyone prefers x to y, the social ordering ranks x above y.
pub fn pareto_optimality_ty() -> Expr {
    arrow(prop(), prop())
}
/// `NonDictatorship : Prop → Prop`
/// Non-dictatorship: no single voter's preferences always determine the social ordering.
pub fn non_dictatorship_ty() -> Expr {
    arrow(prop(), prop())
}
/// `CondorcetWinner : Nat → Nat → Prop`
/// An alternative that beats every other alternative in pairwise majority comparison.
pub fn condorcet_winner_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), prop()))
}
/// `StrategyProof : Prop → Prop`
/// A voting rule is strategyproof if no voter can benefit by misreporting preferences.
pub fn strategy_proof_ty() -> Expr {
    arrow(prop(), prop())
}
/// `Dictatorial : Prop → Prop`
/// A social choice function is dictatorial if it always selects the top choice of one voter.
pub fn dictatorial_ty() -> Expr {
    arrow(prop(), prop())
}
/// `MajorityRule : Nat → Prop`
/// May's theorem: majority rule is the unique SWF satisfying anonymity, neutrality,
/// decisiveness, and positive responsiveness for two alternatives.
pub fn majority_rule_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `ArrowImpossibility : ∀ (n m : Nat), n ≥ 2 → m ≥ 3 →
///   ¬ ∃ f : SWF, IIA f ∧ ParetoOptimal f ∧ NonDictatorial f`
/// Arrow's impossibility theorem: no SWF with ≥3 alternatives satisfies
/// IIA, Pareto, and non-dictatorship simultaneously.
pub fn arrow_impossibility_ty() -> Expr {
    arrow(
        nat_ty(),
        arrow(nat_ty(), arrow(prop(), arrow(prop(), prop()))),
    )
}
/// `GibbardSatterthwaite : ∀ (n m : Nat), m ≥ 3 →
///   StrategyProof SCF → Surjective SCF → Dictatorial SCF`
/// Every strategyproof, surjective SCF with ≥3 alternatives is dictatorial.
pub fn gibbard_satterthwaite_ty() -> Expr {
    arrow(
        nat_ty(),
        arrow(nat_ty(), arrow(prop(), arrow(prop(), prop()))),
    )
}
/// `MayTheorem : MajorityRule satisfies anonymity ∧ neutrality ∧ decisiveness ∧ positive_resp`
pub fn may_theorem_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `CondorcetParadox : transitivity can fail for majority preferences`
pub fn condorcet_paradox_ty() -> Expr {
    prop()
}
/// Build the social choice theory environment: register all axioms as opaque constants.
pub fn build_social_choice_env(env: &mut Environment) {
    let axioms: &[(&str, Expr)] = &[
        ("PreferenceProfile", preference_profile_ty()),
        ("SocialWelfareFunction", social_welfare_function_ty()),
        ("SocialChoiceFunction", social_choice_function_ty()),
        ("IndependenceOfIrrelevantAlternatives", iia_ty()),
        ("ParetoOptimality", pareto_optimality_ty()),
        ("NonDictatorship", non_dictatorship_ty()),
        ("CondorcetWinner", condorcet_winner_ty()),
        ("StrategyProof", strategy_proof_ty()),
        ("Dictatorial", dictatorial_ty()),
        ("MajorityRule", majority_rule_ty()),
        ("arrow_impossibility", arrow_impossibility_ty()),
        ("gibbard_satterthwaite", gibbard_satterthwaite_ty()),
        ("may_theorem", may_theorem_ty()),
        ("condorcet_paradox", condorcet_paradox_ty()),
        ("BordaCount", arrow(nat_ty(), arrow(nat_ty(), nat_ty()))),
        ("PluralityRule", arrow(list_ty(nat_ty()), nat_ty())),
        ("ApprovalVoting", arrow(list_ty(bool_ty()), nat_ty())),
        ("RangeVoting", arrow(list_ty(real_ty()), nat_ty())),
        ("InstantRunoff", arrow(list_ty(list_ty(nat_ty())), nat_ty())),
        (
            "LiquidDemocracy",
            arrow(nat_ty(), arrow(nat_ty(), nat_ty())),
        ),
        ("SchulzeMethod", arrow(list_ty(nat_ty()), nat_ty())),
        (
            "PairwiseMajority",
            arrow(
                nat_ty(),
                arrow(nat_ty(), arrow(list_ty(list_ty(nat_ty())), bool_ty())),
            ),
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
/// Find the Condorcet winner, if one exists.
///
/// An alternative is a Condorcet winner if it beats every other alternative
/// in pairwise majority comparison.
pub fn condorcet_winner(profile: &PreferenceProfile) -> Option<usize> {
    'outer: for alt in 0..profile.n_alts {
        for other in 0..profile.n_alts {
            if alt == other {
                continue;
            }
            if !profile.majority_beats(alt, other) {
                continue 'outer;
            }
        }
        return Some(alt);
    }
    None
}
/// Check whether the Condorcet paradox occurs: majority preferences form a cycle.
///
/// Returns the cycle (a → b → c → a) if one exists among the first three alternatives.
pub fn condorcet_cycle(profile: &PreferenceProfile) -> Option<(usize, usize, usize)> {
    let m = profile.n_alts;
    for a in 0..m {
        for b in 0..m {
            if b == a {
                continue;
            }
            if !profile.majority_beats(a, b) {
                continue;
            }
            for c in 0..m {
                if c == a || c == b {
                    continue;
                }
                if profile.majority_beats(b, c) && profile.majority_beats(c, a) {
                    return Some((a, b, c));
                }
            }
        }
    }
    None
}
/// Compute Borda count scores for all alternatives.
///
/// Each voter assigns score `(n_alts - 1 - rank)` to each alternative.
/// Returns a vector of (alternative, score) pairs sorted by descending score.
pub fn borda_count(profile: &PreferenceProfile) -> Vec<(usize, usize)> {
    let mut scores = vec![0usize; profile.n_alts];
    for voter in 0..profile.n_voters {
        for (rank, &alt) in profile.rankings[voter].iter().enumerate() {
            scores[alt] += profile.n_alts - 1 - rank;
        }
    }
    let mut result: Vec<(usize, usize)> = scores.into_iter().enumerate().collect();
    result.sort_by_key(|b| std::cmp::Reverse(b.1));
    result
}
/// Return the Borda count winner.
pub fn borda_winner(profile: &PreferenceProfile) -> usize {
    borda_count(profile)[0].0
}
/// Plurality voting: each voter votes for their top-ranked alternative.
/// Returns (winner, vote_counts).
pub fn plurality_vote(profile: &PreferenceProfile) -> (usize, Vec<usize>) {
    let mut counts = vec![0usize; profile.n_alts];
    for voter in 0..profile.n_voters {
        if !profile.rankings[voter].is_empty() {
            counts[profile.rankings[voter][0]] += 1;
        }
    }
    let winner = counts
        .iter()
        .enumerate()
        .max_by_key(|(_, &c)| c)
        .map(|(i, _)| i)
        .unwrap_or(0);
    (winner, counts)
}
/// Instant-runoff voting (IRV / alternative vote).
///
/// Repeatedly eliminates the alternative with the fewest first-choice votes
/// until one alternative has a majority.
pub fn instant_runoff(profile: &PreferenceProfile) -> usize {
    let mut active: Vec<bool> = vec![true; profile.n_alts];
    let mut n_active = profile.n_alts;
    loop {
        let mut counts = vec![0usize; profile.n_alts];
        for voter in 0..profile.n_voters {
            for &alt in &profile.rankings[voter] {
                if active[alt] {
                    counts[alt] += 1;
                    break;
                }
            }
        }
        let total_votes: usize = counts.iter().sum();
        for (alt, &count) in counts.iter().enumerate() {
            if active[alt] && count * 2 > total_votes {
                return alt;
            }
        }
        if n_active <= 1 {
            return active.iter().position(|&a| a).unwrap_or(0);
        }
        let min_votes = counts
            .iter()
            .enumerate()
            .filter(|(i, _)| active[*i])
            .map(|(_, &c)| c)
            .min()
            .unwrap_or(0);
        for (alt, &count) in counts.iter().enumerate() {
            if active[alt] && count == min_votes {
                active[alt] = false;
                n_active -= 1;
                break;
            }
        }
    }
}
/// Approval voting: each voter approves a subset of alternatives.
/// `approvals[voter][alt]` = true if voter approves alt.
/// Returns (winner, approval_counts).
pub fn approval_vote(approvals: &[Vec<bool>]) -> (usize, Vec<usize>) {
    let n_alts = if approvals.is_empty() {
        0
    } else {
        approvals[0].len()
    };
    let mut counts = vec![0usize; n_alts];
    for voter_approvals in approvals {
        for (alt, &approved) in voter_approvals.iter().enumerate() {
            if approved {
                counts[alt] += 1;
            }
        }
    }
    let winner = counts
        .iter()
        .enumerate()
        .fold((0usize, 0usize), |(best_i, best_c), (i, &c)| {
            if c > best_c {
                (i, c)
            } else {
                (best_i, best_c)
            }
        })
        .0;
    (winner, counts)
}
/// Range voting: each voter assigns a numeric score to each alternative.
/// `scores[voter][alt]` = score assigned by voter to alt.
/// Returns (winner, total_scores).
pub fn range_vote(scores: &[Vec<f64>]) -> (usize, Vec<f64>) {
    let n_alts = if scores.is_empty() {
        0
    } else {
        scores[0].len()
    };
    let mut totals = vec![0.0f64; n_alts];
    for voter_scores in scores {
        for (alt, &score) in voter_scores.iter().enumerate() {
            totals[alt] += score;
        }
    }
    let winner = totals
        .iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
        .map(|(i, _)| i)
        .unwrap_or(0);
    (winner, totals)
}
/// Check whether alternative `x` Pareto-dominates alternative `y`.
///
/// x Pareto-dominates y if every voter weakly prefers x to y AND
/// at least one voter strictly prefers x to y.
pub fn pareto_dominates(profile: &PreferenceProfile, x: usize, y: usize) -> bool {
    let all_weakly = (0..profile.n_voters).all(|v| !profile.prefers(v, y, x));
    let some_strictly = (0..profile.n_voters).any(|v| profile.prefers(v, x, y));
    all_weakly && some_strictly
}
/// Return the set of Pareto-optimal alternatives (not dominated by any other).
pub fn pareto_optimal_set(profile: &PreferenceProfile) -> Vec<usize> {
    (0..profile.n_alts)
        .filter(|&x| (0..profile.n_alts).all(|y| y == x || !pareto_dominates(profile, y, x)))
        .collect()
}
/// A Social Welfare Function represented as a function from preference profiles
/// to a complete ranking of alternatives.
pub type SocialRanking = Vec<usize>;
/// Compute the relative social ordering of two alternatives `x` and `y`
/// under Borda count, given the full profile. Returns true if x is ranked above y.
pub fn borda_ranks_above(profile: &PreferenceProfile, x: usize, y: usize) -> bool {
    let scores = borda_count(profile);
    let score_x = scores
        .iter()
        .find(|(a, _)| *a == x)
        .map(|(_, s)| *s)
        .unwrap_or(0);
    let score_y = scores
        .iter()
        .find(|(a, _)| *a == y)
        .map(|(_, s)| *s)
        .unwrap_or(0);
    score_x > score_y
}
/// Check if a pair of alternatives (x, y) violates IIA under Borda count,
/// given two preference profiles that agree on the relative ranking of x and y.
///
/// IIA is violated if: profiles agree on all pairwise rankings of {x, y} for every voter,
/// but the social ranking of x vs y differs between profiles.
pub fn check_iia_violation(
    profile1: &PreferenceProfile,
    profile2: &PreferenceProfile,
    x: usize,
    y: usize,
) -> bool {
    let agree = (0..profile1.n_voters.min(profile2.n_voters))
        .all(|v| profile1.prefers(v, x, y) == profile2.prefers(v, x, y));
    if !agree {
        return false;
    }
    borda_ranks_above(profile1, x, y) != borda_ranks_above(profile2, x, y)
}
/// Check if a voter can manipulate plurality voting by misreporting their preferences.
///
/// Returns `Some(alt)` if the voter can achieve outcome `alt` by deviating,
/// where `alt` is better for them than the honest outcome.
pub fn plurality_manipulation(profile: &PreferenceProfile, voter: usize) -> Option<usize> {
    let (honest_winner, _) = plurality_vote(profile);
    let voter_top = profile.rankings[voter][0];
    if honest_winner == voter_top {
        return None;
    }
    for &deceptive_top in &profile.rankings[voter] {
        if deceptive_top == profile.rankings[voter][0] {
            continue;
        }
        let mut modified_rankings = profile.rankings.clone();
        let mut new_ranking: Vec<usize> = vec![deceptive_top];
        new_ranking.extend(
            profile.rankings[voter]
                .iter()
                .filter(|&&a| a != deceptive_top),
        );
        modified_rankings[voter] = new_ranking;
        let modified_profile = PreferenceProfile::new(modified_rankings);
        let (new_winner, _) = plurality_vote(&modified_profile);
        if profile.prefers(voter, new_winner, honest_winner) {
            return Some(new_winner);
        }
    }
    None
}
/// Two-alternative majority vote: returns true if alternative 0 wins, false if alternative 1 wins.
/// Ties are broken in favor of alternative 0 (but May's theorem requires decisiveness; tie = None).
pub fn majority_vote_two(votes_for_0: usize, votes_for_1: usize) -> Option<usize> {
    use std::cmp::Ordering;
    match votes_for_0.cmp(&votes_for_1) {
        Ordering::Greater => Some(0),
        Ordering::Less => Some(1),
        Ordering::Equal => None,
    }
}
/// Compute the Schulze (beatpath) method winner.
///
/// Builds the pairwise strength matrix, then computes the strongest path
/// between all pairs using a Floyd-Warshall-style algorithm.
pub fn schulze_winner(profile: &PreferenceProfile) -> usize {
    let m = profile.n_alts;
    let mut d = vec![vec![0usize; m]; m];
    for i in 0..m {
        for j in 0..m {
            if i != j {
                d[i][j] = profile.majority_margin(i, j);
            }
        }
    }
    let mut p = vec![vec![0usize; m]; m];
    for i in 0..m {
        for j in 0..m {
            if i != j && d[i][j] > d[j][i] {
                p[i][j] = d[i][j];
            }
        }
    }
    for k in 0..m {
        for i in 0..m {
            for j in 0..m {
                if i != j && i != k && j != k {
                    p[i][j] = p[i][j].max(p[i][k].min(p[k][j]));
                }
            }
        }
    }
    let mut wins = vec![0usize; m];
    for i in 0..m {
        for j in 0..m {
            if i != j && p[i][j] > p[j][i] {
                wins[i] += 1;
            }
        }
    }
    wins.iter()
        .enumerate()
        .max_by_key(|(_, &w)| w)
        .map(|(i, _)| i)
        .unwrap_or(0)
}
/// `BrierScore : Nat → Real → Prop`
/// Brier proper scoring rule: the unique strictly proper scoring rule
/// that is local and bounded.
pub fn brier_score_ty() -> Expr {
    arrow(nat_ty(), arrow(real_ty(), prop()))
}
/// `SphericalScore : Nat → Real → Prop`
/// Spherical scoring rule: a proper scoring rule based on normalised probability vectors.
pub fn spherical_score_ty() -> Expr {
    arrow(nat_ty(), arrow(real_ty(), prop()))
}
/// `LogScore : Nat → Real → Prop`
/// Logarithmic scoring rule (log-loss): a proper scoring rule that rewards calibration.
pub fn log_score_ty() -> Expr {
    arrow(nat_ty(), arrow(real_ty(), prop()))
}
/// `ProperScoringRule : Prop → Prop`
/// A scoring rule is proper if the honest report maximises expected score.
pub fn proper_scoring_rule_ty() -> Expr {
    arrow(prop(), prop())
}
/// `StrictlyProperScoringRule : Prop → Prop`
/// A scoring rule is strictly proper if the unique maximiser is the honest report.
pub fn strictly_proper_scoring_rule_ty() -> Expr {
    arrow(prop(), prop())
}
/// `ScoringRuleCharacterization : ∀ (S : Prop), StrictlyProper S → Calibrated S`
/// Characterization theorem: strictly proper scoring rules incentivise calibrated beliefs.
pub fn scoring_rule_characterization_ty() -> Expr {
    arrow(prop(), arrow(prop(), prop()))
}
/// `JudgmentAggregation : Nat → Nat → Prop`
/// A judgment aggregation function: aggregates individual judgments on a set of propositions.
pub fn judgment_aggregation_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), prop()))
}
/// `DoctrinalParadox : Prop`
/// The doctrinal paradox: premise-based and conclusion-based aggregation can disagree.
pub fn doctrinal_paradox_ty() -> Expr {
    prop()
}
/// `JudgmentAggregationImpossibility : Prop`
/// Impossibility of consistent, complete, and independent judgment aggregation.
pub fn judgment_aggregation_impossibility_ty() -> Expr {
    prop()
}
/// `QuotaRule : Nat → Real → Prop`
/// A quota rule: proposition p is collectively accepted iff at least quota-fraction accept it.
pub fn quota_rule_ty() -> Expr {
    arrow(nat_ty(), arrow(real_ty(), prop()))
}
/// `PremiseBasedProcedure : Prop → Prop`
/// A premise-based aggregation procedure: aggregate on premises, derive conclusions logically.
pub fn premise_based_procedure_ty() -> Expr {
    arrow(prop(), prop())
}
/// `ConclusionBasedProcedure : Prop → Prop`
/// A conclusion-based aggregation procedure: aggregate directly on conclusions.
pub fn conclusion_based_procedure_ty() -> Expr {
    arrow(prop(), prop())
}
/// `AGMRevision : Prop → Prop → Prop`
/// AGM belief revision operator: satisfies the classical AGM rationality postulates.
pub fn agm_revision_ty() -> Expr {
    arrow(prop(), arrow(prop(), prop()))
}
/// `AGMContraction : Prop → Prop → Prop`
/// AGM belief contraction: remove a belief while satisfying recovery and other postulates.
pub fn agm_contraction_ty() -> Expr {
    arrow(prop(), arrow(prop(), prop()))
}
/// `BeliefMergeOperator : Nat → Prop → Prop`
/// A belief merge operator combining n belief bases into a single consistent base.
pub fn belief_merge_operator_ty() -> Expr {
    arrow(nat_ty(), arrow(prop(), prop()))
}
/// `MajorityMerge : Prop → Prop`
/// Majority-based belief merging: accept a formula iff a strict majority accepts it.
pub fn majority_merge_ty() -> Expr {
    arrow(prop(), prop())
}
/// `EnvyFreeAllocation : Nat → Nat → Prop`
/// An allocation is envy-free if no agent prefers another's bundle.
pub fn envy_free_allocation_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), prop()))
}
/// `EfficientAllocation : Nat → Nat → Prop`
/// An allocation is efficient (Pareto-optimal) in the housing market sense.
pub fn efficient_allocation_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), prop()))
}
/// `TopTradingCycles : Nat → Nat → Prop`
/// Top Trading Cycles algorithm: yields the unique core allocation in housing markets.
pub fn top_trading_cycles_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), prop()))
}
/// `CakeCutting : Nat → Prop`
/// Cake-cutting problem: divide a heterogeneous good among n agents.
pub fn cake_cutting_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `ProportionalAllocation : Nat → Prop`
/// Proportionality: each of n agents receives at least 1/n of the total value.
pub fn proportional_allocation_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `EnvyFreenessCakeCutting : Nat → Prop`
/// Envy-freeness for cake-cutting: no agent prefers another's piece.
pub fn envy_freeness_cake_cutting_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `DeliberativeDemocracy : Nat → Prop`
/// Deliberation model: agents update beliefs through structured discourse.
pub fn deliberative_democracy_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `ConsensusAggregation : Nat → Real → Prop`
/// Consensus rule: aggregate to the profile closest to all individual views.
pub fn consensus_aggregation_ty() -> Expr {
    arrow(nat_ty(), arrow(real_ty(), prop()))
}
/// `HegselmannKrauseModel : Nat → Real → Prop`
/// Hegselmann-Krause bounded-confidence opinion dynamics model.
pub fn hegselmann_krause_model_ty() -> Expr {
    arrow(nat_ty(), arrow(real_ty(), prop()))
}
/// `InfluencePropagation : Nat → Prop`
/// Influence propagation in voting networks: how votes spread through social ties.
pub fn influence_propagation_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `MedianRule : Nat → Prop`
/// Metric-based median rule: minimises sum of distances to individual positions.
pub fn median_rule_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `DistanceRationalizability : Prop → Prop`
/// Distance rationalizability: a rule is rationalised by a notion of consensus + distance.
pub fn distance_rationalizability_ty() -> Expr {
    arrow(prop(), prop())
}
/// `ProbabilisticSocialChoice : Nat → Nat → Prop`
/// A probabilistic social choice function: a lottery over alternatives.
pub fn probabilistic_social_choice_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), prop()))
}
/// `SSBUtility : Nat → Real → Prop`
/// Skew-symmetric bilinear (SSB) utility representation for probabilistic choice.
pub fn ssb_utility_ty() -> Expr {
    arrow(nat_ty(), arrow(real_ty(), prop()))
}
/// `LotteryDominance : Prop → Prop → Prop`
/// Stochastic dominance for lotteries over alternatives.
pub fn lottery_dominance_ty() -> Expr {
    arrow(prop(), arrow(prop(), prop()))
}
/// `SpatialVotingModel : Nat → Nat → Prop`
/// A spatial voting model with n voters in m-dimensional Euclidean space.
pub fn spatial_voting_model_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), prop()))
}
/// `MedianVoterTheorem : Nat → Prop`
/// Median voter theorem: the median position is a Condorcet winner in 1D spatial models.
pub fn median_voter_theorem_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `MultiDimensionalMedian : Nat → Nat → Prop`
/// Multi-dimensional median: generalisation of the median voter result.
pub fn multi_dimensional_median_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), prop()))
}
/// `HedonicGame : Nat → Prop`
/// A hedonic game: each agent has preferences over coalitions containing them.
pub fn hedonic_game_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `StablePartition : Nat → Prop`
/// A stable partition of agents into coalitions (no blocking coalition).
pub fn stable_partition_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `CoreCoalition : Nat → Prop`
/// The core of a coalition formation game: no group prefers to deviate.
pub fn core_coalition_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `NashStablePartition : Nat → Prop`
/// Nash stable partition: no agent prefers to move to another coalition.
pub fn nash_stable_partition_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `ShapleyShubikIndex : Nat → Real → Prop`
/// Shapley-Shubik power index for weighted voting games.
pub fn shapley_shubik_index_ty() -> Expr {
    arrow(nat_ty(), arrow(real_ty(), prop()))
}
/// `BanzhafIndex : Nat → Real → Prop`
/// Banzhaf power index: measures a voter's swing probability.
pub fn banzhaf_index_ty() -> Expr {
    arrow(nat_ty(), arrow(real_ty(), prop()))
}
/// `WeightedVotingGameTy : Nat → Nat → Prop`
/// A weighted voting game with n voters and quota q.
pub fn weighted_voting_game_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), prop()))
}
/// `CondorcetJuryTheorem : Nat → Real → Prop`
/// Condorcet jury theorem: majority accuracy increases with number of voters.
pub fn condorcet_jury_theorem_ty() -> Expr {
    arrow(nat_ty(), arrow(real_ty(), prop()))
}
/// `TruthTracking : Prop → Prop`
/// Truth tracking property: the aggregation procedure identifies the true state of affairs.
pub fn truth_tracking_ty() -> Expr {
    arrow(prop(), prop())
}
/// `EpistemicDemocracy : Nat → Prop`
/// Epistemic democracy: democracy is justified by its truth-tracking properties.
pub fn epistemic_democracy_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `TransitiveDelegation : Nat → Prop`
/// Transitive delegation: a delegate may further delegate their accumulated votes.
pub fn transitive_delegation_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `DelegationGraph : Nat → Prop`
/// A delegation graph encoding who delegates to whom in liquid democracy.
pub fn delegation_graph_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `GurulyCritique : Prop`
/// The Guruly-style paradox in liquid democracy: popular delegates can concentrate power.
pub fn guruly_critique_ty() -> Expr {
    prop()
}
/// `ManipulationComplexity : Prop → Prop`
/// NP-hardness of manipulation: it is NP-hard to compute a beneficial manipulation.
pub fn manipulation_complexity_ty() -> Expr {
    arrow(prop(), prop())
}
/// `BriberyComplexity : Nat → Prop`
/// Bribery problem: minimum cost to bribe a set of voters to change the winner.
pub fn bribery_complexity_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `ControlComplexity : Prop → Prop`
/// Complexity of election control: NP-hardness of adding/removing candidates or voters.
pub fn control_complexity_ty() -> Expr {
    arrow(prop(), prop())
}
/// Extend the social choice environment with advanced axioms.
pub fn build_social_choice_env_extended(env: &mut Environment) {
    let axioms: &[(&str, Expr)] = &[
        ("BrierScore", brier_score_ty()),
        ("SphericalScore", spherical_score_ty()),
        ("LogScore", log_score_ty()),
        ("ProperScoringRule", proper_scoring_rule_ty()),
        (
            "StrictlyProperScoringRule",
            strictly_proper_scoring_rule_ty(),
        ),
        (
            "scoring_rule_characterization",
            scoring_rule_characterization_ty(),
        ),
        ("JudgmentAggregation", judgment_aggregation_ty()),
        ("doctrinal_paradox", doctrinal_paradox_ty()),
        (
            "judgment_aggregation_impossibility",
            judgment_aggregation_impossibility_ty(),
        ),
        ("QuotaRule", quota_rule_ty()),
        ("PremiseBasedProcedure", premise_based_procedure_ty()),
        ("ConclusionBasedProcedure", conclusion_based_procedure_ty()),
        ("AGMRevision", agm_revision_ty()),
        ("AGMContraction", agm_contraction_ty()),
        ("BeliefMergeOperator", belief_merge_operator_ty()),
        ("MajorityMerge", majority_merge_ty()),
        ("EnvyFreeAllocation", envy_free_allocation_ty()),
        ("EfficientAllocation", efficient_allocation_ty()),
        ("TopTradingCycles", top_trading_cycles_ty()),
        ("CakeCutting", cake_cutting_ty()),
        ("ProportionalAllocation", proportional_allocation_ty()),
        ("EnvyFreenessCakeCutting", envy_freeness_cake_cutting_ty()),
        ("DeliberativeDemocracy", deliberative_democracy_ty()),
        ("ConsensusAggregation", consensus_aggregation_ty()),
        ("HegselmannKrauseModel", hegselmann_krause_model_ty()),
        ("InfluencePropagation", influence_propagation_ty()),
        ("MedianRule", median_rule_ty()),
        ("DistanceRationalizability", distance_rationalizability_ty()),
        (
            "ProbabilisticSocialChoice",
            probabilistic_social_choice_ty(),
        ),
        ("SSBUtility", ssb_utility_ty()),
        ("LotteryDominance", lottery_dominance_ty()),
        ("SpatialVotingModel", spatial_voting_model_ty()),
        ("median_voter_theorem", median_voter_theorem_ty()),
        ("MultiDimensionalMedian", multi_dimensional_median_ty()),
        ("HedonicGame", hedonic_game_ty()),
        ("StablePartition", stable_partition_ty()),
        ("CoreCoalition", core_coalition_ty()),
        ("NashStablePartition", nash_stable_partition_ty()),
        ("ShapleyShubikIndex", shapley_shubik_index_ty()),
        ("BanzhafIndex", banzhaf_index_ty()),
        ("WeightedVotingGame", weighted_voting_game_ty()),
        ("condorcet_jury_theorem", condorcet_jury_theorem_ty()),
        ("TruthTracking", truth_tracking_ty()),
        ("EpistemicDemocracy", epistemic_democracy_ty()),
        ("TransitiveDelegation", transitive_delegation_ty()),
        ("DelegationGraph", delegation_graph_ty()),
        ("guruly_critique", guruly_critique_ty()),
        ("ManipulationComplexity", manipulation_complexity_ty()),
        ("BriberyComplexity", bribery_complexity_ty()),
        ("ControlComplexity", control_complexity_ty()),
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
/// Compute n! as f64.
pub(super) fn factorial(n: usize) -> f64 {
    (1..=n).map(|i| i as f64).product()
}
/// Advance a permutation to the next lexicographic permutation in place.
/// Returns false if already at the last permutation.
pub(super) fn next_permutation(perm: &mut Vec<usize>) -> bool {
    let n = perm.len();
    if n < 2 {
        return false;
    }
    let mut i = n - 1;
    while i > 0 && perm[i - 1] >= perm[i] {
        i -= 1;
    }
    if i == 0 {
        return false;
    }
    let mut j = n - 1;
    while perm[j] <= perm[i - 1] {
        j -= 1;
    }
    perm.swap(i - 1, j);
    perm[i..].reverse();
    true
}
#[cfg(test)]
mod tests {
    use super::*;
    fn three_voter_profile() -> PreferenceProfile {
        PreferenceProfile::new(vec![vec![0, 1, 2], vec![1, 2, 0], vec![2, 0, 1]])
    }
    fn clear_winner_profile() -> PreferenceProfile {
        PreferenceProfile::new(vec![vec![0, 1, 2], vec![0, 2, 1], vec![1, 0, 2]])
    }
    #[test]
    fn test_condorcet_paradox_detection() {
        let profile = three_voter_profile();
        assert!(profile.majority_beats(0, 1), "A should beat B");
        assert!(profile.majority_beats(1, 2), "B should beat C");
        assert!(profile.majority_beats(2, 0), "C should beat A");
        assert_eq!(
            condorcet_winner(&profile),
            None,
            "No Condorcet winner in paradox profile"
        );
        assert!(
            condorcet_cycle(&profile).is_some(),
            "Condorcet cycle should be detected"
        );
    }
    #[test]
    fn test_condorcet_winner() {
        let profile = clear_winner_profile();
        assert_eq!(
            condorcet_winner(&profile),
            Some(0),
            "Alternative 0 should be Condorcet winner"
        );
    }
    #[test]
    fn test_borda_count() {
        let profile = clear_winner_profile();
        let scores = borda_count(&profile);
        assert_eq!(scores[0].0, 0, "Alt 0 should be Borda winner");
        assert_eq!(scores[0].1, 5, "Alt 0 Borda score should be 5");
        assert_eq!(borda_winner(&profile), 0);
    }
    #[test]
    fn test_plurality_vote() {
        let profile = clear_winner_profile();
        let (winner, counts) = plurality_vote(&profile);
        assert_eq!(winner, 0, "Alt 0 wins plurality with 2 votes");
        assert_eq!(counts[0], 2);
        assert_eq!(counts[1], 1);
        assert_eq!(counts[2], 0);
    }
    #[test]
    fn test_instant_runoff() {
        let profile = PreferenceProfile::new(vec![
            vec![0, 1, 2],
            vec![0, 1, 2],
            vec![1, 2, 0],
            vec![1, 2, 0],
            vec![2, 0, 1],
        ]);
        let winner = instant_runoff(&profile);
        assert_eq!(winner, 0, "IRV winner should be alternative 0 (A)");
    }
    #[test]
    fn test_approval_voting() {
        let approvals = vec![
            vec![true, true, false],
            vec![false, true, true],
            vec![true, false, true],
        ];
        let (winner, counts) = approval_vote(&approvals);
        assert_eq!(counts[0], 2);
        assert_eq!(counts[1], 2);
        assert_eq!(counts[2], 2);
        assert_eq!(winner, 0);
    }
    #[test]
    fn test_pareto_optimality() {
        let profile = PreferenceProfile::new(vec![vec![0, 1, 2], vec![0, 1, 2]]);
        assert!(pareto_dominates(&profile, 0, 1), "A Pareto-dominates B");
        assert!(pareto_dominates(&profile, 0, 2), "A Pareto-dominates C");
        assert!(pareto_dominates(&profile, 1, 2), "B Pareto-dominates C");
        assert!(
            !pareto_dominates(&profile, 1, 0),
            "B does not Pareto-dominate A"
        );
        let opt_set = pareto_optimal_set(&profile);
        assert_eq!(opt_set, vec![0], "Only A is Pareto-optimal");
    }
    #[test]
    fn test_liquid_democracy() {
        let delegation = vec![None, None, None, Some(0), Some(2)];
        let direct_vote = vec![Some(0), Some(1), Some(0), None, None];
        let ld = LiquidDemocracy::new(5, delegation, direct_vote);
        assert_eq!(ld.effective_vote(0), Some(0));
        assert_eq!(ld.effective_vote(3), Some(0));
        assert_eq!(ld.effective_vote(4), Some(0));
        let totals = ld.vote_totals(2);
        assert_eq!(totals[0], 4, "Alt 0 should receive 4 effective votes");
        assert_eq!(totals[1], 1, "Alt 1 should receive 1 effective vote");
        assert_eq!(ld.winner(2), Some(0));
        assert!(ld.find_cycle().is_none(), "No delegation cycles");
    }
    #[test]
    fn test_liquid_democracy_cycle() {
        let delegation = vec![Some(1), Some(2), Some(0)];
        let direct_vote = vec![None, None, None];
        let ld = LiquidDemocracy::new(3, delegation, direct_vote);
        assert!(ld.find_cycle().is_some(), "Cycle should be detected");
        assert_eq!(ld.effective_vote(0), None);
        assert_eq!(ld.effective_vote(1), None);
    }
}
/// `ArrowImpossibilityDetailed : ∀ (n m : Nat), n ≥ 2 → m ≥ 3 →
///   ∀ (f : SWF n m), IIA f → ParetoOptimal f → Dictatorial f`
/// Arrow's impossibility: any SWF satisfying IIA and Pareto must be dictatorial.
pub fn sct_ext_arrow_impossibility_detailed_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "m",
            nat_ty(),
            arrow(
                app2(cst("NatGe"), bvar(1), cst("Two")),
                arrow(
                    app2(cst("NatGe"), bvar(1), cst("Three")),
                    pi(
                        BinderInfo::Default,
                        "f",
                        app2(cst("SocialWelfareFunction"), bvar(3), bvar(2)),
                        arrow(
                            app(cst("IIA"), bvar(0)),
                            arrow(
                                app(cst("ParetoOptimal"), bvar(1)),
                                app(cst("Dictatorial"), bvar(2)),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// `DecisiveSet : Nat → Nat → Prop`
/// A decisive set for a SWF: a coalition whose unanimous preference is always adopted.
pub fn sct_ext_decisive_set_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), prop()))
}
/// `PivotLemma : ∀ (f : SWF), IIA f → ParetoOptimal f → ∃ (i : Voter), IsDecisive i f`
/// The pivot lemma in Arrow's proof: there exists a decisive voter.
pub fn sct_ext_pivot_lemma_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "f",
        cst("SocialWelfareFunction"),
        arrow(
            app(cst("IIA"), bvar(0)),
            arrow(
                app(cst("ParetoOptimal"), bvar(1)),
                app2(cst("ExistsDecisive"), bvar(2), bvar(2)),
            ),
        ),
    )
}
/// `FieldExpansionLemma : Prop`
/// The field expansion lemma: a decisive set for one pair is decisive for all pairs.
pub fn sct_ext_field_expansion_lemma_ty() -> Expr {
    prop()
}
/// `SenImpossibility : ∀ (n : Nat), n ≥ 2 →
///   ¬ ∃ f : SCF, ParetoOptimal f ∧ MinimalLiberalism f`
/// Sen's impossibility theorem: Pareto optimality and minimal liberalism are incompatible.
pub fn sct_ext_sen_impossibility_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        arrow(app(cst("NatGeTwo"), bvar(0)), arrow(prop(), prop())),
    )
}
/// `MinimalLiberalism : SCF → Prop`
/// Sen's minimal liberalism condition: there exist at least two voters each with one
/// pair of alternatives over which they are decisive.
pub fn sct_ext_minimal_liberalism_ty() -> Expr {
    arrow(cst("SocialChoiceFunction"), prop())
}
/// `LiberalParadox : Prop`
/// The liberal paradox (Sen, 1970): a Pareto-respecting and liberal SCF may not exist.
pub fn sct_ext_liberal_paradox_ty() -> Expr {
    prop()
}
/// `GibbardSatterthwaiteDetailed : ∀ (n m : Nat), m ≥ 3 →
///   ∀ f : SCF, Surjective f → StrategyProof f → Dictatorial f`
pub fn sct_ext_gibbard_satterthwaite_detailed_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "m",
            nat_ty(),
            arrow(
                app(cst("NatGeThree"), bvar(0)),
                pi(
                    BinderInfo::Default,
                    "f",
                    app2(cst("SocialChoiceFunction"), bvar(2), bvar(1)),
                    arrow(
                        app(cst("IsSurjective"), bvar(0)),
                        arrow(
                            app(cst("IsStrategyProof"), bvar(1)),
                            app(cst("Dictatorial"), bvar(2)),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// `ManipulableVotingRule : SCF → Voter → Prop`
/// A voting rule is manipulable if some voter benefits from misreporting.
pub fn sct_ext_manipulable_voting_rule_ty() -> Expr {
    arrow(cst("SocialChoiceFunction"), arrow(nat_ty(), prop()))
}
/// `GibbardRandomizedThm : ∀ (f : PSCF), StrategyProof f →
///   f is a mixture of dictatorships and duos`
/// Gibbard's theorem for randomized social choice functions.
pub fn sct_ext_gibbard_randomized_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "f",
        cst("ProbabilisticSocialChoice"),
        arrow(
            app(cst("IsStrategyProof"), bvar(0)),
            app(cst("IsMixtureOfDictatorsAndDuos"), bvar(1)),
        ),
    )
}
/// `BlacksTheoremSinglePeak : ∀ (n : Nat), SinglePeaked preferences →
///   ∃ (x : Alt), IsCondorcetWinner x`
/// Black's theorem: if all preferences are single-peaked, a Condorcet winner exists.
pub fn sct_ext_blacks_theorem_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        arrow(
            app(cst("AllSinglePeaked"), bvar(0)),
            app2(cst("ExistsCondorcetWinner"), bvar(1), bvar(1)),
        ),
    )
}
/// `SinglePeakedPreference : Voter → Nat → Prop`
/// A voter has single-peaked preferences over a linearly ordered alternative set.
pub fn sct_ext_single_peaked_preference_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), prop()))
}
/// `MedianVoterEquilibrium : Nat → Prop`
/// In a 1D spatial model with single-peaked preferences, the median voter's position
/// is the unique Condorcet winner (Downs median voter theorem).
pub fn sct_ext_median_voter_equilibrium_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `DownsMedianVoterThm : ∀ (n : Nat), IsOdd n →
///   MedianPosition n = CondorcetWinner n`
/// Downs' median voter theorem: the median voter's ideal point is the Condorcet winner.
pub fn sct_ext_downs_median_voter_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        arrow(
            app(cst("IsOdd"), bvar(0)),
            app2(
                cst("Eq"),
                app(cst("MedianPosition"), bvar(1)),
                app(cst("CondorcetWinnerPos"), bvar(2)),
            ),
        ),
    )
}
/// `CondorcetConsistency : SCF → Prop`
/// Condorcet consistency: if a Condorcet winner exists, the rule selects it.
pub fn sct_ext_condorcet_consistency_ty() -> Expr {
    arrow(cst("SocialChoiceFunction"), prop())
}
/// `CondorcetLoser : Nat → Nat → Prop`
/// A Condorcet loser is beaten by every other alternative in pairwise majority.
pub fn sct_ext_condorcet_loser_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), prop()))
}
/// `CondorcetEfficiency : SCF → Real → Prop`
/// Condorcet efficiency of a rule: probability that the rule selects the Condorcet winner
/// given random IC preferences.
pub fn sct_ext_condorcet_efficiency_ty() -> Expr {
    arrow(cst("SocialChoiceFunction"), arrow(real_ty(), prop()))
}
/// `SmithSet : Nat → Nat → Prop`
/// The Smith set (top cycle): the smallest non-empty set that beats all alternatives outside.
pub fn sct_ext_smith_set_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), prop()))
}
/// `SchwartzSet : Nat → Nat → Prop`
/// The Schwartz set: the union of all minimal undominated sets.
pub fn sct_ext_schwartz_set_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), prop()))
}
/// `SmithSetSubsetsCondorcetWinner : ∀ (n m : Nat),
///   CondorcetWinnerExists n m → SmithSet n m = {CondorcetWinner}`
pub fn sct_ext_smith_set_singleton_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "m",
            nat_ty(),
            arrow(
                app2(cst("CondorcetWinnerExists"), bvar(1), bvar(0)),
                app2(cst("SmithSetIsSingleton"), bvar(2), bvar(1)),
            ),
        ),
    )
}
/// `KemenyYouleRule : Nat → Nat → Nat`
/// The Kemeny-Young rule: finds the ranking that minimizes the sum of pairwise distances.
pub fn sct_ext_kemeny_young_rule_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), nat_ty()))
}
/// `KemenyScore : Nat → Nat → Nat`
/// Kemeny score of a ranking: sum over all pairs of how many voters agree with the ranking.
pub fn sct_ext_kemeny_score_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), nat_ty()))
}
/// `KemenyRuleIsCondorcetConsistent : ∀ (f : KemenyRule), CondorcetConsistent f`
pub fn sct_ext_kemeny_condorcet_consistency_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "f",
        cst("KemenyRule"),
        app(cst("IsCondorcetConsistent"), bvar(0)),
    )
}
/// `KemenyRuleNPHard : Prop`
/// Computing the Kemeny-Young winner is NP-hard (even for 4 candidates).
pub fn sct_ext_kemeny_np_hard_ty() -> Expr {
    prop()
}
/// `BordaCountRule : Nat → Nat → Nat`
/// The Borda count rule assigns score (m-1-rank) to each alternative.
pub fn sct_ext_borda_count_rule_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), nat_ty()))
}
/// `BordaCondorcetLoserElimination : ∀ (profile : Profile),
///   CondorcetLoserExists profile → BordaWinner profile ≠ CondorcetLoser profile`
/// Borda count never elects a Condorcet loser.
pub fn sct_ext_borda_condorcet_loser_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "profile",
        cst("PreferenceProfile"),
        arrow(
            app(cst("CondorcetLoserEx"), bvar(0)),
            arrow(
                cst("True"),
                app2(
                    cst("Neq"),
                    app(cst("BordaWinner"), bvar(2)),
                    app(cst("CondorcetLoserVal"), bvar(2)),
                ),
            ),
        ),
    )
}
/// `BordaAnonymityNeutralityConsistency : Prop`
/// Borda count satisfies anonymity, neutrality, and consistency (reinforcement).
pub fn sct_ext_borda_properties_ty() -> Expr {
    prop()
}
/// `BordaRulePosResp : ∀ (profile : Profile) (alt : Nat),
///   BordaWinner profile = alt → MoreFirstPlaceVotes profile alt → BordaWinner profile = alt`
/// Positive responsiveness of Borda count.
pub fn sct_ext_borda_positive_resp_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "profile",
        cst("PreferenceProfile"),
        pi(
            BinderInfo::Default,
            "alt",
            nat_ty(),
            arrow(
                app2(cst("BordaWinnerIs"), bvar(1), bvar(0)),
                arrow(
                    app2(cst("MoreFirstPlaceVotes"), bvar(2), bvar(1)),
                    app2(cst("BordaWinnerIs"), bvar(3), bvar(2)),
                ),
            ),
        ),
    )
}
/// `DHondtMethod : Nat → Nat → Nat`
/// D'Hondt method: seat allocation rule based on successive division by seat number.
pub fn sct_ext_dhondt_method_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), nat_ty()))
}
/// `SainteLagueMethod : Nat → Nat → Nat`
/// Sainte-Laguë method: seat allocation using odd-number divisors.
pub fn sct_ext_sainte_lague_method_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), nat_ty()))
}
/// `HareQuota : Nat → Nat → Nat`
/// The Hare quota for proportional representation: floor(total_votes / n_seats).
pub fn sct_ext_hare_quota_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), nat_ty()))
}
/// `Droop Quota : Nat → Nat → Nat`
/// The Droop quota: floor(total_votes / (n_seats + 1)) + 1.
pub fn sct_ext_droop_quota_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), nat_ty()))
}
/// `ProportionalRepresentationAnonymity : Prop`
/// Proportional representation satisfies anonymity.
pub fn sct_ext_pr_anonymity_ty() -> Expr {
    prop()
}
/// `LargestRemainder : Nat → Nat → Nat`
/// Largest remainder method (Hamilton method) for seat allocation.
pub fn sct_ext_largest_remainder_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), nat_ty()))
}
/// `UtilitarianSWF : Nat → Real → Prop`
/// Utilitarian social welfare function: maximises sum of individual utilities.
pub fn sct_ext_utilitarian_swf_ty() -> Expr {
    arrow(nat_ty(), arrow(real_ty(), prop()))
}
/// `RawlsianSWF : Nat → Real → Prop`
/// Rawlsian (maximin) social welfare function: maximises the worst-off individual's utility.
pub fn sct_ext_rawlsian_swf_ty() -> Expr {
    arrow(nat_ty(), arrow(real_ty(), prop()))
}
/// `NashSWF : Nat → Real → Prop`
/// Nash social welfare function: maximises the product of individual utilities.
pub fn sct_ext_nash_swf_ty() -> Expr {
    arrow(nat_ty(), arrow(real_ty(), prop()))
}
/// `HarsunyiUtilitarianTheorem : ∀ (n : Nat), VNMUtility n → UtilitarianSWF n`
/// Harsanyi's utilitarian theorem: any SWF satisfying Pareto and VNM axioms is utilitarian.
pub fn sct_ext_harsanyi_utilitarian_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        arrow(
            app(cst("HasVNMUtility"), bvar(0)),
            app(cst("IsUtilitarian"), bvar(1)),
        ),
    )
}
/// `EgalitarianPrinciple : ∀ (n : Nat), Rawlsian n → MaximinsUtilityOf1 n`
/// The egalitarian principle: Rawlsian SWF is the limiting case of prioritarianism.
pub fn sct_ext_egalitarian_principle_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        arrow(
            app(cst("IsRawlsian"), bvar(0)),
            app(cst("MaximisesMinUtility"), bvar(1)),
        ),
    )
}
/// `CakeCuttingProtocol : Nat → Prop`
/// A cake-cutting protocol for n agents producing a fair division.
pub fn sct_ext_cake_cutting_protocol_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `MovingKnifeProtocol : Nat → Prop`
/// A moving-knife protocol achieving envy-free cake cutting.
pub fn sct_ext_moving_knife_protocol_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `SutherlandConway : Nat → Prop`
/// Sutherland-Conway protocol: an envy-free cake-cutting algorithm for 3 agents.
pub fn sct_ext_sutherland_conway_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `BramsTaylorProtocol : Nat → Prop`
/// Brams-Taylor unbounded envy-free protocol for n agents.
pub fn sct_ext_brams_taylor_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `AzizMackenzieProtocol : Nat → Prop`
/// Aziz-Mackenzie bounded envy-free protocol (2016): O(n^{n^{n^{n^{n^n}}}}) queries.
pub fn sct_ext_aziz_mackenzie_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `EnvyFreeCakeCuttingExists : ∀ (n : Nat), ∃ (P : Protocol), EnvyFree P n`
/// Existence of envy-free cake cutting for any number of agents.
pub fn sct_ext_envy_free_existence_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        app2(
            cst("Exists"),
            cst("Protocol"),
            app(cst("IsEnvyFree"), bvar(0)),
        ),
    )
}
/// `CopelandMethod : Nat → Nat → Nat`
/// Copeland's method: score = wins - losses in pairwise majority comparisons.
pub fn sct_ext_copeland_method_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), nat_ty()))
}
/// `CopelandIsCondorcetConsistent : ∀ (f : CopelandRule), CondorcetConsistent f`
pub fn sct_ext_copeland_condorcet_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "f",
        cst("CopelandRule"),
        app(cst("IsCondorcetConsistent"), bvar(0)),
    )
}
/// `CopelandSatisfiesSmithCriterion : Prop`
/// Copeland's method always selects from the Smith set.
pub fn sct_ext_copeland_smith_criterion_ty() -> Expr {
    prop()
}
/// `ApprovalVotingAxioms : Prop`
/// Approval voting satisfies: strategy-proofness in sincere voting, Pareto, neutrality.
pub fn sct_ext_approval_voting_axioms_ty() -> Expr {
    prop()
}
/// `ApprovalVotingNeutrality : ∀ (f : ApprovalRule), IsNeutral f`
pub fn sct_ext_approval_voting_neutrality_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "f",
        cst("ApprovalRule"),
        app(cst("IsNeutral"), bvar(0)),
    )
}
/// `ApprovalVotingStrategyProof : ∀ (f : ApprovalRule), IsStrategyProofUnderSincereVoting f`
/// Approval voting is strategy-proof under the sincere voting assumption.
pub fn sct_ext_approval_strategy_proof_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "f",
        cst("ApprovalRule"),
        app(cst("IsStrategyProofSincere"), bvar(0)),
    )
}
/// `IRVAxioms : Prop`
/// Axioms characterizing instant-runoff voting (IRV): majority, anonymity, neutrality.
pub fn sct_ext_irv_axioms_ty() -> Expr {
    prop()
}
/// `IRVViolatesIIA : ∀ (f : IRVRule), ¬ IIA f`
/// IRV violates the independence of irrelevant alternatives.
pub fn sct_ext_irv_violates_iia_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "f",
        cst("IRVRule"),
        arrow(app(cst("IIA"), bvar(0)), cst("False")),
    )
}
/// `IRVSatisfiesMajorityCriterion : ∀ (f : IRVRule), MajorityCriterion f`
/// IRV satisfies the majority criterion: a candidate with a majority of first-choice votes wins.
pub fn sct_ext_irv_majority_criterion_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "f",
        cst("IRVRule"),
        app(cst("SatisfiesMajorityCriterion"), bvar(0)),
    )
}
/// Register all extended social choice theory axioms.
pub fn register_social_choice_extended(env: &mut Environment) -> Result<(), String> {
    let axioms: &[(&str, Expr)] = &[
        (
            "arrow_impossibility_detailed",
            sct_ext_arrow_impossibility_detailed_ty(),
        ),
        ("DecisiveSet", sct_ext_decisive_set_ty()),
        ("pivot_lemma", sct_ext_pivot_lemma_ty()),
        ("field_expansion_lemma", sct_ext_field_expansion_lemma_ty()),
        ("sen_impossibility", sct_ext_sen_impossibility_ty()),
        ("MinimalLiberalism", sct_ext_minimal_liberalism_ty()),
        ("liberal_paradox", sct_ext_liberal_paradox_ty()),
        (
            "gibbard_satterthwaite_detailed",
            sct_ext_gibbard_satterthwaite_detailed_ty(),
        ),
        (
            "ManipulableVotingRule",
            sct_ext_manipulable_voting_rule_ty(),
        ),
        ("gibbard_randomized", sct_ext_gibbard_randomized_ty()),
        ("blacks_theorem", sct_ext_blacks_theorem_ty()),
        (
            "SinglePeakedPreference",
            sct_ext_single_peaked_preference_ty(),
        ),
        (
            "MedianVoterEquilibrium",
            sct_ext_median_voter_equilibrium_ty(),
        ),
        ("downs_median_voter", sct_ext_downs_median_voter_ty()),
        ("CondorcetConsistency", sct_ext_condorcet_consistency_ty()),
        ("CondorcetLoser", sct_ext_condorcet_loser_ty()),
        ("CondorcetEfficiency", sct_ext_condorcet_efficiency_ty()),
        ("SmithSet", sct_ext_smith_set_ty()),
        ("SchwartzSet", sct_ext_schwartz_set_ty()),
        ("smith_set_singleton", sct_ext_smith_set_singleton_ty()),
        ("KemenyYoungRule", sct_ext_kemeny_young_rule_ty()),
        ("KemenyScore", sct_ext_kemeny_score_ty()),
        (
            "kemeny_condorcet_consistency",
            sct_ext_kemeny_condorcet_consistency_ty(),
        ),
        ("kemeny_np_hard", sct_ext_kemeny_np_hard_ty()),
        ("BordaCountRule", sct_ext_borda_count_rule_ty()),
        ("borda_condorcet_loser", sct_ext_borda_condorcet_loser_ty()),
        ("borda_properties", sct_ext_borda_properties_ty()),
        ("borda_positive_resp", sct_ext_borda_positive_resp_ty()),
        ("DHondtMethod", sct_ext_dhondt_method_ty()),
        ("SainteLagueMethod", sct_ext_sainte_lague_method_ty()),
        ("HareQuota", sct_ext_hare_quota_ty()),
        ("DroopQuota", sct_ext_droop_quota_ty()),
        (
            "ProportionalRepresentationAnonymity",
            sct_ext_pr_anonymity_ty(),
        ),
        ("LargestRemainder", sct_ext_largest_remainder_ty()),
        ("UtilitarianSWF", sct_ext_utilitarian_swf_ty()),
        ("RawlsianSWF", sct_ext_rawlsian_swf_ty()),
        ("NashSWF", sct_ext_nash_swf_ty()),
        ("harsanyi_utilitarian", sct_ext_harsanyi_utilitarian_ty()),
        ("egalitarian_principle", sct_ext_egalitarian_principle_ty()),
        ("CakeCuttingProtocol", sct_ext_cake_cutting_protocol_ty()),
        ("MovingKnifeProtocol", sct_ext_moving_knife_protocol_ty()),
        ("SutherlandConway", sct_ext_sutherland_conway_ty()),
        ("BramsTaylor", sct_ext_brams_taylor_ty()),
        ("AzizMackenzie", sct_ext_aziz_mackenzie_ty()),
        ("envy_free_existence", sct_ext_envy_free_existence_ty()),
        ("CopelandMethod", sct_ext_copeland_method_ty()),
        ("copeland_condorcet", sct_ext_copeland_condorcet_ty()),
        (
            "copeland_smith_criterion",
            sct_ext_copeland_smith_criterion_ty(),
        ),
        (
            "approval_voting_axioms",
            sct_ext_approval_voting_axioms_ty(),
        ),
        (
            "approval_voting_neutrality",
            sct_ext_approval_voting_neutrality_ty(),
        ),
        (
            "approval_strategy_proof",
            sct_ext_approval_strategy_proof_ty(),
        ),
        ("irv_axioms", sct_ext_irv_axioms_ty()),
        ("irv_violates_iia", sct_ext_irv_violates_iia_ty()),
        (
            "irv_majority_criterion",
            sct_ext_irv_majority_criterion_ty(),
        ),
        ("NatGe", arrow(nat_ty(), arrow(nat_ty(), prop()))),
        ("NatGeTwo", arrow(nat_ty(), prop())),
        ("NatGeThree", arrow(nat_ty(), prop())),
        ("Two", nat_ty()),
        ("Three", nat_ty()),
        ("IIA", arrow(cst("SocialWelfareFunction"), prop())),
        ("ParetoOptimal", arrow(cst("SocialWelfareFunction"), prop())),
        ("Dictatorial", arrow(cst("SocialWelfareFunction"), prop())),
        ("IsSurjective", arrow(cst("SocialChoiceFunction"), prop())),
        (
            "IsStrategyProof",
            arrow(cst("SocialChoiceFunction"), prop()),
        ),
        (
            "ExistsDecisive",
            arrow(
                cst("SocialWelfareFunction"),
                arrow(cst("SocialWelfareFunction"), prop()),
            ),
        ),
        ("AllSinglePeaked", arrow(nat_ty(), prop())),
        (
            "ExistsCondorcetWinner",
            arrow(nat_ty(), arrow(nat_ty(), prop())),
        ),
        ("IsOdd", arrow(nat_ty(), prop())),
        ("MedianPosition", arrow(nat_ty(), nat_ty())),
        ("CondorcetWinnerPos", arrow(nat_ty(), nat_ty())),
        ("Eq", arrow(nat_ty(), arrow(nat_ty(), prop()))),
        (
            "CondorcetWinnerExists",
            arrow(nat_ty(), arrow(nat_ty(), prop())),
        ),
        (
            "SmithSetIsSingleton",
            arrow(nat_ty(), arrow(nat_ty(), prop())),
        ),
        ("KemenyRule", type0()),
        ("IsCondorcetConsistent", arrow(type0(), prop())),
        ("CondorcetLoserEx", arrow(cst("PreferenceProfile"), prop())),
        ("BordaWinner", arrow(cst("PreferenceProfile"), nat_ty())),
        (
            "CondorcetLoserVal",
            arrow(cst("PreferenceProfile"), nat_ty()),
        ),
        ("Neq", arrow(nat_ty(), arrow(nat_ty(), prop()))),
        (
            "BordaWinnerIs",
            arrow(cst("PreferenceProfile"), arrow(nat_ty(), prop())),
        ),
        (
            "MoreFirstPlaceVotes",
            arrow(cst("PreferenceProfile"), arrow(nat_ty(), prop())),
        ),
        ("HasVNMUtility", arrow(nat_ty(), prop())),
        ("IsUtilitarian", arrow(nat_ty(), prop())),
        ("IsRawlsian", arrow(nat_ty(), prop())),
        ("MaximisesMinUtility", arrow(nat_ty(), prop())),
        ("Protocol", type0()),
        ("IsEnvyFree", arrow(type0(), prop())),
        ("CopelandRule", type0()),
        ("ApprovalRule", type0()),
        ("IsNeutral", arrow(type0(), prop())),
        ("IsStrategyProofSincere", arrow(type0(), prop())),
        ("IRVRule", type0()),
        ("False", prop()),
        ("SatisfiesMajorityCriterion", arrow(type0(), prop())),
        ("ProbabilisticSocialChoiceFunc", type0()),
        ("IsMixtureOfDictatorsAndDuos", arrow(type0(), prop())),
        ("True", prop()),
    ];
    for (name, ty) in axioms {
        env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty: ty.clone(),
        })
        .ok();
    }
    Ok(())
}
/// Compute the Kemeny-Young score of a ranking (given as a permutation of alternatives).
///
/// Score = number of pairwise agreements between the ranking and the preference profile.
pub fn kemeny_score(profile: &PreferenceProfile, ranking: &[usize]) -> usize {
    let mut score = 0usize;
    let m = ranking.len();
    for i in 0..m {
        for j in (i + 1)..m {
            let a = ranking[i];
            let b = ranking[j];
            score += profile.majority_margin(a, b);
        }
    }
    score
}
/// Find the Kemeny-Young winner by brute-force over all permutations of alternatives.
///
/// WARNING: exponential in the number of alternatives. Use only for small instances.
pub fn kemeny_young_winner(profile: &PreferenceProfile) -> usize {
    let m = profile.n_alts;
    assert!(m <= 8, "Kemeny-Young brute force limited to 8 alternatives");
    let mut perm: Vec<usize> = (0..m).collect();
    let mut best_score = 0usize;
    let mut best_winner = 0usize;
    loop {
        let score = kemeny_score(profile, &perm);
        if score > best_score {
            best_score = score;
            best_winner = perm[0];
        }
        if !kemeny_next_perm(&mut perm) {
            break;
        }
    }
    best_winner
}
/// Advance permutation to next lexicographic order. Returns false at last permutation.
pub fn kemeny_next_perm(perm: &mut Vec<usize>) -> bool {
    let n = perm.len();
    if n < 2 {
        return false;
    }
    let mut i = n - 1;
    while i > 0 && perm[i - 1] >= perm[i] {
        i -= 1;
    }
    if i == 0 {
        return false;
    }
    let mut j = n - 1;
    while perm[j] <= perm[i - 1] {
        j -= 1;
    }
    perm.swap(i - 1, j);
    perm[i..].reverse();
    true
}
/// Compute Copeland scores for all alternatives.
///
/// Copeland score(a) = (# alternatives beaten by a) - (# alternatives that beat a).
pub fn copeland_scores(profile: &PreferenceProfile) -> Vec<(usize, i64)> {
    let m = profile.n_alts;
    let mut scores = vec![0i64; m];
    for a in 0..m {
        for b in 0..m {
            if a == b {
                continue;
            }
            let margin_a = profile.majority_margin(a, b);
            let margin_b = profile.majority_margin(b, a);
            if margin_a > margin_b {
                scores[a] += 1;
            } else if margin_b > margin_a {
                scores[a] -= 1;
            }
        }
    }
    let mut result: Vec<(usize, i64)> = scores.into_iter().enumerate().collect();
    result.sort_by_key(|b| std::cmp::Reverse(b.1));
    result
}
/// Return the Copeland winner (highest Copeland score).
pub fn copeland_winner(profile: &PreferenceProfile) -> usize {
    copeland_scores(profile)[0].0
}
/// Compute the Smith set: the smallest non-empty subset S such that every
/// alternative in S beats every alternative not in S in pairwise majority.
pub fn smith_set(profile: &PreferenceProfile) -> Vec<usize> {
    let m = profile.n_alts;
    let beats: Vec<Vec<bool>> = (0..m)
        .map(|i| {
            (0..m)
                .map(|j| i != j && profile.majority_margin(i, j) > profile.majority_margin(j, i))
                .collect()
        })
        .collect();
    let mut dom = beats.clone();
    for k in 0..m {
        for i in 0..m {
            for j in 0..m {
                if dom[i][k] && dom[k][j] {
                    dom[i][j] = true;
                }
            }
        }
    }
    let mut smith: Vec<bool> = vec![true; m];
    let mut changed = true;
    while changed {
        changed = false;
        for i in 0..m {
            if !smith[i] {
                continue;
            }
            for j in 0..m {
                if !smith[j] && dom[j][i] && !dom[i][j] {
                    smith[i] = false;
                    changed = true;
                    break;
                }
            }
        }
    }
    (0..m).filter(|&i| smith[i]).collect()
}
/// Check if a preference profile is single-peaked with respect to a given linear order
/// on alternatives.
///
/// A profile is single-peaked if for each voter, their ranking has a unique peak
/// such that alternatives further from the peak in the linear order are ranked lower.
pub fn is_single_peaked(profile: &PreferenceProfile, order: &[usize]) -> bool {
    for voter in 0..profile.n_voters {
        let top = profile.rankings[voter][0];
        let top_pos = match order.iter().position(|&a| a == top) {
            Some(p) => p,
            None => return false,
        };
        for rank_a in 0..profile.n_alts {
            for rank_b in (rank_a + 1)..profile.n_alts {
                let a = profile.rankings[voter][rank_a];
                let b = profile.rankings[voter][rank_b];
                let pos_a = match order.iter().position(|&x| x == a) {
                    Some(p) => p,
                    None => return false,
                };
                let pos_b = match order.iter().position(|&x| x == b) {
                    Some(p) => p,
                    None => return false,
                };
                let dist_a = (pos_a as i64 - top_pos as i64).unsigned_abs() as usize;
                let dist_b = (pos_b as i64 - top_pos as i64).unsigned_abs() as usize;
                if dist_a > dist_b {
                    return false;
                }
            }
        }
    }
    true
}
/// Find the median voter position (index in `order`) for single-peaked preferences.
///
/// The median of top-ranked positions is a Condorcet winner by the median voter theorem.
pub fn median_voter_position(profile: &PreferenceProfile, order: &[usize]) -> Option<usize> {
    let mut top_positions: Vec<usize> = profile
        .rankings
        .iter()
        .filter_map(|ranking| {
            if ranking.is_empty() {
                return None;
            }
            order.iter().position(|&a| a == ranking[0])
        })
        .collect();
    if top_positions.is_empty() {
        return None;
    }
    top_positions.sort();
    let median_idx = top_positions[top_positions.len() / 2];
    order.get(median_idx).copied()
}
