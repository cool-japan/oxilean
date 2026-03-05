//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;

use std::collections::{BTreeSet, HashMap};

/// Tracks which de Bruijn indices are live at a given program point.
#[allow(dead_code)]
pub struct ErasedLiveness {
    live: std::collections::BTreeSet<u32>,
}
#[allow(dead_code)]
impl ErasedLiveness {
    /// Creates a new empty liveness set.
    pub fn new() -> Self {
        Self {
            live: std::collections::BTreeSet::new(),
        }
    }
    /// Marks de Bruijn index `i` as live.
    pub fn mark_live(&mut self, i: u32) {
        self.live.insert(i);
    }
    /// Returns `true` if de Bruijn index `i` is live.
    pub fn is_live(&self, i: u32) -> bool {
        self.live.contains(&i)
    }
    /// Returns the number of live indices.
    pub fn count(&self) -> usize {
        self.live.len()
    }
    /// Merges another liveness set into this one (union).
    pub fn merge(&mut self, other: &ErasedLiveness) {
        for &i in &other.live {
            self.live.insert(i);
        }
    }
    /// Returns the maximum live index, or `None` if empty.
    pub fn max_live(&self) -> Option<u32> {
        self.live.iter().next_back().copied()
    }
}
/// High-level AST node wrapping an erased expression with source metadata.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct ErasedAst {
    /// The underlying erased expression.
    pub expr: ErasedExpr,
    /// A human-readable label for debugging.
    pub label: String,
    /// Estimated stack depth needed to evaluate this node.
    pub stack_depth: usize,
}
#[allow(dead_code)]
impl ErasedAst {
    /// Creates a new AST node.
    pub fn new(expr: ErasedExpr, label: impl Into<String>) -> Self {
        let depth = erased_expr_depth(&expr);
        Self {
            expr,
            label: label.into(),
            stack_depth: depth as usize,
        }
    }
    /// Returns the size of the wrapped expression.
    pub fn size(&self) -> usize {
        erased_expr_apps(&self.expr)
    }
}
/// A pass pipeline combining multiple transformation passes.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct ErasurePass {
    pub name: String,
    pub total_transforms: u64,
}
#[allow(dead_code)]
impl ErasurePass {
    /// Create a new pipeline.
    pub fn new(name: impl Into<String>) -> Self {
        ErasurePass {
            name: name.into(),
            total_transforms: 0,
        }
    }
    /// Run the full erasure pipeline on an expression.
    pub fn run(&mut self, expr: ErasedExprExt) -> ErasedExprExt {
        let mut opt = ErasedOptimizer::new(1000);
        let result = opt.optimize(expr);
        self.total_transforms += opt.total_transforms();
        result
    }
    /// Run the pipeline on a module.
    pub fn run_module(&mut self, mut module: ErasedModule) -> ErasedModule {
        let new_decls = module
            .decls
            .drain(..)
            .map(|decl| match decl {
                ErasedDecl::Def { name, body } => {
                    let new_body = self.run(body);
                    ErasedDecl::Def {
                        name,
                        body: new_body,
                    }
                }
                other => other,
            })
            .collect();
        module.decls = new_decls;
        module
    }
}
/// Tuple constructor and projector for erased expressions.
#[allow(dead_code)]
pub struct ErasedTupleOps;
#[allow(dead_code)]
impl ErasedTupleOps {
    /// Construct a pair (2-tuple).
    pub fn make_pair(a: ErasedExprExt, b: ErasedExprExt) -> ErasedExprExt {
        ErasedExprExt::App(
            Box::new(ErasedExprExt::App(
                Box::new(ErasedExprExt::Const("Prod.mk".to_string())),
                Box::new(a),
            )),
            Box::new(b),
        )
    }
    /// First projection (fst) application.
    pub fn fst(pair: ErasedExprExt) -> ErasedExprExt {
        ErasedExprExt::App(
            Box::new(ErasedExprExt::Const("Prod.fst".to_string())),
            Box::new(pair),
        )
    }
    /// Second projection (snd) application.
    pub fn snd(pair: ErasedExprExt) -> ErasedExprExt {
        ErasedExprExt::App(
            Box::new(ErasedExprExt::Const("Prod.snd".to_string())),
            Box::new(pair),
        )
    }
    /// Construct an n-tuple by nesting pairs.
    pub fn n_tuple(exprs: Vec<ErasedExprExt>) -> ErasedExprExt {
        assert!(!exprs.is_empty(), "n_tuple: empty tuple");
        let mut iter = exprs.into_iter().rev();
        let last = iter
            .next()
            .expect("iterator must have at least one element");
        iter.fold(last, |acc, e| Self::make_pair(e, acc))
    }
}
/// Performs beta reduction on erased expressions.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct ErasedBetaReducer {
    pub steps: u64,
    pub max_steps: u64,
}
#[allow(dead_code)]
impl ErasedBetaReducer {
    /// Create a reducer with a step limit.
    pub fn new(max_steps: u64) -> Self {
        ErasedBetaReducer {
            steps: 0,
            max_steps,
        }
    }
    /// Perform one step of beta reduction.
    pub fn step(&mut self, expr: ErasedExprExt) -> ErasedExprExt {
        if self.steps >= self.max_steps {
            return expr;
        }
        match expr {
            ErasedExprExt::App(f, arg) => match *f {
                ErasedExprExt::Lam(body) => {
                    self.steps += 1;
                    subst_bvar0(*body, *arg)
                }
                other_f => ErasedExprExt::App(Box::new(other_f), arg),
            },
            ErasedExprExt::Lam(b) => ErasedExprExt::Lam(Box::new(self.step(*b))),
            other => other,
        }
    }
    /// Return whether the step limit has been reached.
    pub fn is_exhausted(&self) -> bool {
        self.steps >= self.max_steps
    }
}
/// A map from expression ids to their erased forms.
#[allow(dead_code)]
pub struct ErasedTypeMap {
    entries: Vec<(u64, ErasedExprExt)>,
}
#[allow(dead_code)]
impl ErasedTypeMap {
    /// Create an empty map.
    pub fn new() -> Self {
        ErasedTypeMap {
            entries: Vec::new(),
        }
    }
    /// Insert a mapping.
    pub fn insert(&mut self, id: u64, erased: ErasedExprExt) {
        if let Some(e) = self.entries.iter_mut().find(|(i, _)| *i == id) {
            e.1 = erased;
        } else {
            self.entries.push((id, erased));
        }
    }
    /// Look up by id.
    pub fn get(&self, id: u64) -> Option<&ErasedExprExt> {
        self.entries.iter().find(|(i, _)| *i == id).map(|(_, e)| e)
    }
    /// Return the number of entries.
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    /// Return whether empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}
/// A single arm of an erased match expression.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Clone)]
pub struct ErasedMatchArm {
    pub pattern: ErasedPattern,
    pub body: ErasedExprExt,
}
#[allow(dead_code)]
impl ErasedMatchArm {
    /// Create a new match arm.
    pub fn new(pattern: ErasedPattern, body: ErasedExprExt) -> Self {
        ErasedMatchArm { pattern, body }
    }
    /// Return whether this arm is a catch-all.
    pub fn is_catch_all(&self) -> bool {
        self.pattern.is_irrefutable()
    }
}
/// Represents a captured environment for a closure in erased code.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct ErasedClosureEnv {
    /// Captured variable names.
    pub captures: Vec<String>,
    /// Their corresponding erased values.
    pub values: Vec<ErasedExprExt>,
}
#[allow(dead_code)]
impl ErasedClosureEnv {
    /// Creates a new empty closure environment.
    pub fn new() -> Self {
        Self {
            captures: Vec::new(),
            values: Vec::new(),
        }
    }
    /// Adds a captured variable binding.
    pub fn capture(&mut self, name: impl Into<String>, val: ErasedExprExt) {
        self.captures.push(name.into());
        self.values.push(val);
    }
    /// Look up a captured variable by name.
    pub fn lookup(&self, name: &str) -> Option<&ErasedExprExt> {
        self.captures
            .iter()
            .position(|n| n == name)
            .map(|i| &self.values[i])
    }
    /// Returns the number of captured variables.
    pub fn size(&self) -> usize {
        self.captures.len()
    }
}
/// Runs a sequence of `ErasedPass` implementations in order.
#[allow(dead_code)]
pub struct PipelineRunner {
    passes: Vec<Box<dyn ErasedPass>>,
}
#[allow(dead_code)]
impl PipelineRunner {
    /// Creates a new empty pipeline runner.
    pub fn new() -> Self {
        Self { passes: Vec::new() }
    }
    /// Adds a pass to the pipeline.
    pub fn add_pass(&mut self, pass: Box<dyn ErasedPass>) {
        self.passes.push(pass);
    }
    /// Runs all passes on `expr` in order.
    pub fn run_all(&mut self, expr: ErasedExpr) -> ErasedExpr {
        let mut current = expr;
        for pass in self.passes.iter_mut() {
            current = pass.run(current);
        }
        current
    }
    /// Runs all passes on each declaration in `module`.
    pub fn run_on_module(&mut self, decls: &mut Vec<ErasedDecl>) {
        for pass in self.passes.iter_mut() {
            pass.run_on_module(decls);
        }
    }
    /// Returns the number of registered passes.
    pub fn num_passes(&self) -> usize {
        self.passes.len()
    }
}
/// Inline simple constants in an erased expression.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct ErasedInliner {
    /// Map from constant name to its erased definition.
    defs: Vec<(String, ErasedExprExt)>,
    pub inlined: u64,
}
#[allow(dead_code)]
impl ErasedInliner {
    /// Create an inliner with definitions.
    pub fn new() -> Self {
        ErasedInliner {
            defs: Vec::new(),
            inlined: 0,
        }
    }
    /// Register a constant definition.
    pub fn register(&mut self, name: &str, def: ErasedExprExt) {
        self.defs.push((name.to_string(), def));
    }
    /// Inline constants in an expression.
    pub fn inline(&mut self, expr: ErasedExprExt) -> ErasedExprExt {
        match expr {
            ErasedExprExt::Const(ref name) => {
                if let Some((_, def)) = self.defs.iter().find(|(n, _)| n == name) {
                    self.inlined += 1;
                    def.clone()
                } else {
                    expr
                }
            }
            ErasedExprExt::Lam(b) => ErasedExprExt::Lam(Box::new(self.inline(*b))),
            ErasedExprExt::App(f, x) => {
                ErasedExprExt::App(Box::new(self.inline(*f)), Box::new(self.inline(*x)))
            }
            ErasedExprExt::Let(v, b) => {
                ErasedExprExt::Let(Box::new(self.inline(*v)), Box::new(self.inline(*b)))
            }
            other => other,
        }
    }
}
/// An erasure context tracking which variables are types vs terms.
#[allow(dead_code)]
pub struct ErasureContext {
    /// True if variable at index is a type (should be erased).
    type_vars: Vec<bool>,
}
#[allow(dead_code)]
impl ErasureContext {
    /// Create an empty context.
    pub fn new() -> Self {
        ErasureContext {
            type_vars: Vec::new(),
        }
    }
    /// Push a variable; `is_type` indicates whether it should be erased.
    pub fn push(&mut self, is_type: bool) {
        self.type_vars.push(is_type);
    }
    /// Pop the last variable.
    pub fn pop(&mut self) {
        self.type_vars.pop();
    }
    /// Return whether the variable at De Bruijn index `i` is a type.
    pub fn is_type_at(&self, i: u32) -> bool {
        let len = self.type_vars.len();
        if (i as usize) < len {
            self.type_vars[len - 1 - i as usize]
        } else {
            false
        }
    }
    /// Return the current depth.
    pub fn depth(&self) -> usize {
        self.type_vars.len()
    }
}
/// A simple code generator for erased expressions (stub).
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct ErasedCodegen {
    pub output: String,
    indent: usize,
}
#[allow(dead_code)]
impl ErasedCodegen {
    /// Create a new code generator.
    pub fn new() -> Self {
        ErasedCodegen {
            output: String::new(),
            indent: 0,
        }
    }
    /// Emit a line with current indentation.
    pub fn emit(&mut self, line: &str) {
        for _ in 0..self.indent {
            self.output.push_str("  ");
        }
        self.output.push_str(line);
        self.output.push('\n');
    }
    /// Generate code for an expression.
    pub fn gen_expr(&mut self, expr: &ErasedExprExt) -> String {
        match expr {
            ErasedExprExt::Lit(n) => n.to_string(),
            ErasedExprExt::BVar(i) => format!("v{}", i),
            ErasedExprExt::FVar(name) => name.clone(),
            ErasedExprExt::Const(name) => name.clone(),
            ErasedExprExt::Unit => "()".to_string(),
            ErasedExprExt::TypeErased => "_".to_string(),
            ErasedExprExt::Lam(b) => format!("(fun x -> {})", self.gen_expr(b)),
            ErasedExprExt::App(f, x) => {
                format!("({} {})", self.gen_expr(f), self.gen_expr(x))
            }
            ErasedExprExt::Let(v, b) => {
                format!("(let _ = {} in {})", self.gen_expr(v), self.gen_expr(b))
            }
            ErasedExprExt::CtorTag(t) => format!("Ctor({})", t),
        }
    }
    /// Generate code for a module.
    pub fn gen_module(&mut self, module: &ErasedModule) {
        self.emit(&format!("(* Module: {} *)", module.name));
        for decl in &module.decls {
            match decl {
                ErasedDecl::Def { name, body } => {
                    let body_str = self.gen_expr(body);
                    self.emit(&format!("let {} = {}", name, body_str));
                }
                ErasedDecl::Axiom { name } => {
                    self.emit(&format!("let {} = failwith \"axiom\"", name));
                }
                ErasedDecl::Inductive { name, ctor_count } => {
                    self.emit(&format!(
                        "(* Inductive {} with {} ctors *)",
                        name, ctor_count
                    ));
                }
            }
        }
    }
}
/// Environment for erased computation (parallel binding arrays).
#[allow(dead_code)]
pub struct ErasedEnv {
    names: Vec<String>,
    values: Vec<ErasedExpr>,
}
#[allow(dead_code)]
impl ErasedEnv {
    /// Create an empty environment.
    pub fn new() -> Self {
        ErasedEnv {
            names: Vec::new(),
            values: Vec::new(),
        }
    }
    /// Bind a name to an expression.
    pub fn bind(&mut self, name: &str, val: ErasedExpr) {
        self.names.push(name.to_string());
        self.values.push(val);
    }
    /// Look up a name (returns the most recent binding).
    pub fn get(&self, name: &str) -> Option<&ErasedExpr> {
        self.names
            .iter()
            .rev()
            .zip(self.values.iter().rev())
            .find(|(n, _)| n.as_str() == name)
            .map(|(_, v)| v)
    }
    /// Return the number of bindings.
    pub fn len(&self) -> usize {
        self.names.len()
    }
    /// Return whether the environment is empty.
    pub fn is_empty(&self) -> bool {
        self.names.is_empty()
    }
}
/// A collection of erased declarations forming a module.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct ErasedModule {
    pub name: String,
    pub decls: Vec<ErasedDecl>,
}
#[allow(dead_code)]
impl ErasedModule {
    /// Create an empty module.
    pub fn new(name: impl Into<String>) -> Self {
        ErasedModule {
            name: name.into(),
            decls: Vec::new(),
        }
    }
    /// Add a declaration.
    pub fn add(&mut self, decl: ErasedDecl) {
        self.decls.push(decl);
    }
    /// Find a declaration by name.
    pub fn find(&self, name: &str) -> Option<&ErasedDecl> {
        self.decls.iter().find(|d| d.name() == name)
    }
    /// Return the number of declarations.
    pub fn len(&self) -> usize {
        self.decls.len()
    }
    /// Return whether the module is empty.
    pub fn is_empty(&self) -> bool {
        self.decls.is_empty()
    }
    /// Return all function names.
    pub fn function_names(&self) -> Vec<&str> {
        self.decls
            .iter()
            .filter(|d| d.is_def())
            .map(|d| d.name())
            .collect()
    }
}
/// A heap-allocated object in the erased runtime.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Clone)]
pub enum ErasedHeapObj {
    /// A boxed literal value.
    Lit(u64),
    /// A constructor tag + fields.
    Ctor { tag: u32, fields: Vec<usize> },
    /// A closure.
    Closure {
        arity: u32,
        fn_ptr: usize,
        num_caps: u32,
    },
    /// A string constant.
    Str(String),
    /// A thunk (unevaluated expression).
    Thunk { code: usize },
}
impl ErasedHeapObj {
    /// Return `true` if this is a constructor object.
    #[allow(dead_code)]
    pub fn is_ctor(&self) -> bool {
        matches!(self, ErasedHeapObj::Ctor { .. })
    }
    /// Return `true` if this is a closure.
    #[allow(dead_code)]
    pub fn is_closure(&self) -> bool {
        matches!(self, ErasedHeapObj::Closure { .. })
    }
    /// Return `true` if this is a thunk.
    #[allow(dead_code)]
    pub fn is_thunk(&self) -> bool {
        matches!(self, ErasedHeapObj::Thunk { .. })
    }
    /// Return the tag if this is a constructor, else `None`.
    #[allow(dead_code)]
    pub fn ctor_tag(&self) -> Option<u32> {
        if let ErasedHeapObj::Ctor { tag, .. } = self {
            Some(*tag)
        } else {
            None
        }
    }
}
/// A simple optimizer that chains beta reduction, inlining, and DCE.
#[allow(dead_code)]
pub struct ErasedOptimizer {
    reducer: ErasedBetaReducer,
    dce: ErasedDCE,
}
#[allow(dead_code)]
impl ErasedOptimizer {
    /// Create an optimizer.
    pub fn new(max_steps: u64) -> Self {
        ErasedOptimizer {
            reducer: ErasedBetaReducer::new(max_steps),
            dce: ErasedDCE::new(),
        }
    }
    /// Run all optimization passes on an expression.
    pub fn optimize(&mut self, expr: ErasedExprExt) -> ErasedExprExt {
        let after_beta = self.reducer.step(expr);
        self.dce.elim(after_beta)
    }
    /// Return total transformations applied.
    pub fn total_transforms(&self) -> u64 {
        self.reducer.steps + self.dce.eliminated
    }
}
/// An erased expression node with only runtime-relevant information.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErasedExprExt {
    /// A bound variable (De Bruijn index).
    BVar(u32),
    /// A free variable by name.
    FVar(String),
    /// A literal integer value.
    Lit(u64),
    /// A constructor tag (for inductive types).
    CtorTag(u32),
    /// A lambda abstraction.
    Lam(Box<ErasedExprExt>),
    /// An application.
    App(Box<ErasedExprExt>, Box<ErasedExprExt>),
    /// A global constant reference.
    Const(String),
    /// A let binding.
    Let(Box<ErasedExprExt>, Box<ErasedExprExt>),
    /// A type-erased placeholder (occurs when types are erased).
    TypeErased,
    /// A unit value.
    Unit,
}
impl ErasedExprExt {
    /// Return true if this expression is a literal.
    #[allow(dead_code)]
    pub fn is_lit(&self) -> bool {
        matches!(self, ErasedExprExt::Lit(_))
    }
    /// Return true if this is a lambda.
    #[allow(dead_code)]
    pub fn is_lam(&self) -> bool {
        matches!(self, ErasedExprExt::Lam(_))
    }
    /// Return true if this is an application.
    #[allow(dead_code)]
    pub fn is_app(&self) -> bool {
        matches!(self, ErasedExprExt::App(_, _))
    }
    /// Return true if this is type-erased.
    #[allow(dead_code)]
    pub fn is_type_erased(&self) -> bool {
        *self == ErasedExprExt::TypeErased
    }
    /// Count the total number of nodes in the expression tree.
    #[allow(dead_code)]
    pub fn size(&self) -> usize {
        match self {
            ErasedExprExt::Lam(b) => 1 + b.size(),
            ErasedExprExt::App(f, x) => 1 + f.size() + x.size(),
            ErasedExprExt::Let(v, b) => 1 + v.size() + b.size(),
            _ => 1,
        }
    }
}
/// Dead-code elimination for erased expressions.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct ErasedDCE {
    pub eliminated: u64,
}
#[allow(dead_code)]
impl ErasedDCE {
    /// Create a DCE pass.
    pub fn new() -> Self {
        ErasedDCE { eliminated: 0 }
    }
    /// Eliminate TypeErased nodes from applications.
    pub fn elim(&mut self, expr: ErasedExprExt) -> ErasedExprExt {
        match expr {
            ErasedExprExt::App(f, x) => {
                let f = self.elim(*f);
                let x = self.elim(*x);
                if x == ErasedExprExt::TypeErased {
                    self.eliminated += 1;
                    f
                } else {
                    ErasedExprExt::App(Box::new(f), Box::new(x))
                }
            }
            ErasedExprExt::Lam(b) => {
                let b = self.elim(*b);
                if b == ErasedExprExt::TypeErased {
                    self.eliminated += 1;
                    ErasedExprExt::TypeErased
                } else {
                    ErasedExprExt::Lam(Box::new(b))
                }
            }
            ErasedExprExt::Let(v, b) => {
                let v = self.elim(*v);
                let b = self.elim(*b);
                ErasedExprExt::Let(Box::new(v), Box::new(b))
            }
            other => other,
        }
    }
}
/// A stack-based evaluator for erased expressions.
#[allow(dead_code)]
pub struct ErasedStack {
    stack: Vec<ErasedExpr>,
}
#[allow(dead_code)]
impl ErasedStack {
    /// Create an empty stack.
    pub fn new() -> Self {
        ErasedStack { stack: Vec::new() }
    }
    /// Push an expression onto the stack.
    pub fn push(&mut self, expr: ErasedExpr) {
        self.stack.push(expr);
    }
    /// Pop an expression from the stack.
    pub fn pop(&mut self) -> Option<ErasedExpr> {
        self.stack.pop()
    }
    /// Peek at the top expression.
    pub fn peek(&self) -> Option<&ErasedExpr> {
        self.stack.last()
    }
    /// Return the stack depth.
    pub fn depth(&self) -> usize {
        self.stack.len()
    }
    /// Return whether the stack is empty.
    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }
}
/// A flat representation of a spine: `f a0 a1 … aN`.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct ErasedFlatApp {
    /// The head of the application.
    pub head: ErasedExpr,
    /// The argument list.
    pub args: Vec<ErasedExpr>,
}
#[allow(dead_code)]
impl ErasedFlatApp {
    /// Flattens an `ErasedExpr` into its head and argument list.
    pub fn from_expr(mut expr: ErasedExpr) -> Self {
        let mut args = Vec::new();
        loop {
            match expr {
                ErasedExpr::App(f, a) => {
                    args.push(*a);
                    expr = *f;
                }
                other => {
                    args.reverse();
                    return Self { head: other, args };
                }
            }
        }
    }
    /// Rebuilds a left-spine `App` chain from the flat representation.
    pub fn into_expr(self) -> ErasedExpr {
        let mut result = self.head;
        for arg in self.args {
            result = ErasedExpr::App(Box::new(result), Box::new(arg));
        }
        result
    }
    /// Returns the total arity (number of arguments).
    pub fn arity(&self) -> usize {
        self.args.len()
    }
}
/// A small-step interpreter for erased expressions.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct ErasedInterpreter {
    pub steps: u64,
    pub max_steps: u64,
    scope: ErasedScope,
}
#[allow(dead_code)]
impl ErasedInterpreter {
    /// Create a new interpreter.
    pub fn new(max_steps: u64) -> Self {
        ErasedInterpreter {
            steps: 0,
            max_steps,
            scope: ErasedScope::new(),
        }
    }
    /// Evaluate an expression to a value.
    pub fn eval(&mut self, expr: ErasedExprExt) -> Option<ErasedValue> {
        if self.steps >= self.max_steps {
            return None;
        }
        self.steps += 1;
        match expr {
            ErasedExprExt::Lit(n) => Some(ErasedValue::Int(n)),
            ErasedExprExt::Unit => Some(ErasedValue::Unit),
            ErasedExprExt::TypeErased => Some(ErasedValue::Unit),
            ErasedExprExt::Lam(b) => Some(ErasedValue::Closure {
                body: b,
                env: Vec::new(),
            }),
            ErasedExprExt::BVar(_) => None,
            ErasedExprExt::FVar(name) => self.scope.lookup(&name).cloned(),
            ErasedExprExt::Const(_) => None,
            ErasedExprExt::CtorTag(t) => Some(ErasedValue::Ctor(t, Vec::new())),
            ErasedExprExt::App(f, x) => {
                let f_val = self.eval(*f)?;
                let x_val = self.eval(*x)?;
                match f_val {
                    ErasedValue::Closure { body, env: _ } => {
                        let subst =
                            subst_bvar0(*body, ErasedExprExt::Lit(x_val.as_int().unwrap_or(0)));
                        self.eval(subst)
                    }
                    ErasedValue::Ctor(t, mut fields) => {
                        fields.push(x_val);
                        Some(ErasedValue::Ctor(t, fields))
                    }
                    _ => None,
                }
            }
            ErasedExprExt::Let(v, b) => {
                let v_val = self.eval(*v)?;
                let subst = subst_bvar0(*b, ErasedExprExt::Lit(v_val.as_int().unwrap_or(0)));
                self.eval(subst)
            }
        }
    }
    /// Return whether the step limit was exceeded.
    pub fn is_exhausted(&self) -> bool {
        self.steps >= self.max_steps
    }
}
/// Pools integer literals to avoid duplication in the erased IR.
#[allow(dead_code)]
pub struct ErasedConstantPool {
    pool: Vec<i64>,
    index: std::collections::HashMap<i64, usize>,
}
#[allow(dead_code)]
impl ErasedConstantPool {
    /// Creates an empty constant pool.
    pub fn new() -> Self {
        Self {
            pool: Vec::new(),
            index: std::collections::HashMap::new(),
        }
    }
    /// Interns `val` and returns its pool index.
    pub fn intern(&mut self, val: i64) -> usize {
        if let Some(&idx) = self.index.get(&val) {
            return idx;
        }
        let idx = self.pool.len();
        self.pool.push(val);
        self.index.insert(val, idx);
        idx
    }
    /// Returns the value at pool index `idx`, or `None`.
    pub fn get(&self, idx: usize) -> Option<i64> {
        self.pool.get(idx).copied()
    }
    /// Returns the total number of interned constants.
    pub fn len(&self) -> usize {
        self.pool.len()
    }
    /// Returns `true` if the pool is empty.
    pub fn is_empty(&self) -> bool {
        self.pool.is_empty()
    }
}
/// Prints an erased expression to a structured string format.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct ErasedPrinter {
    pub output: String,
    depth: usize,
}
#[allow(dead_code)]
impl ErasedPrinter {
    /// Create a new printer.
    pub fn new() -> Self {
        ErasedPrinter {
            output: String::new(),
            depth: 0,
        }
    }
    /// Print an expression.
    pub fn print(&mut self, expr: &ErasedExprExt) {
        let s = pretty_print_erased(expr);
        for _ in 0..self.depth {
            self.output.push_str("  ");
        }
        self.output.push_str(&s);
        self.output.push('\n');
    }
    /// Return the printed output.
    pub fn result(&self) -> &str {
        &self.output
    }
    /// Clear the output.
    pub fn clear(&mut self) {
        self.output.clear();
        self.depth = 0;
    }
}
/// Performs type erasure on kernel expressions.
pub struct TypeEraser {
    config: EraseConfig,
}
impl TypeEraser {
    /// Create a type eraser with the default configuration.
    pub fn new() -> Self {
        TypeEraser {
            config: EraseConfig::default(),
        }
    }
    /// Create a type eraser with a custom configuration.
    pub fn with_config(config: EraseConfig) -> Self {
        TypeEraser { config }
    }
    /// Erase a sort (universe level) — always becomes `TypeErased`.
    pub fn erase_sort(&self) -> ErasedExpr {
        ErasedExpr::TypeErased
    }
    /// Erase a Pi-type — types are always erased to `TypeErased`.
    pub fn erase_pi(&self) -> ErasedExpr {
        ErasedExpr::TypeErased
    }
    /// Wrap a lambda body in a `Lam` node (type annotation already dropped).
    pub fn erase_lam_body(&self, body: ErasedExpr) -> ErasedExpr {
        ErasedExpr::Lam(Box::new(body))
    }
    /// Build an erased application from an already-erased function and argument.
    pub fn erase_app(f: ErasedExpr, arg: ErasedExpr) -> ErasedExpr {
        ErasedExpr::App(Box::new(f), Box::new(arg))
    }
    /// Erase a natural-number literal.
    pub fn erase_lit(n: u64) -> ErasedExpr {
        ErasedExpr::Lit(n)
    }
    /// Optimise an erased expression by beta-reducing `TypeErased` applications.
    ///
    /// When the function position of an `App` is `TypeErased`, the whole
    /// application is also `TypeErased` (the argument is a type).  This
    /// recursively simplifies the tree.
    pub fn optimize(&self, expr: ErasedExpr) -> ErasedExpr {
        match expr {
            ErasedExpr::App(f, arg) => {
                let f_opt = self.optimize(*f);
                let arg_opt = self.optimize(*arg);
                match f_opt {
                    ErasedExpr::TypeErased => ErasedExpr::TypeErased,
                    _ => {
                        if !self.config.keep_props && arg_opt == ErasedExpr::TypeErased {
                            if let ErasedExpr::Lam(body) = f_opt {
                                return self.optimize(subst_bvar(
                                    *body,
                                    0,
                                    &ErasedExpr::TypeErased,
                                ));
                            }
                        }
                        ErasedExpr::App(Box::new(f_opt), Box::new(arg_opt))
                    }
                }
            }
            ErasedExpr::Lam(body) => ErasedExpr::Lam(Box::new(self.optimize(*body))),
            ErasedExpr::Let(val, body) => ErasedExpr::Let(
                Box::new(self.optimize(*val)),
                Box::new(self.optimize(*body)),
            ),
            other => other,
        }
    }
}
/// A pattern in an erased match expression.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErasedPattern {
    /// Match any value (wildcard).
    Wildcard,
    /// Match a specific constructor tag.
    Ctor(u32, Vec<ErasedPattern>),
    /// Match a specific integer literal.
    Lit(u64),
    /// Bind a variable.
    Var(String),
}
impl ErasedPattern {
    /// Return the depth of the pattern.
    #[allow(dead_code)]
    pub fn depth(&self) -> usize {
        match self {
            ErasedPattern::Ctor(_, pats) => 1 + pats.iter().map(|p| p.depth()).max().unwrap_or(0),
            _ => 1,
        }
    }
    /// Return true if this pattern always matches.
    #[allow(dead_code)]
    pub fn is_irrefutable(&self) -> bool {
        matches!(self, ErasedPattern::Wildcard | ErasedPattern::Var(_))
    }
}
/// A substitution map from de Bruijn index to erased expression.
#[allow(dead_code)]
pub struct ErasedSubstMap {
    map: std::collections::HashMap<u32, ErasedExpr>,
}
#[allow(dead_code)]
impl ErasedSubstMap {
    /// Creates a new empty substitution map.
    pub fn new() -> Self {
        Self {
            map: std::collections::HashMap::new(),
        }
    }
    /// Inserts a substitution for de Bruijn index `i`.
    pub fn insert(&mut self, i: u32, expr: ErasedExpr) {
        self.map.insert(i, expr);
    }
    /// Looks up the substitution for de Bruijn index `i`.
    pub fn get(&self, i: u32) -> Option<&ErasedExpr> {
        self.map.get(&i)
    }
    /// Returns the number of entries in the map.
    pub fn len(&self) -> usize {
        self.map.len()
    }
    /// Returns `true` if the map is empty.
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
}
/// Configuration for the type eraser.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct EraseConfig {
    /// When true, keep `Prop`-sorted terms (proofs) as `TypeErased` rather
    /// than dropping them entirely.
    pub keep_props: bool,
    /// When true, inline small definitions before erasure.
    pub inline_defs: bool,
}
/// A normalizer for erased expressions (beta + delta reduction).
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct ErasedNormalizer {
    pub beta_steps: u64,
    pub const_folds: u64,
    max_beta: u64,
}
#[allow(dead_code)]
impl ErasedNormalizer {
    /// Create a normalizer with a beta step limit.
    pub fn new(max_beta: u64) -> Self {
        ErasedNormalizer {
            beta_steps: 0,
            const_folds: 0,
            max_beta,
        }
    }
    /// Normalize an expression to weak head normal form (stub).
    pub fn whnf(&mut self, expr: ErasedExpr) -> ErasedExpr {
        if self.beta_steps >= self.max_beta {
            return expr;
        }
        match expr {
            ErasedExpr::App(f, x) => {
                let f_whnf = self.whnf(*f);
                match f_whnf {
                    ErasedExpr::Lam(_b) => {
                        self.beta_steps += 1;
                        self.whnf(*x)
                    }
                    other_f => ErasedExpr::App(Box::new(other_f), x),
                }
            }
            other => other,
        }
    }
    /// Constant-fold addition: Nat.add (Lit a) (Lit b) → Lit (a+b).
    pub fn const_fold_add(&mut self, expr: ErasedExpr) -> ErasedExpr {
        match &expr {
            ErasedExpr::App(f, x) => {
                if let ErasedExpr::App(g, a) = f.as_ref() {
                    if let ErasedExpr::Const(name) = g.as_ref() {
                        if name == "Nat.add" {
                            if let (ErasedExpr::Lit(av), ErasedExpr::Lit(bv)) =
                                (a.as_ref(), x.as_ref())
                            {
                                self.const_folds += 1;
                                return ErasedExpr::Lit(av.saturating_add(*bv));
                            }
                        }
                    }
                }
                expr
            }
            _ => expr,
        }
    }
}
/// A-normal form converter for erased expressions.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct AnfConverter {
    fresh_counter: u64,
    pub let_count: u64,
}
#[allow(dead_code)]
impl AnfConverter {
    /// Create a new converter.
    pub fn new() -> Self {
        AnfConverter {
            fresh_counter: 0,
            let_count: 0,
        }
    }
    /// Generate a fresh variable name.
    pub fn fresh(&mut self) -> String {
        let name = format!("_anf_{}", self.fresh_counter);
        self.fresh_counter += 1;
        name
    }
    /// Convert an expression to ANF.
    pub fn convert(&mut self, expr: ErasedExprExt) -> ErasedExprExt {
        match expr {
            ErasedExprExt::App(f, x) => {
                let f = self.convert(*f);
                let x = self.convert(*x);
                if !is_atom(&f) {
                    let _v = self.fresh();
                    self.let_count += 1;
                    ErasedExprExt::Let(
                        Box::new(f),
                        Box::new(ErasedExprExt::App(
                            Box::new(ErasedExprExt::BVar(0)),
                            Box::new(shift_up(x, 1)),
                        )),
                    )
                } else {
                    ErasedExprExt::App(Box::new(f), Box::new(x))
                }
            }
            ErasedExprExt::Lam(b) => ErasedExprExt::Lam(Box::new(self.convert(*b))),
            other => other,
        }
    }
}
/// An erased value (runtime representation after full reduction).
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq)]
pub enum ErasedValue {
    /// An integer literal.
    Int(u64),
    /// A constructor applied to arguments.
    Ctor(u32, Vec<ErasedValue>),
    /// A closure (lambda) with its captured environment.
    Closure {
        body: Box<ErasedExprExt>,
        env: Vec<ErasedValue>,
    },
    /// A unit value.
    Unit,
    /// An unresolved thunk.
    Thunk(Box<ErasedExprExt>),
}
#[allow(dead_code)]
impl ErasedValue {
    /// Return true if this is an integer.
    pub fn is_int(&self) -> bool {
        matches!(self, ErasedValue::Int(_))
    }
    /// Return true if this is a constructor.
    pub fn is_ctor(&self) -> bool {
        matches!(self, ErasedValue::Ctor(_, _))
    }
    /// Return the integer value if this is Int, else None.
    pub fn as_int(&self) -> Option<u64> {
        if let ErasedValue::Int(n) = self {
            Some(*n)
        } else {
            None
        }
    }
}
/// Bit manipulation operations for erased literal values.
#[allow(dead_code)]
pub struct ErasedBitOps;
#[allow(dead_code)]
impl ErasedBitOps {
    /// Fold a list of literals with bitwise AND.
    pub fn fold_and(vals: &[u64]) -> u64 {
        vals.iter().fold(u64::MAX, |acc, &v| acc & v)
    }
    /// Fold a list of literals with bitwise OR.
    pub fn fold_or(vals: &[u64]) -> u64 {
        vals.iter().fold(0u64, |acc, &v| acc | v)
    }
    /// Fold a list of literals with a binary operation.
    pub fn fold_binop<F: Fn(u64, u64) -> u64>(vals: &[u64], init: u64, f: F) -> u64 {
        vals.iter().fold(init, |acc, &v| f(acc, v))
    }
}
/// Rename all free variables in an expression using a substitution map.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct ErasedRenamer {
    map: Vec<(String, String)>,
    pub renames: u64,
}
#[allow(dead_code)]
impl ErasedRenamer {
    /// Create a renamer with a substitution map.
    pub fn new(map: Vec<(String, String)>) -> Self {
        ErasedRenamer { map, renames: 0 }
    }
    /// Apply the renaming to an expression.
    pub fn rename(&mut self, expr: ErasedExprExt) -> ErasedExprExt {
        match expr {
            ErasedExprExt::FVar(name) => {
                if let Some(new) = self
                    .map
                    .iter()
                    .find(|(old, _)| old == &name)
                    .map(|(_, n)| n.clone())
                {
                    self.renames += 1;
                    ErasedExprExt::FVar(new)
                } else {
                    ErasedExprExt::FVar(name)
                }
            }
            ErasedExprExt::Const(name) => {
                if let Some(new) = self
                    .map
                    .iter()
                    .find(|(old, _)| old == &name)
                    .map(|(_, n)| n.clone())
                {
                    self.renames += 1;
                    ErasedExprExt::Const(new)
                } else {
                    ErasedExprExt::Const(name)
                }
            }
            ErasedExprExt::Lam(b) => ErasedExprExt::Lam(Box::new(self.rename(*b))),
            ErasedExprExt::App(f, x) => {
                ErasedExprExt::App(Box::new(self.rename(*f)), Box::new(self.rename(*x)))
            }
            ErasedExprExt::Let(v, b) => {
                ErasedExprExt::Let(Box::new(self.rename(*v)), Box::new(self.rename(*b)))
            }
            other => other,
        }
    }
}
/// A kernel expression with type information erased.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErasedExpr {
    /// Erased free variable.
    Var(String),
    /// Bound variable (de Bruijn index).
    BVar(u32),
    /// Lambda without type annotation.
    Lam(Box<ErasedExpr>),
    /// Application.
    App(Box<ErasedExpr>, Box<ErasedExpr>),
    /// Let binding without type.
    Let(Box<ErasedExpr>, Box<ErasedExpr>),
    /// Global constant reference.
    Const(String),
    /// Literal natural number.
    Lit(u64),
    /// String literal.
    StrLit(String),
    /// Placeholder for erased types (sorts, pi-types, etc.).
    TypeErased,
}
/// Describes a call site in the erased IR.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct ErasedCallSite {
    /// Name of the callee constant.
    pub callee: String,
    /// Arity of the call (number of arguments supplied).
    pub arity: usize,
    /// Whether this is a tail call.
    pub is_tail: bool,
}
#[allow(dead_code)]
impl ErasedCallSite {
    /// Creates a new call site descriptor.
    pub fn new(callee: impl Into<String>, arity: usize, is_tail: bool) -> Self {
        Self {
            callee: callee.into(),
            arity,
            is_tail,
        }
    }
    /// Returns `true` if the call is a self-recursive tail call.
    pub fn is_self_tail(&self, current: &str) -> bool {
        self.is_tail && self.callee == current
    }
}
/// Checks size bounds on erased expressions.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct ErasedSizeBound {
    pub max_size: usize,
}
#[allow(dead_code)]
impl ErasedSizeBound {
    /// Create a size checker.
    pub fn new(max_size: usize) -> Self {
        ErasedSizeBound { max_size }
    }
    /// Return whether an expression is within the size bound.
    pub fn check(&self, expr: &ErasedExprExt) -> bool {
        expr.size() <= self.max_size
    }
    /// Return the size of an expression.
    pub fn size_of(&self, expr: &ErasedExprExt) -> usize {
        expr.size()
    }
}
/// Statistics collected during type erasure.
#[derive(Debug, Clone, Default)]
pub struct ErasureStats {
    /// Number of sorts (universes) erased.
    pub sorts_erased: usize,
    /// Number of Pi-types erased.
    pub pis_erased: usize,
    /// Number of computational terms kept.
    pub terms_kept: usize,
}
impl ErasureStats {
    /// Create a zeroed statistics record.
    pub fn new() -> Self {
        ErasureStats::default()
    }
    /// Record one erased sort.
    pub fn add_sort(&mut self) {
        self.sorts_erased += 1;
    }
    /// Record one erased Pi-type.
    pub fn add_pi(&mut self) {
        self.pis_erased += 1;
    }
    /// Record one kept computational term.
    pub fn add_term(&mut self) {
        self.terms_kept += 1;
    }
    /// Fraction of nodes that were erased (sorts + pis out of total).
    ///
    /// Returns `0.0` when no nodes have been recorded.
    pub fn ratio_erased(&self) -> f64 {
        let total = self.sorts_erased + self.pis_erased + self.terms_kept;
        if total == 0 {
            0.0
        } else {
            (self.sorts_erased + self.pis_erased) as f64 / total as f64
        }
    }
}
/// A reachability tracker for erased expressions.
#[allow(dead_code)]
pub struct ErasedReachability {
    reachable: Vec<String>,
    roots: Vec<String>,
}
#[allow(dead_code)]
impl ErasedReachability {
    /// Create a new tracker.
    pub fn new() -> Self {
        ErasedReachability {
            reachable: Vec::new(),
            roots: Vec::new(),
        }
    }
    /// Add a root (entry point).
    pub fn add_root(&mut self, name: &str) {
        if !self.roots.contains(&name.to_string()) {
            self.roots.push(name.to_string());
            self.mark_reachable(name);
        }
    }
    /// Mark a constant as reachable.
    pub fn mark_reachable(&mut self, name: &str) {
        if !self.reachable.contains(&name.to_string()) {
            self.reachable.push(name.to_string());
        }
    }
    /// Return whether a constant is reachable.
    pub fn is_reachable(&self, name: &str) -> bool {
        self.reachable.contains(&name.to_string())
    }
    /// Return all reachable names.
    pub fn reachable_names(&self) -> &[String] {
        &self.reachable
    }
    /// Return the number of reachable declarations.
    pub fn reachable_count(&self) -> usize {
        self.reachable.len()
    }
    /// Process a module: mark all consts reachable from roots as reachable.
    pub fn analyze_module(&mut self, module: &ErasedModule) {
        let root_names = self.roots.clone();
        for root in &root_names {
            if let Some(ErasedDecl::Def { body, .. }) = module.find(root) {
                for c in collect_consts(body) {
                    self.mark_reachable(&c);
                }
            }
        }
    }
}
/// A chain of let-bindings produced by ANF conversion.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct ErasedLetChain {
    /// The binding pairs: (name_hint, rhs).
    pub bindings: Vec<(String, ErasedExprExt)>,
    /// The final body after all bindings.
    pub body: ErasedExprExt,
}
#[allow(dead_code)]
impl ErasedLetChain {
    /// Create an empty chain with the given body.
    pub fn new(body: ErasedExprExt) -> Self {
        Self {
            bindings: Vec::new(),
            body,
        }
    }
    /// Push a new binding onto the chain (outermost = last).
    pub fn push(&mut self, name: impl Into<String>, rhs: ErasedExprExt) {
        self.bindings.push((name.into(), rhs));
    }
    /// Convert the chain back into nested `ErasedExprExt::Let` nodes.
    pub fn into_expr(self) -> ErasedExprExt {
        let mut result = self.body;
        for (_name, rhs) in self.bindings.into_iter().rev() {
            result = ErasedExprExt::Let(Box::new(rhs), Box::new(result));
        }
        result
    }
    /// Returns the number of bindings in the chain.
    pub fn len(&self) -> usize {
        self.bindings.len()
    }
    /// Returns `true` if there are no bindings.
    pub fn is_empty(&self) -> bool {
        self.bindings.is_empty()
    }
}
/// Constant folding for erased arithmetic expressions.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct ErasedConstFolder {
    pub folds: u64,
}
#[allow(dead_code)]
impl ErasedConstFolder {
    /// Create a constant folder.
    pub fn new() -> Self {
        ErasedConstFolder { folds: 0 }
    }
    /// Try to constant-fold an application of a built-in operator.
    pub fn fold_add(&mut self, expr: ErasedExprExt) -> ErasedExprExt {
        match expr {
            ErasedExprExt::App(f, x) => {
                let f = self.fold_add(*f);
                let x = self.fold_add(*x);
                match (&f, &x) {
                    (ErasedExprExt::App(g, a), ErasedExprExt::Lit(b)) => {
                        if let ErasedExprExt::Const(name) = g.as_ref() {
                            if name == "Nat.add" {
                                if let ErasedExprExt::Lit(av) = a.as_ref() {
                                    self.folds += 1;
                                    return ErasedExprExt::Lit(av.saturating_add(*b));
                                }
                            }
                        }
                        ErasedExprExt::App(
                            Box::new(ErasedExprExt::App(
                                Box::new(*g.clone()),
                                Box::new(*a.clone()),
                            )),
                            Box::new(x),
                        )
                    }
                    _ => ErasedExprExt::App(Box::new(f), Box::new(x)),
                }
            }
            ErasedExprExt::Lam(b) => ErasedExprExt::Lam(Box::new(self.fold_add(*b))),
            other => other,
        }
    }
}
/// An erased module declaration.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Clone)]
pub enum ErasedDecl {
    /// A function definition.
    Def { name: String, body: ErasedExprExt },
    /// An axiom (no body).
    Axiom { name: String },
    /// An inductive type with constructor count.
    Inductive { name: String, ctor_count: u32 },
}
#[allow(dead_code)]
impl ErasedDecl {
    /// Return the name of this declaration.
    pub fn name(&self) -> &str {
        match self {
            ErasedDecl::Def { name, .. } => name,
            ErasedDecl::Axiom { name } => name,
            ErasedDecl::Inductive { name, .. } => name,
        }
    }
    /// Return true if this is a function definition.
    pub fn is_def(&self) -> bool {
        matches!(self, ErasedDecl::Def { .. })
    }
}
/// A scope for erased code: a stack of local bindings.
#[allow(dead_code)]
pub struct ErasedScope {
    vars: Vec<(String, ErasedValue)>,
}
#[allow(dead_code)]
impl ErasedScope {
    /// Create an empty scope.
    pub fn new() -> Self {
        ErasedScope { vars: Vec::new() }
    }
    /// Bind a variable.
    pub fn bind(&mut self, name: &str, val: ErasedValue) {
        self.vars.push((name.to_string(), val));
    }
    /// Look up a variable.
    pub fn lookup(&self, name: &str) -> Option<&ErasedValue> {
        self.vars
            .iter()
            .rev()
            .find(|(n, _)| n == name)
            .map(|(_, v)| v)
    }
    /// Return the scope depth.
    pub fn depth(&self) -> usize {
        self.vars.len()
    }
    /// Push a checkpoint and return it (index into vars).
    pub fn save(&self) -> usize {
        self.vars.len()
    }
    /// Restore to a checkpoint.
    pub fn restore(&mut self, checkpoint: usize) {
        self.vars.truncate(checkpoint);
    }
}
