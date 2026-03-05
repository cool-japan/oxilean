//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;
use std::collections::{HashMap, HashSet};

use super::types::{
    DartAnnotation, DartBackend, DartClass, DartCodeMetrics, DartEnum, DartEnumVariant, DartExpr,
    DartExtension, DartField, DartFile, DartFunction, DartImportExt, DartLit, DartMixin,
    DartNullSafety, DartParam, DartSealedHierarchy, DartStmt, DartStreamBuilder, DartType,
    DartTypeAlias,
};
use std::fmt;

/// Map an LCNF type to a Dart type.
pub(super) fn lcnf_type_to_dart(ty: &LcnfType) -> DartType {
    match ty {
        LcnfType::Nat => DartType::DtInt,
        LcnfType::LcnfString => DartType::DtString,
        LcnfType::Unit => DartType::DtVoid,
        LcnfType::Erased | LcnfType::Irrelevant => DartType::DtDynamic,
        LcnfType::Object => DartType::DtObject,
        LcnfType::Var(name) => DartType::DtNamed(name.clone()),
        LcnfType::Fun(params, ret) => {
            let dart_params: Vec<DartType> = params.iter().map(lcnf_type_to_dart).collect();
            let dart_ret = lcnf_type_to_dart(ret);
            DartType::DtFunction(dart_params, Box::new(dart_ret))
        }
        LcnfType::Ctor(name, _args) => DartType::DtNamed(name.clone()),
    }
}
pub(super) fn fmt_args(f: &mut fmt::Formatter<'_>, args: &[DartExpr]) -> fmt::Result {
    for (i, a) in args.iter().enumerate() {
        if i > 0 {
            write!(f, ", ")?;
        }
        write!(f, "{}", a)?;
    }
    Ok(())
}
pub(super) fn fmt_typed_params(
    f: &mut fmt::Formatter<'_>,
    params: &[(DartType, String)],
) -> fmt::Result {
    for (i, (ty, name)) in params.iter().enumerate() {
        if i > 0 {
            write!(f, ", ")?;
        }
        write!(f, "{} {}", ty, name)?;
    }
    Ok(())
}
/// Emit a DartField as a Dart source string (without indentation).
pub fn emit_dart_field(field: &DartField) -> String {
    let mut modifiers = String::new();
    if field.is_static {
        modifiers.push_str("static ");
    }
    if field.is_final {
        modifiers.push_str("final ");
    }
    if field.is_late {
        modifiers.push_str("late ");
    }
    if let Some(ref init) = field.default_value {
        format!("{}{} {} = {};", modifiers, field.ty, field.name, init)
    } else {
        format!("{}{} {};", modifiers, field.ty, field.name)
    }
}
pub(super) fn mangle_dart_ident(name: &str, keywords: &HashSet<&'static str>) -> String {
    let base: String = name
        .chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '_' {
                c
            } else {
                '_'
            }
        })
        .collect();
    let base = if base.starts_with(|c: char| c.is_ascii_digit()) {
        format!("_{}", base)
    } else {
        base
    };
    if keywords.contains(base.as_str()) {
        format!("{}_", base)
    } else {
        base
    }
}
pub(super) fn collect_ctor_names(module: &LcnfModule) -> HashSet<String> {
    let mut out = HashSet::new();
    for func in &module.fun_decls {
        collect_ctor_names_from_expr(&func.body, &mut out);
    }
    out
}
pub(super) fn collect_ctor_names_from_expr(expr: &LcnfExpr, out: &mut HashSet<String>) {
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
pub(super) fn collect_ctor_names_from_value(value: &LcnfLetValue, out: &mut HashSet<String>) {
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
/// Build a simple OxiLean constructor class.
pub(super) fn make_ctor_class(name: &str) -> DartClass {
    let mut class = DartClass::new(name);
    class.fields.push(DartField {
        ty: DartType::DtInt,
        name: "tag".to_string(),
        is_final: true,
        is_static: false,
        is_late: false,
        default_value: None,
        doc: None,
    });
    class.fields.push(DartField {
        ty: DartType::DtList(Box::new(DartType::DtDynamic)),
        name: "fields".to_string(),
        is_final: true,
        is_static: false,
        is_late: false,
        default_value: None,
        doc: None,
    });
    let mut ctor = DartFunction::new("", DartType::DtVoid);
    ctor.params = vec![
        DartParam::positional(DartType::DtInt, "tag"),
        DartParam::positional(DartType::DtList(Box::new(DartType::DtDynamic)), "fields"),
    ];
    ctor.body = vec![
        DartStmt::Raw("this.tag = tag;".to_string()),
        DartStmt::Raw("this.fields = fields;".to_string()),
    ];
    class.constructors.push(ctor);
    class
}
pub static DART_KEYWORDS: &[&str] = &[
    "abstract",
    "as",
    "assert",
    "async",
    "await",
    "base",
    "break",
    "case",
    "catch",
    "class",
    "const",
    "continue",
    "covariant",
    "default",
    "deferred",
    "do",
    "dynamic",
    "else",
    "enum",
    "export",
    "extends",
    "extension",
    "external",
    "factory",
    "false",
    "final",
    "finally",
    "for",
    "Function",
    "get",
    "hide",
    "if",
    "implements",
    "import",
    "in",
    "interface",
    "is",
    "late",
    "library",
    "mixin",
    "new",
    "null",
    "of",
    "on",
    "operator",
    "part",
    "required",
    "rethrow",
    "return",
    "sealed",
    "set",
    "show",
    "static",
    "super",
    "switch",
    "sync",
    "this",
    "throw",
    "true",
    "try",
    "typedef",
    "var",
    "void",
    "when",
    "while",
    "with",
    "yield",
];
/// Minimal Dart runtime class emitted at the top of every generated file.
pub const DART_RUNTIME: &str = r#"
class OxiLeanRuntime {
  /// Called when pattern matching reaches an unreachable branch.
  static Never unreachable() =>
      throw StateError('OxiLean: unreachable code reached');

  /// Natural number addition.
  static int natAdd(int a, int b) => a + b;

  /// Natural number subtraction (saturating at 0).
  static int natSub(int a, int b) => a - b < 0 ? 0 : a - b;

  /// Natural number multiplication.
  static int natMul(int a, int b) => a * b;

  /// Natural number division (truncating, 0 if divisor is 0).
  static int natDiv(int a, int b) => b == 0 ? 0 : a ~/ b;

  /// Natural number modulo.
  static int natMod(int a, int b) => b == 0 ? a : a % b;

  /// Decide a boolean as a Nat (0 or 1).
  static int decide(bool b) => b ? 1 : 0;

  /// Convert Nat to String.
  static String natToString(int n) => n.toString();

  /// String append.
  static String strAppend(String a, String b) => a + b;

  /// List.cons: prepend element.
  static List<T> cons<T>(T head, List<T> tail) => [head, ...tail];

  /// List.nil: empty list.
  static List<T> nil<T>() => [];
}
"#;
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub(super) fn test_dart_type_display_primitives() {
        assert_eq!(format!("{}", DartType::DtInt), "int");
        assert_eq!(format!("{}", DartType::DtDouble), "double");
        assert_eq!(format!("{}", DartType::DtBool), "bool");
        assert_eq!(format!("{}", DartType::DtString), "String");
        assert_eq!(format!("{}", DartType::DtVoid), "void");
        assert_eq!(format!("{}", DartType::DtDynamic), "dynamic");
    }
    #[test]
    pub(super) fn test_dart_type_display_nullable() {
        let nullable_int = DartType::DtNullable(Box::new(DartType::DtInt));
        assert_eq!(format!("{}", nullable_int), "int?");
        let nullable_str = DartType::DtNullable(Box::new(DartType::DtString));
        assert_eq!(format!("{}", nullable_str), "String?");
    }
    #[test]
    pub(super) fn test_dart_type_display_generics() {
        let list_int = DartType::DtList(Box::new(DartType::DtInt));
        assert_eq!(format!("{}", list_int), "List<int>");
        let map_str_int = DartType::DtMap(Box::new(DartType::DtString), Box::new(DartType::DtInt));
        assert_eq!(format!("{}", map_str_int), "Map<String, int>");
        let future_str = DartType::DtFuture(Box::new(DartType::DtString));
        assert_eq!(format!("{}", future_str), "Future<String>");
    }
    #[test]
    pub(super) fn test_dart_type_display_function() {
        let fn_ty = DartType::DtFunction(
            vec![DartType::DtInt, DartType::DtString],
            Box::new(DartType::DtBool),
        );
        assert_eq!(format!("{}", fn_ty), "bool Function(int, String)");
    }
    #[test]
    pub(super) fn test_dart_lit_display() {
        assert_eq!(format!("{}", DartLit::Int(42)), "42");
        assert_eq!(format!("{}", DartLit::Double(3.14)), "3.14");
        assert_eq!(format!("{}", DartLit::Bool(true)), "true");
        assert_eq!(format!("{}", DartLit::Bool(false)), "false");
        assert_eq!(format!("{}", DartLit::Null), "null");
        assert_eq!(format!("{}", DartLit::Str("hi".to_string())), "'hi'");
        assert_eq!(format!("{}", DartLit::Str("it's".to_string())), "'it\\'s'");
    }
    #[test]
    pub(super) fn test_dart_expr_display() {
        let var = DartExpr::Var("x".to_string());
        assert_eq!(format!("{}", var), "x");
        let field = DartExpr::Field(
            Box::new(DartExpr::Var("obj".to_string())),
            "len".to_string(),
        );
        assert_eq!(format!("{}", field), "obj.len");
        let bin = DartExpr::BinOp(
            Box::new(DartExpr::Lit(DartLit::Int(1))),
            "+".to_string(),
            Box::new(DartExpr::Lit(DartLit::Int(2))),
        );
        assert_eq!(format!("{}", bin), "(1 + 2)");
        let null_coal = DartExpr::NullCoalesce(
            Box::new(DartExpr::Var("x".to_string())),
            Box::new(DartExpr::Lit(DartLit::Int(0))),
        );
        assert_eq!(format!("{}", null_coal), "(x ?? 0)");
    }
    #[test]
    pub(super) fn test_dart_stmt_emit_if() {
        let backend = DartBackend::new();
        let stmt = DartStmt::If(
            DartExpr::Var("cond".to_string()),
            vec![DartStmt::Return(Some(DartExpr::Lit(DartLit::Int(1))))],
            vec![DartStmt::Return(Some(DartExpr::Lit(DartLit::Int(0))))],
        );
        let code = backend.emit_stmt(&stmt, 0);
        assert!(code.contains("if (cond)"));
        assert!(code.contains("return 1;"));
        assert!(code.contains("else"));
        assert!(code.contains("return 0;"));
    }
    #[test]
    pub(super) fn test_dart_stmt_emit_for_in() {
        let backend = DartBackend::new();
        let stmt = DartStmt::ForIn(
            "item".to_string(),
            DartExpr::Var("list".to_string()),
            vec![DartStmt::Expr(DartExpr::MethodCall(
                Box::new(DartExpr::Var("print".to_string())),
                "call".to_string(),
                vec![DartExpr::Var("item".to_string())],
            ))],
        );
        let code = backend.emit_stmt(&stmt, 0);
        assert!(code.contains("for (final item in list)"));
    }
    #[test]
    pub(super) fn test_dart_backend_emit_function() {
        let backend = DartBackend::new();
        let mut func = DartFunction::new("add", DartType::DtInt);
        func.params = vec![
            DartParam::positional(DartType::DtInt, "a"),
            DartParam::positional(DartType::DtInt, "b"),
        ];
        func.body = vec![DartStmt::Return(Some(DartExpr::BinOp(
            Box::new(DartExpr::Var("a".to_string())),
            "+".to_string(),
            Box::new(DartExpr::Var("b".to_string())),
        )))];
        let code = backend.emit_function(&func, 0);
        assert!(code.contains("int add(int a, int b)"));
        assert!(code.contains("return (a + b);"));
    }
    #[test]
    pub(super) fn test_dart_backend_emit_class() {
        let backend = DartBackend::new();
        let mut class = DartClass::new("Point");
        class
            .fields
            .push(DartField::final_field(DartType::DtDouble, "x"));
        class
            .fields
            .push(DartField::final_field(DartType::DtDouble, "y"));
        let mut ctor = DartFunction::new("", DartType::DtVoid);
        ctor.params = vec![
            DartParam::positional(DartType::DtDouble, "x"),
            DartParam::positional(DartType::DtDouble, "y"),
        ];
        ctor.body = vec![
            DartStmt::Raw("this.x = x;".to_string()),
            DartStmt::Raw("this.y = y;".to_string()),
        ];
        class.constructors.push(ctor);
        let code = backend.emit_class(&class, 0);
        assert!(code.contains("class Point {"));
        assert!(code.contains("final double x;"));
        assert!(code.contains("final double y;"));
        assert!(code.contains("Point(double x, double y)"));
    }
    #[test]
    pub(super) fn test_mangle_dart_ident_keyword() {
        let mut backend = DartBackend::new();
        let mangled = backend.mangle_name("class");
        assert_eq!(mangled, "class_");
        let mangled2 = backend.mangle_name("myFunc");
        assert_eq!(mangled2, "myFunc");
    }
}
/// Emit a standalone Dart function that wraps a value in a `Future`.
#[allow(dead_code)]
pub fn emit_future_value_fn(name: &str, ty: DartType, val: DartExpr) -> DartFunction {
    let mut f = DartFunction::new(name, DartType::DtFuture(Box::new(ty)));
    f.is_async = true;
    f.body = vec![DartStmt::Return(Some(val))];
    f
}
/// Emit a `print(expr);` statement.
#[allow(dead_code)]
pub fn dart_print(expr: DartExpr) -> DartStmt {
    DartStmt::Expr(DartExpr::MethodCall(
        Box::new(DartExpr::Var("print".to_string())),
        "call".to_string(),
        vec![expr],
    ))
}
/// Build a `List.generate(n, (i) => expr)` expression.
#[allow(dead_code)]
pub fn list_generate(n: usize, body: DartExpr) -> DartExpr {
    DartExpr::MethodCall(
        Box::new(DartExpr::Var("List".to_string())),
        "generate".to_string(),
        vec![
            DartExpr::Lit(DartLit::Int(n as i64)),
            DartExpr::Arrow(vec![(DartType::DtInt, "i".to_string())], Box::new(body)),
        ],
    )
}
/// Emit a simple `assert(condition, message)` statement.
#[allow(dead_code)]
pub fn dart_assert(cond: DartExpr, msg: &str) -> DartStmt {
    let _ = msg;
    DartStmt::Assert(cond)
}
/// Build a `Map.fromEntries(entries)` expression.
#[allow(dead_code)]
pub fn map_from_entries(
    key_ty: DartType,
    val_ty: DartType,
    entries: Vec<(DartExpr, DartExpr)>,
) -> DartExpr {
    let _ = (key_ty, val_ty);
    let entry_exprs: Vec<DartExpr> = entries
        .into_iter()
        .map(|(k, v)| DartExpr::New("MapEntry".to_string(), None, vec![k, v]))
        .collect();
    DartExpr::MethodCall(
        Box::new(DartExpr::Var("Map".to_string())),
        "fromEntries".to_string(),
        vec![DartExpr::ListLit(entry_exprs)],
    )
}
#[cfg(test)]
mod tests_extended {
    use super::*;
    use crate::dart_backend::*;
    #[test]
    pub(super) fn test_type_alias_emit() {
        let ta = DartTypeAlias::new("StringList", DartType::DtList(Box::new(DartType::DtString)))
            .with_doc("A list of strings");
        let out = ta.emit();
        assert!(out.contains("typedef StringList = List<String>;"));
        assert!(out.contains("/// A list of strings"));
    }
    #[test]
    pub(super) fn test_import_simple() {
        let imp = DartImportExt::simple("dart:async");
        assert_eq!(imp.emit(), "import 'dart:async';\n");
    }
    #[test]
    pub(super) fn test_import_with_prefix() {
        let imp = DartImportExt::simple("package:http/http.dart").with_prefix("http");
        assert!(imp.emit().contains("as http"));
    }
    #[test]
    pub(super) fn test_import_show() {
        let imp = DartImportExt::simple("dart:math")
            .show_identifiers(vec!["Random".to_string(), "pi".to_string()]);
        let s = imp.emit();
        assert!(s.contains("show Random, pi"));
    }
    #[test]
    pub(super) fn test_dart_enum_emit() {
        let mut e = DartEnum::new("Color");
        e.add_variant(DartEnumVariant::simple("red"));
        e.add_variant(DartEnumVariant::simple("green"));
        e.add_variant(DartEnumVariant::simple("blue"));
        let out = e.emit();
        assert!(out.contains("enum Color {"));
        assert!(out.contains("red,"));
        assert!(out.contains("blue;"));
    }
    #[test]
    pub(super) fn test_dart_annotation_display() {
        assert_eq!(DartAnnotation::Override.to_string(), "@override");
        assert_eq!(DartAnnotation::Deprecated.to_string(), "@deprecated");
        let custom = DartAnnotation::Custom(
            "JsonSerializable".into(),
            vec!["explicitToJson: true".into()],
        );
        assert!(custom.to_string().contains("@JsonSerializable"));
    }
    #[test]
    pub(super) fn test_dart_file_emit() {
        let backend = DartBackend::new();
        let mut file = DartFile::new().with_library("my_lib");
        file.add_import(DartImport::simple("dart:core"));
        let mut cls = DartClass::new("Foo");
        cls.fields
            .push(DartField::final_field(DartType::DtInt, "x"));
        file.add_class(cls);
        let out = file.emit(&backend);
        assert!(out.contains("library my_lib;"));
        assert!(out.contains("import 'dart:core';"));
        assert!(out.contains("class Foo {"));
    }
    #[test]
    pub(super) fn test_sealed_hierarchy_emit() {
        let backend = DartBackend::new();
        let mut hier = DartSealedHierarchy::new("Shape");
        let mut circle = DartClass::new("Circle");
        circle.extends = Some("Shape".to_string());
        circle
            .fields
            .push(DartField::final_field(DartType::DtDouble, "radius"));
        hier.add_variant(circle);
        let out = hier.emit(&backend);
        assert!(out.contains("sealed class Shape {"));
        assert!(out.contains("class Circle extends Shape {"));
    }
    #[test]
    pub(super) fn test_dart_metrics_collect() {
        let mut file = DartFile::new();
        file.add_class(DartClass::new("A"));
        file.add_class(DartClass::new("B"));
        file.add_function(DartFunction::new("foo", DartType::DtVoid));
        file.add_import(DartImport::simple("dart:math"));
        let metrics = DartCodeMetrics::collect(&file);
        assert_eq!(metrics.class_count, 2);
        assert_eq!(metrics.function_count, 1);
        assert_eq!(metrics.import_count, 1);
    }
    #[test]
    pub(super) fn test_list_generate_expr() {
        let expr = list_generate(5, DartExpr::Var("i".to_string()));
        let s = format!("{}", expr);
        assert!(s.contains("List"));
        assert!(s.contains("generate"));
    }
    #[test]
    pub(super) fn test_stream_from_iterable() {
        let expr = DartStreamBuilder::from_iterable(vec![
            DartExpr::Lit(DartLit::Int(1)),
            DartExpr::Lit(DartLit::Int(2)),
        ]);
        let s = format!("{}", expr);
        assert!(s.contains("fromIterable"));
    }
    #[test]
    pub(super) fn test_null_safety_helpers() {
        let decl = DartNullSafety::nullable_decl(DartType::DtInt, "count");
        let backend = DartBackend::new();
        let out = backend.emit_stmt(&decl, 0);
        assert!(out.contains("int?"));
        assert!(out.contains("null"));
    }
    #[test]
    pub(super) fn test_mixin_emit() {
        let backend = DartBackend::new();
        let mut mixin = DartMixin::new("Serializable");
        let mut to_json = DartFunction::new(
            "toJson",
            DartType::DtMap(Box::new(DartType::DtString), Box::new(DartType::DtDynamic)),
        );
        to_json.body = vec![DartStmt::Return(Some(DartExpr::MapLit(vec![])))];
        mixin.methods.push(to_json);
        let out = mixin.emit(&backend, 0);
        assert!(out.contains("mixin Serializable {"));
        assert!(out.contains("toJson"));
    }
    #[test]
    pub(super) fn test_extension_emit() {
        let backend = DartBackend::new();
        let mut ext = DartExtension::new(DartType::DtString).named("StringExt");
        let mut is_blank = DartFunction::new("isBlank", DartType::DtBool);
        is_blank.body = vec![DartStmt::Return(Some(DartExpr::MethodCall(
            Box::new(DartExpr::Var("this".to_string())),
            "trim".to_string(),
            vec![],
        )))];
        ext.add_method(is_blank);
        let out = ext.emit(&backend, 0);
        assert!(out.contains("extension StringExt on String {"));
    }
    #[test]
    pub(super) fn test_emit_future_value_fn() {
        let f = emit_future_value_fn(
            "getAnswer",
            DartType::DtInt,
            DartExpr::Lit(DartLit::Int(42)),
        );
        assert_eq!(f.name, "getAnswer");
        assert!(f.is_async);
        assert!(matches!(f.return_type, DartType::DtFuture(_)));
    }
    #[test]
    pub(super) fn test_stream_listen_stmt() {
        let stream = DartExpr::Var("myStream".to_string());
        let stmt = DartStreamBuilder::listen(
            stream,
            "event",
            vec![DartStmt::Expr(DartExpr::Var("event".to_string()))],
        );
        let backend = DartBackend::new();
        let out = backend.emit_stmt(&stmt, 0);
        assert!(out.contains("listen"));
    }
}
