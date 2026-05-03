//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::defs::*;
use super::impls1::*;
use crate::lcnf::{LcnfArg, LcnfExpr, LcnfFunDecl, LcnfLetValue};
use std::collections::{HashMap, HashSet};

use std::collections::VecDeque;

/// Liveness analysis for OEX2.
impl OELivenessInfo {
    #[allow(dead_code)]
    pub fn new(block_count: usize) -> Self {
        OELivenessInfo {
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
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ConnectionGraph {
    pub nodes: std::collections::HashMap<u32, ConnectionNode>,
    pub edges: Vec<(u32, u32)>,
}
impl ConnectionGraph {
    #[allow(dead_code)]
    pub fn new() -> Self {
        ConnectionGraph {
            nodes: std::collections::HashMap::new(),
            edges: Vec::new(),
        }
    }
    #[allow(dead_code)]
    pub fn add_node(&mut self, id: u32, kind: impl Into<String>) {
        self.nodes.insert(
            id,
            ConnectionNode {
                id,
                escape_state: EscapeStatus::NoEscape,
                kind: kind.into(),
            },
        );
    }
    #[allow(dead_code)]
    pub fn add_deferred_edge(&mut self, from: u32, to: u32) {
        self.edges.push((from, to));
    }
    #[allow(dead_code)]
    pub fn propagate_escape(&mut self) {
        let mut changed = true;
        while changed {
            changed = false;
            let edges_copy = self.edges.clone();
            for (from, to) in edges_copy {
                let from_state = self.nodes.get(&from).map(|n| n.escape_state.clone());
                if let Some(state) = from_state {
                    if let Some(to_node) = self.nodes.get_mut(&to) {
                        match (&state, &to_node.escape_state) {
                            (EscapeStatus::HeapEscape, EscapeStatus::NoEscape)
                            | (EscapeStatus::ReturnEscape, EscapeStatus::NoEscape) => {
                                to_node.escape_state = state;
                                changed = true;
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }
    #[allow(dead_code)]
    pub fn non_escaping_allocations(&self) -> Vec<u32> {
        self.nodes
            .iter()
            .filter(|(_, n)| matches!(n.escape_state, EscapeStatus::NoEscape))
            .map(|(id, _)| *id)
            .collect()
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct OEWorklist {
    pub(crate) items: std::collections::VecDeque<u32>,
    pub(crate) in_worklist: std::collections::HashSet<u32>,
}
impl OEWorklist {
    #[allow(dead_code)]
    pub fn new() -> Self {
        OEWorklist {
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
/// Constant folding helper for OEExt.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct OEExtConstFolder {
    pub(crate) folds: usize,
    pub(crate) failures: usize,
    pub(crate) enabled: bool,
}
impl OEExtConstFolder {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            folds: 0,
            failures: 0,
            enabled: true,
        }
    }
    #[allow(dead_code)]
    pub fn add_i64(&mut self, a: i64, b: i64) -> Option<i64> {
        self.folds += 1;
        a.checked_add(b)
    }
    #[allow(dead_code)]
    pub fn sub_i64(&mut self, a: i64, b: i64) -> Option<i64> {
        self.folds += 1;
        a.checked_sub(b)
    }
    #[allow(dead_code)]
    pub fn mul_i64(&mut self, a: i64, b: i64) -> Option<i64> {
        self.folds += 1;
        a.checked_mul(b)
    }
    #[allow(dead_code)]
    pub fn div_i64(&mut self, a: i64, b: i64) -> Option<i64> {
        if b == 0 {
            self.failures += 1;
            None
        } else {
            self.folds += 1;
            a.checked_div(b)
        }
    }
    #[allow(dead_code)]
    pub fn rem_i64(&mut self, a: i64, b: i64) -> Option<i64> {
        if b == 0 {
            self.failures += 1;
            None
        } else {
            self.folds += 1;
            a.checked_rem(b)
        }
    }
    #[allow(dead_code)]
    pub fn neg_i64(&mut self, a: i64) -> Option<i64> {
        self.folds += 1;
        a.checked_neg()
    }
    #[allow(dead_code)]
    pub fn shl_i64(&mut self, a: i64, s: u32) -> Option<i64> {
        if s >= 64 {
            self.failures += 1;
            None
        } else {
            self.folds += 1;
            a.checked_shl(s)
        }
    }
    #[allow(dead_code)]
    pub fn shr_i64(&mut self, a: i64, s: u32) -> Option<i64> {
        if s >= 64 {
            self.failures += 1;
            None
        } else {
            self.folds += 1;
            a.checked_shr(s)
        }
    }
    #[allow(dead_code)]
    pub fn and_i64(&mut self, a: i64, b: i64) -> i64 {
        self.folds += 1;
        a & b
    }
    #[allow(dead_code)]
    pub fn or_i64(&mut self, a: i64, b: i64) -> i64 {
        self.folds += 1;
        a | b
    }
    #[allow(dead_code)]
    pub fn xor_i64(&mut self, a: i64, b: i64) -> i64 {
        self.folds += 1;
        a ^ b
    }
    #[allow(dead_code)]
    pub fn not_i64(&mut self, a: i64) -> i64 {
        self.folds += 1;
        !a
    }
    #[allow(dead_code)]
    pub fn fold_count(&self) -> usize {
        self.folds
    }
    #[allow(dead_code)]
    pub fn failure_count(&self) -> usize {
        self.failures
    }
    #[allow(dead_code)]
    pub fn enable(&mut self) {
        self.enabled = true;
    }
    #[allow(dead_code)]
    pub fn disable(&mut self) {
        self.enabled = false;
    }
    #[allow(dead_code)]
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum AllocationSinkKind {
    Stack,
    ThreadLocal,
    ArenaAllocated,
    HeapLongLived,
    HeapShortLived,
}
impl AllocationSinkKind {
    #[allow(dead_code)]
    pub fn is_stack_eligible(&self) -> bool {
        matches!(
            self,
            AllocationSinkKind::Stack | AllocationSinkKind::ArenaAllocated
        )
    }
    #[allow(dead_code)]
    pub fn description(&self) -> &str {
        match self {
            AllocationSinkKind::Stack => "stack allocation",
            AllocationSinkKind::ThreadLocal => "thread-local storage",
            AllocationSinkKind::ArenaAllocated => "arena allocation",
            AllocationSinkKind::HeapLongLived => "long-lived heap",
            AllocationSinkKind::HeapShortLived => "short-lived heap",
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct EscapeBasedRefCountOpt {
    pub eliminated_increments: u32,
    pub eliminated_decrements: u32,
    pub replaced_with_stack: Vec<u32>,
}
impl EscapeBasedRefCountOpt {
    #[allow(dead_code)]
    pub fn new() -> Self {
        EscapeBasedRefCountOpt {
            eliminated_increments: 0,
            eliminated_decrements: 0,
            replaced_with_stack: Vec::new(),
        }
    }
    #[allow(dead_code)]
    pub fn record_elimination(&mut self) {
        self.eliminated_increments += 1;
        self.eliminated_decrements += 1;
    }
    #[allow(dead_code)]
    pub fn record_stack_replace(&mut self, alloc_id: u32) {
        self.replaced_with_stack.push(alloc_id);
    }
    #[allow(dead_code)]
    pub fn total_eliminated(&self) -> u32 {
        self.eliminated_increments + self.eliminated_decrements
    }
    #[allow(dead_code)]
    pub fn savings_report(&self) -> String {
        format!(
            "Eliminated {} retain/release pairs, {} stack promotions",
            self.eliminated_increments,
            self.replaced_with_stack.len()
        )
    }
}
/// Analysis cache for OEX2.
#[allow(dead_code)]
#[derive(Debug)]
pub struct OEX2Cache {
    pub(crate) entries: Vec<(u64, Vec<u8>, bool, u32)>,
    pub(crate) cap: usize,
    pub(crate) total_hits: u64,
    pub(crate) total_misses: u64,
}
impl OEX2Cache {
    #[allow(dead_code)]
    pub fn new(cap: usize) -> Self {
        Self {
            entries: Vec::new(),
            cap,
            total_hits: 0,
            total_misses: 0,
        }
    }
    #[allow(dead_code)]
    pub fn get(&mut self, key: u64) -> Option<&[u8]> {
        for e in self.entries.iter_mut() {
            if e.0 == key && e.2 {
                e.3 += 1;
                self.total_hits += 1;
                return Some(&e.1);
            }
        }
        self.total_misses += 1;
        None
    }
    #[allow(dead_code)]
    pub fn put(&mut self, key: u64, data: Vec<u8>) {
        if self.entries.len() >= self.cap {
            self.entries.retain(|e| e.2);
            if self.entries.len() >= self.cap {
                self.entries.remove(0);
            }
        }
        self.entries.push((key, data, true, 0));
    }
    #[allow(dead_code)]
    pub fn invalidate(&mut self) {
        for e in self.entries.iter_mut() {
            e.2 = false;
        }
    }
    #[allow(dead_code)]
    pub fn hit_rate(&self) -> f64 {
        let t = self.total_hits + self.total_misses;
        if t == 0 {
            0.0
        } else {
            self.total_hits as f64 / t as f64
        }
    }
    #[allow(dead_code)]
    pub fn live_count(&self) -> usize {
        self.entries.iter().filter(|e| e.2).count()
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum OEPassPhase {
    Analysis,
    Transformation,
    Verification,
    Cleanup,
}
impl OEPassPhase {
    #[allow(dead_code)]
    pub fn name(&self) -> &str {
        match self {
            OEPassPhase::Analysis => "analysis",
            OEPassPhase::Transformation => "transformation",
            OEPassPhase::Verification => "verification",
            OEPassPhase::Cleanup => "cleanup",
        }
    }
    #[allow(dead_code)]
    pub fn is_modifying(&self) -> bool {
        matches!(self, OEPassPhase::Transformation | OEPassPhase::Cleanup)
    }
}
/// Configuration options for the escape-based stack-allocation optimization.
#[derive(Debug, Clone)]
pub struct EscapeOptConfig {
    /// Whether to emit stack-allocation hints.
    pub enable_stack_alloc: bool,
    /// Maximum object size (bytes) eligible for stack allocation.
    pub max_stack_size_bytes: u64,
    /// In aggressive mode, `LocalEscape` allocations are also stack-allocated.
    pub aggressive_mode: bool,
}
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct PointsToSet2 {
    pub targets: std::collections::HashSet<PointsToTarget>,
}
impl PointsToSet2 {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    #[allow(dead_code)]
    pub fn add(&mut self, target: PointsToTarget) -> bool {
        self.targets.insert(target)
    }
    #[allow(dead_code)]
    pub fn may_alias(&self, other: &PointsToSet2) -> bool {
        self.targets.iter().any(|t| other.targets.contains(t))
    }
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.targets.is_empty()
    }
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.targets.len()
    }
    #[allow(dead_code)]
    pub fn union(&mut self, other: &PointsToSet2) -> bool {
        let before = self.targets.len();
        self.targets.extend(other.targets.iter().cloned());
        self.targets.len() > before
    }
}
#[allow(dead_code)]
pub struct OEConstantFoldingHelper;
impl OEConstantFoldingHelper {
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
/// Configuration for OEX2 passes.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct OEX2PassConfig {
    pub name: String,
    pub phase: OEX2PassPhase,
    pub enabled: bool,
    pub max_iterations: usize,
    pub debug: u32,
    pub timeout_ms: Option<u64>,
}
impl OEX2PassConfig {
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            phase: OEX2PassPhase::Middle,
            enabled: true,
            max_iterations: 100,
            debug: 0,
            timeout_ms: None,
        }
    }
    #[allow(dead_code)]
    pub fn with_phase(mut self, phase: OEX2PassPhase) -> Self {
        self.phase = phase;
        self
    }
    #[allow(dead_code)]
    pub fn with_max_iter(mut self, n: usize) -> Self {
        self.max_iterations = n;
        self
    }
    #[allow(dead_code)]
    pub fn with_debug(mut self, d: u32) -> Self {
        self.debug = d;
        self
    }
    #[allow(dead_code)]
    pub fn disabled(mut self) -> Self {
        self.enabled = false;
        self
    }
    #[allow(dead_code)]
    pub fn with_timeout(mut self, ms: u64) -> Self {
        self.timeout_ms = Some(ms);
        self
    }
    #[allow(dead_code)]
    pub fn is_debug_enabled(&self) -> bool {
        self.debug > 0
    }
}
/// Pass registry for OEX2.
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct OEX2PassRegistry {
    pub(crate) configs: Vec<OEX2PassConfig>,
    pub(crate) stats: Vec<OEX2PassStats>,
}
impl OEX2PassRegistry {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    #[allow(dead_code)]
    pub fn register(&mut self, c: OEX2PassConfig) {
        self.stats.push(OEX2PassStats::new());
        self.configs.push(c);
    }
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.configs.len()
    }
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.configs.is_empty()
    }
    #[allow(dead_code)]
    pub fn get(&self, i: usize) -> Option<&OEX2PassConfig> {
        self.configs.get(i)
    }
    #[allow(dead_code)]
    pub fn get_stats(&self, i: usize) -> Option<&OEX2PassStats> {
        self.stats.get(i)
    }
    #[allow(dead_code)]
    pub fn enabled_passes(&self) -> Vec<&OEX2PassConfig> {
        self.configs.iter().filter(|c| c.enabled).collect()
    }
    #[allow(dead_code)]
    pub fn passes_in_phase(&self, ph: &OEX2PassPhase) -> Vec<&OEX2PassConfig> {
        self.configs
            .iter()
            .filter(|c| c.enabled && &c.phase == ph)
            .collect()
    }
    #[allow(dead_code)]
    pub fn total_nodes_visited(&self) -> usize {
        self.stats.iter().map(|s| s.nodes_visited).sum()
    }
    #[allow(dead_code)]
    pub fn any_changed(&self) -> bool {
        self.stats.iter().any(|s| s.changed)
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum EscapeEdgeKind {
    DirectAssign,
    FieldWrite { field: String },
    FieldRead { field: String },
    ArrayWrite,
    ArrayRead,
    Return,
    CallArg { arg_index: u32 },
    CallRet,
    CapturedByLambda,
    GlobalWrite,
}
/// Analysis cache for OEExt.
#[allow(dead_code)]
#[derive(Debug)]
pub struct OEExtCache {
    pub(crate) entries: Vec<(u64, Vec<u8>, bool, u32)>,
    pub(crate) cap: usize,
    pub(crate) total_hits: u64,
    pub(crate) total_misses: u64,
}
impl OEExtCache {
    #[allow(dead_code)]
    pub fn new(cap: usize) -> Self {
        Self {
            entries: Vec::new(),
            cap,
            total_hits: 0,
            total_misses: 0,
        }
    }
    #[allow(dead_code)]
    pub fn get(&mut self, key: u64) -> Option<&[u8]> {
        for e in self.entries.iter_mut() {
            if e.0 == key && e.2 {
                e.3 += 1;
                self.total_hits += 1;
                return Some(&e.1);
            }
        }
        self.total_misses += 1;
        None
    }
    #[allow(dead_code)]
    pub fn put(&mut self, key: u64, data: Vec<u8>) {
        if self.entries.len() >= self.cap {
            self.entries.retain(|e| e.2);
            if self.entries.len() >= self.cap {
                self.entries.remove(0);
            }
        }
        self.entries.push((key, data, true, 0));
    }
    #[allow(dead_code)]
    pub fn invalidate(&mut self) {
        for e in self.entries.iter_mut() {
            e.2 = false;
        }
    }
    #[allow(dead_code)]
    pub fn hit_rate(&self) -> f64 {
        let t = self.total_hits + self.total_misses;
        if t == 0 {
            0.0
        } else {
            self.total_hits as f64 / t as f64
        }
    }
    #[allow(dead_code)]
    pub fn live_count(&self) -> usize {
        self.entries.iter().filter(|e| e.2).count()
    }
}
/// A set of variable names that are known to escape.
#[derive(Debug, Clone, Default)]
pub struct EscapeSet {
    pub(crate) escapees: HashSet<String>,
}
impl EscapeSet {
    /// Create an empty escape set.
    pub fn new() -> Self {
        EscapeSet {
            escapees: HashSet::new(),
        }
    }
    /// Mark `var` as escaping.
    pub fn insert(&mut self, var: &str) {
        self.escapees.insert(var.to_owned());
    }
    /// Returns `true` if `var` is in the escape set.
    pub fn contains(&self, var: &str) -> bool {
        self.escapees.contains(var)
    }
    /// Number of escaping variables.
    pub fn len(&self) -> usize {
        self.escapees.len()
    }
    /// Returns `true` if the escape set is empty.
    pub fn is_empty(&self) -> bool {
        self.escapees.is_empty()
    }
}
#[allow(dead_code)]
pub struct OEPassRegistry {
    pub(crate) configs: Vec<OEPassConfig>,
    pub(crate) stats: std::collections::HashMap<String, OEPassStats>,
}
impl OEPassRegistry {
    #[allow(dead_code)]
    pub fn new() -> Self {
        OEPassRegistry {
            configs: Vec::new(),
            stats: std::collections::HashMap::new(),
        }
    }
    #[allow(dead_code)]
    pub fn register(&mut self, config: OEPassConfig) {
        self.stats
            .insert(config.pass_name.clone(), OEPassStats::new());
        self.configs.push(config);
    }
    #[allow(dead_code)]
    pub fn enabled_passes(&self) -> Vec<&OEPassConfig> {
        self.configs.iter().filter(|c| c.enabled).collect()
    }
    #[allow(dead_code)]
    pub fn get_stats(&self, name: &str) -> Option<&OEPassStats> {
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
/// Constant folding helper for OEX2.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct OEX2ConstFolder {
    pub(crate) folds: usize,
    pub(crate) failures: usize,
    pub(crate) enabled: bool,
}
impl OEX2ConstFolder {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            folds: 0,
            failures: 0,
            enabled: true,
        }
    }
    #[allow(dead_code)]
    pub fn add_i64(&mut self, a: i64, b: i64) -> Option<i64> {
        self.folds += 1;
        a.checked_add(b)
    }
    #[allow(dead_code)]
    pub fn sub_i64(&mut self, a: i64, b: i64) -> Option<i64> {
        self.folds += 1;
        a.checked_sub(b)
    }
    #[allow(dead_code)]
    pub fn mul_i64(&mut self, a: i64, b: i64) -> Option<i64> {
        self.folds += 1;
        a.checked_mul(b)
    }
    #[allow(dead_code)]
    pub fn div_i64(&mut self, a: i64, b: i64) -> Option<i64> {
        if b == 0 {
            self.failures += 1;
            None
        } else {
            self.folds += 1;
            a.checked_div(b)
        }
    }
    #[allow(dead_code)]
    pub fn rem_i64(&mut self, a: i64, b: i64) -> Option<i64> {
        if b == 0 {
            self.failures += 1;
            None
        } else {
            self.folds += 1;
            a.checked_rem(b)
        }
    }
    #[allow(dead_code)]
    pub fn neg_i64(&mut self, a: i64) -> Option<i64> {
        self.folds += 1;
        a.checked_neg()
    }
    #[allow(dead_code)]
    pub fn shl_i64(&mut self, a: i64, s: u32) -> Option<i64> {
        if s >= 64 {
            self.failures += 1;
            None
        } else {
            self.folds += 1;
            a.checked_shl(s)
        }
    }
    #[allow(dead_code)]
    pub fn shr_i64(&mut self, a: i64, s: u32) -> Option<i64> {
        if s >= 64 {
            self.failures += 1;
            None
        } else {
            self.folds += 1;
            a.checked_shr(s)
        }
    }
    #[allow(dead_code)]
    pub fn and_i64(&mut self, a: i64, b: i64) -> i64 {
        self.folds += 1;
        a & b
    }
    #[allow(dead_code)]
    pub fn or_i64(&mut self, a: i64, b: i64) -> i64 {
        self.folds += 1;
        a | b
    }
    #[allow(dead_code)]
    pub fn xor_i64(&mut self, a: i64, b: i64) -> i64 {
        self.folds += 1;
        a ^ b
    }
    #[allow(dead_code)]
    pub fn not_i64(&mut self, a: i64) -> i64 {
        self.folds += 1;
        !a
    }
    #[allow(dead_code)]
    pub fn fold_count(&self) -> usize {
        self.folds
    }
    #[allow(dead_code)]
    pub fn failure_count(&self) -> usize {
        self.failures
    }
    #[allow(dead_code)]
    pub fn enable(&mut self) {
        self.enabled = true;
    }
    #[allow(dead_code)]
    pub fn disable(&mut self) {
        self.enabled = false;
    }
    #[allow(dead_code)]
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}
/// Worklist for OEExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct OEExtWorklist {
    pub(crate) items: std::collections::VecDeque<usize>,
    pub(crate) present: Vec<bool>,
}
impl OEExtWorklist {
    #[allow(dead_code)]
    pub fn new(capacity: usize) -> Self {
        Self {
            items: std::collections::VecDeque::new(),
            present: vec![false; capacity],
        }
    }
    #[allow(dead_code)]
    pub fn push(&mut self, id: usize) {
        if id < self.present.len() && !self.present[id] {
            self.present[id] = true;
            self.items.push_back(id);
        }
    }
    #[allow(dead_code)]
    pub fn push_front(&mut self, id: usize) {
        if id < self.present.len() && !self.present[id] {
            self.present[id] = true;
            self.items.push_front(id);
        }
    }
    #[allow(dead_code)]
    pub fn pop(&mut self) -> Option<usize> {
        let id = self.items.pop_front()?;
        if id < self.present.len() {
            self.present[id] = false;
        }
        Some(id)
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
    pub fn contains(&self, id: usize) -> bool {
        id < self.present.len() && self.present[id]
    }
    #[allow(dead_code)]
    pub fn drain_all(&mut self) -> Vec<usize> {
        let v: Vec<usize> = self.items.drain(..).collect();
        for &id in &v {
            if id < self.present.len() {
                self.present[id] = false;
            }
        }
        v
    }
}
