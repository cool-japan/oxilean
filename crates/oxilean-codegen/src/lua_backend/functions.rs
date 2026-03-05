//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;
use std::collections::HashMap;

use super::types::{
    LuaAnalysisCache, LuaBackend, LuaClass, LuaConstantFoldingHelper, LuaDepGraph,
    LuaDominatorTree, LuaExpr, LuaExtCache, LuaExtConfig, LuaExtConstFolder, LuaExtDepGraph,
    LuaExtDiagCollector, LuaExtDiagMsg, LuaExtDomTree, LuaExtEmitStats, LuaExtEventLog,
    LuaExtFeatures, LuaExtIdGen, LuaExtIncrKey, LuaExtLiveness, LuaExtNameScope, LuaExtPassConfig,
    LuaExtPassPhase, LuaExtPassRegistry, LuaExtPassStats, LuaExtPassTiming, LuaExtProfiler,
    LuaExtSourceBuffer, LuaExtVersion, LuaExtWorklist, LuaFunction, LuaLivenessInfo, LuaModule,
    LuaPassConfig, LuaPassPhase, LuaPassRegistry, LuaPassStats, LuaStmt, LuaTableField, LuaType,
    LuaWorklist,
};

pub(super) fn emit_stmts(stmts: &[LuaStmt], indent: usize) -> std::string::String {
    let pad = "  ".repeat(indent);
    stmts
        .iter()
        .map(|s| format!("{}{}", pad, emit_stmt(s, indent)))
        .collect::<Vec<_>>()
        .join("\n")
}
pub(super) fn emit_stmt(stmt: &LuaStmt, indent: usize) -> std::string::String {
    match stmt {
        LuaStmt::Assign { targets, values } => {
            let ts: Vec<_> = targets.iter().map(|t| t.to_string()).collect();
            let vs: Vec<_> = values.iter().map(|v| v.to_string()).collect();
            format!("{} = {}", ts.join(", "), vs.join(", "))
        }
        LuaStmt::LocalAssign {
            names,
            attribs,
            values,
        } => {
            let ns: Vec<_> = names
                .iter()
                .enumerate()
                .map(|(i, n)| {
                    if let Some(Some(attr)) = attribs.get(i) {
                        format!("{} <{}>", n, attr)
                    } else {
                        n.clone()
                    }
                })
                .collect();
            if values.is_empty() {
                format!("local {}", ns.join(", "))
            } else {
                let vs: Vec<_> = values.iter().map(|v| v.to_string()).collect();
                format!("local {} = {}", ns.join(", "), vs.join(", "))
            }
        }
        LuaStmt::Do(body) => {
            format!(
                "do\n{}\n{}end",
                emit_stmts(body, indent + 1),
                "  ".repeat(indent)
            )
        }
        LuaStmt::While { cond, body } => {
            format!(
                "while {} do\n{}\n{}end",
                cond,
                emit_stmts(body, indent + 1),
                "  ".repeat(indent)
            )
        }
        LuaStmt::Repeat { body, cond } => {
            format!(
                "repeat\n{}\n{}until {}",
                emit_stmts(body, indent + 1),
                "  ".repeat(indent),
                cond
            )
        }
        LuaStmt::If {
            cond,
            then_body,
            elseif_clauses,
            else_body,
        } => {
            let mut out = format!("if {} then\n{}", cond, emit_stmts(then_body, indent + 1));
            for (ei_cond, ei_body) in elseif_clauses {
                out.push_str(&format!(
                    "\n{}elseif {} then\n{}",
                    "  ".repeat(indent),
                    ei_cond,
                    emit_stmts(ei_body, indent + 1)
                ));
            }
            if let Some(eb) = else_body {
                out.push_str(&format!(
                    "\n{}else\n{}",
                    "  ".repeat(indent),
                    emit_stmts(eb, indent + 1)
                ));
            }
            out.push_str(&format!("\n{}end", "  ".repeat(indent)));
            out
        }
        LuaStmt::For {
            var,
            start,
            limit,
            step,
            body,
        } => {
            let step_str = step
                .as_ref()
                .map(|s| format!(", {}", s))
                .unwrap_or_default();
            format!(
                "for {} = {}, {}{} do\n{}\n{}end",
                var,
                start,
                limit,
                step_str,
                emit_stmts(body, indent + 1),
                "  ".repeat(indent)
            )
        }
        LuaStmt::ForIn { names, exprs, body } => {
            let es: Vec<_> = exprs.iter().map(|e| e.to_string()).collect();
            format!(
                "for {} in {} do\n{}\n{}end",
                names.join(", "),
                es.join(", "),
                emit_stmts(body, indent + 1),
                "  ".repeat(indent)
            )
        }
        LuaStmt::Function(func) => emit_function(func, indent, false),
        LuaStmt::Local(func) => emit_function(func, indent, true),
        LuaStmt::Return(exprs) => {
            if exprs.is_empty() {
                "return".to_string()
            } else {
                let es: Vec<_> = exprs.iter().map(|e| e.to_string()).collect();
                format!("return {}", es.join(", "))
            }
        }
        LuaStmt::Break => "break".to_string(),
        LuaStmt::Goto(label) => format!("goto {}", label),
        LuaStmt::Label(label) => format!("::{}::", label),
        LuaStmt::Call(expr) => expr.to_string(),
    }
}
pub(super) fn emit_function(
    func: &LuaFunction,
    indent: usize,
    force_local: bool,
) -> std::string::String {
    let local_kw = if force_local || func.is_local {
        "local "
    } else {
        ""
    };
    let name_part = func.name.as_deref().unwrap_or("_anon");
    let sep = if func.is_method { ":" } else { "." };
    let func_name = if func.is_method {
        if let Some(dot) = name_part.rfind('.') {
            format!("{}{}{}", &name_part[..dot], sep, &name_part[dot + 1..])
        } else {
            name_part.to_string()
        }
    } else {
        name_part.to_string()
    };
    let mut all_params = func.params.clone();
    if func.vararg {
        all_params.push("...".to_string());
    }
    format!(
        "{}function {}({})\n{}\n{}end",
        local_kw,
        func_name,
        all_params.join(", "),
        emit_stmts(&func.body, indent + 1),
        "  ".repeat(indent)
    )
}
/// Lua reserved keywords.
pub const LUA_KEYWORDS: &[&str] = &[
    "and", "break", "do", "else", "elseif", "end", "false", "for", "function", "goto", "if", "in",
    "local", "nil", "not", "or", "repeat", "return", "then", "true", "until", "while",
];
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub(super) fn test_lua_type_display_nil() {
        assert_eq!(LuaType::Nil.to_string(), "nil");
    }
    #[test]
    pub(super) fn test_lua_type_display_boolean() {
        assert_eq!(LuaType::Boolean.to_string(), "boolean");
    }
    #[test]
    pub(super) fn test_lua_type_display_number_int() {
        assert_eq!(LuaType::Number(true).to_string(), "integer");
    }
    #[test]
    pub(super) fn test_lua_type_display_number_float() {
        assert_eq!(LuaType::Number(false).to_string(), "float");
    }
    #[test]
    pub(super) fn test_lua_type_display_custom() {
        assert_eq!(
            LuaType::Custom("MyClass".to_string()).to_string(),
            "MyClass"
        );
    }
    #[test]
    pub(super) fn test_lua_expr_nil() {
        assert_eq!(LuaExpr::Nil.to_string(), "nil");
    }
    #[test]
    pub(super) fn test_lua_expr_bool() {
        assert_eq!(LuaExpr::True.to_string(), "true");
        assert_eq!(LuaExpr::False.to_string(), "false");
    }
    #[test]
    pub(super) fn test_lua_expr_int() {
        assert_eq!(LuaExpr::Int(42).to_string(), "42");
        assert_eq!(LuaExpr::Int(-7).to_string(), "-7");
    }
    #[test]
    pub(super) fn test_lua_expr_float() {
        assert_eq!(LuaExpr::Float(3.14).to_string(), "3.14");
        assert_eq!(LuaExpr::Float(1.0).to_string(), "1.0");
    }
    #[test]
    pub(super) fn test_lua_expr_str_escape() {
        let s = LuaExpr::Str("hello\nworld\"test".to_string());
        assert_eq!(s.to_string(), r#""hello\nworld\"test""#);
    }
    #[test]
    pub(super) fn test_lua_expr_var() {
        assert_eq!(LuaExpr::Var("x".to_string()).to_string(), "x");
    }
    #[test]
    pub(super) fn test_lua_expr_binop() {
        let e = LuaExpr::BinOp {
            op: "+".to_string(),
            lhs: Box::new(LuaExpr::Var("a".to_string())),
            rhs: Box::new(LuaExpr::Int(1)),
        };
        assert_eq!(e.to_string(), "(a + 1)");
    }
    #[test]
    pub(super) fn test_lua_expr_unary_not() {
        let e = LuaExpr::UnaryOp {
            op: "not".to_string(),
            operand: Box::new(LuaExpr::True),
        };
        assert_eq!(e.to_string(), "(not true)");
    }
    #[test]
    pub(super) fn test_lua_expr_call() {
        let e = LuaExpr::Call {
            func: Box::new(LuaExpr::Var("print".to_string())),
            args: vec![LuaExpr::Str("hi".to_string())],
        };
        assert_eq!(e.to_string(), "print(\"hi\")");
    }
    #[test]
    pub(super) fn test_lua_expr_method_call() {
        let e = LuaExpr::MethodCall {
            obj: Box::new(LuaExpr::Var("obj".to_string())),
            method: "greet".to_string(),
            args: vec![LuaExpr::Str("world".to_string())],
        };
        assert_eq!(e.to_string(), "obj:greet(\"world\")");
    }
    #[test]
    pub(super) fn test_lua_expr_table_constructor() {
        let e = LuaExpr::TableConstructor(vec![
            LuaTableField::NamedField("x".to_string(), LuaExpr::Int(1)),
            LuaTableField::ArrayItem(LuaExpr::Int(2)),
        ]);
        assert_eq!(e.to_string(), "{x = 1, 2}");
    }
    #[test]
    pub(super) fn test_lua_expr_index_access() {
        let e = LuaExpr::IndexAccess {
            table: Box::new(LuaExpr::Var("t".to_string())),
            key: Box::new(LuaExpr::Int(1)),
        };
        assert_eq!(e.to_string(), "t[1]");
    }
    #[test]
    pub(super) fn test_lua_expr_field_access() {
        let e = LuaExpr::FieldAccess {
            table: Box::new(LuaExpr::Var("obj".to_string())),
            field: "name".to_string(),
        };
        assert_eq!(e.to_string(), "obj.name");
    }
    #[test]
    pub(super) fn test_lua_expr_ellipsis() {
        assert_eq!(LuaExpr::Ellipsis.to_string(), "...");
    }
    #[test]
    pub(super) fn test_lua_table_field_indexed() {
        let f = LuaTableField::IndexedField(LuaExpr::Str("key".to_string()), LuaExpr::Int(99));
        assert_eq!(f.to_string(), "[\"key\"] = 99");
    }
    #[test]
    pub(super) fn test_lua_stmt_local_assign() {
        let s = LuaStmt::LocalAssign {
            names: vec!["x".to_string()],
            attribs: vec![None],
            values: vec![LuaExpr::Int(5)],
        };
        assert_eq!(s.to_string(), "local x = 5");
    }
    #[test]
    pub(super) fn test_lua_stmt_local_attrib() {
        let s = LuaStmt::LocalAssign {
            names: vec!["x".to_string()],
            attribs: vec![Some("const".to_string())],
            values: vec![LuaExpr::Int(5)],
        };
        assert_eq!(s.to_string(), "local x <const> = 5");
    }
    #[test]
    pub(super) fn test_lua_stmt_assign() {
        let s = LuaStmt::Assign {
            targets: vec![LuaExpr::Var("x".to_string())],
            values: vec![LuaExpr::Int(42)],
        };
        assert_eq!(s.to_string(), "x = 42");
    }
    #[test]
    pub(super) fn test_lua_stmt_return_empty() {
        assert_eq!(LuaStmt::Return(vec![]).to_string(), "return");
    }
    #[test]
    pub(super) fn test_lua_stmt_return_multi() {
        let s = LuaStmt::Return(vec![LuaExpr::Int(1), LuaExpr::Int(2)]);
        assert_eq!(s.to_string(), "return 1, 2");
    }
    #[test]
    pub(super) fn test_lua_stmt_break() {
        assert_eq!(LuaStmt::Break.to_string(), "break");
    }
    #[test]
    pub(super) fn test_lua_stmt_goto_label() {
        assert_eq!(
            LuaStmt::Goto("continue".to_string()).to_string(),
            "goto continue"
        );
        assert_eq!(
            LuaStmt::Label("continue".to_string()).to_string(),
            "::continue::"
        );
    }
    #[test]
    pub(super) fn test_lua_function_basic() {
        let func = LuaFunction::new(
            "add",
            vec!["a".to_string(), "b".to_string()],
            vec![LuaStmt::Return(vec![LuaExpr::BinOp {
                op: "+".to_string(),
                lhs: Box::new(LuaExpr::Var("a".to_string())),
                rhs: Box::new(LuaExpr::Var("b".to_string())),
            }])],
        );
        let s = func.to_string();
        assert!(s.contains("function add(a, b)"));
        assert!(s.contains("return (a + b)"));
        assert!(s.contains("end"));
    }
    #[test]
    pub(super) fn test_lua_function_local() {
        let func = LuaFunction::new_local(
            "helper",
            vec!["x".to_string()],
            vec![LuaStmt::Return(vec![LuaExpr::Var("x".to_string())])],
        );
        let s = format!("{}", LuaStmt::Local(func));
        assert!(s.starts_with("local function helper(x)"));
    }
    #[test]
    pub(super) fn test_lua_function_vararg() {
        let func = LuaFunction {
            name: Some("sum".to_string()),
            params: vec![],
            vararg: true,
            body: vec![LuaStmt::Return(vec![LuaExpr::Ellipsis])],
            is_local: false,
            is_method: false,
        };
        let s = func.to_string();
        assert!(s.contains("function sum(...)"));
    }
    #[test]
    pub(super) fn test_lua_class_emit() {
        let cls = LuaClass::new("Animal");
        let s = cls.emit();
        assert!(s.contains("Animal = {}"));
        assert!(s.contains("Animal.__index = Animal"));
        assert!(s.contains("function Animal:new(o)"));
    }
    #[test]
    pub(super) fn test_lua_class_with_tostring() {
        let mut cls = LuaClass::new("Dog");
        cls.tostring_body = Some(vec![LuaStmt::Return(vec![LuaExpr::Str("Dog".to_string())])]);
        let s = cls.emit();
        assert!(s.contains("Dog.__tostring"));
        assert!(s.contains("return \"Dog\""));
    }
    #[test]
    pub(super) fn test_lua_module_emit_empty() {
        let m = LuaModule::new();
        assert_eq!(m.emit(), "");
    }
    #[test]
    pub(super) fn test_lua_module_emit_with_require() {
        let mut m = LuaModule::new();
        m.requires.push(("json".to_string(), "dkjson".to_string()));
        let s = m.emit();
        assert!(s.contains("local json = require(\"dkjson\")"));
    }
    #[test]
    pub(super) fn test_lua_module_emit_main_block() {
        let mut m = LuaModule::new();
        m.main_block.push(LuaStmt::Call(LuaExpr::Call {
            func: Box::new(LuaExpr::Var("print".to_string())),
            args: vec![LuaExpr::Str("hello".to_string())],
        }));
        let s = m.emit();
        assert!(s.contains("print(\"hello\")"));
    }
    #[test]
    pub(super) fn test_mangle_name_dot() {
        let mut b = LuaBackend::new();
        assert_eq!(b.mangle_name("Nat.add"), "Nat_add");
    }
    #[test]
    pub(super) fn test_mangle_name_keyword() {
        let mut b = LuaBackend::new();
        assert_eq!(b.mangle_name("and"), "_and");
        assert_eq!(b.mangle_name("end"), "_end");
    }
    #[test]
    pub(super) fn test_mangle_name_empty() {
        let mut b = LuaBackend::new();
        assert_eq!(b.mangle_name(""), "_anon");
    }
    #[test]
    pub(super) fn test_fresh_var() {
        let mut b = LuaBackend::new();
        assert_eq!(b.fresh_var(), "_t0");
        assert_eq!(b.fresh_var(), "_t1");
    }
    #[test]
    pub(super) fn test_compile_decl_simple() {
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
        let mut b = LuaBackend::new();
        let func = b
            .compile_decl(&decl)
            .expect("func compilation should succeed");
        assert_eq!(func.name, Some("answer".to_string()));
        let s = func.to_string();
        assert!(s.contains("return 42"), "Expected return 42, got: {}", s);
    }
    #[test]
    pub(super) fn test_compile_decl_with_param() {
        let x_id = LcnfVarId(0);
        let decl = LcnfFunDecl {
            name: "identity".to_string(),
            original_name: None,
            params: vec![LcnfParam {
                id: x_id,
                name: "x".to_string(),
                ty: LcnfType::Nat,
                erased: false,
                borrowed: false,
            }],
            ret_type: LcnfType::Nat,
            body: LcnfExpr::Return(LcnfArg::Var(x_id)),
            is_recursive: false,
            is_lifted: false,
            inline_cost: 0,
        };
        let mut b = LuaBackend::new();
        let func = b
            .compile_decl(&decl)
            .expect("func compilation should succeed");
        let s = func.to_string();
        assert!(s.contains("function identity("), "Got: {}", s);
        assert!(s.contains("return"), "Got: {}", s);
    }
    #[test]
    pub(super) fn test_emit_module_multiple_decls() {
        let decl1 = LcnfFunDecl {
            name: "one".to_string(),
            original_name: None,
            params: vec![],
            ret_type: LcnfType::Nat,
            body: LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Nat(1))),
            is_recursive: false,
            is_lifted: false,
            inline_cost: 0,
        };
        let decl2 = LcnfFunDecl {
            name: "two".to_string(),
            original_name: None,
            params: vec![],
            ret_type: LcnfType::Nat,
            body: LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Nat(2))),
            is_recursive: false,
            is_lifted: false,
            inline_cost: 0,
        };
        let mut b = LuaBackend::new();
        let module = b.emit_module(&[decl1, decl2]);
        assert_eq!(module.functions.len(), 2);
        let s = module.emit();
        assert!(s.contains("function one()"));
        assert!(s.contains("function two()"));
    }
    #[test]
    pub(super) fn test_for_loop_emit() {
        let s = LuaStmt::For {
            var: "i".to_string(),
            start: LuaExpr::Int(1),
            limit: LuaExpr::Int(10),
            step: None,
            body: vec![LuaStmt::Break],
        };
        let out = s.to_string();
        assert!(out.contains("for i = 1, 10 do"));
        assert!(out.contains("break"));
        assert!(out.contains("end"));
    }
    #[test]
    pub(super) fn test_for_loop_with_step() {
        let s = LuaStmt::For {
            var: "i".to_string(),
            start: LuaExpr::Int(10),
            limit: LuaExpr::Int(1),
            step: Some(LuaExpr::Int(-1)),
            body: vec![],
        };
        let out = s.to_string();
        assert!(out.contains("for i = 10, 1, -1 do"));
    }
    #[test]
    pub(super) fn test_for_in_emit() {
        let s = LuaStmt::ForIn {
            names: vec!["k".to_string(), "v".to_string()],
            exprs: vec![LuaExpr::Call {
                func: Box::new(LuaExpr::Var("pairs".to_string())),
                args: vec![LuaExpr::Var("t".to_string())],
            }],
            body: vec![],
        };
        let out = s.to_string();
        assert!(out.contains("for k, v in pairs(t) do"));
    }
    #[test]
    pub(super) fn test_repeat_until_emit() {
        let s = LuaStmt::Repeat {
            body: vec![LuaStmt::Call(LuaExpr::Call {
                func: Box::new(LuaExpr::Var("tick".to_string())),
                args: vec![],
            })],
            cond: LuaExpr::Var("done".to_string()),
        };
        let out = s.to_string();
        assert!(out.contains("repeat"));
        assert!(out.contains("until done"));
    }
    #[test]
    pub(super) fn test_do_block_emit() {
        let s = LuaStmt::Do(vec![LuaStmt::LocalAssign {
            names: vec!["x".to_string()],
            attribs: vec![None],
            values: vec![LuaExpr::Int(1)],
        }]);
        let out = s.to_string();
        assert!(out.contains("do"));
        assert!(out.contains("local x = 1"));
        assert!(out.contains("end"));
    }
}
#[cfg(test)]
mod tests_lua_ext_extra {
    use super::*;
    #[test]
    pub(super) fn test_lua_ext_config() {
        let mut cfg = LuaExtConfig::new();
        cfg.set("mode", "release");
        cfg.set("verbose", "true");
        assert_eq!(cfg.get("mode"), Some("release"));
        assert!(cfg.get_bool("verbose"));
        assert!(cfg.get_int("mode").is_none());
        assert_eq!(cfg.len(), 2);
    }
    #[test]
    pub(super) fn test_lua_ext_source_buffer() {
        let mut buf = LuaExtSourceBuffer::new();
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
    pub(super) fn test_lua_ext_name_scope() {
        let mut scope = LuaExtNameScope::new();
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
    pub(super) fn test_lua_ext_diag_collector() {
        let mut col = LuaExtDiagCollector::new();
        col.emit(LuaExtDiagMsg::warning("pass_a", "slow"));
        col.emit(LuaExtDiagMsg::error("pass_b", "fatal"));
        assert!(col.has_errors());
        assert_eq!(col.errors().len(), 1);
        assert_eq!(col.warnings().len(), 1);
        col.clear();
        assert!(col.is_empty());
    }
    #[test]
    pub(super) fn test_lua_ext_id_gen() {
        let mut gen = LuaExtIdGen::new();
        assert_eq!(gen.next_id(), 0);
        assert_eq!(gen.next_id(), 1);
        gen.skip(10);
        assert_eq!(gen.next_id(), 12);
        gen.reset();
        assert_eq!(gen.peek_next(), 0);
    }
    #[test]
    pub(super) fn test_lua_ext_incr_key() {
        let k1 = LuaExtIncrKey::new(100, 200);
        let k2 = LuaExtIncrKey::new(100, 200);
        let k3 = LuaExtIncrKey::new(999, 200);
        assert!(k1.matches(&k2));
        assert!(!k1.matches(&k3));
    }
    #[test]
    pub(super) fn test_lua_ext_profiler() {
        let mut p = LuaExtProfiler::new();
        p.record(LuaExtPassTiming::new("pass_a", 1000, 50, 200, 100));
        p.record(LuaExtPassTiming::new("pass_b", 500, 30, 100, 200));
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
    pub(super) fn test_lua_ext_event_log() {
        let mut log = LuaExtEventLog::new(3);
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
    pub(super) fn test_lua_ext_version() {
        let v = LuaExtVersion::new(1, 2, 3).with_pre("alpha");
        assert!(!v.is_stable());
        assert_eq!(format!("{}", v), "1.2.3-alpha");
        let stable = LuaExtVersion::new(2, 0, 0);
        assert!(stable.is_stable());
        assert!(stable.is_compatible_with(&LuaExtVersion::new(2, 0, 0)));
        assert!(!stable.is_compatible_with(&LuaExtVersion::new(3, 0, 0)));
    }
    #[test]
    pub(super) fn test_lua_ext_features() {
        let mut f = LuaExtFeatures::new();
        f.enable("sse2");
        f.enable("avx2");
        assert!(f.is_enabled("sse2"));
        assert!(!f.is_enabled("avx512"));
        f.disable("avx2");
        assert!(!f.is_enabled("avx2"));
        let mut g = LuaExtFeatures::new();
        g.enable("sse2");
        g.enable("neon");
        let union = f.union(&g);
        assert!(union.is_enabled("sse2") && union.is_enabled("neon"));
        let inter = f.intersection(&g);
        assert!(inter.is_enabled("sse2"));
    }
    #[test]
    pub(super) fn test_lua_ext_emit_stats() {
        let mut s = LuaExtEmitStats::new();
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
mod Lua_infra_tests {
    use super::*;
    #[test]
    pub(super) fn test_pass_config() {
        let config = LuaPassConfig::new("test_pass", LuaPassPhase::Transformation);
        assert!(config.enabled);
        assert!(config.phase.is_modifying());
        assert_eq!(config.phase.name(), "transformation");
    }
    #[test]
    pub(super) fn test_pass_stats() {
        let mut stats = LuaPassStats::new();
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
        let mut reg = LuaPassRegistry::new();
        reg.register(LuaPassConfig::new("pass_a", LuaPassPhase::Analysis));
        reg.register(LuaPassConfig::new("pass_b", LuaPassPhase::Transformation).disabled());
        assert_eq!(reg.total_passes(), 2);
        assert_eq!(reg.enabled_count(), 1);
        reg.update_stats("pass_a", 5, 50, 2);
        let stats = reg.get_stats("pass_a").expect("stats should exist");
        assert_eq!(stats.total_changes, 5);
    }
    #[test]
    pub(super) fn test_analysis_cache() {
        let mut cache = LuaAnalysisCache::new(10);
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
        let mut wl = LuaWorklist::new();
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
        let mut dt = LuaDominatorTree::new(5);
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
        let mut liveness = LuaLivenessInfo::new(3);
        liveness.add_def(0, 1);
        liveness.add_use(1, 1);
        assert!(liveness.defs[0].contains(&1));
        assert!(liveness.uses[1].contains(&1));
    }
    #[test]
    pub(super) fn test_constant_folding() {
        assert_eq!(LuaConstantFoldingHelper::fold_add_i64(3, 4), Some(7));
        assert_eq!(LuaConstantFoldingHelper::fold_div_i64(10, 0), None);
        assert_eq!(LuaConstantFoldingHelper::fold_div_i64(10, 2), Some(5));
        assert_eq!(
            LuaConstantFoldingHelper::fold_bitand_i64(0b1100, 0b1010),
            0b1000
        );
        assert_eq!(LuaConstantFoldingHelper::fold_bitnot_i64(0), -1);
    }
    #[test]
    pub(super) fn test_dep_graph() {
        let mut g = LuaDepGraph::new();
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
mod luaext_pass_tests {
    use super::*;
    #[test]
    pub(super) fn test_luaext_phase_order() {
        assert_eq!(LuaExtPassPhase::Early.order(), 0);
        assert_eq!(LuaExtPassPhase::Middle.order(), 1);
        assert_eq!(LuaExtPassPhase::Late.order(), 2);
        assert_eq!(LuaExtPassPhase::Finalize.order(), 3);
        assert!(LuaExtPassPhase::Early.is_early());
        assert!(!LuaExtPassPhase::Early.is_late());
    }
    #[test]
    pub(super) fn test_luaext_config_builder() {
        let c = LuaExtPassConfig::new("p")
            .with_phase(LuaExtPassPhase::Late)
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
    pub(super) fn test_luaext_stats() {
        let mut s = LuaExtPassStats::new();
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
    pub(super) fn test_luaext_registry() {
        let mut r = LuaExtPassRegistry::new();
        r.register(LuaExtPassConfig::new("a").with_phase(LuaExtPassPhase::Early));
        r.register(LuaExtPassConfig::new("b").disabled());
        assert_eq!(r.len(), 2);
        assert_eq!(r.enabled_passes().len(), 1);
        assert_eq!(r.passes_in_phase(&LuaExtPassPhase::Early).len(), 1);
    }
    #[test]
    pub(super) fn test_luaext_cache() {
        let mut c = LuaExtCache::new(4);
        assert!(c.get(99).is_none());
        c.put(99, vec![1, 2, 3]);
        let v = c.get(99).expect("v should be present in map");
        assert_eq!(v, &[1u8, 2, 3]);
        assert!(c.hit_rate() > 0.0);
        assert_eq!(c.live_count(), 1);
    }
    #[test]
    pub(super) fn test_luaext_worklist() {
        let mut w = LuaExtWorklist::new(10);
        w.push(5);
        w.push(3);
        w.push(5);
        assert_eq!(w.len(), 2);
        assert!(w.contains(5));
        let first = w.pop().expect("first should be available to pop");
        assert!(!w.contains(first));
    }
    #[test]
    pub(super) fn test_luaext_dom_tree() {
        let mut dt = LuaExtDomTree::new(5);
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
    pub(super) fn test_luaext_liveness() {
        let mut lv = LuaExtLiveness::new(3);
        lv.add_def(0, 1);
        lv.add_use(1, 1);
        assert!(lv.var_is_def_in_block(0, 1));
        assert!(lv.var_is_used_in_block(1, 1));
        assert!(!lv.var_is_def_in_block(1, 1));
    }
    #[test]
    pub(super) fn test_luaext_const_folder() {
        let mut cf = LuaExtConstFolder::new();
        assert_eq!(cf.add_i64(3, 4), Some(7));
        assert_eq!(cf.div_i64(10, 0), None);
        assert_eq!(cf.mul_i64(6, 7), Some(42));
        assert_eq!(cf.and_i64(0b1100, 0b1010), 0b1000);
        assert_eq!(cf.fold_count(), 3);
        assert_eq!(cf.failure_count(), 1);
    }
    #[test]
    pub(super) fn test_luaext_dep_graph() {
        let mut g = LuaExtDepGraph::new(4);
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
