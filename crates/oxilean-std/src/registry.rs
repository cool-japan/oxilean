//! Standard library module registry, dependency graph, feature flags,
//! and metadata types.

#![allow(dead_code)]
#![allow(missing_docs)]

// ============================================================================
// Environment Builder Registry & Orchestration
// ============================================================================

/// Represents a phase in the standard library build process.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BuildPhase {
    /// Phase 1: Primitive types (Nat, Bool, Char, etc.)
    Primitives,
    /// Phase 2: Collection types (List, Array, Set, etc.)
    Collections,
    /// Phase 3: Type class definitions (Eq, Ord, Show, etc.)
    TypeClasses,
    /// Phase 4: Core theorems and lemmas.
    Theorems,
    /// Phase 5: Automation (tactic lemmas, decision procedures).
    Automation,
}

impl BuildPhase {
    /// Returns all phases in build order.
    #[allow(dead_code)]
    pub fn all_in_order() -> &'static [BuildPhase] {
        &[
            BuildPhase::Primitives,
            BuildPhase::Collections,
            BuildPhase::TypeClasses,
            BuildPhase::Theorems,
            BuildPhase::Automation,
        ]
    }

    /// Returns a human-readable name for this phase.
    #[allow(dead_code)]
    pub fn name(&self) -> &'static str {
        match self {
            BuildPhase::Primitives => "primitives",
            BuildPhase::Collections => "collections",
            BuildPhase::TypeClasses => "type_classes",
            BuildPhase::Theorems => "theorems",
            BuildPhase::Automation => "automation",
        }
    }
}

/// A registry entry describing one standard library module.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct StdModuleEntry {
    /// Fully qualified module name.
    pub qualified_name: &'static str,
    /// Build phase this module belongs to.
    pub phase: BuildPhase,
    /// Whether this module is loaded by default.
    pub default_load: bool,
    /// Brief description of module purpose.
    pub description: &'static str,
}

