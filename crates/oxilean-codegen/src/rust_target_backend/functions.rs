//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;
use std::collections::HashMap;

use super::types::{
    RustAnalysisCache, RustConstantFoldingHelper, RustDepGraph, RustDominatorTree, RustEnum,
    RustExpr, RustFn, RustImpl, RustItem, RustLit, RustLivenessInfo, RustModule, RustPassConfig,
    RustPassPhase, RustPassRegistry, RustPassStats, RustPattern, RustStmt, RustStruct,
    RustStructFields, RustTargetBackend, RustType, RustVariant, RustVisibility, RustWorklist,
};

pub(super) fn emit_expr(expr: &RustExpr, indent: usize) -> std::string::String {
    match expr {
        RustExpr::Lit(lit) => lit.to_string(),
        RustExpr::Var(name) => name.clone(),
        RustExpr::BinOp { op, lhs, rhs } => {
            format!(
                "({} {} {})",
                emit_expr(lhs, indent),
                op,
                emit_expr(rhs, indent)
            )
        }
        RustExpr::UnaryOp { op, operand } => {
            format!("({}{})", op, emit_expr(operand, indent))
        }
        RustExpr::Block(stmts, tail) => {
            let pad = "    ".repeat(indent);
            let inner_pad = "    ".repeat(indent + 1);
            let mut out = "{\n".to_string();
            for s in stmts {
                out.push_str(&format!("{}{}\n", inner_pad, emit_stmt(s, indent + 1)));
            }
            if let Some(te) = tail {
                out.push_str(&format!("{}{}\n", inner_pad, emit_expr(te, indent + 1)));
            }
            out.push_str(&format!("{}}}", pad));
            out
        }
        RustExpr::If {
            cond,
            then_block,
            else_block,
        } => {
            let pad = "    ".repeat(indent);
            let inner_pad = "    ".repeat(indent + 1);
            let mut out = format!("if {} {{\n", emit_expr(cond, indent));
            for s in then_block {
                out.push_str(&format!("{}{}\n", inner_pad, emit_stmt(s, indent + 1)));
            }
            out.push_str(&format!("{}}}", pad));
            if let Some(eb) = else_block {
                out.push_str(" else {\n");
                for s in eb {
                    out.push_str(&format!("{}{}\n", inner_pad, emit_stmt(s, indent + 1)));
                }
                out.push_str(&format!("{}}}", pad));
            }
            out
        }
        RustExpr::Match { scrutinee, arms } => {
            let pad = "    ".repeat(indent);
            let inner_pad = "    ".repeat(indent + 1);
            let mut out = format!("match {} {{\n", emit_expr(scrutinee, indent));
            for (pat, body) in arms {
                out.push_str(&format!(
                    "{}{} => {},\n",
                    inner_pad,
                    pat,
                    emit_expr(body, indent + 1)
                ));
            }
            out.push_str(&format!("{}}}", pad));
            out
        }
        RustExpr::Loop(body) => {
            let pad = "    ".repeat(indent);
            let inner_pad = "    ".repeat(indent + 1);
            let mut out = "loop {\n".to_string();
            for s in body {
                out.push_str(&format!("{}{}\n", inner_pad, emit_stmt(s, indent + 1)));
            }
            out.push_str(&format!("{}}}", pad));
            out
        }
        RustExpr::For { pat, iter, body } => {
            let pad = "    ".repeat(indent);
            let inner_pad = "    ".repeat(indent + 1);
            let mut out = format!("for {} in {} {{\n", pat, emit_expr(iter, indent));
            for s in body {
                out.push_str(&format!("{}{}\n", inner_pad, emit_stmt(s, indent + 1)));
            }
            out.push_str(&format!("{}}}", pad));
            out
        }
        RustExpr::While { cond, body } => {
            let pad = "    ".repeat(indent);
            let inner_pad = "    ".repeat(indent + 1);
            let mut out = format!("while {} {{\n", emit_expr(cond, indent));
            for s in body {
                out.push_str(&format!("{}{}\n", inner_pad, emit_stmt(s, indent + 1)));
            }
            out.push_str(&format!("{}}}", pad));
            out
        }
        RustExpr::Return(expr) => match expr {
            Some(e) => format!("return {}", emit_expr(e, indent)),
            None => "return".to_string(),
        },
        RustExpr::Break(expr) => match expr {
            Some(e) => format!("break {}", emit_expr(e, indent)),
            None => "break".to_string(),
        },
        RustExpr::Continue => "continue".to_string(),
        RustExpr::Closure {
            params,
            ret_ty,
            body,
            is_move,
        } => {
            let move_kw = if *is_move { "move " } else { "" };
            let params_str = params
                .iter()
                .map(|(n, ty)| {
                    if let Some(t) = ty {
                        format!("{}: {}", n, t)
                    } else {
                        n.clone()
                    }
                })
                .collect::<Vec<_>>()
                .join(", ");
            let ret_str = ret_ty
                .as_ref()
                .map(|t| format!(" -> {} ", t))
                .unwrap_or_default();
            format!(
                "{}|{}|{}{}",
                move_kw,
                params_str,
                ret_str,
                emit_expr(body, indent)
            )
        }
        RustExpr::Call(func, args) => {
            let args_str = args
                .iter()
                .map(|a| emit_expr(a, indent))
                .collect::<Vec<_>>()
                .join(", ");
            format!("{}({})", emit_expr(func, indent), args_str)
        }
        RustExpr::MethodCall {
            receiver,
            method,
            args,
        } => {
            let args_str = args
                .iter()
                .map(|a| emit_expr(a, indent))
                .collect::<Vec<_>>()
                .join(", ");
            format!("{}.{}({})", emit_expr(receiver, indent), method, args_str)
        }
        RustExpr::Field(obj, field) => format!("{}.{}", emit_expr(obj, indent), field),
        RustExpr::Index(obj, idx) => {
            format!("{}[{}]", emit_expr(obj, indent), emit_expr(idx, indent))
        }
        RustExpr::Ref(mutable, inner) => {
            if *mutable {
                format!("&mut {}", emit_expr(inner, indent))
            } else {
                format!("&{}", emit_expr(inner, indent))
            }
        }
        RustExpr::Deref(inner) => format!("*{}", emit_expr(inner, indent)),
        RustExpr::Struct(name, fields) => {
            if fields.is_empty() {
                name.clone()
            } else {
                let fstr = fields
                    .iter()
                    .map(|(n, e)| format!("{}: {}", n, emit_expr(e, indent)))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("{} {{ {} }}", name, fstr)
            }
        }
        RustExpr::Tuple(elems) => {
            let es = elems
                .iter()
                .map(|e| emit_expr(e, indent))
                .collect::<Vec<_>>()
                .join(", ");
            if elems.len() == 1 {
                format!("({},)", es)
            } else {
                format!("({})", es)
            }
        }
        RustExpr::Array(elems) => {
            let es = elems
                .iter()
                .map(|e| emit_expr(e, indent))
                .collect::<Vec<_>>()
                .join(", ");
            format!("[{}]", es)
        }
        RustExpr::Range(lo, hi, inclusive) => {
            let lo_str = lo
                .as_ref()
                .map(|e| emit_expr(e, indent))
                .unwrap_or_default();
            let hi_str = hi
                .as_ref()
                .map(|e| emit_expr(e, indent))
                .unwrap_or_default();
            let sep = if *inclusive { "..=" } else { ".." };
            format!("{}{}{}", lo_str, sep, hi_str)
        }
        RustExpr::Try(inner) => format!("{}?", emit_expr(inner, indent)),
        RustExpr::Await(inner) => format!("{}.await", emit_expr(inner, indent)),
        RustExpr::Path(segments) => segments.join("::"),
        RustExpr::MacroCall(name, args) => format!("{}!({})", name, args),
    }
}
pub(super) fn emit_stmt(stmt: &RustStmt, indent: usize) -> std::string::String {
    match stmt {
        RustStmt::Let { pat, ty, value } => {
            let ty_str = ty.as_ref().map(|t| format!(": {}", t)).unwrap_or_default();
            let val_str = value
                .as_ref()
                .map(|v| format!(" = {}", emit_expr(v, indent)))
                .unwrap_or_default();
            format!("let {}{}{};", pat, ty_str, val_str)
        }
        RustStmt::Expr(e) => format!("{};", emit_expr(e, indent)),
        RustStmt::ExprNoSemi(e) => emit_expr(e, indent),
        RustStmt::Return(e) => match e {
            Some(expr) => format!("return {};", emit_expr(expr, indent)),
            None => "return;".to_string(),
        },
        RustStmt::Break(e) => match e {
            Some(expr) => format!("break {};", emit_expr(expr, indent)),
            None => "break;".to_string(),
        },
        RustStmt::Continue => "continue;".to_string(),
    }
}
/// Rust reserved keywords.
pub const RUST_KEYWORDS: &[&str] = &[
    "as", "async", "await", "break", "const", "continue", "crate", "dyn", "else", "enum", "extern",
    "false", "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub",
    "ref", "return", "self", "Self", "static", "struct", "super", "trait", "true", "type", "union",
    "unsafe", "use", "where", "while",
];
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub(super) fn test_rust_type_primitives() {
        assert_eq!(RustType::I32.to_string(), "i32");
        assert_eq!(RustType::U64.to_string(), "u64");
        assert_eq!(RustType::F64.to_string(), "f64");
        assert_eq!(RustType::Bool.to_string(), "bool");
        assert_eq!(RustType::Unit.to_string(), "()");
        assert_eq!(RustType::Never.to_string(), "!");
    }
    #[test]
    pub(super) fn test_rust_type_string() {
        assert_eq!(RustType::RustString.to_string(), "String");
    }
    #[test]
    pub(super) fn test_rust_type_vec() {
        assert_eq!(
            RustType::Vec(Box::new(RustType::I32)).to_string(),
            "Vec<i32>"
        );
    }
    #[test]
    pub(super) fn test_rust_type_option() {
        assert_eq!(
            RustType::Option(Box::new(RustType::U64)).to_string(),
            "Option<u64>"
        );
    }
    #[test]
    pub(super) fn test_rust_type_result() {
        let r = RustType::Result(Box::new(RustType::I32), Box::new(RustType::RustString));
        assert_eq!(r.to_string(), "Result<i32, String>");
    }
    #[test]
    pub(super) fn test_rust_type_tuple() {
        let t = RustType::Tuple(vec![RustType::I32, RustType::Bool]);
        assert_eq!(t.to_string(), "(i32, bool)");
    }
    #[test]
    pub(super) fn test_rust_type_tuple_single() {
        let t = RustType::Tuple(vec![RustType::I32]);
        assert_eq!(t.to_string(), "(i32,)");
    }
    #[test]
    pub(super) fn test_rust_type_ref() {
        assert_eq!(
            RustType::Ref(false, Box::new(RustType::I32)).to_string(),
            "&i32"
        );
        assert_eq!(
            RustType::Ref(true, Box::new(RustType::I32)).to_string(),
            "&mut i32"
        );
    }
    #[test]
    pub(super) fn test_rust_type_generic() {
        let t = RustType::Generic(
            "HashMap".to_string(),
            vec![RustType::RustString, RustType::I32],
        );
        assert_eq!(t.to_string(), "HashMap<String, i32>");
    }
    #[test]
    pub(super) fn test_rust_type_fn() {
        let t = RustType::Fn(vec![RustType::I32, RustType::I32], Box::new(RustType::I32));
        assert_eq!(t.to_string(), "impl Fn(i32, i32) -> i32");
    }
    #[test]
    pub(super) fn test_rust_type_lifetime() {
        assert_eq!(RustType::Lifetime("a".to_string()).to_string(), "'a");
    }
    #[test]
    pub(super) fn test_rust_lit_int() {
        assert_eq!(RustLit::Int(42).to_string(), "42");
        assert_eq!(RustLit::Int(-7).to_string(), "-7");
    }
    #[test]
    pub(super) fn test_rust_lit_uint() {
        assert_eq!(RustLit::UInt(100).to_string(), "100");
    }
    #[test]
    pub(super) fn test_rust_lit_float() {
        assert_eq!(RustLit::Float(3.14).to_string(), "3.14");
        assert_eq!(RustLit::Float(1.0).to_string(), "1.0");
    }
    #[test]
    pub(super) fn test_rust_lit_bool() {
        assert_eq!(RustLit::Bool(true).to_string(), "true");
        assert_eq!(RustLit::Bool(false).to_string(), "false");
    }
    #[test]
    pub(super) fn test_rust_lit_str_escape() {
        let s = RustLit::Str("hello\nworld".to_string());
        assert_eq!(s.to_string(), "\"hello\\nworld\"");
    }
    #[test]
    pub(super) fn test_rust_lit_unit() {
        assert_eq!(RustLit::Unit.to_string(), "()");
    }
    #[test]
    pub(super) fn test_rust_pattern_wildcard() {
        assert_eq!(RustPattern::Wildcard.to_string(), "_");
    }
    #[test]
    pub(super) fn test_rust_pattern_var() {
        assert_eq!(RustPattern::Var("x".to_string(), false).to_string(), "x");
        assert_eq!(RustPattern::Var("x".to_string(), true).to_string(), "mut x");
    }
    #[test]
    pub(super) fn test_rust_pattern_enum() {
        let p = RustPattern::Enum(
            "Some".to_string(),
            vec![RustPattern::Var("v".to_string(), false)],
        );
        assert_eq!(p.to_string(), "Some(v)");
    }
    #[test]
    pub(super) fn test_rust_pattern_tuple() {
        let p = RustPattern::Tuple(vec![
            RustPattern::Var("a".to_string(), false),
            RustPattern::Var("b".to_string(), false),
        ]);
        assert_eq!(p.to_string(), "(a, b)");
    }
    #[test]
    pub(super) fn test_rust_pattern_or() {
        let p = RustPattern::Or(vec![
            RustPattern::Lit(RustLit::Int(1)),
            RustPattern::Lit(RustLit::Int(2)),
        ]);
        assert_eq!(p.to_string(), "1 | 2");
    }
    #[test]
    pub(super) fn test_rust_expr_lit() {
        assert_eq!(RustExpr::Lit(RustLit::Int(42)).to_string(), "42");
    }
    #[test]
    pub(super) fn test_rust_expr_var() {
        assert_eq!(RustExpr::Var("x".to_string()).to_string(), "x");
    }
    #[test]
    pub(super) fn test_rust_expr_binop() {
        let e = RustExpr::BinOp {
            op: "+".to_string(),
            lhs: Box::new(RustExpr::Var("a".to_string())),
            rhs: Box::new(RustExpr::Lit(RustLit::Int(1))),
        };
        assert_eq!(e.to_string(), "(a + 1)");
    }
    #[test]
    pub(super) fn test_rust_expr_unary() {
        let e = RustExpr::UnaryOp {
            op: "!".to_string(),
            operand: Box::new(RustExpr::Var("x".to_string())),
        };
        assert_eq!(e.to_string(), "(!x)");
    }
    #[test]
    pub(super) fn test_rust_expr_call() {
        let e = RustExpr::Call(
            Box::new(RustExpr::Var("foo".to_string())),
            vec![
                RustExpr::Lit(RustLit::Int(1)),
                RustExpr::Lit(RustLit::Int(2)),
            ],
        );
        assert_eq!(e.to_string(), "foo(1, 2)");
    }
    #[test]
    pub(super) fn test_rust_expr_method_call() {
        let e = RustExpr::MethodCall {
            receiver: Box::new(RustExpr::Var("v".to_string())),
            method: "push".to_string(),
            args: vec![RustExpr::Lit(RustLit::Int(42))],
        };
        assert_eq!(e.to_string(), "v.push(42)");
    }
    #[test]
    pub(super) fn test_rust_expr_field() {
        let e = RustExpr::Field(Box::new(RustExpr::Var("s".to_string())), "name".to_string());
        assert_eq!(e.to_string(), "s.name");
    }
    #[test]
    pub(super) fn test_rust_expr_closure() {
        let e = RustExpr::Closure {
            params: vec![("x".to_string(), Some(RustType::I32))],
            ret_ty: Some(RustType::I32),
            body: Box::new(RustExpr::Var("x".to_string())),
            is_move: false,
        };
        let s = e.to_string();
        assert!(s.contains("|x: i32|"), "Got: {}", s);
        assert!(s.contains("-> i32"), "Got: {}", s);
    }
    #[test]
    pub(super) fn test_rust_expr_macro_call() {
        let e = RustExpr::MacroCall("println".to_string(), "\"hello\"".to_string());
        assert_eq!(e.to_string(), "println!(\"hello\")");
    }
    #[test]
    pub(super) fn test_rust_expr_range() {
        let e = RustExpr::Range(
            Some(Box::new(RustExpr::Lit(RustLit::Int(0)))),
            Some(Box::new(RustExpr::Lit(RustLit::Int(10)))),
            false,
        );
        assert_eq!(e.to_string(), "0..10");
    }
    #[test]
    pub(super) fn test_rust_expr_range_inclusive() {
        let e = RustExpr::Range(
            Some(Box::new(RustExpr::Lit(RustLit::Int(1)))),
            Some(Box::new(RustExpr::Lit(RustLit::Int(5)))),
            true,
        );
        assert_eq!(e.to_string(), "1..=5");
    }
    #[test]
    pub(super) fn test_rust_expr_try() {
        let e = RustExpr::Try(Box::new(RustExpr::Call(
            Box::new(RustExpr::Var("do_thing".to_string())),
            vec![],
        )));
        assert_eq!(e.to_string(), "do_thing()?");
    }
    #[test]
    pub(super) fn test_rust_expr_path() {
        let e = RustExpr::Path(vec![
            "std".to_string(),
            "collections".to_string(),
            "HashMap".to_string(),
        ]);
        assert_eq!(e.to_string(), "std::collections::HashMap");
    }
    #[test]
    pub(super) fn test_rust_stmt_let() {
        let s = RustStmt::Let {
            pat: RustPattern::Var("x".to_string(), false),
            ty: Some(RustType::I32),
            value: Some(RustExpr::Lit(RustLit::Int(5))),
        };
        assert_eq!(s.to_string(), "let x: i32 = 5;");
    }
    #[test]
    pub(super) fn test_rust_stmt_expr() {
        let s = RustStmt::Expr(RustExpr::MacroCall(
            "println".to_string(),
            "\"hi\"".to_string(),
        ));
        assert_eq!(s.to_string(), "println!(\"hi\");");
    }
    #[test]
    pub(super) fn test_rust_fn_emit_simple() {
        let func = RustFn::new(
            "add",
            vec![
                ("a".to_string(), RustType::I32, false),
                ("b".to_string(), RustType::I32, false),
            ],
            Some(RustType::I32),
            vec![RustStmt::ExprNoSemi(RustExpr::BinOp {
                op: "+".to_string(),
                lhs: Box::new(RustExpr::Var("a".to_string())),
                rhs: Box::new(RustExpr::Var("b".to_string())),
            })],
        );
        let s = func.emit();
        assert!(s.contains("fn add(a: i32, b: i32) -> i32"), "Got: {}", s);
        assert!(s.contains("(a + b)"), "Got: {}", s);
    }
    #[test]
    pub(super) fn test_rust_fn_async_unsafe() {
        let func = RustFn {
            name: "dangerous".to_string(),
            generics: vec![],
            params: vec![],
            return_type: Some(RustType::Unit),
            body: vec![],
            attrs: vec![],
            visibility: RustVisibility::Pub,
            is_async: true,
            is_unsafe: true,
        };
        let s = func.emit();
        assert!(s.contains("async unsafe fn dangerous()"), "Got: {}", s);
    }
    #[test]
    pub(super) fn test_rust_fn_with_generics() {
        let func = RustFn {
            name: "identity".to_string(),
            generics: vec![("T".to_string(), vec!["Clone".to_string()])],
            params: vec![("x".to_string(), RustType::Custom("T".to_string()), false)],
            return_type: Some(RustType::Custom("T".to_string())),
            body: vec![RustStmt::ExprNoSemi(RustExpr::Var("x".to_string()))],
            attrs: vec![],
            visibility: RustVisibility::Pub,
            is_async: false,
            is_unsafe: false,
        };
        let s = func.emit();
        assert!(s.contains("fn identity<T: Clone>(x: T) -> T"), "Got: {}", s);
    }
    #[test]
    pub(super) fn test_rust_struct_named_fields() {
        let s = RustStruct {
            name: "Point".to_string(),
            generics: vec![],
            fields: RustStructFields::Named(vec![
                ("x".to_string(), RustType::F64, RustVisibility::Pub),
                ("y".to_string(), RustType::F64, RustVisibility::Pub),
            ]),
            attrs: vec![],
            derives: vec!["Debug".to_string(), "Clone".to_string()],
            visibility: RustVisibility::Pub,
        };
        let out = s.emit();
        assert!(out.contains("#[derive(Debug, Clone)]"), "Got: {}", out);
        assert!(out.contains("pub struct Point {"), "Got: {}", out);
        assert!(out.contains("x:f64"), "Got: {}", out);
    }
    #[test]
    pub(super) fn test_rust_struct_unit() {
        let s = RustStruct {
            name: "Sentinel".to_string(),
            generics: vec![],
            fields: RustStructFields::Unit,
            attrs: vec![],
            derives: vec![],
            visibility: RustVisibility::Pub,
        };
        let out = s.emit();
        assert_eq!(out, "pub struct Sentinel;");
    }
    #[test]
    pub(super) fn test_rust_enum_emit() {
        let e = RustEnum {
            name: "Shape".to_string(),
            generics: vec![],
            variants: vec![
                RustVariant::Unit("Circle".to_string()),
                RustVariant::Tuple("Rect".to_string(), vec![RustType::F64, RustType::F64]),
            ],
            attrs: vec![],
            derives: vec!["Debug".to_string()],
            visibility: RustVisibility::Pub,
        };
        let out = e.emit();
        assert!(out.contains("#[derive(Debug)]"), "Got: {}", out);
        assert!(out.contains("pub enum Shape {"), "Got: {}", out);
        assert!(out.contains("Circle,"), "Got: {}", out);
        assert!(out.contains("Rect(f64, f64),"), "Got: {}", out);
    }
    #[test]
    pub(super) fn test_rust_impl_emit() {
        let f = RustFn::new(
            "area",
            vec![(
                "self".to_string(),
                RustType::Custom("Self".to_string()),
                false,
            )],
            Some(RustType::F64),
            vec![],
        );
        let imp = RustImpl {
            for_type: "Circle".to_string(),
            trait_impl: None,
            generics: vec![],
            methods: vec![f],
            associated_types: vec![],
        };
        let out = imp.emit();
        assert!(out.contains("impl Circle {"), "Got: {}", out);
        assert!(out.contains("fn area"), "Got: {}", out);
    }
    #[test]
    pub(super) fn test_rust_impl_trait() {
        let imp = RustImpl {
            for_type: "MyType".to_string(),
            trait_impl: Some("Display".to_string()),
            generics: vec![],
            methods: vec![],
            associated_types: vec![],
        };
        let out = imp.emit();
        assert!(out.contains("impl Display for MyType {"), "Got: {}", out);
    }
    #[test]
    pub(super) fn test_rust_module_emit_use() {
        let mut m = RustModule::new("my_module");
        m.items.push(RustItem::Use {
            path: "std::collections::HashMap".to_string(),
            visibility: RustVisibility::Private,
        });
        let out = m.emit();
        assert!(
            out.contains("use std::collections::HashMap;"),
            "Got: {}",
            out
        );
    }
    #[test]
    pub(super) fn test_rust_module_const() {
        let mut m = RustModule::new("consts");
        m.items.push(RustItem::Const {
            name: "MAX".to_string(),
            ty: RustType::U64,
            value: RustExpr::Lit(RustLit::UInt(1000)),
            visibility: RustVisibility::Pub,
        });
        let out = m.emit();
        assert!(out.contains("pub const MAX: u64 = 1000;"), "Got: {}", out);
    }
    #[test]
    pub(super) fn test_mangle_name_keyword() {
        let mut b = RustTargetBackend::new();
        assert_eq!(b.mangle_name("type"), "type_");
        assert_eq!(b.mangle_name("fn"), "fn_");
    }
    #[test]
    pub(super) fn test_mangle_name_dot() {
        let mut b = RustTargetBackend::new();
        assert_eq!(b.mangle_name("Nat.add"), "Nat_add");
    }
    #[test]
    pub(super) fn test_mangle_name_empty() {
        let mut b = RustTargetBackend::new();
        assert_eq!(b.mangle_name(""), "_anon");
    }
    #[test]
    pub(super) fn test_fresh_var() {
        let mut b = RustTargetBackend::new();
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
        let mut b = RustTargetBackend::new();
        let func = b
            .compile_decl(&decl)
            .expect("func compilation should succeed");
        assert_eq!(func.name, "answer");
        let s = func.emit();
        assert!(s.contains("fn answer()"), "Got: {}", s);
        assert!(s.contains("-> u64"), "Got: {}", s);
        assert!(s.contains("42"), "Got: {}", s);
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
        let mut b = RustTargetBackend::new();
        let func = b
            .compile_decl(&decl)
            .expect("func compilation should succeed");
        let s = func.emit();
        assert!(s.contains("fn identity("), "Got: {}", s);
        assert!(s.contains(": u64"), "Got: {}", s);
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
        let mut b = RustTargetBackend::new();
        let module = b.emit_module("test_mod", &[decl1, decl2]);
        assert_eq!(module.items.len(), 2);
        let s = module.emit();
        assert!(s.contains("fn one()"), "Got: {}", s);
        assert!(s.contains("fn two()"), "Got: {}", s);
    }
    #[test]
    pub(super) fn test_rust_item_mod() {
        let inner_use = RustItem::Use {
            path: "super::*".to_string(),
            visibility: RustVisibility::Private,
        };
        let m = RustItem::Mod {
            name: "inner".to_string(),
            items: vec![inner_use],
            visibility: RustVisibility::Pub,
        };
        let out = m.emit();
        assert!(out.contains("pub mod inner {"), "Got: {}", out);
        assert!(out.contains("use super::*;"), "Got: {}", out);
    }
    #[test]
    pub(super) fn test_rust_item_type_alias() {
        let item = RustItem::TypeAlias {
            name: "Bytes".to_string(),
            generics: vec![],
            ty: RustType::Vec(Box::new(RustType::U8)),
            visibility: RustVisibility::Pub,
        };
        assert_eq!(item.emit(), "pub type Bytes = Vec<u8>;");
    }
    #[test]
    pub(super) fn test_rust_item_static() {
        let item = RustItem::Static {
            name: "COUNTER".to_string(),
            ty: RustType::U64,
            value: RustExpr::Lit(RustLit::UInt(0)),
            mutable: true,
            visibility: RustVisibility::Private,
        };
        assert_eq!(item.emit(), "static mut COUNTER: u64 = 0;");
    }
    #[test]
    pub(super) fn test_rust_visibility_display() {
        assert_eq!(RustVisibility::Private.to_string(), "");
        assert_eq!(RustVisibility::Pub.to_string(), "pub ");
        assert_eq!(RustVisibility::PubCrate.to_string(), "pub(crate) ");
    }
}
#[cfg(test)]
mod Rust_infra_tests {
    use super::*;
    #[test]
    pub(super) fn test_pass_config() {
        let config = RustPassConfig::new("test_pass", RustPassPhase::Transformation);
        assert!(config.enabled);
        assert!(config.phase.is_modifying());
        assert_eq!(config.phase.name(), "transformation");
    }
    #[test]
    pub(super) fn test_pass_stats() {
        let mut stats = RustPassStats::new();
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
        let mut reg = RustPassRegistry::new();
        reg.register(RustPassConfig::new("pass_a", RustPassPhase::Analysis));
        reg.register(RustPassConfig::new("pass_b", RustPassPhase::Transformation).disabled());
        assert_eq!(reg.total_passes(), 2);
        assert_eq!(reg.enabled_count(), 1);
        reg.update_stats("pass_a", 5, 50, 2);
        let stats = reg.get_stats("pass_a").expect("stats should exist");
        assert_eq!(stats.total_changes, 5);
    }
    #[test]
    pub(super) fn test_analysis_cache() {
        let mut cache = RustAnalysisCache::new(10);
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
        let mut wl = RustWorklist::new();
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
        let mut dt = RustDominatorTree::new(5);
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
        let mut liveness = RustLivenessInfo::new(3);
        liveness.add_def(0, 1);
        liveness.add_use(1, 1);
        assert!(liveness.defs[0].contains(&1));
        assert!(liveness.uses[1].contains(&1));
    }
    #[test]
    pub(super) fn test_constant_folding() {
        assert_eq!(RustConstantFoldingHelper::fold_add_i64(3, 4), Some(7));
        assert_eq!(RustConstantFoldingHelper::fold_div_i64(10, 0), None);
        assert_eq!(RustConstantFoldingHelper::fold_div_i64(10, 2), Some(5));
        assert_eq!(
            RustConstantFoldingHelper::fold_bitand_i64(0b1100, 0b1010),
            0b1000
        );
        assert_eq!(RustConstantFoldingHelper::fold_bitnot_i64(0), -1);
    }
    #[test]
    pub(super) fn test_dep_graph() {
        let mut g = RustDepGraph::new();
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
