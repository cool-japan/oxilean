//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::HashMap;

use super::types::{
    PHPAnalysisCache, PHPBackend, PHPClass, PHPConstantFoldingHelper, PHPDepGraph,
    PHPDominatorTree, PHPEnum, PHPEnumCase, PHPExpr, PHPExtCache, PHPExtConstFolder,
    PHPExtDepGraph, PHPExtDomTree, PHPExtLiveness, PHPExtPassConfig, PHPExtPassPhase,
    PHPExtPassRegistry, PHPExtPassStats, PHPExtWorklist, PHPFunction, PHPInterface,
    PHPLivenessInfo, PHPParam, PHPPassConfig, PHPPassPhase, PHPPassRegistry, PHPPassStats,
    PHPProperty, PHPScript, PHPType, PHPVisibility, PHPWorklist,
};

pub(super) fn format_param(p: &PHPParam) -> std::string::String {
    let mut s = std::string::String::new();
    if let Some(vis) = &p.promoted {
        s.push_str(&format!("{} ", vis));
    }
    if let Some(ty) = &p.ty {
        s.push_str(&format!("{} ", ty));
    }
    if p.by_ref {
        s.push('&');
    }
    if p.variadic {
        s.push_str("...");
    }
    s.push_str(&format!("${}", p.name));
    if let Some(default) = &p.default {
        s.push_str(&format!(" = {}", default));
    }
    s
}
/// Minimal OxiLean PHP runtime header embedded in emitted files.
pub const PHP_RUNTIME: &str = r#"<?php
// OxiLean PHP Runtime
// Auto-generated — do not edit.

declare(strict_types=1);

namespace OxiLean\Runtime;

/** Represents a lazy thunk (unevaluated computation). */
final class Thunk
{
    private mixed $value = null;
    private bool $evaluated = false;
    /** @var callable */
    private $thunk;

    public function __construct(callable $thunk)
    {
        $this->thunk = $thunk;
    }

    public function force(): mixed
    {
        if (!$this->evaluated) {
            $this->value = ($this->thunk)();
            $this->evaluated = true;
        }
        return $this->value;
    }
}

/** Represents an OxiLean product (pair/tuple). */
final readonly class Prod
{
    public function __construct(
        public readonly mixed $fst,
        public readonly mixed $snd,
    ) {}
}

/** Represents an OxiLean sum (Either). */
abstract class Sum
{
    final public static function inl(mixed $val): self
    {
        return new class($val) extends Sum {
            public function __construct(public readonly mixed $val) {}
        };
    }
    final public static function inr(mixed $val): self
    {
        return new class($val) extends Sum {
            public function __construct(public readonly mixed $val) {}
        };
    }
}

