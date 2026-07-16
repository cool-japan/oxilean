//! Dependency resolver for oxilake.
//!
//! Provides:
//! - `Version` — minimal pure-Rust semver type
//! - `VersionReq` — version requirement (caret, tilde, exact, >=, wildcard, range)
//! - `Dependency` — a parsed dep (path or registry)
//! - `RegistrySource` trait + `LocalDirRegistry` + `UnsupportedRegistry`
//! - `resolve_dependencies` — the main public function, returns topo-ordered `Vec<PathBuf>`

use crate::manifest::{DepSpec, OxilakeManifest};
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::path::{Path, PathBuf};

// ─────────────────────────────────────────────────────────────────────────────
// Error type
// ─────────────────────────────────────────────────────────────────────────────

/// Errors produced by the dependency resolver.
#[derive(Debug)]
pub enum ResolverError {
    /// The manifest file at `path` was not found.
    ManifestNotFound(PathBuf),
    /// The manifest file at `path` could not be parsed.
    ManifestParseError { path: PathBuf, msg: String },
    /// A cyclic dependency was detected. `cycle` lists the package names forming the loop.
    CyclicDependency { cycle: Vec<String> },
    /// A registry dependency was requested, but the registry does not support network resolution.
    RegistryUnsupported { name: String },
    /// The resolved version does not satisfy the requirement.
    ///
    /// Emitted by `LocalDirRegistry::resolve` when the found version is outside the requested range.
    #[allow(dead_code)]
    VersionMismatch {
        name: String,
        required: String,
        found: String,
    },
}

impl fmt::Display for ResolverError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ResolverError::ManifestNotFound(p) => {
                write!(f, "manifest not found: {}", p.display())
            }
            ResolverError::ManifestParseError { path, msg } => {
                write!(f, "manifest parse error at {}: {}", path.display(), msg)
            }
            ResolverError::CyclicDependency { cycle } => {
                write!(f, "cyclic dependency detected: {}", cycle.join(" → "))
            }
            ResolverError::RegistryUnsupported { name } => {
                write!(
                    f,
                    "registry dependencies are not yet supported (dep: '{name}'). \
                     Use `path = \"../...\"` for local dependencies."
                )
            }
            ResolverError::VersionMismatch {
                name,
                required,
                found,
            } => {
                write!(
                    f,
                    "version mismatch for '{name}': required '{required}', found '{found}'"
                )
            }
        }
    }
}

impl std::error::Error for ResolverError {}

// ─────────────────────────────────────────────────────────────────────────────
// Version
// ─────────────────────────────────────────────────────────────────────────────

/// A minimal semantic version: `major.minor.patch[-pre]`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Version {
    pub major: u64,
    pub minor: u64,
    pub patch: u64,
    /// Optional pre-release tag, e.g. `"alpha"`, `"beta.1"`.
    pub pre: Option<String>,
}

impl Version {
    /// Parse a version string such as `"1.2.3"` or `"0.1.0-alpha"`.
    pub fn parse(s: &str) -> Result<Self, String> {
        let (core, pre) = if let Some(idx) = s.find('-') {
            (&s[..idx], Some(s[idx + 1..].to_string()))
        } else {
            (s, None)
        };

        let parts: Vec<&str> = core.split('.').collect();
        if parts.len() != 3 {
            return Err(format!("expected 'major.minor.patch', got '{s}'"));
        }
        let parse_part = |p: &str| {
            p.parse::<u64>()
                .map_err(|_| format!("invalid version component '{p}' in '{s}'"))
        };
        Ok(Self {
            major: parse_part(parts[0])?,
            minor: parse_part(parts[1])?,
            patch: parse_part(parts[2])?,
            pre,
        })
    }

    /// Whether this version satisfies `req`.
    ///
    /// Convenience wrapper over `VersionReq::matches`; part of the public API.
    #[allow(dead_code)]
    pub fn satisfies(&self, req: &VersionReq) -> bool {
        req.matches(self)
    }

