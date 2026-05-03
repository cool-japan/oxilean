//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::defs::*;
use super::impls2::*;
use std::collections::{HashMap, HashSet, VecDeque};

/// A field inside a Metal struct.
/// A feature flag set for MetalExt capabilities.
#[derive(Debug, Clone, Default)]
pub struct MetalExtFeatures {
    pub(crate) flags: std::collections::HashSet<String>,
}
impl MetalExtFeatures {
    pub fn new() -> Self {
        MetalExtFeatures::default()
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
    pub fn union(&self, other: &MetalExtFeatures) -> MetalExtFeatures {
        MetalExtFeatures {
            flags: self.flags.union(&other.flags).cloned().collect(),
        }
    }
    pub fn intersection(&self, other: &MetalExtFeatures) -> MetalExtFeatures {
        MetalExtFeatures {
            flags: self.flags.intersection(&other.flags).cloned().collect(),
        }
    }
}
#[allow(dead_code)]
pub struct MetalConstantFoldingHelper;
impl MetalConstantFoldingHelper {
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
/// Metal Shading Language type system.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MetalType {
    /// `bool`
    Bool,
    /// `half`  (16-bit float)
    Half,
    /// `float` (32-bit float)
    Float,
    /// `int`   (32-bit signed)
    Int,
    /// `uint`  (32-bit unsigned)
    Uint,
    /// `short` (16-bit signed)
    Short,
    /// `ushort` (16-bit unsigned)
    Ushort,
    /// `char`  (8-bit signed)
    Char,
    /// `uchar` (8-bit unsigned)
    Uchar,
    /// `long`  (64-bit signed)
    Long,
    /// `ulong` (64-bit unsigned)
    Ulong,
    /// `float2`
    Float2,
    /// `float3`
    Float3,
    /// `float4`
    Float4,
    /// `half2`
    Half2,
    /// `half3`
    Half3,
    /// `half4`
    Half4,
    /// `int2`
    Int2,
    /// `int3`
    Int3,
    /// `int4`
    Int4,
    /// `uint2`
    Uint2,
    /// `uint3`
    Uint3,
    /// `uint4`
    Uint4,
    /// `float2x2`
    Float2x2,
    /// `float3x3`
    Float3x3,
    /// `float4x4`
    Float4x4,
    /// Fixed-size array: `T[N]`
    Array(Box<MetalType>, usize),
    /// Named struct or typedef
    Struct(String),
    /// `texture2d<float>` etc — simplified to `texture<element_type>`
    Texture(Box<MetalType>),
    /// `sampler`
    Sampler,
    /// Pointer to inner type with an address space
    Pointer(Box<MetalType>, MetalAddressSpace),
    /// `void`
    Void,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct MetalLivenessInfo {
    pub live_in: Vec<std::collections::HashSet<u32>>,
    pub live_out: Vec<std::collections::HashSet<u32>>,
    pub defs: Vec<std::collections::HashSet<u32>>,
    pub uses: Vec<std::collections::HashSet<u32>>,
}
impl MetalLivenessInfo {
    #[allow(dead_code)]
    pub fn new(block_count: usize) -> Self {
        MetalLivenessInfo {
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
/// Dependency graph for MetalExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct MetalExtDepGraph {
    pub(crate) n: usize,
    pub(crate) adj: Vec<Vec<usize>>,
    pub(crate) rev: Vec<Vec<usize>>,
    pub(crate) edge_count: usize,
}
impl MetalExtDepGraph {
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
/// A version tag for MetalExt output artifacts.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MetalExtVersion {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
    pub pre: Option<String>,
}
impl MetalExtVersion {
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        MetalExtVersion {
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
    pub fn is_compatible_with(&self, other: &MetalExtVersion) -> bool {
        self.major == other.major && self.minor >= other.minor
    }
}
/// A text buffer for building MetalExt output source code.
#[derive(Debug, Default)]
pub struct MetalExtSourceBuffer {
    pub(crate) buf: String,
    pub(crate) indent_level: usize,
    pub(crate) indent_str: String,
}
impl MetalExtSourceBuffer {
    pub fn new() -> Self {
        MetalExtSourceBuffer {
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
/// Liveness analysis for MetalExt.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct MetalExtLiveness {
    pub live_in: Vec<Vec<usize>>,
    pub live_out: Vec<Vec<usize>>,
    pub defs: Vec<Vec<usize>>,
    pub uses: Vec<Vec<usize>>,
}
impl MetalExtLiveness {
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
/// Collects MetalExt diagnostics.
#[derive(Debug, Default)]
pub struct MetalExtDiagCollector {
    pub(crate) msgs: Vec<MetalExtDiagMsg>,
}
impl MetalExtDiagCollector {
    pub fn new() -> Self {
        MetalExtDiagCollector::default()
    }
    pub fn emit(&mut self, d: MetalExtDiagMsg) {
        self.msgs.push(d);
    }
    pub fn has_errors(&self) -> bool {
        self.msgs
            .iter()
            .any(|d| d.severity == MetalExtDiagSeverity::Error)
    }
    pub fn errors(&self) -> Vec<&MetalExtDiagMsg> {
        self.msgs
            .iter()
            .filter(|d| d.severity == MetalExtDiagSeverity::Error)
            .collect()
    }
    pub fn warnings(&self) -> Vec<&MetalExtDiagMsg> {
        self.msgs
            .iter()
            .filter(|d| d.severity == MetalExtDiagSeverity::Warning)
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
/// Heuristic freshness key for MetalExt incremental compilation.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MetalExtIncrKey {
    pub content_hash: u64,
    pub config_hash: u64,
}
impl MetalExtIncrKey {
    pub fn new(content: u64, config: u64) -> Self {
        MetalExtIncrKey {
            content_hash: content,
            config_hash: config,
        }
    }
    pub fn combined_hash(&self) -> u64 {
        self.content_hash.wrapping_mul(0x9e3779b97f4a7c15) ^ self.config_hash
    }
    pub fn matches(&self, other: &MetalExtIncrKey) -> bool {
        self.content_hash == other.content_hash && self.config_hash == other.config_hash
    }
}
#[allow(dead_code)]
pub struct MetalPassRegistry {
    pub(crate) configs: Vec<MetalPassConfig>,
    pub(crate) stats: std::collections::HashMap<String, MetalPassStats>,
}
impl MetalPassRegistry {
    #[allow(dead_code)]
    pub fn new() -> Self {
        MetalPassRegistry {
            configs: Vec::new(),
            stats: std::collections::HashMap::new(),
        }
    }
    #[allow(dead_code)]
    pub fn register(&mut self, config: MetalPassConfig) {
        self.stats
            .insert(config.pass_name.clone(), MetalPassStats::new());
        self.configs.push(config);
    }
    #[allow(dead_code)]
    pub fn enabled_passes(&self) -> Vec<&MetalPassConfig> {
        self.configs.iter().filter(|c| c.enabled).collect()
    }
    #[allow(dead_code)]
    pub fn get_stats(&self, name: &str) -> Option<&MetalPassStats> {
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
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct MetalDominatorTree {
    pub idom: Vec<Option<u32>>,
    pub dom_children: Vec<Vec<u32>>,
    pub dom_depth: Vec<u32>,
}
impl MetalDominatorTree {
    #[allow(dead_code)]
    pub fn new(size: usize) -> Self {
        MetalDominatorTree {
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
pub struct MetalPassConfig {
    pub phase: MetalPassPhase,
    pub enabled: bool,
    pub max_iterations: u32,
    pub debug_output: bool,
    pub pass_name: String,
}
impl MetalPassConfig {
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>, phase: MetalPassPhase) -> Self {
        MetalPassConfig {
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
/// Emitter state for producing Metal Shading Language `.metal` source files.
pub struct MetalBackend {
    pub(crate) indent_width: usize,
}
impl MetalBackend {
    /// Create a new backend with 4-space indentation.
    pub fn new() -> Self {
        MetalBackend { indent_width: 4 }
    }
    /// Create a backend with a custom indent width.
    pub fn with_indent(indent_width: usize) -> Self {
        MetalBackend { indent_width }
    }
    pub(crate) fn indent(&self, depth: usize) -> String {
        " ".repeat(self.indent_width * depth)
    }
    /// Emit a Metal expression to a string.
    pub fn emit_expr(&self, expr: &MetalExpr) -> String {
        expr.emit()
    }
    /// Emit a single statement at the given indentation depth.
    pub fn emit_stmt(&self, stmt: &MetalStmt, depth: usize) -> String {
        let ind = self.indent(depth);
        match stmt {
            MetalStmt::VarDecl {
                ty,
                name,
                init,
                is_const,
            } => {
                let const_kw = if *is_const { "const " } else { "" };
                match init {
                    Some(expr) => {
                        format!("{}{}{} {} = {};", ind, const_kw, ty, name, expr.emit())
                    }
                    None => format!("{}{}{} {};", ind, const_kw, ty, name),
                }
            }
            MetalStmt::Assign { lhs, rhs } => {
                format!("{}{} = {};", ind, lhs.emit(), rhs.emit())
            }
            MetalStmt::CompoundAssign { lhs, op, rhs } => {
                format!("{}{} {}= {};", ind, lhs.emit(), op, rhs.emit())
            }
            MetalStmt::IfElse {
                cond,
                then_body,
                else_body,
            } => self.emit_if_else(cond, then_body, else_body.as_deref(), depth),
            MetalStmt::ForLoop {
                init,
                cond,
                step,
                body,
            } => self.emit_for_loop(init, cond, step, body, depth),
            MetalStmt::WhileLoop { cond, body } => self.emit_while(cond, body, depth),
            MetalStmt::Return(Some(expr)) => format!("{}return {};", ind, expr.emit()),
            MetalStmt::Return(None) => format!("{}return;", ind),
            MetalStmt::Expr(expr) => format!("{}{};", ind, expr.emit()),
            MetalStmt::Barrier(flags) => {
                format!("{}threadgroup_barrier({});", ind, flags)
            }
            MetalStmt::Block(stmts) => {
                let mut out = format!("{}{{\n", ind);
                for s in stmts {
                    out.push_str(&self.emit_stmt(s, depth + 1));
                    out.push('\n');
                }
                out.push_str(&format!("{}}}", ind));
                out
            }
            MetalStmt::Break => format!("{}break;", ind),
            MetalStmt::Continue => format!("{}continue;", ind),
        }
    }
    pub(crate) fn emit_if_else(
        &self,
        cond: &MetalExpr,
        then_body: &[MetalStmt],
        else_body: Option<&[MetalStmt]>,
        depth: usize,
    ) -> String {
        let ind = self.indent(depth);
        let mut out = format!("{}if ({}) {{\n", ind, cond.emit());
        for s in then_body {
            out.push_str(&self.emit_stmt(s, depth + 1));
            out.push('\n');
        }
        out.push_str(&format!("{}}}", ind));
        if let Some(eb) = else_body {
            out.push_str(" else {\n");
            for s in eb {
                out.push_str(&self.emit_stmt(s, depth + 1));
                out.push('\n');
            }
            out.push_str(&format!("{}}}", ind));
        }
        out
    }
    pub(crate) fn emit_for_loop(
        &self,
        init: &MetalStmt,
        cond: &MetalExpr,
        step: &MetalExpr,
        body: &[MetalStmt],
        depth: usize,
    ) -> String {
        let ind = self.indent(depth);
        let init_str = self.emit_stmt(init, 0).trim().to_string();
        let init_header = init_str.trim_end_matches(';');
        let mut out = format!(
            "{}for ({}; {}; {}) {{\n",
            ind,
            init_header,
            cond.emit(),
            step.emit()
        );
        for s in body {
            out.push_str(&self.emit_stmt(s, depth + 1));
            out.push('\n');
        }
        out.push_str(&format!("{}}}", ind));
        out
    }
    pub(crate) fn emit_while(&self, cond: &MetalExpr, body: &[MetalStmt], depth: usize) -> String {
        let ind = self.indent(depth);
        let mut out = format!("{}while ({}) {{\n", ind, cond.emit());
        for s in body {
            out.push_str(&self.emit_stmt(s, depth + 1));
            out.push('\n');
        }
        out.push_str(&format!("{}}}", ind));
        out
    }
    pub(crate) fn emit_struct(&self, s: &MetalStruct) -> String {
        let mut out = format!("struct {} {{\n", s.name);
        for field in &s.fields {
            out.push_str(&field.emit());
            out.push('\n');
        }
        out.push_str("};");
        out
    }
    pub(crate) fn emit_function(&self, f: &MetalFunction) -> String {
        let stage_str = format!("{}", f.stage);
        let inline_str = if f.is_inline { "inline " } else { "" };
        let stage_prefix = if stage_str.is_empty() {
            String::new()
        } else {
            format!("{}\n", stage_str)
        };
        let params: Vec<String> = f.params.iter().map(|p| p.emit()).collect();
        let mut out = format!(
            "{}{}{} {}({}) {{\n",
            stage_prefix,
            inline_str,
            f.return_type,
            f.name,
            params.join(",\n    ")
        );
        for s in &f.body {
            out.push_str(&self.emit_stmt(s, 1));
            out.push('\n');
        }
        out.push('}');
        out
    }
    /// Emit the full `.metal` source file as a `String`.
    pub fn emit_shader(&self, shader: &MetalShader) -> String {
        let mut out = String::new();
        for inc in &shader.includes {
            out.push_str(&format!("#include <{}>\n", inc));
        }
        if !shader.includes.is_empty() {
            out.push('\n');
        }
        for ns in &shader.using_namespaces {
            out.push_str(&format!("using namespace {};\n", ns));
        }
        if !shader.using_namespaces.is_empty() {
            out.push('\n');
        }
        for (ty, name, val) in &shader.constants {
            out.push_str(&format!("constant {} {} = {};\n", ty, name, val.emit()));
        }
        if !shader.constants.is_empty() {
            out.push('\n');
        }
        for s in &shader.structs {
            out.push_str(&self.emit_struct(s));
            out.push_str("\n\n");
        }
        for f in &shader.functions {
            out.push_str(&self.emit_function(f));
            out.push_str("\n\n");
        }
        out
    }
}
