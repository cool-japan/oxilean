//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::{BTreeMap, HashMap};
use std::path::{Path, PathBuf};

/// The complete package manifest.
#[derive(Clone, Debug)]
pub struct Manifest {
    /// Package name.
    pub name: String,
    /// Package version.
    pub version: Version,
    /// Package metadata.
    pub metadata: PackageMetadata,
    /// Dependencies.
    pub dependencies: BTreeMap<String, Dependency>,
    /// Dev dependencies.
    pub dev_dependencies: BTreeMap<String, Dependency>,
    /// Build dependencies.
    pub build_dependencies: BTreeMap<String, Dependency>,
    /// Feature definitions.
    pub features: BTreeMap<String, Feature>,
    /// Default features.
    pub default_features: Vec<String>,
    /// Build profiles.
    pub profiles: BTreeMap<String, BuildProfile>,
    /// Build targets.
    pub targets: Vec<Target>,
    /// Path to the manifest file.
    pub manifest_path: PathBuf,
    /// Custom build scripts.
    pub build_scripts: Vec<PathBuf>,
    /// Workspace configuration.
    pub workspace: Option<WorkspaceConfig>,
}
impl Manifest {
    /// Create a minimal manifest.
    pub fn new(name: &str, version: Version) -> Self {
        let mut profiles = BTreeMap::new();
        profiles.insert("debug".to_string(), BuildProfile::debug());
        profiles.insert("release".to_string(), BuildProfile::release());
        profiles.insert("test".to_string(), BuildProfile::test());
        Self {
            name: name.to_string(),
            version,
            metadata: PackageMetadata::new(),
            dependencies: BTreeMap::new(),
            dev_dependencies: BTreeMap::new(),
            build_dependencies: BTreeMap::new(),
            features: BTreeMap::new(),
            default_features: Vec::new(),
            profiles,
            targets: Vec::new(),
            manifest_path: PathBuf::new(),
            build_scripts: Vec::new(),
            workspace: None,
        }
    }
    /// Add a dependency.
    pub fn add_dependency(&mut self, dep: Dependency) {
        self.dependencies.insert(dep.name.clone(), dep);
    }
    /// Add a dev dependency.
    pub fn add_dev_dependency(&mut self, dep: Dependency) {
        self.dev_dependencies.insert(dep.name.clone(), dep);
    }
    /// Add a build dependency.
    pub fn add_build_dependency(&mut self, dep: Dependency) {
        self.build_dependencies.insert(dep.name.clone(), dep);
    }
    /// Add a feature.
    pub fn add_feature(&mut self, feature: Feature) {
        self.features.insert(feature.name.clone(), feature);
    }
    /// Add a target.
    pub fn add_target(&mut self, target: Target) {
        self.targets.push(target);
    }
    /// Get the package root directory.
    pub fn package_root(&self) -> Option<&Path> {
        self.manifest_path.parent()
    }
    /// Get the library target, if any.
    pub fn lib_target(&self) -> Option<&Target> {
        self.targets.iter().find(|t| t.kind == TargetKind::Library)
    }
    /// Get all executable targets.
    pub fn bin_targets(&self) -> Vec<&Target> {
        self.targets
            .iter()
            .filter(|t| t.kind == TargetKind::Executable)
            .collect()
    }
    /// Get all test targets.
    pub fn test_targets(&self) -> Vec<&Target> {
        self.targets
            .iter()
            .filter(|t| t.kind == TargetKind::Test)
            .collect()
    }
    /// Resolve all enabled features given a set of requested features.
    pub fn resolve_features(&self, requested: &[String]) -> HashMap<String, bool> {
        let mut enabled: HashMap<String, bool> = HashMap::new();
        for f in &self.default_features {
            enabled.insert(f.clone(), true);
        }
        for f in requested {
            enabled.insert(f.clone(), true);
        }
        let mut changed = true;
        while changed {
            changed = false;
            let current: Vec<String> = enabled.keys().cloned().collect();
            for name in &current {
                if let Some(feature) = self.features.get(name) {
                    for implied in &feature.implies {
                        if !enabled.contains_key(implied) {
                            enabled.insert(implied.clone(), true);
                            changed = true;
                        }
                    }
                }
            }
        }
        enabled
    }
    /// Validate the manifest for internal consistency.
    pub fn validate(&self) -> Vec<ManifestError> {
        let mut errors = Vec::new();
        if self.name.is_empty() {
            errors.push(ManifestError::InvalidName(
                "package name cannot be empty".to_string(),
            ));
        }
        if self.name.contains(' ') {
            errors.push(ManifestError::InvalidName(
                "package name cannot contain spaces".to_string(),
            ));
        }
        for (fname, feature) in &self.features {
            for implied in &feature.implies {
                if !self.features.contains_key(implied) {
                    errors.push(ManifestError::UnknownFeature(format!(
                        "feature '{}' implies unknown feature '{}'",
                        fname, implied
                    )));
                }
            }
            for dep in &feature.enables {
                if !self.dependencies.contains_key(dep) {
                    errors.push(ManifestError::UnknownDependency(format!(
                        "feature '{}' enables unknown dependency '{}'",
                        fname, dep
                    )));
                }
            }
        }
        for f in &self.default_features {
            if !self.features.contains_key(f) {
                errors.push(ManifestError::UnknownFeature(format!(
                    "default feature '{}' not defined",
                    f
                )));
            }
        }
        errors
    }
    /// Get the profile for a given name.
    pub fn get_profile(&self, name: &str) -> Option<&BuildProfile> {
        self.profiles.get(name)
    }
    /// Get all active dependencies for a given feature set.
    pub fn all_dependencies(&self, features: &HashMap<String, bool>) -> Vec<&Dependency> {
        let mut deps: Vec<&Dependency> = Vec::new();
        for dep in self.dependencies.values() {
            if !dep.optional {
                deps.push(dep);
            } else {
                for feature in self.features.values() {
                    if features.contains_key(&feature.name) && feature.enables.contains(&dep.name) {
                        deps.push(dep);
                        break;
                    }
                }
            }
        }
        deps
    }
}
/// A simple in-memory package registry.
#[allow(dead_code)]
pub struct PackageRegistry {
    entries: BTreeMap<String, RegistryEntry>,
}
#[allow(dead_code)]
impl PackageRegistry {
    /// Create an empty registry.
    pub fn new() -> Self {
        Self {
            entries: BTreeMap::new(),
        }
    }
    /// Publish a package entry.
    pub fn publish(&mut self, entry: RegistryEntry) {
        self.entries.insert(entry.name.clone(), entry);
    }
    /// Look up a package by name.
    pub fn lookup(&self, name: &str) -> Option<&RegistryEntry> {
        self.entries.get(name)
    }
    /// Find the best version matching a constraint.
    pub fn best_version(&self, name: &str, constraint: &VersionConstraint) -> Option<&Version> {
        let entry = self.entries.get(name)?;
        if entry.yanked {
            return None;
        }
        entry
            .matching_versions(constraint)
            .into_iter()
            .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
    }
    /// All package names in the registry.
    pub fn package_names(&self) -> Vec<&str> {
        self.entries.keys().map(|s| s.as_str()).collect()
    }
    /// Number of packages.
    pub fn package_count(&self) -> usize {
        self.entries.len()
    }
    /// Search for packages whose name contains the query string.
    pub fn search(&self, query: &str) -> Vec<&RegistryEntry> {
        self.entries
            .values()
            .filter(|e| e.name.contains(query))
            .collect()
    }
}
/// A semantic version: major.minor.patch with optional pre-release and build.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Version {
    /// Major version number.
    pub major: u64,
    /// Minor version number.
    pub minor: u64,
    /// Patch version number.
    pub patch: u64,
    /// Optional pre-release tag (e.g. "alpha.1").
    pub pre: Option<String>,
    /// Optional build metadata (e.g. "20240101").
    pub build_meta: Option<String>,
}
impl Version {
    /// Create a new version with major.minor.patch.
    pub fn new(major: u64, minor: u64, patch: u64) -> Self {
        Self {
            major,
            minor,
            patch,
            pre: None,
            build_meta: None,
        }
    }
    /// Create a version with a pre-release tag.
    pub fn with_pre(mut self, pre: &str) -> Self {
        self.pre = Some(pre.to_string());
        self
    }
    /// Create a version with build metadata.
    pub fn with_build_meta(mut self, build: &str) -> Self {
        self.build_meta = Some(build.to_string());
        self
    }
    /// Returns true if this is a pre-release version.
    pub fn is_prerelease(&self) -> bool {
        self.pre.is_some()
    }
    /// Parse a version string (e.g. "1.2.3" or "1.0.0-alpha").
    pub fn parse(s: &str) -> Result<Self, VersionParseError> {
        s.parse()
    }
    /// Returns the next major version (e.g. 1.2.3 -> 2.0.0).
    pub fn next_major(&self) -> Self {
        Self::new(self.major + 1, 0, 0)
    }
    /// Returns the next minor version (e.g. 1.2.3 -> 1.3.0).
    pub fn next_minor(&self) -> Self {
        Self::new(self.major, self.minor + 1, 0)
    }
    /// Returns the next patch version (e.g. 1.2.3 -> 1.2.4).
    pub fn next_patch(&self) -> Self {
        Self::new(self.major, self.minor, self.patch + 1)
    }
    /// Compare two versions (ignoring build metadata).
    pub fn cmp_precedence(&self, other: &Self) -> std::cmp::Ordering {
        match self.major.cmp(&other.major) {
            std::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        match self.minor.cmp(&other.minor) {
            std::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        match self.patch.cmp(&other.patch) {
            std::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        match (&self.pre, &other.pre) {
            (None, None) => std::cmp::Ordering::Equal,
            (Some(_), None) => std::cmp::Ordering::Less,
            (None, Some(_)) => std::cmp::Ordering::Greater,
            (Some(a), Some(b)) => a.cmp(b),
        }
    }
}
/// A feature flag definition.
#[derive(Clone, Debug)]
pub struct Feature {
    /// Feature name.
    pub name: String,
    /// Dependencies that this feature enables.
    pub enables: Vec<String>,
    /// Other features that this feature enables.
    pub implies: Vec<String>,
}
impl Feature {
    /// Create a new feature.
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            enables: Vec::new(),
            implies: Vec::new(),
        }
    }
    /// Add a dependency that this feature enables.
    pub fn enable_dep(mut self, dep: &str) -> Self {
        self.enables.push(dep.to_string());
        self
    }
    /// Add a feature that this feature implies.
    pub fn imply(mut self, feature: &str) -> Self {
        self.implies.push(feature.to_string());
        self
    }
}
/// The lockfile containing resolved dependency versions.
#[derive(Clone, Debug)]
pub struct Lockfile {
    /// Lockfile format version.
    pub version: u32,
    /// Resolved packages.
    pub packages: Vec<LockedDependency>,
}
impl Lockfile {
    /// Create an empty lockfile.
    pub fn new() -> Self {
        Self {
            version: 1,
            packages: Vec::new(),
        }
    }
    /// Add a locked dependency.
    pub fn add_package(&mut self, dep: LockedDependency) {
        self.packages.push(dep);
    }
    /// Find a locked package by name.
    pub fn find_package(&self, name: &str) -> Option<&LockedDependency> {
        self.packages.iter().find(|p| p.name == name)
    }
    /// Serialize the lockfile to a string.
    pub fn serialize(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!("# lockfile v{}\n\n", self.version));
        for pkg in &self.packages {
            out.push_str("[[package]]\n");
            out.push_str(&format!("name = \"{}\"\n", pkg.name));
            out.push_str(&format!("version = \"{}\"\n", pkg.version));
            out.push_str(&format!("source = \"{}\"\n", pkg.source));
            if let Some(ref checksum) = pkg.checksum {
                out.push_str(&format!("checksum = \"{}\"\n", checksum));
            }
            if !pkg.dependencies.is_empty() {
                out.push_str("dependencies = [");
                for (i, dep) in pkg.dependencies.iter().enumerate() {
                    if i > 0 {
                        out.push_str(", ");
                    }
                    out.push_str(&format!("\"{}\"", dep));
                }
                out.push_str("]\n");
            }
            out.push('\n');
        }
        out
    }
    /// Check if the lockfile is up-to-date with the manifest.
    pub fn is_up_to_date(&self, manifest: &Manifest) -> bool {
        for (dep_name, dep) in &manifest.dependencies {
            match self.find_package(dep_name) {
                Some(locked) => {
                    if !dep.version.matches(&locked.version) {
                        return false;
                    }
                }
                None => return false,
            }
        }
        true
    }
}
/// An environment-variable override for a manifest field.
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EnvOverride {
    /// The environment variable name.
    pub var: String,
    /// The manifest field being overridden (e.g. "version", "registry").
    pub field: String,
    /// Optional default if the variable is not set.
    pub default: Option<String>,
}
#[allow(dead_code)]
impl EnvOverride {
    /// Create a new override with no default.
    pub fn new(var: &str, field: &str) -> Self {
        Self {
            var: var.to_string(),
            field: field.to_string(),
            default: None,
        }
    }
    /// Set a default value.
    pub fn with_default(mut self, default: &str) -> Self {
        self.default = Some(default.to_string());
        self
    }
    /// Resolve the value: env var → default → None.
    pub fn resolve(&self) -> Option<String> {
        std::env::var(&self.var)
            .ok()
            .or_else(|| self.default.clone())
    }
    /// Whether the environment variable is currently set.
    pub fn is_set(&self) -> bool {
        std::env::var(&self.var).is_ok()
    }
}
/// Git reference for a dependency.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GitReference {
    /// A branch name.
    Branch(String),
    /// A tag name.
    Tag(String),
    /// A commit hash.
    Rev(String),
    /// Default branch (usually `main` or `master`).
    Default,
}
/// A lint configuration for a build target.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct LintConfig {
    /// Allowed lints.
    pub allow: Vec<String>,
    /// Denied lints.
    pub deny: Vec<String>,
    /// Warned lints.
    pub warn: Vec<String>,
    /// Whether to enable all clippy lints.
    pub clippy_all: bool,
}
#[allow(dead_code)]
impl LintConfig {
    /// Create a default lint configuration.
    pub fn new() -> Self {
        Self {
            allow: Vec::new(),
            deny: Vec::new(),
            warn: Vec::new(),
            clippy_all: false,
        }
    }
    /// Allow a lint.
    pub fn allow(mut self, lint: &str) -> Self {
        self.allow.push(lint.to_string());
        self
    }
    /// Deny a lint.
    pub fn deny(mut self, lint: &str) -> Self {
        self.deny.push(lint.to_string());
        self
    }
    /// Warn on a lint.
    pub fn warn(mut self, lint: &str) -> Self {
        self.warn.push(lint.to_string());
        self
    }
    /// Enable all clippy lints.
    pub fn with_clippy_all(mut self) -> Self {
        self.clippy_all = true;
        self
    }
    /// Generate a rustflags-style string for these lints.
    pub fn to_rustflags(&self) -> String {
        let mut flags = Vec::new();
        for lint in &self.allow {
            flags.push(format!("-A {}", lint));
        }
        for lint in &self.deny {
            flags.push(format!("-D {}", lint));
        }
        for lint in &self.warn {
            flags.push(format!("-W {}", lint));
        }
        flags.join(" ")
    }
}
/// A simple TOML value type for serialization without external deps.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum TomlSerialValue {
    /// A string value.
    String(String),
    /// An integer value.
    Integer(i64),
    /// A float value.
    Float(f64),
    /// A boolean value.
    Bool(bool),
    /// An array of values.
    Array(Vec<TomlSerialValue>),
    /// An inline table.
    Table(BTreeMap<String, TomlSerialValue>),
}
#[allow(dead_code)]
impl TomlSerialValue {
    /// Serialize to a TOML-formatted string.
    pub fn to_toml_string(&self) -> String {
        match self {
            Self::String(s) => {
                format!("\"{}\"", s.replace('\\', "\\\\").replace('"', "\\\""))
            }
            Self::Integer(i) => i.to_string(),
            Self::Float(f) => format!("{}", f),
            Self::Bool(b) => b.to_string(),
            Self::Array(items) => {
                let parts: Vec<_> = items.iter().map(|v| v.to_toml_string()).collect();
                format!("[{}]", parts.join(", "))
            }
            Self::Table(map) => {
                let parts: Vec<_> = map
                    .iter()
                    .map(|(k, v)| format!("{} = {}", k, v.to_toml_string()))
                    .collect();
                format!("{{{}}}", parts.join(", "))
            }
        }
    }
    /// Try to extract a string value.
    pub fn as_str(&self) -> Option<&str> {
        if let Self::String(s) = self {
            Some(s.as_str())
        } else {
            None
        }
    }
    /// Try to extract an integer.
    pub fn as_i64(&self) -> Option<i64> {
        if let Self::Integer(i) = self {
            Some(*i)
        } else {
            None
        }
    }
    /// Try to extract a bool.
    pub fn as_bool(&self) -> Option<bool> {
        if let Self::Bool(b) = self {
            Some(*b)
        } else {
            None
        }
    }
}
/// A build target within a package.
#[derive(Clone, Debug)]
pub struct Target {
    /// Target name.
    pub name: String,
    /// Target kind.
    pub kind: TargetKind,
    /// Source file path (relative to package root).
    pub src_path: PathBuf,
    /// Required features for this target.
    pub required_features: Vec<String>,
    /// Extra compiler flags specific to this target.
    pub flags: Vec<String>,
}
impl Target {
    /// Create a library target.
    pub fn library(name: &str, src_path: &Path) -> Self {
        Self {
            name: name.to_string(),
            kind: TargetKind::Library,
            src_path: src_path.to_path_buf(),
            required_features: Vec::new(),
            flags: Vec::new(),
        }
    }
    /// Create an executable target.
    pub fn executable(name: &str, src_path: &Path) -> Self {
        Self {
            name: name.to_string(),
            kind: TargetKind::Executable,
            src_path: src_path.to_path_buf(),
            required_features: Vec::new(),
            flags: Vec::new(),
        }
    }
    /// Create a test target.
    pub fn test(name: &str, src_path: &Path) -> Self {
        Self {
            name: name.to_string(),
            kind: TargetKind::Test,
            src_path: src_path.to_path_buf(),
            required_features: Vec::new(),
            flags: Vec::new(),
        }
    }
}
/// Resolves platform-specific dependencies for the current or a target platform.
#[allow(dead_code)]
pub struct PlatformResolver {
    platform_deps: Vec<PlatformDependency>,
}
#[allow(dead_code)]
impl PlatformResolver {
    /// Create a new resolver.
    pub fn new() -> Self {
        Self {
            platform_deps: Vec::new(),
        }
    }
    /// Add a platform dependency.
    pub fn add(&mut self, dep: PlatformDependency) {
        self.platform_deps.push(dep);
    }
    /// Resolve applicable dependencies for the given (os, arch, triple).
    pub fn resolve(&self, os: &str, arch: &str, triple: &str) -> HashMap<String, Dependency> {
        let mut result = HashMap::new();
        for pd in &self.platform_deps {
            if pd.applies(os, arch, triple) {
                result.insert(pd.name.clone(), pd.dep.clone());
            }
        }
        result
    }
    /// Number of registered platform deps.
    pub fn count(&self) -> usize {
        self.platform_deps.len()
    }
}
/// An entry in a package registry.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct RegistryEntry {
    /// Package name.
    pub name: String,
    /// Available versions (sorted ascending).
    pub versions: Vec<Version>,
    /// Latest stable version.
    pub latest: Version,
    /// Description.
    pub description: Option<String>,
    /// Download count (for popularity ranking).
    pub downloads: u64,
    /// Whether the package is yanked.
    pub yanked: bool,
}
#[allow(dead_code)]
impl RegistryEntry {
    /// Create a registry entry.
    pub fn new(name: &str, latest: Version) -> Self {
        Self {
            name: name.to_string(),
            versions: vec![latest.clone()],
            latest,
            description: None,
            downloads: 0,
            yanked: false,
        }
    }
    /// Add a version.
    pub fn with_version(mut self, v: Version) -> Self {
        if !self.versions.contains(&v) {
            self.versions.push(v.clone());
            self.versions.sort_unstable();
        }
        if v > self.latest {
            self.latest = v;
        }
        self
    }
    /// Set download count.
    pub fn with_downloads(mut self, n: u64) -> Self {
        self.downloads = n;
        self
    }
    /// Yank this package.
    pub fn yank(&mut self) {
        self.yanked = true;
    }
    /// Versions that match the given constraint.
    pub fn matching_versions(&self, constraint: &VersionConstraint) -> Vec<&Version> {
        self.versions
            .iter()
            .filter(|v| constraint.matches(v))
            .collect()
    }
}
/// A package dependency.
#[derive(Clone, Debug)]
pub struct Dependency {
    /// The package name.
    pub name: String,
    /// Version constraint.
    pub version: VersionConstraint,
    /// Dependency source.
    pub source: DependencySource,
    /// Whether this is an optional dependency.
    pub optional: bool,
    /// Feature flags to enable for this dependency.
    pub features: Vec<String>,
    /// Whether default features are enabled.
    pub default_features: bool,
}
impl Dependency {
    /// Create a new registry dependency.
    pub fn new(name: &str, version: VersionConstraint) -> Self {
        Self {
            name: name.to_string(),
            version,
            source: DependencySource::Registry {
                registry: "default".to_string(),
            },
            optional: false,
            features: Vec::new(),
            default_features: true,
        }
    }
    /// Create a registry dependency (alias for `new`).
    pub fn registry(name: &str, version: VersionConstraint) -> Self {
        Self::new(name, version)
    }
    /// Create a path dependency.
    pub fn path(name: &str, path: &Path) -> Self {
        Self {
            name: name.to_string(),
            version: VersionConstraint::Any,
            source: DependencySource::Path {
                path: path.to_path_buf(),
            },
            optional: false,
            features: Vec::new(),
            default_features: true,
        }
    }
    /// Create a git dependency.
    pub fn git(name: &str, url: &str, reference: GitReference) -> Self {
        Self {
            name: name.to_string(),
            version: VersionConstraint::Any,
            source: DependencySource::Git {
                url: url.to_string(),
                reference,
            },
            optional: false,
            features: Vec::new(),
            default_features: true,
        }
    }
    /// Set whether this dependency is optional.
    pub fn set_optional(mut self, optional: bool) -> Self {
        self.optional = optional;
        self
    }
    /// Add a feature flag for this dependency.
    pub fn add_feature(mut self, feature: &str) -> Self {
        self.features.push(feature.to_string());
        self
    }
    /// Check if the dependency is a local path dependency.
    pub fn is_path(&self) -> bool {
        matches!(self.source, DependencySource::Path { .. })
    }
    /// Check if the dependency is from a git repository.
    pub fn is_git(&self) -> bool {
        matches!(self.source, DependencySource::Git { .. })
    }
    /// Check if the dependency is from a registry.
    pub fn is_registry(&self) -> bool {
        matches!(self.source, DependencySource::Registry { .. })
    }
}
/// A constraint on package versions.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum VersionConstraint {
    /// Exact version match: `= 1.2.3`.
    Exact(Version),
    /// Greater than or equal: `>= 1.2.3`.
    GreaterEqual(Version),
    /// Greater than: `> 1.2.3`.
    Greater(Version),
    /// Less than or equal: `<= 1.2.3`.
    LessEqual(Version),
    /// Less than: `< 1.2.3`.
    Less(Version),
    /// Compatible range (caret): `^1.2.3` means `>= 1.2.3, < 2.0.0`.
    Caret(Version),
    /// Tilde range: `~1.2.3` means `>= 1.2.3, < 1.3.0`.
    Tilde(Version),
    /// Wildcard: `1.2.*` means `>= 1.2.0, < 1.3.0`.
    Wildcard {
        /// Major version.
        major: u64,
        /// Optional minor version.
        minor: Option<u64>,
    },
    /// Intersection of multiple constraints.
    And(Vec<VersionConstraint>),
    /// Union of multiple constraints.
    Or(Vec<VersionConstraint>),
    /// Any version.
    Any,
}
impl VersionConstraint {
    /// Check if a version satisfies this constraint.
    pub fn matches(&self, version: &Version) -> bool {
        match self {
            Self::Exact(v) => version == v,
            Self::GreaterEqual(v) => version >= v,
            Self::Greater(v) => version > v,
            Self::LessEqual(v) => version <= v,
            Self::Less(v) => version < v,
            Self::Caret(v) => {
                if version < v {
                    return false;
                }
                if v.major == 0 {
                    if v.minor == 0 {
                        version.major == 0 && version.minor == 0 && version.patch == v.patch
                    } else {
                        version.major == 0 && version.minor == v.minor
                    }
                } else {
                    version.major == v.major
                }
            }
            Self::Tilde(v) => version >= v && version.major == v.major && version.minor == v.minor,
            Self::Wildcard { major, minor } => {
                if version.major != *major {
                    return false;
                }
                match minor {
                    Some(m) => version.minor == *m,
                    None => true,
                }
            }
            Self::And(constraints) => constraints.iter().all(|c| c.matches(version)),
            Self::Or(constraints) => constraints.iter().any(|c| c.matches(version)),
            Self::Any => true,
        }
    }
    /// Return the intersection of two constraints.
    pub fn intersect(self, other: Self) -> Self {
        match (self, other) {
            (Self::Any, c) | (c, Self::Any) => c,
            (Self::And(mut a), Self::And(b)) => {
                a.extend(b);
                Self::And(a)
            }
            (Self::And(mut a), c) => {
                a.push(c);
                Self::And(a)
            }
            (c, Self::And(mut a)) => {
                a.push(c);
                Self::And(a)
            }
            (a, b) => Self::And(vec![a, b]),
        }
    }
    /// Return the union of two constraints.
    pub fn union(self, other: Self) -> Self {
        match (self, other) {
            (Self::Any, _) | (_, Self::Any) => Self::Any,
            (Self::Or(mut a), Self::Or(b)) => {
                a.extend(b);
                Self::Or(a)
            }
            (Self::Or(mut a), c) => {
                a.push(c);
                Self::Or(a)
            }
            (c, Self::Or(mut a)) => {
                a.push(c);
                Self::Or(a)
            }
            (a, b) => Self::Or(vec![a, b]),
        }
    }
    /// Check if this constraint is satisfiable (conservative check).
    pub fn is_satisfiable(&self) -> bool {
        match self {
            Self::Any => true,
            Self::Exact(_)
            | Self::GreaterEqual(_)
            | Self::Greater(_)
            | Self::LessEqual(_)
            | Self::Less(_)
            | Self::Caret(_)
            | Self::Tilde(_)
            | Self::Wildcard { .. } => true,
            Self::And(constraints) => constraints.iter().all(|c| c.is_satisfiable()),
            Self::Or(constraints) => constraints.iter().any(|c| c.is_satisfiable()),
        }
    }
}
/// Error type for version parsing.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum VersionParseError {
    /// The version string is empty.
    Empty,
    /// Missing a required component.
    MissingComponent(&'static str),
    /// A numeric component is invalid.
    InvalidNumber(String),
    /// Unexpected character in version string.
    UnexpectedChar(char),
}
/// A build profile (e.g., debug, release).
#[derive(Clone, Debug)]
pub struct BuildProfile {
    /// Profile name.
    pub name: String,
    /// Optimization level.
    pub opt_level: OptLevel,
    /// Enable debug info.
    pub debug: bool,
    /// Enable link-time optimization.
    pub lto: bool,
    /// Number of codegen units.
    pub codegen_units: Option<u32>,
    /// Enable incremental compilation.
    pub incremental: bool,
    /// Extra compiler flags.
    pub flags: Vec<String>,
    /// Environment variables.
    pub env_vars: HashMap<String, String>,
}
impl BuildProfile {
    /// Create a debug profile with default settings.
    pub fn debug() -> Self {
        Self {
            name: "debug".to_string(),
            opt_level: OptLevel::None,
            debug: true,
            lto: false,
            codegen_units: None,
            incremental: true,
            flags: Vec::new(),
            env_vars: HashMap::new(),
        }
    }
    /// Create a release profile with default settings.
    pub fn release() -> Self {
        Self {
            name: "release".to_string(),
            opt_level: OptLevel::Full,
            debug: false,
            lto: true,
            codegen_units: Some(1),
            incremental: false,
            flags: Vec::new(),
            env_vars: HashMap::new(),
        }
    }
    /// Create a test profile.
    pub fn test() -> Self {
        Self {
            name: "test".to_string(),
            opt_level: OptLevel::None,
            debug: true,
            lto: false,
            codegen_units: None,
            incremental: true,
            flags: vec!["--test".to_string()],
            env_vars: HashMap::new(),
        }
    }
    /// Add a compiler flag.
    pub fn add_flag(mut self, flag: &str) -> Self {
        self.flags.push(flag.to_string());
        self
    }
    /// Set an environment variable.
    pub fn set_env(mut self, key: &str, value: &str) -> Self {
        self.env_vars.insert(key.to_string(), value.to_string());
        self
    }
}
/// Workspace configuration for multi-package projects.
#[derive(Clone, Debug)]
pub struct WorkspaceConfig {
    /// Member package paths (relative to workspace root).
    pub members: Vec<String>,
    /// Exclude patterns.
    pub exclude: Vec<String>,
    /// Shared dependencies.
    pub shared_dependencies: BTreeMap<String, Dependency>,
    /// Default members to build.
    pub default_members: Vec<String>,
}
impl WorkspaceConfig {
    /// Create a new workspace configuration.
    pub fn new() -> Self {
        Self {
            members: Vec::new(),
            exclude: Vec::new(),
            shared_dependencies: BTreeMap::new(),
            default_members: Vec::new(),
        }
    }
    /// Add a member package.
    pub fn add_member(&mut self, path: &str) {
        self.members.push(path.to_string());
    }
    /// Add an exclusion pattern.
    pub fn add_exclude(&mut self, pattern: &str) {
        self.exclude.push(pattern.to_string());
    }
    /// Check if a path is a workspace member.
    pub fn is_member(&self, path: &str) -> bool {
        self.members.iter().any(|m| m == path)
    }
}
/// Error type for manifest operations.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ManifestError {
    /// Invalid package name.
    InvalidName(String),
    /// Invalid version string.
    InvalidVersion(String),
    /// Unknown feature reference.
    UnknownFeature(String),
    /// Unknown dependency reference.
    UnknownDependency(String),
    /// Parse error in manifest file.
    ParseError(String),
    /// IO error (stored as string for Clone/Eq).
    IoError(String),
    /// Duplicate key in manifest.
    DuplicateKey(String),
    /// Missing required field.
    MissingField(String),
}
/// Package metadata.
#[derive(Clone, Debug)]
pub struct PackageMetadata {
    /// Package description.
    pub description: Option<String>,
    /// Package license (SPDX identifier).
    pub license: Option<String>,
    /// Repository URL.
    pub repository: Option<String>,
    /// Documentation URL.
    pub documentation: Option<String>,
    /// Homepage URL.
    pub homepage: Option<String>,
    /// List of authors.
    pub authors: Vec<String>,
    /// Keywords for search.
    pub keywords: Vec<String>,
    /// Categories for classification.
    pub categories: Vec<String>,
    /// Minimum OxiLean version required.
    pub min_oxilean_version: Option<Version>,
}
impl PackageMetadata {
    /// Create empty metadata.
    pub fn new() -> Self {
        Self {
            description: None,
            license: None,
            repository: None,
            documentation: None,
            homepage: None,
            authors: Vec::new(),
            keywords: Vec::new(),
            categories: Vec::new(),
            min_oxilean_version: None,
        }
    }
    /// Check if the metadata is valid for publishing.
    pub fn validate_for_publish(&self) -> Vec<String> {
        let mut warnings = Vec::new();
        if self.description.is_none() {
            warnings.push("missing description".to_string());
        }
        if self.license.is_none() {
            warnings.push("missing license".to_string());
        }
        if self.authors.is_empty() {
            warnings.push("no authors specified".to_string());
        }
        warnings
    }
}
/// Extended build target with additional configuration.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct ExtendedBuildTarget {
    /// Target name.
    pub name: String,
    /// Target kind (lib, bin, test, bench, example).
    pub kind: String,
    /// Entry source file.
    pub src_path: PathBuf,
    /// Required features for this target.
    pub required_features: Vec<String>,
    /// Lint configuration.
    pub lint: LintConfig,
    /// Whether this target is public (exported from the package).
    pub public: bool,
    /// Additional rustc flags.
    pub rustflags: Vec<String>,
    /// Additional link libraries.
    pub link_libs: Vec<String>,
    /// Documentation URL.
    pub doc_url: Option<String>,
}
#[allow(dead_code)]
impl ExtendedBuildTarget {
    /// Create a new binary target.
    pub fn binary(name: &str) -> Self {
        Self {
            name: name.to_string(),
            kind: "bin".to_string(),
            src_path: PathBuf::from(format!("src/bin/{}.rs", name)),
            required_features: Vec::new(),
            lint: LintConfig::new(),
            public: true,
            rustflags: Vec::new(),
            link_libs: Vec::new(),
            doc_url: None,
        }
    }
    /// Create a new library target.
    pub fn library(name: &str) -> Self {
        Self {
            name: name.to_string(),
            kind: "lib".to_string(),
            src_path: PathBuf::from("src/lib.rs"),
            required_features: Vec::new(),
            lint: LintConfig::new(),
            public: true,
            rustflags: Vec::new(),
            link_libs: Vec::new(),
            doc_url: None,
        }
    }
    /// Create a new test target.
    pub fn test(name: &str) -> Self {
        Self {
            name: name.to_string(),
            kind: "test".to_string(),
            src_path: PathBuf::from(format!("tests/{}.rs", name)),
            required_features: Vec::new(),
            lint: LintConfig::new(),
            public: false,
            rustflags: Vec::new(),
            link_libs: Vec::new(),
            doc_url: None,
        }
    }
    /// Create a benchmark target.
    pub fn bench(name: &str) -> Self {
        Self {
            name: name.to_string(),
            kind: "bench".to_string(),
            src_path: PathBuf::from(format!("benches/{}.rs", name)),
            required_features: Vec::new(),
            lint: LintConfig::new(),
            public: false,
            rustflags: Vec::new(),
            link_libs: Vec::new(),
            doc_url: None,
        }
    }
    /// Require a feature.
    pub fn require_feature(mut self, feature: &str) -> Self {
        self.required_features.push(feature.to_string());
        self
    }
    /// Add a link library.
    pub fn link_lib(mut self, lib: &str) -> Self {
        self.link_libs.push(lib.to_string());
        self
    }
    /// Add a rustc flag.
    pub fn rustflag(mut self, flag: &str) -> Self {
        self.rustflags.push(flag.to_string());
        self
    }
    /// Set the documentation URL.
    pub fn with_doc_url(mut self, url: &str) -> Self {
        self.doc_url = Some(url.to_string());
        self
    }
    /// Combined rustflags including lint flags.
    pub fn all_rustflags(&self) -> String {
        let mut flags = self.rustflags.clone();
        let lint_flags = self.lint.to_rustflags();
        if !lint_flags.is_empty() {
            flags.push(lint_flags);
        }
        flags.join(" ")
    }
}
/// A simple TOML value for manifest parsing.
#[derive(Clone, Debug)]
pub enum TomlValue {
    /// A string value.
    Str(String),
    /// An integer value.
    Integer(i64),
    /// A float value.
    Float(f64),
    /// A boolean value.
    Boolean(bool),
    /// An array of values.
    Array(Vec<TomlValue>),
    /// A table (key-value pairs).
    Table(BTreeMap<String, TomlValue>),
}
impl TomlValue {
    /// Try to extract as a string.
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Self::Str(s) => Some(s),
            _ => None,
        }
    }
    /// Try to extract as an integer.
    pub fn as_int(&self) -> Option<i64> {
        match self {
            Self::Integer(n) => Some(*n),
            _ => None,
        }
    }
    /// Try to extract as a float.
    pub fn as_float(&self) -> Option<f64> {
        match self {
            Self::Float(f) => Some(*f),
            _ => None,
        }
    }
    /// Try to extract as a boolean.
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Self::Boolean(b) => Some(*b),
            _ => None,
        }
    }
    /// Try to extract as a table.
    pub fn as_table(&self) -> Option<&BTreeMap<String, TomlValue>> {
        match self {
            Self::Table(t) => Some(t),
            _ => None,
        }
    }
    /// Try to extract as an array.
    pub fn as_array(&self) -> Option<&[TomlValue]> {
        match self {
            Self::Array(a) => Some(a),
            _ => None,
        }
    }
}
/// A simple TOML-like parser for manifest files.
pub struct ManifestParser {
    source: String,
}
impl ManifestParser {
    /// Create a new parser for the given source.
    pub fn new(source: &str) -> Self {
        Self {
            source: source.to_string(),
        }
    }
    /// Parse the source into a TOML value.
    pub fn parse(&self) -> Result<TomlValue, ManifestError> {
        let mut table = BTreeMap::new();
        let mut current_section: Option<String> = None;
        let mut section_table: BTreeMap<String, TomlValue> = BTreeMap::new();
        for line in self.source.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            if line.starts_with('[') && line.ends_with(']') {
                if let Some(ref sec) = current_section {
                    table.insert(sec.clone(), TomlValue::Table(section_table.clone()));
                    section_table.clear();
                }
                current_section = Some(line[1..line.len() - 1].trim().to_string());
                continue;
            }
            if let Some(eq_idx) = line.find('=') {
                let key = line[..eq_idx].trim().to_string();
                let val_str = line[eq_idx + 1..].trim();
                let value = Self::parse_value(val_str)?;
                if current_section.is_some() {
                    section_table.insert(key, value);
                } else {
                    table.insert(key, value);
                }
            }
        }
        if let Some(ref sec) = current_section {
            table.insert(sec.clone(), TomlValue::Table(section_table));
        }
        Ok(TomlValue::Table(table))
    }
    fn parse_value(val_str: &str) -> Result<TomlValue, ManifestError> {
        let val_str = val_str.trim();
        if val_str.starts_with('"') && val_str.ends_with('"') && val_str.len() >= 2 {
            return Ok(TomlValue::Str(val_str[1..val_str.len() - 1].to_string()));
        }
        if val_str == "true" {
            return Ok(TomlValue::Boolean(true));
        }
        if val_str == "false" {
            return Ok(TomlValue::Boolean(false));
        }
        if val_str.starts_with('[') && val_str.ends_with(']') {
            let inner = &val_str[1..val_str.len() - 1];
            if inner.trim().is_empty() {
                return Ok(TomlValue::Array(Vec::new()));
            }
            let items: Vec<TomlValue> = inner
                .split(',')
                .map(|s| Self::parse_value(s.trim()))
                .collect::<Result<_, _>>()?;
            return Ok(TomlValue::Array(items));
        }
        if let Ok(n) = val_str.parse::<i64>() {
            return Ok(TomlValue::Integer(n));
        }
        if let Ok(f) = val_str.parse::<f64>() {
            return Ok(TomlValue::Float(f));
        }
        Ok(TomlValue::Str(val_str.to_string()))
    }
}
/// A platform-specific dependency entry.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct PlatformDependency {
    /// The platform condition.
    pub condition: PlatformCondition,
    /// The dependency name.
    pub name: String,
    /// The dependency specification.
    pub dep: Dependency,
}
#[allow(dead_code)]
impl PlatformDependency {
    /// Create a new platform-specific dependency.
    pub fn new(condition: PlatformCondition, name: &str, dep: Dependency) -> Self {
        Self {
            condition,
            name: name.to_string(),
            dep,
        }
    }
    /// Whether this dependency applies for the given platform.
    pub fn applies(&self, os: &str, arch: &str, triple: &str) -> bool {
        self.condition.eval(os, arch, triple)
    }
}
/// Source of a dependency.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DependencySource {
    /// From the package registry.
    Registry {
        /// Registry URL or name.
        registry: String,
    },
    /// From a Git repository.
    Git {
        /// Repository URL.
        url: String,
        /// Branch, tag, or commit.
        reference: GitReference,
    },
    /// Local path dependency.
    Path {
        /// Path to the local package.
        path: PathBuf,
    },
}
/// Build target type.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TargetKind {
    /// A library target.
    Library,
    /// An executable target.
    Executable,
    /// A test suite.
    Test,
    /// A benchmark suite.
    Benchmark,
    /// An example.
    Example,
}
/// A collection of environment overrides for a manifest.
#[allow(dead_code)]
pub struct EnvOverrideSet {
    overrides: Vec<EnvOverride>,
}
#[allow(dead_code)]
impl EnvOverrideSet {
    /// Create a new override set.
    pub fn new() -> Self {
        Self {
            overrides: Vec::new(),
        }
    }
    /// Add an override.
    pub fn add(&mut self, o: EnvOverride) {
        self.overrides.push(o);
    }
    /// Find the override for a given field.
    pub fn for_field(&self, field: &str) -> Option<&EnvOverride> {
        self.overrides.iter().find(|o| o.field == field)
    }
    /// Resolve all overrides into a map of field → value.
    pub fn resolve_all(&self) -> BTreeMap<String, String> {
        self.overrides
            .iter()
            .filter_map(|o| o.resolve().map(|v| (o.field.clone(), v)))
            .collect()
    }
    /// Number of registered overrides.
    pub fn count(&self) -> usize {
        self.overrides.len()
    }
}
/// A condition under which a platform-specific dependency applies.
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PlatformCondition {
    /// Matches a specific OS (e.g. "linux", "windows", "macos").
    Os(String),
    /// Matches a specific architecture (e.g. "x86_64", "aarch64").
    Arch(String),
    /// Matches a specific target triple.
    Triple(String),
    /// Logical AND of conditions.
    And(Vec<PlatformCondition>),
    /// Logical OR of conditions.
    Or(Vec<PlatformCondition>),
    /// Negation of a condition.
    Not(Box<PlatformCondition>),
    /// Always true.
    Any,
}
#[allow(dead_code)]
impl PlatformCondition {
    /// Evaluate the condition for a given OS and arch.
    pub fn eval(&self, os: &str, arch: &str, triple: &str) -> bool {
        match self {
            Self::Os(s) => s == os,
            Self::Arch(s) => s == arch,
            Self::Triple(s) => s == triple,
            Self::And(conds) => conds.iter().all(|c| c.eval(os, arch, triple)),
            Self::Or(conds) => conds.iter().any(|c| c.eval(os, arch, triple)),
            Self::Not(c) => !c.eval(os, arch, triple),
            Self::Any => true,
        }
    }
    /// Shorthand for Linux condition.
    pub fn linux() -> Self {
        Self::Os("linux".to_string())
    }
    /// Shorthand for Windows condition.
    pub fn windows() -> Self {
        Self::Os("windows".to_string())
    }
    /// Shorthand for macOS condition.
    pub fn macos() -> Self {
        Self::Os("macos".to_string())
    }
    /// Shorthand for x86_64 condition.
    pub fn x86_64() -> Self {
        Self::Arch("x86_64".to_string())
    }
    /// Shorthand for aarch64 condition.
    pub fn aarch64() -> Self {
        Self::Arch("aarch64".to_string())
    }
}
/// Configuration for a workspace (multi-package project).
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct WorkspaceManifestConfig {
    /// Root manifest path.
    pub root: PathBuf,
    /// Member package paths (relative to root).
    pub members: Vec<PathBuf>,
    /// Excluded package paths.
    pub exclude: Vec<PathBuf>,
    /// Inherited workspace version.
    pub version: Option<Version>,
    /// Shared workspace dependencies.
    pub shared_deps: HashMap<String, Dependency>,
    /// Resolver version (1 or 2).
    pub resolver: u32,
}
#[allow(dead_code)]
impl WorkspaceManifestConfig {
    /// Create a new workspace configuration.
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self {
            root: root.into(),
            members: Vec::new(),
            exclude: Vec::new(),
            version: None,
            shared_deps: HashMap::new(),
            resolver: 2,
        }
    }
    /// Add a member package.
    pub fn add_member(mut self, path: impl Into<PathBuf>) -> Self {
        self.members.push(path.into());
        self
    }
    /// Exclude a path from the workspace.
    pub fn exclude(mut self, path: impl Into<PathBuf>) -> Self {
        self.exclude.push(path.into());
        self
    }
    /// Set the workspace version.
    pub fn with_version(mut self, v: Version) -> Self {
        self.version = Some(v);
        self
    }
    /// Add a shared dependency.
    pub fn add_shared_dep(mut self, name: &str, dep: Dependency) -> Self {
        self.shared_deps.insert(name.to_string(), dep);
        self
    }
    /// Number of members.
    pub fn member_count(&self) -> usize {
        self.members.len()
    }
    /// Validate: checks that member paths are non-empty and resolver is 1 or 2.
    pub fn validate(&self) -> Vec<String> {
        let mut errs = Vec::new();
        if self.resolver != 1 && self.resolver != 2 {
            errs.push(format!("invalid resolver version: {}", self.resolver));
        }
        for m in &self.members {
            if m.as_os_str().is_empty() {
                errs.push("empty member path".to_string());
            }
        }
        errs
    }
}
/// Optimization level for builds.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub enum OptLevel {
    /// No optimizations.
    #[default]
    None,
    /// Basic optimizations.
    Basic,
    /// Full optimizations.
    Full,
    /// Size-oriented optimizations.
    Size,
    /// Aggressive size-oriented optimizations.
    MinSize,
}
/// A resolved dependency in the lockfile.
#[derive(Clone, Debug)]
pub struct LockedDependency {
    /// Package name.
    pub name: String,
    /// Resolved version.
    pub version: Version,
    /// Source identifier (e.g., registry URL, git commit).
    pub source: String,
    /// Content hash for verification.
    pub checksum: Option<String>,
    /// Dependencies of this locked package.
    pub dependencies: Vec<String>,
}
