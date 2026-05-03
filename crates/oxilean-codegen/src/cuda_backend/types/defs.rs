use std::collections::{HashMap, HashSet, VecDeque};

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CUDAWorklist {
    pub(crate) items: std::collections::VecDeque<u32>,
    pub(crate) in_worklist: std::collections::HashSet<u32>,
}

/// A CUDA kernel (`__global__` function).
#[derive(Debug, Clone, PartialEq)]
pub struct CudaKernel {
    /// Kernel name
    pub name: String,
    /// Parameter list
    pub params: Vec<CudaParam>,
    /// Shared memory declarations (emitted at the top of the kernel body)
    pub shared_mem_decls: Vec<SharedMemDecl>,
    /// Kernel body statements
    pub body: Vec<CudaStmt>,
    /// Optional `__launch_bounds__` annotation
    pub launch_bounds: Option<LaunchBounds>,
}

/// CUDA kernel launch configuration.
#[derive(Debug, Clone, PartialEq)]
pub struct LaunchConfig {
    /// Grid dimensions (number of blocks)
    pub grid: CudaExpr,
    /// Block dimensions (threads per block)
    pub block: CudaExpr,
    /// Dynamic shared memory bytes (0 if none)
    pub shared_mem: CudaExpr,
    /// CUDA stream (None → default stream)
    pub stream: Option<CudaExpr>,
}

/// Top-level CUDA module representing a single `.cu` file.
#[derive(Debug, Clone, PartialEq)]
pub struct CudaModule {
    /// `#include` directives (just the header names, e.g. `"cuda_runtime.h"`)
    pub includes: Vec<String>,
    /// `__constant__` memory declarations at file scope
    pub constant_decls: Vec<(CudaType, String, Option<CudaExpr>)>,
    /// `__device__` (or `__host__ __device__`) helper functions
    pub device_functions: Vec<DeviceFunction>,
    /// `__global__` kernels
    pub kernels: Vec<CudaKernel>,
    /// Host-side code (helper functions, `main`, etc.) as raw strings
    pub host_code: Vec<String>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct CUDAPassStats {
    pub total_runs: u32,
    pub successful_runs: u32,
    pub total_changes: u64,
    pub time_ms: u64,
    pub iterations_used: u32,
}

/// Worklist for CUDAExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CUDAExtWorklist {
    pub(crate) items: std::collections::VecDeque<usize>,
    pub(crate) present: Vec<bool>,
}

/// Emitter state for producing CUDA `.cu` source code.
pub struct CudaBackend {
    pub(crate) indent_width: usize,
}

/// A `__shared__` memory declaration inside a kernel.
#[derive(Debug, Clone, PartialEq)]
pub struct SharedMemDecl {
    /// Element type
    pub ty: CudaType,
    /// Variable name
    pub name: String,
    /// Array size (None for dynamic shared memory)
    pub size: Option<CudaExpr>,
}

/// Unary prefix operators.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CudaUnOp {
    Neg,
    Not,
    BitNot,
    Deref,
    AddrOf,
}

