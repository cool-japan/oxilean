//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;
use std::collections::{HashMap, HashSet};

use super::functions::*;
use std::collections::VecDeque;

/// Configuration for join point optimization
#[derive(Debug, Clone)]
pub struct JoinPointConfig {
    /// Maximum size (in instructions) for a join point to be inlined
    pub max_join_size: usize,
    /// Whether to inline small join points
    pub inline_small_joins: bool,
    /// Whether to detect and mark tail calls
    pub detect_tail_calls: bool,
    /// Whether to perform contification
    pub enable_contification: bool,
    /// Whether to float join points closer to uses
    pub float_join_points: bool,
    /// Whether to eliminate dead join points
    pub eliminate_dead_joins: bool,
    /// Maximum number of optimization iterations
    pub max_iterations: usize,
}
/// A generic key-value configuration store for OJoin.
#[derive(Debug, Clone, Default)]
pub struct OJoinConfig {
    pub(super) entries: std::collections::HashMap<String, String>,
}
impl OJoinConfig {
    pub fn new() -> Self {
        OJoinConfig::default()
    }
    pub fn set(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.entries.insert(key.into(), value.into());
    }
    pub fn get(&self, key: &str) -> Option<&str> {
        self.entries.get(key).map(|s| s.as_str())
    }
    pub fn get_bool(&self, key: &str) -> bool {
        matches!(self.get(key), Some("true") | Some("1") | Some("yes"))
    }
    pub fn get_int(&self, key: &str) -> Option<i64> {
        self.get(key)?.parse().ok()
    }
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}
/// A feature flag set for OJoin capabilities.
#[derive(Debug, Clone, Default)]
pub struct OJoinFeatures {
    pub(super) flags: std::collections::HashSet<String>,
}
impl OJoinFeatures {
    pub fn new() -> Self {
        OJoinFeatures::default()
    }
    pub fn enable(&mut self, flag: impl Into<String>) {
        self.flags.insert(flag.into());
    }
    pub fn disable(&mut self, flag: &str) {
        self.flags.remove(flag);
    }
    pub fn is_enabled(&self, flag: &str) -> bool {
        self.flags.contains(flag)
    }
    pub fn len(&self) -> usize {
        self.flags.len()
    }
    pub fn is_empty(&self) -> bool {
        self.flags.is_empty()
    }
    pub fn union(&self, other: &OJoinFeatures) -> OJoinFeatures {
        OJoinFeatures {
            flags: self.flags.union(&other.flags).cloned().collect(),
        }
    }
    pub fn intersection(&self, other: &OJoinFeatures) -> OJoinFeatures {
        OJoinFeatures {
            flags: self.flags.intersection(&other.flags).cloned().collect(),
        }
    }
}
#[allow(dead_code)]
pub struct OJConstantFoldingHelper;
impl OJConstantFoldingHelper {
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
/// Information about a function call site
#[derive(Debug, Clone)]
pub struct CallSiteInfo {
    /// The calling function
    pub(super) caller: String,
    /// Whether this call is in tail position
    pub(super) is_tail: bool,
    /// Number of arguments passed
    pub(super) arg_count: usize,
    /// The variable ID of the callee if it's a local var
    pub(super) callee_var: Option<LcnfVarId>,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct OJDepGraph {
    pub(super) nodes: Vec<u32>,
    pub(super) edges: Vec<(u32, u32)>,
}
impl OJDepGraph {
    #[allow(dead_code)]
    pub fn new() -> Self {
        OJDepGraph {
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
/// Statistics for join point optimization
#[derive(Debug, Clone, Default)]
pub struct JoinPointStats {
    /// Number of join points created
    pub joins_created: usize,
    /// Number of join points inlined
    pub joins_inlined: usize,
    /// Number of dead join points eliminated
    pub joins_eliminated: usize,
    /// Number of tail calls detected
    pub tail_calls_detected: usize,
    /// Number of functions contified
    pub functions_contified: usize,
    /// Number of join points floated
    pub joins_floated: usize,
    /// Total optimization iterations run
    pub iterations: usize,
}
impl JoinPointStats {
    pub(super) fn total_changes(&self) -> usize {
        self.joins_created
            + self.joins_inlined
            + self.joins_eliminated
            + self.tail_calls_detected
            + self.functions_contified
            + self.joins_floated
    }
}
/// A text buffer for building OJoin output source code.
#[derive(Debug, Default)]
pub struct OJoinSourceBuffer {
    pub(super) buf: String,
    pub(super) indent_level: usize,
    pub(super) indent_str: String,
}
impl OJoinSourceBuffer {
    pub fn new() -> Self {
        OJoinSourceBuffer {
            buf: String::new(),
            indent_level: 0,
            indent_str: "    ".to_string(),
        }
    }
    pub fn with_indent(mut self, indent: impl Into<String>) -> Self {
        self.indent_str = indent.into();
        self
    }
    pub fn push_line(&mut self, line: &str) {
        for _ in 0..self.indent_level {
            self.buf.push_str(&self.indent_str);
        }
        self.buf.push_str(line);
        self.buf.push('\n');
    }
    pub fn push_raw(&mut self, s: &str) {
        self.buf.push_str(s);
    }
    pub fn indent(&mut self) {
        self.indent_level += 1;
    }
    pub fn dedent(&mut self) {
        self.indent_level = self.indent_level.saturating_sub(1);
    }
    pub fn as_str(&self) -> &str {
        &self.buf
    }
    pub fn len(&self) -> usize {
        self.buf.len()
    }
    pub fn is_empty(&self) -> bool {
        self.buf.is_empty()
    }
    pub fn line_count(&self) -> usize {
        self.buf.lines().count()
    }
    pub fn into_string(self) -> String {
        self.buf
    }
    pub fn reset(&mut self) {
        self.buf.clear();
        self.indent_level = 0;
    }
}
/// Tracks declared names for OJoin scope analysis.
#[derive(Debug, Default)]
pub struct OJoinNameScope {
    pub(super) declared: std::collections::HashSet<String>,
    pub(super) depth: usize,
    pub(super) parent: Option<Box<OJoinNameScope>>,
}
impl OJoinNameScope {
    pub fn new() -> Self {
        OJoinNameScope::default()
    }
    pub fn declare(&mut self, name: impl Into<String>) -> bool {
        self.declared.insert(name.into())
    }
    pub fn is_declared(&self, name: &str) -> bool {
        self.declared.contains(name)
    }
    pub fn push_scope(self) -> Self {
        OJoinNameScope {
            declared: std::collections::HashSet::new(),
            depth: self.depth + 1,
            parent: Some(Box::new(self)),
        }
    }
    pub fn pop_scope(self) -> Self {
        *self.parent.unwrap_or_default()
    }
    pub fn depth(&self) -> usize {
        self.depth
    }
    pub fn len(&self) -> usize {
        self.declared.len()
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct OJAnalysisCache {
    pub(super) entries: std::collections::HashMap<String, OJCacheEntry>,
    pub(super) max_size: usize,
    pub(super) hits: u64,
    pub(super) misses: u64,
}
impl OJAnalysisCache {
    #[allow(dead_code)]
    pub fn new(max_size: usize) -> Self {
        OJAnalysisCache {
            entries: std::collections::HashMap::new(),
            max_size,
            hits: 0,
            misses: 0,
        }
    }
    #[allow(dead_code)]
    pub fn get(&mut self, key: &str) -> Option<&OJCacheEntry> {
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
            OJCacheEntry {
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
/// A monotonically increasing ID generator for OJoin.
#[derive(Debug, Default)]
pub struct OJoinIdGen {
    pub(super) next: u32,
}
impl OJoinIdGen {
    pub fn new() -> Self {
        OJoinIdGen::default()
    }
    pub fn next_id(&mut self) -> u32 {
        let id = self.next;
        self.next += 1;
        id
    }
    pub fn peek_next(&self) -> u32 {
        self.next
    }
    pub fn reset(&mut self) {
        self.next = 0;
    }
    pub fn skip(&mut self, n: u32) {
        self.next += n;
    }
}
/// Heuristic freshness key for OJoin incremental compilation.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OJoinIncrKey {
    pub content_hash: u64,
    pub config_hash: u64,
}
impl OJoinIncrKey {
    pub fn new(content: u64, config: u64) -> Self {
        OJoinIncrKey {
            content_hash: content,
            config_hash: config,
        }
    }
    pub fn combined_hash(&self) -> u64 {
        self.content_hash.wrapping_mul(0x9e3779b97f4a7c15) ^ self.config_hash
    }
    pub fn matches(&self, other: &OJoinIncrKey) -> bool {
        self.content_hash == other.content_hash && self.config_hash == other.config_hash
    }
}
/// A diagnostic message from a OJoin pass.
#[derive(Debug, Clone)]
pub struct OJoinDiagMsg {
    pub severity: OJoinDiagSeverity,
    pub pass: String,
    pub message: String,
}
impl OJoinDiagMsg {
    pub fn error(pass: impl Into<String>, msg: impl Into<String>) -> Self {
        OJoinDiagMsg {
            severity: OJoinDiagSeverity::Error,
            pass: pass.into(),
            message: msg.into(),
        }
    }
    pub fn warning(pass: impl Into<String>, msg: impl Into<String>) -> Self {
        OJoinDiagMsg {
            severity: OJoinDiagSeverity::Warning,
            pass: pass.into(),
            message: msg.into(),
        }
    }
    pub fn note(pass: impl Into<String>, msg: impl Into<String>) -> Self {
        OJoinDiagMsg {
            severity: OJoinDiagSeverity::Note,
            pass: pass.into(),
            message: msg.into(),
        }
    }
}
/// A version tag for OJoin output artifacts.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OJoinVersion {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
    pub pre: Option<String>,
}
impl OJoinVersion {
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        OJoinVersion {
            major,
            minor,
            patch,
            pre: None,
        }
    }
    pub fn with_pre(mut self, pre: impl Into<String>) -> Self {
        self.pre = Some(pre.into());
        self
    }
    pub fn is_stable(&self) -> bool {
        self.pre.is_none()
    }
    pub fn is_compatible_with(&self, other: &OJoinVersion) -> bool {
        self.major == other.major && self.minor >= other.minor
    }
}
/// Collects OJoin diagnostics.
#[derive(Debug, Default)]
pub struct OJoinDiagCollector {
    pub(super) msgs: Vec<OJoinDiagMsg>,
}
impl OJoinDiagCollector {
    pub fn new() -> Self {
        OJoinDiagCollector::default()
    }
    pub fn emit(&mut self, d: OJoinDiagMsg) {
        self.msgs.push(d);
    }
    pub fn has_errors(&self) -> bool {
        self.msgs
            .iter()
            .any(|d| d.severity == OJoinDiagSeverity::Error)
    }
    pub fn errors(&self) -> Vec<&OJoinDiagMsg> {
        self.msgs
            .iter()
            .filter(|d| d.severity == OJoinDiagSeverity::Error)
            .collect()
    }
    pub fn warnings(&self) -> Vec<&OJoinDiagMsg> {
        self.msgs
            .iter()
            .filter(|d| d.severity == OJoinDiagSeverity::Warning)
            .collect()
    }
    pub fn len(&self) -> usize {
        self.msgs.len()
    }
    pub fn is_empty(&self) -> bool {
        self.msgs.is_empty()
    }
    pub fn clear(&mut self) {
        self.msgs.clear();
    }
}
/// Pipeline profiler for OJoin.
#[derive(Debug, Default)]
pub struct OJoinProfiler {
    pub(super) timings: Vec<OJoinPassTiming>,
}
impl OJoinProfiler {
    pub fn new() -> Self {
        OJoinProfiler::default()
    }
    pub fn record(&mut self, t: OJoinPassTiming) {
        self.timings.push(t);
    }
    pub fn total_elapsed_us(&self) -> u64 {
        self.timings.iter().map(|t| t.elapsed_us).sum()
    }
    pub fn slowest_pass(&self) -> Option<&OJoinPassTiming> {
        self.timings.iter().max_by_key(|t| t.elapsed_us)
    }
    pub fn num_passes(&self) -> usize {
        self.timings.len()
    }
    pub fn profitable_passes(&self) -> Vec<&OJoinPassTiming> {
        self.timings.iter().filter(|t| t.is_profitable()).collect()
    }
}
#[allow(dead_code)]
pub struct OJPassRegistry {
    pub(super) configs: Vec<OJPassConfig>,
    pub(super) stats: std::collections::HashMap<String, OJPassStats>,
}
impl OJPassRegistry {
    #[allow(dead_code)]
    pub fn new() -> Self {
        OJPassRegistry {
            configs: Vec::new(),
            stats: std::collections::HashMap::new(),
        }
    }
    #[allow(dead_code)]
    pub fn register(&mut self, config: OJPassConfig) {
        self.stats
            .insert(config.pass_name.clone(), OJPassStats::new());
        self.configs.push(config);
    }
    #[allow(dead_code)]
    pub fn enabled_passes(&self) -> Vec<&OJPassConfig> {
        self.configs.iter().filter(|c| c.enabled).collect()
    }
    #[allow(dead_code)]
    pub fn get_stats(&self, name: &str) -> Option<&OJPassStats> {
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
/// Main join point optimizer
pub struct JoinPointOptimizer {
    pub(super) config: JoinPointConfig,
    pub(super) stats: JoinPointStats,
    pub(super) next_id: u64,
}
impl JoinPointOptimizer {
    /// Create a new join point optimizer with the given configuration
    pub fn new(config: JoinPointConfig) -> Self {
        JoinPointOptimizer {
            config,
            stats: JoinPointStats::default(),
            next_id: 1000,
        }
    }
    /// Get the optimization statistics
    pub fn stats(&self) -> &JoinPointStats {
        &self.stats
    }
    /// Generate a fresh variable ID
    pub(super) fn fresh_id(&mut self) -> LcnfVarId {
        let id = self.next_id;
        self.next_id += 1;
        LcnfVarId(id)
    }
    /// Optimize a single function declaration
    pub(super) fn optimize_decl(&mut self, decl: &mut LcnfFunDecl) {
        for _ in 0..self.config.max_iterations {
            let changes_before = self.stats.total_changes();
            if self.config.detect_tail_calls {
                self.detect_tail_calls_in_expr(&mut decl.body, &decl.name);
            }
            if self.config.inline_small_joins {
                self.inline_small_joins(&mut decl.body);
            }
            if self.config.eliminate_dead_joins {
                self.eliminate_dead_joins(&mut decl.body);
            }
            if self.config.enable_contification {
                self.contify_functions(&mut decl.body);
            }
            if self.config.float_join_points {
                self.float_joins(&mut decl.body);
            }
            self.stats.iterations += 1;
            if self.stats.total_changes() == changes_before {
                break;
            }
        }
    }
    /// Detect tail calls in an expression and convert App to TailCall where appropriate
    pub(super) fn detect_tail_calls_in_expr(&mut self, expr: &mut LcnfExpr, _current_fn: &str) {
        let should_convert = if let LcnfExpr::Let {
            id,
            value: LcnfLetValue::App(func, args),
            body,
            ..
        } = &*expr
        {
            if let LcnfExpr::Return(LcnfArg::Var(ret_var)) = body.as_ref() {
                if *ret_var == *id {
                    Some((func.clone(), args.clone()))
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        };
        if let Some((func, args)) = should_convert {
            *expr = LcnfExpr::TailCall(func, args);
            self.stats.tail_calls_detected += 1;
            return;
        }
        match expr {
            LcnfExpr::Let { body, .. } => {
                self.detect_tail_calls_in_expr(body, _current_fn);
            }
            LcnfExpr::Case { alts, default, .. } => {
                for alt in alts.iter_mut() {
                    self.detect_tail_calls_in_expr(&mut alt.body, _current_fn);
                }
                if let Some(def) = default {
                    self.detect_tail_calls_in_expr(def, _current_fn);
                }
            }
            LcnfExpr::Return(_) | LcnfExpr::Unreachable | LcnfExpr::TailCall(_, _) => {}
        }
    }
    /// Inline small join points (those with few instructions)
    pub(super) fn inline_small_joins(&mut self, expr: &mut LcnfExpr) {
        let small_joins = self.find_small_joins(expr);
        if !small_joins.is_empty() {
            self.apply_join_inlining(expr, &small_joins);
        }
    }
    /// Find let-bound values that are small enough to inline
    pub(super) fn find_small_joins(&self, expr: &LcnfExpr) -> HashMap<LcnfVarId, LcnfLetValue> {
        let mut joins = HashMap::new();
        match expr {
            LcnfExpr::Let {
                id, value, body, ..
            } => {
                let size = self.value_size(value);
                if size <= self.config.max_join_size {
                    joins.insert(*id, value.clone());
                }
                joins.extend(self.find_small_joins(body));
            }
            LcnfExpr::Case { alts, default, .. } => {
                for alt in alts {
                    joins.extend(self.find_small_joins(&alt.body));
                }
                if let Some(def) = default {
                    joins.extend(self.find_small_joins(def));
                }
            }
            _ => {}
        }
        joins
    }
    /// Compute the "size" of a let-value for inlining decisions
    pub(super) fn value_size(&self, value: &LcnfLetValue) -> usize {
        match value {
            LcnfLetValue::Lit(_)
            | LcnfLetValue::Erased
            | LcnfLetValue::FVar(_)
            | LcnfLetValue::Reset(_)
            | LcnfLetValue::Reuse(_, _, _, _) => 1,
            LcnfLetValue::Proj(_, _, _) => 1,
            LcnfLetValue::App(_, args) => 1 + args.len(),
            LcnfLetValue::Ctor(_, _, args) => 1 + args.len(),
        }
    }
    /// Apply inlining of small join points: replace FVar references
    pub(super) fn apply_join_inlining(
        &mut self,
        expr: &mut LcnfExpr,
        joins: &HashMap<LcnfVarId, LcnfLetValue>,
    ) {
        match expr {
            LcnfExpr::Let {
                id, value, body, ..
            } => {
                if let LcnfLetValue::FVar(ref fvar) = value {
                    if let Some(replacement) = joins.get(fvar) {
                        if *id != *fvar {
                            *value = replacement.clone();
                            self.stats.joins_inlined += 1;
                        }
                    }
                }
                self.apply_join_inlining(body, joins);
            }
            LcnfExpr::Case { alts, default, .. } => {
                for alt in alts.iter_mut() {
                    self.apply_join_inlining(&mut alt.body, joins);
                }
                if let Some(def) = default {
                    self.apply_join_inlining(def, joins);
                }
            }
            _ => {}
        }
    }
    /// Eliminate dead join points (unreferenced let-bindings that are pure)
    pub(super) fn eliminate_dead_joins(&mut self, expr: &mut LcnfExpr) {
        let used = collect_used_vars(expr);
        self.remove_dead_lets(expr, &used);
    }
    /// Remove let-bindings for variables that are never used
    pub(super) fn remove_dead_lets(&mut self, expr: &mut LcnfExpr, used: &HashSet<LcnfVarId>) {
        loop {
            let mut changed = false;
            if let LcnfExpr::Let {
                id, value, body, ..
            } = expr
            {
                if !used.contains(id) && is_pure_value(value) {
                    *expr = *body.clone();
                    self.stats.joins_eliminated += 1;
                    changed = true;
                }
            }
            if !changed {
                break;
            }
        }
        match expr {
            LcnfExpr::Let { body, .. } => {
                self.remove_dead_lets(body, used);
            }
            LcnfExpr::Case { alts, default, .. } => {
                for alt in alts.iter_mut() {
                    self.remove_dead_lets(&mut alt.body, used);
                }
                if let Some(def) = default {
                    self.remove_dead_lets(def, used);
                }
            }
            _ => {}
        }
    }
    /// Convert functions that are always called in tail position to join points
    pub(super) fn contify_functions(&mut self, expr: &mut LcnfExpr) {
        let tail_uses = analyze_tail_uses(expr, true);
        let candidates: Vec<LcnfVarId> = tail_uses
            .iter()
            .filter(|(_, use_kind)| **use_kind == TailUse::TailOnly)
            .map(|(var, _)| *var)
            .collect();
        if !candidates.is_empty() {
            self.mark_contified(expr, &candidates);
        }
    }
    /// Mark let-bound functions as contified (join points)
    pub(super) fn mark_contified(&mut self, expr: &mut LcnfExpr, candidates: &[LcnfVarId]) {
        match expr {
            LcnfExpr::Let { id, body, .. } => {
                if candidates.contains(id) {
                    self.stats.functions_contified += 1;
                }
                self.mark_contified(body, candidates);
            }
            LcnfExpr::Case { alts, default, .. } => {
                for alt in alts.iter_mut() {
                    self.mark_contified(&mut alt.body, candidates);
                }
                if let Some(def) = default {
                    self.mark_contified(def, candidates);
                }
            }
            _ => {}
        }
    }
    /// Float join points closer to their uses
    pub(super) fn float_joins(&mut self, expr: &mut LcnfExpr) {
        let moved = self.try_float_into_case(expr);
        if moved {
            self.stats.joins_floated += 1;
        }
        match expr {
            LcnfExpr::Let { body, .. } => {
                self.float_joins(body);
            }
            LcnfExpr::Case { alts, default, .. } => {
                for alt in alts.iter_mut() {
                    self.float_joins(&mut alt.body);
                }
                if let Some(def) = default {
                    self.float_joins(def);
                }
            }
            _ => {}
        }
    }
    /// Try to float a let-binding into the single case branch that uses it.
    ///
    /// Transforms:
    ///   `let x = v; case n { A -> ..x.. | B -> (no x) }`
    /// into:
    ///   `case n { A -> let x = v; ..x.. | B -> (no x) }`
    ///
    /// Only floats when exactly one branch references `x`.
    pub(super) fn try_float_into_case(&mut self, expr: &mut LcnfExpr) -> bool {
        let can_float = if let LcnfExpr::Let { id, body, .. } = &*expr {
            if let LcnfExpr::Case { alts, default, .. } = body.as_ref() {
                let use_count = alts.iter().filter(|a| expr_uses_var(&a.body, *id)).count()
                    + default
                        .as_ref()
                        .map(|d| usize::from(expr_uses_var(d, *id)))
                        .unwrap_or(0);
                use_count == 1
            } else {
                false
            }
        } else {
            false
        };
        if !can_float {
            return false;
        }
        let old = std::mem::replace(expr, LcnfExpr::Unreachable);
        if let LcnfExpr::Let {
            id,
            name,
            ty,
            value,
            body,
        } = old
        {
            if let LcnfExpr::Case {
                scrutinee,
                scrutinee_ty,
                mut alts,
                mut default,
            } = *body
            {
                if let Some(idx) = alts.iter().position(|a| expr_uses_var(&a.body, id)) {
                    let old_body = std::mem::replace(&mut alts[idx].body, LcnfExpr::Unreachable);
                    alts[idx].body = LcnfExpr::Let {
                        id,
                        name,
                        ty,
                        value,
                        body: Box::new(old_body),
                    };
                } else if let Some(def) = default.take() {
                    default = Some(Box::new(LcnfExpr::Let {
                        id,
                        name,
                        ty,
                        value,
                        body: def,
                    }));
                }
                *expr = LcnfExpr::Case {
                    scrutinee,
                    scrutinee_ty,
                    alts,
                    default,
                };
                return true;
            }
        }
        false
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct OJDominatorTree {
    pub idom: Vec<Option<u32>>,
    pub dom_children: Vec<Vec<u32>>,
    pub dom_depth: Vec<u32>,
}
impl OJDominatorTree {
    #[allow(dead_code)]
    pub fn new(size: usize) -> Self {
        OJDominatorTree {
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
pub struct OJWorklist {
    pub(super) items: std::collections::VecDeque<u32>,
    pub(super) in_worklist: std::collections::HashSet<u32>,
}
impl OJWorklist {
    #[allow(dead_code)]
    pub fn new() -> Self {
        OJWorklist {
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
/// Pass-timing record for OJoin profiler.
#[derive(Debug, Clone)]
pub struct OJoinPassTiming {
    pub pass_name: String,
    pub elapsed_us: u64,
    pub items_processed: usize,
    pub bytes_before: usize,
    pub bytes_after: usize,
}
impl OJoinPassTiming {
    pub fn new(
        pass_name: impl Into<String>,
        elapsed_us: u64,
        items: usize,
        before: usize,
        after: usize,
    ) -> Self {
        OJoinPassTiming {
            pass_name: pass_name.into(),
            elapsed_us,
            items_processed: items,
            bytes_before: before,
            bytes_after: after,
        }
    }
    pub fn throughput_mps(&self) -> f64 {
        if self.elapsed_us == 0 {
            0.0
        } else {
            self.items_processed as f64 / (self.elapsed_us as f64 / 1_000_000.0)
        }
    }
    pub fn size_ratio(&self) -> f64 {
        if self.bytes_before == 0 {
            1.0
        } else {
            self.bytes_after as f64 / self.bytes_before as f64
        }
    }
    pub fn is_profitable(&self) -> bool {
        self.size_ratio() <= 1.05
    }
}
/// Emission statistics for OJoin.
#[derive(Debug, Clone, Default)]
pub struct OJoinEmitStats {
    pub bytes_emitted: usize,
    pub items_emitted: usize,
    pub errors: usize,
    pub warnings: usize,
    pub elapsed_ms: u64,
}
impl OJoinEmitStats {
    pub fn new() -> Self {
        OJoinEmitStats::default()
    }
    pub fn throughput_bps(&self) -> f64 {
        if self.elapsed_ms == 0 {
            0.0
        } else {
            self.bytes_emitted as f64 / (self.elapsed_ms as f64 / 1000.0)
        }
    }
    pub fn is_clean(&self) -> bool {
        self.errors == 0
    }
}
/// Information about whether a variable is used only in tail position
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TailUse {
    /// Never used
    Unused,
    /// Used only in tail position
    TailOnly,
    /// Used in non-tail position
    NonTail,
    /// Used in both tail and non-tail positions
    Mixed,
}
impl TailUse {
    pub(super) fn merge(&self, other: &TailUse) -> TailUse {
        match (self, other) {
            (TailUse::Unused, x) | (x, TailUse::Unused) => x.clone(),
            (TailUse::TailOnly, TailUse::TailOnly) => TailUse::TailOnly,
            (TailUse::NonTail, TailUse::NonTail) => TailUse::NonTail,
            _ => TailUse::Mixed,
        }
    }
}
/// Severity of a OJoin diagnostic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum OJoinDiagSeverity {
    Note,
    Warning,
    Error,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct OJLivenessInfo {
    pub live_in: Vec<std::collections::HashSet<u32>>,
    pub live_out: Vec<std::collections::HashSet<u32>>,
    pub defs: Vec<std::collections::HashSet<u32>>,
    pub uses: Vec<std::collections::HashSet<u32>>,
}
impl OJLivenessInfo {
    #[allow(dead_code)]
    pub fn new(block_count: usize) -> Self {
        OJLivenessInfo {
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
pub struct OJCacheEntry {
    pub key: String,
    pub data: Vec<u8>,
    pub timestamp: u64,
    pub valid: bool,
}
/// A fixed-capacity ring buffer of strings (for recent-event logging in OJoin).
#[derive(Debug)]
pub struct OJoinEventLog {
    pub(super) entries: std::collections::VecDeque<String>,
    pub(super) capacity: usize,
}
impl OJoinEventLog {
    pub fn new(capacity: usize) -> Self {
        OJoinEventLog {
            entries: std::collections::VecDeque::with_capacity(capacity),
            capacity,
        }
    }
    pub fn push(&mut self, event: impl Into<String>) {
        if self.entries.len() >= self.capacity {
            self.entries.pop_front();
        }
        self.entries.push_back(event.into());
    }
    pub fn iter(&self) -> impl Iterator<Item = &String> {
        self.entries.iter()
    }
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
    pub fn capacity(&self) -> usize {
        self.capacity
    }
    pub fn clear(&mut self) {
        self.entries.clear();
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct OJPassStats {
    pub total_runs: u32,
    pub successful_runs: u32,
    pub total_changes: u64,
    pub time_ms: u64,
    pub iterations_used: u32,
}
impl OJPassStats {
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
pub struct OJPassConfig {
    pub phase: OJPassPhase,
    pub enabled: bool,
    pub max_iterations: u32,
    pub debug_output: bool,
    pub pass_name: String,
}
impl OJPassConfig {
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>, phase: OJPassPhase) -> Self {
        OJPassConfig {
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
#[derive(Debug, Clone, PartialEq)]
pub enum OJPassPhase {
    Analysis,
    Transformation,
    Verification,
    Cleanup,
}
impl OJPassPhase {
    #[allow(dead_code)]
    pub fn name(&self) -> &str {
        match self {
            OJPassPhase::Analysis => "analysis",
            OJPassPhase::Transformation => "transformation",
            OJPassPhase::Verification => "verification",
            OJPassPhase::Cleanup => "cleanup",
        }
    }
    #[allow(dead_code)]
    pub fn is_modifying(&self) -> bool {
        matches!(self, OJPassPhase::Transformation | OJPassPhase::Cleanup)
    }
}
