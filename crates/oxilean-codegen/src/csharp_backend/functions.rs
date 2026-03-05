//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;
use std::fmt::Write as FmtWrite;

use super::types::{
    CSharpBackend, CSharpClass, CSharpEnum, CSharpExpr, CSharpInterface, CSharpInterpolationPart,
    CSharpLit, CSharpMethod, CSharpModule, CSharpProperty, CSharpRecord, CSharpStmt,
    CSharpSwitchArm, CSharpType,
};

/// Map an LCNF type to a C# type.
pub(super) fn lcnf_type_to_csharp(ty: &LcnfType) -> CSharpType {
    match ty {
        LcnfType::Nat => CSharpType::Long,
        LcnfType::LcnfString => CSharpType::String,
        LcnfType::Unit => CSharpType::Void,
        LcnfType::Erased | LcnfType::Irrelevant => CSharpType::Object,
        LcnfType::Object => CSharpType::Object,
        LcnfType::Var(name) => CSharpType::Custom(name.clone()),
        LcnfType::Fun(params, ret) => {
            let cs_params: Vec<CSharpType> = params.iter().map(lcnf_type_to_csharp).collect();
            let cs_ret = lcnf_type_to_csharp(ret);
            CSharpType::Func(cs_params, Box::new(cs_ret))
        }
        LcnfType::Ctor(name, _args) => CSharpType::Custom(name.clone()),
    }
}
/// Emit a block of statements into a `String` buffer with indentation.
pub(super) fn emit_stmts(stmts: &[CSharpStmt], indent: &str, out: &mut std::string::String) {
    for stmt in stmts {
        emit_stmt(stmt, indent, out);
    }
}
/// Emit a single statement into a `String` buffer.
pub(super) fn emit_stmt(stmt: &CSharpStmt, indent: &str, out: &mut std::string::String) {
    let inner = format!("{}    ", indent);
    match stmt {
        CSharpStmt::Expr(expr) => {
            let _ = writeln!(out, "{}{};", indent, expr);
        }
        CSharpStmt::Assign { target, value } => {
            let _ = writeln!(out, "{}{} = {};", indent, target, value);
        }
        CSharpStmt::LocalVar {
            name,
            ty,
            init,
            is_const,
        } => {
            let kw = if *is_const { "const" } else { "var" };
            match (ty, init) {
                (Some(t), Some(v)) => {
                    let _ = writeln!(out, "{}{} {} {} = {};", indent, kw, t, name, v);
                }
                (Some(t), None) => {
                    let _ = writeln!(out, "{}{} {};", indent, t, name);
                }
                (None, Some(v)) => {
                    let _ = writeln!(out, "{}{} {} = {};", indent, kw, name, v);
                }
                (None, None) => {
                    let _ = writeln!(out, "{}{} {};", indent, kw, name);
                }
            }
        }
        CSharpStmt::Return(None) => {
            let _ = writeln!(out, "{}return;", indent);
        }
        CSharpStmt::Return(Some(expr)) => {
            let _ = writeln!(out, "{}return {};", indent, expr);
        }
        CSharpStmt::Break => {
            let _ = writeln!(out, "{}break;", indent);
        }
        CSharpStmt::Continue => {
            let _ = writeln!(out, "{}continue;", indent);
        }
        CSharpStmt::YieldBreak => {
            let _ = writeln!(out, "{}yield break;", indent);
        }
        CSharpStmt::YieldReturn(expr) => {
            let _ = writeln!(out, "{}yield return {};", indent, expr);
        }
        CSharpStmt::Throw(expr) => {
            let _ = writeln!(out, "{}throw {};", indent, expr);
        }
        CSharpStmt::If {
            cond,
            then_stmts,
            else_stmts,
        } => {
            let _ = writeln!(out, "{}if ({})", indent, cond);
            let _ = writeln!(out, "{}{{", indent);
            emit_stmts(then_stmts, &inner, out);
            let _ = writeln!(out, "{}}}", indent);
            if !else_stmts.is_empty() {
                let _ = writeln!(out, "{}else", indent);
                let _ = writeln!(out, "{}{{", indent);
                emit_stmts(else_stmts, &inner, out);
                let _ = writeln!(out, "{}}}", indent);
            }
        }
        CSharpStmt::Switch {
            expr,
            cases,
            default,
        } => {
            let _ = writeln!(out, "{}switch ({})", indent, expr);
            let _ = writeln!(out, "{}{{", indent);
            for case in cases {
                let _ = writeln!(out, "{}    case {}:", indent, case.label);
                emit_stmts(&case.stmts, &format!("{}        ", indent), out);
            }
            if !default.is_empty() {
                let _ = writeln!(out, "{}    default:", indent);
                emit_stmts(default, &format!("{}        ", indent), out);
            }
            let _ = writeln!(out, "{}}}", indent);
        }
        CSharpStmt::While { cond, body } => {
            let _ = writeln!(out, "{}while ({})", indent, cond);
            let _ = writeln!(out, "{}{{", indent);
            emit_stmts(body, &inner, out);
            let _ = writeln!(out, "{}}}", indent);
        }
        CSharpStmt::For {
            init,
            cond,
            step,
            body,
        } => {
            let init_str = match init {
                None => std::string::String::new(),
                Some(s) => stmt_to_inline_str(s),
            };
            let cond_str = cond.as_ref().map(|c| format!("{}", c)).unwrap_or_default();
            let step_str = step.as_ref().map(|s| format!("{}", s)).unwrap_or_default();
            let _ = writeln!(
                out,
                "{}for ({}; {}; {})",
                indent, init_str, cond_str, step_str
            );
            let _ = writeln!(out, "{}{{", indent);
            emit_stmts(body, &inner, out);
            let _ = writeln!(out, "{}}}", indent);
        }
        CSharpStmt::ForEach {
            var_name,
            var_ty,
            collection,
            body,
        } => {
            let ty_str = var_ty
                .as_ref()
                .map(|t| format!("{} ", t))
                .unwrap_or_else(|| "var ".to_string());
            let _ = writeln!(
                out,
                "{}foreach ({}{} in {})",
                indent, ty_str, var_name, collection
            );
            let _ = writeln!(out, "{}{{", indent);
            emit_stmts(body, &inner, out);
            let _ = writeln!(out, "{}}}", indent);
        }
        CSharpStmt::TryCatch {
            try_stmts,
            catches,
            finally_stmts,
        } => {
            let _ = writeln!(out, "{}try", indent);
            let _ = writeln!(out, "{}{{", indent);
            emit_stmts(try_stmts, &inner, out);
            let _ = writeln!(out, "{}}}", indent);
            for catch in catches {
                let _ = writeln!(
                    out,
                    "{}catch ({} {})",
                    indent, catch.exception_type, catch.var_name
                );
                let _ = writeln!(out, "{}{{", indent);
                emit_stmts(&catch.stmts, &inner, out);
                let _ = writeln!(out, "{}}}", indent);
            }
            if !finally_stmts.is_empty() {
                let _ = writeln!(out, "{}finally", indent);
                let _ = writeln!(out, "{}{{", indent);
                emit_stmts(finally_stmts, &inner, out);
                let _ = writeln!(out, "{}}}", indent);
            }
        }
        CSharpStmt::Using {
            resource,
            var_name,
            body,
        } => {
            if body.is_empty() {
                if let Some(name) = var_name {
                    let _ = writeln!(out, "{}using var {} = {};", indent, name, resource);
                } else {
                    let _ = writeln!(out, "{}using ({});", indent, resource);
                }
            } else {
                let _ = writeln!(out, "{}using ({})", indent, resource);
                let _ = writeln!(out, "{}{{", indent);
                emit_stmts(body, &inner, out);
                let _ = writeln!(out, "{}}}", indent);
            }
        }
        CSharpStmt::Lock { obj, body } => {
            let _ = writeln!(out, "{}lock ({})", indent, obj);
            let _ = writeln!(out, "{}{{", indent);
            emit_stmts(body, &inner, out);
            let _ = writeln!(out, "{}}}", indent);
        }
    }
}
/// Render a statement as a short inline string (for `for` loop init).
pub(super) fn stmt_to_inline_str(stmt: &CSharpStmt) -> std::string::String {
    match stmt {
        CSharpStmt::LocalVar {
            name,
            ty,
            init,
            is_const,
        } => {
            let kw = if *is_const { "const" } else { "var" };
            if let (Some(t), Some(v)) = (ty, init) {
                format!("{} {} {} = {}", kw, t, name, v)
            } else if let (None, Some(v)) = (ty, init) {
                format!("{} {} = {}", kw, name, v)
            } else {
                format!("{} {}", kw, name)
            }
        }
        CSharpStmt::Assign { target, value } => format!("{} = {}", target, value),
        _ => std::string::String::new(),
    }
}
/// All C# reserved keywords (sorted for binary search).
pub const CSHARP_KEYWORDS: &[&str] = &[
    "abstract",
    "add",
    "alias",
    "as",
    "ascending",
    "async",
    "await",
    "base",
    "bool",
    "break",
    "by",
    "byte",
    "case",
    "catch",
    "char",
    "checked",
    "class",
    "const",
    "continue",
    "decimal",
    "default",
    "delegate",
    "descending",
    "do",
    "double",
    "dynamic",
    "else",
    "enum",
    "equals",
    "event",
    "explicit",
    "extern",
    "false",
    "finally",
    "fixed",
    "float",
    "for",
    "foreach",
    "from",
    "get",
    "global",
    "goto",
    "group",
    "if",
    "implicit",
    "in",
    "init",
    "int",
    "interface",
    "internal",
    "into",
    "is",
    "join",
    "let",
    "lock",
    "long",
    "managed",
    "nameof",
    "namespace",
    "new",
    "notnull",
    "null",
    "object",
    "on",
    "operator",
    "orderby",
    "out",
    "override",
    "params",
    "partial",
    "private",
    "protected",
    "public",
    "readonly",
    "record",
    "ref",
    "remove",
    "required",
    "return",
    "sbyte",
    "sealed",
    "select",
    "set",
    "short",
    "sizeof",
    "stackalloc",
    "static",
    "string",
    "struct",
    "switch",
    "this",
    "throw",
    "true",
    "try",
    "typeof",
    "uint",
    "ulong",
    "unchecked",
    "unmanaged",
    "unsafe",
    "ushort",
    "using",
    "value",
    "var",
    "virtual",
    "void",
    "volatile",
    "when",
    "where",
    "while",
    "with",
    "yield",
];
/// Check if a string is a C# keyword.
pub fn is_csharp_keyword(s: &str) -> bool {
    CSHARP_KEYWORDS.contains(&s)
}
/// Minimal C# runtime helper class emitted at the end of every generated file.
pub const CSHARP_RUNTIME: &str = r#"
/// <summary>OxiLean C# Runtime helpers.</summary>
internal static class OxiLeanRt
{
    /// <summary>Called when pattern matching reaches an unreachable branch.</summary>
    public static T Unreachable<T>() =>
        throw new InvalidOperationException("OxiLean: unreachable code reached");

