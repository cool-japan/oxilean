//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;
use std::collections::HashMap;

use super::types::{
    JSAnalysisCache, JSConstantFoldingHelper, JSDepGraph, JSDominatorTree, JSLivenessInfo,
    JSPassConfig, JSPassPhase, JSPassRegistry, JSPassStats, JSWorklist, JsBackend, JsBackendConfig,
    JsEmitContext, JsExpr, JsFunction, JsIdentTable, JsLit, JsMinifier, JsModule, JsModuleFormat,
    JsModuleLinker, JsNameMangler, JsPeephole, JsPrettyPrinter, JsSizeEstimator, JsSourceMap,
    JsStmt, JsType, JsTypeChecker, SourceMapEntry,
};

/// Format a list of statements with the given indentation level.
pub fn display_indented(stmts: &[JsStmt], indent: usize) -> std::string::String {
    let pad = " ".repeat(indent);
    let mut out = std::string::String::new();
    for stmt in stmts {
        let text = format_stmt_indented(stmt, indent);
        for line in text.lines() {
            out.push_str(&pad);
            out.push_str(line);
            out.push('\n');
        }
    }
    if out.ends_with('\n') {
        out.pop();
    }
    out
}
/// Format a single statement with the given base indentation.
pub(super) fn format_stmt_indented(stmt: &JsStmt, indent: usize) -> std::string::String {
    let pad = " ".repeat(indent);
    let inner_pad = " ".repeat(indent + 2);
    match stmt {
        JsStmt::Expr(e) => format!("{};", e),
        JsStmt::Let(name, expr) => format!("let {} = {};", name, expr),
        JsStmt::Const(name, expr) => format!("const {} = {};", name, expr),
        JsStmt::Return(e) => format!("return {};", e),
        JsStmt::ReturnVoid => "return;".to_string(),
        JsStmt::If(cond, then_stmts, else_stmts) => {
            let then_body = display_indented_with_pad(then_stmts, indent + 2, &inner_pad);
            let mut s = format!("if ({}) {{\n{}\n{}}}", cond, then_body, pad);
            if !else_stmts.is_empty() {
                let else_body = display_indented_with_pad(else_stmts, indent + 2, &inner_pad);
                s.push_str(&format!(" else {{\n{}\n{}}}", else_body, pad));
            }
            s
        }
        JsStmt::While(cond, body) => {
            let body_text = display_indented_with_pad(body, indent + 2, &inner_pad);
            format!("while ({}) {{\n{}\n{}}}", cond, body_text, pad)
        }
        JsStmt::For(var, iter, body) => {
            let body_text = display_indented_with_pad(body, indent + 2, &inner_pad);
            format!(
                "for (const {} of {}) {{\n{}\n{}}}",
                var, iter, body_text, pad
            )
        }
        JsStmt::Block(stmts) => {
            let body = display_indented_with_pad(stmts, indent + 2, &inner_pad);
            format!("{{\n{}\n{}}}", body, pad)
        }
        JsStmt::Throw(e) => format!("throw {};", e),
        JsStmt::TryCatch(try_body, catch_var, catch_body) => {
            let try_text = display_indented_with_pad(try_body, indent + 2, &inner_pad);
            let catch_text = display_indented_with_pad(catch_body, indent + 2, &inner_pad);
            format!(
                "try {{\n{}\n{}}} catch ({}) {{\n{}\n{}}}",
                try_text, pad, catch_var, catch_text, pad
            )
        }
        JsStmt::Switch(expr, cases, default) => {
            let mut s = format!("switch ({}) {{\n", expr);
            for (case_expr, case_stmts) in cases {
                s.push_str(&format!("{}  case {}:\n", pad, case_expr));
                for cs in case_stmts {
                    s.push_str(&format!(
                        "{}    {}\n",
                        pad,
                        format_stmt_indented(cs, indent + 4)
                    ));
                }
                s.push_str(&format!("{}    break;\n", pad));
            }
            if !default.is_empty() {
                s.push_str(&format!("{}  default:\n", pad));
                for ds in default {
                    s.push_str(&format!(
                        "{}    {}\n",
                        pad,
                        format_stmt_indented(ds, indent + 4)
                    ));
                }
            }
            s.push_str(&format!("{}}}", pad));
            s
        }
    }
}
/// Helper: format statements with explicit inner padding.
pub(super) fn display_indented_with_pad(
    stmts: &[JsStmt],
    _indent: usize,
    pad: &str,
) -> std::string::String {
    let mut out = std::string::String::new();
    for stmt in stmts {
        let text = format_stmt_indented(stmt, _indent);
        for line in text.lines() {
            out.push_str(pad);
            out.push_str(line);
            out.push('\n');
        }
    }
    if out.ends_with('\n') {
        out.pop();
    }
    out
}
/// The minimal OxiLean JavaScript runtime, prepended to every module.
pub const JS_RUNTIME: &str = r#"// OxiLean JS Runtime
const _OL = {
  natAdd: (a, b) => a + b,
  natMul: (a, b) => a * b,
  natSub: (a, b) => a >= b ? a - b : 0n,
  natDiv: (a, b) => b === 0n ? 0n : a / b,
  natMod: (a, b) => b === 0n ? 0n : a % b,
  natLt: (a, b) => a < b,
  natLe: (a, b) => a <= b,
  natEq: (a, b) => a === b,
  strAppend: (a, b) => a + b,
  strLength: (s) => BigInt(s.length),
  ctor: (tag, ...fields) => ({ tag, fields }),
  proj: (obj, i) => obj.fields[i],
  panic: (msg) => { throw new Error(msg); },
};"#;
/// JavaScript reserved words that must not be used as identifiers.
pub const JS_KEYWORDS: &[&str] = &[
    "break",
    "case",
    "catch",
    "class",
    "const",
    "continue",
    "debugger",
    "default",
    "delete",
    "do",
    "else",
    "export",
    "extends",
    "false",
    "finally",
    "for",
    "function",
    "if",
    "import",
    "in",
    "instanceof",
    "let",
    "new",
    "null",
    "return",
    "static",
    "super",
    "switch",
    "this",
    "throw",
    "true",
    "try",
    "typeof",
    "undefined",
    "var",
    "void",
    "while",
    "with",
    "yield",
    "await",
    "async",
    "of",
    "from",
    "get",
    "set",
    "target",
    "meta",
];
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub(super) fn test_js_expr_display() {
        let expr = JsExpr::BinOp(
            "+".to_string(),
            Box::new(JsExpr::Lit(JsLit::Num(1.0))),
            Box::new(JsExpr::Lit(JsLit::Num(2.0))),
        );
        assert_eq!(expr.to_string(), "1 + 2");
    }
    #[test]
    pub(super) fn test_js_lit_bigint_display() {
        let lit = JsLit::BigInt(42);
        assert_eq!(lit.to_string(), "42n");
        let lit_zero = JsLit::BigInt(0);
        assert_eq!(lit_zero.to_string(), "0n");
    }
    #[test]
    pub(super) fn test_js_lit_str_escape() {
        let lit = JsLit::Str("hello \"world\"\nnewline".to_string());
        assert_eq!(lit.to_string(), "\"hello \\\"world\\\"\\nnewline\"");
    }
    #[test]
    pub(super) fn test_js_function_display() {
        let func = JsFunction {
            name: "add".to_string(),
            params: vec!["a".to_string(), "b".to_string()],
            body: vec![JsStmt::Return(JsExpr::BinOp(
                "+".to_string(),
                Box::new(JsExpr::Var("a".to_string())),
                Box::new(JsExpr::Var("b".to_string())),
            ))],
            is_async: false,
            is_export: false,
        };
        let s = func.to_string();
        assert!(s.contains("function add(a, b)"));
        assert!(s.contains("return a + b;"));
    }
    #[test]
    pub(super) fn test_js_async_export_function_display() {
        let func = JsFunction {
            name: "fetchData".to_string(),
            params: vec!["url".to_string()],
            body: vec![JsStmt::ReturnVoid],
            is_async: true,
            is_export: true,
        };
        let s = func.to_string();
        assert!(s.starts_with("export async function fetchData(url)"));
    }
    #[test]
    pub(super) fn test_mangle_name() {
        let backend = JsBackend::new();
        assert_eq!(backend.mangle_name("Nat.add"), "Nat_add");
        assert_eq!(backend.mangle_name("List.cons"), "List_cons");
        assert_eq!(backend.mangle_name("return"), "_return");
        assert_eq!(backend.mangle_name("class"), "_class");
        assert_eq!(backend.mangle_name("foo'"), "foo_");
        assert_eq!(backend.mangle_name(""), "_anon");
    }
    #[test]
    pub(super) fn test_compile_simple_decl() {
        let decl = LcnfFunDecl {
            name: "answer".to_string(),
            original_name: None,
            params: vec![],
            ret_type: LcnfType::Nat,
            body: LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Nat(42))),
            is_recursive: false,
            is_lifted: false,
            inline_cost: 0,
        };
        let mut backend = JsBackend::new();
        let func = backend.compile_decl(&decl).expect("compile_decl failed");
        assert_eq!(func.name, "answer");
        assert!(func.params.is_empty());
        let s = func.to_string();
        assert!(s.contains("42n"), "Expected BigInt literal 42n, got: {}", s);
    }
    #[test]
    pub(super) fn test_compile_let() {
        let x_id = LcnfVarId(0);
        let y_id = LcnfVarId(1);
        let decl = LcnfFunDecl {
            name: "double".to_string(),
            original_name: None,
            params: vec![LcnfParam {
                id: x_id,
                name: "x".to_string(),
                ty: LcnfType::Nat,
                erased: false,
                borrowed: false,
            }],
            ret_type: LcnfType::Nat,
            body: LcnfExpr::Let {
                id: y_id,
                name: "y".to_string(),
                ty: LcnfType::Nat,
                value: LcnfLetValue::App(LcnfArg::Var(x_id), vec![LcnfArg::Var(x_id)]),
                body: Box::new(LcnfExpr::Return(LcnfArg::Var(y_id))),
            },
            is_recursive: false,
            is_lifted: false,
            inline_cost: 0,
        };
        let mut backend = JsBackend::new();
        let func = backend.compile_decl(&decl).expect("compile_decl failed");
        let s = func.to_string();
        assert!(
            s.contains("function double"),
            "Expected function double, got: {}",
            s
        );
        assert!(
            s.contains("const y"),
            "Expected const y binding, got: {}",
            s
        );
    }
    #[test]
    pub(super) fn test_emit_module() {
        let decl = LcnfFunDecl {
            name: "main".to_string(),
            original_name: None,
            params: vec![],
            ret_type: LcnfType::Unit,
            body: LcnfExpr::Return(LcnfArg::Erased),
            is_recursive: false,
            is_lifted: false,
            inline_cost: 0,
        };
        let js = JsBackend::compile_module(&[decl]).expect("compile_module failed");
        assert!(js.contains("const _OL ="), "Missing runtime preamble");
        assert!(js.contains("natAdd"), "Missing natAdd in runtime");
        assert!(js.contains("function main()"), "Missing main function");
        assert!(js.contains("export {"), "Missing export statement");
        assert!(js.contains("main"), "Missing 'main' in exports");
    }
    #[test]
    pub(super) fn test_js_object_expr_display() {
        let expr = JsExpr::Object(vec![
            (
                "tag".to_string(),
                JsExpr::Lit(JsLit::Str("Some".to_string())),
            ),
            (
                "fields".to_string(),
                JsExpr::Array(vec![JsExpr::Lit(JsLit::BigInt(1))]),
            ),
        ]);
        let s = expr.to_string();
        assert!(s.contains("tag: \"Some\""));
        assert!(s.contains("fields: [1n]"));
    }
    #[test]
    pub(super) fn test_js_ternary_display() {
        let expr = JsExpr::Ternary(
            Box::new(JsExpr::Var("cond".to_string())),
            Box::new(JsExpr::Lit(JsLit::Num(1.0))),
            Box::new(JsExpr::Lit(JsLit::Num(0.0))),
        );
        assert_eq!(expr.to_string(), "(cond) ? (1) : (0)");
    }
    #[test]
    pub(super) fn test_js_type_display() {
        assert_eq!(JsType::BigInt.to_string(), "bigint");
        assert_eq!(JsType::Boolean.to_string(), "boolean");
        assert_eq!(JsType::Unknown.to_string(), "unknown");
    }
    #[test]
    pub(super) fn test_switch_stmt_display() {
        let stmt = JsStmt::Switch(
            JsExpr::Var("x".to_string()),
            vec![(
                JsExpr::Lit(JsLit::Str("Some".to_string())),
                vec![JsStmt::Return(JsExpr::Lit(JsLit::Bool(true)))],
            )],
            vec![JsStmt::Return(JsExpr::Lit(JsLit::Bool(false)))],
        );
        let s = stmt.to_string();
        assert!(s.contains("switch (x)"));
        assert!(s.contains("case \"Some\":"));
        assert!(s.contains("default:"));
    }
}
#[cfg(test)]
mod js_extended_tests {
    use super::*;
    #[test]
    pub(super) fn test_source_map_entry_new() {
        let entry = SourceMapEntry::new(10, 5, "my_fn", 42);
        assert_eq!(entry.gen_line, 10);
        assert_eq!(entry.gen_col, 5);
        assert_eq!(entry.source_fn, "my_fn");
        assert_eq!(entry.source_line, 42);
    }
    #[test]
    pub(super) fn test_source_map_entry_display() {
        let entry = SourceMapEntry::new(1, 0, "f", 10);
        let s = entry.to_string();
        assert!(s.contains("1:0"));
        assert!(s.contains("f:10"));
    }
    #[test]
    pub(super) fn test_js_source_map_new() {
        let sm = JsSourceMap::new();
        assert!(sm.is_empty());
    }
    #[test]
    pub(super) fn test_js_source_map_add_and_query() {
        let mut sm = JsSourceMap::new();
        sm.add(SourceMapEntry::new(5, 0, "add", 1));
        sm.add(SourceMapEntry::new(5, 10, "sub", 2));
        sm.add(SourceMapEntry::new(6, 0, "mul", 3));
        assert_eq!(sm.len(), 3);
        assert_eq!(sm.entries_for_line(5).len(), 2);
        assert_eq!(sm.entries_for_line(6).len(), 1);
        assert!(sm.entries_for_line(99).is_empty());
    }
    #[test]
    pub(super) fn test_js_minifier_strip_comments() {
        let source = "const x = 1; // this is a comment\nconst y = 2;\n";
        let minified = JsMinifier::minify(source);
        assert!(!minified.contains("// this is a comment"));
        assert!(minified.contains("const x = 1;"));
    }
    #[test]
    pub(super) fn test_js_minifier_empty_lines() {
        let source = "\n\n\nconst z = 3;\n\n";
        let minified = JsMinifier::minify(source);
        assert!(minified.contains("const z = 3;"));
    }
    #[test]
    pub(super) fn test_js_minifier_strip_block_comments() {
        let source = "const a = /* inline comment */ 5;";
        let stripped = JsMinifier::strip_block_comments(source);
        assert!(!stripped.contains("inline comment"));
        assert!(stripped.contains("const a ="));
        assert!(stripped.contains("5;"));
    }
    #[test]
    pub(super) fn test_js_pretty_printer_new() {
        let printer = JsPrettyPrinter::new();
        assert_eq!(printer.indent_width, 2);
        assert_eq!(printer.line_width, 80);
    }
    #[test]
    pub(super) fn test_js_pretty_printer_print_function() {
        let printer = JsPrettyPrinter::new();
        let func = JsFunction {
            name: "id".to_string(),
            params: vec!["x".to_string()],
            body: vec![JsStmt::Return(JsExpr::Var("x".to_string()))],
            is_async: false,
            is_export: false,
        };
        let s = printer.print_function(&func);
        assert!(s.contains("function id"));
    }
    #[test]
    pub(super) fn test_type_checker_lit_types() {
        assert_eq!(JsTypeChecker::infer_lit(&JsLit::BigInt(42)), JsType::BigInt);
        assert_eq!(
            JsTypeChecker::infer_lit(&JsLit::Bool(true)),
            JsType::Boolean
        );
        assert_eq!(
            JsTypeChecker::infer_lit(&JsLit::Str("hi".to_string())),
            JsType::String
        );
        assert_eq!(JsTypeChecker::infer_lit(&JsLit::Num(3.14)), JsType::Number);
        assert_eq!(JsTypeChecker::infer_lit(&JsLit::Null), JsType::Null);
        assert_eq!(
            JsTypeChecker::infer_lit(&JsLit::Undefined),
            JsType::Undefined
        );
    }
    #[test]
    pub(super) fn test_type_checker_binop_comparison() {
        let expr = JsExpr::BinOp(
            "===".to_string(),
            Box::new(JsExpr::Lit(JsLit::Num(1.0))),
            Box::new(JsExpr::Lit(JsLit::Num(2.0))),
        );
        assert_eq!(JsTypeChecker::infer_expr(&expr), JsType::Boolean);
    }
    #[test]
    pub(super) fn test_type_checker_object_array() {
        let obj = JsExpr::Object(vec![]);
        assert_eq!(JsTypeChecker::infer_expr(&obj), JsType::Object);
        let arr = JsExpr::Array(vec![]);
        assert_eq!(JsTypeChecker::infer_expr(&arr), JsType::Array);
    }
    #[test]
    pub(super) fn test_type_checker_arrow_function() {
        let arrow = JsExpr::Arrow(vec![], Box::new(JsStmt::ReturnVoid));
        assert_eq!(JsTypeChecker::infer_expr(&arrow), JsType::Function);
    }
    #[test]
    pub(super) fn test_type_checker_typeof() {
        let expr = JsExpr::UnOp("typeof".to_string(), Box::new(JsExpr::Var("x".to_string())));
        assert_eq!(JsTypeChecker::infer_expr(&expr), JsType::String);
    }
    #[test]
    pub(super) fn test_name_mangler_no_namespace() {
        let mangler = JsNameMangler::new("");
        assert_eq!(mangler.mangle("Nat.add"), "Nat_add");
    }
    #[test]
    pub(super) fn test_name_mangler_with_namespace() {
        let mangler = JsNameMangler::new("OL");
        assert_eq!(mangler.mangle("add"), "OL_add");
    }
    #[test]
    pub(super) fn test_name_mangler_qualified() {
        let mangler = JsNameMangler::new("Lean");
        let result = mangler.mangle_qualified(&["Nat", "add"]);
        assert!(result.contains("Lean"));
        assert!(result.contains("Nat_add"));
    }
    #[test]
    pub(super) fn test_js_module_linker_new() {
        let linker = JsModuleLinker::new();
        assert!(linker.is_empty());
    }
    #[test]
    pub(super) fn test_js_module_linker_link_empty() {
        let linker = JsModuleLinker::new();
        let linked = linker.link();
        assert!(linked.functions.is_empty());
        assert!(linked.exports.is_empty());
    }
    #[test]
    pub(super) fn test_js_module_linker_link_multiple() {
        let mut linker = JsModuleLinker::new();
        let mut m1 = JsModule::new();
        m1.add_function(JsFunction {
            name: "f".to_string(),
            params: vec![],
            body: vec![JsStmt::ReturnVoid],
            is_async: false,
            is_export: false,
        });
        m1.add_export("f".to_string());
        let mut m2 = JsModule::new();
        m2.add_function(JsFunction {
            name: "g".to_string(),
            params: vec![],
            body: vec![JsStmt::ReturnVoid],
            is_async: false,
            is_export: false,
        });
        m2.add_export("g".to_string());
        linker.add_module(m1);
        linker.add_module(m2);
        let linked = linker.link();
        assert_eq!(linked.functions.len(), 2);
        assert_eq!(linked.exports.len(), 2);
    }
    #[test]
    pub(super) fn test_js_module_linker_dedup_preamble() {
        let mut linker = JsModuleLinker::new();
        let mut m1 = JsModule::new();
        m1.add_preamble("const X = 1;".to_string());
        let mut m2 = JsModule::new();
        m2.add_preamble("const X = 1;".to_string());
        m2.add_preamble("const Y = 2;".to_string());
        linker.add_module(m1);
        linker.add_module(m2);
        let linked = linker.link();
        assert_eq!(linked.preamble.len(), 2);
    }
    #[test]
    pub(super) fn test_peephole_fold_add() {
        let expr = JsExpr::BinOp(
            "+".to_string(),
            Box::new(JsExpr::Lit(JsLit::Num(3.0))),
            Box::new(JsExpr::Lit(JsLit::Num(4.0))),
        );
        let result = JsPeephole::fold_arith(&expr);
        assert_eq!(result, JsExpr::Lit(JsLit::Num(7.0)));
    }
    #[test]
    pub(super) fn test_peephole_fold_mul() {
        let expr = JsExpr::BinOp(
            "*".to_string(),
            Box::new(JsExpr::Lit(JsLit::Num(5.0))),
            Box::new(JsExpr::Lit(JsLit::Num(6.0))),
        );
        let result = JsPeephole::fold_arith(&expr);
        assert_eq!(result, JsExpr::Lit(JsLit::Num(30.0)));
    }
    #[test]
    pub(super) fn test_peephole_identity_eq() {
        let x = JsExpr::Var("x".to_string());
        let expr = JsExpr::BinOp("===".to_string(), Box::new(x.clone()), Box::new(x));
        let result = JsPeephole::simplify_identity(&expr);
        assert_eq!(result, JsExpr::Lit(JsLit::Bool(true)));
    }
    #[test]
    pub(super) fn test_peephole_not_true() {
        let expr = JsExpr::UnOp("!".to_string(), Box::new(JsExpr::Lit(JsLit::Bool(true))));
        let result = JsPeephole::simplify_not(&expr);
        assert_eq!(result, JsExpr::Lit(JsLit::Bool(false)));
    }
    #[test]
    pub(super) fn test_peephole_not_false() {
        let expr = JsExpr::UnOp("!".to_string(), Box::new(JsExpr::Lit(JsLit::Bool(false))));
        let result = JsPeephole::simplify_not(&expr);
        assert_eq!(result, JsExpr::Lit(JsLit::Bool(true)));
    }
    #[test]
    pub(super) fn test_peephole_no_fold_non_numeric() {
        let expr = JsExpr::BinOp(
            "+".to_string(),
            Box::new(JsExpr::Lit(JsLit::Str("a".to_string()))),
            Box::new(JsExpr::Lit(JsLit::Str("b".to_string()))),
        );
        let result = JsPeephole::fold_arith(&expr);
        assert_eq!(result, expr);
    }
    #[test]
    pub(super) fn test_js_backend_config_default() {
        let cfg = JsBackendConfig::default();
        assert!(cfg.use_bigint_for_nat);
        assert!(!cfg.minify);
        assert!(cfg.include_runtime);
        assert_eq!(cfg.module_format, JsModuleFormat::Es);
    }
    #[test]
    pub(super) fn test_js_backend_config_display() {
        let cfg = JsBackendConfig::default();
        let s = cfg.to_string();
        assert!(s.contains("bigint=true"));
    }
    #[test]
    pub(super) fn test_js_ident_table_new() {
        let table = JsIdentTable::new();
        assert!(table.is_empty());
    }
    #[test]
    pub(super) fn test_js_ident_table_register_unique() {
        let mut table = JsIdentTable::new();
        let name = table.register("add");
        assert_eq!(name, "add");
        assert!(table.is_taken("add"));
    }
    #[test]
    pub(super) fn test_js_ident_table_register_collision() {
        let mut table = JsIdentTable::new();
        table.register("x");
        let renamed = table.register("x");
        assert_ne!(renamed, "x");
        assert!(renamed.starts_with("x_"));
    }
    #[test]
    pub(super) fn test_js_ident_table_len() {
        let mut table = JsIdentTable::new();
        table.register("a");
        table.register("b");
        table.register("c");
        assert_eq!(table.len(), 3);
    }
    #[test]
    pub(super) fn test_js_emit_context_new() {
        let ctx = JsEmitContext::new("  ");
        assert_eq!(ctx.indent_level, 0);
        assert_eq!(ctx.current_line, 0);
    }
    #[test]
    pub(super) fn test_js_emit_context_indent() {
        let mut ctx = JsEmitContext::new("  ");
        ctx.push_indent();
        ctx.push_indent();
        assert_eq!(ctx.indent(), "    ");
        ctx.pop_indent();
        assert_eq!(ctx.indent(), "  ");
    }
    #[test]
    pub(super) fn test_js_emit_context_newline() {
        let mut ctx = JsEmitContext::new("  ");
        ctx.newline();
        assert_eq!(ctx.current_line, 1);
        assert_eq!(ctx.current_col, 0);
    }
    #[test]
    pub(super) fn test_js_emit_context_record_mapping() {
        let mut ctx = JsEmitContext::new("  ");
        ctx.record_mapping("my_fn", 42);
        assert_eq!(ctx.source_map.len(), 1);
    }
    #[test]
    pub(super) fn test_js_size_estimator_expr() {
        let expr = JsExpr::Lit(JsLit::Num(42.0));
        let size = JsSizeEstimator::estimate_expr(&expr);
        assert!(size > 0);
    }
    #[test]
    pub(super) fn test_js_size_estimator_function() {
        let func = JsFunction {
            name: "f".to_string(),
            params: vec![],
            body: vec![JsStmt::Return(JsExpr::Lit(JsLit::Num(0.0)))],
            is_async: false,
            is_export: false,
        };
        let size = JsSizeEstimator::estimate_function(&func);
        assert!(size > 0);
    }
    #[test]
    pub(super) fn test_js_size_estimator_module() {
        let module = JsModule::new();
        let size = JsSizeEstimator::estimate_module(&module);
        assert!(size > 0);
    }
    #[test]
    pub(super) fn test_js_size_estimator_stmt() {
        let stmt = JsStmt::Return(JsExpr::Lit(JsLit::Bool(true)));
        let size = JsSizeEstimator::estimate_stmt(&stmt);
        assert!(size > 0);
    }
    #[test]
    pub(super) fn test_js_lit_bool_display() {
        assert_eq!(JsLit::Bool(true).to_string(), "true");
        assert_eq!(JsLit::Bool(false).to_string(), "false");
    }
    #[test]
    pub(super) fn test_js_lit_null_display() {
        assert_eq!(JsLit::Null.to_string(), "null");
    }
    #[test]
    pub(super) fn test_js_lit_undefined_display() {
        assert_eq!(JsLit::Undefined.to_string(), "undefined");
    }
    #[test]
    pub(super) fn test_js_stmt_while_display() {
        let stmt = JsStmt::While(
            JsExpr::Lit(JsLit::Bool(true)),
            vec![JsStmt::Return(JsExpr::Lit(JsLit::Num(0.0)))],
        );
        let s = stmt.to_string();
        assert!(s.contains("while (true)"));
    }
    #[test]
    pub(super) fn test_js_stmt_try_catch_display() {
        let stmt = JsStmt::TryCatch(
            vec![JsStmt::Expr(JsExpr::Lit(JsLit::Num(1.0)))],
            "e".to_string(),
            vec![JsStmt::Throw(JsExpr::Var("e".to_string()))],
        );
        let s = stmt.to_string();
        assert!(s.contains("try"));
        assert!(s.contains("catch (e)"));
    }
    #[test]
    pub(super) fn test_js_stmt_for_display() {
        let stmt = JsStmt::For(
            "item".to_string(),
            JsExpr::Var("items".to_string()),
            vec![JsStmt::Expr(JsExpr::Var("item".to_string()))],
        );
        let s = stmt.to_string();
        assert!(s.contains("for (const item of items)"));
    }
    #[test]
    pub(super) fn test_js_expr_await_display() {
        let expr = JsExpr::Await(Box::new(JsExpr::Var("promise".to_string())));
        assert_eq!(expr.to_string(), "await promise");
    }
    #[test]
    pub(super) fn test_js_expr_spread_display() {
        let expr = JsExpr::Spread(Box::new(JsExpr::Var("arr".to_string())));
        assert_eq!(expr.to_string(), "...arr");
    }
    #[test]
    pub(super) fn test_js_expr_new_display() {
        let expr = JsExpr::New("MyClass".to_string(), vec![JsExpr::Lit(JsLit::Num(1.0))]);
        assert_eq!(expr.to_string(), "new MyClass(1)");
    }
    #[test]
    pub(super) fn test_js_expr_index_display() {
        let expr = JsExpr::Index(
            Box::new(JsExpr::Var("arr".to_string())),
            Box::new(JsExpr::Lit(JsLit::Num(0.0))),
        );
        assert_eq!(expr.to_string(), "arr[0]");
    }
    #[test]
    pub(super) fn test_js_module_emit_includes_runtime() {
        let module = JsModule::new();
        let output = module.emit();
        assert!(output.contains("_OL"));
        assert!(output.contains("natAdd"));
    }
    #[test]
    pub(super) fn test_js_module_emit_exports() {
        let mut module = JsModule::new();
        module.add_export("foo".to_string());
        module.add_export("bar".to_string());
        let output = module.emit();
        assert!(output.contains("export {"));
        assert!(output.contains("foo"));
        assert!(output.contains("bar"));
    }
    #[test]
    pub(super) fn test_js_module_emit_preamble() {
        let mut module = JsModule::new();
        module.add_preamble("const VERSION = '1.0';".to_string());
        let output = module.emit();
        assert!(output.contains("const VERSION = '1.0';"));
    }
}
#[cfg(test)]
mod JS_infra_tests {
    use super::*;
    #[test]
    pub(super) fn test_pass_config() {
        let config = JSPassConfig::new("test_pass", JSPassPhase::Transformation);
        assert!(config.enabled);
        assert!(config.phase.is_modifying());
        assert_eq!(config.phase.name(), "transformation");
    }
    #[test]
    pub(super) fn test_pass_stats() {
        let mut stats = JSPassStats::new();
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
        let mut reg = JSPassRegistry::new();
        reg.register(JSPassConfig::new("pass_a", JSPassPhase::Analysis));
        reg.register(JSPassConfig::new("pass_b", JSPassPhase::Transformation).disabled());
        assert_eq!(reg.total_passes(), 2);
        assert_eq!(reg.enabled_count(), 1);
        reg.update_stats("pass_a", 5, 50, 2);
        let stats = reg.get_stats("pass_a").expect("stats should exist");
        assert_eq!(stats.total_changes, 5);
    }
    #[test]
    pub(super) fn test_analysis_cache() {
        let mut cache = JSAnalysisCache::new(10);
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
        let mut wl = JSWorklist::new();
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
        let mut dt = JSDominatorTree::new(5);
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
        let mut liveness = JSLivenessInfo::new(3);
        liveness.add_def(0, 1);
        liveness.add_use(1, 1);
        assert!(liveness.defs[0].contains(&1));
        assert!(liveness.uses[1].contains(&1));
    }
    #[test]
    pub(super) fn test_constant_folding() {
        assert_eq!(JSConstantFoldingHelper::fold_add_i64(3, 4), Some(7));
        assert_eq!(JSConstantFoldingHelper::fold_div_i64(10, 0), None);
        assert_eq!(JSConstantFoldingHelper::fold_div_i64(10, 2), Some(5));
        assert_eq!(
            JSConstantFoldingHelper::fold_bitand_i64(0b1100, 0b1010),
            0b1000
        );
        assert_eq!(JSConstantFoldingHelper::fold_bitnot_i64(0), -1);
    }
    #[test]
    pub(super) fn test_dep_graph() {
        let mut g = JSDepGraph::new();
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
