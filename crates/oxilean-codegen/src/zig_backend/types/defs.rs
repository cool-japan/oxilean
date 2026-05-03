//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

#[allow(unused_imports)]
use super::impls::*;
use std::collections::{HashMap, HashSet, VecDeque};

#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct ZigPassStats {
    pub total_runs: u32,
    pub successful_runs: u32,
    pub total_changes: u64,
    pub time_ms: u64,
    pub iterations_used: u32,
}
impl ZigPassStats {
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
/// Pass registry for ZigExt.
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct ZigExtPassRegistry {
    pub(crate) configs: Vec<ZigExtPassConfig>,
    pub(crate) stats: Vec<ZigExtPassStats>,
}
impl ZigExtPassRegistry {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    #[allow(dead_code)]
    pub fn register(&mut self, c: ZigExtPassConfig) {
        self.stats.push(ZigExtPassStats::new());
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
    pub fn get(&self, i: usize) -> Option<&ZigExtPassConfig> {
        self.configs.get(i)
    }
    #[allow(dead_code)]
    pub fn get_stats(&self, i: usize) -> Option<&ZigExtPassStats> {
        self.stats.get(i)
    }
    #[allow(dead_code)]
    pub fn enabled_passes(&self) -> Vec<&ZigExtPassConfig> {
        self.configs.iter().filter(|c| c.enabled).collect()
    }
    #[allow(dead_code)]
    pub fn passes_in_phase(&self, ph: &ZigExtPassPhase) -> Vec<&ZigExtPassConfig> {
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
pub struct ZigErrorSet {
    pub name: String,
    pub errors: Vec<String>,
}
impl ZigErrorSet {
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>) -> Self {
        ZigErrorSet {
            name: name.into(),
            errors: Vec::new(),
        }
    }
    #[allow(dead_code)]
    pub fn add_error(&mut self, err: impl Into<String>) {
        self.errors.push(err.into());
    }
    #[allow(dead_code)]
    pub fn emit(&self) -> String {
        format!(
            "const {} = error{{ {} }};",
            self.name,
            self.errors.join(", ")
        )
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ZigAsyncFn {
    pub name: String,
    pub params: Vec<(String, String)>,
    pub return_type: String,
    pub frame_size: Option<usize>,
}
impl ZigAsyncFn {
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>) -> Self {
        ZigAsyncFn {
            name: name.into(),
            params: Vec::new(),
            return_type: "void".to_string(),
            frame_size: None,
        }
    }
    #[allow(dead_code)]
    pub fn param(mut self, name: impl Into<String>, ty: impl Into<String>) -> Self {
        self.params.push((name.into(), ty.into()));
        self
    }
    #[allow(dead_code)]
    pub fn emit(&self) -> String {
        let params: Vec<String> = self
            .params
            .iter()
            .map(|(n, t)| format!("{}: {}", n, t))
            .collect();
        format!(
            "async fn {}({}) {} {{\n    // async body\n}}",
            self.name,
            params.join(", "),
            self.return_type
        )
    }
    #[allow(dead_code)]
    pub fn emit_await_call(&self, args: &[&str]) -> String {
        format!("await async {}({})", self.name, args.join(", "))
    }
}
/// Pass execution phase for ZigExt.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ZigExtPassPhase {
    Early,
    Middle,
    Late,
    Finalize,
}
impl ZigExtPassPhase {
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
/// Worklist for ZigExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ZigExtWorklist {
    pub(crate) items: std::collections::VecDeque<usize>,
    pub(crate) present: Vec<bool>,
}
impl ZigExtWorklist {
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
struct ZigStructField2 {
    pub name: String,
    pub ty_name: String,
}
#[allow(dead_code)]
pub struct ZigBuildConfiguration {
    pub target_os: String,
    pub target_arch: String,
    pub optimize_mode: ZigOptimizeMode,
    pub root_source_file: String,
    pub dependencies: Vec<String>,
}
impl ZigBuildConfiguration {
    #[allow(dead_code)]
    pub fn new(root: impl Into<String>) -> Self {
        ZigBuildConfiguration {
            target_os: "native".to_string(),
            target_arch: "native".to_string(),
            optimize_mode: ZigOptimizeMode::Debug,
            root_source_file: root.into(),
            dependencies: Vec::new(),
        }
    }
    #[allow(dead_code)]
    pub fn optimize(mut self, mode: ZigOptimizeMode) -> Self {
        self.optimize_mode = mode;
        self
    }
    #[allow(dead_code)]
    pub fn dep(mut self, name: impl Into<String>) -> Self {
        self.dependencies.push(name.into());
        self
    }
    #[allow(dead_code)]
    pub fn emit_build_zig(&self) -> String {
        let mut out = String::from("const std = @import(\"std\");\n\n");
        out.push_str("pub fn build(b: *std.Build) void {\n");
        out.push_str(&format!(
            "    const target = b.standardTargetOptions(.{{}});\n"
        ));
        out.push_str(&format!(
            "    const optimize = b.standardOptimizeOption(.{{}});\n"
        ));
        out.push_str(
            &format!(
                "    const exe = b.addExecutable(.{{\n        .name = \"app\",\n        .root_source_file = .{{ .path = \"{}\" }},\n        .target = target,\n        .optimize = optimize,\n    }});\n",
                self.root_source_file
            ),
        );
        out.push_str("    b.installArtifact(exe);\n");
        out.push('}');
        out
    }
}
/// Represents a Zig function definition.
pub struct ZigFn {
    pub name: String,
    pub params: Vec<(String, ZigType)>,
    pub ret_ty: ZigType,
    pub body: Vec<ZigStmt>,
    pub is_pub: bool,
    pub is_async: bool,
}
impl ZigFn {
    /// Create a new function with the given name and return type.
    pub fn new(name: &str, ret: ZigType) -> Self {
        ZigFn {
            name: name.to_string(),
            params: Vec::new(),
            ret_ty: ret,
            body: Vec::new(),
            is_pub: false,
            is_async: false,
        }
    }
    /// Add a parameter to the function.
    pub fn add_param(&mut self, name: &str, ty: ZigType) {
        self.params.push((name.to_string(), ty));
    }
    /// Add a statement to the function body.
    pub fn add_stmt(&mut self, stmt: ZigStmt) {
        self.body.push(stmt);
    }
    /// Generate Zig source code for this function.
    pub fn codegen(&self) -> String {
        let pub_str = if self.is_pub { "pub " } else { "" };
        let async_str = if self.is_async { "async " } else { "" };
        let params_str: Vec<String> = self
            .params
            .iter()
            .map(|(name, ty)| format!("{}: {}", name, ty.codegen()))
            .collect();
        let body_str: Vec<String> = self
            .body
            .iter()
            .map(|s| format!("    {}", s.codegen()))
            .collect();
        format!(
            "{}{}fn {}({}) {} {{\n{}\n}}",
            pub_str,
            async_str,
            self.name,
            params_str.join(", "),
            self.ret_ty.codegen(),
            body_str.join("\n")
        )
    }
}
#[allow(dead_code)]
pub struct ZigConstantFoldingHelper;
impl ZigConstantFoldingHelper {
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
/// Analysis cache for ZigExt.
#[allow(dead_code)]
#[derive(Debug)]
pub struct ZigExtCache {
    pub(crate) entries: Vec<(u64, Vec<u8>, bool, u32)>,
    pub(crate) cap: usize,
    pub(crate) total_hits: u64,
    pub(crate) total_misses: u64,
}
impl ZigExtCache {
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
#[derive(Debug, Clone)]
pub struct ZigWorklist {
    pub(crate) items: std::collections::VecDeque<u32>,
    pub(crate) in_worklist: std::collections::HashSet<u32>,
}
impl ZigWorklist {
    #[allow(dead_code)]
    pub fn new() -> Self {
        ZigWorklist {
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
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ZigPassConfig {
    pub phase: ZigPassPhase,
    pub enabled: bool,
    pub max_iterations: u32,
    pub debug_output: bool,
    pub pass_name: String,
}
impl ZigPassConfig {
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>, phase: ZigPassPhase) -> Self {
        ZigPassConfig {
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
/// Dependency graph for ZigExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ZigExtDepGraph {
    pub(crate) n: usize,
    pub(crate) adj: Vec<Vec<usize>>,
    pub(crate) rev: Vec<Vec<usize>>,
    pub(crate) edge_count: usize,
}
impl ZigExtDepGraph {
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
/// Zig statement representation.
#[derive(Debug, Clone, PartialEq)]
pub enum ZigStmt {
    VarDecl {
        name: String,
        ty: Option<ZigType>,
        value: ZigExpr,
    },
    ConstDecl {
        name: String,
        ty: Option<ZigType>,
        value: ZigExpr,
    },
    Assign {
        target: ZigExpr,
        value: ZigExpr,
    },
    Return(Option<ZigExpr>),
    Expr(ZigExpr),
    If {
        cond: ZigExpr,
        body: Vec<ZigStmt>,
        else_: Vec<ZigStmt>,
    },
    While {
        cond: ZigExpr,
        body: Vec<ZigStmt>,
    },
    Defer(ZigExpr),
}
impl ZigStmt {
    /// Emit Zig source for this statement.
    pub fn codegen(&self) -> String {
        match self {
            ZigStmt::VarDecl { name, ty, value } => {
                let ty_str = ty
                    .as_ref()
                    .map(|t| format!(": {}", t.codegen()))
                    .unwrap_or_default();
                format!("var {}{} = {};", name, ty_str, value.codegen())
            }
            ZigStmt::ConstDecl { name, ty, value } => {
                let ty_str = ty
                    .as_ref()
                    .map(|t| format!(": {}", t.codegen()))
                    .unwrap_or_default();
                format!("const {}{} = {};", name, ty_str, value.codegen())
            }
            ZigStmt::Assign { target, value } => {
                format!("{} = {};", target.codegen(), value.codegen())
            }
            ZigStmt::Return(None) => "return;".to_string(),
            ZigStmt::Return(Some(expr)) => format!("return {};", expr.codegen()),
            ZigStmt::Expr(expr) => format!("{};", expr.codegen()),
            ZigStmt::If { cond, body, else_ } => {
                let body_str: Vec<String> = body
                    .iter()
                    .map(|s| format!("    {}", s.codegen()))
                    .collect();
                let else_str = if else_.is_empty() {
                    String::new()
                } else {
                    let else_body: Vec<String> = else_
                        .iter()
                        .map(|s| format!("    {}", s.codegen()))
                        .collect();
                    format!(" else {{\n{}\n}}", else_body.join("\n"))
                };
                format!(
                    "if ({}) {{\n{}\n}}{}",
                    cond.codegen(),
                    body_str.join("\n"),
                    else_str
                )
            }
            ZigStmt::While { cond, body } => {
                let body_str: Vec<String> = body
                    .iter()
                    .map(|s| format!("    {}", s.codegen()))
                    .collect();
                format!("while ({}) {{\n{}\n}}", cond.codegen(), body_str.join("\n"))
            }
            ZigStmt::Defer(expr) => format!("defer {};", expr.codegen()),
        }
    }
}
/// Constant folding helper for ZigExt.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct ZigExtConstFolder {
    pub(crate) folds: usize,
    pub(crate) failures: usize,
    pub(crate) enabled: bool,
}
impl ZigExtConstFolder {
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
#[derive(Debug, Clone)]
pub struct ZigLivenessInfo {
    pub live_in: Vec<std::collections::HashSet<u32>>,
    pub live_out: Vec<std::collections::HashSet<u32>>,
    pub defs: Vec<std::collections::HashSet<u32>>,
    pub uses: Vec<std::collections::HashSet<u32>>,
}
impl ZigLivenessInfo {
    #[allow(dead_code)]
    pub fn new(block_count: usize) -> Self {
        ZigLivenessInfo {
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
pub struct ZigFile {
    pub imports: Vec<ZigImport>,
    pub structs: Vec<ZigStruct>,
    pub functions: Vec<ZigFn>,
    pub tests: Vec<ZigTestBlock>,
}
impl ZigFile {
    #[allow(dead_code)]
    pub fn new() -> Self {
        ZigFile {
            imports: Vec::new(),
            structs: Vec::new(),
            functions: Vec::new(),
            tests: Vec::new(),
        }
    }
    #[allow(dead_code)]
    pub fn import(mut self, imp: ZigImport) -> Self {
        self.imports.push(imp);
        self
    }
    #[allow(dead_code)]
    pub fn add_struct(mut self, s: ZigStruct) -> Self {
        self.structs.push(s);
        self
    }
    #[allow(dead_code)]
    pub fn add_fn(mut self, f: ZigFn) -> Self {
        self.functions.push(f);
        self
    }
    #[allow(dead_code)]
    pub fn add_test(mut self, t: ZigTestBlock) -> Self {
        self.tests.push(t);
        self
    }
    #[allow(dead_code)]
    pub fn emit(&self) -> String {
        let mut parts = Vec::new();
        for imp in &self.imports {
            parts.push(imp.emit());
        }
        for s in &self.structs {
            let mut out = format!("const {} = struct {{\n", s.name);
            for f in &s.fields {
                out.push_str(&format!("    {}: {},\n", f.0, f.1.codegen()));
            }
            out.push_str("};");
            parts.push(out);
        }
        for t in &self.tests {
            parts.push(t.emit());
        }
        parts.join("\n\n")
    }
}
#[allow(dead_code)]
pub struct ZigSliceOps;
impl ZigSliceOps {
    #[allow(dead_code)]
    pub fn len_expr(slice: &str) -> String {
        format!("{}.len", slice)
    }
    #[allow(dead_code)]
    pub fn ptr_expr(slice: &str) -> String {
        format!("{}.ptr", slice)
    }
    #[allow(dead_code)]
    pub fn index_expr(slice: &str, idx: &str) -> String {
        format!("{}[{}]", slice, idx)
    }
    #[allow(dead_code)]
    pub fn slice_expr(slice: &str, start: &str, end: &str) -> String {
        format!("{}[{}..{}]", slice, start, end)
    }
    #[allow(dead_code)]
    pub fn concat_alloc(alloc: &str, a: &str, b: &str) -> String {
        format!("try std.mem.concat({}, u8, &.{{ {}, {} }})", alloc, a, b)
    }
    #[allow(dead_code)]
    pub fn copy(dst: &str, src: &str) -> String {
        format!("std.mem.copy(u8, {}, {})", dst, src)
    }
    #[allow(dead_code)]
    pub fn eql(a: &str, b: &str) -> String {
        format!("std.mem.eql(u8, {}, {})", a, b)
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ZigPackedStruct {
    pub name: String,
    pub fields: Vec<ZigPackedField>,
}
impl ZigPackedStruct {
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>) -> Self {
        ZigPackedStruct {
            name: name.into(),
            fields: Vec::new(),
        }
    }
    #[allow(dead_code)]
    pub fn add_field(&mut self, name: impl Into<String>, ty: ZigType, bits: Option<u32>) {
        self.fields.push(ZigPackedField {
            name: name.into(),
            ty,
            bit_width: bits,
        });
    }
    #[allow(dead_code)]
    pub fn emit(&self) -> String {
        let mut out = format!("const {} = packed struct {{\n", self.name);
        for f in &self.fields {
            match f.bit_width {
                Some(bits) => out.push_str(&format!("    {}: u{},\n", f.name, bits)),
                None => out.push_str(&format!("    {}: {},\n", f.name, "type")),
            }
        }
        out.push_str("};");
        out
    }
    #[allow(dead_code)]
    pub fn total_bits(&self) -> u32 {
        self.fields.iter().map(|f| f.bit_width.unwrap_or(8)).sum()
    }
}
