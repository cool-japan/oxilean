//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::super::functions::*;
use super::impls1::*;
use super::impls2::*;
use std::collections::{HashMap, HashSet, VecDeque};

/// Dhall expression AST.
#[derive(Debug, Clone, PartialEq)]
pub enum DhallExpr {
    /// Boolean literal: `True` / `False`
    BoolLit(bool),
    /// Natural number literal: `0`, `42`
    NaturalLit(u64),
    /// Integer literal: `+1`, `-5`
    IntegerLit(i64),
    /// Double literal: `3.14`
    DoubleLit(f64),
    /// Text literal: `"hello, world"`
    TextLit(String),
    /// Text interpolation: `"prefix ${expr} suffix"`
    TextInterp(String, Box<DhallExpr>, String),
    /// List literal: `[ v1, v2, v3 ]`
    ListLit(Vec<DhallExpr>),
    /// Typed empty list: `[] : List Natural`
    EmptyList(DhallType),
    /// `Some value` (Optional constructor)
    Some(Box<DhallExpr>),
    /// `None T` (empty Optional)
    None(DhallType),
    /// Record value: `{ field1 = v1, field2 = v2 }`
    RecordLit(Box<DhallRecord>),
    /// Record type: `{ field1 : T1, field2 : T2 }`
    RecordType(Vec<(String, DhallType)>),
    /// Union value: `< Ctor : T | ... >.Ctor value`
    UnionLit {
        /// Union type
        union_type: DhallType,
        /// Chosen constructor
        ctor: String,
        /// Constructor argument (None for unit constructors)
        value: Option<Box<DhallExpr>>,
    },
    /// Lambda: `\(x : T) -> body`
    Lambda(Box<DhallFunction>),
    /// Dependent forall: `forall (x : T) -> body`
    Forall(String, DhallType, Box<DhallExpr>),
    /// Let binding: `let x : T = e in body`
    Let(Vec<DhallDecl>, Box<DhallExpr>),
    /// If-then-else: `if cond then t else f`
    If(Box<DhallExpr>, Box<DhallExpr>, Box<DhallExpr>),
    /// `merge handler union : T` — union elimination
    Merge(Box<DhallExpr>, Box<DhallExpr>, Option<DhallType>),
    /// `toMap record : List { mapKey : Text, mapValue : T }`
    ToMap(Box<DhallExpr>, Option<DhallType>),
    /// Equivalence assertion: `assert : a === b`
    Assert(Box<DhallExpr>, Box<DhallExpr>),
    /// Equivalence type: `a === b`
    Equivalent(Box<DhallExpr>, Box<DhallExpr>),
    /// Record update: `record // { field = value }`
    With(Box<DhallExpr>, Vec<(String, DhallExpr)>),
    /// Field selection: `record.field`
    Select(Box<DhallExpr>, String),
    /// Projection: `record.{ field1, field2 }`
    Project(Box<DhallExpr>, Vec<String>),
    /// Function application: `f x`
    Application(Box<DhallExpr>, Box<DhallExpr>),
    /// Variable: `x`, `Natural/show`, `List/map`
    Var(String),
    /// Import expression
    Import(DhallImport),
    /// Type ascription: `x : T`
    Annot(Box<DhallExpr>, DhallType),
    /// Boolean operators: `&&`, `||`
    BoolOp(String, Box<DhallExpr>, Box<DhallExpr>),
    /// Natural/Integer arithmetic: `+`, `*`
    NaturalOp(String, Box<DhallExpr>, Box<DhallExpr>),
    /// Text append: `x ++ y`
    TextAppend(Box<DhallExpr>, Box<DhallExpr>),
    /// List append: `xs # ys`
    ListAppend(Box<DhallExpr>, Box<DhallExpr>),
    /// Record merge (types): `T1 /\ T2`
    RecordTypeMerge(Box<DhallExpr>, Box<DhallExpr>),
    /// Record value merge: `r1 // r2`
    RecordMerge(Box<DhallExpr>, Box<DhallExpr>),
    /// `Type`, `Kind`, `Sort`
    Universe(DhallType),
    /// `Bool`, `Natural`, `Integer`, `Double`, `Text` as type expressions
    BuiltinType(DhallType),
}
impl DhallExpr {
    /// Emit this expression as a Dhall source string.
    pub fn emit(&self, indent: usize) -> String {
        let ind = " ".repeat(indent);
        let ind2 = " ".repeat(indent + 2);
        match self {
            DhallExpr::BoolLit(true) => "True".into(),
            DhallExpr::BoolLit(false) => "False".into(),
            DhallExpr::NaturalLit(n) => n.to_string(),
            DhallExpr::IntegerLit(n) => {
                if *n >= 0 {
                    format!("+{}", n)
                } else {
                    n.to_string()
                }
            }
            DhallExpr::DoubleLit(f) => {
                let s = format!("{}", f);
                if s.contains('.') || s.contains('e') {
                    s
                } else {
                    format!("{}.0", s)
                }
            }
            DhallExpr::TextLit(s) => format!("\"{}\"", escape_dhall_string(s)),
            DhallExpr::TextInterp(pre, expr, post) => {
                format!(
                    "\"{}${{{}}}{}\"",
                    escape_dhall_string(pre),
                    expr.emit(indent),
                    escape_dhall_string(post)
                )
            }
            DhallExpr::ListLit(items) => {
                if items.is_empty() {
                    return "[ ]".into();
                }
                let parts: Vec<String> = items.iter().map(|e| e.emit(indent + 2)).collect();
                format!(
                    "[\n{}{}\n{}]",
                    ind2,
                    parts.join(format!(",\n{}", ind2).as_str()),
                    ind
                )
            }
            DhallExpr::EmptyList(t) => format!("[] : List {}", t),
            DhallExpr::Some(e) => format!("Some {}", e.emit(indent)),
            DhallExpr::None(t) => format!("None {}", t),
            DhallExpr::RecordLit(r) => r.emit(indent),
            DhallExpr::RecordType(fields) => {
                if fields.is_empty() {
                    return "{}".into();
                }
                let parts: Vec<String> = fields
                    .iter()
                    .map(|(k, t)| format!("{}{} : {}", ind2, k, t))
                    .collect();
                format!("{{\n{}\n{}}}", parts.join(",\n"), ind)
            }
            DhallExpr::UnionLit {
                union_type,
                ctor,
                value,
            } => {
                let ut = union_type.to_string();
                match value {
                    None => format!("({}).{}", ut, ctor),
                    Some(v) => format!("({}).{} {}", ut, ctor, v.emit(indent)),
                }
            }
            DhallExpr::Lambda(func) => func.emit(indent),
            DhallExpr::Forall(x, t, body) => {
                format!("forall ({} : {}) -> {}", x, t, body.emit(indent))
            }
            DhallExpr::Let(decls, body) => {
                let mut out = String::new();
                for d in decls {
                    out.push_str(&format!("{}\n", d.emit(indent)));
                }
                out.push_str(&format!("in  {}", body.emit(indent)));
                out
            }
            DhallExpr::If(cond, t, f) => {
                format!(
                    "if {}\nthen {}{}\nelse {}{}",
                    cond.emit(indent),
                    ind2,
                    t.emit(indent + 2),
                    ind2,
                    f.emit(indent + 2),
                )
            }
            DhallExpr::Merge(handler, union, ty) => match ty {
                None => {
                    format!("merge {} {}", handler.emit(indent), union.emit(indent))
                }
                Some(t) => {
                    format!(
                        "merge {} {} : {}",
                        handler.emit(indent),
                        union.emit(indent),
                        t
                    )
                }
            },
            DhallExpr::ToMap(expr, ty) => match ty {
                None => format!("toMap {}", expr.emit(indent)),
                Some(t) => format!("toMap {} : {}", expr.emit(indent), t),
            },
            DhallExpr::Assert(lhs, rhs) => {
                format!("assert : {} === {}", lhs.emit(indent), rhs.emit(indent))
            }
            DhallExpr::Equivalent(lhs, rhs) => {
                format!("{} === {}", lhs.emit(indent), rhs.emit(indent))
            }
            DhallExpr::With(record, updates) => {
                let mut s = record.emit(indent);
                for (k, v) in updates {
                    s = format!("({} with {} = {})", s, k, v.emit(indent));
                }
                s
            }
            DhallExpr::Select(record, field) => {
                format!("{}.{}", record.emit(indent), field)
            }
            DhallExpr::Project(record, fields) => {
                format!("{}.{{ {} }}", record.emit(indent), fields.join(", "))
            }
            DhallExpr::Application(func, arg) => {
                let fs = func.emit(indent);
                let needs_parens = matches!(
                    arg.as_ref(),
                    DhallExpr::Application(_, _)
                        | DhallExpr::Lambda(_)
                        | DhallExpr::Forall(_, _, _)
                        | DhallExpr::Let(_, _)
                        | DhallExpr::If(_, _, _)
                        | DhallExpr::BoolOp(_, _, _)
                        | DhallExpr::NaturalOp(_, _, _)
                        | DhallExpr::TextAppend(_, _)
                        | DhallExpr::ListAppend(_, _)
                        | DhallExpr::RecordMerge(_, _)
                );
                if needs_parens {
                    format!("{} ({})", fs, arg.emit(indent))
                } else {
                    format!("{} {}", fs, arg.emit(indent))
                }
            }
            DhallExpr::Var(name) => name.clone(),
            DhallExpr::Import(imp) => imp.to_string(),
            DhallExpr::Annot(e, t) => format!("({} : {})", e.emit(indent), t),
            DhallExpr::BoolOp(op, lhs, rhs) => {
                format!("({} {} {})", lhs.emit(indent), op, rhs.emit(indent))
            }
            DhallExpr::NaturalOp(op, lhs, rhs) => {
                format!("({} {} {})", lhs.emit(indent), op, rhs.emit(indent))
            }
            DhallExpr::TextAppend(lhs, rhs) => {
                format!("({} ++ {})", lhs.emit(indent), rhs.emit(indent))
            }
            DhallExpr::ListAppend(lhs, rhs) => {
                format!("({} # {})", lhs.emit(indent), rhs.emit(indent))
            }
            DhallExpr::RecordTypeMerge(lhs, rhs) => {
                format!("({} /\\ {})", lhs.emit(indent), rhs.emit(indent))
            }
            DhallExpr::RecordMerge(lhs, rhs) => {
                format!("({} // {})", lhs.emit(indent), rhs.emit(indent))
            }
            DhallExpr::Universe(u) => u.to_string(),
            DhallExpr::BuiltinType(t) => t.to_string(),
        }
    }
}
/// Constant folding helper for DhallX2.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct DhallX2ConstFolder {
    pub(crate) folds: usize,
    pub(crate) failures: usize,
    pub(crate) enabled: bool,
}
impl DhallX2ConstFolder {
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
/// Pass registry for DhallX2.
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct DhallX2PassRegistry {
    pub(crate) configs: Vec<DhallX2PassConfig>,
    pub(crate) stats: Vec<DhallX2PassStats>,
}
impl DhallX2PassRegistry {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    #[allow(dead_code)]
    pub fn register(&mut self, c: DhallX2PassConfig) {
        self.stats.push(DhallX2PassStats::new());
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
    pub fn get(&self, i: usize) -> Option<&DhallX2PassConfig> {
        self.configs.get(i)
    }
    #[allow(dead_code)]
    pub fn get_stats(&self, i: usize) -> Option<&DhallX2PassStats> {
        self.stats.get(i)
    }
    #[allow(dead_code)]
    pub fn enabled_passes(&self) -> Vec<&DhallX2PassConfig> {
        self.configs.iter().filter(|c| c.enabled).collect()
    }
    #[allow(dead_code)]
    pub fn passes_in_phase(&self, ph: &DhallX2PassPhase) -> Vec<&DhallX2PassConfig> {
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
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DhallPassConfig {
    pub phase: DhallPassPhase,
    pub enabled: bool,
    pub max_iterations: u32,
    pub debug_output: bool,
    pub pass_name: String,
}
impl DhallPassConfig {
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>, phase: DhallPassPhase) -> Self {
        DhallPassConfig {
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
/// Dependency graph for DhallX2.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DhallX2DepGraph {
    pub(crate) n: usize,
    pub(crate) adj: Vec<Vec<usize>>,
    pub(crate) rev: Vec<Vec<usize>>,
    pub(crate) edge_count: usize,
}
impl DhallX2DepGraph {
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
pub struct DhallConstantFoldingHelper;
impl DhallConstantFoldingHelper {
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
pub struct DhallCacheEntry {
    pub key: String,
    pub data: Vec<u8>,
    pub timestamp: u64,
    pub valid: bool,
}
/// A top-level Dhall declaration (inside a `let` chain or file).
#[derive(Debug, Clone, PartialEq)]
pub struct DhallDecl {
    /// Bound name
    pub name: String,
    /// Optional type annotation
    pub ty: Option<DhallType>,
    /// Value expression
    pub value: DhallExpr,
}
impl DhallDecl {
    /// Create a declaration without a type annotation.
    pub fn new(name: impl Into<String>, value: DhallExpr) -> Self {
        DhallDecl {
            name: name.into(),
            ty: None,
            value,
        }
    }
    /// Create a declaration with a type annotation.
    pub fn typed(name: impl Into<String>, ty: DhallType, value: DhallExpr) -> Self {
        DhallDecl {
            name: name.into(),
            ty: Some(ty),
            value,
        }
    }
    pub(crate) fn emit(&self, indent: usize) -> String {
        match &self.ty {
            None => format!("let {} = {}", self.name, self.value.emit(indent)),
            Some(t) => format!("let {} : {} = {}", self.name, t, self.value.emit(indent)),
        }
    }
}
#[allow(dead_code)]
pub struct DhallPassRegistry {
    pub(crate) configs: Vec<DhallPassConfig>,
    pub(crate) stats: std::collections::HashMap<String, DhallPassStats>,
}
impl DhallPassRegistry {
    #[allow(dead_code)]
    pub fn new() -> Self {
        DhallPassRegistry {
            configs: Vec::new(),
            stats: std::collections::HashMap::new(),
        }
    }
    #[allow(dead_code)]
    pub fn register(&mut self, config: DhallPassConfig) {
        self.stats
            .insert(config.pass_name.clone(), DhallPassStats::new());
        self.configs.push(config);
    }
    #[allow(dead_code)]
    pub fn enabled_passes(&self) -> Vec<&DhallPassConfig> {
        self.configs.iter().filter(|c| c.enabled).collect()
    }
    #[allow(dead_code)]
    pub fn get_stats(&self, name: &str) -> Option<&DhallPassStats> {
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
/// Dominator tree for DhallX2.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DhallX2DomTree {
    pub(crate) idom: Vec<Option<usize>>,
    pub(crate) children: Vec<Vec<usize>>,
    pub(crate) depth: Vec<usize>,
}
impl DhallX2DomTree {
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
/// Worklist for DhallX2.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DhallX2Worklist {
    pub(crate) items: std::collections::VecDeque<usize>,
    pub(crate) present: Vec<bool>,
}
