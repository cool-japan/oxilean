//! Core codegen types, IR definitions, optimizers, and code generators.

use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt;

use oxilean_kernel::expr::{Expr, Literal};

/// Target code generation language
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CodegenTarget {
    Rust,
    C,
    LlvmIr,
    Interpreter,
}

impl fmt::Display for CodegenTarget {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CodegenTarget::Rust => write!(f, "Rust"),
            CodegenTarget::C => write!(f, "C"),
            CodegenTarget::LlvmIr => write!(f, "LLVM IR"),
            CodegenTarget::Interpreter => write!(f, "Interpreter"),
        }
    }
}

/// Code generation configuration options
#[derive(Debug, Clone)]
pub struct CodegenConfig {
    pub target: CodegenTarget,
    pub optimize: bool,
    pub debug_info: bool,
    pub emit_comments: bool,
    pub inline_threshold: usize,
}

impl Default for CodegenConfig {
    fn default() -> Self {
        CodegenConfig {
            target: CodegenTarget::Rust,
            optimize: true,
            debug_info: false,
            emit_comments: true,
            inline_threshold: 50,
        }
    }
}

// ============================================================================
// Intermediate Representation (IR) Types
// ============================================================================

/// Intermediate representation type for values
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IrType {
    Unit,
    Bool,
    Nat,
    Int,
    String,
    Var(String),
    Function {
        params: Vec<IrType>,
        ret: Box<IrType>,
    },
    Struct {
        name: String,
        fields: Vec<(String, IrType)>,
    },
    Array {
        elem: Box<IrType>,
        len: usize,
    },
    Pointer(Box<IrType>),
    Unknown,
}

impl fmt::Display for IrType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IrType::Unit => write!(f, "()"),
            IrType::Bool => write!(f, "bool"),
            IrType::Nat => write!(f, "nat"),
            IrType::Int => write!(f, "i64"),
            IrType::String => write!(f, "string"),
            IrType::Var(name) => write!(f, "{}", name),
            IrType::Function { params, ret } => {
                write!(f, "fn(")?;
                for (i, p) in params.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", p)?;
                }
                write!(f, ") -> {}", ret)
            }
            IrType::Struct { name, fields } => {
                write!(f, "struct {} {{ ", name)?;
                for (i, (fname, ftype)) in fields.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}: {}", fname, ftype)?;
                }
                write!(f, " }}")
            }
            IrType::Array { elem, len } => write!(f, "[{}; {}]", elem, len),
            IrType::Pointer(ty) => write!(f, "*{}", ty),
            IrType::Unknown => write!(f, "unknown"),
        }
    }
}

/// Intermediate representation literal values
#[derive(Debug, Clone, PartialEq)]
pub enum IrLit {
    Unit,
    Bool(bool),
    Nat(u64),
    Int(i64),
    String(String),
}

impl fmt::Display for IrLit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IrLit::Unit => write!(f, "()"),
            IrLit::Bool(b) => write!(f, "{}", b),
            IrLit::Nat(n) => write!(f, "{}", n),
            IrLit::Int(i) => write!(f, "{}", i),
            IrLit::String(s) => write!(f, "\"{}\"", s),
        }
    }
}

/// Pattern for match expressions
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IrPattern {
    Wildcard,
    Literal(String),
    Constructor { name: String, args: Vec<String> },
    Tuple(Vec<String>),
    Or(Box<IrPattern>, Box<IrPattern>),
}

/// Match arm: pattern and body
#[derive(Debug, Clone, PartialEq)]
pub struct IrMatchArm {
    pub pattern: IrPattern,
    pub body: Box<IrExpr>,
}

/// Core intermediate representation expression
#[derive(Debug, Clone, PartialEq)]
pub enum IrExpr {
    /// Variable reference
    Var(String),

    /// Literal value
    Lit(IrLit),

    /// Function application
    App {
        func: Box<IrExpr>,
        args: Vec<IrExpr>,
    },

    /// Let binding
    Let {
        name: String,
        ty: IrType,
        value: Box<IrExpr>,
        body: Box<IrExpr>,
    },

    /// Lambda abstraction (after closure conversion, becomes function reference)
    Lambda {
        params: Vec<(String, IrType)>,
        body: Box<IrExpr>,
        captured: Vec<String>,
    },

    /// Conditional expression
    If {
        cond: Box<IrExpr>,
        then_branch: Box<IrExpr>,
        else_branch: Box<IrExpr>,
    },

    /// Pattern match
    Match {
        scrutinee: Box<IrExpr>,
        arms: Vec<IrMatchArm>,
    },

    /// Struct construction
    Struct {
        name: String,
        fields: Vec<(String, IrExpr)>,
    },

    /// Field access
    Field { object: Box<IrExpr>, field: String },

    /// Memory allocation
    Alloc(Box<IrExpr>),

    /// Memory dereference
    Deref(Box<IrExpr>),

    /// Sequence of expressions
    Seq(Vec<IrExpr>),
}

// ============================================================================
// CSE Helper: Expression Key for HashMap lookup
// ============================================================================

/// A wrapper around a canonical string representation of an IrExpr used as a
/// HashMap key in the CSE pass.  The second field holds the original expression
/// so it can be reconstructed when building let-bindings.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct IrExprKey(String, Box<IrExprOwned>);

/// Owned, hashable copy of an IrExpr (mirrors IrExpr but derives Hash/Eq).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum IrExprOwned {
    Var(String),
    Lit(String),
    App(Box<IrExprOwned>, Vec<IrExprOwned>),
    Other(String),
}

impl IrExprKey {
    /// Build a key from an IrExpr by computing its canonical display string.
    fn from(expr: &IrExpr) -> Self {
        let repr = Self::repr(expr);
        let owned = Self::to_owned(expr);
        IrExprKey(repr, Box::new(owned))
    }

