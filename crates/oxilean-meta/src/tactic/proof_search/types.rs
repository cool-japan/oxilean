//! Types for proof search automation.
//!
//! Defines the data structures used by BFS, A*, DFS, and IDDFS proof search engines.

use std::cmp::Ordering;
use std::collections::BinaryHeap;

use crate::basic::MVarId;
use oxilean_kernel::Expr;

// ─── AutoTactic ────────────────────────────────────────────────────────────

/// An atomic tactic that a proof search engine may attempt.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum AutoTactic {
    /// Close the goal by reflexivity (`rfl`).
    Rfl,
    /// Close the goal by scanning local hypotheses (`assumption`).
    Assumption,
    /// Discharge trivial propositional goals (`trivial`).
    Trivial,
    /// Derive `False` from contradictory hypotheses (`contradiction`).
    Contradiction,
    /// Apply the first matching constructor of the goal's inductive type (`constructor`).
    Constructor,
    /// Introduce the next binder in a Pi goal (`intro`).
    Intro,
    /// Apply an arbitrary expression to the goal (`apply e`).
    Apply(Expr),
    /// Close the goal with an exact term (`exact e`).
    Exact(Expr),
    /// Simplify the goal using the simp lemma set (`simp`).
    Simp,
    /// Discharge linear-arithmetic goals over integers/naturals (`omega`).
    Omega,
    /// Discharge commutative-ring equalities (`ring`).
    Ring,
}

impl AutoTactic {
    /// Return a display name for the tactic, for tracing.
    pub fn display_name(&self) -> String {
        match self {
            AutoTactic::Rfl => "rfl".to_string(),
            AutoTactic::Assumption => "assumption".to_string(),
            AutoTactic::Trivial => "trivial".to_string(),
            AutoTactic::Contradiction => "contradiction".to_string(),
            AutoTactic::Constructor => "constructor".to_string(),
            AutoTactic::Intro => "intro".to_string(),
            AutoTactic::Apply(_) => "apply(...)".to_string(),
            AutoTactic::Exact(_) => "exact(...)".to_string(),
            AutoTactic::Simp => "simp".to_string(),
            AutoTactic::Omega => "omega".to_string(),
            AutoTactic::Ring => "ring".to_string(),
        }
    }

    /// Estimated cost of applying this tactic (lower = prefer trying first).
    pub fn estimated_cost(&self) -> u32 {
        match self {
            AutoTactic::Rfl => 1,
            AutoTactic::Assumption => 1,
            AutoTactic::Trivial => 2,
            AutoTactic::Contradiction => 2,
            AutoTactic::Exact(_) => 2,
            AutoTactic::Simp => 3,
            AutoTactic::Omega => 3,
            AutoTactic::Ring => 3,
            AutoTactic::Constructor => 4,
            AutoTactic::Intro => 4,
            AutoTactic::Apply(_) => 5,
        }
    }
}

// ─── SearchStrategy ─────────────────────────────────────────────────────────

/// The search algorithm to use.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SearchStrategy {
    /// Breadth-first search (BFS) — complete, but may use significant memory.
    Bfs,
    /// A* search — heuristic-guided, finds shorter proofs faster on typical goals.
    Astar,
    /// Depth-first search (DFS) — low memory, may explore deeply before finding a proof.
    Dfs,
    /// Iterative deepening depth-first search (IDDFS) — optimal depth, low memory.
    IterativeDeepening,
}

// ─── ProofSearchConfig ──────────────────────────────────────────────────────

/// Configuration for proof search engines.
#[derive(Clone, Debug)]
pub struct ProofSearchConfig {
    /// Maximum depth to explore. Search terminates at this depth.
    pub max_depth: usize,
    /// Maximum number of successor states to generate per node.
    pub max_branches: usize,
    /// Wall-clock timeout in milliseconds.  `None` means no timeout.
    pub timeout_ms: Option<u64>,
    /// Ordered list of tactics to try at each state.
    pub tactics: Vec<AutoTactic>,
    /// Which search algorithm to use.
    pub strategy: SearchStrategy,
}

