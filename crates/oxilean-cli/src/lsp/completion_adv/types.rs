//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lsp::{
    analyze_document, CompletionItem, CompletionItemKind, Document, MarkupContent, Position,
};
use oxilean_kernel::Environment;
use std::collections::HashSet;

use super::functions::*;

use std::collections::HashMap;

/// Simplified previous token kind for context detection.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PrevTokenKind {
    /// A dot token.
    Dot,
    /// A colon token.
    Colon,
    /// A hash token (#check, #eval).
    Hash,
    /// An open parenthesis.
    OpenParen,
    /// A comma.
    Comma,
    /// A pipe (|) for match arms.
    Pipe,
    /// The `by` keyword.
    By,
    /// The `import` keyword.
    Import,
    /// The `open` keyword.
    Open,
    /// The `@[` attribute start.
    AtBracket,
    /// Some other token.
    Other,
}
/// Statistics for completion operations.
#[allow(dead_code)]
#[derive(Clone, Debug, Default)]
pub struct AdvCompletionStats {
    pub total_requests: u64,
    pub total_items_returned: u64,
    pub total_resolve_requests: u64,
    pub avg_items_per_request: f64,
}
impl AdvCompletionStats {
    /// Record a completion request.
    #[allow(dead_code)]
    pub fn record_request(&mut self, item_count: usize) {
        self.total_requests += 1;
        self.total_items_returned += item_count as u64;
        self.avg_items_per_request = self.total_items_returned as f64 / self.total_requests as f64;
    }
    /// Record a resolve request.
    #[allow(dead_code)]
    pub fn record_resolve(&mut self) {
        self.total_resolve_requests += 1;
    }
}
/// A snippet entry.
#[derive(Clone, Debug)]
pub struct SnippetEntry {
    /// Display label.
    pub label: &'static str,
    /// The snippet template (with $1, $2, etc.).
    pub template: &'static str,
    /// Short description.
    pub description: &'static str,
    /// Context where this snippet is applicable.
    pub context: SnippetContext,
}
/// Provides keyword completions.
#[allow(dead_code)]
pub struct KeywordCompletionProvider {
    pub keywords: Vec<(&'static str, &'static str)>,
}
impl KeywordCompletionProvider {
    /// Create a new keyword provider with OxiLean keywords.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            keywords: vec![
                ("theorem", "Declare a theorem"),
                ("def", "Declare a definition"),
                ("lemma", "Declare a lemma"),
                ("axiom", "Declare an axiom"),
                ("example", "Declare an example"),
                ("have", "Introduce a local hypothesis"),
                ("show", "Change goal type"),
                ("forall", "Universal quantifier"),
                ("exists", "Existential quantifier"),
                ("fun", "Lambda abstraction"),
                ("let", "Local binding"),
                ("match", "Pattern matching"),
                ("if", "Conditional expression"),
                ("then", "Then branch"),
                ("else", "Else branch"),
                ("intro", "Introduce hypotheses"),
                ("intros", "Introduce multiple hypotheses"),
                ("apply", "Apply a lemma"),
                ("exact", "Close goal with expression"),
                ("simp", "Simplify goal"),
                ("rw", "Rewrite goal"),
                ("cases", "Case split"),
                ("induction", "Induction"),
                ("constructor", "Apply constructor"),
                ("left", "Left disjunct"),
                ("right", "Right disjunct"),
                ("split", "Split conjunction/iff"),
                ("assumption", "Close by assumption"),
                ("refl", "Close by reflexivity"),
                ("sorry", "Placeholder proof"),
                ("by_contra", "Proof by contradiction"),
                ("push_neg", "Push negation inward"),
                ("ring", "Ring arithmetic"),
                ("linarith", "Linear arithmetic"),
            ],
        }
    }
}
/// Tactic categories for grouping.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TacticCategory {
    /// Introduction/elimination tactics.
    IntroElim,
    /// Rewriting tactics.
    Rewrite,
    /// Simplification tactics.
    Simplification,
    /// Case analysis/induction.
    CaseAnalysis,
    /// Closing tactics (exact, rfl, etc.).
    Closing,
    /// Structural tactics (have, let, show).
    Structural,
    /// Automation.
    Automation,
    /// Custom/advanced.
    Advanced,
}
/// A snippet definition.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct SnippetDefinition {
    pub prefix: String,
    pub body: String,
    pub description: String,
}
impl SnippetDefinition {
    /// Create a new snippet.
    #[allow(dead_code)]
    pub fn new(
        prefix: impl Into<String>,
        body: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        Self {
            prefix: prefix.into(),
            body: body.into(),
            description: description.into(),
        }
    }
}
/// Provides snippet completions for common proof patterns.
#[allow(dead_code)]
pub struct SnippetCompletionProvider {
    pub snippets: Vec<SnippetDefinition>,
}
impl SnippetCompletionProvider {
    /// Create a new provider with standard OxiLean snippets.
    #[allow(dead_code)]
    pub fn new() -> Self {
        let snippets =
            vec![
            SnippetDefinition::new("thm", "theorem ${1:name} : ${2:type} := by\n  ${0}",
            "Theorem declaration",), SnippetDefinition::new("def",
            "def ${1:name} : ${2:type} := ${0}", "Definition"),
            SnippetDefinition::new("ind",
            "induction ${1:h}\n· case zero =>\n  ${2}\n· case succ ${3:n} ${4:ih} =>\n  ${0}",
            "Induction tactic",), SnippetDefinition::new("forall",
            "forall (${1:x} : ${2:T}), ${0}", "Universal quantifier",),
            SnippetDefinition::new("exists", "exists ${1:witness}",
            "Existential witness"),
        ];
        Self { snippets }
    }
}
/// A list of completion items (LSP CompletionList).
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CompletionList {
    pub is_incomplete: bool,
    pub items: Vec<AdvancedCompletionItem>,
}
impl CompletionList {
    /// Create a complete list.
    #[allow(dead_code)]
    pub fn complete(items: Vec<AdvancedCompletionItem>) -> Self {
        Self {
            is_incomplete: false,
            items,
        }
    }
    /// Create an incomplete list (more items may be available).
    #[allow(dead_code)]
    pub fn incomplete(items: Vec<AdvancedCompletionItem>) -> Self {
        Self {
            is_incomplete: true,
            items,
        }
    }
    /// Sort items by sort_text or label.
    #[allow(dead_code)]
    pub fn sort(&mut self) {
        self.items.sort_by(|a, b| {
            let a_key = a.sort_text.as_deref().unwrap_or(&a.label);
            let b_key = b.sort_text.as_deref().unwrap_or(&b.label);
            a_key.cmp(b_key)
        });
    }
    /// Filter items by a prefix.
    #[allow(dead_code)]
    pub fn filter_by_prefix(&self, prefix: &str) -> Self {
        Self {
            is_incomplete: self.is_incomplete,
            items: self
                .items
                .iter()
                .filter(|item| {
                    let filter = item.filter_text.as_deref().unwrap_or(&item.label);
                    filter.starts_with(prefix)
                })
                .cloned()
                .collect(),
        }
    }
}
/// A step that limits the number of completion items.
#[allow(dead_code)]
pub struct LimitStep {
    pub max_items: usize,
}
/// Registry of completion providers.
#[allow(dead_code)]
pub struct CompletionRegistry {
    providers: Vec<Box<dyn CompletionProvider>>,
}
impl CompletionRegistry {
    /// Create a new empty registry.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self { providers: vec![] }
    }
    /// Add a provider.
    #[allow(dead_code)]
    pub fn add(&mut self, provider: Box<dyn CompletionProvider>) {
        self.providers.push(provider);
    }
    /// Create a default registry with keyword and snippet providers.
    #[allow(dead_code)]
    pub fn with_defaults() -> Self {
        let mut registry = Self::new();
        registry.add(Box::new(KeywordCompletionProvider::new()));
        registry.add(Box::new(SnippetCompletionProvider::new()));
        registry
    }
    /// Get all completions from all providers.
    #[allow(dead_code)]
    pub fn completions(
        &self,
        uri: &str,
        line: u32,
        character: u32,
        context: &LspCompletionContext,
    ) -> CompletionList {
        let mut all_items = vec![];
        let mut is_incomplete = false;
        for provider in &self.providers {
            let list = provider.completions(uri, line, character, context);
            if list.is_incomplete {
                is_incomplete = true;
            }
            all_items.extend(list.items);
        }
        if is_incomplete {
            CompletionList::incomplete(all_items)
        } else {
            CompletionList::complete(all_items)
        }
    }
}
/// Information about a known module.
#[derive(Clone, Debug)]
pub struct ModuleInfo {
    /// Full module path (e.g., "Init.Prelude").
    pub path: String,
    /// Short description.
    pub description: String,
    /// Whether this is from the standard library.
    pub is_std: bool,
}
/// Provides import and module name completions.
pub struct ImportCompleter {
    /// Known module names.
    modules: Vec<ModuleInfo>,
}
impl ImportCompleter {
    /// Create a new import completer with standard modules.
    pub fn new() -> Self {
        Self {
            modules: standard_modules(),
        }
    }
    /// Add a custom module.
    pub fn add_module(&mut self, path: &str, description: &str, is_std: bool) {
        self.modules.push(ModuleInfo {
            path: path.to_string(),
            description: description.to_string(),
            is_std,
        });
    }
    /// Get import completions matching a prefix.
    pub fn complete(&self, prefix: &str) -> Vec<CompletionItem> {
        let mut items = Vec::new();
        for module in &self.modules {
            if prefix.is_empty() || module.path.starts_with(prefix) {
                items.push(CompletionItem {
                    label: module.path.clone(),
                    kind: CompletionItemKind::Module,
                    detail: Some(module.description.clone()),
                    documentation: Some(MarkupContent::markdown(format!(
                        "**Module** `{}`\n\n{}",
                        module.path, module.description
                    ))),
                    insert_text: None,
                    sort_text: Some(format!(
                        "{}_{}",
                        if module.is_std { "0" } else { "1" },
                        module.path
                    )),
                });
            }
        }
        items
    }
    /// Get sub-module completions after a dot.
    pub fn complete_submodule(&self, parent: &str) -> Vec<CompletionItem> {
        let prefix_with_dot = format!("{}.", parent);
        let mut items = Vec::new();
        let mut seen = HashSet::new();
        for module in &self.modules {
            if let Some(rest) = module.path.strip_prefix(&prefix_with_dot) {
                let next_part = rest.split('.').next().unwrap_or("");
                if !next_part.is_empty() && seen.insert(next_part.to_string()) {
                    let full_path = format!("{}.{}", parent, next_part);
                    let has_children = self
                        .modules
                        .iter()
                        .any(|m| m.path.starts_with(&format!("{}.", full_path)));
                    items.push(CompletionItem {
                        label: next_part.to_string(),
                        kind: CompletionItemKind::Module,
                        detail: Some(if has_children {
                            "module (has sub-modules)".to_string()
                        } else {
                            "module".to_string()
                        }),
                        documentation: None,
                        insert_text: None,
                        sort_text: None,
                    });
                }
            }
        }
        items
    }
}
/// Analyzes the document context at a given position.
pub struct ContextAnalyzer<'a> {
    /// The document being analyzed.
    doc: &'a Document,
    /// The kernel environment.
    env: &'a Environment,
}
impl<'a> ContextAnalyzer<'a> {
    /// Create a new context analyzer.
    pub fn new(doc: &'a Document, env: &'a Environment) -> Self {
        Self { doc, env }
    }
    /// Analyze the context at the given position.
    pub fn analyze(&self, pos: &Position) -> CompletionContextInfo {
        let line_before = self.get_line_before(pos);
        let prefix = self.extract_prefix(&line_before);
        let indent_level = self.get_indent_level(pos);
        let prev_token = self.get_prev_token(&line_before);
        let category = self.determine_category(pos, &line_before, &prev_token);
        let in_do_block = self.is_in_do_block(pos);
        let in_where_block = self.is_in_where_block(pos);
        let namespace = self.find_enclosing_namespace(pos);
        let enclosing_def = self.find_enclosing_def(pos);
        CompletionContextInfo {
            category,
            prefix,
            line_before,
            indent_level,
            in_do_block,
            in_where_block,
            namespace,
            enclosing_def,
            prev_token: Some(prev_token),
        }
    }
    /// Get the text before the cursor on the current line.
    fn get_line_before(&self, pos: &Position) -> String {
        if let Some(line) = self.doc.get_line(pos.line) {
            let col = (pos.character as usize).min(line.len());
            line[..col].to_string()
        } else {
            String::new()
        }
    }
    /// Extract the prefix (partial identifier) from text before cursor.
    fn extract_prefix(&self, line_before: &str) -> String {
        let bytes = line_before.as_bytes();
        let end = bytes.len();
        let mut start = end;
        while start > 0 && is_ident_char(bytes[start - 1]) {
            start -= 1;
        }
        if start < end {
            line_before[start..end].to_string()
        } else {
            String::new()
        }
    }
    /// Get the indentation level at a position.
    fn get_indent_level(&self, pos: &Position) -> usize {
        if let Some(line) = self.doc.get_line(pos.line) {
            let trimmed = line.trim_start();
            line.len() - trimmed.len()
        } else {
            0
        }
    }
    /// Determine the previous token kind from the text before cursor.
    fn get_prev_token(&self, line_before: &str) -> PrevTokenKind {
        let trimmed = line_before.trim_end();
        if trimmed.is_empty() {
            return PrevTokenKind::Other;
        }
        if trimmed.ends_with('.') {
            PrevTokenKind::Dot
        } else if trimmed.ends_with(':') && !trimmed.ends_with(":=") {
            PrevTokenKind::Colon
        } else if trimmed.ends_with('(') {
            PrevTokenKind::OpenParen
        } else if trimmed.ends_with(',') {
            PrevTokenKind::Comma
        } else if trimmed.ends_with('|') {
            PrevTokenKind::Pipe
        } else if trimmed.ends_with("#check")
            || trimmed.ends_with("#eval")
            || trimmed.ends_with("#print")
        {
            PrevTokenKind::Hash
        } else if trimmed.ends_with("by") {
            PrevTokenKind::By
        } else if trimmed.ends_with("import") {
            PrevTokenKind::Import
        } else if trimmed.ends_with("open") {
            PrevTokenKind::Open
        } else if trimmed.ends_with("@[") {
            PrevTokenKind::AtBracket
        } else {
            PrevTokenKind::Other
        }
    }
    /// Determine the broad context category.
    fn determine_category(
        &self,
        pos: &Position,
        line_before: &str,
        prev_token: &PrevTokenKind,
    ) -> ContextCategory {
        match prev_token {
            PrevTokenKind::Dot => return ContextCategory::DotAccess,
            PrevTokenKind::Import => return ContextCategory::Import,
            PrevTokenKind::Open => return ContextCategory::Open,
            PrevTokenKind::By => return ContextCategory::Tactic,
            PrevTokenKind::Colon => return ContextCategory::TypeAnnotation,
            PrevTokenKind::AtBracket => return ContextCategory::Attribute,
            PrevTokenKind::Pipe => return ContextCategory::Pattern,
            _ => {}
        }
        let trimmed = line_before.trim_start();
        if trimmed.starts_with("import ") {
            return ContextCategory::Import;
        }
        if trimmed.starts_with("open ") {
            return ContextCategory::Open;
        }
        if self.is_in_tactic_block(pos) {
            return ContextCategory::Tactic;
        }
        if self.is_in_do_block(pos) {
            return ContextCategory::DoNotation;
        }
        if self.is_in_where_block(pos) {
            return ContextCategory::WhereClause;
        }
        if line_before.contains(':') && !line_before.contains(":=") {
            return ContextCategory::TypeAnnotation;
        }
        if trimmed.is_empty() || TOP_LEVEL_KEYWORDS.iter().any(|kw| trimmed.starts_with(kw)) {
            return ContextCategory::TopLevel;
        }
        ContextCategory::Expression
    }
    /// Check if position is inside a tactic block.
    fn is_in_tactic_block(&self, pos: &Position) -> bool {
        let mut line_idx = pos.line as usize;
        let mut our_indent = None;
        loop {
            if let Some(line) = self.doc.get_line(line_idx as u32) {
                let trimmed = line.trim_start();
                let indent = line.len() - trimmed.len();
                if our_indent.is_none() {
                    our_indent = Some(indent);
                }
                if trimmed == "by" || trimmed.starts_with("by ") || trimmed.starts_with("by\n") {
                    if let Some(oi) = our_indent {
                        return oi > indent;
                    }
                }
                if indent == 0 && TOP_LEVEL_KEYWORDS.iter().any(|kw| trimmed.starts_with(kw)) {
                    return false;
                }
            }
            if line_idx == 0 {
                break;
            }
            line_idx -= 1;
        }
        false
    }
    /// Check if inside a `do` block.
    fn is_in_do_block(&self, pos: &Position) -> bool {
        let mut line_idx = pos.line as usize;
        while line_idx > 0 {
            if let Some(line) = self.doc.get_line(line_idx as u32) {
                let trimmed = line.trim_start();
                if trimmed == "do" || trimmed.starts_with("do ") || trimmed.ends_with(" do") {
                    return true;
                }
                if TOP_LEVEL_KEYWORDS.iter().any(|kw| trimmed.starts_with(kw)) {
                    return false;
                }
            }
            line_idx -= 1;
        }
        false
    }
    /// Check if inside a `where` block.
    fn is_in_where_block(&self, pos: &Position) -> bool {
        let mut line_idx = pos.line as usize;
        while line_idx > 0 {
            if let Some(line) = self.doc.get_line(line_idx as u32) {
                let trimmed = line.trim_start();
                if trimmed == "where" || trimmed.ends_with(" where") {
                    return true;
                }
                let indent = line.len() - trimmed.len();
                if indent == 0 && TOP_LEVEL_KEYWORDS.iter().any(|kw| trimmed.starts_with(kw)) {
                    return false;
                }
            }
            line_idx -= 1;
        }
        false
    }
    /// Find the enclosing namespace.
    fn find_enclosing_namespace(&self, pos: &Position) -> Option<String> {
        let mut line_idx = pos.line as usize;
        while line_idx > 0 {
            if let Some(line) = self.doc.get_line(line_idx as u32) {
                let trimmed = line.trim_start();
                if let Some(rest) = trimmed.strip_prefix("namespace ") {
                    return Some(rest.trim().to_string());
                }
            }
            line_idx -= 1;
        }
        None
    }
    /// Find the enclosing definition name.
    fn find_enclosing_def(&self, pos: &Position) -> Option<String> {
        let analysis = analyze_document(&self.doc.uri, &self.doc.content, self.env);
        for def in analysis.definitions.iter().rev() {
            if def.range.start.line <= pos.line {
                return Some(def.name.clone());
            }
        }
        None
    }
}
/// An advanced completion item with extra metadata.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct AdvancedCompletionItem {
    pub label: String,
    pub kind: u32,
    pub detail: Option<String>,
    pub documentation: Option<String>,
    pub deprecated: bool,
    pub preselect: bool,
    pub sort_text: Option<String>,
    pub filter_text: Option<String>,
    pub insert_text: Option<String>,
    pub insert_text_format: u32,
    pub tags: Vec<CompletionItemTag>,
    pub text_edit_kind: TextEditKind,
    pub additional_text_edits: Vec<(String, String)>,
    pub commit_characters: Vec<char>,
    pub data: Option<String>,
}
impl AdvancedCompletionItem {
    /// Create a minimal completion item.
    #[allow(dead_code)]
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            kind: 1,
            detail: None,
            documentation: None,
            deprecated: false,
            preselect: false,
            sort_text: None,
            filter_text: None,
            insert_text: None,
            insert_text_format: 1,
            tags: vec![],
            text_edit_kind: TextEditKind::ReplaceWord,
            additional_text_edits: vec![],
            commit_characters: vec![],
            data: None,
        }
    }
    /// Set the detail string.
    #[allow(dead_code)]
    pub fn with_detail(mut self, detail: impl Into<String>) -> Self {
        self.detail = Some(detail.into());
        self
    }
    /// Set the documentation string.
    #[allow(dead_code)]
    pub fn with_documentation(mut self, doc: impl Into<String>) -> Self {
        self.documentation = Some(doc.into());
        self
    }
    /// Set the kind.
    #[allow(dead_code)]
    pub fn with_kind(mut self, kind: u32) -> Self {
        self.kind = kind;
        self
    }
    /// Mark as preselected.
    #[allow(dead_code)]
    pub fn preselected(mut self) -> Self {
        self.preselect = true;
        self
    }
    /// Mark as deprecated.
    #[allow(dead_code)]
    pub fn deprecated(mut self) -> Self {
        self.deprecated = true;
        self.tags.push(CompletionItemTag::Deprecated);
        self
    }
    /// Set a snippet insert text.
    #[allow(dead_code)]
    pub fn with_snippet(mut self, snippet: impl Into<String>) -> Self {
        self.insert_text = Some(snippet.into());
        self.insert_text_format = 2;
        self
    }
}
/// LSP-style completion context.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct LspCompletionContext {
    pub trigger_kind: CompletionTrigger,
    pub trigger_character: Option<char>,
}
impl LspCompletionContext {
    /// Create an invoked context.
    #[allow(dead_code)]
    pub fn invoked() -> Self {
        Self {
            trigger_kind: CompletionTrigger::Invoked,
            trigger_character: None,
        }
    }
    /// Create a character-triggered context.
    #[allow(dead_code)]
    pub fn triggered_by(c: char) -> Self {
        Self {
            trigger_kind: CompletionTrigger::Character(c),
            trigger_character: Some(c),
        }
    }
}
/// Result of a completion request.
#[derive(Clone, Debug)]
pub struct CompletionResult {
    /// The completion items.
    pub items: Vec<CompletionItem>,
    /// Whether the list is incomplete (more items available).
    pub is_incomplete: bool,
    /// The context category that was detected.
    pub context: ContextCategory,
}
/// Cache entry for completion results.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CompletionCacheEntry {
    pub uri: String,
    pub line: u32,
    pub character: u32,
    pub list: CompletionList,
    pub timestamp: std::time::Instant,
    pub ttl_ms: u64,
}
impl CompletionCacheEntry {
    /// Check if this entry is still valid.
    #[allow(dead_code)]
    pub fn is_valid(&self) -> bool {
        (self.timestamp.elapsed().as_millis() as u64) < self.ttl_ms
    }
}
/// A pipeline of completion processing steps.
#[allow(dead_code)]
pub struct CompletionPipeline {
    steps: Vec<Box<dyn CompletionPipelineStep>>,
}
impl CompletionPipeline {
    /// Create a new empty pipeline.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self { steps: vec![] }
    }
    /// Add a step.
    #[allow(dead_code)]
    pub fn add_step(mut self, step: Box<dyn CompletionPipelineStep>) -> Self {
        self.steps.push(step);
        self
    }
    /// Run the pipeline on a completion list.
    #[allow(dead_code)]
    pub fn run(&self, list: CompletionList) -> CompletionList {
        self.steps.iter().fold(list, |l, step| step.process(l))
    }
}
/// Contexts where snippets are applicable.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SnippetContext {
    /// Top-level command context.
    TopLevel,
    /// Inside a proof.
    Tactic,
    /// Inside an expression.
    Expression,
    /// Inside a structure declaration.
    Structure,
    /// Anywhere.
    Any,
}
/// Cache for completion results.
#[allow(dead_code)]
pub struct CompletionCache {
    entries: std::collections::HashMap<String, CompletionCacheEntry>,
    max_size: usize,
}
impl CompletionCache {
    /// Create a new cache.
    #[allow(dead_code)]
    pub fn new(max_size: usize) -> Self {
        Self {
            entries: std::collections::HashMap::new(),
            max_size,
        }
    }
    fn cache_key(uri: &str, line: u32, character: u32) -> String {
        format!("{}:{}:{}", uri, line, character)
    }
    /// Store a completion list.
    #[allow(dead_code)]
    pub fn store(&mut self, uri: &str, line: u32, character: u32, list: CompletionList) {
        if self.entries.len() >= self.max_size {
            let oldest = self
                .entries
                .iter()
                .min_by_key(|(_, e)| e.timestamp)
                .map(|(k, _)| k.clone());
            if let Some(k) = oldest {
                self.entries.remove(&k);
            }
        }
        let key = Self::cache_key(uri, line, character);
        self.entries.insert(
            key,
            CompletionCacheEntry {
                uri: uri.to_string(),
                line,
                character,
                list,
                timestamp: std::time::Instant::now(),
                ttl_ms: 2000,
            },
        );
    }
    /// Look up a cached completion list.
    #[allow(dead_code)]
    pub fn get(&self, uri: &str, line: u32, character: u32) -> Option<&CompletionList> {
        let key = Self::cache_key(uri, line, character);
        self.entries
            .get(&key)
            .filter(|e| e.is_valid())
            .map(|e| &e.list)
    }
    /// Clear all expired entries.
    #[allow(dead_code)]
    pub fn evict_expired(&mut self) {
        self.entries.retain(|_, e| e.is_valid());
    }
    /// Clear all entries for a URI.
    #[allow(dead_code)]
    pub fn invalidate_uri(&mut self, uri: &str) {
        self.entries.retain(|_, e| e.uri != uri);
    }
}
/// Detailed context information for a completion request.
#[derive(Clone, Debug)]
pub struct CompletionContextInfo {
    /// The broad context category.
    pub category: ContextCategory,
    /// The prefix text being completed.
    pub prefix: String,
    /// The full line text before the cursor.
    pub line_before: String,
    /// The current indentation level.
    pub indent_level: usize,
    /// Whether we are inside a `do` block.
    pub in_do_block: bool,
    /// Whether we are inside a `where` block.
    pub in_where_block: bool,
    /// The enclosing namespace (if any).
    pub namespace: Option<String>,
    /// The enclosing definition name (if any).
    pub enclosing_def: Option<String>,
    /// Previous token kind (for context-sensitive completions).
    pub prev_token: Option<PrevTokenKind>,
}
/// Formats completion items for different display contexts.
#[allow(dead_code)]
pub struct CompletionFormatter;
impl CompletionFormatter {
    /// Format an item for display in a terminal.
    #[allow(dead_code)]
    pub fn format_for_terminal(item: &AdvancedCompletionItem) -> String {
        let deprecated_mark = if item.deprecated { " [DEPRECATED]" } else { "" };
        match &item.detail {
            Some(detail) => format!("{}{} -- {}", item.label, deprecated_mark, detail),
            None => format!("{}{}", item.label, deprecated_mark),
        }
    }
    /// Format an item for a JSON response.
    #[allow(dead_code)]
    pub fn format_for_json(item: &AdvancedCompletionItem) -> String {
        let detail = item
            .detail
            .as_deref()
            .map(|d| format!(",\"detail\":\"{}\"", d.replace('"', "\\\"")))
            .unwrap_or_default();
        let doc = item
            .documentation
            .as_deref()
            .map(|d| format!(",\"documentation\":\"{}\"", d.replace('"', "\\\"")))
            .unwrap_or_default();
        format!(
            "{{\"label\":\"{}\",\"kind\":{}{}{},\"deprecated\":{}}}",
            item.label.replace('"', "\\\""),
            item.kind,
            detail,
            doc,
            item.deprecated,
        )
    }
    /// Format a completion list as a JSON array.
    #[allow(dead_code)]
    pub fn list_to_json(list: &CompletionList) -> String {
        let items: Vec<String> = list.items.iter().map(Self::format_for_json).collect();
        format!(
            "{{\"isIncomplete\":{},\"items\":[{}]}}",
            list.is_incomplete,
            items.join(",")
        )
    }
}
/// Provides import completion for OxiLean files.
#[allow(dead_code)]
pub struct ImportCompletionProvider {
    pub known_modules: Vec<String>,
}
impl ImportCompletionProvider {
    /// Create a new import provider.
    #[allow(dead_code)]
    pub fn new(modules: Vec<String>) -> Self {
        Self {
            known_modules: modules,
        }
    }
    /// Add a module to the known list.
    #[allow(dead_code)]
    pub fn add_module(&mut self, module: String) {
        self.known_modules.push(module);
    }
}
/// A tactic entry with completion metadata.
#[derive(Clone, Debug)]
pub struct TacticEntry {
    /// Tactic name.
    pub name: &'static str,
    /// Short description.
    pub description: &'static str,
    /// Snippet insert text.
    pub snippet: Option<&'static str>,
    /// Category for sorting.
    pub category: TacticCategory,
    /// Whether this is commonly used.
    pub common: bool,
}
/// The unified completion engine that combines all completion sources.
pub struct UnifiedCompletionEngine<'a> {
    /// The kernel environment.
    env: &'a Environment,
    /// Tactic completer.
    tactic_completer: TacticCompleter,
    /// Import completer.
    import_completer: ImportCompleter,
    /// Snippet provider.
    snippet_provider: SnippetProvider,
    /// Type filter.
    type_filter: TypeFilter<'a>,
    /// Maximum completions to return.
    max_completions: usize,
}
impl<'a> UnifiedCompletionEngine<'a> {
    /// Create a new unified completion engine.
    pub fn new(env: &'a Environment) -> Self {
        Self {
            env,
            tactic_completer: TacticCompleter::new(),
            import_completer: ImportCompleter::new(),
            snippet_provider: SnippetProvider::new(),
            type_filter: TypeFilter::new(env),
            max_completions: 100,
        }
    }
    /// Set the maximum number of completions.
    pub fn set_max_completions(&mut self, max: usize) {
        self.max_completions = max;
    }
    /// Compute completions at a position.
    pub fn complete_at(&self, doc: &Document, pos: &Position) -> CompletionResult {
        let analyzer = ContextAnalyzer::new(doc, self.env);
        let context = analyzer.analyze(pos);
        let mut items = Vec::new();
        match context.category {
            ContextCategory::Tactic => {
                items.extend(self.tactic_completer.complete(&context.prefix));
                items.extend(
                    self.snippet_provider
                        .complete(&context.prefix, &context.category),
                );
            }
            ContextCategory::Import => {
                items.extend(self.import_completer.complete(&context.prefix));
            }
            ContextCategory::Open => {
                items.extend(self.import_completer.complete(&context.prefix));
            }
            ContextCategory::DotAccess => {
                items.extend(self.dot_completions(doc, pos));
            }
            ContextCategory::TypeAnnotation => {
                items.extend(self.type_completions(&context.prefix));
                items.extend(keyword_completions(&context.prefix));
            }
            ContextCategory::Attribute => {
                items.extend(self.attribute_completions(&context.prefix));
            }
            ContextCategory::TopLevel => {
                items.extend(keyword_completions(&context.prefix));
                items.extend(
                    self.snippet_provider
                        .complete(&context.prefix, &context.category),
                );
                items.extend(self.env_completions(&context.prefix));
            }
            _ => {
                items.extend(keyword_completions(&context.prefix));
                items.extend(self.env_completions(&context.prefix));
                items.extend(
                    self.snippet_provider
                        .complete(&context.prefix, &context.category),
                );
            }
        }
        let mut seen = HashSet::new();
        items.retain(|item| seen.insert(item.label.clone()));
        let is_incomplete = items.len() > self.max_completions;
        if items.len() > self.max_completions {
            items.truncate(self.max_completions);
        }
        CompletionResult {
            items,
            is_incomplete,
            context: context.category,
        }
    }
    /// Get completions from the environment.
    fn env_completions(&self, prefix: &str) -> Vec<CompletionItem> {
        let mut items = Vec::new();
        for name in self.env.constant_names() {
            let name_str = name.to_string();
            if prefix.is_empty() || name_str.starts_with(prefix) {
                let detail = if self.env.is_inductive(name) {
                    "inductive type"
                } else if self.env.is_constructor(name) {
                    "constructor"
                } else {
                    "declaration"
                };
                items.push(CompletionItem::function(&name_str, detail));
            }
        }
        items
    }
    /// Get type completions (for type annotation context).
    fn type_completions(&self, prefix: &str) -> Vec<CompletionItem> {
        let mut items = Vec::new();
        let builtin_types = [
            ("Nat", "Natural numbers"),
            ("Int", "Integers"),
            ("Float", "Floating-point numbers"),
            ("String", "Unicode strings"),
            ("Bool", "Boolean values"),
            ("Char", "Unicode characters"),
            ("Unit", "The unit type"),
            ("Empty", "The empty type"),
            ("Prop", "Propositions"),
            ("Type", "Types"),
            ("Sort", "Sorts (universe-polymorphic)"),
            ("List", "Linked lists"),
            ("Array", "Dynamic arrays"),
            ("Option", "Optional values"),
            ("IO", "IO monad"),
        ];
        for (ty, desc) in &builtin_types {
            if prefix.is_empty() || ty.starts_with(prefix) {
                items.push(CompletionItem {
                    label: ty.to_string(),
                    kind: CompletionItemKind::Class,
                    detail: Some(desc.to_string()),
                    documentation: None,
                    insert_text: None,
                    sort_text: Some(format!("0_{}", ty)),
                });
            }
        }
        for name in self.env.constant_names() {
            if self.env.is_inductive(name) {
                let name_str = name.to_string();
                if prefix.is_empty() || name_str.starts_with(prefix) {
                    items.push(CompletionItem {
                        label: name_str.clone(),
                        kind: CompletionItemKind::Class,
                        detail: Some("inductive type".to_string()),
                        documentation: None,
                        insert_text: None,
                        sort_text: Some(format!("1_{}", name_str)),
                    });
                }
            }
        }
        items
    }
    /// Get dot-access completions.
    fn dot_completions(&self, _doc: &Document, _pos: &Position) -> Vec<CompletionItem> {
        let mut items = Vec::new();
        let projections = [
            "val",
            "mk",
            "rec",
            "recOn",
            "casesOn",
            "noConfusion",
            "length",
            "head",
            "tail",
            "map",
            "filter",
            "foldl",
            "toList",
            "toArray",
            "toString",
        ];
        for proj in &projections {
            items.push(CompletionItem {
                label: proj.to_string(),
                kind: CompletionItemKind::Field,
                detail: Some("projection/method".to_string()),
                documentation: None,
                insert_text: None,
                sort_text: None,
            });
        }
        items
    }
    /// Get attribute completions.
    pub(crate) fn attribute_completions(&self, prefix: &str) -> Vec<CompletionItem> {
        let attributes = [
            ("simp", "Mark as simp lemma"),
            ("ext", "Mark as extensionality lemma"),
            ("instance", "Mark as instance"),
            ("inline", "Inline this definition"),
            ("reducible", "Mark as reducible"),
            ("irreducible", "Mark as irreducible"),
            ("class", "Mark as type class"),
            ("deprecated", "Mark as deprecated"),
            ("macro", "Define as macro"),
            ("scoped", "Scoped attribute"),
            ("local", "Local attribute"),
        ];
        let mut items = Vec::new();
        for (attr, desc) in &attributes {
            if prefix.is_empty() || attr.starts_with(prefix) {
                items.push(CompletionItem {
                    label: attr.to_string(),
                    kind: CompletionItemKind::Property,
                    detail: Some(desc.to_string()),
                    documentation: None,
                    insert_text: None,
                    sort_text: None,
                });
            }
        }
        items
    }
}
/// A step that removes deprecated items unless they're preselected.
#[allow(dead_code)]
pub struct RemoveDeprecatedStep;
/// Filters completions based on expected types.
pub struct TypeFilter<'a> {
    /// Reference to the environment.
    env: &'a Environment,
}
impl<'a> TypeFilter<'a> {
    /// Create a new type filter.
    pub fn new(env: &'a Environment) -> Self {
        Self { env }
    }
    /// Filter completions by expected return type.
    pub fn filter_by_type(
        &self,
        items: &[CompletionItem],
        expected_type: &str,
    ) -> Vec<CompletionItem> {
        let mut filtered = Vec::new();
        let mut untyped = Vec::new();
        for item in items {
            if let Some(ref detail) = item.detail {
                if self.type_matches(detail, expected_type) {
                    let mut item = item.clone();
                    item.sort_text = Some(format!("0_type_{}", item.label));
                    filtered.push(item);
                } else {
                    let mut item = item.clone();
                    item.sort_text = Some(format!("2_other_{}", item.label));
                    untyped.push(item);
                }
            } else {
                untyped.push(item.clone());
            }
        }
        filtered.extend(untyped);
        filtered
    }
    /// Check if a type string matches the expected type.
    pub(crate) fn type_matches(&self, actual: &str, expected: &str) -> bool {
        if actual == expected {
            return true;
        }
        if actual.ends_with(&format!("-> {}", expected)) {
            return true;
        }
        if actual.ends_with(&format!("-> {}", expected)) {
            return true;
        }
        if (expected == "Prop" && actual.contains("Prop"))
            || (expected == "Bool" && actual.contains("Bool"))
        {
            return true;
        }
        if expected == "Nat" && (actual.contains("Nat") || actual == "natural number") {
            return true;
        }
        false
    }
    /// Get completions from the environment filtered by type.
    pub fn completions_for_type(&self, expected_type: &str, prefix: &str) -> Vec<CompletionItem> {
        let mut items = Vec::new();
        for name in self.env.constant_names() {
            let name_str = name.to_string();
            if !prefix.is_empty() && !name_str.starts_with(prefix) {
                continue;
            }
            if let Some(ci) = self.env.find(name) {
                let ty_str = format!("{:?}", ci.ty());
                if self.type_matches(&ty_str, expected_type) {
                    items.push(CompletionItem {
                        label: name_str,
                        kind: CompletionItemKind::Function,
                        detail: Some(ty_str),
                        documentation: None,
                        insert_text: None,
                        sort_text: Some(format!("0_typed_{}", name)),
                    });
                }
            }
        }
        items
    }
}
/// Provides context-aware snippet completions.
pub struct SnippetProvider {
    /// Snippet entries.
    entries: Vec<SnippetEntry>,
}
impl SnippetProvider {
    /// Create a new snippet provider with standard snippets.
    pub fn new() -> Self {
        Self {
            entries: standard_snippets(),
        }
    }
    /// Get snippets matching a prefix and context.
    pub fn complete(&self, prefix: &str, context: &ContextCategory) -> Vec<CompletionItem> {
        let target_ctx = match context {
            ContextCategory::TopLevel => SnippetContext::TopLevel,
            ContextCategory::Tactic => SnippetContext::Tactic,
            ContextCategory::Expression => SnippetContext::Expression,
            ContextCategory::FieldDeclaration => SnippetContext::Structure,
            _ => SnippetContext::Any,
        };
        let mut items = Vec::new();
        for entry in &self.entries {
            if entry.context != target_ctx && entry.context != SnippetContext::Any {
                continue;
            }
            if prefix.is_empty() || entry.label.starts_with(prefix) {
                items.push(CompletionItem::snippet(
                    entry.label,
                    entry.template,
                    entry.description,
                ));
            }
        }
        items
    }
}
/// A request to resolve additional data for a completion item.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CompletionResolveRequest {
    pub item_label: String,
    pub item_data: Option<String>,
}
impl CompletionResolveRequest {
    /// Create a resolve request.
    #[allow(dead_code)]
    pub fn new(label: impl Into<String>, data: Option<String>) -> Self {
        Self {
            item_label: label.into(),
            item_data: data,
        }
    }
}
/// Tag for a completion item.
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CompletionItemTag {
    /// Item is deprecated
    Deprecated,
}
/// Broad context categories for completion.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ContextCategory {
    /// Top-level command context.
    TopLevel,
    /// Inside a tactic proof (after `by`).
    Tactic,
    /// Inside a term expression.
    Expression,
    /// Inside a type annotation (after `:`).
    TypeAnnotation,
    /// Inside a pattern (in `match` or function definition).
    Pattern,
    /// Inside an import statement.
    Import,
    /// Inside an `open` statement.
    Open,
    /// After a dot (field access or qualified name).
    DotAccess,
    /// Inside a `do` block.
    DoNotation,
    /// Inside a `where` clause.
    WhereClause,
    /// Inside an attribute declaration.
    Attribute,
    /// Inside a structure/class field declaration.
    FieldDeclaration,
    /// Unknown context.
    Unknown,
}
/// What triggered a completion request.
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CompletionTrigger {
    /// User typed a character
    Character(char),
    /// User explicitly invoked completion
    Invoked,
    /// Previous completion was incomplete
    TriggerForIncomplete,
}
/// Provides tactic completions for proof mode.
pub struct TacticCompleter {
    /// Tactic entries with metadata.
    entries: Vec<TacticEntry>,
}
impl TacticCompleter {
    /// Create a new tactic completer with all standard tactics.
    pub fn new() -> Self {
        Self {
            entries: standard_tactics(),
        }
    }
    /// Get completions matching a prefix.
    pub fn complete(&self, prefix: &str) -> Vec<CompletionItem> {
        let mut items = Vec::new();
        let lower_prefix = prefix.to_lowercase();
        for entry in &self.entries {
            if prefix.is_empty() || entry.name.to_lowercase().starts_with(&lower_prefix) {
                let item = CompletionItem {
                    label: entry.name.to_string(),
                    kind: CompletionItemKind::Method,
                    detail: Some(entry.description.to_string()),
                    documentation: Some(MarkupContent::markdown(format!(
                        "**Tactic** `{}`\n\n{}",
                        entry.name, entry.description
                    ))),
                    insert_text: entry.snippet.map(String::from),
                    sort_text: Some(format!(
                        "{}_{:02}_{}",
                        if entry.common { "0" } else { "1" },
                        entry.category as u32,
                        entry.name
                    )),
                };
                items.push(item);
            }
        }
        items
    }
    /// Get completions for a specific tactic category.
    pub fn complete_category(&self, prefix: &str, category: TacticCategory) -> Vec<CompletionItem> {
        self.complete(prefix)
            .into_iter()
            .filter(|item| {
                self.entries
                    .iter()
                    .any(|e| e.name == item.label && e.category == category)
            })
            .collect()
    }
}
/// Kind of text edit applied when accepting a completion.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum TextEditKind {
    /// Replace the current word
    ReplaceWord,
    /// Insert at cursor
    Insert,
    /// Replace a range
    ReplaceRange {
        start_line: u32,
        start_char: u32,
        end_line: u32,
        end_char: u32,
    },
}