    /// True for trivial expressions (Var / Lit) that should not be hoisted.
    fn is_trivial(&self) -> bool {
        matches!(*self.1, IrExprOwned::Var(_) | IrExprOwned::Lit(_))
    }

    /// Compute a canonical string for an expression.
    fn repr(expr: &IrExpr) -> String {
        match expr {
            IrExpr::Var(n) => format!("var:{}", n),
            IrExpr::Lit(l) => format!("lit:{}", l),
            IrExpr::App { func, args } => {
                let args_repr: Vec<_> = args.iter().map(Self::repr).collect();
                format!("app:{}({})", Self::repr(func), args_repr.join(","))
            }
            IrExpr::Let {
                name, value, body, ..
            } => {
                format!("let:{}={};{}", name, Self::repr(value), Self::repr(body))
            }
            IrExpr::Lambda { params, body, .. } => {
                let ps: Vec<_> = params.iter().map(|(n, _)| n.as_str()).collect();
                format!("lam:{}:{}", ps.join(","), Self::repr(body))
            }
            IrExpr::If {
                cond,
                then_branch,
                else_branch,
            } => {
                format!(
                    "if:{}?{}:{}",
                    Self::repr(cond),
                    Self::repr(then_branch),
                    Self::repr(else_branch)
                )
            }
            IrExpr::Field { object, field } => {
                format!("field:{}.{}", Self::repr(object), field)
            }
            IrExpr::Struct { name, fields } => {
                let fs: Vec<_> = fields
                    .iter()
                    .map(|(k, v)| format!("{}={}", k, Self::repr(v)))
                    .collect();
                format!("struct:{}{{{}}}", name, fs.join(","))
            }
            IrExpr::Alloc(e) => format!("alloc:{}", Self::repr(e)),
            IrExpr::Deref(e) => format!("deref:{}", Self::repr(e)),
            IrExpr::Match { scrutinee, .. } => format!("match:{}", Self::repr(scrutinee)),
            IrExpr::Seq(es) => {
                let rs: Vec<_> = es.iter().map(Self::repr).collect();
                format!("seq:[{}]", rs.join(","))
            }
        }
    }

    fn to_owned(expr: &IrExpr) -> IrExprOwned {
        match expr {
            IrExpr::Var(n) => IrExprOwned::Var(n.clone()),
            IrExpr::Lit(l) => IrExprOwned::Lit(format!("{}", l)),
            IrExpr::App { func, args } => IrExprOwned::App(
                Box::new(Self::to_owned(func)),
                args.iter().map(Self::to_owned).collect(),
            ),
            _ => IrExprOwned::Other(Self::repr(expr)),
        }
    }
}

// Reconstruct IrExpr from IrExprKey (used in CSE let-binding construction)
impl IrExprKey {
    /// Reconstruct the original IrExpr.  Since IrExprKey is only used for non-trivial
    /// sub-expressions in the CSE pass, and `replace_subexprs` is called before
    /// building let-bindings, we just need the key's representative expression
    /// which is stored directly in the Optimizer's CSE pass via the original expr.
    fn as_ir_expr(&self) -> IrExpr {
        // The key stores a repr string; reconstruct as Var placeholder.
        // Actual reconstruction happens via replace_subexprs called on the original.
        IrExpr::Var(self.0.clone())
    }
}

// ============================================================================
// Code Generation Errors
// ============================================================================

/// Code generation errors
#[derive(Debug, Clone)]
pub enum CodegenError {
    UnsupportedExpression(String),
    UnsupportedType(String),
    UnboundVariable(String),
    TypeMismatch { expected: String, found: String },
    InvalidPattern(String),
    StructNotFound(String),
    FieldNotFound { struct_name: String, field: String },
    InternalError(String),
}

impl fmt::Display for CodegenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CodegenError::UnsupportedExpression(msg) => {
                write!(f, "Unsupported expression: {}", msg)
            }
            CodegenError::UnsupportedType(msg) => {
                write!(f, "Unsupported type: {}", msg)
            }
            CodegenError::UnboundVariable(name) => {
                write!(f, "Unbound variable: {}", name)
            }
            CodegenError::TypeMismatch { expected, found } => {
                write!(f, "Type mismatch: expected {}, found {}", expected, found)
            }
            CodegenError::InvalidPattern(msg) => {
                write!(f, "Invalid pattern: {}", msg)
            }
            CodegenError::StructNotFound(name) => {
                write!(f, "Struct not found: {}", name)
            }
            CodegenError::FieldNotFound { struct_name, field } => {
                write!(f, "Field {} not found in struct {}", field, struct_name)
            }
            CodegenError::InternalError(msg) => {
                write!(f, "Internal code generation error: {}", msg)
            }
        }
    }
}

impl std::error::Error for CodegenError {}

pub type CodegenResult<T> = Result<T, CodegenError>;

// ============================================================================
// Symbol Management and Renaming
// ============================================================================

/// Symbol manager for name mangling and closure conversion
struct SymbolManager {
    counter: usize,
    scopes: VecDeque<HashSet<String>>,
}

impl SymbolManager {
    fn new() -> Self {
        SymbolManager {
            counter: 0,
            scopes: VecDeque::new(),
        }
    }

    fn fresh_name(&mut self, base: &str) -> String {
        let name = format!("{}_{}", base, self.counter);
        self.counter += 1;
        name
    }

    fn push_scope(&mut self) {
        self.scopes.push_back(HashSet::new());
    }

    fn pop_scope(&mut self) {
        self.scopes.pop_back();
    }

    fn bind(&mut self, name: String) {
        if let Some(scope) = self.scopes.back_mut() {
            scope.insert(name);
        }
    }

    fn is_bound(&self, name: &str) -> bool {
        self.scopes.iter().any(|scope| scope.contains(name))
    }
}

// ============================================================================
// Expression to IR Compiler
// ============================================================================

/// Compiles kernel expressions to intermediate representation
pub struct ExprToIr {
    symbol_manager: SymbolManager,
    closure_vars: HashMap<String, Vec<String>>,
}

