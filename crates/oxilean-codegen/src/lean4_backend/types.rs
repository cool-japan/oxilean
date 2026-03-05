//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::{HashMap, HashSet, VecDeque};

/// Lean 4 code generation backend.
pub struct Lean4Backend {
    pub(super) file: Lean4File,
}
impl Lean4Backend {
    /// Create a new Lean 4 backend.
    pub fn new() -> Self {
        Lean4Backend {
            file: Lean4File::new(),
        }
    }
    /// Create a backend with standard Lean 4 imports.
    pub fn with_std_imports() -> Self {
        let file = Lean4File::new().with_import("Init").with_import("Std");
        Lean4Backend { file }
    }
    /// Compile a simple kernel declaration (name + type + body) to a Lean 4 def.
    pub fn compile_kernel_decl(
        &mut self,
        name: &str,
        args: Vec<(std::string::String, Lean4Type)>,
        ret_ty: Lean4Type,
        body: Lean4Expr,
    ) {
        let def = Lean4Def::simple(name, args, ret_ty, body);
        self.file.add_decl(Lean4Decl::Def(def));
    }
    /// Add a theorem with a tactic proof.
    pub fn add_theorem(
        &mut self,
        name: &str,
        args: Vec<(std::string::String, Lean4Type)>,
        ty: Lean4Type,
        tactics: Vec<std::string::String>,
    ) {
        let thm = Lean4Theorem::tactic(name, args, ty, tactics);
        self.file.add_decl(Lean4Decl::Theorem(thm));
    }
    /// Add an inductive type.
    pub fn add_inductive(&mut self, ind: Lean4Inductive) {
        self.file.add_decl(Lean4Decl::Inductive(ind));
    }
    /// Emit the complete Lean 4 file.
    pub fn emit_file(&self) -> std::string::String {
        self.file.emit()
    }
    /// Get a mutable reference to the file.
    pub fn file_mut(&mut self) -> &mut Lean4File {
        &mut self.file
    }
}
#[allow(dead_code)]
pub struct L4PassRegistry {
    pub(super) configs: Vec<L4PassConfig>,
    pub(super) stats: std::collections::HashMap<String, L4PassStats>,
}
impl L4PassRegistry {
    #[allow(dead_code)]
    pub fn new() -> Self {
        L4PassRegistry {
            configs: Vec::new(),
            stats: std::collections::HashMap::new(),
        }
    }
    #[allow(dead_code)]
    pub fn register(&mut self, config: L4PassConfig) {
        self.stats
            .insert(config.pass_name.clone(), L4PassStats::new());
        self.configs.push(config);
    }
    #[allow(dead_code)]
    pub fn enabled_passes(&self) -> Vec<&L4PassConfig> {
        self.configs.iter().filter(|c| c.enabled).collect()
    }
    #[allow(dead_code)]
    pub fn get_stats(&self, name: &str) -> Option<&L4PassStats> {
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
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct L4DepGraph {
    pub(super) nodes: Vec<u32>,
    pub(super) edges: Vec<(u32, u32)>,
}
impl L4DepGraph {
    #[allow(dead_code)]
    pub fn new() -> Self {
        L4DepGraph {
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
/// Lean 4 type representation.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Lean4Type {
    /// `Nat` — natural numbers
    Nat,
    /// `Int` — integers
    Int,
    /// `Float` — 64-bit floating point
    Float,
    /// `Bool` — boolean
    Bool,
    /// `String` — string type
    String,
    /// `Unit` — unit type
    Unit,
    /// `Prop` — sort of propositions
    Prop,
    /// `Type u` — universe
    Type(u32),
    /// `List α`
    List(Box<Lean4Type>),
    /// `Option α`
    Option(Box<Lean4Type>),
    /// `α × β` (product)
    Prod(Box<Lean4Type>, Box<Lean4Type>),
    /// `α ⊕ β` (sum / Either)
    Sum(Box<Lean4Type>, Box<Lean4Type>),
    /// `α → β` (function type)
    Fun(Box<Lean4Type>, Box<Lean4Type>),
    /// Named type (user-defined or imported)
    Custom(std::string::String),
    /// `∀ (x : α), β x` (dependent function / Pi type)
    ForAll(std::string::String, Box<Lean4Type>, Box<Lean4Type>),
    /// Type application: `f α`
    App(Box<Lean4Type>, Box<Lean4Type>),
    /// `IO α`
    IO(Box<Lean4Type>),
    /// `Array α`
    Array(Box<Lean4Type>),
    /// `Fin n`
    Fin(Box<Lean4Type>),
    /// `Char`
    Char,
}
/// Top-level Lean 4 declaration.
#[derive(Debug, Clone)]
pub enum Lean4Decl {
    /// `def` definition
    Def(Lean4Def),
    /// `theorem` declaration
    Theorem(Lean4Theorem),
    /// `axiom` declaration
    Axiom(Lean4Axiom),
    /// `abbrev` declaration
    Abbrev(Lean4Abbrev),
    /// `structure` declaration
    Structure(Lean4Structure),
    /// `inductive` type
    Inductive(Lean4Inductive),
    /// `instance` declaration
    Instance(Lean4Instance),
    /// `#check` command
    Check(Lean4Expr),
    /// `#eval` command
    Eval(Lean4Expr),
    /// Raw source string (escape hatch)
    Raw(std::string::String),
    /// Section: `section Name ... end Name`
    Section(std::string::String, Vec<Lean4Decl>),
    /// Namespace: `namespace Name ... end Name`
    Namespace(std::string::String, Vec<Lean4Decl>),
}
impl Lean4Decl {
    /// Emit the declaration as Lean 4 source.
    pub fn emit(&self) -> std::string::String {
        match self {
            Lean4Decl::Def(d) => d.emit(),
            Lean4Decl::Theorem(t) => t.emit(),
            Lean4Decl::Axiom(a) => a.emit(),
            Lean4Decl::Abbrev(a) => a.emit(),
            Lean4Decl::Structure(s) => s.emit(),
            Lean4Decl::Inductive(i) => i.emit(),
            Lean4Decl::Instance(inst) => inst.emit(),
            Lean4Decl::Check(e) => format!("#check {}\n", e),
            Lean4Decl::Eval(e) => format!("#eval {}\n", e),
            Lean4Decl::Raw(s) => format!("{}\n", s),
            Lean4Decl::Section(name, decls) => {
                let mut out = format!("section {}\n\n", name);
                for d in decls {
                    out.push_str(&d.emit());
                    out.push('\n');
                }
                out.push_str(&format!("end {}\n", name));
                out
            }
            Lean4Decl::Namespace(name, decls) => {
                let mut out = format!("namespace {}\n\n", name);
                for d in decls {
                    out.push_str(&d.emit());
                    out.push('\n');
                }
                out.push_str(&format!("end {}\n", name));
                out
            }
        }
    }
}
/// A complete Lean 4 source file.
#[derive(Debug, Clone)]
pub struct Lean4File {
    /// Module documentation comment
    pub module_doc: Option<std::string::String>,
    /// Import statements: `import Mathlib.Data.Nat.Basic`
    pub imports: Vec<std::string::String>,
    /// `open` statements: `open Nat List`
    pub opens: Vec<std::string::String>,
    /// Top-level declarations
    pub declarations: Vec<Lean4Decl>,
}
impl Lean4File {
    /// Create a new empty Lean 4 file.
    pub fn new() -> Self {
        Lean4File {
            module_doc: None,
            imports: vec![],
            opens: vec![],
            declarations: vec![],
        }
    }
    /// Add a standard import.
    pub fn with_import(mut self, import: impl Into<std::string::String>) -> Self {
        self.imports.push(import.into());
        self
    }
    /// Add an `open` declaration.
    pub fn with_open(mut self, open: impl Into<std::string::String>) -> Self {
        self.opens.push(open.into());
        self
    }
    /// Add a declaration.
    pub fn add_decl(&mut self, decl: Lean4Decl) {
        self.declarations.push(decl);
    }
    /// Generate the complete Lean 4 source.
    pub fn emit(&self) -> std::string::String {
        let mut out = std::string::String::new();
        if let Some(doc) = &self.module_doc {
            out.push_str(&format!("/-!\n{}\n-/\n\n", doc));
        }
        for import in &self.imports {
            out.push_str(&format!("import {}\n", import));
        }
        if !self.imports.is_empty() {
            out.push('\n');
        }
        for open in &self.opens {
            out.push_str(&format!("open {}\n", open));
        }
        if !self.opens.is_empty() {
            out.push('\n');
        }
        for decl in &self.declarations {
            out.push_str(&decl.emit());
            out.push('\n');
        }
        out
    }
}
/// Analysis cache for L4Ext.
#[allow(dead_code)]
#[derive(Debug)]
pub struct L4ExtCache {
    pub(super) entries: Vec<(u64, Vec<u8>, bool, u32)>,
    pub(super) cap: usize,
    pub(super) total_hits: u64,
    pub(super) total_misses: u64,
}
impl L4ExtCache {
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
/// A constructor of an inductive type.
#[derive(Debug, Clone)]
pub struct Lean4Constructor {
    /// Constructor name
    pub name: std::string::String,
    /// Field types with optional names
    pub fields: Vec<(Option<std::string::String>, Lean4Type)>,
}
impl Lean4Constructor {
    /// Simple constructor with positional fields.
    pub fn positional(name: impl Into<std::string::String>, fields: Vec<Lean4Type>) -> Self {
        Lean4Constructor {
            name: name.into(),
            fields: fields.into_iter().map(|t| (None, t)).collect(),
        }
    }
    /// Constructor with named fields (record-style).
    pub fn named(
        name: impl Into<std::string::String>,
        fields: Vec<(std::string::String, Lean4Type)>,
    ) -> Self {
        Lean4Constructor {
            name: name.into(),
            fields: fields.into_iter().map(|(n, t)| (Some(n), t)).collect(),
        }
    }
    pub fn emit(&self, type_name: &str) -> std::string::String {
        let mut out = std::string::String::new();
        out.push_str(&format!("  | {} : ", self.name));
        for (oname, ty) in &self.fields {
            if let Some(fname) = oname {
                out.push_str(&format!("({} : {}) → ", fname, ty));
            } else {
                out.push_str(&format!("{} → ", ty));
            }
        }
        out.push_str(type_name);
        out
    }
}
/// Worklist for L4Ext.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct L4ExtWorklist {
    pub(super) items: std::collections::VecDeque<usize>,
    pub(super) present: Vec<bool>,
}
impl L4ExtWorklist {
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
/// Configuration for L4Ext passes.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct L4ExtPassConfig {
    pub name: String,
    pub phase: L4ExtPassPhase,
    pub enabled: bool,
    pub max_iterations: usize,
    pub debug: u32,
    pub timeout_ms: Option<u64>,
}
impl L4ExtPassConfig {
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            phase: L4ExtPassPhase::Middle,
            enabled: true,
            max_iterations: 100,
            debug: 0,
            timeout_ms: None,
        }
    }
    #[allow(dead_code)]
    pub fn with_phase(mut self, phase: L4ExtPassPhase) -> Self {
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
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct L4PassStats {
    pub total_runs: u32,
    pub successful_runs: u32,
    pub total_changes: u64,
    pub time_ms: u64,
    pub iterations_used: u32,
}
impl L4PassStats {
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
/// A Lean 4 `instance` declaration.
#[derive(Debug, Clone)]
pub struct Lean4Instance {
    /// Optional instance name
    pub name: Option<std::string::String>,
    /// Type class application (the type being instantiated)
    pub ty: Lean4Type,
    /// Arguments (implicit, typeclass)
    pub args: Vec<(std::string::String, Lean4Type)>,
    /// The instance body (a `where` block or a term)
    pub body: Lean4Expr,
}
impl Lean4Instance {
    /// Emit the instance as Lean 4 source.
    pub fn emit(&self) -> std::string::String {
        let mut out = std::string::String::new();
        out.push_str("instance");
        if let Some(name) = &self.name {
            out.push(' ');
            out.push_str(name);
        }
        for (aname, aty) in &self.args {
            out.push_str(&format!(" ({} : {})", aname, aty));
        }
        out.push_str(&format!(" : {} :=\n  {}\n", self.ty, self.body));
        out
    }
}
/// Statements in a `do` block.
#[derive(Debug, Clone, PartialEq)]
pub enum Lean4DoStmt {
    /// `let x ← action`
    Bind(std::string::String, Option<Lean4Type>, Box<Lean4Expr>),
    /// `let x := value`
    LetBind(std::string::String, Option<Lean4Type>, Box<Lean4Expr>),
    /// Plain expression statement: `action`
    Expr(Box<Lean4Expr>),
    /// `return e`
    Return(Box<Lean4Expr>),
    /// `pure e`
    Pure(Box<Lean4Expr>),
    /// `if c then ...`
    If(Box<Lean4Expr>, Vec<Lean4DoStmt>, Vec<Lean4DoStmt>),
}
/// A Lean 4 `axiom` declaration.
#[derive(Debug, Clone)]
pub struct Lean4Axiom {
    /// Axiom name
    pub name: std::string::String,
    /// Arguments
    pub args: Vec<(std::string::String, Lean4Type)>,
    /// The type (proposition)
    pub ty: Lean4Type,
    /// Optional doc comment
    pub doc_comment: Option<std::string::String>,
}
impl Lean4Axiom {
    /// Emit the axiom as Lean 4 source.
    pub fn emit(&self) -> std::string::String {
        let mut out = std::string::String::new();
        if let Some(doc) = &self.doc_comment {
            out.push_str(&format!("/-- {} -/\n", doc));
        }
        out.push_str("axiom ");
        out.push_str(&self.name);
        for (aname, aty) in &self.args {
            out.push_str(&format!(" ({} : {})", aname, aty));
        }
        out.push_str(&format!(" : {}\n", self.ty));
        out
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct L4LivenessInfo {
    pub live_in: Vec<std::collections::HashSet<u32>>,
    pub live_out: Vec<std::collections::HashSet<u32>>,
    pub defs: Vec<std::collections::HashSet<u32>>,
    pub uses: Vec<std::collections::HashSet<u32>>,
}
impl L4LivenessInfo {
    #[allow(dead_code)]
    pub fn new(block_count: usize) -> Self {
        L4LivenessInfo {
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
/// Dependency graph for L4Ext.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct L4ExtDepGraph {
    pub(super) n: usize,
    pub(super) adj: Vec<Vec<usize>>,
    pub(super) rev: Vec<Vec<usize>>,
    pub(super) edge_count: usize,
}
impl L4ExtDepGraph {
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
/// Pass execution phase for L4Ext.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum L4ExtPassPhase {
    Early,
    Middle,
    Late,
    Finalize,
}
impl L4ExtPassPhase {
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
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct L4CacheEntry {
    pub key: String,
    pub data: Vec<u8>,
    pub timestamp: u64,
    pub valid: bool,
}
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum L4PassPhase {
    Analysis,
    Transformation,
    Verification,
    Cleanup,
}
impl L4PassPhase {
    #[allow(dead_code)]
    pub fn name(&self) -> &str {
        match self {
            L4PassPhase::Analysis => "analysis",
            L4PassPhase::Transformation => "transformation",
            L4PassPhase::Verification => "verification",
            L4PassPhase::Cleanup => "cleanup",
        }
    }
    #[allow(dead_code)]
    pub fn is_modifying(&self) -> bool {
        matches!(self, L4PassPhase::Transformation | L4PassPhase::Cleanup)
    }
}
/// Pattern in a `match` expression.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Lean4Pattern {
    /// Wildcard: `_`
    Wildcard,
    /// Variable binding: `x`
    Var(std::string::String),
    /// Constructor pattern: `.some x`, `List.cons h t`
    Ctor(std::string::String, Vec<Lean4Pattern>),
    /// Tuple pattern: `(a, b)`
    Tuple(Vec<Lean4Pattern>),
    /// Literal pattern: `0`, `true`, `"hello"`
    Lit(std::string::String),
    /// Or pattern: `p | q`
    Or(Box<Lean4Pattern>, Box<Lean4Pattern>),
    /// Anonymous constructor: `⟨a, b⟩`
    Anonymous(Vec<Lean4Pattern>),
}
/// A Lean 4 `theorem` declaration.
#[derive(Debug, Clone)]
pub struct Lean4Theorem {
    /// Name of the theorem
    pub name: std::string::String,
    /// Universe type parameters
    pub type_params: Vec<(std::string::String, std::string::String)>,
    /// Hypotheses/arguments: `(h : P)`
    pub args: Vec<(std::string::String, Lean4Type)>,
    /// The proposition type
    pub ty: Lean4Type,
    /// The proof term or tactic block
    pub proof: Lean4Expr,
    /// Optional doc comment
    pub doc_comment: Option<std::string::String>,
}
impl Lean4Theorem {
    /// Create a theorem with a tactic proof.
    pub fn tactic(
        name: impl Into<std::string::String>,
        args: Vec<(std::string::String, Lean4Type)>,
        ty: Lean4Type,
        tactics: Vec<std::string::String>,
    ) -> Self {
        Lean4Theorem {
            name: name.into(),
            type_params: vec![],
            args,
            ty,
            proof: Lean4Expr::ByTactic(tactics),
            doc_comment: None,
        }
    }
    /// Create a theorem with a term-mode proof.
    pub fn term_mode(
        name: impl Into<std::string::String>,
        args: Vec<(std::string::String, Lean4Type)>,
        ty: Lean4Type,
        proof: Lean4Expr,
    ) -> Self {
        Lean4Theorem {
            name: name.into(),
            type_params: vec![],
            args,
            ty,
            proof,
            doc_comment: None,
        }
    }
    /// Emit the theorem as Lean 4 source.
    pub fn emit(&self) -> std::string::String {
        let mut out = std::string::String::new();
        if let Some(doc) = &self.doc_comment {
            out.push_str(&format!("/-- {} -/\n", doc));
        }
        out.push_str("theorem ");
        out.push_str(&self.name);
        for (name, ty) in &self.args {
            out.push_str(&format!(" ({} : {})", name, ty));
        }
        out.push_str(&format!(" : {} :=\n  {}\n", self.ty, self.proof));
        out
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct L4AnalysisCache {
    pub(super) entries: std::collections::HashMap<String, L4CacheEntry>,
    pub(super) max_size: usize,
    pub(super) hits: u64,
    pub(super) misses: u64,
}
impl L4AnalysisCache {
    #[allow(dead_code)]
    pub fn new(max_size: usize) -> Self {
        L4AnalysisCache {
            entries: std::collections::HashMap::new(),
            max_size,
            hits: 0,
            misses: 0,
        }
    }
    #[allow(dead_code)]
    pub fn get(&mut self, key: &str) -> Option<&L4CacheEntry> {
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
            L4CacheEntry {
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
/// A Lean 4 `inductive` type declaration.
#[derive(Debug, Clone)]
pub struct Lean4Inductive {
    /// Type name
    pub name: std::string::String,
    /// Type parameters: `(α : Type u)`
    pub type_params: Vec<(std::string::String, Lean4Type)>,
    /// Index types (GADTs)
    pub indices: Vec<Lean4Type>,
    /// Constructors
    pub constructors: Vec<Lean4Constructor>,
    /// Derives (e.g., `Repr`, `DecidableEq`, `BEq`)
    pub derives: Vec<std::string::String>,
    /// Optional doc comment
    pub doc_comment: Option<std::string::String>,
}
impl Lean4Inductive {
    /// Create a simple inductive type.
    pub fn simple(
        name: impl Into<std::string::String>,
        constructors: Vec<Lean4Constructor>,
    ) -> Self {
        Lean4Inductive {
            name: name.into(),
            type_params: vec![],
            indices: vec![],
            constructors,
            derives: vec![],
            doc_comment: None,
        }
    }
    /// Emit the inductive as Lean 4 source.
    pub fn emit(&self) -> std::string::String {
        let mut out = std::string::String::new();
        if let Some(doc) = &self.doc_comment {
            out.push_str(&format!("/-- {} -/\n", doc));
        }
        out.push_str("inductive ");
        out.push_str(&self.name);
        for (pname, pty) in &self.type_params {
            out.push_str(&format!(" ({} : {})", pname, pty));
        }
        if !self.indices.is_empty() {
            out.push_str(" : ");
            for idx in &self.indices {
                out.push_str(&format!("{} → ", idx));
            }
            out.push_str("Type");
        }
        out.push_str(" where\n");
        for ctor in &self.constructors {
            out.push_str(&ctor.emit(&self.name));
            out.push('\n');
        }
        if !self.derives.is_empty() {
            out.push_str(&format!("  deriving {}\n", self.derives.join(", ")));
        }
        out
    }
}
/// A Lean 4 `structure` declaration.
#[derive(Debug, Clone)]
pub struct Lean4Structure {
    /// Structure name
    pub name: std::string::String,
    /// Type parameters
    pub type_params: Vec<(std::string::String, Lean4Type)>,
    /// Parent structures (extends)
    pub extends: Vec<std::string::String>,
    /// Fields: (name, type, default_value)
    pub fields: Vec<(std::string::String, Lean4Type, Option<Lean4Expr>)>,
    /// Optional doc comment
    pub doc_comment: Option<std::string::String>,
    /// Derives
    pub derives: Vec<std::string::String>,
}
impl Lean4Structure {
    /// Create a simple structure.
    pub fn simple(
        name: impl Into<std::string::String>,
        fields: Vec<(std::string::String, Lean4Type)>,
    ) -> Self {
        Lean4Structure {
            name: name.into(),
            type_params: vec![],
            extends: vec![],
            fields: fields.into_iter().map(|(n, t)| (n, t, None)).collect(),
            doc_comment: None,
            derives: vec![],
        }
    }
    /// Emit the structure as Lean 4 source.
    pub fn emit(&self) -> std::string::String {
        let mut out = std::string::String::new();
        if let Some(doc) = &self.doc_comment {
            out.push_str(&format!("/-- {} -/\n", doc));
        }
        out.push_str("structure ");
        out.push_str(&self.name);
        for (pname, pty) in &self.type_params {
            out.push_str(&format!(" ({} : {})", pname, pty));
        }
        if !self.extends.is_empty() {
            out.push_str(&format!(" extends {}", self.extends.join(", ")));
        }
        out.push_str(" where\n");
        for (fname, fty, default) in &self.fields {
            if let Some(def_val) = default {
                out.push_str(&format!("  {} : {} := {}\n", fname, fty, def_val));
            } else {
                out.push_str(&format!("  {} : {}\n", fname, fty));
            }
        }
        if !self.derives.is_empty() {
            out.push_str(&format!("  deriving {}\n", self.derives.join(", ")));
        }
        out
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct L4Worklist {
    pub(super) items: std::collections::VecDeque<u32>,
    pub(super) in_worklist: std::collections::HashSet<u32>,
}
impl L4Worklist {
    #[allow(dead_code)]
    pub fn new() -> Self {
        L4Worklist {
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
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct L4DominatorTree {
    pub idom: Vec<Option<u32>>,
    pub dom_children: Vec<Vec<u32>>,
    pub dom_depth: Vec<u32>,
}
impl L4DominatorTree {
    #[allow(dead_code)]
    pub fn new(size: usize) -> Self {
        L4DominatorTree {
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
/// A Lean 4 `abbrev` declaration (transparent definition).
#[derive(Debug, Clone)]
pub struct Lean4Abbrev {
    /// Name
    pub name: std::string::String,
    /// Arguments
    pub args: Vec<(std::string::String, Lean4Type)>,
    /// Return type
    pub ty: Option<Lean4Type>,
    /// Body
    pub body: Lean4Expr,
}
impl Lean4Abbrev {
    /// Emit the abbrev as Lean 4 source.
    pub fn emit(&self) -> std::string::String {
        let mut out = std::string::String::new();
        out.push_str("abbrev ");
        out.push_str(&self.name);
        for (aname, aty) in &self.args {
            out.push_str(&format!(" ({} : {})", aname, aty));
        }
        if let Some(ty) = &self.ty {
            out.push_str(&format!(" : {}", ty));
        }
        out.push_str(&format!(" := {}\n", self.body));
        out
    }
}
#[allow(dead_code)]
pub struct L4ConstantFoldingHelper;
impl L4ConstantFoldingHelper {
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
/// Liveness analysis for L4Ext.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct L4ExtLiveness {
    pub live_in: Vec<Vec<usize>>,
    pub live_out: Vec<Vec<usize>>,
    pub defs: Vec<Vec<usize>>,
    pub uses: Vec<Vec<usize>>,
}
impl L4ExtLiveness {
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
/// Pass registry for L4Ext.
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct L4ExtPassRegistry {
    pub(super) configs: Vec<L4ExtPassConfig>,
    pub(super) stats: Vec<L4ExtPassStats>,
}
impl L4ExtPassRegistry {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    #[allow(dead_code)]
    pub fn register(&mut self, c: L4ExtPassConfig) {
        self.stats.push(L4ExtPassStats::new());
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
    pub fn get(&self, i: usize) -> Option<&L4ExtPassConfig> {
        self.configs.get(i)
    }
    #[allow(dead_code)]
    pub fn get_stats(&self, i: usize) -> Option<&L4ExtPassStats> {
        self.stats.get(i)
    }
    #[allow(dead_code)]
    pub fn enabled_passes(&self) -> Vec<&L4ExtPassConfig> {
        self.configs.iter().filter(|c| c.enabled).collect()
    }
    #[allow(dead_code)]
    pub fn passes_in_phase(&self, ph: &L4ExtPassPhase) -> Vec<&L4ExtPassConfig> {
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
/// Statistics for L4Ext passes.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct L4ExtPassStats {
    pub iterations: usize,
    pub changed: bool,
    pub nodes_visited: usize,
    pub nodes_modified: usize,
    pub time_ms: u64,
    pub memory_bytes: usize,
    pub errors: usize,
}
impl L4ExtPassStats {
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
    pub fn merge(&mut self, o: &L4ExtPassStats) {
        self.iterations += o.iterations;
        self.changed |= o.changed;
        self.nodes_visited += o.nodes_visited;
        self.nodes_modified += o.nodes_modified;
        self.time_ms += o.time_ms;
        self.memory_bytes = self.memory_bytes.max(o.memory_bytes);
        self.errors += o.errors;
    }
}
/// Constant folding helper for L4Ext.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct L4ExtConstFolder {
    pub(super) folds: usize,
    pub(super) failures: usize,
    pub(super) enabled: bool,
}
impl L4ExtConstFolder {
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
/// A Lean 4 `def` definition.
#[derive(Debug, Clone)]
pub struct Lean4Def {
    /// Name of the definition
    pub name: std::string::String,
    /// Universe-polymorphic type parameters: `{u v : Type}`
    pub type_params: Vec<(std::string::String, std::string::String)>,
    /// Term-level arguments: `(x : Nat) (y : Bool)`
    pub args: Vec<(std::string::String, Lean4Type)>,
    /// Return type ascription
    pub type_ascription: Option<Lean4Type>,
    /// Body expression
    pub body: Lean4Expr,
    /// Optional doc comment
    pub doc_comment: Option<std::string::String>,
    /// Lean attributes: `@[simp]`, `@[inline]`, etc.
    pub attributes: Vec<std::string::String>,
    /// `noncomputable`
    pub is_noncomputable: bool,
    /// `private`
    pub is_private: bool,
}
impl Lean4Def {
    /// Create a simple definition.
    pub fn simple(
        name: impl Into<std::string::String>,
        args: Vec<(std::string::String, Lean4Type)>,
        ret_ty: Lean4Type,
        body: Lean4Expr,
    ) -> Self {
        Lean4Def {
            name: name.into(),
            type_params: vec![],
            args,
            type_ascription: Some(ret_ty),
            body,
            doc_comment: None,
            attributes: vec![],
            is_noncomputable: false,
            is_private: false,
        }
    }
    /// Emit the definition as Lean 4 source.
    pub fn emit(&self) -> std::string::String {
        let mut out = std::string::String::new();
        if let Some(doc) = &self.doc_comment {
            out.push_str(&format!("/-- {} -/\n", doc));
        }
        for attr in &self.attributes {
            out.push_str(&format!("@[{}]\n", attr));
        }
        if self.is_noncomputable {
            out.push_str("noncomputable ");
        }
        if self.is_private {
            out.push_str("private ");
        }
        out.push_str("def ");
        out.push_str(&self.name);
        if !self.type_params.is_empty() {
            out.push_str(".{");
            for (i, (n, _k)) in self.type_params.iter().enumerate() {
                if i > 0 {
                    out.push_str(", ");
                }
                out.push_str(n);
            }
            out.push('}');
        }
        for (name, ty) in &self.args {
            out.push_str(&format!(" ({} : {})", name, ty));
        }
        if let Some(ty) = &self.type_ascription {
            out.push_str(&format!(" : {}", ty));
        }
        out.push_str(&format!(" :=\n  {}\n", self.body));
        out
    }
}
/// One step in a `calc` proof.
#[derive(Debug, Clone, PartialEq)]
pub struct Lean4CalcStep {
    /// Left-hand side expression
    pub lhs: Lean4Expr,
    /// Relation: `=`, `≤`, `<`, `≥`, `>`, etc.
    pub relation: std::string::String,
    /// Right-hand side expression
    pub rhs: Lean4Expr,
    /// Justification: `by ...` or a term
    pub justification: Lean4Expr,
}
/// Lean 4 expression representation.
#[derive(Debug, Clone, PartialEq)]
pub enum Lean4Expr {
    /// Variable reference: `x`
    Var(std::string::String),
    /// Natural number literal: `42`
    NatLit(u64),
    /// Integer literal: `-5`
    IntLit(i64),
    /// Boolean literal: `true` / `false`
    BoolLit(bool),
    /// String literal: `"hello"`
    StrLit(std::string::String),
    /// Float literal: `3.14`
    FloatLit(f64),
    /// Hole / sorry placeholder: `_`
    Hole,
    /// `sorry`
    Sorry,
    /// Panic: `panic! "message"`
    Panic(std::string::String),
    /// Function application: `f x`
    App(Box<Lean4Expr>, Box<Lean4Expr>),
    /// Lambda: `fun x => body`
    Lambda(std::string::String, Option<Box<Lean4Type>>, Box<Lean4Expr>),
    /// Dependent function type (Pi): `(x : α) → β x`
    Pi(std::string::String, Box<Lean4Type>, Box<Lean4Expr>),
    /// Let binding: `let x := e; body`
    Let(
        std::string::String,
        Option<Box<Lean4Type>>,
        Box<Lean4Expr>,
        Box<Lean4Expr>,
    ),
    /// Recursive let: `let rec f := ...`
    LetRec(std::string::String, Box<Lean4Expr>, Box<Lean4Expr>),
    /// Pattern match: `match e with | p => b | ...`
    Match(Box<Lean4Expr>, Vec<(Lean4Pattern, Lean4Expr)>),
    /// If-then-else: `if c then t else e`
    If(Box<Lean4Expr>, Box<Lean4Expr>, Box<Lean4Expr>),
    /// Do notation block
    Do(Vec<Lean4DoStmt>),
    /// `have h : T := proof; rest`
    Have(
        std::string::String,
        Box<Lean4Type>,
        Box<Lean4Expr>,
        Box<Lean4Expr>,
    ),
    /// `show T from e`
    Show(Box<Lean4Type>, Box<Lean4Expr>),
    /// Calc block
    Calc(Vec<Lean4CalcStep>),
    /// Tactic block: `by tac1; tac2; ...`
    ByTactic(Vec<std::string::String>),
    /// Type ascription: `(e : T)`
    Ascription(Box<Lean4Expr>, Box<Lean4Type>),
    /// Tuple: `(a, b)`
    Tuple(Vec<Lean4Expr>),
    /// Anonymous constructor: `⟨a, b⟩`
    AnonymousCtor(Vec<Lean4Expr>),
    /// Projection: `e.1`, `e.field`
    Proj(Box<Lean4Expr>, std::string::String),
    /// Structure literal: `{ field := val, ... }`
    StructLit(std::string::String, Vec<(std::string::String, Lean4Expr)>),
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct L4PassConfig {
    pub phase: L4PassPhase,
    pub enabled: bool,
    pub max_iterations: u32,
    pub debug_output: bool,
    pub pass_name: String,
}
impl L4PassConfig {
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>, phase: L4PassPhase) -> Self {
        L4PassConfig {
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
/// Dominator tree for L4Ext.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct L4ExtDomTree {
    pub(super) idom: Vec<Option<usize>>,
    pub(super) children: Vec<Vec<usize>>,
    pub(super) depth: Vec<usize>,
}
impl L4ExtDomTree {
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
