//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::Decl;
use std::collections::{HashMap, HashSet};

/// A module containing declarations.
#[derive(Debug, Clone)]
pub struct Module {
    /// Module name
    pub name: String,
    /// Module path
    pub path: Vec<String>,
    /// Declarations in this module
    pub decls: Vec<Decl>,
    /// Imported modules
    pub imports: Vec<String>,
    /// Exported names
    pub exports: Vec<String>,
    /// Import specifications
    pub import_specs: Vec<ImportSpec>,
    /// Export specification
    pub export_spec: Option<ExportSpec>,
    /// Namespace scopes
    pub namespaces: Vec<NamespaceScope>,
    /// Open directives
    pub opens: Vec<OpenDirective>,
    /// Name visibility information
    pub visibility_map: HashMap<String, Visibility>,
    /// Module configuration
    pub config: ModuleConfig,
}
impl Module {
    /// Create a new module.
    pub fn new(name: String) -> Self {
        Self {
            name,
            path: Vec::new(),
            decls: Vec::new(),
            imports: Vec::new(),
            exports: Vec::new(),
            import_specs: Vec::new(),
            export_spec: None,
            namespaces: Vec::new(),
            opens: Vec::new(),
            visibility_map: HashMap::new(),
            config: ModuleConfig {
                allow_circular: false,
                private_by_default: false,
                allow_shadowing: false,
            },
        }
    }
    /// Create a module with configuration.
    pub fn with_config(name: String, config: ModuleConfig) -> Self {
        let mut module = Self::new(name);
        module.config = config;
        module
    }
    /// Add a declaration to this module.
    pub fn add_decl(&mut self, decl: Decl) {
        self.decls.push(decl);
    }
    /// Add an import.
    pub fn add_import(&mut self, module: String) {
        self.imports.push(module);
    }
    /// Add an export.
    pub fn add_export(&mut self, name: String) {
        self.exports.push(name);
    }
    /// Get the full path of this module.
    pub fn full_path(&self) -> String {
        if self.path.is_empty() {
            self.name.clone()
        } else {
            format!("{}.{}", self.path.join("."), self.name)
        }
    }
    /// Resolve a name within this module.
    ///
    /// Checks local declarations first, then imported modules via import specs,
    /// then namespace aliases.
    #[allow(dead_code)]
    pub fn resolve_name(&self, name: &str) -> ResolvedName {
        let local_names = self.declared_names();
        if local_names.contains(&name.to_string()) {
            return ResolvedName::Local(name.to_string());
        }
        for ns in &self.namespaces {
            if let Some(full) = ns.aliases.get(name) {
                return ResolvedName::Local(full.clone());
            }
        }
        let mut found_sources: Vec<String> = Vec::new();
        for spec in &self.import_specs {
            match spec {
                ImportSpec::All(module_name) => {
                    found_sources.push(module_name.clone());
                }
                ImportSpec::Selective(module_name, selected) => {
                    if selected.contains(&name.to_string()) {
                        found_sources.push(module_name.clone());
                    }
                }
                ImportSpec::Hiding(module_name, hidden) => {
                    if !hidden.contains(&name.to_string()) {
                        found_sources.push(module_name.clone());
                    }
                }
                ImportSpec::Renaming(module_name, renamings) => {
                    for (from, to) in renamings {
                        if to == name {
                            return ResolvedName::Imported {
                                module: module_name.clone(),
                                name: from.clone(),
                            };
                        }
                    }
                }
            }
        }
        if !found_sources.is_empty() {
            if found_sources.len() == 1 {
                return ResolvedName::Imported {
                    module: found_sources[0].clone(),
                    name: name.to_string(),
                };
            }
            return ResolvedName::Ambiguous(found_sources);
        }
        if !self.imports.is_empty() {
            let mut import_sources: Vec<String> = Vec::new();
            for imp in &self.imports {
                import_sources.push(imp.clone());
            }
            if import_sources.len() == 1 {
                return ResolvedName::Imported {
                    module: import_sources[0].clone(),
                    name: name.to_string(),
                };
            }
            if import_sources.len() > 1 {
                return ResolvedName::Ambiguous(import_sources);
            }
        }
        ResolvedName::NotFound
    }
    /// Get all names declared in this module.
    fn declared_names(&self) -> Vec<String> {
        let mut names = Vec::new();
        for decl in &self.decls {
            if let Some(name) = Self::decl_name(decl) {
                names.push(name);
            }
        }
        names
    }
    /// Extract the name from a declaration.
    fn decl_name(decl: &Decl) -> Option<String> {
        match decl {
            Decl::Axiom { name, .. } => Some(name.clone()),
            Decl::Definition { name, .. } => Some(name.clone()),
            Decl::Theorem { name, .. } => Some(name.clone()),
            Decl::Inductive { name, .. } => Some(name.clone()),
            Decl::Namespace { name, .. } => Some(name.clone()),
            Decl::Structure { name, .. } => Some(name.clone()),
            Decl::ClassDecl { name, .. } => Some(name.clone()),
            Decl::InstanceDecl {
                name, class_name, ..
            } => name.clone().or_else(|| Some(class_name.clone())),
            Decl::SectionDecl { name, .. } => Some(name.clone()),
            Decl::Mutual { .. } => None,
            Decl::Derive { type_name, .. } => Some(type_name.clone()),
            Decl::NotationDecl { name, .. } => Some(name.clone()),
            Decl::Universe { .. } => None,
            Decl::Import { .. }
            | Decl::Variable { .. }
            | Decl::Open { .. }
            | Decl::Attribute { .. }
            | Decl::HashCmd { .. } => None,
        }
    }
    /// Get all visible names (declared + imported).
    #[allow(dead_code)]
    pub fn visible_names(&self) -> Vec<String> {
        let mut names = self.declared_names();
        for ns in &self.namespaces {
            for alias in ns.aliases.keys() {
                if !names.contains(alias) {
                    names.push(alias.clone());
                }
            }
        }
        names
    }
    /// Get names visible to importers of this module.
    #[allow(dead_code)]
    pub fn exported_names(&self) -> Vec<String> {
        match &self.export_spec {
            Some(ExportSpec::All) => self.declared_names(),
            Some(ExportSpec::Selective(selected)) => {
                let declared = self.declared_names();
                selected
                    .iter()
                    .filter(|n| declared.contains(n))
                    .cloned()
                    .collect()
            }
            None => {
                if self.exports.is_empty() {
                    self.declared_names()
                } else {
                    self.exports.clone()
                }
            }
        }
    }
    /// Add a namespace scope.
    #[allow(dead_code)]
    pub fn add_namespace(&mut self, ns: NamespaceScope) {
        self.namespaces.push(ns);
    }
    /// Apply an import specification.
    #[allow(dead_code)]
    pub fn with_import_spec(&mut self, spec: ImportSpec) {
        let module_name = match &spec {
            ImportSpec::All(m) => m.clone(),
            ImportSpec::Selective(m, _) => m.clone(),
            ImportSpec::Hiding(m, _) => m.clone(),
            ImportSpec::Renaming(m, _) => m.clone(),
        };
        if !self.imports.contains(&module_name) {
            self.imports.push(module_name);
        }
        self.import_specs.push(spec);
    }
    /// Set visibility for a name.
    #[allow(dead_code)]
    pub fn set_visibility(&mut self, name: String, visibility: Visibility) {
        self.visibility_map.insert(name, visibility);
    }
    /// Get visibility of a name.
    #[allow(dead_code)]
    pub fn get_visibility(&self, name: &str) -> Visibility {
        self.visibility_map
            .get(name)
            .copied()
            .unwrap_or(if self.config.private_by_default {
                Visibility::Private
            } else {
                Visibility::Public
            })
    }
    /// Add an open directive.
    #[allow(dead_code)]
    pub fn add_open(&mut self, module: String, scoped: bool) {
        self.opens.push(OpenDirective { module, scoped });
    }
    /// Get all open modules.
    #[allow(dead_code)]
    pub fn get_opens(&self) -> Vec<String> {
        self.opens.iter().map(|o| o.module.clone()).collect()
    }
    /// Resolve a name considering all open modules and imports.
    #[allow(dead_code)]
    pub fn resolve_with_opens(&self, name: &str) -> ResolvedName {
        if let ResolvedName::Local(n) = self.resolve_name(name) {
            return ResolvedName::Local(n);
        }
        let mut found_sources = Vec::new();
        for open in &self.opens {
            found_sources.push(open.module.clone());
        }
        if !found_sources.is_empty() {
            if found_sources.len() == 1 {
                return ResolvedName::Imported {
                    module: found_sources[0].clone(),
                    name: name.to_string(),
                };
            }
            return ResolvedName::Ambiguous(found_sources);
        }
        self.resolve_name(name)
    }
    /// Get all visible names including from opens.
    #[allow(dead_code)]
    pub fn all_visible_names(&self) -> Vec<NameVisibility> {
        let mut result = Vec::new();
        let declared = self.declared_names();
        for name in declared {
            let visibility = self.get_visibility(&name);
            result.push(NameVisibility {
                name,
                visibility,
                from_module: None,
            });
        }
        for open in &self.opens {
            result.push(NameVisibility {
                name: format!("{}::*", open.module),
                visibility: Visibility::Public,
                from_module: Some(open.module.clone()),
            });
        }
        result
    }
    /// Check if a name is accessible with given visibility context.
    #[allow(dead_code)]
    pub fn is_accessible(&self, name: &str, from_same_module: bool) -> bool {
        let visibility = self.get_visibility(name);
        match visibility {
            Visibility::Public => true,
            Visibility::Private => from_same_module,
            Visibility::Protected => true,
        }
    }
    /// Get all direct dependencies (imports and opens).
    #[allow(dead_code)]
    pub fn get_dependencies(&self) -> HashSet<String> {
        let mut deps = HashSet::new();
        deps.extend(self.imports.clone());
        for open in &self.opens {
            deps.insert(open.module.clone());
        }
        for spec in &self.import_specs {
            let mod_name = match spec {
                ImportSpec::All(m) => m.clone(),
                ImportSpec::Selective(m, _) => m.clone(),
                ImportSpec::Hiding(m, _) => m.clone(),
                ImportSpec::Renaming(m, _) => m.clone(),
            };
            deps.insert(mod_name);
        }
        deps
    }
    /// Check if this module directly imports another.
    #[allow(dead_code)]
    pub fn imports_module(&self, other: &str) -> bool {
        self.get_dependencies().contains(other)
    }
    /// Resolve a selective import to actual names.
    #[allow(dead_code)]
    pub fn resolve_selective_import(&self, _module: &str, selected: &[String]) -> Vec<String> {
        selected.to_vec()
    }
    /// Get all names hidden by a hiding import.
    #[allow(dead_code)]
    pub fn get_hidden_names(&self, module: &str) -> Vec<String> {
        for spec in &self.import_specs {
            if let ImportSpec::Hiding(m, hidden) = spec {
                if m == module {
                    return hidden.clone();
                }
            }
        }
        Vec::new()
    }
    /// Create a nested namespace scope.
    #[allow(dead_code)]
    pub fn create_nested_namespace(&mut self, name: String, parent: NamespaceScope) {
        let mut nested = NamespaceScope {
            name,
            opened: Vec::new(),
            aliases: HashMap::new(),
            visibility: HashMap::new(),
            parent: Some(Box::new(parent)),
        };
        if let Some(ref p) = nested.parent {
            nested.aliases.extend(p.aliases.clone());
        }
        self.namespaces.push(nested);
    }
    /// Look up a name in namespace hierarchy.
    #[allow(dead_code)]
    pub fn lookup_in_namespaces(&self, name: &str) -> Option<String> {
        for ns in self.namespaces.iter().rev() {
            if let Some(full) = ns.aliases.get(name) {
                return Some(full.clone());
            }
            let mut current_parent = &ns.parent;
            while let Some(ref parent) = current_parent {
                if let Some(full) = parent.aliases.get(name) {
                    return Some(full.clone());
                }
                current_parent = &parent.parent;
            }
        }
        None
    }
    /// Get all names in a specific namespace.
    #[allow(dead_code)]
    pub fn namespace_names(&self, ns_name: &str) -> Vec<String> {
        for ns in &self.namespaces {
            if ns.name == ns_name {
                return ns.aliases.keys().cloned().collect();
            }
        }
        Vec::new()
    }
    /// Set default visibility for all names (for private_by_default config).
    #[allow(dead_code)]
    pub fn set_default_visibility(&mut self, visibility: Visibility) {
        match visibility {
            Visibility::Private => self.config.private_by_default = true,
            _ => self.config.private_by_default = false,
        }
    }
}
/// Resolution result for a name.
#[derive(Debug, Clone, PartialEq)]
pub enum ResolvedName {
    /// Name found in local declarations
    Local(String),
    /// Name found in an imported module
    Imported {
        /// Module the name comes from
        module: String,
        /// The resolved name
        name: String,
    },
    /// Name is ambiguous (found in multiple sources)
    Ambiguous(Vec<String>),
    /// Name was not found
    NotFound,
}
/// A module attribute (like #\[inline\], @\[simp\], etc.).
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Clone)]
pub struct ModuleAttribute {
    /// Attribute name
    pub name: String,
    /// Optional argument
    pub arg: Option<String>,
}
impl ModuleAttribute {
    /// Create a new attribute.
    #[allow(dead_code)]
    pub fn new(name: &str) -> Self {
        ModuleAttribute {
            name: name.to_string(),
            arg: None,
        }
    }
    /// Set an argument.
    #[allow(dead_code)]
    pub fn with_arg(mut self, arg: &str) -> Self {
        self.arg = Some(arg.to_string());
        self
    }
    /// Format the attribute.
    #[allow(dead_code)]
    pub fn format(&self) -> String {
        if let Some(ref arg) = self.arg {
            format!("@[{} {}]", self.name, arg)
        } else {
            format!("@[{}]", self.name)
        }
    }
}
/// Export specification.
#[derive(Debug, Clone, PartialEq)]
pub enum ExportSpec {
    /// Export all names
    All,
    /// Export only selected names
    Selective(Vec<String>),
}
/// Visibility information for a name.
#[derive(Debug, Clone)]
pub struct NameVisibility {
    /// Name itself
    pub name: String,
    /// Visibility level
    pub visibility: Visibility,
    /// Module it's from
    pub from_module: Option<String>,
}
/// Module dependency graph node.
#[derive(Debug, Clone)]
pub struct ModuleDep {
    /// Module name
    pub name: String,
    /// Direct dependencies
    pub deps: Vec<String>,
}
/// A module export entry.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Clone)]
pub struct ExportEntry {
    /// Module being re-exported
    pub module: String,
    /// Whether all declarations are re-exported
    pub all: bool,
    /// Specific names to export (if not all)
    pub names: Vec<String>,
}
impl ExportEntry {
    /// Create an "export all" entry.
    #[allow(dead_code)]
    pub fn all(module: &str) -> Self {
        ExportEntry {
            module: module.to_string(),
            all: true,
            names: Vec::new(),
        }
    }
    /// Create an entry for specific names.
    #[allow(dead_code)]
    pub fn names(module: &str, names: Vec<&str>) -> Self {
        ExportEntry {
            module: module.to_string(),
            all: false,
            names: names.into_iter().map(|s| s.to_string()).collect(),
        }
    }
}
/// A module version record.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModuleVersion {
    /// Major version
    pub major: u32,
    /// Minor version
    pub minor: u32,
    /// Patch version
    pub patch: u32,
}
impl ModuleVersion {
    /// Create a new version.
    #[allow(dead_code)]
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        ModuleVersion {
            major,
            minor,
            patch,
        }
    }
    /// Format as "major.minor.patch".
    #[allow(dead_code)]
    pub fn format(&self) -> String {
        format!("{}.{}.{}", self.major, self.minor, self.patch)
    }
}
/// Module configuration settings.
#[derive(Debug, Clone)]
pub struct ModuleConfig {
    /// Allow circular imports
    pub allow_circular: bool,
    /// Private by default
    pub private_by_default: bool,
    /// Enable shadowing
    pub allow_shadowing: bool,
}
/// Import specification.
#[derive(Debug, Clone, PartialEq)]
pub enum ImportSpec {
    /// Import everything: `import Foo`
    All(String),
    /// Import specific names: `import Foo (bar, baz)`
    Selective(String, Vec<String>),
    /// Import hiding names: `import Foo hiding (bar)`
    Hiding(String, Vec<String>),
    /// Import with renaming: `import Foo renaming bar -> baz`
    Renaming(String, Vec<(String, String)>),
}
/// Visibility modifier for declarations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Visibility {
    /// Public - visible outside module
    Public,
    /// Private - only visible within module
    Private,
    /// Protected - visible in submodules
    Protected,
}
/// Scoped open with body expression.
#[derive(Debug, Clone)]
pub struct ScopedOpen {
    /// Module to open
    pub module: String,
    /// Name of the scope variable
    pub scope_var: Option<String>,
}
/// Open directive with optional scope.
#[derive(Debug, Clone, PartialEq)]
pub struct OpenDirective {
    /// Module to open
    pub module: String,
    /// Optional scope: Some(expr) means `open ... in expr`
    pub scoped: bool,
}
/// A module dependency graph.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct DepGraphExt {
    /// All edges
    pub edges: Vec<ModuleDepExt>,
}
impl DepGraphExt {
    /// Create a new empty dependency graph.
    #[allow(dead_code)]
    pub fn new() -> Self {
        DepGraphExt { edges: Vec::new() }
    }
    /// Add an edge.
    #[allow(dead_code)]
    pub fn add(&mut self, dep: ModuleDepExt) {
        self.edges.push(dep);
    }
    /// Returns all direct dependencies of a module.
    #[allow(dead_code)]
    pub fn direct_deps(&self, module: &str) -> Vec<&str> {
        self.edges
            .iter()
            .filter(|e| e.from == module && e.direct)
            .map(|e| e.to.as_str())
            .collect()
    }
    /// Returns all modules that depend on a given module.
    #[allow(dead_code)]
    pub fn dependents(&self, module: &str) -> Vec<&str> {
        self.edges
            .iter()
            .filter(|e| e.to == module)
            .map(|e| e.from.as_str())
            .collect()
    }
    /// Checks for cycles using DFS (simplified: just checks direct self-import).
    #[allow(dead_code)]
    pub fn has_self_import(&self) -> bool {
        self.edges.iter().any(|e| e.from == e.to)
    }
}
/// A module metadata record.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Clone)]
pub struct ModuleMetadata {
    /// Module name
    pub name: String,
    /// Version
    pub version: Option<ModuleVersion>,
    /// Author
    pub author: Option<String>,
}
impl ModuleMetadata {
    /// Create a new metadata record.
    #[allow(dead_code)]
    pub fn new(name: &str) -> Self {
        ModuleMetadata {
            name: name.to_string(),
            version: None,
            author: None,
        }
    }
    /// Set the version.
    #[allow(dead_code)]
    pub fn with_version(mut self, v: ModuleVersion) -> Self {
        self.version = Some(v);
        self
    }
    /// Set the author.
    #[allow(dead_code)]
    pub fn with_author(mut self, a: &str) -> Self {
        self.author = Some(a.to_string());
        self
    }
}
/// A module dependency edge.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Clone)]
pub struct ModuleDepExt {
    /// The importing module name
    pub from: String,
    /// The imported module name
    pub to: String,
    /// Whether the import is direct or transitive
    pub direct: bool,
}
impl ModuleDepExt {
    /// Create a new direct dependency.
    #[allow(dead_code)]
    pub fn direct(from: &str, to: &str) -> Self {
        ModuleDepExt {
            from: from.to_string(),
            to: to.to_string(),
            direct: true,
        }
    }
    /// Create a new transitive dependency.
    #[allow(dead_code)]
    pub fn transitive(from: &str, to: &str) -> Self {
        ModuleDepExt {
            from: from.to_string(),
            to: to.to_string(),
            direct: false,
        }
    }
}
/// Import cycle detection result.
#[derive(Debug, Clone, PartialEq)]
pub enum CycleDetectionResult {
    /// No cycles found
    NoCycles,
    /// Cycle found involving these modules
    CycleFound(Vec<String>),
    /// Multiple cycles found
    MultipleCycles(Vec<Vec<String>>),
}
/// Module registry for managing multiple modules.
pub struct ModuleRegistry {
    /// Registered modules
    modules: HashMap<String, Module>,
}
impl ModuleRegistry {
    /// Create a new module registry.
    pub fn new() -> Self {
        Self {
            modules: HashMap::new(),
        }
    }
    /// Register a module.
    pub fn register(&mut self, module: Module) -> Result<(), String> {
        let path = module.full_path();
        match self.modules.entry(path.clone()) {
            std::collections::hash_map::Entry::Occupied(_) => {
                Err(format!("Module {} already registered", path))
            }
            std::collections::hash_map::Entry::Vacant(entry) => {
                entry.insert(module);
                Ok(())
            }
        }
    }
    /// Get a module by path.
    pub fn get(&self, path: &str) -> Option<&Module> {
        self.modules.get(path)
    }
    /// Get all registered modules.
    pub fn all_modules(&self) -> Vec<&Module> {
        self.modules.values().collect()
    }
    /// Check if a module is registered.
    pub fn contains(&self, path: &str) -> bool {
        self.modules.contains_key(path)
    }
    /// Get a mutable reference to a module by path.
    #[allow(dead_code)]
    pub fn get_mut(&mut self, path: &str) -> Option<&mut Module> {
        self.modules.get_mut(path)
    }
    /// Remove and return a module from the registry.
    #[allow(dead_code)]
    pub fn unregister(&mut self, path: &str) -> Option<Module> {
        self.modules.remove(path)
    }
    /// Resolve a name from the perspective of a given module.
    ///
    /// Checks the module's local declarations first, then searches
    /// through its imports using import specifications.
    #[allow(dead_code)]
    pub fn resolve(&self, from: &str, name: &str) -> ResolvedName {
        let module = match self.modules.get(from) {
            Some(m) => m,
            None => return ResolvedName::NotFound,
        };
        let local = module.resolve_name(name);
        if matches!(local, ResolvedName::Local(_)) {
            return local;
        }
        for spec in &module.import_specs {
            match spec {
                ImportSpec::All(mod_name) => {
                    if let Some(target) = self.modules.get(mod_name) {
                        let exported = target.exported_names();
                        if exported.contains(&name.to_string()) {
                            return ResolvedName::Imported {
                                module: mod_name.clone(),
                                name: name.to_string(),
                            };
                        }
                    }
                }
                ImportSpec::Selective(mod_name, selected) => {
                    if selected.contains(&name.to_string()) {
                        if let Some(target) = self.modules.get(mod_name) {
                            let exported = target.exported_names();
                            if exported.contains(&name.to_string()) {
                                return ResolvedName::Imported {
                                    module: mod_name.clone(),
                                    name: name.to_string(),
                                };
                            }
                        }
                    }
                }
                ImportSpec::Hiding(mod_name, hidden) => {
                    if !hidden.contains(&name.to_string()) {
                        if let Some(target) = self.modules.get(mod_name) {
                            let exported = target.exported_names();
                            if exported.contains(&name.to_string()) {
                                return ResolvedName::Imported {
                                    module: mod_name.clone(),
                                    name: name.to_string(),
                                };
                            }
                        }
                    }
                }
                ImportSpec::Renaming(mod_name, renamings) => {
                    for (from_name, to_name) in renamings {
                        if to_name == name {
                            return ResolvedName::Imported {
                                module: mod_name.clone(),
                                name: from_name.clone(),
                            };
                        }
                    }
                }
            }
        }
        for imp in &module.imports {
            if let Some(target) = self.modules.get(imp) {
                let exported = target.exported_names();
                if exported.contains(&name.to_string()) {
                    return ResolvedName::Imported {
                        module: imp.clone(),
                        name: name.to_string(),
                    };
                }
            }
        }
        ResolvedName::NotFound
    }
    /// Compute a topological sort of modules based on their dependencies.
    ///
    /// Returns `Ok(order)` with module names in dependency order (dependencies first),
    /// or `Err(cycle)` with the names of modules involved in a cycle.
    #[allow(dead_code)]
    pub fn dependency_order(&self) -> Result<Vec<String>, Vec<String>> {
        let deps = self.build_dep_graph();
        let mut in_degree: HashMap<String, usize> = HashMap::new();
        let mut adj: HashMap<String, Vec<String>> = HashMap::new();
        for dep in &deps {
            in_degree.entry(dep.name.clone()).or_insert(0);
            adj.entry(dep.name.clone()).or_default();
            for d in &dep.deps {
                adj.entry(d.clone()).or_default();
                in_degree.entry(d.clone()).or_insert(0);
                *in_degree.entry(dep.name.clone()).or_insert(0) += 1;
            }
        }
        let mut queue: Vec<String> = in_degree
            .iter()
            .filter(|(_, &deg)| deg == 0)
            .map(|(name, _)| name.clone())
            .collect();
        queue.sort();
        let mut result = Vec::new();
        while let Some(node) = queue.pop() {
            result.push(node.clone());
            if let Some(neighbors) = adj.get(&node) {
                for _neighbor in neighbors {}
                let _ = neighbors;
            }
        }
        let mut adj2: HashMap<String, Vec<String>> = HashMap::new();
        let mut in_deg2: HashMap<String, usize> = HashMap::new();
        for dep in &deps {
            in_deg2.entry(dep.name.clone()).or_insert(0);
            adj2.entry(dep.name.clone()).or_default();
            for d in &dep.deps {
                adj2.entry(d.clone()).or_default();
                in_deg2.entry(d.clone()).or_insert(0);
                adj2.entry(d.clone()).or_default().push(dep.name.clone());
                *in_deg2.entry(dep.name.clone()).or_insert(0) += 1;
            }
        }
        let mut queue2: Vec<String> = in_deg2
            .iter()
            .filter(|(_, &deg)| deg == 0)
            .map(|(name, _)| name.clone())
            .collect();
        queue2.sort();
        let mut sorted = Vec::new();
        while let Some(node) = queue2.pop() {
            sorted.push(node.clone());
            if let Some(neighbors) = adj2.get(&node) {
                for neighbor in neighbors.clone() {
                    if let Some(deg) = in_deg2.get_mut(&neighbor) {
                        *deg -= 1;
                        if *deg == 0 {
                            queue2.push(neighbor);
                            queue2.sort();
                        }
                    }
                }
            }
        }
        let total_nodes = in_deg2.len();
        if sorted.len() == total_nodes {
            Ok(sorted)
        } else {
            let cycle: Vec<String> = in_deg2
                .iter()
                .filter(|(_, &deg)| deg > 0)
                .map(|(name, _)| name.clone())
                .collect();
            Err(cycle)
        }
    }
    /// Detect cycles in the module dependency graph.
    ///
    /// Returns a list of cycles, where each cycle is a list of module names.
    #[allow(dead_code)]
    pub fn detect_cycles(&self) -> Vec<Vec<String>> {
        let deps = self.build_dep_graph();
        let mut visited: HashMap<String, u8> = HashMap::new();
        let mut path: Vec<String> = Vec::new();
        let mut cycles: Vec<Vec<String>> = Vec::new();
        let dep_map: HashMap<String, Vec<String>> = deps
            .iter()
            .map(|d| (d.name.clone(), d.deps.clone()))
            .collect();
        for dep in &deps {
            if visited.get(&dep.name).copied().unwrap_or(0) == 0 {
                Self::dfs_cycle(&dep.name, &dep_map, &mut visited, &mut path, &mut cycles);
            }
        }
        cycles
    }
    /// DFS helper for cycle detection.
    fn dfs_cycle(
        node: &str,
        adj: &HashMap<String, Vec<String>>,
        visited: &mut HashMap<String, u8>,
        path: &mut Vec<String>,
        cycles: &mut Vec<Vec<String>>,
    ) {
        visited.insert(node.to_string(), 1);
        path.push(node.to_string());
        if let Some(neighbors) = adj.get(node) {
            for neighbor in neighbors {
                let state = visited.get(neighbor).copied().unwrap_or(0);
                if state == 1 {
                    if let Some(pos) = path.iter().position(|n| n == neighbor) {
                        let cycle: Vec<String> = path[pos..].to_vec();
                        cycles.push(cycle);
                    }
                } else if state == 0 {
                    Self::dfs_cycle(neighbor, adj, visited, path, cycles);
                }
            }
        }
        path.pop();
        visited.insert(node.to_string(), 2);
    }
    /// Get all transitive dependencies of a module.
    #[allow(dead_code)]
    pub fn transitive_deps(&self, module: &str) -> Vec<String> {
        let dep_map: HashMap<String, Vec<String>> = self
            .build_dep_graph()
            .into_iter()
            .map(|d| (d.name, d.deps))
            .collect();
        let mut visited = Vec::new();
        let mut stack = Vec::new();
        if let Some(direct) = dep_map.get(module) {
            stack.extend(direct.clone());
        }
        while let Some(dep) = stack.pop() {
            if visited.contains(&dep) {
                continue;
            }
            visited.push(dep.clone());
            if let Some(transitive) = dep_map.get(&dep) {
                for t in transitive {
                    if !visited.contains(t) {
                        stack.push(t.clone());
                    }
                }
            }
        }
        visited.sort();
        visited
    }
    /// Build the dependency graph from the registered modules.
    fn build_dep_graph(&self) -> Vec<ModuleDep> {
        self.modules
            .values()
            .map(|m| {
                let mut deps = m.imports.clone();
                for spec in &m.import_specs {
                    let mod_name = match spec {
                        ImportSpec::All(n) => n.clone(),
                        ImportSpec::Selective(n, _) => n.clone(),
                        ImportSpec::Hiding(n, _) => n.clone(),
                        ImportSpec::Renaming(n, _) => n.clone(),
                    };
                    if !deps.contains(&mod_name) {
                        deps.push(mod_name);
                    }
                }
                ModuleDep {
                    name: m.full_path(),
                    deps,
                }
            })
            .collect()
    }
    /// Detect mutual (cyclic) imports between modules.
    #[allow(dead_code)]
    pub fn detect_mutual_imports(&self) -> Vec<(String, String)> {
        let mut mutual = Vec::new();
        for module in self.modules.values() {
            for dep in &module.imports {
                if let Some(dep_module) = self.modules.get(dep) {
                    if dep_module.imports.contains(&module.full_path()) {
                        let pair = (module.full_path(), dep.clone());
                        if !mutual.iter().any(|(a, b)| {
                            (a == &pair.0 && b == &pair.1) || (a == &pair.1 && b == &pair.0)
                        }) {
                            mutual.push(pair);
                        }
                    }
                }
            }
        }
        mutual
    }
    /// Check if two modules import each other.
    #[allow(dead_code)]
    pub fn are_mutually_dependent(&self, a: &str, b: &str) -> bool {
        if let (Some(mod_a), Some(mod_b)) = (self.modules.get(a), self.modules.get(b)) {
            (mod_a.imports.contains(&b.to_string()) && mod_b.imports.contains(&a.to_string()))
                || (mod_a.imports_module(b) && mod_b.imports_module(a))
        } else {
            false
        }
    }
    /// Get all modules that depend on a given module.
    #[allow(dead_code)]
    pub fn get_dependents(&self, module_name: &str) -> Vec<String> {
        let mut dependents = Vec::new();
        for module in self.modules.values() {
            if module.imports_module(module_name) {
                dependents.push(module.full_path());
            }
        }
        dependents.sort();
        dependents
    }
    /// Get all modules that a given module depends on (indirect + direct).
    #[allow(dead_code)]
    pub fn get_all_dependencies(&self, module_name: &str) -> Vec<String> {
        let mut all_deps = HashSet::new();
        let mut stack = vec![module_name.to_string()];
        let mut visited = HashSet::new();
        while let Some(current) = stack.pop() {
            if visited.contains(&current) {
                continue;
            }
            visited.insert(current.clone());
            if let Some(module) = self.modules.get(&current) {
                for dep in module.get_dependencies() {
                    if !visited.contains(&dep) {
                        all_deps.insert(dep.clone());
                        stack.push(dep);
                    }
                }
            }
        }
        let mut result: Vec<_> = all_deps.into_iter().collect();
        result.sort();
        result
    }
    /// Verify module consistency (no broken imports).
    #[allow(dead_code)]
    pub fn verify_consistency(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();
        for module in self.modules.values() {
            for imported in &module.imports {
                if !self.modules.contains_key(imported) {
                    errors.push(format!(
                        "Module {} imports non-existent module {}",
                        module.full_path(),
                        imported
                    ));
                }
            }
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
    /// Get all modules that would be transitively affected by changes to a module.
    #[allow(dead_code)]
    pub fn get_reverse_dependencies(&self, module_name: &str) -> Vec<String> {
        let mut affected = HashSet::new();
        let mut stack = vec![module_name.to_string()];
        let mut visited = HashSet::new();
        while let Some(current) = stack.pop() {
            if visited.contains(&current) {
                continue;
            }
            visited.insert(current.clone());
            for dependent in self.get_dependents(&current) {
                if !visited.contains(&dependent) {
                    affected.insert(dependent.clone());
                    stack.push(dependent);
                }
            }
        }
        let mut result: Vec<_> = affected.into_iter().collect();
        result.sort();
        result
    }
    /// Reachability analysis: which modules can be reached from a given module.
    #[allow(dead_code)]
    pub fn reachable_from(&self, module_name: &str) -> Vec<String> {
        let mut reachable = Vec::new();
        let mut visited = HashSet::new();
        let mut stack = vec![module_name.to_string()];
        while let Some(current) = stack.pop() {
            if visited.contains(&current) {
                continue;
            }
            visited.insert(current.clone());
            if let Some(module) = self.modules.get(&current) {
                for dep in module.get_dependencies() {
                    if !visited.contains(&dep) {
                        reachable.push(dep.clone());
                        stack.push(dep);
                    }
                }
            }
        }
        reachable.sort();
        reachable
    }
    /// Get module strongly connected components (for circular dependency analysis).
    #[allow(dead_code)]
    pub fn get_sccs(&self) -> Vec<Vec<String>> {
        let mut sccs = Vec::new();
        let deps = self.build_dep_graph();
        let mut visited: HashMap<String, bool> = HashMap::new();
        let mut rec_stack: HashMap<String, bool> = HashMap::new();
        let mut current_scc = Vec::new();
        for dep in &deps {
            if !visited.contains_key(&dep.name) {
                self.tarjan_visit(
                    &dep.name,
                    &deps,
                    &mut visited,
                    &mut rec_stack,
                    &mut current_scc,
                    &mut sccs,
                );
            }
        }
        sccs
    }
    /// Tarjan's algorithm helper for SCC detection.
    fn tarjan_visit(
        &self,
        node: &str,
        _graph: &[ModuleDep],
        visited: &mut HashMap<String, bool>,
        rec_stack: &mut HashMap<String, bool>,
        _current_scc: &mut Vec<String>,
        sccs: &mut Vec<Vec<String>>,
    ) {
        visited.insert(node.to_string(), true);
        rec_stack.insert(node.to_string(), true);
        if let Some(module) = self.modules.get(node) {
            for dep in &module.imports {
                if !visited.get(dep).copied().unwrap_or(false) {
                    self.tarjan_visit(dep, _graph, visited, rec_stack, _current_scc, sccs);
                } else if rec_stack.get(dep).copied().unwrap_or(false) {
                    sccs.push(vec![node.to_string(), dep.clone()]);
                }
            }
        }
        rec_stack.insert(node.to_string(), false);
    }
    /// Get import statistics for debugging.
    #[allow(dead_code)]
    pub fn get_statistics(&self) -> HashMap<String, usize> {
        let mut stats = HashMap::new();
        let mut total_imports = 0;
        let mut total_opens = 0;
        for module in self.modules.values() {
            total_imports += module.imports.len();
            total_opens += module.opens.len();
        }
        stats.insert("modules".to_string(), self.modules.len());
        stats.insert("total_imports".to_string(), total_imports);
        stats.insert("total_opens".to_string(), total_opens);
        stats
    }
    /// Export a module's public interface.
    #[allow(dead_code)]
    pub fn get_public_interface(&self, module_name: &str) -> Vec<String> {
        if let Some(module) = self.modules.get(module_name) {
            module
                .exported_names()
                .into_iter()
                .filter(|name| module.get_visibility(name) == Visibility::Public)
                .collect()
        } else {
            Vec::new()
        }
    }
    /// Check if name is accessible from module A in module B.
    #[allow(dead_code)]
    pub fn is_name_accessible(&self, name: &str, from_module: &str, to_module: &str) -> bool {
        if let Some(target) = self.modules.get(to_module) {
            let exported = target.exported_names();
            if exported.contains(&name.to_string()) {
                return target.is_accessible(name, from_module == to_module);
            }
        }
        false
    }
}
/// A namespace scope.
#[derive(Debug, Clone)]
pub struct NamespaceScope {
    /// Namespace name
    pub name: String,
    /// Opened namespaces within this scope
    pub opened: Vec<String>,
    /// Aliases (short name -> full name)
    pub aliases: HashMap<String, String>,
    /// Visibility map for names in this namespace
    pub visibility: HashMap<String, Visibility>,
    /// Parent namespace (for nested scopes)
    pub parent: Option<Box<NamespaceScope>>,
}
