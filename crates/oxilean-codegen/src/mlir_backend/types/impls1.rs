use std::collections::HashMap;
use std::collections::{HashSet, VecDeque};

use super::defs::*;

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
