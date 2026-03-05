//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::declaration::{ConstructorVal, InductiveVal};
use crate::def_eq::DefEqChecker;
use crate::error::KernelError;
use crate::expr_util::{get_app_args, get_app_fn, has_any_fvar, mk_app};
use crate::instantiate::instantiate_type_lparams;
use crate::reduce::{Reducer, TransparencyMode};
use crate::subst::{abstract_expr, instantiate};
use crate::{BinderInfo, Environment, Expr, FVarId, Level, Literal, Name};

use std::collections::HashMap;

/// A simple directed acyclic graph.
#[allow(dead_code)]
pub struct SimpleDag {
    /// `edges[i]` is the list of direct successors of node `i`.
    edges: Vec<Vec<usize>>,
}
#[allow(dead_code)]
impl SimpleDag {
    /// Creates a DAG with `n` nodes and no edges.
    pub fn new(n: usize) -> Self {
        Self {
            edges: vec![Vec::new(); n],
        }
    }
    /// Adds an edge from `from` to `to`.
    pub fn add_edge(&mut self, from: usize, to: usize) {
        if from < self.edges.len() {
            self.edges[from].push(to);
        }
    }
    /// Returns the successors of `node`.
    pub fn successors(&self, node: usize) -> &[usize] {
        self.edges.get(node).map(|v| v.as_slice()).unwrap_or(&[])
    }
    /// Returns `true` if `from` can reach `to` via DFS.
    pub fn can_reach(&self, from: usize, to: usize) -> bool {
        let mut visited = vec![false; self.edges.len()];
        self.dfs(from, to, &mut visited)
    }
    fn dfs(&self, cur: usize, target: usize, visited: &mut Vec<bool>) -> bool {
        if cur == target {
            return true;
        }
        if cur >= visited.len() || visited[cur] {
            return false;
        }
        visited[cur] = true;
        for &next in self.successors(cur) {
            if self.dfs(next, target, visited) {
                return true;
            }
        }
        false
    }
    /// Returns the topological order of nodes, or `None` if a cycle is detected.
    pub fn topological_sort(&self) -> Option<Vec<usize>> {
        let n = self.edges.len();
        let mut in_degree = vec![0usize; n];
        for succs in &self.edges {
            for &s in succs {
                if s < n {
                    in_degree[s] += 1;
                }
            }
        }
        let mut queue: std::collections::VecDeque<usize> =
            (0..n).filter(|&i| in_degree[i] == 0).collect();
        let mut order = Vec::new();
        while let Some(node) = queue.pop_front() {
            order.push(node);
            for &s in self.successors(node) {
                if s < n {
                    in_degree[s] -= 1;
                    if in_degree[s] == 0 {
                        queue.push_back(s);
                    }
                }
            }
        }
        if order.len() == n {
            Some(order)
        } else {
            None
        }
    }
    /// Returns the number of nodes.
    pub fn num_nodes(&self) -> usize {
        self.edges.len()
    }
}
/// A dependency closure builder (transitive closure via BFS).
#[allow(dead_code)]
pub struct TransitiveClosure {
    adj: Vec<Vec<usize>>,
    n: usize,
}
#[allow(dead_code)]
impl TransitiveClosure {
    /// Creates a transitive closure builder for `n` nodes.
    pub fn new(n: usize) -> Self {
        Self {
            adj: vec![Vec::new(); n],
            n,
        }
    }
    /// Adds a direct edge.
    pub fn add_edge(&mut self, from: usize, to: usize) {
        if from < self.n {
            self.adj[from].push(to);
        }
    }
    /// Computes all nodes reachable from `start` (including `start`).
    pub fn reachable_from(&self, start: usize) -> Vec<usize> {
        let mut visited = vec![false; self.n];
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(start);
        while let Some(node) = queue.pop_front() {
            if node >= self.n || visited[node] {
                continue;
            }
            visited[node] = true;
            for &next in &self.adj[node] {
                queue.push_back(next);
            }
        }
        (0..self.n).filter(|&i| visited[i]).collect()
    }
    /// Returns `true` if `from` can transitively reach `to`.
    pub fn can_reach(&self, from: usize, to: usize) -> bool {
        self.reachable_from(from).contains(&to)
    }
}
/// A hierarchical configuration tree.
#[allow(dead_code)]
pub struct ConfigNode {
    key: String,
    value: Option<String>,
    children: Vec<ConfigNode>,
}
#[allow(dead_code)]
impl ConfigNode {
    /// Creates a leaf config node with a value.
    pub fn leaf(key: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            value: Some(value.into()),
            children: Vec::new(),
        }
    }
    /// Creates a section node with children.
    pub fn section(key: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            value: None,
            children: Vec::new(),
        }
    }
    /// Adds a child node.
    pub fn add_child(&mut self, child: ConfigNode) {
        self.children.push(child);
    }
    /// Returns the key.
    pub fn key(&self) -> &str {
        &self.key
    }
    /// Returns the value, or `None` for section nodes.
    pub fn value(&self) -> Option<&str> {
        self.value.as_deref()
    }
    /// Returns the number of children.
    pub fn num_children(&self) -> usize {
        self.children.len()
    }
    /// Looks up a dot-separated path.
    pub fn lookup(&self, path: &str) -> Option<&str> {
        let mut parts = path.splitn(2, '.');
        let head = parts.next()?;
        let tail = parts.next();
        if head != self.key {
            return None;
        }
        match tail {
            None => self.value.as_deref(),
            Some(rest) => self.children.iter().find_map(|c| c.lookup_relative(rest)),
        }
    }
    fn lookup_relative(&self, path: &str) -> Option<&str> {
        let mut parts = path.splitn(2, '.');
        let head = parts.next()?;
        let tail = parts.next();
        if head != self.key {
            return None;
        }
        match tail {
            None => self.value.as_deref(),
            Some(rest) => self.children.iter().find_map(|c| c.lookup_relative(rest)),
        }
    }
}
/// A typing judgment: an expression with its inferred type.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct TypingJudgment {
    /// The expression.
    pub expr: Expr,
    /// The inferred type.
    pub ty: Expr,
    /// Whether the inference was successful.
    pub success: bool,
}
impl TypingJudgment {
    /// Create a successful typing judgment.
    #[allow(dead_code)]
    pub fn ok(expr: Expr, ty: Expr) -> Self {
        Self {
            expr,
            ty,
            success: true,
        }
    }
    /// Create a failed typing judgment (no type available).
    #[allow(dead_code)]
    pub fn fail(expr: Expr) -> Self {
        Self {
            ty: Expr::Sort(Level::zero()),
            expr,
            success: false,
        }
    }
    /// Check if the judgment was successful.
    #[allow(dead_code)]
    pub fn is_ok(&self) -> bool {
        self.success
    }
}
/// A sparse vector: stores only non-default elements.
#[allow(dead_code)]
pub struct SparseVec<T: Default + Clone + PartialEq> {
    entries: std::collections::HashMap<usize, T>,
    default_: T,
    logical_len: usize,
}
#[allow(dead_code)]
impl<T: Default + Clone + PartialEq> SparseVec<T> {
    /// Creates a new sparse vector with logical length `len`.
    pub fn new(len: usize) -> Self {
        Self {
            entries: std::collections::HashMap::new(),
            default_: T::default(),
            logical_len: len,
        }
    }
    /// Sets element at `idx`.
    pub fn set(&mut self, idx: usize, val: T) {
        if val == self.default_ {
            self.entries.remove(&idx);
        } else {
            self.entries.insert(idx, val);
        }
    }
    /// Gets element at `idx`.
    pub fn get(&self, idx: usize) -> &T {
        self.entries.get(&idx).unwrap_or(&self.default_)
    }
    /// Returns the logical length.
    pub fn len(&self) -> usize {
        self.logical_len
    }
    /// Returns whether the collection is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    /// Returns the number of non-default elements.
    pub fn nnz(&self) -> usize {
        self.entries.len()
    }
}
/// A simple stack-based calculator for arithmetic expressions.
#[allow(dead_code)]
pub struct StackCalc {
    stack: Vec<i64>,
}
#[allow(dead_code)]
impl StackCalc {
    /// Creates a new empty calculator.
    pub fn new() -> Self {
        Self { stack: Vec::new() }
    }
    /// Pushes an integer literal.
    pub fn push(&mut self, n: i64) {
        self.stack.push(n);
    }
    /// Adds the top two values.  Panics if fewer than two values.
    pub fn add(&mut self) {
        let b = self
            .stack
            .pop()
            .expect("stack must have at least two values for add");
        let a = self
            .stack
            .pop()
            .expect("stack must have at least two values for add");
        self.stack.push(a + b);
    }
    /// Subtracts top from second.
    pub fn sub(&mut self) {
        let b = self
            .stack
            .pop()
            .expect("stack must have at least two values for sub");
        let a = self
            .stack
            .pop()
            .expect("stack must have at least two values for sub");
        self.stack.push(a - b);
    }
    /// Multiplies the top two values.
    pub fn mul(&mut self) {
        let b = self
            .stack
            .pop()
            .expect("stack must have at least two values for mul");
        let a = self
            .stack
            .pop()
            .expect("stack must have at least two values for mul");
        self.stack.push(a * b);
    }
    /// Peeks the top value.
    pub fn peek(&self) -> Option<i64> {
        self.stack.last().copied()
    }
    /// Returns the stack depth.
    pub fn depth(&self) -> usize {
        self.stack.len()
    }
}
/// A window iterator that yields overlapping windows of size `n`.
#[allow(dead_code)]
pub struct WindowIterator<'a, T> {
    pub(super) data: &'a [T],
    pub(super) pos: usize,
    pub(super) window: usize,
}
#[allow(dead_code)]
impl<'a, T> WindowIterator<'a, T> {
    /// Creates a new window iterator.
    pub fn new(data: &'a [T], window: usize) -> Self {
        Self {
            data,
            pos: 0,
            window,
        }
    }
}
/// A cache entry for type inference results.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct InferCacheEntry {
    /// The expression whose type was inferred.
    pub expr: Expr,
    /// The inferred type.
    pub ty: Expr,
}
/// Represents a rewrite rule `lhs → rhs`.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct RewriteRule {
    /// The name of the rule.
    pub name: String,
    /// A string representation of the LHS pattern.
    pub lhs: String,
    /// A string representation of the RHS.
    pub rhs: String,
    /// Whether this is a conditional rule (has side conditions).
    pub conditional: bool,
}
#[allow(dead_code)]
impl RewriteRule {
    /// Creates an unconditional rewrite rule.
    pub fn unconditional(
        name: impl Into<String>,
        lhs: impl Into<String>,
        rhs: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            lhs: lhs.into(),
            rhs: rhs.into(),
            conditional: false,
        }
    }
    /// Creates a conditional rewrite rule.
    pub fn conditional(
        name: impl Into<String>,
        lhs: impl Into<String>,
        rhs: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            lhs: lhs.into(),
            rhs: rhs.into(),
            conditional: true,
        }
    }
    /// Returns a textual representation.
    pub fn display(&self) -> String {
        format!("{}: {} → {}", self.name, self.lhs, self.rhs)
    }
}
/// A set of rewrite rules.
#[allow(dead_code)]
pub struct RewriteRuleSet {
    rules: Vec<RewriteRule>,
}
#[allow(dead_code)]
impl RewriteRuleSet {
    /// Creates an empty rule set.
    pub fn new() -> Self {
        Self { rules: Vec::new() }
    }
    /// Adds a rule.
    pub fn add(&mut self, rule: RewriteRule) {
        self.rules.push(rule);
    }
    /// Returns the number of rules.
    pub fn len(&self) -> usize {
        self.rules.len()
    }
    /// Returns `true` if the set is empty.
    pub fn is_empty(&self) -> bool {
        self.rules.is_empty()
    }
    /// Returns all conditional rules.
    pub fn conditional_rules(&self) -> Vec<&RewriteRule> {
        self.rules.iter().filter(|r| r.conditional).collect()
    }
    /// Returns all unconditional rules.
    pub fn unconditional_rules(&self) -> Vec<&RewriteRule> {
        self.rules.iter().filter(|r| !r.conditional).collect()
    }
    /// Looks up a rule by name.
    pub fn get(&self, name: &str) -> Option<&RewriteRule> {
        self.rules.iter().find(|r| r.name == name)
    }
}
/// A counter that can measure elapsed time between snapshots.
#[allow(dead_code)]
pub struct Stopwatch {
    start: std::time::Instant,
    splits: Vec<f64>,
}
#[allow(dead_code)]
impl Stopwatch {
    /// Creates and starts a new stopwatch.
    pub fn start() -> Self {
        Self {
            start: std::time::Instant::now(),
            splits: Vec::new(),
        }
    }
    /// Records a split time (elapsed since start).
    pub fn split(&mut self) {
        self.splits.push(self.elapsed_ms());
    }
    /// Returns total elapsed milliseconds since start.
    pub fn elapsed_ms(&self) -> f64 {
        self.start.elapsed().as_secs_f64() * 1000.0
    }
    /// Returns all recorded split times.
    pub fn splits(&self) -> &[f64] {
        &self.splits
    }
    /// Returns the number of splits.
    pub fn num_splits(&self) -> usize {
        self.splits.len()
    }
}
/// Statistics collected during type inference.
#[derive(Clone, Debug, Default)]
pub struct InferStats {
    /// Number of types inferred.
    pub infer_calls: usize,
    /// Number of WHNF reductions performed.
    pub whnf_calls: usize,
    /// Number of definitional equality checks.
    pub def_eq_calls: usize,
    /// Number of constant lookups.
    pub const_lookups: usize,
    /// Number of cache hits.
    pub cache_hits: usize,
}
impl InferStats {
    /// Reset all counters to zero.
    pub fn reset(&mut self) {
        *self = InferStats::default();
    }
    /// Total operations performed.
    pub fn total_ops(&self) -> usize {
        self.infer_calls + self.whnf_calls + self.def_eq_calls
    }
}
/// A mutable reference stack for tracking the current "focus" in a tree traversal.
#[allow(dead_code)]
pub struct FocusStack<T> {
    items: Vec<T>,
}
#[allow(dead_code)]
impl<T> FocusStack<T> {
    /// Creates an empty focus stack.
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }
    /// Focuses on `item`.
    pub fn focus(&mut self, item: T) {
        self.items.push(item);
    }
    /// Blurs (pops) the current focus.
    pub fn blur(&mut self) -> Option<T> {
        self.items.pop()
    }
    /// Returns the current focus, or `None`.
    pub fn current(&self) -> Option<&T> {
        self.items.last()
    }
    /// Returns the focus depth.
    pub fn depth(&self) -> usize {
        self.items.len()
    }
    /// Returns `true` if there is no current focus.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}
