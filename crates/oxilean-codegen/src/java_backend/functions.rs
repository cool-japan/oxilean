//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;
use std::collections::HashSet;

use super::types::{
    JavaAnalysisCache, JavaBackend, JavaClass, JavaConstantFoldingHelper, JavaDepGraph,
    JavaDominatorTree, JavaEnum, JavaExpr, JavaField, JavaLit, JavaLivenessInfo, JavaMethod,
    JavaModule, JavaPassConfig, JavaPassPhase, JavaPassRegistry, JavaPassStats, JavaRecord,
    JavaStmt, JavaType, JavaWorklist, SealedInterface, Visibility,
};

/// Convert a primitive JavaType to its boxed reference type for use in generics.
pub(super) fn boxed_to_ref(ty: &JavaType) -> std::string::String {
    match ty {
        JavaType::Int => "Integer".to_string(),
        JavaType::Long => "Long".to_string(),
        JavaType::Double => "Double".to_string(),
        JavaType::Float => "Float".to_string(),
        JavaType::Boolean => "Boolean".to_string(),
        JavaType::Char => "Character".to_string(),
        JavaType::Byte => "Byte".to_string(),
        JavaType::Short => "Short".to_string(),
        _ => ty.to_string(),
    }
}
/// Map an LCNF type to a Java type.
pub(super) fn lcnf_type_to_java(ty: &LcnfType) -> JavaType {
    match ty {
        LcnfType::Nat => JavaType::Long,
        LcnfType::LcnfString => JavaType::String,
        LcnfType::Unit | LcnfType::Erased | LcnfType::Irrelevant => JavaType::Void,
        LcnfType::Object => JavaType::Object,
        LcnfType::Var(name) => JavaType::Custom(name.clone()),
        LcnfType::Fun(params, ret) => {
            if params.is_empty() {
                JavaType::Generic("Supplier".to_string(), vec![lcnf_type_to_java(ret)])
            } else if params.len() == 1 {
                JavaType::Generic(
                    "Function".to_string(),
                    vec![lcnf_type_to_java(&params[0]), lcnf_type_to_java(ret)],
                )
            } else {
                JavaType::Custom("Object".to_string())
            }
        }
        LcnfType::Ctor(name, _args) => JavaType::Custom(name.clone()),
    }
}
pub(super) fn indent(level: usize) -> std::string::String {
    "    ".repeat(level)
}
pub(super) fn emit_annotations(
    out: &mut std::string::String,
    annotations: &[std::string::String],
    level: usize,
) {
    for ann in annotations {
        out.push_str(&format!("{}{}\n", indent(level), ann));
    }
}
pub(super) fn emit_sealed_interface(
    out: &mut std::string::String,
    iface: &SealedInterface,
    level: usize,
) {
    emit_annotations(out, &iface.annotations, level);
    let ind = indent(level);
    out.push_str(&format!("{}public sealed interface {}", ind, iface.name));
    if !iface.extends.is_empty() {
        out.push_str(" extends ");
        out.push_str(&iface.extends.join(", "));
    }
    if !iface.permits.is_empty() {
        out.push_str(" permits ");
        out.push_str(&iface.permits.join(", "));
    }
    out.push_str(" {\n");
    for method in &iface.methods {
        emit_method(out, method, level + 1, true);
    }
    out.push_str(&format!("{}}}\n", ind));
}
pub(super) fn emit_record(out: &mut std::string::String, rec: &JavaRecord, level: usize) {
    emit_annotations(out, &rec.annotations, level);
    let ind = indent(level);
    out.push_str(&format!("{}public record {}", ind, rec.name));
    out.push('(');
    for (i, (name, ty)) in rec.components.iter().enumerate() {
        if i > 0 {
            out.push_str(", ");
        }
        out.push_str(&format!("{} {}", ty, name));
    }
    out.push(')');
    if !rec.implements.is_empty() {
        out.push_str(" implements ");
        out.push_str(&rec.implements.join(", "));
    }
    if rec.methods.is_empty() {
        out.push_str(" {}\n");
    } else {
        out.push_str(" {\n");
        for method in &rec.methods {
            emit_method(out, method, level + 1, false);
        }
        out.push_str(&format!("{}}}\n", ind));
    }
}
pub(super) fn emit_enum(out: &mut std::string::String, en: &JavaEnum, level: usize) {
    emit_annotations(out, &en.annotations, level);
    let ind = indent(level);
    let vis = match en.visibility {
        Visibility::Package => std::string::String::new(),
        ref v => format!("{} ", v),
    };
    out.push_str(&format!("{}{}enum {}", ind, vis, en.name));
    if !en.interfaces.is_empty() {
        out.push_str(" implements ");
        out.push_str(&en.interfaces.join(", "));
    }
    out.push_str(" {\n");
    for (i, constant) in en.constants.iter().enumerate() {
        emit_annotations(out, &constant.annotations, level + 1);
        out.push_str(&format!("{}{}", indent(level + 1), constant.name));
        if !constant.args.is_empty() {
            out.push('(');
            for (j, arg) in constant.args.iter().enumerate() {
                if j > 0 {
                    out.push_str(", ");
                }
                out.push_str(&format!("{}", arg));
            }
            out.push(')');
        }
        if i + 1 < en.constants.len() {
            out.push(',');
        } else {
            out.push(';');
        }
        out.push('\n');
    }
    if !en.fields.is_empty() || !en.methods.is_empty() {
        out.push('\n');
        for field in &en.fields {
            emit_field(out, field, level + 1);
        }
        for method in &en.methods {
            emit_method(out, method, level + 1, false);
        }
    }
    out.push_str(&format!("{}}}\n", ind));
}
pub(super) fn emit_class(out: &mut std::string::String, cls: &JavaClass, level: usize) {
    emit_annotations(out, &cls.annotations, level);
    let ind = indent(level);
    let vis = match cls.visibility {
        Visibility::Package => std::string::String::new(),
        ref v => format!("{} ", v),
    };
    out.push_str(&format!("{}{}", ind, vis));
    for m in &cls.modifiers {
        out.push_str(&format!("{} ", m));
    }
    out.push_str(&format!("class {}", cls.name));
    if !cls.type_params.is_empty() {
        out.push('<');
        out.push_str(&cls.type_params.join(", "));
        out.push('>');
    }
    if let Some(sup) = &cls.superclass {
        out.push_str(&format!(" extends {}", sup));
    }
    if !cls.interfaces.is_empty() {
        out.push_str(" implements ");
        out.push_str(&cls.interfaces.join(", "));
    }
    if !cls.permits.is_empty() {
        out.push_str(" permits ");
        out.push_str(&cls.permits.join(", "));
    }
    out.push_str(" {\n");
    for field in &cls.fields {
        emit_field(out, field, level + 1);
    }
    if !cls.fields.is_empty() {
        out.push('\n');
    }
    for method in &cls.methods {
        emit_method(out, method, level + 1, false);
    }
    for inner in &cls.inner_classes {
        out.push('\n');
        emit_class(out, inner, level + 1);
    }
    out.push_str(&format!("{}}}\n", ind));
}
pub(super) fn emit_field(out: &mut std::string::String, field: &JavaField, level: usize) {
    emit_annotations(out, &field.annotations, level);
    let ind = indent(level);
    let vis = match field.visibility {
        Visibility::Package => std::string::String::new(),
        ref v => format!("{} ", v),
    };
    let static_kw = if field.is_static { "static " } else { "" };
    let final_kw = if field.is_final { "final " } else { "" };
    if let Some(init) = &field.init {
        out.push_str(&format!(
            "{}{}{}{}{} {} = {};\n",
            ind, vis, static_kw, final_kw, field.ty, field.name, init
        ));
    } else {
        out.push_str(&format!(
            "{}{}{}{}{} {};\n",
            ind, vis, static_kw, final_kw, field.ty, field.name
        ));
    }
}
pub(super) fn emit_method(
    out: &mut std::string::String,
    method: &JavaMethod,
    level: usize,
    in_interface: bool,
) {
    emit_annotations(out, &method.annotations, level);
    let ind = indent(level);
    let vis = match method.visibility {
        Visibility::Package => std::string::String::new(),
        ref v => format!("{} ", v),
    };
    let static_kw = if method.is_static { "static " } else { "" };
    let final_kw = if method.is_final && !in_interface {
        "final "
    } else {
        ""
    };
    let abstract_kw = if method.is_abstract { "abstract " } else { "" };
    out.push_str(&format!(
        "{}{}{}{}{}{} {}(",
        ind, vis, static_kw, final_kw, abstract_kw, method.return_type, method.name
    ));
    for (i, (pname, pty)) in method.params.iter().enumerate() {
        if i > 0 {
            out.push_str(", ");
        }
        out.push_str(&format!("{} {}", pty, pname));
    }
    out.push(')');
    if !method.throws.is_empty() {
        out.push_str(" throws ");
        out.push_str(&method.throws.join(", "));
    }
    if method.is_abstract || (in_interface && method.body.is_empty()) {
        out.push_str(";\n");
        return;
    }
    out.push_str(" {\n");
    for stmt in &method.body {
        emit_stmt(out, stmt, level + 1);
    }
    out.push_str(&format!("{}}}\n", ind));
}
pub(super) fn emit_stmt(out: &mut std::string::String, stmt: &JavaStmt, level: usize) {
    let ind = indent(level);
    match stmt {
        JavaStmt::Expr(expr) => {
            out.push_str(&format!("{}{};\n", ind, expr));
        }
        JavaStmt::LocalVar {
            ty,
            name,
            init,
            is_final,
        } => {
            let final_kw = if *is_final { "final " } else { "" };
            let type_str = match ty {
                Some(t) => format!("{}", t),
                None => "var".to_string(),
            };
            match init {
                Some(expr) => {
                    out.push_str(&format!(
                        "{}{}{} {} = {};\n",
                        ind, final_kw, type_str, name, expr
                    ));
                }
                None => {
                    out.push_str(&format!("{}{}{} {};\n", ind, final_kw, type_str, name));
                }
            }
        }
        JavaStmt::If(cond, then_body, else_body) => {
            out.push_str(&format!("{}if ({}) {{\n", ind, cond));
            for s in then_body {
                emit_stmt(out, s, level + 1);
            }
            if else_body.is_empty() {
                out.push_str(&format!("{}}}\n", ind));
            } else {
                out.push_str(&format!("{}}} else {{\n", ind));
                for s in else_body {
                    emit_stmt(out, s, level + 1);
                }
                out.push_str(&format!("{}}}\n", ind));
            }
        }
        JavaStmt::Switch {
            scrutinee,
            cases,
            default,
        } => {
            out.push_str(&format!("{}switch ({}) {{\n", ind, scrutinee));
            for (label, body) in cases {
                out.push_str(&format!("{}    case {} -> {{\n", ind, label));
                for s in body {
                    emit_stmt(out, s, level + 2);
                }
                out.push_str(&format!("{}    }}\n", ind));
            }
            if !default.is_empty() {
                out.push_str(&format!("{}    default -> {{\n", ind));
                for s in default {
                    emit_stmt(out, s, level + 2);
                }
                out.push_str(&format!("{}    }}\n", ind));
            }
            out.push_str(&format!("{}}}\n", ind));
        }
        JavaStmt::For {
            init,
            cond,
            update,
            body,
        } => {
            let init_str = match init {
                Some(s) => {
                    let mut tmp = std::string::String::new();
                    emit_stmt(&mut tmp, s, 0);
                    tmp.trim_end_matches(";\n").trim().to_string()
                }
                None => std::string::String::new(),
            };
            let cond_str = match cond {
                Some(c) => format!("{}", c),
                None => std::string::String::new(),
            };
            let update_str = match update {
                Some(u) => format!("{}", u),
                None => std::string::String::new(),
            };
            out.push_str(&format!(
                "{}for ({}; {}; {}) {{\n",
                ind, init_str, cond_str, update_str
            ));
            for s in body {
                emit_stmt(out, s, level + 1);
            }
            out.push_str(&format!("{}}}\n", ind));
        }
        JavaStmt::ForEach {
            ty,
            elem,
            iterable,
            body,
        } => {
            out.push_str(&format!("{}for ({} {} : {}) {{\n", ind, ty, elem, iterable));
            for s in body {
                emit_stmt(out, s, level + 1);
            }
            out.push_str(&format!("{}}}\n", ind));
        }
        JavaStmt::While(cond, body) => {
            out.push_str(&format!("{}while ({}) {{\n", ind, cond));
            for s in body {
                emit_stmt(out, s, level + 1);
            }
            out.push_str(&format!("{}}}\n", ind));
        }
        JavaStmt::DoWhile(body, cond) => {
            out.push_str(&format!("{}do {{\n", ind));
            for s in body {
                emit_stmt(out, s, level + 1);
            }
            out.push_str(&format!("{}}} while ({});\n", ind, cond));
        }
        JavaStmt::Return(Some(expr)) => {
            out.push_str(&format!("{}return {};\n", ind, expr));
        }
        JavaStmt::Return(None) => {
            out.push_str(&format!("{}return;\n", ind));
        }
        JavaStmt::Throw(expr) => {
            out.push_str(&format!("{}throw {};\n", ind, expr));
        }
        JavaStmt::TryCatch {
            body,
            catches,
            finally,
        } => {
            out.push_str(&format!("{}try {{\n", ind));
            for s in body {
                emit_stmt(out, s, level + 1);
            }
            for catch in catches {
                let exc_str = catch.exception_types.join(" | ");
                out.push_str(&format!(
                    "{}}} catch ({} {}) {{\n",
                    ind, exc_str, catch.var_name
                ));
                for s in &catch.body {
                    emit_stmt(out, s, level + 1);
                }
            }
            if !finally.is_empty() {
                out.push_str(&format!("{}}} finally {{\n", ind));
                for s in finally {
                    emit_stmt(out, s, level + 1);
                }
            }
            out.push_str(&format!("{}}}\n", ind));
        }
        JavaStmt::TryWithResources {
            resources,
            body,
            catches,
            finally,
        } => {
            out.push_str(&format!("{}try (", ind));
            for (i, (name, expr)) in resources.iter().enumerate() {
                if i > 0 {
                    out.push_str("; ");
                }
                out.push_str(&format!("var {} = {}", name, expr));
            }
            out.push_str(") {\n");
            for s in body {
                emit_stmt(out, s, level + 1);
            }
            for catch in catches {
                let exc_str = catch.exception_types.join(" | ");
                out.push_str(&format!(
                    "{}}} catch ({} {}) {{\n",
                    ind, exc_str, catch.var_name
                ));
                for s in &catch.body {
                    emit_stmt(out, s, level + 1);
                }
            }
            if !finally.is_empty() {
                out.push_str(&format!("{}}} finally {{\n", ind));
                for s in finally {
                    emit_stmt(out, s, level + 1);
                }
            }
            out.push_str(&format!("{}}}\n", ind));
        }
        JavaStmt::Synchronized(lock, body) => {
            out.push_str(&format!("{}synchronized ({}) {{\n", ind, lock));
            for s in body {
                emit_stmt(out, s, level + 1);
            }
            out.push_str(&format!("{}}}\n", ind));
        }
        JavaStmt::Break(label) => match label {
            Some(l) => out.push_str(&format!("{}break {};\n", ind, l)),
            None => out.push_str(&format!("{}break;\n", ind)),
        },
        JavaStmt::Continue(label) => match label {
            Some(l) => out.push_str(&format!("{}continue {};\n", ind, l)),
            None => out.push_str(&format!("{}continue;\n", ind)),
        },
        JavaStmt::Assert(cond, msg) => match msg {
            Some(m) => out.push_str(&format!("{}assert {} : {};\n", ind, cond, m)),
            None => out.push_str(&format!("{}assert {};\n", ind, cond)),
        },
    }
}
/// Set of Java reserved keywords.
pub const JAVA_KEYWORDS: &[&str] = &[
    "abstract",
    "assert",
    "boolean",
    "break",
    "byte",
    "case",
    "catch",
    "char",
    "class",
    "const",
    "continue",
    "default",
    "do",
    "double",
    "else",
    "enum",
    "extends",
    "final",
    "finally",
    "float",
    "for",
    "goto",
    "if",
    "implements",
    "import",
    "instanceof",
    "int",
    "interface",
    "long",
    "native",
    "new",
    "package",
    "private",
    "protected",
    "public",
    "return",
    "short",
    "static",
    "strictfp",
    "super",
    "switch",
    "synchronized",
    "this",
    "throw",
    "throws",
    "transient",
    "try",
    "void",
    "volatile",
    "while",
    "true",
    "false",
    "null",
    "var",
    "record",
    "sealed",
    "permits",
    "yield",
    "when",
];
pub(super) fn collect_ctor_names_from_expr(
    expr: &LcnfExpr,
    out: &mut HashSet<std::string::String>,
) {
    match expr {
        LcnfExpr::Let { value, body, .. } => {
            collect_ctor_names_from_value(value, out);
            collect_ctor_names_from_expr(body, out);
        }
        LcnfExpr::Case { alts, default, .. } => {
            for alt in alts {
                out.insert(alt.ctor_name.clone());
                collect_ctor_names_from_expr(&alt.body, out);
            }
            if let Some(d) = default {
                collect_ctor_names_from_expr(d, out);
            }
        }
        LcnfExpr::Return(_) | LcnfExpr::Unreachable | LcnfExpr::TailCall(_, _) => {}
    }
}
pub(super) fn collect_ctor_names_from_value(
    value: &LcnfLetValue,
    out: &mut HashSet<std::string::String>,
) {
    match value {
        LcnfLetValue::Ctor(name, _, _) => {
            out.insert(name.clone());
        }
        LcnfLetValue::Reuse(_, name, _, _) => {
            out.insert(name.clone());
        }
        _ => {}
    }
}
/// Minimal Java runtime class emitted at the top of every generated module.
pub const JAVA_RUNTIME: &str = r#"
/**
 * OxiLean Java Runtime — generated, do not modify.
 */
