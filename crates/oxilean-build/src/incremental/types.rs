//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::manifest::Version;
use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};

/// Eviction policy for the incremental compilation cache.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IncrementalCacheEvictionPolicy {
    /// Evict least-recently-used entries first.
    Lru,
    /// Evict least-frequently-used entries first.
    Lfu,
    /// Evict oldest entries first (by build timestamp).
    Fifo,
}
/// The module dependency graph.
#[derive(Clone, Debug)]
pub struct ModuleGraph {
    /// All modules.
    modules: HashMap<String, FileEntry>,
    /// Forward edges: module -> dependencies.
    deps: HashMap<String, Vec<ModuleDep>>,
    /// Reverse edges: module -> dependents.
    reverse_deps: HashMap<String, Vec<String>>,
}
impl ModuleGraph {
    /// Create a new empty module graph.
    pub fn new() -> Self {
        Self {
            modules: HashMap::new(),
            deps: HashMap::new(),
            reverse_deps: HashMap::new(),
        }
    }
    /// Add a module to the graph.
    pub fn add_module(&mut self, entry: FileEntry) {
        let name = entry.module_path.clone();
        self.modules.insert(name, entry);
    }
    /// Add a dependency between two modules.
    pub fn add_dependency(&mut self, dep: ModuleDep) {
        let from = dep.from.clone();
        let to = dep.to.clone();
        self.reverse_deps.entry(to).or_default().push(from.clone());
        self.deps.entry(from).or_default().push(dep);
    }
    /// Get a module entry by path.
    pub fn get_module(&self, module_path: &str) -> Option<&FileEntry> {
        self.modules.get(module_path)
    }
    /// Get a mutable module entry by path.
    pub fn get_module_mut(&mut self, module_path: &str) -> Option<&mut FileEntry> {
        self.modules.get_mut(module_path)
    }
    /// Get direct dependencies of a module.
    pub fn dependencies_of(&self, module_path: &str) -> Vec<&ModuleDep> {
        self.deps
            .get(module_path)
            .map(|deps| deps.iter().collect())
            .unwrap_or_default()
    }
    /// Get direct dependents of a module.
    pub fn dependents_of(&self, module_path: &str) -> Vec<&str> {
        self.reverse_deps
            .get(module_path)
            .map(|deps| deps.iter().map(|s| s.as_str()).collect())
            .unwrap_or_default()
    }
    /// Compute all transitive dependents (modules affected by a change).
    pub fn transitive_dependents(&self, module_path: &str) -> HashSet<String> {
        let mut affected = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back(module_path.to_string());
        while let Some(current) = queue.pop_front() {
            if !affected.insert(current.clone()) {
                continue;
            }
            if let Some(dependents) = self.reverse_deps.get(&current) {
                for dep in dependents {
                    queue.push_back(dep.clone());
                }
            }
        }
        affected.remove(module_path);
        affected
    }
    /// Compute topological order of modules.
    pub fn topological_order(&self) -> Result<Vec<String>, IncrementalError> {
        let mut in_degree: HashMap<&str, usize> = HashMap::new();
        for key in self.modules.keys() {
            in_degree.insert(key.as_str(), 0);
        }
        for deps in self.deps.values() {
            for dep in deps {
                if let Some(deg) = in_degree.get_mut(dep.to.as_str()) {
                    *deg += 1;
                }
            }
        }
        let mut queue: VecDeque<String> = VecDeque::new();
        for (module, deg) in &in_degree {
            if *deg == 0 {
                queue.push_back(module.to_string());
            }
        }
        let mut result = Vec::new();
        while let Some(module) = queue.pop_front() {
            result.push(module.clone());
            if let Some(deps) = self.deps.get(&module) {
                for dep in deps {
                    if let Some(deg) = in_degree.get_mut(dep.to.as_str()) {
                        *deg -= 1;
                        if *deg == 0 {
                            queue.push_back(dep.to.clone());
                        }
                    }
                }
            }
        }
        if result.len() != self.modules.len() {
            return Err(IncrementalError::CyclicModuleDependency);
        }
        Ok(result)
    }
    /// Count the total number of modules.
    pub fn module_count(&self) -> usize {
        self.modules.len()
    }
    /// Count the total number of dependencies.
    pub fn dependency_count(&self) -> usize {
        self.deps.values().map(|d| d.len()).sum()
    }
    /// Get all module paths.
    pub fn module_paths(&self) -> Vec<&str> {
        self.modules.keys().map(|s| s.as_str()).collect()
    }
}
/// Extracts module dependencies from source file contents.
pub struct DependencyExtractor {
    /// Import patterns to look for.
    import_keywords: Vec<String>,
}
impl DependencyExtractor {
    /// Create a new dependency extractor.
    pub fn new() -> Self {
        Self {
            import_keywords: vec!["import".to_string(), "open".to_string()],
        }
    }
    /// Extract module dependencies from source text.
    pub fn extract(&self, source: &str) -> Vec<(String, ModuleDepKind)> {
        let mut deps = Vec::new();
        for line in source.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("--") {
                continue;
            }
            for keyword in &self.import_keywords {
                if let Some(rest) = trimmed.strip_prefix(keyword.as_str()) {
                    let rest = rest.trim();
                    if !rest.is_empty() {
                        let module_name = rest.split_whitespace().next().unwrap_or("").to_string();
                        if !module_name.is_empty() {
                            let kind = if keyword == "import" {
                                ModuleDepKind::Import
                            } else {
                                ModuleDepKind::Open
                            };
                            deps.push((module_name, kind));
                        }
                    }
                }
            }
        }
        deps
    }
    /// Add a custom import keyword.
    pub fn add_keyword(&mut self, keyword: &str) {
        self.import_keywords.push(keyword.to_string());
    }
}
/// A bounded rolling history of module compile records.
pub struct IncrementalCompileHistory {
    records: std::collections::VecDeque<ModuleCompileRecord>,
    max_size: usize,
}
impl IncrementalCompileHistory {
    /// Create a history with the given capacity.
    pub fn with_capacity(max_size: usize) -> Self {
        Self {
            records: std::collections::VecDeque::new(),
            max_size,
        }
    }
    /// Append a record, evicting oldest if at capacity.
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
    /// Cache hit rate.
    pub fn hit_rate(&self) -> f64 {
        if self.records.is_empty() {
            return 0.0;
        }
        let hits = self.records.iter().filter(|r| r.from_cache).count();
        hits as f64 / self.records.len() as f64
    }
    /// Average elapsed time in milliseconds.
    pub fn avg_elapsed_ms(&self) -> f64 {
        if self.records.is_empty() {
            return 0.0;
        }
        let total: u64 = self.records.iter().map(|r| r.elapsed_ms).sum();
        total as f64 / self.records.len() as f64
    }
    /// Success rate.
    pub fn success_rate(&self) -> f64 {
        if self.records.is_empty() {
            return 1.0;
        }
        let ok = self.records.iter().filter(|r| r.success).count();
        ok as f64 / self.records.len() as f64
    }
}
/// A record of a single module's compilation outcome.
#[derive(Clone, Debug)]
pub struct ModuleCompileRecord {
    /// Module name.
    pub module: String,
    /// Whether the result came from cache.
    pub from_cache: bool,
    /// Compilation time in milliseconds.
    pub elapsed_ms: u64,
    /// Number of warnings.
    pub warning_count: u32,
    /// Whether the compilation succeeded.
    pub success: bool,
}
impl ModuleCompileRecord {
    /// Create a successful record.
    pub fn success(module: &str, from_cache: bool, elapsed_ms: u64, warnings: u32) -> Self {
        Self {
            module: module.to_string(),
            from_cache,
            elapsed_ms,
            warning_count: warnings,
            success: true,
        }
    }
    /// Create a failure record.
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
/// Configuration for incremental compilation.
#[derive(Clone, Debug)]
pub struct IncrementalConfig {
    /// Whether to use file metadata (mtime/size) for quick change detection.
    pub use_metadata_check: bool,
    /// Whether to verify cached artifacts with fingerprints.
    pub verify_cached: bool,
    /// Compiler version string.
    pub compiler_version: String,
    /// Build flags.
    pub build_flags: Vec<String>,
    /// Cache directory.
    pub cache_dir: PathBuf,
    /// Maximum cache size in bytes.
    pub max_cache_bytes: u64,
}
/// Combines `IncrementalCompiler` + `ArtifactRegistry` + history for a full session.
pub struct IncrementalBuildOrchestrator {
    pub compiler: IncrementalCompiler,
    pub artifacts: ArtifactRegistry,
    pub history: IncrementalCompileHistory,
}
impl IncrementalBuildOrchestrator {
    /// Create an orchestrator.
    pub fn new(config: IncrementalConfig, compiler_version: Version) -> Self {
        Self {
            compiler: IncrementalCompiler::new(config, compiler_version),
            artifacts: ArtifactRegistry::new(),
            history: IncrementalCompileHistory::with_capacity(1000),
        }
    }
    /// Register a module source file.
    pub fn register(&mut self, module: &str, source: &Path) {
        self.compiler.register_module(module, source);
    }
    /// Feed updated source content for a module; returns whether the update succeeded.
    pub fn feed_content(&mut self, module: &str, content: &[u8]) -> bool {
        matches!(
            self.compiler.update_module_fingerprint(module, content),
            Ok(())
        )
    }
    /// Record a successful build.
    pub fn record_success(&mut self, module: &str, artifact: PathBuf, elapsed_ms: u64) {
        self.artifacts.register(module, artifact);
        self.history
            .push(ModuleCompileRecord::success(module, false, elapsed_ms, 0));
    }
    /// Record a cached (skipped) build.
    pub fn record_cached(&mut self, module: &str) {
        self.history
            .push(ModuleCompileRecord::success(module, true, 0, 0));
    }
    /// Whether all modules are up to date.
    pub fn is_up_to_date(&self) -> bool {
        self.compiler.compute_invalidation().invalidated.is_empty()
    }
}
/// The kind of build artifact.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ArtifactKind {
    /// Compiled object file (.olean equivalent).
    Object,
    /// Compiled library.
    Library,
    /// Compiled executable.
    Executable,
    /// Interface file (for incremental checking).
    Interface,
    /// Documentation file.
    Documentation,
}
/// The kind of dependency between modules.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ModuleDepKind {
    /// Import dependency (the module imports another).
    Import,
    /// Open dependency (the module opens a namespace).
    Open,
    /// Re-export dependency.
    ReExport,
    /// Type class instance dependency.
    Instance,
}
/// Information about a tracked source file.
#[derive(Clone, Debug)]
pub struct FileEntry {
    /// Absolute path to the file.
    pub path: PathBuf,
    /// Content fingerprint.
    pub fingerprint: Fingerprint,
    /// File size in bytes.
    pub size: u64,
    /// Last modification time.
    pub modified: Option<SystemTime>,
    /// Module path (e.g., "Mathlib.Topology.Basic").
    pub module_path: String,
}
impl FileEntry {
    /// Create a new file entry.
    pub fn new(path: &Path, module_path: &str) -> Self {
        Self {
            path: path.to_path_buf(),
            fingerprint: Fingerprint::zero(),
            size: 0,
            modified: None,
            module_path: module_path.to_string(),
        }
    }
    /// Update the fingerprint from file contents.
    pub fn update_fingerprint(&mut self, contents: &[u8]) {
        self.fingerprint = Fingerprint::from_bytes(contents);
        self.size = contents.len() as u64;
    }
    /// Check if the file might have changed based on metadata.
    pub fn might_have_changed(&self, current_modified: SystemTime, current_size: u64) -> bool {
        if self.size != current_size {
            return true;
        }
        match self.modified {
            Some(prev) => prev != current_modified,
            None => true,
        }
    }
}
/// A detected change in a source file.
#[derive(Clone, Debug)]
pub struct DetectedChange {
    /// Module path.
    pub module_path: String,
    /// File path.
    pub file_path: PathBuf,
    /// Kind of change.
    pub kind: ChangeKind,
}
/// Cache statistics.
#[derive(Clone, Debug, Default)]
pub struct CacheStats {
    /// Number of cache hits.
    pub hits: u64,
    /// Number of cache misses.
    pub misses: u64,
    /// Total bytes served from cache.
    pub bytes_served: u64,
    /// Total artifacts in cache.
    pub total_artifacts: u64,
    /// Total bytes used by cache.
    pub total_bytes: u64,
}
impl CacheStats {
    /// Compute the hit rate (0.0 to 1.0).
    pub fn hit_rate(&self) -> f64 {
        let total = self.hits + self.misses;
        if total == 0 {
            0.0
        } else {
            self.hits as f64 / total as f64
        }
    }
}
/// The kind of change detected.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ChangeKind {
    /// File was added.
    Added,
    /// File was modified.
    Modified,
    /// File was deleted.
    Deleted,
    /// File was renamed.
    Renamed {
        /// Old path.
        from: PathBuf,
    },
}
/// A build artifact stored in the cache.
#[derive(Clone, Debug)]
pub struct BuildArtifact {
    /// Artifact kind.
    pub kind: ArtifactKind,
    /// Path to the artifact file.
    pub path: PathBuf,
    /// Fingerprint of the source that produced this artifact.
    pub source_fingerprint: Fingerprint,
    /// Fingerprint of the artifact itself.
    pub artifact_fingerprint: Fingerprint,
    /// Fingerprints of all dependencies at build time.
    pub dep_fingerprints: BTreeMap<String, Fingerprint>,
    /// Compiler version used.
    pub compiler_version: String,
    /// Build timestamp.
    pub build_time: Option<SystemTime>,
    /// Build flags used.
    pub build_flags: Vec<String>,
}
impl BuildArtifact {
    /// Create a new artifact.
    pub fn new(kind: ArtifactKind, path: &Path, source_fp: Fingerprint) -> Self {
        Self {
            kind,
            path: path.to_path_buf(),
            source_fingerprint: source_fp,
            artifact_fingerprint: Fingerprint::zero(),
            dep_fingerprints: BTreeMap::new(),
            compiler_version: String::new(),
            build_time: None,
            build_flags: Vec::new(),
        }
    }
    /// Check if this artifact is still valid given the current source fingerprint
    /// and dependency fingerprints.
    pub fn is_valid(
        &self,
        current_source_fp: &Fingerprint,
        current_dep_fps: &BTreeMap<String, Fingerprint>,
        current_compiler_version: &str,
    ) -> bool {
        if &self.source_fingerprint != current_source_fp {
            return false;
        }
        if self.compiler_version != current_compiler_version {
            return false;
        }
        for (dep_name, dep_fp) in current_dep_fps {
            match self.dep_fingerprints.get(dep_name) {
                Some(cached_fp) if cached_fp == dep_fp => {}
                _ => return false,
            }
        }
        true
    }
    /// Compute a combined fingerprint including source and all deps.
    pub fn combined_fingerprint(&self) -> Fingerprint {
        let mut combined = self.source_fingerprint;
        for fp in self.dep_fingerprints.values() {
            combined = combined.combine(fp);
        }
        combined
    }
}
/// Tracks progress of an in-flight incremental build.
#[derive(Clone, Debug, Default)]
pub struct IncrementalBuildProgress {
    /// Total modules to build.
    pub total: usize,
    /// Modules completed so far.
    pub completed: usize,
    /// Modules that failed so far.
    pub failed: usize,
}
impl IncrementalBuildProgress {
    /// Create a progress tracker.
    pub fn new(total: usize) -> Self {
        Self {
            total,
            completed: 0,
            failed: 0,
        }
    }
    /// Record a completion.
    pub fn record_done(&mut self) {
        self.completed += 1;
    }
    /// Record a failure.
    pub fn record_failure(&mut self) {
        self.failed += 1;
    }
    /// Fraction complete (0.0–1.0).
    pub fn fraction(&self) -> f64 {
        if self.total == 0 {
            1.0
        } else {
            self.completed as f64 / self.total as f64
        }
    }
    /// Percentage complete.
    pub fn pct(&self) -> f64 {
        self.fraction() * 100.0
    }
    /// Whether the build is done.
    pub fn is_done(&self) -> bool {
        self.completed + self.failed >= self.total
    }
}
/// A build session tracking the progress of an incremental build.
pub struct BuildSession {
    /// Module build states.
    states: HashMap<String, ModuleBuildState>,
    /// Build order.
    build_order: Vec<String>,
    /// Current position in the build order.
    current_index: usize,
    /// Total modules to build.
    total: usize,
}
impl BuildSession {
    /// Create a new build session.
    pub fn new(build_order: Vec<String>) -> Self {
        let total = build_order.len();
        let mut states = HashMap::new();
        for module in &build_order {
            states.insert(module.clone(), ModuleBuildState::Pending);
        }
        Self {
            states,
            build_order,
            current_index: 0,
            total,
        }
    }
    /// Get the next module to build, if any.
    pub fn next_module(&self) -> Option<&str> {
        if self.current_index < self.build_order.len() {
            Some(&self.build_order[self.current_index])
        } else {
            None
        }
    }
    /// Mark a module as in-progress.
    pub fn start_module(&mut self, module: &str) {
        self.states
            .insert(module.to_string(), ModuleBuildState::InProgress);
    }
    /// Mark a module as successfully built.
    pub fn complete_module(&mut self, module: &str) {
        self.states
            .insert(module.to_string(), ModuleBuildState::Done);
        if self.current_index < self.build_order.len()
            && self.build_order[self.current_index] == module
        {
            self.current_index += 1;
        }
    }
    /// Mark a module as failed.
    pub fn fail_module(&mut self, module: &str, error: &str) {
        self.states.insert(
            module.to_string(),
            ModuleBuildState::Failed(error.to_string()),
        );
        if self.current_index < self.build_order.len()
            && self.build_order[self.current_index] == module
        {
            self.current_index += 1;
        }
    }
    /// Mark a module as served from cache.
    pub fn cache_hit(&mut self, module: &str) {
        self.states
            .insert(module.to_string(), ModuleBuildState::Cached);
        if self.current_index < self.build_order.len()
            && self.build_order[self.current_index] == module
        {
            self.current_index += 1;
        }
    }
    /// Get the state of a module.
    pub fn state(&self, module: &str) -> Option<&ModuleBuildState> {
        self.states.get(module)
    }
    /// Check if the session is complete.
    pub fn is_complete(&self) -> bool {
        self.current_index >= self.total
    }
    /// Get progress as a fraction (0.0 to 1.0).
    pub fn progress(&self) -> f64 {
        if self.total == 0 {
            1.0
        } else {
            self.current_index as f64 / self.total as f64
        }
    }
    /// Count modules by state.
    pub fn count_by_state(&self) -> HashMap<&'static str, usize> {
        let mut counts = HashMap::new();
        for state in self.states.values() {
            let key = match state {
                ModuleBuildState::Pending => "pending",
                ModuleBuildState::InProgress => "in_progress",
                ModuleBuildState::Done => "done",
                ModuleBuildState::Failed(_) => "failed",
                ModuleBuildState::Cached => "cached",
            };
            *counts.entry(key).or_insert(0) += 1;
        }
        counts
    }
    /// Check if any module failed.
    pub fn has_failures(&self) -> bool {
        self.states
            .values()
            .any(|s| matches!(s, ModuleBuildState::Failed(_)))
    }
    /// Get all failed modules and their errors.
    pub fn failures(&self) -> Vec<(&str, &str)> {
        self.states
            .iter()
            .filter_map(|(name, state)| match state {
                ModuleBuildState::Failed(err) => Some((name.as_str(), err.as_str())),
                _ => None,
            })
            .collect()
    }
}
/// A persistent fingerprint cache that can be saved/loaded.
#[derive(Clone, Debug)]
pub struct FingerprintCache {
    /// Fingerprints indexed by module path.
    fingerprints: HashMap<String, Fingerprint>,
    /// Cache version.
    version: u32,
}
impl FingerprintCache {
    /// Create a new empty cache.
    pub fn new() -> Self {
        Self {
            fingerprints: HashMap::new(),
            version: 1,
        }
    }
    /// Set a fingerprint.
    pub fn set(&mut self, module_path: &str, fingerprint: Fingerprint) {
        self.fingerprints
            .insert(module_path.to_string(), fingerprint);
    }
    /// Get a fingerprint.
    pub fn get(&self, module_path: &str) -> Option<Fingerprint> {
        self.fingerprints.get(module_path).copied()
    }
    /// Remove a fingerprint.
    pub fn remove(&mut self, module_path: &str) -> Option<Fingerprint> {
        self.fingerprints.remove(module_path)
    }
    /// Get the number of entries.
    pub fn count(&self) -> usize {
        self.fingerprints.len()
    }
    /// Clear all entries.
    pub fn clear(&mut self) {
        self.fingerprints.clear();
    }
    /// Serialize to a string (simple text format).
    pub fn serialize(&self) -> String {
        let mut out = format!("# fingerprint cache v{}\n", self.version);
        let mut sorted: Vec<_> = self.fingerprints.iter().collect();
        sorted.sort_by_key(|(k, _)| (*k).clone());
        for (module, fp) in sorted {
            out.push_str(&format!("{} {}\n", module, fp));
        }
        out
    }
    /// Deserialize from a string.
    pub fn deserialize(input: &str) -> Self {
        let mut cache = Self::new();
        for line in input.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let parts: Vec<&str> = line.splitn(2, ' ').collect();
            if parts.len() == 2 {
                let module = parts[0].to_string();
                let fp_str = parts[1];
                if fp_str.len() >= 32 {
                    let hi = u64::from_str_radix(&fp_str[..16], 16).unwrap_or(0);
                    let lo = u64::from_str_radix(&fp_str[16..32], 16).unwrap_or(0);
                    cache.set(&module, Fingerprint::new(hi, lo));
                }
            }
        }
        cache
    }
}
/// Reason for invalidation.
#[derive(Clone, Debug)]
pub enum InvalidationReason {
    /// The source file changed.
    SourceChanged {
        /// Module that changed.
        module: String,
        /// Old fingerprint.
        old_fp: Fingerprint,
        /// New fingerprint.
        new_fp: Fingerprint,
    },
    /// A dependency was invalidated.
    DependencyInvalidated {
        /// Module that was invalidated.
        module: String,
        /// The dependency that changed.
        changed_dep: String,
    },
    /// The compiler version changed.
    CompilerChanged {
        /// Old compiler version.
        old_version: String,
        /// New compiler version.
        new_version: String,
    },
    /// Build flags changed.
    FlagsChanged {
        /// Module affected.
        module: String,
    },
    /// Manual invalidation (user requested clean build).
    Manual,
}
/// A 128-bit fingerprint (hash) of a file or artifact.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Fingerprint {
    /// High 64 bits.
    pub hi: u64,
    /// Low 64 bits.
    pub lo: u64,
}
impl Fingerprint {
    /// Create a new fingerprint from two u64 halves.
    pub fn new(hi: u64, lo: u64) -> Self {
        Self { hi, lo }
    }
    /// Create a zero fingerprint.
    pub fn zero() -> Self {
        Self { hi: 0, lo: 0 }
    }
    /// Check if this is a zero fingerprint (uninitialized).
    pub fn is_zero(&self) -> bool {
        self.hi == 0 && self.lo == 0
    }
    /// Combine two fingerprints into one (for aggregating).
    pub fn combine(&self, other: &Self) -> Self {
        Self {
            hi: self
                .hi
                .wrapping_mul(6364136223846793005)
                .wrapping_add(other.hi),
            lo: self
                .lo
                .wrapping_mul(6364136223846793005)
                .wrapping_add(other.lo),
        }
    }
    /// Compute fingerprint from bytes using a simple hash function.
    pub fn from_bytes(data: &[u8]) -> Self {
        let mut hi: u64 = 0xcbf29ce484222325;
        let mut lo: u64 = 0x100000001b3;
        for (i, &byte) in data.iter().enumerate() {
            if i % 2 == 0 {
                hi ^= byte as u64;
                hi = hi.wrapping_mul(0x100000001b3);
            } else {
                lo ^= byte as u64;
                lo = lo.wrapping_mul(0xcbf29ce484222325);
            }
        }
        Self { hi, lo }
    }
    /// Compute fingerprint from a string.
    pub fn from_str_content(s: &str) -> Self {
        Self::from_bytes(s.as_bytes())
    }
}
/// The incremental compilation engine.
pub struct IncrementalCompiler {
    /// Module dependency graph.
    pub module_graph: ModuleGraph,
    /// Artifact cache.
    pub cache: ArtifactCache,
    /// Previous fingerprints (from last successful build).
    prev_fingerprints: HashMap<String, Fingerprint>,
    /// Configuration.
    config: IncrementalConfig,
    /// Package version (used in artifact paths).
    _package_version: Version,
}
impl IncrementalCompiler {
    /// Create a new incremental compiler.
    pub fn new(config: IncrementalConfig, package_version: Version) -> Self {
        let cache = ArtifactCache::new(&config.cache_dir);
        Self {
            module_graph: ModuleGraph::new(),
            cache,
            prev_fingerprints: HashMap::new(),
            config,
            _package_version: package_version,
        }
    }
    /// Register a module with its source file.
    pub fn register_module(&mut self, module_path: &str, file_path: &Path) {
        let entry = FileEntry::new(file_path, module_path);
        self.module_graph.add_module(entry);
    }
    /// Register a dependency between modules.
    pub fn register_dependency(&mut self, from: &str, to: &str, kind: ModuleDepKind) {
        self.module_graph.add_dependency(ModuleDep {
            from: from.to_string(),
            to: to.to_string(),
            kind,
        });
    }
    /// Update a module's fingerprint from its contents.
    pub fn update_module_fingerprint(
        &mut self,
        module_path: &str,
        contents: &[u8],
    ) -> Result<(), IncrementalError> {
        let entry = self
            .module_graph
            .get_module_mut(module_path)
            .ok_or_else(|| IncrementalError::ModuleNotFound(module_path.to_string()))?;
        entry.update_fingerprint(contents);
        Ok(())
    }
    /// Compute which modules need to be rebuilt.
    pub fn compute_invalidation(&self) -> InvalidationResult {
        let mut result = InvalidationResult::new();
        let mut directly_changed = HashSet::new();
        for (module_path, entry) in &self.module_graph.modules {
            result.total_checked += 1;
            match self.prev_fingerprints.get(module_path) {
                Some(prev_fp) if prev_fp == &entry.fingerprint => {}
                Some(prev_fp) => {
                    directly_changed.insert(module_path.clone());
                    result.invalidate(
                        module_path,
                        InvalidationReason::SourceChanged {
                            module: module_path.clone(),
                            old_fp: *prev_fp,
                            new_fp: entry.fingerprint,
                        },
                    );
                }
                None => {
                    directly_changed.insert(module_path.clone());
                    result.invalidate(
                        module_path,
                        InvalidationReason::SourceChanged {
                            module: module_path.clone(),
                            old_fp: Fingerprint::zero(),
                            new_fp: entry.fingerprint,
                        },
                    );
                }
            }
        }
        for changed in &directly_changed {
            let affected = self.module_graph.transitive_dependents(changed);
            for module in affected {
                if !result.invalidated.contains(&module) {
                    result.invalidate(
                        &module,
                        InvalidationReason::DependencyInvalidated {
                            module: module.clone(),
                            changed_dep: changed.clone(),
                        },
                    );
                }
            }
        }
        for module_path in self.module_graph.modules.keys() {
            if !result.invalidated.contains(module_path) {
                result.mark_valid(module_path);
            }
        }
        result
    }
    /// Commit current fingerprints as the "previous" state.
    pub fn commit_fingerprints(&mut self) {
        self.prev_fingerprints.clear();
        for (module_path, entry) in &self.module_graph.modules {
            self.prev_fingerprints
                .insert(module_path.clone(), entry.fingerprint);
        }
    }
    /// Get the build order (topological sort of invalidated modules).
    pub fn build_order(
        &self,
        invalidation: &InvalidationResult,
    ) -> Result<Vec<String>, IncrementalError> {
        let topo = self.module_graph.topological_order()?;
        let ordered: Vec<String> = topo
            .into_iter()
            .filter(|m| invalidation.invalidated.contains(m))
            .collect();
        Ok(ordered)
    }
    /// Store a build artifact for a module.
    pub fn store_artifact(&mut self, module_path: &str, artifact: BuildArtifact) {
        self.cache.store(module_path, artifact);
    }
    /// Look up a cached artifact for a module.
    pub fn lookup_artifact(
        &mut self,
        module_path: &str,
        kind: &ArtifactKind,
    ) -> Option<&BuildArtifact> {
        let source_fp = self
            .module_graph
            .get_module(module_path)
            .map(|e| e.fingerprint)
            .unwrap_or(Fingerprint::zero());
        let dep_fps = self.collect_dep_fingerprints(module_path);
        let compiler_version = self.config.compiler_version.clone();
        self.cache
            .lookup(module_path, kind, &source_fp, &dep_fps, &compiler_version)
    }
    fn collect_dep_fingerprints(&self, module_path: &str) -> BTreeMap<String, Fingerprint> {
        let mut fps = BTreeMap::new();
        if let Some(deps) = self.module_graph.deps.get(module_path) {
            for dep in deps {
                if let Some(entry) = self.module_graph.get_module(&dep.to) {
                    fps.insert(dep.to.clone(), entry.fingerprint);
                }
            }
        }
        fps
    }
    /// Get cache statistics.
    pub fn cache_stats(&self) -> &CacheStats {
        &self.cache.stats
    }
    /// Reset the incremental state (equivalent to a clean build).
    pub fn reset(&mut self) {
        self.prev_fingerprints.clear();
        self.cache.clear();
    }
}
/// Represents a file system event for incremental builds.
#[derive(Clone, Debug)]
pub struct FileEvent {
    /// The path that changed.
    pub path: PathBuf,
    /// The kind of event.
    pub kind: FileEventKind,
    /// Timestamp of the event.
    pub timestamp: Option<SystemTime>,
}
/// Processes file system events for incremental compilation.
pub struct FileEventProcessor {
    /// Watched directories.
    watched_dirs: Vec<PathBuf>,
    /// File extension filter.
    extensions: Vec<String>,
    /// Pending events that have not been processed.
    pending_events: Vec<FileEvent>,
    /// Debounce duration (events within this window are merged).
    debounce: Duration,
}
impl FileEventProcessor {
    /// Create a new event processor.
    pub fn new() -> Self {
        Self {
            watched_dirs: Vec::new(),
            extensions: vec!["lean".to_string(), "olean".to_string()],
            pending_events: Vec::new(),
            debounce: Duration::from_millis(200),
        }
    }
    /// Add a directory to watch.
    pub fn watch_dir(&mut self, dir: &Path) {
        self.watched_dirs.push(dir.to_path_buf());
    }
    /// Add a file extension filter.
    pub fn add_extension(&mut self, ext: &str) {
        self.extensions.push(ext.to_string());
    }
    /// Set the debounce duration.
    pub fn set_debounce(&mut self, duration: Duration) {
        self.debounce = duration;
    }
    /// Queue an event for processing.
    pub fn queue_event(&mut self, event: FileEvent) {
        if let Some(ext) = event.path.extension() {
            let ext_str = ext.to_string_lossy().to_string();
            if !self.extensions.contains(&ext_str) {
                return;
            }
        } else {
            return;
        }
        self.pending_events.push(event);
    }
    /// Get and clear pending events.
    pub fn drain_events(&mut self) -> Vec<FileEvent> {
        std::mem::take(&mut self.pending_events)
    }
    /// Get the number of pending events.
    pub fn pending_count(&self) -> usize {
        self.pending_events.len()
    }
    /// Get watched directories.
    pub fn watched_dirs(&self) -> &[PathBuf] {
        &self.watched_dirs
    }
    /// Get the debounce duration.
    pub fn debounce(&self) -> Duration {
        self.debounce
    }
}
/// Result of computing invalidated modules.
#[derive(Clone, Debug)]
pub struct InvalidationResult {
    /// Modules that need to be rebuilt.
    pub invalidated: HashSet<String>,
    /// Reasons for invalidation (per module).
    pub reasons: HashMap<String, Vec<InvalidationReason>>,
    /// Modules that are still valid.
    pub valid: HashSet<String>,
    /// Total number of modules checked.
    pub total_checked: usize,
}
impl InvalidationResult {
    /// Create a new invalidation result.
    pub fn new() -> Self {
        Self {
            invalidated: HashSet::new(),
            reasons: HashMap::new(),
            valid: HashSet::new(),
            total_checked: 0,
        }
    }
    /// Mark a module as invalidated with a reason.
    pub fn invalidate(&mut self, module: &str, reason: InvalidationReason) {
        self.invalidated.insert(module.to_string());
        self.reasons
            .entry(module.to_string())
            .or_default()
            .push(reason);
    }
    /// Mark a module as valid.
    pub fn mark_valid(&mut self, module: &str) {
        self.valid.insert(module.to_string());
    }
    /// Get the number of invalidated modules.
    pub fn invalidated_count(&self) -> usize {
        self.invalidated.len()
    }
    /// Get the number of valid modules.
    pub fn valid_count(&self) -> usize {
        self.valid.len()
    }
    /// Compute what fraction of modules need rebuilding.
    pub fn rebuild_fraction(&self) -> f64 {
        let total = self.invalidated.len() + self.valid.len();
        if total == 0 {
            0.0
        } else {
            self.invalidated.len() as f64 / total as f64
        }
    }
}
/// A computed build schedule for an incremental build.
pub struct IncrementalBuildSchedule {
    /// Ordered layers of modules to build (each layer can be built in parallel).
    pub layers: Vec<Vec<String>>,
    /// Modules to skip (up to date).
    pub skip_set: HashSet<String>,
}
impl IncrementalBuildSchedule {
    /// Create an empty schedule.
    pub fn new() -> Self {
        Self {
            layers: Vec::new(),
            skip_set: HashSet::new(),
        }
    }
    /// Add a build layer.
    pub fn add_layer(&mut self, modules: Vec<String>) {
        if !modules.is_empty() {
            self.layers.push(modules);
        }
    }
    /// Mark a module as skippable.
    pub fn mark_skip(&mut self, module: &str) {
        self.skip_set.insert(module.to_string());
    }
    /// Total modules to build.
    pub fn build_count(&self) -> usize {
        self.layers.iter().map(|l| l.len()).sum()
    }
    /// Number of layers.
    pub fn layer_count(&self) -> usize {
        self.layers.len()
    }
    /// Number of modules to skip.
    pub fn skip_count(&self) -> usize {
        self.skip_set.len()
    }
}
/// Tracks time spent in each build phase.
#[derive(Clone, Debug, Default)]
pub struct BuildPhaseTimer {
    pub parse_ms: u64,
    pub typecheck_ms: u64,
    pub codegen_ms: u64,
    pub link_ms: u64,
}
impl BuildPhaseTimer {
    /// Create zeroed timer.
    pub fn new() -> Self {
        Self::default()
    }
    /// Total time.
    pub fn total_ms(&self) -> u64 {
        self.parse_ms + self.typecheck_ms + self.codegen_ms + self.link_ms
    }
    /// Parse fraction.
    pub fn parse_fraction(&self) -> f64 {
        let total = self.total_ms();
        if total == 0 {
            0.0
        } else {
            self.parse_ms as f64 / total as f64
        }
    }
}
/// Statistics about an incremental build.
#[derive(Clone, Debug)]
pub struct IncrementalStats {
    /// Total modules in the project.
    pub total_modules: usize,
    /// Modules that were rebuilt.
    pub rebuilt_modules: usize,
    /// Modules served from cache.
    pub cached_modules: usize,
    /// Time saved by caching (estimated).
    pub estimated_time_saved: Duration,
    /// Total build time.
    pub total_time: Duration,
    /// Cache hit rate.
    pub cache_hit_rate: f64,
    /// Number of invalidation chains.
    pub invalidation_chains: usize,
    /// Longest invalidation chain length.
    pub max_chain_length: usize,
}
impl IncrementalStats {
    /// Create empty statistics.
    pub fn new() -> Self {
        Self {
            total_modules: 0,
            rebuilt_modules: 0,
            cached_modules: 0,
            estimated_time_saved: Duration::ZERO,
            total_time: Duration::ZERO,
            cache_hit_rate: 0.0,
            invalidation_chains: 0,
            max_chain_length: 0,
        }
    }
    /// Compute statistics from an invalidation result and build session.
    pub fn from_session(
        invalidation: &InvalidationResult,
        session: &BuildSession,
        total_time: Duration,
    ) -> Self {
        let total = invalidation.invalidated.len() + invalidation.valid.len();
        let rebuilt = invalidation.invalidated.len();
        let cached = invalidation.valid.len();
        let cache_hit_rate = if total > 0 {
            cached as f64 / total as f64
        } else {
            0.0
        };
        let estimated_saved = Duration::from_millis(cached as u64 * 100);
        let counts = session.count_by_state();
        let _cached_count = counts.get("cached").copied().unwrap_or(0);
        Self {
            total_modules: total,
            rebuilt_modules: rebuilt,
            cached_modules: cached,
            estimated_time_saved: estimated_saved,
            total_time,
            cache_hit_rate,
            invalidation_chains: 0,
            max_chain_length: 0,
        }
    }
    /// Format a summary.
    pub fn summary(&self) -> String {
        format!(
            "Incremental build: {}/{} modules rebuilt, {:.0}% cache hit rate, {:.2}s total (est. {:.2}s saved)",
            self.rebuilt_modules, self.total_modules, self.cache_hit_rate * 100.0, self
            .total_time.as_secs_f64(), self.estimated_time_saved.as_secs_f64(),
        )
    }
}
/// The state of a module in the current build session.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ModuleBuildState {
    /// Not yet started.
    Pending,
    /// Currently being built.
    InProgress,
    /// Successfully built.
    Done,
    /// Build failed.
    Failed(String),
    /// Skipped (cache hit).
    Cached,
}
/// The kind of file system event.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FileEventKind {
    /// File was created.
    Created,
    /// File was modified.
    Modified,
    /// File was deleted.
    Deleted,
    /// File was renamed.
    Renamed,
}
/// The build artifact cache.
#[derive(Clone, Debug)]
pub struct ArtifactCache {
    /// Cache directory.
    pub cache_dir: PathBuf,
    /// Cached artifacts indexed by module path.
    artifacts: HashMap<String, Vec<BuildArtifact>>,
    /// Cache statistics.
    pub stats: CacheStats,
    /// Maximum cache size in bytes.
    pub max_cache_bytes: u64,
}
impl ArtifactCache {
    /// Create a new artifact cache.
    pub fn new(cache_dir: &Path) -> Self {
        Self {
            cache_dir: cache_dir.to_path_buf(),
            artifacts: HashMap::new(),
            stats: CacheStats::default(),
            max_cache_bytes: 1024 * 1024 * 1024,
        }
    }
    /// Look up an artifact in the cache.
    pub fn lookup(
        &mut self,
        module_path: &str,
        kind: &ArtifactKind,
        source_fp: &Fingerprint,
        dep_fps: &BTreeMap<String, Fingerprint>,
        compiler_version: &str,
    ) -> Option<&BuildArtifact> {
        if let Some(artifacts) = self.artifacts.get(module_path) {
            for artifact in artifacts {
                if &artifact.kind == kind && artifact.is_valid(source_fp, dep_fps, compiler_version)
                {
                    self.stats.hits += 1;
                    return Some(artifact);
                }
            }
        }
        self.stats.misses += 1;
        None
    }
    /// Store an artifact in the cache.
    pub fn store(&mut self, module_path: &str, artifact: BuildArtifact) {
        self.stats.total_artifacts += 1;
        self.artifacts
            .entry(module_path.to_string())
            .or_default()
            .push(artifact);
    }
    /// Evict old artifacts that exceed the cache size.
    pub fn evict_if_needed(&mut self) {
        let mut total_size: u64 = 0;
        for artifacts in self.artifacts.values() {
            for _artifact in artifacts {
                total_size += 1024;
            }
        }
        if total_size > self.max_cache_bytes {
            let mut all_modules: Vec<String> = self.artifacts.keys().cloned().collect();
            all_modules.sort();
            while total_size > self.max_cache_bytes / 2 {
                if let Some(module) = all_modules.pop() {
                    if let Some(artifacts) = self.artifacts.remove(&module) {
                        total_size = total_size.saturating_sub(artifacts.len() as u64 * 1024);
                    }
                } else {
                    break;
                }
            }
        }
        self.stats.total_bytes = total_size;
    }
    /// Clear the entire cache.
    pub fn clear(&mut self) {
        self.artifacts.clear();
        self.stats.total_artifacts = 0;
        self.stats.total_bytes = 0;
    }
    /// Get the number of cached modules.
    pub fn module_count(&self) -> usize {
        self.artifacts.len()
    }
    /// Get the total number of cached artifacts.
    pub fn artifact_count(&self) -> usize {
        self.artifacts.values().map(|a| a.len()).sum()
    }
}
/// Maps module names to their produced artifact paths.
#[derive(Clone, Debug, Default)]
pub struct ArtifactRegistry {
    map: HashMap<String, Vec<PathBuf>>,
}
impl ArtifactRegistry {
    /// Create an empty registry.
    pub fn new() -> Self {
        Self::default()
    }
    /// Register an artifact for `module`.
    pub fn register(&mut self, module: &str, path: PathBuf) {
        self.map.entry(module.to_string()).or_default().push(path);
    }
    /// Get all artifacts for `module`.
    pub fn get(&self, module: &str) -> &[PathBuf] {
        self.map.get(module).map(|v| v.as_slice()).unwrap_or(&[])
    }
    /// Remove all artifacts for `module`.
    pub fn remove(&mut self, module: &str) {
        self.map.remove(module);
    }
    /// Total number of artifact paths registered.
    pub fn total_artifacts(&self) -> usize {
        self.map.values().map(|v| v.len()).sum()
    }
    /// Number of modules with registered artifacts.
    pub fn module_count(&self) -> usize {
        self.map.len()
    }
}
/// Describes what changed between two consecutive builds.
#[derive(Clone, Debug, Default)]
pub struct IncrementalDelta {
    /// Modules whose source changed.
    pub source_changed: Vec<String>,
    /// Modules newly added.
    pub added: Vec<String>,
    /// Modules removed.
    pub removed: Vec<String>,
    /// Modules that were transitively invalidated.
    pub transitive: Vec<String>,
}
impl IncrementalDelta {
    /// Create an empty delta.
    pub fn new() -> Self {
        Self::default()
    }
    /// Total number of affected modules.
    pub fn total_affected(&self) -> usize {
        self.source_changed.len() + self.added.len() + self.removed.len() + self.transitive.len()
    }
    /// Whether anything changed.
    pub fn has_changes(&self) -> bool {
        self.total_affected() > 0
    }
}
/// A dependency between two modules.
#[derive(Clone, Debug)]
pub struct ModuleDep {
    /// Source module path.
    pub from: String,
    /// Target module path.
    pub to: String,
    /// Kind of dependency.
    pub kind: ModuleDepKind,
}
/// Error type for incremental compilation.
#[derive(Clone, Debug)]
pub enum IncrementalError {
    /// A cyclic module dependency was detected.
    CyclicModuleDependency,
    /// A module was not found.
    ModuleNotFound(String),
    /// An artifact was corrupted.
    CorruptedArtifact(String),
    /// IO error.
    IoError(String),
    /// Cache error.
    CacheError(String),
}
/// Classification of a dependency edge in the incremental graph.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DependencyEdgeKind {
    /// Full compilation dependency.
    Full,
    /// Type-only dependency (interface).
    TypeOnly,
    /// Weak dependency (only for documentation).
    Weak,
}
