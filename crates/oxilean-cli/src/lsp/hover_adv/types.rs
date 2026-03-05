//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lsp::{
    analyze_document, Document, Hover, MarkupContent, MarkupKind, Position, Range, SymbolKind,
};
use oxilean_kernel::{Environment, Name};
use oxilean_parse::{Lexer, TokenKind};
use std::collections::HashMap;

use super::functions::*;

/// A hypothesis in a goal.
#[derive(Clone, Debug)]
pub struct HypothesisInfo {
    /// Hypothesis name.
    pub name: String,
    /// Hypothesis type.
    pub ty: String,
}
/// Database for documentation lookup.
pub struct DocumentationDb {
    /// Documentation entries keyed by declaration name.
    entries: HashMap<String, String>,
}
impl DocumentationDb {
    /// Create a new documentation database with standard entries.
    pub fn new() -> Self {
        let mut entries = HashMap::new();
        entries
            .insert(
                "Nat.add".to_string(),
                "Addition of natural numbers.\n\n`Nat.add a b` computes `a + b`.\n\nThis is the basic addition operation on natural numbers, \
             defined by recursion on the second argument."
                    .to_string(),
            );
        entries.insert(
            "Nat.mul".to_string(),
            "Multiplication of natural numbers.\n\n`Nat.mul a b` computes `a * b`.".to_string(),
        );
        entries.insert(
            "Nat.succ".to_string(),
            "Successor function.\n\n`Nat.succ n` is `n + 1`.".to_string(),
        );
        entries.insert(
            "Nat.zero".to_string(),
            "The natural number zero.".to_string(),
        );
        entries.insert(
            "Nat.rec".to_string(),
            "Recursor for natural numbers.\n\nProvides induction/recursion on `Nat`.".to_string(),
        );
        entries.insert("List.nil".to_string(), "The empty list.".to_string());
        entries.insert(
            "List.cons".to_string(),
            "Prepend an element to a list.\n\n`List.cons a as` = `a :: as`.".to_string(),
        );
        entries
            .insert(
                "List.map".to_string(),
                "Apply a function to every element of a list.\n\n`List.map f [a, b, c]` = `[f a, f b, f c]`."
                    .to_string(),
            );
        entries
            .insert(
                "And".to_string(),
                "Logical conjunction.\n\n`And P Q` (written `P /\\ Q`) is true when both `P` and `Q` are true."
                    .to_string(),
            );
        entries
            .insert(
                "Or".to_string(),
                "Logical disjunction.\n\n`Or P Q` (written `P \\/ Q`) is true when at least one of `P` or `Q` is true."
                    .to_string(),
            );
        entries.insert(
            "Not".to_string(),
            "Logical negation.\n\n`Not P` (written `\\neg P`) is `P -> False`.".to_string(),
        );
        entries.insert(
            "Iff".to_string(),
            "Logical biconditional.\n\n`Iff P Q` (written `P <-> Q`) means `P -> Q` and `Q -> P`."
                .to_string(),
        );
        entries
            .insert(
                "Exists".to_string(),
                "Existential quantifier.\n\n`Exists (fun x => P x)` (written `\\exists x, P x`) asserts that \
             some `x` satisfies `P`."
                    .to_string(),
            );
        entries
            .insert(
                "Eq".to_string(),
                "Equality.\n\n`Eq a b` (written `a = b`) is the identity type.\nClosed by `rfl` when `a` is definitionally equal to `b`."
                    .to_string(),
            );
        entries.insert(
            "True".to_string(),
            "The trivially true proposition.\n\nProved by `True.intro` or the `trivial` tactic."
                .to_string(),
        );
        entries
            .insert(
                "False".to_string(),
                "The false proposition.\n\nHas no constructors. `False.elim : False -> a` gives ex falso."
                    .to_string(),
            );
        Self { entries }
    }
    /// Get documentation for a name.
    pub fn get(&self, name: &str) -> Option<&str> {
        self.entries.get(name).map(String::as_str)
    }
    /// Add documentation for a name.
    pub fn insert(&mut self, name: &str, doc: &str) {
        self.entries.insert(name.to_string(), doc.to_string());
    }
    /// Extract documentation from doc comments in source.
    pub fn extract_from_source(&mut self, _uri: &str, content: &str) {
        let mut lexer = Lexer::new(content);
        let tokens = lexer.tokenize();
        let mut pending_doc: Option<String> = None;
        for i in 0..tokens.len() {
            match &tokens[i].kind {
                TokenKind::DocComment(text) => {
                    let doc_text = text
                        .trim_start_matches('-')
                        .trim_end_matches('-')
                        .trim()
                        .to_string();
                    if let Some(ref mut existing) = pending_doc {
                        existing.push('\n');
                        existing.push_str(&doc_text);
                    } else {
                        pending_doc = Some(doc_text);
                    }
                }
                TokenKind::Definition
                | TokenKind::Theorem
                | TokenKind::Lemma
                | TokenKind::Axiom
                | TokenKind::Inductive
                | TokenKind::Structure
                | TokenKind::Class => {
                    if let Some(doc) = pending_doc.take() {
                        if i + 1 < tokens.len() {
                            if let TokenKind::Ident(name) = &tokens[i + 1].kind {
                                self.entries.insert(name.clone(), doc);
                            }
                        }
                    }
                }
                _ => {
                    if !matches!(tokens[i].kind, TokenKind::DocComment(_)) {
                        pending_doc = None;
                    }
                }
            }
        }
    }
    /// Get the number of entries.
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    /// Check if empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}
/// A section in the hover content.
#[derive(Clone, Debug)]
pub struct HoverSection {
    /// Section title (optional).
    pub title: Option<String>,
    /// Section content (markdown).
    pub content: String,
    /// Section kind for ordering.
    pub kind: HoverSectionKind,
}
/// How much detail to show in hover information.
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum HoverPrecisionLevel {
    /// Just the name
    Minimal,
    /// Name and type
    TypeOnly,
    /// Name, type, and documentation
    Full,
    /// Everything including source and examples
    Verbose,
}
/// How to display type information.
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TypeDisplayStyle {
    /// Full expanded type
    Full,
    /// Abbreviated type (truncated)
    Abbreviated,
    /// Only the return type
    ReturnTypeOnly,
}
/// Context for a hover request.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct HoverContext {
    pub uri: String,
    pub line: u32,
    pub character: u32,
    pub trigger: HoverTriggerPattern,
}
impl HoverContext {
    /// Create a new hover context.
    #[allow(dead_code)]
    pub fn new(uri: impl Into<String>, line: u32, character: u32) -> Self {
        Self {
            uri: uri.into(),
            line,
            character,
            trigger: HoverTriggerPattern::Always,
        }
    }
}
/// A proof goal with hypotheses and target.
#[derive(Clone, Debug)]
pub struct GoalInfo {
    /// Goal hypotheses.
    pub hypotheses: Vec<HypothesisInfo>,
    /// Goal target type.
    pub target: String,
}
/// Kind of hover section for ordering.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum HoverSectionKind {
    /// Type signature (shown first).
    Signature = 0,
    /// Type information.
    TypeInfo = 1,
    /// Documentation comment.
    Documentation = 2,
    /// Goal state (in proof context).
    GoalState = 3,
    /// Source location.
    SourceInfo = 4,
    /// LaTeX rendering hint.
    LatexHint = 5,
    /// Additional information.
    Extra = 6,
}
/// A source range reference for hover spans.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HoverSpan {
    /// URI of the file.
    pub uri: String,
    /// Start line (0-based).
    pub start_line: u32,
    /// Start column (0-based).
    pub start_col: u32,
    /// End line (0-based).
    pub end_line: u32,
    /// End column (0-based).
    pub end_col: u32,
}
impl HoverSpan {
    /// Create a hover span.
    pub fn new(
        uri: impl Into<String>,
        start_line: u32,
        start_col: u32,
        end_line: u32,
        end_col: u32,
    ) -> Self {
        Self {
            uri: uri.into(),
            start_line,
            start_col,
            end_line,
            end_col,
        }
    }
    /// Whether this span is a single point.
    pub fn is_point(&self) -> bool {
        self.start_line == self.end_line && self.start_col == self.end_col
    }
    /// Length in lines.
    pub fn line_span(&self) -> u32 {
        self.end_line - self.start_line
    }
}
/// Builder for constructing rich hover content with multiple sections.
pub struct HoverContentBuilder {
    /// Sections of the hover content.
    sections: Vec<HoverSection>,
}
impl HoverContentBuilder {
    /// Create a new builder.
    pub fn new() -> Self {
        Self {
            sections: Vec::new(),
        }
    }
    /// Add a signature section (code block).
    pub fn add_signature(&mut self, kind: &str, name: &str, ty: &str) -> &mut Self {
        self.sections.push(HoverSection {
            title: None,
            content: format!("```lean\n{} {} : {}\n```", kind, name, ty),
            kind: HoverSectionKind::Signature,
        });
        self
    }
    /// Add a simple type annotation section.
    pub fn add_type(&mut self, name: &str, ty: &str) -> &mut Self {
        self.sections.push(HoverSection {
            title: None,
            content: format!("```lean\n{} : {}\n```", name, ty),
            kind: HoverSectionKind::TypeInfo,
        });
        self
    }
    /// Add a documentation section.
    pub fn add_documentation(&mut self, doc: &str) -> &mut Self {
        self.sections.push(HoverSection {
            title: Some("Documentation".to_string()),
            content: doc.to_string(),
            kind: HoverSectionKind::Documentation,
        });
        self
    }
    /// Add a goal state section.
    pub fn add_goal_state(&mut self, goals: &[GoalInfo]) -> &mut Self {
        if goals.is_empty() {
            return self;
        }
        let mut content = String::new();
        content.push_str("**Goals:**\n\n");
        for (i, goal) in goals.iter().enumerate() {
            if goals.len() > 1 {
                content.push_str(&format!("**Goal {}:**\n", i + 1));
            }
            for hyp in &goal.hypotheses {
                content.push_str(&format!("- `{} : {}`\n", hyp.name, hyp.ty));
            }
            content.push_str(&format!("\n```lean\n-- |- {}\n```\n\n", goal.target));
        }
        self.sections.push(HoverSection {
            title: Some("Proof State".to_string()),
            content,
            kind: HoverSectionKind::GoalState,
        });
        self
    }
    /// Add source information.
    pub fn add_source_info(&mut self, uri: &str, line: u32) -> &mut Self {
        self.sections.push(HoverSection {
            title: None,
            content: format!("*Defined at* `{}` line {}", uri, line + 1),
            kind: HoverSectionKind::SourceInfo,
        });
        self
    }
    /// Add a LaTeX rendering hint.
    pub fn add_latex_hint(&mut self, latex: &str) -> &mut Self {
        self.sections.push(HoverSection {
            title: Some("Math".to_string()),
            content: format!("$${}$$", latex),
            kind: HoverSectionKind::LatexHint,
        });
        self
    }
    /// Add custom extra information.
    pub fn add_extra(&mut self, title: &str, content: &str) -> &mut Self {
        self.sections.push(HoverSection {
            title: Some(title.to_string()),
            content: content.to_string(),
            kind: HoverSectionKind::Extra,
        });
        self
    }
    /// Build the final hover content as markdown.
    pub fn build(&mut self) -> String {
        self.sections.sort_by_key(|s| s.kind);
        let mut result = String::new();
        for (i, section) in self.sections.iter().enumerate() {
            if i > 0 {
                result.push_str("\n\n---\n\n");
            }
            if let Some(ref title) = section.title {
                result.push_str(&format!("**{}**\n\n", title));
            }
            result.push_str(&section.content);
        }
        result
    }
    /// Build as an LSP Hover object.
    pub fn build_hover(&mut self, range: Option<Range>) -> Hover {
        let content = self.build();
        Hover::new(MarkupContent::markdown(content), range)
    }
    /// Check if any sections have been added.
    pub fn is_empty(&self) -> bool {
        self.sections.is_empty()
    }
}
/// Cache for hover documentation lookups.
#[allow(dead_code)]
pub struct HoverDocumentationCache {
    entries: std::collections::HashMap<String, HoverDocEntry>,
    max_size: usize,
}
impl HoverDocumentationCache {
    /// Create a new cache.
    #[allow(dead_code)]
    pub fn new(max_size: usize) -> Self {
        Self {
            entries: std::collections::HashMap::new(),
            max_size,
        }
    }
    /// Store an entry.
    #[allow(dead_code)]
    pub fn store(&mut self, name: String, type_str: String, documentation: String) {
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
        self.entries.insert(
            name.clone(),
            HoverDocEntry {
                name,
                type_str,
                documentation,
                timestamp: std::time::Instant::now(),
            },
        );
    }
    /// Look up an entry by name.
    #[allow(dead_code)]
    pub fn get(&self, name: &str) -> Option<&HoverDocEntry> {
        self.entries.get(name)
    }
    /// Clear all entries.
    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.entries.clear();
    }
}
/// Hover information for a symbol.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct HoverInfo {
    pub name: String,
    pub type_str: String,
    pub documentation: Option<String>,
    pub definition_uri: Option<String>,
    pub definition_line: Option<u32>,
    pub is_deprecated: bool,
    pub tags: Vec<String>,
}
impl HoverInfo {
    /// Create minimal hover info.
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>, type_str: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            type_str: type_str.into(),
            documentation: None,
            definition_uri: None,
            definition_line: None,
            is_deprecated: false,
            tags: vec![],
        }
    }
    /// Add documentation.
    #[allow(dead_code)]
    pub fn with_documentation(mut self, doc: impl Into<String>) -> Self {
        self.documentation = Some(doc.into());
        self
    }
    /// Add a definition location.
    #[allow(dead_code)]
    pub fn with_definition(mut self, uri: impl Into<String>, line: u32) -> Self {
        self.definition_uri = Some(uri.into());
        self.definition_line = Some(line);
        self
    }
    /// Mark as deprecated.
    #[allow(dead_code)]
    pub fn deprecated(mut self) -> Self {
        self.is_deprecated = true;
        self
    }
    /// Add a tag.
    #[allow(dead_code)]
    pub fn with_tag(mut self, tag: impl Into<String>) -> Self {
        self.tags.push(tag.into());
        self
    }
}
/// Chains multiple hover providers, returning the first result.
#[allow(dead_code)]
pub struct HoverChain {
    providers: Vec<Box<dyn HoverInfoProvider>>,
}
impl HoverChain {
    /// Create a new chain.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self { providers: vec![] }
    }
    /// Add a provider.
    #[allow(dead_code)]
    pub fn add(&mut self, provider: Box<dyn HoverInfoProvider>) {
        self.providers.push(provider);
    }
    /// Get hover info, trying each provider in order.
    #[allow(dead_code)]
    pub fn hover_for(&self, name: &str) -> Option<HoverInfo> {
        for provider in &self.providers {
            if let Some(info) = provider.hover_for(name) {
                return Some(info);
            }
        }
        None
    }
}
/// Formats type strings for hover display.
#[allow(dead_code)]
pub struct HoverTypeFormatter {
    pub max_line_len: usize,
    pub indent: String,
}
impl HoverTypeFormatter {
    /// Create a formatter with defaults.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            max_line_len: 80,
            indent: "  ".to_string(),
        }
    }
    /// Format a type string, inserting line breaks for long types.
    #[allow(dead_code)]
    pub fn format(&self, type_str: &str) -> String {
        if type_str.len() <= self.max_line_len {
            return type_str.to_string();
        }
        let mut result = String::new();
        let mut depth = 0i32;
        let mut line_len = 0usize;
        for ch in type_str.chars() {
            match ch {
                '(' | '[' | '{' => {
                    depth += 1;
                    result.push(ch);
                    line_len += 1;
                }
                ')' | ']' | '}' => {
                    depth -= 1;
                    result.push(ch);
                    line_len += 1;
                }
                _ => {
                    if line_len > self.max_line_len && depth == 0 && ch == ' ' {
                        result.push('\n');
                        result.push_str(&self.indent);
                        line_len = self.indent.len();
                    } else {
                        result.push(ch);
                        line_len += 1;
                    }
                }
            }
        }
        result
    }
}
/// A breadcrumb navigation item in hover content.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct HoverBreadcrumb {
    pub label: String,
    pub uri: Option<String>,
    pub line: Option<u32>,
}
impl HoverBreadcrumb {
    /// Create a breadcrumb with a navigation target.
    #[allow(dead_code)]
    pub fn with_target(label: impl Into<String>, uri: impl Into<String>, line: u32) -> Self {
        Self {
            label: label.into(),
            uri: Some(uri.into()),
            line: Some(line),
        }
    }
    /// Create a plain breadcrumb without a navigation target.
    #[allow(dead_code)]
    pub fn plain(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            uri: None,
            line: None,
        }
    }
    /// Render as markdown link.
    #[allow(dead_code)]
    pub fn to_markdown(&self) -> String {
        if let (Some(uri), Some(line)) = (&self.uri, self.line) {
            format!("[{}]({}#L{})", self.label, uri, line)
        } else {
            self.label.clone()
        }
    }
}
/// A tactic documentation entry.
#[derive(Clone, Debug)]
pub struct TacticDoc {
    /// Tactic name.
    pub name: String,
    /// Short description.
    pub summary: String,
    /// Usage example.
    pub example: Option<String>,
}
impl TacticDoc {
    /// Create a tactic doc entry.
    pub fn new(name: &str, summary: &str) -> Self {
        Self {
            name: name.to_string(),
            summary: summary.to_string(),
            example: None,
        }
    }
    /// Add an example.
    pub fn with_example(mut self, example: &str) -> Self {
        self.example = Some(example.to_string());
        self
    }
    /// Format as markdown.
    pub fn to_markdown(&self) -> String {
        let mut s = format!("**Tactic `{}`**\n\n{}", self.name, self.summary);
        if let Some(ex) = &self.example {
            s.push_str(&format!("\n\n```lean\n{}\n```", ex));
        }
        s
    }
}
/// A single cached documentation entry.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct HoverDocEntry {
    pub name: String,
    pub type_str: String,
    pub documentation: String,
    pub timestamp: std::time::Instant,
}
/// Pattern that determines when to show hover info.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum HoverTriggerPattern {
    /// Always show on identifier
    Always,
    /// Only show after explicit request
    OnRequest,
    /// Show with a delay in ms
    Delayed(u64),
}
/// Advanced hover provider with rich information.
pub struct AdvHoverProvider<'a> {
    /// Reference to the kernel environment.
    env: &'a Environment,
    /// Documentation database.
    doc_db: DocumentationDb,
    /// LaTeX rendering hints.
    latex_hints: LatexHintDb,
}
impl<'a> AdvHoverProvider<'a> {
    /// Create a new advanced hover provider.
    pub fn new(env: &'a Environment) -> Self {
        Self {
            env,
            doc_db: DocumentationDb::new(),
            latex_hints: LatexHintDb::new(),
        }
    }
    /// Get hover information at a position.
    pub fn hover_at(&self, doc: &Document, pos: &Position) -> Option<Hover> {
        let (word, range) = doc.word_at_position(pos)?;
        let mut builder = HoverContentBuilder::new();
        if let Some(info) = self.hover_keyword(&word) {
            builder.add_documentation(&info);
            return Some(builder.build_hover(Some(range)));
        }
        if let Some(ci) = self.env.find(&Name::str(&word)) {
            let ty_str = format!("{:?}", ci.ty());
            let kind = if ci.is_inductive() {
                "inductive"
            } else if ci.is_constructor() {
                "constructor"
            } else if ci.is_axiom() {
                "axiom"
            } else if ci.is_theorem() {
                "theorem"
            } else {
                "def"
            };
            builder.add_signature(kind, &word, &ty_str);
            if let Some(doc_text) = self.doc_db.get(&word) {
                builder.add_documentation(doc_text);
            }
            if let Some(latex) = self.latex_hints.get(&word) {
                builder.add_latex_hint(latex);
            }
            return Some(builder.build_hover(Some(range)));
        }
        let analysis = analyze_document(&doc.uri, &doc.content, self.env);
        for def in &analysis.definitions {
            if def.name == word {
                let kind_str = match def.kind {
                    SymbolKind::Function => "def",
                    SymbolKind::Method => "theorem",
                    SymbolKind::Constant => "axiom",
                    SymbolKind::Enum => "inductive",
                    SymbolKind::Struct => "structure",
                    SymbolKind::Class => "class",
                    _ => "declaration",
                };
                let ty_info = def.ty.as_deref().unwrap_or("_");
                builder.add_signature(kind_str, &word, ty_info);
                builder.add_source_info(&doc.uri, def.range.start.line);
                return Some(builder.build_hover(Some(range)));
            }
        }
        if let Some(info) = self.hover_tactic(&word) {
            builder.add_documentation(&info);
            return Some(builder.build_hover(Some(range)));
        }
        if let Some(info) = self.hover_literal(&word) {
            builder.add_documentation(&info);
            return Some(builder.build_hover(Some(range)));
        }
        if self.is_in_proof(doc, pos) {
            let goals = self.compute_goal_at(doc, pos);
            if !goals.is_empty() {
                builder.add_goal_state(&goals);
                if !builder.is_empty() {
                    return Some(builder.build_hover(Some(range)));
                }
            }
        }
        None
    }
    /// Get hover info for a keyword.
    fn hover_keyword(&self, word: &str) -> Option<String> {
        KEYWORD_HOVER_DOCS.iter().find_map(|&(kw, doc)| {
            if kw == word {
                Some(doc.to_string())
            } else {
                None
            }
        })
    }
    /// Get hover info for a tactic.
    fn hover_tactic(&self, word: &str) -> Option<String> {
        TACTIC_HOVER_DOCS.iter().find_map(|&(name, doc)| {
            if name == word {
                Some(format!("**Tactic** `{}`\n\n{}", name, doc))
            } else {
                None
            }
        })
    }
    /// Get hover info for a literal.
    fn hover_literal(&self, word: &str) -> Option<String> {
        if word.chars().all(|c| c.is_ascii_digit()) {
            let _val: u64 = word.parse().ok()?;
            return Some(format!(
                "```lean\n{} : Nat\n```\nNatural number literal",
                word
            ));
        }
        if word.starts_with('-') && word[1..].chars().all(|c| c.is_ascii_digit()) {
            let _val: i64 = word.parse().ok()?;
            return Some(format!("```lean\n{} : Int\n```\nInteger literal", word));
        }
        if word == "true" || word == "false" {
            return Some(format!("```lean\n{} : Bool\n```\nBoolean literal", word));
        }
        None
    }
    /// Check if a position is inside a proof block.
    fn is_in_proof(&self, doc: &Document, pos: &Position) -> bool {
        let mut line_idx = pos.line as usize;
        while line_idx > 0 {
            if let Some(line) = doc.get_line(line_idx as u32) {
                let trimmed = line.trim();
                if trimmed == "by" || trimmed.starts_with("by ") {
                    return true;
                }
                if ["def", "theorem", "lemma", "axiom", "inductive"]
                    .iter()
                    .any(|kw| trimmed.starts_with(kw))
                {
                    return false;
                }
            }
            line_idx -= 1;
        }
        false
    }
    /// Compute the goal state at a position (simplified).
    fn compute_goal_at(&self, doc: &Document, pos: &Position) -> Vec<GoalInfo> {
        let analysis = analyze_document(&doc.uri, &doc.content, self.env);
        let mut current_theorem = None;
        for def in &analysis.definitions {
            if def.kind == SymbolKind::Method && def.range.start.line <= pos.line {
                current_theorem = Some(def);
            }
        }
        if let Some(thm) = current_theorem {
            let target = thm.ty.as_deref().unwrap_or("?goal").to_string();
            vec![GoalInfo {
                hypotheses: Vec::new(),
                target,
            }]
        } else {
            Vec::new()
        }
    }
    /// Get hover info with goal state for proof positions.
    pub fn hover_with_goals(
        &self,
        doc: &Document,
        pos: &Position,
        goals: &[GoalInfo],
    ) -> Option<Hover> {
        let (word, range) = doc.word_at_position(pos)?;
        let mut builder = HoverContentBuilder::new();
        if let Some(info) = self
            .hover_keyword(&word)
            .or_else(|| self.hover_tactic(&word))
            .or_else(|| self.hover_literal(&word))
        {
            builder.add_documentation(&info);
        }
        if !goals.is_empty() {
            builder.add_goal_state(goals);
        }
        if builder.is_empty() {
            return None;
        }
        Some(builder.build_hover(Some(range)))
    }
}
/// Database for LaTeX rendering hints for mathematical symbols.
pub struct LatexHintDb {
    /// Mapping from OxiLean symbol to LaTeX.
    entries: HashMap<String, String>,
}
impl LatexHintDb {
    /// Create with standard mathematical symbol mappings.
    pub fn new() -> Self {
        let mut entries = HashMap::new();
        entries.insert("And".to_string(), "P \\land Q".to_string());
        entries.insert("Or".to_string(), "P \\lor Q".to_string());
        entries.insert("Not".to_string(), "\\neg P".to_string());
        entries.insert("Iff".to_string(), "P \\leftrightarrow Q".to_string());
        entries.insert("True".to_string(), "\\top".to_string());
        entries.insert("False".to_string(), "\\bot".to_string());
        entries.insert("Exists".to_string(), "\\exists x, P(x)".to_string());
        entries.insert("Nat".to_string(), "\\mathbb{N}".to_string());
        entries.insert("Int".to_string(), "\\mathbb{Z}".to_string());
        entries.insert("Float".to_string(), "\\mathbb{R}".to_string());
        entries.insert("Prop".to_string(), "\\mathrm{Prop}".to_string());
        entries.insert("Nat.add".to_string(), "a + b".to_string());
        entries.insert("Nat.mul".to_string(), "a \\times b".to_string());
        entries.insert("Nat.sub".to_string(), "a - b".to_string());
        entries.insert("Nat.div".to_string(), "a \\div b".to_string());
        entries.insert("Nat.mod".to_string(), "a \\bmod b".to_string());
        entries.insert("Nat.pow".to_string(), "a^b".to_string());
        entries.insert("Eq".to_string(), "a = b".to_string());
        entries.insert("Ne".to_string(), "a \\neq b".to_string());
        entries.insert("LE.le".to_string(), "a \\leq b".to_string());
        entries.insert("LT.lt".to_string(), "a < b".to_string());
        entries.insert("GE.ge".to_string(), "a \\geq b".to_string());
        entries.insert("GT.gt".to_string(), "a > b".to_string());
        entries.insert("List.length".to_string(), "|\\ell|".to_string());
        entries.insert(
            "List.map".to_string(),
            "\\mathrm{map}\\; f\\; \\ell".to_string(),
        );
        entries.insert("Function.comp".to_string(), "f \\circ g".to_string());
        entries.insert("Sum".to_string(), "\\sum".to_string());
        entries.insert("Prod".to_string(), "\\prod".to_string());
        Self { entries }
    }
    /// Get the LaTeX hint for a name.
    pub fn get(&self, name: &str) -> Option<&str> {
        self.entries.get(name).map(String::as_str)
    }
    /// Add a LaTeX hint.
    pub fn insert(&mut self, name: &str, latex: &str) {
        self.entries.insert(name.to_string(), latex.to_string());
    }
    /// Convert a type expression to LaTeX-like rendering.
    pub fn type_to_latex(&self, ty: &str) -> String {
        let mut result = ty.to_string();
        let replacements = [
            ("Nat", "\\mathbb{N}"),
            ("Int", "\\mathbb{Z}"),
            ("Prop", "\\mathrm{Prop}"),
            ("->", "\\to"),
            ("=>", "\\Rightarrow"),
            ("forall", "\\forall"),
            ("/\\", "\\land"),
            ("\\/", "\\lor"),
            ("=", "="),
        ];
        for (from, to) in &replacements {
            result = result.replace(from, to);
        }
        result
    }
    /// Get the number of entries.
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    /// Check if empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}
/// The kind of item being hovered.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum HoveredKind {
    /// A definition or theorem.
    Declaration,
    /// A tactic keyword.
    Tactic,
    /// A language keyword.
    Keyword,
    /// A numeric literal.
    Literal,
    /// A type class.
    TypeClass,
    /// An instance.
    Instance,
    /// Unknown.
    Unknown,
}
/// Renders hover content as markdown.
#[allow(dead_code)]
pub struct HoverMarkdownRenderer {
    pub include_source: bool,
    pub include_examples: bool,
    pub max_type_length: usize,
}
impl HoverMarkdownRenderer {
    /// Create a new renderer with defaults.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            include_source: true,
            include_examples: true,
            max_type_length: 200,
        }
    }
    /// Render a type annotation as markdown.
    #[allow(dead_code)]
    pub fn render_type(&self, name: &str, type_str: &str) -> String {
        let truncated = if type_str.len() > self.max_type_length {
            format!("{}…", &type_str[..self.max_type_length])
        } else {
            type_str.to_string()
        };
        format!("```lean\n{} : {}\n```", name, truncated)
    }
    /// Render documentation as markdown.
    #[allow(dead_code)]
    pub fn render_doc(&self, doc: &str) -> String {
        format!("{}\n", doc.trim())
    }
    /// Render a source code snippet as markdown.
    #[allow(dead_code)]
    pub fn render_source(&self, source: &str) -> String {
        if self.include_source {
            format!("**Source:**\n```lean\n{}\n```\n", source.trim())
        } else {
            String::new()
        }
    }
    /// Render an example as markdown.
    #[allow(dead_code)]
    pub fn render_example(&self, example: &str) -> String {
        if self.include_examples {
            format!("**Example:**\n```lean\n{}\n```\n", example.trim())
        } else {
            String::new()
        }
    }
    /// Render a full hover entry.
    #[allow(dead_code)]
    pub fn render_entry(
        &self,
        name: &str,
        type_str: &str,
        doc: Option<&str>,
        source: Option<&str>,
        examples: &[&str],
    ) -> String {
        let mut out = self.render_type(name, type_str);
        out.push('\n');
        if let Some(d) = doc {
            out.push_str(&self.render_doc(d));
            out.push('\n');
        }
        if let Some(src) = source {
            out.push_str(&self.render_source(src));
        }
        for example in examples {
            out.push_str(&self.render_example(example));
        }
        out
    }
}
/// Provides hover info for built-in OxiLean symbols.
#[allow(dead_code)]
pub struct StaticHoverProvider {
    pub data: std::collections::HashMap<String, HoverInfo>,
}
impl StaticHoverProvider {
    /// Create a provider with built-in OxiLean data.
    #[allow(dead_code)]
    pub fn builtin() -> Self {
        let mut data = std::collections::HashMap::new();
        data.insert(
            "Nat".to_string(),
            HoverInfo::new("Nat", "Type")
                .with_documentation("The type of natural numbers (non-negative integers).")
                .with_tag("builtin"),
        );
        data.insert(
            "Int".to_string(),
            HoverInfo::new("Int", "Type")
                .with_documentation("The type of integers.")
                .with_tag("builtin"),
        );
        data.insert(
            "Bool".to_string(),
            HoverInfo::new("Bool", "Type")
                .with_documentation("The boolean type with values `true` and `false`.")
                .with_tag("builtin"),
        );
        data.insert(
            "Prop".to_string(),
            HoverInfo::new("Prop", "Sort")
                .with_documentation("The sort of propositions.")
                .with_tag("builtin"),
        );
        data.insert(
            "Type".to_string(),
            HoverInfo::new("Type", "Sort 1")
                .with_documentation("The sort of types.")
                .with_tag("builtin"),
        );
        data.insert(
            "true".to_string(),
            HoverInfo::new("true", "Bool")
                .with_documentation("The boolean value `true`.")
                .with_tag("builtin"),
        );
        data.insert(
            "false".to_string(),
            HoverInfo::new("false", "Bool")
                .with_documentation("The boolean value `false`.")
                .with_tag("builtin"),
        );
        data.insert(
            "Nat.add".to_string(),
            HoverInfo::new("Nat.add", "Nat -> Nat -> Nat")
                .with_documentation("Addition of natural numbers.")
                .with_tag("builtin"),
        );
        data.insert(
            "Nat.mul".to_string(),
            HoverInfo::new("Nat.mul", "Nat -> Nat -> Nat")
                .with_documentation("Multiplication of natural numbers.")
                .with_tag("builtin"),
        );
        data.insert(
            "Nat.sub".to_string(),
            HoverInfo::new("Nat.sub", "Nat -> Nat -> Nat")
                .with_documentation("Saturating subtraction of natural numbers.")
                .with_tag("builtin"),
        );
        Self { data }
    }
}
/// Statistics about hover operations.
#[allow(dead_code)]
#[derive(Clone, Debug, Default)]
pub struct HoverStats {
    pub total_requests: u64,
    pub cache_hits: u64,
    pub misses: u64,
    pub avg_latency_us: f64,
}
impl HoverStats {
    /// Record a cache hit.
    #[allow(dead_code)]
    pub fn record_hit(&mut self) {
        self.total_requests += 1;
        self.cache_hits += 1;
    }
    /// Record a miss with latency.
    #[allow(dead_code)]
    pub fn record_miss(&mut self, latency_us: u64) {
        self.total_requests += 1;
        self.misses += 1;
        let n = self.misses as f64;
        self.avg_latency_us = (self.avg_latency_us * (n - 1.0) + latency_us as f64) / n;
    }
    /// Return hit rate as a percentage.
    #[allow(dead_code)]
    pub fn hit_rate(&self) -> f64 {
        if self.total_requests == 0 {
            0.0
        } else {
            100.0 * self.cache_hits as f64 / self.total_requests as f64
        }
    }
}
/// Keyword documentation map.
///
/// Maps OxiLean keywords to brief human-readable explanations.
pub struct KeywordDocs {
    map: HashMap<&'static str, &'static str>,
}
impl KeywordDocs {
    /// Build the standard keyword documentation map.
    pub fn new() -> Self {
        let mut map = HashMap::new();
        map.insert("def", "Define a new definition.");
        map.insert("theorem", "Prove a theorem.");
        map.insert("axiom", "Declare an axiom (unproven postulate).");
        map.insert("lemma", "Prove an auxiliary lemma.");
        map.insert("example", "Prove an unnamed example.");
        map.insert("import", "Import another module.");
        map.insert("namespace", "Open a namespace scope.");
        map.insert("end", "Close a namespace or section scope.");
        map.insert("section", "Open a section scope.");
        map.insert("open", "Bring names into scope.");
        map.insert(
            "variable",
            "Declare a variable to be automatically introduced.",
        );
        map.insert("class", "Declare a type class.");
        map.insert("instance", "Declare a type class instance.");
        map.insert("structure", "Declare a record type.");
        map.insert("inductive", "Declare an inductive type.");
        map.insert("fun", "Introduce a function (lambda).");
        map.insert("forall", "A universal quantification.");
        map.insert("let", "Introduce a local definition.");
        map.insert("have", "Introduce a local hypothesis in a tactic proof.");
        map.insert("show", "Change the goal type in a tactic proof.");
        map.insert("match", "Pattern match on a value.");
        map.insert("with", "Provide patterns for a match expression.");
        map.insert("if", "Conditional expression.");
        map.insert("then", "The true branch of a conditional.");
        map.insert("else", "The false branch of a conditional.");
        map.insert("do", "Monadic do-notation.");
        map.insert("return", "Return a value in a monadic computation.");
        map.insert("by", "Enter tactic proof mode.");
        map.insert("exact", "Close the goal with an exact term.");
        map.insert("apply", "Apply a lemma to the current goal.");
        map.insert("intro", "Introduce a hypothesis.");
        map.insert("simp", "Simplify using simp lemmas.");
        map.insert("rw", "Rewrite using an equality.");
        map.insert("sorry", "Placeholder proof (admits the goal).");
        Self { map }
    }
    /// Look up documentation for a keyword.
    pub fn get(&self, keyword: &str) -> Option<&str> {
        self.map.get(keyword).copied()
    }
    /// Whether a string is a known keyword.
    pub fn is_keyword(&self, s: &str) -> bool {
        self.map.contains_key(s)
    }
    /// Number of documented keywords.
    pub fn len(&self) -> usize {
        self.map.len()
    }
    /// Whether the map is empty.
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
}
/// Extracts the word under the cursor in a source line.
#[allow(dead_code)]
pub struct HoverRangeExtractor;
impl HoverRangeExtractor {
    /// Extract the identifier at position `col` in `line_text`.
    #[allow(dead_code)]
    pub fn extract_word_at(line_text: &str, col: usize) -> Option<(usize, usize, String)> {
        if col > line_text.len() {
            return None;
        }
        let chars: Vec<char> = line_text.chars().collect();
        if col >= chars.len() {
            return None;
        }
        let is_ident = |c: char| c.is_alphanumeric() || c == '_' || c == '\'';
        if !is_ident(chars[col]) {
            return None;
        }
        let start = {
            let mut i = col;
            while i > 0 && is_ident(chars[i - 1]) {
                i -= 1;
            }
            i
        };
        let end = {
            let mut i = col;
            while i < chars.len() && is_ident(chars[i]) {
                i += 1;
            }
            i
        };
        let word: String = chars[start..end].iter().collect();
        Some((start, end, word))
    }
    /// Extract a qualified name (with dots) at the given position.
    #[allow(dead_code)]
    pub fn extract_qualified_at(line_text: &str, col: usize) -> Option<(usize, usize, String)> {
        let chars: Vec<char> = line_text.chars().collect();
        if col >= chars.len() {
            return None;
        }
        let is_qualified_char = |c: char| c.is_alphanumeric() || c == '_' || c == '\'' || c == '.';
        if !is_qualified_char(chars[col]) {
            return None;
        }
        let start = {
            let mut i = col;
            while i > 0 && is_qualified_char(chars[i - 1]) {
                i -= 1;
            }
            i
        };
        let end = {
            let mut i = col;
            while i < chars.len() && is_qualified_char(chars[i]) {
                i += 1;
            }
            i
        };
        let word: String = chars[start..end].iter().collect();
        Some((start, end, word))
    }
}
/// Registry of tactic documentation.
#[derive(Clone, Debug, Default)]
pub struct TacticDocRegistry {
    entries: Vec<TacticDoc>,
}
impl TacticDocRegistry {
    /// Create the standard tactic registry.
    pub fn new() -> Self {
        let entries = vec![
            TacticDoc::new("intro", "Introduce a hypothesis from a Pi-type goal.")
                .with_example("intro h"),
            TacticDoc::new("apply", "Apply a lemma or function to the current goal.")
                .with_example("apply Nat.add_comm"),
            TacticDoc::new("exact", "Close the goal with a given term.").with_example("exact rfl"),
            TacticDoc::new("simp", "Simplify using the simp lemma set.")
                .with_example("simp [Nat.add_zero]"),
            TacticDoc::new("rw", "Rewrite using an equality.").with_example("rw [← h]"),
            TacticDoc::new("cases", "Case-split on an inductive type.").with_example("cases h"),
            TacticDoc::new("induction", "Prove by structural induction.")
                .with_example("induction n with | zero => ... | succ n ih => ..."),
            TacticDoc::new("refl", "Close a reflexivity goal.").with_example("refl"),
            TacticDoc::new("constructor", "Split a conjunction or provide a product.")
                .with_example("constructor"),
            TacticDoc::new("left", "Choose the left side of a disjunction.").with_example("left"),
            TacticDoc::new("right", "Choose the right side of a disjunction.")
                .with_example("right"),
            TacticDoc::new("assumption", "Close goal using a matching hypothesis.")
                .with_example("assumption"),
            TacticDoc::new("ring", "Prove ring/semiring equalities.").with_example("ring"),
            TacticDoc::new("linarith", "Prove linear arithmetic goals.")
                .with_example("linarith [h1, h2]"),
            TacticDoc::new("norm_num", "Normalise numeric expressions.").with_example("norm_num"),
            TacticDoc::new("omega", "Decide linear arithmetic over integers.")
                .with_example("omega"),
            TacticDoc::new("contradiction", "Close a goal by finding a contradiction.")
                .with_example("contradiction"),
            TacticDoc::new("by_contra", "Introduce the negation of the goal.")
                .with_example("by_contra h"),
            TacticDoc::new("push_neg", "Push negations inward.").with_example("push_neg"),
            TacticDoc::new("have", "Introduce a new hypothesis with proof.")
                .with_example("have h : P := ..."),
            TacticDoc::new(
                "obtain",
                "Destructure an existential or conjunction hypothesis.",
            )
            .with_example("obtain ⟨a, b⟩ := h"),
            TacticDoc::new("use", "Provide an existential witness.").with_example("use 42"),
            TacticDoc::new("clear", "Remove a hypothesis from the context.")
                .with_example("clear h"),
            TacticDoc::new("revert", "Move a hypothesis back into the goal.")
                .with_example("revert h"),
            TacticDoc::new("split", "Split an if-then-else or Iff goal.").with_example("split"),
            TacticDoc::new("trivial", "Close a trivially true goal.").with_example("trivial"),
            TacticDoc::new("sorry", "Admit the current goal (placeholder).").with_example("sorry"),
        ];
        Self { entries }
    }
    /// Look up a tactic by name.
    pub fn get(&self, name: &str) -> Option<&TacticDoc> {
        self.entries.iter().find(|e| e.name == name)
    }
    /// Number of registered tactics.
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    /// Whether the registry is empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
    /// List all tactic names.
    pub fn tactic_names(&self) -> Vec<&str> {
        self.entries.iter().map(|e| e.name.as_str()).collect()
    }
}
/// A structured hover response with optional span.
#[derive(Clone, Debug)]
pub struct StructuredHover {
    /// The hover content.
    pub content: String,
    /// The span to highlight.
    pub span: Option<HoverSpan>,
    /// Kind of the hovered item.
    pub kind: HoveredKind,
}
impl StructuredHover {
    /// Create a structured hover.
    pub fn new(content: impl Into<String>, kind: HoveredKind) -> Self {
        Self {
            content: content.into(),
            span: None,
            kind,
        }
    }
    /// Attach a span.
    pub fn with_span(mut self, span: HoverSpan) -> Self {
        self.span = Some(span);
        self
    }
    /// Convert to a plain-text `Hover`.
    pub fn to_hover(&self) -> Hover {
        Hover {
            contents: MarkupContent {
                kind: MarkupKind::Markdown,
                value: self.content.clone(),
            },
            range: None,
        }
    }
}
/// A cached hover result.
#[derive(Clone, Debug)]
pub struct CachedHover {
    /// Document version when hover was computed.
    pub version: i64,
    /// The hover result.
    pub hover: Option<Hover>,
}
/// Cache for hover results.
pub struct HoverCache {
    /// Cached hover results keyed by (uri, line, character).
    entries: HashMap<(String, u32, u32), CachedHover>,
    /// Maximum cache size.
    max_size: usize,
}
impl HoverCache {
    /// Create a new hover cache.
    pub fn new(max_size: usize) -> Self {
        Self {
            entries: HashMap::new(),
            max_size,
        }
    }
    /// Get a cached hover result.
    pub fn get(
        &self,
        uri: &str,
        line: u32,
        character: u32,
        version: i64,
    ) -> Option<&Option<Hover>> {
        let key = (uri.to_string(), line, character);
        self.entries
            .get(&key)
            .filter(|c| c.version == version)
            .map(|c| &c.hover)
    }
    /// Store a hover result.
    pub fn store(
        &mut self,
        uri: &str,
        line: u32,
        character: u32,
        version: i64,
        hover: Option<Hover>,
    ) {
        if self.entries.len() >= self.max_size {
            if let Some(key) = self.entries.keys().next().cloned() {
                self.entries.remove(&key);
            }
        }
        let key = (uri.to_string(), line, character);
        self.entries.insert(key, CachedHover { version, hover });
    }
    /// Invalidate all entries for a URI.
    pub fn invalidate(&mut self, uri: &str) {
        self.entries.retain(|(u, _, _), _| u != uri);
    }
    /// Clear the entire cache.
    pub fn clear(&mut self) {
        self.entries.clear();
    }
    /// Get the cache size.
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    /// Check if the cache is empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}
/// A hover provider that caches results and serves them with priority.
///
/// Combines the `AdvHoverProvider`, `KeywordDocs`, and `TacticDocRegistry`
/// into a single unified interface.
pub struct UnifiedHoverProvider<'env> {
    adv: AdvHoverProvider<'env>,
    keyword_docs: KeywordDocs,
    tactic_docs: TacticDocRegistry,
    pub(crate) cache: HoverCache,
}
impl<'env> UnifiedHoverProvider<'env> {
    /// Create a new unified provider.
    pub fn new(env: &'env Environment) -> Self {
        Self {
            adv: AdvHoverProvider::new(env),
            keyword_docs: KeywordDocs::new(),
            tactic_docs: TacticDocRegistry::new(),
            cache: HoverCache::new(512),
        }
    }
    /// Serve a hover request with caching.
    pub fn hover(&mut self, doc: &Document, pos: &Position) -> Option<Hover> {
        let version = 0i64;
        if let Some(cached) = self.cache.get(&doc.uri, pos.line, pos.character, version) {
            return (*cached).clone();
        }
        let result = self.adv.hover_at(doc, pos);
        self.cache
            .store(&doc.uri, pos.line, pos.character, version, result.clone());
        result
    }
    /// Keyword documentation lookup.
    pub fn keyword_hover(&self, word: &str) -> Option<Hover> {
        self.keyword_docs.get(word).map(|doc| Hover {
            contents: MarkupContent {
                kind: MarkupKind::Markdown,
                value: format!("**Keyword `{word}`**\n\n{doc}"),
            },
            range: None,
        })
    }
    /// Tactic documentation lookup.
    pub fn tactic_hover(&self, name: &str) -> Option<Hover> {
        self.tactic_docs.get(name).map(|doc| Hover {
            contents: MarkupContent {
                kind: MarkupKind::Markdown,
                value: doc.to_markdown(),
            },
            range: None,
        })
    }
    /// Invalidate cache for a document.
    pub fn invalidate(&mut self, uri: &str) {
        self.cache.invalidate(uri);
    }
}
/// User preferences for hover display formatting.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct HoverFormatOptions {
    pub show_types: bool,
    pub show_docs: bool,
    pub show_source_link: bool,
    pub show_examples: bool,
    pub type_display_style: TypeDisplayStyle,
    pub max_doc_lines: usize,
}
impl HoverFormatOptions {
    /// Create with all information shown.
    #[allow(dead_code)]
    pub fn verbose() -> Self {
        Self {
            show_types: true,
            show_docs: true,
            show_source_link: true,
            show_examples: true,
            type_display_style: TypeDisplayStyle::Full,
            max_doc_lines: 50,
        }
    }
    /// Create minimal options.
    #[allow(dead_code)]
    pub fn minimal() -> Self {
        Self {
            show_types: true,
            show_docs: false,
            show_source_link: false,
            show_examples: false,
            type_display_style: TypeDisplayStyle::Abbreviated,
            max_doc_lines: 2,
        }
    }
}
