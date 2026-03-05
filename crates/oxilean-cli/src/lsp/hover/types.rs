//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lsp::{
    analyze_document, Document, DocumentStore, Hover, Location, MarkupContent, Position, Range,
    SymbolKind,
};
use oxilean_kernel::{Environment, Name};
use oxilean_parse::{Lexer, TokenKind};

use super::functions::*;

use std::collections::{HashMap, VecDeque};

/// A record of a hover event.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct HoverEvent {
    /// The document URI.
    pub uri: String,
    /// The position.
    pub line: u32,
    /// The character offset.
    pub character: u32,
    /// The word hovered.
    pub word: Option<String>,
    /// Whether a result was found.
    pub hit: bool,
}
/// Kinds of completion items.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CompletionKind {
    /// Keyword.
    Keyword,
    /// Function or definition.
    Function,
    /// Type.
    Type,
    /// Tactic.
    Tactic,
    /// Snippet.
    Snippet,
}
/// Provides document symbol outline for navigation.
pub struct OutlineProvider<'a> {
    /// Reference to the kernel environment.
    env: &'a Environment,
}
impl<'a> OutlineProvider<'a> {
    /// Create a new outline provider.
    pub fn new(env: &'a Environment) -> Self {
        Self { env }
    }
    /// Get the document symbol outline.
    pub fn get_outline(&self, doc: &Document) -> Vec<OutlineSymbol> {
        let analysis = analyze_document(&doc.uri, &doc.content, self.env);
        analysis
            .definitions
            .iter()
            .map(|def| OutlineSymbol {
                name: def.name.clone(),
                kind: def.kind,
                range: def.range.clone(),
                ty: def.ty.clone(),
                children: Vec::new(),
            })
            .collect()
    }
    /// Find an outline symbol by name.
    pub fn find_symbol(&self, doc: &Document, name: &str) -> Option<OutlineSymbol> {
        self.get_outline(doc)
            .into_iter()
            .find(|sym| sym.name == name)
    }
    /// Count symbols of a given kind.
    pub fn count_of_kind(&self, doc: &Document, kind: &SymbolKind) -> usize {
        self.get_outline(doc)
            .iter()
            .filter(|sym| &sym.kind == kind)
            .count()
    }
}
/// Handles LSP hover requests by coordinating multiple providers.
#[allow(dead_code)]
pub struct HoverRequestHandler<'a> {
    /// The hover provider.
    hover: HoverProvider<'a>,
    /// The signature help provider.
    signature: SignatureHelpProvider<'a>,
    /// The outline provider.
    outline: OutlineProvider<'a>,
    /// The type info provider.
    env: &'a oxilean_kernel::Environment,
    /// Cache for hover results.
    cache: HoverCache,
}
#[allow(dead_code)]
impl<'a> HoverRequestHandler<'a> {
    /// Create a new handler.
    pub fn new(env: &'a oxilean_kernel::Environment) -> Self {
        Self {
            hover: HoverProvider::new(env),
            signature: SignatureHelpProvider::new(env),
            outline: OutlineProvider::new(env),
            env,
            cache: HoverCache::new(256),
        }
    }
    /// Handle a hover request, returning a Hover LSP value.
    pub fn handle(&mut self, doc: &Document, pos: &Position) -> Option<Hover> {
        if let Some(cached) = self.cache.get(&doc.uri, pos.line, pos.character) {
            return cached.as_ref().map(|r| self.hover.to_lsp_hover(r));
        }
        let result = self.hover.hover_at_position(doc, pos);
        let lsp_hover = result.as_ref().map(|r| self.hover.to_lsp_hover(r));
        self.cache.insert(&doc.uri, pos.line, pos.character, result);
        lsp_hover
    }
    /// Invalidate cache for a document when it changes.
    pub fn on_document_change(&mut self, uri: &str) {
        self.cache.invalidate(uri);
    }
    /// Clear the entire cache.
    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }
    /// Return current cache statistics.
    pub fn cache_stats(&self) -> (usize, usize) {
        (self.cache.len(), 256)
    }
}
/// Represents a proof goal for display.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct ProofGoal {
    /// Named hypotheses in the context.
    pub hypotheses: Vec<(String, String)>,
    /// The goal type.
    pub goal: String,
    /// Optional label for the goal.
    pub label: Option<String>,
}
#[allow(dead_code)]
impl ProofGoal {
    /// Create a new proof goal.
    pub fn new(goal: impl Into<String>) -> Self {
        Self {
            hypotheses: Vec::new(),
            goal: goal.into(),
            label: None,
        }
    }
    /// Add a hypothesis.
    pub fn add_hyp(&mut self, name: impl Into<String>, ty: impl Into<String>) {
        self.hypotheses.push((name.into(), ty.into()));
    }
    /// Format the proof goal for hover display.
    pub fn format_hover(&self) -> String {
        let mut result = String::from("**Proof State**\n\n```lean\n");
        for (name, ty) in &self.hypotheses {
            result.push_str(&format!("{} : {}\n", name, ty));
        }
        result.push_str("⊢ ");
        result.push_str(&self.goal);
        result.push_str("\n```");
        if let Some(label) = &self.label {
            result.push_str(&format!("\n\nGoal: *{}*", label));
        }
        result
    }
    /// Check if the goal is closed (proved).
    pub fn is_closed(&self) -> bool {
        self.goal == "True" || self.goal.is_empty()
    }
    /// Return the number of hypotheses.
    pub fn hyp_count(&self) -> usize {
        self.hypotheses.len()
    }
}
/// Generates structured documentation for hover display.
#[allow(dead_code)]
pub struct DocumentationGenerator;
#[allow(dead_code)]
impl DocumentationGenerator {
    /// Create a new documentation generator.
    pub fn new() -> Self {
        Self
    }
    /// Generate documentation for a declaration.
    pub fn generate_decl_doc(
        &self,
        kind: &str,
        name: &str,
        ty: &str,
        doc_comment: Option<&str>,
    ) -> String {
        let mut result = format!("```lean\n{} {} : {}\n```\n", kind, name, ty);
        if let Some(doc) = doc_comment {
            result.push('\n');
            result.push_str(doc);
        }
        result
    }
    /// Generate a brief one-liner for a keyword.
    pub fn generate_keyword_brief(&self, kw: &str) -> Option<String> {
        let brief =
            KEYWORD_BRIEFS.iter().find_map(
                |&(k, b)| {
                    if k == kw {
                        Some(b.to_string())
                    } else {
                        None
                    }
                },
            );
        brief
    }
    /// Generate documentation for a tactic with an example.
    pub fn generate_tactic_doc(
        &self,
        name: &str,
        description: &str,
        example: Option<&str>,
    ) -> String {
        let mut doc = format!("**Tactic** `{}`\n\n{}", name, description);
        if let Some(ex) = example {
            doc.push_str(&format!("\n\n**Example:**\n```lean\n{}\n```", ex));
        }
        doc
    }
    /// Format a list of type alternatives for union-like types.
    pub fn format_type_alternatives(&self, alternatives: &[&str]) -> String {
        if alternatives.is_empty() {
            return String::new();
        }
        format!(
            "One of:\n{}",
            alternatives
                .iter()
                .map(|a| format!("- `{}`", a))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}
/// Strategy for combining multiple hover providers.
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HoverMergeStrategy {
    /// Return the first result found.
    FirstWins,
    /// Concatenate all results.
    ConcatenateAll,
    /// Return the result with the most content.
    LongestWins,
}
/// Tracks recent hover events for diagnostics.
#[allow(dead_code)]
pub struct HoverHistory {
    events: std::collections::VecDeque<HoverEvent>,
    max_size: usize,
}
#[allow(dead_code)]
impl HoverHistory {
    /// Create a new hover history with limited size.
    pub fn new(max_size: usize) -> Self {
        Self {
            events: std::collections::VecDeque::new(),
            max_size,
        }
    }
    /// Record a hover event.
    pub fn record(&mut self, event: HoverEvent) {
        if self.events.len() >= self.max_size {
            self.events.pop_front();
        }
        self.events.push_back(event);
    }
    /// Return the hit rate (fraction of events with results).
    pub fn hit_rate(&self) -> f64 {
        if self.events.is_empty() {
            return 0.0;
        }
        let hits = self.events.iter().filter(|e| e.hit).count();
        hits as f64 / self.events.len() as f64
    }
    /// Return the total number of recorded events.
    pub fn event_count(&self) -> usize {
        self.events.len()
    }
    /// Return recent events as a slice.
    pub fn recent_events(&self) -> &std::collections::VecDeque<HoverEvent> {
        &self.events
    }
    /// Clear all events.
    pub fn clear(&mut self) {
        self.events.clear();
    }
}
/// A collection of proof goals (multi-goal state).
#[allow(dead_code)]
#[derive(Clone, Debug, Default)]
pub struct ProofState {
    /// The current goals.
    pub goals: Vec<ProofGoal>,
}
#[allow(dead_code)]
impl ProofState {
    /// Create a new proof state.
    pub fn new() -> Self {
        Self::default()
    }
    /// Add a goal.
    pub fn add_goal(&mut self, goal: ProofGoal) {
        self.goals.push(goal);
    }
    /// Format for hover display.
    pub fn format_hover(&self) -> String {
        if self.goals.is_empty() {
            return "**No goals remaining** ✓".to_string();
        }
        if self.goals.len() == 1 {
            return self.goals[0].format_hover();
        }
        let mut result = format!("**{} Goals**\n\n", self.goals.len());
        for (i, goal) in self.goals.iter().enumerate() {
            result.push_str(&format!("**Goal {}:**\n", i + 1));
            result.push_str(&goal.format_hover());
            result.push('\n');
        }
        result
    }
    /// Return the number of remaining goals.
    pub fn goal_count(&self) -> usize {
        self.goals.len()
    }
    /// Return true if all goals are closed.
    pub fn all_closed(&self) -> bool {
        self.goals.iter().all(|g| g.is_closed())
    }
}
/// Provides hover info for namespace and module references.
#[allow(dead_code)]
pub struct NamespaceHoverProvider<'a> {
    env: &'a oxilean_kernel::Environment,
}
#[allow(dead_code)]
impl<'a> NamespaceHoverProvider<'a> {
    /// Create a new namespace hover provider.
    pub fn new(env: &'a oxilean_kernel::Environment) -> Self {
        Self { env }
    }
    /// Get hover info for a qualified name (e.g., `Nat.succ`).
    pub fn hover_qualified_name(&self, qualified: &str) -> Option<String> {
        let name = oxilean_kernel::Name::str(qualified);
        if let Some(ci) = self.env.find(&name) {
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
                "definition"
            };
            return Some(format_declaration_hover(kind, qualified, &ty_str));
        }
        let namespace_prefix = format!("{}.", qualified);
        let mut items: Vec<String> = self
            .env
            .constant_names()
            .filter(|n| n.to_string().starts_with(&namespace_prefix))
            .map(|n| n.to_string())
            .take(10)
            .collect();
        items.sort();
        if !items.is_empty() {
            let list = items
                .iter()
                .map(|i| format!("- `{}`", i))
                .collect::<Vec<_>>()
                .join("\n");
            return Some(format!(
                "**Namespace** `{}`\n\nContains:\n{}",
                qualified, list
            ));
        }
        None
    }
    /// Count the number of declarations in a namespace prefix.
    pub fn count_in_namespace(&self, prefix: &str) -> usize {
        let ns_prefix = format!("{}.", prefix);
        self.env
            .constant_names()
            .filter(|n| n.to_string().starts_with(&ns_prefix))
            .count()
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct HoverSection {
    pub kind: HoverSectionKind,
    pub content: String,
}
#[allow(dead_code)]
impl HoverSection {
    pub fn new(kind: HoverSectionKind, content: &str) -> Self {
        Self {
            kind,
            content: content.to_string(),
        }
    }
    pub fn to_markdown(&self) -> String {
        match &self.kind {
            HoverSectionKind::TypeSignature => format!("```lean\n{}\n```", self.content),
            HoverSectionKind::Documentation => self.content.clone(),
            HoverSectionKind::Examples => {
                format!("**Examples:**\n```lean\n{}\n```", self.content)
            }
            HoverSectionKind::SeeAlso => format!("**See also:** {}", self.content),
            HoverSectionKind::Source => format!("*Source: {}*", self.content),
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HoverSectionKind {
    TypeSignature,
    Documentation,
    Examples,
    SeeAlso,
    Source,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RichHoverContent {
    pub sections: Vec<HoverSection>,
}
#[allow(dead_code)]
impl RichHoverContent {
    pub fn new() -> Self {
        Self {
            sections: Vec::new(),
        }
    }
    pub fn add_section(&mut self, section: HoverSection) {
        self.sections.push(section);
    }
    pub fn add_type_sig(&mut self, sig: &str) {
        self.sections
            .push(HoverSection::new(HoverSectionKind::TypeSignature, sig));
    }
    pub fn add_doc(&mut self, doc: &str) {
        self.sections
            .push(HoverSection::new(HoverSectionKind::Documentation, doc));
    }
    pub fn to_markdown(&self) -> String {
        self.sections
            .iter()
            .map(|s| s.to_markdown())
            .collect::<Vec<_>>()
            .join("\n\n---\n\n")
    }
    pub fn is_empty(&self) -> bool {
        self.sections.is_empty()
    }
    pub fn section_count(&self) -> usize {
        self.sections.len()
    }
}
/// Information about function call signature.
#[derive(Clone, Debug)]
pub struct SignatureInfo {
    /// The function signature string.
    pub label: String,
    /// Optional documentation.
    pub documentation: Option<String>,
    /// Parameters of the function.
    pub parameters: Vec<ParameterInfo>,
    /// Index of the active parameter.
    pub active_parameter: Option<usize>,
}
/// Context for a find-references request.
#[derive(Clone, Debug)]
pub struct ReferenceContext {
    /// Whether to include the declaration itself.
    pub include_declaration: bool,
}
/// A completion item.
#[derive(Clone, Debug)]
pub struct CompletionItem {
    /// The text to insert.
    pub label: String,
    /// Additional detail.
    pub detail: Option<String>,
    /// Documentation.
    pub documentation: Option<String>,
    /// The kind of completion.
    pub kind: CompletionKind,
    /// The text to insert (may differ from label).
    pub insert_text: Option<String>,
}
/// Provides signature help for function calls.
pub struct SignatureHelpProvider<'a> {
    /// Reference to the kernel environment.
    env: &'a Environment,
}
impl<'a> SignatureHelpProvider<'a> {
    /// Create a new signature help provider.
    pub fn new(env: &'a Environment) -> Self {
        Self { env }
    }
    /// Get signature help at the given position.
    pub fn signature_help(&self, doc: &Document, pos: &Position) -> Option<SignatureInfo> {
        let (word, _) = doc.word_at_position(pos)?;
        let name = Name::str(&word);
        if let Some(ci) = self.env.find(&name) {
            let ty_str = format!("{:?}", ci.ty());
            Some(SignatureInfo {
                label: format!("{} : {}", word, ty_str),
                documentation: None,
                parameters: self.extract_parameters_from_str(&ty_str),
                active_parameter: None,
            })
        } else {
            None
        }
    }
    /// Extract parameter information from a type string.
    ///
    /// Splits on `->` at the top nesting level (respecting parentheses/brackets),
    /// treating all but the last segment as parameter types.
    fn extract_parameters_from_str(&self, ty_str: &str) -> Vec<ParameterInfo> {
        let parts = split_arrow_top(ty_str);
        if parts.len() <= 1 {
            return Vec::new();
        }
        parts[..parts.len() - 1]
            .iter()
            .enumerate()
            .map(|(i, ty_segment)| {
                let label = extract_param_label(i, ty_segment.trim());
                ParameterInfo {
                    label,
                    documentation: None,
                }
            })
            .collect()
    }
    /// Get the number of parameters for a function in the environment.
    pub fn param_count(&self, name: &str) -> Option<usize> {
        let n = Name::str(name);
        let ci = self.env.find(&n)?;
        let ty_str = format!("{:?}", ci.ty());
        Some(ty_str.matches("->").count())
    }
}
/// Provides hover information for tokens.
pub struct HoverProvider<'a> {
    /// Reference to the kernel environment.
    env: &'a Environment,
}
impl<'a> HoverProvider<'a> {
    /// Create a new hover provider.
    pub fn new(env: &'a Environment) -> Self {
        Self { env }
    }
    /// Get hover information at a position in a document.
    pub fn hover_at_position(&self, doc: &Document, pos: &Position) -> Option<HoverResult> {
        let (word, range) = doc.word_at_position(pos)?;
        if let Some(info) = self.hover_keyword(&word) {
            return Some(HoverResult {
                content: info,
                range,
            });
        }
        if let Some(info) = self.hover_declaration(&word) {
            return Some(HoverResult {
                content: info,
                range,
            });
        }
        if let Some(info) = self.hover_literal(&word) {
            return Some(HoverResult {
                content: info,
                range,
            });
        }
        if let Some(info) = self.hover_tactic(&word) {
            return Some(HoverResult {
                content: info,
                range,
            });
        }
        if let Some(info) = self.hover_local_definition(doc, &word) {
            return Some(HoverResult {
                content: info,
                range,
            });
        }
        None
    }
    /// Convert a hover result to an LSP Hover value.
    pub fn to_lsp_hover(&self, result: &HoverResult) -> Hover {
        Hover::new(
            MarkupContent::markdown(&result.content),
            Some(result.range.clone()),
        )
    }
    /// Get hover documentation for a keyword.
    pub fn hover_keyword(&self, word: &str) -> Option<String> {
        KEYWORD_DOCS.iter().find_map(|&(kw, doc)| {
            if kw == word {
                Some(doc.to_string())
            } else {
                None
            }
        })
    }
    /// Get hover info for a declaration from the environment.
    pub fn hover_declaration(&self, word: &str) -> Option<String> {
        let name = Name::str(word);
        let ci = self.env.find(&name)?;
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
            "definition"
        };
        Some(format_declaration_hover(kind, word, &ty_str))
    }
    /// Get hover info for a numeric or string literal.
    pub fn hover_literal(&self, word: &str) -> Option<String> {
        if word.chars().all(|c| c.is_ascii_digit()) {
            let val: u64 = word.parse().ok()?;
            return Some(format!(
                "```lean\n{} : Nat\n```\nNatural number literal (= {})",
                word, val
            ));
        }
        if word.starts_with('-') && word[1..].chars().all(|c| c.is_ascii_digit()) {
            let val: i64 = word.parse().ok()?;
            return Some(format!(
                "```lean\n{} : Int\n```\nInteger literal (= {})",
                word, val
            ));
        }
        None
    }
    /// Get hover documentation for a tactic name.
    pub fn hover_tactic(&self, word: &str) -> Option<String> {
        TACTIC_DOCS.iter().find_map(|&(name, doc)| {
            if name == word {
                Some(format!("**Tactic** `{}`\n\n{}", name, doc))
            } else {
                None
            }
        })
    }
    /// Get hover info from local definitions in the document.
    fn hover_local_definition(&self, doc: &Document, word: &str) -> Option<String> {
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
                let ty_info = def
                    .ty
                    .as_deref()
                    .map(|t| format!(" : {}", t))
                    .unwrap_or_default();
                return Some(format!("```lean\n{} {}{}\n```", kind_str, word, ty_info));
            }
        }
        None
    }
}
/// Information about a single parameter.
#[derive(Clone, Debug)]
pub struct ParameterInfo {
    /// Parameter label.
    pub label: String,
    /// Optional documentation.
    pub documentation: Option<String>,
}
/// Represents a symbol in the document outline.
#[derive(Clone, Debug)]
pub struct OutlineSymbol {
    /// Symbol name.
    pub name: String,
    /// Symbol kind.
    pub kind: SymbolKind,
    /// Location in the document.
    pub range: Range,
    /// Optional type annotation.
    pub ty: Option<String>,
    /// Child symbols (e.g., constructors of an inductive).
    pub children: Vec<OutlineSymbol>,
}
/// Provides go-to-definition functionality.
pub struct DefinitionProvider<'a> {
    /// Reference to the kernel environment.
    env: &'a Environment,
}
impl<'a> DefinitionProvider<'a> {
    /// Create a new definition provider.
    pub fn new(env: &'a Environment) -> Self {
        Self { env }
    }
    /// Find the definition location for a symbol at the given position.
    pub fn goto_definition(&self, doc: &Document, pos: &Position) -> Option<Location> {
        let (word, _) = doc.word_at_position(pos)?;
        if let Some(loc) = self.find_definition_in_source(doc, &word) {
            return Some(loc);
        }
        if let Some(loc) = self.find_definition_in_env(&word, &doc.uri) {
            return Some(loc);
        }
        None
    }
    /// Find a definition in the kernel environment.
    pub fn find_definition_in_env(&self, word: &str, uri: &str) -> Option<Location> {
        let name = Name::str(word);
        if self.env.find(&name).is_some() {
            Some(Location::new(
                uri,
                Range::single_line(0, 0, word.len() as u32),
            ))
        } else {
            None
        }
    }
    /// Find a definition in the current source document.
    pub fn find_definition_in_source(&self, doc: &Document, word: &str) -> Option<Location> {
        let analysis = analyze_document(&doc.uri, &doc.content, self.env);
        for def in &analysis.definitions {
            if def.name == word {
                return Some(Location::new(&doc.uri, def.range.clone()));
            }
        }
        None
    }
    /// Find a definition in imported modules (searches open document store).
    pub fn find_definition_in_imports(
        &self,
        store: &DocumentStore,
        word: &str,
        _current_uri: &str,
    ) -> Option<Location> {
        for uri in store.uris() {
            if let Some(doc) = store.get_document(uri) {
                let analysis = analyze_document(uri, &doc.content, self.env);
                for def in &analysis.definitions {
                    if def.name == word {
                        return Some(Location::new(uri.as_str(), def.range.clone()));
                    }
                }
            }
        }
        None
    }
}
/// Provides find-references functionality.
pub struct ReferenceProvider<'a> {
    /// Reference to the kernel environment.
    env: &'a Environment,
}
impl<'a> ReferenceProvider<'a> {
    /// Create a new reference provider.
    pub fn new(env: &'a Environment) -> Self {
        Self { env }
    }
    /// Find all references to the symbol at the given position.
    pub fn find_references(
        &self,
        doc: &Document,
        pos: &Position,
        context: &ReferenceContext,
    ) -> Vec<Location> {
        let (word, _) = match doc.word_at_position(pos) {
            Some(w) => w,
            None => return Vec::new(),
        };
        let mut locations = self.find_references_in_document(doc, &word);
        if !context.include_declaration {
            let analysis = analyze_document(&doc.uri, &doc.content, self.env);
            for def in &analysis.definitions {
                if def.name == word {
                    locations.retain(|loc| loc.range != def.range);
                }
            }
        }
        locations
    }
    /// Find all references to a name in a single document.
    pub fn find_references_in_document(&self, doc: &Document, name: &str) -> Vec<Location> {
        let mut locations = Vec::new();
        let mut lexer = Lexer::new(&doc.content);
        let tokens = lexer.tokenize();
        for token in &tokens {
            if let TokenKind::Ident(ident) = &token.kind {
                if ident == name {
                    let line = if token.span.line > 0 {
                        token.span.line as u32 - 1
                    } else {
                        0
                    };
                    let col = if token.span.column > 0 {
                        token.span.column as u32 - 1
                    } else {
                        0
                    };
                    let end_col = col + (token.span.end - token.span.start) as u32;
                    locations.push(Location::new(
                        &doc.uri,
                        Range::single_line(line, col, end_col),
                    ));
                }
            }
        }
        locations
    }
    /// Find all references across all open documents in the workspace.
    pub fn find_references_in_workspace(&self, store: &DocumentStore, name: &str) -> Vec<Location> {
        let mut all_locations = Vec::new();
        for uri in store.uris() {
            if let Some(doc) = store.get_document(uri) {
                all_locations.extend(self.find_references_in_document(doc, name));
            }
        }
        all_locations
    }
}
/// Result of a hover request.
#[derive(Clone, Debug)]
pub struct HoverResult {
    /// The hover content (markdown or plain text).
    pub content: String,
    /// The range of the hovered token.
    pub range: Range,
}
/// Provides code completion for the LSP.
pub struct CompletionProvider<'a> {
    /// Reference to the kernel environment.
    env: &'a Environment,
}
impl<'a> CompletionProvider<'a> {
    /// Create a new completion provider.
    pub fn new(env: &'a Environment) -> Self {
        Self { env }
    }
    /// Get completions at a position.
    pub fn completions(&self, doc: &Document, pos: &Position) -> Vec<CompletionItem> {
        let prefix = self.get_prefix_at(doc, pos);
        let mut items = Vec::new();
        for &(kw, _kw_doc) in KEYWORD_DOCS {
            if kw.starts_with(&prefix) {
                items.push(CompletionItem {
                    label: kw.to_string(),
                    detail: Some("keyword".to_string()),
                    documentation: None,
                    kind: CompletionKind::Keyword,
                    insert_text: None,
                });
            }
        }
        for &(name, _tac_doc) in TACTIC_DOCS {
            if name.starts_with(&prefix) {
                items.push(CompletionItem {
                    label: name.to_string(),
                    detail: Some("tactic".to_string()),
                    documentation: None,
                    kind: CompletionKind::Tactic,
                    insert_text: None,
                });
            }
        }
        for env_name in self.env.constant_names() {
            let name_str = format!("{}", env_name);
            if name_str.starts_with(&prefix) {
                items.push(CompletionItem {
                    label: name_str,
                    detail: Some("definition".to_string()),
                    documentation: None,
                    kind: CompletionKind::Function,
                    insert_text: None,
                });
            }
        }
        items
    }
    /// Get completions filtered by kind.
    pub fn completions_of_kind(
        &self,
        doc: &Document,
        pos: &Position,
        kind: CompletionKind,
    ) -> Vec<CompletionItem> {
        self.completions(doc, pos)
            .into_iter()
            .filter(|item| item.kind == kind)
            .collect()
    }
    fn get_prefix_at(&self, doc: &Document, pos: &Position) -> String {
        doc.word_at_position(pos)
            .map(|(w, _)| w)
            .unwrap_or_default()
    }
}
/// Cache for computed hover results to avoid redundant computation.
#[allow(dead_code)]
pub struct HoverCache {
    /// Cached results keyed by (uri, line, character).
    entries: std::collections::HashMap<(String, u32, u32), Option<HoverResult>>,
    /// Maximum number of entries before eviction.
    max_entries: usize,
    /// Current size.
    size: usize,
}
#[allow(dead_code)]
impl HoverCache {
    /// Create a new hover cache.
    pub fn new(max_entries: usize) -> Self {
        Self {
            entries: std::collections::HashMap::new(),
            max_entries,
            size: 0,
        }
    }
    /// Look up a cached hover result.
    pub fn get(&self, uri: &str, line: u32, character: u32) -> Option<&Option<HoverResult>> {
        self.entries.get(&(uri.to_string(), line, character))
    }
    /// Store a hover result in the cache.
    pub fn insert(&mut self, uri: &str, line: u32, character: u32, result: Option<HoverResult>) {
        if self.size >= self.max_entries {
            self.entries.clear();
            self.size = 0;
        }
        let key = (uri.to_string(), line, character);
        if self.entries.insert(key, result).is_none() {
            self.size += 1;
        }
    }
    /// Invalidate all entries for a document URI.
    pub fn invalidate(&mut self, uri: &str) {
        self.entries.retain(|(u, _, _), _| u != uri);
        self.size = self.entries.len();
    }
    /// Clear the entire cache.
    pub fn clear(&mut self) {
        self.entries.clear();
        self.size = 0;
    }
    /// Return the number of cached entries.
    pub fn len(&self) -> usize {
        self.size
    }
    /// Return true if the cache is empty.
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }
}
