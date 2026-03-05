//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;
use std::collections::{HashMap, HashSet};

use super::types::{
    GoAnalysisCache, GoBackend, GoCase, GoConstantFoldingHelper, GoDepGraph, GoDominatorTree,
    GoExpr, GoFunc, GoLit, GoLivenessInfo, GoModule, GoPassConfig, GoPassPhase, GoPassRegistry,
    GoPassStats, GoStmt, GoType, GoTypeDecl, GoWorklist,
};

pub(super) fn format_stmts(stmts: &[GoStmt], indent: usize) -> String {
    let prefix = "    ".repeat(indent);
    stmts
        .iter()
        .map(|s| format!("{}{}", prefix, format_stmt(s, indent)))
        .collect::<Vec<_>>()
        .join("\n")
}
pub(super) fn format_stmt(stmt: &GoStmt, indent: usize) -> String {
    let prefix = "    ".repeat(indent);
    match stmt {
        GoStmt::Const(name, ty, val) => {
            let ty_str = ty.as_ref().map(|t| format!(" {}", t)).unwrap_or_default();
            format!("const {}{} = {}", name, ty_str, val)
        }
        GoStmt::Var(name, ty, val) => {
            if let Some(v) = val {
                format!("var {} {} = {}", name, ty, v)
            } else {
                format!("var {} {}", name, ty)
            }
        }
        GoStmt::ShortDecl(name, val) => format!("{} := {}", name, val),
        GoStmt::Assign(target, val) => format!("{} = {}", target, val),
        GoStmt::Return(exprs) => {
            if exprs.is_empty() {
                "return".to_string()
            } else {
                let vals: Vec<String> = exprs.iter().map(|e| e.to_string()).collect();
                format!("return {}", vals.join(", "))
            }
        }
        GoStmt::If(cond, then_body, else_body) => {
            let mut out = format!("if {} {{\n", cond);
            let then_str = format_stmts(then_body, indent + 1);
            if !then_str.is_empty() {
                out.push_str(&then_str);
                out.push('\n');
            }
            out.push_str(&format!("{}}}", prefix));
            if !else_body.is_empty() {
                out.push_str(" else {\n");
                let else_str = format_stmts(else_body, indent + 1);
                if !else_str.is_empty() {
                    out.push_str(&else_str);
                    out.push('\n');
                }
                out.push_str(&format!("{}}}", prefix));
            }
            out
        }
        GoStmt::Switch(scrutinee, cases) => {
            let scr_str = scrutinee
                .as_ref()
                .map(|e| format!(" {}", e))
                .unwrap_or_default();
            let mut out = format!("switch{} {{\n", scr_str);
            for case in cases {
                match &case.pattern {
                    None => {
                        out.push_str(&format!("{}default:\n", prefix));
                    }
                    Some(pats) => {
                        let pat_strs: Vec<String> = pats.iter().map(|p| p.to_string()).collect();
                        out.push_str(&format!("{}case {}:\n", prefix, pat_strs.join(", ")));
                    }
                }
                let body_str = format_stmts(&case.body, indent + 1);
                if !body_str.is_empty() {
                    out.push_str(&body_str);
                    out.push('\n');
                }
            }
            out.push_str(&format!("{}}}", prefix));
            out
        }
        GoStmt::For(init, cond, post, body) => {
            let init_str = init
                .as_ref()
                .map(|s| format_stmt(s, indent))
                .unwrap_or_default();
            let cond_str = cond.as_ref().map(|e| e.to_string()).unwrap_or_default();
            let post_str = post
                .as_ref()
                .map(|s| format_stmt(s, indent))
                .unwrap_or_default();
            let mut out = format!("for {}; {}; {} {{\n", init_str, cond_str, post_str);
            let body_str = format_stmts(body, indent + 1);
            if !body_str.is_empty() {
                out.push_str(&body_str);
                out.push('\n');
            }
            out.push_str(&format!("{}}}", prefix));
            out
        }
        GoStmt::ForRange(key, val, iter, body) => {
            let vars = match (key, val) {
                (None, None) => "_".to_string(),
                (Some(k), None) => k.clone(),
                (None, Some(v)) => format!("_, {}", v),
                (Some(k), Some(v)) => format!("{}, {}", k, v),
            };
            let mut out = format!("for {} := range {} {{\n", vars, iter);
            let body_str = format_stmts(body, indent + 1);
            if !body_str.is_empty() {
                out.push_str(&body_str);
                out.push('\n');
            }
            out.push_str(&format!("{}}}", prefix));
            out
        }
        GoStmt::Block(stmts) => {
            let mut out = "{\n".to_string();
            let body_str = format_stmts(stmts, indent + 1);
            if !body_str.is_empty() {
                out.push_str(&body_str);
                out.push('\n');
            }
            out.push_str(&format!("{}}}", prefix));
            out
        }
        GoStmt::Expr(expr) => expr.to_string(),
        GoStmt::Break => "break".to_string(),
        GoStmt::Continue => "continue".to_string(),
        GoStmt::Goto(label) => format!("goto {}", label),
        GoStmt::Label(label, inner) => {
            format!("{}:\n{}{}", label, prefix, format_stmt(inner, indent))
        }
        GoStmt::Defer(expr) => format!("defer {}", expr),
        GoStmt::GoRoutine(expr) => format!("go {}", expr),
        GoStmt::Panic(expr) => format!("panic({})", expr),
    }
}
/// Returns the set of all Go reserved keywords.
pub(super) fn go_keywords() -> HashSet<&'static str> {
    [
        "break",
        "case",
        "chan",
        "const",
        "continue",
        "default",
        "defer",
        "else",
        "fallthrough",
        "for",
        "func",
        "go",
        "goto",
        "if",
        "import",
        "interface",
        "map",
        "package",
        "range",
        "return",
        "select",
        "struct",
        "switch",
        "type",
        "var",
    ]
    .iter()
    .copied()
    .collect()
}
/// Returns the set of Go pre-declared identifiers (built-ins).
pub(super) fn go_builtins() -> HashSet<&'static str> {
    [
        "any",
        "append",
        "bool",
        "byte",
        "cap",
        "clear",
        "close",
        "comparable",
        "complex",
        "complex128",
        "complex64",
        "copy",
        "delete",
        "error",
        "false",
        "float32",
        "float64",
        "imag",
        "int",
        "int16",
        "int32",
        "int64",
        "int8",
        "iota",
        "len",
        "make",
        "max",
        "min",
        "new",
        "nil",
        "panic",
        "print",
        "println",
        "real",
        "recover",
        "rune",
        "string",
        "true",
        "uint",
        "uint16",
        "uint32",
        "uint64",
        "uint8",
        "uintptr",
    ]
    .iter()
    .copied()
    .collect()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub(super) fn test_go_type_primitives() {
        assert_eq!(GoType::GoBool.to_string(), "bool");
        assert_eq!(GoType::GoInt.to_string(), "int64");
        assert_eq!(GoType::GoFloat.to_string(), "float64");
        assert_eq!(GoType::GoString.to_string(), "string");
        assert_eq!(GoType::GoInterface.to_string(), "interface{}");
        assert_eq!(GoType::GoUnit.to_string(), "struct{}");
        assert_eq!(GoType::GoError.to_string(), "error");
    }
    #[test]
    pub(super) fn test_go_type_slice() {
        let ty = GoType::GoSlice(Box::new(GoType::GoInt));
        assert_eq!(ty.to_string(), "[]int64");
    }
    #[test]
    pub(super) fn test_go_type_map() {
        let ty = GoType::GoMap(Box::new(GoType::GoString), Box::new(GoType::GoInt));
        assert_eq!(ty.to_string(), "map[string]int64");
    }
    #[test]
    pub(super) fn test_go_type_func_no_params_no_ret() {
        let ty = GoType::GoFunc(vec![], vec![]);
        assert_eq!(ty.to_string(), "func()");
    }
    #[test]
    pub(super) fn test_go_type_func_with_params_single_ret() {
        let ty = GoType::GoFunc(vec![GoType::GoInt, GoType::GoBool], vec![GoType::GoString]);
        assert_eq!(ty.to_string(), "func(int64, bool) string");
    }
    #[test]
    pub(super) fn test_go_type_func_multi_ret() {
        let ty = GoType::GoFunc(vec![GoType::GoInt], vec![GoType::GoInt, GoType::GoError]);
        assert_eq!(ty.to_string(), "func(int64) (int64, error)");
    }
    #[test]
    pub(super) fn test_go_type_ptr() {
        let ty = GoType::GoPtr(Box::new(GoType::GoStruct("Foo".to_string())));
        assert_eq!(ty.to_string(), "*Foo");
    }
    #[test]
    pub(super) fn test_go_type_chan() {
        let ty = GoType::GoChan(Box::new(GoType::GoInt));
        assert_eq!(ty.to_string(), "chan int64");
    }
    #[test]
    pub(super) fn test_go_lit_int() {
        assert_eq!(GoLit::Int(42).to_string(), "42");
        assert_eq!(GoLit::Int(-7).to_string(), "-7");
    }
    #[test]
    pub(super) fn test_go_lit_bool() {
        assert_eq!(GoLit::Bool(true).to_string(), "true");
        assert_eq!(GoLit::Bool(false).to_string(), "false");
    }
    #[test]
    pub(super) fn test_go_lit_str_plain() {
        assert_eq!(GoLit::Str("hello".to_string()).to_string(), "\"hello\"");
    }
    #[test]
    pub(super) fn test_go_lit_str_escaping() {
        let s = GoLit::Str("a\"b\\c\nd".to_string()).to_string();
        assert!(s.contains("\\\""));
        assert!(s.contains("\\\\"));
        assert!(s.contains("\\n"));
    }
    #[test]
    pub(super) fn test_go_lit_nil() {
        assert_eq!(GoLit::Nil.to_string(), "nil");
    }
    #[test]
    pub(super) fn test_go_lit_float_whole() {
        let s = GoLit::Float(3.0).to_string();
        assert!(s.contains("3.0") || s.contains("3."), "got: {}", s);
    }
    #[test]
    pub(super) fn test_go_expr_var() {
        let e = GoExpr::Var("myVar".to_string());
        assert_eq!(e.to_string(), "myVar");
    }
    #[test]
    pub(super) fn test_go_expr_lit() {
        let e = GoExpr::Lit(GoLit::Int(99));
        assert_eq!(e.to_string(), "99");
    }
    #[test]
    pub(super) fn test_go_expr_call_no_args() {
        let e = GoExpr::Call(Box::new(GoExpr::Var("init".to_string())), vec![]);
        assert_eq!(e.to_string(), "init()");
    }
    #[test]
    pub(super) fn test_go_expr_call_with_args() {
        let e = GoExpr::Call(
            Box::new(GoExpr::Var("natAdd".to_string())),
            vec![GoExpr::Lit(GoLit::Int(1)), GoExpr::Lit(GoLit::Int(2))],
        );
        assert_eq!(e.to_string(), "natAdd(1, 2)");
    }
    #[test]
    pub(super) fn test_go_expr_binop() {
        let e = GoExpr::BinOp(
            "+".to_string(),
            Box::new(GoExpr::Var("a".to_string())),
            Box::new(GoExpr::Var("b".to_string())),
        );
        assert_eq!(e.to_string(), "(a + b)");
    }
    #[test]
    pub(super) fn test_go_expr_unary() {
        let e = GoExpr::Unary("!".to_string(), Box::new(GoExpr::Var("x".to_string())));
        assert_eq!(e.to_string(), "(!x)");
    }
    #[test]
    pub(super) fn test_go_expr_field() {
        let e = GoExpr::Field(Box::new(GoExpr::Var("obj".to_string())), "Tag".to_string());
        assert_eq!(e.to_string(), "obj.Tag");
    }
    #[test]
    pub(super) fn test_go_expr_index() {
        let e = GoExpr::Index(
            Box::new(GoExpr::Var("arr".to_string())),
            Box::new(GoExpr::Lit(GoLit::Int(0))),
        );
        assert_eq!(e.to_string(), "arr[0]");
    }
    #[test]
    pub(super) fn test_go_expr_new() {
        let e = GoExpr::New(GoType::GoStruct("OxiCtor".to_string()));
        assert_eq!(e.to_string(), "new(OxiCtor)");
    }
    #[test]
    pub(super) fn test_go_expr_make_slice() {
        let e = GoExpr::Make(
            GoType::GoSlice(Box::new(GoType::GoInterface)),
            vec![GoExpr::Lit(GoLit::Int(0))],
        );
        assert_eq!(e.to_string(), "make([]interface{}, 0)");
    }
    #[test]
    pub(super) fn test_go_expr_address_of() {
        let e = GoExpr::AddressOf(Box::new(GoExpr::Var("x".to_string())));
        assert_eq!(e.to_string(), "&x");
    }
    #[test]
    pub(super) fn test_go_expr_deref() {
        let e = GoExpr::Deref(Box::new(GoExpr::Var("p".to_string())));
        assert_eq!(e.to_string(), "*p");
    }
    #[test]
    pub(super) fn test_go_expr_composite_empty() {
        let e = GoExpr::Composite(GoType::GoStruct("Foo".to_string()), vec![]);
        assert_eq!(e.to_string(), "Foo{}");
    }
    #[test]
    pub(super) fn test_go_expr_composite_fields() {
        let e = GoExpr::Composite(
            GoType::GoStruct("OxiCtor".to_string()),
            vec![
                ("Tag".to_string(), GoExpr::Lit(GoLit::Int(0))),
                (
                    "Fields".to_string(),
                    GoExpr::SliceLit(GoType::GoInterface, vec![]),
                ),
            ],
        );
        let s = e.to_string();
        assert!(s.contains("OxiCtor{"));
        assert!(s.contains("Tag: 0"));
    }
    #[test]
    pub(super) fn test_go_expr_slice_lit_empty() {
        let e = GoExpr::SliceLit(GoType::GoInt, vec![]);
        assert_eq!(e.to_string(), "[]int64{}");
    }
    #[test]
    pub(super) fn test_go_expr_slice_lit_elements() {
        let e = GoExpr::SliceLit(
            GoType::GoInt,
            vec![GoExpr::Lit(GoLit::Int(1)), GoExpr::Lit(GoLit::Int(2))],
        );
        let s = e.to_string();
        assert!(s.starts_with("[]int64{"));
        assert!(s.contains("1"));
        assert!(s.contains("2"));
    }
    #[test]
    pub(super) fn test_go_stmt_const() {
        let s = GoStmt::Const(
            "MaxN".to_string(),
            Some(GoType::GoInt),
            GoExpr::Lit(GoLit::Int(1024)),
        );
        assert_eq!(s.to_string(), "const MaxN int64 = 1024");
    }
    #[test]
    pub(super) fn test_go_stmt_var_no_init() {
        let s = GoStmt::Var("x".to_string(), GoType::GoInt, None);
        assert_eq!(s.to_string(), "var x int64");
    }
    #[test]
    pub(super) fn test_go_stmt_var_with_init() {
        let s = GoStmt::Var(
            "x".to_string(),
            GoType::GoInt,
            Some(GoExpr::Lit(GoLit::Int(0))),
        );
        assert_eq!(s.to_string(), "var x int64 = 0");
    }
    #[test]
    pub(super) fn test_go_stmt_short_decl() {
        let s = GoStmt::ShortDecl("x".to_string(), GoExpr::Lit(GoLit::Int(42)));
        assert_eq!(s.to_string(), "x := 42");
    }
    #[test]
    pub(super) fn test_go_stmt_assign() {
        let s = GoStmt::Assign(GoExpr::Var("x".to_string()), GoExpr::Lit(GoLit::Int(7)));
        assert_eq!(s.to_string(), "x = 7");
    }
    #[test]
    pub(super) fn test_go_stmt_return_empty() {
        assert_eq!(GoStmt::Return(vec![]).to_string(), "return");
    }
    #[test]
    pub(super) fn test_go_stmt_return_value() {
        let s = GoStmt::Return(vec![GoExpr::Lit(GoLit::Int(42))]);
        assert_eq!(s.to_string(), "return 42");
    }
    #[test]
    pub(super) fn test_go_stmt_break_continue() {
        assert_eq!(GoStmt::Break.to_string(), "break");
        assert_eq!(GoStmt::Continue.to_string(), "continue");
    }
    #[test]
    pub(super) fn test_go_stmt_goto() {
        assert_eq!(GoStmt::Goto("end".to_string()).to_string(), "goto end");
    }
    #[test]
    pub(super) fn test_go_stmt_defer() {
        let s = GoStmt::Defer(GoExpr::Call(
            Box::new(GoExpr::Var("cleanup".to_string())),
            vec![],
        ));
        assert_eq!(s.to_string(), "defer cleanup()");
    }
    #[test]
    pub(super) fn test_go_stmt_goroutine() {
        let s = GoStmt::GoRoutine(GoExpr::Call(
            Box::new(GoExpr::Var("worker".to_string())),
            vec![],
        ));
        assert_eq!(s.to_string(), "go worker()");
    }
    #[test]
    pub(super) fn test_go_stmt_panic() {
        let s = GoStmt::Panic(GoExpr::Lit(GoLit::Str("oh no".to_string())));
        assert_eq!(s.to_string(), "panic(\"oh no\")");
    }
    #[test]
    pub(super) fn test_go_stmt_if_no_else() {
        let s = GoStmt::If(
            GoExpr::Lit(GoLit::Bool(true)),
            vec![GoStmt::Return(vec![GoExpr::Lit(GoLit::Int(1))])],
            vec![],
        );
        let out = s.to_string();
        assert!(out.contains("if true {"));
        assert!(out.contains("return 1"));
        assert!(!out.contains("else"));
    }
    #[test]
    pub(super) fn test_go_stmt_if_with_else() {
        let s = GoStmt::If(
            GoExpr::Var("cond".to_string()),
            vec![GoStmt::Return(vec![GoExpr::Lit(GoLit::Int(1))])],
            vec![GoStmt::Return(vec![GoExpr::Lit(GoLit::Int(0))])],
        );
        let out = s.to_string();
        assert!(out.contains("else"));
        assert!(out.contains("return 0"));
    }
    #[test]
    pub(super) fn test_go_stmt_switch() {
        let s = GoStmt::Switch(
            Some(GoExpr::Var("tag".to_string())),
            vec![
                GoCase {
                    pattern: Some(vec![GoExpr::Lit(GoLit::Int(0))]),
                    body: vec![GoStmt::Return(vec![GoExpr::Lit(GoLit::Str(
                        "zero".to_string(),
                    ))])],
                },
                GoCase {
                    pattern: None,
                    body: vec![GoStmt::Panic(GoExpr::Lit(GoLit::Str("bad".to_string())))],
                },
            ],
        );
        let out = s.to_string();
        assert!(out.contains("switch tag {"));
        assert!(out.contains("case 0:"));
        assert!(out.contains("default:"));
    }
    #[test]
    pub(super) fn test_go_stmt_for_range() {
        let s = GoStmt::ForRange(
            Some("i".to_string()),
            Some("v".to_string()),
            GoExpr::Var("items".to_string()),
            vec![GoStmt::Expr(GoExpr::Var("v".to_string()))],
        );
        let out = s.to_string();
        assert!(out.contains("for i, v := range items {"));
    }
    #[test]
    pub(super) fn test_go_func_simple() {
        let mut f = GoFunc::new("add");
        f.add_param("a", GoType::GoInt);
        f.add_param("b", GoType::GoInt);
        f.add_return(GoType::GoInt);
        f.body = vec![GoStmt::Return(vec![GoExpr::BinOp(
            "+".to_string(),
            Box::new(GoExpr::Var("a".to_string())),
            Box::new(GoExpr::Var("b".to_string())),
        )])];
        let out = f.codegen();
        assert!(out.starts_with("func add("));
        assert!(out.contains("a int64, b int64"));
        assert!(out.contains("int64"));
        assert!(out.contains("return"));
    }
    #[test]
    pub(super) fn test_go_func_no_params_no_return() {
        let f = GoFunc::new("noop");
        let out = f.codegen();
        assert!(out.starts_with("func noop()"));
        assert!(out.contains('{'));
        assert!(out.contains('}'));
    }
    #[test]
    pub(super) fn test_go_type_decl_empty() {
        let d = GoTypeDecl::new("Empty");
        let out = d.codegen();
        assert!(out.contains("type Empty struct {"));
    }
    #[test]
    pub(super) fn test_go_type_decl_with_fields() {
        let mut d = GoTypeDecl::new("OxiCtor");
        d.add_field("Tag", GoType::GoInt);
        d.add_field("Fields", GoType::GoSlice(Box::new(GoType::GoInterface)));
        let out = d.codegen();
        assert!(out.contains("Tag int64"));
        assert!(out.contains("Fields []interface{}"));
    }
    #[test]
    pub(super) fn test_go_module_package() {
        let m = GoModule::new("main");
        let out = m.codegen();
        assert!(out.starts_with("package main"));
    }
    #[test]
    pub(super) fn test_go_module_single_import() {
        let mut m = GoModule::new("main");
        m.add_import("fmt");
        let out = m.codegen();
        assert!(out.contains("import \"fmt\""));
    }
    #[test]
    pub(super) fn test_go_module_multi_import() {
        let mut m = GoModule::new("main");
        m.add_import("fmt");
        m.add_import("os");
        let out = m.codegen();
        assert!(out.contains("import ("));
        assert!(out.contains("\"fmt\""));
        assert!(out.contains("\"os\""));
    }
    #[test]
    pub(super) fn test_go_module_dedup_imports() {
        let mut m = GoModule::new("main");
        m.add_import("fmt");
        m.add_import("fmt");
        assert_eq!(m.imports.len(), 1);
    }
    #[test]
    pub(super) fn test_go_module_consts() {
        let mut m = GoModule::new("main");
        m.consts.push((
            "Limit".to_string(),
            GoType::GoInt,
            GoExpr::Lit(GoLit::Int(100)),
        ));
        let out = m.codegen();
        assert!(out.contains("Limit int64 = 100"));
    }
    #[test]
    pub(super) fn test_mangle_name_normal() {
        assert_eq!(GoBackend::mangle_name("myFunc"), "myFunc");
    }
    #[test]
    pub(super) fn test_mangle_name_empty() {
        assert_eq!(GoBackend::mangle_name(""), "ox_empty");
    }
    #[test]
    pub(super) fn test_mangle_name_digit_prefix() {
        assert_eq!(GoBackend::mangle_name("0abc"), "ox_0abc");
        assert_eq!(GoBackend::mangle_name("9"), "ox_9");
    }
    #[test]
    pub(super) fn test_mangle_name_go_keywords() {
        for kw in &[
            "break",
            "case",
            "chan",
            "const",
            "continue",
            "default",
            "defer",
            "else",
            "fallthrough",
            "for",
            "func",
            "go",
            "goto",
            "if",
            "import",
            "interface",
            "map",
            "package",
            "range",
            "return",
            "select",
            "struct",
            "switch",
            "type",
            "var",
        ] {
            let result = GoBackend::mangle_name(kw);
            assert!(
                result.starts_with("ox_"),
                "keyword '{}' not prefixed, got '{}'",
                kw,
                result
            );
        }
    }
    #[test]
    pub(super) fn test_mangle_name_go_builtins() {
        for bi in &[
            "nil", "true", "false", "len", "cap", "make", "new", "panic", "append",
        ] {
            let result = GoBackend::mangle_name(bi);
            assert!(
                result.starts_with("ox_"),
                "builtin '{}' not prefixed, got '{}'",
                bi,
                result
            );
        }
    }
    #[test]
    pub(super) fn test_mangle_name_special_chars() {
        assert_eq!(GoBackend::mangle_name("foo-bar"), "foo_bar");
        assert_eq!(GoBackend::mangle_name("a/b"), "a_b");
        assert_eq!(GoBackend::mangle_name("x.y"), "x_y");
    }
    #[test]
    pub(super) fn test_mangle_name_underscore_prefix() {
        assert_eq!(GoBackend::mangle_name("_private"), "_private");
    }
    #[test]
    pub(super) fn test_compile_type_nat() {
        let b = GoBackend::new();
        assert_eq!(b.compile_type(&LcnfType::Nat), GoType::GoInt);
    }
    #[test]
    pub(super) fn test_compile_type_string() {
        let b = GoBackend::new();
        assert_eq!(b.compile_type(&LcnfType::LcnfString), GoType::GoString);
    }
    #[test]
    pub(super) fn test_compile_type_erased() {
        let b = GoBackend::new();
        assert_eq!(b.compile_type(&LcnfType::Erased), GoType::GoUnit);
    }
    #[test]
    pub(super) fn test_compile_type_object() {
        let b = GoBackend::new();
        assert_eq!(b.compile_type(&LcnfType::Object), GoType::GoInterface);
    }
    #[test]
    pub(super) fn test_runtime_nat_add() {
        let b = GoBackend::new();
        let funcs = b.build_runtime();
        let nat_add = funcs.iter().find(|f| f.name == "natAdd");
        assert!(nat_add.is_some(), "natAdd not found in runtime");
        let out = nat_add.expect("out should be Some/Ok").codegen();
        assert!(out.contains("a int64"));
        assert!(out.contains("b int64"));
        assert!(out.contains("return"));
        assert!(out.contains('+'));
    }
    #[test]
    pub(super) fn test_runtime_nat_sub_saturating() {
        let b = GoBackend::new();
        let funcs = b.build_runtime();
        let f = funcs
            .iter()
            .find(|f| f.name == "natSub")
            .expect("f should be found");
        let out = f.codegen();
        assert!(
            out.contains(">=") || out.contains("if"),
            "natSub should have guard: {}",
            out
        );
    }
    #[test]
    pub(super) fn test_runtime_nat_div_guard() {
        let b = GoBackend::new();
        let funcs = b.build_runtime();
        let f = funcs
            .iter()
            .find(|f| f.name == "natDiv")
            .expect("f should be found");
        let out = f.codegen();
        assert!(out.contains("0"), "natDiv should return 0 on b==0");
    }
    #[test]
    pub(super) fn test_runtime_str_append() {
        let b = GoBackend::new();
        let funcs = b.build_runtime();
        let f = funcs
            .iter()
            .find(|f| f.name == "strAppend")
            .expect("f should be found");
        let out = f.codegen();
        assert!(out.contains("string"));
        assert!(out.contains('+'));
    }
    #[test]
    pub(super) fn test_runtime_oxi_print() {
        let b = GoBackend::new();
        let funcs = b.build_runtime();
        let f = funcs
            .iter()
            .find(|f| f.name == "oxiPrint")
            .expect("f should be found");
        let out = f.codegen();
        assert!(out.contains("fmt"));
        assert!(out.contains("Println"));
    }
    #[test]
    pub(super) fn test_runtime_bool_not() {
        let b = GoBackend::new();
        let funcs = b.build_runtime();
        let f = funcs
            .iter()
            .find(|f| f.name == "boolNot")
            .expect("f should be found");
        let out = f.codegen();
        assert!(out.contains('!'));
        assert!(out.contains("bool"));
    }
    #[test]
    pub(super) fn test_emit_func_passthrough() {
        let b = GoBackend::new();
        let f = GoFunc::new("hello");
        let direct = f.codegen();
        let via_emit = b.emit_func(&f);
        assert_eq!(direct, via_emit);
    }
    #[test]
    pub(super) fn test_emit_type_decl_passthrough() {
        let b = GoBackend::new();
        let d = GoTypeDecl::new("Point");
        let direct = d.codegen();
        let via_emit = b.emit_type_decl(&d);
        assert_eq!(direct, via_emit);
    }
    #[test]
    pub(super) fn test_emit_module_passthrough() {
        let b = GoBackend::new();
        let m = GoModule::new("pkg");
        let direct = m.codegen();
        let via_emit = b.emit_module(&m);
        assert_eq!(direct, via_emit);
    }
    #[test]
    pub(super) fn test_compile_module_empty() {
        let mut b = GoBackend::new();
        let module = b.compile_module(&[]);
        let out = module.codegen();
        assert!(out.starts_with("package main"), "got: {}", out);
        assert!(out.contains("natAdd"));
        assert!(out.contains("natSub"));
        assert!(out.contains("OxiCtor"));
    }
    #[test]
    pub(super) fn test_go_backend_default() {
        let b = GoBackend::default();
        let funcs = b.build_runtime();
        assert!(!funcs.is_empty());
    }
}
#[cfg(test)]
mod Go_infra_tests {
    use super::*;
    #[test]
    pub(super) fn test_pass_config() {
        let config = GoPassConfig::new("test_pass", GoPassPhase::Transformation);
        assert!(config.enabled);
        assert!(config.phase.is_modifying());
        assert_eq!(config.phase.name(), "transformation");
    }
    #[test]
    pub(super) fn test_pass_stats() {
        let mut stats = GoPassStats::new();
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
        let mut reg = GoPassRegistry::new();
        reg.register(GoPassConfig::new("pass_a", GoPassPhase::Analysis));
        reg.register(GoPassConfig::new("pass_b", GoPassPhase::Transformation).disabled());
        assert_eq!(reg.total_passes(), 2);
        assert_eq!(reg.enabled_count(), 1);
        reg.update_stats("pass_a", 5, 50, 2);
        let stats = reg.get_stats("pass_a").expect("stats should exist");
        assert_eq!(stats.total_changes, 5);
    }
    #[test]
    pub(super) fn test_analysis_cache() {
        let mut cache = GoAnalysisCache::new(10);
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
        let mut wl = GoWorklist::new();
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
        let mut dt = GoDominatorTree::new(5);
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
        let mut liveness = GoLivenessInfo::new(3);
        liveness.add_def(0, 1);
        liveness.add_use(1, 1);
        assert!(liveness.defs[0].contains(&1));
        assert!(liveness.uses[1].contains(&1));
    }
    #[test]
    pub(super) fn test_constant_folding() {
        assert_eq!(GoConstantFoldingHelper::fold_add_i64(3, 4), Some(7));
        assert_eq!(GoConstantFoldingHelper::fold_div_i64(10, 0), None);
        assert_eq!(GoConstantFoldingHelper::fold_div_i64(10, 2), Some(5));
        assert_eq!(
            GoConstantFoldingHelper::fold_bitand_i64(0b1100, 0b1010),
            0b1000
        );
        assert_eq!(GoConstantFoldingHelper::fold_bitnot_i64(0), -1);
    }
    #[test]
    pub(super) fn test_dep_graph() {
        let mut g = GoDepGraph::new();
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
