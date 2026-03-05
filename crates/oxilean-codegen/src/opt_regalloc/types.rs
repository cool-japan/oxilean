//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use crate::lcnf::{LcnfArg, LcnfExpr, LcnfFunDecl, LcnfLetValue, LcnfType, LcnfVarId};
use std::collections::{HashMap, HashSet, VecDeque};

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RADominatorTree {
    pub idom: Vec<Option<u32>>,
    pub dom_children: Vec<Vec<u32>>,
    pub dom_depth: Vec<u32>,
}
impl RADominatorTree {
    #[allow(dead_code)]
    pub fn new(size: usize) -> Self {
        RADominatorTree {
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
/// Pass execution phase for RAExt.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RAExtPassPhase {
    Early,
    Middle,
    Late,
    Finalize,
}
impl RAExtPassPhase {
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
/// Chaitin-Briggs graph coloring register allocator.
///
/// Phases:
/// 1. Build interference graph from live intervals.
/// 2. Simplify: repeatedly remove nodes with degree < k (available colors).
/// 3. Coalesce: merge copy-related nodes that don't interfere.
/// 4. Spill: if no simplifiable node, mark highest-cost node for potential spill.
/// 5. Select: pop nodes off the stack and assign colors.
#[derive(Debug)]
pub struct GraphColoringAllocator {
    /// Number of available colors (physical registers).
    pub num_colors: usize,
    /// Physical registers available.
    pub phys_regs: Vec<PhysReg>,
    /// Current interference graph (mutated during simplification).
    pub graph: InterferenceGraph,
    /// Stack of removed nodes (in simplification order).
    pub(super) simplify_stack: Vec<LcnfVarId>,
    /// Nodes marked as potential spills.
    pub(super) potential_spills: Vec<LcnfVarId>,
    /// Number of coalescing merges performed.
    pub coalesced_count: usize,
    /// Number of simplification iterations.
    pub iterations: usize,
}
impl GraphColoringAllocator {
    /// Create a new Chaitin-Briggs allocator.
    pub fn new(phys_regs: Vec<PhysReg>) -> Self {
        let k = phys_regs.len();
        GraphColoringAllocator {
            num_colors: k,
            phys_regs,
            graph: InterferenceGraph::new(),
            simplify_stack: Vec::new(),
            potential_spills: Vec::new(),
            coalesced_count: 0,
            iterations: 0,
        }
    }
    /// Build the interference graph from live intervals.
    pub fn build_interference_graph(&mut self, intervals: &[LiveInterval]) -> InterferenceGraph {
        self.graph = InterferenceGraph::build_from_intervals(intervals);
        self.graph.clone()
    }
    /// Simplify phase: push low-degree (<k) nodes onto the stack.
    ///
    /// Returns the number of nodes removed in this pass.
    pub fn simplify(&mut self) -> usize {
        self.iterations += 1;
        let k = self.num_colors;
        let low_degree: Vec<LcnfVarId> = self
            .graph
            .nodes
            .iter()
            .copied()
            .filter(|&v| self.graph.degree(v) < k)
            .collect();
        let removed = low_degree.len();
        for v in low_degree {
            self.graph.remove_node(v);
            self.simplify_stack.push(v);
        }
        removed
    }
    /// Coalesce phase: merge copy-related vregs that don't interfere.
    ///
    /// Uses the George / Briggs conservative coalescing heuristic.
    pub fn coalesce(&mut self, vreg_map: &mut HashMap<LcnfVarId, LcnfVarId>) {
        let pairs = self.graph.move_pairs.clone();
        for (u, v) in pairs {
            let ru = *vreg_map.get(&u).unwrap_or(&u);
            let rv = *vreg_map.get(&v).unwrap_or(&v);
            if ru == rv {
                continue;
            }
            if !self.graph.interferes(ru, rv) {
                let neighbors: Vec<LcnfVarId> = self
                    .graph
                    .edges
                    .get(&rv)
                    .cloned()
                    .unwrap_or_default()
                    .into_iter()
                    .collect();
                for n in neighbors {
                    self.graph.add_edge(ru, n);
                }
                self.graph.remove_node(rv);
                vreg_map.insert(rv, ru);
                self.coalesced_count += 1;
            }
        }
    }
    /// Spill selection: pick the node with the lowest spill weight to spill.
    pub fn spill_select(&mut self, intervals: &[LiveInterval]) -> Option<LcnfVarId> {
        if self.graph.nodes.is_empty() {
            return None;
        }
        let candidate = self.graph.nodes.iter().copied().min_by(|&a, &b| {
            let weight_a = intervals
                .iter()
                .find(|iv| iv.vreg == a)
                .map(|iv| iv.spill_weight)
                .unwrap_or(1.0);
            let weight_b = intervals
                .iter()
                .find(|iv| iv.vreg == b)
                .map(|iv| iv.spill_weight)
                .unwrap_or(1.0);
            weight_a
                .partial_cmp(&weight_b)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        if let Some(v) = candidate {
            self.graph.remove_node(v);
            self.simplify_stack.push(v);
            self.potential_spills.push(v);
        }
        candidate
    }
    /// Color phase: pop nodes from the stack and assign colors.
    ///
    /// Returns the coloring (vreg → phys reg) and the set of actual spills.
    #[allow(clippy::too_many_arguments)]
    pub fn color(
        &self,
        k: usize,
        original_graph: &InterferenceGraph,
    ) -> HashMap<LcnfVarId, PhysReg> {
        let mut coloring: HashMap<LcnfVarId, PhysReg> = HashMap::new();
        for &vreg in self.simplify_stack.iter().rev() {
            let used_colors: HashSet<u32> = original_graph
                .edges
                .get(&vreg)
                .iter()
                .flat_map(|neighbors| neighbors.iter())
                .filter_map(|n| coloring.get(n))
                .map(|pr| pr.id)
                .collect();
            let assigned = (0..k).find(|&c| !used_colors.contains(&(c as u32)));
            if let Some(c) = assigned {
                coloring.insert(vreg, self.phys_regs[c].clone());
            }
        }
        coloring
    }
    /// Run the full Chaitin-Briggs allocation for a set of intervals.
    ///
    /// Returns an `Allocation`.
    pub fn allocate(&mut self, intervals: &[LiveInterval]) -> Allocation {
        let original_graph = self.build_interference_graph(intervals);
        let mut vreg_map: HashMap<LcnfVarId, LcnfVarId> = HashMap::new();
        loop {
            let removed = self.simplify();
            if removed == 0 {
                if self.graph.nodes.is_empty() {
                    break;
                }
                self.coalesce(&mut vreg_map);
                if self.spill_select(intervals).is_none() {
                    break;
                }
            }
            if self.graph.nodes.is_empty() {
                break;
            }
        }
        let coloring = self.color(self.num_colors, &original_graph);
        let mut alloc = Allocation::new();
        for iv in intervals {
            let canonical = *vreg_map.get(&iv.vreg).unwrap_or(&iv.vreg);
            if let Some(preg) = coloring.get(&canonical).or_else(|| coloring.get(&iv.vreg)) {
                alloc.assign(iv.vreg, preg.clone());
            } else {
                alloc.spill(iv.vreg, 8);
            }
        }
        alloc
    }
}
/// A feature flag set for RegAlloc capabilities.
#[derive(Debug, Clone, Default)]
pub struct RegAllocFeatures {
    pub(super) flags: std::collections::HashSet<String>,
}
impl RegAllocFeatures {
    pub fn new() -> Self {
        RegAllocFeatures::default()
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
    pub fn union(&self, other: &RegAllocFeatures) -> RegAllocFeatures {
        RegAllocFeatures {
            flags: self.flags.union(&other.flags).cloned().collect(),
        }
    }
    pub fn intersection(&self, other: &RegAllocFeatures) -> RegAllocFeatures {
        RegAllocFeatures {
            flags: self.flags.intersection(&other.flags).cloned().collect(),
        }
    }
}
/// A candidate for spilling, ranked by cost.
#[derive(Debug, Clone)]
pub struct SpillCandidate {
    /// The virtual register to spill.
    pub vreg: LcnfVarId,
    /// Estimated spill cost (frequency * size).
    pub cost: f64,
}
impl SpillCandidate {
    /// Create a new spill candidate.
    pub fn new(vreg: LcnfVarId, cost: f64) -> Self {
        SpillCandidate { vreg, cost }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RAAnalysisCache {
    pub(super) entries: std::collections::HashMap<String, RACacheEntry>,
    pub(super) max_size: usize,
    pub(super) hits: u64,
    pub(super) misses: u64,
}
impl RAAnalysisCache {
    #[allow(dead_code)]
    pub fn new(max_size: usize) -> Self {
        RAAnalysisCache {
            entries: std::collections::HashMap::new(),
            max_size,
            hits: 0,
            misses: 0,
        }
    }
    #[allow(dead_code)]
    pub fn get(&mut self, key: &str) -> Option<&RACacheEntry> {
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
            RACacheEntry {
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
/// Summary statistics from register allocation.
#[derive(Debug, Clone, Default)]
pub struct RegAllocReport {
    /// Number of virtual registers successfully allocated to physical registers.
    pub vregs_allocated: usize,
    /// Number of virtual registers that were spilled to the stack.
    pub spills: usize,
    /// Number of copy pairs coalesced (eliminates move instructions).
    pub copies_coalesced: usize,
    /// Number of coloring / simplification iterations performed.
    pub coloring_iterations: usize,
    /// Total stack frame size used for spill slots (bytes).
    pub stack_frame_bytes: u32,
    /// Number of functions processed.
    pub functions_processed: usize,
}
impl RegAllocReport {
    /// Spill ratio: fraction of vregs that were spilled.
    pub fn spill_ratio(&self) -> f64 {
        let total = self.vregs_allocated + self.spills;
        if total == 0 {
            0.0
        } else {
            self.spills as f64 / total as f64
        }
    }
}
/// Collects RegAlloc diagnostics.
#[derive(Debug, Default)]
pub struct RegAllocDiagCollector {
    pub(super) msgs: Vec<RegAllocDiagMsg>,
}
impl RegAllocDiagCollector {
    pub fn new() -> Self {
        RegAllocDiagCollector::default()
    }
    pub fn emit(&mut self, d: RegAllocDiagMsg) {
        self.msgs.push(d);
    }
    pub fn has_errors(&self) -> bool {
        self.msgs
            .iter()
            .any(|d| d.severity == RegAllocDiagSeverity::Error)
    }
    pub fn errors(&self) -> Vec<&RegAllocDiagMsg> {
        self.msgs
            .iter()
            .filter(|d| d.severity == RegAllocDiagSeverity::Error)
            .collect()
    }
    pub fn warnings(&self) -> Vec<&RegAllocDiagMsg> {
        self.msgs
            .iter()
            .filter(|d| d.severity == RegAllocDiagSeverity::Warning)
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
/// Register allocation pass.
///
/// Supports both linear scan and graph coloring algorithms.
#[derive(Debug)]
pub struct RegAllocPass {
    /// Physical register bank.
    pub phys_regs: Vec<PhysReg>,
    /// Whether to use graph coloring (Chaitin-Briggs) instead of linear scan.
    pub use_graph_coloring: bool,
    /// Accumulated report.
    pub(super) report: RegAllocReport,
    /// Allocation results per function (function name → allocation).
    pub allocations: HashMap<String, Allocation>,
}
impl RegAllocPass {
    /// Create a new pass with `num_regs` integer registers using linear scan.
    pub fn new(num_regs: u32) -> Self {
        RegAllocPass {
            phys_regs: PhysReg::integer_bank(num_regs),
            use_graph_coloring: false,
            report: RegAllocReport::default(),
            allocations: HashMap::new(),
        }
    }
    /// Create a pass that uses Chaitin-Briggs graph coloring.
    pub fn graph_coloring(num_regs: u32) -> Self {
        RegAllocPass {
            phys_regs: PhysReg::integer_bank(num_regs),
            use_graph_coloring: true,
            report: RegAllocReport::default(),
            allocations: HashMap::new(),
        }
    }
    /// Create a pass with an explicit physical register set.
    pub fn with_regs(phys_regs: Vec<PhysReg>, use_graph_coloring: bool) -> Self {
        RegAllocPass {
            phys_regs,
            use_graph_coloring,
            report: RegAllocReport::default(),
            allocations: HashMap::new(),
        }
    }
    /// Run register allocation on all function declarations.
    pub fn run(&mut self, decls: &mut [LcnfFunDecl]) {
        for decl in decls.iter() {
            self.report.functions_processed += 1;
            let alloc = if self.use_graph_coloring {
                self.run_graph_coloring(decl)
            } else {
                self.run_linear_scan(decl)
            };
            self.report.vregs_allocated += alloc.reg_map.len();
            self.report.spills += alloc.spills.len();
            self.report.stack_frame_bytes += alloc.stack_frame_size();
            self.allocations.insert(decl.name.clone(), alloc);
        }
    }
    /// Run linear scan allocation for a single declaration.
    pub(super) fn run_linear_scan(&self, decl: &LcnfFunDecl) -> Allocation {
        let mut lsa = LinearScanAllocator::new(self.phys_regs.clone());
        let intervals = lsa.build_live_intervals(decl);
        let n = self.phys_regs.len();
        lsa.linear_scan(intervals, n)
    }
    /// Run graph coloring allocation for a single declaration.
    pub(super) fn run_graph_coloring(&self, decl: &LcnfFunDecl) -> Allocation {
        let lsa = LinearScanAllocator::new(self.phys_regs.clone());
        let intervals = lsa.build_live_intervals(decl);
        let mut gca = GraphColoringAllocator::new(self.phys_regs.clone());
        let alloc = gca.allocate(&intervals);
        let _ = self.report.clone();
        alloc
    }
    /// Return the accumulated allocation report.
    pub fn report(&self) -> RegAllocReport {
        self.report.clone()
    }
    /// Lookup the allocation for a function by name.
    pub fn allocation_for(&self, func: &str) -> Option<&Allocation> {
        self.allocations.get(func)
    }
}
/// The register class (hardware register file) of a physical register.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RegClass {
    /// General-purpose integer registers (e.g. rax, rbx, …).
    Integer,
    /// Floating-point / SSE / AVX registers (e.g. xmm0, …).
    Float,
    /// SIMD / vector registers (e.g. ymm0, zmm0, …).
    Vector,
    /// Predicate / condition registers (e.g. ARM p0, x86 k0, …).
    Predicate,
}
impl RegClass {
    /// A short name for this class used in physical register names.
    pub fn prefix(&self) -> &'static str {
        match self {
            RegClass::Integer => "r",
            RegClass::Float => "f",
            RegClass::Vector => "v",
            RegClass::Predicate => "p",
        }
    }
}
/// The result of register allocation.
#[derive(Debug, Clone, Default)]
pub struct Allocation {
    /// Maps virtual register IDs to physical registers.
    pub reg_map: HashMap<LcnfVarId, PhysReg>,
    /// Virtual registers that were spilled to the stack.
    pub spills: Vec<SpillSlot>,
    /// Next available stack offset.
    pub(super) next_stack_offset: u32,
}
impl Allocation {
    /// Create an empty allocation.
    pub fn new() -> Self {
        Allocation::default()
    }
    /// Assign a physical register to a virtual register.
    pub fn assign(&mut self, vreg: LcnfVarId, preg: PhysReg) {
        self.reg_map.insert(vreg, preg);
    }
    /// Spill a virtual register, returning the slot.
    pub fn spill(&mut self, vreg: LcnfVarId, size: u32) -> SpillSlot {
        let slot = SpillSlot::new(vreg, self.next_stack_offset, size);
        self.next_stack_offset += size;
        self.spills.push(slot.clone());
        slot
    }
    /// Look up the physical register for `vreg`.
    pub fn lookup(&self, vreg: LcnfVarId) -> Option<&PhysReg> {
        self.reg_map.get(&vreg)
    }
    /// Returns `true` if `vreg` was spilled.
    pub fn is_spilled(&self, vreg: LcnfVarId) -> bool {
        self.spills.iter().any(|s| s.vreg == vreg)
    }
    /// Total stack frame size needed for spills.
    pub fn stack_frame_size(&self) -> u32 {
        self.next_stack_offset
    }
}
/// A diagnostic message from a RegAlloc pass.
#[derive(Debug, Clone)]
pub struct RegAllocDiagMsg {
    pub severity: RegAllocDiagSeverity,
    pub pass: String,
    pub message: String,
}
impl RegAllocDiagMsg {
    pub fn error(pass: impl Into<String>, msg: impl Into<String>) -> Self {
        RegAllocDiagMsg {
            severity: RegAllocDiagSeverity::Error,
            pass: pass.into(),
            message: msg.into(),
        }
    }
    pub fn warning(pass: impl Into<String>, msg: impl Into<String>) -> Self {
        RegAllocDiagMsg {
            severity: RegAllocDiagSeverity::Warning,
            pass: pass.into(),
            message: msg.into(),
        }
    }
    pub fn note(pass: impl Into<String>, msg: impl Into<String>) -> Self {
        RegAllocDiagMsg {
            severity: RegAllocDiagSeverity::Note,
            pass: pass.into(),
            message: msg.into(),
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RACacheEntry {
    pub key: String,
    pub data: Vec<u8>,
    pub timestamp: u64,
    pub valid: bool,
}
/// Dependency graph for RAExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RAExtDepGraph {
    pub(super) n: usize,
    pub(super) adj: Vec<Vec<usize>>,
    pub(super) rev: Vec<Vec<usize>>,
    pub(super) edge_count: usize,
}
impl RAExtDepGraph {
    #[allow(dead_code)]
    pub fn new(n: usize) -> Self {
        Self {
            n,
            adj: vec![Vec::new(); n],
            rev: vec![Vec::new(); n],
            edge_count: 0,
        }
    }
    #[allow(dead_code)]
    pub fn add_edge(&mut self, from: usize, to: usize) {
        if from < self.n && to < self.n {
            if !self.adj[from].contains(&to) {
                self.adj[from].push(to);
                self.rev[to].push(from);
                self.edge_count += 1;
            }
        }
    }
    #[allow(dead_code)]
    pub fn succs(&self, n: usize) -> &[usize] {
        self.adj.get(n).map(|v| v.as_slice()).unwrap_or(&[])
    }
    #[allow(dead_code)]
    pub fn preds(&self, n: usize) -> &[usize] {
        self.rev.get(n).map(|v| v.as_slice()).unwrap_or(&[])
    }
    #[allow(dead_code)]
    pub fn topo_sort(&self) -> Option<Vec<usize>> {
        let mut deg: Vec<usize> = (0..self.n).map(|i| self.rev[i].len()).collect();
        let mut q: std::collections::VecDeque<usize> =
            (0..self.n).filter(|&i| deg[i] == 0).collect();
        let mut out = Vec::with_capacity(self.n);
        while let Some(u) = q.pop_front() {
            out.push(u);
            for &v in &self.adj[u] {
                deg[v] -= 1;
                if deg[v] == 0 {
                    q.push_back(v);
                }
            }
        }
        if out.len() == self.n {
            Some(out)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn has_cycle(&self) -> bool {
        self.topo_sort().is_none()
    }
    #[allow(dead_code)]
    pub fn reachable(&self, start: usize) -> Vec<usize> {
        let mut vis = vec![false; self.n];
        let mut stk = vec![start];
        let mut out = Vec::new();
        while let Some(u) = stk.pop() {
            if u < self.n && !vis[u] {
                vis[u] = true;
                out.push(u);
                for &v in &self.adj[u] {
                    if !vis[v] {
                        stk.push(v);
                    }
                }
            }
        }
        out
    }
    #[allow(dead_code)]
    pub fn scc(&self) -> Vec<Vec<usize>> {
        let mut visited = vec![false; self.n];
        let mut order = Vec::new();
        for i in 0..self.n {
            if !visited[i] {
                let mut stk = vec![(i, 0usize)];
                while let Some((u, idx)) = stk.last_mut() {
                    if !visited[*u] {
                        visited[*u] = true;
                    }
                    if *idx < self.adj[*u].len() {
                        let v = self.adj[*u][*idx];
                        *idx += 1;
                        if !visited[v] {
                            stk.push((v, 0));
                        }
                    } else {
                        order.push(*u);
                        stk.pop();
                    }
                }
            }
        }
        let mut comp = vec![usize::MAX; self.n];
        let mut components: Vec<Vec<usize>> = Vec::new();
        for &start in order.iter().rev() {
            if comp[start] == usize::MAX {
                let cid = components.len();
                let mut component = Vec::new();
                let mut stk = vec![start];
                while let Some(u) = stk.pop() {
                    if comp[u] == usize::MAX {
                        comp[u] = cid;
                        component.push(u);
                        for &v in &self.rev[u] {
                            if comp[v] == usize::MAX {
                                stk.push(v);
                            }
                        }
                    }
                }
                components.push(component);
            }
        }
        components
    }
    #[allow(dead_code)]
    pub fn node_count(&self) -> usize {
        self.n
    }
    #[allow(dead_code)]
    pub fn edge_count(&self) -> usize {
        self.edge_count
    }
}
/// Pipeline profiler for RegAlloc.
#[derive(Debug, Default)]
pub struct RegAllocProfiler {
    pub(super) timings: Vec<RegAllocPassTiming>,
}
impl RegAllocProfiler {
    pub fn new() -> Self {
        RegAllocProfiler::default()
    }
    pub fn record(&mut self, t: RegAllocPassTiming) {
        self.timings.push(t);
    }
    pub fn total_elapsed_us(&self) -> u64 {
        self.timings.iter().map(|t| t.elapsed_us).sum()
    }
    pub fn slowest_pass(&self) -> Option<&RegAllocPassTiming> {
        self.timings.iter().max_by_key(|t| t.elapsed_us)
    }
    pub fn num_passes(&self) -> usize {
        self.timings.len()
    }
    pub fn profitable_passes(&self) -> Vec<&RegAllocPassTiming> {
        self.timings.iter().filter(|t| t.is_profitable()).collect()
    }
}
/// Emission statistics for RegAlloc.
#[derive(Debug, Clone, Default)]
pub struct RegAllocEmitStats {
    pub bytes_emitted: usize,
    pub items_emitted: usize,
    pub errors: usize,
    pub warnings: usize,
    pub elapsed_ms: u64,
}
impl RegAllocEmitStats {
    pub fn new() -> Self {
        RegAllocEmitStats::default()
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
/// An undirected interference graph for register allocation.
///
/// Nodes are virtual registers; an edge (u, v) means u and v have
/// overlapping live ranges and therefore cannot share a physical register.
#[derive(Debug, Clone, Default)]
pub struct InterferenceGraph {
    /// Adjacency lists.
    pub edges: HashMap<LcnfVarId, HashSet<LcnfVarId>>,
    /// All nodes (vregs) in the graph.
    pub nodes: HashSet<LcnfVarId>,
    /// Move-related pairs: pairs of vregs connected by copy instructions
    /// that are candidates for coalescing.
    pub move_pairs: Vec<(LcnfVarId, LcnfVarId)>,
}
impl InterferenceGraph {
    /// Create an empty interference graph.
    pub fn new() -> Self {
        InterferenceGraph::default()
    }
    /// Add a node to the graph.
    pub fn add_node(&mut self, vreg: LcnfVarId) {
        self.nodes.insert(vreg);
        self.edges.entry(vreg).or_default();
    }
    /// Add an interference edge between `u` and `v`.
    pub fn add_edge(&mut self, u: LcnfVarId, v: LcnfVarId) {
        if u == v {
            return;
        }
        self.nodes.insert(u);
        self.nodes.insert(v);
        self.edges.entry(u).or_default().insert(v);
        self.edges.entry(v).or_default().insert(u);
    }
    /// Returns `true` if `u` and `v` interfere.
    pub fn interferes(&self, u: LcnfVarId, v: LcnfVarId) -> bool {
        self.edges.get(&u).map(|s| s.contains(&v)).unwrap_or(false)
    }
    /// Returns the degree of node `vreg`.
    pub fn degree(&self, vreg: LcnfVarId) -> usize {
        self.edges.get(&vreg).map(|s| s.len()).unwrap_or(0)
    }
    /// Remove a node and all its edges.
    pub fn remove_node(&mut self, vreg: LcnfVarId) {
        if let Some(neighbors) = self.edges.remove(&vreg) {
            for n in neighbors {
                if let Some(s) = self.edges.get_mut(&n) {
                    s.remove(&vreg);
                }
            }
        }
        self.nodes.remove(&vreg);
    }
    /// Record a move-related pair (candidate for coalescing).
    pub fn add_move_pair(&mut self, u: LcnfVarId, v: LcnfVarId) {
        if u != v {
            self.move_pairs.push((u, v));
        }
    }
    /// Build an interference graph from a set of live intervals.
    pub fn build_from_intervals(intervals: &[LiveInterval]) -> Self {
        let mut g = InterferenceGraph::new();
        for iv in intervals {
            g.add_node(iv.vreg);
        }
        for i in 0..intervals.len() {
            for j in (i + 1)..intervals.len() {
                if intervals[i].overlaps(&intervals[j]) {
                    g.add_edge(intervals[i].vreg, intervals[j].vreg);
                }
            }
        }
        g
    }
}
/// Heuristic freshness key for RegAlloc incremental compilation.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RegAllocIncrKey {
    pub content_hash: u64,
    pub config_hash: u64,
}
impl RegAllocIncrKey {
    pub fn new(content: u64, config: u64) -> Self {
        RegAllocIncrKey {
            content_hash: content,
            config_hash: config,
        }
    }
    pub fn combined_hash(&self) -> u64 {
        self.content_hash.wrapping_mul(0x9e3779b97f4a7c15) ^ self.config_hash
    }
    pub fn matches(&self, other: &RegAllocIncrKey) -> bool {
        self.content_hash == other.content_hash && self.config_hash == other.config_hash
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RALivenessInfo {
    pub live_in: Vec<std::collections::HashSet<u32>>,
    pub live_out: Vec<std::collections::HashSet<u32>>,
    pub defs: Vec<std::collections::HashSet<u32>>,
    pub uses: Vec<std::collections::HashSet<u32>>,
}
impl RALivenessInfo {
    #[allow(dead_code)]
    pub fn new(block_count: usize) -> Self {
        RALivenessInfo {
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
/// Liveness analysis for RAExt.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct RAExtLiveness {
    pub live_in: Vec<Vec<usize>>,
    pub live_out: Vec<Vec<usize>>,
    pub defs: Vec<Vec<usize>>,
    pub uses: Vec<Vec<usize>>,
}
impl RAExtLiveness {
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
/// A generic key-value configuration store for RegAlloc.
#[derive(Debug, Clone, Default)]
pub struct RegAllocConfig {
    pub(super) entries: std::collections::HashMap<String, String>,
}
impl RegAllocConfig {
    pub fn new() -> Self {
        RegAllocConfig::default()
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
/// Severity of a RegAlloc diagnostic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RegAllocDiagSeverity {
    Note,
    Warning,
    Error,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RAWorklist {
    pub(super) items: std::collections::VecDeque<u32>,
    pub(super) in_worklist: std::collections::HashSet<u32>,
}
impl RAWorklist {
    #[allow(dead_code)]
    pub fn new() -> Self {
        RAWorklist {
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
/// A physical register on the target architecture.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PhysReg {
    /// Unique numeric ID of this register.
    pub id: u32,
    /// Human-readable name (e.g. "rax", "f0", "xmm3").
    pub name: String,
    /// The register class.
    pub class: RegClass,
}
impl PhysReg {
    /// Create a new physical register.
    pub fn new(id: u32, name: impl Into<String>, class: RegClass) -> Self {
        PhysReg {
            id,
            name: name.into(),
            class,
        }
    }
    /// Create a set of `n` integer physical registers named r0..r{n-1}.
    pub fn integer_bank(n: u32) -> Vec<PhysReg> {
        (0..n)
            .map(|i| PhysReg::new(i, format!("r{}", i), RegClass::Integer))
            .collect()
    }
    /// Create a set of `n` float physical registers named f0..f{n-1}.
    pub fn float_bank(n: u32) -> Vec<PhysReg> {
        (0..n)
            .map(|i| PhysReg::new(i, format!("f{}", i), RegClass::Float))
            .collect()
    }
}
/// A monotonically increasing ID generator for RegAlloc.
#[derive(Debug, Default)]
pub struct RegAllocIdGen {
    pub(super) next: u32,
}
impl RegAllocIdGen {
    pub fn new() -> Self {
        RegAllocIdGen::default()
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
/// A stack spill slot for a virtual register.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SpillSlot {
    /// The spilled virtual register.
    pub vreg: LcnfVarId,
    /// Byte offset on the stack frame.
    pub offset: u32,
    /// Size of the slot in bytes.
    pub size: u32,
}
impl SpillSlot {
    /// Create a new spill slot.
    pub fn new(vreg: LcnfVarId, offset: u32, size: u32) -> Self {
        SpillSlot { vreg, offset, size }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RAPassConfig {
    pub phase: RAPassPhase,
    pub enabled: bool,
    pub max_iterations: u32,
    pub debug_output: bool,
    pub pass_name: String,
}
impl RAPassConfig {
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>, phase: RAPassPhase) -> Self {
        RAPassConfig {
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
/// A text buffer for building RegAlloc output source code.
#[derive(Debug, Default)]
pub struct RegAllocSourceBuffer {
    pub(super) buf: String,
    pub(super) indent_level: usize,
    pub(super) indent_str: String,
}
impl RegAllocSourceBuffer {
    pub fn new() -> Self {
        RegAllocSourceBuffer {
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
/// Tracks declared names for RegAlloc scope analysis.
#[derive(Debug, Default)]
pub struct RegAllocNameScope {
    pub(super) declared: std::collections::HashSet<String>,
    pub(super) depth: usize,
    pub(super) parent: Option<Box<RegAllocNameScope>>,
}
impl RegAllocNameScope {
    pub fn new() -> Self {
        RegAllocNameScope::default()
    }
    pub fn declare(&mut self, name: impl Into<String>) -> bool {
        self.declared.insert(name.into())
    }
    pub fn is_declared(&self, name: &str) -> bool {
        self.declared.contains(name)
    }
    pub fn push_scope(self) -> Self {
        RegAllocNameScope {
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
/// Pass-timing record for RegAlloc profiler.
#[derive(Debug, Clone)]
pub struct RegAllocPassTiming {
    pub pass_name: String,
    pub elapsed_us: u64,
    pub items_processed: usize,
    pub bytes_before: usize,
    pub bytes_after: usize,
}
impl RegAllocPassTiming {
    pub fn new(
        pass_name: impl Into<String>,
        elapsed_us: u64,
        items: usize,
        before: usize,
        after: usize,
    ) -> Self {
        RegAllocPassTiming {
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
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct RAPassStats {
    pub total_runs: u32,
    pub successful_runs: u32,
    pub total_changes: u64,
    pub time_ms: u64,
    pub iterations_used: u32,
}
impl RAPassStats {
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
pub struct RAConstantFoldingHelper;
impl RAConstantFoldingHelper {
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
/// A virtual register (pre-allocation).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VirtualReg {
    /// Unique ID matching the corresponding `LcnfVarId`.
    pub id: u32,
    /// The LCNF type of this register's value.
    pub ty: LcnfType,
    /// An optional hint towards a preferred physical register.
    pub hint: Option<PhysReg>,
}
impl VirtualReg {
    /// Create a new virtual register without a hint.
    pub fn new(id: u32, ty: LcnfType) -> Self {
        VirtualReg { id, ty, hint: None }
    }
    /// Create a virtual register with a physical-register hint.
    pub fn with_hint(id: u32, ty: LcnfType, hint: PhysReg) -> Self {
        VirtualReg {
            id,
            ty,
            hint: Some(hint),
        }
    }
    /// Determine which register class this vreg belongs to.
    pub fn reg_class(&self) -> RegClass {
        match &self.ty {
            LcnfType::Fun(_, _) => RegClass::Integer,
            LcnfType::Nat => RegClass::Integer,
            LcnfType::LcnfString => RegClass::Integer,
            LcnfType::Object => RegClass::Integer,
            LcnfType::Ctor(_, _) => RegClass::Integer,
            LcnfType::Var(_) => RegClass::Integer,
            LcnfType::Erased | LcnfType::Unit | LcnfType::Irrelevant => RegClass::Integer,
        }
    }
}
/// The live interval [start, end) of a virtual register.
///
/// `start` is the instruction index where the vreg is first defined,
/// `end` is one past the last use.
#[derive(Debug, Clone, PartialEq)]
pub struct LiveInterval {
    /// The virtual register this interval belongs to.
    pub vreg: LcnfVarId,
    /// Inclusive start (def point).
    pub start: u32,
    /// Exclusive end (last use + 1).
    pub end: u32,
    /// All use points (instruction indices where this vreg is read).
    pub uses: Vec<u32>,
    /// All def points (instruction indices where this vreg is written).
    pub defs: Vec<u32>,
    /// Spill weight: higher = more costly to spill.
    pub spill_weight: f64,
}
impl LiveInterval {
    /// Create a new live interval.
    pub fn new(vreg: LcnfVarId, start: u32, end: u32) -> Self {
        LiveInterval {
            vreg,
            start,
            end,
            uses: vec![],
            defs: vec![],
            spill_weight: 1.0,
        }
    }
    /// Returns `true` if this interval overlaps with `other`.
    pub fn overlaps(&self, other: &LiveInterval) -> bool {
        self.start < other.end && other.start < self.end
    }
    /// Length of the live interval in instructions.
    pub fn length(&self) -> u32 {
        self.end.saturating_sub(self.start)
    }
    /// Add a use point.
    pub fn add_use(&mut self, pos: u32) {
        if !self.uses.contains(&pos) {
            self.uses.push(pos);
            if pos >= self.end {
                self.end = pos + 1;
            }
        }
    }
    /// Add a def point.
    pub fn add_def(&mut self, pos: u32) {
        if !self.defs.contains(&pos) {
            self.defs.push(pos);
            if pos < self.start {
                self.start = pos;
            }
        }
    }
    /// Compute spill weight = total uses / length (more uses → costlier to spill).
    pub fn compute_spill_weight(&mut self) {
        let len = self.length().max(1) as f64;
        self.spill_weight = self.uses.len() as f64 / len;
    }
}
#[allow(dead_code)]
pub struct RAPassRegistry {
    pub(super) configs: Vec<RAPassConfig>,
    pub(super) stats: std::collections::HashMap<String, RAPassStats>,
}
impl RAPassRegistry {
    #[allow(dead_code)]
    pub fn new() -> Self {
        RAPassRegistry {
            configs: Vec::new(),
            stats: std::collections::HashMap::new(),
        }
    }
    #[allow(dead_code)]
    pub fn register(&mut self, config: RAPassConfig) {
        self.stats
            .insert(config.pass_name.clone(), RAPassStats::new());
        self.configs.push(config);
    }
    #[allow(dead_code)]
    pub fn enabled_passes(&self) -> Vec<&RAPassConfig> {
        self.configs.iter().filter(|c| c.enabled).collect()
    }
    #[allow(dead_code)]
    pub fn get_stats(&self, name: &str) -> Option<&RAPassStats> {
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
/// Configuration for RAExt passes.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RAExtPassConfig {
    pub name: String,
    pub phase: RAExtPassPhase,
    pub enabled: bool,
    pub max_iterations: usize,
    pub debug: u32,
    pub timeout_ms: Option<u64>,
}
impl RAExtPassConfig {
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            phase: RAExtPassPhase::Middle,
            enabled: true,
            max_iterations: 100,
            debug: 0,
            timeout_ms: None,
        }
    }
    #[allow(dead_code)]
    pub fn with_phase(mut self, phase: RAExtPassPhase) -> Self {
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
/// Worklist for RAExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RAExtWorklist {
    pub(super) items: std::collections::VecDeque<usize>,
    pub(super) present: Vec<bool>,
}
impl RAExtWorklist {
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
/// A version tag for RegAlloc output artifacts.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RegAllocVersion {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
    pub pre: Option<String>,
}
impl RegAllocVersion {
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        RegAllocVersion {
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
    pub fn is_compatible_with(&self, other: &RegAllocVersion) -> bool {
        self.major == other.major && self.minor >= other.minor
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum RAPassPhase {
    Analysis,
    Transformation,
    Verification,
    Cleanup,
}
impl RAPassPhase {
    #[allow(dead_code)]
    pub fn name(&self) -> &str {
        match self {
            RAPassPhase::Analysis => "analysis",
            RAPassPhase::Transformation => "transformation",
            RAPassPhase::Verification => "verification",
            RAPassPhase::Cleanup => "cleanup",
        }
    }
    #[allow(dead_code)]
    pub fn is_modifying(&self) -> bool {
        matches!(self, RAPassPhase::Transformation | RAPassPhase::Cleanup)
    }
}
/// Statistics for RAExt passes.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct RAExtPassStats {
    pub iterations: usize,
    pub changed: bool,
    pub nodes_visited: usize,
    pub nodes_modified: usize,
    pub time_ms: u64,
    pub memory_bytes: usize,
    pub errors: usize,
}
impl RAExtPassStats {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    #[allow(dead_code)]
    pub fn visit(&mut self) {
        self.nodes_visited += 1;
    }
    #[allow(dead_code)]
    pub fn modify(&mut self) {
        self.nodes_modified += 1;
        self.changed = true;
    }
    #[allow(dead_code)]
    pub fn iterate(&mut self) {
        self.iterations += 1;
    }
    #[allow(dead_code)]
    pub fn error(&mut self) {
        self.errors += 1;
    }
    #[allow(dead_code)]
    pub fn efficiency(&self) -> f64 {
        if self.nodes_visited == 0 {
            0.0
        } else {
            self.nodes_modified as f64 / self.nodes_visited as f64
        }
    }
    #[allow(dead_code)]
    pub fn merge(&mut self, o: &RAExtPassStats) {
        self.iterations += o.iterations;
        self.changed |= o.changed;
        self.nodes_visited += o.nodes_visited;
        self.nodes_modified += o.nodes_modified;
        self.time_ms += o.time_ms;
        self.memory_bytes = self.memory_bytes.max(o.memory_bytes);
        self.errors += o.errors;
    }
}
/// Pass registry for RAExt.
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct RAExtPassRegistry {
    pub(super) configs: Vec<RAExtPassConfig>,
    pub(super) stats: Vec<RAExtPassStats>,
}
impl RAExtPassRegistry {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    #[allow(dead_code)]
    pub fn register(&mut self, c: RAExtPassConfig) {
        self.stats.push(RAExtPassStats::new());
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
    pub fn get(&self, i: usize) -> Option<&RAExtPassConfig> {
        self.configs.get(i)
    }
    #[allow(dead_code)]
    pub fn get_stats(&self, i: usize) -> Option<&RAExtPassStats> {
        self.stats.get(i)
    }
    #[allow(dead_code)]
    pub fn enabled_passes(&self) -> Vec<&RAExtPassConfig> {
        self.configs.iter().filter(|c| c.enabled).collect()
    }
    #[allow(dead_code)]
    pub fn passes_in_phase(&self, ph: &RAExtPassPhase) -> Vec<&RAExtPassConfig> {
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
/// Analysis cache for RAExt.
#[allow(dead_code)]
#[derive(Debug)]
pub struct RAExtCache {
    pub(super) entries: Vec<(u64, Vec<u8>, bool, u32)>,
    pub(super) cap: usize,
    pub(super) total_hits: u64,
    pub(super) total_misses: u64,
}
impl RAExtCache {
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
/// Constant folding helper for RAExt.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct RAExtConstFolder {
    pub(super) folds: usize,
    pub(super) failures: usize,
    pub(super) enabled: bool,
}
impl RAExtConstFolder {
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
/// Linear scan register allocator.
///
/// Algorithm (Poletto & Sarkar, 1999):
/// 1. Sort live intervals by start point.
/// 2. For each interval:
///    a. Expire all intervals that end before this one starts → free their registers.
///    b. If a physical register is free, assign it.
///    c. Otherwise spill the interval with the furthest end point (or this interval).
#[derive(Debug)]
pub struct LinearScanAllocator {
    /// Available physical registers.
    pub phys_regs: Vec<PhysReg>,
    /// Number of spills performed.
    pub spill_count: usize,
}
impl LinearScanAllocator {
    /// Create a new allocator with the given physical register bank.
    pub fn new(phys_regs: Vec<PhysReg>) -> Self {
        LinearScanAllocator {
            phys_regs,
            spill_count: 0,
        }
    }
    /// Build live intervals for all variables in a function declaration.
    ///
    /// Uses a simple linear numbering: each let-binding gets a consecutive index.
    pub fn build_live_intervals(&self, decl: &LcnfFunDecl) -> Vec<LiveInterval> {
        let mut counter = 0u32;
        let mut intervals: HashMap<LcnfVarId, LiveInterval> = HashMap::new();
        for param in &decl.params {
            let mut iv = LiveInterval::new(param.id, 0, 1);
            iv.add_def(0);
            intervals.insert(param.id, iv);
        }
        collect_intervals_from_expr(&decl.body, &mut counter, &mut intervals);
        let mut result: Vec<LiveInterval> = intervals.into_values().collect();
        for iv in &mut result {
            iv.compute_spill_weight();
        }
        result.sort_by_key(|iv| iv.start);
        result
    }
    /// Run linear scan allocation.
    ///
    /// Returns an `Allocation` mapping vregs to pregs (or spill slots).
    pub fn linear_scan(&mut self, intervals: Vec<LiveInterval>, num_phys: usize) -> Allocation {
        let mut alloc = Allocation::new();
        let mut free_regs: Vec<usize> = (0..num_phys.min(self.phys_regs.len())).collect();
        let mut active: Vec<(u32, LcnfVarId, usize)> = Vec::new();
        for iv in &intervals {
            active.retain(|&(end, old_vreg, preg_idx)| {
                if end <= iv.start {
                    free_regs.push(preg_idx);
                    let _ = old_vreg;
                    false
                } else {
                    true
                }
            });
            if let Some(preg_idx) = free_regs.pop() {
                alloc.assign(iv.vreg, self.phys_regs[preg_idx].clone());
                active.push((iv.end, iv.vreg, preg_idx));
                active.sort_by_key(|&(end, _, _)| end);
            } else {
                if let Some(&(far_end, far_vreg, preg_idx)) = active.last() {
                    if far_end > iv.end {
                        alloc.spill(far_vreg, 8);
                        self.spill_count += 1;
                        active.pop();
                        alloc.assign(iv.vreg, self.phys_regs[preg_idx].clone());
                        active.push((iv.end, iv.vreg, preg_idx));
                        active.sort_by_key(|&(end, _, _)| end);
                    } else {
                        alloc.spill(iv.vreg, 8);
                        self.spill_count += 1;
                    }
                } else {
                    alloc.spill(iv.vreg, 8);
                    self.spill_count += 1;
                }
            }
        }
        alloc
    }
    /// Spill a specific virtual register (creates a spill slot).
    pub fn handle_spill(&mut self, vreg: LcnfVarId, alloc: &mut Allocation) -> SpillSlot {
        self.spill_count += 1;
        alloc.spill(vreg, 8)
    }
}
/// A fixed-capacity ring buffer of strings (for recent-event logging in RegAlloc).
#[derive(Debug)]
pub struct RegAllocEventLog {
    pub(super) entries: std::collections::VecDeque<String>,
    pub(super) capacity: usize,
}
impl RegAllocEventLog {
    pub fn new(capacity: usize) -> Self {
        RegAllocEventLog {
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
#[derive(Debug, Clone)]
pub struct RADepGraph {
    pub(super) nodes: Vec<u32>,
    pub(super) edges: Vec<(u32, u32)>,
}
impl RADepGraph {
    #[allow(dead_code)]
    pub fn new() -> Self {
        RADepGraph {
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
/// Dominator tree for RAExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RAExtDomTree {
    pub(super) idom: Vec<Option<usize>>,
    pub(super) children: Vec<Vec<usize>>,
    pub(super) depth: Vec<usize>,
}
impl RAExtDomTree {
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