/// Inventory of all standard library modules.
#[allow(dead_code)]
pub static STD_MODULE_REGISTRY: &[StdModuleEntry] = &[
    StdModuleEntry {
        qualified_name: "Std.Nat",
        phase: BuildPhase::Primitives,
        default_load: true,
        description: "Natural number type",
    },
    StdModuleEntry {
        qualified_name: "Std.Bool",
        phase: BuildPhase::Primitives,
        default_load: true,
        description: "Boolean type",
    },
    StdModuleEntry {
        qualified_name: "Std.Char",
        phase: BuildPhase::Primitives,
        default_load: true,
        description: "Unicode character type",
    },
    StdModuleEntry {
        qualified_name: "Std.Int",
        phase: BuildPhase::Primitives,
        default_load: true,
        description: "Signed integer type",
    },
    StdModuleEntry {
        qualified_name: "Std.String",
        phase: BuildPhase::Primitives,
        default_load: true,
        description: "Immutable string type",
    },
    StdModuleEntry {
        qualified_name: "Std.List",
        phase: BuildPhase::Collections,
        default_load: true,
        description: "Linked list type",
    },
    StdModuleEntry {
        qualified_name: "Std.Array",
        phase: BuildPhase::Collections,
        default_load: true,
        description: "Fixed-size arrays",
    },
    StdModuleEntry {
        qualified_name: "Std.Set",
        phase: BuildPhase::Collections,
        default_load: false,
        description: "Mathematical sets",
    },
    StdModuleEntry {
        qualified_name: "Std.Fin",
        phase: BuildPhase::Collections,
        default_load: true,
        description: "Finite sets",
    },
    StdModuleEntry {
        qualified_name: "Std.Vec",
        phase: BuildPhase::Collections,
        default_load: false,
        description: "Length-indexed vectors",
    },
    StdModuleEntry {
        qualified_name: "Std.Option",
        phase: BuildPhase::Collections,
        default_load: true,
        description: "Optional value type",
    },
    StdModuleEntry {
        qualified_name: "Std.Result",
        phase: BuildPhase::Collections,
        default_load: true,
        description: "Result type",
    },
    StdModuleEntry {
        qualified_name: "Std.Prod",
        phase: BuildPhase::Collections,
        default_load: true,
        description: "Product type",
    },
    StdModuleEntry {
        qualified_name: "Std.Sum",
        phase: BuildPhase::Collections,
        default_load: true,
        description: "Sum type",
    },
    StdModuleEntry {
        qualified_name: "Std.Sigma",
        phase: BuildPhase::Collections,
        default_load: false,
        description: "Dependent pair type",
    },
    StdModuleEntry {
        qualified_name: "Std.Eq",
        phase: BuildPhase::TypeClasses,
        default_load: true,
        description: "Equality type class",
    },
    StdModuleEntry {
        qualified_name: "Std.Ord",
        phase: BuildPhase::TypeClasses,
        default_load: true,
        description: "Total ordering type class",
    },
    StdModuleEntry {
        qualified_name: "Std.Show",
        phase: BuildPhase::TypeClasses,
        default_load: false,
        description: "String representation",
    },
    StdModuleEntry {
        qualified_name: "Std.Functor",
        phase: BuildPhase::TypeClasses,
        default_load: true,
        description: "Functor type class",
    },
    StdModuleEntry {
        qualified_name: "Std.Monad",
        phase: BuildPhase::TypeClasses,
        default_load: false,
        description: "Monad type class",
    },
    StdModuleEntry {
        qualified_name: "Std.Decidable",
        phase: BuildPhase::TypeClasses,
        default_load: true,
        description: "Decidable propositions",
    },
    StdModuleEntry {
        qualified_name: "Std.Algebra",
        phase: BuildPhase::TypeClasses,
        default_load: false,
        description: "Algebraic structures",
    },
    StdModuleEntry {
        qualified_name: "Std.Logic",
        phase: BuildPhase::Theorems,
        default_load: true,
        description: "Classical logic",
    },
    StdModuleEntry {
        qualified_name: "Std.Prop",
        phase: BuildPhase::Theorems,
        default_load: true,
        description: "Propositional theorems",
    },
    StdModuleEntry {
        qualified_name: "Std.Order",
        phase: BuildPhase::Theorems,
        default_load: false,
        description: "Order theorems",
    },
    StdModuleEntry {
        qualified_name: "Std.TacticLemmas",
        phase: BuildPhase::Automation,
        default_load: true,
        description: "Tactic lemmas",
    },
    StdModuleEntry {
        qualified_name: "Std.WellFounded",
        phase: BuildPhase::Automation,
        default_load: false,
        description: "Well-founded relations",
    },
];

/// Get all module entries for a specific build phase.
#[allow(dead_code)]
pub fn modules_for_phase(phase: BuildPhase) -> Vec<&'static StdModuleEntry> {
    STD_MODULE_REGISTRY
        .iter()
        .filter(|e| e.phase == phase)
        .collect()
}

/// Get all default-loaded modules.
#[allow(dead_code)]
pub fn default_modules() -> Vec<&'static StdModuleEntry> {
    STD_MODULE_REGISTRY
        .iter()
        .filter(|e| e.default_load)
        .collect()
}

/// Get all modules.
#[allow(dead_code)]
pub fn all_modules() -> &'static [StdModuleEntry] {
    STD_MODULE_REGISTRY
}

/// Count how many modules are loaded by default.
#[allow(dead_code)]
pub fn count_default_modules() -> usize {
    STD_MODULE_REGISTRY
        .iter()
        .filter(|e| e.default_load)
        .count()
}

/// Count total number of registered standard library modules.
#[allow(dead_code)]
pub fn count_total_modules() -> usize {
    STD_MODULE_REGISTRY.len()
}

/// Look up a module entry by its qualified name.
#[allow(dead_code)]
pub fn find_module(qualified_name: &str) -> Option<&'static StdModuleEntry> {
    STD_MODULE_REGISTRY
        .iter()
        .find(|e| e.qualified_name == qualified_name)
}

/// A dependency pair: (dependent, dependency).
#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub struct ModuleDep {
    /// The module that depends on another.
    pub dependent: &'static str,
    /// The module that must be built first.
    pub dependency: &'static str,
}

