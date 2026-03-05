//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::{
    SwiftAnalysisCache, SwiftBackend, SwiftClassDecl, SwiftConformance, SwiftConstantFoldingHelper,
    SwiftDepGraph, SwiftDominatorTree, SwiftEnumCase, SwiftEnumDecl, SwiftExpr, SwiftExtension,
    SwiftField, SwiftFunc, SwiftLit, SwiftLivenessInfo, SwiftModule, SwiftParam, SwiftPassConfig,
    SwiftPassPhase, SwiftPassRegistry, SwiftPassStats, SwiftStmt, SwiftStructDecl, SwiftType,
    SwiftTypeDecl, SwiftWorklist,
};

/// Emit a block of statements indented by `indent` spaces.
pub(super) fn emit_block(stmts: &[SwiftStmt], indent: usize) -> String {
    let pad = " ".repeat(indent);
    stmts
        .iter()
        .map(|s| format!("{}{}", pad, emit_stmt(s, indent)))
        .collect::<Vec<_>>()
        .join("\n")
}
/// Emit a single statement at the given indent level.
pub(super) fn emit_stmt(stmt: &SwiftStmt, indent: usize) -> String {
    let pad = " ".repeat(indent);
    let pad2 = " ".repeat(indent + 4);
    match stmt {
        SwiftStmt::Let { name, ty, value } => {
            if let Some(t) = ty {
                format!("let {}: {} = {}", name, t, value)
            } else {
                format!("let {} = {}", name, value)
            }
        }
        SwiftStmt::Var { name, ty, value } => match (ty, value) {
            (Some(t), Some(v)) => format!("var {}: {} = {}", name, t, v),
            (Some(t), None) => format!("var {}: {}", name, t),
            (None, Some(v)) => format!("var {} = {}", name, v),
            (None, None) => format!("var {}", name),
        },
        SwiftStmt::Assign { target, value } => format!("{} = {}", target, value),
        SwiftStmt::Return(None) => "return".to_string(),
        SwiftStmt::Return(Some(expr)) => format!("return {}", expr),
        SwiftStmt::If {
            cond,
            then_body,
            else_body,
        } => {
            let mut out = format!("if {} {{\n", cond);
            out += &emit_block(then_body, indent + 4);
            if !then_body.is_empty() {
                out += "\n";
            }
            if else_body.is_empty() {
                out += &format!("{}}}", pad);
            } else {
                out += &format!("{}}} else {{\n", pad);
                out += &emit_block(else_body, indent + 4);
                if !else_body.is_empty() {
                    out += "\n";
                }
                out += &format!("{}}}", pad);
            }
            out
        }
        SwiftStmt::IfLet {
            name,
            value,
            then_body,
            else_body,
        } => {
            let mut out = format!("if let {} = {} {{\n", name, value);
            out += &emit_block(then_body, indent + 4);
            if !then_body.is_empty() {
                out += "\n";
            }
            if else_body.is_empty() {
                out += &format!("{}}}", pad);
            } else {
                out += &format!("{}}} else {{\n", pad);
                out += &emit_block(else_body, indent + 4);
                if !else_body.is_empty() {
                    out += "\n";
                }
                out += &format!("{}}}", pad);
            }
            out
        }
        SwiftStmt::Guard { cond, else_body } => {
            let mut out = format!("guard {} else {{\n", cond);
            out += &emit_block(else_body, indent + 4);
            if !else_body.is_empty() {
                out += "\n";
            }
            out += &format!("{}}}", pad);
            out
        }
        SwiftStmt::Switch { subject, cases } => {
            let mut out = format!("switch {} {{\n", subject);
            for case in cases {
                out += &format!("{}case {}:\n", pad2, case.pattern);
                out += &emit_block(&case.body, indent + 8);
                if !case.body.is_empty() {
                    out += "\n";
                }
            }
            out += &format!("{}}}", pad);
            out
        }
        SwiftStmt::For {
            name,
            collection,
            body,
        } => {
            let mut out = format!("for {} in {} {{\n", name, collection);
            out += &emit_block(body, indent + 4);
            if !body.is_empty() {
                out += "\n";
            }
            out += &format!("{}}}", pad);
            out
        }
        SwiftStmt::While { cond, body } => {
            let mut out = format!("while {} {{\n", cond);
            out += &emit_block(body, indent + 4);
            if !body.is_empty() {
                out += "\n";
            }
            out += &format!("{}}}", pad);
            out
        }
        SwiftStmt::Throw(expr) => format!("throw {}", expr),
        SwiftStmt::Break => "break".to_string(),
        SwiftStmt::Continue => "continue".to_string(),
        SwiftStmt::ExprStmt(e) => format!("{}", e),
        SwiftStmt::Raw(s) => s.clone(),
        SwiftStmt::Block(stmts) => {
            let mut out = "{\n".to_string();
            out += &emit_block(stmts, indent + 4);
            if !stmts.is_empty() {
                out += "\n";
            }
            out += &format!("{}}}", pad);
            out
        }
    }
}
/// All Swift reserved keywords that cannot be used as bare identifiers.
const SWIFT_KEYWORDS: &[&str] = &[
    "associatedtype",
    "class",
    "deinit",
    "enum",
    "extension",
    "fileprivate",
    "func",
    "import",
    "init",
    "inout",
    "internal",
    "let",
    "open",
    "operator",
    "private",
    "precedencegroup",
    "protocol",
    "public",
    "rethrows",
    "static",
    "struct",
    "subscript",
    "typealias",
    "var",
    "break",
    "case",
    "catch",
    "continue",
    "default",
    "defer",
    "do",
    "else",
    "fallthrough",
    "for",
    "guard",
    "if",
    "in",
    "repeat",
    "return",
    "throw",
    "switch",
    "where",
    "while",
    "any",
    "as",
    "await",
    "false",
    "is",
    "nil",
    "self",
    "Self",
    "super",
    "throws",
    "true",
    "try",
    "async",
    "convenience",
    "didSet",
    "dynamic",
    "final",
    "get",
    "indirect",
    "lazy",
    "mutating",
    "none",
    "nonisolated",
    "nonmutating",
    "optional",
    "override",
    "postfix",
    "prefix",
    "required",
    "set",
    "some",
    "Type",
    "unowned",
    "weak",
    "willSet",
    "associativity",
    "consume",
    "copy",
    "discard",
    "distributed",
    "each",
    "isolated",
    "macro",
    "package",
    "then",
];
/// Return `true` if `name` is a Swift reserved keyword.
pub fn is_swift_keyword(name: &str) -> bool {
    SWIFT_KEYWORDS.contains(&name)
}
/// A minimal Swift runtime preamble embedded in every generated module.
///
/// Provides:
/// - `OxValue` — the universal boxed value type
/// - `OxNat`   — arbitrary-precision natural numbers (wraps `UInt`)
/// - `OxError` — runtime error type conforming to `Swift.Error`
/// - Helper utilities: `ox_panic`, `ox_unreachable`
pub const OXILEAN_SWIFT_RUNTIME: &str = r#"
// ── OxiLean Swift Runtime ────────────────────────────────────────────────────

