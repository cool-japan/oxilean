//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::context::ElabContext;
use oxilean_kernel::{
    instantiate, instantiate_level_params, ConstantInfo, Expr, Level, Name, Reducer,
};

use super::functions::*;

use std::collections::HashMap;

/// A simple type environment mapping variable names to types.
#[derive(Debug, Clone, Default)]
pub struct TypeEnv {
    bindings: Vec<(String, Expr)>,
}
impl TypeEnv {
    /// Create an empty type environment.
    pub fn new() -> Self {
        Self::default()
    }
    /// Push a binding.
    pub fn push(&mut self, name: impl Into<String>, ty: Expr) {
        self.bindings.push((name.into(), ty));
    }
    /// Pop the last binding.
    pub fn pop(&mut self) {
        self.bindings.pop();
    }
    /// Look up a name (innermost binding wins).
    pub fn lookup(&self, name: &str) -> Option<&Expr> {
        self.bindings
            .iter()
            .rev()
            .find_map(|(n, ty)| if n == name { Some(ty) } else { None })
    }
    /// Number of bindings.
    pub fn len(&self) -> usize {
        self.bindings.len()
    }
    /// Whether empty.
    pub fn is_empty(&self) -> bool {
        self.bindings.is_empty()
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InferDecision {
    /// Used the cached result
    CacheHit,
    /// Inferred via rules
    RuleApplied(&'static str),
    /// Deferred to constraint solver
    Deferred,
    /// Failed to infer
    Failed(String),
}
#[allow(dead_code)]
impl InferDecision {
    pub fn is_success(&self) -> bool {
        !matches!(self, InferDecision::Failed(_))
    }
    pub fn rule_name(&self) -> Option<&'static str> {
        if let InferDecision::RuleApplied(name) = self {
            Some(name)
        } else {
            None
        }
    }
}
#[allow(dead_code)]
pub struct UnificationContext {
    subst: MetaVarSubst,
    level_subst: std::collections::HashMap<u64, Level>,
    depth: usize,
    max_depth: usize,
}
#[allow(dead_code)]
impl UnificationContext {
    pub fn new(max_depth: usize) -> Self {
        UnificationContext {
            subst: MetaVarSubst::new(),
            level_subst: std::collections::HashMap::new(),
            depth: 0,
            max_depth,
        }
    }
    pub fn push_depth(&mut self) -> bool {
        if self.depth >= self.max_depth {
            false
        } else {
            self.depth += 1;
            true
        }
    }
    pub fn pop_depth(&mut self) {
        if self.depth > 0 {
            self.depth -= 1;
        }
    }
    pub fn depth(&self) -> usize {
        self.depth
    }
    pub fn assign_meta(&mut self, id: MetaVarId, expr: Expr) {
        self.subst.assign(id, expr);
    }
    pub fn lookup_meta(&self, id: MetaVarId) -> Option<&Expr> {
        self.subst.lookup(id)
    }
    pub fn assign_level(&mut self, id: u64, level: Level) {
        self.level_subst.insert(id, level);
    }
    pub fn lookup_level(&self, id: u64) -> Option<&Level> {
        self.level_subst.get(&id)
    }
    pub fn is_fully_assigned(&self, ids: &[MetaVarId]) -> bool {
        ids.iter().all(|id| self.subst.is_assigned(*id))
    }
    pub fn reset(&mut self) {
        self.subst.clear();
        self.level_subst.clear();
        self.depth = 0;
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TypeInferenceRule {
    Var,
    Const,
    BVar,
    App,
    Lam,
    Pi,
    Let,
    Sort,
    Proj,
    Lit,
}
#[allow(dead_code)]
impl TypeInferenceRule {
    pub fn rule_name(&self) -> &'static str {
        match self {
            TypeInferenceRule::Var => "Var",
            TypeInferenceRule::Const => "Const",
            TypeInferenceRule::BVar => "BVar",
            TypeInferenceRule::App => "App",
            TypeInferenceRule::Lam => "Lam",
            TypeInferenceRule::Pi => "Pi",
            TypeInferenceRule::Let => "Let",
            TypeInferenceRule::Sort => "Sort",
            TypeInferenceRule::Proj => "Proj",
            TypeInferenceRule::Lit => "Lit",
        }
    }
    pub fn applicable_to(&self, expr: &Expr) -> bool {
        matches!(
            (self, expr),
            (TypeInferenceRule::BVar, Expr::BVar(_))
                | (TypeInferenceRule::App, Expr::App(_, _))
                | (TypeInferenceRule::Pi, Expr::Pi(_, _, _, _))
                | (TypeInferenceRule::Lam, Expr::Lam(_, _, _, _))
                | (TypeInferenceRule::Sort, Expr::Sort(_))
        )
    }
    pub fn all_rules() -> Vec<TypeInferenceRule> {
        vec![
            TypeInferenceRule::Var,
            TypeInferenceRule::Const,
            TypeInferenceRule::BVar,
            TypeInferenceRule::App,
            TypeInferenceRule::Lam,
            TypeInferenceRule::Pi,
            TypeInferenceRule::Let,
            TypeInferenceRule::Sort,
            TypeInferenceRule::Proj,
            TypeInferenceRule::Lit,
        ]
    }
}
/// A "fuel" counter for bounding recursive inference.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InferFuel(pub u64);
impl InferFuel {
    /// Create a new fuel counter.
    pub fn new(fuel: u64) -> Self {
        Self(fuel)
    }
    /// Return true if there is any fuel remaining.
    pub fn is_ok(&self) -> bool {
        self.0 > 0
    }
    /// Consume one unit of fuel.
    pub fn consume(&mut self) -> bool {
        if self.0 > 0 {
            self.0 -= 1;
            true
        } else {
            false
        }
    }
}
#[allow(dead_code)]
pub struct TypeAnnotationMap {
    map: std::collections::HashMap<Name, Expr>,
}
#[allow(dead_code)]
impl TypeAnnotationMap {
    pub fn new() -> Self {
        TypeAnnotationMap {
            map: std::collections::HashMap::new(),
        }
    }
    pub fn annotate(&mut self, name: Name, ty: Expr) {
        self.map.insert(name, ty);
    }
    pub fn get_annotation(&self, name: &Name) -> Option<&Expr> {
        self.map.get(name)
    }
    pub fn has_annotation(&self, name: &Name) -> bool {
        self.map.contains_key(name)
    }
    pub fn remove(&mut self, name: &Name) {
        self.map.remove(name);
    }
    pub fn len(&self) -> usize {
        self.map.len()
    }
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
}
/// Priority level for constraint solving.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ConstraintPriority {
    /// Must be solved immediately (blocking).
    Critical = 0,
    /// High priority constraint.
    High = 1,
    /// Normal priority.
    Normal = 2,
    /// Low priority (can be deferred).
    Low = 3,
}
#[allow(dead_code)]
pub struct ExpectedTypeStack {
    stack: Vec<Option<Expr>>,
}
#[allow(dead_code)]
impl ExpectedTypeStack {
    pub fn new() -> Self {
        ExpectedTypeStack { stack: Vec::new() }
    }
    pub fn push(&mut self, expected: Option<Expr>) {
        self.stack.push(expected);
    }
    pub fn pop(&mut self) -> Option<Option<Expr>> {
        self.stack.pop()
    }
    pub fn current(&self) -> Option<&Expr> {
        self.stack.last().and_then(|opt| opt.as_ref())
    }
    pub fn depth(&self) -> usize {
        self.stack.len()
    }
    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }
}
/// Error type for type inference failures.
#[derive(Debug, Clone, PartialEq)]
pub enum InferError {
    /// A free variable was not found in context.
    UnboundFVar(MetaVarId),
    /// A global constant was not found in environment.
    UnknownConst(Name),
    /// Expected a function type but found something else.
    ExpectedFunctionType(String),
    /// Expected a sort (type universe) but found something else.
    ExpectedSort(String),
    /// Projection on non-structure type.
    ProjectionError(String),
    /// Unification failure between two types.
    UnificationFailure(String),
    /// A metavariable was not assigned.
    UnsolvedMetavar(MetaVarId),
    /// Fuel exhausted during inference.
    FuelExhausted,
}
/// Infer result with detailed info.
#[derive(Debug, Clone)]
pub struct DetailedInferResult {
    /// The inferred type.
    pub ty: Expr,
    /// Generated constraints.
    pub constraints: Vec<Constraint>,
    /// Number of inference steps taken.
    pub steps: u32,
    /// Whether inference was cached.
    pub from_cache: bool,
}
impl DetailedInferResult {
    /// Create from a basic InferResult.
    pub fn from_basic(r: InferResult, steps: u32) -> Self {
        Self {
            ty: r.ty,
            constraints: r.constraints,
            steps,
            from_cache: false,
        }
    }
}
#[allow(dead_code)]
pub struct InferExtensionMarker;
/// Result of a bidirectional check.
#[derive(Debug, Clone)]
pub struct BidirResult {
    /// Inferred or checked type.
    pub ty: Expr,
    /// Generated constraints.
    pub constraints: Vec<Constraint>,
    /// Mode in which the check occurred.
    pub mode: BidirMode,
}
impl BidirResult {
    /// Create from infer result.
    pub fn from_infer(ty: Expr, constraints: Vec<Constraint>) -> Self {
        Self {
            ty,
            constraints,
            mode: BidirMode::Infer,
        }
    }
    /// Create from check result.
    pub fn from_check(ty: Expr, constraints: Vec<Constraint>) -> Self {
        Self {
            ty,
            constraints,
            mode: BidirMode::Check,
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct InferStatsExt {
    pub total_inferences: u64,
    pub cache_hits: u64,
    pub metavar_created: u64,
    pub metavar_solved: u64,
    pub constraints_generated: u64,
    pub constraints_solved: u64,
}
#[allow(dead_code)]
impl InferStatsExt {
    pub fn new() -> Self {
        InferStatsExt::default()
    }
    pub fn record_infer(&mut self) {
        self.total_inferences += 1;
    }
    pub fn record_cache_hit(&mut self) {
        self.cache_hits += 1;
    }
    pub fn record_metavar(&mut self) {
        self.metavar_created += 1;
    }
    pub fn record_solved_metavar(&mut self) {
        self.metavar_solved += 1;
    }
    pub fn record_constraint(&mut self) {
        self.constraints_generated += 1;
    }
    pub fn record_solved_constraint(&mut self) {
        self.constraints_solved += 1;
    }
    pub fn unsolved_metavars(&self) -> u64 {
        self.metavar_created.saturating_sub(self.metavar_solved)
    }
    pub fn unsolved_constraints(&self) -> u64 {
        self.constraints_generated
            .saturating_sub(self.constraints_solved)
    }
    pub fn cache_hit_rate(&self) -> f64 {
        if self.total_inferences == 0 {
            0.0
        } else {
            self.cache_hits as f64 / self.total_inferences as f64
        }
    }
    pub fn summary(&self) -> String {
        format!(
            "Infer: {} calls, {} cache hits, {} unsolved MVars, {} unsolved constraints",
            self.total_inferences,
            self.cache_hits,
            self.unsolved_metavars(),
            self.unsolved_constraints()
        )
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum InferErrorKind {
    TypeMismatch { expected: Expr, got: Expr },
    UnboundVariable(usize),
    UnknownConstant(Name),
    MetaVarEscapes(MetaVarId),
    UniverseMismatch(Level, Level),
    NotAFunction(Expr),
    NotASort(Expr),
    RecursionLimit,
    Custom(String),
}
#[allow(dead_code)]
pub struct InferCacheExt {
    entries: std::collections::HashMap<u64, Expr>,
    hit_count: u64,
    miss_count: u64,
}
#[allow(dead_code)]
impl InferCacheExt {
    pub fn new() -> Self {
        InferCacheExt {
            entries: std::collections::HashMap::new(),
            hit_count: 0,
            miss_count: 0,
        }
    }
    pub fn get(&mut self, key: u64) -> Option<&Expr> {
        if let Some(val) = self.entries.get(&key) {
            self.hit_count += 1;
            Some(val)
        } else {
            self.miss_count += 1;
            None
        }
    }
    pub fn insert(&mut self, key: u64, ty: Expr) {
        self.entries.insert(key, ty);
    }
    pub fn invalidate(&mut self, key: u64) {
        self.entries.remove(&key);
    }
    pub fn clear(&mut self) {
        self.entries.clear();
    }
    pub fn hit_rate(&self) -> f64 {
        let total = self.hit_count + self.miss_count;
        if total == 0 {
            0.0
        } else {
            self.hit_count as f64 / total as f64
        }
    }
    pub fn size(&self) -> usize {
        self.entries.len()
    }
}
#[allow(dead_code)]
pub struct MetaVarSubst {
    subst: std::collections::HashMap<MetaVarId, Expr>,
}
#[allow(dead_code)]
impl MetaVarSubst {
    pub fn new() -> Self {
        MetaVarSubst {
            subst: std::collections::HashMap::new(),
        }
    }
    pub fn assign(&mut self, id: MetaVarId, expr: Expr) {
        self.subst.insert(id, expr);
    }
    pub fn lookup(&self, id: MetaVarId) -> Option<&Expr> {
        self.subst.get(&id)
    }
    pub fn is_assigned(&self, id: MetaVarId) -> bool {
        self.subst.contains_key(&id)
    }
    pub fn unassigned_count(&self, all_ids: &[MetaVarId]) -> usize {
        all_ids.iter().filter(|id| !self.is_assigned(**id)).count()
    }
    pub fn len(&self) -> usize {
        self.subst.len()
    }
    pub fn is_empty(&self) -> bool {
        self.subst.is_empty()
    }
    pub fn all_ids(&self) -> Vec<MetaVarId> {
        self.subst.keys().copied().collect()
    }
    pub fn clear(&mut self) {
        self.subst.clear();
    }
}
/// Statistics about type inference.
#[derive(Debug, Clone, Default)]
pub struct InferStats {
    /// Number of successful inferences.
    pub infer_count: u64,
    /// Number of constraints generated.
    pub constraint_count: u64,
    /// Number of metavariables created.
    pub metavar_count: u64,
    /// Number of cache hits.
    pub cache_hits: u64,
    /// Number of cache misses.
    pub cache_misses: u64,
}
impl InferStats {
    /// Create new zeroed statistics.
    pub fn new() -> Self {
        Self::default()
    }
    /// Record one inference.
    pub fn record_infer(&mut self) {
        self.infer_count += 1;
    }
    /// Record a new constraint.
    pub fn record_constraint(&mut self) {
        self.constraint_count += 1;
    }
    /// Record a new metavariable.
    pub fn record_metavar(&mut self) {
        self.metavar_count += 1;
    }
    /// Record a cache hit.
    pub fn record_cache_hit(&mut self) {
        self.cache_hits += 1;
    }
    /// Record a cache miss.
    pub fn record_cache_miss(&mut self) {
        self.cache_misses += 1;
    }
    /// Return the hit rate as a fraction.
    pub fn hit_rate(&self) -> f64 {
        let total = self.cache_hits + self.cache_misses;
        if total == 0 {
            0.0
        } else {
            self.cache_hits as f64 / total as f64
        }
    }
}
/// A type annotation hint provided by the user.
#[derive(Debug, Clone)]
pub struct TypeAnnotation {
    /// The annotated type.
    pub ty: Expr,
    /// The expression being annotated.
    pub expr: Expr,
}
impl TypeAnnotation {
    /// Create a new type annotation.
    pub fn new(expr: Expr, ty: Expr) -> Self {
        Self { ty, expr }
    }
    /// Generate the equality constraint `inferred == annotated`.
    pub fn to_constraint(&self, inferred: Expr) -> Constraint {
        Constraint::Equal(inferred, self.ty.clone())
    }
}
/// Manage a pool of type annotations for the current scope.
#[derive(Debug, Default)]
pub struct AnnotationPool {
    annotations: Vec<TypeAnnotation>,
}
impl AnnotationPool {
    /// Create an empty pool.
    pub fn new() -> Self {
        Self::default()
    }
    /// Add an annotation.
    pub fn push(&mut self, ann: TypeAnnotation) {
        self.annotations.push(ann);
    }
    /// Pop the last annotation.
    pub fn pop(&mut self) -> Option<TypeAnnotation> {
        self.annotations.pop()
    }
    /// Peek at the last annotation.
    pub fn peek(&self) -> Option<&TypeAnnotation> {
        self.annotations.last()
    }
    /// Return the number of annotations.
    pub fn len(&self) -> usize {
        self.annotations.len()
    }
    /// Whether the pool is empty.
    pub fn is_empty(&self) -> bool {
        self.annotations.is_empty()
    }
}
/// A constraint simplifier that rewrites constraints using known assignments.
pub struct ConstraintSimplifier {
    #[allow(dead_code)]
    assignments: std::collections::HashMap<MetaVarId, Expr>,
}
impl ConstraintSimplifier {
    /// Create a new simplifier with known assignments.
    pub fn new(assignments: std::collections::HashMap<MetaVarId, Expr>) -> Self {
        Self { assignments }
    }
    /// Simplify a constraint using known assignments.
    pub fn simplify(&self, c: &Constraint) -> Constraint {
        match c {
            Constraint::Equal(e1, e2) => {
                Constraint::Equal(self.apply_assignments(e1), self.apply_assignments(e2))
            }
            Constraint::HasType(e, ty) => {
                Constraint::HasType(self.apply_assignments(e), self.apply_assignments(ty))
            }
            Constraint::Assign(id, e) => Constraint::Assign(*id, self.apply_assignments(e)),
        }
    }
    fn apply_assignments(&self, expr: &Expr) -> Expr {
        match expr {
            Expr::FVar(fv) if fv.0 >= METAVAR_BASE => {
                let meta_id = fv.0 - METAVAR_BASE;
                if let Some(val) = self.assignments.get(&meta_id) {
                    self.apply_assignments(val)
                } else {
                    expr.clone()
                }
            }
            Expr::App(f, a) => Expr::App(
                Box::new(self.apply_assignments(f)),
                Box::new(self.apply_assignments(a)),
            ),
            Expr::Lam(bi, name, ty, body) => Expr::Lam(
                *bi,
                name.clone(),
                Box::new(self.apply_assignments(ty)),
                Box::new(self.apply_assignments(body)),
            ),
            Expr::Pi(bi, name, ty, body) => Expr::Pi(
                *bi,
                name.clone(),
                Box::new(self.apply_assignments(ty)),
                Box::new(self.apply_assignments(body)),
            ),
            Expr::Let(name, ty, val, body) => Expr::Let(
                name.clone(),
                Box::new(self.apply_assignments(ty)),
                Box::new(self.apply_assignments(val)),
                Box::new(self.apply_assignments(body)),
            ),
            Expr::Proj(name, idx, inner) => {
                Expr::Proj(name.clone(), *idx, Box::new(self.apply_assignments(inner)))
            }
            _ => expr.clone(),
        }
    }
    /// Simplify a list of constraints.
    pub fn simplify_all(&self, cs: &[Constraint]) -> Vec<Constraint> {
        cs.iter().map(|c| self.simplify(c)).collect()
    }
}
#[allow(dead_code)]
pub struct SimpleConstraintSolver {
    constraints: Vec<Constraint>,
    solved: Vec<(Expr, Expr)>,
    max_iterations: usize,
}
#[allow(dead_code)]
impl SimpleConstraintSolver {
    pub fn new() -> Self {
        SimpleConstraintSolver {
            constraints: Vec::new(),
            solved: Vec::new(),
            max_iterations: 1000,
        }
    }
    pub fn with_max_iterations(mut self, n: usize) -> Self {
        self.max_iterations = n;
        self
    }
    pub fn add_constraint(&mut self, c: Constraint) {
        self.constraints.push(c);
    }
    pub fn add_constraints(&mut self, cs: Vec<Constraint>) {
        self.constraints.extend(cs);
    }
    pub fn pending_count(&self) -> usize {
        self.constraints.len()
    }
    pub fn solved_count(&self) -> usize {
        self.solved.len()
    }
    pub fn is_complete(&self) -> bool {
        self.constraints.is_empty()
    }
    /// Attempt a single solving step. Returns number of constraints resolved.
    pub fn step(&mut self) -> usize {
        let initial = self.constraints.len();
        let remaining: Vec<Constraint> = std::mem::take(&mut self.constraints)
            .into_iter()
            .filter(|c| if self.try_solve(c) { false } else { true })
            .collect();
        self.constraints = remaining;
        initial - self.constraints.len()
    }
    fn try_solve(&mut self, c: &Constraint) -> bool {
        match c {
            Constraint::Equal(a, b) => {
                if format!("{:?}", a) == format!("{:?}", b) {
                    self.solved.push((a.clone(), b.clone()));
                    true
                } else {
                    false
                }
            }
            Constraint::HasType(_, _) => false,
            Constraint::Assign(_, _) => false,
        }
    }
    pub fn solve_all(&mut self) -> SolveResult {
        for _ in 0..self.max_iterations {
            if self.is_complete() {
                return SolveResult::Solved;
            }
            let resolved = self.step();
            if resolved == 0 {
                return SolveResult::Stuck;
            }
        }
        SolveResult::Stuck
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SolveResult {
    Solved,
    Stuck,
    Contradiction,
}
#[allow(dead_code)]
pub struct InferLogger {
    entries: Vec<(Expr, InferDecision)>,
    max_entries: usize,
}
#[allow(dead_code)]
impl InferLogger {
    pub fn new(max_entries: usize) -> Self {
        InferLogger {
            entries: Vec::new(),
            max_entries,
        }
    }
    pub fn log(&mut self, expr: Expr, decision: InferDecision) {
        if self.entries.len() >= self.max_entries {
            self.entries.remove(0);
        }
        self.entries.push((expr, decision));
    }
    pub fn failures(&self) -> Vec<&(Expr, InferDecision)> {
        self.entries
            .iter()
            .filter(|(_, d)| !d.is_success())
            .collect()
    }
    pub fn count_rule(&self, rule: &'static str) -> usize {
        self.entries
            .iter()
            .filter(|(_, d)| d.rule_name() == Some(rule))
            .count()
    }
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}
/// Bidirectional typing mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BidirMode {
    /// Inferring the type of an expression.
    Infer,
    /// Checking that an expression has a given type.
    Check,
}
#[allow(dead_code)]
pub struct InferErrorCollector {
    errors: Vec<InferErrorKind>,
    max_errors: usize,
}
#[allow(dead_code)]
impl InferErrorCollector {
    pub fn new(max_errors: usize) -> Self {
        InferErrorCollector {
            errors: Vec::new(),
            max_errors,
        }
    }
    pub fn add(&mut self, err: InferErrorKind) {
        if self.errors.len() < self.max_errors {
            self.errors.push(err);
        }
    }
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }
    pub fn errors(&self) -> &[InferErrorKind] {
        &self.errors
    }
    pub fn count(&self) -> usize {
        self.errors.len()
    }
    pub fn is_saturated(&self) -> bool {
        self.errors.len() >= self.max_errors
    }
    pub fn first_error(&self) -> Option<&InferErrorKind> {
        self.errors.first()
    }
}
#[allow(dead_code)]
pub struct BiDirectionalInferState {
    mode: InferMode,
    expected_type: Option<Expr>,
    inferred_type: Option<Expr>,
    decisions: Vec<InferDecision>,
}
#[allow(dead_code)]
impl BiDirectionalInferState {
    pub fn infer_mode() -> Self {
        BiDirectionalInferState {
            mode: InferMode::Infer,
            expected_type: None,
            inferred_type: None,
            decisions: Vec::new(),
        }
    }
    pub fn check_mode(expected: Expr) -> Self {
        BiDirectionalInferState {
            mode: InferMode::Check,
            expected_type: Some(expected),
            inferred_type: None,
            decisions: Vec::new(),
        }
    }
    pub fn ascribed_mode(ty: Expr) -> Self {
        BiDirectionalInferState {
            mode: InferMode::Ascribed,
            expected_type: Some(ty),
            inferred_type: None,
            decisions: Vec::new(),
        }
    }
    pub fn set_inferred(&mut self, ty: Expr) {
        self.inferred_type = Some(ty);
    }
    pub fn record_decision(&mut self, d: InferDecision) {
        self.decisions.push(d);
    }
    pub fn mode(&self) -> InferMode {
        self.mode
    }
    pub fn expected_type(&self) -> Option<&Expr> {
        self.expected_type.as_ref()
    }
    pub fn inferred_type(&self) -> Option<&Expr> {
        self.inferred_type.as_ref()
    }
    pub fn decisions(&self) -> &[InferDecision] {
        &self.decisions
    }
    pub fn is_done(&self) -> bool {
        self.inferred_type.is_some()
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct InferSessionConfig {
    pub max_unification_depth: usize,
    pub max_infer_depth: usize,
    pub enable_cache: bool,
    pub enable_logging: bool,
    pub universe_polymorphism: bool,
    pub implicit_lambdas: bool,
}
#[allow(dead_code)]
impl InferSessionConfig {
    pub fn new() -> Self {
        InferSessionConfig {
            max_unification_depth: 100,
            max_infer_depth: 500,
            enable_cache: true,
            enable_logging: false,
            universe_polymorphism: true,
            implicit_lambdas: true,
        }
    }
    pub fn without_cache(mut self) -> Self {
        self.enable_cache = false;
        self
    }
    pub fn with_logging(mut self) -> Self {
        self.enable_logging = true;
        self
    }
    pub fn with_max_depth(mut self, depth: usize) -> Self {
        self.max_infer_depth = depth;
        self
    }
    pub fn no_implicit_lambdas(mut self) -> Self {
        self.implicit_lambdas = false;
        self
    }
}
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct InferRuleStats {
    rule_counts: std::collections::HashMap<String, u64>,
    rule_failures: std::collections::HashMap<String, u64>,
}
#[allow(dead_code)]
impl InferRuleStats {
    pub fn new() -> Self {
        InferRuleStats::default()
    }
    pub fn record_success(&mut self, rule: &TypeInferenceRule) {
        *self
            .rule_counts
            .entry(rule.rule_name().to_string())
            .or_insert(0) += 1;
    }
    pub fn record_failure(&mut self, rule: &TypeInferenceRule) {
        *self
            .rule_failures
            .entry(rule.rule_name().to_string())
            .or_insert(0) += 1;
    }
    pub fn success_count(&self, rule: &TypeInferenceRule) -> u64 {
        self.rule_counts.get(rule.rule_name()).copied().unwrap_or(0)
    }
    pub fn failure_count(&self, rule: &TypeInferenceRule) -> u64 {
        self.rule_failures
            .get(rule.rule_name())
            .copied()
            .unwrap_or(0)
    }
    pub fn most_used_rule(&self) -> Option<(&String, u64)> {
        self.rule_counts
            .iter()
            .max_by_key(|(_, v)| *v)
            .map(|(k, v)| (k, *v))
    }
    pub fn total_invocations(&self) -> u64 {
        self.rule_counts.values().sum()
    }
    pub fn total_failures(&self) -> u64 {
        self.rule_failures.values().sum()
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct InferHint {
    pub prefer_prop: bool,
    pub allow_metavars: bool,
    pub unfold_depth: u32,
    pub universe_polymorphic: bool,
}
#[allow(dead_code)]
impl InferHint {
    pub fn new() -> Self {
        InferHint {
            prefer_prop: false,
            allow_metavars: true,
            unfold_depth: 5,
            universe_polymorphic: false,
        }
    }
    pub fn prefer_prop(mut self) -> Self {
        self.prefer_prop = true;
        self
    }
    pub fn no_metavars(mut self) -> Self {
        self.allow_metavars = false;
        self
    }
    pub fn with_unfold_depth(mut self, depth: u32) -> Self {
        self.unfold_depth = depth;
        self
    }
    pub fn polymorphic(mut self) -> Self {
        self.universe_polymorphic = true;
        self
    }
}
/// Type inference engine.
pub struct TypeInferencer<'env> {
    /// Elaboration context
    ctx: &'env mut ElabContext<'env>,
    /// Accumulated constraints
    constraints: Vec<Constraint>,
}
impl<'env> TypeInferencer<'env> {
    /// Create a new type inferencer.
    pub fn new(ctx: &'env mut ElabContext<'env>) -> Self {
        Self {
            ctx,
            constraints: Vec::new(),
        }
    }
    /// Infer the type of an expression.
    pub fn infer(&mut self, expr: &Expr) -> Result<Expr, String> {
        match expr {
            Expr::Sort(level) => Ok(Expr::Sort(Level::succ(level.clone()))),
            Expr::BVar(idx) => Err(format!("Unbound variable #{}", idx)),
            Expr::FVar(fvar) => {
                if let Some(entry) = self.ctx.lookup_fvar(*fvar) {
                    Ok(entry.ty.clone())
                } else {
                    Err(format!("Free variable @{} not found", fvar.0))
                }
            }
            Expr::Const(name, levels) => {
                if let Some(ci) = self.ctx.env().find(name) {
                    let ty = self.instantiate_univs(ci.ty(), ci.level_params(), levels)?;
                    return Ok(ty);
                }
                if let Some(decl) = self.ctx.env().get(name) {
                    let ty = self.instantiate_univs(decl.ty(), decl.univ_params(), levels)?;
                    return Ok(ty);
                }
                Err(format!("Constant {} not found", name))
            }
            Expr::Lam(info, name, ty, body) => {
                let _fvar = self.ctx.push_local(name.clone(), ty.as_ref().clone(), None);
                let body_ty = self.infer(body)?;
                self.ctx.pop_local();
                Ok(Expr::Pi(
                    *info,
                    name.clone(),
                    Box::new(ty.as_ref().clone()),
                    Box::new(body_ty),
                ))
            }
            Expr::Pi(_, _, ty, body) => {
                let ty_ty = self.infer(ty)?;
                self.ensure_type(&ty_ty)?;
                let body_ty = self.infer(body)?;
                self.ensure_type(&body_ty)?;
                Ok(self.max_sort(&ty_ty, &body_ty))
            }
            Expr::App(f, a) => {
                let f_ty = self.infer(f)?;
                let a_ty = self.infer(a)?;
                if let Expr::Pi(_, _, param_ty, result_ty) = f_ty {
                    self.constraints
                        .push(Constraint::Equal(a_ty, param_ty.as_ref().clone()));
                    Ok(result_ty.as_ref().clone())
                } else {
                    Err(format!("Expected function type, got {:?}", f_ty))
                }
            }
            Expr::Let(name, ty, val, body) => {
                let val_ty = self.infer(val)?;
                self.constraints
                    .push(Constraint::Equal(val_ty, ty.as_ref().clone()));
                let _fvar = self.ctx.push_local(
                    name.clone(),
                    ty.as_ref().clone(),
                    Some(val.as_ref().clone()),
                );
                let body_ty = self.infer(body)?;
                self.ctx.pop_local();
                Ok(body_ty)
            }
            Expr::Lit(lit) => {
                use oxilean_kernel::Literal;
                match lit {
                    Literal::Nat(_) => Ok(Expr::Const(Name::str("Nat"), vec![])),
                    Literal::Str(_) => Ok(Expr::Const(Name::str("String"), vec![])),
                }
            }
            Expr::Proj(struct_name, idx, inner) => self.infer_proj(struct_name, *idx, inner),
        }
    }
    /// Ensure an expression is a type (sort).
    fn ensure_type(&self, expr: &Expr) -> Result<(), String> {
        if matches!(expr, Expr::Sort(_)) {
            Ok(())
        } else {
            Err(format!("Expected type, got {:?}", expr))
        }
    }
    /// Compute maximum of two sorts.
    fn max_sort(&self, s1: &Expr, s2: &Expr) -> Expr {
        match (s1, s2) {
            (Expr::Sort(l1), Expr::Sort(l2)) => Expr::Sort(Level::max(l1.clone(), l2.clone())),
            _ => Expr::Sort(Level::zero()),
        }
    }
    /// Instantiate universe parameters.
    fn instantiate_univs(
        &self,
        ty: &Expr,
        level_params: &[Name],
        levels: &[Level],
    ) -> Result<Expr, String> {
        if level_params.is_empty() || levels.is_empty() {
            return Ok(ty.clone());
        }
        Ok(instantiate_level_params(ty, level_params, levels))
    }
    /// Infer the type of a projection `struct_name.idx inner`.
    fn infer_proj(&mut self, struct_name: &Name, idx: u32, inner: &Expr) -> Result<Expr, String> {
        let ind_val = match self.ctx.env().find(struct_name) {
            Some(ConstantInfo::Inductive(iv)) => iv.clone(),
            _ => {
                return Err(format!("Projection on non-inductive type: {}", struct_name));
            }
        };
        if ind_val.ctors.len() != 1 {
            return Err(format!(
                "Projection on non-structure inductive {} (has {} constructors)",
                struct_name,
                ind_val.ctors.len()
            ));
        }
        let ctor_name = ind_val.ctors[0].clone();
        let ctor_val = match self.ctx.env().find(&ctor_name) {
            Some(ConstantInfo::Constructor(cv)) => cv.clone(),
            _ => {
                return Err(format!(
                    "Constructor {} not found in environment",
                    ctor_name
                ));
            }
        };
        if idx >= ctor_val.num_fields {
            return Err(format!(
                "field index {} out of range for {} (has {} fields)",
                idx, struct_name, ctor_val.num_fields
            ));
        }
        let struct_ty = self.infer(inner)?;
        let env = self.ctx.env();
        let struct_ty_whnf = {
            let mut reducer = Reducer::new();
            reducer.whnf_env(&struct_ty, env)
        };
        let levels: Vec<Level> = match oxilean_kernel::expr_util::get_app_fn(&struct_ty_whnf) {
            Expr::Const(_, lvls) => lvls.clone(),
            _ => vec![],
        };
        let level_params = ind_val.common.level_params.clone();
        let ctor_ty = ctor_val.common.ty.clone();
        let mut cur_ty = instantiate_level_params(&ctor_ty, &level_params, &levels);
        let struct_args: Vec<Expr> = oxilean_kernel::expr_util::get_app_args(&struct_ty_whnf)
            .into_iter()
            .cloned()
            .collect();
        for i in 0..ind_val.num_params as usize {
            match cur_ty {
                Expr::Pi(_, _, _, body) => {
                    let param = struct_args.get(i).cloned().unwrap_or(Expr::BVar(0));
                    cur_ty = instantiate(&body, &param);
                }
                _ => {
                    return Err(format!(
                        "Constructor type too short while peeling params for {}",
                        struct_name
                    ));
                }
            }
        }
        for j in 0..idx {
            match cur_ty {
                Expr::Pi(_, _, _, body) => {
                    let field_val =
                        Expr::Proj(ind_val.common.name.clone(), j, Box::new(inner.clone()));
                    cur_ty = instantiate(&body, &field_val);
                }
                _ => {
                    return Err(format!(
                        "Constructor type too short while peeling fields for {}",
                        struct_name
                    ));
                }
            }
        }
        match cur_ty {
            Expr::Pi(_, _, dom, _) => Ok(*dom),
            _ => Err(format!(
                "Expected Pi type for field {} of {}",
                idx, struct_name
            )),
        }
    }
    /// Get accumulated constraints.
    pub fn constraints(&self) -> &[Constraint] {
        &self.constraints
    }
    /// Clear constraints.
    pub fn clear_constraints(&mut self) {
        self.constraints.clear();
    }
}
/// Type constraint for unification.
#[derive(Debug, Clone, PartialEq)]
pub enum Constraint {
    /// e1 = e2 (definitional equality)
    Equal(Expr, Expr),
    /// e1 : e2 (typing constraint)
    HasType(Expr, Expr),
    /// m := e (metavariable assignment)
    Assign(MetaVarId, Expr),
}
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InferMode {
    /// Infer mode: synthesize a type for the expression
    Infer,
    /// Check mode: verify expression has the given type
    Check,
    /// Ascription mode: user provided an explicit type
    Ascribed,
}
/// Direction of a type check.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CheckDirection {
    /// Checking that an expression has a specific type.
    Check,
    /// Inferring the type of an expression.
    Infer,
    /// Synthesizing a term from a type.
    Synth,
}
/// A simple inference cache mapping expression hashes to inferred types.
#[derive(Debug, Default)]
pub struct InferCache {
    entries: std::collections::HashMap<u64, Expr>,
}
impl InferCache {
    /// Create a new empty cache.
    pub fn new() -> Self {
        Self::default()
    }
    /// Look up a cached type for an expression ID.
    pub fn get(&self, id: u64) -> Option<&Expr> {
        self.entries.get(&id)
    }
    /// Insert a cached type.
    pub fn insert(&mut self, id: u64, ty: Expr) {
        self.entries.insert(id, ty);
    }
    /// Clear the cache.
    pub fn clear(&mut self) {
        self.entries.clear();
    }
    /// Return the number of cached entries.
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    /// Return true if the cache is empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}
/// A solver state for incrementally solving type constraints.
#[derive(Debug)]
pub struct ConstraintSolver {
    /// Pending constraints.
    pending: Vec<Constraint>,
    /// Solved assignments (metavar id -> expr).
    solved: std::collections::HashMap<MetaVarId, Expr>,
    /// Errors encountered.
    errors: Vec<String>,
}
impl ConstraintSolver {
    /// Create a new constraint solver.
    pub fn new() -> Self {
        Self {
            pending: Vec::new(),
            solved: std::collections::HashMap::new(),
            errors: Vec::new(),
        }
    }
    /// Add a constraint to solve.
    pub fn add(&mut self, c: Constraint) {
        self.pending.push(c);
    }
    /// Add multiple constraints.
    pub fn add_all(&mut self, cs: impl IntoIterator<Item = Constraint>) {
        self.pending.extend(cs);
    }
    /// Check if solving is complete (no pending constraints).
    pub fn is_done(&self) -> bool {
        self.pending.is_empty()
    }
    /// Get the solved assignment for a metavariable.
    pub fn get_assignment(&self, id: MetaVarId) -> Option<&Expr> {
        self.solved.get(&id)
    }
    /// Get all errors.
    pub fn errors(&self) -> &[String] {
        &self.errors
    }
    /// Perform one round of constraint solving.
    ///
    /// Returns the number of constraints solved.
    pub fn solve_step(&mut self) -> usize {
        let mut solved_count = 0;
        let mut remaining = Vec::new();
        for c in std::mem::take(&mut self.pending) {
            match &c {
                Constraint::Assign(id, val) => {
                    self.solved.insert(*id, val.clone());
                    solved_count += 1;
                }
                Constraint::Equal(e1, e2) if e1 == e2 => {
                    solved_count += 1;
                }
                _ => remaining.push(c),
            }
        }
        self.pending = remaining;
        solved_count
    }
    /// Solve all constraints iteratively.
    pub fn solve_all(&mut self) -> bool {
        loop {
            if self.is_done() {
                return true;
            }
            let solved = self.solve_step();
            if solved == 0 {
                return self.pending.is_empty();
            }
        }
    }
    /// Return the number of pending constraints.
    pub fn pending_count(&self) -> usize {
        self.pending.len()
    }
    /// Return the number of solved assignments.
    pub fn solved_count(&self) -> usize {
        self.solved.len()
    }
}
/// A constraint with an associated priority.
#[derive(Debug, Clone)]
pub struct PrioritizedConstraint {
    /// The constraint itself.
    pub constraint: Constraint,
    /// Priority for solving order.
    pub priority: ConstraintPriority,
    /// Optional source location tag.
    pub tag: Option<String>,
}
impl PrioritizedConstraint {
    /// Create a new prioritized constraint.
    pub fn new(constraint: Constraint, priority: ConstraintPriority) -> Self {
        Self {
            constraint,
            priority,
            tag: None,
        }
    }
    /// Attach a tag for debugging.
    pub fn with_tag(mut self, tag: impl Into<String>) -> Self {
        self.tag = Some(tag.into());
        self
    }
}
/// Type inference result.
#[derive(Debug, Clone)]
pub struct InferResult {
    /// Inferred type
    pub ty: Expr,
    /// Generated constraints
    pub constraints: Vec<Constraint>,
}