    /// Returns `true` if this is a pre-release version.
    #[allow(dead_code)]
    fn is_pre(&self) -> bool {
        self.pre.is_some()
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Version {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        use std::cmp::Ordering::*;
        match self.major.cmp(&other.major) {
            Equal => {}
            o => return o,
        }
        match self.minor.cmp(&other.minor) {
            Equal => {}
            o => return o,
        }
        match self.patch.cmp(&other.patch) {
            Equal => {}
            o => return o,
        }
        // Pre-release versions sort *before* the release (semver spec §11.4):
        // 1.0.0-alpha < 1.0.0
        match (&self.pre, &other.pre) {
            (None, None) => std::cmp::Ordering::Equal,
            (Some(_), None) => std::cmp::Ordering::Less,
            (None, Some(_)) => std::cmp::Ordering::Greater,
            (Some(a), Some(b)) => a.cmp(b),
        }
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)?;
        if let Some(pre) = &self.pre {
            write!(f, "-{pre}")?;
        }
        Ok(())
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// VersionReq
// ─────────────────────────────────────────────────────────────────────────────

/// A version requirement expression.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VersionReq {
    /// `"*"` or `""` — matches any version.
    Wildcard,
    /// `"= 1.2.3"` — matches exactly.
    Exact(Version),
    /// `"^1.2.3"` — compatible release: `>=1.2.3, <2.0.0` (standard semver caret).
    Caret(Version),
    /// `"~1.2.3"` — patch-level changes allowed: `>=1.2.3, <1.3.0`.
    Tilde(Version),
    /// `">=1.2.3"` — at least this version.
    Gte(Version),
    /// `">= A, < B"` — half-open range.
    Range(Version, Version),
}

impl VersionReq {
    /// Parse a version requirement string.
    ///
    /// Supported forms:
    /// - `"*"` or `""` → `Wildcard`
    /// - `"^1.2.3"` → `Caret`
    /// - `"~1.2.3"` → `Tilde`
    /// - `"= 1.2.3"` or `"1.2.3"` → `Exact`
    /// - `">=1.2.3"` → `Gte`
    /// - `">=A, <B"` → `Range`
    pub fn parse(s: &str) -> Result<Self, String> {
        let s = s.trim();
        if s.is_empty() || s == "*" {
            return Ok(VersionReq::Wildcard);
        }

        // Range: contains a comma → ">=A, <B"
        if s.contains(',') {
            return Self::parse_range(s);
        }

        if let Some(rest) = s.strip_prefix('^') {
            return Version::parse(rest.trim()).map(VersionReq::Caret);
        }
        if let Some(rest) = s.strip_prefix('~') {
            return Version::parse(rest.trim()).map(VersionReq::Tilde);
        }
        if let Some(rest) = s.strip_prefix(">=") {
            return Version::parse(rest.trim()).map(VersionReq::Gte);
        }
        if let Some(rest) = s.strip_prefix('=') {
            return Version::parse(rest.trim()).map(VersionReq::Exact);
        }

        // Plain version string → treat as exact.
        Version::parse(s).map(VersionReq::Exact)
    }

    fn parse_range(s: &str) -> Result<Self, String> {
        let parts: Vec<&str> = s.splitn(2, ',').collect();
        if parts.len() != 2 {
            return Err(format!("invalid range requirement '{s}'"));
        }
        let lo_str = parts[0].trim();
        let hi_str = parts[1].trim();

        let lo = if let Some(rest) = lo_str.strip_prefix(">=") {
            Version::parse(rest.trim())?
        } else {
            return Err(format!(
                "range lower bound must start with '>=', got '{lo_str}'"
            ));
        };
        let hi = if let Some(rest) = hi_str.strip_prefix('<') {
            Version::parse(rest.trim())?
        } else {
            return Err(format!(
                "range upper bound must start with '<', got '{hi_str}'"
            ));
        };
        Ok(VersionReq::Range(lo, hi))
    }

    /// Returns `true` if `v` satisfies this requirement.
    ///
    /// Pre-release versions are only matched when the requirement itself
    /// specifies a pre-release on the matching bound (conservative approach).
    ///
    /// Part of the public API; called from tests and from `Version::satisfies`.
    #[allow(dead_code)]
    pub fn matches(&self, v: &Version) -> bool {
        match self {
            VersionReq::Wildcard => true,
            VersionReq::Exact(req) => v == req,
            VersionReq::Caret(base) => Self::matches_caret(v, base),
            VersionReq::Tilde(base) => Self::matches_tilde(v, base),
            VersionReq::Gte(base) => {
                if v.is_pre() && !base.is_pre() {
                    return false;
                }
                v >= base
            }
            VersionReq::Range(lo, hi) => {
                if v.is_pre() && !lo.is_pre() {
                    return false;
                }
                v >= lo && v < hi
            }
        }
    }

