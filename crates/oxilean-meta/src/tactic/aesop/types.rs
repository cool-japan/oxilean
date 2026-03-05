//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::basic::{MVarId, MetaState};
use crate::discr_tree::DiscrTree;
use crate::tactic::state::{TacticError, TacticResult};
use oxilean_kernel::{Expr, Name};
use std::collections::{BinaryHeap, HashMap};
use std::time::Instant;

use super::functions::RuleTacticFn;

/// Transparency mode controlling which definitions the search may unfold.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TransparencyMode {
    /// Only unfold definitions marked `@[reducible]`.
    Reducible,
    /// Unfold all non-opaque definitions (default for most tactics).
    Default,
    /// Unfold everything, including opaque definitions.
    All,
}
/// Safety level of an aesop rule.
///
/// Safety determines how eagerly a rule is applied during search:
/// - **Safe** rules are applied unconditionally — they never cause backtracking.
/// - **AlmostSafe** rules are applied eagerly but create a lightweight checkpoint
///   so they can be undone if they lead to a dead end.
/// - **Unsafe** rules create full branch points and are explored only when the
///   priority queue selects them.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum AesopRuleSafety {
    /// Always applied; never causes backtracking.
    Safe,
    /// Applied eagerly but may be undone.
    AlmostSafe,
    /// Creates a branch point.
    Unsafe,
}
impl AesopRuleSafety {
    /// Returns `true` for `Safe` or `AlmostSafe`.
    pub fn is_safe_or_almost(self) -> bool {
        matches!(self, AesopRuleSafety::Safe | AesopRuleSafety::AlmostSafe)
    }
}
/// The overall state of an aesop proof search.
///
/// This struct owns the search tree, the priority queue, the proof cache,
/// and references the rule set and configuration. It is the main driver
/// for the best-first search algorithm.
pub struct AesopSearchState {
    /// Configuration controlling search limits.
    pub(super) config: AesopConfig,
    /// The flat array of search-tree nodes.
    pub(super) nodes: Vec<AesopSearchNode>,
    /// The best-first priority queue.
    pub(super) queue: BinaryHeap<PQEntry>,
    /// Proof cache for memoization.
    pub(super) cache: ProofCache,
    /// Accumulated statistics.
    pub(super) stats: AesopStats,
    /// Start time of the search.
    pub(super) start_time: Instant,
    /// Whether the search has concluded.
    pub(super) finished: bool,
    /// If the search was successful, the id of the solving leaf node.
    pub(super) solution_node: Option<NodeId>,
}
impl AesopSearchState {
    /// Create a new search state with the given configuration and initial goals.
    pub fn new(config: AesopConfig, initial_goals: Vec<MVarId>) -> Self {
        let root = AesopSearchNode::root(initial_goals);
        let root_entry = PQEntry {
            node_id: root.id,
            cost: 0.0,
        };
        let mut queue = BinaryHeap::new();
        queue.push(root_entry);
        Self {
            config,
            nodes: vec![root],
            queue,
            cache: ProofCache::new(),
            stats: AesopStats::default(),
            start_time: Instant::now(),
            finished: false,
            solution_node: None,
        }
    }
    /// Allocate a new node and return its id.
    pub(super) fn alloc_node(&mut self, node: AesopSearchNode) -> NodeId {
        let id = NodeId(self.nodes.len());
        let mut node = node;
        let depth = node.depth;
        node.id = id;
        self.nodes.push(node);
        self.stats.nodes_created += 1;
        if depth > self.stats.max_depth_reached {
            self.stats.max_depth_reached = depth;
        }
        id
    }
    /// Check whether a resource limit has been reached.
    pub(super) fn check_limits(&self) -> Option<String> {
        if self.stats.nodes_expanded >= self.config.max_iters {
            return Some(format!(
                "iteration limit ({}) exceeded",
                self.config.max_iters
            ));
        }
        if self.stats.rules_tried >= self.config.max_rule_apps {
            return Some(format!(
                "rule application limit ({}) exceeded",
                self.config.max_rule_apps
            ));
        }
        if self.config.timeout_ms > 0 {
            let elapsed = self.start_time.elapsed().as_millis() as u64;
            if elapsed > self.config.timeout_ms {
                return Some(format!("timeout ({}ms) exceeded", self.config.timeout_ms));
            }
        }
        None
    }
    /// Get the current statistics.
    pub fn current_stats(&self) -> &AesopStats {
        &self.stats
    }
    /// Returns `true` if the search has finished (success or failure).
    pub fn is_finished(&self) -> bool {
        self.finished
    }
    /// Get the number of nodes in the tree.
    pub fn num_nodes(&self) -> usize {
        self.nodes.len()
    }
}
/// Statistics collected during an aesop search.
#[derive(Clone, Debug, Default)]
pub struct AesopStats {
    /// Total number of search-tree nodes created.
    pub nodes_created: usize,
    /// Number of nodes expanded (popped from the priority queue).
    pub nodes_expanded: usize,
    /// Total number of rule application attempts.
    pub rules_tried: usize,
    /// Number of successful rule applications.
    pub rules_succeeded: usize,
    /// Number of normalization passes.
    pub norm_passes: usize,
    /// Number of cache hits.
    pub cache_hits: usize,
    /// Number of cache misses.
    pub cache_misses: usize,
    /// Maximum search depth reached.
    pub max_depth_reached: usize,
    /// Total wall-clock time in microseconds.
    pub time_us: u64,
    /// Number of backtrack events.
    pub backtracks: usize,
}
/// The result of an aesop search.
#[derive(Debug)]
pub enum AesopResult {
    /// The search found a proof.
    Success {
        /// The proof term.
        proof: Expr,
        /// Statistics (populated if `collect_stats` was enabled).
        stats: AesopStats,
    },
    /// The search exhausted all possibilities without finding a proof.
    Failure {
        /// Human-readable explanation.
        reason: String,
        /// Statistics.
        stats: AesopStats,
    },
    /// The search exceeded a resource limit.
    ResourceLimit {
        /// Which limit was exceeded.
        limit_kind: String,
        /// Statistics.
        stats: AesopStats,
    },
}
impl AesopResult {
    /// Returns `true` if the search succeeded.
    pub fn is_success(&self) -> bool {
        matches!(self, AesopResult::Success { .. })
    }
    /// Returns the proof if successful.
    pub fn proof(&self) -> Option<&Expr> {
        match self {
            AesopResult::Success { proof, .. } => Some(proof),
            _ => None,
        }
    }
    /// Returns statistics regardless of outcome.
    pub fn stats(&self) -> &AesopStats {
        match self {
            AesopResult::Success { stats, .. } => stats,
            AesopResult::Failure { stats, .. } => stats,
            AesopResult::ResourceLimit { stats, .. } => stats,
        }
    }
}
/// Entry in the best-first search priority queue.
///
/// Ordering is by *lowest cost first* (min-heap behaviour via `Reverse`-style
/// `Ord` implementation).
#[derive(Debug, Clone)]
pub(super) struct PQEntry {
    /// The search-tree node to expand.
    pub(super) node_id: NodeId,
    /// Effective cost (lower is better).
    pub(super) cost: f64,
}
/// A single node in the aesop search tree.
///
/// Each node represents a proof state obtained after applying a rule to
/// a parent node's goal. Leaf nodes with no remaining goals are solved.
#[derive(Debug)]
pub struct AesopSearchNode {
    /// Unique identifier.
    pub id: NodeId,
    /// Parent node, or `None` for the root.
    pub parent: Option<NodeId>,
    /// Children created by expanding this node.
    pub children: Vec<NodeId>,
    /// Depth in the search tree (root = 0).
    pub depth: usize,
    /// Rule that was applied to produce this node (None for root).
    pub applied_rule: Option<RuleId>,
    /// Name of the applied rule (for diagnostics).
    pub applied_rule_name: Option<Name>,
    /// Goal metavariable ids at this node.
    pub goals: Vec<MVarId>,
    /// Effective cost used for priority queue ordering.
    pub cost: f64,
    /// Current status.
    pub status: NodeStatus,
    /// Saved meta context state for backtracking.
    pub saved_meta: Option<MetaState>,
    /// Proof term fragment constructed at this node.
    pub proof_fragment: Option<Expr>,
    /// Number of rule applications attempted from this node.
    pub rule_apps_tried: usize,
}
impl AesopSearchNode {
    /// Create a new root node.
    pub(crate) fn root(goals: Vec<MVarId>) -> Self {
        Self {
            id: NodeId(0),
            parent: None,
            children: Vec::new(),
            depth: 0,
            applied_rule: None,
            applied_rule_name: None,
            goals,
            cost: 0.0,
            status: NodeStatus::Open,
            saved_meta: None,
            proof_fragment: None,
            rule_apps_tried: 0,
        }
    }
    /// Create a child node produced by applying a rule.
    #[allow(clippy::too_many_arguments)]
    pub(super) fn child(
        id: NodeId,
        parent: NodeId,
        depth: usize,
        rule_id: RuleId,
        rule_name: Name,
        goals: Vec<MVarId>,
        cost: f64,
        saved_meta: MetaState,
    ) -> Self {
        Self {
            id,
            parent: Some(parent),
            children: Vec::new(),
            depth,
            applied_rule: Some(rule_id),
            applied_rule_name: Some(rule_name),
            goals,
            cost,
            status: NodeStatus::Open,
            saved_meta: Some(saved_meta),
            proof_fragment: None,
            rule_apps_tried: 0,
        }
    }
    /// Returns `true` if the node has no remaining goals.
    pub fn is_solved(&self) -> bool {
        self.goals.is_empty() || self.status == NodeStatus::Solved
    }
    /// Returns `true` if the node is a dead end.
    pub fn is_dead(&self) -> bool {
        self.status == NodeStatus::Failed || self.status == NodeStatus::Pruned
    }
}
/// Key for the proof cache — based on the goal target expression.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct CacheKey {
    pub(super) target: Expr,
}
/// Lock-free proof cache backed by a `HashMap`.
///
/// Provides memoization: if a goal with the same target type has been solved
/// before, the cached proof can be reused directly.
#[derive(Clone, Debug, Default)]
pub struct ProofCache {
    /// Internal storage.
    pub(super) entries: HashMap<CacheKey, CacheEntry>,
}
impl ProofCache {
    /// Create a new empty cache.
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }
    /// Look up a cached proof for the given target expression.
    pub fn lookup(&self, target: &Expr) -> Option<&Expr> {
        let key = CacheKey {
            target: target.clone(),
        };
        self.entries.get(&key).map(|e| &e.proof)
    }
    /// Insert a proof for the given target expression.
    pub fn insert(&mut self, target: Expr, proof: Expr, rule_apps: usize) {
        let key = CacheKey { target };
        self.entries.insert(key, CacheEntry { proof, rule_apps });
    }
    /// Number of cached entries.
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    /// Returns `true` if empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
    /// Clear all cached entries.
    pub fn clear(&mut self) {
        self.entries.clear();
    }
}
/// Status of a search node.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NodeStatus {
    /// Not yet expanded.
    Open,
    /// Expanded — children have been created.
    Expanded,
    /// This node (and its subtree) proved the goal.
    Solved,
    /// This node is a dead end.
    Failed,
    /// Pruned due to depth or iteration limits.
    Pruned,
}
/// A single rule in the aesop rule database.
///
/// Each rule has a name for identification, a priority for ordering, a safety
/// level, a kind describing what transformation it performs, and a tactic
/// closure that carries out the transformation.
pub struct AesopRule {
    /// Human-readable name used for diagnostics and deduplication.
    pub name: Name,
    /// The tactic closure executed when the rule fires.
    pub tactic: RuleTacticFn,
    /// Safety level controlling eagerness of application.
    pub safety: AesopRuleSafety,
    /// The kind of transformation this rule performs.
    pub kind: AesopRuleKind,
    /// Priority — lower values are tried first. Must be non-negative.
    pub priority: u32,
    /// Optional pattern expression used for discrimination-tree indexing.
    /// When `None` the rule is treated as a wildcard and considered for every goal.
    pub index_pattern: Option<Expr>,
    /// Number of subgoals this rule typically produces (heuristic).
    pub estimated_subgoals: u32,
}
impl AesopRule {
    /// Create a new rule with the given parameters.
    pub fn new(
        name: Name,
        tactic: RuleTacticFn,
        safety: AesopRuleSafety,
        kind: AesopRuleKind,
        priority: u32,
    ) -> Self {
        Self {
            name,
            tactic,
            safety,
            kind,
            priority,
            index_pattern: None,
            estimated_subgoals: 1,
        }
    }
    /// Builder: set the index pattern for discrimination-tree lookup.
    pub fn with_pattern(mut self, pattern: Expr) -> Self {
        self.index_pattern = Some(pattern);
        self
    }
    /// Builder: set the estimated number of subgoals.
    pub fn with_estimated_subgoals(mut self, n: u32) -> Self {
        self.estimated_subgoals = n;
        self
    }
    /// Compute the effective cost of applying this rule at the given depth.
    ///
    /// Cost = priority * depth_penalty^depth * (1 + estimated_subgoals).
    pub fn effective_cost(&self, depth: usize, depth_penalty: f64) -> f64 {
        let base = self.priority as f64;
        let penalty = depth_penalty.powi(depth as i32);
        let subgoal_factor = 1.0 + self.estimated_subgoals as f64;
        base * penalty * subgoal_factor
    }
}
/// Unique identifier for a node in the search tree.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct NodeId(pub usize);
/// Configuration for the `aesop` tactic.
///
/// Controls search limits, normalization behaviour, and rule selection.
#[derive(Clone, Debug)]
pub struct AesopConfig {
    /// Maximum number of rules that may be registered in a single rule set.
    pub max_rules: usize,
    /// Maximum depth of the search tree. Nodes deeper than this are pruned.
    pub max_depth: usize,
    /// Maximum number of search iterations (node expansions) before giving up.
    pub max_iters: usize,
    /// Maximum total number of rule application attempts across the whole search.
    pub max_rule_apps: usize,
    /// Whether to run normalization (simp-like) between rule applications.
    pub norm_simp: bool,
    /// Whether to include the built-in default rule set.
    pub use_default_rules: bool,
    /// Transparency mode for unfolding during search.
    pub transparency: TransparencyMode,
    /// Penalty multiplier applied per depth level. Higher values discourage deep search.
    pub depth_penalty: f64,
    /// Whether to enable proof caching (memoization of solved goals).
    pub enable_cache: bool,
    /// Timeout in milliseconds (0 means no timeout).
    pub timeout_ms: u64,
    /// Whether to warn (instead of error) when the search fails.
    pub warn_on_failure: bool,
    /// Whether to collect detailed statistics during search.
    pub collect_stats: bool,
}
impl AesopConfig {
    /// Create a configuration suitable for fast, shallow searches.
    pub fn fast() -> Self {
        Self {
            max_depth: 8,
            max_iters: 500,
            max_rule_apps: 2000,
            norm_simp: false,
            depth_penalty: 2.0,
            timeout_ms: 1000,
            ..Self::default()
        }
    }
    /// Create a configuration for thorough, deep searches.
    pub fn thorough() -> Self {
        Self {
            max_depth: 60,
            max_iters: 50_000,
            max_rule_apps: 500_000,
            norm_simp: true,
            depth_penalty: 1.05,
            timeout_ms: 30_000,
            ..Self::default()
        }
    }
    /// Builder: set maximum depth.
    pub fn with_max_depth(mut self, d: usize) -> Self {
        self.max_depth = d;
        self
    }
    /// Builder: set maximum iterations.
    pub fn with_max_iters(mut self, n: usize) -> Self {
        self.max_iters = n;
        self
    }
    /// Builder: set timeout in milliseconds.
    pub fn with_timeout_ms(mut self, ms: u64) -> Self {
        self.timeout_ms = ms;
        self
    }
    /// Builder: enable or disable normalization.
    pub fn with_norm_simp(mut self, b: bool) -> Self {
        self.norm_simp = b;
        self
    }
    /// Builder: enable or disable proof caching.
    pub fn with_cache(mut self, b: bool) -> Self {
        self.enable_cache = b;
        self
    }
}
/// Cached proof entry.
#[derive(Clone, Debug)]
struct CacheEntry {
    /// The proof term that solved the goal.
    pub(super) proof: Expr,
    /// Number of rule applications used.
    #[allow(dead_code)]
    pub(super) rule_apps: usize,
}
/// Lightweight identifier referencing a rule inside an `AesopRuleSet`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct RuleId(pub usize);
/// The kind of transformation a rule performs.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum AesopRuleKind {
    /// Apply a lemma to the goal (backward reasoning).
    Apply,
    /// Apply a constructor of the goal's inductive type.
    Constructor,
    /// Perform case analysis on a hypothesis.
    Cases,
    /// Apply extensionality (e.g., `funext`, `propext`).
    Ext,
    /// Apply a lemma to a hypothesis (forward reasoning).
    Forward,
    /// Unfold a definition in the goal.
    Unfold,
    /// Normalization rule (applied during the normalization phase).
    Norm,
}
/// A collection of aesop rules indexed for fast retrieval.
///
/// Rules are stored in a flat vector and indexed both by a discrimination tree
/// (for pattern-based lookup) and by priority (via a sorted index).
pub struct AesopRuleSet {
    /// All registered rules.
    pub(super) rules: Vec<AesopRule>,
    /// Discrimination tree mapping goal patterns to rule ids.
    pub(super) pattern_index: DiscrTree<RuleId>,
    /// Rule ids sorted by priority (ascending = tried first).
    pub(super) priority_order: Vec<RuleId>,
    /// Map from rule name to rule id for O(1) lookup by name.
    pub(super) name_index: HashMap<String, RuleId>,
    /// Ids of rules that have no index pattern (wildcards).
    pub(super) wildcard_ids: Vec<RuleId>,
    /// Ids of normalization rules.
    pub(super) norm_ids: Vec<RuleId>,
    /// Maximum number of rules allowed in this set.
    pub(super) max_rules: usize,
}
impl AesopRuleSet {
    /// Create a new empty rule set.
    pub fn new() -> Self {
        Self {
            rules: Vec::new(),
            pattern_index: DiscrTree::new(),
            priority_order: Vec::new(),
            name_index: HashMap::new(),
            wildcard_ids: Vec::new(),
            norm_ids: Vec::new(),
            max_rules: 1024,
        }
    }
    /// Create a rule set with a given capacity limit.
    pub fn with_max_rules(mut self, n: usize) -> Self {
        self.max_rules = n;
        self
    }
    /// Return the number of registered rules.
    pub fn len(&self) -> usize {
        self.rules.len()
    }
    /// Return `true` if there are no registered rules.
    pub fn is_empty(&self) -> bool {
        self.rules.is_empty()
    }
    /// Add a rule to the set. Returns the assigned `RuleId`.
    ///
    /// If a rule with the same name already exists it is replaced.
    /// Returns an error if the capacity limit would be exceeded.
    pub fn add(&mut self, rule: AesopRule) -> TacticResult<RuleId> {
        let name_key = format!("{}", rule.name);
        if let Some(&old_id) = self.name_index.get(&name_key) {
            self.remove_by_id(old_id);
        }
        if self.rules.len() >= self.max_rules {
            return Err(TacticError::Failed(format!(
                "aesop: rule set capacity exceeded (max {})",
                self.max_rules
            )));
        }
        let id = RuleId(self.rules.len());
        if let Some(ref pat) = rule.index_pattern {
            self.pattern_index.insert(pat, id);
        } else {
            self.wildcard_ids.push(id);
        }
        if rule.kind == AesopRuleKind::Norm {
            self.norm_ids.push(id);
        }
        self.name_index.insert(name_key, id);
        let prio = rule.priority;
        let pos = self
            .priority_order
            .partition_point(|rid| self.rules.get(rid.0).map_or(true, |r| r.priority <= prio));
        self.priority_order.insert(pos, id);
        self.rules.push(rule);
        Ok(id)
    }
    /// Remove a rule by name. Returns `true` if a rule was removed.
    pub fn remove(&mut self, name: &Name) -> bool {
        let key = format!("{}", name);
        if let Some(&id) = self.name_index.get(&key) {
            self.remove_by_id(id);
            self.name_index.remove(&key);
            true
        } else {
            false
        }
    }
    /// Internal removal by id — marks the slot as inactive.
    fn remove_by_id(&mut self, id: RuleId) {
        self.priority_order.retain(|r| *r != id);
        self.wildcard_ids.retain(|r| *r != id);
        self.norm_ids.retain(|r| *r != id);
    }
    /// Query: find all rule ids whose pattern matches the given goal expression.
    ///
    /// The returned list is sorted by priority (ascending).
    pub fn query(&self, goal_expr: &Expr) -> Vec<RuleId> {
        let mut ids: Vec<RuleId> = Vec::new();
        let dt_hits = self.pattern_index.find(goal_expr);
        for id_ref in dt_hits {
            if self.is_active(*id_ref) {
                ids.push(*id_ref);
            }
        }
        for &id in &self.wildcard_ids {
            if self.is_active(id) && !ids.contains(&id) {
                ids.push(id);
            }
        }
        ids.sort_by_key(|id| self.rules.get(id.0).map_or(u32::MAX, |r| r.priority));
        ids
    }
    /// Query: return all normalization rule ids, sorted by priority.
    pub fn norm_rules(&self) -> Vec<RuleId> {
        let mut ids: Vec<RuleId> = self
            .norm_ids
            .iter()
            .copied()
            .filter(|id| self.is_active(*id))
            .collect();
        ids.sort_by_key(|id| self.rules.get(id.0).map_or(u32::MAX, |r| r.priority));
        ids
    }
    /// Get a reference to a rule by id.
    pub fn get(&self, id: RuleId) -> Option<&AesopRule> {
        self.rules.get(id.0)
    }
    /// Check whether a rule id refers to a rule that is still active
    /// (i.e., its name is still in the name index).
    fn is_active(&self, id: RuleId) -> bool {
        if let Some(rule) = self.rules.get(id.0) {
            let key = format!("{}", rule.name);
            self.name_index
                .get(&key)
                .is_some_and(|&stored_id| stored_id == id)
        } else {
            false
        }
    }
    /// Return an iterator over all active rule ids sorted by priority.
    pub fn all_by_priority(&self) -> Vec<RuleId> {
        self.priority_order
            .iter()
            .copied()
            .filter(|id| self.is_active(*id))
            .collect()
    }
    /// Merge another rule set into this one.
    ///
    /// Rules from `other` are re-inserted; duplicates (by name) are overwritten.
    pub fn merge_from(&mut self, other: &AesopRuleSet) -> TacticResult<()> {
        for id in other.all_by_priority() {
            if let Some(rule) = other.get(id) {
                let new_rule = AesopRule {
                    name: rule.name.clone(),
                    tactic: Box::new(|_s, _c| Ok(())),
                    safety: rule.safety,
                    kind: rule.kind,
                    priority: rule.priority,
                    index_pattern: rule.index_pattern.clone(),
                    estimated_subgoals: rule.estimated_subgoals,
                };
                self.add(new_rule)?;
            }
        }
        Ok(())
    }
}
