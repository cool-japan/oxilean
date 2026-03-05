//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use crate::error_impl::ParseError;
use crate::tokens::{Span, Token, TokenKind};
use std::collections::HashMap;

/// A macro template variable.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MacroVarExt {
    /// Variable name
    pub name: String,
    /// Whether this is a repetition variable
    pub is_rep: bool,
}
impl MacroVarExt {
    /// Create a simple macro variable.
    #[allow(dead_code)]
    pub fn simple(name: &str) -> Self {
        MacroVarExt {
            name: name.to_string(),
            is_rep: false,
        }
    }
    /// Create a repetition macro variable.
    #[allow(dead_code)]
    pub fn rep(name: &str) -> Self {
        MacroVarExt {
            name: name.to_string(),
            is_rep: true,
        }
    }
}
/// A macro library with named groups.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct MacroLibrary {
    /// Groups of macros
    pub groups: std::collections::HashMap<String, Vec<MacroDefinitionExt>>,
}
impl MacroLibrary {
    /// Create a new empty library.
    #[allow(dead_code)]
    pub fn new() -> Self {
        MacroLibrary {
            groups: std::collections::HashMap::new(),
        }
    }
    /// Add a macro to a group.
    #[allow(dead_code)]
    pub fn add_to_group(&mut self, group: &str, def: MacroDefinitionExt) {
        self.groups.entry(group.to_string()).or_default().push(def);
    }
    /// Returns all macros in a group.
    #[allow(dead_code)]
    pub fn group(&self, name: &str) -> &[MacroDefinitionExt] {
        self.groups.get(name).map(|v| v.as_slice()).unwrap_or(&[])
    }
    /// Total number of macros.
    #[allow(dead_code)]
    pub fn total_macros(&self) -> usize {
        self.groups.values().map(|v| v.len()).sum()
    }
}
/// An item in a `syntax` parser specification.
#[derive(Debug, Clone, PartialEq)]
pub enum SyntaxItem {
    /// A specific token (keyword, punctuation, etc.).
    Token(TokenKind),
    /// A syntactic category placeholder (e.g. `term`, `ident`).
    Category(String),
    /// An optional sub-item.
    Optional(Box<SyntaxItem>),
    /// Zero-or-more repetition.
    Many(Box<SyntaxItem>),
    /// A grouped sequence.
    Group(Vec<SyntaxItem>),
}
/// A macro substitution record.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Clone)]
pub struct MacroSubst {
    /// Variable name
    pub var: String,
    /// Substituted value
    pub value: String,
}
impl MacroSubst {
    /// Create a new substitution.
    #[allow(dead_code)]
    pub fn new(var: &str, value: &str) -> Self {
        MacroSubst {
            var: var.to_string(),
            value: value.to_string(),
        }
    }
}
/// A complete macro definition, potentially with multiple rules.
#[derive(Debug, Clone)]
pub struct MacroDef {
    /// The macro name.
    pub name: String,
    /// Ordered list of rewrite rules (first match wins).
    pub rules: Vec<MacroRule>,
    /// Optional documentation string.
    pub doc: Option<String>,
    /// Hygiene information for the definition site.
    pub hygiene: HygieneInfo,
}
impl MacroDef {
    /// Create a new macro definition.
    pub fn new(name: String, rules: Vec<MacroRule>, hygiene: HygieneInfo) -> Self {
        Self {
            name,
            rules,
            doc: None,
            hygiene,
        }
    }
    /// Attach documentation.
    pub fn with_doc(mut self, doc: String) -> Self {
        self.doc = Some(doc);
        self
    }
    /// Number of rules.
    pub fn rule_count(&self) -> usize {
        self.rules.len()
    }
}
/// A macro expansion result: either a string or an error.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Clone)]
pub enum MacroExpansionResult {
    /// Successful expansion
    Ok(String),
    /// Failed expansion
    Err(MacroExpansionError),
}
impl MacroExpansionResult {
    /// Returns true if expansion succeeded.
    #[allow(dead_code)]
    pub fn is_ok(&self) -> bool {
        matches!(self, MacroExpansionResult::Ok(_))
    }
    /// Unwrap the success value.
    #[allow(dead_code)]
    pub fn unwrap(self) -> String {
        match self {
            MacroExpansionResult::Ok(s) => s,
            MacroExpansionResult::Err(e) => {
                panic!("macro expansion error: {}", e.message)
            }
        }
    }
    /// Unwrap or return a default.
    #[allow(dead_code)]
    pub fn unwrap_or(self, default: &str) -> String {
        match self {
            MacroExpansionResult::Ok(s) => s,
            MacroExpansionResult::Err(_) => default.to_string(),
        }
    }
}
/// A macro pattern matcher that checks if a token sequence matches a macro pattern.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct MacroMatcher {
    /// The pattern (list of literals and holes)
    pub pattern: Vec<MacroVarExt>,
}
impl MacroMatcher {
    /// Create a new matcher from a space-separated pattern string.
    #[allow(dead_code)]
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Self {
        MacroMatcher {
            pattern: s
                .split_whitespace()
                .map(|tok| {
                    if let Some(name) = tok.strip_prefix('$') {
                        MacroVarExt::simple(name)
                    } else {
                        MacroVarExt {
                            name: format!("__lit_{}", tok),
                            is_rep: false,
                        }
                    }
                })
                .collect(),
        }
    }
    /// Returns the number of holes.
    #[allow(dead_code)]
    pub fn hole_count(&self) -> usize {
        self.pattern
            .iter()
            .filter(|v| !v.name.starts_with("__lit_"))
            .count()
    }
    /// Returns the number of literal tokens.
    #[allow(dead_code)]
    pub fn literal_count(&self) -> usize {
        self.pattern
            .iter()
            .filter(|v| v.name.starts_with("__lit_"))
            .count()
    }
}
/// An error that occurred during macro expansion.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Clone)]
pub struct MacroExpansionErrorExt2 {
    /// Error message
    pub message: String,
    /// Macro name
    pub macro_name: String,
    /// Expansion depth at which error occurred
    pub depth: usize,
}
impl MacroExpansionErrorExt2 {
    /// Create a new expansion error.
    #[allow(dead_code)]
    pub fn new(macro_name: &str, message: &str, depth: usize) -> Self {
        MacroExpansionErrorExt2 {
            message: message.to_string(),
            macro_name: macro_name.to_string(),
            depth,
        }
    }
    /// Format the error as a string.
    #[allow(dead_code)]
    pub fn format(&self) -> String {
        format!(
            "macro '{}' at depth {}: {}",
            self.macro_name, self.depth, self.message
        )
    }
}
/// A macro expansion trace for debugging.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Default)]
pub struct MacroExpansionTraceExt {
    /// Steps in the trace
    pub steps: Vec<String>,
}
impl MacroExpansionTraceExt {
    /// Create a new empty trace.
    #[allow(dead_code)]
    pub fn new() -> Self {
        MacroExpansionTraceExt { steps: Vec::new() }
    }
    /// Record an expansion step.
    #[allow(dead_code)]
    pub fn record(&mut self, step: &str) {
        self.steps.push(step.to_string());
    }
    /// Format the trace as a string.
    #[allow(dead_code)]
    pub fn format(&self) -> String {
        self.steps
            .iter()
            .enumerate()
            .map(|(i, s)| format!("{}: {}", i, s))
            .collect::<Vec<_>>()
            .join("\n")
    }
}
/// A macro error during expansion.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Clone)]
pub struct MacroExpansionError {
    /// The macro that failed
    pub macro_name: String,
    /// The error message
    pub message: String,
    /// Expansion depth at time of error
    pub depth: usize,
}
impl MacroExpansionError {
    /// Create a new expansion error.
    #[allow(dead_code)]
    pub fn new(macro_name: &str, message: &str, depth: usize) -> Self {
        MacroExpansionError {
            macro_name: macro_name.to_string(),
            message: message.to_string(),
            depth,
        }
    }
    /// Format the error.
    #[allow(dead_code)]
    pub fn format(&self) -> String {
        format!(
            "macro error in '{}' at depth {}: {}",
            self.macro_name, self.depth, self.message
        )
    }
}
/// Kinds of macro errors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MacroErrorKind {
    /// The named macro is not registered.
    UnknownMacro,
    /// No rule pattern matched the input.
    PatternMismatch,
    /// A hygiene violation was detected.
    HygieneViolation,
    /// Multiple rules matched ambiguously.
    AmbiguousMatch,
    /// An error occurred during template expansion.
    ExpansionError,
}
/// A depth-limited macro expander.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct DepthLimitedExpanderExt2 {
    /// Maximum expansion depth
    pub max_depth: usize,
    /// Current depth
    pub current_depth: usize,
}
impl DepthLimitedExpanderExt2 {
    /// Create a new expander with the given limit.
    #[allow(dead_code)]
    pub fn new(max_depth: usize) -> Self {
        DepthLimitedExpanderExt2 {
            max_depth,
            current_depth: 0,
        }
    }
    /// Returns true if we can still expand (not at limit).
    #[allow(dead_code)]
    pub fn can_expand(&self) -> bool {
        self.current_depth < self.max_depth
    }
    /// Enter a new level of expansion.
    #[allow(dead_code)]
    pub fn enter(&mut self) -> bool {
        if self.current_depth >= self.max_depth {
            return false;
        }
        self.current_depth += 1;
        true
    }
    /// Exit a level of expansion.
    #[allow(dead_code)]
    pub fn exit(&mut self) {
        if self.current_depth > 0 {
            self.current_depth -= 1;
        }
    }
}
/// The macro expansion engine.
///
/// Maintains a registry of macro definitions and provides expansion services.
pub struct MacroExpander {
    /// Registered macro definitions keyed by name.
    macros: HashMap<String, MacroDef>,
    /// Next available scope id for hygiene.
    next_scope: u64,
    /// Registered syntax definitions.
    syntax_defs: Vec<SyntaxDef>,
    /// Maximum expansion depth (to guard against infinite recursion).
    pub(super) max_depth: u32,
}
impl MacroExpander {
    /// Create a new macro expander.
    pub fn new() -> Self {
        Self {
            macros: HashMap::new(),
            next_scope: 1,
            syntax_defs: Vec::new(),
            max_depth: 128,
        }
    }
    /// Set the maximum expansion depth.
    pub fn set_max_depth(&mut self, depth: u32) {
        self.max_depth = depth;
    }
    /// Generate a fresh scope id for hygiene.
    pub fn fresh_scope(&mut self) -> u64 {
        let id = self.next_scope;
        self.next_scope += 1;
        id
    }
    /// Register a macro definition.
    pub fn register_macro(&mut self, def: MacroDef) {
        self.macros.insert(def.name.clone(), def);
    }
    /// Unregister a macro by name.
    pub fn unregister_macro(&mut self, name: &str) -> Option<MacroDef> {
        self.macros.remove(name)
    }
    /// Check whether a macro with the given name is registered.
    pub fn has_macro(&self, name: &str) -> bool {
        self.macros.contains_key(name)
    }
    /// Get a macro definition by name.
    pub fn get_macro(&self, name: &str) -> Option<&MacroDef> {
        self.macros.get(name)
    }
    /// Get the number of registered macros.
    pub fn macro_count(&self) -> usize {
        self.macros.len()
    }
    /// Register a syntax definition.
    pub fn register_syntax(&mut self, def: SyntaxDef) {
        self.syntax_defs.push(def);
    }
    /// Get all syntax definitions for a given kind.
    pub fn syntax_defs_for(&self, kind: &SyntaxKind) -> Vec<&SyntaxDef> {
        self.syntax_defs
            .iter()
            .filter(|d| &d.kind == kind)
            .collect()
    }
    /// Expand a macro invocation.
    ///
    /// Looks up the macro by `name`, tries each rule in order, and returns
    /// the expanded token sequence on the first match.
    #[allow(clippy::result_large_err)]
    pub fn expand(&mut self, name: &str, input: &[Token]) -> Result<Vec<Token>, MacroError> {
        self.expand_with_depth(name, input, 0)
    }
    /// Internal expansion with depth tracking.
    #[allow(clippy::result_large_err)]
    fn expand_with_depth(
        &mut self,
        name: &str,
        input: &[Token],
        depth: u32,
    ) -> Result<Vec<Token>, MacroError> {
        if depth > self.max_depth {
            return Err(MacroError::new(
                MacroErrorKind::ExpansionError,
                format!(
                    "maximum macro expansion depth ({}) exceeded for '{}'",
                    self.max_depth, name
                ),
            ));
        }
        let def = self.macros.get(name).cloned().ok_or_else(|| {
            MacroError::new(
                MacroErrorKind::UnknownMacro,
                format!("macro '{}' is not defined", name),
            )
        })?;
        let mut matches = Vec::new();
        for rule in &def.rules {
            if let Some(bindings) = try_match_rule(rule, input) {
                matches.push((rule.clone(), bindings));
            }
        }
        match matches.len() {
            0 => Err(MacroError::new(
                MacroErrorKind::PatternMismatch,
                format!(
                    "no rule of macro '{}' matches the input ({} tokens)",
                    name,
                    input.len()
                ),
            )),
            1 => {
                let (rule, bindings) = matches
                    .into_iter()
                    .next()
                    .expect("matches.len() == 1 per match arm");
                let binding_slice: Vec<(String, Vec<Token>)> = bindings;
                Ok(substitute(&rule.template, &binding_slice))
            }
            _ => Err(MacroError::new(
                MacroErrorKind::AmbiguousMatch,
                format!(
                    "{} rules of macro '{}' match the input",
                    matches.len(),
                    name
                ),
            )),
        }
    }
    /// List all registered macro names.
    pub fn macro_names(&self) -> Vec<&str> {
        let mut names: Vec<&str> = self.macros.keys().map(|s| s.as_str()).collect();
        names.sort();
        names
    }
}
/// The result of a macro expansion attempt.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Clone)]
pub enum MacroExpansionResultExt2 {
    /// Expansion succeeded, result is the expanded string
    Success(String),
    /// Expansion failed with an error
    Error(MacroExpansionErrorExt2),
    /// No rule matched
    NoMatch,
}
impl MacroExpansionResultExt2 {
    /// Returns true if expansion succeeded.
    #[allow(dead_code)]
    pub fn is_success(&self) -> bool {
        matches!(self, MacroExpansionResultExt2::Success(_))
    }
    /// Returns the expanded string if successful.
    #[allow(dead_code)]
    pub fn as_str(&self) -> Option<&str> {
        if let MacroExpansionResultExt2::Success(s) = self {
            Some(s)
        } else {
            None
        }
    }
}
/// A macro template node (either a literal token or a variable reference).
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Clone)]
pub enum MacroTemplateNodeExt {
    /// A literal token
    Literal(String),
    /// A variable reference
    Var(MacroVarExt),
    /// A repetition block: separator and repeated body
    Rep {
        sep: Option<String>,
        body: Vec<MacroTemplateNodeExt>,
    },
    /// A parenthesised group
    Group(Vec<MacroTemplateNodeExt>),
}
/// A macro signature: name + param count.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Clone)]
pub struct MacroSignature {
    /// Macro name
    pub name: String,
    /// Number of parameters
    pub param_count: usize,
}
impl MacroSignature {
    /// Create a new signature.
    #[allow(dead_code)]
    pub fn new(name: &str, param_count: usize) -> Self {
        MacroSignature {
            name: name.to_string(),
            param_count,
        }
    }
    /// Format as "name/N".
    #[allow(dead_code)]
    pub fn format(&self) -> String {
        format!("{}/{}", self.name, self.param_count)
    }
}
/// A macro definition with a name, parameter list, and template.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Clone)]
pub struct MacroDefinitionExt {
    /// Macro name
    pub name: String,
    /// Parameter names
    pub params: Vec<String>,
    /// Template string
    pub template: String,
}
impl MacroDefinitionExt {
    /// Create a new macro definition.
    #[allow(dead_code)]
    pub fn new(name: &str, params: Vec<&str>, template: &str) -> Self {
        MacroDefinitionExt {
            name: name.to_string(),
            params: params.into_iter().map(|s| s.to_string()).collect(),
            template: template.to_string(),
        }
    }
    /// Expand this macro with given arguments.
    #[allow(dead_code)]
    pub fn expand(&self, args: &[&str]) -> String {
        if args.len() != self.params.len() {
            return format!("(error: wrong arity for {})", self.name);
        }
        let mut env = std::collections::HashMap::new();
        for (p, a) in self.params.iter().zip(args.iter()) {
            env.insert(p.clone(), a.to_string());
        }
        let nodes = parse_simple_template_ext(&self.template);
        expand_template_ext(&nodes, &env)
    }
}
/// Macro expansion statistics.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Default)]
pub struct MacroStatsExt2 {
    /// Total expansions attempted
    pub attempts: usize,
    /// Total successful expansions
    pub successes: usize,
    /// Total failures
    pub failures: usize,
    /// Maximum depth reached
    pub max_depth: usize,
}
impl MacroStatsExt2 {
    /// Record a successful expansion.
    #[allow(dead_code)]
    pub fn record_success(&mut self, depth: usize) {
        self.attempts += 1;
        self.successes += 1;
        if depth > self.max_depth {
            self.max_depth = depth;
        }
    }
    /// Record a failed expansion.
    #[allow(dead_code)]
    pub fn record_failure(&mut self) {
        self.attempts += 1;
        self.failures += 1;
    }
    /// Returns the success rate as a fraction.
    #[allow(dead_code)]
    pub fn success_rate(&self) -> f64 {
        if self.attempts == 0 {
            return 1.0;
        }
        self.successes as f64 / self.attempts as f64
    }
}
/// Kind of syntactic category that a `syntax` declaration extends.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SyntaxKind {
    /// Term-level syntax.
    Term,
    /// Top-level command syntax.
    Command,
    /// Tactic syntax.
    Tactic,
    /// Universe level syntax.
    Level,
    /// Attribute syntax.
    Attr,
}
/// A hygiene context tracks fresh name generation for macro expansion.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Default)]
pub struct HygieneContextExt2 {
    /// Counter for generating fresh names
    pub counter: u32,
    /// Prefix for generated names
    pub prefix: String,
}
impl HygieneContextExt2 {
    /// Create a new hygiene context.
    #[allow(dead_code)]
    pub fn new(prefix: &str) -> Self {
        HygieneContextExt2 {
            counter: 0,
            prefix: prefix.to_string(),
        }
    }
    /// Generate a fresh name.
    #[allow(dead_code)]
    pub fn fresh(&mut self) -> String {
        let name = format!("{}{}", self.prefix, self.counter);
        self.counter += 1;
        name
    }
    /// Returns the number of names generated so far.
    #[allow(dead_code)]
    pub fn generated_count(&self) -> u32 {
        self.counter
    }
}
/// Token in macro pattern or template.
#[derive(Debug, Clone, PartialEq)]
pub enum MacroToken {
    /// Literal token that must match exactly.
    Literal(TokenKind),
    /// Variable (matches any expression).
    Var(String),
    /// Repetition (zero or more of the sub-pattern).
    Repeat(Vec<MacroToken>),
    /// Optional (zero or one of the sub-pattern).
    Optional(Vec<MacroToken>),
    /// Syntax quotation (`` `(expr) ``).
    Quote(Vec<MacroToken>),
    /// Antiquotation splice (`$x`).
    Antiquote(String),
    /// Array splice (`$[x]*`).
    SpliceArray(String),
}
/// A single macro rewrite rule: pattern => template.
#[derive(Debug, Clone)]
pub struct MacroRule {
    /// Pattern to match.
    pub pattern: Vec<MacroToken>,
    /// Expansion template.
    pub template: Vec<MacroToken>,
}
/// Hygiene information for macro expansion.
///
/// Tracks the scope and definition site so that names introduced by the macro
/// do not accidentally capture or shadow names in the call site.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HygieneInfo {
    /// Unique scope identifier for this macro expansion.
    pub scope_id: u64,
    /// The source location where the macro was defined.
    pub def_site: Span,
}
impl HygieneInfo {
    /// Create new hygiene info.
    pub fn new(scope_id: u64, def_site: Span) -> Self {
        Self { scope_id, def_site }
    }
}
/// A `syntax` declaration that extends a syntactic category.
#[derive(Debug, Clone)]
pub struct SyntaxDef {
    /// Name of the syntax extension.
    pub name: String,
    /// Which syntactic category it extends.
    pub kind: SyntaxKind,
    /// The parser specification.
    pub parser: Vec<SyntaxItem>,
}
impl SyntaxDef {
    /// Create a new syntax definition.
    pub fn new(name: String, kind: SyntaxKind, parser: Vec<SyntaxItem>) -> Self {
        Self { name, kind, parser }
    }
    /// Number of parser items.
    pub fn item_count(&self) -> usize {
        self.parser.len()
    }
}
/// A macro statistics tracker.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Default)]
pub struct MacroStats {
    /// Number of expansions
    pub expansions: usize,
    /// Number of expansion errors
    pub errors: usize,
    /// Maximum expansion depth reached
    pub max_depth: usize,
}
impl MacroStats {
    /// Create a new empty stats record.
    #[allow(dead_code)]
    pub fn new() -> Self {
        MacroStats::default()
    }
    /// Record a successful expansion.
    #[allow(dead_code)]
    pub fn record_success(&mut self, depth: usize) {
        self.expansions += 1;
        if depth > self.max_depth {
            self.max_depth = depth;
        }
    }
    /// Record an error.
    #[allow(dead_code)]
    pub fn record_error(&mut self) {
        self.errors += 1;
    }
}
/// Macro parser for parsing macro definitions.
pub struct MacroParser {
    /// Token stream.
    tokens: Vec<Token>,
    /// Current position.
    pub(super) pos: usize,
}
impl MacroParser {
    /// Create a new macro parser.
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }
    /// Get current token.
    fn current(&self) -> &Token {
        self.tokens
            .get(self.pos)
            .unwrap_or(&self.tokens[self.tokens.len() - 1])
    }
    /// Check if at end of input.
    fn is_eof(&self) -> bool {
        matches!(self.current().kind, TokenKind::Eof)
    }
    /// Advance to next token.
    fn advance(&mut self) -> Token {
        let tok = self.current().clone();
        if !self.is_eof() {
            self.pos += 1;
        }
        tok
    }
    /// Parse a macro rule (pattern => template).
    pub fn parse_rule(&mut self) -> Result<MacroRule, ParseError> {
        let pattern = self.parse_macro_tokens()?;
        if !matches!(self.current().kind, TokenKind::Arrow) {
            return Err(ParseError::unexpected(
                vec!["=>".to_string()],
                self.current().kind.clone(),
                self.current().span.clone(),
            ));
        }
        self.advance();
        let template = self.parse_macro_tokens()?;
        Ok(MacroRule { pattern, template })
    }
    /// Parse multiple macro rules separated by `|`.
    pub fn parse_rules(&mut self) -> Result<Vec<MacroRule>, ParseError> {
        let mut rules = Vec::new();
        rules.push(self.parse_rule()?);
        while !self.is_eof() && matches!(self.current().kind, TokenKind::Bar) {
            self.advance();
            rules.push(self.parse_rule()?);
        }
        Ok(rules)
    }
    /// Parse macro tokens until a delimiter is reached.
    fn parse_macro_tokens(&mut self) -> Result<Vec<MacroToken>, ParseError> {
        let mut tokens = Vec::new();
        while !self.is_eof() && !self.is_delimiter() {
            tokens.push(self.parse_macro_token()?);
        }
        Ok(tokens)
    }
    /// Parse a single macro token.
    pub(super) fn parse_macro_token(&mut self) -> Result<MacroToken, ParseError> {
        let tok = self.current();
        match &tok.kind {
            TokenKind::Ident(name) if name.starts_with("$[") && name.ends_with("]*") => {
                let inner = name[2..name.len() - 2].to_string();
                self.advance();
                Ok(MacroToken::SpliceArray(inner))
            }
            TokenKind::Ident(name) if name.starts_with('$') => {
                let var_name = name[1..].to_string();
                self.advance();
                if var_name.is_empty() && matches!(self.current().kind, TokenKind::LParen) {
                    self.advance();
                    let inner = self.parse_group_tokens()?;
                    if matches!(self.current().kind, TokenKind::RParen) {
                        self.advance();
                    }
                    return match &self.current().kind {
                        TokenKind::Star => {
                            self.advance();
                            Ok(MacroToken::Repeat(inner))
                        }
                        TokenKind::Question => {
                            self.advance();
                            Ok(MacroToken::Optional(inner))
                        }
                        _ => Ok(MacroToken::Repeat(inner)),
                    };
                }
                if !var_name.is_empty()
                    && !self.is_eof()
                    && matches!(self.current().kind, TokenKind::LBracket)
                    && self.pos + 2 < self.tokens.len()
                    && matches!(self.tokens[self.pos + 1].kind, TokenKind::RBracket)
                    && matches!(self.tokens[self.pos + 2].kind, TokenKind::Star)
                {
                    self.advance();
                    self.advance();
                    self.advance();
                    return Ok(MacroToken::SpliceArray(var_name));
                }
                Ok(MacroToken::Var(var_name))
            }
            TokenKind::Ident(name) if name == "`" => {
                self.advance();
                if matches!(self.current().kind, TokenKind::LParen) {
                    self.advance();
                    let inner = self.parse_quoted_tokens()?;
                    if matches!(self.current().kind, TokenKind::RParen) {
                        self.advance();
                    }
                    Ok(MacroToken::Quote(inner))
                } else {
                    Ok(MacroToken::Literal(TokenKind::Ident("`".to_string())))
                }
            }
            TokenKind::Star => {
                let kind = tok.kind.clone();
                self.advance();
                Ok(MacroToken::Literal(kind))
            }
            TokenKind::LParen => {
                let kind = tok.kind.clone();
                self.advance();
                Ok(MacroToken::Literal(kind))
            }
            _ => {
                let kind = tok.kind.clone();
                self.advance();
                Ok(MacroToken::Literal(kind))
            }
        }
    }
    /// Parse tokens inside a `$(...)` repetition/optional group until the matching `)`.
    ///
    /// Handles nested parentheses so that inner `(...)` groups are consumed correctly.
    fn parse_group_tokens(&mut self) -> Result<Vec<MacroToken>, ParseError> {
        let mut tokens = Vec::new();
        let mut depth = 1u32;
        while !self.is_eof() {
            match &self.current().kind {
                TokenKind::LParen => {
                    depth += 1;
                    let kind = self.current().kind.clone();
                    self.advance();
                    tokens.push(MacroToken::Literal(kind));
                }
                TokenKind::RParen => {
                    depth -= 1;
                    if depth == 0 {
                        break;
                    }
                    let kind = self.current().kind.clone();
                    self.advance();
                    tokens.push(MacroToken::Literal(kind));
                }
                _ => {
                    tokens.push(self.parse_macro_token()?);
                }
            }
        }
        Ok(tokens)
    }
    /// Parse tokens inside a quotation until `)`.
    fn parse_quoted_tokens(&mut self) -> Result<Vec<MacroToken>, ParseError> {
        let mut tokens = Vec::new();
        let mut depth = 1u32;
        while !self.is_eof() {
            if matches!(self.current().kind, TokenKind::LParen) {
                depth += 1;
            } else if matches!(self.current().kind, TokenKind::RParen) {
                depth -= 1;
                if depth == 0 {
                    break;
                }
            }
            tokens.push(self.parse_macro_token()?);
        }
        Ok(tokens)
    }
    /// Check if current token is a delimiter for macro token parsing.
    fn is_delimiter(&self) -> bool {
        matches!(
            self.current().kind,
            TokenKind::Arrow | TokenKind::Semicolon | TokenKind::RParen | TokenKind::Bar
        )
    }
}
/// A macro expansion depth limiter.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct DepthLimitedExpander {
    /// Maximum expansion depth
    pub max_depth: usize,
    /// Current depth
    pub current_depth: usize,
}
impl DepthLimitedExpander {
    /// Create a new expander.
    #[allow(dead_code)]
    pub fn new(max_depth: usize) -> Self {
        DepthLimitedExpander {
            max_depth,
            current_depth: 0,
        }
    }
    /// Try to expand (returns false if at limit).
    #[allow(dead_code)]
    pub fn try_expand(&mut self) -> bool {
        if self.current_depth >= self.max_depth {
            return false;
        }
        self.current_depth += 1;
        true
    }
    /// Exit a level.
    #[allow(dead_code)]
    pub fn exit(&mut self) {
        if self.current_depth > 0 {
            self.current_depth -= 1;
        }
    }
}
/// A hygiene context for macro expansion.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Clone)]
pub struct HygieneContext {
    /// Current expansion depth
    pub depth: usize,
    /// A unique tag for this expansion
    pub tag: u64,
}
impl HygieneContext {
    /// Create a new hygiene context.
    #[allow(dead_code)]
    pub fn new(tag: u64) -> Self {
        HygieneContext { depth: 0, tag }
    }
    /// Enter a nested expansion.
    #[allow(dead_code)]
    pub fn nested(&self) -> Self {
        HygieneContext {
            depth: self.depth + 1,
            tag: self.tag,
        }
    }
    /// Generate a hygienic name.
    #[allow(dead_code)]
    pub fn hygienic_name(&self, name: &str) -> String {
        format!("{}__{}_{}", name, self.tag, self.depth)
    }
}
/// A macro call site record.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Clone)]
pub struct MacroCallSiteExt {
    /// Name of the macro being called
    pub macro_name: String,
    /// Arguments provided
    pub args: Vec<String>,
    /// Byte offset of the call site
    pub offset: usize,
}
impl MacroCallSiteExt {
    /// Create a new call site.
    #[allow(dead_code)]
    pub fn new(macro_name: &str, args: Vec<&str>, offset: usize) -> Self {
        MacroCallSiteExt {
            macro_name: macro_name.to_string(),
            args: args.into_iter().map(|s| s.to_string()).collect(),
            offset,
        }
    }
}
/// An error that occurred during macro processing.
#[derive(Debug, Clone)]
pub struct MacroError {
    /// The kind of error.
    pub kind: MacroErrorKind,
    /// Optional source span where the error occurred.
    pub span: Option<Span>,
    /// Human-readable error message.
    pub message: String,
}
impl MacroError {
    /// Create a new macro error.
    pub fn new(kind: MacroErrorKind, message: String) -> Self {
        Self {
            kind,
            span: None,
            message,
        }
    }
    /// Attach a span to the error.
    pub fn with_span(mut self, span: Span) -> Self {
        self.span = Some(span);
        self
    }
}
/// A macro environment that stores all defined macros.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct MacroEnvironmentExt {
    /// All defined macros
    pub macros: Vec<MacroDefinitionExt>,
}
impl MacroEnvironmentExt {
    /// Create a new empty macro environment.
    #[allow(dead_code)]
    pub fn new() -> Self {
        MacroEnvironmentExt { macros: Vec::new() }
    }
    /// Define a new macro.
    #[allow(dead_code)]
    pub fn define(&mut self, def: MacroDefinitionExt) {
        self.macros.push(def);
    }
    /// Find a macro by name.
    #[allow(dead_code)]
    pub fn find(&self, name: &str) -> Option<&MacroDefinitionExt> {
        self.macros.iter().find(|m| m.name == name)
    }
    /// Expand a macro call.
    #[allow(dead_code)]
    pub fn expand_call(&self, name: &str, args: &[&str]) -> Option<String> {
        self.find(name).map(|m| m.expand(args))
    }
    /// Returns the number of defined macros.
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.macros.len()
    }
    /// Returns true if empty.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.macros.is_empty()
    }
}
