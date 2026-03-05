//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use oxilean_kernel::{BinderInfo, Expr, Level, Literal, Name};

use std::collections::HashMap;

/// A fluent builder for constructing `Expr` values.
///
/// Eliminates boilerplate `Box::new` nesting and provides named
/// constructors for the most common expression forms.
#[derive(Debug, Clone)]
pub struct ExprBuilder {
    expr: Expr,
}
impl ExprBuilder {
    /// Build a `Sort` (universe level expression).
    pub fn sort(level: Level) -> Self {
        Self {
            expr: Expr::Sort(level),
        }
    }
    /// Build `Prop` (Sort 0).
    pub fn prop() -> Self {
        Self::sort(Level::zero())
    }
    /// Build `Type 0` (Sort 1).
    pub fn type0() -> Self {
        Self::sort(Level::succ(Level::zero()))
    }
    /// Build a bound variable `BVar(i)`.
    pub fn bvar(i: u32) -> Self {
        Self {
            expr: Expr::BVar(i),
        }
    }
    /// Build a constant reference.
    pub fn cnst(name: impl Into<String>) -> Self {
        Self {
            expr: Expr::Const(Name::str(name.into()), vec![]),
        }
    }
    /// Build a natural number literal.
    pub fn nat_lit(n: u64) -> Self {
        Self {
            expr: Expr::Lit(Literal::Nat(n)),
        }
    }
    /// Build a string literal.
    pub fn str_lit(s: impl Into<String>) -> Self {
        Self {
            expr: Expr::Lit(Literal::Str(s.into())),
        }
    }
    /// Build `App(f, a)`.
    pub fn app(self, arg: ExprBuilder) -> Self {
        Self {
            expr: Expr::App(Box::new(self.expr), Box::new(arg.expr)),
        }
    }
    /// Apply multiple arguments sequentially.
    pub fn app_many(self, args: impl IntoIterator<Item = ExprBuilder>) -> Self {
        args.into_iter().fold(self, |acc, a| acc.app(a))
    }
    /// Build `Lam(bi, name, ty, body)`.
    pub fn lam(name: impl Into<String>, ty: ExprBuilder, body: ExprBuilder) -> Self {
        Self {
            expr: Expr::Lam(
                BinderInfo::Default,
                Name::str(name.into()),
                Box::new(ty.expr),
                Box::new(body.expr),
            ),
        }
    }
    /// Build an implicit lambda.
    pub fn lam_implicit(name: impl Into<String>, ty: ExprBuilder, body: ExprBuilder) -> Self {
        Self {
            expr: Expr::Lam(
                BinderInfo::Implicit,
                Name::str(name.into()),
                Box::new(ty.expr),
                Box::new(body.expr),
            ),
        }
    }
    /// Build `Pi(bi, name, ty, body)` (dependent arrow type).
    pub fn pi(name: impl Into<String>, ty: ExprBuilder, body: ExprBuilder) -> Self {
        Self {
            expr: Expr::Pi(
                BinderInfo::Default,
                Name::str(name.into()),
                Box::new(ty.expr),
                Box::new(body.expr),
            ),
        }
    }
    /// Build a non-dependent function type `ty → ret`.
    pub fn arrow(ty: ExprBuilder, ret: ExprBuilder) -> Self {
        Self::pi("_", ty, ret)
    }
    /// Build `Let(name, ty, val, body)`.
    pub fn let_bind(
        name: impl Into<String>,
        ty: ExprBuilder,
        val: ExprBuilder,
        body: ExprBuilder,
    ) -> Self {
        Self {
            expr: Expr::Let(
                Name::str(name.into()),
                Box::new(ty.expr),
                Box::new(val.expr),
                Box::new(body.expr),
            ),
        }
    }
    /// Build `Proj(struct_name, field_index, inner)`.
    pub fn proj(struct_name: impl Into<String>, index: u32, inner: ExprBuilder) -> Self {
        Self {
            expr: Expr::Proj(Name::str(struct_name.into()), index, Box::new(inner.expr)),
        }
    }
    /// Consume the builder and produce the expression.
    pub fn build(self) -> Expr {
        self.expr
    }
}
/// A stack of `QuoteScope` entries allowing hierarchical scope management.
#[derive(Clone, Debug, Default)]
pub struct QuoteScopeStack {
    scopes: Vec<QuoteScope>,
}
impl QuoteScopeStack {
    /// Create an empty scope stack.
    pub fn new() -> Self {
        Self::default()
    }
    /// Push a new scope.
    pub fn push(&mut self, scope: QuoteScope) {
        self.scopes.push(scope);
    }
    /// Pop the innermost scope.
    pub fn pop(&mut self) -> Option<QuoteScope> {
        self.scopes.pop()
    }
    /// Peek at the innermost scope without removing it.
    pub fn current(&self) -> Option<&QuoteScope> {
        self.scopes.last()
    }
    /// Return the nesting depth.
    pub fn depth(&self) -> usize {
        self.scopes.len()
    }
    /// Return `true` if no scopes are open.
    pub fn is_empty(&self) -> bool {
        self.scopes.is_empty()
    }
    /// Check whether any ancestor scope is transparent.
    pub fn has_transparent_ancestor(&self) -> bool {
        self.scopes.iter().any(|s| s.is_transparent)
    }
}
/// A full quotation session context: bindings + scope stack + metadata.
#[derive(Clone, Debug, Default)]
pub struct QuoteSession {
    /// Variable binding environment.
    pub env: QuoteEnv,
    /// Hierarchical scope stack.
    pub scopes: QuoteScopeStack,
    /// True when we are inside an anti-quotation splice.
    pub in_splice: bool,
    /// Nesting level of quotation macros.
    pub quote_depth: u32,
}
impl QuoteSession {
    /// Create a fresh empty `QuoteSession`.
    pub fn new() -> Self {
        Self::default()
    }
    /// Enter a quotation level.
    pub fn enter_quote(&mut self) {
        self.quote_depth += 1;
    }
    /// Leave a quotation level.
    pub fn exit_quote(&mut self) {
        self.quote_depth = self.quote_depth.saturating_sub(1);
    }
    /// Enter a splice (anti-quotation).
    pub fn enter_splice(&mut self) {
        self.in_splice = true;
    }
    /// Leave a splice.
    pub fn exit_splice(&mut self) {
        self.in_splice = false;
    }
    /// Bind a variable, returning its de Bruijn depth.
    pub fn bind(&mut self, name: Name, ty: Expr) -> usize {
        self.env.push(name, ty)
    }
    /// Resolve a name to a de Bruijn index, if it is in scope.
    pub fn resolve(&self, name: &Name) -> Option<u32> {
        self.env.lookup(name)
    }
    /// Open a new opaque scope.
    pub fn open_scope(&mut self, label: impl Into<String>) {
        let depth = self.env.depth();
        self.scopes.push(QuoteScope::opaque(label, depth));
    }
    /// Close the innermost scope.
    pub fn close_scope(&mut self) -> Option<QuoteScope> {
        self.scopes.pop()
    }
    /// Return the current de Bruijn depth.
    pub fn depth(&self) -> usize {
        self.env.depth()
    }
}
/// The result of matching a `QuotePattern` against an `Expr`.
#[derive(Clone, Debug, Default)]
pub struct QuoteMatchResult {
    /// Named captures from `QuotePattern::Capture`.
    pub captures: std::collections::HashMap<String, Expr>,
}
impl QuoteMatchResult {
    /// Create a new empty match result.
    pub fn new() -> Self {
        Self::default()
    }
    /// Record a named capture.
    pub fn bind(&mut self, name: impl Into<String>, expr: Expr) {
        self.captures.insert(name.into(), expr);
    }
    /// Look up a captured sub-term.
    pub fn get(&self, name: &str) -> Option<&Expr> {
        self.captures.get(name)
    }
    /// Merge another match result into this one (last write wins).
    pub fn merge(&mut self, other: QuoteMatchResult) {
        self.captures.extend(other.captures);
    }
}
/// A pattern for matching quoted expressions.
#[derive(Debug, Clone, PartialEq)]
pub enum QuotedPattern {
    /// Matches any expression (wildcard).
    Any,
    /// Matches a specific constant name.
    Const(Name),
    /// Matches a bound variable.
    BVar(u32),
    /// Matches an application `f a`.
    App(Box<QuotedPattern>, Box<QuotedPattern>),
    /// Matches a lambda with a body pattern.
    Lam(Box<QuotedPattern>),
    /// Matches a pi type with a body pattern.
    Pi(Box<QuotedPattern>),
    /// Matches a sort.
    Sort,
}
impl QuotedPattern {
    /// Check if this pattern matches the given expression.
    pub fn matches(&self, expr: &Expr) -> bool {
        match (self, expr) {
            (QuotedPattern::Any, _) => true,
            (QuotedPattern::Const(n), Expr::Const(m, _)) => n == m,
            (QuotedPattern::BVar(i), Expr::BVar(j)) => i == j,
            (QuotedPattern::Sort, Expr::Sort(_)) => true,
            (QuotedPattern::App(fp, ap), Expr::App(f, a)) => fp.matches(f) && ap.matches(a),
            (QuotedPattern::Lam(bp), Expr::Lam(_, _, _, body)) => bp.matches(body),
            (QuotedPattern::Pi(bp), Expr::Pi(_, _, _, cod)) => bp.matches(cod),
            _ => false,
        }
    }
    /// Return the head constant name if this is a Const pattern.
    pub fn head_name(&self) -> Option<&Name> {
        if let QuotedPattern::Const(n) = self {
            Some(n)
        } else {
            None
        }
    }
}
/// Fluent builder for constructing `Expr` values.
#[derive(Clone, Debug)]
pub struct QuoteBuilder {
    pub(crate) session: QuoteSession,
}
impl QuoteBuilder {
    /// Create a new builder with an empty session.
    pub fn new() -> Self {
        Self {
            session: QuoteSession::new(),
        }
    }
    /// Introduce a lambda binder and execute `body` in the extended context.
    pub fn lam<F>(&mut self, name: Name, ty: Expr, binder_info: BinderInfo, body_fn: F) -> Expr
    where
        F: FnOnce(&mut Self) -> Expr,
    {
        self.session.bind(name.clone(), ty.clone());
        let b = body_fn(self);
        self.session.env.pop();
        Expr::Lam(binder_info, name, Box::new(ty), Box::new(b))
    }
    /// Build a constant reference.
    pub fn konst(&self, name: Name) -> Expr {
        Expr::Const(name, vec![])
    }
    /// Build a bound-variable reference at a specific index.
    pub fn bvar(&self, idx: u32) -> Expr {
        Expr::BVar(idx)
    }
    /// Build an application.
    pub fn app(&self, f: Expr, a: Expr) -> Expr {
        Expr::App(Box::new(f), Box::new(a))
    }
    /// Build a sort at level zero.
    pub fn sort_zero(&self) -> Expr {
        Expr::Sort(Level::zero())
    }
    /// Resolve a name to a BVar in the current session.
    pub fn resolve(&self, name: &Name) -> Option<Expr> {
        self.session.resolve(name).map(Expr::BVar)
    }
    /// Access the inner session (read-only).
    pub fn session(&self) -> &QuoteSession {
        &self.session
    }
}
/// An environment that keeps track of all bindings introduced during quotation.
#[derive(Clone, Debug, Default)]
pub struct QuoteEnv {
    /// Ordered stack of bindings; last entry is innermost.
    bindings: Vec<QuoteBinding>,
}
impl QuoteEnv {
    /// Create an empty `QuoteEnv`.
    pub fn new() -> Self {
        Self::default()
    }
    /// Push a new binding onto the environment and return the new depth.
    pub fn push(&mut self, name: Name, ty: Expr) -> usize {
        let depth = self.bindings.len();
        self.bindings.push(QuoteBinding::new(name, ty, depth));
        depth
    }
    /// Pop the innermost binding.
    pub fn pop(&mut self) -> Option<QuoteBinding> {
        self.bindings.pop()
    }
    /// Return the current depth (number of binders in scope).
    pub fn depth(&self) -> usize {
        self.bindings.len()
    }
    /// Look up the de Bruijn index for a variable `name` at the current depth.
    pub fn lookup(&self, name: &Name) -> Option<u32> {
        let current = self.bindings.len();
        for (i, b) in self.bindings.iter().enumerate().rev() {
            if &b.name == name {
                return Some((current - i - 1) as u32);
            }
        }
        None
    }
    /// Look up the type of a variable `name`.
    pub fn lookup_type(&self, name: &Name) -> Option<&Expr> {
        for b in self.bindings.iter().rev() {
            if &b.name == name {
                return Some(&b.ty);
            }
        }
        None
    }
    /// Return `true` if no bindings are in scope.
    pub fn is_empty(&self) -> bool {
        self.bindings.is_empty()
    }
    /// Snapshot the current env depth for later restoration.
    pub fn snapshot(&self) -> usize {
        self.bindings.len()
    }
    /// Restore the env to a previously snapshotted depth.
    pub fn restore(&mut self, snapshot: usize) {
        self.bindings.truncate(snapshot);
    }
}
/// Quotation context for tracking quasi-quotation depth.
///
/// Maintains the current nesting level and the list of active splices.
#[derive(Debug, Clone)]
pub struct QuoteContext {
    /// Current quotation depth.
    depth: usize,
    /// Splices at current level.
    splices: Vec<(Name, Expr)>,
    /// Whether strict mode is enabled (reject unbound splices).
    strict: bool,
}
impl QuoteContext {
    /// Create a new quotation context.
    pub fn new() -> Self {
        Self {
            depth: 0,
            splices: Vec::new(),
            strict: false,
        }
    }
    /// Create a strict context that rejects unbound splices.
    pub fn strict() -> Self {
        Self {
            depth: 0,
            splices: Vec::new(),
            strict: true,
        }
    }
    /// Enter a quotation (increase depth).
    pub fn enter_quote(&mut self) {
        self.depth += 1;
    }
    /// Exit a quotation (decrease depth).
    pub fn exit_quote(&mut self) {
        if self.depth > 0 {
            self.depth -= 1;
        }
    }
    /// Add a splice at the current level.
    pub fn add_splice(&mut self, name: Name, expr: Expr) {
        self.splices.push((name, expr));
    }
    /// Remove and return the splice for the given name, if present.
    pub fn take_splice(&mut self, name: &Name) -> Option<Expr> {
        if let Some(idx) = self.splices.iter().position(|(n, _)| n == name) {
            Some(self.splices.remove(idx).1)
        } else {
            None
        }
    }
    /// Get the current quotation depth.
    pub fn depth(&self) -> usize {
        self.depth
    }
    /// Check if we're inside a quotation.
    pub fn is_quoted(&self) -> bool {
        self.depth > 0
    }
    /// Get all splices.
    pub fn splices(&self) -> &[(Name, Expr)] {
        &self.splices
    }
    /// Clear all splices.
    pub fn clear_splices(&mut self) {
        self.splices.clear();
    }
    /// Return whether strict mode is on.
    pub fn is_strict(&self) -> bool {
        self.strict
    }
}
/// A scope entry describing one bound variable inside a quotation context.
#[derive(Clone, Debug)]
pub struct QuoteBinding {
    /// The name the binder introduced.
    pub name: Name,
    /// The type of the bound variable.
    pub ty: Expr,
    /// The de Bruijn depth at which this binder was introduced.
    pub depth: usize,
}
impl QuoteBinding {
    /// Create a new quote binding at the given depth.
    pub fn new(name: Name, ty: Expr, depth: usize) -> Self {
        Self { name, ty, depth }
    }
    /// Returns the current de Bruijn index for this binder at depth `current`.
    pub fn bvar_index(&self, current: usize) -> u32 {
        (current - self.depth - 1) as u32
    }
}
/// A named scope within a quotation environment.
#[derive(Clone, Debug)]
pub struct QuoteScope {
    /// Human-readable name of this scope.
    pub label: String,
    /// The depth of `QuoteEnv` at which this scope begins.
    pub start_depth: usize,
    /// Whether this scope is transparent.
    pub is_transparent: bool,
}
impl QuoteScope {
    /// Create an opaque (non-transparent) scope.
    pub fn opaque(label: impl Into<String>, start_depth: usize) -> Self {
        Self {
            label: label.into(),
            start_depth,
            is_transparent: false,
        }
    }
    /// Create a transparent scope whose bindings are visible to the parent.
    pub fn transparent(label: impl Into<String>, start_depth: usize) -> Self {
        Self {
            label: label.into(),
            start_depth,
            is_transparent: true,
        }
    }
    /// Number of bindings introduced in this scope.
    pub fn bindings_in_scope(&self, current_depth: usize) -> usize {
        current_depth.saturating_sub(self.start_depth)
    }
}
/// A pattern used to deconstruct quoted expressions.
#[derive(Clone, Debug)]
pub enum QuotePattern {
    /// Wildcard: matches anything.
    Any,
    /// Match a specific constant name.
    Const(Name),
    /// Match an application of `head` to `arg`.
    App(Box<QuotePattern>, Box<QuotePattern>),
    /// Match a lambda with optional binder-name constraint.
    Lam(Option<Name>, Box<QuotePattern>),
    /// Match a pi-type.
    Pi(Option<Name>, Box<QuotePattern>, Box<QuotePattern>),
    /// Bind the matched sub-term to a meta-variable name.
    Capture(String, Box<QuotePattern>),
    /// Match a de Bruijn variable at a specific index.
    BVar(u32),
    /// Match any sort.
    AnySort,
}
/// Statistics accumulated over a quotation session.
#[derive(Clone, Debug, Default)]
pub struct QuoteStats {
    /// Number of expressions quoted.
    pub quotes: u64,
    /// Number of successful unquotations.
    pub unquotes_ok: u64,
    /// Number of failed unquotations.
    pub unquotes_err: u64,
    /// Number of pattern matches attempted.
    pub match_attempts: u64,
    /// Number of successful pattern matches.
    pub match_hits: u64,
    /// Number of beta reductions performed by `quote_beta_reduce`.
    pub beta_reductions: u64,
}
impl QuoteStats {
    /// Create zeroed statistics.
    pub fn new() -> Self {
        Self::default()
    }
    /// Record a successful quote.
    pub fn record_quote(&mut self) {
        self.quotes += 1;
    }
    /// Record the outcome of an unquote attempt.
    pub fn record_unquote(&mut self, ok: bool) {
        if ok {
            self.unquotes_ok += 1;
        } else {
            self.unquotes_err += 1;
        }
    }
    /// Record the outcome of a pattern-match attempt.
    pub fn record_match(&mut self, hit: bool) {
        self.match_attempts += 1;
        if hit {
            self.match_hits += 1;
        }
    }
    /// Hit-rate of pattern matches in [0, 1].
    pub fn match_hit_rate(&self) -> f64 {
        if self.match_attempts == 0 {
            1.0
        } else {
            self.match_hits as f64 / self.match_attempts as f64
        }
    }
    /// Produce a one-line summary string.
    pub fn summary(&self) -> String {
        format!(
            "quotes={} unquotes={}/{} matches={}/{} beta={}",
            self.quotes,
            self.unquotes_ok,
            self.unquotes_ok + self.unquotes_err,
            self.match_hits,
            self.match_attempts,
            self.beta_reductions,
        )
    }
}