    /// <summary>Natural number addition (long arithmetic).</summary>
    public static long NatAdd(long a, long b) => a + b;

    /// <summary>Natural number subtraction (saturating at 0).</summary>
    public static long NatSub(long a, long b) => Math.Max(0L, a - b);

    /// <summary>Natural number multiplication.</summary>
    public static long NatMul(long a, long b) => a * b;

    /// <summary>Natural number division (truncating, 0 if divisor is 0).</summary>
    public static long NatDiv(long a, long b) => b == 0L ? 0L : a / b;

    /// <summary>Natural number modulo.</summary>
    public static long NatMod(long a, long b) => b == 0L ? a : a % b;

    /// <summary>Boolean to nat (decidable equality).</summary>
    public static long Decide(bool b) => b ? 1L : 0L;

    /// <summary>String representation of a natural number.</summary>
    public static string NatToString(long n) => n.ToString();

    /// <summary>String append.</summary>
    public static string StrAppend(string a, string b) => a + b;

    /// <summary>List.cons — prepend element to list.</summary>
    public static List<A> Cons<A>(A head, List<A> tail)
    {
        var result = new List<A> { head };
        result.AddRange(tail);
        return result;
    }

    /// <summary>List.nil — empty list.</summary>
    public static List<A> Nil<A>() => new List<A>();

