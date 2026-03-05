//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use crate::lcnf::*;
use crate::native_backend::*;
use std::collections::HashMap;

/// Generates code for external (foreign) object creation and finalization.
#[allow(dead_code)]
pub struct ExternalObjectCodegen {
    pub(super) temp_counter: u32,
}
impl ExternalObjectCodegen {
    /// Create a new external object codegen.
    #[allow(dead_code)]
    pub fn new() -> Self {
        ExternalObjectCodegen { temp_counter: 0 }
    }
    pub(super) fn fresh_reg(&mut self) -> Register {
        let r = Register::virt(1300 + self.temp_counter);
        self.temp_counter += 1;
        r
    }
    /// Emit allocation of an external object with a given finalizer.
    #[allow(dead_code)]
    pub fn emit_alloc_external(
        &mut self,
        data_reg: Register,
        finalizer_fn: &str,
    ) -> Vec<NativeInst> {
        let dst = self.fresh_reg();
        vec![
            NativeInst::Comment(format!(
                "Alloc external data={} finalizer={}",
                data_reg, finalizer_fn
            )),
            NativeInst::Call {
                dst: Some(dst),
                func: NativeValue::FRef("lean_alloc_external".to_string()),
                args: vec![
                    NativeValue::Reg(data_reg),
                    NativeValue::FRef(finalizer_fn.to_string()),
                ],
                ret_type: NativeType::Ptr,
            },
        ]
    }
    /// Emit extraction of the raw data pointer from an external object.
    #[allow(dead_code)]
    pub fn emit_get_external_data(&mut self, obj_reg: Register) -> Vec<NativeInst> {
        let dst = self.fresh_reg();
        vec![
            NativeInst::Comment(format!("Get external data {}", obj_reg)),
            NativeInst::Load {
                dst,
                ty: NativeType::Ptr,
                addr: NativeValue::Reg(obj_reg),
            },
        ]
    }
}
/// Generates code for closure creation, application, and partial application.
pub struct ClosureCodegen {
    /// Counter for temporaries.
    pub(super) temp_counter: u32,
}
impl ClosureCodegen {
    pub fn new() -> Self {
        ClosureCodegen { temp_counter: 0 }
    }
    pub(super) fn fresh_reg(&mut self) -> Register {
        let r = Register::virt(800 + self.temp_counter);
        self.temp_counter += 1;
        r
    }
    /// Emit instructions to create a closure.
    pub fn emit_closure_create(
        &mut self,
        fn_name: &str,
        arity: usize,
        env_regs: &[Register],
    ) -> Vec<NativeInst> {
        let mut insts = Vec::new();
        let closure_reg = self.fresh_reg();
        insts.push(NativeInst::Comment(format!(
            "Create closure @{} arity={} captured={}",
            fn_name,
            arity,
            env_regs.len()
        )));
        insts.push(NativeInst::Call {
            dst: Some(closure_reg),
            func: NativeValue::FRef("lean_alloc_closure".to_string()),
            args: vec![
                NativeValue::FRef(fn_name.to_string()),
                NativeValue::Imm(arity as i64),
                NativeValue::Imm(env_regs.len() as i64),
            ],
            ret_type: NativeType::Ptr,
        });
        for (i, env_reg) in env_regs.iter().enumerate() {
            insts.push(NativeInst::Call {
                dst: None,
                func: NativeValue::FRef("lean_closure_set".to_string()),
                args: vec![
                    NativeValue::Reg(closure_reg),
                    NativeValue::Imm(i as i64),
                    NativeValue::Reg(*env_reg),
                ],
                ret_type: NativeType::Void,
            });
        }
        insts
    }
    /// Emit instructions to apply a closure to arguments.
    pub fn emit_closure_apply(
        &mut self,
        closure_reg: Register,
        arg_regs: &[Register],
    ) -> Vec<NativeInst> {
        let mut insts = Vec::new();
        let result_reg = self.fresh_reg();
        insts.push(NativeInst::Comment(format!(
            "Apply closure {} with {} args",
            closure_reg,
            arg_regs.len()
        )));
        let apply_fn = match arg_regs.len() {
            1 => "lean_apply_1",
            2 => "lean_apply_2",
            3 => "lean_apply_3",
            4 => "lean_apply_4",
            _ => "lean_apply_n",
        };
        let mut args = vec![NativeValue::Reg(closure_reg)];
        for r in arg_regs {
            args.push(NativeValue::Reg(*r));
        }
        if arg_regs.len() > 4 {
            args.push(NativeValue::Imm(arg_regs.len() as i64));
        }
        insts.push(NativeInst::Call {
            dst: Some(result_reg),
            func: NativeValue::FRef(apply_fn.to_string()),
            args,
            ret_type: NativeType::Ptr,
        });
        insts
    }
    /// Emit instructions for partial application.
    ///
    /// When a closure with arity N is applied to M < N arguments,
    /// create a new closure with arity N-M that captures the original
    /// closure's environment plus the M new arguments.
    pub fn emit_partial_apply(
        &mut self,
        closure_reg: Register,
        arg_regs: &[Register],
    ) -> Vec<NativeInst> {
        let mut insts = Vec::new();
        if arg_regs.is_empty() {
            insts.push(NativeInst::Comment(
                "Partial apply: no args, identity".to_string(),
            ));
            return insts;
        }
        let new_closure_reg = self.fresh_reg();
        insts.push(NativeInst::Comment(format!(
            "Partial apply {} with {} args",
            closure_reg,
            arg_regs.len()
        )));
        let mut args = vec![NativeValue::Reg(closure_reg)];
        for r in arg_regs {
            args.push(NativeValue::Reg(*r));
        }
        insts.push(NativeInst::Call {
            dst: Some(new_closure_reg),
            func: NativeValue::FRef("lean_apply_m".to_string()),
            args,
            ret_type: NativeType::Ptr,
        });
        insts
    }
}
/// Memory layout for a string object.
///
/// OxiLean strings are heap-allocated objects with a reference count header,
/// a length field, and an inline (or pointer-to) UTF-8 byte array.
///
/// ```text
/// +--------+---------+--------+---------+----------+
/// | RC (8) | Tag (1) | Flags  | Len (4) | Data ... |
/// +--------+---------+--------+---------+----------+
/// ```
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StringLayout {
    /// Byte offset of the length field.
    pub len_offset: usize,
    /// Byte offset of the UTF-8 data.
    pub data_offset: usize,
    /// Whether the string is SSO (short-string optimized / inline).
    pub is_sso: bool,
    /// Maximum length for SSO strings.
    pub sso_max_len: usize,
    /// Total allocated size for an SSO string (fixed).
    pub sso_total_size: usize,
}
impl StringLayout {
    /// Standard (non-SSO) string layout.
    #[allow(dead_code)]
    pub fn standard() -> Self {
        StringLayout {
            len_offset: ObjectLayout::HEADER_SIZE,
            data_offset: ObjectLayout::HEADER_SIZE + 8,
            is_sso: false,
            sso_max_len: 0,
            sso_total_size: 0,
        }
    }
    /// Short-string-optimized layout (inline up to `max_len` bytes).
    #[allow(dead_code)]
    pub fn sso(max_len: usize) -> Self {
        let sso_total_size = align_up(
            ObjectLayout::HEADER_SIZE + 4 + max_len,
            ObjectLayout::DEFAULT_ALIGN,
        );
        StringLayout {
            len_offset: ObjectLayout::HEADER_SIZE,
            data_offset: ObjectLayout::HEADER_SIZE + 4,
            is_sso: true,
            sso_max_len: max_len,
            sso_total_size,
        }
    }
    /// Compute the total allocation size for a given string length.
    #[allow(dead_code)]
    pub fn alloc_size(&self, str_len: usize) -> usize {
        if self.is_sso && str_len <= self.sso_max_len {
            self.sso_total_size
        } else {
            align_up(self.data_offset + str_len, ObjectLayout::DEFAULT_ALIGN)
        }
    }
}
/// Builds a complete runtime support module for a given LCNF module.
///
/// Collects all the runtime operations needed (RC, alloc, closures, strings)
/// and emits them as a unit.
#[allow(dead_code)]
pub struct RuntimeModuleBuilder {
    pub(super) config: RuntimeConfig,
    pub(super) rc: RcCodegen,
    pub(super) alloc: AllocatorCodegen,
    pub(super) closure_codegen: ClosureCodegen,
    pub(super) array_codegen: ArrayCodegen,
    pub(super) string_codegen: StringCodegen,
    pub(super) thunk_codegen: ThunkCodegen,
    pub(super) bignat_codegen: BigNatCodegen,
    pub(super) external_codegen: ExternalObjectCodegen,
    pub(super) layout_computer: LayoutComputer,
    /// All emitted instructions, in order.
    pub(super) instructions: Vec<NativeInst>,
}
impl RuntimeModuleBuilder {
    /// Create a new runtime module builder.
    #[allow(dead_code)]
    pub fn new(config: RuntimeConfig) -> Self {
        RuntimeModuleBuilder {
            rc: RcCodegen::new(config.rc_strategy != RcStrategy::None),
            alloc: AllocatorCodegen::new(config.alloc_strategy),
            closure_codegen: ClosureCodegen::new(),
            array_codegen: ArrayCodegen::new(config.alloc_strategy),
            string_codegen: StringCodegen::default(),
            thunk_codegen: ThunkCodegen::new(),
            bignat_codegen: BigNatCodegen::new(),
            external_codegen: ExternalObjectCodegen::new(),
            layout_computer: LayoutComputer::new(),
            instructions: Vec::new(),
            config,
        }
    }
    /// Emit a constructor allocation.
    #[allow(dead_code)]
    pub fn emit_ctor(&mut self, tag: u32, num_objs: usize, scalar_sz: usize) {
        let insts = self.alloc.emit_alloc_ctor(tag, num_objs, scalar_sz);
        self.instructions.extend(insts);
    }
    /// Emit a closure allocation.
    #[allow(dead_code)]
    pub fn emit_closure(&mut self, fn_name: &str, arity: usize, env: &[Register]) {
        let insts = self
            .closure_codegen
            .emit_closure_create(fn_name, arity, env);
        self.instructions.extend(insts);
    }
    /// Emit RC increment for a value.
    #[allow(dead_code)]
    pub fn emit_inc(&mut self, reg: Register) {
        let insts = self.rc.emit_rc_inc(reg);
        self.instructions.extend(insts);
    }
    /// Emit RC decrement for a value.
    #[allow(dead_code)]
    pub fn emit_dec(&mut self, reg: Register) {
        let insts = self.rc.emit_rc_dec(reg);
        self.instructions.extend(insts);
    }
    /// Emit a BigNat addition.
    #[allow(dead_code)]
    pub fn emit_nat_add(&mut self, lhs: Register, rhs: Register) {
        let insts = self.bignat_codegen.emit_add(lhs, rhs);
        self.instructions.extend(insts);
    }
    /// Emit a string append.
    #[allow(dead_code)]
    pub fn emit_str_append(&mut self, lhs: Register, rhs: Register) {
        let insts = self.string_codegen.emit_string_append(lhs, rhs);
        self.instructions.extend(insts);
    }
    /// Get all emitted instructions.
    #[allow(dead_code)]
    pub fn instructions(&self) -> &[NativeInst] {
        &self.instructions
    }
    /// Total number of emitted instructions.
    #[allow(dead_code)]
    pub fn instruction_count(&self) -> usize {
        self.instructions.len()
    }
    /// Count call instructions.
    #[allow(dead_code)]
    pub fn call_count(&self) -> usize {
        self.instructions
            .iter()
            .filter(|i| matches!(i, NativeInst::Call { .. }))
            .count()
    }
    /// Count comment instructions.
    #[allow(dead_code)]
    pub fn comment_count(&self) -> usize {
        self.instructions
            .iter()
            .filter(|i| matches!(i, NativeInst::Comment(_)))
            .count()
    }
}
/// Generates code for thunk (lazy) operations.
#[allow(dead_code)]
pub struct ThunkCodegen {
    pub(super) temp_counter: u32,
}
impl ThunkCodegen {
    /// Create a new thunk codegen.
    #[allow(dead_code)]
    pub fn new() -> Self {
        ThunkCodegen { temp_counter: 0 }
    }
    pub(super) fn fresh_reg(&mut self) -> Register {
        let r = Register::virt(1100 + self.temp_counter);
        self.temp_counter += 1;
        r
    }
    /// Emit a thunk allocation wrapping a function pointer.
    #[allow(dead_code)]
    pub fn emit_alloc_thunk(&mut self, fn_name: &str) -> Vec<NativeInst> {
        let dst = self.fresh_reg();
        vec![
            NativeInst::Comment(format!("Alloc thunk @{}", fn_name)),
            NativeInst::Call {
                dst: Some(dst),
                func: NativeValue::FRef("lean_alloc_thunk".to_string()),
                args: vec![NativeValue::FRef(fn_name.to_string())],
                ret_type: NativeType::Ptr,
            },
        ]
    }
    /// Emit a thunk force (evaluate the lazy value).
    #[allow(dead_code)]
    pub fn emit_force_thunk(&mut self, thunk_reg: Register) -> Vec<NativeInst> {
        let dst = self.fresh_reg();
        vec![
            NativeInst::Comment(format!("Force thunk {}", thunk_reg)),
            NativeInst::Call {
                dst: Some(dst),
                func: NativeValue::FRef("lean_thunk_get".to_string()),
                args: vec![NativeValue::Reg(thunk_reg)],
                ret_type: NativeType::Ptr,
            },
        ]
    }
    /// Emit a check whether a thunk has already been evaluated.
    #[allow(dead_code)]
    pub fn emit_is_evaluated(&mut self, thunk_reg: Register) -> Vec<NativeInst> {
        let dst = self.fresh_reg();
        vec![
            NativeInst::Comment(format!("Thunk is_evaluated {}", thunk_reg)),
            NativeInst::Call {
                dst: Some(dst),
                func: NativeValue::FRef("lean_thunk_is_evaluated".to_string()),
                args: vec![NativeValue::Reg(thunk_reg)],
                ret_type: NativeType::I8,
            },
        ]
    }
}
/// Closure representation strategy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClosureRepr {
    /// Standard closure: function pointer + environment array.
    Standard,
    /// Flat closure: inline captured variables in the closure struct.
    Flat,
    /// Defunctionalized: use tagged unions instead of function pointers.
    Defunctionalized,
}
/// Generates code for array operations: allocation, get, set, resize.
#[allow(dead_code)]
pub struct ArrayCodegen {
    pub(super) temp_counter: u32,
    pub(super) strategy: AllocStrategy,
}
impl ArrayCodegen {
    /// Create a new array codegen.
    #[allow(dead_code)]
    pub fn new(strategy: AllocStrategy) -> Self {
        ArrayCodegen {
            temp_counter: 0,
            strategy,
        }
    }
    pub(super) fn fresh_reg(&mut self) -> Register {
        let r = Register::virt(900 + self.temp_counter);
        self.temp_counter += 1;
        r
    }
    /// Emit instructions to allocate an array with the given initial capacity.
    #[allow(dead_code)]
    pub fn emit_alloc_array(&mut self, capacity: usize) -> Vec<NativeInst> {
        let dst = self.fresh_reg();
        vec![
            NativeInst::Comment(format!("Alloc array capacity={}", capacity)),
            NativeInst::Call {
                dst: Some(dst),
                func: NativeValue::FRef("lean_alloc_array".to_string()),
                args: vec![NativeValue::Imm(0), NativeValue::Imm(capacity as i64)],
                ret_type: NativeType::Ptr,
            },
        ]
    }
    /// Emit an array get instruction.
    #[allow(dead_code)]
    pub fn emit_array_get(&mut self, arr_reg: Register, idx_reg: Register) -> Vec<NativeInst> {
        let dst = self.fresh_reg();
        vec![
            NativeInst::Comment(format!("Array get {} [{}]", arr_reg, idx_reg)),
            NativeInst::Call {
                dst: Some(dst),
                func: NativeValue::FRef("lean_array_get".to_string()),
                args: vec![NativeValue::Reg(arr_reg), NativeValue::Reg(idx_reg)],
                ret_type: NativeType::Ptr,
            },
        ]
    }
    /// Emit an array set instruction (mutating).
    #[allow(dead_code)]
    pub fn emit_array_set(
        &mut self,
        arr_reg: Register,
        idx_reg: Register,
        val_reg: Register,
    ) -> Vec<NativeInst> {
        let dst = self.fresh_reg();
        vec![
            NativeInst::Comment(format!("Array set {} [{}] = {}", arr_reg, idx_reg, val_reg)),
            NativeInst::Call {
                dst: Some(dst),
                func: NativeValue::FRef("lean_array_set".to_string()),
                args: vec![
                    NativeValue::Reg(arr_reg),
                    NativeValue::Reg(idx_reg),
                    NativeValue::Reg(val_reg),
                ],
                ret_type: NativeType::Ptr,
            },
        ]
    }
    /// Emit an array push (append element).
    #[allow(dead_code)]
    pub fn emit_array_push(&mut self, arr_reg: Register, val_reg: Register) -> Vec<NativeInst> {
        let dst = self.fresh_reg();
        vec![
            NativeInst::Comment(format!("Array push {} val={}", arr_reg, val_reg)),
            NativeInst::Call {
                dst: Some(dst),
                func: NativeValue::FRef("lean_array_push".to_string()),
                args: vec![NativeValue::Reg(arr_reg), NativeValue::Reg(val_reg)],
                ret_type: NativeType::Ptr,
            },
        ]
    }
    /// Emit an array size query.
    #[allow(dead_code)]
    pub fn emit_array_size(&mut self, arr_reg: Register) -> Vec<NativeInst> {
        let dst = self.fresh_reg();
        vec![
            NativeInst::Comment(format!("Array size {}", arr_reg)),
            NativeInst::Call {
                dst: Some(dst),
                func: NativeValue::FRef("lean_array_size".to_string()),
                args: vec![NativeValue::Reg(arr_reg)],
                ret_type: NativeType::I64,
            },
        ]
    }
}
/// A persistent layout cache that survives across multiple compilation units.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct LayoutCache {
    pub(super) ctor_layouts: HashMap<String, ObjectLayout>,
    pub(super) closure_layouts: HashMap<(usize, usize), ClosureLayout>,
}
impl LayoutCache {
    /// Create a new empty layout cache.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    /// Get or compute a constructor layout.
    #[allow(dead_code)]
    pub fn get_ctor(
        &mut self,
        ctor_name: &str,
        ctor_tag: u32,
        num_obj: usize,
        scalar_sz: usize,
    ) -> &ObjectLayout {
        let key = format!("{}#{}#{}#{}", ctor_name, ctor_tag, num_obj, scalar_sz);
        self.ctor_layouts
            .entry(key)
            .or_insert_with(|| ObjectLayout::for_ctor(ctor_tag, num_obj, scalar_sz))
    }
    /// Get or compute a closure layout.
    #[allow(dead_code)]
    pub fn get_closure(&mut self, arity: usize, num_captured: usize) -> &ClosureLayout {
        self.closure_layouts
            .entry((arity, num_captured))
            .or_insert_with(|| ClosureLayout::new(arity, num_captured))
    }
    /// Clear all cached layouts.
    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.ctor_layouts.clear();
        self.closure_layouts.clear();
    }
    /// Number of cached constructor layouts.
    #[allow(dead_code)]
    pub fn ctor_count(&self) -> usize {
        self.ctor_layouts.len()
    }
    /// Number of cached closure layouts.
    #[allow(dead_code)]
    pub fn closure_count(&self) -> usize {
        self.closure_layouts.len()
    }
}
/// Configuration for runtime code generation.
#[derive(Debug, Clone)]
pub struct RuntimeConfig {
    /// Reference counting strategy.
    pub rc_strategy: RcStrategy,
    /// Allocation strategy.
    pub alloc_strategy: AllocStrategy,
    /// Closure representation strategy.
    pub closure_repr: ClosureRepr,
    /// Whether to enable debug checks (null pointer, bounds, etc.).
    pub debug_checks: bool,
    /// Whether to use thread-safe atomic RC.
    pub thread_safe: bool,
}
/// Reference counting strategy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RcStrategy {
    /// Standard reference counting with inc/dec.
    Standard,
    /// Deferred reference counting (batch dec at safe points).
    Deferred,
    /// No reference counting (leak everything; useful for benchmarks).
    None,
}
/// Detailed layout information for a closure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClosureLayout {
    /// Byte offset of the function pointer from closure start.
    pub fn_ptr_offset: usize,
    /// Byte offset of the captured environment from closure start.
    pub env_offset: usize,
    /// Total size of the captured environment in bytes.
    pub env_size: usize,
    /// Number of parameters the function takes (total arity).
    pub arity: usize,
    /// Number of captured variables.
    pub num_captured: usize,
    /// The overall object layout.
    pub object_layout: ObjectLayout,
}
impl ClosureLayout {
    /// Create a closure layout for a function with the given arity
    /// and captured variable count.
    pub fn new(arity: usize, num_captured: usize) -> Self {
        let fn_ptr_offset = ObjectLayout::HEADER_SIZE;
        let env_offset = fn_ptr_offset + 16;
        let env_size = num_captured * 8;
        ClosureLayout {
            fn_ptr_offset,
            env_offset,
            env_size,
            arity,
            num_captured,
            object_layout: ObjectLayout::for_closure(arity, num_captured),
        }
    }
    /// Byte offset of the i-th captured variable.
    pub fn captured_var_offset(&self, idx: usize) -> usize {
        assert!(idx < self.num_captured);
        self.env_offset + idx * 8
    }
}
/// Tag identifying the kind of runtime object.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectTag {
    /// A scalar value (unboxed integer, etc.).
    Scalar,
    /// A closure object (function pointer + environment).
    Closure,
    /// An array object.
    Array,
    /// A constructor/struct object (inductive type).
    Struct,
    /// An external (opaque) object managed by foreign code.
    External,
    /// A string object.
    String,
    /// A big natural number (multi-precision).
    BigNat,
    /// A thunk (lazy value).
    Thunk,
}
impl ObjectTag {
    /// Convert to a numeric tag value.
    pub fn to_u8(self) -> u8 {
        match self {
            ObjectTag::Scalar => 0,
            ObjectTag::Closure => 1,
            ObjectTag::Array => 2,
            ObjectTag::Struct => 3,
            ObjectTag::External => 4,
            ObjectTag::String => 5,
            ObjectTag::BigNat => 6,
            ObjectTag::Thunk => 7,
        }
    }
    /// Parse from a numeric tag value.
    pub fn from_u8(tag: u8) -> Option<Self> {
        match tag {
            0 => Some(ObjectTag::Scalar),
            1 => Some(ObjectTag::Closure),
            2 => Some(ObjectTag::Array),
            3 => Some(ObjectTag::Struct),
            4 => Some(ObjectTag::External),
            5 => Some(ObjectTag::String),
            6 => Some(ObjectTag::BigNat),
            7 => Some(ObjectTag::Thunk),
            _ => None,
        }
    }
}
/// Allocation strategy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AllocStrategy {
    /// Use the system allocator (malloc/free).
    System,
    /// Use a bump allocator (arena-style, no individual free).
    Bump,
    /// Use a pool allocator (fixed-size blocks).
    Pool,
    /// Use the Lean runtime allocator.
    LeanRuntime,
}
/// Computes the number of uses (increments needed) for each variable in
/// an LCNF expression, to drive multi-increment codegen.
#[allow(dead_code)]
pub struct RcUseAnalysis {
    pub(super) use_counts: HashMap<LcnfVarId, usize>,
}
impl RcUseAnalysis {
    /// Create a new RC use analysis.
    #[allow(dead_code)]
    pub fn new() -> Self {
        RcUseAnalysis {
            use_counts: HashMap::new(),
        }
    }
    /// Analyze an LCNF module.
    #[allow(dead_code)]
    pub fn analyze_module(&mut self, module: &LcnfModule) {
        for decl in &module.fun_decls {
            self.analyze_expr(&decl.body);
        }
    }
    /// Analyze an expression.
    #[allow(dead_code)]
    pub fn analyze_expr(&mut self, expr: &LcnfExpr) {
        match expr {
            LcnfExpr::Let { value, body, .. } => {
                self.analyze_let_value(value);
                self.analyze_expr(body);
            }
            LcnfExpr::Case {
                scrutinee,
                alts,
                default,
                ..
            } => {
                self.inc_use(*scrutinee);
                for alt in alts {
                    self.analyze_expr(&alt.body);
                }
                if let Some(def) = default {
                    self.analyze_expr(def);
                }
            }
            LcnfExpr::Return(arg) => self.analyze_arg(arg),
            LcnfExpr::TailCall(func, args) => {
                self.analyze_arg(func);
                for a in args {
                    self.analyze_arg(a);
                }
            }
            LcnfExpr::Unreachable => {}
        }
    }
    pub(super) fn analyze_let_value(&mut self, value: &LcnfLetValue) {
        match value {
            LcnfLetValue::App(func, args) => {
                self.analyze_arg(func);
                for a in args {
                    self.analyze_arg(a);
                }
            }
            LcnfLetValue::Ctor(_, _, args) => {
                for a in args {
                    self.analyze_arg(a);
                }
            }
            LcnfLetValue::Proj(_, _, v) | LcnfLetValue::Reset(v) => {
                self.inc_use(*v);
            }
            LcnfLetValue::FVar(v) => self.inc_use(*v),
            LcnfLetValue::Reuse(slot, _, _, args) => {
                self.inc_use(*slot);
                for a in args {
                    self.analyze_arg(a);
                }
            }
            LcnfLetValue::Lit(_) | LcnfLetValue::Erased => {}
        }
    }
    pub(super) fn analyze_arg(&mut self, arg: &LcnfArg) {
        if let LcnfArg::Var(v) = arg {
            self.inc_use(*v);
        }
    }
    pub(super) fn inc_use(&mut self, var: LcnfVarId) {
        *self.use_counts.entry(var).or_insert(0) += 1;
    }
    /// Get the use count for a variable.
    #[allow(dead_code)]
    pub fn use_count(&self, var: LcnfVarId) -> usize {
        self.use_counts.get(&var).copied().unwrap_or(0)
    }
    /// Get all variables with more than one use.
    #[allow(dead_code)]
    pub fn multi_use_vars(&self) -> Vec<(LcnfVarId, usize)> {
        self.use_counts
            .iter()
            .filter(|(_, &c)| c > 1)
            .map(|(&v, &c)| (v, c))
            .collect()
    }
}
/// Generates memory allocation and deallocation operations.
pub struct AllocatorCodegen {
    /// Counter for generated temporaries.
    pub(super) temp_counter: u32,
    /// Allocation strategy.
    pub(super) strategy: AllocStrategy,
}
impl AllocatorCodegen {
    /// Create a new allocator codegen.
    pub fn new(strategy: AllocStrategy) -> Self {
        AllocatorCodegen {
            temp_counter: 0,
            strategy,
        }
    }
    /// Allocate a fresh temporary register.
    pub(super) fn fresh_reg(&mut self) -> Register {
        let r = Register::virt(700 + self.temp_counter);
        self.temp_counter += 1;
        r
    }
    /// Emit instructions to allocate `size` bytes with the given alignment.
    pub fn emit_alloc(&mut self, size: usize, align: usize) -> Vec<NativeInst> {
        let dst = self.fresh_reg();
        let alloc_fn = match self.strategy {
            AllocStrategy::System => "malloc",
            AllocStrategy::Bump => "bump_alloc",
            AllocStrategy::Pool => "pool_alloc",
            AllocStrategy::LeanRuntime => "lean_alloc_small",
        };
        vec![
            NativeInst::Comment(format!(
                "Alloc {} bytes, align {} ({})",
                size, align, alloc_fn
            )),
            NativeInst::Call {
                dst: Some(dst),
                func: NativeValue::FRef(alloc_fn.to_string()),
                args: vec![NativeValue::Imm(size as i64)],
                ret_type: NativeType::Ptr,
            },
        ]
    }
    /// Emit instructions to free memory at the given pointer.
    pub fn emit_free(&mut self, ptr_reg: Register) -> Vec<NativeInst> {
        let free_fn = match self.strategy {
            AllocStrategy::System => "free",
            AllocStrategy::Bump => {
                return vec![NativeInst::Comment("Bump: no free".to_string())];
            }
            AllocStrategy::Pool => "pool_free",
            AllocStrategy::LeanRuntime => "lean_free_small",
        };
        vec![
            NativeInst::Comment(format!("Free {}", ptr_reg)),
            NativeInst::Call {
                dst: None,
                func: NativeValue::FRef(free_fn.to_string()),
                args: vec![NativeValue::Reg(ptr_reg)],
                ret_type: NativeType::Void,
            },
        ]
    }
    /// Emit allocation for a constructor object.
    pub fn emit_alloc_ctor(
        &mut self,
        tag: u32,
        num_objs: usize,
        scalar_sz: usize,
    ) -> Vec<NativeInst> {
        let dst = self.fresh_reg();
        vec![
            NativeInst::Comment(format!(
                "Alloc ctor tag={}, objs={}, scalar={}",
                tag, num_objs, scalar_sz
            )),
            NativeInst::Call {
                dst: Some(dst),
                func: NativeValue::FRef("lean_alloc_ctor".to_string()),
                args: vec![
                    NativeValue::Imm(tag as i64),
                    NativeValue::Imm(num_objs as i64),
                    NativeValue::Imm(scalar_sz as i64),
                ],
                ret_type: NativeType::Ptr,
            },
        ]
    }
    /// Emit allocation for a closure.
    pub fn emit_alloc_closure(
        &mut self,
        fn_name: &str,
        arity: usize,
        num_captured: usize,
    ) -> Vec<NativeInst> {
        let dst = self.fresh_reg();
        vec![
            NativeInst::Comment(format!(
                "Alloc closure @{} arity={} captured={}",
                fn_name, arity, num_captured
            )),
            NativeInst::Call {
                dst: Some(dst),
                func: NativeValue::FRef("lean_alloc_closure".to_string()),
                args: vec![
                    NativeValue::FRef(fn_name.to_string()),
                    NativeValue::Imm(arity as i64),
                    NativeValue::Imm(num_captured as i64),
                ],
                ret_type: NativeType::Ptr,
            },
        ]
    }
}
/// Type information for layout computation.
#[derive(Debug, Clone)]
pub struct TypeInfo {
    /// Name of the type.
    pub name: String,
    /// Constructors (name, tag, field types).
    pub constructors: Vec<(String, u32, Vec<LcnfType>)>,
    /// Whether this type is recursive.
    pub is_recursive: bool,
}
/// Memory layout of a runtime object.
///
/// All OxiLean heap objects share a common header layout:
///
/// ```text
/// +--------+---------+---------+----------+
/// | RC (8) | Tag (1) | Other   | Padding  |
/// +--------+---------+---------+----------+
/// | Payload ...                            |
/// +----------------------------------------+
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectLayout {
    /// Number of bits used for the object tag in the header.
    pub tag_bits: u8,
    /// Byte offset of the reference count field from object start.
    pub rc_offset: usize,
    /// Byte offset of the payload from object start.
    pub payload_offset: usize,
    /// Total size of the object in bytes (header + payload).
    pub total_size: usize,
    /// Required alignment in bytes.
    pub alignment: usize,
    /// Number of object (pointer) fields in the payload.
    pub num_obj_fields: usize,
    /// Size of scalar (non-pointer) data in bytes.
    pub scalar_size: usize,
    /// The object tag.
    pub tag: ObjectTag,
}
impl ObjectLayout {
    /// Size of the standard object header.
    pub const HEADER_SIZE: usize = 16;
    /// Offset of the reference count within the header.
    pub const RC_OFFSET: usize = 0;
    /// Offset of the tag within the header.
    pub const TAG_OFFSET: usize = 8;
    /// Default alignment.
    pub const DEFAULT_ALIGN: usize = 8;
    /// Create a layout for a constructor object with the given fields.
    pub fn for_ctor(_ctor_tag: u32, num_obj_fields: usize, scalar_size: usize) -> Self {
        let payload_size = num_obj_fields * 8 + scalar_size;
        let total_size = Self::HEADER_SIZE + payload_size;
        let aligned_size = align_up(total_size, Self::DEFAULT_ALIGN);
        ObjectLayout {
            tag_bits: 8,
            rc_offset: Self::RC_OFFSET,
            payload_offset: Self::HEADER_SIZE,
            total_size: aligned_size,
            alignment: Self::DEFAULT_ALIGN,
            num_obj_fields,
            scalar_size,
            tag: ObjectTag::Struct,
        }
    }
    /// Create a layout for a closure object.
    pub fn for_closure(_arity: usize, num_captured: usize) -> Self {
        let closure_header = 16;
        let payload_size = closure_header + num_captured * 8;
        let total_size = Self::HEADER_SIZE + payload_size;
        let aligned_size = align_up(total_size, Self::DEFAULT_ALIGN);
        ObjectLayout {
            tag_bits: 8,
            rc_offset: Self::RC_OFFSET,
            payload_offset: Self::HEADER_SIZE,
            total_size: aligned_size,
            alignment: Self::DEFAULT_ALIGN,
            num_obj_fields: num_captured,
            scalar_size: closure_header,
            tag: ObjectTag::Closure,
        }
    }
    /// Create a layout for an array object.
    pub fn for_array(capacity: usize) -> Self {
        let array_header = 16;
        let payload_size = array_header + capacity * 8;
        let total_size = Self::HEADER_SIZE + payload_size;
        let aligned_size = align_up(total_size, Self::DEFAULT_ALIGN);
        ObjectLayout {
            tag_bits: 8,
            rc_offset: Self::RC_OFFSET,
            payload_offset: Self::HEADER_SIZE,
            total_size: aligned_size,
            alignment: Self::DEFAULT_ALIGN,
            num_obj_fields: capacity,
            scalar_size: array_header,
            tag: ObjectTag::Array,
        }
    }
    /// Create a layout for an external object.
    pub fn for_external() -> Self {
        let payload_size = 24;
        let total_size = Self::HEADER_SIZE + payload_size;
        ObjectLayout {
            tag_bits: 8,
            rc_offset: Self::RC_OFFSET,
            payload_offset: Self::HEADER_SIZE,
            total_size,
            alignment: Self::DEFAULT_ALIGN,
            num_obj_fields: 0,
            scalar_size: payload_size,
            tag: ObjectTag::External,
        }
    }
    /// Byte offset of the i-th object field.
    pub fn obj_field_offset(&self, idx: usize) -> usize {
        assert!(idx < self.num_obj_fields);
        self.payload_offset + idx * 8
    }
    /// Byte offset of the scalar region.
    pub fn scalar_offset(&self) -> usize {
        self.payload_offset + self.num_obj_fields * 8
    }
}
/// Generates reference counting operations as native IR instructions.
pub struct RcCodegen {
    /// Whether to emit RC operations (can be disabled for debugging).
    pub(super) enabled: bool,
    /// Counter for generated temporaries.
    pub(super) temp_counter: u32,
}
impl RcCodegen {
    /// Create a new RC codegen.
    pub fn new(enabled: bool) -> Self {
        RcCodegen {
            enabled,
            temp_counter: 0,
        }
    }
    /// Allocate a fresh temporary register.
    pub(super) fn fresh_reg(&mut self) -> Register {
        let r = Register::virt(500 + self.temp_counter);
        self.temp_counter += 1;
        r
    }
    /// Emit instructions to increment the reference count of an object.
    ///
    /// ```text
    /// if (obj != NULL && !lean_is_scalar(obj)) {
    ///     lean_inc_ref(obj);
    /// }
    /// ```
    pub fn emit_rc_inc(&mut self, obj_reg: Register) -> Vec<NativeInst> {
        if !self.enabled {
            return vec![NativeInst::Comment("RC inc (disabled)".to_string())];
        }
        vec![
            NativeInst::Comment(format!("RC inc {}", obj_reg)),
            NativeInst::Call {
                dst: None,
                func: NativeValue::FRef("lean_inc_ref".to_string()),
                args: vec![NativeValue::Reg(obj_reg)],
                ret_type: NativeType::Void,
            },
        ]
    }
    /// Emit instructions to decrement the reference count of an object.
    /// If the count reaches zero, the object is freed.
    ///
    /// ```text
    /// if (obj != NULL && !lean_is_scalar(obj)) {
    ///     lean_dec_ref(obj);
    /// }
    /// ```
    pub fn emit_rc_dec(&mut self, obj_reg: Register) -> Vec<NativeInst> {
        if !self.enabled {
            return vec![NativeInst::Comment("RC dec (disabled)".to_string())];
        }
        vec![
            NativeInst::Comment(format!("RC dec {}", obj_reg)),
            NativeInst::Call {
                dst: None,
                func: NativeValue::FRef("lean_dec_ref".to_string()),
                args: vec![NativeValue::Reg(obj_reg)],
                ret_type: NativeType::Void,
            },
        ]
    }
    /// Emit an instruction to check if an object has a unique reference
    /// (refcount == 1), enabling in-place mutation.
    pub fn emit_rc_is_unique(&mut self, obj_reg: Register) -> NativeInst {
        let dst = self.fresh_reg();
        NativeInst::Call {
            dst: Some(dst),
            func: NativeValue::FRef("lean_is_exclusive".to_string()),
            args: vec![NativeValue::Reg(obj_reg)],
            ret_type: NativeType::I8,
        }
    }
    /// Emit a conditional reset: if the object is unique, reset it for reuse;
    /// otherwise, decrement and allocate fresh.
    pub fn emit_conditional_reset(
        &mut self,
        obj_reg: Register,
        _num_obj_fields: usize,
        _scalar_size: usize,
    ) -> Vec<NativeInst> {
        if !self.enabled {
            return vec![NativeInst::Comment(
                "Conditional reset (disabled)".to_string(),
            )];
        }
        let is_unique = self.fresh_reg();
        let result_reg = self.fresh_reg();
        vec![
            NativeInst::Comment(format!("Conditional reset {}", obj_reg)),
            NativeInst::Call {
                dst: Some(is_unique),
                func: NativeValue::FRef("lean_is_exclusive".to_string()),
                args: vec![NativeValue::Reg(obj_reg)],
                ret_type: NativeType::I8,
            },
            NativeInst::Select {
                dst: result_reg,
                ty: NativeType::Ptr,
                cond: NativeValue::Reg(is_unique),
                true_val: NativeValue::Reg(obj_reg),
                false_val: NativeValue::Imm(0),
            },
        ]
    }
    /// Emit multi-reference increment (for when a variable is used N times).
    pub fn emit_rc_inc_n(&mut self, obj_reg: Register, count: usize) -> Vec<NativeInst> {
        if !self.enabled || count == 0 {
            return Vec::new();
        }
        let mut insts = Vec::new();
        if count == 1 {
            insts.extend(self.emit_rc_inc(obj_reg));
        } else {
            insts.push(NativeInst::Comment(format!(
                "RC inc {} (x{})",
                obj_reg, count
            )));
            insts.push(NativeInst::Call {
                dst: None,
                func: NativeValue::FRef("lean_inc_ref_n".to_string()),
                args: vec![NativeValue::Reg(obj_reg), NativeValue::Imm(count as i64)],
                ret_type: NativeType::Void,
            });
        }
        insts
    }
}
/// Generates code for string operations.
#[allow(dead_code)]
pub struct StringCodegen {
    pub(super) temp_counter: u32,
    pub(super) layout: StringLayout,
}
impl StringCodegen {
    /// Create a new string codegen.
    #[allow(dead_code)]
    pub fn new(layout: StringLayout) -> Self {
        StringCodegen {
            temp_counter: 0,
            layout,
        }
    }
    pub(super) fn fresh_reg(&mut self) -> Register {
        let r = Register::virt(1000 + self.temp_counter);
        self.temp_counter += 1;
        r
    }
    /// Emit a string literal allocation.
    #[allow(dead_code)]
    pub fn emit_string_lit(&mut self, value: &str) -> Vec<NativeInst> {
        let dst = self.fresh_reg();
        vec![
            NativeInst::Comment(format!("String literal {:?}", value)),
            NativeInst::Call {
                dst: Some(dst),
                func: NativeValue::FRef("lean_mk_string".to_string()),
                args: vec![NativeValue::Imm(value.len() as i64)],
                ret_type: NativeType::Ptr,
            },
        ]
    }
    /// Emit a string append operation.
    #[allow(dead_code)]
    pub fn emit_string_append(&mut self, lhs: Register, rhs: Register) -> Vec<NativeInst> {
        let dst = self.fresh_reg();
        vec![
            NativeInst::Comment(format!("String append {} ++ {}", lhs, rhs)),
            NativeInst::Call {
                dst: Some(dst),
                func: NativeValue::FRef("lean_string_append".to_string()),
                args: vec![NativeValue::Reg(lhs), NativeValue::Reg(rhs)],
                ret_type: NativeType::Ptr,
            },
        ]
    }
    /// Emit a string length query.
    #[allow(dead_code)]
    pub fn emit_string_length(&mut self, str_reg: Register) -> Vec<NativeInst> {
        let dst = self.fresh_reg();
        vec![
            NativeInst::Comment(format!("String length {}", str_reg)),
            NativeInst::Load {
                dst,
                ty: NativeType::I64,
                addr: NativeValue::Reg(str_reg),
            },
        ]
    }
    /// Emit a string equality check.
    #[allow(dead_code)]
    pub fn emit_string_eq(&mut self, lhs: Register, rhs: Register) -> Vec<NativeInst> {
        let dst = self.fresh_reg();
        vec![
            NativeInst::Comment(format!("String eq {} == {}", lhs, rhs)),
            NativeInst::Call {
                dst: Some(dst),
                func: NativeValue::FRef("lean_string_eq".to_string()),
                args: vec![NativeValue::Reg(lhs), NativeValue::Reg(rhs)],
                ret_type: NativeType::I8,
            },
        ]
    }
}
/// Generates code for arbitrary-precision natural number (BigNat) operations.
#[allow(dead_code)]
pub struct BigNatCodegen {
    pub(super) temp_counter: u32,
}
impl BigNatCodegen {
    /// Create a new BigNat codegen.
    #[allow(dead_code)]
    pub fn new() -> Self {
        BigNatCodegen { temp_counter: 0 }
    }
    pub(super) fn fresh_reg(&mut self) -> Register {
        let r = Register::virt(1200 + self.temp_counter);
        self.temp_counter += 1;
        r
    }
    /// Emit BigNat addition.
    #[allow(dead_code)]
    pub fn emit_add(&mut self, lhs: Register, rhs: Register) -> Vec<NativeInst> {
        let dst = self.fresh_reg();
        vec![
            NativeInst::Comment(format!("BigNat add {} + {}", lhs, rhs)),
            NativeInst::Call {
                dst: Some(dst),
                func: NativeValue::FRef("lean_nat_add".to_string()),
                args: vec![NativeValue::Reg(lhs), NativeValue::Reg(rhs)],
                ret_type: NativeType::Ptr,
            },
        ]
    }
    /// Emit BigNat multiplication.
    #[allow(dead_code)]
    pub fn emit_mul(&mut self, lhs: Register, rhs: Register) -> Vec<NativeInst> {
        let dst = self.fresh_reg();
        vec![
            NativeInst::Comment(format!("BigNat mul {} * {}", lhs, rhs)),
            NativeInst::Call {
                dst: Some(dst),
                func: NativeValue::FRef("lean_nat_mul".to_string()),
                args: vec![NativeValue::Reg(lhs), NativeValue::Reg(rhs)],
                ret_type: NativeType::Ptr,
            },
        ]
    }
    /// Emit BigNat subtraction (saturating at zero).
    #[allow(dead_code)]
    pub fn emit_sub(&mut self, lhs: Register, rhs: Register) -> Vec<NativeInst> {
        let dst = self.fresh_reg();
        vec![
            NativeInst::Comment(format!("BigNat sub {} - {}", lhs, rhs)),
            NativeInst::Call {
                dst: Some(dst),
                func: NativeValue::FRef("lean_nat_sub".to_string()),
                args: vec![NativeValue::Reg(lhs), NativeValue::Reg(rhs)],
                ret_type: NativeType::Ptr,
            },
        ]
    }
    /// Emit BigNat division.
    #[allow(dead_code)]
    pub fn emit_div(&mut self, lhs: Register, rhs: Register) -> Vec<NativeInst> {
        let dst = self.fresh_reg();
        vec![
            NativeInst::Comment(format!("BigNat div {} / {}", lhs, rhs)),
            NativeInst::Call {
                dst: Some(dst),
                func: NativeValue::FRef("lean_nat_div".to_string()),
                args: vec![NativeValue::Reg(lhs), NativeValue::Reg(rhs)],
                ret_type: NativeType::Ptr,
            },
        ]
    }
    /// Emit BigNat comparison (returns -1, 0, or 1).
    #[allow(dead_code)]
    pub fn emit_cmp(&mut self, lhs: Register, rhs: Register) -> Vec<NativeInst> {
        let dst = self.fresh_reg();
        vec![
            NativeInst::Comment(format!("BigNat cmp {} vs {}", lhs, rhs)),
            NativeInst::Call {
                dst: Some(dst),
                func: NativeValue::FRef("lean_nat_cmp".to_string()),
                args: vec![NativeValue::Reg(lhs), NativeValue::Reg(rhs)],
                ret_type: NativeType::I8,
            },
        ]
    }
    /// Emit conversion from a small native integer to BigNat.
    #[allow(dead_code)]
    pub fn emit_of_u64(&mut self, val: i64) -> Vec<NativeInst> {
        let dst = self.fresh_reg();
        vec![
            NativeInst::Comment(format!("BigNat from u64 {}", val)),
            NativeInst::Call {
                dst: Some(dst),
                func: NativeValue::FRef("lean_nat_mk_small".to_string()),
                args: vec![NativeValue::Imm(val)],
                ret_type: NativeType::Ptr,
            },
        ]
    }
}
/// Computes memory layouts for types.
pub struct LayoutComputer {
    /// Cached layouts.
    pub(super) cache: HashMap<String, ObjectLayout>,
    /// Type information registry.
    pub(super) type_info: HashMap<String, TypeInfo>,
}
impl LayoutComputer {
    /// Create a new layout computer.
    pub fn new() -> Self {
        LayoutComputer {
            cache: HashMap::new(),
            type_info: HashMap::new(),
        }
    }
    /// Register type information.
    pub fn register_type(&mut self, info: TypeInfo) {
        self.type_info.insert(info.name.clone(), info);
    }
    /// Compute the layout for a constructor application.
    pub fn compute_ctor_layout(
        &mut self,
        ctor_name: &str,
        ctor_tag: u32,
        field_types: &[LcnfType],
    ) -> ObjectLayout {
        let cache_key = format!("{}#{}", ctor_name, ctor_tag);
        if let Some(layout) = self.cache.get(&cache_key) {
            return layout.clone();
        }
        let mut num_obj_fields = 0usize;
        let mut scalar_size = 0usize;
        for ty in field_types {
            if is_boxed_type(ty) {
                num_obj_fields += 1;
            } else {
                scalar_size += scalar_type_size(ty);
            }
        }
        let layout = ObjectLayout::for_ctor(ctor_tag, num_obj_fields, scalar_size);
        self.cache.insert(cache_key, layout.clone());
        layout
    }
    /// Compute the layout for a type given its info.
    pub fn compute_layout(&mut self, type_name: &str) -> Vec<(String, ObjectLayout)> {
        if let Some(info) = self.type_info.get(type_name).cloned() {
            let mut layouts = Vec::new();
            for (ctor_name, ctor_tag, field_types) in &info.constructors {
                let layout = self.compute_ctor_layout(ctor_name, *ctor_tag, field_types);
                layouts.push((ctor_name.clone(), layout));
            }
            layouts
        } else {
            Vec::new()
        }
    }
    /// Get a cached layout.
    pub fn get_cached(&self, key: &str) -> Option<&ObjectLayout> {
        self.cache.get(key)
    }
}
