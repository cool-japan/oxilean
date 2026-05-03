use crate::lcnf::*;
use std::collections::{HashMap, HashSet, VecDeque};

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LLVMLivenessInfo {
    pub live_in: Vec<std::collections::HashSet<u32>>,
    pub live_out: Vec<std::collections::HashSet<u32>>,
    pub defs: Vec<std::collections::HashSet<u32>>,
    pub uses: Vec<std::collections::HashSet<u32>>,
}

/// A top-level LLVM function (define or declare).
#[derive(Debug, Clone, PartialEq)]
pub struct LlvmFunc {
    /// Function name (without leading `@`).
    pub name: String,
    /// Return type.
    pub ret_ty: LlvmType,
    /// Parameters: (type, name without `%`).
    pub params: Vec<(LlvmType, String)>,
    /// Function body (instructions). Empty if `is_declare`.
    pub body: Vec<LlvmInstr>,
    /// Linkage for this function.
    pub linkage: LlvmLinkage,
    /// Function attributes.
    pub attrs: Vec<LlvmAttr>,
    /// If true, emit `declare` (external function) instead of `define`.
    pub is_declare: bool,
}

/// LLVM linkage type for globals and functions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LlvmLinkage {
    /// `private` — local to the module, not exported
    Private,
    /// `internal` — local to the module (like C `static`)
    Internal,
    /// `external` — visible outside the module (default)
    External,
    /// `linkonce` — merged at link time
    LinkOnce,
    /// `weak` — weak symbol
    Weak,
    /// `common` — common symbol (like tentative definitions in C)
    Common,
    /// `appending` — for metadata arrays (e.g. llvm.used)
    Appending,
    /// `extern_weak` — weak external symbol
    ExternWeak,
    /// `linkonce_odr` — one-definition-rule linkonce
    LinkOnceOdr,
    /// `weak_odr` — one-definition-rule weak
    WeakOdr,
}

/// Floating-point comparison predicates for `fcmp`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FcmpPred {
    /// Ordered equal: `oeq`
    Oeq,
    /// Ordered not equal: `one`
    One,
    /// Ordered less than: `olt`
    Olt,
    /// Ordered greater than: `ogt`
    Ogt,
    /// Ordered less or equal: `ole`
    Ole,
    /// Ordered greater or equal: `oge`
    Oge,
    /// Unordered: `uno`
    Uno,
    /// Ordered: `ord`
    Ord,
    /// Always true: `true`
    True_,
    /// Always false: `false`
    False_,
}

/// LLVM type representation.
#[derive(Debug, Clone, PartialEq)]
pub enum LlvmType {
    /// `void`
    Void,
    /// `i1`
    I1,
    /// `i8`
    I8,
    /// `i16`
    I16,
    /// `i32`
    I32,
    /// `i64`
    I64,
    /// `i128`
    I128,
    /// `float`
    F32,
    /// `double`
    F64,
    /// `fp128`
    F128,
    /// `ptr` (opaque pointer, LLVM 15+)
    Ptr,
    /// `[N x T]` — array of N elements of type T
    Array(u64, Box<LlvmType>),
    /// `{ T1, T2, ... }` — anonymous struct
    Struct(Vec<LlvmType>),
    /// `<N x T>` — vector of N elements of type T
    Vector(u32, Box<LlvmType>),
    /// Function type: `ret (params...)`
    FuncType {
        ret: Box<LlvmType>,
        params: Vec<LlvmType>,
        variadic: bool,
    },
    /// Named/opaque type: `%MyType`
    Named(String),
}

/// Pass registry for LLVMExt.
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct LLVMExtPassRegistry {
    pub(crate) configs: Vec<LLVMExtPassConfig>,
    pub(crate) stats: Vec<LLVMExtPassStats>,
}

/// Liveness analysis for LLVMExt.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct LLVMExtLiveness {
    pub live_in: Vec<Vec<usize>>,
    pub live_out: Vec<Vec<usize>>,
    pub defs: Vec<Vec<usize>>,
    pub uses: Vec<Vec<usize>>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct LLVMPassStats {
    pub total_runs: u32,
    pub successful_runs: u32,
    pub total_changes: u64,
    pub time_ms: u64,
    pub iterations_used: u32,
}

/// Worklist for LLVMExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LLVMExtWorklist {
    pub(crate) items: std::collections::VecDeque<usize>,
    pub(crate) present: Vec<bool>,
}