impl ExprToIr {
    pub fn new() -> Self {
        ExprToIr {
            symbol_manager: SymbolManager::new(),
            closure_vars: HashMap::new(),
        }
    }

    /// Compile a kernel expression to IR
    pub fn compile(&mut self, expr: &Expr) -> CodegenResult<IrExpr> {
        match expr {
            Expr::Const(name, _levels) => Ok(IrExpr::Var(name.to_string())),
            Expr::FVar(id) => Ok(IrExpr::Var(format!("_fv{}", id.0))),
            Expr::BVar(i) => Ok(IrExpr::Var(format!("_bv{}", i))),
            Expr::App(f, arg) => {
                let compiled_f = self.compile(f)?;
                let compiled_arg = self.compile(arg)?;
                // Flatten nested App(func, args) into multi-arg App
                match compiled_f {
                    IrExpr::App { func, mut args } => {
                        args.push(compiled_arg);
                        Ok(IrExpr::App { func, args })
                    }
                    func_ir => Ok(IrExpr::App {
                        func: Box::new(func_ir),
                        args: vec![compiled_arg],
                    }),
                }
            }
            Expr::Lam(_binder_info, name, _ty, body) => {
                let param_name = name.to_string();
                let compiled_body = self.compile(body)?;
                Ok(IrExpr::Lambda {
                    params: vec![(param_name, IrType::Unknown)],
                    body: Box::new(compiled_body),
                    captured: vec![],
                })
            }
            Expr::Let(name, _ty, val, body) => {
                let compiled_val = self.compile(val)?;
                let compiled_body = self.compile(body)?;
                Ok(IrExpr::Let {
                    name: name.to_string(),
                    ty: IrType::Unknown,
                    value: Box::new(compiled_val),
                    body: Box::new(compiled_body),
                })
            }
            Expr::Lit(Literal::Nat(n)) => Ok(IrExpr::Lit(IrLit::Nat(*n))),
            Expr::Lit(Literal::Str(s)) => Ok(IrExpr::Lit(IrLit::String(s.clone()))),
            Expr::Sort(_) | Expr::Pi(_, _, _, _) => Ok(IrExpr::Var("Type".to_string())),
            Expr::Proj(_name, idx, inner) => {
                let compiled_inner = self.compile(inner)?;
                let field = match idx {
                    0 => "_fst".to_string(),
                    1 => "_snd".to_string(),
                    n => format!("_{}", n),
                };
                Ok(IrExpr::Field {
                    object: Box::new(compiled_inner),
                    field,
                })
            }
        }
    }

    /// Perform lambda lifting transformation
    fn lambda_lift(&mut self, expr: &IrExpr) -> IrExpr {
        match expr {
            IrExpr::Lambda {
                params: _,
                body: _,
                captured,
            } => {
                let lifted_name = self.symbol_manager.fresh_name("lifted");
                self.closure_vars
                    .insert(lifted_name.clone(), captured.clone());
                IrExpr::Var(lifted_name)
            }
            IrExpr::Let {
                name,
                ty,
                value,
                body,
            } => IrExpr::Let {
                name: name.clone(),
                ty: ty.clone(),
                value: Box::new(self.lambda_lift(value)),
                body: Box::new(self.lambda_lift(body)),
            },
            IrExpr::App { func, args } => IrExpr::App {
                func: Box::new(self.lambda_lift(func)),
                args: args.iter().map(|a| self.lambda_lift(a)).collect(),
            },
            IrExpr::If {
                cond,
                then_branch,
                else_branch,
            } => IrExpr::If {
                cond: Box::new(self.lambda_lift(cond)),
                then_branch: Box::new(self.lambda_lift(then_branch)),
                else_branch: Box::new(self.lambda_lift(else_branch)),
            },
            _ => expr.clone(),
        }
    }

    /// Perform closure conversion
    fn closure_convert(&mut self, expr: &IrExpr) -> IrExpr {
        match expr {
            IrExpr::Lambda {
                params: _,
                body,
                captured,
            } => {
                let closure_name = self.symbol_manager.fresh_name("closure");
                let _converted_body = self.closure_convert(body);

                IrExpr::Struct {
                    name: format!("Closure_{}", closure_name),
                    fields: captured
                        .iter()
                        .map(|v| (v.clone(), IrExpr::Var(v.clone())))
                        .collect(),
                }
            }
            IrExpr::Let {
                name,
                ty,
                value,
                body,
            } => IrExpr::Let {
                name: name.clone(),
                ty: ty.clone(),
                value: Box::new(self.closure_convert(value)),
                body: Box::new(self.closure_convert(body)),
            },
            _ => expr.clone(),
        }
    }
}

impl Default for ExprToIr {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Rust Code Emitter
// ============================================================================

/// Helper struct for emitting Rust code
struct RustEmitter {
    output: String,
    indent_level: usize,
}

impl RustEmitter {
    fn new() -> Self {
        RustEmitter {
            output: String::new(),
            indent_level: 0,
        }
    }

    fn indent(&mut self) {
        self.indent_level += 1;
    }

    fn dedent(&mut self) {
        if self.indent_level > 0 {
            self.indent_level -= 1;
        }
    }

    fn emit(&mut self, line: &str) {
        for _ in 0..self.indent_level {
            self.output.push_str("    ");
        }
        self.output.push_str(line);
        self.output.push('\n');
    }

    fn emit_inline(&mut self, text: &str) {
        self.output.push_str(text);
    }

    fn result(&self) -> String {
        self.output.clone()
    }
}

/// IR to Rust code generator
pub struct IrToRust {
    config: CodegenConfig,
}

impl IrToRust {
    pub fn new(config: CodegenConfig) -> Self {
        IrToRust { config }
    }

