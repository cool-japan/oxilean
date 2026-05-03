//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;
use std::collections::{HashMap, HashSet};

use super::super::functions::*;
use super::impls1::*;
use super::impls2::*;
use std::collections::VecDeque;

/// The escape status of a closure or variable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum EscapeInfo {
    /// The value does not escape its defining scope. It can be
    /// stack-allocated.
    NoEscape,
    /// The value escapes within the current function (e.g. stored in
    /// a local data structure) but does not escape the function itself.
    LocalEscape,
    /// The value escapes the function (e.g. returned, stored in a global,
    /// or passed to an unknown function). Must be heap-allocated.
    GlobalEscape,
}
impl EscapeInfo {
    /// Merge two escape infos, taking the more conservative (higher) one.
    pub fn merge(self, other: EscapeInfo) -> EscapeInfo {
        match (self, other) {
            (EscapeInfo::GlobalEscape, _) | (_, EscapeInfo::GlobalEscape) => {
                EscapeInfo::GlobalEscape
            }
            (EscapeInfo::LocalEscape, _) | (_, EscapeInfo::LocalEscape) => EscapeInfo::LocalEscape,
            _ => EscapeInfo::NoEscape,
        }
    }
    /// Whether heap allocation is required.
    pub fn requires_heap(self) -> bool {
        matches!(self, EscapeInfo::GlobalEscape)
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CCDominatorTree {
    pub idom: Vec<Option<u32>>,
    pub dom_children: Vec<Vec<u32>>,
    pub dom_depth: Vec<u32>,
}
impl CCDominatorTree {
    #[allow(dead_code)]
    pub fn new(size: usize) -> Self {
        CCDominatorTree {
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
pub struct CCAnalysisCache {
    pub(crate) entries: std::collections::HashMap<String, CCCacheEntry>,
    pub(crate) max_size: usize,
    pub(crate) hits: u64,
    pub(crate) misses: u64,
}
impl CCAnalysisCache {
    #[allow(dead_code)]
    pub fn new(max_size: usize) -> Self {
        CCAnalysisCache {
            entries: std::collections::HashMap::new(),
            max_size,
            hits: 0,
            misses: 0,
        }
    }
    #[allow(dead_code)]
    pub fn get(&mut self, key: &str) -> Option<&CCCacheEntry> {
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
            CCCacheEntry {
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
/// Dominator tree for CCExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CCExtDomTree {
    pub(crate) idom: Vec<Option<usize>>,
    pub(crate) children: Vec<Vec<usize>>,
    pub(crate) depth: Vec<usize>,
}
impl CCExtDomTree {
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
/// Performs escape analysis on an LCNF module.
pub struct EscapeAnalysis {
    /// Escape status for each variable.
    pub(crate) var_escape: HashMap<LcnfVarId, EscapeInfo>,
    /// Escape status for each function.
    pub(crate) fn_escape: HashMap<String, EscapeInfo>,
    /// Which variables are returned from functions.
    pub(crate) returned_vars: HashSet<LcnfVarId>,
    /// Which variables are passed to unknown functions.
    pub(crate) externally_passed: HashSet<LcnfVarId>,
    /// Which variables are stored in constructors.
    pub(crate) stored_in_ctor: HashSet<LcnfVarId>,
}
impl EscapeAnalysis {
    /// Create a new escape analysis.
    pub fn new() -> Self {
        EscapeAnalysis {
            var_escape: HashMap::new(),
            fn_escape: HashMap::new(),
            returned_vars: HashSet::new(),
            externally_passed: HashSet::new(),
            stored_in_ctor: HashSet::new(),
        }
    }
    /// Analyze an entire module.
    pub fn analyze(&mut self, module: &LcnfModule) -> HashMap<String, EscapeInfo> {
        for decl in &module.fun_decls {
            self.analyze_function(decl);
        }
        self.fn_escape.clone()
    }
    /// Analyze a single function declaration.
    pub(crate) fn analyze_function(&mut self, decl: &LcnfFunDecl) {
        for param in &decl.params {
            self.var_escape.insert(param.id, EscapeInfo::LocalEscape);
        }
        self.analyze_expr(&decl.body, false);
        let fn_escape = if self.returned_vars.iter().any(|v| {
            self.var_escape
                .get(v)
                .copied()
                .unwrap_or(EscapeInfo::NoEscape)
                == EscapeInfo::GlobalEscape
        }) {
            EscapeInfo::GlobalEscape
        } else {
            EscapeInfo::NoEscape
        };
        self.fn_escape.insert(decl.name.clone(), fn_escape);
    }
    /// Analyze an expression for escape behavior.
    pub(crate) fn analyze_expr(&mut self, expr: &LcnfExpr, _is_tail_position: bool) {
        match expr {
            LcnfExpr::Let {
                id, value, body, ..
            } => {
                let escape = self.analyze_let_value(value);
                self.var_escape.insert(*id, escape);
                self.analyze_expr(body, _is_tail_position);
            }
            LcnfExpr::Case {
                scrutinee,
                alts,
                default,
                ..
            } => {
                self.mark_read(*scrutinee);
                for alt in alts {
                    for param in &alt.params {
                        self.var_escape.insert(param.id, EscapeInfo::LocalEscape);
                    }
                    self.analyze_expr(&alt.body, _is_tail_position);
                }
                if let Some(def) = default {
                    self.analyze_expr(def, _is_tail_position);
                }
            }
            LcnfExpr::Return(arg) => {
                if let LcnfArg::Var(v) = arg {
                    self.returned_vars.insert(*v);
                    self.mark_escape(*v, EscapeInfo::GlobalEscape);
                }
            }
            LcnfExpr::TailCall(func, args) => {
                if let LcnfArg::Var(v) = func {
                    self.mark_escape(*v, EscapeInfo::GlobalEscape);
                }
                for arg in args {
                    if let LcnfArg::Var(v) = arg {
                        self.externally_passed.insert(*v);
                        self.mark_escape(*v, EscapeInfo::GlobalEscape);
                    }
                }
            }
            LcnfExpr::Unreachable => {}
        }
    }
    /// Analyze a let-value and return its escape status.
    pub(crate) fn analyze_let_value(&mut self, value: &LcnfLetValue) -> EscapeInfo {
        match value {
            LcnfLetValue::App(func, args) => {
                if let LcnfArg::Var(v) = func {
                    self.mark_read(*v);
                }
                for arg in args {
                    if let LcnfArg::Var(v) = arg {
                        self.externally_passed.insert(*v);
                        self.mark_escape(*v, EscapeInfo::GlobalEscape);
                    }
                }
                EscapeInfo::LocalEscape
            }
            LcnfLetValue::Ctor(_, _, args) => {
                for arg in args {
                    if let LcnfArg::Var(v) = arg {
                        self.stored_in_ctor.insert(*v);
                        self.mark_escape(*v, EscapeInfo::GlobalEscape);
                    }
                }
                EscapeInfo::LocalEscape
            }
            LcnfLetValue::Proj(_, _, var) => {
                self.mark_read(*var);
                EscapeInfo::LocalEscape
            }
            LcnfLetValue::FVar(var) => {
                self.mark_read(*var);
                self.var_escape
                    .get(var)
                    .copied()
                    .unwrap_or(EscapeInfo::NoEscape)
            }
            LcnfLetValue::Lit(_)
            | LcnfLetValue::Erased
            | LcnfLetValue::Reset(_)
            | LcnfLetValue::Reuse(_, _, _, _) => EscapeInfo::NoEscape,
        }
    }
    /// Mark a variable as read (used but not escaped).
    pub(crate) fn mark_read(&mut self, var: LcnfVarId) {
        self.var_escape.entry(var).or_insert(EscapeInfo::NoEscape);
    }
    /// Mark a variable with a specific escape level, merging with existing.
    pub(crate) fn mark_escape(&mut self, var: LcnfVarId, escape: EscapeInfo) {
        let current = self
            .var_escape
            .get(&var)
            .copied()
            .unwrap_or(EscapeInfo::NoEscape);
        self.var_escape.insert(var, current.merge(escape));
    }
    /// Get the escape status of a variable.
    pub fn get_var_escape(&self, var: LcnfVarId) -> EscapeInfo {
        self.var_escape
            .get(&var)
            .copied()
            .unwrap_or(EscapeInfo::NoEscape)
    }
    /// Get the escape status of a function.
    pub fn get_fn_escape(&self, name: &str) -> EscapeInfo {
        self.fn_escape
            .get(name)
            .copied()
            .unwrap_or(EscapeInfo::NoEscape)
    }
}
/// Constant folding helper for CCExt.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct CCExtConstFolder {
    pub(crate) folds: usize,
    pub(crate) failures: usize,
    pub(crate) enabled: bool,
}
impl CCExtConstFolder {
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
/// Constant folding helper for CCX2.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct CCX2ConstFolder {
    pub(crate) folds: usize,
    pub(crate) failures: usize,
    pub(crate) enabled: bool,
}
impl CCX2ConstFolder {
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
/// Statistics for CCExt passes.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct CCExtPassStats {
    pub iterations: usize,
    pub changed: bool,
    pub nodes_visited: usize,
    pub nodes_modified: usize,
    pub time_ms: u64,
    pub memory_bytes: usize,
    pub errors: usize,
}
impl CCExtPassStats {
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
    pub fn merge(&mut self, o: &CCExtPassStats) {
        self.iterations += o.iterations;
        self.changed |= o.changed;
        self.nodes_visited += o.nodes_visited;
        self.nodes_modified += o.nodes_modified;
        self.time_ms += o.time_ms;
        self.memory_bytes = self.memory_bytes.max(o.memory_bytes);
        self.errors += o.errors;
    }
}
/// Dependency graph for CCExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CCExtDepGraph {
    pub(crate) n: usize,
    pub(crate) adj: Vec<Vec<usize>>,
    pub(crate) rev: Vec<Vec<usize>>,
    pub(crate) edge_count: usize,
}
impl CCExtDepGraph {
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
