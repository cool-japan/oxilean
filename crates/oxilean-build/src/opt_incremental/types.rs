//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::{HashMap, HashSet, VecDeque};

/// Tracks which source roots exist and maps them to module name prefixes.
pub struct SourceRootTracker {
    /// root_path → module_prefix (e.g. "src" → "Mathlib")
    roots: HashMap<String, String>,
}
impl SourceRootTracker {
    /// Create an empty tracker.
    pub fn new() -> Self {
        Self {
            roots: HashMap::new(),
        }
    }
    /// Register a source root.
    pub fn register(&mut self, root_path: &str, module_prefix: &str) {
        self.roots
            .insert(root_path.to_string(), module_prefix.to_string());
    }
    /// Determine the module name for a given file path.
    pub fn module_for_path(&self, file_path: &str) -> Option<String> {
        for (root, prefix) in &self.roots {
            if let Some(rel) = file_path.strip_prefix(root.as_str()) {
                let rel = rel.trim_start_matches('/').trim_end_matches(".lean");
                let mod_name = format!("{}.{}", prefix, rel.replace('/', "."));
                return Some(mod_name);
            }
        }
        None
    }
    /// Number of registered roots.
    pub fn root_count(&self) -> usize {
        self.roots.len()
    }
}
/// Convenience builder for `IncrementalGraph`.
pub struct DependencyGraphBuilder {
    graph: IncrementalGraph,
}
impl DependencyGraphBuilder {
    /// Start a new builder.
    pub fn new() -> Self {
        Self {
            graph: IncrementalGraph::new(),
        }
    }
    /// Register a module with the given fingerprint fields.
    pub fn add_module(mut self, name: &str, size: u64, mtime_ns: u64, content_hash: u64) -> Self {
        let fp = FileFingerprint::new(name, size, mtime_ns, content_hash);
        self.graph.add_module(name, fp);
        self
    }
    /// Add a direct dependency edge.
    pub fn depends(mut self, from: &str, on: &str) -> Self {
        self.graph.add_edge(from, on, EdgeKind::Direct);
        self
    }
    /// Add a type-only dependency edge.
    pub fn type_depends(mut self, from: &str, on: &str) -> Self {
        self.graph.add_edge(from, on, EdgeKind::TypeOnly);
        self
    }
    /// Consume the builder and return the graph.
    pub fn build(self) -> IncrementalGraph {
        self.graph
    }
}
/// A record of one completed module compilation, useful for analytics.
#[derive(Clone, Debug)]
pub struct ModuleCompileRecord {
    /// Module name.
    pub module: String,
    /// Whether the result was served from cache.
    pub from_cache: bool,
    /// Wall-clock time in milliseconds.
    pub elapsed_ms: u64,
    /// Number of warnings emitted.
    pub warning_count: u32,
    /// Whether the compilation succeeded.
    pub success: bool,
}
impl ModuleCompileRecord {
    /// Create a successful compile record.
    pub fn success(module: &str, from_cache: bool, elapsed_ms: u64, warnings: u32) -> Self {
        Self {
            module: module.to_string(),
            from_cache,
            elapsed_ms,
            warning_count: warnings,
            success: true,
        }
    }
    /// Create a failed compile record.
    pub fn failure(module: &str, elapsed_ms: u64) -> Self {
        Self {
            module: module.to_string(),
            from_cache: false,
            elapsed_ms,
            warning_count: 0,
            success: false,
        }
    }
}
/// Utility for filtering dependency edges by kind.
pub struct GraphEdgeKindFilter;
impl GraphEdgeKindFilter {
    /// Collect all direct dependents of `module` in `graph`.
    pub fn direct_dependents(graph: &IncrementalGraph, module: &str) -> Vec<String> {
        graph
            .edges
            .iter()
            .filter(|e| e.to == module && e.edge_kind == EdgeKind::Direct)
            .map(|e| e.from.clone())
            .collect()
    }
    /// Collect all type-only dependents of `module` in `graph`.
    pub fn type_only_dependents(graph: &IncrementalGraph, module: &str) -> Vec<String> {
        graph
            .edges
            .iter()
            .filter(|e| e.to == module && e.edge_kind == EdgeKind::TypeOnly)
            .map(|e| e.from.clone())
            .collect()
    }
    /// Count edges of each kind.
    pub fn edge_kind_counts(graph: &IncrementalGraph) -> HashMap<String, usize> {
        let mut counts: HashMap<String, usize> = HashMap::new();
        for edge in &graph.edges {
            let kind_str = match edge.edge_kind {
                EdgeKind::Direct => "direct",
                EdgeKind::Transitive => "transitive",
                EdgeKind::TypeOnly => "type-only",
            };
            *counts.entry(kind_str.to_string()).or_insert(0) += 1;
        }
        counts
    }
}
/// A full cache record for one module, combining fingerprint + interface hash.
#[derive(Clone, Debug)]
pub struct IncrementalCacheEntry {
    /// Source fingerprint.
    pub source_fp: FileFingerprint,
    /// Interface hash (stable under impl-only changes).
    pub iface_hash: InterfaceHash,
    /// Artifact paths produced by this compilation.
    pub artifacts: Vec<String>,
    /// Compilation time in milliseconds.
    pub compile_ms: u64,
}
impl IncrementalCacheEntry {
    /// Create a new entry.
    pub fn new(
        source_fp: FileFingerprint,
        iface_hash: InterfaceHash,
        artifacts: Vec<String>,
        compile_ms: u64,
    ) -> Self {
        Self {
            source_fp,
            iface_hash,
            artifacts,
            compile_ms,
        }
    }
    /// Number of artifacts.
    pub fn artifact_count(&self) -> usize {
        self.artifacts.len()
    }
}
/// Describes a dependency cycle detected in the build graph.
#[derive(Clone, Debug)]
pub struct BuildCycle {
    /// Modules forming the cycle (in order).
    pub modules: Vec<String>,
}
impl BuildCycle {
    /// Create a cycle from a list of modules.
    pub fn new(modules: Vec<String>) -> Self {
        Self { modules }
    }
    /// Number of modules in the cycle.
    pub fn len(&self) -> usize {
        self.modules.len()
    }
    /// Human-readable cycle description.
    pub fn description(&self) -> String {
        self.modules.join(" -> ")
    }
}
/// Configuration knobs for the incremental compilation engine.
#[derive(Clone, Debug)]
pub struct IncrementalCompilationConfig {
    /// Whether to use interface hashes for fine-grained invalidation.
    pub use_interface_hashes: bool,
    /// Whether to propagate invalidation transitively.
    pub transitive_propagation: bool,
    /// Whether to persist fingerprints across sessions.
    pub persist_fingerprints: bool,
    /// Maximum number of modules to rebuild in parallel.
    pub max_parallel_rebuilds: usize,
    /// Whether to log detailed incremental decisions.
    pub verbose_logging: bool,
}
impl IncrementalCompilationConfig {
    /// Create a config suitable for CI (more conservative).
    pub fn ci() -> Self {
        Self {
            use_interface_hashes: true,
            transitive_propagation: true,
            persist_fingerprints: false,
            max_parallel_rebuilds: 2,
            verbose_logging: true,
        }
    }
    /// Enable verbose logging.
    pub fn with_verbose(mut self) -> Self {
        self.verbose_logging = true;
        self
    }
    /// Set parallel rebuild limit.
    pub fn with_parallelism(mut self, n: usize) -> Self {
        self.max_parallel_rebuilds = n.max(1);
        self
    }
}
/// The reason a module was invalidated.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InvalidationCause {
    /// Source file content changed.
    SourceChanged,
    /// A dependency was invalidated.
    DependencyInvalidated(String),
    /// The module was explicitly invalidated by the user.
    Explicit,
    /// The compiler version changed.
    CompilerVersionChanged,
    /// A build flag changed.
    BuildFlagChanged(String),
}
/// A directed dependency edge between two modules.
#[derive(Clone, Debug)]
pub struct DependencyEdge {
    pub from: String,
    pub to: String,
    pub edge_kind: EdgeKind,
}
/// Accumulates a batch of filesystem changes before feeding them to the graph.
pub struct ChangeBatch {
    pub changed_files: Vec<String>,
    pub added_files: Vec<String>,
    pub removed_files: Vec<String>,
}
impl ChangeBatch {
    pub fn new() -> Self {
        Self {
            changed_files: Vec::new(),
            added_files: Vec::new(),
            removed_files: Vec::new(),
        }
    }
    pub fn record_change(&mut self, path: &str) {
        self.changed_files.push(path.to_string());
    }
    pub fn record_add(&mut self, path: &str) {
        self.added_files.push(path.to_string());
    }
    pub fn record_remove(&mut self, path: &str) {
        self.removed_files.push(path.to_string());
    }
    pub fn is_empty(&self) -> bool {
        self.changed_files.is_empty()
            && self.added_files.is_empty()
            && self.removed_files.is_empty()
    }
    pub fn total_changes(&self) -> usize {
        self.changed_files.len() + self.added_files.len() + self.removed_files.len()
    }
}
/// Computes and stores fingerprints for a set of modules.
pub struct IncrementalFingerprinter {
    /// module_name → current fingerprint hash.
    stored: HashMap<String, u64>,
}
impl IncrementalFingerprinter {
    /// Create an empty fingerprinter.
    pub fn new() -> Self {
        Self {
            stored: HashMap::new(),
        }
    }
    /// Compute a simple FNV-1a 64-bit hash of a byte slice.
    pub fn hash_content(data: &[u8]) -> u64 {
        let mut h: u64 = 0xcbf29ce484222325;
        for &b in data {
            h ^= b as u64;
            h = h.wrapping_mul(0x100000001b3);
        }
        h
    }
    /// Store the fingerprint for `module` computed from `content`.
    pub fn record(&mut self, module: &str, content: &[u8]) {
        self.stored
            .insert(module.to_string(), Self::hash_content(content));
    }
    /// Check whether `content` matches the stored fingerprint for `module`.
    pub fn is_up_to_date(&self, module: &str, content: &[u8]) -> bool {
        match self.stored.get(module) {
            Some(&stored_hash) => stored_hash == Self::hash_content(content),
            None => false,
        }
    }
    /// Remove the stored fingerprint for `module`.
    pub fn invalidate(&mut self, module: &str) {
        self.stored.remove(module);
    }
    /// Number of stored fingerprints.
    pub fn count(&self) -> usize {
        self.stored.len()
    }
    /// Whether any fingerprint is stored for `module`.
    pub fn has(&self, module: &str) -> bool {
        self.stored.contains_key(module)
    }
}
/// A combined cache of all incremental compilation entries.
pub struct FullIncrementalCache {
    entries: HashMap<String, IncrementalCacheEntry>,
}
impl FullIncrementalCache {
    /// Create an empty cache.
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }
    /// Insert or replace an entry.
    pub fn put(&mut self, module: &str, entry: IncrementalCacheEntry) {
        self.entries.insert(module.to_string(), entry);
    }
    /// Retrieve an entry.
    pub fn get(&self, module: &str) -> Option<&IncrementalCacheEntry> {
        self.entries.get(module)
    }
    /// Remove an entry.
    pub fn invalidate(&mut self, module: &str) {
        self.entries.remove(module);
    }
    /// Number of entries.
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    /// Whether the cache is empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
    /// All module names with cached entries.
    pub fn cached_modules(&self) -> Vec<&str> {
        self.entries.keys().map(|k| k.as_str()).collect()
    }
    /// Total compile time across all cached entries.
    pub fn total_compile_ms(&self) -> u64 {
        self.entries.values().map(|e| e.compile_ms).sum()
    }
    /// Average compile time in milliseconds.
    pub fn avg_compile_ms(&self) -> f64 {
        if self.entries.is_empty() {
            0.0
        } else {
            self.total_compile_ms() as f64 / self.entries.len() as f64
        }
    }
}
/// Detects cycles in an `IncrementalGraph` using DFS.
pub struct CycleDetector;
impl CycleDetector {
    /// Detect the first cycle in the graph, if any.
    pub fn detect(graph: &IncrementalGraph) -> Option<BuildCycle> {
        let nodes: Vec<&str> = graph.nodes.keys().map(|k| k.as_str()).collect();
        let mut visited: HashSet<&str> = HashSet::new();
        let mut stack: HashSet<&str> = HashSet::new();
        let mut path: Vec<&str> = Vec::new();
        for node in nodes {
            if !visited.contains(node) {
                if let Some(cycle) = Self::dfs(graph, node, &mut visited, &mut stack, &mut path) {
                    return Some(cycle);
                }
            }
        }
        None
    }
    fn dfs<'a>(
        graph: &'a IncrementalGraph,
        node: &'a str,
        visited: &mut HashSet<&'a str>,
        stack: &mut HashSet<&'a str>,
        path: &mut Vec<&'a str>,
    ) -> Option<BuildCycle> {
        visited.insert(node);
        stack.insert(node);
        path.push(node);
        let neighbors: Vec<&str> = graph
            .edges
            .iter()
            .filter(|e| e.from == node)
            .map(|e| e.to.as_str())
            .collect();
        for neighbor in neighbors {
            if !visited.contains(neighbor) {
                if let Some(cycle) = Self::dfs(graph, neighbor, visited, stack, path) {
                    return Some(cycle);
                }
            } else if stack.contains(neighbor) {
                let start = path.iter().position(|&n| n == neighbor).unwrap_or(0);
                let cycle_modules: Vec<String> =
                    path[start..].iter().map(|s| s.to_string()).collect();
                return Some(BuildCycle::new(cycle_modules));
            }
        }
        stack.remove(node);
        path.pop();
        None
    }
    /// Whether the graph is acyclic.
    pub fn is_acyclic(graph: &IncrementalGraph) -> bool {
        Self::detect(graph).is_none()
    }
}
/// A human-readable report from one incremental build run.
#[derive(Clone, Debug, Default)]
pub struct IncrementalBuildReport {
    /// Modules that were skipped (up to date).
    pub skipped: Vec<String>,
    /// Modules that were rebuilt.
    pub rebuilt: Vec<String>,
    /// Modules that failed to rebuild.
    pub failed: Vec<String>,
    /// Total wall-clock time in milliseconds.
    pub elapsed_ms: u64,
}
impl IncrementalBuildReport {
    /// Create an empty report.
    pub fn new() -> Self {
        Self::default()
    }
    /// Record a skipped module.
    pub fn add_skipped(&mut self, module: &str) {
        self.skipped.push(module.to_string());
    }
    /// Record a rebuilt module.
    pub fn add_rebuilt(&mut self, module: &str) {
        self.rebuilt.push(module.to_string());
    }
    /// Record a failed module.
    pub fn add_failed(&mut self, module: &str) {
        self.failed.push(module.to_string());
    }
    /// Whether the build was completely successful.
    pub fn is_success(&self) -> bool {
        self.failed.is_empty()
    }
    /// Total modules processed.
    pub fn total(&self) -> usize {
        self.skipped.len() + self.rebuilt.len() + self.failed.len()
    }
    /// Summary string.
    pub fn summary(&self) -> String {
        format!(
            "rebuilt={} skipped={} failed={} elapsed={}ms",
            self.rebuilt.len(),
            self.skipped.len(),
            self.failed.len(),
            self.elapsed_ms,
        )
    }
}
/// Maintains the set of modules marked as dirty (needing recompilation).
pub struct DirtySet {
    inner: HashSet<String>,
}
impl DirtySet {
    /// Create an empty dirty set.
    pub fn new() -> Self {
        Self {
            inner: HashSet::new(),
        }
    }
    /// Mark a module as dirty.
    pub fn mark(&mut self, module: &str) {
        self.inner.insert(module.to_string());
    }
    /// Mark multiple modules as dirty.
    pub fn mark_all(&mut self, modules: &[&str]) {
        for &m in modules {
            self.mark(m);
        }
    }
    /// Unmark a module (e.g. after it has been rebuilt).
    pub fn unmark(&mut self, module: &str) {
        self.inner.remove(module);
    }
    /// Whether `module` is dirty.
    pub fn is_dirty(&self, module: &str) -> bool {
        self.inner.contains(module)
    }
    /// Number of dirty modules.
    pub fn len(&self) -> usize {
        self.inner.len()
    }
    /// Whether the dirty set is empty.
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
    /// All dirty modules as a sorted Vec.
    pub fn sorted(&self) -> Vec<&str> {
        let mut v: Vec<&str> = self.inner.iter().map(|s| s.as_str()).collect();
        v.sort_unstable();
        v
    }
    /// Clear all dirty flags.
    pub fn clear(&mut self) {
        self.inner.clear();
    }
}
/// The kind of an incremental build artifact.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IncrArtifactKind {
    /// Compiled object file (.o).
    Object,
    /// Type-checked interface file.
    Interface,
    /// Documentation output.
    Docs,
    /// Proof term export.
    ProofExport,
}
/// Persistent-style store for module fingerprints (in-memory stub).
pub struct FingerprintStore {
    data: HashMap<String, FileFingerprint>,
}
impl FingerprintStore {
    /// Create an empty store.
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }
    /// Store a fingerprint.
    pub fn put(&mut self, module: &str, fp: FileFingerprint) {
        self.data.insert(module.to_string(), fp);
    }
    /// Retrieve a stored fingerprint.
    pub fn get(&self, module: &str) -> Option<&FileFingerprint> {
        self.data.get(module)
    }
    /// Whether a fingerprint is stored.
    pub fn has(&self, module: &str) -> bool {
        self.data.contains_key(module)
    }
    /// Remove a fingerprint.
    pub fn remove(&mut self, module: &str) {
        self.data.remove(module);
    }
    /// Number of stored fingerprints.
    pub fn len(&self) -> usize {
        self.data.len()
    }
    /// Whether the store is empty.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    /// All module names that are stored.
    pub fn module_names(&self) -> Vec<&str> {
        self.data.keys().map(|k| k.as_str()).collect()
    }
    /// Merge another store into this one (overwrite on conflict).
    pub fn merge(&mut self, other: FingerprintStore) {
        for (k, v) in other.data {
            self.data.insert(k, v);
        }
    }
}
/// A bounded history of `ModuleCompileRecord`s for trend analysis.
pub struct CompileHistory {
    records: std::collections::VecDeque<ModuleCompileRecord>,
    max_size: usize,
}
impl CompileHistory {
    /// Create a history with the given capacity.
    pub fn with_capacity(max_size: usize) -> Self {
        Self {
            records: std::collections::VecDeque::new(),
            max_size,
        }
    }
    /// Add a record, evicting the oldest if at capacity.
    pub fn push(&mut self, rec: ModuleCompileRecord) {
        if self.records.len() >= self.max_size {
            self.records.pop_front();
        }
        self.records.push_back(rec);
    }
    /// Number of records.
    pub fn len(&self) -> usize {
        self.records.len()
    }
    /// Whether empty.
    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
    /// Cache hit rate across all records.
    pub fn cache_hit_rate(&self) -> f64 {
        if self.records.is_empty() {
            return 0.0;
        }
        let hits = self.records.iter().filter(|r| r.from_cache).count();
        hits as f64 / self.records.len() as f64
    }
    /// Average elapsed time.
    pub fn avg_elapsed_ms(&self) -> f64 {
        if self.records.is_empty() {
            return 0.0;
        }
        let total: u64 = self.records.iter().map(|r| r.elapsed_ms).sum();
        total as f64 / self.records.len() as f64
    }
    /// Success rate across all records.
    pub fn success_rate(&self) -> f64 {
        if self.records.is_empty() {
            return 1.0;
        }
        let ok = self.records.iter().filter(|r| r.success).count();
        ok as f64 / self.records.len() as f64
    }
}
/// Statistics gathered over an incremental build session.
#[derive(Clone, Debug, Default)]
pub struct IncrementalStats {
    /// Number of modules checked for staleness.
    pub modules_checked: u64,
    /// Number of modules that were up to date.
    pub cache_hits: u64,
    /// Number of modules that needed recompilation.
    pub cache_misses: u64,
    /// Total time saved (ms) by skipping cached modules.
    pub time_saved_ms: u64,
    /// Number of dependency edges propagated.
    pub edges_propagated: u64,
}
impl IncrementalStats {
    /// Create zeroed stats.
    pub fn new() -> Self {
        Self::default()
    }
    /// Record a cache hit for a module.
    pub fn record_hit(&mut self, time_saved_ms: u64) {
        self.modules_checked += 1;
        self.cache_hits += 1;
        self.time_saved_ms += time_saved_ms;
    }
    /// Record a cache miss.
    pub fn record_miss(&mut self) {
        self.modules_checked += 1;
        self.cache_misses += 1;
    }
    /// Hit rate as a fraction.
    pub fn hit_rate(&self) -> f64 {
        if self.modules_checked == 0 {
            0.0
        } else {
            self.cache_hits as f64 / self.modules_checked as f64
        }
    }
    /// Human-readable summary.
    pub fn summary(&self) -> String {
        format!(
            "checked={} hits={} misses={} hit_rate={:.1}% saved={}ms",
            self.modules_checked,
            self.cache_hits,
            self.cache_misses,
            self.hit_rate() * 100.0,
            self.time_saved_ms,
        )
    }
}
/// A typed cache of `InterfaceHash` values per module.
pub struct ModuleCache {
    hashes: HashMap<String, InterfaceHash>,
}
impl ModuleCache {
    /// Create an empty cache.
    pub fn new() -> Self {
        Self {
            hashes: HashMap::new(),
        }
    }
    /// Store the interface hash for `module`.
    pub fn store(&mut self, module: &str, hash: InterfaceHash) {
        self.hashes.insert(module.to_string(), hash);
    }
    /// Retrieve the interface hash for `module`.
    pub fn get(&self, module: &str) -> Option<InterfaceHash> {
        self.hashes.get(module).copied()
    }
    /// Returns `true` when the stored hash matches `hash`.
    pub fn is_current(&self, module: &str, hash: InterfaceHash) -> bool {
        self.hashes.get(module) == Some(&hash)
    }
    /// Remove the cached hash for `module`.
    pub fn invalidate(&mut self, module: &str) {
        self.hashes.remove(module);
    }
    /// Number of cached modules.
    pub fn len(&self) -> usize {
        self.hashes.len()
    }
    /// Whether the cache is empty.
    pub fn is_empty(&self) -> bool {
        self.hashes.is_empty()
    }
}
/// Detects which modules changed by comparing old and new fingerprints.
pub struct ChangeDetector {
    /// Old fingerprints: module_name → hash.
    old: HashMap<String, u64>,
    /// New fingerprints: module_name → hash.
    new: HashMap<String, u64>,
}
impl ChangeDetector {
    /// Create a detector from two fingerprint maps.
    pub fn new(old: HashMap<String, u64>, new: HashMap<String, u64>) -> Self {
        Self { old, new }
    }
    /// Modules whose fingerprint changed (or are new).
    pub fn changed_modules(&self) -> Vec<&str> {
        self.new
            .iter()
            .filter(|(k, &new_hash)| {
                self.old
                    .get(*k)
                    .map(|&old_hash| old_hash != new_hash)
                    .unwrap_or(true)
            })
            .map(|(k, _)| k.as_str())
            .collect()
    }
    /// Modules that were present in old but absent in new (deleted).
    pub fn deleted_modules(&self) -> Vec<&str> {
        self.old
            .keys()
            .filter(|k| !self.new.contains_key(*k))
            .map(|k| k.as_str())
            .collect()
    }
    /// Modules that are new (not in old).
    pub fn added_modules(&self) -> Vec<&str> {
        self.new
            .keys()
            .filter(|k| !self.old.contains_key(*k))
            .map(|k| k.as_str())
            .collect()
    }
    /// Total number of changes (changed + added + deleted).
    pub fn total_changes(&self) -> usize {
        self.changed_modules().len() + self.deleted_modules().len()
    }
}
/// Full incremental compilation state: fingerprints + dependency graph + dirty set.
pub struct IncrementalBuildState {
    pub graph: IncrementalGraph,
    pub fingerprinter: IncrementalFingerprinter,
    pub dirty: DirtySet,
    pub tracker: ArtifactTracker,
}
impl IncrementalBuildState {
    /// Create an empty state.
    pub fn new() -> Self {
        Self {
            graph: IncrementalGraph::new(),
            fingerprinter: IncrementalFingerprinter::new(),
            dirty: DirtySet::new(),
            tracker: ArtifactTracker::new(),
        }
    }
    /// Feed a file change: update fingerprint, mark module dirty, propagate.
    pub fn feed_change(&mut self, module: &str, new_content: &[u8]) {
        if !self.fingerprinter.is_up_to_date(module, new_content) {
            self.fingerprinter.record(module, new_content);
            self.dirty.mark(module);
            self.graph.invalidate(module);
        }
    }
    /// Mark a module as successfully built and clean it from the dirty set.
    pub fn mark_built(&mut self, module: &str, artifact_path: &str) {
        self.dirty.unmark(module);
        self.tracker.record(module, artifact_path);
    }
    /// Number of modules that need rebuilding.
    pub fn rebuild_count(&self) -> usize {
        self.dirty.len()
    }
    /// Whether all modules are up to date.
    pub fn is_clean(&self) -> bool {
        self.dirty.is_empty()
    }
}
/// Ordered log of `InvalidationRecord`s for audit/debug purposes.
pub struct InvalidationLog {
    records: Vec<InvalidationRecord>,
}
impl InvalidationLog {
    /// Create an empty log.
    pub fn new() -> Self {
        Self {
            records: Vec::new(),
        }
    }
    /// Append a record.
    pub fn push(&mut self, rec: InvalidationRecord) {
        self.records.push(rec);
    }
    /// Record an invalidation.
    pub fn record(&mut self, module: &str, cause: InvalidationCause, timestamp: u64) {
        self.push(InvalidationRecord::new(module, cause, timestamp));
    }
    /// Number of records.
    pub fn len(&self) -> usize {
        self.records.len()
    }
    /// Whether the log is empty.
    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
    /// Records for a specific module.
    pub fn for_module(&self, module: &str) -> Vec<&InvalidationRecord> {
        self.records.iter().filter(|r| r.module == module).collect()
    }
    /// Most recent record for `module`, if any.
    pub fn latest_for(&self, module: &str) -> Option<&InvalidationRecord> {
        self.records.iter().rev().find(|r| r.module == module)
    }
}
/// A scheduling hint produced by the incremental engine.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BuildScheduleHint {
    /// Module is up to date; skip.
    Skip,
    /// Module needs recompilation; rebuild it.
    Rebuild,
    /// Module's implementation changed but interface is stable; re-link only.
    RelinkOnly,
}
/// A FIFO queue of modules awaiting recompilation.
pub struct RebuildQueue {
    inner: VecDeque<String>,
}
impl RebuildQueue {
    /// Create an empty queue.
    pub fn new() -> Self {
        Self {
            inner: VecDeque::new(),
        }
    }
    /// Enqueue a module.
    pub fn push(&mut self, module: &str) {
        self.inner.push_back(module.to_string());
    }
    /// Enqueue multiple modules.
    pub fn push_all(&mut self, modules: &[&str]) {
        for &m in modules {
            self.push(m);
        }
    }
    /// Dequeue the next module.
    pub fn pop(&mut self) -> Option<String> {
        self.inner.pop_front()
    }
    /// Number of modules in the queue.
    pub fn len(&self) -> usize {
        self.inner.len()
    }
    /// Whether the queue is empty.
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
    /// Drain all modules into a Vec.
    pub fn drain_all(&mut self) -> Vec<String> {
        self.inner.drain(..).collect()
    }
}
/// A log record tracking why a module was invalidated.
#[derive(Clone, Debug)]
pub struct InvalidationRecord {
    /// The invalidated module.
    pub module: String,
    /// Why it was invalidated.
    pub cause: InvalidationCause,
    /// Timestamp (seconds) of the invalidation.
    pub timestamp: u64,
}
impl InvalidationRecord {
    /// Create a record.
    pub fn new(module: &str, cause: InvalidationCause, timestamp: u64) -> Self {
        Self {
            module: module.to_string(),
            cause,
            timestamp,
        }
    }
}
/// A filesystem watch event that may trigger an incremental rebuild.
#[derive(Clone, Debug)]
pub enum WatchEvent {
    /// File was modified.
    Modified(String),
    /// File was created.
    Created(String),
    /// File was deleted.
    Deleted(String),
    /// File was renamed from `old` to `new`.
    Renamed(String, String),
}
impl WatchEvent {
    /// The primary file path associated with this event.
    pub fn path(&self) -> &str {
        match self {
            WatchEvent::Modified(p) | WatchEvent::Created(p) | WatchEvent::Deleted(p) => p.as_str(),
            WatchEvent::Renamed(_, new) => new.as_str(),
        }
    }
    /// Whether the event indicates the file now exists.
    pub fn file_exists_after(&self) -> bool {
        !matches!(self, WatchEvent::Deleted(_))
    }
}
/// Tracks module fingerprints and dependency edges to determine what needs
/// recompilation after a change.
pub struct IncrementalGraph {
    /// module name → current fingerprint
    pub nodes: HashMap<String, FileFingerprint>,
    pub edges: Vec<DependencyEdge>,
    /// modules that need recompilation
    pub invalidated: HashSet<String>,
}
impl IncrementalGraph {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            edges: Vec::new(),
            invalidated: HashSet::new(),
        }
    }
    /// Register (or update) a module fingerprint.
    pub fn add_module(&mut self, name: &str, fp: FileFingerprint) {
        self.nodes.insert(name.to_string(), fp);
    }
    /// Add a dependency edge from `from` to `to`.
    pub fn add_edge(&mut self, from: &str, to: &str, kind: EdgeKind) {
        self.edges.push(DependencyEdge {
            from: from.to_string(),
            to: to.to_string(),
            edge_kind: kind,
        });
    }
    /// Mark `module` as invalid and propagate to all dependents via BFS.
    pub fn invalidate(&mut self, module: &str) {
        let mut queue: VecDeque<String> = VecDeque::new();
        queue.push_back(module.to_string());
        while let Some(current) = queue.pop_front() {
            if self.invalidated.insert(current.clone()) {
                let dependents: Vec<String> = self
                    .edges
                    .iter()
                    .filter(|e| e.to == current)
                    .map(|e| e.from.clone())
                    .collect();
                for dep in dependents {
                    queue.push_back(dep);
                }
            }
        }
    }
    /// Propagate invalidation to all transitive dependents of already-invalidated modules.
    pub fn propagate_invalidation(&mut self) {
        let seeds: Vec<String> = self.invalidated.iter().cloned().collect();
        for seed in seeds {
            let dependents: Vec<String> = self
                .edges
                .iter()
                .filter(|e| e.to == seed)
                .map(|e| e.from.clone())
                .collect();
            for dep in dependents {
                self.invalidate(&dep);
            }
        }
    }
    /// Returns names of all modules that require rebuilding.
    pub fn modules_to_rebuild(&self) -> Vec<&str> {
        self.invalidated.iter().map(|s| s.as_str()).collect()
    }
    /// Number of modules that do NOT need recompilation.
    pub fn clean_count(&self) -> usize {
        self.nodes
            .keys()
            .filter(|k| !self.invalidated.contains(*k))
            .count()
    }
    /// Returns `true` when `new_fp` differs from the stored fingerprint for `name`.
    pub fn fingerprint_changed(&self, name: &str, new_fp: &FileFingerprint) -> bool {
        match self.nodes.get(name) {
            Some(stored) => !stored.matches(new_fp),
            None => true,
        }
    }
    /// Replace the stored fingerprint for `name`.
    pub fn update_fingerprint(&mut self, name: &str, fp: FileFingerprint) {
        self.nodes.insert(name.to_string(), fp);
    }
}
/// Thread-safe-style wrapper around a dirty set (single-threaded stub).
pub struct ConcurrentInvalidationSet {
    inner: HashSet<String>,
}
impl ConcurrentInvalidationSet {
    /// Create empty.
    pub fn new() -> Self {
        Self {
            inner: HashSet::new(),
        }
    }
    /// Mark a module as dirty.
    pub fn mark(&mut self, module: &str) {
        self.inner.insert(module.to_string());
    }
    /// Unmark a module.
    pub fn unmark(&mut self, module: &str) {
        self.inner.remove(module);
    }
    /// Whether dirty.
    pub fn is_dirty(&self, module: &str) -> bool {
        self.inner.contains(module)
    }
    /// Drain all dirty modules.
    pub fn drain(&mut self) -> Vec<String> {
        self.inner.drain().collect()
    }
    /// Number of dirty modules.
    pub fn count(&self) -> usize {
        self.inner.len()
    }
}
/// A hash of a module's public interface, used for type-level dependency
/// invalidation: if the interface hash does not change, downstream type-only
/// dependents do not need recompilation even if the implementation changed.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct InterfaceHash(pub(super) u64);
impl InterfaceHash {
    /// Create from a precomputed hash value.
    pub fn new(hash: u64) -> Self {
        Self(hash)
    }
    /// Compute from a byte slice.
    pub fn from_bytes(data: &[u8]) -> Self {
        Self(IncrementalFingerprinter::hash_content(data))
    }
    /// The raw hash value.
    pub fn value(&self) -> u64 {
        self.0
    }
}
/// Classification of how one module depends on another.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EdgeKind {
    Direct,
    Transitive,
    TypeOnly,
}
/// Tracks which artifacts have been produced for each module during a build.
pub struct ArtifactTracker {
    /// module_name → list of output artifact paths.
    pub artifacts: HashMap<String, Vec<String>>,
}
impl ArtifactTracker {
    /// Create an empty tracker.
    pub fn new() -> Self {
        Self {
            artifacts: HashMap::new(),
        }
    }
    /// Record an artifact for `module`.
    pub fn record(&mut self, module: &str, path: &str) {
        self.artifacts
            .entry(module.to_string())
            .or_default()
            .push(path.to_string());
    }
    /// Return all artifact paths for `module`.
    pub fn get(&self, module: &str) -> &[String] {
        self.artifacts
            .get(module)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }
    /// Whether any artifacts exist for `module`.
    pub fn has_artifacts(&self, module: &str) -> bool {
        !self.get(module).is_empty()
    }
    /// Total number of artifact records.
    pub fn total_artifacts(&self) -> usize {
        self.artifacts.values().map(|v| v.len()).sum()
    }
    /// Remove all artifacts for `module`.
    pub fn remove(&mut self, module: &str) {
        self.artifacts.remove(module);
    }
    /// Clear all records.
    pub fn clear(&mut self) {
        self.artifacts.clear();
    }
}
/// Snapshot of a source file's identity used to detect changes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FileFingerprint {
    pub path: String,
    pub size: u64,
    pub mtime_ns: u64,
    pub content_hash: u64,
}
impl FileFingerprint {
    pub fn new(path: &str, size: u64, mtime_ns: u64, content_hash: u64) -> Self {
        Self {
            path: path.to_string(),
            size,
            mtime_ns,
            content_hash,
        }
    }
    /// Returns `true` when both fingerprints represent the same file content.
    pub fn matches(&self, other: &FileFingerprint) -> bool {
        self.size == other.size
            && self.mtime_ns == other.mtime_ns
            && self.content_hash == other.content_hash
    }
}
/// Generates `BuildScheduleHint` for each module based on incremental state.
pub struct IncrementalScheduler {
    module_cache: ModuleCache,
    fingerprinter: IncrementalFingerprinter,
}
impl IncrementalScheduler {
    /// Create a fresh scheduler.
    pub fn new() -> Self {
        Self {
            module_cache: ModuleCache::new(),
            fingerprinter: IncrementalFingerprinter::new(),
        }
    }
    /// Compute a hint for `module` given its current source content and interface bytes.
    pub fn hint(
        &self,
        module: &str,
        source_content: &[u8],
        interface_bytes: &[u8],
    ) -> BuildScheduleHint {
        let source_current = self.fingerprinter.is_up_to_date(module, source_content);
        if source_current {
            return BuildScheduleHint::Skip;
        }
        let new_iface_hash = InterfaceHash::from_bytes(interface_bytes);
        if self.module_cache.is_current(module, new_iface_hash) {
            BuildScheduleHint::RelinkOnly
        } else {
            BuildScheduleHint::Rebuild
        }
    }
    /// Record a completed build for `module`.
    pub fn record_built(&mut self, module: &str, source_content: &[u8], interface_bytes: &[u8]) {
        self.fingerprinter.record(module, source_content);
        self.module_cache
            .store(module, InterfaceHash::from_bytes(interface_bytes));
    }
    /// Invalidate `module` explicitly.
    pub fn invalidate(&mut self, module: &str) {
        self.fingerprinter.invalidate(module);
        self.module_cache.invalidate(module);
    }
}
/// High-level API combining all incremental compilation components.
pub struct IncrementalEngine {
    pub state: IncrementalBuildState,
    pub scheduler: IncrementalScheduler,
    pub queue: RebuildQueue,
    pub stats: IncrementalStats,
    pub log: InvalidationLog,
    pub config: IncrementalCompilationConfig,
}
impl IncrementalEngine {
    /// Create a fresh engine with default configuration.
    pub fn new() -> Self {
        Self {
            state: IncrementalBuildState::new(),
            scheduler: IncrementalScheduler::new(),
            queue: RebuildQueue::new(),
            stats: IncrementalStats::new(),
            log: InvalidationLog::new(),
            config: IncrementalCompilationConfig::default(),
        }
    }
    /// Create a fresh engine with a custom configuration.
    pub fn with_config(config: IncrementalCompilationConfig) -> Self {
        Self {
            state: IncrementalBuildState::new(),
            scheduler: IncrementalScheduler::new(),
            queue: RebuildQueue::new(),
            stats: IncrementalStats::new(),
            log: InvalidationLog::new(),
            config,
        }
    }
    /// Feed a source file change. Returns the hint for what to do.
    pub fn on_file_change(
        &mut self,
        module: &str,
        content: &[u8],
        iface: &[u8],
    ) -> BuildScheduleHint {
        let hint = self.scheduler.hint(module, content, iface);
        match &hint {
            BuildScheduleHint::Skip => {
                self.stats.record_hit(0);
            }
            BuildScheduleHint::Rebuild | BuildScheduleHint::RelinkOnly => {
                self.stats.record_miss();
                self.state.feed_change(module, content);
                self.queue.push(module);
                self.log.record(module, InvalidationCause::SourceChanged, 0);
            }
        }
        hint
    }
    /// Mark a module as built.
    pub fn on_build_complete(&mut self, module: &str, artifact: &str, src: &[u8], iface: &[u8]) {
        self.scheduler.record_built(module, src, iface);
        self.state.mark_built(module, artifact);
    }
    /// Check whether the build is complete.
    pub fn is_complete(&self) -> bool {
        self.queue.is_empty() && self.state.is_clean()
    }
    /// Summary of the current engine state.
    pub fn summary(&self) -> String {
        format!(
            "engine: queue={} dirty={} {}",
            self.queue.len(),
            self.state.rebuild_count(),
            self.stats.summary(),
        )
    }
}
/// Comprehensive metrics collected over a full incremental build run.
#[derive(Clone, Debug, Default)]
pub struct IncrementalBuildMetrics {
    /// Total modules in the build graph.
    pub total_modules: usize,
    /// Modules that were skipped (cache hit).
    pub skipped_modules: usize,
    /// Modules that were rebuilt.
    pub rebuilt_modules: usize,
    /// Modules that failed.
    pub failed_modules: usize,
    /// Total wall-clock time in milliseconds.
    pub wall_ms: u64,
    /// CPU time consumed in milliseconds (stub).
    pub cpu_ms: u64,
    /// Peak memory usage in bytes (stub).
    pub peak_memory_bytes: u64,
}
impl IncrementalBuildMetrics {
    /// Create zeroed metrics.
    pub fn new() -> Self {
        Self::default()
    }
    /// Cache hit rate (modules skipped / total).
    pub fn cache_hit_rate(&self) -> f64 {
        if self.total_modules == 0 {
            0.0
        } else {
            self.skipped_modules as f64 / self.total_modules as f64
        }
    }
    /// Whether the build succeeded (no failures).
    pub fn is_success(&self) -> bool {
        self.failed_modules == 0
    }
    /// Time saved by caching (estimate: assume avg rebuild time = wall_ms / rebuilt_modules).
    pub fn estimated_time_saved_ms(&self) -> u64 {
        if self.rebuilt_modules == 0 || self.skipped_modules == 0 {
            return 0;
        }
        let avg_rebuild_ms = self.wall_ms / self.rebuilt_modules as u64;
        avg_rebuild_ms * self.skipped_modules as u64
    }
    /// Human-readable summary.
    pub fn summary(&self) -> String {
        format!(
            "total={} rebuilt={} skipped={} failed={} hit_rate={:.1}% wall={}ms",
            self.total_modules,
            self.rebuilt_modules,
            self.skipped_modules,
            self.failed_modules,
            self.cache_hit_rate() * 100.0,
            self.wall_ms,
        )
    }
}
/// Processes `WatchEvent`s and converts them into dirty-set updates.
pub struct WatchEventProcessor {
    /// Map from file path to module name.
    file_to_module: HashMap<String, String>,
}
impl WatchEventProcessor {
    /// Create a processor with the given file→module mapping.
    pub fn new(file_to_module: HashMap<String, String>) -> Self {
        Self { file_to_module }
    }
    /// Process a watch event; return the affected module name (if known).
    pub fn process(&self, event: &WatchEvent) -> Option<&str> {
        self.file_to_module.get(event.path()).map(|s| s.as_str())
    }
    /// Register a new file→module mapping.
    pub fn register(&mut self, path: &str, module: &str) {
        self.file_to_module
            .insert(path.to_string(), module.to_string());
    }
    /// Unregister a file.
    pub fn unregister(&mut self, path: &str) {
        self.file_to_module.remove(path);
    }
    /// Number of registered file mappings.
    pub fn registered_count(&self) -> usize {
        self.file_to_module.len()
    }
}