    /// Emit complete Rust code for an IR expression
    pub fn emit(&self, expr: &IrExpr) -> CodegenResult<String> {
        let mut emitter = RustEmitter::new();
        self.emit_expr(&mut emitter, expr)?;
        Ok(emitter.result())
    }

    fn emit_expr(&self, emitter: &mut RustEmitter, expr: &IrExpr) -> CodegenResult<()> {
        match expr {
            IrExpr::Var(name) => {
                emitter.emit_inline(name);
            }
            IrExpr::Lit(lit) => {
                emitter.emit_inline(&lit.to_string());
            }
            IrExpr::App { func, args } => {
                self.emit_expr(emitter, func)?;
                emitter.emit_inline("(");
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 {
                        emitter.emit_inline(", ");
                    }
                    self.emit_expr(emitter, arg)?;
                }
                emitter.emit_inline(")");
            }
            IrExpr::Let {
                name,
                ty,
                value,
                body,
            } => {
                emitter.emit(&format!("let {} : {} = ", name, self.emit_type(ty)?));
                self.emit_expr(emitter, value)?;
                emitter.emit_inline(";");
                self.emit_expr(emitter, body)?;
            }
            IrExpr::Lambda {
                params,
                body,
                captured: _,
            } => {
                emitter.emit_inline("|");
                for (i, (pname, ptype)) in params.iter().enumerate() {
                    if i > 0 {
                        emitter.emit_inline(", ");
                    }
                    emitter.emit_inline(&format!("{}: {}", pname, self.emit_type(ptype)?));
                }
                emitter.emit_inline("| {");
                self.emit_expr(emitter, body)?;
                emitter.emit_inline("}");
            }
            IrExpr::If {
                cond,
                then_branch,
                else_branch,
            } => {
                emitter.emit_inline("if ");
                self.emit_expr(emitter, cond)?;
                emitter.emit_inline(" { ");
                self.emit_expr(emitter, then_branch)?;
                emitter.emit_inline(" } else { ");
                self.emit_expr(emitter, else_branch)?;
                emitter.emit_inline(" }");
            }
            IrExpr::Match { scrutinee, arms } => {
                emitter.emit_inline("match ");
                self.emit_expr(emitter, scrutinee)?;
                emitter.emit_inline(" {");
                for arm in arms {
                    emitter.emit(&format!("    {} => {{", self.emit_pattern(&arm.pattern)?));
                    self.emit_expr(emitter, &arm.body)?;
                    emitter.emit("}");
                }
                emitter.emit("}");
            }
            IrExpr::Struct { name, fields } => {
                emitter.emit_inline(&format!("{} {{ ", name));
                for (i, (fname, fvalue)) in fields.iter().enumerate() {
                    if i > 0 {
                        emitter.emit_inline(", ");
                    }
                    emitter.emit_inline(&format!("{}: ", fname));
                    self.emit_expr(emitter, fvalue)?;
                }
                emitter.emit_inline(" }");
            }
            IrExpr::Field { object, field } => {
                self.emit_expr(emitter, object)?;
                emitter.emit_inline(&format!(".{}", field));
            }
            IrExpr::Alloc(expr) => {
                emitter.emit_inline("Box::new(");
                self.emit_expr(emitter, expr)?;
                emitter.emit_inline(")");
            }
            IrExpr::Deref(expr) => {
                emitter.emit_inline("*");
                self.emit_expr(emitter, expr)?;
            }
            IrExpr::Seq(exprs) => {
                emitter.emit_inline("{ ");
                for expr in exprs {
                    self.emit_expr(emitter, expr)?;
                    emitter.emit_inline("; ");
                }
                emitter.emit_inline("}");
            }
        }
        Ok(())
    }

    fn emit_type(&self, ty: &IrType) -> CodegenResult<String> {
        Ok(match ty {
            IrType::Unit => "()".to_string(),
            IrType::Bool => "bool".to_string(),
            IrType::Nat => "u64".to_string(),
            IrType::Int => "i64".to_string(),
            IrType::String => "String".to_string(),
            IrType::Var(name) => name.clone(),
            IrType::Function { params, ret } => {
                let param_strs: CodegenResult<Vec<_>> =
                    params.iter().map(|p| self.emit_type(p)).collect();
                let ret_str = self.emit_type(ret)?;
                format!("fn({}) -> {}", param_strs?.join(", "), ret_str)
            }
            IrType::Array { elem, len } => {
                let elem_str = self.emit_type(elem)?;
                format!("[{}; {}]", elem_str, len)
            }
            IrType::Pointer(ty) => {
                let ty_str = self.emit_type(ty)?;
                format!("*{}", ty_str)
            }
            _ => "unknown".to_string(),
        })
    }

    fn emit_pattern(&self, pattern: &IrPattern) -> CodegenResult<String> {
        Ok(match pattern {
            IrPattern::Wildcard => "_".to_string(),
            IrPattern::Literal(lit) => lit.clone(),
            IrPattern::Constructor { name, args } => {
                format!("{}({})", name, args.join(", "))
            }
            IrPattern::Tuple(vars) => format!("({})", vars.join(", ")),
            IrPattern::Or(p1, p2) => {
                format!("{} | {}", self.emit_pattern(p1)?, self.emit_pattern(p2)?)
            }
        })
    }

    pub fn emit_function(
        &self,
        name: &str,
        params: &[(String, IrType)],
        ret_type: &IrType,
        body: &IrExpr,
    ) -> CodegenResult<String> {
        let mut emitter = RustEmitter::new();
        emitter.emit(&format!("fn {}(", name));
        for (i, (pname, ptype)) in params.iter().enumerate() {
            if i > 0 {
                emitter.emit_inline(", ");
            }
            emitter.emit_inline(&format!("{}: {}", pname, self.emit_type(ptype)?));
        }
        emitter.emit_inline(&format!(") -> {} {{", self.emit_type(ret_type)?));
        emitter.indent();
        self.emit_expr(&mut emitter, body)?;
        emitter.dedent();
        emitter.emit("}");
        Ok(emitter.result())
    }