impl Default for ProofSearchConfig {
    fn default() -> Self {
        Self {
            max_depth: 4,
            max_branches: 32,
            timeout_ms: Some(5_000),
            tactics: vec![
                AutoTactic::Rfl,
                AutoTactic::Assumption,
                AutoTactic::Trivial,
                AutoTactic::Contradiction,
                AutoTactic::Simp,
                AutoTactic::Omega,
                AutoTactic::Ring,
                AutoTactic::Constructor,
                AutoTactic::Intro,
            ],
            strategy: SearchStrategy::Bfs,
        }
    }
}

impl ProofSearchConfig {
    /// Construct a configuration with all defaults overridden by `max_depth`.
    pub fn with_depth(max_depth: usize) -> Self {
        Self {
            max_depth,
            ..Self::default()
        }
    }

    /// Set the strategy.
    pub fn with_strategy(mut self, strategy: SearchStrategy) -> Self {
        self.strategy = strategy;
        self
    }

    /// Replace the tactic list entirely.
    pub fn with_tactics(mut self, tactics: Vec<AutoTactic>) -> Self {
        self.tactics = tactics;
        self
    }

    /// Set the wall-clock timeout.
    pub fn with_timeout(mut self, ms: u64) -> Self {
        self.timeout_ms = Some(ms);
        self
    }

    /// Disable the timeout.
    pub fn no_timeout(mut self) -> Self {
        self.timeout_ms = None;
        self
    }
}

// ─── ProofSearchResult ──────────────────────────────────────────────────────

/// The successful output of a proof search.
#[derive(Clone, Debug)]
pub struct ProofSearchResult {
    /// The proof term produced.
    pub proof_term: Expr,
    /// Human-readable trace of each tactic that was applied, in order.
    pub tactics_used: Vec<String>,
    /// Total number of search nodes that were visited.
    pub nodes_explored: usize,
    /// Depth at which the proof was found (0 = immediate, n = n tactics applied).
    pub depth: usize,
}

// ─── ProofSearchError ───────────────────────────────────────────────────────

/// Reasons why proof search may fail.
#[derive(Clone, Debug)]
pub enum ProofSearchError {
    /// Search exhausted the configured depth without finding a proof.
    NoProofFound,
    /// Search was aborted because the timeout elapsed.
    Timeout,
    /// The initial proof state has no goals — nothing to prove.
    NoGoals,
    /// An internal error occurred (e.g. a tactic panicked).
    Internal(String),
}

impl std::fmt::Display for ProofSearchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProofSearchError::NoProofFound => write!(f, "no proof found within search limits"),
            ProofSearchError::Timeout => write!(f, "proof search timed out"),
            ProofSearchError::NoGoals => write!(f, "proof state has no goals"),
            ProofSearchError::Internal(msg) => write!(f, "internal error: {msg}"),
        }
    }
}

// ─── SearchNode ─────────────────────────────────────────────────────────────

/// A node in the proof search tree.
///
/// Each node records the partial proof state that must still be discharged,
/// the sequence of tactics applied to reach it, and the proof term accumulated so far.
#[derive(Clone, Debug)]
pub struct SearchNode {
    /// Remaining open goals in this node's proof state.
    pub remaining_goals: Vec<MVarId>,
    /// Tactics applied from the root to reach this node.
    pub tactic_trace: Vec<String>,
    /// Partial proof term built so far.
    pub partial_proof: Option<Expr>,
    /// Depth of this node (= number of tactics applied so far).
    pub depth: usize,
}

impl SearchNode {
    /// Create the root search node.
    pub fn root(initial_goals: Vec<MVarId>) -> Self {
        Self {
            remaining_goals: initial_goals,
            tactic_trace: Vec::new(),
            partial_proof: None,
            depth: 0,
        }
    }

    /// Check if this node represents a completed proof.
    pub fn is_proof(&self) -> bool {
        self.remaining_goals.is_empty()
    }

    /// Estimate the heuristic cost for A* ordering.
    ///
    /// Heuristic = `g + h`:
    /// - `g` = depth (cost to reach this node, uniform cost 1 per tactic)
    /// - `h` = remaining goals × avg-goal-complexity weight
    ///
    /// We deliberately keep the heuristic admissible by treating
    /// each remaining goal as needing at least one more tactic.
    pub fn astar_score(&self) -> i64 {
        let g = self.depth as i64;
        // Each remaining goal requires at least one tactic → h ≥ goal_count.
        // We add a complexity multiplier derived from the goal count itself
        // as a simple stand-in for per-goal complexity (monotone, admissible).
        let h = self.remaining_goals.len() as i64;
        // A* uses lowest f = g + h first; we negate for a max-heap.
        -(g + h)
    }
}

