//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;
use std::collections::HashSet;

use super::types::{
    RubyAnalysisCache, RubyBackend, RubyClass, RubyConstantFoldingHelper, RubyDepGraph,
    RubyDominatorTree, RubyExpr, RubyLit, RubyLivenessInfo, RubyMethod, RubyModule, RubyPassConfig,
    RubyPassPhase, RubyPassRegistry, RubyPassStats, RubyStmt, RubyType, RubyVisibility,
    RubyWorklist,
};
use std::fmt;

/// Map an LCNF type to a Ruby type annotation.
pub(super) fn lcnf_type_to_ruby(ty: &LcnfType) -> RubyType {
    match ty {
        LcnfType::Nat => RubyType::Integer,
        LcnfType::LcnfString => RubyType::String,
        LcnfType::Unit | LcnfType::Erased | LcnfType::Irrelevant => RubyType::Nil,
        LcnfType::Object => RubyType::Object("Object".to_string()),
        LcnfType::Var(name) => RubyType::Object(name.clone()),
        LcnfType::Fun(_, _) => RubyType::Proc,
        LcnfType::Ctor(name, _) => RubyType::Object(ruby_const_name(name)),
    }
}
pub(super) fn fmt_ruby_stmt(
    stmt: &RubyStmt,
    indent: &str,
    f: &mut fmt::Formatter<'_>,
) -> fmt::Result {
    let inner = format!("{}  ", indent);
    match stmt {
        RubyStmt::Expr(expr) => writeln!(f, "{}{}", indent, expr),
        RubyStmt::Assign(name, expr) => writeln!(f, "{}{} = {}", indent, name, expr),
        RubyStmt::Return(expr) => writeln!(f, "{}return {}", indent, expr),
        RubyStmt::Def(method) => fmt_ruby_method(method, indent, f),
        RubyStmt::Class(class) => fmt_ruby_class(class, indent, f),
        RubyStmt::Mod(module) => fmt_ruby_module_stmt(module, indent, f),
        RubyStmt::If(cond, then_stmts, elsif_branches, else_stmts) => {
            writeln!(f, "{}if {}", indent, cond)?;
            for s in then_stmts {
                fmt_ruby_stmt(s, &inner, f)?;
            }
            for (elsif_cond, elsif_body) in elsif_branches {
                writeln!(f, "{}elsif {}", indent, elsif_cond)?;
                for s in elsif_body {
                    fmt_ruby_stmt(s, &inner, f)?;
                }
            }
            if let Some(else_body) = else_stmts {
                writeln!(f, "{}else", indent)?;
                for s in else_body {
                    fmt_ruby_stmt(s, &inner, f)?;
                }
            }
            writeln!(f, "{}end", indent)
        }
        RubyStmt::While(cond, body) => {
            writeln!(f, "{}while {}", indent, cond)?;
            for s in body {
                fmt_ruby_stmt(s, &inner, f)?;
            }
            writeln!(f, "{}end", indent)
        }
        RubyStmt::Begin(body, rescue, ensure) => {
            writeln!(f, "{}begin", indent)?;
            for s in body {
                fmt_ruby_stmt(s, &inner, f)?;
            }
            if let Some((exc_var, rescue_body)) = rescue {
                writeln!(f, "{}rescue => {}", indent, exc_var)?;
                for s in rescue_body {
                    fmt_ruby_stmt(s, &inner, f)?;
                }
            }
            if let Some(ensure_body) = ensure {
                writeln!(f, "{}ensure", indent)?;
                for s in ensure_body {
                    fmt_ruby_stmt(s, &inner, f)?;
                }
            }
            writeln!(f, "{}end", indent)
        }
    }
}
pub(super) fn fmt_ruby_method(
    method: &RubyMethod,
    indent: &str,
    f: &mut fmt::Formatter<'_>,
) -> fmt::Result {
    let inner = format!("{}  ", indent);
    if method.visibility != RubyVisibility::Public {
        writeln!(f, "{}{}", indent, method.visibility)?;
    }
    write!(f, "{}def {}", indent, method.name)?;
    if !method.params.is_empty() {
        write!(f, "({})", method.params.join(", "))?;
    }
    writeln!(f)?;
    for stmt in &method.body {
        fmt_ruby_stmt(stmt, &inner, f)?;
    }
    writeln!(f, "{}end", indent)
}
pub(super) fn fmt_ruby_class(
    class: &RubyClass,
    indent: &str,
    f: &mut fmt::Formatter<'_>,
) -> fmt::Result {
    let inner = format!("{}  ", indent);
    match &class.superclass {
        Some(sup) => writeln!(f, "{}class {} < {}", indent, class.name, sup)?,
        None => writeln!(f, "{}class {}", indent, class.name)?,
    }
    if !class.attr_readers.is_empty() {
        let readers: Vec<&str> = class.attr_readers.iter().map(|s| s.as_str()).collect();
        let syms: Vec<std::string::String> = readers.iter().map(|s| format!(":{}", s)).collect();
        writeln!(f, "{}attr_reader {}", inner, syms.join(", "))?;
    }
    if !class.attr_writers.is_empty() {
        let syms: Vec<std::string::String> = class
            .attr_writers
            .iter()
            .map(|s| format!(":{}", s))
            .collect();
        writeln!(f, "{}attr_writer {}", inner, syms.join(", "))?;
    }
    for method in &class.class_methods {
        let self_method = RubyMethod {
            name: format!("self.{}", method.name),
            ..method.clone()
        };
        fmt_ruby_method(&self_method, &inner, f)?;
    }
    for method in &class.methods {
        fmt_ruby_method(method, &inner, f)?;
    }
    writeln!(f, "{}end", indent)
}
pub(super) fn fmt_ruby_module_stmt(
    module: &RubyModule,
    indent: &str,
    f: &mut fmt::Formatter<'_>,
) -> fmt::Result {
    let inner = format!("{}  ", indent);
    writeln!(f, "{}module {}", indent, module.name)?;
    for (name, expr) in &module.constants {
        writeln!(f, "{}{} = {}", inner, name, expr)?;
    }
    if !module.functions.is_empty() {
        if module.module_function {
            writeln!(f, "{}module_function", inner)?;
            writeln!(f)?;
        }
        for method in &module.functions {
            fmt_ruby_method(method, &inner, f)?;
        }
    }
    for class in &module.classes {
        fmt_ruby_class(class, &inner, f)?;
    }
    writeln!(f, "{}end", indent)
}
/// Ruby reserved keywords that cannot be used as identifiers.
const RUBY_KEYWORDS: &[&str] = &[
    "alias", "and", "begin", "break", "case", "class", "def", "defined", "do", "else", "elsif",
    "end", "ensure", "false", "for", "if", "in", "module", "next", "nil", "not", "or", "redo",
    "rescue", "retry", "return", "self", "super", "then", "true", "undef", "unless", "until",
    "when", "while", "yield",
];
/// Convert a potentially dotted/primed LCNF name to a valid Ruby snake_case identifier.
pub(super) fn ruby_mangle(name: &str) -> std::string::String {
    if name.is_empty() {
        return "_anon".to_string();
    }
    let mut result = std::string::String::new();
    for c in name.chars() {
        match c {
            '.' | ':' => result.push('_'),
            '\'' => result.push('_'),
            c if c.is_alphanumeric() || c == '_' => result.push(c),
            _ => result.push('_'),
        }
    }
    if result.starts_with(|c: char| c.is_ascii_digit()) {
        result.insert(0, '_');
    }
    if RUBY_KEYWORDS.contains(&result.as_str()) {
        result.insert(0, '_');
    }
    result
}
/// Convert a constructor/class name to Ruby CamelCase constant.
pub(super) fn ruby_const_name(name: &str) -> std::string::String {
    if name.is_empty() {
        return "Anon".to_string();
    }
    let parts: Vec<&str> = name.split(['.', '_']).collect();
    parts
        .iter()
        .map(|p| {
            let mut s = std::string::String::new();
            let mut first = true;
            for c in p.chars() {
                if first {
                    for upper in c.to_uppercase() {
                        s.push(upper);
                    }
                    first = false;
                } else {
                    s.push(c);
                }
            }
            s
        })
        .collect::<Vec<_>>()
        .join("")
}
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
        LcnfLetValue::Ctor(name, _, _) | LcnfLetValue::Reuse(_, name, _, _) => {
            out.insert(name.clone());
        }
        _ => {}
    }
}
/// Minimal Ruby runtime module emitted before the generated code.
pub const RUBY_RUNTIME: &str = r#"# frozen_string_literal: true
# OxiLean Ruby Runtime — auto-generated

