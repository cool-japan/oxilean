//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::HashMap;

use super::functions::SOLIDITY_RUNTIME;

use std::collections::{HashSet, VecDeque};

/// State mutability of a function.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StateMutability {
    /// May read and modify state.
    NonPayable,
    /// May receive Ether.
    Payable,
    /// Reads state but does not modify it.
    View,
    /// Does not read or modify state.
    Pure,
}
/// Dominator tree for SolExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SolExtDomTree {
    pub(super) idom: Vec<Option<usize>>,
    pub(super) children: Vec<Vec<usize>>,
    pub(super) depth: Vec<usize>,
}
impl SolExtDomTree {
    #[allow(dead_code)]
    pub fn new(n: usize) -> Self {
        Self {
            idom: vec![None; n],
            children: vec![Vec::new(); n],
            depth: vec![0; n],
        }
    }
    #[allow(dead_code)]
    pub fn set_idom(&mut self, node: usize, dom: usize) {
        if node < self.idom.len() {
            self.idom[node] = Some(dom);
            if dom < self.children.len() {
                self.children[dom].push(node);
            }
            self.depth[node] = if dom < self.depth.len() {
                self.depth[dom] + 1
            } else {
                1
            };
        }
    }
    #[allow(dead_code)]
    pub fn dominates(&self, a: usize, mut b: usize) -> bool {
        if a == b {
            return true;
        }
        let n = self.idom.len();
        for _ in 0..n {
            match self.idom.get(b).copied().flatten() {
                None => return false,
                Some(p) if p == a => return true,
                Some(p) if p == b => return false,
                Some(p) => b = p,
            }
        }
        false
    }
    #[allow(dead_code)]
    pub fn children_of(&self, n: usize) -> &[usize] {
        self.children.get(n).map(|v| v.as_slice()).unwrap_or(&[])
    }
    #[allow(dead_code)]
    pub fn depth_of(&self, n: usize) -> usize {
        self.depth.get(n).copied().unwrap_or(0)
    }
    #[allow(dead_code)]
    pub fn lca(&self, mut a: usize, mut b: usize) -> usize {
        let n = self.idom.len();
        for _ in 0..(2 * n) {
            if a == b {
                return a;
            }
            if self.depth_of(a) > self.depth_of(b) {
                a = self.idom.get(a).and_then(|x| *x).unwrap_or(a);
            } else {
                b = self.idom.get(b).and_then(|x| *x).unwrap_or(b);
            }
        }
        0
    }
}
/// Solidity expression AST node.
#[derive(Debug, Clone, PartialEq)]
pub enum SolidityExpr {
    /// Integer literal: `42`
    IntLit(i128),
    /// Boolean literal: `true` / `false`
    BoolLit(bool),
    /// String literal: `"hello"`
    StrLit(String),
    /// Address literal: `0x1234...`
    AddressLit(String),
    /// Hex literal: `0xdeadbeef`
    HexLit(String),
    /// Variable reference: `myVar`
    Var(String),
    /// `this`
    This,
    /// `msg.sender`
    MsgSender,
    /// `msg.value`
    MsgValue,
    /// `msg.data`
    MsgData,
    /// `block.timestamp`
    BlockTimestamp,
    /// `block.number`
    BlockNumber,
    /// `block.basefee`
    BlockBasefee,
    /// `tx.origin`
    TxOrigin,
    /// `gasleft()`
    GasLeft,
    /// Field access: `expr.field`
    FieldAccess(Box<SolidityExpr>, String),
    /// Index access: `expr[index]`
    Index(Box<SolidityExpr>, Box<SolidityExpr>),
    /// Function call: `f(args...)`
    Call(Box<SolidityExpr>, Vec<SolidityExpr>),
    /// Named argument call: `f({name: val, ...})`
    NamedCall(Box<SolidityExpr>, Vec<(String, SolidityExpr)>),
    /// Type cast: `uint256(expr)`
    Cast(SolidityType, Box<SolidityExpr>),
    /// `abi.encode(args...)`
    AbiEncode(Vec<SolidityExpr>),
    /// `abi.encodePacked(args...)`
    AbiEncodePacked(Vec<SolidityExpr>),
    /// `abi.encodeWithSelector(selector, args...)`
    AbiEncodeWithSelector(Box<SolidityExpr>, Vec<SolidityExpr>),
    /// `keccak256(data)`
    Keccak256(Box<SolidityExpr>),
    /// `sha256(data)`
    Sha256(Box<SolidityExpr>),
    /// `ecrecover(hash, v, r, s)`
    Ecrecover(
        Box<SolidityExpr>,
        Box<SolidityExpr>,
        Box<SolidityExpr>,
        Box<SolidityExpr>,
    ),
    /// Binary operation: `a + b`
    BinOp(String, Box<SolidityExpr>, Box<SolidityExpr>),
    /// Unary operation: `!a`, `-a`, `~a`
    UnaryOp(String, Box<SolidityExpr>),
    /// Ternary: `cond ? then_ : else_`
    Ternary(Box<SolidityExpr>, Box<SolidityExpr>, Box<SolidityExpr>),
    /// `new T(args...)`
    New(SolidityType, Vec<SolidityExpr>),
    /// `delete expr`
    Delete(Box<SolidityExpr>),
    /// Array literal: `[a, b, c]`
    ArrayLit(Vec<SolidityExpr>),
    /// Tuple literal: `(a, b, c)`
    TupleLit(Vec<SolidityExpr>),
    /// `type(T).max` / `type(T).min`
    TypeMax(SolidityType),
    TypeMin(SolidityType),
    /// `payable(addr)`
    Payable(Box<SolidityExpr>),
}
/// A struct definition.
#[derive(Debug, Clone)]
pub struct SolidityStruct {
    pub name: String,
    pub fields: Vec<(SolidityType, String)>,
    pub doc: Option<String>,
}
/// A function parameter or return value.
#[derive(Debug, Clone)]
pub struct SolidityParam {
    /// Parameter type.
    pub ty: SolidityType,
    /// Optional data location (`memory`, `calldata`, `storage`).
    pub location: Option<String>,
    /// Parameter name (may be empty for returns).
    pub name: String,
}
impl SolidityParam {
    pub fn new(ty: SolidityType, name: impl Into<String>) -> Self {
        let location = if ty.is_reference_type() {
            Some("memory".into())
        } else {
            None
        };
        Self {
            ty,
            location,
            name: name.into(),
        }
    }
    pub fn calldata(ty: SolidityType, name: impl Into<String>) -> Self {
        Self {
            ty,
            location: Some("calldata".into()),
            name: name.into(),
        }
    }
    pub fn storage(ty: SolidityType, name: impl Into<String>) -> Self {
        Self {
            ty,
            location: Some("storage".into()),
            name: name.into(),
        }
    }
}
/// Compilation context for a single Solidity source unit.
#[derive(Debug, Clone)]
pub struct CompilationCtx {
    /// Pragma directives.
    pub pragmas: Vec<String>,
    /// Import statements.
    pub imports: Vec<String>,
    /// Whether to include the runtime library.
    pub include_runtime: bool,
}
#[allow(dead_code)]
pub struct SolPassRegistry {
    pub(super) configs: Vec<SolPassConfig>,
    pub(super) stats: std::collections::HashMap<String, SolPassStats>,
}
impl SolPassRegistry {
    #[allow(dead_code)]
    pub fn new() -> Self {
        SolPassRegistry {
            configs: Vec::new(),
            stats: std::collections::HashMap::new(),
        }
    }
    #[allow(dead_code)]
    pub fn register(&mut self, config: SolPassConfig) {
        self.stats
            .insert(config.pass_name.clone(), SolPassStats::new());
        self.configs.push(config);
    }
    #[allow(dead_code)]
    pub fn enabled_passes(&self) -> Vec<&SolPassConfig> {
        self.configs.iter().filter(|c| c.enabled).collect()
    }
    #[allow(dead_code)]
    pub fn get_stats(&self, name: &str) -> Option<&SolPassStats> {
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
/// Pass execution phase for SolExt.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SolExtPassPhase {
    Early,
    Middle,
    Late,
    Finalize,
}
impl SolExtPassPhase {
    #[allow(dead_code)]
    pub fn is_early(&self) -> bool {
        matches!(self, Self::Early)
    }
    #[allow(dead_code)]
    pub fn is_middle(&self) -> bool {
        matches!(self, Self::Middle)
    }
    #[allow(dead_code)]
    pub fn is_late(&self) -> bool {
        matches!(self, Self::Late)
    }
    #[allow(dead_code)]
    pub fn is_finalize(&self) -> bool {
        matches!(self, Self::Finalize)
    }
    #[allow(dead_code)]
    pub fn order(&self) -> u32 {
        match self {
            Self::Early => 0,
            Self::Middle => 1,
            Self::Late => 2,
            Self::Finalize => 3,
        }
    }
    #[allow(dead_code)]
    pub fn from_order(n: u32) -> Option<Self> {
        match n {
            0 => Some(Self::Early),
            1 => Some(Self::Middle),
            2 => Some(Self::Late),
            3 => Some(Self::Finalize),
            _ => None,
        }
    }
}
/// Worklist for SolExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SolExtWorklist {
    pub(super) items: std::collections::VecDeque<usize>,
    pub(super) present: Vec<bool>,
}
impl SolExtWorklist {
    #[allow(dead_code)]
    pub fn new(capacity: usize) -> Self {
        Self {
            items: std::collections::VecDeque::new(),
            present: vec![false; capacity],
        }
    }
    #[allow(dead_code)]
    pub fn push(&mut self, id: usize) {
        if id < self.present.len() && !self.present[id] {
            self.present[id] = true;
            self.items.push_back(id);
        }
    }
    #[allow(dead_code)]
    pub fn push_front(&mut self, id: usize) {
        if id < self.present.len() && !self.present[id] {
            self.present[id] = true;
            self.items.push_front(id);
        }
    }
    #[allow(dead_code)]
    pub fn pop(&mut self) -> Option<usize> {
        let id = self.items.pop_front()?;
        if id < self.present.len() {
            self.present[id] = false;
        }
        Some(id)
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
    pub fn contains(&self, id: usize) -> bool {
        id < self.present.len() && self.present[id]
    }
    #[allow(dead_code)]
    pub fn drain_all(&mut self) -> Vec<usize> {
        let v: Vec<usize> = self.items.drain(..).collect();
        for &id in &v {
            if id < self.present.len() {
                self.present[id] = false;
            }
        }
        v
    }
}
/// An enum definition.
#[derive(Debug, Clone)]
pub struct SolidityEnum {
    pub name: String,
    pub variants: Vec<String>,
    pub doc: Option<String>,
}
/// A state variable in a contract.
#[derive(Debug, Clone)]
pub struct SolidityStateVar {
    pub ty: SolidityType,
    pub name: String,
    pub visibility: Visibility,
    pub is_immutable: bool,
    pub is_constant: bool,
    pub init: Option<SolidityExpr>,
    pub doc: Option<String>,
}
/// The main Solidity code generation backend.
#[derive(Debug, Default)]
pub struct SolidityBackend {
    /// Emitted contracts (in order).
    pub contracts: Vec<SolidityContract>,
    /// Compilation context.
    pub ctx: CompilationCtx,
    /// Type alias table: `alias → canonical SolidityType`.
    pub type_aliases: HashMap<String, SolidityType>,
    /// Source buffer accumulated during emission.
    pub source: String,
}
impl SolidityBackend {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_runtime(mut self) -> Self {
        self.ctx.include_runtime = true;
        self
    }
    pub fn add_contract(&mut self, contract: SolidityContract) {
        self.contracts.push(contract);
    }
    pub fn add_pragma(&mut self, pragma: impl Into<String>) {
        self.ctx.pragmas.push(pragma.into());
    }
    pub fn add_import(&mut self, import: impl Into<String>) {
        self.ctx.imports.push(import.into());
    }
    /// Compile a single LCNF-style declaration (simplified: name → state var).
    pub fn compile_decl(&mut self, name: &str, ty: SolidityType) -> SolidityStateVar {
        SolidityStateVar {
            ty,
            name: name.into(),
            visibility: Visibility::Private,
            is_immutable: false,
            is_constant: false,
            init: None,
            doc: None,
        }
    }
    /// Emit the full Solidity source for all registered contracts.
    pub fn emit_contract(&mut self) -> String {
        let mut out = String::new();
        out.push_str("// SPDX-License-Identifier: MIT\n");
        out.push_str("// Generated by OxiLean Solidity Backend\n\n");
        for pragma in &self.ctx.pragmas {
            out.push_str(&format!("pragma solidity {};\n", pragma));
        }
        out.push('\n');
        for imp in &self.ctx.imports {
            out.push_str(&format!("import \"{}\";\n", imp));
        }
        if !self.ctx.imports.is_empty() {
            out.push('\n');
        }
        if self.ctx.include_runtime {
            out.push_str(SOLIDITY_RUNTIME);
            out.push('\n');
        }
        for contract in &self.contracts {
            out.push_str(&Self::emit_single_contract(contract));
            out.push('\n');
        }
        self.source = out.clone();
        out
    }
    pub(super) fn emit_single_contract(c: &SolidityContract) -> String {
        let mut out = String::new();
        if let Some(doc) = &c.doc {
            out.push_str(&format!("/// @title {}\n", doc));
        }
        let bases = if c.bases.is_empty() {
            String::new()
        } else {
            format!(" is {}", c.bases.join(", "))
        };
        out.push_str(&format!("{} {}{} {{\n", c.kind, c.name, bases));
        for s in &c.structs {
            out.push_str(&Self::emit_struct(s, 1));
        }
        for e in &c.enums {
            out.push_str(&Self::emit_enum(e, 1));
        }
        for ev in &c.events {
            out.push_str(&Self::emit_event(ev, 1));
        }
        for err in &c.errors {
            out.push_str(&Self::emit_error(err, 1));
        }
        for sv in &c.state_vars {
            out.push_str(&Self::emit_state_var(sv, 1));
        }
        if let Some(ctor) = &c.constructor {
            out.push_str(&Self::emit_constructor(ctor, 1));
        }
        if let Some(recv) = &c.receive {
            out.push_str(&Self::emit_receive(recv, 1));
        }
        if let Some(fb) = &c.fallback {
            out.push_str(&Self::emit_fallback(fb, 1));
        }
        for m in &c.modifiers {
            out.push_str(&Self::emit_modifier(m, 1));
        }
        for func in &c.functions {
            out.push_str(&Self::emit_function(func, 1));
        }
        out.push_str("}\n");
        out
    }
    pub(super) fn indent(level: usize) -> String {
        "    ".repeat(level)
    }
    pub(super) fn emit_struct(s: &SolidityStruct, indent: usize) -> String {
        let ind = Self::indent(indent);
        let mut out = String::new();
        if let Some(doc) = &s.doc {
            out.push_str(&format!("{}/// @dev {}\n", ind, doc));
        }
        out.push_str(&format!("{}struct {} {{\n", ind, s.name));
        for (ty, name) in &s.fields {
            out.push_str(&format!("{}    {} {};\n", ind, ty, name));
        }
        out.push_str(&format!("{}}}\n", ind));
        out
    }
    pub(super) fn emit_enum(e: &SolidityEnum, indent: usize) -> String {
        let ind = Self::indent(indent);
        let mut out = String::new();
        if let Some(doc) = &e.doc {
            out.push_str(&format!("{}/// @dev {}\n", ind, doc));
        }
        out.push_str(&format!(
            "{}enum {} {{ {} }}\n",
            ind,
            e.name,
            e.variants.join(", ")
        ));
        out
    }
    pub(super) fn emit_event(ev: &SolidityEvent, indent: usize) -> String {
        let ind = Self::indent(indent);
        let mut out = String::new();
        if let Some(doc) = &ev.doc {
            out.push_str(&format!("{}/// @dev {}\n", ind, doc));
        }
        let fields: Vec<String> = ev
            .fields
            .iter()
            .map(|(ty, indexed, name)| {
                if *indexed {
                    format!("{} indexed {}", ty, name)
                } else {
                    format!("{} {}", ty, name)
                }
            })
            .collect();
        let anon = if ev.anonymous { " anonymous" } else { "" };
        out.push_str(&format!(
            "{}event {}({}){};\n",
            ind,
            ev.name,
            fields.join(", "),
            anon
        ));
        out
    }
    pub(super) fn emit_error(err: &SolidityError, indent: usize) -> String {
        let ind = Self::indent(indent);
        let params: Vec<String> = err.params.iter().map(|p| p.to_string()).collect();
        format!("{}error {}({});\n", ind, err.name, params.join(", "))
    }
    pub(super) fn emit_state_var(sv: &SolidityStateVar, indent: usize) -> String {
        let ind = Self::indent(indent);
        let mut out = String::new();
        if let Some(doc) = &sv.doc {
            out.push_str(&format!("{}/// @dev {}\n", ind, doc));
        }
        let mut parts = vec![sv.ty.to_string(), sv.visibility.to_string()];
        if sv.is_constant {
            parts.push("constant".into());
        } else if sv.is_immutable {
            parts.push("immutable".into());
        }
        parts.push(sv.name.clone());
        if let Some(init) = &sv.init {
            out.push_str(&format!("{}{}  = {};\n", ind, parts.join(" "), init));
        } else {
            out.push_str(&format!("{}{};\n", ind, parts.join(" ")));
        }
        out
    }
    pub(super) fn emit_constructor(ctor: &SolidityFunction, indent: usize) -> String {
        let ind = Self::indent(indent);
        let params: Vec<String> = ctor.params.iter().map(|p| p.to_string()).collect();
        let mut header = format!("{}constructor({})", ind, params.join(", "));
        let mut muts = String::new();
        let m_str = ctor.mutability.to_string();
        if !m_str.is_empty() {
            muts.push(' ');
            muts.push_str(&m_str);
        }
        header.push_str(&muts);
        Self::emit_fn_body(&header, &ctor.body, indent)
    }
    pub(super) fn emit_receive(recv: &SolidityFunction, indent: usize) -> String {
        let ind = Self::indent(indent);
        let header = format!("{}receive() external payable", ind);
        Self::emit_fn_body(&header, &recv.body, indent)
    }
    pub(super) fn emit_fallback(fb: &SolidityFunction, indent: usize) -> String {
        let ind = Self::indent(indent);
        let header = format!("{}fallback() external payable", ind);
        Self::emit_fn_body(&header, &fb.body, indent)
    }
    pub(super) fn emit_modifier(m: &SolidityModifier, indent: usize) -> String {
        let ind = Self::indent(indent);
        let params: Vec<String> = m.params.iter().map(|p| p.to_string()).collect();
        let header = format!("{}modifier {}({})", ind, m.name, params.join(", "));
        Self::emit_fn_body(&header, &m.body, indent)
    }
    pub(super) fn emit_function(func: &SolidityFunction, indent: usize) -> String {
        let ind = Self::indent(indent);
        let mut out = String::new();
        if let Some(doc) = &func.doc {
            out.push_str(&format!("{}/// @notice {}\n", ind, doc));
        }
        let params: Vec<String> = func.params.iter().map(|p| p.to_string()).collect();
        let mut header = format!(
            "{}function {}({}) {} {}",
            ind,
            func.name,
            params.join(", "),
            func.visibility,
            func.mutability
        );
        header = header.trim_end().to_string();
        if func.is_virtual {
            header.push_str(" virtual");
        }
        if func.is_override {
            header.push_str(" override");
        }
        for (mod_name, mod_args) in &func.modifiers {
            if mod_args.is_empty() {
                header.push_str(&format!(" {}", mod_name));
            } else {
                let args: Vec<String> = mod_args.iter().map(|a| a.to_string()).collect();
                header.push_str(&format!(" {}({})", mod_name, args.join(", ")));
            }
        }
        if !func.returns.is_empty() {
            let rets: Vec<String> = func.returns.iter().map(|p| p.to_string()).collect();
            header.push_str(&format!(" returns ({})", rets.join(", ")));
        }
        if func.body.is_empty() {
            out.push_str(&format!("{};\n", header));
        } else {
            out.push_str(&Self::emit_fn_body(&header, &func.body, indent));
        }
        out
    }
    pub(super) fn emit_fn_body(header: &str, body: &[SolidityStmt], indent: usize) -> String {
        let mut out = format!("{} {{\n", header);
        for stmt in body {
            out.push_str(&Self::emit_stmt(stmt, indent + 1));
        }
        out.push_str(&format!("{}}}\n", Self::indent(indent)));
        out
    }
    pub(super) fn emit_stmt(stmt: &SolidityStmt, indent: usize) -> String {
        let ind = Self::indent(indent);
        match stmt {
            SolidityStmt::VarDecl {
                ty,
                location,
                name,
                init,
            } => {
                let loc_str = location.as_deref().unwrap_or("");
                let loc_part = if loc_str.is_empty() {
                    String::new()
                } else {
                    format!(" {}", loc_str)
                };
                if let Some(expr) = init {
                    format!("{}{}{} {} = {};\n", ind, ty, loc_part, name, expr)
                } else {
                    format!("{}{}{} {};\n", ind, ty, loc_part, name)
                }
            }
            SolidityStmt::Assign(lhs, rhs) => format!("{}{} = {};\n", ind, lhs, rhs),
            SolidityStmt::CompoundAssign(op, lhs, rhs) => {
                format!("{}{} {}= {};\n", ind, lhs, op, rhs)
            }
            SolidityStmt::ExprStmt(expr) => format!("{}{};\n", ind, expr),
            SolidityStmt::Return(None) => format!("{}return;\n", ind),
            SolidityStmt::Return(Some(expr)) => format!("{}return {};\n", ind, expr),
            SolidityStmt::If(cond, then_stmts, else_stmts) => {
                let mut out = format!("{}if ({}) {{\n", ind, cond);
                for s in then_stmts {
                    out.push_str(&Self::emit_stmt(s, indent + 1));
                }
                if else_stmts.is_empty() {
                    out.push_str(&format!("{}}}\n", ind));
                } else {
                    out.push_str(&format!("{}}} else {{\n", ind));
                    for s in else_stmts {
                        out.push_str(&Self::emit_stmt(s, indent + 1));
                    }
                    out.push_str(&format!("{}}}\n", ind));
                }
                out
            }
            SolidityStmt::While(cond, body) => {
                let mut out = format!("{}while ({}) {{\n", ind, cond);
                for s in body {
                    out.push_str(&Self::emit_stmt(s, indent + 1));
                }
                out.push_str(&format!("{}}}\n", ind));
                out
            }
            SolidityStmt::For(init, cond, update, body) => {
                let init_str = init
                    .as_ref()
                    .map(|s| {
                        Self::emit_stmt(s, 0)
                            .trim_end_matches('\n')
                            .trim_end_matches(';')
                            .to_string()
                    })
                    .unwrap_or_default();
                let cond_str = cond.as_ref().map(|e| e.to_string()).unwrap_or_default();
                let upd_str = update
                    .as_ref()
                    .map(|s| {
                        Self::emit_stmt(s, 0)
                            .trim_end_matches('\n')
                            .trim_end_matches(';')
                            .to_string()
                    })
                    .unwrap_or_default();
                let mut out = format!("{}for ({}; {}; {}) {{\n", ind, init_str, cond_str, upd_str);
                for s in body {
                    out.push_str(&Self::emit_stmt(s, indent + 1));
                }
                out.push_str(&format!("{}}}\n", ind));
                out
            }
            SolidityStmt::DoWhile(body, cond) => {
                let mut out = format!("{}do {{\n", ind);
                for s in body {
                    out.push_str(&Self::emit_stmt(s, indent + 1));
                }
                out.push_str(&format!("{}}} while ({});\n", ind, cond));
                out
            }
            SolidityStmt::Emit(name, args) => {
                let args_str: Vec<String> = args.iter().map(|a| a.to_string()).collect();
                format!("{}emit {}({});\n", ind, name, args_str.join(", "))
            }
            SolidityStmt::Revert(name, args) => {
                let args_str: Vec<String> = args.iter().map(|a| a.to_string()).collect();
                format!("{}revert {}({});\n", ind, name, args_str.join(", "))
            }
            SolidityStmt::Require(cond, msg) => {
                if let Some(m) = msg {
                    format!("{}require({}, \"{}\");\n", ind, cond, m)
                } else {
                    format!("{}require({});\n", ind, cond)
                }
            }
            SolidityStmt::Assert(cond) => format!("{}assert({});\n", ind, cond),
            SolidityStmt::Break => format!("{}break;\n", ind),
            SolidityStmt::Continue => format!("{}continue;\n", ind),
            SolidityStmt::Unchecked(stmts) => {
                let mut out = format!("{}unchecked {{\n", ind);
                for s in stmts {
                    out.push_str(&Self::emit_stmt(s, indent + 1));
                }
                out.push_str(&format!("{}}}\n", ind));
                out
            }
            SolidityStmt::Assembly(body) => {
                format!("{}assembly {{\n{}{}}}\n", ind, body, ind)
            }
            SolidityStmt::MultiAssign(lhs, rhs) => {
                let lhs_str: Vec<String> = lhs.iter().map(|e| e.to_string()).collect();
                format!("{}({}) = {};\n", ind, lhs_str.join(", "), rhs)
            }
            SolidityStmt::Block(stmts) => {
                let mut out = format!("{}{{\n", ind);
                for s in stmts {
                    out.push_str(&Self::emit_stmt(s, indent + 1));
                }
                out.push_str(&format!("{}}}\n", ind));
                out
            }
        }
    }
}
/// A custom Solidity error definition (Solidity 0.8+).
#[derive(Debug, Clone)]
pub struct SolidityError {
    pub name: String,
    pub params: Vec<SolidityParam>,
    pub doc: Option<String>,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SolLivenessInfo {
    pub live_in: Vec<std::collections::HashSet<u32>>,
    pub live_out: Vec<std::collections::HashSet<u32>>,
    pub defs: Vec<std::collections::HashSet<u32>>,
    pub uses: Vec<std::collections::HashSet<u32>>,
}
impl SolLivenessInfo {
    #[allow(dead_code)]
    pub fn new(block_count: usize) -> Self {
        SolLivenessInfo {
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
pub struct SolCacheEntry {
    pub key: String,
    pub data: Vec<u8>,
    pub timestamp: u64,
    pub valid: bool,
}
/// Analysis cache for SolExt.
#[allow(dead_code)]
#[derive(Debug)]
pub struct SolExtCache {
    pub(super) entries: Vec<(u64, Vec<u8>, bool, u32)>,
    pub(super) cap: usize,
    pub(super) total_hits: u64,
    pub(super) total_misses: u64,
}
impl SolExtCache {
    #[allow(dead_code)]
    pub fn new(cap: usize) -> Self {
        Self {
            entries: Vec::new(),
            cap,
            total_hits: 0,
            total_misses: 0,
        }
    }
    #[allow(dead_code)]
    pub fn get(&mut self, key: u64) -> Option<&[u8]> {
        for e in self.entries.iter_mut() {
            if e.0 == key && e.2 {
                e.3 += 1;
                self.total_hits += 1;
                return Some(&e.1);
            }
        }
        self.total_misses += 1;
        None
    }
    #[allow(dead_code)]
    pub fn put(&mut self, key: u64, data: Vec<u8>) {
        if self.entries.len() >= self.cap {
            self.entries.retain(|e| e.2);
            if self.entries.len() >= self.cap {
                self.entries.remove(0);
            }
        }
        self.entries.push((key, data, true, 0));
    }
    #[allow(dead_code)]
    pub fn invalidate(&mut self) {
        for e in self.entries.iter_mut() {
            e.2 = false;
        }
    }
    #[allow(dead_code)]
    pub fn hit_rate(&self) -> f64 {
        let t = self.total_hits + self.total_misses;
        if t == 0 {
            0.0
        } else {
            self.total_hits as f64 / t as f64
        }
    }
    #[allow(dead_code)]
    pub fn live_count(&self) -> usize {
        self.entries.iter().filter(|e| e.2).count()
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SolDepGraph {
    pub(super) nodes: Vec<u32>,
    pub(super) edges: Vec<(u32, u32)>,
}
impl SolDepGraph {
    #[allow(dead_code)]
    pub fn new() -> Self {
        SolDepGraph {
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
/// Solidity ABI type representation.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SolidityType {
    /// `uint256`
    Uint256,
    /// `uint128`
    Uint128,
    /// `uint64`
    Uint64,
    /// `uint32`
    Uint32,
    /// `uint8`
    Uint8,
    /// `int256`
    Int256,
    /// `int128`
    Int128,
    /// `int64`
    Int64,
    /// `int32`
    Int32,
    /// `int8`
    Int8,
    /// `address`
    Address,
    /// `address payable`
    AddressPayable,
    /// `bool`
    Bool,
    /// `bytes`
    Bytes,
    /// `bytes32`
    Bytes32,
    /// `bytes16`
    Bytes16,
    /// `bytes4`
    Bytes4,
    /// `string`
    StringTy,
    /// `mapping(K => V)`
    Mapping(Box<SolidityType>, Box<SolidityType>),
    /// `T[]` — dynamic array
    DynArray(Box<SolidityType>),
    /// `T[N]` — fixed-size array
    FixedArray(Box<SolidityType>, usize),
    /// A named struct or enum type
    Named(String),
    /// `tuple(T0, T1, ...)` — used for ABI encoding
    Tuple(Vec<SolidityType>),
}
impl SolidityType {
    /// Returns true if this type is a reference type (stored in memory/storage).
    pub fn is_reference_type(&self) -> bool {
        matches!(
            self,
            SolidityType::Bytes
                | SolidityType::StringTy
                | SolidityType::Mapping(_, _)
                | SolidityType::DynArray(_)
                | SolidityType::FixedArray(_, _)
                | SolidityType::Named(_)
                | SolidityType::Tuple(_)
        )
    }
    /// Returns the default data location for function parameters of this type.
    pub fn default_param_location(&self) -> &'static str {
        if self.is_reference_type() {
            "memory"
        } else {
            ""
        }
    }
    /// Returns the ABI-canonical type string (for selector computation).
    pub fn abi_canonical(&self) -> String {
        match self {
            SolidityType::Uint256 => "uint256".into(),
            SolidityType::Uint128 => "uint128".into(),
            SolidityType::Uint64 => "uint64".into(),
            SolidityType::Uint32 => "uint32".into(),
            SolidityType::Uint8 => "uint8".into(),
            SolidityType::Int256 => "int256".into(),
            SolidityType::Int128 => "int128".into(),
            SolidityType::Int64 => "int64".into(),
            SolidityType::Int32 => "int32".into(),
            SolidityType::Int8 => "int8".into(),
            SolidityType::Address | SolidityType::AddressPayable => "address".into(),
            SolidityType::Bool => "bool".into(),
            SolidityType::Bytes => "bytes".into(),
            SolidityType::Bytes32 => "bytes32".into(),
            SolidityType::Bytes16 => "bytes16".into(),
            SolidityType::Bytes4 => "bytes4".into(),
            SolidityType::StringTy => "string".into(),
            SolidityType::Mapping(k, v) => {
                format!("mapping({},{})", k.abi_canonical(), v.abi_canonical())
            }
            SolidityType::DynArray(elem) => format!("{}[]", elem.abi_canonical()),
            SolidityType::FixedArray(elem, n) => {
                format!("{}[{}]", elem.abi_canonical(), n)
            }
            SolidityType::Named(name) => name.clone(),
            SolidityType::Tuple(elems) => {
                let inner: Vec<String> = elems.iter().map(|t| t.abi_canonical()).collect();
                format!("({})", inner.join(","))
            }
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct SolPassStats {
    pub total_runs: u32,
    pub successful_runs: u32,
    pub total_changes: u64,
    pub time_ms: u64,
    pub iterations_used: u32,
}
impl SolPassStats {
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
pub struct SolDominatorTree {
    pub idom: Vec<Option<u32>>,
    pub dom_children: Vec<Vec<u32>>,
    pub dom_depth: Vec<u32>,
}
impl SolDominatorTree {
    #[allow(dead_code)]
    pub fn new(size: usize) -> Self {
        SolDominatorTree {
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
#[derive(Debug, Clone)]
pub struct SolPassConfig {
    pub phase: SolPassPhase,
    pub enabled: bool,
    pub max_iterations: u32,
    pub debug_output: bool,
    pub pass_name: String,
}
impl SolPassConfig {
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>, phase: SolPassPhase) -> Self {
        SolPassConfig {
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
/// Visibility of a state variable or function.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Visibility {
    Public,
    Private,
    Internal,
    External,
}
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum SolPassPhase {
    Analysis,
    Transformation,
    Verification,
    Cleanup,
}
impl SolPassPhase {
    #[allow(dead_code)]
    pub fn name(&self) -> &str {
        match self {
            SolPassPhase::Analysis => "analysis",
            SolPassPhase::Transformation => "transformation",
            SolPassPhase::Verification => "verification",
            SolPassPhase::Cleanup => "cleanup",
        }
    }
    #[allow(dead_code)]
    pub fn is_modifying(&self) -> bool {
        matches!(self, SolPassPhase::Transformation | SolPassPhase::Cleanup)
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SolAnalysisCache {
    pub(super) entries: std::collections::HashMap<String, SolCacheEntry>,
    pub(super) max_size: usize,
    pub(super) hits: u64,
    pub(super) misses: u64,
}
impl SolAnalysisCache {
    #[allow(dead_code)]
    pub fn new(max_size: usize) -> Self {
        SolAnalysisCache {
            entries: std::collections::HashMap::new(),
            max_size,
            hits: 0,
            misses: 0,
        }
    }
    #[allow(dead_code)]
    pub fn get(&mut self, key: &str) -> Option<&SolCacheEntry> {
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
            SolCacheEntry {
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
/// Configuration for SolExt passes.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SolExtPassConfig {
    pub name: String,
    pub phase: SolExtPassPhase,
    pub enabled: bool,
    pub max_iterations: usize,
    pub debug: u32,
    pub timeout_ms: Option<u64>,
}
impl SolExtPassConfig {
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            phase: SolExtPassPhase::Middle,
            enabled: true,
            max_iterations: 100,
            debug: 0,
            timeout_ms: None,
        }
    }
    #[allow(dead_code)]
    pub fn with_phase(mut self, phase: SolExtPassPhase) -> Self {
        self.phase = phase;
        self
    }
    #[allow(dead_code)]
    pub fn with_max_iter(mut self, n: usize) -> Self {
        self.max_iterations = n;
        self
    }
    #[allow(dead_code)]
    pub fn with_debug(mut self, d: u32) -> Self {
        self.debug = d;
        self
    }
    #[allow(dead_code)]
    pub fn disabled(mut self) -> Self {
        self.enabled = false;
        self
    }
    #[allow(dead_code)]
    pub fn with_timeout(mut self, ms: u64) -> Self {
        self.timeout_ms = Some(ms);
        self
    }
    #[allow(dead_code)]
    pub fn is_debug_enabled(&self) -> bool {
        self.debug > 0
    }
}
/// Statistics for SolExt passes.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct SolExtPassStats {
    pub iterations: usize,
    pub changed: bool,
    pub nodes_visited: usize,
    pub nodes_modified: usize,
    pub time_ms: u64,
    pub memory_bytes: usize,
    pub errors: usize,
}
impl SolExtPassStats {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    #[allow(dead_code)]
    pub fn visit(&mut self) {
        self.nodes_visited += 1;
    }
    #[allow(dead_code)]
    pub fn modify(&mut self) {
        self.nodes_modified += 1;
        self.changed = true;
    }
    #[allow(dead_code)]
    pub fn iterate(&mut self) {
        self.iterations += 1;
    }
    #[allow(dead_code)]
    pub fn error(&mut self) {
        self.errors += 1;
    }
    #[allow(dead_code)]
    pub fn efficiency(&self) -> f64 {
        if self.nodes_visited == 0 {
            0.0
        } else {
            self.nodes_modified as f64 / self.nodes_visited as f64
        }
    }
    #[allow(dead_code)]
    pub fn merge(&mut self, o: &SolExtPassStats) {
        self.iterations += o.iterations;
        self.changed |= o.changed;
        self.nodes_visited += o.nodes_visited;
        self.nodes_modified += o.nodes_modified;
        self.time_ms += o.time_ms;
        self.memory_bytes = self.memory_bytes.max(o.memory_bytes);
        self.errors += o.errors;
    }
}
/// A Solidity modifier definition.
#[derive(Debug, Clone)]
pub struct SolidityModifier {
    pub name: String,
    pub params: Vec<SolidityParam>,
    pub body: Vec<SolidityStmt>,
    pub doc: Option<String>,
}
/// Pass registry for SolExt.
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct SolExtPassRegistry {
    pub(super) configs: Vec<SolExtPassConfig>,
    pub(super) stats: Vec<SolExtPassStats>,
}
impl SolExtPassRegistry {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    #[allow(dead_code)]
    pub fn register(&mut self, c: SolExtPassConfig) {
        self.stats.push(SolExtPassStats::new());
        self.configs.push(c);
    }
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.configs.len()
    }
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.configs.is_empty()
    }
    #[allow(dead_code)]
    pub fn get(&self, i: usize) -> Option<&SolExtPassConfig> {
        self.configs.get(i)
    }
    #[allow(dead_code)]
    pub fn get_stats(&self, i: usize) -> Option<&SolExtPassStats> {
        self.stats.get(i)
    }
    #[allow(dead_code)]
    pub fn enabled_passes(&self) -> Vec<&SolExtPassConfig> {
        self.configs.iter().filter(|c| c.enabled).collect()
    }
    #[allow(dead_code)]
    pub fn passes_in_phase(&self, ph: &SolExtPassPhase) -> Vec<&SolExtPassConfig> {
        self.configs
            .iter()
            .filter(|c| c.enabled && &c.phase == ph)
            .collect()
    }
    #[allow(dead_code)]
    pub fn total_nodes_visited(&self) -> usize {
        self.stats.iter().map(|s| s.nodes_visited).sum()
    }
    #[allow(dead_code)]
    pub fn any_changed(&self) -> bool {
        self.stats.iter().any(|s| s.changed)
    }
}
/// Contract kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContractKind {
    Contract,
    Abstract,
    Interface,
    Library,
}
/// A Solidity function (or constructor / fallback / receive).
#[derive(Debug, Clone)]
pub struct SolidityFunction {
    /// Function name (empty for constructor/fallback/receive).
    pub name: String,
    /// Input parameters.
    pub params: Vec<SolidityParam>,
    /// Return parameters.
    pub returns: Vec<SolidityParam>,
    /// Visibility.
    pub visibility: Visibility,
    /// State mutability.
    pub mutability: StateMutability,
    /// Whether this is `virtual`.
    pub is_virtual: bool,
    /// Whether this overrides a base function.
    pub is_override: bool,
    /// List of modifier invocations: `(name, args)`.
    pub modifiers: Vec<(String, Vec<SolidityExpr>)>,
    /// Function body statements (empty = abstract/interface).
    pub body: Vec<SolidityStmt>,
    /// NatSpec dev comment.
    pub doc: Option<String>,
}
impl SolidityFunction {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            params: Vec::new(),
            returns: Vec::new(),
            visibility: Visibility::External,
            mutability: StateMutability::NonPayable,
            is_virtual: false,
            is_override: false,
            modifiers: Vec::new(),
            body: Vec::new(),
            doc: None,
        }
    }
    /// ABI signature for selector computation: `name(T0,T1,...)`
    pub fn abi_signature(&self) -> String {
        let params: Vec<String> = self.params.iter().map(|p| p.ty.abi_canonical()).collect();
        format!("{}({})", self.name, params.join(","))
    }
    /// Simple 4-byte selector (djb2-based placeholder, not real keccak).
    pub fn selector(&self) -> [u8; 4] {
        let sig = self.abi_signature();
        let mut h: u32 = 5381;
        for b in sig.bytes() {
            h = h.wrapping_shl(5).wrapping_add(h).wrapping_add(b as u32);
        }
        h.to_be_bytes()
    }
}
/// Solidity statement AST node.
#[derive(Debug, Clone)]
pub enum SolidityStmt {
    /// Variable declaration: `T loc name = expr;`
    VarDecl {
        ty: SolidityType,
        location: Option<String>,
        name: String,
        init: Option<SolidityExpr>,
    },
    /// Assignment: `lhs = rhs;`
    Assign(SolidityExpr, SolidityExpr),
    /// Compound assignment: `lhs += rhs;`
    CompoundAssign(String, SolidityExpr, SolidityExpr),
    /// Expression statement: `f();`
    ExprStmt(SolidityExpr),
    /// `return expr;`
    Return(Option<SolidityExpr>),
    /// `if (cond) { then } else { else_ }`
    If(SolidityExpr, Vec<SolidityStmt>, Vec<SolidityStmt>),
    /// `while (cond) { body }`
    While(SolidityExpr, Vec<SolidityStmt>),
    /// `for (init; cond; update) { body }`
    For(
        Option<Box<SolidityStmt>>,
        Option<SolidityExpr>,
        Option<Box<SolidityStmt>>,
        Vec<SolidityStmt>,
    ),
    /// `do { body } while (cond);`
    DoWhile(Vec<SolidityStmt>, SolidityExpr),
    /// `emit EventName(args...);`
    Emit(String, Vec<SolidityExpr>),
    /// `revert ErrorName(args...);`
    Revert(String, Vec<SolidityExpr>),
    /// `require(cond, msg);`
    Require(SolidityExpr, Option<String>),
    /// `assert(cond);`
    Assert(SolidityExpr),
    /// `break;`
    Break,
    /// `continue;`
    Continue,
    /// `unchecked { stmts }`
    Unchecked(Vec<SolidityStmt>),
    /// `assembly { body }`
    Assembly(String),
    /// Multi-return: `(a, b) = f();`
    MultiAssign(Vec<SolidityExpr>, SolidityExpr),
    /// Block of statements `{ ... }`
    Block(Vec<SolidityStmt>),
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SolWorklist {
    pub(super) items: std::collections::VecDeque<u32>,
    pub(super) in_worklist: std::collections::HashSet<u32>,
}
impl SolWorklist {
    #[allow(dead_code)]
    pub fn new() -> Self {
        SolWorklist {
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
/// A complete Solidity contract, interface, abstract contract, or library.
#[derive(Debug, Clone)]
pub struct SolidityContract {
    pub name: String,
    pub kind: ContractKind,
    /// Inheritance list.
    pub bases: Vec<String>,
    pub structs: Vec<SolidityStruct>,
    pub enums: Vec<SolidityEnum>,
    pub events: Vec<SolidityEvent>,
    pub errors: Vec<SolidityError>,
    pub state_vars: Vec<SolidityStateVar>,
    pub modifiers: Vec<SolidityModifier>,
    pub functions: Vec<SolidityFunction>,
    /// Constructor (if present).
    pub constructor: Option<SolidityFunction>,
    /// Receive function (if present).
    pub receive: Option<SolidityFunction>,
    /// Fallback function (if present).
    pub fallback: Option<SolidityFunction>,
    /// NatSpec title/dev comment.
    pub doc: Option<String>,
}
impl SolidityContract {
    pub fn new(name: impl Into<String>, kind: ContractKind) -> Self {
        Self {
            name: name.into(),
            kind,
            bases: Vec::new(),
            structs: Vec::new(),
            enums: Vec::new(),
            events: Vec::new(),
            errors: Vec::new(),
            state_vars: Vec::new(),
            modifiers: Vec::new(),
            functions: Vec::new(),
            constructor: None,
            receive: None,
            fallback: None,
            doc: None,
        }
    }
}
/// A Solidity event definition.
#[derive(Debug, Clone)]
pub struct SolidityEvent {
    pub name: String,
    /// `(ty, indexed, name)`
    pub fields: Vec<(SolidityType, bool, String)>,
    pub anonymous: bool,
    pub doc: Option<String>,
}
/// Liveness analysis for SolExt.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct SolExtLiveness {
    pub live_in: Vec<Vec<usize>>,
    pub live_out: Vec<Vec<usize>>,
    pub defs: Vec<Vec<usize>>,
    pub uses: Vec<Vec<usize>>,
}
impl SolExtLiveness {
    #[allow(dead_code)]
    pub fn new(n: usize) -> Self {
        Self {
            live_in: vec![Vec::new(); n],
            live_out: vec![Vec::new(); n],
            defs: vec![Vec::new(); n],
            uses: vec![Vec::new(); n],
        }
    }
    #[allow(dead_code)]
    pub fn live_in(&self, b: usize, v: usize) -> bool {
        self.live_in.get(b).map(|s| s.contains(&v)).unwrap_or(false)
    }
    #[allow(dead_code)]
    pub fn live_out(&self, b: usize, v: usize) -> bool {
        self.live_out
            .get(b)
            .map(|s| s.contains(&v))
            .unwrap_or(false)
    }
    #[allow(dead_code)]
    pub fn add_def(&mut self, b: usize, v: usize) {
        if let Some(s) = self.defs.get_mut(b) {
            if !s.contains(&v) {
                s.push(v);
            }
        }
    }
    #[allow(dead_code)]
    pub fn add_use(&mut self, b: usize, v: usize) {
        if let Some(s) = self.uses.get_mut(b) {
            if !s.contains(&v) {
                s.push(v);
            }
        }
    }
    #[allow(dead_code)]
    pub fn var_is_used_in_block(&self, b: usize, v: usize) -> bool {
        self.uses.get(b).map(|s| s.contains(&v)).unwrap_or(false)
    }
    #[allow(dead_code)]
    pub fn var_is_def_in_block(&self, b: usize, v: usize) -> bool {
        self.defs.get(b).map(|s| s.contains(&v)).unwrap_or(false)
    }
}
/// Constant folding helper for SolExt.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct SolExtConstFolder {
    pub(super) folds: usize,
    pub(super) failures: usize,
    pub(super) enabled: bool,
}
impl SolExtConstFolder {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            folds: 0,
            failures: 0,
            enabled: true,
        }
    }
    #[allow(dead_code)]
    pub fn add_i64(&mut self, a: i64, b: i64) -> Option<i64> {
        self.folds += 1;
        a.checked_add(b)
    }
    #[allow(dead_code)]
    pub fn sub_i64(&mut self, a: i64, b: i64) -> Option<i64> {
        self.folds += 1;
        a.checked_sub(b)
    }
    #[allow(dead_code)]
    pub fn mul_i64(&mut self, a: i64, b: i64) -> Option<i64> {
        self.folds += 1;
        a.checked_mul(b)
    }
    #[allow(dead_code)]
    pub fn div_i64(&mut self, a: i64, b: i64) -> Option<i64> {
        if b == 0 {
            self.failures += 1;
            None
        } else {
            self.folds += 1;
            a.checked_div(b)
        }
    }
    #[allow(dead_code)]
    pub fn rem_i64(&mut self, a: i64, b: i64) -> Option<i64> {
        if b == 0 {
            self.failures += 1;
            None
        } else {
            self.folds += 1;
            a.checked_rem(b)
        }
    }
    #[allow(dead_code)]
    pub fn neg_i64(&mut self, a: i64) -> Option<i64> {
        self.folds += 1;
        a.checked_neg()
    }
    #[allow(dead_code)]
    pub fn shl_i64(&mut self, a: i64, s: u32) -> Option<i64> {
        if s >= 64 {
            self.failures += 1;
            None
        } else {
            self.folds += 1;
            a.checked_shl(s)
        }
    }
    #[allow(dead_code)]
    pub fn shr_i64(&mut self, a: i64, s: u32) -> Option<i64> {
        if s >= 64 {
            self.failures += 1;
            None
        } else {
            self.folds += 1;
            a.checked_shr(s)
        }
    }
    #[allow(dead_code)]
    pub fn and_i64(&mut self, a: i64, b: i64) -> i64 {
        self.folds += 1;
        a & b
    }
    #[allow(dead_code)]
    pub fn or_i64(&mut self, a: i64, b: i64) -> i64 {
        self.folds += 1;
        a | b
    }
    #[allow(dead_code)]
    pub fn xor_i64(&mut self, a: i64, b: i64) -> i64 {
        self.folds += 1;
        a ^ b
    }
    #[allow(dead_code)]
    pub fn not_i64(&mut self, a: i64) -> i64 {
        self.folds += 1;
        !a
    }
    #[allow(dead_code)]
    pub fn fold_count(&self) -> usize {
        self.folds
    }
    #[allow(dead_code)]
    pub fn failure_count(&self) -> usize {
        self.failures
    }
    #[allow(dead_code)]
    pub fn enable(&mut self) {
        self.enabled = true;
    }
    #[allow(dead_code)]
    pub fn disable(&mut self) {
        self.enabled = false;
    }
    #[allow(dead_code)]
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}
/// Dependency graph for SolExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SolExtDepGraph {
    pub(super) n: usize,
    pub(super) adj: Vec<Vec<usize>>,
    pub(super) rev: Vec<Vec<usize>>,
    pub(super) edge_count: usize,
}
impl SolExtDepGraph {
    #[allow(dead_code)]
    pub fn new(n: usize) -> Self {
        Self {
            n,
            adj: vec![Vec::new(); n],
            rev: vec![Vec::new(); n],
            edge_count: 0,
        }
    }
    #[allow(dead_code)]
    pub fn add_edge(&mut self, from: usize, to: usize) {
        if from < self.n && to < self.n {
            if !self.adj[from].contains(&to) {
                self.adj[from].push(to);
                self.rev[to].push(from);
                self.edge_count += 1;
            }
        }
    }
    #[allow(dead_code)]
    pub fn succs(&self, n: usize) -> &[usize] {
        self.adj.get(n).map(|v| v.as_slice()).unwrap_or(&[])
    }
    #[allow(dead_code)]
    pub fn preds(&self, n: usize) -> &[usize] {
        self.rev.get(n).map(|v| v.as_slice()).unwrap_or(&[])
    }
    #[allow(dead_code)]
    pub fn topo_sort(&self) -> Option<Vec<usize>> {
        let mut deg: Vec<usize> = (0..self.n).map(|i| self.rev[i].len()).collect();
        let mut q: std::collections::VecDeque<usize> =
            (0..self.n).filter(|&i| deg[i] == 0).collect();
        let mut out = Vec::with_capacity(self.n);
        while let Some(u) = q.pop_front() {
            out.push(u);
            for &v in &self.adj[u] {
                deg[v] -= 1;
                if deg[v] == 0 {
                    q.push_back(v);
                }
            }
        }
        if out.len() == self.n {
            Some(out)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn has_cycle(&self) -> bool {
        self.topo_sort().is_none()
    }
    #[allow(dead_code)]
    pub fn reachable(&self, start: usize) -> Vec<usize> {
        let mut vis = vec![false; self.n];
        let mut stk = vec![start];
        let mut out = Vec::new();
        while let Some(u) = stk.pop() {
            if u < self.n && !vis[u] {
                vis[u] = true;
                out.push(u);
                for &v in &self.adj[u] {
                    if !vis[v] {
                        stk.push(v);
                    }
                }
            }
        }
        out
    }
    #[allow(dead_code)]
    pub fn scc(&self) -> Vec<Vec<usize>> {
        let mut visited = vec![false; self.n];
        let mut order = Vec::new();
        for i in 0..self.n {
            if !visited[i] {
                let mut stk = vec![(i, 0usize)];
                while let Some((u, idx)) = stk.last_mut() {
                    if !visited[*u] {
                        visited[*u] = true;
                    }
                    if *idx < self.adj[*u].len() {
                        let v = self.adj[*u][*idx];
                        *idx += 1;
                        if !visited[v] {
                            stk.push((v, 0));
                        }
                    } else {
                        order.push(*u);
                        stk.pop();
                    }
                }
            }
        }
        let mut comp = vec![usize::MAX; self.n];
        let mut components: Vec<Vec<usize>> = Vec::new();
        for &start in order.iter().rev() {
            if comp[start] == usize::MAX {
                let cid = components.len();
                let mut component = Vec::new();
                let mut stk = vec![start];
                while let Some(u) = stk.pop() {
                    if comp[u] == usize::MAX {
                        comp[u] = cid;
                        component.push(u);
                        for &v in &self.rev[u] {
                            if comp[v] == usize::MAX {
                                stk.push(v);
                            }
                        }
                    }
                }
                components.push(component);
            }
        }
        components
    }
    #[allow(dead_code)]
    pub fn node_count(&self) -> usize {
        self.n
    }
    #[allow(dead_code)]
    pub fn edge_count(&self) -> usize {
        self.edge_count
    }
}
#[allow(dead_code)]
pub struct SolConstantFoldingHelper;
impl SolConstantFoldingHelper {
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