/** Nat operations. */
final class Nat
{
    public static function succ(int $n): int { return $n + 1; }
    public static function pred(int $n): int { return max(0, $n - 1); }
    public static function add(int $a, int $b): int { return $a + $b; }
    public static function mul(int $a, int $b): int { return $a * $b; }
    public static function sub(int $a, int $b): int { return max(0, $a - $b); }
}
"#;
#[cfg(test)]
mod tests {
    use super::*;
    pub(super) fn backend() -> PHPBackend {
        PHPBackend::new()
    }
    #[test]
    pub(super) fn test_php_type_primitives() {
        let b = backend();
        assert_eq!(b.emit_type(&PHPType::Int), "int");
        assert_eq!(b.emit_type(&PHPType::Float), "float");
        assert_eq!(b.emit_type(&PHPType::String), "string");
        assert_eq!(b.emit_type(&PHPType::Bool), "bool");
        assert_eq!(b.emit_type(&PHPType::Null), "null");
        assert_eq!(b.emit_type(&PHPType::Mixed), "mixed");
        assert_eq!(b.emit_type(&PHPType::Void), "void");
        assert_eq!(b.emit_type(&PHPType::Never), "never");
        assert_eq!(b.emit_type(&PHPType::Array), "array");
        assert_eq!(b.emit_type(&PHPType::Callable), "callable");
        assert_eq!(b.emit_type(&PHPType::Self_), "self");
        assert_eq!(b.emit_type(&PHPType::Static), "static");
        assert_eq!(b.emit_type(&PHPType::Parent), "parent");
    }
    #[test]
    pub(super) fn test_php_type_nullable() {
        let b = backend();
        let ty = PHPType::Nullable(Box::new(PHPType::String));
        assert_eq!(b.emit_type(&ty), "?string");
    }
    #[test]
    pub(super) fn test_php_type_union() {
        let b = backend();
        let ty = PHPType::Union(vec![PHPType::Int, PHPType::String, PHPType::Null]);
        assert_eq!(b.emit_type(&ty), "int|string|null");
    }
    #[test]
    pub(super) fn test_php_type_intersection() {
        let b = backend();
        let ty = PHPType::Intersection(vec![
            PHPType::Named("Countable".to_string()),
            PHPType::Named("Iterator".to_string()),
        ]);
        assert_eq!(b.emit_type(&ty), "Countable&Iterator");
    }
    #[test]
    pub(super) fn test_php_type_named() {
        let b = backend();
        let ty = PHPType::Named("MyClass".to_string());
        assert_eq!(b.emit_type(&ty), "MyClass");
    }
    #[test]
    pub(super) fn test_php_expr_lit() {
        let b = backend();
        let expr = PHPExpr::Lit("42".to_string());
        assert_eq!(b.emit_expr(&expr), "42");
    }
    #[test]
    pub(super) fn test_php_expr_var() {
        let b = backend();
        let expr = PHPExpr::Var("count".to_string());
        assert_eq!(b.emit_expr(&expr), "$count");
    }
    #[test]
    pub(super) fn test_php_expr_binop() {
        let b = backend();
        let expr = PHPExpr::BinOp(
            Box::new(PHPExpr::Var("a".to_string())),
            "+".to_string(),
            Box::new(PHPExpr::Lit("1".to_string())),
        );
        assert_eq!(b.emit_expr(&expr), "($a + 1)");
    }
    #[test]
    pub(super) fn test_php_expr_func_call() {
        let b = backend();
        let expr = PHPExpr::FuncCall("strlen".to_string(), vec![PHPExpr::Var("s".to_string())]);
        assert_eq!(b.emit_expr(&expr), "strlen($s)");
    }
    #[test]
    pub(super) fn test_php_expr_array_lit() {
        let b = backend();
        let expr = PHPExpr::ArrayLit(vec![
            PHPExpr::Lit("1".to_string()),
            PHPExpr::Lit("2".to_string()),
            PHPExpr::Lit("3".to_string()),
        ]);
        assert_eq!(b.emit_expr(&expr), "[1, 2, 3]");
    }
    #[test]
    pub(super) fn test_php_expr_new() {
        let b = backend();
        let expr = PHPExpr::New("DateTime".to_string(), vec![]);
        assert_eq!(b.emit_expr(&expr), "new DateTime()");
    }
    #[test]
    pub(super) fn test_php_expr_arrow() {
        let b = backend();
        let expr = PHPExpr::Arrow(
            Box::new(PHPExpr::Var("obj".to_string())),
            "name".to_string(),
        );
        assert_eq!(b.emit_expr(&expr), "$obj->name");
    }
    #[test]
    pub(super) fn test_php_expr_null_coalesce() {
        let b = backend();
        let expr = PHPExpr::NullCoalesce(
            Box::new(PHPExpr::Var("val".to_string())),
            Box::new(PHPExpr::Lit("'default'".to_string())),
        );
        assert_eq!(b.emit_expr(&expr), "($val ?? 'default')");
    }
    #[test]
    pub(super) fn test_php_expr_static_access() {
        let b = backend();
        let expr = PHPExpr::StaticAccess("MyClass".to_string(), "CONST_VAL".to_string());
        assert_eq!(b.emit_expr(&expr), "MyClass::CONST_VAL");
    }
    #[test]
    pub(super) fn test_php_expr_index() {
        let b = backend();
        let expr = PHPExpr::Index(
            Box::new(PHPExpr::Var("arr".to_string())),
            Box::new(PHPExpr::Lit("0".to_string())),
        );
        assert_eq!(b.emit_expr(&expr), "$arr[0]");
    }
    #[test]
    pub(super) fn test_php_expr_spread() {
        let b = backend();
        let expr = PHPExpr::Spread(Box::new(PHPExpr::Var("args".to_string())));
        assert_eq!(b.emit_expr(&expr), "...$args");
    }
    #[test]
    pub(super) fn test_php_expr_cast() {
        let b = backend();
        let expr = PHPExpr::Cast("int".to_string(), Box::new(PHPExpr::Var("x".to_string())));
        assert_eq!(b.emit_expr(&expr), "(int) $x");
    }
    #[test]
    pub(super) fn test_php_expr_isset_empty() {
        let b = backend();
        let isset = PHPExpr::Isset(Box::new(PHPExpr::Var("x".to_string())));
        let empty = PHPExpr::Empty(Box::new(PHPExpr::Var("arr".to_string())));
        assert_eq!(b.emit_expr(&isset), "isset($x)");
        assert_eq!(b.emit_expr(&empty), "empty($arr)");
    }
    #[test]
    pub(super) fn test_mangle_simple() {
        let b = backend();
        assert_eq!(b.mangle_name("myFunc"), "myFunc");
        assert_eq!(b.mangle_name("my_func"), "my_func");
    }
    #[test]
    pub(super) fn test_mangle_dot_separator() {
        let b = backend();
        assert_eq!(b.mangle_name("Nat.add"), "Nat_add");
    }
    #[test]
    pub(super) fn test_mangle_reserved_word() {
        let b = backend();
        assert_eq!(b.mangle_name("match"), "match_ox");
        assert_eq!(b.mangle_name("class"), "class_ox");
        assert_eq!(b.mangle_name("enum"), "enum_ox");
    }
    #[test]
    pub(super) fn test_mangle_leading_digit() {
        let b = backend();
        let result = b.mangle_name("1foo");
        assert!(
            result.starts_with('_'),
            "should start with underscore, got: {}",
            result
        );
    }
    #[test]
    pub(super) fn test_emit_simple_function() {
        let b = backend();
        let func = PHPFunction {
            name: "add".to_string(),
            params: vec![
                PHPParam::typed("a", PHPType::Int),
                PHPParam::typed("b", PHPType::Int),
            ],
            return_type: Some(PHPType::Int),
            body: vec!["return $a + $b;".to_string()],
            is_static: false,
            is_abstract: false,
            visibility: None,
            doc_comment: None,
        };
        let code = b.emit_function(&func);
        assert!(code.contains("function add(int $a, int $b): int"));
        assert!(code.contains("return $a + $b;"));
    }
    #[test]
    pub(super) fn test_emit_static_method() {
        let b = backend();
        let func = PHPFunction::method(
            "create",
            PHPVisibility::Public,
            vec![],
            Some(PHPType::Static),
            vec!["return new static();".to_string()],
        );
        let mut f = func.clone();
        f.is_static = true;
        let code = b.emit_function(&f);
        assert!(code.contains("public static"));
        assert!(code.contains("function create()"));
    }
    #[test]
    pub(super) fn test_emit_simple_class() {
        let b = backend();
        let mut cls = PHPClass::new("Calculator");
        cls.properties
            .push(PHPProperty::private("result", Some(PHPType::Int)));
        cls.methods.push(PHPFunction::method(
            "getResult",
            PHPVisibility::Public,
            vec![],
            Some(PHPType::Int),
            vec!["return $this->result;".to_string()],
        ));
        let code = b.emit_class(&cls);
        assert!(code.contains("class Calculator"));
        assert!(code.contains("private int $result;"));
        assert!(code.contains("function getResult(): int"));
    }
    #[test]
    pub(super) fn test_emit_class_with_parent() {
        let b = backend();
        let mut cls = PHPClass::new("Dog");
        cls.parent = Some("Animal".to_string());
        cls.interfaces.push("Serializable".to_string());
        let code = b.emit_class(&cls);
        assert!(code.contains("class Dog extends Animal implements Serializable"));
    }
    #[test]
    pub(super) fn test_emit_abstract_class() {
        let b = backend();
        let cls = PHPClass::abstract_class("Shape");
        let code = b.emit_class(&cls);
        assert!(code.contains("abstract class Shape"));
    }
    #[test]
    pub(super) fn test_emit_pure_enum() {
        let b = backend();
        let mut en = PHPEnum::new("Status");
        en.cases.push(PHPEnumCase {
            name: "Active".to_string(),
            value: None,
        });
        en.cases.push(PHPEnumCase {
            name: "Inactive".to_string(),
            value: None,
        });
        let code = b.emit_enum(&en);
        assert!(code.contains("enum Status"));
        assert!(code.contains("case Active;"));
        assert!(code.contains("case Inactive;"));
    }
    #[test]
    pub(super) fn test_emit_backed_enum() {
        let b = backend();
        let mut en = PHPEnum::string_backed("Color");
        en.cases.push(PHPEnumCase {
            name: "Red".to_string(),
            value: Some("'red'".to_string()),
        });
        en.cases.push(PHPEnumCase {
            name: "Blue".to_string(),
            value: Some("'blue'".to_string()),
        });
        let code = b.emit_enum(&en);
        assert!(code.contains("enum Color: string"));
        assert!(code.contains("case Red = 'red';"));
    }
    #[test]
    pub(super) fn test_emit_interface() {
        let b = backend();
        let mut iface = PHPInterface::new("Drawable");
        iface.methods.push(PHPFunction {
            name: "draw".to_string(),
            params: vec![],
            return_type: Some(PHPType::Void),
            body: vec![],
            is_static: false,
            is_abstract: true,
            visibility: Some(PHPVisibility::Public),
            doc_comment: None,
        });
        let code = b.emit_interface(&iface);
        assert!(code.contains("interface Drawable"));
        assert!(code.contains("function draw(): void"));
    }
    #[test]
    pub(super) fn test_emit_script_strict_types() {
        let b = backend();
        let script = PHPScript::new();
        let code = b.emit_script(&script);
        assert!(code.starts_with("<?php\n"));
        assert!(code.contains("declare(strict_types=1);"));
    }
    #[test]
    pub(super) fn test_emit_script_namespace() {
        let b = backend();
        let mut script = PHPScript::new();
        script.namespace = Some("OxiLean\\Generated".to_string());
        let code = b.emit_script(&script);
        assert!(code.contains("namespace OxiLean\\Generated;"));
    }
    #[test]
    pub(super) fn test_emit_script_with_use() {
        let b = backend();
        let mut script = PHPScript::new();
        script
            .uses
            .push(("OxiLean\\Runtime\\Thunk".to_string(), None));
        script
            .uses
            .push(("OxiLean\\Runtime\\Nat".to_string(), Some("N".to_string())));
        let code = b.emit_script(&script);
        assert!(code.contains("use OxiLean\\Runtime\\Thunk;"));
        assert!(code.contains("use OxiLean\\Runtime\\Nat as N;"));
    }
    #[test]
    pub(super) fn test_emit_full_script() {
        let b = backend();
        let mut script = PHPScript::new();
        script.namespace = Some("App".to_string());
        let func = PHPFunction {
            name: "greet".to_string(),
            params: vec![PHPParam::typed("name", PHPType::String)],
            return_type: Some(PHPType::String),
            body: vec!["return 'Hello, ' . $name . '!';".to_string()],
            is_static: false,
            is_abstract: false,
            visibility: None,
            doc_comment: None,
        };
        script.functions.push(func);
        script.main.push("echo greet('World');".to_string());
        let code = b.emit_script(&script);
        assert!(code.contains("<?php"));
        assert!(code.contains("namespace App;"));
        assert!(code.contains("function greet(string $name): string"));
        assert!(code.contains("echo greet('World');"));
    }
    #[test]
    pub(super) fn test_php_runtime_constant() {
        assert!(PHP_RUNTIME.contains("OxiLean PHP Runtime"));
        assert!(PHP_RUNTIME.contains("class Thunk"));
        assert!(PHP_RUNTIME.contains("class Nat"));
        assert!(PHP_RUNTIME.contains("declare(strict_types=1)"));
    }
    #[test]
    pub(super) fn test_param_simple() {
        let p = PHPParam::simple("x");
        assert_eq!(p.name, "x");
        assert!(p.ty.is_none());
        assert!(p.default.is_none());
        assert!(!p.by_ref);
        assert!(!p.variadic);
    }
    #[test]
    pub(super) fn test_param_typed() {
        let p = PHPParam::typed("count", PHPType::Int);
        assert_eq!(p.name, "count");
        assert_eq!(p.ty, Some(PHPType::Int));
    }
    #[test]
    pub(super) fn test_param_with_default() {
        let p = PHPParam::with_default("limit", Some(PHPType::Int), PHPExpr::Lit("10".to_string()));
        assert_eq!(p.name, "limit");
        assert!(p.default.is_some());
        let formatted = format_param(&p);
        assert!(
            formatted.contains("$limit"),
            "expected $limit in: {}",
            formatted
        );
        assert!(
            formatted.contains("= 10"),
            "expected = 10 in: {}",
            formatted
        );
    }
}
#[cfg(test)]
mod PHP_infra_tests {
    use super::*;
    #[test]
    pub(super) fn test_pass_config() {
        let config = PHPPassConfig::new("test_pass", PHPPassPhase::Transformation);
        assert!(config.enabled);
        assert!(config.phase.is_modifying());
        assert_eq!(config.phase.name(), "transformation");
    }
    #[test]
    pub(super) fn test_pass_stats() {
        let mut stats = PHPPassStats::new();
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
        let mut reg = PHPPassRegistry::new();
        reg.register(PHPPassConfig::new("pass_a", PHPPassPhase::Analysis));
        reg.register(PHPPassConfig::new("pass_b", PHPPassPhase::Transformation).disabled());
        assert_eq!(reg.total_passes(), 2);
        assert_eq!(reg.enabled_count(), 1);
        reg.update_stats("pass_a", 5, 50, 2);
        let stats = reg.get_stats("pass_a").expect("stats should exist");
        assert_eq!(stats.total_changes, 5);
    }
    #[test]
    pub(super) fn test_analysis_cache() {
        let mut cache = PHPAnalysisCache::new(10);
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
        let mut wl = PHPWorklist::new();
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
        let mut dt = PHPDominatorTree::new(5);
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
        let mut liveness = PHPLivenessInfo::new(3);
        liveness.add_def(0, 1);
        liveness.add_use(1, 1);
        assert!(liveness.defs[0].contains(&1));
        assert!(liveness.uses[1].contains(&1));
    }
    #[test]
    pub(super) fn test_constant_folding() {
        assert_eq!(PHPConstantFoldingHelper::fold_add_i64(3, 4), Some(7));
        assert_eq!(PHPConstantFoldingHelper::fold_div_i64(10, 0), None);
        assert_eq!(PHPConstantFoldingHelper::fold_div_i64(10, 2), Some(5));
        assert_eq!(
            PHPConstantFoldingHelper::fold_bitand_i64(0b1100, 0b1010),
            0b1000
        );
        assert_eq!(PHPConstantFoldingHelper::fold_bitnot_i64(0), -1);
    }
    #[test]
    pub(super) fn test_dep_graph() {
        let mut g = PHPDepGraph::new();
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
#[cfg(test)]
mod phpext_pass_tests {
    use super::*;
    #[test]
    pub(super) fn test_phpext_phase_order() {
        assert_eq!(PHPExtPassPhase::Early.order(), 0);
        assert_eq!(PHPExtPassPhase::Middle.order(), 1);
        assert_eq!(PHPExtPassPhase::Late.order(), 2);
        assert_eq!(PHPExtPassPhase::Finalize.order(), 3);
        assert!(PHPExtPassPhase::Early.is_early());
        assert!(!PHPExtPassPhase::Early.is_late());
    }
    #[test]
    pub(super) fn test_phpext_config_builder() {
        let c = PHPExtPassConfig::new("p")
            .with_phase(PHPExtPassPhase::Late)
            .with_max_iter(50)
            .with_debug(1);
        assert_eq!(c.name, "p");
        assert_eq!(c.max_iterations, 50);
        assert!(c.is_debug_enabled());
        assert!(c.enabled);
        let c2 = c.disabled();
        assert!(!c2.enabled);
    }
    #[test]
    pub(super) fn test_phpext_stats() {
        let mut s = PHPExtPassStats::new();
        s.visit();
        s.visit();
        s.modify();
        s.iterate();
        assert_eq!(s.nodes_visited, 2);
        assert_eq!(s.nodes_modified, 1);
        assert!(s.changed);
        assert_eq!(s.iterations, 1);
        let e = s.efficiency();
        assert!((e - 0.5).abs() < 1e-9);
    }
    #[test]
    pub(super) fn test_phpext_registry() {
        let mut r = PHPExtPassRegistry::new();
        r.register(PHPExtPassConfig::new("a").with_phase(PHPExtPassPhase::Early));
        r.register(PHPExtPassConfig::new("b").disabled());
        assert_eq!(r.len(), 2);
        assert_eq!(r.enabled_passes().len(), 1);
        assert_eq!(r.passes_in_phase(&PHPExtPassPhase::Early).len(), 1);
    }
    #[test]
    pub(super) fn test_phpext_cache() {
        let mut c = PHPExtCache::new(4);
        assert!(c.get(99).is_none());
        c.put(99, vec![1, 2, 3]);
        let v = c.get(99).expect("v should be present in map");
        assert_eq!(v, &[1u8, 2, 3]);
        assert!(c.hit_rate() > 0.0);
        assert_eq!(c.live_count(), 1);
    }
    #[test]
    pub(super) fn test_phpext_worklist() {
        let mut w = PHPExtWorklist::new(10);
        w.push(5);
        w.push(3);
        w.push(5);
        assert_eq!(w.len(), 2);
        assert!(w.contains(5));
        let first = w.pop().expect("first should be available to pop");
        assert!(!w.contains(first));
    }
    #[test]
    pub(super) fn test_phpext_dom_tree() {
        let mut dt = PHPExtDomTree::new(5);
        dt.set_idom(1, 0);
        dt.set_idom(2, 0);
        dt.set_idom(3, 1);
        dt.set_idom(4, 1);
        assert!(dt.dominates(0, 3));
        assert!(dt.dominates(1, 4));
        assert!(!dt.dominates(2, 3));
        assert_eq!(dt.depth_of(3), 2);
    }
    #[test]
    pub(super) fn test_phpext_liveness() {
        let mut lv = PHPExtLiveness::new(3);
        lv.add_def(0, 1);
        lv.add_use(1, 1);
        assert!(lv.var_is_def_in_block(0, 1));
        assert!(lv.var_is_used_in_block(1, 1));
        assert!(!lv.var_is_def_in_block(1, 1));
    }
    #[test]
    pub(super) fn test_phpext_const_folder() {
        let mut cf = PHPExtConstFolder::new();
        assert_eq!(cf.add_i64(3, 4), Some(7));
        assert_eq!(cf.div_i64(10, 0), None);
        assert_eq!(cf.mul_i64(6, 7), Some(42));
        assert_eq!(cf.and_i64(0b1100, 0b1010), 0b1000);
        assert_eq!(cf.fold_count(), 3);
        assert_eq!(cf.failure_count(), 1);
    }
    #[test]
    pub(super) fn test_phpext_dep_graph() {
        let mut g = PHPExtDepGraph::new(4);
        g.add_edge(0, 1);
        g.add_edge(1, 2);
        g.add_edge(2, 3);
        assert!(!g.has_cycle());
        assert_eq!(g.topo_sort(), Some(vec![0, 1, 2, 3]));
        assert_eq!(g.reachable(0).len(), 4);
        let sccs = g.scc();
        assert_eq!(sccs.len(), 4);
    }
}
