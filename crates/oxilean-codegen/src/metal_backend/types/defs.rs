//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::impls1::*;
use super::impls2::*;
use std::collections::{HashMap, HashSet, VecDeque};

/// A field inside a Metal struct.
#[derive(Debug, Clone, PartialEq)]
pub struct MetalField {
    /// Field type
    pub ty: MetalType,
    /// Field name
    pub name: String,
    /// Optional attribute, e.g. `[[position]]`
    pub attr: MetalParamAttr,
}
impl MetalField {
    /// Create a plain field without an attribute.
    pub fn new(ty: MetalType, name: impl Into<String>) -> Self {
        MetalField {
            ty,
            name: name.into(),
            attr: MetalParamAttr::None,
        }
    }
    /// Create a field with a built-in attribute.
    pub fn with_builtin(ty: MetalType, name: impl Into<String>, b: MetalBuiltin) -> Self {
        MetalField {
            ty,
            name: name.into(),
            attr: MetalParamAttr::Builtin(b),
        }
    }
    pub(crate) fn emit(&self) -> String {
        format!("    {}{} {};", self.attr, self.ty, self.name)
    }
}
/// A fixed-capacity ring buffer of strings (for recent-event logging in MetalExt).
#[derive(Debug)]
pub struct MetalExtEventLog {
    pub(crate) entries: std::collections::VecDeque<String>,
    pub(crate) capacity: usize,
}
impl MetalExtEventLog {
    pub fn new(capacity: usize) -> Self {
        MetalExtEventLog {
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
/// Memory flags for `threadgroup_barrier`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MemFlags {
    /// `mem_flags::mem_none`
    None,
    /// `mem_flags::mem_device`
    Device,
    /// `mem_flags::mem_threadgroup`
    Threadgroup,
    /// `mem_flags::mem_texture`
    Texture,
}
/// A diagnostic message from a MetalExt pass.
#[derive(Debug, Clone)]
pub struct MetalExtDiagMsg {
    pub severity: MetalExtDiagSeverity,
    pub pass: String,
    pub message: String,
}
impl MetalExtDiagMsg {
    pub fn error(pass: impl Into<String>, msg: impl Into<String>) -> Self {
        MetalExtDiagMsg {
            severity: MetalExtDiagSeverity::Error,
            pass: pass.into(),
            message: msg.into(),
        }
    }
    pub fn warning(pass: impl Into<String>, msg: impl Into<String>) -> Self {
        MetalExtDiagMsg {
            severity: MetalExtDiagSeverity::Warning,
            pass: pass.into(),
            message: msg.into(),
        }
    }
    pub fn note(pass: impl Into<String>, msg: impl Into<String>) -> Self {
        MetalExtDiagMsg {
            severity: MetalExtDiagSeverity::Note,
            pass: pass.into(),
            message: msg.into(),
        }
    }
}
/// Unary prefix operators.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MetalUnOp {
    Neg,
    Not,
    BitNot,
}
/// Binary operators available in Metal expressions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MetalBinOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Eq,
    Neq,
    Lt,
    Le,
    Gt,
    Ge,
    And,
    Or,
    BitAnd,
    BitOr,
    BitXor,
    Shl,
    Shr,
}
/// Emission statistics for MetalExt.
#[derive(Debug, Clone, Default)]
pub struct MetalExtEmitStats {
    pub bytes_emitted: usize,
    pub items_emitted: usize,
    pub errors: usize,
    pub warnings: usize,
    pub elapsed_ms: u64,
}
impl MetalExtEmitStats {
    pub fn new() -> Self {
        MetalExtEmitStats::default()
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
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct MetalAnalysisCache {
    pub(crate) entries: std::collections::HashMap<String, MetalCacheEntry>,
    pub(crate) max_size: usize,
    pub(crate) hits: u64,
    pub(crate) misses: u64,
}
impl MetalAnalysisCache {
    #[allow(dead_code)]
    pub fn new(max_size: usize) -> Self {
        MetalAnalysisCache {
            entries: std::collections::HashMap::new(),
            max_size,
            hits: 0,
            misses: 0,
        }
    }
    #[allow(dead_code)]
    pub fn get(&mut self, key: &str) -> Option<&MetalCacheEntry> {
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
            MetalCacheEntry {
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
/// Tracks declared names for MetalExt scope analysis.
#[derive(Debug, Default)]
pub struct MetalExtNameScope {
    pub(crate) declared: std::collections::HashSet<String>,
    pub(crate) depth: usize,
    pub(crate) parent: Option<Box<MetalExtNameScope>>,
}
impl MetalExtNameScope {
    pub fn new() -> Self {
        MetalExtNameScope::default()
    }
    pub fn declare(&mut self, name: impl Into<String>) -> bool {
        self.declared.insert(name.into())
    }
    pub fn is_declared(&self, name: &str) -> bool {
        self.declared.contains(name)
    }
    pub fn push_scope(self) -> Self {
        MetalExtNameScope {
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
/// A Metal `struct` definition (used for vertex input/output, etc.).
#[derive(Debug, Clone, PartialEq)]
pub struct MetalStruct {
    /// Struct name
    pub name: String,
    /// Fields
    pub fields: Vec<MetalField>,
}
impl MetalStruct {
    /// Create a new empty struct.
    pub fn new(name: impl Into<String>) -> Self {
        MetalStruct {
            name: name.into(),
            fields: Vec::new(),
        }
    }
    /// Append a field.
    pub fn add_field(mut self, f: MetalField) -> Self {
        self.fields.push(f);
        self
    }
}
/// Top-level Metal shader module representing a single `.metal` file.
#[derive(Debug, Clone, PartialEq)]
pub struct MetalShader {
    /// `#include` headers (just names, e.g. `"metal_stdlib"`)
    pub includes: Vec<String>,
    /// `using namespace` directives (e.g. `"metal"`)
    pub using_namespaces: Vec<String>,
    /// Struct definitions
    pub structs: Vec<MetalStruct>,
    /// All functions (device helpers + shader entry points)
    pub functions: Vec<MetalFunction>,
    /// Raw constant definitions emitted at file scope
    pub constants: Vec<(MetalType, String, MetalExpr)>,
}
impl MetalShader {
    /// Create a new module with the standard Metal stdlib include.
    pub fn new() -> Self {
        MetalShader {
            includes: vec!["metal_stdlib".to_string()],
            using_namespaces: vec!["metal".to_string()],
            structs: Vec::new(),
            functions: Vec::new(),
            constants: Vec::new(),
        }
    }
    /// Add an include header name.
    pub fn add_include(mut self, header: impl Into<String>) -> Self {
        self.includes.push(header.into());
        self
    }
    /// Add a `using namespace` directive.
    pub fn add_namespace(mut self, ns: impl Into<String>) -> Self {
        self.using_namespaces.push(ns.into());
        self
    }
    /// Add a struct definition.
    pub fn add_struct(mut self, s: MetalStruct) -> Self {
        self.structs.push(s);
        self
    }
    /// Add a function.
    pub fn add_function(mut self, f: MetalFunction) -> Self {
        self.functions.push(f);
        self
    }
    /// Add a `constant` declaration at file scope.
    pub fn add_constant(mut self, ty: MetalType, name: impl Into<String>, val: MetalExpr) -> Self {
        self.constants.push((ty, name.into(), val));
        self
    }
}
/// Pass execution phase for MetalExt.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MetalExtPassPhase {
    Early,
    Middle,
    Late,
    Finalize,
}
impl MetalExtPassPhase {
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
/// Worklist for MetalExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct MetalExtWorklist {
    pub(crate) items: std::collections::VecDeque<usize>,
    pub(crate) present: Vec<bool>,
}
impl MetalExtWorklist {
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
/// Constant folding helper for MetalExt.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct MetalExtConstFolder {
    pub(crate) folds: usize,
    pub(crate) failures: usize,
    pub(crate) enabled: bool,
}
impl MetalExtConstFolder {
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
/// Configuration for MetalExt passes.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct MetalExtPassConfig {
    pub name: String,
    pub phase: MetalExtPassPhase,
    pub enabled: bool,
    pub max_iterations: usize,
    pub debug: u32,
    pub timeout_ms: Option<u64>,
}
impl MetalExtPassConfig {
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            phase: MetalExtPassPhase::Middle,
            enabled: true,
            max_iterations: 100,
            debug: 0,
            timeout_ms: None,
        }
    }
    #[allow(dead_code)]
    pub fn with_phase(mut self, phase: MetalExtPassPhase) -> Self {
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
/// Built-in thread/grid position variables in Metal.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MetalBuiltin {
    /// `[[thread_position_in_grid]]`  — uint3
    ThreadPositionInGrid,
    /// `[[thread_position_in_threadgroup]]` — uint3
    ThreadPositionInThreadgroup,
    /// `[[threadgroup_position_in_grid]]` — uint3
    ThreadgroupPositionInGrid,
    /// `[[threads_per_threadgroup]]` — uint3
    ThreadsPerThreadgroup,
    /// `[[threads_per_grid]]` — uint3
    ThreadsPerGrid,
    /// `[[thread_index_in_simdgroup]]` — uint
    ThreadIndexInSimdgroup,
    /// `[[simdgroup_index_in_threadgroup]]` — uint
    SimdgroupIndexInThreadgroup,
    /// `[[vertex_id]]`
    VertexId,
    /// `[[instance_id]]`
    InstanceId,
    /// `[[position]]` — float4 (vertex output position)
    Position,
    /// `[[front_facing]]` — bool
    FrontFacing,
    /// `[[sample_id]]` — uint
    SampleId,
    /// `[[depth(any)]]` — float (fragment depth output)
    Depth,
}
impl MetalBuiltin {
    /// The MSL attribute string for this built-in.
    pub fn attribute(&self) -> &'static str {
        match self {
            MetalBuiltin::ThreadPositionInGrid => "[[thread_position_in_grid]]",
            MetalBuiltin::ThreadPositionInThreadgroup => "[[thread_position_in_threadgroup]]",
            MetalBuiltin::ThreadgroupPositionInGrid => "[[threadgroup_position_in_grid]]",
            MetalBuiltin::ThreadsPerThreadgroup => "[[threads_per_threadgroup]]",
            MetalBuiltin::ThreadsPerGrid => "[[threads_per_grid]]",
            MetalBuiltin::ThreadIndexInSimdgroup => "[[thread_index_in_simdgroup]]",
            MetalBuiltin::SimdgroupIndexInThreadgroup => "[[simdgroup_index_in_threadgroup]]",
            MetalBuiltin::VertexId => "[[vertex_id]]",
            MetalBuiltin::InstanceId => "[[instance_id]]",
            MetalBuiltin::Position => "[[position]]",
            MetalBuiltin::FrontFacing => "[[front_facing]]",
            MetalBuiltin::SampleId => "[[sample_id]]",
            MetalBuiltin::Depth => "[[depth(any)]]",
        }
    }
    /// Canonical MSL type for this built-in.
    pub fn metal_type(&self) -> MetalType {
        match self {
            MetalBuiltin::ThreadPositionInGrid
            | MetalBuiltin::ThreadPositionInThreadgroup
            | MetalBuiltin::ThreadgroupPositionInGrid
            | MetalBuiltin::ThreadsPerThreadgroup
            | MetalBuiltin::ThreadsPerGrid => MetalType::Uint3,
            MetalBuiltin::ThreadIndexInSimdgroup
            | MetalBuiltin::SimdgroupIndexInThreadgroup
            | MetalBuiltin::VertexId
            | MetalBuiltin::InstanceId
            | MetalBuiltin::SampleId => MetalType::Uint,
            MetalBuiltin::Position => MetalType::Float4,
            MetalBuiltin::FrontFacing => MetalType::Bool,
            MetalBuiltin::Depth => MetalType::Float,
        }
    }
}
/// Metal Shading Language statement AST node.
#[derive(Debug, Clone, PartialEq)]
pub enum MetalStmt {
    /// Variable declaration with optional initializer: `T name [ = init ];`
    VarDecl {
        ty: MetalType,
        name: String,
        init: Option<MetalExpr>,
        /// If true, emit `const T name = ...`
        is_const: bool,
    },
    /// Simple assignment: `lhs = rhs;`
    Assign { lhs: MetalExpr, rhs: MetalExpr },
    /// Compound assignment: `lhs += rhs;` etc.
    CompoundAssign {
        lhs: MetalExpr,
        op: MetalBinOp,
        rhs: MetalExpr,
    },
    /// If / optional else
    IfElse {
        cond: MetalExpr,
        then_body: Vec<MetalStmt>,
        else_body: Option<Vec<MetalStmt>>,
    },
    /// C-style for loop
    ForLoop {
        init: Box<MetalStmt>,
        cond: MetalExpr,
        step: MetalExpr,
        body: Vec<MetalStmt>,
    },
    /// While loop
    WhileLoop {
        cond: MetalExpr,
        body: Vec<MetalStmt>,
    },
    /// `return [expr];`
    Return(Option<MetalExpr>),
    /// Raw expression statement: `expr;`
    Expr(MetalExpr),
    /// `threadgroup_barrier(flags);`
    Barrier(MemFlags),
    /// Block of statements: `{ ... }`
    Block(Vec<MetalStmt>),
    /// `break;`
    Break,
    /// `continue;`
    Continue,
}
/// Severity of a MetalExt diagnostic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MetalExtDiagSeverity {
    Note,
    Warning,
    Error,
}
/// Pass-timing record for MetalExt profiler.
#[derive(Debug, Clone)]
pub struct MetalExtPassTiming {
    pub pass_name: String,
    pub elapsed_us: u64,
    pub items_processed: usize,
    pub bytes_before: usize,
    pub bytes_after: usize,
}
impl MetalExtPassTiming {
    pub fn new(
        pass_name: impl Into<String>,
        elapsed_us: u64,
        items: usize,
        before: usize,
        after: usize,
    ) -> Self {
        MetalExtPassTiming {
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
/// Shader function stage, determines the `[[stage_in]]` / attribute emitted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MetalStage {
    /// `[[vertex]]`
    Vertex,
    /// `[[fragment]]`
    Fragment,
    /// `[[kernel]]`
    Kernel,
    /// `[[mesh]]` (Metal 3)
    Mesh,
    /// Plain device function — no stage attribute
    Device,
}
/// Pipeline profiler for MetalExt.
#[derive(Debug, Default)]
pub struct MetalExtProfiler {
    pub(crate) timings: Vec<MetalExtPassTiming>,
}
impl MetalExtProfiler {
    pub fn new() -> Self {
        MetalExtProfiler::default()
    }
    pub fn record(&mut self, t: MetalExtPassTiming) {
        self.timings.push(t);
    }
    pub fn total_elapsed_us(&self) -> u64 {
        self.timings.iter().map(|t| t.elapsed_us).sum()
    }
    pub fn slowest_pass(&self) -> Option<&MetalExtPassTiming> {
        self.timings.iter().max_by_key(|t| t.elapsed_us)
    }
    pub fn num_passes(&self) -> usize {
        self.timings.len()
    }
    pub fn profitable_passes(&self) -> Vec<&MetalExtPassTiming> {
        self.timings.iter().filter(|t| t.is_profitable()).collect()
    }
}
