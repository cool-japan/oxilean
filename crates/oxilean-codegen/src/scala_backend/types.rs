//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::functions::*;
use std::collections::{HashMap, HashSet, VecDeque};

/// A single parameter in a Scala method.
#[derive(Debug, Clone, PartialEq)]
pub struct ScalaParam {
    pub name: String,
    pub ty: ScalaType,
    pub default: Option<ScalaExpr>,
}
/// Statistics for ScalaExt passes.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct ScalaExtPassStats {
    pub iterations: usize,
    pub changed: bool,
    pub nodes_visited: usize,
    pub nodes_modified: usize,
    pub time_ms: u64,
    pub memory_bytes: usize,
    pub errors: usize,
}
impl ScalaExtPassStats {
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
    pub fn merge(&mut self, o: &ScalaExtPassStats) {
        self.iterations += o.iterations;
        self.changed |= o.changed;
        self.nodes_visited += o.nodes_visited;
        self.nodes_modified += o.nodes_modified;
        self.time_ms += o.time_ms;
        self.memory_bytes = self.memory_bytes.max(o.memory_bytes);
        self.errors += o.errors;
    }
}
/// Dependency graph for ScalaExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ScalaExtDepGraph {
    pub(super) n: usize,
    pub(super) adj: Vec<Vec<usize>>,
    pub(super) rev: Vec<Vec<usize>>,
    pub(super) edge_count: usize,
}
impl ScalaExtDepGraph {
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
pub struct ScalaConstantFoldingHelper;
impl ScalaConstantFoldingHelper {
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
/// A `catch` clause in a Scala try expression.
#[derive(Debug, Clone, PartialEq)]
pub struct ScalaCatch {
    pub pattern: ScalaPattern,
    pub body: ScalaExpr,
}
/// A Scala method definition.
#[derive(Debug, Clone, PartialEq)]
pub struct ScalaMethod {
    /// Method name
    pub name: String,
    /// Type parameters: `[A, B]`
    pub type_params: Vec<String>,
    /// Parameter lists (multiple for currying)
    pub params: Vec<Vec<ScalaParam>>,
    /// Return type
    pub return_type: ScalaType,
    /// Method body (None for abstract)
    pub body: Option<ScalaExpr>,
    /// Modifiers
    pub modifiers: Vec<ScalaModifier>,
}
/// Liveness analysis for ScalaExt.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct ScalaExtLiveness {
    pub live_in: Vec<Vec<usize>>,
    pub live_out: Vec<Vec<usize>>,
    pub defs: Vec<Vec<usize>>,
    pub uses: Vec<Vec<usize>>,
}
impl ScalaExtLiveness {
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
/// Scala expression AST.
#[derive(Debug, Clone, PartialEq)]
pub enum ScalaExpr {
    /// Literal value
    Lit(ScalaLit),
    /// Variable reference: `foo`
    Var(String),
    /// Method call / application: `f(a, b)` or `obj.method(args)`
    App(Box<ScalaExpr>, Vec<ScalaExpr>),
    /// Infix operator: `a + b`
    Infix(Box<ScalaExpr>, String, Box<ScalaExpr>),
    /// Prefix operator: `!x`, `-x`
    Prefix(String, Box<ScalaExpr>),
    /// If-else expression
    If(Box<ScalaExpr>, Box<ScalaExpr>, Box<ScalaExpr>),
    /// Match expression
    Match(Box<ScalaExpr>, Vec<ScalaCaseClause>),
    /// For comprehension / for-yield
    For(Vec<ScalaEnumerator>, Box<ScalaExpr>),
    /// Try-catch-finally
    Try(Box<ScalaExpr>, Vec<ScalaCatch>, Option<Box<ScalaExpr>>),
    /// Lambda: `x => body` or `(x, y) => body`
    Lambda(Vec<String>, Box<ScalaExpr>),
    /// Block: `{ stmts; expr }`
    Block(Vec<ScalaExpr>, Box<ScalaExpr>),
    /// `new ClassName(args)`
    New(String, Vec<ScalaExpr>),
    /// `this`
    This,
    /// `super`
    Super,
    /// Assignment: `x = expr`
    Assign(String, Box<ScalaExpr>),
    /// Type ascription: `expr: Type`
    TypeAnnotation(Box<ScalaExpr>, ScalaType),
    /// Throw expression: `throw new Exception(...)`
    Throw(Box<ScalaExpr>),
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ScalaDepGraph {
    pub(super) nodes: Vec<u32>,
    pub(super) edges: Vec<(u32, u32)>,
}
impl ScalaDepGraph {
    #[allow(dead_code)]
    pub fn new() -> Self {
        ScalaDepGraph {
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
/// Constant folding helper for ScalaExt.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct ScalaExtConstFolder {
    pub(super) folds: usize,
    pub(super) failures: usize,
    pub(super) enabled: bool,
}
impl ScalaExtConstFolder {
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
/// Analysis cache for ScalaExt.
#[allow(dead_code)]
#[derive(Debug)]
pub struct ScalaExtCache {
    pub(super) entries: Vec<(u64, Vec<u8>, bool, u32)>,
    pub(super) cap: usize,
    pub(super) total_hits: u64,
    pub(super) total_misses: u64,
}
impl ScalaExtCache {
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
pub struct ScalaLivenessInfo {
    pub live_in: Vec<std::collections::HashSet<u32>>,
    pub live_out: Vec<std::collections::HashSet<u32>>,
    pub defs: Vec<std::collections::HashSet<u32>>,
    pub uses: Vec<std::collections::HashSet<u32>>,
}
impl ScalaLivenessInfo {
    #[allow(dead_code)]
    pub fn new(block_count: usize) -> Self {
        ScalaLivenessInfo {
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
pub struct ScalaCacheEntry {
    pub key: String,
    pub data: Vec<u8>,
    pub timestamp: u64,
    pub valid: bool,
}
/// Worklist for ScalaExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ScalaExtWorklist {
    pub(super) items: std::collections::VecDeque<usize>,
    pub(super) present: Vec<bool>,
}
impl ScalaExtWorklist {
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
/// Pass execution phase for ScalaExt.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ScalaExtPassPhase {
    Early,
    Middle,
    Late,
    Finalize,
}
impl ScalaExtPassPhase {
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
pub struct ScalaPassRegistry {
    pub(super) configs: Vec<ScalaPassConfig>,
    pub(super) stats: std::collections::HashMap<String, ScalaPassStats>,
}
impl ScalaPassRegistry {
    #[allow(dead_code)]
    pub fn new() -> Self {
        ScalaPassRegistry {
            configs: Vec::new(),
            stats: std::collections::HashMap::new(),
        }
    }
    #[allow(dead_code)]
    pub fn register(&mut self, config: ScalaPassConfig) {
        self.stats
            .insert(config.pass_name.clone(), ScalaPassStats::new());
        self.configs.push(config);
    }
    #[allow(dead_code)]
    pub fn enabled_passes(&self) -> Vec<&ScalaPassConfig> {
        self.configs.iter().filter(|c| c.enabled).collect()
    }
    #[allow(dead_code)]
    pub fn get_stats(&self, name: &str) -> Option<&ScalaPassStats> {
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
/// A single arm in a Scala `match` expression.
#[derive(Debug, Clone, PartialEq)]
pub struct ScalaCaseClause {
    pub pattern: ScalaPattern,
    pub guard: Option<ScalaExpr>,
    pub body: ScalaExpr,
}
/// A complete Scala compilation unit (file / package object).
#[derive(Debug, Clone, PartialEq)]
pub struct ScalaModule {
    /// Package declaration: `com.example.mylib`
    pub package: Option<String>,
    /// Import declarations
    pub imports: Vec<ScalaImport>,
    /// Top-level declarations
    pub declarations: Vec<ScalaDecl>,
}
impl ScalaModule {
    /// Create a new empty module.
    pub fn new(package: Option<impl Into<String>>) -> Self {
        ScalaModule {
            package: package.map(|p| p.into()),
            imports: Vec::new(),
            declarations: Vec::new(),
        }
    }
    /// Add an import.
    pub fn add_import(&mut self, imp: ScalaImport) {
        self.imports.push(imp);
    }
    /// Add a top-level declaration.
    pub fn add_decl(&mut self, decl: ScalaDecl) {
        self.declarations.push(decl);
    }
    /// Emit the complete Scala 3 source for this module.
    pub fn emit(&self) -> String {
        let mut out = String::new();
        if let Some(pkg) = &self.package {
            out.push_str(&format!("package {}\n\n", pkg));
        }
        for imp in &self.imports {
            out.push_str(&format!("{}\n", imp));
        }
        if !self.imports.is_empty() {
            out.push('\n');
        }
        for decl in &self.declarations {
            out.push_str(&format!("{}\n\n", decl));
        }
        out
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct ScalaEnum {
    /// Enum name
    pub name: String,
    /// Type parameters
    pub type_params: Vec<String>,
    /// Enum cases
    pub cases: Vec<ScalaEnumCase>,
    /// Extends list (for ADTs)
    pub extends_list: Vec<String>,
}
/// Pass registry for ScalaExt.
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct ScalaExtPassRegistry {
    pub(super) configs: Vec<ScalaExtPassConfig>,
    pub(super) stats: Vec<ScalaExtPassStats>,
}
impl ScalaExtPassRegistry {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    #[allow(dead_code)]
    pub fn register(&mut self, c: ScalaExtPassConfig) {
        self.stats.push(ScalaExtPassStats::new());
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
    pub fn get(&self, i: usize) -> Option<&ScalaExtPassConfig> {
        self.configs.get(i)
    }
    #[allow(dead_code)]
    pub fn get_stats(&self, i: usize) -> Option<&ScalaExtPassStats> {
        self.stats.get(i)
    }
    #[allow(dead_code)]
    pub fn enabled_passes(&self) -> Vec<&ScalaExtPassConfig> {
        self.configs.iter().filter(|c| c.enabled).collect()
    }
    #[allow(dead_code)]
    pub fn passes_in_phase(&self, ph: &ScalaExtPassPhase) -> Vec<&ScalaExtPassConfig> {
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
/// Dominator tree for ScalaExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ScalaExtDomTree {
    pub(super) idom: Vec<Option<usize>>,
    pub(super) children: Vec<Vec<usize>>,
    pub(super) depth: Vec<usize>,
}
impl ScalaExtDomTree {
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
/// Scala literal values.
#[derive(Debug, Clone, PartialEq)]
pub enum ScalaLit {
    /// Integer literal: `42`
    Int(i64),
    /// Long literal: `42L`
    Long(i64),
    /// Double literal: `3.14`
    Double(f64),
    /// Float literal: `3.14f`
    Float(f32),
    /// Boolean literal: `true` / `false`
    Bool(bool),
    /// Character literal: `'a'`
    Char(char),
    /// String literal: `"hello"`
    Str(String),
    /// `null`
    Null,
    /// `()` / unit
    Unit,
}
/// Scala pattern AST for `match` expressions.
#[derive(Debug, Clone, PartialEq)]
pub enum ScalaPattern {
    /// `_` — wildcard
    Wildcard,
    /// Variable binding: `x`
    Var(String),
    /// Literal pattern: `42`, `"hello"`, `true`
    Lit(ScalaLit),
    /// Type pattern: `x: SomeType`
    Typed(String, ScalaType),
    /// Tuple pattern: `(a, b, c)`
    Tuple(Vec<ScalaPattern>),
    /// Extractor pattern: `Some(x)`, `Cons(h, t)`
    Extractor(String, Vec<ScalaPattern>),
    /// Alternative patterns: `1 | 2 | 3`
    Alt(Vec<ScalaPattern>),
}
/// Modifiers for a Scala method or field.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ScalaModifier {
    Private,
    Protected,
    Override,
    Final,
    Abstract,
    Implicit,
    Inline,
    Lazy,
    Given,
    Extension,
}
/// A Scala `case class` declaration.
///
/// Example: `case class Person(name: String, age: Int) extends Entity`
#[derive(Debug, Clone, PartialEq)]
pub struct ScalaCaseClass {
    /// Class name
    pub name: String,
    /// Type parameters: `A`, `B`
    pub type_params: Vec<String>,
    /// Constructor fields
    pub fields: Vec<ScalaParam>,
    /// Extends list
    pub extends_list: Vec<String>,
}
/// A Scala `object` declaration (companion or standalone).
#[derive(Debug, Clone, PartialEq)]
pub struct ScalaObject {
    /// Object name
    pub name: String,
    /// Extends list
    pub extends_list: Vec<String>,
    /// Methods
    pub methods: Vec<ScalaMethod>,
    /// Constants / val definitions: (name, type, expr)
    pub constants: Vec<(String, ScalaType, ScalaExpr)>,
}
/// A general Scala `class` declaration.
#[derive(Debug, Clone, PartialEq)]
pub struct ScalaClass {
    /// Class name
    pub name: String,
    /// Type parameters
    pub type_params: Vec<String>,
    /// Constructor parameters
    pub constructor_params: Vec<ScalaParam>,
    /// Extends list
    pub extends_list: Vec<String>,
    /// Methods
    pub methods: Vec<ScalaMethod>,
    /// Modifiers
    pub modifiers: Vec<ScalaModifier>,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ScalaWorklist {
    pub(super) items: std::collections::VecDeque<u32>,
    pub(super) in_worklist: std::collections::HashSet<u32>,
}
impl ScalaWorklist {
    #[allow(dead_code)]
    pub fn new() -> Self {
        ScalaWorklist {
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
/// An enumerator in a Scala for-comprehension.
#[derive(Debug, Clone, PartialEq)]
pub enum ScalaEnumerator {
    /// Generator: `x <- xs`
    Generator(String, ScalaExpr),
    /// Guard/filter: `if x > 0`
    Guard(ScalaExpr),
    /// Definition: `y = f(x)`
    Definition(String, ScalaExpr),
}
/// A Scala import declaration.
#[derive(Debug, Clone, PartialEq)]
pub struct ScalaImport {
    /// Import path: `scala.collection.mutable`
    pub path: String,
    /// Specific names or `_` for wildcard
    pub items: Vec<String>,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ScalaDominatorTree {
    pub idom: Vec<Option<u32>>,
    pub dom_children: Vec<Vec<u32>>,
    pub dom_depth: Vec<u32>,
}
impl ScalaDominatorTree {
    #[allow(dead_code)]
    pub fn new(size: usize) -> Self {
        ScalaDominatorTree {
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
#[derive(Debug, Clone, PartialEq)]
pub enum ScalaPassPhase {
    Analysis,
    Transformation,
    Verification,
    Cleanup,
}
impl ScalaPassPhase {
    #[allow(dead_code)]
    pub fn name(&self) -> &str {
        match self {
            ScalaPassPhase::Analysis => "analysis",
            ScalaPassPhase::Transformation => "transformation",
            ScalaPassPhase::Verification => "verification",
            ScalaPassPhase::Cleanup => "cleanup",
        }
    }
    #[allow(dead_code)]
    pub fn is_modifying(&self) -> bool {
        matches!(
            self,
            ScalaPassPhase::Transformation | ScalaPassPhase::Cleanup
        )
    }
}
/// The Scala code generation backend.
pub struct ScalaBackend {
    pub(super) module: ScalaModule,
}
impl ScalaBackend {
    /// Create a new backend for the given package.
    pub fn new(package: Option<impl Into<String>>) -> Self {
        let mut module = ScalaModule::new(package);
        module.add_import(ScalaImport {
            path: "scala.annotation".to_string(),
            items: vec!["tailrec".to_string()],
        });
        ScalaBackend { module }
    }
    /// Compile a single LCNF function declaration and add it to the module.
    pub fn compile_decl(&mut self, decl: &LcnfFunDecl) {
        let method = self.compile_fun(decl);
        self.module.add_decl(ScalaDecl::Method(method));
    }
    /// Compile an LCNF function to a Scala method.
    pub(super) fn compile_fun(&self, decl: &LcnfFunDecl) -> ScalaMethod {
        let params: Vec<ScalaParam> = decl
            .params
            .iter()
            .map(|p| ScalaParam {
                name: p.name.clone(),
                ty: lcnf_type_to_scala(&p.ty),
                default: None,
            })
            .collect();
        let body = self.compile_expr(&decl.body);
        let ret_type = lcnf_type_to_scala(&decl.ret_type);
        ScalaMethod {
            name: sanitize_scala_ident(&decl.name),
            type_params: Vec::new(),
            params: if params.is_empty() {
                Vec::new()
            } else {
                vec![params]
            },
            return_type: ret_type,
            body: Some(body),
            modifiers: Vec::new(),
        }
    }
    /// Compile an LCNF expression to a Scala expression.
    pub(super) fn compile_expr(&self, expr: &LcnfExpr) -> ScalaExpr {
        match expr {
            LcnfExpr::Return(arg) => self.compile_arg(arg),
            LcnfExpr::Let {
                name, value, body, ..
            } => {
                let rhs_expr = self.compile_let_value(value);
                let cont_expr = self.compile_expr(body);
                ScalaExpr::Block(
                    vec![ScalaExpr::Assign(name.clone(), Box::new(rhs_expr))],
                    Box::new(cont_expr),
                )
            }
            LcnfExpr::Case {
                scrutinee,
                alts,
                default,
                ..
            } => {
                let scrut = ScalaExpr::Var(format!("{}", scrutinee));
                let mut arms: Vec<ScalaCaseClause> =
                    alts.iter().map(|alt| self.compile_alt(alt)).collect();
                if let Some(def) = default {
                    let def_expr = self.compile_expr(def);
                    arms.push(ScalaCaseClause {
                        pattern: ScalaPattern::Wildcard,
                        guard: None,
                        body: def_expr,
                    });
                }
                ScalaExpr::Match(Box::new(scrut), arms)
            }
            LcnfExpr::TailCall(func, args) => {
                let func_expr = self.compile_arg(func);
                if args.is_empty() {
                    func_expr
                } else {
                    let arg_exprs: Vec<ScalaExpr> =
                        args.iter().map(|a| self.compile_arg(a)).collect();
                    ScalaExpr::App(Box::new(func_expr), arg_exprs)
                }
            }
            LcnfExpr::Unreachable => ScalaExpr::Throw(Box::new(ScalaExpr::New(
                "RuntimeException".to_string(),
                vec![ScalaExpr::Lit(ScalaLit::Str("unreachable".to_string()))],
            ))),
        }
    }
    /// Compile an LCNF let-value to a Scala expression.
    pub(super) fn compile_let_value(&self, val: &LcnfLetValue) -> ScalaExpr {
        match val {
            LcnfLetValue::App(func, args) => {
                let func_expr = self.compile_arg(func);
                if args.is_empty() {
                    func_expr
                } else {
                    let arg_exprs: Vec<ScalaExpr> =
                        args.iter().map(|a| self.compile_arg(a)).collect();
                    ScalaExpr::App(Box::new(func_expr), arg_exprs)
                }
            }
            LcnfLetValue::Ctor(name, _tag, args) => {
                let ctor_expr = ScalaExpr::Var(name.clone());
                if args.is_empty() {
                    ctor_expr
                } else {
                    let arg_exprs: Vec<ScalaExpr> =
                        args.iter().map(|a| self.compile_arg(a)).collect();
                    ScalaExpr::App(Box::new(ctor_expr), arg_exprs)
                }
            }
            LcnfLetValue::Proj(_name, idx, var) => {
                let field = format!("_{}", idx + 1);
                ScalaExpr::App(
                    Box::new(ScalaExpr::Var(format!("{}. {}", var, field))),
                    Vec::new(),
                )
            }
            LcnfLetValue::Lit(lit) => match lit {
                LcnfLit::Nat(n) => ScalaExpr::Lit(ScalaLit::Long(*n as i64)),
                LcnfLit::Str(s) => ScalaExpr::Lit(ScalaLit::Str(s.clone())),
            },
            LcnfLetValue::Erased | LcnfLetValue::Reset(_) => ScalaExpr::Lit(ScalaLit::Unit),
            LcnfLetValue::FVar(v) => ScalaExpr::Var(format!("{}", v)),
            LcnfLetValue::Reuse(_, name, _tag, args) => {
                let ctor_expr = ScalaExpr::Var(name.clone());
                if args.is_empty() {
                    ctor_expr
                } else {
                    let arg_exprs: Vec<ScalaExpr> =
                        args.iter().map(|a| self.compile_arg(a)).collect();
                    ScalaExpr::App(Box::new(ctor_expr), arg_exprs)
                }
            }
        }
    }
    /// Compile an LCNF case alternative to a Scala case clause.
    pub(super) fn compile_alt(&self, alt: &LcnfAlt) -> ScalaCaseClause {
        let body = self.compile_expr(&alt.body);
        let pat = ScalaPattern::Extractor(
            alt.ctor_name.clone(),
            alt.params
                .iter()
                .map(|p| ScalaPattern::Var(p.name.clone()))
                .collect(),
        );
        ScalaCaseClause {
            pattern: pat,
            guard: None,
            body,
        }
    }
    /// Compile an LCNF argument to a Scala expression.
    pub(super) fn compile_arg(&self, arg: &LcnfArg) -> ScalaExpr {
        match arg {
            LcnfArg::Var(v) => ScalaExpr::Var(format!("{}", v)),
            LcnfArg::Lit(lit) => match lit {
                LcnfLit::Nat(n) => ScalaExpr::Lit(ScalaLit::Long(*n as i64)),
                LcnfLit::Str(s) => ScalaExpr::Lit(ScalaLit::Str(s.clone())),
            },
            LcnfArg::Erased | LcnfArg::Type(_) => ScalaExpr::Lit(ScalaLit::Unit),
        }
    }
    /// Emit the complete Scala module source.
    pub fn emit_module(&self) -> String {
        self.module.emit()
    }
}
/// The various top-level declarations in a Scala compilation unit.
#[derive(Debug, Clone, PartialEq)]
pub enum ScalaDecl {
    CaseClass(ScalaCaseClass),
    Trait(ScalaTrait),
    Enum(ScalaEnum),
    Object(ScalaObject),
    Class(ScalaClass),
    Method(ScalaMethod),
    /// `val name: Type = expr` at top level
    Val(String, ScalaType, ScalaExpr),
    /// `type Name = Type` (alias)
    TypeAlias(String, Vec<String>, ScalaType),
    /// `opaque type Name = Type`
    OpaqueType(String, Vec<String>, ScalaType),
    /// `extension (x: Type) def method...`
    Extension(ScalaType, Vec<ScalaMethod>),
    /// `given name: Type with { ... }`
    Given(String, ScalaType, Vec<ScalaMethod>),
    Comment(String),
    RawLine(String),
}
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct ScalaPassStats {
    pub total_runs: u32,
    pub successful_runs: u32,
    pub total_changes: u64,
    pub time_ms: u64,
    pub iterations_used: u32,
}
impl ScalaPassStats {
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
/// A Scala `trait` declaration.
#[derive(Debug, Clone, PartialEq)]
pub struct ScalaTrait {
    /// Trait name
    pub name: String,
    /// Type parameters
    pub type_params: Vec<String>,
    /// Extends list
    pub extends_list: Vec<String>,
    /// Abstract method signatures
    pub abstract_methods: Vec<ScalaMethod>,
    /// Concrete method implementations
    pub concrete_methods: Vec<ScalaMethod>,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ScalaAnalysisCache {
    pub(super) entries: std::collections::HashMap<String, ScalaCacheEntry>,
    pub(super) max_size: usize,
    pub(super) hits: u64,
    pub(super) misses: u64,
}
impl ScalaAnalysisCache {
    #[allow(dead_code)]
    pub fn new(max_size: usize) -> Self {
        ScalaAnalysisCache {
            entries: std::collections::HashMap::new(),
            max_size,
            hits: 0,
            misses: 0,
        }
    }
    #[allow(dead_code)]
    pub fn get(&mut self, key: &str) -> Option<&ScalaCacheEntry> {
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
            ScalaCacheEntry {
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
/// A Scala 3 `enum` declaration.
///
/// Example:
/// ```text
/// enum Color:
///   case Red, Green, Blue
/// ```
///
/// Also supports parameterized cases:
/// ```text
/// enum Expr:
///   case Lit(n: Int)
///   case Add(l: Expr, r: Expr)
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct ScalaEnumCase {
    /// Case name: `Red`, `Lit`
    pub name: String,
    /// Fields (empty for simple cases)
    pub fields: Vec<ScalaParam>,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ScalaPassConfig {
    pub phase: ScalaPassPhase,
    pub enabled: bool,
    pub max_iterations: u32,
    pub debug_output: bool,
    pub pass_name: String,
}
impl ScalaPassConfig {
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>, phase: ScalaPassPhase) -> Self {
        ScalaPassConfig {
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
/// Configuration for ScalaExt passes.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ScalaExtPassConfig {
    pub name: String,
    pub phase: ScalaExtPassPhase,
    pub enabled: bool,
    pub max_iterations: usize,
    pub debug: u32,
    pub timeout_ms: Option<u64>,
}
impl ScalaExtPassConfig {
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            phase: ScalaExtPassPhase::Middle,
            enabled: true,
            max_iterations: 100,
            debug: 0,
            timeout_ms: None,
        }
    }
    #[allow(dead_code)]
    pub fn with_phase(mut self, phase: ScalaExtPassPhase) -> Self {
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
/// Scala type representation for type-directed code generation.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ScalaType {
    /// `Int` — 32-bit signed integer
    Int,
    /// `Long` — 64-bit signed integer
    Long,
    /// `Double` — 64-bit IEEE floating-point
    Double,
    /// `Float` — 32-bit IEEE floating-point
    Float,
    /// `Boolean` — boolean
    Boolean,
    /// `Char` — Unicode character
    Char,
    /// `String` — UTF-16 string
    ScalaString,
    /// `Unit` — unit type
    Unit,
    /// `Null` — null type (subtype of all AnyRef)
    Null,
    /// `Nothing` — bottom type
    Nothing,
    /// `Any` — top type
    Any,
    /// `AnyRef` — reference type root
    AnyRef,
    /// `AnyVal` — value type root
    AnyVal,
    /// `List[T]`
    List(Box<ScalaType>),
    /// `Option[T]`
    Option(Box<ScalaType>),
    /// `Either[A, B]`
    Either(Box<ScalaType>, Box<ScalaType>),
    /// `(A, B, ...)` — tuple
    Tuple(Vec<ScalaType>),
    /// `(A, B) => R` — function type
    Function(Vec<ScalaType>, Box<ScalaType>),
    /// Named type: class, trait, object
    Custom(String),
    /// Generic type application: `Map[K, V]`, `Future[T]`
    Generic(String, Vec<ScalaType>),
}
