//! Core LSP protocol types: Position, Range, Diagnostic, CompletionItem, etc.

use super::json_rpc::JsonValue;

// ── Position / Range / Location ───────────────────────────────────────────────

/// A position in a text document (0-indexed line and character).
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct Position {
    /// Line number (0-indexed).
    pub line: u32,
    /// Character offset within the line (0-indexed, UTF-16 code units).
    pub character: u32,
}

impl Position {
    /// Create a new position.
    pub fn new(line: u32, character: u32) -> Self {
        Self { line, character }
    }

    /// Serialize to JSON.
    pub fn to_json(&self) -> JsonValue {
        JsonValue::Object(vec![
            ("line".to_string(), JsonValue::Number(self.line as f64)),
            (
                "character".to_string(),
                JsonValue::Number(self.character as f64),
            ),
        ])
    }

    /// Parse from JSON.
    pub fn from_json(val: &JsonValue) -> Result<Self, String> {
        let line = val
            .get("line")
            .and_then(|v| v.as_u32())
            .ok_or("missing line")?;
        let character = val
            .get("character")
            .and_then(|v| v.as_u32())
            .ok_or("missing character")?;
        Ok(Self { line, character })
    }
}

/// A range in a text document.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct Range {
    /// Start position (inclusive).
    pub start: Position,
    /// End position (exclusive).
    pub end: Position,
}

impl Range {
    /// Create a new range.
    pub fn new(start: Position, end: Position) -> Self {
        Self { start, end }
    }

    /// Create a range covering a single line.
    pub fn single_line(line: u32, start_char: u32, end_char: u32) -> Self {
        Self {
            start: Position::new(line, start_char),
            end: Position::new(line, end_char),
        }
    }

    /// Serialize to JSON.
    pub fn to_json(&self) -> JsonValue {
        JsonValue::Object(vec![
            ("start".to_string(), self.start.to_json()),
            ("end".to_string(), self.end.to_json()),
        ])
    }

    /// Parse from JSON.
    pub fn from_json(val: &JsonValue) -> Result<Self, String> {
        let start_val = val
            .get("start")
            .ok_or_else(|| "missing start".to_string())?;
        let start = Position::from_json(start_val)?;
        let end_val = val.get("end").ok_or_else(|| "missing end".to_string())?;
        let end = Position::from_json(end_val)?;
        Ok(Self { start, end })
    }
}

/// A location (URI + range).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Location {
    /// Document URI.
    pub uri: String,
    /// Range in the document.
    pub range: Range,
}

impl Location {
    /// Create a new location.
    pub fn new(uri: impl Into<String>, range: Range) -> Self {
        Self {
            uri: uri.into(),
            range,
        }
    }

    /// Serialize to JSON.
    pub fn to_json(&self) -> JsonValue {
        JsonValue::Object(vec![
            ("uri".to_string(), JsonValue::String(self.uri.clone())),
            ("range".to_string(), self.range.to_json()),
        ])
    }
}

// ── Diagnostic ────────────────────────────────────────────────────────────────

/// Diagnostic severity levels.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum DiagnosticSeverity {
    /// Error.
    Error = 1,
    /// Warning.
    Warning = 2,
    /// Information.
    Information = 3,
    /// Hint.
    Hint = 4,
}

impl DiagnosticSeverity {
    /// Convert to LSP integer.
    pub fn to_number(self) -> f64 {
        self as i32 as f64
    }
}

/// A diagnostic message (error, warning, etc.).
#[derive(Clone, Debug)]
pub struct Diagnostic {
    /// Range in the source.
    pub range: Range,
    /// Severity level.
    pub severity: DiagnosticSeverity,
    /// Human-readable message.
    pub message: String,
    /// Source identifier (e.g. "oxilean").
    pub source: Option<String>,
    /// Machine-readable code.
    pub code: Option<String>,
}

impl Diagnostic {
    /// Create a new diagnostic.
    pub fn new(range: Range, severity: DiagnosticSeverity, message: impl Into<String>) -> Self {
        Self {
            range,
            severity,
            message: message.into(),
            source: Some("oxilean".to_string()),
            code: None,
        }
    }

    /// Create an error diagnostic.
    pub fn error(range: Range, message: impl Into<String>) -> Self {
        Self::new(range, DiagnosticSeverity::Error, message)
    }

    /// Create a warning diagnostic.
    pub fn warning(range: Range, message: impl Into<String>) -> Self {
        Self::new(range, DiagnosticSeverity::Warning, message)
    }

