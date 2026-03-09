//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};

/// Information about parallelization opportunities.
#[derive(Clone, Debug)]
pub struct BuildStages {
    /// List of independent modules that can be built in parallel.
    pub stages: Vec<Vec<String>>,
}
impl BuildStages {
    /// Create build stages from a module graph.
    /// Each stage contains modules with no interdependencies.
    pub fn from_graph(graph: &ModuleGraph) -> Result<Self, ProjectError> {
        let mut remaining: HashSet<String> = graph.nodes.iter().cloned().collect();
        let mut stages = Vec::new();
        while !remaining.is_empty() {
            let mut stage = Vec::new();
            for node in &remaining {
                let deps = graph.dependencies_of(node);
                if deps.iter().all(|d| !remaining.contains(d)) {
                    stage.push(node.clone());
                }
            }
            if stage.is_empty() {
                let cycle_node = remaining
                    .iter()
                    .next()
                    .expect("remaining is non-empty: stage is empty means cycle")
                    .clone();
                return Err(ProjectError::CyclicDependency(vec![cycle_node]));
            }
            stage.sort();
            for node in &stage {
                remaining.remove(node);
            }
            stages.push(stage);
        }
        Ok(Self { stages })
    }
    /// Maximum parallelism achievable at any stage.
    pub fn max_parallelism(&self) -> usize {
        self.stages.iter().map(|s| s.len()).max().unwrap_or(1)
    }
}
/// A single entry in the lockfile.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LockEntry {
    /// Package name.
    pub name: String,
    /// Exact version.
    pub version: String,
    /// Source description.
    pub source: String,
    /// Content hash for integrity checking.
    pub checksum: Option<String>,
}
/// Errors in the project system.
#[derive(Clone, Debug)]
pub enum ProjectError {
    /// Failed to parse the config file.
    ParseError {
        /// Line number (1-based).
        line: usize,
        /// Error description.
        message: String,
    },
    /// Configuration is invalid.
    InvalidConfig(String),
    /// Project file not found.
    NotFound(String),
    /// IO error.
    IoError(String),
    /// Cyclic module dependencies.
    CyclicDependency(Vec<String>),
    /// Dependency not found in registry.
    DependencyNotFound(String),
    /// Version not found.
    VersionNotFound {
        /// Dependency name.
        name: String,
        /// Requested version.
        version: String,
    },
    /// Build failure.
    BuildFailed(String),
}
/// Information about a single module.
#[derive(Clone, Debug)]
pub struct ModuleInfo {
    /// Fully-qualified module name (e.g., "Mathlib.Algebra.Group").
    pub name: String,
    /// Filesystem path to the source file.
    pub path: PathBuf,
    /// Names of modules this module depends on.
    pub dependencies: Vec<String>,
    /// Whether this module needs rebuilding.
    pub is_stale: bool,
    /// Last modification time.
    pub last_modified: Option<SystemTime>,
}
/// Options for module discovery.
#[derive(Clone, Debug)]
pub struct DiscoveryOptions {
    /// Extensions to scan for (e.g., "lean", "ox").
    pub extensions: Vec<String>,
    /// Directories to exclude.
    pub exclude_dirs: HashSet<String>,
    /// Whether to auto-detect namespaces from directory structure.
    pub auto_namespace: bool,
}
/// Version constraint operator.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum VersionConstraint {
    /// Any version (*).
    Any,
    /// Exact version.
    Exact(String),
    /// Minimum version (e.g., >=1.0.0).
    AtLeast(String),
    /// Compatible version (e.g., ~1.2.0 means >=1.2.0 and <1.3.0).
    Compatible(String),
}
impl VersionConstraint {
    /// Parse a version constraint string.
    pub fn parse(s: &str) -> Self {
        let s = s.trim();
        if s == "*" || s == "latest" {
            return VersionConstraint::Any;
        }
        if let Some(rest) = s.strip_prefix(">=") {
            return VersionConstraint::AtLeast(rest.trim().to_string());
        }
        if let Some(rest) = s.strip_prefix('~') {
            return VersionConstraint::Compatible(rest.trim().to_string());
        }
        VersionConstraint::Exact(s.to_string())
    }
    /// Check if a version satisfies this constraint.
    pub fn matches(&self, version: &str) -> bool {
        match self {
            VersionConstraint::Any => true,
            VersionConstraint::Exact(v) => v == version,
            VersionConstraint::AtLeast(min) => compare_versions(version, min) >= 0,
            VersionConstraint::Compatible(base) => {
                if !version.starts_with(&base[0..base.rfind('.').unwrap_or(0)]) {
                    return false;
                }
                compare_versions(version, base) >= 0
            }
        }
    }
}
/// Source of a dependency.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DependencySource {
    /// Local path dependency.
    Path(PathBuf),
    /// Git repository dependency.
    Git {
        /// Repository URL.
        url: String,
        /// Branch, tag, or commit.
        rev: Option<String>,
    },
    /// Registry-hosted dependency.
    Registry {
        /// Registry URL.
        registry: String,
    },
}
/// Project configuration (analogous to lakefile.lean / oxilean.toml).
#[derive(Clone, Debug)]
pub struct ProjectConfig {
    /// Project name.
    pub name: String,
    /// Project version (semver).
    pub version: String,
    /// List of authors.
    pub authors: Vec<String>,
    /// Project description.
    pub description: String,
    /// External dependencies.
    pub dependencies: Vec<Dependency>,
    /// Source directories to search for modules.
    pub source_dirs: Vec<PathBuf>,
    /// Directory for build output.
    pub output_dir: PathBuf,
    /// Compatible OxiLean version (semver constraint).
    pub lean_version: String,
    /// Extra arguments passed to the checker.
    pub extra_args: Vec<String>,
}
impl ProjectConfig {
    /// Create a default configuration for a project with the given name.
    pub fn default_for(name: &str) -> Self {
        Self {
            name: name.to_string(),
            version: "0.1.1".to_string(),
            authors: Vec::new(),
            description: String::new(),
            dependencies: Vec::new(),
            source_dirs: vec![PathBuf::from("src")],
            output_dir: PathBuf::from("build"),
            lean_version: "0.1.1".to_string(),
            extra_args: Vec::new(),
        }
    }
    /// Parse a `ProjectConfig` from a TOML-like string (custom parser).
    pub fn load(content: &str) -> Result<Self, ProjectError> {
        let mut config = ProjectConfig::default_for("unnamed");
        let mut current_section = String::new();
        let mut current_dep: Option<PartialDependency> = None;
        for (line_no, raw_line) in content.lines().enumerate() {
            let line = raw_line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            if line.starts_with('[') {
                if let Some(dep) = current_dep.take() {
                    config.dependencies.push(dep.finish(line_no)?);
                }
                let section = line.trim_matches(|c| c == '[' || c == ']').trim();
                current_section = section.to_string();
                if current_section == "dependencies" {
                    current_dep = Some(PartialDependency::default());
                }
                continue;
            }
            let (key, value) = parse_kv(line).ok_or_else(|| ProjectError::ParseError {
                line: line_no + 1,
                message: format!("expected key = value, got: {}", line),
            })?;
            match current_section.as_str() {
                "package" | "" => match key.as_str() {
                    "name" => config.name = value,
                    "version" => config.version = value,
                    "description" => config.description = value,
                    "lean_version" | "oxilean_version" => config.lean_version = value,
                    "authors" => {
                        config.authors = parse_string_array(&value);
                    }
                    "source_dirs" => {
                        config.source_dirs = parse_string_array(&value)
                            .into_iter()
                            .map(PathBuf::from)
                            .collect();
                    }
                    "output_dir" => config.output_dir = PathBuf::from(value),
                    "extra_args" => {
                        config.extra_args = parse_string_array(&value);
                    }
                    _ => {
                        return Err(ProjectError::ParseError {
                            line: line_no + 1,
                            message: format!("unknown key '{}' in [package]", key),
                        });
                    }
                },
                "dependencies" => {
                    let dep = current_dep.get_or_insert_with(PartialDependency::default);
                    match key.as_str() {
                        "name" => dep.name = Some(value),
                        "version" => dep.version = Some(value),
                        "path" => dep.source_path = Some(PathBuf::from(value)),
                        "git" => dep.git_url = Some(value),
                        "rev" => dep.git_rev = Some(value),
                        "registry" => dep.registry = Some(value),
                        _ => {
                            return Err(ProjectError::ParseError {
                                line: line_no + 1,
                                message: format!("unknown key '{}' in [dependencies]", key),
                            });
                        }
                    }
                }
                other => {
                    return Err(ProjectError::ParseError {
                        line: line_no + 1,
                        message: format!("unknown section [{}]", other),
                    });
                }
            }
        }
        if let Some(dep) = current_dep.take() {
            if dep.name.is_some() {
                let line_no = content.lines().count();
                config.dependencies.push(dep.finish(line_no)?);
            }
        }
        Ok(config)
    }
    /// Serialize to a TOML-like string.
    pub fn save(&self) -> String {
        let mut out = String::new();
        out.push_str("[package]\n");
        push_kv(&mut out, "name", &self.name);
        push_kv(&mut out, "version", &self.version);
        if !self.description.is_empty() {
            push_kv(&mut out, "description", &self.description);
        }
        if !self.authors.is_empty() {
            out.push_str(&format!(
                "authors = [{}]\n",
                self.authors
                    .iter()
                    .map(|a| format!("\"{}\"", a))
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }
        if self.source_dirs != vec![PathBuf::from("src")] {
            out.push_str(&format!(
                "source_dirs = [{}]\n",
                self.source_dirs
                    .iter()
                    .map(|p| format!("\"{}\"", p.display()))
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }
        if self.output_dir != Path::new("build") {
            push_kv(
                &mut out,
                "output_dir",
                &self.output_dir.display().to_string(),
            );
        }
        push_kv(&mut out, "lean_version", &self.lean_version);
        if !self.extra_args.is_empty() {
            out.push_str(&format!(
                "extra_args = [{}]\n",
                self.extra_args
                    .iter()
                    .map(|a| format!("\"{}\"", a))
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }
        for dep in &self.dependencies {
            out.push('\n');
            out.push_str("[[dependencies]]\n");
            push_kv(&mut out, "name", &dep.name);
            push_kv(&mut out, "version", &dep.version);
            match &dep.source {
                DependencySource::Path(p) => {
                    push_kv(&mut out, "path", &p.display().to_string());
                }
                DependencySource::Git { url, rev } => {
                    push_kv(&mut out, "git", url);
                    if let Some(r) = rev {
                        push_kv(&mut out, "rev", r);
                    }
                }
                DependencySource::Registry { registry } => {
                    push_kv(&mut out, "registry", registry);
                }
            }
        }
        out
    }
    /// Validate that the configuration is consistent.
    pub fn validate(&self) -> Result<(), ProjectError> {
        if self.name.is_empty() {
            return Err(ProjectError::InvalidConfig("project name is empty".into()));
        }
        if !is_valid_semver(&self.version) {
            return Err(ProjectError::InvalidConfig(format!(
                "invalid version: {}",
                self.version
            )));
        }
        if self.source_dirs.is_empty() {
            return Err(ProjectError::InvalidConfig(
                "at least one source directory is required".into(),
            ));
        }
        let mut dep_names = HashSet::new();
        for dep in &self.dependencies {
            if !dep_names.insert(&dep.name) {
                return Err(ProjectError::InvalidConfig(format!(
                    "duplicate dependency: {}",
                    dep.name
                )));
            }
        }
        Ok(())
    }
}
/// Helper for partially constructed dependency during parsing.
#[derive(Default)]
struct PartialDependency {
    name: Option<String>,
    version: Option<String>,
    source_path: Option<PathBuf>,
    git_url: Option<String>,
    git_rev: Option<String>,
    registry: Option<String>,
}
impl PartialDependency {
    fn finish(self, line: usize) -> Result<Dependency, ProjectError> {
        let name = self.name.ok_or_else(|| ProjectError::ParseError {
            line,
            message: "dependency missing 'name'".into(),
        })?;
        let version = self.version.unwrap_or_else(|| "*".to_string());
        let source = if let Some(p) = self.source_path {
            DependencySource::Path(p)
        } else if let Some(url) = self.git_url {
            DependencySource::Git {
                url,
                rev: self.git_rev,
            }
        } else if let Some(registry) = self.registry {
            DependencySource::Registry { registry }
        } else {
            DependencySource::Registry {
                registry: "https://packages.oxilean.dev".to_string(),
            }
        };
        Ok(Dependency {
            name,
            version,
            source,
        })
    }
}
/// Represents a discovered project.
#[derive(Clone, Debug)]
pub struct Project {
    /// Root directory of the project.
    pub root: PathBuf,
    /// Parsed project configuration.
    pub config: ProjectConfig,
    /// Discovered modules.
    pub modules: Vec<ModuleInfo>,
    /// Module dependency graph.
    pub build_graph: ModuleGraph,
}
impl Project {
    /// Walk up directories from `path` looking for `oxilean.toml`.
    pub fn discover(path: &Path) -> Result<Self, ProjectError> {
        let config_path = find_project_file(path)?;
        let root = config_path.parent().unwrap_or(path).to_path_buf();
        let content = std::fs::read_to_string(&config_path).map_err(|e| {
            ProjectError::IoError(format!("failed to read {}: {}", config_path.display(), e))
        })?;
        let config = ProjectConfig::load(&content)?;
        Ok(Self {
            root,
            config,
            modules: Vec::new(),
            build_graph: ModuleGraph::new(),
        })
    }
    /// Create a project from an in-memory config and root path (useful for tests).
    pub fn from_config(root: PathBuf, config: ProjectConfig) -> Self {
        Self {
            root,
            config,
            modules: Vec::new(),
            build_graph: ModuleGraph::new(),
        }
    }
    /// Find all `.lean` / `.ox` module files under the source directories.
    pub fn find_modules(&mut self) -> Result<(), ProjectError> {
        self.modules.clear();
        for src_dir in &self.config.source_dirs {
            let abs_dir = self.root.join(src_dir);
            collect_modules(&abs_dir, &abs_dir, &mut self.modules)?;
        }
        Ok(())
    }
    /// Build the module dependency graph from discovered modules.
    pub fn build_dependency_graph(&mut self) {
        self.build_graph = build_module_graph(&self.modules);
    }
    /// Convenience: discover modules and build graph.
    pub fn initialize(&mut self) -> Result<(), ProjectError> {
        self.find_modules()?;
        self.build_dependency_graph();
        Ok(())
    }
}
/// A plan describing what to build and in what order.
#[derive(Clone, Debug)]
pub struct BuildPlan {
    /// Modules to build, in topological order.
    pub modules_to_build: Vec<String>,
    /// Full topological order (including cached).
    pub order: Vec<String>,
    /// Max parallelism hint.
    pub parallelism: usize,
}
/// A project dependency declaration.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Dependency {
    /// Name of the dependency.
    pub name: String,
    /// Required version (semver string).
    pub version: String,
    /// Where to fetch the dependency.
    pub source: DependencySource,
}
impl Dependency {
    /// Create a new path dependency.
    pub fn path(name: &str, version: &str, path: PathBuf) -> Self {
        Self {
            name: name.to_string(),
            version: version.to_string(),
            source: DependencySource::Path(path),
        }
    }
    /// Create a new git dependency.
    pub fn git(name: &str, version: &str, url: &str, rev: Option<&str>) -> Self {
        Self {
            name: name.to_string(),
            version: version.to_string(),
            source: DependencySource::Git {
                url: url.to_string(),
                rev: rev.map(String::from),
            },
        }
    }
    /// Create a new registry dependency.
    pub fn registry(name: &str, version: &str, registry: &str) -> Self {
        Self {
            name: name.to_string(),
            version: version.to_string(),
            source: DependencySource::Registry {
                registry: registry.to_string(),
            },
        }
    }
}
/// Resolved dependency with concrete version and location.
#[derive(Clone, Debug)]
pub struct ResolvedDep {
    /// Dependency name.
    pub name: String,
    /// Resolved version.
    pub version: String,
    /// Local path to the dependency source.
    pub local_path: PathBuf,
}
/// Result of building a single module.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BuildResult {
    /// Module compiled successfully.
    Success,
    /// Module failed to compile.
    Failure(String),
    /// Module was up-to-date and used the cached output.
    Cached,
}
/// Report produced after a build run.
#[derive(Clone, Debug)]
pub struct BuildReport {
    /// Total number of modules considered.
    pub total: usize,
    /// Number of modules that compiled successfully.
    pub succeeded: usize,
    /// Number of modules that failed.
    pub failed: usize,
    /// Number of modules that were cached.
    pub cached: usize,
    /// Wall-clock elapsed time.
    pub elapsed: Duration,
    /// Individual results per module.
    pub results: HashMap<String, BuildResult>,
}
impl BuildReport {
    /// Create an empty report.
    pub fn new() -> Self {
        Self {
            total: 0,
            succeeded: 0,
            failed: 0,
            cached: 0,
            elapsed: Duration::ZERO,
            results: HashMap::new(),
        }
    }
    /// Whether the build was fully successful.
    pub fn is_success(&self) -> bool {
        self.failed == 0
    }
    /// Summary string.
    pub fn summary(&self) -> String {
        format!(
            "Build: {} total, {} succeeded, {} cached, {} failed ({:.2}s)",
            self.total,
            self.succeeded,
            self.cached,
            self.failed,
            self.elapsed.as_secs_f64()
        )
    }
}
/// A lockfile recording exact resolved versions.
#[derive(Clone, Debug, Default)]
pub struct LockFile {
    /// Resolved dependencies.
    pub entries: Vec<LockEntry>,
}
impl LockFile {
    /// Create an empty lockfile.
    pub fn new() -> Self {
        Self::default()
    }
    /// Create a lockfile from resolved dependencies.
    pub fn from_resolved(deps: &[ResolvedDep]) -> Self {
        let entries = deps
            .iter()
            .map(|d| LockEntry {
                name: d.name.clone(),
                version: d.version.clone(),
                source: d.local_path.display().to_string(),
                checksum: None,
            })
            .collect();
        Self { entries }
    }
    /// Serialize to a TOML-like string.
    pub fn serialize(&self) -> String {
        let mut out = String::from("# OxiLean lock file — do not edit manually\n\n");
        for entry in &self.entries {
            out.push_str("[[lock]]\n");
            push_kv(&mut out, "name", &entry.name);
            push_kv(&mut out, "version", &entry.version);
            push_kv(&mut out, "source", &entry.source);
            if let Some(cs) = &entry.checksum {
                push_kv(&mut out, "checksum", cs);
            }
            out.push('\n');
        }
        out
    }
    /// Deserialize from a TOML-like string.
    pub fn deserialize(content: &str) -> Result<Self, ProjectError> {
        let mut entries = Vec::new();
        let mut current: Option<LockEntry> = None;
        for (line_no, raw_line) in content.lines().enumerate() {
            let line = raw_line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            if line == "[[lock]]" {
                if let Some(entry) = current.take() {
                    entries.push(entry);
                }
                current = Some(LockEntry {
                    name: String::new(),
                    version: String::new(),
                    source: String::new(),
                    checksum: None,
                });
                continue;
            }
            if let Some((key, value)) = parse_kv(line) {
                let entry = current.as_mut().ok_or_else(|| ProjectError::ParseError {
                    line: line_no + 1,
                    message: "key=value outside [[lock]] section".into(),
                })?;
                match key.as_str() {
                    "name" => entry.name = value,
                    "version" => entry.version = value,
                    "source" => entry.source = value,
                    "checksum" => entry.checksum = Some(value),
                    _ => {}
                }
            }
        }
        if let Some(entry) = current {
            entries.push(entry);
        }
        Ok(Self { entries })
    }
    /// Check if the lock file already has a given package at a specific version.
    pub fn is_locked(&self, name: &str, version: &str) -> bool {
        self.entries
            .iter()
            .any(|e| e.name == name && e.version == version)
    }
}
/// Registry of available packages.
#[derive(Clone, Debug, Default)]
pub struct PackageRegistry {
    /// Known packages, keyed by name.
    pub packages: HashMap<String, PackageEntry>,
}
impl PackageRegistry {
    /// Create an empty registry.
    pub fn new() -> Self {
        Self::default()
    }
    /// Register a package.
    pub fn add(&mut self, entry: PackageEntry) {
        self.packages.insert(entry.name.clone(), entry);
    }
    /// Look up a package.
    pub fn find(&self, name: &str) -> Option<&PackageEntry> {
        self.packages.get(name)
    }
    /// Check if a version satisfies a constraint (simplified: exact or `*`).
    pub fn version_matches(constraint: &str, version: &str) -> bool {
        if constraint == "*" {
            return true;
        }
        constraint == version
    }
}
/// Configuration for creating a new project.
#[derive(Clone, Debug)]
pub struct ProjectInitOptions {
    /// Project name.
    pub name: String,
    /// Project version.
    pub version: String,
    /// Authors list.
    pub authors: Vec<String>,
    /// Project description.
    pub description: Option<String>,
    /// Create a git repository.
    pub with_git: bool,
    /// Create example files.
    pub with_examples: bool,
}
impl ProjectInitOptions {
    /// Create options with defaults.
    pub fn new(name: String) -> Self {
        Self {
            name,
            version: "0.1.1".to_string(),
            authors: Vec::new(),
            description: None,
            with_git: true,
            with_examples: false,
        }
    }
}
/// Advanced dependency solver.
#[derive(Clone, Debug)]
pub struct DependencySolver {
    /// All known packages.
    pub registry: PackageRegistry,
    /// Resolved dependencies cache.
    pub resolved: HashMap<String, ResolvedDep>,
    /// Conflict tracking.
    pub conflicts: Vec<(String, String, String)>,
}
impl DependencySolver {
    /// Create a new solver with a registry.
    pub fn new(registry: PackageRegistry) -> Self {
        Self {
            registry,
            resolved: HashMap::new(),
            conflicts: Vec::new(),
        }
    }
    /// Resolve dependencies recursively, returning the full closure.
    pub fn resolve_all(&mut self, deps: &[Dependency]) -> Result<Vec<ResolvedDep>, ProjectError> {
        let mut work_queue = Vec::new();
        for dep in deps {
            work_queue.push(dep.clone());
        }
        while let Some(dep) = work_queue.pop() {
            if self.resolved.contains_key(&dep.name) {
                continue;
            }
            let resolved_dep = self.resolve_one(&dep)?;
            if let DependencySource::Registry { .. } = &dep.source {
                if let Some(pkg) = self.registry.find(&dep.name) {
                    for transitive in &pkg.deps {
                        if !self.resolved.contains_key(&transitive.name) {
                            work_queue.push(transitive.clone());
                        }
                    }
                }
            }
            self.resolved.insert(dep.name.clone(), resolved_dep);
        }
        Ok(self.resolved.values().cloned().collect())
    }
    fn resolve_one(&self, dep: &Dependency) -> Result<ResolvedDep, ProjectError> {
        match &dep.source {
            DependencySource::Path(p) => Ok(ResolvedDep {
                name: dep.name.clone(),
                version: dep.version.clone(),
                local_path: p.clone(),
            }),
            DependencySource::Git { .. } => Ok(ResolvedDep {
                name: dep.name.clone(),
                version: dep.version.clone(),
                local_path: PathBuf::from(format!(".oxilean/deps/{}", dep.name)),
            }),
            DependencySource::Registry { .. } => {
                let pkg = self
                    .registry
                    .find(&dep.name)
                    .ok_or_else(|| ProjectError::DependencyNotFound(dep.name.clone()))?;
                let constraint = VersionConstraint::parse(&dep.version);
                let matched = pkg
                    .versions
                    .iter()
                    .find(|v| constraint.matches(v))
                    .ok_or_else(|| ProjectError::VersionNotFound {
                        name: dep.name.clone(),
                        version: dep.version.clone(),
                    })?;
                Ok(ResolvedDep {
                    name: dep.name.clone(),
                    version: matched.clone(),
                    local_path: PathBuf::from(format!(
                        ".oxilean/registry/{}/{}",
                        dep.name, matched
                    )),
                })
            }
        }
    }
}
/// Information about a built artifact.
#[derive(Clone, Debug)]
pub struct BuildArtifact {
    /// Module name.
    pub module_name: String,
    /// Artifact path (.olean file, etc.)
    pub artifact_path: PathBuf,
    /// Hash of source at build time.
    pub source_hash: String,
    /// Build timestamp.
    pub built_at: SystemTime,
}
/// Tracks build artifacts and cache validity.
#[derive(Clone, Debug)]
pub struct BuildCache {
    /// Last successful build artifacts.
    pub artifacts: BTreeMap<String, BuildArtifact>,
    /// Global cache invalidation timestamp.
    pub invalidated_at: Option<SystemTime>,
}
impl BuildCache {
    /// Create an empty cache.
    pub fn new() -> Self {
        Self {
            artifacts: BTreeMap::new(),
            invalidated_at: None,
        }
    }
    /// Add or update a cached artifact.
    pub fn insert(&mut self, artifact: BuildArtifact) {
        self.artifacts
            .insert(artifact.module_name.clone(), artifact);
    }
    /// Check if a module's artifact is still valid.
    pub fn is_valid(&self, module_name: &str, current_hash: &str) -> bool {
        if let Some(inv_time) = self.invalidated_at {
            if let Some(artifact) = self.artifacts.get(module_name) {
                return artifact.source_hash == current_hash && artifact.built_at > inv_time;
            }
            return false;
        }
        self.artifacts
            .get(module_name)
            .map(|a| a.source_hash == current_hash)
            .unwrap_or(false)
    }
    /// Invalidate the cache.
    pub fn invalidate(&mut self) {
        self.invalidated_at = Some(SystemTime::now());
    }
    /// Clear all cached artifacts.
    pub fn clear(&mut self) {
        self.artifacts.clear();
        self.invalidated_at = None;
    }
}
/// A package in the registry.
#[derive(Clone, Debug, Default)]
pub struct PackageEntry {
    /// Package name.
    pub name: String,
    /// Available versions (newest first).
    pub versions: Vec<String>,
    /// Description.
    pub description: String,
    /// Declared dependencies of this package (for transitive resolution).
    pub deps: Vec<Dependency>,
}
/// Directed dependency graph over modules.
#[derive(Clone, Debug, Default)]
pub struct ModuleGraph {
    /// Adjacency list: module name -> set of module names it depends on.
    pub edges: HashMap<String, Vec<String>>,
    /// Reverse edges: module name -> set of modules that depend on it.
    pub reverse_edges: HashMap<String, Vec<String>>,
    /// All known module names.
    pub nodes: HashSet<String>,
}
impl ModuleGraph {
    /// Create an empty graph.
    pub fn new() -> Self {
        Self::default()
    }
    /// Add a module node.
    pub fn add_node(&mut self, name: &str) {
        self.nodes.insert(name.to_string());
        self.edges.entry(name.to_string()).or_default();
        self.reverse_edges.entry(name.to_string()).or_default();
    }
    /// Add a dependency edge: `from` depends on `to`.
    pub fn add_edge(&mut self, from: &str, to: &str) {
        self.add_node(from);
        self.add_node(to);
        self.edges
            .entry(from.to_string())
            .or_default()
            .push(to.to_string());
        self.reverse_edges
            .entry(to.to_string())
            .or_default()
            .push(from.to_string());
    }
    /// Get direct dependencies of a module.
    pub fn dependencies_of(&self, name: &str) -> &[String] {
        self.edges.get(name).map(|v| v.as_slice()).unwrap_or(&[])
    }
    /// Get direct dependents (reverse dependencies) of a module.
    pub fn dependents_of(&self, name: &str) -> &[String] {
        self.reverse_edges
            .get(name)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }
    /// Get the number of modules.
    pub fn len(&self) -> usize {
        self.nodes.len()
    }
    /// Check if the graph is empty.
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }
    /// Compute transitive closure of dependencies for a module.
    pub fn transitive_deps(&self, name: &str) -> HashSet<String> {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back(name.to_string());
        while let Some(current) = queue.pop_front() {
            for dep in self.dependencies_of(&current) {
                if visited.insert(dep.clone()) {
                    queue.push_back(dep.clone());
                }
            }
        }
        visited
    }
}
