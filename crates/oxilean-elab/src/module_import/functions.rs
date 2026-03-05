//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{Expr, Name};
use std::collections::{HashMap, HashSet, VecDeque};
use std::path::PathBuf;

use super::types::{
    DependencyGraph, ExportInfo, ImportConflict, ImportDecl, ImportError, ImportFilter,
    ImportResolver, ImportResult, ImportSession, ImportStats, ModuleData, ModuleEnv, ModuleHeader,
    ModuleLoadBatch, ModuleLoadOutcome, ModulePath, ResolvedImport, Visibility,
};

/// Resolve all transitive imports for a module.
///
/// Starting from the given module, follow all import chains and collect
/// the full set of names available to the module.
pub fn resolve_transitive_imports(
    module_path: &ModulePath,
    env: &ModuleEnv,
    graph: &DependencyGraph,
) -> ImportResult {
    let trans_deps = graph.transitive_deps(module_path);
    let mut result = ImportResult::new();
    for dep_path in &trans_deps {
        if let Some(module_data) = env.get_module(dep_path) {
            for (name, export_info) in module_data.public_exports() {
                result.resolved_names.push(ResolvedImport {
                    local_name: name.clone(),
                    original_name: name.clone(),
                    source_module: dep_path.clone(),
                    ty: export_info.ty.clone(),
                    is_reexport: true,
                });
            }
        }
    }
    if let Some(module_data) = env.get_module(module_path) {
        let resolver = ImportResolver::new(env);
        let direct = resolver.resolve_all(&module_data.header.imports);
        result.merge(direct);
    }
    result
}
/// Check for import conflicts (ambiguous names) in a list of resolved imports.
///
/// Returns a list of conflicts where the same local name is provided by
/// multiple source modules.
pub fn check_import_conflicts(imports: &[ResolvedImport]) -> Vec<ImportConflict> {
    let mut name_sources: HashMap<&Name, Vec<&ModulePath>> = HashMap::new();
    for imp in imports {
        name_sources
            .entry(&imp.local_name)
            .or_default()
            .push(&imp.source_module);
    }
    let mut conflicts = Vec::new();
    for (name, sources) in &name_sources {
        let unique: Vec<&ModulePath> = {
            let mut seen = HashSet::new();
            sources
                .iter()
                .filter(|p| seen.insert((**p).clone()))
                .copied()
                .collect()
        };
        if unique.len() > 1 {
            for i in 0..unique.len() {
                for j in (i + 1)..unique.len() {
                    conflicts.push(ImportConflict::new(
                        (*name).clone(),
                        unique[i].clone(),
                        unique[j].clone(),
                    ));
                }
            }
        }
    }
    conflicts
}
/// Deduplicate resolved imports by keeping the first occurrence of each name.
pub fn deduplicate_imports(imports: Vec<ResolvedImport>) -> Vec<ResolvedImport> {
    let mut seen = HashSet::new();
    imports
        .into_iter()
        .filter(|imp| seen.insert(imp.local_name.clone()))
        .collect()
}
/// Determine the load order for a set of modules.
///
/// Uses the dependency graph to produce a topological order.
pub fn compute_load_order(env: &ModuleEnv) -> Result<Vec<ModulePath>, ImportError> {
    let graph = DependencyGraph::from_env(env);
    match graph.topological_sort() {
        Ok(order) => Ok(order),
        Err(cycle) => Err(ImportError::CircularImport { cycle }),
    }
}
/// Validate that all imports of a module can be resolved.
pub fn validate_imports(header: &ModuleHeader, env: &ModuleEnv) -> Vec<ImportError> {
    let mut errors = Vec::new();
    for import in &header.imports {
        if !env.contains_module(&import.path) {
            errors.push(ImportError::ModuleNotFound(import.path.clone()));
        }
    }
    errors
}
/// Build a `BTreeMap` of all names available to a module after import resolution.
///
/// The map is sorted for deterministic output.
pub fn build_name_table(
    module_path: &ModulePath,
    env: &ModuleEnv,
    graph: &DependencyGraph,
) -> HashMap<Name, ResolvedImport> {
    let result = resolve_transitive_imports(module_path, env, graph);
    let deduped = deduplicate_imports(result.resolved_names);
    let mut table = HashMap::new();
    for imp in deduped {
        table.insert(imp.local_name.clone(), imp);
    }
    table
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::module_import::*;
    use oxilean_kernel::Level;
    fn mk_path(s: &str) -> ModulePath {
        ModulePath::parse_dot_separated(s)
    }
    fn mk_name(s: &str) -> Name {
        Name::str(s)
    }
    fn mk_module(path: &str, exports: &[(&str, Visibility)]) -> ModuleData {
        let module_path = mk_path(path);
        let header = ModuleHeader::new(module_path.clone());
        let mut data = ModuleData::new(module_path.clone(), header);
        for (name, vis) in exports {
            data.add_export(
                mk_name(name),
                ExportInfo::new(mk_name(name), *vis, module_path.clone()),
            );
        }
        data
    }
    fn mk_env_with_modules(modules: Vec<ModuleData>) -> ModuleEnv {
        let mut env = ModuleEnv::new();
        for m in modules {
            env.add_module(m);
        }
        env
    }
    #[test]
    fn test_module_path_from_str() {
        let path = mk_path("Mathlib.Data.Nat");
        assert_eq!(path.0, vec!["Mathlib", "Data", "Nat"]);
    }
    #[test]
    fn test_module_path_to_file_path() {
        let path = mk_path("Mathlib.Data.Nat");
        let fp = path.to_file_path();
        assert_eq!(fp, PathBuf::from("Mathlib/Data/Nat.lean"));
    }
    #[test]
    fn test_module_path_parent() {
        let path = mk_path("Mathlib.Data.Nat");
        let parent = path.parent().expect("test operation should succeed");
        assert_eq!(parent, mk_path("Mathlib.Data"));
        let root = mk_path("Mathlib");
        assert!(root.parent().is_none());
    }
    #[test]
    fn test_module_path_leaf() {
        let path = mk_path("Mathlib.Data.Nat");
        assert_eq!(path.leaf(), Some("Nat"));
    }
    #[test]
    fn test_module_path_is_prefix_of() {
        let a = mk_path("Mathlib.Data");
        let b = mk_path("Mathlib.Data.Nat");
        assert!(a.is_prefix_of(&b));
        assert!(!b.is_prefix_of(&a));
        assert!(a.is_prefix_of(&a));
        assert!(a.is_strict_prefix_of(&b));
        assert!(!a.is_strict_prefix_of(&a));
    }
    #[test]
    fn test_module_path_child() {
        let path = mk_path("Mathlib.Data");
        let child = path.child("Nat");
        assert_eq!(child, mk_path("Mathlib.Data.Nat"));
    }
    #[test]
    fn test_module_path_join() {
        let a = mk_path("Mathlib");
        let b = mk_path("Data.Nat");
        assert_eq!(a.join(&b), mk_path("Mathlib.Data.Nat"));
    }
    #[test]
    fn test_module_path_depth() {
        assert_eq!(mk_path("A.B.C").depth(), 3);
        assert_eq!(mk_path("A").depth(), 1);
    }
    #[test]
    fn test_module_path_display() {
        assert_eq!(format!("{}", mk_path("A.B.C")), "A.B.C");
    }
    #[test]
    fn test_module_path_to_name() {
        assert_eq!(mk_path("A.B").to_name(), mk_name("A.B"));
    }
    #[test]
    fn test_module_path_root() {
        assert_eq!(mk_path("A.B.C").root(), Some("A"));
    }
    #[test]
    fn test_module_path_strip_prefix() {
        let path = mk_path("A.B.C.D");
        let prefix = mk_path("A.B");
        let relative = path
            .strip_prefix(&prefix)
            .expect("test operation should succeed");
        assert_eq!(relative, mk_path("C.D"));
        let no_prefix = mk_path("X.Y");
        assert!(path.strip_prefix(&no_prefix).is_none());
    }
    #[test]
    fn test_module_path_to_file_path_with_ext() {
        let path = mk_path("A.B.C");
        assert_eq!(
            path.to_file_path_with_ext("olean"),
            PathBuf::from("A/B/C.olean")
        );
    }
    #[test]
    fn test_visibility() {
        assert!(Visibility::Public.is_public());
        assert!(!Visibility::Protected.is_public());
        assert!(!Visibility::Private.is_public());
        assert!(Visibility::Public.is_at_least_protected());
        assert!(Visibility::Protected.is_at_least_protected());
        assert!(!Visibility::Private.is_at_least_protected());
    }
    #[test]
    fn test_visibility_display() {
        assert_eq!(format!("{}", Visibility::Public), "public");
        assert_eq!(format!("{}", Visibility::Protected), "protected");
        assert_eq!(format!("{}", Visibility::Private), "private");
    }
    #[test]
    fn test_export_info() {
        let ei = ExportInfo::new(mk_name("foo"), Visibility::Public, mk_path("A.B"));
        assert!(ei.is_public());
        assert!(!ei.is_reexport);
    }
    #[test]
    fn test_export_info_with_type() {
        let ei = ExportInfo::new(mk_name("foo"), Visibility::Public, mk_path("A.B"))
            .with_type(Expr::Sort(Level::zero()));
        assert!(ei.ty.is_some());
    }
    #[test]
    fn test_export_info_reexport() {
        let ei = ExportInfo::new(mk_name("foo"), Visibility::Public, mk_path("A.B")).as_reexport();
        assert!(ei.is_reexport);
    }
    #[test]
    fn test_import_decl_simple() {
        let import = ImportDecl::all(mk_path("A.B"));
        assert!(import.is_simple());
        assert!(import.accepts_name(&mk_name("foo")));
    }
    #[test]
    fn test_import_decl_selective() {
        let import = ImportDecl::selective(mk_path("A.B"), vec![mk_name("foo"), mk_name("bar")]);
        assert!(!import.is_simple());
        assert!(import.accepts_name(&mk_name("foo")));
        assert!(import.accepts_name(&mk_name("bar")));
        assert!(!import.accepts_name(&mk_name("baz")));
    }
    #[test]
    fn test_import_decl_hiding() {
        let import = ImportDecl::hiding(mk_path("A.B"), vec![mk_name("secret")]);
        assert!(!import.is_simple());
        assert!(import.accepts_name(&mk_name("foo")));
        assert!(!import.accepts_name(&mk_name("secret")));
    }
    #[test]
    fn test_import_decl_renamed() {
        let mut renames = HashMap::new();
        renames.insert(mk_name("old"), mk_name("new"));
        let import = ImportDecl::renamed(mk_path("A.B"), renames);
        assert_eq!(import.effective_name(&mk_name("old")), mk_name("new"));
        assert_eq!(import.effective_name(&mk_name("foo")), mk_name("foo"));
    }
    #[test]
    fn test_import_decl_public() {
        let import = ImportDecl::all(mk_path("A.B")).make_public();
        assert!(import.is_public);
    }
    #[test]
    fn test_import_decl_display() {
        let import = ImportDecl::all(mk_path("A.B"));
        let s = format!("{}", import);
        assert!(s.contains("import A.B"));
    }
    #[test]
    fn test_import_decl_selective_display() {
        let import = ImportDecl::selective(mk_path("A.B"), vec![mk_name("foo")]);
        let s = format!("{}", import);
        assert!(s.contains("import A.B"));
        assert!(s.contains("foo"));
    }
    #[test]
    fn test_module_header() {
        let mut header = ModuleHeader::new(mk_path("MyModule"));
        header.add_import(ImportDecl::all(mk_path("A.B")));
        header.add_export(mk_name("foo"));
        assert_eq!(header.imported_paths().len(), 1);
    }
    #[test]
    fn test_module_data() {
        let mut data = mk_module("A.B", &[("foo", Visibility::Public)]);
        assert_eq!(data.num_exports(), 1);
        assert!(data.lookup_export(&mk_name("foo")).is_some());
        assert!(data.lookup_export(&mk_name("bar")).is_none());
        assert!(!data.is_elaborated);
        data.mark_elaborated();
        assert!(data.is_elaborated);
    }
    #[test]
    fn test_module_data_public_exports() {
        let data = mk_module(
            "A.B",
            &[
                ("pub_fn", Visibility::Public),
                ("priv_fn", Visibility::Private),
                ("prot_fn", Visibility::Protected),
            ],
        );
        let public: Vec<_> = data.public_exports().collect();
        assert_eq!(public.len(), 1);
        let protected: Vec<_> = data.protected_exports().collect();
        assert_eq!(protected.len(), 2);
    }
    #[test]
    fn test_module_env() {
        let env = mk_env_with_modules(vec![
            mk_module("A", &[("x", Visibility::Public)]),
            mk_module("B", &[("y", Visibility::Public)]),
        ]);
        assert_eq!(env.num_modules(), 2);
        assert!(env.contains_module(&mk_path("A")));
        assert!(!env.contains_module(&mk_path("C")));
    }
    #[test]
    fn test_module_env_resolve_name() {
        let env = mk_env_with_modules(vec![
            mk_module("A", &[("foo", Visibility::Public)]),
            mk_module("B", &[("foo", Visibility::Public)]),
        ]);
        let results = env.resolve_name(&mk_name("foo"));
        assert_eq!(results.len(), 2);
    }
    #[test]
    fn test_import_error_display() {
        let e = ImportError::ModuleNotFound(mk_path("A.B"));
        assert!(format!("{}", e).contains("A.B"));
        let e = ImportError::NameNotFound {
            name: mk_name("foo"),
            module: mk_path("A.B"),
        };
        assert!(format!("{}", e).contains("foo"));
        assert!(format!("{}", e).contains("A.B"));
        let e = ImportError::PrivateName {
            name: mk_name("secret"),
            module: mk_path("A.B"),
        };
        assert!(format!("{}", e).contains("private"));
        let e = ImportError::CircularImport {
            cycle: vec![mk_path("A"), mk_path("B"), mk_path("A")],
        };
        assert!(format!("{}", e).contains("circular"));
        let e = ImportError::AmbiguousImport {
            name: mk_name("foo"),
            candidates: vec![mk_path("A"), mk_path("B")],
        };
        assert!(format!("{}", e).contains("ambiguous"));
    }
    #[test]
    fn test_resolve_simple_import() {
        let env = mk_env_with_modules(vec![mk_module(
            "A.B",
            &[("foo", Visibility::Public), ("bar", Visibility::Public)],
        )]);
        let resolver = ImportResolver::new(&env);
        let import = ImportDecl::all(mk_path("A.B"));
        let result = resolver.resolve(&import);
        assert!(result.is_ok());
        assert_eq!(result.num_resolved(), 2);
    }
    #[test]
    fn test_resolve_selective_import() {
        let env = mk_env_with_modules(vec![mk_module(
            "A.B",
            &[
                ("foo", Visibility::Public),
                ("bar", Visibility::Public),
                ("baz", Visibility::Public),
            ],
        )]);
        let resolver = ImportResolver::new(&env);
        let import = ImportDecl::selective(mk_path("A.B"), vec![mk_name("foo")]);
        let result = resolver.resolve(&import);
        assert!(result.is_ok());
        assert_eq!(result.num_resolved(), 1);
        assert_eq!(result.resolved_names[0].local_name, mk_name("foo"));
    }
    #[test]
    fn test_resolve_renamed_import() {
        let env = mk_env_with_modules(vec![mk_module("A.B", &[("foo", Visibility::Public)])]);
        let resolver = ImportResolver::new(&env);
        let mut renames = HashMap::new();
        renames.insert(mk_name("foo"), mk_name("myFoo"));
        let import = ImportDecl::renamed(mk_path("A.B"), renames);
        let result = resolver.resolve(&import);
        assert!(result.is_ok());
        assert_eq!(result.num_resolved(), 1);
        assert_eq!(result.resolved_names[0].local_name, mk_name("myFoo"));
    }
    #[test]
    fn test_resolve_module_not_found() {
        let env = ModuleEnv::new();
        let resolver = ImportResolver::new(&env);
        let import = ImportDecl::all(mk_path("NonExistent"));
        let result = resolver.resolve(&import);
        assert!(!result.is_ok());
        assert!(matches!(result.errors[0], ImportError::ModuleNotFound(_)));
    }
    #[test]
    fn test_resolve_name_not_found() {
        let env = mk_env_with_modules(vec![mk_module("A", &[("foo", Visibility::Public)])]);
        let resolver = ImportResolver::new(&env);
        let import = ImportDecl::selective(mk_path("A"), vec![mk_name("nonexistent")]);
        let result = resolver.resolve(&import);
        assert!(!result.is_ok());
    }
    #[test]
    fn test_resolve_private_name() {
        let env = mk_env_with_modules(vec![mk_module("A", &[("secret", Visibility::Private)])]);
        let resolver = ImportResolver::new(&env);
        let import = ImportDecl::selective(mk_path("A"), vec![mk_name("secret")]);
        let result = resolver.resolve(&import);
        assert!(!result.is_ok());
        assert!(matches!(result.errors[0], ImportError::PrivateName { .. }));
    }
    #[test]
    fn test_resolve_allow_private() {
        let env = mk_env_with_modules(vec![mk_module("A", &[("secret", Visibility::Private)])]);
        let resolver = ImportResolver::new(&env).allow_private_imports();
        let import = ImportDecl::selective(mk_path("A"), vec![mk_name("secret")]);
        let result = resolver.resolve(&import);
        assert!(result.is_ok());
    }
    #[test]
    fn test_dependency_graph_basic() {
        let mut graph = DependencyGraph::new();
        graph.add_edge(mk_path("B"), mk_path("A"));
        graph.add_edge(mk_path("C"), mk_path("B"));
        assert_eq!(graph.num_nodes(), 3);
        assert_eq!(graph.num_edges(), 2);
    }
    #[test]
    fn test_dependency_graph_no_cycle() {
        let mut graph = DependencyGraph::new();
        graph.add_edge(mk_path("B"), mk_path("A"));
        graph.add_edge(mk_path("C"), mk_path("B"));
        assert!(graph.detect_cycle().is_none());
    }
    #[test]
    fn test_dependency_graph_cycle() {
        let mut graph = DependencyGraph::new();
        graph.add_edge(mk_path("A"), mk_path("B"));
        graph.add_edge(mk_path("B"), mk_path("C"));
        graph.add_edge(mk_path("C"), mk_path("A"));
        assert!(graph.detect_cycle().is_some());
    }
    #[test]
    fn test_topological_sort_no_cycle() {
        let mut graph = DependencyGraph::new();
        graph.add_edge(mk_path("B"), mk_path("A"));
        graph.add_edge(mk_path("C"), mk_path("B"));
        let order = graph.topological_sort();
        assert!(order.is_ok());
        let order = order.expect("test operation should succeed");
        let pos_a = order.iter().position(|p| *p == mk_path("A"));
        let pos_b = order.iter().position(|p| *p == mk_path("B"));
        let pos_c = order.iter().position(|p| *p == mk_path("C"));
        assert!(pos_a.is_some());
        assert!(pos_b.is_some());
        assert!(pos_c.is_some());
    }
    #[test]
    fn test_topological_sort_cycle() {
        let mut graph = DependencyGraph::new();
        graph.add_edge(mk_path("A"), mk_path("B"));
        graph.add_edge(mk_path("B"), mk_path("A"));
        let order = graph.topological_sort();
        assert!(order.is_err());
    }
    #[test]
    fn test_transitive_deps() {
        let mut graph = DependencyGraph::new();
        graph.add_edge(mk_path("C"), mk_path("B"));
        graph.add_edge(mk_path("B"), mk_path("A"));
        let deps = graph.transitive_deps(&mk_path("C"));
        assert!(deps.contains(&mk_path("B")));
        assert!(deps.contains(&mk_path("A")));
        assert!(!deps.contains(&mk_path("C")));
    }
    #[test]
    fn test_reverse_deps() {
        let mut graph = DependencyGraph::new();
        graph.add_edge(mk_path("B"), mk_path("A"));
        graph.add_edge(mk_path("C"), mk_path("A"));
        let rev = graph.reverse_deps(&mk_path("A"));
        assert!(rev.contains(&mk_path("B")));
        assert!(rev.contains(&mk_path("C")));
    }
    #[test]
    fn test_resolve_transitive_imports() {
        let mut env = ModuleEnv::new();
        let mod_a = mk_module("A", &[("x", Visibility::Public)]);
        env.add_module(mod_a);
        let mut header_b = ModuleHeader::new(mk_path("B"));
        header_b.add_import(ImportDecl::all(mk_path("A")));
        let mut mod_b = ModuleData::new(mk_path("B"), header_b);
        mod_b.add_export(
            mk_name("y"),
            ExportInfo::new(mk_name("y"), Visibility::Public, mk_path("B")),
        );
        env.add_module(mod_b);
        let mut header_c = ModuleHeader::new(mk_path("C"));
        header_c.add_import(ImportDecl::all(mk_path("B")));
        let mod_c = ModuleData::new(mk_path("C"), header_c);
        env.add_module(mod_c);
        let graph = DependencyGraph::from_env(&env);
        let result = resolve_transitive_imports(&mk_path("C"), &env, &graph);
        let names: HashSet<Name> = result
            .resolved_names
            .iter()
            .map(|r| r.local_name.clone())
            .collect();
        assert!(names.contains(&mk_name("y")));
        assert!(names.contains(&mk_name("x")));
    }
    #[test]
    fn test_check_import_conflicts_none() {
        let imports = vec![
            ResolvedImport {
                local_name: mk_name("foo"),
                original_name: mk_name("foo"),
                source_module: mk_path("A"),
                ty: None,
                is_reexport: false,
            },
            ResolvedImport {
                local_name: mk_name("bar"),
                original_name: mk_name("bar"),
                source_module: mk_path("B"),
                ty: None,
                is_reexport: false,
            },
        ];
        let conflicts = check_import_conflicts(&imports);
        assert!(conflicts.is_empty());
    }
    #[test]
    fn test_check_import_conflicts_found() {
        let imports = vec![
            ResolvedImport {
                local_name: mk_name("foo"),
                original_name: mk_name("foo"),
                source_module: mk_path("A"),
                ty: None,
                is_reexport: false,
            },
            ResolvedImport {
                local_name: mk_name("foo"),
                original_name: mk_name("foo"),
                source_module: mk_path("B"),
                ty: None,
                is_reexport: false,
            },
        ];
        let conflicts = check_import_conflicts(&imports);
        assert_eq!(conflicts.len(), 1);
        assert_eq!(conflicts[0].name, mk_name("foo"));
    }
    #[test]
    fn test_conflict_display() {
        let c = ImportConflict::new(mk_name("foo"), mk_path("A"), mk_path("B"));
        let s = format!("{}", c);
        assert!(s.contains("foo"));
        assert!(s.contains("A"));
        assert!(s.contains("B"));
    }
    #[test]
    fn test_deduplicate_imports() {
        let imports = vec![
            ResolvedImport {
                local_name: mk_name("foo"),
                original_name: mk_name("foo"),
                source_module: mk_path("A"),
                ty: None,
                is_reexport: false,
            },
            ResolvedImport {
                local_name: mk_name("foo"),
                original_name: mk_name("foo"),
                source_module: mk_path("B"),
                ty: None,
                is_reexport: false,
            },
            ResolvedImport {
                local_name: mk_name("bar"),
                original_name: mk_name("bar"),
                source_module: mk_path("A"),
                ty: None,
                is_reexport: false,
            },
        ];
        let deduped = deduplicate_imports(imports);
        assert_eq!(deduped.len(), 2);
    }
    #[test]
    fn test_validate_imports_ok() {
        let env = mk_env_with_modules(vec![mk_module("A", &[])]);
        let mut header = ModuleHeader::new(mk_path("B"));
        header.add_import(ImportDecl::all(mk_path("A")));
        let errors = validate_imports(&header, &env);
        assert!(errors.is_empty());
    }
    #[test]
    fn test_validate_imports_missing() {
        let env = ModuleEnv::new();
        let mut header = ModuleHeader::new(mk_path("B"));
        header.add_import(ImportDecl::all(mk_path("A")));
        let errors = validate_imports(&header, &env);
        assert_eq!(errors.len(), 1);
    }
    #[test]
    fn test_build_name_table() {
        let env = mk_env_with_modules(vec![mk_module("A", &[("x", Visibility::Public)])]);
        let mut header = ModuleHeader::new(mk_path("B"));
        header.add_import(ImportDecl::all(mk_path("A")));
        let mod_b = ModuleData::new(mk_path("B"), header);
        let mut env = env;
        env.add_module(mod_b);
        let graph = DependencyGraph::from_env(&env);
        let table = build_name_table(&mk_path("B"), &env, &graph);
        assert!(table.contains_key(&mk_name("x")));
    }
    #[test]
    fn test_compute_load_order() {
        let env = mk_env_with_modules(vec![mk_module("A", &[]), mk_module("B", &[])]);
        let order = compute_load_order(&env);
        assert!(order.is_ok());
    }
    #[test]
    fn test_dependency_graph_from_env() {
        let mut env = ModuleEnv::new();
        let header_a = ModuleHeader::new(mk_path("A"));
        env.add_module(ModuleData::new(mk_path("A"), header_a));
        let mut header_b = ModuleHeader::new(mk_path("B"));
        header_b.add_import(ImportDecl::all(mk_path("A")));
        env.add_module(ModuleData::new(mk_path("B"), header_b));
        let graph = DependencyGraph::from_env(&env);
        assert_eq!(graph.num_nodes(), 2);
        let deps = graph.dependencies(&mk_path("B"));
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0], mk_path("A"));
    }
    #[test]
    fn test_import_result_merge() {
        let mut r1 = ImportResult::new();
        r1.resolved_names.push(ResolvedImport {
            local_name: mk_name("a"),
            original_name: mk_name("a"),
            source_module: mk_path("X"),
            ty: None,
            is_reexport: false,
        });
        let mut r2 = ImportResult::new();
        r2.resolved_names.push(ResolvedImport {
            local_name: mk_name("b"),
            original_name: mk_name("b"),
            source_module: mk_path("Y"),
            ty: None,
            is_reexport: false,
        });
        r1.merge(r2);
        assert_eq!(r1.num_resolved(), 2);
    }
    #[test]
    fn test_module_path_empty() {
        let path = ModulePath::new(vec![]);
        assert!(path.is_empty());
        assert_eq!(path.depth(), 0);
        assert!(path.parent().is_none());
        assert!(path.leaf().is_none());
    }
    #[test]
    fn test_module_path_to_dir_path() {
        let path = mk_path("A.B.C");
        assert_eq!(path.to_dir_path(), PathBuf::from("A/B/C"));
    }
    #[test]
    fn test_module_path_from_name() {
        let name = mk_name("Mathlib.Data");
        let path = ModulePath::from_name(&name);
        assert_eq!(path, mk_path("Mathlib.Data"));
    }
    #[test]
    fn test_resolve_hiding_import() {
        let env = mk_env_with_modules(vec![mk_module(
            "A",
            &[
                ("foo", Visibility::Public),
                ("bar", Visibility::Public),
                ("baz", Visibility::Public),
            ],
        )]);
        let resolver = ImportResolver::new(&env);
        let import = ImportDecl::hiding(mk_path("A"), vec![mk_name("bar")]);
        let result = resolver.resolve(&import);
        assert!(result.is_ok());
        let names: HashSet<Name> = result
            .resolved_names
            .iter()
            .map(|r| r.local_name.clone())
            .collect();
        assert!(names.contains(&mk_name("foo")));
        assert!(!names.contains(&mk_name("bar")));
        assert!(names.contains(&mk_name("baz")));
    }
    #[test]
    fn test_resolve_all() {
        let env = mk_env_with_modules(vec![
            mk_module("A", &[("x", Visibility::Public)]),
            mk_module("B", &[("y", Visibility::Public)]),
        ]);
        let resolver = ImportResolver::new(&env);
        let imports = vec![ImportDecl::all(mk_path("A")), ImportDecl::all(mk_path("B"))];
        let result = resolver.resolve_all(&imports);
        assert!(result.is_ok());
        assert_eq!(result.num_resolved(), 2);
    }
}
#[cfg(test)]
mod module_import_ext_tests {
    use super::*;
    use crate::module_import::*;
    fn mk_name(s: &str) -> Name {
        Name::str(s)
    }
    fn mk_path(s: &str) -> ModulePath {
        ModulePath(s.split('.').map(|p| p.to_string()).collect())
    }
    #[test]
    fn test_import_filter_all() {
        let f = ImportFilter::All;
        assert!(f.accepts(&mk_name("foo")));
        assert!(f.accepts(&mk_name("bar")));
    }
    #[test]
    fn test_import_filter_only() {
        let mut set = HashSet::new();
        set.insert(mk_name("foo"));
        let f = ImportFilter::Only(set);
        assert!(f.accepts(&mk_name("foo")));
        assert!(!f.accepts(&mk_name("bar")));
    }
    #[test]
    fn test_import_filter_except() {
        let mut set = HashSet::new();
        set.insert(mk_name("foo"));
        let f = ImportFilter::Except(set);
        assert!(!f.accepts(&mk_name("foo")));
        assert!(f.accepts(&mk_name("bar")));
    }
    #[test]
    fn test_import_filter_apply() {
        let mut set = HashSet::new();
        set.insert(mk_name("x"));
        let f = ImportFilter::Only(set);
        let names = vec![mk_name("x"), mk_name("y"), mk_name("x")];
        let filtered = f.apply(names);
        assert_eq!(filtered.len(), 2);
        assert!(filtered.iter().all(|n| n == &mk_name("x")));
    }
    #[test]
    fn test_import_stats_merge() {
        let mut a = ImportStats {
            total_modules: 3,
            total_decls: 15,
            ..Default::default()
        };
        let b = ImportStats {
            total_modules: 2,
            total_decls: 10,
            hiding_count: 1,
            ..Default::default()
        };
        a.merge(&b);
        assert_eq!(a.total_modules, 5);
        assert_eq!(a.total_decls, 25);
        assert_eq!(a.hiding_count, 1);
    }
    #[test]
    fn test_import_stats_avg_decls() {
        let s = ImportStats {
            total_modules: 4,
            total_decls: 20,
            ..Default::default()
        };
        assert!((s.avg_decls_per_module() - 5.0).abs() < 1e-10);
    }
    #[test]
    fn test_import_stats_avg_decls_zero_modules() {
        let s = ImportStats::new();
        assert!((s.avg_decls_per_module() - 0.0).abs() < 1e-10);
    }
    #[test]
    fn test_import_stats_summary() {
        let s = ImportStats {
            total_modules: 3,
            total_decls: 12,
            ..Default::default()
        };
        let sum = s.summary();
        assert!(sum.contains("modules=3"));
        assert!(sum.contains("decls=12"));
    }
    #[test]
    fn test_import_session_add_lookup() {
        let mut sess = ImportSession::new();
        let r = ResolvedImport {
            local_name: mk_name("foo"),
            source_module: mk_path("A"),
            original_name: mk_name("foo"),
            ty: None,
            is_reexport: false,
        };
        let ok = sess.add(r);
        assert!(ok);
        assert_eq!(sess.num_resolved(), 1);
        assert!(sess.lookup(&mk_name("foo")).is_some());
    }
    #[test]
    fn test_import_session_conflict() {
        let mut sess = ImportSession::new();
        let r1 = ResolvedImport {
            local_name: mk_name("foo"),
            source_module: mk_path("A"),
            original_name: mk_name("foo"),
            ty: None,
            is_reexport: false,
        };
        let r2 = ResolvedImport {
            local_name: mk_name("foo"),
            source_module: mk_path("B"),
            original_name: mk_name("foo"),
            ty: None,
            is_reexport: false,
        };
        sess.add(r1);
        let ok = sess.add(r2);
        assert!(!ok);
        assert!(sess.has_conflicts());
        assert_eq!(sess.conflicts().len(), 1);
    }
    #[test]
    fn test_import_session_empty() {
        let sess = ImportSession::new();
        assert_eq!(sess.num_resolved(), 0);
        assert!(!sess.has_conflicts());
    }
}
#[cfg(test)]
mod module_load_batch_tests {
    use super::*;
    use crate::module_import::*;
    fn mk_path(s: &str) -> ModulePath {
        ModulePath(s.split('.').map(String::from).collect())
    }
    #[test]
    fn test_module_load_batch_counts() {
        let mut b = ModuleLoadBatch::new();
        b.push(ModuleLoadOutcome::Loaded {
            path: mk_path("A"),
            decl_count: 10,
        });
        b.push(ModuleLoadOutcome::Cached { path: mk_path("B") });
        b.push(ModuleLoadOutcome::Failed {
            path: mk_path("C"),
            reason: "not found".to_string(),
        });
        assert_eq!(b.loaded_count(), 1);
        assert_eq!(b.cached_count(), 1);
        assert_eq!(b.failure_count(), 1);
        assert_eq!(b.total_decls(), 10);
    }
    #[test]
    fn test_module_load_batch_failed_paths() {
        let mut b = ModuleLoadBatch::new();
        b.push(ModuleLoadOutcome::Failed {
            path: mk_path("X.Y"),
            reason: "err".to_string(),
        });
        let paths = b.failed_paths();
        assert_eq!(paths.len(), 1);
        assert_eq!(paths[0].0.len(), 2);
    }
    #[test]
    fn test_module_load_outcome_path() {
        let o = ModuleLoadOutcome::Cached {
            path: mk_path("Std.Data.Nat"),
        };
        assert!(o.is_cached());
        assert_eq!(o.path().0.len(), 3);
    }
}
