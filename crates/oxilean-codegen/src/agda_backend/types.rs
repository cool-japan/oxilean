//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use std::collections::{HashMap, HashSet, VecDeque};

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct AgdaAnalysisCache {
    pub(super) entries: std::collections::HashMap<String, AgdaCacheEntry>,
    pub(super) max_size: usize,
    pub(super) hits: u64,
    pub(super) misses: u64,
}
impl AgdaAnalysisCache {
    #[allow(dead_code)]
    pub fn new(max_size: usize) -> Self {
        AgdaAnalysisCache {
            entries: std::collections::HashMap::new(),
            max_size,
            hits: 0,
            misses: 0,
        }
    }
    #[allow(dead_code)]
    pub fn get(&mut self, key: &str) -> Option<&AgdaCacheEntry> {
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
            AgdaCacheEntry {
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
/// Top-level `record` declaration.
#[derive(Debug, Clone, PartialEq)]
pub struct AgdaRecord {
    /// Record name
    pub name: String,
    /// Parameters
    pub params: Vec<(String, AgdaExpr)>,
    /// Universe sort of the record
    pub universe: AgdaExpr,
    /// Optional constructor name (default: `mk<Name>`)
    pub constructor: Option<String>,
    /// Fields
    pub fields: Vec<AgdaField>,
    /// Optional copattern-style definitions
    pub copattern_defs: Vec<AgdaClause>,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct AgdaDepGraph {
    pub(super) nodes: Vec<u32>,
    pub(super) edges: Vec<(u32, u32)>,
}
impl AgdaDepGraph {
    #[allow(dead_code)]
    pub fn new() -> Self {
        AgdaDepGraph {
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
/// A single constructor in a `data` declaration.
#[derive(Debug, Clone, PartialEq)]
pub struct AgdaConstructor {
    /// Constructor name
    pub name: String,
    /// Constructor type (Agda uses GADT-style `ctor : Type`)
    pub ty: AgdaExpr,
}
/// Analysis cache for AgdaX2.
#[allow(dead_code)]
#[derive(Debug)]
pub struct AgdaX2Cache {
    pub(super) entries: Vec<(u64, Vec<u8>, bool, u32)>,
    pub(super) cap: usize,
    pub(super) total_hits: u64,
    pub(super) total_misses: u64,
}
impl AgdaX2Cache {
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
/// Dominator tree for AgdaX2.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct AgdaX2DomTree {
    pub(super) idom: Vec<Option<usize>>,
    pub(super) children: Vec<Vec<usize>>,
    pub(super) depth: Vec<usize>,
}
impl AgdaX2DomTree {
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
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct AgdaCacheEntry {
    pub key: String,
    pub data: Vec<u8>,
    pub timestamp: u64,
    pub valid: bool,
}
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct AgdaPassStats {
    pub total_runs: u32,
    pub successful_runs: u32,
    pub total_changes: u64,
    pub time_ms: u64,
    pub iterations_used: u32,
}
impl AgdaPassStats {
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
pub struct AgdaPassConfig {
    pub phase: AgdaPassPhase,
    pub enabled: bool,
    pub max_iterations: u32,
    pub debug_output: bool,
    pub pass_name: String,
}
impl AgdaPassConfig {
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>, phase: AgdaPassPhase) -> Self {
        AgdaPassConfig {
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
/// Top-level Agda 2 declaration.
#[derive(Debug, Clone, PartialEq)]
pub enum AgdaDecl {
    /// Type signature: `f : T`
    FuncType { name: String, ty: AgdaExpr },
    /// Function definition: one or more clauses
    FuncDef {
        name: String,
        clauses: Vec<AgdaClause>,
    },
    /// Data declaration
    DataDecl(AgdaData),
    /// Record declaration
    RecordDecl(AgdaRecord),
    /// Module declaration: `module M where ...`
    ModuleDecl {
        name: String,
        params: Vec<(String, AgdaExpr)>,
        body: Vec<AgdaDecl>,
    },
    /// `import Data.Nat`
    Import(String),
    /// `open Data.Nat` (or `open Data.Nat using (...)`)
    Open(String),
    /// `variable {A B : Set}`
    Variable(Vec<(String, AgdaExpr)>),
    /// `postulate name : T`
    Postulate(Vec<(String, AgdaExpr)>),
    /// `{-# BUILTIN ... #-}` pragma
    Pragma(String),
    /// Plain comment: `-- text`
    Comment(String),
    /// Raw verbatim Agda text (fallback)
    Raw(String),
}
impl AgdaDecl {
    /// Emit as an Agda source string at the given indentation level.
    pub fn emit(&self, indent: usize) -> String {
        let pad = "  ".repeat(indent);
        match self {
            AgdaDecl::FuncType { name, ty } => {
                format!("{}{} : {}", pad, name, ty.emit(indent))
            }
            AgdaDecl::FuncDef { name, clauses } => clauses
                .iter()
                .map(|c| c.emit_clause(name, indent))
                .collect::<Vec<_>>()
                .join("\n"),
            AgdaDecl::DataDecl(data) => {
                let ps = emit_agda_params(&data.params, indent);
                let ps_str = if ps.is_empty() {
                    String::new()
                } else {
                    format!(" {}", ps)
                };
                let mut out = format!(
                    "{}data {}{} : {} where\n",
                    pad,
                    data.name,
                    ps_str,
                    data.indices.emit(indent)
                );
                for ctor in &data.constructors {
                    out.push_str(&format!(
                        "{}  {} : {}\n",
                        pad,
                        ctor.name,
                        ctor.ty.emit(indent + 1)
                    ));
                }
                out
            }
            AgdaDecl::RecordDecl(rec) => {
                let ps = emit_agda_params(&rec.params, indent);
                let ps_str = if ps.is_empty() {
                    String::new()
                } else {
                    format!(" {}", ps)
                };
                let mut out = format!(
                    "{}record {}{} : {} where\n",
                    pad,
                    rec.name,
                    ps_str,
                    rec.universe.emit(indent)
                );
                if let Some(ctor) = &rec.constructor {
                    out.push_str(&format!("{}  constructor {}\n", pad, ctor));
                }
                out.push_str(&format!("{}  field\n", pad));
                for field in &rec.fields {
                    out.push_str(&format!(
                        "{}    {} : {}\n",
                        pad,
                        field.name,
                        field.ty.emit(indent + 2)
                    ));
                }
                for clause in &rec.copattern_defs {
                    out.push_str(&clause.emit_clause(&rec.name, indent + 1));
                    out.push('\n');
                }
                out
            }
            AgdaDecl::ModuleDecl { name, params, body } => {
                let ps = emit_agda_params(params, indent);
                let ps_str = if ps.is_empty() {
                    String::new()
                } else {
                    format!(" {}", ps)
                };
                let mut out = format!("{}module {}{} where\n", pad, name, ps_str);
                for decl in body {
                    out.push_str(&decl.emit(indent + 1));
                    out.push('\n');
                }
                out
            }
            AgdaDecl::Import(module) => format!("{}import {}", pad, module),
            AgdaDecl::Open(module) => format!("{}open {}", pad, module),
            AgdaDecl::Variable(vars) => {
                let vs: Vec<String> = vars
                    .iter()
                    .map(|(x, ty)| format!("{{{} : {}}}", x, ty.emit(indent)))
                    .collect();
                format!("{}variable {}", pad, vs.join(" "))
            }
            AgdaDecl::Postulate(sigs) => {
                let mut out = format!("{}postulate\n", pad);
                for (name, ty) in sigs {
                    out.push_str(&format!("{}  {} : {}\n", pad, name, ty.emit(indent + 1)));
                }
                out
            }
            AgdaDecl::Pragma(text) => format!("{{-# {} #-}}", text),
            AgdaDecl::Comment(text) => format!("{}-- {}", pad, text),
            AgdaDecl::Raw(s) => s.clone(),
        }
    }
}
/// Configuration for AgdaExt passes.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct AgdaExtPassConfig {
    pub name: String,
    pub phase: AgdaExtPassPhase,
    pub enabled: bool,
    pub max_iterations: usize,
    pub debug: u32,
    pub timeout_ms: Option<u64>,
}
impl AgdaExtPassConfig {
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            phase: AgdaExtPassPhase::Middle,
            enabled: true,
            max_iterations: 100,
            debug: 0,
            timeout_ms: None,
        }
    }
    #[allow(dead_code)]
    pub fn with_phase(mut self, phase: AgdaExtPassPhase) -> Self {
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
/// Statistics for AgdaX2 passes.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct AgdaX2PassStats {
    pub iterations: usize,
    pub changed: bool,
    pub nodes_visited: usize,
    pub nodes_modified: usize,
    pub time_ms: u64,
    pub memory_bytes: usize,
    pub errors: usize,
}
impl AgdaX2PassStats {
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
    pub fn merge(&mut self, o: &AgdaX2PassStats) {
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
#[derive(Debug, Clone, PartialEq)]
pub enum AgdaPassPhase {
    Analysis,
    Transformation,
    Verification,
    Cleanup,
}
impl AgdaPassPhase {
    #[allow(dead_code)]
    pub fn name(&self) -> &str {
        match self {
            AgdaPassPhase::Analysis => "analysis",
            AgdaPassPhase::Transformation => "transformation",
            AgdaPassPhase::Verification => "verification",
            AgdaPassPhase::Cleanup => "cleanup",
        }
    }
    #[allow(dead_code)]
    pub fn is_modifying(&self) -> bool {
        matches!(self, AgdaPassPhase::Transformation | AgdaPassPhase::Cleanup)
    }
}
/// A pattern in an Agda function definition clause.
#[derive(Debug, Clone, PartialEq)]
pub enum AgdaPattern {
    /// Named variable pattern: `n`
    Var(String),
    /// Constructor pattern: `(ctor p1 p2)` or `ctor` (nullary)
    Con(String, Vec<AgdaPattern>),
    /// Wildcard pattern: `_`
    Wildcard,
    /// Literal number: `0`, `42`
    Num(i64),
    /// Dot pattern (inaccessible): `.t`
    Dot(Box<AgdaPattern>),
    /// Absurd pattern: `()`
    Absurd,
    /// As-pattern: `n@p`
    As(String, Box<AgdaPattern>),
    /// Implicit argument pattern: `{p}`
    Implicit(Box<AgdaPattern>),
}
/// Worklist for AgdaExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct AgdaExtWorklist {
    pub(super) items: std::collections::VecDeque<usize>,
    pub(super) present: Vec<bool>,
}
impl AgdaExtWorklist {
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
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct AgdaLivenessInfo {
    pub live_in: Vec<std::collections::HashSet<u32>>,
    pub live_out: Vec<std::collections::HashSet<u32>>,
    pub defs: Vec<std::collections::HashSet<u32>>,
    pub uses: Vec<std::collections::HashSet<u32>>,
}
impl AgdaLivenessInfo {
    #[allow(dead_code)]
    pub fn new(block_count: usize) -> Self {
        AgdaLivenessInfo {
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
/// Pass execution phase for AgdaExt.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AgdaExtPassPhase {
    Early,
    Middle,
    Late,
    Finalize,
}
impl AgdaExtPassPhase {
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
/// Pass registry for AgdaExt.
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct AgdaExtPassRegistry {
    pub(super) configs: Vec<AgdaExtPassConfig>,
    pub(super) stats: Vec<AgdaExtPassStats>,
}
impl AgdaExtPassRegistry {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    #[allow(dead_code)]
    pub fn register(&mut self, c: AgdaExtPassConfig) {
        self.stats.push(AgdaExtPassStats::new());
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
    pub fn get(&self, i: usize) -> Option<&AgdaExtPassConfig> {
        self.configs.get(i)
    }
    #[allow(dead_code)]
    pub fn get_stats(&self, i: usize) -> Option<&AgdaExtPassStats> {
        self.stats.get(i)
    }
    #[allow(dead_code)]
    pub fn enabled_passes(&self) -> Vec<&AgdaExtPassConfig> {
        self.configs.iter().filter(|c| c.enabled).collect()
    }
    #[allow(dead_code)]
    pub fn passes_in_phase(&self, ph: &AgdaExtPassPhase) -> Vec<&AgdaExtPassConfig> {
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
/// Constant folding helper for AgdaExt.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct AgdaExtConstFolder {
    pub(super) folds: usize,
    pub(super) failures: usize,
    pub(super) enabled: bool,
}
impl AgdaExtConstFolder {
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
#[allow(dead_code)]
pub struct AgdaPassRegistry {
    pub(super) configs: Vec<AgdaPassConfig>,
    pub(super) stats: std::collections::HashMap<String, AgdaPassStats>,
}
impl AgdaPassRegistry {
    #[allow(dead_code)]
    pub fn new() -> Self {
        AgdaPassRegistry {
            configs: Vec::new(),
            stats: std::collections::HashMap::new(),
        }
    }
    #[allow(dead_code)]
    pub fn register(&mut self, config: AgdaPassConfig) {
        self.stats
            .insert(config.pass_name.clone(), AgdaPassStats::new());
        self.configs.push(config);
    }
    #[allow(dead_code)]
    pub fn enabled_passes(&self) -> Vec<&AgdaPassConfig> {
        self.configs.iter().filter(|c| c.enabled).collect()
    }
    #[allow(dead_code)]
    pub fn get_stats(&self, name: &str) -> Option<&AgdaPassStats> {
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
/// A single function definition clause.
/// `f p1 p2 = rhs (where decls...)`
#[derive(Debug, Clone, PartialEq)]
pub struct AgdaClause {
    /// Argument patterns
    pub patterns: Vec<AgdaPattern>,
    /// Right-hand side (`None` for absurd clauses)
    pub rhs: Option<AgdaExpr>,
    /// Optional `where` declarations
    pub where_decls: Vec<AgdaDecl>,
}
impl AgdaClause {
    /// Emit pattern list (space-separated).
    pub fn emit_patterns(&self) -> String {
        self.patterns
            .iter()
            .map(|p| p.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    }
    /// Emit a full clause line: `func_name patterns = rhs`
    pub fn emit_clause(&self, func_name: &str, indent: usize) -> String {
        let pad = "  ".repeat(indent);
        let pats = self.emit_patterns();
        let lhs = if pats.is_empty() {
            func_name.to_string()
        } else {
            format!("{} {}", func_name, pats)
        };
        match &self.rhs {
            None => format!("{}{}", pad, lhs),
            Some(rhs) => {
                let mut out = format!("{}{} = {}", pad, lhs, rhs.emit(indent));
                if !self.where_decls.is_empty() {
                    out.push_str(&format!("\n{}  where", pad));
                    for w in &self.where_decls {
                        for line in w.emit(indent + 2).lines() {
                            out.push_str(&format!("\n{}  {}", pad, line));
                        }
                    }
                }
                out
            }
        }
    }
}
/// Dependency graph for AgdaExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct AgdaExtDepGraph {
    pub(super) n: usize,
    pub(super) adj: Vec<Vec<usize>>,
    pub(super) rev: Vec<Vec<usize>>,
    pub(super) edge_count: usize,
}
impl AgdaExtDepGraph {
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
#[derive(Debug, Clone)]
pub struct AgdaDominatorTree {
    pub idom: Vec<Option<u32>>,
    pub dom_children: Vec<Vec<u32>>,
    pub dom_depth: Vec<u32>,
}
impl AgdaDominatorTree {
    #[allow(dead_code)]
    pub fn new(size: usize) -> Self {
        AgdaDominatorTree {
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
/// Dominator tree for AgdaExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct AgdaExtDomTree {
    pub(super) idom: Vec<Option<usize>>,
    pub(super) children: Vec<Vec<usize>>,
    pub(super) depth: Vec<usize>,
}
impl AgdaExtDomTree {
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
/// A single field in a `record` declaration.
#[derive(Debug, Clone, PartialEq)]
pub struct AgdaField {
    /// Field name
    pub name: String,
    /// Field type
    pub ty: AgdaExpr,
}
/// Agda 2 expression AST.
#[derive(Debug, Clone, PartialEq)]
pub enum AgdaExpr {
    /// Variable or qualified name: `n`, `List.map`, `Data.Nat.zero`
    Var(String),
    /// Function application: `f a` (left-associative)
    App(Box<AgdaExpr>, Box<AgdaExpr>),
    /// Lambda abstraction: `λ x → body` (uses `\` ASCII prefix in code)
    Lambda(String, Box<AgdaExpr>),
    /// Dependent Pi type: `(x : A) → B` or `{x : A} → B`
    /// `None` = non-dependent `A → B`
    Pi(Option<String>, Box<AgdaExpr>, Box<AgdaExpr>),
    /// Let binding: `let x = rhs in body`
    Let(String, Box<AgdaExpr>, Box<AgdaExpr>),
    /// With clause extension (for auxiliary matches)
    With(Box<AgdaExpr>, Vec<AgdaExpr>),
    /// Case expression (implemented via a helper lambda + with)
    Case(Box<AgdaExpr>, Vec<AgdaClause>),
    /// Universe: `Set`, `Set₁`, `Set n`
    Set(Option<u32>),
    /// `Prop` universe
    Prop,
    /// Typed hole: `{! !}` (interactive proof obligation)
    Hole,
    /// Anonymous/inferred: `_`
    Underscore,
    /// Integer literal: `0`, `42`
    Num(i64),
    /// String literal: `"hello"`
    Str(String),
    /// A qualified module expression: `Module.Name`
    Module(String),
    /// Implicit argument: `{e}`
    Implicit(Box<AgdaExpr>),
    /// Tuple / pair: `(a , b)`
    Tuple(Vec<AgdaExpr>),
    /// Record construction: `record { f = v ; g = w }`
    Record(Vec<(String, AgdaExpr)>),
    /// Type ascription: `(e : T)`
    Ascription(Box<AgdaExpr>, Box<AgdaExpr>),
    /// If-then-else: `if b then t else f`
    IfThenElse(Box<AgdaExpr>, Box<AgdaExpr>, Box<AgdaExpr>),
}
impl AgdaExpr {
    /// Emit the expression with the given indentation level.
    pub fn emit(&self, indent: usize) -> String {
        let pad = "  ".repeat(indent);
        match self {
            AgdaExpr::Var(x) => x.clone(),
            AgdaExpr::Num(n) => n.to_string(),
            AgdaExpr::Str(s) => format!("\"{}\"", escape_agda_string(s)),
            AgdaExpr::Hole => "{! !}".to_string(),
            AgdaExpr::Underscore => "_".to_string(),
            AgdaExpr::Prop => "Prop".to_string(),
            AgdaExpr::Module(m) => m.clone(),
            AgdaExpr::Set(None) => "Set".to_string(),
            AgdaExpr::Set(Some(0)) => "Set".to_string(),
            AgdaExpr::Set(Some(1)) => "Set₁".to_string(),
            AgdaExpr::Set(Some(2)) => "Set₂".to_string(),
            AgdaExpr::Set(Some(n)) => format!("Set{}", n),
            AgdaExpr::App(f, a) => {
                let fs = f.emit_func(indent);
                let as_ = a.emit_atom(indent);
                format!("{} {}", fs, as_)
            }
            AgdaExpr::Lambda(x, body) => format!("λ {} → {}", x, body.emit(indent)),
            AgdaExpr::Pi(None, dom, cod) => {
                format!("{} → {}", dom.emit_pi_dom(indent), cod.emit(indent))
            }
            AgdaExpr::Pi(Some(x), dom, cod) => {
                format!("({} : {}) → {}", x, dom.emit(indent), cod.emit(indent))
            }
            AgdaExpr::Let(x, rhs, body) => {
                format!(
                    "let {} = {}\n{}in {}",
                    x,
                    rhs.emit(indent + 1),
                    pad,
                    body.emit(indent)
                )
            }
            AgdaExpr::With(e, ws) => {
                let ws_s: Vec<String> = ws.iter().map(|w| w.emit(indent)).collect();
                format!("{} | {}", e.emit(indent), ws_s.join(" | "))
            }
            AgdaExpr::Case(scrutinee, clauses) => {
                let mut out = "(λ _case → {\n".to_string();
                for clause in clauses {
                    out.push_str(&format!(
                        "{}  ; {} → {}\n",
                        pad,
                        clause.emit_patterns(),
                        clause
                            .rhs
                            .as_ref()
                            .map(|r| r.emit(indent + 2))
                            .unwrap_or_else(|| "⊥-elim _".to_string())
                    ));
                }
                out.push_str(&format!("{}}} {}", pad, scrutinee.emit(indent)));
                out.push(')');
                out
            }
            AgdaExpr::Implicit(e) => format!("{{{}}}", e.emit(indent)),
            AgdaExpr::Tuple(elems) => {
                let es: Vec<String> = elems.iter().map(|e| e.emit(indent)).collect();
                format!("({})", es.join(" , "))
            }
            AgdaExpr::Record(fields) => {
                let fs: Vec<String> = fields
                    .iter()
                    .map(|(k, v)| format!("{} = {}", k, v.emit(indent)))
                    .collect();
                format!("record {{ {} }}", fs.join(" ; "))
            }
            AgdaExpr::Ascription(e, ty) => {
                format!("({} : {})", e.emit(indent), ty.emit(indent))
            }
            AgdaExpr::IfThenElse(cond, then_, else_) => {
                format!(
                    "if {} then {} else {}",
                    cond.emit(indent),
                    then_.emit(indent),
                    else_.emit(indent)
                )
            }
        }
    }
    /// Emit in the domain (left) position of a non-dependent Pi (arrow).
    /// Only wraps Pi/Lambda/Let/With/Case forms in parens; App is fine without.
    pub(super) fn emit_pi_dom(&self, indent: usize) -> String {
        match self {
            AgdaExpr::Pi(_, _, _) | AgdaExpr::Lambda(_, _) | AgdaExpr::Let(_, _, _) => {
                format!("({})", self.emit(indent))
            }
            _ => self.emit(indent),
        }
    }
    /// Emit in function position (left side of application).
    /// Application is left-associative, so `App` nodes do not need parens here.
    pub(super) fn emit_func(&self, indent: usize) -> String {
        match self {
            AgdaExpr::Var(_)
            | AgdaExpr::Num(_)
            | AgdaExpr::Str(_)
            | AgdaExpr::Hole
            | AgdaExpr::Underscore
            | AgdaExpr::Prop
            | AgdaExpr::Set(_)
            | AgdaExpr::Module(_)
            | AgdaExpr::Tuple(_)
            | AgdaExpr::Record(_)
            | AgdaExpr::Implicit(_)
            | AgdaExpr::App(_, _) => self.emit(indent),
            _ => format!("({})", self.emit(indent)),
        }
    }
    /// Emit as an atomic expression (wrap compound forms in parentheses).
    pub(super) fn emit_atom(&self, indent: usize) -> String {
        match self {
            AgdaExpr::Var(_)
            | AgdaExpr::Num(_)
            | AgdaExpr::Str(_)
            | AgdaExpr::Hole
            | AgdaExpr::Underscore
            | AgdaExpr::Prop
            | AgdaExpr::Set(_)
            | AgdaExpr::Module(_)
            | AgdaExpr::Tuple(_)
            | AgdaExpr::Record(_)
            | AgdaExpr::Implicit(_) => self.emit(indent),
            _ => format!("({})", self.emit(indent)),
        }
    }
}
/// Statistics for AgdaExt passes.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct AgdaExtPassStats {
    pub iterations: usize,
    pub changed: bool,
    pub nodes_visited: usize,
    pub nodes_modified: usize,
    pub time_ms: u64,
    pub memory_bytes: usize,
    pub errors: usize,
}
impl AgdaExtPassStats {
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
    pub fn merge(&mut self, o: &AgdaExtPassStats) {
        self.iterations += o.iterations;
        self.changed |= o.changed;
        self.nodes_visited += o.nodes_visited;
        self.nodes_modified += o.nodes_modified;
        self.time_ms += o.time_ms;
        self.memory_bytes = self.memory_bytes.max(o.memory_bytes);
        self.errors += o.errors;
    }
}
/// Configuration for AgdaX2 passes.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct AgdaX2PassConfig {
    pub name: String,
    pub phase: AgdaX2PassPhase,
    pub enabled: bool,
    pub max_iterations: usize,
    pub debug: u32,
    pub timeout_ms: Option<u64>,
}
impl AgdaX2PassConfig {
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            phase: AgdaX2PassPhase::Middle,
            enabled: true,
            max_iterations: 100,
            debug: 0,
            timeout_ms: None,
        }
    }
    #[allow(dead_code)]
    pub fn with_phase(mut self, phase: AgdaX2PassPhase) -> Self {
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
/// Constant folding helper for AgdaX2.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct AgdaX2ConstFolder {
    pub(super) folds: usize,
    pub(super) failures: usize,
    pub(super) enabled: bool,
}
impl AgdaX2ConstFolder {
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
/// Dependency graph for AgdaX2.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct AgdaX2DepGraph {
    pub(super) n: usize,
    pub(super) adj: Vec<Vec<usize>>,
    pub(super) rev: Vec<Vec<usize>>,
    pub(super) edge_count: usize,
}
impl AgdaX2DepGraph {
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
/// Pass execution phase for AgdaX2.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AgdaX2PassPhase {
    Early,
    Middle,
    Late,
    Finalize,
}
impl AgdaX2PassPhase {
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
/// Analysis cache for AgdaExt.
#[allow(dead_code)]
#[derive(Debug)]
pub struct AgdaExtCache {
    pub(super) entries: Vec<(u64, Vec<u8>, bool, u32)>,
    pub(super) cap: usize,
    pub(super) total_hits: u64,
    pub(super) total_misses: u64,
}
impl AgdaExtCache {
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
/// Liveness analysis for AgdaExt.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct AgdaExtLiveness {
    pub live_in: Vec<Vec<usize>>,
    pub live_out: Vec<Vec<usize>>,
    pub defs: Vec<Vec<usize>>,
    pub uses: Vec<Vec<usize>>,
}
impl AgdaExtLiveness {
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
/// Worklist for AgdaX2.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct AgdaX2Worklist {
    pub(super) items: std::collections::VecDeque<usize>,
    pub(super) present: Vec<bool>,
}
impl AgdaX2Worklist {
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
/// Liveness analysis for AgdaX2.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct AgdaX2Liveness {
    pub live_in: Vec<Vec<usize>>,
    pub live_out: Vec<Vec<usize>>,
    pub defs: Vec<Vec<usize>>,
    pub uses: Vec<Vec<usize>>,
}
impl AgdaX2Liveness {
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
/// A complete Agda 2 source file.
#[derive(Debug, Clone)]
pub struct AgdaModule {
    /// Top-level module name (must match filename without `.agda` extension)
    pub module_name: String,
    /// Module-level parameters (for parameterised modules)
    pub params: Vec<(String, AgdaExpr)>,
    /// `import` directives
    pub imports: Vec<String>,
    /// `open` directives (after imports)
    pub opens: Vec<String>,
    /// Top-level declarations
    pub declarations: Vec<AgdaDecl>,
}
impl AgdaModule {
    /// Construct an empty module.
    pub fn new(module_name: impl Into<String>) -> Self {
        Self {
            module_name: module_name.into(),
            params: Vec::new(),
            imports: Vec::new(),
            opens: Vec::new(),
            declarations: Vec::new(),
        }
    }
    /// Add an `import`.
    pub fn import(&mut self, module: impl Into<String>) {
        self.imports.push(module.into());
    }
    /// Add an `open`.
    pub fn open(&mut self, module: impl Into<String>) {
        self.opens.push(module.into());
    }
    /// Add a declaration.
    pub fn add(&mut self, decl: AgdaDecl) {
        self.declarations.push(decl);
    }
    /// Emit the full Agda 2 source as a `String`.
    pub fn emit(&self) -> String {
        let mut out = "-- Generated by OxiLean\n".to_string();
        let ps = emit_agda_params(&self.params, 0);
        if ps.is_empty() {
            out.push_str(&format!("module {} where\n\n", self.module_name));
        } else {
            out.push_str(&format!("module {} {} where\n\n", self.module_name, ps));
        }
        for imp in &self.imports {
            out.push_str(&format!("import {}\n", imp));
        }
        if !self.imports.is_empty() {
            out.push('\n');
        }
        for op in &self.opens {
            out.push_str(&format!("open {}\n", op));
        }
        if !self.opens.is_empty() {
            out.push('\n');
        }
        for decl in &self.declarations {
            out.push_str(&decl.emit(0));
            out.push('\n');
        }
        out
    }
}
/// Pass registry for AgdaX2.
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct AgdaX2PassRegistry {
    pub(super) configs: Vec<AgdaX2PassConfig>,
    pub(super) stats: Vec<AgdaX2PassStats>,
}
impl AgdaX2PassRegistry {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    #[allow(dead_code)]
    pub fn register(&mut self, c: AgdaX2PassConfig) {
        self.stats.push(AgdaX2PassStats::new());
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
    pub fn get(&self, i: usize) -> Option<&AgdaX2PassConfig> {
        self.configs.get(i)
    }
    #[allow(dead_code)]
    pub fn get_stats(&self, i: usize) -> Option<&AgdaX2PassStats> {
        self.stats.get(i)
    }
    #[allow(dead_code)]
    pub fn enabled_passes(&self) -> Vec<&AgdaX2PassConfig> {
        self.configs.iter().filter(|c| c.enabled).collect()
    }
    #[allow(dead_code)]
    pub fn passes_in_phase(&self, ph: &AgdaX2PassPhase) -> Vec<&AgdaX2PassConfig> {
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
/// Top-level `data` declaration.
#[derive(Debug, Clone, PartialEq)]
pub struct AgdaData {
    /// Type name
    pub name: String,
    /// Parameters (bound variables with types): `(A : Set)`
    pub params: Vec<(String, AgdaExpr)>,
    /// Indices / kind annotation: the `: Set` after parameters
    pub indices: AgdaExpr,
    /// Constructors
    pub constructors: Vec<AgdaConstructor>,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct AgdaWorklist {
    pub(super) items: std::collections::VecDeque<u32>,
    pub(super) in_worklist: std::collections::HashSet<u32>,
}
impl AgdaWorklist {
    #[allow(dead_code)]
    pub fn new() -> Self {
        AgdaWorklist {
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
pub struct AgdaConstantFoldingHelper;
impl AgdaConstantFoldingHelper {
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