/// Universal boxed value for OxiLean-compiled terms.
public indirect enum OxValue {
    case int(Int)
    case bool(Bool)
    case string(String)
    case float(Double)
    case ctor(tag: Int, fields: [OxValue])
    case closure(([OxValue]) -> OxValue)
    case unit
    case erased
}

/// Natural number wrapper (Lean `Nat` maps to `UInt` on 64-bit platforms).
public struct OxNat: Equatable, Comparable, CustomStringConvertible {
    public let value: UInt
    public init(_ value: UInt) { self.value = value }
    public var description: String { "\(value)" }
    public static func < (lhs: OxNat, rhs: OxNat) -> Bool { lhs.value < rhs.value }
    public static func + (lhs: OxNat, rhs: OxNat) -> OxNat { OxNat(lhs.value + rhs.value) }
    public static func * (lhs: OxNat, rhs: OxNat) -> OxNat { OxNat(lhs.value * rhs.value) }
    public func pred() -> OxNat { OxNat(value > 0 ? value - 1 : 0) }
}

/// Runtime error thrown by compiled OxiLean programs.
public struct OxError: Error, CustomStringConvertible {
    public let message: String
    public init(_ message: String) { self.message = message }
    public var description: String { "OxError: \(message)" }
}

/// Panic unconditionally — never returns.
@inline(never)
public func ox_panic(_ message: String = "unreachable") -> Never {
    fatalError("[OxiLean] \(message)")
}

