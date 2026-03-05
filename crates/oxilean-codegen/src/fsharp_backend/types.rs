//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;

/// An F# expression node.
#[derive(Debug, Clone)]
pub enum FSharpExpr {
    /// Literal source (integer, float, string, bool, unit `()`)
    Lit(String),
    /// Variable / identifier reference
    Var(String),
    /// Function application: `f x`
    App(Box<FSharpExpr>, Box<FSharpExpr>),
    /// Lambda: `fun param -> body`
    Lambda(String, Box<FSharpExpr>),
    /// Multi-parameter lambda: `fun a b c -> body`
    MultiLambda(Vec<String>, Box<FSharpExpr>),
    /// Let binding: `let name = value in body`
    Let(String, Box<FSharpExpr>, Box<FSharpExpr>),
    /// Recursive let binding: `let rec name = value in body`
    LetRec(String, Box<FSharpExpr>, Box<FSharpExpr>),
    /// Pattern match: `match expr with | pat -> branch`
    Match(Box<FSharpExpr>, Vec<(FSharpPattern, FSharpExpr)>),
    /// If-then-else
    If(Box<FSharpExpr>, Box<FSharpExpr>, Box<FSharpExpr>),
    /// Tuple construction: `(a, b, c)`
    Tuple(Vec<FSharpExpr>),
    /// List literal: `[a; b; c]`
    FsList(Vec<FSharpExpr>),
    /// Array literal: `[| a; b; c |]`
    FsArray(Vec<FSharpExpr>),
    /// Binary operator: `lhs op rhs`
    BinOp(String, Box<FSharpExpr>, Box<FSharpExpr>),
    /// Unary operator: `op expr`
    UnaryOp(String, Box<FSharpExpr>),
    /// Record construction: `{ field1 = v1; field2 = v2 }`
    Record(Vec<(String, FSharpExpr)>),
    /// Record update: `{ base with field = v }`
    RecordUpdate(Box<FSharpExpr>, Vec<(String, FSharpExpr)>),
    /// Field access: `expr.Field`
    FieldAccess(Box<FSharpExpr>, String),
    /// Sequence: `expr1; expr2` (semicolon-separated, result is `expr2`)
    Seq(Box<FSharpExpr>, Box<FSharpExpr>),
    /// Type annotation: `(expr : T)`
    Ann(Box<FSharpExpr>, FSharpType),
    /// Pipe forward: `expr |> f`
    Pipe(Box<FSharpExpr>, Box<FSharpExpr>),
    /// Constructor application: `Some x`, `MyCase(a, b)`
    Ctor(String, Vec<FSharpExpr>),
    /// `do { ... }` computation expression (simplified)
    Do(Vec<FSharpExpr>),
    /// Raw source snippet (escape hatch)
    Raw(String),
}
/// A discriminated union declaration: `type T = | A | B of int`.
#[derive(Debug, Clone)]
pub struct FSharpUnion {
    /// Type name
    pub name: String,
    /// Generic type parameters
    pub type_params: Vec<String>,
    /// Constructor cases
    pub cases: Vec<FSharpUnionCase>,
    /// Optional XML doc comment
    pub doc: Option<String>,
}
/// Fluent builder for `FSharpFunction`.
#[allow(dead_code)]
pub struct FSharpFunctionBuilder {
    pub(super) func: FSharpFunction,
}
impl FSharpFunctionBuilder {
    /// Start a function with the given name.
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>) -> Self {
        FSharpFunctionBuilder {
            func: FSharpFunction {
                name: name.into(),
                is_recursive: false,
                is_inline: false,
                type_params: vec![],
                params: vec![],
                return_type: None,
                body: funit(),
                doc: None,
            },
        }
    }
    /// Mark as recursive.
    #[allow(dead_code)]
    pub fn recursive(mut self) -> Self {
        self.func.is_recursive = true;
        self
    }
    /// Mark as inline.
    #[allow(dead_code)]
    pub fn inline(mut self) -> Self {
        self.func.is_inline = true;
        self
    }
    /// Add a type parameter.
    #[allow(dead_code)]
    pub fn type_param(mut self, tp: impl Into<String>) -> Self {
        self.func.type_params.push(tp.into());
        self
    }
    /// Add a parameter (with optional type annotation).
    #[allow(dead_code)]
    pub fn param(mut self, name: impl Into<String>, ty: Option<FSharpType>) -> Self {
        self.func.params.push((name.into(), ty));
        self
    }
    /// Set return type.
    #[allow(dead_code)]
    pub fn returns(mut self, ty: FSharpType) -> Self {
        self.func.return_type = Some(ty);
        self
    }
    /// Set the body.
    #[allow(dead_code)]
    pub fn body(mut self, expr: FSharpExpr) -> Self {
        self.func.body = expr;
        self
    }
    /// Add a doc comment.
    #[allow(dead_code)]
    pub fn doc(mut self, doc: impl Into<String>) -> Self {
        self.func.doc = Some(doc.into());
        self
    }
    /// Finalise.
    #[allow(dead_code)]
    pub fn build(self) -> FSharpFunction {
        self.func
    }
    /// Emit the function directly.
    #[allow(dead_code)]
    pub fn emit(self) -> String {
        FSharpBackend::new().emit_function(&self.func)
    }
}
/// A group of mutually recursive `let rec ... and ...` functions.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct FSharpMutualGroup {
    /// Functions in this mutual group (all sharing `let rec ... and ...`).
    pub functions: Vec<FSharpFunction>,
}
impl FSharpMutualGroup {
    /// Emit the mutual group.
    #[allow(dead_code)]
    pub fn emit(&self, backend: &FSharpBackend) -> String {
        if self.functions.is_empty() {
            return String::new();
        }
        let mut parts: Vec<String> = Vec::new();
        for (i, func) in self.functions.iter().enumerate() {
            let s = backend.emit_function(func);
            if i == 0 {
                parts.push(s.trim_end().to_string());
            } else {
                let replaced = if s.starts_with("let rec ") {
                    s.replacen("let rec ", "and rec ", 1)
                } else if s.starts_with("let ") {
                    s.replacen("let ", "and ", 1)
                } else {
                    s
                };
                parts.push(replaced.trim_end().to_string());
            }
        }
        parts.join("\n") + "\n"
    }
}
/// Common standard library–style helper definitions.
#[allow(dead_code)]
pub struct FSharpStdLib;
impl FSharpStdLib {
    /// Build the `id` function.
    #[allow(dead_code)]
    pub fn id_function() -> FSharpFunction {
        FSharpFunctionBuilder::new("id")
            .type_param("'a")
            .param("x", Some(FSharpType::TypeVar("a".to_string())))
            .returns(FSharpType::TypeVar("a".to_string()))
            .body(fvar("x"))
            .doc("Identity function.")
            .build()
    }
    /// Build the `flip` function.
    #[allow(dead_code)]
    pub fn flip_function() -> FSharpFunction {
        FSharpFunctionBuilder::new("flip")
            .type_param("'a")
            .type_param("'b")
            .type_param("'c")
            .param("f", None)
            .param("x", None)
            .param("y", None)
            .body(fapp(fapp(fvar("f"), fvar("y")), fvar("x")))
            .doc("Flip a two-argument function.")
            .build()
    }
    /// Build the `const` function.
    #[allow(dead_code)]
    pub fn const_function() -> FSharpFunction {
        FSharpFunctionBuilder::new("konst")
            .type_param("'a")
            .type_param("'b")
            .param("x", None)
            .param("_y", None)
            .body(fvar("x"))
            .doc("Constant function (K combinator).")
            .build()
    }
    /// Build a `foldl` function.
    #[allow(dead_code)]
    pub fn foldl_function() -> FSharpFunction {
        FSharpFunctionBuilder::new("foldl")
            .recursive()
            .type_param("'a")
            .type_param("'b")
            .param("f", None)
            .param("acc", None)
            .param("lst", None)
            .body(FSharpExpr::Match(
                Box::new(fvar("lst")),
                vec![
                    (FSharpPattern::FsList(vec![]), fvar("acc")),
                    (
                        pcons(pvar("h"), pvar("t")),
                        fapp_multi(
                            fvar("foldl"),
                            vec![
                                fvar("f"),
                                fapp(fapp(fvar("f"), fvar("acc")), fvar("h")),
                                fvar("t"),
                            ],
                        ),
                    ),
                ],
            ))
            .doc("Left fold over a list.")
            .build()
    }
    /// Build a `filter` function.
    #[allow(dead_code)]
    pub fn filter_function() -> FSharpFunction {
        FSharpFunctionBuilder::new("filterList")
            .recursive()
            .type_param("'a")
            .param("pred", None)
            .param("lst", None)
            .body(FSharpExpr::Match(
                Box::new(fvar("lst")),
                vec![
                    (FSharpPattern::FsList(vec![]), FSharpExpr::FsList(vec![])),
                    (
                        pcons(pvar("h"), pvar("t")),
                        fif(
                            fapp(fvar("pred"), fvar("h")),
                            FSharpExpr::BinOp(
                                "::".to_string(),
                                Box::new(fvar("h")),
                                Box::new(fapp(fapp(fvar("filterList"), fvar("pred")), fvar("t"))),
                            ),
                            fapp(fapp(fvar("filterList"), fvar("pred")), fvar("t")),
                        ),
                    ),
                ],
            ))
            .doc("Filter a list by a predicate.")
            .build()
    }
    /// Build a `zipWith` function.
    #[allow(dead_code)]
    pub fn zip_with_function() -> FSharpFunction {
        FSharpFunctionBuilder::new("zipWith")
            .recursive()
            .type_param("'a")
            .type_param("'b")
            .type_param("'c")
            .param("f", None)
            .param("xs", None)
            .param("ys", None)
            .body(FSharpExpr::Match(
                Box::new(FSharpExpr::Tuple(vec![fvar("xs"), fvar("ys")])),
                vec![
                    (
                        ptuple(vec![FSharpPattern::FsList(vec![]), FSharpPattern::Wildcard]),
                        FSharpExpr::FsList(vec![]),
                    ),
                    (
                        ptuple(vec![FSharpPattern::Wildcard, FSharpPattern::FsList(vec![])]),
                        FSharpExpr::FsList(vec![]),
                    ),
                    (
                        ptuple(vec![
                            pcons(pvar("x"), pvar("xt")),
                            pcons(pvar("y"), pvar("yt")),
                        ]),
                        FSharpExpr::BinOp(
                            "::".to_string(),
                            Box::new(fapp(fapp(fvar("f"), fvar("x")), fvar("y"))),
                            Box::new(fapp_multi(
                                fvar("zipWith"),
                                vec![fvar("f"), fvar("xt"), fvar("yt")],
                            )),
                        ),
                    ),
                ],
            ))
            .doc("Zip two lists with a combining function.")
            .build()
    }
}
/// An F# type expression used during code generation.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FSharpType {
    /// `int` — default integer (32-bit on .NET)
    Int,
    /// `int64` — 64-bit signed integer
    Int64,
    /// `float` — 64-bit IEEE floating-point (`double` in .NET)
    Float,
    /// `float32` — 32-bit IEEE floating-point
    Float32,
    /// `bool` — boolean
    Bool,
    /// `string` — .NET `System.String`
    FsString,
    /// `char` — .NET `System.Char` (UTF-16 code unit)
    Char,
    /// `unit` — `()`
    Unit,
    /// `byte` — unsigned 8-bit integer
    Byte,
    /// `T list` — immutable singly-linked list
    List(Box<FSharpType>),
    /// `T array` — mutable .NET array
    Array(Box<FSharpType>),
    /// `T option` — optional value
    Option(Box<FSharpType>),
    /// `Result<T, E>` — result type
    Result(Box<FSharpType>, Box<FSharpType>),
    /// `T * U * …` — tuple type
    Tuple(Vec<FSharpType>),
    /// `T -> U` — function type
    Fun(Box<FSharpType>, Box<FSharpType>),
    /// Named type (discriminated union, record, alias)
    Custom(String),
    /// Generic type application: `Map<K,V>`, `IEnumerable<T>`, …
    Generic(String, Vec<FSharpType>),
    /// Polymorphic type variable: `'a`, `'b`
    TypeVar(String),
}
/// A top-level F# function or value definition.
#[derive(Debug, Clone)]
pub struct FSharpFunction {
    /// Function name (lowercase by convention)
    pub name: String,
    /// Whether this is a `let rec` definition
    pub is_recursive: bool,
    /// Whether this is a `let inline` definition
    pub is_inline: bool,
    /// Parameters: `(name, optional_type_annotation)`
    pub params: Vec<(String, Option<FSharpType>)>,
    /// Return type annotation (optional)
    pub return_type: Option<FSharpType>,
    /// Function body expression
    pub body: FSharpExpr,
    /// Optional XML doc comment
    pub doc: Option<String>,
    /// Generic type parameters (e.g. `["'a", "'b"]`)
    pub type_params: Vec<String>,
}
/// F# code generation backend.
pub struct FSharpBackend {
    /// Indentation string (default: four spaces)
    pub(super) indent_str: String,
}
impl FSharpBackend {
    /// Create a new `FSharpBackend` with default settings.
    pub fn new() -> Self {
        FSharpBackend {
            indent_str: "    ".to_string(),
        }
    }
    /// Create a backend with a custom indentation string.
    pub fn with_indent(indent: &str) -> Self {
        FSharpBackend {
            indent_str: indent.to_string(),
        }
    }
    /// Emit an [`FSharpType`] as F# source text.
    pub fn emit_type(&self, ty: &FSharpType) -> String {
        format!("{}", ty)
    }
    /// Emit an [`FSharpPattern`] as F# source text.
    pub fn emit_pattern(&self, pat: &FSharpPattern) -> String {
        match pat {
            FSharpPattern::Wildcard => "_".to_string(),
            FSharpPattern::Var(v) => v.clone(),
            FSharpPattern::Lit(l) => l.clone(),
            FSharpPattern::Tuple(pats) => {
                let inner = pats
                    .iter()
                    .map(|p| self.emit_pattern(p))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("({})", inner)
            }
            FSharpPattern::Ctor(name, pats) => {
                if pats.is_empty() {
                    name.clone()
                } else {
                    let inner = pats
                        .iter()
                        .map(|p| self.emit_pattern(p))
                        .collect::<Vec<_>>()
                        .join(", ");
                    format!("{}({})", name, inner)
                }
            }
            FSharpPattern::FsList(pats) => {
                let inner = pats
                    .iter()
                    .map(|p| self.emit_pattern(p))
                    .collect::<Vec<_>>()
                    .join("; ");
                format!("[{}]", inner)
            }
            FSharpPattern::Cons(head, tail) => {
                format!("{} :: {}", self.emit_pattern(head), self.emit_pattern(tail))
            }
            FSharpPattern::As(inner, name) => {
                format!("({} as {})", self.emit_pattern(inner), name)
            }
            FSharpPattern::When(inner, cond) => {
                format!(
                    "{} when {}",
                    self.emit_pattern(inner),
                    self.emit_expr(cond, 0)
                )
            }
        }
    }
    /// Emit an [`FSharpExpr`] at a given indentation depth.
    pub fn emit_expr(&self, expr: &FSharpExpr, depth: usize) -> String {
        let pad = self.indent_str.repeat(depth);
        let inner_pad = self.indent_str.repeat(depth + 1);
        match expr {
            FSharpExpr::Lit(s) => s.clone(),
            FSharpExpr::Var(v) => v.clone(),
            FSharpExpr::Raw(s) => s.clone(),
            FSharpExpr::App(f, x) => {
                format!(
                    "{} {}",
                    self.emit_expr_paren(f, depth),
                    self.emit_expr_paren(x, depth)
                )
            }
            FSharpExpr::Lambda(param, body) => {
                format!("fun {} -> {}", param, self.emit_expr(body, depth))
            }
            FSharpExpr::MultiLambda(params, body) => {
                format!(
                    "fun {} -> {}",
                    params.join(" "),
                    self.emit_expr(body, depth)
                )
            }
            FSharpExpr::Let(name, val, body) => {
                format!(
                    "let {} = {}\n{}{}",
                    name,
                    self.emit_expr(val, depth),
                    pad,
                    self.emit_expr(body, depth)
                )
            }
            FSharpExpr::LetRec(name, val, body) => {
                format!(
                    "let rec {} = {}\n{}{}",
                    name,
                    self.emit_expr(val, depth),
                    pad,
                    self.emit_expr(body, depth)
                )
            }
            FSharpExpr::Match(scrutinee, arms) => {
                let mut out = format!("match {} with\n", self.emit_expr(scrutinee, depth));
                for (pat, branch) in arms {
                    out += &format!(
                        "{}| {} -> {}\n",
                        inner_pad,
                        self.emit_pattern(pat),
                        self.emit_expr(branch, depth + 1)
                    );
                }
                out.trim_end().to_string()
            }
            FSharpExpr::If(cond, then_e, else_e) => {
                format!(
                    "if {} then\n{}{}\n{}else\n{}{}",
                    self.emit_expr(cond, depth),
                    inner_pad,
                    self.emit_expr(then_e, depth + 1),
                    pad,
                    inner_pad,
                    self.emit_expr(else_e, depth + 1)
                )
            }
            FSharpExpr::Tuple(elems) => {
                let inner = elems
                    .iter()
                    .map(|e| self.emit_expr(e, depth))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("({})", inner)
            }
            FSharpExpr::FsList(elems) => {
                if elems.is_empty() {
                    "[]".to_string()
                } else {
                    let inner = elems
                        .iter()
                        .map(|e| self.emit_expr(e, depth))
                        .collect::<Vec<_>>()
                        .join("; ");
                    format!("[{}]", inner)
                }
            }
            FSharpExpr::FsArray(elems) => {
                if elems.is_empty() {
                    "[||]".to_string()
                } else {
                    let inner = elems
                        .iter()
                        .map(|e| self.emit_expr(e, depth))
                        .collect::<Vec<_>>()
                        .join("; ");
                    format!("[|{}|]", inner)
                }
            }
            FSharpExpr::BinOp(op, lhs, rhs) => {
                format!(
                    "{} {} {}",
                    self.emit_expr_paren(lhs, depth),
                    op,
                    self.emit_expr_paren(rhs, depth)
                )
            }
            FSharpExpr::UnaryOp(op, operand) => {
                format!("{}{}", op, self.emit_expr_paren(operand, depth))
            }
            FSharpExpr::Record(fields) => {
                if fields.is_empty() {
                    "{}".to_string()
                } else {
                    let inner = fields
                        .iter()
                        .map(|(k, v)| format!("{} = {}", k, self.emit_expr(v, depth)))
                        .collect::<Vec<_>>()
                        .join("; ");
                    format!("{{ {} }}", inner)
                }
            }
            FSharpExpr::RecordUpdate(base, fields) => {
                let inner = fields
                    .iter()
                    .map(|(k, v)| format!("{} = {}", k, self.emit_expr(v, depth)))
                    .collect::<Vec<_>>()
                    .join("; ");
                format!("{{ {} with {} }}", self.emit_expr(base, depth), inner)
            }
            FSharpExpr::FieldAccess(base, field) => {
                format!("{}.{}", self.emit_expr_paren(base, depth), field)
            }
            FSharpExpr::Seq(e1, e2) => {
                format!(
                    "{}\n{}{}",
                    self.emit_expr(e1, depth),
                    pad,
                    self.emit_expr(e2, depth)
                )
            }
            FSharpExpr::Ann(e, ty) => {
                format!("({} : {})", self.emit_expr(e, depth), self.emit_type(ty))
            }
            FSharpExpr::Pipe(lhs, rhs) => {
                format!(
                    "{}\n{}|> {}",
                    self.emit_expr(lhs, depth),
                    pad,
                    self.emit_expr(rhs, depth)
                )
            }
            FSharpExpr::Ctor(name, args) => {
                if args.is_empty() {
                    name.clone()
                } else {
                    let arg_str = args
                        .iter()
                        .map(|a| self.emit_expr_paren(a, depth))
                        .collect::<Vec<_>>()
                        .join(" ");
                    format!("{} {}", name, arg_str)
                }
            }
            FSharpExpr::Do(stmts) => {
                let mut out = "do\n".to_string();
                for s in stmts {
                    out += &format!("{}{}\n", inner_pad, self.emit_expr(s, depth + 1));
                }
                out.trim_end().to_string()
            }
        }
    }
    /// Emit an expression, wrapping in parentheses if needed for disambiguation.
    pub(super) fn emit_expr_paren(&self, expr: &FSharpExpr, depth: usize) -> String {
        let needs_parens = matches!(
            expr,
            FSharpExpr::App(_, _)
                | FSharpExpr::Lambda(_, _)
                | FSharpExpr::MultiLambda(_, _)
                | FSharpExpr::Let(_, _, _)
                | FSharpExpr::LetRec(_, _, _)
                | FSharpExpr::Match(_, _)
                | FSharpExpr::If(_, _, _)
                | FSharpExpr::BinOp(_, _, _)
                | FSharpExpr::Seq(_, _)
                | FSharpExpr::Pipe(_, _)
        );
        let s = self.emit_expr(expr, depth);
        if needs_parens {
            format!("({})", s)
        } else {
            s
        }
    }
    /// Emit a record type declaration.
    pub fn emit_record(&self, rec: &FSharpRecord) -> String {
        let mut out = String::new();
        if let Some(doc) = &rec.doc {
            out += &format!("/// {}\n", doc);
        }
        let params = if rec.type_params.is_empty() {
            String::new()
        } else {
            format!("<{}>", rec.type_params.join(", "))
        };
        out += &format!("type {}{} =\n", rec.name, params);
        out += "    {";
        let field_strs: Vec<String> = rec
            .fields
            .iter()
            .map(|(name, ty)| format!(" {}: {}", name, self.emit_type(ty)))
            .collect();
        out += &field_strs.join(";");
        out += " }\n";
        out
    }
    /// Emit a discriminated union type declaration.
    pub fn emit_union(&self, union: &FSharpUnion) -> String {
        let mut out = String::new();
        if let Some(doc) = &union.doc {
            out += &format!("/// {}\n", doc);
        }
        let params = if union.type_params.is_empty() {
            String::new()
        } else {
            format!("<{}>", union.type_params.join(", "))
        };
        out += &format!("type {}{} =\n", union.name, params);
        for case in &union.cases {
            if case.fields.is_empty() {
                out += &format!("    | {}\n", case.name);
            } else {
                let field_str: Vec<String> =
                    case.fields.iter().map(|t| self.emit_type(t)).collect();
                out += &format!("    | {} of {}\n", case.name, field_str.join(" * "));
            }
        }
        out
    }
    /// Emit an [`FSharpFunction`] as F# source text.
    pub fn emit_function(&self, func: &FSharpFunction) -> String {
        let mut out = String::new();
        if let Some(doc) = &func.doc {
            out += &format!("/// {}\n", doc);
        }
        let rec_kw = if func.is_recursive { " rec" } else { "" };
        let inline_kw = if func.is_inline { " inline" } else { "" };
        let type_params = if func.type_params.is_empty() {
            String::new()
        } else {
            format!("<{}>", func.type_params.join(", "))
        };
        let params: Vec<String> = func
            .params
            .iter()
            .map(|(name, ty)| match ty {
                Some(t) => format!("({}: {})", name, self.emit_type(t)),
                None => name.clone(),
            })
            .collect();
        let ret_ann = func
            .return_type
            .as_ref()
            .map(|t| format!(": {} ", self.emit_type(t)))
            .unwrap_or_default();
        out += &format!(
            "let{}{}{} {}{} {}=\n    {}\n",
            rec_kw,
            inline_kw,
            type_params,
            func.name,
            if params.is_empty() {
                String::new()
            } else {
                format!(" {}", params.join(" "))
            },
            ret_ann,
            self.emit_expr(&func.body, 1)
        );
        out
    }
    /// Emit an [`FSharpModule`] as a complete F# source file.
    pub fn emit_module(&self, module: &FSharpModule) -> String {
        let mut out = String::new();
        out += "// Generated by OxiLean F# Backend\n\n";
        if module.auto_open {
            out += "[<AutoOpen>]\n";
        }
        out += &format!("module {}\n\n", module.name);
        for o in &module.opens {
            out += &format!("open {}\n", o);
        }
        if !module.opens.is_empty() {
            out.push('\n');
        }
        for rec in &module.records {
            out += &self.emit_record(rec);
            out.push('\n');
        }
        for union in &module.unions {
            out += &self.emit_union(union);
            out.push('\n');
        }
        for func in &module.functions {
            out += &self.emit_function(func);
            out.push('\n');
        }
        out
    }
}
/// An F# pattern used in `match` arms.
#[derive(Debug, Clone)]
pub enum FSharpPattern {
    /// Wildcard `_`
    Wildcard,
    /// Variable binding `name`
    Var(String),
    /// Literal pattern
    Lit(String),
    /// Tuple pattern `(a, b)`
    Tuple(Vec<FSharpPattern>),
    /// Constructor pattern `Some x` / `MyCase(a, b)`
    Ctor(String, Vec<FSharpPattern>),
    /// List pattern `[a; b]`
    FsList(Vec<FSharpPattern>),
    /// Cons pattern `h :: t`
    Cons(Box<FSharpPattern>, Box<FSharpPattern>),
    /// As-pattern `p as name`
    As(Box<FSharpPattern>, String),
    /// Guard: when `condition`
    When(Box<FSharpPattern>, Box<FSharpExpr>),
}
/// A unit of measure declaration.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct FSharpMeasure {
    /// Measure name (e.g. `kg`, `m`, `s`).
    pub name: String,
    /// Optional abbreviation.
    pub abbrev: Option<String>,
}
impl FSharpMeasure {
    /// Emit the measure declaration.
    #[allow(dead_code)]
    pub fn emit(&self) -> String {
        match &self.abbrev {
            None => format!("[<Measure>] type {}\n", self.name),
            Some(abbrev) => format!("[<Measure>] type {} = {}\n", self.name, abbrev),
        }
    }
}
/// An active pattern definition.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct FSharpActivePattern {
    /// The active pattern kind.
    pub kind: ActivePatternKind,
    /// Parameters.
    pub params: Vec<String>,
    /// Body.
    pub body: FSharpExpr,
}
impl FSharpActivePattern {
    /// Emit the active pattern.
    #[allow(dead_code)]
    pub fn emit(&self, backend: &FSharpBackend) -> String {
        let name = match &self.kind {
            ActivePatternKind::Total(cases) => format!("(|{}|)", cases.join("|")),
            ActivePatternKind::Partial(case) => format!("(|{}|_|)", case),
            ActivePatternKind::Parameterised(cases) => format!("(|{}|)", cases.join("|")),
        };
        let params = if self.params.is_empty() {
            String::new()
        } else {
            format!(" {}", self.params.join(" "))
        };
        format!(
            "let ({}){} =\n    {}\n",
            name,
            params,
            backend.emit_expr(&self.body, 1)
        )
    }
}
/// Fluent builder for constructing `FSharpModule` objects.
#[allow(dead_code)]
pub struct FSharpModuleBuilder {
    pub(super) module: FSharpModule,
}
impl FSharpModuleBuilder {
    /// Start building a module.
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>) -> Self {
        FSharpModuleBuilder {
            module: FSharpModule {
                name: name.into(),
                auto_open: false,
                records: vec![],
                unions: vec![],
                functions: vec![],
                opens: vec![],
            },
        }
    }
    /// Mark the module as `[<AutoOpen>]`.
    #[allow(dead_code)]
    pub fn auto_open(mut self) -> Self {
        self.module.auto_open = true;
        self
    }
    /// Add an `open` directive.
    #[allow(dead_code)]
    pub fn open(mut self, ns: impl Into<String>) -> Self {
        self.module.opens.push(ns.into());
        self
    }
    /// Add a record type.
    #[allow(dead_code)]
    pub fn record(mut self, rec: FSharpRecord) -> Self {
        self.module.records.push(rec);
        self
    }
    /// Add a discriminated union.
    #[allow(dead_code)]
    pub fn union(mut self, u: FSharpUnion) -> Self {
        self.module.unions.push(u);
        self
    }
    /// Add a function.
    #[allow(dead_code)]
    pub fn function(mut self, f: FSharpFunction) -> Self {
        self.module.functions.push(f);
        self
    }
    /// Finalise and return the module.
    #[allow(dead_code)]
    pub fn build(self) -> FSharpModule {
        self.module
    }
    /// Emit the module directly.
    #[allow(dead_code)]
    pub fn emit(self) -> String {
        FSharpBackend::new().emit_module(&self.module)
    }
}
/// Simple metrics gathered from an `FSharpModule`.
#[derive(Debug, Default)]
#[allow(dead_code)]
pub struct FSharpModuleMetrics {
    /// Number of type declarations.
    pub type_count: usize,
    /// Number of function declarations.
    pub function_count: usize,
    /// Number of recursive functions.
    pub recursive_count: usize,
    /// Number of inline functions.
    pub inline_count: usize,
    /// Estimated total source lines.
    pub estimated_lines: usize,
}
/// A discriminated union case with named fields (record-like).
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct FSharpUnionCaseNamed {
    /// Case name.
    pub name: String,
    /// Named fields: `(field_name, type)`.
    pub named_fields: Vec<(String, FSharpType)>,
}
impl FSharpUnionCaseNamed {
    /// Emit as `| Name of { field1: T1; field2: T2 }`.
    #[allow(dead_code)]
    pub fn emit(&self) -> String {
        if self.named_fields.is_empty() {
            return format!("    | {}\n", self.name);
        }
        let fields: Vec<String> = self
            .named_fields
            .iter()
            .map(|(n, t)| format!("{}: {}", n, t))
            .collect();
        format!("    | {} of {{ {} }}\n", self.name, fields.join("; "))
    }
}
/// Kind of F# active pattern.
#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum ActivePatternKind {
    /// Total active pattern `(|A|B|)`.
    Total(Vec<String>),
    /// Partial active pattern `(|A|_|)`.
    Partial(String),
    /// Parameterised active pattern `(|A|B|) arg`.
    Parameterised(Vec<String>),
}
/// Build common numeric helper functions.
#[allow(dead_code)]
pub struct FSharpNumericHelpers;
impl FSharpNumericHelpers {
    /// `clamp min max value` function.
    #[allow(dead_code)]
    pub fn clamp() -> FSharpFunction {
        FSharpFunction {
            name: "clamp".to_string(),
            is_recursive: false,
            is_inline: true,
            type_params: vec![],
            params: vec![
                ("lo".to_string(), Some(FSharpType::Int)),
                ("hi".to_string(), Some(FSharpType::Int)),
                ("x".to_string(), Some(FSharpType::Int)),
            ],
            return_type: Some(FSharpType::Int),
            body: FSharpExpr::Raw("max lo (min hi x)".to_string()),
            doc: Some("Clamp x to [lo, hi].".to_string()),
        }
    }
    /// `square x = x * x` function.
    #[allow(dead_code)]
    pub fn square() -> FSharpFunction {
        FSharpFunction {
            name: "square".to_string(),
            is_recursive: false,
            is_inline: true,
            type_params: vec![],
            params: vec![("x".to_string(), Some(FSharpType::Int))],
            return_type: Some(FSharpType::Int),
            body: fbinop("*", fvar("x"), fvar("x")),
            doc: Some("Square of x.".to_string()),
        }
    }
    /// `pow base exp` (integer exponentiation) function.
    #[allow(dead_code)]
    pub fn pow_int() -> FSharpFunction {
        FSharpFunction {
            name: "powInt".to_string(),
            is_recursive: true,
            is_inline: false,
            type_params: vec![],
            params: vec![
                ("b".to_string(), Some(FSharpType::Int)),
                ("n".to_string(), Some(FSharpType::Int)),
            ],
            return_type: Some(FSharpType::Int),
            body: FSharpExpr::Match(
                Box::new(fvar("n")),
                vec![
                    (plit("0"), fint(1)),
                    (
                        pvar("k"),
                        fbinop(
                            "*",
                            fvar("b"),
                            fapp_multi(
                                fvar("powInt"),
                                vec![fvar("b"), fbinop("-", fvar("k"), fint(1))],
                            ),
                        ),
                    ),
                ],
            ),
            doc: Some("Integer power.".to_string()),
        }
    }
    /// `gcd` function.
    #[allow(dead_code)]
    pub fn gcd() -> FSharpFunction {
        FSharpFunction {
            name: "gcd".to_string(),
            is_recursive: true,
            is_inline: false,
            type_params: vec![],
            params: vec![
                ("a".to_string(), Some(FSharpType::Int)),
                ("b".to_string(), Some(FSharpType::Int)),
            ],
            return_type: Some(FSharpType::Int),
            body: FSharpExpr::Match(
                Box::new(fvar("b")),
                vec![
                    (plit("0"), fvar("a")),
                    (
                        pvar("k"),
                        fapp_multi(
                            fvar("gcd"),
                            vec![fvar("k"), fbinop("%", fvar("a"), fvar("k"))],
                        ),
                    ),
                ],
            ),
            doc: Some("Greatest common divisor.".to_string()),
        }
    }
}
/// A record type declaration: `type Name = { field1: T1; field2: T2 }`.
#[derive(Debug, Clone)]
pub struct FSharpRecord {
    /// Type name
    pub name: String,
    /// Generic type parameters (e.g. `["'a", "'b"]`)
    pub type_params: Vec<String>,
    /// Fields: `(field_name, type)`
    pub fields: Vec<(String, FSharpType)>,
    /// Optional XML doc comment
    pub doc: Option<String>,
}
/// A type alias: `type Alias = ExistingType`.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct FSharpTypeAlias {
    /// Alias name.
    pub name: String,
    /// Generic type parameters.
    pub type_params: Vec<String>,
    /// The aliased type.
    pub aliased_type: FSharpType,
    /// Optional doc comment.
    pub doc: Option<String>,
}
impl FSharpTypeAlias {
    /// Emit the type alias.
    #[allow(dead_code)]
    pub fn emit(&self) -> String {
        let mut out = String::new();
        if let Some(doc) = &self.doc {
            out.push_str(&format!("/// {}\n", doc));
        }
        let params = if self.type_params.is_empty() {
            String::new()
        } else {
            format!("<{}>", self.type_params.join(", "))
        };
        out.push_str(&format!(
            "type {}{} = {}\n",
            self.name, params, self.aliased_type
        ));
        out
    }
}
/// Pre-built F# code snippets for common patterns.
#[allow(dead_code)]
pub struct FSharpSnippets;
impl FSharpSnippets {
    /// An option map function.
    #[allow(dead_code)]
    pub fn option_map() -> String {
        "let optionMap f = function\n    | Some x -> Some (f x)\n    | None -> None\n".to_string()
    }
    /// An option bind function.
    #[allow(dead_code)]
    pub fn option_bind() -> String {
        "let optionBind f = function\n    | Some x -> f x\n    | None -> None\n".to_string()
    }
    /// A result map function.
    #[allow(dead_code)]
    pub fn result_map() -> String {
        "let resultMap f = function\n    | Ok x -> Ok (f x)\n    | Error e -> Error e\n".to_string()
    }
    /// A list fold-left function.
    #[allow(dead_code)]
    pub fn list_fold() -> String {
        "let rec foldLeft f acc = function\n    | [] -> acc\n    | h :: t -> foldLeft f (f acc h) t\n"
            .to_string()
    }
    /// An identity function.
    #[allow(dead_code)]
    pub fn identity() -> String {
        "let id x = x\n".to_string()
    }
    /// A constant function.
    #[allow(dead_code)]
    pub fn constant() -> String {
        "let const x _ = x\n".to_string()
    }
    /// Function composition `>>`.
    #[allow(dead_code)]
    pub fn compose() -> String {
        "let compose f g x = g (f x)\n".to_string()
    }
    /// Flip a two-argument function.
    #[allow(dead_code)]
    pub fn flip() -> String {
        "let flip f x y = f y x\n".to_string()
    }
    /// Curry a function taking a tuple.
    #[allow(dead_code)]
    pub fn curry() -> String {
        "let curry f a b = f (a, b)\n".to_string()
    }
    /// Uncurry a curried function.
    #[allow(dead_code)]
    pub fn uncurry() -> String {
        "let uncurry f (a, b) = f a b\n".to_string()
    }
    /// A memoization function using a dictionary.
    #[allow(dead_code)]
    pub fn memoize() -> String {
        "open System.Collections.Generic\n\
         let memoize f =\n    \
             let cache = Dictionary<_,_>()\n    \
             fun x ->\n        \
                 match cache.TryGetValue(x) with\n        \
                 | true, v -> v\n        \
                 | _ ->\n            \
                     let v = f x\n            \
                     cache.[x] <- v\n            \
                     v\n"
        .to_string()
    }
    /// A fixed-point (Y combinator) function.
    #[allow(dead_code)]
    pub fn fix_point() -> String {
        "let fix f x = let rec go x = f go x in go x\n".to_string()
    }
}
/// A member in an F# class or object expression.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct FSharpMember {
    /// Member name.
    pub name: String,
    /// Parameters.
    pub params: Vec<(String, Option<FSharpType>)>,
    /// Return type annotation.
    pub return_type: Option<FSharpType>,
    /// Body expression.
    pub body: FSharpExpr,
    /// Whether this is an override.
    pub is_override: bool,
    /// Whether this is static.
    pub is_static: bool,
    /// Optional doc comment.
    pub doc: Option<String>,
}
impl FSharpMember {
    /// Emit the member.
    #[allow(dead_code)]
    pub fn emit(&self) -> String {
        let mut out = String::new();
        if let Some(doc) = &self.doc {
            out.push_str(&format!("    /// {}\n", doc));
        }
        let kw = if self.is_override {
            "override"
        } else {
            "member"
        };
        let stat = if self.is_static { " static" } else { "" };
        let params: Vec<String> = self
            .params
            .iter()
            .map(|(n, t)| match t {
                Some(ty) => format!("({}: {})", n, ty),
                None => n.clone(),
            })
            .collect();
        let ret = self
            .return_type
            .as_ref()
            .map(|t| format!(": {} ", t))
            .unwrap_or_default();
        out.push_str(&format!(
            "   {}{} this.{} {} {}=\n        {}\n",
            kw,
            stat,
            self.name,
            params.join(" "),
            ret,
            "..."
        ));
        out
    }
}
/// A computation expression kind.
#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum ComputationExprKind {
    /// `async { ... }` — asynchronous computation.
    Async,
    /// `seq { ... }` — lazy sequence.
    Seq,
    /// `result { ... }` — result monad.
    Result,
    /// `option { ... }` — option monad.
    OptionCe,
    /// Custom builder: `builder { ... }`.
    Custom(String),
}
/// A .NET/F# interface declaration.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct FSharpInterface {
    /// Interface name (conventionally starts with `I`).
    pub name: String,
    /// Generic type parameters.
    pub type_params: Vec<String>,
    /// Abstract method signatures: `(name, params, return_type)`.
    pub methods: Vec<(String, Vec<(String, FSharpType)>, FSharpType)>,
    /// Abstract property signatures: `(name, type)`.
    pub properties: Vec<(String, FSharpType, bool)>,
    /// Optional XML doc comment.
    pub doc: Option<String>,
}
impl FSharpInterface {
    /// Emit the interface declaration.
    #[allow(dead_code)]
    pub fn emit(&self) -> String {
        let mut out = String::new();
        if let Some(doc) = &self.doc {
            out.push_str(&format!("/// {}\n", doc));
        }
        let params = if self.type_params.is_empty() {
            String::new()
        } else {
            format!("<{}>", self.type_params.join(", "))
        };
        out.push_str(&format!("type {}{} =\n", self.name, params));
        for (name, params, ret) in &self.methods {
            let param_str: Vec<String> = params
                .iter()
                .map(|(n, t)| format!("{}: {}", n, t))
                .collect();
            out.push_str(&format!(
                "    abstract member {}: {} -> {}\n",
                name,
                param_str.join(" -> "),
                ret
            ));
        }
        for (name, ty, is_settable) in &self.properties {
            let access = if *is_settable { "get, set" } else { "get" };
            out.push_str(&format!(
                "    abstract member {}: {} with {}\n",
                name, ty, access
            ));
        }
        out
    }
}
/// A discriminated union case: `| Case of T1 * T2`.
#[derive(Debug, Clone)]
pub struct FSharpUnionCase {
    /// Constructor name (must start with uppercase)
    pub name: String,
    /// Payload types (empty for constant constructors)
    pub fields: Vec<FSharpType>,
}
/// An F# module definition.
#[derive(Debug, Clone)]
pub struct FSharpModule {
    /// Fully-qualified module name, e.g. `"OxiLean.Math"`
    pub name: String,
    /// Whether this is an `open` module (auto-opened)
    pub auto_open: bool,
    /// Record type declarations
    pub records: Vec<FSharpRecord>,
    /// Discriminated union declarations
    pub unions: Vec<FSharpUnion>,
    /// Function / value definitions
    pub functions: Vec<FSharpFunction>,
    /// `open` directives
    pub opens: Vec<String>,
}
/// A .NET attribute applied to a type, function, or property.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct FSharpAttribute {
    /// Attribute name (e.g. `Serializable`, `DllImport`).
    pub name: String,
    /// Positional arguments as raw strings.
    pub args: Vec<String>,
    /// Named arguments as `(key, value)` pairs.
    pub named_args: Vec<(String, String)>,
}
impl FSharpAttribute {
    /// Build a simple attribute with no arguments.
    #[allow(dead_code)]
    pub fn simple(name: impl Into<String>) -> Self {
        FSharpAttribute {
            name: name.into(),
            args: vec![],
            named_args: vec![],
        }
    }
    /// Build an attribute with positional arguments.
    #[allow(dead_code)]
    pub fn with_args(name: impl Into<String>, args: Vec<String>) -> Self {
        FSharpAttribute {
            name: name.into(),
            args,
            named_args: vec![],
        }
    }
    /// Emit the attribute as `[<Name(args)>]`.
    #[allow(dead_code)]
    pub fn emit(&self) -> String {
        if self.args.is_empty() && self.named_args.is_empty() {
            return format!("[<{}>]", self.name);
        }
        let mut parts: Vec<String> = self.args.clone();
        for (k, v) in &self.named_args {
            parts.push(format!("{} = {}", k, v));
        }
        format!("[<{}({})>]", self.name, parts.join(", "))
    }
}
/// An F# class declaration.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct FSharpClass {
    /// Class name.
    pub name: String,
    /// Generic type parameters.
    pub type_params: Vec<String>,
    /// Constructor parameters.
    pub ctor_params: Vec<(String, FSharpType)>,
    /// Member methods.
    pub members: Vec<FSharpMember>,
    /// Interfaces this class implements.
    pub implements: Vec<String>,
    /// Optional base class.
    pub inherits: Option<String>,
    /// Optional doc comment.
    pub doc: Option<String>,
    /// Attributes on the class.
    pub attributes: Vec<FSharpAttribute>,
}
impl FSharpClass {
    /// Emit the class declaration.
    #[allow(dead_code)]
    pub fn emit(&self) -> String {
        let mut out = String::new();
        for attr in &self.attributes {
            out.push_str(&format!("{}\n", attr.emit()));
        }
        if let Some(doc) = &self.doc {
            out.push_str(&format!("/// {}\n", doc));
        }
        let params = if self.type_params.is_empty() {
            String::new()
        } else {
            format!("<{}>", self.type_params.join(", "))
        };
        let ctor_str: Vec<String> = self
            .ctor_params
            .iter()
            .map(|(n, t)| format!("{}: {}", n, t))
            .collect();
        out.push_str(&format!(
            "type {}{}({}) =\n",
            self.name,
            params,
            ctor_str.join(", ")
        ));
        if let Some(base) = &self.inherits {
            out.push_str(&format!("    inherit {}\n", base));
        }
        for iface in &self.implements {
            out.push_str(&format!("    interface {}\n", iface));
        }
        for member in &self.members {
            out.push_str(&member.emit());
        }
        out
    }
}
