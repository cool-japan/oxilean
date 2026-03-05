//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{
    TSAnalysisCache, TSConstantFoldingHelper, TSDepGraph, TSDominatorTree, TSLivenessInfo,
    TSPassConfig, TSPassPhase, TSPassRegistry, TSPassStats, TSWorklist, TsClass, TsClassField,
    TsClassMethod, TsDeclaration, TsEnum, TsEnumMember, TsExpr, TsExtConfig, TsExtDiagCollector,
    TsExtDiagMsg, TsExtEmitStats, TsExtEventLog, TsExtFeatures, TsExtIdGen, TsExtIncrKey,
    TsExtNameScope, TsExtPassTiming, TsExtProfiler, TsExtSourceBuffer, TsExtVersion, TsFunction,
    TsImport, TsInterface, TsInterfaceMember, TsLit, TsModule, TsParam, TsStmt, TsTemplatePart,
    TsType, TsTypeAlias, TypeScriptBackend,
};

pub(super) fn format_ts_stmt(stmt: &TsStmt, indent: usize) -> std::string::String {
    let pad = " ".repeat(indent);
    let _ipad = " ".repeat(indent + 2);
    match stmt {
        TsStmt::Expr(e) => format!("{};", e),
        TsStmt::Const(name, ty, expr) => {
            if let Some(t) = ty {
                format!("const {}: {} = {};", name, t, expr)
            } else {
                format!("const {} = {};", name, expr)
            }
        }
        TsStmt::Let(name, ty, expr) => {
            if let Some(t) = ty {
                format!("let {}: {} = {};", name, t, expr)
            } else {
                format!("let {} = {};", name, expr)
            }
        }
        TsStmt::Var(name, ty, expr) => {
            if let Some(t) = ty {
                format!("var {}: {} = {};", name, t, expr)
            } else {
                format!("var {} = {};", name, expr)
            }
        }
        TsStmt::If(cond, then_s, else_s) => {
            let then_body = format_ts_stmts(then_s, indent + 2);
            let mut s = format!("if ({}) {{\n{}\n{}}}", cond, then_body, pad);
            if !else_s.is_empty() {
                let else_body = format_ts_stmts(else_s, indent + 2);
                s.push_str(&format!(" else {{\n{}\n{}}}", else_body, pad));
            }
            s
        }
        TsStmt::Switch(expr, cases, default) => {
            let mut s = format!("switch ({}) {{\n", expr);
            for (case_val, case_stmts) in cases {
                s.push_str(&format!("{}  case {}:\n", pad, case_val));
                for cs in case_stmts {
                    s.push_str(&format!("{}    {}\n", pad, format_ts_stmt(cs, indent + 4)));
                }
                s.push_str(&format!("{}    break;\n", pad));
            }
            if !default.is_empty() {
                s.push_str(&format!("{}  default:\n", pad));
                for ds in default {
                    s.push_str(&format!("{}    {}\n", pad, format_ts_stmt(ds, indent + 4)));
                }
            }
            s.push_str(&format!("{}}}", pad));
            s
        }
        TsStmt::For(init, cond, step, body) => {
            let body_text = format_ts_stmts(body, indent + 2);
            format!(
                "for ({}; {}; {}) {{\n{}\n{}}}",
                format_ts_stmt(init, 0).trim_end_matches(';'),
                cond,
                step,
                body_text,
                pad
            )
        }
        TsStmt::ForOf(var, iter, body) => {
            let body_text = format_ts_stmts(body, indent + 2);
            format!(
                "for (const {} of {}) {{\n{}\n{}}}",
                var, iter, body_text, pad
            )
        }
        TsStmt::ForIn(var, obj, body) => {
            let body_text = format_ts_stmts(body, indent + 2);
            format!(
                "for (const {} in {}) {{\n{}\n{}}}",
                var, obj, body_text, pad
            )
        }
        TsStmt::While(cond, body) => {
            let body_text = format_ts_stmts(body, indent + 2);
            format!("while ({}) {{\n{}\n{}}}", cond, body_text, pad)
        }
        TsStmt::Return(e) => format!("return {};", e),
        TsStmt::Throw(e) => format!("throw {};", e),
        TsStmt::TryCatch(try_body, catch_var, catch_body, fin_body) => {
            let try_text = format_ts_stmts(try_body, indent + 2);
            let catch_text = format_ts_stmts(catch_body, indent + 2);
            let mut s = format!(
                "try {{\n{}\n{}}} catch ({}) {{\n{}\n{}}}",
                try_text, pad, catch_var, catch_text, pad
            );
            if !fin_body.is_empty() {
                let fin_text = format_ts_stmts(fin_body, indent + 2);
                s.push_str(&format!(" finally {{\n{}\n{}}}", fin_text, pad));
            }
            s
        }
        TsStmt::Block(stmts) => {
            let body = format_ts_stmts(stmts, indent + 2);
            format!("{{\n{}\n{}}}", body, pad)
        }
        TsStmt::Break => "break;".to_string(),
        TsStmt::Continue => "continue;".to_string(),
    }
}
pub(super) fn format_ts_stmts(stmts: &[TsStmt], indent: usize) -> std::string::String {
    let pad = " ".repeat(indent);
    let mut out = std::string::String::new();
    for stmt in stmts {
        let text = format_ts_stmt(stmt, indent);
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
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub(super) fn test_ts_type_primitives() {
        assert_eq!(TsType::Number.to_string(), "number");
        assert_eq!(TsType::String.to_string(), "string");
        assert_eq!(TsType::Boolean.to_string(), "boolean");
        assert_eq!(TsType::Void.to_string(), "void");
        assert_eq!(TsType::Never.to_string(), "never");
        assert_eq!(TsType::Unknown.to_string(), "unknown");
        assert_eq!(TsType::Any.to_string(), "any");
        assert_eq!(TsType::Null.to_string(), "null");
        assert_eq!(TsType::Undefined.to_string(), "undefined");
    }
    #[test]
    pub(super) fn test_ts_type_array() {
        let t = TsType::Array(Box::new(TsType::Number));
        assert_eq!(t.to_string(), "number[]");
    }
    #[test]
    pub(super) fn test_ts_type_tuple() {
        let t = TsType::Tuple(vec![TsType::Number, TsType::String, TsType::Boolean]);
        assert_eq!(t.to_string(), "[number, string, boolean]");
    }
    #[test]
    pub(super) fn test_ts_type_union() {
        let t = TsType::Union(vec![TsType::Number, TsType::Null, TsType::Undefined]);
        assert_eq!(t.to_string(), "number | null | undefined");
    }
    #[test]
    pub(super) fn test_ts_type_intersection() {
        let t = TsType::Intersection(vec![
            TsType::Custom("Serializable".to_string()),
            TsType::Custom("Loggable".to_string()),
        ]);
        assert_eq!(t.to_string(), "Serializable & Loggable");
    }
    #[test]
    pub(super) fn test_ts_type_function() {
        let t = TsType::Function {
            params: vec![TsType::Number, TsType::String],
            ret: Box::new(TsType::Boolean),
        };
        assert_eq!(t.to_string(), "(p0: number, p1: string) => boolean");
    }
    #[test]
    pub(super) fn test_ts_type_generic() {
        let t = TsType::Generic("Promise".to_string(), vec![TsType::String]);
        assert_eq!(t.to_string(), "Promise<string>");
    }
    #[test]
    pub(super) fn test_ts_type_generic_map() {
        let t = TsType::Generic("Map".to_string(), vec![TsType::String, TsType::Number]);
        assert_eq!(t.to_string(), "Map<string, number>");
    }
    #[test]
    pub(super) fn test_ts_type_readonly() {
        let t = TsType::ReadOnly(Box::new(TsType::Array(Box::new(TsType::Number))));
        assert_eq!(t.to_string(), "readonly number[]");
    }
    #[test]
    pub(super) fn test_ts_lit_number() {
        assert_eq!(TsLit::Num(42.0).to_string(), "42");
        assert_eq!(TsLit::Num(3.14).to_string(), "3.14");
    }
    #[test]
    pub(super) fn test_ts_lit_string_escape() {
        assert_eq!(TsLit::Str("hello".to_string()).to_string(), "\"hello\"");
        assert_eq!(
            TsLit::Str("say \"hi\"".to_string()).to_string(),
            "\"say \\\"hi\\\"\""
        );
    }
    #[test]
    pub(super) fn test_ts_lit_bool_and_null() {
        assert_eq!(TsLit::Bool(true).to_string(), "true");
        assert_eq!(TsLit::Bool(false).to_string(), "false");
        assert_eq!(TsLit::Null.to_string(), "null");
        assert_eq!(TsLit::Undefined.to_string(), "undefined");
    }
    #[test]
    pub(super) fn test_ts_expr_binop() {
        let e = TsExpr::BinOp(
            "+".to_string(),
            Box::new(TsExpr::Lit(TsLit::Num(1.0))),
            Box::new(TsExpr::Lit(TsLit::Num(2.0))),
        );
        assert_eq!(e.to_string(), "(1 + 2)");
    }
    #[test]
    pub(super) fn test_ts_expr_ternary() {
        let e = TsExpr::Ternary(
            Box::new(TsExpr::Var("cond".to_string())),
            Box::new(TsExpr::Lit(TsLit::Num(1.0))),
            Box::new(TsExpr::Lit(TsLit::Num(0.0))),
        );
        assert_eq!(e.to_string(), "(cond) ? (1) : (0)");
    }
    #[test]
    pub(super) fn test_ts_expr_as() {
        let e = TsExpr::As(Box::new(TsExpr::Var("x".to_string())), TsType::Number);
        assert_eq!(e.to_string(), "(x as number)");
    }
    #[test]
    pub(super) fn test_ts_expr_satisfies() {
        let e = TsExpr::Satisfies(
            Box::new(TsExpr::Var("config".to_string())),
            TsType::Custom("Config".to_string()),
        );
        assert_eq!(e.to_string(), "(config satisfies Config)");
    }
    #[test]
    pub(super) fn test_ts_expr_template_literal() {
        let e = TsExpr::Template(vec![
            TsTemplatePart::Text("Hello, ".to_string()),
            TsTemplatePart::Expr(TsExpr::Var("name".to_string())),
            TsTemplatePart::Text("!".to_string()),
        ]);
        assert_eq!(e.to_string(), "`Hello, ${name}!`");
    }
    #[test]
    pub(super) fn test_ts_expr_nullish() {
        let e = TsExpr::Nullish(
            Box::new(TsExpr::Var("a".to_string())),
            Box::new(TsExpr::Lit(TsLit::Str("default".to_string()))),
        );
        assert_eq!(e.to_string(), "(a ?? \"default\")");
    }
    #[test]
    pub(super) fn test_ts_expr_optchain() {
        let e = TsExpr::OptChain(
            Box::new(TsExpr::Var("user".to_string())),
            "address".to_string(),
        );
        assert_eq!(e.to_string(), "user?.address");
    }
    #[test]
    pub(super) fn test_ts_expr_object_lit() {
        let e = TsExpr::ObjectLit(vec![
            (
                "kind".to_string(),
                TsExpr::Lit(TsLit::Str("circle".to_string())),
            ),
            ("radius".to_string(), TsExpr::Lit(TsLit::Num(5.0))),
        ]);
        assert_eq!(e.to_string(), "{ kind: \"circle\", radius: 5 }");
    }
    #[test]
    pub(super) fn test_ts_expr_array_lit() {
        let e = TsExpr::ArrayLit(vec![
            TsExpr::Lit(TsLit::Num(1.0)),
            TsExpr::Lit(TsLit::Num(2.0)),
            TsExpr::Lit(TsLit::Num(3.0)),
        ]);
        assert_eq!(e.to_string(), "[1, 2, 3]");
    }
    #[test]
    pub(super) fn test_ts_interface_basic() {
        let iface = TsInterface {
            name: "Animal".to_string(),
            extends: vec![],
            members: vec![
                TsInterfaceMember {
                    name: "name".to_string(),
                    ty: TsType::String,
                    optional: false,
                    readonly: true,
                },
                TsInterfaceMember {
                    name: "age".to_string(),
                    ty: TsType::Number,
                    optional: true,
                    readonly: false,
                },
            ],
            type_params: vec![],
        };
        let src = iface.to_string();
        assert!(src.contains("interface Animal {"));
        assert!(src.contains("readonly name: string;"));
        assert!(src.contains("age?: number;"));
    }
    #[test]
    pub(super) fn test_ts_interface_extends() {
        let iface = TsInterface {
            name: "Dog".to_string(),
            extends: vec!["Animal".to_string(), "Pet".to_string()],
            members: vec![],
            type_params: vec![],
        };
        let src = iface.to_string();
        assert!(src.contains("extends Animal, Pet"));
    }
    #[test]
    pub(super) fn test_ts_type_alias() {
        let alias = TsTypeAlias {
            name: "StringOrNumber".to_string(),
            type_params: vec![],
            definition: TsType::Union(vec![TsType::String, TsType::Number]),
        };
        assert_eq!(alias.to_string(), "type StringOrNumber = string | number;");
    }
    #[test]
    pub(super) fn test_ts_type_alias_generic() {
        let alias = TsTypeAlias {
            name: "Nullable".to_string(),
            type_params: vec!["T".to_string()],
            definition: TsType::Union(vec![TsType::Custom("T".to_string()), TsType::Null]),
        };
        assert_eq!(alias.to_string(), "type Nullable<T> = T | null;");
    }
    #[test]
    pub(super) fn test_ts_enum() {
        let e = TsEnum {
            name: "Direction".to_string(),
            is_const: false,
            members: vec![
                TsEnumMember {
                    name: "North".to_string(),
                    value: None,
                },
                TsEnumMember {
                    name: "South".to_string(),
                    value: None,
                },
            ],
        };
        let src = e.to_string();
        assert!(src.contains("enum Direction {"));
        assert!(src.contains("North,"));
        assert!(src.contains("South,"));
    }
    #[test]
    pub(super) fn test_ts_const_enum_with_values() {
        let e = TsEnum {
            name: "Status".to_string(),
            is_const: true,
            members: vec![
                TsEnumMember {
                    name: "OK".to_string(),
                    value: Some(TsLit::Num(200.0)),
                },
                TsEnumMember {
                    name: "NotFound".to_string(),
                    value: Some(TsLit::Num(404.0)),
                },
            ],
        };
        let src = e.to_string();
        assert!(src.contains("const enum Status {"));
        assert!(src.contains("OK = 200,"));
        assert!(src.contains("NotFound = 404,"));
    }
    #[test]
    pub(super) fn test_ts_function_emit() {
        let f = TsFunction {
            name: "add".to_string(),
            params: vec![
                TsParam {
                    name: "a".to_string(),
                    ty: TsType::Number,
                    optional: false,
                    rest: false,
                },
                TsParam {
                    name: "b".to_string(),
                    ty: TsType::Number,
                    optional: false,
                    rest: false,
                },
            ],
            return_type: TsType::Number,
            body: vec![TsStmt::Return(TsExpr::BinOp(
                "+".to_string(),
                Box::new(TsExpr::Var("a".to_string())),
                Box::new(TsExpr::Var("b".to_string())),
            ))],
            is_async: false,
            type_params: vec![],
            is_exported: true,
        };
        let src = f.to_string();
        assert!(src.contains("export function add(a: number, b: number): number {"));
        assert!(src.contains("return (a + b);"));
    }
    #[test]
    pub(super) fn test_backend_discriminated_union() {
        let backend = TypeScriptBackend::new();
        let alias = backend.make_discriminated_union(
            "Shape",
            &[
                ("circle", vec![("radius", TsType::Number)]),
                ("rect", vec![("w", TsType::Number), ("h", TsType::Number)]),
            ],
        );
        let src = alias.to_string();
        assert!(src.contains("type Shape ="));
        assert!(src.contains("'circle'"));
        assert!(src.contains("radius: number"));
        assert!(src.contains("'rect'"));
        assert!(src.contains("w: number"));
    }
    #[test]
    pub(super) fn test_module_emit() {
        let mut module = TsModule::new();
        module.imports.push(TsImport {
            names: vec!["readFileSync".to_string()],
            from: "fs".to_string(),
            is_type: false,
        });
        module.declarations.push(TsDeclaration::Const(
            "VERSION".to_string(),
            Some(TsType::String),
            TsExpr::Lit(TsLit::Str("1.0.0".to_string())),
        ));
        let src = module.emit();
        assert!(src.contains("import { readFileSync } from \"fs\";"));
        assert!(src.contains("export const VERSION: string = \"1.0.0\";"));
    }
    #[test]
    pub(super) fn test_module_emit_d_ts() {
        let mut module = TsModule::new();
        module
            .declarations
            .push(TsDeclaration::Function(TsFunction {
                name: "greet".to_string(),
                params: vec![TsParam {
                    name: "name".to_string(),
                    ty: TsType::String,
                    optional: false,
                    rest: false,
                }],
                return_type: TsType::String,
                body: vec![],
                is_async: false,
                type_params: vec![],
                is_exported: true,
            }));
        let dts = module.emit_d_ts();
        assert!(dts.contains("declare function greet"));
        assert!(dts.contains("name: string"));
        assert!(dts.contains(": string;"));
    }
    #[test]
    pub(super) fn test_ts_class_emit() {
        let cls = TsClass {
            name: "Counter".to_string(),
            extends: None,
            implements: vec!["ICounter".to_string()],
            fields: vec![TsClassField {
                name: "count".to_string(),
                ty: TsType::Number,
                readonly: false,
                optional: false,
                is_private: true,
                is_static: false,
            }],
            methods: vec![TsClassMethod {
                name: "increment".to_string(),
                params: vec![],
                return_type: TsType::Void,
                body: vec![TsStmt::Expr(TsExpr::BinOp(
                    "+=".to_string(),
                    Box::new(TsExpr::Var("this.count".to_string())),
                    Box::new(TsExpr::Lit(TsLit::Num(1.0))),
                ))],
                is_async: false,
                is_static: false,
                is_private: false,
                is_getter: false,
                is_setter: false,
            }],
            type_params: vec![],
            is_exported: true,
        };
        let src = cls.to_string();
        assert!(src.contains("export class Counter"));
        assert!(src.contains("implements ICounter"));
        assert!(src.contains("private count: number;"));
        assert!(src.contains("increment()"));
    }
    #[test]
    pub(super) fn test_ts_stmt_try_catch_finally() {
        let stmt = TsStmt::TryCatch(
            vec![TsStmt::Expr(TsExpr::Call(
                Box::new(TsExpr::Var("risky".to_string())),
                vec![],
            ))],
            "err".to_string(),
            vec![TsStmt::Throw(TsExpr::Var("err".to_string()))],
            vec![TsStmt::Expr(TsExpr::Call(
                Box::new(TsExpr::Var("cleanup".to_string())),
                vec![],
            ))],
        );
        let src = stmt.to_string();
        assert!(src.contains("try {"));
        assert!(src.contains("catch (err)"));
        assert!(src.contains("finally {"));
    }
    #[test]
    pub(super) fn test_ts_stmt_for_of() {
        let stmt = TsStmt::ForOf(
            "item".to_string(),
            TsExpr::Var("items".to_string()),
            vec![TsStmt::Expr(TsExpr::Call(
                Box::new(TsExpr::Var("process".to_string())),
                vec![TsExpr::Var("item".to_string())],
            ))],
        );
        let src = stmt.to_string();
        assert!(src.contains("for (const item of items)"));
    }
    #[test]
    pub(super) fn test_ts_stmt_switch() {
        let stmt = TsStmt::Switch(
            TsExpr::Var("x".to_string()),
            vec![
                (
                    TsExpr::Lit(TsLit::Num(1.0)),
                    vec![TsStmt::Return(TsExpr::Lit(TsLit::Str("one".to_string())))],
                ),
                (
                    TsExpr::Lit(TsLit::Num(2.0)),
                    vec![TsStmt::Return(TsExpr::Lit(TsLit::Str("two".to_string())))],
                ),
            ],
            vec![TsStmt::Return(TsExpr::Lit(TsLit::Str("other".to_string())))],
        );
        let src = stmt.to_string();
        assert!(src.contains("switch (x)"));
        assert!(src.contains("case 1:"));
        assert!(src.contains("case 2:"));
        assert!(src.contains("default:"));
    }
}
#[cfg(test)]
mod tests_ts_ext_extra {
    use super::*;
    #[test]
    pub(super) fn test_ts_ext_config() {
        let mut cfg = TsExtConfig::new();
        cfg.set("mode", "release");
        cfg.set("verbose", "true");
        assert_eq!(cfg.get("mode"), Some("release"));
        assert!(cfg.get_bool("verbose"));
        assert!(cfg.get_int("mode").is_none());
        assert_eq!(cfg.len(), 2);
    }
    #[test]
    pub(super) fn test_ts_ext_source_buffer() {
        let mut buf = TsExtSourceBuffer::new();
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
    pub(super) fn test_ts_ext_name_scope() {
        let mut scope = TsExtNameScope::new();
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
    pub(super) fn test_ts_ext_diag_collector() {
        let mut col = TsExtDiagCollector::new();
        col.emit(TsExtDiagMsg::warning("pass_a", "slow"));
        col.emit(TsExtDiagMsg::error("pass_b", "fatal"));
        assert!(col.has_errors());
        assert_eq!(col.errors().len(), 1);
        assert_eq!(col.warnings().len(), 1);
        col.clear();
        assert!(col.is_empty());
    }
    #[test]
    pub(super) fn test_ts_ext_id_gen() {
        let mut gen = TsExtIdGen::new();
        assert_eq!(gen.next_id(), 0);
        assert_eq!(gen.next_id(), 1);
        gen.skip(10);
        assert_eq!(gen.next_id(), 12);
        gen.reset();
        assert_eq!(gen.peek_next(), 0);
    }
    #[test]
    pub(super) fn test_ts_ext_incr_key() {
        let k1 = TsExtIncrKey::new(100, 200);
        let k2 = TsExtIncrKey::new(100, 200);
        let k3 = TsExtIncrKey::new(999, 200);
        assert!(k1.matches(&k2));
        assert!(!k1.matches(&k3));
    }
    #[test]
    pub(super) fn test_ts_ext_profiler() {
        let mut p = TsExtProfiler::new();
        p.record(TsExtPassTiming::new("pass_a", 1000, 50, 200, 100));
        p.record(TsExtPassTiming::new("pass_b", 500, 30, 100, 200));
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
    pub(super) fn test_ts_ext_event_log() {
        let mut log = TsExtEventLog::new(3);
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
    pub(super) fn test_ts_ext_version() {
        let v = TsExtVersion::new(1, 2, 3).with_pre("alpha");
        assert!(!v.is_stable());
        assert_eq!(format!("{}", v), "1.2.3-alpha");
        let stable = TsExtVersion::new(2, 0, 0);
        assert!(stable.is_stable());
        assert!(stable.is_compatible_with(&TsExtVersion::new(2, 0, 0)));
        assert!(!stable.is_compatible_with(&TsExtVersion::new(3, 0, 0)));
    }
    #[test]
    pub(super) fn test_ts_ext_features() {
        let mut f = TsExtFeatures::new();
        f.enable("sse2");
        f.enable("avx2");
        assert!(f.is_enabled("sse2"));
        assert!(!f.is_enabled("avx512"));
        f.disable("avx2");
        assert!(!f.is_enabled("avx2"));
        let mut g = TsExtFeatures::new();
        g.enable("sse2");
        g.enable("neon");
        let union = f.union(&g);
        assert!(union.is_enabled("sse2") && union.is_enabled("neon"));
        let inter = f.intersection(&g);
        assert!(inter.is_enabled("sse2"));
    }
    #[test]
    pub(super) fn test_ts_ext_emit_stats() {
        let mut s = TsExtEmitStats::new();
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
mod TS_infra_tests {
    use super::*;
    #[test]
    pub(super) fn test_pass_config() {
        let config = TSPassConfig::new("test_pass", TSPassPhase::Transformation);
        assert!(config.enabled);
        assert!(config.phase.is_modifying());
        assert_eq!(config.phase.name(), "transformation");
    }
    #[test]
    pub(super) fn test_pass_stats() {
        let mut stats = TSPassStats::new();
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
        let mut reg = TSPassRegistry::new();
        reg.register(TSPassConfig::new("pass_a", TSPassPhase::Analysis));
        reg.register(TSPassConfig::new("pass_b", TSPassPhase::Transformation).disabled());
        assert_eq!(reg.total_passes(), 2);
        assert_eq!(reg.enabled_count(), 1);
        reg.update_stats("pass_a", 5, 50, 2);
        let stats = reg.get_stats("pass_a").expect("stats should exist");
        assert_eq!(stats.total_changes, 5);
    }
    #[test]
    pub(super) fn test_analysis_cache() {
        let mut cache = TSAnalysisCache::new(10);
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
        let mut wl = TSWorklist::new();
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
        let mut dt = TSDominatorTree::new(5);
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
        let mut liveness = TSLivenessInfo::new(3);
        liveness.add_def(0, 1);
        liveness.add_use(1, 1);
        assert!(liveness.defs[0].contains(&1));
        assert!(liveness.uses[1].contains(&1));
    }
    #[test]
    pub(super) fn test_constant_folding() {
        assert_eq!(TSConstantFoldingHelper::fold_add_i64(3, 4), Some(7));
        assert_eq!(TSConstantFoldingHelper::fold_div_i64(10, 0), None);
        assert_eq!(TSConstantFoldingHelper::fold_div_i64(10, 2), Some(5));
        assert_eq!(
            TSConstantFoldingHelper::fold_bitand_i64(0b1100, 0b1010),
            0b1000
        );
        assert_eq!(TSConstantFoldingHelper::fold_bitnot_i64(0), -1);
    }
    #[test]
    pub(super) fn test_dep_graph() {
        let mut g = TSDepGraph::new();
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