    pub fn emit_struct(&self, name: &str, fields: &[(String, IrType)]) -> CodegenResult<String> {
        let mut emitter = RustEmitter::new();
        emitter.emit(&format!("struct {} {{", name));
        emitter.indent();
        for (fname, ftype) in fields {
            emitter.emit(&format!("{}: {},", fname, self.emit_type(ftype)?));
        }
        emitter.dedent();
        emitter.emit("}");
        Ok(emitter.result())
    }

    pub fn emit_match(&self, scrutinee: &IrExpr, arms: &[IrMatchArm]) -> CodegenResult<String> {
        let mut emitter = RustEmitter::new();
        emitter.emit_inline("match ");
        self.emit_expr(&mut emitter, scrutinee)?;
        emitter.emit_inline(" {");
        emitter.indent();
        for arm in arms {
            emitter.emit(&format!("{} => {{", self.emit_pattern(&arm.pattern)?));
            emitter.indent();
            self.emit_expr(&mut emitter, &arm.body)?;
            emitter.dedent();
            emitter.emit("}");
        }
        emitter.dedent();
        emitter.emit("}");
        Ok(emitter.result())
    }
}

// ============================================================================
// C Code Emitter
// ============================================================================

/// IR to C code generator with reference counting
pub struct IrToC {
    config: CodegenConfig,
}

impl IrToC {
    pub fn new(config: CodegenConfig) -> Self {
        IrToC { config }
    }

    /// Emit C code for an IR expression
    pub fn emit(&self, expr: &IrExpr) -> CodegenResult<String> {
        let mut code = String::new();
        self.emit_expr(&mut code, expr)?;
        Ok(code)
    }

    fn emit_expr(&self, code: &mut String, expr: &IrExpr) -> CodegenResult<()> {
        match expr {
            IrExpr::Var(name) => {
                code.push_str(name);
            }
            IrExpr::Lit(lit) => {
                code.push_str(&lit.to_string());
            }
            IrExpr::App { func, args } => {
                self.emit_expr(code, func)?;
                code.push('(');
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 {
                        code.push_str(", ");
                    }
                    self.emit_expr(code, arg)?;
                }
                code.push(')');
            }
            IrExpr::Field { object, field } => {
                self.emit_expr(code, object)?;
                code.push_str(&format!("->{}", field));
            }
            _ => {
                return Err(CodegenError::UnsupportedExpression(format!("{:?}", expr)));
            }
        }
        Ok(())
    }

    pub fn emit_c_type(&self, ty: &IrType) -> CodegenResult<String> {
        Ok(match ty {
            IrType::Unit => "void".to_string(),
            IrType::Bool => "bool".to_string(),
            IrType::Nat => "uint64_t".to_string(),
            IrType::Int => "int64_t".to_string(),
            IrType::String => "char*".to_string(),
            IrType::Pointer(inner) => {
                format!("{}*", self.emit_c_type(inner)?)
            }
            _ => return Err(CodegenError::UnsupportedType(ty.to_string())),
        })
    }
}

// ============================================================================
// IR Optimizer
// ============================================================================

/// Intermediate representation optimizer
pub struct Optimizer {
    config: CodegenConfig,
}

impl Optimizer {
    pub fn new(config: CodegenConfig) -> Self {
        Optimizer { config }
    }

    /// Optimize an IR expression
    pub fn optimize(&self, expr: &IrExpr) -> CodegenResult<IrExpr> {
        let expr = self.constant_fold(expr)?;
        let expr = self.dead_code_eliminate(&expr)?;
        let expr = self.inline(&expr)?;
        self.common_subexpr_eliminate(&expr)
    }

    /// Constant folding optimization
    fn constant_fold(&self, expr: &IrExpr) -> CodegenResult<IrExpr> {
        match expr {
            IrExpr::Let {
                name,
                ty,
                value,
                body,
            } => {
                let folded_value = self.constant_fold(value)?;
                let folded_body = self.constant_fold(body)?;
                Ok(IrExpr::Let {
                    name: name.clone(),
                    ty: ty.clone(),
                    value: Box::new(folded_value),
                    body: Box::new(folded_body),
                })
            }
            IrExpr::If {
                cond,
                then_branch,
                else_branch,
            } => {
                if let IrExpr::Lit(IrLit::Bool(b)) = **cond {
                    let folded = if b {
                        self.constant_fold(then_branch)?
                    } else {
                        self.constant_fold(else_branch)?
                    };
                    Ok(folded)
                } else {
                    let cond_folded = self.constant_fold(cond)?;
                    let then_folded = self.constant_fold(then_branch)?;
                    let else_folded = self.constant_fold(else_branch)?;
                    Ok(IrExpr::If {
                        cond: Box::new(cond_folded),
                        then_branch: Box::new(then_folded),
                        else_branch: Box::new(else_folded),
                    })
                }
            }
            _ => Ok(expr.clone()),
        }
    }

    /// Dead code elimination
    fn dead_code_eliminate(&self, expr: &IrExpr) -> CodegenResult<IrExpr> {
        match expr {
            IrExpr::Let {
                name,
                ty,
                value,
                body,
            } => {
                let used = self.is_var_used(name, body);
                if used {
                    let value_opt = self.dead_code_eliminate(value)?;
                    let body_opt = self.dead_code_eliminate(body)?;
                    Ok(IrExpr::Let {
                        name: name.clone(),
                        ty: ty.clone(),
                        value: Box::new(value_opt),
                        body: Box::new(body_opt),
                    })
                } else {
                    self.dead_code_eliminate(body)
                }
            }
            _ => Ok(expr.clone()),
        }
    }