/// CUDA C++ expression AST node.
#[derive(Debug, Clone, PartialEq)]
pub enum CudaExpr {
    /// Integer literal: `42`
    LitInt(i64),
    /// Float literal: `3.14f`
    LitFloat(f64),
    /// Boolean literal: `true` / `false`
    LitBool(bool),
    /// Named variable or parameter: `x`
    Var(String),
    /// `threadIdx.x`, `threadIdx.y`, `threadIdx.z`
    ThreadIdx(char),
    /// `blockIdx.x`, `blockIdx.y`, `blockIdx.z`
    BlockIdx(char),
    /// `blockDim.x`, `blockDim.y`, `blockDim.z`
    BlockDim(char),
    /// `gridDim.x`, `gridDim.y`, `gridDim.z`
    GridDim(char),
    /// `__syncthreads()`
    SyncThreads,
    /// `atomicAdd(addr, val)` — atomic addition
    AtomicAdd(Box<CudaExpr>, Box<CudaExpr>),
    /// `atomicSub(addr, val)`
    AtomicSub(Box<CudaExpr>, Box<CudaExpr>),
    /// `atomicExch(addr, val)`
    AtomicExch(Box<CudaExpr>, Box<CudaExpr>),
    /// `atomicCAS(addr, compare, val)`
    AtomicCas(Box<CudaExpr>, Box<CudaExpr>, Box<CudaExpr>),
    /// `atomicMax(addr, val)`
    AtomicMax(Box<CudaExpr>, Box<CudaExpr>),
    /// `atomicMin(addr, val)`
    AtomicMin(Box<CudaExpr>, Box<CudaExpr>),
    /// Binary operation: `a + b`
    BinOp(Box<CudaExpr>, CudaBinOp, Box<CudaExpr>),
    /// Unary operation: `!a`
    UnOp(CudaUnOp, Box<CudaExpr>),
    /// Array subscript: `arr[idx]`
    Index(Box<CudaExpr>, Box<CudaExpr>),
    /// Struct member access: `s.field`
    Member(Box<CudaExpr>, String),
    /// Pointer member access: `p->field`
    PtrMember(Box<CudaExpr>, String),
    /// C-style cast: `(T)expr`
    Cast(CudaType, Box<CudaExpr>),
    /// Function call: `func(args...)`
    Call(String, Vec<CudaExpr>),
    /// Ternary conditional: `cond ? then : else`
    Ternary(Box<CudaExpr>, Box<CudaExpr>, Box<CudaExpr>),
    /// `__ldg(&x)` — read-only cache load
    Ldg(Box<CudaExpr>),
    /// `__shfl_down_sync(mask, var, delta)`
    ShflDownSync(Box<CudaExpr>, Box<CudaExpr>, Box<CudaExpr>),
    /// `__shfl_xor_sync(mask, var, laneMask)`
    ShflXorSync(Box<CudaExpr>, Box<CudaExpr>, Box<CudaExpr>),
    /// `warpSize` builtin
    WarpSize,
    /// `__ballot_sync(mask, predicate)`
    BallotSync(Box<CudaExpr>, Box<CudaExpr>),
    /// `__popc(x)` — popcount
    Popc(Box<CudaExpr>),
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CUDALivenessInfo {
    pub live_in: Vec<std::collections::HashSet<u32>>,
    pub live_out: Vec<std::collections::HashSet<u32>>,
    pub defs: Vec<std::collections::HashSet<u32>>,
    pub uses: Vec<std::collections::HashSet<u32>>,
}

/// CUDA statement AST node.
#[derive(Debug, Clone, PartialEq)]
pub enum CudaStmt {
    /// Variable declaration with optional initializer:
    /// `CudaType name [ = init ];`
    VarDecl {
        ty: CudaType,
        name: String,
        init: Option<CudaExpr>,
    },
    /// Simple assignment: `lhs = rhs;`
    Assign { lhs: CudaExpr, rhs: CudaExpr },
    /// Compound assignment: `lhs += rhs;` etc.
    CompoundAssign {
        lhs: CudaExpr,
        op: CudaBinOp,
        rhs: CudaExpr,
    },
    /// If / optional else:
    IfElse {
        cond: CudaExpr,
        then_body: Vec<CudaStmt>,
        else_body: Option<Vec<CudaStmt>>,
    },
    /// C-style for loop:
    /// `for (init; cond; step) { body }`
    ForLoop {
        init: Box<CudaStmt>,
        cond: CudaExpr,
        step: CudaExpr,
        body: Vec<CudaStmt>,
    },
    /// While loop: `while (cond) { body }`
    WhileLoop { cond: CudaExpr, body: Vec<CudaStmt> },
    /// CUDA kernel launch: `name<<<grid, block, shmem, stream>>>(args...);`
    KernelLaunch {
        name: String,
        config: LaunchConfig,
        args: Vec<CudaExpr>,
    },
    /// `cudaMalloc((void**)&ptr, size);`
    CudaMalloc { ptr: String, size: CudaExpr },
    /// `cudaMemcpy(dst, src, size, kind);`
    CudaMemcpy {
        dst: CudaExpr,
        src: CudaExpr,
        size: CudaExpr,
        kind: MemcpyKind,
    },
    /// `cudaFree(ptr);`
    CudaFree(CudaExpr),
    /// `return expr;`
    Return(Option<CudaExpr>),
    /// Raw expression statement: `expr;`
    Expr(CudaExpr),
    /// `cudaDeviceSynchronize();`
    DeviceSync,
    /// `cudaCheckError()` macro invocation
    CheckError(CudaExpr),
    /// Block of statements grouped with `{}`
    Block(Vec<CudaStmt>),
    /// `break;`
    Break,
    /// `continue;`
    Continue,
}

/// A parameter in a CUDA kernel or device function.
#[derive(Debug, Clone, PartialEq)]
pub struct CudaParam {
    /// CUDA type
    pub ty: CudaType,
    /// Parameter name
    pub name: String,
    /// Whether the parameter is `const`
    pub is_const: bool,
    /// Optional qualifier such as `__restrict__`
    pub qualifier: Option<CudaQualifier>,
}

/// A `__device__` (or `__host__ __device__`) helper function.
#[derive(Debug, Clone, PartialEq)]
pub struct DeviceFunction {
    /// Function name
    pub name: String,
    /// Qualifiers (should include at least `Device`)
    pub qualifiers: Vec<CudaQualifier>,
    /// Return type
    pub ret: CudaType,
    /// Parameter list
    pub params: Vec<CudaParam>,
    /// Body statements
    pub body: Vec<CudaStmt>,
    /// Whether the function is `inline`
    pub is_inline: bool,
}

/// CUDA / C++ type representation used in generated `.cu` files.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CudaType {
    /// `int`
    Int,
    /// `long`
    Long,
    /// `float`
    Float,
    /// `double`
    Double,
    /// `__half` (CUDA half-precision float)
    Half,
    /// `bool`
    Bool,
    /// `dim3` (three-component grid/block dimension)
    Dim3,
    /// `size_t`
    DimT,
    /// `cudaError_t`
    CudaErrorT,
    /// Pointer to inner type: `T*`
    Pointer(Box<CudaType>),
    /// `__shared__` qualified type (used internally for shared-mem decls)
    Shared(Box<CudaType>),
    /// `__constant__` qualified type
    Constant(Box<CudaType>),
    /// `__device__` qualified type
    Device(Box<CudaType>),
    /// Void: `void`
    Void,
    /// Unsigned int: `unsigned int`
    UInt,
    /// Named struct or typedef
    Named(String),
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CUDACacheEntry {
    pub key: String,
    pub data: Vec<u8>,
    pub timestamp: u64,
    pub valid: bool,
}

/// Constant folding helper for CUDAExt.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct CUDAExtConstFolder {
    pub(crate) folds: usize,
    pub(crate) failures: usize,
    pub(crate) enabled: bool,
}

