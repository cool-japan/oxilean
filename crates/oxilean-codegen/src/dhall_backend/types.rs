//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
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
    pub(super) folds: usize,
    pub(super) failures: usize,
    pub(super) enabled: bool,
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
    pub(super) configs: Vec<DhallX2PassConfig>,
    pub(super) stats: Vec<DhallX2PassStats>,
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
    pub(super) n: usize,
    pub(super) adj: Vec<Vec<usize>>,
    pub(super) rev: Vec<Vec<usize>>,
    pub(super) edge_count: usize,
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
    pub(super) fn emit(&self, indent: usize) -> String {
        match &self.ty {
            None => format!("let {} = {}", self.name, self.value.emit(indent)),
            Some(t) => format!("let {} : {} = {}", self.name, t, self.value.emit(indent)),
        }
    }
}
#[allow(dead_code)]
pub struct DhallPassRegistry {
    pub(super) configs: Vec<DhallPassConfig>,
    pub(super) stats: std::collections::HashMap<String, DhallPassStats>,
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
    pub(super) idom: Vec<Option<usize>>,
    pub(super) children: Vec<Vec<usize>>,
    pub(super) depth: Vec<usize>,
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
    pub(super) items: std::collections::VecDeque<usize>,
    pub(super) present: Vec<bool>,
}
impl DhallX2Worklist {
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
pub struct DhallAnalysisCache {
    pub(super) entries: std::collections::HashMap<String, DhallCacheEntry>,
    pub(super) max_size: usize,
    pub(super) hits: u64,
    pub(super) misses: u64,
}
impl DhallAnalysisCache {
    #[allow(dead_code)]
    pub fn new(max_size: usize) -> Self {
        DhallAnalysisCache {
            entries: std::collections::HashMap::new(),
            max_size,
            hits: 0,
            misses: 0,
        }
    }
    #[allow(dead_code)]
    pub fn get(&mut self, key: &str) -> Option<&DhallCacheEntry> {
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
            DhallCacheEntry {
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
/// Liveness analysis for DhallX2.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct DhallX2Liveness {
    pub live_in: Vec<Vec<usize>>,
    pub live_out: Vec<Vec<usize>>,
    pub defs: Vec<Vec<usize>>,
    pub uses: Vec<Vec<usize>>,
}
impl DhallX2Liveness {
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
#[derive(Debug, Clone)]
pub struct DhallWorklist {
    pub(super) items: std::collections::VecDeque<u32>,
    pub(super) in_worklist: std::collections::HashSet<u32>,
}
impl DhallWorklist {
    #[allow(dead_code)]
    pub fn new() -> Self {
        DhallWorklist {
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
/// Statistics for DhallExt passes.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct DhallExtPassStats {
    pub iterations: usize,
    pub changed: bool,
    pub nodes_visited: usize,
    pub nodes_modified: usize,
    pub time_ms: u64,
    pub memory_bytes: usize,
    pub errors: usize,
}
impl DhallExtPassStats {
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
    pub fn merge(&mut self, o: &DhallExtPassStats) {
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
#[derive(Debug, Clone, Default)]
pub struct DhallPassStats {
    pub total_runs: u32,
    pub successful_runs: u32,
    pub total_changes: u64,
    pub time_ms: u64,
    pub iterations_used: u32,
}
impl DhallPassStats {
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
pub struct DhallLivenessInfo {
    pub live_in: Vec<std::collections::HashSet<u32>>,
    pub live_out: Vec<std::collections::HashSet<u32>>,
    pub defs: Vec<std::collections::HashSet<u32>>,
    pub uses: Vec<std::collections::HashSet<u32>>,
}
impl DhallLivenessInfo {
    #[allow(dead_code)]
    pub fn new(block_count: usize) -> Self {
        DhallLivenessInfo {
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
/// Dependency graph for DhallExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DhallExtDepGraph {
    pub(super) n: usize,
    pub(super) adj: Vec<Vec<usize>>,
    pub(super) rev: Vec<Vec<usize>>,
    pub(super) edge_count: usize,
}
impl DhallExtDepGraph {
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
/// A complete Dhall file.
///
/// Dhall files are structured as:
/// ```dhall
/// let import1 = ./lib.dhall
/// let decl1 : T = expr1
/// let decl2 = expr2
/// in  finalExpression
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct DhallFile {
    /// Import declarations at the top level
    pub imports: Vec<(String, DhallImport)>,
    /// Local declarations (let-bindings)
    pub declarations: Vec<DhallDecl>,
    /// The final expression the file evaluates to
    pub expression: DhallExpr,
}
impl DhallFile {
    /// Create a new Dhall file with only a final expression.
    pub fn new(expression: DhallExpr) -> Self {
        DhallFile {
            imports: vec![],
            declarations: vec![],
            expression,
        }
    }
    /// Add an import at the top.
    pub fn import(mut self, name: impl Into<String>, imp: DhallImport) -> Self {
        self.imports.push((name.into(), imp));
        self
    }
    /// Add a local declaration.
    pub fn declare(mut self, decl: DhallDecl) -> Self {
        self.declarations.push(decl);
        self
    }
    /// Emit the complete `.dhall` file contents.
    pub fn emit(&self) -> String {
        let mut out = String::from("-- Dhall configuration generated by OxiLean\n");
        for (name, imp) in &self.imports {
            out.push_str(&format!("let {} = {}\n", name, imp));
        }
        for decl in &self.declarations {
            out.push_str(&decl.emit(0));
            out.push('\n');
        }
        if !self.imports.is_empty() || !self.declarations.is_empty() {
            out.push_str("in  ");
        }
        out.push_str(&self.expression.emit(0));
        out.push('\n');
        out
    }
}
/// Constant folding helper for DhallExt.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct DhallExtConstFolder {
    pub(super) folds: usize,
    pub(super) failures: usize,
    pub(super) enabled: bool,
}
impl DhallExtConstFolder {
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
/// Dominator tree for DhallExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DhallExtDomTree {
    pub(super) idom: Vec<Option<usize>>,
    pub(super) children: Vec<Vec<usize>>,
    pub(super) depth: Vec<usize>,
}
impl DhallExtDomTree {
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
/// A Dhall import statement.
#[derive(Debug, Clone, PartialEq)]
pub enum DhallImport {
    /// Local file import: `./foo.dhall`
    Local(String),
    /// Remote URL import: `https://example.com/foo.dhall`
    Remote(String),
    /// Environment variable import: `env:HOME`
    Env(String),
    /// Missing import (always fails): `missing`
    Missing,
    /// Import with hash: `./foo.dhall sha256:abc123`
    Hashed(Box<DhallImport>, String),
    /// Import with fallback: `./foo.dhall ? ./bar.dhall`
    Fallback(Box<DhallImport>, Box<DhallImport>),
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DhallDepGraph {
    pub(super) nodes: Vec<u32>,
    pub(super) edges: Vec<(u32, u32)>,
}
impl DhallDepGraph {
    #[allow(dead_code)]
    pub fn new() -> Self {
        DhallDepGraph {
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
/// Worklist for DhallExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DhallExtWorklist {
    pub(super) items: std::collections::VecDeque<usize>,
    pub(super) present: Vec<bool>,
}
impl DhallExtWorklist {
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
/// Pass execution phase for DhallExt.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DhallExtPassPhase {
    Early,
    Middle,
    Late,
    Finalize,
}
impl DhallExtPassPhase {
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
/// Statistics for DhallX2 passes.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct DhallX2PassStats {
    pub iterations: usize,
    pub changed: bool,
    pub nodes_visited: usize,
    pub nodes_modified: usize,
    pub time_ms: u64,
    pub memory_bytes: usize,
    pub errors: usize,
}
impl DhallX2PassStats {
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
    pub fn merge(&mut self, o: &DhallX2PassStats) {
        self.iterations += o.iterations;
        self.changed |= o.changed;
        self.nodes_visited += o.nodes_visited;
        self.nodes_modified += o.nodes_modified;
        self.time_ms += o.time_ms;
        self.memory_bytes = self.memory_bytes.max(o.memory_bytes);
        self.errors += o.errors;
    }
}
/// A Dhall record value: `{ field1 = v1, field2 = v2 }`
#[derive(Debug, Clone, PartialEq)]
pub struct DhallRecord {
    /// Ordered list of (field name, value) pairs
    pub fields: Vec<(String, DhallExpr)>,
}
impl DhallRecord {
    /// Create a new empty record.
    pub fn new() -> Self {
        DhallRecord { fields: vec![] }
    }
    /// Add a field.
    pub fn field(mut self, name: impl Into<String>, value: DhallExpr) -> Self {
        self.fields.push((name.into(), value));
        self
    }
    pub(super) fn emit(&self, indent: usize) -> String {
        if self.fields.is_empty() {
            return "{=}".into();
        }
        let ind2 = " ".repeat(indent + 2);
        let ind = " ".repeat(indent);
        let parts: Vec<String> = self
            .fields
            .iter()
            .map(|(k, v)| format!("{}{} = {}", ind2, k, v.emit(indent + 2)))
            .collect();
        format!("{{\n{}\n{}}}", parts.join(",\n"), ind)
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum DhallPassPhase {
    Analysis,
    Transformation,
    Verification,
    Cleanup,
}
impl DhallPassPhase {
    #[allow(dead_code)]
    pub fn name(&self) -> &str {
        match self {
            DhallPassPhase::Analysis => "analysis",
            DhallPassPhase::Transformation => "transformation",
            DhallPassPhase::Verification => "verification",
            DhallPassPhase::Cleanup => "cleanup",
        }
    }
    #[allow(dead_code)]
    pub fn is_modifying(&self) -> bool {
        matches!(
            self,
            DhallPassPhase::Transformation | DhallPassPhase::Cleanup
        )
    }
}
/// Analysis cache for DhallX2.
#[allow(dead_code)]
#[derive(Debug)]
pub struct DhallX2Cache {
    pub(super) entries: Vec<(u64, Vec<u8>, bool, u32)>,
    pub(super) cap: usize,
    pub(super) total_hits: u64,
    pub(super) total_misses: u64,
}
impl DhallX2Cache {
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
/// The Dhall code generation backend.
///
/// Converts OxiLean IR constructs into Dhall configuration language output.
pub struct DhallBackend {
    /// Whether to emit types with full annotation (default true)
    pub emit_annotations: bool,
}
impl DhallBackend {
    /// Create a new DhallBackend with default settings.
    pub fn new() -> Self {
        DhallBackend {
            emit_annotations: true,
        }
    }
    /// Emit a DhallExpr to string.
    pub fn emit_expr(&self, expr: &DhallExpr, indent: usize) -> String {
        expr.emit(indent)
    }
    /// Emit a complete DhallFile to string.
    pub fn emit_file(&self, file: &DhallFile) -> String {
        file.emit()
    }
    /// Emit a DhallRecord to string.
    pub fn emit_record(&self, record: &DhallRecord, indent: usize) -> String {
        record.emit(indent)
    }
    /// Emit a DhallFunction to string.
    pub fn emit_function(&self, func: &DhallFunction, indent: usize) -> String {
        func.emit(indent)
    }
    /// Build a Dhall record schema (record type) from a field map.
    pub fn make_schema(&self, fields: Vec<(&str, DhallType)>) -> DhallExpr {
        DhallExpr::RecordType(
            fields
                .into_iter()
                .map(|(k, t)| (k.to_string(), t))
                .collect(),
        )
    }
    /// Build a `List/map` application.
    pub fn make_list_map(
        &self,
        input_type: DhallType,
        output_type: DhallType,
        func: DhallExpr,
        list: DhallExpr,
    ) -> DhallExpr {
        DhallExpr::Application(
            Box::new(DhallExpr::Application(
                Box::new(DhallExpr::Application(
                    Box::new(DhallExpr::Application(
                        Box::new(DhallExpr::Var("List/map".into())),
                        Box::new(DhallExpr::BuiltinType(input_type)),
                    )),
                    Box::new(DhallExpr::BuiltinType(output_type)),
                )),
                Box::new(func),
            )),
            Box::new(list),
        )
    }
    /// Build a `Natural/fold`-style loop body.
    #[allow(clippy::too_many_arguments)]
    pub fn make_natural_fold(
        &self,
        n: DhallExpr,
        result_type: DhallType,
        succ: DhallExpr,
        zero: DhallExpr,
    ) -> DhallExpr {
        DhallExpr::Application(
            Box::new(DhallExpr::Application(
                Box::new(DhallExpr::Application(
                    Box::new(DhallExpr::Application(
                        Box::new(DhallExpr::Var("Natural/fold".into())),
                        Box::new(n),
                    )),
                    Box::new(DhallExpr::BuiltinType(result_type)),
                )),
                Box::new(succ),
            )),
            Box::new(zero),
        )
    }
    /// Build a configuration record for a service-like schema.
    #[allow(clippy::too_many_arguments)]
    pub fn make_service_config(
        &self,
        enable: bool,
        name: &str,
        port: u64,
        extra_fields: Vec<(String, DhallExpr)>,
    ) -> DhallExpr {
        let mut fields = vec![
            ("enable".to_string(), DhallExpr::BoolLit(enable)),
            ("name".to_string(), DhallExpr::TextLit(name.to_string())),
            ("port".to_string(), DhallExpr::NaturalLit(port)),
        ];
        fields.extend(extra_fields);
        DhallExpr::RecordLit(Box::new(DhallRecord { fields }))
    }
    /// Build a union type for an enumeration.
    pub fn make_enum(&self, variants: Vec<&str>) -> DhallType {
        DhallType::Union(
            variants
                .into_iter()
                .map(|v| (v.to_string(), None))
                .collect(),
        )
    }
    /// Build an Optional handling pattern with `merge`.
    pub fn make_optional_merge(
        &self,
        optional_value: DhallExpr,
        some_handler: DhallExpr,
        none_value: DhallExpr,
        result_type: DhallType,
    ) -> DhallExpr {
        let handler = DhallExpr::RecordLit(Box::new(DhallRecord {
            fields: vec![
                ("Some".to_string(), some_handler),
                ("None".to_string(), none_value),
            ],
        }));
        DhallExpr::Merge(
            Box::new(handler),
            Box::new(optional_value),
            Some(result_type),
        )
    }
    /// Generate a Dhall prelude-style package.dhall skeleton.
    pub fn make_package(&self, _module_name: &str, exports: Vec<(&str, DhallExpr)>) -> DhallFile {
        let record = DhallExpr::RecordLit(Box::new(DhallRecord {
            fields: exports
                .into_iter()
                .map(|(k, v)| (k.to_string(), v))
                .collect(),
        }));
        DhallFile::new(record).declare(DhallDecl::new(
            "version",
            DhallExpr::TextLit("1.0.0".into()),
        ))
    }
}
/// Pass execution phase for DhallX2.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DhallX2PassPhase {
    Early,
    Middle,
    Late,
    Finalize,
}
impl DhallX2PassPhase {
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
pub struct DhallDominatorTree {
    pub idom: Vec<Option<u32>>,
    pub dom_children: Vec<Vec<u32>>,
    pub dom_depth: Vec<u32>,
}
impl DhallDominatorTree {
    #[allow(dead_code)]
    pub fn new(size: usize) -> Self {
        DhallDominatorTree {
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
/// Analysis cache for DhallExt.
#[allow(dead_code)]
#[derive(Debug)]
pub struct DhallExtCache {
    pub(super) entries: Vec<(u64, Vec<u8>, bool, u32)>,
    pub(super) cap: usize,
    pub(super) total_hits: u64,
    pub(super) total_misses: u64,
}
impl DhallExtCache {
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
/// Dhall type-level expressions (a subset of DhallExpr, named for clarity).
#[derive(Debug, Clone, PartialEq)]
pub enum DhallType {
    /// `Bool`
    Bool,
    /// `Natural`
    Natural,
    /// `Integer`
    Integer,
    /// `Double`
    Double,
    /// `Text`
    Text,
    /// `List T`
    List(Box<DhallType>),
    /// `Optional T`
    Optional(Box<DhallType>),
    /// Record type: `{ field1 : T1, field2 : T2 }`
    Record(Vec<(String, DhallType)>),
    /// Union type: `< Ctor1 : T1 | Ctor2 | Ctor3 : T3 >`
    Union(Vec<(String, Option<DhallType>)>),
    /// Function type: `T1 -> T2`
    Function(Box<DhallType>, Box<DhallType>),
    /// Dependent function type: `forall (x : T1) -> T2`
    Forall(String, Box<DhallType>, Box<DhallType>),
    /// `Type` universe
    Type,
    /// `Kind` universe (type of types)
    Kind,
    /// `Sort` universe (type of kinds)
    Sort,
    /// Named type reference: `Natural/show`, `MyRecord`
    Named(String),
}
/// Configuration for DhallExt passes.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DhallExtPassConfig {
    pub name: String,
    pub phase: DhallExtPassPhase,
    pub enabled: bool,
    pub max_iterations: usize,
    pub debug: u32,
    pub timeout_ms: Option<u64>,
}
impl DhallExtPassConfig {
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            phase: DhallExtPassPhase::Middle,
            enabled: true,
            max_iterations: 100,
            debug: 0,
            timeout_ms: None,
        }
    }
    #[allow(dead_code)]
    pub fn with_phase(mut self, phase: DhallExtPassPhase) -> Self {
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
/// Configuration for DhallX2 passes.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DhallX2PassConfig {
    pub name: String,
    pub phase: DhallX2PassPhase,
    pub enabled: bool,
    pub max_iterations: usize,
    pub debug: u32,
    pub timeout_ms: Option<u64>,
}
impl DhallX2PassConfig {
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            phase: DhallX2PassPhase::Middle,
            enabled: true,
            max_iterations: 100,
            debug: 0,
            timeout_ms: None,
        }
    }
    #[allow(dead_code)]
    pub fn with_phase(mut self, phase: DhallX2PassPhase) -> Self {
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
/// Liveness analysis for DhallExt.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct DhallExtLiveness {
    pub live_in: Vec<Vec<usize>>,
    pub live_out: Vec<Vec<usize>>,
    pub defs: Vec<Vec<usize>>,
    pub uses: Vec<Vec<usize>>,
}
impl DhallExtLiveness {
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
/// Pass registry for DhallExt.
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct DhallExtPassRegistry {
    pub(super) configs: Vec<DhallExtPassConfig>,
    pub(super) stats: Vec<DhallExtPassStats>,
}
impl DhallExtPassRegistry {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    #[allow(dead_code)]
    pub fn register(&mut self, c: DhallExtPassConfig) {
        self.stats.push(DhallExtPassStats::new());
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
    pub fn get(&self, i: usize) -> Option<&DhallExtPassConfig> {
        self.configs.get(i)
    }
    #[allow(dead_code)]
    pub fn get_stats(&self, i: usize) -> Option<&DhallExtPassStats> {
        self.stats.get(i)
    }
    #[allow(dead_code)]
    pub fn enabled_passes(&self) -> Vec<&DhallExtPassConfig> {
        self.configs.iter().filter(|c| c.enabled).collect()
    }
    #[allow(dead_code)]
    pub fn passes_in_phase(&self, ph: &DhallExtPassPhase) -> Vec<&DhallExtPassConfig> {
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
/// A Dhall function (lambda): `\(label : annotation) -> body`
#[derive(Debug, Clone, PartialEq)]
pub struct DhallFunction {
    /// Parameter label
    pub label: String,
    /// Parameter type annotation
    pub annotation: DhallType,
    /// Function body
    pub body: Box<DhallExpr>,
}
impl DhallFunction {
    /// Create a new function.
    pub fn new(label: impl Into<String>, annotation: DhallType, body: DhallExpr) -> Self {
        DhallFunction {
            label: label.into(),
            annotation,
            body: Box::new(body),
        }
    }
    pub(super) fn emit(&self, indent: usize) -> String {
        format!(
            r"\({} : {}) -> {}",
            self.label,
            self.annotation,
            self.body.emit(indent)
        )
    }
}