module OxiLeanRuntime
  module_function

  # Natural-number addition (Ruby Integer is arbitrary-precision)
  def nat_add(a, b) = a + b

  # Natural-number saturating subtraction
  def nat_sub(a, b) = [0, a - b].max

  # Natural-number multiplication
  def nat_mul(a, b) = a * b

  # Natural-number division (truncating, div-by-zero → 0)
  def nat_div(a, b) = b.zero? ? 0 : a / b

  # Natural-number modulo (div-by-zero → a)
  def nat_mod(a, b) = b.zero? ? a : a % b

  # Decidable boolean → 0/1 as Integer
  def decide(b) = b ? 1 : 0

  # Natural number to string
  def nat_to_string(n) = n.to_s

  # String append
  def str_append(a, b) = a + b

  # String length
  def str_length(s) = s.length

  # List cons: prepend element
  def cons(head, tail) = [head, *tail]

  # List nil: empty list
  def nil_list = []

  # Pair constructor
  def mk_pair(a, b) = [a, b]

  # Unreachable branch
  def unreachable! = raise(RuntimeError, "OxiLean: unreachable code reached")
end
"#;
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub(super) fn test_ruby_type_display_integer() {
        assert_eq!(RubyType::Integer.to_string(), "Integer");
    }
    #[test]
    pub(super) fn test_ruby_type_display_float() {
        assert_eq!(RubyType::Float.to_string(), "Float");
    }
    #[test]
    pub(super) fn test_ruby_type_display_array() {
        let ty = RubyType::Array(Box::new(RubyType::Integer));
        assert_eq!(ty.to_string(), "Array[Integer]");
    }
    #[test]
    pub(super) fn test_ruby_type_display_hash() {
        let ty = RubyType::Hash(Box::new(RubyType::Symbol), Box::new(RubyType::String));
        assert_eq!(ty.to_string(), "Hash[Symbol, String]");
    }
    #[test]
    pub(super) fn test_ruby_type_display_proc() {
        assert_eq!(RubyType::Proc.to_string(), "Proc");
    }
    #[test]
    pub(super) fn test_ruby_lit_int() {
        assert_eq!(RubyLit::Int(42).to_string(), "42");
        assert_eq!(RubyLit::Int(-7).to_string(), "-7");
    }
    #[test]
    pub(super) fn test_ruby_lit_float() {
        assert_eq!(RubyLit::Float(1.0).to_string(), "1.0");
        assert_eq!(RubyLit::Float(3.14).to_string(), "3.14");
    }
    #[test]
    pub(super) fn test_ruby_lit_str_escape() {
        let lit = RubyLit::Str("hello \"world\"\nnewline".to_string());
        assert_eq!(lit.to_string(), "\"hello \\\"world\\\"\\nnewline\"");
    }
    #[test]
    pub(super) fn test_ruby_lit_str_hash_escape() {
        let lit = RubyLit::Str("a#b".to_string());
        assert_eq!(lit.to_string(), "\"a\\#b\"");
    }
    #[test]
    pub(super) fn test_ruby_lit_bool() {
        assert_eq!(RubyLit::Bool(true).to_string(), "true");
        assert_eq!(RubyLit::Bool(false).to_string(), "false");
    }
    #[test]
    pub(super) fn test_ruby_lit_nil() {
        assert_eq!(RubyLit::Nil.to_string(), "nil");
    }
    #[test]
    pub(super) fn test_ruby_lit_symbol() {
        assert_eq!(RubyLit::Symbol("foo".to_string()).to_string(), ":foo");
    }
    #[test]
    pub(super) fn test_ruby_expr_binop() {
        let expr = RubyExpr::BinOp(
            "+".to_string(),
            Box::new(RubyExpr::Lit(RubyLit::Int(1))),
            Box::new(RubyExpr::Lit(RubyLit::Int(2))),
        );
        assert_eq!(expr.to_string(), "(1 + 2)");
    }
    #[test]
    pub(super) fn test_ruby_expr_call() {
        let expr = RubyExpr::Call(
            "puts".to_string(),
            vec![RubyExpr::Lit(RubyLit::Str("hi".to_string()))],
        );
        assert_eq!(expr.to_string(), "puts(\"hi\")");
    }
    #[test]
    pub(super) fn test_ruby_expr_method_call() {
        let expr = RubyExpr::MethodCall(
            Box::new(RubyExpr::Var("arr".to_string())),
            "map".to_string(),
            vec![RubyExpr::Lit(RubyLit::Symbol("to_s".to_string()))],
        );
        assert_eq!(expr.to_string(), "arr.map(:to_s)");
    }
    #[test]
    pub(super) fn test_ruby_expr_array() {
        let expr = RubyExpr::Array(vec![
            RubyExpr::Lit(RubyLit::Int(1)),
            RubyExpr::Lit(RubyLit::Int(2)),
            RubyExpr::Lit(RubyLit::Int(3)),
        ]);
        assert_eq!(expr.to_string(), "[1, 2, 3]");
    }
    #[test]
    pub(super) fn test_ruby_expr_hash_symbol_key() {
        let expr = RubyExpr::Hash(vec![(
            RubyExpr::Lit(RubyLit::Symbol("name".to_string())),
            RubyExpr::Lit(RubyLit::Str("Alice".to_string())),
        )]);
        assert_eq!(expr.to_string(), "{name: \"Alice\"}");
    }
    #[test]
    pub(super) fn test_ruby_expr_hash_string_key() {
        let expr = RubyExpr::Hash(vec![(
            RubyExpr::Lit(RubyLit::Str("key".to_string())),
            RubyExpr::Lit(RubyLit::Int(42)),
        )]);
        assert_eq!(expr.to_string(), "{\"key\" => 42}");
    }
    #[test]
    pub(super) fn test_ruby_expr_lambda() {
        let expr = RubyExpr::Lambda(
            vec!["x".to_string(), "y".to_string()],
            vec![RubyStmt::Return(RubyExpr::BinOp(
                "+".to_string(),
                Box::new(RubyExpr::Var("x".to_string())),
                Box::new(RubyExpr::Var("y".to_string())),
            ))],
        );
        let s = expr.to_string();
        assert!(s.contains("->(x, y)"), "Expected lambda params, got: {}", s);
        assert!(s.contains("x + y"), "Expected body, got: {}", s);
    }
    #[test]
    pub(super) fn test_ruby_expr_ternary() {
        let expr = RubyExpr::If(
            Box::new(RubyExpr::Var("flag".to_string())),
            Box::new(RubyExpr::Lit(RubyLit::Int(1))),
            Box::new(RubyExpr::Lit(RubyLit::Int(0))),
        );
        assert_eq!(expr.to_string(), "(flag ? 1 : 0)");
    }
    #[test]
    pub(super) fn test_ruby_method_display() {
        let method = RubyMethod::new(
            "add",
            vec!["a", "b"],
            vec![RubyStmt::Return(RubyExpr::BinOp(
                "+".to_string(),
                Box::new(RubyExpr::Var("a".to_string())),
                Box::new(RubyExpr::Var("b".to_string())),
            ))],
        );
        let s = method.to_string();
        assert!(s.contains("def add(a, b)"), "Expected def, got: {}", s);
        assert!(s.contains("return (a + b)"), "Expected return, got: {}", s);
        assert!(s.contains("end"), "Expected end, got: {}", s);
    }
    #[test]
    pub(super) fn test_ruby_method_private_display() {
        let method = RubyMethod::private("secret", vec![], vec![]);
        let s = method.to_string();
        assert!(
            s.contains("private"),
            "Expected private visibility, got: {}",
            s
        );
        assert!(s.contains("def secret"), "Expected def secret, got: {}", s);
    }
    #[test]
    pub(super) fn test_ruby_class_display() {
        let mut class = RubyClass::new("Animal");
        class.add_attr_reader("name");
        class.add_method(RubyMethod::new(
            "speak",
            vec![],
            vec![RubyStmt::Return(RubyExpr::Lit(RubyLit::Str(
                "...".to_string(),
            )))],
        ));
        let s = class.to_string();
        assert!(
            s.contains("class Animal"),
            "Expected class Animal, got: {}",
            s
        );
        assert!(
            s.contains("attr_reader :name"),
            "Expected attr_reader, got: {}",
            s
        );
        assert!(s.contains("def speak"), "Expected def speak, got: {}", s);
        assert!(s.contains("end"), "Expected end, got: {}", s);
    }
    #[test]
    pub(super) fn test_ruby_class_with_superclass() {
        let class = RubyClass::new("Dog").with_superclass("Animal");
        let s = class.to_string();
        assert!(
            s.contains("class Dog < Animal"),
            "Expected inheritance, got: {}",
            s
        );
    }
    #[test]
    pub(super) fn test_ruby_module_emit_frozen_literal() {
        let module = RubyModule::new("MyLib");
        let src = module.emit();
        assert!(
            src.starts_with("# frozen_string_literal: true"),
            "Expected frozen string literal pragma, got: {}",
            &src[..50.min(src.len())]
        );
    }
    #[test]
    pub(super) fn test_ruby_module_emit_structure() {
        let mut module = RubyModule::new("OxiLean");
        module.functions.push(RubyMethod::new(
            "hello",
            vec![],
            vec![RubyStmt::Return(RubyExpr::Lit(RubyLit::Str(
                "world".to_string(),
            )))],
        ));
        let src = module.emit();
        assert!(
            src.contains("module OxiLean"),
            "Expected module OxiLean, got: {}",
            src
        );
        assert!(
            src.contains("module_function"),
            "Expected module_function, got: {}",
            src
        );
        assert!(
            src.contains("def hello"),
            "Expected def hello, got: {}",
            src
        );
        assert!(src.contains("end"), "Expected end, got: {}", src);
    }
    #[test]
    pub(super) fn test_ruby_mangle_dotted() {
        assert_eq!(ruby_mangle("Nat.add"), "Nat_add");
        assert_eq!(ruby_mangle("List.cons"), "List_cons");
    }
    #[test]
    pub(super) fn test_ruby_mangle_prime() {
        assert_eq!(ruby_mangle("foo'"), "foo_");
    }
    #[test]
    pub(super) fn test_ruby_mangle_keyword() {
        assert_eq!(ruby_mangle("return"), "_return");
        assert_eq!(ruby_mangle("class"), "_class");
        assert_eq!(ruby_mangle("end"), "_end");
    }
    #[test]
    pub(super) fn test_ruby_mangle_empty() {
        assert_eq!(ruby_mangle(""), "_anon");
    }
    #[test]
    pub(super) fn test_ruby_const_name() {
        assert_eq!(ruby_const_name("some"), "Some");
        assert_eq!(ruby_const_name("list.nil"), "ListNil");
        assert_eq!(ruby_const_name("nat_add"), "NatAdd");
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
        let mut backend = RubyBackend::new();
        let method = backend.compile_decl(&decl).expect("compile failed");
        assert_eq!(method.name, "answer");
        assert!(method.params.is_empty());
        let s = method.to_string();
        assert!(s.contains("return 42"), "Expected return 42, got: {}", s);
    }
    #[test]
    pub(super) fn test_compile_let_binding() {
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
        let mut backend = RubyBackend::new();
        let method = backend.compile_decl(&decl).expect("compile failed");
        let s = method.to_string();
        assert!(
            s.contains("def double(x)"),
            "Expected def double(x), got: {}",
            s
        );
        assert!(s.contains("y ="), "Expected y = assignment, got: {}", s);
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
        let src = RubyBackend::emit_module(&[decl]).expect("emit failed");
        assert!(src.contains("OxiLeanRuntime"), "Missing runtime module");
        assert!(src.contains("nat_add"), "Missing nat_add runtime helper");
        assert!(src.contains("def main"), "Missing main method");
        assert!(src.contains("module OxiLean"), "Missing OxiLean module");
    }
    #[test]
    pub(super) fn test_mangle_name_caching() {
        let mut backend = RubyBackend::new();
        let a = backend.mangle_name("Nat.add");
        let b = backend.mangle_name("Nat.add");
        assert_eq!(a, b);
        assert_eq!(a, "Nat_add");
    }
    #[test]
    pub(super) fn test_lcnf_type_nat_to_ruby() {
        assert_eq!(lcnf_type_to_ruby(&LcnfType::Nat), RubyType::Integer);
    }
    #[test]
    pub(super) fn test_lcnf_type_string_to_ruby() {
        assert_eq!(lcnf_type_to_ruby(&LcnfType::LcnfString), RubyType::String);
    }
    #[test]
    pub(super) fn test_lcnf_type_unit_to_ruby() {
        assert_eq!(lcnf_type_to_ruby(&LcnfType::Unit), RubyType::Nil);
    }
}
/// Ruby pass version
#[allow(dead_code)]
pub const RUBY_PASS_VERSION: &str = "1.0.0";
/// Ruby version string
#[allow(dead_code)]
pub const RUBY_BACKEND_VERSION: &str = "1.0.0";
/// Ruby min version supported
#[allow(dead_code)]
pub const RUBY_MIN_VERSION: &str = "3.0";
/// Ruby frozen literal constants
#[allow(dead_code)]
pub fn ruby_frozen_str(s: &str) -> String {
    format!("{}.freeze", s)
}
/// Ruby safe navigation operator
#[allow(dead_code)]
pub fn ruby_safe_nav(obj: &str, method: &str) -> String {
    format!("{}?.{}", obj, method)
}
/// Ruby tap helper
#[allow(dead_code)]
pub fn ruby_tap(expr: &str, block: &str) -> String {
    format!("{}.tap {{ |it| {} }}", expr, block)
}
/// Ruby then/yield_self
#[allow(dead_code)]
pub fn ruby_then(expr: &str, block: &str) -> String {
    format!("{}.then {{ |it| {} }}", expr, block)
}
/// Ruby memoize pattern
#[allow(dead_code)]
pub fn ruby_memoize(ivar: &str, expr: &str) -> String {
    format!("{} ||= {}", ivar, expr)
}
/// Ruby double splat
#[allow(dead_code)]
pub fn ruby_double_splat(hash: &str) -> String {
    format!("**{}", hash)
}
/// Ruby format string (Kernel#format)
#[allow(dead_code)]
pub fn ruby_format(template: &str, args: &[&str]) -> String {
    let args_str = args.join(", ");
    format!("format({:?}, {})", template, args_str)
}
/// Ruby heredoc
#[allow(dead_code)]
pub fn ruby_heredoc(label: &str, content: &str) -> String {
    format!("<<~{}\n{}{}\n", label, content, label)
}
/// Ruby method missing
#[allow(dead_code)]
pub fn ruby_method_missing(name_var: &str, args_var: &str, block_var: &str, body: &str) -> String {
    format!(
        "def method_missing({}, *{}, &{})\n  {}\nend",
        name_var, args_var, block_var, body
    )
}
/// Ruby respond_to_missing?
#[allow(dead_code)]
pub fn ruby_respond_to_missing(name_var: &str, include_private: &str, body: &str) -> String {
    format!(
        "def respond_to_missing?({}, {} = false)\n  {}\nend",
        name_var, include_private, body
    )
}
/// Ruby concurrent-ruby primitives
#[allow(dead_code)]
pub fn ruby_concurrent_promise(body: &str) -> String {
    format!("Concurrent::Promise.execute {{ {} }}", body)
}
#[allow(dead_code)]
pub fn ruby_concurrent_future(body: &str) -> String {
    format!("Concurrent::Future.execute {{ {} }}", body)
}
/// Ruby Comparable mixin helper
#[allow(dead_code)]
pub fn ruby_comparable_impl(spaceship_body: &str) -> String {
    format!(
        "include Comparable\n\ndef <=>(other)\n  {}\nend",
        spaceship_body
    )
}
/// Ruby Enumerable mixin helper
#[allow(dead_code)]
pub fn ruby_enumerable_impl(each_body: &str) -> String {
    format!(
        "include Enumerable\n\ndef each(&block)\n  {}\nend",
        each_body
    )
}
/// Ruby ObjectSpace finalizer
#[allow(dead_code)]
pub fn ruby_define_finalizer(var: &str, finalizer: &str) -> String {
    format!("ObjectSpace.define_finalizer({}, {})", var, finalizer)
}
/// Ruby backend version
#[allow(dead_code)]
pub const RUBY_BACKEND_PASS_VERSION: &str = "1.0.0";
#[cfg(test)]
mod Ruby_infra_tests {
    use super::*;
    #[test]
    pub(super) fn test_pass_config() {
        let config = RubyPassConfig::new("test_pass", RubyPassPhase::Transformation);
        assert!(config.enabled);
        assert!(config.phase.is_modifying());
        assert_eq!(config.phase.name(), "transformation");
    }
    #[test]
    pub(super) fn test_pass_stats() {
        let mut stats = RubyPassStats::new();
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
        let mut reg = RubyPassRegistry::new();
        reg.register(RubyPassConfig::new("pass_a", RubyPassPhase::Analysis));
        reg.register(RubyPassConfig::new("pass_b", RubyPassPhase::Transformation).disabled());
        assert_eq!(reg.total_passes(), 2);
        assert_eq!(reg.enabled_count(), 1);
        reg.update_stats("pass_a", 5, 50, 2);
        let stats = reg.get_stats("pass_a").expect("stats should exist");
        assert_eq!(stats.total_changes, 5);
    }
    #[test]
    pub(super) fn test_analysis_cache() {
        let mut cache = RubyAnalysisCache::new(10);
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
        let mut wl = RubyWorklist::new();
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
        let mut dt = RubyDominatorTree::new(5);
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
        let mut liveness = RubyLivenessInfo::new(3);
        liveness.add_def(0, 1);
        liveness.add_use(1, 1);
        assert!(liveness.defs[0].contains(&1));
        assert!(liveness.uses[1].contains(&1));
    }
    #[test]
    pub(super) fn test_constant_folding() {
        assert_eq!(RubyConstantFoldingHelper::fold_add_i64(3, 4), Some(7));
        assert_eq!(RubyConstantFoldingHelper::fold_div_i64(10, 0), None);
        assert_eq!(RubyConstantFoldingHelper::fold_div_i64(10, 2), Some(5));
        assert_eq!(
            RubyConstantFoldingHelper::fold_bitand_i64(0b1100, 0b1010),
            0b1000
        );
        assert_eq!(RubyConstantFoldingHelper::fold_bitnot_i64(0), -1);
    }
    #[test]
    pub(super) fn test_dep_graph() {
        let mut g = RubyDepGraph::new();
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
/// Ruby concurrent array access pattern
#[allow(dead_code)]
pub fn ruby_thread_safe_array_read(arr: &str, idx: &str) -> String {
    format!("{}.synchronize {{ {}[{}] }}", arr, arr, idx)
}
/// Ruby mutable default args (common Ruby gotcha)
#[allow(dead_code)]
pub fn ruby_warn_mutable_default(param: &str) -> String {
    format!("# Warning: mutable default for {}", param)
}
/// Ruby inline C extension helper
#[allow(dead_code)]
pub fn ruby_inline_c(c_body: &str) -> String {
    format!(
        "require 'inline'\ninline do |builder|\n  builder.c <<~C\n    {}\n  C\nend",
        c_body
    )
}