/// Pass execution phase for CUDAExt.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CUDAExtPassPhase {
    Early,
    Middle,
    Late,
    Finalize,
}

/// Kind of `cudaMemcpy` transfer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemcpyKind {
    /// `cudaMemcpyHostToDevice`
    HostToDevice,
    /// `cudaMemcpyDeviceToHost`
    DeviceToHost,
    /// `cudaMemcpyDeviceToDevice`
    DeviceToDevice,
    /// `cudaMemcpyHostToHost`
    HostToHost,
}

/// Analysis cache for CUDAExt.
#[allow(dead_code)]
#[derive(Debug)]
pub struct CUDAExtCache {
    pub(crate) entries: Vec<(u64, Vec<u8>, bool, u32)>,
    pub(crate) cap: usize,
    pub(crate) total_hits: u64,
    pub(crate) total_misses: u64,
}

/// Liveness analysis for CUDAExt.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct CUDAExtLiveness {
    pub live_in: Vec<Vec<usize>>,
    pub live_out: Vec<Vec<usize>>,
    pub defs: Vec<Vec<usize>>,
    pub uses: Vec<Vec<usize>>,
}

#[allow(dead_code)]
pub struct CUDAPassRegistry {
    pub(crate) configs: Vec<CUDAPassConfig>,
    pub(crate) stats: std::collections::HashMap<String, CUDAPassStats>,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CUDAAnalysisCache {
    pub(crate) entries: std::collections::HashMap<String, CUDACacheEntry>,
    pub(crate) max_size: usize,
    pub(crate) hits: u64,
    pub(crate) misses: u64,
}

/// Dependency graph for CUDAExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CUDAExtDepGraph {
    pub(crate) n: usize,
    pub(crate) adj: Vec<Vec<usize>>,
    pub(crate) rev: Vec<Vec<usize>>,
    pub(crate) edge_count: usize,
}

