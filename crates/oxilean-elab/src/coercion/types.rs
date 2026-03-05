//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use oxilean_kernel::{BinderInfo, Expr, Level, Name};
use std::collections::{HashMap, HashSet, VecDeque};

/// Statistics about coercions inserted during elaboration.
#[derive(Clone, Debug, Default)]
pub struct CoercionStats {
    /// Number of successful coercions inserted.
    pub inserted: u64,
    /// Number of failed coercion lookups.
    pub failed: u64,
    /// Number of chained (multi-step) coercions used.
    pub chained: u64,
    /// Number of sort coercions.
    pub sort_coercions: u64,
}
impl CoercionStats {
    /// Create zeroed stats.
    pub fn new() -> Self {
        Self::default()
    }
    /// Record a successful single-step coercion.
    pub fn record_inserted(&mut self) {
        self.inserted += 1;
    }
    /// Record a failed coercion lookup.
    pub fn record_failed(&mut self) {
        self.failed += 1;
    }
    /// Record a chained coercion.
    pub fn record_chained(&mut self) {
        self.inserted += 1;
        self.chained += 1;
    }
    /// Record a sort coercion.
    pub fn record_sort(&mut self) {
        self.inserted += 1;
        self.sort_coercions += 1;
    }
    /// Total attempts (inserted + failed).
    pub fn total_attempts(&self) -> u64 {
        self.inserted + self.failed
    }
    /// Success rate in [0, 1].
    pub fn success_rate(&self) -> f64 {
        let total = self.total_attempts() as f64;
        if total == 0.0 {
            1.0
        } else {
            self.inserted as f64 / total
        }
    }
}
/// A cache for coercion path lookup results.
#[derive(Debug, Default)]
pub struct CoercionCache {
    entries: HashMap<String, Option<CoercionPath>>,
    hits: usize,
    misses: usize,
}
impl CoercionCache {
    /// Create an empty cache.
    pub fn new() -> Self {
        Self::default()
    }
    /// Compute the cache key for a (from, to) pair.
    fn key(from: &Expr, to: &Expr) -> String {
        format!("{}->{}", coercion_type_key(from), coercion_type_key(to))
    }
    /// Look up a cached path.
    pub fn get(&mut self, from: &Expr, to: &Expr) -> Option<&Option<CoercionPath>> {
        let key = Self::key(from, to);
        if let Some(entry) = self.entries.get(&key) {
            self.hits += 1;
            Some(unsafe { &*(entry as *const Option<CoercionPath>) })
        } else {
            self.misses += 1;
            None
        }
    }
    /// Cache a path result.
    pub fn store(&mut self, from: &Expr, to: &Expr, path: Option<CoercionPath>) {
        let key = Self::key(from, to);
        self.entries.insert(key, path);
    }
    /// Return the cache hit count.
    pub fn hits(&self) -> usize {
        self.hits
    }
    /// Return the cache miss count.
    pub fn misses(&self) -> usize {
        self.misses
    }
    /// Return the number of cached entries.
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    /// Return true if the cache is empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
    /// Clear the cache.
    pub fn clear(&mut self) {
        self.entries.clear();
        self.hits = 0;
        self.misses = 0;
    }
    /// Return the hit rate in [0, 1].
    pub fn hit_rate(&self) -> f64 {
        let total = self.hits + self.misses;
        if total == 0 {
            0.0
        } else {
            self.hits as f64 / total as f64
        }
    }
}
#[allow(dead_code)]
pub struct CoercionPrettyPrinter {
    indent: usize,
    show_priority: bool,
    show_kind: bool,
}
#[allow(dead_code)]
impl CoercionPrettyPrinter {
    pub fn new() -> Self {
        CoercionPrettyPrinter {
            indent: 0,
            show_priority: true,
            show_kind: true,
        }
    }
    pub fn with_indent(mut self, indent: usize) -> Self {
        self.indent = indent;
        self
    }
    pub fn without_priority(mut self) -> Self {
        self.show_priority = false;
        self
    }
    pub fn without_kind(mut self) -> Self {
        self.show_kind = false;
        self
    }
    fn indent_str(&self) -> String {
        " ".repeat(self.indent)
    }
    pub fn print_coercion(&self, c: &Coercion) -> String {
        let mut parts = vec![format!("{}`{}`", self.indent_str(), c.coerce)];
        if self.show_kind {
            parts.push(format!("kind={:?}", c.kind));
        }
        if self.show_priority {
            parts.push(format!("priority={}", c.priority));
        }
        if c.is_instance {
            parts.push("instance".to_string());
        }
        parts.join(" ")
    }
    pub fn print_registry(&self, registry: &CoercionRegistry) -> String {
        let mut out = String::new();
        out.push_str(&format!(
            "{}CoercionRegistry ({} coercions):\n",
            self.indent_str(),
            registry.coercions.len()
        ));
        let pp = CoercionPrettyPrinter::new().with_indent(self.indent + 2);
        for c in &registry.coercions {
            out.push_str(&pp.print_coercion(c));
            out.push('\n');
        }
        out
    }
    pub fn print_graph(&self, graph: &CoercionGraph) -> String {
        let mut out = String::new();
        out.push_str(&format!(
            "{}CoercionGraph ({} nodes, {} edges):\n",
            self.indent_str(),
            graph.node_count(),
            graph.edge_count()
        ));
        out
    }
}
/// A coercion between function types `(A → B)` and `(A' → B')`.
#[derive(Debug, Clone)]
pub struct FunctionCoercion {
    /// Domain coercion (A' → A).
    pub domain_coerce: Option<Box<Coercion>>,
    /// Codomain coercion (B → B').
    pub codomain_coerce: Option<Box<Coercion>>,
    /// Name of the combined function coercion.
    pub name: Name,
}
impl FunctionCoercion {
    /// Create a new function coercion.
    pub fn new(name: Name) -> Self {
        Self {
            domain_coerce: None,
            codomain_coerce: None,
            name,
        }
    }
    /// Set the domain coercion.
    pub fn with_domain(mut self, c: Coercion) -> Self {
        self.domain_coerce = Some(Box::new(c));
        self
    }
    /// Set the codomain coercion.
    pub fn with_codomain(mut self, c: Coercion) -> Self {
        self.codomain_coerce = Some(Box::new(c));
        self
    }
    /// Return true if this requires a domain coercion.
    pub fn has_domain_coerce(&self) -> bool {
        self.domain_coerce.is_some()
    }
    /// Return true if this requires a codomain coercion.
    pub fn has_codomain_coerce(&self) -> bool {
        self.codomain_coerce.is_some()
    }
}
#[allow(dead_code)]
pub struct CoercionEventLog {
    events: Vec<CoercionEvent>,
    next_seq: u64,
    max_capacity: usize,
}
#[allow(dead_code)]
impl CoercionEventLog {
    pub fn new(max_capacity: usize) -> Self {
        CoercionEventLog {
            events: Vec::with_capacity(max_capacity.min(1024)),
            next_seq: 0,
            max_capacity,
        }
    }
    pub fn log(&mut self, kind: CoercionEventKind) {
        let seq = self.next_seq;
        self.next_seq += 1;
        if self.events.len() >= self.max_capacity {
            self.events.remove(0);
        }
        self.events.push(CoercionEvent::new(kind, seq));
    }
    pub fn events(&self) -> &[CoercionEvent] {
        &self.events
    }
    pub fn count_by_kind<F>(&self, pred: F) -> usize
    where
        F: Fn(&CoercionEventKind) -> bool,
    {
        self.events.iter().filter(|e| pred(&e.kind)).count()
    }
    pub fn cache_hit_rate(&self) -> f64 {
        let hits = self.count_by_kind(|k| matches!(k, CoercionEventKind::CacheHit));
        let misses = self.count_by_kind(|k| matches!(k, CoercionEventKind::CacheMiss));
        let total = hits + misses;
        if total == 0 {
            0.0
        } else {
            hits as f64 / total as f64
        }
    }
    pub fn applied_coercions(&self) -> Vec<Name> {
        self.events
            .iter()
            .filter_map(|e| {
                if let CoercionEventKind::Applied { coerce } = &e.kind {
                    Some(coerce.clone())
                } else {
                    None
                }
            })
            .collect()
    }
    pub fn clear(&mut self) {
        self.events.clear();
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CoercionTypeClass {
    pub class_name: Name,
    pub method_name: Name,
    pub coercion: Coercion,
    pub instance_priority: i32,
}
#[allow(dead_code)]
impl CoercionTypeClass {
    pub fn new(class_name: Name, method_name: Name, coercion: Coercion) -> Self {
        CoercionTypeClass {
            class_name,
            method_name,
            coercion,
            instance_priority: 0,
        }
    }
    pub fn with_priority(mut self, priority: i32) -> Self {
        self.instance_priority = priority;
        self
    }
    pub fn class_name(&self) -> &Name {
        &self.class_name
    }
    pub fn method_name(&self) -> &Name {
        &self.method_name
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CoercionEvent {
    pub kind: CoercionEventKind,
    pub seq: u64,
}
#[allow(dead_code)]
impl CoercionEvent {
    pub fn new(kind: CoercionEventKind, seq: u64) -> Self {
        CoercionEvent { kind, seq }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum CoercionEventKind {
    Registered { coerce: Name },
    Applied { coerce: Name },
    CacheMiss,
    CacheHit,
    PathFound { length: usize },
    PathNotFound,
    ScopeEntered,
    ScopeExited,
}
/// A scope of locally-defined coercions.
#[derive(Debug, Default)]
pub struct CoercionScope {
    coercions: Vec<Coercion>,
}
impl CoercionScope {
    /// Create a new empty coercion scope.
    pub fn new() -> Self {
        Self::default()
    }
    /// Add a coercion to this scope.
    pub fn add(&mut self, coercion: Coercion) {
        self.coercions.push(coercion);
    }
    /// Return all coercions in this scope.
    pub fn all(&self) -> &[Coercion] {
        &self.coercions
    }
    /// Return the number of coercions.
    pub fn len(&self) -> usize {
        self.coercions.len()
    }
    /// Return true if the scope is empty.
    pub fn is_empty(&self) -> bool {
        self.coercions.is_empty()
    }
}
#[allow(dead_code)]
pub struct CoercionValidator {
    errors: Vec<CoercionValidationError>,
    warnings: Vec<String>,
}
#[allow(dead_code)]
impl CoercionValidator {
    pub fn new() -> Self {
        CoercionValidator {
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }
    pub fn validate_coercion(&mut self, c: &Coercion) {
        if c.priority > i32::MAX as u32 {
            self.errors.push(CoercionValidationError::InvalidPriority {
                coerce: c.coerce.clone(),
                priority: c.priority as i32,
            });
        }
    }
    pub fn validate_registry(&mut self, registry: &CoercionRegistry) {
        for c in &registry.coercions {
            self.validate_coercion(c);
        }
        let mut path_counts: HashMap<(String, String), usize> = HashMap::new();
        for c in &registry.coercions {
            let key = (format!("{:?}", c.from), format!("{:?}", c.to));
            *path_counts.entry(key).or_insert(0) += 1;
        }
        for ((from, to), count) in &path_counts {
            if *count > 1 {
                self.warnings.push(format!(
                    "ambiguous: {} coercions from {:?} to {:?}",
                    count, from, to
                ));
            }
        }
    }
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }
    pub fn errors(&self) -> &[CoercionValidationError] {
        &self.errors
    }
    pub fn warnings(&self) -> &[String] {
        &self.warnings
    }
    pub fn take_errors(self) -> Vec<CoercionValidationError> {
        self.errors
    }
}
#[allow(dead_code)]
pub struct CoercionNormalizer {
    strategy: NormalizationStrategy,
    max_steps: u32,
}
#[allow(dead_code)]
impl CoercionNormalizer {
    pub fn new(strategy: NormalizationStrategy) -> Self {
        CoercionNormalizer {
            strategy,
            max_steps: 1000,
        }
    }
    pub fn with_max_steps(mut self, max_steps: u32) -> Self {
        self.max_steps = max_steps;
        self
    }
    pub fn strategy(&self) -> NormalizationStrategy {
        self.strategy
    }
    pub fn normalize(&self, expr: Expr) -> Expr {
        match self.strategy {
            NormalizationStrategy::None => expr,
            NormalizationStrategy::BetaReduce | NormalizationStrategy::Whnf => expr,
        }
    }
    pub fn normalize_after_coerce(&self, _coerce: &Coercion, expr: Expr) -> Expr {
        self.normalize(expr)
    }
}
/// The kind of a coercion.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CoercionKind {
    /// Coercion via function application.
    FunCoercion,
    /// Coercion between sorts (e.g. Prop -> Type).
    SortCoercion,
    /// Chained coercion via multiple named steps.
    CompoundCoercion(Vec<Name>),
    /// User-defined coercion.
    UserCoercion,
}
/// Registry for tracking available coercions.
pub struct CoercionRegistry {
    /// Available coercions.
    coercions: Vec<Coercion>,
}
impl CoercionRegistry {
    /// Create a new coercion registry.
    pub fn new() -> Self {
        Self {
            coercions: Vec::new(),
        }
    }
    /// Register a coercion.
    pub fn register(&mut self, coercion: Coercion) {
        self.coercions.push(coercion);
    }
    /// Find a direct coercion from `from` to `to`.
    pub fn find_coercion(&self, from: &Expr, to: &Expr) -> Option<&Coercion> {
        self.coercions
            .iter()
            .find(|c| &c.from == from && &c.to == to)
    }
    /// Apply a direct coercion to an expression.
    pub fn apply_coercion(&self, expr: Expr, from: &Expr, to: &Expr) -> Option<Expr> {
        self.find_coercion(from, to).map(|coercion| {
            Expr::App(
                Box::new(Expr::Const(coercion.coerce.clone(), vec![])),
                Box::new(expr),
            )
        })
    }
    /// Get all registered coercions.
    pub fn all_coercions(&self) -> &[Coercion] {
        &self.coercions
    }
    /// Register a function coercion between two named types.
    #[allow(dead_code)]
    pub fn register_function_coercion(
        &mut self,
        from: Name,
        to: Name,
        fun_name: Name,
        priority: u32,
    ) {
        let from_expr = Expr::Const(from, vec![]);
        let to_expr = Expr::Const(to, vec![]);
        self.register(Coercion {
            from: from_expr,
            to: to_expr,
            coerce: fun_name,
            kind: CoercionKind::FunCoercion,
            priority,
            is_instance: false,
        });
    }
    /// Register a sort coercion (e.g. Prop -> Type).
    #[allow(dead_code)]
    pub fn register_sort_coercion(&mut self, from_sort: Level, to_sort: Level) {
        let from_expr = Expr::Sort(from_sort);
        let to_expr = Expr::Sort(to_sort.clone());
        let coerce_name = Name::str("sortLift");
        self.register(Coercion {
            from: from_expr,
            to: to_expr,
            coerce: coerce_name,
            kind: CoercionKind::SortCoercion,
            priority: 100,
            is_instance: false,
        });
    }
    /// Unregister a coercion from `from` to `to`.
    #[allow(dead_code)]
    pub fn unregister_coercion(&mut self, from: &Expr, to: &Expr) -> bool {
        let before = self.coercions.len();
        self.coercions.retain(|c| &c.from != from || &c.to != to);
        self.coercions.len() < before
    }
    /// Check whether any coercion (direct or chained) exists between two types.
    #[allow(dead_code)]
    pub fn has_coercion(&self, from: &Expr, to: &Expr) -> bool {
        self.find_coercion_chain(from, to).is_some()
    }
    /// Return the coercion graph as an adjacency list of `(from_name, to_name)`.
    #[allow(dead_code)]
    pub fn coercion_graph(&self) -> Vec<(Name, Name)> {
        let mut edges = Vec::new();
        for c in &self.coercions {
            if let (Expr::Const(f, _), Expr::Const(t, _)) = (&c.from, &c.to) {
                edges.push((f.clone(), t.clone()));
            }
        }
        edges
    }
    /// Find the shortest coercion chain from `from` to `to` via BFS.
    ///
    /// Returns `None` if no chain exists.
    #[allow(dead_code)]
    pub fn find_coercion_chain(&self, from: &Expr, to: &Expr) -> Option<CoercionPath> {
        if let Some(c) = self.find_coercion(from, to) {
            return Some(CoercionPath::single(c.clone()));
        }
        let adj = self.build_adjacency();
        let mut visited: HashSet<Expr> = HashSet::new();
        let mut queue: VecDeque<(Expr, Vec<Coercion>)> = VecDeque::new();
        visited.insert(from.clone());
        queue.push_back((from.clone(), Vec::new()));
        while let Some((current, path)) = queue.pop_front() {
            if let Some(nexts) = adj.get(&current) {
                for coercion in nexts {
                    let mut new_path = path.clone();
                    new_path.push(coercion.clone());
                    if &coercion.to == to {
                        return Some(CoercionPath::from_steps(new_path));
                    }
                    if visited.insert(coercion.to.clone()) {
                        queue.push_back((coercion.to.clone(), new_path));
                    }
                }
            }
        }
        None
    }
    /// Apply a coercion chain to an expression.
    #[allow(dead_code)]
    pub fn apply_coercion_chain(&self, mut expr: Expr, path: &CoercionPath) -> Expr {
        for step in &path.steps {
            expr = Expr::App(
                Box::new(Expr::Const(step.coerce.clone(), vec![])),
                Box::new(expr),
            );
        }
        expr
    }
    /// Register built-in coercions: Nat->Int, Bool->Prop, coe (generic).
    #[allow(dead_code)]
    pub fn register_builtins(&mut self) {
        self.register(Coercion {
            from: Expr::Const(Name::str("Nat"), vec![]),
            to: Expr::Const(Name::str("Int"), vec![]),
            coerce: Name::str("Int.ofNat"),
            kind: CoercionKind::FunCoercion,
            priority: 100,
            is_instance: false,
        });
        self.register(Coercion {
            from: Expr::Const(Name::str("Bool"), vec![]),
            to: Expr::Sort(Level::zero()),
            coerce: Name::str("Bool.toProp"),
            kind: CoercionKind::FunCoercion,
            priority: 100,
            is_instance: false,
        });
        self.register(Coercion {
            from: Expr::Const(Name::str("Int"), vec![]),
            to: Expr::Const(Name::str("Rat"), vec![]),
            coerce: Name::str("Rat.ofInt"),
            kind: CoercionKind::FunCoercion,
            priority: 100,
            is_instance: false,
        });
        self.register(Coercion {
            from: Expr::Const(Name::str("coe.src"), vec![]),
            to: Expr::Const(Name::str("coe.tgt"), vec![]),
            coerce: Name::str("coe"),
            kind: CoercionKind::UserCoercion,
            priority: 1000,
            is_instance: false,
        });
    }
    /// Build adjacency map for BFS.
    fn build_adjacency(&self) -> HashMap<Expr, Vec<Coercion>> {
        let mut adj: HashMap<Expr, Vec<Coercion>> = HashMap::new();
        for c in &self.coercions {
            adj.entry(c.from.clone()).or_default().push(c.clone());
        }
        adj
    }
}
#[allow(dead_code)]
pub struct TypeClassCoercionRegistry {
    entries: Vec<CoercionTypeClass>,
}
#[allow(dead_code)]
impl TypeClassCoercionRegistry {
    pub fn new() -> Self {
        TypeClassCoercionRegistry {
            entries: Vec::new(),
        }
    }
    pub fn register(&mut self, entry: CoercionTypeClass) {
        self.entries.push(entry);
    }
    pub fn lookup_by_class(&self, class_name: &Name) -> Vec<&CoercionTypeClass> {
        self.entries
            .iter()
            .filter(|e| &e.class_name == class_name)
            .collect()
    }
    pub fn lookup_by_method(&self, method_name: &Name) -> Option<&CoercionTypeClass> {
        self.entries.iter().find(|e| &e.method_name == method_name)
    }
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NormalizationStrategy {
    /// Apply beta reduction after each coercion
    BetaReduce,
    /// Apply weak head normal form
    Whnf,
    /// No normalization
    None,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CoercionInferenceHint {
    pub preferred_coerce: Option<Name>,
    pub forbidden_coercions: Vec<Name>,
    pub max_path_length: Option<usize>,
    pub require_instance: bool,
}
#[allow(dead_code)]
impl CoercionInferenceHint {
    pub fn new() -> Self {
        CoercionInferenceHint {
            preferred_coerce: None,
            forbidden_coercions: Vec::new(),
            max_path_length: None,
            require_instance: false,
        }
    }
    pub fn prefer(mut self, name: Name) -> Self {
        self.preferred_coerce = Some(name);
        self
    }
    pub fn forbid(mut self, name: Name) -> Self {
        self.forbidden_coercions.push(name);
        self
    }
    pub fn max_length(mut self, n: usize) -> Self {
        self.max_path_length = Some(n);
        self
    }
    pub fn require_instance(mut self) -> Self {
        self.require_instance = true;
        self
    }
    pub fn is_forbidden(&self, name: &Name) -> bool {
        self.forbidden_coercions.contains(name)
    }
}
/// A registry of coercions, supporting lookup and path search.
#[derive(Debug, Default)]
pub struct CoercionGraphRegistry {
    coercions: Vec<Coercion>,
    graph: CoercionGraph,
}
impl CoercionGraphRegistry {
    /// Create an empty registry.
    pub fn new() -> Self {
        Self::default()
    }
    /// Register a new coercion.
    pub fn register(&mut self, coercion: Coercion) {
        self.graph.add_coercion(coercion.clone());
        self.coercions.push(coercion);
    }
    /// Return all registered coercions.
    pub fn all_coercions(&self) -> &[Coercion] {
        &self.coercions
    }
    /// Return the number of registered coercions.
    pub fn len(&self) -> usize {
        self.coercions.len()
    }
    /// Return true if the registry is empty.
    pub fn is_empty(&self) -> bool {
        self.coercions.is_empty()
    }
    /// Find a direct coercion from `from` to `to`.
    pub fn find_direct(&self, from: &Expr, to: &Expr) -> Option<&Coercion> {
        let from_key = coercion_type_key(from);
        let to_key = coercion_type_key(to);
        self.coercions
            .iter()
            .filter(|c| {
                coercion_type_key(&c.from) == from_key && coercion_type_key(&c.to) == to_key
            })
            .min_by_key(|c| c.priority)
    }
    /// Find a coercion path (possibly multi-step) from `from` to `to`.
    pub fn find_path(&self, from: &Expr, to: &Expr, max_depth: usize) -> Option<CoercionPath> {
        if let Some(direct) = self.find_direct(from, to) {
            return Some(CoercionPath::single(direct.clone()));
        }
        self.graph.find_path(from, to, max_depth)
    }
    /// Return all coercions departing from the given type.
    pub fn coercions_from_type(&self, ty: &Expr) -> Vec<&Coercion> {
        let key = coercion_type_key(ty);
        self.coercions
            .iter()
            .filter(|c| coercion_type_key(&c.from) == key)
            .collect()
    }
    /// Return all user-defined coercions (excludes instance coercions).
    pub fn user_coercions(&self) -> Vec<&Coercion> {
        self.coercions
            .iter()
            .filter(|c| matches!(c.kind, CoercionKind::UserCoercion) && !c.is_instance)
            .collect()
    }
    /// Return all instance coercions.
    pub fn instance_coercions(&self) -> Vec<&Coercion> {
        self.coercions.iter().filter(|c| c.is_instance).collect()
    }
}
/// A coercion between universe sorts.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SortCoercionDirection {
    /// Coerce from Prop to Type (lift a proposition to a type).
    PropToType,
    /// Coerce from Type to Prop (forget computational content).
    TypeToProp,
}
/// A directed graph of coercions between types, used for BFS-based path finding.
#[derive(Debug, Default)]
pub struct CoercionGraph {
    /// Adjacency list: type name -> list of coercions departing that type.
    edges: HashMap<String, Vec<Coercion>>,
}
impl CoercionGraph {
    /// Create an empty coercion graph.
    pub fn new() -> Self {
        Self::default()
    }
    /// Add a coercion edge to the graph.
    pub fn add_coercion(&mut self, coercion: Coercion) {
        let key = coercion_type_key(&coercion.from);
        self.edges.entry(key).or_default().push(coercion);
    }
    /// Return all coercions departing from the given type.
    pub fn coercions_from(&self, ty: &Expr) -> &[Coercion] {
        let key = coercion_type_key(ty);
        self.edges.get(&key).map(|v| v.as_slice()).unwrap_or(&[])
    }
    /// Return the number of edges in the graph.
    pub fn edge_count(&self) -> usize {
        self.edges.values().map(|v| v.len()).sum()
    }
    /// Return the number of distinct source types.
    pub fn node_count(&self) -> usize {
        self.edges.len()
    }
    /// BFS search for a coercion path from `from` to `to`.
    pub fn find_path(&self, from: &Expr, to: &Expr, max_depth: usize) -> Option<CoercionPath> {
        if coercion_type_key(from) == coercion_type_key(to) {
            return None;
        }
        let mut queue: VecDeque<CoercionPath> = VecDeque::new();
        let start_coercions = self.coercions_from(from);
        for c in start_coercions {
            queue.push_back(CoercionPath::single(c.clone()));
        }
        let mut visited: HashSet<String> = HashSet::new();
        visited.insert(coercion_type_key(from));
        while let Some(path) = queue.pop_front() {
            let last_step = path.steps.last()?;
            let current_ty_key = coercion_type_key(&last_step.to);
            if current_ty_key == coercion_type_key(to) {
                return Some(path);
            }
            if path.steps.len() >= max_depth {
                continue;
            }
            if visited.contains(&current_ty_key) {
                continue;
            }
            visited.insert(current_ty_key.clone());
            for next_coercion in self.coercions_from(&last_step.to) {
                let mut new_steps = path.steps.clone();
                new_steps.push(next_coercion.clone());
                let new_cost = path.total_cost + next_coercion.priority;
                queue.push_back(CoercionPath {
                    steps: new_steps,
                    total_cost: new_cost,
                });
            }
        }
        None
    }
}
/// A sequence of coercions forming a path from one type to another.
#[derive(Debug, Clone)]
pub struct CoercionPath {
    /// Ordered steps in the coercion chain.
    pub steps: Vec<Coercion>,
    /// Sum of priorities across all steps.
    pub total_cost: u32,
}
impl CoercionPath {
    /// Create a single-step coercion path.
    #[allow(dead_code)]
    pub fn single(c: Coercion) -> Self {
        let cost = c.priority;
        Self {
            steps: vec![c],
            total_cost: cost,
        }
    }
    /// Create a coercion path from a list of steps.
    #[allow(dead_code)]
    pub fn from_steps(steps: Vec<Coercion>) -> Self {
        let total_cost = steps.iter().map(|s| s.priority).sum();
        Self { steps, total_cost }
    }
    /// Return the number of steps in this path.
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.steps.len()
    }
    /// Return whether the path is empty.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.steps.is_empty()
    }
}
/// Represents a sort-level coercion.
#[derive(Debug, Clone)]
pub struct SortCoercionDecl {
    /// Direction of the sort coercion.
    pub direction: SortCoercionDirection,
    /// The coercion function name.
    pub coerce_fn: Name,
}
impl SortCoercionDecl {
    /// Create a Prop → Type coercion declaration.
    pub fn prop_to_type(coerce_fn: Name) -> Self {
        Self {
            direction: SortCoercionDirection::PropToType,
            coerce_fn,
        }
    }
    /// Create a Type → Prop coercion declaration.
    pub fn type_to_prop(coerce_fn: Name) -> Self {
        Self {
            direction: SortCoercionDirection::TypeToProp,
            coerce_fn,
        }
    }
    /// Return true if this is a Prop → Type coercion.
    pub fn is_prop_to_type(&self) -> bool {
        self.direction == SortCoercionDirection::PropToType
    }
}
/// A stack of coercion scopes.
#[derive(Debug, Default)]
pub struct CoercionScopeStack {
    scopes: Vec<CoercionScope>,
}
impl CoercionScopeStack {
    /// Create a new empty scope stack.
    pub fn new() -> Self {
        Self::default()
    }
    /// Push a new scope.
    pub fn push_scope(&mut self) {
        self.scopes.push(CoercionScope::new());
    }
    /// Pop the top scope.
    pub fn pop_scope(&mut self) -> Option<CoercionScope> {
        self.scopes.pop()
    }
    /// Add a coercion to the top scope.
    pub fn add_to_top(&mut self, coercion: Coercion) {
        if let Some(top) = self.scopes.last_mut() {
            top.add(coercion);
        }
    }
    /// Return all coercions visible in the current stack (innermost first).
    pub fn visible_coercions(&self) -> Vec<&Coercion> {
        self.scopes.iter().rev().flat_map(|s| s.all()).collect()
    }
    /// Return the stack depth.
    pub fn depth(&self) -> usize {
        self.scopes.len()
    }
    /// Return true if the stack is empty.
    pub fn is_empty(&self) -> bool {
        self.scopes.is_empty()
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CoercionValidationError {
    CyclicCoercion(Vec<Name>),
    AmbiguousPath {
        from: String,
        to: String,
        count: usize,
    },
    IncompatibleTypes {
        coerce: Name,
        expected: String,
        got: String,
    },
    MissingCoercionFn(Name),
    InvalidPriority {
        coerce: Name,
        priority: i32,
    },
}
/// A coercion from one type to another.
#[derive(Debug, Clone)]
pub struct Coercion {
    /// Source type.
    pub from: Expr,
    /// Target type.
    pub to: Expr,
    /// Coercion function name.
    pub coerce: Name,
    /// The kind of this coercion.
    pub kind: CoercionKind,
    /// Priority (lower = preferred).
    pub priority: u32,
    /// Whether this coercion was derived from a type class instance.
    pub is_instance: bool,
}
/// Statistics about coercion applications during elaboration.
#[derive(Debug, Clone, Default)]
pub struct CoercionAppStats {
    /// Total coercions applied.
    pub total_applied: usize,
    /// Coercions that required multi-step paths.
    pub multi_step: usize,
    /// Coercions that used the cache.
    pub cache_hits: usize,
    /// Coercions that failed (no path found).
    pub failures: usize,
}
impl CoercionAppStats {
    /// Create empty stats.
    pub fn new() -> Self {
        Self::default()
    }
    /// Record an applied coercion.
    pub fn record_application(&mut self, steps: usize, from_cache: bool) {
        self.total_applied += 1;
        if steps > 1 {
            self.multi_step += 1;
        }
        if from_cache {
            self.cache_hits += 1;
        }
    }
    /// Record a coercion failure.
    pub fn record_failure(&mut self) {
        self.failures += 1;
    }
    /// Return the success rate.
    pub fn success_rate(&self) -> f64 {
        let total = self.total_applied + self.failures;
        if total == 0 {
            1.0
        } else {
            self.total_applied as f64 / total as f64
        }
    }
}