    /// Serialize to JSON.
    pub fn to_json(&self) -> JsonValue {
        let mut entries = vec![
            ("range".to_string(), self.range.to_json()),
            (
                "severity".to_string(),
                JsonValue::Number(self.severity.to_number()),
            ),
            (
                "message".to_string(),
                JsonValue::String(self.message.clone()),
            ),
        ];
        if let Some(ref source) = self.source {
            entries.push(("source".to_string(), JsonValue::String(source.clone())));
        }
        if let Some(ref code) = self.code {
            entries.push(("code".to_string(), JsonValue::String(code.clone())));
        }
        JsonValue::Object(entries)
    }
}

// ── Text document types ───────────────────────────────────────────────────────

/// Text document identifier (just a URI).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TextDocumentIdentifier {
    /// URI of the document.
    pub uri: String,
}

impl TextDocumentIdentifier {
    /// Create a new identifier.
    pub fn new(uri: impl Into<String>) -> Self {
        Self { uri: uri.into() }
    }

    /// Parse from JSON.
    pub fn from_json(val: &JsonValue) -> Result<Self, String> {
        let uri = val
            .get("uri")
            .and_then(|v| v.as_str())
            .ok_or("missing uri")?
            .to_string();
        Ok(Self { uri })
    }
}

/// A full text document item (used on open).
#[derive(Clone, Debug)]
pub struct TextDocumentItem {
    /// URI of the document.
    pub uri: String,
    /// Language identifier (e.g. "lean4").
    pub language_id: String,
    /// Version number.
    pub version: i64,
    /// Full document text.
    pub text: String,
}

impl TextDocumentItem {
    /// Parse from JSON.
    pub fn from_json(val: &JsonValue) -> Result<Self, String> {
        let uri = val
            .get("uri")
            .and_then(|v| v.as_str())
            .ok_or("missing uri")?
            .to_string();
        let language_id = val
            .get("languageId")
            .and_then(|v| v.as_str())
            .unwrap_or("lean4")
            .to_string();
        let version = val.get("version").and_then(|v| v.as_i64()).unwrap_or(0);
        let text = val
            .get("text")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        Ok(Self {
            uri,
            language_id,
            version,
            text,
        })
    }
}

/// A text edit.
#[derive(Clone, Debug)]
pub struct TextEdit {
    /// Range to replace.
    pub range: Range,
    /// New text to insert.
    pub new_text: String,
}

impl TextEdit {
    /// Create a new text edit.
    pub fn new(range: Range, new_text: impl Into<String>) -> Self {
        Self {
            range,
            new_text: new_text.into(),
        }
    }

    /// Serialize to JSON.
    pub fn to_json(&self) -> JsonValue {
        JsonValue::Object(vec![
            ("range".to_string(), self.range.to_json()),
            (
                "newText".to_string(),
                JsonValue::String(self.new_text.clone()),
            ),
        ])
    }
}

// ── Completion types ──────────────────────────────────────────────────────────

/// Completion item kind (LSP spec values).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CompletionItemKind {
    /// A text completion.
    Text = 1,
    /// A method completion.
    Method = 2,
    /// A function completion.
    Function = 3,
    /// A constructor.
    Constructor = 4,
    /// A field.
    Field = 5,
    /// A variable.
    Variable = 6,
    /// A class.
    Class = 7,
    /// An interface.
    Interface = 8,
    /// A module.
    Module = 9,
    /// A property.
    Property = 10,
    /// A keyword.
    Keyword = 14,
    /// A snippet.
    Snippet = 15,
    /// A constant.
    Constant = 21,
}

/// A completion item.
#[derive(Clone, Debug)]
pub struct CompletionItem {
    /// Label shown in the completion menu.
    pub label: String,
    /// Kind of completion.
    pub kind: CompletionItemKind,
    /// Short detail string.
    pub detail: Option<String>,
    /// Documentation.
    pub documentation: Option<MarkupContent>,
    /// Text to insert (if different from label).
    pub insert_text: Option<String>,
    /// Sort text for ordering.
    pub sort_text: Option<String>,
}

impl CompletionItem {
    /// Create a keyword completion.
    pub fn keyword(label: &str) -> Self {
        Self {
            label: label.to_string(),
            kind: CompletionItemKind::Keyword,
            detail: Some("keyword".to_string()),
            documentation: None,
            insert_text: None,
            sort_text: Some(format!("0_{}", label)),
        }
    }

