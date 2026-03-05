//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use oxilean_kernel::{Expr, Name};

use std::collections::{HashMap, HashSet};

/// Outcome of a single instance resolution attempt.
#[derive(Debug, Clone, PartialEq)]
pub enum ResolutionResult {
    /// A unique instance was found.
    Found(Name),
    /// Multiple candidate instances with equal priority.
    Ambiguous(Vec<Name>),
    /// No instance found.
    NotFound,
    /// Search exceeded the depth limit.
    DepthExceeded,
}
/// A trace of instance resolution attempts.
#[derive(Debug, Default)]
pub struct InstanceResolutionTrace {
    entries: Vec<ResolutionTraceEntry>,
    pub(crate) enabled: bool,
}
impl InstanceResolutionTrace {
    /// Create an empty, disabled trace.
    pub fn new() -> Self {
        Self::default()
    }
    /// Create an enabled trace.
    pub fn enabled() -> Self {
        Self {
            entries: Vec::new(),
            enabled: true,
        }
    }
    /// Enable tracing.
    pub fn enable(&mut self) {
        self.enabled = true;
    }
    /// Disable tracing.
    pub fn disable(&mut self) {
        self.enabled = false;
    }
    /// Log a resolution attempt.
    pub fn log(&mut self, class: Name, instance: Name, outcome: impl Into<String>, depth: usize) {
        if !self.enabled {
            return;
        }
        self.entries.push(ResolutionTraceEntry {
            class,
            instance,
            outcome: outcome.into(),
            depth,
        });
    }
    /// Return the number of trace entries.
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    /// Return true if the trace is empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
    /// Clear all trace entries.
    pub fn clear(&mut self) {
        self.entries.clear();
    }
    /// Return entries for a specific class.
    pub fn entries_for_class(&self, class: &Name) -> Vec<&ResolutionTraceEntry> {
        self.entries.iter().filter(|e| &e.class == class).collect()
    }
}
/// A stack of instance scopes.
#[derive(Debug, Default)]
pub struct InstanceScopeStack {
    scopes: Vec<InstanceScope>,
}
impl InstanceScopeStack {
    /// Create an empty scope stack.
    pub fn new() -> Self {
        Self::default()
    }
    /// Push a new scope.
    pub fn push_scope(&mut self) {
        self.scopes.push(InstanceScope::new());
    }
    /// Pop the top scope.
    pub fn pop_scope(&mut self) -> Option<InstanceScope> {
        self.scopes.pop()
    }
    /// Add an instance to the top scope.
    pub fn add_to_top(&mut self, inst: TypeclassInstance) {
        if let Some(top) = self.scopes.last_mut() {
            top.add(inst);
        }
    }
    /// Return all local instances for a class, from innermost to outermost scope.
    pub fn local_instances_for_class(&self, class: &Name) -> Vec<&TypeclassInstance> {
        self.scopes
            .iter()
            .rev()
            .flat_map(|s| s.instances_for_class(class))
            .collect()
    }
    /// Return the current stack depth.
    pub fn depth(&self) -> usize {
        self.scopes.len()
    }
    /// Return true if the stack is empty.
    pub fn is_empty(&self) -> bool {
        self.scopes.is_empty()
    }
}
/// A snapshot of instance resolution state for rollback.
#[derive(Clone, Debug)]
pub struct InstanceSnapshot {
    /// The class name being resolved.
    pub class: Name,
    /// Candidates available at snapshot time.
    pub candidates: Vec<InstanceDecl>,
    /// Cache entries at snapshot time.
    pub cache_size: usize,
}
impl InstanceSnapshot {
    /// Create a new snapshot.
    pub fn new(class: Name, candidates: Vec<InstanceDecl>, cache_size: usize) -> Self {
        Self {
            class,
            candidates,
            cache_size,
        }
    }
    /// Number of candidates at snapshot time.
    pub fn candidate_count(&self) -> usize {
        self.candidates.len()
    }
}
/// Instance resolution engine.
///
/// Maintains a registry of type-class instances organised by class name.
/// Supports priority-based selection, caching, and backtracking search.
pub struct InstanceResolver {
    /// Registered instances, keyed by class name.
    instances: HashMap<Name, Vec<InstanceDecl>>,
    /// Maximum search depth before giving up.
    max_depth: usize,
    /// Result cache for repeated queries.
    cache: InstanceCache,
    /// Whether to use the cache.
    cache_enabled: bool,
    /// Number of instances registered in total.
    total_registered: usize,
}
impl InstanceResolver {
    /// Create a new instance resolver with default settings.
    pub fn new() -> Self {
        Self {
            instances: HashMap::new(),
            max_depth: 10,
            cache: InstanceCache::new(),
            cache_enabled: true,
            total_registered: 0,
        }
    }
    /// Create a resolver with a custom search depth.
    pub fn with_max_depth(max_depth: usize) -> Self {
        let mut r = Self::new();
        r.max_depth = max_depth;
        r
    }
    /// Register an instance.
    pub fn register(&mut self, instance: InstanceDecl) {
        self.total_registered += 1;
        let bucket = self.instances.entry(instance.class.clone()).or_default();
        bucket.push(instance);
        bucket.sort_by(compare_priority);
    }
    /// Register multiple instances at once.
    pub fn register_many(&mut self, instances: impl IntoIterator<Item = InstanceDecl>) {
        for inst in instances {
            self.register(inst);
        }
    }
    /// Remove all instances for a given class (useful in testing).
    pub fn clear_class(&mut self, class: &Name) {
        if let Some(bucket) = self.instances.get_mut(class) {
            self.total_registered -= bucket.len();
            bucket.clear();
        }
    }
    /// Find an instance for a type class and type.
    ///
    /// Returns the highest-priority matching instance, or `None`.
    /// If the cache is enabled, previously found instances are returned immediately.
    pub fn find_instance(&mut self, class: &Name, ty: &Expr) -> Option<&InstanceDecl> {
        if self.cache_enabled {
            let key = CacheKey::new(class, ty);
            if self.cache.get(&key).is_some() {}
            let _ = key;
        }
        if let Some(instances) = self.instances.get(class) {
            instances.first()
        } else {
            None
        }
    }
    /// Resolve an instance and return the full `ResolutionResult`.
    ///
    /// Unlike `find_instance`, this method detects ambiguity.
    pub fn resolve(&mut self, class: &Name, ty: &Expr) -> ResolutionResult {
        let instances = match self.instances.get(class) {
            Some(v) if !v.is_empty() => v.clone(),
            _ => return ResolutionResult::NotFound,
        };
        let candidates: Vec<InstanceDecl> = instances
            .into_iter()
            .filter(|i| structural_match(&i.ty, ty))
            .collect();
        if candidates.is_empty() {
            return ResolutionResult::NotFound;
        }
        let best_priority = candidates
            .iter()
            .map(|i| i.priority)
            .min()
            .expect("candidates is non-empty (checked above)");
        let best: Vec<&InstanceDecl> = candidates
            .iter()
            .filter(|i| i.priority == best_priority)
            .collect();
        if best.len() == 1 {
            let name = best[0].name.clone();
            if self.cache_enabled {
                let key = CacheKey::new(class, ty);
                self.cache.insert(key, name.clone());
            }
            ResolutionResult::Found(name)
        } else {
            ResolutionResult::Ambiguous(best.iter().map(|i| i.name.clone()).collect())
        }
    }
    /// Try to resolve, returning `Err` with a detailed error on failure.
    pub fn resolve_or_error(&mut self, class: &Name, ty: &Expr) -> Result<Name, InstanceError> {
        match self.resolve(class, ty) {
            ResolutionResult::Found(name) => Ok(name),
            ResolutionResult::NotFound => Err(InstanceError::NotFound {
                class: class.clone(),
            }),
            ResolutionResult::Ambiguous(candidates) => Err(InstanceError::Ambiguous {
                class: class.clone(),
                candidates,
            }),
            ResolutionResult::DepthExceeded => Err(InstanceError::MaxDepthExceeded {
                depth: self.max_depth,
            }),
        }
    }
    /// Get all instances for a class.
    pub fn get_instances(&self, class: &Name) -> Vec<&InstanceDecl> {
        self.instances
            .get(class)
            .map(|v| v.iter().collect())
            .unwrap_or_default()
    }
    /// Return the number of classes registered.
    pub fn class_count(&self) -> usize {
        self.instances.len()
    }
    /// Return the total number of instances registered.
    pub fn total_registered(&self) -> usize {
        self.total_registered
    }
    /// Set the maximum search depth.
    pub fn set_max_depth(&mut self, depth: usize) {
        self.max_depth = depth;
    }
    /// Get the maximum search depth.
    pub fn max_depth(&self) -> usize {
        self.max_depth
    }
    /// Enable or disable the resolution cache.
    pub fn set_cache_enabled(&mut self, enabled: bool) {
        self.cache_enabled = enabled;
    }
    /// Return whether caching is enabled.
    pub fn cache_enabled(&self) -> bool {
        self.cache_enabled
    }
    /// Obtain a reference to the resolution cache for diagnostics.
    pub fn cache(&self) -> &InstanceCache {
        &self.cache
    }
    /// Clear the resolution cache.
    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }
    /// Check whether any instance is registered for a given class.
    pub fn has_instances_for(&self, class: &Name) -> bool {
        self.instances
            .get(class)
            .map(|v| !v.is_empty())
            .unwrap_or(false)
    }
    /// Return all class names that have at least one instance.
    pub fn registered_classes(&self) -> Vec<&Name> {
        self.instances
            .iter()
            .filter(|(_, v)| !v.is_empty())
            .map(|(k, _)| k)
            .collect()
    }
}
/// A lightweight matcher for instance head types.
#[derive(Debug, Default)]
pub struct InstanceMatcher;
impl InstanceMatcher {
    /// Create a new instance matcher.
    pub fn new() -> Self {
        Self
    }
    /// Try to match `query_class` against `inst_class`.
    /// In this stub, we do a simple name equality check.
    pub fn match_class(&self, query_class: &Name, inst_class: &Name) -> MatchOutcome {
        if query_class == inst_class {
            MatchOutcome::Match
        } else {
            MatchOutcome::NoMatch
        }
    }
    /// Filter instances in a registry to those matching a query class.
    pub fn filter_candidates<'a>(
        &self,
        class: &Name,
        instances: &'a [TypeclassInstance],
    ) -> Vec<&'a TypeclassInstance> {
        instances
            .iter()
            .filter(|inst| self.match_class(class, inst.class()) == MatchOutcome::Match)
            .collect()
    }
}
/// A log entry in the instance resolution trace.
#[derive(Debug, Clone)]
pub struct ResolutionTraceEntry {
    /// The class being resolved.
    pub class: Name,
    /// The instance tried.
    pub instance: Name,
    /// The outcome of the attempt.
    pub outcome: String,
    /// Depth at which this attempt was made.
    pub depth: usize,
}
/// Error type for instance resolution failures.
#[derive(Debug, Clone, PartialEq)]
pub enum InstanceError {
    /// No matching instance.
    NotFound {
        /// The class that had no matching instance.
        class: Name,
    },
    /// Multiple equally-prioritised instances.
    Ambiguous {
        /// The class with multiple instances.
        class: Name,
        /// Candidate instance names.
        candidates: Vec<Name>,
    },
    /// Recursive search went too deep.
    MaxDepthExceeded {
        /// The depth at which the search was aborted.
        depth: usize,
    },
    /// Circular dependency between instances.
    CircularDependency {
        /// The dependency chain forming the cycle.
        chain: Vec<Name>,
    },
    /// Instance has unresolvable sub-goals.
    UnresolvableSubgoal {
        /// The instance with the unresolvable sub-goal.
        instance: Name,
        /// The sub-goal that could not be resolved.
        subgoal: Name,
    },
}
/// An extended representation of a typeclass instance, including
/// diamond-inheritance metadata and priority queue support.
#[derive(Debug, Clone)]
pub struct TypeclassInstance {
    /// Core instance declaration.
    pub decl: InstanceDecl,
    /// Sub-instances required to build this instance (dependencies).
    pub dependencies: Vec<Name>,
    /// Functional dependency chains from superclasses.
    pub super_instances: Vec<Name>,
    /// Whether this is a "default" instance (lower priority).
    pub is_default: bool,
    /// Whether this is a "local" instance (file-scoped).
    pub is_local: bool,
}
impl TypeclassInstance {
    /// Create a new typeclass instance.
    pub fn new(decl: InstanceDecl) -> Self {
        Self {
            decl,
            dependencies: Vec::new(),
            super_instances: Vec::new(),
            is_default: false,
            is_local: false,
        }
    }
    /// Mark as a default instance.
    pub fn default_instance(mut self) -> Self {
        self.is_default = true;
        self
    }
    /// Mark as a local instance.
    pub fn local(mut self) -> Self {
        self.is_local = true;
        self
    }
    /// Add a dependency.
    pub fn add_dependency(&mut self, dep: Name) {
        self.dependencies.push(dep);
    }
    /// Add a super-instance.
    pub fn add_super(&mut self, sup: Name) {
        self.super_instances.push(sup);
    }
    /// Return the class name.
    pub fn class(&self) -> &Name {
        &self.decl.class
    }
    /// Return the instance name.
    pub fn name(&self) -> &Name {
        &self.decl.name
    }
    /// Return the instance priority.
    pub fn priority(&self) -> u32 {
        self.decl.priority
    }
}
/// A cache entry for a resolved instance.
#[derive(Debug, Clone)]
pub struct InstanceCacheEntry {
    /// The class and type key.
    pub key: String,
    /// The resolved chain.
    pub chain: InstanceChain,
    /// How many times this entry was hit.
    pub hit_count: usize,
}
/// A cache for resolved instance chains.
#[derive(Debug, Default)]
pub struct InstanceChainCache {
    entries: HashMap<String, InstanceCacheEntry>,
}
impl InstanceChainCache {
    /// Create an empty instance cache.
    pub fn new() -> Self {
        Self::default()
    }
    /// Store a resolved chain.
    pub fn store(&mut self, key: impl Into<String>, chain: InstanceChain) {
        let key = key.into();
        self.entries.insert(
            key.clone(),
            InstanceCacheEntry {
                key,
                chain,
                hit_count: 0,
            },
        );
    }
    /// Look up a cached chain.
    pub fn lookup(&mut self, key: &str) -> Option<&InstanceChain> {
        if let Some(entry) = self.entries.get_mut(key) {
            entry.hit_count += 1;
            Some(&entry.chain)
        } else {
            None
        }
    }
    /// Return the number of entries.
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    /// Return true if the cache is empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
    /// Clear all entries.
    pub fn clear(&mut self) {
        self.entries.clear();
    }
    /// Return total hit count across all entries.
    pub fn total_hits(&self) -> usize {
        self.entries.values().map(|e| e.hit_count).sum()
    }
}
/// Configuration for instance synthesis.
#[derive(Debug, Clone)]
pub struct SynthConfig {
    /// Maximum search depth.
    pub max_depth: usize,
    /// Maximum total instances to explore.
    pub max_instances: usize,
    /// Whether to allow default instances.
    pub allow_defaults: bool,
    /// Diamond resolution strategy.
    pub diamond_strategy: DiamondResolutionStrategy,
}
/// Represents a match outcome between a type and an instance head.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MatchOutcome {
    /// The types match (possibly with substitutions).
    Match,
    /// The types do not match.
    NoMatch,
    /// Matching is deferred (requires further unification).
    Deferred,
}
/// Strategy for resolving diamond inheritance in typeclass hierarchies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DiamondResolutionStrategy {
    /// Reject all ambiguous instances (strict mode).
    Strict,
    /// Prefer the instance with the lowest priority number.
    #[default]
    PreferLowestPriority,
    /// Prefer the most recently declared instance.
    PreferMostRecent,
    /// Use depth-first search order.
    DepthFirst,
}
/// Statistics about instance resolution activity.
#[derive(Debug, Clone, Default)]
pub struct InstanceRegistryStats {
    /// Number of successful resolutions.
    pub successes: usize,
    /// Number of failed resolutions.
    pub failures: usize,
    /// Number of ambiguous resolutions.
    pub ambiguous: usize,
    /// Total depth explored.
    pub total_depth: usize,
}
impl InstanceRegistryStats {
    /// Create new empty stats.
    pub fn new() -> Self {
        Self::default()
    }
    /// Record a successful resolution.
    pub fn record_success(&mut self, depth: usize) {
        self.successes += 1;
        self.total_depth += depth;
    }
    /// Record a failed resolution.
    pub fn record_failure(&mut self) {
        self.failures += 1;
    }
    /// Record an ambiguous resolution.
    pub fn record_ambiguous(&mut self) {
        self.ambiguous += 1;
    }
    /// Return the total number of resolutions attempted.
    pub fn total(&self) -> usize {
        self.successes + self.failures + self.ambiguous
    }
    /// Return the success rate.
    pub fn success_rate(&self) -> f64 {
        if self.total() == 0 {
            0.0
        } else {
            self.successes as f64 / self.total() as f64
        }
    }
    /// Return the average depth of successful resolutions.
    pub fn average_depth(&self) -> f64 {
        if self.successes == 0 {
            0.0
        } else {
            self.total_depth as f64 / self.successes as f64
        }
    }
}
/// A priority queue of instance candidates, sorted by priority (ascending = higher priority).
#[derive(Debug, Clone, Default)]
pub struct InstancePriorityQueue {
    /// Candidates, sorted by priority.
    candidates: Vec<TypeclassInstance>,
}
impl InstancePriorityQueue {
    /// Create an empty priority queue.
    pub fn new() -> Self {
        Self::default()
    }
    /// Insert an instance, maintaining sorted order.
    pub fn push(&mut self, inst: TypeclassInstance) {
        let pos = self
            .candidates
            .partition_point(|c| c.priority() <= inst.priority());
        self.candidates.insert(pos, inst);
    }
    /// Remove and return the highest-priority candidate (lowest priority number).
    pub fn pop_best(&mut self) -> Option<TypeclassInstance> {
        if self.candidates.is_empty() {
            None
        } else {
            Some(self.candidates.remove(0))
        }
    }
    /// Peek at the highest-priority candidate.
    pub fn peek_best(&self) -> Option<&TypeclassInstance> {
        self.candidates.first()
    }
    /// Return all candidates with the same best (lowest) priority.
    pub fn best_candidates(&self) -> Vec<&TypeclassInstance> {
        if self.candidates.is_empty() {
            return Vec::new();
        }
        let best_priority = self.candidates[0].priority();
        self.candidates
            .iter()
            .take_while(|c| c.priority() == best_priority)
            .collect()
    }
    /// Return the number of candidates.
    pub fn len(&self) -> usize {
        self.candidates.len()
    }
    /// Return true if the queue is empty.
    pub fn is_empty(&self) -> bool {
        self.candidates.is_empty()
    }
    /// Clear all candidates.
    pub fn clear(&mut self) {
        self.candidates.clear();
    }
    /// Return all candidates (in priority order).
    pub fn all_candidates(&self) -> &[TypeclassInstance] {
        &self.candidates
    }
}
/// A scope that can hold local instances (which shadow global ones).
#[derive(Debug, Default)]
pub struct InstanceScope {
    /// Local instances in this scope.
    local_instances: Vec<TypeclassInstance>,
    /// Whether this scope is active.
    active: bool,
}
impl InstanceScope {
    /// Create a new scope.
    pub fn new() -> Self {
        Self {
            local_instances: Vec::new(),
            active: true,
        }
    }
    /// Add a local instance.
    pub fn add(&mut self, inst: TypeclassInstance) {
        self.local_instances.push(inst);
    }
    /// Return local instances for a given class.
    pub fn instances_for_class(&self, class: &Name) -> Vec<&TypeclassInstance> {
        if !self.active {
            return Vec::new();
        }
        self.local_instances
            .iter()
            .filter(|i| i.class() == class)
            .collect()
    }
    /// Return the number of local instances.
    pub fn len(&self) -> usize {
        self.local_instances.len()
    }
    /// Return true if the scope is empty.
    pub fn is_empty(&self) -> bool {
        self.local_instances.is_empty()
    }
    /// Deactivate this scope.
    pub fn deactivate(&mut self) {
        self.active = false;
    }
    /// Return true if this scope is active.
    pub fn is_active(&self) -> bool {
        self.active
    }
}
/// An instance of a type class.
#[derive(Debug, Clone)]
pub struct InstanceDecl {
    /// Instance name
    pub name: Name,
    /// Type class
    pub class: Name,
    /// Type being instantiated
    pub ty: Expr,
    /// Instance priority (lower = higher priority)
    pub priority: u32,
}
/// The result of instance synthesis.
#[derive(Debug, Clone)]
pub enum SynthResult {
    /// Successfully synthesized an instance.
    Success(InstanceChain),
    /// Synthesis failed with an error.
    Failure(InstanceError),
    /// Synthesis is ambiguous (multiple chains found).
    Ambiguous(Vec<InstanceChain>),
}
impl SynthResult {
    /// Return true if synthesis succeeded.
    pub fn is_success(&self) -> bool {
        matches!(self, SynthResult::Success(_))
    }
    /// Return true if synthesis failed.
    pub fn is_failure(&self) -> bool {
        matches!(self, SynthResult::Failure(_))
    }
    /// Return true if synthesis is ambiguous.
    pub fn is_ambiguous(&self) -> bool {
        matches!(self, SynthResult::Ambiguous(_))
    }
    /// Extract the chain from a successful result.
    pub fn chain(&self) -> Option<&InstanceChain> {
        match self {
            SynthResult::Success(chain) => Some(chain),
            _ => None,
        }
    }
}
/// A directed graph of instance relationships.
///
/// Nodes are instances; edges represent "X depends on Y" (X needs Y as a sub-instance).
#[derive(Debug, Default)]
pub struct InstanceGraph {
    edges: HashMap<Name, Vec<Name>>,
}
impl InstanceGraph {
    /// Create an empty instance graph.
    pub fn new() -> Self {
        Self::default()
    }
    /// Add an edge: `from` depends on `to`.
    pub fn add_edge(&mut self, from: Name, to: Name) {
        self.edges.entry(from).or_default().push(to);
    }
    /// Return the dependencies of the given instance.
    pub fn dependencies(&self, name: &Name) -> &[Name] {
        self.edges.get(name).map(|v| v.as_slice()).unwrap_or(&[])
    }
    /// Return true if there is an edge from `from` to `to`.
    pub fn has_edge(&self, from: &Name, to: &Name) -> bool {
        self.edges
            .get(from)
            .map(|deps| deps.contains(to))
            .unwrap_or(false)
    }
    /// Return all nodes in the graph.
    pub fn nodes(&self) -> Vec<&Name> {
        self.edges.keys().collect()
    }
    /// Detect cycles using DFS. Returns true if a cycle is found.
    pub fn has_cycle(&self) -> bool {
        let mut visited = std::collections::HashSet::new();
        let mut rec_stack = std::collections::HashSet::new();
        for node in self.edges.keys() {
            if self.dfs_cycle_check(node, &mut visited, &mut rec_stack) {
                return true;
            }
        }
        false
    }
    fn dfs_cycle_check(
        &self,
        node: &Name,
        visited: &mut std::collections::HashSet<Name>,
        rec_stack: &mut std::collections::HashSet<Name>,
    ) -> bool {
        if rec_stack.contains(node) {
            return true;
        }
        if visited.contains(node) {
            return false;
        }
        visited.insert(node.clone());
        rec_stack.insert(node.clone());
        for dep in self.dependencies(node) {
            if self.dfs_cycle_check(dep, visited, rec_stack) {
                return true;
            }
        }
        rec_stack.remove(node);
        false
    }
}
/// A summary report of an instance resolution attempt.
#[derive(Debug, Clone)]
pub struct InstanceReport {
    /// The class that was resolved.
    pub class: Name,
    /// The chain found (or None if resolution failed).
    pub chain: Option<InstanceChain>,
    /// The error (if resolution failed).
    pub error: Option<InstanceError>,
    /// The number of instances explored.
    pub instances_explored: usize,
}
impl InstanceReport {
    /// Create a successful report.
    pub fn success(class: Name, chain: InstanceChain, explored: usize) -> Self {
        Self {
            class,
            chain: Some(chain),
            error: None,
            instances_explored: explored,
        }
    }
    /// Create a failure report.
    pub fn failure(class: Name, error: InstanceError, explored: usize) -> Self {
        Self {
            class,
            chain: None,
            error: Some(error),
            instances_explored: explored,
        }
    }
    /// Return true if the resolution succeeded.
    pub fn is_success(&self) -> bool {
        self.chain.is_some()
    }
}
/// Cache of previously resolved instances.
#[derive(Debug, Default)]
pub struct InstanceCache {
    entries: HashMap<CacheKey, Name>,
    hits: u64,
    misses: u64,
}
impl InstanceCache {
    /// Create an empty cache.
    pub fn new() -> Self {
        Self::default()
    }
    /// Insert a successful resolution into the cache.
    pub fn insert(&mut self, key: CacheKey, instance_name: Name) {
        self.entries.insert(key, instance_name);
    }
    /// Look up a cached resolution.
    pub fn get(&mut self, key: &CacheKey) -> Option<&Name> {
        if let Some(v) = self.entries.get(key) {
            self.hits += 1;
            Some(v)
        } else {
            self.misses += 1;
            None
        }
    }
    /// Return the number of cache hits.
    pub fn hits(&self) -> u64 {
        self.hits
    }
    /// Return the number of cache misses.
    pub fn misses(&self) -> u64 {
        self.misses
    }
    /// Return total queries served.
    pub fn total_queries(&self) -> u64 {
        self.hits + self.misses
    }
    /// Clear all cached entries.
    pub fn clear(&mut self) {
        self.entries.clear();
    }
    /// Return the number of cached entries.
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    /// Return whether the cache is empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}
/// A chain of instance applications that resolves a typeclass constraint.
#[derive(Debug, Clone)]
pub struct InstanceChain {
    /// The steps in the chain (instance names, from root to leaf).
    pub steps: Vec<Name>,
    /// The total cost of this chain (sum of priorities).
    pub total_cost: u32,
    /// Whether this chain involves any default instances.
    pub has_default: bool,
}
impl InstanceChain {
    /// Create an empty chain.
    pub fn empty() -> Self {
        Self {
            steps: Vec::new(),
            total_cost: 0,
            has_default: false,
        }
    }
    /// Create a single-step chain.
    pub fn single(name: Name, cost: u32) -> Self {
        Self {
            steps: vec![name],
            total_cost: cost,
            has_default: false,
        }
    }
    /// Extend the chain with a new step.
    pub fn extend(&self, name: Name, cost: u32, is_default: bool) -> Self {
        let mut new_chain = self.clone();
        new_chain.steps.push(name);
        new_chain.total_cost += cost;
        if is_default {
            new_chain.has_default = true;
        }
        new_chain
    }
    /// Return the number of steps.
    pub fn len(&self) -> usize {
        self.steps.len()
    }
    /// Return true if this chain has no steps.
    pub fn is_empty(&self) -> bool {
        self.steps.is_empty()
    }
    /// Return the last step in the chain (the leaf instance), if any.
    pub fn leaf(&self) -> Option<&Name> {
        self.steps.last()
    }
}
/// An `InstanceResolver` that also records search statistics.
pub struct TracedResolver {
    /// Underlying resolver.
    pub resolver: InstanceResolver,
    /// Collected statistics.
    pub stats: SearchStats,
}
impl TracedResolver {
    /// Create a new traced resolver.
    pub fn new() -> Self {
        Self {
            resolver: InstanceResolver::new(),
            stats: SearchStats::new(),
        }
    }
    /// Register an instance in the underlying resolver.
    pub fn register(&mut self, inst: InstanceDecl) {
        self.resolver.register(inst);
    }
    /// Resolve and record statistics.
    pub fn resolve(&mut self, class: &Name, ty: &Expr) -> ResolutionResult {
        let n_before = self.resolver.get_instances(class).len() as u64;
        let result = self.resolver.resolve(class, ty);
        match &result {
            ResolutionResult::Found(_) => self.stats.record_success(n_before),
            ResolutionResult::NotFound => self.stats.record_failure(),
            ResolutionResult::Ambiguous(_) => self.stats.record_ambiguous(),
            ResolutionResult::DepthExceeded => self.stats.record_depth_exceeded(),
        }
        result
    }
}
/// Mutable state accumulated during instance search.
#[derive(Debug, Default)]
pub struct InstanceSearchState {
    /// Instances that have already been tried (to avoid revisiting).
    pub tried: std::collections::HashSet<Name>,
    /// The current depth of the search.
    pub depth: usize,
    /// The maximum depth reached so far.
    pub max_depth_reached: usize,
    /// Total number of nodes explored.
    pub nodes_explored: usize,
}
impl InstanceSearchState {
    /// Create a new search state.
    pub fn new() -> Self {
        Self::default()
    }
    /// Mark an instance as tried.
    pub fn mark_tried(&mut self, name: Name) {
        self.tried.insert(name);
    }
    /// Return true if the instance has been tried.
    pub fn already_tried(&self, name: &Name) -> bool {
        self.tried.contains(name)
    }
    /// Enter a new search depth.
    pub fn enter_depth(&mut self) {
        self.depth += 1;
        if self.depth > self.max_depth_reached {
            self.max_depth_reached = self.depth;
        }
        self.nodes_explored += 1;
    }
    /// Exit a search depth.
    pub fn exit_depth(&mut self) {
        if self.depth > 0 {
            self.depth -= 1;
        }
    }
    /// Return true if the current depth exceeds the given limit.
    pub fn depth_exceeded(&self, limit: usize) -> bool {
        self.depth > limit
    }
}
/// Diagnostic statistics collected during instance resolution.
#[derive(Debug, Default, Clone)]
pub struct SearchStats {
    /// Number of successful resolutions.
    pub successes: u64,
    /// Number of failed resolutions (not-found).
    pub failures: u64,
    /// Number of ambiguous resolutions.
    pub ambiguous: u64,
    /// Number of times the depth limit was reached.
    pub depth_exceeded: u64,
    /// Total instances inspected across all queries.
    pub instances_inspected: u64,
}
impl SearchStats {
    /// Create zeroed statistics.
    pub fn new() -> Self {
        Self::default()
    }
    /// Record a successful resolution that inspected `n` candidates.
    pub fn record_success(&mut self, candidates_inspected: u64) {
        self.successes += 1;
        self.instances_inspected += candidates_inspected;
    }
    /// Record a failed resolution.
    pub fn record_failure(&mut self) {
        self.failures += 1;
    }
    /// Record an ambiguous resolution.
    pub fn record_ambiguous(&mut self) {
        self.ambiguous += 1;
    }
    /// Record that the depth limit was exceeded.
    pub fn record_depth_exceeded(&mut self) {
        self.depth_exceeded += 1;
    }
    /// Total number of queries processed.
    pub fn total_queries(&self) -> u64 {
        self.successes + self.failures + self.ambiguous + self.depth_exceeded
    }
    /// Success rate as a value in `[0.0, 1.0]`.
    pub fn success_rate(&self) -> f64 {
        let total = self.total_queries();
        if total == 0 {
            1.0
        } else {
            self.successes as f64 / total as f64
        }
    }
}
/// A scoped stack of locally visible instances.
///
/// Used inside `do`-notation, tactic blocks, and `haveI`/`letI` binders
/// where an instance is in scope only for the duration of a sub-expression.
#[derive(Debug, Default)]
pub struct LocalInstanceScope {
    stack: Vec<Vec<InstanceDecl>>,
}
impl LocalInstanceScope {
    /// Create a new empty scope.
    pub fn new() -> Self {
        Self::default()
    }
    /// Push a new layer onto the scope stack.
    pub fn push_layer(&mut self) {
        self.stack.push(Vec::new());
    }
    /// Pop the top layer from the scope stack.
    ///
    /// Returns the instances that were in the top layer.
    pub fn pop_layer(&mut self) -> Vec<InstanceDecl> {
        self.stack.pop().unwrap_or_default()
    }
    /// Add an instance to the current (top) layer.
    ///
    /// Returns `false` if no layer has been pushed yet.
    pub fn add_to_current(&mut self, inst: InstanceDecl) -> bool {
        if let Some(top) = self.stack.last_mut() {
            top.push(inst);
            true
        } else {
            false
        }
    }
    /// Collect all instances visible in the current scope (all layers).
    pub fn visible_instances(&self) -> Vec<&InstanceDecl> {
        self.stack.iter().flatten().collect()
    }
    /// Return the current depth of the scope stack.
    pub fn depth(&self) -> usize {
        self.stack.len()
    }
    /// Check whether the scope is empty (no layers).
    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }
}
/// An orchestrator for instance synthesis.
#[derive(Default)]
pub struct InstanceSynthesizer {
    config: SynthConfig,
    graph: InstanceGraph,
}
impl InstanceSynthesizer {
    /// Create a new synthesizer with default config.
    pub fn new() -> Self {
        Self::default()
    }
    /// Create a synthesizer with custom config.
    pub fn with_config(config: SynthConfig) -> Self {
        Self {
            config,
            graph: InstanceGraph::new(),
        }
    }
    /// Return the current configuration.
    pub fn config(&self) -> &SynthConfig {
        &self.config
    }
    /// Return the instance dependency graph.
    pub fn graph(&self) -> &InstanceGraph {
        &self.graph
    }
    /// Add an instance edge to the dependency graph.
    pub fn add_dependency(&mut self, from: Name, to: Name) {
        self.graph.add_edge(from, to);
    }
    /// Check if there are any circular dependencies.
    pub fn has_circular_deps(&self) -> bool {
        self.graph.has_cycle()
    }
}
/// A key used for instance resolution caching.
///
/// Caches successful resolutions to avoid redundant search.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CacheKey {
    /// The type class name.
    pub class: Name,
    /// A stable string representation of the type being resolved.
    pub ty_repr: String,
}
impl CacheKey {
    /// Build a cache key from a class name and expression.
    pub fn new(class: &Name, ty: &Expr) -> Self {
        Self {
            class: class.clone(),
            ty_repr: format!("{:?}", ty),
        }
    }
}
/// Priority group: a collection of instances sharing the same numeric priority.
#[derive(Clone, Debug)]
pub struct PriorityGroup {
    /// Shared priority value.
    pub priority: u32,
    /// Members.
    pub instances: Vec<InstanceDecl>,
}
impl PriorityGroup {
    /// Create a new group.
    pub fn new(priority: u32) -> Self {
        Self {
            priority,
            instances: Vec::new(),
        }
    }
    /// Add an instance to the group.
    pub fn add(&mut self, inst: InstanceDecl) {
        self.instances.push(inst);
    }
    /// Number of instances in this group.
    pub fn len(&self) -> usize {
        self.instances.len()
    }
    /// Check if the group is empty.
    pub fn is_empty(&self) -> bool {
        self.instances.is_empty()
    }
    /// Check if the group is unambiguous (exactly one instance).
    pub fn is_unambiguous(&self) -> bool {
        self.instances.len() == 1
    }
}
/// A typed instance search path — the sequence of class/type pairs explored
/// during a single resolution attempt.
#[derive(Clone, Debug, Default)]
pub struct SearchPath {
    /// Steps in the search, in order.
    steps: Vec<(Name, Name)>,
}
impl SearchPath {
    /// Create an empty path.
    pub fn new() -> Self {
        Self::default()
    }
    /// Push a step (class, instance_name).
    pub fn push(&mut self, class: Name, instance: Name) {
        self.steps.push((class, instance));
    }
    /// Number of steps.
    pub fn len(&self) -> usize {
        self.steps.len()
    }
    /// Check if empty.
    pub fn is_empty(&self) -> bool {
        self.steps.is_empty()
    }
    /// Get step at index `i`.
    pub fn get(&self, i: usize) -> Option<&(Name, Name)> {
        self.steps.get(i)
    }
    /// Check if the path contains a given class (cycle detection).
    pub fn has_class(&self, class: &Name) -> bool {
        self.steps.iter().any(|(c, _)| c == class)
    }
    /// Clear all steps.
    pub fn clear(&mut self) {
        self.steps.clear();
    }
}
