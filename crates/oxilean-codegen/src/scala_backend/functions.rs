//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::{
    ScalaAnalysisCache, ScalaBackend, ScalaCaseClass, ScalaCaseClause, ScalaCatch,
    ScalaConstantFoldingHelper, ScalaDecl, ScalaDepGraph, ScalaDominatorTree, ScalaEnum,
    ScalaEnumCase, ScalaEnumerator, ScalaExpr, ScalaExtCache, ScalaExtConstFolder,
    ScalaExtDepGraph, ScalaExtDomTree, ScalaExtLiveness, ScalaExtPassConfig, ScalaExtPassPhase,
    ScalaExtPassRegistry, ScalaExtPassStats, ScalaExtWorklist, ScalaImport, ScalaLit,
    ScalaLivenessInfo, ScalaMethod, ScalaModule, ScalaObject, ScalaParam, ScalaPassConfig,
    ScalaPassPhase, ScalaPassRegistry, ScalaPassStats, ScalaPattern, ScalaTrait, ScalaType,
    ScalaWorklist,
};

/// Map an LCNF type to a Scala type.
pub fn lcnf_type_to_scala(ty: &LcnfType) -> ScalaType {
    match ty {
        LcnfType::Nat => ScalaType::Long,
        LcnfType::LcnfString => ScalaType::ScalaString,
        LcnfType::Unit | LcnfType::Erased | LcnfType::Irrelevant => ScalaType::Unit,
        LcnfType::Object => ScalaType::Any,
        LcnfType::Var(name) => ScalaType::Custom(name.clone()),
        LcnfType::Fun(params, ret) => {
            let scala_params: Vec<ScalaType> = params.iter().map(lcnf_type_to_scala).collect();
            let scala_ret = lcnf_type_to_scala(ret);
            ScalaType::Function(scala_params, Box::new(scala_ret))
        }
        LcnfType::Ctor(name, args) => {
            if args.is_empty() {
                ScalaType::Custom(name.clone())
            } else {
                ScalaType::Generic(name.clone(), args.iter().map(lcnf_type_to_scala).collect())
            }
        }
    }
}
/// Sanitize an identifier to be a valid Scala identifier.
pub(super) fn sanitize_scala_ident(name: &str) -> String {
    let s: String = name
        .chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '_' {
                c
            } else {
                '_'
            }
        })
        .collect();
    if s.is_empty() {
        "fn_".to_string()
    } else {
        s
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub(super) fn test_type_primitives() {
        assert_eq!(ScalaType::Int.to_string(), "Int");
        assert_eq!(ScalaType::Long.to_string(), "Long");
        assert_eq!(ScalaType::Double.to_string(), "Double");
        assert_eq!(ScalaType::Float.to_string(), "Float");
        assert_eq!(ScalaType::Boolean.to_string(), "Boolean");
        assert_eq!(ScalaType::Char.to_string(), "Char");
        assert_eq!(ScalaType::ScalaString.to_string(), "String");
        assert_eq!(ScalaType::Unit.to_string(), "Unit");
        assert_eq!(ScalaType::Nothing.to_string(), "Nothing");
        assert_eq!(ScalaType::Any.to_string(), "Any");
    }
    #[test]
    pub(super) fn test_type_list() {
        let ty = ScalaType::List(Box::new(ScalaType::Int));
        assert_eq!(ty.to_string(), "List[Int]");
    }
    #[test]
    pub(super) fn test_type_option() {
        let ty = ScalaType::Option(Box::new(ScalaType::ScalaString));
        assert_eq!(ty.to_string(), "Option[String]");
    }
    #[test]
    pub(super) fn test_type_either() {
        let ty = ScalaType::Either(Box::new(ScalaType::ScalaString), Box::new(ScalaType::Int));
        assert_eq!(ty.to_string(), "Either[String, Int]");
    }
    #[test]
    pub(super) fn test_type_tuple() {
        let ty = ScalaType::Tuple(vec![
            ScalaType::Int,
            ScalaType::Boolean,
            ScalaType::ScalaString,
        ]);
        assert_eq!(ty.to_string(), "(Int, Boolean, String)");
    }
    #[test]
    pub(super) fn test_type_function_single_param() {
        let ty = ScalaType::Function(vec![ScalaType::Int], Box::new(ScalaType::Boolean));
        assert_eq!(ty.to_string(), "Int => Boolean");
    }
    #[test]
    pub(super) fn test_type_function_multi_param() {
        let ty = ScalaType::Function(
            vec![ScalaType::Int, ScalaType::ScalaString],
            Box::new(ScalaType::Boolean),
        );
        assert_eq!(ty.to_string(), "(Int, String) => Boolean");
    }
    #[test]
    pub(super) fn test_type_generic() {
        let ty = ScalaType::Generic(
            "Map".to_string(),
            vec![ScalaType::ScalaString, ScalaType::Int],
        );
        assert_eq!(ty.to_string(), "Map[String, Int]");
    }
    #[test]
    pub(super) fn test_type_custom() {
        let ty = ScalaType::Custom("MyClass".to_string());
        assert_eq!(ty.to_string(), "MyClass");
    }
    #[test]
    pub(super) fn test_lit_int() {
        assert_eq!(ScalaLit::Int(42).to_string(), "42");
    }
    #[test]
    pub(super) fn test_lit_long() {
        assert_eq!(ScalaLit::Long(100).to_string(), "100L");
    }
    #[test]
    pub(super) fn test_lit_bool() {
        assert_eq!(ScalaLit::Bool(true).to_string(), "true");
        assert_eq!(ScalaLit::Bool(false).to_string(), "false");
    }
    #[test]
    pub(super) fn test_lit_char() {
        assert_eq!(ScalaLit::Char('z').to_string(), "'z'");
    }
    #[test]
    pub(super) fn test_lit_str_with_escapes() {
        let s = ScalaLit::Str("hello\nworld".to_string());
        assert_eq!(s.to_string(), "\"hello\\nworld\"");
    }
    #[test]
    pub(super) fn test_lit_null_and_unit() {
        assert_eq!(ScalaLit::Null.to_string(), "null");
        assert_eq!(ScalaLit::Unit.to_string(), "()");
    }
    #[test]
    pub(super) fn test_pattern_wildcard() {
        assert_eq!(ScalaPattern::Wildcard.to_string(), "_");
    }
    #[test]
    pub(super) fn test_pattern_var() {
        assert_eq!(ScalaPattern::Var("x".to_string()).to_string(), "x");
    }
    #[test]
    pub(super) fn test_pattern_typed() {
        let p = ScalaPattern::Typed("e".to_string(), ScalaType::Custom("Exception".to_string()));
        assert_eq!(p.to_string(), "e: Exception");
    }
    #[test]
    pub(super) fn test_pattern_extractor_no_args() {
        let p = ScalaPattern::Extractor("Nil".to_string(), Vec::new());
        assert_eq!(p.to_string(), "Nil");
    }
    #[test]
    pub(super) fn test_pattern_extractor_with_args() {
        let p =
            ScalaPattern::Extractor("Some".to_string(), vec![ScalaPattern::Var("x".to_string())]);
        assert_eq!(p.to_string(), "Some(x)");
    }
    #[test]
    pub(super) fn test_pattern_tuple() {
        let p = ScalaPattern::Tuple(vec![
            ScalaPattern::Var("a".to_string()),
            ScalaPattern::Var("b".to_string()),
        ]);
        assert_eq!(p.to_string(), "(a, b)");
    }
    #[test]
    pub(super) fn test_pattern_alt() {
        let p = ScalaPattern::Alt(vec![
            ScalaPattern::Lit(ScalaLit::Int(1)),
            ScalaPattern::Lit(ScalaLit::Int(2)),
            ScalaPattern::Lit(ScalaLit::Int(3)),
        ]);
        assert_eq!(p.to_string(), "1 | 2 | 3");
    }
    #[test]
    pub(super) fn test_expr_lambda_single() {
        let e = ScalaExpr::Lambda(
            vec!["x".to_string()],
            Box::new(ScalaExpr::Var("x".to_string())),
        );
        assert_eq!(e.to_string(), "x => x");
    }
    #[test]
    pub(super) fn test_expr_lambda_multi() {
        let e = ScalaExpr::Lambda(
            vec!["x".to_string(), "y".to_string()],
            Box::new(ScalaExpr::Infix(
                Box::new(ScalaExpr::Var("x".to_string())),
                "+".to_string(),
                Box::new(ScalaExpr::Var("y".to_string())),
            )),
        );
        assert_eq!(e.to_string(), "(x, y) => (x + y)");
    }
    #[test]
    pub(super) fn test_expr_match() {
        let e = ScalaExpr::Match(
            Box::new(ScalaExpr::Var("opt".to_string())),
            vec![
                ScalaCaseClause {
                    pattern: ScalaPattern::Extractor(
                        "Some".to_string(),
                        vec![ScalaPattern::Var("v".to_string())],
                    ),
                    guard: None,
                    body: ScalaExpr::Var("v".to_string()),
                },
                ScalaCaseClause {
                    pattern: ScalaPattern::Extractor("None".to_string(), Vec::new()),
                    guard: None,
                    body: ScalaExpr::Lit(ScalaLit::Int(0)),
                },
            ],
        );
        let s = e.to_string();
        assert!(s.contains("opt match"));
        assert!(s.contains("case Some(v) => v"));
        assert!(s.contains("case None => 0"));
    }
    #[test]
    pub(super) fn test_expr_for_comprehension() {
        let e = ScalaExpr::For(
            vec![
                ScalaEnumerator::Generator("x".to_string(), ScalaExpr::Var("list".to_string())),
                ScalaEnumerator::Guard(ScalaExpr::Infix(
                    Box::new(ScalaExpr::Var("x".to_string())),
                    ">".to_string(),
                    Box::new(ScalaExpr::Lit(ScalaLit::Int(0))),
                )),
            ],
            Box::new(ScalaExpr::Var("x".to_string())),
        );
        let s = e.to_string();
        assert!(s.contains("x <- list"));
        assert!(s.contains("if (x > 0)"));
        assert!(s.contains("yield x"));
    }
    #[test]
    pub(super) fn test_expr_try_catch() {
        let e = ScalaExpr::Try(
            Box::new(ScalaExpr::Var("riskyOp".to_string())),
            vec![ScalaCatch {
                pattern: ScalaPattern::Typed(
                    "e".to_string(),
                    ScalaType::Custom("Exception".to_string()),
                ),
                body: ScalaExpr::Lit(ScalaLit::Int(-1)),
            }],
            None,
        );
        let s = e.to_string();
        assert!(s.contains("try { riskyOp }"));
        assert!(s.contains("case e: Exception => -1"));
    }
    #[test]
    pub(super) fn test_case_class_simple() {
        let c = ScalaCaseClass {
            name: "Point".to_string(),
            type_params: Vec::new(),
            fields: vec![
                ScalaParam {
                    name: "x".to_string(),
                    ty: ScalaType::Int,
                    default: None,
                },
                ScalaParam {
                    name: "y".to_string(),
                    ty: ScalaType::Int,
                    default: None,
                },
            ],
            extends_list: Vec::new(),
        };
        assert_eq!(c.to_string(), "case class Point(x: Int, y: Int)");
    }
    #[test]
    pub(super) fn test_case_class_with_type_params() {
        let c = ScalaCaseClass {
            name: "Pair".to_string(),
            type_params: vec!["A".to_string(), "B".to_string()],
            fields: vec![
                ScalaParam {
                    name: "first".to_string(),
                    ty: ScalaType::Custom("A".to_string()),
                    default: None,
                },
                ScalaParam {
                    name: "second".to_string(),
                    ty: ScalaType::Custom("B".to_string()),
                    default: None,
                },
            ],
            extends_list: Vec::new(),
        };
        assert_eq!(c.to_string(), "case class Pair[A, B](first: A, second: B)");
    }
    #[test]
    pub(super) fn test_enum_simple_cases() {
        let e = ScalaEnum {
            name: "Color".to_string(),
            type_params: Vec::new(),
            cases: vec![
                ScalaEnumCase {
                    name: "Red".to_string(),
                    fields: Vec::new(),
                },
                ScalaEnumCase {
                    name: "Green".to_string(),
                    fields: Vec::new(),
                },
                ScalaEnumCase {
                    name: "Blue".to_string(),
                    fields: Vec::new(),
                },
            ],
            extends_list: Vec::new(),
        };
        let s = e.to_string();
        assert!(s.contains("enum Color:"));
        assert!(s.contains("case Red, Green, Blue"));
    }
    #[test]
    pub(super) fn test_enum_adt() {
        let e = ScalaEnum {
            name: "Expr".to_string(),
            type_params: Vec::new(),
            cases: vec![
                ScalaEnumCase {
                    name: "Lit".to_string(),
                    fields: vec![ScalaParam {
                        name: "n".to_string(),
                        ty: ScalaType::Int,
                        default: None,
                    }],
                },
                ScalaEnumCase {
                    name: "Add".to_string(),
                    fields: vec![
                        ScalaParam {
                            name: "l".to_string(),
                            ty: ScalaType::Custom("Expr".to_string()),
                            default: None,
                        },
                        ScalaParam {
                            name: "r".to_string(),
                            ty: ScalaType::Custom("Expr".to_string()),
                            default: None,
                        },
                    ],
                },
            ],
            extends_list: Vec::new(),
        };
        let s = e.to_string();
        assert!(s.contains("case Lit(n: Int)"));
        assert!(s.contains("case Add(l: Expr, r: Expr)"));
    }
    #[test]
    pub(super) fn test_trait() {
        let t = ScalaTrait {
            name: "Functor".to_string(),
            type_params: vec!["F[_]".to_string()],
            extends_list: Vec::new(),
            abstract_methods: vec![ScalaMethod {
                name: "map".to_string(),
                type_params: vec!["A".to_string(), "B".to_string()],
                params: vec![vec![
                    ScalaParam {
                        name: "fa".to_string(),
                        ty: ScalaType::Custom("F[A]".to_string()),
                        default: None,
                    },
                    ScalaParam {
                        name: "f".to_string(),
                        ty: ScalaType::Custom("A => B".to_string()),
                        default: None,
                    },
                ]],
                return_type: ScalaType::Custom("F[B]".to_string()),
                body: None,
                modifiers: Vec::new(),
            }],
            concrete_methods: Vec::new(),
        };
        let s = t.to_string();
        assert!(s.contains("trait Functor[F[_]]"));
        assert!(s.contains("def map[A, B]"));
    }
    #[test]
    pub(super) fn test_object() {
        let o = ScalaObject {
            name: "MathUtils".to_string(),
            extends_list: Vec::new(),
            constants: vec![(
                "PI".to_string(),
                ScalaType::Double,
                ScalaExpr::Lit(ScalaLit::Double(3.14159)),
            )],
            methods: Vec::new(),
        };
        let s = o.to_string();
        assert!(s.contains("object MathUtils {"));
        assert!(s.contains("val PI: Double = 3.14159"));
    }
    #[test]
    pub(super) fn test_module_with_package() {
        let mut m = ScalaModule::new(Some("com.example"));
        m.add_import(ScalaImport {
            path: "scala.collection.mutable".to_string(),
            items: vec!["ListBuffer".to_string()],
        });
        m.add_decl(ScalaDecl::Comment("Generated by OxiLean".to_string()));
        let s = m.emit();
        assert!(s.contains("package com.example"));
        assert!(s.contains("import scala.collection.mutable.ListBuffer"));
        assert!(s.contains("// Generated by OxiLean"));
    }
    #[test]
    pub(super) fn test_module_no_package() {
        let m: ScalaModule = ScalaModule::new(None::<String>);
        let s = m.emit();
        assert!(!s.contains("package"));
    }
    #[test]
    pub(super) fn test_import_wildcard() {
        let imp = ScalaImport {
            path: "scala.collection.mutable".to_string(),
            items: vec!["*".to_string()],
        };
        assert_eq!(imp.to_string(), "import scala.collection.mutable.*");
    }
    #[test]
    pub(super) fn test_import_specific() {
        let imp = ScalaImport {
            path: "scala.util".to_string(),
            items: vec![
                "Try".to_string(),
                "Success".to_string(),
                "Failure".to_string(),
            ],
        };
        assert_eq!(imp.to_string(), "import scala.util.{Try, Success, Failure}");
    }
    #[test]
    pub(super) fn test_lcnf_type_to_scala_nat() {
        assert_eq!(lcnf_type_to_scala(&LcnfType::Nat), ScalaType::Long);
    }
    #[test]
    pub(super) fn test_lcnf_type_to_scala_string() {
        assert_eq!(
            lcnf_type_to_scala(&LcnfType::LcnfString),
            ScalaType::ScalaString
        );
    }
    #[test]
    pub(super) fn test_lcnf_type_to_scala_fun() {
        let ty = LcnfType::Fun(vec![LcnfType::Nat], Box::new(LcnfType::LcnfString));
        let scala = lcnf_type_to_scala(&ty);
        assert_eq!(
            scala,
            ScalaType::Function(vec![ScalaType::Long], Box::new(ScalaType::ScalaString))
        );
    }
    #[test]
    pub(super) fn test_sanitize_ident() {
        assert_eq!(sanitize_scala_ident("foo"), "foo");
        assert_eq!(sanitize_scala_ident("foo.bar"), "foo_bar");
        assert_eq!(sanitize_scala_ident("foo-bar"), "foo_bar");
    }
    #[test]
    pub(super) fn test_backend_emit_module() {
        let backend = ScalaBackend::new(Some("org.oxilean"));
        let src = backend.emit_module();
        assert!(src.contains("package org.oxilean"));
        assert!(src.contains("import scala.annotation.tailrec"));
    }
    #[test]
    pub(super) fn test_extension_decl() {
        let d = ScalaDecl::Extension(
            ScalaType::Int,
            vec![ScalaMethod {
                name: "doubled".to_string(),
                type_params: Vec::new(),
                params: Vec::new(),
                return_type: ScalaType::Int,
                body: Some(ScalaExpr::Infix(
                    Box::new(ScalaExpr::Var("x".to_string())),
                    "*".to_string(),
                    Box::new(ScalaExpr::Lit(ScalaLit::Int(2))),
                )),
                modifiers: Vec::new(),
            }],
        );
        let s = d.to_string();
        assert!(s.contains("extension (x: Int)"));
        assert!(s.contains("def doubled"));
    }
    #[test]
    pub(super) fn test_given_decl() {
        let d = ScalaDecl::Given(
            "intOrd".to_string(),
            ScalaType::Generic("Ordering".to_string(), vec![ScalaType::Int]),
            Vec::new(),
        );
        let s = d.to_string();
        assert!(s.contains("given intOrd: Ordering[Int] with"));
    }
    #[test]
    pub(super) fn test_opaque_type() {
        let d = ScalaDecl::OpaqueType("Nat".to_string(), Vec::new(), ScalaType::Int);
        assert_eq!(d.to_string(), "opaque type Nat = Int");
    }
    #[test]
    pub(super) fn test_type_alias() {
        let d = ScalaDecl::TypeAlias(
            "IntList".to_string(),
            Vec::new(),
            ScalaType::List(Box::new(ScalaType::Int)),
        );
        assert_eq!(d.to_string(), "type IntList = List[Int]");
    }
}
#[cfg(test)]
mod Scala_infra_tests {
    use super::*;
    #[test]
    pub(super) fn test_pass_config() {
        let config = ScalaPassConfig::new("test_pass", ScalaPassPhase::Transformation);
        assert!(config.enabled);
        assert!(config.phase.is_modifying());
        assert_eq!(config.phase.name(), "transformation");
    }
    #[test]
    pub(super) fn test_pass_stats() {
        let mut stats = ScalaPassStats::new();
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
        let mut reg = ScalaPassRegistry::new();
        reg.register(ScalaPassConfig::new("pass_a", ScalaPassPhase::Analysis));
        reg.register(ScalaPassConfig::new("pass_b", ScalaPassPhase::Transformation).disabled());
        assert_eq!(reg.total_passes(), 2);
        assert_eq!(reg.enabled_count(), 1);
        reg.update_stats("pass_a", 5, 50, 2);
        let stats = reg.get_stats("pass_a").expect("stats should exist");
        assert_eq!(stats.total_changes, 5);
    }
    #[test]
    pub(super) fn test_analysis_cache() {
        let mut cache = ScalaAnalysisCache::new(10);
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
        let mut wl = ScalaWorklist::new();
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
        let mut dt = ScalaDominatorTree::new(5);
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
        let mut liveness = ScalaLivenessInfo::new(3);
        liveness.add_def(0, 1);
        liveness.add_use(1, 1);
        assert!(liveness.defs[0].contains(&1));
        assert!(liveness.uses[1].contains(&1));
    }
    #[test]
    pub(super) fn test_constant_folding() {
        assert_eq!(ScalaConstantFoldingHelper::fold_add_i64(3, 4), Some(7));
        assert_eq!(ScalaConstantFoldingHelper::fold_div_i64(10, 0), None);
        assert_eq!(ScalaConstantFoldingHelper::fold_div_i64(10, 2), Some(5));
        assert_eq!(
            ScalaConstantFoldingHelper::fold_bitand_i64(0b1100, 0b1010),
            0b1000
        );
        assert_eq!(ScalaConstantFoldingHelper::fold_bitnot_i64(0), -1);
    }
    #[test]
    pub(super) fn test_dep_graph() {
        let mut g = ScalaDepGraph::new();
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
mod scalaext_pass_tests {
    use super::*;
    #[test]
    pub(super) fn test_scalaext_phase_order() {
        assert_eq!(ScalaExtPassPhase::Early.order(), 0);
        assert_eq!(ScalaExtPassPhase::Middle.order(), 1);
        assert_eq!(ScalaExtPassPhase::Late.order(), 2);
        assert_eq!(ScalaExtPassPhase::Finalize.order(), 3);
        assert!(ScalaExtPassPhase::Early.is_early());
        assert!(!ScalaExtPassPhase::Early.is_late());
    }
    #[test]
    pub(super) fn test_scalaext_config_builder() {
        let c = ScalaExtPassConfig::new("p")
            .with_phase(ScalaExtPassPhase::Late)
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
    pub(super) fn test_scalaext_stats() {
        let mut s = ScalaExtPassStats::new();
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
    pub(super) fn test_scalaext_registry() {
        let mut r = ScalaExtPassRegistry::new();
        r.register(ScalaExtPassConfig::new("a").with_phase(ScalaExtPassPhase::Early));
        r.register(ScalaExtPassConfig::new("b").disabled());
        assert_eq!(r.len(), 2);
        assert_eq!(r.enabled_passes().len(), 1);
        assert_eq!(r.passes_in_phase(&ScalaExtPassPhase::Early).len(), 1);
    }
    #[test]
    pub(super) fn test_scalaext_cache() {
        let mut c = ScalaExtCache::new(4);
        assert!(c.get(99).is_none());
        c.put(99, vec![1, 2, 3]);
        let v = c.get(99).expect("v should be present in map");
        assert_eq!(v, &[1u8, 2, 3]);
        assert!(c.hit_rate() > 0.0);
        assert_eq!(c.live_count(), 1);
    }
    #[test]
    pub(super) fn test_scalaext_worklist() {
        let mut w = ScalaExtWorklist::new(10);
        w.push(5);
        w.push(3);
        w.push(5);
        assert_eq!(w.len(), 2);
        assert!(w.contains(5));
        let first = w.pop().expect("first should be available to pop");
        assert!(!w.contains(first));
    }
    #[test]
    pub(super) fn test_scalaext_dom_tree() {
        let mut dt = ScalaExtDomTree::new(5);
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
    pub(super) fn test_scalaext_liveness() {
        let mut lv = ScalaExtLiveness::new(3);
        lv.add_def(0, 1);
        lv.add_use(1, 1);
        assert!(lv.var_is_def_in_block(0, 1));
        assert!(lv.var_is_used_in_block(1, 1));
        assert!(!lv.var_is_def_in_block(1, 1));
    }
    #[test]
    pub(super) fn test_scalaext_const_folder() {
        let mut cf = ScalaExtConstFolder::new();
        assert_eq!(cf.add_i64(3, 4), Some(7));
        assert_eq!(cf.div_i64(10, 0), None);
        assert_eq!(cf.mul_i64(6, 7), Some(42));
        assert_eq!(cf.and_i64(0b1100, 0b1010), 0b1000);
        assert_eq!(cf.fold_count(), 3);
        assert_eq!(cf.failure_count(), 1);
    }
    #[test]
    pub(super) fn test_scalaext_dep_graph() {
        let mut g = ScalaExtDepGraph::new(4);
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
