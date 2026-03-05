//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;
use std::collections::{HashMap, HashSet, VecDeque};

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum DCEPassPhase {
    Analysis,
    Transformation,
    Verification,
    Cleanup,
}
impl DCEPassPhase {
    #[allow(dead_code)]
    pub fn name(&self) -> &str {
        match self {
            DCEPassPhase::Analysis => "analysis",
            DCEPassPhase::Transformation => "transformation",
            DCEPassPhase::Verification => "verification",
            DCEPassPhase::Cleanup => "cleanup",
        }
    }
    #[allow(dead_code)]
    pub fn is_modifying(&self) -> bool {
        matches!(self, DCEPassPhase::Transformation | DCEPassPhase::Cleanup)
    }
}
/// Per-variable usage information collected by occurrence analysis.
#[derive(Debug, Clone, Default)]
pub struct UsageInfo {
    /// How many times the variable is referenced.
    pub use_count: usize,
    /// Whether the variable escapes into a closure, constructor field,
    /// or any context where its lifetime is not locally bounded.
    pub is_escaping: bool,
    /// Whether any use of the variable occurs inside a syntactic loop
    /// (i.e., inside a recursive function body or after a back-edge).
    pub is_in_loop: bool,
}
impl UsageInfo {
    /// Create a fresh usage info with zero uses and no flags set.
    pub(super) fn new() -> Self {
        UsageInfo {
            use_count: 0,
            is_escaping: false,
            is_in_loop: false,
        }
    }
    /// Record one additional use of the variable.
    pub(super) fn add_use(&mut self) {
        self.use_count += 1;
    }
    /// Mark the variable as escaping.
    pub(super) fn mark_escaping(&mut self) {
        self.is_escaping = true;
    }
    /// Mark the variable as used inside a loop.
    pub(super) fn mark_in_loop(&mut self) {
        self.is_in_loop = true;
    }
    /// Returns `true` if the variable is dead (zero uses).
    pub fn is_dead(&self) -> bool {
        self.use_count == 0
    }
    /// Returns `true` if the variable is used exactly once.
    pub fn is_once(&self) -> bool {
        self.use_count == 1
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DCEPassConfig {
    pub phase: DCEPassPhase,
    pub enabled: bool,
    pub max_iterations: u32,
    pub debug_output: bool,
    pub pass_name: String,
}
impl DCEPassConfig {
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>, phase: DCEPassPhase) -> Self {
        DCEPassConfig {
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
#[allow(dead_code)]
pub struct DCEPassRegistry {
    pub(super) configs: Vec<DCEPassConfig>,
    pub(super) stats: std::collections::HashMap<String, DCEPassStats>,
}
impl DCEPassRegistry {
    #[allow(dead_code)]
    pub fn new() -> Self {
        DCEPassRegistry {
            configs: Vec::new(),
            stats: std::collections::HashMap::new(),
        }
    }
    #[allow(dead_code)]
    pub fn register(&mut self, config: DCEPassConfig) {
        self.stats
            .insert(config.pass_name.clone(), DCEPassStats::new());
        self.configs.push(config);
    }
    #[allow(dead_code)]
    pub fn enabled_passes(&self) -> Vec<&DCEPassConfig> {
        self.configs.iter().filter(|c| c.enabled).collect()
    }
    #[allow(dead_code)]
    pub fn get_stats(&self, name: &str) -> Option<&DCEPassStats> {
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
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DCECacheEntry {
    pub key: String,
    pub data: Vec<u8>,
    pub timestamp: u64,
    pub valid: bool,
}
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct DCEPassStats {
    pub total_runs: u32,
    pub successful_runs: u32,
    pub total_changes: u64,
    pub time_ms: u64,
    pub iterations_used: u32,
}
impl DCEPassStats {
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
#[derive(Debug, Clone)]
pub struct DCEDominatorTree {
    pub idom: Vec<Option<u32>>,
    pub dom_children: Vec<Vec<u32>>,
    pub dom_depth: Vec<u32>,
}
impl DCEDominatorTree {
    #[allow(dead_code)]
    pub fn new(size: usize) -> Self {
        DCEDominatorTree {
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
pub struct DCEWorklist {
    pub(super) items: std::collections::VecDeque<u32>,
    pub(super) in_worklist: std::collections::HashSet<u32>,
}
impl DCEWorklist {
    #[allow(dead_code)]
    pub fn new() -> Self {
        DCEWorklist {
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
/// Accumulated statistics for a DCE run.
#[derive(Debug, Clone, Default)]
pub struct DceStats {
    /// Number of dead let-bindings removed.
    pub lets_eliminated: usize,
    /// Number of unreachable case alternatives removed.
    pub alts_eliminated: usize,
    /// Number of constant values propagated (and let removed).
    pub constants_propagated: usize,
    /// Number of copy bindings propagated (and let removed).
    pub copies_propagated: usize,
    /// Number of unreachable function declarations removed.
    pub functions_eliminated: usize,
    /// Total number of fixed-point iterations executed.
    pub iterations: usize,
}
impl DceStats {
    /// Total number of transformations applied.
    pub fn total_changes(&self) -> usize {
        self.lets_eliminated
            + self.alts_eliminated
            + self.constants_propagated
            + self.copies_propagated
            + self.functions_eliminated
    }
    /// Merge the statistics from `other` into `self`.
    pub(super) fn merge(&mut self, other: &DceStats) {
        self.lets_eliminated += other.lets_eliminated;
        self.alts_eliminated += other.alts_eliminated;
        self.constants_propagated += other.constants_propagated;
        self.copies_propagated += other.copies_propagated;
        self.functions_eliminated += other.functions_eliminated;
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DCELivenessInfo {
    pub live_in: Vec<std::collections::HashSet<u32>>,
    pub live_out: Vec<std::collections::HashSet<u32>>,
    pub defs: Vec<std::collections::HashSet<u32>>,
    pub uses: Vec<std::collections::HashSet<u32>>,
}
impl DCELivenessInfo {
    #[allow(dead_code)]
    pub fn new(block_count: usize) -> Self {
        DCELivenessInfo {
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
/// A known constant value discovered during analysis.
///
/// This forms a simple two-level lattice:
///   Unknown  (top -- we know nothing)
///      |
///   Lit / Ctor  (known concrete value)
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConstValue {
    /// A literal constant (nat or string).
    Lit(LcnfLit),
    /// A fully applied constructor with known tag and arguments.
    Ctor(String, u32, Vec<LcnfArg>),
    /// Value is not statically known.
    Unknown,
}
impl ConstValue {
    /// Returns `true` if the value is statically known (not Unknown).
    pub fn is_known(&self) -> bool {
        !matches!(self, ConstValue::Unknown)
    }
    /// Attempt to extract a literal from the const value.
    pub fn as_lit(&self) -> Option<&LcnfLit> {
        match self {
            ConstValue::Lit(l) => Some(l),
            _ => None,
        }
    }
    /// Attempt to extract constructor info from the const value.
    pub fn as_ctor(&self) -> Option<(&str, u32, &[LcnfArg])> {
        match self {
            ConstValue::Ctor(name, tag, args) => Some((name.as_str(), *tag, args.as_slice())),
            _ => None,
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DCEAnalysisCache {
    pub(super) entries: std::collections::HashMap<String, DCECacheEntry>,
    pub(super) max_size: usize,
    pub(super) hits: u64,
    pub(super) misses: u64,
}
impl DCEAnalysisCache {
    #[allow(dead_code)]
    pub fn new(max_size: usize) -> Self {
        DCEAnalysisCache {
            entries: std::collections::HashMap::new(),
            max_size,
            hits: 0,
            misses: 0,
        }
    }
    #[allow(dead_code)]
    pub fn get(&mut self, key: &str) -> Option<&DCECacheEntry> {
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
            DCECacheEntry {
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
/// Configuration knobs for the DCE pipeline.
#[derive(Debug, Clone)]
pub struct DceConfig {
    /// Eliminate unused let-bindings.
    pub eliminate_unused_lets: bool,
    /// Eliminate case alternatives that are statically unreachable.
    pub eliminate_unreachable_alts: bool,
    /// Propagate literal constants through let-bindings.
    pub propagate_constants: bool,
    /// Propagate copies (`let x = y`) by substituting `y` for `x`.
    pub propagate_copies: bool,
    /// Fold case expressions whose scrutinee is a known constructor.
    pub fold_known_calls: bool,
    /// Maximum number of fixed-point iterations.
    pub max_iterations: usize,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DCEDepGraph {
    pub(super) nodes: Vec<u32>,
    pub(super) edges: Vec<(u32, u32)>,
}
impl DCEDepGraph {
    #[allow(dead_code)]
    pub fn new() -> Self {
        DCEDepGraph {
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
pub struct DCEConstantFoldingHelper;
impl DCEConstantFoldingHelper {
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