/// Abort on unreachable code path.
@inline(never)
public func ox_unreachable() -> Never {
    ox_panic("unreachable code executed")
}

// ── End OxiLean Swift Runtime ─────────────────────────────────────────────────
"#;
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub(super) fn test_swift_type_display_primitives() {
        assert_eq!(SwiftType::SwiftInt.to_string(), "Int");
        assert_eq!(SwiftType::SwiftBool.to_string(), "Bool");
        assert_eq!(SwiftType::SwiftString.to_string(), "String");
        assert_eq!(SwiftType::SwiftDouble.to_string(), "Double");
        assert_eq!(SwiftType::SwiftVoid.to_string(), "Void");
        assert_eq!(SwiftType::SwiftAny.to_string(), "Any");
        assert_eq!(SwiftType::SwiftNever.to_string(), "Never");
    }
    #[test]
    pub(super) fn test_swift_type_display_compound() {
        let arr = SwiftType::SwiftArray(Box::new(SwiftType::SwiftInt));
        assert_eq!(arr.to_string(), "[Int]");
        let opt = SwiftType::SwiftOptional(Box::new(SwiftType::SwiftString));
        assert_eq!(opt.to_string(), "String?");
        let dict = SwiftType::SwiftDict(
            Box::new(SwiftType::SwiftString),
            Box::new(SwiftType::SwiftInt),
        );
        assert_eq!(dict.to_string(), "[String: Int]");
        let tuple = SwiftType::SwiftTuple(vec![SwiftType::SwiftInt, SwiftType::SwiftBool]);
        assert_eq!(tuple.to_string(), "(Int, Bool)");
        let func = SwiftType::SwiftFunc(
            vec![SwiftType::SwiftInt, SwiftType::SwiftString],
            Box::new(SwiftType::SwiftBool),
        );
        assert_eq!(func.to_string(), "(Int, String) -> Bool");
    }
    #[test]
    pub(super) fn test_swift_type_display_generic() {
        let g = SwiftType::SwiftGeneric(
            "Result".to_string(),
            vec![
                SwiftType::SwiftString,
                SwiftType::SwiftNamed("Error".to_string()),
            ],
        );
        assert_eq!(g.to_string(), "Result<String, Error>");
    }
    #[test]
    pub(super) fn test_swift_lit_int() {
        assert_eq!(SwiftLit::Int(42).to_string(), "42");
        assert_eq!(SwiftLit::Int(-7).to_string(), "-7");
    }
    #[test]
    pub(super) fn test_swift_lit_bool() {
        assert_eq!(SwiftLit::Bool(true).to_string(), "true");
        assert_eq!(SwiftLit::Bool(false).to_string(), "false");
    }
    #[test]
    pub(super) fn test_swift_lit_nil() {
        assert_eq!(SwiftLit::Nil.to_string(), "nil");
    }
    #[test]
    pub(super) fn test_swift_lit_float() {
        assert_eq!(SwiftLit::Float(1.0).to_string(), "1.0");
        assert_eq!(SwiftLit::Float(3.14).to_string(), "3.14");
    }
    #[test]
    pub(super) fn test_swift_lit_str_escaping() {
        let s = SwiftLit::Str("say \"hi\"\nbye".to_string());
        assert_eq!(s.to_string(), r#""say \"hi\"\nbye""#);
    }
    #[test]
    pub(super) fn test_swift_expr_var() {
        assert_eq!(
            SwiftExpr::SwiftVar("myVar".to_string()).to_string(),
            "myVar"
        );
    }
    #[test]
    pub(super) fn test_swift_expr_call_unlabeled() {
        let call = SwiftExpr::SwiftCall {
            callee: Box::new(SwiftExpr::SwiftVar("foo".to_string())),
            args: vec![
                ("".to_string(), SwiftExpr::SwiftLitExpr(SwiftLit::Int(1))),
                ("".to_string(), SwiftExpr::SwiftLitExpr(SwiftLit::Int(2))),
            ],
        };
        assert_eq!(call.to_string(), "foo(1, 2)");
    }
    #[test]
    pub(super) fn test_swift_expr_call_labeled() {
        let call = SwiftExpr::SwiftCall {
            callee: Box::new(SwiftExpr::SwiftVar("foo".to_string())),
            args: vec![
                ("x".to_string(), SwiftExpr::SwiftLitExpr(SwiftLit::Int(1))),
                ("y".to_string(), SwiftExpr::SwiftLitExpr(SwiftLit::Int(2))),
            ],
        };
        assert_eq!(call.to_string(), "foo(x: 1, y: 2)");
    }
    #[test]
    pub(super) fn test_swift_expr_binop() {
        let e = SwiftExpr::SwiftBinOp {
            op: "+".to_string(),
            lhs: Box::new(SwiftExpr::SwiftLitExpr(SwiftLit::Int(1))),
            rhs: Box::new(SwiftExpr::SwiftLitExpr(SwiftLit::Int(2))),
        };
        assert_eq!(e.to_string(), "(1 + 2)");
    }
    #[test]
    pub(super) fn test_swift_expr_member() {
        let e = SwiftExpr::SwiftMember(
            Box::new(SwiftExpr::SwiftVar("obj".to_string())),
            "field".to_string(),
        );
        assert_eq!(e.to_string(), "obj.field");
    }
    #[test]
    pub(super) fn test_swift_expr_optional_chain() {
        let e = SwiftExpr::SwiftOptionalChain(
            Box::new(SwiftExpr::SwiftVar("obj".to_string())),
            "name".to_string(),
        );
        assert_eq!(e.to_string(), "obj?.name");
    }
    #[test]
    pub(super) fn test_swift_expr_array_lit() {
        let e = SwiftExpr::SwiftArrayLit(vec![
            SwiftExpr::SwiftLitExpr(SwiftLit::Int(1)),
            SwiftExpr::SwiftLitExpr(SwiftLit::Int(2)),
        ]);
        assert_eq!(e.to_string(), "[1, 2]");
    }
    #[test]
    pub(super) fn test_swift_expr_dict_lit_empty() {
        let e = SwiftExpr::SwiftDictLit(vec![]);
        assert_eq!(e.to_string(), "[:]");
    }
    #[test]
    pub(super) fn test_swift_expr_ternary() {
        let e = SwiftExpr::SwiftTernary(
            Box::new(SwiftExpr::SwiftVar("c".to_string())),
            Box::new(SwiftExpr::SwiftLitExpr(SwiftLit::Int(1))),
            Box::new(SwiftExpr::SwiftLitExpr(SwiftLit::Int(0))),
        );
        assert_eq!(e.to_string(), "(c ? 1 : 0)");
    }
    #[test]
    pub(super) fn test_swift_stmt_let() {
        let s = SwiftStmt::Let {
            name: "x".to_string(),
            ty: Some(SwiftType::SwiftInt),
            value: SwiftExpr::SwiftLitExpr(SwiftLit::Int(42)),
        };
        assert_eq!(s.to_string(), "let x: Int = 42");
    }
    #[test]
    pub(super) fn test_swift_stmt_var_no_value() {
        let s = SwiftStmt::Var {
            name: "count".to_string(),
            ty: Some(SwiftType::SwiftInt),
            value: None,
        };
        assert_eq!(s.to_string(), "var count: Int");
    }
    #[test]
    pub(super) fn test_swift_stmt_return_none() {
        assert_eq!(SwiftStmt::Return(None).to_string(), "return");
    }
    #[test]
    pub(super) fn test_swift_stmt_return_expr() {
        let s = SwiftStmt::Return(Some(SwiftExpr::SwiftLitExpr(SwiftLit::Bool(true))));
        assert_eq!(s.to_string(), "return true");
    }
    #[test]
    pub(super) fn test_swift_stmt_throw() {
        let s = SwiftStmt::Throw(SwiftExpr::SwiftCall {
            callee: Box::new(SwiftExpr::SwiftVar("OxError".to_string())),
            args: vec![(
                "".to_string(),
                SwiftExpr::SwiftLitExpr(SwiftLit::Str("oops".to_string())),
            )],
        });
        assert_eq!(s.to_string(), "throw OxError(\"oops\")");
    }
    #[test]
    pub(super) fn test_swift_stmt_if_no_else() {
        let s = SwiftStmt::If {
            cond: SwiftExpr::SwiftVar("cond".to_string()),
            then_body: vec![SwiftStmt::Return(Some(SwiftExpr::SwiftLitExpr(
                SwiftLit::Int(1),
            )))],
            else_body: vec![],
        };
        let out = s.to_string();
        assert!(out.contains("if cond {"));
        assert!(out.contains("return 1"));
        assert!(!out.contains("else"));
    }
    #[test]
    pub(super) fn test_swift_stmt_if_with_else() {
        let s = SwiftStmt::If {
            cond: SwiftExpr::SwiftVar("cond".to_string()),
            then_body: vec![SwiftStmt::Return(Some(SwiftExpr::SwiftLitExpr(
                SwiftLit::Int(1),
            )))],
            else_body: vec![SwiftStmt::Return(Some(SwiftExpr::SwiftLitExpr(
                SwiftLit::Int(0),
            )))],
        };
        let out = s.to_string();
        assert!(out.contains("} else {"));
    }
    #[test]
    pub(super) fn test_swift_stmt_guard() {
        let s = SwiftStmt::Guard {
            cond: SwiftExpr::SwiftVar("ok".to_string()),
            else_body: vec![SwiftStmt::Return(None)],
        };
        let out = s.to_string();
        assert!(out.contains("guard ok else {"));
        assert!(out.contains("return"));
    }
    #[test]
    pub(super) fn test_swift_stmt_for() {
        let s = SwiftStmt::For {
            name: "item".to_string(),
            collection: SwiftExpr::SwiftVar("items".to_string()),
            body: vec![SwiftStmt::Break],
        };
        let out = s.to_string();
        assert!(out.contains("for item in items {"));
        assert!(out.contains("break"));
    }
    #[test]
    pub(super) fn test_swift_stmt_while() {
        let s = SwiftStmt::While {
            cond: SwiftExpr::SwiftLitExpr(SwiftLit::Bool(true)),
            body: vec![SwiftStmt::Break],
        };
        let out = s.to_string();
        assert!(out.contains("while true {"));
    }
    #[test]
    pub(super) fn test_swift_param_simple() {
        let p = SwiftParam::new("x", SwiftType::SwiftInt);
        assert_eq!(p.to_string(), "x: Int");
    }
    #[test]
    pub(super) fn test_swift_param_labeled() {
        let p = SwiftParam::labeled("from", "start", SwiftType::SwiftInt);
        assert_eq!(p.to_string(), "from start: Int");
    }
    #[test]
    pub(super) fn test_swift_param_variadic() {
        let mut p = SwiftParam::new("args", SwiftType::SwiftInt);
        p.variadic = true;
        assert_eq!(p.to_string(), "args: Int...");
    }
    #[test]
    pub(super) fn test_swift_func_simple() {
        let mut f = SwiftFunc::new("greet", SwiftType::SwiftVoid);
        f.body = vec![SwiftStmt::ExprStmt(SwiftExpr::SwiftCall {
            callee: Box::new(SwiftExpr::SwiftVar("print".to_string())),
            args: vec![(
                "".to_string(),
                SwiftExpr::SwiftLitExpr(SwiftLit::Str("hi".to_string())),
            )],
        })];
        let out = f.codegen();
        assert!(out.contains("func greet()"));
        assert!(out.contains("print(\"hi\")"));
    }
    #[test]
    pub(super) fn test_swift_func_public_throws_async() {
        let mut f = SwiftFunc::new("fetchData", SwiftType::SwiftString);
        f.is_public = true;
        f.throws = true;
        f.is_async = true;
        let out = f.codegen();
        assert!(out.contains("public func fetchData()"));
        assert!(out.contains("async throws"));
        assert!(out.contains("-> String"));
    }
    #[test]
    pub(super) fn test_swift_func_with_generic() {
        let mut f = SwiftFunc::new("identity", SwiftType::SwiftNamed("T".to_string()));
        f.generic_params = vec!["T".to_string()];
        f.params = vec![SwiftParam::new(
            "value",
            SwiftType::SwiftNamed("T".to_string()),
        )];
        let out = f.codegen();
        assert!(out.contains("func identity<T>"));
        assert!(out.contains("-> T"));
    }
    #[test]
    pub(super) fn test_swift_enum_bare_cases() {
        let mut e = SwiftEnumDecl::new("Direction");
        e.cases.push(SwiftEnumCase::bare("north"));
        e.cases.push(SwiftEnumCase::bare("south"));
        let out = e.codegen();
        assert!(out.contains("enum Direction {"));
        assert!(out.contains("case north"));
        assert!(out.contains("case south"));
    }
    #[test]
    pub(super) fn test_swift_enum_associated_values() {
        let mut e = SwiftEnumDecl::new("Result");
        e.cases
            .push(SwiftEnumCase::with_values("ok", vec![SwiftType::SwiftInt]));
        e.cases.push(SwiftEnumCase::with_values(
            "err",
            vec![SwiftType::SwiftString],
        ));
        let out = e.codegen();
        assert!(out.contains("case ok(Int)"));
        assert!(out.contains("case err(String)"));
    }
    #[test]
    pub(super) fn test_swift_enum_public_generic() {
        let mut e = SwiftEnumDecl::new("Option");
        e.is_public = true;
        e.generic_params = vec!["T".to_string()];
        e.cases.push(SwiftEnumCase::bare("none"));
        e.cases.push(SwiftEnumCase::with_values(
            "some",
            vec![SwiftType::SwiftNamed("T".to_string())],
        ));
        let out = e.codegen();
        assert!(out.contains("public enum Option<T> {"));
    }
    #[test]
    pub(super) fn test_swift_struct_fields() {
        let mut s = SwiftStructDecl::new("Point");
        s.fields
            .push(SwiftField::new_let("x", SwiftType::SwiftDouble));
        s.fields
            .push(SwiftField::new_let("y", SwiftType::SwiftDouble));
        let out = s.codegen();
        assert!(out.contains("struct Point {"));
        assert!(out.contains("let x: Double"));
        assert!(out.contains("let y: Double"));
    }
    #[test]
    pub(super) fn test_swift_struct_conformance() {
        let mut s = SwiftStructDecl::new("Foo");
        s.conformances
            .push(SwiftConformance("Equatable".to_string()));
        s.conformances
            .push(SwiftConformance("Hashable".to_string()));
        let out = s.codegen();
        assert!(out.contains("struct Foo: Equatable, Hashable {"));
    }
    #[test]
    pub(super) fn test_swift_class_basic() {
        let mut c = SwiftClassDecl::new("Animal");
        c.fields
            .push(SwiftField::new_var("name", SwiftType::SwiftString));
        let out = c.codegen();
        assert!(out.contains("class Animal {"));
        assert!(out.contains("var name: String"));
    }
    #[test]
    pub(super) fn test_swift_class_final_public() {
        let mut c = SwiftClassDecl::new("Dog");
        c.is_final = true;
        c.is_public = true;
        c.superclass = Some("Animal".to_string());
        let out = c.codegen();
        assert!(out.contains("public final class Dog: Animal {"));
    }
    #[test]
    pub(super) fn test_swift_extension_basic() {
        let mut ext = SwiftExtension::new("Int");
        let mut f = SwiftFunc::new("doubled", SwiftType::SwiftInt);
        f.body = vec![SwiftStmt::Return(Some(SwiftExpr::SwiftBinOp {
            op: "*".to_string(),
            lhs: Box::new(SwiftExpr::SwiftSelf),
            rhs: Box::new(SwiftExpr::SwiftLitExpr(SwiftLit::Int(2))),
        }))];
        ext.methods.push(f);
        let out = ext.codegen();
        assert!(out.contains("extension Int {"));
        assert!(out.contains("func doubled()"));
    }
    #[test]
    pub(super) fn test_swift_module_imports_deduped() {
        let mut m = SwiftModule::new("Test");
        m.add_import("Foundation");
        m.add_import("Foundation");
        m.add_import("Swift");
        let out = m.codegen();
        assert_eq!(out.matches("import Foundation").count(), 1);
        assert!(out.contains("import Swift"));
    }
    #[test]
    pub(super) fn test_swift_module_contains_runtime() {
        let m = SwiftModule::new("Runtime");
        let out = m.codegen();
        assert!(out.contains("OxiLean Swift Runtime"));
        assert!(out.contains("enum OxValue"));
        assert!(out.contains("struct OxNat"));
        assert!(out.contains("struct OxError"));
    }
    #[test]
    pub(super) fn test_swift_module_func_and_type() {
        let mut m = SwiftModule::new("MyMod");
        let mut e = SwiftEnumDecl::new("Color");
        e.cases.push(SwiftEnumCase::bare("red"));
        m.types.push(SwiftTypeDecl::Enum(e));
        let f = SwiftFunc::new("noop", SwiftType::SwiftVoid);
        m.funcs.push(f);
        let out = m.codegen();
        assert!(out.contains("enum Color {"));
        assert!(out.contains("func noop()"));
    }
    #[test]
    pub(super) fn test_mangle_name_keywords() {
        for kw in SWIFT_KEYWORDS {
            let result = SwiftBackend::mangle_name(kw);
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
        assert_eq!(SwiftBackend::mangle_name("0abc"), "ox_0abc");
        assert_eq!(SwiftBackend::mangle_name("9"), "ox_9");
    }
    #[test]
    pub(super) fn test_mangle_name_empty() {
        assert_eq!(SwiftBackend::mangle_name(""), "ox_empty");
    }
    #[test]
    pub(super) fn test_mangle_name_special_chars() {
        assert_eq!(SwiftBackend::mangle_name("foo-bar"), "foo_bar");
        assert_eq!(SwiftBackend::mangle_name("a.b.c"), "a_b_c");
        assert_eq!(SwiftBackend::mangle_name("hello world"), "hello_world");
    }
    #[test]
    pub(super) fn test_mangle_name_valid_identifier() {
        assert_eq!(SwiftBackend::mangle_name("myFunc"), "myFunc");
        assert_eq!(SwiftBackend::mangle_name("_private"), "_private");
        assert_eq!(SwiftBackend::mangle_name("camelCase"), "camelCase");
    }
    #[test]
    pub(super) fn test_is_swift_keyword_true() {
        assert!(is_swift_keyword("func"));
        assert!(is_swift_keyword("let"));
        assert!(is_swift_keyword("var"));
        assert!(is_swift_keyword("class"));
        assert!(is_swift_keyword("return"));
        assert!(is_swift_keyword("throws"));
        assert!(is_swift_keyword("async"));
        assert!(is_swift_keyword("await"));
    }
    #[test]
    pub(super) fn test_is_swift_keyword_false() {
        assert!(!is_swift_keyword("myFunc"));
        assert!(!is_swift_keyword("data"));
        assert!(!is_swift_keyword("oxilean"));
    }
    #[test]
    pub(super) fn test_compile_decl_simple() {
        use crate::lcnf::{LcnfFunDecl, LcnfParam as LParam, LcnfVarId};
        let decl = LcnfFunDecl {
            name: "myFn".to_string(),
            original_name: None,
            params: vec![LParam {
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
        let backend = SwiftBackend::new();
        let func = backend.compile_decl(&decl);
        assert_eq!(func.name, "myFn");
        assert_eq!(func.params.len(), 1);
        let out = func.codegen();
        assert!(out.contains("func myFn("));
        assert!(out.contains("return _x0"));
    }
    #[test]
    pub(super) fn test_compile_module_empty() {
        let backend = SwiftBackend::new();
        let module = backend.compile_module("EmptyMod", &[]);
        let out = module.codegen();
        assert!(out.contains("OxiLean-generated Swift module: EmptyMod"));
        assert!(out.contains("import Foundation"));
    }
    #[test]
    pub(super) fn test_fresh_var() {
        let mut backend = SwiftBackend::new();
        assert_eq!(backend.fresh_var(), "_ox0");
        assert_eq!(backend.fresh_var(), "_ox1");
        assert_eq!(backend.fresh_var(), "_ox2");
    }
    #[test]
    pub(super) fn test_backend_default() {
        let b = SwiftBackend::default();
        assert!(b.emit_public);
        assert!(b.emit_comments);
    }
    #[test]
    pub(super) fn test_runtime_contains_ox_panic() {
        assert!(OXILEAN_SWIFT_RUNTIME.contains("func ox_panic"));
    }
    #[test]
    pub(super) fn test_runtime_contains_ox_unreachable() {
        assert!(OXILEAN_SWIFT_RUNTIME.contains("func ox_unreachable"));
    }
}
#[cfg(test)]
mod Swift_infra_tests {
    use super::*;
    #[test]
    pub(super) fn test_pass_config() {
        let config = SwiftPassConfig::new("test_pass", SwiftPassPhase::Transformation);
        assert!(config.enabled);
        assert!(config.phase.is_modifying());
        assert_eq!(config.phase.name(), "transformation");
    }
    #[test]
    pub(super) fn test_pass_stats() {
        let mut stats = SwiftPassStats::new();
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
        let mut reg = SwiftPassRegistry::new();
        reg.register(SwiftPassConfig::new("pass_a", SwiftPassPhase::Analysis));
        reg.register(SwiftPassConfig::new("pass_b", SwiftPassPhase::Transformation).disabled());
        assert_eq!(reg.total_passes(), 2);
        assert_eq!(reg.enabled_count(), 1);
        reg.update_stats("pass_a", 5, 50, 2);
        let stats = reg.get_stats("pass_a").expect("stats should exist");
        assert_eq!(stats.total_changes, 5);
    }
    #[test]
    pub(super) fn test_analysis_cache() {
        let mut cache = SwiftAnalysisCache::new(10);
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
        let mut wl = SwiftWorklist::new();
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
        let mut dt = SwiftDominatorTree::new(5);
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
        let mut liveness = SwiftLivenessInfo::new(3);
        liveness.add_def(0, 1);
        liveness.add_use(1, 1);
        assert!(liveness.defs[0].contains(&1));
        assert!(liveness.uses[1].contains(&1));
    }
    #[test]
    pub(super) fn test_constant_folding() {
        assert_eq!(SwiftConstantFoldingHelper::fold_add_i64(3, 4), Some(7));
        assert_eq!(SwiftConstantFoldingHelper::fold_div_i64(10, 0), None);
        assert_eq!(SwiftConstantFoldingHelper::fold_div_i64(10, 2), Some(5));
        assert_eq!(
            SwiftConstantFoldingHelper::fold_bitand_i64(0b1100, 0b1010),
            0b1000
        );
        assert_eq!(SwiftConstantFoldingHelper::fold_bitnot_i64(0), -1);
    }
    #[test]
    pub(super) fn test_dep_graph() {
        let mut g = SwiftDepGraph::new();
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
