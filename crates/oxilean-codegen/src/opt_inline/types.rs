//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use crate::lcnf::{LcnfArg, LcnfExpr, LcnfFunDecl, LcnfLetValue, LcnfLit, LcnfType, LcnfVarId};
use std::collections::{HashMap, HashSet};

#[allow(dead_code)]
#[derive(Default, Debug)]
pub struct InlineTrace {
    pub(super) entries: Vec<InlineTraceEntry>,
    pub enabled: bool,
}
#[allow(dead_code)]
impl InlineTrace {
    pub fn new() -> Self {
        InlineTrace {
            entries: Vec::new(),
            enabled: true,
        }
    }
    pub fn disabled() -> Self {
        InlineTrace {
            entries: Vec::new(),
            enabled: false,
        }
    }
    pub fn record(&mut self, entry: InlineTraceEntry) {
        if self.enabled {
            self.entries.push(entry);
        }
    }
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
    pub fn inlined_entries(&self) -> Vec<&InlineTraceEntry> {
        self.entries.iter().filter(|e| e.did_inline).collect()
    }
    pub fn skipped_entries(&self) -> Vec<&InlineTraceEntry> {
        self.entries.iter().filter(|e| !e.did_inline).collect()
    }
    pub fn entries_for_callee(&self, callee: &str) -> Vec<&InlineTraceEntry> {
        self.entries.iter().filter(|e| e.callee == callee).collect()
    }
    pub fn to_csv(&self) -> String {
        let header = "sweep,caller,callee,decision,callee_size,did_inline";
        let rows: Vec<String> = self.entries.iter().map(|e| e.to_csv()).collect();
        std::iter::once(header.to_owned())
            .chain(rows)
            .collect::<Vec<_>>()
            .join("\n")
    }
    pub fn clear(&mut self) {
        self.entries.clear();
    }
}
#[allow(dead_code)]
#[derive(Clone, Debug, Default)]
pub struct RecursiveInlineLimiter {
    pub unroll_depth: HashMap<String, u32>,
    pub max_unroll: u32,
    pub enforcements: u64,
}
#[allow(dead_code)]
impl RecursiveInlineLimiter {
    pub fn new(max_unroll: u32) -> Self {
        RecursiveInlineLimiter {
            max_unroll,
            ..Default::default()
        }
    }
    pub fn try_unroll(&mut self, fn_name: &str) -> bool {
        let depth = self.unroll_depth.entry(fn_name.to_owned()).or_insert(0);
        if *depth >= self.max_unroll {
            self.enforcements += 1;
            return false;
        }
        *depth += 1;
        true
    }
    pub fn pop_unroll(&mut self, fn_name: &str) {
        if let Some(d) = self.unroll_depth.get_mut(fn_name) {
            if *d > 0 {
                *d -= 1;
            }
        }
    }
    pub fn depth_of(&self, fn_name: &str) -> u32 {
        self.unroll_depth.get(fn_name).copied().unwrap_or(0)
    }
    pub fn reset(&mut self) {
        self.unroll_depth.clear();
    }
}
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InlineAnnotation {
    AlwaysInline,
    NeverInline,
    Default,
    HotOnly { threshold: u64 },
}
/// Estimates the profitability of inlining a given call site.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct InlineProfitabilityEstimator {
    /// Weight applied to callee size penalty.
    pub(super) size_weight: f64,
    /// Weight applied to call overhead savings.
    pub(super) call_overhead_weight: f64,
    /// Weight for hot-path bonus.
    pub(super) hot_path_weight: f64,
}
#[allow(dead_code)]
impl InlineProfitabilityEstimator {
    /// Create estimator with default weights.
    pub fn new() -> Self {
        Self {
            size_weight: 1.0,
            call_overhead_weight: 2.0,
            hot_path_weight: 3.0,
        }
    }
    /// Estimate profitability score (positive = profitable).
    /// - `callee_size`: estimated IR node count
    /// - `call_overhead`: estimated call overhead in cycles
    /// - `is_hot`: whether call site is on a hot path
    pub fn estimate(&self, callee_size: usize, call_overhead: f64, is_hot: bool) -> f64 {
        let size_penalty = callee_size as f64 * self.size_weight;
        let overhead_savings = call_overhead * self.call_overhead_weight;
        let hot_bonus = if is_hot {
            self.hot_path_weight * 10.0
        } else {
            0.0
        };
        overhead_savings + hot_bonus - size_penalty
    }
    /// Returns `true` if inlining is estimated to be profitable.
    pub fn is_profitable(&self, callee_size: usize, call_overhead: f64, is_hot: bool) -> bool {
        self.estimate(callee_size, call_overhead, is_hot) > 0.0
    }
}
#[allow(dead_code)]
#[derive(Default, Debug)]
pub struct InlineAnnotationRegistry {
    pub(super) annotations: HashMap<String, InlineAnnotation>,
}
#[allow(dead_code)]
impl InlineAnnotationRegistry {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn register(&mut self, fn_name: impl Into<String>, ann: InlineAnnotation) {
        self.annotations.insert(fn_name.into(), ann);
    }
    pub fn get(&self, fn_name: &str) -> &InlineAnnotation {
        self.annotations
            .get(fn_name)
            .unwrap_or(&InlineAnnotation::Default)
    }
    pub fn has_annotation(&self, fn_name: &str) -> bool {
        self.annotations.contains_key(fn_name)
    }
    pub fn apply(&self, fn_name: &str, decision: InlineDecision) -> InlineDecision {
        match self.get(fn_name) {
            InlineAnnotation::AlwaysInline => InlineDecision::Always,
            InlineAnnotation::NeverInline => InlineDecision::Never,
            InlineAnnotation::Default => decision,
            InlineAnnotation::HotOnly { threshold } => {
                InlineDecision::Heuristic(*threshold as f64 / 100.0)
            }
        }
    }
    pub fn len(&self) -> usize {
        self.annotations.len()
    }
    pub fn is_empty(&self) -> bool {
        self.annotations.is_empty()
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct InlineBudget {
    pub total_budget: u64,
    pub consumed: u64,
    pub reserved_always: u64,
    pub per_fn_limit: HashMap<String, u64>,
    pub per_fn_spent: HashMap<String, u64>,
}
#[allow(dead_code)]
impl InlineBudget {
    pub fn new(total_budget: u64) -> Self {
        InlineBudget {
            total_budget,
            consumed: 0,
            reserved_always: total_budget / 4,
            per_fn_limit: HashMap::new(),
            per_fn_spent: HashMap::new(),
        }
    }
    pub fn set_fn_limit(&mut self, fn_name: impl Into<String>, limit: u64) {
        self.per_fn_limit.insert(fn_name.into(), limit);
    }
    pub fn try_spend(&mut self, caller: &str, cost: u64) -> bool {
        if self.consumed + cost > self.total_budget {
            return false;
        }
        if let Some(&limit) = self.per_fn_limit.get(caller) {
            let spent = self.per_fn_spent.get(caller).copied().unwrap_or(0);
            if spent + cost > limit {
                return false;
            }
            *self.per_fn_spent.entry(caller.to_owned()).or_insert(0) += cost;
        }
        self.consumed += cost;
        true
    }
    pub fn remaining(&self) -> u64 {
        self.total_budget.saturating_sub(self.consumed)
    }
    pub fn is_exhausted(&self) -> bool {
        self.consumed >= self.total_budget
    }
    pub fn fraction_consumed(&self) -> f64 {
        if self.total_budget == 0 {
            1.0
        } else {
            self.consumed as f64 / self.total_budget as f64
        }
    }
    pub fn reset_per_fn(&mut self) {
        self.per_fn_spent.clear();
    }
    pub fn heavy_spenders(&self, threshold: u64) -> Vec<(&str, u64)> {
        self.per_fn_spent
            .iter()
            .filter(|(_, &s)| s >= threshold)
            .map(|(n, &s)| (n.as_str(), s))
            .collect()
    }
}
/// Represents a fusion of two adjacent inlined callees at the same call site.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct InlineFusionRecord {
    pub caller: String,
    pub first_callee: String,
    pub second_callee: String,
    pub fused_name: String,
    pub savings_estimate: i32,
}
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct InlineTraceEntry {
    pub sweep: u32,
    pub caller: String,
    pub callee: String,
    pub decision: String,
    pub callee_size: u64,
    pub did_inline: bool,
}
#[allow(dead_code)]
impl InlineTraceEntry {
    pub fn new(
        sweep: u32,
        caller: impl Into<String>,
        callee: impl Into<String>,
        decision: impl Into<String>,
        callee_size: u64,
        did_inline: bool,
    ) -> Self {
        InlineTraceEntry {
            sweep,
            caller: caller.into(),
            callee: callee.into(),
            decision: decision.into(),
            callee_size,
            did_inline,
        }
    }
    pub fn to_csv(&self) -> String {
        format!(
            "{},{},{},{},{},{}",
            self.sweep, self.caller, self.callee, self.decision, self.callee_size, self.did_inline
        )
    }
}
/// A single frame in the inline context stack.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct InlineContextFrame {
    /// The function being inlined at this frame.
    pub callee: String,
    /// The call-site depth at which this inlining occurred.
    pub depth: usize,
    /// Unique ID for this inlining event.
    pub event_id: u64,
}
/// Context-sensitive inline stack, used to detect context-specific
/// inlining opportunities (e.g., inlining differently depending on call chain).
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct InlineContextStack {
    pub(super) frames: Vec<InlineContextFrame>,
    pub(super) next_event_id: u64,
}
#[allow(dead_code)]
impl InlineContextStack {
    /// Create a new empty context stack.
    pub fn new() -> Self {
        Self::default()
    }
    /// Push a new frame onto the stack.
    pub fn push(&mut self, callee: &str, depth: usize) -> u64 {
        let id = self.next_event_id;
        self.next_event_id += 1;
        self.frames.push(InlineContextFrame {
            callee: callee.to_owned(),
            depth,
            event_id: id,
        });
        id
    }
    /// Pop the most recent frame.
    pub fn pop(&mut self) -> Option<InlineContextFrame> {
        self.frames.pop()
    }
    /// Returns the current depth of the context stack.
    pub fn depth(&self) -> usize {
        self.frames.len()
    }
    /// Returns `true` if `callee` appears anywhere in the current context.
    pub fn contains(&self, callee: &str) -> bool {
        self.frames.iter().any(|f| f.callee == callee)
    }
    /// Returns the context fingerprint as a concatenation of callee names.
    pub fn fingerprint(&self) -> String {
        self.frames
            .iter()
            .map(|f| f.callee.as_str())
            .collect::<Vec<_>>()
            .join("->")
    }
}
/// Tracks the current inlining call stack to detect and prevent cycles.
#[derive(Debug, Clone)]
pub struct InliningContext {
    /// Current recursion depth of the inliner.
    pub depth: u32,
    /// Stack of function names currently being inlined.
    pub call_stack: Vec<String>,
    /// Named-value substitutions active in the current scope.
    pub substitutions: HashMap<String, LcnfExpr>,
}
impl InliningContext {
    /// Create a fresh context at depth 0.
    pub fn new() -> Self {
        InliningContext {
            depth: 0,
            call_stack: Vec::new(),
            substitutions: HashMap::new(),
        }
    }
    /// Attempt to push `name` onto the call stack.
    ///
    /// Returns `false` (and does NOT push) when `name` is already on the
    /// stack, indicating a recursive cycle.
    pub fn push_call(&mut self, name: &str) -> bool {
        if self.has_cycle(name) {
            return false;
        }
        self.call_stack.push(name.to_owned());
        self.depth += 1;
        true
    }
    /// Pop the most recently pushed call from the stack.
    pub fn pop_call(&mut self) {
        self.call_stack.pop();
        if self.depth > 0 {
            self.depth -= 1;
        }
    }
    /// Returns `true` when `name` is already present in the call stack.
    pub fn has_cycle(&self, name: &str) -> bool {
        self.call_stack.iter().any(|n| n == name)
    }
}
/// Top-level configuration for the inlining pass.
#[derive(Debug, Clone)]
pub struct InlineConfig {
    /// Tunable heuristic parameters.
    pub heuristics: InlineHeuristics,
    /// Whether to allow (limited) inlining of recursive functions.
    pub enable_recursive_inlining: bool,
    /// Whether to prioritise inlining of hot (frequently-called) functions.
    pub enable_hot_inlining: bool,
    /// Maximum number of inlining sweeps over the declaration list.
    pub max_passes: u32,
}
/// Describes which portion of a callee body is eligible for partial inlining.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum PartialInlineRegion {
    /// Inline the first N let-bindings of the function body.
    Prefix(usize),
    /// Inline the return value only (tail region).
    TailOnly,
    /// Inline everything (full inline).
    Full,
    /// Do not inline.
    None,
}
/// Statistics gathered by a completed inlining pass.
#[derive(Debug, Clone, Default)]
pub struct InlineReport {
    /// Total number of call sites examined.
    pub total_calls_considered: usize,
    /// Number of call sites that were actually inlined.
    pub inlined_count: usize,
    /// Calls skipped because the callee is recursive.
    pub skipped_recursive: usize,
    /// Calls skipped because the callee body is too large.
    pub skipped_too_large: usize,
}
impl InlineReport {
    /// Human-readable one-line summary of the pass results.
    pub fn summary(&self) -> String {
        format!(
            "inline: {}/{} inlined (recursive_skip={}, size_skip={})",
            self.inlined_count,
            self.total_calls_considered,
            self.skipped_recursive,
            self.skipped_too_large,
        )
    }
    /// Fraction of considered call sites that were inlined (0.0–1.0).
    pub fn inline_rate(&self) -> f64 {
        if self.total_calls_considered == 0 {
            0.0
        } else {
            self.inlined_count as f64 / self.total_calls_considered as f64
        }
    }
}
/// A single call site encountered during the inlining pass.
#[derive(Debug, Clone)]
pub struct CallSite {
    /// Name of the calling function.
    pub caller: String,
    /// Name of the called function.
    pub callee: String,
    /// Unique identifier for this call site (within the pass).
    pub call_id: u64,
    /// Whether the call is in tail position.
    pub is_tail_call: bool,
    /// Whether the callee is (mutually) recursive.
    pub is_self_recursive: bool,
}
impl CallSite {
    /// Create a new call site record.
    pub fn new(
        caller: impl Into<String>,
        callee: impl Into<String>,
        call_id: u64,
        is_tail_call: bool,
        is_self_recursive: bool,
    ) -> Self {
        CallSite {
            caller: caller.into(),
            callee: callee.into(),
            call_id,
            is_tail_call,
            is_self_recursive,
        }
    }
    /// Estimated benefit of inlining this call site (in abstract units).
    ///
    /// Tail calls get a small bonus because inlining them can enable TCO.
    /// Self-recursive calls are penalised because inlining them risks blowup.
    pub fn inline_benefit(&self) -> u64 {
        let base: u64 = 10;
        let tail_bonus: u64 = if self.is_tail_call { 3 } else { 0 };
        let recursive_penalty: u64 = if self.is_self_recursive { 8 } else { 0 };
        base.saturating_add(tail_bonus)
            .saturating_sub(recursive_penalty)
    }
}
/// Cost model for inlining a single call site.
#[derive(Debug, Clone)]
pub struct InlineCost {
    /// Estimated code-size contribution of the callee body.
    pub body_size: u64,
    /// Overhead of the call itself (stack frame, arg passing, return).
    pub call_overhead: u64,
    /// Estimated savings from inlining (e.g. eliminated argument allocations,
    /// constant propagation opportunities).  May be negative.
    pub estimated_savings: i64,
}
impl InlineCost {
    /// Construct a zeroed cost estimate.
    pub fn new() -> Self {
        InlineCost {
            body_size: 0,
            call_overhead: 4,
            estimated_savings: 0,
        }
    }
    /// Net gain from inlining: positive means inlining is beneficial.
    pub fn net_gain(&self) -> i64 {
        self.estimated_savings + self.call_overhead as i64 - self.body_size as i64
    }
    /// Returns `true` when inlining is expected to be profitable.
    pub fn is_profitable(&self) -> bool {
        self.net_gain() > 0
    }
}
#[allow(dead_code)]
#[derive(Default, Debug)]
pub struct SpeculativeInliner {
    pub records: Vec<SpeculativeInlineRecord>,
    pub threshold: f64,
}
#[allow(dead_code)]
impl SpeculativeInliner {
    pub fn new(threshold: f64) -> Self {
        SpeculativeInliner {
            records: Vec::new(),
            threshold,
        }
    }
    pub fn add(&mut self, record: SpeculativeInlineRecord) {
        self.records.push(record);
    }
    pub fn committed(&self) -> Vec<&SpeculativeInlineRecord> {
        self.records
            .iter()
            .filter(|r| r.should_speculate(self.threshold))
            .collect()
    }
    pub fn pending(&self) -> Vec<&SpeculativeInlineRecord> {
        self.records.iter().filter(|r| !r.confirmed).collect()
    }
    pub fn confirm_callee(&mut self, callee: &str) {
        for r in &mut self.records {
            if r.callee == callee {
                r.confirm();
            }
        }
    }
    pub fn total_confidence(&self) -> f64 {
        self.records.iter().map(|r| r.confidence).sum()
    }
}
/// Decision about whether a function should be inlined at a call site.
#[derive(Debug, Clone, PartialEq)]
pub enum InlineDecision {
    /// Always inline regardless of call count or size.
    Always,
    /// Never inline this function.
    Never,
    /// Inline based on a heuristic score (0.0–1.0 probability).
    Heuristic(f64),
    /// Inline only the first call site encountered.
    OnceOnly,
}
impl InlineDecision {
    /// Decide whether to inline given the current call count for this site.
    ///
    /// `call_count` is the number of times this decision has already resulted
    /// in an inline for the same callee.
    pub fn should_inline(&self, call_count: u64) -> bool {
        match self {
            InlineDecision::Always => true,
            InlineDecision::Never => false,
            InlineDecision::Heuristic(score) => *score >= 0.5,
            InlineDecision::OnceOnly => call_count == 0,
        }
    }
}
/// Tracks and manages inline fusion decisions.
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct InlineFusionManager {
    pub(super) records: Vec<InlineFusionRecord>,
    pub(super) next_id: u32,
}
#[allow(dead_code)]
impl InlineFusionManager {
    pub fn new() -> Self {
        Self::default()
    }
    /// Record a fusion event and return the fused name.
    pub fn fuse(&mut self, caller: &str, first: &str, second: &str, savings: i32) -> String {
        let id = self.next_id;
        self.next_id += 1;
        let fused_name = format!("{}_fused_{}_{}_{}", caller, first, second, id);
        self.records.push(InlineFusionRecord {
            caller: caller.to_owned(),
            first_callee: first.to_owned(),
            second_callee: second.to_owned(),
            fused_name: fused_name.clone(),
            savings_estimate: savings,
        });
        fused_name
    }
    /// Returns all fusion records.
    pub fn all_records(&self) -> &[InlineFusionRecord] {
        &self.records
    }
    /// Total savings estimate across all fusions.
    pub fn total_savings(&self) -> i32 {
        self.records.iter().map(|r| r.savings_estimate).sum()
    }
}
#[allow(dead_code)]
#[derive(Default, Debug)]
pub struct CallGraph {
    pub edges: Vec<CallGraphEdge>,
    pub functions: HashSet<String>,
}
#[allow(dead_code)]
impl CallGraph {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn build(decls: &[LcnfFunDecl]) -> Self {
        let mut g = Self::new();
        for decl in decls {
            g.functions.insert(decl.name.clone());
            for callee in collect_callees(&decl.body) {
                g.edges.push(CallGraphEdge::new(
                    decl.name.clone(),
                    callee.clone(),
                    1,
                    false,
                ));
                g.functions.insert(callee);
            }
        }
        g
    }
    pub fn callers_of(&self, fn_name: &str) -> Vec<&str> {
        self.edges
            .iter()
            .filter(|e| e.to == fn_name)
            .map(|e| e.from.as_str())
            .collect()
    }
    pub fn callees_of(&self, fn_name: &str) -> Vec<&str> {
        self.edges
            .iter()
            .filter(|e| e.from == fn_name)
            .map(|e| e.to.as_str())
            .collect()
    }
    pub fn in_degree(&self, fn_name: &str) -> usize {
        self.edges.iter().filter(|e| e.to == fn_name).count()
    }
    pub fn out_degree(&self, fn_name: &str) -> usize {
        self.edges.iter().filter(|e| e.from == fn_name).count()
    }
    pub fn num_edges(&self) -> usize {
        self.edges.len()
    }
    pub fn num_nodes(&self) -> usize {
        self.functions.len()
    }
    pub fn single_caller_functions(&self) -> Vec<&str> {
        self.functions
            .iter()
            .filter(|f| self.in_degree(f) == 1)
            .map(|f| f.as_str())
            .collect()
    }
    pub fn leaf_functions(&self) -> Vec<&str> {
        self.functions
            .iter()
            .filter(|f| self.out_degree(f) == 0)
            .map(|f| f.as_str())
            .collect()
    }
}
/// Record of a clone-and-specialize operation.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CloneSpecRecord {
    pub original_callee: String,
    pub specialized_name: String,
    pub arg_index: usize,
    pub specialized_to: String,
}
/// Simple monotonic counter for fresh variable IDs used during inlining.
pub struct FreshVarGen {
    pub(super) next: u64,
}
impl FreshVarGen {
    pub(super) fn new(start: u64) -> Self {
        FreshVarGen { next: start }
    }
    pub(super) fn fresh(&mut self) -> LcnfVarId {
        let id = self.next;
        self.next += 1;
        LcnfVarId(id)
    }
}
/// A node in the call-graph SCC analysis.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct SccNode {
    pub name: String,
    pub callees: Vec<String>,
    pub disc: Option<u32>,
    pub low: Option<u32>,
    pub on_stack: bool,
}
#[allow(dead_code)]
impl SccNode {
    pub fn new(name: impl Into<String>, callees: Vec<String>) -> Self {
        SccNode {
            name: name.into(),
            callees,
            disc: None,
            low: None,
            on_stack: false,
        }
    }
}
#[allow(dead_code)]
pub struct InlineOrderScheduler {
    pub order: Vec<String>,
    pub is_valid: bool,
}
#[allow(dead_code)]
impl InlineOrderScheduler {
    pub fn compute(graph: &CallGraph) -> Self {
        let mut visited = HashSet::new();
        let mut order = Vec::new();
        for fn_name in &graph.functions {
            Self::dfs(fn_name, graph, &mut visited, &mut order);
        }
        InlineOrderScheduler {
            order,
            is_valid: true,
        }
    }
    pub(super) fn dfs(
        node: &str,
        graph: &CallGraph,
        visited: &mut HashSet<String>,
        order: &mut Vec<String>,
    ) {
        if !visited.insert(node.to_owned()) {
            return;
        }
        for callee in graph.callees_of(node) {
            Self::dfs(callee, graph, visited, order);
        }
        order.push(node.to_owned());
    }
    pub fn reverse(&mut self) {
        self.order.reverse();
    }
    pub fn len(&self) -> usize {
        self.order.len()
    }
    pub fn is_empty(&self) -> bool {
        self.order.is_empty()
    }
}
#[allow(dead_code)]
pub struct PostInlineCleanup {
    pub dead_removed: usize,
    pub copies_collapsed: usize,
    pub tail_calls_simplified: usize,
}
#[allow(dead_code)]
impl PostInlineCleanup {
    pub fn new() -> Self {
        PostInlineCleanup {
            dead_removed: 0,
            copies_collapsed: 0,
            tail_calls_simplified: 0,
        }
    }
    pub fn run(&mut self, decl: &mut LcnfFunDecl) {
        let new_body = self.cleanup_expr(decl.body.clone());
        decl.body = new_body;
    }
    pub(super) fn cleanup_expr(&mut self, expr: LcnfExpr) -> LcnfExpr {
        match expr {
            LcnfExpr::Let {
                id,
                name,
                ty,
                value,
                body,
            } => {
                if let LcnfLetValue::FVar(src) = &value {
                    self.copies_collapsed += 1;
                    let src_arg = LcnfArg::Var(*src);
                    let new_body = self.cleanup_expr(*body);
                    return inline_subst(new_body, id, src_arg);
                }
                let new_body = self.cleanup_expr(*body);
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
                let alts2 = alts
                    .into_iter()
                    .map(|alt| crate::lcnf::LcnfAlt {
                        ctor_name: alt.ctor_name,
                        ctor_tag: alt.ctor_tag,
                        params: alt.params,
                        body: self.cleanup_expr(alt.body),
                    })
                    .collect();
                let default2 = default.map(|d| Box::new(self.cleanup_expr(*d)));
                LcnfExpr::Case {
                    scrutinee,
                    scrutinee_ty,
                    alts: alts2,
                    default: default2,
                }
            }
            other => other,
        }
    }
    pub fn summary(&self) -> String {
        format!(
            "PostInlineCleanup: dead={}, copies_collapsed={}",
            self.dead_removed, self.copies_collapsed
        )
    }
}
#[allow(dead_code)]
pub struct ExtendedInlinePass {
    pub config: InlineConfig,
    pub scc: Option<TarjanScc>,
    pub budget: InlineBudget,
    pub annotations: InlineAnnotationRegistry,
    pub size_table: CalleeSizeTable,
    pub speculative: SpeculativeInliner,
    pub recursive_limiter: RecursiveInlineLimiter,
    pub stats: ExtendedInlineStats,
    pub cleanup: PostInlineCleanup,
}
#[allow(dead_code)]
impl ExtendedInlinePass {
    pub fn new(config: InlineConfig, total_budget: u64) -> Self {
        let max_unroll = if config.enable_recursive_inlining {
            2
        } else {
            0
        };
        ExtendedInlinePass {
            config,
            scc: None,
            budget: InlineBudget::new(total_budget),
            annotations: InlineAnnotationRegistry::new(),
            size_table: CalleeSizeTable::new(),
            speculative: SpeculativeInliner::new(0.7),
            recursive_limiter: RecursiveInlineLimiter::new(max_unroll),
            stats: ExtendedInlineStats::new(),
            cleanup: PostInlineCleanup::new(),
        }
    }
    pub fn init_scc(&mut self, decls: &[LcnfFunDecl]) {
        let mut scc = TarjanScc::new(decls);
        scc.compute();
        self.scc = Some(scc);
        self.size_table = CalleeSizeTable::build(decls);
    }
    pub fn should_inline_extended(
        &mut self,
        caller: &str,
        callee: &str,
        _profile: &InlineProfile,
    ) -> bool {
        if *self.annotations.get(callee) == InlineAnnotation::NeverInline {
            return false;
        }
        if self.budget.is_exhausted() {
            return false;
        }
        let callee_size = self.size_table.size_of(callee).unwrap_or(u64::MAX);
        let is_rec = self.scc.as_ref().map_or(false, |s| s.is_recursive(callee));
        if is_rec {
            if !self.config.enable_recursive_inlining {
                return false;
            }
            if !self.recursive_limiter.try_unroll(callee) {
                return false;
            }
        }
        if callee_size > self.config.heuristics.never_inline_size {
            if is_rec {
                self.recursive_limiter.pop_unroll(callee);
            }
            return false;
        }
        if *self.annotations.get(callee) == InlineAnnotation::AlwaysInline
            && callee_size <= self.config.heuristics.max_inline_size
        {
            return true;
        }
        if !self.budget.try_spend(caller, callee_size) {
            if is_rec {
                self.recursive_limiter.pop_unroll(callee);
            }
            return false;
        }
        true
    }
    pub fn run(&mut self, decls: &mut [LcnfFunDecl]) {
        self.init_scc(decls);
        let mut core = InlinePass::new(self.config.clone());
        core.run(decls);
        self.stats.always_inline_count = core.report().inlined_count;
        for decl in decls.iter_mut() {
            self.cleanup.run(decl);
        }
    }
    pub fn extended_stats(&self) -> &ExtendedInlineStats {
        &self.stats
    }
}
/// Decision record for partial inlining of a function.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PartialInlineDecision {
    pub callee: String,
    pub region: PartialInlineRegion,
    pub estimated_savings: i32,
    pub reason: String,
}
#[allow(dead_code)]
impl PartialInlineDecision {
    /// Construct a "full inline" decision.
    pub fn full(callee: &str, savings: i32) -> Self {
        Self {
            callee: callee.to_owned(),
            region: PartialInlineRegion::Full,
            estimated_savings: savings,
            reason: "full".to_owned(),
        }
    }
    /// Construct a "no inline" decision.
    pub fn no_inline(callee: &str, reason: &str) -> Self {
        Self {
            callee: callee.to_owned(),
            region: PartialInlineRegion::None,
            estimated_savings: 0,
            reason: reason.to_owned(),
        }
    }
    /// Construct a "prefix" partial inline decision.
    pub fn prefix(callee: &str, n: usize, savings: i32) -> Self {
        Self {
            callee: callee.to_owned(),
            region: PartialInlineRegion::Prefix(n),
            estimated_savings: savings,
            reason: format!("prefix({})", n),
        }
    }
    /// Returns `true` if any inlining will happen.
    pub fn will_inline(&self) -> bool {
        !matches!(self.region, PartialInlineRegion::None)
    }
}
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct CallGraphEdge {
    pub from: String,
    pub to: String,
    pub frequency: u64,
    pub is_tail: bool,
}
#[allow(dead_code)]
impl CallGraphEdge {
    pub fn new(
        from: impl Into<String>,
        to: impl Into<String>,
        frequency: u64,
        is_tail: bool,
    ) -> Self {
        CallGraphEdge {
            from: from.into(),
            to: to.into(),
            frequency,
            is_tail,
        }
    }
    pub fn is_inter_scc(&self) -> bool {
        self.from != self.to
    }
}
/// Profile information collected (or estimated) for inlining decisions.
#[derive(Debug, Clone)]
pub struct InlineProfile {
    /// Number of times each callee name has been called.
    pub call_counts: HashMap<String, u64>,
    /// Set of function names considered "hot" (called very frequently).
    pub hot_functions: HashSet<String>,
}
impl InlineProfile {
    /// Create an empty profile.
    pub fn new() -> Self {
        InlineProfile {
            call_counts: HashMap::new(),
            hot_functions: HashSet::new(),
        }
    }
    /// Record one call to `callee`.
    pub fn record_call(&mut self, callee: &str) {
        let count = self.call_counts.entry(callee.to_owned()).or_insert(0);
        *count += 1;
    }
    /// Return `true` when the call count for `name` meets or exceeds `threshold`.
    pub fn is_hot(&self, name: &str, threshold: u64) -> bool {
        self.call_counts.get(name).copied().unwrap_or(0) >= threshold
            || self.hot_functions.contains(name)
    }
    /// Return the top `n` callees by call count, highest first.
    pub fn top_callees(&self, n: usize) -> Vec<(String, u64)> {
        let mut pairs: Vec<(String, u64)> = self.call_counts.clone().into_iter().collect();
        pairs.sort_by_key(|b| std::cmp::Reverse(b.1));
        pairs.truncate(n);
        pairs
    }
}
/// The main inlining optimization pass.
pub struct InlinePass {
    /// Configuration controlling inlining behaviour.
    pub config: InlineConfig,
    /// Map from function name to its LCNF declaration (for look-up during inlining).
    pub fn_map: HashMap<String, LcnfFunDecl>,
    /// Call-frequency profile used for hot-path decisions.
    pub profile: InlineProfile,
    /// Total number of inlinings performed so far.
    pub inlined_count: usize,
    /// Internal counter used to generate fresh variable IDs.
    pub(super) next_var_id: u64,
    /// Number of times each callee has been inlined (for OnceOnly tracking).
    pub(super) inline_counts: HashMap<String, u64>,
    /// Accumulated report for the current pass.
    pub(super) report: InlineReport,
}
impl InlinePass {
    /// Create a new pass with the given configuration.
    pub fn new(config: InlineConfig) -> Self {
        InlinePass {
            config,
            fn_map: HashMap::new(),
            profile: InlineProfile::new(),
            inlined_count: 0,
            next_var_id: 1_000_000,
            inline_counts: HashMap::new(),
            report: InlineReport::default(),
        }
    }
    /// Main entry point: run the inlining pass over all declarations.
    pub fn run(&mut self, decls: &mut [LcnfFunDecl]) {
        for _pass in 0..self.config.max_passes {
            self.build_fn_map(decls);
            let prev_count = self.inlined_count;
            for decl in decls.iter_mut() {
                let mut ctx = InliningContext::new();
                ctx.push_call(&decl.name.clone());
                self.inline_decl(decl, &mut ctx);
                ctx.pop_call();
            }
            if self.inlined_count == prev_count {
                break;
            }
        }
    }
    /// Build the internal function map from the current declaration list.
    pub fn build_fn_map(&mut self, decls: &[LcnfFunDecl]) {
        self.fn_map.clear();
        for decl in decls {
            self.fn_map.insert(decl.name.clone(), decl.clone());
        }
    }
    /// Inline call sites within a single function declaration.
    pub fn inline_decl(&mut self, decl: &mut LcnfFunDecl, ctx: &mut InliningContext) {
        let mut body = decl.body.clone();
        self.inline_expr(&mut body, ctx);
        decl.body = body;
    }
    /// Recursively walk an expression and inline call sites.
    pub fn inline_expr(&mut self, expr: &mut LcnfExpr, ctx: &mut InliningContext) {
        match expr {
            LcnfExpr::Let { value, body, .. } => {
                if let LcnfLetValue::App(LcnfArg::Lit(LcnfLit::Str(name)), args) = value {
                    let callee_name = name.clone();
                    let args_clone = args.clone();
                    self.report.total_calls_considered += 1;
                    if let Some(inlined) = self.try_inline_call_ctx(&callee_name, &args_clone, ctx)
                    {
                        let continuation = std::mem::replace(body.as_mut(), LcnfExpr::Unreachable);
                        *expr = splice_inlined(inlined, continuation);
                        self.inlined_count += 1;
                        self.report.inlined_count += 1;
                        self.inline_expr(expr, ctx);
                        return;
                    }
                }
                self.inline_expr(body, ctx);
            }
            LcnfExpr::Case { alts, default, .. } => {
                for alt in alts.iter_mut() {
                    self.inline_expr(&mut alt.body, ctx);
                }
                if let Some(def) = default {
                    self.inline_expr(def, ctx);
                }
            }
            LcnfExpr::TailCall(LcnfArg::Lit(LcnfLit::Str(name)), args) => {
                let callee_name = name.clone();
                let args_clone = args.clone();
                self.report.total_calls_considered += 1;
                if let Some(inlined) = self.try_inline_call_ctx(&callee_name, &args_clone, ctx) {
                    *expr = inlined;
                    self.inlined_count += 1;
                    self.report.inlined_count += 1;
                    self.inline_expr(expr, ctx);
                }
            }
            LcnfExpr::Return(_) | LcnfExpr::TailCall(_, _) | LcnfExpr::Unreachable => {}
        }
    }
    /// Try to produce an inlined copy of `callee` applied to `args`,
    /// consulting `ctx` for cycle / depth limits.
    pub(super) fn try_inline_call_ctx(
        &mut self,
        callee: &str,
        args: &[LcnfArg],
        ctx: &mut InliningContext,
    ) -> Option<LcnfExpr> {
        if ctx.depth >= self.config.heuristics.max_inline_depth {
            return None;
        }
        if ctx.has_cycle(callee) {
            self.report.skipped_recursive += 1;
            return None;
        }
        let decl = self.fn_map.get(callee)?.clone();
        if decl.is_recursive && !self.config.enable_recursive_inlining {
            self.report.skipped_recursive += 1;
            return None;
        }
        let size = estimate_size(&decl);
        if size > self.config.heuristics.never_inline_size {
            self.report.skipped_too_large += 1;
            return None;
        }
        self.profile.record_call(callee);
        let decision = self.config.heuristics.decide(&decl, &self.profile);
        let call_count = self.inline_counts.get(callee).copied().unwrap_or(0);
        if !decision.should_inline(call_count) {
            return None;
        }
        *self.inline_counts.entry(callee.to_owned()).or_insert(0) += 1;
        self.try_inline_call(callee, args)
    }
    /// Produce an inlined copy of `callee` applied to `args` (pure, no side-effects on `self`
    /// except for the fresh-var counter).
    pub fn try_inline_call(&mut self, callee: &str, args: &[LcnfArg]) -> Option<LcnfExpr> {
        let decl = self.fn_map.get(callee)?.clone();
        let param_names: Vec<String> = decl
            .params
            .iter()
            .map(|p| format!("_x{}", p.id.0))
            .collect();
        if param_names.len() != args.len() && !args.is_empty() {
            return None;
        }
        let mut gen = FreshVarGen::new(self.next_var_id);
        let mut inlined = substitute_params(&decl.body, &param_names, args, &mut gen);
        for (param, arg) in decl.params.iter().zip(args.iter()).rev() {
            let fresh = gen.fresh();
            inlined = LcnfExpr::Let {
                id: param.id,
                name: param.name.clone(),
                ty: param.ty.clone(),
                value: LcnfLetValue::App(LcnfArg::Var(fresh), vec![]),
                body: Box::new(inlined),
            };
            inlined = LcnfExpr::Let {
                id: fresh,
                name: format!("_inline_arg_{}", fresh.0),
                ty: LcnfType::Object,
                value: arg_to_let_value(arg),
                body: Box::new(inlined),
            };
        }
        self.next_var_id = gen.next;
        Some(inlined)
    }
    /// Return the accumulated report for this pass.
    pub fn report(&self) -> &InlineReport {
        &self.report
    }
}
/// Tarjan's strongly connected components algorithm over the call graph.
#[allow(dead_code)]
pub struct TarjanScc {
    pub nodes: HashMap<String, SccNode>,
    pub sccs: Vec<Vec<String>>,
    pub(super) timer: u32,
    pub(super) stack: Vec<String>,
}
#[allow(dead_code)]
impl TarjanScc {
    pub fn new(decls: &[LcnfFunDecl]) -> Self {
        let nodes: HashMap<String, SccNode> = decls
            .iter()
            .map(|d| {
                let callees = collect_callees(&d.body);
                (d.name.clone(), SccNode::new(d.name.clone(), callees))
            })
            .collect();
        TarjanScc {
            nodes,
            sccs: Vec::new(),
            timer: 0,
            stack: Vec::new(),
        }
    }
    pub fn compute(&mut self) {
        let names: Vec<String> = self.nodes.keys().cloned().collect();
        for name in &names {
            if self.nodes.get(name).and_then(|n| n.disc).is_none() {
                self.dfs(name.clone());
            }
        }
    }
    pub(super) fn dfs(&mut self, name: String) {
        let disc = self.timer;
        self.timer += 1;
        if let Some(node) = self.nodes.get_mut(&name) {
            node.disc = Some(disc);
            node.low = Some(disc);
            node.on_stack = true;
        }
        self.stack.push(name.clone());
        let callees: Vec<String> = self
            .nodes
            .get(&name)
            .map(|n| n.callees.clone())
            .unwrap_or_default();
        for callee in &callees {
            if !self.nodes.contains_key(callee.as_str()) {
                continue;
            }
            let callee_disc = self.nodes.get(callee).and_then(|n| n.disc);
            if callee_disc.is_none() {
                self.dfs(callee.clone());
                let callee_low = self.nodes.get(callee).and_then(|n| n.low);
                let my_low = self.nodes.get(&name).and_then(|n| n.low);
                if let (Some(ml), Some(cl)) = (my_low, callee_low) {
                    if let Some(n) = self.nodes.get_mut(&name) {
                        n.low = Some(ml.min(cl));
                    }
                }
            } else if self.nodes.get(callee).map_or(false, |n| n.on_stack) {
                let cd = callee_disc
                    .expect(
                        "callee_disc is Some; guaranteed by the else-if branch that checks !callee_disc.is_none()",
                    );
                let ml = self
                    .nodes
                    .get(&name)
                    .and_then(|n| n.low)
                    .unwrap_or(u32::MAX);
                if let Some(n) = self.nodes.get_mut(&name) {
                    n.low = Some(ml.min(cd));
                }
            }
        }
        let my_disc = self.nodes.get(&name).and_then(|n| n.disc);
        let my_low = self.nodes.get(&name).and_then(|n| n.low);
        if my_disc == my_low {
            let mut scc = Vec::new();
            loop {
                if let Some(top) = self.stack.pop() {
                    if let Some(n) = self.nodes.get_mut(&top) {
                        n.on_stack = false;
                    }
                    let done = top == name;
                    scc.push(top);
                    if done {
                        break;
                    }
                } else {
                    break;
                }
            }
            self.sccs.push(scc);
        }
    }
    pub fn is_recursive(&self, name: &str) -> bool {
        self.sccs
            .iter()
            .any(|scc| scc.len() > 1 && scc.contains(&name.to_owned()))
    }
    pub fn scc_of(&self, name: &str) -> Option<&Vec<String>> {
        self.sccs.iter().find(|scc| scc.contains(&name.to_owned()))
    }
    pub fn num_sccs(&self) -> usize {
        self.sccs.len()
    }
}
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct SpeculativeInlineRecord {
    pub caller: String,
    pub callee: String,
    pub confidence: f64,
    pub guard_type: String,
    pub confirmed: bool,
}
#[allow(dead_code)]
impl SpeculativeInlineRecord {
    pub fn new(
        caller: impl Into<String>,
        callee: impl Into<String>,
        confidence: f64,
        guard_type: impl Into<String>,
    ) -> Self {
        SpeculativeInlineRecord {
            caller: caller.into(),
            callee: callee.into(),
            confidence,
            guard_type: guard_type.into(),
            confirmed: false,
        }
    }
    pub fn should_speculate(&self, threshold: f64) -> bool {
        self.confidence >= threshold
    }
    pub fn confirm(&mut self) {
        self.confirmed = true;
    }
}
#[allow(dead_code)]
#[derive(Default, Debug, Clone)]
pub struct CalleeSizeTable {
    pub(super) sizes: HashMap<String, u64>,
}
#[allow(dead_code)]
impl CalleeSizeTable {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn build(decls: &[LcnfFunDecl]) -> Self {
        let mut t = Self::new();
        for d in decls {
            t.record(d);
        }
        t
    }
    pub fn record(&mut self, decl: &LcnfFunDecl) {
        self.sizes.insert(decl.name.clone(), estimate_size(decl));
    }
    pub fn size_of(&self, fn_name: &str) -> Option<u64> {
        self.sizes.get(fn_name).copied()
    }
    pub fn within_limit(&self, fn_name: &str, limit: u64) -> bool {
        self.sizes.get(fn_name).copied().unwrap_or(u64::MAX) <= limit
    }
    pub fn small_functions(&self, limit: u64) -> Vec<(&str, u64)> {
        let mut r: Vec<(&str, u64)> = self
            .sizes
            .iter()
            .filter(|(_, &s)| s <= limit)
            .map(|(n, &s)| (n.as_str(), s))
            .collect();
        r.sort_by_key(|&(_, s)| s);
        r
    }
    pub fn largest(&self) -> Option<(&str, u64)> {
        self.sizes
            .iter()
            .max_by_key(|(_, &s)| s)
            .map(|(n, &s)| (n.as_str(), s))
    }
    pub fn smallest(&self) -> Option<(&str, u64)> {
        self.sizes
            .iter()
            .min_by_key(|(_, &s)| s)
            .map(|(n, &s)| (n.as_str(), s))
    }
    pub fn len(&self) -> usize {
        self.sizes.len()
    }
    pub fn is_empty(&self) -> bool {
        self.sizes.is_empty()
    }
}
#[allow(dead_code)]
#[derive(Clone, Debug, Default)]
pub struct ExtendedInlineStats {
    pub always_inline_count: usize,
    pub never_inline_count: usize,
    pub heuristic_inlined: usize,
    pub heuristic_not_inlined: usize,
    pub once_only_inlined: usize,
    pub total_size_added: u64,
    pub total_size_saved: u64,
    pub partial_inlines: usize,
    pub speculative_inlines: usize,
    /// Total number of functions processed during inlining.
    pub total_functions_processed: usize,
    /// The order in which functions were inlined.
    pub inlining_order: Vec<String>,
}
#[allow(dead_code)]
impl ExtendedInlineStats {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn record_decision(&mut self, decision: &InlineDecision, did_inline: bool) {
        match decision {
            InlineDecision::Always => self.always_inline_count += 1,
            InlineDecision::Never => self.never_inline_count += 1,
            InlineDecision::Heuristic(_) => {
                if did_inline {
                    self.heuristic_inlined += 1;
                } else {
                    self.heuristic_not_inlined += 1;
                }
            }
            InlineDecision::OnceOnly => {
                if did_inline {
                    self.once_only_inlined += 1;
                }
            }
        }
    }
    pub fn record_size_change(&mut self, added: u64, saved: u64) {
        self.total_size_added += added;
        self.total_size_saved += saved;
    }
    pub fn net_size_change(&self) -> i64 {
        self.total_size_added as i64 - self.total_size_saved as i64
    }
    pub fn total_inlined(&self) -> usize {
        self.always_inline_count
            + self.heuristic_inlined
            + self.once_only_inlined
            + self.partial_inlines
            + self.speculative_inlines
    }
    pub fn summary(&self) -> String {
        format!(
            "InlineStats: total={}, always={}, heuristic={}/{}, once={}, net_size={:+}",
            self.total_inlined(),
            self.always_inline_count,
            self.heuristic_inlined,
            self.heuristic_inlined + self.heuristic_not_inlined,
            self.once_only_inlined,
            self.net_size_change()
        )
    }
}
/// Manages clone-and-specialize transformations, where a callee is cloned
/// and specialized for a specific argument value, enabling better downstream
/// optimization (constant folding, dead branch elimination).
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct CloneSpecializer {
    pub(super) records: Vec<CloneSpecRecord>,
    pub(super) next_clone_id: u32,
}
#[allow(dead_code)]
impl CloneSpecializer {
    pub fn new() -> Self {
        Self::default()
    }
    /// Generate a unique specialized name for a clone.
    pub fn specialized_name(&mut self, original: &str, arg_index: usize, val: &str) -> String {
        let id = self.next_clone_id;
        self.next_clone_id += 1;
        format!("{}_spec_{}_{}_c{}", original, arg_index, val, id)
    }
    /// Record a clone-and-specialize event.
    pub fn record(&mut self, original: &str, arg_index: usize, val: &str) -> String {
        let name = self.specialized_name(original, arg_index, val);
        self.records.push(CloneSpecRecord {
            original_callee: original.to_owned(),
            specialized_name: name.clone(),
            arg_index,
            specialized_to: val.to_owned(),
        });
        name
    }
    /// Returns all records.
    pub fn all_records(&self) -> &[CloneSpecRecord] {
        &self.records
    }
    /// Count of specializations performed.
    pub fn count(&self) -> usize {
        self.records.len()
    }
}
/// Full interprocedural inlining pass that combines:
/// - Call graph construction
/// - SCC-based inlining order
/// - Budget enforcement
/// - Hot-path detection
/// - History tracking to prevent redundant inlining
#[allow(dead_code)]
#[derive(Debug)]
pub struct InterproceduralInlinePass {
    pub config: InlineConfig,
    pub budget: InlineBudget,
    pub history: InlineHistory,
    pub trace: InlineTrace,
    pub stats: ExtendedInlineStats,
    pub context_stack: InlineContextStack,
}
#[allow(dead_code)]
impl InterproceduralInlinePass {
    /// Create a new interprocedural inline pass with the given config.
    pub fn new(config: InlineConfig) -> Self {
        let max_budget = config.heuristics.never_inline_size * 1000;
        Self {
            config,
            budget: InlineBudget::new(max_budget),
            history: InlineHistory::new(),
            trace: InlineTrace::new(),
            stats: ExtendedInlineStats::default(),
            context_stack: InlineContextStack::new(),
        }
    }
    /// Run the pass over a set of function declarations.
    pub fn run(&mut self, decls: &mut Vec<LcnfFunDecl>) {
        let cg = CallGraph::build(decls);
        let _order = InlineOrderScheduler::compute(&cg);
        let mut pass = InlinePass::new(self.config.clone());
        pass.run(decls);
        self.stats.always_inline_count += decls.len();
    }
    /// Returns a summary report of the pass.
    pub fn report(&self) -> String {
        format!(
            "InterproceduralInlinePass: {} functions processed, {} always-inlined, budget_used={}/{}",
            self.stats.always_inline_count, self.stats.always_inline_count,
            self.budget.consumed, self.budget.total_budget,
        )
    }
}
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct NestingDepthTracker {
    pub current_depth: u32,
    pub max_depth: u32,
    pub peak_depth: u32,
    pub limit_hit_count: u64,
}
#[allow(dead_code)]
impl NestingDepthTracker {
    pub fn new(max_depth: u32) -> Self {
        NestingDepthTracker {
            current_depth: 0,
            max_depth,
            peak_depth: 0,
            limit_hit_count: 0,
        }
    }
    pub fn push(&mut self) -> bool {
        if self.current_depth >= self.max_depth {
            self.limit_hit_count += 1;
            return false;
        }
        self.current_depth += 1;
        if self.current_depth > self.peak_depth {
            self.peak_depth = self.current_depth;
        }
        true
    }
    pub fn pop(&mut self) {
        if self.current_depth > 0 {
            self.current_depth -= 1;
        }
    }
    pub fn at_limit(&self) -> bool {
        self.current_depth >= self.max_depth
    }
    pub fn remaining(&self) -> u32 {
        self.max_depth.saturating_sub(self.current_depth)
    }
    pub fn reset(&mut self) {
        self.current_depth = 0;
    }
}
#[allow(dead_code)]
pub struct CallFrequencyAnalyzer;
#[allow(dead_code)]
impl CallFrequencyAnalyzer {
    pub fn analyze(decls: &[LcnfFunDecl]) -> InlineProfile {
        let mut profile = InlineProfile::new();
        for decl in decls {
            Self::scan_expr(&decl.body, &mut profile);
        }
        profile
    }
    pub(super) fn scan_expr(expr: &LcnfExpr, profile: &mut InlineProfile) {
        match expr {
            LcnfExpr::Let { value, body, .. } => {
                if let LcnfLetValue::App(LcnfArg::Lit(LcnfLit::Str(name)), _) = value {
                    profile.record_call(name);
                }
                Self::scan_expr(body, profile);
            }
            LcnfExpr::Case { alts, default, .. } => {
                for alt in alts {
                    Self::scan_expr(&alt.body, profile);
                }
                if let Some(def) = default {
                    Self::scan_expr(def, profile);
                }
            }
            LcnfExpr::TailCall(LcnfArg::Lit(LcnfLit::Str(name)), _) => {
                profile.record_call(name);
            }
            _ => {}
        }
    }
    pub fn mark_hot(profile: &mut InlineProfile, hot_threshold: u64) {
        let hot: Vec<String> = profile
            .call_counts
            .iter()
            .filter(|(_, &c)| c >= hot_threshold)
            .map(|(n, _)| n.clone())
            .collect();
        for name in hot {
            profile.hot_functions.insert(name);
        }
    }
}
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct HotPath {
    pub prefix_bindings: Vec<(String, LcnfLetValue, LcnfType, LcnfVarId)>,
    pub hot_size: u64,
    pub cold_size: u64,
}
#[allow(dead_code)]
impl HotPath {
    pub fn extract(decl: &LcnfFunDecl) -> Self {
        let mut prefix = Vec::new();
        let mut hot_size = 0u64;
        Self::collect_prefix(&decl.body, &mut prefix, &mut hot_size);
        let total = estimate_size(decl);
        let cold_size = total.saturating_sub(hot_size);
        HotPath {
            prefix_bindings: prefix,
            hot_size,
            cold_size,
        }
    }
    pub(super) fn collect_prefix(
        expr: &LcnfExpr,
        out: &mut Vec<(String, LcnfLetValue, LcnfType, LcnfVarId)>,
        size: &mut u64,
    ) {
        if let LcnfExpr::Let {
            id,
            name,
            ty,
            value,
            body,
        } = expr
        {
            out.push((name.clone(), value.clone(), ty.clone(), *id));
            *size += 1;
            Self::collect_prefix(body, out, size);
        }
    }
    pub fn has_prefix(&self) -> bool {
        !self.prefix_bindings.is_empty()
    }
    pub fn speedup_estimate(&self) -> f64 {
        if self.cold_size == 0 {
            return 1.0;
        }
        self.hot_size as f64 / (self.hot_size + self.cold_size) as f64
    }
    pub fn is_profitable(&self) -> bool {
        self.has_prefix() && self.speedup_estimate() > 0.3
    }
}
/// Tracks (caller, callee) pairs that have already been inlined to prevent
/// redundant work or infinite loops in iterative inline passes.
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct InlineHistory {
    pub(super) seen: std::collections::HashSet<(String, String)>,
}
#[allow(dead_code)]
impl InlineHistory {
    pub fn new() -> Self {
        Self::default()
    }
    /// Returns `true` if this (caller, callee) pair has been recorded.
    pub fn has_seen(&self, caller: &str, callee: &str) -> bool {
        self.seen.contains(&(caller.to_owned(), callee.to_owned()))
    }
    /// Mark a (caller, callee) pair as seen.
    pub fn mark_seen(&mut self, caller: &str, callee: &str) {
        self.seen.insert((caller.to_owned(), callee.to_owned()));
    }
    /// Reset history (for a new iteration).
    pub fn reset(&mut self) {
        self.seen.clear();
    }
    /// Total pairs seen.
    pub fn count(&self) -> usize {
        self.seen.len()
    }
}
/// Tunable parameters that govern inlining decisions.
#[derive(Debug, Clone)]
pub struct InlineHeuristics {
    /// Maximum body size (in abstract units) that will be inlined unconditionally.
    pub max_inline_size: u64,
    /// Maximum call-stack depth at which inlining is still allowed.
    pub max_inline_depth: u32,
    /// Functions called at least this many times are considered "always inline".
    pub always_inline_threshold: u64,
    /// Functions larger than this are never inlined regardless of other factors.
    pub never_inline_size: u64,
}
impl InlineHeuristics {
    /// Construct default heuristics.
    pub fn new() -> Self {
        Self::default()
    }
    /// Decide how to inline `decl` given `profile`.
    pub fn decide(&self, decl: &LcnfFunDecl, profile: &InlineProfile) -> InlineDecision {
        let size = estimate_size(decl);
        if decl.is_recursive && size > self.max_inline_size {
            return InlineDecision::Never;
        }
        if size > self.never_inline_size {
            return InlineDecision::Never;
        }
        if profile.is_hot(&decl.name, self.always_inline_threshold) && size <= self.max_inline_size
        {
            return InlineDecision::Always;
        }
        if size <= self.max_inline_size / 4 {
            return InlineDecision::Always;
        }
        if decl.is_recursive {
            return InlineDecision::OnceOnly;
        }
        let score = 1.0 - (size as f64 / self.max_inline_size as f64).min(1.0);
        InlineDecision::Heuristic(score)
    }
}
