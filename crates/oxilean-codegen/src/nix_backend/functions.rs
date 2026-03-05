//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{
    NixAnalysisCache, NixAttr, NixBackend, NixBuiltin, NixConstantFoldingHelper, NixDepGraph,
    NixDominatorTree, NixExpr, NixExprStats, NixFlakeHelper, NixFormatter, NixFunction,
    NixLetBinding, NixLibHelper, NixLivenessInfo, NixModule, NixModuleValidator, NixOptionalHelper,
    NixPassConfig, NixPassPhase, NixPassRegistry, NixPassStats, NixPattern, NixPkgsHelper,
    NixStringInterpolator, NixSystemdHelper, NixType, NixTypeChecker, NixValue, NixWorklist,
};

/// Returns true if an argument expression needs parentheses when used as an Apply argument.
pub(super) fn arg_needs_parens(e: &NixExpr) -> bool {
    matches!(
        e,
        NixExpr::Let(_, _)
            | NixExpr::If(_, _, _)
            | NixExpr::With(_, _)
            | NixExpr::Assert(_, _)
            | NixExpr::Lambda(_)
            | NixExpr::Apply(_, _)
            | NixExpr::BinOp(_, _, _)
    )
}
/// Escape characters that need escaping inside a double-quoted Nix string.
pub(super) fn escape_nix_string(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace("${", "\\${")
        .replace('\n', "\\n")
        .replace('\t', "\\t")
}
/// Shorthand: integer literal.
pub fn nix_int(n: i64) -> NixExpr {
    NixExpr::Int(n)
}
/// Shorthand: string literal.
pub fn nix_str(s: &str) -> NixExpr {
    NixExpr::Str(s.to_string())
}
/// Shorthand: boolean literal.
pub fn nix_bool(b: bool) -> NixExpr {
    NixExpr::Bool(b)
}
/// Shorthand: variable reference.
pub fn nix_var(name: &str) -> NixExpr {
    NixExpr::Var(name.to_string())
}
/// Shorthand: path literal.
pub fn nix_path(p: &str) -> NixExpr {
    NixExpr::Path(p.to_string())
}
/// Shorthand: null.
pub fn nix_null() -> NixExpr {
    NixExpr::Null
}
/// Shorthand: list.
pub fn nix_list(items: Vec<NixExpr>) -> NixExpr {
    NixExpr::List(items)
}
/// Shorthand: attribute set.
pub fn nix_set(attrs: Vec<(&str, NixExpr)>) -> NixExpr {
    NixExpr::AttrSet(attrs.into_iter().map(|(k, v)| NixAttr::new(k, v)).collect())
}
/// Shorthand: binary operator.
pub fn nix_binop(op: &str, lhs: NixExpr, rhs: NixExpr) -> NixExpr {
    NixExpr::BinOp(op.to_string(), Box::new(lhs), Box::new(rhs))
}
/// Shorthand: function application.
pub fn nix_apply(func: NixExpr, arg: NixExpr) -> NixExpr {
    NixExpr::Apply(Box::new(func), Box::new(arg))
}
/// Shorthand: let-in expression.
pub fn nix_let(bindings: Vec<(&str, NixExpr)>, body: NixExpr) -> NixExpr {
    NixExpr::Let(
        bindings
            .into_iter()
            .map(|(k, v)| NixLetBinding::new(k, v))
            .collect(),
        Box::new(body),
    )
}
/// Shorthand: if-then-else.
pub fn nix_if(cond: NixExpr, t: NixExpr, f: NixExpr) -> NixExpr {
    NixExpr::If(Box::new(cond), Box::new(t), Box::new(f))
}
/// Shorthand: attribute selection.
pub fn nix_select(expr: NixExpr, attr: &str) -> NixExpr {
    NixExpr::Select(Box::new(expr), attr.to_string(), None)
}
/// Shorthand: with-expression.
pub fn nix_with(src: NixExpr, body: NixExpr) -> NixExpr {
    NixExpr::With(Box::new(src), Box::new(body))
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub(super) fn test_emit_literals() {
        assert_eq!(nix_int(42).emit(0), "42");
        assert_eq!(NixExpr::Float(3.14).emit(0), "3.14");
        assert_eq!(nix_bool(true).emit(0), "true");
        assert_eq!(nix_bool(false).emit(0), "false");
        assert_eq!(nix_str("hello").emit(0), "\"hello\"");
        assert_eq!(nix_null().emit(0), "null");
        assert_eq!(nix_path("./foo/bar").emit(0), "./foo/bar");
    }
    #[test]
    pub(super) fn test_emit_list() {
        let empty = nix_list(vec![]);
        assert_eq!(empty.emit(0), "[ ]");
        let lst = nix_list(vec![nix_int(1), nix_int(2), nix_int(3)]);
        let s = lst.emit(0);
        assert!(s.starts_with('['));
        assert!(s.contains('1'));
        assert!(s.contains('2'));
        assert!(s.contains('3'));
        assert!(s.ends_with(']'));
    }
    #[test]
    pub(super) fn test_emit_attrset() {
        let set = nix_set(vec![("x", nix_int(1)), ("y", nix_bool(true))]);
        let s = set.emit(0);
        assert!(s.contains("x = 1;"));
        assert!(s.contains("y = true;"));
        assert!(s.starts_with('{'));
        assert!(s.ends_with('}'));
    }
    #[test]
    pub(super) fn test_emit_let_in() {
        let expr = nix_let(
            vec![("x", nix_int(5)), ("y", nix_int(10))],
            nix_binop("+", nix_var("x"), nix_var("y")),
        );
        let s = expr.emit(0);
        assert!(s.contains("let"));
        assert!(s.contains("x = 5;"));
        assert!(s.contains("y = 10;"));
        assert!(s.contains("in"));
        assert!(s.contains("(x + y)"));
    }
    #[test]
    pub(super) fn test_emit_lambda_and_apply() {
        let func = NixExpr::Lambda(Box::new(NixFunction::new(
            "x",
            nix_binop("+", nix_var("x"), nix_int(1)),
        )));
        let s = func.emit(0);
        assert_eq!(s, "x: (x + 1)");
        let app = nix_apply(func, nix_int(42));
        let s2 = app.emit(0);
        assert!(s2.contains("x: (x + 1)"));
        assert!(s2.contains("42"));
    }
    #[test]
    pub(super) fn test_emit_attr_pattern_function() {
        let func = NixFunction::with_attr_pattern(
            vec![
                ("pkgs".into(), None),
                ("lib".into(), None),
                ("config".into(), Some(nix_null())),
            ],
            true,
            nix_var("pkgs"),
        );
        let s = func.emit(0);
        assert!(s.contains("pkgs"));
        assert!(s.contains("lib"));
        assert!(s.contains("config ? null"));
        assert!(s.contains("..."));
        assert!(s.contains(": pkgs"));
    }
    #[test]
    pub(super) fn test_emit_if_then_else() {
        let expr = nix_if(nix_bool(true), nix_int(1), nix_int(0));
        let s = expr.emit(0);
        assert!(s.contains("if true"));
        assert!(s.contains("then"));
        assert!(s.contains('1'));
        assert!(s.contains("else"));
        assert!(s.contains('0'));
    }
    #[test]
    pub(super) fn test_emit_select_with_default() {
        let expr = NixExpr::Select(
            Box::new(nix_var("config")),
            "services.nginx.enable".into(),
            Some(Box::new(nix_bool(false))),
        );
        let s = expr.emit(0);
        assert_eq!(s, "config.services.nginx.enable or false");
    }
    #[test]
    pub(super) fn test_make_overlay() {
        let backend = NixBackend::new();
        let overlay = backend.make_overlay(vec![NixAttr::new("hello", nix_var("prev.hello"))]);
        let s = overlay.emit(0);
        assert!(s.contains("final"));
        assert!(s.contains("prev"));
        assert!(s.contains("hello"));
    }
    #[test]
    pub(super) fn test_nixos_module_emit() {
        let backend = NixBackend::new();
        let m = backend.make_nixos_module(
            vec![
                ("config".into(), None),
                ("pkgs".into(), None),
                ("lib".into(), None),
            ],
            vec![NixAttr::new(
                "services.myservice.enable",
                nix_apply(
                    nix_select(nix_select(nix_var("lib"), "mkOption"), "mkOption"),
                    nix_set(vec![("default", nix_bool(false))]),
                ),
            )],
            vec![NixAttr::new(
                "environment.systemPackages",
                nix_list(vec![nix_var("pkgs.hello")]),
            )],
        );
        let s = backend.emit_module(&m);
        assert!(s.contains("NixOS module generated by OxiLean"));
        assert!(s.contains("options"));
        assert!(s.contains("config"));
        assert!(s.contains("..."));
    }
    #[test]
    pub(super) fn test_escape_string() {
        let s = nix_str("say \"hello\" and \\go").emit(0);
        assert_eq!(s, r#""say \"hello\" and \\go""#);
    }
    #[test]
    pub(super) fn test_rec_attrset() {
        let r = NixExpr::Rec(vec![
            NixAttr::new("a", nix_int(1)),
            NixAttr::new("b", nix_binop("+", nix_var("a"), nix_int(1))),
        ]);
        let s = r.emit(0);
        assert!(s.starts_with("rec {"));
        assert!(s.contains("a = 1;"));
        assert!(s.contains("b = (a + 1);"));
    }
    #[test]
    pub(super) fn test_multiline_string() {
        let s = NixExpr::Multiline("  echo hello\n  echo world".into());
        let out = s.emit(0);
        assert!(out.starts_with("''"));
        assert!(out.contains("echo hello"));
        assert!(out.ends_with("''"));
    }
    #[test]
    pub(super) fn test_inherit() {
        let inh = NixExpr::Inherit(
            Some(Box::new(nix_var("pkgs"))),
            vec!["hello".into(), "git".into()],
        );
        let s = inh.emit(0);
        assert_eq!(s, "inherit (pkgs) hello git;");
        let inh2 = NixExpr::Inherit(None, vec!["foo".into()]);
        assert_eq!(inh2.emit(0), "inherit foo;");
    }
    #[test]
    pub(super) fn test_module_emit_plain() {
        let m = NixModule::new(nix_set(vec![("a", nix_int(1)), ("b", nix_str("hello"))]));
        let s = m.emit();
        assert!(s.contains("Nix expression generated by OxiLean"));
        assert!(s.contains("a = 1;"));
        assert!(s.contains("b = \"hello\";"));
        assert!(s.ends_with('\n'));
    }
    #[test]
    pub(super) fn test_nix_type_display() {
        assert_eq!(NixType::Int.to_string(), "int");
        assert_eq!(NixType::Bool.to_string(), "bool");
        assert_eq!(
            NixType::Function(Box::new(NixType::Int), Box::new(NixType::Bool)).to_string(),
            "int -> bool"
        );
        assert_eq!(
            NixType::List(Box::new(NixType::String)).to_string(),
            "[ string ]"
        );
    }
    #[test]
    pub(super) fn test_make_derivation() {
        let backend = NixBackend::new();
        let drv = backend.make_derivation(
            "my-tool",
            "1.0.0",
            nix_path("./src"),
            vec![nix_var("pkgs.gcc"), nix_var("pkgs.cmake")],
            Some("make"),
            Some("make install"),
            vec![],
        );
        let s = drv.emit(0);
        assert!(s.contains("my-tool-1.0.0"));
        assert!(s.contains("buildInputs"));
        assert!(s.contains("pkgs.gcc"));
    }
    #[test]
    pub(super) fn test_make_flake() {
        let backend = NixBackend::new();
        let flake = backend.make_flake(
            "My Flake",
            vec![NixAttr::new(
                "packages.x86_64-linux.default",
                nix_var("pkgs.hello"),
            )],
        );
        let s = flake.emit(0);
        assert!(s.contains("description"));
        assert!(s.contains("My Flake"));
        assert!(s.contains("outputs"));
        assert!(s.contains("self"));
        assert!(s.contains("nixpkgs"));
    }
}
/// Emit a call to a Nix built-in function.
#[allow(dead_code)]
pub fn nix_builtin_call(builtin: NixBuiltin, args: Vec<NixExpr>) -> NixExpr {
    let mut expr = NixExpr::Var(builtin.to_string());
    for arg in args {
        expr = nix_apply(expr, arg);
    }
    expr
}
#[cfg(test)]
mod extended_tests {
    use super::*;
    #[test]
    pub(super) fn test_nix_value_type_name() {
        assert_eq!(NixValue::Int(1).type_name(), "int");
        assert_eq!(NixValue::Bool(true).type_name(), "bool");
        assert_eq!(NixValue::Str("x".into()).type_name(), "string");
        assert_eq!(NixValue::Null.type_name(), "null");
        assert_eq!(NixValue::List(vec![]).type_name(), "list");
    }
    #[test]
    pub(super) fn test_nix_value_is_truthy() {
        assert!(NixValue::Bool(true).is_truthy());
        assert!(!NixValue::Bool(false).is_truthy());
        assert!(!NixValue::Null.is_truthy());
        assert!(NixValue::Int(1).is_truthy());
        assert!(!NixValue::Int(0).is_truthy());
        assert!(!NixValue::Str(String::new()).is_truthy());
        assert!(NixValue::Str("x".into()).is_truthy());
    }
    #[test]
    pub(super) fn test_nix_value_get_attr() {
        let val = NixValue::AttrSet(vec![
            ("a".into(), NixValue::Int(1)),
            ("b".into(), NixValue::Bool(true)),
        ]);
        assert!(val.get_attr("a").is_some());
        assert!(val.get_attr("c").is_none());
    }
    #[test]
    pub(super) fn test_nix_value_to_expr() {
        let val = NixValue::Int(42);
        let expr = val.to_expr();
        assert_eq!(expr.emit(0), "42");
        let list_val = NixValue::List(vec![NixValue::Int(1), NixValue::Int(2)]);
        let list_expr = list_val.to_expr();
        let s = list_expr.emit(0);
        assert!(s.contains('1'));
        assert!(s.contains('2'));
    }
    #[test]
    pub(super) fn test_nix_builtin_display() {
        assert_eq!(NixBuiltin::Map.to_string(), "builtins.map");
        assert_eq!(NixBuiltin::Filter.to_string(), "builtins.filter");
        assert_eq!(NixBuiltin::AttrNames.to_string(), "builtins.attrNames");
        assert_eq!(NixBuiltin::ToJSON.to_string(), "builtins.toJSON");
    }
    #[test]
    pub(super) fn test_nix_builtin_call() {
        let call = nix_builtin_call(NixBuiltin::Length, vec![nix_var("myList")]);
        let s = call.emit(0);
        assert!(s.contains("builtins.length"));
        assert!(s.contains("myList"));
    }
    #[test]
    pub(super) fn test_pkgs_helper_fetch_url() {
        let expr = NixPkgsHelper::fetch_url("https://example.com/file.tar.gz", "sha256-abc123");
        let s = expr.emit(0);
        assert!(s.contains("fetchurl"));
        assert!(s.contains("example.com"));
        assert!(s.contains("sha256-abc123"));
    }
    #[test]
    pub(super) fn test_pkgs_helper_fetch_github() {
        let expr = NixPkgsHelper::fetch_from_github("NixOS", "nixpkgs", "abc123", "sha256-xyz");
        let s = expr.emit(0);
        assert!(s.contains("fetchFromGitHub"));
        assert!(s.contains("NixOS"));
        assert!(s.contains("nixpkgs"));
    }
    #[test]
    pub(super) fn test_lib_helper_mk_option() {
        let opt = NixLibHelper::mk_option(
            NixLibHelper::type_str(),
            Some(nix_str("default")),
            Some("A description"),
        );
        let s = opt.emit(0);
        assert!(s.contains("mkOption"));
        assert!(s.contains("description"));
    }
    #[test]
    pub(super) fn test_lib_helper_mk_if() {
        let expr = NixLibHelper::mk_if(nix_bool(true), nix_str("value"));
        let s = expr.emit(0);
        assert!(s.contains("mkIf"));
    }
    #[test]
    pub(super) fn test_lib_helper_mk_merge() {
        let expr = NixLibHelper::mk_merge(vec![
            nix_set(vec![("a", nix_int(1))]),
            nix_set(vec![("b", nix_int(2))]),
        ]);
        let s = expr.emit(0);
        assert!(s.contains("mkMerge"));
    }
    #[test]
    pub(super) fn test_string_interpolator_simple() {
        let result = NixStringInterpolator::new()
            .lit("Hello, ")
            .interp(nix_var("name"))
            .lit("!")
            .build();
        let s = result.emit(0);
        assert!(s.contains("Hello,"));
        assert!(s.contains("name"));
        assert!(s.contains('!'));
    }
    #[test]
    pub(super) fn test_string_interpolator_literal_only() {
        let result = NixStringInterpolator::new()
            .lit("hello")
            .lit(" world")
            .build();
        assert_eq!(result, NixExpr::Str("hello world".into()));
    }
    #[test]
    pub(super) fn test_nix_formatter_sort_attrs() {
        let expr = nix_set(vec![
            ("z", nix_int(3)),
            ("a", nix_int(1)),
            ("m", nix_int(2)),
        ]);
        let fmt = NixFormatter::new();
        let s = fmt.format(&expr);
        let a_pos = s.find('a').expect("a_pos should be found");
        let m_pos = s.find('m').expect("m_pos should be found");
        let z_pos = s.find('z').expect("z_pos should be found");
        assert!(a_pos < m_pos && m_pos < z_pos);
    }
    #[test]
    pub(super) fn test_nix_type_checker() {
        assert_eq!(NixTypeChecker::infer(&nix_int(42)), NixType::Int);
        assert_eq!(NixTypeChecker::infer(&nix_bool(true)), NixType::Bool);
        assert_eq!(NixTypeChecker::infer(&nix_str("hi")), NixType::String);
        assert_eq!(NixTypeChecker::infer(&NixExpr::Null), NixType::NullType);
        assert_eq!(NixTypeChecker::infer(&nix_path("./foo")), NixType::Path);
        let list_ty = NixTypeChecker::infer(&nix_list(vec![nix_int(1)]));
        assert!(matches!(list_ty, NixType::List(_)));
    }
    #[test]
    pub(super) fn test_nix_expr_stats() {
        let expr = nix_let(
            vec![("x", nix_int(1)), ("y", nix_int(2))],
            nix_binop("+", nix_var("x"), nix_var("y")),
        );
        let stats = NixExprStats::collect(&expr);
        assert!(stats.node_count > 0);
        assert_eq!(stats.let_bindings, 2);
        assert_eq!(stats.literals, 2);
        assert_eq!(stats.var_refs, 2);
    }
    #[test]
    pub(super) fn test_nixos_module_validator_valid() {
        let backend = NixBackend::new();
        let module = backend.make_nixos_module(
            vec![("config".into(), None), ("pkgs".into(), None)],
            vec![NixAttr::new("myservice.enable", nix_bool(false))],
            vec![NixAttr::new("environment.packages", nix_list(vec![]))],
        );
        let report = NixModuleValidator::validate(&module);
        assert!(report.is_valid(), "{:?}", report.errors);
    }
    #[test]
    pub(super) fn test_nixos_module_validator_missing_keys_warning() {
        let module = NixModule::nixos_module(NixExpr::Lambda(Box::new(NixFunction {
            pattern: NixPattern::Ident("args".into()),
            body: Box::new(NixExpr::AttrSet(vec![])),
        })));
        let report = NixModuleValidator::validate(&module);
        assert!(!report.warnings.is_empty());
    }
    #[test]
    pub(super) fn test_flake_helper_full_flake() {
        let flake = NixFlakeHelper::full_flake(
            "Test flake",
            vec![NixFlakeHelper::input(
                "nixpkgs",
                "github:NixOS/nixpkgs/nixos-unstable",
            )],
            nix_set(vec![("default", nix_var("pkgs.hello"))]),
            vec![("self".into(), None), ("nixpkgs".into(), None)],
        );
        let s = flake.emit(0);
        assert!(s.contains("Test flake"));
        assert!(s.contains("nixpkgs"));
        assert!(s.contains("outputs"));
    }
    #[test]
    pub(super) fn test_optional_helper_map_nullable() {
        let result = NixOptionalHelper::map_nullable(
            nix_var("x"),
            nix_int(0),
            NixExpr::Lambda(Box::new(NixFunction::new("v", nix_var("v")))),
        );
        let s = result.emit(0);
        assert!(s.contains("if"));
        assert!(s.contains("null"));
    }
    #[test]
    pub(super) fn test_systemd_helper_make_service() {
        let svc = NixSystemdHelper::make_service(
            "My Service",
            "/bin/my-service",
            vec!["network.target"],
            vec!["network-online.target"],
            "on-failure",
            Some("myuser"),
            vec![],
        );
        let s = svc.emit(0);
        assert!(s.contains("My Service"));
        assert!(s.contains("/bin/my-service"));
        assert!(s.contains("on-failure"));
    }
    #[test]
    pub(super) fn test_pkgs_symlink_join() {
        let expr = NixPkgsHelper::symlink_join(
            "all-tools",
            vec![nix_var("pkgs.git"), nix_var("pkgs.curl")],
        );
        let s = expr.emit(0);
        assert!(s.contains("symlinkJoin"));
        assert!(s.contains("all-tools"));
    }
}
#[cfg(test)]
mod Nix_infra_tests {
    use super::*;
    #[test]
    pub(super) fn test_pass_config() {
        let config = NixPassConfig::new("test_pass", NixPassPhase::Transformation);
        assert!(config.enabled);
        assert!(config.phase.is_modifying());
        assert_eq!(config.phase.name(), "transformation");
    }
    #[test]
    pub(super) fn test_pass_stats() {
        let mut stats = NixPassStats::new();
        stats.record_run(10, 100, 3);
        stats.record_run(20, 200, 5);
        assert_eq!(stats.total_runs, 2);
        assert!((stats.average_changes_per_run() - 15.0).abs() < 0.01);
        assert!((stats.success_rate() - 1.0).abs() < 0.01);
        let s = stats.format_summary();
        assert!(s.contains("Runs: 2/2"));
    }
    #[test]
    pub(super) fn test_pass_registry() {
        let mut reg = NixPassRegistry::new();
        reg.register(NixPassConfig::new("pass_a", NixPassPhase::Analysis));
        reg.register(NixPassConfig::new("pass_b", NixPassPhase::Transformation).disabled());
        assert_eq!(reg.total_passes(), 2);
        assert_eq!(reg.enabled_count(), 1);
        reg.update_stats("pass_a", 5, 50, 2);
        let stats = reg.get_stats("pass_a").expect("stats should exist");
        assert_eq!(stats.total_changes, 5);
    }
    #[test]
    pub(super) fn test_analysis_cache() {
        let mut cache = NixAnalysisCache::new(10);
        cache.insert("key1".to_string(), vec![1, 2, 3]);
        assert!(cache.get("key1").is_some());
        assert!(cache.get("key2").is_none());
        assert!((cache.hit_rate() - 0.5).abs() < 0.01);
        cache.invalidate("key1");
        assert!(!cache.entries["key1"].valid);
        assert_eq!(cache.size(), 1);
    }
    #[test]
    pub(super) fn test_worklist() {
        let mut wl = NixWorklist::new();
        assert!(wl.push(1));
        assert!(wl.push(2));
        assert!(!wl.push(1));
        assert_eq!(wl.len(), 2);
        assert_eq!(wl.pop(), Some(1));
        assert!(!wl.contains(1));
        assert!(wl.contains(2));
    }
    #[test]
    pub(super) fn test_dominator_tree() {
        let mut dt = NixDominatorTree::new(5);
        dt.set_idom(1, 0);
        dt.set_idom(2, 0);
        dt.set_idom(3, 1);
        assert!(dt.dominates(0, 3));
        assert!(dt.dominates(1, 3));
        assert!(!dt.dominates(2, 3));
        assert!(dt.dominates(3, 3));
    }
    #[test]
    pub(super) fn test_liveness() {
        let mut liveness = NixLivenessInfo::new(3);
        liveness.add_def(0, 1);
        liveness.add_use(1, 1);
        assert!(liveness.defs[0].contains(&1));
        assert!(liveness.uses[1].contains(&1));
    }
    #[test]
    pub(super) fn test_constant_folding() {
        assert_eq!(NixConstantFoldingHelper::fold_add_i64(3, 4), Some(7));
        assert_eq!(NixConstantFoldingHelper::fold_div_i64(10, 0), None);
        assert_eq!(NixConstantFoldingHelper::fold_div_i64(10, 2), Some(5));
        assert_eq!(
            NixConstantFoldingHelper::fold_bitand_i64(0b1100, 0b1010),
            0b1000
        );
        assert_eq!(NixConstantFoldingHelper::fold_bitnot_i64(0), -1);
    }
    #[test]
    pub(super) fn test_dep_graph() {
        let mut g = NixDepGraph::new();
        g.add_dep(1, 2);
        g.add_dep(2, 3);
        g.add_dep(1, 3);
        assert_eq!(g.dependencies_of(2), vec![1]);
        let topo = g.topological_sort();
        assert_eq!(topo.len(), 3);
        assert!(!g.has_cycle());
        let pos: std::collections::HashMap<u32, usize> =
            topo.iter().enumerate().map(|(i, &n)| (n, i)).collect();
        assert!(pos[&1] < pos[&2]);
        assert!(pos[&1] < pos[&3]);
        assert!(pos[&2] < pos[&3]);
    }
}