// ─── AstarNode ──────────────────────────────────────────────────────────────

/// Wrapper that makes `SearchNode` usable in `BinaryHeap` (max-heap → min-f).
#[derive(Clone, Debug)]
pub struct AstarNode(pub SearchNode);

impl PartialEq for AstarNode {
    fn eq(&self, other: &Self) -> bool {
        self.0.astar_score() == other.0.astar_score()
    }
}

impl Eq for AstarNode {}

impl PartialOrd for AstarNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for AstarNode {
    fn cmp(&self, other: &Self) -> Ordering {
        // Higher score = better (less negation) → put first in max-heap.
        self.0.astar_score().cmp(&other.0.astar_score())
    }
}

// ─── SearchStats ────────────────────────────────────────────────────────────

/// Internal mutable statistics gathered during a search.
#[derive(Clone, Debug, Default)]
pub struct SearchStats {
    /// Total nodes popped from the frontier.
    pub nodes_explored: usize,
    /// Nodes skipped due to depth limit.
    pub nodes_pruned: usize,
    /// Number of tactic applications attempted.
    pub tactic_attempts: usize,
    /// Number of tactic applications that succeeded (state changed).
    pub tactic_successes: usize,
}

impl SearchStats {
    /// Create a zeroed stats record.
    pub fn new() -> Self {
        Self::default()
    }

    /// Record that one node was visited.
    pub fn record_explore(&mut self) {
        self.nodes_explored += 1;
    }

    /// Record that one node was skipped.
    pub fn record_prune(&mut self) {
        self.nodes_pruned += 1;
    }

    /// Record one tactic attempt (whether or not it succeeded).
    pub fn record_attempt(&mut self) {
        self.tactic_attempts += 1;
    }

    /// Record one tactic success.
    pub fn record_success(&mut self) {
        self.tactic_successes += 1;
    }
}

// ─── TacticOutcome (local simulation) ───────────────────────────────────────

/// Result of applying an `AutoTactic` to a proof state node.
///
/// During proof search we do not own a live `MetaContext`, so we simulate
/// tactic outcomes using a *goal-manipulation model*:
///
/// - Goal-closing tactics (`Rfl`, `Assumption`, `Trivial`, `Contradiction`,
///   `Simp`, `Omega`, `Ring`, `Exact`) close the focused goal (if the
///   simulated precondition is met), producing zero new goals.
/// - `Constructor` / `Intro` may open additional goals.
/// - `Apply` closes the focused goal and may open zero or more sub-goals.
///
/// This is a *sound approximation* — a real implementation would thread the
/// `MetaContext` through; here we model search topology precisely and leave
/// `proof_term` generation to the caller who builds the final term from the trace.
#[derive(Clone, Debug)]
pub enum TacticApplicationResult {
    /// The tactic closed at least one goal; `new_goals` are the freshly
    /// opened sub-goals that replace the focused goal.
    Progress {
        /// Goals that replaced the focused goal (may be empty = proof found).
        new_goals: Vec<MVarId>,
        /// A human-readable description of what happened.
        description: String,
    },
    /// The tactic made no progress (precondition not met, goal does not match, …).
    NoProgress,
}

// ─── BfsProofSearch ─────────────────────────────────────────────────────────

/// Breadth-first proof search.
///
/// Explores the proof state graph layer-by-layer (by depth). This guarantees
/// finding the *shortest* proof (in terms of number of tactics applied) within
/// the configured depth limit.
///
/// # Algorithm
///
/// ```text
/// frontier: VecDeque<SearchNode>  (FIFO)
/// push root
/// while frontier not empty:
///     node = pop_front
///     if node.is_proof(): return Ok(node)
///     if node.depth >= max_depth: continue
///     for tactic in config.tactics (up to max_branches):
///         child = apply(tactic, node)
///         if progress: push_back(child)
/// return Err(NoProofFound)
/// ```
pub struct BfsProofSearch {
    /// Configuration controlling depth, branching, timeout, and tactics.
    pub config: ProofSearchConfig,
}