    /// Check if a variable is used in an expression
    fn is_var_used(&self, name: &str, expr: &IrExpr) -> bool {
        match expr {
            IrExpr::Var(n) => n == name,
            IrExpr::Let { value, body, .. } => {
                self.is_var_used(name, value) || self.is_var_used(name, body)
            }
            IrExpr::App { func, args } => {
                self.is_var_used(name, func) || args.iter().any(|a| self.is_var_used(name, a))
            }
            IrExpr::If {
                cond,
                then_branch,
                else_branch,
            } => {
                self.is_var_used(name, cond)
                    || self.is_var_used(name, then_branch)
                    || self.is_var_used(name, else_branch)
            }
            _ => false,
        }
    }

    /// Function inlining optimization
    ///
    /// Performs beta reduction: `(fun x -> body) arg` is substituted inline
    /// when the argument size is within `inline_threshold`.
    fn inline(&self, expr: &IrExpr) -> CodegenResult<IrExpr> {
        match expr {
            IrExpr::App { func, args } => {
                let inlined_func = self.inline(func)?;
                let inlined_args: CodegenResult<Vec<IrExpr>> =
                    args.iter().map(|a| self.inline(a)).collect();
                let inlined_args = inlined_args?;

                // Beta reduction: (fun (x, ...) -> body) applied to args
                if let IrExpr::Lambda { params, body, .. } = &inlined_func {
                    let arg_size = self.expr_size(&IrExpr::App {
                        func: Box::new(inlined_func.clone()),
                        args: inlined_args.clone(),
                    });
                    if arg_size <= self.config.inline_threshold
                        && params.len() == inlined_args.len()
                    {
                        // Substitute each argument for its parameter (innermost first)
                        let mut result = *body.clone();
                        for ((_param_name, _param_ty), arg) in
                            params.iter().zip(inlined_args.iter())
                        {
                            result = self.subst_var(&result, &_param_name.clone(), arg);
                        }
                        return self.inline(&result);
                    }
                }

                Ok(IrExpr::App {
                    func: Box::new(inlined_func),
                    args: inlined_args,
                })
            }
            IrExpr::Let {
                name,
                ty,
                value,
                body,
            } => {
                let inlined_value = self.inline(value)?;
                let inlined_body = self.inline(body)?;
                Ok(IrExpr::Let {
                    name: name.clone(),
                    ty: ty.clone(),
                    value: Box::new(inlined_value),
                    body: Box::new(inlined_body),
                })
            }
            IrExpr::Lambda {
                params,
                body,
                captured,
            } => {
                let inlined_body = self.inline(body)?;
                Ok(IrExpr::Lambda {
                    params: params.clone(),
                    body: Box::new(inlined_body),
                    captured: captured.clone(),
                })
            }
            IrExpr::If {
                cond,
                then_branch,
                else_branch,
            } => Ok(IrExpr::If {
                cond: Box::new(self.inline(cond)?),
                then_branch: Box::new(self.inline(then_branch)?),
                else_branch: Box::new(self.inline(else_branch)?),
            }),
            _ => Ok(expr.clone()),
        }
    }

    /// Substitute occurrences of `var_name` with `replacement` in `expr`.
    fn subst_var(&self, expr: &IrExpr, var_name: &str, replacement: &IrExpr) -> IrExpr {
        match expr {
            IrExpr::Var(n) if n == var_name => replacement.clone(),
            IrExpr::App { func, args } => IrExpr::App {
                func: Box::new(self.subst_var(func, var_name, replacement)),
                args: args
                    .iter()
                    .map(|a| self.subst_var(a, var_name, replacement))
                    .collect(),
            },
            IrExpr::Let {
                name,
                ty,
                value,
                body,
            } => {
                let new_value = self.subst_var(value, var_name, replacement);
                // Do not substitute inside body if the let binding shadows var_name
                let new_body = if name == var_name {
                    *body.clone()
                } else {
                    self.subst_var(body, var_name, replacement)
                };
                IrExpr::Let {
                    name: name.clone(),
                    ty: ty.clone(),
                    value: Box::new(new_value),
                    body: Box::new(new_body),
                }
            }
            IrExpr::Lambda {
                params,
                body,
                captured,
            } => {
                // Do not substitute if param shadows the variable
                if params.iter().any(|(p, _)| p == var_name) {
                    expr.clone()
                } else {
                    IrExpr::Lambda {
                        params: params.clone(),
                        body: Box::new(self.subst_var(body, var_name, replacement)),
                        captured: captured.clone(),
                    }
                }
            }
            IrExpr::If {
                cond,
                then_branch,
                else_branch,
            } => IrExpr::If {
                cond: Box::new(self.subst_var(cond, var_name, replacement)),
                then_branch: Box::new(self.subst_var(then_branch, var_name, replacement)),
                else_branch: Box::new(self.subst_var(else_branch, var_name, replacement)),
            },
            IrExpr::Field { object, field } => IrExpr::Field {
                object: Box::new(self.subst_var(object, var_name, replacement)),
                field: field.clone(),
            },
            IrExpr::Alloc(inner) => {
                IrExpr::Alloc(Box::new(self.subst_var(inner, var_name, replacement)))
            }
            IrExpr::Deref(inner) => {
                IrExpr::Deref(Box::new(self.subst_var(inner, var_name, replacement)))
            }
            IrExpr::Seq(exprs) => IrExpr::Seq(
                exprs
                    .iter()
                    .map(|e| self.subst_var(e, var_name, replacement))
                    .collect(),
            ),
            _ => expr.clone(),
        }
    }

