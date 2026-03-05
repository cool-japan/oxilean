//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::{LcnfArg, LcnfExpr, LcnfFunDecl, LcnfLetValue, LcnfLit, LcnfVarId};
use std::collections::HashMap;

use std::collections::{HashSet, VecDeque};

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DBDepGraph {
    pub(super) nodes: Vec<u32>,
    pub(super) edges: Vec<(u32, u32)>,
}
impl DBDepGraph {
    #[allow(dead_code)]
    pub fn new() -> Self {
        DBDepGraph {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }
    #[allow(dead_code)]
    pub fn add_node(&mut self, id: u32) {
        if !self.nodes.contains(&id) {
            self.nodes.push(id);
        }
    }
    #[allow(dead_code)]
    pub fn add_dep(&mut self, dep: u32, dependent: u32) {
        self.add_node(dep);
        self.add_node(dependent);
        self.edges.push((dep, dependent));
    }
    #[allow(dead_code)]
    pub fn dependents_of(&self, node: u32) -> Vec<u32> {
        self.edges
            .iter()
            .filter(|(d, _)| *d == node)
            .map(|(_, dep)| *dep)
            .collect()
    }
    #[allow(dead_code)]
    pub fn dependencies_of(&self, node: u32) -> Vec<u32> {
        self.edges
            .iter()
            .filter(|(_, dep)| *dep == node)
            .map(|(d, _)| *d)
            .collect()
    }
    #[allow(dead_code)]
    pub fn topological_sort(&self) -> Vec<u32> {
        let mut in_degree: std::collections::HashMap<u32, u32> = std::collections::HashMap::new();
        for &n in &self.nodes {
            in_degree.insert(n, 0);
        }
        for (_, dep) in &self.edges {
            *in_degree.entry(*dep).or_insert(0) += 1;
        }
        let mut queue: std::collections::VecDeque<u32> = self
            .nodes
            .iter()
            .filter(|&&n| in_degree[&n] == 0)
            .copied()
            .collect();
        let mut result = Vec::new();
        while let Some(node) = queue.pop_front() {
            result.push(node);
            for dep in self.dependents_of(node) {
                let cnt = in_degree.entry(dep).or_insert(0);
                *cnt = cnt.saturating_sub(1);
                if *cnt == 0 {
                    queue.push_back(dep);
                }
            }
        }
        result
    }
    #[allow(dead_code)]
    pub fn has_cycle(&self) -> bool {
        self.topological_sort().len() < self.nodes.len()
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DBWorklist {
    pub(super) items: std::collections::VecDeque<u32>,
    pub(super) in_worklist: std::collections::HashSet<u32>,
}
impl DBWorklist {
    #[allow(dead_code)]
    pub fn new() -> Self {
        DBWorklist {
            items: std::collections::VecDeque::new(),
            in_worklist: std::collections::HashSet::new(),
        }
    }
    #[allow(dead_code)]
    pub fn push(&mut self, item: u32) -> bool {
        if self.in_worklist.insert(item) {
            self.items.push_back(item);
            true
        } else {
            false
        }
    }
    #[allow(dead_code)]
    pub fn pop(&mut self) -> Option<u32> {
        let item = self.items.pop_front()?;
        self.in_worklist.remove(&item);
        Some(item)
    }
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.items.len()
    }
    #[allow(dead_code)]
    pub fn contains(&self, item: u32) -> bool {
        self.in_worklist.contains(&item)
    }
}
/// Result of a conditional constant propagation query.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum CcpValue {
    /// The variable is definitely undefined (top in the lattice)
    Undefined,
    /// The variable has a known constant value
    Constant(crate::lcnf::LcnfLit),
    /// The variable may have any value (bottom in the lattice)
    Overdefined,
}
impl CcpValue {
    /// Meet operation for the CCP lattice.
    ///
    /// ```text
    /// Undefined meet x = x
    /// x meet Undefined = x
    /// Constant(a) meet Constant(a) = Constant(a)
    /// Constant(a) meet Constant(b) = Overdefined (a ≠ b)
    /// Overdefined meet x = Overdefined
    /// ```
    #[allow(dead_code)]
    pub fn meet(&self, other: &CcpValue) -> CcpValue {
        match (self, other) {
            (CcpValue::Undefined, x) | (x, CcpValue::Undefined) => x.clone(),
            (CcpValue::Overdefined, _) | (_, CcpValue::Overdefined) => CcpValue::Overdefined,
            (CcpValue::Constant(a), CcpValue::Constant(b)) => {
                if a == b {
                    CcpValue::Constant(a.clone())
                } else {
                    CcpValue::Overdefined
                }
            }
        }
    }
    /// Returns true if the value is a known constant.
    #[allow(dead_code)]
    pub fn is_constant(&self) -> bool {
        matches!(self, CcpValue::Constant(_))
    }
    /// Returns true if the value might vary at runtime.
    #[allow(dead_code)]
    pub fn is_overdefined(&self) -> bool {
        matches!(self, CcpValue::Overdefined)
    }
    /// Extract the literal if constant.
    #[allow(dead_code)]
    pub fn literal(&self) -> Option<&crate::lcnf::LcnfLit> {
        if let CcpValue::Constant(lit) = self {
            Some(lit)
        } else {
            None
        }
    }
}
#[allow(dead_code)]
pub struct DBConstantFoldingHelper;
impl DBConstantFoldingHelper {
    #[allow(dead_code)]
    pub fn fold_add_i64(a: i64, b: i64) -> Option<i64> {
        a.checked_add(b)
    }
    #[allow(dead_code)]
    pub fn fold_sub_i64(a: i64, b: i64) -> Option<i64> {
        a.checked_sub(b)
    }
    #[allow(dead_code)]
    pub fn fold_mul_i64(a: i64, b: i64) -> Option<i64> {
        a.checked_mul(b)
    }
    #[allow(dead_code)]
    pub fn fold_div_i64(a: i64, b: i64) -> Option<i64> {
        if b == 0 {
            None
        } else {
            a.checked_div(b)
        }
    }
    #[allow(dead_code)]
    pub fn fold_add_f64(a: f64, b: f64) -> f64 {
        a + b
    }
    #[allow(dead_code)]
    pub fn fold_mul_f64(a: f64, b: f64) -> f64 {
        a * b
    }
    #[allow(dead_code)]
    pub fn fold_neg_i64(a: i64) -> Option<i64> {
        a.checked_neg()
    }
    #[allow(dead_code)]
    pub fn fold_not_bool(a: bool) -> bool {
        !a
    }
    #[allow(dead_code)]
    pub fn fold_and_bool(a: bool, b: bool) -> bool {
        a && b
    }
    #[allow(dead_code)]
    pub fn fold_or_bool(a: bool, b: bool) -> bool {
        a || b
    }
    #[allow(dead_code)]
    pub fn fold_shl_i64(a: i64, b: u32) -> Option<i64> {
        a.checked_shl(b)
    }
    #[allow(dead_code)]
    pub fn fold_shr_i64(a: i64, b: u32) -> Option<i64> {
        a.checked_shr(b)
    }
    #[allow(dead_code)]
    pub fn fold_rem_i64(a: i64, b: i64) -> Option<i64> {
        if b == 0 {
            None
        } else {
            Some(a % b)
        }
    }
    #[allow(dead_code)]
    pub fn fold_bitand_i64(a: i64, b: i64) -> i64 {
        a & b
    }
    #[allow(dead_code)]
    pub fn fold_bitor_i64(a: i64, b: i64) -> i64 {
        a | b
    }
    #[allow(dead_code)]
    pub fn fold_bitxor_i64(a: i64, b: i64) -> i64 {
        a ^ b
    }
    #[allow(dead_code)]
    pub fn fold_bitnot_i64(a: i64) -> i64 {
        !a
    }
}
/// A static branch probability estimator using heuristics.
///
/// These heuristics are inspired by LLVM's BranchProbabilityInfo pass.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct BranchProbabilityEstimator {
    /// Override probability for specific constructor tags
    pub(super) overrides: std::collections::HashMap<(String, u32), f64>,
}
impl BranchProbabilityEstimator {
    /// Create a new estimator.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    /// Override the probability for a specific constructor.
    #[allow(dead_code)]
    pub fn set_probability(&mut self, ctor: impl Into<String>, tag: u32, prob: f64) {
        self.overrides
            .insert((ctor.into(), tag), prob.clamp(0.0, 1.0));
    }
    /// Estimate the probability of a constructor arm being taken.
    ///
    /// Uses the "loop exit heuristic": alternatives that continue the
    /// recursion are more likely than base cases.
    #[allow(dead_code)]
    pub fn estimate(&self, ctor_name: &str, tag: u32, _total_alts: usize) -> f64 {
        let key = (ctor_name.to_string(), tag);
        if let Some(&p) = self.overrides.get(&key) {
            return p;
        }
        if tag == 0 {
            0.15
        } else {
            0.85
        }
    }
    /// Estimate all probabilities for a set of constructors, ensuring they sum to 1.
    #[allow(dead_code)]
    pub fn estimate_all(&self, alts: &[(String, u32)]) -> Vec<f64> {
        let n = alts.len();
        if n == 0 {
            return Vec::new();
        }
        let raw: Vec<f64> = alts
            .iter()
            .map(|(name, tag)| self.estimate(name, *tag, n))
            .collect();
        let sum: f64 = raw.iter().sum();
        if sum == 0.0 {
            return vec![1.0 / n as f64; n];
        }
        raw.iter().map(|p| p / sum).collect()
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DBPassConfig {
    pub phase: DBPassPhase,
    pub enabled: bool,
    pub max_iterations: u32,
    pub debug_output: bool,
    pub pass_name: String,
}
impl DBPassConfig {
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>, phase: DBPassPhase) -> Self {
        DBPassConfig {
            phase,
            enabled: true,
            max_iterations: 10,
            debug_output: false,
            pass_name: name.into(),
        }
    }
    #[allow(dead_code)]
    pub fn disabled(mut self) -> Self {
        self.enabled = false;
        self
    }
    #[allow(dead_code)]
    pub fn with_debug(mut self) -> Self {
        self.debug_output = true;
        self
    }
    #[allow(dead_code)]
    pub fn max_iter(mut self, n: u32) -> Self {
        self.max_iterations = n;
        self
    }
}
/// Reachability status of a case arm.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArmReachability {
    /// This arm is definitely reachable
    Reachable,
    /// This arm is definitely unreachable (can be deleted)
    Unreachable,
    /// This arm's reachability is unknown
    MaybeReachable,
}
/// Configuration for the dead branch elimination pass.
#[derive(Debug, Clone)]
pub struct DeadBranchConfig {
    /// Maximum number of fixed-point passes to run.
    pub max_passes: usize,
    /// Fold case expressions where all branches return the same value.
    pub fold_constants: bool,
    /// Enable constructor frequency profiling.
    pub use_profiling: bool,
    /// Maximum case alternatives before skipping analysis (performance guard).
    pub max_alts_analyzed: usize,
}
/// The kind of dead branch optimization applied.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum DeadBranchOptKind {
    /// A case arm was eliminated because the constructor was known
    ArmEliminated,
    /// A case expression was folded (scrutinee known at compile time)
    CaseFolded,
    /// A uniform-return folding (all arms return the same value)
    UniformReturn,
    /// A single-arm case was inlined
    SingleArmInlined,
    /// An `Unreachable` node was removed
    UnreachableRemoved,
}
/// Detailed statistics for the dead branch pass, beyond what DeadBranchReport tracks.
#[derive(Debug, Clone, Default)]
pub struct DeadBranchStats {
    /// Number of case expressions analyzed.
    pub cases_analyzed: usize,
    /// Number of known-ctor matches that succeeded.
    pub known_ctor_matches: usize,
    /// Number of unreachable defaults removed.
    pub unreachable_defaults: usize,
    /// Number of single-branch inlines.
    pub single_branch_inlines: usize,
    /// Number of uniform-return folds.
    pub uniform_folds: usize,
    /// Number of variables with known values at some point.
    pub env_entries_created: usize,
}
impl DeadBranchStats {
    /// Total transformations.
    pub fn total(&self) -> usize {
        self.known_ctor_matches
            + self.unreachable_defaults
            + self.single_branch_inlines
            + self.uniform_folds
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DBLivenessInfo {
    pub live_in: Vec<std::collections::HashSet<u32>>,
    pub live_out: Vec<std::collections::HashSet<u32>>,
    pub defs: Vec<std::collections::HashSet<u32>>,
    pub uses: Vec<std::collections::HashSet<u32>>,
}
impl DBLivenessInfo {
    #[allow(dead_code)]
    pub fn new(block_count: usize) -> Self {
        DBLivenessInfo {
            live_in: vec![std::collections::HashSet::new(); block_count],
            live_out: vec![std::collections::HashSet::new(); block_count],
            defs: vec![std::collections::HashSet::new(); block_count],
            uses: vec![std::collections::HashSet::new(); block_count],
        }
    }
    #[allow(dead_code)]
    pub fn add_def(&mut self, block: usize, var: u32) {
        if block < self.defs.len() {
            self.defs[block].insert(var);
        }
    }
    #[allow(dead_code)]
    pub fn add_use(&mut self, block: usize, var: u32) {
        if block < self.uses.len() {
            self.uses[block].insert(var);
        }
    }
    #[allow(dead_code)]
    pub fn is_live_in(&self, block: usize, var: u32) -> bool {
        self.live_in
            .get(block)
            .map(|s| s.contains(&var))
            .unwrap_or(false)
    }
    #[allow(dead_code)]
    pub fn is_live_out(&self, block: usize, var: u32) -> bool {
        self.live_out
            .get(block)
            .map(|s| s.contains(&var))
            .unwrap_or(false)
    }
}
/// A structured log entry from the dead branch pass.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DeadBranchLogEntry {
    /// The function where the optimization occurred
    pub function: String,
    /// The type of optimization
    pub kind: DeadBranchOptKind,
    /// Description of what was eliminated or folded
    pub detail: String,
}
impl DeadBranchLogEntry {
    /// Create a new log entry.
    #[allow(dead_code)]
    pub fn new(
        function: impl Into<String>,
        kind: DeadBranchOptKind,
        detail: impl Into<String>,
    ) -> Self {
        DeadBranchLogEntry {
            function: function.into(),
            kind,
            detail: detail.into(),
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DBAnalysisCache {
    pub(super) entries: std::collections::HashMap<String, DBCacheEntry>,
    pub(super) max_size: usize,
    pub(super) hits: u64,
    pub(super) misses: u64,
}
impl DBAnalysisCache {
    #[allow(dead_code)]
    pub fn new(max_size: usize) -> Self {
        DBAnalysisCache {
            entries: std::collections::HashMap::new(),
            max_size,
            hits: 0,
            misses: 0,
        }
    }
    #[allow(dead_code)]
    pub fn get(&mut self, key: &str) -> Option<&DBCacheEntry> {
        if self.entries.contains_key(key) {
            self.hits += 1;
            self.entries.get(key)
        } else {
            self.misses += 1;
            None
        }
    }
    #[allow(dead_code)]
    pub fn insert(&mut self, key: String, data: Vec<u8>) {
        if self.entries.len() >= self.max_size {
            if let Some(oldest) = self.entries.keys().next().cloned() {
                self.entries.remove(&oldest);
            }
        }
        self.entries.insert(
            key.clone(),
            DBCacheEntry {
                key,
                data,
                timestamp: 0,
                valid: true,
            },
        );
    }
    #[allow(dead_code)]
    pub fn invalidate(&mut self, key: &str) {
        if let Some(entry) = self.entries.get_mut(key) {
            entry.valid = false;
        }
    }
    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.entries.clear();
    }
    #[allow(dead_code)]
    pub fn hit_rate(&self) -> f64 {
        let total = self.hits + self.misses;
        if total == 0 {
            return 0.0;
        }
        self.hits as f64 / total as f64
    }
    #[allow(dead_code)]
    pub fn size(&self) -> usize {
        self.entries.len()
    }
}
/// A phi-node conceptually merges values from multiple predecessor branches.
///
/// In SSA form, after a branch merge point, phi nodes select which
/// predecessor's value to use. After dead branch elimination, phi nodes
/// with only one surviving predecessor become simple copies.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PhiNode {
    /// The destination variable
    pub dest: crate::lcnf::LcnfVarId,
    /// Incoming (predecessor_label, value) pairs
    pub incoming: Vec<(u32, crate::lcnf::LcnfArg)>,
}
impl PhiNode {
    /// Create a new phi node.
    #[allow(dead_code)]
    pub fn new(dest: crate::lcnf::LcnfVarId) -> Self {
        PhiNode {
            dest,
            incoming: Vec::new(),
        }
    }
    /// Add an incoming value from a predecessor label.
    #[allow(dead_code)]
    pub fn add_incoming(&mut self, label: u32, val: crate::lcnf::LcnfArg) {
        self.incoming.push((label, val));
    }
    /// Returns true if this phi can be simplified to a copy (only one live predecessor).
    #[allow(dead_code)]
    pub fn is_trivial(&self) -> bool {
        self.incoming.len() == 1
    }
    /// Simplify: if trivial, return the single incoming value.
    #[allow(dead_code)]
    pub fn simplify(&self) -> Option<&crate::lcnf::LcnfArg> {
        if self.is_trivial() {
            self.incoming.first().map(|(_, v)| v)
        } else {
            None
        }
    }
    /// Count live predecessors (those not eliminated).
    #[allow(dead_code)]
    pub fn live_count(&self) -> usize {
        self.incoming.len()
    }
}
/// A cache for frequently seen scrutinee-constructor patterns.
///
/// Reuses previous analysis results to avoid redundant work when
/// the same value is scrutinized multiple times.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct BranchPatternCache {
    /// Map from scrutinee variable to cached known constructor name
    pub(super) cache: std::collections::HashMap<crate::lcnf::LcnfVarId, String>,
    /// Hit count
    pub(super) hits: u64,
    /// Miss count
    pub(super) misses: u64,
}
impl BranchPatternCache {
    /// Create a new empty cache.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    /// Store a constructor pattern for a scrutinee.
    #[allow(dead_code)]
    pub fn store(&mut self, var: crate::lcnf::LcnfVarId, ctor: impl Into<String>) {
        self.cache.insert(var, ctor.into());
    }
    /// Look up a scrutinee's known constructor.
    #[allow(dead_code)]
    pub fn lookup(&mut self, var: crate::lcnf::LcnfVarId) -> Option<&str> {
        if let Some(s) = self.cache.get(&var) {
            self.hits += 1;
            Some(s.as_str())
        } else {
            self.misses += 1;
            None
        }
    }
    /// Invalidate a cache entry (the variable's value may have changed).
    #[allow(dead_code)]
    pub fn invalidate(&mut self, var: crate::lcnf::LcnfVarId) {
        self.cache.remove(&var);
    }
    /// Cache hit rate as a percentage.
    #[allow(dead_code)]
    pub fn hit_rate(&self) -> f64 {
        let total = self.hits + self.misses;
        if total == 0 {
            0.0
        } else {
            self.hits as f64 / total as f64 * 100.0
        }
    }
    /// Clear the entire cache.
    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.cache.clear();
        self.hits = 0;
        self.misses = 0;
    }
    /// Return cache statistics as a string.
    #[allow(dead_code)]
    pub fn stats(&self) -> String {
        format!(
            "BranchPatternCache{{ entries={}, hits={}, misses={}, rate={:.1}% }}",
            self.cache.len(),
            self.hits,
            self.misses,
            self.hit_rate()
        )
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DBDominatorTree {
    pub idom: Vec<Option<u32>>,
    pub dom_children: Vec<Vec<u32>>,
    pub dom_depth: Vec<u32>,
}
impl DBDominatorTree {
    #[allow(dead_code)]
    pub fn new(size: usize) -> Self {
        DBDominatorTree {
            idom: vec![None; size],
            dom_children: vec![Vec::new(); size],
            dom_depth: vec![0; size],
        }
    }
    #[allow(dead_code)]
    pub fn set_idom(&mut self, node: usize, idom: u32) {
        self.idom[node] = Some(idom);
    }
    #[allow(dead_code)]
    pub fn dominates(&self, a: usize, b: usize) -> bool {
        if a == b {
            return true;
        }
        let mut cur = b;
        loop {
            match self.idom[cur] {
                Some(parent) if parent as usize == a => return true,
                Some(parent) if parent as usize == cur => return false,
                Some(parent) => cur = parent as usize,
                None => return false,
            }
        }
    }
    #[allow(dead_code)]
    pub fn depth(&self, node: usize) -> u32 {
        self.dom_depth.get(node).copied().unwrap_or(0)
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DBCacheEntry {
    pub key: String,
    pub data: Vec<u8>,
    pub timestamp: u64,
    pub valid: bool,
}
/// A compile-time-known value associated with a variable.
#[derive(Debug, Clone, PartialEq)]
pub enum KnownValue {
    /// A literal (Nat or Str).
    Lit(LcnfLit),
    /// A constructor name + tag (fields may not be tracked).
    Ctor(String, u32),
}
impl KnownValue {
    /// Whether this is a literal value.
    pub(super) fn is_lit(&self) -> bool {
        matches!(self, KnownValue::Lit(_))
    }
    /// Whether this is a constructor value.
    pub(super) fn is_ctor(&self) -> bool {
        matches!(self, KnownValue::Ctor(_, _))
    }
}
/// Simplifies known-true / known-false conditions.
#[derive(Debug, Default)]
pub struct ConditionSimplifier {
    /// Known-true conditions (variable ids that are known to be true/nonzero).
    pub(super) known_true: Vec<LcnfVarId>,
    /// Known-false conditions.
    pub(super) known_false: Vec<LcnfVarId>,
    /// Simplifications performed.
    pub simplifications: usize,
}
impl ConditionSimplifier {
    /// Create a new simplifier.
    pub fn new() -> Self {
        ConditionSimplifier::default()
    }
    /// Mark a variable as known-true.
    pub fn mark_true(&mut self, var: LcnfVarId) {
        if !self.known_true.contains(&var) {
            self.known_true.push(var);
        }
    }
    /// Mark a variable as known-false.
    pub fn mark_false(&mut self, var: LcnfVarId) {
        if !self.known_false.contains(&var) {
            self.known_false.push(var);
        }
    }
    /// Check if a variable is known-true.
    pub fn is_known_true(&self, var: &LcnfVarId) -> bool {
        self.known_true.contains(var)
    }
    /// Check if a variable is known-false.
    pub fn is_known_false(&self, var: &LcnfVarId) -> bool {
        self.known_false.contains(var)
    }
    /// Number of known conditions.
    pub fn num_known(&self) -> usize {
        self.known_true.len() + self.known_false.len()
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct DBPassStats {
    pub total_runs: u32,
    pub successful_runs: u32,
    pub total_changes: u64,
    pub time_ms: u64,
    pub iterations_used: u32,
}
impl DBPassStats {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    #[allow(dead_code)]
    pub fn record_run(&mut self, changes: u64, time_ms: u64, iterations: u32) {
        self.total_runs += 1;
        self.successful_runs += 1;
        self.total_changes += changes;
        self.time_ms += time_ms;
        self.iterations_used = iterations;
    }
    #[allow(dead_code)]
    pub fn average_changes_per_run(&self) -> f64 {
        if self.total_runs == 0 {
            return 0.0;
        }
        self.total_changes as f64 / self.total_runs as f64
    }
    #[allow(dead_code)]
    pub fn success_rate(&self) -> f64 {
        if self.total_runs == 0 {
            return 0.0;
        }
        self.successful_runs as f64 / self.total_runs as f64
    }
    #[allow(dead_code)]
    pub fn format_summary(&self) -> String {
        format!(
            "Runs: {}/{}, Changes: {}, Time: {}ms",
            self.successful_runs, self.total_runs, self.total_changes, self.time_ms
        )
    }
}
#[allow(dead_code)]
pub struct DBPassRegistry {
    pub(super) configs: Vec<DBPassConfig>,
    pub(super) stats: std::collections::HashMap<String, DBPassStats>,
}
impl DBPassRegistry {
    #[allow(dead_code)]
    pub fn new() -> Self {
        DBPassRegistry {
            configs: Vec::new(),
            stats: std::collections::HashMap::new(),
        }
    }
    #[allow(dead_code)]
    pub fn register(&mut self, config: DBPassConfig) {
        self.stats
            .insert(config.pass_name.clone(), DBPassStats::new());
        self.configs.push(config);
    }
    #[allow(dead_code)]
    pub fn enabled_passes(&self) -> Vec<&DBPassConfig> {
        self.configs.iter().filter(|c| c.enabled).collect()
    }
    #[allow(dead_code)]
    pub fn get_stats(&self, name: &str) -> Option<&DBPassStats> {
        self.stats.get(name)
    }
    #[allow(dead_code)]
    pub fn total_passes(&self) -> usize {
        self.configs.len()
    }
    #[allow(dead_code)]
    pub fn enabled_count(&self) -> usize {
        self.enabled_passes().len()
    }
    #[allow(dead_code)]
    pub fn update_stats(&mut self, name: &str, changes: u64, time_ms: u64, iter: u32) {
        if let Some(stats) = self.stats.get_mut(name) {
            stats.record_run(changes, time_ms, iter);
        }
    }
}
/// Aggregates dead branch statistics across multiple compilation units.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct DeadBranchAggregator {
    pub(super) reports: Vec<DeadBranchReport>,
}
impl DeadBranchAggregator {
    /// Create a new aggregator.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    /// Add a report from one function/module.
    #[allow(dead_code)]
    pub fn add(&mut self, report: DeadBranchReport) {
        self.reports.push(report);
    }
    /// Total arms eliminated across all reports.
    #[allow(dead_code)]
    pub fn total_arms_eliminated(&self) -> usize {
        self.reports.iter().map(|r| r.arms_eliminated).sum()
    }
    /// Total cases folded across all reports.
    #[allow(dead_code)]
    pub fn total_cases_folded(&self) -> usize {
        self.reports.iter().map(|r| r.cases_folded).sum()
    }
    /// Total uniform returns across all reports.
    #[allow(dead_code)]
    pub fn total_uniform_returns(&self) -> usize {
        self.reports.iter().map(|r| r.uniform_returns).sum()
    }
    /// Number of compilation units processed.
    #[allow(dead_code)]
    pub fn unit_count(&self) -> usize {
        self.reports.len()
    }
    /// Generate a summary string.
    #[allow(dead_code)]
    pub fn summary(&self) -> String {
        format!(
            "DeadBranchAggregate{{ units={}, arms_eliminated={}, cases_folded={}, uniform_returns={} }}",
            self.unit_count(), self.total_arms_eliminated(), self.total_cases_folded(),
            self.total_uniform_returns()
        )
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum DBPassPhase {
    Analysis,
    Transformation,
    Verification,
    Cleanup,
}
impl DBPassPhase {
    #[allow(dead_code)]
    pub fn name(&self) -> &str {
        match self {
            DBPassPhase::Analysis => "analysis",
            DBPassPhase::Transformation => "transformation",
            DBPassPhase::Verification => "verification",
            DBPassPhase::Cleanup => "cleanup",
        }
    }
    #[allow(dead_code)]
    pub fn is_modifying(&self) -> bool {
        matches!(self, DBPassPhase::Transformation | DBPassPhase::Cleanup)
    }
}
/// Helper for building a constant propagation environment incrementally.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct ConstEnvBuilder {
    pub(super) env: std::collections::HashMap<crate::lcnf::LcnfVarId, crate::lcnf::LcnfLit>,
}
impl ConstEnvBuilder {
    /// Create an empty environment builder.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    /// Bind a variable to a literal constant.
    #[allow(dead_code)]
    pub fn bind(&mut self, var: crate::lcnf::LcnfVarId, lit: crate::lcnf::LcnfLit) {
        self.env.insert(var, lit);
    }
    /// Look up a variable in the environment.
    #[allow(dead_code)]
    pub fn get(&self, var: &crate::lcnf::LcnfVarId) -> Option<&crate::lcnf::LcnfLit> {
        self.env.get(var)
    }
    /// Build the environment (consume builder).
    #[allow(dead_code)]
    pub fn build(self) -> std::collections::HashMap<crate::lcnf::LcnfVarId, crate::lcnf::LcnfLit> {
        self.env
    }
    /// Number of known constants.
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.env.len()
    }
    /// Whether the environment is empty.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.env.is_empty()
    }
    /// Merge another environment (optimistic: prefer known values).
    #[allow(dead_code)]
    pub fn merge_optimistic(&mut self, other: &ConstEnvBuilder) {
        for (k, v) in &other.env {
            self.env.entry(*k).or_insert_with(|| v.clone());
        }
    }
}
/// Dead branch elimination optimizer for a single LCNF function declaration.
pub struct DeadBranchElim {
    pub(super) config: DeadBranchConfig,
    pub(super) report: DeadBranchReport,
    pub(super) stats: DeadBranchStats,
}
impl DeadBranchElim {
    /// Create a new pass with default configuration.
    pub fn new() -> Self {
        DeadBranchElim {
            config: DeadBranchConfig::default(),
            report: DeadBranchReport::new(),
            stats: DeadBranchStats::default(),
        }
    }
    /// Create a new pass with the given configuration.
    pub fn with_config(config: DeadBranchConfig) -> Self {
        DeadBranchElim {
            config,
            report: DeadBranchReport::new(),
            stats: DeadBranchStats::default(),
        }
    }
    /// Run the pass on a single function declaration.
    pub fn run(&mut self, decl: &mut LcnfFunDecl) -> DeadBranchReport {
        self.report = DeadBranchReport::new();
        self.stats = DeadBranchStats::default();
        for i in 0..self.config.max_passes {
            let mut env: HashMap<LcnfVarId, KnownValue> = HashMap::new();
            let before = (self.report.branches_eliminated, self.report.cases_folded);
            decl.body = self.elim_expr(
                std::mem::replace(&mut decl.body, LcnfExpr::Unreachable),
                &mut env,
            );
            self.report.iterations = i + 1;
            self.report.known_values_tracked += env.len();
            self.stats.env_entries_created += env.len();
            let after = (self.report.branches_eliminated, self.report.cases_folded);
            if before == after {
                break;
            }
        }
        self.report.clone()
    }
    /// Retrieve the final report without running again.
    pub fn report(&self) -> &DeadBranchReport {
        &self.report
    }
    /// Retrieve detailed statistics.
    pub fn detailed_stats(&self) -> &DeadBranchStats {
        &self.stats
    }
    pub(super) fn elim_expr(
        &mut self,
        expr: LcnfExpr,
        env: &mut HashMap<LcnfVarId, KnownValue>,
    ) -> LcnfExpr {
        match expr {
            LcnfExpr::Let {
                id,
                name,
                ty,
                value,
                body,
            } => {
                match &value {
                    LcnfLetValue::Lit(lit) => {
                        env.insert(id, KnownValue::Lit(lit.clone()));
                    }
                    LcnfLetValue::Ctor(ctor_name, tag, _) => {
                        env.insert(id, KnownValue::Ctor(ctor_name.clone(), *tag));
                    }
                    LcnfLetValue::FVar(src_id) => {
                        if let Some(kv) = env.get(src_id).cloned() {
                            env.insert(id, kv);
                        }
                    }
                    _ => {}
                }
                let new_body = self.elim_expr(*body, env);
                LcnfExpr::Let {
                    id,
                    name,
                    ty,
                    value,
                    body: Box::new(new_body),
                }
            }
            LcnfExpr::Case {
                scrutinee,
                scrutinee_ty,
                alts,
                default,
            } => {
                self.stats.cases_analyzed += 1;
                if let Some(kv) = env.get(&scrutinee).cloned() {
                    match &kv {
                        KnownValue::Ctor(known_ctor, known_tag) => {
                            let matching = alts.iter().position(|a| {
                                &a.ctor_name == known_ctor && a.ctor_tag == *known_tag
                            });
                            if let Some(idx) = matching {
                                let alt = alts
                                    .into_iter()
                                    .nth(idx)
                                    .expect(
                                        "idx was returned by Iterator::position so it must be a valid index",
                                    );
                                self.report.cases_folded += 1;
                                self.stats.known_ctor_matches += 1;
                                return self.elim_expr(alt.body, env);
                            }
                            if let Some(def) = default {
                                self.report.cases_folded += 1;
                                self.stats.known_ctor_matches += 1;
                                return self.elim_expr(*def, env);
                            }
                            return LcnfExpr::Unreachable;
                        }
                        KnownValue::Lit(_) => {}
                    }
                }
                if alts.len() > self.config.max_alts_analyzed {
                    return LcnfExpr::Case {
                        scrutinee,
                        scrutinee_ty,
                        alts,
                        default,
                    };
                }
                let mut new_alts = Vec::new();
                for alt in alts {
                    let mut child_env = env.clone();
                    let new_body = self.elim_expr(alt.body, &mut child_env);
                    if new_body == LcnfExpr::Unreachable {
                        self.report.branches_eliminated += 1;
                    } else {
                        new_alts.push(crate::lcnf::LcnfAlt {
                            body: new_body,
                            ..alt
                        });
                    }
                }
                let new_default = default.map(|def| {
                    let mut child_env = env.clone();
                    Box::new(self.elim_expr(*def, &mut child_env))
                });
                let new_default = new_default.and_then(|d| {
                    if *d == LcnfExpr::Unreachable {
                        self.report.branches_eliminated += 1;
                        self.stats.unreachable_defaults += 1;
                        None
                    } else {
                        Some(d)
                    }
                });
                if new_alts.len() == 1 && new_default.is_none() {
                    self.report.cases_folded += 1;
                    self.stats.single_branch_inlines += 1;
                    return new_alts.remove(0).body;
                }
                if new_alts.is_empty() {
                    if let Some(def) = new_default {
                        self.report.cases_folded += 1;
                        self.stats.single_branch_inlines += 1;
                        return *def;
                    }
                    return LcnfExpr::Unreachable;
                }
                if self.config.fold_constants {
                    if let Some(folded) = self.try_fold_uniform(&new_alts, new_default.as_deref()) {
                        self.report.cases_folded += 1;
                        self.stats.uniform_folds += 1;
                        return folded;
                    }
                }
                LcnfExpr::Case {
                    scrutinee,
                    scrutinee_ty,
                    alts: new_alts,
                    default: new_default,
                }
            }
            other => other,
        }
    }
    /// Try to fold a case expression where every branch returns the same
    /// atomic argument.
    pub(super) fn try_fold_uniform(
        &self,
        alts: &[crate::lcnf::LcnfAlt],
        default: Option<&LcnfExpr>,
    ) -> Option<LcnfExpr> {
        fn terminal_arg(e: &LcnfExpr) -> Option<&LcnfArg> {
            match e {
                LcnfExpr::Return(arg) => Some(arg),
                _ => None,
            }
        }
        let first_arg = alts.first().and_then(|a| terminal_arg(&a.body))?;
        for alt in alts.iter().skip(1) {
            let arg = terminal_arg(&alt.body)?;
            if arg != first_arg {
                return None;
            }
        }
        if let Some(def) = default {
            let arg = terminal_arg(def)?;
            if arg != first_arg {
                return None;
            }
        }
        Some(LcnfExpr::Return(first_arg.clone()))
    }
}
/// Represents a compile-time guard condition analysis result.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum GuardResult {
    /// The guard is always true (arm is always taken)
    AlwaysTrue,
    /// The guard is always false (arm is never taken)
    AlwaysFalse,
    /// The guard's truth value is unknown at compile time
    Unknown,
}
impl GuardResult {
    /// Returns true if the guard may evaluate to true.
    #[allow(dead_code)]
    pub fn may_be_true(&self) -> bool {
        !matches!(self, GuardResult::AlwaysFalse)
    }
    /// Returns true if the guard may evaluate to false.
    #[allow(dead_code)]
    pub fn may_be_false(&self) -> bool {
        !matches!(self, GuardResult::AlwaysTrue)
    }
}
/// Frequency estimates for constructor tags within a type.
#[derive(Debug, Clone, Default)]
pub struct CtorFrequencyTable {
    /// Map from (type_name, tag) to frequency.
    pub(super) frequencies: HashMap<(String, u32), f64>,
}
impl CtorFrequencyTable {
    /// Create an empty frequency table.
    pub fn new() -> Self {
        CtorFrequencyTable::default()
    }
    /// Record a frequency for a constructor.
    pub fn record(&mut self, type_name: &str, tag: u32, freq: f64) {
        self.frequencies
            .insert((type_name.to_string(), tag), freq.clamp(0.0, 1.0));
    }
    /// Look up the frequency for a constructor.
    pub fn lookup(&self, type_name: &str, tag: u32) -> Option<f64> {
        self.frequencies.get(&(type_name.to_string(), tag)).copied()
    }
    /// Whether a constructor is "rare" (frequency < threshold).
    pub fn is_rare(&self, type_name: &str, tag: u32, threshold: f64) -> bool {
        self.lookup(type_name, tag)
            .map(|f| f < threshold)
            .unwrap_or(false)
    }
    /// Number of entries.
    pub fn len(&self) -> usize {
        self.frequencies.len()
    }
    /// Whether the table is empty.
    pub fn is_empty(&self) -> bool {
        self.frequencies.is_empty()
    }
}
/// Profiling data for a single case alternative.
#[derive(Debug, Clone, Default)]
pub struct BranchProfile {
    /// Constructor name.
    pub ctor_name: String,
    /// Constructor tag.
    pub ctor_tag: u32,
    /// Estimated execution frequency (0.0 to 1.0).
    pub frequency: f64,
    /// Whether this branch is marked as cold.
    pub is_cold: bool,
    /// Number of times this branch was taken (from PGO data).
    pub taken_count: u64,
}
impl BranchProfile {
    /// Create a new profile for a constructor.
    pub fn new(ctor_name: impl Into<String>, ctor_tag: u32) -> Self {
        BranchProfile {
            ctor_name: ctor_name.into(),
            ctor_tag,
            frequency: 0.0,
            is_cold: false,
            taken_count: 0,
        }
    }
    /// Mark this branch as cold (rarely taken).
    pub fn mark_cold(&mut self) {
        self.is_cold = true;
    }
    /// Set the frequency estimate.
    pub fn with_frequency(mut self, freq: f64) -> Self {
        self.frequency = freq.clamp(0.0, 1.0);
        self
    }
    /// Set the taken count.
    pub fn with_taken_count(mut self, count: u64) -> Self {
        self.taken_count = count;
        self
    }
}
/// Report produced by the dead branch elimination pass.
#[derive(Debug, Clone, Default)]
pub struct DeadBranchReport {
    /// Number of unreachable case alternatives eliminated.
    pub branches_eliminated: usize,
    /// Number of individual arms eliminated (alias for branches_eliminated).
    pub arms_eliminated: usize,
    /// Number of case expressions folded (all branches same value, or
    /// single-branch case inlined).
    pub cases_folded: usize,
    /// Number of fixed-point iterations performed.
    pub iterations: usize,
    /// Number of known-value environment entries created.
    pub known_values_tracked: usize,
    /// Number of uniform-return optimizations applied.
    pub uniform_returns: usize,
}
impl DeadBranchReport {
    pub(super) fn new() -> Self {
        DeadBranchReport::default()
    }
    /// Returns `true` if any transformation was applied.
    pub fn any_changes(&self) -> bool {
        self.branches_eliminated > 0 || self.cases_folded > 0
    }
    /// Total transformations applied.
    pub fn total_changes(&self) -> usize {
        self.branches_eliminated + self.cases_folded
    }
    /// Merge another report into this one.
    pub fn merge(&mut self, other: &DeadBranchReport) {
        self.branches_eliminated += other.branches_eliminated;
        self.arms_eliminated += other.arms_eliminated;
        self.cases_folded += other.cases_folded;
        self.iterations += other.iterations;
        self.known_values_tracked += other.known_values_tracked;
        self.uniform_returns += other.uniform_returns;
    }
}
/// A trace log accumulating all dead branch optimizations.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct DeadBranchTrace {
    pub(super) entries: Vec<DeadBranchLogEntry>,
}
impl DeadBranchTrace {
    /// Create an empty trace.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    /// Append a log entry.
    #[allow(dead_code)]
    pub fn log(&mut self, entry: DeadBranchLogEntry) {
        self.entries.push(entry);
    }
    /// Number of entries.
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    /// Whether the trace is empty.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
    /// Count entries of a specific kind.
    #[allow(dead_code)]
    pub fn count_kind(&self, kind: &DeadBranchOptKind) -> usize {
        self.entries.iter().filter(|e| &e.kind == kind).count()
    }
    /// Render the trace as a multi-line string.
    #[allow(dead_code)]
    pub fn render(&self) -> String {
        self.entries
            .iter()
            .map(|e| e.to_string())
            .collect::<Vec<_>>()
            .join("\n")
    }
}
/// A simplified dominator set for identifying unreachable code regions.
///
/// A block B is dominated by A if every path from the entry to B passes
/// through A. Here we model this as a simple set of labels.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct DominatorInfo {
    /// Map from block label to its immediate dominator
    pub(super) idom: std::collections::HashMap<u32, u32>,
    /// Map from block label to set of dominated labels
    pub(super) dominated: std::collections::HashMap<u32, std::collections::HashSet<u32>>,
}
impl DominatorInfo {
    /// Create an empty dominator info.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the immediate dominator of `block` to `dom`.
    #[allow(dead_code)]
    pub fn set_idom(&mut self, block: u32, dom: u32) {
        self.idom.insert(block, dom);
        self.dominated.entry(dom).or_default().insert(block);
    }
    /// Get the immediate dominator of `block`.
    #[allow(dead_code)]
    pub fn idom_of(&self, block: u32) -> Option<u32> {
        self.idom.get(&block).copied()
    }
    /// Returns true if `dom` dominates `block`.
    #[allow(dead_code)]
    pub fn dominates(&self, dom: u32, block: u32) -> bool {
        if dom == block {
            return true;
        }
        let mut current = block;
        loop {
            match self.idom.get(&current) {
                Some(&parent) if parent == dom => return true,
                Some(&parent) => current = parent,
                None => return false,
            }
        }
    }
    /// Collect all blocks dominated by `dom`.
    #[allow(dead_code)]
    pub fn dominated_by(&self, dom: u32) -> Vec<u32> {
        let mut result = Vec::new();
        self.collect_dominated(dom, &mut result);
        result
    }
    pub(super) fn collect_dominated(&self, dom: u32, out: &mut Vec<u32>) {
        if let Some(children) = self.dominated.get(&dom) {
            for &child in children {
                out.push(child);
                self.collect_dominated(child, out);
            }
        }
    }
}
