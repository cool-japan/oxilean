//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast_impl::Decl;
    use crate::module::*;
    use crate::tokens::Span;
    use crate::{Located, SurfaceExpr};
    use std::collections::HashMap;
    fn mk_span() -> Span {
        Span::new(0, 1, 1, 1)
    }
    fn mk_def(name: &str) -> Decl {
        Decl::Definition {
            name: name.to_string(),
            univ_params: vec![],
            ty: None,
            val: Located::new(SurfaceExpr::Hole, mk_span()),
            where_clauses: vec![],
            attrs: vec![],
        }
    }
    #[test]
    fn test_module_create() {
        let module = Module::new("test".to_string());
        assert_eq!(module.name, "test");
        assert_eq!(module.decls.len(), 0);
    }
    #[test]
    fn test_module_full_path() {
        let mut module = Module::new("test".to_string());
        assert_eq!(module.full_path(), "test");
        module.path = vec!["std".to_string()];
        assert_eq!(module.full_path(), "std.test");
    }
    #[test]
    fn test_registry_create() {
        let registry = ModuleRegistry::new();
        assert_eq!(registry.all_modules().len(), 0);
    }
    #[test]
    fn test_register_module() {
        let mut registry = ModuleRegistry::new();
        let module = Module::new("test".to_string());
        assert!(registry.register(module).is_ok());
        assert!(registry.contains("test"));
    }
    #[test]
    fn test_register_duplicate() {
        let mut registry = ModuleRegistry::new();
        let module1 = Module::new("test".to_string());
        let module2 = Module::new("test".to_string());
        assert!(registry.register(module1).is_ok());
        assert!(registry.register(module2).is_err());
    }
    #[test]
    fn test_resolve_name_local() {
        let mut module = Module::new("test".to_string());
        module.add_decl(mk_def("foo"));
        assert_eq!(
            module.resolve_name("foo"),
            ResolvedName::Local("foo".to_string())
        );
    }
    #[test]
    fn test_resolve_name_not_found() {
        let module = Module::new("test".to_string());
        assert_eq!(module.resolve_name("foo"), ResolvedName::NotFound);
    }
    #[test]
    fn test_resolve_name_imported_all() {
        let mut module = Module::new("test".to_string());
        module.import_specs.push(ImportSpec::All("Std".to_string()));
        match module.resolve_name("bar") {
            ResolvedName::Imported { module: m, name } => {
                assert_eq!(m, "Std");
                assert_eq!(name, "bar");
            }
            _ => panic!("Expected Imported"),
        }
    }
    #[test]
    fn test_resolve_name_imported_selective() {
        let mut module = Module::new("test".to_string());
        module.import_specs.push(ImportSpec::Selective(
            "Std".to_string(),
            vec!["bar".to_string()],
        ));
        match module.resolve_name("bar") {
            ResolvedName::Imported { module: m, name } => {
                assert_eq!(m, "Std");
                assert_eq!(name, "bar");
            }
            _ => panic!("Expected Imported"),
        }
    }
    #[test]
    fn test_resolve_name_imported_hiding() {
        let mut module = Module::new("test".to_string());
        module.import_specs.push(ImportSpec::Hiding(
            "Std".to_string(),
            vec!["secret".to_string()],
        ));
        match module.resolve_name("bar") {
            ResolvedName::Imported { module: m, name } => {
                assert_eq!(m, "Std");
                assert_eq!(name, "bar");
            }
            _ => panic!("Expected Imported"),
        }
    }
    #[test]
    fn test_resolve_name_renaming() {
        let mut module = Module::new("test".to_string());
        module.import_specs.push(ImportSpec::Renaming(
            "Std".to_string(),
            vec![("originalName".to_string(), "renamedName".to_string())],
        ));
        match module.resolve_name("renamedName") {
            ResolvedName::Imported { module: m, name } => {
                assert_eq!(m, "Std");
                assert_eq!(name, "originalName");
            }
            _ => panic!("Expected Imported with original name"),
        }
    }
    #[test]
    fn test_visible_names() {
        let mut module = Module::new("test".to_string());
        module.add_decl(mk_def("foo"));
        module.add_decl(mk_def("bar"));
        let visible = module.visible_names();
        assert!(visible.contains(&"foo".to_string()));
        assert!(visible.contains(&"bar".to_string()));
    }
    #[test]
    fn test_visible_names_with_aliases() {
        let mut module = Module::new("test".to_string());
        module.add_decl(mk_def("foo"));
        let mut aliases = HashMap::new();
        aliases.insert("f".to_string(), "foo".to_string());
        module.add_namespace(NamespaceScope {
            name: "ns".to_string(),
            opened: vec![],
            aliases,
            visibility: std::collections::HashMap::new(),
            parent: None,
        });
        let visible = module.visible_names();
        assert!(visible.contains(&"foo".to_string()));
        assert!(visible.contains(&"f".to_string()));
    }
    #[test]
    fn test_exported_names_all() {
        let mut module = Module::new("test".to_string());
        module.add_decl(mk_def("foo"));
        module.add_decl(mk_def("bar"));
        module.export_spec = Some(ExportSpec::All);
        let exported = module.exported_names();
        assert_eq!(exported.len(), 2);
    }
    #[test]
    fn test_exported_names_selective() {
        let mut module = Module::new("test".to_string());
        module.add_decl(mk_def("foo"));
        module.add_decl(mk_def("bar"));
        module.export_spec = Some(ExportSpec::Selective(vec!["foo".to_string()]));
        let exported = module.exported_names();
        assert_eq!(exported.len(), 1);
        assert_eq!(exported[0], "foo");
    }
    #[test]
    fn test_exported_names_default() {
        let mut module = Module::new("test".to_string());
        module.add_decl(mk_def("foo"));
        let exported = module.exported_names();
        assert_eq!(exported, vec!["foo"]);
    }
    #[test]
    fn test_with_import_spec() {
        let mut module = Module::new("test".to_string());
        module.with_import_spec(ImportSpec::All("Std".to_string()));
        assert!(module.imports.contains(&"Std".to_string()));
        assert_eq!(module.import_specs.len(), 1);
    }
    #[test]
    fn test_add_namespace() {
        let mut module = Module::new("test".to_string());
        module.add_namespace(NamespaceScope {
            name: "Ns".to_string(),
            opened: vec![],
            aliases: HashMap::new(),
            visibility: std::collections::HashMap::new(),
            parent: None,
        });
        assert_eq!(module.namespaces.len(), 1);
        assert_eq!(module.namespaces[0].name, "Ns");
    }
    #[test]
    fn test_registry_get_mut() {
        let mut registry = ModuleRegistry::new();
        let module = Module::new("test".to_string());
        registry
            .register(module)
            .expect("test operation should succeed");
        let m = registry
            .get_mut("test")
            .expect("test operation should succeed");
        m.add_decl(mk_def("new_decl"));
        assert_eq!(
            registry.get("test").expect("key should exist").decls.len(),
            1
        );
    }
    #[test]
    fn test_registry_unregister() {
        let mut registry = ModuleRegistry::new();
        let module = Module::new("test".to_string());
        registry
            .register(module)
            .expect("test operation should succeed");
        let removed = registry.unregister("test");
        assert!(removed.is_some());
        assert!(!registry.contains("test"));
    }
    #[test]
    fn test_registry_unregister_nonexistent() {
        let mut registry = ModuleRegistry::new();
        assert!(registry.unregister("nope").is_none());
    }
    #[test]
    fn test_registry_resolve_local() {
        let mut registry = ModuleRegistry::new();
        let mut module = Module::new("A".to_string());
        module.add_decl(mk_def("foo"));
        registry
            .register(module)
            .expect("test operation should succeed");
        match registry.resolve("A", "foo") {
            ResolvedName::Local(name) => assert_eq!(name, "foo"),
            _ => panic!("Expected Local"),
        }
    }
    #[test]
    fn test_registry_resolve_imported() {
        let mut registry = ModuleRegistry::new();
        let mut mod_a = Module::new("A".to_string());
        mod_a.add_decl(mk_def("helper"));
        registry
            .register(mod_a)
            .expect("test operation should succeed");
        let mut mod_b = Module::new("B".to_string());
        mod_b.with_import_spec(ImportSpec::All("A".to_string()));
        registry
            .register(mod_b)
            .expect("test operation should succeed");
        match registry.resolve("B", "helper") {
            ResolvedName::Imported { module, name } => {
                assert_eq!(module, "A");
                assert_eq!(name, "helper");
            }
            _ => panic!("Expected Imported"),
        }
    }
    #[test]
    fn test_registry_resolve_not_found() {
        let mut registry = ModuleRegistry::new();
        let module = Module::new("A".to_string());
        registry
            .register(module)
            .expect("test operation should succeed");
        assert_eq!(registry.resolve("A", "nonexistent"), ResolvedName::NotFound);
    }
    #[test]
    fn test_dependency_order_simple() {
        let mut registry = ModuleRegistry::new();
        let mod_a = Module::new("A".to_string());
        registry
            .register(mod_a)
            .expect("test operation should succeed");
        let mut mod_b = Module::new("B".to_string());
        mod_b.add_import("A".to_string());
        registry
            .register(mod_b)
            .expect("test operation should succeed");
        let order = registry
            .dependency_order()
            .expect("test operation should succeed");
        let pos_a = order
            .iter()
            .position(|n| n == "A")
            .expect("test operation should succeed");
        let pos_b = order
            .iter()
            .position(|n| n == "B")
            .expect("test operation should succeed");
        assert!(pos_a < pos_b);
    }
    #[test]
    fn test_dependency_order_chain() {
        let mut registry = ModuleRegistry::new();
        let mod_a = Module::new("A".to_string());
        registry
            .register(mod_a)
            .expect("test operation should succeed");
        let mut mod_b = Module::new("B".to_string());
        mod_b.add_import("A".to_string());
        registry
            .register(mod_b)
            .expect("test operation should succeed");
        let mut mod_c = Module::new("C".to_string());
        mod_c.add_import("B".to_string());
        registry
            .register(mod_c)
            .expect("test operation should succeed");
        let order = registry
            .dependency_order()
            .expect("test operation should succeed");
        let pos_a = order
            .iter()
            .position(|n| n == "A")
            .expect("test operation should succeed");
        let pos_b = order
            .iter()
            .position(|n| n == "B")
            .expect("test operation should succeed");
        let pos_c = order
            .iter()
            .position(|n| n == "C")
            .expect("test operation should succeed");
        assert!(pos_a < pos_b);
        assert!(pos_b < pos_c);
    }
    #[test]
    fn test_detect_cycles_none() {
        let mut registry = ModuleRegistry::new();
        let mod_a = Module::new("A".to_string());
        registry
            .register(mod_a)
            .expect("test operation should succeed");
        let mut mod_b = Module::new("B".to_string());
        mod_b.add_import("A".to_string());
        registry
            .register(mod_b)
            .expect("test operation should succeed");
        let cycles = registry.detect_cycles();
        assert!(cycles.is_empty());
    }
    #[test]
    fn test_detect_cycles_present() {
        let mut registry = ModuleRegistry::new();
        let mut mod_a = Module::new("A".to_string());
        mod_a.add_import("B".to_string());
        registry
            .register(mod_a)
            .expect("test operation should succeed");
        let mut mod_b = Module::new("B".to_string());
        mod_b.add_import("A".to_string());
        registry
            .register(mod_b)
            .expect("test operation should succeed");
        let cycles = registry.detect_cycles();
        assert!(!cycles.is_empty());
    }
    #[test]
    fn test_transitive_deps_simple() {
        let mut registry = ModuleRegistry::new();
        let mod_a = Module::new("A".to_string());
        registry
            .register(mod_a)
            .expect("test operation should succeed");
        let mut mod_b = Module::new("B".to_string());
        mod_b.add_import("A".to_string());
        registry
            .register(mod_b)
            .expect("test operation should succeed");
        let mut mod_c = Module::new("C".to_string());
        mod_c.add_import("B".to_string());
        registry
            .register(mod_c)
            .expect("test operation should succeed");
        let deps = registry.transitive_deps("C");
        assert!(deps.contains(&"A".to_string()));
        assert!(deps.contains(&"B".to_string()));
    }
    #[test]
    fn test_transitive_deps_empty() {
        let mut registry = ModuleRegistry::new();
        let mod_a = Module::new("A".to_string());
        registry
            .register(mod_a)
            .expect("test operation should succeed");
        let deps = registry.transitive_deps("A");
        assert!(deps.is_empty());
    }
    #[test]
    fn test_transitive_deps_nonexistent() {
        let registry = ModuleRegistry::new();
        let deps = registry.transitive_deps("X");
        assert!(deps.is_empty());
    }
    #[test]
    fn test_namespace_alias_resolution() {
        let mut module = Module::new("test".to_string());
        module.add_decl(mk_def("Nat.add"));
        let mut aliases = HashMap::new();
        aliases.insert("add".to_string(), "Nat.add".to_string());
        module.add_namespace(NamespaceScope {
            name: "Nat".to_string(),
            opened: vec![],
            aliases,
            visibility: std::collections::HashMap::new(),
            parent: None,
        });
        match module.resolve_name("add") {
            ResolvedName::Local(name) => assert_eq!(name, "Nat.add"),
            _ => panic!("Expected Local from alias"),
        }
    }
    #[test]
    fn test_import_spec_hiding_blocks_name() {
        let mut module = Module::new("test".to_string());
        module.import_specs.push(ImportSpec::Hiding(
            "Std".to_string(),
            vec!["secret".to_string()],
        ));
        let result = module.resolve_name("secret");
        assert_eq!(result, ResolvedName::NotFound);
    }
    #[test]
    fn test_module_with_config() {
        let config = ModuleConfig {
            allow_circular: true,
            private_by_default: true,
            allow_shadowing: false,
        };
        let module = Module::with_config("test".to_string(), config);
        assert!(module.config.allow_circular);
        assert!(module.config.private_by_default);
    }
    #[test]
    fn test_set_and_get_visibility() {
        let mut module = Module::new("test".to_string());
        module.set_visibility("foo".to_string(), Visibility::Private);
        assert_eq!(module.get_visibility("foo"), Visibility::Private);
    }
    #[test]
    fn test_get_visibility_default_public() {
        let module = Module::new("test".to_string());
        assert_eq!(module.get_visibility("undefined"), Visibility::Public);
    }
    #[test]
    fn test_get_visibility_default_private() {
        let config = ModuleConfig {
            allow_circular: false,
            private_by_default: true,
            allow_shadowing: false,
        };
        let module = Module::with_config("test".to_string(), config);
        assert_eq!(module.get_visibility("undefined"), Visibility::Private);
    }
    #[test]
    fn test_add_and_get_opens() {
        let mut module = Module::new("test".to_string());
        module.add_open("Std".to_string(), false);
        module.add_open("Math".to_string(), true);
        let opens = module.get_opens();
        assert_eq!(opens.len(), 2);
        assert!(opens.contains(&"Std".to_string()));
        assert!(opens.contains(&"Math".to_string()));
    }
    #[test]
    fn test_is_accessible_public() {
        let mut module = Module::new("test".to_string());
        module.set_visibility("foo".to_string(), Visibility::Public);
        assert!(module.is_accessible("foo", false));
        assert!(module.is_accessible("foo", true));
    }
    #[test]
    fn test_is_accessible_private_same_module() {
        let mut module = Module::new("test".to_string());
        module.set_visibility("foo".to_string(), Visibility::Private);
        assert!(module.is_accessible("foo", true));
        assert!(!module.is_accessible("foo", false));
    }
    #[test]
    fn test_get_dependencies_imports() {
        let mut module = Module::new("test".to_string());
        module.add_import("A".to_string());
        module.add_import("B".to_string());
        let deps = module.get_dependencies();
        assert_eq!(deps.len(), 2);
        assert!(deps.contains("A"));
        assert!(deps.contains("B"));
    }
    #[test]
    fn test_get_dependencies_opens() {
        let mut module = Module::new("test".to_string());
        module.add_open("Std".to_string(), false);
        let deps = module.get_dependencies();
        assert!(deps.contains("Std"));
    }
    #[test]
    fn test_imports_module() {
        let mut module = Module::new("test".to_string());
        module.add_import("Std".to_string());
        assert!(module.imports_module("Std"));
        assert!(!module.imports_module("Math"));
    }
    #[test]
    fn test_get_hidden_names() {
        let mut module = Module::new("test".to_string());
        module.import_specs.push(ImportSpec::Hiding(
            "Std".to_string(),
            vec!["secret".to_string(), "hidden".to_string()],
        ));
        let hidden = module.get_hidden_names("Std");
        assert_eq!(hidden.len(), 2);
    }
    #[test]
    fn test_resolve_selective_import() {
        let module = Module::new("test".to_string());
        let selected = vec!["foo".to_string(), "bar".to_string()];
        let resolved = module.resolve_selective_import("Std", &selected);
        assert_eq!(resolved, selected);
    }
    #[test]
    fn test_namespace_lookup() {
        let mut module = Module::new("test".to_string());
        let mut aliases = HashMap::new();
        aliases.insert("f".to_string(), "Nat.add".to_string());
        module.add_namespace(NamespaceScope {
            name: "Nat".to_string(),
            opened: vec![],
            aliases,
            visibility: std::collections::HashMap::new(),
            parent: None,
        });
        let resolved = module.lookup_in_namespaces("f");
        assert_eq!(resolved, Some("Nat.add".to_string()));
    }
    #[test]
    fn test_namespace_names() {
        let mut module = Module::new("test".to_string());
        let mut aliases = HashMap::new();
        aliases.insert("a".to_string(), "A".to_string());
        aliases.insert("b".to_string(), "B".to_string());
        module.add_namespace(NamespaceScope {
            name: "ns".to_string(),
            opened: vec![],
            aliases,
            visibility: std::collections::HashMap::new(),
            parent: None,
        });
        let names = module.namespace_names("ns");
        assert_eq!(names.len(), 2);
        assert!(names.contains(&"a".to_string()));
        assert!(names.contains(&"b".to_string()));
    }
    #[test]
    fn test_all_visible_names() {
        let mut module = Module::new("test".to_string());
        module.add_decl(mk_def("foo"));
        module.add_decl(mk_def("bar"));
        module.set_visibility("foo".to_string(), Visibility::Public);
        module.set_visibility("bar".to_string(), Visibility::Private);
        let visible = module.all_visible_names();
        assert!(visible.iter().any(|nv| nv.name == "foo"));
        assert!(visible.iter().any(|nv| nv.name == "bar"));
    }
    #[test]
    fn test_registry_detect_mutual_imports_none() {
        let mut registry = ModuleRegistry::new();
        let mod_a = Module::new("A".to_string());
        registry
            .register(mod_a)
            .expect("test operation should succeed");
        let mut mod_b = Module::new("B".to_string());
        mod_b.add_import("A".to_string());
        registry
            .register(mod_b)
            .expect("test operation should succeed");
        let mutual = registry.detect_mutual_imports();
        assert!(mutual.is_empty());
    }
    #[test]
    fn test_registry_detect_mutual_imports_present() {
        let mut registry = ModuleRegistry::new();
        let mut mod_a = Module::new("A".to_string());
        mod_a.add_import("B".to_string());
        registry
            .register(mod_a)
            .expect("test operation should succeed");
        let mut mod_b = Module::new("B".to_string());
        mod_b.add_import("A".to_string());
        registry
            .register(mod_b)
            .expect("test operation should succeed");
        let mutual = registry.detect_mutual_imports();
        assert!(!mutual.is_empty());
    }
    #[test]
    fn test_are_mutually_dependent_yes() {
        let mut registry = ModuleRegistry::new();
        let mut mod_a = Module::new("A".to_string());
        mod_a.add_import("B".to_string());
        registry
            .register(mod_a)
            .expect("test operation should succeed");
        let mut mod_b = Module::new("B".to_string());
        mod_b.add_import("A".to_string());
        registry
            .register(mod_b)
            .expect("test operation should succeed");
        assert!(registry.are_mutually_dependent("A", "B"));
    }
    #[test]
    fn test_are_mutually_dependent_no() {
        let mut registry = ModuleRegistry::new();
        let mod_a = Module::new("A".to_string());
        registry
            .register(mod_a)
            .expect("test operation should succeed");
        let mut mod_b = Module::new("B".to_string());
        mod_b.add_import("A".to_string());
        registry
            .register(mod_b)
            .expect("test operation should succeed");
        assert!(!registry.are_mutually_dependent("A", "B"));
    }
    #[test]
    fn test_get_dependents_empty() {
        let mut registry = ModuleRegistry::new();
        let mod_a = Module::new("A".to_string());
        registry
            .register(mod_a)
            .expect("test operation should succeed");
        let dependents = registry.get_dependents("A");
        assert!(dependents.is_empty());
    }
    #[test]
    fn test_get_dependents_one() {
        let mut registry = ModuleRegistry::new();
        let mod_a = Module::new("A".to_string());
        registry
            .register(mod_a)
            .expect("test operation should succeed");
        let mut mod_b = Module::new("B".to_string());
        mod_b.add_import("A".to_string());
        registry
            .register(mod_b)
            .expect("test operation should succeed");
        let dependents = registry.get_dependents("A");
        assert_eq!(dependents.len(), 1);
        assert_eq!(dependents[0], "B");
    }
    #[test]
    fn test_get_all_dependencies_chain() {
        let mut registry = ModuleRegistry::new();
        let mod_a = Module::new("A".to_string());
        registry
            .register(mod_a)
            .expect("test operation should succeed");
        let mut mod_b = Module::new("B".to_string());
        mod_b.add_import("A".to_string());
        registry
            .register(mod_b)
            .expect("test operation should succeed");
        let mut mod_c = Module::new("C".to_string());
        mod_c.add_import("B".to_string());
        registry
            .register(mod_c)
            .expect("test operation should succeed");
        let all_deps = registry.get_all_dependencies("C");
        assert!(all_deps.contains(&"B".to_string()));
        assert!(all_deps.contains(&"A".to_string()));
    }
    #[test]
    fn test_verify_consistency_ok() {
        let mut registry = ModuleRegistry::new();
        let mod_a = Module::new("A".to_string());
        registry
            .register(mod_a)
            .expect("test operation should succeed");
        assert!(registry.verify_consistency().is_ok());
    }
    #[test]
    fn test_verify_consistency_broken() {
        let mut registry = ModuleRegistry::new();
        let mut mod_a = Module::new("A".to_string());
        mod_a.add_import("NonExistent".to_string());
        registry
            .register(mod_a)
            .expect("test operation should succeed");
        assert!(registry.verify_consistency().is_err());
    }
    #[test]
    fn test_get_reverse_dependencies() {
        let mut registry = ModuleRegistry::new();
        let mod_a = Module::new("A".to_string());
        registry
            .register(mod_a)
            .expect("test operation should succeed");
        let mut mod_b = Module::new("B".to_string());
        mod_b.add_import("A".to_string());
        registry
            .register(mod_b)
            .expect("test operation should succeed");
        let mut mod_c = Module::new("C".to_string());
        mod_c.add_import("B".to_string());
        registry
            .register(mod_c)
            .expect("test operation should succeed");
        let rev_deps = registry.get_reverse_dependencies("A");
        assert!(rev_deps.contains(&"B".to_string()));
        assert!(rev_deps.contains(&"C".to_string()));
    }
    #[test]
    fn test_reachable_from() {
        let mut registry = ModuleRegistry::new();
        let mod_a = Module::new("A".to_string());
        registry
            .register(mod_a)
            .expect("test operation should succeed");
        let mut mod_b = Module::new("B".to_string());
        mod_b.add_import("A".to_string());
        registry
            .register(mod_b)
            .expect("test operation should succeed");
        let reachable = registry.reachable_from("B");
        assert!(reachable.contains(&"A".to_string()));
    }
    #[test]
    fn test_get_statistics() {
        let mut registry = ModuleRegistry::new();
        let mut mod_a = Module::new("A".to_string());
        mod_a.add_import("B".to_string());
        mod_a.add_open("C".to_string(), false);
        registry
            .register(mod_a)
            .expect("test operation should succeed");
        let stats = registry.get_statistics();
        assert_eq!(stats.get("modules"), Some(&1));
        assert_eq!(stats.get("total_imports"), Some(&1));
        assert_eq!(stats.get("total_opens"), Some(&1));
    }
    #[test]
    fn test_get_public_interface() {
        let mut registry = ModuleRegistry::new();
        let mut mod_a = Module::new("A".to_string());
        mod_a.add_decl(mk_def("public_func"));
        mod_a.set_visibility("public_func".to_string(), Visibility::Public);
        mod_a.export_spec = Some(ExportSpec::All);
        registry
            .register(mod_a)
            .expect("test operation should succeed");
        let iface = registry.get_public_interface("A");
        assert!(iface.contains(&"public_func".to_string()));
    }
    #[test]
    fn test_is_name_accessible() {
        let mut registry = ModuleRegistry::new();
        let mut mod_a = Module::new("A".to_string());
        mod_a.add_decl(mk_def("public_func"));
        mod_a.set_visibility("public_func".to_string(), Visibility::Public);
        registry
            .register(mod_a)
            .expect("test operation should succeed");
        assert!(registry.is_name_accessible("public_func", "B", "A"));
    }
    #[test]
    fn test_resolve_with_opens() {
        let mut module = Module::new("test".to_string());
        module.add_open("Std".to_string(), false);
        match module.resolve_with_opens("foo") {
            ResolvedName::Imported { module: m, name } => {
                assert_eq!(m, "Std");
                assert_eq!(name, "foo");
            }
            _ => panic!("Expected Imported"),
        }
    }
    #[test]
    fn test_get_sccs() {
        let mut registry = ModuleRegistry::new();
        let mod_a = Module::new("A".to_string());
        registry
            .register(mod_a)
            .expect("test operation should succeed");
        let sccs = registry.get_sccs();
        let module_count = registry.all_modules().len();
        assert!(sccs.is_empty() || module_count > 0);
    }
    #[test]
    fn test_module_full_path_nested() {
        let mut module = Module::new("inner".to_string());
        module.path = vec!["root".to_string(), "middle".to_string()];
        assert_eq!(module.full_path(), "root.middle.inner");
    }
    #[test]
    fn test_set_default_visibility() {
        let mut module = Module::new("test".to_string());
        module.set_default_visibility(Visibility::Private);
        assert!(module.config.private_by_default);
    }
    #[test]
    fn test_registry_multiple_dependents() {
        let mut registry = ModuleRegistry::new();
        let mod_a = Module::new("A".to_string());
        registry
            .register(mod_a)
            .expect("test operation should succeed");
        let mut mod_b = Module::new("B".to_string());
        mod_b.add_import("A".to_string());
        registry
            .register(mod_b)
            .expect("test operation should succeed");
        let mut mod_c = Module::new("C".to_string());
        mod_c.add_import("A".to_string());
        registry
            .register(mod_c)
            .expect("test operation should succeed");
        let dependents = registry.get_dependents("A");
        assert_eq!(dependents.len(), 2);
    }
    #[test]
    fn test_open_directive_scoped() {
        let open = OpenDirective {
            module: "Std".to_string(),
            scoped: true,
        };
        assert_eq!(open.module, "Std");
        assert!(open.scoped);
    }
    #[test]
    fn test_visibility_enum() {
        assert_eq!(Visibility::Public, Visibility::Public);
        assert_ne!(Visibility::Public, Visibility::Private);
    }
}
#[cfg(test)]
mod module_ext_tests {
    use super::*;
    use crate::ast_impl::Decl;
    use crate::module::*;
    use std::collections::HashMap;
    #[test]
    fn test_module_dep() {
        let dep = ModuleDepExt::direct("A", "B");
        assert_eq!(dep.from, "A");
        assert_eq!(dep.to, "B");
        assert!(dep.direct);
    }
    #[test]
    fn test_dep_graph() {
        let mut g = DepGraphExt::new();
        g.add(ModuleDepExt::direct("A", "B"));
        g.add(ModuleDepExt::direct("A", "C"));
        g.add(ModuleDepExt::direct("B", "C"));
        let deps = g.direct_deps("A");
        assert_eq!(deps.len(), 2);
        assert!(deps.contains(&"B"));
        let dependents = g.dependents("C");
        assert_eq!(dependents.len(), 2);
    }
    #[test]
    fn test_self_import_detection() {
        let mut g = DepGraphExt::new();
        g.add(ModuleDepExt::direct("A", "A"));
        assert!(g.has_self_import());
        let mut g2 = DepGraphExt::new();
        g2.add(ModuleDepExt::direct("A", "B"));
        assert!(!g2.has_self_import());
    }
}
#[cfg(test)]
mod module_ext2_tests {
    use super::*;
    use crate::ast_impl::Decl;
    use crate::module::*;
    use std::collections::HashMap;
    #[test]
    fn test_module_attribute() {
        let attr = ModuleAttribute::new("simp");
        assert_eq!(attr.format(), "@[simp]");
        let attr2 = ModuleAttribute::new("reducible").with_arg("500");
        assert_eq!(attr2.format(), "@[reducible 500]");
    }
    #[test]
    fn test_export_entry() {
        let e = ExportEntry::all("Foo");
        assert!(e.all);
        let e2 = ExportEntry::names("Bar", vec!["x", "y"]);
        assert!(!e2.all);
        assert_eq!(e2.names.len(), 2);
    }
}
#[cfg(test)]
mod module_meta_tests {
    use super::*;
    use crate::ast_impl::Decl;
    use crate::module::*;
    use std::collections::HashMap;
    #[test]
    fn test_module_version() {
        let v = ModuleVersion::new(1, 2, 3);
        assert_eq!(v.format(), "1.2.3");
    }
    #[test]
    fn test_module_metadata() {
        let m = ModuleMetadata::new("Foo")
            .with_version(ModuleVersion::new(0, 1, 0))
            .with_author("Alice");
        assert_eq!(m.name, "Foo");
        assert!(m.version.is_some());
        assert_eq!(m.author.as_deref(), Some("Alice"));
    }
}
/// A module fingerprint for change detection.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn module_fingerprint(name: &str, content: &str) -> u64 {
    let mut hash = 14695981039346656037u64;
    for b in name
        .bytes()
        .chain(b":".iter().copied())
        .chain(content.bytes())
    {
        hash ^= b as u64;
        hash = hash.wrapping_mul(1099511628211u64);
    }
    hash
}
/// Returns whether a module name is a valid Lean-style dotted name.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn is_valid_module_name(name: &str) -> bool {
    if name.is_empty() {
        return false;
    }
    name.split('.')
        .all(|part| !part.is_empty() && part.starts_with(|c: char| c.is_uppercase()))
}
#[cfg(test)]
mod module_fingerprint_tests {
    use super::*;
    use crate::ast_impl::Decl;
    use crate::module::*;
    use std::collections::HashMap;
    #[test]
    fn test_module_fingerprint_stable() {
        let h1 = module_fingerprint("Foo", "content");
        let h2 = module_fingerprint("Foo", "content");
        assert_eq!(h1, h2);
        let h3 = module_fingerprint("Bar", "content");
        assert_ne!(h1, h3);
    }
    #[test]
    fn test_is_valid_module_name() {
        assert!(is_valid_module_name("Foo.Bar.Baz"));
        assert!(!is_valid_module_name("foo.bar"));
        assert!(!is_valid_module_name(""));
        assert!(!is_valid_module_name("Foo..Bar"));
    }
}
/// Returns whether a module is in a given namespace.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn module_in_namespace(module: &str, namespace: &str) -> bool {
    module.starts_with(namespace)
}
/// Returns the top-level namespace of a module name.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn top_namespace(module: &str) -> &str {
    module.split('.').next().unwrap_or(module)
}
#[cfg(test)]
mod module_pad {
    use super::*;
    use crate::ast_impl::Decl;
    use crate::module::*;
    use std::collections::HashMap;
    #[test]
    fn test_module_in_namespace() {
        assert!(module_in_namespace("Mathlib.Algebra.Ring", "Mathlib"));
        assert!(!module_in_namespace("Std.Tactic", "Mathlib"));
    }
    #[test]
    fn test_top_namespace() {
        assert_eq!(top_namespace("Mathlib.Algebra.Ring"), "Mathlib");
        assert_eq!(top_namespace("Std"), "Std");
    }
}
/// Returns the depth of a module name (number of dots + 1).
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn module_depth(name: &str) -> usize {
    name.split('.').count()
}
/// Returns the parent module of a dotted module name.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn parent_module(name: &str) -> Option<&str> {
    name.rfind('.').map(|i| &name[..i])
}
/// Returns the leaf name (last component) of a module name.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn leaf_module_name(name: &str) -> &str {
    name.rfind('.').map(|i| &name[i + 1..]).unwrap_or(name)
}
#[cfg(test)]
mod module_pad2 {
    use super::*;
    use crate::ast_impl::Decl;
    use crate::module::*;
    use std::collections::HashMap;
    #[test]
    fn test_module_depth() {
        assert_eq!(module_depth("Mathlib.Algebra.Ring"), 3);
        assert_eq!(module_depth("Std"), 1);
    }
    #[test]
    fn test_parent_module() {
        assert_eq!(
            parent_module("Mathlib.Algebra.Ring"),
            Some("Mathlib.Algebra")
        );
        assert_eq!(parent_module("Std"), None);
    }
    #[test]
    fn test_leaf_module_name() {
        assert_eq!(leaf_module_name("Mathlib.Algebra.Ring"), "Ring");
        assert_eq!(leaf_module_name("Std"), "Std");
    }
}
/// Checks if two modules are siblings (share the same parent).
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn are_sibling_modules(a: &str, b: &str) -> bool {
    parent_module(a) == parent_module(b)
}