public final class OxiLeanRuntime {

    private OxiLeanRuntime() {}

    /** Called when pattern matching reaches an unreachable branch. */
    public static RuntimeException unreachable() {
        throw new IllegalStateException("OxiLean: unreachable code reached");
    }

    /** Saturating natural-number subtraction (truncates at 0). */
    public static long natSub(long a, long b) {
        return Math.max(0L, a - b);
    }

    /** Natural-number division (returns 0 on division by zero). */
    public static long natDiv(long a, long b) {
        return b == 0L ? 0L : a / b;
    }

    /** Natural-number modulo (returns a on division by zero). */
    public static long natMod(long a, long b) {
        return b == 0L ? a : a % b;
    }

    /** Boolean to Nat conversion. */
    public static long decide(boolean b) {
        return b ? 1L : 0L;
    }

    /** String representation of a Nat. */
    public static String natToString(long n) {
        return Long.toString(n);
    }

    /** String append. */
    public static String strAppend(String a, String b) {
        return a + b;
    }

    /** Pair (generic tuple). */
    public record Pair<A, B>(A fst, B snd) {}

    /** Pair constructor. */
    public static <A, B> Pair<A, B> mkPair(A a, B b) {
        return new Pair<>(a, b);
    }
}
"#;
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub(super) fn test_java_type_primitives() {
        assert_eq!(JavaType::Int.to_string(), "int");
        assert_eq!(JavaType::Long.to_string(), "long");
        assert_eq!(JavaType::Double.to_string(), "double");
        assert_eq!(JavaType::Float.to_string(), "float");
        assert_eq!(JavaType::Boolean.to_string(), "boolean");
        assert_eq!(JavaType::Char.to_string(), "char");
        assert_eq!(JavaType::Byte.to_string(), "byte");
        assert_eq!(JavaType::Short.to_string(), "short");
        assert_eq!(JavaType::Void.to_string(), "void");
        assert_eq!(JavaType::String.to_string(), "String");
        assert_eq!(JavaType::Object.to_string(), "Object");
    }
    #[test]
    pub(super) fn test_java_type_array() {
        let t = JavaType::Array(Box::new(JavaType::Int));
        assert_eq!(t.to_string(), "int[]");
    }
    #[test]
    pub(super) fn test_java_type_list() {
        let t = JavaType::List(Box::new(JavaType::String));
        assert_eq!(t.to_string(), "List<String>");
    }
    #[test]
    pub(super) fn test_java_type_list_primitive_boxed() {
        let t = JavaType::List(Box::new(JavaType::Int));
        assert_eq!(t.to_string(), "List<Integer>");
    }
    #[test]
    pub(super) fn test_java_type_map() {
        let t = JavaType::Map(Box::new(JavaType::String), Box::new(JavaType::Int));
        assert_eq!(t.to_string(), "Map<String, Integer>");
    }
    #[test]
    pub(super) fn test_java_type_optional() {
        let t = JavaType::Optional(Box::new(JavaType::String));
        assert_eq!(t.to_string(), "Optional<String>");
    }
    #[test]
    pub(super) fn test_java_type_custom() {
        let t = JavaType::Custom("MyClass".to_string());
        assert_eq!(t.to_string(), "MyClass");
    }
    #[test]
    pub(super) fn test_java_type_generic() {
        let t = JavaType::Generic("Map".to_string(), vec![JavaType::String, JavaType::Long]);
        assert_eq!(t.to_string(), "Map<String, long>");
    }
    #[test]
    pub(super) fn test_java_lit_int() {
        assert_eq!(JavaLit::Int(42).to_string(), "42");
        assert_eq!(JavaLit::Int(-7).to_string(), "-7");
    }
    #[test]
    pub(super) fn test_java_lit_long() {
        assert_eq!(JavaLit::Long(100).to_string(), "100L");
    }
    #[test]
    pub(super) fn test_java_lit_bool() {
        assert_eq!(JavaLit::Bool(true).to_string(), "true");
        assert_eq!(JavaLit::Bool(false).to_string(), "false");
    }
    #[test]
    pub(super) fn test_java_lit_null() {
        assert_eq!(JavaLit::Null.to_string(), "null");
    }
    #[test]
    pub(super) fn test_java_lit_string_escaping() {
        let s = JavaLit::Str("hello\nworld\"test".to_string());
        assert_eq!(s.to_string(), r#""hello\nworld\"test""#);
    }
    #[test]
    pub(super) fn test_java_lit_char() {
        assert_eq!(JavaLit::Char('a').to_string(), "'a'");
    }
    #[test]
    pub(super) fn test_java_expr_var() {
        assert_eq!(JavaExpr::Var("x".to_string()).to_string(), "x");
    }
    #[test]
    pub(super) fn test_java_expr_binop() {
        let e = JavaExpr::BinOp(
            "+".to_string(),
            Box::new(JavaExpr::Var("a".to_string())),
            Box::new(JavaExpr::Var("b".to_string())),
        );
        assert_eq!(e.to_string(), "(a + b)");
    }
    #[test]
    pub(super) fn test_java_expr_method_call() {
        let e = JavaExpr::MethodCall(
            Box::new(JavaExpr::Var("list".to_string())),
            "stream".to_string(),
            vec![],
        );
        assert_eq!(e.to_string(), "list.stream()");
    }
    #[test]
    pub(super) fn test_java_expr_new() {
        let e = JavaExpr::New("ArrayList".to_string(), vec![]);
        assert_eq!(e.to_string(), "new ArrayList()");
    }
    #[test]
    pub(super) fn test_java_expr_lambda_single_param() {
        let e = JavaExpr::Lambda(
            vec!["x".to_string()],
            Box::new(JavaExpr::BinOp(
                ">".to_string(),
                Box::new(JavaExpr::Var("x".to_string())),
                Box::new(JavaExpr::Lit(JavaLit::Int(0))),
            )),
        );
        assert_eq!(e.to_string(), "x -> (x > 0)");
    }
    #[test]
    pub(super) fn test_java_expr_lambda_no_params() {
        let e = JavaExpr::Lambda(vec![], Box::new(JavaExpr::Lit(JavaLit::Int(42))));
        assert_eq!(e.to_string(), "() -> 42");
    }
    #[test]
    pub(super) fn test_java_expr_lambda_multi_param() {
        let e = JavaExpr::Lambda(
            vec!["x".to_string(), "y".to_string()],
            Box::new(JavaExpr::BinOp(
                "+".to_string(),
                Box::new(JavaExpr::Var("x".to_string())),
                Box::new(JavaExpr::Var("y".to_string())),
            )),
        );
        assert_eq!(e.to_string(), "(x, y) -> (x + y)");
    }
    #[test]
    pub(super) fn test_java_expr_ternary() {
        let e = JavaExpr::Ternary(
            Box::new(JavaExpr::Var("cond".to_string())),
            Box::new(JavaExpr::Lit(JavaLit::Int(1))),
            Box::new(JavaExpr::Lit(JavaLit::Int(0))),
        );
        assert_eq!(e.to_string(), "(cond ? 1 : 0)");
    }
    #[test]
    pub(super) fn test_java_expr_method_ref() {
        let e = JavaExpr::MethodRef("String".to_string(), "valueOf".to_string());
        assert_eq!(e.to_string(), "String::valueOf");
    }
    #[test]
    pub(super) fn test_java_expr_instanceof() {
        let e = JavaExpr::Instanceof(
            Box::new(JavaExpr::Var("obj".to_string())),
            "String".to_string(),
        );
        assert_eq!(e.to_string(), "(obj instanceof String)");
    }
    #[test]
    pub(super) fn test_java_expr_cast() {
        let e = JavaExpr::Cast(JavaType::Long, Box::new(JavaExpr::Var("n".to_string())));
        assert_eq!(e.to_string(), "((long) n)");
    }
    #[test]
    pub(super) fn test_java_expr_array_access() {
        let e = JavaExpr::ArrayAccess(
            Box::new(JavaExpr::Var("arr".to_string())),
            Box::new(JavaExpr::Lit(JavaLit::Int(0))),
        );
        assert_eq!(e.to_string(), "arr[0]");
    }
    #[test]
    pub(super) fn test_emit_simple_record() {
        let rec = JavaRecord::new("Point", vec![("x", JavaType::Int), ("y", JavaType::Int)]);
        let mut out = std::string::String::new();
        emit_record(&mut out, &rec, 0);
        assert!(out.contains("record Point"));
        assert!(out.contains("int x"));
        assert!(out.contains("int y"));
    }
    #[test]
    pub(super) fn test_emit_record_with_implements() {
        let mut rec = JavaRecord::new("Lit", vec![("value", JavaType::Int)]);
        rec.implements.push("Expr".to_string());
        let mut out = std::string::String::new();
        emit_record(&mut out, &rec, 0);
        assert!(out.contains("implements Expr"));
    }
    #[test]
    pub(super) fn test_emit_sealed_interface() {
        let iface = SealedInterface::new("Expr", vec!["Lit", "Add", "Mul"]);
        let mut out = std::string::String::new();
        emit_sealed_interface(&mut out, &iface, 0);
        assert!(out.contains("sealed interface Expr"));
        assert!(out.contains("permits Lit, Add, Mul"));
    }
    #[test]
    pub(super) fn test_emit_simple_enum() {
        let en = JavaEnum::new("Color", vec!["RED", "GREEN", "BLUE"]);
        let mut out = std::string::String::new();
        emit_enum(&mut out, &en, 0);
        assert!(out.contains("enum Color"));
        assert!(out.contains("RED,"));
        assert!(out.contains("GREEN,"));
        assert!(out.contains("BLUE;"));
    }
    #[test]
    pub(super) fn test_emit_simple_class() {
        let cls = JavaClass::new("Foo");
        let mut out = std::string::String::new();
        emit_class(&mut out, &cls, 0);
        assert!(out.contains("public class Foo"));
        assert!(out.contains('{'));
        assert!(out.contains('}'));
    }
    #[test]
    pub(super) fn test_emit_class_with_superclass() {
        let mut cls = JavaClass::new("Bar");
        cls.superclass = Some("Foo".to_string());
        let mut out = std::string::String::new();
        emit_class(&mut out, &cls, 0);
        assert!(out.contains("extends Foo"));
    }
    #[test]
    pub(super) fn test_module_emit_package() {
        let module = JavaModule::new("com.example");
        let src = module.emit();
        assert!(src.starts_with("package com.example;"));
    }
    #[test]
    pub(super) fn test_module_emit_imports() {
        let mut module = JavaModule::new("com.example");
        module.imports.push("java.util.List".to_string());
        let src = module.emit();
        assert!(src.contains("import java.util.List;"));
    }
    #[test]
    pub(super) fn test_mangle_reserved_keyword() {
        let backend = JavaBackend::new();
        assert_eq!(backend.mangle_name("class"), "class_");
        assert_eq!(backend.mangle_name("int"), "int_");
        assert_eq!(backend.mangle_name("return"), "return_");
    }
    #[test]
    pub(super) fn test_mangle_digit_start() {
        let backend = JavaBackend::new();
        assert_eq!(backend.mangle_name("1foo"), "_1foo");
    }
    #[test]
    pub(super) fn test_mangle_special_chars() {
        let backend = JavaBackend::new();
        assert_eq!(backend.mangle_name("foo.bar"), "foo_bar");
        assert_eq!(backend.mangle_name("foo::bar"), "foo__bar");
    }
    #[test]
    pub(super) fn test_mangle_empty() {
        let backend = JavaBackend::new();
        assert_eq!(backend.mangle_name(""), "_anon");
    }
    #[test]
    pub(super) fn test_visibility_display() {
        assert_eq!(Visibility::Public.to_string(), "public");
        assert_eq!(Visibility::Protected.to_string(), "protected");
        assert_eq!(Visibility::Private.to_string(), "private");
        assert_eq!(Visibility::Package.to_string(), "");
    }
    #[test]
    pub(super) fn test_java_runtime_content() {
        assert!(JAVA_RUNTIME.contains("OxiLeanRuntime"));
        assert!(JAVA_RUNTIME.contains("natSub"));
        assert!(JAVA_RUNTIME.contains("natDiv"));
        assert!(JAVA_RUNTIME.contains("strAppend"));
    }
}
#[cfg(test)]
mod Java_infra_tests {
    use super::*;
    #[test]
    pub(super) fn test_pass_config() {
        let config = JavaPassConfig::new("test_pass", JavaPassPhase::Transformation);
        assert!(config.enabled);
        assert!(config.phase.is_modifying());
        assert_eq!(config.phase.name(), "transformation");
    }
    #[test]
    pub(super) fn test_pass_stats() {
        let mut stats = JavaPassStats::new();
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
        let mut reg = JavaPassRegistry::new();
        reg.register(JavaPassConfig::new("pass_a", JavaPassPhase::Analysis));
        reg.register(JavaPassConfig::new("pass_b", JavaPassPhase::Transformation).disabled());
        assert_eq!(reg.total_passes(), 2);
        assert_eq!(reg.enabled_count(), 1);
        reg.update_stats("pass_a", 5, 50, 2);
        let stats = reg.get_stats("pass_a").expect("stats should exist");
        assert_eq!(stats.total_changes, 5);
    }
    #[test]
    pub(super) fn test_analysis_cache() {
        let mut cache = JavaAnalysisCache::new(10);
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
        let mut wl = JavaWorklist::new();
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
        let mut dt = JavaDominatorTree::new(5);
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
        let mut liveness = JavaLivenessInfo::new(3);
        liveness.add_def(0, 1);
        liveness.add_use(1, 1);
        assert!(liveness.defs[0].contains(&1));
        assert!(liveness.uses[1].contains(&1));
    }
    #[test]
    pub(super) fn test_constant_folding() {
        assert_eq!(JavaConstantFoldingHelper::fold_add_i64(3, 4), Some(7));
        assert_eq!(JavaConstantFoldingHelper::fold_div_i64(10, 0), None);
        assert_eq!(JavaConstantFoldingHelper::fold_div_i64(10, 2), Some(5));
        assert_eq!(
            JavaConstantFoldingHelper::fold_bitand_i64(0b1100, 0b1010),
            0b1000
        );
        assert_eq!(JavaConstantFoldingHelper::fold_bitnot_i64(0), -1);
    }
    #[test]
    pub(super) fn test_dep_graph() {
        let mut g = JavaDepGraph::new();
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
