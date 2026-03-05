//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::{LcnfArg, LcnfExpr, LcnfFunDecl, LcnfLetValue, LcnfLit, LcnfVarId};
use std::collections::HashMap;

use super::functions::LoopOptPass;

use std::collections::{HashSet, VecDeque};

#[allow(dead_code)]
pub struct LUConstantFoldingHelper;
impl LUConstantFoldingHelper {
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
/// Information about a detected loop inside an LCNF function body.
#[derive(Debug, Clone)]
pub struct LoopInfo {
    /// The induction variable name/id.
    pub loop_var: LcnfVarId,
    /// Loop start bound (inclusive).
    pub start: u64,
    /// Loop end bound (exclusive).
    pub end: u64,
    /// Step size (usually 1).
    pub step: u64,
    /// The loop body expressions to be replicated.
    pub body: Vec<LcnfExpr>,
    /// Statically known trip count, if computable.
    pub trip_count: Option<u64>,
    /// Whether this loop has no nested loops inside it.
    pub is_innermost: bool,
    /// Whether the loop has a statically determinable iteration count.
    pub is_counted: bool,
    /// Estimated size of the loop body in abstract instruction units.
    pub estimated_size: u64,
}
impl LoopInfo {
    /// Create a new loop info record.
    pub fn new(loop_var: LcnfVarId, start: u64, end: u64, step: u64, body: Vec<LcnfExpr>) -> Self {
        let trip_count = if step > 0 && end > start {
            Some((end - start).div_ceil(step))
        } else {
            None
        };
        let is_counted = trip_count.is_some();
        let estimated_size = body.len() as u64 * 3;
        LoopInfo {
            loop_var,
            start,
            end,
            step,
            body,
            trip_count,
            is_innermost: true,
            is_counted,
            estimated_size,
        }
    }
    /// Compute a freshness score for unrolling priority (higher is better).
    pub fn priority_score(&self) -> u64 {
        let count_bonus = self.trip_count.unwrap_or(0).min(64);
        let innermost_bonus: u64 = if self.is_innermost { 20 } else { 0 };
        let size_penalty = self.estimated_size.min(50);
        count_bonus + innermost_bonus - size_penalty / 5
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LUDominatorTree {
    pub idom: Vec<Option<u32>>,
    pub dom_children: Vec<Vec<u32>>,
    pub dom_depth: Vec<u32>,
}
impl LUDominatorTree {
    #[allow(dead_code)]
    pub fn new(size: usize) -> Self {
        LUDominatorTree {
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
pub struct LULivenessInfo {
    pub live_in: Vec<std::collections::HashSet<u32>>,
    pub live_out: Vec<std::collections::HashSet<u32>>,
    pub defs: Vec<std::collections::HashSet<u32>>,
    pub uses: Vec<std::collections::HashSet<u32>>,
}
impl LULivenessInfo {
    #[allow(dead_code)]
    pub fn new(block_count: usize) -> Self {
        LULivenessInfo {
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
pub struct LUPassRegistry {
    pub(super) configs: Vec<LUPassConfig>,
    pub(super) stats: std::collections::HashMap<String, LUPassStats>,
}
impl LUPassRegistry {
    #[allow(dead_code)]
    pub fn new() -> Self {
        LUPassRegistry {
            configs: Vec::new(),
            stats: std::collections::HashMap::new(),
        }
    }
    #[allow(dead_code)]
    pub fn register(&mut self, config: LUPassConfig) {
        self.stats
            .insert(config.pass_name.clone(), LUPassStats::new());
        self.configs.push(config);
    }
    #[allow(dead_code)]
    pub fn enabled_passes(&self) -> Vec<&LUPassConfig> {
        self.configs.iter().filter(|c| c.enabled).collect()
    }
    #[allow(dead_code)]
    pub fn get_stats(&self, name: &str) -> Option<&LUPassStats> {
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
/// Information about a nested loop structure.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LoopNestInfo {
    /// Depth of nesting (0 = outermost).
    pub depth: usize,
    /// The loop at this level.
    pub loop_info: LoopInfo,
    /// Child loops nested inside this one.
    pub children: Vec<LoopNestInfo>,
}
impl LoopNestInfo {
    /// Create a new loop nest info node.
    #[allow(dead_code)]
    pub fn new(loop_info: LoopInfo) -> Self {
        LoopNestInfo {
            depth: 0,
            loop_info,
            children: Vec::new(),
        }
    }
    /// Total number of loops in this nest (self + all children recursively).
    #[allow(dead_code)]
    pub fn total_count(&self) -> usize {
        1 + self.children.iter().map(|c| c.total_count()).sum::<usize>()
    }
    /// Maximum nesting depth below this node.
    #[allow(dead_code)]
    pub fn max_depth(&self) -> usize {
        if self.children.is_empty() {
            self.depth
        } else {
            self.children
                .iter()
                .map(|c| c.max_depth())
                .max()
                .unwrap_or(self.depth)
        }
    }
    /// Collect all leaf (innermost) loops.
    #[allow(dead_code)]
    pub fn collect_leaves(&self) -> Vec<&LoopInfo> {
        if self.children.is_empty() {
            vec![&self.loop_info]
        } else {
            self.children
                .iter()
                .flat_map(|c| c.collect_leaves())
                .collect()
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LUDepGraph {
    pub(super) nodes: Vec<u32>,
    pub(super) edges: Vec<(u32, u32)>,
}
impl LUDepGraph {
    #[allow(dead_code)]
    pub fn new() -> Self {
        LUDepGraph {
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
/// A strength reduction opportunity detected in a loop.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct StrengthReduction {
    /// Variable computed by a multiply-by-induction expression.
    pub target: LcnfVarId,
    /// The stride (constant multiplier per iteration).
    pub stride: u64,
    /// Initial value of the reduced expression.
    pub initial_value: u64,
}
impl StrengthReduction {
    /// Create a new strength reduction record.
    #[allow(dead_code)]
    pub fn new(target: LcnfVarId, stride: u64, initial_value: u64) -> Self {
        StrengthReduction {
            target,
            stride,
            initial_value,
        }
    }
    /// Describe the reduction.
    #[allow(dead_code)]
    pub fn describe(&self) -> String {
        format!(
            "v{:?} → start={} + k*{}",
            self.target, self.initial_value, self.stride
        )
    }
}
/// Scheduler that prioritizes unroll candidates by profitability.
#[allow(dead_code)]
pub struct UnrollScheduler {
    /// Maximum number of loops to unroll per pass.
    pub max_unrolls: usize,
    /// Accumulated candidates from all analyzed functions.
    pub candidates: Vec<UnrollCandidate>,
}
impl UnrollScheduler {
    /// Create a new scheduler.
    #[allow(dead_code)]
    pub fn new(max_unrolls: usize) -> Self {
        UnrollScheduler {
            max_unrolls,
            candidates: Vec::new(),
        }
    }
    /// Add a candidate to the scheduler.
    #[allow(dead_code)]
    pub fn push(&mut self, candidate: UnrollCandidate) {
        self.candidates.push(candidate);
    }
    /// Schedule the top `max_unrolls` profitable candidates.
    #[allow(dead_code)]
    pub fn schedule(&mut self) -> Vec<&UnrollCandidate> {
        self.candidates
            .sort_by(|a, b| b.estimated_savings.cmp(&a.estimated_savings));
        self.candidates
            .iter()
            .filter(|c| c.is_profitable())
            .take(self.max_unrolls)
            .collect()
    }
    /// Clear all candidates.
    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.candidates.clear();
    }
    /// Number of profitable candidates.
    #[allow(dead_code)]
    pub fn num_profitable(&self) -> usize {
        self.candidates.iter().filter(|c| c.is_profitable()).count()
    }
}
/// Recommended prefetch distance for a loop.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PrefetchRecommendation {
    /// Loop induction variable.
    pub loop_var: LcnfVarId,
    /// Suggested prefetch distance in iterations.
    pub distance: u64,
    /// Whether a prefetch is beneficial.
    pub is_beneficial: bool,
}
impl PrefetchRecommendation {
    /// Create a prefetch recommendation.
    #[allow(dead_code)]
    pub fn new(loop_var: LcnfVarId, trip_count: u64, cache_line_iterations: u64) -> Self {
        let distance = cache_line_iterations.min(trip_count / 4).max(1);
        let is_beneficial = trip_count > cache_line_iterations * 2;
        PrefetchRecommendation {
            loop_var,
            distance,
            is_beneficial,
        }
    }
}
/// The type of dependence between two loop iterations.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DependenceKind {
    /// Read-After-Write: a later iteration reads what an earlier one wrote.
    ReadAfterWrite,
    /// Write-After-Read: a later iteration writes what an earlier one read.
    WriteAfterRead,
    /// Write-After-Write: two iterations write to the same location.
    WriteAfterWrite,
    /// No dependence.
    Independent,
}
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct LUPassStats {
    pub total_runs: u32,
    pub successful_runs: u32,
    pub total_changes: u64,
    pub time_ms: u64,
    pub iterations_used: u32,
}
impl LUPassStats {
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
#[derive(Debug, Clone, PartialEq)]
pub enum LUPassPhase {
    Analysis,
    Transformation,
    Verification,
    Cleanup,
}
impl LUPassPhase {
    #[allow(dead_code)]
    pub fn name(&self) -> &str {
        match self {
            LUPassPhase::Analysis => "analysis",
            LUPassPhase::Transformation => "transformation",
            LUPassPhase::Verification => "verification",
            LUPassPhase::Cleanup => "cleanup",
        }
    }
    #[allow(dead_code)]
    pub fn is_modifying(&self) -> bool {
        matches!(self, LUPassPhase::Transformation | LUPassPhase::Cleanup)
    }
}
/// A pass that identifies loop-invariant computations.
#[allow(dead_code)]
pub struct LoopInvariantMotionPass {
    /// Candidates found during analysis.
    pub candidates: Vec<HoistCandidate>,
}
impl LoopInvariantMotionPass {
    /// Create a new pass.
    #[allow(dead_code)]
    pub fn new() -> Self {
        LoopInvariantMotionPass {
            candidates: Vec::new(),
        }
    }
    /// Analyze a function and collect hoist candidates.
    #[allow(dead_code)]
    pub fn analyze(&mut self, decl: &LcnfFunDecl) {
        self.collect_from_expr(&decl.body, None, 0);
    }
    pub(super) fn collect_from_expr(
        &mut self,
        expr: &LcnfExpr,
        loop_var: Option<LcnfVarId>,
        trip_count: u64,
    ) {
        match expr {
            LcnfExpr::Let {
                id, value, body, ..
            } => {
                if let Some(lv) = loop_var {
                    let is_invariant = !self.value_uses_var(value, lv);
                    if is_invariant && trip_count > 1 {
                        self.candidates.push(HoistCandidate::new(
                            *id,
                            value.clone(),
                            lv,
                            trip_count,
                        ));
                    }
                }
                self.collect_from_expr(body, loop_var, trip_count);
            }
            LcnfExpr::Case { alts, default, .. } => {
                for alt in alts {
                    self.collect_from_expr(&alt.body, loop_var, trip_count);
                }
                if let Some(d) = default {
                    self.collect_from_expr(d, loop_var, trip_count);
                }
            }
            _ => {}
        }
    }
    pub(super) fn value_uses_var(&self, value: &LcnfLetValue, var: LcnfVarId) -> bool {
        match value {
            LcnfLetValue::FVar(id) => *id == var,
            LcnfLetValue::App(f_arg, args) => {
                self.arg_is_var_id(f_arg, var) || args.iter().any(|a| self.arg_is_var_id(a, var))
            }
            _ => false,
        }
    }
    pub(super) fn arg_is_var_id(&self, arg: &LcnfArg, var: LcnfVarId) -> bool {
        matches!(arg, LcnfArg::Var(id) if * id == var)
    }
    /// Number of profitable hoist candidates.
    #[allow(dead_code)]
    pub fn num_profitable(&self) -> usize {
        self.candidates.iter().filter(|c| c.is_profitable()).count()
    }
}
/// Tracks the remaining unroll code-size budget during a pass.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct UnrollBudget {
    /// Total budget in abstract units.
    pub total: u64,
    /// Amount consumed so far.
    pub consumed: u64,
}
impl UnrollBudget {
    /// Create a new budget.
    #[allow(dead_code)]
    pub fn new(total: u64) -> Self {
        UnrollBudget { total, consumed: 0 }
    }
    /// Remaining budget.
    #[allow(dead_code)]
    pub fn remaining(&self) -> u64 {
        self.total.saturating_sub(self.consumed)
    }
    /// Attempt to consume `amount` from the budget.
    /// Returns `true` if the consumption is within budget.
    #[allow(dead_code)]
    pub fn try_consume(&mut self, amount: u64) -> bool {
        if amount <= self.remaining() {
            self.consumed += amount;
            true
        } else {
            false
        }
    }
    /// Reset the budget to full.
    #[allow(dead_code)]
    pub fn reset(&mut self) {
        self.consumed = 0;
    }
    /// Utilization ratio (0.0 – 1.0).
    #[allow(dead_code)]
    pub fn utilization(&self) -> f64 {
        if self.total == 0 {
            0.0
        } else {
            self.consumed as f64 / self.total as f64
        }
    }
}
/// The main loop unrolling optimization pass.
///
/// # Usage
/// ```rust
/// use oxilean_codegen::opt_loop_unroll::{LoopUnrollPass, UnrollConfig};
/// let pass = LoopUnrollPass::new(UnrollConfig::default());
/// ```
pub struct LoopUnrollPass {
    pub(super) config: UnrollConfig,
    pub(super) report: UnrollReport,
    /// Counter for fresh variable IDs in replicated bodies.
    pub(super) next_var_id: u64,
}
impl LoopUnrollPass {
    /// Create a new pass with the given configuration.
    pub fn new(config: UnrollConfig) -> Self {
        LoopUnrollPass {
            config,
            report: UnrollReport::default(),
            next_var_id: 100_000,
        }
    }
    /// Create a pass with the default configuration.
    pub fn default_pass() -> Self {
        Self::new(UnrollConfig::default())
    }
    /// Run the unrolling pass over a list of function declarations.
    /// Mutates declarations in place and accumulates statistics.
    pub fn run(&mut self, decls: &mut [LcnfFunDecl]) {
        for decl in decls.iter_mut() {
            self.process_decl(decl);
        }
    }
    /// Return the accumulated report after running.
    pub fn report(&self) -> &UnrollReport {
        &self.report
    }
    /// Analyze loops in a single declaration and return their info.
    pub fn analyze_loops(&self, decl: &LcnfFunDecl) -> Vec<LoopInfo> {
        let mut loops = Vec::new();
        self.collect_loops_in_expr(&decl.body, &mut loops, &HashMap::new(), true);
        loops
    }
    /// Compute the recommended unroll factor for a loop.
    pub fn compute_unroll_factor(&self, info: &LoopInfo) -> UnrollFactor {
        let trip = match info.trip_count {
            Some(t) => t,
            None => return UnrollFactor::Partial(2),
        };
        if !info.is_counted {
            return UnrollFactor::Partial(2);
        }
        if trip <= self.config.unroll_full_threshold {
            let unrolled = info.estimated_size * trip;
            if unrolled <= self.config.max_unrolled_size {
                return UnrollFactor::Full;
            }
        }
        if self.config.enable_vectorizable && info.is_innermost {
            if trip % 8 == 0 {
                let size = info.estimated_size * 8;
                if size <= self.config.max_unrolled_size {
                    return UnrollFactor::Vectorizable(8);
                }
            }
            if trip % 4 == 0 {
                let size = info.estimated_size * 4;
                if size <= self.config.max_unrolled_size {
                    return UnrollFactor::Vectorizable(4);
                }
            }
        }
        if trip >= self.config.min_trip_count_for_partial {
            let mut best_factor = 1u32;
            for &f in &[8u32, 4, 2] {
                if f <= self.config.max_unroll_factor {
                    let size = info.estimated_size * f as u64;
                    if size <= self.config.max_unrolled_size {
                        best_factor = f;
                        break;
                    }
                }
            }
            if best_factor > 1 {
                return UnrollFactor::Partial(best_factor);
            }
        }
        UnrollFactor::Partial(1)
    }
    /// Replicate `body` according to `factor`.
    ///
    /// For `Full` and `Partial(n)`, the body is duplicated n times with
    /// fresh variable IDs.  For `Jamming`, returns the body unchanged
    /// (jamming is handled at a higher level).
    pub fn unroll_loop(&mut self, body: &[LcnfExpr], factor: &UnrollFactor) -> Vec<LcnfExpr> {
        let n = match factor {
            UnrollFactor::Full => body.len(),
            UnrollFactor::Partial(f) => *f as usize,
            UnrollFactor::Vectorizable(f) => *f as usize,
            UnrollFactor::Jamming => return body.to_vec(),
        };
        let mut result = Vec::with_capacity(body.len() * n.max(1));
        for _ in 0..n.max(1) {
            for expr in body {
                result.push(self.clone_expr_fresh(expr));
            }
        }
        result
    }
    pub(super) fn process_decl(&mut self, decl: &mut LcnfFunDecl) {
        let loops = self.analyze_loops(decl);
        self.report.loops_analyzed += loops.len();
        let mut candidates: Vec<UnrollCandidate> = loops
            .into_iter()
            .map(|info| {
                let factor = self.compute_unroll_factor(&info);
                let savings = self.estimate_savings(&info, &factor);
                UnrollCandidate::new(decl.name.clone(), info, factor, savings)
            })
            .filter(|c| c.is_profitable())
            .collect();
        candidates.sort_by(|a, b| b.estimated_savings.cmp(&a.estimated_savings));
        if self.config.enable_jamming
            && candidates.len() >= 2
            && self.can_jam(&candidates[0].loop_info, &candidates[1].loop_info)
        {
            self.report.jammed_loops += 2;
            self.report.loops_unrolled += 2;
        }
        for candidate in &candidates {
            match &candidate.recommended_factor {
                UnrollFactor::Full => {
                    self.report.full_unrolls += 1;
                    self.report.loops_unrolled += 1;
                }
                UnrollFactor::Partial(f) if *f > 1 => {
                    self.report.partial_unrolls += 1;
                    self.report.loops_unrolled += 1;
                }
                UnrollFactor::Vectorizable(_) => {
                    self.report.vectorizable_loops += 1;
                    self.report.loops_unrolled += 1;
                }
                _ => {}
            }
        }
        let speedup = self.compute_overall_speedup(&candidates);
        self.report.estimated_speedup =
            (self.report.estimated_speedup * 0.8 + speedup * 0.2).max(1.0);
        if !candidates.is_empty() {
            decl.body = self.rewrite_expr(&decl.body, &candidates);
        }
    }
    pub(super) fn collect_loops_in_expr(
        &self,
        expr: &LcnfExpr,
        out: &mut Vec<LoopInfo>,
        constants: &HashMap<LcnfVarId, u64>,
        _outermost: bool,
    ) {
        match expr {
            LcnfExpr::Let {
                id, value, body, ..
            } => {
                let mut inner_constants = constants.clone();
                if let LcnfLetValue::Lit(LcnfLit::Nat(n)) = value {
                    inner_constants.insert(*id, *n);
                }
                if let Some(info) = self.try_detect_loop(id, value, body, &inner_constants) {
                    let mut loop_info = info;
                    let mut nested = Vec::new();
                    for sub_expr in &loop_info.body {
                        self.collect_loops_in_expr(sub_expr, &mut nested, &inner_constants, false);
                    }
                    if !nested.is_empty() {
                        loop_info.is_innermost = false;
                        out.extend(nested);
                    }
                    out.push(loop_info);
                } else {
                    self.collect_loops_in_expr(body, out, &inner_constants, _outermost);
                }
            }
            LcnfExpr::Case { alts, default, .. } => {
                for alt in alts {
                    self.collect_loops_in_expr(&alt.body, out, constants, false);
                }
                if let Some(def) = default {
                    self.collect_loops_in_expr(def, out, constants, false);
                }
            }
            LcnfExpr::TailCall(_, _) | LcnfExpr::Return(_) | LcnfExpr::Unreachable => {}
        }
    }
    /// Try to detect a counted loop starting at `id` with `value`.
    ///
    /// In LCNF, a loop is represented as a tail-recursive function call or a
    /// case split on a counter.  We detect the simplified pattern:
    ///   `let i = Nat(start); case i of { ... tailcall loop(i+step) } until i == end`
    ///
    /// For the purposes of this pass we look for tail calls with a Nat literal
    /// argument pattern that suggests a bounded loop.
    pub(super) fn try_detect_loop(
        &self,
        id: &LcnfVarId,
        value: &LcnfLetValue,
        body: &LcnfExpr,
        constants: &HashMap<LcnfVarId, u64>,
    ) -> Option<LoopInfo> {
        let start = match value {
            LcnfLetValue::Lit(LcnfLit::Nat(n)) => *n,
            LcnfLetValue::FVar(other_id) => *constants.get(other_id)?,
            _ => return None,
        };
        let (end, step, sub_body) = self.infer_loop_bounds(id, body, constants, start)?;
        Some(LoopInfo::new(*id, start, end, step, sub_body))
    }
    /// Infer end bound and step from a body expression involving `counter`.
    pub(super) fn infer_loop_bounds(
        &self,
        counter: &LcnfVarId,
        expr: &LcnfExpr,
        constants: &HashMap<LcnfVarId, u64>,
        start: u64,
    ) -> Option<(u64, u64, Vec<LcnfExpr>)> {
        match expr {
            LcnfExpr::Let {
                id, value, body, ..
            } => {
                if let LcnfLetValue::App(func_arg, args) = value {
                    if self.is_nat_add(func_arg) && args.iter().any(|a| self.arg_is_var(a, counter))
                    {
                        if let Some(step) = self.extract_nat_arg(args, constants) {
                            if let Some((end, _, sub)) =
                                self.infer_loop_bounds(id, body, constants, start)
                            {
                                return Some((end, step, sub));
                            }
                        }
                    }
                }
                self.infer_loop_bounds(counter, body, constants, start)
            }
            LcnfExpr::TailCall(_, args) => {
                let _ = args;
                let end = constants.values().copied().find(|&v| v > start)?;
                let step = 1u64;
                Some((end, step, vec![expr.clone()]))
            }
            LcnfExpr::Case {
                scrutinee,
                alts,
                default,
                ..
            } => {
                if scrutinee == counter {
                    let end = constants
                        .values()
                        .copied()
                        .find(|&v| v > start)
                        .unwrap_or(start + 4);
                    let bodies: Vec<LcnfExpr> = alts
                        .iter()
                        .map(|a| a.body.clone())
                        .chain(default.iter().map(|d| *d.clone()))
                        .collect();
                    return Some((end, 1, bodies));
                }
                None
            }
            _ => None,
        }
    }
    pub(super) fn is_nat_add(&self, arg: &crate::lcnf::LcnfArg) -> bool {
        match arg {
            crate::lcnf::LcnfArg::Var(_) => false,
            crate::lcnf::LcnfArg::Lit(_) => false,
            crate::lcnf::LcnfArg::Erased => false,
            crate::lcnf::LcnfArg::Type(_) => false,
        }
    }
    pub(super) fn arg_is_var(&self, arg: &crate::lcnf::LcnfArg, id: &LcnfVarId) -> bool {
        matches!(arg, crate ::lcnf::LcnfArg::Var(v) if v == id)
    }
    pub(super) fn extract_nat_arg(
        &self,
        args: &[crate::lcnf::LcnfArg],
        constants: &HashMap<LcnfVarId, u64>,
    ) -> Option<u64> {
        for arg in args {
            match arg {
                crate::lcnf::LcnfArg::Lit(LcnfLit::Nat(n)) => return Some(*n),
                crate::lcnf::LcnfArg::Var(id) => {
                    if let Some(&v) = constants.get(id) {
                        return Some(v);
                    }
                }
                _ => {}
            }
        }
        None
    }
    pub(super) fn estimate_savings(&self, info: &LoopInfo, factor: &UnrollFactor) -> i64 {
        let trip = info.trip_count.unwrap_or(8) as i64;
        let body_size = info.estimated_size as i64;
        match factor {
            UnrollFactor::Full => {
                let overhead_per_iter = 3i64;
                let total_overhead = overhead_per_iter * trip;
                let unrolled_size = body_size * trip;
                if unrolled_size <= self.config.max_unrolled_size as i64 {
                    total_overhead
                } else {
                    -1
                }
            }
            UnrollFactor::Partial(f) => {
                let f = *f as i64;
                let overhead_per_iter = 3i64;
                let saved_overhead = overhead_per_iter * (trip - trip / f);
                let code_growth = body_size * (f - 1);
                saved_overhead - code_growth / 10
            }
            UnrollFactor::Vectorizable(f) => {
                let f = *f as i64;
                let simd_gain = body_size * (f - 1);
                simd_gain - body_size / 2
            }
            UnrollFactor::Jamming => {
                let overhead = 3i64 * trip;
                overhead - body_size / 4
            }
        }
    }
    pub(super) fn can_jam(&self, a: &LoopInfo, b: &LoopInfo) -> bool {
        a.start == b.start && a.end == b.end && a.step == b.step
    }
    pub(super) fn compute_overall_speedup(&self, candidates: &[UnrollCandidate]) -> f64 {
        if candidates.is_empty() {
            return 1.0;
        }
        let total_savings: i64 = candidates.iter().map(|c| c.estimated_savings.max(0)).sum();
        let total_cost: i64 = candidates
            .iter()
            .map(|c| c.loop_info.estimated_size as i64)
            .sum();
        if total_cost == 0 {
            1.0
        } else {
            1.0 + (total_savings as f64 / total_cost as f64).min(4.0)
        }
    }
    pub(super) fn rewrite_expr(
        &mut self,
        expr: &LcnfExpr,
        candidates: &[UnrollCandidate],
    ) -> LcnfExpr {
        match expr {
            LcnfExpr::Let {
                id,
                name,
                ty,
                value,
                body,
            } => {
                if let Some(candidate) = candidates.iter().find(|c| c.loop_info.loop_var == *id) {
                    let unrolled =
                        self.unroll_loop(&candidate.loop_info.body, &candidate.recommended_factor);
                    let new_body = self.chain_exprs(unrolled, body);
                    return LcnfExpr::Let {
                        id: *id,
                        name: name.clone(),
                        ty: ty.clone(),
                        value: value.clone(),
                        body: Box::new(new_body),
                    };
                }
                let new_body = self.rewrite_expr(body, candidates);
                LcnfExpr::Let {
                    id: *id,
                    name: name.clone(),
                    ty: ty.clone(),
                    value: value.clone(),
                    body: Box::new(new_body),
                }
            }
            LcnfExpr::Case {
                scrutinee,
                scrutinee_ty,
                alts,
                default,
            } => {
                let new_alts = alts
                    .iter()
                    .map(|alt| {
                        let mut new_alt = alt.clone();
                        new_alt.body = self.rewrite_expr(&alt.body, candidates);
                        new_alt
                    })
                    .collect();
                let new_default = default
                    .as_ref()
                    .map(|d| Box::new(self.rewrite_expr(d, candidates)));
                LcnfExpr::Case {
                    scrutinee: *scrutinee,
                    scrutinee_ty: scrutinee_ty.clone(),
                    alts: new_alts,
                    default: new_default,
                }
            }
            other => other.clone(),
        }
    }
    pub(super) fn chain_exprs(&self, unrolled: Vec<LcnfExpr>, continuation: &LcnfExpr) -> LcnfExpr {
        if unrolled.is_empty() {
            return continuation.clone();
        }
        let mut result = continuation.clone();
        for expr in unrolled.into_iter().rev() {
            result = self.prepend_expr(expr, result);
        }
        result
    }
    pub(super) fn prepend_expr(&self, prefix: LcnfExpr, continuation: LcnfExpr) -> LcnfExpr {
        match prefix {
            LcnfExpr::Let {
                id,
                name,
                ty,
                value,
                body: _,
            } => LcnfExpr::Let {
                id,
                name,
                ty,
                value,
                body: Box::new(continuation),
            },
            other => other,
        }
    }
    /// Clone an expression with fresh variable IDs (alpha-renaming).
    pub(super) fn clone_expr_fresh(&mut self, expr: &LcnfExpr) -> LcnfExpr {
        let mut id_map: HashMap<LcnfVarId, LcnfVarId> = HashMap::new();
        self.clone_expr_with_map(expr, &mut id_map)
    }
    pub(super) fn fresh_id(&mut self) -> LcnfVarId {
        let id = LcnfVarId(self.next_var_id);
        self.next_var_id += 1;
        id
    }
    pub(super) fn clone_expr_with_map(
        &mut self,
        expr: &LcnfExpr,
        map: &mut HashMap<LcnfVarId, LcnfVarId>,
    ) -> LcnfExpr {
        match expr {
            LcnfExpr::Let {
                id,
                name,
                ty,
                value,
                body,
            } => {
                let new_id = self.fresh_id();
                map.insert(*id, new_id);
                let new_value = self.clone_value_with_map(value, map);
                let new_body = self.clone_expr_with_map(body, map);
                LcnfExpr::Let {
                    id: new_id,
                    name: name.clone(),
                    ty: ty.clone(),
                    value: new_value,
                    body: Box::new(new_body),
                }
            }
            LcnfExpr::Case {
                scrutinee,
                scrutinee_ty,
                alts,
                default,
            } => {
                let new_scrutinee = map.get(scrutinee).copied().unwrap_or(*scrutinee);
                let new_alts = alts
                    .iter()
                    .map(|alt| {
                        let mut inner_map = map.clone();
                        let new_params = alt
                            .params
                            .iter()
                            .map(|p| {
                                let np = self.fresh_id();
                                inner_map.insert(p.id, np);
                                crate::lcnf::LcnfParam {
                                    id: np,
                                    name: p.name.clone(),
                                    ty: p.ty.clone(),
                                    erased: p.erased,
                                    borrowed: p.borrowed,
                                }
                            })
                            .collect();
                        let new_body = self.clone_expr_with_map(&alt.body, &mut inner_map);
                        crate::lcnf::LcnfAlt {
                            ctor_name: alt.ctor_name.clone(),
                            ctor_tag: alt.ctor_tag,
                            params: new_params,
                            body: new_body,
                        }
                    })
                    .collect();
                let new_default = default
                    .as_ref()
                    .map(|d| Box::new(self.clone_expr_with_map(d, map)));
                LcnfExpr::Case {
                    scrutinee: new_scrutinee,
                    scrutinee_ty: scrutinee_ty.clone(),
                    alts: new_alts,
                    default: new_default,
                }
            }
            LcnfExpr::Return(arg) => LcnfExpr::Return(self.clone_arg_with_map(arg, map)),
            LcnfExpr::Unreachable => LcnfExpr::Unreachable,
            LcnfExpr::TailCall(func, args) => {
                let new_func = self.clone_arg_with_map(func, map);
                let new_args = args
                    .iter()
                    .map(|a| self.clone_arg_with_map(a, map))
                    .collect();
                LcnfExpr::TailCall(new_func, new_args)
            }
        }
    }
    pub(super) fn clone_value_with_map(
        &mut self,
        value: &LcnfLetValue,
        map: &HashMap<LcnfVarId, LcnfVarId>,
    ) -> LcnfLetValue {
        match value {
            LcnfLetValue::App(func, args) => {
                let new_func = self.clone_arg_with_map(func, map);
                let new_args = args
                    .iter()
                    .map(|a| self.clone_arg_with_map(a, map))
                    .collect();
                LcnfLetValue::App(new_func, new_args)
            }
            LcnfLetValue::Proj(name, idx, var) => {
                let new_var = map.get(var).copied().unwrap_or(*var);
                LcnfLetValue::Proj(name.clone(), *idx, new_var)
            }
            LcnfLetValue::Ctor(name, tag, args) => {
                let new_args = args
                    .iter()
                    .map(|a| self.clone_arg_with_map(a, map))
                    .collect();
                LcnfLetValue::Ctor(name.clone(), *tag, new_args)
            }
            LcnfLetValue::Lit(lit) => LcnfLetValue::Lit(lit.clone()),
            LcnfLetValue::Erased => LcnfLetValue::Erased,
            LcnfLetValue::FVar(id) => {
                let new_id = map.get(id).copied().unwrap_or(*id);
                LcnfLetValue::FVar(new_id)
            }
            LcnfLetValue::Reset(var) => {
                let new_var = map.get(var).copied().unwrap_or(*var);
                LcnfLetValue::Reset(new_var)
            }
            LcnfLetValue::Reuse(slot, name, tag, args) => {
                let new_slot = map.get(slot).copied().unwrap_or(*slot);
                let new_args = args
                    .iter()
                    .map(|a| self.clone_arg_with_map(a, map))
                    .collect();
                LcnfLetValue::Reuse(new_slot, name.clone(), *tag, new_args)
            }
        }
    }
    pub(super) fn clone_arg_with_map(
        &self,
        arg: &crate::lcnf::LcnfArg,
        map: &HashMap<LcnfVarId, LcnfVarId>,
    ) -> crate::lcnf::LcnfArg {
        match arg {
            crate::lcnf::LcnfArg::Var(id) => {
                crate::lcnf::LcnfArg::Var(map.get(id).copied().unwrap_or(*id))
            }
            crate::lcnf::LcnfArg::Lit(lit) => crate::lcnf::LcnfArg::Lit(lit.clone()),
            crate::lcnf::LcnfArg::Erased => crate::lcnf::LcnfArg::Erased,
            crate::lcnf::LcnfArg::Type(ty) => crate::lcnf::LcnfArg::Type(ty.clone()),
        }
    }
}
/// Information about a loop peeling transformation.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LoopPeelingInfo {
    /// The loop to peel.
    pub loop_info: LoopInfo,
    /// Number of iterations to peel from the front.
    pub peel_front: u64,
    /// Number of iterations to peel from the back.
    pub peel_back: u64,
    /// Whether peeling is beneficial.
    pub is_beneficial: bool,
}
impl LoopPeelingInfo {
    /// Create a peeling decision for the given loop.
    #[allow(dead_code)]
    pub fn new(loop_info: LoopInfo, peel_front: u64, peel_back: u64) -> Self {
        let is_beneficial =
            loop_info.is_innermost && loop_info.trip_count.unwrap_or(0) > peel_front + peel_back;
        LoopPeelingInfo {
            loop_info,
            peel_front,
            peel_back,
            is_beneficial,
        }
    }
    /// Number of iterations remaining after peeling.
    #[allow(dead_code)]
    pub fn remaining_iterations(&self) -> u64 {
        let tc = self.loop_info.trip_count.unwrap_or(0);
        tc.saturating_sub(self.peel_front + self.peel_back)
    }
}
/// Configuration for the loop unrolling pass.
#[derive(Debug, Clone)]
pub struct UnrollConfig {
    /// Maximum unroll factor for partial unrolling.
    pub max_unroll_factor: u32,
    /// Maximum code size after unrolling (in abstract units).
    pub max_unrolled_size: u64,
    /// Trip count threshold below which to fully unroll.
    pub unroll_full_threshold: u64,
    /// Whether to attempt vectorizable unrolling.
    pub enable_vectorizable: bool,
    /// Whether to perform loop jamming.
    pub enable_jamming: bool,
    /// Minimum trip count to consider partial unrolling.
    pub min_trip_count_for_partial: u64,
}
impl UnrollConfig {
    /// Create a config tuned for aggressive unrolling.
    pub fn aggressive() -> Self {
        UnrollConfig {
            max_unroll_factor: 16,
            max_unrolled_size: 512,
            unroll_full_threshold: 32,
            enable_vectorizable: true,
            enable_jamming: true,
            min_trip_count_for_partial: 2,
        }
    }
    /// Create a config tuned for minimal code growth.
    pub fn conservative() -> Self {
        UnrollConfig {
            max_unroll_factor: 2,
            max_unrolled_size: 64,
            unroll_full_threshold: 4,
            enable_vectorizable: false,
            enable_jamming: false,
            min_trip_count_for_partial: 8,
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LUCacheEntry {
    pub key: String,
    pub data: Vec<u8>,
    pub timestamp: u64,
    pub valid: bool,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LUWorklist {
    pub(super) items: std::collections::VecDeque<u32>,
    pub(super) in_worklist: std::collections::HashSet<u32>,
}
impl LUWorklist {
    #[allow(dead_code)]
    pub fn new() -> Self {
        LUWorklist {
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
/// Estimates loop trip counts from available constant information.
#[allow(dead_code)]
pub struct TripCountEstimator {
    /// Constant variable values known at the estimation point.
    pub constants: HashMap<LcnfVarId, u64>,
}
impl TripCountEstimator {
    /// Create a new estimator with no known constants.
    #[allow(dead_code)]
    pub fn new() -> Self {
        TripCountEstimator {
            constants: HashMap::new(),
        }
    }
    /// Record a constant binding.
    #[allow(dead_code)]
    pub fn bind(&mut self, id: LcnfVarId, value: u64) {
        self.constants.insert(id, value);
    }
    /// Estimate the trip count of a loop.
    #[allow(dead_code)]
    pub fn estimate(&self, info: &LoopInfo) -> Option<u64> {
        if info.step == 0 {
            return None;
        }
        if info.end <= info.start {
            return Some(0);
        }
        Some((info.end - info.start).div_ceil(info.step))
    }
    /// Determine if a loop is likely a power-of-two trip count.
    #[allow(dead_code)]
    pub fn is_power_of_two_trip_count(&self, info: &LoopInfo) -> bool {
        if let Some(tc) = self.estimate(info) {
            tc > 0 && tc.is_power_of_two()
        } else {
            false
        }
    }
    /// Estimate the trip count with a maximum cap.
    #[allow(dead_code)]
    pub fn estimate_capped(&self, info: &LoopInfo, cap: u64) -> u64 {
        self.estimate(info).unwrap_or(cap).min(cap)
    }
}
/// Summary statistics produced after running the unrolling pass.
#[derive(Debug, Clone, Default)]
pub struct UnrollReport {
    /// Total loops analyzed across all functions.
    pub loops_analyzed: usize,
    /// Loops that were actually unrolled.
    pub loops_unrolled: usize,
    /// Loops that were fully unrolled.
    pub full_unrolls: usize,
    /// Loops that were partially unrolled.
    pub partial_unrolls: usize,
    /// Loops fused via jamming.
    pub jammed_loops: usize,
    /// Loops marked for vectorization.
    pub vectorizable_loops: usize,
    /// Estimated total speedup (1.0 = no change).
    pub estimated_speedup: f64,
}
impl UnrollReport {
    /// Merge another report into `self`.
    pub fn merge(&mut self, other: &UnrollReport) {
        self.loops_analyzed += other.loops_analyzed;
        self.loops_unrolled += other.loops_unrolled;
        self.full_unrolls += other.full_unrolls;
        self.partial_unrolls += other.partial_unrolls;
        self.jammed_loops += other.jammed_loops;
        self.vectorizable_loops += other.vectorizable_loops;
        self.estimated_speedup =
            (self.estimated_speedup + other.estimated_speedup) / 2.0_f64.max(1.0);
    }
    /// Human-readable summary.
    pub fn summary(&self) -> String {
        format!(
            "UnrollReport {{ analyzed={}, unrolled={}, full={}, partial={}, \
             jammed={}, vectorizable={}, speedup={:.2}x }}",
            self.loops_analyzed,
            self.loops_unrolled,
            self.full_unrolls,
            self.partial_unrolls,
            self.jammed_loops,
            self.vectorizable_loops,
            self.estimated_speedup,
        )
    }
}
/// A source-level annotation that advises the unroller.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnrollAnnotation {
    /// `#[unroll]` — let the pass decide.
    Auto,
    /// `#[unroll(full)]` — fully unroll.
    Full,
    /// `#[unroll(factor = N)]` — unroll by factor N.
    Factor(u32),
    /// `#[unroll(disable)]` — never unroll this loop.
    Disable,
    /// `#[vectorize]` — mark for vectorization.
    Vectorize,
    /// `#[no_vectorize]` — suppress vectorization.
    NoVectorize,
}
impl UnrollAnnotation {
    /// Parse an annotation from a string.
    #[allow(dead_code)]
    pub fn parse(s: &str) -> Option<Self> {
        match s.trim() {
            "unroll" | "unroll(auto)" => Some(UnrollAnnotation::Auto),
            "unroll(full)" => Some(UnrollAnnotation::Full),
            "unroll(disable)" => Some(UnrollAnnotation::Disable),
            "vectorize" => Some(UnrollAnnotation::Vectorize),
            "no_vectorize" => Some(UnrollAnnotation::NoVectorize),
            other => {
                if let Some(rest) = other.strip_prefix("unroll(factor=") {
                    let n_str = rest.trim_end_matches(')');
                    n_str.parse::<u32>().ok().map(UnrollAnnotation::Factor)
                } else {
                    None
                }
            }
        }
    }
    /// Convert to the corresponding `UnrollFactor`.
    #[allow(dead_code)]
    pub fn to_unroll_factor(&self) -> Option<UnrollFactor> {
        match self {
            UnrollAnnotation::Auto => None,
            UnrollAnnotation::Full => Some(UnrollFactor::Full),
            UnrollAnnotation::Factor(n) => Some(UnrollFactor::Partial(*n)),
            UnrollAnnotation::Disable => None,
            UnrollAnnotation::Vectorize => Some(UnrollFactor::Vectorizable(8)),
            UnrollAnnotation::NoVectorize => None,
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LUPassConfig {
    pub phase: LUPassPhase,
    pub enabled: bool,
    pub max_iterations: u32,
    pub debug_output: bool,
    pub pass_name: String,
}
impl LUPassConfig {
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>, phase: LUPassPhase) -> Self {
        LUPassConfig {
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
/// A pair of loops that are candidates for loop fusion (jamming).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LoopFusionPair {
    /// The first loop (lexically earlier).
    pub first: LoopInfo,
    /// The second loop (lexically later).
    pub second: LoopInfo,
    /// Whether the fusion is legal.
    pub is_legal: bool,
    /// Estimated savings from fusion (in abstract units).
    pub estimated_savings: i64,
}
impl LoopFusionPair {
    /// Create a new fusion pair.
    #[allow(dead_code)]
    pub fn new(first: LoopInfo, second: LoopInfo) -> Self {
        let compatible =
            first.start == second.start && first.end == second.end && first.step == second.step;
        let is_legal = compatible;
        let estimated_savings = if is_legal { 10 } else { -1 };
        LoopFusionPair {
            first,
            second,
            is_legal,
            estimated_savings,
        }
    }
    /// Whether this pair is profitable to fuse.
    #[allow(dead_code)]
    pub fn is_profitable(&self) -> bool {
        self.is_legal && self.estimated_savings > 0
    }
    /// Emit a description of the fusion.
    #[allow(dead_code)]
    pub fn describe(&self) -> String {
        if self.is_legal {
            format!(
                "Fuse loops over [{}..{}] step {} (savings: {})",
                self.first.start, self.first.end, self.first.step, self.estimated_savings
            )
        } else {
            "Fusion illegal: incompatible loop bounds".to_string()
        }
    }
}
/// SIMD width options for vectorization.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimdWidth {
    /// 64-bit (2 × f32 or 1 × f64).
    W64 = 64,
    /// 128-bit SSE / NEON.
    W128 = 128,
    /// 256-bit AVX2.
    W256 = 256,
    /// 512-bit AVX-512.
    W512 = 512,
}
impl SimdWidth {
    /// Number of 32-bit lanes.
    #[allow(dead_code)]
    pub fn lanes_i32(self) -> u32 {
        self as u32 / 32
    }
    /// Number of 64-bit lanes.
    #[allow(dead_code)]
    pub fn lanes_i64(self) -> u32 {
        self as u32 / 64
    }
}
/// Dependence information for a loop.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LoopDependenceInfo {
    /// The loop this analysis covers.
    pub loop_var: LcnfVarId,
    /// Whether the loop has any loop-carried dependences.
    pub has_loop_carried: bool,
    /// The kind of the strongest dependence, if any.
    pub strongest: DependenceKind,
    /// Variables with loop-carried dependences.
    pub dependent_vars: Vec<LcnfVarId>,
}
impl LoopDependenceInfo {
    /// Create an independent loop dependence record.
    #[allow(dead_code)]
    pub fn independent(loop_var: LcnfVarId) -> Self {
        LoopDependenceInfo {
            loop_var,
            has_loop_carried: false,
            strongest: DependenceKind::Independent,
            dependent_vars: Vec::new(),
        }
    }
    /// Mark this loop as having a read-after-write dependence for `var`.
    #[allow(dead_code)]
    pub fn add_raw(&mut self, var: LcnfVarId) {
        self.has_loop_carried = true;
        self.dependent_vars.push(var);
        if self.strongest == DependenceKind::Independent {
            self.strongest = DependenceKind::ReadAfterWrite;
        }
    }
    /// Check if it is safe to vectorize despite the dependences.
    #[allow(dead_code)]
    pub fn safe_to_vectorize(&self) -> bool {
        !self.has_loop_carried
    }
}
/// A loop-invariant computation that can be hoisted out of the loop.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct HoistCandidate {
    /// The variable being defined by the hoistable expression.
    pub var: LcnfVarId,
    /// The expression value (loop-invariant).
    pub value: LcnfLetValue,
    /// The loop from which this can be hoisted.
    pub from_loop: LcnfVarId,
    /// Estimated savings (how many times the computation is skipped).
    pub saved_iterations: u64,
}
impl HoistCandidate {
    /// Create a new hoist candidate.
    #[allow(dead_code)]
    pub fn new(var: LcnfVarId, value: LcnfLetValue, from_loop: LcnfVarId, trip_count: u64) -> Self {
        HoistCandidate {
            var,
            value,
            from_loop,
            saved_iterations: trip_count.saturating_sub(1),
        }
    }
    /// Whether this hoist is profitable.
    #[allow(dead_code)]
    pub fn is_profitable(&self) -> bool {
        self.saved_iterations > 0
    }
}
/// The strategy to use when unrolling a loop.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnrollFactor {
    /// Completely unroll the loop (only safe for known small trip counts).
    Full,
    /// Unroll by the given factor (must be a power of two).
    Partial(u32),
    /// Fuse two adjacent loops over the same range (loop jamming).
    Jamming,
    /// Unroll by the given SIMD width and mark for the vectorizer.
    Vectorizable(u32),
}
impl UnrollFactor {
    /// Returns the numeric factor, or `None` for `Full` / `Jamming`.
    pub fn factor(&self) -> Option<u32> {
        match self {
            UnrollFactor::Full => None,
            UnrollFactor::Partial(n) => Some(*n),
            UnrollFactor::Jamming => None,
            UnrollFactor::Vectorizable(n) => Some(*n),
        }
    }
    /// Human-readable name.
    pub fn name(&self) -> &'static str {
        match self {
            UnrollFactor::Full => "full",
            UnrollFactor::Partial(_) => "partial",
            UnrollFactor::Jamming => "jamming",
            UnrollFactor::Vectorizable(_) => "vectorizable",
        }
    }
}
/// An abstract cost model for estimating loop execution cost.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LoopCostModel {
    /// Cost per loop iteration (in abstract cycles).
    pub iter_cost: f64,
    /// Loop overhead cost (init + branch check).
    pub overhead_cost: f64,
    /// Memory access cost per iteration.
    pub memory_cost: f64,
    /// Branch misprediction probability.
    pub branch_miss_prob: f64,
}
impl LoopCostModel {
    /// Create a default cost model.
    #[allow(dead_code)]
    pub fn default_model() -> Self {
        LoopCostModel {
            iter_cost: 1.0,
            overhead_cost: 3.0,
            memory_cost: 0.5,
            branch_miss_prob: 0.02,
        }
    }
    /// Estimate total execution cost for a loop.
    #[allow(dead_code)]
    pub fn estimate_cost(&self, info: &LoopInfo) -> f64 {
        let tc = info.trip_count.unwrap_or(8) as f64;
        let iter_total = tc * (self.iter_cost + self.memory_cost);
        let overhead = self.overhead_cost;
        let branch_cost = tc * self.branch_miss_prob * 15.0;
        iter_total + overhead + branch_cost
    }
    /// Estimate cost after unrolling by `factor`.
    #[allow(dead_code)]
    pub fn estimate_unrolled_cost(&self, info: &LoopInfo, factor: u32) -> f64 {
        let tc = info.trip_count.unwrap_or(8) as f64;
        let new_iters = (tc / factor as f64).ceil();
        let iter_total = new_iters * (self.iter_cost * factor as f64 + self.memory_cost);
        let overhead = self.overhead_cost;
        let branch_cost = new_iters * self.branch_miss_prob * 15.0;
        iter_total + overhead + branch_cost
    }
    /// Speedup ratio from unrolling by `factor`.
    #[allow(dead_code)]
    pub fn speedup(&self, info: &LoopInfo, factor: u32) -> f64 {
        let original = self.estimate_cost(info);
        let unrolled = self.estimate_unrolled_cost(info, factor);
        if unrolled == 0.0 {
            1.0
        } else {
            original / unrolled
        }
    }
}
/// Vectorization metadata for a loop.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct VectorizationInfo {
    /// The loop's induction variable.
    pub loop_var: LcnfVarId,
    /// The SIMD width chosen.
    pub simd_width: SimdWidth,
    /// Whether alignment is guaranteed.
    pub aligned: bool,
    /// Whether the loop count is a multiple of the SIMD lane count.
    pub count_is_multiple: bool,
    /// Estimated speedup from vectorization.
    pub estimated_speedup: f64,
}
impl VectorizationInfo {
    /// Create a vectorization info record.
    #[allow(dead_code)]
    pub fn new(loop_var: LcnfVarId, simd_width: SimdWidth, trip_count: Option<u64>) -> Self {
        let lanes = simd_width.lanes_i32();
        let count_is_multiple = trip_count.map(|tc| tc % lanes as u64 == 0).unwrap_or(false);
        let estimated_speedup = if count_is_multiple {
            lanes as f64 * 0.8
        } else {
            lanes as f64 * 0.6
        };
        VectorizationInfo {
            loop_var,
            simd_width,
            aligned: false,
            count_is_multiple,
            estimated_speedup,
        }
    }
    /// Mark this loop as having aligned memory accesses.
    #[allow(dead_code)]
    pub fn with_aligned(mut self) -> Self {
        self.aligned = true;
        self.estimated_speedup *= 1.1;
        self
    }
}
/// An ordered sequence of loop transforms to apply to a single loop.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct LoopTransformSequence {
    /// The transforms in application order.
    pub transforms: Vec<LoopTransform>,
}
impl LoopTransformSequence {
    /// Create an empty sequence.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    /// Add a transform.
    #[allow(dead_code)]
    pub fn push(mut self, t: LoopTransform) -> Self {
        self.transforms.push(t);
        self
    }
    /// Compute the total estimated size multiplier.
    #[allow(dead_code)]
    pub fn total_size_multiplier(&self) -> f64 {
        self.transforms
            .iter()
            .map(|t| t.size_multiplier())
            .fold(1.0_f64, |acc, m| acc * m)
    }
    /// Describe the sequence.
    #[allow(dead_code)]
    pub fn describe(&self) -> String {
        self.transforms
            .iter()
            .map(|t| t.name())
            .collect::<Vec<_>>()
            .join(" → ")
    }
}
/// A pipeline that chains multiple loop optimization passes.
#[allow(dead_code)]
pub struct LoopOptPipeline {
    /// The passes in order of execution.
    pub passes: Vec<Box<dyn LoopOptPass>>,
    /// Combined report from all passes.
    pub report: UnrollReport,
}
impl LoopOptPipeline {
    /// Create an empty pipeline.
    #[allow(dead_code)]
    pub fn new() -> Self {
        LoopOptPipeline {
            passes: Vec::new(),
            report: UnrollReport::default(),
        }
    }
    /// Add a pass to the pipeline.
    #[allow(dead_code)]
    pub fn add_pass(&mut self, pass: Box<dyn LoopOptPass>) {
        self.passes.push(pass);
    }
    /// Run all passes in sequence.
    #[allow(dead_code)]
    pub fn run(&mut self, decls: &mut [LcnfFunDecl]) {
        for pass in &mut self.passes {
            let r = pass.run_pass(decls);
            self.report.merge(&r);
        }
    }
    /// Number of passes in the pipeline.
    #[allow(dead_code)]
    pub fn num_passes(&self) -> usize {
        self.passes.len()
    }
}
/// A wrapper implementing `LoopOptPass` for `LoopUnrollPass`.
#[allow(dead_code)]
pub struct UnrollPassAdapter {
    pub(super) inner: LoopUnrollPass,
}
impl UnrollPassAdapter {
    /// Create a new adapter.
    #[allow(dead_code)]
    pub fn new(config: UnrollConfig) -> Self {
        UnrollPassAdapter {
            inner: LoopUnrollPass::new(config),
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LUAnalysisCache {
    pub(super) entries: std::collections::HashMap<String, LUCacheEntry>,
    pub(super) max_size: usize,
    pub(super) hits: u64,
    pub(super) misses: u64,
}
impl LUAnalysisCache {
    #[allow(dead_code)]
    pub fn new(max_size: usize) -> Self {
        LUAnalysisCache {
            entries: std::collections::HashMap::new(),
            max_size,
            hits: 0,
            misses: 0,
        }
    }
    #[allow(dead_code)]
    pub fn get(&mut self, key: &str) -> Option<&LUCacheEntry> {
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
            LUCacheEntry {
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
/// An individual loop transformation.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoopTransform {
    /// Unroll by the given factor.
    Unroll(u32),
    /// Fully unroll a small loop.
    FullUnroll,
    /// Peel the given number of iterations from front and back.
    Peel { front: u64, back: u64 },
    /// Fuse with the adjacent loop.
    Fuse,
    /// Interchange inner and outer loops.
    Interchange,
    /// Tile the loop by the given block size.
    Tile(u64),
    /// Mark as vectorizable.
    Vectorize(u32),
}
impl LoopTransform {
    /// Human-readable name.
    #[allow(dead_code)]
    pub fn name(&self) -> String {
        match self {
            LoopTransform::Unroll(f) => format!("unroll({})", f),
            LoopTransform::FullUnroll => "full_unroll".to_string(),
            LoopTransform::Peel { front, back } => format!("peel({},{})", front, back),
            LoopTransform::Fuse => "fuse".to_string(),
            LoopTransform::Interchange => "interchange".to_string(),
            LoopTransform::Tile(s) => format!("tile({})", s),
            LoopTransform::Vectorize(w) => format!("vectorize({})", w),
        }
    }
    /// Estimated code-size multiplier for this transform.
    #[allow(dead_code)]
    pub fn size_multiplier(&self) -> f64 {
        match self {
            LoopTransform::Unroll(f) => *f as f64,
            LoopTransform::FullUnroll => 1.0,
            LoopTransform::Peel { front, back } => 1.0 + (*front + *back) as f64 * 0.1,
            LoopTransform::Fuse => 0.9,
            LoopTransform::Interchange => 1.0,
            LoopTransform::Tile(_) => 1.1,
            LoopTransform::Vectorize(w) => *w as f64 * 0.5,
        }
    }
}
/// A loop that has been identified as a profitable unrolling candidate.
#[derive(Debug, Clone)]
pub struct UnrollCandidate {
    /// The name of the containing function.
    pub function_name: String,
    /// The detected loop.
    pub loop_info: LoopInfo,
    /// The recommended unrolling strategy.
    pub recommended_factor: UnrollFactor,
    /// Estimated runtime savings (in abstract units).
    pub estimated_savings: i64,
}
impl UnrollCandidate {
    /// Create a new unroll candidate.
    pub fn new(
        function_name: impl Into<String>,
        loop_info: LoopInfo,
        recommended_factor: UnrollFactor,
        estimated_savings: i64,
    ) -> Self {
        UnrollCandidate {
            function_name: function_name.into(),
            loop_info,
            recommended_factor,
            estimated_savings,
        }
    }
    /// Whether the candidate is profitable (positive savings).
    pub fn is_profitable(&self) -> bool {
        self.estimated_savings > 0
    }
}
/// Detailed per-function unrolling statistics.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct LoopUnrollStats {
    /// Function name.
    pub function_name: String,
    /// Loops analyzed in this function.
    pub loops_analyzed: usize,
    /// Loops unrolled in this function.
    pub loops_unrolled: usize,
    /// Total body size before unrolling.
    pub original_size: u64,
    /// Total body size after unrolling.
    pub unrolled_size: u64,
    /// Number of vectorizable loops.
    pub vectorizable: usize,
}
impl LoopUnrollStats {
    /// Create stats for a function.
    #[allow(dead_code)]
    pub fn new(function_name: impl Into<String>) -> Self {
        LoopUnrollStats {
            function_name: function_name.into(),
            ..Default::default()
        }
    }
    /// Code expansion ratio after unrolling.
    #[allow(dead_code)]
    pub fn expansion_ratio(&self) -> f64 {
        if self.original_size == 0 {
            1.0
        } else {
            self.unrolled_size as f64 / self.original_size as f64
        }
    }
    /// Human-readable summary.
    #[allow(dead_code)]
    pub fn summary(&self) -> String {
        format!(
            "{}: analyzed={}, unrolled={}, expansion={:.2}x, vectorizable={}",
            self.function_name,
            self.loops_analyzed,
            self.loops_unrolled,
            self.expansion_ratio(),
            self.vectorizable,
        )
    }
}