/// Minimal dependency graph for core standard library modules.
#[allow(dead_code)]
pub static CORE_DEPS: &[ModuleDep] = &[
    ModuleDep {
        dependent: "Std.List",
        dependency: "Std.Nat",
    },
    ModuleDep {
        dependent: "Std.Array",
        dependency: "Std.Nat",
    },
    ModuleDep {
        dependent: "Std.Fin",
        dependency: "Std.Nat",
    },
    ModuleDep {
        dependent: "Std.Vec",
        dependency: "Std.List",
    },
    ModuleDep {
        dependent: "Std.Set",
        dependency: "Std.List",
    },
    ModuleDep {
        dependent: "Std.Ord",
        dependency: "Std.Eq",
    },
    ModuleDep {
        dependent: "Std.Functor",
        dependency: "Std.Eq",
    },
    ModuleDep {
        dependent: "Std.Monad",
        dependency: "Std.Functor",
    },
    ModuleDep {
        dependent: "Std.Algebra",
        dependency: "Std.Eq",
    },
    ModuleDep {
        dependent: "Std.Logic",
        dependency: "Std.Bool",
    },
    ModuleDep {
        dependent: "Std.Prop",
        dependency: "Std.Logic",
    },
    ModuleDep {
        dependent: "Std.Order",
        dependency: "Std.Ord",
    },
    ModuleDep {
        dependent: "Std.TacticLemmas",
        dependency: "Std.Nat",
    },
    ModuleDep {
        dependent: "Std.WellFounded",
        dependency: "Std.Nat",
    },
];

/// Get all dependencies of a named module (direct only).
#[allow(dead_code)]
pub fn direct_deps(module: &str) -> Vec<&'static str> {
    CORE_DEPS
        .iter()
        .filter(|d| d.dependent == module)
        .map(|d| d.dependency)
        .collect()
}

/// Get all modules that depend on the given module.
#[allow(dead_code)]
pub fn dependents_of(module: &str) -> Vec<&'static str> {
    CORE_DEPS
        .iter()
        .filter(|d| d.dependency == module)
        .map(|d| d.dependent)
        .collect()
}

/// Compute a topological ordering of modules based on `CORE_DEPS`.
///
/// Returns `None` if there is a cycle.
#[allow(dead_code)]
pub fn topological_sort_modules() -> Option<Vec<&'static str>> {
    let mut in_degree: std::collections::HashMap<&'static str, usize> =
        std::collections::HashMap::new();
    let mut adjacency: std::collections::HashMap<&'static str, Vec<&'static str>> =
        std::collections::HashMap::new();

    // Initialize all modules
    for entry in STD_MODULE_REGISTRY {
        in_degree.entry(entry.qualified_name).or_insert(0);
        adjacency.entry(entry.qualified_name).or_default();
    }

    // Build graph from dependencies
    for dep in CORE_DEPS {
        *in_degree.entry(dep.dependent).or_insert(0) += 1;
        adjacency
            .entry(dep.dependency)
            .or_default()
            .push(dep.dependent);
    }

    // Kahn's algorithm
    let mut queue: std::collections::VecDeque<&'static str> = in_degree
        .iter()
        .filter(|(_, &d)| d == 0)
        .map(|(&n, _)| n)
        .collect();

    let mut sorted = Vec::new();
    while let Some(node) = queue.pop_front() {
        sorted.push(node);
        if let Some(neighbors) = adjacency.get(node) {
            for &neighbor in neighbors {
                let deg = in_degree.entry(neighbor).or_insert(0);
                *deg -= 1;
                if *deg == 0 {
                    queue.push_back(neighbor);
                }
            }
        }
    }

    if sorted.len() == in_degree.len() {
        Some(sorted)
    } else {
        None // cycle detected
    }
}

/// OxiLean standard library version string.
#[allow(dead_code)]
pub const STD_VERSION: &str = "0.1.1";

/// Feature flags for optional standard library components.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct StdFeatures {
    /// Enable classical logic axioms (excluded middle, choice).
    pub classical: bool,
    /// Enable propext (propositional extensionality).
    pub propext: bool,
    /// Enable funext (function extensionality).
    pub funext: bool,
    /// Enable quotient types.
    pub quotient: bool,
    /// Enable experimental category theory module.
    pub category_theory: bool,
    /// Enable topology module.
    pub topology: bool,
    /// Enable real number support.
    pub reals: bool,
}

