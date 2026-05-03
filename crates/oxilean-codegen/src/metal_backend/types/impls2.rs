//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::defs::*;
use super::impls1::*;
use std::collections::{HashMap, HashSet, VecDeque};

/// A field inside a Metal struct.
/// A generic key-value configuration store for MetalExt.
#[derive(Debug, Clone, Default)]
pub struct MetalExtConfig {
    pub(crate) entries: std::collections::HashMap<String, String>,
}
impl MetalExtConfig {
    pub fn new() -> Self {
        MetalExtConfig::default()
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
/// A parameter in a Metal shader function.
#[derive(Debug, Clone, PartialEq)]
pub struct MetalParam {
    /// Parameter type
    pub ty: MetalType,
    /// Parameter name
    pub name: String,
    /// Metal binding attribute
    pub attr: MetalParamAttr,
}
impl MetalParam {
    /// Create a plain parameter without an attribute.
    pub fn new(ty: MetalType, name: impl Into<String>) -> Self {
        MetalParam {
            ty,
            name: name.into(),
            attr: MetalParamAttr::None,
        }
    }
    /// Create a parameter with a buffer binding.
    pub fn buffer(ty: MetalType, name: impl Into<String>, index: u32) -> Self {
        MetalParam {
            ty,
            name: name.into(),
            attr: MetalParamAttr::Buffer(index),
        }
    }
    /// Create a parameter with a texture binding.
    pub fn texture(ty: MetalType, name: impl Into<String>, index: u32) -> Self {
        MetalParam {
            ty,
            name: name.into(),
            attr: MetalParamAttr::Texture(index),
        }
    }
    /// Create a parameter with a built-in attribute.
    pub fn builtin(b: MetalBuiltin) -> Self {
        let ty = b.metal_type();
        let name = format!("{:?}", b).to_lowercase();
        MetalParam {
            ty,
            name,
            attr: MetalParamAttr::Builtin(b),
        }
    }
    pub(crate) fn emit(&self) -> String {
        format!("{} {}{}", self.ty, self.name, self.attr)
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct MetalPassStats {
    pub total_runs: u32,
    pub successful_runs: u32,
    pub total_changes: u64,
    pub time_ms: u64,
    pub iterations_used: u32,
}
impl MetalPassStats {
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
/// A Metal shader function (vertex / fragment / kernel / mesh / device).
#[derive(Debug, Clone, PartialEq)]
pub struct MetalFunction {
    /// Function name
    pub name: String,
    /// Shader stage
    pub stage: MetalStage,
    /// Parameter list
    pub params: Vec<MetalParam>,
    /// Return type
    pub return_type: MetalType,
    /// Function body
    pub body: Vec<MetalStmt>,
    /// Whether the function is `inline`
    pub is_inline: bool,
}
impl MetalFunction {
    /// Create a new function with the given stage.
    pub fn new(name: impl Into<String>, stage: MetalStage, return_type: MetalType) -> Self {
        MetalFunction {
            name: name.into(),
            stage,
            params: Vec::new(),
            return_type,
            body: Vec::new(),
            is_inline: false,
        }
    }
    /// Create a compute (kernel) function returning `void`.
    pub fn kernel(name: impl Into<String>) -> Self {
        MetalFunction::new(name, MetalStage::Kernel, MetalType::Void)
    }
    /// Create a vertex shader.
    pub fn vertex(name: impl Into<String>, return_type: MetalType) -> Self {
        MetalFunction::new(name, MetalStage::Vertex, return_type)
    }
    /// Create a fragment shader.
    pub fn fragment(name: impl Into<String>, return_type: MetalType) -> Self {
        MetalFunction::new(name, MetalStage::Fragment, return_type)
    }
    /// Create a device helper function.
    pub fn device_fn(name: impl Into<String>, return_type: MetalType) -> Self {
        MetalFunction::new(name, MetalStage::Device, return_type)
    }
    /// Mark as `inline`.
    pub fn with_inline(mut self) -> Self {
        self.is_inline = true;
        self
    }
    /// Append a parameter.
    pub fn add_param(mut self, p: MetalParam) -> Self {
        self.params.push(p);
        self
    }
    /// Append a body statement.
    pub fn add_stmt(mut self, s: MetalStmt) -> Self {
        self.body.push(s);
        self
    }
}
/// Metal memory address spaces (analogous to CUDA memory qualifiers).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MetalAddressSpace {
    /// `device` — GPU-accessible memory (buffers)
    Device,
    /// `constant` — read-only, broadcast-cached memory
    Constant,
    /// `threadgroup` — shared memory within a threadgroup (≈ CUDA `__shared__`)
    Threadgroup,
    /// `threadgroup_imageblock` — imageblock memory
    ThreadgroupImageblock,
    /// `ray_data` — ray-tracing payload
    RayData,
    /// `object_data` — mesh pipeline object data
    ObjectData,
    /// `thread` — private per-thread memory (default for local vars)
    Thread,
}
/// The binding kind for a Metal function parameter attribute.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MetalParamAttr {
    /// `[[buffer(index)]]`
    Buffer(u32),
    /// `[[texture(index)]]`
    Texture(u32),
    /// `[[sampler(index)]]`
    Sampler(u32),
    /// `[[stage_in]]`
    StageIn,
    /// Built-in attribute, e.g. `[[thread_position_in_grid]]`
    Builtin(MetalBuiltin),
    /// No attribute
    None,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct MetalCacheEntry {
    pub key: String,
    pub data: Vec<u8>,
    pub timestamp: u64,
    pub valid: bool,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct MetalWorklist {
    pub(crate) items: std::collections::VecDeque<u32>,
    pub(crate) in_worklist: std::collections::HashSet<u32>,
}
impl MetalWorklist {
    #[allow(dead_code)]
    pub fn new() -> Self {
        MetalWorklist {
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
/// Statistics for MetalExt passes.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct MetalExtPassStats {
    pub iterations: usize,
    pub changed: bool,
    pub nodes_visited: usize,
    pub nodes_modified: usize,
    pub time_ms: u64,
    pub memory_bytes: usize,
    pub errors: usize,
}
impl MetalExtPassStats {
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
    pub fn merge(&mut self, o: &MetalExtPassStats) {
        self.iterations += o.iterations;
        self.changed |= o.changed;
        self.nodes_visited += o.nodes_visited;
        self.nodes_modified += o.nodes_modified;
        self.time_ms += o.time_ms;
        self.memory_bytes = self.memory_bytes.max(o.memory_bytes);
        self.errors += o.errors;
    }
}
/// Metal Shading Language expression AST node.
#[derive(Debug, Clone, PartialEq)]
pub enum MetalExpr {
    /// Integer literal
    LitInt(i64),
    /// Float literal
    LitFloat(f64),
    /// Boolean literal
    LitBool(bool),
    /// Named variable
    Var(String),
    /// Built-in variable access (used inside shader parameters)
    Builtin(MetalBuiltin),
    /// Array subscript: `arr[idx]`
    Index(Box<MetalExpr>, Box<MetalExpr>),
    /// Struct member: `s.field`
    Member(Box<MetalExpr>, String),
    /// Pointer member: `p->field`
    PtrMember(Box<MetalExpr>, String),
    /// C-style cast: `(T)expr` — MSL also supports static_cast
    Cast(MetalType, Box<MetalExpr>),
    /// Function / constructor call: `func(args...)`
    Call(String, Vec<MetalExpr>),
    /// Binary operation
    BinOp(Box<MetalExpr>, MetalBinOp, Box<MetalExpr>),
    /// Unary operation
    UnOp(MetalUnOp, Box<MetalExpr>),
    /// Ternary conditional: `cond ? then : else`
    Ternary(Box<MetalExpr>, Box<MetalExpr>, Box<MetalExpr>),
    /// `simd_sum(val)` — warp/simd group reduction
    SimdSum(Box<MetalExpr>),
    /// `simd_shuffle_down(val, delta)`
    SimdShuffleDown(Box<MetalExpr>, Box<MetalExpr>),
    /// `simd_broadcast(val, lane)`
    SimdBroadcast(Box<MetalExpr>, Box<MetalExpr>),
    /// `atomic_fetch_add_explicit(&atom, val, order)` — simplified
    AtomicFetchAdd(Box<MetalExpr>, Box<MetalExpr>),
    /// `threadgroup_barrier(mem_flags::mem_device)` etc.
    ThreadgroupBarrier(MemFlags),
    /// `as_type<T>(expr)` — bitcast
    AsType(MetalType, Box<MetalExpr>),
    /// `select(a, b, cond)` — component-wise select
    Select(Box<MetalExpr>, Box<MetalExpr>, Box<MetalExpr>),
    /// `dot(a, b)` — dot product
    Dot(Box<MetalExpr>, Box<MetalExpr>),
    /// `length(v)` — vector length
    Length(Box<MetalExpr>),
    /// `normalize(v)` — vector normalize
    Normalize(Box<MetalExpr>),
    /// `clamp(val, lo, hi)`
    Clamp(Box<MetalExpr>, Box<MetalExpr>, Box<MetalExpr>),
}
impl MetalExpr {
    pub(crate) fn emit(&self) -> String {
        match self {
            MetalExpr::LitInt(n) => n.to_string(),
            MetalExpr::LitFloat(f) => format!("{:.6}f", f),
            MetalExpr::LitBool(b) => if *b { "true" } else { "false" }.to_string(),
            MetalExpr::Var(name) => name.clone(),
            MetalExpr::Builtin(b) => format!("{:?}", b).to_lowercase(),
            MetalExpr::Index(base, idx) => format!("{}[{}]", base.emit(), idx.emit()),
            MetalExpr::Member(base, field) => format!("{}.{}", base.emit(), field),
            MetalExpr::PtrMember(base, field) => format!("{}->{}", base.emit(), field),
            MetalExpr::Cast(ty, expr) => format!("(({})({})))", ty, expr.emit()),
            MetalExpr::Call(name, args) => {
                let arg_strs: Vec<String> = args.iter().map(|a| a.emit()).collect();
                format!("{}({})", name, arg_strs.join(", "))
            }
            MetalExpr::BinOp(lhs, op, rhs) => {
                format!("({} {} {})", lhs.emit(), op, rhs.emit())
            }
            MetalExpr::UnOp(op, expr) => format!("({}{})", op, expr.emit()),
            MetalExpr::Ternary(cond, then, els) => {
                format!("({} ? {} : {})", cond.emit(), then.emit(), els.emit())
            }
            MetalExpr::SimdSum(val) => format!("simd_sum({})", val.emit()),
            MetalExpr::SimdShuffleDown(val, delta) => {
                format!("simd_shuffle_down({}, {})", val.emit(), delta.emit())
            }
            MetalExpr::SimdBroadcast(val, lane) => {
                format!("simd_broadcast({}, {})", val.emit(), lane.emit())
            }
            MetalExpr::AtomicFetchAdd(atom, val) => {
                format!(
                    "atomic_fetch_add_explicit({}, {}, memory_order_relaxed)",
                    atom.emit(),
                    val.emit()
                )
            }
            MetalExpr::ThreadgroupBarrier(flags) => {
                format!("threadgroup_barrier({})", flags)
            }
            MetalExpr::AsType(ty, expr) => format!("as_type<{}>({})", ty, expr.emit()),
            MetalExpr::Select(a, b, cond) => {
                format!("select({}, {}, {})", a.emit(), b.emit(), cond.emit())
            }
            MetalExpr::Dot(a, b) => format!("dot({}, {})", a.emit(), b.emit()),
            MetalExpr::Length(v) => format!("length({})", v.emit()),
            MetalExpr::Normalize(v) => format!("normalize({})", v.emit()),
            MetalExpr::Clamp(val, lo, hi) => {
                format!("clamp({}, {}, {})", val.emit(), lo.emit(), hi.emit())
            }
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum MetalPassPhase {
    Analysis,
    Transformation,
    Verification,
    Cleanup,
}
impl MetalPassPhase {
    #[allow(dead_code)]
    pub fn name(&self) -> &str {
        match self {
            MetalPassPhase::Analysis => "analysis",
            MetalPassPhase::Transformation => "transformation",
            MetalPassPhase::Verification => "verification",
            MetalPassPhase::Cleanup => "cleanup",
        }
    }
    #[allow(dead_code)]
    pub fn is_modifying(&self) -> bool {
        matches!(
            self,
            MetalPassPhase::Transformation | MetalPassPhase::Cleanup
        )
    }
}
/// Pass registry for MetalExt.
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct MetalExtPassRegistry {
    pub(crate) configs: Vec<MetalExtPassConfig>,
    pub(crate) stats: Vec<MetalExtPassStats>,
}
impl MetalExtPassRegistry {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    #[allow(dead_code)]
    pub fn register(&mut self, c: MetalExtPassConfig) {
        self.stats.push(MetalExtPassStats::new());
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
    pub fn get(&self, i: usize) -> Option<&MetalExtPassConfig> {
        self.configs.get(i)
    }
    #[allow(dead_code)]
    pub fn get_stats(&self, i: usize) -> Option<&MetalExtPassStats> {
        self.stats.get(i)
    }
    #[allow(dead_code)]
    pub fn enabled_passes(&self) -> Vec<&MetalExtPassConfig> {
        self.configs.iter().filter(|c| c.enabled).collect()
    }
    #[allow(dead_code)]
    pub fn passes_in_phase(&self, ph: &MetalExtPassPhase) -> Vec<&MetalExtPassConfig> {
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
/// Analysis cache for MetalExt.
#[allow(dead_code)]
#[derive(Debug)]
pub struct MetalExtCache {
    pub(crate) entries: Vec<(u64, Vec<u8>, bool, u32)>,
    pub(crate) cap: usize,
    pub(crate) total_hits: u64,
    pub(crate) total_misses: u64,
}
impl MetalExtCache {
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
pub struct MetalDepGraph {
    pub(crate) nodes: Vec<u32>,
    pub(crate) edges: Vec<(u32, u32)>,
}
impl MetalDepGraph {
    #[allow(dead_code)]
    pub fn new() -> Self {
        MetalDepGraph {
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
/// Dominator tree for MetalExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct MetalExtDomTree {
    pub(crate) idom: Vec<Option<usize>>,
    pub(crate) children: Vec<Vec<usize>>,
    pub(crate) depth: Vec<usize>,
}
impl MetalExtDomTree {
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
/// A monotonically increasing ID generator for MetalExt.
#[derive(Debug, Default)]
pub struct MetalExtIdGen {
    pub(crate) next: u32,
}
impl MetalExtIdGen {
    pub fn new() -> Self {
        MetalExtIdGen::default()
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
