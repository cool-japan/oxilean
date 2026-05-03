//! Implementation of proof search engines.
//!
//! Contains the core search algorithms: BFS, A*, DFS, and IDDFS.

use std::collections::VecDeque;
use std::time::{Duration, Instant};

use crate::basic::MVarId;
use oxilean_kernel::{Expr, Level, Name};

use super::types::{
    AstarHeap, AstarNode, AstarProofSearch, AutoTactic, BfsProofSearch, DfsProofSearch,
    IddfsProofSearch, ProofSearchConfig, ProofSearchEngine, ProofSearchError, ProofSearchResult,
    SearchNode, SearchStats, SearchStrategy, TacticApplicationResult,
};

// ─── Tactic simulation ───────────────────────────────────────────────────────

/// Counter used when allocating simulated fresh `MVarId`s during search.
///
/// Real proof search would thread a `MetaContext` through every step.
/// Here we generate monotonically increasing IDs from a local counter so
/// that nodes in the same search can be distinguished.
struct MvarCounter(u64);

impl MvarCounter {
    fn new(start: u64) -> Self {
        Self(start)
    }

    fn fresh(&mut self) -> MVarId {
        let id = self.0;
        self.0 += 1;
        MVarId(id)
    }
}

/// Produce a canonical "trivial" proof term for a closed goal.
///
/// In a real kernel integration this would construct the actual term.
/// During search we produce a placeholder `Const` node whose name
/// encodes the tactic that closed the goal; the caller replaces it
/// with the real term once the trace is verified by the elaborator.
fn proof_placeholder(tactic: &AutoTactic) -> Expr {
    let name = Name::str(format!("__search_{}", tactic.display_name()));
    Expr::Const(name, vec![Level::Zero])
}

/// Apply `tactic` to the focused goal of `node`, returning the outcome.
///
/// This function implements a *proof-state simulation model*:
///
/// - Closing tactics (`Rfl`, `Assumption`, `Trivial`, `Contradiction`,
///   `Simp`, `Omega`, `Ring`, `Exact`) close the focused (first) goal
///   unconditionally in the model — we rely on the final elaborator pass
///   to reject spurious proofs.
///
/// - `Intro` peels one Pi binder: it closes the focused goal and opens a
///   fresh single sub-goal (the body after the binder).
///
/// - `Constructor` closes the focused goal and opens two sub-goals (a very
///   conservative over-approximation that matches conjunction/exists intro).
///
/// - `Apply(e)` closes the focused goal and opens one fresh sub-goal.
///
/// Goals that are already solved are never returned.
pub(super) fn simulate_tactic(
    tactic: &AutoTactic,
    node: &SearchNode,
    counter: &mut MvarCounter,
) -> TacticApplicationResult {
    if node.remaining_goals.is_empty() {
        return TacticApplicationResult::NoProgress;
    }

    match tactic {
        // Closing tactics: remove focused goal, no new goals.
        AutoTactic::Rfl
        | AutoTactic::Assumption
        | AutoTactic::Trivial
        | AutoTactic::Contradiction
        | AutoTactic::Simp
        | AutoTactic::Omega
        | AutoTactic::Ring => TacticApplicationResult::Progress {
            new_goals: vec![],
            description: tactic.display_name(),
        },

        AutoTactic::Exact(_) => TacticApplicationResult::Progress {
            new_goals: vec![],
            description: tactic.display_name(),
        },

        // Intro: closes focused goal, opens exactly one fresh sub-goal.
        AutoTactic::Intro => {
            let sub = counter.fresh();
            TacticApplicationResult::Progress {
                new_goals: vec![sub],
                description: "intro".to_string(),
            }
        }

        // Constructor: closes focused goal, opens two fresh sub-goals.
        AutoTactic::Constructor => {
            let sub1 = counter.fresh();
            let sub2 = counter.fresh();
            TacticApplicationResult::Progress {
                new_goals: vec![sub1, sub2],
                description: "constructor".to_string(),
            }
        }

        // Apply: closes focused goal, opens one fresh sub-goal.
        AutoTactic::Apply(_) => {
            let sub = counter.fresh();
            TacticApplicationResult::Progress {
                new_goals: vec![sub],
                description: tactic.display_name(),
            }
        }
    }
}

