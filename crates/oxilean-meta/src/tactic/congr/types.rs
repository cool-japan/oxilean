//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;

/// State for a congruence closure computation.
#[allow(dead_code)]
pub struct CongrClosureState {
    /// Terms indexed by their id.
    pub terms: Vec<String>,
    /// Union-Find for term equivalence.
    pub uf: UnionFind,
    /// Function application pairs: (f_id, arg_id) -> result_id.
    pub apps: std::collections::HashMap<(usize, usize), usize>,
    /// Pending equalities to process.
    pub pending: Vec<(usize, usize)>,
}
#[allow(dead_code)]
impl CongrClosureState {
    pub fn new() -> Self {
        CongrClosureState {
            terms: Vec::new(),
            uf: UnionFind::new(0),
            apps: std::collections::HashMap::new(),
            pending: Vec::new(),
        }
    }
    pub fn add_term(&mut self, term: &str) -> usize {
        if let Some(pos) = self.terms.iter().position(|t| t == term) {
            return pos;
        }
        let id = self.terms.len();
        self.terms.push(term.to_string());
        self.uf.parent.push(id);
        self.uf.rank.push(0);
        self.uf.size += 1;
        id
    }
    pub fn merge(&mut self, a: &str, b: &str) {
        let id_a = self.add_term(a);
        let id_b = self.add_term(b);
        if !self.uf.are_equal(id_a, id_b) {
            self.pending.push((id_a, id_b));
            self.uf.union(id_a, id_b);
        }
    }
    pub fn are_congruent(&mut self, a: &str, b: &str) -> bool {
        let id_a = self.add_term(a);
        let id_b = self.add_term(b);
        self.uf.are_equal(id_a, id_b)
    }
    pub fn num_terms(&self) -> usize {
        self.terms.len()
    }
    pub fn num_pending(&self) -> usize {
        self.pending.len()
    }
    /// Ensure the union-find has capacity for index `idx`.
    fn ensure_capacity(&mut self, idx: usize) {
        while self.uf.size <= idx {
            let id = self.uf.size;
            self.uf.parent.push(id);
            self.uf.rank.push(0);
            self.uf.size += 1;
        }
    }
    pub fn add_eq(&mut self, a: usize, b: usize) {
        self.ensure_capacity(a.max(b));
        if !self.uf.are_equal(a, b) {
            self.pending.push((a, b));
            self.uf.union(a, b);
        }
    }
    pub fn are_equal(&mut self, a: usize, b: usize) -> bool {
        self.ensure_capacity(a.max(b));
        self.uf.are_equal(a, b)
    }
}
#[allow(dead_code)]
pub struct CongrExtPipeline600 {
    pub name: String,
    pub passes: Vec<CongrExtPass600>,
    pub run_count: usize,
}
impl CongrExtPipeline600 {
    #[allow(dead_code)]
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            passes: Vec::new(),
            run_count: 0,
        }
    }
    #[allow(dead_code)]
    pub fn add_pass(&mut self, pass: CongrExtPass600) {
        self.passes.push(pass);
    }
    #[allow(dead_code)]
    pub fn run_all(&mut self, input: &str) -> Vec<CongrExtResult600> {
        self.run_count += 1;
        self.passes
            .iter_mut()
            .filter(|p| p.enabled)
            .map(|p| p.run(input))
            .collect()
    }
    #[allow(dead_code)]
    pub fn num_passes(&self) -> usize {
        self.passes.len()
    }
    #[allow(dead_code)]
    pub fn num_enabled_passes(&self) -> usize {
        self.passes.iter().filter(|p| p.enabled).count()
    }
    #[allow(dead_code)]
    pub fn total_success_rate(&self) -> f64 {
        let total: usize = self.passes.iter().map(|p| p.total_runs).sum();
        let ok: usize = self.passes.iter().map(|p| p.successes).sum();
        if total == 0 {
            0.0
        } else {
            ok as f64 / total as f64
        }
    }
}
/// A congruence lemma record.
#[derive(Clone, Debug)]
pub struct CongrLemma {
    /// Name of the lemma.
    pub name: String,
    /// Number of arguments the function takes.
    pub arity: usize,
    /// The kind of congruence this lemma provides.
    pub congr_type: CongrType,
}
impl CongrLemma {
    /// Create a new `CongrLemma`.
    pub fn new(name: impl Into<String>, arity: usize, congr_type: CongrType) -> Self {
        CongrLemma {
            name: name.into(),
            arity,
            congr_type,
        }
    }
}
#[allow(dead_code)]
pub struct CongrExtConfig600 {
    pub(super) values: std::collections::HashMap<String, CongrExtConfigVal600>,
    pub(super) read_only: bool,
    pub(super) name: String,
}
impl CongrExtConfig600 {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            values: std::collections::HashMap::new(),
            read_only: false,
            name: String::new(),
        }
    }
    #[allow(dead_code)]
    pub fn named(name: &str) -> Self {
        Self {
            values: std::collections::HashMap::new(),
            read_only: false,
            name: name.to_string(),
        }
    }
    #[allow(dead_code)]
    pub fn set(&mut self, key: &str, value: CongrExtConfigVal600) -> bool {
        if self.read_only {
            return false;
        }
        self.values.insert(key.to_string(), value);
        true
    }
    #[allow(dead_code)]
    pub fn get(&self, key: &str) -> Option<&CongrExtConfigVal600> {
        self.values.get(key)
    }
    #[allow(dead_code)]
    pub fn get_bool(&self, key: &str) -> Option<bool> {
        self.get(key)?.as_bool()
    }
    #[allow(dead_code)]
    pub fn get_int(&self, key: &str) -> Option<i64> {
        self.get(key)?.as_int()
    }
    #[allow(dead_code)]
    pub fn get_str(&self, key: &str) -> Option<&str> {
        self.get(key)?.as_str()
    }
    #[allow(dead_code)]
    pub fn set_bool(&mut self, key: &str, v: bool) -> bool {
        self.set(key, CongrExtConfigVal600::Bool(v))
    }
    #[allow(dead_code)]
    pub fn set_int(&mut self, key: &str, v: i64) -> bool {
        self.set(key, CongrExtConfigVal600::Int(v))
    }
    #[allow(dead_code)]
    pub fn set_str(&mut self, key: &str, v: &str) -> bool {
        self.set(key, CongrExtConfigVal600::Str(v.to_string()))
    }
    #[allow(dead_code)]
    pub fn lock(&mut self) {
        self.read_only = true;
    }
    #[allow(dead_code)]
    pub fn unlock(&mut self) {
        self.read_only = false;
    }
    #[allow(dead_code)]
    pub fn size(&self) -> usize {
        self.values.len()
    }
    #[allow(dead_code)]
    pub fn has(&self, key: &str) -> bool {
        self.values.contains_key(key)
    }
    #[allow(dead_code)]
    pub fn remove(&mut self, key: &str) -> bool {
        self.values.remove(key).is_some()
    }
}
/// A result type for TacticCongr analysis.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum TacticCongrResult {
    Ok(String),
    Err(String),
    Partial { done: usize, total: usize },
    Skipped,
}
#[allow(dead_code)]
impl TacticCongrResult {
    pub fn is_ok(&self) -> bool {
        matches!(self, TacticCongrResult::Ok(_))
    }
    pub fn is_err(&self) -> bool {
        matches!(self, TacticCongrResult::Err(_))
    }
    pub fn is_partial(&self) -> bool {
        matches!(self, TacticCongrResult::Partial { .. })
    }
    pub fn is_skipped(&self) -> bool {
        matches!(self, TacticCongrResult::Skipped)
    }
    pub fn ok_msg(&self) -> Option<&str> {
        match self {
            TacticCongrResult::Ok(s) => Some(s),
            _ => None,
        }
    }
    pub fn err_msg(&self) -> Option<&str> {
        match self {
            TacticCongrResult::Err(s) => Some(s),
            _ => None,
        }
    }
    pub fn progress(&self) -> f64 {
        match self {
            TacticCongrResult::Ok(_) => 1.0,
            TacticCongrResult::Err(_) => 0.0,
            TacticCongrResult::Skipped => 0.0,
            TacticCongrResult::Partial { done, total } => {
                if *total == 0 {
                    0.0
                } else {
                    *done as f64 / *total as f64
                }
            }
        }
    }
}
/// A simple rewrite rule: lhs → rhs.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RewriteRule {
    pub name: String,
    pub lhs_pattern: SymExpr,
    pub rhs: SymExpr,
    pub is_conditional: bool,
}
#[allow(dead_code)]
impl RewriteRule {
    pub fn new(name: &str, lhs: SymExpr, rhs: SymExpr) -> Self {
        RewriteRule {
            name: name.to_string(),
            lhs_pattern: lhs,
            rhs,
            is_conditional: false,
        }
    }
    pub fn conditional(name: &str, lhs: SymExpr, rhs: SymExpr) -> Self {
        RewriteRule {
            name: name.to_string(),
            lhs_pattern: lhs,
            rhs,
            is_conditional: true,
        }
    }
    pub fn complexity(&self) -> usize {
        self.lhs_pattern.depth() + self.rhs.depth()
    }
}
/// A step in an equation-solving derivation.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DerivationStep {
    pub from: SymExpr,
    pub to: SymExpr,
    pub justification: String,
}
#[allow(dead_code)]
impl DerivationStep {
    pub fn new(from: SymExpr, to: SymExpr, just: &str) -> Self {
        DerivationStep {
            from,
            to,
            justification: just.to_string(),
        }
    }
    pub fn is_trivial(&self) -> bool {
        self.from == self.to
    }
}
/// A stored congruence lemma (v2, with symmetric/transitive flags).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CongrLemmaV2 {
    pub name: String,
    pub func_name: String,
    pub arity: usize,
    pub is_symmetric: bool,
    pub is_transitive: bool,
}
#[allow(dead_code)]
impl CongrLemmaV2 {
    pub fn new(name: &str, func: &str, arity: usize) -> Self {
        CongrLemmaV2 {
            name: name.to_string(),
            func_name: func.to_string(),
            arity,
            is_symmetric: false,
            is_transitive: false,
        }
    }
    pub fn symmetric(mut self) -> Self {
        self.is_symmetric = true;
        self
    }
    pub fn transitive(mut self) -> Self {
        self.is_transitive = true;
        self
    }
    pub fn is_equivalence(&self) -> bool {
        self.is_symmetric && self.is_transitive
    }
}
/// Union-Find (disjoint set) data structure for congruence closure.
#[allow(dead_code)]
pub struct UnionFind {
    pub(super) parent: Vec<usize>,
    pub(super) rank: Vec<usize>,
    pub(super) size: usize,
}
#[allow(dead_code)]
impl UnionFind {
    pub fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            rank: vec![0; n],
            size: n,
        }
    }
    pub fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }
    pub fn union(&mut self, x: usize, y: usize) -> bool {
        let rx = self.find(x);
        let ry = self.find(y);
        if rx == ry {
            return false;
        }
        if self.rank[rx] < self.rank[ry] {
            self.parent[rx] = ry;
        } else if self.rank[rx] > self.rank[ry] {
            self.parent[ry] = rx;
        } else {
            self.parent[ry] = rx;
            self.rank[rx] += 1;
        }
        true
    }
    pub fn are_equal(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }
    pub fn num_components(&mut self) -> usize {
        let mut roots = std::collections::HashSet::new();
        for i in 0..self.size {
            roots.insert(self.find(i));
        }
        roots.len()
    }
}
/// Database of congruence lemmas.
#[allow(dead_code)]
pub struct CongrLemmaDb {
    pub lemmas: Vec<CongrLemmaV2>,
}
#[allow(dead_code)]
impl CongrLemmaDb {
    pub fn new() -> Self {
        CongrLemmaDb { lemmas: Vec::new() }
    }
    pub fn register(&mut self, lemma: CongrLemmaV2) {
        self.lemmas.push(lemma);
    }
    pub fn lookup_by_func(&self, func: &str) -> Vec<&CongrLemmaV2> {
        self.lemmas.iter().filter(|l| l.func_name == func).collect()
    }
    pub fn num_lemmas(&self) -> usize {
        self.lemmas.len()
    }
    pub fn lookup_by_name(&self, name: &str) -> Option<&CongrLemmaV2> {
        self.lemmas.iter().find(|l| l.name == name)
    }
}
/// An analysis pass for TacticCongr.
#[allow(dead_code)]
pub struct TacticCongrAnalysisPass {
    pub name: String,
    pub enabled: bool,
    pub results: Vec<TacticCongrResult>,
    pub total_runs: usize,
}
#[allow(dead_code)]
impl TacticCongrAnalysisPass {
    pub fn new(name: &str) -> Self {
        TacticCongrAnalysisPass {
            name: name.to_string(),
            enabled: true,
            results: Vec::new(),
            total_runs: 0,
        }
    }
    pub fn run(&mut self, input: &str) -> TacticCongrResult {
        self.total_runs += 1;
        let result = if input.is_empty() {
            TacticCongrResult::Err("empty input".to_string())
        } else {
            TacticCongrResult::Ok(format!("processed: {}", input))
        };
        self.results.push(result.clone());
        result
    }
    pub fn success_count(&self) -> usize {
        self.results.iter().filter(|r| r.is_ok()).count()
    }
    pub fn error_count(&self) -> usize {
        self.results.iter().filter(|r| r.is_err()).count()
    }
    pub fn success_rate(&self) -> f64 {
        if self.total_runs == 0 {
            0.0
        } else {
            self.success_count() as f64 / self.total_runs as f64
        }
    }
    pub fn disable(&mut self) {
        self.enabled = false;
    }
    pub fn enable(&mut self) {
        self.enabled = true;
    }
    pub fn clear_results(&mut self) {
        self.results.clear();
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum CongrExtResult600 {
    /// Operation completed successfully.
    Ok(String),
    /// Operation encountered an error.
    Err(String),
    /// Operation partially completed.
    Partial { done: usize, total: usize },
    /// Operation was skipped.
    Skipped,
}
impl CongrExtResult600 {
    #[allow(dead_code)]
    pub fn is_ok(&self) -> bool {
        matches!(self, CongrExtResult600::Ok(_))
    }
    #[allow(dead_code)]
    pub fn is_err(&self) -> bool {
        matches!(self, CongrExtResult600::Err(_))
    }
    #[allow(dead_code)]
    pub fn is_partial(&self) -> bool {
        matches!(self, CongrExtResult600::Partial { .. })
    }
    #[allow(dead_code)]
    pub fn is_skipped(&self) -> bool {
        matches!(self, CongrExtResult600::Skipped)
    }
    #[allow(dead_code)]
    pub fn ok_msg(&self) -> Option<&str> {
        if let CongrExtResult600::Ok(s) = self {
            Some(s)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn err_msg(&self) -> Option<&str> {
        if let CongrExtResult600::Err(s) = self {
            Some(s)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn progress(&self) -> f64 {
        match self {
            CongrExtResult600::Ok(_) => 1.0,
            CongrExtResult600::Err(_) => 0.0,
            CongrExtResult600::Partial { done, total } => {
                if *total == 0 {
                    0.0
                } else {
                    *done as f64 / *total as f64
                }
            }
            CongrExtResult600::Skipped => 0.5,
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum CongrExtConfigVal600 {
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    List(Vec<String>),
}
impl CongrExtConfigVal600 {
    #[allow(dead_code)]
    pub fn as_bool(&self) -> Option<bool> {
        if let CongrExtConfigVal600::Bool(b) = self {
            Some(*b)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_int(&self) -> Option<i64> {
        if let CongrExtConfigVal600::Int(i) = self {
            Some(*i)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_float(&self) -> Option<f64> {
        if let CongrExtConfigVal600::Float(f) = self {
            Some(*f)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_str(&self) -> Option<&str> {
        if let CongrExtConfigVal600::Str(s) = self {
            Some(s)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_list(&self) -> Option<&[String]> {
        if let CongrExtConfigVal600::List(l) = self {
            Some(l)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn type_name(&self) -> &'static str {
        match self {
            CongrExtConfigVal600::Bool(_) => "bool",
            CongrExtConfigVal600::Int(_) => "int",
            CongrExtConfigVal600::Float(_) => "float",
            CongrExtConfigVal600::Str(_) => "str",
            CongrExtConfigVal600::List(_) => "list",
        }
    }
}
/// Node in an E-graph.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ENode {
    Symbol(String),
    Application { func: usize, args: Vec<usize> },
    Literal(i64),
}
/// A diagnostic reporter for TacticCongr.
#[allow(dead_code)]
pub struct TacticCongrDiagnostics {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub notes: Vec<String>,
    pub max_errors: usize,
}
#[allow(dead_code)]
impl TacticCongrDiagnostics {
    pub fn new(max_errors: usize) -> Self {
        TacticCongrDiagnostics {
            errors: Vec::new(),
            warnings: Vec::new(),
            notes: Vec::new(),
            max_errors,
        }
    }
    pub fn error(&mut self, msg: &str) {
        if self.errors.len() < self.max_errors {
            self.errors.push(msg.to_string());
        }
    }
    pub fn warning(&mut self, msg: &str) {
        self.warnings.push(msg.to_string());
    }
    pub fn note(&mut self, msg: &str) {
        self.notes.push(msg.to_string());
    }
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }
    pub fn num_errors(&self) -> usize {
        self.errors.len()
    }
    pub fn num_warnings(&self) -> usize {
        self.warnings.len()
    }
    pub fn is_clean(&self) -> bool {
        self.errors.is_empty() && self.warnings.is_empty()
    }
    pub fn at_error_limit(&self) -> bool {
        self.errors.len() >= self.max_errors
    }
    pub fn clear(&mut self) {
        self.errors.clear();
        self.warnings.clear();
        self.notes.clear();
    }
    pub fn summary(&self) -> String {
        format!(
            "{} error(s), {} warning(s)",
            self.errors.len(),
            self.warnings.len()
        )
    }
}
/// The kind of a congruence lemma.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CongrType {
    /// Default congruence (function application congruence).
    Default,
    /// Simp-style congruence (used by `simp` for rewrites under binders).
    Simp,
    /// Iff congruence (for propositional equivalences).
    Iff,
    /// Eq congruence (for definitional equality).
    Eq,
}
#[allow(dead_code)]
pub struct CongrExtPass600 {
    pub name: String,
    pub total_runs: usize,
    pub successes: usize,
    pub errors: usize,
    pub enabled: bool,
    pub results: Vec<CongrExtResult600>,
}
impl CongrExtPass600 {
    #[allow(dead_code)]
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            total_runs: 0,
            successes: 0,
            errors: 0,
            enabled: true,
            results: Vec::new(),
        }
    }
    #[allow(dead_code)]
    pub fn run(&mut self, input: &str) -> CongrExtResult600 {
        if !self.enabled {
            return CongrExtResult600::Skipped;
        }
        self.total_runs += 1;
        let result = if input.is_empty() {
            self.errors += 1;
            CongrExtResult600::Err(format!("empty input in pass '{}'", self.name))
        } else {
            self.successes += 1;
            CongrExtResult600::Ok(format!(
                "processed {} chars in pass '{}'",
                input.len(),
                self.name
            ))
        };
        self.results.push(result.clone());
        result
    }
    #[allow(dead_code)]
    pub fn success_count(&self) -> usize {
        self.successes
    }
    #[allow(dead_code)]
    pub fn error_count(&self) -> usize {
        self.errors
    }
    #[allow(dead_code)]
    pub fn success_rate(&self) -> f64 {
        if self.total_runs == 0 {
            0.0
        } else {
            self.successes as f64 / self.total_runs as f64
        }
    }
    #[allow(dead_code)]
    pub fn disable(&mut self) {
        self.enabled = false;
    }
    #[allow(dead_code)]
    pub fn enable(&mut self) {
        self.enabled = true;
    }
    #[allow(dead_code)]
    pub fn clear_results(&mut self) {
        self.results.clear();
    }
}
/// A symbolic expression for congruence closure.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SymExpr {
    Var(String),
    Const(i64),
    Add(Box<SymExpr>, Box<SymExpr>),
    Mul(Box<SymExpr>, Box<SymExpr>),
    Neg(Box<SymExpr>),
    App(String, Vec<SymExpr>),
}
#[allow(dead_code, clippy::should_implement_trait)]
impl SymExpr {
    pub fn var(name: &str) -> Self {
        SymExpr::Var(name.to_string())
    }
    pub fn konst(v: i64) -> Self {
        SymExpr::Const(v)
    }
    pub fn add(a: SymExpr, b: SymExpr) -> Self {
        SymExpr::Add(Box::new(a), Box::new(b))
    }
    pub fn mul(a: SymExpr, b: SymExpr) -> Self {
        SymExpr::Mul(Box::new(a), Box::new(b))
    }
    pub fn neg(a: SymExpr) -> Self {
        SymExpr::Neg(Box::new(a))
    }
    pub fn depth(&self) -> usize {
        match self {
            SymExpr::Var(_) | SymExpr::Const(_) => 0,
            SymExpr::Neg(x) => 1 + x.depth(),
            SymExpr::Add(a, b) | SymExpr::Mul(a, b) => 1 + a.depth().max(b.depth()),
            SymExpr::App(_, args) => 1 + args.iter().map(|a| a.depth()).max().unwrap_or(0),
        }
    }
    pub fn eval(&self, env: &std::collections::HashMap<String, i64>) -> Option<i64> {
        match self {
            SymExpr::Var(v) => env.get(v).copied(),
            SymExpr::Const(c) => Some(*c),
            SymExpr::Neg(x) => x.eval(env).map(|v| -v),
            SymExpr::Add(a, b) => Some(a.eval(env)? + b.eval(env)?),
            SymExpr::Mul(a, b) => Some(a.eval(env)? * b.eval(env)?),
            SymExpr::App(_, _) => None,
        }
    }
    pub fn free_vars(&self) -> std::collections::HashSet<String> {
        match self {
            SymExpr::Var(v) => {
                let mut s = std::collections::HashSet::new();
                s.insert(v.clone());
                s
            }
            SymExpr::Const(_) => Default::default(),
            SymExpr::Neg(x) => x.free_vars(),
            SymExpr::Add(a, b) | SymExpr::Mul(a, b) => {
                let mut s = a.free_vars();
                s.extend(b.free_vars());
                s
            }
            SymExpr::App(_, args) => args.iter().flat_map(|a| a.free_vars()).collect(),
        }
    }
    pub fn is_constant(&self) -> bool {
        matches!(self, SymExpr::Const(_))
    }
    pub fn is_linear(&self) -> bool {
        self.depth() <= 1
    }
}
/// A proof justification for a congruence step.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum CongrJust {
    Refl,
    Hyp(String),
    Cong {
        func: String,
        arg_proofs: Vec<CongrJust>,
    },
    Trans(Box<CongrJust>, Box<CongrJust>),
    Sym(Box<CongrJust>),
}
#[allow(dead_code)]
impl CongrJust {
    pub fn refl() -> Self {
        CongrJust::Refl
    }
    pub fn hyp(name: &str) -> Self {
        CongrJust::Hyp(name.to_string())
    }
    pub fn cong(func: &str, proofs: Vec<CongrJust>) -> Self {
        CongrJust::Cong {
            func: func.to_string(),
            arg_proofs: proofs,
        }
    }
    pub fn trans(p1: CongrJust, p2: CongrJust) -> Self {
        CongrJust::Trans(Box::new(p1), Box::new(p2))
    }
    pub fn sym(p: CongrJust) -> Self {
        CongrJust::Sym(Box::new(p))
    }
    pub fn is_trivial(&self) -> bool {
        matches!(self, CongrJust::Refl)
    }
    pub fn just_depth(&self) -> usize {
        match self {
            CongrJust::Refl | CongrJust::Hyp(_) => 0,
            CongrJust::Sym(p) => 1 + p.just_depth(),
            CongrJust::Trans(a, b) => 1 + a.just_depth().max(b.just_depth()),
            CongrJust::Cong { arg_proofs, .. } => {
                1 + arg_proofs.iter().map(|p| p.just_depth()).max().unwrap_or(0)
            }
        }
    }
    pub fn just_steps(&self) -> usize {
        match self {
            CongrJust::Refl => 0,
            CongrJust::Hyp(_) => 1,
            CongrJust::Sym(p) => p.just_steps(),
            CongrJust::Trans(a, b) => a.just_steps() + b.just_steps(),
            CongrJust::Cong { arg_proofs, .. } => {
                arg_proofs.iter().map(|p| p.just_steps()).sum::<usize>() + 1
            }
        }
    }
}
/// A derivation sequence.
#[allow(dead_code)]
pub struct Derivation {
    pub steps: Vec<DerivationStep>,
    pub start: SymExpr,
    pub end: SymExpr,
}
#[allow(dead_code)]
impl Derivation {
    pub fn new(start: SymExpr) -> Self {
        let end = start.clone();
        Derivation {
            steps: Vec::new(),
            start,
            end,
        }
    }
    pub fn add_step(&mut self, to: SymExpr, just: &str) {
        let from = self.end.clone();
        self.steps.push(DerivationStep::new(from, to.clone(), just));
        self.end = to;
    }
    pub fn length(&self) -> usize {
        self.steps.len()
    }
    pub fn is_valid(&self) -> bool {
        if self.steps.is_empty() {
            return self.start == self.end;
        }
        let first = &self.steps[0].from;
        let last = &self.steps[self.steps.len() - 1].to;
        first == &self.start && last == &self.end
    }
}
#[allow(dead_code)]
pub struct CongrExtDiff600 {
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub unchanged: Vec<String>,
}
impl CongrExtDiff600 {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            added: Vec::new(),
            removed: Vec::new(),
            unchanged: Vec::new(),
        }
    }
    #[allow(dead_code)]
    pub fn add(&mut self, s: &str) {
        self.added.push(s.to_string());
    }
    #[allow(dead_code)]
    pub fn remove(&mut self, s: &str) {
        self.removed.push(s.to_string());
    }
    #[allow(dead_code)]
    pub fn keep(&mut self, s: &str) {
        self.unchanged.push(s.to_string());
    }
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.added.is_empty() && self.removed.is_empty()
    }
    #[allow(dead_code)]
    pub fn total_changes(&self) -> usize {
        self.added.len() + self.removed.len()
    }
    #[allow(dead_code)]
    pub fn net_additions(&self) -> i64 {
        self.added.len() as i64 - self.removed.len() as i64
    }
    #[allow(dead_code)]
    pub fn summary(&self) -> String {
        format!(
            "+{} -{} =={}",
            self.added.len(),
            self.removed.len(),
            self.unchanged.len()
        )
    }
}
/// A diff for TacticCongr analysis results.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct TacticCongrDiff {
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub unchanged: Vec<String>,
}
#[allow(dead_code)]
impl TacticCongrDiff {
    pub fn new() -> Self {
        TacticCongrDiff {
            added: Vec::new(),
            removed: Vec::new(),
            unchanged: Vec::new(),
        }
    }
    pub fn add(&mut self, s: &str) {
        self.added.push(s.to_string());
    }
    pub fn remove(&mut self, s: &str) {
        self.removed.push(s.to_string());
    }
    pub fn keep(&mut self, s: &str) {
        self.unchanged.push(s.to_string());
    }
    pub fn is_empty(&self) -> bool {
        self.added.is_empty() && self.removed.is_empty()
    }
    pub fn total_changes(&self) -> usize {
        self.added.len() + self.removed.len()
    }
    pub fn net_additions(&self) -> i64 {
        self.added.len() as i64 - self.removed.len() as i64
    }
    pub fn summary(&self) -> String {
        format!(
            "+{} -{} =={}",
            self.added.len(),
            self.removed.len(),
            self.unchanged.len()
        )
    }
}
/// An E-class: a set of equivalent E-nodes.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct EClass {
    pub id: usize,
    pub nodes: Vec<ENode>,
}
/// A system of equations for congruence closure.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct EquationSystem {
    pub equations: Vec<(SymExpr, SymExpr, String)>,
    pub goal: Option<(SymExpr, SymExpr)>,
}
#[allow(dead_code)]
impl EquationSystem {
    pub fn new() -> Self {
        EquationSystem {
            equations: Vec::new(),
            goal: None,
        }
    }
    pub fn add_eq(&mut self, lhs: SymExpr, rhs: SymExpr, label: &str) {
        self.equations.push((lhs, rhs, label.to_string()));
    }
    pub fn set_goal(&mut self, lhs: SymExpr, rhs: SymExpr) {
        self.goal = Some((lhs, rhs));
    }
    pub fn num_equations(&self) -> usize {
        self.equations.len()
    }
    pub fn has_trivial_equation(&self) -> bool {
        self.equations.iter().any(|(l, r, _)| l == r)
    }
    pub fn normalize_equations(&mut self) {
        self.equations.retain(|(l, r, _)| l != r);
    }
}
#[allow(dead_code)]
pub struct CongrExtDiag600 {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub notes: Vec<String>,
    pub max_errors: usize,
}
impl CongrExtDiag600 {
    #[allow(dead_code)]
    pub fn new(max_errors: usize) -> Self {
        Self {
            errors: Vec::new(),
            warnings: Vec::new(),
            notes: Vec::new(),
            max_errors,
        }
    }
    #[allow(dead_code)]
    pub fn error(&mut self, msg: &str) {
        if self.errors.len() < self.max_errors {
            self.errors.push(msg.to_string());
        }
    }
    #[allow(dead_code)]
    pub fn warning(&mut self, msg: &str) {
        self.warnings.push(msg.to_string());
    }
    #[allow(dead_code)]
    pub fn note(&mut self, msg: &str) {
        self.notes.push(msg.to_string());
    }
    #[allow(dead_code)]
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }
    #[allow(dead_code)]
    pub fn num_errors(&self) -> usize {
        self.errors.len()
    }
    #[allow(dead_code)]
    pub fn num_warnings(&self) -> usize {
        self.warnings.len()
    }
    #[allow(dead_code)]
    pub fn is_clean(&self) -> bool {
        self.errors.is_empty() && self.warnings.is_empty()
    }
    #[allow(dead_code)]
    pub fn at_error_limit(&self) -> bool {
        self.errors.len() >= self.max_errors
    }
    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.errors.clear();
        self.warnings.clear();
        self.notes.clear();
    }
    #[allow(dead_code)]
    pub fn summary(&self) -> String {
        format!(
            "{} error(s), {} warning(s)",
            self.errors.len(),
            self.warnings.len()
        )
    }
}
/// A simple E-graph for congruence closure.
#[allow(dead_code)]
pub struct EGraph {
    pub classes: Vec<EClass>,
    pub node_to_class: std::collections::HashMap<ENode, usize>,
    pub union_find_eg: Vec<usize>,
    pub num_merges: usize,
}
#[allow(dead_code)]
impl EGraph {
    pub fn new() -> Self {
        EGraph {
            classes: Vec::new(),
            node_to_class: std::collections::HashMap::new(),
            union_find_eg: Vec::new(),
            num_merges: 0,
        }
    }
    pub fn add_node(&mut self, node: ENode) -> usize {
        if let Some(&id) = self.node_to_class.get(&node) {
            return self.eg_find(id);
        }
        let id = self.classes.len();
        self.classes.push(EClass {
            id,
            nodes: vec![node.clone()],
        });
        self.union_find_eg.push(id);
        self.node_to_class.insert(node, id);
        id
    }
    pub fn eg_find(&mut self, x: usize) -> usize {
        if self.union_find_eg[x] != x {
            let root = self.eg_find(self.union_find_eg[x]);
            self.union_find_eg[x] = root;
        }
        self.union_find_eg[x]
    }
    pub fn eg_union(&mut self, x: usize, y: usize) -> bool {
        let rx = self.eg_find(x);
        let ry = self.eg_find(y);
        if rx == ry {
            return false;
        }
        self.union_find_eg[rx] = ry;
        let nodes = self.classes[rx].nodes.clone();
        self.classes[ry].nodes.extend(nodes);
        self.num_merges += 1;
        true
    }
    pub fn eg_are_equal(&mut self, x: usize, y: usize) -> bool {
        self.eg_find(x) == self.eg_find(y)
    }
    pub fn eg_num_classes(&self) -> usize {
        let mut roots: std::collections::HashSet<usize> = std::collections::HashSet::new();
        let uf = self.union_find_eg.clone();
        for i in 0..uf.len() {
            let mut r = i;
            while uf[r] != r {
                r = uf[r];
            }
            roots.insert(r);
        }
        roots.len()
    }
    pub fn eg_size(&self) -> usize {
        self.classes.len()
    }
}
/// A typed slot for TacticCongr configuration.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum TacticCongrConfigValue {
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    List(Vec<String>),
}
#[allow(dead_code)]
impl TacticCongrConfigValue {
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            TacticCongrConfigValue::Bool(b) => Some(*b),
            _ => None,
        }
    }
    pub fn as_int(&self) -> Option<i64> {
        match self {
            TacticCongrConfigValue::Int(i) => Some(*i),
            _ => None,
        }
    }
    pub fn as_float(&self) -> Option<f64> {
        match self {
            TacticCongrConfigValue::Float(f) => Some(*f),
            _ => None,
        }
    }
    pub fn as_str(&self) -> Option<&str> {
        match self {
            TacticCongrConfigValue::Str(s) => Some(s),
            _ => None,
        }
    }
    pub fn as_list(&self) -> Option<&[String]> {
        match self {
            TacticCongrConfigValue::List(v) => Some(v),
            _ => None,
        }
    }
    pub fn type_name(&self) -> &'static str {
        match self {
            TacticCongrConfigValue::Bool(_) => "bool",
            TacticCongrConfigValue::Int(_) => "int",
            TacticCongrConfigValue::Float(_) => "float",
            TacticCongrConfigValue::Str(_) => "str",
            TacticCongrConfigValue::List(_) => "list",
        }
    }
}
/// Result of running equality saturation.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SaturationResult {
    pub iterations: usize,
    pub nodes_added: usize,
    pub merges_performed: usize,
    pub saturated: bool,
    pub goal_proved: Option<bool>,
}
#[allow(dead_code)]
impl SaturationResult {
    pub fn new() -> Self {
        SaturationResult {
            iterations: 0,
            nodes_added: 0,
            merges_performed: 0,
            saturated: false,
            goal_proved: None,
        }
    }
    pub fn is_success(&self) -> bool {
        self.goal_proved == Some(true)
    }
}
/// State maintained by the congruence closure algorithm.
#[derive(Clone, Debug)]
pub struct CongrState {
    /// Registered congruence lemmas.
    pub lemmas: Vec<CongrLemma>,
    /// Current recursion depth.
    pub depth: usize,
}
impl CongrState {
    /// Create a new empty `CongrState`.
    pub fn new() -> Self {
        CongrState {
            lemmas: Vec::new(),
            depth: 0,
        }
    }
    /// Add a lemma to the state.
    pub fn add_lemma(&mut self, lemma: CongrLemma) {
        self.lemmas.push(lemma);
    }
    /// Find the first lemma whose name matches `target`.
    pub fn find_matching(&self, target: &str) -> Option<&CongrLemma> {
        self.lemmas.iter().find(|l| l.name == target)
    }
}
/// A configuration store for TacticCongr.
#[allow(dead_code)]
pub struct TacticCongrConfig {
    pub values: std::collections::HashMap<String, TacticCongrConfigValue>,
    pub read_only: bool,
}
#[allow(dead_code)]
impl TacticCongrConfig {
    pub fn new() -> Self {
        TacticCongrConfig {
            values: std::collections::HashMap::new(),
            read_only: false,
        }
    }
    pub fn set(&mut self, key: &str, value: TacticCongrConfigValue) -> bool {
        if self.read_only {
            return false;
        }
        self.values.insert(key.to_string(), value);
        true
    }
    pub fn get(&self, key: &str) -> Option<&TacticCongrConfigValue> {
        self.values.get(key)
    }
    pub fn get_bool(&self, key: &str) -> Option<bool> {
        self.get(key)?.as_bool()
    }
    pub fn get_int(&self, key: &str) -> Option<i64> {
        self.get(key)?.as_int()
    }
    pub fn get_str(&self, key: &str) -> Option<&str> {
        self.get(key)?.as_str()
    }
    pub fn set_bool(&mut self, key: &str, v: bool) -> bool {
        self.set(key, TacticCongrConfigValue::Bool(v))
    }
    pub fn set_int(&mut self, key: &str, v: i64) -> bool {
        self.set(key, TacticCongrConfigValue::Int(v))
    }
    pub fn set_str(&mut self, key: &str, v: &str) -> bool {
        self.set(key, TacticCongrConfigValue::Str(v.to_string()))
    }
    pub fn lock(&mut self) {
        self.read_only = true;
    }
    pub fn unlock(&mut self) {
        self.read_only = false;
    }
    pub fn size(&self) -> usize {
        self.values.len()
    }
    pub fn has(&self, key: &str) -> bool {
        self.values.contains_key(key)
    }
    pub fn remove(&mut self, key: &str) -> bool {
        self.values.remove(key).is_some()
    }
}
/// A pattern variable (wildcards in rewrite rules).
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum Pattern {
    Wildcard,
    PVar(String),
    PSym(String),
    PApp(Box<Pattern>, Vec<Pattern>),
    PLit(i64),
}
#[allow(dead_code)]
impl Pattern {
    pub fn is_ground_pat(&self) -> bool {
        match self {
            Pattern::Wildcard | Pattern::PVar(_) => false,
            Pattern::PSym(_) | Pattern::PLit(_) => true,
            Pattern::PApp(f, args) => f.is_ground_pat() && args.iter().all(|a| a.is_ground_pat()),
        }
    }
    pub fn pat_depth(&self) -> usize {
        match self {
            Pattern::Wildcard | Pattern::PVar(_) | Pattern::PSym(_) | Pattern::PLit(_) => 0,
            Pattern::PApp(f, args) => {
                1 + std::cmp::max(
                    f.pat_depth(),
                    args.iter().map(|a| a.pat_depth()).max().unwrap_or(0),
                )
            }
        }
    }
    pub fn pat_variables(&self) -> Vec<String> {
        match self {
            Pattern::PVar(s) => vec![s.clone()],
            Pattern::PApp(f, args) => {
                let mut vars = f.pat_variables();
                for a in args {
                    vars.extend(a.pat_variables());
                }
                vars.sort();
                vars.dedup();
                vars
            }
            _ => vec![],
        }
    }
}
/// Theory-aware congruence closure supporting AC (associative-commutative) theories.
#[allow(dead_code)]
pub struct TheoryCongrState {
    pub uf: UnionFind,
    pub ac_ops: std::collections::HashSet<String>,
    pub assoc_ops: std::collections::HashSet<String>,
    pub comm_ops: std::collections::HashSet<String>,
}
#[allow(dead_code)]
impl TheoryCongrState {
    pub fn new() -> Self {
        let mut ac = std::collections::HashSet::new();
        ac.insert("add".to_string());
        ac.insert("mul".to_string());
        TheoryCongrState {
            uf: UnionFind::new(256),
            ac_ops: ac.clone(),
            assoc_ops: ac.clone(),
            comm_ops: ac,
        }
    }
    pub fn is_ac(&self, op: &str) -> bool {
        self.ac_ops.contains(op)
    }
    pub fn is_assoc(&self, op: &str) -> bool {
        self.assoc_ops.contains(op)
    }
    pub fn is_comm(&self, op: &str) -> bool {
        self.comm_ops.contains(op)
    }
    pub fn add_ac_op(&mut self, op: &str) {
        self.ac_ops.insert(op.to_string());
        self.assoc_ops.insert(op.to_string());
        self.comm_ops.insert(op.to_string());
    }
    /// Flatten an AC operator: `add(add(a,b),c)` → `[a, b, c]`.
    pub fn flatten_ac<'a>(&self, op: &str, expr: &'a SymExpr) -> Vec<&'a SymExpr> {
        match expr {
            SymExpr::App(fname, args) if args.len() == 2 && fname == op => {
                let mut v = self.flatten_ac(op, &args[0]);
                v.extend(self.flatten_ac(op, &args[1]));
                v
            }
            _ => vec![expr],
        }
    }
    pub fn theory_ac_equal(&self, op: &str, a: &SymExpr, b: &SymExpr) -> bool {
        if !self.is_ac(op) {
            return a == b;
        }
        let mut fa = self.flatten_ac(op, a);
        let mut fb = self.flatten_ac(op, b);
        fa.sort_by_key(|e| format!("{:?}", e));
        fb.sort_by_key(|e| format!("{:?}", e));
        fa == fb
    }
}
/// Configuration for the congr tactic.
#[derive(Clone, Debug)]
pub struct CongrConfig {
    /// Maximum recursion depth for congruence closure.
    pub max_depth: usize,
    /// Whether to use local hypotheses as congruence lemmas.
    pub use_hyps: bool,
}
/// A pipeline of TacticCongr analysis passes.
#[allow(dead_code)]
pub struct TacticCongrPipeline {
    pub passes: Vec<TacticCongrAnalysisPass>,
    pub name: String,
    pub total_inputs_processed: usize,
}
#[allow(dead_code)]
impl TacticCongrPipeline {
    pub fn new(name: &str) -> Self {
        TacticCongrPipeline {
            passes: Vec::new(),
            name: name.to_string(),
            total_inputs_processed: 0,
        }
    }
    pub fn add_pass(&mut self, pass: TacticCongrAnalysisPass) {
        self.passes.push(pass);
    }
    pub fn run_all(&mut self, input: &str) -> Vec<TacticCongrResult> {
        self.total_inputs_processed += 1;
        self.passes
            .iter_mut()
            .filter(|p| p.enabled)
            .map(|p| p.run(input))
            .collect()
    }
    pub fn num_passes(&self) -> usize {
        self.passes.len()
    }
    pub fn num_enabled_passes(&self) -> usize {
        self.passes.iter().filter(|p| p.enabled).count()
    }
    pub fn total_success_rate(&self) -> f64 {
        if self.passes.is_empty() {
            0.0
        } else {
            let total_rate: f64 = self.passes.iter().map(|p| p.success_rate()).sum();
            total_rate / self.passes.len() as f64
        }
    }
}
/// The `ext` (function extensionality) tactic.
#[derive(Clone, Debug)]
pub struct ExtTactic {
    /// Variable name to use for the extensionality argument.
    pub(super) var_name: String,
}
impl ExtTactic {
    /// Create a new `ExtTactic`.
    pub fn new() -> Self {
        ExtTactic {
            var_name: "x".to_string(),
        }
    }
    /// Apply function extensionality to a goal string.
    ///
    /// For `f = g` produces `f x = g x` where `x` is fresh.
    pub fn apply(&self, goal: &str) -> Option<String> {
        let trimmed = goal.trim();
        if let Some(pos) = find_eq_at_depth0(trimmed) {
            let lhs = trimmed[..pos].trim();
            let rhs = trimmed[pos + 3..].trim();
            Some(format!(
                "{} {} = {} {}",
                lhs, self.var_name, rhs, self.var_name
            ))
        } else {
            None
        }
    }
}
/// A collection of rewrite rules.
#[allow(dead_code)]
pub struct RewriteSystem {
    pub rules: Vec<RewriteRule>,
}
#[allow(dead_code)]
impl RewriteSystem {
    pub fn new() -> Self {
        RewriteSystem { rules: Vec::new() }
    }
    pub fn add_rule(&mut self, rule: RewriteRule) {
        self.rules.push(rule);
    }
    pub fn num_rules(&self) -> usize {
        self.rules.len()
    }
    pub fn find_applicable(&self, expr: &SymExpr) -> Vec<&RewriteRule> {
        self.rules
            .iter()
            .filter(|r| sym_matches(&r.lhs_pattern, expr))
            .collect()
    }
    pub fn total_complexity(&self) -> usize {
        self.rules.iter().map(|r| r.complexity()).sum()
    }
}
/// The `congr` tactic: prove equalities by congruence closure.
#[derive(Clone, Debug)]
pub struct CongrTactic {
    pub(super) config: CongrConfig,
}
impl CongrTactic {
    /// Create a new `CongrTactic` with default configuration.
    pub fn new() -> Self {
        CongrTactic {
            config: CongrConfig::default(),
        }
    }
    /// Create a new `CongrTactic` with a custom configuration.
    pub fn with_config(config: CongrConfig) -> Self {
        CongrTactic { config }
    }
    /// Apply congruence to a goal, returning sub-goals as strings.
    ///
    /// For a goal `f a = f b`, produces `[a = b]`.
    /// For a goal `f a b = f c d`, produces `[a = c, b = d]`.
    pub fn apply_congr(&self, state: &mut CongrState, target: &str) -> Vec<String> {
        if state.depth >= self.config.max_depth {
            return vec![];
        }
        state.depth += 1;
        let subgoals = self.congr_1(target);
        state.depth -= 1;
        subgoals
    }
    /// Apply function extensionality to a goal of the form `f = g`,
    /// producing `∀ x, f x = g x`.
    pub fn apply_ext(&self, _state: &mut CongrState, target: &str) -> Option<String> {
        if let Some(pos) = target.find(" = ") {
            let lhs = target[..pos].trim();
            let rhs = target[pos + 3..].trim();
            if !lhs.contains(' ') && !rhs.contains(' ') {
                return Some(format!("∀ x, {} x = {} x", lhs, rhs));
            }
        }
        None
    }
    /// One-step congruence: decompose a goal into argument sub-goals.
    ///
    /// For `f a₁ a₂ ... = f b₁ b₂ ...` produces `[a₁ = b₁, a₂ = b₂, ...]`.
    pub fn congr_1(&self, target: &str) -> Vec<String> {
        let eq_pos = match find_eq_at_depth0(target) {
            Some(p) => p,
            None => return vec![],
        };
        let lhs = target[..eq_pos].trim();
        let rhs = target[eq_pos + 3..].trim();
        let lhs_parts = split_app(lhs);
        let rhs_parts = split_app(rhs);
        if lhs_parts.is_empty() || rhs_parts.is_empty() {
            return vec![];
        }
        if lhs_parts[0] != rhs_parts[0] {
            return vec![];
        }
        if lhs_parts.len() != rhs_parts.len() {
            return vec![];
        }
        lhs_parts[1..]
            .iter()
            .zip(rhs_parts[1..].iter())
            .filter(|(a, b)| a != b)
            .map(|(a, b)| format!("{} = {}", a, b))
            .collect()
    }
    /// Generate a congruence lemma for a function of the given arity.
    pub fn generate_congr_lemma(fn_name: &str, arity: usize) -> CongrLemma {
        CongrLemma::new(format!("{}.congr", fn_name), arity, CongrType::Default)
    }
}
