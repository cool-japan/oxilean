//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_parse::{Decl, Located, Span, SurfaceExpr};
use std::collections::{HashMap, HashSet};

use super::functions::{offset_to_line_col, LintRule};

/// A single text edit (replace a range with new text).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TextEdit {
    /// The range to replace.
    pub range: SourceRange,
    /// The new text to insert.
    pub new_text: String,
}
impl TextEdit {
    /// Create a new text edit.
    pub fn new(range: SourceRange, new_text: String) -> Self {
        Self { range, new_text }
    }
    /// Create a deletion edit.
    pub fn delete(range: SourceRange) -> Self {
        Self {
            range,
            new_text: String::new(),
        }
    }
    /// Create an insertion edit.
    pub fn insert(pos: usize, text: String) -> Self {
        Self {
            range: SourceRange::new(pos, pos),
            new_text: text,
        }
    }
}
/// Registry of all available lint rules.
///
/// The registry manages the collection of lint rules and provides
/// lookup by id, category, etc.
pub struct LintRegistry {
    /// All registered lint rules.
    rules: Vec<Box<dyn LintRule>>,
    /// Index by lint id.
    id_index: HashMap<LintId, usize>,
    /// Index by category.
    category_index: HashMap<LintCategory, Vec<usize>>,
}
impl LintRegistry {
    /// Create a new empty registry.
    pub fn new() -> Self {
        Self {
            rules: Vec::new(),
            id_index: HashMap::new(),
            category_index: HashMap::new(),
        }
    }
    /// Register a lint rule.
    pub fn register(&mut self, rule: Box<dyn LintRule>) {
        let idx = self.rules.len();
        let lint_id = rule.id();
        let category = rule.category();
        self.id_index.insert(lint_id, idx);
        self.category_index.entry(category).or_default().push(idx);
        self.rules.push(rule);
    }
    /// Look up a lint rule by id.
    pub fn get(&self, lint_id: &LintId) -> Option<&dyn LintRule> {
        self.id_index.get(lint_id).map(|&idx| &*self.rules[idx])
    }
    /// Get all lint rules.
    pub fn all_rules(&self) -> &[Box<dyn LintRule>] {
        &self.rules
    }
    /// Get all lint rules in a category.
    pub fn rules_in_category(&self, category: LintCategory) -> Vec<&dyn LintRule> {
        self.category_index
            .get(&category)
            .map(|indices| indices.iter().map(|&idx| &*self.rules[idx]).collect())
            .unwrap_or_default()
    }
    /// Get all lint ids.
    pub fn all_ids(&self) -> Vec<LintId> {
        self.rules.iter().map(|r| r.id()).collect()
    }
    /// Get the number of registered rules.
    pub fn len(&self) -> usize {
        self.rules.len()
    }
    /// Check if the registry is empty.
    pub fn is_empty(&self) -> bool {
        self.rules.is_empty()
    }
}
/// The main lint engine that orchestrates analysis.
///
/// The engine holds a registry of rules, a configuration, and provides
/// the `run` method to analyze a list of declarations.
pub struct LintEngine {
    /// Lint rule registry.
    registry: LintRegistry,
    /// Lint configuration.
    config: LintConfig,
}
impl LintEngine {
    /// Create a new lint engine.
    pub fn new(registry: LintRegistry, config: LintConfig) -> Self {
        Self { registry, config }
    }
    /// Create a lint engine with default configuration.
    pub fn with_defaults(registry: LintRegistry) -> Self {
        Self {
            registry,
            config: LintConfig::default(),
        }
    }
    /// Get the configuration.
    pub fn config(&self) -> &LintConfig {
        &self.config
    }
    /// Get a mutable reference to the configuration.
    pub fn config_mut(&mut self) -> &mut LintConfig {
        &mut self.config
    }
    /// Get the registry.
    pub fn registry(&self) -> &LintRegistry {
        &self.registry
    }
    /// Run lint analysis on a list of declarations.
    pub fn run(&self, source: &str, decls: &[Located<Decl>]) -> Vec<LintDiagnostic> {
        let mut ctx = LintContext::new(source, &self.config);
        let suppressions = SuppressionParser::parse_source(source);
        ctx.add_suppressions(suppressions);
        for decl in decls {
            self.collect_info(&mut ctx, decl);
        }
        for rule in self.registry.all_rules() {
            if !self.config.is_enabled(&rule.id()) {
                continue;
            }
            for decl in decls {
                rule.check_decl(&mut ctx, decl);
            }
            rule.finalize(&mut ctx);
        }
        ctx.take_diagnostics()
    }
    /// Run lint analysis on a single declaration.
    pub fn run_single(&self, source: &str, decl: &Located<Decl>) -> Vec<LintDiagnostic> {
        self.run(source, std::slice::from_ref(decl))
    }
    /// Collect information from a declaration for lint analysis.
    fn collect_info(&self, ctx: &mut LintContext<'_>, decl: &Located<Decl>) {
        match &decl.value {
            Decl::Definition { name, .. } => {
                ctx.register_name(name);
                ctx.push_decl(DeclInfo {
                    name: name.clone(),
                    kind: DeclKind::Definition,
                    range: SourceRange::from_span(&decl.span),
                    has_doc: false,
                    is_public: true,
                });
                ctx.pop_decl();
            }
            Decl::Theorem { name, .. } => {
                ctx.register_name(name);
                ctx.push_decl(DeclInfo {
                    name: name.clone(),
                    kind: DeclKind::Theorem,
                    range: SourceRange::from_span(&decl.span),
                    has_doc: false,
                    is_public: true,
                });
                ctx.pop_decl();
            }
            Decl::Axiom { name, .. } => {
                ctx.register_name(name);
            }
            Decl::Inductive { name, .. } => {
                ctx.register_name(name);
            }
            Decl::Structure { name, .. } => {
                ctx.register_name(name);
            }
            Decl::ClassDecl { name, .. } => {
                ctx.register_name(name);
            }
            Decl::Import { path } => {
                ctx.register_import(path.clone(), SourceRange::from_span(&decl.span));
            }
            Decl::Namespace { decls, .. } => {
                for inner_decl in decls {
                    self.collect_info(ctx, inner_decl);
                }
            }
            _ => {}
        }
    }
    /// Run expression-level lint on a single expression.
    pub fn check_expr(&self, source: &str, expr: &Located<SurfaceExpr>) -> Vec<LintDiagnostic> {
        let mut ctx = LintContext::new(source, &self.config);
        for rule in self.registry.all_rules() {
            if !self.config.is_enabled(&rule.id()) {
                continue;
            }
            rule.check_expr(&mut ctx, expr);
        }
        ctx.take_diagnostics()
    }
}
/// Kind of declaration.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum DeclKind {
    /// Definition.
    Definition,
    /// Theorem.
    Theorem,
    /// Inductive type.
    Inductive,
    /// Structure.
    Structure,
    /// Class.
    Class,
    /// Instance.
    Instance,
    /// Axiom.
    Axiom,
    /// Variable.
    Variable,
    /// Namespace.
    Namespace,
}
/// Parses inline lint suppression annotations from source comments.
///
/// Example annotations:
/// - `-- oxilean-ignore: unused_import`
/// - `-- oxilean-disable-next-line: naming_convention`
#[allow(dead_code)]
pub struct AnnotationParser;
impl AnnotationParser {
    /// Parse all suppression annotations from `source`.
    #[allow(dead_code)]
    pub fn parse(source: &str) -> Vec<ParsedAnnotation> {
        let mut annotations = Vec::new();
        for (line_idx, line) in source.lines().enumerate() {
            let t = line.trim();
            if let Some(rest) = t.strip_prefix("-- oxilean-ignore:") {
                let ids: Vec<String> = rest
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect();
                annotations.push(ParsedAnnotation {
                    kind: AnnotationKind::IgnoreLine,
                    lint_ids: ids,
                    line_number: line_idx + 1,
                });
            } else if let Some(rest) = t.strip_prefix("-- oxilean-disable-next-line:") {
                let ids: Vec<String> = rest
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect();
                annotations.push(ParsedAnnotation {
                    kind: AnnotationKind::DisableNextLine,
                    lint_ids: ids,
                    line_number: line_idx + 1,
                });
            } else if t == "-- oxilean-enable-all" {
                annotations.push(ParsedAnnotation {
                    kind: AnnotationKind::EnableAll,
                    lint_ids: Vec::new(),
                    line_number: line_idx + 1,
                });
            } else if t == "-- oxilean-disable-all" {
                annotations.push(ParsedAnnotation {
                    kind: AnnotationKind::DisableAll,
                    lint_ids: Vec::new(),
                    line_number: line_idx + 1,
                });
            }
        }
        annotations
    }
    /// Count suppression annotations in source.
    #[allow(dead_code)]
    pub fn count_suppressions(source: &str) -> usize {
        Self::parse(source).len()
    }
}
/// Kind of parsed annotation.
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AnnotationKind {
    IgnoreLine,
    DisableNextLine,
    EnableAll,
    DisableAll,
}
/// A parsed lint suppression annotation.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct ParsedAnnotation {
    pub kind: AnnotationKind,
    pub lint_ids: Vec<String>,
    pub line_number: usize,
}
/// A lint suppression directive.
///
/// Suppressions can be placed at the file level, declaration level,
/// or inline (via comments like `-- @[nolint unused_variable]`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LintSuppression {
    /// The lint ids to suppress (empty means suppress all).
    pub lint_ids: Vec<LintId>,
    /// The scope of suppression.
    pub scope: SuppressionScope,
    /// Optional reason for suppression.
    pub reason: Option<String>,
}
impl LintSuppression {
    /// Create a suppression for a specific lint in a given scope.
    pub fn new(lint_id: LintId, scope: SuppressionScope) -> Self {
        Self {
            lint_ids: vec![lint_id],
            scope,
            reason: None,
        }
    }
    /// Create a suppression for multiple lints.
    pub fn multi(lint_ids: Vec<LintId>, scope: SuppressionScope) -> Self {
        Self {
            lint_ids,
            scope,
            reason: None,
        }
    }
    /// Create a global suppression for all lints.
    pub fn all(scope: SuppressionScope) -> Self {
        Self {
            lint_ids: Vec::new(),
            scope,
            reason: None,
        }
    }
    /// Attach a reason.
    pub fn with_reason(mut self, reason: impl Into<String>) -> Self {
        self.reason = Some(reason.into());
        self
    }
    /// Check if this suppression applies to a given lint id.
    pub fn suppresses(&self, lint_id: &LintId) -> bool {
        if self.lint_ids.is_empty() {
            return true;
        }
        self.lint_ids.iter().any(|id| id == lint_id)
    }
    /// Check if this suppression covers a given source range.
    pub fn covers_range(&self, range: &SourceRange) -> bool {
        match &self.scope {
            SuppressionScope::File => true,
            SuppressionScope::Declaration(decl_range) => decl_range.contains(range.start),
            SuppressionScope::Line(line_start) => {
                range.start >= *line_start && range.start < line_start + 200
            }
            SuppressionScope::Range(sup_range) => sup_range.overlaps(range),
        }
    }
}
/// Applies multiple auto-fixes to a source string.
pub struct FixApplier;
impl FixApplier {
    /// Apply all auto-fixes from diagnostics to a source string.
    ///
    /// Fixes are applied in reverse order to preserve offsets.
    pub fn apply_all(source: &str, diagnostics: &[LintDiagnostic]) -> String {
        let mut all_edits: Vec<&TextEdit> = diagnostics
            .iter()
            .filter_map(|d| d.fix.as_ref())
            .flat_map(|fix| fix.edits.iter())
            .collect();
        all_edits.sort_by(|a, b| b.range.start.cmp(&a.range.start));
        let mut result = source.to_string();
        let mut last_start = usize::MAX;
        for edit in &all_edits {
            if edit.range.end > last_start {
                continue;
            }
            let start = edit.range.start.min(result.len());
            let end = edit.range.end.min(result.len());
            result.replace_range(start..end, &edit.new_text);
            last_start = edit.range.start;
        }
        result
    }
    /// Apply only preferred fixes.
    pub fn apply_preferred(source: &str, diagnostics: &[LintDiagnostic]) -> String {
        let preferred: Vec<&LintDiagnostic> = diagnostics
            .iter()
            .filter(|d| d.fix.as_ref().map(|f| f.is_preferred).unwrap_or(false))
            .collect();
        Self::apply_all(source, &preferred.into_iter().cloned().collect::<Vec<_>>())
    }
}
/// Configuration for the lint engine.
///
/// Controls which lints are enabled, their severity overrides,
/// and global suppression patterns.
#[derive(Clone, Debug)]
pub struct LintConfig {
    /// Enabled lint ids (empty = all enabled).
    pub enabled: HashSet<LintId>,
    /// Disabled lint ids.
    pub disabled: HashSet<LintId>,
    /// Severity overrides (lint_id -> severity).
    pub severity_overrides: HashMap<LintId, Severity>,
    /// Maximum number of diagnostics to report.
    pub max_diagnostics: usize,
    /// Whether to include auto-fix suggestions.
    pub suggest_fixes: bool,
    /// Global suppression patterns.
    pub suppression_patterns: Vec<String>,
    /// Whether to report dead code.
    pub report_dead_code: bool,
    /// Whether to enforce naming conventions.
    pub enforce_naming: bool,
    /// Whether to enforce style rules.
    pub enforce_style: bool,
    /// Whether to enforce documentation.
    pub enforce_docs: bool,
    /// Minimum severity to report.
    pub min_severity: Severity,
}
impl LintConfig {
    /// Allow a specific lint (add to enabled set).
    pub fn allow(&mut self, id: LintId) {
        self.enabled.insert(id.clone());
        self.disabled.remove(&id);
    }
    /// Deny a specific lint (add to disabled set and enable it).
    pub fn deny(&mut self, id: LintId) {
        self.disabled.insert(id.clone());
        self.enabled.remove(&id);
    }
    /// Check if a lint is explicitly allowed.
    pub fn is_allowed(&self, id: &LintId) -> bool {
        self.enabled.contains(id)
    }
    /// Check if a lint is explicitly denied.
    pub fn is_denied(&self, id: &LintId) -> bool {
        self.disabled.contains(id)
    }
    /// Create a strict configuration (all rules, error severity).
    pub fn strict() -> Self {
        Self {
            enforce_docs: true,
            min_severity: Severity::Error,
            ..Default::default()
        }
    }
    /// Create a permissive configuration (only errors).
    pub fn permissive() -> Self {
        Self {
            enforce_naming: false,
            enforce_style: false,
            enforce_docs: false,
            report_dead_code: false,
            min_severity: Severity::Error,
            ..Default::default()
        }
    }
    /// Check if a lint is enabled.
    pub fn is_enabled(&self, lint_id: &LintId) -> bool {
        if self.disabled.contains(lint_id) {
            return false;
        }
        if self.enabled.is_empty() {
            return true;
        }
        self.enabled.contains(lint_id)
    }
    /// Get the effective severity for a lint (considering overrides).
    pub fn effective_severity(&self, lint_id: &LintId, default: Severity) -> Severity {
        self.severity_overrides
            .get(lint_id)
            .copied()
            .unwrap_or(default)
    }
    /// Enable a lint.
    pub fn enable(&mut self, lint_id: LintId) {
        self.disabled.remove(&lint_id);
        self.enabled.insert(lint_id);
    }
    /// Disable a lint.
    pub fn disable(&mut self, lint_id: LintId) {
        self.enabled.remove(&lint_id);
        self.disabled.insert(lint_id);
    }
    /// Override the severity of a lint.
    pub fn set_severity(&mut self, lint_id: LintId, severity: Severity) {
        self.severity_overrides.insert(lint_id, severity);
    }
    /// Check whether severity passes the minimum threshold.
    pub fn passes_severity_filter(&self, severity: Severity) -> bool {
        severity <= self.min_severity
    }
    /// Check whether a suppression pattern matches a lint id.
    pub fn is_suppressed_by_pattern(&self, lint_id: &LintId) -> bool {
        self.suppression_patterns
            .iter()
            .any(|pat| lint_id.matches_pattern(pat))
    }
}
/// A range in source code for diagnostics.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct SourceRange {
    /// Start byte offset.
    pub start: usize,
    /// End byte offset.
    pub end: usize,
    /// Optional file path.
    pub file: Option<String>,
}
impl SourceRange {
    /// Create a new source range.
    pub fn new(start: usize, end: usize) -> Self {
        Self {
            start,
            end,
            file: None,
        }
    }
    /// Create a source range with a file path.
    pub fn with_file(start: usize, end: usize, file: String) -> Self {
        Self {
            start,
            end,
            file: Some(file),
        }
    }
    /// Create from a parser span.
    pub fn from_span(span: &Span) -> Self {
        Self {
            start: span.start,
            end: span.end,
            file: None,
        }
    }
    /// Check if this range contains a position.
    pub fn contains(&self, pos: usize) -> bool {
        pos >= self.start && pos < self.end
    }
    /// Check if two ranges overlap.
    pub fn overlaps(&self, other: &SourceRange) -> bool {
        self.start < other.end && other.start < self.end
    }
    /// Merge two ranges into their union.
    pub fn merge(&self, other: &SourceRange) -> Self {
        Self {
            start: self.start.min(other.start),
            end: self.end.max(other.end),
            file: self.file.clone(),
        }
    }
    /// Length of the range in bytes.
    pub fn len(&self) -> usize {
        self.end.saturating_sub(self.start)
    }
    /// Whether the range is empty.
    pub fn is_empty(&self) -> bool {
        self.start >= self.end
    }
}
/// Information about an import.
#[derive(Clone, Debug)]
pub struct ImportInfo {
    /// Imported module path.
    pub path: Vec<String>,
    /// Source range of the import declaration.
    pub range: SourceRange,
    /// Whether any names from this import have been used.
    pub used: bool,
}
/// An auto-fix suggestion for a lint diagnostic.
///
/// Auto-fixes describe a concrete text edit that can be applied to resolve
/// the lint warning or error.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AutoFix {
    /// Human-readable description of the fix.
    pub message: String,
    /// The edits to apply.
    pub edits: Vec<TextEdit>,
    /// Whether this fix is safe to apply automatically.
    pub is_preferred: bool,
}
impl AutoFix {
    /// Create a new auto-fix with a single text replacement.
    pub fn replacement(message: impl Into<String>, range: SourceRange, new_text: String) -> Self {
        Self {
            message: message.into(),
            edits: vec![TextEdit { range, new_text }],
            is_preferred: true,
        }
    }
    /// Create a new auto-fix with a deletion.
    pub fn deletion(message: impl Into<String>, range: SourceRange) -> Self {
        Self {
            message: message.into(),
            edits: vec![TextEdit {
                range,
                new_text: String::new(),
            }],
            is_preferred: true,
        }
    }
    /// Create an auto-fix with an insertion.
    pub fn insertion(message: impl Into<String>, pos: usize, text: String) -> Self {
        Self {
            message: message.into(),
            edits: vec![TextEdit {
                range: SourceRange::new(pos, pos),
                new_text: text,
            }],
            is_preferred: true,
        }
    }
    /// Create a multi-edit auto-fix.
    pub fn multi_edit(message: impl Into<String>, edits: Vec<TextEdit>) -> Self {
        Self {
            message: message.into(),
            edits,
            is_preferred: false,
        }
    }
    /// Mark this fix as not preferred (requires user confirmation).
    pub fn not_preferred(mut self) -> Self {
        self.is_preferred = false;
        self
    }
    /// Sort edits in reverse order for safe application.
    pub fn sorted_edits(&self) -> Vec<&TextEdit> {
        let mut edits: Vec<&TextEdit> = self.edits.iter().collect();
        edits.sort_by(|a, b| b.range.start.cmp(&a.range.start));
        edits
    }
    /// Apply this fix to a source string.
    pub fn apply(&self, source: &str) -> String {
        let mut result = source.to_string();
        for edit in self.sorted_edits() {
            let start = edit.range.start.min(result.len());
            let end = edit.range.end.min(result.len());
            result.replace_range(start..end, &edit.new_text);
        }
        result
    }
}
/// The scope of a lint suppression.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SuppressionScope {
    /// File-level suppression (applies to entire file).
    File,
    /// Declaration-level suppression.
    Declaration(SourceRange),
    /// Single-line suppression.
    Line(usize),
    /// Range suppression.
    Range(SourceRange),
}
/// Context provided to lint rules during analysis.
///
/// The lint context carries information about the current AST node, the
/// surrounding scope, available names, and the source text.
pub struct LintContext<'a> {
    /// Source text of the file being analyzed.
    pub source: &'a str,
    /// File path.
    pub file_path: Option<&'a str>,
    /// The current declaration stack (for nested scopes).
    decl_stack: Vec<DeclInfo>,
    /// Known names in the environment.
    known_names: HashSet<String>,
    /// Variable usage tracking.
    var_uses: HashMap<String, Vec<SourceRange>>,
    /// Variable declarations.
    var_decls: HashMap<String, SourceRange>,
    /// Import declarations.
    imports: Vec<ImportInfo>,
    /// Accumulated diagnostics.
    diagnostics: Vec<LintDiagnostic>,
    /// Active suppressions.
    suppressions: Vec<LintSuppression>,
    /// Lint configuration.
    config: &'a LintConfig,
    /// Counter for generating unique ids.
    next_id: u64,
}
impl<'a> LintContext<'a> {
    /// Create a new lint context.
    pub fn new(source: &'a str, config: &'a LintConfig) -> Self {
        Self {
            source,
            file_path: None,
            decl_stack: Vec::new(),
            known_names: HashSet::new(),
            var_uses: HashMap::new(),
            var_decls: HashMap::new(),
            imports: Vec::new(),
            diagnostics: Vec::new(),
            suppressions: Vec::new(),
            config,
            next_id: 0,
        }
    }
    /// Set the file path.
    pub fn with_file_path(mut self, path: &'a str) -> Self {
        self.file_path = Some(path);
        self
    }
    /// Add a suppression.
    pub fn add_suppression(&mut self, suppression: LintSuppression) {
        self.suppressions.push(suppression);
    }
    /// Add multiple suppressions.
    pub fn add_suppressions(&mut self, suppressions: Vec<LintSuppression>) {
        self.suppressions.extend(suppressions);
    }
    /// Push a declaration onto the scope stack.
    pub fn push_decl(&mut self, info: DeclInfo) {
        self.decl_stack.push(info);
    }
    /// Pop a declaration from the scope stack.
    pub fn pop_decl(&mut self) -> Option<DeclInfo> {
        self.decl_stack.pop()
    }
    /// Get the current declaration depth.
    pub fn depth(&self) -> usize {
        self.decl_stack.len()
    }
    /// Get the current declaration.
    pub fn current_decl(&self) -> Option<&DeclInfo> {
        self.decl_stack.last()
    }
    /// Register a known name.
    pub fn register_name(&mut self, name: &str) {
        self.known_names.insert(name.to_string());
    }
    /// Register a variable declaration.
    pub fn register_var_decl(&mut self, name: &str, range: SourceRange) {
        self.var_decls.insert(name.to_string(), range);
    }
    /// Register a variable use.
    pub fn register_var_use(&mut self, name: &str, range: SourceRange) {
        self.var_uses
            .entry(name.to_string())
            .or_default()
            .push(range);
    }
    /// Register an import.
    pub fn register_import(&mut self, path: Vec<String>, range: SourceRange) {
        self.imports.push(ImportInfo {
            path,
            range,
            used: false,
        });
    }
    /// Mark an import as used by module path.
    pub fn mark_import_used(&mut self, path: &[String]) {
        for imp in &mut self.imports {
            if imp.path == path {
                imp.used = true;
            }
        }
    }
    /// Check if a name is known (registered in the environment).
    pub fn is_known_name(&self, name: &str) -> bool {
        self.known_names.contains(name)
    }
    /// Get the declaration range for a variable name.
    pub fn get_var_decl(&self, name: &str) -> Option<&SourceRange> {
        self.var_decls.get(name)
    }
    /// Get all uses of a variable.
    pub fn get_var_uses(&self, name: &str) -> &[SourceRange] {
        self.var_uses.get(name).map(|v| v.as_slice()).unwrap_or(&[])
    }
    /// Get unused variables (declared but never used).
    pub fn unused_variables(&self) -> Vec<(&str, &SourceRange)> {
        self.var_decls
            .iter()
            .filter(|(name, _)| {
                !name.starts_with('_') && !self.var_uses.contains_key(name.as_str())
            })
            .map(|(name, range)| (name.as_str(), range))
            .collect()
    }
    /// Get unused imports.
    pub fn unused_imports(&self) -> Vec<&ImportInfo> {
        self.imports.iter().filter(|imp| !imp.used).collect()
    }
    /// Emit a diagnostic.
    pub fn emit(&mut self, diagnostic: LintDiagnostic) {
        if self.is_suppressed(&diagnostic.lint_id, &diagnostic.range) {
            return;
        }
        if !self.config.passes_severity_filter(diagnostic.severity) {
            return;
        }
        if self.diagnostics.len() < self.config.max_diagnostics {
            self.diagnostics.push(diagnostic);
        }
    }
    /// Emit a simple warning.
    pub fn warn(&mut self, lint_id: &str, message: impl Into<String>, range: SourceRange) {
        self.emit(LintDiagnostic::new(
            LintId::new(lint_id),
            Severity::Warning,
            message,
            range,
        ));
    }
    /// Emit a simple error.
    pub fn error(&mut self, lint_id: &str, message: impl Into<String>, range: SourceRange) {
        self.emit(LintDiagnostic::new(
            LintId::new(lint_id),
            Severity::Error,
            message,
            range,
        ));
    }
    /// Emit a hint.
    pub fn hint(&mut self, lint_id: &str, message: impl Into<String>, range: SourceRange) {
        self.emit(LintDiagnostic::new(
            LintId::new(lint_id),
            Severity::Hint,
            message,
            range,
        ));
    }
    /// Check if a diagnostic is suppressed.
    pub fn is_suppressed(&self, lint_id: &LintId, range: &SourceRange) -> bool {
        if self.config.is_suppressed_by_pattern(lint_id) {
            return true;
        }
        self.suppressions
            .iter()
            .any(|s| s.suppresses(lint_id) && s.covers_range(range))
    }
    /// Get all accumulated diagnostics.
    pub fn diagnostics(&self) -> &[LintDiagnostic] {
        &self.diagnostics
    }
    /// Take all accumulated diagnostics.
    pub fn take_diagnostics(&mut self) -> Vec<LintDiagnostic> {
        std::mem::take(&mut self.diagnostics)
    }
    /// Count diagnostics by severity.
    pub fn count_by_severity(&self, severity: Severity) -> usize {
        self.diagnostics
            .iter()
            .filter(|d| d.severity == severity)
            .count()
    }
    /// Check if there are any errors.
    pub fn has_errors(&self) -> bool {
        self.diagnostics
            .iter()
            .any(|d| d.severity == Severity::Error)
    }
    /// Generate a unique id.
    pub fn next_id(&mut self) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        id
    }
    /// Get a source snippet for a range.
    pub fn source_snippet(&self, range: &SourceRange) -> &str {
        let start = range.start.min(self.source.len());
        let end = range.end.min(self.source.len());
        &self.source[start..end]
    }
}
/// Builder for constructing a fully-configured lint engine.
pub struct LintEngineBuilder {
    /// Registry under construction.
    registry: LintRegistry,
    /// Configuration under construction.
    config: LintConfig,
}
impl LintEngineBuilder {
    /// Create a new builder.
    pub fn new() -> Self {
        Self {
            registry: LintRegistry::new(),
            config: LintConfig::default(),
        }
    }
    /// Register a lint rule.
    pub fn rule(mut self, rule: Box<dyn LintRule>) -> Self {
        self.registry.register(rule);
        self
    }
    /// Set the configuration.
    pub fn config(mut self, config: LintConfig) -> Self {
        self.config = config;
        self
    }
    /// Enable a specific lint.
    pub fn enable(mut self, lint_id: LintId) -> Self {
        self.config.enable(lint_id);
        self
    }
    /// Disable a specific lint.
    pub fn disable(mut self, lint_id: LintId) -> Self {
        self.config.disable(lint_id);
        self
    }
    /// Set severity for a lint.
    pub fn severity(mut self, lint_id: LintId, severity: Severity) -> Self {
        self.config.set_severity(lint_id, severity);
        self
    }
    /// Build the lint engine.
    pub fn build(self) -> LintEngine {
        LintEngine::new(self.registry, self.config)
    }
}
/// Describes ordering dependencies between lint passes.
///
/// A pass listed in `requires` must run before this pass;
/// a pass listed in `conflicts` must NOT run in the same session.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct LintPassDependencies {
    /// Lint pass names that must have already run.
    pub requires: Vec<String>,
    /// Lint pass names that must not be active simultaneously.
    pub conflicts: Vec<String>,
}
#[allow(dead_code)]
impl LintPassDependencies {
    /// Create an empty dependency descriptor.
    pub fn new() -> Self {
        Self::default()
    }
    /// Add a required predecessor pass.
    pub fn require(mut self, pass: impl Into<String>) -> Self {
        self.requires.push(pass.into());
        self
    }
    /// Add a conflicting pass.
    pub fn conflict(mut self, pass: impl Into<String>) -> Self {
        self.conflicts.push(pass.into());
        self
    }
    /// Check whether a given pass name is listed as a requirement.
    pub fn needs(&self, pass: &str) -> bool {
        self.requires.iter().any(|r| r == pass)
    }
    /// Check whether a given pass name is listed as a conflict.
    pub fn conflicts_with(&self, pass: &str) -> bool {
        self.conflicts.iter().any(|c| c == pass)
    }
    /// Return `true` if there are no requirements or conflicts declared.
    pub fn is_empty(&self) -> bool {
        self.requires.is_empty() && self.conflicts.is_empty()
    }
}
/// Parser for lint suppression comments.
///
/// Recognizes patterns like:
/// - `-- @[nolint unused_variable]`
/// - `-- @[nolint unused_variable, naming_convention]`
/// - `-- @[nolint all]`
pub struct SuppressionParser;
impl SuppressionParser {
    /// Parse a comment string for lint suppressions.
    pub fn parse_comment(comment: &str, line_start: usize) -> Option<LintSuppression> {
        let trimmed = comment.trim();
        if !trimmed.starts_with("@[nolint") {
            return None;
        }
        let inner = trimmed.strip_prefix("@[nolint")?.strip_suffix(']')?.trim();
        if inner == "all" || inner.is_empty() {
            return Some(LintSuppression::all(SuppressionScope::Line(line_start)));
        }
        let lint_ids: Vec<LintId> = inner.split(',').map(|s| LintId::new(s.trim())).collect();
        Some(LintSuppression::multi(
            lint_ids,
            SuppressionScope::Line(line_start),
        ))
    }
    /// Parse all suppression comments from a source string.
    pub fn parse_source(source: &str) -> Vec<LintSuppression> {
        let mut suppressions = Vec::new();
        let mut offset = 0;
        for line in source.lines() {
            let trimmed = line.trim();
            if let Some(comment_start) = trimmed.strip_prefix("--") {
                if let Some(sup) = Self::parse_comment(comment_start.trim(), offset) {
                    suppressions.push(sup);
                }
            }
            offset += line.len() + 1;
        }
        suppressions
    }
}
/// Per-file lint statistics.
#[allow(dead_code)]
pub struct LintFileStats {
    pub file: String,
    pub total_lines: usize,
    pub diagnostic_count: usize,
    pub suppressed_count: usize,
}
impl LintFileStats {
    #[allow(dead_code)]
    pub fn new(file: &str, total_lines: usize) -> Self {
        Self {
            file: file.to_string(),
            total_lines,
            diagnostic_count: 0,
            suppressed_count: 0,
        }
    }
    #[allow(dead_code)]
    pub fn diagnostic_density(&self) -> f64 {
        if self.total_lines == 0 {
            return 0.0;
        }
        self.diagnostic_count as f64 / self.total_lines as f64
    }
    #[allow(dead_code)]
    pub fn suppression_rate(&self) -> f64 {
        let total = self.diagnostic_count + self.suppressed_count;
        if total == 0 {
            return 0.0;
        }
        self.suppressed_count as f64 / total as f64
    }
}
/// Information about a declaration in scope.
#[derive(Clone, Debug)]
pub struct DeclInfo {
    /// Declaration name.
    pub name: String,
    /// Declaration kind.
    pub kind: DeclKind,
    /// Source range.
    pub range: SourceRange,
    /// Whether this declaration has documentation.
    pub has_doc: bool,
    /// Whether this declaration is public.
    pub is_public: bool,
}
/// Formats lint diagnostics for display.
pub struct DiagnosticFormatter {
    /// Whether to use color in output.
    pub use_color: bool,
    /// Whether to show source snippets.
    pub show_snippets: bool,
    /// Whether to show fix suggestions.
    pub show_fixes: bool,
    /// Context lines to show around the diagnostic.
    pub context_lines: usize,
}
impl DiagnosticFormatter {
    /// Create a new formatter.
    pub fn new() -> Self {
        Self::default()
    }
    /// Format a single diagnostic.
    pub fn format(&self, diag: &LintDiagnostic, source: &str) -> String {
        let mut output = String::new();
        output.push_str(&format!(
            "{}: {} [{}]\n",
            diag.severity, diag.message, diag.lint_id
        ));
        let (line, col) = offset_to_line_col(source, diag.range.start);
        output.push_str(&format!("  --> {}:{}\n", line + 1, col + 1));
        if self.show_snippets && !source.is_empty() {
            if let Some(snippet) = self.get_snippet(source, diag.range.start, diag.range.end) {
                output.push_str(&snippet);
            }
        }
        for note in &diag.notes {
            output.push_str(&format!("  = note: {}\n", note));
        }
        if self.show_fixes {
            if let Some(ref fix) = diag.fix {
                output.push_str(&format!("  = fix: {}\n", fix.message));
            }
        }
        output
    }
    /// Format all diagnostics.
    pub fn format_all(&self, diagnostics: &[LintDiagnostic], source: &str) -> String {
        let mut output = String::new();
        for diag in diagnostics {
            output.push_str(&self.format(diag, source));
            output.push('\n');
        }
        let errors = diagnostics
            .iter()
            .filter(|d| d.severity == Severity::Error)
            .count();
        let warnings = diagnostics
            .iter()
            .filter(|d| d.severity == Severity::Warning)
            .count();
        let hints = diagnostics
            .iter()
            .filter(|d| d.severity == Severity::Hint || d.severity == Severity::Info)
            .count();
        output.push_str(&format!(
            "lint: {} error(s), {} warning(s), {} hint(s)\n",
            errors, warnings, hints
        ));
        output
    }
    /// Get a source snippet around a range.
    fn get_snippet(&self, source: &str, start: usize, end: usize) -> Option<String> {
        let lines: Vec<&str> = source.lines().collect();
        let (start_line, _) = offset_to_line_col(source, start);
        let (end_line, _) = offset_to_line_col(source, end.min(source.len().saturating_sub(1)));
        let from = start_line.saturating_sub(self.context_lines);
        let to = (end_line + self.context_lines + 1).min(lines.len());
        let mut snippet = String::new();
        for (i, line) in lines.iter().enumerate().skip(from).take(to - from) {
            snippet.push_str(&format!("{:>4} | {}\n", i + 1, line));
        }
        Some(snippet)
    }
}
/// Summary of lint analysis results.
#[derive(Clone, Debug, Default)]
pub struct LintSummary {
    /// Total number of diagnostics.
    pub total: usize,
    /// Number of errors.
    pub errors: usize,
    /// Number of warnings.
    pub warnings: usize,
    /// Number of info messages.
    pub info: usize,
    /// Number of hints.
    pub hints: usize,
    /// Number of auto-fixable issues.
    pub fixable: usize,
    /// Per-lint counts.
    pub per_lint: HashMap<LintId, usize>,
    /// Per-category counts.
    pub per_category: HashMap<LintCategory, usize>,
}
impl LintSummary {
    /// Create a summary from a list of diagnostics.
    pub fn from_diagnostics(diagnostics: &[LintDiagnostic]) -> Self {
        let mut errors = 0;
        let mut warnings = 0;
        let mut info = 0;
        let mut hints = 0;
        let mut fixable = 0;
        let mut per_lint = HashMap::new();
        for diag in diagnostics {
            match diag.severity {
                Severity::Error => errors += 1,
                Severity::Warning => warnings += 1,
                Severity::Info => info += 1,
                Severity::Hint => hints += 1,
            }
            if diag.fix.is_some() {
                fixable += 1;
            }
            *per_lint.entry(diag.lint_id.clone()).or_insert(0) += 1;
        }
        Self {
            total: diagnostics.len(),
            errors,
            warnings,
            info,
            hints,
            fixable,
            per_lint,
            per_category: HashMap::new(),
        }
    }
    /// Whether the analysis passed (no errors).
    pub fn passed(&self) -> bool {
        self.errors == 0
    }
}
/// Unique identifier for a lint rule.
///
/// Each lint is identified by a string key like `"unused_variable"` or
/// `"naming_convention"`. The identifier is used for suppression, configuration,
/// and error reporting.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct LintId(String);
impl LintId {
    /// Create a new lint identifier.
    pub fn new(id: impl Into<String>) -> Self {
        LintId(id.into())
    }
    /// Get the string representation of this lint id.
    pub fn as_str(&self) -> &str {
        &self.0
    }
    /// Check whether this lint id matches a given pattern (glob-like).
    ///
    /// Patterns like `"unused_*"` match any lint id starting with `"unused_"`.
    pub fn matches_pattern(&self, pattern: &str) -> bool {
        if pattern == "*" {
            return true;
        }
        if let Some(prefix) = pattern.strip_suffix('*') {
            return self.0.starts_with(prefix);
        }
        if let Some(suffix) = pattern.strip_prefix('*') {
            return self.0.ends_with(suffix);
        }
        self.0 == pattern
    }
}
/// Related information for a diagnostic.
#[derive(Clone, Debug)]
pub struct RelatedInfo {
    /// Source range of the related info.
    pub range: SourceRange,
    /// Explanation message.
    pub message: String,
}
/// Schedules which lint passes run in which order.
#[allow(dead_code)]
pub struct LintPassScheduler {
    /// Ordered list of pass names.
    schedule: Vec<String>,
    /// Disabled passes.
    disabled: std::collections::HashSet<String>,
}
impl LintPassScheduler {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            schedule: Vec::new(),
            disabled: std::collections::HashSet::new(),
        }
    }
    /// Add a pass to the end of the schedule.
    #[allow(dead_code)]
    pub fn add_pass(&mut self, name: &str) {
        self.schedule.push(name.to_string());
    }
    /// Disable a pass by name.
    #[allow(dead_code)]
    pub fn disable_pass(&mut self, name: &str) {
        self.disabled.insert(name.to_string());
    }
    /// Return the ordered list of enabled passes.
    #[allow(dead_code)]
    pub fn enabled_passes(&self) -> Vec<&str> {
        self.schedule
            .iter()
            .filter(|p| !self.disabled.contains(p.as_str()))
            .map(|p| p.as_str())
            .collect()
    }
    /// Number of scheduled passes (including disabled).
    #[allow(dead_code)]
    pub fn total_scheduled(&self) -> usize {
        self.schedule.len()
    }
}
/// Category for lint rules.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum LintCategory {
    /// Correctness: likely bugs or errors.
    Correctness,
    /// Style: naming, formatting, conventions.
    Style,
    /// Performance: potential performance issues.
    Performance,
    /// Complexity: overly complex code.
    Complexity,
    /// Documentation: missing or incomplete docs.
    Documentation,
    /// Deprecated: usage of deprecated features.
    Deprecated,
}
/// Severity level for lint diagnostics.
///
/// Severities are ordered from most severe (Error) to least severe (Hint).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Severity {
    /// An error that must be fixed.
    Error,
    /// A warning that should be addressed.
    Warning,
    /// Informational message.
    Info,
    /// A hint for possible improvement.
    Hint,
}
impl Severity {
    /// Returns the string representation of this severity.
    pub fn as_str(&self) -> &'static str {
        match self {
            Severity::Error => "error",
            Severity::Warning => "warning",
            Severity::Info => "info",
            Severity::Hint => "hint",
        }
    }
    /// Returns the numeric level (0 = error, 3 = hint).
    pub fn level(&self) -> u8 {
        match self {
            Severity::Error => 0,
            Severity::Warning => 1,
            Severity::Info => 2,
            Severity::Hint => 3,
        }
    }
    /// Parse from string.
    pub fn from_str_name(s: &str) -> Option<Self> {
        match s {
            "error" => Some(Severity::Error),
            "warning" | "warn" => Some(Severity::Warning),
            "info" => Some(Severity::Info),
            "hint" => Some(Severity::Hint),
            _ => None,
        }
    }
}
/// A diagnostic emitted by a lint rule.
///
/// Contains the lint id, severity, message, source range, optional fix,
/// and related information (e.g., "variable declared here").
#[derive(Clone, Debug)]
pub struct LintDiagnostic {
    /// The lint that emitted this diagnostic.
    pub lint_id: LintId,
    /// Severity level.
    pub severity: Severity,
    /// Human-readable message.
    pub message: String,
    /// Primary source range.
    pub range: SourceRange,
    /// Optional auto-fix suggestion.
    pub fix: Option<AutoFix>,
    /// Related information (secondary locations and messages).
    pub related: Vec<RelatedInfo>,
    /// Optional notes (extra explanations).
    pub notes: Vec<String>,
}
impl LintDiagnostic {
    /// Create a new lint diagnostic.
    pub fn new(
        lint_id: LintId,
        severity: Severity,
        message: impl Into<String>,
        range: SourceRange,
    ) -> Self {
        Self {
            lint_id,
            severity,
            message: message.into(),
            range,
            fix: None,
            related: Vec::new(),
            notes: Vec::new(),
        }
    }
    /// Builder method: attach an auto-fix.
    pub fn with_fix(mut self, fix: AutoFix) -> Self {
        self.fix = Some(fix);
        self
    }
    /// Builder method: add related information.
    pub fn with_related(mut self, range: SourceRange, message: impl Into<String>) -> Self {
        self.related.push(RelatedInfo {
            range,
            message: message.into(),
        });
        self
    }
    /// Builder method: add a note.
    pub fn with_note(mut self, note: impl Into<String>) -> Self {
        self.notes.push(note.into());
        self
    }
    /// Check if this diagnostic has an auto-fix.
    pub fn has_fix(&self) -> bool {
        self.fix.is_some()
    }
    /// Check if severity is error.
    pub fn is_error(&self) -> bool {
        self.severity == Severity::Error
    }
    /// Check if severity is warning.
    pub fn is_warning(&self) -> bool {
        self.severity == Severity::Warning
    }
}
