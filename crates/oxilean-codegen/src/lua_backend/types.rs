//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;
use std::collections::HashMap;

use super::functions::LUA_KEYWORDS;

use super::functions::*;
use std::collections::{HashSet, VecDeque};

/// A text buffer for building LuaExt output source code.
#[derive(Debug, Default)]
pub struct LuaExtSourceBuffer {
    pub(super) buf: String,
    pub(super) indent_level: usize,
    pub(super) indent_str: String,
}
impl LuaExtSourceBuffer {
    pub fn new() -> Self {
        LuaExtSourceBuffer {
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
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LuaDominatorTree {
    pub idom: Vec<Option<u32>>,
    pub dom_children: Vec<Vec<u32>>,
    pub dom_depth: Vec<u32>,
}
impl LuaDominatorTree {
    #[allow(dead_code)]
    pub fn new(size: usize) -> Self {
        LuaDominatorTree {
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
/// Emission statistics for LuaExt.
#[derive(Debug, Clone, Default)]
pub struct LuaExtEmitStats {
    pub bytes_emitted: usize,
    pub items_emitted: usize,
    pub errors: usize,
    pub warnings: usize,
    pub elapsed_ms: u64,
}
impl LuaExtEmitStats {
    pub fn new() -> Self {
        LuaExtEmitStats::default()
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
pub struct LuaPassConfig {
    pub phase: LuaPassPhase,
    pub enabled: bool,
    pub max_iterations: u32,
    pub debug_output: bool,
    pub pass_name: String,
}
impl LuaPassConfig {
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>, phase: LuaPassPhase) -> Self {
        LuaPassConfig {
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
/// A version tag for LuaExt output artifacts.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LuaExtVersion {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
    pub pre: Option<String>,
}
impl LuaExtVersion {
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        LuaExtVersion {
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
    pub fn is_compatible_with(&self, other: &LuaExtVersion) -> bool {
        self.major == other.major && self.minor >= other.minor
    }
}
/// Lua type representation (runtime tags + structural hints).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LuaType {
    /// `nil`
    Nil,
    /// `boolean`
    Boolean,
    /// `number` — integer when `is_int` is true, float otherwise
    Number(bool),
    /// `string`
    String,
    /// `table`
    Table,
    /// `function`
    Function,
    /// `userdata`
    Userdata,
    /// `thread` (coroutine)
    Thread,
    /// Named/custom type
    Custom(std::string::String),
}
/// A named Lua function definition.
#[derive(Debug, Clone, PartialEq)]
pub struct LuaFunction {
    /// Function name (None for anonymous)
    pub name: Option<std::string::String>,
    /// Parameter names
    pub params: Vec<std::string::String>,
    /// Whether the function accepts varargs (`...`)
    pub vararg: bool,
    /// Function body statements
    pub body: Vec<LuaStmt>,
    /// Whether the function is `local`
    pub is_local: bool,
    /// Whether the function is a method (uses `:` syntax)
    pub is_method: bool,
}
impl LuaFunction {
    /// Create a new named function.
    pub fn new(
        name: impl Into<std::string::String>,
        params: Vec<std::string::String>,
        body: Vec<LuaStmt>,
    ) -> Self {
        LuaFunction {
            name: Some(name.into()),
            params,
            vararg: false,
            body,
            is_local: false,
            is_method: false,
        }
    }
    /// Create a new local function.
    pub fn new_local(
        name: impl Into<std::string::String>,
        params: Vec<std::string::String>,
        body: Vec<LuaStmt>,
    ) -> Self {
        LuaFunction {
            name: Some(name.into()),
            params,
            vararg: false,
            body,
            is_local: true,
            is_method: false,
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum LuaPassPhase {
    Analysis,
    Transformation,
    Verification,
    Cleanup,
}
impl LuaPassPhase {
    #[allow(dead_code)]
    pub fn name(&self) -> &str {
        match self {
            LuaPassPhase::Analysis => "analysis",
            LuaPassPhase::Transformation => "transformation",
            LuaPassPhase::Verification => "verification",
            LuaPassPhase::Cleanup => "cleanup",
        }
    }
    #[allow(dead_code)]
    pub fn is_modifying(&self) -> bool {
        matches!(self, LuaPassPhase::Transformation | LuaPassPhase::Cleanup)
    }
}
/// Worklist for LuaExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LuaExtWorklist {
    pub(super) items: std::collections::VecDeque<usize>,
    pub(super) present: Vec<bool>,
}
impl LuaExtWorklist {
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
/// Statistics for LuaExt passes.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct LuaExtPassStats {
    pub iterations: usize,
    pub changed: bool,
    pub nodes_visited: usize,
    pub nodes_modified: usize,
    pub time_ms: u64,
    pub memory_bytes: usize,
    pub errors: usize,
}
impl LuaExtPassStats {
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
    pub fn merge(&mut self, o: &LuaExtPassStats) {
        self.iterations += o.iterations;
        self.changed |= o.changed;
        self.nodes_visited += o.nodes_visited;
        self.nodes_modified += o.nodes_modified;
        self.time_ms += o.time_ms;
        self.memory_bytes = self.memory_bytes.max(o.memory_bytes);
        self.errors += o.errors;
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LuaDepGraph {
    pub(super) nodes: Vec<u32>,
    pub(super) edges: Vec<(u32, u32)>,
}
impl LuaDepGraph {
    #[allow(dead_code)]
    pub fn new() -> Self {
        LuaDepGraph {
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
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct LuaPassStats {
    pub total_runs: u32,
    pub successful_runs: u32,
    pub total_changes: u64,
    pub time_ms: u64,
    pub iterations_used: u32,
}
impl LuaPassStats {
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
/// Lua statement AST node.
#[derive(Debug, Clone, PartialEq)]
pub enum LuaStmt {
    /// Assignment: `targets = values`
    Assign {
        targets: Vec<LuaExpr>,
        values: Vec<LuaExpr>,
    },
    /// Local variable declaration: `local names = values`
    LocalAssign {
        names: Vec<std::string::String>,
        attribs: Vec<Option<std::string::String>>,
        values: Vec<LuaExpr>,
    },
    /// Do block: `do ... end`
    Do(Vec<LuaStmt>),
    /// While loop: `while cond do ... end`
    While { cond: LuaExpr, body: Vec<LuaStmt> },
    /// Repeat-until loop: `repeat ... until cond`
    Repeat { body: Vec<LuaStmt>, cond: LuaExpr },
    /// If-elseif-else: `if cond then ... [elseif ...] [else ...] end`
    If {
        cond: LuaExpr,
        then_body: Vec<LuaStmt>,
        elseif_clauses: Vec<(LuaExpr, Vec<LuaStmt>)>,
        else_body: Option<Vec<LuaStmt>>,
    },
    /// Numeric for: `for var = start, limit[, step] do ... end`
    For {
        var: std::string::String,
        start: LuaExpr,
        limit: LuaExpr,
        step: Option<LuaExpr>,
        body: Vec<LuaStmt>,
    },
    /// Generic for: `for names in exprs do ... end`
    ForIn {
        names: Vec<std::string::String>,
        exprs: Vec<LuaExpr>,
        body: Vec<LuaStmt>,
    },
    /// Named function definition: `function name(params) ... end`
    Function(LuaFunction),
    /// Local function definition: `local function name(params) ... end`
    Local(LuaFunction),
    /// Return statement: `return [exprs]`
    Return(Vec<LuaExpr>),
    /// Break statement: `break`
    Break,
    /// Goto statement: `goto label`
    Goto(std::string::String),
    /// Label statement: `::label::`
    Label(std::string::String),
    /// Expression statement (function call): `expr`
    Call(LuaExpr),
}
/// A fixed-capacity ring buffer of strings (for recent-event logging in LuaExt).
#[derive(Debug)]
pub struct LuaExtEventLog {
    pub(super) entries: std::collections::VecDeque<String>,
    pub(super) capacity: usize,
}
impl LuaExtEventLog {
    pub fn new(capacity: usize) -> Self {
        LuaExtEventLog {
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
/// Liveness analysis for LuaExt.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct LuaExtLiveness {
    pub live_in: Vec<Vec<usize>>,
    pub live_out: Vec<Vec<usize>>,
    pub defs: Vec<Vec<usize>>,
    pub uses: Vec<Vec<usize>>,
}
impl LuaExtLiveness {
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
/// Pipeline profiler for LuaExt.
#[derive(Debug, Default)]
pub struct LuaExtProfiler {
    pub(super) timings: Vec<LuaExtPassTiming>,
}
impl LuaExtProfiler {
    pub fn new() -> Self {
        LuaExtProfiler::default()
    }
    pub fn record(&mut self, t: LuaExtPassTiming) {
        self.timings.push(t);
    }
    pub fn total_elapsed_us(&self) -> u64 {
        self.timings.iter().map(|t| t.elapsed_us).sum()
    }
    pub fn slowest_pass(&self) -> Option<&LuaExtPassTiming> {
        self.timings.iter().max_by_key(|t| t.elapsed_us)
    }
    pub fn num_passes(&self) -> usize {
        self.timings.len()
    }
    pub fn profitable_passes(&self) -> Vec<&LuaExtPassTiming> {
        self.timings.iter().filter(|t| t.is_profitable()).collect()
    }
}
/// Collects LuaExt diagnostics.
#[derive(Debug, Default)]
pub struct LuaExtDiagCollector {
    pub(super) msgs: Vec<LuaExtDiagMsg>,
}
impl LuaExtDiagCollector {
    pub fn new() -> Self {
        LuaExtDiagCollector::default()
    }
    pub fn emit(&mut self, d: LuaExtDiagMsg) {
        self.msgs.push(d);
    }
    pub fn has_errors(&self) -> bool {
        self.msgs
            .iter()
            .any(|d| d.severity == LuaExtDiagSeverity::Error)
    }
    pub fn errors(&self) -> Vec<&LuaExtDiagMsg> {
        self.msgs
            .iter()
            .filter(|d| d.severity == LuaExtDiagSeverity::Error)
            .collect()
    }
    pub fn warnings(&self) -> Vec<&LuaExtDiagMsg> {
        self.msgs
            .iter()
            .filter(|d| d.severity == LuaExtDiagSeverity::Warning)
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
/// Heuristic freshness key for LuaExt incremental compilation.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LuaExtIncrKey {
    pub content_hash: u64,
    pub config_hash: u64,
}
impl LuaExtIncrKey {
    pub fn new(content: u64, config: u64) -> Self {
        LuaExtIncrKey {
            content_hash: content,
            config_hash: config,
        }
    }
    pub fn combined_hash(&self) -> u64 {
        self.content_hash.wrapping_mul(0x9e3779b97f4a7c15) ^ self.config_hash
    }
    pub fn matches(&self, other: &LuaExtIncrKey) -> bool {
        self.content_hash == other.content_hash && self.config_hash == other.config_hash
    }
}
/// Lua code generation backend.
pub struct LuaBackend {
    /// Counter for fresh variable names.
    pub(super) fresh_counter: u64,
    /// Name mangling cache.
    pub(super) name_cache: HashMap<std::string::String, std::string::String>,
}
impl LuaBackend {
    /// Create a new `LuaBackend`.
    pub fn new() -> Self {
        LuaBackend {
            fresh_counter: 0,
            name_cache: HashMap::new(),
        }
    }
    /// Generate a fresh variable name.
    pub fn fresh_var(&mut self) -> std::string::String {
        let n = self.fresh_counter;
        self.fresh_counter += 1;
        format!("_t{}", n)
    }
    /// Mangle an OxiLean name to a valid Lua identifier.
    pub fn mangle_name(&mut self, name: &str) -> std::string::String {
        if let Some(cached) = self.name_cache.get(name) {
            return cached.clone();
        }
        if name.is_empty() {
            return "_anon".to_string();
        }
        let mangled: std::string::String = name
            .chars()
            .map(|c| match c {
                '.' | ':' => '_',
                '\'' => '_',
                c if c.is_alphanumeric() || c == '_' => c,
                _ => '_',
            })
            .collect();
        let mangled = if LUA_KEYWORDS.contains(&mangled.as_str())
            || mangled.starts_with(|c: char| c.is_ascii_digit())
        {
            format!("_{}", mangled)
        } else {
            mangled
        };
        self.name_cache.insert(name.to_string(), mangled.clone());
        mangled
    }
    /// Map an LCNF type to a Lua type hint.
    pub fn lcnf_to_lua_type(ty: &LcnfType) -> LuaType {
        match ty {
            LcnfType::Nat => LuaType::Number(true),
            LcnfType::LcnfString => LuaType::String,
            LcnfType::Unit | LcnfType::Erased | LcnfType::Irrelevant => LuaType::Nil,
            LcnfType::Object => LuaType::Table,
            LcnfType::Var(name) => LuaType::Custom(name.clone()),
            LcnfType::Fun(..) => LuaType::Function,
            LcnfType::Ctor(name, _) => LuaType::Custom(name.clone()),
        }
    }
    /// Compile an LCNF literal to a Lua expression.
    pub fn compile_lit(lit: &LcnfLit) -> LuaExpr {
        match lit {
            LcnfLit::Nat(n) => LuaExpr::Int(*n as i64),
            LcnfLit::Str(s) => LuaExpr::Str(s.clone()),
        }
    }
    /// Compile an LCNF literal value to a Lua expression.
    pub fn compile_let_value(&mut self, value: &LcnfLetValue) -> LuaExpr {
        match value {
            LcnfLetValue::App(func, args) => {
                let func_expr = self.compile_arg(func);
                let lua_args: Vec<_> = args.iter().map(|a| self.compile_arg(a)).collect();
                LuaExpr::Call {
                    func: Box::new(func_expr),
                    args: lua_args,
                }
            }
            LcnfLetValue::Ctor(ctor_name, _tag, fields) => {
                let mut all_fields = vec![LuaTableField::NamedField(
                    "tag".to_string(),
                    LuaExpr::Str(ctor_name.clone()),
                )];
                for f in fields {
                    all_fields.push(LuaTableField::ArrayItem(self.compile_arg(f)));
                }
                LuaExpr::TableConstructor(all_fields)
            }
            LcnfLetValue::Proj(_name, index, var) => {
                let val_expr = LuaExpr::Var(var.to_string());
                LuaExpr::IndexAccess {
                    table: Box::new(val_expr),
                    key: Box::new(LuaExpr::Int(*index as i64 + 2)),
                }
            }
            LcnfLetValue::Lit(lit) => Self::compile_lit(lit),
            LcnfLetValue::Erased => LuaExpr::Nil,
            LcnfLetValue::FVar(id) => LuaExpr::Var(id.to_string()),
            LcnfLetValue::Reset(_) => LuaExpr::Nil,
            LcnfLetValue::Reuse(_slot, ctor_name, _tag, fields) => {
                let mut all_fields = vec![LuaTableField::NamedField(
                    "tag".to_string(),
                    LuaExpr::Str(ctor_name.clone()),
                )];
                for f in fields {
                    all_fields.push(LuaTableField::ArrayItem(self.compile_arg(f)));
                }
                LuaExpr::TableConstructor(all_fields)
            }
        }
    }
    /// Compile an LCNF argument to a Lua expression.
    pub fn compile_arg(&mut self, arg: &LcnfArg) -> LuaExpr {
        match arg {
            LcnfArg::Var(id) => LuaExpr::Var(id.to_string()),
            LcnfArg::Lit(lit) => Self::compile_lit(lit),
            LcnfArg::Erased => LuaExpr::Nil,
            LcnfArg::Type(_) => LuaExpr::Nil,
        }
    }
    /// Compile an LCNF expression into a list of Lua statements,
    /// returning the result expression.
    pub fn compile_expr(&mut self, expr: &LcnfExpr, stmts: &mut Vec<LuaStmt>) -> LuaExpr {
        match expr {
            LcnfExpr::Return(arg) => self.compile_arg(arg),
            LcnfExpr::Let {
                id,
                ty: _,
                value,
                body,
                ..
            } => {
                let val_expr = self.compile_let_value(value);
                stmts.push(LuaStmt::LocalAssign {
                    names: vec![id.to_string()],
                    attribs: vec![None],
                    values: vec![val_expr],
                });
                self.compile_expr(body, stmts)
            }
            LcnfExpr::TailCall(func, args) => {
                let func_expr = self.compile_arg(func);
                let lua_args: Vec<_> = args.iter().map(|a| self.compile_arg(a)).collect();
                LuaExpr::Call {
                    func: Box::new(func_expr),
                    args: lua_args,
                }
            }
            LcnfExpr::Case {
                scrutinee,
                alts,
                default,
                ..
            } => {
                let scrut_expr = LuaExpr::Var(scrutinee.to_string());
                let result_var = self.fresh_var();
                stmts.push(LuaStmt::LocalAssign {
                    names: vec![result_var.clone()],
                    attribs: vec![None],
                    values: vec![],
                });
                let tag_expr = LuaExpr::FieldAccess {
                    table: Box::new(scrut_expr.clone()),
                    field: "tag".to_string(),
                };
                let mut if_cond: Option<LuaExpr> = None;
                let mut then_stmts: Vec<LuaStmt> = Vec::new();
                let mut elseif_clauses: Vec<(LuaExpr, Vec<LuaStmt>)> = Vec::new();
                for (idx, alt) in alts.iter().enumerate() {
                    let mut case_stmts: Vec<LuaStmt> = Vec::new();
                    for (field_idx, param) in alt.params.iter().enumerate() {
                        let field_access = LuaExpr::IndexAccess {
                            table: Box::new(scrut_expr.clone()),
                            key: Box::new(LuaExpr::Int(field_idx as i64 + 2)),
                        };
                        case_stmts.push(LuaStmt::LocalAssign {
                            names: vec![param.id.to_string()],
                            attribs: vec![None],
                            values: vec![field_access],
                        });
                    }
                    let case_result = self.compile_expr(&alt.body, &mut case_stmts);
                    case_stmts.push(LuaStmt::Assign {
                        targets: vec![LuaExpr::Var(result_var.clone())],
                        values: vec![case_result],
                    });
                    let cond = LuaExpr::BinOp {
                        op: "==".to_string(),
                        lhs: Box::new(tag_expr.clone()),
                        rhs: Box::new(LuaExpr::Str(alt.ctor_name.clone())),
                    };
                    if idx == 0 {
                        if_cond = Some(cond);
                        then_stmts = case_stmts;
                    } else {
                        elseif_clauses.push((cond, case_stmts));
                    }
                }
                let else_body = if let Some(def) = default {
                    let mut def_stmts: Vec<LuaStmt> = Vec::new();
                    let def_result = self.compile_expr(def, &mut def_stmts);
                    def_stmts.push(LuaStmt::Assign {
                        targets: vec![LuaExpr::Var(result_var.clone())],
                        values: vec![def_result],
                    });
                    Some(def_stmts)
                } else {
                    None
                };
                if let Some(cond) = if_cond {
                    stmts.push(LuaStmt::If {
                        cond,
                        then_body: then_stmts,
                        elseif_clauses,
                        else_body,
                    });
                }
                LuaExpr::Var(result_var)
            }
            LcnfExpr::Unreachable => LuaExpr::Call {
                func: Box::new(LuaExpr::Var("error".to_string())),
                args: vec![LuaExpr::Str("unreachable".to_string())],
            },
        }
    }
    /// Compile an LCNF function declaration to a `LuaFunction`.
    pub fn compile_decl(&mut self, decl: &LcnfFunDecl) -> Result<LuaFunction, std::string::String> {
        let lua_name = self.mangle_name(&decl.name);
        let params: Vec<_> = decl.params.iter().map(|p| p.id.to_string()).collect();
        let mut body_stmts: Vec<LuaStmt> = Vec::new();
        let result_expr = self.compile_expr(&decl.body, &mut body_stmts);
        body_stmts.push(LuaStmt::Return(vec![result_expr]));
        Ok(LuaFunction {
            name: Some(lua_name),
            params,
            vararg: false,
            body: body_stmts,
            is_local: false,
            is_method: false,
        })
    }
    /// Compile a list of declarations and emit a `LuaModule`.
    pub fn emit_module(&mut self, decls: &[LcnfFunDecl]) -> LuaModule {
        let mut module = LuaModule::new();
        for decl in decls {
            if let Ok(func) = self.compile_decl(decl) {
                module.functions.push(func);
            }
        }
        module
    }
}
/// A generic key-value configuration store for LuaExt.
#[derive(Debug, Clone, Default)]
pub struct LuaExtConfig {
    pub(super) entries: std::collections::HashMap<String, String>,
}
impl LuaExtConfig {
    pub fn new() -> Self {
        LuaExtConfig::default()
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
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LuaCacheEntry {
    pub key: String,
    pub data: Vec<u8>,
    pub timestamp: u64,
    pub valid: bool,
}
/// A complete Lua module / script.
#[derive(Debug, Clone, PartialEq)]
pub struct LuaModule {
    /// `require` imports: `local modname = require("modname")`
    pub requires: Vec<(std::string::String, std::string::String)>,
    /// Top-level local declarations (before functions)
    pub local_decls: Vec<LuaStmt>,
    /// Top-level function definitions
    pub functions: Vec<LuaFunction>,
    /// Main block statements (executed at top level)
    pub main_block: Vec<LuaStmt>,
}
impl LuaModule {
    /// Create an empty Lua module.
    pub fn new() -> Self {
        LuaModule {
            requires: Vec::new(),
            local_decls: Vec::new(),
            functions: Vec::new(),
            main_block: Vec::new(),
        }
    }
    /// Emit valid Lua 5.4 source code for this module.
    pub fn emit(&self) -> std::string::String {
        let mut out = std::string::String::new();
        for (alias, path) in &self.requires {
            out.push_str(&format!("local {} = require(\"{}\")\n", alias, path));
        }
        if !self.requires.is_empty() {
            out.push('\n');
        }
        for decl in &self.local_decls {
            out.push_str(&format!("{}\n", emit_stmt(decl, 0)));
        }
        if !self.local_decls.is_empty() {
            out.push('\n');
        }
        for func in &self.functions {
            out.push_str(&format!("{}\n\n", emit_function(func, 0, false)));
        }
        for stmt in &self.main_block {
            out.push_str(&format!("{}\n", emit_stmt(stmt, 0)));
        }
        out
    }
}
/// Tracks declared names for LuaExt scope analysis.
#[derive(Debug, Default)]
pub struct LuaExtNameScope {
    pub(super) declared: std::collections::HashSet<String>,
    pub(super) depth: usize,
    pub(super) parent: Option<Box<LuaExtNameScope>>,
}
impl LuaExtNameScope {
    pub fn new() -> Self {
        LuaExtNameScope::default()
    }
    pub fn declare(&mut self, name: impl Into<String>) -> bool {
        self.declared.insert(name.into())
    }
    pub fn is_declared(&self, name: &str) -> bool {
        self.declared.contains(name)
    }
    pub fn push_scope(self) -> Self {
        LuaExtNameScope {
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
/// A Lua class emulated via metatables.
#[derive(Debug, Clone, PartialEq)]
pub struct LuaClass {
    /// Class name
    pub name: std::string::String,
    /// Methods (not including `new`)
    pub methods: Vec<LuaFunction>,
    /// __index implementation (default: self-reference)
    pub index: Option<LuaExpr>,
    /// __newindex implementation
    pub newindex: Option<LuaExpr>,
    /// __tostring implementation body
    pub tostring_body: Option<Vec<LuaStmt>>,
}
impl LuaClass {
    /// Create a new class with the given name.
    pub fn new(name: impl Into<std::string::String>) -> Self {
        LuaClass {
            name: name.into(),
            methods: Vec::new(),
            index: None,
            newindex: None,
            tostring_body: None,
        }
    }
    /// Emit Lua source for this class definition.
    pub fn emit(&self) -> std::string::String {
        let mut out = std::string::String::new();
        let n = &self.name;
        out.push_str(&format!("{} = {{}}\n", n));
        let index_str = match &self.index {
            Some(expr) => expr.to_string(),
            None => n.clone(),
        };
        out.push_str(&format!("{}.__index = {}\n", n, index_str));
        if let Some(ts_body) = &self.tostring_body {
            out.push_str(&format!(
                "{}.__tostring = function(self)\n{}\nend\n",
                n,
                emit_stmts(ts_body, 1)
            ));
        }
        if let Some(ni) = &self.newindex {
            out.push_str(&format!("{}.__newindex = {}\n", n, ni));
        }
        out.push_str(
            &format!(
                "function {}:new(o)\n  o = o or {{}}\n  setmetatable(o, self)\n  self.__index = self\n  return o\nend\n",
                n
            ),
        );
        for method in &self.methods {
            let mut m = method.clone();
            m.is_method = true;
            if let Some(ref mname) = m.name.clone() {
                m.name = Some(format!("{}.{}", n, mname));
            }
            out.push_str(&format!("{}\n", emit_function(&m, 0, false)));
        }
        out
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LuaWorklist {
    pub(super) items: std::collections::VecDeque<u32>,
    pub(super) in_worklist: std::collections::HashSet<u32>,
}
impl LuaWorklist {
    #[allow(dead_code)]
    pub fn new() -> Self {
        LuaWorklist {
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
/// Pass registry for LuaExt.
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct LuaExtPassRegistry {
    pub(super) configs: Vec<LuaExtPassConfig>,
    pub(super) stats: Vec<LuaExtPassStats>,
}
impl LuaExtPassRegistry {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    #[allow(dead_code)]
    pub fn register(&mut self, c: LuaExtPassConfig) {
        self.stats.push(LuaExtPassStats::new());
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
    pub fn get(&self, i: usize) -> Option<&LuaExtPassConfig> {
        self.configs.get(i)
    }
    #[allow(dead_code)]
    pub fn get_stats(&self, i: usize) -> Option<&LuaExtPassStats> {
        self.stats.get(i)
    }
    #[allow(dead_code)]
    pub fn enabled_passes(&self) -> Vec<&LuaExtPassConfig> {
        self.configs.iter().filter(|c| c.enabled).collect()
    }
    #[allow(dead_code)]
    pub fn passes_in_phase(&self, ph: &LuaExtPassPhase) -> Vec<&LuaExtPassConfig> {
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
/// A feature flag set for LuaExt capabilities.
#[derive(Debug, Clone, Default)]
pub struct LuaExtFeatures {
    pub(super) flags: std::collections::HashSet<String>,
}
impl LuaExtFeatures {
    pub fn new() -> Self {
        LuaExtFeatures::default()
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
    pub fn union(&self, other: &LuaExtFeatures) -> LuaExtFeatures {
        LuaExtFeatures {
            flags: self.flags.union(&other.flags).cloned().collect(),
        }
    }
    pub fn intersection(&self, other: &LuaExtFeatures) -> LuaExtFeatures {
        LuaExtFeatures {
            flags: self.flags.intersection(&other.flags).cloned().collect(),
        }
    }
}
/// Lua expression AST node.
#[derive(Debug, Clone, PartialEq)]
pub enum LuaExpr {
    /// `nil`
    Nil,
    /// `true`
    True,
    /// `false`
    False,
    /// Integer literal: `42`
    Int(i64),
    /// Float literal: `3.14`
    Float(f64),
    /// String literal: `"hello"`
    Str(std::string::String),
    /// Variable reference: `x`
    Var(std::string::String),
    /// Binary operation: `lhs op rhs`
    BinOp {
        op: std::string::String,
        lhs: Box<LuaExpr>,
        rhs: Box<LuaExpr>,
    },
    /// Unary operation: `op operand`
    UnaryOp {
        op: std::string::String,
        operand: Box<LuaExpr>,
    },
    /// Function call: `func(args)`
    Call {
        func: Box<LuaExpr>,
        args: Vec<LuaExpr>,
    },
    /// Method call: `obj:method(args)`
    MethodCall {
        obj: Box<LuaExpr>,
        method: std::string::String,
        args: Vec<LuaExpr>,
    },
    /// Table constructor: `{field, ...}`
    TableConstructor(Vec<LuaTableField>),
    /// Index access: `table[key]`
    IndexAccess {
        table: Box<LuaExpr>,
        key: Box<LuaExpr>,
    },
    /// Field access: `table.field`
    FieldAccess {
        table: Box<LuaExpr>,
        field: std::string::String,
    },
    /// Lambda (anonymous function): `function(params) body end`
    Lambda {
        params: Vec<std::string::String>,
        vararg: bool,
        body: Vec<LuaStmt>,
    },
    /// Vararg expression: `...`
    Ellipsis,
}
/// Configuration for LuaExt passes.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LuaExtPassConfig {
    pub name: String,
    pub phase: LuaExtPassPhase,
    pub enabled: bool,
    pub max_iterations: usize,
    pub debug: u32,
    pub timeout_ms: Option<u64>,
}
impl LuaExtPassConfig {
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            phase: LuaExtPassPhase::Middle,
            enabled: true,
            max_iterations: 100,
            debug: 0,
            timeout_ms: None,
        }
    }
    #[allow(dead_code)]
    pub fn with_phase(mut self, phase: LuaExtPassPhase) -> Self {
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
#[derive(Debug, Clone)]
pub struct LuaLivenessInfo {
    pub live_in: Vec<std::collections::HashSet<u32>>,
    pub live_out: Vec<std::collections::HashSet<u32>>,
    pub defs: Vec<std::collections::HashSet<u32>>,
    pub uses: Vec<std::collections::HashSet<u32>>,
}
impl LuaLivenessInfo {
    #[allow(dead_code)]
    pub fn new(block_count: usize) -> Self {
        LuaLivenessInfo {
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
/// A monotonically increasing ID generator for LuaExt.
#[derive(Debug, Default)]
pub struct LuaExtIdGen {
    pub(super) next: u32,
}
impl LuaExtIdGen {
    pub fn new() -> Self {
        LuaExtIdGen::default()
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
pub struct LuaPassRegistry {
    pub(super) configs: Vec<LuaPassConfig>,
    pub(super) stats: std::collections::HashMap<String, LuaPassStats>,
}
impl LuaPassRegistry {
    #[allow(dead_code)]
    pub fn new() -> Self {
        LuaPassRegistry {
            configs: Vec::new(),
            stats: std::collections::HashMap::new(),
        }
    }
    #[allow(dead_code)]
    pub fn register(&mut self, config: LuaPassConfig) {
        self.stats
            .insert(config.pass_name.clone(), LuaPassStats::new());
        self.configs.push(config);
    }
    #[allow(dead_code)]
    pub fn enabled_passes(&self) -> Vec<&LuaPassConfig> {
        self.configs.iter().filter(|c| c.enabled).collect()
    }
    #[allow(dead_code)]
    pub fn get_stats(&self, name: &str) -> Option<&LuaPassStats> {
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
/// Constant folding helper for LuaExt.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct LuaExtConstFolder {
    pub(super) folds: usize,
    pub(super) failures: usize,
    pub(super) enabled: bool,
}
impl LuaExtConstFolder {
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
/// Analysis cache for LuaExt.
#[allow(dead_code)]
#[derive(Debug)]
pub struct LuaExtCache {
    pub(super) entries: Vec<(u64, Vec<u8>, bool, u32)>,
    pub(super) cap: usize,
    pub(super) total_hits: u64,
    pub(super) total_misses: u64,
}
impl LuaExtCache {
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
/// Pass-timing record for LuaExt profiler.
#[derive(Debug, Clone)]
pub struct LuaExtPassTiming {
    pub pass_name: String,
    pub elapsed_us: u64,
    pub items_processed: usize,
    pub bytes_before: usize,
    pub bytes_after: usize,
}
impl LuaExtPassTiming {
    pub fn new(
        pass_name: impl Into<String>,
        elapsed_us: u64,
        items: usize,
        before: usize,
        after: usize,
    ) -> Self {
        LuaExtPassTiming {
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
/// Pass execution phase for LuaExt.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LuaExtPassPhase {
    Early,
    Middle,
    Late,
    Finalize,
}
impl LuaExtPassPhase {
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
/// A single field in a Lua table constructor.
#[derive(Debug, Clone, PartialEq)]
pub enum LuaTableField {
    /// Positional element: `expr` (auto-indexed)
    ArrayItem(LuaExpr),
    /// Named field: `key = expr`
    NamedField(std::string::String, LuaExpr),
    /// Computed-key field: `[key_expr] = expr`
    IndexedField(LuaExpr, LuaExpr),
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LuaAnalysisCache {
    pub(super) entries: std::collections::HashMap<String, LuaCacheEntry>,
    pub(super) max_size: usize,
    pub(super) hits: u64,
    pub(super) misses: u64,
}
impl LuaAnalysisCache {
    #[allow(dead_code)]
    pub fn new(max_size: usize) -> Self {
        LuaAnalysisCache {
            entries: std::collections::HashMap::new(),
            max_size,
            hits: 0,
            misses: 0,
        }
    }
    #[allow(dead_code)]
    pub fn get(&mut self, key: &str) -> Option<&LuaCacheEntry> {
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
            LuaCacheEntry {
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
pub struct LuaConstantFoldingHelper;
impl LuaConstantFoldingHelper {
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
/// Severity of a LuaExt diagnostic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LuaExtDiagSeverity {
    Note,
    Warning,
    Error,
}
/// A diagnostic message from a LuaExt pass.
#[derive(Debug, Clone)]
pub struct LuaExtDiagMsg {
    pub severity: LuaExtDiagSeverity,
    pub pass: String,
    pub message: String,
}
impl LuaExtDiagMsg {
    pub fn error(pass: impl Into<String>, msg: impl Into<String>) -> Self {
        LuaExtDiagMsg {
            severity: LuaExtDiagSeverity::Error,
            pass: pass.into(),
            message: msg.into(),
        }
    }
    pub fn warning(pass: impl Into<String>, msg: impl Into<String>) -> Self {
        LuaExtDiagMsg {
            severity: LuaExtDiagSeverity::Warning,
            pass: pass.into(),
            message: msg.into(),
        }
    }
    pub fn note(pass: impl Into<String>, msg: impl Into<String>) -> Self {
        LuaExtDiagMsg {
            severity: LuaExtDiagSeverity::Note,
            pass: pass.into(),
            message: msg.into(),
        }
    }
}
/// Dominator tree for LuaExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LuaExtDomTree {
    pub(super) idom: Vec<Option<usize>>,
    pub(super) children: Vec<Vec<usize>>,
    pub(super) depth: Vec<usize>,
}
impl LuaExtDomTree {
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
/// Dependency graph for LuaExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LuaExtDepGraph {
    pub(super) n: usize,
    pub(super) adj: Vec<Vec<usize>>,
    pub(super) rev: Vec<Vec<usize>>,
    pub(super) edge_count: usize,
}
impl LuaExtDepGraph {
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
