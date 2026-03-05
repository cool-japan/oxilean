//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lsp::{
    analyze_document, CompletionItem, CompletionItemKind, Document, MarkupContent, Position, Range,
};
use oxilean_kernel::Environment;
use std::collections::HashMap;

use super::functions::*;

/// The result of a completion request.
#[derive(Clone, Debug)]
pub struct CompletionList {
    /// The completion items.
    pub items: Vec<CompletionItem>,
    /// If true, further typing should re-trigger completion.
    pub is_incomplete: bool,
}
impl CompletionList {
    /// Create a new completion list.
    pub fn new(items: Vec<CompletionItem>, is_incomplete: bool) -> Self {
        Self {
            items,
            is_incomplete,
        }
    }
    /// Create an empty completion list.
    pub fn empty() -> Self {
        Self {
            items: Vec::new(),
            is_incomplete: false,
        }
    }
}
/// Information about the expected type at a completion position.
#[derive(Clone, Debug)]
pub struct ExpectedTypeInfo {
    /// The expected type string.
    pub ty: String,
    /// Confidence (0.0 = unknown, 1.0 = certain).
    pub confidence: f32,
    /// The source of this expectation.
    pub source: TypeExpectationSource,
}
/// A tactic entry with description.
pub struct TacticEntry {
    /// The tactic name.
    pub name: &'static str,
    /// Short description.
    pub doc: &'static str,
}
/// A text edit for multi-edit completions.
#[derive(Clone, Debug)]
pub struct SimpleTextEdit {
    /// The range to replace.
    pub range: Range,
    /// The new text.
    pub new_text: String,
}
/// Information to display in the completion item's detail pane.
#[derive(Clone, Debug)]
pub struct CompletionPreview {
    /// The kind label.
    pub kind_label: String,
    /// The full type signature.
    pub type_signature: Option<String>,
    /// Short documentation.
    pub documentation: Option<String>,
    /// Whether the item is deprecated.
    pub deprecated: bool,
}
impl CompletionPreview {
    /// Build a preview for a keyword.
    pub fn keyword(_name: &str, description: &str) -> Self {
        Self {
            kind_label: "keyword".to_string(),
            type_signature: None,
            documentation: Some(description.to_string()),
            deprecated: false,
        }
    }
    /// Build a preview for a tactic.
    pub fn tactic(name: &str, description: &str) -> Self {
        Self {
            kind_label: "tactic".to_string(),
            type_signature: None,
            documentation: Some(format!("**Tactic** `{}`\n\n{}", name, description)),
            deprecated: false,
        }
    }
    /// Build a preview from an environment constant.
    pub fn from_env(name: &str, ty: &str) -> Self {
        Self {
            kind_label: "declaration".to_string(),
            type_signature: Some(format!("{} : {}", name, ty)),
            documentation: None,
            deprecated: false,
        }
    }
    /// Format as markdown.
    pub fn to_markdown(&self) -> String {
        let mut md = format!("*{}*\n\n", self.kind_label);
        if let Some(sig) = &self.type_signature {
            md.push_str(&format!("```lean\n{}\n```\n\n", sig));
        }
        if let Some(doc) = &self.documentation {
            md.push_str(doc);
        }
        if self.deprecated {
            md.push_str("\n\n*Deprecated*");
        }
        md
    }
}
/// A small LRU-like history of recently accepted completions.
#[derive(Clone, Debug, Default)]
pub struct CompletionHistory {
    /// Ordered list of recently used labels (most recent first).
    entries: Vec<String>,
    /// Maximum history size.
    max_size: usize,
}
impl CompletionHistory {
    /// Create a new history with a given capacity.
    pub fn new(max_size: usize) -> Self {
        Self {
            entries: Vec::new(),
            max_size,
        }
    }
    /// Record that a completion was accepted.
    pub fn record(&mut self, label: &str) {
        self.entries.retain(|e| e != label);
        self.entries.insert(0, label.to_string());
        if self.entries.len() > self.max_size {
            self.entries.truncate(self.max_size);
        }
    }
    /// Get the recency rank of a label (0 = most recent).
    pub fn rank(&self, label: &str) -> Option<usize> {
        self.entries.iter().position(|e| e == label)
    }
    /// Boost completion items that appear in history.
    pub fn boost_items(&self, items: &mut Vec<CompletionItem>) {
        for item in items.iter_mut() {
            if let Some(rank) = self.rank(&item.label) {
                let boost = format!("{:03}", rank);
                item.sort_text = Some(format!("0{}{}", boost, item.label));
            }
        }
    }
    /// How many entries are in history.
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    /// Whether the history is empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
    /// Clear the history.
    pub fn clear(&mut self) {
        self.entries.clear();
    }
}
/// Priority tiers for completion items.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum CompletionPriority {
    /// Lowest priority.
    Low = 0,
    /// Medium priority.
    Medium = 50,
    /// High priority.
    High = 100,
    /// Exact match priority.
    Exact = 200,
}
impl CompletionPriority {
    /// Return a sort_text string that orders items (lexicographic ascending = lower value first).
    pub fn sort_key(self) -> String {
        let inverted = 999 - (self as i32);
        format!("{:04}", inverted)
    }
}
/// A smart completion provider that uses the expected type.
pub struct SmartCompletionProvider<'a> {
    /// Environment reference.
    env: &'a Environment,
}
impl<'a> SmartCompletionProvider<'a> {
    /// Create a new smart provider.
    pub fn new(env: &'a Environment) -> Self {
        Self { env }
    }
    /// Get completions tailored to an expected type.
    pub fn completions_for_type(&self, expected_type: &str, prefix: &str) -> Vec<CompletionItem> {
        let mut items = Vec::new();
        match expected_type {
            "Nat" | "nat" => items.extend(self.static_candidates(
                &[
                    ("0", "zero"),
                    ("1", "one"),
                    ("Nat.zero", "zero ctor"),
                    ("Nat.succ", "succ ctor"),
                    ("Nat.add", "addition"),
                    ("Nat.mul", "multiplication"),
                ],
                prefix,
            )),
            "Bool" | "bool" => items.extend(self.static_candidates(
                &[
                    ("true", "Boolean true"),
                    ("false", "Boolean false"),
                    ("Bool.and", "and"),
                    ("Bool.or", "or"),
                    ("Bool.not", "not"),
                ],
                prefix,
            )),
            "Prop" => items.extend(self.static_candidates(
                &[
                    ("True", "trivially true"),
                    ("False", "trivially false"),
                    ("And", "conjunction"),
                    ("Or", "disjunction"),
                    ("Not", "negation"),
                ],
                prefix,
            )),
            "String" => items.extend(self.static_candidates(
                &[
                    ("\"\"", "empty string"),
                    ("String.append", "concatenate"),
                    ("String.length", "length"),
                ],
                prefix,
            )),
            _ => {
                let pattern = format!("{}.", expected_type);
                for name in self.env.constant_names() {
                    let name_str = name.to_string();
                    if name_str.starts_with(&pattern) && self.env.is_constructor(name) {
                        let field = &name_str[pattern.len()..];
                        if prefix.is_empty() || field.starts_with(prefix) {
                            items.push(CompletionItem {
                                label: name_str,
                                kind: CompletionItemKind::Constructor,
                                detail: Some(format!("constructor of {}", expected_type)),
                                documentation: None,
                                insert_text: None,
                                sort_text: None,
                            });
                        }
                    }
                }
            }
        }
        items
    }
    /// Filter and convert static candidate list.
    fn static_candidates(&self, candidates: &[(&str, &str)], prefix: &str) -> Vec<CompletionItem> {
        candidates
            .iter()
            .filter(|&&(name, _)| prefix.is_empty() || name.starts_with(prefix))
            .map(|&(name, doc)| CompletionItem {
                label: name.to_string(),
                kind: CompletionItemKind::Variable,
                detail: Some(doc.to_string()),
                documentation: Some(MarkupContent::plain(doc)),
                insert_text: None,
                sort_text: None,
            })
            .collect()
    }
}
/// A keyword entry with its documentation.
pub struct KeywordEntry {
    /// The keyword text.
    pub keyword: &'static str,
    /// Short documentation.
    pub doc: &'static str,
    /// Whether it's a top-level command keyword.
    pub is_command: bool,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CompletionTriggerChar {
    pub character: char,
    pub description: String,
}
#[allow(dead_code)]
impl CompletionTriggerChar {
    pub fn new(character: char, description: &str) -> Self {
        Self {
            character,
            description: description.to_string(),
        }
    }
}
/// The advanced completion engine.
pub struct CompletionEngine<'a> {
    /// Reference to the kernel environment.
    env: &'a Environment,
    /// Cache for computed completions by prefix.
    cache: HashMap<String, Vec<CompletionItem>>,
}
impl<'a> CompletionEngine<'a> {
    /// Create a new completion engine with the given environment.
    pub fn new(env: &'a Environment) -> Self {
        Self {
            env,
            cache: HashMap::new(),
        }
    }
    /// Compute completions at the given position in a document.
    pub fn complete_at_position(&mut self, doc: &Document, pos: &Position) -> CompletionList {
        let prefix = self.extract_prefix(doc, pos);
        let context = self.determine_context(doc, pos);
        let mut items = Vec::new();
        match context {
            CompletionContext::InTactic => {
                items.extend(tactic_completions(&prefix));
                items.extend(keyword_completions(&prefix));
            }
            CompletionContext::InImport => {
                items.extend(self.module_completions(&prefix));
            }
            CompletionContext::TriggerCharacter('.') => {
                items.extend(self.field_completions(doc, pos));
            }
            CompletionContext::InType => {
                items.extend(self.type_aware_completions(&prefix, None));
                items.extend(keyword_completions(&prefix));
            }
            CompletionContext::InCommand => {
                items.extend(keyword_completions(&prefix));
                items.extend(snippet_completions(&prefix));
                items.extend(self.env_completions(&prefix));
            }
            _ => {
                items.extend(keyword_completions(&prefix));
                items.extend(self.env_completions(&prefix));
                items.extend(snippet_completions(&prefix));
            }
        }
        items.sort_by(|a, b| {
            let sa = self.score_completion(a, &prefix, &context);
            let sb = self.score_completion(b, &prefix, &context);
            sb.cmp(&sa)
        });
        let mut seen = std::collections::HashSet::new();
        items.retain(|item| seen.insert(item.label.clone()));
        let is_incomplete = items.len() > 100;
        if items.len() > 100 {
            items.truncate(100);
        }
        CompletionList::new(items, is_incomplete)
    }
    /// Extract the prefix (partial identifier) before the cursor.
    pub(crate) fn extract_prefix(&self, doc: &Document, pos: &Position) -> String {
        if let Some(line) = doc.get_line(pos.line) {
            let col = (pos.character as usize).min(line.len());
            let bytes = line.as_bytes();
            let mut start = col;
            while start > 0 && is_ident_char(bytes[start - 1]) {
                start -= 1;
            }
            line[start..col].to_string()
        } else {
            String::new()
        }
    }
    /// Determine the completion context at the given position.
    pub fn determine_context(&self, doc: &Document, pos: &Position) -> CompletionContext {
        if let Some(line) = doc.get_line(pos.line) {
            let col = (pos.character as usize).min(line.len());
            let before = &line[..col];
            if before.ends_with('.') {
                return CompletionContext::TriggerCharacter('.');
            }
            if before.ends_with(':') {
                return CompletionContext::TriggerCharacter(':');
            }
            let trimmed = before.trim_start();
            if trimmed.starts_with("import") || trimmed.starts_with("open") {
                return CompletionContext::InImport;
            }
            if self.is_in_tactic_block(doc, pos) {
                return CompletionContext::InTactic;
            }
            if before.contains(':') && !before.contains(":=") {
                return CompletionContext::InType;
            }
            if trimmed.is_empty() || COMMAND_KEYWORDS.iter().any(|kw| trimmed.starts_with(kw)) {
                return CompletionContext::InCommand;
            }
            CompletionContext::InExpr
        } else {
            CompletionContext::Invoked
        }
    }
    /// Check if the position is inside a tactic block (after `by`).
    fn is_in_tactic_block(&self, doc: &Document, pos: &Position) -> bool {
        let mut line_idx = pos.line as usize;
        let mut indent_at_pos = None;
        loop {
            if let Some(line) = doc.get_line(line_idx as u32) {
                let trimmed = line.trim_start();
                let indent = line.len() - trimmed.len();
                if indent_at_pos.is_none() {
                    indent_at_pos = Some(indent);
                }
                if trimmed == "by" || trimmed.starts_with("by ") || trimmed.starts_with("by\n") {
                    if let Some(our_indent) = indent_at_pos {
                        return our_indent > indent;
                    }
                }
                if indent == 0 && COMMAND_KEYWORDS.iter().any(|kw| trimmed.starts_with(kw)) {
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
    /// Score a completion item for relevance (higher is better).
    pub fn score_completion(
        &self,
        item: &CompletionItem,
        prefix: &str,
        context: &CompletionContext,
    ) -> i32 {
        let mut score = 0i32;
        if item.label.starts_with(prefix) {
            score += 100;
        }
        if item
            .label
            .to_lowercase()
            .starts_with(&prefix.to_lowercase())
        {
            score += 50;
        }
        match context {
            CompletionContext::InTactic => {
                if item.kind == CompletionItemKind::Method {
                    score += 80;
                }
            }
            CompletionContext::InCommand => {
                if item.kind == CompletionItemKind::Keyword {
                    score += 80;
                }
                if item.kind == CompletionItemKind::Snippet {
                    score += 60;
                }
            }
            CompletionContext::InType => {
                if item.kind == CompletionItemKind::Class
                    || item.kind == CompletionItemKind::Interface
                    || item.kind == CompletionItemKind::Constructor
                {
                    score += 80;
                }
            }
            _ => {}
        }
        score -= (item.label.len() as i32) / 4;
        score
    }
    /// Get completions from the environment.
    fn env_completions(&mut self, prefix: &str) -> Vec<CompletionItem> {
        if let Some(cached) = self.cache.get(prefix) {
            return cached.clone();
        }
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
        self.cache.insert(prefix.to_string(), items.clone());
        items
    }
    /// Get module-like completions (for import/open statements).
    pub(crate) fn module_completions(&self, prefix: &str) -> Vec<CompletionItem> {
        let mut items = Vec::new();
        let modules = [
            "Init",
            "Init.Prelude",
            "Init.Core",
            "Std",
            "Std.Data",
            "Std.Logic",
            "Mathlib",
        ];
        for m in &modules {
            if prefix.is_empty() || m.starts_with(prefix) {
                items.push(CompletionItem {
                    label: m.to_string(),
                    kind: CompletionItemKind::Module,
                    detail: Some("module".to_string()),
                    documentation: Some(MarkupContent::plain(format!("Import module {}", m))),
                    insert_text: None,
                    sort_text: None,
                });
            }
        }
        items
    }
    /// Type-aware completions: find constants whose type is compatible.
    pub fn type_aware_completions(
        &self,
        prefix: &str,
        _expected_type: Option<&str>,
    ) -> Vec<CompletionItem> {
        let mut items = Vec::new();
        items.extend(self.compatible_constants(prefix));
        items
    }
    /// Find constants in the environment matching a prefix.
    pub fn compatible_constants(&self, prefix: &str) -> Vec<CompletionItem> {
        let mut items = Vec::new();
        for name in self.env.constant_names() {
            let name_str = name.to_string();
            if prefix.is_empty() || name_str.starts_with(prefix) {
                if let Some(ci) = self.env.find(name) {
                    let ty_str = format!("{:?}", ci.ty());
                    items.push(CompletionItem {
                        label: name_str,
                        kind: CompletionItemKind::Function,
                        detail: Some(ty_str),
                        documentation: None,
                        insert_text: None,
                        sort_text: None,
                    });
                }
            }
        }
        items
    }
    /// Complete structure fields after `.`.
    pub fn field_completions(&self, doc: &Document, pos: &Position) -> Vec<CompletionItem> {
        let mut items = Vec::new();
        if let Some(line) = doc.get_line(pos.line) {
            let col = (pos.character as usize).min(line.len());
            if col > 0 && line.as_bytes().get(col.wrapping_sub(1)) == Some(&b'.') {
                let bytes = line.as_bytes();
                let end = col - 1;
                let mut start = end;
                while start > 0 && is_ident_char(bytes[start - 1]) {
                    start -= 1;
                }
                if start < end {
                    let ident_before_dot = &line[start..end];
                    let found = self.field_completions_for_type(ident_before_dot);
                    if !found.is_empty() {
                        items.extend(found);
                    } else {
                        let fallback = ["mk", "rec", "casesOn", "recOn", "noConfusion"];
                        for f in &fallback {
                            items.push(CompletionItem {
                                label: f.to_string(),
                                kind: CompletionItemKind::Method,
                                detail: Some("recursor/constructor".to_string()),
                                documentation: None,
                                insert_text: None,
                                sort_text: None,
                            });
                        }
                    }
                }
            }
        }
        items
    }
    /// Return field/projection completions for `type_name.`.
    ///
    /// Searches the environment for constants whose name starts with `type_name.`
    /// (projection functions, constructors, recursors, etc.) and returns them
    /// as field completion items.
    fn field_completions_for_type(&self, type_name: &str) -> Vec<CompletionItem> {
        let mut items = Vec::new();
        let prefix = format!("{}.", type_name);
        for name in self.env.constant_names() {
            let name_str = name.to_string();
            if name_str.starts_with(&prefix) {
                let field = &name_str[prefix.len()..];
                if field.starts_with('_') || field.contains('.') {
                    continue;
                }
                let (kind, detail) = if self.env.is_constructor(name) {
                    (CompletionItemKind::Constructor, "constructor")
                } else if self.env.is_recursor(name) {
                    (CompletionItemKind::Method, "recursor")
                } else {
                    (CompletionItemKind::Field, "projection")
                };
                items.push(CompletionItem {
                    label: field.to_string(),
                    kind,
                    detail: Some(detail.to_string()),
                    documentation: None,
                    insert_text: None,
                    sort_text: None,
                });
            }
        }
        items
    }
    /// Complete inductive type constructors.
    pub fn constructor_completions(&self, prefix: &str) -> Vec<CompletionItem> {
        let mut items = Vec::new();
        for name in self.env.constant_names() {
            if self.env.is_constructor(name) {
                let name_str = name.to_string();
                if prefix.is_empty() || name_str.starts_with(prefix) {
                    items.push(CompletionItem {
                        label: name_str,
                        kind: CompletionItemKind::Constructor,
                        detail: Some("constructor".to_string()),
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
/// Where the expected type came from.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TypeExpectationSource {
    /// From an explicit annotation `x : T`.
    Annotation,
    /// From function application context.
    Application,
    /// From a match arm.
    MatchArm,
    /// Unknown source.
    Unknown,
}
/// A completion that involves multiple edits.
#[derive(Clone, Debug)]
pub struct MultiEditCompletion {
    /// The primary label.
    pub label: String,
    /// The kind.
    pub kind: CompletionItemKind,
    /// All edits to perform.
    pub edits: Vec<SimpleTextEdit>,
    /// Detail string.
    pub detail: Option<String>,
}
impl MultiEditCompletion {
    /// Create from a label and a set of edits.
    pub fn new(label: &str, kind: CompletionItemKind, edits: Vec<SimpleTextEdit>) -> Self {
        Self {
            label: label.to_string(),
            kind,
            edits,
            detail: None,
        }
    }
    /// Build a multi-edit completion that adds an import.
    pub fn with_import(label: &str, module: &str, insert_range: Range) -> Self {
        Self {
            label: label.to_string(),
            kind: CompletionItemKind::Function,
            edits: vec![
                SimpleTextEdit {
                    range: Range::single_line(0, 0, 0),
                    new_text: format!("import {}\n", module),
                },
                SimpleTextEdit {
                    range: insert_range,
                    new_text: label.to_string(),
                },
            ],
            detail: Some(format!("from {}", module)),
        }
    }
    /// Convert to a standard CompletionItem.
    pub fn to_completion_item(&self) -> CompletionItem {
        CompletionItem {
            label: self.label.clone(),
            kind: self.kind,
            detail: self.detail.clone(),
            documentation: None,
            insert_text: Some(self.label.clone()),
            sort_text: None,
        }
    }
}
/// The context in which a completion was triggered.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CompletionContext {
    /// Triggered by typing a trigger character (e.g. `.`, `:`).
    TriggerCharacter(char),
    /// Explicitly invoked by the user (Ctrl+Space).
    Invoked,
    /// Inside an expression.
    InExpr,
    /// Inside a type annotation.
    InType,
    /// Inside a tactic block (after `by`).
    InTactic,
    /// At the top level where commands are expected.
    InCommand,
    /// Inside an import statement.
    InImport,
}
/// A postfix completion template.
#[derive(Clone, Debug)]
pub struct PostfixTemplate {
    /// The trigger suffix (e.g. ".map").
    pub trigger: String,
    /// The expansion template where `$EXPR` is the base expression.
    pub template: String,
    /// A description.
    pub description: String,
}
impl PostfixTemplate {
    /// Apply this template, replacing `$EXPR` with `expr`.
    pub fn expand(&self, expr: &str) -> String {
        self.template.replace("$EXPR", expr)
    }
}
/// A completion item that also inserts an import at the top of the file.
#[derive(Clone, Debug)]
pub struct ImportedCompletion {
    /// The completion item label.
    pub label: String,
    /// The module to import.
    pub import_module: String,
    /// Whether the import is already present.
    pub already_imported: bool,
    /// The completion kind.
    pub kind: CompletionItemKind,
    /// Detail string.
    pub detail: Option<String>,
}
impl ImportedCompletion {
    /// Create a new imported completion.
    pub fn new(label: &str, import_module: &str, kind: CompletionItemKind) -> Self {
        Self {
            label: label.to_string(),
            import_module: import_module.to_string(),
            already_imported: false,
            kind,
            detail: None,
        }
    }
    /// Convert to a standard completion item.
    pub fn to_completion_item(&self) -> CompletionItem {
        let detail = if self.already_imported {
            self.detail.clone()
        } else {
            Some(format!(
                "{} (adds import {})",
                self.detail.as_deref().unwrap_or(""),
                self.import_module
            ))
        };
        CompletionItem {
            label: self.label.clone(),
            kind: self.kind,
            detail,
            documentation: None,
            insert_text: None,
            sort_text: None,
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CompletionScore {
    pub label: String,
    pub score: f64,
}
#[allow(dead_code)]
impl CompletionScore {
    pub fn new(label: &str, score: f64) -> Self {
        Self {
            label: label.to_string(),
            score: score.clamp(0.0, 1.0),
        }
    }
}
