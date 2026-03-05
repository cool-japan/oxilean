//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

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
    pub(super) fn emit(&self) -> String {
        format!("    {}{} {};", self.attr, self.ty, self.name)
    }
}
/// A fixed-capacity ring buffer of strings (for recent-event logging in MetalExt).
#[derive(Debug)]
pub struct MetalExtEventLog {
    pub(super) entries: std::collections::VecDeque<String>,
    pub(super) capacity: usize,
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
    pub(super) entries: std::collections::HashMap<String, MetalCacheEntry>,
    pub(super) max_size: usize,
    pub(super) hits: u64,
    pub(super) misses: u64,
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
    pub(super) declared: std::collections::HashSet<String>,
    pub(super) depth: usize,
    pub(super) parent: Option<Box<MetalExtNameScope>>,
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
    pub(super) items: std::collections::VecDeque<usize>,
    pub(super) present: Vec<bool>,
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
    pub(super) folds: usize,
    pub(super) failures: usize,
    pub(super) enabled: bool,
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
    pub(super) timings: Vec<MetalExtPassTiming>,
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
/// A feature flag set for MetalExt capabilities.
#[derive(Debug, Clone, Default)]
pub struct MetalExtFeatures {
    pub(super) flags: std::collections::HashSet<String>,
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
    pub(super) n: usize,
    pub(super) adj: Vec<Vec<usize>>,
    pub(super) rev: Vec<Vec<usize>>,
    pub(super) edge_count: usize,
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
    pub(super) buf: String,
    pub(super) indent_level: usize,
    pub(super) indent_str: String,
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
    pub(super) msgs: Vec<MetalExtDiagMsg>,
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
    pub(super) configs: Vec<MetalPassConfig>,
    pub(super) stats: std::collections::HashMap<String, MetalPassStats>,
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
    pub(super) indent_width: usize,
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
    pub(super) fn indent(&self, depth: usize) -> String {
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
    pub(super) fn emit_if_else(
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
    pub(super) fn emit_for_loop(
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
    pub(super) fn emit_while(&self, cond: &MetalExpr, body: &[MetalStmt], depth: usize) -> String {
        let ind = self.indent(depth);
        let mut out = format!("{}while ({}) {{\n", ind, cond.emit());
        for s in body {
            out.push_str(&self.emit_stmt(s, depth + 1));
            out.push('\n');
        }
        out.push_str(&format!("{}}}", ind));
        out
    }
    pub(super) fn emit_struct(&self, s: &MetalStruct) -> String {
        let mut out = format!("struct {} {{\n", s.name);
        for field in &s.fields {
            out.push_str(&field.emit());
            out.push('\n');
        }
        out.push_str("};");
        out
    }
    pub(super) fn emit_function(&self, f: &MetalFunction) -> String {
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
/// A generic key-value configuration store for MetalExt.
#[derive(Debug, Clone, Default)]
pub struct MetalExtConfig {
    pub(super) entries: std::collections::HashMap<String, String>,
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
    pub(super) fn emit(&self) -> String {
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
    pub(super) items: std::collections::VecDeque<u32>,
    pub(super) in_worklist: std::collections::HashSet<u32>,
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
    pub(super) fn emit(&self) -> String {
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
    pub(super) configs: Vec<MetalExtPassConfig>,
    pub(super) stats: Vec<MetalExtPassStats>,
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
    pub(super) entries: Vec<(u64, Vec<u8>, bool, u32)>,
    pub(super) cap: usize,
    pub(super) total_hits: u64,
    pub(super) total_misses: u64,
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
    pub(super) nodes: Vec<u32>,
    pub(super) edges: Vec<(u32, u32)>,
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
    pub(super) idom: Vec<Option<usize>>,
    pub(super) children: Vec<Vec<usize>>,
    pub(super) depth: Vec<usize>,
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
    pub(super) next: u32,
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
