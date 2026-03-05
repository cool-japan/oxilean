//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use oxilean_kernel::{Expr, Name};
use std::collections::HashMap;

/// An ordered collection of instances for a single type class.
///
/// Instances are sorted by descending priority so that the highest-priority
/// instance can be selected efficiently.
#[derive(Clone, Debug, Default)]
pub struct InstanceSet {
    /// Instances sorted by priority (highest first).
    instances: Vec<Instance>,
    /// The class this set belongs to.
    pub class_name: Name,
}
impl InstanceSet {
    /// Create an empty instance set for the given class.
    pub fn new(class_name: Name) -> Self {
        Self {
            instances: Vec::new(),
            class_name,
        }
    }
    /// Insert an instance, maintaining priority order.
    pub fn insert(&mut self, inst: Instance) {
        let pos = self
            .instances
            .iter()
            .position(|i| i.priority < inst.priority)
            .unwrap_or(self.instances.len());
        self.instances.insert(pos, inst);
    }
    /// Number of instances in the set.
    pub fn len(&self) -> usize {
        self.instances.len()
    }
    /// Whether the set is empty.
    pub fn is_empty(&self) -> bool {
        self.instances.is_empty()
    }
    /// The highest-priority instance, if any.
    pub fn best(&self) -> Option<&Instance> {
        self.instances.first()
    }
    /// Iterate over all instances (highest priority first).
    pub fn iter(&self) -> impl Iterator<Item = &Instance> {
        self.instances.iter()
    }
    /// Remove an instance by name. Returns `true` if found.
    pub fn remove_by_name(&mut self, name: &Name) -> bool {
        if let Some(pos) = self
            .instances
            .iter()
            .position(|i| i.name.as_ref() == Some(name))
        {
            self.instances.remove(pos);
            true
        } else {
            false
        }
    }
}
/// A type class method.
#[derive(Clone, Debug)]
pub struct Method {
    /// Method name
    pub name: Name,
    /// Method type (may reference class parameters)
    pub ty: Expr,
    /// Optional default implementation
    pub default_impl: Option<Expr>,
}
impl Method {
    /// Create a method with no default implementation.
    pub fn new(name: Name, ty: Expr) -> Self {
        Self {
            name,
            ty,
            default_impl: None,
        }
    }
    /// Create a method with a default implementation.
    pub fn with_default(name: Name, ty: Expr, default_impl: Expr) -> Self {
        Self {
            name,
            ty,
            default_impl: Some(default_impl),
        }
    }
    /// Return whether this method has a default implementation.
    pub fn has_default(&self) -> bool {
        self.default_impl.is_some()
    }
}
/// A cached instance lookup result.
#[derive(Clone, Debug)]
pub enum InstanceCacheEntry {
    /// A concrete instance was found.
    Found(Name),
    /// No instance was found (negative cache entry).
    NotFound,
    /// Result is ambiguous (multiple instances).
    Ambiguous(Vec<Name>),
}
/// Cache for typeclass instance resolution results.
#[derive(Clone, Debug, Default)]
pub struct InstanceCache {
    entries: std::collections::HashMap<InstanceCacheKey, InstanceCacheEntry>,
    /// Number of cache hits.
    hits: u64,
    /// Number of cache misses.
    misses: u64,
}
impl InstanceCache {
    /// Create an empty cache.
    pub fn new() -> Self {
        Self::default()
    }
    /// Insert a result into the cache.
    pub fn insert(&mut self, key: InstanceCacheKey, entry: InstanceCacheEntry) {
        self.entries.insert(key, entry);
    }
    /// Look up a key in the cache.
    pub fn lookup(&mut self, key: &InstanceCacheKey) -> Option<&InstanceCacheEntry> {
        if let Some(e) = self.entries.get(key) {
            self.hits += 1;
            Some(e)
        } else {
            self.misses += 1;
            None
        }
    }
    /// Invalidate a specific cache entry.
    pub fn invalidate(&mut self, key: &InstanceCacheKey) {
        self.entries.remove(key);
    }
    /// Clear the entire cache.
    pub fn clear(&mut self) {
        self.entries.clear();
        self.hits = 0;
        self.misses = 0;
    }
    /// Return the cache hit rate in [0, 1].
    pub fn hit_rate(&self) -> f64 {
        let total = self.hits + self.misses;
        if total == 0 {
            1.0
        } else {
            self.hits as f64 / total as f64
        }
    }
    /// Number of entries in the cache.
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    /// Return `true` if the cache is empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}
/// Tracks statistics about typeclass instance synthesis.
#[derive(Clone, Debug, Default)]
pub struct SynthStats {
    /// Total synthesis requests.
    pub requests: u64,
    /// Successful syntheses.
    pub successes: u64,
    /// Failed syntheses.
    pub failures: u64,
    /// Cache hits.
    pub cache_hits: u64,
    /// Syntheses resolved via superclass chains.
    pub superclass_resolutions: u64,
    /// Maximum search depth reached.
    pub max_search_depth: u32,
}
impl SynthStats {
    /// Create zeroed stats.
    pub fn new() -> Self {
        Self::default()
    }
    /// Record a synthesis attempt outcome.
    pub fn record(&mut self, success: bool, cache_hit: bool, depth: u32) {
        self.requests += 1;
        if success {
            self.successes += 1;
        } else {
            self.failures += 1;
        }
        if cache_hit {
            self.cache_hits += 1;
        }
        if depth > self.max_search_depth {
            self.max_search_depth = depth;
        }
    }
    /// Record a superclass chain resolution.
    pub fn record_superclass(&mut self) {
        self.superclass_resolutions += 1;
    }
    /// Success rate in [0, 1].
    pub fn success_rate(&self) -> f64 {
        if self.requests == 0 {
            1.0
        } else {
            self.successes as f64 / self.requests as f64
        }
    }
    /// Cache hit rate in [0, 1].
    pub fn cache_hit_rate(&self) -> f64 {
        if self.requests == 0 {
            1.0
        } else {
            self.cache_hits as f64 / self.requests as f64
        }
    }
    /// Merge another stats object into this one.
    pub fn merge(&mut self, other: &SynthStats) {
        self.requests += other.requests;
        self.successes += other.successes;
        self.failures += other.failures;
        self.cache_hits += other.cache_hits;
        self.superclass_resolutions += other.superclass_resolutions;
        if other.max_search_depth > self.max_search_depth {
            self.max_search_depth = other.max_search_depth;
        }
    }
    /// Summary string.
    pub fn summary(&self) -> String {
        format!(
            "requests={} ok={} fail={} cache_hits={} sc_res={} max_depth={}",
            self.requests,
            self.successes,
            self.failures,
            self.cache_hits,
            self.superclass_resolutions,
            self.max_search_depth,
        )
    }
}
/// Error kinds detected during coherence checking.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CoherenceError {
    /// Two instances overlap (match the same type).
    OverlappingInstances(Name, Name),
    /// An instance does not satisfy the orphan rule.
    OrphanInstance(Name),
    /// A duplicate instance (same class and same type).
    DuplicateInstance(Name),
}
/// Outcome of a single instance resolution attempt.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum InstanceResolutionOutcome {
    /// Resolution succeeded with the given expression.
    Success(Expr),
    /// No matching instance was found.
    NoInstance,
    /// Resolution failed due to a constraint conflict.
    Conflict(String),
    /// Resolution hit the budget limit.
    BudgetExceeded,
}
#[allow(dead_code)]
impl InstanceResolutionOutcome {
    /// Return true if resolution succeeded.
    pub fn is_success(&self) -> bool {
        matches!(self, InstanceResolutionOutcome::Success(_))
    }
    /// Return the resolved expression, if any.
    pub fn success_expr(&self) -> Option<&Expr> {
        match self {
            InstanceResolutionOutcome::Success(e) => Some(e),
            _ => None,
        }
    }
    /// Return true if no instance was found.
    pub fn is_no_instance(&self) -> bool {
        matches!(self, InstanceResolutionOutcome::NoInstance)
    }
}
/// How to build the derived implementation expression.
#[derive(Debug, Clone)]
pub enum DeriveBuilder {
    /// Use a named combinator constant (e.g. `deriveEqFromOrd`).
    Combinator(Name),
    /// Use a sorry placeholder (for tests).
    Sorry,
}
/// A typeclass constraint that needs to be resolved.
#[derive(Debug, Clone)]
pub struct ClassConstraint {
    /// The class name.
    pub class: Name,
    /// The type argument.
    pub ty: Expr,
    /// Source location hint (for diagnostics).
    pub source: Option<String>,
}
impl ClassConstraint {
    /// Create a new class constraint.
    pub fn new(class: Name, ty: Expr) -> Self {
        Self {
            class,
            ty,
            source: None,
        }
    }
    /// Attach a source hint.
    pub fn with_source(mut self, src: impl Into<String>) -> Self {
        self.source = Some(src.into());
        self
    }
}
/// Checks for coherence violations in type class instance sets.
///
/// Coherence requires that for any type `T` and class `C`, there is at most
/// one canonical instance. Overlapping instances at the same priority violate
/// coherence.
#[derive(Clone, Debug, Default)]
pub struct CoherenceChecker {
    /// Violations found during checking.
    pub violations: Vec<CoherenceViolation>,
}
impl CoherenceChecker {
    /// Create a new checker.
    pub fn new() -> Self {
        Self::default()
    }
    /// Check a set of instances for coherence violations.
    ///
    /// Two instances violate coherence if they have the same priority.
    pub fn check(&mut self, class: &Name, set: &InstanceSet) {
        let insts: Vec<&Instance> = set.iter().collect();
        for i in 0..insts.len() {
            for j in (i + 1)..insts.len() {
                if insts[i].priority == insts[j].priority {
                    self.violations.push(CoherenceViolation::new(
                        class.clone(),
                        insts[i].name.clone(),
                        insts[j].name.clone(),
                        format!(
                            "instances {:?} and {:?} have the same priority {}",
                            insts[i].name, insts[j].name, insts[i].priority
                        ),
                    ));
                }
            }
        }
    }
    /// Whether any violations were found.
    pub fn has_violations(&self) -> bool {
        !self.violations.is_empty()
    }
    /// Clear all violations.
    pub fn clear(&mut self) {
        self.violations.clear();
    }
}
/// A cache key for instance lookup results.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct InstanceCacheKey {
    /// The typeclass name being searched.
    pub class: Name,
    /// A string representation of the type argument (for hashing).
    pub type_repr: String,
}
impl InstanceCacheKey {
    /// Create a new cache key.
    pub fn new(class: Name, type_repr: impl Into<String>) -> Self {
        Self {
            class,
            type_repr: type_repr.into(),
        }
    }
}
/// A stack of locally-scoped instance contexts.
///
/// Used to track instances introduced by `local instance` declarations.
#[derive(Clone, Debug, Default)]
pub struct InstanceContextStack {
    frames: Vec<Vec<Instance>>,
}
impl InstanceContextStack {
    /// Create an empty stack.
    pub fn new() -> Self {
        Self::default()
    }
    /// Push a new scope frame.
    pub fn push_frame(&mut self) {
        self.frames.push(Vec::new());
    }
    /// Pop the current scope frame, returning its instances.
    pub fn pop_frame(&mut self) -> Vec<Instance> {
        self.frames.pop().unwrap_or_default()
    }
    /// Add an instance to the current frame.
    pub fn add_local(&mut self, inst: Instance) {
        if let Some(frame) = self.frames.last_mut() {
            frame.push(inst);
        }
    }
    /// Collect all locally-visible instances (all frames).
    pub fn all_locals(&self) -> Vec<&Instance> {
        self.frames.iter().flat_map(|f| f.iter()).collect()
    }
    /// Depth of the stack.
    pub fn depth(&self) -> usize {
        self.frames.len()
    }
}
/// Type class registry.
///
/// Holds all registered classes and instances, and performs constraint
/// resolution on demand.
pub struct TypeClassRegistry {
    /// Registered classes, keyed by name.
    classes: HashMap<Name, TypeClass>,
    /// Registered instances.
    pub(super) instances: Vec<Instance>,
    /// Pending constraints to resolve.
    pending: Vec<ClassConstraint>,
    /// Cache: (class, ty_repr) → instance index.
    cache: HashMap<(Name, String), usize>,
}
impl TypeClassRegistry {
    /// Create a new type class registry.
    pub fn new() -> Self {
        Self {
            classes: HashMap::new(),
            instances: Vec::new(),
            pending: Vec::new(),
            cache: HashMap::new(),
        }
    }
    /// Register a type class.
    pub fn register_class(&mut self, class: TypeClass) {
        self.classes.insert(class.name.clone(), class);
    }
    /// Register an instance.
    pub fn register_instance(&mut self, instance: Instance) {
        self.instances.push(instance);
        self.cache.clear();
    }
    /// Find an instance for a given class and type (exact structural match).
    pub fn find_instance(&self, class: &Name, ty: &Expr) -> Option<&Instance> {
        self.instances
            .iter()
            .find(|inst| &inst.class == class && &inst.ty == ty)
    }
    /// Find the best (lowest priority) instance for a class and type.
    ///
    /// Returns `None` if no candidates exist.
    pub fn find_best_instance(&self, class: &Name, ty: &Expr) -> Option<&Instance> {
        let mut best: Option<&Instance> = None;
        for inst in &self.instances {
            if &inst.class == class && type_matches(&inst.ty, ty) {
                match best {
                    None => best = Some(inst),
                    Some(b) if inst.priority < b.priority => best = Some(inst),
                    _ => {}
                }
            }
        }
        best
    }
    /// Get all instances for a class.
    pub fn get_instances(&self, class: &Name) -> Vec<&Instance> {
        self.instances
            .iter()
            .filter(|inst| &inst.class == class)
            .collect()
    }
    /// Get a class by name.
    pub fn get_class(&self, name: &Name) -> Option<&TypeClass> {
        self.classes.get(name)
    }
    /// Add a pending constraint.
    pub fn add_constraint(&mut self, c: ClassConstraint) {
        self.pending.push(c);
    }
    /// Attempt to resolve all pending constraints.
    ///
    /// Returns the list of constraints that could not be resolved.
    pub fn resolve_pending(&mut self) -> Vec<ClassConstraint> {
        let pending = std::mem::take(&mut self.pending);
        let mut unresolved = Vec::new();
        for c in pending {
            if self.find_best_instance(&c.class, &c.ty).is_none() {
                unresolved.push(c);
            }
        }
        unresolved
    }
    /// Return the number of registered classes.
    pub fn class_count(&self) -> usize {
        self.classes.len()
    }
    /// Return the number of registered instances.
    pub fn instance_count(&self) -> usize {
        self.instances.len()
    }
    /// Check that all instances satisfy the required class methods.
    ///
    /// Returns a list of errors for each incomplete instance.
    pub fn check_completeness(&self) -> Vec<ClassError> {
        let mut errors = Vec::new();
        for inst in &self.instances {
            if let Some(class) = self.classes.get(&inst.class) {
                for method in &class.methods {
                    if !method.has_default() && inst.get_method_impl(&method.name).is_none() {
                        errors.push(ClassError::MissingMethod {
                            instance: inst
                                .name
                                .clone()
                                .unwrap_or_else(|| Name::str("<anonymous>")),
                            method: method.name.clone(),
                        });
                    }
                }
            }
        }
        errors
    }
    /// Perform a simple coherence check: warn if two instances for the same
    /// class have the same type (exact structural equality).
    pub fn check_coherence(&self) -> Vec<ClassError> {
        let mut errors = Vec::new();
        let n = self.instances.len();
        for i in 0..n {
            for j in (i + 1)..n {
                let a = &self.instances[i];
                let b = &self.instances[j];
                if a.class == b.class && a.ty == b.ty {
                    errors.push(ClassError::Incoherent {
                        class: a.class.clone(),
                        inst_a: a.name.clone().unwrap_or_else(|| Name::str("<anon-a>")),
                        inst_b: b.name.clone().unwrap_or_else(|| Name::str("<anon-b>")),
                    });
                }
            }
        }
        errors
    }
    /// Look up instances for all superclasses of a given class.
    ///
    /// Returns a map from superclass name to the found instance.
    pub fn resolve_superclasses(
        &self,
        class_name: &Name,
        ty: &Expr,
    ) -> HashMap<Name, Option<&Instance>> {
        let mut result = HashMap::new();
        if let Some(class) = self.classes.get(class_name) {
            for super_name in &class.superclasses {
                let inst = self.find_best_instance(super_name, ty);
                result.insert(super_name.clone(), inst);
            }
        }
        result
    }
}
/// A coherence violation: two instances at the same priority for the same type.
#[derive(Clone, Debug)]
pub struct CoherenceViolation {
    /// The class involved.
    pub class: Name,
    /// The two conflicting instance names.
    pub inst1: Option<Name>,
    /// The second conflicting instance name.
    pub inst2: Option<Name>,
    /// Explanation message.
    pub message: String,
}
impl CoherenceViolation {
    /// Create a new violation.
    pub fn new(
        class: Name,
        inst1: Option<Name>,
        inst2: Option<Name>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            class,
            inst1,
            inst2,
            message: message.into(),
        }
    }
}
/// Describes an inheritance edge in the typeclass hierarchy.
#[derive(Clone, Debug)]
pub struct SuperclassEdge {
    /// The child class.
    pub child: Name,
    /// The parent class (superclass).
    pub parent: Name,
    /// The superclass projection name (e.g. `Ord.toEq`).
    pub projection: Name,
    /// Position of this superclass in the parent class parameters.
    pub param_pos: usize,
}
impl SuperclassEdge {
    /// Create a new superclass edge.
    pub fn new(child: Name, parent: Name, projection: Name, param_pos: usize) -> Self {
        Self {
            child,
            parent,
            projection,
            param_pos,
        }
    }
}
/// Budget limiting instance resolution search.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct InstanceSearchBudget {
    /// Maximum number of instances to enumerate.
    pub max_candidates: usize,
    /// Maximum chain length for instance chains.
    pub max_chain_length: usize,
    /// Maximum number of typeclass goals to discharge.
    pub max_goals: usize,
}
#[allow(dead_code)]
impl InstanceSearchBudget {
    /// Create a budget with default limits.
    pub fn new() -> Self {
        Self::default()
    }
    /// Check if the number of candidates is within budget.
    pub fn allows_candidates(&self, n: usize) -> bool {
        n <= self.max_candidates
    }
    /// Check if the chain length is within budget.
    pub fn allows_chain_length(&self, n: usize) -> bool {
        n <= self.max_chain_length
    }
}
/// The full typeclass hierarchy graph.
///
/// Stores all superclass edges and provides queries for superclass inclusion
/// and topological ordering of classes.
#[derive(Clone, Debug, Default)]
pub struct TypeClassHierarchy {
    /// All superclass edges.
    edges: Vec<SuperclassEdge>,
}
impl TypeClassHierarchy {
    /// Create an empty hierarchy.
    pub fn new() -> Self {
        Self::default()
    }
    /// Register a superclass relationship.
    pub fn add_superclass(&mut self, edge: SuperclassEdge) {
        self.edges.push(edge);
    }
    /// Return all direct superclasses of `class`.
    pub fn direct_superclasses(&self, class: &Name) -> Vec<&SuperclassEdge> {
        self.edges.iter().filter(|e| &e.child == class).collect()
    }
    /// Return all direct subclasses of `class`.
    pub fn direct_subclasses(&self, class: &Name) -> Vec<&Name> {
        self.edges
            .iter()
            .filter(|e| &e.parent == class)
            .map(|e| &e.child)
            .collect()
    }
    /// Check if `ancestor` is a (direct or transitive) superclass of `class`.
    pub fn is_superclass(&self, class: &Name, ancestor: &Name) -> bool {
        if class == ancestor {
            return true;
        }
        let mut visited = std::collections::HashSet::new();
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(class.clone());
        while let Some(cur) = queue.pop_front() {
            if visited.contains(&cur) {
                continue;
            }
            visited.insert(cur.clone());
            for edge in self.direct_superclasses(&cur) {
                if &edge.parent == ancestor {
                    return true;
                }
                queue.push_back(edge.parent.clone());
            }
        }
        false
    }
    /// Return all transitive superclasses of `class` (excluding itself).
    pub fn all_superclasses(&self, class: &Name) -> Vec<Name> {
        let mut visited = std::collections::HashSet::new();
        let mut queue = std::collections::VecDeque::new();
        let mut result = Vec::new();
        for edge in self.direct_superclasses(class) {
            queue.push_back(edge.parent.clone());
        }
        while let Some(cur) = queue.pop_front() {
            if visited.contains(&cur) {
                continue;
            }
            visited.insert(cur.clone());
            result.push(cur.clone());
            for edge in self.direct_superclasses(&cur) {
                queue.push_back(edge.parent.clone());
            }
        }
        result
    }
    /// Number of edges in the hierarchy.
    pub fn num_edges(&self) -> usize {
        self.edges.len()
    }
}
/// An edge in the typeclass dependency graph (used for analysis).
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct TypeclassDep {
    /// The class that depends on another.
    pub dependent: Name,
    /// The class being depended on.
    pub dependency: Name,
    /// Whether the dependency is a superclass (vs. a method constraint).
    pub is_superclass: bool,
}
/// A type class definition.
#[derive(Clone, Debug)]
pub struct TypeClass {
    /// Class name
    pub name: Name,
    /// Class parameters (type parameters, e.g. `α` in `Eq α`)
    pub params: Vec<Name>,
    /// Methods defined by this class
    pub methods: Vec<Method>,
    /// Parent classes (superclasses)
    pub superclasses: Vec<Name>,
    /// Whether the class is a `Prop`-valued relation
    pub is_prop: bool,
}
impl TypeClass {
    /// Create a new type class with no methods or superclasses.
    pub fn new(name: Name, params: Vec<Name>) -> Self {
        Self {
            name,
            params,
            methods: Vec::new(),
            superclasses: Vec::new(),
            is_prop: false,
        }
    }
    /// Add a method to this class.
    pub fn add_method(&mut self, method: Method) {
        self.methods.push(method);
    }
    /// Add a superclass constraint.
    pub fn add_superclass(&mut self, class: Name) {
        self.superclasses.push(class);
    }
    /// Return the method with the given name, if any.
    pub fn get_method(&self, name: &Name) -> Option<&Method> {
        self.methods.iter().find(|m| &m.name == name)
    }
    /// Return whether this class has any methods with default implementations.
    pub fn has_defaults(&self) -> bool {
        self.methods.iter().any(|m| m.has_default())
    }
    /// Return the number of methods (including those with defaults).
    pub fn method_count(&self) -> usize {
        self.methods.len()
    }
    /// Return the arity (number of parameters).
    pub fn arity(&self) -> usize {
        self.params.len()
    }
}
/// A graph of typeclass dependencies.
#[allow(dead_code)]
#[derive(Clone, Debug, Default)]
pub struct TypeclassDepGraph {
    edges: Vec<TypeclassDep>,
}
#[allow(dead_code)]
impl TypeclassDepGraph {
    /// Create an empty dependency graph.
    pub fn new() -> Self {
        Self::default()
    }
    /// Add a dependency edge.
    pub fn add_dep(&mut self, dep: TypeclassDep) {
        self.edges.push(dep);
    }
    /// Return all dependencies of a class.
    pub fn deps_of(&self, class: &Name) -> Vec<&TypeclassDep> {
        self.edges
            .iter()
            .filter(|e| &e.dependent == class)
            .collect()
    }
    /// Return all classes that depend on a given class.
    pub fn dependents_of(&self, class: &Name) -> Vec<&TypeclassDep> {
        self.edges
            .iter()
            .filter(|e| &e.dependency == class)
            .collect()
    }
    /// Return the number of edges.
    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }
    /// Return true if the graph is empty.
    pub fn is_empty(&self) -> bool {
        self.edges.is_empty()
    }
    /// Return only superclass edges.
    pub fn superclass_edges(&self) -> Vec<&TypeclassDep> {
        self.edges.iter().filter(|e| e.is_superclass).collect()
    }
}
/// A type class instance.
#[derive(Clone, Debug)]
pub struct Instance {
    /// Class being instantiated
    pub class: Name,
    /// Instance type (the type being given the class structure)
    pub ty: Expr,
    /// Implementation expressions, one per class method
    pub implementation: Expr,
    /// Optional instance name
    pub name: Option<Name>,
    /// Instance priority (lower = higher priority)
    pub priority: u32,
    /// Method implementations (name → expr)
    pub method_impls: HashMap<Name, Expr>,
}
impl Instance {
    /// Create a new instance.
    pub fn new(class: Name, ty: Expr, implementation: Expr) -> Self {
        Self {
            class,
            ty,
            implementation,
            name: None,
            priority: 100,
            method_impls: HashMap::new(),
        }
    }
    /// Assign a name to this instance.
    pub fn with_name(mut self, name: Name) -> Self {
        self.name = Some(name);
        self
    }
    /// Set the priority.
    pub fn with_priority(mut self, priority: u32) -> Self {
        self.priority = priority;
        self
    }
    /// Add a method implementation.
    pub fn add_method_impl(&mut self, method: Name, expr: Expr) {
        self.method_impls.insert(method, expr);
    }
    /// Look up an implementation for a given method name.
    pub fn get_method_impl(&self, method: &Name) -> Option<&Expr> {
        self.method_impls.get(method)
    }
    /// Return whether this instance supplies all methods of a given class.
    pub fn is_complete_for(&self, class: &TypeClass) -> bool {
        class
            .methods
            .iter()
            .all(|m| self.method_impls.contains_key(&m.name) || m.has_default())
    }
}
/// A registry of default method implementations.
///
/// When an instance does not provide a method, the elaborator looks up the
/// default from this registry.
#[derive(Clone, Debug, Default)]
pub struct DefaultMethodRegistry {
    /// Map from (class_name, method_name) to default expression.
    defaults: HashMap<(Name, Name), Expr>,
}
impl DefaultMethodRegistry {
    /// Create an empty registry.
    pub fn new() -> Self {
        Self::default()
    }
    /// Register a default method implementation.
    pub fn register(&mut self, class: Name, method: Name, default_impl: Expr) {
        self.defaults.insert((class, method), default_impl);
    }
    /// Look up a default implementation.
    pub fn get(&self, class: &Name, method: &Name) -> Option<&Expr> {
        self.defaults.get(&(class.clone(), method.clone()))
    }
    /// Number of registered defaults.
    pub fn len(&self) -> usize {
        self.defaults.len()
    }
    /// Whether the registry is empty.
    pub fn is_empty(&self) -> bool {
        self.defaults.is_empty()
    }
    /// All registered (class, method) pairs.
    pub fn all_keys(&self) -> Vec<(&Name, &Name)> {
        self.defaults.keys().map(|(c, m)| (c, m)).collect()
    }
}
/// Error arising from typeclass resolution.
#[derive(Debug, Clone)]
pub enum ClassError {
    /// No instance exists.
    NoInstance {
        /// The class for which no instance was found.
        class: Name,
        /// The type that was searched for an instance.
        ty: Expr,
    },
    /// Multiple equally-ranked instances.
    Ambiguous {
        /// The class with multiple instances.
        class: Name,
        /// Names of the candidate instances.
        candidates: Vec<Name>,
    },
    /// A required method was not provided.
    MissingMethod {
        /// The instance that is missing the method.
        instance: Name,
        /// The missing method name.
        method: Name,
    },
    /// Incoherence: two instances conflict.
    Incoherent {
        /// The class with conflicting instances.
        class: Name,
        /// First conflicting instance.
        inst_a: Name,
        /// Second conflicting instance.
        inst_b: Name,
    },
    /// A superclass constraint failed.
    SuperclassFailed {
        /// The class whose superclass constraint failed.
        class: Name,
        /// The failed superclass.
        superclass: Name,
    },
}
/// A rule for automatically deriving instances.
///
/// If the premise instances all exist, the conclusion instance is synthesised.
#[derive(Debug, Clone)]
pub struct DeriveRule {
    /// Name of this rule.
    pub name: Name,
    /// Class being derived.
    pub conclusion_class: Name,
    /// Premise classes (must all have instances for the same type).
    pub premise_classes: Vec<Name>,
    /// Derived instance implementation builder.
    pub builder: DeriveBuilder,
}
impl DeriveRule {
    /// Create a derive rule that uses a combinator.
    pub fn combinator(
        name: Name,
        conclusion_class: Name,
        premise_classes: Vec<Name>,
        combinator: Name,
    ) -> Self {
        Self {
            name,
            conclusion_class,
            premise_classes,
            builder: DeriveBuilder::Combinator(combinator),
        }
    }
    /// Create a sorry-based derive rule (for testing).
    pub fn sorry(name: Name, conclusion_class: Name, premise_classes: Vec<Name>) -> Self {
        Self {
            name,
            conclusion_class,
            premise_classes,
            builder: DeriveBuilder::Sorry,
        }
    }
}
/// A structured query for type class instance resolution.
#[derive(Clone, Debug)]
pub struct TypeClassQuery {
    /// The class being searched.
    pub class: Name,
    /// The type argument.
    pub ty: Expr,
    /// Maximum search depth.
    pub max_depth: u32,
    /// Whether to allow synthetic (sorry-based) instances.
    pub allow_synthetic: bool,
}
impl TypeClassQuery {
    /// Create a simple query.
    pub fn new(class: Name, ty: Expr) -> Self {
        Self {
            class,
            ty,
            max_depth: 8,
            allow_synthetic: false,
        }
    }
    /// Set the maximum search depth.
    pub fn with_max_depth(mut self, depth: u32) -> Self {
        self.max_depth = depth;
        self
    }
    /// Allow synthetic instances.
    pub fn allow_synthetic(mut self) -> Self {
        self.allow_synthetic = true;
        self
    }
}
/// The result of filling in a default method for an instance.
#[derive(Clone, Debug)]
pub struct DefaultMethodFill {
    /// Name of the method being filled.
    pub method_name: Name,
    /// The expression used to fill the method.
    pub impl_expr: Expr,
    /// Whether the fill is from an explicit default or synthesised.
    pub is_synthesised: bool,
}
impl DefaultMethodFill {
    /// Create a fill from an explicit default.
    pub fn from_default(method_name: Name, impl_expr: Expr) -> Self {
        Self {
            method_name,
            impl_expr,
            is_synthesised: false,
        }
    }
    /// Create a synthesised fill.
    pub fn synthesised(method_name: Name, impl_expr: Expr) -> Self {
        Self {
            method_name,
            impl_expr,
            is_synthesised: true,
        }
    }
}
/// The result of searching for a type class instance.
#[derive(Clone, Debug)]
#[allow(clippy::large_enum_variant)]
pub enum InstanceSearchResult {
    /// A unique best instance was found.
    Found(Instance),
    /// No instance was found.
    NotFound,
    /// Multiple instances with the same priority were found.
    Ambiguous(Vec<Instance>),
}
impl InstanceSearchResult {
    /// Whether a unique instance was found.
    pub fn is_found(&self) -> bool {
        matches!(self, InstanceSearchResult::Found(_))
    }
    /// Whether no instance was found.
    pub fn is_not_found(&self) -> bool {
        matches!(self, InstanceSearchResult::NotFound)
    }
    /// Whether the result is ambiguous.
    pub fn is_ambiguous(&self) -> bool {
        matches!(self, InstanceSearchResult::Ambiguous(_))
    }
    /// Convert to `Option<Instance>`.
    pub fn into_option(self) -> Option<Instance> {
        match self {
            InstanceSearchResult::Found(i) => Some(i),
            _ => None,
        }
    }
}
