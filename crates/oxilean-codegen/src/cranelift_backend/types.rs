//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use std::collections::HashMap;

/// An instruction together with its optional result SSA value.
#[derive(Debug, Clone, PartialEq)]
pub struct CraneliftInstResult {
    /// The result SSA value produced by this instruction (None for void/side-effect-only)
    pub result: Option<CraneliftValue>,
    /// The instruction
    pub instr: CraneliftInstr,
}
impl CraneliftInstResult {
    /// Create an instruction with a result.
    pub fn with_result(result: CraneliftValue, instr: CraneliftInstr) -> Self {
        CraneliftInstResult {
            result: Some(result),
            instr,
        }
    }
    /// Create an instruction with no result.
    pub fn no_result(instr: CraneliftInstr) -> Self {
        CraneliftInstResult {
            result: None,
            instr,
        }
    }
    /// Emit this instruction as a textual IR string.
    pub fn emit(&self) -> String {
        let instr_str = emit_instr(&self.instr);
        if let Some(ref v) = self.result {
            format!("{} = {}", v, instr_str)
        } else {
            instr_str
        }
    }
}
/// Function signature.
#[derive(Debug, Clone, PartialEq)]
pub struct Signature {
    /// Calling convention
    pub call_conv: CallConv,
    /// Parameter types
    pub params: Vec<CraneliftType>,
    /// Return types (Cranelift supports multiple returns)
    pub returns: Vec<CraneliftType>,
}
impl Signature {
    /// Create a new signature.
    pub fn new(
        call_conv: CallConv,
        params: Vec<CraneliftType>,
        returns: Vec<CraneliftType>,
    ) -> Self {
        Signature {
            call_conv,
            params,
            returns,
        }
    }
    /// Create a simple C-like signature.
    pub fn c_like(params: Vec<CraneliftType>, returns: Vec<CraneliftType>) -> Self {
        Signature::new(CallConv::SystemV, params, returns)
    }
}
/// Helpers for generating type conversion instructions.
#[allow(dead_code)]
pub struct CraneliftTypeCoerce;
impl CraneliftTypeCoerce {
    /// Generate the instruction to narrow `val` from `src_ty` to `dst_ty` (truncation).
    #[allow(dead_code)]
    pub fn narrow(
        src_ty: &CraneliftType,
        dst_ty: &CraneliftType,
        val: CraneliftValue,
    ) -> Option<CraneliftInstr> {
        match (src_ty, dst_ty) {
            (CraneliftType::I64, CraneliftType::I32)
            | (CraneliftType::I64, CraneliftType::I16)
            | (CraneliftType::I64, CraneliftType::I8)
            | (CraneliftType::I32, CraneliftType::I16)
            | (CraneliftType::I32, CraneliftType::I8)
            | (CraneliftType::I16, CraneliftType::I8) => {
                Some(CraneliftInstr::Ireduce(dst_ty.clone(), val))
            }
            (CraneliftType::F64, CraneliftType::F32) => {
                Some(CraneliftInstr::Fdemote(CraneliftType::F32, val))
            }
            _ => None,
        }
    }
    /// Generate the instruction to widen `val` from `src_ty` to `dst_ty` (extension).
    #[allow(dead_code)]
    pub fn widen_signed(
        _src_ty: &CraneliftType,
        dst_ty: &CraneliftType,
        val: CraneliftValue,
    ) -> Option<CraneliftInstr> {
        match dst_ty {
            CraneliftType::I16 | CraneliftType::I32 | CraneliftType::I64 | CraneliftType::I128 => {
                Some(CraneliftInstr::Sextend(dst_ty.clone(), val))
            }
            CraneliftType::F64 => Some(CraneliftInstr::Fpromote(CraneliftType::F64, val)),
            _ => None,
        }
    }
    /// Generate the instruction to zero-extend `val` to `dst_ty`.
    #[allow(dead_code)]
    pub fn widen_unsigned(
        _src_ty: &CraneliftType,
        dst_ty: &CraneliftType,
        val: CraneliftValue,
    ) -> Option<CraneliftInstr> {
        match dst_ty {
            CraneliftType::I16 | CraneliftType::I32 | CraneliftType::I64 | CraneliftType::I128 => {
                Some(CraneliftInstr::Uextend(dst_ty.clone(), val))
            }
            _ => None,
        }
    }
}
/// How a global value is defined.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum CraneliftGlobalValueDef {
    /// A symbol external to the module.
    Symbol { name: String, colocated: bool },
    /// The value of another global value plus an offset.
    IAddImm { base: u32, offset: i64 },
    /// Loaded from memory (for GOT entries, etc.).
    Load {
        base: u32,
        offset: i32,
        global_type: CraneliftType,
        readonly: bool,
    },
}
/// Fluent builder for constructing CraneliftModule objects.
#[derive(Debug)]
#[allow(dead_code)]
pub struct CraneliftModuleBuilder {
    pub(super) module: CraneliftModule,
}
impl CraneliftModuleBuilder {
    /// Start building a new module.
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>) -> Self {
        CraneliftModuleBuilder {
            module: CraneliftModule::new(name),
        }
    }
    /// Set the target triple.
    #[allow(dead_code)]
    pub fn target(mut self, triple: impl Into<String>) -> Self {
        self.module.target = triple.into();
        self
    }
    /// Declare an external function.
    #[allow(dead_code)]
    pub fn extern_func(mut self, name: impl Into<String>, sig: Signature) -> Self {
        self.module.add_func_decl(name, sig);
        self
    }
    /// Add a data object.
    #[allow(dead_code)]
    pub fn data(mut self, obj: CraneliftDataObject) -> Self {
        self.module.add_data_object(obj);
        self
    }
    /// Add a function definition.
    #[allow(dead_code)]
    pub fn func(mut self, f: CraneliftFunction) -> Self {
        self.module.add_function(f);
        self
    }
    /// Consume and return the completed module.
    #[allow(dead_code)]
    pub fn build(self) -> CraneliftModule {
        self.module
    }
}
/// Fluent builder for constructing CraneliftFunction objects.
#[derive(Debug)]
#[allow(dead_code)]
pub struct CraneliftFunctionBuilder {
    pub(super) func: CraneliftFunction,
    pub(super) current_block: Option<u32>,
}
impl CraneliftFunctionBuilder {
    /// Create a new builder for the named function with given signature.
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>, sig: Signature) -> Self {
        let func = CraneliftFunction::new(name, sig);
        CraneliftFunctionBuilder {
            func,
            current_block: None,
        }
    }
    /// Create an entry block and set it as current.
    #[allow(dead_code)]
    pub fn create_entry_block(mut self) -> Self {
        let b = self.func.new_block();
        self.current_block = Some(b);
        self
    }
    /// Create a new block and set it as current.
    #[allow(dead_code)]
    pub fn create_block(mut self) -> (Self, u32) {
        let b = self.func.new_block();
        self.current_block = Some(b);
        (self, b)
    }
    /// Switch to an existing block.
    #[allow(dead_code)]
    pub fn switch_to_block(mut self, block: u32) -> Self {
        self.current_block = Some(block);
        self
    }
    /// Emit a value-producing instruction in the current block.
    #[allow(dead_code)]
    pub fn ins_result(
        mut self,
        ty: CraneliftType,
        instr: CraneliftInstr,
    ) -> (Self, CraneliftValue) {
        let val = self.func.fresh_value(ty);
        if let Some(b_idx) = self.current_block {
            if let Some(b) = self.func.block_mut(b_idx) {
                b.push_with_result(val.clone(), instr);
            }
        }
        (self, val)
    }
    /// Emit a void (side-effecting) instruction in the current block.
    #[allow(dead_code)]
    pub fn ins_void(mut self, instr: CraneliftInstr) -> Self {
        if let Some(b_idx) = self.current_block {
            if let Some(b) = self.func.block_mut(b_idx) {
                b.push_void(instr);
            }
        }
        self
    }
    /// Add a block parameter to the current block.
    #[allow(dead_code)]
    pub fn block_param(mut self, ty: CraneliftType) -> (Self, CraneliftValue) {
        let val = self.func.fresh_value(ty);
        if let Some(b_idx) = self.current_block {
            if let Some(b) = self.func.block_mut(b_idx) {
                b.params.push(val.clone());
            }
        }
        (self, val)
    }
    /// Consume the builder and return the completed function.
    #[allow(dead_code)]
    pub fn finish(self) -> CraneliftFunction {
        self.func
    }
}
/// Cranelift IR type representation.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CraneliftType {
    /// 1-bit boolean / integer
    B1,
    /// 8-bit integer
    I8,
    /// 16-bit integer
    I16,
    /// 32-bit integer
    I32,
    /// 64-bit integer
    I64,
    /// 128-bit integer
    I128,
    /// 32-bit float (IEEE 754 single-precision)
    F32,
    /// 64-bit float (IEEE 754 double-precision)
    F64,
    /// 32-bit reference (GC-managed pointer)
    R32,
    /// 64-bit reference (GC-managed pointer)
    R64,
    /// SIMD vector: `i32x4`, `f64x2`, etc.
    Vector(Box<CraneliftType>, u32),
    /// No type (used for void results / side-effecting instructions)
    Void,
}
impl CraneliftType {
    /// Return the byte width of this type (None for non-scalar or void).
    pub fn byte_width(&self) -> Option<u32> {
        match self {
            CraneliftType::B1 => Some(1),
            CraneliftType::I8 => Some(1),
            CraneliftType::I16 => Some(2),
            CraneliftType::I32 => Some(4),
            CraneliftType::I64 => Some(8),
            CraneliftType::I128 => Some(16),
            CraneliftType::F32 => Some(4),
            CraneliftType::F64 => Some(8),
            CraneliftType::R32 => Some(4),
            CraneliftType::R64 => Some(8),
            CraneliftType::Vector(base, lanes) => base.byte_width().map(|w| w * lanes),
            CraneliftType::Void => None,
        }
    }
    /// Return true if this is an integer type.
    pub fn is_int(&self) -> bool {
        matches!(
            self,
            CraneliftType::I8
                | CraneliftType::I16
                | CraneliftType::I32
                | CraneliftType::I64
                | CraneliftType::I128
                | CraneliftType::B1
        )
    }
    /// Return true if this is a floating-point type.
    pub fn is_float(&self) -> bool {
        matches!(self, CraneliftType::F32 | CraneliftType::F64)
    }
}
/// A basic block in Cranelift IR.
#[derive(Debug, Clone, PartialEq)]
pub struct CraneliftBlock {
    /// Block ID (e.g. block0, block1)
    pub id: u32,
    /// Block parameters (like phi nodes in SSA form — Cranelift uses explicit params)
    pub params: Vec<CraneliftValue>,
    /// Instructions in this block
    pub instrs: Vec<CraneliftInstResult>,
}
impl CraneliftBlock {
    /// Create a new block.
    pub fn new(id: u32) -> Self {
        CraneliftBlock {
            id,
            params: vec![],
            instrs: vec![],
        }
    }
    /// Create a block with parameters.
    pub fn with_params(id: u32, params: Vec<CraneliftValue>) -> Self {
        CraneliftBlock {
            id,
            params,
            instrs: vec![],
        }
    }
    /// Return the block reference for this block.
    pub fn block_ref(&self) -> BlockRef {
        BlockRef::new(self.id)
    }
    /// Append an instruction with a result value.
    pub fn push_with_result(&mut self, result: CraneliftValue, instr: CraneliftInstr) {
        self.instrs
            .push(CraneliftInstResult::with_result(result, instr));
    }
    /// Append a void instruction.
    pub fn push_void(&mut self, instr: CraneliftInstr) {
        self.instrs.push(CraneliftInstResult::no_result(instr));
    }
    /// Return true if this block ends with a terminator instruction.
    pub fn is_terminated(&self) -> bool {
        self.instrs.last().is_some_and(|ir| {
            matches!(
                ir.instr,
                CraneliftInstr::Jump(_, _)
                    | CraneliftInstr::Brif(_, _, _, _, _)
                    | CraneliftInstr::BrTable(_, _, _)
                    | CraneliftInstr::Return(_)
                    | CraneliftInstr::Trap(_)
                    | CraneliftInstr::Unreachable
                    | CraneliftInstr::ReturnCall(_, _)
            )
        })
    }
    /// Emit this block as textual IR.
    pub fn emit(&self) -> String {
        let mut s = String::new();
        if self.params.is_empty() {
            s.push_str(&format!("block{}:\n", self.id));
        } else {
            let params = self
                .params
                .iter()
                .map(|v| format!("{}: {}", v, v.ty))
                .collect::<Vec<_>>()
                .join(", ");
            s.push_str(&format!("block{}({}):\n", self.id, params));
        }
        for ir in &self.instrs {
            s.push_str(&format!("    {}\n", ir.emit()));
        }
        s
    }
}
/// Helpers for recognizing common Cranelift instruction patterns.
#[allow(dead_code)]
pub struct CraneliftInstPattern;
impl CraneliftInstPattern {
    /// Return true if `instr` is a pure arithmetic (no side effects, no memory) instruction.
    #[allow(dead_code)]
    pub fn is_pure_arith(instr: &CraneliftInstr) -> bool {
        matches!(
            instr,
            CraneliftInstr::Iconst(..)
                | CraneliftInstr::F32Const(..)
                | CraneliftInstr::F64Const(..)
                | CraneliftInstr::Iadd(..)
                | CraneliftInstr::Isub(..)
                | CraneliftInstr::Imul(..)
                | CraneliftInstr::Sdiv(..)
                | CraneliftInstr::Udiv(..)
                | CraneliftInstr::Srem(..)
                | CraneliftInstr::Urem(..)
                | CraneliftInstr::Ineg(..)
                | CraneliftInstr::Iabs(..)
                | CraneliftInstr::IaddImm(..)
                | CraneliftInstr::ImulImm(..)
                | CraneliftInstr::Band(..)
                | CraneliftInstr::Bor(..)
                | CraneliftInstr::Bxor(..)
                | CraneliftInstr::Bnot(..)
                | CraneliftInstr::Ishl(..)
                | CraneliftInstr::Sshr(..)
                | CraneliftInstr::Ushr(..)
                | CraneliftInstr::Rotl(..)
                | CraneliftInstr::Rotr(..)
                | CraneliftInstr::Clz(..)
                | CraneliftInstr::Ctz(..)
                | CraneliftInstr::Popcnt(..)
                | CraneliftInstr::Fadd(..)
                | CraneliftInstr::Fsub(..)
                | CraneliftInstr::Fmul(..)
                | CraneliftInstr::Fdiv(..)
                | CraneliftInstr::Fneg(..)
                | CraneliftInstr::Fabs(..)
                | CraneliftInstr::Sqrt(..)
                | CraneliftInstr::Ceil(..)
                | CraneliftInstr::Floor(..)
                | CraneliftInstr::FTrunc(..)
                | CraneliftInstr::Nearest(..)
                | CraneliftInstr::Fmin(..)
                | CraneliftInstr::Fmax(..)
                | CraneliftInstr::Icmp(..)
                | CraneliftInstr::Fcmp(..)
        )
    }
    /// Return true if `instr` is a terminator (ends a block).
    #[allow(dead_code)]
    pub fn is_terminator(instr: &CraneliftInstr) -> bool {
        matches!(
            instr,
            CraneliftInstr::Return(..)
                | CraneliftInstr::Jump(..)
                | CraneliftInstr::Brif(..)
                | CraneliftInstr::BrTable(..)
                | CraneliftInstr::Trap(..)
        )
    }
    /// Return true if `instr` has side effects (memory or control flow).
    #[allow(dead_code)]
    pub fn has_side_effects(instr: &CraneliftInstr) -> bool {
        matches!(
            instr,
            CraneliftInstr::Store(..)
                | CraneliftInstr::Return(..)
                | CraneliftInstr::Jump(..)
                | CraneliftInstr::Brif(..)
                | CraneliftInstr::BrTable(..)
                | CraneliftInstr::Trap(..)
                | CraneliftInstr::Call(..)
        )
    }
    /// Try to extract the constant value from an `Iconst` instruction.
    #[allow(dead_code)]
    pub fn iconst_value(instr: &CraneliftInstr) -> Option<i64> {
        match instr {
            CraneliftInstr::Iconst(_, n) => Some(*n),
            _ => None,
        }
    }
}
/// Cranelift IR code generation backend.
pub struct CraneliftBackend {
    /// The module being built
    pub module: CraneliftModule,
    /// Current function being constructed (if any)
    pub(super) current_func: Option<CraneliftFunction>,
    /// Current block id being populated (if any)
    pub(super) current_block: Option<u32>,
}
impl CraneliftBackend {
    /// Create a new Cranelift backend.
    pub fn new(module_name: impl Into<String>) -> Self {
        CraneliftBackend {
            module: CraneliftModule::new(module_name),
            current_func: None,
            current_block: None,
        }
    }
    /// Start building a new function.
    pub fn begin_function(&mut self, name: impl Into<String>, sig: Signature) {
        let mut func = CraneliftFunction::new(name, sig);
        let entry_id = func.new_block();
        for ty in func.sig.params.clone() {
            let v = func.fresh_value(ty.clone());
            if let Some(block) = func.blocks.iter_mut().find(|b| b.id == entry_id) {
                block.params.push(v);
            }
        }
        self.current_func = Some(func);
        self.current_block = Some(entry_id);
    }
    /// End the current function and add it to the module.
    pub fn end_function(&mut self) {
        if let Some(func) = self.current_func.take() {
            self.module.add_function(func);
        }
        self.current_block = None;
    }
    /// Switch to an existing block.
    pub fn switch_to_block(&mut self, block_id: u32) {
        self.current_block = Some(block_id);
    }
    /// Allocate a fresh SSA value in the current function.
    pub fn fresh_value(&mut self, ty: CraneliftType) -> Option<CraneliftValue> {
        self.current_func.as_mut().map(|f| f.fresh_value(ty))
    }
    /// Create a new block in the current function.
    pub fn new_block(&mut self) -> Option<u32> {
        self.current_func.as_mut().map(|f| f.new_block())
    }
    /// Emit an instruction with a result into the current block.
    pub fn emit_with_result(
        &mut self,
        ty: CraneliftType,
        instr: CraneliftInstr,
    ) -> Option<CraneliftValue> {
        let v = self.fresh_value(ty)?;
        let block_id = self.current_block?;
        let func = self.current_func.as_mut()?;
        if let Some(block) = func.block_mut(block_id) {
            block.push_with_result(v.clone(), instr);
        }
        Some(v)
    }
    /// Emit a void instruction into the current block.
    pub fn emit_void(&mut self, instr: CraneliftInstr) {
        let block_id = match self.current_block {
            Some(id) => id,
            None => return,
        };
        if let Some(func) = self.current_func.as_mut() {
            if let Some(block) = func.block_mut(block_id) {
                block.push_void(instr);
            }
        }
    }
    /// Emit an `iconst` instruction.
    pub fn iconst(&mut self, ty: CraneliftType, val: i64) -> Option<CraneliftValue> {
        self.emit_with_result(ty.clone(), CraneliftInstr::Iconst(ty, val))
    }
    /// Emit an `iadd` instruction.
    pub fn iadd(&mut self, a: CraneliftValue, b: CraneliftValue) -> Option<CraneliftValue> {
        let ty = a.ty.clone();
        self.emit_with_result(ty, CraneliftInstr::Iadd(a, b))
    }
    /// Emit an `isub` instruction.
    pub fn isub(&mut self, a: CraneliftValue, b: CraneliftValue) -> Option<CraneliftValue> {
        let ty = a.ty.clone();
        self.emit_with_result(ty, CraneliftInstr::Isub(a, b))
    }
    /// Emit an `imul` instruction.
    pub fn imul(&mut self, a: CraneliftValue, b: CraneliftValue) -> Option<CraneliftValue> {
        let ty = a.ty.clone();
        self.emit_with_result(ty, CraneliftInstr::Imul(a, b))
    }
    /// Emit an `sdiv` instruction.
    pub fn sdiv(&mut self, a: CraneliftValue, b: CraneliftValue) -> Option<CraneliftValue> {
        let ty = a.ty.clone();
        self.emit_with_result(ty, CraneliftInstr::Sdiv(a, b))
    }
    /// Emit an `icmp` instruction.
    pub fn icmp(
        &mut self,
        cc: IntCC,
        a: CraneliftValue,
        b: CraneliftValue,
    ) -> Option<CraneliftValue> {
        self.emit_with_result(CraneliftType::B1, CraneliftInstr::Icmp(cc, a, b))
    }
    /// Emit an `fcmp` instruction.
    pub fn fcmp(
        &mut self,
        cc: FloatCC,
        a: CraneliftValue,
        b: CraneliftValue,
    ) -> Option<CraneliftValue> {
        self.emit_with_result(CraneliftType::B1, CraneliftInstr::Fcmp(cc, a, b))
    }
    /// Emit a `load` instruction.
    pub fn load(
        &mut self,
        ty: CraneliftType,
        flags: MemFlags,
        addr: CraneliftValue,
        offset: i32,
    ) -> Option<CraneliftValue> {
        self.emit_with_result(ty.clone(), CraneliftInstr::Load(ty, flags, addr, offset))
    }
    /// Emit a `store` instruction.
    pub fn store(
        &mut self,
        flags: MemFlags,
        val: CraneliftValue,
        addr: CraneliftValue,
        offset: i32,
    ) {
        self.emit_void(CraneliftInstr::Store(flags, val, addr, offset));
    }
    /// Emit a `call` instruction returning a single value.
    pub fn call_single(
        &mut self,
        ret_ty: CraneliftType,
        func: impl Into<String>,
        args: Vec<CraneliftValue>,
    ) -> Option<CraneliftValue> {
        self.emit_with_result(ret_ty, CraneliftInstr::Call(func.into(), args))
    }
    /// Emit a `jump` terminator.
    pub fn jump(&mut self, target: BlockRef, args: Vec<CraneliftValue>) {
        self.emit_void(CraneliftInstr::Jump(target, args));
    }
    /// Emit a `brif` terminator.
    pub fn brif(
        &mut self,
        cond: CraneliftValue,
        t: BlockRef,
        t_args: Vec<CraneliftValue>,
        f: BlockRef,
        f_args: Vec<CraneliftValue>,
    ) {
        self.emit_void(CraneliftInstr::Brif(cond, t, t_args, f, f_args));
    }
    /// Emit a `return` terminator.
    pub fn emit_return(&mut self, vals: Vec<CraneliftValue>) {
        self.emit_void(CraneliftInstr::Return(vals));
    }
    /// Emit the full module as textual IR.
    pub fn emit_module(&self) -> String {
        self.module.emit()
    }
}
/// Stack slot allocator for a function.
#[derive(Debug, Default)]
#[allow(dead_code)]
pub struct CraneliftStackAllocator {
    pub(super) slots: Vec<CraneliftStackSlot>,
    pub(super) next_id: u32,
    pub(super) frame_size: u32,
}
impl CraneliftStackAllocator {
    /// Create a new stack allocator.
    #[allow(dead_code)]
    pub fn new() -> Self {
        CraneliftStackAllocator::default()
    }
    /// Allocate a new stack slot with the given size and alignment.
    #[allow(dead_code)]
    pub fn alloc(&mut self, size: u32, align: u32) -> &CraneliftStackSlot {
        let aligned = (self.frame_size + align - 1) & !(align - 1);
        self.frame_size = aligned + size;
        let slot = CraneliftStackSlot::new(self.next_id, size, align);
        self.next_id += 1;
        self.slots.push(slot);
        self.slots
            .last()
            .expect("slots is non-empty after push; invariant guaranteed by alloc")
    }
    /// Allocate a named stack slot.
    #[allow(dead_code)]
    pub fn alloc_named(
        &mut self,
        size: u32,
        align: u32,
        name: impl Into<String>,
    ) -> &CraneliftStackSlot {
        let aligned = (self.frame_size + align - 1) & !(align - 1);
        self.frame_size = aligned + size;
        let slot = CraneliftStackSlot::named(self.next_id, size, align, name);
        self.next_id += 1;
        self.slots.push(slot);
        self.slots
            .last()
            .expect("slots is non-empty after push; invariant guaranteed by alloc_named")
    }
    /// Return all allocated slots.
    #[allow(dead_code)]
    pub fn slots(&self) -> &[CraneliftStackSlot] {
        &self.slots
    }
    /// Return the total frame size.
    #[allow(dead_code)]
    pub fn frame_size(&self) -> u32 {
        self.frame_size
    }
    /// Emit all slot declarations.
    #[allow(dead_code)]
    pub fn emit_decls(&self) -> String {
        self.slots
            .iter()
            .map(|s| s.emit())
            .collect::<Vec<_>>()
            .join("\n")
    }
}
/// A data object in the Cranelift module (global variable / constant).
#[derive(Debug, Clone, PartialEq)]
pub struct CraneliftDataObject {
    /// Symbol name
    pub name: String,
    /// Whether the data is read-only
    pub is_readonly: bool,
    /// Data contents (raw bytes)
    pub data: Vec<u8>,
    /// Alignment in bytes
    pub align: u32,
}
impl CraneliftDataObject {
    /// Create a new read-only data object.
    pub fn readonly(name: impl Into<String>, data: Vec<u8>, align: u32) -> Self {
        CraneliftDataObject {
            name: name.into(),
            is_readonly: true,
            data,
            align,
        }
    }
    /// Create a writable data object.
    pub fn writable(name: impl Into<String>, data: Vec<u8>, align: u32) -> Self {
        CraneliftDataObject {
            name: name.into(),
            is_readonly: false,
            data,
            align,
        }
    }
    /// Emit this data object as textual IR.
    pub fn emit(&self) -> String {
        let rw = if self.is_readonly { "rodata" } else { "data" };
        let hex: String = self.data.iter().map(|b| format!("\\{:02x}", b)).collect();
        format!(
            "{} %{} align={} {{\n    ascii \"{}\"\n}}\n",
            rw, self.name, self.align, hex
        )
    }
}
/// ABI parameter classification.
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(dead_code)]
pub enum CraneliftABIClass {
    /// Passed in integer register.
    Integer,
    /// Passed in floating-point register.
    Float,
    /// Passed in SSE register (packed integers).
    SSE,
    /// Passed on the stack.
    Memory,
    /// Not passed (zero-size or erased).
    NoValue,
}
/// ABI layout for a function parameter or return value.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct CraneliftABIParam {
    /// The IR type.
    pub ty: CraneliftType,
    /// How this parameter is classified.
    pub class: CraneliftABIClass,
    /// Stack offset (if passed on stack), or register index.
    pub location: i32,
}
impl CraneliftABIParam {
    /// Create a new ABI parameter.
    #[allow(dead_code)]
    pub fn new(ty: CraneliftType, class: CraneliftABIClass, location: i32) -> Self {
        CraneliftABIParam {
            ty,
            class,
            location,
        }
    }
    /// Return true if this parameter is register-allocated.
    #[allow(dead_code)]
    pub fn is_register(&self) -> bool {
        !matches!(
            self.class,
            CraneliftABIClass::Memory | CraneliftABIClass::NoValue
        )
    }
}
/// Which optimization passes to run on Cranelift IR.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct CraneliftPassConfig {
    /// Enable constant folding.
    pub const_folding: bool,
    /// Enable dead code elimination.
    pub dce: bool,
    /// Enable instruction combining (e.g., add + mul → madd).
    pub inst_combine: bool,
    /// Enable branch optimization.
    pub branch_opt: bool,
    /// Enable loop-invariant code motion.
    pub licm: bool,
    /// Enable register coalescing.
    pub reg_coalescing: bool,
    /// Enable redundant load elimination.
    pub load_elim: bool,
    /// Enable tail call optimization.
    pub tail_call_opt: bool,
    /// Maximum inlining depth.
    pub inline_depth: u32,
    /// Whether to emit debug information.
    pub debug_info: bool,
}
impl CraneliftPassConfig {
    /// Return a no-optimization configuration.
    #[allow(dead_code)]
    pub fn no_opt() -> Self {
        CraneliftPassConfig {
            const_folding: false,
            dce: false,
            inst_combine: false,
            branch_opt: false,
            licm: false,
            reg_coalescing: false,
            load_elim: false,
            tail_call_opt: false,
            inline_depth: 0,
            debug_info: false,
        }
    }
    /// Return a maximum optimization configuration.
    #[allow(dead_code)]
    pub fn max_opt() -> Self {
        CraneliftPassConfig {
            const_folding: true,
            dce: true,
            inst_combine: true,
            branch_opt: true,
            licm: true,
            reg_coalescing: true,
            load_elim: true,
            tail_call_opt: true,
            inline_depth: 10,
            debug_info: false,
        }
    }
}
/// A block reference — corresponds to `block0`, `block1`, etc.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BlockRef {
    /// Numeric ID for this block
    pub id: u32,
}
impl BlockRef {
    /// Create a new block reference.
    pub fn new(id: u32) -> Self {
        BlockRef { id }
    }
}
/// Memory access flags for load/store instructions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemFlags {
    /// True if the access is aligned (UB if not)
    pub aligned: bool,
    /// True if the access is volatile (not reordered)
    pub notrap: bool,
    /// True if the access does not alias any other access
    pub readonly: bool,
}
impl MemFlags {
    /// Create default (safe) memory flags.
    pub fn new() -> Self {
        MemFlags {
            aligned: false,
            notrap: false,
            readonly: false,
        }
    }
    /// Create trusted (notrap) memory flags.
    pub fn trusted() -> Self {
        MemFlags {
            aligned: false,
            notrap: true,
            readonly: false,
        }
    }
    /// Create aligned, notrap flags.
    pub fn aligned_notrap() -> Self {
        MemFlags {
            aligned: true,
            notrap: true,
            readonly: false,
        }
    }
}
/// Known calling conventions for Cranelift.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(dead_code)]
pub enum CraneliftCallingConvention {
    /// System V AMD64 ABI (Linux, macOS x86-64).
    SystemV,
    /// Windows x64 calling convention.
    WindowsFastcall,
    /// Wasmtime calling convention (WebAssembly).
    WasmtimeSystem,
    /// Cold (rarely-called) function convention.
    Cold,
    /// Tail-call optimized convention.
    Tail,
    /// Fast (internal) convention (no caller-saves).
    Fast,
}
/// Debug information attached to a Cranelift function.
#[derive(Debug, Clone, Default)]
#[allow(dead_code)]
pub struct CraneliftDebugInfo {
    /// Source file name.
    pub source_file: Option<String>,
    /// Source function name (may differ from IR name).
    pub source_function: Option<String>,
    /// Mapping from instruction offset to (line, column) in source.
    pub location_map: Vec<(usize, u32, u32)>,
    /// Local variable names: (ir_value, source_name).
    pub var_names: HashMap<String, String>,
}
impl CraneliftDebugInfo {
    /// Create a new debug info record.
    #[allow(dead_code)]
    pub fn new(source_file: impl Into<String>, source_function: impl Into<String>) -> Self {
        CraneliftDebugInfo {
            source_file: Some(source_file.into()),
            source_function: Some(source_function.into()),
            location_map: Vec::new(),
            var_names: HashMap::new(),
        }
    }
    /// Record a source location for an instruction.
    #[allow(dead_code)]
    pub fn add_location(&mut self, offset: usize, line: u32, column: u32) {
        self.location_map.push((offset, line, column));
    }
    /// Record a variable name mapping.
    #[allow(dead_code)]
    pub fn add_var_name(&mut self, ir_name: impl Into<String>, source_name: impl Into<String>) {
        self.var_names.insert(ir_name.into(), source_name.into());
    }
    /// Emit debug info as comments.
    #[allow(dead_code)]
    pub fn emit_comments(&self) -> String {
        let mut out = String::new();
        if let Some(ref f) = self.source_file {
            out.push_str(&format!("; source_file: {}\n", f));
        }
        if let Some(ref f) = self.source_function {
            out.push_str(&format!("; source_function: {}\n", f));
        }
        for (offset, line, col) in &self.location_map {
            out.push_str(&format!("; @{}: {}:{}\n", offset, line, col));
        }
        for (ir, src) in &self.var_names {
            out.push_str(&format!("; var {} => {}\n", ir, src));
        }
        out
    }
}
/// A Cranelift global value declaration.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct CraneliftGlobalValue {
    /// Global value ID.
    pub id: u32,
    /// The global value type.
    pub ty: CraneliftType,
    /// How the value is obtained.
    pub def: CraneliftGlobalValueDef,
}
impl CraneliftGlobalValue {
    /// Create a symbol global value.
    #[allow(dead_code)]
    pub fn symbol(id: u32, name: impl Into<String>, colocated: bool) -> Self {
        CraneliftGlobalValue {
            id,
            ty: CraneliftType::I64,
            def: CraneliftGlobalValueDef::Symbol {
                name: name.into(),
                colocated,
            },
        }
    }
    /// Emit the global value declaration.
    #[allow(dead_code)]
    pub fn emit(&self) -> String {
        match &self.def {
            CraneliftGlobalValueDef::Symbol { name, colocated } => {
                format!(
                    "gv{} = symbol {}{}",
                    self.id,
                    if *colocated { "colocated " } else { "" },
                    name
                )
            }
            CraneliftGlobalValueDef::IAddImm { base, offset } => {
                format!("gv{} = iadd_imm gv{}, {}", self.id, base, offset)
            }
            CraneliftGlobalValueDef::Load {
                base,
                offset,
                global_type,
                readonly,
            } => {
                format!(
                    "gv{} = load.{} {}[{}]{}",
                    self.id,
                    global_type,
                    base,
                    offset,
                    if *readonly { " readonly" } else { "" }
                )
            }
        }
    }
}
/// WebAssembly-style heap (linear memory) configuration for Cranelift.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct CraneliftHeapConfig {
    /// Base global value (pointer to start of heap).
    pub base: u32,
    /// Minimum heap size in bytes.
    pub min_size: u64,
    /// Maximum heap size (None = unlimited).
    pub max_size: Option<u64>,
    /// Page size (typically 65536 for Wasm).
    pub page_size: u64,
    /// Whether bounds checking is needed.
    pub needs_bounds_check: bool,
    /// Guard size in bytes (0 = no guard pages).
    pub guard_size: u64,
}
impl CraneliftHeapConfig {
    /// Create a 4 GiB static heap (no guard needed in 64-bit mode).
    #[allow(dead_code)]
    pub fn static_4gib() -> Self {
        CraneliftHeapConfig {
            min_size: 65536,
            max_size: Some(4 * 1024 * 1024 * 1024),
            guard_size: 2 * 1024 * 1024 * 1024,
            needs_bounds_check: false,
            ..Default::default()
        }
    }
    /// Emit a comment describing the heap configuration.
    #[allow(dead_code)]
    pub fn emit_comment(&self) -> String {
        format!(
            "; heap base=gv{} min={} max={} page={} guard={} bounds_check={}",
            self.base,
            self.min_size,
            self.max_size
                .map(|m| m.to_string())
                .unwrap_or_else(|| "unlimited".to_string()),
            self.page_size,
            self.guard_size,
            self.needs_bounds_check,
        )
    }
}
/// A stack slot in a Cranelift function frame.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct CraneliftStackSlot {
    /// Slot identifier.
    pub id: u32,
    /// Size in bytes.
    pub size: u32,
    /// Alignment in bytes.
    pub align: u32,
    /// Optional debug name.
    pub name: Option<String>,
}
impl CraneliftStackSlot {
    /// Create a new stack slot.
    #[allow(dead_code)]
    pub fn new(id: u32, size: u32, align: u32) -> Self {
        CraneliftStackSlot {
            id,
            size,
            align,
            name: None,
        }
    }
    /// Create a named stack slot.
    #[allow(dead_code)]
    pub fn named(id: u32, size: u32, align: u32, name: impl Into<String>) -> Self {
        CraneliftStackSlot {
            id,
            size,
            align,
            name: Some(name.into()),
        }
    }
    /// Emit the stack slot declaration.
    #[allow(dead_code)]
    pub fn emit(&self) -> String {
        let name_comment = self
            .name
            .as_ref()
            .map(|n| format!(" ; {}", n))
            .unwrap_or_default();
        format!(
            "ss{} = explicit_slot {}, align = {}{}",
            self.id, self.size, self.align, name_comment
        )
    }
    /// Generate a `stack_addr` expression for this slot at the given offset.
    #[allow(dead_code)]
    pub fn addr_expr(&self, offset: i32) -> String {
        format!("stack_addr.i64 ss{}, {}", self.id, offset)
    }
}
/// A Cranelift IR instruction.
#[derive(Debug, Clone, PartialEq)]
pub enum CraneliftInstr {
    /// Integer constant: `iconst.i64 42`
    Iconst(CraneliftType, i64),
    /// Boolean constant: `bconst.b1 true`
    Bconst(bool),
    /// 32-bit float constant: `f32const 3.14`
    F32Const(f32),
    /// 64-bit float constant: `f64const 3.14`
    F64Const(f64),
    /// Add: `iadd v1, v2`
    Iadd(CraneliftValue, CraneliftValue),
    /// Subtract: `isub v1, v2`
    Isub(CraneliftValue, CraneliftValue),
    /// Multiply: `imul v1, v2`
    Imul(CraneliftValue, CraneliftValue),
    /// Signed divide: `sdiv v1, v2`
    Sdiv(CraneliftValue, CraneliftValue),
    /// Unsigned divide: `udiv v1, v2`
    Udiv(CraneliftValue, CraneliftValue),
    /// Signed remainder: `srem v1, v2`
    Srem(CraneliftValue, CraneliftValue),
    /// Unsigned remainder: `urem v1, v2`
    Urem(CraneliftValue, CraneliftValue),
    /// Negate: `ineg v1`
    Ineg(CraneliftValue),
    /// Absolute value: `iabs v1`
    Iabs(CraneliftValue),
    /// Add with immediate: `iadd_imm v1, 5`
    IaddImm(CraneliftValue, i64),
    /// Multiply with immediate: `imul_imm v1, 5`
    ImulImm(CraneliftValue, i64),
    /// Bitwise AND: `band v1, v2`
    Band(CraneliftValue, CraneliftValue),
    /// Bitwise OR: `bor v1, v2`
    Bor(CraneliftValue, CraneliftValue),
    /// Bitwise XOR: `bxor v1, v2`
    Bxor(CraneliftValue, CraneliftValue),
    /// Bitwise NOT: `bnot v1`
    Bnot(CraneliftValue),
    /// Shift left: `ishl v1, v2`
    Ishl(CraneliftValue, CraneliftValue),
    /// Arithmetic shift right: `sshr v1, v2`
    Sshr(CraneliftValue, CraneliftValue),
    /// Logical shift right: `ushr v1, v2`
    Ushr(CraneliftValue, CraneliftValue),
    /// Rotate left: `rotl v1, v2`
    Rotl(CraneliftValue, CraneliftValue),
    /// Rotate right: `rotr v1, v2`
    Rotr(CraneliftValue, CraneliftValue),
    /// Count leading zeros: `clz v1`
    Clz(CraneliftValue),
    /// Count trailing zeros: `ctz v1`
    Ctz(CraneliftValue),
    /// Population count: `popcnt v1`
    Popcnt(CraneliftValue),
    /// Float add: `fadd v1, v2`
    Fadd(CraneliftValue, CraneliftValue),
    /// Float subtract: `fsub v1, v2`
    Fsub(CraneliftValue, CraneliftValue),
    /// Float multiply: `fmul v1, v2`
    Fmul(CraneliftValue, CraneliftValue),
    /// Float divide: `fdiv v1, v2`
    Fdiv(CraneliftValue, CraneliftValue),
    /// Float negate: `fneg v1`
    Fneg(CraneliftValue),
    /// Float absolute value: `fabs v1`
    Fabs(CraneliftValue),
    /// Float square root: `sqrt v1`
    Sqrt(CraneliftValue),
    /// Fused multiply-add: `fma v1, v2, v3`
    Fma(CraneliftValue, CraneliftValue, CraneliftValue),
    /// Float minimum: `fmin v1, v2`
    Fmin(CraneliftValue, CraneliftValue),
    /// Float maximum: `fmax v1, v2`
    Fmax(CraneliftValue, CraneliftValue),
    /// Float floor: `floor v1`
    Floor(CraneliftValue),
    /// Float ceiling: `ceil v1`
    Ceil(CraneliftValue),
    /// Float truncate toward zero: `trunc v1`
    FTrunc(CraneliftValue),
    /// Float round to nearest: `nearest v1`
    Nearest(CraneliftValue),
    /// Integer compare: `icmp eq v1, v2`
    Icmp(IntCC, CraneliftValue, CraneliftValue),
    /// Float compare: `fcmp lt v1, v2`
    Fcmp(FloatCC, CraneliftValue, CraneliftValue),
    /// Select: `select v_cond, v_true, v_false`
    Select(CraneliftValue, CraneliftValue, CraneliftValue),
    /// Sign-extend: `sextend.i64 v1`
    Sextend(CraneliftType, CraneliftValue),
    /// Zero-extend: `uextend.i64 v1`
    Uextend(CraneliftType, CraneliftValue),
    /// Truncate: `ireduce.i32 v1`
    Ireduce(CraneliftType, CraneliftValue),
    /// Float convert: `fpromote.f64 v1`
    Fpromote(CraneliftType, CraneliftValue),
    /// Float demote: `fdemote.f32 v1`
    Fdemote(CraneliftType, CraneliftValue),
    /// Float to int (trunc): `fcvt_to_sint.i64 v1`
    FcvtToSint(CraneliftType, CraneliftValue),
    /// Float to unsigned int (trunc): `fcvt_to_uint.i64 v1`
    FcvtToUint(CraneliftType, CraneliftValue),
    /// Signed int to float: `fcvt_from_sint.f64 v1`
    FcvtFromSint(CraneliftType, CraneliftValue),
    /// Unsigned int to float: `fcvt_from_uint.f64 v1`
    FcvtFromUint(CraneliftType, CraneliftValue),
    /// Bitcast: `bitcast.f64 v1`
    Bitcast(CraneliftType, CraneliftValue),
    /// Load: `load.i64 notrap aligned v_addr+offset`
    Load(CraneliftType, MemFlags, CraneliftValue, i32),
    /// Store: `store notrap aligned v_val, v_addr+offset`
    Store(MemFlags, CraneliftValue, CraneliftValue, i32),
    /// Stack slot address: `stack_addr.i64 ss0`
    StackAddr(CraneliftType, u32),
    /// Global value address: `global_value.i64 gv0`
    GlobalValue(CraneliftType, u32),
    /// Unconditional jump: `jump block1(v1, v2)`
    Jump(BlockRef, Vec<CraneliftValue>),
    /// Conditional branch: `brif v_cond, block1(args), block2(args)`
    Brif(
        CraneliftValue,
        BlockRef,
        Vec<CraneliftValue>,
        BlockRef,
        Vec<CraneliftValue>,
    ),
    /// Branch table (indirect jump): `br_table v, block_default, jt0`
    BrTable(CraneliftValue, BlockRef, Vec<BlockRef>),
    /// Return: `return v1, v2`
    Return(Vec<CraneliftValue>),
    /// Trap: `trap user1`
    Trap(String),
    /// Trap if condition: `trapif eq v1, user1`
    Trapif(IntCC, CraneliftValue, String),
    /// Unreachable trap
    Unreachable,
    /// Direct call: `call func(args)`
    Call(String, Vec<CraneliftValue>),
    /// Indirect call: `call_indirect sig0, v_callee(args)`
    CallIndirect(u32, CraneliftValue, Vec<CraneliftValue>),
    /// Return call (tail call): `return_call func(args)`
    ReturnCall(String, Vec<CraneliftValue>),
    /// Function argument: `func_addr.i64 func_name`
    FuncAddr(CraneliftType, String),
    /// Null reference constant: `null.r64`
    Null(CraneliftType),
    /// Vector splat: `splat.i32x4 v1`
    Splat(CraneliftType, CraneliftValue),
    /// Vector extract lane: `extractlane v1, 0`
    ExtractLane(CraneliftValue, u8),
    /// Vector insert lane: `insertlane v1, 0, v2`
    InsertLane(CraneliftValue, u8, CraneliftValue),
    /// Copy / identity: `copy v1` (used for renames)
    Copy(CraneliftValue),
    /// Nop
    Nop,
}
/// Float comparison condition codes for `fcmp`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FloatCC {
    /// Ordered and equal
    Equal,
    /// Ordered and not equal
    NotEqual,
    /// Ordered and less than
    LessThan,
    /// Ordered and less than or equal
    LessThanOrEqual,
    /// Ordered and greater than
    GreaterThan,
    /// Ordered and greater than or equal
    GreaterThanOrEqual,
    /// Ordered (neither is NaN)
    Ordered,
    /// Unordered (at least one is NaN)
    Unordered,
    /// Unordered or equal
    UnorderedOrEqual,
    /// Unordered or less than
    UnorderedOrLessThan,
    /// Unordered or greater than
    UnorderedOrGreaterThan,
}
/// Integer comparison condition codes for `icmp`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IntCC {
    /// Equal (`==`)
    Equal,
    /// Not equal (`!=`)
    NotEqual,
    /// Signed less than (`<`)
    SignedLessThan,
    /// Signed less than or equal (`<=`)
    SignedLessThanOrEqual,
    /// Signed greater than (`>`)
    SignedGreaterThan,
    /// Signed greater than or equal (`>=`)
    SignedGreaterThanOrEqual,
    /// Unsigned less than
    UnsignedLessThan,
    /// Unsigned less than or equal
    UnsignedLessThanOrEqual,
    /// Unsigned greater than
    UnsignedGreaterThan,
    /// Unsigned greater than or equal
    UnsignedGreaterThanOrEqual,
    /// Overflow: signed add overflows
    Overflow,
    /// No overflow
    NotOverflow,
}
/// An SSA value reference in Cranelift IR — corresponds to `v0`, `v1`, etc.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CraneliftValue {
    /// Numeric ID for this SSA value
    pub id: u32,
    /// The type of this value
    pub ty: CraneliftType,
}
impl CraneliftValue {
    /// Create a new SSA value.
    pub fn new(id: u32, ty: CraneliftType) -> Self {
        CraneliftValue { id, ty }
    }
}
/// Calling convention.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CallConv {
    /// Fast calling convention (Cranelift internal)
    Fast,
    /// Cold calling convention (infrequently called)
    Cold,
    /// System V AMD64 ABI
    SystemV,
    /// Windows x64 ABI
    WindowsFastcall,
    /// WebAssembly calling convention
    WasmtimeSystemV,
}
/// Static metrics about a Cranelift function.
#[derive(Debug, Clone, Default)]
#[allow(dead_code)]
pub struct CraneliftCodeMetrics {
    /// Number of basic blocks.
    pub num_blocks: usize,
    /// Total number of instructions.
    pub total_instructions: usize,
    /// Number of value-producing instructions.
    pub num_value_instructions: usize,
    /// Number of void (side-effecting) instructions.
    pub num_void_instructions: usize,
    /// Number of block parameters across all blocks.
    pub total_block_params: usize,
    /// Number of call instructions.
    pub num_calls: usize,
    /// Number of load instructions.
    pub num_loads: usize,
    /// Number of store instructions.
    pub num_stores: usize,
    /// Number of branch instructions.
    pub num_branches: usize,
}
impl CraneliftCodeMetrics {
    /// Compute metrics for a function.
    #[allow(dead_code)]
    pub fn compute(func: &CraneliftFunction) -> Self {
        let mut m = CraneliftCodeMetrics {
            num_blocks: func.blocks.len(),
            ..Default::default()
        };
        for block in &func.blocks {
            m.total_block_params += block.params.len();
            for inst in &block.instrs {
                m.total_instructions += 1;
                if inst.result.is_some() {
                    m.num_value_instructions += 1;
                } else {
                    m.num_void_instructions += 1;
                }
                match &inst.instr {
                    CraneliftInstr::Call(..) => {
                        m.num_calls += 1;
                    }
                    CraneliftInstr::Load(..) => {
                        m.num_loads += 1;
                    }
                    CraneliftInstr::Store(..) => {
                        m.num_stores += 1;
                    }
                    CraneliftInstr::Brif(..)
                    | CraneliftInstr::BrTable(..)
                    | CraneliftInstr::Jump(..) => {
                        m.num_branches += 1;
                    }
                    _ => {}
                }
            }
        }
        m
    }
    /// Return a summary string.
    #[allow(dead_code)]
    pub fn summary(&self) -> String {
        format!(
            "blocks={} total_instrs={} values={} voids={} params={} calls={} loads={} stores={} branches={}",
            self.num_blocks, self.total_instructions, self.num_value_instructions, self
            .num_void_instructions, self.total_block_params, self.num_calls, self
            .num_loads, self.num_stores, self.num_branches,
        )
    }
}
/// An inline assembly fragment that can be embedded in a function.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct CraneliftInlineAsm {
    /// The assembly template string.
    pub template: String,
    /// Input operands: (constraint, value).
    pub inputs: Vec<(String, CraneliftValue)>,
    /// Output operands: (constraint, result_value).
    pub outputs: Vec<(String, CraneliftValue)>,
    /// Whether the assembly has side effects (prevents reordering).
    pub volatile: bool,
    /// Whether the assembly can trap.
    pub can_trap: bool,
}
impl CraneliftInlineAsm {
    /// Create a new inline assembly fragment.
    #[allow(dead_code)]
    pub fn new(template: impl Into<String>) -> Self {
        CraneliftInlineAsm {
            template: template.into(),
            inputs: Vec::new(),
            outputs: Vec::new(),
            volatile: true,
            can_trap: false,
        }
    }
    /// Add an input operand.
    #[allow(dead_code)]
    pub fn add_input(&mut self, constraint: impl Into<String>, val: CraneliftValue) {
        self.inputs.push((constraint.into(), val));
    }
    /// Add an output operand.
    #[allow(dead_code)]
    pub fn add_output(&mut self, constraint: impl Into<String>, val: CraneliftValue) {
        self.outputs.push((constraint.into(), val));
    }
    /// Emit a comment describing this inline asm fragment.
    #[allow(dead_code)]
    pub fn emit_comment(&self) -> String {
        format!(
            "; inline_asm template={:?} inputs={} outputs={} volatile={}",
            self.template,
            self.inputs.len(),
            self.outputs.len(),
            self.volatile,
        )
    }
}
/// A Cranelift function definition.
#[derive(Debug, Clone, PartialEq)]
pub struct CraneliftFunction {
    /// Function name
    pub name: String,
    /// Function signature
    pub sig: Signature,
    /// Stack slots: (slot_id, size_bytes)
    pub stack_slots: Vec<(u32, u32)>,
    /// Global values referenced: (gv_id, name)
    pub global_values: Vec<(u32, String)>,
    /// Signature references for indirect calls: (sig_id, signature)
    pub sig_refs: Vec<(u32, Signature)>,
    /// Function references for direct calls: (fn_ref_id, name, sig_id)
    pub func_refs: Vec<(u32, String, u32)>,
    /// Basic blocks (first block is the entry block)
    pub blocks: Vec<CraneliftBlock>,
    /// Next available SSA value id
    pub(super) next_value: u32,
    /// Next available block id
    pub(super) next_block: u32,
}
impl CraneliftFunction {
    /// Create a new function.
    pub fn new(name: impl Into<String>, sig: Signature) -> Self {
        CraneliftFunction {
            name: name.into(),
            sig,
            stack_slots: vec![],
            global_values: vec![],
            sig_refs: vec![],
            func_refs: vec![],
            blocks: vec![],
            next_value: 0,
            next_block: 0,
        }
    }
    /// Allocate a fresh SSA value with the given type.
    pub fn fresh_value(&mut self, ty: CraneliftType) -> CraneliftValue {
        let v = CraneliftValue::new(self.next_value, ty);
        self.next_value += 1;
        v
    }
    /// Create a new block and return its block reference.
    pub fn new_block(&mut self) -> u32 {
        let id = self.next_block;
        self.next_block += 1;
        self.blocks.push(CraneliftBlock::new(id));
        id
    }
    /// Create a new block with parameters.
    pub fn new_block_with_params(&mut self, param_types: &[CraneliftType]) -> u32 {
        let id = self.next_block;
        self.next_block += 1;
        let params = param_types
            .iter()
            .map(|ty| {
                let v = CraneliftValue::new(self.next_value, ty.clone());
                self.next_value += 1;
                v
            })
            .collect();
        self.blocks.push(CraneliftBlock::with_params(id, params));
        id
    }
    /// Get a mutable reference to a block by id.
    pub fn block_mut(&mut self, id: u32) -> Option<&mut CraneliftBlock> {
        self.blocks.iter_mut().find(|b| b.id == id)
    }
    /// Add a stack slot.
    pub fn add_stack_slot(&mut self, size_bytes: u32) -> u32 {
        let id = self.stack_slots.len() as u32;
        self.stack_slots.push((id, size_bytes));
        id
    }
    /// Emit this function as textual IR.
    pub fn emit(&self) -> String {
        let mut s = String::new();
        s.push_str(&format!("function %{}(", self.name));
        for (i, ty) in self.sig.params.iter().enumerate() {
            if i > 0 {
                s.push_str(", ");
            }
            s.push_str(&ty.to_string());
        }
        s.push_str(") -> ");
        for (i, ty) in self.sig.returns.iter().enumerate() {
            if i > 0 {
                s.push_str(", ");
            }
            s.push_str(&ty.to_string());
        }
        s.push_str(" system_v {\n");
        for (id, size) in &self.stack_slots {
            s.push_str(&format!("    ss{} = explicit_slot {}\n", id, size));
        }
        for (id, name) in &self.global_values {
            s.push_str(&format!("    gv{} = symbol colocated %{}\n", id, name));
        }
        for (id, sig) in &self.sig_refs {
            s.push_str(&format!("    sig{} = {}\n", id, sig));
        }
        for (id, name, sig_id) in &self.func_refs {
            s.push_str(&format!(
                "    fn{} = colocated %{} sig{}\n",
                id, name, sig_id
            ));
        }
        if !self.stack_slots.is_empty()
            || !self.global_values.is_empty()
            || !self.func_refs.is_empty()
        {
            s.push('\n');
        }
        for block in &self.blocks {
            s.push_str(&block.emit());
        }
        s.push_str("}\n");
        s
    }
}
/// A Cranelift module — the top-level compilation unit.
#[derive(Debug, Clone)]
pub struct CraneliftModule {
    /// Module name (for display purposes)
    pub name: String,
    /// Target triple (e.g. "x86_64-unknown-linux-gnu")
    pub target: String,
    /// Function definitions
    pub functions: Vec<CraneliftFunction>,
    /// Function declarations (external references)
    pub func_decls: Vec<(String, Signature)>,
    /// Data objects
    pub data_objects: Vec<CraneliftDataObject>,
    /// Global value map: name → gv_id
    pub global_values: HashMap<String, u32>,
}
impl CraneliftModule {
    /// Create a new module.
    pub fn new(name: impl Into<String>) -> Self {
        CraneliftModule {
            name: name.into(),
            target: "x86_64-unknown-linux-gnu".to_string(),
            functions: vec![],
            func_decls: vec![],
            data_objects: vec![],
            global_values: HashMap::new(),
        }
    }
    /// Add a function definition.
    pub fn add_function(&mut self, func: CraneliftFunction) {
        self.functions.push(func);
    }
    /// Add a function declaration (external).
    pub fn add_func_decl(&mut self, name: impl Into<String>, sig: Signature) {
        self.func_decls.push((name.into(), sig));
    }
    /// Add a data object.
    pub fn add_data_object(&mut self, obj: CraneliftDataObject) {
        self.data_objects.push(obj);
    }
    /// Emit the full module as textual IR.
    pub fn emit(&self) -> String {
        let mut s = String::new();
        s.push_str(&format!("; target = \"{}\"\n\n", self.target));
        for (name, sig) in &self.func_decls {
            s.push_str(&format!("declare %{}{};\n", name, sig));
        }
        if !self.func_decls.is_empty() {
            s.push('\n');
        }
        for obj in &self.data_objects {
            s.push_str(&obj.emit());
            s.push('\n');
        }
        for func in &self.functions {
            s.push_str(&func.emit());
            s.push('\n');
        }
        s
    }
}
