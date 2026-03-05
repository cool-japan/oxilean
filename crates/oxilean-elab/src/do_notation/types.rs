//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Expr, Name};
use std::collections::HashMap;

use super::functions::*;

/// A builder for constructing a chain of monadic bind steps.
#[derive(Clone, Debug)]
pub struct MonadChain {
    /// The monad type expression.
    monad: Expr,
    /// The elements accumulated so far.
    elems: Vec<DoElem>,
}
impl MonadChain {
    /// Create a new chain for the given monad type.
    pub fn new(monad: Expr) -> Self {
        Self {
            monad,
            elems: Vec::new(),
        }
    }
    /// Append a bind step: `name <- action`.
    pub fn bind(mut self, name: Name, action: Expr) -> Self {
        self.elems.push(DoElem::bind(name, action));
        self
    }
    /// Append a let-binding step: `let name := val`.
    pub fn let_bind(mut self, name: Name, val: Expr) -> Self {
        self.elems.push(DoElem::let_bind(name, val));
        self
    }
    /// Append a pure action step.
    pub fn action(mut self, expr: Expr) -> Self {
        self.elems.push(DoElem::action(expr));
        self
    }
    /// Append a `return` step.
    pub fn return_(mut self, expr: Expr) -> Self {
        self.elems.push(DoElem::return_(expr));
        self
    }
    /// Build the final `DoBlock`.
    pub fn build(self) -> DoBlock {
        DoBlock::with_monad(self.monad, self.elems)
    }
    /// Build the `DoBlock` and elaborate it.
    pub fn elaborate(self, config: &DoElabConfig) -> Result<Expr, DoElabError> {
        let block = self.build();
        elaborate_do(&block, None, config)
    }
    /// Return the number of steps accumulated.
    pub fn len(&self) -> usize {
        self.elems.len()
    }
    /// Return `true` if no steps have been added.
    pub fn is_empty(&self) -> bool {
        self.elems.is_empty()
    }
    /// Access the monad type.
    pub fn monad(&self) -> &Expr {
        &self.monad
    }
}
/// Statistics gathered during do-block elaboration.
#[derive(Clone, Debug, Default)]
pub struct DoElabStats {
    /// Number of do-blocks elaborated.
    pub blocks_elaborated: u64,
    /// Total bind operations desugared.
    pub binds_desugared: u64,
    /// Total let bindings handled.
    pub lets_desugared: u64,
    /// Total for-loops desugared.
    pub for_loops_desugared: u64,
    /// Total try-catch blocks desugared.
    pub try_catch_desugared: u64,
    /// Elaboration failures.
    pub failures: u64,
}
impl DoElabStats {
    /// Create zeroed stats.
    pub fn new() -> Self {
        Self::default()
    }
    /// Record one block elaboration.
    pub fn record_block(&mut self, binds: u64, lets: u64, ok: bool) {
        self.blocks_elaborated += 1;
        self.binds_desugared += binds;
        self.lets_desugared += lets;
        if !ok {
            self.failures += 1;
        }
    }
    /// Record a for-loop desugaring.
    pub fn record_for_loop(&mut self) {
        self.for_loops_desugared += 1;
    }
    /// Record a try-catch desugaring.
    pub fn record_try_catch(&mut self) {
        self.try_catch_desugared += 1;
    }
    /// Merge another stats object into this one.
    pub fn merge(&mut self, other: &DoElabStats) {
        self.blocks_elaborated += other.blocks_elaborated;
        self.binds_desugared += other.binds_desugared;
        self.lets_desugared += other.lets_desugared;
        self.for_loops_desugared += other.for_loops_desugared;
        self.try_catch_desugared += other.try_catch_desugared;
        self.failures += other.failures;
    }
    /// Success rate in [0, 1].
    pub fn success_rate(&self) -> f64 {
        if self.blocks_elaborated == 0 {
            1.0
        } else {
            (self.blocks_elaborated - self.failures) as f64 / self.blocks_elaborated as f64
        }
    }
    /// One-line summary.
    pub fn summary(&self) -> String {
        format!(
            "blocks={} binds={} lets={} for={} try={} failures={}",
            self.blocks_elaborated,
            self.binds_desugared,
            self.lets_desugared,
            self.for_loops_desugared,
            self.try_catch_desugared,
            self.failures,
        )
    }
}
/// A complete do-notation block.
#[derive(Clone, Debug)]
pub struct DoBlock {
    /// The monad type (if explicitly specified or inferred).
    pub monad_type: Option<Expr>,
    /// The elements of the do block.
    pub elems: Vec<DoElem>,
}
impl DoBlock {
    /// Create a new do block.
    pub fn new(elems: Vec<DoElem>) -> Self {
        Self {
            monad_type: None,
            elems,
        }
    }
    /// Create a do block with an explicit monad type.
    pub fn with_monad(monad_type: Expr, elems: Vec<DoElem>) -> Self {
        Self {
            monad_type: Some(monad_type),
            elems,
        }
    }
    /// Check if the do block is empty.
    pub fn is_empty(&self) -> bool {
        self.elems.is_empty()
    }
    /// Number of elements.
    pub fn len(&self) -> usize {
        self.elems.len()
    }
    /// Get the last element.
    pub fn last(&self) -> Option<&DoElem> {
        self.elems.last()
    }
    /// Validate the structure of the do block.
    ///
    /// A well-formed do block must:
    /// - Not be empty
    /// - End with an action or return (not a bind)
    pub fn validate(&self) -> Result<(), DoElabError> {
        if self.elems.is_empty() {
            return Err(DoElabError::EmptyDoBlock);
        }
        let last = self
            .elems
            .last()
            .expect("elems is non-empty (checked above)");
        match last {
            DoElem::Bind { .. } => Err(DoElabError::BindAtEnd(
                "do block cannot end with a bind (<-)".to_string(),
            )),
            DoElem::LetBind { .. } => Err(DoElabError::BindAtEnd(
                "do block cannot end with a let binding".to_string(),
            )),
            _ => Ok(()),
        }
    }
}
/// A resolved monad instance, providing the bind, pure, map, and seq functions.
#[derive(Clone, Debug)]
pub struct MonadInstance {
    /// The `bind` function: `m a -> (a -> m b) -> m b`.
    pub bind_fn: Expr,
    /// The `pure` function: `a -> m a`.
    pub pure_fn: Expr,
    /// The `map` function: `(a -> b) -> m a -> m b` (optional, derivable).
    pub map_fn: Option<Expr>,
    /// The `seq` function: `m a -> m b -> m b` (optional, derivable).
    pub seq_fn: Option<Expr>,
    /// The monad type constructor (e.g., `IO`, `Option`, `State S`).
    pub monad_type: Expr,
    /// The name of the monad (for error messages).
    pub monad_name: Name,
}
impl MonadInstance {
    /// Create a new monad instance with bind and pure.
    pub fn new(bind_fn: Expr, pure_fn: Expr, monad_type: Expr, monad_name: Name) -> Self {
        Self {
            bind_fn,
            pure_fn,
            map_fn: None,
            seq_fn: None,
            monad_type,
            monad_name,
        }
    }
    /// Set the map function.
    pub fn with_map(mut self, map_fn: Expr) -> Self {
        self.map_fn = Some(map_fn);
        self
    }
    /// Set the seq function.
    pub fn with_seq(mut self, seq_fn: Expr) -> Self {
        self.seq_fn = Some(seq_fn);
        self
    }
    /// Build a bind expression: `bind rhs (fun var => body)`.
    pub fn mk_bind(&self, var: &Name, var_ty: &Expr, rhs: &Expr, body: &Expr) -> Expr {
        let continuation = Expr::Lam(
            BinderInfo::Default,
            var.clone(),
            Box::new(var_ty.clone()),
            Box::new(body.clone()),
        );
        let app1 = Expr::App(Box::new(self.bind_fn.clone()), Box::new(rhs.clone()));
        Expr::App(Box::new(app1), Box::new(continuation))
    }
    /// Build a pure expression: `pure val`.
    pub fn mk_pure(&self, val: &Expr) -> Expr {
        Expr::App(Box::new(self.pure_fn.clone()), Box::new(val.clone()))
    }
    /// Build a seq expression: `seq a b` or `bind a (fun _ => b)`.
    pub fn mk_seq(&self, first: &Expr, second: &Expr) -> Expr {
        if let Some(seq_fn) = &self.seq_fn {
            let app1 = Expr::App(Box::new(seq_fn.clone()), Box::new(first.clone()));
            Expr::App(Box::new(app1), Box::new(second.clone()))
        } else {
            let unit_ty = Expr::Const(Name::str("Unit"), vec![]);
            self.mk_bind(&Name::str("_"), &unit_ty, first, second)
        }
    }
    /// Build a map expression: `map f a` or `bind a (fun x => pure (f x))`.
    pub fn mk_map(&self, f: &Expr, action: &Expr) -> Expr {
        if let Some(map_fn) = &self.map_fn {
            let app1 = Expr::App(Box::new(map_fn.clone()), Box::new(f.clone()));
            Expr::App(Box::new(app1), Box::new(action.clone()))
        } else {
            let x_name = Name::str("x");
            let x_var = Expr::BVar(0);
            let f_x = Expr::App(Box::new(f.clone()), Box::new(x_var));
            let pure_f_x = self.mk_pure(&f_x);
            let inferred_ty = Expr::Const(Name::str("_"), vec![]);
            self.mk_bind(&x_name, &inferred_ty, action, &pure_f_x)
        }
    }
}
/// A parser-level do action (simplified representation).
#[derive(Clone, Debug)]
pub enum ParseDoAction {
    /// let x := e
    Let(String, Expr),
    /// let x : tau := e
    LetTyped(String, Expr, Expr),
    /// x <- e
    Bind(String, Expr),
    /// pure expression
    Expr(Expr),
    /// return e
    Return(Expr),
}
/// An element within a do-notation block.
///
/// Each element represents one "line" of monadic computation.
#[derive(Clone, Debug)]
pub enum DoElem {
    /// Monadic bind: `pat <- rhs`
    ///
    /// Binds the result of `rhs` to `pat` and continues with the rest.
    Bind {
        /// The pattern to bind to (usually a simple variable name).
        pat: Name,
        /// Optional type annotation for the bound variable.
        ty: Option<Expr>,
        /// The monadic action whose result is bound.
        rhs: Expr,
    },
    /// Let-bind: `let pat := val`
    ///
    /// A pure (non-monadic) local binding.
    LetBind {
        /// The variable name.
        pat: Name,
        /// Optional type annotation.
        ty: Option<Expr>,
        /// The value to bind.
        val: Expr,
    },
    /// A pure monadic action (no binding).
    ///
    /// `action` is executed for its effects, result discarded.
    Action(Expr),
    /// Return expression: `return e`
    ///
    /// Wraps a value in the monad using `pure`.
    Return(Expr),
    /// For loop: `for var in collection do body`
    ///
    /// Iterates over a collection, executing `body` for each element.
    For {
        /// The loop variable name.
        var: Name,
        /// The collection to iterate over.
        collection: Expr,
        /// The body of the loop.
        body: Box<DoElem>,
    },
    /// If-then-else inside a do block.
    If {
        /// The condition expression.
        cond: Expr,
        /// The then branch (a sequence of do elements).
        then_: Vec<DoElem>,
        /// The else branch (a sequence of do elements).
        else_: Vec<DoElem>,
    },
    /// Match expression inside a do block.
    Match {
        /// The expression being matched.
        scrutinee: Expr,
        /// The match arms: (pattern_name, body elements).
        arms: Vec<(Name, Vec<DoElem>)>,
    },
    /// Try-catch: `try body catch var => catch_body`
    TryCatch {
        /// The body that may throw.
        body: Vec<DoElem>,
        /// The variable name for the caught exception.
        catch_var: Name,
        /// The handler executed on exception.
        catch_body: Vec<DoElem>,
    },
    /// Unless: `unless cond do body`
    ///
    /// Executes `body` only if `cond` is false.
    Unless {
        /// The condition.
        cond: Expr,
        /// The body (executed when cond is false).
        body: Vec<DoElem>,
    },
}
impl DoElem {
    /// Create a simple bind element.
    pub fn bind(pat: Name, rhs: Expr) -> Self {
        DoElem::Bind { pat, ty: None, rhs }
    }
    /// Create a typed bind element.
    pub fn bind_typed(pat: Name, ty: Expr, rhs: Expr) -> Self {
        DoElem::Bind {
            pat,
            ty: Some(ty),
            rhs,
        }
    }
    /// Create a let-bind element.
    pub fn let_bind(pat: Name, val: Expr) -> Self {
        DoElem::LetBind { pat, ty: None, val }
    }
    /// Create a typed let-bind element.
    pub fn let_bind_typed(pat: Name, ty: Expr, val: Expr) -> Self {
        DoElem::LetBind {
            pat,
            ty: Some(ty),
            val,
        }
    }
    /// Create an action element.
    pub fn action(expr: Expr) -> Self {
        DoElem::Action(expr)
    }
    /// Create a return element.
    pub fn return_(expr: Expr) -> Self {
        DoElem::Return(expr)
    }
    /// Create a for loop element.
    pub fn for_loop(var: Name, collection: Expr, body: DoElem) -> Self {
        DoElem::For {
            var,
            collection,
            body: Box::new(body),
        }
    }
    /// Create an unless element.
    pub fn unless(cond: Expr, body: Vec<DoElem>) -> Self {
        DoElem::Unless { cond, body }
    }
    /// Create a try-catch element.
    pub fn try_catch(body: Vec<DoElem>, catch_var: Name, catch_body: Vec<DoElem>) -> Self {
        DoElem::TryCatch {
            body,
            catch_var,
            catch_body,
        }
    }
    /// Check if this element is a terminal element (return, action without continuation).
    pub fn is_terminal(&self) -> bool {
        matches!(self, DoElem::Return(_) | DoElem::Action(_))
    }
    /// Check if this element introduces a binding.
    pub fn is_binding(&self) -> bool {
        matches!(self, DoElem::Bind { .. } | DoElem::LetBind { .. })
    }
}
/// Errors that can occur during do-notation elaboration.
#[derive(Clone, Debug)]
pub enum DoElabError {
    /// The do block is empty.
    EmptyDoBlock,
    /// A bind expression at the end of a do block (no continuation).
    BindAtEnd(String),
    /// Failed to resolve the monad instance.
    NoMonadInstance(String),
    /// Type mismatch in a bind expression.
    TypeMismatch(String),
    /// Unknown monad operation.
    UnknownOperation(String),
    /// For-loop on a non-iterable type.
    NotIterable(String),
    /// Try-catch on a monad without exception support.
    NoExceptionSupport(String),
    /// Nesting depth exceeded.
    MaxDepthExceeded(usize),
    /// Internal error.
    InternalError(String),
}
/// A set of optimization passes for `DoBlock`s before elaboration.
#[derive(Clone, Debug, Default)]
pub struct DoBlockOptimizer {
    /// Replace `x <- pure e; rest` with `let x := e; rest`.
    pub inline_pure_bind: bool,
    /// Remove trailing `return ()` from IO blocks.
    pub elide_trailing_return_unit: bool,
}
impl DoBlockOptimizer {
    /// Create a default optimizer (all passes disabled).
    pub fn new() -> Self {
        Self::default()
    }
    /// Enable all optimization passes.
    pub fn all() -> Self {
        Self {
            inline_pure_bind: true,
            elide_trailing_return_unit: true,
        }
    }
    /// Apply all enabled passes to `block`.
    pub fn optimize(&self, mut block: DoBlock) -> DoBlock {
        if self.inline_pure_bind {
            block = self.apply_inline_pure_bind(block);
        }
        block
    }
    fn apply_inline_pure_bind(&self, block: DoBlock) -> DoBlock {
        let new_elems: Vec<DoElem> = block
            .elems
            .into_iter()
            .map(|e| {
                if let DoElem::Bind { pat, ty: None, rhs } = e {
                    if let Expr::App(f, arg) = &rhs {
                        if let Expr::Const(fn_name, _) = f.as_ref() {
                            if fn_name == &Name::str("pure") {
                                return DoElem::let_bind(pat, *arg.clone());
                            }
                        }
                    }
                    DoElem::bind(pat, rhs)
                } else {
                    e
                }
            })
            .collect();
        DoBlock {
            monad_type: block.monad_type,
            elems: new_elems,
        }
    }
}
/// Stateful do-notation expander that tracks bindings and context.
pub struct DoNotationExpander {
    /// Configuration.
    config: DoElabConfig,
    /// Resolved monad instances, keyed by type name.
    monad_cache: HashMap<Name, MonadInstance>,
    /// Current nesting depth.
    depth: usize,
    /// Binding counter for generating fresh names.
    fresh_counter: u64,
}
impl DoNotationExpander {
    /// Create a new do-notation expander.
    pub fn new(config: DoElabConfig) -> Self {
        Self {
            config,
            monad_cache: HashMap::new(),
            depth: 0,
            fresh_counter: 0,
        }
    }
    /// Create with default configuration.
    pub fn with_defaults() -> Self {
        Self::new(DoElabConfig::new())
    }
    /// Generate a fresh variable name.
    pub fn fresh_name(&mut self, prefix: &str) -> Name {
        let name = Name::str(format!("{}_{}", prefix, self.fresh_counter));
        self.fresh_counter += 1;
        name
    }
    /// Elaborate a do block.
    pub fn elaborate(&mut self, block: &DoBlock, expected_type: Option<&Expr>) -> ElabResult<Expr> {
        self.depth += 1;
        if self.depth > self.config.max_depth {
            self.depth -= 1;
            return Err(DoElabError::MaxDepthExceeded(self.depth));
        }
        let result = elaborate_do(block, expected_type, &self.config);
        self.depth -= 1;
        result
    }
    /// Cache a monad instance.
    pub fn cache_monad(&mut self, name: Name, instance: MonadInstance) {
        self.monad_cache.insert(name, instance);
    }
    /// Look up a cached monad instance.
    pub fn lookup_monad(&self, name: &Name) -> Option<&MonadInstance> {
        self.monad_cache.get(name)
    }
    /// Reset the expander state.
    pub fn reset(&mut self) {
        self.depth = 0;
        self.fresh_counter = 0;
        self.monad_cache.clear();
    }
    /// Get the current depth.
    pub fn current_depth(&self) -> usize {
        self.depth
    }
}
/// Tracks the nesting level of do-block expressions.
#[derive(Clone, Debug, Default)]
pub struct DoNestingLevel {
    depth: usize,
    monad_stack: Vec<Expr>,
}
impl DoNestingLevel {
    /// Create at depth 0.
    pub fn new() -> Self {
        Self::default()
    }
    /// Enter a nested do-block.
    pub fn enter(&mut self, monad: Expr) {
        self.depth += 1;
        self.monad_stack.push(monad);
    }
    /// Leave a nested do-block.
    pub fn leave(&mut self) {
        self.depth = self.depth.saturating_sub(1);
        self.monad_stack.pop();
    }
    /// Return the current nesting depth.
    pub fn depth(&self) -> usize {
        self.depth
    }
    /// Return the monad type at the current level.
    pub fn current_monad(&self) -> Option<&Expr> {
        self.monad_stack.last()
    }
    /// Return `true` if we are inside any do-block.
    pub fn is_nested(&self) -> bool {
        self.depth > 0
    }
}
/// Configuration for do-notation elaboration.
#[derive(Clone, Debug)]
pub struct DoElabConfig {
    /// Whether to automatically lift types through monad transformers.
    pub lift_types: bool,
    /// Whether to auto-insert `return` for the last expression.
    pub auto_return: bool,
    /// Whether to use strict bind (left-to-right evaluation).
    pub strict_bind: bool,
    /// Whether to allow for-loops.
    pub allow_for: bool,
    /// Whether to allow try-catch.
    pub allow_try_catch: bool,
    /// Whether to allow unless.
    pub allow_unless: bool,
    /// Maximum nesting depth for do blocks.
    pub max_depth: usize,
}
impl DoElabConfig {
    /// Create a default configuration.
    pub fn new() -> Self {
        Self {
            lift_types: true,
            auto_return: true,
            strict_bind: true,
            allow_for: true,
            allow_try_catch: true,
            allow_unless: true,
            max_depth: 100,
        }
    }
    /// Disable auto-return.
    pub fn without_auto_return(mut self) -> Self {
        self.auto_return = false;
        self
    }
    /// Disable type lifting.
    pub fn without_lift(mut self) -> Self {
        self.lift_types = false;
        self
    }
    /// Set strict bind mode.
    pub fn with_strict(mut self, strict: bool) -> Self {
        self.strict_bind = strict;
        self
    }
}
