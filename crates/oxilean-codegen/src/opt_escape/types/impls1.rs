//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::defs::*;
use super::impls2::*;
use crate::lcnf::{LcnfArg, LcnfExpr, LcnfFunDecl, LcnfLetValue};
use std::collections::{HashMap, HashSet};

use std::collections::VecDeque;

/// Liveness analysis for OEX2.
#[allow(dead_code)]
pub struct InterproceduralEscapeAnalysis {
    pub function_summaries: std::collections::HashMap<String, EscapeSummary>,
}
impl InterproceduralEscapeAnalysis {
    #[allow(dead_code)]
    pub fn new() -> Self {
        InterproceduralEscapeAnalysis {
            function_summaries: std::collections::HashMap::new(),
        }
    }
    #[allow(dead_code)]
    pub fn register_summary(&mut self, func: impl Into<String>, summary: EscapeSummary) {
        self.function_summaries.insert(func.into(), summary);
    }
    #[allow(dead_code)]
    pub fn get_summary(&self, func: &str) -> Option<&EscapeSummary> {
        self.function_summaries.get(func)
    }
    #[allow(dead_code)]
    pub fn param_escapes(&self, func: &str, param_idx: u32) -> bool {
        self.function_summaries
            .get(func)
            .map(|s| s.escaping_params.contains(&param_idx))
            .unwrap_or(true)
    }
    #[allow(dead_code)]
    pub fn function_count(&self) -> usize {
        self.function_summaries.len()
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct OEPassStats {
    pub total_runs: u32,
    pub successful_runs: u32,
    pub total_changes: u64,
    pub time_ms: u64,
    pub iterations_used: u32,
}
impl OEPassStats {
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PointsToTarget {
    HeapObject(u32),
    StackSlot(u32),
    GlobalVar(String),
    Unknown,
}
#[allow(dead_code)]
pub struct StructFieldEscapeAnalyzer {
    pub field_info: Vec<FieldEscapeInfo>,
}
impl StructFieldEscapeAnalyzer {
    #[allow(dead_code)]
    pub fn new() -> Self {
        StructFieldEscapeAnalyzer {
            field_info: Vec::new(),
        }
    }
    #[allow(dead_code)]
    pub fn add_field(&mut self, info: FieldEscapeInfo) {
        self.field_info.push(info);
    }
    #[allow(dead_code)]
    pub fn escaping_fields(&self) -> Vec<&FieldEscapeInfo> {
        self.field_info.iter().filter(|f| f.escapes()).collect()
    }
    #[allow(dead_code)]
    pub fn non_escaping_fields(&self) -> Vec<&FieldEscapeInfo> {
        self.field_info.iter().filter(|f| !f.escapes()).collect()
    }
    #[allow(dead_code)]
    pub fn can_scalar_replace_struct(&self, struct_type: &str) -> bool {
        let fields: Vec<_> = self
            .field_info
            .iter()
            .filter(|f| f.struct_type == struct_type)
            .collect();
        !fields.is_empty() && fields.iter().all(|f| !f.escapes())
    }
}
/// Dominator tree for OEExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct OEExtDomTree {
    pub(crate) idom: Vec<Option<usize>>,
    pub(crate) children: Vec<Vec<usize>>,
    pub(crate) depth: Vec<usize>,
}
impl OEExtDomTree {
    #[allow(dead_code)]
    pub fn new(n: usize) -> Self {
        Self {
            idom: vec![None; n],
            children: vec![Vec::new(); n],
            depth: vec![0; n],
        }
    }
    #[allow(dead_code)]
    pub fn set_idom(&mut self, node: usize, dom: usize) {
        if node < self.idom.len() {
            self.idom[node] = Some(dom);
            if dom < self.children.len() {
                self.children[dom].push(node);
            }
            self.depth[node] = if dom < self.depth.len() {
                self.depth[dom] + 1
            } else {
                1
            };
        }
    }
    #[allow(dead_code)]
    pub fn dominates(&self, a: usize, mut b: usize) -> bool {
        if a == b {
            return true;
        }
        let n = self.idom.len();
        for _ in 0..n {
            match self.idom.get(b).copied().flatten() {
                None => return false,
                Some(p) if p == a => return true,
                Some(p) if p == b => return false,
                Some(p) => b = p,
            }
        }
        false
    }
    #[allow(dead_code)]
    pub fn children_of(&self, n: usize) -> &[usize] {
        self.children.get(n).map(|v| v.as_slice()).unwrap_or(&[])
    }
    #[allow(dead_code)]
    pub fn depth_of(&self, n: usize) -> usize {
        self.depth.get(n).copied().unwrap_or(0)
    }
    #[allow(dead_code)]
    pub fn lca(&self, mut a: usize, mut b: usize) -> usize {
        let n = self.idom.len();
        for _ in 0..(2 * n) {
            if a == b {
                return a;
            }
            if self.depth_of(a) > self.depth_of(b) {
                a = self.idom.get(a).and_then(|x| *x).unwrap_or(a);
            } else {
                b = self.idom.get(b).and_then(|x| *x).unwrap_or(b);
            }
        }
        0
    }
}
/// The escape status of an allocation site.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EscapeStatus {
    /// The allocation does not escape: safe for stack allocation.
    NoEscape,
    /// The allocation escapes to a local variable only.
    LocalEscape,
    /// The allocation escapes to the heap (assigned to a heap-allocated struct).
    HeapEscape,
    /// The allocation is returned from the function.
    ReturnEscape,
    /// The allocation escapes as argument index `usize` of some call.
    ArgumentEscape(usize),
    /// Escape status is unknown (conservative assumption).
    Unknown,
}
impl EscapeStatus {
    /// Returns `true` if this allocation must live on the heap.
    pub fn is_heap_allocated(&self) -> bool {
        matches!(
            self,
            EscapeStatus::HeapEscape
                | EscapeStatus::ReturnEscape
                | EscapeStatus::ArgumentEscape(_)
                | EscapeStatus::Unknown
        )
    }
    /// Returns `true` if this allocation can be placed on the stack.
    pub fn can_stack_allocate(&self) -> bool {
        matches!(self, EscapeStatus::NoEscape | EscapeStatus::LocalEscape)
    }
}
/// Liveness analysis for OEExt.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct OEExtLiveness {
    pub live_in: Vec<Vec<usize>>,
    pub live_out: Vec<Vec<usize>>,
    pub defs: Vec<Vec<usize>>,
    pub uses: Vec<Vec<usize>>,
}
impl OEExtLiveness {
    #[allow(dead_code)]
    pub fn new(n: usize) -> Self {
        Self {
            live_in: vec![Vec::new(); n],
            live_out: vec![Vec::new(); n],
            defs: vec![Vec::new(); n],
            uses: vec![Vec::new(); n],
        }
    }
    #[allow(dead_code)]
    pub fn live_in(&self, b: usize, v: usize) -> bool {
        self.live_in.get(b).map(|s| s.contains(&v)).unwrap_or(false)
    }
    #[allow(dead_code)]
    pub fn live_out(&self, b: usize, v: usize) -> bool {
        self.live_out
            .get(b)
            .map(|s| s.contains(&v))
            .unwrap_or(false)
    }
    #[allow(dead_code)]
    pub fn add_def(&mut self, b: usize, v: usize) {
        if let Some(s) = self.defs.get_mut(b) {
            if !s.contains(&v) {
                s.push(v);
            }
        }
    }
    #[allow(dead_code)]
    pub fn add_use(&mut self, b: usize, v: usize) {
        if let Some(s) = self.uses.get_mut(b) {
            if !s.contains(&v) {
                s.push(v);
            }
        }
    }
    #[allow(dead_code)]
    pub fn var_is_used_in_block(&self, b: usize, v: usize) -> bool {
        self.uses.get(b).map(|s| s.contains(&v)).unwrap_or(false)
    }
    #[allow(dead_code)]
    pub fn var_is_def_in_block(&self, b: usize, v: usize) -> bool {
        self.defs.get(b).map(|s| s.contains(&v)).unwrap_or(false)
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct EscapeSummary {
    pub escaping_params: Vec<u32>,
    pub return_escapes: bool,
    pub modifies_global: bool,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct OEPassConfig {
    pub phase: OEPassPhase,
    pub enabled: bool,
    pub max_iterations: u32,
    pub debug_output: bool,
    pub pass_name: String,
}
impl OEPassConfig {
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>, phase: OEPassPhase) -> Self {
        OEPassConfig {
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
#[derive(Debug, Clone)]
pub struct OEDepGraph {
    pub(crate) nodes: Vec<u32>,
    pub(crate) edges: Vec<(u32, u32)>,
}
impl OEDepGraph {
    #[allow(dead_code)]
    pub fn new() -> Self {
        OEDepGraph {
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
/// Pass execution phase for OEExt.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum OEExtPassPhase {
    Early,
    Middle,
    Late,
    Finalize,
}
impl OEExtPassPhase {
    #[allow(dead_code)]
    pub fn is_early(&self) -> bool {
        matches!(self, Self::Early)
    }
    #[allow(dead_code)]
    pub fn is_middle(&self) -> bool {
        matches!(self, Self::Middle)
    }
    #[allow(dead_code)]
    pub fn is_late(&self) -> bool {
        matches!(self, Self::Late)
    }
    #[allow(dead_code)]
    pub fn is_finalize(&self) -> bool {
        matches!(self, Self::Finalize)
    }
    #[allow(dead_code)]
    pub fn order(&self) -> u32 {
        match self {
            Self::Early => 0,
            Self::Middle => 1,
            Self::Late => 2,
            Self::Finalize => 3,
        }
    }
    #[allow(dead_code)]
    pub fn from_order(n: u32) -> Option<Self> {
        match n {
            0 => Some(Self::Early),
            1 => Some(Self::Middle),
            2 => Some(Self::Late),
            3 => Some(Self::Finalize),
            _ => None,
        }
    }
}
/// Describes a single allocation site within a function.
#[derive(Debug, Clone)]
pub struct AllocationSite {
    /// The variable that holds the allocated value.
    pub var: String,
    /// The function in which this allocation occurs.
    pub func: String,
    /// Estimated size of the allocation in bytes.
    pub size_estimate: u64,
    /// The escape status for this allocation.
    pub status: EscapeStatus,
}
impl AllocationSite {
    /// Create a new allocation site with `Unknown` status and zero size estimate.
    pub fn new(var: impl Into<String>, func: impl Into<String>) -> Self {
        AllocationSite {
            var: var.into(),
            func: func.into(),
            size_estimate: 0,
            status: EscapeStatus::Unknown,
        }
    }
    /// Update the escape status of this allocation site.
    pub fn set_status(&mut self, s: EscapeStatus) {
        self.status = s;
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct OECacheEntry {
    pub key: String,
    pub data: Vec<u8>,
    pub timestamp: u64,
    pub valid: bool,
}
/// A flow graph whose edges indicate "var A can flow into var B".
///
/// If A flows into B and B escapes, then A also escapes.
#[derive(Debug, Clone, Default)]
pub struct EscapeGraph {
    /// `edges[a]` lists all variables that `a` can flow into.
    pub(crate) edges: HashMap<String, Vec<String>>,
}
impl EscapeGraph {
    /// Create an empty escape graph.
    pub fn new() -> Self {
        EscapeGraph {
            edges: HashMap::new(),
        }
    }
    /// Record that the allocation held in `from` can flow into `to`.
    pub fn add_edge(&mut self, from: &str, to: &str) {
        self.edges
            .entry(from.to_owned())
            .or_default()
            .push(to.to_owned());
    }
    /// Compute all variables reachable from `src` via the flow edges.
    pub fn reachable_from(&self, src: &str) -> HashSet<String> {
        let mut visited = HashSet::new();
        let mut worklist = vec![src.to_owned()];
        while let Some(node) = worklist.pop() {
            if visited.insert(node.clone()) {
                if let Some(neighbors) = self.edges.get(&node) {
                    for n in neighbors {
                        if !visited.contains(n) {
                            worklist.push(n.clone());
                        }
                    }
                }
            }
        }
        visited
    }
}
/// Worklist for OEX2.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct OEX2Worklist {
    pub(crate) items: std::collections::VecDeque<usize>,
    pub(crate) present: Vec<bool>,
}
impl OEX2Worklist {
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
#[allow(dead_code)]
pub struct EscapeOptimizationPass {
    pub results: Vec<EscapeOptimizationResult>,
    pub min_confidence: f64,
}
impl EscapeOptimizationPass {
    #[allow(dead_code)]
    pub fn new() -> Self {
        EscapeOptimizationPass {
            results: Vec::new(),
            min_confidence: 0.8,
        }
    }
    #[allow(dead_code)]
    pub fn add_result(&mut self, result: EscapeOptimizationResult) {
        self.results.push(result);
    }
    #[allow(dead_code)]
    pub fn stack_promotable(&self) -> Vec<&EscapeOptimizationResult> {
        self.results
            .iter()
            .filter(|r| {
                r.recommended_sink.is_stack_eligible() && r.confidence >= self.min_confidence
            })
            .collect()
    }
    #[allow(dead_code)]
    pub fn total_optimizations(&self) -> usize {
        self.results.len()
    }
    #[allow(dead_code)]
    pub fn emit_report(&self) -> String {
        let mut out = format!(
            "Escape Optimization Report: {} results\n",
            self.results.len()
        );
        let promotable = self.stack_promotable();
        out.push_str(&format!(
            "  Stack-promotable allocations: {}\n",
            promotable.len()
        ));
        for r in &promotable {
            out.push_str(&format!(
                "    Alloc #{}: {} (confidence: {:.0}%)\n",
                r.allocation_id,
                r.reason,
                r.confidence * 100.0
            ));
        }
        out
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ConnectionNode {
    pub id: u32,
    pub escape_state: EscapeStatus,
    pub kind: String,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct EscapeFlowEdge {
    pub from: u32,
    pub to: u32,
    pub edge_kind: EscapeEdgeKind,
}
/// Optimization pass that uses escape analysis results to annotate or rewrite
/// LCNF declarations so that non-escaping allocations can be stack-allocated.
#[derive(Debug, Default)]
pub struct StackAllocationOpt {
    /// The escape analyzer backing this pass.
    pub analyzer: EscapeAnalyzer,
    /// Configuration.
    pub config: EscapeOptConfig,
}
impl StackAllocationOpt {
    /// Create a new pass with default configuration.
    pub fn new() -> Self {
        StackAllocationOpt {
            analyzer: EscapeAnalyzer::new(),
            config: EscapeOptConfig::default(),
        }
    }
    /// Create a new pass with explicit configuration.
    pub fn with_config(config: EscapeOptConfig) -> Self {
        StackAllocationOpt {
            analyzer: EscapeAnalyzer::new(),
            config,
        }
    }
    /// Run the pass over all declarations.  Declarations are modified in-place
    /// (currently: analysis results are stored; future work: rewrite IR).
    pub fn run(&mut self, decls: &mut [LcnfFunDecl]) {
        self.analyzer.analyze(decls);
        let names: Vec<String> = decls.iter().map(|d| d.name.clone()).collect();
        for decl in decls.iter_mut() {
            if let Some(analysis) = names
                .iter()
                .find(|n| *n == &decl.name)
                .and_then(|n| self.analyzer.results.get(n))
            {
                self.optimize_decl(decl, analysis);
            }
        }
    }
    /// Apply escape-based optimizations to a single declaration.
    pub fn optimize_decl(&self, decl: &mut LcnfFunDecl, analysis: &EscapeAnalysisResult) {
        if !self.config.enable_stack_alloc {
            return;
        }
        let candidates: HashSet<String> = analysis
            .stack_allocation_candidates()
            .into_iter()
            .filter(|site| site.size_estimate <= self.config.max_stack_size_bytes)
            .map(|site| site.var.clone())
            .collect();
        if !candidates.is_empty() {
            Self::mark_stack_allocated(&mut decl.body, &candidates);
        }
    }
    /// Walk the expression tree and mark allocation sites that appear in
    /// `candidates` as stack-allocatable (currently a no-op annotation hook;
    /// real backends would lower these differently).
    pub fn mark_stack_allocated(expr: &mut LcnfExpr, candidates: &HashSet<String>) {
        match expr {
            LcnfExpr::Let { name, body, .. } => {
                let _is_stack = candidates.contains(name.as_str());
                Self::mark_stack_allocated(body, candidates);
            }
            LcnfExpr::Case { alts, default, .. } => {
                for alt in alts.iter_mut() {
                    Self::mark_stack_allocated(&mut alt.body, candidates);
                }
                if let Some(def) = default {
                    Self::mark_stack_allocated(def, candidates);
                }
            }
            LcnfExpr::Return(_) | LcnfExpr::TailCall(_, _) | LcnfExpr::Unreachable => {}
        }
    }
    /// Build an `EscapeReport` from the analysis results accumulated so far.
    pub fn report(&self) -> EscapeReport {
        let mut rep = EscapeReport::default();
        for analysis in self.analyzer.results.values() {
            rep.total_allocations += analysis.allocations.len();
            for site in &analysis.allocations {
                match &site.status {
                    EscapeStatus::NoEscape | EscapeStatus::LocalEscape => {
                        rep.stack_allocated += 1;
                    }
                    EscapeStatus::HeapEscape
                    | EscapeStatus::ReturnEscape
                    | EscapeStatus::ArgumentEscape(_) => {
                        rep.heap_allocated += 1;
                    }
                    EscapeStatus::Unknown => {
                        rep.unknown += 1;
                    }
                }
            }
        }
        rep
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct EscapeAnnotationPass {
    pub annotated_nodes: Vec<(u32, EscapeAnnotation)>,
}
impl EscapeAnnotationPass {
    #[allow(dead_code)]
    pub fn new() -> Self {
        EscapeAnnotationPass {
            annotated_nodes: Vec::new(),
        }
    }
    #[allow(dead_code)]
    pub fn annotate(&mut self, node: u32, annotation: EscapeAnnotation) {
        self.annotated_nodes.push((node, annotation));
    }
    #[allow(dead_code)]
    pub fn get_annotation(&self, node: u32) -> Option<&EscapeAnnotation> {
        self.annotated_nodes
            .iter()
            .find(|(id, _)| *id == node)
            .map(|(_, a)| a)
    }
    #[allow(dead_code)]
    pub fn stack_promote_candidates(&self) -> Vec<u32> {
        self.annotated_nodes
            .iter()
            .filter(|(_, a)| matches!(a, EscapeAnnotation::StackAlloc))
            .map(|(id, _)| *id)
            .collect()
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct EscapeOptimizationResult {
    pub allocation_id: u32,
    pub original_kind: String,
    pub recommended_sink: AllocationSinkKind,
    pub confidence: f64,
    pub reason: String,
}
impl EscapeOptimizationResult {
    #[allow(dead_code)]
    pub fn new(allocation_id: u32, sink: AllocationSinkKind, reason: impl Into<String>) -> Self {
        EscapeOptimizationResult {
            allocation_id,
            original_kind: "heap".to_string(),
            recommended_sink: sink,
            confidence: 1.0,
            reason: reason.into(),
        }
    }
    #[allow(dead_code)]
    pub fn with_confidence(mut self, c: f64) -> Self {
        self.confidence = c;
        self
    }
    #[allow(dead_code)]
    pub fn is_high_confidence(&self) -> bool {
        self.confidence >= 0.9
    }
}
/// Pass execution phase for OEX2.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum OEX2PassPhase {
    Early,
    Middle,
    Late,
    Finalize,
}
impl OEX2PassPhase {
    #[allow(dead_code)]
    pub fn is_early(&self) -> bool {
        matches!(self, Self::Early)
    }
    #[allow(dead_code)]
    pub fn is_middle(&self) -> bool {
        matches!(self, Self::Middle)
    }
    #[allow(dead_code)]
    pub fn is_late(&self) -> bool {
        matches!(self, Self::Late)
    }
    #[allow(dead_code)]
    pub fn is_finalize(&self) -> bool {
        matches!(self, Self::Finalize)
    }
    #[allow(dead_code)]
    pub fn order(&self) -> u32 {
        match self {
            Self::Early => 0,
            Self::Middle => 1,
            Self::Late => 2,
            Self::Finalize => 3,
        }
    }
    #[allow(dead_code)]
    pub fn from_order(n: u32) -> Option<Self> {
        match n {
            0 => Some(Self::Early),
            1 => Some(Self::Middle),
            2 => Some(Self::Late),
            3 => Some(Self::Finalize),
            _ => None,
        }
    }
}
/// Dominator tree for OEX2.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct OEX2DomTree {
    pub(crate) idom: Vec<Option<usize>>,
    pub(crate) children: Vec<Vec<usize>>,
    pub(crate) depth: Vec<usize>,
}
impl OEX2DomTree {
    #[allow(dead_code)]
    pub fn new(n: usize) -> Self {
        Self {
            idom: vec![None; n],
            children: vec![Vec::new(); n],
            depth: vec![0; n],
        }
    }
    #[allow(dead_code)]
    pub fn set_idom(&mut self, node: usize, dom: usize) {
        if node < self.idom.len() {
            self.idom[node] = Some(dom);
            if dom < self.children.len() {
                self.children[dom].push(node);
            }
            self.depth[node] = if dom < self.depth.len() {
                self.depth[dom] + 1
            } else {
                1
            };
        }
    }
    #[allow(dead_code)]
    pub fn dominates(&self, a: usize, mut b: usize) -> bool {
        if a == b {
            return true;
        }
        let n = self.idom.len();
        for _ in 0..n {
            match self.idom.get(b).copied().flatten() {
                None => return false,
                Some(p) if p == a => return true,
                Some(p) if p == b => return false,
                Some(p) => b = p,
            }
        }
        false
    }
    #[allow(dead_code)]
    pub fn children_of(&self, n: usize) -> &[usize] {
        self.children.get(n).map(|v| v.as_slice()).unwrap_or(&[])
    }
    #[allow(dead_code)]
    pub fn depth_of(&self, n: usize) -> usize {
        self.depth.get(n).copied().unwrap_or(0)
    }
    #[allow(dead_code)]
    pub fn lca(&self, mut a: usize, mut b: usize) -> usize {
        let n = self.idom.len();
        for _ in 0..(2 * n) {
            if a == b {
                return a;
            }
            if self.depth_of(a) > self.depth_of(b) {
                a = self.idom.get(a).and_then(|x| *x).unwrap_or(a);
            } else {
                b = self.idom.get(b).and_then(|x| *x).unwrap_or(b);
            }
        }
        0
    }
}
#[allow(dead_code)]
pub struct EscapeAnalysisSummaryPrinter;
impl EscapeAnalysisSummaryPrinter {
    #[allow(dead_code)]
    pub fn print_result(result: &EscapeAnalysisResult) -> String {
        let mut out = String::from("=== Escape Analysis Result ===\n");
        out.push_str(&format!("Allocations: {}\n", result.allocations.len()));
        out.push_str(&format!("Escape sets: {}\n", result.escape_sets.len()));
        out.push_str(&format!("Function: {}\n", result.func_name));
        out
    }
    #[allow(dead_code)]
    pub fn print_report(report: &EscapeReport) -> String {
        let mut out = String::from("=== Escape Report ===\n");
        out.push_str(&format!(
            "Total allocations: {}\n",
            report.total_allocations
        ));
        out.push_str(&format!("Stack-allocated: {}\n", report.stack_allocated));
        out.push_str(&format!("Heap-allocated: {}\n", report.heap_allocated));
        out
    }
}
/// An annotation that records the chosen allocation strategy for an expression.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EscapeAnnotation {
    /// This allocation will be placed on the stack.
    StackAlloc,
    /// This allocation will be placed on the heap.
    HeapAlloc,
    /// Allocation strategy is not yet determined.
    Unknown,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct OELivenessInfo {
    pub live_in: Vec<std::collections::HashSet<u32>>,
    pub live_out: Vec<std::collections::HashSet<u32>>,
    pub defs: Vec<std::collections::HashSet<u32>>,
    pub uses: Vec<std::collections::HashSet<u32>>,
}