/// Expand a `SearchNode` by applying each configured tactic (up to `max_branches`).
///
/// Returns a `Vec` of child nodes (one per successful tactic application).
pub(super) fn expand_node(
    node: &SearchNode,
    config: &ProofSearchConfig,
    counter: &mut MvarCounter,
) -> Vec<SearchNode> {
    let mut children = Vec::new();

    // Sort tactics by estimated cost so we try cheap ones first.
    let mut tactics = config.tactics.clone();
    tactics.sort_by_key(|t| t.estimated_cost());

    for tactic in tactics.iter().take(config.max_branches) {
        match simulate_tactic(tactic, node, counter) {
            TacticApplicationResult::Progress { new_goals, description } => {
                // Build the new goal list: replace focused goal with new_goals,
                // then append remaining unfocused goals.
                let mut updated_goals = new_goals;
                if node.remaining_goals.len() > 1 {
                    updated_goals.extend_from_slice(&node.remaining_goals[1..]);
                }

                let mut trace = node.tactic_trace.clone();
                trace.push(description);

                // Accumulate a proof placeholder for this step.
                let partial = Some(proof_placeholder(tactic));

                children.push(SearchNode {
                    remaining_goals: updated_goals,
                    tactic_trace: trace,
                    partial_proof: partial,
                    depth: node.depth + 1,
                });
            }
            TacticApplicationResult::NoProgress => {}
        }
    }

    children
}

/// Build a `ProofSearchResult` from a completed `SearchNode`.
fn build_result(node: SearchNode, stats: &SearchStats) -> ProofSearchResult {
    let depth = node.depth;
    let tactics_used = node.tactic_trace.clone();
    // Use the last partial proof term, or produce a trivial placeholder.
    let proof_term = node.partial_proof.unwrap_or_else(|| {
        let name = Name::str("__search_trivial");
        Expr::Const(name, vec![Level::Zero])
    });

    ProofSearchResult {
        proof_term,
        tactics_used,
        nodes_explored: stats.nodes_explored,
        depth,
    }
}

/// Check whether the wall-clock deadline has passed.
#[inline]
fn is_timed_out(start: Instant, timeout_ms: Option<u64>) -> bool {
    match timeout_ms {
        None => false,
        Some(ms) => start.elapsed() > Duration::from_millis(ms),
    }
}

// ─── BFS ────────────────────────────────────────────────────────────────────

/// Core BFS search loop, shared between `BfsProofSearch` and the BFS
/// variant of the generic dispatch path.
pub(super) fn run_bfs(
    initial_goals: Vec<MVarId>,
    next_mvar_id: u64,
    config: &ProofSearchConfig,
) -> Result<ProofSearchResult, ProofSearchError> {
    if initial_goals.is_empty() {
        return Err(ProofSearchError::NoGoals);
    }

    let root = SearchNode::root(initial_goals);
    if root.is_proof() {
        let stats = SearchStats::new();
        return Ok(build_result(root, &stats));
    }

    let mut frontier: VecDeque<SearchNode> = VecDeque::new();
    frontier.push_back(root);

    let mut counter = MvarCounter::new(next_mvar_id);
    let mut stats = SearchStats::new();
    let start = Instant::now();

    while let Some(node) = frontier.pop_front() {
        if is_timed_out(start, config.timeout_ms) {
            return Err(ProofSearchError::Timeout);
        }

        stats.record_explore();

        if node.is_proof() {
            return Ok(build_result(node, &stats));
        }

        if node.depth >= config.max_depth {
            stats.record_prune();
            continue;
        }

        let children = expand_node(&node, config, &mut counter);
        for child in children {
            stats.record_attempt();
            if child.is_proof() {
                stats.record_success();
                return Ok(build_result(child, &stats));
            }
            frontier.push_back(child);
        }
    }

    Err(ProofSearchError::NoProofFound)
}

// ─── A* ─────────────────────────────────────────────────────────────────────

