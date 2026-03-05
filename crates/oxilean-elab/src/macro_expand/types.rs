//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{Expr, Level, Literal, Name};
use std::collections::HashMap;

use super::functions::*;

#[allow(dead_code)]
pub struct IdentityStep;
/// A template for constructing expressions during macro expansion.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MacroTemplate {
    /// A literal expression.
    Expr(Expr),
    /// Substitution variable (replaced from bindings).
    Var(Name),
    /// Application of two templates.
    App(Box<MacroTemplate>, Box<MacroTemplate>),
    /// Sequence of templates.
    Seq(Vec<MacroTemplate>),
    /// Splice a list variable (expand repetition).
    Splice(Name),
    /// Quotation: produce a quoted representation.
    Quote(Box<MacroTemplate>),
}
#[allow(dead_code)]
pub struct MacroRegistry {
    macros: HashMap<Name, MacroDef>,
    categories: HashMap<String, Vec<Name>>,
}
#[allow(dead_code)]
impl MacroRegistry {
    pub fn new() -> Self {
        MacroRegistry {
            macros: HashMap::new(),
            categories: HashMap::new(),
        }
    }
    pub fn register(&mut self, def: MacroDef) {
        let name = def.name.clone();
        let cat = format!("{:?}", def.kind);
        self.categories.entry(cat).or_default().push(name.clone());
        self.macros.insert(name, def);
    }
    pub fn lookup(&self, name: &Name) -> Option<&MacroDef> {
        self.macros.get(name)
    }
    pub fn has_macro(&self, name: &Name) -> bool {
        self.macros.contains_key(name)
    }
    pub fn all_names(&self) -> Vec<&Name> {
        self.macros.keys().collect()
    }
    pub fn by_kind(&self, kind: &MacroKind) -> Vec<&MacroDef> {
        let key = format!("{:?}", kind);
        self.categories
            .get(&key)
            .map(|names| names.iter().filter_map(|n| self.macros.get(n)).collect())
            .unwrap_or_default()
    }
    pub fn count(&self) -> usize {
        self.macros.len()
    }
    pub fn is_empty(&self) -> bool {
        self.macros.is_empty()
    }
    pub fn remove(&mut self, name: &Name) -> bool {
        self.macros.remove(name).is_some()
    }
}
#[allow(dead_code)]
pub struct MacroScopeStack {
    scopes: Vec<MacroScope>,
}
#[allow(dead_code)]
impl MacroScopeStack {
    pub fn new() -> Self {
        MacroScopeStack { scopes: Vec::new() }
    }
    pub fn push_scope(&mut self) {
        let level = self.scopes.len();
        self.scopes.push(MacroScope::new(level));
    }
    pub fn pop_scope(&mut self) -> Option<MacroScope> {
        self.scopes.pop()
    }
    pub fn current_mut(&mut self) -> Option<&mut MacroScope> {
        self.scopes.last_mut()
    }
    pub fn add_to_current(&mut self, def: MacroDef) {
        if let Some(scope) = self.current_mut() {
            scope.add(def);
        }
    }
    pub fn visible_macros(&self) -> Vec<&MacroDef> {
        self.scopes.iter().flat_map(|s| s.defs()).collect()
    }
    pub fn depth(&self) -> usize {
        self.scopes.len()
    }
    pub fn is_empty(&self) -> bool {
        self.scopes.is_empty()
    }
}
#[allow(dead_code)]
pub struct MacroHygieneMap {
    renames: HashMap<Name, Name>,
    counter: u64,
    prefix: String,
}
#[allow(dead_code)]
impl MacroHygieneMap {
    pub fn new(prefix: impl Into<String>) -> Self {
        MacroHygieneMap {
            renames: HashMap::new(),
            counter: 0,
            prefix: prefix.into(),
        }
    }
    pub fn fresh_name(&mut self, original: &Name) -> Name {
        let fresh = Name::str(format!("{}{}", self.prefix, self.counter));
        self.counter += 1;
        self.renames.insert(original.clone(), fresh.clone());
        fresh
    }
    pub fn lookup(&self, name: &Name) -> Option<&Name> {
        self.renames.get(name)
    }
    pub fn apply(&self, name: &Name) -> Name {
        self.renames
            .get(name)
            .cloned()
            .unwrap_or_else(|| name.clone())
    }
    pub fn rename_count(&self) -> usize {
        self.renames.len()
    }
    pub fn reset(&mut self) {
        self.renames.clear();
        self.counter = 0;
    }
}
/// A single macro matching rule: pattern -> template.
#[derive(Clone, Debug)]
pub struct MacroRule {
    /// The pattern to match against.
    pub pattern: MacroPattern,
    /// The template to produce on match.
    pub template: MacroTemplate,
}
#[allow(dead_code)]
pub struct MacroInterpreter {
    env: HashMap<Name, MacroAst>,
    call_depth: usize,
    max_depth: usize,
}
#[allow(dead_code)]
impl MacroInterpreter {
    pub fn new(max_depth: usize) -> Self {
        MacroInterpreter {
            env: HashMap::new(),
            call_depth: 0,
            max_depth,
        }
    }
    pub fn define(&mut self, name: Name, val: MacroAst) {
        self.env.insert(name, val);
    }
    pub fn eval_atom(&self, name: &Name) -> Option<&MacroAst> {
        self.env.get(name)
    }
    pub fn call_depth(&self) -> usize {
        self.call_depth
    }
    pub fn push_call(&mut self) -> bool {
        if self.call_depth >= self.max_depth {
            false
        } else {
            self.call_depth += 1;
            true
        }
    }
    pub fn pop_call(&mut self) {
        if self.call_depth > 0 {
            self.call_depth -= 1;
        }
    }
    pub fn is_defined(&self, name: &Name) -> bool {
        self.env.contains_key(name)
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct MacroExpansionConfig {
    pub max_depth: usize,
    pub enable_trace: bool,
    pub hygiene_mode: HygieneMode,
    pub enable_caching: bool,
    pub max_cache_size: usize,
}
#[allow(dead_code)]
impl MacroExpansionConfig {
    pub fn new() -> Self {
        MacroExpansionConfig {
            max_depth: 100,
            enable_trace: false,
            hygiene_mode: HygieneMode::Hygienic,
            enable_caching: true,
            max_cache_size: 1024,
        }
    }
    pub fn with_max_depth(mut self, depth: usize) -> Self {
        self.max_depth = depth;
        self
    }
    pub fn with_trace(mut self) -> Self {
        self.enable_trace = true;
        self
    }
    pub fn without_cache(mut self) -> Self {
        self.enable_caching = false;
        self
    }
    pub fn with_hygiene(mut self, mode: HygieneMode) -> Self {
        self.hygiene_mode = mode;
        self
    }
}
#[allow(dead_code)]
pub struct MacroExtensionMarker;
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum MacroTraceEntry {
    Entered { macro_name: Name, depth: usize },
    Matched { rule_index: usize },
    NoMatch,
    Expanded { result_size: usize },
    Error { message: String },
    CacheHit { macro_name: Name },
}
#[allow(dead_code)]
pub struct MacroTracer {
    entries: Vec<MacroTraceEntry>,
    max_entries: usize,
    enabled: bool,
}
#[allow(dead_code)]
impl MacroTracer {
    pub fn new(max_entries: usize) -> Self {
        MacroTracer {
            entries: Vec::new(),
            max_entries,
            enabled: true,
        }
    }
    pub fn disabled() -> Self {
        MacroTracer {
            entries: Vec::new(),
            max_entries: 0,
            enabled: false,
        }
    }
    pub fn record(&mut self, entry: MacroTraceEntry) {
        if !self.enabled {
            return;
        }
        if self.entries.len() >= self.max_entries {
            self.entries.remove(0);
        }
        self.entries.push(entry);
    }
    pub fn entries(&self) -> &[MacroTraceEntry] {
        &self.entries
    }
    pub fn error_count(&self) -> usize {
        self.entries
            .iter()
            .filter(|e| matches!(e, MacroTraceEntry::Error { .. }))
            .count()
    }
    pub fn expansion_count(&self) -> usize {
        self.entries
            .iter()
            .filter(|e| matches!(e, MacroTraceEntry::Expanded { .. }))
            .count()
    }
    pub fn clear(&mut self) {
        self.entries.clear();
    }
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}
#[allow(dead_code)]
pub struct MacroScope {
    level: usize,
    defs: Vec<MacroDef>,
}
#[allow(dead_code)]
impl MacroScope {
    pub fn new(level: usize) -> Self {
        MacroScope {
            level,
            defs: Vec::new(),
        }
    }
    pub fn add(&mut self, def: MacroDef) {
        self.defs.push(def);
    }
    pub fn defs(&self) -> &[MacroDef] {
        &self.defs
    }
    pub fn level(&self) -> usize {
        self.level
    }
    pub fn is_empty(&self) -> bool {
        self.defs.is_empty()
    }
}
#[allow(dead_code)]
pub struct MacroNamespace {
    name: String,
    macros: HashMap<String, MacroDef>,
}
#[allow(dead_code)]
impl MacroNamespace {
    pub fn new(name: impl Into<String>) -> Self {
        MacroNamespace {
            name: name.into(),
            macros: HashMap::new(),
        }
    }
    pub fn register(&mut self, def: MacroDef) {
        self.macros.insert(def.name.to_string(), def);
    }
    pub fn lookup(&self, local_name: &str) -> Option<&MacroDef> {
        self.macros.get(local_name)
    }
    pub fn qualified_name(&self, local: &str) -> String {
        format!("{}.{}", self.name, local)
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn count(&self) -> usize {
        self.macros.len()
    }
}
/// The kind of a macro.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MacroKind {
    /// Syntax extension (general).
    SyntaxMacro,
    /// Command-level macro (top-level declaration).
    CommandMacro,
    /// Tactic macro.
    TacticMacro,
    /// Term-level macro (expression macro).
    TermMacro,
    /// Notation-backed macro.
    NotationMacro,
}
/// Controls how macro expansion handles variable names.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum HygieneMode {
    /// Default: renames bound variables to avoid capture.
    Hygienic,
    /// Raw substitution with no renaming.
    Unhygienic,
    /// Some names are preserved, others are renamed.
    SemiHygienic,
}
/// Macro expansion engine.
pub struct MacroExpander {
    /// Registered macros.
    macros: HashMap<Name, MacroDef>,
    /// Maximum expansion depth (prevents infinite loops).
    max_depth: usize,
    /// Hygiene mode for fresh name generation.
    hygiene_mode: HygieneMode,
    /// Counter for generating fresh names.
    fresh_counter: u64,
    /// Expansion trace (for debugging).
    trace: Vec<Expr>,
    /// Whether tracing is enabled.
    trace_enabled: bool,
}
impl MacroExpander {
    /// Create a new macro expander.
    pub fn new() -> Self {
        Self {
            macros: HashMap::new(),
            max_depth: 100,
            hygiene_mode: HygieneMode::Hygienic,
            fresh_counter: 0,
            trace: Vec::new(),
            trace_enabled: false,
        }
    }
    /// Register a macro.
    pub fn register(&mut self, macro_def: MacroDef) {
        self.macros.insert(macro_def.name.clone(), macro_def);
    }
    /// Register an additional rule for an existing macro.
    #[allow(dead_code)]
    pub fn register_rule(&mut self, name: &Name, rule: MacroRule) {
        if let Some(def) = self.macros.get_mut(name) {
            def.rules.push(rule);
        }
    }
    /// Expand macros in an expression (simple mode, backward compat).
    pub fn expand(&self, expr: &Expr) -> Result<Expr, String> {
        self.expand_impl(expr, 0).map_err(|e| e.to_string())
    }
    /// Expand macros with a full error type.
    #[allow(dead_code)]
    pub fn expand_checked(&self, expr: &Expr) -> Result<Expr, MacroError> {
        self.expand_impl(expr, 0)
    }
    /// Implementation with depth tracking.
    fn expand_impl(&self, expr: &Expr, depth: usize) -> Result<Expr, MacroError> {
        if depth > self.max_depth {
            return Err(MacroError::DepthExceeded);
        }
        match expr {
            Expr::Const(name, levels) => {
                if let Some(macro_def) = self.macros.get(name) {
                    if !macro_def.rules.is_empty() {
                        for rule in &macro_def.rules {
                            if let Some(bindings) = self.match_pattern(&rule.pattern, expr) {
                                let result = self.substitute_template(&rule.template, &bindings)?;
                                return self.expand_impl(&result, depth + 1);
                            }
                        }
                    }
                    let expanded = if self.hygiene_mode == HygieneMode::Hygienic {
                        self.apply_hygiene_to_expr(&macro_def.template, depth as u64)
                    } else {
                        macro_def.template.clone()
                    };
                    self.expand_impl(&expanded, depth + 1)
                } else {
                    Ok(Expr::Const(name.clone(), levels.clone()))
                }
            }
            Expr::App(f, a) => {
                let f_expanded = self.expand_impl(f, depth)?;
                let a_expanded = self.expand_impl(a, depth)?;
                Ok(Expr::App(Box::new(f_expanded), Box::new(a_expanded)))
            }
            Expr::Lam(bi, name, ty, body) => {
                let ty_expanded = self.expand_impl(ty, depth)?;
                let body_expanded = self.expand_impl(body, depth)?;
                Ok(Expr::Lam(
                    *bi,
                    name.clone(),
                    Box::new(ty_expanded),
                    Box::new(body_expanded),
                ))
            }
            Expr::Pi(bi, name, ty, body) => {
                let ty_expanded = self.expand_impl(ty, depth)?;
                let body_expanded = self.expand_impl(body, depth)?;
                Ok(Expr::Pi(
                    *bi,
                    name.clone(),
                    Box::new(ty_expanded),
                    Box::new(body_expanded),
                ))
            }
            Expr::Let(name, ty, val, body) => {
                let ty_expanded = self.expand_impl(ty, depth)?;
                let val_expanded = self.expand_impl(val, depth)?;
                let body_expanded = self.expand_impl(body, depth)?;
                Ok(Expr::Let(
                    name.clone(),
                    Box::new(ty_expanded),
                    Box::new(val_expanded),
                    Box::new(body_expanded),
                ))
            }
            _ => Ok(expr.clone()),
        }
    }
    /// Expand with explicit arguments (for parameterized macros).
    #[allow(dead_code)]
    pub fn expand_with_args(&self, name: &Name, args: &[Expr]) -> Result<Expr, MacroError> {
        let macro_def = self
            .macros
            .get(name)
            .ok_or_else(|| MacroError::UndefinedMacro(name.to_string()))?;
        if !macro_def.rules.is_empty() {
            let mut app_expr = Expr::Const(name.clone(), vec![]);
            for arg in args {
                app_expr = Expr::App(Box::new(app_expr), Box::new(arg.clone()));
            }
            for rule in &macro_def.rules {
                if let Some(bindings) = self.match_pattern(&rule.pattern, &app_expr) {
                    return self.substitute_template(&rule.template, &bindings);
                }
            }
        }
        let mut result = macro_def.template.clone();
        for (i, param) in macro_def.params.iter().enumerate() {
            if let Some(arg) = args.get(i) {
                result = substitute_name_in_expr(&result, param, arg);
            }
        }
        Ok(result)
    }
    /// Match a pattern against an expression, returning bindings on success.
    #[allow(dead_code)]
    pub fn match_pattern(
        &self,
        pattern: &MacroPattern,
        expr: &Expr,
    ) -> Option<HashMap<Name, Expr>> {
        let mut bindings = HashMap::new();
        if match_pattern_impl(pattern, expr, &mut bindings) {
            Some(bindings)
        } else {
            None
        }
    }
    /// Substitute a template with bindings to produce an expression.
    #[allow(dead_code)]
    pub fn substitute_template(
        &self,
        template: &MacroTemplate,
        bindings: &HashMap<Name, Expr>,
    ) -> Result<Expr, MacroError> {
        substitute_template_impl(template, bindings)
    }
    /// Apply hygienic renaming to an expression.
    #[allow(dead_code)]
    pub fn apply_hygiene(&mut self, expr: &Expr, scope_id: u64) -> Expr {
        self.apply_hygiene_to_expr(expr, scope_id)
    }
    /// Internal: apply hygienic renaming (immutable self version).
    fn apply_hygiene_to_expr(&self, expr: &Expr, scope_id: u64) -> Expr {
        match expr {
            Expr::Lam(bi, name, ty, body) => {
                let new_name = hygiene_rename(name, scope_id);
                Expr::Lam(
                    *bi,
                    new_name,
                    Box::new(self.apply_hygiene_to_expr(ty, scope_id)),
                    Box::new(self.apply_hygiene_to_expr(body, scope_id)),
                )
            }
            Expr::Pi(bi, name, ty, body) => {
                let new_name = hygiene_rename(name, scope_id);
                Expr::Pi(
                    *bi,
                    new_name,
                    Box::new(self.apply_hygiene_to_expr(ty, scope_id)),
                    Box::new(self.apply_hygiene_to_expr(body, scope_id)),
                )
            }
            Expr::Let(name, ty, val, body) => {
                let new_name = hygiene_rename(name, scope_id);
                Expr::Let(
                    new_name,
                    Box::new(self.apply_hygiene_to_expr(ty, scope_id)),
                    Box::new(self.apply_hygiene_to_expr(val, scope_id)),
                    Box::new(self.apply_hygiene_to_expr(body, scope_id)),
                )
            }
            Expr::App(f, a) => Expr::App(
                Box::new(self.apply_hygiene_to_expr(f, scope_id)),
                Box::new(self.apply_hygiene_to_expr(a, scope_id)),
            ),
            _ => expr.clone(),
        }
    }
    /// Expand an expression recursively until no more macros remain (fixed point).
    #[allow(dead_code)]
    pub fn expand_fully(&self, expr: &Expr) -> Result<Expr, MacroError> {
        let mut current = expr.clone();
        for depth in 0..self.max_depth {
            let expanded = self.expand_impl(&current, depth)?;
            if expanded == current {
                return Ok(expanded);
            }
            current = expanded;
        }
        Err(MacroError::DepthExceeded)
    }
    /// Check if an expression is terminal (contains no more expandable macros).
    #[allow(dead_code)]
    pub fn is_terminal(&self, expr: &Expr) -> bool {
        match expr {
            Expr::Const(name, _) => !self.macros.contains_key(name),
            Expr::App(f, a) => self.is_terminal(f) && self.is_terminal(a),
            Expr::Lam(_, _, ty, body) => self.is_terminal(ty) && self.is_terminal(body),
            Expr::Pi(_, _, ty, body) => self.is_terminal(ty) && self.is_terminal(body),
            Expr::Let(_, ty, val, body) => {
                self.is_terminal(ty) && self.is_terminal(val) && self.is_terminal(body)
            }
            _ => true,
        }
    }
    /// Produce a trace of each expansion step (for debugging).
    #[allow(dead_code)]
    pub fn trace_expansion(&mut self, expr: &Expr) -> Vec<Expr> {
        self.trace.clear();
        self.trace_enabled = true;
        let mut current = expr.clone();
        self.trace.push(current.clone());
        for _ in 0..self.max_depth {
            match self.expand_impl(&current, 0) {
                Ok(expanded) => {
                    if expanded == current {
                        break;
                    }
                    self.trace.push(expanded.clone());
                    current = expanded;
                }
                Err(_) => break,
            }
        }
        self.trace_enabled = false;
        self.trace.clone()
    }
    /// Check if a name is a registered macro.
    pub fn is_macro(&self, name: &Name) -> bool {
        self.macros.contains_key(name)
    }
    /// Get all registered macros.
    pub fn all_macros(&self) -> Vec<&MacroDef> {
        self.macros.values().collect()
    }
    /// Set maximum expansion depth.
    pub fn set_max_depth(&mut self, depth: usize) {
        self.max_depth = depth;
    }
    /// Set the hygiene mode.
    #[allow(dead_code)]
    pub fn set_hygiene_mode(&mut self, mode: HygieneMode) {
        self.hygiene_mode = mode;
    }
    /// Generate a fresh hygienic name.
    #[allow(dead_code)]
    pub fn fresh_name(&mut self, base: &str) -> Name {
        let id = self.fresh_counter;
        self.fresh_counter += 1;
        Name::str(format!("_hyg_{}_{}", base, id))
    }
}
#[allow(dead_code)]
pub struct MacroPatternMatcher;
#[allow(dead_code)]
impl MacroPatternMatcher {
    pub fn new() -> Self {
        MacroPatternMatcher
    }
    /// Returns true if `expr` structurally matches `pattern` (shallow check).
    pub fn matches_shallow(&self, pattern: &MacroPattern, expr: &Expr) -> bool {
        match (pattern, expr) {
            (MacroPattern::Var(_), _) => true,
            (MacroPattern::Lit(s), Expr::Lit(_)) => !s.is_empty(),
            (MacroPattern::App(_, _), Expr::App(_, _)) => true,
            _ => false,
        }
    }
    /// Count patterns in a rule that are wildcards.
    pub fn wildcard_count(&self, rule: &MacroRule) -> usize {
        self.count_wilds(&rule.pattern)
    }
    fn count_wilds(&self, pat: &MacroPattern) -> usize {
        match pat {
            MacroPattern::Var(_) => 1,
            MacroPattern::App(f, a) => self.count_wilds(f) + self.count_wilds(a),
            _ => 0,
        }
    }
    /// Score a rule by specificity (fewer wildcards = more specific = higher score).
    pub fn specificity_score(&self, rule: &MacroRule) -> i32 {
        let wildcards = self.wildcard_count(rule) as i32;
        100 - wildcards * 10
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MacroAst {
    Atom(Name),
    Num(u64),
    Str(String),
    List(Vec<MacroAst>),
    Cons(Box<MacroAst>, Box<MacroAst>),
    Nil,
}
#[allow(dead_code)]
impl MacroAst {
    pub fn atom(s: &str) -> Self {
        MacroAst::Atom(Name::str(s))
    }
    pub fn num(n: u64) -> Self {
        MacroAst::Num(n)
    }
    pub fn list(items: Vec<MacroAst>) -> Self {
        MacroAst::List(items)
    }
    pub fn is_nil(&self) -> bool {
        matches!(self, MacroAst::Nil)
    }
    pub fn is_atom(&self) -> bool {
        matches!(self, MacroAst::Atom(_))
    }
    pub fn len(&self) -> usize {
        match self {
            MacroAst::List(items) => items.len(),
            MacroAst::Nil => 0,
            _ => 1,
        }
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    pub fn head(&self) -> Option<&MacroAst> {
        match self {
            MacroAst::List(items) => items.first(),
            MacroAst::Cons(h, _) => Some(h),
            _ => None,
        }
    }
    pub fn tail(&self) -> Option<&MacroAst> {
        match self {
            MacroAst::Cons(_, t) => Some(t),
            _ => None,
        }
    }
}
/// Errors that can occur during macro expansion.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MacroError {
    /// The expression does not match the macro pattern.
    PatternMismatch(String),
    /// Expansion depth limit exceeded (probable infinite loop).
    DepthExceeded,
    /// Referenced macro is not defined.
    UndefinedMacro(String),
    /// Multiple macro rules match ambiguously.
    AmbiguousMatch(String),
    /// Hygienic renaming detected a conflict.
    HygieneViolation(String),
    /// General expansion error.
    ExpansionError(String),
}
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct MacroExpansionStats {
    pub total_calls: u64,
    pub successful: u64,
    pub failed: u64,
    pub cache_hits: u64,
    pub max_depth_seen: usize,
    pub total_expansions: u64,
}
#[allow(dead_code)]
impl MacroExpansionStats {
    pub fn new() -> Self {
        MacroExpansionStats::default()
    }
    pub fn record_success(&mut self, expansions: usize, depth: usize) {
        self.total_calls += 1;
        self.successful += 1;
        self.total_expansions += expansions as u64;
        if depth > self.max_depth_seen {
            self.max_depth_seen = depth;
        }
    }
    pub fn record_failure(&mut self) {
        self.total_calls += 1;
        self.failed += 1;
    }
    pub fn record_cache_hit(&mut self) {
        self.cache_hits += 1;
    }
    pub fn success_rate(&self) -> f64 {
        if self.total_calls == 0 {
            0.0
        } else {
            self.successful as f64 / self.total_calls as f64
        }
    }
    pub fn mean_expansions(&self) -> f64 {
        if self.successful == 0 {
            0.0
        } else {
            self.total_expansions as f64 / self.successful as f64
        }
    }
}
/// A macro definition.
#[derive(Clone, Debug)]
pub struct MacroDef {
    /// Macro name.
    pub name: Name,
    /// Parameter names (for simple positional macros).
    pub params: Vec<Name>,
    /// Expansion template (for simple macros).
    pub template: Expr,
    /// The kind of macro.
    pub kind: MacroKind,
    /// Documentation string.
    pub doc: Option<String>,
    /// Matching rules (for multi-rule macros).
    pub rules: Vec<MacroRule>,
    /// Optional scope restriction.
    pub scope: Option<Name>,
    /// Hygiene mode.
    pub hygiene: HygieneMode,
}
impl MacroDef {
    /// Create a simple macro with a name, params, and template expression.
    #[allow(dead_code)]
    pub fn simple(name: Name, params: Vec<Name>, template: Expr) -> Self {
        Self {
            name,
            params,
            template,
            kind: MacroKind::TermMacro,
            doc: None,
            rules: Vec::new(),
            scope: None,
            hygiene: HygieneMode::Hygienic,
        }
    }
    /// Create a rule-based macro.
    #[allow(dead_code)]
    pub fn with_rules(name: Name, kind: MacroKind, rules: Vec<MacroRule>) -> Self {
        Self {
            name,
            params: Vec::new(),
            template: Expr::Sort(Level::zero()),
            kind,
            doc: None,
            rules,
            scope: None,
            hygiene: HygieneMode::Hygienic,
        }
    }
}
#[allow(dead_code)]
pub struct MacroPipeline {
    steps: Vec<Box<dyn MacroTransformStep>>,
}
#[allow(dead_code)]
impl MacroPipeline {
    pub fn new() -> Self {
        MacroPipeline { steps: Vec::new() }
    }
    pub fn add_step<S: MacroTransformStep + 'static>(mut self, step: S) -> Self {
        self.steps.push(Box::new(step));
        self
    }
    pub fn run(&self, expr: Expr) -> Result<Expr, MacroError> {
        self.steps
            .iter()
            .try_fold(expr, |e, step| step.transform(e))
    }
    pub fn step_names(&self) -> Vec<&'static str> {
        self.steps.iter().map(|s| s.step_name()).collect()
    }
    pub fn len(&self) -> usize {
        self.steps.len()
    }
    pub fn is_empty(&self) -> bool {
        self.steps.is_empty()
    }
}
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct MacroExpansionReport {
    pub macros_applied: Vec<Name>,
    pub depth_reached: usize,
    pub total_rewrites: usize,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}
#[allow(dead_code)]
impl MacroExpansionReport {
    pub fn new() -> Self {
        MacroExpansionReport::default()
    }
    pub fn record_application(&mut self, macro_name: Name) {
        self.macros_applied.push(macro_name);
        self.total_rewrites += 1;
    }
    pub fn record_error(&mut self, err: impl Into<String>) {
        self.errors.push(err.into());
    }
    pub fn record_warning(&mut self, warn: impl Into<String>) {
        self.warnings.push(warn.into());
    }
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }
    pub fn applied_count(&self) -> usize {
        self.macros_applied.len()
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct MacroExpansionResult {
    pub expr: Expr,
    pub expansions_applied: usize,
    pub max_depth_reached: usize,
    pub cache_hits: u64,
    pub warnings: Vec<String>,
}
#[allow(dead_code)]
impl MacroExpansionResult {
    pub fn new(expr: Expr) -> Self {
        MacroExpansionResult {
            expr,
            expansions_applied: 0,
            cache_hits: 0,
            max_depth_reached: 0,
            warnings: Vec::new(),
        }
    }
    pub fn with_expansions(mut self, n: usize) -> Self {
        self.expansions_applied = n;
        self
    }
    pub fn with_warning(mut self, msg: impl Into<String>) -> Self {
        self.warnings.push(msg.into());
        self
    }
    pub fn has_warnings(&self) -> bool {
        !self.warnings.is_empty()
    }
}
/// A pattern for matching expressions during macro expansion.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MacroPattern {
    /// Match an exact name.
    Exact(Name),
    /// Match an application `f a`.
    App(Box<MacroPattern>, Box<MacroPattern>),
    /// Capture variable: matches anything and binds it.
    Var(Name),
    /// Sequence of patterns (for multi-argument matching).
    Seq(Vec<MacroPattern>),
    /// Optional: match zero or one occurrence.
    Optional(Box<MacroPattern>),
    /// Many: match zero or more occurrences.
    Many(Box<MacroPattern>),
    /// Literal text match.
    Lit(String),
}
#[allow(dead_code)]
pub struct MacroEnvironment {
    bindings: HashMap<Name, Expr>,
    parent: Option<Box<MacroEnvironment>>,
    depth: usize,
}
#[allow(dead_code)]
impl MacroEnvironment {
    pub fn new() -> Self {
        MacroEnvironment {
            bindings: HashMap::new(),
            parent: None,
            depth: 0,
        }
    }
    pub fn child(parent: MacroEnvironment) -> Self {
        let depth = parent.depth + 1;
        MacroEnvironment {
            bindings: HashMap::new(),
            parent: Some(Box::new(parent)),
            depth,
        }
    }
    pub fn bind(&mut self, name: Name, expr: Expr) {
        self.bindings.insert(name, expr);
    }
    pub fn lookup(&self, name: &Name) -> Option<&Expr> {
        if let Some(val) = self.bindings.get(name) {
            Some(val)
        } else if let Some(parent) = &self.parent {
            parent.lookup(name)
        } else {
            None
        }
    }
    pub fn depth(&self) -> usize {
        self.depth
    }
    pub fn is_root(&self) -> bool {
        self.parent.is_none()
    }
    pub fn local_count(&self) -> usize {
        self.bindings.len()
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum MacroValidationError {
    EmptyRuleList,
    DuplicateRulePriority(i32),
    TemplateVarNotInPattern(Name),
    InvalidRecursivePattern,
}
#[allow(dead_code)]
pub struct MacroValidator;
#[allow(dead_code)]
impl MacroValidator {
    pub fn new() -> Self {
        MacroValidator
    }
    pub fn validate(&self, def: &MacroDef) -> Vec<MacroValidationError> {
        let mut errors = Vec::new();
        if def.rules.is_empty() {
            errors.push(MacroValidationError::EmptyRuleList);
        }
        // Detect rules with duplicate patterns (same matching priority).
        let mut seen_patterns: HashMap<usize, i32> = HashMap::new();
        for (i, rule) in def.rules.iter().enumerate() {
            for (j, other) in def.rules.iter().enumerate().take(i) {
                if rule.pattern == other.pattern {
                    let prio = *seen_patterns.entry(j).or_insert(j as i32);
                    errors.push(MacroValidationError::DuplicateRulePriority(prio));
                    break;
                }
            }
        }
        errors
    }
    pub fn is_valid(&self, def: &MacroDef) -> bool {
        self.validate(def).is_empty()
    }
}
