//! Module system types for import resolution and dependency tracking.

use std::collections::HashMap;
use std::fmt;
use std::path::PathBuf;

// ─────────────────────────────────────────────────────────────────────────────
// ModulePath
// ─────────────────────────────────────────────────────────────────────────────

/// A dot-separated module path, e.g. `Mathlib.Algebra.Ring` →
/// `["Mathlib", "Algebra", "Ring"]`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ModulePath {
    /// Ordered components of the path (no empty strings).
    pub components: Vec<String>,
}

impl fmt::Display for ModulePath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.components.join("."))
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// ImportDecl
// ─────────────────────────────────────────────────────────────────────────────

/// A parsed `import` declaration.
///
/// Supports three forms:
/// - `import Foo.Bar`            — bare import
/// - `import Foo.Bar as FB`      — aliased import
/// - `import Foo.Bar (f, g)`     — selective import
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImportDecl {
    /// The module being imported.
    pub module: ModulePath,
    /// Optional alias (`import X as Y`).
    pub alias: Option<String>,
    /// Selective names (`import X (f, g)`).  Empty means "import all".
    pub selective: Vec<String>,
}

// ─────────────────────────────────────────────────────────────────────────────
// ModuleInfo
// ─────────────────────────────────────────────────────────────────────────────

/// Metadata about a resolved module.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModuleInfo {
    /// Canonical file-system path of the module file.
    pub path: PathBuf,
    /// Names exported by the module.
    pub exports: Vec<String>,
    /// Direct module dependencies (as declared by `import` statements inside
    /// the module).
    pub dependencies: Vec<ModulePath>,
}

// ─────────────────────────────────────────────────────────────────────────────
// ModuleRegistry
// ─────────────────────────────────────────────────────────────────────────────

/// Registry that maps [`ModulePath`]s to [`ModuleInfo`]s, with file-system
/// root search and a resolution cache.
#[derive(Debug, Clone)]
pub struct ModuleRegistry {
    /// Root directories searched when resolving a module path to a file.
    pub roots: Vec<PathBuf>,
    /// Cache: module path → resolved info.  Populated on first successful
    /// [`ModuleRegistry::resolve`] or explicit [`ModuleRegistry::register`].
    pub cache: HashMap<ModulePath, ModuleInfo>,
}

// ─────────────────────────────────────────────────────────────────────────────
// ModuleGraph
// ─────────────────────────────────────────────────────────────────────────────

/// A directed graph of module dependencies.
///
/// - `nodes`: module → its info
/// - `edges`: `(from, to)` — `from` imports `to`
#[derive(Debug, Clone, Default)]
pub struct ModuleGraph {
    /// All known modules, keyed by path.
    pub nodes: HashMap<ModulePath, ModuleInfo>,
    /// Directed dependency edges `(importer, importee)`.
    pub edges: Vec<(ModulePath, ModulePath)>,
}

// ─────────────────────────────────────────────────────────────────────────────
// CycleError
// ─────────────────────────────────────────────────────────────────────────────

/// A detected import cycle.
///
/// `cycle` contains the sequence of modules that form the cycle, in order.
/// The last element imports the first element.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CycleError {
    /// Ordered list of modules forming the cycle.
    pub cycle: Vec<ModulePath>,
}

impl fmt::Display for CycleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let names: Vec<String> = self.cycle.iter().map(|p| p.to_string()).collect();
        write!(f, "import cycle detected: {}", names.join(" → "))
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// ModuleResolutionResult
// ─────────────────────────────────────────────────────────────────────────────

/// The outcome of attempting to resolve a [`ModulePath`] in a
/// [`ModuleRegistry`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModuleResolutionResult {
    /// The module was found and its info is returned.
    Found(ModuleInfo),
    /// No file could be located for the given path.
    NotFound(ModulePath),
    /// Resolving the module would create an import cycle.
    Cycle(CycleError),
}