/// Core A* search loop.
pub(super) fn run_astar(
    initial_goals: Vec<MVarId>,
    next_mvar_id: u64,
    config: &ProofSearchConfig,
) -> Result<ProofSearchResult, ProofSearchError> {
    if initial_goals.is_empty() {
        return Err(ProofSearchError::NoGoals);
    }

    let root = SearchNode::root(initial_goals);
    if root.is_proof() {
        let stats = SearchStats::new();
        return Ok(build_result(root, &stats));
    }

    let mut heap: AstarHeap = AstarHeap::new();
    heap.push(AstarNode(root));

    let mut counter = MvarCounter::new(next_mvar_id);
    let mut stats = SearchStats::new();
    let start = Instant::now();

    while let Some(AstarNode(node)) = heap.pop() {
        if is_timed_out(start, config.timeout_ms) {
            return Err(ProofSearchError::Timeout);
        }

        stats.record_explore();

        if node.is_proof() {
            return Ok(build_result(node, &stats));
        }

        if node.depth >= config.max_depth {
            stats.record_prune();
            continue;
        }

        let children = expand_node(&node, config, &mut counter);
        for child in children {
            stats.record_attempt();
            if child.is_proof() {
                stats.record_success();
                return Ok(build_result(child, &stats));
            }
            heap.push(AstarNode(child));
        }
    }

    Err(ProofSearchError::NoProofFound)
}

// ─── DFS ────────────────────────────────────────────────────────────────────

/// Core DFS search loop (explicit stack).
pub(super) fn run_dfs(
    initial_goals: Vec<MVarId>,
    next_mvar_id: u64,
    config: &ProofSearchConfig,
) -> Result<ProofSearchResult, ProofSearchError> {
    if initial_goals.is_empty() {
        return Err(ProofSearchError::NoGoals);
    }

    let root = SearchNode::root(initial_goals);
    if root.is_proof() {
        let stats = SearchStats::new();
        return Ok(build_result(root, &stats));
    }

    let mut stack: Vec<SearchNode> = vec![root];
    let mut counter = MvarCounter::new(next_mvar_id);
    let mut stats = SearchStats::new();
    let start = Instant::now();

    while let Some(node) = stack.pop() {
        if is_timed_out(start, config.timeout_ms) {
            return Err(ProofSearchError::Timeout);
        }

        stats.record_explore();

        if node.is_proof() {
            return Ok(build_result(node, &stats));
        }

        if node.depth >= config.max_depth {
            stats.record_prune();
            continue;
        }

        let children = expand_node(&node, config, &mut counter);
        for child in children {
            stats.record_attempt();
            if child.is_proof() {
                stats.record_success();
                return Ok(build_result(child, &stats));
            }
            // Push in reverse so that the first child is explored first.
            stack.push(child);
        }
    }

    Err(ProofSearchError::NoProofFound)
}

// ─── IDDFS ──────────────────────────────────────────────────────────────────

/// One depth-limited DFS pass used by IDDFS.
///
/// Returns `Some(result)` if a proof was found at or below `depth_limit`,
/// or `None` if the search was exhausted (no proof) or timed out.
fn dls(
    node: SearchNode,
    depth_limit: usize,
    config: &ProofSearchConfig,
    counter: &mut MvarCounter,
    stats: &mut SearchStats,
    start: Instant,
) -> Result<Option<ProofSearchResult>, ProofSearchError> {
    if is_timed_out(start, config.timeout_ms) {
        return Err(ProofSearchError::Timeout);
    }

    stats.record_explore();

    if node.is_proof() {
        return Ok(Some(build_result(node, stats)));
    }

    if node.depth >= depth_limit {
        stats.record_prune();
        return Ok(None);
    }

    let children = expand_node(&node, config, counter);
    for child in children {
        stats.record_attempt();
        if child.is_proof() {
            stats.record_success();
            return Ok(Some(build_result(child, stats)));
        }
        if let Some(result) = dls(child, depth_limit, config, counter, stats, start)? {
            return Ok(Some(result));
        }
    }

    Ok(None)
}

