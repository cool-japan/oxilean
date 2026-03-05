//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lsp::{
    analyze_document, DiagnosticSeverity, Document, DocumentStore, JsonValue, Location, Range,
    SymbolKind, TextEdit,
};
use oxilean_kernel::{Environment, Name};
use oxilean_parse::{Lexer, TokenKind};
use std::collections::{HashMap, HashSet};

use super::functions::*;

/// Advanced diagnostic collector that performs multi-pass analysis.
pub struct AdvDiagnosticCollector<'a> {
    /// Reference to the kernel environment.
    env: &'a Environment,
    /// Severity configuration.
    severity_config: SeverityConfig,
    /// Maximum line length for style checks.
    max_line_length: usize,
}
impl<'a> AdvDiagnosticCollector<'a> {
    /// Create a new collector.
    pub fn new(env: &'a Environment) -> Self {
        Self {
            env,
            severity_config: SeverityConfig::default(),
            max_line_length: 120,
        }
    }
    /// Create with custom severity config.
    pub fn with_config(env: &'a Environment, config: SeverityConfig) -> Self {
        Self {
            env,
            severity_config: config,
            max_line_length: 120,
        }
    }
    /// Set the maximum line length for style checks.
    pub fn set_max_line_length(&mut self, length: usize) {
        self.max_line_length = length;
    }
    /// Collect all diagnostics for a document.
    pub fn collect_all(&self, doc: &Document) -> Vec<AdvDiagnostic> {
        let mut diagnostics = Vec::new();
        diagnostics.extend(self.collect_lex_errors(doc));
        diagnostics.extend(self.collect_parse_errors(doc));
        diagnostics.extend(self.collect_semantic_errors(doc));
        diagnostics.extend(self.collect_style_warnings(doc));
        self.generate_fixes(&mut diagnostics, doc);
        self.apply_severity_config(&mut diagnostics);
        self.enforce_limits(&mut diagnostics);
        diagnostics
    }
    /// Collect lexer-level errors.
    pub(crate) fn collect_lex_errors(&self, doc: &Document) -> Vec<AdvDiagnostic> {
        let mut diagnostics = Vec::new();
        let mut lexer = Lexer::new(&doc.content);
        let tokens = lexer.tokenize();
        for token in &tokens {
            if let TokenKind::Error(msg) = &token.kind {
                let line = token.span.line.saturating_sub(1) as u32;
                let col = token.span.column.saturating_sub(1) as u32;
                diagnostics.push(AdvDiagnostic {
                    code: AdvDiagnosticCode::LexError,
                    severity: DiagnosticSeverity::Error,
                    range: Range::single_line(line, col, col + 1),
                    message: format!("lexer error: {}", msg),
                    related: Vec::new(),
                    fixes: Vec::new(),
                    tags: Vec::new(),
                    uri: doc.uri.clone(),
                });
            }
        }
        diagnostics
    }
    /// Collect parse-level errors including bracket matching.
    fn collect_parse_errors(&self, doc: &Document) -> Vec<AdvDiagnostic> {
        let mut diagnostics = Vec::new();
        let mut lexer = Lexer::new(&doc.content);
        let tokens = lexer.tokenize();
        let mut stack: Vec<(char, u32, u32)> = Vec::new();
        for token in &tokens {
            let line = token.span.line.saturating_sub(1) as u32;
            let col = token.span.column.saturating_sub(1) as u32;
            match &token.kind {
                TokenKind::LParen => stack.push(('(', line, col)),
                TokenKind::LBracket => stack.push(('[', line, col)),
                TokenKind::LBrace => stack.push(('{', line, col)),
                TokenKind::RParen => {
                    if let Some((ch, open_line, open_col)) = stack.last() {
                        if *ch == '(' {
                            stack.pop();
                        } else {
                            diagnostics.push(AdvDiagnostic {
                                code: AdvDiagnosticCode::ParseError,
                                severity: DiagnosticSeverity::Error,
                                range: Range::single_line(line, col, col + 1),
                                message: format!(
                                    "mismatched ')'; expected '{}' to close",
                                    closing_for(*ch)
                                ),
                                related: vec![AdvRelatedInfo {
                                    message: format!("opening '{}' is here", ch),
                                    uri: doc.uri.clone(),
                                    range: Range::single_line(*open_line, *open_col, *open_col + 1),
                                }],
                                fixes: Vec::new(),
                                tags: Vec::new(),
                                uri: doc.uri.clone(),
                            });
                        }
                    } else {
                        diagnostics.push(AdvDiagnostic {
                            code: AdvDiagnosticCode::ParseError,
                            severity: DiagnosticSeverity::Error,
                            range: Range::single_line(line, col, col + 1),
                            message: "unmatched ')'".to_string(),
                            related: Vec::new(),
                            fixes: vec![AdvQuickFix {
                                title: "Remove unmatched ')'".to_string(),
                                kind: FixKind::QuickFix,
                                edits: vec![FixEdit {
                                    uri: doc.uri.clone(),
                                    range: Range::single_line(line, col, col + 1),
                                    new_text: String::new(),
                                }],
                                is_preferred: true,
                            }],
                            tags: Vec::new(),
                            uri: doc.uri.clone(),
                        });
                    }
                }
                TokenKind::RBracket => {
                    if let Some((ch, _, _)) = stack.last() {
                        if *ch == '[' {
                            stack.pop();
                        }
                    }
                }
                TokenKind::RBrace => {
                    if let Some((ch, _, _)) = stack.last() {
                        if *ch == '{' {
                            stack.pop();
                        }
                    }
                }
                _ => {}
            }
        }
        for (ch, line, col) in &stack {
            let closer = closing_for(*ch);
            diagnostics.push(AdvDiagnostic {
                code: AdvDiagnosticCode::ParseError,
                severity: DiagnosticSeverity::Error,
                range: Range::single_line(*line, *col, *col + 1),
                message: format!("unclosed '{}' (expected '{}')", ch, closer),
                related: Vec::new(),
                fixes: Vec::new(),
                tags: Vec::new(),
                uri: doc.uri.clone(),
            });
        }
        diagnostics
    }
    /// Collect semantic analysis errors: unused variables, unresolved names, etc.
    fn collect_semantic_errors(&self, doc: &Document) -> Vec<AdvDiagnostic> {
        let mut diagnostics = Vec::new();
        let analysis = analyze_document(&doc.uri, &doc.content, self.env);
        let mut lexer = Lexer::new(&doc.content);
        let tokens = lexer.tokenize();
        let mut usage_counts: HashMap<String, usize> = HashMap::new();
        let mut import_names: Vec<String> = Vec::new();
        for token in &tokens {
            if let TokenKind::Ident(name) = &token.kind {
                *usage_counts.entry(name.clone()).or_insert(0) += 1;
            }
        }
        let mut i = 0;
        while i < tokens.len() {
            if tokens[i].kind == TokenKind::Import {
                i += 1;
                let mut parts = Vec::new();
                while i < tokens.len() {
                    match &tokens[i].kind {
                        TokenKind::Ident(n) => parts.push(n.clone()),
                        TokenKind::Dot => {}
                        _ => break,
                    }
                    i += 1;
                }
                if !parts.is_empty() {
                    import_names.push(parts.join("."));
                }
            } else {
                i += 1;
            }
        }
        for def in &analysis.definitions {
            if !def.name.starts_with('_') {
                let count = usage_counts.get(&def.name).copied().unwrap_or(0);
                if count <= 1 {
                    diagnostics.push(AdvDiagnostic {
                        code: AdvDiagnosticCode::UnusedVariable,
                        severity: DiagnosticSeverity::Warning,
                        range: def.range.clone(),
                        message: format!("unused variable '{}'", def.name),
                        related: Vec::new(),
                        fixes: Vec::new(),
                        tags: vec![DiagnosticTag::Unnecessary],
                        uri: doc.uri.clone(),
                    });
                }
            }
        }
        for def in &analysis.definitions {
            let kernel_name = Name::str(&def.name);
            if self.env.contains(&kernel_name) {
                diagnostics.push(AdvDiagnostic {
                    code: AdvDiagnosticCode::Shadowing,
                    severity: DiagnosticSeverity::Warning,
                    range: def.range.clone(),
                    message: format!(
                        "'{}' shadows an existing declaration in the environment",
                        def.name
                    ),
                    related: Vec::new(),
                    fixes: Vec::new(),
                    tags: Vec::new(),
                    uri: doc.uri.clone(),
                });
            }
        }
        for token in &tokens {
            if let TokenKind::Ident(name) = &token.kind {
                if name == "sorry" {
                    let line = token.span.line.saturating_sub(1) as u32;
                    let col = token.span.column.saturating_sub(1) as u32;
                    diagnostics.push(AdvDiagnostic {
                        code: AdvDiagnosticCode::SorryUsed,
                        severity: DiagnosticSeverity::Warning,
                        range: Range::single_line(line, col, col + 5),
                        message: "'sorry' used -- proof is incomplete".to_string(),
                        related: Vec::new(),
                        fixes: Vec::new(),
                        tags: Vec::new(),
                        uri: doc.uri.clone(),
                    });
                }
            }
        }
        let mut seen_defs: HashMap<&str, &Range> = HashMap::new();
        for def in &analysis.definitions {
            if let Some(prev_range) = seen_defs.get(def.name.as_str()) {
                diagnostics.push(AdvDiagnostic {
                    code: AdvDiagnosticCode::DuplicateDefinition,
                    severity: DiagnosticSeverity::Error,
                    range: def.range.clone(),
                    message: format!("duplicate definition '{}'", def.name),
                    related: vec![AdvRelatedInfo {
                        message: "previous definition here".to_string(),
                        uri: doc.uri.clone(),
                        range: (*prev_range).clone(),
                    }],
                    fixes: Vec::new(),
                    tags: Vec::new(),
                    uri: doc.uri.clone(),
                });
            } else {
                seen_defs.insert(&def.name, &def.range);
            }
        }
        diagnostics
    }
    /// Collect style warnings.
    pub(crate) fn collect_style_warnings(&self, doc: &Document) -> Vec<AdvDiagnostic> {
        let mut diagnostics = Vec::new();
        let lines: Vec<&str> = doc.content.lines().collect();
        for (i, line) in lines.iter().enumerate() {
            let line_num = i as u32;
            if line.len() > self.max_line_length {
                diagnostics.push(AdvDiagnostic {
                    code: AdvDiagnosticCode::StyleLineLength,
                    severity: DiagnosticSeverity::Information,
                    range: Range::single_line(
                        line_num,
                        self.max_line_length as u32,
                        line.len() as u32,
                    ),
                    message: format!(
                        "line exceeds {} characters ({} chars)",
                        self.max_line_length,
                        line.len()
                    ),
                    related: Vec::new(),
                    fixes: Vec::new(),
                    tags: Vec::new(),
                    uri: doc.uri.clone(),
                });
            }
            let trimmed = line.trim_end();
            if trimmed.len() < line.len() && !trimmed.is_empty() {
                diagnostics.push(AdvDiagnostic {
                    code: AdvDiagnosticCode::StyleTrailingWhitespace,
                    severity: DiagnosticSeverity::Information,
                    range: Range::single_line(line_num, trimmed.len() as u32, line.len() as u32),
                    message: "trailing whitespace".to_string(),
                    related: Vec::new(),
                    fixes: vec![AdvQuickFix {
                        title: "Remove trailing whitespace".to_string(),
                        kind: FixKind::QuickFix,
                        edits: vec![FixEdit {
                            uri: doc.uri.clone(),
                            range: Range::single_line(
                                line_num,
                                trimmed.len() as u32,
                                line.len() as u32,
                            ),
                            new_text: String::new(),
                        }],
                        is_preferred: true,
                    }],
                    tags: vec![DiagnosticTag::Unnecessary],
                    uri: doc.uri.clone(),
                });
            }
        }
        let analysis = analyze_document(&doc.uri, &doc.content, self.env);
        for def in &analysis.definitions {
            match def.kind {
                SymbolKind::Function | SymbolKind::Method | SymbolKind::Variable => {
                    if def.name.starts_with(|c: char| c.is_uppercase())
                        && def.kind != SymbolKind::Constant
                    {
                        diagnostics
                            .push(AdvDiagnostic {
                                code: AdvDiagnosticCode::StyleNaming,
                                severity: DiagnosticSeverity::Information,
                                range: def.range.clone(),
                                message: format!(
                                    "'{}' should start with a lowercase letter (convention for definitions)",
                                    def.name
                                ),
                                related: Vec::new(),
                                fixes: Vec::new(),
                                tags: Vec::new(),
                                uri: doc.uri.clone(),
                            });
                    }
                }
                SymbolKind::Enum | SymbolKind::Struct | SymbolKind::Class => {
                    if def.name.starts_with(|c: char| c.is_lowercase()) {
                        diagnostics.push(AdvDiagnostic {
                            code: AdvDiagnosticCode::StyleNaming,
                            severity: DiagnosticSeverity::Information,
                            range: def.range.clone(),
                            message: format!(
                                "'{}' should start with an uppercase letter (convention for types)",
                                def.name
                            ),
                            related: Vec::new(),
                            fixes: Vec::new(),
                            tags: Vec::new(),
                            uri: doc.uri.clone(),
                        });
                    }
                }
                _ => {}
            }
        }
        diagnostics
    }
    /// Generate quick fixes for diagnostics that don't already have them.
    fn generate_fixes(&self, diagnostics: &mut [AdvDiagnostic], doc: &Document) {
        for diag in diagnostics.iter_mut() {
            if !diag.fixes.is_empty() {
                continue;
            }
            match diag.code {
                AdvDiagnosticCode::UnusedVariable => {
                    if let Some(line) = doc.get_line(diag.range.start.line) {
                        let start = diag.range.start.character as usize;
                        let end = (diag.range.end.character as usize).min(line.len());
                        if start < end && start < line.len() {
                            let name = &line[start..end];
                            if !name.starts_with('_') {
                                diag.fixes.push(AdvQuickFix {
                                    title: format!("Prefix with underscore: '_{}'", name),
                                    kind: FixKind::QuickFix,
                                    edits: vec![FixEdit {
                                        uri: doc.uri.clone(),
                                        range: diag.range.clone(),
                                        new_text: format!("_{}", name),
                                    }],
                                    is_preferred: true,
                                });
                            }
                        }
                    }
                }
                AdvDiagnosticCode::Shadowing => {
                    if let Some(line) = doc.get_line(diag.range.start.line) {
                        let start = diag.range.start.character as usize;
                        let end = (diag.range.end.character as usize).min(line.len());
                        if start < end && start < line.len() {
                            let name = &line[start..end];
                            diag.fixes.push(AdvQuickFix {
                                title: format!("Rename to \"{}'\"", name),
                                kind: FixKind::QuickFix,
                                edits: vec![FixEdit {
                                    uri: doc.uri.clone(),
                                    range: diag.range.clone(),
                                    new_text: format!("{}'", name),
                                }],
                                is_preferred: false,
                            });
                        }
                    }
                }
                AdvDiagnosticCode::SorryUsed => {
                    diag.fixes.push(AdvQuickFix {
                        title: "Replace with 'exact ?_'".to_string(),
                        kind: FixKind::QuickFix,
                        edits: vec![FixEdit {
                            uri: doc.uri.clone(),
                            range: diag.range.clone(),
                            new_text: "exact ?_".to_string(),
                        }],
                        is_preferred: false,
                    });
                }
                _ => {}
            }
        }
    }
    /// Apply severity configuration to diagnostics.
    fn apply_severity_config(&self, diagnostics: &mut Vec<AdvDiagnostic>) {
        diagnostics.retain_mut(|diag| {
            let code_str = diag.code.as_str();
            if let Some(effective) = self
                .severity_config
                .effective_severity(code_str, diag.severity)
            {
                diag.severity = effective;
                true
            } else {
                false
            }
        });
    }
    /// Enforce maximum diagnostic limits.
    fn enforce_limits(&self, diagnostics: &mut Vec<AdvDiagnostic>) {
        let mut error_count = 0usize;
        let mut warning_count = 0usize;
        diagnostics.retain(|diag| {
            match diag.severity {
                DiagnosticSeverity::Error => {
                    if error_count >= self.severity_config.max_errors_per_file {
                        return false;
                    }
                    error_count += 1;
                }
                DiagnosticSeverity::Warning => {
                    if warning_count >= self.severity_config.max_warnings_per_file {
                        return false;
                    }
                    warning_count += 1;
                }
                _ => {}
            }
            true
        });
    }
}
/// Filter diagnostics by minimum severity.
#[allow(dead_code)]
pub struct SeverityFilter {
    pub min_severity: DiagnosticSeverity,
}
/// Collects diagnostics across all files in a workspace.
pub struct ProjectDiagnostics<'a> {
    /// Reference to the environment.
    env: &'a Environment,
    /// Severity configuration.
    severity_config: SeverityConfig,
}
impl<'a> ProjectDiagnostics<'a> {
    /// Create a new project diagnostics collector.
    pub fn new(env: &'a Environment) -> Self {
        Self {
            env,
            severity_config: SeverityConfig::default(),
        }
    }
    /// Create with custom severity config.
    pub fn with_config(env: &'a Environment, config: SeverityConfig) -> Self {
        Self {
            env,
            severity_config: config,
        }
    }
    /// Collect diagnostics for all open documents.
    pub fn collect_all(&self, store: &DocumentStore) -> HashMap<String, Vec<AdvDiagnostic>> {
        let mut all_diagnostics = HashMap::new();
        for uri in store.uris() {
            if let Some(doc) = store.get_document(uri) {
                let collector =
                    AdvDiagnosticCollector::with_config(self.env, self.severity_config.clone());
                let diags = collector.collect_all(doc);
                all_diagnostics.insert(uri.clone(), diags);
            }
        }
        self.check_cross_file_issues(store, &mut all_diagnostics);
        all_diagnostics
    }
    /// Check for cross-file issues (duplicate definitions across files, etc.).
    fn check_cross_file_issues(
        &self,
        store: &DocumentStore,
        diagnostics: &mut HashMap<String, Vec<AdvDiagnostic>>,
    ) {
        let mut all_defs: HashMap<String, Vec<(String, Range)>> = HashMap::new();
        for uri in store.uris() {
            if let Some(doc) = store.get_document(uri) {
                let analysis = analyze_document(uri, &doc.content, self.env);
                for def in &analysis.definitions {
                    all_defs
                        .entry(def.name.clone())
                        .or_default()
                        .push((uri.clone(), def.range.clone()));
                }
            }
        }
        for (name, locations) in &all_defs {
            if locations.len() > 1 {
                for (uri, range) in locations {
                    let related: Vec<AdvRelatedInfo> = locations
                        .iter()
                        .filter(|(u, _)| u != uri)
                        .map(|(u, r)| AdvRelatedInfo {
                            message: format!("'{}' also defined here", name),
                            uri: u.clone(),
                            range: r.clone(),
                        })
                        .collect();
                    let diag = AdvDiagnostic {
                        code: AdvDiagnosticCode::DuplicateDefinition,
                        severity: DiagnosticSeverity::Warning,
                        range: range.clone(),
                        message: format!("'{}' is defined in {} files", name, locations.len()),
                        related,
                        fixes: Vec::new(),
                        tags: Vec::new(),
                        uri: uri.clone(),
                    };
                    diagnostics.entry(uri.clone()).or_default().push(diag);
                }
            }
        }
    }
    /// Get a summary of diagnostics across the project.
    pub fn summary(&self, store: &DocumentStore) -> DiagnosticSummary {
        let all = self.collect_all(store);
        let mut summary = DiagnosticSummary::default();
        for diags in all.values() {
            for diag in diags {
                match diag.severity {
                    DiagnosticSeverity::Error => summary.errors += 1,
                    DiagnosticSeverity::Warning => summary.warnings += 1,
                    DiagnosticSeverity::Information => summary.info += 1,
                    DiagnosticSeverity::Hint => summary.hints += 1,
                }
            }
            summary.files_with_issues += 1;
        }
        summary.total_files = store.uris().len();
        summary
    }
}
/// Summary of diagnostics across a project.
#[derive(Clone, Debug, Default)]
pub struct DiagnosticSummary {
    /// Total number of errors.
    pub errors: usize,
    /// Total number of warnings.
    pub warnings: usize,
    /// Total number of informational messages.
    pub info: usize,
    /// Total number of hints.
    pub hints: usize,
    /// Number of files with issues.
    pub files_with_issues: usize,
    /// Total number of files checked.
    pub total_files: usize,
}
impl DiagnosticSummary {
    /// Check if there are any errors.
    pub fn has_errors(&self) -> bool {
        self.errors > 0
    }
    /// Total number of diagnostics.
    pub fn total(&self) -> usize {
        self.errors + self.warnings + self.info + self.hints
    }
    /// Format as a human-readable string.
    pub fn format(&self) -> String {
        format!(
            "{} error(s), {} warning(s), {} info, {} hint(s) across {}/{} file(s)",
            self.errors,
            self.warnings,
            self.info,
            self.hints,
            self.files_with_issues,
            self.total_files,
        )
    }
}
/// Kind of fix.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FixKind {
    /// A quick fix for a diagnostic.
    QuickFix,
    /// A refactoring action.
    Refactor,
    /// Organize imports.
    OrganizeImports,
    /// Extract to definition.
    Extract,
    /// Inline a definition.
    Inline,
}
impl FixKind {
    /// Return the LSP string representation.
    pub fn as_str(&self) -> &str {
        match self {
            Self::QuickFix => "quickfix",
            Self::Refactor => "refactor",
            Self::OrganizeImports => "source.organizeImports",
            Self::Extract => "refactor.extract",
            Self::Inline => "refactor.inline",
        }
    }
}
/// A text edit within a fix.
#[derive(Clone, Debug)]
pub struct FixEdit {
    /// URI of the file to edit.
    pub uri: String,
    /// Range to replace.
    pub range: Range,
    /// New text.
    pub new_text: String,
}
/// Filter diagnostics by URI prefix.
#[allow(dead_code)]
pub struct UriPrefixFilter {
    pub prefix: String,
}
/// An advanced diagnostic entry with code, related info, and fix suggestions.
#[derive(Clone, Debug)]
pub struct AdvDiagnostic {
    /// The diagnostic code.
    pub code: AdvDiagnosticCode,
    /// Severity.
    pub severity: DiagnosticSeverity,
    /// The range in the source.
    pub range: Range,
    /// Human-readable message.
    pub message: String,
    /// Related information.
    pub related: Vec<AdvRelatedInfo>,
    /// Suggested fixes.
    pub fixes: Vec<AdvQuickFix>,
    /// Tags (e.g., deprecated, unnecessary).
    pub tags: Vec<DiagnosticTag>,
    /// Source file URI.
    pub uri: String,
}
impl AdvDiagnostic {
    /// Convert to LSP JSON diagnostic format.
    pub fn to_json(&self) -> JsonValue {
        let mut entries = vec![
            ("range".to_string(), self.range.to_json()),
            (
                "severity".to_string(),
                JsonValue::Number(self.severity.to_number()),
            ),
            (
                "code".to_string(),
                JsonValue::String(self.code.as_str().to_string()),
            ),
            (
                "source".to_string(),
                JsonValue::String("oxilean".to_string()),
            ),
            (
                "message".to_string(),
                JsonValue::String(self.message.clone()),
            ),
        ];
        if !self.tags.is_empty() {
            let tag_arr: Vec<JsonValue> = self
                .tags
                .iter()
                .map(|t| JsonValue::Number(*t as i32 as f64))
                .collect();
            entries.push(("tags".to_string(), JsonValue::Array(tag_arr)));
        }
        if !self.related.is_empty() {
            let related_arr: Vec<JsonValue> = self
                .related
                .iter()
                .map(|r| {
                    JsonValue::Object(vec![
                        (
                            "location".to_string(),
                            Location::new(&r.uri, r.range.clone()).to_json(),
                        ),
                        ("message".to_string(), JsonValue::String(r.message.clone())),
                    ])
                })
                .collect();
            entries.push((
                "relatedInformation".to_string(),
                JsonValue::Array(related_arr),
            ));
        }
        JsonValue::Object(entries)
    }
    /// Convert the quick fixes to LSP code action JSON.
    pub fn fixes_to_json(&self) -> Vec<JsonValue> {
        self.fixes
            .iter()
            .map(|fix| {
                let mut entries = vec![
                    ("title".to_string(), JsonValue::String(fix.title.clone())),
                    (
                        "kind".to_string(),
                        JsonValue::String(fix.kind.as_str().to_string()),
                    ),
                    ("isPreferred".to_string(), JsonValue::Bool(fix.is_preferred)),
                ];
                if !fix.edits.is_empty() {
                    let changes: Vec<JsonValue> = fix
                        .edits
                        .iter()
                        .map(|e| TextEdit::new(e.range.clone(), &e.new_text).to_json())
                        .collect();
                    entries.push((
                        "edit".to_string(),
                        JsonValue::Object(vec![(
                            "changes".to_string(),
                            JsonValue::Object(vec![(self.uri.clone(), JsonValue::Array(changes))]),
                        )]),
                    ));
                }
                entries.push((
                    "diagnostics".to_string(),
                    JsonValue::Array(vec![self.to_json()]),
                ));
                JsonValue::Object(entries)
            })
            .collect()
    }
}
/// A parsed query expression for filtering diagnostics.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum DiagnosticQuery {
    /// Match all diagnostics
    All,
    /// Match only errors
    Errors,
    /// Match only warnings
    Warnings,
    /// Match diagnostics containing a keyword in the message
    HasKeyword(String),
    /// Match diagnostics in a specific file
    InFile(String),
    /// Combine with AND
    And(Box<DiagnosticQuery>, Box<DiagnosticQuery>),
    /// Combine with OR
    Or(Box<DiagnosticQuery>, Box<DiagnosticQuery>),
    /// Negate
    Not(Box<DiagnosticQuery>),
}
impl DiagnosticQuery {
    /// Evaluate this query against a diagnostic.
    #[allow(dead_code)]
    pub fn matches(&self, diag: &AdvDiagnostic) -> bool {
        match self {
            DiagnosticQuery::All => true,
            DiagnosticQuery::Errors => matches!(diag.severity, DiagnosticSeverity::Error),
            DiagnosticQuery::Warnings => {
                matches!(diag.severity, DiagnosticSeverity::Warning)
            }
            DiagnosticQuery::HasKeyword(kw) => diag.message.contains(kw.as_str()),
            DiagnosticQuery::InFile(uri) => diag.uri == *uri,
            DiagnosticQuery::And(a, b) => a.matches(diag) && b.matches(diag),
            DiagnosticQuery::Or(a, b) => a.matches(diag) || b.matches(diag),
            DiagnosticQuery::Not(inner) => !inner.matches(diag),
        }
    }
    /// Filter a list of diagnostics using this query.
    #[allow(dead_code)]
    pub fn filter<'d>(&self, diagnostics: &'d [AdvDiagnostic]) -> Vec<&'d AdvDiagnostic> {
        diagnostics.iter().filter(|d| self.matches(d)).collect()
    }
}
/// Tracks the trend of diagnostics for a file.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct DiagnosticTrend {
    pub uri: String,
    pub history: Vec<usize>,
    pub window: usize,
}
impl DiagnosticTrend {
    /// Create a new trend tracker for a file.
    #[allow(dead_code)]
    pub fn new(uri: impl Into<String>, window: usize) -> Self {
        Self {
            uri: uri.into(),
            history: Vec::new(),
            window,
        }
    }
    /// Record a new diagnostic count.
    #[allow(dead_code)]
    pub fn record(&mut self, count: usize) {
        self.history.push(count);
        if self.history.len() > self.window {
            self.history.remove(0);
        }
    }
    /// Return the current trend direction.
    #[allow(dead_code)]
    pub fn direction(&self) -> TrendDirection {
        if self.history.len() < 2 {
            return TrendDirection::Stable;
        }
        let first = self.history[0] as f64;
        let last = *self
            .history
            .last()
            .expect("history has at least 2 elements: checked by early return")
            as f64;
        if last > first + 0.5 {
            TrendDirection::Increasing
        } else if last < first - 0.5 {
            TrendDirection::Decreasing
        } else {
            TrendDirection::Stable
        }
    }
    /// Return the moving average of the trend window.
    #[allow(dead_code)]
    pub fn moving_average(&self) -> f64 {
        if self.history.is_empty() {
            return 0.0;
        }
        let sum: usize = self.history.iter().sum();
        sum as f64 / self.history.len() as f64
    }
}
/// Cached diagnostics for a single file.
#[derive(Clone, Debug)]
pub struct CachedDiagnostics {
    /// Document version when diagnostics were computed.
    pub version: i64,
    /// The diagnostics.
    pub diagnostics: Vec<AdvDiagnostic>,
}
/// Cache for advanced diagnostics, keyed by URI and version.
#[derive(Debug, Default)]
pub struct AdvDiagnosticCache {
    /// Cached diagnostics per file.
    entries: HashMap<String, CachedDiagnostics>,
}
impl AdvDiagnosticCache {
    /// Create a new cache.
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }
    /// Get cached diagnostics if version matches.
    pub fn get(&self, uri: &str, version: i64) -> Option<&[AdvDiagnostic]> {
        self.entries
            .get(uri)
            .filter(|c| c.version == version)
            .map(|c| c.diagnostics.as_slice())
    }
    /// Store diagnostics.
    pub fn store(&mut self, uri: &str, version: i64, diagnostics: Vec<AdvDiagnostic>) {
        self.entries.insert(
            uri.to_string(),
            CachedDiagnostics {
                version,
                diagnostics,
            },
        );
    }
    /// Invalidate cached diagnostics for a file.
    pub fn invalidate(&mut self, uri: &str) {
        self.entries.remove(uri);
    }
    /// Clear all cached diagnostics.
    pub fn clear(&mut self) {
        self.entries.clear();
    }
    /// Get the number of cached files.
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    /// Check if the cache is empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}
/// State of a diagnostic lifecycle.
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DiagnosticState {
    /// Just appeared
    New,
    /// Persisted across multiple checks
    Persistent,
    /// Resolved (disappeared)
    Resolved,
    /// Suppressed by user
    Suppressed,
}
/// Rate-limits how often diagnostics are published.
#[allow(dead_code)]
pub struct DiagnosticRateLimiter {
    min_interval_ms: u64,
    last_publish: std::collections::HashMap<String, std::time::Instant>,
}
impl DiagnosticRateLimiter {
    /// Create a rate limiter with the given minimum interval in milliseconds.
    #[allow(dead_code)]
    pub fn new(min_interval_ms: u64) -> Self {
        Self {
            min_interval_ms,
            last_publish: std::collections::HashMap::new(),
        }
    }
    /// Check whether publishing is allowed for the given URI.
    #[allow(dead_code)]
    pub fn should_publish(&mut self, uri: &str) -> bool {
        let now = std::time::Instant::now();
        if let Some(last) = self.last_publish.get(uri) {
            if (last.elapsed().as_millis() as u64) < self.min_interval_ms {
                return false;
            }
        }
        self.last_publish.insert(uri.to_string(), now);
        true
    }
    /// Force-reset the timer for a URI.
    #[allow(dead_code)]
    pub fn reset(&mut self, uri: &str) {
        self.last_publish.remove(uri);
    }
}
/// Export format for diagnostics.
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DiagnosticExportFormat {
    Json,
    Csv,
    Sarif,
    Plain,
}
/// Related information for an advanced diagnostic.
#[derive(Clone, Debug)]
pub struct AdvRelatedInfo {
    /// Message.
    pub message: String,
    /// Location (URI + range).
    pub uri: String,
    /// Range in the related document.
    pub range: Range,
}
/// Exports diagnostics to various formats.
#[allow(dead_code)]
pub struct DiagnosticExporter;
impl DiagnosticExporter {
    /// Export a list of diagnostics to the given format.
    #[allow(dead_code)]
    pub fn export(diagnostics: &[AdvDiagnostic], format: DiagnosticExportFormat) -> String {
        match format {
            DiagnosticExportFormat::Json => Self::to_json(diagnostics),
            DiagnosticExportFormat::Csv => Self::to_csv(diagnostics),
            DiagnosticExportFormat::Sarif => Self::to_sarif(diagnostics),
            DiagnosticExportFormat::Plain => Self::to_plain(diagnostics),
        }
    }
    fn to_json(diagnostics: &[AdvDiagnostic]) -> String {
        let items: Vec<String> = diagnostics
            .iter()
            .map(|d| {
                format!(
                    "{{\"uri\":\"{}\",\"line\":{},\"message\":\"{}\"}}",
                    d.uri,
                    d.range.start.line,
                    d.message.replace('"', "\\\"")
                )
            })
            .collect();
        format!("[{}]", items.join(","))
    }
    fn to_csv(diagnostics: &[AdvDiagnostic]) -> String {
        let mut out = "uri,line,character,severity,message\n".to_string();
        for d in diagnostics {
            let severity = match d.severity {
                DiagnosticSeverity::Error => "error",
                DiagnosticSeverity::Warning => "warning",
                DiagnosticSeverity::Information => "info",
                DiagnosticSeverity::Hint => "hint",
            };
            out.push_str(&format!(
                "{},{},{},{},{}\n",
                d.uri,
                d.range.start.line,
                d.range.start.character,
                severity,
                d.message.replace(',', ";")
            ));
        }
        out
    }
    fn to_sarif(diagnostics: &[AdvDiagnostic]) -> String {
        let results: Vec<String> = diagnostics
            .iter()
            .map(|d| {
                format!(
                    "{{\"message\":{{\"text\":\"{msg}\"}},\"locations\":[{{\"physicalLocation\":{{\"artifactLocation\":{{\"uri\":\"{uri}\"}},\"region\":{{\"startLine\":{line}}}}}}}]}}",
                    msg = d.message.replace('"', "\\\""), uri = d.uri, line = d.range
                    .start.line + 1,
                )
            })
            .collect();
        format!(
            "{{\"version\":\"2.1.0\",\"runs\":[{{\"tool\":{{\"driver\":{{\"name\":\"oxilean\"}}}},\"results\":[{}]}}]}}",
            results.join(",")
        )
    }
    fn to_plain(diagnostics: &[AdvDiagnostic]) -> String {
        let mut out = String::new();
        for d in diagnostics {
            let severity = match d.severity {
                DiagnosticSeverity::Error => "error",
                DiagnosticSeverity::Warning => "warning",
                DiagnosticSeverity::Information => "info",
                DiagnosticSeverity::Hint => "hint",
            };
            out.push_str(&format!(
                "[{}] {}:{}:{}: {}\n",
                severity.to_uppercase(),
                d.uri,
                d.range.start.line,
                d.range.start.character,
                d.message
            ));
        }
        out
    }
}
/// A heatmap of diagnostics across lines of a file.
#[allow(dead_code)]
pub struct DiagnosticHeatmap {
    pub uri: String,
    pub cells: Vec<HeatmapCell>,
}
impl DiagnosticHeatmap {
    /// Build a heatmap from a list of diagnostics.
    #[allow(dead_code)]
    pub fn build(uri: impl Into<String>, diagnostics: &[AdvDiagnostic], line_count: usize) -> Self {
        let mut cells = vec![HeatmapCell::default(); line_count.max(1)];
        for d in diagnostics {
            let line = d.range.start.line as usize;
            if line >= cells.len() {
                cells.resize(line + 1, HeatmapCell::default());
            }
            match d.severity {
                DiagnosticSeverity::Error => cells[line].error_count += 1,
                DiagnosticSeverity::Warning => cells[line].warning_count += 1,
                _ => cells[line].info_count += 1,
            }
        }
        Self {
            uri: uri.into(),
            cells,
        }
    }
    /// Return the hottest line (most diagnostics).
    #[allow(dead_code)]
    pub fn hottest_line(&self) -> Option<usize> {
        self.cells
            .iter()
            .enumerate()
            .max_by_key(|(_, c)| c.total())
            .map(|(i, _)| i)
    }
    /// Render the heatmap as a simple bar chart string.
    #[allow(dead_code)]
    pub fn render_bars(&self, max_lines: usize) -> String {
        let mut out = String::new();
        for (i, cell) in self.cells.iter().enumerate().take(max_lines) {
            let bar: String = "#".repeat(cell.total() as usize);
            out.push_str(&format!("{:4}: {}\n", i, bar));
        }
        out
    }
}
/// Extended diagnostic codes for advanced analysis.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum AdvDiagnosticCode {
    /// Lexer error: invalid token.
    LexError,
    /// Parser error: syntax error.
    ParseError,
    /// Type error: type mismatch.
    TypeError,
    /// Unused variable.
    UnusedVariable,
    /// Unused import.
    UnusedImport,
    /// Shadowing.
    Shadowing,
    /// Deprecation warning.
    Deprecation,
    /// Unresolved name.
    UnresolvedName,
    /// Missing import.
    MissingImport,
    /// Redundant import.
    RedundantImport,
    /// Style warning (naming convention).
    StyleNaming,
    /// Style warning (line length).
    StyleLineLength,
    /// Style warning (trailing whitespace).
    StyleTrailingWhitespace,
    /// Incomplete pattern match.
    IncompleteMatch,
    /// Non-exhaustive pattern.
    NonExhaustivePattern,
    /// Infinite recursion detected.
    InfiniteRecursion,
    /// Axiom usage warning.
    AxiomUsage,
    /// Sorry placeholder in production code.
    SorryUsed,
    /// Duplicate definition.
    DuplicateDefinition,
    /// Unreachable code.
    UnreachableCode,
    /// Missing type annotation.
    MissingTypeAnnotation,
    /// Implicit argument could be explicit.
    ImplicitCouldBeExplicit,
}
impl AdvDiagnosticCode {
    /// Return the string code.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::LexError => "E001",
            Self::ParseError => "E002",
            Self::TypeError => "E003",
            Self::UnusedVariable => "W001",
            Self::UnusedImport => "W002",
            Self::Shadowing => "W003",
            Self::Deprecation => "W004",
            Self::UnresolvedName => "E004",
            Self::MissingImport => "E005",
            Self::RedundantImport => "W005",
            Self::StyleNaming => "S001",
            Self::StyleLineLength => "S002",
            Self::StyleTrailingWhitespace => "S003",
            Self::IncompleteMatch => "E006",
            Self::NonExhaustivePattern => "E007",
            Self::InfiniteRecursion => "E008",
            Self::AxiomUsage => "W006",
            Self::SorryUsed => "W007",
            Self::DuplicateDefinition => "E009",
            Self::UnreachableCode => "W008",
            Self::MissingTypeAnnotation => "H001",
            Self::ImplicitCouldBeExplicit => "H002",
        }
    }
    /// Return the default severity.
    pub fn default_severity(self) -> DiagnosticSeverity {
        match self {
            Self::LexError
            | Self::ParseError
            | Self::TypeError
            | Self::UnresolvedName
            | Self::MissingImport
            | Self::IncompleteMatch
            | Self::NonExhaustivePattern
            | Self::InfiniteRecursion
            | Self::DuplicateDefinition => DiagnosticSeverity::Error,
            Self::UnusedVariable
            | Self::UnusedImport
            | Self::Shadowing
            | Self::Deprecation
            | Self::RedundantImport
            | Self::AxiomUsage
            | Self::SorryUsed
            | Self::UnreachableCode => DiagnosticSeverity::Warning,
            Self::StyleNaming | Self::StyleLineLength | Self::StyleTrailingWhitespace => {
                DiagnosticSeverity::Information
            }
            Self::MissingTypeAnnotation | Self::ImplicitCouldBeExplicit => DiagnosticSeverity::Hint,
        }
    }
    /// Return a human-readable description.
    pub fn description(self) -> &'static str {
        match self {
            Self::LexError => "lexer error",
            Self::ParseError => "parse error",
            Self::TypeError => "type error",
            Self::UnusedVariable => "unused variable",
            Self::UnusedImport => "unused import",
            Self::Shadowing => "name shadows existing declaration",
            Self::Deprecation => "deprecated feature",
            Self::UnresolvedName => "unresolved name",
            Self::MissingImport => "missing import",
            Self::RedundantImport => "redundant import",
            Self::StyleNaming => "naming convention violation",
            Self::StyleLineLength => "line too long",
            Self::StyleTrailingWhitespace => "trailing whitespace",
            Self::IncompleteMatch => "incomplete pattern match",
            Self::NonExhaustivePattern => "non-exhaustive pattern",
            Self::InfiniteRecursion => "infinite recursion detected",
            Self::AxiomUsage => "axiom usage",
            Self::SorryUsed => "sorry used in code",
            Self::DuplicateDefinition => "duplicate definition",
            Self::UnreachableCode => "unreachable code",
            Self::MissingTypeAnnotation => "missing type annotation",
            Self::ImplicitCouldBeExplicit => "implicit argument could be explicit",
        }
    }
}
/// Annotates source text with diagnostic messages.
#[allow(dead_code)]
pub struct DiagnosticAnnotator {
    annotations: Vec<DiagnosticAnnotation>,
}
impl DiagnosticAnnotator {
    /// Create a new annotator.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            annotations: vec![],
        }
    }
    /// Add an annotation from an AdvDiagnostic.
    #[allow(dead_code)]
    pub fn add_from_diagnostic(&mut self, diag: &AdvDiagnostic) {
        self.annotations.push(DiagnosticAnnotation {
            line: diag.range.start.line,
            col_start: diag.range.start.character,
            col_end: diag.range.end.character,
            message: diag.message.clone(),
            severity: diag.severity.clone(),
        });
    }
    /// Render annotated source text with inline annotations.
    #[allow(dead_code)]
    pub fn render(&self, source: &str) -> String {
        let lines: Vec<&str> = source.lines().collect();
        let mut output = String::new();
        for (idx, line_text) in lines.iter().enumerate() {
            output.push_str(line_text);
            output.push('\n');
            let line_num = idx as u32;
            for ann in self.annotations.iter().filter(|a| a.line == line_num) {
                let indent: String = " ".repeat(ann.col_start as usize);
                let marker: String =
                    "^".repeat((ann.col_end.saturating_sub(ann.col_start) as usize).max(1));
                let prefix = match ann.severity {
                    DiagnosticSeverity::Error => "error",
                    DiagnosticSeverity::Warning => "warning",
                    DiagnosticSeverity::Information => "info",
                    DiagnosticSeverity::Hint => "hint",
                };
                output.push_str(&format!(
                    "{}{} [{}]: {}\n",
                    indent, marker, prefix, ann.message
                ));
            }
        }
        output
    }
    /// Return the number of annotations.
    #[allow(dead_code)]
    pub fn count(&self) -> usize {
        self.annotations.len()
    }
}
/// A heatmap cell recording diagnostic density at a line.
#[allow(dead_code)]
#[derive(Clone, Debug, Default)]
pub struct HeatmapCell {
    pub error_count: u32,
    pub warning_count: u32,
    pub info_count: u32,
}
impl HeatmapCell {
    /// Return the total count.
    #[allow(dead_code)]
    pub fn total(&self) -> u32 {
        self.error_count + self.warning_count + self.info_count
    }
}
/// A composite filter that applies all inner filters (AND semantics).
#[allow(dead_code)]
pub struct CompositeFilter {
    pub filters: Vec<Box<dyn DiagnosticFilter>>,
}
impl CompositeFilter {
    /// Create a new composite filter.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self { filters: vec![] }
    }
    /// Add a filter.
    #[allow(dead_code)]
    pub fn add(&mut self, filter: Box<dyn DiagnosticFilter>) {
        self.filters.push(filter);
    }
    /// Apply all filters to a list of diagnostics.
    #[allow(dead_code)]
    pub fn apply<'d>(&self, diagnostics: &'d [AdvDiagnostic]) -> Vec<&'d AdvDiagnostic> {
        diagnostics
            .iter()
            .filter(|d| self.filters.iter().all(|f| f.accepts(d)))
            .collect()
    }
}
/// An event describing changes to diagnostics.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct DiagnosticChangeEvent {
    pub uri: String,
    pub added: usize,
    pub removed: usize,
    pub total: usize,
}
/// Configuration for diagnostic severity overrides.
#[derive(Clone, Debug)]
pub struct SeverityConfig {
    /// Override severity for specific diagnostic codes.
    pub overrides: HashMap<String, DiagnosticSeverity>,
    /// Codes that are suppressed entirely.
    pub suppressed: HashSet<String>,
    /// Whether warnings should be treated as errors.
    pub warnings_as_errors: bool,
    /// Whether hints should be suppressed.
    pub suppress_hints: bool,
    /// Maximum number of errors to report per file.
    pub max_errors_per_file: usize,
    /// Maximum number of warnings to report per file.
    pub max_warnings_per_file: usize,
}
impl SeverityConfig {
    /// Create a new severity config.
    pub fn new() -> Self {
        Self::default()
    }
    /// Override the severity of a diagnostic code.
    pub fn set_override(&mut self, code: &str, severity: DiagnosticSeverity) {
        self.overrides.insert(code.to_string(), severity);
    }
    /// Suppress a diagnostic code entirely.
    pub fn suppress(&mut self, code: &str) {
        self.suppressed.insert(code.to_string());
    }
    /// Unsuppress a diagnostic code.
    pub fn unsuppress(&mut self, code: &str) {
        self.suppressed.remove(code);
    }
    /// Check if a diagnostic code is suppressed.
    pub fn is_suppressed(&self, code: &str) -> bool {
        self.suppressed.contains(code)
    }
    /// Get the effective severity for a diagnostic code.
    pub fn effective_severity(
        &self,
        code: &str,
        original: DiagnosticSeverity,
    ) -> Option<DiagnosticSeverity> {
        if self.is_suppressed(code) {
            return None;
        }
        if let Some(&overridden) = self.overrides.get(code) {
            return Some(overridden);
        }
        if self.warnings_as_errors && original == DiagnosticSeverity::Warning {
            return Some(DiagnosticSeverity::Error);
        }
        if self.suppress_hints && original == DiagnosticSeverity::Hint {
            return None;
        }
        Some(original)
    }
    /// Parse from JSON configuration.
    pub fn from_json(val: &JsonValue) -> Self {
        let mut config = Self::default();
        if let Some(wae) = val.get("warningsAsErrors").and_then(|v| v.as_bool()) {
            config.warnings_as_errors = wae;
        }
        if let Some(sh) = val.get("suppressHints").and_then(|v| v.as_bool()) {
            config.suppress_hints = sh;
        }
        if let Some(me) = val.get("maxErrorsPerFile").and_then(|v| v.as_i64()) {
            config.max_errors_per_file = me as usize;
        }
        if let Some(mw) = val.get("maxWarningsPerFile").and_then(|v| v.as_i64()) {
            config.max_warnings_per_file = mw as usize;
        }
        if let Some(suppressed) = val.get("suppressed").and_then(|v| v.as_array()) {
            for item in suppressed {
                if let Some(code) = item.as_str() {
                    config.suppress(code);
                }
            }
        }
        config
    }
}
/// Direction of a diagnostic trend.
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TrendDirection {
    /// Diagnostic count is increasing
    Increasing,
    /// Diagnostic count is decreasing
    Decreasing,
    /// Diagnostic count is stable
    Stable,
}
/// Filter diagnostics by diagnostic code.
#[allow(dead_code)]
pub struct CodeFilter {
    pub allowed_codes: Vec<AdvDiagnosticCode>,
}
/// A group of diagnostics for a single file.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct DiagnosticGroup {
    pub uri: String,
    pub diagnostics: Vec<AdvDiagnostic>,
}
impl DiagnosticGroup {
    /// Count errors in this group.
    #[allow(dead_code)]
    pub fn error_count(&self) -> usize {
        self.diagnostics
            .iter()
            .filter(|d| matches!(d.severity, DiagnosticSeverity::Error))
            .count()
    }
    /// Count warnings in this group.
    #[allow(dead_code)]
    pub fn warning_count(&self) -> usize {
        self.diagnostics
            .iter()
            .filter(|d| matches!(d.severity, DiagnosticSeverity::Warning))
            .count()
    }
}
/// Tracks the lifecycle state of diagnostics.
#[allow(dead_code)]
pub struct DiagnosticStateMachine {
    states: std::collections::HashMap<String, DiagnosticState>,
}
impl DiagnosticStateMachine {
    /// Create a new state machine.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            states: std::collections::HashMap::new(),
        }
    }
    /// Key for a diagnostic.
    fn key(diag: &AdvDiagnostic) -> String {
        format!(
            "{}:{}:{}:{}",
            diag.uri, diag.range.start.line, diag.range.start.character, diag.message
        )
    }
    /// Update states given the new set of active diagnostics.
    #[allow(dead_code)]
    pub fn update(&mut self, active: &[AdvDiagnostic]) {
        let active_keys: std::collections::HashSet<String> = active.iter().map(Self::key).collect();
        for (k, state) in self.states.iter_mut() {
            if !active_keys.contains(k) && *state != DiagnosticState::Resolved {
                *state = DiagnosticState::Resolved;
            }
        }
        for d in active {
            let k = Self::key(d);
            let entry = self.states.entry(k).or_insert(DiagnosticState::New);
            if *entry == DiagnosticState::New {
                *entry = DiagnosticState::Persistent;
            }
        }
    }
    /// Get the state of a specific diagnostic.
    #[allow(dead_code)]
    pub fn state_of(&self, diag: &AdvDiagnostic) -> DiagnosticState {
        self.states
            .get(&Self::key(diag))
            .cloned()
            .unwrap_or(DiagnosticState::New)
    }
}
/// Diagnostic tags (LSP spec).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DiagnosticTag {
    /// Reported as unnecessary (e.g., unused variable).
    Unnecessary = 1,
    /// Reported as deprecated.
    Deprecated = 2,
}
/// A suggested quick fix.
#[derive(Clone, Debug)]
pub struct AdvQuickFix {
    /// Title for the fix.
    pub title: String,
    /// The kind of fix.
    pub kind: FixKind,
    /// Text edits to apply.
    pub edits: Vec<FixEdit>,
    /// Whether this is the preferred fix.
    pub is_preferred: bool,
}
/// A single annotation applied to a line of source code.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct DiagnosticAnnotation {
    pub line: u32,
    pub col_start: u32,
    pub col_end: u32,
    pub message: String,
    pub severity: DiagnosticSeverity,
}
/// A suppression rule that disables diagnostics in a file region.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct DiagnosticSuppression {
    pub uri: String,
    pub start_line: u32,
    pub end_line: u32,
    pub codes: Vec<AdvDiagnosticCode>,
    pub reason: String,
}
impl DiagnosticSuppression {
    /// Create a new suppression rule.
    #[allow(dead_code)]
    pub fn new(
        uri: impl Into<String>,
        start_line: u32,
        end_line: u32,
        codes: Vec<AdvDiagnosticCode>,
        reason: impl Into<String>,
    ) -> Self {
        Self {
            uri: uri.into(),
            start_line,
            end_line,
            codes,
            reason: reason.into(),
        }
    }
    /// Check whether this rule suppresses the given diagnostic.
    #[allow(dead_code)]
    pub fn suppresses(&self, diag: &AdvDiagnostic) -> bool {
        if diag.uri != self.uri {
            return false;
        }
        let line = diag.range.start.line;
        if line < self.start_line || line > self.end_line {
            return false;
        }
        self.codes.is_empty() || self.codes.contains(&diag.code)
    }
}
/// A summary report of diagnostics for a workspace.
#[allow(dead_code)]
#[derive(Clone, Debug, Default)]
pub struct DiagnosticSummaryReport {
    pub total_errors: usize,
    pub total_warnings: usize,
    pub total_info: usize,
    pub total_hints: usize,
    pub files_with_errors: usize,
    pub files_checked: usize,
    pub top_error_codes: Vec<(String, usize)>,
}
impl DiagnosticSummaryReport {
    /// Create a new empty report.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    /// Return total diagnostics across all severities.
    #[allow(dead_code)]
    pub fn total(&self) -> usize {
        self.total_errors + self.total_warnings + self.total_info + self.total_hints
    }
    /// Return whether the workspace has any errors.
    #[allow(dead_code)]
    pub fn has_errors(&self) -> bool {
        self.total_errors > 0
    }
    /// Render the report as a human-readable string.
    #[allow(dead_code)]
    pub fn render(&self) -> String {
        format!(
            "Diagnostic Report: {} files checked\n  Errors:   {}\n  Warnings: {}\n  Info:     {}\n  Hints:    {}\n  Files with errors: {}\n",
            self.files_checked, self.total_errors, self.total_warnings, self.total_info,
            self.total_hints, self.files_with_errors,
        )
    }
}
/// Registry of suppression rules.
#[allow(dead_code)]
pub struct SuppressionRegistry {
    rules: Vec<DiagnosticSuppression>,
}
impl SuppressionRegistry {
    /// Create a new empty registry.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self { rules: vec![] }
    }
    /// Add a suppression rule.
    #[allow(dead_code)]
    pub fn add(&mut self, rule: DiagnosticSuppression) {
        self.rules.push(rule);
    }
    /// Return true if the diagnostic is suppressed.
    #[allow(dead_code)]
    pub fn is_suppressed(&self, diag: &AdvDiagnostic) -> bool {
        self.rules.iter().any(|r| r.suppresses(diag))
    }
    /// Filter out suppressed diagnostics.
    #[allow(dead_code)]
    pub fn filter<'d>(&self, diagnostics: &'d [AdvDiagnostic]) -> Vec<&'d AdvDiagnostic> {
        diagnostics
            .iter()
            .filter(|d| !self.is_suppressed(d))
            .collect()
    }
}
