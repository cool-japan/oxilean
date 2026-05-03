//! Type definitions for wasm_runtime types

use super::super::super::functions::{WASM_MAGIC, WASM_VERSION};
use std::collections::HashMap;

/// A WebAssembly value type (for function type signatures).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WasmType {
    I32,
    I64,
    F32,
    F64,
    V128,
    FuncRef,
    ExternRef,
}

/// A WASM import descriptor.
#[derive(Debug, Clone)]
pub struct WasmImport {
    pub module: String,
    pub name: String,
    pub kind: WasmExternKind,
    pub index: u32,
}

pub struct WasiEnvironment {
    pub args: Vec<String>,
    pub env_vars: HashMap<String, String>,
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
    pub exit_code: Option<WasiExitCode>,
}

/// Linear memory for a WASM module.
pub struct WasmMemory {
    pub data: Vec<u8>,
    pub page_count: usize,
}

pub struct WasmModuleLoader {
    pub sections: Vec<WasmSectionHeader>,
    pub errors: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct GcArrayType {
    pub name: String,
    pub element_ty: WasmType,
    pub mutable: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExecError {
    StackUnderflow,
    TypeMismatch { expected: String, got: String },
    DivisionByZero,
    Unreachable,
    OutOfBoundsMemory(usize),
    UndefinedLocal(u32),
    UndefinedGlobal(u32),
    CallStackOverflow,
    Custom(String),
}

/// The kind of a structured control-flow label frame.
#[derive(Debug, Clone)]
pub enum LabelKind {
    Block,
    Loop,
    IfThen { else_pc: Option<usize> },
}

/// A structured control-flow frame pushed onto the label stack.
#[derive(Debug, Clone)]
pub struct LabelFrame {
    pub kind: LabelKind,
    /// Number of result values this block leaves on the stack.
    pub arity: u32,
    /// For Block/If: index past the matching End.
    /// For Loop: index of the Loop instruction itself (branch target for back-edge).
    pub branch_target: usize,
    /// Value-stack depth at block entry (used to restore on branch).
    pub stack_base: usize,
}

/// A call-frame pushed onto the call stack when entering a callee.
#[derive(Debug, Clone)]
pub struct CallFrame {
    /// Saved locals of the caller.
    pub saved_locals: Vec<WasmValue>,
    /// PC in the caller's body to return to after this call.
    pub return_pc: usize,
    /// label_stack length at the call site (so End only pops this frame's labels).
    pub label_base: usize,
    /// value-stack depth before the call arguments were pushed.
    pub value_base: usize,
    /// Name of the function being called (for error messages).
    pub func_name: String,
}

pub struct StackMachine {
    pub stack: Vec<WasmValue>,
    pub locals: Vec<WasmValue>,
    pub globals: Vec<WasmValue>,
    pub memory: WasmMemory,
    pub max_stack_depth: usize,
    pub instruction_count: u64,
    pub label_stack: Vec<LabelFrame>,
    pub call_stack: Vec<CallFrame>,
}

#[derive(Debug, Clone)]
pub struct WasiFdWriteResult {
    pub bytes_written: usize,
    pub errno: i32,
}

/// A WASM export descriptor.
#[derive(Debug, Clone)]
pub struct WasmExport {
    pub name: String,
    pub kind: WasmExternKind,
    pub index: u32,
}

#[derive(Debug, Clone)]
pub struct WasmSectionHeader {
    pub id: WasmSectionId,
    pub size: u32,
    pub offset: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WasmExternKind {
    Function,
    Table,
    Memory,
    Global,
}

#[derive(Debug, Clone)]
pub struct WasmFunction {
    pub name: String,
    pub type_idx: u32,
    pub locals: Vec<WasmType>,
    pub body: Vec<WasmInstruction>,
}

#[derive(Debug, Clone)]
pub struct GcStructField {
    pub name: String,
    pub ty: WasmType,
    pub mutable: bool,
}

pub struct GcTypeRegistry {
    pub structs: Vec<GcStructType>,
    pub arrays: Vec<GcArrayType>,
}

/// A WebAssembly function type (signature): parameter types and result types.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WasmFuncType {
    pub params: Vec<WasmType>,
    pub results: Vec<WasmType>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StreamingState {
    AwaitingHeader,
    ReadingSections,
    Compiling,
    Done,
    Error(String),
}

/// A WebAssembly instruction.
#[derive(Debug, Clone, PartialEq)]
pub enum WasmInstruction {
    Unreachable,
    Nop,
    Block { ty: Option<WasmType> },
    Loop { ty: Option<WasmType> },
    If { ty: Option<WasmType> },
    Else,
    End,
    Br(u32),
    BrIf(u32),
    BrTable { targets: Vec<u32>, default: u32 },
    Return,
    Call(u32),
    CallIndirect { type_idx: u32, table_idx: u32 },
    Drop,
    Select,
    LocalGet(u32),
    LocalSet(u32),
    LocalTee(u32),
    GlobalGet(u32),
    GlobalSet(u32),
    I32Load { align: u32, offset: u32 },
    I64Load { align: u32, offset: u32 },
    F32Load { align: u32, offset: u32 },
    F64Load { align: u32, offset: u32 },
    I32Store { align: u32, offset: u32 },
    I64Store { align: u32, offset: u32 },
    F32Store { align: u32, offset: u32 },
    F64Store { align: u32, offset: u32 },
    MemorySize,
    MemoryGrow,
    I32Const(i32),
    I64Const(i64),
    F32Const(f32),
    F64Const(f64),
    I32Eqz,
    I32Eq,
    I32Ne,
    I32LtS,
    I32LtU,
    I32GtS,
    I32GtU,
    I32LeS,
    I32LeU,
    I32GeS,
    I32GeU,
    I32Clz,
    I32Ctz,
    I32Popcnt,
    I32Add,
    I32Sub,
    I32Mul,
    I32DivS,
    I32DivU,
    I32RemS,
    I32RemU,
    I32And,
    I32Or,
    I32Xor,
    I32Shl,
    I32ShrS,
    I32ShrU,
    I32Rotl,
    I32Rotr,
    I64Eqz,
    I64Eq,
    I64Ne,
    I64LtS,
    I64LtU,
    I64GtS,
    I64GtU,
    I64LeS,
    I64LeU,
    I64GeS,
    I64GeU,
    I64Clz,
    I64Ctz,
    I64Popcnt,
    I64Add,
    I64Sub,
    I64Mul,
    I64DivS,
    I64DivU,
    I64RemS,
    I64RemU,
    I64And,
    I64Or,
    I64Xor,
    I64Shl,
    I64ShrS,
    I64ShrU,
    I64Rotl,
    I64Rotr,
    F32Eq,
    F32Ne,
    F32Lt,
    F32Gt,
    F32Le,
    F32Ge,
    F32Abs,
    F32Neg,
    F32Ceil,
    F32Floor,
    F32Trunc,
    F32Nearest,
    F32Sqrt,
    F32Add,
    F32Sub,
    F32Mul,
    F32Div,
    F32Min,
    F32Max,
    F32Copysign,
    F64Eq,
    F64Ne,
    F64Lt,
    F64Gt,
    F64Le,
    F64Ge,
    F64Abs,
    F64Neg,
    F64Ceil,
    F64Floor,
    F64Trunc,
    F64Nearest,
    F64Sqrt,
    F64Add,
    F64Sub,
    F64Mul,
    F64Div,
    F64Min,
    F64Max,
    F64Copysign,
    I32WrapI64,
    I32TruncF32S,
    I32TruncF32U,
    I32TruncF64S,
    I32TruncF64U,
    I64ExtendI32S,
    I64ExtendI32U,
    I64TruncF32S,
    I64TruncF32U,
    I64TruncF64S,
    I64TruncF64U,
    F32ConvertI32S,
    F32ConvertI32U,
    F32ConvertI64S,
    F32ConvertI64U,
    F32DemoteF64,
    F64ConvertI32S,
    F64ConvertI32U,
    F64ConvertI64S,
    F64ConvertI64U,
    F64PromoteF32,
    I32ReinterpretF32,
    I64ReinterpretF64,
    F32ReinterpretI32,
    F64ReinterpretI64,
}

#[derive(Debug, Clone)]
pub struct GcStructType {
    pub name: String,
    pub fields: Vec<GcStructField>,
}

#[derive(Debug, Clone)]
pub struct StackMachineStats {
    pub instruction_count: u64,
    pub stack_depth: usize,
    pub local_count: usize,
    pub global_count: usize,
    pub memory_pages: usize,
}

/// A WebAssembly global variable.
#[derive(Debug, Clone)]
pub struct WasmGlobal {
    pub value: WasmValue,
    pub mutable: bool,
    pub name: String,
}

/// Function table for indirect calls.
pub struct WasmTable {
    pub elements: Vec<Option<String>>,
    pub max_size: Option<usize>,
}

/// Runtime that hosts multiple WASM modules.
pub struct WasmRuntime {
    pub modules: HashMap<String, WasmModule>,
}

pub struct StreamingCompiler {
    pub buffer: Vec<u8>,
    pub state: StreamingState,
    pub bytes_consumed: usize,
    pub module_name: String,
}

/// A loaded WASM module.
pub struct WasmModule {
    pub name: String,
    pub memory: WasmMemory,
    pub table: WasmTable,
    pub exports: HashMap<String, String>,
    pub functions: HashMap<String, WasmFunction>,
}

/// A WebAssembly value.
#[derive(Debug, Clone, PartialEq)]
pub enum WasmValue {
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
    V128([u8; 16]),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WasmSectionId {
    Custom = 0,
    Type = 1,
    Import = 2,
    Function = 3,
    Table = 4,
    Memory = 5,
    Global = 6,
    Export = 7,
    Start = 8,
    Element = 9,
    Code = 10,
    Data = 11,
    DataCount = 12,
}

#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub errors: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WasiExitCode(pub i32);

pub struct WasmValidator;
