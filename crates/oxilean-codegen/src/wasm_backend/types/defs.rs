//! Type definitions

use super::super::functions::*;
use crate::lcnf::*;
use std::collections::{HashMap, HashSet, VecDeque};

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct WasmCacheEntry {
    pub key: String,
    pub data: Vec<u8>,
    pub timestamp: u64,
    pub valid: bool,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct WasmDepGraph {
    pub(super) nodes: Vec<u32>,
    pub(super) edges: Vec<(u32, u32)>,
}
/// Pass registry for WasmX2.
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct WasmX2PassRegistry {
    pub(super) configs: Vec<WasmX2PassConfig>,
    pub(super) stats: Vec<WasmX2PassStats>,
}
/// Dominator tree for WasmExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct WasmExtDomTree {
    pub(super) idom: Vec<Option<usize>>,
    pub(super) children: Vec<Vec<usize>>,
    pub(super) depth: Vec<usize>,
}
/// A complete WebAssembly module.
#[derive(Debug, Clone, Default)]
pub struct WasmModule {
    /// Import declarations.
    pub imports: Vec<WasmImport>,
    /// Function definitions.
    pub functions: Vec<WasmFunc>,
    /// Names of functions to export.
    pub exports: Vec<String>,
    /// Optional memory size in pages (64 KiB each). `None` = no memory.
    pub memory: Option<u32>,
    /// Global variable declarations.
    pub globals: Vec<WasmGlobal>,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct WasmPassConfig {
    pub phase: WasmPassPhase,
    pub enabled: bool,
    pub max_iterations: u32,
    pub debug_output: bool,
    pub pass_name: String,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct WasmWorklist {
    pub(super) items: std::collections::VecDeque<u32>,
    pub(super) in_worklist: std::collections::HashSet<u32>,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct WasmDominatorTree {
    pub idom: Vec<Option<u32>>,
    pub dom_children: Vec<Vec<u32>>,
    pub dom_depth: Vec<u32>,
}
/// Dependency graph for WasmExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct WasmExtDepGraph {
    pub(super) n: usize,
    pub(super) adj: Vec<Vec<usize>>,
    pub(super) rev: Vec<Vec<usize>>,
    pub(super) edge_count: usize,
}
/// Statistics for WasmX2 passes.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct WasmX2PassStats {
    pub iterations: usize,
    pub changed: bool,
    pub nodes_visited: usize,
    pub nodes_modified: usize,
    pub time_ms: u64,
    pub memory_bytes: usize,
    pub errors: usize,
}
/// Worklist for WasmExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct WasmExtWorklist {
    pub(super) items: std::collections::VecDeque<usize>,
    pub(super) present: Vec<bool>,
}
/// Analysis cache for WasmExt.
#[allow(dead_code)]
#[derive(Debug)]
pub struct WasmExtCache {
    pub(super) entries: Vec<(u64, Vec<u8>, bool, u32)>,
    pub(super) cap: usize,
    pub(super) total_hits: u64,
    pub(super) total_misses: u64,
}
/// Dominator tree for WasmX2.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct WasmX2DomTree {
    pub(super) idom: Vec<Option<usize>>,
    pub(super) children: Vec<Vec<usize>>,
    pub(super) depth: Vec<usize>,
}
/// The kind of WebAssembly import.
#[derive(Debug, Clone, PartialEq)]
pub enum WasmImportKind {
    /// Imported function with parameter and result types.
    Func {
        params: Vec<WasmType>,
        results: Vec<WasmType>,
    },
    /// Imported memory with minimum pages.
    Memory { min_pages: u32 },
    /// Imported global with type and mutability.
    Global { ty: WasmType, mutable: bool },
}
/// Dependency graph for WasmX2.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct WasmX2DepGraph {
    pub(super) n: usize,
    pub(super) adj: Vec<Vec<usize>>,
    pub(super) rev: Vec<Vec<usize>>,
    pub(super) edge_count: usize,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct WasmLivenessInfo {
    pub live_in: Vec<std::collections::HashSet<u32>>,
    pub live_out: Vec<std::collections::HashSet<u32>>,
    pub defs: Vec<std::collections::HashSet<u32>>,
    pub uses: Vec<std::collections::HashSet<u32>>,
}
/// Pass execution phase for WasmX2.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum WasmX2PassPhase {
    Early,
    Middle,
    Late,
    Finalize,
}
/// Configuration for WasmX2 passes.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct WasmX2PassConfig {
    pub name: String,
    pub phase: WasmX2PassPhase,
    pub enabled: bool,
    pub max_iterations: usize,
    pub debug: u32,
    pub timeout_ms: Option<u64>,
}
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct WasmPassStats {
    pub total_runs: u32,
    pub successful_runs: u32,
    pub total_changes: u64,
    pub time_ms: u64,
    pub iterations_used: u32,
}
/// A WebAssembly global variable declaration.
#[derive(Debug, Clone, PartialEq)]
pub struct WasmGlobal {
    /// Identifier name of the global.
    pub name: String,
    /// Value type of the global.
    pub ty: WasmType,
    /// Whether the global is mutable.
    pub mutable: bool,
    /// Initial value expression (as a WAT constant expression string).
    pub init_value: String,
}
/// Configuration for WasmExt passes.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct WasmExtPassConfig {
    pub name: String,
    pub phase: WasmExtPassPhase,
    pub enabled: bool,
    pub max_iterations: usize,
    pub debug: u32,
    pub timeout_ms: Option<u64>,
}
/// Statistics for WasmExt passes.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct WasmExtPassStats {
    pub iterations: usize,
    pub changed: bool,
    pub nodes_visited: usize,
    pub nodes_modified: usize,
    pub time_ms: u64,
    pub memory_bytes: usize,
    pub errors: usize,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct WasmAnalysisCache {
    pub(crate) entries: std::collections::HashMap<String, WasmCacheEntry>,
    pub(super) max_size: usize,
    pub(super) hits: u64,
    pub(super) misses: u64,
}
/// Pass registry for WasmExt.
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct WasmExtPassRegistry {
    pub(super) configs: Vec<WasmExtPassConfig>,
    pub(super) stats: Vec<WasmExtPassStats>,
}
/// WebAssembly value types.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum WasmType {
    /// 32-bit integer
    I32,
    /// 64-bit integer
    I64,
    /// 32-bit float
    F32,
    /// 64-bit float
    F64,
    /// Function reference
    FuncRef,
    /// External reference
    ExternRef,
    /// 128-bit SIMD vector
    V128,
}
#[allow(dead_code)]
pub struct WasmPassRegistry {
    pub(super) configs: Vec<WasmPassConfig>,
    pub(super) stats: std::collections::HashMap<String, WasmPassStats>,
}
/// Worklist for WasmX2.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct WasmX2Worklist {
    pub(super) items: std::collections::VecDeque<usize>,
    pub(super) present: Vec<bool>,
}
/// Constant folding helper for WasmX2.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct WasmX2ConstFolder {
    pub(super) folds: usize,
    pub(super) failures: usize,
    pub(super) enabled: bool,
}
/// The WebAssembly code generation backend.
///
/// Compiles LCNF function declarations to WebAssembly text format (WAT).
#[derive(Debug, Default)]
pub struct WasmBackend {
    /// Counter for generating unique local variable names.
    pub(super) local_counter: u64,
}
/// Analysis cache for WasmX2.
#[allow(dead_code)]
#[derive(Debug)]
pub struct WasmX2Cache {
    pub(super) entries: Vec<(u64, Vec<u8>, bool, u32)>,
    pub(super) cap: usize,
    pub(super) total_hits: u64,
    pub(super) total_misses: u64,
}
/// Pass execution phase for WasmExt.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum WasmExtPassPhase {
    Early,
    Middle,
    Late,
    Finalize,
}
/// WebAssembly instructions for the WAT text format.
#[derive(Debug, Clone, PartialEq)]
pub enum WasmInstr {
    /// Get local variable: `local.get $name`
    LocalGet(String),
    /// Set local variable: `local.set $name`
    LocalSet(String),
    /// 32-bit integer constant: `i32.const N`
    I32Const(i32),
    /// 64-bit integer constant: `i64.const N`
    I64Const(i64),
    /// 64-bit float constant: `f64.const N`
    F64Const(f64),
    /// Direct function call: `call $name`
    Call(String),
    /// Indirect function call via table: `call_indirect`
    CallIndirect,
    /// Return from function: `return`
    Return,
    /// Branch if condition is true: `br_if N`
    BrIf(u32),
    /// Begin a block: `block`
    Block,
    /// Begin a loop: `loop`
    Loop,
    /// End a block/loop/function: `end`
    End,
    /// Drop top of stack: `drop`
    Drop,
    /// Select one of two values based on condition: `select`
    Select,
    /// No-op: `nop`
    Nop,
    /// Trap unconditionally: `unreachable`
    Unreachable,
    /// `i32.add`
    I32Add,
    /// `i32.sub`
    I32Sub,
    /// `i32.mul`
    I32Mul,
    /// `i32.div_s`
    I32DivS,
    /// `i64.add`
    I64Add,
    /// `i64.mul`
    I64Mul,
    /// `f64.add`
    F64Add,
    /// `f64.mul`
    F64Mul,
    /// `f64.div`
    F64Div,
    /// `f64.sqrt`
    F64Sqrt,
    /// Load from linear memory with alignment: `i32.load align=N`
    MemLoad(u32),
    /// Store to linear memory with alignment: `i32.store align=N`
    MemStore(u32),
    /// `i32.eqz`
    I32Eqz,
    /// `i32.eq`
    I32Eq,
    /// `i32.ne`
    I32Ne,
    /// `i32.lt_s`
    I32LtS,
    /// `i32.gt_s`
    I32GtS,
    /// `i32.le_s`
    I32LeS,
    /// `i32.ge_s`
    I32GeS,
    /// Null reference: `ref.null funcref`
    RefNull,
    /// Test if reference is null: `ref.is_null`
    RefIsNull,
    /// Get element from table: `table.get N`
    TableGet(u32),
    /// Set element in table: `table.set N`
    TableSet(u32),
}
/// Constant folding helper for WasmExt.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct WasmExtConstFolder {
    pub(super) folds: usize,
    pub(super) failures: usize,
    pub(super) enabled: bool,
}
/// A WebAssembly function definition.
#[derive(Debug, Clone, PartialEq)]
pub struct WasmFunc {
    /// Function name (used as `$name` in WAT).
    pub name: String,
    /// Named parameters with their types: `(param $name type)`.
    pub params: Vec<(String, WasmType)>,
    /// Result types: `(result type)`.
    pub results: Vec<WasmType>,
    /// Named local variables: `(local $name type)`.
    pub locals: Vec<(String, WasmType)>,
    /// Function body instructions.
    pub body: Vec<WasmInstr>,
}
/// Liveness analysis for WasmExt.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct WasmExtLiveness {
    pub live_in: Vec<Vec<usize>>,
    pub live_out: Vec<Vec<usize>>,
    pub defs: Vec<Vec<usize>>,
    pub uses: Vec<Vec<usize>>,
}
/// A WebAssembly import declaration.
#[derive(Debug, Clone, PartialEq)]
pub struct WasmImport {
    /// The external module name (e.g. `"env"`)
    pub module: String,
    /// The name of the imported item (e.g. `"memory"`)
    pub name: String,
    /// The kind of the import (func/memory/global)
    pub kind: WasmImportKind,
}
#[allow(dead_code)]
pub struct WasmConstantFoldingHelper;
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum WasmPassPhase {
    Analysis,
    Transformation,
    Verification,
    Cleanup,
}
/// Liveness analysis for WasmX2.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct WasmX2Liveness {
    pub live_in: Vec<Vec<usize>>,
    pub live_out: Vec<Vec<usize>>,
    pub defs: Vec<Vec<usize>>,
    pub uses: Vec<Vec<usize>>,
}
