//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::manifest::{Dependency, DependencySource, Manifest, Version, VersionConstraint};
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};

use super::functions::PackageSource;

/// Cache for resolved package versions.
#[allow(dead_code)]
pub struct ResolutionCache {
    entries: HashMap<String, Version>,
    hit_count: u64,
    miss_count: u64,
}
#[allow(dead_code)]
impl ResolutionCache {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
            hit_count: 0,
            miss_count: 0,
        }
    }
    pub fn get(&mut self, package: &str) -> Option<&Version> {
        if let Some(v) = self.entries.get(package) {
            self.hit_count += 1;
            Some(v)
        } else {
            self.miss_count += 1;
            None
        }
    }
    pub fn insert(&mut self, package: &str, version: Version) {
        self.entries.insert(package.to_string(), version);
    }
    pub fn invalidate(&mut self, package: &str) -> bool {
        self.entries.remove(package).is_some()
    }
    pub fn clear(&mut self) {
        self.entries.clear();
    }
    pub fn hit_ratio(&self) -> f64 {
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
/// A summary of an available package version.
#[derive(Clone, Debug)]
pub struct PackageSummary {
    /// Package name.
    pub name: String,
    /// Available version.
    pub version: Version,
    /// Dependencies of this version.
    pub dependencies: Vec<Dependency>,
    /// Available features.
    pub features: BTreeSet<String>,
    /// Whether this version is yanked.
    pub yanked: bool,
}
/// Unique identifier for a resolved package.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PackageId {
    /// Package name.
    pub name: String,
    /// Resolved version.
    pub version: Version,
    /// Source identifier.
    pub source: String,
}
impl PackageId {
    /// Create a new package ID.
    pub fn new(name: &str, version: Version, source: &str) -> Self {
        Self {
            name: name.to_string(),
            version,
            source: source.to_string(),
        }
    }
    /// Create a package ID for a root package.
    pub fn root(name: &str, version: Version) -> Self {
        Self::new(name, version, "root")
    }
}
/// A set of version ranges, representing a union of ranges.
#[derive(Clone, Debug)]
pub struct VersionSet {
    /// Ranges in the set (union).
    pub(super) ranges: Vec<VersionRange>,
}
impl VersionSet {
    /// Create an empty version set (matches nothing).
    pub fn empty() -> Self {
        Self { ranges: Vec::new() }
    }
    /// Create a universal version set (matches everything).
    pub fn universal() -> Self {
        Self {
            ranges: vec![VersionRange::any()],
        }
    }
    /// Create a version set from a single range.
    pub fn from_range(range: VersionRange) -> Self {
        Self {
            ranges: vec![range],
        }
    }
    /// Create a version set from a constraint.
    pub fn from_constraint(constraint: &VersionConstraint) -> Self {
        Self {
            ranges: VersionRange::from_constraint(constraint),
        }
    }
    /// Check if a version is in the set.
    pub fn contains(&self, version: &Version) -> bool {
        self.ranges.iter().any(|r| r.contains(version))
    }
    /// Compute the intersection of two version sets.
    pub fn intersect(&self, other: &Self) -> Self {
        let mut ranges = Vec::new();
        for r1 in &self.ranges {
            for r2 in &other.ranges {
                if let Some(intersection) = r1.intersect(r2) {
                    ranges.push(intersection);
                }
            }
        }
        Self { ranges }
    }
    /// Compute the union of two version sets.
    pub fn union(&self, other: &Self) -> Self {
        let mut ranges = self.ranges.clone();
        ranges.extend(other.ranges.clone());
        Self { ranges }
    }
    /// Check if the set is empty (matches nothing).
    pub fn is_empty(&self) -> bool {
        self.ranges.is_empty() || self.ranges.iter().all(|r| r.is_empty())
    }
    /// Get the number of ranges in the set.
    pub fn range_count(&self) -> usize {
        self.ranges.len()
    }
    /// Select the best version from a list of candidates.
    pub fn best_match<'a>(&self, candidates: &'a [Version]) -> Option<&'a Version> {
        let mut matching: Vec<&Version> = candidates.iter().filter(|v| self.contains(v)).collect();
        matching.sort();
        matching.last().copied()
    }
}
/// An incompatibility (set of terms that cannot all be true).
#[derive(Clone, Debug)]
pub struct Incompatibility {
    /// Terms in the incompatibility (package -> version set that is NOT allowed).
    pub terms: BTreeMap<String, VersionSet>,
    /// The cause of this incompatibility.
    pub cause: IncompatibilityCause,
}
impl Incompatibility {
    /// Create a new incompatibility.
    pub fn new(terms: BTreeMap<String, VersionSet>, cause: IncompatibilityCause) -> Self {
        Self { terms, cause }
    }
    /// Check if this incompatibility is satisfied (all terms match) given
    /// the current partial solution.
    pub fn is_satisfied(&self, decided: &HashMap<String, Version>) -> bool {
        for (pkg, vset) in &self.terms {
            match decided.get(pkg) {
                Some(v) => {
                    if !vset.contains(v) {
                        return false;
                    }
                }
                None => return false,
            }
        }
        true
    }
    /// Check if all terms but one are satisfied.
    pub fn almost_satisfied(&self, decided: &HashMap<String, Version>) -> Option<String> {
        let mut unsatisfied = Vec::new();
        for (pkg, vset) in &self.terms {
            match decided.get(pkg) {
                Some(v) => {
                    if !vset.contains(v) {
                        return None;
                    }
                }
                None => unsatisfied.push(pkg.clone()),
            }
        }
        if unsatisfied.len() == 1 {
            // Safety: len() == 1 guaranteed by the if condition above
            Some(
                unsatisfied
                    .into_iter()
                    .next()
                    .expect("unsatisfied has exactly one element"),
            )
        } else {
            None
        }
    }
    /// Get all packages mentioned in this incompatibility.
    pub fn packages(&self) -> Vec<&str> {
        self.terms.keys().map(|s| s.as_str()).collect()
    }
}
/// An edge in the dependency graph.
#[derive(Clone, Debug)]
pub struct DepEdge {
    /// The source package.
    pub from: PackageId,
    /// The target package.
    pub to: PackageId,
    /// The version constraint on the edge.
    pub constraint: VersionConstraint,
    /// Whether the dependency is optional.
    pub optional: bool,
    /// Features requested for the target.
    pub features: Vec<String>,
}
/// An in-memory package source for testing.
pub struct MemoryPackageSource {
    pub(super) packages: HashMap<String, Vec<PackageSummary>>,
}
impl MemoryPackageSource {
    /// Create a new empty source.
    pub fn new() -> Self {
        Self {
            packages: HashMap::new(),
        }
    }
    /// Add a package summary.
    pub fn add_summary(&mut self, summary: PackageSummary) {
        self.packages
            .entry(summary.name.clone())
            .or_default()
            .push(summary);
    }
}
/// The partial solution being built up by the solver.
struct PartialSolution {
    assignments: Vec<Assignment>,
    decision_level: u32,
    decided: HashMap<String, Version>,
    constraints: HashMap<String, Vec<VersionConstraint>>,
}
impl PartialSolution {
    fn new() -> Self {
        Self {
            assignments: Vec::new(),
            decision_level: 0,
            decided: HashMap::new(),
            constraints: HashMap::new(),
        }
    }
    fn add_decision(&mut self, package: &str, version: Version) {
        self.decision_level += 1;
        let dl = self.decision_level;
        self.decided.insert(package.to_string(), version.clone());
        self.assignments.push(Assignment::Decision {
            package: package.to_string(),
            version,
            decision_level: dl,
        });
    }
    fn add_derivation(&mut self, package: &str, constraint: VersionConstraint, cause: &str) {
        let dl = self.decision_level;
        self.constraints
            .entry(package.to_string())
            .or_default()
            .push(constraint.clone());
        self.assignments.push(Assignment::Derivation {
            package: package.to_string(),
            constraint,
            cause: cause.to_string(),
            decision_level: dl,
        });
    }
    fn is_decided(&self, package: &str) -> bool {
        self.decided.contains_key(package)
    }
    fn get_decision(&self, package: &str) -> Option<&Version> {
        self.decided.get(package)
    }
    fn effective_constraint(&self, package: &str) -> VersionConstraint {
        match self.constraints.get(package) {
            Some(cs) if !cs.is_empty() => {
                let mut result = cs[0].clone();
                for c in &cs[1..] {
                    result = result.intersect(c.clone());
                }
                result
            }
            _ => VersionConstraint::Any,
        }
    }
    fn backtrack_to(&mut self, level: u32) {
        self.assignments.retain(|a| {
            let dl = match a {
                Assignment::Decision { decision_level, .. } => *decision_level,
                Assignment::Derivation { decision_level, .. } => *decision_level,
            };
            dl <= level
        });
        self.decided.retain(|_k, _v| true);
        let mut new_decided = HashMap::new();
        let mut new_constraints: HashMap<String, Vec<VersionConstraint>> = HashMap::new();
        for a in &self.assignments {
            match a {
                Assignment::Decision {
                    package,
                    version,
                    decision_level,
                } => {
                    if *decision_level <= level {
                        new_decided.insert(package.clone(), version.clone());
                    }
                }
                Assignment::Derivation {
                    package,
                    constraint,
                    decision_level,
                    ..
                } => {
                    if *decision_level <= level {
                        new_constraints
                            .entry(package.clone())
                            .or_default()
                            .push(constraint.clone());
                    }
                }
            }
        }
        self.decided = new_decided;
        self.constraints = new_constraints;
        self.decision_level = level;
    }
}
/// A step in the resolution plan.
#[derive(Clone, Debug)]
pub enum ResolutionStep {
    /// Download a package from the registry.
    Download {
        /// Package to download.
        package: PackageId,
        /// Registry URL.
        registry_url: String,
    },
    /// Clone a git repository.
    GitClone {
        /// Package to clone.
        package: PackageId,
        /// Repository URL.
        url: String,
        /// Git reference.
        reference: String,
    },
    /// Link a local path dependency.
    LinkLocal {
        /// Package to link.
        package: PackageId,
        /// Local path.
        path: String,
    },
}
/// A summary of the dependency resolution result.
#[derive(Clone, Debug)]
pub struct ResolutionSummary {
    /// Total number of resolved packages.
    pub total_packages: usize,
    /// Total number of direct dependencies.
    pub direct_deps: usize,
    /// Total number of transitive dependencies.
    pub transitive_deps: usize,
    /// Maximum dependency depth.
    pub max_depth: usize,
    /// Packages with multiple version requirements.
    pub shared_packages: Vec<String>,
    /// Total iterations needed for resolution.
    pub iterations: u32,
}
impl ResolutionSummary {
    /// Compute summary from a dependency graph.
    pub fn from_graph(graph: &DependencyGraph) -> Self {
        let root_deps = graph.edges.get(&graph.root).map(|e| e.len()).unwrap_or(0);
        let total = graph.package_count();
        let transitive = total.saturating_sub(root_deps + 1);
        let mut max_depth = 0;
        let mut visited: HashSet<PackageId> = HashSet::new();
        let mut queue: VecDeque<(PackageId, usize)> = VecDeque::new();
        queue.push_back((graph.root.clone(), 0));
        while let Some((pkg, depth)) = queue.pop_front() {
            if !visited.insert(pkg.clone()) {
                continue;
            }
            if depth > max_depth {
                max_depth = depth;
            }
            if let Some(edges) = graph.edges.get(&pkg) {
                for edge in edges {
                    queue.push_back((edge.to.clone(), depth + 1));
                }
            }
        }
        Self {
            total_packages: total,
            direct_deps: root_deps,
            transitive_deps: transitive,
            max_depth,
            shared_packages: Vec::new(),
            iterations: 0,
        }
    }
    /// Format a summary string.
    pub fn format(&self) -> String {
        format!(
            "Resolved {} packages ({} direct, {} transitive, max depth {})",
            self.total_packages, self.direct_deps, self.transitive_deps, self.max_depth
        )
    }
}
/// Simulated git dependency resolver.
#[allow(dead_code)]
pub struct GitResolver {
    known_refs: HashMap<(String, String), String>,
}
#[allow(dead_code)]
impl GitResolver {
    pub fn new() -> Self {
        Self {
            known_refs: HashMap::new(),
        }
    }
    pub fn register(&mut self, url: &str, reference: &str, commit: &str) {
        self.known_refs
            .insert((url.to_string(), reference.to_string()), commit.to_string());
    }
    pub fn resolve(&self, url: &str, reference: Option<&str>) -> Option<GitPinned> {
        let r = reference.unwrap_or("HEAD");
        let commit = self
            .known_refs
            .get(&(url.to_string(), r.to_string()))?
            .clone();
        Some(GitPinned::new(url, &commit).with_reference(r))
    }
    pub fn known_count(&self) -> usize {
        self.known_refs.len()
    }
}
/// Error type for resolution.
#[derive(Clone, Debug)]
pub struct ResolveError {
    /// The cause of the error.
    pub cause: ConflictCause,
    /// Derivation chain explaining how the conflict arose.
    pub derivation: Vec<String>,
}
impl ResolveError {
    /// Create a new resolve error.
    pub fn new(cause: ConflictCause) -> Self {
        Self {
            cause,
            derivation: Vec::new(),
        }
    }
    /// Add a derivation step.
    pub fn with_derivation(mut self, step: &str) -> Self {
        self.derivation.push(step.to_string());
        self
    }
    /// Format a user-friendly error message.
    pub fn format_error(&self) -> String {
        let mut msg = format!("Resolution failed: {}\n", self.cause);
        if !self.derivation.is_empty() {
            msg.push_str("  Derivation:\n");
            for (i, step) in self.derivation.iter().enumerate() {
                msg.push_str(&format!("    {}. {}\n", i + 1, step));
            }
        }
        msg
    }
}
/// Unifies feature sets across multiple dependents requesting the same package.
#[allow(dead_code)]
pub struct FeatureUnifier {
    unified: HashMap<String, BTreeSet<String>>,
}
#[allow(dead_code)]
impl FeatureUnifier {
    pub fn new() -> Self {
        Self {
            unified: HashMap::new(),
        }
    }
    pub fn add_features(&mut self, package: &str, features: &[String]) {
        let entry = self.unified.entry(package.to_string()).or_default();
        for f in features {
            entry.insert(f.clone());
        }
    }
    pub fn get(&self, package: &str) -> Option<&BTreeSet<String>> {
        self.unified.get(package)
    }
    pub fn has_feature(&self, package: &str, feature: &str) -> bool {
        self.unified
            .get(package)
            .map(|fs| fs.contains(feature))
            .unwrap_or(false)
    }
    pub fn package_count(&self) -> usize {
        self.unified.len()
    }
    pub fn total_feature_count(&self) -> usize {
        self.unified.values().map(|s| s.len()).sum()
    }
    pub fn merge(&mut self, other: &FeatureUnifier) {
        for (pkg, features) in &other.unified {
            let entry = self.unified.entry(pkg.clone()).or_default();
            entry.extend(features.iter().cloned());
        }
    }
}
/// A contiguous version range with inclusive lower and exclusive upper bound.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VersionRange {
    /// Inclusive lower bound (None = no lower bound).
    pub lower: Option<Version>,
    /// Exclusive upper bound (None = no upper bound).
    pub upper: Option<Version>,
}
impl VersionRange {
    /// Create an unbounded range (matches everything).
    pub fn any() -> Self {
        Self {
            lower: None,
            upper: None,
        }
    }
    /// Create a range `[lower, upper)`.
    pub fn between(lower: Version, upper: Version) -> Self {
        Self {
            lower: Some(lower),
            upper: Some(upper),
        }
    }
    /// Create a range `[v, v.next_patch())` (exact version).
    pub fn exact(v: Version) -> Self {
        let upper = v.next_patch();
        Self {
            lower: Some(v),
            upper: Some(upper),
        }
    }
    /// Create a range `[lower, inf)`.
    pub fn at_least(lower: Version) -> Self {
        Self {
            lower: Some(lower),
            upper: None,
        }
    }
    /// Create a range `[0.0.0, upper)`.
    pub fn less_than(upper: Version) -> Self {
        Self {
            lower: None,
            upper: Some(upper),
        }
    }
    /// Check if a version is within this range.
    pub fn contains(&self, v: &Version) -> bool {
        if let Some(ref lo) = self.lower {
            if v < lo {
                return false;
            }
        }
        if let Some(ref hi) = self.upper {
            if v >= hi {
                return false;
            }
        }
        true
    }
    /// Check if two ranges overlap.
    pub fn overlaps(&self, other: &Self) -> bool {
        let lo = match (&self.lower, &other.lower) {
            (Some(a), Some(b)) => Some(a.max(b).clone()),
            (Some(a), None) => Some(a.clone()),
            (None, Some(b)) => Some(b.clone()),
            (None, None) => None,
        };
        let hi = match (&self.upper, &other.upper) {
            (Some(a), Some(b)) => Some(a.min(b).clone()),
            (Some(a), None) => Some(a.clone()),
            (None, Some(b)) => Some(b.clone()),
            (None, None) => None,
        };
        match (lo, hi) {
            (Some(l), Some(h)) => l < h,
            _ => true,
        }
    }
    /// Compute the intersection of two ranges.
    pub fn intersect(&self, other: &Self) -> Option<Self> {
        let lower = match (&self.lower, &other.lower) {
            (Some(a), Some(b)) => Some(a.max(b).clone()),
            (Some(a), None) => Some(a.clone()),
            (None, Some(b)) => Some(b.clone()),
            (None, None) => None,
        };
        let upper = match (&self.upper, &other.upper) {
            (Some(a), Some(b)) => Some(a.min(b).clone()),
            (Some(a), None) => Some(a.clone()),
            (None, Some(b)) => Some(b.clone()),
            (None, None) => None,
        };
        if let (Some(ref lo), Some(ref hi)) = (&lower, &upper) {
            if lo >= hi {
                return None;
            }
        }
        Some(Self { lower, upper })
    }
    /// Check if this range is empty.
    pub fn is_empty(&self) -> bool {
        match (&self.lower, &self.upper) {
            (Some(lo), Some(hi)) => lo >= hi,
            _ => false,
        }
    }
    /// Convert a VersionConstraint to a VersionRange.
    pub fn from_constraint(constraint: &VersionConstraint) -> Vec<Self> {
        match constraint {
            VersionConstraint::Any => vec![Self::any()],
            VersionConstraint::Exact(v) => vec![Self::exact(v.clone())],
            VersionConstraint::GreaterEqual(v) => vec![Self::at_least(v.clone())],
            VersionConstraint::Greater(v) => vec![Self::at_least(v.next_patch())],
            VersionConstraint::LessEqual(v) => vec![Self::less_than(v.next_patch())],
            VersionConstraint::Less(v) => vec![Self::less_than(v.clone())],
            VersionConstraint::Caret(v) => {
                let upper = if v.major == 0 {
                    if v.minor == 0 {
                        v.next_patch()
                    } else {
                        v.next_minor()
                    }
                } else {
                    v.next_major()
                };
                vec![Self::between(v.clone(), upper)]
            }
            VersionConstraint::Tilde(v) => {
                let upper = v.next_minor();
                vec![Self::between(v.clone(), upper)]
            }
            VersionConstraint::Wildcard { major, minor } => match minor {
                Some(m) => {
                    let lower = Version::new(*major, *m, 0);
                    let upper = Version::new(*major, *m + 1, 0);
                    vec![Self::between(lower, upper)]
                }
                None => {
                    let lower = Version::new(*major, 0, 0);
                    let upper = Version::new(*major + 1, 0, 0);
                    vec![Self::between(lower, upper)]
                }
            },
            VersionConstraint::And(cs) => {
                let mut result = vec![Self::any()];
                for c in cs {
                    let ranges = Self::from_constraint(c);
                    let mut new_result = Vec::new();
                    for r in &result {
                        for rng in &ranges {
                            if let Some(intersection) = r.intersect(rng) {
                                new_result.push(intersection);
                            }
                        }
                    }
                    result = new_result;
                }
                result
            }
            VersionConstraint::Or(cs) => {
                let mut result = Vec::new();
                for c in cs {
                    result.extend(Self::from_constraint(c));
                }
                result
            }
        }
    }
}
/// Resolves path dependencies relative to a workspace root.
#[allow(dead_code)]
pub struct PathDepResolver {
    workspace_root: std::path::PathBuf,
    registry: HashMap<std::path::PathBuf, (String, Version)>,
}
#[allow(dead_code)]
impl PathDepResolver {
    pub fn new(workspace_root: impl Into<std::path::PathBuf>) -> Self {
        Self {
            workspace_root: workspace_root.into(),
            registry: HashMap::new(),
        }
    }
    pub fn register(
        &mut self,
        relative_path: impl Into<std::path::PathBuf>,
        name: &str,
        version: Version,
    ) {
        self.registry
            .insert(relative_path.into(), (name.to_string(), version));
    }
    pub fn resolve(&self, relative_path: &std::path::Path) -> Option<ResolvedPathDep> {
        let abs = self.workspace_root.join(relative_path);
        let (name, version) = self.registry.get(relative_path)?.clone();
        Some(ResolvedPathDep::new(&name, abs, version))
    }
    pub fn count(&self) -> usize {
        self.registry.len()
    }
}
/// Audit information about a resolved dependency.
#[derive(Clone, Debug)]
pub struct DependencyAudit {
    /// Package name.
    pub name: String,
    /// Resolved version.
    pub version: Version,
    /// Number of dependents.
    pub dependent_count: usize,
    /// Whether this is a direct dependency.
    pub is_direct: bool,
    /// Depth from root.
    pub depth: usize,
    /// License (if known).
    pub license: Option<String>,
}
/// Strategy for choosing among multiple versions that satisfy a constraint.
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum VersionSelectionStrategy {
    /// Choose the highest compatible version (default).
    Maximal,
    /// Choose the lowest compatible version (for reproducibility testing).
    Minimal,
    /// Choose the version closest to the current lockfile entry.
    Locked,
    /// Choose a specific pinned version.
    Pinned(Version),
}
#[allow(dead_code)]
impl VersionSelectionStrategy {
    /// Select the best version from a list of candidates.
    pub fn select<'a>(&self, candidates: &'a [Version]) -> Option<&'a Version> {
        if candidates.is_empty() {
            return None;
        }
        match self {
            Self::Maximal => candidates.iter().max(),
            Self::Minimal => candidates.iter().min(),
            Self::Locked => candidates.first(),
            Self::Pinned(v) => candidates.iter().find(|&c| c == v),
        }
    }
    /// Whether this strategy is deterministic given the same input set.
    pub fn is_deterministic(&self) -> bool {
        !matches!(self, Self::Locked)
    }
}
/// Detailed statistics from a dependency resolution run.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct ResolutionStats {
    pub resolved_packages: usize,
    pub backtrack_count: usize,
    pub registry_queries: usize,
    pub conflicts_resolved: usize,
    pub cycles_detected: usize,
    pub candidates_evaluated: usize,
    pub resolution_time_ms: u64,
}
#[allow(dead_code)]
impl ResolutionStats {
    pub fn zero() -> Self {
        Self {
            resolved_packages: 0,
            backtrack_count: 0,
            registry_queries: 0,
            conflicts_resolved: 0,
            cycles_detected: 0,
            candidates_evaluated: 0,
            resolution_time_ms: 0,
        }
    }
    pub fn avg_candidates_per_package(&self) -> f64 {
        if self.resolved_packages == 0 {
            0.0
        } else {
            self.candidates_evaluated as f64 / self.resolved_packages as f64
        }
    }
    pub fn had_backtracking(&self) -> bool {
        self.backtrack_count > 0
    }
}
/// A resolved git dependency pinned to a specific commit.
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct GitPinned {
    pub url: String,
    pub commit: String,
    pub reference: Option<String>,
}
#[allow(dead_code)]
impl GitPinned {
    pub fn new(url: &str, commit: &str) -> Self {
        Self {
            url: url.to_string(),
            commit: commit.to_string(),
            reference: None,
        }
    }
    pub fn with_reference(mut self, r: &str) -> Self {
        self.reference = Some(r.to_string());
        self
    }
    pub fn is_valid_commit(&self) -> bool {
        let len = self.commit.len();
        (7..=40).contains(&len) && self.commit.chars().all(|c| c.is_ascii_hexdigit())
    }
    pub fn short_commit(&self) -> &str {
        &self.commit[..self.commit.len().min(7)]
    }
}
/// Analyze a resolution error and produce human-readable explanations.
pub struct ConflictAnalyzer {
    explanations: Vec<String>,
}
impl ConflictAnalyzer {
    /// Create a new conflict analyzer.
    pub fn new() -> Self {
        Self {
            explanations: Vec::new(),
        }
    }
    /// Analyze a resolve error.
    pub fn analyze(&mut self, error: &ResolveError) -> &[String] {
        self.explanations.clear();
        match &error.cause {
            ConflictCause::NoMatchingVersion {
                package,
                constraint,
            } => {
                self.explanations.push(format!(
                    "Package '{}' has no version matching '{}'",
                    package, constraint
                ));
                self.explanations.push(
                    "Try relaxing the version constraint or checking the registry.".to_string(),
                );
            }
            ConflictCause::IncompatibleRequirements {
                package,
                req_a,
                source_a,
                req_b,
                source_b,
            } => {
                self.explanations.push(format!(
                    "Package '{}' has incompatible version requirements:",
                    package
                ));
                self.explanations
                    .push(format!("  - {} requires {}", source_a, req_a));
                self.explanations
                    .push(format!("  - {} requires {}", source_b, req_b));
                self.explanations
                    .push("Try finding a version that satisfies both constraints.".to_string());
            }
            ConflictCause::CyclicDependency { cycle } => {
                self.explanations
                    .push("Circular dependency detected:".to_string());
                self.explanations.push(format!("  {}", cycle.join(" -> ")));
            }
            ConflictCause::MissingFeature { package, feature } => {
                self.explanations.push(format!(
                    "Feature '{}' is not available in package '{}'",
                    feature, package
                ));
            }
        }
        for step in &error.derivation {
            self.explanations.push(format!("  Note: {}", step));
        }
        &self.explanations
    }
}
/// A plan for fetching and linking resolved dependencies.
#[derive(Clone, Debug)]
pub struct ResolutionPlan {
    /// Steps to execute in order.
    pub steps: Vec<ResolutionStep>,
    /// Total number of packages.
    pub total_packages: usize,
    /// Total estimated download size in bytes.
    pub estimated_download_bytes: u64,
}
impl ResolutionPlan {
    /// Create an empty plan.
    pub fn new() -> Self {
        Self {
            steps: Vec::new(),
            total_packages: 0,
            estimated_download_bytes: 0,
        }
    }
    /// Create a plan from a dependency graph.
    pub fn from_graph(graph: &DependencyGraph) -> Self {
        let mut plan = Self::new();
        plan.total_packages = graph.package_count();
        for pkg in graph.packages.values() {
            if pkg == &graph.root {
                continue;
            }
            let step = if pkg.source == "path" {
                ResolutionStep::LinkLocal {
                    package: pkg.clone(),
                    path: "local".to_string(),
                }
            } else if pkg.source.starts_with("https://") || pkg.source.starts_with("git://") {
                ResolutionStep::GitClone {
                    package: pkg.clone(),
                    url: pkg.source.clone(),
                    reference: "main".to_string(),
                }
            } else {
                ResolutionStep::Download {
                    package: pkg.clone(),
                    registry_url: pkg.source.clone(),
                }
            };
            plan.steps.push(step);
        }
        plan
    }
}
/// A simple backtracking dependency resolver.
#[allow(dead_code)]
pub struct BacktrackingResolver {
    available: HashMap<String, Vec<Version>>,
    strategy: VersionSelectionStrategy,
    stats: ResolutionStats,
}
#[allow(dead_code)]
impl BacktrackingResolver {
    pub fn new(strategy: VersionSelectionStrategy) -> Self {
        Self {
            available: HashMap::new(),
            strategy,
            stats: ResolutionStats::zero(),
        }
    }
    pub fn register(&mut self, package: &str, versions: Vec<Version>) {
        let mut versions = versions;
        versions.sort_unstable();
        self.available.insert(package.to_string(), versions);
    }
    /// Attempt to resolve a set of package -> constraint pairs.
    pub fn resolve(
        &mut self,
        requirements: &[(String, VersionConstraint)],
    ) -> Result<HashMap<String, Version>, ResolveError> {
        let mut assignment: HashMap<String, Version> = HashMap::new();
        self.stats = ResolutionStats::zero();
        if self.backtrack(requirements, 0, &mut assignment) {
            self.stats.resolved_packages = assignment.len();
            Ok(assignment)
        } else {
            let cause = requirements
                .iter()
                .find(|(pkg, _)| !assignment.contains_key(pkg))
                .map(|(pkg, constraint)| ConflictCause::NoMatchingVersion {
                    package: pkg.clone(),
                    constraint: constraint.clone(),
                })
                .unwrap_or(ConflictCause::CyclicDependency { cycle: vec![] });
            Err(ResolveError::new(cause))
        }
    }
    fn backtrack(
        &mut self,
        requirements: &[(String, VersionConstraint)],
        index: usize,
        assignment: &mut HashMap<String, Version>,
    ) -> bool {
        if index >= requirements.len() {
            return true;
        }
        let (ref pkg, ref constraint) = requirements[index];
        self.stats.registry_queries += 1;
        let candidates: Vec<Version> = self
            .available
            .get(pkg)
            .cloned()
            .unwrap_or_default()
            .into_iter()
            .filter(|v| constraint.matches(v))
            .collect();
        self.stats.candidates_evaluated += candidates.len();
        let ordered: Vec<Version> = match &self.strategy {
            VersionSelectionStrategy::Minimal => {
                let mut c = candidates;
                c.sort_unstable();
                c
            }
            _ => {
                let mut c = candidates;
                c.sort_unstable_by(|a, b| b.cmp(a));
                c
            }
        };
        for version in ordered {
            if let Some(existing) = assignment.get(pkg) {
                if existing != &version {
                    self.stats.backtrack_count += 1;
                    continue;
                }
            }
            assignment.insert(pkg.clone(), version);
            if self.backtrack(requirements, index + 1, assignment) {
                return true;
            }
            assignment.remove(pkg);
            self.stats.backtrack_count += 1;
        }
        false
    }
    pub fn stats(&self) -> &ResolutionStats {
        &self.stats
    }
}
/// A resolved path dependency with an absolute path.
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ResolvedPathDep {
    pub name: String,
    pub path: std::path::PathBuf,
    pub version: Version,
}
#[allow(dead_code)]
impl ResolvedPathDep {
    pub fn new(name: &str, path: impl Into<std::path::PathBuf>, version: Version) -> Self {
        Self {
            name: name.to_string(),
            path: path.into(),
            version,
        }
    }
    pub fn path_exists(&self) -> bool {
        self.path.exists()
    }
    pub fn manifest_path(&self) -> std::path::PathBuf {
        self.path.join("OxiLean.toml")
    }
}
/// A dependency resolver that can produce a lockfile from a manifest.
#[allow(dead_code)]
pub struct LockfileGenerator {
    strategy: VersionSelectionStrategy,
    available: HashMap<String, Vec<Version>>,
}
#[allow(dead_code)]
impl LockfileGenerator {
    pub fn new(strategy: VersionSelectionStrategy) -> Self {
        Self {
            strategy,
            available: HashMap::new(),
        }
    }
    pub fn register_versions(&mut self, pkg: &str, versions: Vec<Version>) {
        let mut vs = versions;
        vs.sort_unstable();
        self.available.insert(pkg.to_string(), vs);
    }
    pub fn generate(&self, manifest: &Manifest) -> Result<crate::manifest::Lockfile, ResolveError> {
        use crate::manifest::{LockedDependency, Lockfile};
        let mut lockfile = Lockfile::new();
        for (name, dep) in &manifest.dependencies {
            let constraint = dep.version.clone();
            match &dep.source {
                DependencySource::Registry { .. } => {
                    let versions = self.available.get(name).cloned().unwrap_or_default();
                    let matching: Vec<&Version> =
                        versions.iter().filter(|v| constraint.matches(v)).collect();
                    let selected = match &self.strategy {
                        VersionSelectionStrategy::Minimal => matching.into_iter().min(),
                        _ => matching.into_iter().max(),
                    };
                    let version = selected.cloned().ok_or_else(|| {
                        ResolveError::new(ConflictCause::NoMatchingVersion {
                            package: name.clone(),
                            constraint: constraint.clone(),
                        })
                    })?;
                    lockfile.add_package(LockedDependency {
                        name: name.clone(),
                        version,
                        source: "registry".to_string(),
                        checksum: None,
                        dependencies: Vec::new(),
                    });
                }
                DependencySource::Path { path: p } => {
                    let version = match &dep.version {
                        VersionConstraint::Exact(v) => v.clone(),
                        _ => Version::new(0, 0, 0),
                    };
                    lockfile.add_package(LockedDependency {
                        name: name.clone(),
                        version,
                        source: format!("path+{}", p.display()),
                        checksum: None,
                        dependencies: Vec::new(),
                    });
                }
                DependencySource::Git { url, reference } => {
                    let ref_str = match reference {
                        crate::manifest::GitReference::Rev(r) => format!("#{}", r),
                        crate::manifest::GitReference::Branch(b) => {
                            format!("?branch={}", b)
                        }
                        crate::manifest::GitReference::Tag(t) => format!("?tag={}", t),
                        crate::manifest::GitReference::Default => String::new(),
                    };
                    lockfile.add_package(LockedDependency {
                        name: name.clone(),
                        version: Version::new(0, 0, 0),
                        source: format!("git+{}{}", url, ref_str),
                        checksum: None,
                        dependencies: Vec::new(),
                    });
                }
            }
        }
        Ok(lockfile)
    }
}
/// Reason for a resolution conflict.
#[derive(Clone, Debug)]
pub enum ConflictCause {
    /// No version of the package satisfies the constraint.
    NoMatchingVersion {
        /// The package that had no matching version.
        package: String,
        /// The constraint that could not be satisfied.
        constraint: VersionConstraint,
    },
    /// Two requirements for the same package are incompatible.
    IncompatibleRequirements {
        /// The package with incompatible requirements.
        package: String,
        /// Requirement from the first source.
        req_a: VersionConstraint,
        /// Source of the first requirement.
        source_a: String,
        /// Requirement from the second source.
        req_b: VersionConstraint,
        /// Source of the second requirement.
        source_b: String,
    },
    /// A dependency cycle was detected.
    CyclicDependency {
        /// The packages forming the cycle.
        cycle: Vec<String>,
    },
    /// A required feature is not available.
    MissingFeature {
        /// Package name.
        package: String,
        /// Feature name.
        feature: String,
    },
}
/// The cause of an incompatibility.
#[derive(Clone, Debug)]
pub enum IncompatibilityCause {
    /// Root requirement from the manifest.
    Root,
    /// Dependency of a package.
    Dependency {
        /// The package that declared the dependency.
        package: String,
        /// The version of that package.
        version: Version,
    },
    /// Derived from two other incompatibilities.
    Derived {
        /// First cause.
        cause_a: Box<IncompatibilityCause>,
        /// Second cause.
        cause_b: Box<IncompatibilityCause>,
    },
    /// No versions available.
    NoVersions(String),
}
/// The dependency resolver implementing a PubGrub-style algorithm.
pub struct Resolver<'a, S: PackageSource> {
    source: &'a S,
    solution: PartialSolution,
    max_iterations: u32,
}
impl<'a, S: PackageSource> Resolver<'a, S> {
    /// Create a new resolver with the given package source.
    pub fn new(source: &'a S) -> Self {
        Self {
            source,
            solution: PartialSolution::new(),
            max_iterations: 10_000,
        }
    }
    /// Set the maximum number of iterations.
    pub fn set_max_iterations(&mut self, max: u32) {
        self.max_iterations = max;
    }
    /// Resolve dependencies for a manifest.
    pub fn resolve(&mut self, manifest: &Manifest) -> Result<DependencyGraph, ResolveError> {
        let root = PackageId::root(&manifest.name, manifest.version.clone());
        let mut graph = DependencyGraph::new(root.clone());
        for dep in manifest.dependencies.values() {
            self.solution
                .add_derivation(&dep.name, dep.version.clone(), &manifest.name);
        }
        let mut iteration = 0;
        let mut to_resolve: VecDeque<String> = manifest.dependencies.keys().cloned().collect();
        while let Some(pkg_name) = to_resolve.pop_front() {
            iteration += 1;
            if iteration > self.max_iterations {
                return Err(ResolveError::new(ConflictCause::NoMatchingVersion {
                    package: pkg_name,
                    constraint: VersionConstraint::Any,
                })
                .with_derivation("exceeded maximum resolution iterations"));
            }
            if self.solution.is_decided(&pkg_name) {
                continue;
            }
            let constraint = self.solution.effective_constraint(&pkg_name);
            let available = self.source.available_versions(&pkg_name);
            let mut candidates: Vec<&PackageSummary> = available
                .iter()
                .filter(|s| !s.yanked && constraint.matches(&s.version))
                .collect();
            candidates.sort_by(|a, b| b.version.cmp(&a.version));
            let chosen = candidates.first().ok_or_else(|| {
                ResolveError::new(ConflictCause::NoMatchingVersion {
                    package: pkg_name.clone(),
                    constraint: constraint.clone(),
                })
            })?;
            self.solution
                .add_decision(&pkg_name, chosen.version.clone());
            let pkg_id = PackageId::new(
                &pkg_name,
                chosen.version.clone(),
                match &manifest.dependencies.get(&pkg_name).map(|d| &d.source) {
                    Some(DependencySource::Registry { registry }) => registry.as_str(),
                    Some(DependencySource::Git { url, .. }) => url.as_str(),
                    Some(DependencySource::Path { path }) => {
                        let _ = path;
                        "path"
                    }
                    None => "registry",
                },
            );
            graph.add_package(pkg_id.clone());
            graph.add_edge(DepEdge {
                from: root.clone(),
                to: pkg_id.clone(),
                constraint: constraint.clone(),
                optional: false,
                features: Vec::new(),
            });
            for dep in &chosen.dependencies {
                self.solution
                    .add_derivation(&dep.name, dep.version.clone(), &pkg_name);
                if !self.solution.is_decided(&dep.name) {
                    to_resolve.push_back(dep.name.clone());
                }
            }
        }
        Ok(graph)
    }
}
/// The resolved dependency graph.
#[derive(Clone, Debug)]
pub struct DependencyGraph {
    /// All resolved packages.
    pub packages: BTreeMap<String, PackageId>,
    /// Directed edges (from -> \[to\]).
    pub edges: HashMap<PackageId, Vec<DepEdge>>,
    /// Reverse edges (to -> \[from\]).
    pub reverse_edges: HashMap<PackageId, Vec<PackageId>>,
    /// The root package.
    pub root: PackageId,
}
impl DependencyGraph {
    /// Create a new dependency graph with the given root.
    pub fn new(root: PackageId) -> Self {
        let root_name = root.name.clone();
        let mut packages = BTreeMap::new();
        packages.insert(root_name, root.clone());
        Self {
            packages,
            edges: HashMap::new(),
            reverse_edges: HashMap::new(),
            root,
        }
    }
    /// Add a package to the graph.
    pub fn add_package(&mut self, id: PackageId) {
        self.packages.insert(id.name.clone(), id);
    }
    /// Add an edge to the graph.
    pub fn add_edge(&mut self, edge: DepEdge) {
        let from = edge.from.clone();
        let to = edge.to.clone();
        self.edges.entry(from.clone()).or_default().push(edge);
        self.reverse_edges.entry(to).or_default().push(from);
    }
    /// Get direct dependencies of a package.
    pub fn dependencies_of(&self, pkg: &PackageId) -> Vec<&PackageId> {
        self.edges
            .get(pkg)
            .map(|edges| edges.iter().map(|e| &e.to).collect())
            .unwrap_or_default()
    }
    /// Get reverse dependencies (dependents) of a package.
    pub fn dependents_of(&self, pkg: &PackageId) -> Vec<&PackageId> {
        self.reverse_edges
            .get(pkg)
            .map(|parents| parents.iter().collect())
            .unwrap_or_default()
    }
    /// Compute a topological ordering of all packages.
    pub fn topological_sort(&self) -> Result<Vec<PackageId>, ResolveError> {
        let mut in_degree: HashMap<PackageId, usize> = HashMap::new();
        for pkg_id in self.packages.values() {
            in_degree.entry(pkg_id.clone()).or_insert(0);
        }
        for edges in self.edges.values() {
            for edge in edges {
                *in_degree.entry(edge.to.clone()).or_insert(0) += 1;
            }
        }
        let mut queue: VecDeque<PackageId> = VecDeque::new();
        for (pkg, deg) in &in_degree {
            if *deg == 0 {
                queue.push_back(pkg.clone());
            }
        }
        let mut result = Vec::new();
        while let Some(pkg) = queue.pop_front() {
            result.push(pkg.clone());
            if let Some(edges) = self.edges.get(&pkg) {
                for edge in edges {
                    if let Some(deg) = in_degree.get_mut(&edge.to) {
                        *deg -= 1;
                        if *deg == 0 {
                            queue.push_back(edge.to.clone());
                        }
                    }
                }
            }
        }
        if result.len() != self.packages.len() {
            let resolved: HashSet<_> = result.iter().map(|p| p.name.clone()).collect();
            let cycle: Vec<String> = self
                .packages
                .keys()
                .filter(|name| !resolved.contains(*name))
                .cloned()
                .collect();
            return Err(ResolveError::new(ConflictCause::CyclicDependency { cycle }));
        }
        Ok(result)
    }
    /// Count the total number of packages.
    pub fn package_count(&self) -> usize {
        self.packages.len()
    }
    /// Count the total number of edges.
    pub fn edge_count(&self) -> usize {
        self.edges.values().map(|e| e.len()).sum()
    }
    /// Check if a package is in the graph.
    pub fn contains(&self, name: &str) -> bool {
        self.packages.contains_key(name)
    }
    /// Get a package by name.
    pub fn get_package(&self, name: &str) -> Option<&PackageId> {
        self.packages.get(name)
    }
    /// Compute the transitive closure of dependencies for a given package.
    pub fn transitive_deps(&self, pkg: &PackageId) -> HashSet<PackageId> {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back(pkg.clone());
        while let Some(current) = queue.pop_front() {
            if !visited.insert(current.clone()) {
                continue;
            }
            if let Some(edges) = self.edges.get(&current) {
                for edge in edges {
                    queue.push_back(edge.to.clone());
                }
            }
        }
        visited.remove(pkg);
        visited
    }
}
/// An assignment in the partial solution.
#[derive(Clone, Debug)]
enum Assignment {
    /// A decision (chosen by the solver).
    Decision {
        package: String,
        version: Version,
        decision_level: u32,
    },
    /// A derivation (forced by unit propagation).
    Derivation {
        package: String,
        constraint: VersionConstraint,
        cause: String,
        decision_level: u32,
    },
}