impl BfsProofSearch {
    /// Create a BFS searcher with default configuration.
    pub fn new() -> Self {
        Self {
            config: ProofSearchConfig::default(),
        }
    }

    /// Create a BFS searcher with an explicit configuration.
    pub fn with_config(config: ProofSearchConfig) -> Self {
        Self { config }
    }
}

impl Default for BfsProofSearch {
    fn default() -> Self {
        Self::new()
    }
}

// ─── AstarProofSearch ───────────────────────────────────────────────────────

/// A* heuristic proof search.
///
/// Uses a priority queue ordered by `f = g + h` where:
/// - `g` = depth of node
/// - `h` = number of remaining goals (admissible, consistent heuristic)
///
/// In practice this finds shorter proofs faster than BFS on goals with many sub-goals,
/// while still being complete within the depth bound.
pub struct AstarProofSearch {
    /// Configuration controlling depth, branching, timeout, and tactics.
    pub config: ProofSearchConfig,
}

impl AstarProofSearch {
    /// Create an A* searcher with default configuration.
    pub fn new() -> Self {
        Self {
            config: ProofSearchConfig {
                strategy: SearchStrategy::Astar,
                ..ProofSearchConfig::default()
            },
        }
    }

    /// Create an A* searcher with an explicit configuration.
    pub fn with_config(config: ProofSearchConfig) -> Self {
        Self { config }
    }
}

impl Default for AstarProofSearch {
    fn default() -> Self {
        Self::new()
    }
}

// ─── DfsProofSearch ─────────────────────────────────────────────────────────

/// Depth-first proof search.
///
/// Explores the proof state graph using a stack (LIFO). Minimal memory usage,
/// but not guaranteed to find the shortest proof.
pub struct DfsProofSearch {
    /// Configuration controlling depth, branching, timeout, and tactics.
    pub config: ProofSearchConfig,
}

impl DfsProofSearch {
    /// Create a DFS searcher with default configuration.
    pub fn new() -> Self {
        Self {
            config: ProofSearchConfig {
                strategy: SearchStrategy::Dfs,
                ..ProofSearchConfig::default()
            },
        }
    }

    /// Create a DFS searcher with an explicit configuration.
    pub fn with_config(config: ProofSearchConfig) -> Self {
        Self { config }
    }
}

impl Default for DfsProofSearch {
    fn default() -> Self {
        Self::new()
    }
}

// ─── IddfsProofSearch ───────────────────────────────────────────────────────

/// Iterative-deepening depth-first search (IDDFS).
///
/// Runs DFS with increasing depth bounds 0, 1, 2, …, `max_depth`.
/// This combines the space efficiency of DFS with the optimality
/// (shortest-proof) guarantee of BFS.
pub struct IddfsProofSearch {
    /// Configuration controlling depth, branching, timeout, and tactics.
    pub config: ProofSearchConfig,
}

impl IddfsProofSearch {
    /// Create an IDDFS searcher with default configuration.
    pub fn new() -> Self {
        Self {
            config: ProofSearchConfig {
                strategy: SearchStrategy::IterativeDeepening,
                ..ProofSearchConfig::default()
            },
        }
    }

    /// Create an IDDFS searcher with an explicit configuration.
    pub fn with_config(config: ProofSearchConfig) -> Self {
        Self { config }
    }
}

impl Default for IddfsProofSearch {
    fn default() -> Self {
        Self::new()
    }
}

// ─── ProofSearchEngine trait ─────────────────────────────────────────────────

/// Common interface implemented by all proof search engines.
pub trait ProofSearchEngine {
    /// Run proof search starting from the given open goals.
    ///
    /// Returns `Ok(ProofSearchResult)` on success or `Err(ProofSearchError)` on failure.
    fn search(
        &self,
        initial_goals: Vec<MVarId>,
        next_mvar_id: u64,
    ) -> Result<ProofSearchResult, ProofSearchError>;

    /// Return a reference to the engine's configuration.
    fn config(&self) -> &ProofSearchConfig;
}

// ─── Re-exports ──────────────────────────────────────────────────────────────

/// Convenience type alias: the BinaryHeap used by A* search.
pub type AstarHeap = BinaryHeap<AstarNode>;
