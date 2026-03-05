//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use oxilean_kernel::{Expr, Name};
use std::collections::{HashMap, HashSet, VecDeque};
use std::path::PathBuf;

/// An import declaration with optional selective/renaming.
#[derive(Debug, Clone)]
pub struct ImportDecl {
    /// The module path being imported.
    pub path: ModulePath,
    /// Selective imports: if `Some`, only these names are imported.
    pub selective: Option<Vec<Name>>,
    /// Hidden names (imported everything except these).
    pub hiding: Option<Vec<Name>>,
    /// Renamed imports: `original_name -> new_name`.
    pub renamed: HashMap<Name, Name>,
    /// Whether this is a public (re-exporting) import.
    pub is_public: bool,
}
impl ImportDecl {
    /// Create a simple import of all names.
    pub fn all(path: ModulePath) -> Self {
        Self {
            path,
            selective: None,
            hiding: None,
            renamed: HashMap::new(),
            is_public: false,
        }
    }
    /// Create a selective import.
    pub fn selective(path: ModulePath, names: Vec<Name>) -> Self {
        Self {
            path,
            selective: Some(names),
            hiding: None,
            renamed: HashMap::new(),
            is_public: false,
        }
    }
    /// Create a hiding import.
    pub fn hiding(path: ModulePath, hidden: Vec<Name>) -> Self {
        Self {
            path,
            selective: None,
            hiding: Some(hidden),
            renamed: HashMap::new(),
            is_public: false,
        }
    }
    /// Create a renaming import.
    pub fn renamed(path: ModulePath, renames: HashMap<Name, Name>) -> Self {
        Self {
            path,
            selective: None,
            hiding: None,
            renamed: renames,
            is_public: false,
        }
    }
    /// Mark this import as public (re-exporting).
    pub fn make_public(mut self) -> Self {
        self.is_public = true;
        self
    }
    /// Check if this is a simple (non-selective, non-hiding) import.
    pub fn is_simple(&self) -> bool {
        self.selective.is_none() && self.hiding.is_none() && self.renamed.is_empty()
    }
    /// Check if a name should be imported through this declaration.
    pub fn accepts_name(&self, name: &Name) -> bool {
        if let Some(ref selected) = self.selective {
            return selected.contains(name);
        }
        if let Some(ref hidden) = self.hiding {
            return !hidden.contains(name);
        }
        true
    }
    /// Get the effective name for an import (after renaming).
    pub fn effective_name(&self, original: &Name) -> Name {
        self.renamed
            .get(original)
            .cloned()
            .unwrap_or_else(|| original.clone())
    }
}
/// Information about an exported name from a module.
#[derive(Debug, Clone)]
pub struct ExportInfo {
    /// The exported name.
    pub name: Name,
    /// Visibility of the export.
    pub visibility: Visibility,
    /// The module this name originates from.
    pub origin_module: ModulePath,
    /// The type of the exported constant (if known).
    pub ty: Option<Expr>,
    /// Whether this is a re-export from a transitive import.
    pub is_reexport: bool,
}
impl ExportInfo {
    /// Create a new export info.
    pub fn new(name: Name, visibility: Visibility, origin_module: ModulePath) -> Self {
        Self {
            name,
            visibility,
            origin_module,
            ty: None,
            is_reexport: false,
        }
    }
    /// Create a new export with a type.
    pub fn with_type(mut self, ty: Expr) -> Self {
        self.ty = Some(ty);
        self
    }
    /// Mark as a re-export.
    pub fn as_reexport(mut self) -> Self {
        self.is_reexport = true;
        self
    }
    /// Check if this is a public export.
    pub fn is_public(&self) -> bool {
        self.visibility.is_public()
    }
}
/// A hierarchical module path such as `Mathlib.Data.Nat`.
///
/// Internally stored as a vector of path components.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ModulePath(pub Vec<String>);
impl ModulePath {
    /// Create a module path from components.
    pub fn new(components: Vec<String>) -> Self {
        ModulePath(components)
    }
    /// Create a module path from a dot-separated string.
    pub fn parse_dot_separated(s: &str) -> Self {
        s.parse().expect("ModulePath::from_str is infallible")
    }
    /// Create a module path from a `Name`.
    pub fn from_name(name: &Name) -> Self {
        let s = format!("{}", name);
        Self::parse_dot_separated(&s)
    }
    /// Convert to a file system path (e.g. `Mathlib/Data/Nat.lean`).
    pub fn to_file_path(&self) -> PathBuf {
        let mut path = PathBuf::new();
        for (i, component) in self.0.iter().enumerate() {
            if i == self.0.len() - 1 {
                path.push(format!("{}.lean", component));
            } else {
                path.push(component);
            }
        }
        path
    }
    /// Convert to a file system path with a custom extension.
    pub fn to_file_path_with_ext(&self, ext: &str) -> PathBuf {
        let mut path = PathBuf::new();
        for (i, component) in self.0.iter().enumerate() {
            if i == self.0.len() - 1 {
                path.push(format!("{}.{}", component, ext));
            } else {
                path.push(component);
            }
        }
        path
    }
    /// Convert to a directory path (for module directories).
    pub fn to_dir_path(&self) -> PathBuf {
        let mut path = PathBuf::new();
        for component in &self.0 {
            path.push(component);
        }
        path
    }
    /// Get the parent module path, if any.
    pub fn parent(&self) -> Option<ModulePath> {
        if self.0.len() <= 1 {
            None
        } else {
            Some(ModulePath(self.0[..self.0.len() - 1].to_vec()))
        }
    }
    /// Get the leaf (last component) of the path.
    pub fn leaf(&self) -> Option<&str> {
        self.0.last().map(|s| s.as_str())
    }
    /// Check if this path is a prefix of another path.
    pub fn is_prefix_of(&self, other: &ModulePath) -> bool {
        if self.0.len() > other.0.len() {
            return false;
        }
        self.0.iter().zip(other.0.iter()).all(|(a, b)| a == b)
    }
    /// Check if this path is a strict prefix of another path.
    pub fn is_strict_prefix_of(&self, other: &ModulePath) -> bool {
        self.is_prefix_of(other) && self.0.len() < other.0.len()
    }
    /// Append a component to the path.
    pub fn child(&self, name: &str) -> ModulePath {
        let mut components = self.0.clone();
        components.push(name.to_string());
        ModulePath(components)
    }
    /// Join two paths.
    pub fn join(&self, other: &ModulePath) -> ModulePath {
        let mut components = self.0.clone();
        components.extend(other.0.clone());
        ModulePath(components)
    }
    /// Get the number of components.
    pub fn depth(&self) -> usize {
        self.0.len()
    }
    /// Check if the path is empty.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    /// Convert to a dot-separated string.
    pub fn to_dot_string(&self) -> String {
        self.0.join(".")
    }
    /// Convert to a kernel `Name`.
    pub fn to_name(&self) -> Name {
        Name::str(self.to_dot_string())
    }
    /// Get the components as a slice.
    pub fn components(&self) -> &[String] {
        &self.0
    }
    /// Get the root component.
    pub fn root(&self) -> Option<&str> {
        self.0.first().map(|s| s.as_str())
    }
    /// Strip a common prefix, returning the relative path.
    pub fn strip_prefix(&self, prefix: &ModulePath) -> Option<ModulePath> {
        if !prefix.is_prefix_of(self) {
            return None;
        }
        Some(ModulePath(self.0[prefix.0.len()..].to_vec()))
    }
}
/// A filter applied to resolved import names.
#[derive(Clone, Debug)]
pub enum ImportFilter {
    /// Accept all names.
    All,
    /// Accept only names in the whitelist.
    Only(HashSet<Name>),
    /// Accept all names except those in the blacklist.
    Except(HashSet<Name>),
    /// Accept names matching a prefix.
    Prefix(String),
}
impl ImportFilter {
    /// Return `true` if `name` passes this filter.
    pub fn accepts(&self, name: &Name) -> bool {
        match self {
            ImportFilter::All => true,
            ImportFilter::Only(set) => set.contains(name),
            ImportFilter::Except(set) => !set.contains(name),
            ImportFilter::Prefix(prefix) => name.to_string().starts_with(prefix.as_str()),
        }
    }
    /// Filter a list of resolved imports.
    pub fn apply(&self, names: Vec<Name>) -> Vec<Name> {
        names.into_iter().filter(|n| self.accepts(n)).collect()
    }
}
/// An import session accumulates all resolved imports for one file.
#[derive(Debug, Default)]
pub struct ImportSession {
    /// All resolved imports, indexed by local name.
    resolved: HashMap<Name, ResolvedImport>,
    /// Conflicts detected during this session.
    conflicts: Vec<ImportConflict>,
    /// Stats about this session.
    pub stats: ImportStats,
}
impl ImportSession {
    /// Create a new empty session.
    pub fn new() -> Self {
        Self::default()
    }
    /// Add a resolved import to the session.  Returns `false` if the name
    /// conflicts with an existing entry.
    pub fn add(&mut self, import: ResolvedImport) -> bool {
        let local = import.local_name.clone();
        if self.resolved.contains_key(&local) {
            let existing_source = self.resolved[&local].source_module.clone();
            let new_source = import.source_module.clone();
            self.conflicts
                .push(ImportConflict::new(local, existing_source, new_source));
            self.stats.conflicts_resolved += 1;
            false
        } else {
            self.resolved.insert(import.local_name.clone(), import);
            self.stats.total_decls += 1;
            true
        }
    }
    /// Look up a resolved import by local name.
    pub fn lookup(&self, name: &Name) -> Option<&ResolvedImport> {
        self.resolved.get(name)
    }
    /// Return all conflicts detected so far.
    pub fn conflicts(&self) -> &[ImportConflict] {
        &self.conflicts
    }
    /// Number of successfully resolved imports.
    pub fn num_resolved(&self) -> usize {
        self.resolved.len()
    }
    /// Return `true` if any conflicts were detected.
    pub fn has_conflicts(&self) -> bool {
        !self.conflicts.is_empty()
    }
    /// Return all local names in this session.
    pub fn local_names(&self) -> impl Iterator<Item = &Name> {
        self.resolved.keys()
    }
}
/// Data for a loaded module.
#[derive(Debug, Clone)]
pub struct ModuleData {
    /// The module path.
    pub path: ModulePath,
    /// Exported names.
    pub exports: HashMap<Name, ExportInfo>,
    /// Direct dependencies (imported modules).
    pub dependencies: Vec<ModulePath>,
    /// Whether the module has been fully elaborated.
    pub is_elaborated: bool,
    /// Module header.
    pub header: ModuleHeader,
}
impl ModuleData {
    /// Create a new module data.
    pub fn new(path: ModulePath, header: ModuleHeader) -> Self {
        Self {
            path,
            exports: HashMap::new(),
            dependencies: header.imported_paths().into_iter().cloned().collect(),
            is_elaborated: false,
            header,
        }
    }
    /// Add an exported name.
    pub fn add_export(&mut self, name: Name, info: ExportInfo) {
        self.exports.insert(name, info);
    }
    /// Get all public exports.
    pub fn public_exports(&self) -> impl Iterator<Item = (&Name, &ExportInfo)> {
        self.exports.iter().filter(|(_, info)| info.is_public())
    }
    /// Get all exports with at least protected visibility.
    pub fn protected_exports(&self) -> impl Iterator<Item = (&Name, &ExportInfo)> {
        self.exports
            .iter()
            .filter(|(_, info)| info.visibility.is_at_least_protected())
    }
    /// Look up an export by name.
    pub fn lookup_export(&self, name: &Name) -> Option<&ExportInfo> {
        self.exports.get(name)
    }
    /// Number of exported names.
    pub fn num_exports(&self) -> usize {
        self.exports.len()
    }
    /// Mark as elaborated.
    pub fn mark_elaborated(&mut self) {
        self.is_elaborated = true;
    }
}
/// A single resolved import entry: name + origin info.
#[derive(Debug, Clone)]
pub struct ResolvedImport {
    /// The name as it will appear in the importing module.
    pub local_name: Name,
    /// The original name in the source module.
    pub original_name: Name,
    /// The source module.
    pub source_module: ModulePath,
    /// The type of the imported constant (if known).
    pub ty: Option<Expr>,
    /// Whether this was re-exported.
    pub is_reexport: bool,
}
/// The result of resolving an import declaration.
#[derive(Debug, Clone)]
pub struct ImportResult {
    /// Successfully resolved names.
    pub resolved_names: Vec<ResolvedImport>,
    /// Errors encountered during resolution.
    pub errors: Vec<ImportError>,
}
impl ImportResult {
    /// Create an empty result.
    pub fn new() -> Self {
        Self {
            resolved_names: Vec::new(),
            errors: Vec::new(),
        }
    }
    /// Create a result with errors only.
    pub fn with_error(err: ImportError) -> Self {
        Self {
            resolved_names: Vec::new(),
            errors: vec![err],
        }
    }
    /// Check if the resolution was fully successful (no errors).
    pub fn is_ok(&self) -> bool {
        self.errors.is_empty()
    }
    /// Number of resolved names.
    pub fn num_resolved(&self) -> usize {
        self.resolved_names.len()
    }
    /// Merge another result into this one.
    pub fn merge(&mut self, other: ImportResult) {
        self.resolved_names.extend(other.resolved_names);
        self.errors.extend(other.errors);
    }
}
/// Aggregated result of loading a module batch.
#[allow(dead_code)]
#[derive(Clone, Debug, Default)]
pub struct ModuleLoadBatch {
    outcomes: Vec<ModuleLoadOutcome>,
}
#[allow(dead_code)]
impl ModuleLoadBatch {
    /// Create an empty batch.
    pub fn new() -> Self {
        Self::default()
    }
    /// Add an outcome.
    pub fn push(&mut self, outcome: ModuleLoadOutcome) {
        self.outcomes.push(outcome);
    }
    /// Return the number of freshly-loaded modules.
    pub fn loaded_count(&self) -> usize {
        self.outcomes.iter().filter(|o| o.is_loaded()).count()
    }
    /// Return the number of cache hits.
    pub fn cached_count(&self) -> usize {
        self.outcomes.iter().filter(|o| o.is_cached()).count()
    }
    /// Return the number of failures.
    pub fn failure_count(&self) -> usize {
        self.outcomes
            .iter()
            .filter(|o| matches!(o, ModuleLoadOutcome::Failed { .. }))
            .count()
    }
    /// Return the total declaration count across loaded modules.
    pub fn total_decls(&self) -> usize {
        self.outcomes.iter().filter_map(|o| o.decl_count()).sum()
    }
    /// Return paths of failed modules.
    pub fn failed_paths(&self) -> Vec<&ModulePath> {
        self.outcomes
            .iter()
            .filter_map(|o| {
                if let ModuleLoadOutcome::Failed { path, .. } = o {
                    Some(path)
                } else {
                    None
                }
            })
            .collect()
    }
}
/// A traversal order for the module DAG.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TraversalOrder {
    /// Process leaves (no dependencies) first.
    BottomUp,
    /// Process roots (nothing depends on them) first.
    TopDown,
}
/// Environment of all loaded modules.
#[derive(Debug, Clone)]
pub struct ModuleEnv {
    /// Loaded modules keyed by path.
    pub modules: HashMap<ModulePath, ModuleData>,
}
impl ModuleEnv {
    /// Create a new empty module environment.
    pub fn new() -> Self {
        Self {
            modules: HashMap::new(),
        }
    }
    /// Add a module to the environment.
    pub fn add_module(&mut self, data: ModuleData) {
        self.modules.insert(data.path.clone(), data);
    }
    /// Look up a module by path.
    pub fn get_module(&self, path: &ModulePath) -> Option<&ModuleData> {
        self.modules.get(path)
    }
    /// Look up a module mutably.
    pub fn get_module_mut(&mut self, path: &ModulePath) -> Option<&mut ModuleData> {
        self.modules.get_mut(path)
    }
    /// Check if a module is loaded.
    pub fn contains_module(&self, path: &ModulePath) -> bool {
        self.modules.contains_key(path)
    }
    /// Number of loaded modules.
    pub fn num_modules(&self) -> usize {
        self.modules.len()
    }
    /// Get all module paths.
    pub fn module_paths(&self) -> Vec<&ModulePath> {
        self.modules.keys().collect()
    }
    /// Resolve a name across all loaded modules.
    /// Returns the module(s) that export this name.
    pub fn resolve_name(&self, name: &Name) -> Vec<(&ModulePath, &ExportInfo)> {
        self.modules
            .iter()
            .filter_map(|(path, data)| data.lookup_export(name).map(|info| (path, info)))
            .collect()
    }
}
/// Visibility of an export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Visibility {
    /// Visible to all importers.
    #[default]
    Public,
    /// Visible to modules in the same package/directory.
    Protected,
    /// Not visible outside the module.
    Private,
}
impl Visibility {
    /// Check if visible to external importers.
    pub fn is_public(&self) -> bool {
        matches!(self, Visibility::Public)
    }
    /// Check if visible within the same package.
    pub fn is_at_least_protected(&self) -> bool {
        matches!(self, Visibility::Public | Visibility::Protected)
    }
}
/// A directed acyclic graph of module dependencies.
///
/// Supports cycle detection and topological sorting.
#[derive(Debug, Clone)]
pub struct DependencyGraph {
    /// Adjacency list: module -> list of dependencies.
    edges: HashMap<ModulePath, Vec<ModulePath>>,
    /// All known nodes.
    nodes: HashSet<ModulePath>,
}
impl DependencyGraph {
    /// Create a new empty dependency graph.
    pub fn new() -> Self {
        Self {
            edges: HashMap::new(),
            nodes: HashSet::new(),
        }
    }
    /// Build a dependency graph from a module environment.
    pub fn from_env(env: &ModuleEnv) -> Self {
        let mut graph = Self::new();
        for (path, data) in &env.modules {
            graph.add_node(path.clone());
            for dep in &data.dependencies {
                graph.add_edge(path.clone(), dep.clone());
            }
        }
        graph
    }
    /// Add a node to the graph.
    pub fn add_node(&mut self, path: ModulePath) {
        self.nodes.insert(path.clone());
        self.edges.entry(path).or_default();
    }
    /// Add a dependency edge: `from` depends on `to`.
    pub fn add_edge(&mut self, from: ModulePath, to: ModulePath) {
        self.nodes.insert(from.clone());
        self.nodes.insert(to.clone());
        self.edges.entry(from).or_default().push(to);
    }
    /// Get the direct dependencies of a module.
    pub fn dependencies(&self, path: &ModulePath) -> &[ModulePath] {
        self.edges.get(path).map_or(&[], |v| v.as_slice())
    }
    /// Get all nodes in the graph.
    pub fn all_nodes(&self) -> &HashSet<ModulePath> {
        &self.nodes
    }
    /// Number of nodes.
    pub fn num_nodes(&self) -> usize {
        self.nodes.len()
    }
    /// Number of edges.
    pub fn num_edges(&self) -> usize {
        self.edges.values().map(|v| v.len()).sum()
    }
    /// Detect cycles using DFS. Returns the first cycle found, if any.
    pub fn detect_cycle(&self) -> Option<Vec<ModulePath>> {
        let mut visited = HashSet::new();
        let mut in_stack = HashSet::new();
        let mut stack = Vec::new();
        for node in &self.nodes {
            if !visited.contains(node) {
                if let Some(cycle) =
                    self.dfs_detect_cycle(node, &mut visited, &mut in_stack, &mut stack)
                {
                    return Some(cycle);
                }
            }
        }
        None
    }
    /// DFS helper for cycle detection.
    fn dfs_detect_cycle(
        &self,
        node: &ModulePath,
        visited: &mut HashSet<ModulePath>,
        in_stack: &mut HashSet<ModulePath>,
        stack: &mut Vec<ModulePath>,
    ) -> Option<Vec<ModulePath>> {
        visited.insert(node.clone());
        in_stack.insert(node.clone());
        stack.push(node.clone());
        if let Some(neighbors) = self.edges.get(node) {
            for neighbor in neighbors {
                if !visited.contains(neighbor) {
                    if let Some(cycle) = self.dfs_detect_cycle(neighbor, visited, in_stack, stack) {
                        return Some(cycle);
                    }
                } else if in_stack.contains(neighbor) {
                    let cycle_start = stack.iter().position(|p| p == neighbor).unwrap_or(0);
                    let mut cycle: Vec<ModulePath> = stack[cycle_start..].to_vec();
                    cycle.push(neighbor.clone());
                    return Some(cycle);
                }
            }
        }
        stack.pop();
        in_stack.remove(node);
        None
    }
    /// Topological sort of the dependency graph.
    ///
    /// Returns modules in dependency order (dependencies before dependents).
    /// Returns `Err` with the cycle if the graph contains cycles.
    pub fn topological_sort(&self) -> Result<Vec<ModulePath>, Vec<ModulePath>> {
        if let Some(cycle) = self.detect_cycle() {
            return Err(cycle);
        }
        let mut in_degree: HashMap<&ModulePath, usize> = HashMap::new();
        for node in &self.nodes {
            in_degree.entry(node).or_insert(0);
        }
        for deps in self.edges.values() {
            for dep in deps {
                *in_degree.entry(dep).or_insert(0) += 0;
            }
        }
        let mut incoming: HashMap<&ModulePath, usize> = HashMap::new();
        for node in &self.nodes {
            incoming.insert(node, 0);
        }
        for deps in self.edges.values() {
            for dep in deps {
                if let Some(count) = incoming.get_mut(dep) {
                    *count += 1;
                }
            }
        }
        let mut queue: VecDeque<&ModulePath> = VecDeque::new();
        for (node, &count) in &incoming {
            if count == 0 {
                queue.push_back(node);
            }
        }
        let mut result = Vec::new();
        while let Some(node) = queue.pop_front() {
            result.push(node.clone());
            if let Some(deps) = self.edges.get(node) {
                for dep in deps {
                    if let Some(count) = incoming.get_mut(dep) {
                        *count = count.saturating_sub(1);
                        if *count == 0 {
                            queue.push_back(dep);
                        }
                    }
                }
            }
        }
        result.reverse();
        Ok(result)
    }
    /// Get all transitive dependencies of a module.
    pub fn transitive_deps(&self, path: &ModulePath) -> HashSet<ModulePath> {
        let mut result = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back(path.clone());
        while let Some(current) = queue.pop_front() {
            if let Some(deps) = self.edges.get(&current) {
                for dep in deps {
                    if result.insert(dep.clone()) {
                        queue.push_back(dep.clone());
                    }
                }
            }
        }
        result
    }
    /// Get all modules that depend on the given module (reverse deps).
    pub fn reverse_deps(&self, path: &ModulePath) -> HashSet<ModulePath> {
        let mut result = HashSet::new();
        for (node, deps) in &self.edges {
            if deps.contains(path) {
                result.insert(node.clone());
            }
        }
        result
    }
}
/// A conflict between two imports.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImportConflict {
    /// The conflicting name.
    pub name: Name,
    /// First source module.
    pub source1: ModulePath,
    /// Second source module.
    pub source2: ModulePath,
}
impl ImportConflict {
    /// Create a new import conflict.
    pub fn new(name: Name, source1: ModulePath, source2: ModulePath) -> Self {
        Self {
            name,
            source1,
            source2,
        }
    }
}
/// Errors that can occur during import resolution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ImportError {
    /// Module not found.
    ModuleNotFound(ModulePath),
    /// Name not found in module.
    NameNotFound {
        /// The name that was not found.
        name: Name,
        /// The module path.
        module: ModulePath,
    },
    /// Name is private and cannot be imported.
    PrivateName {
        /// The private name.
        name: Name,
        /// The module path.
        module: ModulePath,
    },
    /// Circular import detected.
    CircularImport {
        /// The cycle path.
        cycle: Vec<ModulePath>,
    },
    /// Ambiguous import — name resolved to multiple modules.
    AmbiguousImport {
        /// The ambiguous name.
        name: Name,
        /// The candidate modules.
        candidates: Vec<ModulePath>,
    },
}
/// Statistics about the import graph of a module compilation.
#[derive(Clone, Debug, Default)]
pub struct ImportStats {
    /// Total modules imported (directly or transitively).
    pub total_modules: usize,
    /// Total declarations made available via imports.
    pub total_decls: usize,
    /// Number of hiding directives.
    pub hiding_count: usize,
    /// Number of renaming directives.
    pub renaming_count: usize,
    /// Number of selective imports.
    pub selective_count: usize,
    /// Number of conflict resolutions required.
    pub conflicts_resolved: usize,
}
impl ImportStats {
    /// Create zeroed stats.
    pub fn new() -> Self {
        Self::default()
    }
    /// Merge stats from another object.
    pub fn merge(&mut self, other: &ImportStats) {
        self.total_modules += other.total_modules;
        self.total_decls += other.total_decls;
        self.hiding_count += other.hiding_count;
        self.renaming_count += other.renaming_count;
        self.selective_count += other.selective_count;
        self.conflicts_resolved += other.conflicts_resolved;
    }
    /// Average declarations per module.
    pub fn avg_decls_per_module(&self) -> f64 {
        if self.total_modules == 0 {
            0.0
        } else {
            self.total_decls as f64 / self.total_modules as f64
        }
    }
    /// One-line summary.
    pub fn summary(&self) -> String {
        format!(
            "modules={} decls={} hiding={} renaming={} selective={} conflicts={}",
            self.total_modules,
            self.total_decls,
            self.hiding_count,
            self.renaming_count,
            self.selective_count,
            self.conflicts_resolved,
        )
    }
}
/// Resolves import declarations against a module environment.
pub struct ImportResolver<'env> {
    /// The module environment.
    env: &'env ModuleEnv,
    /// Whether to allow importing private names (for internal use).
    allow_private: bool,
}
impl<'env> ImportResolver<'env> {
    /// Create a new import resolver.
    pub fn new(env: &'env ModuleEnv) -> Self {
        Self {
            env,
            allow_private: false,
        }
    }
    /// Enable importing private names.
    pub fn allow_private_imports(mut self) -> Self {
        self.allow_private = true;
        self
    }
    /// Resolve a single import declaration.
    pub fn resolve(&self, import: &ImportDecl) -> ImportResult {
        let module_data = match self.env.get_module(&import.path) {
            Some(data) => data,
            None => {
                return ImportResult::with_error(ImportError::ModuleNotFound(import.path.clone()));
            }
        };
        let mut result = ImportResult::new();
        if let Some(ref selected) = import.selective {
            for name in selected {
                match module_data.lookup_export(name) {
                    Some(export_info) => {
                        if !self.allow_private && !export_info.visibility.is_public() {
                            result.errors.push(ImportError::PrivateName {
                                name: name.clone(),
                                module: import.path.clone(),
                            });
                        } else {
                            let local_name = import.effective_name(name);
                            result.resolved_names.push(ResolvedImport {
                                local_name,
                                original_name: name.clone(),
                                source_module: import.path.clone(),
                                ty: export_info.ty.clone(),
                                is_reexport: import.is_public,
                            });
                        }
                    }
                    None => {
                        result.errors.push(ImportError::NameNotFound {
                            name: name.clone(),
                            module: import.path.clone(),
                        });
                    }
                }
            }
            return result;
        }
        for (name, export_info) in module_data.public_exports() {
            if !import.accepts_name(name) {
                continue;
            }
            let local_name = import.effective_name(name);
            result.resolved_names.push(ResolvedImport {
                local_name,
                original_name: name.clone(),
                source_module: import.path.clone(),
                ty: export_info.ty.clone(),
                is_reexport: import.is_public,
            });
        }
        result
    }
    /// Resolve multiple import declarations.
    pub fn resolve_all(&self, imports: &[ImportDecl]) -> ImportResult {
        let mut combined = ImportResult::new();
        for import in imports {
            let result = self.resolve(import);
            combined.merge(result);
        }
        combined
    }
}
/// Extended module DAG utilities built on top of `DependencyGraph`.
#[derive(Debug, Default)]
pub struct ModuleDag {
    /// The underlying dependency graph.
    graph: DependencyGraph,
}
impl ModuleDag {
    /// Create an empty DAG.
    pub fn new() -> Self {
        Self::default()
    }
    /// Return the underlying graph.
    pub fn graph(&self) -> &DependencyGraph {
        &self.graph
    }
    /// Return the number of modules in the DAG.
    pub fn num_modules(&self) -> usize {
        self.graph.num_nodes()
    }
    /// Return `true` if the DAG has any cycles.
    pub fn has_cycles(&self) -> bool {
        self.graph.detect_cycle().is_some()
    }
    /// Return all modules that have no dependencies (leaf modules).
    pub fn leaves(&self, env: &ModuleEnv) -> Vec<ModulePath> {
        env.modules
            .values()
            .filter(|m| m.dependencies.is_empty())
            .map(|m| m.path.clone())
            .collect()
    }
    /// Compute the transitive closure of dependencies for `path`.
    pub fn transitive_deps(&self, path: &ModulePath, env: &ModuleEnv) -> HashSet<ModulePath> {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back(path.clone());
        while let Some(cur) = queue.pop_front() {
            if visited.contains(&cur) {
                continue;
            }
            visited.insert(cur.clone());
            if let Some(module) = env.get_module(&cur) {
                for imp in &module.dependencies {
                    queue.push_back(imp.clone());
                }
            }
        }
        visited.remove(path);
        visited
    }
    /// Count the depth of the longest dependency chain ending at `path`.
    pub fn max_depth(&self, path: &ModulePath, env: &ModuleEnv) -> usize {
        fn depth_of(
            p: &ModulePath,
            env: &ModuleEnv,
            cache: &mut HashMap<ModulePath, usize>,
        ) -> usize {
            if let Some(&d) = cache.get(p) {
                return d;
            }
            let d = if let Some(m) = env.get_module(p) {
                m.dependencies
                    .iter()
                    .map(|imp| depth_of(imp, env, cache) + 1)
                    .max()
                    .unwrap_or(0)
            } else {
                0
            };
            cache.insert(p.clone(), d);
            d
        }
        let mut cache = HashMap::new();
        depth_of(path, env, &mut cache)
    }
}
/// Outcome of loading a single module.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum ModuleLoadOutcome {
    /// Module was loaded successfully.
    Loaded { path: ModulePath, decl_count: usize },
    /// Module was already in the cache.
    Cached { path: ModulePath },
    /// Module load failed.
    Failed { path: ModulePath, reason: String },
}
#[allow(dead_code)]
impl ModuleLoadOutcome {
    /// Return true if this is a successful load.
    pub fn is_loaded(&self) -> bool {
        matches!(self, ModuleLoadOutcome::Loaded { .. })
    }
    /// Return true if this was a cache hit.
    pub fn is_cached(&self) -> bool {
        matches!(self, ModuleLoadOutcome::Cached { .. })
    }
    /// Return the module path.
    pub fn path(&self) -> &ModulePath {
        match self {
            ModuleLoadOutcome::Loaded { path, .. } => path,
            ModuleLoadOutcome::Cached { path } => path,
            ModuleLoadOutcome::Failed { path, .. } => path,
        }
    }
    /// Return the declaration count, if available.
    pub fn decl_count(&self) -> Option<usize> {
        match self {
            ModuleLoadOutcome::Loaded { decl_count, .. } => Some(*decl_count),
            _ => None,
        }
    }
}
/// The header of a module file, containing its name and import list.
#[derive(Debug, Clone)]
pub struct ModuleHeader {
    /// Module name (fully qualified).
    pub name: ModulePath,
    /// Import declarations.
    pub imports: Vec<ImportDecl>,
    /// Explicit exports (if any; otherwise all public defs are exported).
    pub exports: Vec<Name>,
    /// Whether this module is prelude (auto-imported).
    pub is_prelude: bool,
}
impl ModuleHeader {
    /// Create a new module header.
    pub fn new(name: ModulePath) -> Self {
        Self {
            name,
            imports: Vec::new(),
            exports: Vec::new(),
            is_prelude: false,
        }
    }
    /// Add an import.
    pub fn add_import(&mut self, import: ImportDecl) {
        self.imports.push(import);
    }
    /// Add an export name.
    pub fn add_export(&mut self, name: Name) {
        self.exports.push(name);
    }
    /// Get the imported module paths.
    pub fn imported_paths(&self) -> Vec<&ModulePath> {
        self.imports.iter().map(|i| &i.path).collect()
    }
}
