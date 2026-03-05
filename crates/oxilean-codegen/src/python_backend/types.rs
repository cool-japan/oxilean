//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;
use std::collections::HashMap;

use super::functions::*;
use super::functions::{FromImports, PYTHON_KEYWORDS};

/// A class variable (field) in a Python class.
#[derive(Debug, Clone, PartialEq)]
pub struct PythonClassVar {
    /// Field name
    pub name: String,
    /// Type annotation
    pub annotation: PythonType,
    /// Optional default value
    pub default: Option<PythonExpr>,
}
/// A Python function definition.
#[derive(Debug, Clone, PartialEq)]
pub struct PythonFunction {
    /// Function name
    pub name: String,
    /// Parameters
    pub params: Vec<PythonParam>,
    /// Return type annotation
    pub return_type: Option<PythonType>,
    /// Function body
    pub body: Vec<PythonStmt>,
    /// Decorator expressions (e.g. `property`, `classmethod`, `staticmethod`)
    pub decorators: Vec<String>,
    /// Whether this is an async function
    pub is_async: bool,
    /// Whether this is a classmethod
    pub is_classmethod: bool,
    /// Whether this is a staticmethod
    pub is_staticmethod: bool,
}
impl PythonFunction {
    /// Create a new function with just a name, empty body.
    pub fn new(name: impl Into<String>) -> Self {
        PythonFunction {
            name: name.into(),
            params: Vec::new(),
            return_type: None,
            body: Vec::new(),
            decorators: Vec::new(),
            is_async: false,
            is_classmethod: false,
            is_staticmethod: false,
        }
    }
}
/// A Python class definition.
#[derive(Debug, Clone, PartialEq)]
pub struct PythonClass {
    /// Class name
    pub name: String,
    /// Base classes
    pub bases: Vec<String>,
    /// Methods
    pub methods: Vec<PythonFunction>,
    /// Class variables / fields (for dataclasses)
    pub class_vars: Vec<PythonClassVar>,
    /// Whether to annotate with `@dataclass`
    pub is_dataclass: bool,
    /// Whether to annotate with `ABC` base or `@abstractmethod`
    pub is_abstract: bool,
    /// Additional decorators
    pub decorators: Vec<String>,
    /// Docstring
    pub docstring: Option<String>,
}
impl PythonClass {
    /// Create a new empty class.
    pub fn new(name: impl Into<String>) -> Self {
        PythonClass {
            name: name.into(),
            bases: Vec::new(),
            methods: Vec::new(),
            class_vars: Vec::new(),
            is_dataclass: false,
            is_abstract: false,
            decorators: Vec::new(),
            docstring: None,
        }
    }
}
/// Python function parameter with optional type annotation and default.
#[derive(Debug, Clone, PartialEq)]
pub struct PythonParam {
    /// Parameter name
    pub name: String,
    /// Optional type annotation
    pub annotation: Option<PythonType>,
    /// Optional default value
    pub default: Option<PythonExpr>,
    /// Whether this is a `*args` parameter
    pub is_vararg: bool,
    /// Whether this is a `**kwargs` parameter
    pub is_kwarg: bool,
    /// Whether this is a keyword-only parameter (after `*`)
    pub is_keyword_only: bool,
}
impl PythonParam {
    /// Create a simple parameter with just a name.
    pub fn simple(name: impl Into<String>) -> Self {
        PythonParam {
            name: name.into(),
            annotation: None,
            default: None,
            is_vararg: false,
            is_kwarg: false,
            is_keyword_only: false,
        }
    }
    /// Create a parameter with a type annotation.
    pub fn typed(name: impl Into<String>, ty: PythonType) -> Self {
        PythonParam {
            name: name.into(),
            annotation: Some(ty),
            default: None,
            is_vararg: false,
            is_kwarg: false,
            is_keyword_only: false,
        }
    }
}
/// A match case arm: `case <pattern>: <body>`
#[derive(Debug, Clone, PartialEq)]
pub struct MatchArm {
    /// The pattern (as raw Python text for flexibility)
    pub pattern: String,
    /// Optional guard: `if guard`
    pub guard: Option<PythonExpr>,
    /// Body statements
    pub body: Vec<PythonStmt>,
}
/// Python literal value.
#[derive(Debug, Clone, PartialEq)]
pub enum PythonLit {
    /// Integer literal: `42`, `-7`
    Int(i64),
    /// Float literal: `3.14`, `-0.5`
    Float(f64),
    /// String literal: `"hello"` or `'hello'`
    Str(String),
    /// Boolean literal: `True` or `False`
    Bool(bool),
    /// `None` literal
    None,
    /// Bytes literal: `b"data"`
    Bytes(Vec<u8>),
    /// Ellipsis: `...`
    Ellipsis,
}
/// Python code generation backend.
///
/// Compiles LCNF function declarations to a `PythonModule` containing
/// Python 3.10+ code with type hints.
pub struct PythonBackend {
    /// The module being built.
    pub module: PythonModule,
    /// Mapping from LCNF names to mangled Python names.
    pub fn_map: HashMap<String, String>,
    /// Counter for generating fresh temporary variable names.
    pub fresh_counter: usize,
}
impl PythonBackend {
    /// Create a new Python backend.
    pub fn new() -> Self {
        PythonBackend {
            module: PythonModule::new(),
            fn_map: HashMap::new(),
            fresh_counter: 0,
        }
    }
    /// Generate a fresh temporary variable name: `_t0`, `_t1`, etc.
    pub fn fresh_var(&mut self) -> String {
        let n = self.fresh_counter;
        self.fresh_counter += 1;
        format!("_t{}", n)
    }
    /// Mangle an LCNF name into a valid Python identifier.
    ///
    /// Rules:
    /// - Replace `.` with `_`
    /// - Replace `'` (prime) with `_prime`
    /// - Replace `-` with `_`
    /// - Prefix Python reserved words with `_`
    pub fn mangle_name(&self, name: &str) -> String {
        let mangled: String = name
            .chars()
            .map(|c| match c {
                '.' | '\'' | '-' | ' ' => '_',
                c if c.is_alphanumeric() || c == '_' => c,
                _ => '_',
            })
            .collect();
        if PYTHON_KEYWORDS.contains(&mangled.as_str())
            || mangled.starts_with(|c: char| c.is_ascii_digit())
        {
            format!("_{}", mangled)
        } else if mangled.is_empty() {
            "_anon".to_string()
        } else {
            mangled
        }
    }
    /// Top-level entry point: compile a slice of LCNF function declarations
    /// into a Python module string.
    pub fn compile_module(decls: &[LcnfFunDecl]) -> Result<String, String> {
        let mut backend = PythonBackend::new();
        backend.module.add_from_import(
            "typing",
            vec![
                ("Any".to_string(), None),
                ("Optional".to_string(), None),
                ("Union".to_string(), None),
                ("List".to_string(), None),
                ("Dict".to_string(), None),
                ("Tuple".to_string(), None),
                ("Callable".to_string(), None),
            ],
        );
        for decl in decls {
            let py_name = backend.mangle_name(&decl.name);
            backend.fn_map.insert(decl.name.clone(), py_name);
        }
        for decl in decls {
            let func = backend.compile_decl(decl)?;
            backend.module.add_function(func);
        }
        for decl in decls {
            if let Some(py_name) = backend.fn_map.get(&decl.name) {
                backend.module.all_exports.push(py_name.clone());
            }
        }
        Ok(backend.module.emit())
    }
    /// Compile a single LCNF function declaration into a `PythonFunction`.
    pub fn compile_decl(&mut self, decl: &LcnfFunDecl) -> Result<PythonFunction, String> {
        let py_name = self.mangle_name(&decl.name);
        let mut func = PythonFunction::new(py_name);
        for param in &decl.params {
            let param_name = format!("_v{}", param.id.0);
            func.params.push(PythonParam {
                name: param_name,
                annotation: Some(PythonType::Any),
                default: None,
                is_vararg: false,
                is_kwarg: false,
                is_keyword_only: false,
            });
        }
        func.return_type = Some(PythonType::Any);
        let body_stmts = self.compile_expr_to_stmts(&decl.body)?;
        func.body = body_stmts;
        Ok(func)
    }
    /// Compile an LCNF expression into a list of Python statements,
    /// ending with a `return`.
    pub(super) fn compile_expr_to_stmts(
        &mut self,
        expr: &LcnfExpr,
    ) -> Result<Vec<PythonStmt>, String> {
        self.compile_expr_stmts_inner(expr, &mut Vec::new())
    }
    /// Compile an LCNF expression into a sequence of Python statements.
    pub(super) fn compile_expr_stmts_inner(
        &mut self,
        expr: &LcnfExpr,
        stmts: &mut Vec<PythonStmt>,
    ) -> Result<Vec<PythonStmt>, String> {
        match expr {
            LcnfExpr::Let {
                name, value, body, ..
            } => {
                let py_name = self.mangle_name(name);
                let py_val = self.compile_let_value(value)?;
                stmts.push(PythonStmt::Assign(vec![PythonExpr::Var(py_name)], py_val));
                self.compile_expr_stmts_inner(body, stmts)
            }
            LcnfExpr::Return(arg) => {
                let py_expr = self.compile_arg(arg);
                stmts.push(PythonStmt::Return(Some(py_expr)));
                Ok(stmts.clone())
            }
            LcnfExpr::Unreachable => {
                stmts.push(PythonStmt::Raise(Some(PythonExpr::Call(
                    Box::new(PythonExpr::Var("RuntimeError".to_string())),
                    vec![PythonExpr::Lit(PythonLit::Str("unreachable".to_string()))],
                    vec![],
                ))));
                Ok(stmts.clone())
            }
            LcnfExpr::TailCall(func, args) => {
                let py_func = self.compile_arg(func);
                let py_args: Vec<PythonExpr> = args.iter().map(|a| self.compile_arg(a)).collect();
                stmts.push(PythonStmt::Return(Some(PythonExpr::Call(
                    Box::new(py_func),
                    py_args,
                    vec![],
                ))));
                Ok(stmts.clone())
            }
            LcnfExpr::Case {
                scrutinee,
                alts,
                default,
                ..
            } => {
                let scrutinee_name = format!("_v{}", scrutinee.0);
                let match_arms: Vec<MatchArm> = alts
                    .iter()
                    .map(|alt| {
                        let pattern = if alt.params.is_empty() {
                            alt.ctor_name.clone()
                        } else {
                            let param_names: Vec<String> =
                                alt.params.iter().map(|p| format!("_v{}", p.id.0)).collect();
                            format!("{}({})", alt.ctor_name, param_names.join(", "))
                        };
                        let mut arm_stmts = Vec::new();
                        let _ = self.compile_expr_stmts_inner(&alt.body, &mut arm_stmts);
                        MatchArm {
                            pattern,
                            guard: None,
                            body: arm_stmts,
                        }
                    })
                    .collect();
                let mut all_arms = match_arms;
                if let Some(def) = default {
                    let mut def_stmts = Vec::new();
                    let _ = self.compile_expr_stmts_inner(def, &mut def_stmts);
                    all_arms.push(MatchArm {
                        pattern: "_".to_string(),
                        guard: None,
                        body: def_stmts,
                    });
                }
                stmts.push(PythonStmt::Match(PythonExpr::Var(scrutinee_name), all_arms));
                Ok(stmts.clone())
            }
        }
    }
    /// Compile an LCNF argument (atomic value) into a Python expression.
    pub(super) fn compile_arg(&self, arg: &LcnfArg) -> PythonExpr {
        match arg {
            LcnfArg::Var(id) => PythonExpr::Var(format!("_v{}", id.0)),
            LcnfArg::Lit(lit) => self.compile_lit(lit),
            LcnfArg::Erased => PythonExpr::Lit(PythonLit::None),
            LcnfArg::Type(_) => PythonExpr::Lit(PythonLit::None),
        }
    }
    /// Compile an LCNF let-bound value into a Python expression.
    pub(super) fn compile_let_value(&mut self, value: &LcnfLetValue) -> Result<PythonExpr, String> {
        match value {
            LcnfLetValue::App(func, args) => {
                let py_func = self.compile_arg(func);
                let py_args: Vec<PythonExpr> = args.iter().map(|a| self.compile_arg(a)).collect();
                Ok(PythonExpr::Call(Box::new(py_func), py_args, vec![]))
            }
            LcnfLetValue::Proj(_, idx, var) => {
                let obj = PythonExpr::Var(format!("_v{}", var.0));
                Ok(PythonExpr::Subscript(
                    Box::new(obj),
                    Box::new(PythonExpr::Lit(PythonLit::Int(*idx as i64))),
                ))
            }
            LcnfLetValue::Ctor(name, _, args) => {
                let py_args: Vec<PythonExpr> = args.iter().map(|a| self.compile_arg(a)).collect();
                Ok(PythonExpr::Call(
                    Box::new(PythonExpr::Var(self.mangle_name(name))),
                    py_args,
                    vec![],
                ))
            }
            LcnfLetValue::Lit(lit) => Ok(self.compile_lit(lit)),
            LcnfLetValue::Erased => Ok(PythonExpr::Lit(PythonLit::None)),
            LcnfLetValue::FVar(id) => Ok(PythonExpr::Var(format!("_v{}", id.0))),
            LcnfLetValue::Reset(id) => Ok(PythonExpr::Var(format!("_v{}", id.0))),
            LcnfLetValue::Reuse(id, name, _, args) => {
                let py_args: Vec<PythonExpr> = args.iter().map(|a| self.compile_arg(a)).collect();
                let _ = id;
                Ok(PythonExpr::Call(
                    Box::new(PythonExpr::Var(self.mangle_name(name))),
                    py_args,
                    vec![],
                ))
            }
        }
    }
    /// Compile an LCNF literal into a Python expression.
    pub(super) fn compile_lit(&self, lit: &LcnfLit) -> PythonExpr {
        match lit {
            LcnfLit::Nat(n) => PythonExpr::Lit(PythonLit::Int(*n as i64)),
            LcnfLit::Str(s) => PythonExpr::Lit(PythonLit::Str(s.clone())),
        }
    }
    /// Emit the compiled module as a Python string.
    pub fn emit_module(&self) -> String {
        self.module.emit()
    }
}
/// Python type annotation for type-hinted code generation (PEP 484).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PythonType {
    /// `int`
    Int,
    /// `float`
    Float,
    /// `str`
    Str,
    /// `bool`
    Bool,
    /// `None`
    None_,
    /// `list[T]`
    List(Box<PythonType>),
    /// `dict[K, V]`
    Dict(Box<PythonType>, Box<PythonType>),
    /// `tuple[T1, T2, ...]`
    Tuple(Vec<PythonType>),
    /// `T | None` (Optional\[T\])
    Optional(Box<PythonType>),
    /// `T1 | T2 | ...` (Union)
    Union(Vec<PythonType>),
    /// User-defined type / class name
    Custom(String),
    /// `Any` (typing.Any)
    Any,
    /// `Callable[[A, B], R]`
    Callable,
    /// `set[T]`
    Set(Box<PythonType>),
    /// `frozenset[T]`
    FrozenSet(Box<PythonType>),
    /// `Generator[Y, S, R]`
    Generator(Box<PythonType>, Box<PythonType>, Box<PythonType>),
    /// `AsyncGenerator[Y, S]`
    AsyncGenerator(Box<PythonType>, Box<PythonType>),
    /// `Iterator[T]`
    Iterator(Box<PythonType>),
    /// `Iterable[T]`
    Iterable(Box<PythonType>),
    /// `Sequence[T]`
    Sequence(Box<PythonType>),
    /// `Mapping[K, V]`
    Mapping(Box<PythonType>, Box<PythonType>),
    /// `ClassVar[T]`
    ClassVar(Box<PythonType>),
    /// `Final[T]`
    Final(Box<PythonType>),
    /// `type[T]` (the class itself)
    Type(Box<PythonType>),
}
/// Part of an f-string.
#[derive(Debug, Clone, PartialEq)]
pub enum FStringPart {
    /// Raw string text
    Literal(String),
    /// Interpolated expression: `{expr}`
    Expr(PythonExpr),
    /// Interpolated expression with format spec: `{expr:.2f}`
    ExprWithFormat(PythonExpr, String),
}
/// Python statement for code generation.
#[derive(Debug, Clone, PartialEq)]
pub enum PythonStmt {
    /// Expression statement: `expr`
    Expr(PythonExpr),
    /// Simple assignment: `target = value`
    Assign(Vec<PythonExpr>, PythonExpr),
    /// Augmented assignment: `target op= value`
    AugAssign(PythonExpr, String, PythonExpr),
    /// Annotated assignment: `target: type = value`
    AnnAssign(String, PythonType, Option<PythonExpr>),
    /// If/elif/else statement
    If(
        PythonExpr,
        Vec<PythonStmt>,
        Vec<(PythonExpr, Vec<PythonStmt>)>,
        Vec<PythonStmt>,
    ),
    /// For loop: `for var in iter: body`
    For(String, PythonExpr, Vec<PythonStmt>, Vec<PythonStmt>),
    /// While loop: `while cond: body`
    While(PythonExpr, Vec<PythonStmt>, Vec<PythonStmt>),
    /// With statement: `with expr as var: body`
    With(Vec<(PythonExpr, Option<String>)>, Vec<PythonStmt>),
    /// Try/except/else/finally statement
    Try(
        Vec<PythonStmt>,
        Vec<(Option<PythonExpr>, Option<String>, Vec<PythonStmt>)>,
        Vec<PythonStmt>,
        Vec<PythonStmt>,
    ),
    /// Return statement: `return expr`
    Return(Option<PythonExpr>),
    /// Raise statement: `raise expr`
    Raise(Option<PythonExpr>),
    /// Delete statement: `del name`
    Del(Vec<PythonExpr>),
    /// Pass statement: `pass`
    Pass,
    /// Break statement: `break`
    Break,
    /// Continue statement: `continue`
    Continue,
    /// Import statement: `import module` or `import module as alias`
    Import(Vec<(String, Option<String>)>),
    /// From-import statement: `from module import name` or `from module import *`
    From(String, Vec<(String, Option<String>)>),
    /// Class definition
    ClassDef(PythonClass),
    /// Function definition
    FuncDef(PythonFunction),
    /// Async function definition
    AsyncFuncDef(PythonFunction),
    /// Docstring statement
    Docstring(String),
    /// Assert statement: `assert expr, msg`
    Assert(PythonExpr, Option<PythonExpr>),
    /// Global declaration: `global x, y`
    Global(Vec<String>),
    /// Nonlocal declaration: `nonlocal x, y`
    Nonlocal(Vec<String>),
    /// Match statement (Python 3.10+)
    Match(PythonExpr, Vec<MatchArm>),
    /// Raw Python text (escape hatch)
    Raw(String),
}
/// A complete Python module (one `.py` file).
#[derive(Debug, Clone)]
pub struct PythonModule {
    /// Module-level imports: `import X` or `import X as Y`
    pub imports: Vec<(String, Option<String>)>,
    /// From-imports: `from X import Y` or `from X import Y as Z`
    pub from_imports: FromImports,
    /// Top-level class definitions
    pub classes: Vec<PythonClass>,
    /// Top-level function definitions
    pub functions: Vec<PythonFunction>,
    /// Other top-level statements
    pub statements: Vec<PythonStmt>,
    /// Module docstring
    pub module_docstring: Option<String>,
    /// `__all__` exports
    pub all_exports: Vec<String>,
}
impl PythonModule {
    /// Create a new empty Python module.
    pub fn new() -> Self {
        PythonModule {
            imports: Vec::new(),
            from_imports: Vec::new(),
            classes: Vec::new(),
            functions: Vec::new(),
            statements: Vec::new(),
            module_docstring: None,
            all_exports: Vec::new(),
        }
    }
    /// Add a module-level import.
    pub fn add_import(&mut self, module: impl Into<String>, alias: Option<String>) {
        self.imports.push((module.into(), alias));
    }
    /// Add a from-import.
    pub fn add_from_import(
        &mut self,
        module: impl Into<String>,
        names: Vec<(String, Option<String>)>,
    ) {
        self.from_imports.push((module.into(), names));
    }
    /// Add a class definition.
    pub fn add_class(&mut self, cls: PythonClass) {
        self.classes.push(cls);
    }
    /// Add a function definition.
    pub fn add_function(&mut self, func: PythonFunction) {
        self.functions.push(func);
    }
    /// Add a top-level statement.
    pub fn add_statement(&mut self, stmt: PythonStmt) {
        self.statements.push(stmt);
    }
    /// Emit the full Python module as a string.
    pub fn emit(&self) -> String {
        let mut out = String::new();
        if let Some(doc) = &self.module_docstring {
            out.push_str(&format!("\"\"\"{}\"\"\"\n\n", doc));
        }
        let future_imports: Vec<_> = self
            .from_imports
            .iter()
            .filter(|(m, _)| m == "__future__")
            .collect();
        for (module, names) in &future_imports {
            out.push_str(&format_from_import(module, names));
            out.push('\n');
        }
        if !future_imports.is_empty() {
            out.push('\n');
        }
        for (module, alias) in &self.imports {
            match alias {
                Some(a) => out.push_str(&format!("import {} as {}\n", module, a)),
                None => out.push_str(&format!("import {}\n", module)),
            }
        }
        if !self.imports.is_empty() {
            out.push('\n');
        }
        let non_future: Vec<_> = self
            .from_imports
            .iter()
            .filter(|(m, _)| m != "__future__")
            .collect();
        for (module, names) in &non_future {
            out.push_str(&format_from_import(module, names));
            out.push('\n');
        }
        if !non_future.is_empty() {
            out.push('\n');
        }
        if !self.all_exports.is_empty() {
            out.push_str("__all__ = [");
            for (i, name) in self.all_exports.iter().enumerate() {
                if i > 0 {
                    out.push_str(", ");
                }
                out.push_str(&format!("\"{}\"", name));
            }
            out.push_str("]\n\n");
        }
        for cls in &self.classes {
            out.push_str(&emit_class(cls, 0));
            out.push_str("\n\n");
        }
        for func in &self.functions {
            out.push_str(&emit_function(func, 0));
            out.push_str("\n\n");
        }
        for stmt in &self.statements {
            out.push_str(&emit_stmt(stmt, 0));
            out.push('\n');
        }
        out
    }
}
/// Python expression for code generation.
#[derive(Debug, Clone, PartialEq)]
pub enum PythonExpr {
    /// A literal value: `42`, `"hello"`, `True`, `None`, etc.
    Lit(PythonLit),
    /// A variable identifier: `x`, `my_var`
    Var(String),
    /// Binary operator: `lhs + rhs`, `a == b`, etc.
    BinOp(String, Box<PythonExpr>, Box<PythonExpr>),
    /// Unary operator: `-x`, `not x`, `~x`
    UnaryOp(String, Box<PythonExpr>),
    /// Function call: `f(a, b, key=val)`
    Call(Box<PythonExpr>, Vec<PythonExpr>, Vec<(String, PythonExpr)>),
    /// Attribute access: `obj.field`
    Attr(Box<PythonExpr>, String),
    /// Subscript: `obj[idx]`
    Subscript(Box<PythonExpr>, Box<PythonExpr>),
    /// Lambda expression: `lambda x, y: x + y`
    Lambda(Vec<String>, Box<PythonExpr>),
    /// Conditional (ternary) expression: `a if cond else b`
    IfExpr(Box<PythonExpr>, Box<PythonExpr>, Box<PythonExpr>),
    /// List comprehension: `[expr for var in iter if cond]`
    ListComp(
        Box<PythonExpr>,
        String,
        Box<PythonExpr>,
        Option<Box<PythonExpr>>,
    ),
    /// Dict comprehension: `{k: v for k, v in items}`
    DictComp(
        Box<PythonExpr>,
        Box<PythonExpr>,
        String,
        String,
        Box<PythonExpr>,
    ),
    /// Set comprehension: `{x for x in iter}`
    SetComp(
        Box<PythonExpr>,
        String,
        Box<PythonExpr>,
        Option<Box<PythonExpr>>,
    ),
    /// Generator expression: `(expr for var in iter if cond)`
    GenExpr(
        Box<PythonExpr>,
        String,
        Box<PythonExpr>,
        Option<Box<PythonExpr>>,
    ),
    /// Tuple literal: `(a, b, c)` or `a, b, c`
    Tuple(Vec<PythonExpr>),
    /// List literal: `[a, b, c]`
    List(Vec<PythonExpr>),
    /// Dict literal: `{"key": val, ...}`
    Dict(Vec<(PythonExpr, PythonExpr)>),
    /// Set literal: `{a, b, c}`
    Set(Vec<PythonExpr>),
    /// Await expression: `await expr`
    Await(Box<PythonExpr>),
    /// Yield expression: `yield expr`
    Yield(Option<Box<PythonExpr>>),
    /// Yield from expression: `yield from expr`
    YieldFrom(Box<PythonExpr>),
    /// Match expression (Python 3.10+): used as target in match stmt
    Match(Box<PythonExpr>),
    /// f-string: `f"hello {name}!"`
    FString(Vec<FStringPart>),
    /// Walrus operator (assignment expression): `name := expr`
    Walrus(String, Box<PythonExpr>),
    /// Star expression: `*args`
    Star(Box<PythonExpr>),
    /// Double-star expression: `**kwargs`
    DoubleStar(Box<PythonExpr>),
    /// Slice: `a:b:c`
    Slice(
        Option<Box<PythonExpr>>,
        Option<Box<PythonExpr>>,
        Option<Box<PythonExpr>>,
    ),
}