    /// Create a function completion.
    pub fn function(label: &str, detail: &str) -> Self {
        Self {
            label: label.to_string(),
            kind: CompletionItemKind::Function,
            detail: Some(detail.to_string()),
            documentation: None,
            insert_text: None,
            sort_text: Some(format!("1_{}", label)),
        }
    }

    /// Create a variable completion.
    pub fn variable(label: &str, ty: &str) -> Self {
        Self {
            label: label.to_string(),
            kind: CompletionItemKind::Variable,
            detail: Some(ty.to_string()),
            documentation: None,
            insert_text: None,
            sort_text: Some(format!("2_{}", label)),
        }
    }

    /// Create a snippet completion.
    pub fn snippet(label: &str, insert_text: &str, detail: &str) -> Self {
        Self {
            label: label.to_string(),
            kind: CompletionItemKind::Snippet,
            detail: Some(detail.to_string()),
            documentation: None,
            insert_text: Some(insert_text.to_string()),
            sort_text: Some(format!("3_{}", label)),
        }
    }

    /// Serialize to JSON.
    pub fn to_json(&self) -> JsonValue {
        let mut entries = vec![
            ("label".to_string(), JsonValue::String(self.label.clone())),
            (
                "kind".to_string(),
                JsonValue::Number(self.kind as i32 as f64),
            ),
        ];
        if let Some(ref detail) = self.detail {
            entries.push(("detail".to_string(), JsonValue::String(detail.clone())));
        }
        if let Some(ref doc) = self.documentation {
            entries.push(("documentation".to_string(), doc.to_json()));
        }
        if let Some(ref text) = self.insert_text {
            entries.push(("insertText".to_string(), JsonValue::String(text.clone())));
        }
        if let Some(ref sort) = self.sort_text {
            entries.push(("sortText".to_string(), JsonValue::String(sort.clone())));
        }
        JsonValue::Object(entries)
    }
}

// ── Markup ────────────────────────────────────────────────────────────────────

/// Markup kind.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MarkupKind {
    /// Plain text.
    PlainText,
    /// Markdown.
    Markdown,
}

impl MarkupKind {
    /// Convert to LSP string.
    pub fn as_str(&self) -> &str {
        match self {
            MarkupKind::PlainText => "plaintext",
            MarkupKind::Markdown => "markdown",
        }
    }
}

/// Markup content for documentation.
#[derive(Clone, Debug)]
pub struct MarkupContent {
    /// The kind of markup.
    pub kind: MarkupKind,
    /// The content.
    pub value: String,
}

impl MarkupContent {
    /// Create plain text content.
    pub fn plain(text: impl Into<String>) -> Self {
        Self {
            kind: MarkupKind::PlainText,
            value: text.into(),
        }
    }

    /// Create markdown content.
    pub fn markdown(text: impl Into<String>) -> Self {
        Self {
            kind: MarkupKind::Markdown,
            value: text.into(),
        }
    }

    /// Serialize to JSON.
    pub fn to_json(&self) -> JsonValue {
        JsonValue::Object(vec![
            (
                "kind".to_string(),
                JsonValue::String(self.kind.as_str().to_string()),
            ),
            ("value".to_string(), JsonValue::String(self.value.clone())),
        ])
    }
}

// ── Hover / Signature ─────────────────────────────────────────────────────────

/// Hover result.
#[derive(Clone, Debug)]
pub struct Hover {
    /// Hover contents.
    pub contents: MarkupContent,
    /// Optional range this hover applies to.
    pub range: Option<Range>,
}

impl Hover {
    /// Create a new hover.
    pub fn new(contents: MarkupContent, range: Option<Range>) -> Self {
        Self { contents, range }
    }

    /// Serialize to JSON.
    pub fn to_json(&self) -> JsonValue {
        let mut entries = vec![("contents".to_string(), self.contents.to_json())];
        if let Some(ref range) = self.range {
            entries.push(("range".to_string(), range.to_json()));
        }
        JsonValue::Object(entries)
    }
}

/// Signature help result.
#[derive(Clone, Debug)]
pub struct SignatureHelp {
    /// Available signatures.
    pub signatures: Vec<SignatureInformation>,
    /// The active signature index.
    pub active_signature: u32,
    /// The active parameter index.
    pub active_parameter: u32,
}

