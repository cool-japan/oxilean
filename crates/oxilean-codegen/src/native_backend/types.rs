//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;
use std::collections::{HashMap, HashSet};

use super::functions::*;
use std::collections::VecDeque;

/// A unique identifier for a basic block.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct BlockId(pub u32);
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct NatLivenessInfo {
    pub live_in: Vec<std::collections::HashSet<u32>>,
    pub live_out: Vec<std::collections::HashSet<u32>>,
    pub defs: Vec<std::collections::HashSet<u32>>,
    pub uses: Vec<std::collections::HashSet<u32>>,
}
impl NatLivenessInfo {
    #[allow(dead_code)]
    pub fn new(block_count: usize) -> Self {
        NatLivenessInfo {
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
pub struct NatConstantFoldingHelper;
impl NatConstantFoldingHelper {
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
/// A compiled native module.
#[derive(Debug, Clone)]
pub struct NativeModule {
    /// Functions in this module.
    pub functions: Vec<NativeFunc>,
    /// Global variables.
    pub globals: Vec<(String, NativeType, Option<i64>)>,
    /// External function declarations.
    pub extern_fns: Vec<(String, Vec<NativeType>, NativeType)>,
    /// Module name.
    pub name: String,
}
impl NativeModule {
    /// Create a new empty module.
    pub(super) fn new(name: &str) -> Self {
        NativeModule {
            functions: Vec::new(),
            globals: Vec::new(),
            extern_fns: Vec::new(),
            name: name.to_string(),
        }
    }
    /// Total number of instructions in the module.
    pub fn total_instructions(&self) -> usize {
        self.functions.iter().map(|f| f.instruction_count()).sum()
    }
    /// Find a function by name.
    pub fn get_function(&self, name: &str) -> Option<&NativeFunc> {
        self.functions.iter().find(|f| f.name == name)
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct NatDepGraph {
    pub(super) nodes: Vec<u32>,
    pub(super) edges: Vec<(u32, u32)>,
}
impl NatDepGraph {
    #[allow(dead_code)]
    pub fn new() -> Self {
        NatDepGraph {
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
#[derive(Debug, Clone)]
pub struct NatDominatorTree {
    pub idom: Vec<Option<u32>>,
    pub dom_children: Vec<Vec<u32>>,
    pub dom_depth: Vec<u32>,
}
impl NatDominatorTree {
    #[allow(dead_code)]
    pub fn new(size: usize) -> Self {
        NatDominatorTree {
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
/// Configuration for the native backend.
#[derive(Debug, Clone)]
pub struct NativeEmitConfig {
    /// Optimization level: 0 = none, 1 = basic, 2 = full, 3 = aggressive.
    pub opt_level: u8,
    /// Whether to generate debug info.
    pub debug_info: bool,
    /// Target architecture hint.
    pub target_arch: String,
    /// Number of general-purpose registers available.
    pub num_gp_regs: usize,
    /// Whether to emit comments in the IR.
    pub emit_comments: bool,
}
/// A register in the native IR (SSA form).
///
/// Registers numbered 0..65535 are virtual (pre-allocation).
/// Registers >= 65536 are physical (post-allocation).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Register(pub u32);

/// The boundary between virtual and physical register indices.
pub(super) const VIRT_PHYS_BOUNDARY: u32 = 65536;

impl Register {
    /// Whether this is a virtual register (pre-allocation).
    pub fn is_virtual(&self) -> bool {
        self.0 < VIRT_PHYS_BOUNDARY
    }
    /// Whether this is a physical register (post-allocation).
    pub fn is_physical(&self) -> bool {
        self.0 >= VIRT_PHYS_BOUNDARY
    }
    /// Create a virtual register.
    pub fn virt(n: u32) -> Self {
        assert!(
            n < VIRT_PHYS_BOUNDARY,
            "Virtual register index must be < {}",
            VIRT_PHYS_BOUNDARY
        );
        Register(n)
    }
    /// Create a physical register.
    pub fn phys(n: u32) -> Self {
        Register(n + VIRT_PHYS_BOUNDARY)
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct NatPassStats {
    pub total_runs: u32,
    pub successful_runs: u32,
    pub total_changes: u64,
    pub time_ms: u64,
    pub iterations_used: u32,
}
impl NatPassStats {
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
/// The native code generation backend.
///
/// Compiles LCNF modules to native IR suitable for JIT or AOT compilation.
pub struct NativeBackend {
    pub(super) config: NativeEmitConfig,
    pub(super) stats: NativeEmitStats,
    /// Next virtual register to allocate.
    pub(super) next_vreg: u32,
    /// Next block ID to allocate.
    pub(super) next_block: u32,
    /// Mapping from LCNF variable IDs to native registers.
    pub(super) var_map: HashMap<LcnfVarId, Register>,
    /// Next stack slot.
    pub(super) next_stack_slot: u32,
}
impl NativeBackend {
    /// Create a new native backend.
    pub fn new(config: NativeEmitConfig) -> Self {
        NativeBackend {
            config,
            stats: NativeEmitStats::default(),
            next_vreg: 0,
            next_block: 0,
            var_map: HashMap::new(),
            next_stack_slot: 0,
        }
    }
    /// Create a native backend with default configuration.
    pub fn default_backend() -> Self {
        Self::new(NativeEmitConfig::default())
    }
    /// Allocate a fresh virtual register.
    pub(super) fn alloc_vreg(&mut self) -> Register {
        let r = Register::virt(self.next_vreg);
        self.next_vreg += 1;
        self.stats.virtual_regs_used += 1;
        r
    }
    /// Allocate a fresh block ID.
    pub(super) fn alloc_block(&mut self) -> BlockId {
        let id = BlockId(self.next_block);
        self.next_block += 1;
        self.stats.blocks_generated += 1;
        id
    }
    /// Allocate a stack slot.
    pub(super) fn alloc_stack_slot(&mut self) -> u32 {
        let slot = self.next_stack_slot;
        self.next_stack_slot += 1;
        self.stats.stack_slots_allocated += 1;
        slot
    }
    /// Reset per-function state.
    pub(super) fn reset_function_state(&mut self) {
        self.next_vreg = 0;
        self.next_block = 0;
        self.var_map.clear();
        self.next_stack_slot = 0;
    }
    /// Get or allocate a register for an LCNF variable.
    pub(super) fn get_var_reg(&mut self, id: LcnfVarId) -> Register {
        if let Some(r) = self.var_map.get(&id) {
            *r
        } else {
            let r = self.alloc_vreg();
            self.var_map.insert(id, r);
            r
        }
    }
    /// Compile an entire LCNF module to native IR.
    pub fn compile_module(&mut self, module: &LcnfModule) -> NativeModule {
        let mut native_module = NativeModule::new(&module.name);
        for ext in &module.extern_decls {
            let params: Vec<NativeType> = ext
                .params
                .iter()
                .filter(|p| !p.erased)
                .map(|p| lcnf_type_to_native(&p.ty))
                .collect();
            let ret = lcnf_type_to_native(&ext.ret_type);
            native_module
                .extern_fns
                .push((ext.name.clone(), params, ret));
        }
        for fun in &module.fun_decls {
            let native_func = self.compile_fun_decl(fun);
            native_module.functions.push(native_func);
            self.stats.functions_compiled += 1;
        }
        native_module
    }
    /// Compile a single LCNF function declaration.
    pub fn compile_fun_decl(&mut self, decl: &LcnfFunDecl) -> NativeFunc {
        self.reset_function_state();
        let mut params = Vec::new();
        for p in &decl.params {
            if p.erased {
                continue;
            }
            let r = self.alloc_vreg();
            self.var_map.insert(p.id, r);
            params.push((r, lcnf_type_to_native(&p.ty)));
        }
        let ret_type = lcnf_type_to_native(&decl.ret_type);
        let mut func = NativeFunc::new(&decl.name, params, ret_type);
        func.is_recursive = decl.is_recursive;
        let entry_id = self.alloc_block();
        let mut entry_block = BasicBlock::new(entry_id);
        if self.config.emit_comments {
            entry_block.push_inst(NativeInst::Comment(format!("Function: {}", decl.name)));
        }
        let mut extra_blocks = Vec::new();
        self.compile_expr(&decl.body, &mut entry_block, &mut extra_blocks, &ret_type);
        func.blocks.push(entry_block);
        func.blocks.extend(extra_blocks);
        func.stack_size = self.next_stack_slot as usize * 8;
        func
    }
    /// Compile an LCNF expression, appending instructions to the current block.
    /// May create additional basic blocks for control flow.
    pub(super) fn compile_expr(
        &mut self,
        expr: &LcnfExpr,
        current_block: &mut BasicBlock,
        extra_blocks: &mut Vec<BasicBlock>,
        ret_type: &NativeType,
    ) {
        match expr {
            LcnfExpr::Let {
                id,
                ty,
                value,
                body,
                ..
            } => {
                let native_ty = lcnf_type_to_native(ty);
                let dst = self.alloc_vreg();
                self.var_map.insert(*id, dst);
                self.compile_let_value(value, dst, &native_ty, current_block);
                self.compile_expr(body, current_block, extra_blocks, ret_type);
            }
            LcnfExpr::Case {
                scrutinee,
                alts,
                default,
                ..
            } => {
                let scrut_reg = self.get_var_reg(*scrutinee);
                if alts.is_empty() {
                    if let Some(def) = default {
                        self.compile_expr(def, current_block, extra_blocks, ret_type);
                    } else {
                        current_block.push_inst(NativeInst::Ret { value: None });
                    }
                    return;
                }
                let merge_id = self.alloc_block();
                let merge_result_reg = self.alloc_vreg();
                let mut merge_block = BasicBlock::new(merge_id);
                if *ret_type != NativeType::Void {
                    merge_block.params.push((merge_result_reg, *ret_type));
                }
                if alts.len() == 1 && default.is_some() {
                    let alt = &alts[0];
                    let tag_reg = self.alloc_vreg();
                    current_block.push_inst(NativeInst::Call {
                        dst: Some(tag_reg),
                        func: NativeValue::FRef("lean_obj_tag".to_string()),
                        args: vec![NativeValue::Reg(scrut_reg)],
                        ret_type: NativeType::I32,
                    });
                    let cmp_reg = self.alloc_vreg();
                    current_block.push_inst(NativeInst::Cmp {
                        dst: cmp_reg,
                        cc: CondCode::Eq,
                        ty: NativeType::I32,
                        lhs: NativeValue::Reg(tag_reg),
                        rhs: NativeValue::Imm(alt.ctor_tag as i64),
                    });
                    let then_id = self.alloc_block();
                    let else_id = self.alloc_block();
                    current_block.push_inst(NativeInst::CondBr {
                        cond: NativeValue::Reg(cmp_reg),
                        then_target: then_id,
                        else_target: else_id,
                    });
                    let mut then_block = BasicBlock::new(then_id);
                    self.bind_ctor_params(&alt.params, scrut_reg, &mut then_block);
                    self.compile_expr(&alt.body, &mut then_block, extra_blocks, ret_type);
                    if then_block.terminator.is_none() {
                        then_block.push_inst(NativeInst::Br { target: merge_id });
                    }
                    extra_blocks.push(then_block);
                    let mut else_block = BasicBlock::new(else_id);
                    self.compile_expr(
                        default
                            .as_ref()
                            .expect(
                                "default is Some; guaranteed by default.is_some() check at if condition",
                            ),
                        &mut else_block,
                        extra_blocks,
                        ret_type,
                    );
                    if else_block.terminator.is_none() {
                        else_block.push_inst(NativeInst::Br { target: merge_id });
                    }
                    extra_blocks.push(else_block);
                } else {
                    let tag_reg = self.alloc_vreg();
                    current_block.push_inst(NativeInst::Call {
                        dst: Some(tag_reg),
                        func: NativeValue::FRef("lean_obj_tag".to_string()),
                        args: vec![NativeValue::Reg(scrut_reg)],
                        ret_type: NativeType::I32,
                    });
                    let default_id = self.alloc_block();
                    let mut targets = Vec::new();
                    for alt in alts {
                        let alt_block_id = self.alloc_block();
                        targets.push((alt.ctor_tag as u64, alt_block_id));
                        let mut alt_block = BasicBlock::new(alt_block_id);
                        self.bind_ctor_params(&alt.params, scrut_reg, &mut alt_block);
                        self.compile_expr(&alt.body, &mut alt_block, extra_blocks, ret_type);
                        if alt_block.terminator.is_none() {
                            alt_block.push_inst(NativeInst::Br { target: merge_id });
                        }
                        extra_blocks.push(alt_block);
                    }
                    current_block.push_inst(NativeInst::Switch {
                        value: NativeValue::Reg(tag_reg),
                        default: default_id,
                        targets,
                    });
                    let mut def_block = BasicBlock::new(default_id);
                    if let Some(def_expr) = default {
                        self.compile_expr(def_expr, &mut def_block, extra_blocks, ret_type);
                    } else {
                        def_block.push_inst(NativeInst::Ret { value: None });
                    }
                    if def_block.terminator.is_none() {
                        def_block.push_inst(NativeInst::Br { target: merge_id });
                    }
                    extra_blocks.push(def_block);
                }
                extra_blocks.push(merge_block);
            }
            LcnfExpr::Return(arg) => {
                let val = self.compile_arg(arg, current_block);
                current_block.push_inst(NativeInst::Ret { value: Some(val) });
            }
            LcnfExpr::Unreachable => {
                current_block.push_inst(NativeInst::Call {
                    dst: None,
                    func: NativeValue::FRef("lean_internal_panic_unreachable".to_string()),
                    args: vec![],
                    ret_type: NativeType::Void,
                });
                current_block.push_inst(NativeInst::Ret { value: None });
            }
            LcnfExpr::TailCall(func, args) => {
                let func_val = self.compile_arg(func, current_block);
                let arg_vals: Vec<NativeValue> = args
                    .iter()
                    .map(|a| self.compile_arg(a, current_block))
                    .collect();
                let result_reg = self.alloc_vreg();
                current_block.push_inst(NativeInst::Call {
                    dst: Some(result_reg),
                    func: func_val,
                    args: arg_vals,
                    ret_type: *ret_type,
                });
                current_block.push_inst(NativeInst::Ret {
                    value: Some(NativeValue::Reg(result_reg)),
                });
            }
        }
    }
    /// Compile a let-value into instructions.
    pub(super) fn compile_let_value(
        &mut self,
        value: &LcnfLetValue,
        dst: Register,
        _ty: &NativeType,
        block: &mut BasicBlock,
    ) {
        match value {
            LcnfLetValue::App(func, args) => {
                let func_val = self.compile_arg_to_value(func);
                let arg_vals: Vec<NativeValue> =
                    args.iter().map(|a| self.compile_arg_to_value(a)).collect();
                block.push_inst(NativeInst::Call {
                    dst: Some(dst),
                    func: func_val,
                    args: arg_vals,
                    ret_type: NativeType::Ptr,
                });
                self.stats.instructions_generated += 1;
            }
            LcnfLetValue::Proj(_name, idx, var) => {
                let obj_reg = self.get_var_reg(*var);
                block.push_inst(NativeInst::Call {
                    dst: Some(dst),
                    func: NativeValue::FRef("lean_ctor_get".to_string()),
                    args: vec![NativeValue::Reg(obj_reg), NativeValue::Imm(*idx as i64)],
                    ret_type: NativeType::Ptr,
                });
                self.stats.instructions_generated += 1;
            }
            LcnfLetValue::Ctor(_name, tag, args) => {
                let num_objs = args.len();
                block.push_inst(NativeInst::Call {
                    dst: Some(dst),
                    func: NativeValue::FRef("lean_alloc_ctor".to_string()),
                    args: vec![
                        NativeValue::Imm(*tag as i64),
                        NativeValue::Imm(num_objs as i64),
                        NativeValue::Imm(0),
                    ],
                    ret_type: NativeType::Ptr,
                });
                self.stats.instructions_generated += 1;
                for (i, arg) in args.iter().enumerate() {
                    let val = self.compile_arg_to_value(arg);
                    block.push_inst(NativeInst::Call {
                        dst: None,
                        func: NativeValue::FRef("lean_ctor_set".to_string()),
                        args: vec![NativeValue::Reg(dst), NativeValue::Imm(i as i64), val],
                        ret_type: NativeType::Void,
                    });
                    self.stats.instructions_generated += 1;
                }
            }
            LcnfLetValue::Lit(lit) => match lit {
                LcnfLit::Nat(n) => {
                    block.push_inst(NativeInst::LoadImm {
                        dst,
                        ty: NativeType::I64,
                        value: *n as i64,
                    });
                    self.stats.instructions_generated += 1;
                }
                LcnfLit::Str(s) => {
                    block.push_inst(NativeInst::Call {
                        dst: Some(dst),
                        func: NativeValue::FRef("lean_mk_string".to_string()),
                        args: vec![NativeValue::StrRef(s.clone())],
                        ret_type: NativeType::Ptr,
                    });
                    if self.config.emit_comments {
                        block.push_inst(NativeInst::Comment(format!("string: \"{}\"", s)));
                    }
                    self.stats.instructions_generated += 1;
                }
            },
            LcnfLetValue::Erased => {
                block.push_inst(NativeInst::LoadImm {
                    dst,
                    ty: NativeType::Ptr,
                    value: 0,
                });
                self.stats.instructions_generated += 1;
            }
            LcnfLetValue::FVar(var) => {
                let src = self.get_var_reg(*var);
                block.push_inst(NativeInst::Copy {
                    dst,
                    src: NativeValue::Reg(src),
                });
                self.stats.instructions_generated += 1;
            }
            LcnfLetValue::Reset(var) => {
                let obj_reg = self.get_var_reg(*var);
                block.push_inst(NativeInst::Call {
                    dst: Some(dst),
                    func: NativeValue::FRef("lean_obj_reset".to_string()),
                    args: vec![NativeValue::Reg(obj_reg)],
                    ret_type: NativeType::Ptr,
                });
                self.stats.instructions_generated += 1;
            }
            LcnfLetValue::Reuse(slot, _name, tag, args) => {
                let num_objs = args.len();
                let slot_reg = self.get_var_reg(*slot);
                block.push_inst(NativeInst::Call {
                    dst: Some(dst),
                    func: NativeValue::FRef("lean_alloc_ctor_using".to_string()),
                    args: vec![
                        NativeValue::Reg(slot_reg),
                        NativeValue::Imm(*tag as i64),
                        NativeValue::Imm(num_objs as i64),
                        NativeValue::Imm(0),
                    ],
                    ret_type: NativeType::Ptr,
                });
                self.stats.instructions_generated += 1;
                for (i, arg) in args.iter().enumerate() {
                    let val = self.compile_arg_to_value(arg);
                    block.push_inst(NativeInst::Call {
                        dst: None,
                        func: NativeValue::FRef("lean_ctor_set".to_string()),
                        args: vec![NativeValue::Reg(dst), NativeValue::Imm(i as i64), val],
                        ret_type: NativeType::Void,
                    });
                    self.stats.instructions_generated += 1;
                }
            }
        }
    }
    /// Bind constructor parameters to registers by extracting fields.
    pub(super) fn bind_ctor_params(
        &mut self,
        params: &[LcnfParam],
        scrut_reg: Register,
        block: &mut BasicBlock,
    ) {
        for (i, param) in params.iter().enumerate() {
            if param.erased {
                continue;
            }
            let dst = self.alloc_vreg();
            self.var_map.insert(param.id, dst);
            block.push_inst(NativeInst::Call {
                dst: Some(dst),
                func: NativeValue::FRef("lean_ctor_get".to_string()),
                args: vec![NativeValue::Reg(scrut_reg), NativeValue::Imm(i as i64)],
                ret_type: NativeType::Ptr,
            });
            self.stats.instructions_generated += 1;
        }
    }
    /// Compile an LCNF argument to a NativeValue, possibly emitting instructions.
    pub(super) fn compile_arg(&mut self, arg: &LcnfArg, block: &mut BasicBlock) -> NativeValue {
        match arg {
            LcnfArg::Var(id) => {
                let r = self.get_var_reg(*id);
                NativeValue::Reg(r)
            }
            LcnfArg::Lit(lit) => match lit {
                LcnfLit::Nat(n) => {
                    let r = self.alloc_vreg();
                    block.push_inst(NativeInst::LoadImm {
                        dst: r,
                        ty: NativeType::I64,
                        value: *n as i64,
                    });
                    self.stats.instructions_generated += 1;
                    NativeValue::Reg(r)
                }
                LcnfLit::Str(s) => {
                    let r = self.alloc_vreg();
                    block.push_inst(NativeInst::Call {
                        dst: Some(r),
                        func: NativeValue::FRef("lean_mk_string".to_string()),
                        args: vec![NativeValue::StrRef(s.clone())],
                        ret_type: NativeType::Ptr,
                    });
                    self.stats.instructions_generated += 1;
                    NativeValue::Reg(r)
                }
            },
            LcnfArg::Erased | LcnfArg::Type(_) => NativeValue::Imm(0),
        }
    }
    /// Compile an LCNF argument to a NativeValue without emitting instructions.
    pub(super) fn compile_arg_to_value(&mut self, arg: &LcnfArg) -> NativeValue {
        match arg {
            LcnfArg::Var(id) => NativeValue::Reg(self.get_var_reg(*id)),
            LcnfArg::Lit(LcnfLit::Nat(n)) => NativeValue::Imm(*n as i64),
            LcnfArg::Lit(LcnfLit::Str(s)) => NativeValue::StrRef(s.clone()),
            LcnfArg::Erased | LcnfArg::Type(_) => NativeValue::Imm(0),
        }
    }
    /// Get accumulated statistics.
    pub fn stats(&self) -> &NativeEmitStats {
        &self.stats
    }
}
/// Live interval for a virtual register.
#[derive(Debug, Clone)]
struct LiveInterval {
    pub(super) vreg: Register,
    pub(super) start: usize,
    pub(super) end: usize,
    pub(super) assigned_phys: Option<Register>,
    pub(super) spill_slot: Option<u32>,
}
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum NatPassPhase {
    Analysis,
    Transformation,
    Verification,
    Cleanup,
}
impl NatPassPhase {
    #[allow(dead_code)]
    pub fn name(&self) -> &str {
        match self {
            NatPassPhase::Analysis => "analysis",
            NatPassPhase::Transformation => "transformation",
            NatPassPhase::Verification => "verification",
            NatPassPhase::Cleanup => "cleanup",
        }
    }
    #[allow(dead_code)]
    pub fn is_modifying(&self) -> bool {
        matches!(self, NatPassPhase::Transformation | NatPassPhase::Cleanup)
    }
}
#[allow(dead_code)]
pub struct NatPassRegistry {
    pub(super) configs: Vec<NatPassConfig>,
    pub(super) stats: std::collections::HashMap<String, NatPassStats>,
}
impl NatPassRegistry {
    #[allow(dead_code)]
    pub fn new() -> Self {
        NatPassRegistry {
            configs: Vec::new(),
            stats: std::collections::HashMap::new(),
        }
    }
    #[allow(dead_code)]
    pub fn register(&mut self, config: NatPassConfig) {
        self.stats
            .insert(config.pass_name.clone(), NatPassStats::new());
        self.configs.push(config);
    }
    #[allow(dead_code)]
    pub fn enabled_passes(&self) -> Vec<&NatPassConfig> {
        self.configs.iter().filter(|c| c.enabled).collect()
    }
    #[allow(dead_code)]
    pub fn get_stats(&self, name: &str) -> Option<&NatPassStats> {
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
/// A value operand in the native IR.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NativeValue {
    /// A register (virtual or physical).
    Reg(Register),
    /// An immediate constant.
    Imm(i64),
    /// A function reference (by name).
    FRef(String),
    /// A stack slot.
    StackSlot(u32),
    /// An unsigned immediate.
    UImm(u64),
    /// A string literal reference (pointer to the string bytes).
    StrRef(String),
}
impl NativeValue {
    /// Create a register value.
    pub(super) fn reg(r: Register) -> Self {
        NativeValue::Reg(r)
    }
    /// Create an immediate value.
    pub(super) fn imm(n: i64) -> Self {
        NativeValue::Imm(n)
    }
    /// Create an unsigned immediate value.
    pub(super) fn uimm(n: u64) -> Self {
        NativeValue::UImm(n)
    }
}
/// A basic block in the native IR.
#[derive(Debug, Clone)]
pub struct BasicBlock {
    /// The block identifier.
    pub label: BlockId,
    /// Block parameters (for SSA phi-elimination).
    pub params: Vec<(Register, NativeType)>,
    /// The instructions in this block (excluding the terminator).
    pub instructions: Vec<NativeInst>,
    /// The terminator instruction.
    pub terminator: Option<NativeInst>,
}
impl BasicBlock {
    /// Create a new empty basic block.
    pub(super) fn new(label: BlockId) -> Self {
        BasicBlock {
            label,
            params: Vec::new(),
            instructions: Vec::new(),
            terminator: None,
        }
    }
    /// Append an instruction to this block.
    pub(super) fn push_inst(&mut self, inst: NativeInst) {
        if inst.is_terminator() {
            self.terminator = Some(inst);
        } else {
            self.instructions.push(inst);
        }
    }
    /// Get the successor block IDs of this block.
    pub fn successors(&self) -> Vec<BlockId> {
        match &self.terminator {
            Some(NativeInst::Br { target }) => vec![*target],
            Some(NativeInst::CondBr {
                then_target,
                else_target,
                ..
            }) => {
                vec![*then_target, *else_target]
            }
            Some(NativeInst::Switch {
                default, targets, ..
            }) => {
                let mut succs = vec![*default];
                for (_, t) in targets {
                    succs.push(*t);
                }
                succs
            }
            _ => vec![],
        }
    }
    /// Total number of instructions (including terminator).
    pub fn instruction_count(&self) -> usize {
        self.instructions.len() + if self.terminator.is_some() { 1 } else { 0 }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct NatCacheEntry {
    pub key: String,
    pub data: Vec<u8>,
    pub timestamp: u64,
    pub valid: bool,
}
/// A function in the native IR.
#[derive(Debug, Clone)]
pub struct NativeFunc {
    /// Function name.
    pub name: String,
    /// Parameter registers and types.
    pub params: Vec<(Register, NativeType)>,
    /// Return type.
    pub ret_type: NativeType,
    /// Basic blocks (entry is blocks\[0\]).
    pub blocks: Vec<BasicBlock>,
    /// Total stack size needed.
    pub stack_size: usize,
    /// Whether this function is recursive.
    pub is_recursive: bool,
}
impl NativeFunc {
    /// Create a new function.
    pub(super) fn new(
        name: &str,
        params: Vec<(Register, NativeType)>,
        ret_type: NativeType,
    ) -> Self {
        NativeFunc {
            name: name.to_string(),
            params,
            ret_type,
            blocks: Vec::new(),
            stack_size: 0,
            is_recursive: false,
        }
    }
    /// Get the entry block.
    pub fn entry_block(&self) -> Option<&BasicBlock> {
        self.blocks.first()
    }
    /// Total number of instructions across all blocks.
    pub fn instruction_count(&self) -> usize {
        self.blocks.iter().map(|b| b.instruction_count()).sum()
    }
    /// Get a block by ID.
    pub fn get_block(&self, id: BlockId) -> Option<&BasicBlock> {
        self.blocks.iter().find(|b| b.label == id)
    }
    /// Get all virtual registers used in this function.
    pub fn virtual_registers(&self) -> HashSet<Register> {
        let mut regs = HashSet::new();
        for (r, _) in &self.params {
            if r.is_virtual() {
                regs.insert(*r);
            }
        }
        for block in &self.blocks {
            for (r, _) in &block.params {
                if r.is_virtual() {
                    regs.insert(*r);
                }
            }
            for inst in &block.instructions {
                if let Some(dst) = inst.dst_reg() {
                    if dst.is_virtual() {
                        regs.insert(dst);
                    }
                }
                for src in inst.src_regs() {
                    if src.is_virtual() {
                        regs.insert(src);
                    }
                }
            }
            if let Some(term) = &block.terminator {
                if let Some(dst) = term.dst_reg() {
                    if dst.is_virtual() {
                        regs.insert(dst);
                    }
                }
                for src in term.src_regs() {
                    if src.is_virtual() {
                        regs.insert(src);
                    }
                }
            }
        }
        regs
    }
}
/// A single instruction in the native IR.
#[derive(Debug, Clone, PartialEq)]
pub enum NativeInst {
    /// Load immediate: `dst = imm`.
    LoadImm {
        dst: Register,
        ty: NativeType,
        value: i64,
    },
    /// Add: `dst = lhs + rhs`.
    Add {
        dst: Register,
        ty: NativeType,
        lhs: NativeValue,
        rhs: NativeValue,
    },
    /// Subtract: `dst = lhs - rhs`.
    Sub {
        dst: Register,
        ty: NativeType,
        lhs: NativeValue,
        rhs: NativeValue,
    },
    /// Multiply: `dst = lhs * rhs`.
    Mul {
        dst: Register,
        ty: NativeType,
        lhs: NativeValue,
        rhs: NativeValue,
    },
    /// Divide: `dst = lhs / rhs`.
    Div {
        dst: Register,
        ty: NativeType,
        lhs: NativeValue,
        rhs: NativeValue,
    },
    /// Bitwise AND: `dst = lhs & rhs`.
    And {
        dst: Register,
        ty: NativeType,
        lhs: NativeValue,
        rhs: NativeValue,
    },
    /// Bitwise OR: `dst = lhs | rhs`.
    Or {
        dst: Register,
        ty: NativeType,
        lhs: NativeValue,
        rhs: NativeValue,
    },
    /// Bitwise XOR: `dst = lhs ^ rhs`.
    Xor {
        dst: Register,
        ty: NativeType,
        lhs: NativeValue,
        rhs: NativeValue,
    },
    /// Shift left: `dst = lhs << rhs`.
    Shl {
        dst: Register,
        ty: NativeType,
        lhs: NativeValue,
        rhs: NativeValue,
    },
    /// Shift right (arithmetic): `dst = lhs >> rhs`.
    Shr {
        dst: Register,
        ty: NativeType,
        lhs: NativeValue,
        rhs: NativeValue,
    },
    /// Compare: `dst = cmp(cc, lhs, rhs)`.
    Cmp {
        dst: Register,
        cc: CondCode,
        ty: NativeType,
        lhs: NativeValue,
        rhs: NativeValue,
    },
    /// Unconditional branch to a block.
    Br { target: BlockId },
    /// Conditional branch: if cond != 0, goto then_target, else goto else_target.
    CondBr {
        cond: NativeValue,
        then_target: BlockId,
        else_target: BlockId,
    },
    /// Function call: `dst = func(args...)`.
    Call {
        dst: Option<Register>,
        func: NativeValue,
        args: Vec<NativeValue>,
        ret_type: NativeType,
    },
    /// Return from function.
    Ret { value: Option<NativeValue> },
    /// Load from memory: `dst = *addr`.
    Load {
        dst: Register,
        ty: NativeType,
        addr: NativeValue,
    },
    /// Store to memory: `*addr = value`.
    Store {
        ty: NativeType,
        addr: NativeValue,
        value: NativeValue,
    },
    /// Allocate stack space: `dst = alloca(size, align)`.
    Alloc {
        dst: Register,
        size: usize,
        align: usize,
    },
    /// Free heap memory.
    Free { ptr: NativeValue },
    /// Phi node (SSA): `dst = phi([(val, block), ...])`.
    Phi {
        dst: Register,
        ty: NativeType,
        incoming: Vec<(NativeValue, BlockId)>,
    },
    /// Select: `dst = cond ? true_val : false_val`.
    Select {
        dst: Register,
        ty: NativeType,
        cond: NativeValue,
        true_val: NativeValue,
        false_val: NativeValue,
    },
    /// Copy: `dst = src`.
    Copy { dst: Register, src: NativeValue },
    /// No-operation (placeholder).
    Nop,
    /// Comment (for debugging).
    Comment(String),
    /// Integer to pointer cast.
    IntToPtr { dst: Register, src: NativeValue },
    /// Pointer to integer cast.
    PtrToInt { dst: Register, src: NativeValue },
    /// Get element pointer (pointer arithmetic).
    GetElementPtr {
        dst: Register,
        base: NativeValue,
        offset: NativeValue,
        elem_size: usize,
    },
    /// Switch/table branch.
    Switch {
        value: NativeValue,
        default: BlockId,
        targets: Vec<(u64, BlockId)>,
    },
}
impl NativeInst {
    /// Get the destination register of this instruction, if any.
    pub fn dst_reg(&self) -> Option<Register> {
        match self {
            NativeInst::LoadImm { dst, .. }
            | NativeInst::Add { dst, .. }
            | NativeInst::Sub { dst, .. }
            | NativeInst::Mul { dst, .. }
            | NativeInst::Div { dst, .. }
            | NativeInst::And { dst, .. }
            | NativeInst::Or { dst, .. }
            | NativeInst::Xor { dst, .. }
            | NativeInst::Shl { dst, .. }
            | NativeInst::Shr { dst, .. }
            | NativeInst::Cmp { dst, .. }
            | NativeInst::Load { dst, .. }
            | NativeInst::Alloc { dst, .. }
            | NativeInst::Phi { dst, .. }
            | NativeInst::Select { dst, .. }
            | NativeInst::Copy { dst, .. }
            | NativeInst::IntToPtr { dst, .. }
            | NativeInst::PtrToInt { dst, .. }
            | NativeInst::GetElementPtr { dst, .. } => Some(*dst),
            NativeInst::Call { dst, .. } => *dst,
            _ => None,
        }
    }
    /// Get all source registers read by this instruction.
    pub fn src_regs(&self) -> Vec<Register> {
        let mut regs = Vec::new();
        let extract = |v: &NativeValue, regs: &mut Vec<Register>| {
            if let NativeValue::Reg(r) = v {
                regs.push(*r);
            }
        };
        match self {
            NativeInst::Add { lhs, rhs, .. }
            | NativeInst::Sub { lhs, rhs, .. }
            | NativeInst::Mul { lhs, rhs, .. }
            | NativeInst::Div { lhs, rhs, .. }
            | NativeInst::And { lhs, rhs, .. }
            | NativeInst::Or { lhs, rhs, .. }
            | NativeInst::Xor { lhs, rhs, .. }
            | NativeInst::Shl { lhs, rhs, .. }
            | NativeInst::Shr { lhs, rhs, .. }
            | NativeInst::Cmp { lhs, rhs, .. } => {
                extract(lhs, &mut regs);
                extract(rhs, &mut regs);
            }
            NativeInst::CondBr { cond, .. } => extract(cond, &mut regs),
            NativeInst::Call { func, args, .. } => {
                extract(func, &mut regs);
                for a in args {
                    extract(a, &mut regs);
                }
            }
            NativeInst::Ret { value: Some(v) } => extract(v, &mut regs),
            NativeInst::Load { addr, .. } => extract(addr, &mut regs),
            NativeInst::Store { addr, value, .. } => {
                extract(addr, &mut regs);
                extract(value, &mut regs);
            }
            NativeInst::Free { ptr } => extract(ptr, &mut regs),
            NativeInst::Phi { incoming, .. } => {
                for (v, _) in incoming {
                    extract(v, &mut regs);
                }
            }
            NativeInst::Select {
                cond,
                true_val,
                false_val,
                ..
            } => {
                extract(cond, &mut regs);
                extract(true_val, &mut regs);
                extract(false_val, &mut regs);
            }
            NativeInst::Copy { src, .. } => extract(src, &mut regs),
            NativeInst::IntToPtr { src, .. } | NativeInst::PtrToInt { src, .. } => {
                extract(src, &mut regs)
            }
            NativeInst::GetElementPtr { base, offset, .. } => {
                extract(base, &mut regs);
                extract(offset, &mut regs);
            }
            NativeInst::Switch { value, .. } => extract(value, &mut regs),
            _ => {}
        }
        regs
    }
    /// Whether this instruction is a terminator (ends a basic block).
    pub fn is_terminator(&self) -> bool {
        matches!(
            self,
            NativeInst::Br { .. }
                | NativeInst::CondBr { .. }
                | NativeInst::Ret { .. }
                | NativeInst::Switch { .. }
        )
    }
}
/// Statistics from native code generation.
#[derive(Debug, Clone, Default)]
pub struct NativeEmitStats {
    /// Number of functions compiled.
    pub functions_compiled: usize,
    /// Number of basic blocks generated.
    pub blocks_generated: usize,
    /// Number of instructions generated.
    pub instructions_generated: usize,
    /// Number of virtual registers used.
    pub virtual_regs_used: usize,
    /// Number of stack slots allocated.
    pub stack_slots_allocated: usize,
    /// Number of spills during register allocation.
    pub spills: usize,
}
/// Machine-level types for the native IR.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NativeType {
    /// 8-bit integer.
    I8,
    /// 16-bit integer.
    I16,
    /// 32-bit integer.
    I32,
    /// 64-bit integer.
    I64,
    /// 32-bit floating point.
    F32,
    /// 64-bit floating point.
    F64,
    /// Pointer (word-sized).
    Ptr,
    /// Function reference.
    FuncRef,
    /// Void (no value).
    Void,
}
impl NativeType {
    /// Size in bytes of this type.
    pub fn size_bytes(&self) -> usize {
        match self {
            NativeType::I8 => 1,
            NativeType::I16 => 2,
            NativeType::I32 | NativeType::F32 => 4,
            NativeType::I64 | NativeType::F64 | NativeType::Ptr | NativeType::FuncRef => 8,
            NativeType::Void => 0,
        }
    }
    /// Alignment in bytes.
    pub fn alignment(&self) -> usize {
        self.size_bytes().max(1)
    }
    /// Whether this is a floating-point type.
    pub fn is_float(&self) -> bool {
        matches!(self, NativeType::F32 | NativeType::F64)
    }
    /// Whether this is an integer type.
    pub fn is_integer(&self) -> bool {
        matches!(
            self,
            NativeType::I8 | NativeType::I16 | NativeType::I32 | NativeType::I64
        )
    }
    /// Whether this is a pointer type.
    pub fn is_pointer(&self) -> bool {
        matches!(self, NativeType::Ptr | NativeType::FuncRef)
    }
}
/// Comparison condition code.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CondCode {
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    /// Unsigned less than.
    Ult,
    /// Unsigned less or equal.
    Ule,
    /// Unsigned greater than.
    Ugt,
    /// Unsigned greater or equal.
    Uge,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct NatPassConfig {
    pub phase: NatPassPhase,
    pub enabled: bool,
    pub max_iterations: u32,
    pub debug_output: bool,
    pub pass_name: String,
}
impl NatPassConfig {
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>, phase: NatPassPhase) -> Self {
        NatPassConfig {
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
pub struct NatAnalysisCache {
    pub(super) entries: std::collections::HashMap<String, NatCacheEntry>,
    pub(super) max_size: usize,
    pub(super) hits: u64,
    pub(super) misses: u64,
}
impl NatAnalysisCache {
    #[allow(dead_code)]
    pub fn new(max_size: usize) -> Self {
        NatAnalysisCache {
            entries: std::collections::HashMap::new(),
            max_size,
            hits: 0,
            misses: 0,
        }
    }
    #[allow(dead_code)]
    pub fn get(&mut self, key: &str) -> Option<&NatCacheEntry> {
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
            NatCacheEntry {
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
/// Linear scan register allocator.
///
/// Takes a NativeFunc with virtual registers and assigns physical registers
/// using a simple linear scan algorithm.
pub struct RegisterAllocator {
    /// Number of available physical registers.
    pub(super) num_phys_regs: usize,
    /// Live intervals computed during analysis.
    pub(super) intervals: Vec<LiveInterval>,
    /// Active intervals (sorted by end point).
    pub(super) active: Vec<usize>,
    /// Next spill slot to allocate.
    pub(super) next_spill: u32,
    /// Set of free physical registers.
    pub(super) free_regs: Vec<bool>,
    /// Total number of spills performed.
    pub(super) spill_count: usize,
}
impl RegisterAllocator {
    /// Create a new register allocator.
    pub fn new(num_phys_regs: usize) -> Self {
        RegisterAllocator {
            num_phys_regs,
            intervals: Vec::new(),
            active: Vec::new(),
            next_spill: 0,
            free_regs: vec![true; num_phys_regs],
            spill_count: 0,
        }
    }
    /// Perform register allocation on a function.
    pub fn allocate(&mut self, func: &NativeFunc) -> HashMap<Register, Register> {
        self.intervals.clear();
        self.active.clear();
        self.free_regs = vec![true; self.num_phys_regs];
        self.spill_count = 0;
        self.compute_live_intervals(func);
        self.intervals.sort_by_key(|i| i.start);
        let mut assignment: HashMap<Register, Register> = HashMap::new();
        for idx in 0..self.intervals.len() {
            let current_start = self.intervals[idx].start;
            self.expire_old_intervals(current_start);
            if let Some(phys_idx) = self.find_free_reg() {
                self.intervals[idx].assigned_phys = Some(Register::phys(phys_idx as u32));
                self.free_regs[phys_idx] = false;
                self.active.push(idx);
                self.active.sort_by_key(|&i| self.intervals[i].end);
                assignment.insert(self.intervals[idx].vreg, Register::phys(phys_idx as u32));
            } else {
                self.spill_at_interval(idx, &mut assignment);
            }
        }
        assignment
    }
    /// Compute live intervals for all virtual registers.
    pub(super) fn compute_live_intervals(&mut self, func: &NativeFunc) {
        let mut intervals_map: HashMap<Register, (usize, usize)> = HashMap::new();
        let mut position = 0usize;
        for block in &func.blocks {
            for inst in &block.instructions {
                if let Some(dst) = inst.dst_reg() {
                    if dst.is_virtual() {
                        intervals_map
                            .entry(dst)
                            .and_modify(|(_s, e)| *e = position)
                            .or_insert((position, position));
                    }
                }
                for src in inst.src_regs() {
                    if src.is_virtual() {
                        intervals_map
                            .entry(src)
                            .and_modify(|(_s, e)| *e = position)
                            .or_insert((position, position));
                    }
                }
                position += 1;
            }
            if let Some(term) = &block.terminator {
                if let Some(dst) = term.dst_reg() {
                    if dst.is_virtual() {
                        intervals_map
                            .entry(dst)
                            .and_modify(|(_s, e)| *e = position)
                            .or_insert((position, position));
                    }
                }
                for src in term.src_regs() {
                    if src.is_virtual() {
                        intervals_map
                            .entry(src)
                            .and_modify(|(_s, e)| *e = position)
                            .or_insert((position, position));
                    }
                }
                position += 1;
            }
        }
        for (r, _) in &func.params {
            if r.is_virtual() {
                intervals_map
                    .entry(*r)
                    .and_modify(|(s, _e)| *s = 0)
                    .or_insert((0, 0));
            }
        }
        self.intervals = intervals_map
            .into_iter()
            .map(|(vreg, (start, end))| LiveInterval {
                vreg,
                start,
                end,
                assigned_phys: None,
                spill_slot: None,
            })
            .collect();
    }
    /// Expire intervals that end before the current position.
    pub(super) fn expire_old_intervals(&mut self, current_start: usize) {
        let mut to_remove = Vec::new();
        for (active_idx, &interval_idx) in self.active.iter().enumerate() {
            if self.intervals[interval_idx].end < current_start {
                if let Some(phys) = self.intervals[interval_idx].assigned_phys {
                    let phys_idx = (phys.0 - VIRT_PHYS_BOUNDARY) as usize;
                    if phys_idx < self.free_regs.len() {
                        self.free_regs[phys_idx] = true;
                    }
                }
                to_remove.push(active_idx);
            }
        }
        for idx in to_remove.into_iter().rev() {
            self.active.remove(idx);
        }
    }
    /// Find a free physical register.
    pub(super) fn find_free_reg(&self) -> Option<usize> {
        self.free_regs.iter().position(|&free| free)
    }
    /// Spill at the current interval.
    pub(super) fn spill_at_interval(
        &mut self,
        current_idx: usize,
        assignment: &mut HashMap<Register, Register>,
    ) {
        if !self.active.is_empty() {
            let last_active_idx = *self
                .active
                .last()
                .expect("active is non-empty; guaranteed by !self.active.is_empty() guard");
            if self.intervals[last_active_idx].end > self.intervals[current_idx].end {
                let phys = self
                    .intervals[last_active_idx]
                    .assigned_phys
                    .expect(
                        "last active interval must have an assigned physical register; invariant of linear scan regalloc",
                    );
                self.intervals[current_idx].assigned_phys = Some(phys);
                assignment.insert(self.intervals[current_idx].vreg, phys);
                let spill_slot = self.next_spill;
                self.next_spill += 1;
                self.intervals[last_active_idx].assigned_phys = None;
                self.intervals[last_active_idx].spill_slot = Some(spill_slot);
                assignment.remove(&self.intervals[last_active_idx].vreg);
                self.active.pop();
                self.active.push(current_idx);
                self.active.sort_by_key(|&i| self.intervals[i].end);
                self.spill_count += 1;
                return;
            }
        }
        let spill_slot = self.next_spill;
        self.next_spill += 1;
        self.intervals[current_idx].spill_slot = Some(spill_slot);
        self.spill_count += 1;
    }
    /// Get the number of spills performed.
    pub fn spill_count(&self) -> usize {
        self.spill_count
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct NatWorklist {
    pub(super) items: std::collections::VecDeque<u32>,
    pub(super) in_worklist: std::collections::HashSet<u32>,
}
impl NatWorklist {
    #[allow(dead_code)]
    pub fn new() -> Self {
        NatWorklist {
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