    /// <summary>Option.some.</summary>
    public static A? Some<A>(A value) where A : class => value;

    /// <summary>Option.none.</summary>
    public static A? None<A>() where A : class => null;
}
"#;
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub(super) fn test_type_primitives() {
        assert_eq!(CSharpType::Int.to_string(), "int");
        assert_eq!(CSharpType::Long.to_string(), "long");
        assert_eq!(CSharpType::Double.to_string(), "double");
        assert_eq!(CSharpType::Float.to_string(), "float");
        assert_eq!(CSharpType::Bool.to_string(), "bool");
        assert_eq!(CSharpType::String.to_string(), "string");
        assert_eq!(CSharpType::Void.to_string(), "void");
        assert_eq!(CSharpType::Object.to_string(), "object");
    }
    #[test]
    pub(super) fn test_type_list() {
        let ty = CSharpType::List(Box::new(CSharpType::Int));
        assert_eq!(ty.to_string(), "List<int>");
    }
    #[test]
    pub(super) fn test_type_dict() {
        let ty = CSharpType::Dict(Box::new(CSharpType::String), Box::new(CSharpType::Int));
        assert_eq!(ty.to_string(), "Dictionary<string, int>");
    }
    #[test]
    pub(super) fn test_type_tuple() {
        let ty = CSharpType::Tuple(vec![CSharpType::Int, CSharpType::String]);
        assert_eq!(ty.to_string(), "(int, string)");
    }
    #[test]
    pub(super) fn test_type_nullable() {
        let ty = CSharpType::Nullable(Box::new(CSharpType::String));
        assert_eq!(ty.to_string(), "string?");
    }
    #[test]
    pub(super) fn test_type_task_void() {
        let ty = CSharpType::Task(Box::new(CSharpType::Void));
        assert_eq!(ty.to_string(), "Task");
    }
    #[test]
    pub(super) fn test_type_task_int() {
        let ty = CSharpType::Task(Box::new(CSharpType::Int));
        assert_eq!(ty.to_string(), "Task<int>");
    }
    #[test]
    pub(super) fn test_type_custom() {
        let ty = CSharpType::Custom("MyClass".to_string());
        assert_eq!(ty.to_string(), "MyClass");
    }
    #[test]
    pub(super) fn test_type_ienumerable() {
        let ty = CSharpType::IEnumerable(Box::new(CSharpType::Long));
        assert_eq!(ty.to_string(), "IEnumerable<long>");
    }
    #[test]
    pub(super) fn test_type_func() {
        let ty = CSharpType::Func(
            vec![CSharpType::Int, CSharpType::Int],
            Box::new(CSharpType::Bool),
        );
        assert_eq!(ty.to_string(), "Func<int, int, bool>");
    }
    #[test]
    pub(super) fn test_type_action_empty() {
        let ty = CSharpType::Action(vec![]);
        assert_eq!(ty.to_string(), "Action");
    }
    #[test]
    pub(super) fn test_lit_int() {
        assert_eq!(CSharpLit::Int(42).to_string(), "42");
        assert_eq!(CSharpLit::Int(-7).to_string(), "-7");
    }
    #[test]
    pub(super) fn test_lit_long() {
        assert_eq!(CSharpLit::Long(100).to_string(), "100L");
    }
    #[test]
    pub(super) fn test_lit_bool() {
        assert_eq!(CSharpLit::Bool(true).to_string(), "true");
        assert_eq!(CSharpLit::Bool(false).to_string(), "false");
    }
    #[test]
    pub(super) fn test_lit_null() {
        assert_eq!(CSharpLit::Null.to_string(), "null");
    }
    #[test]
    pub(super) fn test_lit_str_basic() {
        assert_eq!(CSharpLit::Str("hello".to_string()).to_string(), "\"hello\"");
    }
    #[test]
    pub(super) fn test_lit_str_escapes() {
        let s = CSharpLit::Str("hi\n\"world\"\\".to_string());
        let result = s.to_string();
        assert!(result.contains("\\n"));
        assert!(result.contains("\\\""));
        assert!(result.contains("\\\\"));
    }
    #[test]
    pub(super) fn test_lit_double() {
        assert_eq!(CSharpLit::Double(3.14).to_string(), "3.14");
        assert_eq!(CSharpLit::Double(2.0).to_string(), "2.0");
    }
    #[test]
    pub(super) fn test_lit_float() {
        assert_eq!(CSharpLit::Float(1.0).to_string(), "1.0f");
    }
    #[test]
    pub(super) fn test_expr_var() {
        let e = CSharpExpr::Var("myVar".to_string());
        assert_eq!(e.to_string(), "myVar");
    }
    #[test]
    pub(super) fn test_expr_binop() {
        let e = CSharpExpr::BinOp {
            op: "+".to_string(),
            lhs: Box::new(CSharpExpr::Lit(CSharpLit::Int(1))),
            rhs: Box::new(CSharpExpr::Lit(CSharpLit::Int(2))),
        };
        assert_eq!(e.to_string(), "(1 + 2)");
    }
    #[test]
    pub(super) fn test_expr_call() {
        let e = CSharpExpr::Call {
            callee: Box::new(CSharpExpr::Var("Foo".to_string())),
            args: vec![
                CSharpExpr::Lit(CSharpLit::Int(1)),
                CSharpExpr::Lit(CSharpLit::Int(2)),
            ],
        };
        assert_eq!(e.to_string(), "Foo(1, 2)");
    }
    #[test]
    pub(super) fn test_expr_method_call_linq() {
        let e = CSharpExpr::MethodCall {
            receiver: Box::new(CSharpExpr::Var("list".to_string())),
            method: "Where".to_string(),
            type_args: vec![],
            args: vec![CSharpExpr::Lambda {
                params: vec![("x".to_string(), None)],
                body: Box::new(CSharpExpr::BinOp {
                    op: ">".to_string(),
                    lhs: Box::new(CSharpExpr::Var("x".to_string())),
                    rhs: Box::new(CSharpExpr::Lit(CSharpLit::Int(0))),
                }),
            }],
        };
        assert!(e.to_string().contains("list.Where("));
        assert!(e.to_string().contains("x => (x > 0)"));
    }
    #[test]
    pub(super) fn test_expr_new() {
        let e = CSharpExpr::New {
            ty: CSharpType::Custom("MyClass".to_string()),
            args: vec![CSharpExpr::Lit(CSharpLit::Int(42))],
        };
        assert_eq!(e.to_string(), "new MyClass(42)");
    }
    #[test]
    pub(super) fn test_expr_lambda_single_param() {
        let e = CSharpExpr::Lambda {
            params: vec![("x".to_string(), None)],
            body: Box::new(CSharpExpr::BinOp {
                op: "*".to_string(),
                lhs: Box::new(CSharpExpr::Var("x".to_string())),
                rhs: Box::new(CSharpExpr::Lit(CSharpLit::Int(2))),
            }),
        };
        assert_eq!(e.to_string(), "x => (x * 2)");
    }
    #[test]
    pub(super) fn test_expr_lambda_multi_param() {
        let e = CSharpExpr::Lambda {
            params: vec![
                ("x".to_string(), Some(CSharpType::Int)),
                ("y".to_string(), Some(CSharpType::Int)),
            ],
            body: Box::new(CSharpExpr::BinOp {
                op: "+".to_string(),
                lhs: Box::new(CSharpExpr::Var("x".to_string())),
                rhs: Box::new(CSharpExpr::Var("y".to_string())),
            }),
        };
        assert!(e.to_string().contains("(int x, int y)"));
        assert!(e.to_string().contains("=> (x + y)"));
    }
    #[test]
    pub(super) fn test_expr_ternary() {
        let e = CSharpExpr::Ternary {
            cond: Box::new(CSharpExpr::Lit(CSharpLit::Bool(true))),
            then_expr: Box::new(CSharpExpr::Lit(CSharpLit::Int(1))),
            else_expr: Box::new(CSharpExpr::Lit(CSharpLit::Int(0))),
        };
        assert_eq!(e.to_string(), "(true ? 1 : 0)");
    }
    #[test]
    pub(super) fn test_expr_await() {
        let e = CSharpExpr::Await(Box::new(CSharpExpr::Call {
            callee: Box::new(CSharpExpr::Var("FetchAsync".to_string())),
            args: vec![],
        }));
        assert_eq!(e.to_string(), "await FetchAsync()");
    }
    #[test]
    pub(super) fn test_expr_is_pattern() {
        let e = CSharpExpr::Is {
            expr: Box::new(CSharpExpr::Var("obj".to_string())),
            pattern: "string s".to_string(),
        };
        assert_eq!(e.to_string(), "(obj is string s)");
    }
    #[test]
    pub(super) fn test_expr_as_cast() {
        let e = CSharpExpr::As {
            expr: Box::new(CSharpExpr::Var("obj".to_string())),
            ty: CSharpType::Custom("MyClass".to_string()),
        };
        assert_eq!(e.to_string(), "(obj as MyClass)");
    }
    #[test]
    pub(super) fn test_expr_switch_expression() {
        let e = CSharpExpr::SwitchExpr {
            scrutinee: Box::new(CSharpExpr::Var("x".to_string())),
            arms: vec![
                CSharpSwitchArm {
                    pattern: "1".to_string(),
                    guard: None,
                    body: CSharpExpr::Lit(CSharpLit::Str("one".to_string())),
                },
                CSharpSwitchArm {
                    pattern: "_".to_string(),
                    guard: None,
                    body: CSharpExpr::Lit(CSharpLit::Str("other".to_string())),
                },
            ],
        };
        let out = e.to_string();
        assert!(out.contains("x switch"));
        assert!(out.contains("1 =>"));
        assert!(out.contains("_ =>"));
    }
    #[test]
    pub(super) fn test_expr_nameof_typeof() {
        let e1 = CSharpExpr::NameOf("myProp".to_string());
        let e2 = CSharpExpr::TypeOf(CSharpType::Custom("MyClass".to_string()));
        assert_eq!(e1.to_string(), "nameof(myProp)");
        assert_eq!(e2.to_string(), "typeof(MyClass)");
    }
    #[test]
    pub(super) fn test_expr_collection() {
        let e = CSharpExpr::CollectionExpr(vec![
            CSharpExpr::Lit(CSharpLit::Int(1)),
            CSharpExpr::Lit(CSharpLit::Int(2)),
            CSharpExpr::Lit(CSharpLit::Int(3)),
        ]);
        assert_eq!(e.to_string(), "[1, 2, 3]");
    }
    #[test]
    pub(super) fn test_expr_default() {
        let e1 = CSharpExpr::Default(None);
        let e2 = CSharpExpr::Default(Some(CSharpType::Int));
        assert_eq!(e1.to_string(), "default");
        assert_eq!(e2.to_string(), "default(int)");
    }
    #[test]
    pub(super) fn test_record_simple() {
        let mut r = CSharpRecord::new("Point");
        r.fields.push(("X".to_string(), CSharpType::Int));
        r.fields.push(("Y".to_string(), CSharpType::Int));
        let out = r.emit("");
        assert!(
            out.contains("public record Point(int X, int Y)"),
            "got: {}",
            out
        );
    }
    #[test]
    pub(super) fn test_record_sealed() {
        let mut r = CSharpRecord::new("Token");
        r.is_sealed = true;
        r.fields.push(("Value".to_string(), CSharpType::String));
        let out = r.emit("");
        assert!(
            out.contains("public sealed record Token(string Value)"),
            "got: {}",
            out
        );
    }
    #[test]
    pub(super) fn test_record_readonly_struct() {
        let mut r = CSharpRecord::new("Vec2");
        r.is_readonly = true;
        r.fields.push(("X".to_string(), CSharpType::Double));
        r.fields.push(("Y".to_string(), CSharpType::Double));
        let out = r.emit("");
        assert!(
            out.contains("record struct Vec2(double X, double Y)"),
            "got: {}",
            out
        );
    }
    #[test]
    pub(super) fn test_record_with_methods() {
        let mut r = CSharpRecord::new("Person");
        r.fields.push(("Name".to_string(), CSharpType::String));
        let mut m = CSharpMethod::new("Greet", CSharpType::String);
        m.expr_body = Some(CSharpExpr::Interpolated(vec![
            CSharpInterpolationPart::Text("Hello, ".to_string()),
            CSharpInterpolationPart::Expr(CSharpExpr::Var("Name".to_string())),
        ]));
        r.methods.push(m);
        let out = r.emit("");
        assert!(
            out.contains("public record Person(string Name)"),
            "got: {}",
            out
        );
        assert!(out.contains("Greet"), "got: {}", out);
    }
    #[test]
    pub(super) fn test_interface_basic() {
        let mut iface = CSharpInterface::new("IFoo");
        let mut m = CSharpMethod::new("Bar", CSharpType::Int);
        m.is_abstract = true;
        iface.methods.push(m);
        let out = iface.emit("");
        assert!(out.contains("public interface IFoo"), "got: {}", out);
        assert!(out.contains("public int Bar()"), "got: {}", out);
    }
    #[test]
    pub(super) fn test_interface_with_type_params() {
        let mut iface = CSharpInterface::new("IRepository");
        iface.type_params.push("T".to_string());
        let out = iface.emit("");
        assert!(
            out.contains("public interface IRepository<T>"),
            "got: {}",
            out
        );
    }
    #[test]
    pub(super) fn test_class_basic() {
        let cls = CSharpClass::new("Foo");
        let out = cls.emit("");
        assert!(out.contains("public class Foo"), "got: {}", out);
        assert!(out.contains("{"));
        assert!(out.contains("}"));
    }
    #[test]
    pub(super) fn test_class_abstract() {
        let mut cls = CSharpClass::new("Base");
        cls.is_abstract = true;
        let out = cls.emit("");
        assert!(out.contains("public abstract class Base"), "got: {}", out);
    }
    #[test]
    pub(super) fn test_class_sealed() {
        let mut cls = CSharpClass::new("Final");
        cls.is_sealed = true;
        let out = cls.emit("");
        assert!(out.contains("public sealed class Final"), "got: {}", out);
    }
    #[test]
    pub(super) fn test_class_with_base_and_interfaces() {
        let mut cls = CSharpClass::new("Dog");
        cls.base_class = Some("Animal".to_string());
        cls.interfaces.push("IComparable".to_string());
        let out = cls.emit("");
        assert!(
            out.contains("class Dog : Animal, IComparable"),
            "got: {}",
            out
        );
    }
    #[test]
    pub(super) fn test_class_with_method() {
        let mut cls = CSharpClass::new("Calculator");
        let mut m = CSharpMethod::new("Add", CSharpType::Int);
        m.params.push(("a".to_string(), CSharpType::Int));
        m.params.push(("b".to_string(), CSharpType::Int));
        m.body.push(CSharpStmt::Return(Some(CSharpExpr::BinOp {
            op: "+".to_string(),
            lhs: Box::new(CSharpExpr::Var("a".to_string())),
            rhs: Box::new(CSharpExpr::Var("b".to_string())),
        })));
        cls.methods.push(m);
        let out = cls.emit("");
        assert!(out.contains("public int Add(int a, int b)"), "got: {}", out);
        assert!(out.contains("return (a + b)"), "got: {}", out);
    }
    #[test]
    pub(super) fn test_class_async_method() {
        let mut cls = CSharpClass::new("Fetcher");
        let mut m = CSharpMethod::new("FetchAsync", CSharpType::Task(Box::new(CSharpType::String)));
        m.is_async = true;
        m.body
            .push(CSharpStmt::Return(Some(CSharpExpr::Await(Box::new(
                CSharpExpr::Call {
                    callee: Box::new(CSharpExpr::Var("httpClient.GetStringAsync".to_string())),
                    args: vec![CSharpExpr::Lit(CSharpLit::Str(
                        "https://example.com".to_string(),
                    ))],
                },
            )))));
        cls.methods.push(m);
        let out = cls.emit("");
        assert!(
            out.contains("public async Task<string> FetchAsync()"),
            "got: {}",
            out
        );
        assert!(out.contains("await"), "got: {}", out);
    }
    #[test]
    pub(super) fn test_module_namespace() {
        let m = CSharpModule::new("OxiLean.Generated");
        let out = m.emit();
        assert!(out.contains("namespace OxiLean.Generated;"), "got: {}", out);
    }
    #[test]
    pub(super) fn test_module_nullable_enable() {
        let m = CSharpModule::new("Test");
        let out = m.emit();
        assert!(out.contains("#nullable enable"), "got: {}", out);
    }
    #[test]
    pub(super) fn test_module_using_dedup() {
        let mut m = CSharpModule::new("Test");
        m.add_using("System");
        m.add_using("System");
        m.add_using("System.Linq");
        let out = m.emit();
        assert_eq!(out.matches("using System;").count(), 1, "got: {}", out);
        assert!(out.contains("using System.Linq;"), "got: {}", out);
    }
    #[test]
    pub(super) fn test_module_contains_runtime() {
        let m = CSharpModule::new("Test");
        let out = m.emit();
        assert!(out.contains("OxiLeanRt"), "got: {}", out);
        assert!(out.contains("NatAdd"), "got: {}", out);
        assert!(out.contains("NatSub"), "got: {}", out);
        assert!(out.contains("Cons"), "got: {}", out);
    }
    #[test]
    pub(super) fn test_mangle_name_keywords() {
        for kw in &["int", "class", "namespace", "return", "void", "var"] {
            let result = CSharpBackend::mangle_name(kw);
            assert!(
                result.starts_with("ox_"),
                "keyword '{}' should be prefixed, got '{}'",
                kw,
                result
            );
        }
    }
    #[test]
    pub(super) fn test_mangle_name_digit_prefix() {
        assert_eq!(CSharpBackend::mangle_name("0abc"), "ox_0abc");
    }
    #[test]
    pub(super) fn test_mangle_name_empty() {
        assert_eq!(CSharpBackend::mangle_name(""), "ox_empty");
    }
    #[test]
    pub(super) fn test_mangle_name_special_chars() {
        assert_eq!(CSharpBackend::mangle_name("foo-bar"), "foo_bar");
        assert_eq!(CSharpBackend::mangle_name("a.b.c"), "a_b_c");
    }
    #[test]
    pub(super) fn test_mangle_name_valid() {
        assert_eq!(CSharpBackend::mangle_name("myFunc"), "myFunc");
        assert_eq!(CSharpBackend::mangle_name("_private"), "_private");
    }
    #[test]
    pub(super) fn test_fresh_var() {
        let mut backend = CSharpBackend::new();
        assert_eq!(backend.fresh_var(), "_cs0");
        assert_eq!(backend.fresh_var(), "_cs1");
        assert_eq!(backend.fresh_var(), "_cs2");
    }
    #[test]
    pub(super) fn test_compile_decl_simple() {
        let decl = LcnfFunDecl {
            name: "myFn".to_string(),
            original_name: None,
            params: vec![LcnfParam {
                id: LcnfVarId(0),
                name: "x".to_string(),
                ty: LcnfType::Nat,
                erased: false,
                borrowed: false,
            }],
            body: LcnfExpr::Return(LcnfArg::Var(LcnfVarId(0))),
            ret_type: LcnfType::Nat,
            is_recursive: false,
            is_lifted: false,
            inline_cost: 0,
        };
        let backend = CSharpBackend::new();
        let method = backend.compile_decl(&decl);
        assert_eq!(method.name, "myFn");
        assert_eq!(method.params.len(), 1);
        let out = method.emit("");
        assert!(
            out.contains("public static long myFn(long _x0)"),
            "got: {}",
            out
        );
        assert!(out.contains("return _x0"), "got: {}", out);
    }
    #[test]
    pub(super) fn test_compile_decl_string_return() {
        let decl = LcnfFunDecl {
            name: "greeting".to_string(),
            original_name: None,
            params: vec![],
            body: LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Str("hello".to_string()))),
            ret_type: LcnfType::LcnfString,
            is_recursive: false,
            is_lifted: false,
            inline_cost: 0,
        };
        let backend = CSharpBackend::new();
        let method = backend.compile_decl(&decl);
        let out = method.emit("");
        assert!(
            out.contains("public static string greeting()"),
            "got: {}",
            out
        );
        assert!(out.contains("return \"hello\""), "got: {}", out);
    }
    #[test]
    pub(super) fn test_emit_module_empty() {
        let backend = CSharpBackend::new();
        let module = backend.emit_module("OxiLean.Test", &[]);
        let out = module.emit();
        assert!(
            out.contains("OxiLean-generated C# module: OxiLean.Test"),
            "got: {}",
            out
        );
        assert!(out.contains("namespace OxiLean.Test;"), "got: {}", out);
        assert!(out.contains("using System;"), "got: {}", out);
    }
    #[test]
    pub(super) fn test_backend_default() {
        let b = CSharpBackend::default();
        assert!(b.emit_public);
        assert!(b.emit_comments);
    }
    #[test]
    pub(super) fn test_runtime_nat_ops() {
        assert!(CSHARP_RUNTIME.contains("NatAdd"));
        assert!(CSHARP_RUNTIME.contains("NatSub"));
        assert!(CSHARP_RUNTIME.contains("NatMul"));
        assert!(CSHARP_RUNTIME.contains("NatDiv"));
        assert!(CSHARP_RUNTIME.contains("NatMod"));
    }
    #[test]
    pub(super) fn test_runtime_list_ops() {
        assert!(CSHARP_RUNTIME.contains("Cons"));
        assert!(CSHARP_RUNTIME.contains("Nil"));
    }
    #[test]
    pub(super) fn test_enum_basic() {
        let mut e = CSharpEnum::new("Color");
        e.variants.push(("Red".to_string(), None));
        e.variants.push(("Green".to_string(), Some(10)));
        e.variants.push(("Blue".to_string(), None));
        let out = e.emit("");
        assert!(out.contains("public enum Color"), "got: {}", out);
        assert!(out.contains("Red,"), "got: {}", out);
        assert!(out.contains("Green = 10,"), "got: {}", out);
        assert!(out.contains("Blue,"), "got: {}", out);
    }
    #[test]
    pub(super) fn test_enum_with_underlying_type() {
        let mut e = CSharpEnum::new("Flags");
        e.underlying_type = Some(CSharpType::Int);
        e.variants.push(("None".to_string(), Some(0)));
        e.variants.push(("Read".to_string(), Some(1)));
        e.variants.push(("Write".to_string(), Some(2)));
        let out = e.emit("");
        assert!(out.contains("public enum Flags : int"), "got: {}", out);
    }
    #[test]
    pub(super) fn test_property_auto_readwrite() {
        let p = CSharpProperty::new_auto("Name", CSharpType::String);
        let out = p.emit("    ");
        assert!(
            out.contains("public string Name { get; set; }"),
            "got: {}",
            out
        );
    }
    #[test]
    pub(super) fn test_property_expr_body() {
        let mut p = CSharpProperty::new_auto("Count", CSharpType::Int);
        p.has_setter = false;
        p.expr_body = Some(CSharpExpr::Lit(CSharpLit::Int(42)));
        let out = p.emit("    ");
        assert!(out.contains("public int Count => 42"), "got: {}", out);
    }
    #[test]
    pub(super) fn test_property_init_only() {
        let mut p = CSharpProperty::new_auto("Id", CSharpType::Long);
        p.is_init_only = true;
        let out = p.emit("    ");
        assert!(out.contains("{ get; init; }"), "got: {}", out);
    }
    #[test]
    pub(super) fn test_is_keyword_true() {
        assert!(is_csharp_keyword("int"));
        assert!(is_csharp_keyword("class"));
        assert!(is_csharp_keyword("namespace"));
        assert!(is_csharp_keyword("async"));
        assert!(is_csharp_keyword("await"));
        assert!(is_csharp_keyword("record"));
        assert!(is_csharp_keyword("var"));
        assert!(is_csharp_keyword("yield"));
    }
    #[test]
    pub(super) fn test_is_keyword_false() {
        assert!(!is_csharp_keyword("myFunc"));
        assert!(!is_csharp_keyword("oxilean"));
        assert!(!is_csharp_keyword("Foo"));
    }
}