impl SignatureHelp {
    /// Serialize to JSON.
    pub fn to_json(&self) -> JsonValue {
        JsonValue::Object(vec![
            (
                "signatures".to_string(),
                JsonValue::Array(self.signatures.iter().map(|s| s.to_json()).collect()),
            ),
            (
                "activeSignature".to_string(),
                JsonValue::Number(self.active_signature as f64),
            ),
            (
                "activeParameter".to_string(),
                JsonValue::Number(self.active_parameter as f64),
            ),
        ])
    }
}

/// A single signature in signature help.
#[derive(Clone, Debug)]
pub struct SignatureInformation {
    /// The label of the signature.
    pub label: String,
    /// Documentation.
    pub documentation: Option<MarkupContent>,
    /// Parameters.
    pub parameters: Vec<ParameterInformation>,
}

impl SignatureInformation {
    /// Serialize to JSON.
    pub fn to_json(&self) -> JsonValue {
        let mut entries = vec![
            ("label".to_string(), JsonValue::String(self.label.clone())),
            (
                "parameters".to_string(),
                JsonValue::Array(self.parameters.iter().map(|p| p.to_json()).collect()),
            ),
        ];
        if let Some(ref doc) = self.documentation {
            entries.push(("documentation".to_string(), doc.to_json()));
        }
        JsonValue::Object(entries)
    }
}

/// Parameter information in a signature.
#[derive(Clone, Debug)]
pub struct ParameterInformation {
    /// The label of this parameter.
    pub label: String,
    /// Documentation.
    pub documentation: Option<MarkupContent>,
}

impl ParameterInformation {
    /// Serialize to JSON.
    pub fn to_json(&self) -> JsonValue {
        let mut entries = vec![("label".to_string(), JsonValue::String(self.label.clone()))];
        if let Some(ref doc) = self.documentation {
            entries.push(("documentation".to_string(), doc.to_json()));
        }
        JsonValue::Object(entries)
    }
}

// ── Symbol types ──────────────────────────────────────────────────────────────

/// Symbol kind (LSP spec values).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SymbolKind {
    /// File.
    File = 1,
    /// Module.
    Module = 2,
    /// Namespace.
    Namespace = 3,
    /// Class.
    Class = 5,
    /// Method.
    Method = 6,
    /// Property.
    Property = 7,
    /// Function.
    Function = 12,
    /// Variable.
    Variable = 13,
    /// Constant.
    Constant = 14,
    /// String.
    StringKind = 15,
    /// Enum.
    Enum = 10,
    /// Struct.
    Struct = 23,
    /// Type parameter.
    TypeParameter = 26,
}

/// Symbol information for workspace/document symbols.
#[derive(Clone, Debug)]
pub struct SymbolInformation {
    /// Symbol name.
    pub name: String,
    /// Symbol kind.
    pub kind: SymbolKind,
    /// Location of the symbol.
    pub location: Location,
}

impl SymbolInformation {
    /// Serialize to JSON.
    pub fn to_json(&self) -> JsonValue {
        JsonValue::Object(vec![
            ("name".to_string(), JsonValue::String(self.name.clone())),
            (
                "kind".to_string(),
                JsonValue::Number(self.kind as i32 as f64),
            ),
            ("location".to_string(), self.location.to_json()),
        ])
    }
}

/// Document symbol (hierarchical).
#[derive(Clone, Debug)]
pub struct DocumentSymbol {
    /// Symbol name.
    pub name: String,
    /// Optional detail.
    pub detail: Option<String>,
    /// Symbol kind.
    pub kind: SymbolKind,
    /// Full range of the symbol.
    pub range: Range,
    /// Range for the name of the symbol.
    pub selection_range: Range,
    /// Children.
    pub children: Vec<DocumentSymbol>,
}

impl DocumentSymbol {
    /// Serialize to JSON.
    pub fn to_json(&self) -> JsonValue {
        let mut entries = vec![
            ("name".to_string(), JsonValue::String(self.name.clone())),
            (
                "kind".to_string(),
                JsonValue::Number(self.kind as i32 as f64),
            ),
            ("range".to_string(), self.range.to_json()),
            ("selectionRange".to_string(), self.selection_range.to_json()),
            (
                "children".to_string(),
                JsonValue::Array(self.children.iter().map(|c| c.to_json()).collect()),
            ),
        ];
        if let Some(ref detail) = self.detail {
            entries.push(("detail".to_string(), JsonValue::String(detail.clone())));
        }
        JsonValue::Object(entries)
    }
}

// ── Server capabilities ───────────────────────────────────────────────────────

