//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;

use oxilean_kernel::*;
use std::collections::HashMap;

/// The result of reflecting an expression back into the meta level.
#[derive(Debug, Clone)]
pub struct ReflectedTerm {
    /// The reflected quoted expression.
    pub quoted: QuotedExpr,
    /// The type of the reflected term (as a string).
    pub ty: String,
    /// Whether reflection was exact (no approximation).
    pub exact: bool,
}
impl ReflectedTerm {
    /// Create a new reflected term.
    pub fn new(quoted: QuotedExpr, ty: impl Into<String>) -> Self {
        Self {
            quoted,
            ty: ty.into(),
            exact: true,
        }
    }
    /// Create an approximate reflected term.
    pub fn approximate(quoted: QuotedExpr, ty: impl Into<String>) -> Self {
        Self {
            quoted,
            ty: ty.into(),
            exact: false,
        }
    }
}
/// A registry of user-defined meta-level definitions.
#[derive(Debug, Default)]
pub struct MetaProgRegistry {
    defs: std::collections::HashMap<String, MetaDefinition>,
}
impl MetaProgRegistry {
    /// Create an empty registry.
    pub fn new() -> Self {
        Self::default()
    }
    /// Register a new meta-level definition.
    pub fn register(&mut self, def: MetaDefinition) {
        self.defs.insert(def.name.clone(), def);
    }
    /// Look up a definition by name.
    pub fn lookup(&self, name: &str) -> Option<&MetaDefinition> {
        self.defs.get(name)
    }
    /// Return the number of registered definitions.
    pub fn len(&self) -> usize {
        self.defs.len()
    }
    /// Return true if no definitions are registered.
    pub fn is_empty(&self) -> bool {
        self.defs.is_empty()
    }
    /// Return all registered definition names.
    pub fn names(&self) -> Vec<&str> {
        let mut names: Vec<&str> = self.defs.keys().map(|s| s.as_str()).collect();
        names.sort();
        names
    }
    /// Remove a definition by name.
    pub fn remove(&mut self, name: &str) -> Option<MetaDefinition> {
        self.defs.remove(name)
    }
}
/// A built-in meta tactic that implements the `ring` decision procedure stub.
pub struct RingMetaTactic;
/// A built-in meta tactic that implements `omega` (linear arithmetic stub).
pub struct OmegaMetaTactic;
/// A built-in meta tactic that implements `simp` (simplification stub).
pub struct SimpMetaTactic {
    lemmas: Vec<String>,
}
impl SimpMetaTactic {
    /// Create a simp meta-tactic with the given lemma set.
    pub fn new(lemmas: Vec<String>) -> Self {
        Self { lemmas }
    }
    /// Create a simp meta-tactic with no lemmas.
    pub fn empty() -> Self {
        Self { lemmas: Vec::new() }
    }
    /// Return the registered lemmas.
    pub fn lemmas(&self) -> &[String] {
        &self.lemmas
    }
}
/// A pipeline of meta-elaboration steps.
#[derive(Default)]
pub struct MetaElabPipeline {
    steps: Vec<Box<dyn MetaElabStep>>,
}
impl MetaElabPipeline {
    /// Create an empty pipeline.
    pub fn new() -> Self {
        Self::default()
    }
    /// Add a step to the pipeline.
    pub fn add<S: MetaElabStep + 'static>(&mut self, step: S) {
        self.steps.push(Box::new(step));
    }
    /// Run all steps in sequence.
    pub fn run(&self, expr: QuotedExpr, env: &mut MetaEnv) -> MetaResult<QuotedExpr> {
        let mut current = expr;
        for step in &self.steps {
            current = step.apply(current, env)?;
        }
        Ok(current)
    }
    /// Return the number of steps.
    pub fn len(&self) -> usize {
        self.steps.len()
    }
    /// Return true if there are no steps.
    pub fn is_empty(&self) -> bool {
        self.steps.is_empty()
    }
    /// Return step names.
    pub fn step_names(&self) -> Vec<&str> {
        self.steps.iter().map(|s| s.step_name()).collect()
    }
}
/// Example built-in user tactic: decides goals of the form `True`.
pub struct TrueDecider;
/// A user-defined notation macro rule.
#[derive(Debug, Clone)]
pub struct MacroRule {
    /// The name of this macro rule.
    pub name: String,
    /// The pattern to match against.
    pub pattern: Vec<MacroToken>,
    /// The expansion template (uses `{var}` placeholders).
    pub expansion: String,
    /// Priority for rule ordering (higher = tried first).
    pub priority: i32,
}
/// Context for elaborating metaprogramming expressions.
#[derive(Debug, Default)]
pub struct MetaElabContext {
    /// Metaprogramming environment.
    pub env: MetaEnv,
    /// Evaluation stack.
    pub stack: MetaStack,
    /// Maximum allowed depth for macro expansion.
    pub max_depth: usize,
    /// Current expansion depth.
    pub current_depth: usize,
}
impl MetaElabContext {
    /// Create a new meta elaboration context.
    pub fn new() -> Self {
        Self {
            env: MetaEnv::new(),
            stack: MetaStack::new(),
            max_depth: 64,
            current_depth: 0,
        }
    }
    /// Return true if the maximum expansion depth has been exceeded.
    pub fn depth_exceeded(&self) -> bool {
        self.current_depth > self.max_depth
    }
    /// Enter a new quotation context.
    pub fn enter_quotation(&mut self, label: Option<&str>) {
        let depth = self.stack.depth();
        let mut frame = MetaFrame::new(MetaFrameKind::Quotation, depth);
        if let Some(lbl) = label {
            frame = frame.with_label(lbl);
        }
        self.stack.push(frame);
        self.current_depth += 1;
    }
    /// Exit a quotation context.
    pub fn exit_quotation(&mut self) -> Option<MetaFrame> {
        let frame = self.stack.pop();
        if self.current_depth > 0 {
            self.current_depth -= 1;
        }
        frame
    }
    /// Enter a splice context.
    pub fn enter_splice(&mut self) {
        let depth = self.stack.depth();
        self.stack
            .push(MetaFrame::new(MetaFrameKind::Splice, depth));
        self.current_depth += 1;
    }
    /// Exit a splice context.
    pub fn exit_splice(&mut self) -> Option<MetaFrame> {
        let frame = self.stack.pop();
        if self.current_depth > 0 {
            self.current_depth -= 1;
        }
        frame
    }
    /// Bind a variable in the current meta environment.
    pub fn bind(&mut self, name: impl Into<String>, val: impl Into<String>) {
        self.env.bind(name, val);
    }
    /// Look up a variable.
    pub fn lookup(&self, name: &str) -> Option<&str> {
        self.env.lookup(name)
    }
}
/// A frame in the metaprogramming evaluation stack.
#[derive(Debug, Clone)]
pub struct MetaFrame {
    /// Kind of this frame.
    pub kind: MetaFrameKind,
    /// Depth at which this frame was pushed.
    pub depth: usize,
    /// Optional label for diagnostics.
    pub label: Option<String>,
}
impl MetaFrame {
    /// Create a new frame.
    pub fn new(kind: MetaFrameKind, depth: usize) -> Self {
        Self {
            kind,
            depth,
            label: None,
        }
    }
    /// Create a labelled frame.
    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }
    /// Return true if this is a quotation frame.
    pub fn is_quotation(&self) -> bool {
        self.kind == MetaFrameKind::Quotation
    }
    /// Return true if this is a splice frame.
    pub fn is_splice(&self) -> bool {
        self.kind == MetaFrameKind::Splice
    }
}
/// Tracks the nesting level of quotation and splice contexts.
#[derive(Debug, Clone, Default)]
pub struct SpliceContext {
    /// Current quotation depth (0 = object level).
    pub quote_depth: u32,
    /// Current splice depth.
    pub splice_depth: u32,
    /// History of depth changes for debugging.
    pub depth_log: Vec<(String, u32)>,
}
impl SpliceContext {
    /// Create a new splice context at the object level.
    pub fn new() -> Self {
        Self::default()
    }
    /// Enter a quotation level.
    pub fn enter_quote(&mut self) {
        self.quote_depth += 1;
        self.depth_log
            .push(("enter_quote".to_string(), self.quote_depth));
    }
    /// Exit a quotation level.
    pub fn exit_quote(&mut self) {
        if self.quote_depth > 0 {
            self.quote_depth -= 1;
        }
        self.depth_log
            .push(("exit_quote".to_string(), self.quote_depth));
    }
    /// Enter a splice level.
    pub fn enter_splice(&mut self) {
        self.splice_depth += 1;
        self.depth_log
            .push(("enter_splice".to_string(), self.splice_depth));
    }
    /// Exit a splice level.
    pub fn exit_splice(&mut self) {
        if self.splice_depth > 0 {
            self.splice_depth -= 1;
        }
        self.depth_log
            .push(("exit_splice".to_string(), self.splice_depth));
    }
    /// Return true if we are currently inside a quotation.
    pub fn in_quotation(&self) -> bool {
        self.quote_depth > 0
    }
    /// Return true if we are at the object level (no quotation).
    pub fn at_object_level(&self) -> bool {
        self.quote_depth == 0
    }
    /// Return the effective nesting level (quote - splice).
    pub fn effective_level(&self) -> i32 {
        self.quote_depth as i32 - self.splice_depth as i32
    }
    /// Clear the depth log.
    pub fn clear_log(&mut self) {
        self.depth_log.clear();
    }
}
/// The kind of a MetaFrame.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MetaFrameKind {
    /// A quotation context (inside `quote { ... }`).
    Quotation,
    /// A splice context (inside `splice { ... }`).
    Splice,
    /// A tactic evaluation context.
    TacticEval,
    /// A macro expansion context.
    MacroExpansion(String),
}
/// Expands user-defined macros given a set of registered rules.
pub struct MacroEngine {
    rules: Vec<MacroRule>,
}
impl MacroEngine {
    /// Create an empty macro engine.
    pub fn new() -> Self {
        MacroEngine { rules: Vec::new() }
    }
    /// Add a macro rule to the engine.
    pub fn add_rule(&mut self, rule: MacroRule) {
        self.rules.push(rule);
        self.rules.sort_by(|a, b| b.priority.cmp(&a.priority));
    }
    /// Try to expand `input` using registered macro rules.
    ///
    /// Returns `Some(expanded)` if a rule matches, `None` otherwise.
    pub fn try_expand(&self, input: &str) -> Option<String> {
        for rule in &self.rules {
            if let Some(expanded) = self.apply_rule(rule, input) {
                return Some(expanded);
            }
        }
        None
    }
    /// Return all registered macro rules.
    pub fn rules(&self) -> &[MacroRule] {
        &self.rules
    }
    /// Attempt to apply a single macro rule to the input.
    fn apply_rule(&self, rule: &MacroRule, input: &str) -> Option<String> {
        let mut bindings: std::collections::HashMap<String, String> =
            std::collections::HashMap::new();
        let input_trimmed = input.trim();
        let mut remaining = input_trimmed;
        let tokens = &rule.pattern;
        for (idx, token) in tokens.iter().enumerate() {
            match token {
                MacroToken::Literal(lit) => {
                    remaining = remaining.strip_prefix(lit.as_str())?.trim();
                }
                MacroToken::Var(var_name) => {
                    let next_literal = tokens[idx + 1..].iter().find_map(|t| {
                        if let MacroToken::Literal(s) = t {
                            Some(s.as_str())
                        } else {
                            None
                        }
                    });
                    let (captured, rest) = if let Some(next_lit) = next_literal {
                        let pos = remaining.find(next_lit)?;
                        (&remaining[..pos], &remaining[pos..])
                    } else {
                        (remaining, "")
                    };
                    bindings.insert(var_name.clone(), captured.trim().to_string());
                    remaining = rest;
                }
                MacroToken::Many(var_name) => {
                    bindings.insert(var_name.clone(), remaining.trim().to_string());
                    remaining = "";
                }
            }
        }
        if !remaining.is_empty() {
            return None;
        }
        let mut result = rule.expansion.clone();
        for (var, val) in &bindings {
            let placeholder = format!("{{{var}}}");
            result = result.replace(&placeholder, val);
        }
        Some(result)
    }
}
/// A user-defined meta-level definition.
#[derive(Debug, Clone)]
pub struct MetaDefinition {
    /// The name of the definition.
    pub name: String,
    /// Parameter names.
    pub params: Vec<String>,
    /// Body as a quoted expression.
    pub body: QuotedExpr,
    /// Whether this definition is recursive.
    pub is_recursive: bool,
    /// Optional documentation string.
    pub doc: Option<String>,
}
impl MetaDefinition {
    /// Create a new meta definition.
    pub fn new(name: impl Into<String>, params: Vec<String>, body: QuotedExpr) -> Self {
        Self {
            name: name.into(),
            params,
            body,
            is_recursive: false,
            doc: None,
        }
    }
    /// Mark this definition as recursive.
    pub fn recursive(mut self) -> Self {
        self.is_recursive = true;
        self
    }
    /// Set the documentation string.
    pub fn with_doc(mut self, doc: impl Into<String>) -> Self {
        self.doc = Some(doc.into());
        self
    }
    /// Apply this definition to a list of argument expressions.
    ///
    /// Substitutes the parameters with the arguments in the body.
    pub fn apply(&self, args: &[QuotedExpr]) -> Option<QuotedExpr> {
        if args.len() != self.params.len() {
            return None;
        }
        let mut result = self.body.clone();
        for (param, arg) in self.params.iter().zip(args.iter()) {
            result = subst_quoted(&result, param, arg);
        }
        Some(result)
    }
}
/// The result of running a user-defined tactic.
#[derive(Debug, Clone)]
pub enum UserTacticResult {
    /// The tactic solved the goal.
    Solved,
    /// The tactic failed with a reason message.
    Failed(String),
    /// The tactic produced sub-goals (as target strings).
    Subgoals(Vec<String>),
}
/// Registry for user-defined tactics.
pub struct UserTacticRegistry {
    tactics: Vec<Box<dyn UserTactic>>,
}
impl UserTacticRegistry {
    /// Create an empty registry.
    pub fn new() -> Self {
        UserTacticRegistry {
            tactics: Vec::new(),
        }
    }
    /// Register a new user-defined tactic.
    pub fn register(&mut self, tactic: Box<dyn UserTactic>) {
        self.tactics.push(tactic);
    }
    /// Find a tactic by name.
    pub fn find(&self, name: &str) -> Option<&dyn UserTactic> {
        self.tactics
            .iter()
            .find(|t| t.name() == name)
            .map(|t| t.as_ref())
    }
    /// Return the names of all registered tactics.
    pub fn names(&self) -> Vec<&str> {
        self.tactics.iter().map(|t| t.name()).collect()
    }
    /// Return the number of registered tactics.
    pub fn len(&self) -> usize {
        self.tactics.len()
    }
    /// Return true if no tactics are registered.
    pub fn is_empty(&self) -> bool {
        self.tactics.is_empty()
    }
}
/// A single token in a macro pattern.
#[derive(Debug, Clone)]
pub enum MacroToken {
    /// A literal string that must appear verbatim.
    Literal(String),
    /// A named variable that captures a single expression.
    Var(String),
    /// A named variable that captures zero or more remaining tokens.
    Many(String),
}
/// A metaprogramming environment that associates variable names with
/// quoted expression strings.
#[derive(Debug, Clone, Default)]
pub struct MetaEnv {
    bindings: std::collections::HashMap<String, String>,
    parent: Option<Box<MetaEnv>>,
}
impl MetaEnv {
    /// Create a new empty MetaEnv.
    pub fn new() -> Self {
        Self::default()
    }
    /// Create a child MetaEnv with the given parent.
    pub fn child(parent: MetaEnv) -> Self {
        Self {
            bindings: std::collections::HashMap::new(),
            parent: Some(Box::new(parent)),
        }
    }
    /// Bind a name to a quoted expression string.
    pub fn bind(&mut self, name: impl Into<String>, expr: impl Into<String>) {
        self.bindings.insert(name.into(), expr.into());
    }
    /// Look up a name in this environment and its parents.
    pub fn lookup(&self, name: &str) -> Option<&str> {
        if let Some(val) = self.bindings.get(name) {
            return Some(val.as_str());
        }
        if let Some(parent) = &self.parent {
            return parent.lookup(name);
        }
        None
    }
    /// Return all names bound in this environment (not including parent).
    pub fn local_names(&self) -> Vec<&str> {
        let mut names: Vec<&str> = self.bindings.keys().map(|s| s.as_str()).collect();
        names.sort();
        names
    }
    /// Return the number of local bindings (not including parent).
    pub fn local_len(&self) -> usize {
        self.bindings.len()
    }
    /// Return true if the environment has no local bindings.
    pub fn is_local_empty(&self) -> bool {
        self.bindings.is_empty()
    }
    /// Remove a binding.
    pub fn unbind(&mut self, name: &str) -> Option<String> {
        self.bindings.remove(name)
    }
}
/// A macro transformer that applies a sequence of rewrite rules to a `QuotedExpr`.
#[derive(Default)]
pub struct MacroTransformer {
    rules: Vec<(&'static str, QuotedTransform)>,
}
impl MacroTransformer {
    /// Create an empty transformer.
    pub fn new() -> Self {
        Self::default()
    }
    /// Add a named rewrite rule.
    pub fn add_rule(&mut self, name: &'static str, rule: QuotedTransform) {
        self.rules.push((name, rule));
    }
    /// Apply all rules to the given expression (top-down, first match).
    pub fn transform(&self, expr: &QuotedExpr) -> QuotedExpr {
        for (_name, rule) in &self.rules {
            if let Some(result) = rule(expr) {
                return result;
            }
        }
        match expr {
            QuotedExpr::Atom(_) => expr.clone(),
            QuotedExpr::Splice(inner) => QuotedExpr::Splice(Box::new(self.transform(inner))),
            QuotedExpr::App { func, args } => QuotedExpr::App {
                func: Box::new(self.transform(func)),
                args: args.iter().map(|a| self.transform(a)).collect(),
            },
            QuotedExpr::Lambda { binders, body } => QuotedExpr::Lambda {
                binders: binders.clone(),
                body: Box::new(self.transform(body)),
            },
            QuotedExpr::Let { name, value, body } => QuotedExpr::Let {
                name: name.clone(),
                value: Box::new(self.transform(value)),
                body: Box::new(self.transform(body)),
            },
        }
    }
    /// Return the number of rules.
    pub fn len(&self) -> usize {
        self.rules.len()
    }
    /// Return true if there are no rules.
    pub fn is_empty(&self) -> bool {
        self.rules.is_empty()
    }
    /// Return rule names.
    pub fn rule_names(&self) -> Vec<&str> {
        self.rules.iter().map(|(n, _)| *n).collect()
    }
}
/// Represents a quoted expression in the metaprogramming system.
#[derive(Debug, Clone)]
pub enum QuotedExpr {
    /// A quoted atom (variable, literal, keyword).
    Atom(String),
    /// A quoted application: (f args...).
    App {
        /// Function being applied (as a quoted expression).
        func: Box<QuotedExpr>,
        /// Arguments (as quoted expressions).
        args: Vec<QuotedExpr>,
    },
    /// A splice: evaluates the inner expression at meta-level and inserts result.
    Splice(Box<QuotedExpr>),
    /// A quoted lambda: `fun binders => body`.
    Lambda {
        /// Binder names.
        binders: Vec<String>,
        /// Body expression.
        body: Box<QuotedExpr>,
    },
    /// A quoted let binding.
    Let {
        /// Variable name.
        name: String,
        /// Bound value.
        value: Box<QuotedExpr>,
        /// Body.
        body: Box<QuotedExpr>,
    },
}
impl QuotedExpr {
    /// Create an atom quoted expression.
    pub fn atom(s: impl Into<String>) -> Self {
        QuotedExpr::Atom(s.into())
    }
    /// Create an application quoted expression.
    pub fn app(func: QuotedExpr, args: Vec<QuotedExpr>) -> Self {
        QuotedExpr::App {
            func: Box::new(func),
            args,
        }
    }
    /// Create a splice.
    pub fn splice(inner: QuotedExpr) -> Self {
        QuotedExpr::Splice(Box::new(inner))
    }
    /// Create a lambda.
    pub fn lambda(binders: Vec<String>, body: QuotedExpr) -> Self {
        QuotedExpr::Lambda {
            binders,
            body: Box::new(body),
        }
    }
    /// Create a let binding.
    pub fn let_binding(name: impl Into<String>, value: QuotedExpr, body: QuotedExpr) -> Self {
        QuotedExpr::Let {
            name: name.into(),
            value: Box::new(value),
            body: Box::new(body),
        }
    }
    /// Return true if this is an atom.
    pub fn is_atom(&self) -> bool {
        matches!(self, QuotedExpr::Atom(_))
    }
    /// Return true if this is an application.
    pub fn is_app(&self) -> bool {
        matches!(self, QuotedExpr::App { .. })
    }
    /// Return true if this contains any splice nodes.
    pub fn has_splice(&self) -> bool {
        match self {
            QuotedExpr::Atom(_) => false,
            QuotedExpr::Splice(_) => true,
            QuotedExpr::App { func, args } => {
                func.has_splice() || args.iter().any(|a| a.has_splice())
            }
            QuotedExpr::Lambda { body, .. } => body.has_splice(),
            QuotedExpr::Let { value, body, .. } => value.has_splice() || body.has_splice(),
        }
    }
    /// Stringify the quoted expression for diagnostics.
    pub fn to_debug_string(&self) -> String {
        match self {
            QuotedExpr::Atom(s) => s.clone(),
            QuotedExpr::Splice(inner) => format!("$({})", inner.to_debug_string()),
            QuotedExpr::App { func, args } => {
                let arg_strs: Vec<_> = args.iter().map(|a| a.to_debug_string()).collect();
                format!("({} {})", func.to_debug_string(), arg_strs.join(" "))
            }
            QuotedExpr::Lambda { binders, body } => {
                format!("(fun {} => {})", binders.join(" "), body.to_debug_string())
            }
            QuotedExpr::Let { name, value, body } => {
                format!(
                    "(let {} := {} in {})",
                    name,
                    value.to_debug_string(),
                    body.to_debug_string()
                )
            }
        }
    }
}
impl QuotedExpr {
    #[allow(non_snake_case)]
    #[allow(dead_code)]
    pub(crate) fn Int_zero_placeholder() -> Self {
        QuotedExpr::atom("0")
    }
}
/// An identity step that does nothing.
pub struct IdentityMetaStep;
/// The result of interpreting a meta-level expression.
#[derive(Debug, Clone)]
pub enum MetaValue {
    /// A quoted expression value.
    Quoted(QuotedExpr),
    /// A boolean value.
    Bool(bool),
    /// A string value.
    String(String),
    /// An integer value.
    Int(i64),
    /// A list of meta values.
    List(Vec<MetaValue>),
    /// Unit value.
    Unit,
    /// An error occurred during interpretation.
    Error(String),
}
impl MetaValue {
    /// Return true if this is a Unit value.
    pub fn is_unit(&self) -> bool {
        matches!(self, MetaValue::Unit)
    }
    /// Return true if this is an Error value.
    pub fn is_error(&self) -> bool {
        matches!(self, MetaValue::Error(_))
    }
    /// Return true if this is a quoted expression.
    pub fn is_quoted(&self) -> bool {
        matches!(self, MetaValue::Quoted(_))
    }
    /// Try to extract a quoted expression.
    pub fn as_quoted(&self) -> Option<&QuotedExpr> {
        match self {
            MetaValue::Quoted(q) => Some(q),
            _ => None,
        }
    }
    /// Try to extract a bool.
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            MetaValue::Bool(b) => Some(*b),
            _ => None,
        }
    }
    /// Try to extract a string.
    pub fn as_str(&self) -> Option<&str> {
        match self {
            MetaValue::String(s) => Some(s.as_str()),
            _ => None,
        }
    }
    /// Try to extract an int.
    pub fn as_int(&self) -> Option<i64> {
        match self {
            MetaValue::Int(n) => Some(*n),
            _ => None,
        }
    }
}
/// Registry for user-defined elaborators.
pub struct UserElabRegistry {
    elabs: Vec<Box<dyn UserElab>>,
}
impl UserElabRegistry {
    /// Create an empty registry.
    pub fn new() -> Self {
        UserElabRegistry { elabs: Vec::new() }
    }
    /// Register a new user-defined elaborator.
    pub fn register(&mut self, elab: Box<dyn UserElab>) {
        self.elabs.push(elab);
    }
    /// Find an elaborator that handles the given head symbol.
    pub fn find_for(&self, head: &str) -> Option<&dyn UserElab> {
        self.elabs
            .iter()
            .find(|e| e.handles(head))
            .map(|e| e.as_ref())
    }
    /// Return the number of registered elaborators.
    pub fn len(&self) -> usize {
        self.elabs.len()
    }
    /// Return true if no elaborators are registered.
    pub fn is_empty(&self) -> bool {
        self.elabs.is_empty()
    }
}
/// Example built-in user tactic: closes any goal when a `False` hypothesis is present.
pub struct FalseEliminator;
/// Quotation mode for the term quoter.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum QuotationMode {
    /// Quote the entire expression (no splices).
    #[default]
    Full,
    /// Allow splice sites inside the quotation.
    Partial,
}
/// A simple term quoter that converts strings into `QuotedExpr`.
#[derive(Debug, Default)]
pub struct TermQuoter {
    mode: QuotationMode,
}
impl TermQuoter {
    /// Create a full quoter.
    pub fn full() -> Self {
        Self {
            mode: QuotationMode::Full,
        }
    }
    /// Create a partial quoter (allows splices).
    pub fn partial() -> Self {
        Self {
            mode: QuotationMode::Partial,
        }
    }
    /// Quote a surface expression string.
    pub fn quote(&self, input: &str) -> QuotedExpr {
        reflect_expr(input)
    }
    /// Quote a list of argument strings.
    pub fn quote_args(&self, args: &[&str]) -> Vec<QuotedExpr> {
        args.iter().map(|a| self.quote(a)).collect()
    }
    /// Return the current quotation mode.
    pub fn mode(&self) -> QuotationMode {
        self.mode
    }
}
/// Errors that can occur during meta-level evaluation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MetaProgrammingError {
    /// An undefined meta-level variable was referenced.
    UndefinedVariable(String),
    /// A macro application failed.
    MacroFailed { name: String, reason: String },
    /// Type mismatch at the meta level.
    TypeMismatch { expected: String, found: String },
    /// Quotation depth exceeded.
    QuotationDepthExceeded(u32),
    /// Splice outside a quotation context.
    SpliceOutsideQuotation,
    /// General evaluation error.
    EvalError(String),
}
/// A stack of meta-evaluation frames.
#[derive(Debug, Clone, Default)]
pub struct MetaStack {
    frames: Vec<MetaFrame>,
}
impl MetaStack {
    /// Create an empty stack.
    pub fn new() -> Self {
        Self::default()
    }
    /// Push a frame onto the stack.
    pub fn push(&mut self, frame: MetaFrame) {
        self.frames.push(frame);
    }
    /// Pop a frame from the stack.
    pub fn pop(&mut self) -> Option<MetaFrame> {
        self.frames.pop()
    }
    /// Peek at the top frame.
    pub fn top(&self) -> Option<&MetaFrame> {
        self.frames.last()
    }
    /// Return the current depth (number of frames).
    pub fn depth(&self) -> usize {
        self.frames.len()
    }
    /// Return true if the stack is empty.
    pub fn is_empty(&self) -> bool {
        self.frames.is_empty()
    }
    /// Return true if the current context is inside a quotation (at any depth).
    pub fn in_quotation(&self) -> bool {
        self.frames.iter().any(|f| f.is_quotation())
    }
    /// Return true if the current context is inside a splice (at any depth).
    pub fn in_splice(&self) -> bool {
        self.frames.iter().any(|f| f.is_splice())
    }
    /// Return the depth of the innermost quotation frame, or None.
    pub fn quotation_depth(&self) -> Option<usize> {
        self.frames.iter().rposition(|f| f.is_quotation())
    }
}