impl StdFeatures {
    /// Create the default feature set (classical logic enabled by default).
    #[allow(dead_code)]
    pub fn default_features() -> Self {
        Self {
            classical: true,
            propext: true,
            funext: true,
            quotient: false,
            category_theory: false,
            topology: false,
            reals: false,
        }
    }

    /// Create the full feature set.
    #[allow(dead_code)]
    pub fn full() -> Self {
        Self {
            classical: true,
            propext: true,
            funext: true,
            quotient: true,
            category_theory: true,
            topology: true,
            reals: true,
        }
    }

    /// Count how many features are enabled.
    #[allow(dead_code)]
    pub fn count_enabled(&self) -> usize {
        [
            self.classical,
            self.propext,
            self.funext,
            self.quotient,
            self.category_theory,
            self.topology,
            self.reals,
        ]
        .iter()
        .filter(|&&v| v)
        .count()
    }
}

/// Standard library module statistics.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct StdLibStats {
    /// Total modules registered.
    pub total_modules: usize,
    /// Modules enabled by default.
    pub default_modules: usize,
    /// Modules per build phase.
    pub per_phase: [usize; 5],
}

impl StdLibStats {
    /// Compute statistics from the registry.
    #[allow(dead_code)]
    pub fn compute() -> Self {
        let total = count_total_modules();
        let defaults = count_default_modules();
        let phases = [
            modules_for_phase(BuildPhase::Primitives).len(),
            modules_for_phase(BuildPhase::Collections).len(),
            modules_for_phase(BuildPhase::TypeClasses).len(),
            modules_for_phase(BuildPhase::Theorems).len(),
            modules_for_phase(BuildPhase::Automation).len(),
        ];
        Self {
            total_modules: total,
            default_modules: defaults,
            per_phase: phases,
        }
    }

    /// Check if all phases have at least one module.
    #[allow(dead_code)]
    pub fn all_phases_populated(&self) -> bool {
        self.per_phase.iter().all(|&n| n > 0)
    }

    /// Get total modules across all phases.
    #[allow(dead_code)]
    pub fn phase_total(&self) -> usize {
        self.per_phase.iter().sum()
    }
}

/// A record of a single OxiLean standard library theorem or definition
/// that the elaborator knows about.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct StdEntry {
    /// Qualified name (e.g., `Nat.add_comm`).
    pub name: &'static str,
    /// Which module this entry belongs to.
    pub module: &'static str,
    /// Human-readable description.
    pub description: &'static str,
    /// Whether this is a theorem (vs. a definition).
    pub is_theorem: bool,
}

/// A representative sample of well-known standard library entries.
#[allow(dead_code)]
pub const STD_KNOWN_ENTRIES: &[StdEntry] = &[
    StdEntry {
        name: "Nat.zero_add",
        module: "Std.Nat",
        description: "0 + n = n",
        is_theorem: true,
    },
    StdEntry {
        name: "Nat.add_zero",
        module: "Std.Nat",
        description: "n + 0 = n",
        is_theorem: true,
    },
    StdEntry {
        name: "Nat.add_comm",
        module: "Std.Nat",
        description: "Commutativity of natural number addition",
        is_theorem: true,
    },
    StdEntry {
        name: "Nat.add_assoc",
        module: "Std.Nat",
        description: "Associativity of natural number addition",
        is_theorem: true,
    },
    StdEntry {
        name: "Nat.mul_comm",
        module: "Std.Nat",
        description: "Commutativity of natural number multiplication",
        is_theorem: true,
    },
    StdEntry {
        name: "List.length_nil",
        module: "Std.List",
        description: "Length of the empty list is 0",
        is_theorem: true,
    },
    StdEntry {
        name: "List.length_cons",
        module: "Std.List",
        description: "Length of cons is 1 + length of tail",
        is_theorem: true,
    },
    StdEntry {
        name: "Bool.not_not",
        module: "Std.Bool",
        description: "Double negation elimination for Bool",
        is_theorem: true,
    },
    StdEntry {
        name: "Bool.and_comm",
        module: "Std.Bool",
        description: "Commutativity of boolean and",
        is_theorem: true,
    },
    StdEntry {
        name: "Option.some_ne_none",
        module: "Std.Option",
        description: "Some x is never None",
        is_theorem: true,
    },
];