/// A non-empty list (at least one element guaranteed).
#[allow(dead_code)]
pub struct NonEmptyVec<T> {
    head: T,
    tail: Vec<T>,
}
#[allow(dead_code)]
impl<T> NonEmptyVec<T> {
    /// Creates a non-empty vec with a single element.
    pub fn singleton(val: T) -> Self {
        Self {
            head: val,
            tail: Vec::new(),
        }
    }
    /// Pushes an element.
    pub fn push(&mut self, val: T) {
        self.tail.push(val);
    }
    /// Returns a reference to the first element.
    pub fn first(&self) -> &T {
        &self.head
    }
    /// Returns a reference to the last element.
    pub fn last(&self) -> &T {
        self.tail.last().unwrap_or(&self.head)
    }
    /// Returns the number of elements.
    pub fn len(&self) -> usize {
        1 + self.tail.len()
    }
    /// Returns whether the collection is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    /// Returns all elements as a Vec.
    pub fn to_vec(&self) -> Vec<&T> {
        let mut v = vec![&self.head];
        v.extend(self.tail.iter());
        v
    }
}
/// A pair of `StatSummary` values tracking before/after a transformation.
#[allow(dead_code)]
pub struct TransformStat {
    before: StatSummary,
    after: StatSummary,
}
#[allow(dead_code)]
impl TransformStat {
    /// Creates a new transform stat recorder.
    pub fn new() -> Self {
        Self {
            before: StatSummary::new(),
            after: StatSummary::new(),
        }
    }
    /// Records a before value.
    pub fn record_before(&mut self, v: f64) {
        self.before.record(v);
    }
    /// Records an after value.
    pub fn record_after(&mut self, v: f64) {
        self.after.record(v);
    }
    /// Returns the mean reduction ratio (after/before).
    pub fn mean_ratio(&self) -> Option<f64> {
        let b = self.before.mean()?;
        let a = self.after.mean()?;
        if b.abs() < f64::EPSILON {
            return None;
        }
        Some(a / b)
    }
}
/// A token bucket rate limiter.
#[allow(dead_code)]
pub struct TokenBucket {
    capacity: u64,
    tokens: u64,
    refill_per_ms: u64,
    last_refill: std::time::Instant,
}
#[allow(dead_code)]
impl TokenBucket {
    /// Creates a new token bucket.
    pub fn new(capacity: u64, refill_per_ms: u64) -> Self {
        Self {
            capacity,
            tokens: capacity,
            refill_per_ms,
            last_refill: std::time::Instant::now(),
        }
    }
    /// Attempts to consume `n` tokens.  Returns `true` on success.
    pub fn try_consume(&mut self, n: u64) -> bool {
        self.refill();
        if self.tokens >= n {
            self.tokens -= n;
            true
        } else {
            false
        }
    }
    fn refill(&mut self) {
        let now = std::time::Instant::now();
        let elapsed_ms = now.duration_since(self.last_refill).as_millis() as u64;
        if elapsed_ms > 0 {
            let new_tokens = elapsed_ms * self.refill_per_ms;
            self.tokens = (self.tokens + new_tokens).min(self.capacity);
            self.last_refill = now;
        }
    }
    /// Returns the number of currently available tokens.
    pub fn available(&self) -> u64 {
        self.tokens
    }
    /// Returns the bucket capacity.
    pub fn capacity(&self) -> u64 {
        self.capacity
    }
}
/// A flat list of substitution pairs `(from, to)`.
#[allow(dead_code)]
pub struct FlatSubstitution {
    pairs: Vec<(String, String)>,
}
#[allow(dead_code)]
impl FlatSubstitution {
    /// Creates an empty substitution.
    pub fn new() -> Self {
        Self { pairs: Vec::new() }
    }
    /// Adds a pair.
    pub fn add(&mut self, from: impl Into<String>, to: impl Into<String>) {
        self.pairs.push((from.into(), to.into()));
    }
    /// Applies all substitutions to `s` (leftmost-first order).
    pub fn apply(&self, s: &str) -> String {
        let mut result = s.to_string();
        for (from, to) in &self.pairs {
            result = result.replace(from.as_str(), to.as_str());
        }
        result
    }
    /// Returns the number of pairs.
    pub fn len(&self) -> usize {
        self.pairs.len()
    }
    /// Returns `true` if empty.
    pub fn is_empty(&self) -> bool {
        self.pairs.is_empty()
    }
}
/// Type-level kind classification of an expression.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TypeKind {
    /// A proposition (Sort 0 / Prop).
    Prop,
    /// A small type (Sort 1, i.e., Type 0).
    Type0,
    /// A large type (Sort n for n ≥ 2).
    LargeType,
    /// A universe Sort expression itself.
    Universe,
    /// A function type (Pi).
    Pi,
    /// A lambda (not a type).
    Lambda,
    /// An application (not directly a type).
    Application,
    /// A free variable.
    FreeVar,
    /// A constant.
    Constant,
    /// A literal value.
    Literal,
    /// Unknown or uncategorized.
    Unknown,
}
/// A write-once cell.
#[allow(dead_code)]
pub struct WriteOnce<T> {
    value: std::cell::Cell<Option<T>>,
}
#[allow(dead_code)]
impl<T: Copy> WriteOnce<T> {
    /// Creates an empty write-once cell.
    pub fn new() -> Self {
        Self {
            value: std::cell::Cell::new(None),
        }
    }
    /// Writes a value.  Returns `false` if already written.
    pub fn write(&self, val: T) -> bool {
        if self.value.get().is_some() {
            return false;
        }
        self.value.set(Some(val));
        true
    }
    /// Returns the value if written.
    pub fn read(&self) -> Option<T> {
        self.value.get()
    }
    /// Returns `true` if the value has been written.
    pub fn is_written(&self) -> bool {
        self.value.get().is_some()
    }
}
/// Unified type checker state.
///
/// Owns WHNF reduction, definitional equality checking, and type inference.
/// Mirrors LEAN 4's `type_checker` class.
pub struct TypeChecker<'env> {
    env: &'env Environment,
    local_ctx: Vec<LocalDecl>,
    reducer: Reducer,
    def_eq_checker: DefEqChecker<'env>,
    next_fvar: u64,
    /// Whether to perform full checking (vs. infer-only mode).
    check_mode: bool,
}
impl<'env> TypeChecker<'env> {
    /// Create a new type checker for the given environment.
    pub fn new(env: &'env Environment) -> Self {
        Self {
            env,
            local_ctx: Vec::new(),
            reducer: Reducer::new(),
            def_eq_checker: DefEqChecker::new(env),
            next_fvar: 0,
            check_mode: true,
        }
    }
    /// Create a type checker in infer-only mode (skips some checks).
    pub fn new_infer_only(env: &'env Environment) -> Self {
        Self {
            env,
            local_ctx: Vec::new(),
            reducer: Reducer::new(),
            def_eq_checker: DefEqChecker::new(env),
            next_fvar: 0,
            check_mode: false,
        }
    }
    /// Set the transparency mode.
    pub fn set_transparency(&mut self, mode: TransparencyMode) {
        self.reducer.set_transparency(mode);
        self.def_eq_checker.set_transparency(mode);
    }
    /// Get the environment.
    pub fn env(&self) -> &Environment {
        self.env
    }
    /// Generate a fresh free variable.
    pub fn fresh_fvar(&mut self, name: Name, ty: Expr) -> FVarId {
        let fvar = FVarId(self.next_fvar);
        self.next_fvar += 1;
        self.local_ctx.push(LocalDecl {
            fvar,
            name,
            ty,
            val: None,
        });
        fvar
    }
    /// Generate a fresh free variable with a value (let-binding).
    pub fn fresh_fvar_let(&mut self, name: Name, ty: Expr, val: Expr) -> FVarId {
        let fvar = FVarId(self.next_fvar);
        self.next_fvar += 1;
        self.local_ctx.push(LocalDecl {
            fvar,
            name,
            ty,
            val: Some(val),
        });
        fvar
    }
    /// Push a local declaration onto the context.
    pub fn push_local(&mut self, decl: LocalDecl) {
        self.local_ctx.push(decl);
    }
    /// Pop a local declaration from the context.
    pub fn pop_local(&mut self) -> Option<LocalDecl> {
        self.local_ctx.pop()
    }
    /// Look up a free variable in the local context.
    #[allow(clippy::result_large_err)]
    fn lookup_fvar(&self, fvar: FVarId) -> Result<&LocalDecl, KernelError> {
        self.local_ctx
            .iter()
            .find(|decl| decl.fvar == fvar)
            .ok_or_else(|| KernelError::Other(format!("free variable not found: {:?}", fvar)))
    }
    /// Get the local context.
    pub fn local_ctx(&self) -> &[LocalDecl] {
        &self.local_ctx
    }
    /// Reduce an expression to WHNF.
    pub fn whnf(&mut self, expr: &Expr) -> Expr {
        self.reducer.whnf_env(expr, self.env)
    }
    /// Ensure an expression is a Sort, returning the level.
    ///
    /// Infers the type, reduces to WHNF, and checks it's a Sort.
    #[allow(clippy::result_large_err)]
    pub fn ensure_sort(&mut self, expr: &Expr) -> Result<Level, KernelError> {
        let ty = self.infer_type(expr)?;
        let ty_whnf = self.whnf(&ty);
        match ty_whnf {
            Expr::Sort(l) => Ok(l),
            _ => Err(KernelError::NotASort(ty_whnf)),
        }
    }
    /// Ensure an expression's type is a Pi type, returning it.
    #[allow(clippy::result_large_err)]
    pub fn ensure_pi(&mut self, expr: &Expr) -> Result<Expr, KernelError> {
        let ty = self.infer_type(expr)?;
        let ty_whnf = self.whnf(&ty);
        if ty_whnf.is_pi() {
            Ok(ty_whnf)
        } else {
            Err(KernelError::NotAFunction(ty_whnf))
        }
    }
    /// Check if two expressions are definitionally equal.
    pub fn is_def_eq(&mut self, t: &Expr, s: &Expr) -> bool {
        self.def_eq_checker.is_def_eq(t, s)
    }
    /// Check that an expression has the expected type.
    #[allow(clippy::result_large_err)]
    pub fn check_type(
        &mut self,
        expr: &Expr,
        inferred: &Expr,
        expected: &Expr,
    ) -> Result<(), KernelError> {
        if self.is_def_eq(inferred, expected) {
            Ok(())
        } else {
            Err(KernelError::TypeMismatch {
                expected: expected.clone(),
                got: inferred.clone(),
                context: format!("checking {}", expr),
            })
        }
    }
    /// Infer the type of an expression.
    #[allow(clippy::result_large_err)]
    pub fn infer_type(&mut self, expr: &Expr) -> Result<Expr, KernelError> {
        match expr {
            Expr::Sort(l) => Ok(Expr::Sort(Level::succ(l.clone()))),
            Expr::BVar(idx) => Err(KernelError::UnboundVariable(*idx)),
            Expr::FVar(fvar) => {
                let decl = self.lookup_fvar(*fvar)?;
                Ok(decl.ty.clone())
            }
            Expr::Const(name, levels) => self.infer_const(name, levels),
            Expr::App(f, a) => self.infer_app(f, a),
            Expr::Lam(bi, name, ty, body) => {
                if self.check_mode {
                    self.ensure_sort(ty)?;
                }
                let fvar = self.fresh_fvar(name.clone(), (**ty).clone());
                let body_open = instantiate(body, &Expr::FVar(fvar));
                let body_ty = self.infer_type(&body_open)?;
                self.pop_local();
                let body_ty_closed = abstract_expr(&body_ty, fvar);
                Ok(Expr::Pi(
                    *bi,
                    name.clone(),
                    ty.clone(),
                    Box::new(body_ty_closed),
                ))
            }
            Expr::Pi(_, _, dom, cod) => {
                let dom_sort = self.ensure_sort(dom)?;
                let fvar = self.fresh_fvar(Name::str("_"), (**dom).clone());
                let cod_open = instantiate(cod, &Expr::FVar(fvar));
                let cod_sort = self.ensure_sort(&cod_open)?;
                self.pop_local();
                Ok(Expr::Sort(Level::imax(dom_sort, cod_sort)))
            }
            Expr::Let(_, ty, val, body) => {
                if self.check_mode {
                    self.ensure_sort(ty)?;
                    let val_ty = self.infer_type(val)?;
                    self.check_type(val, &val_ty, ty)?;
                }
                let body_inst = instantiate(body, val);
                self.infer_type(&body_inst)
            }
            Expr::Lit(Literal::Nat(_)) => Ok(Expr::Const(Name::str("Nat"), vec![])),
            Expr::Lit(Literal::Str(_)) => Ok(Expr::Const(Name::str("String"), vec![])),
            Expr::Proj(struct_name, idx, struct_expr) => {
                self.infer_proj(struct_name, *idx, struct_expr)
            }
        }
    }
    /// Infer the type of a constant reference.
    #[allow(clippy::result_large_err)]
    fn infer_const(&self, name: &Name, levels: &[Level]) -> Result<Expr, KernelError> {
        if let Some(ci) = self.env.find(name) {
            let params = ci.level_params();
            if !params.is_empty() && !levels.is_empty() && params.len() != levels.len() {
                return Err(KernelError::Other(format!(
                    "universe parameter count mismatch for {}: expected {}, got {}",
                    name,
                    params.len(),
                    levels.len()
                )));
            }
            if params.is_empty() || levels.is_empty() {
                return Ok(ci.ty().clone());
            }
            return Ok(instantiate_type_lparams(ci.ty(), params, levels));
        }
        let decl = self
            .env
            .get(name)
            .ok_or_else(|| KernelError::UnknownConstant(name.clone()))?;
        Ok(decl.ty().clone())
    }
    /// Infer the type of a function application.
    #[allow(clippy::result_large_err)]
    fn infer_app(&mut self, f: &Expr, a: &Expr) -> Result<Expr, KernelError> {
        let f_ty = self.infer_type(f)?;
        let f_ty_whnf = self.whnf(&f_ty);
        match &f_ty_whnf {
            Expr::Pi(_, _, dom, cod) => {
                if self.check_mode {
                    let a_ty = self.infer_type(a)?;
                    self.check_type(a, &a_ty, dom)?;
                }
                Ok(instantiate(cod, a))
            }
            _ => Err(KernelError::NotAFunction(f_ty_whnf)),
        }
    }
    /// Infer the type of a structure projection.
    #[allow(clippy::result_large_err)]
    fn infer_proj(
        &mut self,
        struct_name: &Name,
        idx: u32,
        struct_expr: &Expr,
    ) -> Result<Expr, KernelError> {
        let ind_val = self
            .env
            .get_inductive_val(struct_name)
            .ok_or_else(|| KernelError::Other(format!("not a structure type: {}", struct_name)))?
            .clone();
        if ind_val.ctors.len() != 1 {
            return Err(KernelError::Other(format!(
                "{} is not a structure (has {} constructors)",
                struct_name,
                ind_val.ctors.len()
            )));
        }
        let ctor_name = &ind_val.ctors[0];
        let ctor_val = self
            .env
            .get_constructor_val(ctor_name)
            .ok_or_else(|| KernelError::Other(format!("constructor not found: {}", ctor_name)))?
            .clone();
        if idx >= ctor_val.num_fields {
            return Err(KernelError::Other(format!(
                "field index {} out of range for {} (has {} fields)",
                idx, struct_name, ctor_val.num_fields
            )));
        }
        let struct_ty = self.infer_type(struct_expr)?;
        Ok(self.infer_proj_field_type(&ind_val, &ctor_val, idx, struct_expr, &struct_ty))
    }
    /// Compute the type of a projection field.
    ///
    /// Telescopes through the constructor's Pi-type, instantiating:
    /// 1. Universe parameters from the struct type's head constant
    /// 2. Inductive parameters from the struct type's applied arguments
    /// 3. Preceding fields with `Proj(S, j, struct_expr)` for `j < idx`
    ///
    /// Returns the domain of the `idx`-th Pi binder.
    fn infer_proj_field_type(
        &mut self,
        ind_val: &InductiveVal,
        ctor_val: &ConstructorVal,
        idx: u32,
        struct_expr: &Expr,
        struct_ty: &Expr,
    ) -> Expr {
        let ctor_ty = ctor_val.common.ty.clone();
        let struct_ty_whnf = self.whnf(struct_ty);
        let levels: Vec<Level> = match get_app_fn(&struct_ty_whnf) {
            Expr::Const(_, lvls) => lvls.clone(),
            _ => vec![],
        };
        let level_params = &ind_val.common.level_params;
        let mut cur_ty = instantiate_type_lparams(&ctor_ty, level_params, &levels);
        let struct_args: Vec<Expr> = get_app_args(&struct_ty_whnf).into_iter().cloned().collect();
        for i in 0..ind_val.num_params as usize {
            match cur_ty {
                Expr::Pi(_, _, _, body) => {
                    let param = struct_args.get(i).cloned().unwrap_or(Expr::BVar(0));
                    cur_ty = instantiate(&body, &param);
                }
                _ => return struct_ty.clone(),
            }
        }
        for j in 0..idx {
            match cur_ty {
                Expr::Pi(_, _, _, body) => {
                    let field_val = Expr::Proj(
                        ind_val.common.name.clone(),
                        j,
                        Box::new(struct_expr.clone()),
                    );
                    cur_ty = instantiate(&body, &field_val);
                }
                _ => return struct_ty.clone(),
            }
        }
        match cur_ty {
            Expr::Pi(_, _, dom, _) => *dom,
            _ => struct_ty.clone(),
        }
    }
    /// Check if an expression is a proposition (has type Prop).
    pub fn is_prop(&mut self, expr: &Expr) -> bool {
        if let Ok(ty) = self.infer_type(expr) {
            let ty_whnf = self.whnf(&ty);
            matches!(ty_whnf, Expr::Sort(l) if l.is_zero())
        } else {
            false
        }
    }
    /// Check if an expression is a proof (its type is a proposition).
    pub fn is_proof(&mut self, expr: &Expr) -> bool {
        if let Ok(ty) = self.infer_type(expr) {
            self.is_prop(&ty)
        } else {
            false
        }
    }
    /// Check if an expression is a type (has type Sort u for some u).
    pub fn is_type(&mut self, expr: &Expr) -> bool {
        if let Ok(ty) = self.infer_type(expr) {
            let ty_whnf = self.whnf(&ty);
            ty_whnf.is_sort()
        } else {
            false
        }
    }
    /// Get the universe level of a type expression.
    #[allow(clippy::result_large_err)]
    pub fn get_level(&mut self, expr: &Expr) -> Result<Level, KernelError> {
        let ty = self.infer_type(expr)?;
        let ty_whnf = self.whnf(&ty);
        match ty_whnf {
            Expr::Sort(l) => Ok(l),
            _ => Err(KernelError::NotASort(ty_whnf)),
        }
    }
    /// Try to unfold a constant definition.
    pub fn unfold_definition(&mut self, expr: &Expr) -> Option<Expr> {
        let head = get_app_fn(expr);
        if let Expr::Const(name, levels) = head {
            if let Some(ci) = self.env.find(name) {
                if let Some(val) = ci.value() {
                    let hint = ci.reducibility_hint();
                    if hint.should_unfold() {
                        let val_inst = if ci.level_params().is_empty() || levels.is_empty() {
                            val.clone()
                        } else {
                            instantiate_type_lparams(val, ci.level_params(), levels)
                        };
                        let args: Vec<Expr> = get_app_args(expr).into_iter().cloned().collect();
                        return Some(mk_app(val_inst, &args));
                    }
                }
            }
        }
        None
    }
}
impl<'env> TypeChecker<'env> {
    /// Infer the type of a chain of applications at once.
    #[allow(clippy::result_large_err)]
    pub fn infer_app_chain(
        &mut self,
        f: &Expr,
        args: &[Expr],
    ) -> Result<Expr, crate::error::KernelError> {
        let mut ty = self.infer_type(f)?;
        for arg in args {
            let whnf = self.whnf(&ty);
            match whnf {
                Expr::Pi(_, _, dom, cod) => {
                    if self.check_mode {
                        let arg_ty = self.infer_type(arg)?;
                        if !self.is_def_eq(&arg_ty, &dom) {
                            return Err(crate::error::KernelError::TypeMismatch {
                                expected: *dom,
                                got: arg_ty,
                                context: "application argument".to_string(),
                            });
                        }
                    }
                    ty = instantiate(&cod, arg);
                }
                other => return Err(crate::error::KernelError::NotAFunction(other)),
            }
        }
        Ok(ty)
    }
    /// Telescope a type, instantiating leading Pi binders with fresh free variables.
    pub fn telescope_type(&mut self, ty: &Expr, max_pis: usize) -> (Vec<LocalDecl>, Expr) {
        let mut fvars = Vec::new();
        let mut current = ty.clone();
        for _ in 0..max_pis {
            let whnf = self.whnf(&current);
            match whnf {
                Expr::Pi(bi, name, dom, cod) => {
                    let fvar_id = self.fresh_fvar(name.clone(), *dom.clone());
                    let decl = LocalDecl {
                        fvar: fvar_id,
                        name,
                        ty: *dom,
                        val: None,
                    };
                    let body = instantiate(&cod, &Expr::FVar(fvar_id));
                    fvars.push(decl);
                    current = body;
                    let _ = bi;
                }
                _ => break,
            }
        }
        (fvars, current)
    }
    /// Close a type over a list of free variables into Pi types.
    pub fn close_type_over_fvars(&mut self, fvars: &[LocalDecl], ty: Expr) -> Expr {
        let mut result = ty;
        for decl in fvars.iter().rev() {
            result = abstract_expr(&result, decl.fvar);
            result = Expr::Pi(
                crate::BinderInfo::Default,
                decl.name.clone(),
                Box::new(decl.ty.clone()),
                Box::new(result),
            );
        }
        result
    }
    /// Close a term over free variables into lambdas.
    pub fn close_term_over_fvars(&mut self, fvars: &[LocalDecl], term: Expr) -> Expr {
        let mut result = term;
        for decl in fvars.iter().rev() {
            result = abstract_expr(&result, decl.fvar);
            result = Expr::Lam(
                crate::BinderInfo::Default,
                decl.name.clone(),
                Box::new(decl.ty.clone()),
                Box::new(result),
            );
        }
        result
    }
    /// Check that `expr` has type `expected`.
    #[allow(clippy::result_large_err)]
    pub fn check(&mut self, expr: &Expr, expected: &Expr) -> Result<(), crate::error::KernelError> {
        let inferred = self.infer_type(expr)?;
        if self.is_def_eq(&inferred, expected) {
            Ok(())
        } else {
            Err(crate::error::KernelError::TypeMismatch {
                expected: expected.clone(),
                got: inferred,
                context: format!("check({:?})", expr),
            })
        }
    }
    /// Try to check; return true if the type matches.
    pub fn try_check(&mut self, expr: &Expr, expected: &Expr) -> bool {
        if let Ok(inferred) = self.infer_type(expr) {
            self.is_def_eq(&inferred, expected)
        } else {
            false
        }
    }
    /// Get the number of Pi binders in the WHNF of a type.
    pub fn count_pi_binders(&mut self, ty: &Expr) -> usize {
        let mut count = 0;
        let mut current = ty.clone();
        loop {
            let whnf = self.whnf(&current);
            if let Expr::Pi(_, _, _, cod) = whnf {
                count += 1;
                current = *cod;
            } else {
                break;
            }
        }
        count
    }
    /// Verify a declaration's type and optional value.
    #[allow(clippy::result_large_err)]
    pub fn verify_declaration(
        &mut self,
        name: &Name,
        ty: &Expr,
        val: Option<&Expr>,
    ) -> Result<(), crate::error::KernelError> {
        self.ensure_sort(ty)?;
        if let Some(v) = val {
            let v_ty = self.infer_type(v)?;
            if !self.is_def_eq(&v_ty, ty) {
                return Err(crate::error::KernelError::TypeMismatch {
                    expected: ty.clone(),
                    got: v_ty,
                    context: format!("verifying declaration {}", name),
                });
            }
        }
        Ok(())
    }
    /// Normalize an expression (full normal form).
    pub fn normalize(&mut self, expr: &Expr) -> Expr {
        let whnf = self.whnf(expr);
        match &whnf {
            Expr::App(f, a) => {
                let f_norm = self.normalize(f);
                let a_norm = self.normalize(a);
                Expr::App(Box::new(f_norm), Box::new(a_norm))
            }
            Expr::Lam(bi, name, ty, body) => {
                let ty_norm = self.normalize(ty);
                let body_norm = self.normalize(body);
                Expr::Lam(*bi, name.clone(), Box::new(ty_norm), Box::new(body_norm))
            }
            Expr::Pi(bi, name, ty, body) => {
                let ty_norm = self.normalize(ty);
                let body_norm = self.normalize(body);
                Expr::Pi(*bi, name.clone(), Box::new(ty_norm), Box::new(body_norm))
            }
            Expr::Let(name, ty, val, body) => {
                let ty_norm = self.normalize(ty);
                let val_norm = self.normalize(val);
                let body_norm = self.normalize(body);
                Expr::Let(
                    name.clone(),
                    Box::new(ty_norm),
                    Box::new(val_norm),
                    Box::new(body_norm),
                )
            }
            Expr::Proj(sname, idx, inner) => {
                let inner_norm = self.normalize(inner);
                Expr::Proj(sname.clone(), *idx, Box::new(inner_norm))
            }
            _ => whnf,
        }
    }
    /// Return the arity (number of Pi binders) in the type of a constant.
    pub fn constant_arity(&mut self, name: &Name) -> Option<usize> {
        let ty = if let Some(ci) = self.env.find(name) {
            ci.ty().clone()
        } else {
            self.env.get(name)?.ty().clone()
        };
        Some(self.count_pi_binders(&ty))
    }
    /// Check if two universe levels are definitionally equal.
    pub fn is_level_eq(&self, l1: &Level, l2: &Level) -> bool {
        l1 == l2 || (l1.is_zero() && l2.is_zero())
    }
}
/// A simple key-value store backed by a sorted Vec for small maps.
#[allow(dead_code)]
pub struct SmallMap<K: Ord + Clone, V: Clone> {
    entries: Vec<(K, V)>,
}
#[allow(dead_code)]
impl<K: Ord + Clone, V: Clone> SmallMap<K, V> {
    /// Creates a new empty small map.
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }
    /// Inserts or replaces the value for `key`.
    pub fn insert(&mut self, key: K, val: V) {
        match self.entries.binary_search_by_key(&&key, |(k, _)| k) {
            Ok(i) => self.entries[i].1 = val,
            Err(i) => self.entries.insert(i, (key, val)),
        }
    }
    /// Returns the value for `key`, or `None`.
    pub fn get(&self, key: &K) -> Option<&V> {
        self.entries
            .binary_search_by_key(&key, |(k, _)| k)
            .ok()
            .map(|i| &self.entries[i].1)
    }
    /// Returns the number of entries.
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    /// Returns `true` if empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
    /// Returns all keys.
    pub fn keys(&self) -> Vec<&K> {
        self.entries.iter().map(|(k, _)| k).collect()
    }
    /// Returns all values.
    pub fn values(&self) -> Vec<&V> {
        self.entries.iter().map(|(_, v)| v).collect()
    }
}
/// A versioned record that stores a history of values.
#[allow(dead_code)]
pub struct VersionedRecord<T: Clone> {
    history: Vec<T>,
}
#[allow(dead_code)]
impl<T: Clone> VersionedRecord<T> {
    /// Creates a new record with an initial value.
    pub fn new(initial: T) -> Self {
        Self {
            history: vec![initial],
        }
    }
    /// Updates the record with a new version.
    pub fn update(&mut self, val: T) {
        self.history.push(val);
    }
    /// Returns the current (latest) value.
    pub fn current(&self) -> &T {
        self.history
            .last()
            .expect("VersionedRecord history is always non-empty after construction")
    }
    /// Returns the value at version `n` (0-indexed), or `None`.
    pub fn at_version(&self, n: usize) -> Option<&T> {
        self.history.get(n)
    }
    /// Returns the version number of the current value.
    pub fn version(&self) -> usize {
        self.history.len() - 1
    }
    /// Returns `true` if more than one version exists.
    pub fn has_history(&self) -> bool {
        self.history.len() > 1
    }
}
/// A simple LRU-style type inference cache.
///
/// In practice, this is a bounded list of recent (expr → type) pairs.
#[allow(dead_code)]
pub struct InferCache {
    entries: Vec<InferCacheEntry>,
    capacity: usize,
}
impl InferCache {
    /// Create a new cache with a given capacity.
    #[allow(dead_code)]
    pub fn new(capacity: usize) -> Self {
        Self {
            entries: Vec::with_capacity(capacity),
            capacity,
        }
    }
    /// Look up a cached type for an expression.
    #[allow(dead_code)]
    pub fn get(&self, expr: &Expr) -> Option<&Expr> {
        self.entries
            .iter()
            .rev()
            .find(|e| &e.expr == expr)
            .map(|e| &e.ty)
    }
    /// Insert a new (expr, type) pair.
    #[allow(dead_code)]
    pub fn insert(&mut self, expr: Expr, ty: Expr) {
        if self.entries.len() >= self.capacity {
            self.entries.remove(0);
        }
        self.entries.push(InferCacheEntry { expr, ty });
    }
    /// Clear the cache.
    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.entries.clear();
    }
    /// Number of cached entries.
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    /// Whether the cache is empty.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}
/// A fixed-size sliding window that computes a running sum.
#[allow(dead_code)]
pub struct SlidingSum {
    window: Vec<f64>,
    capacity: usize,
    pos: usize,
    sum: f64,
    count: usize,
}
#[allow(dead_code)]
impl SlidingSum {
    /// Creates a sliding sum with the given window size.
    pub fn new(capacity: usize) -> Self {
        Self {
            window: vec![0.0; capacity],
            capacity,
            pos: 0,
            sum: 0.0,
            count: 0,
        }
    }
    /// Adds a value to the window, removing the oldest if full.
    pub fn push(&mut self, val: f64) {
        let oldest = self.window[self.pos];
        self.sum -= oldest;
        self.sum += val;
        self.window[self.pos] = val;
        self.pos = (self.pos + 1) % self.capacity;
        if self.count < self.capacity {
            self.count += 1;
        }
    }
    /// Returns the current window sum.
    pub fn sum(&self) -> f64 {
        self.sum
    }
    /// Returns the window mean, or `None` if empty.
    pub fn mean(&self) -> Option<f64> {
        if self.count == 0 {
            None
        } else {
            Some(self.sum / self.count as f64)
        }
    }
    /// Returns the current window size (number of valid elements).
    pub fn count(&self) -> usize {
        self.count
    }
}
/// Local declaration (free variable with type).
#[derive(Clone, Debug)]
pub struct LocalDecl {
    /// Free variable ID
    pub fvar: FVarId,
    /// Name (for pretty-printing)
    pub name: Name,
    /// Type of this variable
    pub ty: Expr,
    /// Optional value (for let-bound variables)
    pub val: Option<Expr>,
}
/// A pool of reusable string buffers.
#[allow(dead_code)]
pub struct StringPool {
    free: Vec<String>,
}
#[allow(dead_code)]
impl StringPool {
    /// Creates a new empty string pool.
    pub fn new() -> Self {
        Self { free: Vec::new() }
    }
    /// Takes a string from the pool (may be empty).
    pub fn take(&mut self) -> String {
        self.free.pop().unwrap_or_default()
    }
    /// Returns a string to the pool.
    pub fn give(&mut self, mut s: String) {
        s.clear();
        self.free.push(s);
    }
    /// Returns the number of free strings in the pool.
    pub fn free_count(&self) -> usize {
        self.free.len()
    }
}
/// A simple decision tree node for rule dispatching.
#[allow(dead_code)]
#[allow(missing_docs)]
pub enum DecisionNode {
    /// A leaf with an action string.
    Leaf(String),
    /// An interior node: check `key` equals `val` → `yes_branch`, else `no_branch`.
    Branch {
        key: String,
        val: String,
        yes_branch: Box<DecisionNode>,
        no_branch: Box<DecisionNode>,
    },
}
#[allow(dead_code)]
impl DecisionNode {
    /// Evaluates the decision tree with the given context.
    pub fn evaluate(&self, ctx: &std::collections::HashMap<String, String>) -> &str {
        match self {
            DecisionNode::Leaf(action) => action.as_str(),
            DecisionNode::Branch {
                key,
                val,
                yes_branch,
                no_branch,
            } => {
                let actual = ctx.get(key).map(|s| s.as_str()).unwrap_or("");
                if actual == val.as_str() {
                    yes_branch.evaluate(ctx)
                } else {
                    no_branch.evaluate(ctx)
                }
            }
        }
    }
    /// Returns the depth of the decision tree.
    pub fn depth(&self) -> usize {
        match self {
            DecisionNode::Leaf(_) => 0,
            DecisionNode::Branch {
                yes_branch,
                no_branch,
                ..
            } => 1 + yes_branch.depth().max(no_branch.depth()),
        }
    }
}
/// A generic counter that tracks min/max/sum for statistical summaries.
#[allow(dead_code)]
pub struct StatSummary {
    count: u64,
    sum: f64,
    min: f64,
    max: f64,
}
#[allow(dead_code)]
impl StatSummary {
    /// Creates an empty summary.
    pub fn new() -> Self {
        Self {
            count: 0,
            sum: 0.0,
            min: f64::INFINITY,
            max: f64::NEG_INFINITY,
        }
    }
    /// Records a sample.
    pub fn record(&mut self, val: f64) {
        self.count += 1;
        self.sum += val;
        if val < self.min {
            self.min = val;
        }
        if val > self.max {
            self.max = val;
        }
    }
    /// Returns the mean, or `None` if no samples.
    pub fn mean(&self) -> Option<f64> {
        if self.count == 0 {
            None
        } else {
            Some(self.sum / self.count as f64)
        }
    }
    /// Returns the minimum, or `None` if no samples.
    pub fn min(&self) -> Option<f64> {
        if self.count == 0 {
            None
        } else {
            Some(self.min)
        }
    }
    /// Returns the maximum, or `None` if no samples.
    pub fn max(&self) -> Option<f64> {
        if self.count == 0 {
            None
        } else {
            Some(self.max)
        }
    }
    /// Returns the count of recorded samples.
    pub fn count(&self) -> u64 {
        self.count
    }
}
/// A label set for a graph node.
#[allow(dead_code)]
pub struct LabelSet {
    labels: Vec<String>,
}
#[allow(dead_code)]
impl LabelSet {
    /// Creates a new empty label set.
    pub fn new() -> Self {
        Self { labels: Vec::new() }
    }
    /// Adds a label (deduplicates).
    pub fn add(&mut self, label: impl Into<String>) {
        let s = label.into();
        if !self.labels.contains(&s) {
            self.labels.push(s);
        }
    }
    /// Returns `true` if `label` is present.
    pub fn has(&self, label: &str) -> bool {
        self.labels.iter().any(|l| l == label)
    }
    /// Returns the count of labels.
    pub fn count(&self) -> usize {
        self.labels.len()
    }
    /// Returns all labels.
    pub fn all(&self) -> &[String] {
        &self.labels
    }
}
/// A reusable scratch buffer for path computations.
#[allow(dead_code)]
pub struct PathBuf {
    components: Vec<String>,
}
#[allow(dead_code)]
impl PathBuf {
    /// Creates a new empty path buffer.
    pub fn new() -> Self {
        Self {
            components: Vec::new(),
        }
    }
    /// Pushes a component.
    pub fn push(&mut self, comp: impl Into<String>) {
        self.components.push(comp.into());
    }
    /// Pops the last component.
    pub fn pop(&mut self) {
        self.components.pop();
    }
    /// Returns the current path as a `/`-separated string.
    pub fn as_str(&self) -> String {
        self.components.join("/")
    }
    /// Returns the depth of the path.
    pub fn depth(&self) -> usize {
        self.components.len()
    }
    /// Clears the path.
    pub fn clear(&mut self) {
        self.components.clear();
    }
}
/// A type-erased function pointer with arity tracking.
#[allow(dead_code)]
pub struct RawFnPtr {
    /// The raw function pointer (stored as usize for type erasure).
    ptr: usize,
    arity: usize,
    name: String,
}
#[allow(dead_code)]
impl RawFnPtr {
    /// Creates a new raw function pointer descriptor.
    pub fn new(ptr: usize, arity: usize, name: impl Into<String>) -> Self {
        Self {
            ptr,
            arity,
            name: name.into(),
        }
    }
    /// Returns the arity.
    pub fn arity(&self) -> usize {
        self.arity
    }
    /// Returns the name.
    pub fn name(&self) -> &str {
        &self.name
    }
    /// Returns the raw pointer value.
    pub fn raw(&self) -> usize {
        self.ptr
    }
}
/// A tagged union for representing a simple two-case discriminated union.
#[allow(dead_code)]
pub enum Either2<A, B> {
    /// The first alternative.
    First(A),
    /// The second alternative.
    Second(B),
}
#[allow(dead_code)]
impl<A, B> Either2<A, B> {
    /// Returns `true` if this is the first alternative.
    pub fn is_first(&self) -> bool {
        matches!(self, Either2::First(_))
    }
    /// Returns `true` if this is the second alternative.
    pub fn is_second(&self) -> bool {
        matches!(self, Either2::Second(_))
    }
    /// Returns the first value if present.
    pub fn first(self) -> Option<A> {
        match self {
            Either2::First(a) => Some(a),
            _ => None,
        }
    }
    /// Returns the second value if present.
    pub fn second(self) -> Option<B> {
        match self {
            Either2::Second(b) => Some(b),
            _ => None,
        }
    }
    /// Maps over the first alternative.
    pub fn map_first<C, F: FnOnce(A) -> C>(self, f: F) -> Either2<C, B> {
        match self {
            Either2::First(a) => Either2::First(f(a)),
            Either2::Second(b) => Either2::Second(b),
        }
    }
}
