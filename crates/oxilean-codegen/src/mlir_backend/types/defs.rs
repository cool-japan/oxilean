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

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum MLIRPassPhase {
    Analysis,
    Transformation,
    Verification,
    Cleanup,
}

/// MLIR code generation backend.
pub struct MlirBackend {
    pub(crate) module: MlirModule,
    pub(crate) ssa: SsaCounter,
    pub(crate) pass_pipeline: Vec<String>,
}

/// Constant folding helper for MLIRExt.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct MLIRExtConstFolder {
    pub(crate) folds: usize,
    pub(crate) failures: usize,
    pub(crate) enabled: bool,
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

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct MLIRWorklist {
    pub(crate) items: std::collections::VecDeque<u32>,
    pub(crate) in_worklist: std::collections::HashSet<u32>,
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

/// Analysis cache for MLIRExt.
#[allow(dead_code)]
#[derive(Debug)]
pub struct MLIRExtCache {
    pub(crate) entries: Vec<(u64, Vec<u8>, bool, u32)>,
    pub(crate) cap: usize,
    pub(crate) total_hits: u64,
    pub(crate) total_misses: u64,
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

/// Counter for generating fresh SSA value names.
#[derive(Debug, Default)]
pub struct SsaCounter {
    pub(crate) counter: u32,
    pub(crate) named: HashMap<String, u32>,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct MLIRDominatorTree {
    pub idom: Vec<Option<u32>>,
    pub dom_children: Vec<Vec<u32>>,
    pub dom_depth: Vec<u32>,
}

/// Builder for constructing MLIR operations conveniently.
pub struct MlirBuilder {
    pub(crate) ssa: SsaCounter,
    pub(crate) ops: Vec<MlirOp>,
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

/// An MLIR region (contains a list of basic blocks).
#[derive(Debug, Clone)]
pub struct MlirRegion {
    /// Blocks in this region
    pub blocks: Vec<MlirBlock>,
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

/// Worklist for MLIRExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct MLIRExtWorklist {
    pub(crate) items: std::collections::VecDeque<usize>,
    pub(crate) present: Vec<bool>,
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

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct MLIRAnalysisCache {
    pub(crate) entries: std::collections::HashMap<String, MLIRCacheEntry>,
    pub(crate) max_size: usize,
    pub(crate) hits: u64,
    pub(crate) misses: u64,
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

/// Dependency graph for MLIRExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct MLIRExtDepGraph {
    pub(crate) n: usize,
    pub(crate) adj: Vec<Vec<usize>>,
    pub(crate) rev: Vec<Vec<usize>>,
    pub(crate) edge_count: usize,
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
    pub(crate) idom: Vec<Option<usize>>,
    pub(crate) children: Vec<Vec<usize>>,
    pub(crate) depth: Vec<usize>,
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

/// Pass registry for MLIRExt.
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct MLIRExtPassRegistry {
    pub(crate) configs: Vec<MLIRExtPassConfig>,
    pub(crate) stats: Vec<MLIRExtPassStats>,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct MLIRDepGraph {
    pub(crate) nodes: Vec<u32>,
    pub(crate) edges: Vec<(u32, u32)>,
}

#[allow(dead_code)]
pub struct MLIRPassRegistry {
    pub(crate) configs: Vec<MLIRPassConfig>,
    pub(crate) stats: std::collections::HashMap<String, MLIRPassStats>,
}

#[allow(dead_code)]
pub struct MLIRConstantFoldingHelper;