/// Look up a standard library entry by its qualified name.
#[allow(dead_code)]
pub fn lookup_std_entry(name: &str) -> Option<&'static StdEntry> {
    STD_KNOWN_ENTRIES.iter().find(|e| e.name == name)
}

/// Return all entries from a given module.
#[allow(dead_code)]
pub fn entries_in_module(module: &str) -> Vec<&'static StdEntry> {
    STD_KNOWN_ENTRIES
        .iter()
        .filter(|e| e.module == module)
        .collect()
}

/// Return all theorems (non-definitions) in the standard library sample.
#[allow(dead_code)]
pub fn all_theorems() -> Vec<&'static StdEntry> {
    STD_KNOWN_ENTRIES.iter().filter(|e| e.is_theorem).collect()
}

/// A category tag for standard library modules.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StdCategory {
    /// Core arithmetic (Nat, Int).
    Arithmetic,
    /// Logic (Prop, And, Or, Not, Iff).
    Logic,
    /// Data structures (List, Array, Option, etc.).
    Data,
    /// Type classes (Eq, Ord, Functor, etc.).
    TypeClass,
    /// IO and system operations.
    IO,
    /// String operations.
    String,
    /// Tactics and proof automation.
    Tactic,
    /// Universe polymorphism.
    Universe,
}

impl StdCategory {
    /// Human-readable label.
    #[allow(dead_code)]
    pub fn label(self) -> &'static str {
        match self {
            Self::Arithmetic => "Arithmetic",
            Self::Logic => "Logic",
            Self::Data => "Data",
            Self::TypeClass => "TypeClass",
            Self::IO => "IO",
            Self::String => "String",
            Self::Tactic => "Tactic",
            Self::Universe => "Universe",
        }
    }
}

/// Map a module name to its `StdCategory`.
#[allow(dead_code)]
pub fn module_category(module: &str) -> StdCategory {
    if module.contains("Nat") || module.contains("Int") {
        StdCategory::Arithmetic
    } else if module.contains("Logic") || module.contains("Prop") {
        StdCategory::Logic
    } else if module.contains("List") || module.contains("Option") || module.contains("Array") {
        StdCategory::Data
    } else if module.contains("Functor") || module.contains("Monad") || module.contains("Eq") {
        StdCategory::TypeClass
    } else if module.contains("IO") {
        StdCategory::IO
    } else if module.contains("String") || module.contains("Char") {
        StdCategory::String
    } else if module.contains("Tactic") {
        StdCategory::Tactic
    } else {
        StdCategory::Universe
    }
}

/// Version information for the OxiLean standard library.
#[allow(dead_code)]
pub struct StdVersion {
    /// Major version number.
    pub major: u32,
    /// Minor version number.
    pub minor: u32,
    /// Patch version number.
    pub patch: u32,
    /// Pre-release label (empty for stable).
    pub pre: &'static str,
}

impl StdVersion {
    /// The current standard library version.
    #[allow(dead_code)]
    pub const CURRENT: StdVersion = StdVersion {
        major: 0,
        minor: 1,
        patch: 0,
        pre: "alpha",
    };

    /// Format as a semver string.
    #[allow(dead_code)]
    pub fn format_version(&self) -> String {
        self.to_string()
    }

    /// Check if this is a stable release.
    #[allow(dead_code)]
    pub fn is_stable(&self) -> bool {
        self.pre.is_empty()
    }
}

impl std::fmt::Display for StdVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.pre.is_empty() {
            write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
        } else {
            write!(
                f,
                "{}.{}.{}-{}",
                self.major, self.minor, self.patch, self.pre
            )
        }
    }
}

