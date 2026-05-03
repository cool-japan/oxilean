//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::expr::{Expr, Literal};
use std::collections::{HashMap, HashSet, VecDeque};

use super::functions::CodegenResult;

/// Compiles kernel expressions to intermediate representation
pub struct ExprToIr {
    symbol_manager: SymbolManager,
    pub(super) closure_vars: HashMap<String, Vec<String>>,
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
/// Match arm: pattern and body
#[derive(Debug, Clone, PartialEq)]
pub struct IrMatchArm {
    pub pattern: IrPattern,
    pub body: Box<IrExpr>,
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
/// Pattern for match expressions
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IrPattern {
    Wildcard,
    Literal(String),
    Constructor { name: String, args: Vec<String> },
    Tuple(Vec<String>),
    Or(Box<IrPattern>, Box<IrPattern>),
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
    pub(super) fn emit_type(&self, ty: &IrType) -> CodegenResult<String> {
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
/// A wrapper around a canonical string representation of an IrExpr used as a
/// HashMap key in the CSE pass.  The second field holds the original expression
/// so it can be reconstructed when building let-bindings.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct IrExprKey(String, Box<IrExprOwned>);
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
impl IrExprKey {
    /// Reconstruct the original IrExpr.  Since IrExprKey is only used for non-trivial
    /// sub-expressions in the CSE pass, and `replace_subexprs` is called before
    /// building let-bindings, we just need the key's representative expression
    /// which is stored directly in the Optimizer's CSE pass via the original expr.
    fn as_ir_expr(&self) -> IrExpr {
        IrExpr::Var(self.0.clone())
    }
}
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
            IrType::Pointer(inner) => format!("{}*", self.emit_c_type(inner)?),
            _ => return Err(CodegenError::UnsupportedType(ty.to_string())),
        })
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum LibPassPhase {
    Analysis,
    Transformation,
    Verification,
    Cleanup,
}
impl LibPassPhase {
    #[allow(dead_code)]
    pub fn name(&self) -> &str {
        match self {
            LibPassPhase::Analysis => "analysis",
            LibPassPhase::Transformation => "transformation",
            LibPassPhase::Verification => "verification",
            LibPassPhase::Cleanup => "cleanup",
        }
    }
    #[allow(dead_code)]
    pub fn is_modifying(&self) -> bool {
        matches!(self, LibPassPhase::Transformation | LibPassPhase::Cleanup)
    }
}
/// Helper struct for emitting Rust code
pub struct RustEmitter {
    output: String,
    pub(super) indent_level: usize,
}
impl RustEmitter {
    pub(super) fn new() -> Self {
        RustEmitter {
            output: String::new(),
            indent_level: 0,
        }
    }
    pub(super) fn indent(&mut self) {
        self.indent_level += 1;
    }
    pub(super) fn dedent(&mut self) {
        if self.indent_level > 0 {
            self.indent_level -= 1;
        }
    }
    pub(super) fn emit(&mut self, line: &str) {
        for _ in 0..self.indent_level {
            self.output.push_str("    ");
        }
        self.output.push_str(line);
        self.output.push('\n');
    }
    pub(super) fn emit_inline(&mut self, text: &str) {
        self.output.push_str(text);
    }
    pub(super) fn result(&self) -> String {
        self.output.clone()
    }
}
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
    pub(super) fn constant_fold(&self, expr: &IrExpr) -> CodegenResult<IrExpr> {
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
    pub(super) fn dead_code_eliminate(&self, expr: &IrExpr) -> CodegenResult<IrExpr> {
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
    pub(super) fn is_var_used(&self, name: &str, expr: &IrExpr) -> bool {
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
    pub(super) fn inline(&self, expr: &IrExpr) -> CodegenResult<IrExpr> {
        match expr {
            IrExpr::App { func, args } => {
                let inlined_func = self.inline(func)?;
                let inlined_args: CodegenResult<Vec<IrExpr>> =
                    args.iter().map(|a| self.inline(a)).collect();
                let inlined_args = inlined_args?;
                if let IrExpr::Lambda { params, body, .. } = &inlined_func {
                    let arg_size = self.expr_size(&IrExpr::App {
                        func: Box::new(inlined_func.clone()),
                        args: inlined_args.clone(),
                    });
                    if arg_size <= self.config.inline_threshold
                        && params.len() == inlined_args.len()
                    {
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
    pub(super) fn common_subexpr_eliminate(&self, expr: &IrExpr) -> CodegenResult<IrExpr> {
        let mut counts: HashMap<IrExprKey, usize> = HashMap::new();
        self.count_subexprs(expr, &mut counts);
        let mut to_hoist: Vec<IrExprKey> = counts
            .iter()
            .filter(|(key, &count)| count > 1 && !key.is_trivial())
            .map(|(key, _)| key.clone())
            .collect();
        to_hoist.sort_by_key(|k| k.0.clone());
        if to_hoist.is_empty() {
            return Ok(expr.clone());
        }
        let mut subst: HashMap<IrExprKey, String> = HashMap::new();
        for (counter, key) in to_hoist.iter().enumerate() {
            let name = format!("_cse{}", counter);
            subst.insert(key.clone(), name);
        }
        let rewritten = self.replace_subexprs(expr, &subst);
        let mut result = rewritten;
        for key in to_hoist.iter().rev() {
            let var_name = subst[key].clone();
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
/// Symbol manager for name mangling and closure conversion
pub struct SymbolManager {
    counter: usize,
    scopes: VecDeque<HashSet<String>>,
}
impl SymbolManager {
    pub(super) fn new() -> Self {
        SymbolManager {
            counter: 0,
            scopes: VecDeque::new(),
        }
    }
    pub(super) fn fresh_name(&mut self, base: &str) -> String {
        let name = format!("{}_{}", base, self.counter);
        self.counter += 1;
        name
    }
    pub(super) fn push_scope(&mut self) {
        self.scopes.push_back(HashSet::new());
    }
    pub(super) fn pop_scope(&mut self) {
        self.scopes.pop_back();
    }
    pub(super) fn bind(&mut self, name: String) {
        if let Some(scope) = self.scopes.back_mut() {
            scope.insert(name);
        }
    }
    pub(super) fn is_bound(&self, name: &str) -> bool {
        self.scopes.iter().any(|scope| scope.contains(name))
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LibPassConfig {
    pub phase: LibPassPhase,
    pub enabled: bool,
    pub max_iterations: u32,
    pub debug_output: bool,
    pub pass_name: String,
}
impl LibPassConfig {
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>, phase: LibPassPhase) -> Self {
        LibPassConfig {
            phase,
            enabled: true,
            max_iterations: 10,
            debug_output: false,
            pass_name: name.into(),
        }
    }
    #[allow(dead_code)]
    pub fn disabled(mut self) -> Self {
        self.enabled = false;
        self
    }
    #[allow(dead_code)]
    pub fn with_debug(mut self) -> Self {
        self.debug_output = true;
        self
    }
    #[allow(dead_code)]
    pub fn max_iter(mut self, n: u32) -> Self {
        self.max_iterations = n;
        self
    }
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
/// Code generation configuration options
#[derive(Debug, Clone)]
pub struct CodegenConfig {
    pub target: CodegenTarget,
    pub optimize: bool,
    pub debug_info: bool,
    pub emit_comments: bool,
    pub inline_threshold: usize,
}
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
/// Target code generation language
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CodegenTarget {
    Rust,
    C,
    LlvmIr,
    Interpreter,
}
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
            CodegenTarget::LlvmIr => self.emit_llvm_ir(&ir),
            CodegenTarget::Interpreter => {
                Err(
                    CodegenError::UnsupportedExpression(
                        "Interpreter target does not support single-expression compilation; use compile_module instead"
                            .to_string(),
                    ),
                )
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
#[allow(dead_code)]
pub struct LibPassRegistry {
    configs: Vec<LibPassConfig>,
    stats: std::collections::HashMap<String, LibPassStats>,
}
impl LibPassRegistry {
    #[allow(dead_code)]
    pub fn new() -> Self {
        LibPassRegistry {
            configs: Vec::new(),
            stats: std::collections::HashMap::new(),
        }
    }
    #[allow(dead_code)]
    pub fn register(&mut self, config: LibPassConfig) {
        self.stats
            .insert(config.pass_name.clone(), LibPassStats::new());
        self.configs.push(config);
    }
    #[allow(dead_code)]
    pub fn enabled_passes(&self) -> Vec<&LibPassConfig> {
        self.configs.iter().filter(|c| c.enabled).collect()
    }
    #[allow(dead_code)]
    pub fn get_stats(&self, name: &str) -> Option<&LibPassStats> {
        self.stats.get(name)
    }
    #[allow(dead_code)]
    pub fn total_passes(&self) -> usize {
        self.configs.len()
    }
    #[allow(dead_code)]
    pub fn enabled_count(&self) -> usize {
        self.enabled_passes().len()
    }
    #[allow(dead_code)]
    pub fn update_stats(&mut self, name: &str, changes: u64, time_ms: u64, iter: u32) {
        if let Some(stats) = self.stats.get_mut(name) {
            stats.record_run(changes, time_ms, iter);
        }
    }
}
/// Owned, hashable copy of an IrExpr (mirrors IrExpr but derives Hash/Eq).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum IrExprOwned {
    Var(String),
    Lit(String),
    App(Box<IrExprOwned>, Vec<IrExprOwned>),
    Other(String),
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LibWorklist {
    items: std::collections::VecDeque<u32>,
    in_worklist: std::collections::HashSet<u32>,
}
impl LibWorklist {
    #[allow(dead_code)]
    pub fn new() -> Self {
        LibWorklist {
            items: std::collections::VecDeque::new(),
            in_worklist: std::collections::HashSet::new(),
        }
    }
    #[allow(dead_code)]
    pub fn push(&mut self, item: u32) -> bool {
        if self.in_worklist.insert(item) {
            self.items.push_back(item);
            true
        } else {
            false
        }
    }
    #[allow(dead_code)]
    pub fn pop(&mut self) -> Option<u32> {
        let item = self.items.pop_front()?;
        self.in_worklist.remove(&item);
        Some(item)
    }
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.items.len()
    }
    #[allow(dead_code)]
    pub fn contains(&self, item: u32) -> bool {
        self.in_worklist.contains(&item)
    }
}
#[allow(dead_code)]
pub struct LibConstantFoldingHelper;
impl LibConstantFoldingHelper {
    #[allow(dead_code)]
    pub fn fold_add_i64(a: i64, b: i64) -> Option<i64> {
        a.checked_add(b)
    }
    #[allow(dead_code)]
    pub fn fold_sub_i64(a: i64, b: i64) -> Option<i64> {
        a.checked_sub(b)
    }
    #[allow(dead_code)]
    pub fn fold_mul_i64(a: i64, b: i64) -> Option<i64> {
        a.checked_mul(b)
    }
    #[allow(dead_code)]
    pub fn fold_div_i64(a: i64, b: i64) -> Option<i64> {
        if b == 0 {
            None
        } else {
            a.checked_div(b)
        }
    }
    #[allow(dead_code)]
    pub fn fold_add_f64(a: f64, b: f64) -> f64 {
        a + b
    }
    #[allow(dead_code)]
    pub fn fold_mul_f64(a: f64, b: f64) -> f64 {
        a * b
    }
    #[allow(dead_code)]
    pub fn fold_neg_i64(a: i64) -> Option<i64> {
        a.checked_neg()
    }
    #[allow(dead_code)]
    pub fn fold_not_bool(a: bool) -> bool {
        !a
    }
    #[allow(dead_code)]
    pub fn fold_and_bool(a: bool, b: bool) -> bool {
        a && b
    }
    #[allow(dead_code)]
    pub fn fold_or_bool(a: bool, b: bool) -> bool {
        a || b
    }
    #[allow(dead_code)]
    pub fn fold_shl_i64(a: i64, b: u32) -> Option<i64> {
        a.checked_shl(b)
    }
    #[allow(dead_code)]
    pub fn fold_shr_i64(a: i64, b: u32) -> Option<i64> {
        a.checked_shr(b)
    }
    #[allow(dead_code)]
    pub fn fold_rem_i64(a: i64, b: i64) -> Option<i64> {
        if b == 0 {
            None
        } else {
            Some(a % b)
        }
    }
    #[allow(dead_code)]
    pub fn fold_bitand_i64(a: i64, b: i64) -> i64 {
        a & b
    }
    #[allow(dead_code)]
    pub fn fold_bitor_i64(a: i64, b: i64) -> i64 {
        a | b
    }
    #[allow(dead_code)]
    pub fn fold_bitxor_i64(a: i64, b: i64) -> i64 {
        a ^ b
    }
    #[allow(dead_code)]
    pub fn fold_bitnot_i64(a: i64) -> i64 {
        !a
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LibDominatorTree {
    pub idom: Vec<Option<u32>>,
    pub dom_children: Vec<Vec<u32>>,
    pub dom_depth: Vec<u32>,
}
impl LibDominatorTree {
    #[allow(dead_code)]
    pub fn new(size: usize) -> Self {
        LibDominatorTree {
            idom: vec![None; size],
            dom_children: vec![Vec::new(); size],
            dom_depth: vec![0; size],
        }
    }
    #[allow(dead_code)]
    pub fn set_idom(&mut self, node: usize, idom: u32) {
        self.idom[node] = Some(idom);
    }
    #[allow(dead_code)]
    pub fn dominates(&self, a: usize, b: usize) -> bool {
        if a == b {
            return true;
        }
        let mut cur = b;
        loop {
            match self.idom[cur] {
                Some(parent) if parent as usize == a => return true,
                Some(parent) if parent as usize == cur => return false,
                Some(parent) => cur = parent as usize,
                None => return false,
            }
        }
    }
    #[allow(dead_code)]
    pub fn depth(&self, node: usize) -> u32 {
        self.dom_depth.get(node).copied().unwrap_or(0)
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LibCacheEntry {
    pub key: String,
    pub data: Vec<u8>,
    pub timestamp: u64,
    pub valid: bool,
}
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct LibPassStats {
    pub total_runs: u32,
    pub successful_runs: u32,
    pub total_changes: u64,
    pub time_ms: u64,
    pub iterations_used: u32,
}
impl LibPassStats {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    #[allow(dead_code)]
    pub fn record_run(&mut self, changes: u64, time_ms: u64, iterations: u32) {
        self.total_runs += 1;
        self.successful_runs += 1;
        self.total_changes += changes;
        self.time_ms += time_ms;
        self.iterations_used = iterations;
    }
    #[allow(dead_code)]
    pub fn average_changes_per_run(&self) -> f64 {
        if self.total_runs == 0 {
            return 0.0;
        }
        self.total_changes as f64 / self.total_runs as f64
    }
    #[allow(dead_code)]
    pub fn success_rate(&self) -> f64 {
        if self.total_runs == 0 {
            return 0.0;
        }
        self.successful_runs as f64 / self.total_runs as f64
    }
    #[allow(dead_code)]
    pub fn format_summary(&self) -> String {
        format!(
            "Runs: {}/{}, Changes: {}, Time: {}ms",
            self.successful_runs, self.total_runs, self.total_changes, self.time_ms
        )
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LibAnalysisCache {
    pub(super) entries: std::collections::HashMap<String, LibCacheEntry>,
    max_size: usize,
    hits: u64,
    misses: u64,
}
impl LibAnalysisCache {
    #[allow(dead_code)]
    pub fn new(max_size: usize) -> Self {
        LibAnalysisCache {
            entries: std::collections::HashMap::new(),
            max_size,
            hits: 0,
            misses: 0,
        }
    }
    #[allow(dead_code)]
    pub fn get(&mut self, key: &str) -> Option<&LibCacheEntry> {
        if self.entries.contains_key(key) {
            self.hits += 1;
            self.entries.get(key)
        } else {
            self.misses += 1;
            None
        }
    }
    #[allow(dead_code)]
    pub fn insert(&mut self, key: String, data: Vec<u8>) {
        if self.entries.len() >= self.max_size {
            if let Some(oldest) = self.entries.keys().next().cloned() {
                self.entries.remove(&oldest);
            }
        }
        self.entries.insert(
            key.clone(),
            LibCacheEntry {
                key,
                data,
                timestamp: 0,
                valid: true,
            },
        );
    }
    #[allow(dead_code)]
    pub fn invalidate(&mut self, key: &str) {
        if let Some(entry) = self.entries.get_mut(key) {
            entry.valid = false;
        }
    }
    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.entries.clear();
    }
    #[allow(dead_code)]
    pub fn hit_rate(&self) -> f64 {
        let total = self.hits + self.misses;
        if total == 0 {
            return 0.0;
        }
        self.hits as f64 / total as f64
    }
    #[allow(dead_code)]
    pub fn size(&self) -> usize {
        self.entries.len()
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LibLivenessInfo {
    pub live_in: Vec<std::collections::HashSet<u32>>,
    pub live_out: Vec<std::collections::HashSet<u32>>,
    pub defs: Vec<std::collections::HashSet<u32>>,
    pub uses: Vec<std::collections::HashSet<u32>>,
}
impl LibLivenessInfo {
    #[allow(dead_code)]
    pub fn new(block_count: usize) -> Self {
        LibLivenessInfo {
            live_in: vec![std::collections::HashSet::new(); block_count],
            live_out: vec![std::collections::HashSet::new(); block_count],
            defs: vec![std::collections::HashSet::new(); block_count],
            uses: vec![std::collections::HashSet::new(); block_count],
        }
    }
    #[allow(dead_code)]
    pub fn add_def(&mut self, block: usize, var: u32) {
        if block < self.defs.len() {
            self.defs[block].insert(var);
        }
    }
    #[allow(dead_code)]
    pub fn add_use(&mut self, block: usize, var: u32) {
        if block < self.uses.len() {
            self.uses[block].insert(var);
        }
    }
    #[allow(dead_code)]
    pub fn is_live_in(&self, block: usize, var: u32) -> bool {
        self.live_in
            .get(block)
            .map(|s| s.contains(&var))
            .unwrap_or(false)
    }
    #[allow(dead_code)]
    pub fn is_live_out(&self, block: usize, var: u32) -> bool {
        self.live_out
            .get(block)
            .map(|s| s.contains(&var))
            .unwrap_or(false)
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LibDepGraph {
    nodes: Vec<u32>,
    edges: Vec<(u32, u32)>,
}
impl LibDepGraph {
    #[allow(dead_code)]
    pub fn new() -> Self {
        LibDepGraph {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }
    #[allow(dead_code)]
    pub fn add_node(&mut self, id: u32) {
        if !self.nodes.contains(&id) {
            self.nodes.push(id);
        }
    }
    #[allow(dead_code)]
    pub fn add_dep(&mut self, dep: u32, dependent: u32) {
        self.add_node(dep);
        self.add_node(dependent);
        self.edges.push((dep, dependent));
    }
    #[allow(dead_code)]
    pub fn dependents_of(&self, node: u32) -> Vec<u32> {
        self.edges
            .iter()
            .filter(|(d, _)| *d == node)
            .map(|(_, dep)| *dep)
            .collect()
    }
    #[allow(dead_code)]
    pub fn dependencies_of(&self, node: u32) -> Vec<u32> {
        self.edges
            .iter()
            .filter(|(_, dep)| *dep == node)
            .map(|(d, _)| *d)
            .collect()
    }
    #[allow(dead_code)]
    pub fn topological_sort(&self) -> Vec<u32> {
        let mut in_degree: std::collections::HashMap<u32, u32> = std::collections::HashMap::new();
        for &n in &self.nodes {
            in_degree.insert(n, 0);
        }
        for (_, dep) in &self.edges {
            *in_degree.entry(*dep).or_insert(0) += 1;
        }
        let mut queue: std::collections::VecDeque<u32> = self
            .nodes
            .iter()
            .filter(|&&n| in_degree[&n] == 0)
            .copied()
            .collect();
        let mut result = Vec::new();
        while let Some(node) = queue.pop_front() {
            result.push(node);
            for dep in self.dependents_of(node) {
                let cnt = in_degree.entry(dep).or_insert(0);
                *cnt = cnt.saturating_sub(1);
                if *cnt == 0 {
                    queue.push_back(dep);
                }
            }
        }
        result
    }
    #[allow(dead_code)]
    pub fn has_cycle(&self) -> bool {
        self.topological_sort().len() < self.nodes.len()
    }
}
