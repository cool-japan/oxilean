//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;
use std::collections::HashMap;

use super::functions::*;
use std::collections::{HashSet, VecDeque};

/// A reference to a .NET field.
#[derive(Debug, Clone)]
pub struct CilFieldRef {
    pub field_type: CilType,
    pub declaring_type: CilType,
    pub name: std::string::String,
}
/// CIL instruction set.
///
/// Covers the major opcodes from ECMA-335 Partition III.
#[derive(Debug, Clone)]
pub enum CilInstr {
    /// `nop` — no operation
    Nop,
    /// `ldloc.s <idx>` — load local variable
    LdlocS(u16),
    /// `stloc.s <idx>` — store local variable
    StlocS(u16),
    /// `ldloca.s <idx>` — load address of local
    LdlocaS(u16),
    /// `ldarg.s <idx>` — load argument
    LdargS(u16),
    /// `starg.s <idx>` — store argument
    StargS(u16),
    /// `ldarga.s <idx>` — load argument address
    LdargaS(u16),
    /// `ldc.i4 <val>` — load int32 constant
    LdcI4(i32),
    /// `ldc.i4.s <val>` — load int32 constant (short form, -128..127)
    LdcI4S(i8),
    /// `ldc.i4.N` — load small int32 (N = 0..8)
    LdcI4Small(i32),
    /// `ldc.i8 <val>` — load int64 constant
    LdcI8(i64),
    /// `ldc.r4 <val>` — load float32 constant
    LdcR4(f32),
    /// `ldc.r8 <val>` — load float64 constant
    LdcR8(f64),
    /// `ldnull` — load null reference
    Ldnull,
    /// `ldstr <s>` — load string literal
    Ldstr(std::string::String),
    /// `ldsflda <field>` — load address of static field
    Ldsflda(CilFieldRef),
    /// `ldsfld <field>` — load static field value
    Ldsfld(CilFieldRef),
    /// `stsfld <field>` — store static field value
    Stsfld(CilFieldRef),
    /// `add`
    Add,
    /// `add.ovf` — checked addition
    AddOvf,
    /// `sub`
    Sub,
    /// `sub.ovf` — checked subtraction
    SubOvf,
    /// `mul`
    Mul,
    /// `mul.ovf` — checked multiplication
    MulOvf,
    /// `div`
    Div,
    /// `div.un` — unsigned division
    DivUn,
    /// `rem`
    Rem,
    /// `rem.un` — unsigned remainder
    RemUn,
    /// `neg` — negate
    Neg,
    /// `and`
    And,
    /// `or`
    Or,
    /// `xor`
    Xor,
    /// `not`
    Not,
    /// `shl` — shift left
    Shl,
    /// `shr` — shift right (signed)
    Shr,
    /// `shr.un` — shift right (unsigned)
    ShrUn,
    /// `ceq` — compare equal (push 1 or 0)
    Ceq,
    /// `cgt`
    Cgt,
    /// `cgt.un`
    CgtUn,
    /// `clt`
    Clt,
    /// `clt.un`
    CltUn,
    /// `br <label>`
    Br(std::string::String),
    /// `brfalse <label>`
    Brfalse(std::string::String),
    /// `brtrue <label>`
    Brtrue(std::string::String),
    /// `beq <label>`
    Beq(std::string::String),
    /// `bne.un <label>`
    BneUn(std::string::String),
    /// `blt <label>`
    Blt(std::string::String),
    /// `bgt <label>`
    Bgt(std::string::String),
    /// `ble <label>`
    Ble(std::string::String),
    /// `bge <label>`
    Bge(std::string::String),
    /// `switch <labels...>`
    Switch(Vec<std::string::String>),
    /// `ret`
    Ret,
    /// `throw`
    Throw,
    /// `rethrow`
    Rethrow,
    /// A label definition point
    Label(std::string::String),
    /// `call <method>`
    Call(CilMethodRef),
    /// `callvirt <method>`
    Callvirt(CilMethodRef),
    /// `tail. call <method>`
    TailCall(CilMethodRef),
    /// `calli <signature>`
    Calli(CilCallSig),
    /// `ldftn <method>`
    Ldftn(CilMethodRef),
    /// `ldvirtftn <method>`
    Ldvirtftn(CilMethodRef),
    /// `newobj <ctor>`
    Newobj(CilMethodRef),
    /// `ldobj <type>`
    Ldobj(CilType),
    /// `stobj <type>`
    Stobj(CilType),
    /// `ldfld <field>`
    Ldfld(CilFieldRef),
    /// `stfld <field>`
    Stfld(CilFieldRef),
    /// `ldflda <field>`
    Ldflda(CilFieldRef),
    /// `box <type>`
    Box_(CilType),
    /// `unbox <type>`
    Unbox(CilType),
    /// `unbox.any <type>`
    UnboxAny(CilType),
    /// `isinst <type>`
    Isinst(CilType),
    /// `castclass <type>`
    Castclass(CilType),
    /// `initobj <type>`
    Initobj(CilType),
    /// `sizeof <type>`
    Sizeof(CilType),
    /// `ldtoken <type>`
    Ldtoken(CilType),
    /// `newarr <type>`
    Newarr(CilType),
    /// `ldlen`
    Ldlen,
    /// `ldelem <type>`
    Ldelem(CilType),
    /// `stelem <type>`
    Stelem(CilType),
    /// `ldelema <type>`
    Ldelema(CilType),
    /// `dup`
    Dup,
    /// `pop`
    Pop,
    /// `conv.i4`
    ConvI4,
    /// `conv.i8`
    ConvI8,
    /// `conv.r4`
    ConvR4,
    /// `conv.r8`
    ConvR8,
    /// `conv.u4`
    ConvU4,
    /// `conv.u8`
    ConvU8,
    /// `ldind.i4`
    LdindI4,
    /// `stind.i4`
    StindI4,
    /// `localloc`
    Localloc,
    /// Comment (for IL dump readability)
    Comment(std::string::String),
}
#[allow(dead_code)]
pub struct CILPassRegistry {
    pub(super) configs: Vec<CILPassConfig>,
    pub(super) stats: std::collections::HashMap<String, CILPassStats>,
}
impl CILPassRegistry {
    #[allow(dead_code)]
    pub fn new() -> Self {
        CILPassRegistry {
            configs: Vec::new(),
            stats: std::collections::HashMap::new(),
        }
    }
    #[allow(dead_code)]
    pub fn register(&mut self, config: CILPassConfig) {
        self.stats
            .insert(config.pass_name.clone(), CILPassStats::new());
        self.configs.push(config);
    }
    #[allow(dead_code)]
    pub fn enabled_passes(&self) -> Vec<&CILPassConfig> {
        self.configs.iter().filter(|c| c.enabled).collect()
    }
    #[allow(dead_code)]
    pub fn get_stats(&self, name: &str) -> Option<&CILPassStats> {
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
/// Tracks declared names for CilExt scope analysis.
#[derive(Debug, Default)]
pub struct CilExtNameScope {
    pub(super) declared: std::collections::HashSet<String>,
    pub(super) depth: usize,
    pub(super) parent: Option<Box<CilExtNameScope>>,
}
impl CilExtNameScope {
    pub fn new() -> Self {
        CilExtNameScope::default()
    }
    pub fn declare(&mut self, name: impl Into<String>) -> bool {
        self.declared.insert(name.into())
    }
    pub fn is_declared(&self, name: &str) -> bool {
        self.declared.contains(name)
    }
    pub fn push_scope(self) -> Self {
        CilExtNameScope {
            declared: std::collections::HashSet::new(),
            depth: self.depth + 1,
            parent: Some(Box::new(self)),
        }
    }
    pub fn pop_scope(self) -> Self {
        *self.parent.unwrap_or_default()
    }
    pub fn depth(&self) -> usize {
        self.depth
    }
    pub fn len(&self) -> usize {
        self.declared.len()
    }
}
/// A fixed-capacity ring buffer of strings (for recent-event logging in CilExt).
#[derive(Debug)]
pub struct CilExtEventLog {
    pub(super) entries: std::collections::VecDeque<String>,
    pub(super) capacity: usize,
}
impl CilExtEventLog {
    pub fn new(capacity: usize) -> Self {
        CilExtEventLog {
            entries: std::collections::VecDeque::with_capacity(capacity),
            capacity,
        }
    }
    pub fn push(&mut self, event: impl Into<String>) {
        if self.entries.len() >= self.capacity {
            self.entries.pop_front();
        }
        self.entries.push_back(event.into());
    }
    pub fn iter(&self) -> impl Iterator<Item = &String> {
        self.entries.iter()
    }
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
    pub fn capacity(&self) -> usize {
        self.capacity
    }
    pub fn clear(&mut self) {
        self.entries.clear();
    }
}
/// A monotonically increasing ID generator for CilExt.
#[derive(Debug, Default)]
pub struct CilExtIdGen {
    pub(super) next: u32,
}
impl CilExtIdGen {
    pub fn new() -> Self {
        CilExtIdGen::default()
    }
    pub fn next_id(&mut self) -> u32 {
        let id = self.next;
        self.next += 1;
        id
    }
    pub fn peek_next(&self) -> u32 {
        self.next
    }
    pub fn reset(&mut self) {
        self.next = 0;
    }
    pub fn skip(&mut self, n: u32) {
        self.next += n;
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum CILPassPhase {
    Analysis,
    Transformation,
    Verification,
    Cleanup,
}
impl CILPassPhase {
    #[allow(dead_code)]
    pub fn name(&self) -> &str {
        match self {
            CILPassPhase::Analysis => "analysis",
            CILPassPhase::Transformation => "transformation",
            CILPassPhase::Verification => "verification",
            CILPassPhase::Cleanup => "cleanup",
        }
    }
    #[allow(dead_code)]
    pub fn is_modifying(&self) -> bool {
        matches!(self, CILPassPhase::Transformation | CILPassPhase::Cleanup)
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CILDepGraph {
    pub(super) nodes: Vec<u32>,
    pub(super) edges: Vec<(u32, u32)>,
}
impl CILDepGraph {
    #[allow(dead_code)]
    pub fn new() -> Self {
        CILDepGraph {
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
/// A compile-time literal value (used in field initializers).
#[derive(Debug, Clone)]
pub enum CilLiteral {
    Bool(bool),
    Int32(i32),
    Int64(i64),
    Float32(f32),
    Float64(f64),
    String(std::string::String),
    Null,
}
/// CIL code generation backend.
///
/// Converts LCNF IR into .NET CIL IL assembly (suitable for `ilasm`).
pub struct CilBackend {
    pub assembly: CilAssembly,
    pub(super) label_counter: u32,
    pub(super) var_locals: HashMap<u64, u16>,
    pub default_namespace: std::string::String,
}
impl CilBackend {
    /// Create a new CIL backend.
    pub fn new(assembly_name: impl Into<std::string::String>) -> Self {
        CilBackend {
            assembly: CilAssembly::new(assembly_name),
            label_counter: 0,
            var_locals: HashMap::new(),
            default_namespace: "OxiLean.Generated".to_string(),
        }
    }
    /// Generate a fresh IL label.
    pub(super) fn fresh_label(&mut self) -> std::string::String {
        let n = self.label_counter;
        self.label_counter += 1;
        format!("IL_{:04X}", n)
    }
    /// Map an LCNF type to a CIL type.
    pub fn lcnf_to_cil_type(&self, ty: &LcnfType) -> CilType {
        match ty {
            LcnfType::Erased | LcnfType::Object | LcnfType::Irrelevant => CilType::Object,
            LcnfType::Nat => CilType::UInt64,
            LcnfType::LcnfString => CilType::String,
            LcnfType::Unit => CilType::Void,
            LcnfType::Var(name) => match name.as_str() {
                "Int32" | "int32" => CilType::Int32,
                "Int64" | "int64" | "Int" => CilType::Int64,
                "UInt32" | "uint32" => CilType::UInt32,
                "Float" | "Float32" | "float32" => CilType::Float32,
                "Float64" | "float64" | "Double" => CilType::Float64,
                "Bool" | "bool" => CilType::Bool,
                "String" | "string" => CilType::String,
                "Unit" | "void" => CilType::Void,
                "Char" | "char" => CilType::Char,
                _ => CilType::class_in(&self.default_namespace, name.clone()),
            },
            LcnfType::Fun(params, ret) => {
                if params.len() == 1 {
                    CilType::func_of(
                        self.lcnf_to_cil_type(&params[0]),
                        self.lcnf_to_cil_type(ret),
                    )
                } else {
                    CilType::class_in("System", "Delegate")
                }
            }
            LcnfType::Ctor(name, _) => CilType::class_in(&self.default_namespace, name.clone()),
        }
    }
    /// Emit CIL instructions for an LCNF literal.
    pub fn emit_literal(&self, method: &mut CilMethod, lit: &LcnfLit) {
        match lit {
            LcnfLit::Nat(n) => method.emit(CilInstr::LdcI8(*n as i64)),
            LcnfLit::Str(s) => method.emit(CilInstr::Ldstr(s.clone())),
        }
    }
    /// Emit CIL instructions for an LCNF argument.
    pub fn emit_arg(&mut self, method: &mut CilMethod, arg: &LcnfArg) {
        match arg {
            LcnfArg::Var(id) => {
                if let Some(&local_idx) = self.var_locals.get(&id.0) {
                    method.emit(CilInstr::LdlocS(local_idx));
                } else {
                    method.emit(CilInstr::LdargS(0));
                }
            }
            LcnfArg::Lit(lit) => self.emit_literal(method, lit),
            LcnfArg::Erased | LcnfArg::Type(_) => method.emit(CilInstr::Ldnull),
        }
    }
    /// Emit CIL instructions for an LCNF let-value.
    pub fn emit_let_value(&mut self, method: &mut CilMethod, val: &LcnfLetValue) {
        match val {
            LcnfLetValue::App(func, args) => {
                for arg in args.iter() {
                    self.emit_arg(method, arg);
                }
                self.emit_arg(method, func);
                let invoke_ref = CilMethodRef {
                    call_conv: CilCallConv::Instance,
                    return_type: CilType::Object,
                    declaring_type: CilType::class_in("System", "Delegate"),
                    name: "DynamicInvoke".to_string(),
                    param_types: vec![CilType::Array(Box::new(CilType::Object))],
                };
                method.emit(CilInstr::Callvirt(invoke_ref));
            }
            LcnfLetValue::Proj(_struct_name, idx, var_id) => {
                if let Some(&local_idx) = self.var_locals.get(&var_id.0) {
                    method.emit(CilInstr::LdlocS(local_idx));
                } else {
                    method.emit(CilInstr::LdargS(0));
                }
                let field_ref = CilFieldRef {
                    field_type: CilType::Object,
                    declaring_type: CilType::Object,
                    name: format!("_field{}", idx),
                };
                method.emit(CilInstr::Ldfld(field_ref));
            }
            LcnfLetValue::Ctor(name, _tag, args) => {
                let ctor_type = CilType::class_in(self.default_namespace.clone(), name.clone());
                let param_types: Vec<CilType> = args.iter().map(|_| CilType::Object).collect();
                for arg in args.iter() {
                    self.emit_arg(method, arg);
                }
                method.emit(CilInstr::Newobj(CilMethodRef {
                    call_conv: CilCallConv::Instance,
                    return_type: CilType::Void,
                    declaring_type: ctor_type,
                    name: ".ctor".to_string(),
                    param_types,
                }));
            }
            LcnfLetValue::Lit(lit) => self.emit_literal(method, lit),
            LcnfLetValue::Erased => method.emit(CilInstr::Ldnull),
            LcnfLetValue::FVar(id) => {
                if let Some(&local_idx) = self.var_locals.get(&id.0) {
                    method.emit(CilInstr::LdlocS(local_idx));
                } else {
                    method.emit(CilInstr::LdargS(0));
                }
            }
            LcnfLetValue::Reset(var) => {
                if let Some(&local_idx) = self.var_locals.get(&var.0) {
                    method.emit(CilInstr::LdlocS(local_idx));
                } else {
                    method.emit(CilInstr::LdargS(0));
                }
                method.emit(CilInstr::Comment("reset (reuse optimization)".to_string()));
            }
            LcnfLetValue::Reuse(slot, name, _tag, args) => {
                let ctor_type = CilType::class_in(self.default_namespace.clone(), name.clone());
                let param_types: Vec<CilType> = args.iter().map(|_| CilType::Object).collect();
                if let Some(&local_idx) = self.var_locals.get(&slot.0) {
                    method.emit(CilInstr::LdlocS(local_idx));
                } else {
                    method.emit(CilInstr::LdargS(0));
                }
                for arg in args.iter() {
                    self.emit_arg(method, arg);
                }
                method.emit(CilInstr::Comment(format!("reuse -> {}", name)));
                method.emit(CilInstr::Newobj(CilMethodRef {
                    call_conv: CilCallConv::Instance,
                    return_type: CilType::Void,
                    declaring_type: ctor_type,
                    name: ".ctor".to_string(),
                    param_types,
                }));
            }
        }
    }
    /// Emit CIL instructions for an LCNF expression.
    #[allow(clippy::too_many_arguments)]
    pub fn emit_expr(&mut self, method: &mut CilMethod, expr: &LcnfExpr) {
        match expr {
            LcnfExpr::Let {
                id,
                ty,
                value,
                body,
                ..
            } => {
                self.emit_let_value(method, value);
                let cil_ty = self.lcnf_to_cil_type(ty);
                let local_idx = method.add_local(cil_ty, None);
                self.var_locals.insert(id.0, local_idx);
                method.emit(CilInstr::StlocS(local_idx));
                self.emit_expr(method, body);
            }
            LcnfExpr::Case {
                scrutinee,
                alts,
                default,
                ..
            } => {
                let end_label = self.fresh_label();
                for alt in alts.iter() {
                    let next_label = self.fresh_label();
                    if let Some(&local_idx) = self.var_locals.get(&scrutinee.0) {
                        method.emit(CilInstr::LdlocS(local_idx));
                    } else {
                        method.emit(CilInstr::LdargS(0));
                    }
                    let ctor_type =
                        CilType::class_in(self.default_namespace.clone(), alt.ctor_name.clone());
                    method.emit(CilInstr::Isinst(ctor_type.clone()));
                    method.emit(CilInstr::Dup);
                    method.emit(CilInstr::Brfalse(next_label.clone()));
                    for (i, param) in alt.params.iter().enumerate() {
                        method.emit(CilInstr::Dup);
                        let field_ref = CilFieldRef {
                            field_type: CilType::Object,
                            declaring_type: ctor_type.clone(),
                            name: format!("_field{}", i),
                        };
                        method.emit(CilInstr::Ldfld(field_ref));
                        let param_ty = self.lcnf_to_cil_type(&param.ty);
                        let local_idx = method.add_local(param_ty, Some(param.name.clone()));
                        self.var_locals.insert(param.id.0, local_idx);
                        method.emit(CilInstr::StlocS(local_idx));
                    }
                    method.emit(CilInstr::Pop);
                    let body = alt.body.clone();
                    self.emit_expr(method, &body);
                    method.emit(CilInstr::Br(end_label.clone()));
                    method.emit_label(next_label);
                    method.emit(CilInstr::Pop);
                }
                if let Some(def_body) = default {
                    let def_body = def_body.clone();
                    self.emit_expr(method, &def_body);
                } else {
                    method.emit(CilInstr::Ldstr("MatchFailure".to_string()));
                    method.emit(CilInstr::Newobj(CilMethodRef {
                        call_conv: CilCallConv::Instance,
                        return_type: CilType::Void,
                        declaring_type: CilType::class_in("System", "Exception"),
                        name: ".ctor".to_string(),
                        param_types: vec![CilType::String],
                    }));
                    method.emit(CilInstr::Throw);
                }
                method.emit_label(end_label);
            }
            LcnfExpr::Return(arg) => {
                self.emit_arg(method, arg);
            }
            LcnfExpr::Unreachable => {
                method.emit(CilInstr::Ldstr("unreachable".to_string()));
                method.emit(CilInstr::Newobj(CilMethodRef {
                    call_conv: CilCallConv::Instance,
                    return_type: CilType::Void,
                    declaring_type: CilType::class_in("System", "InvalidOperationException"),
                    name: ".ctor".to_string(),
                    param_types: vec![CilType::String],
                }));
                method.emit(CilInstr::Throw);
            }
            LcnfExpr::TailCall(func, args) => {
                for arg in args.iter() {
                    self.emit_arg(method, arg);
                }
                self.emit_arg(method, func);
                let invoke_ref = CilMethodRef {
                    call_conv: CilCallConv::Instance,
                    return_type: CilType::Object,
                    declaring_type: CilType::class_in("System", "Delegate"),
                    name: "DynamicInvoke".to_string(),
                    param_types: vec![CilType::Array(Box::new(CilType::Object))],
                };
                method.emit(CilInstr::TailCall(invoke_ref));
            }
        }
    }
    /// Emit a complete LCNF function declaration as a CIL method.
    pub fn emit_fun_decl(&mut self, decl: &LcnfFunDecl) -> CilMethod {
        let ret_ty = self.lcnf_to_cil_type(&decl.ret_type);
        let mut method = CilMethod::new_static(&decl.name, ret_ty);
        for param in &decl.params {
            let cil_ty = self.lcnf_to_cil_type(&param.ty);
            let idx = method.add_param(param.name.clone(), cil_ty);
            self.var_locals.insert(param.id.0, idx);
        }
        let body = decl.body.clone();
        self.emit_expr(&mut method, &body);
        method.emit(CilInstr::Ret);
        method
    }
    /// Emit the assembly as IL assembly source text (for `ilasm`).
    pub fn emit_ilasm(&self) -> std::string::String {
        let mut out = std::string::String::new();
        out.push_str(".assembly extern mscorlib {}\n");
        out.push_str(&format!(".assembly '{}'\n{{\n", self.assembly.name));
        let (maj, min, bld, rev) = self.assembly.version;
        out.push_str(&format!("  .ver {}:{}:{}:{}\n", maj, min, bld, rev));
        out.push_str("}\n\n");
        out.push_str(&format!(".module '{}.exe'\n\n", self.assembly.name));
        for class in &self.assembly.classes {
            out.push_str(&self.emit_class_ilasm(class));
            out.push('\n');
        }
        out
    }
    /// Emit a single class in IL assembly syntax.
    pub(super) fn emit_class_ilasm(&self, class: &CilClass) -> std::string::String {
        let mut out = std::string::String::new();
        let vis = class.visibility.to_string();
        let kind = if class.is_interface {
            "interface"
        } else if class.is_value_type {
            "value class"
        } else {
            "class"
        };
        let sealed = if class.is_sealed { " sealed" } else { "" };
        let abst = if class.is_abstract { " abstract" } else { "" };
        out.push_str(&format!(
            ".class {} {}{}{} {}\n{{\n",
            vis,
            kind,
            sealed,
            abst,
            class.full_name()
        ));
        if let Some(base) = &class.base_type {
            out.push_str(&format!("  extends {}\n", base));
        }
        for field in &class.fields {
            let static_kw = if field.is_static { "static " } else { "" };
            out.push_str(&format!(
                "  .field {} {}{} '{}'\n",
                field.visibility, static_kw, field.ty, field.name
            ));
        }
        for method in &class.methods {
            out.push_str(&self.emit_method_ilasm(method));
        }
        out.push_str("} // end of class\n");
        out
    }
    /// Emit a single method in IL assembly syntax.
    pub(super) fn emit_method_ilasm(&self, method: &CilMethod) -> std::string::String {
        let mut out = std::string::String::new();
        let static_kw = if method.is_static {
            "static "
        } else {
            "instance "
        };
        let virtual_kw = if method.is_virtual { "virtual " } else { "" };
        let vis = method.visibility.to_string();
        let params_str = method
            .params
            .iter()
            .map(|(name, ty)| format!("{} '{}'", ty, name))
            .collect::<Vec<_>>()
            .join(", ");
        out.push_str(&format!(
            "  .method {} {}{}{} '{}'({}) cil managed\n  {{\n",
            vis, static_kw, virtual_kw, method.return_type, method.name, params_str
        ));
        if let Some((_, ref ep_method)) = self.assembly.entry_point {
            if ep_method == &method.name {
                out.push_str("    .entrypoint\n");
            }
        }
        out.push_str(&format!("    .maxstack {}\n", method.max_stack));
        if !method.locals.is_empty() {
            out.push_str("    .locals init (");
            for (i, local) in method.locals.iter().enumerate() {
                if i > 0 {
                    out.push_str(", ");
                }
                let name_str = local
                    .name
                    .as_deref()
                    .map(|n| format!(" '{}'", n))
                    .unwrap_or_default();
                out.push_str(&format!("[{}] {}{}", i, local.ty, name_str));
            }
            out.push_str(")\n");
        }
        for instr in &method.instructions {
            match instr {
                CilInstr::Label(lbl) => out.push_str(&format!("  {}:\n", lbl)),
                CilInstr::Comment(s) => out.push_str(&format!("    // {}\n", s)),
                _ => out.push_str(&format!("    {}\n", emit_cil_instr(instr))),
            }
        }
        out.push_str("  } // end of method\n");
        out
    }
}
#[allow(dead_code)]
pub struct CILConstantFoldingHelper;
impl CILConstantFoldingHelper {
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
pub struct CILCacheEntry {
    pub key: String,
    pub data: Vec<u8>,
    pub timestamp: u64,
    pub valid: bool,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CILLivenessInfo {
    pub live_in: Vec<std::collections::HashSet<u32>>,
    pub live_out: Vec<std::collections::HashSet<u32>>,
    pub defs: Vec<std::collections::HashSet<u32>>,
    pub uses: Vec<std::collections::HashSet<u32>>,
}
impl CILLivenessInfo {
    #[allow(dead_code)]
    pub fn new(block_count: usize) -> Self {
        CILLivenessInfo {
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
/// A feature flag set for CilExt capabilities.
#[derive(Debug, Clone, Default)]
pub struct CilExtFeatures {
    pub(super) flags: std::collections::HashSet<String>,
}
impl CilExtFeatures {
    pub fn new() -> Self {
        CilExtFeatures::default()
    }
    pub fn enable(&mut self, flag: impl Into<String>) {
        self.flags.insert(flag.into());
    }
    pub fn disable(&mut self, flag: &str) {
        self.flags.remove(flag);
    }
    pub fn is_enabled(&self, flag: &str) -> bool {
        self.flags.contains(flag)
    }
    pub fn len(&self) -> usize {
        self.flags.len()
    }
    pub fn is_empty(&self) -> bool {
        self.flags.is_empty()
    }
    pub fn union(&self, other: &CilExtFeatures) -> CilExtFeatures {
        CilExtFeatures {
            flags: self.flags.union(&other.flags).cloned().collect(),
        }
    }
    pub fn intersection(&self, other: &CilExtFeatures) -> CilExtFeatures {
        CilExtFeatures {
            flags: self.flags.intersection(&other.flags).cloned().collect(),
        }
    }
}
/// A local variable declaration within a CIL method.
#[derive(Debug, Clone)]
pub struct CilLocal {
    pub index: u16,
    pub ty: CilType,
    pub name: Option<std::string::String>,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CILPassConfig {
    pub phase: CILPassPhase,
    pub enabled: bool,
    pub max_iterations: u32,
    pub debug_output: bool,
    pub pass_name: String,
}
impl CILPassConfig {
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>, phase: CILPassPhase) -> Self {
        CILPassConfig {
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
pub struct CILAnalysisCache {
    pub(super) entries: std::collections::HashMap<String, CILCacheEntry>,
    pub(super) max_size: usize,
    pub(super) hits: u64,
    pub(super) misses: u64,
}
impl CILAnalysisCache {
    #[allow(dead_code)]
    pub fn new(max_size: usize) -> Self {
        CILAnalysisCache {
            entries: std::collections::HashMap::new(),
            max_size,
            hits: 0,
            misses: 0,
        }
    }
    #[allow(dead_code)]
    pub fn get(&mut self, key: &str) -> Option<&CILCacheEntry> {
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
            CILCacheEntry {
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
/// CIL type representation.
///
/// Covers all types available in the .NET Common Type System (CTS).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CilType {
    /// `void` — no return value
    Void,
    /// `bool` — System.Boolean (1 byte)
    Bool,
    /// `int8` — System.SByte
    Int8,
    /// `int16` — System.Int16
    Int16,
    /// `int32` — System.Int32
    Int32,
    /// `int64` — System.Int64
    Int64,
    /// `uint8` — System.Byte
    UInt8,
    /// `uint16` — System.UInt16
    UInt16,
    /// `uint32` — System.UInt32
    UInt32,
    /// `uint64` — System.UInt64
    UInt64,
    /// `float32` — System.Single
    Float32,
    /// `float64` — System.Double
    Float64,
    /// `char` — System.Char (UTF-16 code unit)
    Char,
    /// `string` — System.String (immutable UTF-16 string)
    String,
    /// `object` — System.Object (root of the type hierarchy)
    Object,
    /// `class [Assembly]Namespace.TypeName`
    Class {
        assembly: Option<std::string::String>,
        namespace: std::string::String,
        name: std::string::String,
    },
    /// `valuetype [Assembly]Namespace.TypeName`
    ValueType {
        assembly: Option<std::string::String>,
        namespace: std::string::String,
        name: std::string::String,
    },
    /// `T[]` — single-dimensional array (szarray)
    Array(Box<CilType>),
    /// `T[,]` — multi-dimensional array
    MdArray(Box<CilType>, u32),
    /// `T*` — unmanaged pointer
    Ptr(Box<CilType>),
    /// `T&` — managed reference (byref)
    ByRef(Box<CilType>),
    /// Generic instance: `class MyType<T1, T2>`
    Generic(Box<CilType>, Vec<CilType>),
    /// Generic parameter `!T` (class-level)
    GenericParam(u32),
    /// Generic method parameter `!!T`
    GenericMethodParam(u32),
    /// `native int` — platform-native integer
    NativeInt,
    /// `native uint`
    NativeUInt,
}
impl CilType {
    /// Returns `true` if this is a value type (not a reference type).
    pub fn is_value_type(&self) -> bool {
        matches!(
            self,
            CilType::Bool
                | CilType::Int8
                | CilType::Int16
                | CilType::Int32
                | CilType::Int64
                | CilType::UInt8
                | CilType::UInt16
                | CilType::UInt32
                | CilType::UInt64
                | CilType::Float32
                | CilType::Float64
                | CilType::Char
                | CilType::NativeInt
                | CilType::NativeUInt
                | CilType::ValueType { .. }
        )
    }
    /// Returns `true` if this is a reference type.
    pub fn is_reference_type(&self) -> bool {
        !self.is_value_type()
    }
    /// Return the fully-qualified boxed name for use in generics.
    pub fn boxed_name(&self) -> &'static str {
        match self {
            CilType::Bool => "System.Boolean",
            CilType::Int8 => "System.SByte",
            CilType::Int16 => "System.Int16",
            CilType::Int32 => "System.Int32",
            CilType::Int64 => "System.Int64",
            CilType::Float32 => "System.Single",
            CilType::Float64 => "System.Double",
            CilType::Char => "System.Char",
            _ => "System.Object",
        }
    }
    /// Helper: create a `class` type in the given namespace.
    pub fn class_in(
        namespace: impl Into<std::string::String>,
        name: impl Into<std::string::String>,
    ) -> Self {
        CilType::Class {
            assembly: None,
            namespace: namespace.into(),
            name: name.into(),
        }
    }
    /// Helper: `System.Collections.Generic.List<T>`.
    pub fn list_of(elem: CilType) -> Self {
        CilType::Generic(
            Box::new(CilType::class_in("System.Collections.Generic", "List`1")),
            vec![elem],
        )
    }
    /// Helper: `System.Func<TArg, TResult>`.
    pub fn func_of(arg: CilType, result: CilType) -> Self {
        CilType::Generic(
            Box::new(CilType::class_in("System", "Func`2")),
            vec![arg, result],
        )
    }
}
/// A text buffer for building CilExt output source code.
#[derive(Debug, Default)]
pub struct CilExtSourceBuffer {
    pub(super) buf: String,
    pub(super) indent_level: usize,
    pub(super) indent_str: String,
}
impl CilExtSourceBuffer {
    pub fn new() -> Self {
        CilExtSourceBuffer {
            buf: String::new(),
            indent_level: 0,
            indent_str: "    ".to_string(),
        }
    }
    pub fn with_indent(mut self, indent: impl Into<String>) -> Self {
        self.indent_str = indent.into();
        self
    }
    pub fn push_line(&mut self, line: &str) {
        for _ in 0..self.indent_level {
            self.buf.push_str(&self.indent_str);
        }
        self.buf.push_str(line);
        self.buf.push('\n');
    }
    pub fn push_raw(&mut self, s: &str) {
        self.buf.push_str(s);
    }
    pub fn indent(&mut self) {
        self.indent_level += 1;
    }
    pub fn dedent(&mut self) {
        self.indent_level = self.indent_level.saturating_sub(1);
    }
    pub fn as_str(&self) -> &str {
        &self.buf
    }
    pub fn len(&self) -> usize {
        self.buf.len()
    }
    pub fn is_empty(&self) -> bool {
        self.buf.is_empty()
    }
    pub fn line_count(&self) -> usize {
        self.buf.lines().count()
    }
    pub fn into_string(self) -> String {
        self.buf
    }
    pub fn reset(&mut self) {
        self.buf.clear();
        self.indent_level = 0;
    }
}
/// Pipeline profiler for CilExt.
#[derive(Debug, Default)]
pub struct CilExtProfiler {
    pub(super) timings: Vec<CilExtPassTiming>,
}
impl CilExtProfiler {
    pub fn new() -> Self {
        CilExtProfiler::default()
    }
    pub fn record(&mut self, t: CilExtPassTiming) {
        self.timings.push(t);
    }
    pub fn total_elapsed_us(&self) -> u64 {
        self.timings.iter().map(|t| t.elapsed_us).sum()
    }
    pub fn slowest_pass(&self) -> Option<&CilExtPassTiming> {
        self.timings.iter().max_by_key(|t| t.elapsed_us)
    }
    pub fn num_passes(&self) -> usize {
        self.timings.len()
    }
    pub fn profitable_passes(&self) -> Vec<&CilExtPassTiming> {
        self.timings.iter().filter(|t| t.is_profitable()).collect()
    }
}
/// A CIL method definition.
#[derive(Debug, Clone)]
pub struct CilMethod {
    pub name: std::string::String,
    pub params: Vec<(std::string::String, CilType)>,
    pub return_type: CilType,
    pub locals: Vec<CilLocal>,
    pub instructions: Vec<CilInstr>,
    pub is_static: bool,
    pub is_virtual: bool,
    pub is_abstract: bool,
    pub visibility: CilVisibility,
    pub max_stack: u32,
    pub custom_attrs: Vec<std::string::String>,
}
impl CilMethod {
    /// Create a new static method.
    pub fn new_static(name: impl Into<std::string::String>, return_type: CilType) -> Self {
        CilMethod {
            name: name.into(),
            params: Vec::new(),
            return_type,
            locals: Vec::new(),
            instructions: Vec::new(),
            is_static: true,
            is_virtual: false,
            is_abstract: false,
            visibility: CilVisibility::Public,
            max_stack: 8,
            custom_attrs: Vec::new(),
        }
    }
    /// Create a new instance method.
    pub fn new_instance(name: impl Into<std::string::String>, return_type: CilType) -> Self {
        CilMethod {
            name: name.into(),
            params: Vec::new(),
            return_type,
            locals: Vec::new(),
            instructions: Vec::new(),
            is_static: false,
            is_virtual: false,
            is_abstract: false,
            visibility: CilVisibility::Public,
            max_stack: 8,
            custom_attrs: Vec::new(),
        }
    }
    /// Add a parameter and return its index.
    pub fn add_param(&mut self, name: impl Into<std::string::String>, ty: CilType) -> u16 {
        let idx = self.params.len() as u16;
        self.params.push((name.into(), ty));
        idx
    }
    /// Add a local variable and return its index.
    pub fn add_local(&mut self, ty: CilType, name: Option<std::string::String>) -> u16 {
        let idx = self.locals.len() as u16;
        self.locals.push(CilLocal {
            index: idx,
            ty,
            name,
        });
        idx
    }
    /// Emit an instruction.
    pub fn emit(&mut self, instr: CilInstr) {
        self.instructions.push(instr);
    }
    /// Emit a label.
    pub fn emit_label(&mut self, label: impl Into<std::string::String>) {
        self.instructions.push(CilInstr::Label(label.into()));
    }
}
/// Visibility of a CIL method or field.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CilVisibility {
    Private,
    Assembly,
    Family,
    Public,
}
/// Heuristic freshness key for CilExt incremental compilation.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CilExtIncrKey {
    pub content_hash: u64,
    pub config_hash: u64,
}
impl CilExtIncrKey {
    pub fn new(content: u64, config: u64) -> Self {
        CilExtIncrKey {
            content_hash: content,
            config_hash: config,
        }
    }
    pub fn combined_hash(&self) -> u64 {
        self.content_hash.wrapping_mul(0x9e3779b97f4a7c15) ^ self.config_hash
    }
    pub fn matches(&self, other: &CilExtIncrKey) -> bool {
        self.content_hash == other.content_hash && self.config_hash == other.config_hash
    }
}
/// Severity of a CilExt diagnostic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CilExtDiagSeverity {
    Note,
    Warning,
    Error,
}
/// A version tag for CilExt output artifacts.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CilExtVersion {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
    pub pre: Option<String>,
}
impl CilExtVersion {
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        CilExtVersion {
            major,
            minor,
            patch,
            pre: None,
        }
    }
    pub fn with_pre(mut self, pre: impl Into<String>) -> Self {
        self.pre = Some(pre.into());
        self
    }
    pub fn is_stable(&self) -> bool {
        self.pre.is_none()
    }
    pub fn is_compatible_with(&self, other: &CilExtVersion) -> bool {
        self.major == other.major && self.minor >= other.minor
    }
}
/// A CIL class definition.
#[derive(Debug, Clone)]
pub struct CilClass {
    pub name: std::string::String,
    pub namespace: std::string::String,
    pub fields: Vec<CilField>,
    pub methods: Vec<CilMethod>,
    pub interfaces: Vec<CilType>,
    pub base_type: Option<CilType>,
    pub is_value_type: bool,
    pub is_abstract: bool,
    pub is_sealed: bool,
    pub is_interface: bool,
    pub visibility: CilVisibility,
    pub type_params: Vec<std::string::String>,
    pub custom_attrs: Vec<std::string::String>,
    pub nested: Vec<CilClass>,
}
impl CilClass {
    /// Create a new public class.
    pub fn new(
        namespace: impl Into<std::string::String>,
        name: impl Into<std::string::String>,
    ) -> Self {
        CilClass {
            name: name.into(),
            namespace: namespace.into(),
            fields: Vec::new(),
            methods: Vec::new(),
            interfaces: Vec::new(),
            base_type: None,
            is_value_type: false,
            is_abstract: false,
            is_sealed: false,
            is_interface: false,
            visibility: CilVisibility::Public,
            type_params: Vec::new(),
            custom_attrs: Vec::new(),
            nested: Vec::new(),
        }
    }
    /// Add a method.
    pub fn add_method(&mut self, method: CilMethod) {
        self.methods.push(method);
    }
    /// Add a field.
    pub fn add_field(&mut self, field: CilField) {
        self.fields.push(field);
    }
    /// Add a nested class.
    pub fn add_nested(&mut self, nested: CilClass) {
        self.nested.push(nested);
    }
    /// Get the fully qualified type name.
    pub fn full_name(&self) -> std::string::String {
        if self.namespace.is_empty() {
            self.name.clone()
        } else {
            format!("{}.{}", self.namespace, self.name)
        }
    }
    /// Find a method by name.
    pub fn find_method(&self, name: &str) -> Option<&CilMethod> {
        self.methods.iter().find(|m| m.name == name)
    }
}
/// A .NET assembly containing one or more classes.
#[derive(Debug, Clone)]
pub struct CilAssembly {
    pub name: std::string::String,
    pub version: (u16, u16, u16, u16),
    pub classes: Vec<CilClass>,
    pub entry_point: Option<(std::string::String, std::string::String)>,
    pub custom_attrs: Vec<std::string::String>,
    pub references: Vec<std::string::String>,
    pub target_runtime: std::string::String,
}
impl CilAssembly {
    /// Create a new assembly.
    pub fn new(name: impl Into<std::string::String>) -> Self {
        CilAssembly {
            name: name.into(),
            version: (1, 0, 0, 0),
            classes: Vec::new(),
            entry_point: None,
            custom_attrs: Vec::new(),
            references: vec!["mscorlib".to_string()],
            target_runtime: "v4.0.30319".to_string(),
        }
    }
    /// Add a class.
    pub fn add_class(&mut self, class: CilClass) {
        self.classes.push(class);
    }
    /// Find a class by full name.
    pub fn find_class(&self, full_name: &str) -> Option<&CilClass> {
        self.classes.iter().find(|c| c.full_name() == full_name)
    }
    /// Set the entry point.
    pub fn set_entry_point(
        &mut self,
        class_name: impl Into<std::string::String>,
        method_name: impl Into<std::string::String>,
    ) {
        self.entry_point = Some((class_name.into(), method_name.into()));
    }
}
/// A diagnostic message from a CilExt pass.
#[derive(Debug, Clone)]
pub struct CilExtDiagMsg {
    pub severity: CilExtDiagSeverity,
    pub pass: String,
    pub message: String,
}
impl CilExtDiagMsg {
    pub fn error(pass: impl Into<String>, msg: impl Into<String>) -> Self {
        CilExtDiagMsg {
            severity: CilExtDiagSeverity::Error,
            pass: pass.into(),
            message: msg.into(),
        }
    }
    pub fn warning(pass: impl Into<String>, msg: impl Into<String>) -> Self {
        CilExtDiagMsg {
            severity: CilExtDiagSeverity::Warning,
            pass: pass.into(),
            message: msg.into(),
        }
    }
    pub fn note(pass: impl Into<String>, msg: impl Into<String>) -> Self {
        CilExtDiagMsg {
            severity: CilExtDiagSeverity::Note,
            pass: pass.into(),
            message: msg.into(),
        }
    }
}
/// A generic key-value configuration store for CilExt.
#[derive(Debug, Clone, Default)]
pub struct CilExtConfig {
    pub(super) entries: std::collections::HashMap<String, String>,
}
impl CilExtConfig {
    pub fn new() -> Self {
        CilExtConfig::default()
    }
    pub fn set(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.entries.insert(key.into(), value.into());
    }
    pub fn get(&self, key: &str) -> Option<&str> {
        self.entries.get(key).map(|s| s.as_str())
    }
    pub fn get_bool(&self, key: &str) -> bool {
        matches!(self.get(key), Some("true") | Some("1") | Some("yes"))
    }
    pub fn get_int(&self, key: &str) -> Option<i64> {
        self.get(key)?.parse().ok()
    }
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}
/// Collects CilExt diagnostics.
#[derive(Debug, Default)]
pub struct CilExtDiagCollector {
    pub(super) msgs: Vec<CilExtDiagMsg>,
}
impl CilExtDiagCollector {
    pub fn new() -> Self {
        CilExtDiagCollector::default()
    }
    pub fn emit(&mut self, d: CilExtDiagMsg) {
        self.msgs.push(d);
    }
    pub fn has_errors(&self) -> bool {
        self.msgs
            .iter()
            .any(|d| d.severity == CilExtDiagSeverity::Error)
    }
    pub fn errors(&self) -> Vec<&CilExtDiagMsg> {
        self.msgs
            .iter()
            .filter(|d| d.severity == CilExtDiagSeverity::Error)
            .collect()
    }
    pub fn warnings(&self) -> Vec<&CilExtDiagMsg> {
        self.msgs
            .iter()
            .filter(|d| d.severity == CilExtDiagSeverity::Warning)
            .collect()
    }
    pub fn len(&self) -> usize {
        self.msgs.len()
    }
    pub fn is_empty(&self) -> bool {
        self.msgs.is_empty()
    }
    pub fn clear(&mut self) {
        self.msgs.clear();
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CILWorklist {
    pub(super) items: std::collections::VecDeque<u32>,
    pub(super) in_worklist: std::collections::HashSet<u32>,
}
impl CILWorklist {
    #[allow(dead_code)]
    pub fn new() -> Self {
        CILWorklist {
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
/// A reference to a .NET method for use in call instructions.
#[derive(Debug, Clone)]
pub struct CilMethodRef {
    pub call_conv: CilCallConv,
    pub return_type: CilType,
    pub declaring_type: CilType,
    pub name: std::string::String,
    pub param_types: Vec<CilType>,
}
/// An indirect call signature.
#[derive(Debug, Clone)]
pub struct CilCallSig {
    pub call_conv: CilCallConv,
    pub return_type: CilType,
    pub param_types: Vec<CilType>,
}
/// Emission statistics for CilExt.
#[derive(Debug, Clone, Default)]
pub struct CilExtEmitStats {
    pub bytes_emitted: usize,
    pub items_emitted: usize,
    pub errors: usize,
    pub warnings: usize,
    pub elapsed_ms: u64,
}
impl CilExtEmitStats {
    pub fn new() -> Self {
        CilExtEmitStats::default()
    }
    pub fn throughput_bps(&self) -> f64 {
        if self.elapsed_ms == 0 {
            0.0
        } else {
            self.bytes_emitted as f64 / (self.elapsed_ms as f64 / 1000.0)
        }
    }
    pub fn is_clean(&self) -> bool {
        self.errors == 0
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CILDominatorTree {
    pub idom: Vec<Option<u32>>,
    pub dom_children: Vec<Vec<u32>>,
    pub dom_depth: Vec<u32>,
}
impl CILDominatorTree {
    #[allow(dead_code)]
    pub fn new(size: usize) -> Self {
        CILDominatorTree {
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
/// A field declaration in a CIL class.
#[derive(Debug, Clone)]
pub struct CilField {
    pub name: std::string::String,
    pub ty: CilType,
    pub is_static: bool,
    pub visibility: CilVisibility,
    pub init_value: Option<CilLiteral>,
}
/// Calling convention for CIL method references.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CilCallConv {
    /// Static method
    Default,
    /// Instance method
    Instance,
    /// Generic instance method
    Generic(u32),
}
/// Pass-timing record for CilExt profiler.
#[derive(Debug, Clone)]
pub struct CilExtPassTiming {
    pub pass_name: String,
    pub elapsed_us: u64,
    pub items_processed: usize,
    pub bytes_before: usize,
    pub bytes_after: usize,
}
impl CilExtPassTiming {
    pub fn new(
        pass_name: impl Into<String>,
        elapsed_us: u64,
        items: usize,
        before: usize,
        after: usize,
    ) -> Self {
        CilExtPassTiming {
            pass_name: pass_name.into(),
            elapsed_us,
            items_processed: items,
            bytes_before: before,
            bytes_after: after,
        }
    }
    pub fn throughput_mps(&self) -> f64 {
        if self.elapsed_us == 0 {
            0.0
        } else {
            self.items_processed as f64 / (self.elapsed_us as f64 / 1_000_000.0)
        }
    }
    pub fn size_ratio(&self) -> f64 {
        if self.bytes_before == 0 {
            1.0
        } else {
            self.bytes_after as f64 / self.bytes_before as f64
        }
    }
    pub fn is_profitable(&self) -> bool {
        self.size_ratio() <= 1.05
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct CILPassStats {
    pub total_runs: u32,
    pub successful_runs: u32,
    pub total_changes: u64,
    pub time_ms: u64,
    pub iterations_used: u32,
}
impl CILPassStats {
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
