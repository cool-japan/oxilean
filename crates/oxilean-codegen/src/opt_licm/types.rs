//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;
use std::collections::HashSet;

use super::functions::*;
use std::collections::{HashMap, VecDeque};

/// Loop level (nesting depth)
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LoopLevelId(pub u32);
/// Recognizes structural patterns in let-values that are always loop-invariant.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum InvariantPattern {
    /// A literal constant.
    Literal,
    /// An erased value (always invariant).
    Erased,
    /// A free-variable reference that is not defined inside the loop.
    ExternalFVar,
    /// A projection of an external variable.
    ExternalProj,
    /// A constructor whose all args are invariant.
    InvariantCtor,
}
/// The main Loop Invariant Code Motion pass.
pub struct LICMPass {
    pub(super) config: LICMConfig,
    pub(super) report: LICMReport,
}
impl LICMPass {
    /// Create a new LICM pass with the given configuration.
    pub fn new(config: LICMConfig) -> Self {
        LICMPass {
            config,
            report: LICMReport::default(),
        }
    }
    /// Run the LICM pass over all function declarations, mutating them
    /// in-place to hoist loop-invariant bindings.
    pub fn run(&mut self, decls: &mut [LcnfFunDecl]) {
        for decl in decls.iter_mut() {
            let loops = self.find_loops(decl);
            self.report.loops_analyzed += loops.len();
            for lp in &loops {
                self.hoist_invariants(decl, lp);
            }
        }
    }
    /// Identify all loops in a function declaration by scanning for
    /// recursive `Let` bindings (self-referencing variables).
    pub fn find_loops(&self, decl: &LcnfFunDecl) -> Vec<LoopStructure> {
        let mut loops = Vec::new();
        let mut depth = 0u32;
        Self::find_loops_in_expr(&decl.body, &mut depth, &mut loops);
        loops
    }
    /// Check whether a `LcnfLetValue` is loop-invariant with respect to
    /// the given loop structure (i.e., none of its free variables are
    /// defined inside the loop body).
    pub fn is_loop_invariant(&self, value: &LcnfLetValue, lp: &LoopStructure) -> bool {
        let free = free_vars_of_let_value(value);
        if let LcnfLetValue::App(LcnfArg::Var(f), _) = value {
            if f == &lp.header {
                return false;
            }
        }
        if !self.config.hoist_function_calls {
            if let LcnfLetValue::App(..) = value {
                return false;
            }
        }
        free.iter().all(|v| !lp.body_vars.contains(v))
    }
    /// Hoist all loop-invariant bindings from `lp` into a preheader and
    /// update `decl.body` accordingly.
    pub fn hoist_invariants(&mut self, decl: &mut LcnfFunDecl, lp: &LoopStructure) {
        let mut candidates: Vec<HoistCandidate> = Vec::new();
        Self::collect_invariant_candidates(&decl.body, lp, &self.config, &mut candidates);
        let threshold = self.config.min_savings_threshold;
        candidates.retain(|c| c.savings_estimate >= threshold);
        if candidates.is_empty() {
            return;
        }
        let hoisted_vars: HashSet<LcnfVarId> = candidates.iter().map(|c| c.expr.var).collect();
        self.report.expressions_hoisted += hoisted_vars.len();
        self.report.estimated_savings += candidates
            .iter()
            .map(|c| c.savings_estimate as u64)
            .sum::<u64>();
        let mut new_body = decl.body.clone();
        remove_hoisted_bindings(&mut new_body, &hoisted_vars);
        for c in candidates.iter().rev() {
            new_body = LcnfExpr::Let {
                id: c.expr.var,
                name: format!("{}", c.expr.var),
                ty: c.expr.ty.clone(),
                value: c.expr.value.clone(),
                body: Box::new(new_body),
            };
        }
        decl.body = new_body;
    }
    /// Return a copy of the accumulated statistics report.
    pub fn report(&self) -> LICMReport {
        self.report.clone()
    }
    /// Recursively walk `expr` looking for recursive `Let` bindings.
    pub(super) fn find_loops_in_expr(
        expr: &LcnfExpr,
        depth: &mut u32,
        loops: &mut Vec<LoopStructure>,
    ) {
        match expr {
            LcnfExpr::Let { id, body, .. } => {
                let mut call_targets = HashSet::new();
                collect_call_targets(body, &mut call_targets);
                if call_targets.contains(id) {
                    let mut body_vars = HashSet::new();
                    collect_defined_vars(body, &mut body_vars);
                    let exit_vars = {
                        let mut ev = HashSet::new();
                        collect_used_vars(body, &mut ev);
                        ev.retain(|v| !body_vars.contains(v));
                        ev
                    };
                    loops.push(LoopStructure {
                        header: *id,
                        body_vars,
                        exit_vars,
                        nest_depth: *depth,
                    });
                    *depth += 1;
                    Self::find_loops_in_expr(body, depth, loops);
                    *depth -= 1;
                } else {
                    Self::find_loops_in_expr(body, depth, loops);
                }
            }
            LcnfExpr::Case { alts, default, .. } => {
                for alt in alts {
                    Self::find_loops_in_expr(&alt.body, depth, loops);
                }
                if let Some(d) = default {
                    Self::find_loops_in_expr(d, depth, loops);
                }
            }
            LcnfExpr::Return(_) | LcnfExpr::Unreachable | LcnfExpr::TailCall(..) => {}
        }
    }
    /// Collect `HoistCandidate`s from inside the loop body.
    pub(super) fn collect_invariant_candidates(
        expr: &LcnfExpr,
        lp: &LoopStructure,
        config: &LICMConfig,
        out: &mut Vec<HoistCandidate>,
    ) {
        match expr {
            LcnfExpr::Let {
                id,
                name: _,
                ty,
                value,
                body,
            } => {
                if lp.body_vars.contains(id) {
                    let free = free_vars_of_let_value(value);
                    let all_outside = free.iter().all(|v| !lp.body_vars.contains(v));
                    let is_call = matches!(value, LcnfLetValue::App(..));
                    let ok_call = !is_call || config.hoist_function_calls;
                    let is_self_call = matches!(
                        value, LcnfLetValue::App(LcnfArg::Var(f), ..) if f == & lp.header
                    );
                    if all_outside && ok_call && !is_self_call {
                        out.push(HoistCandidate {
                            expr: LoopInvariantExpr {
                                var: *id,
                                value: value.clone(),
                                ty: ty.clone(),
                                loop_depth: lp.nest_depth,
                            },
                            target_loop_header: lp.header,
                            savings_estimate: 10,
                        });
                    }
                }
                Self::collect_invariant_candidates(body, lp, config, out);
            }
            LcnfExpr::Case { alts, default, .. } => {
                for alt in alts {
                    Self::collect_invariant_candidates(&alt.body, lp, config, out);
                }
                if let Some(d) = default {
                    Self::collect_invariant_candidates(d, lp, config, out);
                }
            }
            LcnfExpr::Return(_) | LcnfExpr::Unreachable | LcnfExpr::TailCall(..) => {}
        }
    }
}
/// LICM pass config (extended)
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LicmConfigExt {
    pub enable_hoist: bool,
    pub enable_sink: bool,
    pub enable_speculative_hoist: bool,
    pub max_hoist_cost: i32,
    pub min_trip_count: u64,
    pub hoist_stores: bool,
    pub hoist_calls: bool,
    pub max_loop_depth: u32,
}
/// LICM profiler
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct LicmExtProfiler {
    pub timings: Vec<(String, u64)>,
}
#[allow(dead_code)]
impl LicmExtProfiler {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn record(&mut self, pass: &str, us: u64) {
        self.timings.push((pass.to_string(), us));
    }
    pub fn total_us(&self) -> u64 {
        self.timings.iter().map(|(_, t)| *t).sum()
    }
}
/// LICM loop analysis result
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct LicmLoopAnalysis {
    pub loop_tree: LoopTree,
    pub preheaders: LicmPreheaderMap,
    pub invariant_vars: std::collections::HashMap<u32, LoopLevelId>,
}
#[allow(dead_code)]
impl LicmLoopAnalysis {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn add_invariant(&mut self, var: u32, loop_id: LoopLevelId) {
        self.invariant_vars.insert(var, loop_id);
    }
    pub fn is_invariant(&self, var: u32) -> bool {
        self.invariant_vars.contains_key(&var)
    }
    pub fn invariant_count(&self) -> usize {
        self.invariant_vars.len()
    }
}
/// LICM preheader builder
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct LicmPreheaderBuilder {
    pub created: Vec<LoopPreheader>,
    pub next_block_id: u32,
}
#[allow(dead_code)]
impl LicmPreheaderBuilder {
    pub fn new(start_id: u32) -> Self {
        Self {
            created: Vec::new(),
            next_block_id: start_id,
        }
    }
    pub fn create_preheader(&mut self, loop_id: LoopLevelId) -> LoopPreheader {
        let ph = LoopPreheader {
            loop_id,
            preheader_block: self.next_block_id,
            inserted_insts: Vec::new(),
        };
        self.next_block_id += 1;
        self.created.push(ph.clone());
        ph
    }
    pub fn count(&self) -> usize {
        self.created.len()
    }
}
/// LICM id generator
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct LicmExtIdGen {
    pub(super) counter: u32,
}
#[allow(dead_code)]
impl LicmExtIdGen {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn next(&mut self) -> u32 {
        let id = self.counter;
        self.counter += 1;
        id
    }
}
/// LICM scheduler (ordering of hoisted instructions)
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct LicmScheduler {
    pub order: Vec<u32>,
    pub scheduled: std::collections::HashSet<u32>,
}
#[allow(dead_code)]
impl LicmScheduler {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn schedule(&mut self, inst: u32) {
        if self.scheduled.insert(inst) {
            self.order.push(inst);
        }
    }
    pub fn is_scheduled(&self, inst: u32) -> bool {
        self.scheduled.contains(&inst)
    }
    pub fn scheduled_count(&self) -> usize {
        self.order.len()
    }
}
/// LICM hoistable expression
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LicmHoistCandidate {
    pub inst_id: u32,
    pub loop_id: LoopLevelId,
    pub cost: i32,
    pub is_side_effect_free: bool,
    pub operands: Vec<u32>,
    pub def_uses: usize,
}
/// Measures the "complexity" of a loop body to guide hoisting aggressiveness.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct LoopBodyComplexity {
    /// Total number of let-bindings in the body.
    pub let_count: usize,
    /// Number of function application let-values.
    pub app_count: usize,
    /// Number of constructor let-values.
    pub ctor_count: usize,
    /// Number of case expressions in the body.
    pub case_count: usize,
    /// Maximum depth of nested case expressions.
    pub max_case_depth: usize,
}
impl LoopBodyComplexity {
    /// Compute the complexity of an expression.
    #[allow(dead_code)]
    pub fn compute(expr: &LcnfExpr) -> Self {
        let mut c = LoopBodyComplexity::default();
        c.visit(expr, 0);
        c
    }
    pub(super) fn visit(&mut self, expr: &LcnfExpr, case_depth: usize) {
        match expr {
            LcnfExpr::Let { value, body, .. } => {
                self.let_count += 1;
                match value {
                    LcnfLetValue::App(..) => self.app_count += 1,
                    LcnfLetValue::Ctor(..) => self.ctor_count += 1,
                    _ => {}
                }
                self.visit(body, case_depth);
            }
            LcnfExpr::Case { alts, default, .. } => {
                self.case_count += 1;
                if case_depth + 1 > self.max_case_depth {
                    self.max_case_depth = case_depth + 1;
                }
                for alt in alts {
                    self.visit(&alt.body, case_depth + 1);
                }
                if let Some(d) = default {
                    self.visit(d, case_depth + 1);
                }
            }
            LcnfExpr::Return(_) | LcnfExpr::Unreachable | LcnfExpr::TailCall(..) => {}
        }
    }
    /// Return a scalar complexity score.
    #[allow(dead_code)]
    pub fn score(&self) -> usize {
        self.let_count + self.app_count * 2 + self.ctor_count + self.case_count * 3
    }
}
/// LICM pass statistics (extended)
#[allow(dead_code)]
#[derive(Debug, Default, Clone)]
pub struct LicmStatsExt {
    pub loops_analyzed: usize,
    pub candidates_found: usize,
    pub hoisted: usize,
    pub sunk: usize,
    pub rejected: usize,
    pub speculative_hoists: usize,
}
/// LICM candidate registry
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct LicmCandidateRegistry {
    pub hoists: Vec<LicmHoistCandidate>,
    pub sinks: Vec<LicmSinkCandidate>,
}
#[allow(dead_code)]
impl LicmCandidateRegistry {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn add_hoist(&mut self, c: LicmHoistCandidate) {
        self.hoists.push(c);
    }
    pub fn add_sink(&mut self, c: LicmSinkCandidate) {
        self.sinks.push(c);
    }
    pub fn hoist_count(&self) -> usize {
        self.hoists.len()
    }
    pub fn sink_count(&self) -> usize {
        self.sinks.len()
    }
    pub fn pure_hoists(&self) -> Vec<&LicmHoistCandidate> {
        self.hoists
            .iter()
            .filter(|c| c.is_side_effect_free)
            .collect()
    }
}
/// LICM pass builder
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct LicmPassBuilder {
    pub config: LicmConfigExt,
    pub loop_tree: LoopTree,
    pub candidates: LicmCandidateRegistry,
    pub stats: LicmStatsExt,
    pub diags: LicmDiagSink,
}
#[allow(dead_code)]
impl LicmPassBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_config(mut self, cfg: LicmConfigExt) -> Self {
        self.config = cfg;
        self
    }
    pub fn report(&self) -> String {
        format!("{}", self.stats)
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LICMCacheEntry {
    pub key: String,
    pub data: Vec<u8>,
    pub timestamp: u64,
    pub valid: bool,
}
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct LicmDiagSink {
    pub diags: Vec<(LicmDiagLevel, String)>,
}
#[allow(dead_code)]
impl LicmDiagSink {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn push(&mut self, level: LicmDiagLevel, msg: &str) {
        self.diags.push((level, msg.to_string()));
    }
    pub fn has_errors(&self) -> bool {
        self.diags.iter().any(|(l, _)| *l == LicmDiagLevel::Error)
    }
}
/// LICM loop info summary
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct LoopInfoSummary {
    pub total_loops: usize,
    pub inner_loops: usize,
    pub countable_loops: usize,
    pub avg_depth: f64,
    pub max_depth: u32,
}
/// Profiling data used to guide LICM decisions.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct LICMProfileData {
    /// Execution counts for each loop (by header var id).
    pub loop_counts: std::collections::HashMap<LcnfVarId, u64>,
    /// Execution counts for each let-binding (by var id).
    pub binding_counts: std::collections::HashMap<LcnfVarId, u64>,
}
impl LICMProfileData {
    /// Create a new empty profile.
    #[allow(dead_code)]
    pub fn new() -> Self {
        LICMProfileData::default()
    }
    /// Record an execution count for a loop.
    #[allow(dead_code)]
    pub fn record_loop(&mut self, header: LcnfVarId, count: u64) {
        self.loop_counts.insert(header, count);
    }
    /// Get the execution count for a loop (default 1 if unknown).
    #[allow(dead_code)]
    pub fn loop_count(&self, header: LcnfVarId) -> u64 {
        self.loop_counts.get(&header).copied().unwrap_or(1)
    }
    /// Record an execution count for a binding.
    #[allow(dead_code)]
    pub fn record_binding(&mut self, var: LcnfVarId, count: u64) {
        self.binding_counts.insert(var, count);
    }
    /// Compute the dynamic savings for a hoist candidate given profile data.
    #[allow(dead_code)]
    pub fn dynamic_savings(&self, candidate: &HoistCandidate) -> u64 {
        let loop_exec = self.loop_count(candidate.target_loop_header);
        (candidate.savings_estimate as u64).saturating_mul(loop_exec)
    }
}
/// Represents a versioned loop: an if-then-else where each branch has a
/// specialized loop copy optimized for a specific condition.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LoopVersion {
    /// The discriminating condition variable.
    pub cond_var: LcnfVarId,
    /// Header of the "true" branch (optimized version).
    pub fast_path_header: LcnfVarId,
    /// Header of the "false" branch (generic version).
    pub slow_path_header: LcnfVarId,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LICMLivenessInfo {
    pub live_in: Vec<std::collections::HashSet<u32>>,
    pub live_out: Vec<std::collections::HashSet<u32>>,
    pub defs: Vec<std::collections::HashSet<u32>>,
    pub uses: Vec<std::collections::HashSet<u32>>,
}
impl LICMLivenessInfo {
    #[allow(dead_code)]
    pub fn new(block_count: usize) -> Self {
        LICMLivenessInfo {
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
/// LICM code stats
#[allow(dead_code)]
#[derive(Debug, Default, Clone)]
pub struct LicmCodeStats {
    pub loops_analyzed: usize,
    pub invariant_exprs: usize,
    pub hoisted: usize,
    pub sunk: usize,
    pub speculative: usize,
    pub rejected: usize,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LICMDepGraph {
    pub(super) nodes: Vec<u32>,
    pub(super) edges: Vec<(u32, u32)>,
}
impl LICMDepGraph {
    #[allow(dead_code)]
    pub fn new() -> Self {
        LICMDepGraph {
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
/// Configuration knobs for the LICM pass.
#[derive(Debug, Clone, Default)]
pub struct LICMConfig {
    /// Minimum estimated savings to justify hoisting an expression.
    /// Defaults to 0 (hoist everything that is safe).
    pub min_savings_threshold: u32,
    /// Whether to hoist calls to named functions.
    /// Defaults to `false` because side-effecting calls must stay in place.
    pub hoist_function_calls: bool,
}
/// Loop preheader (block before loop)
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LoopPreheader {
    pub loop_id: LoopLevelId,
    pub preheader_block: u32,
    pub inserted_insts: Vec<u32>,
}
/// Loop tree node
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LoopNode {
    pub id: LoopLevelId,
    pub header: u32,
    pub body_blocks: Vec<u32>,
    pub parent: Option<LoopLevelId>,
    pub children: Vec<LoopLevelId>,
    pub depth: u32,
    pub is_inner_most: bool,
    pub trip_count: Option<u64>,
    pub is_countable: bool,
}
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum LICMPassPhase {
    Analysis,
    Transformation,
    Verification,
    Cleanup,
}
impl LICMPassPhase {
    #[allow(dead_code)]
    pub fn name(&self) -> &str {
        match self {
            LICMPassPhase::Analysis => "analysis",
            LICMPassPhase::Transformation => "transformation",
            LICMPassPhase::Verification => "verification",
            LICMPassPhase::Cleanup => "cleanup",
        }
    }
    #[allow(dead_code)]
    pub fn is_modifying(&self) -> bool {
        matches!(self, LICMPassPhase::Transformation | LICMPassPhase::Cleanup)
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LICMPassConfig {
    pub phase: LICMPassPhase,
    pub enabled: bool,
    pub max_iterations: u32,
    pub debug_output: bool,
    pub pass_name: String,
}
impl LICMPassConfig {
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>, phase: LICMPassPhase) -> Self {
        LICMPassConfig {
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
/// A synthetic block that is prepended before a loop to receive hoisted code.
///
/// After LICM the `preheader_lets` bindings appear sequentially before the
/// loop entry in the output LCNF.
#[derive(Debug, Clone)]
pub struct PreheaderBlock {
    /// The loop this preheader guards.
    pub loop_header: LcnfVarId,
    /// Bindings to materialise in the preheader, in hoisting order.
    pub preheader_lets: Vec<LoopInvariantExpr>,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LICMDominatorTree {
    pub idom: Vec<Option<u32>>,
    pub dom_children: Vec<Vec<u32>>,
    pub dom_depth: Vec<u32>,
}
impl LICMDominatorTree {
    #[allow(dead_code)]
    pub fn new(size: usize) -> Self {
        LICMDominatorTree {
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
/// Identifies load-like operations (projections, field reads) that are redundant
/// because the same value has already been loaded earlier in the same scope.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct RedundantLoadInfo {
    /// Maps (base_var, field_index) -> binding_var that holds the loaded value.
    pub available_loads: std::collections::HashMap<(LcnfVarId, u32), LcnfVarId>,
    /// Number of redundant loads detected.
    pub redundant_count: usize,
}
impl RedundantLoadInfo {
    /// Create a new empty `RedundantLoadInfo`.
    #[allow(dead_code)]
    pub fn new() -> Self {
        RedundantLoadInfo::default()
    }
    /// Register a load: reading field `field_idx` of `base` into `dest`.
    #[allow(dead_code)]
    pub fn register_load(&mut self, base: LcnfVarId, field_idx: u32, dest: LcnfVarId) {
        self.available_loads.insert((base, field_idx), dest);
    }
    /// Check if a load is redundant. Returns the earlier binding var if so.
    #[allow(dead_code)]
    pub fn lookup_load(&self, base: LcnfVarId, field_idx: u32) -> Option<LcnfVarId> {
        self.available_loads.get(&(base, field_idx)).copied()
    }
    /// Scan an expression for redundant projections.
    #[allow(dead_code)]
    pub fn analyze(&mut self, expr: &LcnfExpr) {
        match expr {
            LcnfExpr::Let {
                id,
                value: LcnfLetValue::Proj(_field_name, ctor_tag, base),
                body,
                ..
            } => {
                if self.lookup_load(*base, *ctor_tag).is_some() {
                    self.redundant_count += 1;
                } else {
                    self.register_load(*base, *ctor_tag, *id);
                }
                self.analyze(body);
            }
            LcnfExpr::Let { body, .. } => self.analyze(body),
            LcnfExpr::Case { alts, default, .. } => {
                for alt in alts {
                    self.analyze(&alt.body);
                }
                if let Some(d) = default {
                    self.analyze(d);
                }
            }
            LcnfExpr::Return(_) | LcnfExpr::Unreachable | LcnfExpr::TailCall(..) => {}
        }
    }
}
/// Configuration for loop versioning.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct LoopVersioningConfig {
    /// Maximum number of loop versions to create.
    pub max_versions: usize,
    /// Minimum estimated speedup (as a ratio) to justify versioning.
    pub min_speedup_ratio: f32,
}
impl LoopVersioningConfig {
    /// Create a conservative configuration.
    #[allow(dead_code)]
    pub fn conservative() -> Self {
        LoopVersioningConfig {
            max_versions: 2,
            min_speedup_ratio: 1.5,
        }
    }
}
/// Statistics produced by a single LICM run.
#[derive(Debug, Clone, Default)]
pub struct LICMReport {
    /// Total number of loops analysed.
    pub loops_analyzed: usize,
    /// Number of expressions hoisted out of loops.
    pub expressions_hoisted: usize,
    /// Total estimated dynamic evaluations saved.
    pub estimated_savings: u64,
}
impl LICMReport {
    /// Merge another report into `self`.
    pub fn merge(&mut self, other: &LICMReport) {
        self.loops_analyzed += other.loops_analyzed;
        self.expressions_hoisted += other.expressions_hoisted;
        self.estimated_savings += other.estimated_savings;
    }
}
/// A candidate expression ready to be hoisted to a preheader.
#[derive(Debug, Clone)]
pub struct HoistCandidate {
    /// The invariant binding to hoist.
    pub expr: LoopInvariantExpr,
    /// The id of the loop this candidate should be hoisted out of.
    pub target_loop_header: LcnfVarId,
    /// Estimated number of redundant evaluations saved (loop trip count
    /// heuristic; we use 10 as a conservative default).
    pub savings_estimate: u32,
}
/// LICM loop nest (for perfectly nested loops)
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LoopNest {
    pub loops: Vec<LoopLevelId>,
    pub depth: u32,
    pub is_perfect: bool,
}
/// LICM emit stats
#[allow(dead_code)]
#[derive(Debug, Default, Clone)]
pub struct LicmExtEmitStats {
    pub bytes_written: usize,
    pub items_emitted: usize,
    pub errors: usize,
    pub warnings: usize,
}
/// A let-binding inside a loop whose value is loop-invariant.
#[derive(Debug, Clone)]
pub struct LoopInvariantExpr {
    /// The variable introduced by this binding.
    pub var: LcnfVarId,
    /// The right-hand side value (invariant computation).
    pub value: LcnfLetValue,
    /// The type of the variable.
    pub ty: LcnfType,
    /// Nesting depth of the loop this expression was found in.
    pub loop_depth: u32,
}
/// LICM result map
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct LicmResultMap {
    pub hoisted: std::collections::HashMap<u32, LoopLevelId>,
    pub sunk: std::collections::HashMap<u32, u32>,
    pub versioned: Vec<LicmVersion>,
}
#[allow(dead_code)]
impl LicmResultMap {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn mark_hoisted(&mut self, inst: u32, loop_id: LoopLevelId) {
        self.hoisted.insert(inst, loop_id);
    }
    pub fn mark_sunk(&mut self, inst: u32, block: u32) {
        self.sunk.insert(inst, block);
    }
    pub fn add_version(&mut self, v: LicmVersion) {
        self.versioned.push(v);
    }
    pub fn hoist_count(&self) -> usize {
        self.hoisted.len()
    }
    pub fn sink_count(&self) -> usize {
        self.sunk.len()
    }
}
/// LICM preheader map
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct LicmPreheaderMap {
    pub map: std::collections::HashMap<LoopLevelId, LoopPreheader>,
}
#[allow(dead_code)]
impl LicmPreheaderMap {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn insert(&mut self, ph: LoopPreheader) {
        self.map.insert(ph.loop_id.clone(), ph);
    }
    pub fn get(&self, id: &LoopLevelId) -> Option<&LoopPreheader> {
        self.map.get(id)
    }
    pub fn count(&self) -> usize {
        self.map.len()
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct LICMPassStats {
    pub total_runs: u32,
    pub successful_runs: u32,
    pub total_changes: u64,
    pub time_ms: u64,
    pub iterations_used: u32,
}
impl LICMPassStats {
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
/// LICM source buffer
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct LicmExtSourceBuffer {
    pub content: String,
}
#[allow(dead_code)]
impl LicmExtSourceBuffer {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn write(&mut self, s: &str) {
        self.content.push_str(s);
    }
    pub fn writeln(&mut self, s: &str) {
        self.content.push_str(s);
        self.content.push('\n');
    }
    pub fn finish(self) -> String {
        self.content
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LICMAnalysisCache {
    pub(super) entries: std::collections::HashMap<String, LICMCacheEntry>,
    pub(super) max_size: usize,
    pub(super) hits: u64,
    pub(super) misses: u64,
}
impl LICMAnalysisCache {
    #[allow(dead_code)]
    pub fn new(max_size: usize) -> Self {
        LICMAnalysisCache {
            entries: std::collections::HashMap::new(),
            max_size,
            hits: 0,
            misses: 0,
        }
    }
    #[allow(dead_code)]
    pub fn get(&mut self, key: &str) -> Option<&LICMCacheEntry> {
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
            LICMCacheEntry {
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
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LICMWorklist {
    pub(super) items: std::collections::VecDeque<u32>,
    pub(super) in_worklist: std::collections::HashSet<u32>,
}
impl LICMWorklist {
    #[allow(dead_code)]
    pub fn new() -> Self {
        LICMWorklist {
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
/// LICM feature flags
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct LicmFeatureFlags {
    pub speculative: bool,
    pub sink_enabled: bool,
    pub hoist_loads: bool,
    pub hoist_pure_calls: bool,
}
#[allow(dead_code)]
pub struct LICMConstantFoldingHelper;
impl LICMConstantFoldingHelper {
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
/// LICM versioning (for speculative hoisting)
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LicmVersion {
    pub original: u32,
    pub versioned: u32,
    pub guard_cond: String,
}
#[allow(dead_code)]
pub struct LICMPassRegistry {
    pub(super) configs: Vec<LICMPassConfig>,
    pub(super) stats: std::collections::HashMap<String, LICMPassStats>,
}
impl LICMPassRegistry {
    #[allow(dead_code)]
    pub fn new() -> Self {
        LICMPassRegistry {
            configs: Vec::new(),
            stats: std::collections::HashMap::new(),
        }
    }
    #[allow(dead_code)]
    pub fn register(&mut self, config: LICMPassConfig) {
        self.stats
            .insert(config.pass_name.clone(), LICMPassStats::new());
        self.configs.push(config);
    }
    #[allow(dead_code)]
    pub fn enabled_passes(&self) -> Vec<&LICMPassConfig> {
        self.configs.iter().filter(|c| c.enabled).collect()
    }
    #[allow(dead_code)]
    pub fn get_stats(&self, name: &str) -> Option<&LICMPassStats> {
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
/// LICM loop body analysis result
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct LoopBodyAnalysis {
    pub loop_id: u32,
    pub def_vars: std::collections::HashSet<u32>,
    pub use_vars: std::collections::HashSet<u32>,
    pub load_insts: Vec<u32>,
    pub store_insts: Vec<u32>,
    pub call_insts: Vec<u32>,
    pub has_volatile: bool,
    pub has_exception: bool,
}
#[allow(dead_code)]
impl LoopBodyAnalysis {
    pub fn new(loop_id: u32) -> Self {
        Self {
            loop_id,
            ..Default::default()
        }
    }
    pub fn add_def(&mut self, var: u32) {
        self.def_vars.insert(var);
    }
    pub fn add_use(&mut self, var: u32) {
        self.use_vars.insert(var);
    }
    pub fn is_def(&self, var: u32) -> bool {
        self.def_vars.contains(&var)
    }
    pub fn is_invariant(&self, var: u32) -> bool {
        !self.def_vars.contains(&var)
    }
    pub fn num_memory_ops(&self) -> usize {
        self.load_insts.len() + self.store_insts.len()
    }
}
/// LICM loop info builder
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct LoopInfoBuilder {
    pub loops: Vec<LoopNode>,
}
#[allow(dead_code)]
impl LoopInfoBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn add_loop(&mut self, n: LoopNode) {
        self.loops.push(n);
    }
    pub fn build_tree(self) -> LoopTree {
        let mut tree = LoopTree::new();
        for n in self.loops {
            tree.add(n);
        }
        tree
    }
    pub fn summarize(&self) -> LoopInfoSummary {
        let total = self.loops.len();
        let inner = self.loops.iter().filter(|n| n.is_inner_most).count();
        let countable = self.loops.iter().filter(|n| n.is_countable).count();
        let depths: Vec<u32> = self.loops.iter().map(|n| n.depth).collect();
        let max_depth = depths.iter().copied().max().unwrap_or(0);
        let avg_depth = if total > 0 {
            depths.iter().sum::<u32>() as f64 / total as f64
        } else {
            0.0
        };
        LoopInfoSummary {
            total_loops: total,
            inner_loops: inner,
            countable_loops: countable,
            avg_depth,
            max_depth,
        }
    }
}
/// Identifies a loop in the LCNF control-flow graph.
///
/// In ANF / LCNF, loops manifest as (mutually) recursive function bindings.
/// The "header" is the function id that forms the back-edge target.
#[derive(Debug, Clone)]
pub struct LoopStructure {
    /// The variable id of the loop header (recursive entry point).
    pub header: LcnfVarId,
    /// All variables defined inside the loop body.
    pub body_vars: HashSet<LcnfVarId>,
    /// Variables that are live on loop exit (used outside the loop).
    pub exit_vars: HashSet<LcnfVarId>,
    /// Nesting depth (0 = outermost loop).
    pub nest_depth: u32,
}
/// LICM pass summary
#[allow(dead_code)]
#[derive(Debug, Default, Clone)]
pub struct LicmPassSummary {
    pub pass_name: String,
    pub functions_processed: usize,
    pub hoisted: usize,
    pub sunk: usize,
    pub duration_us: u64,
}
/// LICM sink candidate (for loop exit sinking)
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LicmSinkCandidate {
    pub inst_id: u32,
    pub loop_id: LoopLevelId,
    pub sink_to_block: u32,
    pub benefit: i32,
}
/// Loop tree
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct LoopTree {
    pub loops: std::collections::HashMap<LoopLevelId, LoopNode>,
    pub top_level: Vec<LoopLevelId>,
}
#[allow(dead_code)]
impl LoopTree {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn add(&mut self, node: LoopNode) {
        if node.parent.is_none() {
            self.top_level.push(node.id.clone());
        }
        self.loops.insert(node.id.clone(), node);
    }
    pub fn get(&self, id: &LoopLevelId) -> Option<&LoopNode> {
        self.loops.get(id)
    }
    pub fn num_loops(&self) -> usize {
        self.loops.len()
    }
    pub fn inner_loops(&self) -> Vec<&LoopNode> {
        self.loops.values().filter(|n| n.is_inner_most).collect()
    }
}
/// LICM diagnostic
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LicmDiagLevel {
    Info,
    Warning,
    Error,
}
/// A named phase in a LICM-centric optimization pipeline.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LICMPhase {
    /// Run LICM before CSE.
    LICMBeforeCSE,
    /// Run LICM after CSE.
    LICMAfterCSE,
    /// Run LICM in a loop until no more changes.
    LICMIterative,
    /// Run LICM once and stop.
    LICMOnce,
}
/// Loop nest information aggregated from a set of loop structures.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LoopNestInfo {
    /// Maximum nesting depth among all loops.
    pub max_depth: u32,
    /// Total number of body variables across all loops.
    pub total_body_vars: usize,
    /// The loop structures.
    pub loops: Vec<LoopStructure>,
}
impl LoopNestInfo {
    /// Build loop nest info from a vector of loop structures.
    #[allow(dead_code)]
    pub fn from_loops(loops: Vec<LoopStructure>) -> Self {
        let max_depth = loops.iter().map(|l| l.nest_depth).max().unwrap_or(0);
        let total_body_vars: usize = loops.iter().map(|l| l.body_vars.len()).sum();
        LoopNestInfo {
            max_depth,
            total_body_vars,
            loops,
        }
    }
}
/// LICM heuristics configuration for LICMPassV2.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LICMHeuristics {
    /// Maximum cost of an expression to consider for hoisting.
    pub max_hoist_cost: u32,
    /// Minimum savings estimate to justify hoisting.
    pub min_savings: u32,
}
impl Default for LICMHeuristics {
    fn default() -> Self {
        LICMHeuristics {
            max_hoist_cost: 100,
            min_savings: 0,
        }
    }
}
/// Version 2 of the LICM pass with heuristic control.
#[allow(dead_code)]
pub struct LICMPassV2 {
    /// Heuristics for deciding what to hoist.
    pub heuristics: LICMHeuristics,
    /// The inner LICM pass.
    inner: LICMPass,
}
#[allow(dead_code)]
impl LICMPassV2 {
    /// Create a new LICM v2 pass with default heuristics.
    pub fn new() -> Self {
        LICMPassV2 {
            heuristics: LICMHeuristics::default(),
            inner: LICMPass::new(LICMConfig {
                min_savings_threshold: 0,
                hoist_function_calls: false,
            }),
        }
    }
    /// Run the LICM v2 pass over function declarations.
    pub fn run(&mut self, decls: &mut [LcnfFunDecl]) {
        self.inner.config.min_savings_threshold = self.heuristics.min_savings;
        self.inner.run(decls);
    }
    /// Get the report from the inner pass.
    pub fn report(&self) -> &LICMReport {
        &self.inner.report
    }
}
/// Materialize a preheader block by wrapping an inner expression with
/// the let bindings from the preheader.
#[allow(dead_code)]
pub fn materialize_preheader(pb: &PreheaderBlock, inner: LcnfExpr) -> LcnfExpr {
    let mut result = inner;
    for inv in pb.preheader_lets.iter().rev() {
        result = LcnfExpr::Let {
            id: inv.var,
            name: format!("_pre_{}", inv.var.0),
            ty: inv.ty.clone(),
            value: inv.value.clone(),
            body: Box::new(result),
        };
    }
    result
}
/// Topologically sort hoist candidates so that producers come before
/// consumers (i.e., if candidate B uses the variable defined by A,
/// A must appear first).
#[allow(dead_code)]
pub fn topo_sort_candidates(candidates: &[HoistCandidate]) -> Vec<HoistCandidate> {
    let defined: HashSet<LcnfVarId> = candidates.iter().map(|c| c.expr.var).collect();
    // Build dependency map: deps[v] = variables that v depends on
    let mut deps: HashMap<LcnfVarId, Vec<LcnfVarId>> = HashMap::new();
    for c in candidates {
        let mut c_deps = Vec::new();
        match &c.expr.value {
            LcnfLetValue::FVar(v) if defined.contains(v) => {
                c_deps.push(*v);
            }
            LcnfLetValue::App(f, args) => {
                if let LcnfArg::Var(v) = f {
                    if defined.contains(v) {
                        c_deps.push(*v);
                    }
                }
                for a in args {
                    if let LcnfArg::Var(v) = a {
                        if defined.contains(v) {
                            c_deps.push(*v);
                        }
                    }
                }
            }
            LcnfLetValue::Proj(_, _, v) if defined.contains(v) => {
                c_deps.push(*v);
            }
            _ => {}
        }
        deps.insert(c.expr.var, c_deps);
    }
    // Build adjacency list and in-degree for Kahn's algorithm
    // Edge dep -> v means dep must come before v
    let mut adj: HashMap<LcnfVarId, Vec<LcnfVarId>> = HashMap::new();
    let mut in_degree: HashMap<LcnfVarId, usize> =
        candidates.iter().map(|c| (c.expr.var, 0)).collect();
    for (v, d) in &deps {
        for dep in d {
            adj.entry(*dep).or_default().push(*v);
            *in_degree.entry(*v).or_default() += 1;
        }
    }
    let mut queue: VecDeque<LcnfVarId> = in_degree
        .iter()
        .filter(|(_, &d)| d == 0)
        .map(|(v, _)| *v)
        .collect();
    let by_var: HashMap<LcnfVarId, &HoistCandidate> =
        candidates.iter().map(|c| (c.expr.var, c)).collect();
    let mut result = Vec::new();
    while let Some(v) = queue.pop_front() {
        if let Some(c) = by_var.get(&v) {
            result.push((*c).clone());
        }
        if let Some(neighbors) = adj.get(&v) {
            for n in neighbors {
                if let Some(d) = in_degree.get_mut(n) {
                    *d -= 1;
                    if *d == 0 {
                        queue.push_back(*n);
                    }
                }
            }
        }
    }
    result
}
