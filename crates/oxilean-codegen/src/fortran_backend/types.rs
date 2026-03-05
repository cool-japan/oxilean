//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;
use std::collections::{HashMap, HashSet};

use super::functions::FORTRAN_KEYWORDS;

use super::functions::*;
use std::collections::VecDeque;

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum FortPassPhase {
    Analysis,
    Transformation,
    Verification,
    Cleanup,
}
impl FortPassPhase {
    #[allow(dead_code)]
    pub fn name(&self) -> &str {
        match self {
            FortPassPhase::Analysis => "analysis",
            FortPassPhase::Transformation => "transformation",
            FortPassPhase::Verification => "verification",
            FortPassPhase::Cleanup => "cleanup",
        }
    }
    #[allow(dead_code)]
    pub fn is_modifying(&self) -> bool {
        matches!(self, FortPassPhase::Transformation | FortPassPhase::Cleanup)
    }
}
/// Dominator tree for FortranExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FortranExtDomTree {
    pub(super) idom: Vec<Option<usize>>,
    pub(super) children: Vec<Vec<usize>>,
    pub(super) depth: Vec<usize>,
}
impl FortranExtDomTree {
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
#[derive(Debug, Clone, Default)]
pub struct FortPassStats {
    pub total_runs: u32,
    pub successful_runs: u32,
    pub total_changes: u64,
    pub time_ms: u64,
    pub iterations_used: u32,
}
impl FortPassStats {
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
/// Liveness analysis for FortranExt.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct FortranExtLiveness {
    pub live_in: Vec<Vec<usize>>,
    pub live_out: Vec<Vec<usize>>,
    pub defs: Vec<Vec<usize>>,
    pub uses: Vec<Vec<usize>>,
}
impl FortranExtLiveness {
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
#[allow(dead_code)]
pub struct FortPassRegistry {
    pub(super) configs: Vec<FortPassConfig>,
    pub(super) stats: std::collections::HashMap<String, FortPassStats>,
}
impl FortPassRegistry {
    #[allow(dead_code)]
    pub fn new() -> Self {
        FortPassRegistry {
            configs: Vec::new(),
            stats: std::collections::HashMap::new(),
        }
    }
    #[allow(dead_code)]
    pub fn register(&mut self, config: FortPassConfig) {
        self.stats
            .insert(config.pass_name.clone(), FortPassStats::new());
        self.configs.push(config);
    }
    #[allow(dead_code)]
    pub fn enabled_passes(&self) -> Vec<&FortPassConfig> {
        self.configs.iter().filter(|c| c.enabled).collect()
    }
    #[allow(dead_code)]
    pub fn get_stats(&self, name: &str) -> Option<&FortPassStats> {
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
pub struct FortAnalysisCache {
    pub(super) entries: std::collections::HashMap<String, FortCacheEntry>,
    pub(super) max_size: usize,
    pub(super) hits: u64,
    pub(super) misses: u64,
}
impl FortAnalysisCache {
    #[allow(dead_code)]
    pub fn new(max_size: usize) -> Self {
        FortAnalysisCache {
            entries: std::collections::HashMap::new(),
            max_size,
            hits: 0,
            misses: 0,
        }
    }
    #[allow(dead_code)]
    pub fn get(&mut self, key: &str) -> Option<&FortCacheEntry> {
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
            FortCacheEntry {
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
pub struct FortDepGraph {
    pub(super) nodes: Vec<u32>,
    pub(super) edges: Vec<(u32, u32)>,
}
impl FortDepGraph {
    #[allow(dead_code)]
    pub fn new() -> Self {
        FortDepGraph {
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
/// Fortran binary operators.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FortranBinOp {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    Concat,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    And,
    Or,
    Eqv,
    Neqv,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FortPassConfig {
    pub phase: FortPassPhase,
    pub enabled: bool,
    pub max_iterations: u32,
    pub debug_output: bool,
    pub pass_name: String,
}
impl FortPassConfig {
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>, phase: FortPassPhase) -> Self {
        FortPassConfig {
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
/// Configuration for FortranExt passes.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FortranExtPassConfig {
    pub name: String,
    pub phase: FortranExtPassPhase,
    pub enabled: bool,
    pub max_iterations: usize,
    pub debug: u32,
    pub timeout_ms: Option<u64>,
}
impl FortranExtPassConfig {
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            phase: FortranExtPassPhase::Middle,
            enabled: true,
            max_iterations: 100,
            debug: 0,
            timeout_ms: None,
        }
    }
    #[allow(dead_code)]
    pub fn with_phase(mut self, phase: FortranExtPassPhase) -> Self {
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
pub struct FortConstantFoldingHelper;
impl FortConstantFoldingHelper {
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
/// Worklist for FortranExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FortranExtWorklist {
    pub(super) items: std::collections::VecDeque<usize>,
    pub(super) present: Vec<bool>,
}
impl FortranExtWorklist {
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
/// A Fortran derived-type declaration.
#[derive(Debug, Clone)]
pub struct FortranDerivedType {
    pub name: String,
    pub fields: Vec<FortranDecl>,
    pub doc: Option<String>,
}
impl FortranDerivedType {
    pub fn new(name: impl Into<String>) -> Self {
        FortranDerivedType {
            name: name.into(),
            fields: Vec::new(),
            doc: None,
        }
    }
}
/// A Fortran PROGRAM.
#[derive(Debug, Clone)]
pub struct FortranProgram {
    pub name: String,
    pub use_modules: Vec<String>,
    pub implicit_none: bool,
    pub decls: Vec<FortranDecl>,
    pub body: Vec<FortranStmt>,
    pub contains: Vec<FortranSubprogram>,
}
impl FortranProgram {
    pub fn new(name: impl Into<String>) -> Self {
        FortranProgram {
            name: name.into(),
            use_modules: Vec::new(),
            implicit_none: true,
            decls: Vec::new(),
            body: Vec::new(),
            contains: Vec::new(),
        }
    }
}
/// Fortran statement AST.
#[derive(Debug, Clone, PartialEq)]
pub enum FortranStmt {
    /// `var = expr`
    Assign(FortranExpr, FortranExpr),
    /// `CALL sub(args)`
    Call(String, Vec<FortranExpr>),
    /// `RETURN`
    Return,
    /// `IF (cond) THEN ... [ELSE IF ...] [ELSE ...] END IF`
    If(Vec<(FortranExpr, Vec<FortranStmt>)>, Vec<FortranStmt>),
    /// `SELECT CASE (expr) ... END SELECT`
    SelectCase(FortranExpr, Vec<FortranCase>, Vec<FortranStmt>),
    /// `DO [label] [var = lo, hi [, step]] ... END DO`
    Do(Option<String>, Vec<FortranStmt>),
    /// Counted DO loop: `DO var = lo, hi [, step]`
    DoCount(
        String,
        FortranExpr,
        FortranExpr,
        Option<FortranExpr>,
        Vec<FortranStmt>,
    ),
    /// `DO WHILE (cond) ... END DO`
    DoWhile(FortranExpr, Vec<FortranStmt>),
    /// `EXIT [label]`
    Exit(Option<String>),
    /// `CYCLE [label]`
    Cycle(Option<String>),
    /// `STOP [code]`
    Stop(Option<FortranExpr>),
    /// `ALLOCATE(var(dims), STAT=stat)`
    Allocate(FortranExpr, Option<String>),
    /// `DEALLOCATE(var, STAT=stat)`
    Deallocate(FortranExpr, Option<String>),
    /// `NULLIFY(ptr)`
    Nullify(FortranExpr),
    /// `PRINT *, expr1, expr2, ...`
    Print(Vec<FortranExpr>),
    /// `WRITE(unit, fmt) exprs`
    Write(String, String, Vec<FortranExpr>),
    /// `READ(unit, fmt) vars`
    Read(String, String, Vec<FortranExpr>),
    /// `OPEN(unit, FILE=..., STATUS=...)`
    Open(u32, String, String),
    /// `CLOSE(unit)`
    Close(u32),
    /// `CONTINUE`
    Continue,
    /// Raw Fortran statement
    Raw(String),
    /// Block: sequential statements
    Block(Vec<FortranStmt>),
}
/// Fortran 90+ code generation backend.
pub struct FortranBackend {
    pub(super) var_counter: u64,
    /// Map original name → mangled Fortran identifier.
    pub(super) name_cache: HashMap<String, String>,
    /// Fortran reserved words.
    pub(super) reserved: HashSet<String>,
    /// Indentation step (spaces).
    pub(super) indent_width: usize,
    /// Line-continuation character position (Fortran 90 free-form: `&`).
    pub(super) max_line_len: usize,
}
impl FortranBackend {
    pub fn new() -> Self {
        let mut reserved = HashSet::new();
        for kw in FORTRAN_KEYWORDS {
            reserved.insert(kw.to_lowercase());
        }
        FortranBackend {
            var_counter: 0,
            name_cache: HashMap::new(),
            reserved,
            indent_width: 2,
            max_line_len: 120,
        }
    }
    pub fn fresh_var(&mut self) -> String {
        let id = self.var_counter;
        self.var_counter += 1;
        format!("ftv{}", id)
    }
    pub fn mangle_name(&mut self, name: &str) -> String {
        if let Some(cached) = self.name_cache.get(name) {
            return cached.clone();
        }
        let mangled = mangle_fortran_ident(name, &self.reserved);
        self.name_cache.insert(name.to_string(), mangled.clone());
        mangled
    }
    /// Emit a complete Fortran module as a String.
    pub fn emit_module(&mut self, module: &FortranModule) -> String {
        let mut out = String::new();
        if let Some(doc) = &module.doc {
            for line in doc.lines() {
                out.push_str(&format!("! {}\n", line));
            }
        }
        out.push_str(&format!("MODULE {}\n", module.name.to_uppercase()));
        for used in &module.use_modules {
            out.push_str(&format!("  USE {}\n", used.to_uppercase()));
        }
        if module.implicit_none {
            out.push_str("  IMPLICIT NONE\n");
        }
        for dt in &module.derived_types {
            out.push_str(&self.emit_derived_type(dt, 1));
        }
        for decl in &module.module_vars {
            out.push_str(&self.emit_decl(decl, 1));
        }
        if !module.contains.is_empty() {
            out.push_str("CONTAINS\n");
            for sub in &module.contains {
                out.push_str(&self.emit_subprogram(sub, 0));
            }
        }
        out.push_str(&format!("END MODULE {}\n", module.name.to_uppercase()));
        out
    }
    /// Emit a standalone PROGRAM.
    pub fn emit_program(&mut self, prog: &FortranProgram) -> String {
        let mut out = String::new();
        out.push_str(&format!("PROGRAM {}\n", prog.name.to_uppercase()));
        for used in &prog.use_modules {
            out.push_str(&format!("  USE {}\n", used.to_uppercase()));
        }
        if prog.implicit_none {
            out.push_str("  IMPLICIT NONE\n");
        }
        for decl in &prog.decls {
            out.push_str(&self.emit_decl(decl, 1));
        }
        out.push_str("  ! --- executable section ---\n");
        for stmt in &prog.body {
            out.push_str(&self.emit_stmt(stmt, 1));
        }
        if !prog.contains.is_empty() {
            out.push_str("CONTAINS\n");
            for sub in &prog.contains {
                out.push_str(&self.emit_subprogram(sub, 0));
            }
        }
        out.push_str(&format!("END PROGRAM {}\n", prog.name.to_uppercase()));
        out
    }
    /// Emit a FUNCTION or SUBROUTINE.
    pub fn emit_subprogram(&mut self, sub: &FortranSubprogram, depth: usize) -> String {
        let indent = self.indent(depth);
        let mut out = String::new();
        if let Some(doc) = &sub.doc {
            for line in doc.lines() {
                out.push_str(&format!("{}! {}\n", indent, line));
            }
        }
        let mut prefix = String::new();
        if sub.is_pure {
            prefix.push_str("PURE ");
        }
        if sub.is_elemental {
            prefix.push_str("ELEMENTAL ");
        }
        if sub.is_recursive {
            prefix.push_str("RECURSIVE ");
        }
        let dummy_list = sub.dummy_args.join(", ");
        if sub.is_function() {
            let ret_ty = &sub.return_type;
            out.push_str(&format!(
                "{}{}{} FUNCTION {}({})\n",
                indent,
                prefix,
                ret_ty,
                sub.name.to_uppercase(),
                dummy_list
            ));
        } else {
            out.push_str(&format!(
                "{}{}SUBROUTINE {}({})\n",
                indent,
                prefix,
                sub.name.to_uppercase(),
                dummy_list
            ));
        }
        let inner = self.indent(depth + 1);
        out.push_str(&format!("{}IMPLICIT NONE\n", inner));
        if sub.is_function() {
            out.push_str(&format!(
                "{}{} :: {}\n",
                inner,
                sub.return_type,
                sub.name.to_uppercase()
            ));
        }
        for decl in &sub.decls {
            out.push_str(&self.emit_decl(decl, depth + 1));
        }
        if !sub.decls.is_empty() {
            out.push_str(&format!("{}! --- executable section ---\n", inner));
        }
        for stmt in &sub.body {
            out.push_str(&self.emit_stmt(stmt, depth + 1));
        }
        if sub.is_function() {
            out.push_str(&format!(
                "{}END FUNCTION {}\n",
                indent,
                sub.name.to_uppercase()
            ));
        } else {
            out.push_str(&format!(
                "{}END SUBROUTINE {}\n",
                indent,
                sub.name.to_uppercase()
            ));
        }
        out
    }
    /// Emit a derived-type definition.
    pub fn emit_derived_type(&self, dt: &FortranDerivedType, depth: usize) -> String {
        let indent = self.indent(depth);
        let mut out = String::new();
        if let Some(doc) = &dt.doc {
            out.push_str(&format!("{}! {}\n", indent, doc));
        }
        out.push_str(&format!("{}TYPE :: {}\n", indent, dt.name.to_uppercase()));
        for field in &dt.fields {
            out.push_str(&self.emit_decl(field, depth + 1));
        }
        out.push_str(&format!("{}END TYPE {}\n", indent, dt.name.to_uppercase()));
        out
    }
    /// Emit a variable declaration.
    pub fn emit_decl(&self, decl: &FortranDecl, depth: usize) -> String {
        let indent = self.indent(depth);
        let mut attrs: Vec<String> = Vec::new();
        if let Some(intent) = &decl.intent {
            attrs.push(format!("INTENT({})", intent));
        }
        if decl.is_parameter {
            attrs.push("PARAMETER".to_string());
        }
        let attr_str = if attrs.is_empty() {
            String::new()
        } else {
            format!(", {}", attrs.join(", "))
        };
        if let Some(init) = &decl.initial_value {
            format!(
                "{}{}{} :: {} = {}\n",
                indent,
                decl.ty,
                attr_str,
                decl.name.to_uppercase(),
                init
            )
        } else {
            format!(
                "{}{}{} :: {}\n",
                indent,
                decl.ty,
                attr_str,
                decl.name.to_uppercase()
            )
        }
    }
    /// Emit a Fortran statement with proper indentation.
    pub fn emit_stmt(&self, stmt: &FortranStmt, depth: usize) -> String {
        let indent = self.indent(depth);
        match stmt {
            FortranStmt::Assign(lhs, rhs) => format!("{}{} = {}\n", indent, lhs, rhs),
            FortranStmt::Call(name, args) => {
                let arg_list: Vec<String> = args.iter().map(|a| format!("{}", a)).collect();
                format!(
                    "{}CALL {}({})\n",
                    indent,
                    name.to_uppercase(),
                    arg_list.join(", ")
                )
            }
            FortranStmt::Return => format!("{}RETURN\n", indent),
            FortranStmt::If(branches, else_body) => {
                let mut out = String::new();
                for (idx, (cond, body)) in branches.iter().enumerate() {
                    if idx == 0 {
                        out.push_str(&format!("{}IF ({}) THEN\n", indent, cond));
                    } else {
                        out.push_str(&format!("{}ELSE IF ({}) THEN\n", indent, cond));
                    }
                    for s in body {
                        out.push_str(&self.emit_stmt(s, depth + 1));
                    }
                }
                if !else_body.is_empty() {
                    out.push_str(&format!("{}ELSE\n", indent));
                    for s in else_body {
                        out.push_str(&self.emit_stmt(s, depth + 1));
                    }
                }
                out.push_str(&format!("{}END IF\n", indent));
                out
            }
            FortranStmt::SelectCase(expr, cases, default) => {
                let mut out = format!("{}SELECT CASE ({})\n", indent, expr);
                for case in cases {
                    if let Some(vals) = &case.values {
                        let val_str: Vec<String> = vals.iter().map(|v| format!("{}", v)).collect();
                        out.push_str(&format!("{}CASE ({})\n", indent, val_str.join(", ")));
                    } else {
                        out.push_str(&format!("{}CASE DEFAULT\n", indent));
                    }
                    for s in &case.body {
                        out.push_str(&self.emit_stmt(s, depth + 1));
                    }
                }
                if !default.is_empty() {
                    out.push_str(&format!("{}CASE DEFAULT\n", indent));
                    for s in default {
                        out.push_str(&self.emit_stmt(s, depth + 1));
                    }
                }
                out.push_str(&format!("{}END SELECT\n", indent));
                out
            }
            FortranStmt::Do(label, body) => {
                let label_str = label
                    .as_deref()
                    .map(|l| format!("{}: ", l))
                    .unwrap_or_default();
                let mut out = format!("{}{}DO\n", indent, label_str);
                for s in body {
                    out.push_str(&self.emit_stmt(s, depth + 1));
                }
                out.push_str(&format!("{}END DO\n", indent));
                out
            }
            FortranStmt::DoCount(var, lo, hi, step, body) => {
                let step_str = step
                    .as_ref()
                    .map(|s| format!(", {}", s))
                    .unwrap_or_default();
                let mut out = format!(
                    "{}DO {} = {}, {}{}\n",
                    indent,
                    var.to_uppercase(),
                    lo,
                    hi,
                    step_str
                );
                for s in body {
                    out.push_str(&self.emit_stmt(s, depth + 1));
                }
                out.push_str(&format!("{}END DO\n", indent));
                out
            }
            FortranStmt::DoWhile(cond, body) => {
                let mut out = format!("{}DO WHILE ({})\n", indent, cond);
                for s in body {
                    out.push_str(&self.emit_stmt(s, depth + 1));
                }
                out.push_str(&format!("{}END DO\n", indent));
                out
            }
            FortranStmt::Exit(label) => match label {
                Some(l) => format!("{}EXIT {}\n", indent, l),
                None => format!("{}EXIT\n", indent),
            },
            FortranStmt::Cycle(label) => match label {
                Some(l) => format!("{}CYCLE {}\n", indent, l),
                None => format!("{}CYCLE\n", indent),
            },
            FortranStmt::Stop(code) => match code {
                Some(c) => format!("{}STOP {}\n", indent, c),
                None => format!("{}STOP\n", indent),
            },
            FortranStmt::Allocate(var, stat) => {
                if let Some(stat_var) = stat {
                    format!("{}ALLOCATE({}, STAT={})\n", indent, var, stat_var)
                } else {
                    format!("{}ALLOCATE({})\n", indent, var)
                }
            }
            FortranStmt::Deallocate(var, stat) => {
                if let Some(stat_var) = stat {
                    format!("{}DEALLOCATE({}, STAT={})\n", indent, var, stat_var)
                } else {
                    format!("{}DEALLOCATE({})\n", indent, var)
                }
            }
            FortranStmt::Nullify(ptr) => format!("{}NULLIFY({})\n", indent, ptr),
            FortranStmt::Print(exprs) => {
                if exprs.is_empty() {
                    format!("{}PRINT *\n", indent)
                } else {
                    let args: Vec<String> = exprs.iter().map(|e| format!("{}", e)).collect();
                    format!("{}PRINT *, {}\n", indent, args.join(", "))
                }
            }
            FortranStmt::Write(unit, fmt, exprs) => {
                let args: Vec<String> = exprs.iter().map(|e| format!("{}", e)).collect();
                let data = if args.is_empty() {
                    String::new()
                } else {
                    format!(" {}", args.join(", "))
                };
                format!("{}WRITE({}, {}){}\n", indent, unit, fmt, data)
            }
            FortranStmt::Read(unit, fmt, vars) => {
                let args: Vec<String> = vars.iter().map(|v| format!("{}", v)).collect();
                format!("{}READ({}, {}) {}\n", indent, unit, fmt, args.join(", "))
            }
            FortranStmt::Open(unit, file, status) => {
                format!(
                    "{}OPEN(UNIT={}, FILE='{}', STATUS='{}')\n",
                    indent, unit, file, status
                )
            }
            FortranStmt::Close(unit) => format!("{}CLOSE(UNIT={})\n", indent, unit),
            FortranStmt::Continue => format!("{}CONTINUE\n", indent),
            FortranStmt::Raw(code) => format!("{}{}\n", indent, code),
            FortranStmt::Block(stmts) => {
                let mut out = String::new();
                for s in stmts {
                    out.push_str(&self.emit_stmt(s, depth));
                }
                out
            }
        }
    }
    pub(super) fn indent(&self, depth: usize) -> String {
        " ".repeat(depth * self.indent_width)
    }
    /// Compile an LCNF function to a `FortranSubprogram`.
    pub fn compile_lcnf_function(
        &mut self,
        func: &LcnfFunDecl,
    ) -> Result<FortranSubprogram, String> {
        let name = self.mangle_name(&func.name.to_string());
        let ret_ty = lcnf_type_to_fortran(&func.ret_type);
        let mut dummy_args: Vec<String> = Vec::new();
        let mut decls: Vec<FortranDecl> = Vec::new();
        for param in &func.params {
            let pname = format!("px{}", param.id.0);
            let pty = lcnf_type_to_fortran(&param.ty);
            dummy_args.push(pname.clone());
            decls.push(FortranDecl::param_in(pty, &pname));
        }
        let mut body_stmts = Vec::new();
        let result_expr = self.compile_expr(&func.body, &mut body_stmts, &mut decls)?;
        body_stmts.push(FortranStmt::Assign(
            FortranExpr::Var(name.to_uppercase()),
            result_expr,
        ));
        body_stmts.push(FortranStmt::Return);
        let mut sub = FortranSubprogram::function(name.clone(), ret_ty);
        sub.dummy_args = dummy_args;
        sub.decls = decls;
        sub.body = body_stmts;
        Ok(sub)
    }
    /// Compile an LCNF module to a `FortranModule`.
    pub fn compile_lcnf_module(&mut self, module: &LcnfModule) -> Result<FortranModule, String> {
        let mut fort_module = FortranModule::new("oxilean_generated");
        fort_module.use_modules.push("iso_fortran_env".to_string());
        let ctor_names = collect_ctor_names_module(module);
        for ctor_name in &ctor_names {
            fort_module
                .derived_types
                .push(make_ctor_derived_type(ctor_name));
        }
        for func in &module.fun_decls {
            let sub = self.compile_lcnf_function(func)?;
            fort_module.contains.push(sub);
        }
        Ok(fort_module)
    }
    pub(super) fn compile_expr(
        &mut self,
        expr: &LcnfExpr,
        stmts: &mut Vec<FortranStmt>,
        decls: &mut Vec<FortranDecl>,
    ) -> Result<FortranExpr, String> {
        match expr {
            LcnfExpr::Return(arg) => Ok(self.compile_arg(arg)),
            LcnfExpr::Unreachable => {
                stmts.push(FortranStmt::Raw(
                    "STOP 'OxiLean: unreachable code reached'".to_string(),
                ));
                Ok(FortranExpr::Lit(FortranLit::Int(0)))
            }
            LcnfExpr::TailCall(func, args) => {
                let name = match func {
                    LcnfArg::Var(id) => format!("FX{}", id.0),
                    LcnfArg::Lit(_) => "UNKNOWN_FUNC".to_string(),
                    _ => "ERASED_FUNC".to_string(),
                };
                let fort_args: Vec<FortranExpr> =
                    args.iter().map(|a| self.compile_arg(a)).collect();
                Ok(FortranExpr::Call(name.to_uppercase(), fort_args))
            }
            LcnfExpr::Let {
                id,
                ty,
                value,
                body,
                ..
            } => {
                let var_name = format!("lv{}", id.0);
                let fort_ty = lcnf_type_to_fortran(ty);
                let val_expr = self.compile_let_value(value)?;
                decls.push(FortranDecl::local(fort_ty, &var_name));
                stmts.push(FortranStmt::Assign(
                    FortranExpr::Var(var_name.to_uppercase()),
                    val_expr,
                ));
                self.compile_expr(body, stmts, decls)
            }
            LcnfExpr::Case {
                scrutinee,
                alts,
                default,
                ..
            } => {
                let scrutinee_expr = FortranExpr::Var(format!("LV{}", scrutinee.0));
                let result_var = self.fresh_var();
                let result_decl = FortranDecl::local(FortranType::FtIntegerK(8), &result_var);
                decls.push(result_decl);
                let tag_expr = FortranExpr::Component(Box::new(scrutinee_expr), "tag".to_string());
                let mut cases: Vec<FortranCase> = Vec::new();
                for alt in alts {
                    let mut branch_stmts: Vec<FortranStmt> = Vec::new();
                    for (idx, param) in alt.params.iter().enumerate() {
                        let pname = format!("lv{}", param.id.0);
                        let pty = lcnf_type_to_fortran(&param.ty);
                        decls.push(FortranDecl::local(pty, &pname));
                        let field_expr = FortranExpr::Component(
                            Box::new(FortranExpr::Var(format!("SCRUTINEE_F{}", idx))),
                            format!("field{}", idx),
                        );
                        branch_stmts.push(FortranStmt::Assign(
                            FortranExpr::Var(pname.to_uppercase()),
                            field_expr,
                        ));
                    }
                    let branch_result = self.compile_expr(&alt.body, &mut branch_stmts, decls)?;
                    branch_stmts.push(FortranStmt::Assign(
                        FortranExpr::Var(result_var.to_uppercase()),
                        branch_result,
                    ));
                    cases.push(FortranCase {
                        values: Some(vec![FortranExpr::Lit(FortranLit::Int(alt.ctor_tag as i64))]),
                        body: branch_stmts,
                    });
                }
                let mut default_stmts: Vec<FortranStmt> = Vec::new();
                if let Some(def) = default {
                    let def_result = self.compile_expr(def, &mut default_stmts, decls)?;
                    default_stmts.push(FortranStmt::Assign(
                        FortranExpr::Var(result_var.to_uppercase()),
                        def_result,
                    ));
                } else {
                    default_stmts.push(FortranStmt::Raw(
                        "STOP 'OxiLean: unreachable branch'".to_string(),
                    ));
                }
                stmts.push(FortranStmt::SelectCase(tag_expr, cases, default_stmts));
                Ok(FortranExpr::Var(result_var.to_uppercase()))
            }
        }
    }
    pub(super) fn compile_let_value(
        &mut self,
        value: &LcnfLetValue,
    ) -> Result<FortranExpr, String> {
        match value {
            LcnfLetValue::Lit(lit) => Ok(self.compile_lit(lit)),
            LcnfLetValue::Erased => Ok(FortranExpr::Lit(FortranLit::Logical(false))),
            LcnfLetValue::FVar(id) => Ok(FortranExpr::Var(format!("FX{}", id.0).to_uppercase())),
            LcnfLetValue::App(func, args) => {
                let name = match func {
                    LcnfArg::Var(id) => format!("FX{}", id.0),
                    _ => "UNKNOWN_FUNC".to_string(),
                };
                let fort_args: Vec<FortranExpr> =
                    args.iter().map(|a| self.compile_arg(a)).collect();
                Ok(FortranExpr::Call(name.to_uppercase(), fort_args))
            }
            LcnfLetValue::Proj(_name, idx, var) => {
                let base = FortranExpr::Var(format!("LV{}", var.0));
                Ok(FortranExpr::Component(
                    Box::new(base),
                    format!("field{}", idx),
                ))
            }
            LcnfLetValue::Ctor(name, tag, args) => {
                let ctor_name = self.mangle_name(name);
                let mut fields: Vec<(String, FortranExpr)> = Vec::new();
                fields.push((
                    "tag".to_string(),
                    FortranExpr::Lit(FortranLit::Int(*tag as i64)),
                ));
                for (idx, arg) in args.iter().enumerate() {
                    fields.push((format!("field{}", idx), self.compile_arg(arg)));
                }
                Ok(FortranExpr::TypeCtor(ctor_name.to_uppercase(), fields))
            }
            LcnfLetValue::Reset(_var) => Ok(FortranExpr::Lit(FortranLit::Logical(false))),
            LcnfLetValue::Reuse(_slot, name, tag, args) => {
                let ctor_name = self.mangle_name(name);
                let mut fields: Vec<(String, FortranExpr)> = Vec::new();
                fields.push((
                    "tag".to_string(),
                    FortranExpr::Lit(FortranLit::Int(*tag as i64)),
                ));
                for (idx, arg) in args.iter().enumerate() {
                    fields.push((format!("field{}", idx), self.compile_arg(arg)));
                }
                Ok(FortranExpr::TypeCtor(ctor_name.to_uppercase(), fields))
            }
        }
    }
    pub(super) fn compile_arg(&self, arg: &LcnfArg) -> FortranExpr {
        match arg {
            LcnfArg::Var(id) => FortranExpr::Var(format!("LV{}", id.0)),
            LcnfArg::Lit(lit) => self.compile_lit(lit),
            LcnfArg::Erased => FortranExpr::Lit(FortranLit::Logical(false)),
            LcnfArg::Type(_) => FortranExpr::Lit(FortranLit::Logical(false)),
        }
    }
    pub(super) fn compile_lit(&self, lit: &LcnfLit) -> FortranExpr {
        match lit {
            LcnfLit::Nat(n) => FortranExpr::Lit(FortranLit::Int(*n as i64)),
            LcnfLit::Str(s) => FortranExpr::Lit(FortranLit::Char(s.clone())),
        }
    }
}
/// Constant folding helper for FortranExt.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct FortranExtConstFolder {
    pub(super) folds: usize,
    pub(super) failures: usize,
    pub(super) enabled: bool,
}
impl FortranExtConstFolder {
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
/// Fortran unary operators.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FortranUnaryOp {
    Neg,
    Not,
    Pos,
}
/// Fortran type declaration.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FortranType {
    /// `INTEGER` (default kind, typically 4 bytes)
    FtInteger,
    /// `INTEGER(KIND=k)` — explicit kind
    FtIntegerK(u8),
    /// `REAL` (default, typically 4 bytes)
    FtReal,
    /// `REAL(KIND=8)` / `DOUBLE PRECISION`
    FtDouble,
    /// `COMPLEX`
    FtComplex,
    /// `COMPLEX(KIND=8)` — double-precision complex
    FtComplexDouble,
    /// `LOGICAL`
    FtLogical,
    /// `CHARACTER(LEN=n)` — fixed-length string
    FtCharacter(Option<usize>),
    /// `CHARACTER(LEN=*)` — assumed-length string
    FtCharacterStar,
    /// 1-D array: `TYPE, DIMENSION(n) :: var`
    FtArray(Box<FortranType>, ArrayDimension),
    /// Derived type: `TYPE(name)`
    FtDerived(String),
    /// `CLASS(name)` (polymorphic, F2003)
    FtClass(String),
    /// `CLASS(*)` — unlimited polymorphic
    FtClassStar,
    /// `TYPE(*)` — assumed-type (F2018)
    FtAssumedType,
    /// Pointer to a type: `TYPE, POINTER :: var`
    FtPointer(Box<FortranType>),
    /// Allocatable: `TYPE, ALLOCATABLE :: var`
    FtAllocatable(Box<FortranType>),
    /// Void (used only for subroutines)
    FtVoid,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FortLivenessInfo {
    pub live_in: Vec<std::collections::HashSet<u32>>,
    pub live_out: Vec<std::collections::HashSet<u32>>,
    pub defs: Vec<std::collections::HashSet<u32>>,
    pub uses: Vec<std::collections::HashSet<u32>>,
}
impl FortLivenessInfo {
    #[allow(dead_code)]
    pub fn new(block_count: usize) -> Self {
        FortLivenessInfo {
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
/// A Fortran FUNCTION or SUBROUTINE.
#[derive(Debug, Clone)]
pub struct FortranSubprogram {
    pub name: String,
    /// For a FUNCTION, the return type; for SUBROUTINE, use `FtVoid`.
    pub return_type: FortranType,
    /// Dummy argument names (order matters in Fortran).
    pub dummy_args: Vec<String>,
    /// All declarations (both dummy and local).
    pub decls: Vec<FortranDecl>,
    /// Executable body.
    pub body: Vec<FortranStmt>,
    /// Whether this is a pure function.
    pub is_pure: bool,
    /// Whether this is an elemental function.
    pub is_elemental: bool,
    /// Whether this is recursive.
    pub is_recursive: bool,
    pub doc: Option<String>,
}
impl FortranSubprogram {
    pub fn function(name: impl Into<String>, ret: FortranType) -> Self {
        FortranSubprogram {
            name: name.into(),
            return_type: ret,
            dummy_args: Vec::new(),
            decls: Vec::new(),
            body: Vec::new(),
            is_pure: false,
            is_elemental: false,
            is_recursive: false,
            doc: None,
        }
    }
    pub fn subroutine(name: impl Into<String>) -> Self {
        FortranSubprogram {
            name: name.into(),
            return_type: FortranType::FtVoid,
            dummy_args: Vec::new(),
            decls: Vec::new(),
            body: Vec::new(),
            is_pure: false,
            is_elemental: false,
            is_recursive: false,
            doc: None,
        }
    }
    pub fn is_function(&self) -> bool {
        self.return_type != FortranType::FtVoid
    }
}
/// Dependency graph for FortranExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FortranExtDepGraph {
    pub(super) n: usize,
    pub(super) adj: Vec<Vec<usize>>,
    pub(super) rev: Vec<Vec<usize>>,
    pub(super) edge_count: usize,
}
impl FortranExtDepGraph {
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
/// A variable declaration inside a subprogram.
#[derive(Debug, Clone)]
pub struct FortranDecl {
    pub ty: FortranType,
    pub name: String,
    pub intent: Option<FortranIntent>,
    pub is_parameter: bool,
    pub initial_value: Option<FortranExpr>,
    pub doc: Option<String>,
}
impl FortranDecl {
    pub fn local(ty: FortranType, name: impl Into<String>) -> Self {
        FortranDecl {
            ty,
            name: name.into(),
            intent: None,
            is_parameter: false,
            initial_value: None,
            doc: None,
        }
    }
    pub fn param_in(ty: FortranType, name: impl Into<String>) -> Self {
        FortranDecl {
            ty,
            name: name.into(),
            intent: Some(FortranIntent::In),
            is_parameter: false,
            initial_value: None,
            doc: None,
        }
    }
    pub fn param_out(ty: FortranType, name: impl Into<String>) -> Self {
        FortranDecl {
            ty,
            name: name.into(),
            intent: Some(FortranIntent::Out),
            is_parameter: false,
            initial_value: None,
            doc: None,
        }
    }
}
/// A CASE block: `CASE (values) stmts`
#[derive(Debug, Clone, PartialEq)]
pub struct FortranCase {
    /// `None` means `CASE DEFAULT`
    pub values: Option<Vec<FortranExpr>>,
    pub body: Vec<FortranStmt>,
}
/// Pass registry for FortranExt.
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct FortranExtPassRegistry {
    pub(super) configs: Vec<FortranExtPassConfig>,
    pub(super) stats: Vec<FortranExtPassStats>,
}
impl FortranExtPassRegistry {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    #[allow(dead_code)]
    pub fn register(&mut self, c: FortranExtPassConfig) {
        self.stats.push(FortranExtPassStats::new());
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
    pub fn get(&self, i: usize) -> Option<&FortranExtPassConfig> {
        self.configs.get(i)
    }
    #[allow(dead_code)]
    pub fn get_stats(&self, i: usize) -> Option<&FortranExtPassStats> {
        self.stats.get(i)
    }
    #[allow(dead_code)]
    pub fn enabled_passes(&self) -> Vec<&FortranExtPassConfig> {
        self.configs.iter().filter(|c| c.enabled).collect()
    }
    #[allow(dead_code)]
    pub fn passes_in_phase(&self, ph: &FortranExtPassPhase) -> Vec<&FortranExtPassConfig> {
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
/// A Fortran MODULE compilation unit.
#[derive(Debug, Clone)]
pub struct FortranModule {
    pub name: String,
    /// Modules this module USEs.
    pub use_modules: Vec<String>,
    /// `IMPLICIT NONE` — always recommended.
    pub implicit_none: bool,
    /// Module-level variable declarations.
    pub module_vars: Vec<FortranDecl>,
    /// Derived types defined in this module.
    pub derived_types: Vec<FortranDerivedType>,
    /// Subprograms in the CONTAINS section.
    pub contains: Vec<FortranSubprogram>,
    pub doc: Option<String>,
}
impl FortranModule {
    pub fn new(name: impl Into<String>) -> Self {
        FortranModule {
            name: name.into(),
            use_modules: Vec::new(),
            implicit_none: true,
            module_vars: Vec::new(),
            derived_types: Vec::new(),
            contains: Vec::new(),
            doc: None,
        }
    }
}
/// Fortran INTENT attribute.
#[derive(Debug, Clone, PartialEq)]
pub enum FortranIntent {
    In,
    Out,
    InOut,
}
/// Pass execution phase for FortranExt.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FortranExtPassPhase {
    Early,
    Middle,
    Late,
    Finalize,
}
impl FortranExtPassPhase {
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
/// Analysis cache for FortranExt.
#[allow(dead_code)]
#[derive(Debug)]
pub struct FortranExtCache {
    pub(super) entries: Vec<(u64, Vec<u8>, bool, u32)>,
    pub(super) cap: usize,
    pub(super) total_hits: u64,
    pub(super) total_misses: u64,
}
impl FortranExtCache {
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
pub struct FortWorklist {
    pub(super) items: std::collections::VecDeque<u32>,
    pub(super) in_worklist: std::collections::HashSet<u32>,
}
impl FortWorklist {
    #[allow(dead_code)]
    pub fn new() -> Self {
        FortWorklist {
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
/// Statistics for FortranExt passes.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct FortranExtPassStats {
    pub iterations: usize,
    pub changed: bool,
    pub nodes_visited: usize,
    pub nodes_modified: usize,
    pub time_ms: u64,
    pub memory_bytes: usize,
    pub errors: usize,
}
impl FortranExtPassStats {
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
    pub fn merge(&mut self, o: &FortranExtPassStats) {
        self.iterations += o.iterations;
        self.changed |= o.changed;
        self.nodes_visited += o.nodes_visited;
        self.nodes_modified += o.nodes_modified;
        self.time_ms += o.time_ms;
        self.memory_bytes = self.memory_bytes.max(o.memory_bytes);
        self.errors += o.errors;
    }
}
/// Array dimension specification.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ArrayDimension {
    /// `DIMENSION(n)` — explicit size
    Explicit(usize),
    /// `DIMENSION(:)` — deferred (allocatable/pointer)
    Deferred,
    /// `DIMENSION(*)` — assumed-size
    Assumed,
    /// Multi-dimensional: `DIMENSION(n1, n2, ...)`
    Multi(Vec<usize>),
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FortCacheEntry {
    pub key: String,
    pub data: Vec<u8>,
    pub timestamp: u64,
    pub valid: bool,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FortDominatorTree {
    pub idom: Vec<Option<u32>>,
    pub dom_children: Vec<Vec<u32>>,
    pub dom_depth: Vec<u32>,
}
impl FortDominatorTree {
    #[allow(dead_code)]
    pub fn new(size: usize) -> Self {
        FortDominatorTree {
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
/// Fortran expression AST.
#[derive(Debug, Clone, PartialEq)]
pub enum FortranExpr {
    /// Literal value
    Lit(FortranLit),
    /// Variable reference
    Var(String),
    /// Array element access: `arr(i)`
    ArrayIndex(Box<FortranExpr>, Vec<FortranExpr>),
    /// Structure component: `obj%field`
    Component(Box<FortranExpr>, String),
    /// Binary operation: `left .OP. right`
    BinOp(Box<FortranExpr>, FortranBinOp, Box<FortranExpr>),
    /// Unary operation: `.NOT. expr` or `-expr`
    UnaryOp(FortranUnaryOp, Box<FortranExpr>),
    /// Function/intrinsic call: `FUNC(args)`
    Call(String, Vec<FortranExpr>),
    /// Array constructor: `[e1, e2, ...]`  (F2003 syntax)
    ArrayCtor(Vec<FortranExpr>),
    /// Implied DO: `(expr, var=lo, hi)` inside an array constructor
    ImpliedDo(Box<FortranExpr>, String, Box<FortranExpr>, Box<FortranExpr>),
    /// Type constructor: `TypeName(field1=v1, field2=v2)`
    TypeCtor(String, Vec<(String, FortranExpr)>),
    /// Conditional expression via `MERGE`: `MERGE(a, b, mask)`
    Merge(Box<FortranExpr>, Box<FortranExpr>, Box<FortranExpr>),
    /// Raw Fortran expression snippet
    Raw(String),
}
/// Fortran literal values.
#[derive(Debug, Clone, PartialEq)]
pub enum FortranLit {
    /// Integer literal: `42_8`
    Int(i64),
    /// Real literal: `3.14_8`
    Real(f64),
    /// Logical literal: `.TRUE.` / `.FALSE.`
    Logical(bool),
    /// Character literal: `'hello'`
    Char(String),
}