    /// Caret semantics: `^X.Y.Z` allows changes that do not modify the
    /// left-most non-zero digit.
    ///
    /// - `^1.2.3` → `>=1.2.3, <2.0.0`
    /// - `^0.2.3` → `>=0.2.3, <0.3.0`
    /// - `^0.0.3` → `>=0.0.3, <0.0.4`
    #[allow(dead_code)]
    fn matches_caret(v: &Version, base: &Version) -> bool {
        if v.is_pre() && !base.is_pre() {
            return false;
        }
        if v < base {
            return false;
        }
        if base.major > 0 {
            v.major == base.major
        } else if base.minor > 0 {
            v.major == 0 && v.minor == base.minor
        } else {
            v.major == 0 && v.minor == 0 && v.patch == base.patch
        }
    }

    /// Tilde semantics: `~X.Y.Z` → `>=X.Y.Z, <X.(Y+1).0`.
    #[allow(dead_code)]
    fn matches_tilde(v: &Version, base: &Version) -> bool {
        if v.is_pre() && !base.is_pre() {
            return false;
        }
        v >= base && v.major == base.major && v.minor == base.minor
    }
}

impl fmt::Display for VersionReq {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VersionReq::Wildcard => write!(f, "*"),
            VersionReq::Exact(v) => write!(f, "={v}"),
            VersionReq::Caret(v) => write!(f, "^{v}"),
            VersionReq::Tilde(v) => write!(f, "~{v}"),
            VersionReq::Gte(v) => write!(f, ">={v}"),
            VersionReq::Range(lo, hi) => write!(f, ">={lo}, <{hi}"),
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Dependency
// ─────────────────────────────────────────────────────────────────────────────

/// A resolved dependency declaration.
#[derive(Debug, Clone)]
pub enum Dependency {
    /// A local path dependency, optionally with a version requirement.
    Path {
        path: PathBuf,
        /// Reserved for future version pinning of path deps.
        #[allow(dead_code)]
        version_req: Option<VersionReq>,
    },
    /// A registry (crates.io-style) dependency.
    Registry {
        name: String,
        version_req: VersionReq,
    },
}

impl Dependency {
    /// Parse a `DepSpec` from the manifest into a `Dependency`.
    ///
    /// `base_dir` is the directory containing the manifest that declares this dep.
    pub fn from_spec(name: &str, spec: &DepSpec, base_dir: &Path) -> Result<Self, String> {
        match spec {
            DepSpec::Version(v) => {
                let req = VersionReq::parse(v)?;
                Ok(Dependency::Registry {
                    name: name.to_string(),
                    version_req: req,
                })
            }
            DepSpec::Path { path } => {
                let dep_path = if Path::new(path).is_absolute() {
                    PathBuf::from(path)
                } else {
                    base_dir.join(path)
                };
                Ok(Dependency::Path {
                    path: dep_path,
                    version_req: None,
                })
            }
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// RegistrySource trait + impls
// ─────────────────────────────────────────────────────────────────────────────

/// A source that can resolve a package name + version requirement to a concrete version.
pub trait RegistrySource {
    /// Resolve `name` against `req`, returning the best matching `Version`.
    fn resolve(&self, name: &str, req: &VersionReq) -> Result<Version, ResolverError>;
}

/// A registry backed by a local directory of `oxilake.toml` manifests.
///
/// The directory is expected to contain one subdirectory per package, each
/// holding an `oxilake.toml` with the package's metadata.
///
/// This type is part of the public API; it is not yet used in the CLI but is
/// available for embedders and future workspace tooling.
#[allow(dead_code)]
pub struct LocalDirRegistry {
    pub root: PathBuf,
}

impl RegistrySource for LocalDirRegistry {
    fn resolve(&self, name: &str, req: &VersionReq) -> Result<Version, ResolverError> {
        // Scan subdirectories for a matching package.
        let pkg_dir = self.root.join(name);
        let manifest_path = pkg_dir.join("oxilake.toml");
        if !manifest_path.exists() {
            return Err(ResolverError::ManifestNotFound(manifest_path));
        }
        let manifest = OxilakeManifest::load(&manifest_path).map_err(|e| {
            ResolverError::ManifestParseError {
                path: manifest_path.clone(),
                msg: e.to_string(),
            }
        })?;
        let v = Version::parse(&manifest.package.version).map_err(|e| {
            ResolverError::ManifestParseError {
                path: manifest_path,
                msg: e,
            }
        })?;
        if req.matches(&v) {
            Ok(v)
        } else {
            Err(ResolverError::VersionMismatch {
                name: name.to_string(),
                required: req.to_string(),
                found: v.to_string(),
            })
        }
    }
}

/// A stub registry that always returns `RegistryUnsupported`.
///
/// Use this when network-based package resolution is not yet supported.
pub struct UnsupportedRegistry;

impl RegistrySource for UnsupportedRegistry {
    fn resolve(&self, name: &str, _req: &VersionReq) -> Result<Version, ResolverError> {
        Err(ResolverError::RegistryUnsupported {
            name: name.to_string(),
        })
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// DFS / topological sort
// ─────────────────────────────────────────────────────────────────────────────

/// Internal node state for DFS.
///
/// `White` (absent from the map) means "not yet visited". The variant is kept
/// for semantic completeness but nodes start as absent rather than `White`.
#[derive(PartialEq, Eq)]
#[allow(dead_code)]
enum NodeColor {
    White, // not yet visited
    Gray,  // currently in the DFS stack (in progress)
    Black, // fully processed
}

/// Recursive DFS that builds the topologically-ordered list and detects cycles.
///
/// `path` is the canonical path of the manifest being processed.
/// `stack` holds the current DFS path (as canonical manifest paths) for cycle reporting.
fn dfs(
    manifest_path: &Path,
    registry: &dyn RegistrySource,
    color: &mut HashMap<PathBuf, NodeColor>,
    stack: &mut Vec<PathBuf>,
    result: &mut Vec<PathBuf>,
) -> Result<(), ResolverError> {
    let canonical = canonical_or_self(manifest_path);

    // Already fully processed → skip.
    if color.get(&canonical) == Some(&NodeColor::Black) {
        return Ok(());
    }

    // Currently in the stack → cycle!
    if color.get(&canonical) == Some(&NodeColor::Gray) {
        let cycle = build_cycle_names(&canonical, stack);
        return Err(ResolverError::CyclicDependency { cycle });
    }

    // Check existence before marking gray.
    if !manifest_path.exists() {
        return Err(ResolverError::ManifestNotFound(manifest_path.to_path_buf()));
    }

    color.insert(canonical.clone(), NodeColor::Gray);
    stack.push(canonical.clone());

    // Load the manifest.
    let manifest =
        OxilakeManifest::load(manifest_path).map_err(|e| ResolverError::ManifestParseError {
            path: manifest_path.to_path_buf(),
            msg: e.to_string(),
        })?;

    let base_dir = manifest_path.parent().unwrap_or_else(|| Path::new("."));

    // Process each dependency.
    for (dep_name, dep_spec) in &manifest.dependencies {
        let dep = Dependency::from_spec(dep_name, dep_spec, base_dir).map_err(|e| {
            ResolverError::ManifestParseError {
                path: manifest_path.to_path_buf(),
                msg: e,
            }
        })?;

        match dep {
            Dependency::Path { path: dep_path, .. } => {
                let dep_manifest = dep_path.join("oxilake.toml");
                dfs(&dep_manifest, registry, color, stack, result)?;
            }
            Dependency::Registry { name, version_req } => {
                // Delegate to the registry — UnsupportedRegistry will surface an error.
                registry.resolve(&name, &version_req)?;
                // Registry deps are treated as leaves; no further recursion.
            }
        }
    }

    stack.pop();
    color.insert(canonical, NodeColor::Black);

    // Add to result only if it's not the root (we only want dep paths, not the root itself).
    // We'll add the root's dir separately in `resolve_dependencies`.
    result.push(manifest_path.to_path_buf());

    Ok(())
}

/// Convert a stack of canonical paths into a cycle description (package names or paths).
fn build_cycle_names(culprit: &Path, stack: &[PathBuf]) -> Vec<String> {
    // Find where the cycle starts in the stack.
    let start = stack.iter().position(|p| p == culprit).unwrap_or(0);
    let mut names: Vec<String> = stack[start..]
        .iter()
        .map(|p| path_to_package_name(p))
        .collect();
    // Close the cycle.
    names.push(path_to_package_name(culprit));
    names
}

/// Best-effort package name from a manifest path.
fn path_to_package_name(manifest_path: &Path) -> String {
    manifest_path
        .parent()
        .and_then(|p| p.file_name())
        .and_then(|n| n.to_str())
        .map(|s| s.to_string())
        .unwrap_or_else(|| manifest_path.display().to_string())
}

/// Canonicalize a path, falling back to the original on error.
fn canonical_or_self(p: &Path) -> PathBuf {
    std::fs::canonicalize(p).unwrap_or_else(|_| p.to_path_buf())
}

// ─────────────────────────────────────────────────────────────────────────────
// Public entry point
// ─────────────────────────────────────────────────────────────────────────────

/// Resolve all dependencies of the package described by `manifest_path`.
///
/// Returns a topologically-ordered list of *manifest paths* (the `oxilake.toml`
/// files of each dependency), with dependencies before the packages that need
/// them. The root manifest itself is **included** at the end so callers can
/// iterate the slice and build in order.
///
/// # Errors
/// - `ResolverError::ManifestNotFound` — a dep's manifest was missing
/// - `ResolverError::ManifestParseError` — a manifest could not be parsed
/// - `ResolverError::CyclicDependency` — a cycle was detected in the graph
/// - `ResolverError::RegistryUnsupported` — a registry dep was encountered with `UnsupportedRegistry`
pub fn resolve_dependencies(
    manifest_path: &Path,
    registry: &dyn RegistrySource,
) -> Result<Vec<PathBuf>, ResolverError> {
    if !manifest_path.exists() {
        return Err(ResolverError::ManifestNotFound(manifest_path.to_path_buf()));
    }

    let mut color: HashMap<PathBuf, NodeColor> = HashMap::new();
    let mut stack: Vec<PathBuf> = Vec::new();
    let mut result: Vec<PathBuf> = Vec::new();

    dfs(manifest_path, registry, &mut color, &mut stack, &mut result)?;

    // `result` now contains all manifests (including root) in topo order.
    // Convert each manifest path to the project root directory (the parent dir).
    let dirs: Vec<PathBuf> = result
        .iter()
        .map(|p| {
            p.parent()
                .map(|d| d.to_path_buf())
                .unwrap_or_else(|| PathBuf::from("."))
        })
        .collect();

    Ok(dirs)
}

/// Deduplicate a topo-ordered list while preserving order.
///
/// When a package appears as a dep of multiple packages, the DFS will visit it
/// once (Black → skip), so duplicates should not arise in practice. This guard
/// removes any that slip through.
#[allow(dead_code)]
pub fn dedup_topo(paths: Vec<PathBuf>) -> Vec<PathBuf> {
    let mut seen: HashSet<PathBuf> = HashSet::new();
    paths
        .into_iter()
        .filter(|p| seen.insert(p.clone()))
        .collect()
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs;

    /// Write a minimal `oxilake.toml` to `dir/oxilake.toml`.
    fn write_manifest_simple(dir: &Path, name: &str, version: &str) {
        fs::create_dir_all(dir).expect("create dir");
        fs::write(
            dir.join("oxilake.toml"),
            format!(
                "[package]\nname = \"{name}\"\nversion = \"{version}\"\nlean_version = \"0.1\"\n"
            ),
        )
        .expect("write oxilake.toml");
    }

    /// Write a manifest with one path dependency.
    fn write_manifest_with_path_dep(
        dir: &Path,
        name: &str,
        version: &str,
        dep_name: &str,
        dep_rel_path: &str,
    ) {
        fs::create_dir_all(dir).expect("create dir");
        fs::write(
            dir.join("oxilake.toml"),
            format!(
                "[package]\nname = \"{name}\"\nversion = \"{version}\"\nlean_version = \"0.1\"\n\
                 [dependencies]\n{dep_name} = {{ path = \"{dep_rel_path}\" }}\n"
            ),
        )
        .expect("write oxilake.toml");
    }

    // ─── 1. Version parse and ordering ───────────────────────────────────────

    #[test]
    fn test_version_parse_and_ord() {
        let v123 = Version::parse("1.2.3").expect("parse 1.2.3");
        let v010 = Version::parse("0.1.0").expect("parse 0.1.0");
        let v_alpha = Version::parse("0.1.0-alpha").expect("parse 0.1.0-alpha");

        assert_eq!(v123.major, 1);
        assert_eq!(v123.minor, 2);
        assert_eq!(v123.patch, 3);
        assert!(v123.pre.is_none());

        assert_eq!(v_alpha.major, 0);
        assert_eq!(v_alpha.minor, 1);
        assert_eq!(v_alpha.patch, 0);
        assert_eq!(v_alpha.pre.as_deref(), Some("alpha"));

        // Ordering
        assert!(v010 < v123);
        assert!(v123 > v010);

        // Pre-release < release with same core
        assert!(v_alpha < v010, "0.1.0-alpha should be less than 0.1.0");

        // Equality
        let v010b = Version::parse("0.1.0").expect("parse 0.1.0 again");
        assert_eq!(v010, v010b);

        // Display
        assert_eq!(v123.to_string(), "1.2.3");
        assert_eq!(v_alpha.to_string(), "0.1.0-alpha");

        // Parse error
        assert!(Version::parse("1.2").is_err());
        assert!(Version::parse("not-a-version").is_err());
        assert!(Version::parse("1.2.x").is_err());
    }

    // ─── 2. VersionReq caret ─────────────────────────────────────────────────

    #[test]
    fn test_version_req_caret() {
        let req = VersionReq::parse("^1.2.3").expect("parse ^1.2.3");

        // Must be >= 1.2.3
        assert!(!req.matches(&Version::parse("1.2.2").unwrap()));
        assert!(req.matches(&Version::parse("1.2.3").unwrap()));
        assert!(req.matches(&Version::parse("1.3.0").unwrap()));
        assert!(req.matches(&Version::parse("1.9.9").unwrap()));

        // Must be < 2.0.0
        assert!(!req.matches(&Version::parse("2.0.0").unwrap()));
        assert!(!req.matches(&Version::parse("2.1.0").unwrap()));

        // ^0.2.3 → >=0.2.3, <0.3.0
        let req02 = VersionReq::parse("^0.2.3").expect("parse ^0.2.3");
        assert!(!req02.matches(&Version::parse("0.2.2").unwrap()));
        assert!(req02.matches(&Version::parse("0.2.3").unwrap()));
        assert!(req02.matches(&Version::parse("0.2.9").unwrap()));
        assert!(!req02.matches(&Version::parse("0.3.0").unwrap()));

        // ^0.0.3 → exactly 0.0.3
        let req003 = VersionReq::parse("^0.0.3").expect("parse ^0.0.3");
        assert!(req003.matches(&Version::parse("0.0.3").unwrap()));
        assert!(!req003.matches(&Version::parse("0.0.4").unwrap()));

        // Wildcard
        let wild = VersionReq::parse("*").expect("parse *");
        assert!(wild.matches(&Version::parse("99.99.99").unwrap()));

        // Caret display
        assert_eq!(req.to_string(), "^1.2.3");
    }

    // ─── 3. VersionReq tilde ─────────────────────────────────────────────────

    #[test]
    fn test_version_req_tilde() {
        let req = VersionReq::parse("~1.2.3").expect("parse ~1.2.3");

        // Must be >= 1.2.3 and < 1.3.0
        assert!(!req.matches(&Version::parse("1.2.2").unwrap()));
        assert!(req.matches(&Version::parse("1.2.3").unwrap()));
        assert!(req.matches(&Version::parse("1.2.5").unwrap()));
        assert!(req.matches(&Version::parse("1.2.9").unwrap()));

        // Must NOT match 1.3.0 or higher
        assert!(!req.matches(&Version::parse("1.3.0").unwrap()));
        assert!(!req.matches(&Version::parse("2.0.0").unwrap()));

        // Tilde display
        assert_eq!(req.to_string(), "~1.2.3");

        // Exact requirement
        let exact = VersionReq::parse("= 1.2.3").expect("parse =1.2.3");
        assert!(exact.matches(&Version::parse("1.2.3").unwrap()));
        assert!(!exact.matches(&Version::parse("1.2.4").unwrap()));

        // >= requirement
        let gte = VersionReq::parse(">=1.0.0").expect("parse >=1.0.0");
        assert!(gte.matches(&Version::parse("1.0.0").unwrap()));
        assert!(gte.matches(&Version::parse("2.5.0").unwrap()));
        assert!(!gte.matches(&Version::parse("0.9.9").unwrap()));

        // Range requirement
        let range = VersionReq::parse(">=1.0.0, <2.0.0").expect("parse range");
        assert!(range.matches(&Version::parse("1.0.0").unwrap()));
        assert!(range.matches(&Version::parse("1.9.9").unwrap()));
        assert!(!range.matches(&Version::parse("2.0.0").unwrap()));
        assert!(!range.matches(&Version::parse("0.9.9").unwrap()));
    }

    // ─── 4. Path dep topo order ──────────────────────────────────────────────

    #[test]
    fn test_path_dep_topo_order() {
        // Layout: a → b → c (c has no deps)
        // Expected resolve order: [c, b, a]  (deps before the package)
        let base = env::temp_dir().join("oxilake_resolver_topo_001");
        fs::remove_dir_all(&base).ok();

        let dir_c = base.join("c");
        let dir_b = base.join("b");
        let dir_a = base.join("a");

        write_manifest_simple(&dir_c, "pkg-c", "0.1.0");
        write_manifest_with_path_dep(&dir_b, "pkg-b", "0.1.0", "pkg-c", "../c");
        write_manifest_with_path_dep(&dir_a, "pkg-a", "0.1.0", "pkg-b", "../b");

        let registry = UnsupportedRegistry;
        let result = resolve_dependencies(&dir_a.join("oxilake.toml"), &registry).expect("resolve");

        // result is directory paths in topo order
        assert_eq!(result.len(), 3, "expected 3 packages in result");

        // c should come before b, b before a
        let names: Vec<String> = result
            .iter()
            .map(|p| p.file_name().unwrap().to_str().unwrap().to_string())
            .collect();

        let pos_c = names.iter().position(|n| n == "c").expect("c in result");
        let pos_b = names.iter().position(|n| n == "b").expect("b in result");
        let pos_a = names.iter().position(|n| n == "a").expect("a in result");

        assert!(pos_c < pos_b, "c must come before b (c={pos_c}, b={pos_b})");
        assert!(pos_b < pos_a, "b must come before a (b={pos_b}, a={pos_a})");

        fs::remove_dir_all(&base).ok();
    }

    // ─── 5. Cycle detection ──────────────────────────────────────────────────

    #[test]
    fn test_cycle_detection() {
        // Layout: a → b → a (cycle)
        let base = env::temp_dir().join("oxilake_resolver_cycle_002");
        fs::remove_dir_all(&base).ok();

        let dir_a = base.join("a");
        let dir_b = base.join("b");

        write_manifest_with_path_dep(&dir_a, "pkg-a", "0.1.0", "pkg-b", "../b");
        write_manifest_with_path_dep(&dir_b, "pkg-b", "0.1.0", "pkg-a", "../a");

        let registry = UnsupportedRegistry;
        let result = resolve_dependencies(&dir_a.join("oxilake.toml"), &registry);

        assert!(result.is_err(), "cycle should be detected");
        let err = result.unwrap_err();
        match err {
            ResolverError::CyclicDependency { cycle } => {
                assert!(!cycle.is_empty(), "cycle description should not be empty");
            }
            other => panic!("expected CyclicDependency, got: {other}"),
        }

        fs::remove_dir_all(&base).ok();
    }

    // ─── 6. Missing dep ──────────────────────────────────────────────────────

    #[test]
    fn test_missing_dep() {
        let base = env::temp_dir().join("oxilake_resolver_missing_003");
        fs::remove_dir_all(&base).ok();

        let dir_a = base.join("a");
        // a depends on "missing" which does not exist
        write_manifest_with_path_dep(&dir_a, "pkg-a", "0.1.0", "missing", "../missing");

        let registry = UnsupportedRegistry;
        let result = resolve_dependencies(&dir_a.join("oxilake.toml"), &registry);

        assert!(result.is_err(), "missing dep should cause an error");
        let err = result.unwrap_err();
        match err {
            ResolverError::ManifestNotFound(_) => {}
            other => panic!("expected ManifestNotFound, got: {other}"),
        }

        fs::remove_dir_all(&base).ok();
    }

    // ─── 7. Registry unsupported ─────────────────────────────────────────────

    #[test]
    fn test_registry_unsupported() {
        let base = env::temp_dir().join("oxilake_resolver_registry_004");
        fs::remove_dir_all(&base).ok();

        let dir_a = base.join("a");
        fs::create_dir_all(&dir_a).expect("create dir");
        // a depends on a registry package "some-lib"
        fs::write(
            dir_a.join("oxilake.toml"),
            "[package]\nname = \"pkg-a\"\nversion = \"0.1.0\"\nlean_version = \"0.1\"\n\
             [dependencies]\nsome-lib = \"^1.0.0\"\n",
        )
        .expect("write manifest");

        let registry = UnsupportedRegistry;
        let result = resolve_dependencies(&dir_a.join("oxilake.toml"), &registry);

        assert!(
            result.is_err(),
            "registry dep should fail with UnsupportedRegistry"
        );
        let err = result.unwrap_err();
        match err {
            ResolverError::RegistryUnsupported { name } => {
                assert_eq!(name, "some-lib");
            }
            other => panic!("expected RegistryUnsupported, got: {other}"),
        }

        fs::remove_dir_all(&base).ok();
    }

    // ─── Extra: empty deps (root-only) ───────────────────────────────────────

    #[test]
    fn test_resolve_no_deps() {
        let base = env::temp_dir().join("oxilake_resolver_nodeps_005");
        fs::remove_dir_all(&base).ok();

        let dir_a = base.join("a");
        write_manifest_simple(&dir_a, "pkg-a", "0.1.0");

        let registry = UnsupportedRegistry;
        let result = resolve_dependencies(&dir_a.join("oxilake.toml"), &registry).expect("resolve");

        // Should return just the root package directory.
        assert_eq!(result.len(), 1);
        let name = result[0].file_name().unwrap().to_str().unwrap();
        assert_eq!(name, "a");

        fs::remove_dir_all(&base).ok();
    }

    // ─── Extra: deep diamond dep (a → b, a → c, b → d, c → d) ──────────────

    #[test]
    fn test_diamond_dep_no_cycle() {
        let base = env::temp_dir().join("oxilake_resolver_diamond_006");
        fs::remove_dir_all(&base).ok();

        let dir_d = base.join("d");
        let dir_b = base.join("b");
        let dir_c = base.join("c");
        let dir_a = base.join("a");

        write_manifest_simple(&dir_d, "pkg-d", "0.1.0");
        write_manifest_with_path_dep(&dir_b, "pkg-b", "0.1.0", "pkg-d", "../d");
        write_manifest_with_path_dep(&dir_c, "pkg-c", "0.1.0", "pkg-d", "../d");

        // a depends on both b and c
        fs::create_dir_all(&dir_a).expect("create dir");
        fs::write(
            dir_a.join("oxilake.toml"),
            "[package]\nname = \"pkg-a\"\nversion = \"0.1.0\"\nlean_version = \"0.1\"\n\
             [dependencies]\npkg-b = { path = \"../b\" }\npkg-c = { path = \"../c\" }\n",
        )
        .expect("write oxilake.toml");

        let registry = UnsupportedRegistry;
        let result = resolve_dependencies(&dir_a.join("oxilake.toml"), &registry).expect("resolve");

        // d should appear only once (the second visit is skipped as Black).
        let names: Vec<String> = result
            .iter()
            .map(|p| p.file_name().unwrap().to_str().unwrap().to_string())
            .collect();

        let d_count = names.iter().filter(|n| n.as_str() == "d").count();
        assert_eq!(d_count, 1, "d should appear exactly once, got: {names:?}");

        // a must be last.
        assert_eq!(names.last().unwrap().as_str(), "a");

        fs::remove_dir_all(&base).ok();
    }
}