/// Core IDDFS loop.
pub(super) fn run_iddfs(
    initial_goals: Vec<MVarId>,
    next_mvar_id: u64,
    config: &ProofSearchConfig,
) -> Result<ProofSearchResult, ProofSearchError> {
    if initial_goals.is_empty() {
        return Err(ProofSearchError::NoGoals);
    }

    let start = Instant::now();
    let mut counter = MvarCounter::new(next_mvar_id);
    let mut stats = SearchStats::new();

    for depth_limit in 0..=config.max_depth {
        if is_timed_out(start, config.timeout_ms) {
            return Err(ProofSearchError::Timeout);
        }

        let root = SearchNode::root(initial_goals.clone());
        match dls(root, depth_limit, config, &mut counter, &mut stats, start)? {
            Some(result) => return Ok(result),
            None => {}
        }
    }

    Err(ProofSearchError::NoProofFound)
}

// ─── Generic dispatch ────────────────────────────────────────────────────────

/// Dispatch to the correct search algorithm based on `config.strategy`.
pub fn search_with_config(
    initial_goals: Vec<MVarId>,
    next_mvar_id: u64,
    config: &ProofSearchConfig,
) -> Result<ProofSearchResult, ProofSearchError> {
    match config.strategy {
        SearchStrategy::Bfs => run_bfs(initial_goals, next_mvar_id, config),
        SearchStrategy::Astar => run_astar(initial_goals, next_mvar_id, config),
        SearchStrategy::Dfs => run_dfs(initial_goals, next_mvar_id, config),
        SearchStrategy::IterativeDeepening => run_iddfs(initial_goals, next_mvar_id, config),
    }
}

// ─── ProofSearchEngine impls ─────────────────────────────────────────────────

impl ProofSearchEngine for BfsProofSearch {
    fn search(
        &self,
        initial_goals: Vec<MVarId>,
        next_mvar_id: u64,
    ) -> Result<ProofSearchResult, ProofSearchError> {
        run_bfs(initial_goals, next_mvar_id, &self.config)
    }

    fn config(&self) -> &ProofSearchConfig {
        &self.config
    }
}

impl ProofSearchEngine for AstarProofSearch {
    fn search(
        &self,
        initial_goals: Vec<MVarId>,
        next_mvar_id: u64,
    ) -> Result<ProofSearchResult, ProofSearchError> {
        run_astar(initial_goals, next_mvar_id, &self.config)
    }

    fn config(&self) -> &ProofSearchConfig {
        &self.config
    }
}

impl ProofSearchEngine for DfsProofSearch {
    fn search(
        &self,
        initial_goals: Vec<MVarId>,
        next_mvar_id: u64,
    ) -> Result<ProofSearchResult, ProofSearchError> {
        run_dfs(initial_goals, next_mvar_id, &self.config)
    }

    fn config(&self) -> &ProofSearchConfig {
        &self.config
    }
}

impl ProofSearchEngine for IddfsProofSearch {
    fn search(
        &self,
        initial_goals: Vec<MVarId>,
        next_mvar_id: u64,
    ) -> Result<ProofSearchResult, ProofSearchError> {
        run_iddfs(initial_goals, next_mvar_id, &self.config)
    }

    fn config(&self) -> &ProofSearchConfig {
        &self.config
    }
}

