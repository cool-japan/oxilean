//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use std::collections::{HashMap, HashSet};

/// Top-level Coq declaration.
#[derive(Debug, Clone, PartialEq)]
pub enum CoqDecl {
    /// `Definition f (params) : T := body.`
    Definition {
        name: String,
        params: Vec<(String, CoqTerm)>,
        ty: Option<CoqTerm>,
        body: CoqTerm,
    },
    /// `Theorem name (params) : T. Proof. ... Qed.`
    Theorem {
        name: String,
        params: Vec<(String, CoqTerm)>,
        ty: CoqTerm,
        proof: CoqProof,
    },
    /// `Lemma name (params) : T. Proof. ... Qed.`
    Lemma {
        name: String,
        params: Vec<(String, CoqTerm)>,
        ty: CoqTerm,
        proof: CoqProof,
    },
    /// `Axiom name : T.`
    Axiom { name: String, ty: CoqTerm },
    /// `Inductive name params : Sort := constructors.`
    Inductive(CoqInductive),
    /// `Fixpoint f params : T := body.` (possibly mutual)
    Fixpoint(Vec<CoqFixPoint>),
    /// `Record name params : Sort := { fields }.`
    RecordDecl(CoqRecord),
    /// `Class name params := { methods }.`
    ClassDecl(CoqClass),
    /// `Instance name : class := { methods }.`
    Instance(CoqInstance),
    /// `Require Import module.`
    Require(String),
    /// `Open Scope scope_name.`
    OpenScope(String),
    /// `Notation "..." := term.`
    Notation(String, CoqTerm),
    /// A raw comment line
    Comment(String),
    /// A raw verbatim declaration (fallback)
    Raw(String),
}
impl CoqDecl {
    /// Emit the declaration as a Coq source string.
    pub fn emit(&self) -> String {
        match self {
            CoqDecl::Definition {
                name,
                params,
                ty,
                body,
            } => {
                let ps = if params.is_empty() {
                    String::new()
                } else {
                    format!(" {}", emit_binders(params, 0))
                };
                let ty_ann = ty
                    .as_ref()
                    .map(|t| format!(" : {}", t.emit(0)))
                    .unwrap_or_default();
                format!("Definition {}{}{} := {}.", name, ps, ty_ann, body.emit(1))
            }
            CoqDecl::Theorem {
                name,
                params,
                ty,
                proof,
            }
            | CoqDecl::Lemma {
                name,
                params,
                ty,
                proof,
            } => {
                let kw = match self {
                    CoqDecl::Theorem { .. } => "Theorem",
                    _ => "Lemma",
                };
                let ps = if params.is_empty() {
                    String::new()
                } else {
                    format!(" {}", emit_binders(params, 0))
                };
                format!("{} {}{} : {}.\n{}", kw, name, ps, ty.emit(0), proof.emit(0))
            }
            CoqDecl::Axiom { name, ty } => format!("Axiom {} : {}.", name, ty.emit(0)),
            CoqDecl::Inductive(ind) => {
                let ps = if ind.params.is_empty() {
                    String::new()
                } else {
                    format!(" {}", emit_binders(&ind.params, 0))
                };
                let mut out = format!("Inductive {}{} : {} :=\n", ind.name, ps, ind.sort);
                for ctor in &ind.constructors {
                    out.push_str(&format!("  | {} : {}\n", ctor.name, ctor.ty.emit(1)));
                }
                out.push('.');
                out
            }
            CoqDecl::Fixpoint(fps) => {
                let mut parts = Vec::new();
                for fp in fps {
                    let params = emit_binders(&fp.params, 0);
                    let ret = fp
                        .return_type
                        .as_ref()
                        .map(|t| format!(" : {}", t.emit(0)))
                        .unwrap_or_default();
                    let struct_ann = fp
                        .struct_arg
                        .as_ref()
                        .map(|s| format!(" {{struct {}}}", s))
                        .unwrap_or_default();
                    parts.push(format!(
                        "{} {}{}{} :=\n  {}",
                        fp.name,
                        params,
                        struct_ann,
                        ret,
                        fp.body.emit(1)
                    ));
                }
                if parts.len() == 1 {
                    format!("Fixpoint {}.", parts[0])
                } else {
                    let head = parts[0].clone();
                    let rest = parts[1..].join("\nwith ");
                    format!("Fixpoint {}\nwith {}.", head, rest)
                }
            }
            CoqDecl::RecordDecl(rec) => {
                let ps = if rec.params.is_empty() {
                    String::new()
                } else {
                    format!(" {}", emit_binders(&rec.params, 0))
                };
                let ctor = rec
                    .constructor
                    .as_ref()
                    .map(|c| format!(" {}", c))
                    .unwrap_or_default();
                let mut out = format!("Record {}{} : {} :={} {{\n", rec.name, ps, rec.sort, ctor);
                for field in &rec.fields {
                    out.push_str(&format!("  {} : {};\n", field.name, field.ty.emit(1)));
                }
                out.push_str("}.");
                out
            }
            CoqDecl::ClassDecl(cls) => {
                let ps = if cls.params.is_empty() {
                    String::new()
                } else {
                    format!(" {}", emit_binders(&cls.params, 0))
                };
                let mut out = format!("Class {}{} := {{\n", cls.name, ps);
                for m in &cls.methods {
                    out.push_str(&format!("  {} : {};\n", m.name, m.ty.emit(1)));
                }
                out.push_str("}.");
                out
            }
            CoqDecl::Instance(inst) => {
                let mut out = format!("Instance {} : {} := {{\n", inst.name, inst.class.emit(0));
                for (meth, body) in &inst.methods {
                    out.push_str(&format!("  {} := {};\n", meth, body.emit(1)));
                }
                out.push_str("}.");
                out
            }
            CoqDecl::Require(module) => format!("Require Import {}.", module),
            CoqDecl::OpenScope(scope) => format!("Open Scope {}.", scope),
            CoqDecl::Notation(notation, term) => {
                format!("Notation \"{}\" := {}.", notation, term.emit(0))
            }
            CoqDecl::Comment(text) => format!("(* {} *)", text),
            CoqDecl::Raw(s) => s.clone(),
        }
    }
}
/// Coq context (list of section vars + hypotheses)
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct CoqContext {
    pub vars: Vec<CoqSectionVar>,
    pub hyps: Vec<CoqHypothesis>,
}
#[allow(dead_code)]
impl CoqContext {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn add_var(&mut self, v: CoqSectionVar) {
        self.vars.push(v);
    }
    pub fn add_hyp(&mut self, h: CoqHypothesis) {
        self.hyps.push(h);
    }
    pub fn emit(&self) -> String {
        let mut out = String::new();
        for v in &self.vars {
            out.push_str(&format!("{}\n", v));
        }
        for h in &self.hyps {
            out.push_str(&format!("{}\n", h));
        }
        out
    }
}
/// A `Class` declaration (type class).
#[derive(Debug, Clone, PartialEq)]
pub struct CoqClass {
    /// Class name
    pub name: String,
    /// Class parameters (including the carrier type)
    pub params: Vec<(String, CoqTerm)>,
    /// Methods
    pub methods: Vec<CoqField>,
}
/// Coq import / require
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum CoqImport {
    Require(Vec<String>),
    RequireImport(Vec<String>),
    RequireExport(Vec<String>),
    Import(Vec<String>),
    Open(String),
}
/// Coq attribute
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum CoqAttribute {
    Global,
    Local,
    Export,
    Transparent,
    Opaque,
    Polymorphic,
    Monomorphic,
    Program,
    Equations,
    Custom(String),
}
/// Coq solve_obligations shorthand
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CoqSolveObligations {
    pub tactic: Option<String>,
}
/// A Coq proof script: `Proof. ... Qed.` (or `Admitted.`)
#[derive(Debug, Clone, PartialEq)]
pub struct CoqProof {
    /// Tactic steps
    pub tactics: Vec<CoqTactic>,
    /// Use `Admitted.` instead of `Qed.`
    pub admitted: bool,
}
impl CoqProof {
    /// Construct a simple proof from a slice of tactics.
    pub fn new(tactics: Vec<CoqTactic>) -> Self {
        Self {
            tactics,
            admitted: false,
        }
    }
    /// Construct an admitted (placeholder) proof.
    pub fn admitted() -> Self {
        Self {
            tactics: Vec::new(),
            admitted: true,
        }
    }
    /// Emit as a `Proof. ... Qed.` block.
    pub fn emit(&self, indent: usize) -> String {
        let mut out = "Proof.\n".to_string();
        for tac in &self.tactics {
            out.push_str(&format!("  {}.\n", tac.emit(indent + 1)));
        }
        if self.admitted {
            out.push_str("Admitted.");
        } else {
            out.push_str("Qed.");
        }
        out
    }
}
/// Coq Example definition
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CoqExample {
    pub name: String,
    pub statement: String,
    pub proof: CoqTacticBlock,
}
/// A complete Coq source file (`.v`).
#[derive(Debug, Clone)]
pub struct CoqModule {
    /// Top-level module name (used as a comment / `Module` block if needed)
    pub name: String,
    /// `Require Import` directives emitted before declarations
    pub requires: Vec<String>,
    /// `Open Scope` directives emitted after requires
    pub opens: Vec<String>,
    /// Top-level declarations
    pub declarations: Vec<CoqDecl>,
}
impl CoqModule {
    /// Construct an empty module.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            requires: Vec::new(),
            opens: Vec::new(),
            declarations: Vec::new(),
        }
    }
    /// Add a `Require Import` directive.
    pub fn require(&mut self, module: impl Into<String>) {
        self.requires.push(module.into());
    }
    /// Add an `Open Scope` directive.
    pub fn open_scope(&mut self, scope: impl Into<String>) {
        self.opens.push(scope.into());
    }
    /// Add a declaration.
    pub fn add(&mut self, decl: CoqDecl) {
        self.declarations.push(decl);
    }
    /// Emit the full Coq source as a `String`.
    pub fn emit(&self) -> String {
        let mut out = format!("(* Generated by OxiLean: {} *)\n\n", self.name);
        for req in &self.requires {
            out.push_str(&format!("Require Import {}.\n", req));
        }
        if !self.requires.is_empty() {
            out.push('\n');
        }
        for scope in &self.opens {
            out.push_str(&format!("Open Scope {}.\n", scope));
        }
        if !self.opens.is_empty() {
            out.push('\n');
        }
        for decl in &self.declarations {
            out.push_str(&decl.emit());
            out.push_str("\n\n");
        }
        out
    }
}
/// Coq hint database
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum CoqHint {
    Resolve(Vec<String>, Option<String>),
    Rewrite(Vec<String>, Option<String>),
    Unfold(Vec<String>, Option<String>),
    Immediate(Vec<String>, Option<String>),
    Constructors(Vec<String>, Option<String>),
    Extern(u32, Option<String>, String),
}
/// Coq definition with let-binding
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CoqLetDef {
    pub name: String,
    pub params: Vec<(String, Option<String>)>,
    pub return_type: Option<String>,
    pub body: String,
    pub is_opaque: bool,
}
/// Coq backend feature flags
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct CoqFeatureFlags {
    pub use_ssreflect: bool,
    pub use_mathcomp: bool,
    pub use_equations: bool,
    pub use_stdpp: bool,
    pub use_iris: bool,
}
/// Coq hypothesis declaration
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CoqHypothesis {
    pub name: String,
    pub hyp_type: String,
}
/// Coq extraction directive
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum CoqExtraction {
    Language(String),
    Constant(String, String),
    Inductive(String, String),
    Inline(Vec<String>),
    NoInline(Vec<String>),
    RecursiveExtraction(String),
    Extraction(String, String),
    ExtractionLibrary(String, String),
}
/// Coq obligating tactic
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CoqObligation {
    pub n: u32,
    pub tactics: Vec<CoqTacticExt>,
}
/// Coq ltac definition
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CoqLtacDef {
    pub name: String,
    pub params: Vec<String>,
    pub body: String,
    pub is_recursive: bool,
}
/// Coq instance definition
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CoqInstanceDef {
    pub name: Option<String>,
    pub class: String,
    pub args: Vec<String>,
    pub methods: Vec<(String, String)>,
}
/// Coq source buffer
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct CoqExtSourceBuffer {
    pub sections: Vec<String>,
    pub current: String,
    pub indent: usize,
}
#[allow(dead_code)]
impl CoqExtSourceBuffer {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn write(&mut self, s: &str) {
        let indent = " ".repeat(self.indent * 2);
        for line in s.lines() {
            self.current.push_str(&indent);
            self.current.push_str(line);
            self.current.push('\n');
        }
    }
    pub fn write_raw(&mut self, s: &str) {
        self.current.push_str(s);
    }
    pub fn push_indent(&mut self) {
        self.indent += 1;
    }
    pub fn pop_indent(&mut self) {
        if self.indent > 0 {
            self.indent -= 1;
        }
    }
    pub fn commit_section(&mut self) {
        let sec = std::mem::take(&mut self.current);
        if !sec.is_empty() {
            self.sections.push(sec);
        }
    }
    pub fn finish(mut self) -> String {
        self.commit_section();
        self.sections.join("\n")
    }
}
/// A single branch inside a `match` expression.
/// `| ctor arg1 arg2 => body`
#[derive(Debug, Clone, PartialEq)]
pub struct CoqBranch {
    /// Constructor name (e.g. `"O"`, `"S"`, `"nil"`, `"cons"`)
    pub constructor: String,
    /// Bound variable names for constructor arguments
    pub args: Vec<String>,
    /// Right-hand side of the branch
    pub body: CoqTerm,
}
/// Gallina term AST.
#[derive(Debug, Clone, PartialEq)]
pub enum CoqTerm {
    /// Variable or qualified name: `nat`, `List.length`, `Nat.add`
    Var(String),
    /// Application: `f a b c`
    App(Box<CoqTerm>, Vec<CoqTerm>),
    /// Lambda abstraction: `fun (x : T) (y : U) => body`
    Lambda(Vec<(String, CoqTerm)>, Box<CoqTerm>),
    /// Universal quantifier: `forall (x : T), P x`
    Forall(Vec<(String, CoqTerm)>, Box<CoqTerm>),
    /// Non-dependent function type: `T -> U`
    Prod(Box<CoqTerm>, Box<CoqTerm>),
    /// Let binding: `let x [: T] := rhs in body`
    Let(String, Option<Box<CoqTerm>>, Box<CoqTerm>, Box<CoqTerm>),
    /// Pattern match: `match scrutinee [as name] with | ... end`
    Match(Box<CoqTerm>, Option<String>, Vec<CoqBranch>),
    /// (Mutual) fixpoint: `fix f (n : nat) ... := body`
    Fix(Vec<CoqFixPoint>),
    /// Universe sort: `Prop`, `Set`, `Type`
    Sort(CoqSort),
    /// Hole / unification variable: `_`
    Hole,
    /// Integer literal: `0`, `42`, `-1`
    Num(i64),
    /// String literal: `"hello"`
    Str(String),
    /// Coercion / type ascription: `(term : T)`
    Cast(Box<CoqTerm>, Box<CoqTerm>),
    /// Implicit argument: `@f` (explicit application)
    Explicit(Box<CoqTerm>),
    /// Tuple/pair: `(a, b)` (syntactic sugar for `pair a b`)
    Tuple(Vec<CoqTerm>),
    /// List literal: `[a; b; c]`
    List(Vec<CoqTerm>),
    /// If-then-else: `if b then t else f`
    IfThenElse(Box<CoqTerm>, Box<CoqTerm>, Box<CoqTerm>),
}
impl CoqTerm {
    /// Emit as a Gallina string with the given indentation.
    pub fn emit(&self, indent: usize) -> String {
        let pad = "  ".repeat(indent);
        match self {
            CoqTerm::Var(s) => s.clone(),
            CoqTerm::Num(n) => n.to_string(),
            CoqTerm::Str(s) => format!("\"{}\"", escape_coq_string(s)),
            CoqTerm::Hole => "_".to_string(),
            CoqTerm::Sort(s) => s.to_string(),
            CoqTerm::App(func, args) => {
                let f = func.emit_atom(indent);
                let a: Vec<String> = args.iter().map(|a| a.emit_atom(indent)).collect();
                format!("{} {}", f, a.join(" "))
            }
            CoqTerm::Lambda(binders, body) => {
                let bs = emit_binders(binders, indent);
                format!("fun {} =>\n{}  {}", bs, pad, body.emit(indent + 1))
            }
            CoqTerm::Forall(binders, body) => {
                let bs = emit_binders(binders, indent);
                format!("forall {}, {}", bs, body.emit(indent))
            }
            CoqTerm::Prod(dom, cod) => {
                format!("{} -> {}", dom.emit_atom(indent), cod.emit(indent))
            }
            CoqTerm::Let(x, ty, rhs, body) => {
                let ty_ann = ty
                    .as_ref()
                    .map(|t| format!(" : {}", t.emit(indent)))
                    .unwrap_or_default();
                format!(
                    "let {}{} := {} in\n{}{}",
                    x,
                    ty_ann,
                    rhs.emit(indent + 1),
                    pad,
                    body.emit(indent)
                )
            }
            CoqTerm::Match(scrutinee, alias, branches) => {
                let alias_str = alias
                    .as_ref()
                    .map(|a| format!(" as {}", a))
                    .unwrap_or_default();
                let mut out = format!("match {}{} with\n", scrutinee.emit(indent), alias_str);
                for b in branches {
                    let args = if b.args.is_empty() {
                        String::new()
                    } else {
                        format!(" {}", b.args.join(" "))
                    };
                    out.push_str(&format!(
                        "{}| {}{} => {}\n",
                        pad,
                        b.constructor,
                        args,
                        b.body.emit(indent + 1)
                    ));
                }
                out.push_str(&format!("{}end", pad));
                out
            }
            CoqTerm::Fix(fps) => {
                let mut parts = Vec::new();
                for fp in fps {
                    let params = emit_binders(&fp.params, indent);
                    let ret = fp
                        .return_type
                        .as_ref()
                        .map(|t| format!(" : {}", t.emit(indent)))
                        .unwrap_or_default();
                    let struct_ann = fp
                        .struct_arg
                        .as_ref()
                        .map(|s| format!(" {{struct {}}}", s))
                        .unwrap_or_default();
                    parts.push(format!(
                        "{} {}{}{} :=\n{}  {}",
                        fp.name,
                        params,
                        struct_ann,
                        ret,
                        pad,
                        fp.body.emit(indent + 1)
                    ));
                }
                format!("fix {}", parts.join("\nwith "))
            }
            CoqTerm::Cast(term, ty) => {
                format!("({} : {})", term.emit(indent), ty.emit(indent))
            }
            CoqTerm::Explicit(term) => format!("@{}", term.emit(indent)),
            CoqTerm::Tuple(elems) => {
                let es: Vec<String> = elems.iter().map(|e| e.emit(indent)).collect();
                format!("({})", es.join(", "))
            }
            CoqTerm::List(elems) => {
                let es: Vec<String> = elems.iter().map(|e| e.emit(indent)).collect();
                format!("[{}]", es.join("; "))
            }
            CoqTerm::IfThenElse(cond, then_, else_) => {
                format!(
                    "if {} then {} else {}",
                    cond.emit(indent),
                    then_.emit(indent),
                    else_.emit(indent)
                )
            }
        }
    }
    /// Emit as an atomic term (wrap compound terms in parens).
    pub(super) fn emit_atom(&self, indent: usize) -> String {
        match self {
            CoqTerm::Var(_)
            | CoqTerm::Num(_)
            | CoqTerm::Str(_)
            | CoqTerm::Hole
            | CoqTerm::Sort(_)
            | CoqTerm::Tuple(_)
            | CoqTerm::List(_) => self.emit(indent),
            _ => format!("({})", self.emit(indent)),
        }
    }
}
/// Coq section variable declaration
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CoqSectionVar {
    pub names: Vec<String>,
    pub var_type: String,
}
/// An `Instance` declaration (type class instance).
#[derive(Debug, Clone, PartialEq)]
pub struct CoqInstance {
    /// Instance name
    pub name: String,
    /// Class applied to concrete types
    pub class: CoqTerm,
    /// Method implementations: `method := body`
    pub methods: Vec<(String, CoqTerm)>,
}
/// Coq tactic
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum CoqTacticExt {
    Intro(Vec<String>),
    Apply(String),
    Exact(String),
    Rewrite(bool, String),
    Simpl,
    Ring,
    Omega,
    Lia,
    Lra,
    Auto,
    EAuto,
    Tauto,
    Constructor,
    Split,
    Left,
    Right,
    Exists(String),
    Induction(String),
    Destruct(String),
    Inversion(String),
    Reflexivity,
    Symmetry,
    Transitivity(String),
    Unfold(Vec<String>),
    Fold(Vec<String>),
    Assumption,
    Contradiction,
    Exfalso,
    Clear(Vec<String>),
    Rename(String, String),
    Trivial,
    Discriminate,
    Injection(String),
    FApply(String),
    Subst(Option<String>),
    Custom(String),
}
/// Coq version target
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CoqVersion {
    V8_14,
    V8_15,
    V8_16,
    V8_17,
    V8_18,
    V8_19,
    V8_20,
    Rocq0_1,
    Latest,
}
/// Coq name scope
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct CoqNameScope {
    pub names: Vec<std::collections::HashMap<String, String>>,
}
#[allow(dead_code)]
impl CoqNameScope {
    pub fn new() -> Self {
        let mut s = Self::default();
        s.names.push(std::collections::HashMap::new());
        s
    }
    pub fn push(&mut self) {
        self.names.push(std::collections::HashMap::new());
    }
    pub fn pop(&mut self) {
        if self.names.len() > 1 {
            self.names.pop();
        }
    }
    pub fn bind(&mut self, name: &str, coq_name: &str) {
        if let Some(scope) = self.names.last_mut() {
            scope.insert(name.to_string(), coq_name.to_string());
        }
    }
    pub fn lookup(&self, name: &str) -> Option<&String> {
        for scope in self.names.iter().rev() {
            if let Some(v) = scope.get(name) {
                return Some(v);
            }
        }
        None
    }
}
/// A single fixpoint function (inside `Fixpoint` or `fix`).
#[derive(Debug, Clone, PartialEq)]
pub struct CoqFixPoint {
    /// Function name
    pub name: String,
    /// Parameter list: `(x : T)`
    pub params: Vec<(String, CoqTerm)>,
    /// Optional return type annotation
    pub return_type: Option<CoqTerm>,
    /// Structurally decreasing argument name (for `{struct arg}`)
    pub struct_arg: Option<String>,
    /// Function body
    pub body: CoqTerm,
}
/// Coq emit stats
#[allow(dead_code)]
#[derive(Debug, Default, Clone)]
pub struct CoqExtEmitStats {
    pub theorems_emitted: usize,
    pub definitions_emitted: usize,
    pub lemmas_emitted: usize,
    pub axioms_emitted: usize,
    pub tactics_emitted: usize,
}
/// Coq eval command variants
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum CoqEvalCmd {
    Compute(String),
    Eval(String, String),
    Check(String),
    Print(String),
    About(String),
    SearchPattern(String),
}
/// Coq universe level
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum CoqUniverse {
    Prop,
    Set,
    Type(Option<u32>),
    SProp,
}
/// Top-level `Record` declaration.
#[derive(Debug, Clone, PartialEq)]
pub struct CoqRecord {
    /// Record name
    pub name: String,
    /// Type parameters
    pub params: Vec<(String, CoqTerm)>,
    /// Sort of the record
    pub sort: CoqSort,
    /// Optional constructor name (defaults to `"Build_<name>"`)
    pub constructor: Option<String>,
    /// Fields
    pub fields: Vec<CoqField>,
}
/// Coq profiler
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct CoqExtProfiler {
    pub timings: Vec<(String, u64)>,
}
#[allow(dead_code)]
impl CoqExtProfiler {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn record(&mut self, pass: &str, us: u64) {
        self.timings.push((pass.to_string(), us));
    }
    pub fn total_us(&self) -> u64 {
        self.timings.iter().map(|(_, t)| t).sum()
    }
}
/// Coq class definition
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CoqClassDef {
    pub name: String,
    pub params: Vec<(String, String)>,
    pub methods: Vec<(String, String)>,
}
/// Coq module system
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CoqModuleDef {
    pub name: String,
    pub items: Vec<String>,
    pub is_section: bool,
}
/// Coq name mangler
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct CoqNameMangler {
    pub used: std::collections::HashSet<String>,
    pub map: std::collections::HashMap<String, String>,
}
#[allow(dead_code)]
impl CoqNameMangler {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn mangle(&mut self, name: &str) -> String {
        if let Some(m) = self.map.get(name) {
            return m.clone();
        }
        let mangled: String = name
            .chars()
            .map(|c| {
                if c.is_alphanumeric() || c == '_' || c == '\'' {
                    c
                } else {
                    '_'
                }
            })
            .collect();
        let coq_reserved = [
            "Definition",
            "Theorem",
            "Lemma",
            "Proof",
            "Qed",
            "Admitted",
            "Require",
            "Import",
            "Module",
            "Section",
            "End",
            "Record",
            "Inductive",
            "CoInductive",
            "Class",
            "Instance",
            "forall",
            "exists",
            "fun",
            "match",
            "with",
            "end",
            "in",
            "let",
            "fix",
            "cofix",
            "if",
            "then",
            "else",
            "return",
            "Type",
            "Prop",
            "Set",
        ];
        let mut candidate = if coq_reserved.contains(&mangled.as_str()) {
            format!("ox_{}", mangled)
        } else {
            mangled.clone()
        };
        let base = candidate.clone();
        let mut counter = 0;
        while self.used.contains(&candidate) {
            counter += 1;
            candidate = format!("{}_{}", base, counter);
        }
        self.used.insert(candidate.clone());
        self.map.insert(name.to_string(), candidate.clone());
        candidate
    }
}
/// Coq id generator
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct CoqExtIdGen {
    pub(super) counter: u64,
    pub(super) prefix: String,
}
#[allow(dead_code)]
impl CoqExtIdGen {
    pub fn new(prefix: &str) -> Self {
        Self {
            counter: 0,
            prefix: prefix.to_string(),
        }
    }
    pub fn next(&mut self) -> String {
        let id = self.counter;
        self.counter += 1;
        format!("{}_{}", self.prefix, id)
    }
}
/// Coq proof term emitter (for extract-based proofs)
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct CoqProofTermEmitter {
    pub proof_terms: Vec<(String, String)>,
}
#[allow(dead_code)]
impl CoqProofTermEmitter {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn add(&mut self, name: &str, term: &str) {
        self.proof_terms.push((name.to_string(), term.to_string()));
    }
    pub fn emit_all(&self) -> String {
        let mut out = String::new();
        for (n, t) in &self.proof_terms {
            out.push_str(&format!("Definition {}_proof := {}.\n", n, t));
        }
        out
    }
}
/// Coq notation definition
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CoqNotation {
    pub pattern: String,
    pub body: String,
    pub level: Option<u32>,
    pub assoc: Option<String>,
    pub scope: Option<String>,
}
/// A single constructor in an `Inductive` type.
#[derive(Debug, Clone, PartialEq)]
pub struct CoqConstructor {
    /// Constructor name
    pub name: String,
    /// Constructor type (usually a chain of `forall` / `->`)
    pub ty: CoqTerm,
}
/// Coq code statistics
#[allow(dead_code)]
#[derive(Debug, Default, Clone)]
pub struct CoqCodeStats {
    pub theorems: usize,
    pub definitions: usize,
    pub lemmas: usize,
    pub axioms: usize,
    pub instances: usize,
    pub classes: usize,
    pub modules: usize,
    pub total_lines: usize,
}
/// Coq tactic block
#[allow(dead_code)]
#[derive(Debug, Default, Clone)]
pub struct CoqTacticBlock {
    pub tactics: Vec<CoqTacticExt>,
}
#[allow(dead_code)]
impl CoqTacticBlock {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn add(&mut self, tac: CoqTacticExt) {
        self.tactics.push(tac);
    }
    pub fn emit(&self) -> String {
        let mut out = String::from("Proof.\n");
        for t in &self.tactics {
            out.push_str(&format!("  {}.\n", t));
        }
        out.push_str("Qed.\n");
        out
    }
    pub fn emit_admitted(&self) -> String {
        let mut out = String::from("Proof.\n");
        for t in &self.tactics {
            out.push_str(&format!("  {}.\n", t));
        }
        out.push_str("Admitted.\n");
        out
    }
}
/// Coq inductive definition (extended)
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CoqInductiveDef {
    pub name: String,
    pub params: Vec<(String, String)>,
    pub indices: Vec<(String, String)>,
    pub universe: CoqUniverse,
    pub constructors: Vec<(String, Vec<(String, String)>, String)>,
    pub is_coinductive: bool,
}
/// A single field in a `Record`.
#[derive(Debug, Clone, PartialEq)]
pub struct CoqField {
    /// Field name
    pub name: String,
    /// Field type
    pub ty: CoqTerm,
}
/// Coq diagnostics
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CoqDiagLevel {
    Info,
    Warning,
    Error,
}
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct CoqDiagSink {
    pub diags: Vec<CoqDiag>,
}
#[allow(dead_code)]
impl CoqDiagSink {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn push(&mut self, level: CoqDiagLevel, msg: &str, item: Option<&str>) {
        self.diags.push(CoqDiag {
            level,
            message: msg.to_string(),
            item: item.map(|s| s.to_string()),
        });
    }
    pub fn has_errors(&self) -> bool {
        self.diags.iter().any(|d| d.level == CoqDiagLevel::Error)
    }
}
/// Coq module signature
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct CoqModuleSig {
    pub name: String,
    pub exports: Vec<String>,
}
#[allow(dead_code)]
impl CoqModuleSig {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            exports: Vec::new(),
        }
    }
    pub fn export(&mut self, item: &str) {
        self.exports.push(item.to_string());
    }
    pub fn emit(&self) -> String {
        format!(
            "Module Type {}.\n{}\nEnd {}.",
            self.name,
            self.exports
                .iter()
                .map(|e| format!("  Parameter {}.", e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.name,
        )
    }
}
/// Top-level `Inductive` (or `Variant`) declaration.
#[derive(Debug, Clone, PartialEq)]
pub struct CoqInductive {
    /// Type name
    pub name: String,
    /// Universe parameters and type-level parameters: `(A : Set)`
    pub params: Vec<(String, CoqTerm)>,
    /// Sort of the inductive type itself
    pub sort: CoqSort,
    /// Constructors
    pub constructors: Vec<CoqConstructor>,
}
/// Coq extended backend config
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CoqExtConfig {
    pub emit_comments: bool,
    pub use_program: bool,
    pub use_equations: bool,
    pub universe_polymorphism: bool,
    pub default_db: String,
    pub emit_extracted: bool,
}
/// Ltac tactic AST.
#[derive(Debug, Clone, PartialEq)]
pub enum CoqTactic {
    /// `intro x y z`
    Intro(Vec<String>),
    /// `apply t`
    Apply(CoqTerm),
    /// `exact t`
    Exact(CoqTerm),
    /// `simpl` / `simpl in *`
    Simpl,
    /// `reflexivity`
    Reflexivity,
    /// `rewrite [<-] h`
    Rewrite(bool, CoqTerm),
    /// `induction x`
    Induction(String),
    /// `destruct x`
    Destruct(String),
    /// `auto`
    Auto,
    /// `omega`
    Omega,
    /// `lia` (Linear Integer Arithmetic)
    Lia,
    /// `ring`
    Ring,
    /// `unfold f g h`
    Unfold(Vec<String>),
    /// `compute`
    Compute,
    /// `assumption`
    Assumption,
    /// `tauto`
    Tauto,
    /// `decide`
    Decide,
    /// `trivial`
    Trivial,
    /// `split`
    Split,
    /// `left`
    Left,
    /// `right`
    Right,
    /// `exists witness`
    Exists(CoqTerm),
    /// `generalize t`
    Generalize(CoqTerm),
    /// `specialize (f a b)`
    Specialize(CoqTerm, Vec<CoqTerm>),
    /// `case t`
    Case(CoqTerm),
    /// `admit`
    Admit,
    /// Raw / custom tactic string
    Custom(String),
    /// Sequential composition: `t1. t2. t3.`
    Sequence(Vec<CoqTactic>),
    /// Then combinator: `t1; t2` (apply t2 to all subgoals of t1)
    Then(Box<CoqTactic>, Box<CoqTactic>),
}
impl CoqTactic {
    /// Emit as an Ltac string.
    pub fn emit(&self, indent: usize) -> String {
        let pad = "  ".repeat(indent);
        match self {
            CoqTactic::Intro(names) => {
                if names.is_empty() {
                    "intro".to_string()
                } else {
                    format!("intro {}", names.join(" "))
                }
            }
            CoqTactic::Apply(t) => format!("apply {}", t.emit(indent)),
            CoqTactic::Exact(t) => format!("exact {}", t.emit(indent)),
            CoqTactic::Simpl => "simpl".to_string(),
            CoqTactic::Reflexivity => "reflexivity".to_string(),
            CoqTactic::Rewrite(backward, t) => {
                if *backward {
                    format!("rewrite <- {}", t.emit(indent))
                } else {
                    format!("rewrite -> {}", t.emit(indent))
                }
            }
            CoqTactic::Induction(x) => format!("induction {}", x),
            CoqTactic::Destruct(x) => format!("destruct {}", x),
            CoqTactic::Auto => "auto".to_string(),
            CoqTactic::Omega => "omega".to_string(),
            CoqTactic::Lia => "lia".to_string(),
            CoqTactic::Ring => "ring".to_string(),
            CoqTactic::Unfold(names) => format!("unfold {}", names.join(", ")),
            CoqTactic::Compute => "compute".to_string(),
            CoqTactic::Assumption => "assumption".to_string(),
            CoqTactic::Tauto => "tauto".to_string(),
            CoqTactic::Decide => "decide".to_string(),
            CoqTactic::Trivial => "trivial".to_string(),
            CoqTactic::Split => "split".to_string(),
            CoqTactic::Left => "left".to_string(),
            CoqTactic::Right => "right".to_string(),
            CoqTactic::Exists(w) => format!("exists {}", w.emit(indent)),
            CoqTactic::Generalize(t) => format!("generalize {}", t.emit(indent)),
            CoqTactic::Specialize(f, args) => {
                let all: Vec<CoqTerm> = std::iter::once(f.clone())
                    .chain(args.iter().cloned())
                    .collect();
                let parts: Vec<String> = all.iter().map(|t| t.emit(indent)).collect();
                format!("specialize ({} {})", parts[0], parts[1..].join(" "))
            }
            CoqTactic::Case(t) => format!("case {}", t.emit(indent)),
            CoqTactic::Admit => "admit".to_string(),
            CoqTactic::Custom(s) => s.clone(),
            CoqTactic::Sequence(tactics) => tactics
                .iter()
                .map(|t| format!("{}{}.", pad, t.emit(indent)))
                .collect::<Vec<_>>()
                .join("\n"),
            CoqTactic::Then(t1, t2) => {
                format!("{}; {}", t1.emit(indent), t2.emit(indent))
            }
        }
    }
}
/// Coq program builder
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct CoqProgramBuilder {
    pub imports: Vec<CoqImport>,
    pub type_defs: Vec<String>,
    pub items: Vec<String>,
}
#[allow(dead_code)]
impl CoqProgramBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn add_import(&mut self, import: CoqImport) {
        self.imports.push(import);
    }
    pub fn add_type_def(&mut self, td: &str) {
        self.type_defs.push(td.to_string());
    }
    pub fn add_item(&mut self, item: &str) {
        self.items.push(item.to_string());
    }
    pub fn build(&self) -> String {
        let mut out = String::new();
        for imp in &self.imports {
            out.push_str(&format!("{}\n", imp));
        }
        if !self.imports.is_empty() {
            out.push('\n');
        }
        for td in &self.type_defs {
            out.push_str(td);
            out.push('\n');
        }
        if !self.type_defs.is_empty() {
            out.push('\n');
        }
        for item in &self.items {
            out.push_str(item);
            out.push('\n');
        }
        out
    }
}
/// Coq record definition
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CoqRecordDef {
    pub name: String,
    pub params: Vec<(String, String)>,
    pub universe: CoqUniverse,
    pub constructor: Option<String>,
    pub fields: Vec<(String, String)>,
}
/// Universe sort in Coq's Calculus of Inductive Constructions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CoqSort {
    /// `Prop` — impredicative sort for logical propositions
    Prop,
    /// `Set` — predicative sort for computational data types
    Set,
    /// `Type` — universe-polymorphic sort; `Type(None)` = `Type`, `Type(Some(i))` = `Type@{i}`
    Type(Option<u32>),
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CoqDiag {
    pub level: CoqDiagLevel,
    pub message: String,
    pub item: Option<String>,
}
/// Coq tactic notation
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CoqTacticNotation {
    pub level: u32,
    pub pattern: Vec<String>,
    pub body: String,
}
/// Coq compute command
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CoqCompute {
    pub expr: String,
}
/// Coq pass builder (for integrating into pipeline)
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct CoqPassBuilder {
    pub config: CoqExtConfig,
    pub source: CoqExtSourceBuffer,
    pub stats: CoqExtEmitStats,
}
#[allow(dead_code)]
impl CoqPassBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_config(mut self, cfg: CoqExtConfig) -> Self {
        self.config = cfg;
        self
    }
    pub fn finish(self) -> String {
        self.source.finish()
    }
    pub fn report(&self) -> String {
        format!("{}", self.stats)
    }
}
/// Coq universe polymorphism context
#[allow(dead_code)]
#[derive(Debug, Default, Clone)]
pub struct CoqUniverseCtx {
    pub levels: Vec<String>,
    pub constraints: Vec<(String, String, String)>,
}
#[allow(dead_code)]
impl CoqUniverseCtx {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn add_level(&mut self, name: &str) {
        self.levels.push(name.to_string());
    }
    pub fn add_lt(&mut self, u: &str, v: &str) {
        self.constraints
            .push((u.to_string(), "<".to_string(), v.to_string()));
    }
    pub fn add_le(&mut self, u: &str, v: &str) {
        self.constraints
            .push((u.to_string(), "<=".to_string(), v.to_string()));
    }
    pub fn emit_poly_annotation(&self) -> String {
        if self.levels.is_empty() {
            return String::new();
        }
        format!("@{{{}}} ", self.levels.join(" "))
    }
}
/// Coq fixpoint definition
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CoqFixpointDef {
    pub name: String,
    pub params: Vec<(String, String)>,
    pub return_type: String,
    pub struct_arg: Option<String>,
    pub body: String,
}
/// Coq where-clause
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CoqWhere {
    pub vars: Vec<(String, String)>,
}
