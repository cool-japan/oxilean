//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::impls1::*;
use super::impls2::*;
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
    pub(crate) n: usize,
    pub(crate) adj: Vec<Vec<usize>>,
    pub(crate) rev: Vec<Vec<usize>>,
    pub(crate) edge_count: usize,
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
    pub(crate) n: usize,
    pub(crate) adj: Vec<Vec<usize>>,
    pub(crate) rev: Vec<Vec<usize>>,
    pub(crate) edge_count: usize,
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
    pub(crate) configs: Vec<OEExtPassConfig>,
    pub(crate) stats: Vec<OEExtPassStats>,
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
    pub(crate) fn analyze_let_value(
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
    pub(crate) entries: std::collections::HashMap<String, OECacheEntry>,
    pub(crate) max_size: usize,
    pub(crate) hits: u64,
    pub(crate) misses: u64,
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