/// Server capabilities advertised during initialization.
#[derive(Clone, Debug, Default)]
pub struct ServerCapabilities {
    /// Sync mode: 1=Full, 2=Incremental.
    pub text_document_sync: u8,
    /// Whether the server supports completions.
    pub completion_provider: bool,
    /// Whether the server supports hover.
    pub hover_provider: bool,
    /// Whether the server supports go-to-definition.
    pub definition_provider: bool,
    /// Whether the server supports find-references.
    pub references_provider: bool,
    /// Whether the server supports document symbol.
    pub document_symbol_provider: bool,
    /// Whether the server supports formatting.
    pub document_formatting_provider: bool,
    /// Whether the server supports signature help.
    pub signature_help_provider: bool,
    /// Whether the server supports code actions.
    pub code_action_provider: bool,
}

impl ServerCapabilities {
    /// Create default OxiLean server capabilities.
    pub fn oxilean_defaults() -> Self {
        Self {
            text_document_sync: 1, // Full sync
            completion_provider: true,
            hover_provider: true,
            definition_provider: true,
            references_provider: true,
            document_symbol_provider: true,
            document_formatting_provider: true,
            signature_help_provider: true,
            code_action_provider: true,
        }
    }

    /// Serialize to JSON.
    pub fn to_json(&self) -> JsonValue {
        let mut entries = vec![(
            "textDocumentSync".to_string(),
            JsonValue::Number(self.text_document_sync as f64),
        )];
        if self.completion_provider {
            entries.push((
                "completionProvider".to_string(),
                JsonValue::Object(vec![(
                    "triggerCharacters".to_string(),
                    JsonValue::Array(vec![
                        JsonValue::String(".".to_string()),
                        JsonValue::String(":".to_string()),
                    ]),
                )]),
            ));
        }
        if self.hover_provider {
            entries.push(("hoverProvider".to_string(), JsonValue::Bool(true)));
        }
        if self.definition_provider {
            entries.push(("definitionProvider".to_string(), JsonValue::Bool(true)));
        }
        if self.references_provider {
            entries.push(("referencesProvider".to_string(), JsonValue::Bool(true)));
        }
        if self.document_symbol_provider {
            entries.push(("documentSymbolProvider".to_string(), JsonValue::Bool(true)));
        }
        if self.document_formatting_provider {
            entries.push((
                "documentFormattingProvider".to_string(),
                JsonValue::Bool(true),
            ));
        }
        if self.signature_help_provider {
            entries.push((
                "signatureHelpProvider".to_string(),
                JsonValue::Object(vec![(
                    "triggerCharacters".to_string(),
                    JsonValue::Array(vec![
                        JsonValue::String("(".to_string()),
                        JsonValue::String(",".to_string()),
                    ]),
                )]),
            ));
        }
        if self.code_action_provider {
            entries.push(("codeActionProvider".to_string(), JsonValue::Bool(true)));
        }
        JsonValue::Object(entries)
    }
}

/// Initialize result sent back to the client.
#[derive(Clone, Debug)]
pub struct InitializeResult {
    /// Server capabilities.
    pub capabilities: ServerCapabilities,
    /// Server name.
    pub server_name: String,
    /// Server version.
    pub server_version: String,
}

impl InitializeResult {
    /// Serialize to JSON.
    pub fn to_json(&self) -> JsonValue {
        JsonValue::Object(vec![
            ("capabilities".to_string(), self.capabilities.to_json()),
            (
                "serverInfo".to_string(),
                JsonValue::Object(vec![
                    (
                        "name".to_string(),
                        JsonValue::String(self.server_name.clone()),
                    ),
                    (
                        "version".to_string(),
                        JsonValue::String(self.server_version.clone()),
                    ),
                ]),
            ),
        ])
    }
}

/// Parameters for publishDiagnostics notification.
#[derive(Clone, Debug)]
pub struct PublishDiagnosticsParams {
    /// Document URI.
    pub uri: String,
    /// The diagnostics.
    pub diagnostics: Vec<Diagnostic>,
    /// Document version.
    pub version: Option<i64>,
}

impl PublishDiagnosticsParams {
    /// Serialize to JSON.
    pub fn to_json(&self) -> JsonValue {
        let mut entries = vec![
            ("uri".to_string(), JsonValue::String(self.uri.clone())),
            (
                "diagnostics".to_string(),
                JsonValue::Array(self.diagnostics.iter().map(|d| d.to_json()).collect()),
            ),
        ];
        if let Some(ver) = self.version {
            entries.push(("version".to_string(), JsonValue::Number(ver as f64)));
        }
        JsonValue::Object(entries)
    }
}
