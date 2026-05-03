//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::super::functions::*;
use super::defs::*;
use super::impls2::*;
use std::collections::{HashMap, HashSet, VecDeque};

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
    pub(crate) configs: Vec<AgdaExtPassConfig>,
    pub(crate) stats: Vec<AgdaExtPassStats>,
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
    pub(crate) folds: usize,
    pub(crate) failures: usize,
    pub(crate) enabled: bool,
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
    pub(crate) configs: Vec<AgdaPassConfig>,
    pub(crate) stats: std::collections::HashMap<String, AgdaPassStats>,
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
    pub(crate) n: usize,
    pub(crate) adj: Vec<Vec<usize>>,
    pub(crate) rev: Vec<Vec<usize>>,
    pub(crate) edge_count: usize,
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
    pub(crate) idom: Vec<Option<usize>>,
    pub(crate) children: Vec<Vec<usize>>,
    pub(crate) depth: Vec<usize>,
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
    pub(crate) fn emit_pi_dom(&self, indent: usize) -> String {
        match self {
            AgdaExpr::Pi(_, _, _) | AgdaExpr::Lambda(_, _) | AgdaExpr::Let(_, _, _) => {
                format!("({})", self.emit(indent))
            }
            _ => self.emit(indent),
        }
    }
    /// Emit in function position (left side of application).
    /// Application is left-associative, so `App` nodes do not need parens here.
    pub(crate) fn emit_func(&self, indent: usize) -> String {
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
    pub(crate) fn emit_atom(&self, indent: usize) -> String {
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
