//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::HashMap;
use std::collections::{HashSet, VecDeque};

use super::types_2::{IcmpPred, LITPassConfig, LITPassStats, Linkage, LlvmIrType};

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