/// A complete LLVM IR module.
#[derive(Debug, Clone, Default)]
pub struct LlvmModule {
    /// Source filename hint.
    pub source_filename: String,
    /// Target triple, e.g. `"x86_64-unknown-linux-gnu"`.
    pub target_triple: String,
    /// Data layout string.
    pub data_layout: String,
    /// Named type aliases.
    pub type_aliases: Vec<LlvmTypeAlias>,
    /// Global variables.
    pub globals: Vec<LlvmGlobal>,
    /// Function definitions and declarations.
    pub functions: Vec<LlvmFunc>,
    /// Module-level metadata (name, value pairs).
    pub metadata: Vec<(String, String)>,
}

/// Constant folding helper for LLVMExt.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct LLVMExtConstFolder {
    pub(crate) folds: usize,
    pub(crate) failures: usize,
    pub(crate) enabled: bool,
}

/// LLVM function or parameter attribute.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LlvmAttr {
    /// `nounwind` — function does not unwind
    NoUnwind,
    /// `readonly` — function only reads memory
    ReadOnly,
    /// `writeonly` — function only writes memory
    WriteOnly,
    /// `noreturn` — function never returns
    NoReturn,
    /// `noalias` — pointer does not alias
    NoAlias,
    /// `align(N)` — alignment attribute
    Align(u32),
    /// `dereferenceable(N)` — pointer is dereferenceable for N bytes
    Dereferenceable(u64),
    /// `inlinehint` — hint to inline
    InlineHint,
    /// `alwaysinline` — always inline
    AlwaysInline,
    /// `noinline` — never inline
    NoInline,
    /// `cold` — function is rarely called
    Cold,
    /// `optsize` — optimize for size
    OptSize,
    /// `uwtable` — requires unwind table
    UwTable,
    /// `sspstrong` — stack smash protector
    StackProtect,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LLVMDepGraph {
    pub(crate) nodes: Vec<u32>,
    pub(crate) edges: Vec<(u32, u32)>,
}

/// Pass execution phase for LLVMExt.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LLVMExtPassPhase {
    Early,
    Middle,
    Late,
    Finalize,
}

/// A named type alias: `%Name = type { ... }`
#[derive(Debug, Clone, PartialEq)]
pub struct LlvmTypeAlias {
    /// Name of the type (without leading `%`).
    pub name: String,
    /// The underlying type.
    pub ty: LlvmType,
}

/// LLVM value (operand) representation.
#[derive(Debug, Clone, PartialEq)]
pub enum LlvmValue {
    /// Integer constant: `42`
    Const(i64),
    /// Floating-point constant: `3.14`
    Float(f64),
    /// `undef`
    Undef,
    /// `null`
    Null,
    /// `true` (i1 constant 1)
    True_,
    /// `false` (i1 constant 0)
    False_,
    /// Global reference: `@name`
    GlobalRef(String),
    /// Local register reference: `%name`
    LocalRef(String),
    /// Constant array: `[i32 1, i32 2, ...]`
    ConstArray(LlvmType, Vec<LlvmValue>),
    /// Constant struct: `{ i32 1, ptr null }`
    ConstStruct(Vec<LlvmValue>),
    /// `zeroinitializer`
    ZeroInitializer,
}

/// Integer comparison predicates for `icmp`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IcmpPred {
    /// Equal: `eq`
    Eq,
    /// Not equal: `ne`
    Ne,
    /// Signed less than: `slt`
    Slt,
    /// Signed greater than: `sgt`
    Sgt,
    /// Signed less or equal: `sle`
    Sle,
    /// Signed greater or equal: `sge`
    Sge,
    /// Unsigned less than: `ult`
    Ult,
    /// Unsigned greater than: `ugt`
    Ugt,
    /// Unsigned less or equal: `ule`
    Ule,
    /// Unsigned greater or equal: `uge`
    Uge,
}

/// Configuration for LLVMExt passes.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LLVMExtPassConfig {
    pub name: String,
    pub phase: LLVMExtPassPhase,
    pub enabled: bool,
    pub max_iterations: usize,
    pub debug: u32,
    pub timeout_ms: Option<u64>,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LLVMAnalysisCache {
    pub(crate) entries: std::collections::HashMap<String, LLVMCacheEntry>,
    pub(crate) max_size: usize,
    pub(crate) hits: u64,
    pub(crate) misses: u64,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LLVMCacheEntry {
    pub key: String,
    pub data: Vec<u8>,
    pub timestamp: u64,
    pub valid: bool,
}

/// Analysis cache for LLVMExt.
#[allow(dead_code)]
#[derive(Debug)]
pub struct LLVMExtCache {
    pub(crate) entries: Vec<(u64, Vec<u8>, bool, u32)>,
    pub(crate) cap: usize,
    pub(crate) total_hits: u64,
    pub(crate) total_misses: u64,
}

/// Dependency graph for LLVMExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LLVMExtDepGraph {
    pub(crate) n: usize,
    pub(crate) adj: Vec<Vec<usize>>,
    pub(crate) rev: Vec<Vec<usize>>,
    pub(crate) edge_count: usize,
}

