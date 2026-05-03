//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::super::functions::*;
use super::impls1::*;
use super::impls2::*;
use std::collections::{HashMap, HashSet, VecDeque};

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct AgdaAnalysisCache {
    pub(crate) entries: std::collections::HashMap<String, AgdaCacheEntry>,
    pub(crate) max_size: usize,
    pub(crate) hits: u64,
    pub(crate) misses: u64,
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
    pub(crate) nodes: Vec<u32>,
    pub(crate) edges: Vec<(u32, u32)>,
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
    pub(crate) entries: Vec<(u64, Vec<u8>, bool, u32)>,
    pub(crate) cap: usize,
    pub(crate) total_hits: u64,
    pub(crate) total_misses: u64,
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
    pub(crate) idom: Vec<Option<usize>>,
    pub(crate) children: Vec<Vec<usize>>,
    pub(crate) depth: Vec<usize>,
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
    pub(crate) items: std::collections::VecDeque<usize>,
    pub(crate) present: Vec<bool>,
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