/// Statistics for CUDAExt passes.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct CUDAExtPassStats {
    pub iterations: usize,
    pub changed: bool,
    pub nodes_visited: usize,
    pub nodes_modified: usize,
    pub time_ms: u64,
    pub memory_bytes: usize,
    pub errors: usize,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum CUDAPassPhase {
    Analysis,
    Transformation,
    Verification,
    Cleanup,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CUDAPassConfig {
    pub phase: CUDAPassPhase,
    pub enabled: bool,
    pub max_iterations: u32,
    pub debug_output: bool,
    pub pass_name: String,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CUDADepGraph {
    pub(crate) nodes: Vec<u32>,
    pub(crate) edges: Vec<(u32, u32)>,
}

#[allow(dead_code)]
pub struct CUDAConstantFoldingHelper;

/// Configuration for CUDAExt passes.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CUDAExtPassConfig {
    pub name: String,
    pub phase: CUDAExtPassPhase,
    pub enabled: bool,
    pub max_iterations: usize,
    pub debug: u32,
    pub timeout_ms: Option<u64>,
}

/// CUDA function / variable qualifiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CudaQualifier {
    /// `__global__` — kernel callable from host, runs on device
    Global,
    /// `__device__` — callable/usable only on device
    Device,
    /// `__host__` — callable only from host (default)
    Host,
    /// `__shared__` — shared memory within a thread block
    Shared,
    /// `__constant__` — read-only constant memory
    Constant,
    /// `__managed__` — accessible from both host and device
    Managed,
    /// `__restrict__` — pointer alias hint
    Restrict,
    /// `volatile` — volatile memory access
    Volatile,
}

/// Optional launch-bounds hint: `__launch_bounds__(maxThreads[, minBlocks])`.
#[derive(Debug, Clone, PartialEq)]
pub struct LaunchBounds {
    /// Maximum threads per block
    pub max_threads: u32,
    /// Minimum blocks per multiprocessor (optional)
    pub min_blocks: Option<u32>,
}

/// Dominator tree for CUDAExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CUDAExtDomTree {
    pub(crate) idom: Vec<Option<usize>>,
    pub(crate) children: Vec<Vec<usize>>,
    pub(crate) depth: Vec<usize>,
}

/// Binary operators available in CUDA C++ expressions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CudaBinOp {
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

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CUDADominatorTree {
    pub idom: Vec<Option<u32>>,
    pub dom_children: Vec<Vec<u32>>,
    pub dom_depth: Vec<u32>,
}

/// Pass registry for CUDAExt.
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct CUDAExtPassRegistry {
    pub(crate) configs: Vec<CUDAExtPassConfig>,
    pub(crate) stats: Vec<CUDAExtPassStats>,
}
