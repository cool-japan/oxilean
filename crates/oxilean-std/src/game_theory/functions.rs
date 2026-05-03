//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)
#![allow(clippy::items_after_test_module)]

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{
    AuctionResult, CooperativeGameImpl, EvolutionaryGame, GameNode, NPlayerGame, TwoPlayerGame,
    VCGMechanism,
};

pub fn app(f: Expr, a: Expr) -> Expr {
    Expr::App(Box::new(f), Box::new(a))
}
pub fn app2(f: Expr, a: Expr, b: Expr) -> Expr {
    app(app(f, a), b)
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
pub fn impl_pi(name: &str, dom: Expr, body: Expr) -> Expr {
    pi(BinderInfo::Implicit, name, dom, body)
}
pub fn _use_helpers() {
    let _ = (
        app(cst("a"), cst("b")),
        app2(cst("a"), cst("b"), cst("c")),
        prop(),
        type0(),
        arrow(prop(), prop()),
        bvar(0),
        nat_ty(),
        real_ty(),
        list_ty(cst("x")),
        impl_pi("x", type0(), bvar(0)),
    );
}
pub fn normal_form_game_ty() -> Expr {
    arrow(nat_ty(), prop())
}
pub fn nash_equilibrium_ty() -> Expr {
    arrow(prop(), prop())
}
pub fn dominant_strategy_ty() -> Expr {
    arrow(nat_ty(), arrow(prop(), prop()))
}
pub fn pareto_optimal_ty() -> Expr {
    arrow(prop(), prop())
}
pub fn zero_sum_game_ty() -> Expr {
    arrow(prop(), prop())
}
pub fn minimax_theorem_ty() -> Expr {
    arrow(prop(), prop())
}
pub fn extensive_form_game_ty() -> Expr {
    arrow(nat_ty(), prop())
}
pub fn cooperative_game_ty() -> Expr {
    arrow(nat_ty(), prop())
}
pub fn auction_mechanism_ty() -> Expr {
    arrow(type0(), prop())
}
pub fn nash_existence_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        arrow(app(cst("NormalFormGame"), bvar(0)), prop()),
    )
}
pub fn minimax_equality_ty() -> Expr {
    arrow(prop(), arrow(app(cst("ZeroSumGame"), bvar(0)), prop()))
}
pub fn prisoners_dilemma_ty() -> Expr {
    prop()
}
pub fn backward_induction_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        arrow(app(cst("ExtensiveFormGame"), bvar(0)), prop()),
    )
}
pub fn shapley_existence_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        arrow(app(cst("CooperativeGame"), bvar(0)), prop()),
    )
}
pub fn vickrey_truthful_ty() -> Expr {
    prop()
}
pub fn build_game_theory_env(env: &mut Environment) {
    let axioms: &[(&str, Expr)] = &[
        ("NormalFormGame", normal_form_game_ty()),
        ("NashEquilibrium", nash_equilibrium_ty()),
        ("DominantStrategy", dominant_strategy_ty()),
        ("ParetoOptimal", pareto_optimal_ty()),
        ("ZeroSumGame", zero_sum_game_ty()),
        ("MinimaxTheorem", minimax_theorem_ty()),
        ("ExtensiveFormGame", extensive_form_game_ty()),
        ("CooperativeGame", cooperative_game_ty()),
        ("AuctionMechanism", auction_mechanism_ty()),
        ("nash_existence", nash_existence_ty()),
        ("minimax_equality", minimax_equality_ty()),
        ("prisoners_dilemma_dominant", prisoners_dilemma_ty()),
        ("backward_induction", backward_induction_ty()),
        ("shapley_existence", shapley_existence_ty()),
        ("vickrey_truthful", vickrey_truthful_ty()),
        ("MixedStrategy", arrow(nat_ty(), list_ty(real_ty()))),
        ("PureStrategy", arrow(nat_ty(), nat_ty())),
        ("PayoffFunction", arrow(nat_ty(), real_ty())),
        ("EvolutionaryStableStrategy", prop()),
        (
            "ReplicatorDynamics",
            arrow(list_ty(real_ty()), list_ty(real_ty())),
        ),
        ("HawkDoveESS", prop()),
        ("MinimaxValue", arrow(prop(), real_ty())),
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
pub fn prisoners_dilemma() -> TwoPlayerGame {
    TwoPlayerGame::new(
        vec![vec![3.0, 0.0], vec![5.0, 1.0]],
        vec![vec![3.0, 5.0], vec![0.0, 1.0]],
    )
}
pub fn battle_of_sexes() -> TwoPlayerGame {
    TwoPlayerGame::new(
        vec![vec![2.0, 0.0], vec![0.0, 1.0]],
        vec![vec![1.0, 0.0], vec![0.0, 2.0]],
    )
}
pub fn coordination_game() -> TwoPlayerGame {
    TwoPlayerGame::new(
        vec![vec![1.0, 0.0], vec![0.0, 1.0]],
        vec![vec![1.0, 0.0], vec![0.0, 1.0]],
    )
}
pub fn matching_pennies() -> TwoPlayerGame {
    TwoPlayerGame::new(
        vec![vec![1.0, -1.0], vec![-1.0, 1.0]],
        vec![vec![-1.0, 1.0], vec![1.0, -1.0]],
    )
}
pub fn stag_hunt() -> TwoPlayerGame {
    TwoPlayerGame::new(
        vec![vec![4.0, 0.0], vec![3.0, 3.0]],
        vec![vec![4.0, 3.0], vec![0.0, 3.0]],
    )
}
pub fn chicken_game() -> TwoPlayerGame {
    TwoPlayerGame::new(
        vec![vec![0.0, -1.0], vec![1.0, -5.0]],
        vec![vec![0.0, 1.0], vec![-1.0, -5.0]],
    )
}
pub fn rock_paper_scissors() -> TwoPlayerGame {
    TwoPlayerGame::zero_sum(vec![
        vec![0.0, -1.0, 1.0],
        vec![1.0, 0.0, -1.0],
        vec![-1.0, 1.0, 0.0],
    ])
}
/// Ultimatum game: player 0 offers, player 1 accepts/rejects.
pub fn ultimatum_game(total: f64, offer: f64) -> GameNode {
    GameNode::decision(
        0,
        vec![(
            format!("offer_{offer}"),
            GameNode::decision(
                1,
                vec![
                    (
                        "accept".to_string(),
                        GameNode::terminal(vec![total - offer, offer]),
                    ),
                    ("reject".to_string(), GameNode::terminal(vec![0.0, 0.0])),
                ],
            ),
        )],
    )
}
/// Centipede game of given length.
pub fn centipede_game(length: usize) -> GameNode {
    fn build(step: usize, length: usize) -> GameNode {
        if step >= length {
            let p0 = (step / 2 + 1) as f64;
            let p1 = ((step + 1) / 2) as f64;
            return GameNode::terminal(vec![p0, p1]);
        }
        let player = step % 2;
        let take_p0 = if player == 0 {
            (step / 2 + 1) as f64 + 1.0
        } else {
            (step / 2) as f64
        };
        let take_p1 = if player == 0 {
            (step / 2) as f64
        } else {
            ((step + 1) / 2) as f64 + 1.0
        };
        GameNode::decision(
            player,
            vec![
                (
                    "take".to_string(),
                    GameNode::terminal(vec![take_p0, take_p1]),
                ),
                ("pass".to_string(), build(step + 1, length)),
            ],
        )
    }
    build(0, length)
}
pub fn factorial(n: usize) -> usize {
    (1..=n).product()
}
pub fn hawks_doves(v: f64, c: f64) -> EvolutionaryGame {
    EvolutionaryGame::new(
        vec!["Hawk".to_string(), "Dove".to_string()],
        vec![vec![(v - c) / 2.0, v], vec![0.0, v / 2.0]],
    )
}
pub fn rps_evolutionary() -> EvolutionaryGame {
    EvolutionaryGame::new(
        vec!["Rock".into(), "Paper".into(), "Scissors".into()],
        vec![
            vec![0.0, -1.0, 1.0],
            vec![1.0, 0.0, -1.0],
            vec![-1.0, 1.0, 0.0],
        ],
    )
}
pub fn first_price_auction(bids: &[f64]) -> AuctionResult {
    let (mut winner, mut max_bid) = (0, f64::NEG_INFINITY);
    for (i, &bid) in bids.iter().enumerate() {
        if bid > max_bid {
            max_bid = bid;
            winner = i;
        }
    }
    AuctionResult {
        winner,
        price: max_bid,
    }
}
pub fn vickrey_auction(bids: &[f64]) -> AuctionResult {
    if bids.len() < 2 {
        return AuctionResult {
            winner: 0,
            price: if bids.is_empty() { 0.0 } else { bids[0] },
        };
    }
    let mut sorted: Vec<(usize, f64)> = bids.iter().cloned().enumerate().collect();
    sorted.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    AuctionResult {
        winner: sorted[0].0,
        price: sorted[1].1,
    }
}
pub fn all_pay_auction(bids: &[f64]) -> (AuctionResult, f64) {
    let revenue: f64 = bids.iter().sum();
    let (mut winner, mut max_bid) = (0, f64::NEG_INFINITY);
    for (i, &bid) in bids.iter().enumerate() {
        if bid > max_bid {
            max_bid = bid;
            winner = i;
        }
    }
    (
        AuctionResult {
            winner,
            price: max_bid,
        },
        revenue,
    )
}
pub fn vickrey_surplus(valuations: &[f64]) -> Vec<f64> {
    let result = vickrey_auction(valuations);
    let mut surplus = vec![0.0; valuations.len()];
    surplus[result.winner] = valuations[result.winner] - result.price;
    surplus
}
/// Gale-Shapley algorithm for stable matching.
/// `proposer_prefs\[i\]` is proposer i's preference list (most preferred first).
/// `responder_prefs\[j\]` is responder j's preference list.
/// Returns a matching: result\[proposer\] = Some(responder).
pub fn gale_shapley(
    proposer_prefs: &[Vec<usize>],
    responder_prefs: &[Vec<usize>],
) -> Vec<Option<usize>> {
    let n_p = proposer_prefs.len();
    let n_r = responder_prefs.len();
    let mut resp_rank: Vec<Vec<usize>> = vec![vec![usize::MAX; n_p]; n_r];
    for (j, prefs) in responder_prefs.iter().enumerate() {
        for (rank, &p) in prefs.iter().enumerate() {
            if p < n_p {
                resp_rank[j][p] = rank;
            }
        }
    }
    let mut prop_next: Vec<usize> = vec![0; n_p];
    let mut prop_match: Vec<Option<usize>> = vec![None; n_p];
    let mut resp_match: Vec<Option<usize>> = vec![None; n_r];
    let mut free: Vec<usize> = (0..n_p).collect();
    while let Some(&p) = free.last() {
        if prop_next[p] >= proposer_prefs[p].len() {
            free.pop();
            continue;
        }
        let r = proposer_prefs[p][prop_next[p]];
        prop_next[p] += 1;
        if r >= n_r {
            continue;
        }
        match resp_match[r] {
            None => {
                prop_match[p] = Some(r);
                resp_match[r] = Some(p);
                free.pop();
            }
            Some(curr_p) => {
                if resp_rank[r][p] < resp_rank[r][curr_p] {
                    prop_match[p] = Some(r);
                    prop_match[curr_p] = None;
                    resp_match[r] = Some(p);
                    free.pop();
                    free.push(curr_p);
                }
            }
        }
    }
    prop_match
}
/// Check if a matching is stable (no blocking pair).
pub fn is_stable_matching(
    matching: &[Option<usize>],
    proposer_prefs: &[Vec<usize>],
    responder_prefs: &[Vec<usize>],
) -> bool {
    let n_p = proposer_prefs.len();
    let n_r = responder_prefs.len();
    let mut resp_to_prop: Vec<Option<usize>> = vec![None; n_r];
    for (p, &opt_r) in matching.iter().enumerate() {
        if let Some(r) = opt_r {
            if r < n_r {
                resp_to_prop[r] = Some(p);
            }
        }
    }
    let mut prop_rank: Vec<Vec<usize>> = vec![vec![usize::MAX; n_r]; n_p];
    for (p, prefs) in proposer_prefs.iter().enumerate() {
        for (rank, &r) in prefs.iter().enumerate() {
            if r < n_r {
                prop_rank[p][r] = rank;
            }
        }
    }
    let mut resp_rank: Vec<Vec<usize>> = vec![vec![usize::MAX; n_p]; n_r];
    for (r, prefs) in responder_prefs.iter().enumerate() {
        for (rank, &p) in prefs.iter().enumerate() {
            if p < n_p {
                resp_rank[r][p] = rank;
            }
        }
    }
    for p in 0..n_p {
        let p_current_r = matching[p];
        let p_current_rank = p_current_r.map(|r| prop_rank[p][r]).unwrap_or(usize::MAX);
        for &r in &proposer_prefs[p] {
            if r >= n_r {
                continue;
            }
            if prop_rank[p][r] >= p_current_rank {
                continue;
            }
            let r_current_p = resp_to_prop[r];
            let r_current_rank = r_current_p.map(|pp| resp_rank[r][pp]).unwrap_or(usize::MAX);
            if resp_rank[r][p] < r_current_rank {
                return false;
            }
        }
    }
    true
}
/// Plurality voting: candidate with most first-place votes wins.
pub fn plurality_vote(ballots: &[Vec<usize>], n_candidates: usize) -> usize {
    let mut counts = vec![0usize; n_candidates];
    for ballot in ballots {
        if let Some(&first) = ballot.first() {
            if first < n_candidates {
                counts[first] += 1;
            }
        }
    }
    counts
        .iter()
        .enumerate()
        .max_by_key(|&(_, &c)| c)
        .map(|(i, _)| i)
        .unwrap_or(0)
}
/// Borda count voting: each ballot awards n-1 points to first choice, n-2 to second, etc.
pub fn borda_count(ballots: &[Vec<usize>], n_candidates: usize) -> Vec<f64> {
    let mut scores = vec![0.0; n_candidates];
    for ballot in ballots {
        let n = ballot.len();
        for (rank, &cand) in ballot.iter().enumerate() {
            if cand < n_candidates {
                scores[cand] += (n - 1 - rank) as f64;
            }
        }
    }
    scores
}
/// Borda winner: candidate with highest Borda score.
pub fn borda_winner(ballots: &[Vec<usize>], n_candidates: usize) -> usize {
    let scores = borda_count(ballots, n_candidates);
    scores
        .iter()
        .enumerate()
        .max_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal))
        .map(|(i, _)| i)
        .unwrap_or(0)
}
/// Condorcet winner: candidate who beats every other in pairwise majority.
pub fn condorcet_winner(ballots: &[Vec<usize>], n_candidates: usize) -> Option<usize> {
    let mut pairwise = vec![vec![0usize; n_candidates]; n_candidates];
    for ballot in ballots {
        for (i, &a) in ballot.iter().enumerate() {
            for &b in ballot.iter().skip(i + 1) {
                if a < n_candidates && b < n_candidates {
                    pairwise[a][b] += 1;
                }
            }
        }
    }
    let total = ballots.len();
    'outer: for c in 0..n_candidates {
        for d in 0..n_candidates {
            if c == d {
                continue;
            }
            if pairwise[c][d] * 2 <= total {
                continue 'outer;
            }
        }
        return Some(c);
    }
    None
}
/// Instant-runoff voting (IRV): repeatedly eliminate candidate with fewest first-place votes.
pub fn instant_runoff(ballots: &[Vec<usize>], n_candidates: usize) -> usize {
    let mut eliminated = vec![false; n_candidates];
    let mut active_ballots: Vec<Vec<usize>> = ballots.to_vec();
    loop {
        let mut counts = vec![0usize; n_candidates];
        let mut total_votes = 0;
        for ballot in &active_ballots {
            for &c in ballot {
                if c < n_candidates && !eliminated[c] {
                    counts[c] += 1;
                    total_votes += 1;
                    break;
                }
            }
        }
        for (c, &cnt) in counts.iter().enumerate() {
            if !eliminated[c] && cnt * 2 > total_votes {
                return c;
            }
        }
        let mut min_count = usize::MAX;
        let mut to_eliminate = 0;
        for (c, &cnt) in counts.iter().enumerate() {
            if !eliminated[c] && cnt < min_count {
                min_count = cnt;
                to_eliminate = c;
            }
        }
        eliminated[to_eliminate] = true;
        let remaining: usize = eliminated.iter().filter(|&&e| !e).count();
        if remaining <= 1 {
            return eliminated.iter().position(|&e| !e).unwrap_or(0);
        }
        for ballot in &mut active_ballots {
            ballot.retain(|&c| c < n_candidates && !eliminated[c]);
        }
    }
}
/// Payoff in an infinitely repeated game with discount factor delta.
/// Given per-period payoff g, the discounted sum is g * (1 / (1 - delta)).
pub fn repeated_game_payoff(per_period: f64, discount: f64) -> f64 {
    if discount >= 1.0 || discount < 0.0 {
        return f64::INFINITY;
    }
    per_period / (1.0 - discount)
}
/// Folk theorem feasibility check: can the target payoff vector be sustained
/// as a Nash equilibrium of the infinitely repeated game?
/// Returns true if each player's target payoff exceeds their minimax value.
pub fn folk_theorem_feasible(target_payoffs: &[f64], minimax_values: &[f64]) -> bool {
    target_payoffs
        .iter()
        .zip(minimax_values.iter())
        .all(|(&t, &m)| t >= m - 1e-10)
}
/// Tit-for-tat payoff in repeated prisoner's dilemma.
/// Returns the average per-period payoff if both play tit-for-tat starting with cooperate.
pub fn tit_for_tat_payoff(game: &TwoPlayerGame, discount: f64, periods: usize) -> (f64, f64) {
    let mut total_a = 0.0;
    let mut total_b = 0.0;
    let mut weight_sum = 0.0;
    let mut w = 1.0;
    for _ in 0..periods {
        total_a += w * game.payoffs_a[0][0];
        total_b += w * game.payoffs_b[0][0];
        weight_sum += w;
        w *= discount;
    }
    if weight_sum > 0.0 {
        (total_a / weight_sum, total_b / weight_sum)
    } else {
        (0.0, 0.0)
    }
}
/// Grim trigger threshold: minimum discount factor for cooperation to be sustainable.
/// For prisoner's dilemma: delta >= (temptation - reward) / (temptation - punishment)
pub fn grim_trigger_threshold(game: &TwoPlayerGame) -> Option<f64> {
    if game.n_strategies_a < 2 || game.n_strategies_b < 2 {
        return None;
    }
    let reward = game.payoffs_a[0][0];
    let temptation = game.payoffs_a[1][0];
    let punishment = game.payoffs_a[1][1];
    if (temptation - punishment).abs() < 1e-10 {
        return None;
    }
    let threshold = (temptation - reward) / (temptation - punishment);
    Some(threshold)
}
/// Check if a probability distribution over strategy profiles forms a correlated equilibrium
/// for a two-player game. dist\[i\]\[j\] = probability of recommending (i, j).
pub fn is_correlated_equilibrium(game: &TwoPlayerGame, dist: &[Vec<f64>]) -> bool {
    let (m, n) = (game.n_strategies_a, game.n_strategies_b);
    let total: f64 = dist.iter().flat_map(|row| row.iter()).sum();
    if (total - 1.0).abs() > 1e-8 {
        return false;
    }
    for i in 0..m {
        let row_prob: f64 = (0..n).map(|j| dist[i][j]).sum();
        if row_prob < 1e-12 {
            continue;
        }
        for i_prime in 0..m {
            if i_prime == i {
                continue;
            }
            let current_expected: f64 = (0..n).map(|j| dist[i][j] * game.payoffs_a[i][j]).sum();
            let deviate_expected: f64 = (0..n)
                .map(|j| dist[i][j] * game.payoffs_a[i_prime][j])
                .sum();
            if deviate_expected > current_expected + 1e-8 {
                return false;
            }
        }
    }
    for j in 0..n {
        let col_prob: f64 = (0..m).map(|i| dist[i][j]).sum();
        if col_prob < 1e-12 {
            continue;
        }
        for j_prime in 0..n {
            if j_prime == j {
                continue;
            }
            let current_expected: f64 = (0..m).map(|i| dist[i][j] * game.payoffs_b[i][j]).sum();
            let deviate_expected: f64 = (0..m)
                .map(|i| dist[i][j] * game.payoffs_b[i][j_prime])
                .sum();
            if deviate_expected > current_expected + 1e-8 {
                return false;
            }
        }
    }
    true
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_prisoners_dilemma_dominant() {
        let g = prisoners_dilemma();
        assert_eq!(g.dominant_strategy_a(), Some(1));
        assert_eq!(g.dominant_strategy_b(), Some(1));
    }
    #[test]
    fn test_zero_sum_matching_pennies() {
        assert!(matching_pennies().is_zero_sum());
        assert!(!prisoners_dilemma().is_zero_sum());
    }
    #[test]
    fn test_saddle_point() {
        assert_eq!(rock_paper_scissors().saddle_point(), None);
        let g = TwoPlayerGame::new(
            vec![vec![2.0, 4.0], vec![1.0, 3.0]],
            vec![vec![-2.0, -4.0], vec![-1.0, -3.0]],
        );
        assert!(g.is_zero_sum());
        assert_eq!(g.saddle_point(), Some((0, 0)));
    }
    #[test]
    fn test_best_response() {
        let pd = prisoners_dilemma();
        assert_eq!(pd.best_response_a(0), 1);
        assert_eq!(pd.best_response_a(1), 1);
        assert_eq!(pd.best_response_b(0), 1);
        assert_eq!(pd.best_response_b(1), 1);
    }
    #[test]
    fn test_is_pure_nash() {
        let pd = prisoners_dilemma();
        assert!(pd.is_pure_nash(1, 1));
        assert!(!pd.is_pure_nash(0, 0));
        let cg = coordination_game();
        assert!(cg.is_pure_nash(0, 0));
        assert!(cg.is_pure_nash(1, 1));
        assert!(!cg.is_pure_nash(0, 1));
    }
    #[test]
    fn test_all_pure_nash() {
        let nash = coordination_game().all_pure_nash();
        assert_eq!(nash.len(), 2);
        assert!(nash.contains(&(0, 0)));
        assert!(nash.contains(&(1, 1)));
    }
    #[test]
    fn test_pareto_optimal() {
        let pd = prisoners_dilemma();
        assert!(pd.is_pareto_optimal(0, 0));
        assert!(!pd.is_pareto_optimal(1, 1));
    }
    #[test]
    fn test_expected_payoff() {
        let ep = matching_pennies().expected_payoff_a(&[0.5, 0.5], &[0.5, 0.5]);
        assert!(ep.abs() < 1e-10);
    }
    #[test]
    fn test_mixed_nash_2x2() {
        let (p, q) = matching_pennies()
            .mixed_nash_2x2()
            .expect("mixed_nash_2x2 should succeed");
        assert!((p - 0.5).abs() < 1e-10);
        assert!((q - 0.5).abs() < 1e-10);
    }
    #[test]
    fn test_iterated_elimination() {
        let (a, b) = prisoners_dilemma().iterated_elimination();
        assert_eq!(a, vec![1]);
        assert_eq!(b, vec![1]);
    }
    #[test]
    fn test_maximin_minimax_values() {
        let mp = matching_pennies();
        assert!((mp.maximin_value() - (-1.0)).abs() < 1e-10);
        assert!((mp.minimax_value() - 1.0).abs() < 1e-10);
    }
    #[test]
    fn test_zero_sum_constructor() {
        assert!(TwoPlayerGame::zero_sum(vec![vec![1.0, -1.0], vec![-1.0, 1.0]]).is_zero_sum());
    }
    #[test]
    fn test_stag_hunt() {
        let sh = stag_hunt();
        assert!(sh.is_pure_nash(0, 0));
        assert!(sh.is_pure_nash(1, 1));
        assert!(!sh.is_pure_nash(0, 1));
    }
    #[test]
    fn test_chicken_game() {
        let g = chicken_game();
        assert!(g.is_pure_nash(0, 1));
        assert!(g.is_pure_nash(1, 0));
        assert!(!g.is_pure_nash(1, 1));
    }
    #[test]
    fn test_evolutionary_game_avg_fitness() {
        let avg = hawks_doves(4.0, 6.0).avg_fitness(&[0.5, 0.5]);
        assert!((avg - 1.25).abs() < 1e-10);
    }
    #[test]
    fn test_hawks_doves_creation() {
        let hd = hawks_doves(4.0, 6.0);
        assert_eq!(hd.strategies.len(), 2);
        assert!((hd.fitness_matrix[0][0] - (-1.0)).abs() < 1e-10);
        assert!((hd.fitness_matrix[1][1] - 2.0).abs() < 1e-10);
    }
    #[test]
    fn test_ess_check() {
        let hd = hawks_doves(4.0, 6.0);
        assert!(!hd.is_ess(0));
        assert!(!hd.is_ess(1));
    }
    #[test]
    fn test_ess_dominant() {
        let g = EvolutionaryGame::new(
            vec!["A".into(), "B".into()],
            vec![vec![3.0, 1.0], vec![0.0, 2.0]],
        );
        assert!(g.is_ess(0));
    }
    #[test]
    fn test_rps_evolutionary() {
        assert!(rps_evolutionary().find_all_ess().is_empty());
    }
    #[test]
    fn test_replicator_dynamics() {
        let history = hawks_doves(4.0, 6.0).simulate(vec![0.5, 0.5], 1000, 0.05);
        let final_hawk = history.last().expect("last should succeed")[0];
        assert!((final_hawk - 2.0 / 3.0).abs() < 0.05, "got {final_hawk}");
    }
    #[test]
    fn test_minimax_row_col() {
        let g = TwoPlayerGame::new(
            vec![vec![3.0, 1.0], vec![0.0, 4.0]],
            vec![vec![-3.0, -1.0], vec![0.0, -4.0]],
        );
        assert_eq!(g.minimax_row(), 0);
        assert_eq!(g.minimax_col(), 0);
    }
    #[test]
    fn test_backward_induction() {
        let (payoffs, _) = ultimatum_game(10.0, 3.0).backward_induction();
        assert!((payoffs[0] - 7.0).abs() < 1e-10);
        assert!((payoffs[1] - 3.0).abs() < 1e-10);
    }
    #[test]
    fn test_node_count() {
        let g = ultimatum_game(10.0, 3.0);
        assert_eq!(g.node_count(), 4);
        assert_eq!(g.terminal_count(), 2);
    }
    #[test]
    fn test_centipede_game() {
        let (_, action) = centipede_game(4).backward_induction();
        assert_eq!(action, "take");
    }
    #[test]
    fn test_shapley_value() {
        let mut vals = vec![0.0; 8];
        for mask in 0..8u32 {
            if mask.count_ones() >= 2 {
                vals[mask as usize] = 1.0;
            }
        }
        let phi = CooperativeGameImpl::new(3, vals).shapley_value();
        for &p in &phi {
            assert!((p - 1.0 / 3.0).abs() < 1e-10);
        }
    }
    #[test]
    fn test_superadditive() {
        assert!(CooperativeGameImpl::new(2, vec![0.0, 3.0, 4.0, 10.0]).is_superadditive());
    }
    #[test]
    fn test_core() {
        let g = CooperativeGameImpl::new(2, vec![0.0, 3.0, 4.0, 10.0]);
        assert!(g.is_in_core(&[5.0, 5.0]));
        assert!(!g.is_in_core(&[2.0, 8.0]));
    }
    #[test]
    fn test_shapley_in_core() {
        assert!(CooperativeGameImpl::new(2, vec![0.0, 0.0, 0.0, 10.0]).shapley_in_core());
    }
    #[test]
    fn test_first_price_auction() {
        let r = first_price_auction(&[10.0, 20.0, 15.0]);
        assert_eq!(r.winner, 1);
        assert!((r.price - 20.0).abs() < 1e-10);
    }
    #[test]
    fn test_vickrey_auction() {
        let r = vickrey_auction(&[10.0, 20.0, 15.0]);
        assert_eq!(r.winner, 1);
        assert!((r.price - 15.0).abs() < 1e-10);
    }
    #[test]
    fn test_vickrey_truthful() {
        let s = vickrey_surplus(&[10.0, 20.0, 15.0]);
        assert!((s[1] - 5.0).abs() < 1e-10);
        assert!(s[0].abs() < 1e-10);
    }
    #[test]
    fn test_all_pay_auction() {
        let (r, rev) = all_pay_auction(&[10.0, 20.0, 15.0]);
        assert_eq!(r.winner, 1);
        assert!((rev - 45.0).abs() < 1e-10);
    }
    #[test]
    fn test_weakly_dominant() {
        let g = TwoPlayerGame::new(
            vec![vec![3.0, 2.0], vec![3.0, 1.0]],
            vec![vec![1.0, 1.0], vec![1.0, 1.0]],
        );
        assert_eq!(g.weakly_dominant_strategy_a(), Some(0));
        assert!(g.dominant_strategy_a().is_none());
    }
    #[test]
    fn test_build_game_theory_env() {
        let mut env = Environment::new();
        build_game_theory_env(&mut env);
        assert!(env.get(&Name::str("NormalFormGame")).is_some());
        assert!(env.get(&Name::str("ExtensiveFormGame")).is_some());
        assert!(env.get(&Name::str("vickrey_truthful")).is_some());
    }
    #[test]
    fn test_chance_node() {
        let (p, _) = GameNode::chance(vec![
            (0.5, GameNode::terminal(vec![10.0, 0.0])),
            (0.5, GameNode::terminal(vec![0.0, 10.0])),
        ])
        .backward_induction();
        assert!((p[0] - 5.0).abs() < 1e-10);
        assert!((p[1] - 5.0).abs() < 1e-10);
    }
    #[test]
    fn test_battle_of_sexes_mixed() {
        let (p, q) = battle_of_sexes()
            .mixed_nash_2x2()
            .expect("mixed_nash_2x2 should succeed");
        assert!((p - 2.0 / 3.0).abs() < 1e-10);
        assert!((q - 1.0 / 3.0).abs() < 1e-10);
    }
    #[test]
    fn test_rps_no_pure_nash() {
        assert!(rock_paper_scissors().all_pure_nash().is_empty());
    }
    #[test]
    fn test_gale_shapley_basic() {
        let prop_prefs = vec![vec![0, 1, 2], vec![1, 0, 2], vec![0, 1, 2]];
        let resp_prefs = vec![vec![1, 0, 2], vec![0, 1, 2], vec![0, 1, 2]];
        let matching = gale_shapley(&prop_prefs, &resp_prefs);
        for m in &matching {
            assert!(m.is_some());
        }
        let mut matched_r: Vec<bool> = vec![false; 3];
        for r in matching.iter().flatten() {
            assert!(!matched_r[*r]);
            matched_r[*r] = true;
        }
    }
    #[test]
    fn test_gale_shapley_stability() {
        let prop_prefs = vec![vec![0, 1], vec![1, 0]];
        let resp_prefs = vec![vec![0, 1], vec![1, 0]];
        let matching = gale_shapley(&prop_prefs, &resp_prefs);
        assert!(is_stable_matching(&matching, &prop_prefs, &resp_prefs));
    }
    #[test]
    fn test_gale_shapley_proposer_optimal() {
        let prop_prefs = vec![vec![0, 1], vec![0, 1]];
        let resp_prefs = vec![vec![0, 1], vec![0, 1]];
        let matching = gale_shapley(&prop_prefs, &resp_prefs);
        assert_eq!(matching[0], Some(0));
        assert_eq!(matching[1], Some(1));
        assert!(is_stable_matching(&matching, &prop_prefs, &resp_prefs));
    }
    #[test]
    fn test_plurality_vote() {
        let ballots = vec![vec![0, 1, 2], vec![0, 2, 1], vec![1, 0, 2], vec![2, 1, 0]];
        assert_eq!(plurality_vote(&ballots, 3), 0);
    }
    #[test]
    fn test_borda_count() {
        let ballots = vec![vec![0, 1, 2], vec![1, 0, 2], vec![2, 1, 0]];
        let scores = borda_count(&ballots, 3);
        assert!((scores[0] - 3.0).abs() < 1e-10);
        assert!((scores[1] - 4.0).abs() < 1e-10);
        assert!((scores[2] - 2.0).abs() < 1e-10);
        assert_eq!(borda_winner(&ballots, 3), 1);
    }
    #[test]
    fn test_condorcet_winner() {
        let ballots = vec![
            vec![0, 1, 2],
            vec![0, 1, 2],
            vec![0, 1, 2],
            vec![1, 2, 0],
            vec![1, 2, 0],
            vec![2, 0, 1],
            vec![2, 0, 1],
        ];
        assert_eq!(condorcet_winner(&ballots, 3), None);
    }
    #[test]
    fn test_condorcet_winner_exists() {
        let ballots = vec![vec![0, 1, 2], vec![0, 1, 2], vec![0, 1, 2]];
        assert_eq!(condorcet_winner(&ballots, 3), Some(0));
    }
    #[test]
    fn test_instant_runoff() {
        let ballots = vec![
            vec![0, 1, 2],
            vec![0, 1, 2],
            vec![1, 2, 0],
            vec![2, 1, 0],
            vec![2, 1, 0],
        ];
        assert_eq!(instant_runoff(&ballots, 3), 2);
    }
    #[test]
    fn test_repeated_game_payoff() {
        let payoff = repeated_game_payoff(3.0, 0.5);
        assert!((payoff - 6.0).abs() < 1e-10);
    }
    #[test]
    fn test_folk_theorem_feasible() {
        assert!(folk_theorem_feasible(&[3.0, 3.0], &[1.0, 1.0]));
        assert!(!folk_theorem_feasible(&[0.5, 3.0], &[1.0, 1.0]));
    }
    #[test]
    fn test_tit_for_tat_payoff() {
        let pd = prisoners_dilemma();
        let (pa, pb) = tit_for_tat_payoff(&pd, 0.9, 100);
        assert!((pa - 3.0).abs() < 1e-6);
        assert!((pb - 3.0).abs() < 1e-6);
    }
    #[test]
    fn test_grim_trigger_threshold() {
        let pd = prisoners_dilemma();
        let threshold = grim_trigger_threshold(&pd).expect("operation should succeed");
        assert!((threshold - 0.5).abs() < 1e-10);
    }
    #[test]
    fn test_n_player_profile_conversion() {
        let game = NPlayerGame::new(3, vec![2, 2, 2], vec![vec![0.0; 3]; 8]);
        assert_eq!(game.total_profiles(), 8);
        assert_eq!(game.profile_to_index(&[0, 0, 0]), 0);
        assert_eq!(game.profile_to_index(&[1, 1, 1]), 7);
        assert_eq!(game.index_to_profile(0), vec![0, 0, 0]);
        assert_eq!(game.index_to_profile(7), vec![1, 1, 1]);
        assert_eq!(game.index_to_profile(5), vec![1, 0, 1]);
    }
    #[test]
    fn test_n_player_nash() {
        let payoffs = vec![
            vec![3.0, 3.0],
            vec![0.0, 5.0],
            vec![5.0, 0.0],
            vec![1.0, 1.0],
        ];
        let game = NPlayerGame::new(2, vec![2, 2], payoffs);
        let nash = game.all_pure_nash_profiles();
        assert_eq!(nash.len(), 1);
        assert_eq!(nash[0], vec![1, 1]);
    }
    #[test]
    fn test_n_player_coordination() {
        let payoffs = vec![
            vec![2.0, 2.0],
            vec![0.0, 0.0],
            vec![0.0, 0.0],
            vec![1.0, 1.0],
        ];
        let game = NPlayerGame::new(2, vec![2, 2], payoffs);
        let nash = game.all_pure_nash_profiles();
        assert_eq!(nash.len(), 2);
        assert!(nash.contains(&vec![0, 0]));
        assert!(nash.contains(&vec![1, 1]));
    }
    #[test]
    fn test_correlated_equilibrium_pure_nash() {
        let game = coordination_game();
        let dist = vec![vec![1.0, 0.0], vec![0.0, 0.0]];
        assert!(is_correlated_equilibrium(&game, &dist));
    }
    #[test]
    fn test_correlated_equilibrium_mixed() {
        let game = coordination_game();
        let dist = vec![vec![0.5, 0.0], vec![0.0, 0.5]];
        assert!(is_correlated_equilibrium(&game, &dist));
    }
    #[test]
    fn test_not_correlated_equilibrium() {
        let pd = prisoners_dilemma();
        let dist = vec![vec![1.0, 0.0], vec![0.0, 0.0]];
        assert!(!is_correlated_equilibrium(&pd, &dist));
    }
}
/// `TremblingHandEquilibrium : NormalFormGame → MixedStrategy → Prop`
/// Trembling-hand perfect equilibrium: each player's strategy is a best
/// response even when other players tremble with small probability ε.
pub fn gt_ext_trembling_hand_ty() -> Expr {
    arrow(
        app(cst("NormalFormGame"), nat_ty()),
        arrow(app(cst("MixedStrategy"), nat_ty()), prop()),
    )
}
/// `ProperEquilibrium : NormalFormGame → MixedStrategy → Prop`
/// Myerson's proper equilibrium: less costly mistakes are made less often.
pub fn gt_ext_proper_equilibrium_ty() -> Expr {
    arrow(
        app(cst("NormalFormGame"), nat_ty()),
        arrow(app(cst("MixedStrategy"), nat_ty()), prop()),
    )
}
/// `SequentialEquilibrium : ExtensiveFormGame → List MixedStrategy → Prop`
/// Kreps-Wilson sequential equilibrium: consistent beliefs + sequential rationality.
pub fn gt_ext_sequential_equilibrium_ty() -> Expr {
    arrow(
        app(cst("ExtensiveFormGame"), nat_ty()),
        arrow(list_ty(app(cst("MixedStrategy"), nat_ty())), prop()),
    )
}
/// `PerfectBayesianEquilibrium : ExtensiveFormGame → List MixedStrategy → Prop`
/// Perfect Bayesian equilibrium: sequential rationality + Bayesian-consistent beliefs.
pub fn gt_ext_perfect_bayesian_ty() -> Expr {
    arrow(
        app(cst("ExtensiveFormGame"), nat_ty()),
        arrow(list_ty(app(cst("MixedStrategy"), nat_ty())), prop()),
    )
}
/// `SubgamePerfectEquilibrium : ExtensiveFormGame → List MixedStrategy → Prop`
/// Subgame perfect equilibrium: a Nash equilibrium in every subgame.
pub fn gt_ext_subgame_perfect_ty() -> Expr {
    arrow(
        app(cst("ExtensiveFormGame"), nat_ty()),
        arrow(list_ty(app(cst("MixedStrategy"), nat_ty())), prop()),
    )
}
/// `QuantalResponseEquilibrium : NormalFormGame → Real → MixedStrategy → Prop`
/// Quantal response equilibrium (QRE) with rationality parameter λ.
pub fn gt_ext_qre_ty() -> Expr {
    arrow(
        app(cst("NormalFormGame"), nat_ty()),
        arrow(
            real_ty(),
            arrow(app(cst("MixedStrategy"), nat_ty()), prop()),
        ),
    )
}
/// `FictitiousPlay : NormalFormGame → List MixedStrategy → Prop`
/// Fictitious play: each player best-responds to the empirical distribution.
pub fn gt_ext_fictitious_play_ty() -> Expr {
    arrow(
        app(cst("NormalFormGame"), nat_ty()),
        arrow(list_ty(app(cst("MixedStrategy"), nat_ty())), prop()),
    )
}
/// `BestResponseDynamics : NormalFormGame → List PureStrategy → Prop`
/// Best response dynamics: each player updates to best response in each period.
pub fn gt_ext_best_response_dynamics_ty() -> Expr {
    arrow(
        app(cst("NormalFormGame"), nat_ty()),
        arrow(list_ty(app(cst("PureStrategy"), nat_ty())), prop()),
    )
}
/// `NoRegretLearning : NormalFormGame → List MixedStrategy → Prop`
/// No-regret learning: sequence of strategies with vanishing external regret.
pub fn gt_ext_no_regret_ty() -> Expr {
    arrow(
        app(cst("NormalFormGame"), nat_ty()),
        arrow(list_ty(app(cst("MixedStrategy"), nat_ty())), prop()),
    )
}
/// `RegretBound : NormalFormGame → Nat → Real`
/// External regret bound after T rounds of no-regret learning.
pub fn gt_ext_regret_bound_ty() -> Expr {
    arrow(
        app(cst("NormalFormGame"), nat_ty()),
        arrow(nat_ty(), real_ty()),
    )
}
/// `FictitiousPlayConvergence : NormalFormGame → Prop`
/// Theorem: fictitious play converges to Nash in zero-sum games.
pub fn gt_ext_fictitious_play_convergence_ty() -> Expr {
    arrow(app(cst("NormalFormGame"), nat_ty()), prop())
}
/// `PotentialFunction : NormalFormGame → (List PureStrategy → Real) → Prop`
/// Monderer-Shapley potential: a function Φ s.t. unilateral deviations in payoff
/// equal changes in Φ.
pub fn gt_ext_potential_function_ty() -> Expr {
    arrow(
        app(cst("NormalFormGame"), nat_ty()),
        arrow(
            arrow(list_ty(app(cst("PureStrategy"), nat_ty())), real_ty()),
            prop(),
        ),
    )
}
/// `ExactPotentialGame : NormalFormGame → Prop`
/// A game admitting an exact potential function.
pub fn gt_ext_exact_potential_game_ty() -> Expr {
    arrow(app(cst("NormalFormGame"), nat_ty()), prop())
}
/// `SupermodularGame : NormalFormGame → Prop`
/// A supermodular game: payoffs have increasing differences in strategies.
pub fn gt_ext_supermodular_game_ty() -> Expr {
    arrow(app(cst("NormalFormGame"), nat_ty()), prop())
}
/// `StrategicComplementarity : NormalFormGame → Prop`
/// Strategic complementarity: best responses are non-decreasing in others' strategies.
pub fn gt_ext_strategic_complementarity_ty() -> Expr {
    arrow(app(cst("NormalFormGame"), nat_ty()), prop())
}
/// Theorem: Every finite potential game has at least one pure Nash equilibrium.
pub fn gt_ext_potential_game_nash_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "g",
        app(cst("NormalFormGame"), nat_ty()),
        arrow(app(cst("ExactPotentialGame"), bvar(0)), prop()),
    )
}
/// `DirectMechanism : Type → Type`
/// A direct revelation mechanism mapping type reports to outcomes.
pub fn gt_ext_direct_mechanism_ty() -> Expr {
    arrow(type0(), type0())
}
/// `IncentiveCompatibility : DirectMechanism → Prop`
/// Dominant strategy incentive compatibility (DSIC): truthful reporting is optimal.
pub fn gt_ext_incentive_compatibility_ty() -> Expr {
    arrow(app(cst("DirectMechanism"), type0()), prop())
}
/// `IndividualRationality : DirectMechanism → Prop`
/// Participation constraint: agents prefer to participate truthfully.
pub fn gt_ext_individual_rationality_ty() -> Expr {
    arrow(app(cst("DirectMechanism"), type0()), prop())
}
/// `RevealationPrinciple : DirectMechanism → Prop`
/// Any equilibrium outcome can be implemented by a direct truthful mechanism.
pub fn gt_ext_revelation_principle_ty() -> Expr {
    arrow(app(cst("DirectMechanism"), type0()), prop())
}
/// `VCGMechanism : Type → DirectMechanism`
/// Vickrey-Clarke-Groves mechanism: efficient + DSIC allocation rule.
pub fn gt_ext_vcg_mechanism_ty() -> Expr {
    arrow(type0(), app(cst("DirectMechanism"), type0()))
}
/// `MyersonLemma : DirectMechanism → Prop`
/// Myerson's lemma: a monotone allocation rule is DSIC iff payments satisfy
/// the payment identity.
pub fn gt_ext_myerson_lemma_ty() -> Expr {
    arrow(app(cst("DirectMechanism"), type0()), prop())
}
/// `RevenueEquivalenceTheorem : DirectMechanism → DirectMechanism → Prop`
/// Revenue equivalence: all efficient DSIC mechanisms yield same expected revenue.
pub fn gt_ext_revenue_equivalence_ty() -> Expr {
    arrow(
        app(cst("DirectMechanism"), type0()),
        arrow(app(cst("DirectMechanism"), type0()), prop()),
    )
}
/// `OptimalAuction : Type → DirectMechanism`
/// Myerson's optimal auction mechanism maximizing expected revenue.
pub fn gt_ext_optimal_auction_ty() -> Expr {
    arrow(type0(), app(cst("DirectMechanism"), type0()))
}
/// `EnglishAuction : Nat → Real → Prop`
/// English (ascending) auction: winner pays second-highest value + ε.
pub fn gt_ext_english_auction_ty() -> Expr {
    arrow(nat_ty(), arrow(real_ty(), prop()))
}
/// `DutchAuction : Nat → Real → Prop`
/// Dutch (descending) auction: strategically equivalent to first-price sealed-bid.
pub fn gt_ext_dutch_auction_ty() -> Expr {
    arrow(nat_ty(), arrow(real_ty(), prop()))
}
/// `CommonValueAuction : Nat → Real → Prop`
/// Common value auction: winners curse phenomenon in affiliated value settings.
pub fn gt_ext_common_value_auction_ty() -> Expr {
    arrow(nat_ty(), arrow(real_ty(), prop()))
}
/// `WinnersCurse : CommonValueAuction → Prop`
/// Winner's curse: the winning bidder systematically overestimates the value.
pub fn gt_ext_winners_curse_ty() -> Expr {
    arrow(app2(cst("CommonValueAuction"), nat_ty(), real_ty()), prop())
}
/// `AffiliatedValues : Nat → Prop`
/// Milgrom-Weber affiliated values model for auction theory.
pub fn gt_ext_affiliated_values_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `ConvexGame : CooperativeGame → Prop`
/// A convex game: v(S ∪ T) + v(S ∩ T) ≥ v(S) + v(T) for all S, T.
pub fn gt_ext_convex_game_ty() -> Expr {
    arrow(app(cst("CooperativeGame"), nat_ty()), prop())
}
/// `BanzhafValue : CooperativeGame → List Real`
/// Banzhaf power index: marginal contribution averaged over all coalitions.
pub fn gt_ext_banzhaf_value_ty() -> Expr {
    arrow(app(cst("CooperativeGame"), nat_ty()), list_ty(real_ty()))
}
/// `WeightedVotingGame : List Real → Real → CooperativeGame`
/// Weighted voting game \[q; w_1, ..., w_n\] with quota q.
pub fn gt_ext_weighted_voting_game_ty() -> Expr {
    arrow(
        list_ty(real_ty()),
        arrow(real_ty(), app(cst("CooperativeGame"), nat_ty())),
    )
}
/// `Nucleolus : CooperativeGame → List Real`
/// Schmeidler's nucleolus: lexicographically minimizing the vector of excesses.
pub fn gt_ext_nucleolus_ty() -> Expr {
    arrow(app(cst("CooperativeGame"), nat_ty()), list_ty(real_ty()))
}
/// `Prenucleolus : CooperativeGame → List Real`
/// The prenucleolus (without individual rationality constraint).
pub fn gt_ext_prenucleolus_ty() -> Expr {
    arrow(app(cst("CooperativeGame"), nat_ty()), list_ty(real_ty()))
}
/// `ConvexGameShapleyCore : ConvexGame → Prop`
/// Theorem: the Shapley value lies in the core of a convex game.
pub fn gt_ext_convex_shapley_core_ty() -> Expr {
    arrow(
        app2(
            cst("ConvexGame"),
            app(cst("CooperativeGame"), nat_ty()),
            nat_ty(),
        ),
        prop(),
    )
}
/// `NashBargainingSolution : Type → Real × Real`
/// Nash bargaining solution: maximizes the product of utility gains.
pub fn gt_ext_nash_bargaining_ty() -> Expr {
    arrow(type0(), app(app(cst("Prod"), real_ty()), real_ty()))
}
/// `KalaiSmorodinskySolution : Type → Real × Real`
/// Kalai-Smorodinsky solution: proportional to players' ideal payoffs.
pub fn gt_ext_kalai_smorodinsky_ty() -> Expr {
    arrow(type0(), app(app(cst("Prod"), real_ty()), real_ty()))
}
/// `BargainingProblem : Real × Real → Real × Real → Type`
/// A bargaining problem: (feasible set, disagreement point).
pub fn gt_ext_bargaining_problem_ty() -> Expr {
    arrow(
        app(app(cst("Prod"), real_ty()), real_ty()),
        arrow(app(app(cst("Prod"), real_ty()), real_ty()), type0()),
    )
}
/// Nash axioms for bargaining: efficiency, symmetry, IIA, invariance.
pub fn gt_ext_nash_bargaining_axioms_ty() -> Expr {
    prop()
}
/// `BayesianGame : Nat → Type`
/// Bayesian game: a game with private type information for each player.
pub fn gt_ext_bayesian_game_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `BayesianNashEquilibrium : BayesianGame → List MixedStrategy → Prop`
/// Bayesian Nash equilibrium: maximize expected payoff given beliefs about types.
pub fn gt_ext_bayesian_nash_ty() -> Expr {
    arrow(
        app(cst("BayesianGame"), nat_ty()),
        arrow(list_ty(app(cst("MixedStrategy"), nat_ty())), prop()),
    )
}
/// `HarsanyiPurification : NormalFormGame → Prop`
/// Harsanyi purification theorem: any mixed Nash is a limit of pure Bayesian Nash.
pub fn gt_ext_harsanyi_purification_ty() -> Expr {
    arrow(app(cst("NormalFormGame"), nat_ty()), prop())
}
/// `GlobalGame : BayesianGame → Prop`
/// A global game: unique equilibrium selection via higher-order beliefs.
pub fn gt_ext_global_game_ty() -> Expr {
    arrow(app(cst("BayesianGame"), nat_ty()), prop())
}
/// Register all extended game theory axioms into the environment.
pub fn register_game_theory_extended(env: &mut Environment) -> Result<(), String> {
    let axioms: &[(&str, Expr)] = &[
        ("TremblingHandEquilibrium", gt_ext_trembling_hand_ty()),
        ("ProperEquilibrium", gt_ext_proper_equilibrium_ty()),
        ("SequentialEquilibrium", gt_ext_sequential_equilibrium_ty()),
        ("PerfectBayesianEquilibrium", gt_ext_perfect_bayesian_ty()),
        ("SubgamePerfectEquilibrium", gt_ext_subgame_perfect_ty()),
        ("QuantalResponseEquilibrium", gt_ext_qre_ty()),
        ("FictitiousPlay", gt_ext_fictitious_play_ty()),
        ("BestResponseDynamics", gt_ext_best_response_dynamics_ty()),
        ("NoRegretLearning", gt_ext_no_regret_ty()),
        ("RegretBound", gt_ext_regret_bound_ty()),
        (
            "FictitiousPlayConvergence",
            gt_ext_fictitious_play_convergence_ty(),
        ),
        ("PotentialFunction", gt_ext_potential_function_ty()),
        ("ExactPotentialGame", gt_ext_exact_potential_game_ty()),
        ("SupermodularGame", gt_ext_supermodular_game_ty()),
        (
            "StrategicComplementarity",
            gt_ext_strategic_complementarity_ty(),
        ),
        ("PotentialGameNash", gt_ext_potential_game_nash_ty()),
        ("DirectMechanism", gt_ext_direct_mechanism_ty()),
        (
            "IncentiveCompatibility",
            gt_ext_incentive_compatibility_ty(),
        ),
        ("IndividualRationality", gt_ext_individual_rationality_ty()),
        ("RevelationPrinciple", gt_ext_revelation_principle_ty()),
        ("VCGMechanism", gt_ext_vcg_mechanism_ty()),
        ("MyersonLemma", gt_ext_myerson_lemma_ty()),
        ("RevenueEquivalenceTheorem", gt_ext_revenue_equivalence_ty()),
        ("OptimalAuction", gt_ext_optimal_auction_ty()),
        ("EnglishAuction", gt_ext_english_auction_ty()),
        ("DutchAuction", gt_ext_dutch_auction_ty()),
        ("CommonValueAuction", gt_ext_common_value_auction_ty()),
        ("WinnersCurse", gt_ext_winners_curse_ty()),
        ("AffiliatedValues", gt_ext_affiliated_values_ty()),
        ("ConvexGame", gt_ext_convex_game_ty()),
        ("BanzhafValue", gt_ext_banzhaf_value_ty()),
        ("WeightedVotingGame", gt_ext_weighted_voting_game_ty()),
        ("Nucleolus", gt_ext_nucleolus_ty()),
        ("Prenucleolus", gt_ext_prenucleolus_ty()),
        ("ConvexGameShapleyCore", gt_ext_convex_shapley_core_ty()),
        ("NashBargainingSolution", gt_ext_nash_bargaining_ty()),
        ("KalaiSmorodinskySolution", gt_ext_kalai_smorodinsky_ty()),
        ("BargainingProblem", gt_ext_bargaining_problem_ty()),
        ("NashBargainingAxioms", gt_ext_nash_bargaining_axioms_ty()),
        ("BayesianGame", gt_ext_bayesian_game_ty()),
        ("BayesianNashEquilibrium", gt_ext_bayesian_nash_ty()),
        ("HarsanyiPurification", gt_ext_harsanyi_purification_ty()),
        ("GlobalGame", gt_ext_global_game_ty()),
    ];
    for (name, ty) in axioms {
        env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty: ty.clone(),
        })
        .map_err(|e| format!("Failed to add '{}': {:?}", name, e))?;
    }
    Ok(())
}
/// Softmax function: s_i = exp(λ * u_i) / ∑_j exp(λ * u_j).
pub fn softmax(utils: &[f64], lambda: f64) -> Vec<f64> {
    if utils.is_empty() {
        return vec![];
    }
    let max_u = utils.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let exps: Vec<f64> = utils
        .iter()
        .map(|&u| ((u - max_u) * lambda).exp())
        .collect();
    let sum: f64 = exps.iter().sum();
    if sum < 1e-300 {
        let n = utils.len();
        return vec![1.0 / n as f64; n];
    }
    exps.iter().map(|&e| e / sum).collect()
}