/// LLVM IR code generation backend.
///
/// Compiles LCNF declarations to LLVM IR text format.
pub struct LlvmBackend {
    /// Counter for generating fresh register names.
    pub(crate) reg_counter: u64,
}

#[allow(dead_code)]
pub struct LLVMPassRegistry {
    pub(crate) configs: Vec<LLVMPassConfig>,
    pub(crate) stats: std::collections::HashMap<String, LLVMPassStats>,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LLVMWorklist {
    pub(crate) items: std::collections::VecDeque<u32>,
    pub(crate) in_worklist: std::collections::HashSet<u32>,
}

#[allow(dead_code)]
pub struct LLVMConstantFoldingHelper;

/// Dominator tree for LLVMExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LLVMExtDomTree {
    pub(crate) idom: Vec<Option<usize>>,
    pub(crate) children: Vec<Vec<usize>>,
    pub(crate) depth: Vec<usize>,
}

/// A top-level LLVM global variable.
#[derive(Debug, Clone, PartialEq)]
pub struct LlvmGlobal {
    /// Global name (without leading `@`).
    pub name: String,
    /// Type of the global.
    pub ty: LlvmType,
    /// Linkage.
    pub linkage: LlvmLinkage,
    /// If true, emit `constant` instead of `global`.
    pub is_constant: bool,
    /// Optional initializer. `None` for `external` declarations.
    pub init: Option<LlvmValue>,
    /// Optional alignment.
    pub align: Option<u32>,
}

/// Statistics for LLVMExt passes.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct LLVMExtPassStats {
    pub iterations: usize,
    pub changed: bool,
    pub nodes_visited: usize,
    pub nodes_modified: usize,
    pub time_ms: u64,
    pub memory_bytes: usize,
    pub errors: usize,
}

