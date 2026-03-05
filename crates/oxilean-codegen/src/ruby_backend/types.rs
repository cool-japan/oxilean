//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;
use std::collections::HashSet;

use super::functions::RUBY_RUNTIME;

use super::functions::*;
use std::collections::{HashMap, VecDeque};
use std::fmt;
use std::fmt::Write as FmtWrite;

/// Ruby name mangler
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct RubyNameMangler {
    pub used: std::collections::HashSet<String>,
    pub map: std::collections::HashMap<String, String>,
}
#[allow(dead_code)]
impl RubyNameMangler {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn mangle_constant(&mut self, name: &str) -> String {
        let mut mangled: String = name
            .chars()
            .map(|c| {
                if c.is_alphanumeric() || c == '_' {
                    c
                } else {
                    '_'
                }
            })
            .collect();
        if !mangled.starts_with(|c: char| c.is_uppercase()) {
            mangled = format!("Ox{}", mangled);
        }
        let base = mangled.clone();
        let mut cnt = 0;
        while self.used.contains(&mangled) {
            cnt += 1;
            mangled = format!("{}_{}", base, cnt);
        }
        self.used.insert(mangled.clone());
        self.map.insert(name.to_string(), mangled.clone());
        mangled
    }
    pub fn mangle_method(&mut self, name: &str) -> String {
        let mangled: String = name
            .chars()
            .map(|c| {
                if c.is_alphanumeric() || c == '_' {
                    c
                } else {
                    '_'
                }
            })
            .collect();
        let ruby_reserved = [
            "__method__",
            "__dir__",
            "__callee__",
            "begin",
            "end",
            "do",
            "if",
            "unless",
            "while",
            "until",
            "for",
            "return",
            "yield",
            "class",
            "module",
            "def",
            "alias",
            "and",
            "or",
            "not",
            "in",
            "then",
            "case",
            "when",
            "rescue",
            "ensure",
        ];
        let base = if ruby_reserved.contains(&mangled.as_str()) {
            format!("ox_{}", mangled)
        } else {
            mangled
        };
        let mut candidate = base.clone();
        let mut cnt = 0;
        while self.used.contains(&candidate) {
            cnt += 1;
            candidate = format!("{}_{}", base, cnt);
        }
        self.used.insert(candidate.clone());
        self.map.insert(name.to_string(), candidate.clone());
        candidate
    }
}
/// Ruby method alias
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RubyAlias {
    pub new_name: String,
    pub old_name: String,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RubyLivenessInfo {
    pub live_in: Vec<std::collections::HashSet<u32>>,
    pub live_out: Vec<std::collections::HashSet<u32>>,
    pub defs: Vec<std::collections::HashSet<u32>>,
    pub uses: Vec<std::collections::HashSet<u32>>,
}
impl RubyLivenessInfo {
    #[allow(dead_code)]
    pub fn new(block_count: usize) -> Self {
        RubyLivenessInfo {
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
pub struct RubyConstantFoldingHelper;
impl RubyConstantFoldingHelper {
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
pub struct RubyDepGraph {
    pub(super) nodes: Vec<u32>,
    pub(super) edges: Vec<(u32, u32)>,
}
impl RubyDepGraph {
    #[allow(dead_code)]
    pub fn new() -> Self {
        RubyDepGraph {
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
/// Ruby source buffer
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct RubyExtSourceBuffer {
    pub sections: Vec<(String, String)>,
    pub current: String,
    pub indent: usize,
}
#[allow(dead_code)]
impl RubyExtSourceBuffer {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn write(&mut self, s: &str) {
        let pad = "  ".repeat(self.indent);
        self.current.push_str(&pad);
        self.current.push_str(s);
    }
    pub fn writeln(&mut self, s: &str) {
        let pad = "  ".repeat(self.indent);
        self.current.push_str(&pad);
        self.current.push_str(s);
        self.current.push('\n');
    }
    pub fn indent(&mut self) {
        self.indent += 1;
    }
    pub fn dedent(&mut self) {
        if self.indent > 0 {
            self.indent -= 1;
        }
    }
    pub fn finish(mut self) -> String {
        let done = std::mem::take(&mut self.current);
        if !done.is_empty() {
            self.sections.push(("".to_string(), done));
        }
        self.sections
            .iter()
            .map(|(_, s)| s.as_str())
            .collect::<Vec<_>>()
            .join("")
    }
}
/// Ruby proc vs lambda differences
#[allow(dead_code)]
pub struct RubyProcLambdaDiff {
    pub arity_strict: bool,
    pub return_behavior: String,
}
/// Ruby class definition
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RubyClassDef {
    pub name: String,
    pub superclass: Option<String>,
    pub includes: Vec<String>,
    pub extends: Vec<String>,
    pub prepends: Vec<String>,
    pub methods: Vec<RubyMethodDef>,
    pub attrs: Vec<(String, bool, bool)>,
    pub constants: Vec<(String, String)>,
}
/// Ruby final pass summary
#[allow(dead_code)]
#[derive(Debug, Default, Clone)]
pub struct RubyPassSummary {
    pub pass_name: String,
    pub functions_compiled: usize,
    pub classes_emitted: usize,
    pub modules_emitted: usize,
    pub duration_us: u64,
}
/// Ruby class registry
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct RubyClassRegistry {
    pub classes: Vec<RubyClassDef>,
    pub modules: Vec<RubyModuleDef>,
}
#[allow(dead_code)]
impl RubyClassRegistry {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn add_class(&mut self, c: RubyClassDef) {
        self.classes.push(c);
    }
    pub fn add_module(&mut self, m: RubyModuleDef) {
        self.modules.push(m);
    }
    pub fn total_items(&self) -> usize {
        self.classes.len() + self.modules.len()
    }
    pub fn emit_all(&self) -> String {
        let mut out = String::new();
        for m in &self.modules {
            out.push_str(&format!("{}\n\n", m));
        }
        for c in &self.classes {
            out.push_str(&format!("{}\n\n", c));
        }
        out
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RubyAnalysisCache {
    pub(super) entries: std::collections::HashMap<String, RubyCacheEntry>,
    pub(super) max_size: usize,
    pub(super) hits: u64,
    pub(super) misses: u64,
}
impl RubyAnalysisCache {
    #[allow(dead_code)]
    pub fn new(max_size: usize) -> Self {
        RubyAnalysisCache {
            entries: std::collections::HashMap::new(),
            max_size,
            hits: 0,
            misses: 0,
        }
    }
    #[allow(dead_code)]
    pub fn get(&mut self, key: &str) -> Option<&RubyCacheEntry> {
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
            RubyCacheEntry {
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
/// A Ruby method definition (`def name(params) ... end`).
#[derive(Debug, Clone, PartialEq)]
pub struct RubyMethod {
    /// Method name (snake_case).
    pub name: std::string::String,
    /// Parameter names.
    pub params: Vec<std::string::String>,
    /// Body statements.
    pub body: Vec<RubyStmt>,
    /// Visibility modifier (default: Public).
    pub visibility: RubyVisibility,
}
impl RubyMethod {
    /// Create a new public method.
    pub fn new(name: &str, params: Vec<&str>, body: Vec<RubyStmt>) -> Self {
        RubyMethod {
            name: name.to_string(),
            params: params.into_iter().map(|s| s.to_string()).collect(),
            body,
            visibility: RubyVisibility::Public,
        }
    }
    /// Create a private method.
    pub fn private(name: &str, params: Vec<&str>, body: Vec<RubyStmt>) -> Self {
        RubyMethod {
            name: name.to_string(),
            params: params.into_iter().map(|s| s.to_string()).collect(),
            body,
            visibility: RubyVisibility::Private,
        }
    }
}
/// Ruby method visibility modifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RubyVisibility {
    /// Public (default) — callable from anywhere
    Public,
    /// Protected — callable from the class and subclasses
    Protected,
    /// Private — callable only within the defining class
    Private,
}
/// Ruby id generator
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct RubyExtIdGen {
    pub(super) counter: u64,
    pub(super) prefix: String,
}
#[allow(dead_code)]
impl RubyExtIdGen {
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
/// Ruby diagnostic
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RubyDiagLevel {
    Info,
    Warning,
    Error,
}
/// Ruby fiber definition
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RubyFiber {
    pub name: String,
    pub params: Vec<String>,
    pub body: String,
    pub is_async: bool,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RubyCacheEntry {
    pub key: String,
    pub data: Vec<u8>,
    pub timestamp: u64,
    pub valid: bool,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RubyPassConfig {
    pub phase: RubyPassPhase,
    pub enabled: bool,
    pub max_iterations: u32,
    pub debug_output: bool,
    pub pass_name: String,
}
impl RubyPassConfig {
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>, phase: RubyPassPhase) -> Self {
        RubyPassConfig {
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
pub struct RubyWorklist {
    pub(super) items: std::collections::VecDeque<u32>,
    pub(super) in_worklist: std::collections::HashSet<u32>,
}
impl RubyWorklist {
    #[allow(dead_code)]
    pub fn new() -> Self {
        RubyWorklist {
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
pub struct RubyDominatorTree {
    pub idom: Vec<Option<u32>>,
    pub dom_children: Vec<Vec<u32>>,
    pub dom_depth: Vec<u32>,
}
impl RubyDominatorTree {
    #[allow(dead_code)]
    pub fn new(size: usize) -> Self {
        RubyDominatorTree {
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
/// Ruby method contract
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RubyMethodContract {
    pub preconditions: Vec<String>,
    pub postconditions: Vec<String>,
}
#[allow(dead_code)]
impl RubyMethodContract {
    pub fn new() -> Self {
        Self {
            preconditions: Vec::new(),
            postconditions: Vec::new(),
        }
    }
    pub fn add_pre(&mut self, cond: &str) {
        self.preconditions.push(cond.to_string());
    }
    pub fn add_post(&mut self, cond: &str) {
        self.postconditions.push(cond.to_string());
    }
    pub fn emit_assertions(&self) -> String {
        let mut out = String::new();
        for pre in &self.preconditions {
            out.push_str(&format!("raise ArgumentError unless {}\n", pre));
        }
        for post in &self.postconditions {
            out.push_str(&format!("raise RuntimeError unless {}\n", post));
        }
        out
    }
}
/// Ruby type representation for type-directed code generation.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RubyType {
    /// `Integer` — arbitrary-precision integer (maps to Lean Nat)
    Integer,
    /// `Float` — 64-bit IEEE 754 float
    Float,
    /// `String`
    String,
    /// `TrueClass` / `FalseClass` — boolean
    Bool,
    /// `NilClass`
    Nil,
    /// `Array` — homogeneous list
    Array(Box<RubyType>),
    /// `Hash` — key-value map
    Hash(Box<RubyType>, Box<RubyType>),
    /// `Symbol`
    Symbol,
    /// Arbitrary named class / module
    Object(std::string::String),
    /// `Proc` — first-class callable
    Proc,
}
/// Ruby pattern matching (Ruby 3.x)
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum RubyPattern {
    Pin(String),
    Variable(String),
    Literal(String),
    Array(Vec<RubyPattern>),
    Hash(Vec<(String, Option<RubyPattern>)>),
    Find(Vec<RubyPattern>),
    Deconstruct(String, Vec<(String, RubyPattern)>),
    Guard(Box<RubyPattern>, String),
}
#[allow(dead_code)]
pub struct RubyPassRegistry {
    pub(super) configs: Vec<RubyPassConfig>,
    pub(super) stats: std::collections::HashMap<String, RubyPassStats>,
}
impl RubyPassRegistry {
    #[allow(dead_code)]
    pub fn new() -> Self {
        RubyPassRegistry {
            configs: Vec::new(),
            stats: std::collections::HashMap::new(),
        }
    }
    #[allow(dead_code)]
    pub fn register(&mut self, config: RubyPassConfig) {
        self.stats
            .insert(config.pass_name.clone(), RubyPassStats::new());
        self.configs.push(config);
    }
    #[allow(dead_code)]
    pub fn enabled_passes(&self) -> Vec<&RubyPassConfig> {
        self.configs.iter().filter(|c| c.enabled).collect()
    }
    #[allow(dead_code)]
    pub fn get_stats(&self, name: &str) -> Option<&RubyPassStats> {
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
/// Ruby type (extended)
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum RubyTypeExt {
    Integer,
    Float,
    String,
    Symbol,
    Bool,
    Nil,
    Array(Box<RubyTypeExt>),
    Hash(Box<RubyTypeExt>, Box<RubyTypeExt>),
    Proc,
    Lambda,
    Range,
    Struct(String),
    Class(String),
    Module(String),
    Any,
}
/// Ruby literal values.
#[derive(Debug, Clone, PartialEq)]
pub enum RubyLit {
    /// Integer literal: `42`, `0`, `-7`
    Int(i64),
    /// Float literal: `3.14`
    Float(f64),
    /// String literal: `"hello"`
    Str(std::string::String),
    /// Boolean literal: `true` or `false`
    Bool(bool),
    /// `nil` literal
    Nil,
    /// Symbol literal: `:name`
    Symbol(std::string::String),
}
/// Ruby begin/rescue/ensure
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RubyRescueBlock {
    pub body: String,
    pub rescues: Vec<(Vec<String>, Option<String>, String)>,
    pub ensure: Option<String>,
}
/// Ruby case/in expression (pattern matching)
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RubyCaseIn {
    pub scrutinee: String,
    pub arms: Vec<(RubyPattern, String)>,
    pub else_body: Option<String>,
}
/// Ruby statement AST.
#[derive(Debug, Clone, PartialEq)]
pub enum RubyStmt {
    /// Standalone expression statement.
    Expr(RubyExpr),
    /// Local variable assignment: `name = expr`
    Assign(std::string::String, RubyExpr),
    /// Method definition: `def name(params) ... end`
    Def(RubyMethod),
    /// Class definition: `class Name ... end`
    Class(RubyClass),
    /// Module definition: `module Name ... end`
    Mod(RubyModule),
    /// `if cond ... elsif ... else ... end`
    If(
        RubyExpr,
        Vec<RubyStmt>,
        Vec<(RubyExpr, Vec<RubyStmt>)>,
        Option<Vec<RubyStmt>>,
    ),
    /// `while cond ... end`
    While(RubyExpr, Vec<RubyStmt>),
    /// `return expr`
    Return(RubyExpr),
    /// `begin ... rescue ... ensure ... end`
    Begin(
        Vec<RubyStmt>,
        Option<(std::string::String, Vec<RubyStmt>)>,
        Option<Vec<RubyStmt>>,
    ),
}
/// Ruby expression AST.
#[derive(Debug, Clone, PartialEq)]
pub enum RubyExpr {
    /// Literal value: `42`, `"hello"`, `:sym`, `nil`
    Lit(RubyLit),
    /// Variable / local name: `x`, `result`, `_t0`
    Var(std::string::String),
    /// Binary operator: `lhs + rhs`, `a == b`, `x && y`
    BinOp(std::string::String, Box<RubyExpr>, Box<RubyExpr>),
    /// Unary operator: `!x`, `-n`, `~bits`
    UnaryOp(std::string::String, Box<RubyExpr>),
    /// Free function call: `foo(a, b)`
    Call(std::string::String, Vec<RubyExpr>),
    /// Method call: `obj.method(a, b)`
    MethodCall(Box<RubyExpr>, std::string::String, Vec<RubyExpr>),
    /// Block with brace syntax: `{ |x| expr }`
    Block(Vec<std::string::String>, Vec<RubyStmt>),
    /// Lambda (stabby): `->(x, y) { body }`
    Lambda(Vec<std::string::String>, Vec<RubyStmt>),
    /// Ternary / conditional expression: `cond ? then_e : else_e`
    If(Box<RubyExpr>, Box<RubyExpr>, Box<RubyExpr>),
    /// `case` expression with `when` branches (value-based)
    Case(
        Box<RubyExpr>,
        Vec<(RubyExpr, RubyExpr)>,
        Option<Box<RubyExpr>>,
    ),
    /// Array literal: `[a, b, c]`
    Array(Vec<RubyExpr>),
    /// Hash literal: `{ key: val, ... }` (symbol keys) or `{ "k" => v }`
    Hash(Vec<(RubyExpr, RubyExpr)>),
    /// Local variable assignment expression: `x = expr` (returns the rhs)
    Assign(std::string::String, Box<RubyExpr>),
    /// `return expr`
    Return(Box<RubyExpr>),
}
/// Ruby backend config (extended)
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RubyExtConfig {
    pub ruby_version: String,
    pub use_sorbet: bool,
    pub use_rbs: bool,
    pub frozen_string_literals: bool,
    pub encoding: String,
    pub indent_size: usize,
    pub use_keyword_args: bool,
}
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum RubyPassPhase {
    Analysis,
    Transformation,
    Verification,
    Cleanup,
}
impl RubyPassPhase {
    #[allow(dead_code)]
    pub fn name(&self) -> &str {
        match self {
            RubyPassPhase::Analysis => "analysis",
            RubyPassPhase::Transformation => "transformation",
            RubyPassPhase::Verification => "verification",
            RubyPassPhase::Cleanup => "cleanup",
        }
    }
    #[allow(dead_code)]
    pub fn is_modifying(&self) -> bool {
        matches!(self, RubyPassPhase::Transformation | RubyPassPhase::Cleanup)
    }
}
/// Ruby module definition
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RubyModuleDef {
    pub name: String,
    pub includes: Vec<String>,
    pub methods: Vec<RubyMethodDef>,
    pub constants: Vec<(String, String)>,
}
/// Ruby block parameter
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RubyBlock {
    pub params: Vec<String>,
    pub body: String,
    pub is_proc: bool,
}
/// Ruby require statement
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum RubyRequire {
    Require(String),
    RequireRelative(String),
    Autoload(String, String),
}
/// Ruby method definition
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RubyMethodDef {
    pub name: String,
    pub params: Vec<(String, Option<RubyTypeExt>)>,
    pub return_type: Option<RubyTypeExt>,
    pub body: String,
    pub visibility: RubyVisibility,
    pub is_class_method: bool,
    pub is_abstract: bool,
}
/// Ruby Enumerator::Lazy
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RubyLazyEnumerator {
    pub source: String,
    pub transforms: Vec<String>,
}
/// A Ruby class definition.
#[derive(Debug, Clone, PartialEq)]
pub struct RubyClass {
    /// Class name (CamelCase).
    pub name: std::string::String,
    /// Optional superclass name.
    pub superclass: Option<std::string::String>,
    /// Instance methods.
    pub methods: Vec<RubyMethod>,
    /// Class-level methods (will be emitted as `def self.name`).
    pub class_methods: Vec<RubyMethod>,
    /// `attr_reader` accessor names.
    pub attr_readers: Vec<std::string::String>,
    /// `attr_writer` accessor names.
    pub attr_writers: Vec<std::string::String>,
}
impl RubyClass {
    /// Create a new empty class.
    pub fn new(name: &str) -> Self {
        RubyClass {
            name: name.to_string(),
            superclass: None,
            methods: Vec::new(),
            class_methods: Vec::new(),
            attr_readers: Vec::new(),
            attr_writers: Vec::new(),
        }
    }
    /// Set the superclass.
    pub fn with_superclass(mut self, superclass: &str) -> Self {
        self.superclass = Some(superclass.to_string());
        self
    }
    /// Add an `attr_reader`.
    pub fn add_attr_reader(&mut self, name: &str) {
        self.attr_readers.push(name.to_string());
    }
    /// Add an instance method.
    pub fn add_method(&mut self, method: RubyMethod) {
        self.methods.push(method);
    }
    /// Add a class method.
    pub fn add_class_method(&mut self, method: RubyMethod) {
        self.class_methods.push(method);
    }
}
/// Ruby code stats
#[allow(dead_code)]
#[derive(Debug, Default, Clone)]
pub struct RubyCodeStats {
    pub classes: usize,
    pub modules: usize,
    pub methods: usize,
    pub lambdas: usize,
    pub blocks: usize,
    pub total_lines: usize,
}
/// Ruby backend code stats v2
#[allow(dead_code)]
#[derive(Debug, Default, Clone)]
pub struct RubyBackendCodeStats {
    pub files: usize,
    pub classes: usize,
    pub modules: usize,
    pub methods: usize,
    pub lines: usize,
}
/// Ruby emit stats
#[allow(dead_code)]
#[derive(Debug, Default, Clone)]
pub struct RubyExtEmitStats {
    pub bytes_written: usize,
    pub items_emitted: usize,
    pub errors: usize,
    pub warnings: usize,
    pub classes_emitted: usize,
    pub modules_emitted: usize,
    pub methods_emitted: usize,
}
/// Ruby struct definition
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RubyStructDef {
    pub name: String,
    pub members: Vec<(String, Option<RubyTypeExt>)>,
}
/// A Ruby module definition (also used as the top-level compilation unit).
#[derive(Debug, Clone, PartialEq)]
pub struct RubyModule {
    /// Module name (CamelCase).
    pub name: std::string::String,
    /// Module-level constants and assignments.
    pub constants: Vec<(std::string::String, RubyExpr)>,
    /// Module methods (emitted as `module_function` or `def self.name`).
    pub functions: Vec<RubyMethod>,
    /// Nested classes inside this module.
    pub classes: Vec<RubyClass>,
    /// Nested sub-modules.
    pub submodules: Vec<RubyModule>,
    /// Whether to emit `module_function` marker before functions.
    pub module_function: bool,
}
impl RubyModule {
    /// Create a new empty module.
    pub fn new(name: &str) -> Self {
        RubyModule {
            name: name.to_string(),
            constants: Vec::new(),
            functions: Vec::new(),
            classes: Vec::new(),
            submodules: Vec::new(),
            module_function: true,
        }
    }
    /// Generate valid Ruby source for this module.
    pub fn emit(&self) -> std::string::String {
        let mut out = std::string::String::new();
        writeln!(out, "# frozen_string_literal: true").expect("writing to String never fails");
        writeln!(out).expect("writing to String never fails");
        self.emit_module_body(&mut out, "");
        out
    }
    pub(super) fn emit_module_body(&self, out: &mut std::string::String, indent: &str) {
        let inner = format!("{}  ", indent);
        writeln!(out, "{}module {}", indent, self.name).expect("writing to String never fails");
        for (name, expr) in &self.constants {
            writeln!(out, "{}{} = {}", inner, name, expr).expect("writing to String never fails");
        }
        if !self.constants.is_empty() {
            writeln!(out).expect("writing to String never fails");
        }
        for submod in &self.submodules {
            submod.emit_module_body(out, &inner);
            writeln!(out).expect("writing to String never fails");
        }
        for class in &self.classes {
            let mut fmt_buf = std::string::String::new();
            struct Wrapper<'a>(&'a RubyClass, &'a str);
            impl fmt::Display for Wrapper<'_> {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    fmt_ruby_class(self.0, self.1, f)
                }
            }
            write!(fmt_buf, "{}", Wrapper(class, &inner)).expect("writing to String never fails");
            out.push_str(&fmt_buf);
            writeln!(out).expect("writing to String never fails");
        }
        if !self.functions.is_empty() {
            if self.module_function {
                writeln!(out, "{}module_function", inner).expect("writing to String never fails");
                writeln!(out).expect("writing to String never fails");
            }
            for method in &self.functions {
                let mut fmt_buf = std::string::String::new();
                struct Wrapper<'a>(&'a RubyMethod, &'a str);
                impl fmt::Display for Wrapper<'_> {
                    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                        fmt_ruby_method(self.0, self.1, f)
                    }
                }
                write!(fmt_buf, "{}", Wrapper(method, &inner))
                    .expect("writing to String never fails");
                out.push_str(&fmt_buf);
            }
        }
        writeln!(out, "{}end", indent).expect("writing to String never fails");
    }
}
/// Ruby feature flags
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct RubyFeatureFlags {
    pub use_data_class: bool,
    pub use_pattern_matching: bool,
    pub use_numbered_params: bool,
    pub use_hash_shorthand: bool,
}
/// Ruby code generation backend.
///
/// Compiles LCNF function declarations to Ruby methods and bundles
/// them into a `RubyModule`.
pub struct RubyBackend {
    /// Counter for fresh temporary variable names.
    pub(super) tmp_counter: u64,
    /// Mangle cache.
    pub(super) mangle_cache: std::collections::HashMap<std::string::String, std::string::String>,
}
impl RubyBackend {
    /// Create a new `RubyBackend`.
    pub fn new() -> Self {
        RubyBackend {
            tmp_counter: 0,
            mangle_cache: std::collections::HashMap::new(),
        }
    }
    /// Generate a fresh temporary variable name.
    pub(super) fn fresh_tmp(&mut self) -> std::string::String {
        let n = self.tmp_counter;
        self.tmp_counter += 1;
        format!("_t{}", n)
    }
    /// Mangle an LCNF name to a valid Ruby identifier (snake_case).
    pub fn mangle_name(&mut self, name: &str) -> std::string::String {
        if let Some(cached) = self.mangle_cache.get(name) {
            return cached.clone();
        }
        let result = ruby_mangle(name);
        self.mangle_cache.insert(name.to_string(), result.clone());
        result
    }
    /// Compile an LCNF function declaration to a `RubyMethod`.
    pub fn compile_decl(&mut self, decl: &LcnfFunDecl) -> Result<RubyMethod, std::string::String> {
        let method_name = self.mangle_name(&decl.name);
        let params: Vec<std::string::String> = decl
            .params
            .iter()
            .map(|p| {
                if p.name.is_empty() || p.name == "_" {
                    format!("_x{}", p.id.0)
                } else {
                    ruby_mangle(&p.name)
                }
            })
            .collect();
        let mut stmts: Vec<RubyStmt> = Vec::new();
        let result_expr = self.compile_expr(&decl.body, &mut stmts)?;
        let already_returns = matches!(stmts.last(), Some(RubyStmt::Return(_)));
        if !already_returns {
            stmts.push(RubyStmt::Return(result_expr));
        }
        Ok(RubyMethod {
            name: method_name,
            params: params.iter().map(|s| s.to_string()).collect(),
            body: stmts,
            visibility: RubyVisibility::Public,
        })
    }
    /// Compile a complete set of LCNF declarations to a Ruby source string.
    pub fn emit_module(decls: &[LcnfFunDecl]) -> Result<std::string::String, std::string::String> {
        let mut backend = RubyBackend::new();
        let mut module = RubyModule::new("OxiLean");
        module.module_function = true;
        let mut ctor_names: HashSet<std::string::String> = HashSet::new();
        for decl in decls {
            collect_ctor_names_from_expr(&decl.body, &mut ctor_names);
        }
        for ctor in &ctor_names {
            let class_name = ruby_const_name(ctor);
            let mut class = RubyClass::new(&class_name);
            class.superclass = Some("Data".to_string());
            module.constants.push((
                class_name.clone(),
                RubyExpr::MethodCall(
                    Box::new(RubyExpr::Var("Data".to_string())),
                    "define".to_string(),
                    vec![
                        RubyExpr::Lit(RubyLit::Symbol("tag".to_string())),
                        RubyExpr::Lit(RubyLit::Symbol("fields".to_string())),
                    ],
                ),
            ));
        }
        for decl in decls {
            let method = backend.compile_decl(decl)?;
            module.functions.push(method);
        }
        let mut source = RUBY_RUNTIME.to_string();
        source.push('\n');
        source.push_str(&module.emit());
        Ok(source)
    }
    /// Compile an LCNF expression to a Ruby expression, pushing any needed
    /// intermediate statements into `stmts`.
    pub(super) fn compile_expr(
        &mut self,
        expr: &LcnfExpr,
        stmts: &mut Vec<RubyStmt>,
    ) -> Result<RubyExpr, std::string::String> {
        match expr {
            LcnfExpr::Return(arg) => Ok(self.compile_arg(arg)),
            LcnfExpr::Unreachable => {
                let raise_call = RubyExpr::Call(
                    "raise".to_string(),
                    vec![
                        RubyExpr::Var("RuntimeError".to_string()),
                        RubyExpr::Lit(RubyLit::Str("OxiLean: unreachable".to_string())),
                    ],
                );
                stmts.push(RubyStmt::Expr(raise_call));
                Ok(RubyExpr::Lit(RubyLit::Nil))
            }
            LcnfExpr::TailCall(func, args) => {
                let callee = self.compile_arg(func);
                let rb_args: Vec<RubyExpr> = args.iter().map(|a| self.compile_arg(a)).collect();
                match callee {
                    RubyExpr::Var(name) => Ok(RubyExpr::Call(name, rb_args)),
                    other => Ok(RubyExpr::MethodCall(
                        Box::new(other),
                        "call".to_string(),
                        rb_args,
                    )),
                }
            }
            LcnfExpr::Let {
                id,
                name,
                ty: _,
                value,
                body,
            } => {
                let var_name = if name.is_empty() || name == "_" {
                    format!("_x{}", id.0)
                } else {
                    ruby_mangle(name)
                };
                let val_expr = self.compile_let_value(value)?;
                stmts.push(RubyStmt::Assign(var_name, val_expr));
                self.compile_expr(body, stmts)
            }
            LcnfExpr::Case {
                scrutinee,
                alts,
                default,
                ..
            } => {
                let scrutinee_expr = RubyExpr::Var(format!("_x{}", scrutinee.0));
                let tag_expr = RubyExpr::MethodCall(
                    Box::new(scrutinee_expr.clone()),
                    "tag".to_string(),
                    vec![],
                );
                let result_var = self.fresh_tmp();
                stmts.push(RubyStmt::Assign(
                    result_var.clone(),
                    RubyExpr::Lit(RubyLit::Nil),
                ));
                let mut when_branches: Vec<(RubyExpr, Vec<RubyStmt>)> = Vec::new();
                for alt in alts {
                    let mut branch_stmts: Vec<RubyStmt> = Vec::new();
                    for (field_idx, param) in alt.params.iter().enumerate() {
                        let field_var = if param.name.is_empty() || param.name == "_" {
                            format!("_x{}", param.id.0)
                        } else {
                            ruby_mangle(&param.name)
                        };
                        let field_access = RubyExpr::MethodCall(
                            Box::new(RubyExpr::MethodCall(
                                Box::new(scrutinee_expr.clone()),
                                "fields".to_string(),
                                vec![],
                            )),
                            "[]".to_string(),
                            vec![RubyExpr::Lit(RubyLit::Int(field_idx as i64))],
                        );
                        branch_stmts.push(RubyStmt::Assign(field_var, field_access));
                    }
                    let branch_result = self.compile_expr(&alt.body, &mut branch_stmts)?;
                    branch_stmts.push(RubyStmt::Assign(result_var.clone(), branch_result));
                    when_branches.push((
                        RubyExpr::Lit(RubyLit::Int(alt.ctor_tag as i64)),
                        branch_stmts,
                    ));
                }
                let mut default_stmts: Vec<RubyStmt> = Vec::new();
                if let Some(def) = default {
                    let def_result = self.compile_expr(def, &mut default_stmts)?;
                    default_stmts.push(RubyStmt::Assign(result_var.clone(), def_result));
                } else {
                    default_stmts.push(RubyStmt::Expr(RubyExpr::Call(
                        "raise".to_string(),
                        vec![
                            RubyExpr::Var("RuntimeError".to_string()),
                            RubyExpr::Lit(RubyLit::Str("OxiLean: unreachable".to_string())),
                        ],
                    )));
                }
                let mut all_stmts_flat: Vec<RubyStmt> = Vec::new();
                if when_branches.is_empty() {
                    for s in default_stmts {
                        all_stmts_flat.push(s);
                    }
                } else {
                    let (first_pat, first_body) = when_branches.remove(0);
                    let cond = RubyExpr::BinOp(
                        "==".to_string(),
                        Box::new(tag_expr.clone()),
                        Box::new(first_pat),
                    );
                    let elsif: Vec<(RubyExpr, Vec<RubyStmt>)> = when_branches
                        .into_iter()
                        .map(|(pat, body)| {
                            let c = RubyExpr::BinOp(
                                "==".to_string(),
                                Box::new(tag_expr.clone()),
                                Box::new(pat),
                            );
                            (c, body)
                        })
                        .collect();
                    all_stmts_flat.push(RubyStmt::If(cond, first_body, elsif, Some(default_stmts)));
                }
                for s in all_stmts_flat {
                    stmts.push(s);
                }
                Ok(RubyExpr::Var(result_var))
            }
        }
    }
    /// Compile an LCNF let-value to a Ruby expression.
    pub(super) fn compile_let_value(
        &mut self,
        value: &LcnfLetValue,
    ) -> Result<RubyExpr, std::string::String> {
        match value {
            LcnfLetValue::Lit(lit) => Ok(self.compile_lit(lit)),
            LcnfLetValue::Erased => Ok(RubyExpr::Lit(RubyLit::Nil)),
            LcnfLetValue::FVar(id) => Ok(RubyExpr::Var(format!("_x{}", id.0))),
            LcnfLetValue::App(func, args) => {
                let callee = self.compile_arg(func);
                let rb_args: Vec<RubyExpr> = args.iter().map(|a| self.compile_arg(a)).collect();
                match callee {
                    RubyExpr::Var(name) => Ok(RubyExpr::Call(name, rb_args)),
                    other => Ok(RubyExpr::MethodCall(
                        Box::new(other),
                        "call".to_string(),
                        rb_args,
                    )),
                }
            }
            LcnfLetValue::Proj(_name, idx, var) => {
                let base = RubyExpr::Var(format!("_x{}", var.0));
                Ok(RubyExpr::MethodCall(
                    Box::new(RubyExpr::MethodCall(
                        Box::new(base),
                        "fields".to_string(),
                        vec![],
                    )),
                    "[]".to_string(),
                    vec![RubyExpr::Lit(RubyLit::Int(*idx as i64))],
                ))
            }
            LcnfLetValue::Ctor(name, tag, args) => {
                let class_name = ruby_const_name(name);
                let rb_args: Vec<RubyExpr> = args.iter().map(|a| self.compile_arg(a)).collect();
                Ok(RubyExpr::MethodCall(
                    Box::new(RubyExpr::Var(class_name)),
                    "new".to_string(),
                    vec![RubyExpr::Hash(vec![
                        (
                            RubyExpr::Lit(RubyLit::Symbol("tag".to_string())),
                            RubyExpr::Lit(RubyLit::Int(*tag as i64)),
                        ),
                        (
                            RubyExpr::Lit(RubyLit::Symbol("fields".to_string())),
                            RubyExpr::Array(rb_args),
                        ),
                    ])],
                ))
            }
            LcnfLetValue::Reset(_var) => Ok(RubyExpr::Lit(RubyLit::Nil)),
            LcnfLetValue::Reuse(_slot, name, tag, args) => {
                let class_name = ruby_const_name(name);
                let rb_args: Vec<RubyExpr> = args.iter().map(|a| self.compile_arg(a)).collect();
                Ok(RubyExpr::MethodCall(
                    Box::new(RubyExpr::Var(class_name)),
                    "new".to_string(),
                    vec![RubyExpr::Hash(vec![
                        (
                            RubyExpr::Lit(RubyLit::Symbol("tag".to_string())),
                            RubyExpr::Lit(RubyLit::Int(*tag as i64)),
                        ),
                        (
                            RubyExpr::Lit(RubyLit::Symbol("fields".to_string())),
                            RubyExpr::Array(rb_args),
                        ),
                    ])],
                ))
            }
        }
    }
    /// Compile an LCNF argument to a Ruby expression.
    pub(super) fn compile_arg(&self, arg: &LcnfArg) -> RubyExpr {
        match arg {
            LcnfArg::Var(id) => RubyExpr::Var(format!("_x{}", id.0)),
            LcnfArg::Lit(lit) => self.compile_lit(lit),
            LcnfArg::Erased => RubyExpr::Lit(RubyLit::Nil),
            LcnfArg::Type(_) => RubyExpr::Lit(RubyLit::Nil),
        }
    }
    /// Compile an LCNF literal to a Ruby expression.
    pub(super) fn compile_lit(&self, lit: &LcnfLit) -> RubyExpr {
        match lit {
            LcnfLit::Nat(n) => RubyExpr::Lit(RubyLit::Int(*n as i64)),
            LcnfLit::Str(s) => RubyExpr::Lit(RubyLit::Str(s.clone())),
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct RubyPassStats {
    pub total_runs: u32,
    pub successful_runs: u32,
    pub total_changes: u64,
    pub time_ms: u64,
    pub iterations_used: u32,
}
impl RubyPassStats {
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
#[derive(Debug, Default)]
pub struct RubyDiagSink {
    pub diags: Vec<(RubyDiagLevel, String)>,
}
#[allow(dead_code)]
impl RubyDiagSink {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn push(&mut self, level: RubyDiagLevel, msg: &str) {
        self.diags.push((level, msg.to_string()));
    }
    pub fn has_errors(&self) -> bool {
        self.diags.iter().any(|(l, _)| *l == RubyDiagLevel::Error)
    }
}
