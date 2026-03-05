//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::HashMap;

use std::collections::{HashSet, VecDeque};

/// An MLIR function definition.
#[derive(Debug, Clone)]
pub struct MlirFunc {
    /// Function name (without `@`)
    pub name: String,
    /// Function arguments: list of (name, type)
    pub args: Vec<(String, MlirType)>,
    /// Return types
    pub results: Vec<MlirType>,
    /// Function body (a region)
    pub body: MlirRegion,
    /// Extra attributes (e.g., sym_visibility = "private")
    pub attributes: Vec<(String, MlirAttr)>,
    /// Whether this is a declaration only (no body)
    pub is_declaration: bool,
}
impl MlirFunc {
    /// Create a simple function with a body.
    pub fn new(
        name: impl Into<String>,
        args: Vec<(String, MlirType)>,
        results: Vec<MlirType>,
        body: MlirRegion,
    ) -> Self {
        MlirFunc {
            name: name.into(),
            args,
            results,
            body,
            attributes: vec![],
            is_declaration: false,
        }
    }
    /// Create a function declaration (no body, for extern functions).
    pub fn declaration(
        name: impl Into<String>,
        args: Vec<MlirType>,
        results: Vec<MlirType>,
    ) -> Self {
        let arg_vals = args
            .into_iter()
            .enumerate()
            .map(|(i, t)| (format!("arg{}", i), t))
            .collect();
        MlirFunc {
            name: name.into(),
            args: arg_vals,
            results,
            body: MlirRegion::empty(),
            attributes: vec![],
            is_declaration: true,
        }
    }
    /// Emit the function as MLIR text.
    pub fn emit(&self) -> String {
        let mut out = String::new();
        if self.is_declaration {
            out.push_str("  func.func private @");
        } else {
            out.push_str("  func.func @");
        }
        out.push_str(&self.name);
        out.push('(');
        for (i, (name, ty)) in self.args.iter().enumerate() {
            if i > 0 {
                out.push_str(", ");
            }
            out.push_str(&format!("%{}: {}", name, ty));
        }
        out.push(')');
        if !self.results.is_empty() {
            out.push_str(" -> ");
            if self.results.len() == 1 {
                out.push_str(&self.results[0].to_string());
            } else {
                out.push('(');
                for (i, r) in self.results.iter().enumerate() {
                    if i > 0 {
                        out.push_str(", ");
                    }
                    out.push_str(&r.to_string());
                }
                out.push(')');
            }
        }
        if !self.attributes.is_empty() {
            out.push_str(" attributes {");
            for (i, (k, v)) in self.attributes.iter().enumerate() {
                if i > 0 {
                    out.push_str(", ");
                }
                out.push_str(&format!("{} = {}", k, v));
            }
            out.push('}');
        }
        if self.is_declaration {
            out.push('\n');
        } else {
            out.push_str(" {\n");
            for block in &self.body.blocks {
                out.push_str(&format!("{}", block));
            }
            out.push_str("  }\n");
        }
        out
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum MLIRPassPhase {
    Analysis,
    Transformation,
    Verification,
    Cleanup,
}
impl MLIRPassPhase {
    #[allow(dead_code)]
    pub fn name(&self) -> &str {
        match self {
            MLIRPassPhase::Analysis => "analysis",
            MLIRPassPhase::Transformation => "transformation",
            MLIRPassPhase::Verification => "verification",
            MLIRPassPhase::Cleanup => "cleanup",
        }
    }
    #[allow(dead_code)]
    pub fn is_modifying(&self) -> bool {
        matches!(self, MLIRPassPhase::Transformation | MLIRPassPhase::Cleanup)
    }
}
/// MLIR code generation backend.
pub struct MlirBackend {
    pub(super) module: MlirModule,
    pub(super) ssa: SsaCounter,
    pub(super) pass_pipeline: Vec<String>,
}
impl MlirBackend {
    /// Create a new MLIR backend.
    pub fn new() -> Self {
        MlirBackend {
            module: MlirModule::new(),
            ssa: SsaCounter::new(),
            pass_pipeline: vec![],
        }
    }
    /// Create a backend with a module name.
    pub fn with_name(name: impl Into<String>) -> Self {
        MlirBackend {
            module: MlirModule::named(name),
            ssa: SsaCounter::new(),
            pass_pipeline: vec![],
        }
    }
    /// Add a pass to the pipeline.
    pub fn add_pass(&mut self, pass: impl Into<String>) {
        self.pass_pipeline.push(pass.into());
    }
    /// Add a simple integer add function.
    pub fn compile_add_func(&mut self, name: &str, bits: u32) {
        let int_ty = MlirType::Integer(bits, false);
        let mut builder = MlirBuilder::new();
        let arg0 = MlirValue::named("arg0", int_ty.clone());
        let arg1 = MlirValue::named("arg1", int_ty.clone());
        let sum = builder.addi(arg0.clone(), arg1.clone());
        builder.return_op(vec![sum]);
        let block = MlirBlock::entry(vec![arg0, arg1], builder.take_ops());
        let region = MlirRegion::single_block(block);
        let func = MlirFunc::new(name, vec![], vec![int_ty.clone()], region);
        self.module.add_function(func);
    }
    /// Compile a declaration to a simple wrapper function.
    pub fn compile_decl(&mut self, name: &str, arg_types: Vec<MlirType>, ret_type: MlirType) {
        let args: Vec<(String, MlirType)> = arg_types
            .into_iter()
            .enumerate()
            .map(|(i, t)| (format!("arg{}", i), t))
            .collect();
        let mut builder = MlirBuilder::new();
        let zero = builder.const_int(0, 64);
        builder.return_op(vec![zero]);
        let block = MlirBlock::entry(vec![], builder.take_ops());
        let func = MlirFunc::new(name, args, vec![ret_type], MlirRegion::single_block(block));
        self.module.add_function(func);
    }
    /// Emit the full MLIR module as text.
    pub fn emit_module(&self) -> String {
        self.module.emit()
    }
    /// Generate the `mlir-opt` pass pipeline string.
    pub fn run_passes(&self) -> String {
        if self.pass_pipeline.is_empty() {
            String::new()
        } else {
            format!("mlir-opt --{}", self.pass_pipeline.join(" --"))
        }
    }
    /// Get the module (for further manipulation).
    pub fn module_mut(&mut self) -> &mut MlirModule {
        &mut self.module
    }
}
/// Constant folding helper for MLIRExt.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct MLIRExtConstFolder {
    pub(super) folds: usize,
    pub(super) failures: usize,
    pub(super) enabled: bool,
}
impl MLIRExtConstFolder {
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
pub struct MLIRCacheEntry {
    pub key: String,
    pub data: Vec<u8>,
    pub timestamp: u64,
    pub valid: bool,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct MLIRLivenessInfo {
    pub live_in: Vec<std::collections::HashSet<u32>>,
    pub live_out: Vec<std::collections::HashSet<u32>>,
    pub defs: Vec<std::collections::HashSet<u32>>,
    pub uses: Vec<std::collections::HashSet<u32>>,
}
impl MLIRLivenessInfo {
    #[allow(dead_code)]
    pub fn new(block_count: usize) -> Self {
        MLIRLivenessInfo {
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
pub struct MLIRWorklist {
    pub(super) items: std::collections::VecDeque<u32>,
    pub(super) in_worklist: std::collections::HashSet<u32>,
}
impl MLIRWorklist {
    #[allow(dead_code)]
    pub fn new() -> Self {
        MLIRWorklist {
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
/// Pass execution phase for MLIRExt.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MLIRExtPassPhase {
    Early,
    Middle,
    Late,
    Finalize,
}
impl MLIRExtPassPhase {
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
/// MLIR dialect classification.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MlirDialect {
    /// Built-in dialect (module, func types)
    Builtin,
    /// Arithmetic operations: addi, addf, muli, divsi, cmpi, extsi, trunci
    Arith,
    /// Function definitions and calls: func.func, func.call, func.return
    Func,
    /// Control flow: cf.br, cf.cond_br, cf.switch
    CF,
    /// Memory references: memref.alloc, memref.load, memref.store, memref.dealloc
    MemRef,
    /// Structured control flow: scf.if, scf.for, scf.while
    SCF,
    /// Affine transformations: affine.for, affine.load, affine.store
    Affine,
    /// Tensor operations: tensor.extract, tensor.insert, tensor.reshape
    Tensor,
    /// Vector operations: vector.load, vector.store, vector.broadcast
    Vector,
    /// Linear algebra operations for ML: linalg.matmul, linalg.generic
    Linalg,
    /// GPU dialect: gpu.launch, gpu.thread_id, gpu.block_dim
    GPU,
    /// LLVM IR dialect: llvm.add, llvm.mlir.constant, llvm.call
    LLVM,
    /// Math functions: math.sin, math.cos, math.exp, math.log, math.sqrt
    Math,
    /// Index type operations
    Index,
}
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct MLIRPassStats {
    pub total_runs: u32,
    pub successful_runs: u32,
    pub total_changes: u64,
    pub time_ms: u64,
    pub iterations_used: u32,
}
impl MLIRPassStats {
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
/// Analysis cache for MLIRExt.
#[allow(dead_code)]
#[derive(Debug)]
pub struct MLIRExtCache {
    pub(super) entries: Vec<(u64, Vec<u8>, bool, u32)>,
    pub(super) cap: usize,
    pub(super) total_hits: u64,
    pub(super) total_misses: u64,
}
impl MLIRExtCache {
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
/// Configuration for MLIRExt passes.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct MLIRExtPassConfig {
    pub name: String,
    pub phase: MLIRExtPassPhase,
    pub enabled: bool,
    pub max_iterations: usize,
    pub debug: u32,
    pub timeout_ms: Option<u64>,
}
impl MLIRExtPassConfig {
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            phase: MLIRExtPassPhase::Middle,
            enabled: true,
            max_iterations: 100,
            debug: 0,
            timeout_ms: None,
        }
    }
    #[allow(dead_code)]
    pub fn with_phase(mut self, phase: MLIRExtPassPhase) -> Self {
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
/// Counter for generating fresh SSA value names.
#[derive(Debug, Default)]
pub struct SsaCounter {
    pub(super) counter: u32,
    pub(super) named: HashMap<String, u32>,
}
impl SsaCounter {
    /// Create a new counter.
    pub fn new() -> Self {
        SsaCounter::default()
    }
    /// Allocate the next numbered SSA value.
    pub fn next(&mut self, ty: MlirType) -> MlirValue {
        let id = self.counter;
        self.counter += 1;
        MlirValue::numbered(id, ty)
    }
    /// Allocate a named SSA value (deduplicated).
    pub fn named(&mut self, base: &str, ty: MlirType) -> MlirValue {
        let count = self.named.entry(base.to_string()).or_insert(0);
        let name = if *count == 0 {
            base.to_string()
        } else {
            format!("{}_{}", base, count)
        };
        *count += 1;
        MlirValue::named(name, ty)
    }
    /// Reset the counter.
    pub fn reset(&mut self) {
        self.counter = 0;
        self.named.clear();
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct MLIRDominatorTree {
    pub idom: Vec<Option<u32>>,
    pub dom_children: Vec<Vec<u32>>,
    pub dom_depth: Vec<u32>,
}
impl MLIRDominatorTree {
    #[allow(dead_code)]
    pub fn new(size: usize) -> Self {
        MLIRDominatorTree {
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
/// Builder for constructing MLIR operations conveniently.
pub struct MlirBuilder {
    pub(super) ssa: SsaCounter,
    pub(super) ops: Vec<MlirOp>,
}
impl MlirBuilder {
    /// Create a new builder.
    pub fn new() -> Self {
        MlirBuilder {
            ssa: SsaCounter::new(),
            ops: vec![],
        }
    }
    /// Emit `arith.constant` for integer.
    pub fn const_int(&mut self, value: i64, bits: u32) -> MlirValue {
        let ty = MlirType::Integer(bits, false);
        let result = self.ssa.next(ty.clone());
        let mut op = MlirOp::unary_result(
            result.clone(),
            "arith.constant",
            vec![],
            vec![("value".to_string(), MlirAttr::Integer(value, ty))],
        );
        op.type_annotations = vec![result.ty.clone()];
        self.ops.push(op);
        result
    }
    /// Emit `arith.constant` for float.
    pub fn const_float(&mut self, value: f64, bits: u32) -> MlirValue {
        let ty = MlirType::Float(bits);
        let result = self.ssa.next(ty.clone());
        let op = MlirOp::unary_result(
            result.clone(),
            "arith.constant",
            vec![],
            vec![("value".to_string(), MlirAttr::Float(value))],
        );
        self.ops.push(op);
        result
    }
    /// Emit `arith.addi`.
    pub fn addi(&mut self, lhs: MlirValue, rhs: MlirValue) -> MlirValue {
        let ty = lhs.ty.clone();
        let result = self.ssa.next(ty.clone());
        let mut op = MlirOp::unary_result(result.clone(), "arith.addi", vec![lhs, rhs], vec![]);
        op.type_annotations = vec![ty];
        self.ops.push(op);
        result
    }
    /// Emit `arith.subi`.
    pub fn subi(&mut self, lhs: MlirValue, rhs: MlirValue) -> MlirValue {
        let ty = lhs.ty.clone();
        let result = self.ssa.next(ty.clone());
        let mut op = MlirOp::unary_result(result.clone(), "arith.subi", vec![lhs, rhs], vec![]);
        op.type_annotations = vec![ty];
        self.ops.push(op);
        result
    }
    /// Emit `arith.muli`.
    pub fn muli(&mut self, lhs: MlirValue, rhs: MlirValue) -> MlirValue {
        let ty = lhs.ty.clone();
        let result = self.ssa.next(ty.clone());
        let mut op = MlirOp::unary_result(result.clone(), "arith.muli", vec![lhs, rhs], vec![]);
        op.type_annotations = vec![ty];
        self.ops.push(op);
        result
    }
    /// Emit `arith.divsi` (signed integer division).
    pub fn divsi(&mut self, lhs: MlirValue, rhs: MlirValue) -> MlirValue {
        let ty = lhs.ty.clone();
        let result = self.ssa.next(ty.clone());
        let mut op = MlirOp::unary_result(result.clone(), "arith.divsi", vec![lhs, rhs], vec![]);
        op.type_annotations = vec![ty];
        self.ops.push(op);
        result
    }
    /// Emit `arith.addf`.
    pub fn addf(&mut self, lhs: MlirValue, rhs: MlirValue) -> MlirValue {
        let ty = lhs.ty.clone();
        let result = self.ssa.next(ty.clone());
        let mut op = MlirOp::unary_result(result.clone(), "arith.addf", vec![lhs, rhs], vec![]);
        op.type_annotations = vec![ty];
        self.ops.push(op);
        result
    }
    /// Emit `arith.mulf`.
    pub fn mulf(&mut self, lhs: MlirValue, rhs: MlirValue) -> MlirValue {
        let ty = lhs.ty.clone();
        let result = self.ssa.next(ty.clone());
        let mut op = MlirOp::unary_result(result.clone(), "arith.mulf", vec![lhs, rhs], vec![]);
        op.type_annotations = vec![ty];
        self.ops.push(op);
        result
    }
    /// Emit `arith.cmpi`.
    pub fn cmpi(&mut self, pred: CmpiPred, lhs: MlirValue, rhs: MlirValue) -> MlirValue {
        let result = self.ssa.next(MlirType::Integer(1, false));
        let mut op = MlirOp::unary_result(
            result.clone(),
            "arith.cmpi",
            vec![lhs.clone(), rhs],
            vec![("predicate".to_string(), MlirAttr::Str(pred.to_string()))],
        );
        op.type_annotations = vec![lhs.ty];
        self.ops.push(op);
        result
    }
    /// Emit `arith.extsi` (sign-extend integer).
    pub fn extsi(&mut self, val: MlirValue, target_bits: u32) -> MlirValue {
        let result = self.ssa.next(MlirType::Integer(target_bits, false));
        let src_ty = val.ty.clone();
        let dst_ty = result.ty.clone();
        let mut op = MlirOp::unary_result(result.clone(), "arith.extsi", vec![val], vec![]);
        op.type_annotations = vec![src_ty, dst_ty];
        self.ops.push(op);
        result
    }
    /// Emit `arith.trunci` (truncate integer).
    pub fn trunci(&mut self, val: MlirValue, target_bits: u32) -> MlirValue {
        let result = self.ssa.next(MlirType::Integer(target_bits, false));
        let src_ty = val.ty.clone();
        let dst_ty = result.ty.clone();
        let mut op = MlirOp::unary_result(result.clone(), "arith.trunci", vec![val], vec![]);
        op.type_annotations = vec![src_ty, dst_ty];
        self.ops.push(op);
        result
    }
    /// Emit `math.sin`.
    pub fn sin(&mut self, val: MlirValue) -> MlirValue {
        let ty = val.ty.clone();
        let result = self.ssa.next(ty.clone());
        let mut op = MlirOp::unary_result(result.clone(), "math.sin", vec![val], vec![]);
        op.type_annotations = vec![ty];
        self.ops.push(op);
        result
    }
    /// Emit `math.cos`.
    pub fn cos(&mut self, val: MlirValue) -> MlirValue {
        let ty = val.ty.clone();
        let result = self.ssa.next(ty.clone());
        let mut op = MlirOp::unary_result(result.clone(), "math.cos", vec![val], vec![]);
        op.type_annotations = vec![ty];
        self.ops.push(op);
        result
    }
    /// Emit `math.exp`.
    pub fn exp(&mut self, val: MlirValue) -> MlirValue {
        let ty = val.ty.clone();
        let result = self.ssa.next(ty.clone());
        let mut op = MlirOp::unary_result(result.clone(), "math.exp", vec![val], vec![]);
        op.type_annotations = vec![ty];
        self.ops.push(op);
        result
    }
    /// Emit `math.log`.
    pub fn log(&mut self, val: MlirValue) -> MlirValue {
        let ty = val.ty.clone();
        let result = self.ssa.next(ty.clone());
        let mut op = MlirOp::unary_result(result.clone(), "math.log", vec![val], vec![]);
        op.type_annotations = vec![ty];
        self.ops.push(op);
        result
    }
    /// Emit `math.sqrt`.
    pub fn sqrt(&mut self, val: MlirValue) -> MlirValue {
        let ty = val.ty.clone();
        let result = self.ssa.next(ty.clone());
        let mut op = MlirOp::unary_result(result.clone(), "math.sqrt", vec![val], vec![]);
        op.type_annotations = vec![ty];
        self.ops.push(op);
        result
    }
    /// Emit `memref.alloc`.
    pub fn alloc(&mut self, elem_ty: MlirType, dims: Vec<i64>) -> MlirValue {
        let memref_ty = MlirType::MemRef(Box::new(elem_ty), dims, AffineMap::Constant);
        let result = self.ssa.next(memref_ty.clone());
        let op = MlirOp::unary_result(result.clone(), "memref.alloc", vec![], vec![]);
        self.ops.push(op);
        result
    }
    /// Emit `memref.dealloc`.
    pub fn dealloc(&mut self, memref: MlirValue) {
        let op = MlirOp::void_op("memref.dealloc", vec![memref], vec![]);
        self.ops.push(op);
    }
    /// Emit `func.return`.
    pub fn return_op(&mut self, values: Vec<MlirValue>) {
        let op = MlirOp::void_op("func.return", values, vec![]);
        self.ops.push(op);
    }
    /// Emit `func.call`.
    pub fn call(
        &mut self,
        callee: &str,
        args: Vec<MlirValue>,
        result_types: Vec<MlirType>,
    ) -> Vec<MlirValue> {
        let results: Vec<MlirValue> = result_types.into_iter().map(|t| self.ssa.next(t)).collect();
        let mut op = MlirOp {
            results: results.clone(),
            op_name: "func.call".to_string(),
            operands: args,
            regions: vec![],
            successors: vec![],
            attributes: vec![("callee".to_string(), MlirAttr::Symbol(callee.to_string()))],
            type_annotations: vec![],
        };
        op.type_annotations = results.iter().map(|r| r.ty.clone()).collect();
        self.ops.push(op);
        results
    }
    /// Take the accumulated ops.
    pub fn take_ops(&mut self) -> Vec<MlirOp> {
        std::mem::take(&mut self.ops)
    }
    /// Build a basic block from accumulated ops.
    pub fn finish_block(&mut self, args: Vec<MlirValue>) -> MlirBlock {
        let ops = self.take_ops();
        MlirBlock::entry(args, ops)
    }
}
/// MLIR attribute representation.
#[derive(Debug, Clone, PartialEq)]
pub enum MlirAttr {
    /// Integer attribute: `42 : i64`
    Integer(i64, MlirType),
    /// Float attribute: `3.14 : f64`
    Float(f64),
    /// String attribute: `"hello"`
    Str(String),
    /// Type attribute: `i32`
    Type(MlirType),
    /// Array attribute: `[1, 2, 3]`
    Array(Vec<MlirAttr>),
    /// Dictionary attribute: `{key = val, ...}`
    Dict(Vec<(String, MlirAttr)>),
    /// Affine map: `affine_map<(d0) -> (d0)>`
    AffineMap(String),
    /// Unit attribute (presence marker)
    Unit,
    /// Boolean attribute
    Bool(bool),
    /// Symbol reference: `@name`
    Symbol(String),
    /// Dense elements: `dense<[1.0, 2.0]> : tensor<2xf32>`
    Dense(Vec<MlirAttr>, MlirType),
}
/// MLIR type system.
#[derive(Debug, Clone, PartialEq)]
pub enum MlirType {
    /// Signless integer: `i1`, `i8`, `i16`, `i32`, `i64`
    /// bool: signed = false (signless), i.e. `iN`
    /// With signed = true, displayed as `si<N>` (for annotation only)
    Integer(u32, bool),
    /// Float types: `f16`, `f32`, `f64`, `f80`, `f128`, `bf16`
    Float(u32),
    /// Index type (platform-dependent integer, pointer-sized)
    Index,
    /// MemRef type: `memref<NxMxT, affine_map>` or `memref<?xT>`
    MemRef(Box<MlirType>, Vec<i64>, AffineMap),
    /// Ranked tensor: `tensor<2x3xf32>` or `tensor<?x4xi64>`
    Tensor(Vec<i64>, Box<MlirType>),
    /// Vector type (always statically shaped): `vector<4xf32>`
    Vector(Vec<u64>, Box<MlirType>),
    /// Tuple type: `tuple<i32, f64>`
    Tuple(Vec<MlirType>),
    /// None type
    NoneType,
    /// Custom / opaque type (e.g., from external dialect)
    Custom(String),
    /// Function type: `(i32, i64) -> f32`
    FuncType(Vec<MlirType>, Vec<MlirType>),
    /// Complex type: `complex<f32>`
    Complex(Box<MlirType>),
    /// Unranked memref: `memref<*xT>`
    UnrankedMemRef(Box<MlirType>),
}
/// Liveness analysis for MLIRExt.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct MLIRExtLiveness {
    pub live_in: Vec<Vec<usize>>,
    pub live_out: Vec<Vec<usize>>,
    pub defs: Vec<Vec<usize>>,
    pub uses: Vec<Vec<usize>>,
}
impl MLIRExtLiveness {
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
/// An MLIR region (contains a list of basic blocks).
#[derive(Debug, Clone)]
pub struct MlirRegion {
    /// Blocks in this region
    pub blocks: Vec<MlirBlock>,
}
impl MlirRegion {
    /// Create a region with a single entry block.
    pub fn single_block(block: MlirBlock) -> Self {
        MlirRegion {
            blocks: vec![block],
        }
    }
    /// Create an empty region.
    pub fn empty() -> Self {
        MlirRegion { blocks: vec![] }
    }
}
/// Statistics for MLIRExt passes.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct MLIRExtPassStats {
    pub iterations: usize,
    pub changed: bool,
    pub nodes_visited: usize,
    pub nodes_modified: usize,
    pub time_ms: u64,
    pub memory_bytes: usize,
    pub errors: usize,
}
impl MLIRExtPassStats {
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
    pub fn merge(&mut self, o: &MLIRExtPassStats) {
        self.iterations += o.iterations;
        self.changed |= o.changed;
        self.nodes_visited += o.nodes_visited;
        self.nodes_modified += o.nodes_modified;
        self.time_ms += o.time_ms;
        self.memory_bytes = self.memory_bytes.max(o.memory_bytes);
        self.errors += o.errors;
    }
}
/// An MLIR basic block.
#[derive(Debug, Clone)]
pub struct MlirBlock {
    /// Block label (None for entry block)
    pub label: Option<String>,
    /// Block arguments: (value, type) pairs
    pub arguments: Vec<MlirValue>,
    /// Operations in this block
    pub body: Vec<MlirOp>,
    /// Terminator operation (explicit for clarity, also included in body)
    pub terminator: Option<MlirOp>,
}
impl MlirBlock {
    /// Create an entry block (no label).
    pub fn entry(arguments: Vec<MlirValue>, body: Vec<MlirOp>) -> Self {
        MlirBlock {
            label: None,
            arguments,
            body,
            terminator: None,
        }
    }
    /// Create a labeled block.
    pub fn labeled(label: impl Into<String>, arguments: Vec<MlirValue>, body: Vec<MlirOp>) -> Self {
        MlirBlock {
            label: Some(label.into()),
            arguments,
            body,
            terminator: None,
        }
    }
}
/// Worklist for MLIRExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct MLIRExtWorklist {
    pub(super) items: std::collections::VecDeque<usize>,
    pub(super) present: Vec<bool>,
}
impl MLIRExtWorklist {
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
/// A single MLIR operation.
#[derive(Debug, Clone)]
pub struct MlirOp {
    /// SSA result values (may be empty for side-effecting ops)
    pub results: Vec<MlirValue>,
    /// Operation name: `arith.addi`, `func.return`, etc.
    pub op_name: String,
    /// Operands (SSA values used by this op)
    pub operands: Vec<MlirValue>,
    /// Nested regions (for scf.if, scf.for, func.func, etc.)
    pub regions: Vec<MlirRegion>,
    /// Successor block labels (for cf.br, cf.cond_br)
    pub successors: Vec<String>,
    /// Named attributes: `{value = 42 : i64}`
    pub attributes: Vec<(String, MlirAttr)>,
    /// Type annotations if needed (e.g., the result types for arith ops)
    pub type_annotations: Vec<MlirType>,
}
impl MlirOp {
    /// Create a simple op with one result.
    pub fn unary_result(
        result: MlirValue,
        op_name: impl Into<String>,
        operands: Vec<MlirValue>,
        attrs: Vec<(String, MlirAttr)>,
    ) -> Self {
        MlirOp {
            results: vec![result],
            op_name: op_name.into(),
            operands,
            regions: vec![],
            successors: vec![],
            attributes: attrs,
            type_annotations: vec![],
        }
    }
    /// Create a void op (no results).
    pub fn void_op(
        op_name: impl Into<String>,
        operands: Vec<MlirValue>,
        attrs: Vec<(String, MlirAttr)>,
    ) -> Self {
        MlirOp {
            results: vec![],
            op_name: op_name.into(),
            operands,
            regions: vec![],
            successors: vec![],
            attributes: attrs,
            type_annotations: vec![],
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct MLIRAnalysisCache {
    pub(super) entries: std::collections::HashMap<String, MLIRCacheEntry>,
    pub(super) max_size: usize,
    pub(super) hits: u64,
    pub(super) misses: u64,
}
impl MLIRAnalysisCache {
    #[allow(dead_code)]
    pub fn new(max_size: usize) -> Self {
        MLIRAnalysisCache {
            entries: std::collections::HashMap::new(),
            max_size,
            hits: 0,
            misses: 0,
        }
    }
    #[allow(dead_code)]
    pub fn get(&mut self, key: &str) -> Option<&MLIRCacheEntry> {
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
            MLIRCacheEntry {
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
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct MLIRPassConfig {
    pub phase: MLIRPassPhase,
    pub enabled: bool,
    pub max_iterations: u32,
    pub debug_output: bool,
    pub pass_name: String,
}
impl MLIRPassConfig {
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>, phase: MLIRPassPhase) -> Self {
        MLIRPassConfig {
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
/// Float comparison predicates for `arith.cmpf`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CmpfPred {
    /// Ordered equal
    Oeq,
    /// Ordered not equal
    One,
    /// Ordered less than
    Olt,
    /// Ordered less than or equal
    Ole,
    /// Ordered greater than
    Ogt,
    /// Ordered greater than or equal
    Oge,
    /// Unordered equal
    Ueq,
    /// Unordered not equal
    Une,
}
/// Affine map representation for MemRef and Affine dialect operations.
#[derive(Debug, Clone, PartialEq)]
pub enum AffineMap {
    /// Identity map: `(d0, d1) -> (d0, d1)`
    Identity(usize),
    /// Constant map: `() -> ()`
    Constant,
    /// Custom affine map expression string
    Custom(String),
}
/// MLIR SSA value (operand).
#[derive(Debug, Clone, PartialEq)]
pub struct MlirValue {
    /// The name/id of the SSA value (without the `%` prefix)
    pub name: String,
    /// Type of the value
    pub ty: MlirType,
}
impl MlirValue {
    /// Create a numbered SSA value.
    pub fn numbered(id: u32, ty: MlirType) -> Self {
        MlirValue {
            name: id.to_string(),
            ty,
        }
    }
    /// Create a named SSA value.
    pub fn named(name: impl Into<String>, ty: MlirType) -> Self {
        MlirValue {
            name: name.into(),
            ty,
        }
    }
}
/// Dependency graph for MLIRExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct MLIRExtDepGraph {
    pub(super) n: usize,
    pub(super) adj: Vec<Vec<usize>>,
    pub(super) rev: Vec<Vec<usize>>,
    pub(super) edge_count: usize,
}
impl MLIRExtDepGraph {
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
/// Top-level MLIR module.
#[derive(Debug, Clone)]
pub struct MlirModule {
    /// Optional module name/attribute
    pub name: Option<String>,
    /// Function definitions
    pub functions: Vec<MlirFunc>,
    /// Global variables
    pub globals: Vec<MlirGlobal>,
    /// Dialect requirements (for `mlir-opt` pass specification)
    pub required_dialects: Vec<MlirDialect>,
}
impl MlirModule {
    /// Create a new empty MLIR module.
    pub fn new() -> Self {
        MlirModule {
            name: None,
            functions: vec![],
            globals: vec![],
            required_dialects: vec![],
        }
    }
    /// Create a module with a name.
    pub fn named(name: impl Into<String>) -> Self {
        MlirModule {
            name: Some(name.into()),
            functions: vec![],
            globals: vec![],
            required_dialects: vec![],
        }
    }
    /// Add a function to the module.
    pub fn add_function(&mut self, func: MlirFunc) {
        self.functions.push(func);
    }
    /// Add a global to the module.
    pub fn add_global(&mut self, global: MlirGlobal) {
        self.globals.push(global);
    }
    /// Generate textual MLIR format.
    pub fn emit(&self) -> String {
        let mut out = String::new();
        if let Some(name) = &self.name {
            out.push_str(&format!("module @{} {{\n", name));
        } else {
            out.push_str("module {\n");
        }
        for global in &self.globals {
            out.push_str(&global.emit());
        }
        for func in &self.functions {
            out.push_str(&func.emit());
        }
        out.push_str("}\n");
        out
    }
}
/// Arithmetic comparison predicates for `arith.cmpi`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CmpiPred {
    /// Equal
    Eq,
    /// Not equal
    Ne,
    /// Signed less than
    Slt,
    /// Signed less than or equal
    Sle,
    /// Signed greater than
    Sgt,
    /// Signed greater than or equal
    Sge,
    /// Unsigned less than
    Ult,
    /// Unsigned less than or equal
    Ule,
    /// Unsigned greater than
    Ugt,
    /// Unsigned greater than or equal
    Uge,
}
/// Dominator tree for MLIRExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct MLIRExtDomTree {
    pub(super) idom: Vec<Option<usize>>,
    pub(super) children: Vec<Vec<usize>>,
    pub(super) depth: Vec<usize>,
}
impl MLIRExtDomTree {
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
/// MLIR global variable.
#[derive(Debug, Clone)]
pub struct MlirGlobal {
    /// Global name (without `@`)
    pub name: String,
    /// Type of the global
    pub ty: MlirType,
    /// Initial value (attribute)
    pub initial_value: Option<MlirAttr>,
    /// Whether this is a constant
    pub is_constant: bool,
    /// Linkage: public, private, etc.
    pub linkage: String,
}
impl MlirGlobal {
    /// Create a simple global constant.
    pub fn constant(name: impl Into<String>, ty: MlirType, value: MlirAttr) -> Self {
        MlirGlobal {
            name: name.into(),
            ty,
            initial_value: Some(value),
            is_constant: true,
            linkage: "public".to_string(),
        }
    }
    /// Emit the global as MLIR text.
    pub fn emit(&self) -> String {
        let mut out = String::new();
        out.push_str("  memref.global ");
        if self.is_constant {
            out.push_str("constant ");
        }
        out.push_str(&format!("@{} : {}", self.name, self.ty));
        if let Some(v) = &self.initial_value {
            out.push_str(&format!(" = {}", v));
        }
        out.push('\n');
        out
    }
}
/// Pass registry for MLIRExt.
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct MLIRExtPassRegistry {
    pub(super) configs: Vec<MLIRExtPassConfig>,
    pub(super) stats: Vec<MLIRExtPassStats>,
}
impl MLIRExtPassRegistry {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    #[allow(dead_code)]
    pub fn register(&mut self, c: MLIRExtPassConfig) {
        self.stats.push(MLIRExtPassStats::new());
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
    pub fn get(&self, i: usize) -> Option<&MLIRExtPassConfig> {
        self.configs.get(i)
    }
    #[allow(dead_code)]
    pub fn get_stats(&self, i: usize) -> Option<&MLIRExtPassStats> {
        self.stats.get(i)
    }
    #[allow(dead_code)]
    pub fn enabled_passes(&self) -> Vec<&MLIRExtPassConfig> {
        self.configs.iter().filter(|c| c.enabled).collect()
    }
    #[allow(dead_code)]
    pub fn passes_in_phase(&self, ph: &MLIRExtPassPhase) -> Vec<&MLIRExtPassConfig> {
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
pub struct MLIRDepGraph {
    pub(super) nodes: Vec<u32>,
    pub(super) edges: Vec<(u32, u32)>,
}
impl MLIRDepGraph {
    #[allow(dead_code)]
    pub fn new() -> Self {
        MLIRDepGraph {
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
#[allow(dead_code)]
pub struct MLIRPassRegistry {
    pub(super) configs: Vec<MLIRPassConfig>,
    pub(super) stats: std::collections::HashMap<String, MLIRPassStats>,
}
impl MLIRPassRegistry {
    #[allow(dead_code)]
    pub fn new() -> Self {
        MLIRPassRegistry {
            configs: Vec::new(),
            stats: std::collections::HashMap::new(),
        }
    }
    #[allow(dead_code)]
    pub fn register(&mut self, config: MLIRPassConfig) {
        self.stats
            .insert(config.pass_name.clone(), MLIRPassStats::new());
        self.configs.push(config);
    }
    #[allow(dead_code)]
    pub fn enabled_passes(&self) -> Vec<&MLIRPassConfig> {
        self.configs.iter().filter(|c| c.enabled).collect()
    }
    #[allow(dead_code)]
    pub fn get_stats(&self, name: &str) -> Option<&MLIRPassStats> {
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
pub struct MLIRConstantFoldingHelper;
impl MLIRConstantFoldingHelper {
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