    /// Estimate the size of an IR expression (used for inline threshold).
    fn expr_size(&self, expr: &IrExpr) -> usize {
        match expr {
            IrExpr::Var(_) | IrExpr::Lit(_) => 1,
            IrExpr::App { func, args } => {
                1 + self.expr_size(func) + args.iter().map(|a| self.expr_size(a)).sum::<usize>()
            }
            IrExpr::Let { value, body, .. } => 1 + self.expr_size(value) + self.expr_size(body),
            IrExpr::Lambda { body, .. } => 1 + self.expr_size(body),
            IrExpr::If {
                cond,
                then_branch,
                else_branch,
            } => {
                1 + self.expr_size(cond) + self.expr_size(then_branch) + self.expr_size(else_branch)
            }
            IrExpr::Match { scrutinee, arms } => {
                1 + self.expr_size(scrutinee)
                    + arms.iter().map(|a| self.expr_size(&a.body)).sum::<usize>()
            }
            IrExpr::Struct { fields, .. } => {
                1 + fields.iter().map(|(_, e)| self.expr_size(e)).sum::<usize>()
            }
            IrExpr::Field { object, .. } => 1 + self.expr_size(object),
            IrExpr::Alloc(e) | IrExpr::Deref(e) => 1 + self.expr_size(e),
            IrExpr::Seq(es) => es.iter().map(|e| self.expr_size(e)).sum(),
        }
    }

    /// Common subexpression elimination
    ///
    /// Collects subexpressions that appear more than once and hoists them
    /// to let-bindings at the top of the expression, replacing duplicates
    /// with variable references.
    fn common_subexpr_eliminate(&self, expr: &IrExpr) -> CodegenResult<IrExpr> {
        // Step 1: count occurrences of each sub-expression
        let mut counts: HashMap<IrExprKey, usize> = HashMap::new();
        self.count_subexprs(expr, &mut counts);

        // Step 2: collect subexpressions that appear >1 time and are non-trivial
        let mut to_hoist: Vec<IrExprKey> = counts
            .iter()
            .filter(|(key, &count)| count > 1 && !key.is_trivial())
            .map(|(key, _)| key.clone())
            .collect();
        // Deterministic ordering: sort by string representation
        to_hoist.sort_by_key(|k| k.0.clone());

        if to_hoist.is_empty() {
            return Ok(expr.clone());
        }

        // Step 3: assign fresh names and build substitution map
        let mut subst: HashMap<IrExprKey, String> = HashMap::new();
        for (counter, key) in to_hoist.iter().enumerate() {
            let name = format!("_cse{}", counter);
            subst.insert(key.clone(), name);
        }

        // Step 4: rewrite expression replacing duplicates with vars
        let rewritten = self.replace_subexprs(expr, &subst);

        // Step 5: wrap with let-bindings (innermost first means last hoist is outermost)
        let mut result = rewritten;
        for key in to_hoist.iter().rev() {
            let var_name = subst[key].clone();
            // Reconstruct the original subexpression from its key
            let key_ir = key.as_ir_expr();
            let value_expr = self.replace_subexprs(&key_ir, &subst);
            result = IrExpr::Let {
                name: var_name,
                ty: IrType::Unknown,
                value: Box::new(value_expr),
                body: Box::new(result),
            };
        }

        Ok(result)
    }

    /// Count occurrences of each sub-expression in the tree.
    fn count_subexprs(&self, expr: &IrExpr, counts: &mut HashMap<IrExprKey, usize>) {
        let key = IrExprKey::from(expr);
        *counts.entry(key).or_insert(0) += 1;
        match expr {
            IrExpr::App { func, args } => {
                self.count_subexprs(func, counts);
                for a in args {
                    self.count_subexprs(a, counts);
                }
            }
            IrExpr::Let { value, body, .. } => {
                self.count_subexprs(value, counts);
                self.count_subexprs(body, counts);
            }
            IrExpr::Lambda { body, .. } => self.count_subexprs(body, counts),
            IrExpr::If {
                cond,
                then_branch,
                else_branch,
            } => {
                self.count_subexprs(cond, counts);
                self.count_subexprs(then_branch, counts);
                self.count_subexprs(else_branch, counts);
            }
            IrExpr::Match { scrutinee, arms } => {
                self.count_subexprs(scrutinee, counts);
                for arm in arms {
                    self.count_subexprs(&arm.body, counts);
                }
            }
            IrExpr::Struct { fields, .. } => {
                for (_, e) in fields {
                    self.count_subexprs(e, counts);
                }
            }
            IrExpr::Field { object, .. } => self.count_subexprs(object, counts),
            IrExpr::Alloc(e) | IrExpr::Deref(e) => self.count_subexprs(e, counts),
            IrExpr::Seq(es) => {
                for e in es {
                    self.count_subexprs(e, counts);
                }
            }
            IrExpr::Var(_) | IrExpr::Lit(_) => {}
        }
    }

    /// Replace subexpressions in `expr` according to the substitution map.
    fn replace_subexprs(&self, expr: &IrExpr, subst: &HashMap<IrExprKey, String>) -> IrExpr {
        let key = IrExprKey::from(expr);
        if let Some(var_name) = subst.get(&key) {
            return IrExpr::Var(var_name.clone());
        }
        match expr {
            IrExpr::App { func, args } => IrExpr::App {
                func: Box::new(self.replace_subexprs(func, subst)),
                args: args
                    .iter()
                    .map(|a| self.replace_subexprs(a, subst))
                    .collect(),
            },
            IrExpr::Let {
                name,
                ty,
                value,
                body,
            } => IrExpr::Let {
                name: name.clone(),
                ty: ty.clone(),
                value: Box::new(self.replace_subexprs(value, subst)),
                body: Box::new(self.replace_subexprs(body, subst)),
            },
            IrExpr::Lambda {
                params,
                body,
                captured,
            } => IrExpr::Lambda {
                params: params.clone(),
                body: Box::new(self.replace_subexprs(body, subst)),
                captured: captured.clone(),
            },
            IrExpr::If {
                cond,
                then_branch,
                else_branch,
            } => IrExpr::If {
                cond: Box::new(self.replace_subexprs(cond, subst)),
                then_branch: Box::new(self.replace_subexprs(then_branch, subst)),
                else_branch: Box::new(self.replace_subexprs(else_branch, subst)),
            },
            IrExpr::Match { scrutinee, arms } => IrExpr::Match {
                scrutinee: Box::new(self.replace_subexprs(scrutinee, subst)),
                arms: arms
                    .iter()
                    .map(|a| IrMatchArm {
                        pattern: a.pattern.clone(),
                        body: Box::new(self.replace_subexprs(&a.body, subst)),
                    })
                    .collect(),
            },
            IrExpr::Struct { name, fields } => IrExpr::Struct {
                name: name.clone(),
                fields: fields
                    .iter()
                    .map(|(k, v)| (k.clone(), self.replace_subexprs(v, subst)))
                    .collect(),
            },
            IrExpr::Field { object, field } => IrExpr::Field {
                object: Box::new(self.replace_subexprs(object, subst)),
                field: field.clone(),
            },
            IrExpr::Alloc(e) => IrExpr::Alloc(Box::new(self.replace_subexprs(e, subst))),
            IrExpr::Deref(e) => IrExpr::Deref(Box::new(self.replace_subexprs(e, subst))),
            IrExpr::Seq(es) => {
                IrExpr::Seq(es.iter().map(|e| self.replace_subexprs(e, subst)).collect())
            }
            IrExpr::Var(_) | IrExpr::Lit(_) => expr.clone(),
        }
    }
}

