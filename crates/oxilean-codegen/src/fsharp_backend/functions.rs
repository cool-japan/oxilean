//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{
    ActivePatternKind, ComputationExprKind, FSharpActivePattern, FSharpAttribute, FSharpBackend,
    FSharpExpr, FSharpFunction, FSharpFunctionBuilder, FSharpInterface, FSharpMeasure,
    FSharpModule, FSharpModuleBuilder, FSharpModuleMetrics, FSharpMutualGroup,
    FSharpNumericHelpers, FSharpPattern, FSharpRecord, FSharpSnippets, FSharpStdLib, FSharpType,
    FSharpTypeAlias, FSharpUnion, FSharpUnionCase, FSharpUnionCaseNamed,
};

/// Wrap a type in parentheses if it contains arrows (for clarity in nested types).
pub(super) fn paren_type(ty: &FSharpType) -> String {
    match ty {
        FSharpType::Fun(_, _) | FSharpType::Tuple(_) => format!("({})", ty),
        _ => format!("{}", ty),
    }
}
/// Wrap a function type in parens on the LHS of `->`.
pub(super) fn paren_fun_type(ty: &FSharpType) -> String {
    match ty {
        FSharpType::Fun(_, _) => format!("({})", ty),
        _ => format!("{}", ty),
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    pub(super) fn b() -> FSharpBackend {
        FSharpBackend::new()
    }
    #[test]
    pub(super) fn test_type_int() {
        assert_eq!(b().emit_type(&FSharpType::Int), "int");
    }
    #[test]
    pub(super) fn test_type_fun() {
        let ty = FSharpType::Fun(Box::new(FSharpType::Int), Box::new(FSharpType::Bool));
        assert_eq!(b().emit_type(&ty), "int -> bool");
    }
    #[test]
    pub(super) fn test_type_list() {
        let ty = FSharpType::List(Box::new(FSharpType::FsString));
        assert_eq!(b().emit_type(&ty), "string list");
    }
    #[test]
    pub(super) fn test_type_option() {
        let ty = FSharpType::Option(Box::new(FSharpType::Int));
        assert_eq!(b().emit_type(&ty), "int option");
    }
    #[test]
    pub(super) fn test_type_tuple() {
        let ty = FSharpType::Tuple(vec![FSharpType::Int, FSharpType::Bool]);
        assert_eq!(b().emit_type(&ty), "int * bool");
    }
    #[test]
    pub(super) fn test_type_generic() {
        let ty = FSharpType::Generic(
            "Map".to_string(),
            vec![FSharpType::FsString, FSharpType::Int],
        );
        assert_eq!(b().emit_type(&ty), "Map<string, int>");
    }
    #[test]
    pub(super) fn test_expr_lit() {
        assert_eq!(b().emit_expr(&FSharpExpr::Lit("42".to_string()), 0), "42");
    }
    #[test]
    pub(super) fn test_expr_var() {
        assert_eq!(b().emit_expr(&FSharpExpr::Var("x".to_string()), 0), "x");
    }
    #[test]
    pub(super) fn test_expr_lambda() {
        let e = FSharpExpr::Lambda("x".to_string(), Box::new(FSharpExpr::Var("x".to_string())));
        assert_eq!(b().emit_expr(&e, 0), "fun x -> x");
    }
    #[test]
    pub(super) fn test_expr_binop() {
        let e = FSharpExpr::BinOp(
            "+".to_string(),
            Box::new(FSharpExpr::Lit("1".to_string())),
            Box::new(FSharpExpr::Lit("2".to_string())),
        );
        assert_eq!(b().emit_expr(&e, 0), "1 + 2");
    }
    #[test]
    pub(super) fn test_expr_list() {
        let e = FSharpExpr::FsList(vec![
            FSharpExpr::Lit("1".to_string()),
            FSharpExpr::Lit("2".to_string()),
        ]);
        assert_eq!(b().emit_expr(&e, 0), "[1; 2]");
    }
    #[test]
    pub(super) fn test_expr_empty_list() {
        assert_eq!(b().emit_expr(&FSharpExpr::FsList(vec![]), 0), "[]");
    }
    #[test]
    pub(super) fn test_expr_array() {
        let e = FSharpExpr::FsArray(vec![FSharpExpr::Lit("3".to_string())]);
        assert_eq!(b().emit_expr(&e, 0), "[|3|]");
    }
    #[test]
    pub(super) fn test_expr_tuple() {
        let e = FSharpExpr::Tuple(vec![
            FSharpExpr::Lit("1".to_string()),
            FSharpExpr::Lit("2".to_string()),
        ]);
        assert_eq!(b().emit_expr(&e, 0), "(1, 2)");
    }
    #[test]
    pub(super) fn test_expr_record() {
        let e = FSharpExpr::Record(vec![
            ("x".to_string(), FSharpExpr::Lit("0".to_string())),
            ("y".to_string(), FSharpExpr::Lit("1".to_string())),
        ]);
        let s = b().emit_expr(&e, 0);
        assert!(s.contains("x = 0"), "got: {}", s);
        assert!(s.contains("y = 1"), "got: {}", s);
    }
    #[test]
    pub(super) fn test_expr_pipe() {
        let e = FSharpExpr::Pipe(
            Box::new(FSharpExpr::Var("xs".to_string())),
            Box::new(FSharpExpr::Var("List.sort".to_string())),
        );
        let s = b().emit_expr(&e, 0);
        assert!(s.contains("|>"), "pipe missing: {}", s);
        assert!(s.contains("xs"), "lhs missing: {}", s);
    }
    #[test]
    pub(super) fn test_expr_match() {
        let e = FSharpExpr::Match(
            Box::new(FSharpExpr::Var("x".to_string())),
            vec![
                (
                    FSharpPattern::Lit("0".to_string()),
                    FSharpExpr::Lit("\"zero\"".to_string()),
                ),
                (
                    FSharpPattern::Wildcard,
                    FSharpExpr::Lit("\"nonzero\"".to_string()),
                ),
            ],
        );
        let s = b().emit_expr(&e, 0);
        assert!(s.contains("match x with"), "got: {}", s);
        assert!(s.contains("| 0 -> "), "got: {}", s);
        assert!(s.contains("| _ -> "), "got: {}", s);
    }
    #[test]
    pub(super) fn test_expr_if() {
        let e = FSharpExpr::If(
            Box::new(FSharpExpr::Lit("true".to_string())),
            Box::new(FSharpExpr::Lit("1".to_string())),
            Box::new(FSharpExpr::Lit("0".to_string())),
        );
        let s = b().emit_expr(&e, 0);
        assert!(s.contains("if true then"), "got: {}", s);
        assert!(s.contains("else"), "got: {}", s);
    }
    #[test]
    pub(super) fn test_pattern_ctor() {
        let p = FSharpPattern::Ctor(
            "Some".to_string(),
            vec![FSharpPattern::Var("v".to_string())],
        );
        assert_eq!(b().emit_pattern(&p), "Some(v)");
    }
    #[test]
    pub(super) fn test_pattern_cons() {
        let p = FSharpPattern::Cons(
            Box::new(FSharpPattern::Var("h".to_string())),
            Box::new(FSharpPattern::Var("t".to_string())),
        );
        assert_eq!(b().emit_pattern(&p), "h :: t");
    }
    #[test]
    pub(super) fn test_emit_record() {
        let rec = FSharpRecord {
            name: "Point".to_string(),
            type_params: vec![],
            fields: vec![
                ("x".to_string(), FSharpType::Float),
                ("y".to_string(), FSharpType::Float),
            ],
            doc: None,
        };
        let s = b().emit_record(&rec);
        assert!(s.contains("type Point ="), "got: {}", s);
        assert!(s.contains("x: float"), "got: {}", s);
        assert!(s.contains("y: float"), "got: {}", s);
    }
    #[test]
    pub(super) fn test_emit_union() {
        let union = FSharpUnion {
            name: "Shape".to_string(),
            type_params: vec![],
            cases: vec![
                FSharpUnionCase {
                    name: "Circle".to_string(),
                    fields: vec![FSharpType::Float],
                },
                FSharpUnionCase {
                    name: "Rect".to_string(),
                    fields: vec![FSharpType::Float, FSharpType::Float],
                },
                FSharpUnionCase {
                    name: "Point".to_string(),
                    fields: vec![],
                },
            ],
            doc: None,
        };
        let s = b().emit_union(&union);
        assert!(s.contains("type Shape ="), "got: {}", s);
        assert!(s.contains("| Circle of float"), "got: {}", s);
        assert!(s.contains("| Rect of float * float"), "got: {}", s);
        assert!(s.contains("| Point"), "got: {}", s);
    }
    #[test]
    pub(super) fn test_emit_function() {
        let func = FSharpFunction {
            name: "add".to_string(),
            is_recursive: false,
            is_inline: false,
            type_params: vec![],
            params: vec![
                ("a".to_string(), Some(FSharpType::Int)),
                ("b".to_string(), Some(FSharpType::Int)),
            ],
            return_type: Some(FSharpType::Int),
            body: FSharpExpr::BinOp(
                "+".to_string(),
                Box::new(FSharpExpr::Var("a".to_string())),
                Box::new(FSharpExpr::Var("b".to_string())),
            ),
            doc: None,
        };
        let s = b().emit_function(&func);
        assert!(s.contains("let add"), "got: {}", s);
        assert!(s.contains("(a: int)"), "got: {}", s);
        assert!(s.contains(": int"), "got: {}", s);
        assert!(s.contains("a + b"), "got: {}", s);
    }
    #[test]
    pub(super) fn test_emit_module() {
        let module = FSharpModule {
            name: "OxiLean.Math".to_string(),
            auto_open: false,
            records: vec![],
            unions: vec![],
            functions: vec![FSharpFunction {
                name: "square".to_string(),
                is_recursive: false,
                is_inline: false,
                type_params: vec![],
                params: vec![("x".to_string(), Some(FSharpType::Int))],
                return_type: Some(FSharpType::Int),
                body: FSharpExpr::BinOp(
                    "*".to_string(),
                    Box::new(FSharpExpr::Var("x".to_string())),
                    Box::new(FSharpExpr::Var("x".to_string())),
                ),
                doc: None,
            }],
            opens: vec!["System".to_string()],
        };
        let s = b().emit_module(&module);
        assert!(s.contains("module OxiLean.Math"), "got: {}", s);
        assert!(s.contains("open System"), "got: {}", s);
        assert!(s.contains("let square"), "got: {}", s);
        assert!(s.contains("x * x"), "got: {}", s);
    }
    #[test]
    pub(super) fn test_recursive_function() {
        let func = FSharpFunction {
            name: "factorial".to_string(),
            is_recursive: true,
            is_inline: false,
            type_params: vec![],
            params: vec![("n".to_string(), Some(FSharpType::Int))],
            return_type: Some(FSharpType::Int),
            body: FSharpExpr::Match(
                Box::new(FSharpExpr::Var("n".to_string())),
                vec![
                    (
                        FSharpPattern::Lit("0".to_string()),
                        FSharpExpr::Lit("1".to_string()),
                    ),
                    (
                        FSharpPattern::Var("k".to_string()),
                        FSharpExpr::BinOp(
                            "*".to_string(),
                            Box::new(FSharpExpr::Var("k".to_string())),
                            Box::new(FSharpExpr::App(
                                Box::new(FSharpExpr::Var("factorial".to_string())),
                                Box::new(FSharpExpr::BinOp(
                                    "-".to_string(),
                                    Box::new(FSharpExpr::Var("k".to_string())),
                                    Box::new(FSharpExpr::Lit("1".to_string())),
                                )),
                            )),
                        ),
                    ),
                ],
            ),
            doc: None,
        };
        let s = b().emit_function(&func);
        assert!(s.contains("let rec factorial"), "got: {}", s);
        assert!(s.contains("match n with"), "got: {}", s);
    }
    #[test]
    pub(super) fn test_expr_ann() {
        let e = FSharpExpr::Ann(Box::new(FSharpExpr::Lit("42".to_string())), FSharpType::Int);
        assert_eq!(b().emit_expr(&e, 0), "(42 : int)");
    }
    #[test]
    pub(super) fn test_type_var() {
        assert_eq!(b().emit_type(&FSharpType::TypeVar("a".to_string())), "'a");
    }
}
/// Build a computation expression: `async { body }`.
#[allow(dead_code)]
pub fn computation_expr(kind: ComputationExprKind, stmts: Vec<FSharpExpr>) -> FSharpExpr {
    let _kind_str = kind.to_string();
    FSharpExpr::Raw(format!(
        "{} {{\n{}\n}}",
        _kind_str,
        stmts
            .iter()
            .map(|s| format!("    {}", {
                let b = FSharpBackend::new();
                b.emit_expr(s, 1)
            }))
            .collect::<Vec<_>>()
            .join("\n")
    ))
}
/// Build a float with a unit of measure annotation: `42.0<kg>`.
#[allow(dead_code)]
pub fn float_with_measure(value: f64, measure: &str) -> FSharpExpr {
    FSharpExpr::Lit(format!("{}<{}>", value, measure))
}
/// Build a simple variable reference.
#[allow(dead_code)]
pub fn fvar(name: impl Into<String>) -> FSharpExpr {
    FSharpExpr::Var(name.into())
}
/// Build a literal expression.
#[allow(dead_code)]
pub fn flit(s: impl Into<String>) -> FSharpExpr {
    FSharpExpr::Lit(s.into())
}
/// Build a function application: `f x`.
#[allow(dead_code)]
pub fn fapp(f: FSharpExpr, x: FSharpExpr) -> FSharpExpr {
    FSharpExpr::App(Box::new(f), Box::new(x))
}
/// Build a multi-argument application: `f x1 x2 ...`.
#[allow(dead_code)]
pub fn fapp_multi(f: FSharpExpr, args: Vec<FSharpExpr>) -> FSharpExpr {
    args.into_iter().fold(f, fapp)
}
/// Build a lambda expression.
#[allow(dead_code)]
pub fn flam(param: impl Into<String>, body: FSharpExpr) -> FSharpExpr {
    FSharpExpr::Lambda(param.into(), Box::new(body))
}
/// Build a multi-parameter lambda.
#[allow(dead_code)]
pub fn flam_multi(params: Vec<impl Into<String>>, body: FSharpExpr) -> FSharpExpr {
    FSharpExpr::MultiLambda(
        params.into_iter().map(|p| p.into()).collect(),
        Box::new(body),
    )
}
/// Build a `let` binding.
#[allow(dead_code)]
pub fn flet(name: impl Into<String>, val: FSharpExpr, body: FSharpExpr) -> FSharpExpr {
    FSharpExpr::Let(name.into(), Box::new(val), Box::new(body))
}
/// Build a `let rec` binding.
#[allow(dead_code)]
pub fn fletrec(name: impl Into<String>, val: FSharpExpr, body: FSharpExpr) -> FSharpExpr {
    FSharpExpr::LetRec(name.into(), Box::new(val), Box::new(body))
}
/// Build an `if-then-else` expression.
#[allow(dead_code)]
pub fn fif(cond: FSharpExpr, then_e: FSharpExpr, else_e: FSharpExpr) -> FSharpExpr {
    FSharpExpr::If(Box::new(cond), Box::new(then_e), Box::new(else_e))
}
/// Build a binary operation.
#[allow(dead_code)]
pub fn fbinop(op: impl Into<String>, lhs: FSharpExpr, rhs: FSharpExpr) -> FSharpExpr {
    FSharpExpr::BinOp(op.into(), Box::new(lhs), Box::new(rhs))
}
/// Build a unary operation.
#[allow(dead_code)]
pub fn funary(op: impl Into<String>, operand: FSharpExpr) -> FSharpExpr {
    FSharpExpr::UnaryOp(op.into(), Box::new(operand))
}
/// Build a pipe expression: `lhs |> rhs`.
#[allow(dead_code)]
pub fn fpipe(lhs: FSharpExpr, rhs: FSharpExpr) -> FSharpExpr {
    FSharpExpr::Pipe(Box::new(lhs), Box::new(rhs))
}
/// Build a pipe chain: `e |> f1 |> f2 |> ...`.
#[allow(dead_code)]
pub fn fpipe_chain(init: FSharpExpr, funcs: Vec<FSharpExpr>) -> FSharpExpr {
    funcs.into_iter().fold(init, fpipe)
}
/// Build a constructor application.
#[allow(dead_code)]
pub fn fctor(name: impl Into<String>, args: Vec<FSharpExpr>) -> FSharpExpr {
    FSharpExpr::Ctor(name.into(), args)
}
/// Build a `Some x` expression.
#[allow(dead_code)]
pub fn fsome(x: FSharpExpr) -> FSharpExpr {
    fctor("Some", vec![x])
}
/// Build a `None` expression.
#[allow(dead_code)]
pub fn fnone() -> FSharpExpr {
    fvar("None")
}
/// Build an `Ok x` expression.
#[allow(dead_code)]
pub fn fok(x: FSharpExpr) -> FSharpExpr {
    fctor("Ok", vec![x])
}
/// Build an `Error e` expression.
#[allow(dead_code)]
pub fn ferror(e: FSharpExpr) -> FSharpExpr {
    fctor("Error", vec![e])
}
/// Build a sequence operator: `e1; e2`.
#[allow(dead_code)]
pub fn fseq(e1: FSharpExpr, e2: FSharpExpr) -> FSharpExpr {
    FSharpExpr::Seq(Box::new(e1), Box::new(e2))
}
/// Build a field access: `expr.Field`.
#[allow(dead_code)]
pub fn ffield(base: FSharpExpr, field: impl Into<String>) -> FSharpExpr {
    FSharpExpr::FieldAccess(Box::new(base), field.into())
}
/// Build a type-annotated expression: `(expr : ty)`.
#[allow(dead_code)]
pub fn fann(expr: FSharpExpr, ty: FSharpType) -> FSharpExpr {
    FSharpExpr::Ann(Box::new(expr), ty)
}
/// Build an integer literal.
#[allow(dead_code)]
pub fn fint(n: i64) -> FSharpExpr {
    FSharpExpr::Lit(n.to_string())
}
/// Build a float literal.
#[allow(dead_code)]
pub fn ffloat(x: f64) -> FSharpExpr {
    FSharpExpr::Lit(format!("{}", x))
}
/// Build a boolean literal.
#[allow(dead_code)]
pub fn fbool(b: bool) -> FSharpExpr {
    FSharpExpr::Lit(if b { "true" } else { "false" }.to_string())
}
/// Build a string literal (quoted).
#[allow(dead_code)]
pub fn fstring(s: impl Into<String>) -> FSharpExpr {
    FSharpExpr::Lit(format!("\"{}\"", s.into()))
}
/// Build the unit value `()`.
#[allow(dead_code)]
pub fn funit() -> FSharpExpr {
    FSharpExpr::Lit("()".to_string())
}
/// Build a wildcard pattern `_`.
#[allow(dead_code)]
pub fn pwild() -> FSharpPattern {
    FSharpPattern::Wildcard
}
/// Build a variable pattern.
#[allow(dead_code)]
pub fn pvar(name: impl Into<String>) -> FSharpPattern {
    FSharpPattern::Var(name.into())
}
/// Build a literal pattern.
#[allow(dead_code)]
pub fn plit(s: impl Into<String>) -> FSharpPattern {
    FSharpPattern::Lit(s.into())
}
/// Build a `Some v` pattern.
#[allow(dead_code)]
pub fn psome(inner: FSharpPattern) -> FSharpPattern {
    FSharpPattern::Ctor("Some".to_string(), vec![inner])
}
/// Build a `None` pattern.
#[allow(dead_code)]
pub fn pnone() -> FSharpPattern {
    FSharpPattern::Ctor("None".to_string(), vec![])
}
/// Build a `Ok v` pattern.
#[allow(dead_code)]
pub fn pok(inner: FSharpPattern) -> FSharpPattern {
    FSharpPattern::Ctor("Ok".to_string(), vec![inner])
}
/// Build an `Error e` pattern.
#[allow(dead_code)]
pub fn perror(inner: FSharpPattern) -> FSharpPattern {
    FSharpPattern::Ctor("Error".to_string(), vec![inner])
}
/// Build a tuple pattern `(a, b)`.
#[allow(dead_code)]
pub fn ptuple(pats: Vec<FSharpPattern>) -> FSharpPattern {
    FSharpPattern::Tuple(pats)
}
/// Build a cons pattern `h :: t`.
#[allow(dead_code)]
pub fn pcons(head: FSharpPattern, tail: FSharpPattern) -> FSharpPattern {
    FSharpPattern::Cons(Box::new(head), Box::new(tail))
}
/// Build an `as` pattern `p as name`.
#[allow(dead_code)]
pub fn pas(pat: FSharpPattern, name: impl Into<String>) -> FSharpPattern {
    FSharpPattern::As(Box::new(pat), name.into())
}
/// Build a guarded pattern `p when expr`.
#[allow(dead_code)]
pub fn pwhen(pat: FSharpPattern, guard: FSharpExpr) -> FSharpPattern {
    FSharpPattern::When(Box::new(pat), Box::new(guard))
}
/// Build `int list`.
#[allow(dead_code)]
pub fn fs_int_list() -> FSharpType {
    FSharpType::List(Box::new(FSharpType::Int))
}
/// Build `string list`.
#[allow(dead_code)]
pub fn fs_string_list() -> FSharpType {
    FSharpType::List(Box::new(FSharpType::FsString))
}
/// Build `int option`.
#[allow(dead_code)]
pub fn fs_int_option() -> FSharpType {
    FSharpType::Option(Box::new(FSharpType::Int))
}
/// Build `T array`.
#[allow(dead_code)]
pub fn fs_array(ty: FSharpType) -> FSharpType {
    FSharpType::Array(Box::new(ty))
}
/// Build a generic `Map<K, V>` type.
#[allow(dead_code)]
pub fn fs_map(key: FSharpType, value: FSharpType) -> FSharpType {
    FSharpType::Generic("Map".to_string(), vec![key, value])
}
/// Build a generic `Set<T>` type.
#[allow(dead_code)]
pub fn fs_set(elem: FSharpType) -> FSharpType {
    FSharpType::Generic("Set".to_string(), vec![elem])
}
/// Build `Result<T, E>`.
#[allow(dead_code)]
pub fn fs_result(ok: FSharpType, err: FSharpType) -> FSharpType {
    FSharpType::Result(Box::new(ok), Box::new(err))
}
/// Build `T -> U`.
#[allow(dead_code)]
pub fn fs_fun(from: FSharpType, to: FSharpType) -> FSharpType {
    FSharpType::Fun(Box::new(from), Box::new(to))
}
/// Build `T1 * T2 * T3` tuple type.
#[allow(dead_code)]
pub fn fs_tuple(tys: Vec<FSharpType>) -> FSharpType {
    FSharpType::Tuple(tys)
}
/// Collect simple metrics from an `FSharpModule`.
#[allow(dead_code)]
pub fn collect_module_metrics(module: &FSharpModule) -> FSharpModuleMetrics {
    let mut metrics = FSharpModuleMetrics::default();
    metrics.type_count = module.records.len() + module.unions.len();
    metrics.function_count = module.functions.len();
    for f in &module.functions {
        if f.is_recursive {
            metrics.recursive_count += 1;
        }
        if f.is_inline {
            metrics.inline_count += 1;
        }
    }
    metrics.estimated_lines = 5
        + module.opens.len()
        + module.records.len() * 4
        + module.unions.len() * 5
        + module.functions.len() * 6;
    metrics
}
/// Build an `async { return! x }` expression.
#[allow(dead_code)]
pub fn async_return(x: FSharpExpr) -> FSharpExpr {
    FSharpExpr::Raw(format!(
        "async {{ return! {} }}",
        FSharpBackend::new().emit_expr(&x, 0)
    ))
}
/// Build an `async { let! v = comp in body }` expression.
#[allow(dead_code)]
pub fn async_let_bang(name: impl Into<String>, comp: FSharpExpr, body: FSharpExpr) -> FSharpExpr {
    let b = FSharpBackend::new();
    FSharpExpr::Raw(format!(
        "async {{\n    let! {} = {}\n    return! {}\n}}",
        name.into(),
        b.emit_expr(&comp, 1),
        b.emit_expr(&body, 1)
    ))
}
/// Build an `Async.map f computation` expression.
#[allow(dead_code)]
pub fn async_map(f: FSharpExpr, comp: FSharpExpr) -> FSharpExpr {
    fapp_multi(fvar("Async.map"), vec![f, comp])
}
/// Build an `Async.bind f computation` expression.
#[allow(dead_code)]
pub fn async_bind(f: FSharpExpr, comp: FSharpExpr) -> FSharpExpr {
    fapp_multi(fvar("Async.bind"), vec![f, comp])
}
#[cfg(test)]
mod extended_fsharp_tests {
    use super::*;
    #[test]
    pub(super) fn test_attribute_simple() {
        let a = FSharpAttribute::simple("Serializable");
        assert_eq!(a.emit(), "[<Serializable>]");
    }
    #[test]
    pub(super) fn test_attribute_with_args() {
        let a = FSharpAttribute::with_args("DllImport", vec!["\"mylib.dll\"".to_string()]);
        assert!(a.emit().contains("DllImport"));
        assert!(a.emit().contains("mylib.dll"));
    }
    #[test]
    pub(super) fn test_type_alias_emit() {
        let alias = FSharpTypeAlias {
            name: "IntList".to_string(),
            type_params: vec![],
            aliased_type: FSharpType::List(Box::new(FSharpType::Int)),
            doc: None,
        };
        let s = alias.emit();
        assert!(s.contains("type IntList = "));
        assert!(s.contains("int list"));
    }
    #[test]
    pub(super) fn test_type_alias_generic() {
        let alias = FSharpTypeAlias {
            name: "Pair".to_string(),
            type_params: vec!["'a".to_string(), "'b".to_string()],
            aliased_type: FSharpType::Tuple(vec![
                FSharpType::TypeVar("a".to_string()),
                FSharpType::TypeVar("b".to_string()),
            ]),
            doc: None,
        };
        let s = alias.emit();
        assert!(s.contains("type Pair<"));
        assert!(s.contains("'a * 'b"));
    }
    #[test]
    pub(super) fn test_fvar_flit() {
        let b = FSharpBackend::new();
        assert_eq!(b.emit_expr(&fvar("x"), 0), "x");
        assert_eq!(b.emit_expr(&flit("42"), 0), "42");
    }
    #[test]
    pub(super) fn test_fapp_multi() {
        let b = FSharpBackend::new();
        let e = fapp_multi(fvar("f"), vec![fvar("x"), fvar("y")]);
        let s = b.emit_expr(&e, 0);
        assert!(s.contains("f"));
        assert!(s.contains("x"));
        assert!(s.contains("y"));
    }
    #[test]
    pub(super) fn test_fpipe_chain() {
        let b = FSharpBackend::new();
        let e = fpipe_chain(fvar("xs"), vec![fvar("List.sort"), fvar("List.rev")]);
        let s = b.emit_expr(&e, 0);
        assert!(s.contains("|>"));
        assert!(s.contains("xs"));
    }
    #[test]
    pub(super) fn test_fsome_fnone() {
        let b = FSharpBackend::new();
        let s = b.emit_expr(&fsome(fvar("x")), 0);
        assert!(s.contains("Some"));
        let n = b.emit_expr(&fnone(), 0);
        assert_eq!(n, "None");
    }
    #[test]
    pub(super) fn test_fok_ferror() {
        let b = FSharpBackend::new();
        let ok = b.emit_expr(&fok(fvar("v")), 0);
        assert!(ok.contains("Ok"));
        let err = b.emit_expr(&ferror(fvar("e")), 0);
        assert!(err.contains("Error"));
    }
    #[test]
    pub(super) fn test_fint_ffloat() {
        let b = FSharpBackend::new();
        assert_eq!(b.emit_expr(&fint(42), 0), "42");
        assert!(b.emit_expr(&ffloat(3.14), 0).contains("3.14"));
    }
    #[test]
    pub(super) fn test_fbool_fstring_funit() {
        let b = FSharpBackend::new();
        assert_eq!(b.emit_expr(&fbool(true), 0), "true");
        assert_eq!(b.emit_expr(&fbool(false), 0), "false");
        assert!(b.emit_expr(&fstring("hello"), 0).contains("hello"));
        assert_eq!(b.emit_expr(&funit(), 0), "()");
    }
    #[test]
    pub(super) fn test_pattern_psome_pnone() {
        let b = FSharpBackend::new();
        let p = psome(pvar("v"));
        assert!(b.emit_pattern(&p).contains("Some"));
        let n = pnone();
        assert!(b.emit_pattern(&n).contains("None"));
    }
    #[test]
    pub(super) fn test_pattern_pok_perror() {
        let b = FSharpBackend::new();
        let p = pok(pvar("x"));
        assert!(b.emit_pattern(&p).contains("Ok"));
        let e = perror(pvar("err"));
        assert!(b.emit_pattern(&e).contains("Error"));
    }
    #[test]
    pub(super) fn test_pattern_ptuple() {
        let b = FSharpBackend::new();
        let p = ptuple(vec![pvar("a"), pvar("b")]);
        let s = b.emit_pattern(&p);
        assert!(s.contains("a"));
        assert!(s.contains("b"));
        assert!(s.contains("("));
    }
    #[test]
    pub(super) fn test_pattern_pcons() {
        let b = FSharpBackend::new();
        let p = pcons(pvar("h"), pvar("t"));
        let s = b.emit_pattern(&p);
        assert!(s.contains("h :: t"));
    }
    #[test]
    pub(super) fn test_fs_type_constructors() {
        assert!(format!("{}", fs_int_list()).contains("int list"));
        assert!(format!("{}", fs_string_list()).contains("string list"));
        assert!(format!("{}", fs_int_option()).contains("int option"));
        assert!(format!("{}", fs_array(FSharpType::Float)).contains("array"));
        assert!(format!("{}", fs_map(FSharpType::FsString, FSharpType::Int)).contains("Map"));
        assert!(format!("{}", fs_set(FSharpType::Int)).contains("Set"));
        assert!(format!("{}", fs_result(FSharpType::Int, FSharpType::FsString)).contains("Result"));
    }
    #[test]
    pub(super) fn test_fs_fun_type() {
        let ty = fs_fun(FSharpType::Int, FSharpType::Bool);
        assert!(format!("{}", ty).contains("->"));
    }
    #[test]
    pub(super) fn test_fs_tuple_type() {
        let ty = fs_tuple(vec![FSharpType::Int, FSharpType::Float, FSharpType::Bool]);
        let s = format!("{}", ty);
        assert!(s.contains("int"));
        assert!(s.contains("float"));
        assert!(s.contains("bool"));
        assert!(s.contains("*"));
    }
    #[test]
    pub(super) fn test_snippets() {
        assert!(FSharpSnippets::option_map().contains("Some"));
        assert!(FSharpSnippets::option_bind().contains("None"));
        assert!(FSharpSnippets::result_map().contains("Ok"));
        assert!(FSharpSnippets::list_fold().contains("foldLeft"));
        assert!(FSharpSnippets::memoize().contains("Dictionary"));
        assert!(FSharpSnippets::fix_point().contains("fix"));
    }
    #[test]
    pub(super) fn test_numeric_helpers() {
        let b = FSharpBackend::new();
        let clamp = FSharpNumericHelpers::clamp();
        let s = b.emit_function(&clamp);
        assert!(s.contains("clamp"));
        assert!(s.contains("inline"));
        let sq = FSharpNumericHelpers::square();
        let s2 = b.emit_function(&sq);
        assert!(s2.contains("square"));
        let pow = FSharpNumericHelpers::pow_int();
        let s3 = b.emit_function(&pow);
        assert!(s3.contains("powInt"));
        assert!(s3.contains("rec"));
        let gcd = FSharpNumericHelpers::gcd();
        let s4 = b.emit_function(&gcd);
        assert!(s4.contains("gcd"));
    }
    #[test]
    pub(super) fn test_module_builder() {
        let s = FSharpModuleBuilder::new("MyLib")
            .open("System")
            .function(FSharpNumericHelpers::square())
            .emit();
        assert!(s.contains("module MyLib"));
        assert!(s.contains("open System"));
        assert!(s.contains("square"));
    }
    #[test]
    pub(super) fn test_function_builder() {
        let f = FSharpFunctionBuilder::new("double")
            .param("x", Some(FSharpType::Int))
            .returns(FSharpType::Int)
            .body(fbinop("*", flit("2"), fvar("x")))
            .doc("Double an integer.")
            .build();
        let b = FSharpBackend::new();
        let s = b.emit_function(&f);
        assert!(s.contains("double"));
        assert!(s.contains("2 * x"));
    }
    #[test]
    pub(super) fn test_collect_module_metrics() {
        let module = FSharpModule {
            name: "Test".to_string(),
            auto_open: false,
            records: vec![FSharpRecord {
                name: "Point".to_string(),
                type_params: vec![],
                fields: vec![("x".to_string(), FSharpType::Float)],
                doc: None,
            }],
            unions: vec![],
            functions: vec![FSharpNumericHelpers::square(), FSharpNumericHelpers::gcd()],
            opens: vec!["System".to_string()],
        };
        let m = collect_module_metrics(&module);
        assert_eq!(m.type_count, 1);
        assert_eq!(m.function_count, 2);
        assert_eq!(m.recursive_count, 1);
        assert_eq!(m.inline_count, 1);
        assert!(m.estimated_lines > 0);
    }
    #[test]
    pub(super) fn test_stdlib_id() {
        let b = FSharpBackend::new();
        let f = FSharpStdLib::id_function();
        let s = b.emit_function(&f);
        assert!(s.contains("id"));
    }
    #[test]
    pub(super) fn test_stdlib_flip() {
        let b = FSharpBackend::new();
        let f = FSharpStdLib::flip_function();
        let s = b.emit_function(&f);
        assert!(s.contains("flip"));
    }
    #[test]
    pub(super) fn test_stdlib_const() {
        let b = FSharpBackend::new();
        let f = FSharpStdLib::const_function();
        let s = b.emit_function(&f);
        assert!(s.contains("konst"));
    }
    #[test]
    pub(super) fn test_stdlib_foldl() {
        let b = FSharpBackend::new();
        let f = FSharpStdLib::foldl_function();
        let s = b.emit_function(&f);
        assert!(s.contains("foldl"));
        assert!(s.contains("rec"));
    }
    #[test]
    pub(super) fn test_stdlib_filter() {
        let b = FSharpBackend::new();
        let f = FSharpStdLib::filter_function();
        let s = b.emit_function(&f);
        assert!(s.contains("filterList"));
    }
    #[test]
    pub(super) fn test_stdlib_zip_with() {
        let b = FSharpBackend::new();
        let f = FSharpStdLib::zip_with_function();
        let s = b.emit_function(&f);
        assert!(s.contains("zipWith"));
    }
    #[test]
    pub(super) fn test_union_case_named() {
        let c = FSharpUnionCaseNamed {
            name: "Circle".to_string(),
            named_fields: vec![("radius".to_string(), FSharpType::Float)],
        };
        let s = c.emit();
        assert!(s.contains("Circle"));
        assert!(s.contains("radius: float"));
    }
    #[test]
    pub(super) fn test_measure_decl() {
        let m = FSharpMeasure {
            name: "kg".to_string(),
            abbrev: None,
        };
        let s = m.emit();
        assert!(s.contains("[<Measure>]"));
        assert!(s.contains("type kg"));
    }
    #[test]
    pub(super) fn test_float_with_measure() {
        let b = FSharpBackend::new();
        let e = float_with_measure(42.0, "kg");
        let s = b.emit_expr(&e, 0);
        assert!(s.contains("kg"));
    }
    #[test]
    pub(super) fn test_computation_expr() {
        let e = computation_expr(ComputationExprKind::Async, vec![fvar("doSomething")]);
        let b = FSharpBackend::new();
        let s = b.emit_expr(&e, 0);
        assert!(s.contains("async"));
        assert!(s.contains("doSomething"));
    }
    #[test]
    pub(super) fn test_mutual_group_emit() {
        let f1 = FSharpFunctionBuilder::new("isEven")
            .recursive()
            .param("n", Some(FSharpType::Int))
            .returns(FSharpType::Bool)
            .body(fif(
                fbinop("=", fvar("n"), fint(0)),
                fbool(true),
                fapp(fvar("isOdd"), fbinop("-", fvar("n"), fint(1))),
            ))
            .build();
        let f2 = FSharpFunctionBuilder::new("isOdd")
            .recursive()
            .param("n", Some(FSharpType::Int))
            .returns(FSharpType::Bool)
            .body(fif(
                fbinop("=", fvar("n"), fint(0)),
                fbool(false),
                fapp(fvar("isEven"), fbinop("-", fvar("n"), fint(1))),
            ))
            .build();
        let group = FSharpMutualGroup {
            functions: vec![f1, f2],
        };
        let b = FSharpBackend::new();
        let s = group.emit(&b);
        assert!(s.contains("isEven"));
        assert!(s.contains("isOdd"));
    }
    #[test]
    pub(super) fn test_interface_emit() {
        let iface = FSharpInterface {
            name: "IComparable".to_string(),
            type_params: vec!["'T".to_string()],
            methods: vec![(
                "CompareTo".to_string(),
                vec![("other".to_string(), FSharpType::TypeVar("T".to_string()))],
                FSharpType::Int,
            )],
            properties: vec![],
            doc: None,
        };
        let s = iface.emit();
        assert!(s.contains("IComparable"));
        assert!(s.contains("abstract member CompareTo"));
    }
    #[test]
    pub(super) fn test_active_pattern_total() {
        let ap = FSharpActivePattern {
            kind: ActivePatternKind::Total(vec!["Even".to_string(), "Odd".to_string()]),
            params: vec!["n".to_string()],
            body: fif(
                fbinop("=", fbinop("%", fvar("n"), fint(2)), fint(0)),
                fvar("Even"),
                fvar("Odd"),
            ),
        };
        let b = FSharpBackend::new();
        let s = ap.emit(&b);
        assert!(s.contains("Even|Odd"));
        assert!(s.contains("let"));
    }
    #[test]
    pub(super) fn test_active_pattern_partial() {
        let ap = FSharpActivePattern {
            kind: ActivePatternKind::Partial("IsPositive".to_string()),
            params: vec!["x".to_string()],
            body: fif(fbinop(">", fvar("x"), fint(0)), fsome(fvar("x")), fnone()),
        };
        let b = FSharpBackend::new();
        let s = ap.emit(&b);
        assert!(s.contains("IsPositive"));
        assert!(s.contains("|_|"));
    }
    #[test]
    pub(super) fn test_async_return() {
        let b = FSharpBackend::new();
        let e = async_return(fvar("result"));
        let s = b.emit_expr(&e, 0);
        assert!(s.contains("async"));
        assert!(s.contains("result"));
    }
    #[test]
    pub(super) fn test_async_map_bind() {
        let b = FSharpBackend::new();
        let e = async_map(fvar("f"), fvar("comp"));
        let s = b.emit_expr(&e, 0);
        assert!(s.contains("Async.map"));
        let e2 = async_bind(fvar("f"), fvar("comp"));
        let s2 = b.emit_expr(&e2, 0);
        assert!(s2.contains("Async.bind"));
    }
    #[test]
    pub(super) fn test_auto_open_module() {
        let s = FSharpModuleBuilder::new("Operators").auto_open().emit();
        assert!(s.contains("[<AutoOpen>]"));
        assert!(s.contains("module Operators"));
    }
}
