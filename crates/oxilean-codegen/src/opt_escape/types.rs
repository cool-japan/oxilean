//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::{LcnfArg, LcnfExpr, LcnfFunDecl, LcnfLetValue};
use std::collections::{HashMap, HashSet};

use std::collections::VecDeque;

/// Liveness analysis for OEX2.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct OEX2Liveness {
    pub live_in: Vec<Vec<usize>>,
    pub live_out: Vec<Vec<usize>>,
    pub defs: Vec<Vec<usize>>,
    pub uses: Vec<Vec<usize>>,
}
impl OEX2Liveness {
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
/// Dependency graph for OEExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct OEExtDepGraph {
    pub(super) n: usize,
    pub(super) adj: Vec<Vec<usize>>,
    pub(super) rev: Vec<Vec<usize>>,
    pub(super) edge_count: usize,
}
impl OEExtDepGraph {
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
/// A summary report produced after running the escape optimization.
#[derive(Debug, Clone, Default)]
pub struct EscapeReport {
    /// Total allocation sites examined.
    pub total_allocations: usize,
    /// Sites that will be stack-allocated.
    pub stack_allocated: usize,
    /// Sites that must remain heap-allocated.
    pub heap_allocated: usize,
    /// Sites whose allocation strategy is unknown.
    pub unknown: usize,
}
impl EscapeReport {
    /// Return a human-readable one-line summary.
    pub fn summary(&self) -> String {
        format!(
            "EscapeReport: total={} stack={} heap={} unknown={} (estimated savings: {} bytes)",
            self.total_allocations,
            self.stack_allocated,
            self.heap_allocated,
            self.unknown,
            self.stack_savings_estimate(),
        )
    }
    /// Estimate memory savings from stack allocation (assumes 8-byte pointer overhead saved
    /// per stack-allocated object).
    pub fn stack_savings_estimate(&self) -> u64 {
        self.stack_allocated as u64 * 32
    }
}
/// Dependency graph for OEX2.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct OEX2DepGraph {
    pub(super) n: usize,
    pub(super) adj: Vec<Vec<usize>>,
    pub(super) rev: Vec<Vec<usize>>,
    pub(super) edge_count: usize,
}
impl OEX2DepGraph {
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
/// Pass registry for OEExt.
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct OEExtPassRegistry {
    pub(super) configs: Vec<OEExtPassConfig>,
    pub(super) stats: Vec<OEExtPassStats>,
}
impl OEExtPassRegistry {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    #[allow(dead_code)]
    pub fn register(&mut self, c: OEExtPassConfig) {
        self.stats.push(OEExtPassStats::new());
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
    pub fn get(&self, i: usize) -> Option<&OEExtPassConfig> {
        self.configs.get(i)
    }
    #[allow(dead_code)]
    pub fn get_stats(&self, i: usize) -> Option<&OEExtPassStats> {
        self.stats.get(i)
    }
    #[allow(dead_code)]
    pub fn enabled_passes(&self) -> Vec<&OEExtPassConfig> {
        self.configs.iter().filter(|c| c.enabled).collect()
    }
    #[allow(dead_code)]
    pub fn passes_in_phase(&self, ph: &OEExtPassPhase) -> Vec<&OEExtPassConfig> {
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
/// Configuration for OEExt passes.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct OEExtPassConfig {
    pub name: String,
    pub phase: OEExtPassPhase,
    pub enabled: bool,
    pub max_iterations: usize,
    pub debug: u32,
    pub timeout_ms: Option<u64>,
}
impl OEExtPassConfig {
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            phase: OEExtPassPhase::Middle,
            enabled: true,
            max_iterations: 100,
            debug: 0,
            timeout_ms: None,
        }
    }
    #[allow(dead_code)]
    pub fn with_phase(mut self, phase: OEExtPassPhase) -> Self {
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
/// Performs escape analysis over a collection of LCNF function declarations.
#[derive(Debug, Default)]
pub struct EscapeAnalyzer {
    /// Analysis results keyed by function name.
    pub results: HashMap<String, EscapeAnalysisResult>,
}
impl EscapeAnalyzer {
    /// Create a new analyzer.
    pub fn new() -> Self {
        EscapeAnalyzer {
            results: HashMap::new(),
        }
    }
    /// Analyze all declarations and store results.
    pub fn analyze(&mut self, decls: &[LcnfFunDecl]) {
        for decl in decls {
            let result = self.analyze_decl(decl);
            self.results.insert(decl.name.clone(), result);
        }
    }
    /// Analyze a single function declaration.
    pub fn analyze_decl(&mut self, decl: &LcnfFunDecl) -> EscapeAnalysisResult {
        let mut result = EscapeAnalysisResult::new(&decl.name);
        self.analyze_expr(&decl.body, &mut result, true);
        self.propagate_escapes(&mut result);
        result
    }
    /// Recursively analyze an expression.
    ///
    /// `in_tail` is `true` when the expression appears in tail position
    /// (i.e., its value is directly returned from the function).
    pub fn analyze_expr(&self, expr: &LcnfExpr, result: &mut EscapeAnalysisResult, in_tail: bool) {
        match expr {
            LcnfExpr::Let {
                name, value, body, ..
            } => {
                self.analyze_let_value(name, value, result);
                self.analyze_expr(body, result, in_tail);
            }
            LcnfExpr::Case { alts, default, .. } => {
                for alt in alts {
                    self.analyze_expr(&alt.body, result, in_tail);
                }
                if let Some(def) = default {
                    self.analyze_expr(def, result, in_tail);
                }
            }
            LcnfExpr::Return(arg) => {
                if let LcnfArg::Var(vid) = arg {
                    let var_name = format!("_x{}", vid.0);
                    result
                        .escape_sets
                        .insert(var_name.clone(), EscapeStatus::ReturnEscape);
                    for site in &mut result.allocations {
                        if site.var == var_name {
                            site.set_status(EscapeStatus::ReturnEscape);
                        }
                    }
                }
                let _ = in_tail;
            }
            LcnfExpr::TailCall(func, args) => {
                if let LcnfArg::Var(vid) = func {
                    let var_name = format!("_x{}", vid.0);
                    result
                        .escape_sets
                        .insert(var_name, EscapeStatus::ArgumentEscape(0));
                }
                for (idx, arg) in args.iter().enumerate() {
                    if let LcnfArg::Var(vid) = arg {
                        let var_name = format!("_x{}", vid.0);
                        result
                            .escape_sets
                            .insert(var_name, EscapeStatus::ArgumentEscape(idx + 1));
                    }
                }
            }
            LcnfExpr::Unreachable => {}
        }
    }
    /// Analyze a let-binding value and record allocation sites.
    pub(super) fn analyze_let_value(
        &self,
        name: &str,
        value: &LcnfLetValue,
        result: &mut EscapeAnalysisResult,
    ) {
        match value {
            LcnfLetValue::Ctor(_, _, args) | LcnfLetValue::Reuse(_, _, _, args) => {
                let mut site = AllocationSite::new(name, &result.func_name);
                let obj_fields = args.iter().filter(|a| matches!(a, LcnfArg::Var(_))).count();
                site.size_estimate = (obj_fields as u64) * 8;
                site.set_status(EscapeStatus::NoEscape);
                result.allocations.push(site);
                result
                    .escape_sets
                    .insert(name.to_owned(), EscapeStatus::NoEscape);
                for (idx, arg) in args.iter().enumerate() {
                    if let LcnfArg::Var(vid) = arg {
                        let arg_name = format!("_x{}", vid.0);
                        result
                            .escape_sets
                            .entry(arg_name)
                            .or_insert(EscapeStatus::HeapEscape);
                        let _ = idx;
                    }
                }
            }
            LcnfLetValue::App(func, args) => {
                if let LcnfArg::Var(vid) = func {
                    let fn_name = format!("_x{}", vid.0);
                    result
                        .escape_sets
                        .entry(fn_name)
                        .or_insert(EscapeStatus::ArgumentEscape(0));
                }
                for (idx, arg) in args.iter().enumerate() {
                    if let LcnfArg::Var(vid) = arg {
                        let arg_name = format!("_x{}", vid.0);
                        result
                            .escape_sets
                            .insert(arg_name, EscapeStatus::ArgumentEscape(idx + 1));
                    }
                }
            }
            LcnfLetValue::Proj(_, _, _src_vid) => {
                result
                    .escape_sets
                    .insert(name.to_owned(), EscapeStatus::LocalEscape);
            }
            LcnfLetValue::FVar(vid) => {
                let src = format!("_x{}", vid.0);
                result
                    .escape_sets
                    .entry(src)
                    .or_insert(EscapeStatus::LocalEscape);
                result
                    .escape_sets
                    .insert(name.to_owned(), EscapeStatus::LocalEscape);
            }
            LcnfLetValue::Reset(_) => {
                result
                    .escape_sets
                    .insert(name.to_owned(), EscapeStatus::LocalEscape);
            }
            LcnfLetValue::Lit(_) | LcnfLetValue::Erased => {
                result
                    .escape_sets
                    .insert(name.to_owned(), EscapeStatus::NoEscape);
            }
        }
    }
    /// Propagate escape information: if an allocation's variable is found in
    /// the escape set with a heap-escaping status, upgrade the allocation site.
    pub fn propagate_escapes(&self, result: &mut EscapeAnalysisResult) {
        for site in &mut result.allocations {
            if let Some(status) = result.escape_sets.get(&site.var) {
                if status.is_heap_allocated() {
                    site.set_status(status.clone());
                }
            }
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FieldEscapeInfo {
    pub struct_type: String,
    pub field_name: String,
    pub escapes_via_return: bool,
    pub escapes_via_store: bool,
    pub escapes_via_call: bool,
}
impl FieldEscapeInfo {
    #[allow(dead_code)]
    pub fn new(struct_type: impl Into<String>, field: impl Into<String>) -> Self {
        FieldEscapeInfo {
            struct_type: struct_type.into(),
            field_name: field.into(),
            escapes_via_return: false,
            escapes_via_store: false,
            escapes_via_call: false,
        }
    }
    #[allow(dead_code)]
    pub fn escapes(&self) -> bool {
        self.escapes_via_return || self.escapes_via_store || self.escapes_via_call
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct OEDominatorTree {
    pub idom: Vec<Option<u32>>,
    pub dom_children: Vec<Vec<u32>>,
    pub dom_depth: Vec<u32>,
}
impl OEDominatorTree {
    #[allow(dead_code)]
    pub fn new(size: usize) -> Self {
        OEDominatorTree {
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
pub struct OEAnalysisCache {
    pub(super) entries: std::collections::HashMap<String, OECacheEntry>,
    pub(super) max_size: usize,
    pub(super) hits: u64,
    pub(super) misses: u64,
}
impl OEAnalysisCache {
    #[allow(dead_code)]
    pub fn new(max_size: usize) -> Self {
        OEAnalysisCache {
            entries: std::collections::HashMap::new(),
            max_size,
            hits: 0,
            misses: 0,
        }
    }
    #[allow(dead_code)]
    pub fn get(&mut self, key: &str) -> Option<&OECacheEntry> {
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
            OECacheEntry {
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
/// Statistics for OEX2 passes.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct OEX2PassStats {
    pub iterations: usize,
    pub changed: bool,
    pub nodes_visited: usize,
    pub nodes_modified: usize,
    pub time_ms: u64,
    pub memory_bytes: usize,
    pub errors: usize,
}
impl OEX2PassStats {
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
    pub fn merge(&mut self, o: &OEX2PassStats) {
        self.iterations += o.iterations;
        self.changed |= o.changed;
        self.nodes_visited += o.nodes_visited;
        self.nodes_modified += o.nodes_modified;
        self.time_ms += o.time_ms;
        self.memory_bytes = self.memory_bytes.max(o.memory_bytes);
        self.errors += o.errors;
    }
}
#[allow(dead_code)]
pub struct EscapeFlowGraph2 {
    pub nodes: Vec<u32>,
    pub edges: Vec<EscapeFlowEdge>,
    pub allocation_sites: std::collections::HashMap<u32, PointsToTarget>,
}
impl EscapeFlowGraph2 {
    #[allow(dead_code)]
    pub fn new() -> Self {
        EscapeFlowGraph2 {
            nodes: Vec::new(),
            edges: Vec::new(),
            allocation_sites: std::collections::HashMap::new(),
        }
    }
    #[allow(dead_code)]
    pub fn add_node(&mut self, id: u32) {
        if !self.nodes.contains(&id) {
            self.nodes.push(id);
        }
    }
    #[allow(dead_code)]
    pub fn add_edge(&mut self, from: u32, to: u32, kind: EscapeEdgeKind) {
        self.add_node(from);
        self.add_node(to);
        self.edges.push(EscapeFlowEdge {
            from,
            to,
            edge_kind: kind,
        });
    }
    #[allow(dead_code)]
    pub fn register_allocation(&mut self, node: u32, target: PointsToTarget) {
        self.allocation_sites.insert(node, target);
    }
    #[allow(dead_code)]
    pub fn successors(&self, node: u32) -> Vec<u32> {
        self.edges
            .iter()
            .filter(|e| e.from == node)
            .map(|e| e.to)
            .collect()
    }
    #[allow(dead_code)]
    pub fn predecessors(&self, node: u32) -> Vec<u32> {
        self.edges
            .iter()
            .filter(|e| e.to == node)
            .map(|e| e.from)
            .collect()
    }
    #[allow(dead_code)]
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }
    #[allow(dead_code)]
    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }
}
/// The result of escape analysis for a single function.
#[derive(Debug, Clone)]
pub struct EscapeAnalysisResult {
    /// Name of the analyzed function.
    pub func_name: String,
    /// All allocation sites found in the function.
    pub allocations: Vec<AllocationSite>,
    /// Per-variable escape status.
    pub escape_sets: HashMap<String, EscapeStatus>,
}
impl EscapeAnalysisResult {
    /// Create an empty result for `func`.
    pub fn new(func: impl Into<String>) -> Self {
        EscapeAnalysisResult {
            func_name: func.into(),
            allocations: Vec::new(),
            escape_sets: HashMap::new(),
        }
    }
    /// Number of allocations that escape.
    pub fn num_escaped(&self) -> usize {
        self.allocations
            .iter()
            .filter(|a| a.status.is_heap_allocated())
            .count()
    }
    /// Number of allocations eligible for stack allocation.
    pub fn num_stack_eligible(&self) -> usize {
        self.allocations
            .iter()
            .filter(|a| a.status.can_stack_allocate())
            .count()
    }
    /// Return references to allocations that can be stack-allocated.
    pub fn stack_allocation_candidates(&self) -> Vec<&AllocationSite> {
        self.allocations
            .iter()
            .filter(|a| a.status.can_stack_allocate())
            .collect()
    }
}
/// Statistics for OEExt passes.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct OEExtPassStats {
    pub iterations: usize,
    pub changed: bool,
    pub nodes_visited: usize,
    pub nodes_modified: usize,
    pub time_ms: u64,
    pub memory_bytes: usize,
    pub errors: usize,
}
impl OEExtPassStats {
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
    pub fn merge(&mut self, o: &OEExtPassStats) {
        self.iterations += o.iterations;
        self.changed |= o.changed;
        self.nodes_visited += o.nodes_visited;
        self.nodes_modified += o.nodes_modified;
        self.time_ms += o.time_ms;
        self.memory_bytes = self.memory_bytes.max(o.memory_bytes);
        self.errors += o.errors;
    }
}
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
    pub(super) idom: Vec<Option<usize>>,
    pub(super) children: Vec<Vec<usize>>,
    pub(super) depth: Vec<usize>,
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
    pub(super) nodes: Vec<u32>,
    pub(super) edges: Vec<(u32, u32)>,
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
    pub(super) edges: HashMap<String, Vec<String>>,
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
    pub(super) items: std::collections::VecDeque<usize>,
    pub(super) present: Vec<bool>,
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
    pub(super) idom: Vec<Option<usize>>,
    pub(super) children: Vec<Vec<usize>>,
    pub(super) depth: Vec<usize>,
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
    pub(super) items: std::collections::VecDeque<u32>,
    pub(super) in_worklist: std::collections::HashSet<u32>,
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
    pub(super) folds: usize,
    pub(super) failures: usize,
    pub(super) enabled: bool,
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
    pub(super) entries: Vec<(u64, Vec<u8>, bool, u32)>,
    pub(super) cap: usize,
    pub(super) total_hits: u64,
    pub(super) total_misses: u64,
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
    pub(super) configs: Vec<OEX2PassConfig>,
    pub(super) stats: Vec<OEX2PassStats>,
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
    pub(super) entries: Vec<(u64, Vec<u8>, bool, u32)>,
    pub(super) cap: usize,
    pub(super) total_hits: u64,
    pub(super) total_misses: u64,
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
    pub(super) escapees: HashSet<String>,
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
    pub(super) configs: Vec<OEPassConfig>,
    pub(super) stats: std::collections::HashMap<String, OEPassStats>,
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
    pub(super) folds: usize,
    pub(super) failures: usize,
    pub(super) enabled: bool,
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
    pub(super) items: std::collections::VecDeque<usize>,
    pub(super) present: Vec<bool>,
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
