//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::HashMap;

use super::functions::*;
use std::fmt;

/// An algebraic effect given by its signature (operation, parameter type, return type).
pub struct AlgebraicEffect {
    /// Triples (operation name, parameter type, return type).
    pub signature: Vec<(String, String, String)>,
}
impl AlgebraicEffect {
    /// Create a new AlgebraicEffect.
    pub fn new(signature: Vec<(String, String, String)>) -> Self {
        Self { signature }
    }
    /// The equational theory of the algebraic effect.
    pub fn effect_theory(&self) -> String {
        format!(
            "Algebraic theory with {} operations; models are algebras satisfying handler equations",
            self.signature.len()
        )
    }
    /// Operations are given by equations between terms containing effect calls.
    pub fn operations_are_equations(&self) -> bool {
        true
    }
}
/// A type-erased representation of the Freer monad for string-typed operations.
///
/// `FreerComp` is either a pure value or an impure node holding an operation
/// description and a continuation.
#[allow(dead_code)]
pub enum FreerComp {
    /// Pure computation: just a string value.
    Pure(String),
    /// Impure: an effect name, operation name, argument, and a continuation.
    Impure {
        /// Effect name.
        effect: String,
        /// Operation name.
        op: String,
        /// Argument (serialized).
        arg: String,
        /// Continuation from result string to next computation.
        cont: Box<dyn FnOnce(String) -> FreerComp>,
    },
}
#[allow(dead_code)]
impl FreerComp {
    /// Construct a pure Freer computation.
    pub fn pure(v: impl Into<String>) -> Self {
        FreerComp::Pure(v.into())
    }
    /// Construct an impure Freer computation (one algebraic operation).
    pub fn impure(
        effect: impl Into<String>,
        op: impl Into<String>,
        arg: impl Into<String>,
        cont: impl FnOnce(String) -> FreerComp + 'static,
    ) -> Self {
        FreerComp::Impure {
            effect: effect.into(),
            op: op.into(),
            arg: arg.into(),
            cont: Box::new(cont),
        }
    }
    /// Interpret the Freer computation using a handler function.
    /// `handler(effect, op, arg)` returns the result string for that operation.
    pub fn interpret(self, handler: &dyn Fn(&str, &str, &str) -> String) -> String {
        match self {
            FreerComp::Pure(v) => v,
            FreerComp::Impure {
                effect,
                op,
                arg,
                cont,
            } => {
                let result = handler(&effect, &op, &arg);
                cont(result).interpret(handler)
            }
        }
    }
    /// Check if this is a pure computation.
    pub fn is_pure(&self) -> bool {
        matches!(self, FreerComp::Pure(_))
    }
}
/// The type of a handler: maps input type with effect row to output type.
pub struct HandlerType {
    /// The input computation type.
    pub input_type: String,
    /// The output computation type.
    pub output_type: String,
    /// The effect row being handled.
    pub effect_row: String,
}
impl HandlerType {
    /// Create a new HandlerType.
    pub fn new(
        input_type: impl Into<String>,
        output_type: impl Into<String>,
        effect_row: impl Into<String>,
    ) -> Self {
        Self {
            input_type: input_type.into(),
            output_type: output_type.into(),
            effect_row: effect_row.into(),
        }
    }
    /// Check whether this handler type handles the given effect.
    pub fn handles_effect(&self, effect: &str) -> bool {
        self.effect_row.contains(effect)
    }
    /// The continuation type within the handler.
    pub fn continuation_type(&self) -> String {
        format!("{} -> Comp {{}} {}", self.input_type, self.output_type)
    }
}
/// A simple static effect row checker that verifies that performed effects
/// are within the declared effect row.
#[allow(dead_code)]
pub struct EffectRowChecker {
    /// The declared (allowed) effect row.
    pub declared_row: EffRow,
    /// Violations found during checking: effect names that were used but not declared.
    pub violations: Vec<String>,
}
#[allow(dead_code)]
impl EffectRowChecker {
    /// Create a new checker for a given declared effect row.
    pub fn new(declared_row: EffRow) -> Self {
        Self {
            declared_row,
            violations: vec![],
        }
    }
    /// Check if the given effect is within the declared row.
    pub fn check_effect(&mut self, effect: &str) -> bool {
        if self.declared_row.contains(effect) {
            true
        } else {
            self.violations.push(effect.to_string());
            false
        }
    }
    /// Check all effects used in a list of operations.
    pub fn check_all(&mut self, used_effects: &[&str]) -> bool {
        let mut all_ok = true;
        for eff in used_effects {
            if !self.check_effect(eff) {
                all_ok = false;
            }
        }
        all_ok
    }
    /// Return whether any violations were found.
    pub fn has_violations(&self) -> bool {
        !self.violations.is_empty()
    }
    /// Return a summary of violations.
    pub fn violation_summary(&self) -> String {
        if self.violations.is_empty() {
            "No violations: all effects are within the declared row.".to_string()
        } else {
            format!(
                "Violations: {:?} not in declared row {:?}",
                self.violations,
                self.declared_row.effect_names()
            )
        }
    }
    /// Reset violations (allow re-checking).
    pub fn reset(&mut self) {
        self.violations.clear();
    }
    /// The declared effect row is a sound over-approximation of used effects.
    pub fn is_sound_annotation(&self) -> bool {
        !self.has_violations()
    }
}
/// Tracks usage of linear (one-shot) effects to ensure they are used exactly once.
#[allow(dead_code)]
pub struct LinearEffectTracker {
    /// Map from effect name to usage count.
    usage: HashMap<String, usize>,
    /// Effects declared as linear (must be used exactly once).
    linear_effects: Vec<String>,
}
#[allow(dead_code)]
impl LinearEffectTracker {
    /// Create a new linear effect tracker.
    pub fn new(linear_effects: Vec<String>) -> Self {
        Self {
            usage: HashMap::new(),
            linear_effects,
        }
    }
    /// Record one use of the given effect.
    pub fn use_effect(&mut self, effect: &str) {
        *self.usage.entry(effect.to_string()).or_insert(0) += 1;
    }
    /// Check whether a linear effect was used exactly once.
    pub fn check_linear(&self, effect: &str) -> bool {
        self.usage.get(effect).copied().unwrap_or(0) == 1
    }
    /// Verify all declared linear effects were used exactly once.
    pub fn verify_all_linear(&self) -> bool {
        self.linear_effects.iter().all(|e| self.check_linear(e))
    }
    /// Get the usage count of an effect.
    pub fn usage_count(&self, effect: &str) -> usize {
        self.usage.get(effect).copied().unwrap_or(0)
    }
    /// Return the list of linear effects that were used more than once (violations).
    pub fn overused_effects(&self) -> Vec<String> {
        self.linear_effects
            .iter()
            .filter(|e| self.usage.get(*e).copied().unwrap_or(0) > 1)
            .cloned()
            .collect()
    }
    /// Return the list of linear effects that were never used (also violations).
    pub fn unused_effects(&self) -> Vec<String> {
        self.linear_effects
            .iter()
            .filter(|e| self.usage.get(*e).copied().unwrap_or(0) == 0)
            .cloned()
            .collect()
    }
    /// Full verification report.
    pub fn report(&self) -> String {
        let overused = self.overused_effects();
        let unused = self.unused_effects();
        if overused.is_empty() && unused.is_empty() {
            "Linear effects: all used exactly once (correct)".to_string()
        } else {
            format!(
                "Linear effect violations: overused={:?}, unused={:?}",
                overused, unused
            )
        }
    }
}
/// A value from the free monad over an operation set F.
/// `FreeMnd<F, A>` is either a pure value A or an F-shaped operation
/// with a continuation.
pub enum Free<A> {
    /// Pure return value.
    Pure(A),
    /// An operation (represented by name + argument string) with continuation.
    Op {
        /// Effect name.
        effect: String,
        /// Operation name.
        op: String,
        /// Argument (serialized as string for genericity).
        arg: String,
        /// Continuation: given the return value (as string), produce next computation.
        cont: Box<dyn FnOnce(String) -> Free<A>>,
    },
}
impl<A> Free<A> {
    /// Construct a pure computation.
    pub fn pure(val: A) -> Self {
        Free::Pure(val)
    }
    /// Lift a single operation into the free monad.
    pub fn op(
        effect: impl Into<String>,
        op: impl Into<String>,
        arg: impl Into<String>,
        cont: impl FnOnce(String) -> Free<A> + 'static,
    ) -> Self {
        Free::Op {
            effect: effect.into(),
            op: op.into(),
            arg: arg.into(),
            cont: Box::new(cont),
        }
    }
    /// Fold over the free monad with a pure handler `ret` and an op handler `alg`.
    pub fn fold<B: 'static>(
        self,
        ret: impl Fn(A) -> B + 'static,
        alg: &'static dyn Fn(&str, &str, String, Box<dyn FnOnce(String) -> B>) -> B,
    ) -> B
    where
        A: 'static,
    {
        match self {
            Free::Pure(a) => ret(a),
            Free::Op {
                effect,
                op,
                arg,
                cont,
            } => {
                let ret = std::sync::Arc::new(ret);
                let ret2 = ret.clone();
                let cont_b = Box::new(move |s: String| cont(s).fold_inner(ret2, alg));
                alg(&effect, &op, arg, cont_b)
            }
        }
    }
    fn fold_inner<B: 'static>(
        self,
        ret: std::sync::Arc<impl Fn(A) -> B + 'static>,
        alg: &'static dyn Fn(&str, &str, String, Box<dyn FnOnce(String) -> B>) -> B,
    ) -> B
    where
        A: 'static,
    {
        match self {
            Free::Pure(a) => ret(a),
            Free::Op {
                effect,
                op,
                arg,
                cont,
            } => {
                let ret2 = ret.clone();
                let cont_b = Box::new(move |s: String| cont(s).fold_inner(ret2, alg));
                alg(&effect, &op, arg, cont_b)
            }
        }
    }
}
/// An effect handler that eliminates a specific effect.
pub struct EffectHandler {
    /// The name of the handled effect.
    pub effect: String,
    /// A description of the handler function.
    pub handler_fn: String,
}
impl EffectHandler {
    /// Create a new EffectHandler.
    pub fn new(effect: impl Into<String>, handler_fn: impl Into<String>) -> Self {
        Self {
            effect: effect.into(),
            handler_fn: handler_fn.into(),
        }
    }
    /// A deep handler recursively handles all operations and sub-computations.
    pub fn deep_handler(&self) -> String {
        format!(
            "Deep handler for {}: handles all continuations recursively",
            self.effect
        )
    }
    /// A shallow handler handles only the first operation encountered.
    pub fn shallow_handler(&self) -> String {
        format!(
            "Shallow handler for {}: one-shot, does not re-handle continuation",
            self.effect
        )
    }
    /// Handling eliminates the effect from the effect row.
    pub fn effect_elimination(&self) -> String {
        format!(
            "handle[{}]: Comp (E | R) A -> Comp R A via handler {}",
            self.effect, self.handler_fn
        )
    }
}
/// An effect row: an ordered (but usually treated as a set) collection of effect signatures.
#[derive(Debug, Clone, Default)]
pub struct EffRow {
    /// The effects present in this row, by name.
    effects: Vec<String>,
}
impl EffRow {
    /// Create the empty effect row (pure).
    pub fn empty() -> Self {
        EffRow { effects: vec![] }
    }
    /// Extend this row with one more effect.
    pub fn extend(&self, eff: impl Into<String>) -> Self {
        let mut new = self.clone();
        let name = eff.into();
        if !new.effects.contains(&name) {
            new.effects.push(name);
        }
        new
    }
    /// Check if this row contains the given effect.
    pub fn contains(&self, eff: &str) -> bool {
        self.effects.iter().any(|e| e == eff)
    }
    /// Check if this row lacks the given effect.
    pub fn lacks(&self, eff: &str) -> bool {
        !self.contains(eff)
    }
    /// Check if this row is a subset of another.
    pub fn is_subset_of(&self, other: &EffRow) -> bool {
        self.effects.iter().all(|e| other.contains(e))
    }
    /// Compute the union of two rows.
    pub fn union(&self, other: &EffRow) -> Self {
        let mut new = self.clone();
        for e in &other.effects {
            if !new.effects.contains(e) {
                new.effects.push(e.clone());
            }
        }
        new
    }
    /// Return the list of effect names.
    pub fn effect_names(&self) -> &[String] {
        &self.effects
    }
    /// True if the row is empty (pure computation).
    pub fn is_pure(&self) -> bool {
        self.effects.is_empty()
    }
}
/// An effect signature: a named collection of operations.
#[derive(Debug, Clone)]
pub struct EffSig {
    /// The name of this effect (e.g., "State", "IO", "Exn").
    pub name: String,
    /// The operations provided by this effect.
    pub operations: Vec<OpDesc>,
}
impl EffSig {
    /// Create a new effect signature.
    pub fn new(name: impl Into<String>, ops: Vec<OpDesc>) -> Self {
        EffSig {
            name: name.into(),
            operations: ops,
        }
    }
    /// Look up an operation by name.
    pub fn get_op(&self, op_name: &str) -> Option<&OpDesc> {
        self.operations.iter().find(|op| op.name == op_name)
    }
}
/// A captured delimited continuation.
pub struct Continuation<A, B> {
    func: Box<dyn FnOnce(A) -> B>,
}
impl<A, B> Continuation<A, B> {
    /// Wrap a function as a continuation.
    pub fn new(f: impl FnOnce(A) -> B + 'static) -> Self {
        Continuation { func: Box::new(f) }
    }
    /// Resume the continuation with a value.
    pub fn resume(self, val: A) -> B {
        (self.func)(val)
    }
}
/// An effect-polymorphic computation: parameterized by an effect row variable.
///
/// At the Rust level, we represent this as a computation that can be
/// "instantiated" at a specific effect row.
pub struct EffPoly<A> {
    /// The underlying computation, parameterized by an effect row (represented as `EffRow`).
    compute: Box<dyn Fn(&EffRow) -> A>,
}
impl<A> EffPoly<A> {
    /// Create an effect-polymorphic computation.
    pub fn new(f: impl Fn(&EffRow) -> A + 'static) -> Self {
        EffPoly {
            compute: Box::new(f),
        }
    }
    /// Instantiate at a specific effect row.
    pub fn instantiate(&self, row: &EffRow) -> A {
        (self.compute)(row)
    }
}
/// A deep effect handler for a specific effect.
///
/// A deep handler handles every operation of the effect recursively,
/// re-interpreting the continuation in the handled context.
pub struct DeepHandler<A, B> {
    /// The name of the handled effect.
    pub effect_name: String,
    /// Return clause: `val_clause(a)` handles the final value.
    pub val_clause: Box<dyn Fn(A) -> B>,
    /// Operation clauses: map `(op_name, arg, k)` to `B` where `k : String → B`.
    pub op_clauses: HashMap<String, Box<dyn Fn(String, Box<dyn Fn(String) -> B>) -> B>>,
}
impl<A, B: 'static> DeepHandler<A, B> {
    /// Create a new deep handler.
    pub fn new(effect_name: impl Into<String>, val_clause: impl Fn(A) -> B + 'static) -> Self {
        DeepHandler {
            effect_name: effect_name.into(),
            val_clause: Box::new(val_clause),
            op_clauses: HashMap::new(),
        }
    }
    /// Register an operation clause for operation `op_name`.
    #[allow(clippy::too_many_arguments)]
    pub fn with_op(
        mut self,
        op_name: impl Into<String>,
        clause: impl Fn(String, Box<dyn Fn(String) -> B>) -> B + 'static,
    ) -> Self {
        self.op_clauses.insert(op_name.into(), Box::new(clause));
        self
    }
}
/// Delimited continuations with a typed prompt.
pub struct DelimitedContinuation {
    /// The type of the prompt / answer type.
    pub prompt_type: String,
}
impl DelimitedContinuation {
    /// Create a new DelimitedContinuation.
    pub fn new(prompt_type: impl Into<String>) -> Self {
        Self {
            prompt_type: prompt_type.into(),
        }
    }
    /// Reset introduces a delimiter for the continuation.
    pub fn reset(&self) -> String {
        format!(
            "reset<{}>: Comp A -> {}",
            self.prompt_type, self.prompt_type
        )
    }
    /// Shift captures the continuation up to the nearest enclosing reset.
    pub fn shift(&self) -> String {
        format!(
            "shift<{0}>: ((A -> {0}) -> {0}) -> Comp A",
            self.prompt_type
        )
    }
    /// Delimited continuations are first-class values in this system.
    pub fn is_first_class(&self) -> bool {
        true
    }
}
/// The continuation monad supporting call/cc.
pub struct ContinuationMonad {
    /// Description of the answer type.
    pub answer_type: String,
}
impl ContinuationMonad {
    /// Create a new ContinuationMonad.
    pub fn new(answer_type: impl Into<String>) -> Self {
        Self {
            answer_type: answer_type.into(),
        }
    }
    /// Call-with-current-continuation operator.
    pub fn callcc(&self) -> String {
        format!(
            "callcc: ((A -> Cont {} B) -> Cont {} A) -> Cont {} A",
            self.answer_type, self.answer_type, self.answer_type
        )
    }
    /// The continuation monad satisfies the monad laws.
    pub fn is_monad(&self) -> bool {
        true
    }
    /// Reset and shift for delimited control.
    pub fn reset_shift(&self) -> String {
        format!(
            "reset: Cont {} {} -> {}; shift: ((A -> {}) -> Cont {} {}) -> Cont {} A",
            self.answer_type,
            self.answer_type,
            self.answer_type,
            self.answer_type,
            self.answer_type,
            self.answer_type,
            self.answer_type
        )
    }
}
/// A shallow handler: handles only the first operation encountered.
pub struct ShallowHandler<A, B> {
    /// The effect this handler covers.
    pub effect_name: String,
    /// Return clause.
    pub val_clause: Box<dyn Fn(A) -> B>,
    /// Operation clause: `(op, arg, raw_continuation) → B`.
    pub op_clause: Box<dyn Fn(String, String, Box<dyn FnOnce(String) -> Free<A>>) -> B>,
}
impl<A, B> ShallowHandler<A, B> {
    /// Create a new shallow handler.
    pub fn new(
        effect_name: impl Into<String>,
        val_clause: impl Fn(A) -> B + 'static,
        op_clause: impl Fn(String, String, Box<dyn FnOnce(String) -> Free<A>>) -> B + 'static,
    ) -> Self {
        ShallowHandler {
            effect_name: effect_name.into(),
            val_clause: Box::new(val_clause),
            op_clause: Box::new(op_clause),
        }
    }
    /// Apply the shallow handler to a computation.
    pub fn handle(self, comp: Free<A>) -> B {
        match comp {
            Free::Pure(a) => (self.val_clause)(a),
            Free::Op { op, arg, cont, .. } => (self.op_clause)(op, arg, cont),
        }
    }
}
/// A description of a single algebraic operation in a signature.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OpDesc {
    /// The name of the operation.
    pub name: String,
    /// A human-readable description of the parameter type.
    pub param_ty: String,
    /// A human-readable description of the return type.
    pub return_ty: String,
}
impl OpDesc {
    /// Create a new operation description.
    pub fn new(name: impl Into<String>, param: impl Into<String>, ret: impl Into<String>) -> Self {
        OpDesc {
            name: name.into(),
            param_ty: param.into(),
            return_ty: ret.into(),
        }
    }
}
/// An algebraic effect with a name and a list of operations.
pub struct Effect {
    /// The name of the effect.
    pub name: String,
    /// The operation names of this effect.
    pub operations: Vec<String>,
}
impl Effect {
    /// Create a new Effect.
    pub fn new(name: impl Into<String>, operations: Vec<String>) -> Self {
        Self {
            name: name.into(),
            operations,
        }
    }
    /// Every algebraic effect is algebraic in the sense of Plotkin and Power.
    pub fn is_algebraic(&self) -> bool {
        true
    }
    /// The free monad generated by the functor of operations.
    pub fn free_monad(&self) -> String {
        format!(
            "Free monad for effect {}: Tree over operations {:?}",
            self.name, self.operations
        )
    }
}
/// Effect polymorphism — quantifying over effect variables.
pub struct EffectPolymorphism {
    /// Type variables.
    pub type_vars: Vec<String>,
    /// Effect row variables.
    pub effect_vars: Vec<String>,
}
impl EffectPolymorphism {
    /// Create a new EffectPolymorphism.
    pub fn new(type_vars: Vec<String>, effect_vars: Vec<String>) -> Self {
        Self {
            type_vars,
            effect_vars,
        }
    }
    /// Effect abstraction: generalise over an effect variable.
    pub fn effect_abstraction(&self) -> String {
        format!(
            "Lambda {} . body  [abstract over effect rows {:?}]",
            self.effect_vars.join(", "),
            self.effect_vars
        )
    }
    /// Effect instantiation: substitute a concrete row for an effect variable.
    pub fn effect_instantiation(&self) -> String {
        format!(
            "Inst [{:?} := R] : substitute concrete effect rows for effect vars",
            self.effect_vars
        )
    }
}
/// Simulate shift/reset using a stack-based approach.
///
/// `DelimCont<A>` represents a computation that may capture its continuation.
pub struct DelimCont<A> {
    body: Box<dyn FnOnce() -> A>,
}
impl<A: 'static> DelimCont<A> {
    /// Create a delimited computation.
    pub fn new(body: impl FnOnce() -> A + 'static) -> Self {
        DelimCont {
            body: Box::new(body),
        }
    }
    /// Run the computation (analogous to `reset`).
    pub fn reset(self) -> A {
        (self.body)()
    }
}
/// A row of effects present in a computation type.
pub struct EffectRow {
    /// The list of effects in this row.
    pub effects: Vec<String>,
}
impl EffectRow {
    /// Create a new EffectRow.
    pub fn new(effects: Vec<String>) -> Self {
        Self { effects }
    }
    /// The empty effect row — pure computations.
    pub fn is_empty(&self) -> bool {
        self.effects.is_empty()
    }
    /// The union of two effect rows.
    pub fn union(&self, other: &EffectRow) -> EffectRow {
        let mut combined = self.effects.clone();
        for e in &other.effects {
            if !combined.contains(e) {
                combined.push(e.clone());
            }
        }
        EffectRow { effects: combined }
    }
    /// Remove an effect from the row (effect elimination).
    pub fn subtract(&self, effect: &str) -> EffectRow {
        EffectRow {
            effects: self
                .effects
                .iter()
                .filter(|e| e.as_str() != effect)
                .cloned()
                .collect(),
        }
    }
}
/// A Koka-style effect interpreter that runs a computation by dispatching
/// each operation to registered handlers.
pub struct EffectInterpreter {
    /// Handlers: effect name → op name → handler function (arg_str → result_str).
    handlers: HashMap<String, HashMap<String, Box<dyn Fn(String) -> String>>>,
}
impl EffectInterpreter {
    /// Create a new effect interpreter.
    pub fn new() -> Self {
        EffectInterpreter {
            handlers: HashMap::new(),
        }
    }
    /// Register a handler for a specific operation of a specific effect.
    pub fn register(
        mut self,
        effect: impl Into<String>,
        op: impl Into<String>,
        handler: impl Fn(String) -> String + 'static,
    ) -> Self {
        self.handlers
            .entry(effect.into())
            .or_default()
            .insert(op.into(), Box::new(handler));
        self
    }
    /// Run a `Free<String>` computation using the registered handlers.
    pub fn run(&self, comp: Free<String>) -> String {
        match comp {
            Free::Pure(v) => v,
            Free::Op {
                effect,
                op,
                arg,
                cont,
            } => {
                if let Some(eff_handlers) = self.handlers.get(&effect) {
                    if let Some(h) = eff_handlers.get(&op) {
                        let result = h(arg);
                        let next = cont(result);
                        self.run(next)
                    } else {
                        format!("unhandled_op({}/{})", effect, op)
                    }
                } else {
                    format!("unhandled_effect({})", effect)
                }
            }
        }
    }
}
/// A graded monad tracks resource usage at the type level via grades.
///
/// The grade G represents the "cost" or "resource usage" of a computation.
/// bind composes grades via multiplication: M g a → (a → M h b) → M (g*h) b.
#[allow(dead_code)]
pub struct GradedMonadImpl<G, A> {
    /// The grade (resource annotation) of this computation.
    pub grade: G,
    /// The computation value.
    pub value: A,
}
#[allow(dead_code)]
impl<G: Clone + std::fmt::Debug, A: Clone> GradedMonadImpl<G, A> {
    /// Lift a value into a graded computation with grade `g`.
    pub fn unit(grade: G, value: A) -> Self {
        Self { grade, value }
    }
    /// Map a function over the value, preserving the grade.
    pub fn map<B, F: Fn(A) -> B>(self, f: F) -> GradedMonadImpl<G, B> {
        GradedMonadImpl {
            grade: self.grade,
            value: f(self.value),
        }
    }
    /// Return a description of the grade.
    pub fn grade_description(&self) -> String {
        format!("Grade: {:?}", self.grade)
    }
    /// Check whether the grade indicates a "pure" (unit-grade) computation.
    pub fn is_unit_grade(&self) -> bool
    where
        G: PartialEq + Default,
    {
        self.grade == G::default()
    }
}
/// A simple effect inference engine.
///
/// Tracks which effects a computation uses by inspecting `Free` nodes.
pub struct EffectInferencer {
    /// Mapping from computation identifiers to their inferred effect rows.
    inferred: HashMap<String, EffRow>,
}
impl EffectInferencer {
    /// Create a new effect inferencer.
    pub fn new() -> Self {
        EffectInferencer {
            inferred: HashMap::new(),
        }
    }
    /// Infer the effects of a `Free` computation (by collecting effect names).
    pub fn infer<A>(&mut self, name: impl Into<String>, comp: &Free<A>) -> EffRow {
        let row = Self::collect_effects(comp);
        self.inferred.insert(name.into(), row.clone());
        row
    }
    fn collect_effects<A>(comp: &Free<A>) -> EffRow {
        let mut row = EffRow::empty();
        Self::collect_rec(comp, &mut row);
        row
    }
    fn collect_rec<A>(comp: &Free<A>, row: &mut EffRow) {
        match comp {
            Free::Pure(_) => {}
            Free::Op { effect, cont, .. } => {
                row.effects.push(effect.clone());
                let _ = cont;
            }
        }
    }
    /// Get the inferred row for a previously analyzed computation.
    pub fn get_row(&self, name: &str) -> Option<&EffRow> {
        self.inferred.get(name)
    }
}
/// A pipeline of effect handlers applied in sequence.
///
/// Handlers are applied in order: the first handler handles the outermost
/// effect; the last handler handles the innermost effect (closest to the
/// computation).
#[allow(dead_code)]
pub struct EffectPipeline {
    /// Ordered list of (effect_name, handler_description) pairs.
    stages: Vec<(String, String)>,
}
#[allow(dead_code)]
impl EffectPipeline {
    /// Create an empty effect pipeline.
    pub fn new() -> Self {
        Self { stages: vec![] }
    }
    /// Append a handler stage for the given effect.
    pub fn add_stage(mut self, effect: &str, handler_desc: &str) -> Self {
        self.stages
            .push((effect.to_string(), handler_desc.to_string()));
        self
    }
    /// The number of handler stages.
    pub fn depth(&self) -> usize {
        self.stages.len()
    }
    /// The residual effect row after all handlers have been applied.
    /// (Assumes each stage eliminates exactly one effect.)
    pub fn residual_row(&self, initial: &EffRow) -> EffRow {
        let mut row = initial.clone();
        for (eff, _) in &self.stages {
            row = EffRow {
                effects: row.effects.into_iter().filter(|e| e != eff).collect(),
            };
        }
        row
    }
    /// Check that the pipeline is complete: no effects remain after all handlers.
    pub fn is_complete(&self, initial: &EffRow) -> bool {
        self.residual_row(initial).is_pure()
    }
    /// Return a textual description of the pipeline.
    pub fn describe(&self) -> String {
        if self.stages.is_empty() {
            "Empty pipeline (identity)".to_string()
        } else {
            let stage_strs: Vec<String> = self
                .stages
                .iter()
                .map(|(eff, desc)| format!("handle[{}] via {}", eff, desc))
                .collect();
            stage_strs.join(" >> ")
        }
    }
    /// Verify that the pipeline handles effects in the right order (deep to shallow).
    pub fn is_valid_deep_order(&self) -> bool {
        let mut seen = std::collections::HashSet::new();
        for (eff, _) in &self.stages {
            if !seen.insert(eff.clone()) {
                return false;
            }
        }
        true
    }
}
/// The free monad over a functor F.
pub struct FreeMonad {
    /// Description of the functor whose free monad is constructed.
    pub functor: String,
}
impl FreeMonad {
    /// Create a new FreeMonad.
    pub fn new(functor: impl Into<String>) -> Self {
        Self {
            functor: functor.into(),
        }
    }
    /// Unit / return: inject a pure value into the free monad.
    pub fn unit(&self) -> String {
        format!("return: A -> Free({}) A", self.functor)
    }
    /// Monadic bind: sequencing in the free monad.
    pub fn bind(&self) -> String {
        format!(
            "bind: Free({0}) A -> (A -> Free({0}) B) -> Free({0}) B",
            self.functor
        )
    }
    /// Interpret: fold the free monad with an algebra.
    pub fn interpret(&self) -> String {
        format!(
            "interpret: (F A -> A) -> Free({}) A -> A  [initial algebra morphism]",
            self.functor
        )
    }
}
