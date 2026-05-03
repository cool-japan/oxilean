//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::{LcnfArg, LcnfExpr, LcnfFunDecl, LcnfLetValue, LcnfLit, LcnfVarId};
use std::collections::{HashMap, HashSet};

use super::functions::*;
use std::collections::VecDeque;

/// Beta reduction pass -- reduce lambda applications.
pub struct BetaReductionPass {
    pub reductions: u32,
}
impl BetaReductionPass {
    pub fn new() -> Self {
        BetaReductionPass { reductions: 0 }
    }
    pub fn run(&mut self, decls: &mut [LcnfFunDecl]) {
        for decl in decls.iter_mut() {
            self.reduce_expr(&mut decl.body);
        }
    }
    pub(super) fn reduce_expr(&mut self, expr: &mut LcnfExpr) {
        match expr {
            LcnfExpr::Let { body, .. } => {
                self.reduce_expr(body);
            }
            LcnfExpr::Case { alts, default, .. } => {
                for alt in alts.iter_mut() {
                    self.reduce_expr(&mut alt.body);
                }
                if let Some(def) = default {
                    self.reduce_expr(def);
                }
            }
            LcnfExpr::TailCall(LcnfArg::Lit(_), _) => {
                self.reductions += 1;
            }
            LcnfExpr::Return(_) | LcnfExpr::Unreachable | LcnfExpr::TailCall(_, _) => {}
        }
    }
}
/// Describes a dependency between two passes.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PassDependency {
    /// Name of the pass that depends on another.
    pub pass: String,
    /// Name of the pass that must run first.
    pub depends_on: String,
}
impl PassDependency {
    /// Create a new dependency.
    pub fn new(pass: impl Into<String>, depends_on: impl Into<String>) -> Self {
        PassDependency {
            pass: pass.into(),
            depends_on: depends_on.into(),
        }
    }
}
#[allow(dead_code)]
pub struct OPPassRegistry {
    pub(super) configs: Vec<OPPassConfig>,
    pub(super) stats: std::collections::HashMap<String, OPPassStats>,
}
impl OPPassRegistry {
    #[allow(dead_code)]
    pub fn new() -> Self {
        OPPassRegistry {
            configs: Vec::new(),
            stats: std::collections::HashMap::new(),
        }
    }
    #[allow(dead_code)]
    pub fn register(&mut self, config: OPPassConfig) {
        self.stats
            .insert(config.pass_name.clone(), OPPassStats::new());
        self.configs.push(config);
    }
    #[allow(dead_code)]
    pub fn enabled_passes(&self) -> Vec<&OPPassConfig> {
        self.configs.iter().filter(|c| c.enabled).collect()
    }
    #[allow(dead_code)]
    pub fn get_stats(&self, name: &str) -> Option<&OPPassStats> {
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
#[derive(Debug, Clone, PartialEq)]
pub enum OPPassPhase {
    Analysis,
    Transformation,
    Verification,
    Cleanup,
}
impl OPPassPhase {
    #[allow(dead_code)]
    pub fn name(&self) -> &str {
        match self {
            OPPassPhase::Analysis => "analysis",
            OPPassPhase::Transformation => "transformation",
            OPPassPhase::Verification => "verification",
            OPPassPhase::Cleanup => "cleanup",
        }
    }
    #[allow(dead_code)]
    pub fn is_modifying(&self) -> bool {
        matches!(self, OPPassPhase::Transformation | OPPassPhase::Cleanup)
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct OPWorklist {
    pub(super) items: std::collections::VecDeque<u32>,
    pub(super) in_worklist: std::collections::HashSet<u32>,
}
impl OPWorklist {
    #[allow(dead_code)]
    pub fn new() -> Self {
        OPWorklist {
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
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct OPAnalysisCache {
    pub(super) entries: std::collections::HashMap<String, OPCacheEntry>,
    pub(super) max_size: usize,
    pub(super) hits: u64,
    pub(super) misses: u64,
}
impl OPAnalysisCache {
    #[allow(dead_code)]
    pub fn new(max_size: usize) -> Self {
        OPAnalysisCache {
            entries: std::collections::HashMap::new(),
            max_size,
            hits: 0,
            misses: 0,
        }
    }
    #[allow(dead_code)]
    pub fn get(&mut self, key: &str) -> Option<&OPCacheEntry> {
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
            OPCacheEntry {
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
/// Strength reduction: replaces expensive operations with cheaper equivalents.
/// For example, multiplication by a power of 2 becomes a left shift.
pub struct StrengthReductionPass {
    pub reductions: u32,
}
impl StrengthReductionPass {
    pub fn new() -> Self {
        StrengthReductionPass { reductions: 0 }
    }
    /// Check if a value is a power of two.
    pub fn is_power_of_two(n: u64) -> bool {
        n > 0 && (n & (n - 1)) == 0
    }
    /// Compute log2 for a power of two, returning None if not a power of two.
    pub fn log2_exact(n: u64) -> Option<u32> {
        if Self::is_power_of_two(n) {
            Some(n.trailing_zeros())
        } else {
            None
        }
    }
    /// Check if a value is a power of two minus one (e.g. 0x7F, 0xFF, 0xFFFF).
    pub fn is_mask(n: u64) -> bool {
        n > 0 && (n & (n + 1)) == 0
    }
    /// Count trailing zeros.
    pub fn ctz(n: u64) -> u32 {
        if n == 0 {
            64
        } else {
            n.trailing_zeros()
        }
    }
    /// Count leading zeros.
    pub fn clz(n: u64) -> u32 {
        n.leading_zeros()
    }
    /// Population count (number of set bits).
    pub fn popcount(n: u64) -> u32 {
        n.count_ones()
    }
}
/// Statistics for a single pass execution.
#[derive(Debug, Clone, Default)]
pub struct PassStats {
    /// Name of the pass.
    pub name: String,
    /// Number of times the pass has been run.
    pub run_count: u32,
    /// Total number of changes made across all runs.
    pub total_changes: usize,
    /// Duration of the last run in microseconds.
    pub last_duration_us: u64,
    /// Whether the last run made any changes.
    pub last_changed: bool,
}
impl PassStats {
    /// Create a new stats entry for the named pass.
    pub fn new(name: impl Into<String>) -> Self {
        PassStats {
            name: name.into(),
            ..Default::default()
        }
    }
    /// Record a run of this pass.
    pub fn record_run(&mut self, changes: usize, duration_us: u64) {
        self.run_count += 1;
        self.total_changes += changes;
        self.last_duration_us = duration_us;
        self.last_changed = changes > 0;
    }
    /// Average changes per run.
    pub fn avg_changes(&self) -> f64 {
        if self.run_count == 0 {
            0.0
        } else {
            self.total_changes as f64 / self.run_count as f64
        }
    }
}
/// Estimates the size and complexity of LCNF expressions.
///
/// Used by inlining heuristics to decide whether a function body is small
/// enough to inline.
pub struct ExprSizeEstimator;
impl ExprSizeEstimator {
    /// Count the number of let-bindings in an expression.
    pub fn count_lets(expr: &LcnfExpr) -> usize {
        match expr {
            LcnfExpr::Let { body, .. } => 1 + Self::count_lets(body),
            LcnfExpr::Case { alts, default, .. } => {
                let alt_sum: usize = alts.iter().map(|a| Self::count_lets(&a.body)).sum();
                let def_sum = default.as_ref().map(|d| Self::count_lets(d)).unwrap_or(0);
                alt_sum + def_sum
            }
            _ => 0,
        }
    }
    /// Count the number of case expressions.
    pub fn count_cases(expr: &LcnfExpr) -> usize {
        match expr {
            LcnfExpr::Let { body, .. } => Self::count_cases(body),
            LcnfExpr::Case { alts, default, .. } => {
                let alt_sum: usize = alts.iter().map(|a| Self::count_cases(&a.body)).sum();
                let def_sum = default.as_ref().map(|d| Self::count_cases(d)).unwrap_or(0);
                1 + alt_sum + def_sum
            }
            _ => 0,
        }
    }
    /// Compute a complexity score (lets + 2*cases + tail_calls).
    pub fn complexity(expr: &LcnfExpr) -> usize {
        match expr {
            LcnfExpr::Let { body, .. } => 1 + Self::complexity(body),
            LcnfExpr::Case { alts, default, .. } => {
                let alt_sum: usize = alts.iter().map(|a| Self::complexity(&a.body)).sum();
                let def_sum = default.as_ref().map(|d| Self::complexity(d)).unwrap_or(0);
                2 + alt_sum + def_sum
            }
            LcnfExpr::TailCall(_, _) => 1,
            LcnfExpr::Return(_) => 0,
            LcnfExpr::Unreachable => 0,
        }
    }
    /// Maximum nesting depth of the expression.
    pub fn max_depth(expr: &LcnfExpr) -> usize {
        match expr {
            LcnfExpr::Let { body, .. } => 1 + Self::max_depth(body),
            LcnfExpr::Case { alts, default, .. } => {
                let max_alt = alts
                    .iter()
                    .map(|a| Self::max_depth(&a.body))
                    .max()
                    .unwrap_or(0);
                let max_def = default.as_ref().map(|d| Self::max_depth(d)).unwrap_or(0);
                1 + max_alt.max(max_def)
            }
            _ => 0,
        }
    }
    /// Count all variable references in the expression.
    pub fn count_var_refs(expr: &LcnfExpr) -> usize {
        match expr {
            LcnfExpr::Let { value, body, .. } => {
                Self::count_var_refs_in_value(value) + Self::count_var_refs(body)
            }
            LcnfExpr::Case { alts, default, .. } => {
                let alt_sum: usize = alts.iter().map(|a| Self::count_var_refs(&a.body)).sum();
                let def_sum = default
                    .as_ref()
                    .map(|d| Self::count_var_refs(d))
                    .unwrap_or(0);
                1 + alt_sum + def_sum
            }
            LcnfExpr::Return(LcnfArg::Var(_)) => 1,
            LcnfExpr::TailCall(f, args) => {
                let f_count = if matches!(f, LcnfArg::Var(_)) { 1 } else { 0 };
                let a_count = args.iter().filter(|a| matches!(a, LcnfArg::Var(_))).count();
                f_count + a_count
            }
            _ => 0,
        }
    }
    pub(super) fn count_var_refs_in_value(value: &LcnfLetValue) -> usize {
        match value {
            LcnfLetValue::App(f, args) => {
                let f_count = if matches!(f, LcnfArg::Var(_)) { 1 } else { 0 };
                let a_count = args.iter().filter(|a| matches!(a, LcnfArg::Var(_))).count();
                f_count + a_count
            }
            LcnfLetValue::FVar(_) => 1,
            LcnfLetValue::Proj(_, _, _) => 1,
            LcnfLetValue::Reset(_) => 1,
            LcnfLetValue::Ctor(_, _, args) | LcnfLetValue::Reuse(_, _, _, args) => {
                args.iter().filter(|a| matches!(a, LcnfArg::Var(_))).count()
            }
            LcnfLetValue::Lit(_) | LcnfLetValue::Erased => 0,
        }
    }
    /// Whether an expression is "trivial" (just a return or unreachable).
    pub fn is_trivial(expr: &LcnfExpr) -> bool {
        matches!(expr, LcnfExpr::Return(_) | LcnfExpr::Unreachable)
    }
    /// Whether an expression is suitable for inlining (complexity below threshold).
    pub fn should_inline(expr: &LcnfExpr, threshold: usize) -> bool {
        Self::complexity(expr) <= threshold
    }
}
/// Manages a pipeline of optimization passes with dependency ordering.
///
/// Passes are executed in topological order based on their declared
/// dependencies. Cycle detection uses Kahn's algorithm.
#[derive(Debug, Default)]
pub struct PassManager {
    /// Registered pass names in insertion order.
    pub(super) pass_names: Vec<String>,
    /// Dependencies between passes.
    pub(super) dependencies: Vec<PassDependency>,
    /// Per-pass statistics.
    pub(super) stats: HashMap<String, PassStats>,
    /// Maximum number of fixed-point iterations.
    pub max_iterations: u32,
}
impl PassManager {
    /// Create a new pass manager.
    pub fn new() -> Self {
        PassManager {
            pass_names: Vec::new(),
            dependencies: Vec::new(),
            stats: HashMap::new(),
            max_iterations: 10,
        }
    }
    /// Register a pass by name.
    pub fn add_pass(&mut self, name: impl Into<String>) {
        let n = name.into();
        if !self.pass_names.contains(&n) {
            self.stats.insert(n.clone(), PassStats::new(&n));
            self.pass_names.push(n);
        }
    }
    /// Add a dependency: `pass` depends on `depends_on`.
    pub fn add_dependency(&mut self, pass: impl Into<String>, depends_on: impl Into<String>) {
        let dep = PassDependency::new(pass, depends_on);
        if !self.dependencies.contains(&dep) {
            self.dependencies.push(dep);
        }
    }
    /// Record a run of the named pass.
    pub fn record_run(&mut self, name: &str, changes: usize, duration_us: u64) {
        if let Some(stats) = self.stats.get_mut(name) {
            stats.record_run(changes, duration_us);
        }
    }
    /// Get statistics for a named pass.
    pub fn get_stats(&self, name: &str) -> Option<&PassStats> {
        self.stats.get(name)
    }
    /// Get all statistics.
    pub fn all_stats(&self) -> &HashMap<String, PassStats> {
        &self.stats
    }
    /// Number of registered passes.
    pub fn num_passes(&self) -> usize {
        self.pass_names.len()
    }
    /// Compute topological ordering of passes using Kahn's algorithm.
    ///
    /// Returns `None` if there is a cycle in the dependency graph.
    pub fn topological_order(&self) -> Option<Vec<String>> {
        let mut in_degree: HashMap<&str, usize> = HashMap::new();
        let mut adj: HashMap<&str, Vec<&str>> = HashMap::new();
        for name in &self.pass_names {
            in_degree.insert(name.as_str(), 0);
            adj.entry(name.as_str()).or_default();
        }
        for dep in &self.dependencies {
            if self.pass_names.contains(&dep.pass) && self.pass_names.contains(&dep.depends_on) {
                adj.entry(dep.depends_on.as_str())
                    .or_default()
                    .push(dep.pass.as_str());
                *in_degree.entry(dep.pass.as_str()).or_insert(0) += 1;
            }
        }
        let mut queue: Vec<&str> = in_degree
            .iter()
            .filter(|(_, &deg)| deg == 0)
            .map(|(&name, _)| name)
            .collect();
        queue.sort();
        let mut result = Vec::new();
        while let Some(node) = queue.pop() {
            result.push(node.to_string());
            if let Some(neighbors) = adj.get(node) {
                for &neighbor in neighbors {
                    let deg = in_degree
                        .get_mut(neighbor)
                        .expect(
                            "neighbor must be in in_degree; all passes were inserted during initialization",
                        );
                    *deg -= 1;
                    if *deg == 0 {
                        queue.push(neighbor);
                        queue.sort();
                    }
                }
            }
        }
        if result.len() == self.pass_names.len() {
            Some(result)
        } else {
            None
        }
    }
    /// Check if the dependency graph has a cycle.
    pub fn has_cycle(&self) -> bool {
        self.topological_order().is_none()
    }
    /// Total changes across all passes.
    pub fn total_changes(&self) -> usize {
        self.stats.values().map(|s| s.total_changes).sum()
    }
    /// Total runs across all passes.
    pub fn total_runs(&self) -> u32 {
        self.stats.values().map(|s| s.run_count).sum()
    }
}
/// Constant folding pass -- evaluate constant expressions at compile time.
pub struct ConstantFoldingPass {
    pub folds_performed: u32,
}
impl ConstantFoldingPass {
    pub fn new() -> Self {
        ConstantFoldingPass { folds_performed: 0 }
    }
    pub fn run(&mut self, decls: &mut [LcnfFunDecl]) {
        for decl in decls.iter_mut() {
            self.fold_expr(&mut decl.body);
        }
    }
    pub(super) fn fold_expr(&mut self, expr: &mut LcnfExpr) {
        match expr {
            LcnfExpr::Let { value, body, .. } => {
                if let LcnfLetValue::App(LcnfArg::Lit(LcnfLit::Nat(lhs)), args) = value {
                    if args.len() == 2 {
                        if let (LcnfArg::Lit(LcnfLit::Nat(rhs)), LcnfArg::Lit(LcnfLit::Nat(op_n))) =
                            (&args[0], &args[1])
                        {
                            let op = match op_n {
                                0 => "add",
                                1 => "sub",
                                2 => "mul",
                                _ => "",
                            };
                            if let Some(result) = self.try_fold_nat_op(op, *lhs, *rhs) {
                                *value = LcnfLetValue::Lit(LcnfLit::Nat(result));
                                self.folds_performed += 1;
                            }
                        }
                    }
                }
                self.fold_expr(body);
            }
            LcnfExpr::Case { alts, default, .. } => {
                for alt in alts.iter_mut() {
                    self.fold_expr(&mut alt.body);
                }
                if let Some(def) = default {
                    self.fold_expr(def);
                }
            }
            LcnfExpr::Return(_) | LcnfExpr::Unreachable | LcnfExpr::TailCall(_, _) => {}
        }
    }
    /// Try to fold a nat binary operation.
    pub fn try_fold_nat_op(&self, op: &str, lhs: u64, rhs: u64) -> Option<u64> {
        match op {
            "add" => Some(lhs.wrapping_add(rhs)),
            "sub" => Some(lhs.saturating_sub(rhs)),
            "mul" => Some(lhs.wrapping_mul(rhs)),
            "div" => lhs.checked_div(rhs),
            "mod" => lhs.checked_rem(rhs),
            "min" => Some(lhs.min(rhs)),
            "max" => Some(lhs.max(rhs)),
            "pow" => Some(lhs.wrapping_pow(rhs as u32)),
            "and" => Some(lhs & rhs),
            "or" => Some(lhs | rhs),
            "xor" => Some(lhs ^ rhs),
            "shl" => Some(lhs.wrapping_shl(rhs as u32)),
            "shr" => Some(lhs.wrapping_shr(rhs as u32)),
            _ => None,
        }
    }
    /// Try to fold a boolean operation.
    pub fn try_fold_bool_op(&self, op: &str, lhs: bool, rhs: bool) -> Option<bool> {
        match op {
            "and" => Some(lhs && rhs),
            "or" => Some(lhs || rhs),
            "xor" => Some(lhs ^ rhs),
            "eq" => Some(lhs == rhs),
            "ne" => Some(lhs != rhs),
            _ => None,
        }
    }
    /// Try to fold a comparison operation.
    pub fn try_fold_cmp(&self, op: &str, lhs: u64, rhs: u64) -> Option<bool> {
        match op {
            "eq" => Some(lhs == rhs),
            "ne" => Some(lhs != rhs),
            "lt" => Some(lhs < rhs),
            "le" => Some(lhs <= rhs),
            "gt" => Some(lhs > rhs),
            "ge" => Some(lhs >= rhs),
            _ => None,
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct OPCacheEntry {
    pub key: String,
    pub data: Vec<u8>,
    pub timestamp: u64,
    pub valid: bool,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct OPDominatorTree {
    pub idom: Vec<Option<u32>>,
    pub dom_children: Vec<Vec<u32>>,
    pub dom_depth: Vec<u32>,
}
impl OPDominatorTree {
    #[allow(dead_code)]
    pub fn new(size: usize) -> Self {
        OPDominatorTree {
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
/// Profile-guided optimization hints
#[derive(Debug, Clone)]
pub struct PgoHints {
    pub hot_functions: Vec<String>,
    pub likely_branches: Vec<(String, u32, bool)>,
    pub inline_candidates: Vec<String>,
    pub cold_functions: Vec<String>,
    pub call_counts: HashMap<String, u64>,
}
impl PgoHints {
    pub fn new() -> Self {
        PgoHints {
            hot_functions: Vec::new(),
            likely_branches: Vec::new(),
            inline_candidates: Vec::new(),
            cold_functions: Vec::new(),
            call_counts: HashMap::new(),
        }
    }
    pub fn mark_hot(&mut self, func_name: &str) {
        if !self.hot_functions.iter().any(|f| f == func_name) {
            self.hot_functions.push(func_name.to_string());
        }
    }
    pub fn mark_cold(&mut self, func_name: &str) {
        if !self.cold_functions.iter().any(|f| f == func_name) {
            self.cold_functions.push(func_name.to_string());
        }
    }
    pub fn mark_inline(&mut self, func_name: &str) {
        if !self.inline_candidates.iter().any(|f| f == func_name) {
            self.inline_candidates.push(func_name.to_string());
        }
    }
    pub fn record_call(&mut self, func_name: &str, count: u64) {
        *self.call_counts.entry(func_name.to_string()).or_insert(0) += count;
    }
    pub fn is_hot(&self, func_name: &str) -> bool {
        self.hot_functions.iter().any(|f| f == func_name)
    }
    pub fn is_cold(&self, func_name: &str) -> bool {
        self.cold_functions.iter().any(|f| f == func_name)
    }
    pub fn should_inline(&self, func_name: &str) -> bool {
        self.inline_candidates.iter().any(|f| f == func_name)
    }
    pub fn call_count(&self, func_name: &str) -> u64 {
        self.call_counts.get(func_name).copied().unwrap_or(0)
    }
    /// Total number of hints across all categories.
    pub fn total_hints(&self) -> usize {
        self.hot_functions.len()
            + self.cold_functions.len()
            + self.inline_candidates.len()
            + self.likely_branches.len()
            + self.call_counts.len()
    }
    /// Merge another set of hints into this one.
    pub fn merge(&mut self, other: &PgoHints) {
        for f in &other.hot_functions {
            self.mark_hot(f);
        }
        for f in &other.cold_functions {
            self.mark_cold(f);
        }
        for f in &other.inline_candidates {
            self.mark_inline(f);
        }
        for (name, count) in &other.call_counts {
            self.record_call(name, *count);
        }
    }
    /// Classify a function by its hotness: Hot, Cold, or Normal.
    pub fn classify(&self, func_name: &str) -> &'static str {
        if self.is_hot(func_name) {
            "hot"
        } else if self.is_cold(func_name) {
            "cold"
        } else {
            "normal"
        }
    }
}
/// Dead code elimination -- remove unreachable let expressions.
pub struct DeadCodeEliminationPass {
    pub removed: u32,
}
impl DeadCodeEliminationPass {
    pub fn new() -> Self {
        DeadCodeEliminationPass { removed: 0 }
    }
    pub fn run(&mut self, decls: &mut [LcnfFunDecl]) {
        for decl in decls.iter_mut() {
            let mut used: HashSet<LcnfVarId> = HashSet::new();
            Self::collect_used_vars(&decl.body, &mut used);
            let mut body = decl.body.clone();
            self.eliminate_dead_lets(&mut body, &used);
            decl.body = body;
        }
    }
    pub(super) fn eliminate_dead_lets(&mut self, expr: &mut LcnfExpr, used: &HashSet<LcnfVarId>) {
        match expr {
            LcnfExpr::Let {
                id, value, body, ..
            } => {
                let is_pure = matches!(
                    value,
                    LcnfLetValue::Lit(_) | LcnfLetValue::FVar(_) | LcnfLetValue::Erased
                );
                if is_pure && !used.contains(id) {
                    let new_body = *body.clone();
                    *expr = new_body;
                    self.removed += 1;
                    self.eliminate_dead_lets(expr, used);
                } else {
                    self.eliminate_dead_lets(body, used);
                }
            }
            LcnfExpr::Case { alts, default, .. } => {
                for alt in alts.iter_mut() {
                    self.eliminate_dead_lets(&mut alt.body, used);
                }
                if let Some(def) = default {
                    self.eliminate_dead_lets(def, used);
                }
            }
            LcnfExpr::Return(_) | LcnfExpr::Unreachable | LcnfExpr::TailCall(_, _) => {}
        }
    }
    pub(super) fn collect_used_vars(expr: &LcnfExpr, used: &mut HashSet<LcnfVarId>) {
        match expr {
            LcnfExpr::Let {
                id: _, value, body, ..
            } => {
                match value {
                    LcnfLetValue::App(func, args) => {
                        if let LcnfArg::Var(v) = func {
                            used.insert(*v);
                        }
                        for a in args {
                            if let LcnfArg::Var(v) = a {
                                used.insert(*v);
                            }
                        }
                    }
                    LcnfLetValue::FVar(v) => {
                        used.insert(*v);
                    }
                    LcnfLetValue::Ctor(_, _, args) | LcnfLetValue::Reuse(_, _, _, args) => {
                        for a in args {
                            if let LcnfArg::Var(v) = a {
                                used.insert(*v);
                            }
                        }
                    }
                    LcnfLetValue::Proj(_, _, v) => {
                        used.insert(*v);
                    }
                    LcnfLetValue::Reset(v) => {
                        used.insert(*v);
                    }
                    LcnfLetValue::Lit(_) | LcnfLetValue::Erased => {}
                }
                Self::collect_used_vars(body, used);
            }
            LcnfExpr::Case {
                scrutinee,
                alts,
                default,
                ..
            } => {
                used.insert(*scrutinee);
                for alt in alts {
                    Self::collect_used_vars(&alt.body, used);
                }
                if let Some(def) = default {
                    Self::collect_used_vars(def, used);
                }
            }
            LcnfExpr::Return(a) | LcnfExpr::TailCall(a, _) => {
                if let LcnfArg::Var(v) = a {
                    used.insert(*v);
                }
                if let LcnfExpr::TailCall(_, args) = expr {
                    for a in args {
                        if let LcnfArg::Var(v) = a {
                            used.insert(*v);
                        }
                    }
                }
            }
            LcnfExpr::Unreachable => {}
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct OPPassConfig {
    pub phase: OPPassPhase,
    pub enabled: bool,
    pub max_iterations: u32,
    pub debug_output: bool,
    pub pass_name: String,
}
impl OPPassConfig {
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>, phase: OPPassPhase) -> Self {
        OPPassConfig {
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
/// Copy propagation -- replace uses of copied variables with originals.
pub struct CopyPropagationPass {
    pub substitutions: u32,
}
impl CopyPropagationPass {
    pub fn new() -> Self {
        CopyPropagationPass { substitutions: 0 }
    }
    pub fn run(&mut self, decls: &mut [LcnfFunDecl]) {
        for decl in decls.iter_mut() {
            self.propagate_copies_in_expr(&mut decl.body);
        }
    }
    pub(super) fn propagate_copies_in_expr(&mut self, expr: &mut LcnfExpr) {
        if let LcnfExpr::Let {
            id,
            value: LcnfLetValue::FVar(src),
            body,
            ..
        } = expr
        {
            let from = *id;
            let to = *src;
            substitute_var_in_expr(body, from, to);
            self.substitutions += 1;
            self.propagate_copies_in_expr(body);
        } else {
            match expr {
                LcnfExpr::Let { body, .. } => self.propagate_copies_in_expr(body),
                LcnfExpr::Case { alts, default, .. } => {
                    for alt in alts.iter_mut() {
                        self.propagate_copies_in_expr(&mut alt.body);
                    }
                    if let Some(def) = default {
                        self.propagate_copies_in_expr(def);
                    }
                }
                _ => {}
            }
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct OPPassStats {
    pub total_runs: u32,
    pub successful_runs: u32,
    pub total_changes: u64,
    pub time_ms: u64,
    pub iterations_used: u32,
}
impl OPPassStats {
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
pub struct OPLivenessInfo {
    pub live_in: Vec<std::collections::HashSet<u32>>,
    pub live_out: Vec<std::collections::HashSet<u32>>,
    pub defs: Vec<std::collections::HashSet<u32>>,
    pub uses: Vec<std::collections::HashSet<u32>>,
}
impl OPLivenessInfo {
    #[allow(dead_code)]
    pub fn new(block_count: usize) -> Self {
        OPLivenessInfo {
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
pub struct OPDepGraph {
    pub(super) nodes: Vec<u32>,
    pub(super) edges: Vec<(u32, u32)>,
}
impl OPDepGraph {
    #[allow(dead_code)]
    pub fn new() -> Self {
        OPDepGraph {
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
pub struct OPConstantFoldingHelper;
impl OPConstantFoldingHelper {
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
/// Estimates the cost of inlining a function.
#[derive(Debug, Clone)]
pub struct InlineCostEstimator {
    /// Base cost threshold below which functions are always inlined.
    pub always_inline_threshold: usize,
    /// Threshold for functions in hot call sites.
    pub hot_threshold: usize,
    /// Threshold for cold call sites.
    pub cold_threshold: usize,
    /// Bonus for tail-recursive functions (they benefit less from inlining).
    pub tail_recursive_penalty: usize,
}
impl InlineCostEstimator {
    /// Compute the inlining cost for a function body.
    pub fn cost(&self, decl: &LcnfFunDecl) -> usize {
        let base = ExprSizeEstimator::complexity(&decl.body);
        let penalty = if decl.is_recursive {
            self.tail_recursive_penalty
        } else {
            0
        };
        base + penalty
    }
    /// Decide whether to inline based on cost and PGO hints.
    pub fn should_inline(&self, decl: &LcnfFunDecl, pgo: Option<&PgoHints>) -> bool {
        let cost = self.cost(decl);
        if cost <= self.always_inline_threshold {
            return true;
        }
        if let Some(hints) = pgo {
            if hints.should_inline(&decl.name) {
                return true;
            }
            if hints.is_hot(&decl.name) {
                return cost <= self.hot_threshold;
            }
            if hints.is_cold(&decl.name) {
                return cost <= self.cold_threshold;
            }
        }
        cost <= self.cold_threshold
    }
}
/// Eliminates identity let-bindings of the form `let x = x`.
pub struct IdentityEliminationPass {
    pub eliminated: u32,
}
impl IdentityEliminationPass {
    pub fn new() -> Self {
        IdentityEliminationPass { eliminated: 0 }
    }
    pub fn run(&mut self, decls: &mut [LcnfFunDecl]) {
        for decl in decls.iter_mut() {
            self.elim_expr(&mut decl.body);
        }
    }
    pub(super) fn elim_expr(&mut self, expr: &mut LcnfExpr) {
        loop {
            if let LcnfExpr::Let {
                id,
                value: LcnfLetValue::FVar(src),
                body,
                ..
            } = expr
            {
                if *id == *src {
                    let new_body = *body.clone();
                    *expr = new_body;
                    self.eliminated += 1;
                    continue;
                }
            }
            break;
        }
        match expr {
            LcnfExpr::Let { body, .. } => self.elim_expr(body),
            LcnfExpr::Case { alts, default, .. } => {
                for alt in alts.iter_mut() {
                    self.elim_expr(&mut alt.body);
                }
                if let Some(def) = default {
                    self.elim_expr(def);
                }
            }
            _ => {}
        }
    }
}
/// Eliminates code after `Unreachable` terminators.
pub struct UnreachableCodeEliminationPass {
    pub eliminated: u32,
}
impl UnreachableCodeEliminationPass {
    pub fn new() -> Self {
        UnreachableCodeEliminationPass { eliminated: 0 }
    }
    pub fn run(&mut self, decls: &mut [LcnfFunDecl]) {
        for decl in decls.iter_mut() {
            self.elim_expr(&mut decl.body);
        }
    }
    pub(super) fn elim_expr(&mut self, expr: &mut LcnfExpr) {
        match expr {
            LcnfExpr::Let { body, .. } => {
                self.elim_expr(body);
                if matches!(**body, LcnfExpr::Unreachable) {
                    *expr = LcnfExpr::Unreachable;
                    self.eliminated += 1;
                }
            }
            LcnfExpr::Case { alts, default, .. } => {
                for alt in alts.iter_mut() {
                    self.elim_expr(&mut alt.body);
                }
                if let Some(def) = default {
                    self.elim_expr(def);
                }
            }
            _ => {}
        }
    }
}
