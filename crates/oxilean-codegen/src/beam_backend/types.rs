//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;
use std::collections::HashMap;

use super::functions::*;
use std::collections::HashSet;

/// A BEAM value operand (register or immediate).
#[derive(Debug, Clone)]
pub enum BeamVal {
    /// A register
    Reg(BeamReg),
    /// Integer immediate
    Int(i64),
    /// Float immediate
    Float(f64),
    /// Atom immediate
    Atom(String),
    /// Nil immediate
    Nil,
    /// Literal term from literal pool
    Literal(u32),
}
/// Represents a BEAM process (actor) in the actor model.
///
/// BEAM processes are lightweight and communicate via asynchronous message
/// passing through mailboxes.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct BeamProcess {
    /// Unique process identifier (symbolic)
    pub pid_sym: String,
    /// Module where this process is defined
    pub module: String,
    /// Startup function name
    pub init_fn: String,
    /// Initial arguments
    pub init_args: Vec<BeamExpr>,
    /// Whether this process is linked (crashes propagate)
    pub linked: bool,
    /// Whether this process is monitored
    pub monitored: bool,
    /// Process dictionary entries (name → value)
    pub dictionary: Vec<(String, BeamExpr)>,
    /// Trap-exit flag: catches EXIT signals as messages
    pub trap_exit: bool,
}
impl BeamProcess {
    /// Create a new process description.
    #[allow(dead_code)]
    pub fn new(
        pid_sym: impl Into<String>,
        module: impl Into<String>,
        init_fn: impl Into<String>,
    ) -> Self {
        BeamProcess {
            pid_sym: pid_sym.into(),
            module: module.into(),
            init_fn: init_fn.into(),
            init_args: Vec::new(),
            linked: false,
            monitored: false,
            dictionary: Vec::new(),
            trap_exit: false,
        }
    }
    /// Add an initial argument.
    #[allow(dead_code)]
    pub fn with_arg(mut self, arg: BeamExpr) -> Self {
        self.init_args.push(arg);
        self
    }
    /// Mark the process as linked.
    #[allow(dead_code)]
    pub fn linked(mut self) -> Self {
        self.linked = true;
        self
    }
    /// Enable trap_exit.
    #[allow(dead_code)]
    pub fn trap_exit(mut self) -> Self {
        self.trap_exit = true;
        self
    }
    /// Emit the `spawn/3` call that creates this process.
    #[allow(dead_code)]
    pub fn emit_spawn(&self) -> BeamExpr {
        let args_list = self
            .init_args
            .iter()
            .cloned()
            .fold(BeamExpr::Nil, |tail, head| {
                BeamExpr::Cons(Box::new(head), Box::new(tail))
            });
        let spawn_fn = if self.linked { "spawn_link" } else { "spawn" };
        BeamExpr::Call {
            module: Some("erlang".to_string()),
            func: spawn_fn.to_string(),
            args: vec![
                BeamExpr::LitAtom(self.module.clone()),
                BeamExpr::LitAtom(self.init_fn.clone()),
                args_list,
            ],
        }
    }
}
/// ETS table access permissions.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EtsAccess {
    /// Only owner can read/write
    Private,
    /// All can read, owner can write
    Protected,
    /// All can read and write
    Public,
}
/// Description of an ETS table.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct EtsTable {
    /// Table name (atom)
    pub name: String,
    /// Table type
    pub table_type: EtsType,
    /// Access permissions
    pub access: EtsAccess,
    /// Whether the table is named (accessible by name instead of ref)
    pub named_table: bool,
    /// Key position (1-indexed tuple position)
    pub key_pos: u32,
}
impl EtsTable {
    /// Create a new named public ETS set table.
    #[allow(dead_code)]
    pub fn new_set(name: impl Into<String>) -> Self {
        EtsTable {
            name: name.into(),
            table_type: EtsType::Set,
            access: EtsAccess::Public,
            named_table: true,
            key_pos: 1,
        }
    }
    /// Emit the `ets:new/2` call that creates this table.
    #[allow(dead_code)]
    pub fn emit_new(&self) -> BeamExpr {
        let mut opts = vec![BeamExpr::LitAtom(self.table_type.to_string())];
        if self.named_table {
            opts.push(BeamExpr::LitAtom("named_table".to_string()));
        }
        opts.push(BeamExpr::LitAtom(self.access.to_string()));
        if self.key_pos != 1 {
            opts.push(BeamExpr::Tuple(vec![
                BeamExpr::LitAtom("keypos".to_string()),
                BeamExpr::LitInt(self.key_pos as i64),
            ]));
        }
        let opts_list = opts.into_iter().fold(BeamExpr::Nil, |tail, head| {
            BeamExpr::Cons(Box::new(head), Box::new(tail))
        });
        BeamExpr::Call {
            module: Some("ets".to_string()),
            func: "new".to_string(),
            args: vec![BeamExpr::LitAtom(self.name.clone()), opts_list],
        }
    }
    /// Emit `ets:insert/2` call.
    #[allow(dead_code)]
    pub fn emit_insert(&self, tuple: BeamExpr) -> BeamExpr {
        BeamExpr::Call {
            module: Some("ets".to_string()),
            func: "insert".to_string(),
            args: vec![BeamExpr::LitAtom(self.name.clone()), tuple],
        }
    }
    /// Emit `ets:lookup/2` call.
    #[allow(dead_code)]
    pub fn emit_lookup(&self, key: BeamExpr) -> BeamExpr {
        BeamExpr::Call {
            module: Some("ets".to_string()),
            func: "lookup".to_string(),
            args: vec![BeamExpr::LitAtom(self.name.clone()), key],
        }
    }
    /// Emit `ets:delete/2` call.
    #[allow(dead_code)]
    pub fn emit_delete(&self, key: BeamExpr) -> BeamExpr {
        BeamExpr::Call {
            module: Some("ets".to_string()),
            func: "delete".to_string(),
            args: vec![BeamExpr::LitAtom(self.name.clone()), key],
        }
    }
}
/// Result of tail-call analysis for a BEAM function.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct TailCallInfo {
    /// Number of self-recursive tail calls found
    pub self_tail_calls: u32,
    /// Names of external functions called in tail position
    pub external_tails: Vec<String>,
    /// Whether the function is tail-recursive (pure loop)
    pub is_tail_recursive: bool,
}
impl TailCallInfo {
    /// Create empty tail call info.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    /// Record a self-recursive tail call.
    #[allow(dead_code)]
    pub fn add_self_tail(&mut self) {
        self.self_tail_calls += 1;
        self.is_tail_recursive = true;
    }
    /// Record an external tail call.
    #[allow(dead_code)]
    pub fn add_external_tail(&mut self, name: impl Into<String>) {
        self.external_tails.push(name.into());
    }
    /// Returns true if there are any tail calls.
    #[allow(dead_code)]
    pub fn has_tail_calls(&self) -> bool {
        self.self_tail_calls > 0 || !self.external_tails.is_empty()
    }
}
/// A pattern-matching clause in a case expression.
#[derive(Debug, Clone)]
pub struct BeamClause {
    /// Pattern to match against
    pub pattern: BeamPattern,
    /// Optional guard expression
    pub guard: Option<BeamExpr>,
    /// Body expression evaluated on match
    pub body: BeamExpr,
}
/// Context for emitting BEAM / Core Erlang code.
pub struct BeamEmitCtx {
    /// Current label counter
    pub(super) label_counter: u32,
    /// Current variable counter (for fresh names)
    pub(super) var_counter: u32,
    /// Indent level for pretty-printing
    pub(super) indent: usize,
    /// Output buffer
    pub(super) output: String,
}
impl BeamEmitCtx {
    pub fn new() -> Self {
        BeamEmitCtx {
            label_counter: 1,
            var_counter: 0,
            indent: 0,
            output: String::new(),
        }
    }
    pub(super) fn fresh_label(&mut self) -> u32 {
        let l = self.label_counter;
        self.label_counter += 1;
        l
    }
    pub(super) fn fresh_var(&mut self) -> String {
        let v = self.var_counter;
        self.var_counter += 1;
        format!("_V{}", v)
    }
    pub(super) fn indent_str(&self) -> String {
        "  ".repeat(self.indent)
    }
    pub(super) fn emit_line(&mut self, line: &str) {
        let indent = self.indent_str();
        self.output.push_str(&indent);
        self.output.push_str(line);
        self.output.push('\n');
    }
    pub(super) fn indented<F: FnOnce(&mut Self)>(&mut self, f: F) {
        self.indent += 1;
        f(self);
        self.indent -= 1;
    }
}
/// Normalizes BEAM patterns for comparison and deduplication.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct PatternNormalizer {
    /// Counter for generating fresh wildcard names
    pub(super) wildcard_counter: u32,
}
impl PatternNormalizer {
    /// Create a new normalizer.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    /// Normalize a pattern (replace all variables with canonical names).
    #[allow(dead_code)]
    pub fn normalize(&mut self, pat: BeamPattern) -> BeamPattern {
        match pat {
            BeamPattern::Var(_) => {
                let n = self.wildcard_counter;
                self.wildcard_counter += 1;
                BeamPattern::Var(format!("_N{}", n))
            }
            BeamPattern::Alias(_, inner) => self.normalize(*inner),
            BeamPattern::Cons(h, t) => {
                let hn = self.normalize(*h);
                let tn = self.normalize(*t);
                BeamPattern::Cons(Box::new(hn), Box::new(tn))
            }
            BeamPattern::Tuple(pats) => {
                BeamPattern::Tuple(pats.into_iter().map(|p| self.normalize(p)).collect())
            }
            other => other,
        }
    }
    /// Check whether two patterns are structurally equivalent after normalization.
    #[allow(dead_code)]
    pub fn equivalent(&mut self, a: BeamPattern, b: BeamPattern) -> bool {
        let na = self.normalize(a);
        let nb = self.normalize(b);
        patterns_structurally_equal(&na, &nb)
    }
}
/// BEAM VM type representation.
///
/// BEAM is dynamically typed; these types are used for documentation
/// and static analysis purposes within the code generator.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BeamType {
    /// Fixed-precision integer (maps to Erlang `integer()`)
    Integer,
    /// Floating-point number (maps to Erlang `float()`)
    Float,
    /// Atom (interned symbol, maps to Erlang `atom()`)
    Atom,
    /// Process identifier (maps to Erlang `pid()`)
    Pid,
    /// Port identifier (maps to Erlang `port()`)
    Port,
    /// Unique reference (maps to Erlang `reference()`)
    Reference,
    /// Binary data (maps to Erlang `binary()`)
    Binary,
    /// Linked list (maps to Erlang `list()`)
    List(Box<BeamType>),
    /// Heterogeneous tuple (maps to Erlang `tuple()`)
    Tuple(Vec<BeamType>),
    /// Key-value map (maps to Erlang `map()`)
    Map(Box<BeamType>, Box<BeamType>),
    /// First-class function value (maps to Erlang `fun()`)
    Fun(Vec<BeamType>, Box<BeamType>),
    /// Any type (Erlang `any()`)
    Any,
    /// No return type (Erlang `none()`)
    None,
    /// Union of types
    Union(Vec<BeamType>),
    /// Named type alias or user-defined type
    Named(String),
}
/// A function in a BEAM module.
#[derive(Debug, Clone)]
pub struct BeamFunction {
    /// Function name (atom)
    pub name: String,
    /// Number of formal parameters
    pub arity: usize,
    /// Core Erlang style clauses (pattern, guard, body)
    pub clauses: Vec<BeamClause>,
    /// Optional parameter names (for documentation)
    pub params: Vec<String>,
    /// Key-value annotations (e.g., `{file, "foo.erl"}`)
    pub annotations: Vec<(String, String)>,
    /// Whether this function is exported
    pub exported: bool,
    /// Low-level instruction sequence (populated by lowering pass)
    pub instrs: Vec<BeamInstr>,
    /// Number of Y-register (stack) slots needed
    pub frame_size: u32,
    /// Return type annotation
    pub return_type: Option<BeamType>,
    /// Parameter type annotations
    pub param_types: Vec<BeamType>,
}
impl BeamFunction {
    /// Create a new function with the given name and arity.
    pub fn new(name: impl Into<String>, arity: usize) -> Self {
        BeamFunction {
            name: name.into(),
            arity,
            clauses: Vec::new(),
            params: Vec::new(),
            annotations: Vec::new(),
            exported: false,
            instrs: Vec::new(),
            frame_size: 0,
            return_type: None,
            param_types: Vec::new(),
        }
    }
    /// Add a clause to this function.
    pub fn add_clause(&mut self, clause: BeamClause) {
        self.clauses.push(clause);
    }
    /// Add an annotation.
    pub fn annotate(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.annotations.push((key.into(), value.into()));
    }
    /// Mark as exported.
    pub fn export(&mut self) {
        self.exported = true;
    }
    /// Get the function's name/arity key.
    pub fn key(&self) -> String {
        format!("{}/{}", self.name, self.arity)
    }
}
/// Eliminates unexported, unreachable functions from a BEAM module.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct BeamDeadEliminator {
    /// Set of known reachable function names
    pub(super) reachable: std::collections::HashSet<String>,
}
impl BeamDeadEliminator {
    /// Create a new eliminator.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    /// Mark all exported functions as reachable entry points.
    #[allow(dead_code)]
    pub fn seed_exports(&mut self, module: &BeamModule) {
        for (name, arity) in &module.exports {
            self.reachable.insert(format!("{}/{}", name, arity));
        }
    }
    /// Eliminate dead (unreachable) functions from the module.
    #[allow(dead_code)]
    pub fn eliminate(&self, module: BeamModule) -> BeamModule {
        let mut result = BeamModule::new(module.name.clone());
        result.attributes = module.attributes;
        result.exports = module.exports;
        result.on_load = module.on_load;
        result.compile_info = module.compile_info;
        for func in module.functions {
            let key = format!("{}/{}", func.name, func.arity);
            if self.reachable.contains(&key) || func.exported {
                result.functions.push(func);
            }
        }
        result
    }
    /// Number of eliminated functions from a before/after comparison.
    #[allow(dead_code)]
    pub fn eliminated_count(before: &BeamModule, after: &BeamModule) -> usize {
        before.functions.len().saturating_sub(after.functions.len())
    }
}
/// Helper for building Erlang module attributes.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct AttributeBuilder {
    pub(super) attrs: Vec<(String, String)>,
}
impl AttributeBuilder {
    /// Create a new attribute builder.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    /// Add a `vsn` attribute (module version).
    #[allow(dead_code)]
    pub fn vsn(mut self, version: impl Into<String>) -> Self {
        self.attrs.push(("vsn".into(), version.into()));
        self
    }
    /// Add a `author` attribute.
    #[allow(dead_code)]
    pub fn author(mut self, name: impl Into<String>) -> Self {
        self.attrs.push(("author".into(), name.into()));
        self
    }
    /// Add a `compile` attribute (e.g., `{compile, [debug_info]}`).
    #[allow(dead_code)]
    pub fn compile(mut self, option: impl Into<String>) -> Self {
        self.attrs.push(("compile".into(), option.into()));
        self
    }
    /// Add a custom attribute.
    #[allow(dead_code)]
    pub fn custom(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.attrs.push((key.into(), value.into()));
        self
    }
    /// Build the attributes vector.
    #[allow(dead_code)]
    pub fn build(self) -> Vec<(String, String)> {
        self.attrs
    }
    /// Apply all attributes to a module.
    #[allow(dead_code)]
    pub fn apply(self, module: &mut BeamModule) {
        for (k, v) in self.build() {
            module.add_attribute(k, v);
        }
    }
}
/// Patterns for BEAM case expressions.
#[derive(Debug, Clone)]
pub enum BeamPattern {
    /// Wildcard: matches anything, binds nothing
    Wildcard,
    /// Variable binding: matches anything, binds to name
    Var(String),
    /// Literal integer
    LitInt(i64),
    /// Literal atom
    LitAtom(String),
    /// Literal string
    LitString(String),
    /// Nil pattern `[]`
    Nil,
    /// Cons pattern `[Head | Tail]`
    Cons(Box<BeamPattern>, Box<BeamPattern>),
    /// Tuple pattern `{P1, P2, ...}`
    Tuple(Vec<BeamPattern>),
    /// Map pattern `#{Key := Var, ...}`
    MapPat(Vec<(BeamExpr, BeamPattern)>),
    /// Alias pattern `Var = Pattern`
    Alias(String, Box<BeamPattern>),
}
/// Endianness for binary segments.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BeamEndian {
    Big,
    Little,
    Native,
}
/// A lightweight type-checking context for BEAM expressions.
///
/// This performs best-effort type inference for documentation purposes.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct BeamTypeCtx {
    /// Variable type environment
    pub(super) env: HashMap<String, BeamType>,
    /// Known function types
    pub(super) fun_types: HashMap<String, (Vec<BeamType>, BeamType)>,
}
impl BeamTypeCtx {
    /// Create an empty type context.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    /// Bind a variable to a type.
    #[allow(dead_code)]
    pub fn bind(&mut self, var: impl Into<String>, ty: BeamType) {
        self.env.insert(var.into(), ty);
    }
    /// Look up the type of a variable.
    #[allow(dead_code)]
    pub fn lookup(&self, var: &str) -> Option<&BeamType> {
        self.env.get(var)
    }
    /// Register a function's type signature.
    #[allow(dead_code)]
    pub fn register_fun(&mut self, name: impl Into<String>, params: Vec<BeamType>, ret: BeamType) {
        self.fun_types.insert(name.into(), (params, ret));
    }
    /// Infer the type of a BeamExpr (returns `Any` when unknown).
    #[allow(dead_code)]
    pub fn infer(&self, expr: &BeamExpr) -> BeamType {
        match expr {
            BeamExpr::LitInt(_) => BeamType::Integer,
            BeamExpr::LitFloat(_) => BeamType::Float,
            BeamExpr::LitAtom(_) => BeamType::Atom,
            BeamExpr::LitString(_) => BeamType::Named("binary".to_string()),
            BeamExpr::Nil => BeamType::List(Box::new(BeamType::Any)),
            BeamExpr::Var(name) => self
                .env
                .get(name.as_str())
                .cloned()
                .unwrap_or(BeamType::Any),
            BeamExpr::Tuple(elems) => {
                BeamType::Tuple(elems.iter().map(|e| self.infer(e)).collect())
            }
            BeamExpr::Cons(head, _) => BeamType::List(Box::new(self.infer(head))),
            BeamExpr::Map(_) => BeamType::Map(Box::new(BeamType::Any), Box::new(BeamType::Any)),
            BeamExpr::Fun { params, .. } => BeamType::Fun(
                params.iter().map(|_| BeamType::Any).collect(),
                Box::new(BeamType::Any),
            ),
            _ => BeamType::Any,
        }
    }
    /// Merge another context (e.g., from two branches).
    #[allow(dead_code)]
    pub fn merge(&self, other: &BeamTypeCtx) -> BeamTypeCtx {
        let mut merged = self.clone();
        for (k, v) in &other.env {
            if let Some(existing) = merged.env.get(k) {
                if existing != v {
                    merged.env.insert(k.clone(), BeamType::Any);
                }
            } else {
                merged.env.insert(k.clone(), v.clone());
            }
        }
        merged
    }
}
/// Low-level BEAM instruction set representation.
///
/// These correspond to actual BEAM opcodes as documented in
/// <https://github.com/erlang/otp/blob/master/lib/compiler/src/beam_opcodes.erl>
#[derive(Debug, Clone)]
pub enum BeamInstr {
    /// `label L` — defines a jump label
    Label(u32),
    /// `func_info Mod Func Arity` — function header
    FuncInfo {
        module: String,
        function: String,
        arity: u32,
    },
    /// `call Arity Label` — local function call
    Call { arity: u32, label: u32 },
    /// `call_last Arity Label Deallocate` — tail call
    CallLast {
        arity: u32,
        label: u32,
        deallocate: u32,
    },
    /// `call_ext Arity Destination` — external call
    CallExt {
        arity: u32,
        destination: BeamExtFunc,
    },
    /// `call_ext_last Arity Destination Deallocate` — external tail call
    CallExtLast {
        arity: u32,
        destination: BeamExtFunc,
        deallocate: u32,
    },
    /// `call_fun Arity` — call a fun value
    CallFun { arity: u32 },
    /// `move Source Destination` — move a register or literal to register
    Move { src: BeamReg, dst: BeamReg },
    /// `put_tuple Arity Destination` — begin tuple construction
    PutTuple { arity: u32, dst: BeamReg },
    /// `put Value` — add element to tuple under construction
    Put(BeamVal),
    /// `get_tuple_element Source Index Destination`
    GetTupleElement {
        src: BeamReg,
        index: u32,
        dst: BeamReg,
    },
    /// `set_tuple_element Value Tuple Index`
    SetTupleElement {
        value: BeamVal,
        tuple: BeamReg,
        index: u32,
    },
    /// `is_eq FailLabel Arg1 Arg2` — equality test
    IsEq {
        fail: u32,
        lhs: BeamVal,
        rhs: BeamVal,
    },
    /// `is_eq_exact FailLabel Arg1 Arg2` — strict equality test
    IsEqExact {
        fail: u32,
        lhs: BeamVal,
        rhs: BeamVal,
    },
    /// `is_ne FailLabel Arg1 Arg2` — inequality test
    IsNe {
        fail: u32,
        lhs: BeamVal,
        rhs: BeamVal,
    },
    /// `is_lt FailLabel Arg1 Arg2` — less-than test
    IsLt {
        fail: u32,
        lhs: BeamVal,
        rhs: BeamVal,
    },
    /// `is_ge FailLabel Arg1 Arg2` — greater-than-or-equal test
    IsGe {
        fail: u32,
        lhs: BeamVal,
        rhs: BeamVal,
    },
    /// `is_integer FailLabel Arg` — type check
    IsInteger { fail: u32, arg: BeamVal },
    /// `is_float FailLabel Arg`
    IsFloat { fail: u32, arg: BeamVal },
    /// `is_atom FailLabel Arg`
    IsAtom { fail: u32, arg: BeamVal },
    /// `is_nil FailLabel Arg`
    IsNil { fail: u32, arg: BeamVal },
    /// `is_list FailLabel Arg`
    IsList { fail: u32, arg: BeamVal },
    /// `is_tuple FailLabel Arg`
    IsTuple { fail: u32, arg: BeamVal },
    /// `is_binary FailLabel Arg`
    IsBinary { fail: u32, arg: BeamVal },
    /// `is_function FailLabel Arg`
    IsFunction { fail: u32, arg: BeamVal },
    /// `jump Label` — unconditional jump
    Jump(u32),
    /// `return` — return X0
    Return,
    /// `send` — send message: `X0 ! X1`
    Send,
    /// `remove_message` — consume current message
    RemoveMessage,
    /// `loop_rec FailLabel Destination` — receive loop
    LoopRec { fail: u32, dst: BeamReg },
    /// `wait Label` — wait for message
    Wait(u32),
    /// `wait_timeout FailLabel Time`
    WaitTimeout { fail: u32, timeout: BeamVal },
    /// `gc_bif Name FailLabel Live Args Destination` — garbage-collecting BIF
    GcBif {
        name: String,
        fail: u32,
        live: u32,
        args: Vec<BeamVal>,
        dst: BeamReg,
    },
    /// `bif0 Name Destination` — 0-argument BIF
    Bif0 { name: String, dst: BeamReg },
    /// `allocate StackNeed Live` — allocate stack frame
    Allocate { stack_need: u32, live: u32 },
    /// `deallocate N` — deallocate stack frame
    Deallocate(u32),
    /// `init Destination` — initialize to nil
    Init(BeamReg),
    /// `make_fun2 Index` — create closure from lambda table
    MakeFun2(u32),
    /// `get_list Source Head Tail` — deconstruct list
    GetList {
        src: BeamReg,
        head: BeamReg,
        tail: BeamReg,
    },
    /// `put_list Head Tail Destination` — construct list cell
    PutList {
        head: BeamVal,
        tail: BeamVal,
        dst: BeamReg,
    },
    /// `raise Class Reason` — raise exception
    Raise { class: BeamVal, reason: BeamVal },
    /// `try Label Register` — begin try block
    TryBegin { label: u32, reg: BeamReg },
    /// `try_end Register` — end try block
    TryEnd(BeamReg),
    /// `try_case Register` — enter catch handler
    TryCase(BeamReg),
    /// Raw comment for readability in dumps
    Comment(String),
}
/// An external function reference (Module:Function/Arity).
#[derive(Debug, Clone)]
pub struct BeamExtFunc {
    pub module: String,
    pub function: String,
    pub arity: u32,
}
/// A binary segment in a bit syntax expression.
#[derive(Debug, Clone)]
pub struct BeamBitSegment {
    /// The value expression
    pub value: BeamExpr,
    /// Size specification (in bits)
    pub size: Option<BeamExpr>,
    /// Type specifier (integer, float, binary, etc.)
    pub type_spec: String,
    /// Signedness (signed / unsigned)
    pub signed: bool,
    /// Endianness (big / little / native)
    pub endian: BeamEndian,
    /// Unit size multiplier
    pub unit: Option<u8>,
}
/// ETS table type.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EtsType {
    /// `set` — one value per key
    Set,
    /// `ordered_set` — sorted, one value per key
    OrderedSet,
    /// `bag` — multiple values per key, duplicates allowed
    Bag,
    /// `duplicate_bag` — like bag but allows exact duplicates
    DuplicateBag,
}
/// Core Erlang-style expression representation.
///
/// These map closely to Core Erlang (the intermediate language used by the
/// Erlang compiler before generating BEAM bytecode).
#[derive(Debug, Clone)]
pub enum BeamExpr {
    /// Integer literal
    LitInt(i64),
    /// Float literal
    LitFloat(f64),
    /// Atom literal (e.g., `ok`, `error`, `true`, `false`)
    LitAtom(String),
    /// String (represented as binary or list of chars in BEAM)
    LitString(String),
    /// Nil (empty list `[]`)
    Nil,
    /// Variable reference
    Var(String),
    /// Tuple constructor: `{Elem1, Elem2, ...}`
    Tuple(Vec<BeamExpr>),
    /// List cons: `[Head | Tail]`
    Cons(Box<BeamExpr>, Box<BeamExpr>),
    /// Map literal: `#{Key => Value, ...}`
    Map(Vec<(BeamExpr, BeamExpr)>),
    /// Map update: `Map#{Key => Value}`
    MapUpdate(Box<BeamExpr>, Vec<(BeamExpr, BeamExpr)>),
    /// Local function call: `Module:FuncName(Args...)`
    Call {
        module: Option<String>,
        func: String,
        args: Vec<BeamExpr>,
    },
    /// Inter-module call: `Module:Function(Args...)`
    CallExt {
        module: Box<BeamExpr>,
        func: Box<BeamExpr>,
        args: Vec<BeamExpr>,
    },
    /// Apply a fun value: `Fun(Args...)`
    Apply {
        fun: Box<BeamExpr>,
        args: Vec<BeamExpr>,
    },
    /// Case expression for pattern matching
    Case {
        subject: Box<BeamExpr>,
        clauses: Vec<BeamClause>,
    },
    /// Let binding: `let Var = Expr in Body`
    Let {
        var: String,
        value: Box<BeamExpr>,
        body: Box<BeamExpr>,
    },
    /// Letrec (mutually recursive functions)
    Letrec {
        bindings: Vec<(String, Vec<String>, Box<BeamExpr>)>,
        body: Box<BeamExpr>,
    },
    /// Fun literal (lambda / closure)
    Fun {
        params: Vec<String>,
        body: Box<BeamExpr>,
        arity: usize,
    },
    /// Primitive operation (BIF — Built-In Function)
    Primop { name: String, args: Vec<BeamExpr> },
    /// Receive block with optional timeout
    Receive {
        clauses: Vec<BeamClause>,
        timeout: Option<Box<BeamExpr>>,
        timeout_action: Option<Box<BeamExpr>>,
    },
    /// Try-catch block
    Try {
        body: Box<BeamExpr>,
        vars: Vec<String>,
        handler: Box<BeamExpr>,
        evars: Vec<String>,
        catch_body: Box<BeamExpr>,
    },
    /// Sequence (do-notation style)
    Seq(Box<BeamExpr>, Box<BeamExpr>),
    /// Binary construction: `<<Segment, ...>>`
    Binary(Vec<BeamBitSegment>),
}
/// A `gen_server` behaviour specification.
///
/// gen_server is the OTP generic server process behaviour.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct GenServerSpec {
    /// Module name
    pub module_name: String,
    /// Initial state type (as a comment / documentation)
    pub state_doc: String,
    /// Synchronous call handlers: (request_pattern, reply_expr)
    pub handle_calls: Vec<(BeamPattern, BeamExpr)>,
    /// Asynchronous cast handlers: (message_pattern, new_state_expr)
    pub handle_casts: Vec<(BeamPattern, BeamExpr)>,
    /// Info message handlers: (message_pattern, new_state_expr)
    pub handle_infos: Vec<(BeamPattern, BeamExpr)>,
    /// Initial state expression
    pub init_state: BeamExpr,
    /// Termination callback body
    pub terminate_body: Option<BeamExpr>,
}
impl GenServerSpec {
    /// Create a new minimal gen_server spec.
    #[allow(dead_code)]
    pub fn new(module_name: impl Into<String>, init_state: BeamExpr) -> Self {
        GenServerSpec {
            module_name: module_name.into(),
            state_doc: "State".to_string(),
            handle_calls: Vec::new(),
            handle_casts: Vec::new(),
            handle_infos: Vec::new(),
            init_state,
            terminate_body: None,
        }
    }
    /// Generate a complete BeamModule from this spec.
    #[allow(dead_code)]
    pub fn generate_module(&self) -> BeamModule {
        let mut module = BeamModule::new(self.module_name.clone());
        module.add_attribute("behaviour", "gen_server");
        let mut start_link = BeamFunction::new("start_link", 0);
        start_link.export();
        module.add_function(start_link);
        let mut init_fn = BeamFunction::new("init", 1);
        init_fn.export();
        module.add_function(init_fn);
        let mut handle_call = BeamFunction::new("handle_call", 3);
        handle_call.export();
        module.add_function(handle_call);
        let mut handle_cast = BeamFunction::new("handle_cast", 2);
        handle_cast.export();
        module.add_function(handle_cast);
        let mut handle_info = BeamFunction::new("handle_info", 2);
        handle_info.export();
        module.add_function(handle_info);
        let mut terminate = BeamFunction::new("terminate", 2);
        terminate.export();
        module.add_function(terminate);
        module
    }
    /// Emit Core Erlang for the init/1 callback.
    #[allow(dead_code)]
    pub fn emit_init(&self) -> String {
        format!("init([]) ->\n    {{ok, {}}}.\n", "State")
    }
}
/// Links multiple BEAM modules into a combined module by merging functions.
///
/// Used for compiling multiple source files into a single Erlang module
/// (e.g., when inlining helper libraries).
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct BeamLinker {
    /// Modules being linked
    pub(super) modules: Vec<BeamModule>,
    /// Rename map: (original_module, original_name) → new_name
    pub(super) rename_map: HashMap<(String, String), String>,
    /// Target module name
    pub(super) target_name: String,
}
impl BeamLinker {
    /// Create a new linker targeting the given module name.
    #[allow(dead_code)]
    pub fn new(target: impl Into<String>) -> Self {
        BeamLinker {
            modules: Vec::new(),
            rename_map: HashMap::new(),
            target_name: target.into(),
        }
    }
    /// Add a module to be linked.
    #[allow(dead_code)]
    pub fn add_module(&mut self, module: BeamModule) {
        self.modules.push(module);
    }
    /// Rename a function during linking to avoid name collisions.
    #[allow(dead_code)]
    pub fn rename(
        &mut self,
        src_module: impl Into<String>,
        src_name: impl Into<String>,
        new_name: impl Into<String>,
    ) {
        self.rename_map
            .insert((src_module.into(), src_name.into()), new_name.into());
    }
    /// Perform the link and produce a merged BeamModule.
    #[allow(dead_code)]
    pub fn link(self) -> BeamModule {
        let mut result = BeamModule::new(self.target_name);
        for module in self.modules {
            for mut func in module.functions {
                let key = (module.name.clone(), func.name.clone());
                if let Some(new_name) = self.rename_map.get(&key) {
                    func.name = new_name.clone();
                }
                result.add_function(func);
            }
            for attr in module.attributes {
                result.attributes.push(attr);
            }
        }
        result
    }
}
/// A constant pool for BEAM modules (literal table).
///
/// BEAM stores large constants (tuples, lists, binaries) in a literal pool
/// to avoid re-creating them on every call.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct BeamConstPool {
    /// Literals stored in the pool
    pub(super) literals: Vec<BeamExpr>,
    /// Map from a canonical string key to index
    pub(super) index_map: HashMap<String, u32>,
}
impl BeamConstPool {
    /// Create an empty constant pool.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    /// Intern a literal into the pool, returning its index.
    #[allow(dead_code)]
    pub fn intern(&mut self, expr: BeamExpr, key: impl Into<String>) -> u32 {
        let k = key.into();
        if let Some(&idx) = self.index_map.get(&k) {
            return idx;
        }
        let idx = self.literals.len() as u32;
        self.literals.push(expr);
        self.index_map.insert(k, idx);
        idx
    }
    /// Get a literal by index.
    #[allow(dead_code)]
    pub fn get(&self, idx: u32) -> Option<&BeamExpr> {
        self.literals.get(idx as usize)
    }
    /// Number of literals in the pool.
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.literals.len()
    }
    /// Whether the pool is empty.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.literals.is_empty()
    }
}
/// Simple register allocator for BEAM X-registers.
///
/// BEAM X registers are general-purpose argument/result registers.
/// This allocator assigns virtual variables to X registers.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct XRegAllocator {
    /// Next free X register index
    pub(super) next_x: u32,
    /// Map from variable name to assigned X register
    pub(super) assignment: HashMap<String, u32>,
    /// Maximum X register used
    pub(super) max_x: u32,
}
impl XRegAllocator {
    /// Create a new allocator.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    /// Allocate a fresh X register for a variable.
    #[allow(dead_code)]
    pub fn alloc(&mut self, var: impl Into<String>) -> u32 {
        let x = self.next_x;
        let name = var.into();
        self.assignment.insert(name, x);
        if x > self.max_x {
            self.max_x = x;
        }
        self.next_x += 1;
        x
    }
    /// Get the X register for an already-allocated variable.
    #[allow(dead_code)]
    pub fn get(&self, var: &str) -> Option<u32> {
        self.assignment.get(var).copied()
    }
    /// Free all allocations and reset to start.
    #[allow(dead_code)]
    pub fn reset(&mut self) {
        self.next_x = 0;
        self.assignment.clear();
        self.max_x = 0;
    }
    /// Number of X registers used.
    #[allow(dead_code)]
    pub fn registers_used(&self) -> u32 {
        if self.assignment.is_empty() {
            0
        } else {
            self.max_x + 1
        }
    }
}
/// BEAM VM code generation backend.
///
/// Converts LCNF IR into Core Erlang source representation suitable for
/// further compilation by the Erlang compiler (`erlc`).
pub struct BeamBackend {
    /// The module being generated
    pub module: BeamModule,
    /// Emit context
    pub(super) ctx: BeamEmitCtx,
    /// Mapping from LCNF variable IDs to BEAM variable names
    pub(super) var_map: HashMap<u64, String>,
    /// Label counter for instruction lowering
    pub(super) next_label: u32,
}
impl BeamBackend {
    /// Create a new BEAM backend for the given module name.
    pub fn new(module_name: impl Into<String>) -> Self {
        BeamBackend {
            module: BeamModule::new(module_name),
            ctx: BeamEmitCtx::new(),
            var_map: HashMap::new(),
            next_label: 1,
        }
    }
    /// Allocate a fresh label.
    pub(super) fn fresh_label(&mut self) -> u32 {
        let l = self.next_label;
        self.next_label += 1;
        l
    }
    /// Map an LCNF variable to a BEAM variable name.
    pub(super) fn beam_var(&mut self, id: LcnfVarId) -> String {
        self.var_map
            .entry(id.0)
            .or_insert_with(|| format!("_X{}", id.0))
            .clone()
    }
    /// Convert an LCNF literal to a BeamExpr.
    pub fn emit_literal(&self, lit: &LcnfLit) -> BeamExpr {
        match lit {
            LcnfLit::Nat(n) => BeamExpr::LitInt(*n as i64),
            LcnfLit::Str(s) => BeamExpr::LitString(s.clone()),
        }
    }
    /// Convert an LCNF argument to a BeamExpr.
    pub fn emit_arg(&mut self, arg: &LcnfArg) -> BeamExpr {
        match arg {
            LcnfArg::Var(id) => BeamExpr::Var(self.beam_var(*id)),
            LcnfArg::Lit(lit) => self.emit_literal(lit),
            LcnfArg::Erased => BeamExpr::LitAtom("erased".to_string()),
            LcnfArg::Type(_) => BeamExpr::LitAtom("type".to_string()),
        }
    }
    /// Convert an LCNF let-value to a BeamExpr.
    #[allow(clippy::too_many_arguments)]
    pub fn emit_let_value(&mut self, val: &LcnfLetValue) -> BeamExpr {
        match val {
            LcnfLetValue::App(func, args) => {
                let func_expr = self.emit_arg(func);
                let arg_exprs: Vec<BeamExpr> = args.iter().map(|a| self.emit_arg(a)).collect();
                BeamExpr::Apply {
                    fun: Box::new(func_expr),
                    args: arg_exprs,
                }
            }
            LcnfLetValue::Proj(struct_name, idx, var) => {
                let var_expr = BeamExpr::Var(self.beam_var(*var));
                let beam_idx = (*idx as i64) + 2;
                BeamExpr::Primop {
                    name: format!("element({}, {})", beam_idx, struct_name),
                    args: vec![var_expr],
                }
            }
            LcnfLetValue::Ctor(name, _tag, args) => {
                let mut elems = vec![BeamExpr::LitAtom(sanitize_atom(name))];
                for a in args {
                    elems.push(self.emit_arg(a));
                }
                BeamExpr::Tuple(elems)
            }
            LcnfLetValue::Lit(lit) => self.emit_literal(lit),
            LcnfLetValue::Erased => BeamExpr::LitAtom("erased".to_string()),
            LcnfLetValue::FVar(id) => BeamExpr::Var(self.beam_var(*id)),
            LcnfLetValue::Reset(var) => BeamExpr::Primop {
                name: "reset".to_string(),
                args: vec![BeamExpr::Var(self.beam_var(*var))],
            },
            LcnfLetValue::Reuse(slot, name, _tag, args) => {
                let slot_expr = BeamExpr::Var(self.beam_var(*slot));
                let mut elems = vec![BeamExpr::LitAtom(sanitize_atom(name)), slot_expr];
                for a in args {
                    elems.push(self.emit_arg(a));
                }
                BeamExpr::Tuple(elems)
            }
        }
    }
    /// Emit a Core Erlang expression from an LCNF expression.
    #[allow(clippy::too_many_arguments)]
    pub fn emit_expr(&mut self, expr: &LcnfExpr) -> BeamExpr {
        match expr {
            LcnfExpr::Let {
                id, value, body, ..
            } => {
                let vname = self.beam_var(*id);
                let val_expr = self.emit_let_value(value);
                let body_expr = self.emit_expr(body);
                BeamExpr::Let {
                    var: vname,
                    value: Box::new(val_expr),
                    body: Box::new(body_expr),
                }
            }
            LcnfExpr::Case {
                scrutinee,
                alts,
                default,
                ..
            } => {
                let subj_expr = BeamExpr::Var(self.beam_var(*scrutinee));
                let mut clauses: Vec<BeamClause> =
                    alts.iter().map(|alt| self.emit_case_alt(alt)).collect();
                if let Some(def_body) = default {
                    let def_expr = self.emit_expr(def_body);
                    clauses.push(BeamClause {
                        pattern: BeamPattern::Wildcard,
                        guard: None,
                        body: def_expr,
                    });
                }
                BeamExpr::Case {
                    subject: Box::new(subj_expr),
                    clauses,
                }
            }
            LcnfExpr::Return(arg) => self.emit_arg(arg),
            LcnfExpr::Unreachable => BeamExpr::Primop {
                name: "error".to_string(),
                args: vec![BeamExpr::Tuple(vec![
                    BeamExpr::LitAtom("error".to_string()),
                    BeamExpr::LitAtom("unreachable".to_string()),
                ])],
            },
            LcnfExpr::TailCall(func, args) => {
                let func_expr = self.emit_arg(func);
                let arg_exprs: Vec<BeamExpr> = args.iter().map(|a| self.emit_arg(a)).collect();
                BeamExpr::Apply {
                    fun: Box::new(func_expr),
                    args: arg_exprs,
                }
            }
        }
    }
    /// Emit a BeamClause from an LCNF case alternative.
    pub(super) fn emit_case_alt(&mut self, alt: &LcnfAlt) -> BeamClause {
        let mut pats = vec![BeamPattern::LitAtom(sanitize_atom(&alt.ctor_name))];
        for param in &alt.params {
            pats.push(BeamPattern::Var(self.beam_var(param.id)));
        }
        let body = self.emit_expr(&alt.body);
        BeamClause {
            pattern: BeamPattern::Tuple(pats),
            guard: None,
            body,
        }
    }
    /// Emit a complete LCNF function declaration as a BeamFunction.
    pub fn emit_fun_decl(&mut self, decl: &LcnfFunDecl) -> BeamFunction {
        let mut func = BeamFunction::new(sanitize_atom(&decl.name), decl.params.len());
        func.params = decl.params.iter().map(|p| self.beam_var(p.id)).collect();
        let patterns = func
            .params
            .iter()
            .map(|p| BeamPattern::Var(p.clone()))
            .collect();
        let body = self.emit_expr(&decl.body);
        func.add_clause(BeamClause {
            pattern: BeamPattern::Tuple(patterns),
            guard: None,
            body,
        });
        self.lower_function(&mut func);
        func
    }
    /// Lower a BeamFunction's clauses to a linear instruction sequence.
    pub(super) fn lower_function(&mut self, func: &mut BeamFunction) {
        let entry = self.fresh_label();
        func.instrs.push(BeamInstr::Label(entry));
        func.instrs.push(BeamInstr::FuncInfo {
            module: self.module.name.clone(),
            function: func.name.clone(),
            arity: func.arity as u32,
        });
        let body_label = self.fresh_label();
        func.instrs.push(BeamInstr::Label(body_label));
        for (i, _param) in func.params.iter().enumerate() {
            func.instrs.push(BeamInstr::Move {
                src: BeamReg::X(i as u32),
                dst: BeamReg::Y(i as u32),
            });
        }
        func.instrs.push(BeamInstr::Return);
    }
    /// Emit a full Core Erlang source file for the module.
    pub fn emit_core_erlang(&mut self) -> String {
        let mut out = String::new();
        out.push_str(&format!("module '{}'\n", self.module.name));
        out.push_str("  [");
        let exports = self.module.export_list();
        for (i, exp) in exports.iter().enumerate() {
            if i > 0 {
                out.push_str(", ");
            }
            out.push_str(&format!("'{}'", exp));
        }
        out.push_str("]\n");
        out.push_str("  attributes []\n");
        for func in &self.module.functions.clone() {
            out.push_str(&self.emit_function_core_erlang(func));
        }
        out.push_str("end\n");
        out
    }
    /// Emit a single function in Core Erlang syntax.
    pub(super) fn emit_function_core_erlang(&self, func: &BeamFunction) -> String {
        let mut out = String::new();
        let params_str = func
            .params
            .iter()
            .map(|p| format!("_{}", p))
            .collect::<Vec<_>>()
            .join(", ");
        out.push_str(&format!(
            "\n'{}'/{}  =\n  fun ({}) ->\n",
            func.name, func.arity, params_str
        ));
        out.push_str("    'ok'\n");
        out
    }
    /// Emit BEAM assembly (human-readable instruction dump).
    pub fn emit_asm(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!("{{module, '{}'}}.\n\n", self.module.name));
        out.push_str(&format!(
            "{{exports, [{}]}}.\n\n",
            self.module
                .exports
                .iter()
                .map(|(n, a)| format!("{{{}, {}}}", n, a))
                .collect::<Vec<_>>()
                .join(", ")
        ));
        for func in &self.module.functions {
            out.push_str(&format!("%% Function: {}/{}\n", func.name, func.arity));
            for instr in &func.instrs {
                out.push_str(&format!("  {}\n", emit_instr(instr)));
            }
            out.push('\n');
        }
        out
    }
}
/// A pretty-printer for BEAM expressions producing human-readable Core Erlang.
#[allow(dead_code)]
pub struct BeamPrinter {
    pub(super) indent: usize,
    pub(super) buf: String,
}
impl BeamPrinter {
    /// Create a new printer.
    #[allow(dead_code)]
    pub fn new() -> Self {
        BeamPrinter {
            indent: 0,
            buf: String::new(),
        }
    }
    pub(super) fn pad(&self) -> String {
        "  ".repeat(self.indent)
    }
    pub(super) fn push(&mut self, s: &str) {
        self.buf.push_str(s);
    }
    pub(super) fn newline(&mut self) {
        self.buf.push('\n');
        self.buf.push_str(&self.pad());
    }
    /// Print a BeamExpr.
    #[allow(dead_code)]
    pub fn print_expr(&mut self, expr: &BeamExpr) {
        match expr {
            BeamExpr::LitInt(n) => self.push(&n.to_string()),
            BeamExpr::LitFloat(v) => self.push(&format!("{:.6}", v)),
            BeamExpr::LitAtom(a) => self.push(&format!("'{}'", a)),
            BeamExpr::LitString(s) => self.push(&format!("\"{}\"", s)),
            BeamExpr::Nil => self.push("[]"),
            BeamExpr::Var(name) => self.push(name),
            BeamExpr::Tuple(elems) => {
                self.push("{");
                for (i, e) in elems.iter().enumerate() {
                    if i > 0 {
                        self.push(", ");
                    }
                    self.print_expr(e);
                }
                self.push("}");
            }
            BeamExpr::Cons(head, tail) => {
                self.push("[");
                self.print_expr(head);
                self.push(" | ");
                self.print_expr(tail);
                self.push("]");
            }
            BeamExpr::Let { var, value, body } => {
                self.push(&format!("let {} =", var));
                self.indent += 1;
                self.newline();
                self.print_expr(value);
                self.indent -= 1;
                self.newline();
                self.push("in ");
                self.print_expr(body);
            }
            BeamExpr::Apply { fun, args } => {
                self.push("apply ");
                self.print_expr(fun);
                self.push("(");
                for (i, a) in args.iter().enumerate() {
                    if i > 0 {
                        self.push(", ");
                    }
                    self.print_expr(a);
                }
                self.push(")");
            }
            BeamExpr::Call { module, func, args } => {
                if let Some(m) = module {
                    self.push(&format!("call '{}':'{}' (", m, func));
                } else {
                    self.push(&format!("call '{}' (", func));
                }
                for (i, a) in args.iter().enumerate() {
                    if i > 0 {
                        self.push(", ");
                    }
                    self.print_expr(a);
                }
                self.push(")");
            }
            BeamExpr::Primop { name, args } => {
                self.push(&format!("primop '{}' (", name));
                for (i, a) in args.iter().enumerate() {
                    if i > 0 {
                        self.push(", ");
                    }
                    self.print_expr(a);
                }
                self.push(")");
            }
            _ => self.push("..."),
        }
    }
    /// Return the accumulated output.
    #[allow(dead_code)]
    pub fn finish(self) -> String {
        self.buf
    }
}
/// A BEAM module containing functions and metadata.
#[derive(Debug, Clone)]
pub struct BeamModule {
    /// Module name (atom)
    pub name: String,
    /// Module-level attributes (e.g., `{vsn, [12345]}`)
    pub attributes: Vec<(String, String)>,
    /// All functions defined in the module
    pub functions: Vec<BeamFunction>,
    /// Exported functions as (Name, Arity) pairs
    pub exports: Vec<(String, usize)>,
    /// On-load function (optional)
    pub on_load: Option<String>,
    /// Module info strings (for beam_lib)
    pub compile_info: HashMap<String, String>,
}
impl BeamModule {
    /// Create a new empty module.
    pub fn new(name: impl Into<String>) -> Self {
        BeamModule {
            name: name.into(),
            attributes: Vec::new(),
            functions: Vec::new(),
            exports: Vec::new(),
            on_load: None,
            compile_info: HashMap::new(),
        }
    }
    /// Add a function to the module.
    pub fn add_function(&mut self, func: BeamFunction) {
        if func.exported {
            self.exports.push((func.name.clone(), func.arity));
        }
        self.functions.push(func);
    }
    /// Add a module attribute.
    pub fn add_attribute(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.attributes.push((key.into(), value.into()));
    }
    /// Find a function by name and arity.
    pub fn find_function(&self, name: &str, arity: usize) -> Option<&BeamFunction> {
        self.functions
            .iter()
            .find(|f| f.name == name && f.arity == arity)
    }
    /// Returns the list of exported function keys.
    pub fn export_list(&self) -> Vec<String> {
        self.exports
            .iter()
            .map(|(n, a)| format!("{}/{}", n, a))
            .collect()
    }
}
/// A BEAM register operand.
#[derive(Debug, Clone, PartialEq)]
pub enum BeamReg {
    /// X register (general-purpose argument/return register)
    X(u32),
    /// Y register (stack slot / saved value)
    Y(u32),
    /// Floating-point register
    FR(u32),
}
