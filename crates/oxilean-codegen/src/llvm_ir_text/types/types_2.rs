//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::HashMap;
use std::collections::{HashSet, VecDeque};

use super::types::{LITPassPhase, LlvmIrBlock, LlvmIrFunction, LlvmIrGlobal, LlvmIrInstr};

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
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LITAnalysisCache {
    pub(crate) entries: std::collections::HashMap<String, LITCacheEntry>,
    pub(crate) max_size: usize,
    pub(crate) hits: u64,
    pub(crate) misses: u64,
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
pub struct LITCacheEntry {
    pub key: String,
    pub data: Vec<u8>,
    pub timestamp: u64,
    pub valid: bool,
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