/// LLVM IR instruction.
#[derive(Debug, Clone, PartialEq)]
pub enum LlvmInstr {
    /// `%result = alloca ty [, align N]`
    Alloca {
        result: String,
        ty: LlvmType,
        align: Option<u32>,
    },
    /// `%result = load ty, ptr %ptr [, align N]`
    Load {
        result: String,
        ty: LlvmType,
        ptr: LlvmValue,
        align: Option<u32>,
    },
    /// `store ty %val, ptr %ptr [, align N]`
    Store {
        val: LlvmValue,
        ty: LlvmType,
        ptr: LlvmValue,
        align: Option<u32>,
    },
    /// `%result = add i64 %lhs, %rhs`
    Add {
        result: String,
        lhs: LlvmValue,
        rhs: LlvmValue,
    },
    /// `%result = sub i64 %lhs, %rhs`
    Sub {
        result: String,
        lhs: LlvmValue,
        rhs: LlvmValue,
    },
    /// `%result = mul i64 %lhs, %rhs`
    Mul {
        result: String,
        lhs: LlvmValue,
        rhs: LlvmValue,
    },
    /// `%result = sdiv i64 %lhs, %rhs`
    SDiv {
        result: String,
        lhs: LlvmValue,
        rhs: LlvmValue,
    },
    /// `%result = srem i64 %lhs, %rhs`
    SRem {
        result: String,
        lhs: LlvmValue,
        rhs: LlvmValue,
    },
    /// `%result = fadd double %lhs, %rhs`
    FAdd {
        result: String,
        lhs: LlvmValue,
        rhs: LlvmValue,
    },
    /// `%result = fsub double %lhs, %rhs`
    FSub {
        result: String,
        lhs: LlvmValue,
        rhs: LlvmValue,
    },
    /// `%result = fmul double %lhs, %rhs`
    FMul {
        result: String,
        lhs: LlvmValue,
        rhs: LlvmValue,
    },
    /// `%result = fdiv double %lhs, %rhs`
    FDiv {
        result: String,
        lhs: LlvmValue,
        rhs: LlvmValue,
    },
    /// `%result = and i64 %lhs, %rhs`
    And {
        result: String,
        lhs: LlvmValue,
        rhs: LlvmValue,
    },
    /// `%result = or i64 %lhs, %rhs`
    Or {
        result: String,
        lhs: LlvmValue,
        rhs: LlvmValue,
    },
    /// `%result = xor i64 %lhs, %rhs`
    Xor {
        result: String,
        lhs: LlvmValue,
        rhs: LlvmValue,
    },
    /// `%result = shl i64 %lhs, %rhs`
    Shl {
        result: String,
        lhs: LlvmValue,
        rhs: LlvmValue,
    },
    /// `%result = lshr i64 %lhs, %rhs`
    LShr {
        result: String,
        lhs: LlvmValue,
        rhs: LlvmValue,
    },
    /// `%result = ashr i64 %lhs, %rhs`
    AShr {
        result: String,
        lhs: LlvmValue,
        rhs: LlvmValue,
    },
    /// `%result = icmp pred i64 %lhs, %rhs`
    ICmp {
        result: String,
        pred: IcmpPred,
        lhs: LlvmValue,
        rhs: LlvmValue,
    },
    /// `%result = fcmp pred double %lhs, %rhs`
    FCmp {
        result: String,
        pred: FcmpPred,
        lhs: LlvmValue,
        rhs: LlvmValue,
    },
    /// `br label %target`
    Br(String),
    /// `br i1 %cond, label %true_, label %false_`
    CondBr {
        cond: LlvmValue,
        true_: String,
        false_: String,
    },
    /// `ret void` or `ret ty val`
    Ret(Option<(LlvmType, LlvmValue)>),
    /// `unreachable`
    Unreachable,
    /// A basic block label: `name:`
    Label(String),
    /// `[%result = ] call ret_ty @func(args...)`
    Call {
        result: Option<String>,
        ret_ty: LlvmType,
        func: String,
        args: Vec<(LlvmType, LlvmValue)>,
    },
    /// `%result = getelementptr inbounds base_ty, ptr %ptr, indices...`
    GetElementPtr {
        result: String,
        base_ty: LlvmType,
        ptr: LlvmValue,
        indices: Vec<(LlvmType, LlvmValue)>,
    },
    /// `%result = bitcast from_ty %val to to_ty`
    BitCast {
        result: String,
        val: LlvmValue,
        from_ty: LlvmType,
        to_ty: LlvmType,
    },
    /// `%result = phi ty [ %val1, %bb1 ], [ %val2, %bb2 ]`
    Phi {
        result: String,
        ty: LlvmType,
        incoming: Vec<(LlvmValue, String)>,
    },
    /// `%result = select i1 %cond, ty %true_val, ty %false_val`
    Select {
        result: String,
        cond: LlvmValue,
        true_val: LlvmValue,
        false_val: LlvmValue,
        ty: LlvmType,
    },
    /// `%result = extractvalue agg_ty %agg, indices...`
    ExtractValue {
        result: String,
        agg: LlvmValue,
        agg_ty: LlvmType,
        indices: Vec<u32>,
    },
    /// `%result = insertvalue agg_ty %agg, val_ty %val, indices...`
    InsertValue {
        result: String,
        agg: LlvmValue,
        agg_ty: LlvmType,
        val: LlvmValue,
        val_ty: LlvmType,
        indices: Vec<u32>,
    },
    /// `%result = zext from_ty %val to to_ty`
    ZExt {
        result: String,
        val: LlvmValue,
        from_ty: LlvmType,
        to_ty: LlvmType,
    },
    /// `%result = sext from_ty %val to to_ty`
    SExt {
        result: String,
        val: LlvmValue,
        from_ty: LlvmType,
        to_ty: LlvmType,
    },
    /// `%result = trunc from_ty %val to to_ty`
    Trunc {
        result: String,
        val: LlvmValue,
        from_ty: LlvmType,
        to_ty: LlvmType,
    },
    /// `%result = fptosi from_ty %val to to_ty`
    FpToSI {
        result: String,
        val: LlvmValue,
        from_ty: LlvmType,
        to_ty: LlvmType,
    },
    /// `%result = sitofp from_ty %val to to_ty`
    SIToFp {
        result: String,
        val: LlvmValue,
        from_ty: LlvmType,
        to_ty: LlvmType,
    },
    /// `%result = fpext from_ty %val to to_ty`
    FpExt {
        result: String,
        val: LlvmValue,
        from_ty: LlvmType,
        to_ty: LlvmType,
    },
    /// `%result = fptrunc from_ty %val to to_ty`
    FpTrunc {
        result: String,
        val: LlvmValue,
        from_ty: LlvmType,
        to_ty: LlvmType,
    },
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum LLVMPassPhase {
    Analysis,
    Transformation,
    Verification,
    Cleanup,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LLVMPassConfig {
    pub phase: LLVMPassPhase,
    pub enabled: bool,
    pub max_iterations: u32,
    pub debug_output: bool,
    pub pass_name: String,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LLVMDominatorTree {
    pub idom: Vec<Option<u32>>,
    pub dom_children: Vec<Vec<u32>>,
    pub dom_depth: Vec<u32>,
}
