use super::super::functions::LoopOptPass;
use crate::lcnf::{LcnfArg, LcnfExpr, LcnfFunDecl, LcnfLetValue, LcnfLit, LcnfVarId};
use std::collections::HashMap;
use std::collections::{HashSet, VecDeque};

use super::defs::*;

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
            .sort_by_key(|c| std::cmp::Reverse(c.estimated_savings));
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
    pub(crate) fn collect_from_expr(
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
    pub(crate) fn value_uses_var(&self, value: &LcnfLetValue, var: LcnfVarId) -> bool {
        match value {
            LcnfLetValue::FVar(id) => *id == var,
            LcnfLetValue::App(f_arg, args) => {
                self.arg_is_var_id(f_arg, var) || args.iter().any(|a| self.arg_is_var_id(a, var))
            }
            _ => false,
        }
    }
    pub(crate) fn arg_is_var_id(&self, arg: &LcnfArg, var: LcnfVarId) -> bool {
        matches!(arg, LcnfArg::Var(id) if * id == var)
    }
    /// Number of profitable hoist candidates.
    #[allow(dead_code)]
    pub fn num_profitable(&self) -> usize {
        self.candidates.iter().filter(|c| c.is_profitable()).count()
    }
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