// ============================================================
// Tests
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_registry_not_empty() {
        assert!(!STD_MODULE_REGISTRY.is_empty());
        assert!(count_total_modules() > 10);
    }

    #[test]
    fn test_default_modules_subset() {
        let defaults = default_modules();
        assert!(!defaults.is_empty());
        assert!(defaults.len() <= count_total_modules());
    }

    #[test]
    fn test_modules_for_phase() {
        let primitives = modules_for_phase(BuildPhase::Primitives);
        assert!(!primitives.is_empty());
        for m in &primitives {
            assert_eq!(m.phase, BuildPhase::Primitives);
        }
    }

    #[test]
    fn test_find_module_existing() {
        let m = find_module("Std.Nat");
        assert!(m.is_some());
        assert_eq!(m.expect("m should be valid").phase, BuildPhase::Primitives);
    }

    #[test]
    fn test_find_module_nonexistent() {
        assert!(find_module("Std.DoesNotExist").is_none());
    }

    #[test]
    fn test_direct_deps() {
        let deps = direct_deps("Std.List");
        assert!(deps.contains(&"Std.Nat"));
    }

    #[test]
    fn test_dependents_of() {
        let deps = dependents_of("Std.Nat");
        assert!(deps.contains(&"Std.List"));
    }

    #[test]
    fn test_build_phase_order() {
        let phases = BuildPhase::all_in_order();
        assert_eq!(phases.len(), 5);
        assert_eq!(phases[0], BuildPhase::Primitives);
        assert_eq!(phases[4], BuildPhase::Automation);
    }

    #[test]
    fn test_std_features_default() {
        let f = StdFeatures::default_features();
        assert!(f.classical);
        assert!(f.propext);
        assert!(!f.topology);
    }

    #[test]
    fn test_std_features_full() {
        let f = StdFeatures::full();
        assert!(f.classical && f.topology && f.reals && f.quotient);
        assert_eq!(f.count_enabled(), 7);
    }

    #[test]
    fn test_build_phase_names() {
        assert_eq!(BuildPhase::Primitives.name(), "primitives");
        assert_eq!(BuildPhase::Automation.name(), "automation");
    }

    #[test]
    fn test_count_default_modules() {
        let count = count_default_modules();
        assert!(count > 0);
        assert!(count <= count_total_modules());
    }

    #[test]
    fn test_std_lib_stats() {
        let stats = StdLibStats::compute();
        assert!(stats.total_modules > 0);
        assert!(stats.all_phases_populated());
    }

    #[test]
    fn test_all_modules_count() {
        assert_eq!(all_modules().len(), STD_MODULE_REGISTRY.len());
    }

    #[test]
    fn test_std_version_nonempty() {
        assert!(!STD_VERSION.is_empty());
    }

    #[test]
    fn test_std_lib_stats_compute() {
        let s = StdLibStats::compute();
        assert_eq!(s.total_modules, count_total_modules());
        assert!(s.all_phases_populated());
        assert!(s.phase_total() > 0);
    }

    #[test]
    fn test_std_lib_stats_phase_total() {
        let s = StdLibStats::compute();
        assert_eq!(s.phase_total(), s.total_modules);
    }

    #[test]
    fn test_module_descriptions_not_empty() {
        for m in STD_MODULE_REGISTRY {
            assert!(
                !m.description.is_empty(),
                "Module {} has empty description",
                m.qualified_name
            );
        }
    }

    #[test]
    fn test_topological_sort_acyclic() {
        let result = topological_sort_modules();
        assert!(result.is_some(), "Dependency graph should have no cycles");
        let sorted = result.expect("result should be valid");
        assert!(!sorted.is_empty());
    }

    #[test]
    fn test_nat_before_list() {
        let sorted = topological_sort_modules().expect("operation should succeed");
        let nat_pos = sorted.iter().position(|&s| s == "Std.Nat");
        let list_pos = sorted.iter().position(|&s| s == "Std.List");
        if let (Some(np), Some(lp)) = (nat_pos, list_pos) {
            assert!(np < lp, "Nat must appear before List");
        }
    }

    #[test]
    fn test_nat_before_array() {
        let sorted = topological_sort_modules().expect("operation should succeed");
        let nat_pos = sorted.iter().position(|&s| s == "Std.Nat");
        let arr_pos = sorted.iter().position(|&s| s == "Std.Array");
        if let (Some(np), Some(ap)) = (nat_pos, arr_pos) {
            assert!(np < ap);
        }
    }

    #[test]
    fn test_all_modules_in_sorted() {
        let sorted = topological_sort_modules().expect("operation should succeed");
        for entry in STD_MODULE_REGISTRY {
            assert!(
                sorted.contains(&entry.qualified_name),
                "Module {} missing from topological sort",
                entry.qualified_name
            );
        }
    }

    #[test]
    fn test_std_features_default_count() {
        let f = StdFeatures::default_features();
        // classical, propext, funext are true → 3 enabled
        assert_eq!(f.count_enabled(), 3);
    }

    #[test]
    fn test_module_dep_dependent_in_registry() {
        let names: Vec<_> = STD_MODULE_REGISTRY
            .iter()
            .map(|e| e.qualified_name)
            .collect();
        for dep in CORE_DEPS {
            assert!(
                names.contains(&dep.dependent),
                "Dependent {} not in registry",
                dep.dependent
            );
        }
    }

    #[test]
    fn test_module_dep_dependency_in_registry() {
        let names: Vec<_> = STD_MODULE_REGISTRY
            .iter()
            .map(|e| e.qualified_name)
            .collect();
        for dep in CORE_DEPS {
            assert!(
                names.contains(&dep.dependency),
                "Dependency {} not in registry",
                dep.dependency
            );
        }
    }

    #[test]
    fn test_std_lib_stats_phase_count() {
        let s = StdLibStats::compute();
        assert_eq!(s.per_phase.len(), 5);
    }

    #[test]
    fn test_direct_deps_non_empty() {
        let deps = direct_deps("Std.Monad");
        assert!(!deps.is_empty());
    }

    #[test]
    fn test_lookup_std_entry_found() {
        let e = lookup_std_entry("Nat.add_comm");
        assert!(e.is_some());
        assert!(e.expect("e should be valid").is_theorem);
    }

    #[test]
    fn test_lookup_std_entry_not_found() {
        assert!(lookup_std_entry("Nonexistent.theorem").is_none());
    }

    #[test]
    fn test_entries_in_module() {
        let nat_entries = entries_in_module("Std.Nat");
        assert!(!nat_entries.is_empty());
        assert!(nat_entries.iter().all(|e| e.module == "Std.Nat"));
    }

    #[test]
    fn test_all_theorems_nonempty() {
        let thms = all_theorems();
        assert!(!thms.is_empty());
        assert!(thms.iter().all(|e| e.is_theorem));
    }

    #[test]
    fn test_module_category_nat() {
        assert_eq!(module_category("Std.Nat"), StdCategory::Arithmetic);
    }

    #[test]
    fn test_module_category_list() {
        assert_eq!(module_category("Std.List"), StdCategory::Data);
    }

    #[test]
    fn test_module_category_io() {
        assert_eq!(module_category("Std.IO"), StdCategory::IO);
    }

    #[test]
    fn test_std_version_to_string() {
        let v = StdVersion {
            major: 1,
            minor: 2,
            patch: 3,
            pre: "",
        };
        assert_eq!(v.to_string(), "1.2.3");
    }

    #[test]
    fn test_std_version_prerelease_to_string() {
        let v = StdVersion {
            major: 0,
            minor: 1,
            patch: 0,
            pre: "alpha",
        };
        assert_eq!(v.to_string(), "0.1.0-alpha");
    }

    #[test]
    fn test_std_version_is_stable_false() {
        assert!(!StdVersion::CURRENT.is_stable());
    }

    #[test]
    fn test_std_category_label() {
        assert_eq!(StdCategory::Arithmetic.label(), "Arithmetic");
        assert_eq!(StdCategory::Logic.label(), "Logic");
        assert_eq!(StdCategory::Data.label(), "Data");
    }

    #[test]
    fn test_std_known_entries_nonempty() {
        assert!(!STD_KNOWN_ENTRIES.is_empty());
    }

    #[test]
    fn test_all_entries_have_module() {
        for e in STD_KNOWN_ENTRIES {
            assert!(!e.module.is_empty(), "Entry {} has empty module", e.name);
        }
    }
}
