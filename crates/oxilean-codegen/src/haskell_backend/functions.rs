//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::{
    HaskellBackend, HaskellDataDecl, HaskellDecl, HaskellDoStmt, HaskellEquation, HaskellExpr,
    HaskellFunction, HaskellGuard, HaskellImport, HaskellLit, HaskellModule, HaskellNewtype,
    HaskellPattern, HaskellType, HaskellTypeClass, HsExtConfig, HsExtDiagCollector, HsExtDiagMsg,
    HsExtEmitStats, HsExtEventLog, HsExtFeatures, HsExtIdGen, HsExtIncrKey, HsExtNameScope,
    HsExtPassTiming, HsExtProfiler, HsExtSourceBuffer, HsExtVersion, HsListQual, HskAnalysisCache,
    HskConstantFoldingHelper, HskDepGraph, HskDominatorTree, HskLivenessInfo, HskPassConfig,
    HskPassPhase, HskPassRegistry, HskPassStats, HskWorklist,
};

/// Wrap a type in parentheses if it is a compound type (for applications).
pub(super) fn paren_type(ty: &HaskellType) -> String {
    match ty {
        HaskellType::Fun(_, _)
        | HaskellType::IO(_)
        | HaskellType::Maybe(_)
        | HaskellType::Either(_, _) => format!("({})", ty),
        _ => format!("{}", ty),
    }
}
/// Wrap a function type in parentheses only when used as left side of `->`
pub(super) fn paren_fun_type(ty: &HaskellType) -> String {
    match ty {
        HaskellType::Fun(_, _) => format!("({})", ty),
        _ => paren_type(ty),
    }
}
pub(super) fn paren_pattern(pat: &HaskellPattern) -> String {
    match pat {
        HaskellPattern::Constructor(_, args) if !args.is_empty() => format!("({})", pat),
        HaskellPattern::Cons(_, _) => format!("({})", pat),
        HaskellPattern::As(_, _) => format!("({})", pat),
        HaskellPattern::LazyPat(_) => format!("({})", pat),
        _ => format!("{}", pat),
    }
}
/// Map an LCNF type to a Haskell type.
pub fn lcnf_type_to_haskell(ty: &LcnfType) -> HaskellType {
    match ty {
        LcnfType::Nat => HaskellType::Integer,
        LcnfType::LcnfString => HaskellType::HsString,
        LcnfType::Unit | LcnfType::Erased | LcnfType::Irrelevant => HaskellType::Unit,
        LcnfType::Object => HaskellType::Polymorphic("a".to_string()),
        LcnfType::Var(name) => HaskellType::Custom(name.clone()),
        LcnfType::Fun(params, ret) => {
            let hs_ret = lcnf_type_to_haskell(ret);
            params.iter().rev().fold(hs_ret, |acc, p| {
                HaskellType::Fun(Box::new(lcnf_type_to_haskell(p)), Box::new(acc))
            })
        }
        LcnfType::Ctor(name, _args) => HaskellType::Custom(name.clone()),
    }
}
/// Sanitize an identifier to be a valid Haskell identifier.
pub(super) fn sanitize_hs_ident(name: &str) -> String {
    let s: String = name
        .chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '_' || c == '\'' {
                c
            } else {
                '_'
            }
        })
        .collect();
    if s.starts_with(|c: char| c.is_uppercase()) {
        format!("fn_{}", s)
    } else if s.is_empty() {
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
        assert_eq!(HaskellType::Int.to_string(), "Int");
        assert_eq!(HaskellType::Integer.to_string(), "Integer");
        assert_eq!(HaskellType::Double.to_string(), "Double");
        assert_eq!(HaskellType::Float.to_string(), "Float");
        assert_eq!(HaskellType::Bool.to_string(), "Bool");
        assert_eq!(HaskellType::Char.to_string(), "Char");
        assert_eq!(HaskellType::HsString.to_string(), "String");
        assert_eq!(HaskellType::Unit.to_string(), "()");
    }
    #[test]
    pub(super) fn test_type_io() {
        let io_int = HaskellType::IO(Box::new(HaskellType::Int));
        assert_eq!(io_int.to_string(), "IO Int");
    }
    #[test]
    pub(super) fn test_type_list() {
        let list_int = HaskellType::List(Box::new(HaskellType::Int));
        assert_eq!(list_int.to_string(), "[Int]");
    }
    #[test]
    pub(super) fn test_type_maybe() {
        let maybe_str = HaskellType::Maybe(Box::new(HaskellType::HsString));
        assert_eq!(maybe_str.to_string(), "Maybe String");
    }
    #[test]
    pub(super) fn test_type_either() {
        let either =
            HaskellType::Either(Box::new(HaskellType::HsString), Box::new(HaskellType::Int));
        assert_eq!(either.to_string(), "Either String Int");
    }
    #[test]
    pub(super) fn test_type_tuple() {
        let tup = HaskellType::Tuple(vec![
            HaskellType::Int,
            HaskellType::Bool,
            HaskellType::HsString,
        ]);
        assert_eq!(tup.to_string(), "(Int, Bool, String)");
    }
    #[test]
    pub(super) fn test_type_fun() {
        let fun = HaskellType::Fun(Box::new(HaskellType::Int), Box::new(HaskellType::Bool));
        assert_eq!(fun.to_string(), "Int -> Bool");
    }
    #[test]
    pub(super) fn test_type_fun_nested() {
        let inner = HaskellType::Fun(Box::new(HaskellType::Int), Box::new(HaskellType::Int));
        let outer = HaskellType::Fun(Box::new(inner), Box::new(HaskellType::Bool));
        assert_eq!(outer.to_string(), "(Int -> Int) -> Bool");
    }
    #[test]
    pub(super) fn test_type_polymorphic() {
        let p = HaskellType::Polymorphic("a".to_string());
        assert_eq!(p.to_string(), "a");
    }
    #[test]
    pub(super) fn test_type_constraint() {
        let c = HaskellType::Constraint(
            "Eq".to_string(),
            vec![HaskellType::Polymorphic("a".to_string())],
        );
        assert_eq!(c.to_string(), "Eq a");
    }
    #[test]
    pub(super) fn test_lit_int_positive() {
        assert_eq!(HaskellLit::Int(42).to_string(), "42");
    }
    #[test]
    pub(super) fn test_lit_int_negative() {
        assert_eq!(HaskellLit::Int(-7).to_string(), "(-7)");
    }
    #[test]
    pub(super) fn test_lit_bool() {
        assert_eq!(HaskellLit::Bool(true).to_string(), "True");
        assert_eq!(HaskellLit::Bool(false).to_string(), "False");
    }
    #[test]
    pub(super) fn test_lit_char() {
        assert_eq!(HaskellLit::Char('a').to_string(), "'a'");
    }
    #[test]
    pub(super) fn test_lit_str_with_escapes() {
        let s = HaskellLit::Str("hello\nworld".to_string());
        assert_eq!(s.to_string(), "\"hello\\nworld\"");
    }
    #[test]
    pub(super) fn test_lit_unit() {
        assert_eq!(HaskellLit::Unit.to_string(), "()");
    }
    #[test]
    pub(super) fn test_pattern_wildcard() {
        assert_eq!(HaskellPattern::Wildcard.to_string(), "_");
    }
    #[test]
    pub(super) fn test_pattern_var() {
        assert_eq!(HaskellPattern::Var("xs".to_string()).to_string(), "xs");
    }
    #[test]
    pub(super) fn test_pattern_constructor() {
        let p = HaskellPattern::Constructor(
            "Just".to_string(),
            vec![HaskellPattern::Var("x".to_string())],
        );
        assert_eq!(p.to_string(), "Just x");
    }
    #[test]
    pub(super) fn test_pattern_cons() {
        let p = HaskellPattern::Cons(
            Box::new(HaskellPattern::Var("x".to_string())),
            Box::new(HaskellPattern::Var("xs".to_string())),
        );
        assert_eq!(p.to_string(), "(x : xs)");
    }
    #[test]
    pub(super) fn test_pattern_tuple() {
        let p = HaskellPattern::Tuple(vec![
            HaskellPattern::Var("a".to_string()),
            HaskellPattern::Var("b".to_string()),
        ]);
        assert_eq!(p.to_string(), "(a, b)");
    }
    #[test]
    pub(super) fn test_pattern_as() {
        let inner = Box::new(HaskellPattern::Constructor(
            "Just".to_string(),
            vec![HaskellPattern::Var("x".to_string())],
        ));
        let p = HaskellPattern::As("v".to_string(), inner);
        assert_eq!(p.to_string(), "v@(Just x)");
    }
    #[test]
    pub(super) fn test_pattern_lazy() {
        let p = HaskellPattern::LazyPat(Box::new(HaskellPattern::Var("x".to_string())));
        assert_eq!(p.to_string(), "~x");
    }
    #[test]
    pub(super) fn test_expr_lambda() {
        let e = HaskellExpr::Lambda(
            vec![HaskellPattern::Var("x".to_string())],
            Box::new(HaskellExpr::Var("x".to_string())),
        );
        assert_eq!(e.to_string(), "(\\x -> x)");
    }
    #[test]
    pub(super) fn test_expr_infix() {
        let e = HaskellExpr::InfixApp(
            Box::new(HaskellExpr::Lit(HaskellLit::Int(1))),
            "+".to_string(),
            Box::new(HaskellExpr::Lit(HaskellLit::Int(2))),
        );
        assert_eq!(e.to_string(), "(1 + 2)");
    }
    #[test]
    pub(super) fn test_expr_list_comp() {
        let e = HaskellExpr::ListComp(
            Box::new(HaskellExpr::InfixApp(
                Box::new(HaskellExpr::Var("x".to_string())),
                "*".to_string(),
                Box::new(HaskellExpr::Var("x".to_string())),
            )),
            vec![HsListQual::Generator(
                "x".to_string(),
                HaskellExpr::App(
                    Box::new(HaskellExpr::Var("enumFromTo".to_string())),
                    vec![
                        HaskellExpr::Lit(HaskellLit::Int(1)),
                        HaskellExpr::Lit(HaskellLit::Int(10)),
                    ],
                ),
            )],
        );
        assert!(e.to_string().contains("x <- "));
        assert!(e.to_string().contains("x * x"));
    }
    #[test]
    pub(super) fn test_expr_type_annotation() {
        let e = HaskellExpr::TypeAnnotation(
            Box::new(HaskellExpr::Lit(HaskellLit::Int(42))),
            HaskellType::Int,
        );
        assert_eq!(e.to_string(), "(42 :: Int)");
    }
    #[test]
    pub(super) fn test_data_decl_simple() {
        let d = HaskellDataDecl {
            name: "Color".to_string(),
            type_params: Vec::new(),
            constructors: vec![
                ("Red".to_string(), Vec::new()),
                ("Green".to_string(), Vec::new()),
                ("Blue".to_string(), Vec::new()),
            ],
            deriving_clauses: vec!["Show".to_string(), "Eq".to_string()],
        };
        let s = d.to_string();
        assert!(s.contains("data Color"));
        assert!(s.contains("= Red"));
        assert!(s.contains("| Green"));
        assert!(s.contains("| Blue"));
        assert!(s.contains("deriving (Show, Eq)"));
    }
    #[test]
    pub(super) fn test_data_decl_with_fields() {
        let d = HaskellDataDecl {
            name: "Expr".to_string(),
            type_params: Vec::new(),
            constructors: vec![
                ("Lit".to_string(), vec![HaskellType::Int]),
                (
                    "Add".to_string(),
                    vec![
                        HaskellType::Custom("Expr".to_string()),
                        HaskellType::Custom("Expr".to_string()),
                    ],
                ),
            ],
            deriving_clauses: vec!["Show".to_string()],
        };
        let s = d.to_string();
        assert!(s.contains("Lit Int"));
        assert!(s.contains("Add Expr Expr"));
    }
    #[test]
    pub(super) fn test_newtype() {
        let n = HaskellNewtype {
            name: "Name".to_string(),
            type_param: None,
            constructor: "Name".to_string(),
            field: ("unName".to_string(), HaskellType::HsString),
            deriving_clauses: vec!["Show".to_string(), "Eq".to_string()],
        };
        let s = n.to_string();
        assert!(s.contains("newtype Name"));
        assert!(s.contains("{ unName :: String }"));
        assert!(s.contains("deriving (Show, Eq)"));
    }
    #[test]
    pub(super) fn test_typeclass() {
        let c = HaskellTypeClass {
            name: "Container".to_string(),
            type_params: vec!["f".to_string()],
            superclasses: Vec::new(),
            methods: vec![(
                "empty".to_string(),
                HaskellType::Custom("f a".to_string()),
                None,
            )],
        };
        let s = c.to_string();
        assert!(s.contains("class Container f where"));
        assert!(s.contains("empty :: f a"));
    }
    #[test]
    pub(super) fn test_function_single_equation() {
        let f = HaskellFunction {
            name: "double".to_string(),
            type_annotation: Some(HaskellType::Fun(
                Box::new(HaskellType::Int),
                Box::new(HaskellType::Int),
            )),
            equations: vec![HaskellEquation {
                patterns: vec![HaskellPattern::Var("n".to_string())],
                guards: Vec::new(),
                body: Some(HaskellExpr::InfixApp(
                    Box::new(HaskellExpr::Lit(HaskellLit::Int(2))),
                    "*".to_string(),
                    Box::new(HaskellExpr::Var("n".to_string())),
                )),
                where_clause: Vec::new(),
            }],
        };
        let s = f.to_string();
        assert!(s.contains("double :: Int -> Int"));
        assert!(s.contains("double n = (2 * n)"));
    }
    #[test]
    pub(super) fn test_function_with_guards() {
        let f = HaskellFunction {
            name: "signum'".to_string(),
            type_annotation: None,
            equations: vec![HaskellEquation {
                patterns: vec![HaskellPattern::Var("x".to_string())],
                guards: vec![
                    HaskellGuard {
                        condition: HaskellExpr::InfixApp(
                            Box::new(HaskellExpr::Var("x".to_string())),
                            ">".to_string(),
                            Box::new(HaskellExpr::Lit(HaskellLit::Int(0))),
                        ),
                        body: HaskellExpr::Lit(HaskellLit::Int(1)),
                    },
                    HaskellGuard {
                        condition: HaskellExpr::Var("otherwise".to_string()),
                        body: HaskellExpr::Lit(HaskellLit::Int(-1)),
                    },
                ],
                body: None,
                where_clause: Vec::new(),
            }],
        };
        let s = f.to_string();
        assert!(s.contains("| (x > 0) = 1"));
        assert!(s.contains("| otherwise = (-1)"));
    }
    #[test]
    pub(super) fn test_module_emit() {
        let mut m = HaskellModule::new("MyModule");
        m.add_import(HaskellImport {
            module: "Data.List".to_string(),
            qualified: false,
            alias: None,
            items: vec!["sort".to_string()],
            hiding: Vec::new(),
        });
        m.add_decl(HaskellDecl::Comment("Example module".to_string()));
        let s = m.emit();
        assert!(s.contains("module MyModule where"));
        assert!(s.contains("import Data.List (sort)"));
        assert!(s.contains("-- Example module"));
    }
    #[test]
    pub(super) fn test_module_with_exports() {
        let mut m = HaskellModule::new("Lib");
        m.exports = vec!["foo".to_string(), "bar".to_string()];
        let s = m.emit();
        assert!(s.contains("module Lib ("));
        assert!(s.contains("  foo"));
        assert!(s.contains(") where"));
    }
    #[test]
    pub(super) fn test_import_qualified() {
        let imp = HaskellImport {
            module: "Data.Map.Strict".to_string(),
            qualified: true,
            alias: Some("Map".to_string()),
            items: Vec::new(),
            hiding: Vec::new(),
        };
        assert_eq!(imp.to_string(), "import qualified Data.Map.Strict as Map");
    }
    #[test]
    pub(super) fn test_import_hiding() {
        let imp = HaskellImport {
            module: "Prelude".to_string(),
            qualified: false,
            alias: None,
            items: Vec::new(),
            hiding: vec!["lookup".to_string()],
        };
        assert_eq!(imp.to_string(), "import Prelude hiding (lookup)");
    }
    #[test]
    pub(super) fn test_lcnf_type_to_haskell_nat() {
        assert_eq!(lcnf_type_to_haskell(&LcnfType::Nat), HaskellType::Integer);
    }
    #[test]
    pub(super) fn test_lcnf_type_to_haskell_string() {
        assert_eq!(
            lcnf_type_to_haskell(&LcnfType::LcnfString),
            HaskellType::HsString
        );
    }
    #[test]
    pub(super) fn test_lcnf_type_to_haskell_fun() {
        let ty = LcnfType::Fun(vec![LcnfType::Nat], Box::new(LcnfType::LcnfString));
        let hs = lcnf_type_to_haskell(&ty);
        assert_eq!(
            hs,
            HaskellType::Fun(
                Box::new(HaskellType::Integer),
                Box::new(HaskellType::HsString)
            )
        );
    }
    #[test]
    pub(super) fn test_sanitize_ident() {
        assert_eq!(sanitize_hs_ident("foo"), "foo");
        assert_eq!(sanitize_hs_ident("Foo"), "fn_Foo");
        assert_eq!(sanitize_hs_ident("foo.bar"), "foo_bar");
    }
    #[test]
    pub(super) fn test_backend_emit_module() {
        let backend = HaskellBackend::new("Generated");
        let src = backend.emit_module();
        assert!(src.contains("module Generated where"));
        assert!(src.contains("import Prelude"));
    }
    #[test]
    pub(super) fn test_do_notation_display() {
        let e = HaskellExpr::Do(vec![
            HaskellDoStmt::Bind("x".to_string(), HaskellExpr::Var("getLine".to_string())),
            HaskellDoStmt::Stmt(HaskellExpr::App(
                Box::new(HaskellExpr::Var("putStrLn".to_string())),
                vec![HaskellExpr::Var("x".to_string())],
            )),
        ]);
        let s = e.to_string();
        assert!(s.contains("x <- getLine"));
        assert!(s.contains("putStrLn"));
    }
    #[test]
    pub(super) fn test_type_synonym() {
        let d = HaskellDecl::TypeSynonym("Name".to_string(), Vec::new(), HaskellType::HsString);
        assert_eq!(d.to_string(), "type Name = String");
    }
}
#[cfg(test)]
mod tests_hs_ext_extra {
    use super::*;
    #[test]
    pub(super) fn test_hs_ext_config() {
        let mut cfg = HsExtConfig::new();
        cfg.set("mode", "release");
        cfg.set("verbose", "true");
        assert_eq!(cfg.get("mode"), Some("release"));
        assert!(cfg.get_bool("verbose"));
        assert!(cfg.get_int("mode").is_none());
        assert_eq!(cfg.len(), 2);
    }
    #[test]
    pub(super) fn test_hs_ext_source_buffer() {
        let mut buf = HsExtSourceBuffer::new();
        buf.push_line("fn main() {");
        buf.indent();
        buf.push_line("println!(\"hello\");");
        buf.dedent();
        buf.push_line("}");
        assert!(buf.as_str().contains("fn main()"));
        assert!(buf.as_str().contains("    println!"));
        assert_eq!(buf.line_count(), 3);
        buf.reset();
        assert!(buf.is_empty());
    }
    #[test]
    pub(super) fn test_hs_ext_name_scope() {
        let mut scope = HsExtNameScope::new();
        assert!(scope.declare("x"));
        assert!(!scope.declare("x"));
        assert!(scope.is_declared("x"));
        let scope = scope.push_scope();
        assert_eq!(scope.depth(), 1);
        let mut scope = scope.pop_scope();
        assert_eq!(scope.depth(), 0);
        scope.declare("y");
        assert_eq!(scope.len(), 2);
    }
    #[test]
    pub(super) fn test_hs_ext_diag_collector() {
        let mut col = HsExtDiagCollector::new();
        col.emit(HsExtDiagMsg::warning("pass_a", "slow"));
        col.emit(HsExtDiagMsg::error("pass_b", "fatal"));
        assert!(col.has_errors());
        assert_eq!(col.errors().len(), 1);
        assert_eq!(col.warnings().len(), 1);
        col.clear();
        assert!(col.is_empty());
    }
    #[test]
    pub(super) fn test_hs_ext_id_gen() {
        let mut gen = HsExtIdGen::new();
        assert_eq!(gen.next_id(), 0);
        assert_eq!(gen.next_id(), 1);
        gen.skip(10);
        assert_eq!(gen.next_id(), 12);
        gen.reset();
        assert_eq!(gen.peek_next(), 0);
    }
    #[test]
    pub(super) fn test_hs_ext_incr_key() {
        let k1 = HsExtIncrKey::new(100, 200);
        let k2 = HsExtIncrKey::new(100, 200);
        let k3 = HsExtIncrKey::new(999, 200);
        assert!(k1.matches(&k2));
        assert!(!k1.matches(&k3));
    }
    #[test]
    pub(super) fn test_hs_ext_profiler() {
        let mut p = HsExtProfiler::new();
        p.record(HsExtPassTiming::new("pass_a", 1000, 50, 200, 100));
        p.record(HsExtPassTiming::new("pass_b", 500, 30, 100, 200));
        assert_eq!(p.total_elapsed_us(), 1500);
        assert_eq!(
            p.slowest_pass()
                .expect("slowest pass should exist")
                .pass_name,
            "pass_a"
        );
        assert_eq!(p.profitable_passes().len(), 1);
    }
    #[test]
    pub(super) fn test_hs_ext_event_log() {
        let mut log = HsExtEventLog::new(3);
        log.push("event1");
        log.push("event2");
        log.push("event3");
        assert_eq!(log.len(), 3);
        log.push("event4");
        assert_eq!(log.len(), 3);
        assert_eq!(
            log.iter()
                .next()
                .expect("iterator should have next element"),
            "event2"
        );
    }
    #[test]
    pub(super) fn test_hs_ext_version() {
        let v = HsExtVersion::new(1, 2, 3).with_pre("alpha");
        assert!(!v.is_stable());
        assert_eq!(format!("{}", v), "1.2.3-alpha");
        let stable = HsExtVersion::new(2, 0, 0);
        assert!(stable.is_stable());
        assert!(stable.is_compatible_with(&HsExtVersion::new(2, 0, 0)));
        assert!(!stable.is_compatible_with(&HsExtVersion::new(3, 0, 0)));
    }
    #[test]
    pub(super) fn test_hs_ext_features() {
        let mut f = HsExtFeatures::new();
        f.enable("sse2");
        f.enable("avx2");
        assert!(f.is_enabled("sse2"));
        assert!(!f.is_enabled("avx512"));
        f.disable("avx2");
        assert!(!f.is_enabled("avx2"));
        let mut g = HsExtFeatures::new();
        g.enable("sse2");
        g.enable("neon");
        let union = f.union(&g);
        assert!(union.is_enabled("sse2") && union.is_enabled("neon"));
        let inter = f.intersection(&g);
        assert!(inter.is_enabled("sse2"));
    }
    #[test]
    pub(super) fn test_hs_ext_emit_stats() {
        let mut s = HsExtEmitStats::new();
        s.bytes_emitted = 50_000;
        s.items_emitted = 500;
        s.elapsed_ms = 100;
        assert!(s.is_clean());
        assert!((s.throughput_bps() - 500_000.0).abs() < 1.0);
        let disp = format!("{}", s);
        assert!(disp.contains("bytes=50000"));
    }
}
#[cfg(test)]
mod Hsk_infra_tests {
    use super::*;
    #[test]
    pub(super) fn test_pass_config() {
        let config = HskPassConfig::new("test_pass", HskPassPhase::Transformation);
        assert!(config.enabled);
        assert!(config.phase.is_modifying());
        assert_eq!(config.phase.name(), "transformation");
    }
    #[test]
    pub(super) fn test_pass_stats() {
        let mut stats = HskPassStats::new();
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
        let mut reg = HskPassRegistry::new();
        reg.register(HskPassConfig::new("pass_a", HskPassPhase::Analysis));
        reg.register(HskPassConfig::new("pass_b", HskPassPhase::Transformation).disabled());
        assert_eq!(reg.total_passes(), 2);
        assert_eq!(reg.enabled_count(), 1);
        reg.update_stats("pass_a", 5, 50, 2);
        let stats = reg.get_stats("pass_a").expect("stats should exist");
        assert_eq!(stats.total_changes, 5);
    }
    #[test]
    pub(super) fn test_analysis_cache() {
        let mut cache = HskAnalysisCache::new(10);
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
        let mut wl = HskWorklist::new();
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
        let mut dt = HskDominatorTree::new(5);
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
        let mut liveness = HskLivenessInfo::new(3);
        liveness.add_def(0, 1);
        liveness.add_use(1, 1);
        assert!(liveness.defs[0].contains(&1));
        assert!(liveness.uses[1].contains(&1));
    }
    #[test]
    pub(super) fn test_constant_folding() {
        assert_eq!(HskConstantFoldingHelper::fold_add_i64(3, 4), Some(7));
        assert_eq!(HskConstantFoldingHelper::fold_div_i64(10, 0), None);
        assert_eq!(HskConstantFoldingHelper::fold_div_i64(10, 2), Some(5));
        assert_eq!(
            HskConstantFoldingHelper::fold_bitand_i64(0b1100, 0b1010),
            0b1000
        );
        assert_eq!(HskConstantFoldingHelper::fold_bitnot_i64(0), -1);
    }
    #[test]
    pub(super) fn test_dep_graph() {
        let mut g = HskDepGraph::new();
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
