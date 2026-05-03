//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{Environment, Name};
use std::collections::HashMap;

use super::functions::{
    analyze_document, compute_line_offsets, find_references_in_document, format_document,
    format_json_value, get_keyword_hover, is_ident_char, make_code_action, parse_json_value,
    INTERNAL_ERROR, INVALID_REQUEST, METHOD_NOT_FOUND, PARSE_ERROR,
};

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
/// Cache for analysis results.
#[derive(Debug, Default)]
pub struct AnalysisCache {
    /// Cached results by URI.
    results: HashMap<String, (i64, AnalysisResult)>,
}
impl AnalysisCache {
    /// Create a new empty cache.
    pub fn new() -> Self {
        Self {
            results: HashMap::new(),
        }
    }
    /// Get a cached result if the version matches.
    pub fn get(&self, uri: &str, version: i64) -> Option<&AnalysisResult> {
        self.results
            .get(uri)
            .filter(|(v, _)| *v == version)
            .map(|(_, r)| r)
    }
    /// Store a result.
    pub fn store(&mut self, uri: impl Into<String>, version: i64, result: AnalysisResult) {
        self.results.insert(uri.into(), (version, result));
    }
    /// Invalidate a cached result.
    pub fn invalidate(&mut self, uri: &str) {
        self.results.remove(uri);
    }
    /// Clear all cached results.
    pub fn clear(&mut self) {
        self.results.clear();
    }
}
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
/// A JSON-RPC message (request, response, or notification).
#[derive(Clone, Debug)]
pub struct JsonRpcMessage {
    /// Request ID (None for notifications).
    pub id: Option<JsonValue>,
    /// Method name (for requests/notifications).
    pub method: Option<String>,
    /// Parameters.
    pub params: Option<JsonValue>,
    /// Result (for responses).
    pub result: Option<JsonValue>,
    /// Error (for error responses).
    pub error: Option<JsonRpcError>,
}
impl JsonRpcMessage {
    /// Create a request message.
    pub fn request(id: JsonValue, method: &str, params: JsonValue) -> Self {
        Self {
            id: Some(id),
            method: Some(method.to_string()),
            params: Some(params),
            result: None,
            error: None,
        }
    }
    /// Create a notification message (no id).
    pub fn notification(method: &str, params: JsonValue) -> Self {
        Self {
            id: None,
            method: Some(method.to_string()),
            params: Some(params),
            result: None,
            error: None,
        }
    }
    /// Create a success response.
    pub fn response(id: JsonValue, result: JsonValue) -> Self {
        Self {
            id: Some(id),
            method: None,
            params: None,
            result: Some(result),
            error: None,
        }
    }
    /// Create an error response.
    pub fn error_response(id: JsonValue, error: JsonRpcError) -> Self {
        Self {
            id: Some(id),
            method: None,
            params: None,
            result: None,
            error: Some(error),
        }
    }
    /// Serialize to JsonValue.
    pub fn to_json(&self) -> JsonValue {
        let mut entries = vec![("jsonrpc".to_string(), JsonValue::String("2.0".to_string()))];
        if let Some(ref id) = self.id {
            entries.push(("id".to_string(), id.clone()));
        }
        if let Some(ref method) = self.method {
            entries.push(("method".to_string(), JsonValue::String(method.clone())));
        }
        if let Some(ref params) = self.params {
            entries.push(("params".to_string(), params.clone()));
        }
        if let Some(ref result) = self.result {
            entries.push(("result".to_string(), result.clone()));
        }
        if let Some(ref error) = self.error {
            entries.push(("error".to_string(), error.to_json()));
        }
        JsonValue::Object(entries)
    }
    /// Parse from a JsonValue.
    pub fn from_json(val: &JsonValue) -> Result<Self, String> {
        let id = val.get("id").cloned();
        let method = val.get("method").and_then(|v| v.as_str()).map(String::from);
        let params = val.get("params").cloned();
        let result = val.get("result").cloned();
        let error = val.get("error").map(JsonRpcError::from_json).transpose()?;
        Ok(Self {
            id,
            method,
            params,
            result,
            error,
        })
    }
}
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
/// Result of analyzing a document.
#[derive(Clone, Debug, Default)]
pub struct AnalysisResult {
    /// Diagnostics found.
    pub diagnostics: Vec<Diagnostic>,
    /// Symbols found.
    pub symbols: Vec<DocumentSymbol>,
    /// Definitions found.
    pub definitions: Vec<DefinitionInfo>,
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
/// The core LSP server.
pub struct LspServer {
    /// Open document storage.
    pub document_store: DocumentStore,
    /// Kernel environment.
    pub env: Environment,
    /// Server configuration.
    pub config: LspConfig,
    /// Analysis cache.
    pub cache: AnalysisCache,
    /// Whether the server has been initialized.
    pub initialized: bool,
    /// Whether a shutdown was requested.
    pub shutdown_requested: bool,
}
impl LspServer {
    /// Create a new LSP server.
    pub fn new() -> Self {
        Self {
            document_store: DocumentStore::new(),
            env: Environment::new(),
            config: LspConfig::default(),
            cache: AnalysisCache::new(),
            initialized: false,
            shutdown_requested: false,
        }
    }
    /// Create a new LSP server with custom config.
    pub fn with_config(config: LspConfig) -> Self {
        Self {
            document_store: DocumentStore::new(),
            env: Environment::new(),
            config,
            cache: AnalysisCache::new(),
            initialized: false,
            shutdown_requested: false,
        }
    }
    /// Handle a JSON-RPC message and return a response (if any).
    pub fn handle_message(&mut self, msg: &JsonRpcMessage) -> Option<JsonRpcMessage> {
        let method = match &msg.method {
            Some(m) => m.clone(),
            None => {
                return None;
            }
        };
        let params = msg.params.clone().unwrap_or(JsonValue::Null);
        let id = msg.id.clone();
        match method.as_str() {
            "initialize" => {
                let result = self.handle_initialize(&params);
                id.map(|id_val| JsonRpcMessage::response(id_val, result))
            }
            "initialized" => None,
            "shutdown" => {
                let result = self.handle_shutdown();
                id.map(|id_val| JsonRpcMessage::response(id_val, result))
            }
            "exit" => None,
            "textDocument/didOpen" => {
                self.handle_text_document_did_open(&params);
                None
            }
            "textDocument/didChange" => {
                self.handle_text_document_did_change(&params);
                None
            }
            "textDocument/didClose" => {
                self.handle_text_document_did_close(&params);
                None
            }
            "textDocument/completion" => {
                let result = self.handle_completion(&params);
                id.map(|id_val| JsonRpcMessage::response(id_val, result))
            }
            "textDocument/hover" => {
                let result = self.handle_hover(&params);
                id.map(|id_val| JsonRpcMessage::response(id_val, result))
            }
            "textDocument/definition" => {
                let result = self.handle_goto_definition(&params);
                id.map(|id_val| JsonRpcMessage::response(id_val, result))
            }
            "textDocument/documentSymbol" => {
                let result = self.handle_document_symbol(&params);
                id.map(|id_val| JsonRpcMessage::response(id_val, result))
            }
            "textDocument/formatting" => {
                let result = self.handle_formatting(&params);
                id.map(|id_val| JsonRpcMessage::response(id_val, result))
            }
            "textDocument/signatureHelp" => {
                let result = self.handle_signature_help(&params);
                id.map(|id_val| JsonRpcMessage::response(id_val, result))
            }
            "textDocument/references" => {
                let result = self.handle_references(&params);
                id.map(|id_val| JsonRpcMessage::response(id_val, result))
            }
            "textDocument/codeAction" => {
                let result = self.handle_code_action(&params);
                id.map(|id_val| JsonRpcMessage::response(id_val, result))
            }
            _ => id.map(|id_val| {
                JsonRpcMessage::error_response(id_val, JsonRpcError::method_not_found(&method))
            }),
        }
    }
    /// Handle initialize request.
    pub fn handle_initialize(&mut self, _params: &JsonValue) -> JsonValue {
        self.initialized = true;
        let result = InitializeResult {
            capabilities: ServerCapabilities::oxilean_defaults(),
            server_name: "oxilean-lsp".to_string(),
            server_version: "0.1.1".to_string(),
        };
        result.to_json()
    }
    /// Handle shutdown request.
    pub fn handle_shutdown(&mut self) -> JsonValue {
        self.shutdown_requested = true;
        JsonValue::Null
    }
    /// Handle textDocument/didOpen.
    pub fn handle_text_document_did_open(&mut self, params: &JsonValue) {
        if let Some(td) = params.get("textDocument") {
            if let Ok(item) = TextDocumentItem::from_json(td) {
                self.document_store
                    .open_document(&item.uri, item.version, &item.text);
                self.cache.invalidate(&item.uri);
            }
        }
    }
    /// Handle textDocument/didChange.
    pub fn handle_text_document_did_change(&mut self, params: &JsonValue) {
        let uri = params
            .get("textDocument")
            .and_then(|td| td.get("uri"))
            .and_then(|v| v.as_str());
        let version = params
            .get("textDocument")
            .and_then(|td| td.get("version"))
            .and_then(|v| v.as_i64());
        if let (Some(uri), Some(version)) = (uri, version) {
            if let Some(changes) = params.get("contentChanges").and_then(|v| v.as_array()) {
                if let Some(last_change) = changes.last() {
                    if let Some(text) = last_change.get("text").and_then(|v| v.as_str()) {
                        self.document_store.update_document(uri, version, text);
                        self.cache.invalidate(uri);
                    }
                }
            }
        }
    }
    /// Handle textDocument/didClose.
    pub fn handle_text_document_did_close(&mut self, params: &JsonValue) {
        if let Some(uri) = params
            .get("textDocument")
            .and_then(|td| td.get("uri"))
            .and_then(|v| v.as_str())
        {
            self.document_store.close_document(uri);
            self.cache.invalidate(uri);
        }
    }
    /// Handle textDocument/completion.
    pub fn handle_completion(&self, params: &JsonValue) -> JsonValue {
        let uri = params
            .get("textDocument")
            .and_then(|td| td.get("uri"))
            .and_then(|v| v.as_str());
        let position = params
            .get("position")
            .and_then(|p| Position::from_json(p).ok());
        if let (Some(uri), Some(pos)) = (uri, position) {
            let items = self.get_completions_at(uri, &pos);
            JsonValue::Array(items.iter().map(|c| c.to_json()).collect())
        } else {
            JsonValue::Array(Vec::new())
        }
    }
    /// Handle textDocument/hover.
    pub fn handle_hover(&self, params: &JsonValue) -> JsonValue {
        let uri = params
            .get("textDocument")
            .and_then(|td| td.get("uri"))
            .and_then(|v| v.as_str());
        let position = params
            .get("position")
            .and_then(|p| Position::from_json(p).ok());
        if let (Some(uri), Some(pos)) = (uri, position) {
            if let Some(hover) = self.get_hover_info_at(uri, &pos) {
                return hover.to_json();
            }
        }
        JsonValue::Null
    }
    /// Handle textDocument/definition.
    pub fn handle_goto_definition(&self, params: &JsonValue) -> JsonValue {
        let uri = params
            .get("textDocument")
            .and_then(|td| td.get("uri"))
            .and_then(|v| v.as_str());
        let position = params
            .get("position")
            .and_then(|p| Position::from_json(p).ok());
        if let (Some(uri), Some(pos)) = (uri, position) {
            if let Some(location) = self.find_definition(uri, &pos) {
                return location.to_json();
            }
        }
        JsonValue::Null
    }
    /// Handle textDocument/documentSymbol.
    pub fn handle_document_symbol(&self, params: &JsonValue) -> JsonValue {
        let uri = params
            .get("textDocument")
            .and_then(|td| td.get("uri"))
            .and_then(|v| v.as_str());
        if let Some(uri) = uri {
            if let Some(doc) = self.document_store.get_document(uri) {
                let analysis = analyze_document(uri, &doc.content, &self.env);
                return JsonValue::Array(analysis.symbols.iter().map(|s| s.to_json()).collect());
            }
        }
        JsonValue::Array(Vec::new())
    }
    /// Handle textDocument/formatting.
    pub fn handle_formatting(&self, params: &JsonValue) -> JsonValue {
        let uri = params
            .get("textDocument")
            .and_then(|td| td.get("uri"))
            .and_then(|v| v.as_str());
        if let Some(uri) = uri {
            if let Some(doc) = self.document_store.get_document(uri) {
                let edits = format_document(&doc.content);
                return JsonValue::Array(edits.iter().map(|e| e.to_json()).collect());
            }
        }
        JsonValue::Array(Vec::new())
    }
    /// Handle textDocument/signatureHelp.
    pub fn handle_signature_help(&self, params: &JsonValue) -> JsonValue {
        let uri = params
            .get("textDocument")
            .and_then(|td| td.get("uri"))
            .and_then(|v| v.as_str());
        let position = params
            .get("position")
            .and_then(|p| Position::from_json(p).ok());
        if let (Some(uri), Some(pos)) = (uri, position) {
            if let Some(help) = self.get_signature_help(uri, &pos) {
                return help.to_json();
            }
        }
        JsonValue::Null
    }
    /// Handle textDocument/references.
    pub fn handle_references(&self, params: &JsonValue) -> JsonValue {
        let uri = params
            .get("textDocument")
            .and_then(|td| td.get("uri"))
            .and_then(|v| v.as_str());
        let position = params
            .get("position")
            .and_then(|p| Position::from_json(p).ok());
        if let (Some(uri), Some(pos)) = (uri, position) {
            if let Some(doc) = self.document_store.get_document(uri) {
                if let Some((word, _)) = doc.word_at_position(&pos) {
                    let locations = find_references_in_document(uri, &doc.content, &word);
                    return JsonValue::Array(locations.iter().map(|l| l.to_json()).collect());
                }
            }
        }
        JsonValue::Array(Vec::new())
    }
    /// Handle textDocument/codeAction.
    pub fn handle_code_action(&self, params: &JsonValue) -> JsonValue {
        let uri = params
            .get("textDocument")
            .and_then(|td| td.get("uri"))
            .and_then(|v| v.as_str());
        if let Some(uri) = uri {
            if let Some(doc) = self.document_store.get_document(uri) {
                let actions = self.get_code_actions(uri, &doc.content, params);
                return JsonValue::Array(actions);
            }
        }
        JsonValue::Array(Vec::new())
    }
    /// Validate a document and return diagnostics.
    pub fn validate_document(&self, uri: &str) -> Vec<Diagnostic> {
        if let Some(doc) = self.document_store.get_document(uri) {
            let analysis = analyze_document(uri, &doc.content, &self.env);
            analysis.diagnostics
        } else {
            Vec::new()
        }
    }
    /// Get completions at a position.
    pub fn get_completions_at(&self, uri: &str, pos: &Position) -> Vec<CompletionItem> {
        let mut items = Vec::new();
        let prefix = if let Some(doc) = self.document_store.get_document(uri) {
            if let Some(line) = doc.get_line(pos.line) {
                let col = (pos.character as usize).min(line.len());
                let mut start = col;
                while start > 0 && is_ident_char(line.as_bytes()[start - 1]) {
                    start -= 1;
                }
                line[start..col].to_string()
            } else {
                String::new()
            }
        } else {
            String::new()
        };
        let keywords = [
            "def",
            "theorem",
            "lemma",
            "axiom",
            "inductive",
            "structure",
            "class",
            "instance",
            "namespace",
            "section",
            "end",
            "variable",
            "open",
            "import",
            "export",
            "where",
            "let",
            "in",
            "fun",
            "forall",
            "match",
            "with",
            "if",
            "then",
            "else",
            "do",
            "have",
            "show",
            "by",
            "Prop",
            "Type",
            "Sort",
        ];
        for kw in &keywords {
            if prefix.is_empty() || kw.starts_with(&prefix) {
                items.push(CompletionItem::keyword(kw));
            }
        }
        for name in self.env.constant_names() {
            let name_str = name.to_string();
            if prefix.is_empty() || name_str.starts_with(&prefix) {
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
        if prefix.is_empty() || "def".starts_with(&prefix) {
            items.push(CompletionItem::snippet(
                "def ...",
                "def ${1:name} : ${2:Type} := ${0:sorry}",
                "definition template",
            ));
        }
        if prefix.is_empty() || "theorem".starts_with(&prefix) {
            items.push(CompletionItem::snippet(
                "theorem ...",
                "theorem ${1:name} : ${2:Prop} := by\n  ${0:sorry}",
                "theorem template",
            ));
        }
        if prefix.is_empty() || "inductive".starts_with(&prefix) {
            items.push(CompletionItem::snippet(
                "inductive ...",
                "inductive ${1:Name} where\n  | ${0:ctor}",
                "inductive type template",
            ));
        }
        items
    }
    /// Get hover information at a position.
    pub fn get_hover_info_at(&self, uri: &str, pos: &Position) -> Option<Hover> {
        let doc = self.document_store.get_document(uri)?;
        let (word, range) = doc.word_at_position(pos)?;
        let keyword_info = get_keyword_hover(&word);
        if let Some(info) = keyword_info {
            return Some(Hover::new(MarkupContent::markdown(info), Some(range)));
        }
        let name = Name::str(&word);
        if let Some(ci) = self.env.find(&name) {
            let ty_str = format!("{:?}", ci.ty());
            let kind = if ci.is_inductive() {
                "inductive"
            } else if ci.is_constructor() {
                "constructor"
            } else if ci.is_axiom() {
                "axiom"
            } else {
                "definition"
            };
            let md = format!("```lean\n{} {} : {}\n```", kind, word, ty_str);
            return Some(Hover::new(MarkupContent::markdown(md), Some(range)));
        }
        let analysis = analyze_document(uri, &doc.content, &self.env);
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
                let md = format!("```lean\n{} {}\n```", kind_str, word);
                return Some(Hover::new(MarkupContent::markdown(md), Some(range)));
            }
        }
        None
    }
    /// Find the definition location of a name.
    pub fn find_definition(&self, uri: &str, pos: &Position) -> Option<Location> {
        let doc = self.document_store.get_document(uri)?;
        let (word, _) = doc.word_at_position(pos)?;
        let analysis = analyze_document(uri, &doc.content, &self.env);
        for def in &analysis.definitions {
            if def.name == word {
                return Some(Location::new(uri, def.range.clone()));
            }
        }
        None
    }
    /// Get signature help at a position.
    fn get_signature_help(&self, uri: &str, pos: &Position) -> Option<SignatureHelp> {
        let doc = self.document_store.get_document(uri)?;
        let line = doc.get_line(pos.line)?;
        let col = (pos.character as usize).min(line.len());
        let before = &line[..col];
        let paren_pos = before.rfind('(')?;
        let name_end = paren_pos;
        let mut name_start = name_end;
        while name_start > 0 && is_ident_char(before.as_bytes()[name_start - 1]) {
            name_start -= 1;
        }
        if name_start == name_end {
            return None;
        }
        let func_name = &before[name_start..name_end];
        let after_paren = &before[paren_pos + 1..];
        let active_param = after_paren.chars().filter(|c| *c == ',').count() as u32;
        let name = Name::str(func_name);
        if self.env.find(&name).is_some() {
            return Some(SignatureHelp {
                signatures: vec![SignatureInformation {
                    label: format!("{} (...)", func_name),
                    documentation: None,
                    parameters: vec![ParameterInformation {
                        label: "arg".to_string(),
                        documentation: None,
                    }],
                }],
                active_signature: 0,
                active_parameter: active_param,
            });
        }
        let analysis = analyze_document(uri, &doc.content, &self.env);
        for def in &analysis.definitions {
            if def.name == func_name {
                return Some(SignatureHelp {
                    signatures: vec![SignatureInformation {
                        label: format!("{} (...)", func_name),
                        documentation: None,
                        parameters: vec![],
                    }],
                    active_signature: 0,
                    active_parameter: active_param,
                });
            }
        }
        None
    }
    /// Get code actions for a context.
    fn get_code_actions(&self, uri: &str, content: &str, params: &JsonValue) -> Vec<JsonValue> {
        let mut actions = Vec::new();
        let diagnostics = params
            .get("context")
            .and_then(|c| c.get("diagnostics"))
            .and_then(|d| d.as_array());
        if let Some(diags) = diagnostics {
            for diag in diags {
                if let Some(msg) = diag.get("message").and_then(|m| m.as_str()) {
                    if msg.contains("shadows existing declaration") {
                        actions.push(make_code_action(
                            "Rename to avoid shadowing",
                            uri,
                            Vec::new(),
                        ));
                    }
                }
            }
        }
        let _ = content;
        actions
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
/// A JSON value (subset sufficient for LSP).
#[derive(Clone, Debug, PartialEq)]
pub enum JsonValue {
    /// JSON null.
    Null,
    /// JSON boolean.
    Bool(bool),
    /// JSON number (stored as f64).
    Number(f64),
    /// JSON string.
    String(String),
    /// JSON array.
    Array(Vec<JsonValue>),
    /// JSON object (ordered by insertion via Vec).
    Object(Vec<(String, JsonValue)>),
}
impl JsonValue {
    /// Get a value from an object by key.
    pub fn get(&self, key: &str) -> Option<&JsonValue> {
        match self {
            JsonValue::Object(entries) => entries.iter().find(|(k, _)| k == key).map(|(_, v)| v),
            _ => None,
        }
    }
    /// Get a string value.
    pub fn as_str(&self) -> Option<&str> {
        match self {
            JsonValue::String(s) => Some(s),
            _ => None,
        }
    }
    /// Get a number value as i64.
    pub fn as_i64(&self) -> Option<i64> {
        match self {
            JsonValue::Number(n) => Some(*n as i64),
            _ => None,
        }
    }
    /// Get a number value as u32.
    pub fn as_u32(&self) -> Option<u32> {
        match self {
            JsonValue::Number(n) => {
                let i = *n as i64;
                if i >= 0 && i <= u32::MAX as i64 {
                    Some(i as u32)
                } else {
                    None
                }
            }
            _ => None,
        }
    }
    /// Get a boolean value.
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            JsonValue::Bool(b) => Some(*b),
            _ => None,
        }
    }
    /// Get an array value.
    pub fn as_array(&self) -> Option<&Vec<JsonValue>> {
        match self {
            JsonValue::Array(arr) => Some(arr),
            _ => None,
        }
    }
    /// Check if the value is null.
    pub fn is_null(&self) -> bool {
        matches!(self, JsonValue::Null)
    }
    /// Create a JSON object from key-value pairs.
    pub fn object(pairs: Vec<(String, JsonValue)>) -> Self {
        JsonValue::Object(pairs)
    }
}
/// Configuration for the LSP server.
#[derive(Clone, Debug)]
pub struct LspConfig {
    /// Maximum number of diagnostics to report.
    pub max_diagnostics: usize,
    /// Whether to enable completions.
    pub enable_completions: bool,
    /// Whether to enable hover.
    pub enable_hover: bool,
    /// Whether to enable go-to-definition.
    pub enable_goto_definition: bool,
}
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
    /// Whether the server supports pull-model diagnostics
    /// (LSP 3.17 `textDocument/diagnostic`).
    ///
    /// When `true` the `diagnosticProvider` capability object is emitted
    /// with `interFileDependencies: true` and `workspaceDiagnostics: true`.
    pub diagnostic_provider: bool,
    /// Whether the server supports inlay hints
    /// (`textDocument/inlayHint`).
    pub inlay_hints_provider: bool,
    /// Whether the server supports call hierarchy
    /// (`callHierarchy/incomingCalls`, `callHierarchy/outgoingCalls`).
    pub call_hierarchy_provider: bool,
    /// Whether the server supports folding ranges
    /// (`textDocument/foldingRange`).
    pub folding_range_provider: bool,
}
impl ServerCapabilities {
    /// Create default OxiLean server capabilities.
    pub fn oxilean_defaults() -> Self {
        Self {
            text_document_sync: 1,
            completion_provider: true,
            hover_provider: true,
            definition_provider: true,
            references_provider: true,
            document_symbol_provider: true,
            document_formatting_provider: true,
            signature_help_provider: true,
            code_action_provider: true,
            diagnostic_provider: true,
            inlay_hints_provider: true,
            call_hierarchy_provider: true,
            folding_range_provider: true,
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
        if self.diagnostic_provider {
            // LSP 3.17 pull-diagnostics capability with inter-file dependency
            // tracking and workspace-wide diagnostics support enabled.
            entries.push((
                "diagnosticProvider".to_string(),
                JsonValue::Object(vec![
                    (
                        "identifier".to_string(),
                        JsonValue::String("oxilean".to_string()),
                    ),
                    ("interFileDependencies".to_string(), JsonValue::Bool(true)),
                    ("workspaceDiagnostics".to_string(), JsonValue::Bool(true)),
                ]),
            ));
        }
        if self.inlay_hints_provider {
            // Advertise inlay hints support; `resolveProvider: false` indicates
            // hints are fully resolved in the initial response.
            entries.push((
                "inlayHintsProvider".to_string(),
                JsonValue::Object(vec![(
                    "resolveProvider".to_string(),
                    JsonValue::Bool(false),
                )]),
            ));
        }
        if self.call_hierarchy_provider {
            entries.push(("callHierarchyProvider".to_string(), JsonValue::Bool(true)));
        }
        if self.folding_range_provider {
            entries.push(("foldingRangeProvider".to_string(), JsonValue::Bool(true)));
        }
        JsonValue::Object(entries)
    }
}
/// Storage for all open documents.
#[derive(Debug, Default)]
pub struct DocumentStore {
    /// Map from URI to Document.
    documents: HashMap<String, Document>,
}
impl DocumentStore {
    /// Create a new empty document store.
    pub fn new() -> Self {
        Self {
            documents: HashMap::new(),
        }
    }
    /// Open a document.
    pub fn open_document(
        &mut self,
        uri: impl Into<String>,
        version: i64,
        content: impl Into<String>,
    ) {
        let uri = uri.into();
        let doc = Document::new(uri.clone(), version, content);
        self.documents.insert(uri, doc);
    }
    /// Update a document's content.
    pub fn update_document(&mut self, uri: &str, version: i64, content: impl Into<String>) -> bool {
        if let Some(doc) = self.documents.get_mut(uri) {
            doc.update(version, content);
            true
        } else {
            false
        }
    }
    /// Close a document.
    pub fn close_document(&mut self, uri: &str) -> bool {
        self.documents.remove(uri).is_some()
    }
    /// Get a document.
    pub fn get_document(&self, uri: &str) -> Option<&Document> {
        self.documents.get(uri)
    }
    /// Get all open document URIs.
    pub fn uris(&self) -> Vec<&String> {
        self.documents.keys().collect()
    }
    /// Get the number of open documents.
    pub fn len(&self) -> usize {
        self.documents.len()
    }
    /// Check if there are no open documents.
    pub fn is_empty(&self) -> bool {
        self.documents.is_empty()
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
/// A JSON-RPC error object.
#[derive(Clone, Debug)]
pub struct JsonRpcError {
    /// Error code.
    pub code: i64,
    /// Human-readable message.
    pub message: String,
    /// Optional additional data.
    pub data: Option<JsonValue>,
}
impl JsonRpcError {
    /// Create a new error.
    pub fn new(code: i64, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
            data: None,
        }
    }
    /// Create a parse error.
    pub fn parse_error(msg: impl Into<String>) -> Self {
        Self::new(PARSE_ERROR, msg)
    }
    /// Create an invalid request error.
    pub fn invalid_request(msg: impl Into<String>) -> Self {
        Self::new(INVALID_REQUEST, msg)
    }
    /// Create a method not found error.
    pub fn method_not_found(method: &str) -> Self {
        Self::new(METHOD_NOT_FOUND, format!("method not found: {}", method))
    }
    /// Create an internal error.
    pub fn internal_error(msg: impl Into<String>) -> Self {
        Self::new(INTERNAL_ERROR, msg)
    }
    /// Serialize to JsonValue.
    pub fn to_json(&self) -> JsonValue {
        let mut entries = vec![
            ("code".to_string(), JsonValue::Number(self.code as f64)),
            (
                "message".to_string(),
                JsonValue::String(self.message.clone()),
            ),
        ];
        if let Some(ref data) = self.data {
            entries.push(("data".to_string(), data.clone()));
        }
        JsonValue::Object(entries)
    }
    /// Parse from JsonValue.
    pub fn from_json(val: &JsonValue) -> Result<Self, String> {
        let code = val
            .get("code")
            .and_then(|v| v.as_i64())
            .ok_or("missing error code")?;
        let message = val
            .get("message")
            .and_then(|v| v.as_str())
            .ok_or("missing error message")?
            .to_string();
        let data = val.get("data").cloned();
        Ok(Self {
            code,
            message,
            data,
        })
    }
}
/// A definition found during analysis.
#[derive(Clone, Debug)]
pub struct DefinitionInfo {
    /// Name of the definition.
    pub name: String,
    /// Kind of the definition.
    pub kind: SymbolKind,
    /// Range where it's defined.
    pub range: Range,
    /// Type annotation if available.
    pub ty: Option<String>,
    /// Documentation if available.
    pub doc: Option<String>,
}
/// An open document with computed line offsets.
#[derive(Clone, Debug)]
pub struct Document {
    /// URI of the document.
    pub uri: String,
    /// Version number.
    pub version: i64,
    /// Full text content.
    pub content: String,
    /// Byte offsets of line starts (line_offsets\[i\] = byte offset of line i).
    pub line_offsets: Vec<usize>,
}
impl Document {
    /// Create a new document from text.
    pub fn new(uri: impl Into<String>, version: i64, content: impl Into<String>) -> Self {
        let uri = uri.into();
        let content = content.into();
        let line_offsets = compute_line_offsets(&content);
        Self {
            uri,
            version,
            content,
            line_offsets,
        }
    }
    /// Update the document content.
    pub fn update(&mut self, version: i64, content: impl Into<String>) {
        self.version = version;
        self.content = content.into();
        self.line_offsets = compute_line_offsets(&self.content);
    }
    /// Get the text of a specific line (0-indexed).
    pub fn get_line(&self, line: u32) -> Option<&str> {
        let idx = line as usize;
        if idx >= self.line_offsets.len() {
            return None;
        }
        let start = self.line_offsets[idx];
        let end = if idx + 1 < self.line_offsets.len() {
            let e = self.line_offsets[idx + 1];
            if e > 0 && self.content.as_bytes().get(e - 1) == Some(&b'\n') {
                e - 1
            } else {
                e
            }
        } else {
            self.content.len()
        };
        Some(&self.content[start..end])
    }
    /// Convert an LSP position to a byte offset.
    pub fn position_to_offset(&self, pos: &Position) -> Option<usize> {
        let line_idx = pos.line as usize;
        if line_idx >= self.line_offsets.len() {
            return None;
        }
        let line_start = self.line_offsets[line_idx];
        let line_text = self.get_line(pos.line)?;
        let mut utf16_offset = 0u32;
        let mut byte_offset = 0usize;
        for ch in line_text.chars() {
            if utf16_offset >= pos.character {
                break;
            }
            utf16_offset += ch.len_utf16() as u32;
            byte_offset += ch.len_utf8();
        }
        Some(line_start + byte_offset)
    }
    /// Convert a byte offset to an LSP position.
    pub fn offset_to_position(&self, offset: usize) -> Position {
        let offset = offset.min(self.content.len());
        let line_idx = match self.line_offsets.binary_search(&offset) {
            Ok(idx) => idx,
            Err(idx) => {
                if idx > 0 {
                    idx - 1
                } else {
                    0
                }
            }
        };
        let line_start = self.line_offsets[line_idx];
        let text_slice = &self.content[line_start..offset];
        let character: u32 = text_slice.chars().map(|c| c.len_utf16() as u32).sum();
        Position::new(line_idx as u32, character)
    }
    /// Get the number of lines.
    pub fn line_count(&self) -> usize {
        self.line_offsets.len()
    }
    /// Get the word at a given position.
    pub fn word_at_position(&self, pos: &Position) -> Option<(String, Range)> {
        let line_text = self.get_line(pos.line)?;
        let char_idx = pos.character as usize;
        if char_idx > line_text.len() {
            return None;
        }
        let bytes = line_text.as_bytes();
        let mut start = char_idx;
        while start > 0 && is_ident_char(bytes[start - 1]) {
            start -= 1;
        }
        let mut end = char_idx;
        while end < bytes.len() && is_ident_char(bytes[end]) {
            end += 1;
        }
        if start == end {
            return None;
        }
        let word = line_text[start..end].to_string();
        let range = Range::single_line(pos.line, start as u32, end as u32);
        Some((word, range))
    }
}
