//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::HashMap;

use std::collections::{HashSet, VecDeque};

/// LLVM IR type representation.
///
/// Covers all primitive and compound types found in LLVM IR text format.
#[derive(Debug, Clone, PartialEq)]
pub enum LlvmIrType {
    /// `void` — no value.
    Void,
    /// `i1` — single-bit integer (boolean).
    I1,
    /// `i8` — 8-bit integer.
    I8,
    /// `i16` — 16-bit integer.
    I16,
    /// `i32` — 32-bit integer.
    I32,
    /// `i64` — 64-bit integer.
    I64,
    /// `i128` — 128-bit integer.
    I128,
    /// Arbitrary-width integer `iN`.
    IArb(u32),
    /// `float` — 32-bit IEEE 754 float.
    Float,
    /// `double` — 64-bit IEEE 754 float.
    Double,
    /// `fp128` — 128-bit IEEE 754 float.
    Fp128,
    /// `x86_fp80` — 80-bit x86 extended precision.
    X86Fp80,
    /// `ptr` — opaque pointer (LLVM 15+ default).
    Ptr,
    /// `ptr addrspace(N)` — pointer in address space N.
    PtrAs(u32),
    /// `[N x T]` — array of N elements of type T.
    Array(u64, Box<LlvmIrType>),
    /// `{ T1, T2, ... }` — literal struct type.
    Struct(Vec<LlvmIrType>),
    /// `<{ T1, T2, ... }>` — packed struct type.
    PackedStruct(Vec<LlvmIrType>),
    /// `<N x T>` — fixed-length vector type.
    Vector(u32, Box<LlvmIrType>),
    /// `<vscale x N x T>` — scalable vector type.
    ScalableVector(u32, Box<LlvmIrType>),
    /// Named/identified struct type: `%Name`.
    Named(String),
    /// Function type: `ret_ty (param_ty, ...)`.
    Func {
        /// Return type.
        ret: Box<LlvmIrType>,
        /// Parameter types.
        params: Vec<LlvmIrType>,
        /// Whether the function is variadic.
        variadic: bool,
    },
    /// `label` — used for branch targets.
    Label,
    /// `metadata` — LLVM metadata type.
    Metadata,
    /// `token` — LLVM token type.
    Token,
}
/// LLVM IR instruction representation.
///
/// Each variant maps directly to one (or occasionally two) LLVM IR text lines.
#[derive(Debug, Clone, PartialEq)]
pub enum LlvmIrInstr {
    /// `%dest = alloca ty [, align N]`
    Alloca {
        dest: String,
        ty: LlvmIrType,
        align: Option<u32>,
    },
    /// `%dest = load ty, ptr %ptr [, align N]`
    Load {
        dest: String,
        ty: LlvmIrType,
        ptr: LlvmIrValue,
        align: Option<u32>,
        volatile: bool,
    },
    /// `store ty %val, ptr %ptr [, align N]`
    Store {
        ty: LlvmIrType,
        val: LlvmIrValue,
        ptr: LlvmIrValue,
        align: Option<u32>,
        volatile: bool,
    },
    /// `%dest = getelementptr [inbounds] ty, ptr %ptr, i32/i64 %idx, ...`
    Gep {
        dest: String,
        inbounds: bool,
        elem_ty: LlvmIrType,
        ptr: LlvmIrValue,
        indices: Vec<(LlvmIrType, LlvmIrValue)>,
    },
    /// `%dest = add [nsw] [nuw] ty %lhs, %rhs`
    Add {
        dest: String,
        ty: LlvmIrType,
        lhs: LlvmIrValue,
        rhs: LlvmIrValue,
        nsw: bool,
        nuw: bool,
    },
    /// `%dest = sub [nsw] [nuw] ty %lhs, %rhs`
    Sub {
        dest: String,
        ty: LlvmIrType,
        lhs: LlvmIrValue,
        rhs: LlvmIrValue,
        nsw: bool,
        nuw: bool,
    },
    /// `%dest = mul [nsw] [nuw] ty %lhs, %rhs`
    Mul {
        dest: String,
        ty: LlvmIrType,
        lhs: LlvmIrValue,
        rhs: LlvmIrValue,
        nsw: bool,
        nuw: bool,
    },
    /// `%dest = sdiv [exact] ty %lhs, %rhs`
    Sdiv {
        dest: String,
        ty: LlvmIrType,
        lhs: LlvmIrValue,
        rhs: LlvmIrValue,
        exact: bool,
    },
    /// `%dest = udiv [exact] ty %lhs, %rhs`
    Udiv {
        dest: String,
        ty: LlvmIrType,
        lhs: LlvmIrValue,
        rhs: LlvmIrValue,
        exact: bool,
    },
    /// `%dest = srem ty %lhs, %rhs`
    Srem {
        dest: String,
        ty: LlvmIrType,
        lhs: LlvmIrValue,
        rhs: LlvmIrValue,
    },
    /// `%dest = urem ty %lhs, %rhs`
    Urem {
        dest: String,
        ty: LlvmIrType,
        lhs: LlvmIrValue,
        rhs: LlvmIrValue,
    },
    /// `%dest = and ty %lhs, %rhs`
    And {
        dest: String,
        ty: LlvmIrType,
        lhs: LlvmIrValue,
        rhs: LlvmIrValue,
    },
    /// `%dest = or ty %lhs, %rhs`
    Or {
        dest: String,
        ty: LlvmIrType,
        lhs: LlvmIrValue,
        rhs: LlvmIrValue,
    },
    /// `%dest = xor ty %lhs, %rhs`
    Xor {
        dest: String,
        ty: LlvmIrType,
        lhs: LlvmIrValue,
        rhs: LlvmIrValue,
    },
    /// `%dest = shl [nsw] [nuw] ty %lhs, %rhs`
    Shl {
        dest: String,
        ty: LlvmIrType,
        lhs: LlvmIrValue,
        rhs: LlvmIrValue,
    },
    /// `%dest = lshr [exact] ty %lhs, %rhs`
    Lshr {
        dest: String,
        ty: LlvmIrType,
        lhs: LlvmIrValue,
        rhs: LlvmIrValue,
    },
    /// `%dest = ashr [exact] ty %lhs, %rhs`
    Ashr {
        dest: String,
        ty: LlvmIrType,
        lhs: LlvmIrValue,
        rhs: LlvmIrValue,
    },
    /// `%dest = fadd [fast] ty %lhs, %rhs`
    Fadd {
        dest: String,
        ty: LlvmIrType,
        lhs: LlvmIrValue,
        rhs: LlvmIrValue,
        fast: bool,
    },
    /// `%dest = fsub [fast] ty %lhs, %rhs`
    Fsub {
        dest: String,
        ty: LlvmIrType,
        lhs: LlvmIrValue,
        rhs: LlvmIrValue,
        fast: bool,
    },
    /// `%dest = fmul [fast] ty %lhs, %rhs`
    Fmul {
        dest: String,
        ty: LlvmIrType,
        lhs: LlvmIrValue,
        rhs: LlvmIrValue,
        fast: bool,
    },
    /// `%dest = fdiv [fast] ty %lhs, %rhs`
    Fdiv {
        dest: String,
        ty: LlvmIrType,
        lhs: LlvmIrValue,
        rhs: LlvmIrValue,
        fast: bool,
    },
    /// `%dest = fneg [fast] ty %val`
    Fneg {
        dest: String,
        ty: LlvmIrType,
        val: LlvmIrValue,
        fast: bool,
    },
    /// `%dest = icmp pred ty %lhs, %rhs`
    Icmp {
        dest: String,
        pred: IcmpPred,
        ty: LlvmIrType,
        lhs: LlvmIrValue,
        rhs: LlvmIrValue,
    },
    /// `%dest = fcmp [fast] pred ty %lhs, %rhs`
    Fcmp {
        dest: String,
        pred: FcmpPred,
        ty: LlvmIrType,
        lhs: LlvmIrValue,
        rhs: LlvmIrValue,
        fast: bool,
    },
    /// `%dest = trunc ty %val to ty2`
    Trunc {
        dest: String,
        val: LlvmIrValue,
        from_ty: LlvmIrType,
        to_ty: LlvmIrType,
    },
    /// `%dest = zext ty %val to ty2`
    Zext {
        dest: String,
        val: LlvmIrValue,
        from_ty: LlvmIrType,
        to_ty: LlvmIrType,
    },
    /// `%dest = sext ty %val to ty2`
    Sext {
        dest: String,
        val: LlvmIrValue,
        from_ty: LlvmIrType,
        to_ty: LlvmIrType,
    },
    /// `%dest = fptrunc ty %val to ty2`
    Fptrunc {
        dest: String,
        val: LlvmIrValue,
        from_ty: LlvmIrType,
        to_ty: LlvmIrType,
    },
    /// `%dest = fpext ty %val to ty2`
    Fpext {
        dest: String,
        val: LlvmIrValue,
        from_ty: LlvmIrType,
        to_ty: LlvmIrType,
    },
    /// `%dest = fptoui ty %val to ty2`
    Fptoui {
        dest: String,
        val: LlvmIrValue,
        from_ty: LlvmIrType,
        to_ty: LlvmIrType,
    },
    /// `%dest = fptosi ty %val to ty2`
    Fptosi {
        dest: String,
        val: LlvmIrValue,
        from_ty: LlvmIrType,
        to_ty: LlvmIrType,
    },
    /// `%dest = uitofp ty %val to ty2`
    Uitofp {
        dest: String,
        val: LlvmIrValue,
        from_ty: LlvmIrType,
        to_ty: LlvmIrType,
    },
    /// `%dest = sitofp ty %val to ty2`
    Sitofp {
        dest: String,
        val: LlvmIrValue,
        from_ty: LlvmIrType,
        to_ty: LlvmIrType,
    },
    /// `%dest = ptrtoint ty %val to ty2`
    Ptrtoint {
        dest: String,
        val: LlvmIrValue,
        from_ty: LlvmIrType,
        to_ty: LlvmIrType,
    },
    /// `%dest = inttoptr ty %val to ty2`
    Inttoptr {
        dest: String,
        val: LlvmIrValue,
        from_ty: LlvmIrType,
        to_ty: LlvmIrType,
    },
    /// `%dest = bitcast ty %val to ty2`
    Bitcast {
        dest: String,
        val: LlvmIrValue,
        from_ty: LlvmIrType,
        to_ty: LlvmIrType,
    },
    /// `br label %dest`
    BrUnconditional { dest: String },
    /// `br i1 %cond, label %true_dest, label %false_dest`
    BrConditional {
        cond: LlvmIrValue,
        true_dest: String,
        false_dest: String,
    },
    /// `ret void`
    RetVoid,
    /// `ret ty %val`
    Ret { ty: LlvmIrType, val: LlvmIrValue },
    /// `unreachable`
    Unreachable,
    /// `switch ty %val, label %default [ ty val1, label %dest1 ... ]`
    Switch {
        ty: LlvmIrType,
        val: LlvmIrValue,
        default: String,
        cases: Vec<(LlvmIrValue, String)>,
    },
    /// `[%dest =] call [tailcc] ret_ty @func(args...)`
    Call {
        dest: Option<String>,
        ret_ty: LlvmIrType,
        func: LlvmIrValue,
        args: Vec<(LlvmIrType, LlvmIrValue)>,
        tail: bool,
        cc: CallingConv,
    },
    /// `[%dest =] invoke ret_ty @func(args) to label %normal unwind label %unwind`
    Invoke {
        dest: Option<String>,
        ret_ty: LlvmIrType,
        func: LlvmIrValue,
        args: Vec<(LlvmIrType, LlvmIrValue)>,
        normal: String,
        unwind: String,
    },
    /// `%dest = phi ty [ %v1, %bb1 ], [ %v2, %bb2 ], ...`
    Phi {
        dest: String,
        ty: LlvmIrType,
        incoming: Vec<(LlvmIrValue, String)>,
    },
    /// `%dest = select i1 %cond, ty %true_val, ty %false_val`
    Select {
        dest: String,
        cond: LlvmIrValue,
        ty: LlvmIrType,
        true_val: LlvmIrValue,
        false_val: LlvmIrValue,
    },
    /// `%dest = extractelement <N x ty> %vec, i32 %idx`
    ExtractElement {
        dest: String,
        vec_ty: LlvmIrType,
        vec: LlvmIrValue,
        idx: LlvmIrValue,
    },
    /// `%dest = insertelement <N x ty> %vec, ty %val, i32 %idx`
    InsertElement {
        dest: String,
        vec_ty: LlvmIrType,
        vec: LlvmIrValue,
        ty: LlvmIrType,
        val: LlvmIrValue,
        idx: LlvmIrValue,
    },
    /// `%dest = extractvalue {ty, ...} %agg, idx, ...`
    ExtractValue {
        dest: String,
        agg_ty: LlvmIrType,
        agg: LlvmIrValue,
        indices: Vec<u32>,
    },
    /// `%dest = insertvalue {ty, ...} %agg, ty %val, idx, ...`
    InsertValue {
        dest: String,
        agg_ty: LlvmIrType,
        agg: LlvmIrValue,
        elem_ty: LlvmIrType,
        val: LlvmIrValue,
        indices: Vec<u32>,
    },
    /// A verbatim text line (for comments, annotations, etc.).
    Raw(String),
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LITLivenessInfo {
    pub live_in: Vec<std::collections::HashSet<u32>>,
    pub live_out: Vec<std::collections::HashSet<u32>>,
    pub defs: Vec<std::collections::HashSet<u32>>,
    pub uses: Vec<std::collections::HashSet<u32>>,
}
impl LITLivenessInfo {
    #[allow(dead_code)]
    pub fn new(block_count: usize) -> Self {
        LITLivenessInfo {
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
pub struct LITDepGraph {
    pub(super) nodes: Vec<u32>,
    pub(super) edges: Vec<(u32, u32)>,
}
impl LITDepGraph {
    #[allow(dead_code)]
    pub fn new() -> Self {
        LITDepGraph {
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
/// A function parameter with type, name, and optional attributes.
#[derive(Debug, Clone)]
pub struct LlvmIrParam {
    /// Parameter type.
    pub ty: LlvmIrType,
    /// Parameter name (without `%` prefix).
    pub name: String,
    /// LLVM parameter attributes (e.g. `noalias`, `nonnull`, `readonly`).
    pub attrs: Vec<String>,
}
impl LlvmIrParam {
    /// Create a simple parameter with no attributes.
    pub fn new(ty: LlvmIrType, name: impl Into<String>) -> Self {
        Self {
            ty,
            name: name.into(),
            attrs: Vec::new(),
        }
    }
    /// Create a parameter with attributes.
    pub fn with_attrs(ty: LlvmIrType, name: impl Into<String>, attrs: Vec<String>) -> Self {
        Self {
            ty,
            name: name.into(),
            attrs,
        }
    }
}
/// LLVM linkage types for globals and functions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Linkage {
    /// Default external linkage.
    #[default]
    External,
    /// Internal (file-private) linkage.
    Internal,
    /// Private (not in symbol table) linkage.
    Private,
    /// Weak (overridable) linkage.
    Weak,
    /// Linkonce (merged if identical) linkage.
    Linkonce,
    /// Common (zero-initialized external) linkage.
    Common,
    /// Available externally (imported definition).
    AvailableExternally,
    /// Weak ODR linkage.
    WeakOdr,
    /// Linkonce ODR linkage.
    LinkonceOdr,
    /// External weak linkage.
    ExternalWeak,
    /// Appending linkage (for global arrays).
    Appending,
}
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum LITPassPhase {
    Analysis,
    Transformation,
    Verification,
    Cleanup,
}
impl LITPassPhase {
    #[allow(dead_code)]
    pub fn name(&self) -> &str {
        match self {
            LITPassPhase::Analysis => "analysis",
            LITPassPhase::Transformation => "transformation",
            LITPassPhase::Verification => "verification",
            LITPassPhase::Cleanup => "cleanup",
        }
    }
    #[allow(dead_code)]
    pub fn is_modifying(&self) -> bool {
        matches!(self, LITPassPhase::Transformation | LITPassPhase::Cleanup)
    }
}
#[allow(dead_code)]
pub struct LITPassRegistry {
    pub(super) configs: Vec<LITPassConfig>,
    pub(super) stats: std::collections::HashMap<String, LITPassStats>,
}
impl LITPassRegistry {
    #[allow(dead_code)]
    pub fn new() -> Self {
        LITPassRegistry {
            configs: Vec::new(),
            stats: std::collections::HashMap::new(),
        }
    }
    #[allow(dead_code)]
    pub fn register(&mut self, config: LITPassConfig) {
        self.stats
            .insert(config.pass_name.clone(), LITPassStats::new());
        self.configs.push(config);
    }
    #[allow(dead_code)]
    pub fn enabled_passes(&self) -> Vec<&LITPassConfig> {
        self.configs.iter().filter(|c| c.enabled).collect()
    }
    #[allow(dead_code)]
    pub fn get_stats(&self, name: &str) -> Option<&LITPassStats> {
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
/// An LLVM IR global variable or constant.
#[derive(Debug, Clone)]
pub struct LlvmIrGlobal {
    /// Global name (without `@` prefix).
    pub name: String,
    /// Linkage.
    pub linkage: Linkage,
    /// Whether the global is a constant (vs. mutable variable).
    pub is_constant: bool,
    /// Value type.
    pub ty: LlvmIrType,
    /// Initial value (`None` = `zeroinitializer` for mutable, `undef` for external).
    pub initializer: Option<LlvmIrValue>,
    /// Optional alignment.
    pub align: Option<u32>,
    /// Optional section name.
    pub section: Option<String>,
    /// Address space.
    pub addr_space: Option<u32>,
}
impl LlvmIrGlobal {
    /// Create a global variable with an initializer.
    pub fn new(name: impl Into<String>, ty: LlvmIrType, initializer: LlvmIrValue) -> Self {
        Self {
            name: name.into(),
            linkage: Linkage::External,
            is_constant: false,
            ty,
            initializer: Some(initializer),
            align: None,
            section: None,
            addr_space: None,
        }
    }
    /// Create a global constant.
    pub fn constant(name: impl Into<String>, ty: LlvmIrType, initializer: LlvmIrValue) -> Self {
        let mut g = Self::new(name, ty, initializer);
        g.is_constant = true;
        g
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LITCacheEntry {
    pub key: String,
    pub data: Vec<u8>,
    pub timestamp: u64,
    pub valid: bool,
}
/// LLVM calling conventions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CallingConv {
    /// C calling convention (default).
    #[default]
    C,
    /// Fast calling convention.
    Fast,
    /// Cold calling convention.
    Cold,
    /// GHC calling convention.
    Ghc,
    /// WebAssembly calling convention.
    Wasm,
    /// Numbered calling convention.
    Num(u32),
}
/// A complete LLVM IR module.
///
/// Corresponds to a single `.ll` file.
#[derive(Debug, Clone)]
pub struct LlvmIrModule {
    /// Module identifier (e.g. filename or logical name).
    pub module_id: String,
    /// Target triple string (e.g. `"x86_64-pc-linux-gnu"`).
    pub target_triple: String,
    /// Data layout string.
    pub data_layout: String,
    /// Named type definitions (`%Name = type { ... }`).
    pub type_defs: Vec<(String, LlvmIrType)>,
    /// Global variables and constants.
    pub globals: Vec<LlvmIrGlobal>,
    /// Function definitions and declarations.
    pub functions: Vec<LlvmIrFunction>,
    /// Module-level metadata comments.
    pub metadata: HashMap<String, String>,
}
impl LlvmIrModule {
    /// Create a new empty module.
    pub fn new(module_id: impl Into<String>) -> Self {
        Self {
            module_id: module_id.into(),
            target_triple: String::new(),
            data_layout: String::new(),
            type_defs: Vec::new(),
            globals: Vec::new(),
            functions: Vec::new(),
            metadata: HashMap::new(),
        }
    }
    /// Set the target triple.
    pub fn set_target_triple(&mut self, triple: impl Into<String>) {
        self.target_triple = triple.into();
    }
    /// Set the data layout.
    pub fn set_data_layout(&mut self, layout: impl Into<String>) {
        self.data_layout = layout.into();
    }
    /// Add a named type definition.
    pub fn add_type_def(&mut self, name: impl Into<String>, ty: LlvmIrType) {
        self.type_defs.push((name.into(), ty));
    }
    /// Add a global variable.
    pub fn add_global(&mut self, global: LlvmIrGlobal) {
        self.globals.push(global);
    }
    /// Add a function.
    pub fn add_function(&mut self, func: LlvmIrFunction) {
        self.functions.push(func);
    }
    /// Add a metadata comment.
    pub fn set_metadata(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.metadata.insert(key.into(), value.into());
    }
}
/// A basic block in an LLVM IR function.
///
/// Starts with an optional label, contains non-terminator instructions,
/// and ends with exactly one terminator instruction.
#[derive(Debug, Clone)]
pub struct LlvmIrBlock {
    /// Label name for this block (e.g. `"entry"`, `"loop.head"`).
    pub label: String,
    /// Non-terminator instructions.
    pub instructions: Vec<LlvmIrInstr>,
    /// The terminator instruction (branch, ret, unreachable, etc.).
    pub terminator: LlvmIrInstr,
}
impl LlvmIrBlock {
    /// Create a new basic block with the given label and terminator.
    pub fn new(label: impl Into<String>, terminator: LlvmIrInstr) -> Self {
        Self {
            label: label.into(),
            instructions: Vec::new(),
            terminator,
        }
    }
    /// Append a non-terminator instruction to this block.
    pub fn push(&mut self, instr: LlvmIrInstr) {
        self.instructions.push(instr);
    }
    /// Number of instructions including the terminator.
    pub fn len(&self) -> usize {
        self.instructions.len() + 1
    }
    /// Returns true if the block has no instructions (only a terminator).
    pub fn is_empty(&self) -> bool {
        self.instructions.is_empty()
    }
}
/// Floating-point comparison predicates for `fcmp` instruction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FcmpPred {
    /// Always false.
    False,
    /// Ordered and equal.
    Oeq,
    /// Ordered and greater than.
    Ogt,
    /// Ordered and greater or equal.
    Oge,
    /// Ordered and less than.
    Olt,
    /// Ordered and less or equal.
    Ole,
    /// Ordered and not equal.
    One,
    /// Ordered (no NaN).
    Ord,
    /// Unordered or equal.
    Ueq,
    /// Unordered or greater than.
    Ugt,
    /// Unordered or greater or equal.
    Uge,
    /// Unordered or less than.
    Ult,
    /// Unordered or less or equal.
    Ule,
    /// Unordered or not equal.
    Une,
    /// Unordered (either NaN).
    Uno,
    /// Always true.
    True,
}
/// Emits `LlvmIrModule` as valid LLVM IR text (.ll format).
///
/// Usage:
/// ```
/// # use oxilean_codegen::llvm_ir_text::*;
/// let module = LlvmIrModule::new("example");
/// let emitter = LlvmIrTextEmitter::new();
/// let text = emitter.emit(&module);
/// assert!(text.contains("example"));
/// ```
#[derive(Debug, Default)]
pub struct LlvmIrTextEmitter {
    /// Indent string for function body lines (default: two spaces).
    pub(super) indent: String,
}
impl LlvmIrTextEmitter {
    /// Create a new emitter with default settings.
    pub fn new() -> Self {
        Self {
            indent: "  ".to_string(),
        }
    }
    /// Set the indentation string.
    pub fn with_indent(mut self, indent: impl Into<String>) -> Self {
        self.indent = indent.into();
        self
    }
    /// Emit the full module as an LLVM IR text string.
    pub fn emit(&self, module: &LlvmIrModule) -> String {
        let mut out = String::new();
        out.push_str(&format!("; ModuleID = '{}'\n", module.module_id));
        let mut meta_keys: Vec<&String> = module.metadata.keys().collect();
        meta_keys.sort();
        for key in meta_keys {
            out.push_str(&format!("; {}: {}\n", key, module.metadata[key]));
        }
        if !module.data_layout.is_empty() {
            out.push_str(&format!("target datalayout = \"{}\"\n", module.data_layout));
        }
        if !module.target_triple.is_empty() {
            out.push_str(&format!("target triple = \"{}\"\n", module.target_triple));
        }
        if !module.data_layout.is_empty() || !module.target_triple.is_empty() {
            out.push('\n');
        }
        for (name, ty) in &module.type_defs {
            out.push_str(&format!("%{} = type {}\n", name, ty));
        }
        if !module.type_defs.is_empty() {
            out.push('\n');
        }
        for global in &module.globals {
            out.push_str(&self.emit_global(global));
        }
        if !module.globals.is_empty() {
            out.push('\n');
        }
        for func in &module.functions {
            out.push_str(&self.emit_function(func));
            out.push('\n');
        }
        out
    }
    pub(super) fn emit_global(&self, global: &LlvmIrGlobal) -> String {
        let mut line = format!("@{} = {}", global.name, global.linkage);
        if let Some(addr) = global.addr_space {
            line.push_str(&format!("addrspace({}) ", addr));
        }
        if global.is_constant {
            line.push_str("constant ");
        } else {
            line.push_str("global ");
        }
        line.push_str(&format!("{} ", global.ty));
        match &global.initializer {
            Some(val) => line.push_str(&val.to_string()),
            None => {
                if global.is_constant {
                    line.push_str("undef");
                } else {
                    line.push_str("zeroinitializer");
                }
            }
        }
        if let Some(sec) = &global.section {
            line.push_str(&format!(", section \"{}\"", sec));
        }
        if let Some(align) = global.align {
            line.push_str(&format!(", align {}", align));
        }
        line.push('\n');
        line
    }
    pub(super) fn emit_function(&self, func: &LlvmIrFunction) -> String {
        let mut out = String::new();
        let params_str = func
            .params
            .iter()
            .map(|p| {
                let mut s = p.ty.to_string();
                for attr in &p.attrs {
                    s.push(' ');
                    s.push_str(attr);
                }
                s.push_str(&format!(" %{}", p.name));
                s
            })
            .collect::<Vec<_>>()
            .join(", ");
        let variadic_str = if func.variadic {
            if func.params.is_empty() {
                "..."
            } else {
                ", ..."
            }
        } else {
            ""
        };
        let attrs_str = if func.attributes.is_empty() {
            String::new()
        } else {
            format!(" #{}", func.attributes.join(" #"))
        };
        let keyword = if func.is_declaration {
            "declare"
        } else {
            "define"
        };
        if func.is_declaration {
            out.push_str(&format!(
                "declare {}{}{} @{}({}{}){}",
                func.linkage, func.cc, func.ret_ty, func.name, params_str, variadic_str, attrs_str
            ));
        } else {
            out.push_str(&format!(
                "{} {}{}{} @{}({}{}){} {{",
                keyword,
                func.linkage,
                func.cc,
                func.ret_ty,
                func.name,
                params_str,
                variadic_str,
                attrs_str
            ));
        }
        if func.is_declaration {
            if let Some(ref sec) = func.section {
                out.push_str(&format!(" section \"{}\"", sec));
            }
            out.push('\n');
            return out;
        }
        if let Some(ref sec) = func.section {
            out.push_str(&format!(" section \"{}\"", sec));
        }
        if let Some(align) = func.align {
            out.push_str(&format!(" align {}", align));
        }
        if let Some(ref gc) = func.gc {
            out.push_str(&format!(" gc \"{}\"", gc));
        }
        out.push('\n');
        for block in &func.blocks {
            out.push_str(&self.emit_block(block));
        }
        out.push_str("}\n");
        out
    }
    pub(super) fn emit_block(&self, block: &LlvmIrBlock) -> String {
        let mut out = String::new();
        out.push_str(&format!("{}:\n", block.label));
        for instr in &block.instructions {
            out.push_str(&format!("{}{}\n", self.indent, self.emit_instr(instr)));
        }
        out.push_str(&format!(
            "{}{}\n",
            self.indent,
            self.emit_instr(&block.terminator)
        ));
        out
    }
    /// Emit a single instruction as an LLVM IR text line.
    pub fn emit_instr(&self, instr: &LlvmIrInstr) -> String {
        match instr {
            LlvmIrInstr::Alloca { dest, ty, align } => {
                let mut s = format!("%{} = alloca {}", dest, ty);
                if let Some(a) = align {
                    s.push_str(&format!(", align {}", a));
                }
                s
            }
            LlvmIrInstr::Load {
                dest,
                ty,
                ptr,
                align,
                volatile,
            } => {
                let vol = if *volatile { "volatile " } else { "" };
                let mut s = format!("%{} = {}load {}, ptr {}", dest, vol, ty, ptr);
                if let Some(a) = align {
                    s.push_str(&format!(", align {}", a));
                }
                s
            }
            LlvmIrInstr::Store {
                ty,
                val,
                ptr,
                align,
                volatile,
            } => {
                let vol = if *volatile { "volatile " } else { "" };
                let mut s = format!("{}store {} {}, ptr {}", vol, ty, val, ptr);
                if let Some(a) = align {
                    s.push_str(&format!(", align {}", a));
                }
                s
            }
            LlvmIrInstr::Gep {
                dest,
                inbounds,
                elem_ty,
                ptr,
                indices,
            } => {
                let ib = if *inbounds { " inbounds" } else { "" };
                let mut s = format!("%{} = getelementptr{} {}, ptr {}", dest, ib, elem_ty, ptr);
                for (idx_ty, idx_val) in indices {
                    s.push_str(&format!(", {} {}", idx_ty, idx_val));
                }
                s
            }
            LlvmIrInstr::Add {
                dest,
                ty,
                lhs,
                rhs,
                nsw,
                nuw,
            } => {
                let flags = Self::wrapping_flags(*nsw, *nuw);
                format!("%{} = add{} {} {}, {}", dest, flags, ty, lhs, rhs)
            }
            LlvmIrInstr::Sub {
                dest,
                ty,
                lhs,
                rhs,
                nsw,
                nuw,
            } => {
                let flags = Self::wrapping_flags(*nsw, *nuw);
                format!("%{} = sub{} {} {}, {}", dest, flags, ty, lhs, rhs)
            }
            LlvmIrInstr::Mul {
                dest,
                ty,
                lhs,
                rhs,
                nsw,
                nuw,
            } => {
                let flags = Self::wrapping_flags(*nsw, *nuw);
                format!("%{} = mul{} {} {}, {}", dest, flags, ty, lhs, rhs)
            }
            LlvmIrInstr::Sdiv {
                dest,
                ty,
                lhs,
                rhs,
                exact,
            } => {
                let e = if *exact { " exact" } else { "" };
                format!("%{} = sdiv{} {} {}, {}", dest, e, ty, lhs, rhs)
            }
            LlvmIrInstr::Udiv {
                dest,
                ty,
                lhs,
                rhs,
                exact,
            } => {
                let e = if *exact { " exact" } else { "" };
                format!("%{} = udiv{} {} {}, {}", dest, e, ty, lhs, rhs)
            }
            LlvmIrInstr::Srem { dest, ty, lhs, rhs } => {
                format!("%{} = srem {} {}, {}", dest, ty, lhs, rhs)
            }
            LlvmIrInstr::Urem { dest, ty, lhs, rhs } => {
                format!("%{} = urem {} {}, {}", dest, ty, lhs, rhs)
            }
            LlvmIrInstr::And { dest, ty, lhs, rhs } => {
                format!("%{} = and {} {}, {}", dest, ty, lhs, rhs)
            }
            LlvmIrInstr::Or { dest, ty, lhs, rhs } => {
                format!("%{} = or {} {}, {}", dest, ty, lhs, rhs)
            }
            LlvmIrInstr::Xor { dest, ty, lhs, rhs } => {
                format!("%{} = xor {} {}, {}", dest, ty, lhs, rhs)
            }
            LlvmIrInstr::Shl { dest, ty, lhs, rhs } => {
                format!("%{} = shl {} {}, {}", dest, ty, lhs, rhs)
            }
            LlvmIrInstr::Lshr { dest, ty, lhs, rhs } => {
                format!("%{} = lshr {} {}, {}", dest, ty, lhs, rhs)
            }
            LlvmIrInstr::Ashr { dest, ty, lhs, rhs } => {
                format!("%{} = ashr {} {}, {}", dest, ty, lhs, rhs)
            }
            LlvmIrInstr::Fadd {
                dest,
                ty,
                lhs,
                rhs,
                fast,
            } => {
                let f = if *fast { " fast" } else { "" };
                format!("%{} = fadd{} {} {}, {}", dest, f, ty, lhs, rhs)
            }
            LlvmIrInstr::Fsub {
                dest,
                ty,
                lhs,
                rhs,
                fast,
            } => {
                let f = if *fast { " fast" } else { "" };
                format!("%{} = fsub{} {} {}, {}", dest, f, ty, lhs, rhs)
            }
            LlvmIrInstr::Fmul {
                dest,
                ty,
                lhs,
                rhs,
                fast,
            } => {
                let f = if *fast { " fast" } else { "" };
                format!("%{} = fmul{} {} {}, {}", dest, f, ty, lhs, rhs)
            }
            LlvmIrInstr::Fdiv {
                dest,
                ty,
                lhs,
                rhs,
                fast,
            } => {
                let f = if *fast { " fast" } else { "" };
                format!("%{} = fdiv{} {} {}, {}", dest, f, ty, lhs, rhs)
            }
            LlvmIrInstr::Fneg {
                dest,
                ty,
                val,
                fast,
            } => {
                let f = if *fast { " fast" } else { "" };
                format!("%{} = fneg{} {} {}", dest, f, ty, val)
            }
            LlvmIrInstr::Icmp {
                dest,
                pred,
                ty,
                lhs,
                rhs,
            } => {
                format!("%{} = icmp {} {} {}, {}", dest, pred, ty, lhs, rhs)
            }
            LlvmIrInstr::Fcmp {
                dest,
                pred,
                ty,
                lhs,
                rhs,
                fast,
            } => {
                let f = if *fast { " fast" } else { "" };
                format!("%{} = fcmp{} {} {} {}, {}", dest, f, pred, ty, lhs, rhs)
            }
            LlvmIrInstr::Trunc {
                dest,
                val,
                from_ty,
                to_ty,
            } => {
                format!("%{} = trunc {} {} to {}", dest, from_ty, val, to_ty)
            }
            LlvmIrInstr::Zext {
                dest,
                val,
                from_ty,
                to_ty,
            } => {
                format!("%{} = zext {} {} to {}", dest, from_ty, val, to_ty)
            }
            LlvmIrInstr::Sext {
                dest,
                val,
                from_ty,
                to_ty,
            } => {
                format!("%{} = sext {} {} to {}", dest, from_ty, val, to_ty)
            }
            LlvmIrInstr::Fptrunc {
                dest,
                val,
                from_ty,
                to_ty,
            } => {
                format!("%{} = fptrunc {} {} to {}", dest, from_ty, val, to_ty)
            }
            LlvmIrInstr::Fpext {
                dest,
                val,
                from_ty,
                to_ty,
            } => {
                format!("%{} = fpext {} {} to {}", dest, from_ty, val, to_ty)
            }
            LlvmIrInstr::Fptoui {
                dest,
                val,
                from_ty,
                to_ty,
            } => {
                format!("%{} = fptoui {} {} to {}", dest, from_ty, val, to_ty)
            }
            LlvmIrInstr::Fptosi {
                dest,
                val,
                from_ty,
                to_ty,
            } => {
                format!("%{} = fptosi {} {} to {}", dest, from_ty, val, to_ty)
            }
            LlvmIrInstr::Uitofp {
                dest,
                val,
                from_ty,
                to_ty,
            } => {
                format!("%{} = uitofp {} {} to {}", dest, from_ty, val, to_ty)
            }
            LlvmIrInstr::Sitofp {
                dest,
                val,
                from_ty,
                to_ty,
            } => {
                format!("%{} = sitofp {} {} to {}", dest, from_ty, val, to_ty)
            }
            LlvmIrInstr::Ptrtoint {
                dest,
                val,
                from_ty,
                to_ty,
            } => {
                format!("%{} = ptrtoint {} {} to {}", dest, from_ty, val, to_ty)
            }
            LlvmIrInstr::Inttoptr {
                dest,
                val,
                from_ty,
                to_ty,
            } => {
                format!("%{} = inttoptr {} {} to {}", dest, from_ty, val, to_ty)
            }
            LlvmIrInstr::Bitcast {
                dest,
                val,
                from_ty,
                to_ty,
            } => {
                format!("%{} = bitcast {} {} to {}", dest, from_ty, val, to_ty)
            }
            LlvmIrInstr::BrUnconditional { dest } => format!("br label %{}", dest),
            LlvmIrInstr::BrConditional {
                cond,
                true_dest,
                false_dest,
            } => {
                format!(
                    "br i1 {}, label %{}, label %{}",
                    cond, true_dest, false_dest
                )
            }
            LlvmIrInstr::RetVoid => "ret void".to_string(),
            LlvmIrInstr::Ret { ty, val } => format!("ret {} {}", ty, val),
            LlvmIrInstr::Unreachable => "unreachable".to_string(),
            LlvmIrInstr::Switch {
                ty,
                val,
                default,
                cases,
            } => {
                let mut s = format!("switch {} {}, label %{} [\n", ty, val, default);
                for (case_val, case_dest) in cases {
                    s.push_str(&format!("    {} {}, label %{}\n", ty, case_val, case_dest));
                }
                s.push(']');
                s
            }
            LlvmIrInstr::Call {
                dest,
                ret_ty,
                func,
                args,
                tail,
                cc,
            } => {
                let tail_str = if *tail { "tail " } else { "" };
                let args_str = args
                    .iter()
                    .map(|(t, v)| format!("{} {}", t, v))
                    .collect::<Vec<_>>()
                    .join(", ");
                let dest_str = match dest {
                    Some(d) => format!("%{} = ", d),
                    None => String::new(),
                };
                format!(
                    "{}{}call {}{} {}({})",
                    dest_str, tail_str, cc, ret_ty, func, args_str
                )
            }
            LlvmIrInstr::Invoke {
                dest,
                ret_ty,
                func,
                args,
                normal,
                unwind,
            } => {
                let args_str = args
                    .iter()
                    .map(|(t, v)| format!("{} {}", t, v))
                    .collect::<Vec<_>>()
                    .join(", ");
                let dest_str = match dest {
                    Some(d) => format!("%{} = ", d),
                    None => String::new(),
                };
                format!(
                    "{}invoke {} {}({}) to label %{} unwind label %{}",
                    dest_str, ret_ty, func, args_str, normal, unwind
                )
            }
            LlvmIrInstr::Phi { dest, ty, incoming } => {
                let inc_str = incoming
                    .iter()
                    .map(|(v, bb)| format!("[ {}, %{} ]", v, bb))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("%{} = phi {} {}", dest, ty, inc_str)
            }
            LlvmIrInstr::Select {
                dest,
                cond,
                ty,
                true_val,
                false_val,
            } => {
                format!(
                    "%{} = select i1 {}, {} {}, {} {}",
                    dest, cond, ty, true_val, ty, false_val
                )
            }
            LlvmIrInstr::ExtractElement {
                dest,
                vec_ty,
                vec,
                idx,
            } => {
                format!("%{} = extractelement {} {}, i32 {}", dest, vec_ty, vec, idx)
            }
            LlvmIrInstr::InsertElement {
                dest,
                vec_ty,
                vec,
                ty,
                val,
                idx,
            } => {
                format!(
                    "%{} = insertelement {} {}, {} {}, i32 {}",
                    dest, vec_ty, vec, ty, val, idx
                )
            }
            LlvmIrInstr::ExtractValue {
                dest,
                agg_ty,
                agg,
                indices,
            } => {
                let idx_str = indices
                    .iter()
                    .map(|i| i.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("%{} = extractvalue {} {}, {}", dest, agg_ty, agg, idx_str)
            }
            LlvmIrInstr::InsertValue {
                dest,
                agg_ty,
                agg,
                elem_ty,
                val,
                indices,
            } => {
                let idx_str = indices
                    .iter()
                    .map(|i| i.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");
                format!(
                    "%{} = insertvalue {} {}, {} {}, {}",
                    dest, agg_ty, agg, elem_ty, val, idx_str
                )
            }
            LlvmIrInstr::Raw(text) => text.clone(),
        }
    }
    pub(super) fn wrapping_flags(nsw: bool, nuw: bool) -> &'static str {
        match (nsw, nuw) {
            (true, true) => " nsw nuw",
            (true, false) => " nsw",
            (false, true) => " nuw",
            (false, false) => "",
        }
    }
}
/// Generates fresh SSA register names for LLVM IR emission.
///
/// Names are of the form `_r0`, `_r1`, `_r2`, ...
#[derive(Debug, Default)]
pub struct RegisterAllocator {
    pub(super) counter: u32,
}
impl RegisterAllocator {
    /// Create a new allocator.
    pub fn new() -> Self {
        Self::default()
    }
    /// Allocate the next register name.
    pub fn next_reg(&mut self) -> String {
        let name = format!("_r{}", self.counter);
        self.counter += 1;
        name
    }
    /// Allocate a named register (adds a disambiguating suffix if needed).
    pub fn named(&mut self, prefix: &str) -> String {
        let name = format!("{}_{}", prefix, self.counter);
        self.counter += 1;
        name
    }
    /// Reset the counter.
    pub fn reset(&mut self) {
        self.counter = 0;
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct LITPassStats {
    pub total_runs: u32,
    pub successful_runs: u32,
    pub total_changes: u64,
    pub time_ms: u64,
    pub iterations_used: u32,
}
impl LITPassStats {
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
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LITAnalysisCache {
    pub(super) entries: std::collections::HashMap<String, LITCacheEntry>,
    pub(super) max_size: usize,
    pub(super) hits: u64,
    pub(super) misses: u64,
}
impl LITAnalysisCache {
    #[allow(dead_code)]
    pub fn new(max_size: usize) -> Self {
        LITAnalysisCache {
            entries: std::collections::HashMap::new(),
            max_size,
            hits: 0,
            misses: 0,
        }
    }
    #[allow(dead_code)]
    pub fn get(&mut self, key: &str) -> Option<&LITCacheEntry> {
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
            LITCacheEntry {
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
pub struct LITWorklist {
    pub(super) items: std::collections::VecDeque<u32>,
    pub(super) in_worklist: std::collections::HashSet<u32>,
}
impl LITWorklist {
    #[allow(dead_code)]
    pub fn new() -> Self {
        LITWorklist {
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
/// LLVM IR value representation used as instruction operands.
#[derive(Debug, Clone, PartialEq)]
pub enum LlvmIrValue {
    /// Integer constant: `42`, `-1`, `true` (for i1), etc.
    ConstInt(i64),
    /// Large unsigned integer constant (for i64/i128 unsigned range).
    ConstUint(u64),
    /// Floating-point constant: `1.0`, `0.0`, etc.
    ConstFloat(f64),
    /// Null pointer constant: `null`.
    ConstNull,
    /// Zero-initializer: `zeroinitializer`.
    ZeroInitializer,
    /// Undefined value: `undef`.
    Undef,
    /// Poison value: `poison`.
    Poison,
    /// Local register: `%name`.
    Register(String),
    /// Global reference: `@name`.
    Global(String),
    /// Constant GEP expression.
    ConstGep {
        /// Element type for the GEP.
        ty: Box<LlvmIrType>,
        /// Base pointer.
        base: Box<LlvmIrValue>,
        /// Index list (type-index pairs).
        indices: Vec<(LlvmIrType, LlvmIrValue)>,
    },
    /// Constant bitcast expression.
    ConstBitcast {
        /// Value to cast.
        val: Box<LlvmIrValue>,
        /// Source type.
        from_ty: Box<LlvmIrType>,
        /// Target type.
        to_ty: Box<LlvmIrType>,
    },
    /// Inline aggregate/array constant: `[i32 1, i32 2]`.
    ConstArray {
        /// Element type.
        elem_ty: Box<LlvmIrType>,
        /// Elements.
        elems: Vec<LlvmIrValue>,
    },
    /// Inline struct constant: `{ i32 1, i8 0 }`.
    ConstStruct {
        /// Field (type, value) pairs.
        fields: Vec<(LlvmIrType, LlvmIrValue)>,
    },
}
/// An LLVM IR function definition or declaration.
#[derive(Debug, Clone)]
pub struct LlvmIrFunction {
    /// Function name (without `@` prefix).
    pub name: String,
    /// Linkage type.
    pub linkage: Linkage,
    /// Calling convention.
    pub cc: CallingConv,
    /// Return type.
    pub ret_ty: LlvmIrType,
    /// Parameters.
    pub params: Vec<LlvmIrParam>,
    /// Whether the function is variadic.
    pub variadic: bool,
    /// Function body blocks (empty = declaration only).
    pub blocks: Vec<LlvmIrBlock>,
    /// Function-level attributes (e.g. `nounwind`, `readnone`, `alwaysinline`).
    pub attributes: Vec<String>,
    /// Whether this is just a declaration (`declare`) vs. definition (`define`).
    pub is_declaration: bool,
    /// Optional section name.
    pub section: Option<String>,
    /// Optional alignment.
    pub align: Option<u32>,
    /// Optional GC strategy name.
    pub gc: Option<String>,
}
impl LlvmIrFunction {
    /// Create a function definition.
    pub fn new(name: impl Into<String>, ret_ty: LlvmIrType, params: Vec<LlvmIrParam>) -> Self {
        Self {
            name: name.into(),
            linkage: Linkage::External,
            cc: CallingConv::C,
            ret_ty,
            params,
            variadic: false,
            blocks: Vec::new(),
            attributes: Vec::new(),
            is_declaration: false,
            section: None,
            align: None,
            gc: None,
        }
    }
    /// Create a function declaration (no body).
    pub fn declare(name: impl Into<String>, ret_ty: LlvmIrType, params: Vec<LlvmIrParam>) -> Self {
        let mut f = Self::new(name, ret_ty, params);
        f.is_declaration = true;
        f
    }
    /// Add a basic block to this function.
    pub fn add_block(&mut self, block: LlvmIrBlock) {
        self.blocks.push(block);
    }
    /// Add a function attribute.
    pub fn add_attr(&mut self, attr: impl Into<String>) {
        self.attributes.push(attr.into());
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LITDominatorTree {
    pub idom: Vec<Option<u32>>,
    pub dom_children: Vec<Vec<u32>>,
    pub dom_depth: Vec<u32>,
}
impl LITDominatorTree {
    #[allow(dead_code)]
    pub fn new(size: usize) -> Self {
        LITDominatorTree {
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
#[allow(dead_code)]
pub struct LITConstantFoldingHelper;
impl LITConstantFoldingHelper {
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
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LITPassConfig {
    pub phase: LITPassPhase,
    pub enabled: bool,
    pub max_iterations: u32,
    pub debug_output: bool,
    pub pass_name: String,
}
impl LITPassConfig {
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>, phase: LITPassPhase) -> Self {
        LITPassConfig {
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
/// Integer comparison predicates for `icmp` instruction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IcmpPred {
    /// Equal.
    Eq,
    /// Not equal.
    Ne,
    /// Unsigned greater than.
    Ugt,
    /// Unsigned greater or equal.
    Uge,
    /// Unsigned less than.
    Ult,
    /// Unsigned less or equal.
    Ule,
    /// Signed greater than.
    Sgt,
    /// Signed greater or equal.
    Sge,
    /// Signed less than.
    Slt,
    /// Signed less or equal.
    Sle,
}