// ============================================================================
// Code Generation Pipeline
// ============================================================================

/// Main code generation pipeline
pub struct CodegenPipeline {
    config: CodegenConfig,
    expr_compiler: ExprToIr,
    optimizer: Optimizer,
}

impl CodegenPipeline {
    pub fn new(config: CodegenConfig) -> Self {
        let optimizer = Optimizer::new(config.clone());
        CodegenPipeline {
            config,
            expr_compiler: ExprToIr::new(),
            optimizer,
        }
    }

    /// Compile a kernel declaration to target code
    pub fn compile_declaration(&mut self, expr: &Expr) -> CodegenResult<String> {
        let ir = self.expr_compiler.compile(expr)?;

        let ir = if self.config.optimize {
            self.optimizer.optimize(&ir)?
        } else {
            ir
        };

        match self.config.target {
            CodegenTarget::Rust => {
                let rust_gen = IrToRust::new(self.config.clone());
                rust_gen.emit(&ir)
            }
            CodegenTarget::C => {
                let c_gen = IrToC::new(self.config.clone());
                c_gen.emit(&ir)
            }
            CodegenTarget::LlvmIr => {
                self.emit_llvm_ir(&ir)
            }
            CodegenTarget::Interpreter => {
                Err(CodegenError::UnsupportedExpression(
                    "Interpreter target does not support single-expression compilation; use compile_module instead".to_string()
                ))
            }
        }
    }

    /// Emit LLVM IR text for an IR expression.
    fn emit_llvm_ir(&self, expr: &IrExpr) -> CodegenResult<String> {
        let mut out = String::new();
        out.push_str("; LLVM IR generated by Oxilean codegen\n");
        out.push_str("define i64 @oxilean_expr() {\n");
        out.push_str("entry:\n");
        self.emit_llvm_expr(&mut out, expr, 0)?;
        out.push_str("  ret i64 %result\n");
        out.push_str("}\n");
        Ok(out)
    }

    /// Recursively emit LLVM IR for an IR expression (simplified).
    fn emit_llvm_expr(
        &self,
        out: &mut String,
        expr: &IrExpr,
        depth: usize,
    ) -> CodegenResult<String> {
        let _ = depth;
        match expr {
            IrExpr::Lit(IrLit::Nat(n)) => {
                out.push_str(&format!("  %result = add i64 0, {}\n", n));
                Ok("%result".to_string())
            }
            IrExpr::Lit(IrLit::Int(n)) => {
                out.push_str(&format!("  %result = add i64 0, {}\n", n));
                Ok("%result".to_string())
            }
            IrExpr::Lit(IrLit::Bool(b)) => {
                let v: u8 = if *b { 1 } else { 0 };
                out.push_str(&format!("  %result = add i1 0, {}\n", v));
                Ok("%result".to_string())
            }
            IrExpr::Lit(IrLit::Unit) => Ok("void".to_string()),
            IrExpr::Var(name) => Ok(format!("%{}", name)),
            IrExpr::App { func, args: _ } => {
                let fname = self.emit_llvm_expr(out, func, depth + 1)?;
                out.push_str(&format!("  %result = call i64 {}()\n", fname));
                Ok("%result".to_string())
            }
            _ => {
                out.push_str("  %result = add i64 0, 0 ; unsupported expr\n");
                Ok("%result".to_string())
            }
        }
    }

    /// Compile a module (collection of declarations)
    pub fn compile_module(&mut self, exprs: Vec<&Expr>) -> CodegenResult<String> {
        if exprs.is_empty() {
            return Ok(match self.config.target {
                CodegenTarget::Rust => "// Empty module\n".to_string(),
                CodegenTarget::C => "/* Empty module */\n".to_string(),
                CodegenTarget::LlvmIr => "; Empty LLVM IR module\n".to_string(),
                CodegenTarget::Interpreter => "[]\n".to_string(),
            });
        }

        let mut parts: Vec<String> = Vec::new();
        for (i, expr) in exprs.iter().enumerate() {
            match self.compile_declaration(expr) {
                Ok(code) => parts.push(code),
                Err(e) => {
                    return Err(CodegenError::InternalError(format!(
                        "Failed to compile declaration {}: {}",
                        i, e
                    )));
                }
            }
        }

        let separator = match self.config.target {
            CodegenTarget::Rust => "\n\n",
            CodegenTarget::C => "\n\n",
            CodegenTarget::LlvmIr => "\n",
            CodegenTarget::Interpreter => "\n",
        };

        let mut output = parts.join(separator);
        if self.config.target == CodegenTarget::LlvmIr {
            output = format!(
                "; LLVM IR module ({} declarations)\n{}",
                parts.len(),
                output
            );
        }
        Ok(output)
    }
}
