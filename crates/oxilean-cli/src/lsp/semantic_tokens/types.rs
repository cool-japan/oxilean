//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use crate::lsp::{Document, JsonValue, Position, Range};
use oxilean_kernel::{Environment, Name};
use oxilean_parse::{Lexer, TokenKind};
use std::collections::HashMap;

/// Cached token data for a document.
#[derive(Clone, Debug)]
struct CachedTokenData {
    /// Document version.
    version: i64,
    /// The encoded tokens.
    tokens: EncodedSemanticTokens,
}
/// Provides inlay hints for a document.
pub struct InlayHintProvider<'a> {
    /// Reference to the environment.
    env: &'a Environment,
    /// Whether to show type hints.
    pub show_type_hints: bool,
    /// Whether to show parameter name hints.
    pub show_parameter_hints: bool,
    /// Maximum number of hints to return.
    pub max_hints: usize,
}
impl<'a> InlayHintProvider<'a> {
    /// Create a new provider.
    pub fn new(env: &'a Environment) -> Self {
        Self {
            env,
            show_type_hints: true,
            show_parameter_hints: true,
            max_hints: 200,
        }
    }
    /// Compute inlay hints for a document within a range.
    pub fn compute_hints(&self, doc: &Document, range: &Range) -> Vec<InlayHint> {
        let mut hints = Vec::new();
        if self.show_type_hints {
            hints.extend(self.compute_type_hints(doc, range));
        }
        if self.show_parameter_hints {
            hints.extend(self.compute_parameter_hints(doc, range));
        }
        if hints.len() > self.max_hints {
            hints.truncate(self.max_hints);
        }
        hints
    }
    /// Compute type annotation hints for definitions without explicit types.
    fn compute_type_hints(&self, doc: &Document, range: &Range) -> Vec<InlayHint> {
        let mut hints = Vec::new();
        let mut lexer = Lexer::new(&doc.content);
        let tokens = lexer.tokenize();
        let mut i = 0;
        while i < tokens.len() {
            let token = &tokens[i];
            let line = token.span.line.saturating_sub(1) as u32;
            if line < range.start.line || line > range.end.line {
                i += 1;
                continue;
            }
            if matches!(token.kind, TokenKind::Definition | TokenKind::Let) && i + 1 < tokens.len()
            {
                if let TokenKind::Ident(name) = &tokens[i + 1].kind {
                    let mut has_type = false;
                    let mut j = i + 2;
                    while j < tokens.len() {
                        match &tokens[j].kind {
                            TokenKind::Colon => {
                                has_type = true;
                                break;
                            }
                            TokenKind::Assign => break,
                            TokenKind::LParen | TokenKind::LBracket | TokenKind::LBrace => {
                                break;
                            }
                            _ => {}
                        }
                        j += 1;
                    }
                    if !has_type {
                        let kernel_name = Name::str(name);
                        if let Some(ci) = self.env.find(&kernel_name) {
                            let ty_str = format!("{:?}", ci.ty());
                            let name_tok = &tokens[i + 1];
                            let name_end = name_tok.span.column.saturating_sub(1) as u32
                                + (name_tok.span.end - name_tok.span.start) as u32;
                            hints
                                .push(InlayHint::type_hint(Position::new(line, name_end), &ty_str));
                        }
                    }
                }
            }
            i += 1;
        }
        hints
    }
    /// Compute parameter name hints for function calls.
    ///
    /// For each `ident(` pattern, looks up the function in the environment,
    /// extracts Pi binder names from its type, and emits a `param:` hint
    /// at the start of each argument position.
    fn compute_parameter_hints(&self, doc: &Document, range: &Range) -> Vec<InlayHint> {
        let mut hints = Vec::new();
        let mut lexer = Lexer::new(&doc.content);
        let tokens = lexer.tokenize();
        let mut i = 0;
        while i < tokens.len() {
            let token = &tokens[i];
            let line = token.span.line.saturating_sub(1) as u32;
            if line < range.start.line || line > range.end.line {
                i += 1;
                continue;
            }
            if let TokenKind::Ident(func_name) = &token.kind {
                if i + 1 < tokens.len() && tokens[i + 1].kind == TokenKind::LParen {
                    let kernel_name = Name::str(func_name);
                    if let Some(ci) = self.env.find(&kernel_name) {
                        let param_names = collect_pi_param_names(ci.ty());
                        let mut arg_tok = i + 2;
                        for param_name in &param_names {
                            if arg_tok >= tokens.len() {
                                break;
                            }
                            let arg_token = &tokens[arg_tok];
                            if arg_token.kind == TokenKind::RParen {
                                break;
                            }
                            let arg_line = arg_token.span.line.saturating_sub(1) as u32;
                            let arg_col = arg_token.span.column.saturating_sub(1) as u32;
                            hints.push(InlayHint::parameter_hint(
                                Position::new(arg_line, arg_col),
                                &param_name.to_string(),
                            ));
                            let mut depth: i32 = 0;
                            loop {
                                if arg_tok >= tokens.len() {
                                    break;
                                }
                                match tokens[arg_tok].kind {
                                    TokenKind::LParen | TokenKind::LBracket | TokenKind::LBrace => {
                                        depth += 1
                                    }
                                    TokenKind::RParen | TokenKind::RBracket | TokenKind::RBrace => {
                                        if depth == 0 {
                                            break;
                                        }
                                        depth -= 1;
                                    }
                                    TokenKind::Comma if depth == 0 => {
                                        arg_tok += 1;
                                        break;
                                    }
                                    _ => {}
                                }
                                arg_tok += 1;
                            }
                        }
                    }
                }
            }
            i += 1;
        }
        hints
    }
}
/// Manages incremental semantic token updates.
pub struct IncrementalTokenManager {
    /// Cached full token data per URI.
    cache: HashMap<String, CachedTokenData>,
    /// Next result ID counter.
    next_result_id: u64,
}
impl IncrementalTokenManager {
    /// Create a new manager.
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
            next_result_id: 0,
        }
    }
    /// Get the full tokens for a document, computing if necessary.
    pub fn get_full(&mut self, doc: &Document, env: &Environment) -> EncodedSemanticTokens {
        self.next_result_id += 1;
        let result_id = format!("result_{}", self.next_result_id);
        let classifier = TokenClassifier::new(env);
        let tokens = classifier.classify(doc);
        let encoded = EncodedSemanticTokens::from_tokens(&tokens, Some(result_id.clone()));
        self.cache.insert(
            doc.uri.clone(),
            CachedTokenData {
                version: doc.version,
                tokens: encoded.clone(),
            },
        );
        encoded
    }
    /// Get incremental token updates (delta) for a document.
    pub fn get_delta(
        &mut self,
        doc: &Document,
        env: &Environment,
        _previous_result_id: &str,
    ) -> SemanticTokensDelta {
        let new_classifier = TokenClassifier::new(env);
        let new_tokens = new_classifier.classify(doc);
        self.next_result_id += 1;
        let new_result_id = format!("result_{}", self.next_result_id);
        let new_encoded =
            EncodedSemanticTokens::from_tokens(&new_tokens, Some(new_result_id.clone()));
        let edits = if let Some(cached) = self.cache.get(&doc.uri) {
            compute_token_edits(&cached.tokens.data, &new_encoded.data)
        } else {
            vec![SemanticTokenEdit {
                start: 0,
                delete_count: 0,
                data: new_encoded.data.clone(),
            }]
        };
        self.cache.insert(
            doc.uri.clone(),
            CachedTokenData {
                version: doc.version,
                tokens: new_encoded,
            },
        );
        SemanticTokensDelta {
            result_id: new_result_id,
            edits,
        }
    }
    /// Invalidate cached tokens for a document.
    pub fn invalidate(&mut self, uri: &str) {
        self.cache.remove(uri);
    }
    /// Clear all cached tokens.
    pub fn clear(&mut self) {
        self.cache.clear();
    }
}
/// An inlay hint to display in the editor.
#[derive(Clone, Debug)]
pub struct InlayHint {
    /// Position where the hint should be displayed.
    pub position: Position,
    /// The hint label.
    pub label: String,
    /// Kind of hint.
    pub kind: InlayHintKind,
    /// Whether to add padding on the left.
    pub padding_left: bool,
    /// Whether to add padding on the right.
    pub padding_right: bool,
    /// Tooltip text.
    pub tooltip: Option<String>,
}
impl InlayHint {
    /// Create a type hint.
    pub fn type_hint(position: Position, ty: &str) -> Self {
        Self {
            position,
            label: format!(": {}", ty),
            kind: InlayHintKind::Type,
            padding_left: true,
            padding_right: false,
            tooltip: Some(format!("Inferred type: {}", ty)),
        }
    }
    /// Create a parameter name hint.
    pub fn parameter_hint(position: Position, name: &str) -> Self {
        Self {
            position,
            label: format!("{}:", name),
            kind: InlayHintKind::Parameter,
            padding_left: false,
            padding_right: true,
            tooltip: Some(format!("Parameter: {}", name)),
        }
    }
    /// Create a type annotation inlay hint from line/character coordinates.
    #[allow(dead_code)]
    pub fn type_annotation(line: u32, char_pos: u32, type_str: impl Into<String>) -> Self {
        let ty = type_str.into();
        Self {
            position: Position::new(line, char_pos),
            label: format!(": {}", ty),
            kind: InlayHintKind::Type,
            padding_left: true,
            padding_right: false,
            tooltip: Some(format!("Inferred type: {}", ty)),
        }
    }
    /// Create a parameter name inlay hint from line/character coordinates.
    #[allow(dead_code)]
    pub fn parameter_name(line: u32, char_pos: u32, name: impl Into<String>) -> Self {
        let n = name.into();
        Self {
            position: Position::new(line, char_pos),
            label: format!("{}:", n),
            kind: InlayHintKind::Parameter,
            padding_left: false,
            padding_right: true,
            tooltip: Some(format!("Parameter: {}", n)),
        }
    }
    /// Serialize to JSON.
    pub fn to_json(&self) -> JsonValue {
        let mut entries = vec![
            ("position".to_string(), self.position.to_json()),
            ("label".to_string(), JsonValue::String(self.label.clone())),
            (
                "kind".to_string(),
                JsonValue::Number(self.kind as i32 as f64),
            ),
            (
                "paddingLeft".to_string(),
                JsonValue::Bool(self.padding_left),
            ),
            (
                "paddingRight".to_string(),
                JsonValue::Bool(self.padding_right),
            ),
        ];
        if let Some(ref tooltip) = self.tooltip {
            entries.push(("tooltip".to_string(), JsonValue::String(tooltip.clone())));
        }
        JsonValue::Object(entries)
    }
}
/// Matched bracket pair.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct BracketPairPos {
    pub open_line: u32,
    pub open_char: u32,
    pub close_line: u32,
    pub close_char: u32,
}
/// Represents an edit to the semantic tokens.
#[derive(Clone, Debug)]
pub struct SemanticTokenEdit {
    /// Start offset in the data array.
    pub start: u32,
    /// Number of elements to delete.
    pub delete_count: u32,
    /// New data to insert.
    pub data: Vec<u32>,
}
impl SemanticTokenEdit {
    /// Serialize to JSON.
    pub fn to_json(&self) -> JsonValue {
        JsonValue::Object(vec![
            ("start".to_string(), JsonValue::Number(self.start as f64)),
            (
                "deleteCount".to_string(),
                JsonValue::Number(self.delete_count as f64),
            ),
            (
                "data".to_string(),
                JsonValue::Array(
                    self.data
                        .iter()
                        .map(|n| JsonValue::Number(*n as f64))
                        .collect(),
                ),
            ),
        ])
    }
}
/// A single semantic token with position and classification.
#[derive(Clone, Debug)]
pub struct SemanticToken {
    /// Line number (0-indexed).
    pub line: u32,
    /// Start character (0-indexed).
    pub start_char: u32,
    /// Length in characters.
    pub length: u32,
    /// Token type.
    pub token_type: OxiTokenType,
    /// Token modifier bitmask.
    pub modifiers: u32,
}
impl SemanticToken {
    /// Create a new semantic token.
    pub fn new(line: u32, start_char: u32, length: u32, token_type: OxiTokenType) -> Self {
        Self {
            line,
            start_char,
            length,
            token_type,
            modifiers: 0,
        }
    }
    /// Create with modifiers.
    pub fn with_modifiers(
        line: u32,
        start_char: u32,
        length: u32,
        token_type: OxiTokenType,
        modifiers: u32,
    ) -> Self {
        Self {
            line,
            start_char,
            length,
            token_type,
            modifiers,
        }
    }
    /// Add a modifier.
    pub fn add_modifier(&mut self, modifier: OxiTokenModifier) {
        self.modifiers |= modifier.bitmask();
    }
    /// Check if a modifier is set.
    pub fn has_modifier(&self, modifier: OxiTokenModifier) -> bool {
        self.modifiers & modifier.bitmask() != 0
    }
}
/// A highlight range in a document.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct TokenHighlightRange {
    pub start_line: u32,
    pub start_char: u32,
    pub end_line: u32,
    pub end_char: u32,
    pub highlight_kind: TokenHighlightKind,
}
impl TokenHighlightRange {
    /// Create a simple text highlight.
    #[allow(dead_code)]
    pub fn text(start_line: u32, start_char: u32, end_line: u32, end_char: u32) -> Self {
        Self {
            start_line,
            start_char,
            end_line,
            end_char,
            highlight_kind: TokenHighlightKind::Text,
        }
    }
}
/// Semantic token modifiers.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum OxiTokenModifier {
    /// The token is a declaration.
    Declaration,
    /// The token is a definition.
    Definition,
    /// The token is readonly.
    Readonly,
    /// The token is deprecated.
    Deprecated,
    /// The token is a documentation comment.
    Documentation,
    /// The token refers to a library item.
    DefaultLibrary,
}
impl OxiTokenModifier {
    /// Return the bitmask for this modifier.
    pub fn bitmask(self) -> u32 {
        1 << self.index()
    }
    /// Return the index in the legend.
    pub fn index(self) -> u32 {
        match self {
            Self::Declaration => 0,
            Self::Definition => 1,
            Self::Readonly => 2,
            Self::Deprecated => 3,
            Self::Documentation => 4,
            Self::DefaultLibrary => 5,
        }
    }
    /// Return the LSP modifier name.
    pub fn lsp_name(self) -> &'static str {
        match self {
            Self::Declaration => "declaration",
            Self::Definition => "definition",
            Self::Readonly => "readonly",
            Self::Deprecated => "deprecated",
            Self::Documentation => "documentation",
            Self::DefaultLibrary => "defaultLibrary",
        }
    }
    /// All modifiers for the legend.
    pub fn all() -> &'static [OxiTokenModifier] {
        &[
            Self::Declaration,
            Self::Definition,
            Self::Readonly,
            Self::Deprecated,
            Self::Documentation,
            Self::DefaultLibrary,
        ]
    }
}
/// Cached semantic tokens for a document version.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct SemanticTokenCache {
    entries: std::collections::HashMap<String, (String, Vec<u32>)>,
}
impl SemanticTokenCache {
    /// Create a new cache.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            entries: std::collections::HashMap::new(),
        }
    }
    /// Store tokens for a document version.
    #[allow(dead_code)]
    pub fn store(&mut self, uri: String, version: String, data: Vec<u32>) {
        self.entries.insert(uri, (version, data));
    }
    /// Retrieve tokens if the version matches.
    #[allow(dead_code)]
    pub fn get(&self, uri: &str, version: &str) -> Option<&Vec<u32>> {
        self.entries
            .get(uri)
            .filter(|(v, _)| v == version)
            .map(|(_, d)| d)
    }
    /// Invalidate cache for a URI.
    #[allow(dead_code)]
    pub fn invalidate(&mut self, uri: &str) {
        self.entries.remove(uri);
    }
}
/// The legend mapping token type/modifier indices to names.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct SemanticTokenLegend {
    pub token_types: Vec<String>,
    pub token_modifiers: Vec<String>,
}
impl SemanticTokenLegend {
    /// Create the default OxiLean legend.
    #[allow(dead_code)]
    pub fn default_oxilean() -> Self {
        Self {
            token_types: vec![
                "namespace".to_string(),
                "type".to_string(),
                "class".to_string(),
                "enumMember".to_string(),
                "typeParameter".to_string(),
                "function".to_string(),
                "method".to_string(),
                "property".to_string(),
                "variable".to_string(),
                "parameter".to_string(),
                "keyword".to_string(),
                "comment".to_string(),
                "string".to_string(),
                "number".to_string(),
                "operator".to_string(),
                "decorator".to_string(),
            ],
            token_modifiers: vec![
                "declaration".to_string(),
                "definition".to_string(),
                "readonly".to_string(),
                "static".to_string(),
                "deprecated".to_string(),
                "abstract".to_string(),
                "modification".to_string(),
                "documentation".to_string(),
            ],
        }
    }
    /// Look up the index for a token type name.
    #[allow(dead_code)]
    pub fn type_index(&self, name: &str) -> Option<usize> {
        self.token_types.iter().position(|t| t == name)
    }
    /// Look up the index for a modifier name.
    #[allow(dead_code)]
    pub fn modifier_index(&self, name: &str) -> Option<usize> {
        self.token_modifiers.iter().position(|m| m == name)
    }
    /// Compute the bitmask for a set of modifier names.
    #[allow(dead_code)]
    pub fn modifier_mask(&self, modifiers: &[&str]) -> u32 {
        let mut mask = 0u32;
        for name in modifiers {
            if let Some(idx) = self.modifier_index(name) {
                mask |= 1 << idx;
            }
        }
        mask
    }
}
/// Kind of an inlay hint.
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InlayHintKindExt {
    /// Type annotation
    Type,
    /// Parameter name
    Parameter,
    /// Other hint
    Other,
}
/// An extended inlay hint using line/char positions.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct InlayHintExt {
    pub position_line: u32,
    pub position_char: u32,
    pub label: String,
    pub kind: InlayHintKindExt,
    pub padding_left: bool,
    pub padding_right: bool,
    pub tooltip: Option<String>,
}
impl InlayHintExt {
    /// Create a type annotation inlay hint.
    #[allow(dead_code)]
    pub fn type_annotation(line: u32, char_pos: u32, type_str: impl Into<String>) -> Self {
        Self {
            position_line: line,
            position_char: char_pos,
            label: format!(": {}", type_str.into()),
            kind: InlayHintKindExt::Type,
            padding_left: true,
            padding_right: false,
            tooltip: None,
        }
    }
    /// Create a parameter name inlay hint.
    #[allow(dead_code)]
    pub fn parameter_name(line: u32, char_pos: u32, name: impl Into<String>) -> Self {
        Self {
            position_line: line,
            position_char: char_pos,
            label: format!("{}:", name.into()),
            kind: InlayHintKindExt::Parameter,
            padding_left: false,
            padding_right: true,
            tooltip: None,
        }
    }
}
/// Result of tokenizing a source file.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct TokenizeResult {
    pub uri: String,
    pub version: String,
    pub encoded: Vec<u32>,
    pub token_count: usize,
    pub duration_us: u64,
}
impl TokenizeResult {
    /// Decode back to raw tokens.
    #[allow(dead_code)]
    pub fn decode(&self) -> Vec<RawSemanticToken> {
        decode_semantic_tokens(&self.encoded)
    }
}
/// A single semantic token (before encoding).
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct RawSemanticToken {
    pub line: u32,
    pub start_char: u32,
    pub length: u32,
    pub token_type: u32,
    pub token_modifiers: u32,
}
/// Fluent builder for constructing semantic token lists.
#[allow(dead_code)]
pub struct SemanticTokenBuilder {
    legend: SemanticTokenLegend,
    accumulator: SemanticTokenAccumulator,
}
impl SemanticTokenBuilder {
    /// Create a new builder with the default OxiLean legend.
    #[allow(dead_code)]
    pub fn new() -> Self {
        let legend = SemanticTokenLegend::default_oxilean();
        let accumulator = SemanticTokenAccumulator::new(legend.clone());
        Self {
            legend,
            accumulator,
        }
    }
    /// Add a keyword token.
    #[allow(dead_code)]
    pub fn keyword(mut self, line: u32, start: u32, len: u32) -> Self {
        self.accumulator.add(line, start, len, "keyword", &[]);
        self
    }
    /// Add a type token.
    #[allow(dead_code)]
    pub fn type_token(mut self, line: u32, start: u32, len: u32) -> Self {
        self.accumulator.add(line, start, len, "type", &[]);
        self
    }
    /// Add a function token.
    #[allow(dead_code)]
    pub fn function(mut self, line: u32, start: u32, len: u32) -> Self {
        self.accumulator.add(line, start, len, "function", &[]);
        self
    }
    /// Add a variable token.
    #[allow(dead_code)]
    pub fn variable(mut self, line: u32, start: u32, len: u32) -> Self {
        self.accumulator.add(line, start, len, "variable", &[]);
        self
    }
    /// Add a comment token.
    #[allow(dead_code)]
    pub fn comment(mut self, line: u32, start: u32, len: u32) -> Self {
        self.accumulator.add(line, start, len, "comment", &[]);
        self
    }
    /// Add a number token.
    #[allow(dead_code)]
    pub fn number(mut self, line: u32, start: u32, len: u32) -> Self {
        self.accumulator.add(line, start, len, "number", &[]);
        self
    }
    /// Build and encode the tokens.
    #[allow(dead_code)]
    pub fn build(mut self) -> Vec<u32> {
        self.accumulator.sort();
        self.accumulator.encode()
    }
    /// Build and return the raw tokens.
    #[allow(dead_code)]
    pub fn build_raw(mut self) -> Vec<RawSemanticToken> {
        self.accumulator.sort();
        self.accumulator.tokens().to_vec()
    }
}
/// The kind of token highlight.
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TokenHighlightKind {
    Text,
    Read,
    Write,
}
/// Classifies tokens in a document for semantic highlighting.
pub struct TokenClassifier<'a> {
    /// Reference to the environment.
    env: &'a Environment,
}
impl<'a> TokenClassifier<'a> {
    /// Create a new token classifier.
    pub fn new(env: &'a Environment) -> Self {
        Self { env }
    }
    /// Classify all tokens in a document.
    pub fn classify(&self, doc: &Document) -> Vec<SemanticToken> {
        let mut tokens = Vec::new();
        let mut lexer = Lexer::new(&doc.content);
        let lex_tokens = lexer.tokenize();
        let mut ctx = ClassificationContext::new();
        for (i, lex_token) in lex_tokens.iter().enumerate() {
            let line = lex_token.span.line.saturating_sub(1) as u32;
            let col = lex_token.span.column.saturating_sub(1) as u32;
            let length = (lex_token.span.end - lex_token.span.start) as u32;
            if length == 0 {
                continue;
            }
            let (token_type, modifiers) =
                self.classify_token(&lex_token.kind, &mut ctx, i, &lex_tokens);
            tokens.push(SemanticToken {
                line,
                start_char: col,
                length,
                token_type,
                modifiers,
            });
            self.update_context(&lex_token.kind, &mut ctx, i, &lex_tokens);
        }
        tokens
    }
    /// Classify a single token.
    fn classify_token(
        &self,
        kind: &TokenKind,
        ctx: &mut ClassificationContext,
        _index: usize,
        _tokens: &[oxilean_parse::tokens::Token],
    ) -> (OxiTokenType, u32) {
        match kind {
            TokenKind::Definition
            | TokenKind::Theorem
            | TokenKind::Lemma
            | TokenKind::Axiom
            | TokenKind::Where
            | TokenKind::Let
            | TokenKind::In
            | TokenKind::Fun
            | TokenKind::Forall
            | TokenKind::Match
            | TokenKind::With
            | TokenKind::If
            | TokenKind::Then
            | TokenKind::Else
            | TokenKind::Do
            | TokenKind::By
            | TokenKind::Import
            | TokenKind::Open
            | TokenKind::End
            | TokenKind::Variable
            | TokenKind::Have
            | TokenKind::Show
            | TokenKind::Return => (OxiTokenType::Keyword, 0),
            TokenKind::Inductive | TokenKind::Structure => (OxiTokenType::Keyword, 0),
            TokenKind::Class => (OxiTokenType::Keyword, 0),
            TokenKind::Instance => (OxiTokenType::Keyword, 0),
            TokenKind::Namespace => (OxiTokenType::Keyword, 0),
            TokenKind::Section => (OxiTokenType::Keyword, 0),
            TokenKind::Ident(name) => {
                if ctx.expect_decl_name {
                    let decl_mod = OxiTokenModifier::Declaration.bitmask()
                        | OxiTokenModifier::Definition.bitmask();
                    (OxiTokenType::Function, decl_mod)
                } else if ctx.in_tactic {
                    if is_tactic_name(name) {
                        (OxiTokenType::Method, 0)
                    } else {
                        (OxiTokenType::Variable, 0)
                    }
                } else {
                    self.classify_identifier(name, ctx)
                }
            }
            TokenKind::Nat(_) => (OxiTokenType::Number, 0),
            TokenKind::String(_) => (OxiTokenType::String, 0),
            TokenKind::DocComment(text) => {
                let mods = if text.starts_with('-') {
                    OxiTokenModifier::Documentation.bitmask()
                } else {
                    0
                };
                (OxiTokenType::Comment, mods)
            }
            TokenKind::Plus
            | TokenKind::Minus
            | TokenKind::Star
            | TokenKind::Slash
            | TokenKind::Arrow
            | TokenKind::FatArrow
            | TokenKind::Eq
            | TokenKind::Ne
            | TokenKind::Lt
            | TokenKind::Le
            | TokenKind::Gt
            | TokenKind::Ge
            | TokenKind::And
            | TokenKind::Or
            | TokenKind::Not
            | TokenKind::Bar
            | TokenKind::AndAnd
            | TokenKind::LeftArrow => (OxiTokenType::Operator, 0),
            TokenKind::Colon | TokenKind::Assign => (OxiTokenType::Operator, 0),
            TokenKind::At => (OxiTokenType::Decorator, 0),
            TokenKind::Hash => (OxiTokenType::Macro, 0),
            _ => (OxiTokenType::Variable, 0),
        }
    }
    /// Classify an identifier based on context and environment.
    fn classify_identifier(&self, name: &str, ctx: &ClassificationContext) -> (OxiTokenType, u32) {
        if let Some(ty) = ctx.local_defs.get(name) {
            return (*ty, 0);
        }
        let kernel_name = Name::str(name);
        if self.env.is_inductive(&kernel_name) {
            return (OxiTokenType::Type, 0);
        }
        if self.env.is_constructor(&kernel_name) {
            return (OxiTokenType::EnumMember, 0);
        }
        if self.env.find(&kernel_name).is_some() {
            let mods = OxiTokenModifier::DefaultLibrary.bitmask();
            return (OxiTokenType::Function, mods);
        }
        if ctx.in_type || name.starts_with(|c: char| c.is_uppercase()) {
            (OxiTokenType::Type, 0)
        } else {
            (OxiTokenType::Variable, 0)
        }
    }
    /// Update the classification context after processing a token.
    fn update_context(
        &self,
        kind: &TokenKind,
        ctx: &mut ClassificationContext,
        _index: usize,
        _tokens: &[oxilean_parse::tokens::Token],
    ) {
        match kind {
            TokenKind::Definition
            | TokenKind::Theorem
            | TokenKind::Lemma
            | TokenKind::Axiom
            | TokenKind::Inductive
            | TokenKind::Structure
            | TokenKind::Class
            | TokenKind::Instance => {
                ctx.expect_decl_name = true;
                ctx.in_tactic = false;
            }
            TokenKind::Ident(name) => {
                if ctx.expect_decl_name {
                    ctx.local_defs.insert(name.clone(), OxiTokenType::Function);
                    ctx.expect_decl_name = false;
                }
            }
            TokenKind::By => {
                ctx.in_tactic = true;
            }
            TokenKind::Colon => {
                if !ctx.in_tactic {
                    ctx.in_type = true;
                }
            }
            TokenKind::Assign | TokenKind::Where => {
                ctx.in_type = false;
            }
            TokenKind::Namespace | TokenKind::Section | TokenKind::End => {
                ctx.in_tactic = false;
                ctx.in_type = false;
                ctx.expect_decl_name = false;
            }
            _ => {}
        }
    }
}
/// Kind of inlay hint.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InlayHintKind {
    /// A type annotation hint.
    Type = 1,
    /// A parameter name hint.
    Parameter = 2,
}
/// Accumulates semantic tokens during document analysis.
#[allow(dead_code)]
pub struct SemanticTokenAccumulator {
    tokens: Vec<RawSemanticToken>,
    legend: SemanticTokenLegend,
}
impl SemanticTokenAccumulator {
    /// Create a new accumulator.
    #[allow(dead_code)]
    pub fn new(legend: SemanticTokenLegend) -> Self {
        Self {
            tokens: vec![],
            legend,
        }
    }
    /// Add a token by type name and modifier names.
    #[allow(dead_code)]
    pub fn add(
        &mut self,
        line: u32,
        start_char: u32,
        length: u32,
        token_type: &str,
        modifiers: &[&str],
    ) {
        let type_idx = self.legend.type_index(token_type).unwrap_or(0) as u32;
        let mod_mask = self.legend.modifier_mask(modifiers);
        self.tokens.push(RawSemanticToken {
            line,
            start_char,
            length,
            token_type: type_idx,
            token_modifiers: mod_mask,
        });
    }
    /// Sort tokens by position (line, then start_char).
    #[allow(dead_code)]
    pub fn sort(&mut self) {
        self.tokens
            .sort_by(|a, b| a.line.cmp(&b.line).then(a.start_char.cmp(&b.start_char)));
    }
    /// Encode to LSP format.
    #[allow(dead_code)]
    pub fn encode(&self) -> Vec<u32> {
        encode_semantic_tokens(&self.tokens)
    }
    /// Return the collected tokens.
    #[allow(dead_code)]
    pub fn tokens(&self) -> &[RawSemanticToken] {
        &self.tokens
    }
}
/// Context for token classification.
struct ClassificationContext {
    /// Whether we are inside a tactic block.
    in_tactic: bool,
    /// Whether the next identifier is a declaration name.
    expect_decl_name: bool,
    /// Whether we are inside a type annotation.
    in_type: bool,
    /// Set of names known to be defined in this file.
    local_defs: HashMap<String, OxiTokenType>,
    /// Whether we are inside a comment block.
    in_comment: bool,
}
impl ClassificationContext {
    fn new() -> Self {
        Self {
            in_tactic: false,
            expect_decl_name: false,
            in_type: false,
            local_defs: HashMap::new(),
            in_comment: false,
        }
    }
}
/// A single edit in a semantic token delta.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct SemanticTokenEditV2 {
    pub start: u32,
    pub delete_count: u32,
    pub data: Vec<u32>,
}
/// Filters tokens by a set of allowed types.
#[allow(dead_code)]
pub struct SemanticTokenFilter {
    pub allowed_types: Vec<u32>,
}
impl SemanticTokenFilter {
    /// Create a new filter allowing all types.
    #[allow(dead_code)]
    pub fn allow_all() -> Self {
        Self {
            allowed_types: vec![],
        }
    }
    /// Create a filter allowing specific type indices.
    #[allow(dead_code)]
    pub fn allow_types(types: Vec<u32>) -> Self {
        Self {
            allowed_types: types,
        }
    }
    /// Apply the filter to a list of raw tokens.
    #[allow(dead_code)]
    pub fn apply<'t>(&self, tokens: &'t [RawSemanticToken]) -> Vec<&'t RawSemanticToken> {
        if self.allowed_types.is_empty() {
            return tokens.iter().collect();
        }
        tokens
            .iter()
            .filter(|t| self.allowed_types.contains(&t.token_type))
            .collect()
    }
}
/// Maps OxiTokenType to ANSI color codes for terminal output.
#[allow(dead_code)]
pub struct TokenColorizer {
    color_map: std::collections::HashMap<String, String>,
}
impl TokenColorizer {
    /// Create with default OxiLean color mapping.
    #[allow(dead_code)]
    pub fn new() -> Self {
        let mut map = std::collections::HashMap::new();
        map.insert("keyword".to_string(), "\x1b[34m".to_string());
        map.insert("type".to_string(), "\x1b[32m".to_string());
        map.insert("function".to_string(), "\x1b[33m".to_string());
        map.insert("variable".to_string(), "\x1b[37m".to_string());
        map.insert("comment".to_string(), "\x1b[90m".to_string());
        map.insert("string".to_string(), "\x1b[31m".to_string());
        map.insert("number".to_string(), "\x1b[35m".to_string());
        map.insert("operator".to_string(), "\x1b[36m".to_string());
        Self { color_map: map }
    }
    /// Get the ANSI color for a token type name.
    #[allow(dead_code)]
    pub fn color_for(&self, token_type: &str) -> Option<&str> {
        self.color_map.get(token_type).map(|s| s.as_str())
    }
    /// Colorize text with a given token type.
    #[allow(dead_code)]
    pub fn colorize(&self, text: &str, token_type: &str) -> String {
        if let Some(color) = self.color_for(token_type) {
            format!("{}{}\x1b[0m", color, text)
        } else {
            text.to_string()
        }
    }
}
/// Statistics about semantic tokens in a document.
#[allow(dead_code)]
#[derive(Clone, Debug, Default)]
pub struct SemanticTokenStatistics {
    pub total_tokens: usize,
    pub tokens_by_type: std::collections::HashMap<u32, usize>,
    pub lines_with_tokens: usize,
    pub avg_tokens_per_line: f64,
}
impl SemanticTokenStatistics {
    /// Compute statistics from a list of tokens.
    #[allow(dead_code)]
    pub fn compute(tokens: &[RawSemanticToken]) -> Self {
        let total_tokens = tokens.len();
        let mut tokens_by_type = std::collections::HashMap::new();
        let mut lines = std::collections::HashSet::new();
        for t in tokens {
            *tokens_by_type.entry(t.token_type).or_insert(0) += 1;
            lines.insert(t.line);
        }
        let lines_with_tokens = lines.len();
        let avg_tokens_per_line = if lines_with_tokens > 0 {
            total_tokens as f64 / lines_with_tokens as f64
        } else {
            0.0
        };
        Self {
            total_tokens,
            tokens_by_type,
            lines_with_tokens,
            avg_tokens_per_line,
        }
    }
    /// Return the most frequent token type.
    #[allow(dead_code)]
    pub fn most_frequent_type(&self) -> Option<u32> {
        self.tokens_by_type
            .iter()
            .max_by_key(|(_, count)| *count)
            .map(|(t, _)| *t)
    }
}
/// Encoded semantic tokens data (as per LSP spec).
#[derive(Clone, Debug)]
pub struct EncodedSemanticTokens {
    /// The encoded data (groups of 5 integers).
    pub data: Vec<u32>,
    /// Result ID for incremental updates.
    pub result_id: Option<String>,
}
impl EncodedSemanticTokens {
    /// Create from a list of semantic tokens.
    pub fn from_tokens(tokens: &[SemanticToken], result_id: Option<String>) -> Self {
        let mut data = Vec::with_capacity(tokens.len() * 5);
        let mut prev_line: u32 = 0;
        let mut prev_char: u32 = 0;
        for token in tokens {
            let delta_line = token.line - prev_line;
            let delta_start = if delta_line == 0 {
                token.start_char.saturating_sub(prev_char)
            } else {
                token.start_char
            };
            data.push(delta_line);
            data.push(delta_start);
            data.push(token.length);
            data.push(token.token_type.index());
            data.push(token.modifiers);
            prev_line = token.line;
            prev_char = token.start_char;
        }
        Self { data, result_id }
    }
    /// Serialize to JSON.
    pub fn to_json(&self) -> JsonValue {
        let mut entries = vec![(
            "data".to_string(),
            JsonValue::Array(
                self.data
                    .iter()
                    .map(|n| JsonValue::Number(*n as f64))
                    .collect(),
            ),
        )];
        if let Some(ref id) = self.result_id {
            entries.push(("resultId".to_string(), JsonValue::String(id.clone())));
        }
        JsonValue::Object(entries)
    }
}
/// A delta update for semantic tokens.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct SemanticTokenDelta {
    pub result_id: Option<String>,
    pub edits: Vec<SemanticTokenEdit>,
}
/// Kind of bracket.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BracketKind {
    /// Round parentheses `()`.
    Paren,
    /// Square brackets `[]`.
    Bracket,
    /// Curly braces `{}`.
    Brace,
    /// Angle brackets `<>`.
    Angle,
}
impl BracketKind {
    /// Get the opening character.
    pub fn open_char(self) -> char {
        match self {
            Self::Paren => '(',
            Self::Bracket => '[',
            Self::Brace => '{',
            Self::Angle => '<',
        }
    }
    /// Get the closing character.
    pub fn close_char(self) -> char {
        match self {
            Self::Paren => ')',
            Self::Bracket => ']',
            Self::Brace => '}',
            Self::Angle => '>',
        }
    }
}
/// Provides inlay hints for a document.
#[allow(dead_code)]
pub struct InlayHintProviderSimple {
    pub show_type_annotations: bool,
    pub show_parameter_names: bool,
    pub max_hints: usize,
}
impl InlayHintProviderSimple {
    /// Create a new provider with defaults.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            show_type_annotations: true,
            show_parameter_names: true,
            max_hints: 100,
        }
    }
    /// Generate inlay hints for a line of text.
    #[allow(dead_code)]
    pub fn hints_for_line(&self, line: u32, _text: &str) -> Vec<InlayHint> {
        let _ = line;
        vec![]
    }
}
/// Semantic token types for OxiLean.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum OxiTokenType {
    /// A namespace identifier.
    Namespace,
    /// A type (inductive, structure, etc.).
    Type,
    /// A class (type class).
    Class,
    /// An enum member (constructor).
    EnumMember,
    /// A type parameter.
    TypeParameter,
    /// A function definition.
    Function,
    /// A method (theorem/lemma).
    Method,
    /// A property (instance field).
    Property,
    /// A variable (local binding).
    Variable,
    /// A parameter.
    Parameter,
    /// A string literal.
    String,
    /// A number literal.
    Number,
    /// A keyword.
    Keyword,
    /// A comment.
    Comment,
    /// An operator.
    Operator,
    /// A macro.
    Macro,
    /// A decorator (attribute).
    Decorator,
}
impl OxiTokenType {
    /// Return the index in the legend.
    pub fn index(self) -> u32 {
        match self {
            Self::Namespace => 0,
            Self::Type => 1,
            Self::Class => 2,
            Self::EnumMember => 3,
            Self::TypeParameter => 4,
            Self::Function => 5,
            Self::Method => 6,
            Self::Property => 7,
            Self::Variable => 8,
            Self::Parameter => 9,
            Self::String => 10,
            Self::Number => 11,
            Self::Keyword => 12,
            Self::Comment => 13,
            Self::Operator => 14,
            Self::Macro => 15,
            Self::Decorator => 16,
        }
    }
    /// Return the LSP token type name.
    pub fn lsp_name(self) -> &'static str {
        match self {
            Self::Namespace => "namespace",
            Self::Type => "type",
            Self::Class => "class",
            Self::EnumMember => "enumMember",
            Self::TypeParameter => "typeParameter",
            Self::Function => "function",
            Self::Method => "method",
            Self::Property => "property",
            Self::Variable => "variable",
            Self::Parameter => "parameter",
            Self::String => "string",
            Self::Number => "number",
            Self::Keyword => "keyword",
            Self::Comment => "comment",
            Self::Operator => "operator",
            Self::Macro => "macro",
            Self::Decorator => "decorator",
        }
    }
    /// All token types for the legend.
    pub fn all() -> &'static [OxiTokenType] {
        &[
            Self::Namespace,
            Self::Type,
            Self::Class,
            Self::EnumMember,
            Self::TypeParameter,
            Self::Function,
            Self::Method,
            Self::Property,
            Self::Variable,
            Self::Parameter,
            Self::String,
            Self::Number,
            Self::Keyword,
            Self::Comment,
            Self::Operator,
            Self::Macro,
            Self::Decorator,
        ]
    }
}
/// A bracket pair.
#[derive(Clone, Debug)]
pub struct BracketPair {
    /// Opening bracket position.
    pub open: Position,
    /// Closing bracket position.
    pub close: Position,
    /// Nesting depth.
    pub depth: usize,
    /// Bracket kind.
    pub kind: BracketKind,
}
/// Semantic tokens delta response.
#[derive(Clone, Debug)]
pub struct SemanticTokensDelta {
    /// New result ID.
    pub result_id: String,
    /// List of edits to apply.
    pub edits: Vec<SemanticTokenEdit>,
}
impl SemanticTokensDelta {
    /// Serialize to JSON.
    pub fn to_json(&self) -> JsonValue {
        JsonValue::Object(vec![
            (
                "resultId".to_string(),
                JsonValue::String(self.result_id.clone()),
            ),
            (
                "edits".to_string(),
                JsonValue::Array(self.edits.iter().map(|e| e.to_json()).collect()),
            ),
        ])
    }
}
