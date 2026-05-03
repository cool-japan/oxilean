//! Implementation blocks (part 1)

use super::super::functions::*;
use super::defs::*;
use crate::lcnf::{LcnfArg, LcnfExpr, LcnfFunDecl, LcnfLetValue, LcnfType, LcnfVarId};
use std::collections::{HashMap, HashSet, VecDeque};

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
impl SpillCandidate {
    /// Create a new spill candidate.
    pub fn new(vreg: LcnfVarId, cost: f64) -> Self {
        SpillCandidate { vreg, cost }
    }
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
