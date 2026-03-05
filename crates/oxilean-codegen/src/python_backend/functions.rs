//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::{
    FStringPart, MatchArm, PythonBackend, PythonClass, PythonClassVar, PythonExpr, PythonFunction,
    PythonLit, PythonModule, PythonParam, PythonStmt, PythonType,
};

pub type FromImports = Vec<(String, Vec<(String, Option<String>)>)>;
pub(super) fn indent_str(level: usize) -> String {
    "    ".repeat(level)
}
pub(super) fn format_from_import(module: &str, names: &[(String, Option<String>)]) -> String {
    if names.len() == 1 && names[0].0 == "*" {
        return format!("from {} import *", module);
    }
    let names_str: Vec<String> = names
        .iter()
        .map(|(n, a)| match a {
            Some(alias) => format!("{} as {}", n, alias),
            None => n.clone(),
        })
        .collect();
    if names_str.len() > 4 {
        let joined = names_str.join(",\n    ");
        format!("from {} import (\n    {}\n)", module, joined)
    } else {
        format!("from {} import {}", module, names_str.join(", "))
    }
}
pub(super) fn emit_docstring(doc: &str, indent: usize) -> String {
    let pad = indent_str(indent);
    format!("{}\"\"\"{}\"\"\"", pad, doc)
}
pub(super) fn emit_function(func: &PythonFunction, indent: usize) -> String {
    let pad = indent_str(indent);
    let mut out = String::new();
    for dec in &func.decorators {
        out.push_str(&format!("{}@{}\n", pad, dec));
    }
    if func.is_classmethod && !func.decorators.contains(&"classmethod".to_string()) {
        out.push_str(&format!("{}@classmethod\n", pad));
    }
    if func.is_staticmethod && !func.decorators.contains(&"staticmethod".to_string()) {
        out.push_str(&format!("{}@staticmethod\n", pad));
    }
    if func.is_async {
        out.push_str(&format!("{}async def {}(", pad, func.name));
    } else {
        out.push_str(&format!("{}def {}(", pad, func.name));
    }
    let params_str: Vec<String> = func.params.iter().map(|p| p.to_string()).collect();
    out.push_str(&params_str.join(", "));
    out.push(')');
    if let Some(ret) = &func.return_type {
        out.push_str(&format!(" -> {}", ret));
    }
    out.push_str(":\n");
    if func.body.is_empty() {
        out.push_str(&format!("{}    pass\n", pad));
    } else {
        for stmt in &func.body {
            out.push_str(&emit_stmt(stmt, indent + 1));
            out.push('\n');
        }
    }
    out
}
pub(super) fn emit_class(cls: &PythonClass, indent: usize) -> String {
    let pad = indent_str(indent);
    let mut out = String::new();
    if cls.is_dataclass && !cls.decorators.contains(&"dataclass".to_string()) {
        out.push_str(&format!("{}@dataclass\n", pad));
    }
    for dec in &cls.decorators {
        out.push_str(&format!("{}@{}\n", pad, dec));
    }
    out.push_str(&format!("{}class {}", pad, cls.name));
    let mut bases = cls.bases.clone();
    if cls.is_abstract && !bases.contains(&"ABC".to_string()) {
        bases.push("ABC".to_string());
    }
    if !bases.is_empty() {
        out.push_str(&format!("({})", bases.join(", ")));
    }
    out.push_str(":\n");
    if let Some(doc) = &cls.docstring {
        out.push_str(&emit_docstring(doc, indent + 1));
        out.push('\n');
    }
    let mut body_empty = true;
    for cv in &cls.class_vars {
        body_empty = false;
        let inner_pad = indent_str(indent + 1);
        match &cv.default {
            Some(default) => {
                out.push_str(&format!(
                    "{}{}: {} = {}\n",
                    inner_pad, cv.name, cv.annotation, default
                ));
            }
            None => {
                out.push_str(&format!("{}{}: {}\n", inner_pad, cv.name, cv.annotation));
            }
        }
    }
    for method in &cls.methods {
        body_empty = false;
        out.push('\n');
        out.push_str(&emit_function(method, indent + 1));
    }
    if body_empty {
        out.push_str(&format!("{}    pass\n", pad));
    }
    out
}
pub(super) fn emit_match_arm(arm: &MatchArm, indent: usize) -> String {
    let pad = indent_str(indent);
    let mut out = String::new();
    match &arm.guard {
        Some(guard) => out.push_str(&format!("{}case {} if {}:\n", pad, arm.pattern, guard)),
        None => out.push_str(&format!("{}case {}:\n", pad, arm.pattern)),
    }
    if arm.body.is_empty() {
        out.push_str(&format!("{}    pass\n", pad));
    } else {
        for stmt in &arm.body {
            out.push_str(&emit_stmt(stmt, indent + 1));
            out.push('\n');
        }
    }
    out
}
pub(super) fn emit_stmt(stmt: &PythonStmt, indent: usize) -> String {
    let pad = indent_str(indent);
    match stmt {
        PythonStmt::Expr(e) => format!("{}{}", pad, e),
        PythonStmt::Assign(targets, value) => {
            let tgts: Vec<String> = targets.iter().map(|t| format!("{}", t)).collect();
            format!("{}{} = {}", pad, tgts.join(" = "), value)
        }
        PythonStmt::AugAssign(target, op, value) => {
            format!("{}{} {}= {}", pad, target, op, value)
        }
        PythonStmt::AnnAssign(name, ty, value) => match value {
            Some(v) => format!("{}{}: {} = {}", pad, name, ty, v),
            None => format!("{}{}: {}", pad, name, ty),
        },
        PythonStmt::If(cond, then_body, elif_branches, else_body) => {
            let mut out = format!("{}if {}:\n", pad, cond);
            if then_body.is_empty() {
                out.push_str(&format!("{}    pass\n", pad));
            } else {
                for s in then_body {
                    out.push_str(&emit_stmt(s, indent + 1));
                    out.push('\n');
                }
            }
            for (elif_cond, elif_body) in elif_branches {
                out.push_str(&format!("{}elif {}:\n", pad, elif_cond));
                if elif_body.is_empty() {
                    out.push_str(&format!("{}    pass\n", pad));
                } else {
                    for s in elif_body {
                        out.push_str(&emit_stmt(s, indent + 1));
                        out.push('\n');
                    }
                }
            }
            if !else_body.is_empty() {
                out.push_str(&format!("{}else:\n", pad));
                for s in else_body {
                    out.push_str(&emit_stmt(s, indent + 1));
                    out.push('\n');
                }
            }
            if out.ends_with('\n') {
                out.pop();
            }
            out
        }
        PythonStmt::For(var, iter, body, else_body) => {
            let mut out = format!("{}for {} in {}:\n", pad, var, iter);
            if body.is_empty() {
                out.push_str(&format!("{}    pass\n", pad));
            } else {
                for s in body {
                    out.push_str(&emit_stmt(s, indent + 1));
                    out.push('\n');
                }
            }
            if !else_body.is_empty() {
                out.push_str(&format!("{}else:\n", pad));
                for s in else_body {
                    out.push_str(&emit_stmt(s, indent + 1));
                    out.push('\n');
                }
            }
            if out.ends_with('\n') {
                out.pop();
            }
            out
        }
        PythonStmt::While(cond, body, else_body) => {
            let mut out = format!("{}while {}:\n", pad, cond);
            if body.is_empty() {
                out.push_str(&format!("{}    pass\n", pad));
            } else {
                for s in body {
                    out.push_str(&emit_stmt(s, indent + 1));
                    out.push('\n');
                }
            }
            if !else_body.is_empty() {
                out.push_str(&format!("{}else:\n", pad));
                for s in else_body {
                    out.push_str(&emit_stmt(s, indent + 1));
                    out.push('\n');
                }
            }
            if out.ends_with('\n') {
                out.pop();
            }
            out
        }
        PythonStmt::With(items, body) => {
            let items_str: Vec<String> = items
                .iter()
                .map(|(e, alias)| match alias {
                    Some(a) => format!("{} as {}", e, a),
                    None => format!("{}", e),
                })
                .collect();
            let mut out = format!("{}with {}:\n", pad, items_str.join(", "));
            if body.is_empty() {
                out.push_str(&format!("{}    pass\n", pad));
            } else {
                for s in body {
                    out.push_str(&emit_stmt(s, indent + 1));
                    out.push('\n');
                }
            }
            if out.ends_with('\n') {
                out.pop();
            }
            out
        }
        PythonStmt::Try(try_body, except_clauses, else_body, finally_body) => {
            let mut out = format!("{}try:\n", pad);
            if try_body.is_empty() {
                out.push_str(&format!("{}    pass\n", pad));
            } else {
                for s in try_body {
                    out.push_str(&emit_stmt(s, indent + 1));
                    out.push('\n');
                }
            }
            for (exc_type, exc_name, exc_body) in except_clauses {
                match (exc_type, exc_name) {
                    (Some(t), Some(n)) => out.push_str(&format!("{}except {} as {}:\n", pad, t, n)),
                    (Some(t), None) => out.push_str(&format!("{}except {}:\n", pad, t)),
                    (None, _) => out.push_str(&format!("{}except:\n", pad)),
                }
                if exc_body.is_empty() {
                    out.push_str(&format!("{}    pass\n", pad));
                } else {
                    for s in exc_body {
                        out.push_str(&emit_stmt(s, indent + 1));
                        out.push('\n');
                    }
                }
            }
            if !else_body.is_empty() {
                out.push_str(&format!("{}else:\n", pad));
                for s in else_body {
                    out.push_str(&emit_stmt(s, indent + 1));
                    out.push('\n');
                }
            }
            if !finally_body.is_empty() {
                out.push_str(&format!("{}finally:\n", pad));
                for s in finally_body {
                    out.push_str(&emit_stmt(s, indent + 1));
                    out.push('\n');
                }
            }
            if out.ends_with('\n') {
                out.pop();
            }
            out
        }
        PythonStmt::Return(expr) => match expr {
            Some(e) => format!("{}return {}", pad, e),
            None => format!("{}return", pad),
        },
        PythonStmt::Raise(expr) => match expr {
            Some(e) => format!("{}raise {}", pad, e),
            None => format!("{}raise", pad),
        },
        PythonStmt::Del(targets) => {
            let tgts: Vec<String> = targets.iter().map(|t| format!("{}", t)).collect();
            format!("{}del {}", pad, tgts.join(", "))
        }
        PythonStmt::Pass => format!("{}pass", pad),
        PythonStmt::Break => format!("{}break", pad),
        PythonStmt::Continue => format!("{}continue", pad),
        PythonStmt::Import(names) => {
            let parts: Vec<String> = names
                .iter()
                .map(|(n, a)| match a {
                    Some(alias) => format!("{} as {}", n, alias),
                    None => n.clone(),
                })
                .collect();
            format!("{}import {}", pad, parts.join(", "))
        }
        PythonStmt::From(module, names) => {
            format!("{}{}", pad, format_from_import(module, names))
        }
        PythonStmt::ClassDef(cls) => {
            let text = emit_class(cls, indent);
            text.trim_end_matches('\n').to_string()
        }
        PythonStmt::FuncDef(func) => {
            let text = emit_function(func, indent);
            text.trim_end_matches('\n').to_string()
        }
        PythonStmt::AsyncFuncDef(func) => {
            let mut f = func.clone();
            f.is_async = true;
            let text = emit_function(&f, indent);
            text.trim_end_matches('\n').to_string()
        }
        PythonStmt::Docstring(doc) => emit_docstring(doc, indent),
        PythonStmt::Assert(expr, msg) => match msg {
            Some(m) => format!("{}assert {}, {}", pad, expr, m),
            None => format!("{}assert {}", pad, expr),
        },
        PythonStmt::Global(names) => format!("{}global {}", pad, names.join(", ")),
        PythonStmt::Nonlocal(names) => format!("{}nonlocal {}", pad, names.join(", ")),
        PythonStmt::Match(subject, arms) => {
            let mut out = format!("{}match {}:\n", pad, subject);
            for arm in arms {
                out.push_str(&emit_match_arm(arm, indent + 1));
            }
            if out.ends_with('\n') {
                out.pop();
            }
            out
        }
        PythonStmt::Raw(text) => format!("{}{}", pad, text),
    }
}
/// Python 3 reserved keywords that must not be used as plain identifiers.
pub const PYTHON_KEYWORDS: &[&str] = &[
    "False", "None", "True", "and", "as", "assert", "async", "await", "break", "class", "continue",
    "def", "del", "elif", "else", "except", "finally", "for", "from", "global", "if", "import",
    "in", "is", "lambda", "nonlocal", "not", "or", "pass", "raise", "return", "try", "while",
    "with", "yield", "match", "case", "type",
];
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub(super) fn test_type_int() {
        assert_eq!(PythonType::Int.to_string(), "int");
    }
    #[test]
    pub(super) fn test_type_float() {
        assert_eq!(PythonType::Float.to_string(), "float");
    }
    #[test]
    pub(super) fn test_type_str() {
        assert_eq!(PythonType::Str.to_string(), "str");
    }
    #[test]
    pub(super) fn test_type_bool() {
        assert_eq!(PythonType::Bool.to_string(), "bool");
    }
    #[test]
    pub(super) fn test_type_none() {
        assert_eq!(PythonType::None_.to_string(), "None");
    }
    #[test]
    pub(super) fn test_type_list() {
        let t = PythonType::List(Box::new(PythonType::Int));
        assert_eq!(t.to_string(), "list[int]");
    }
    #[test]
    pub(super) fn test_type_dict() {
        let t = PythonType::Dict(Box::new(PythonType::Str), Box::new(PythonType::Int));
        assert_eq!(t.to_string(), "dict[str, int]");
    }
    #[test]
    pub(super) fn test_type_tuple() {
        let t = PythonType::Tuple(vec![PythonType::Int, PythonType::Str, PythonType::Bool]);
        assert_eq!(t.to_string(), "tuple[int, str, bool]");
    }
    #[test]
    pub(super) fn test_type_optional() {
        let t = PythonType::Optional(Box::new(PythonType::Int));
        assert_eq!(t.to_string(), "int | None");
    }
    #[test]
    pub(super) fn test_type_union() {
        let t = PythonType::Union(vec![PythonType::Int, PythonType::Str]);
        assert_eq!(t.to_string(), "int | str");
    }
    #[test]
    pub(super) fn test_type_custom() {
        let t = PythonType::Custom("MyClass".to_string());
        assert_eq!(t.to_string(), "MyClass");
    }
    #[test]
    pub(super) fn test_type_any() {
        assert_eq!(PythonType::Any.to_string(), "Any");
    }
    #[test]
    pub(super) fn test_lit_int() {
        assert_eq!(PythonLit::Int(42).to_string(), "42");
    }
    #[test]
    pub(super) fn test_lit_float() {
        assert_eq!(PythonLit::Float(3.14).to_string(), "3.14");
    }
    #[test]
    pub(super) fn test_lit_float_whole() {
        assert_eq!(PythonLit::Float(2.0).to_string(), "2.0");
    }
    #[test]
    pub(super) fn test_lit_str() {
        assert_eq!(PythonLit::Str("hello".to_string()).to_string(), "\"hello\"");
    }
    #[test]
    pub(super) fn test_lit_str_escapes() {
        let s = PythonLit::Str("a\"b\\c\nd".to_string()).to_string();
        assert!(s.contains("\\\""));
        assert!(s.contains("\\\\"));
        assert!(s.contains("\\n"));
    }
    #[test]
    pub(super) fn test_lit_bool_true() {
        assert_eq!(PythonLit::Bool(true).to_string(), "True");
    }
    #[test]
    pub(super) fn test_lit_bool_false() {
        assert_eq!(PythonLit::Bool(false).to_string(), "False");
    }
    #[test]
    pub(super) fn test_lit_none() {
        assert_eq!(PythonLit::None.to_string(), "None");
    }
    #[test]
    pub(super) fn test_lit_ellipsis() {
        assert_eq!(PythonLit::Ellipsis.to_string(), "...");
    }
    #[test]
    pub(super) fn test_expr_var() {
        assert_eq!(PythonExpr::Var("x".to_string()).to_string(), "x");
    }
    #[test]
    pub(super) fn test_expr_binop() {
        let e = PythonExpr::BinOp(
            "+".to_string(),
            Box::new(PythonExpr::Var("x".to_string())),
            Box::new(PythonExpr::Lit(PythonLit::Int(1))),
        );
        assert_eq!(e.to_string(), "(x + 1)");
    }
    #[test]
    pub(super) fn test_expr_unary_not() {
        let e = PythonExpr::UnaryOp(
            "not".to_string(),
            Box::new(PythonExpr::Var("x".to_string())),
        );
        assert_eq!(e.to_string(), "not x");
    }
    #[test]
    pub(super) fn test_expr_unary_neg() {
        let e = PythonExpr::UnaryOp("-".to_string(), Box::new(PythonExpr::Var("x".to_string())));
        assert_eq!(e.to_string(), "-x");
    }
    #[test]
    pub(super) fn test_expr_call_no_args() {
        let e = PythonExpr::Call(Box::new(PythonExpr::Var("foo".to_string())), vec![], vec![]);
        assert_eq!(e.to_string(), "foo()");
    }
    #[test]
    pub(super) fn test_expr_call_with_args() {
        let e = PythonExpr::Call(
            Box::new(PythonExpr::Var("foo".to_string())),
            vec![
                PythonExpr::Lit(PythonLit::Int(1)),
                PythonExpr::Lit(PythonLit::Int(2)),
            ],
            vec![],
        );
        assert_eq!(e.to_string(), "foo(1, 2)");
    }
    #[test]
    pub(super) fn test_expr_call_with_kwargs() {
        let e = PythonExpr::Call(
            Box::new(PythonExpr::Var("print".to_string())),
            vec![PythonExpr::Lit(PythonLit::Str("hi".to_string()))],
            vec![(
                "end".to_string(),
                PythonExpr::Lit(PythonLit::Str("".to_string())),
            )],
        );
        let s = e.to_string();
        assert!(s.contains("end=\"\""));
    }
    #[test]
    pub(super) fn test_expr_attr() {
        let e = PythonExpr::Attr(
            Box::new(PythonExpr::Var("obj".to_string())),
            "field".to_string(),
        );
        assert_eq!(e.to_string(), "obj.field");
    }
    #[test]
    pub(super) fn test_expr_subscript() {
        let e = PythonExpr::Subscript(
            Box::new(PythonExpr::Var("lst".to_string())),
            Box::new(PythonExpr::Lit(PythonLit::Int(0))),
        );
        assert_eq!(e.to_string(), "lst[0]");
    }
    #[test]
    pub(super) fn test_expr_lambda() {
        let e = PythonExpr::Lambda(
            vec!["x".to_string(), "y".to_string()],
            Box::new(PythonExpr::BinOp(
                "+".to_string(),
                Box::new(PythonExpr::Var("x".to_string())),
                Box::new(PythonExpr::Var("y".to_string())),
            )),
        );
        assert_eq!(e.to_string(), "lambda x, y: (x + y)");
    }
    #[test]
    pub(super) fn test_expr_if_expr() {
        let e = PythonExpr::IfExpr(
            Box::new(PythonExpr::Lit(PythonLit::Int(1))),
            Box::new(PythonExpr::Var("cond".to_string())),
            Box::new(PythonExpr::Lit(PythonLit::Int(0))),
        );
        assert_eq!(e.to_string(), "1 if cond else 0");
    }
    #[test]
    pub(super) fn test_expr_list_comp() {
        let e = PythonExpr::ListComp(
            Box::new(PythonExpr::BinOp(
                "*".to_string(),
                Box::new(PythonExpr::Var("x".to_string())),
                Box::new(PythonExpr::Lit(PythonLit::Int(2))),
            )),
            "x".to_string(),
            Box::new(PythonExpr::Var("lst".to_string())),
            None,
        );
        assert_eq!(e.to_string(), "[(x * 2) for x in lst]");
    }
    #[test]
    pub(super) fn test_expr_list_comp_with_filter() {
        let e = PythonExpr::ListComp(
            Box::new(PythonExpr::Var("x".to_string())),
            "x".to_string(),
            Box::new(PythonExpr::Var("lst".to_string())),
            Some(Box::new(PythonExpr::BinOp(
                ">".to_string(),
                Box::new(PythonExpr::Var("x".to_string())),
                Box::new(PythonExpr::Lit(PythonLit::Int(0))),
            ))),
        );
        assert_eq!(e.to_string(), "[x for x in lst if (x > 0)]");
    }
    #[test]
    pub(super) fn test_expr_dict_comp() {
        let e = PythonExpr::DictComp(
            Box::new(PythonExpr::Var("k".to_string())),
            Box::new(PythonExpr::Var("v".to_string())),
            "k".to_string(),
            "v".to_string(),
            Box::new(PythonExpr::Call(
                Box::new(PythonExpr::Attr(
                    Box::new(PythonExpr::Var("d".to_string())),
                    "items".to_string(),
                )),
                vec![],
                vec![],
            )),
        );
        assert_eq!(e.to_string(), "{k: v for k, v in d.items()}");
    }
    #[test]
    pub(super) fn test_expr_tuple_empty() {
        let e = PythonExpr::Tuple(vec![]);
        assert_eq!(e.to_string(), "()");
    }
    #[test]
    pub(super) fn test_expr_tuple_single() {
        let e = PythonExpr::Tuple(vec![PythonExpr::Lit(PythonLit::Int(1))]);
        assert_eq!(e.to_string(), "(1,)");
    }
    #[test]
    pub(super) fn test_expr_tuple_multi() {
        let e = PythonExpr::Tuple(vec![
            PythonExpr::Lit(PythonLit::Int(1)),
            PythonExpr::Lit(PythonLit::Int(2)),
        ]);
        assert_eq!(e.to_string(), "(1, 2)");
    }
    #[test]
    pub(super) fn test_expr_list() {
        let e = PythonExpr::List(vec![
            PythonExpr::Lit(PythonLit::Int(1)),
            PythonExpr::Lit(PythonLit::Int(2)),
            PythonExpr::Lit(PythonLit::Int(3)),
        ]);
        assert_eq!(e.to_string(), "[1, 2, 3]");
    }
    #[test]
    pub(super) fn test_expr_dict() {
        let e = PythonExpr::Dict(vec![(
            PythonExpr::Lit(PythonLit::Str("key".to_string())),
            PythonExpr::Lit(PythonLit::Int(42)),
        )]);
        assert_eq!(e.to_string(), "{\"key\": 42}");
    }
    #[test]
    pub(super) fn test_expr_set_empty() {
        let e = PythonExpr::Set(vec![]);
        assert_eq!(e.to_string(), "set()");
    }
    #[test]
    pub(super) fn test_expr_set_nonempty() {
        let e = PythonExpr::Set(vec![
            PythonExpr::Lit(PythonLit::Int(1)),
            PythonExpr::Lit(PythonLit::Int(2)),
        ]);
        assert_eq!(e.to_string(), "{1, 2}");
    }
    #[test]
    pub(super) fn test_expr_await() {
        let e = PythonExpr::Await(Box::new(PythonExpr::Var("coro".to_string())));
        assert_eq!(e.to_string(), "await coro");
    }
    #[test]
    pub(super) fn test_expr_yield() {
        let e = PythonExpr::Yield(Some(Box::new(PythonExpr::Lit(PythonLit::Int(42)))));
        assert_eq!(e.to_string(), "yield 42");
    }
    #[test]
    pub(super) fn test_expr_yield_none() {
        let e = PythonExpr::Yield(None);
        assert_eq!(e.to_string(), "yield");
    }
    #[test]
    pub(super) fn test_expr_fstring() {
        let e = PythonExpr::FString(vec![
            FStringPart::Literal("hello ".to_string()),
            FStringPart::Expr(PythonExpr::Var("name".to_string())),
            FStringPart::Literal("!".to_string()),
        ]);
        assert_eq!(e.to_string(), "f\"hello {name}!\"");
    }
    #[test]
    pub(super) fn test_expr_fstring_with_format() {
        let e = PythonExpr::FString(vec![FStringPart::ExprWithFormat(
            PythonExpr::Var("pi".to_string()),
            ".2f".to_string(),
        )]);
        assert_eq!(e.to_string(), "f\"{pi:.2f}\"");
    }
    #[test]
    pub(super) fn test_expr_walrus() {
        let e = PythonExpr::Walrus(
            "n".to_string(),
            Box::new(PythonExpr::Call(
                Box::new(PythonExpr::Var("len".to_string())),
                vec![PythonExpr::Var("lst".to_string())],
                vec![],
            )),
        );
        assert_eq!(e.to_string(), "(n := len(lst))");
    }
    #[test]
    pub(super) fn test_stmt_pass() {
        assert_eq!(emit_stmt(&PythonStmt::Pass, 0), "pass");
    }
    #[test]
    pub(super) fn test_stmt_return_none() {
        assert_eq!(emit_stmt(&PythonStmt::Return(None), 0), "return");
    }
    #[test]
    pub(super) fn test_stmt_return_expr() {
        let s = PythonStmt::Return(Some(PythonExpr::Lit(PythonLit::Int(42))));
        assert_eq!(emit_stmt(&s, 0), "return 42");
    }
    #[test]
    pub(super) fn test_stmt_assign() {
        let s = PythonStmt::Assign(
            vec![PythonExpr::Var("x".to_string())],
            PythonExpr::Lit(PythonLit::Int(5)),
        );
        assert_eq!(emit_stmt(&s, 0), "x = 5");
    }
    #[test]
    pub(super) fn test_stmt_ann_assign() {
        let s = PythonStmt::AnnAssign(
            "x".to_string(),
            PythonType::Int,
            Some(PythonExpr::Lit(PythonLit::Int(0))),
        );
        assert_eq!(emit_stmt(&s, 0), "x: int = 0");
    }
    #[test]
    pub(super) fn test_stmt_ann_assign_no_value() {
        let s = PythonStmt::AnnAssign("y".to_string(), PythonType::Str, None);
        assert_eq!(emit_stmt(&s, 0), "y: str");
    }
    #[test]
    pub(super) fn test_stmt_aug_assign() {
        let s = PythonStmt::AugAssign(
            PythonExpr::Var("x".to_string()),
            "+".to_string(),
            PythonExpr::Lit(PythonLit::Int(1)),
        );
        assert_eq!(emit_stmt(&s, 0), "x += 1");
    }
    #[test]
    pub(super) fn test_stmt_if_simple() {
        let s = PythonStmt::If(
            PythonExpr::Var("cond".to_string()),
            vec![PythonStmt::Pass],
            vec![],
            vec![],
        );
        let out = emit_stmt(&s, 0);
        assert!(out.contains("if cond:"));
        assert!(out.contains("pass"));
    }
    #[test]
    pub(super) fn test_stmt_if_else() {
        let s = PythonStmt::If(
            PythonExpr::Var("cond".to_string()),
            vec![PythonStmt::Return(Some(PythonExpr::Lit(PythonLit::Int(1))))],
            vec![],
            vec![PythonStmt::Return(Some(PythonExpr::Lit(PythonLit::Int(0))))],
        );
        let out = emit_stmt(&s, 0);
        assert!(out.contains("if cond:"));
        assert!(out.contains("else:"));
        assert!(out.contains("return 1"));
        assert!(out.contains("return 0"));
    }
    #[test]
    pub(super) fn test_stmt_for() {
        let s = PythonStmt::For(
            "x".to_string(),
            PythonExpr::Var("items".to_string()),
            vec![PythonStmt::Pass],
            vec![],
        );
        let out = emit_stmt(&s, 0);
        assert!(out.contains("for x in items:"));
    }
    #[test]
    pub(super) fn test_stmt_while() {
        let s = PythonStmt::While(
            PythonExpr::Lit(PythonLit::Bool(true)),
            vec![PythonStmt::Break],
            vec![],
        );
        let out = emit_stmt(&s, 0);
        assert!(out.contains("while True:"));
        assert!(out.contains("break"));
    }
    #[test]
    pub(super) fn test_stmt_with() {
        let s = PythonStmt::With(
            vec![(
                PythonExpr::Call(
                    Box::new(PythonExpr::Var("open".to_string())),
                    vec![PythonExpr::Lit(PythonLit::Str("f.txt".to_string()))],
                    vec![],
                ),
                Some("fh".to_string()),
            )],
            vec![PythonStmt::Pass],
        );
        let out = emit_stmt(&s, 0);
        assert!(out.contains("with open(\"f.txt\") as fh:"));
    }
    #[test]
    pub(super) fn test_stmt_try_except() {
        let s = PythonStmt::Try(
            vec![PythonStmt::Pass],
            vec![(
                Some(PythonExpr::Var("ValueError".to_string())),
                Some("e".to_string()),
                vec![PythonStmt::Pass],
            )],
            vec![],
            vec![],
        );
        let out = emit_stmt(&s, 0);
        assert!(out.contains("try:"));
        assert!(out.contains("except ValueError as e:"));
    }
    #[test]
    pub(super) fn test_stmt_raise() {
        let s = PythonStmt::Raise(Some(PythonExpr::Call(
            Box::new(PythonExpr::Var("ValueError".to_string())),
            vec![PythonExpr::Lit(PythonLit::Str("bad".to_string()))],
            vec![],
        )));
        let out = emit_stmt(&s, 0);
        assert!(out.contains("raise ValueError(\"bad\")"));
    }
    #[test]
    pub(super) fn test_stmt_import() {
        let s = PythonStmt::Import(vec![("os".to_string(), None)]);
        assert_eq!(emit_stmt(&s, 0), "import os");
    }
    #[test]
    pub(super) fn test_stmt_import_alias() {
        let s = PythonStmt::Import(vec![("numpy".to_string(), Some("np".to_string()))]);
        assert_eq!(emit_stmt(&s, 0), "import numpy as np");
    }
    #[test]
    pub(super) fn test_stmt_from_import() {
        let s = PythonStmt::From(
            "os.path".to_string(),
            vec![("join".to_string(), None), ("exists".to_string(), None)],
        );
        let out = emit_stmt(&s, 0);
        assert!(out.contains("from os.path import"));
        assert!(out.contains("join"));
        assert!(out.contains("exists"));
    }
    #[test]
    pub(super) fn test_stmt_delete() {
        let s = PythonStmt::Del(vec![PythonExpr::Var("x".to_string())]);
        assert_eq!(emit_stmt(&s, 0), "del x");
    }
    #[test]
    pub(super) fn test_stmt_assert() {
        let s = PythonStmt::Assert(
            PythonExpr::BinOp(
                "==".to_string(),
                Box::new(PythonExpr::Var("x".to_string())),
                Box::new(PythonExpr::Lit(PythonLit::Int(1))),
            ),
            None,
        );
        let out = emit_stmt(&s, 0);
        assert!(out.contains("assert (x == 1)"));
    }
    #[test]
    pub(super) fn test_stmt_global() {
        let s = PythonStmt::Global(vec!["x".to_string(), "y".to_string()]);
        assert_eq!(emit_stmt(&s, 0), "global x, y");
    }
    #[test]
    pub(super) fn test_stmt_nonlocal() {
        let s = PythonStmt::Nonlocal(vec!["count".to_string()]);
        assert_eq!(emit_stmt(&s, 0), "nonlocal count");
    }
    #[test]
    pub(super) fn test_stmt_match() {
        let s = PythonStmt::Match(
            PythonExpr::Var("cmd".to_string()),
            vec![
                MatchArm {
                    pattern: "\"quit\"".to_string(),
                    guard: None,
                    body: vec![PythonStmt::Return(None)],
                },
                MatchArm {
                    pattern: "_".to_string(),
                    guard: None,
                    body: vec![PythonStmt::Pass],
                },
            ],
        );
        let out = emit_stmt(&s, 0);
        assert!(out.contains("match cmd:"));
        assert!(out.contains("case \"quit\":"));
        assert!(out.contains("case _:"));
    }
    #[test]
    pub(super) fn test_stmt_docstring() {
        let s = PythonStmt::Docstring("This is a docstring.".to_string());
        let out = emit_stmt(&s, 0);
        assert!(out.contains("\"\"\"This is a docstring.\"\"\""));
    }
    #[test]
    pub(super) fn test_function_simple() {
        let mut func = PythonFunction::new("add");
        func.params = vec![
            PythonParam::typed("x", PythonType::Int),
            PythonParam::typed("y", PythonType::Int),
        ];
        func.return_type = Some(PythonType::Int);
        func.body = vec![PythonStmt::Return(Some(PythonExpr::BinOp(
            "+".to_string(),
            Box::new(PythonExpr::Var("x".to_string())),
            Box::new(PythonExpr::Var("y".to_string())),
        )))];
        let out = emit_function(&func, 0);
        assert!(out.contains("def add(x: int, y: int) -> int:"));
        assert!(out.contains("return (x + y)"));
    }
    #[test]
    pub(super) fn test_function_async() {
        let mut func = PythonFunction::new("fetch");
        func.params = vec![PythonParam::typed("url", PythonType::Str)];
        func.return_type = Some(PythonType::Custom("bytes".to_string()));
        func.is_async = true;
        func.body = vec![PythonStmt::Return(Some(PythonExpr::Await(Box::new(
            PythonExpr::Var("response".to_string()),
        ))))];
        let out = emit_function(&func, 0);
        assert!(out.contains("async def fetch(url: str) -> bytes:"));
        assert!(out.contains("return await response"));
    }
    #[test]
    pub(super) fn test_function_with_decorator() {
        let mut func = PythonFunction::new("value");
        func.params = vec![PythonParam::typed(
            "self",
            PythonType::Custom("Self".to_string()),
        )];
        func.return_type = Some(PythonType::Int);
        func.decorators = vec!["property".to_string()];
        func.body = vec![PythonStmt::Return(Some(PythonExpr::Attr(
            Box::new(PythonExpr::Var("self".to_string())),
            "_value".to_string(),
        )))];
        let out = emit_function(&func, 0);
        assert!(out.contains("@property"));
        assert!(out.contains("def value(self: Self) -> int:"));
    }
    #[test]
    pub(super) fn test_function_empty_body_pass() {
        let func = PythonFunction::new("noop");
        let out = emit_function(&func, 0);
        assert!(out.contains("def noop():"));
        assert!(out.contains("pass"));
    }
    #[test]
    pub(super) fn test_class_simple() {
        let cls = PythonClass::new("MyClass");
        let out = emit_class(&cls, 0);
        assert!(out.contains("class MyClass:"));
        assert!(out.contains("pass"));
    }
    #[test]
    pub(super) fn test_class_with_bases() {
        let mut cls = PythonClass::new("MyError");
        cls.bases = vec!["Exception".to_string()];
        let out = emit_class(&cls, 0);
        assert!(out.contains("class MyError(Exception):"));
    }
    #[test]
    pub(super) fn test_class_dataclass() {
        let mut cls = PythonClass::new("Point");
        cls.is_dataclass = true;
        cls.class_vars = vec![
            PythonClassVar {
                name: "x".to_string(),
                annotation: PythonType::Float,
                default: None,
            },
            PythonClassVar {
                name: "y".to_string(),
                annotation: PythonType::Float,
                default: Some(PythonExpr::Lit(PythonLit::Float(0.0))),
            },
        ];
        let out = emit_class(&cls, 0);
        assert!(out.contains("@dataclass"));
        assert!(out.contains("class Point:"));
        assert!(out.contains("x: float"));
        assert!(out.contains("y: float = 0.0"));
    }
    #[test]
    pub(super) fn test_class_with_method() {
        let mut cls = PythonClass::new("Counter");
        let mut method = PythonFunction::new("increment");
        method.params = vec![PythonParam::simple("self")];
        method.return_type = Some(PythonType::None_);
        method.body = vec![PythonStmt::AugAssign(
            PythonExpr::Attr(
                Box::new(PythonExpr::Var("self".to_string())),
                "count".to_string(),
            ),
            "+".to_string(),
            PythonExpr::Lit(PythonLit::Int(1)),
        )];
        cls.methods.push(method);
        let out = emit_class(&cls, 0);
        assert!(out.contains("class Counter:"));
        assert!(out.contains("def increment(self) -> None:"));
    }
    #[test]
    pub(super) fn test_class_abstract() {
        let mut cls = PythonClass::new("Animal");
        cls.is_abstract = true;
        let out = emit_class(&cls, 0);
        assert!(out.contains("class Animal(ABC):"));
    }
    #[test]
    pub(super) fn test_module_empty() {
        let module = PythonModule::new();
        let out = module.emit();
        assert!(out.is_empty() || !out.contains("syntax error"));
    }
    #[test]
    pub(super) fn test_module_with_imports() {
        let mut module = PythonModule::new();
        module.add_import("os", None);
        module.add_import("sys", None);
        let out = module.emit();
        assert!(out.contains("import os"));
        assert!(out.contains("import sys"));
    }
    #[test]
    pub(super) fn test_module_with_from_imports() {
        let mut module = PythonModule::new();
        module.add_from_import("pathlib", vec![("Path".to_string(), None)]);
        let out = module.emit();
        assert!(out.contains("from pathlib import Path"));
    }
    #[test]
    pub(super) fn test_module_with_all_exports() {
        let mut module = PythonModule::new();
        module.all_exports = vec!["MyClass".to_string(), "my_func".to_string()];
        let out = module.emit();
        assert!(out.contains("__all__"));
        assert!(out.contains("\"MyClass\""));
        assert!(out.contains("\"my_func\""));
    }
    #[test]
    pub(super) fn test_module_with_docstring() {
        let mut module = PythonModule::new();
        module.module_docstring = Some("My module documentation.".to_string());
        let out = module.emit();
        assert!(out.contains("\"\"\"My module documentation.\"\"\""));
    }
    #[test]
    pub(super) fn test_mangle_dots() {
        let backend = PythonBackend::new();
        assert_eq!(backend.mangle_name("Nat.add"), "Nat_add");
    }
    #[test]
    pub(super) fn test_mangle_keyword() {
        let backend = PythonBackend::new();
        assert_eq!(backend.mangle_name("class"), "_class");
    }
    #[test]
    pub(super) fn test_mangle_digit_start() {
        let backend = PythonBackend::new();
        assert_eq!(backend.mangle_name("3d"), "_3d");
    }
    #[test]
    pub(super) fn test_mangle_prime() {
        let backend = PythonBackend::new();
        assert_eq!(backend.mangle_name("f'"), "f_");
    }
    #[test]
    pub(super) fn test_mangle_empty() {
        let backend = PythonBackend::new();
        assert_eq!(backend.mangle_name(""), "_anon");
    }
    #[test]
    pub(super) fn test_param_simple() {
        let p = PythonParam::simple("x");
        assert_eq!(p.to_string(), "x");
    }
    #[test]
    pub(super) fn test_param_typed() {
        let p = PythonParam::typed("count", PythonType::Int);
        assert_eq!(p.to_string(), "count: int");
    }
    #[test]
    pub(super) fn test_param_with_default() {
        let mut p = PythonParam::typed("n", PythonType::Int);
        p.default = Some(PythonExpr::Lit(PythonLit::Int(0)));
        assert_eq!(p.to_string(), "n: int = 0");
    }
    #[test]
    pub(super) fn test_param_vararg() {
        let mut p = PythonParam::simple("args");
        p.is_vararg = true;
        assert_eq!(p.to_string(), "*args");
    }
    #[test]
    pub(super) fn test_param_kwarg() {
        let mut p = PythonParam::simple("kwargs");
        p.is_kwarg = true;
        assert_eq!(p.to_string(), "**kwargs");
    }
    #[test]
    pub(super) fn test_full_module_with_dataclass_and_function() {
        let mut module = PythonModule::new();
        module.add_import("dataclasses", None);
        module.add_from_import("dataclasses", vec![("dataclass".to_string(), None)]);
        let mut cls = PythonClass::new("Point");
        cls.is_dataclass = true;
        cls.class_vars = vec![
            PythonClassVar {
                name: "x".to_string(),
                annotation: PythonType::Float,
                default: None,
            },
            PythonClassVar {
                name: "y".to_string(),
                annotation: PythonType::Float,
                default: None,
            },
        ];
        module.add_class(cls);
        let mut func = PythonFunction::new("distance");
        func.params = vec![
            PythonParam::typed("p1", PythonType::Custom("Point".to_string())),
            PythonParam::typed("p2", PythonType::Custom("Point".to_string())),
        ];
        func.return_type = Some(PythonType::Float);
        func.body = vec![
            PythonStmt::AnnAssign(
                "dx".to_string(),
                PythonType::Float,
                Some(PythonExpr::BinOp(
                    "-".to_string(),
                    Box::new(PythonExpr::Attr(
                        Box::new(PythonExpr::Var("p1".to_string())),
                        "x".to_string(),
                    )),
                    Box::new(PythonExpr::Attr(
                        Box::new(PythonExpr::Var("p2".to_string())),
                        "x".to_string(),
                    )),
                )),
            ),
            PythonStmt::Return(Some(PythonExpr::Call(
                Box::new(PythonExpr::Attr(
                    Box::new(PythonExpr::Var("math".to_string())),
                    "sqrt".to_string(),
                )),
                vec![PythonExpr::BinOp(
                    "+".to_string(),
                    Box::new(PythonExpr::BinOp(
                        "**".to_string(),
                        Box::new(PythonExpr::Var("dx".to_string())),
                        Box::new(PythonExpr::Lit(PythonLit::Int(2))),
                    )),
                    Box::new(PythonExpr::Lit(PythonLit::Int(0))),
                )],
                vec![],
            ))),
        ];
        module.add_function(func);
        let out = module.emit();
        assert!(out.contains("@dataclass"));
        assert!(out.contains("class Point:"));
        assert!(out.contains("x: float"));
        assert!(out.contains("def distance(p1: Point, p2: Point) -> float:"));
        assert!(out.contains("dx: float = (p1.x - p2.x)"));
    }
    #[test]
    pub(super) fn test_match_with_guard() {
        let s = PythonStmt::Match(
            PythonExpr::Var("point".to_string()),
            vec![MatchArm {
                pattern: "Point(x, y)".to_string(),
                guard: Some(PythonExpr::BinOp(
                    ">".to_string(),
                    Box::new(PythonExpr::Var("x".to_string())),
                    Box::new(PythonExpr::Lit(PythonLit::Int(0))),
                )),
                body: vec![PythonStmt::Pass],
            }],
        );
        let out = emit_stmt(&s, 0);
        assert!(out.contains("match point:"));
        assert!(out.contains("case Point(x, y) if (x > 0):"));
    }
    #[test]
    pub(super) fn test_indented_function_in_class() {
        let mut cls = PythonClass::new("Foo");
        let mut method = PythonFunction::new("bar");
        method.params = vec![PythonParam::simple("self")];
        method.body = vec![PythonStmt::Return(Some(PythonExpr::Lit(PythonLit::Int(
            42,
        ))))];
        cls.methods.push(method);
        let out = emit_class(&cls, 0);
        assert!(out.contains("    def bar(self):"));
        assert!(out.contains("        return 42"));
    }
    #[test]
    pub(super) fn test_type_set() {
        let t = PythonType::Set(Box::new(PythonType::Str));
        assert_eq!(t.to_string(), "set[str]");
    }
    #[test]
    pub(super) fn test_type_iterator() {
        let t = PythonType::Iterator(Box::new(PythonType::Int));
        assert_eq!(t.to_string(), "Iterator[int]");
    }
    #[test]
    pub(super) fn test_type_callable() {
        assert_eq!(PythonType::Callable.to_string(), "Callable");
    }
    #[test]
    pub(super) fn test_expr_yield_from() {
        let e = PythonExpr::YieldFrom(Box::new(PythonExpr::Var("gen".to_string())));
        assert_eq!(e.to_string(), "yield from gen");
    }
    #[test]
    pub(super) fn test_expr_slice() {
        let e = PythonExpr::Slice(
            Some(Box::new(PythonExpr::Lit(PythonLit::Int(1)))),
            Some(Box::new(PythonExpr::Lit(PythonLit::Int(5)))),
            None,
        );
        assert_eq!(e.to_string(), "1:5");
    }
    #[test]
    pub(super) fn test_expr_slice_with_step() {
        let e = PythonExpr::Slice(
            None,
            None,
            Some(Box::new(PythonExpr::Lit(PythonLit::Int(2)))),
        );
        assert_eq!(e.to_string(), "::2");
    }
    #[test]
    pub(super) fn test_expr_set_comp() {
        let e = PythonExpr::SetComp(
            Box::new(PythonExpr::Var("x".to_string())),
            "x".to_string(),
            Box::new(PythonExpr::Var("items".to_string())),
            None,
        );
        assert_eq!(e.to_string(), "{x for x in items}");
    }
    #[test]
    pub(super) fn test_expr_gen_expr() {
        let e = PythonExpr::GenExpr(
            Box::new(PythonExpr::Var("x".to_string())),
            "x".to_string(),
            Box::new(PythonExpr::Var("nums".to_string())),
            Some(Box::new(PythonExpr::BinOp(
                ">".to_string(),
                Box::new(PythonExpr::Var("x".to_string())),
                Box::new(PythonExpr::Lit(PythonLit::Int(0))),
            ))),
        );
        assert_eq!(e.to_string(), "(x for x in nums if (x > 0))");
    }
    #[test]
    pub(super) fn test_stmt_continue_break() {
        assert_eq!(emit_stmt(&PythonStmt::Continue, 0), "continue");
        assert_eq!(emit_stmt(&PythonStmt::Break, 0), "break");
    }
    #[test]
    pub(super) fn test_stmt_indented() {
        let s = PythonStmt::Return(Some(PythonExpr::Lit(PythonLit::Int(1))));
        assert_eq!(emit_stmt(&s, 1), "    return 1");
        assert_eq!(emit_stmt(&s, 2), "        return 1");
    }
    #[test]
    pub(super) fn test_fresh_var() {
        let mut backend = PythonBackend::new();
        assert_eq!(backend.fresh_var(), "_t0");
        assert_eq!(backend.fresh_var(), "_t1");
        assert_eq!(backend.fresh_var(), "_t2");
    }
    #[test]
    pub(super) fn test_module_default() {
        let m = PythonModule::default();
        assert!(m.imports.is_empty());
        assert!(m.functions.is_empty());
        assert!(m.classes.is_empty());
    }
    #[test]
    pub(super) fn test_backend_default() {
        let b = PythonBackend::default();
        assert_eq!(b.fresh_counter, 0);
        assert!(b.fn_map.is_empty());
    }
}