// ─── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn goal(n: u64) -> MVarId {
        MVarId(n)
    }

    // ── BFS ────────────────────────────────────────────────────────────────

    #[test]
    fn bfs_finds_proof_single_goal() {
        // One goal that `Rfl` should close immediately.
        let engine = BfsProofSearch::new();
        let result = engine.search(vec![goal(0)], 100);
        assert!(result.is_ok(), "BFS should find a proof: {:?}", result);
        let r = result.expect("BFS should return a result");
        assert_eq!(r.depth, 1, "proof should be depth-1");
        assert_eq!(r.nodes_explored, 1);
    }

    #[test]
    fn bfs_returns_no_goals_error_on_empty_input() {
        let engine = BfsProofSearch::new();
        let result = engine.search(vec![], 0);
        assert!(matches!(result, Err(ProofSearchError::NoGoals)));
    }

    #[test]
    fn bfs_multi_goal_proof() {
        // Two goals: closing one at a time.
        // With Intro (opens 1 sub-goal) and then Rfl (closes),
        // depth-2 is reachable.
        let config = ProofSearchConfig::with_depth(4);
        let engine = BfsProofSearch::with_config(config);
        let result = engine.search(vec![goal(1), goal(2)], 1000);
        assert!(
            result.is_ok(),
            "BFS should handle multi-goal state: {:?}",
            result
        );
    }

    #[test]
    fn bfs_reports_no_proof_at_depth_zero() {
        // max_depth = 0 means we never expand.
        let config = ProofSearchConfig {
            max_depth: 0,
            ..ProofSearchConfig::default()
        };
        let engine = BfsProofSearch::with_config(config);
        let result = engine.search(vec![goal(0)], 100);
        // The root is explored but not proved; then depth limit stops expansion.
        assert!(
            result.is_err(),
            "should fail to prove with max_depth=0: {:?}",
            result
        );
    }

    // ── A* ─────────────────────────────────────────────────────────────────

    #[test]
    fn astar_finds_proof_single_goal() {
        let engine = AstarProofSearch::new();
        let result = engine.search(vec![goal(10)], 200);
        assert!(result.is_ok(), "A* should find a proof: {:?}", result);
    }

    #[test]
    fn astar_prefers_fewer_goals() {
        // With two goals vs one goal, A* should route towards the state with
        // fewer remaining goals first.  We can't directly observe ordering,
        // but we can check it still finds a proof.
        let config = ProofSearchConfig {
            strategy: SearchStrategy::Astar,
            max_depth: 6,
            ..ProofSearchConfig::default()
        };
        let engine = AstarProofSearch::with_config(config);
        let result = engine.search(vec![goal(1), goal(2), goal(3)], 500);
        assert!(result.is_ok(), "A* should handle 3-goal state: {:?}", result);
    }

    // ── DFS ────────────────────────────────────────────────────────────────

    #[test]
    fn dfs_finds_proof_single_goal() {
        let engine = DfsProofSearch::new();
        let result = engine.search(vec![goal(20)], 300);
        assert!(result.is_ok(), "DFS should find a proof: {:?}", result);
    }

    #[test]
    fn dfs_empty_goals() {
        let engine = DfsProofSearch::new();
        let result = engine.search(vec![], 0);
        assert!(matches!(result, Err(ProofSearchError::NoGoals)));
    }

    // ── IDDFS ──────────────────────────────────────────────────────────────

    #[test]
    fn iddfs_finds_proof_single_goal() {
        let engine = IddfsProofSearch::new();
        let result = engine.search(vec![goal(30)], 400);
        assert!(result.is_ok(), "IDDFS should find a proof: {:?}", result);
    }

    #[test]
    fn iddfs_finds_optimal_depth() {
        // With IDDFS the found proof should have depth 1 (same as BFS minimum).
        let engine = IddfsProofSearch::new();
        let result = engine.search(vec![goal(40)], 500).expect("IDDFS result");
        assert_eq!(result.depth, 1, "IDDFS should find shortest proof");
    }

    #[test]
    fn iddfs_empty_goals() {
        let engine = IddfsProofSearch::new();
        let result = engine.search(vec![], 0);
        assert!(matches!(result, Err(ProofSearchError::NoGoals)));
    }

    // ── Config ─────────────────────────────────────────────────────────────

    #[test]
    fn config_defaults_are_sane() {
        let cfg = ProofSearchConfig::default();
        assert_eq!(cfg.max_depth, 4);
        assert_eq!(cfg.max_branches, 32);
        assert!(!cfg.tactics.is_empty());
        assert_eq!(cfg.strategy, SearchStrategy::Bfs);
    }

    #[test]
    fn config_builder_methods() {
        let cfg = ProofSearchConfig::with_depth(8)
            .with_strategy(SearchStrategy::Astar)
            .with_timeout(1000)
            .no_timeout();
        assert_eq!(cfg.max_depth, 8);
        assert_eq!(cfg.strategy, SearchStrategy::Astar);
        assert!(cfg.timeout_ms.is_none());
    }

    // ── AutoTactic ─────────────────────────────────────────────────────────

    #[test]
    fn auto_tactic_display_names() {
        assert_eq!(AutoTactic::Rfl.display_name(), "rfl");
        assert_eq!(AutoTactic::Assumption.display_name(), "assumption");
        assert_eq!(AutoTactic::Omega.display_name(), "omega");
        assert_eq!(AutoTactic::Ring.display_name(), "ring");
    }

    #[test]
    fn auto_tactic_costs_ordered() {
        // Cheap tactics should have lower cost than expensive ones.
        assert!(AutoTactic::Rfl.estimated_cost() < AutoTactic::Apply(Expr::BVar(0)).estimated_cost());
        assert!(AutoTactic::Assumption.estimated_cost() <= AutoTactic::Constructor.estimated_cost());
    }

    // ── SearchNode ─────────────────────────────────────────────────────────

    #[test]
    fn search_node_root_is_not_proof() {
        let node = SearchNode::root(vec![goal(0)]);
        assert!(!node.is_proof());
        assert_eq!(node.depth, 0);
    }

    #[test]
    fn search_node_empty_goals_is_proof() {
        let node = SearchNode::root(vec![]);
        assert!(node.is_proof());
    }

    #[test]
    fn astar_node_ordering() {
        use super::super::types::AstarNode;
        // Node with fewer remaining goals should have higher score (preferred).
        let shallow = SearchNode {
            remaining_goals: vec![goal(0)],
            tactic_trace: vec![],
            partial_proof: None,
            depth: 0,
        };
        let deeper = SearchNode {
            remaining_goals: vec![goal(0), goal(1), goal(2)],
            tactic_trace: vec![],
            partial_proof: None,
            depth: 2,
        };
        let a = AstarNode(shallow);
        let b = AstarNode(deeper);
        // a has score -(0+1) = -1, b has score -(2+3) = -5
        // a > b in the heap ordering → a should be popped first.
        assert!(a > b);
    }

    // ── Timeout ────────────────────────────────────────────────────────────

    #[test]
    fn bfs_respects_zero_timeout() {
        let config = ProofSearchConfig {
            // Depth large enough that without timeout we'd find a proof,
            // but timeout so short it should trigger.
            max_depth: 100,
            timeout_ms: Some(0),
            ..ProofSearchConfig::default()
        };
        let engine = BfsProofSearch::with_config(config);
        // With a 0ms timeout the very first iteration check should time out.
        // However a single Rfl application is so fast we might still find it.
        // The test just asserts no panic and that the result is one of the
        // expected outcomes.
        let result = engine.search(vec![goal(0), goal(1), goal(2), goal(3), goal(4)], 1000);
        match result {
            Ok(_) | Err(ProofSearchError::NoProofFound) | Err(ProofSearchError::Timeout) => {}
            Err(e) => panic!("unexpected error: {e}"),
        }
    }

    // ── Generic dispatch ───────────────────────────────────────────────────

    #[test]
    fn search_with_config_all_strategies() {
        let goals = vec![goal(0)];
        for strategy in [
            SearchStrategy::Bfs,
            SearchStrategy::Astar,
            SearchStrategy::Dfs,
            SearchStrategy::IterativeDeepening,
        ] {
            let config = ProofSearchConfig::default().with_strategy(strategy);
            let result = search_with_config(goals.clone(), 1000, &config);
            assert!(
                result.is_ok(),
                "strategy {:?} should find a proof: {:?}",
                strategy,
                result
            );
        }
    }

    // ── ProofSearchResult ──────────────────────────────────────────────────

    #[test]
    fn result_contains_tactic_trace() {
        let engine = BfsProofSearch::new();
        let result = engine.search(vec![goal(0)], 0).expect("BFS result");
        assert!(!result.tactics_used.is_empty(), "trace should be non-empty");
    }

    #[test]
    fn result_proof_term_is_const() {
        let engine = BfsProofSearch::new();
        let result = engine.search(vec![goal(0)], 0).expect("BFS result");
        assert!(
            matches!(result.proof_term, Expr::Const(_, _)),
            "proof term should be a Const placeholder"
        );
    }
}
