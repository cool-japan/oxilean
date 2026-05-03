//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::defs::*;
use super::impls1::*;
use std::collections::HashMap;

use std::collections::{HashSet, VecDeque};

impl ChapelParam {
    /// Simple parameter: `name: type`
    pub fn simple(name: impl Into<String>, ty: ChapelType) -> Self {
        ChapelParam {
            name: name.into(),
            ty: Some(ty),
            intent: None,
            default: None,
        }
    }
    /// Parameter with intent: `intent name: type`
    pub fn with_intent(name: impl Into<String>, ty: ChapelType, intent: ChapelIntent) -> Self {
        ChapelParam {
            name: name.into(),
            ty: Some(ty),
            intent: Some(intent),
            default: None,
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct ChplPassStats {
    pub total_runs: u32,
    pub successful_runs: u32,
    pub total_changes: u64,
    pub time_ms: u64,
    pub iterations_used: u32,
}
impl ChplPassStats {
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
/// Liveness analysis for ChapelExt.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct ChapelExtLiveness {
    pub live_in: Vec<Vec<usize>>,
    pub live_out: Vec<Vec<usize>>,
    pub defs: Vec<Vec<usize>>,
    pub uses: Vec<Vec<usize>>,
}
impl ChapelExtLiveness {
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
/// A Chapel procedure (function).
#[derive(Debug, Clone)]
pub struct ChapelProc {
    /// Procedure name
    pub name: String,
    /// Parameters
    pub params: Vec<ChapelParam>,
    /// Return type (None = void)
    pub return_type: Option<ChapelType>,
    /// Body statements
    pub body: Vec<ChapelStmt>,
    /// Whether this is a parallel iterator (`iter` keyword)
    pub is_iter: bool,
    /// Whether the proc is `inline`
    pub is_inline: bool,
    /// Whether the proc is `override`
    pub is_override: bool,
    /// Whether the proc is `operator`
    pub is_operator: bool,
    /// `where` clause expression
    pub where_clause: Option<String>,
}
impl ChapelProc {
    /// Create a simple procedure.
    pub fn new(
        name: impl Into<String>,
        params: Vec<ChapelParam>,
        return_type: Option<ChapelType>,
        body: Vec<ChapelStmt>,
    ) -> Self {
        ChapelProc {
            name: name.into(),
            params,
            return_type,
            body,
            is_iter: false,
            is_inline: false,
            is_override: false,
            is_operator: false,
            where_clause: None,
        }
    }
    /// Mark as parallel iterator.
    pub fn as_iter(mut self) -> Self {
        self.is_iter = true;
        self
    }
    /// Mark as inline.
    pub fn as_inline(mut self) -> Self {
        self.is_inline = true;
        self
    }
    /// Add a `where` clause.
    pub fn with_where(mut self, clause: impl Into<String>) -> Self {
        self.where_clause = Some(clause.into());
        self
    }
}
/// A field/member of a record or class.
#[derive(Debug, Clone)]
pub struct ChapelField {
    /// Field name
    pub name: String,
    /// Field type
    pub ty: ChapelType,
    /// Whether the field is `var` or `const`
    pub is_const: bool,
    /// Optional default value
    pub default: Option<ChapelExpr>,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ChplCacheEntry {
    pub key: String,
    pub data: Vec<u8>,
    pub timestamp: u64,
    pub valid: bool,
}
/// Dominator tree for ChapelExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ChapelExtDomTree {
    pub(crate) idom: Vec<Option<usize>>,
    pub(crate) children: Vec<Vec<usize>>,
    pub(crate) depth: Vec<usize>,
}
impl ChapelExtDomTree {
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
#[derive(Debug, Clone)]
pub struct ChplWorklist {
    pub(crate) items: std::collections::VecDeque<u32>,
    pub(crate) in_worklist: std::collections::HashSet<u32>,
}
impl ChplWorklist {
    #[allow(dead_code)]
    pub fn new() -> Self {
        ChplWorklist {
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
/// Statistics for ChapelExt passes.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct ChapelExtPassStats {
    pub iterations: usize,
    pub changed: bool,
    pub nodes_visited: usize,
    pub nodes_modified: usize,
    pub time_ms: u64,
    pub memory_bytes: usize,
    pub errors: usize,
}
impl ChapelExtPassStats {
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
    pub fn merge(&mut self, o: &ChapelExtPassStats) {
        self.iterations += o.iterations;
        self.changed |= o.changed;
        self.nodes_visited += o.nodes_visited;
        self.nodes_modified += o.nodes_modified;
        self.time_ms += o.time_ms;
        self.memory_bytes = self.memory_bytes.max(o.memory_bytes);
        self.errors += o.errors;
    }
}
/// A Chapel class definition.
#[derive(Debug, Clone)]
pub struct ChapelClass {
    /// Class name
    pub name: String,
    /// Optional parent class
    pub parent: Option<String>,
    /// Fields
    pub fields: Vec<ChapelField>,
    /// Methods
    pub methods: Vec<ChapelProc>,
    /// Optional generic type parameters
    pub type_params: Vec<String>,
}
impl ChapelClass {
    /// Create an empty class.
    pub fn new(name: impl Into<String>) -> Self {
        ChapelClass {
            name: name.into(),
            parent: None,
            fields: vec![],
            methods: vec![],
            type_params: vec![],
        }
    }
    /// Set the parent class.
    pub fn with_parent(mut self, parent: impl Into<String>) -> Self {
        self.parent = Some(parent.into());
        self
    }
    /// Add a field.
    pub fn add_field(&mut self, field: ChapelField) {
        self.fields.push(field);
    }
    /// Add a method.
    pub fn add_method(&mut self, method: ChapelProc) {
        self.methods.push(method);
    }
}
/// Configuration for ChapelExt passes.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ChapelExtPassConfig {
    pub name: String,
    pub phase: ChapelExtPassPhase,
    pub enabled: bool,
    pub max_iterations: usize,
    pub debug: u32,
    pub timeout_ms: Option<u64>,
}
impl ChapelExtPassConfig {
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            phase: ChapelExtPassPhase::Middle,
            enabled: true,
            max_iterations: 100,
            debug: 0,
            timeout_ms: None,
        }
    }
    #[allow(dead_code)]
    pub fn with_phase(mut self, phase: ChapelExtPassPhase) -> Self {
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
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum ChplPassPhase {
    Analysis,
    Transformation,
    Verification,
    Cleanup,
}
impl ChplPassPhase {
    #[allow(dead_code)]
    pub fn name(&self) -> &str {
        match self {
            ChplPassPhase::Analysis => "analysis",
            ChplPassPhase::Transformation => "transformation",
            ChplPassPhase::Verification => "verification",
            ChplPassPhase::Cleanup => "cleanup",
        }
    }
    #[allow(dead_code)]
    pub fn is_modifying(&self) -> bool {
        matches!(self, ChplPassPhase::Transformation | ChplPassPhase::Cleanup)
    }
}
/// A Chapel source module (single `.chpl` file).
#[derive(Debug, Clone)]
pub struct ChapelModule {
    /// Module name (None = implicit top-level)
    pub name: Option<String>,
    /// `use` imports
    pub uses: Vec<String>,
    /// `require` directives
    pub requires: Vec<String>,
    /// Top-level constant declarations
    pub globals: Vec<(String, ChapelType, ChapelExpr)>,
    /// Top-level config variable declarations
    pub configs: Vec<(String, ChapelType, Option<ChapelExpr>)>,
    /// Top-level procedures
    pub procs: Vec<ChapelProc>,
    /// Top-level record definitions
    pub records: Vec<ChapelRecord>,
    /// Top-level class definitions
    pub classes: Vec<ChapelClass>,
    /// Sub-modules
    pub submodules: Vec<ChapelModule>,
    /// Module-level doc comment
    pub doc: Option<String>,
}
impl ChapelModule {
    /// Create an empty unnamed module.
    pub fn new() -> Self {
        ChapelModule {
            name: None,
            uses: vec![],
            requires: vec![],
            globals: vec![],
            configs: vec![],
            procs: vec![],
            records: vec![],
            classes: vec![],
            submodules: vec![],
            doc: None,
        }
    }
    /// Create a named module.
    pub fn named(name: impl Into<String>) -> Self {
        let mut m = ChapelModule::new();
        m.name = Some(name.into());
        m
    }
    /// Add a `use` directive.
    pub fn add_use(&mut self, name: impl Into<String>) {
        self.uses.push(name.into());
    }
    /// Add a `require` directive.
    pub fn add_require(&mut self, path: impl Into<String>) {
        self.requires.push(path.into());
    }
    /// Add a top-level constant.
    pub fn add_global(&mut self, name: impl Into<String>, ty: ChapelType, expr: ChapelExpr) {
        self.globals.push((name.into(), ty, expr));
    }
    /// Add a config variable.
    pub fn add_config(
        &mut self,
        name: impl Into<String>,
        ty: ChapelType,
        default: Option<ChapelExpr>,
    ) {
        self.configs.push((name.into(), ty, default));
    }
    /// Add a procedure.
    pub fn add_proc(&mut self, proc: ChapelProc) {
        self.procs.push(proc);
    }
    /// Add a record.
    pub fn add_record(&mut self, rec: ChapelRecord) {
        self.records.push(rec);
    }
    /// Add a class.
    pub fn add_class(&mut self, cls: ChapelClass) {
        self.classes.push(cls);
    }
    /// Set the doc comment.
    pub fn set_doc(&mut self, doc: impl Into<String>) {
        self.doc = Some(doc.into());
    }
}
/// Worklist for ChapelExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ChapelExtWorklist {
    pub(crate) items: std::collections::VecDeque<usize>,
    pub(crate) present: Vec<bool>,
}
impl ChapelExtWorklist {
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
/// Dependency graph for ChapelExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ChapelExtDepGraph {
    pub(crate) n: usize,
    pub(crate) adj: Vec<Vec<usize>>,
    pub(crate) rev: Vec<Vec<usize>>,
    pub(crate) edge_count: usize,
}
impl ChapelExtDepGraph {
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
/// Pass registry for ChapelExt.
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct ChapelExtPassRegistry {
    pub(crate) configs: Vec<ChapelExtPassConfig>,
    pub(crate) stats: Vec<ChapelExtPassStats>,
}
impl ChapelExtPassRegistry {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    #[allow(dead_code)]
    pub fn register(&mut self, c: ChapelExtPassConfig) {
        self.stats.push(ChapelExtPassStats::new());
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
    pub fn get(&self, i: usize) -> Option<&ChapelExtPassConfig> {
        self.configs.get(i)
    }
    #[allow(dead_code)]
    pub fn get_stats(&self, i: usize) -> Option<&ChapelExtPassStats> {
        self.stats.get(i)
    }
    #[allow(dead_code)]
    pub fn enabled_passes(&self) -> Vec<&ChapelExtPassConfig> {
        self.configs.iter().filter(|c| c.enabled).collect()
    }
    #[allow(dead_code)]
    pub fn passes_in_phase(&self, ph: &ChapelExtPassPhase) -> Vec<&ChapelExtPassConfig> {
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
