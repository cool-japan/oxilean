//! LSP (Language Server Protocol) server for OxiLean.
//!
//! Implements a JSON-RPC based language server that provides IDE features
//! such as diagnostics, completions, hover, go-to-definition, and more.
//! Uses only std library + internal oxilean crates (zero external deps).

#![allow(dead_code)]

pub mod completion;
pub mod completion_adv;
pub mod diagnostics;
pub mod diagnostics_adv;
pub mod hover;
pub mod hover_adv;
pub mod semantic_tokens;
pub mod server;
pub mod widgets;

use std::collections::HashMap;
use std::fmt;

use oxilean_kernel::{Environment, Name};
use oxilean_parse::{Lexer, TokenKind};

// ============================================================================
// 1. JSON-RPC Protocol (~400 lines)
// ============================================================================

/// Standard JSON-RPC error codes.
pub const PARSE_ERROR: i64 = -32700;
/// Invalid JSON-RPC request.
pub const INVALID_REQUEST: i64 = -32600;
/// Method not found.
pub const METHOD_NOT_FOUND: i64 = -32601;
/// Invalid method parameters.
pub const INVALID_PARAMS: i64 = -32602;
/// Internal JSON-RPC error.
pub const INTERNAL_ERROR: i64 = -32603;
/// Server not initialized.
pub const SERVER_NOT_INITIALIZED: i64 = -32002;
/// Request cancelled.
pub const REQUEST_CANCELLED: i64 = -32800;

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

impl fmt::Display for JsonValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", format_json_value(self))
    }
}

/// Format a JSON value into a string.
pub fn format_json_value(val: &JsonValue) -> String {
    match val {
        JsonValue::Null => "null".to_string(),
        JsonValue::Bool(b) => {
            if *b {
                "true".to_string()
            } else {
                "false".to_string()
            }
        }
        JsonValue::Number(n) => {
            if *n == (*n as i64) as f64 && n.is_finite() {
                format!("{}", *n as i64)
            } else {
                format!("{}", n)
            }
        }
        JsonValue::String(s) => format!("\"{}\"", escape_json_string(s)),
        JsonValue::Array(arr) => {
            let items: Vec<String> = arr.iter().map(format_json_value).collect();
            format!("[{}]", items.join(","))
        }
        JsonValue::Object(entries) => {
            let items: Vec<String> = entries
                .iter()
                .map(|(k, v)| format!("\"{}\":{}", escape_json_string(k), format_json_value(v)))
                .collect();
            format!("{{{}}}", items.join(","))
        }
    }
}

fn escape_json_string(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    for ch in s.chars() {
        match ch {
            '"' => result.push_str("\\\""),
            '\\' => result.push_str("\\\\"),
            '\n' => result.push_str("\\n"),
            '\r' => result.push_str("\\r"),
            '\t' => result.push_str("\\t"),
            c if (c as u32) < 0x20 => {
                result.push_str(&format!("\\u{:04x}", c as u32));
            }
            c => result.push(c),
        }
    }
    result
}

/// Parse a JSON string into a JsonValue.
pub fn parse_json_value(input: &str) -> Result<(JsonValue, usize), String> {
    let input = input.as_bytes();
    parse_value(input, 0)
}

fn skip_whitespace(input: &[u8], mut pos: usize) -> usize {
    while pos < input.len() {
        match input[pos] {
            b' ' | b'\t' | b'\n' | b'\r' => pos += 1,
            _ => break,
        }
    }
    pos
}

fn parse_value(input: &[u8], pos: usize) -> Result<(JsonValue, usize), String> {
    let pos = skip_whitespace(input, pos);
    if pos >= input.len() {
        return Err("unexpected end of input".to_string());
    }
    match input[pos] {
        b'n' => parse_null(input, pos),
        b't' | b'f' => parse_bool(input, pos),
        b'"' => parse_string(input, pos),
        b'[' => parse_array(input, pos),
        b'{' => parse_object(input, pos),
        b'-' | b'0'..=b'9' => parse_number(input, pos),
        c => Err(format!(
            "unexpected character '{}' at position {}",
            c as char, pos
        )),
    }
}

fn parse_null(input: &[u8], pos: usize) -> Result<(JsonValue, usize), String> {
    if input.len() >= pos + 4 && &input[pos..pos + 4] == b"null" {
        Ok((JsonValue::Null, pos + 4))
    } else {
        Err(format!("expected 'null' at position {}", pos))
    }
}

fn parse_bool(input: &[u8], pos: usize) -> Result<(JsonValue, usize), String> {
    if input.len() >= pos + 4 && &input[pos..pos + 4] == b"true" {
        Ok((JsonValue::Bool(true), pos + 4))
    } else if input.len() >= pos + 5 && &input[pos..pos + 5] == b"false" {
        Ok((JsonValue::Bool(false), pos + 5))
    } else {
        Err(format!("expected boolean at position {}", pos))
    }
}

fn parse_string(input: &[u8], pos: usize) -> Result<(JsonValue, usize), String> {
    let (s, end) = parse_raw_string(input, pos)?;
    Ok((JsonValue::String(s), end))
}

fn parse_raw_string(input: &[u8], pos: usize) -> Result<(String, usize), String> {
    if pos >= input.len() || input[pos] != b'"' {
        return Err(format!("expected '\"' at position {}", pos));
    }
    let mut result = String::new();
    let mut i = pos + 1;
    while i < input.len() {
        match input[i] {
            b'"' => return Ok((result, i + 1)),
            b'\\' => {
                i += 1;
                if i >= input.len() {
                    return Err("unterminated string escape".to_string());
                }
                match input[i] {
                    b'"' => result.push('"'),
                    b'\\' => result.push('\\'),
                    b'/' => result.push('/'),
                    b'n' => result.push('\n'),
                    b'r' => result.push('\r'),
                    b't' => result.push('\t'),
                    b'b' => result.push('\u{0008}'),
                    b'f' => result.push('\u{000C}'),
                    b'u' => {
                        if i + 4 >= input.len() {
                            return Err("unterminated unicode escape".to_string());
                        }
                        let hex = std::str::from_utf8(&input[i + 1..i + 5])
                            .map_err(|_| "invalid unicode escape".to_string())?;
                        let cp = u32::from_str_radix(hex, 16)
                            .map_err(|_| "invalid unicode escape".to_string())?;
                        if let Some(c) = char::from_u32(cp) {
                            result.push(c);
                        }
                        i += 4;
                    }
                    _ => {
                        result.push('\\');
                        result.push(input[i] as char);
                    }
                }
                i += 1;
            }
            c => {
                result.push(c as char);
                i += 1;
            }
        }
    }
    Err("unterminated string".to_string())
}

fn parse_number(input: &[u8], pos: usize) -> Result<(JsonValue, usize), String> {
    let mut i = pos;
    if i < input.len() && input[i] == b'-' {
        i += 1;
    }
    let start = i;
    while i < input.len() && input[i].is_ascii_digit() {
        i += 1;
    }
    if i == start && (pos == i || (pos + 1 == i && input[pos] == b'-')) {
        return Err(format!("expected digit at position {}", i));
    }
    if i < input.len() && input[i] == b'.' {
        i += 1;
        while i < input.len() && input[i].is_ascii_digit() {
            i += 1;
        }
    }
    if i < input.len() && (input[i] == b'e' || input[i] == b'E') {
        i += 1;
        if i < input.len() && (input[i] == b'+' || input[i] == b'-') {
            i += 1;
        }
        while i < input.len() && input[i].is_ascii_digit() {
            i += 1;
        }
    }
    let s = std::str::from_utf8(&input[pos..i]).map_err(|_| "invalid number".to_string())?;
    let n: f64 = s.parse().map_err(|_| format!("invalid number: {}", s))?;
    Ok((JsonValue::Number(n), i))
}

fn parse_array(input: &[u8], pos: usize) -> Result<(JsonValue, usize), String> {
    let mut i = pos + 1; // skip '['
    let mut items = Vec::new();
    i = skip_whitespace(input, i);
    if i < input.len() && input[i] == b']' {
        return Ok((JsonValue::Array(items), i + 1));
    }
    loop {
        let (val, next) = parse_value(input, i)?;
        items.push(val);
        i = skip_whitespace(input, next);
        if i >= input.len() {
            return Err("unterminated array".to_string());
        }
        if input[i] == b']' {
            return Ok((JsonValue::Array(items), i + 1));
        }
        if input[i] != b',' {
            return Err(format!("expected ',' or ']' at position {}", i));
        }
        i += 1;
    }
}

fn parse_object(input: &[u8], pos: usize) -> Result<(JsonValue, usize), String> {
    let mut i = pos + 1; // skip '{'
    let mut entries = Vec::new();
    i = skip_whitespace(input, i);
    if i < input.len() && input[i] == b'}' {
        return Ok((JsonValue::Object(entries), i + 1));
    }
    loop {
        i = skip_whitespace(input, i);
        let (key, next) = parse_raw_string(input, i)?;
        i = skip_whitespace(input, next);
        if i >= input.len() || input[i] != b':' {
            return Err(format!("expected ':' at position {}", i));
        }
        i += 1;
        let (val, next) = parse_value(input, i)?;
        entries.push((key, val));
        i = skip_whitespace(input, next);
        if i >= input.len() {
            return Err("unterminated object".to_string());
        }
        if input[i] == b'}' {
            return Ok((JsonValue::Object(entries), i + 1));
        }
        if input[i] != b',' {
            return Err(format!("expected ',' or '}}' at position {}", i));
        }
        i += 1;
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

// ============================================================================
// 2. LSP Types (~600 lines)
// ============================================================================

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

// ============================================================================
// 3. Document Management (~400 lines)
// ============================================================================

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
            // Strip trailing newline
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
        // LSP character is UTF-16 code units. For ASCII-heavy code,
        // character == byte offset within line. We handle basic UTF-8
        // by iterating characters.
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
        // Binary search for the line
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
        // Count UTF-16 code units from line start to offset
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
        // Find word boundaries
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

fn is_ident_char(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_' || b == b'.' || b == b'\''
}

/// Compute the byte offsets of each line start.
fn compute_line_offsets(text: &str) -> Vec<usize> {
    let mut offsets = vec![0];
    for (i, byte) in text.bytes().enumerate() {
        if byte == b'\n' {
            offsets.push(i + 1);
        }
    }
    offsets
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

// ============================================================================
// 4. Analysis Engine (~300 lines)
// ============================================================================

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

/// Analyze a document using the lexer and parser.
pub fn analyze_document(uri: &str, content: &str, env: &Environment) -> AnalysisResult {
    let mut result = AnalysisResult::default();
    // Run lexer to find token-level errors
    let mut lexer = Lexer::new(content);
    let tokens = lexer.tokenize();
    for token in &tokens {
        if let TokenKind::Error(msg) = &token.kind {
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
            result.diagnostics.push(Diagnostic::error(
                Range::single_line(line, col, col + 1),
                format!("lexer error: {}", msg),
            ));
        }
    }

    // Extract symbols and definitions from tokens
    extract_symbols_from_tokens(uri, &tokens, &mut result);

    // Check environment for name collisions
    for def in &result.definitions {
        let name = Name::str(&def.name);
        if env.contains(&name) {
            result.diagnostics.push(Diagnostic::warning(
                def.range.clone(),
                format!("'{}' shadows existing declaration in environment", def.name),
            ));
        }
    }

    result
}

/// Extract symbols from token stream.
fn extract_symbols_from_tokens(
    _uri: &str,
    tokens: &[oxilean_parse::tokens::Token],
    result: &mut AnalysisResult,
) {
    let mut i = 0;
    while i < tokens.len() {
        let token = &tokens[i];
        let (decl_keyword, sym_kind) = match &token.kind {
            TokenKind::Definition => ("def", SymbolKind::Function),
            TokenKind::Theorem => ("theorem", SymbolKind::Method),
            TokenKind::Lemma => ("theorem", SymbolKind::Method),
            TokenKind::Axiom => ("axiom", SymbolKind::Constant),
            TokenKind::Inductive => ("inductive", SymbolKind::Enum),
            TokenKind::Structure => ("structure", SymbolKind::Struct),
            TokenKind::Class => ("class", SymbolKind::Class),
            TokenKind::Instance => ("instance", SymbolKind::Property),
            TokenKind::Namespace => ("namespace", SymbolKind::Namespace),
            _ => {
                i += 1;
                continue;
            }
        };

        // The next token should be the name
        if i + 1 < tokens.len() {
            if let TokenKind::Ident(name) = &tokens[i + 1].kind {
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
                let name_token = &tokens[i + 1];
                let name_col = if name_token.span.column > 0 {
                    name_token.span.column as u32 - 1
                } else {
                    0
                };
                let name_end = name_col + (name_token.span.end - name_token.span.start) as u32;

                let range = Range::single_line(line, col, name_end);
                let selection_range = Range::single_line(line, name_col, name_end);

                result.symbols.push(DocumentSymbol {
                    name: name.clone(),
                    detail: Some(format!("{} {}", decl_keyword, name)),
                    kind: sym_kind,
                    range: range.clone(),
                    selection_range,
                    children: Vec::new(),
                });

                result.definitions.push(DefinitionInfo {
                    name: name.clone(),
                    kind: sym_kind,
                    range,
                    ty: None,
                    doc: None,
                });
            }
        }
        i += 1;
    }
}

/// Find all references to a name in a token stream.
pub fn find_references_in_document(uri: &str, content: &str, name: &str) -> Vec<Location> {
    let mut locations = Vec::new();
    let mut lexer = Lexer::new(content);
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
                locations.push(Location::new(uri, Range::single_line(line, col, end_col)));
            }
        }
    }
    locations
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

// ============================================================================
// 5. LSP Server Core (~600 lines)
// ============================================================================

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

impl Default for LspConfig {
    fn default() -> Self {
        Self {
            max_diagnostics: 100,
            enable_completions: true,
            enable_hover: true,
            enable_goto_definition: true,
        }
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
                // This is a response, not a request; ignore it
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
            "initialized" => {
                // Client confirms initialization, nothing to respond
                None
            }
            "shutdown" => {
                let result = self.handle_shutdown();
                id.map(|id_val| JsonRpcMessage::response(id_val, result))
            }
            "exit" => {
                // Server should exit
                None
            }
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
            // Full sync: take the last content change
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

        // Get the document and try to determine context
        let prefix = if let Some(doc) = self.document_store.get_document(uri) {
            if let Some(line) = doc.get_line(pos.line) {
                let col = (pos.character as usize).min(line.len());
                // Walk backwards to find the prefix
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

        // Add keyword completions
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

        // Add environment completions
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

        // Add snippet completions
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

        // Check if it's a keyword
        let keyword_info = get_keyword_hover(&word);
        if let Some(info) = keyword_info {
            return Some(Hover::new(MarkupContent::markdown(info), Some(range)));
        }

        // Check environment
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

        // Check local definitions from analysis
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

        // Search in the current document's analysis
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

        // Walk backwards to find function name before '('
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

        // Count commas to determine active parameter
        let after_paren = &before[paren_pos + 1..];
        let active_param = after_paren.chars().filter(|c| *c == ',').count() as u32;

        // Look up in environment
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

        // Return a placeholder if the function is locally defined
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
                    // Offer quick fixes based on diagnostic messages
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

        // Offer "add sorry" if there's an incomplete proof region
        let _ = content; // Used above conceptually; prevent unused warning
        actions
    }
}

/// Create a code action JSON value.
fn make_code_action(title: &str, _uri: &str, edits: Vec<TextEdit>) -> JsonValue {
    let mut entries = vec![
        ("title".to_string(), JsonValue::String(title.to_string())),
        (
            "kind".to_string(),
            JsonValue::String("quickfix".to_string()),
        ),
    ];
    if !edits.is_empty() {
        entries.push((
            "edit".to_string(),
            JsonValue::Object(vec![(
                "changes".to_string(),
                JsonValue::Array(edits.iter().map(|e| e.to_json()).collect()),
            )]),
        ));
    }
    JsonValue::Object(entries)
}

/// Get hover information for Lean keywords.
fn get_keyword_hover(word: &str) -> Option<String> {
    let info = match word {
        "def" | "definition" => {
            "**def** -- Define a new function or value.\n\n```lean\ndef name : Type := value\n```"
        }
        "theorem" | "lemma" => {
            "**theorem** -- State and prove a proposition.\n\n```lean\ntheorem name : Prop := proof\n```"
        }
        "axiom" => {
            "**axiom** -- Postulate a type without proof.\n\n```lean\naxiom name : Type\n```"
        }
        "inductive" => {
            "**inductive** -- Define an inductive type.\n\n```lean\ninductive Name where\n  | ctor : Name\n```"
        }
        "structure" => "**structure** -- Define a record type with named fields.",
        "class" => "**class** -- Define a type class for ad-hoc polymorphism.",
        "instance" => "**instance** -- Provide a type class instance.",
        "fun" => "**fun** -- Lambda abstraction.\n\n```lean\nfun x => x + 1\n```",
        "forall" => "**forall** -- Universal quantification / dependent function type.",
        "match" => "**match** -- Pattern matching.\n\n```lean\nmatch x with\n  | pattern => result\n```",
        "let" => "**let** -- Local binding.\n\n```lean\nlet x := value\nin body\n```",
        "if" => "**if** -- Conditional expression.\n\n```lean\nif cond then a else b\n```",
        "do" => "**do** -- Do-notation for monadic code.",
        "by" => "**by** -- Enter tactic mode to construct a proof.",
        "sorry" => "**sorry** -- Placeholder for incomplete proofs (axiom).",
        "Prop" => "**Prop** -- The type of propositions (`Sort 0`).",
        "Type" => "**Type** -- The type of types (`Sort 1`).",
        "Sort" => "**Sort** -- The type of a universe level.",
        "where" => "**where** -- Introduce local definitions after a declaration.",
        "have" => "**have** -- Introduce a local hypothesis.",
        "show" => "**show** -- Annotate the expected type of an expression.",
        "namespace" => "**namespace** -- Open a namespace for declarations.",
        "section" => "**section** -- Begin a section for local variables.",
        "open" => "**open** -- Open a namespace to use its names unqualified.",
        "import" => "**import** -- Import definitions from another module.",
        _ => return None,
    };
    Some(info.to_string())
}

/// Simple document formatting: normalize whitespace.
fn format_document(content: &str) -> Vec<TextEdit> {
    let mut edits = Vec::new();
    let lines: Vec<&str> = content.lines().collect();

    for (i, line) in lines.iter().enumerate() {
        // Remove trailing whitespace
        let trimmed_end = line.trim_end();
        if trimmed_end.len() != line.len() {
            edits.push(TextEdit::new(
                Range::single_line(i as u32, trimmed_end.len() as u32, line.len() as u32),
                "",
            ));
        }
    }

    // Ensure file ends with a newline
    if !content.is_empty() && !content.ends_with('\n') {
        let last_line = lines.len().saturating_sub(1);
        let last_col = lines.last().map_or(0, |l| l.len());
        edits.push(TextEdit::new(
            Range::single_line(last_line as u32, last_col as u32, last_col as u32),
            "\n",
        ));
    }

    edits
}

// ============================================================================
// 6. Tests (~200 lines)
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // --- JSON parsing tests ---

    #[test]
    fn test_parse_json_null() {
        let (val, _) = parse_json_value("null").expect("parsing should succeed");
        assert_eq!(val, JsonValue::Null);
    }

    #[test]
    fn test_parse_json_bool() {
        let (val, _) = parse_json_value("true").expect("parsing should succeed");
        assert_eq!(val, JsonValue::Bool(true));
        let (val, _) = parse_json_value("false").expect("parsing should succeed");
        assert_eq!(val, JsonValue::Bool(false));
    }

    #[test]
    fn test_parse_json_number() {
        let (val, _) = parse_json_value("42").expect("parsing should succeed");
        assert_eq!(val, JsonValue::Number(42.0));
        let (val, _) = parse_json_value("-7").expect("parsing should succeed");
        assert_eq!(val, JsonValue::Number(-7.0));
        let (val, _) = parse_json_value("3.5").expect("parsing should succeed");
        assert_eq!(val, JsonValue::Number(3.5));
    }

    #[test]
    fn test_parse_json_string() {
        let (val, _) = parse_json_value("\"hello\"").expect("parsing should succeed");
        assert_eq!(val, JsonValue::String("hello".to_string()));
    }

    #[test]
    fn test_parse_json_string_escapes() {
        let (val, _) = parse_json_value("\"a\\nb\"").expect("parsing should succeed");
        assert_eq!(val, JsonValue::String("a\nb".to_string()));
        let (val, _) = parse_json_value("\"a\\\\b\"").expect("parsing should succeed");
        assert_eq!(val, JsonValue::String("a\\b".to_string()));
        let (val, _) = parse_json_value("\"a\\\"b\"").expect("parsing should succeed");
        assert_eq!(val, JsonValue::String("a\"b".to_string()));
    }

    #[test]
    fn test_parse_json_array() {
        let (val, _) = parse_json_value("[1, 2, 3]").expect("parsing should succeed");
        assert_eq!(
            val,
            JsonValue::Array(vec![
                JsonValue::Number(1.0),
                JsonValue::Number(2.0),
                JsonValue::Number(3.0),
            ])
        );
    }

    #[test]
    fn test_parse_json_empty_array() {
        let (val, _) = parse_json_value("[]").expect("parsing should succeed");
        assert_eq!(val, JsonValue::Array(Vec::new()));
    }

    #[test]
    fn test_parse_json_object() {
        let (val, _) =
            parse_json_value("{\"a\": 1, \"b\": \"two\"}").expect("parsing should succeed");
        assert_eq!(
            val,
            JsonValue::Object(vec![
                ("a".to_string(), JsonValue::Number(1.0)),
                ("b".to_string(), JsonValue::String("two".to_string())),
            ])
        );
    }

    #[test]
    fn test_parse_json_empty_object() {
        let (val, _) = parse_json_value("{}").expect("parsing should succeed");
        assert_eq!(val, JsonValue::Object(Vec::new()));
    }

    #[test]
    fn test_parse_json_nested() {
        let input = "{\"items\": [1, {\"nested\": true}]}";
        let (val, _) = parse_json_value(input).expect("parsing should succeed");
        let items = val
            .get("items")
            .expect("key should exist")
            .as_array()
            .expect("key should exist");
        assert_eq!(items.len(), 2);
        assert_eq!(
            items[1].get("nested").expect("key should exist").as_bool(),
            Some(true)
        );
    }

    // --- JSON serialization tests ---

    #[test]
    fn test_format_json_null() {
        assert_eq!(format_json_value(&JsonValue::Null), "null");
    }

    #[test]
    fn test_format_json_string_escape() {
        let val = JsonValue::String("a\"b\nc".to_string());
        let s = format_json_value(&val);
        assert_eq!(s, "\"a\\\"b\\nc\"");
    }

    #[test]
    fn test_format_json_roundtrip() {
        let original = JsonValue::Object(vec![
            ("id".to_string(), JsonValue::Number(1.0)),
            ("method".to_string(), JsonValue::String("test".to_string())),
            ("active".to_string(), JsonValue::Bool(true)),
            ("data".to_string(), JsonValue::Null),
        ]);
        let s = format_json_value(&original);
        let (parsed, _) = parse_json_value(&s).expect("parsing should succeed");
        assert_eq!(original, parsed);
    }

    // --- Document store tests ---

    #[test]
    fn test_document_store_open_close() {
        let mut store = DocumentStore::new();
        assert!(store.is_empty());
        store.open_document("file:///test.lean", 1, "def x := 1");
        assert_eq!(store.len(), 1);
        assert!(store.get_document("file:///test.lean").is_some());
        store.close_document("file:///test.lean");
        assert!(store.is_empty());
    }

    #[test]
    fn test_document_store_update() {
        let mut store = DocumentStore::new();
        store.open_document("file:///a.lean", 1, "original");
        assert!(store.update_document("file:///a.lean", 2, "updated"));
        let doc = store
            .get_document("file:///a.lean")
            .expect("test operation should succeed");
        assert_eq!(doc.content, "updated");
        assert_eq!(doc.version, 2);
    }

    #[test]
    fn test_document_store_update_nonexistent() {
        let mut store = DocumentStore::new();
        assert!(!store.update_document("file:///nope.lean", 1, "text"));
    }

    // --- Position/offset conversion tests ---

    #[test]
    fn test_position_to_offset() {
        let doc = Document::new("test", 1, "hello\nworld\nfoo");
        assert_eq!(doc.position_to_offset(&Position::new(0, 0)), Some(0));
        assert_eq!(doc.position_to_offset(&Position::new(0, 5)), Some(5));
        assert_eq!(doc.position_to_offset(&Position::new(1, 0)), Some(6));
        assert_eq!(doc.position_to_offset(&Position::new(1, 3)), Some(9));
        assert_eq!(doc.position_to_offset(&Position::new(2, 0)), Some(12));
    }

    #[test]
    fn test_offset_to_position() {
        let doc = Document::new("test", 1, "hello\nworld\nfoo");
        assert_eq!(doc.offset_to_position(0), Position::new(0, 0));
        assert_eq!(doc.offset_to_position(5), Position::new(0, 5));
        assert_eq!(doc.offset_to_position(6), Position::new(1, 0));
        assert_eq!(doc.offset_to_position(9), Position::new(1, 3));
        assert_eq!(doc.offset_to_position(12), Position::new(2, 0));
    }

    #[test]
    fn test_get_line() {
        let doc = Document::new("test", 1, "line one\nline two\nline three");
        assert_eq!(doc.get_line(0), Some("line one"));
        assert_eq!(doc.get_line(1), Some("line two"));
        assert_eq!(doc.get_line(2), Some("line three"));
        assert_eq!(doc.get_line(3), None);
    }

    #[test]
    fn test_line_count() {
        let doc = Document::new("test", 1, "a\nb\nc\n");
        assert_eq!(doc.line_count(), 4); // 4 lines: "a", "b", "c", ""
    }

    #[test]
    fn test_word_at_position() {
        let doc = Document::new("test", 1, "def hello_world : Nat := 42");
        let (word, _) = doc
            .word_at_position(&Position::new(0, 5))
            .expect("test operation should succeed");
        assert_eq!(word, "hello_world");
    }

    #[test]
    fn test_word_at_position_no_word() {
        let doc = Document::new("test", 1, "   ");
        assert!(doc.word_at_position(&Position::new(0, 1)).is_none());
    }

    // --- Server initialization test ---

    #[test]
    fn test_server_initialize() {
        let mut server = LspServer::new();
        assert!(!server.initialized);
        let result = server.handle_initialize(&JsonValue::Object(Vec::new()));
        assert!(server.initialized);
        // Check that capabilities are returned
        assert!(result.get("capabilities").is_some());
        assert!(result.get("serverInfo").is_some());
    }

    #[test]
    fn test_server_shutdown() {
        let mut server = LspServer::new();
        server.initialized = true;
        let result = server.handle_shutdown();
        assert!(server.shutdown_requested);
        assert_eq!(result, JsonValue::Null);
    }

    // --- Message dispatch tests ---

    #[test]
    fn test_handle_message_initialize() {
        let mut server = LspServer::new();
        let msg = JsonRpcMessage::request(
            JsonValue::Number(1.0),
            "initialize",
            JsonValue::Object(Vec::new()),
        );
        let resp = server
            .handle_message(&msg)
            .expect("test operation should succeed");
        assert!(resp.result.is_some());
        assert!(resp.error.is_none());
    }

    #[test]
    fn test_handle_message_unknown_method() {
        let mut server = LspServer::new();
        let msg =
            JsonRpcMessage::request(JsonValue::Number(1.0), "unknown/method", JsonValue::Null);
        let resp = server
            .handle_message(&msg)
            .expect("test operation should succeed");
        assert!(resp.error.is_some());
        assert_eq!(
            resp.error.expect("test operation should succeed").code,
            METHOD_NOT_FOUND
        );
    }

    #[test]
    fn test_handle_did_open_and_completion() {
        let mut server = LspServer::new();
        server.initialized = true;

        // Open a document
        let open_params = JsonValue::Object(vec![(
            "textDocument".to_string(),
            JsonValue::Object(vec![
                (
                    "uri".to_string(),
                    JsonValue::String("file:///test.lean".to_string()),
                ),
                (
                    "languageId".to_string(),
                    JsonValue::String("lean4".to_string()),
                ),
                ("version".to_string(), JsonValue::Number(1.0)),
                (
                    "text".to_string(),
                    JsonValue::String("def foo := 1\n".to_string()),
                ),
            ]),
        )]);
        server.handle_text_document_did_open(&open_params);
        assert!(server
            .document_store
            .get_document("file:///test.lean")
            .is_some());

        // Request completions
        let comp_params = JsonValue::Object(vec![
            (
                "textDocument".to_string(),
                JsonValue::Object(vec![(
                    "uri".to_string(),
                    JsonValue::String("file:///test.lean".to_string()),
                )]),
            ),
            (
                "position".to_string(),
                JsonValue::Object(vec![
                    ("line".to_string(), JsonValue::Number(1.0)),
                    ("character".to_string(), JsonValue::Number(0.0)),
                ]),
            ),
        ]);
        let result = server.handle_completion(&comp_params);
        // Should have keyword completions at minimum
        assert!(
            result
                .as_array()
                .expect("type conversion should succeed")
                .len()
                > 5
        );
    }

    // --- Completion tests ---

    #[test]
    fn test_completions_keywords() {
        let server = LspServer::new();
        let completions = server.get_completions_at("file:///test.lean", &Position::new(0, 0));
        let labels: Vec<&str> = completions.iter().map(|c| c.label.as_str()).collect();
        assert!(labels.contains(&"def"));
        assert!(labels.contains(&"theorem"));
        assert!(labels.contains(&"axiom"));
    }

    #[test]
    fn test_completions_prefix_filter() {
        let mut server = LspServer::new();
        server
            .document_store
            .open_document("file:///t.lean", 1, "th");
        let completions = server.get_completions_at("file:///t.lean", &Position::new(0, 2));
        let labels: Vec<&str> = completions.iter().map(|c| c.label.as_str()).collect();
        assert!(labels.contains(&"theorem"));
        assert!(labels.contains(&"then"));
        assert!(!labels.contains(&"def"));
    }

    // --- Diagnostic tests ---

    #[test]
    fn test_validate_empty_document() {
        let mut server = LspServer::new();
        server
            .document_store
            .open_document("file:///empty.lean", 1, "");
        let diags = server.validate_document("file:///empty.lean");
        assert!(diags.is_empty());
    }

    #[test]
    fn test_validate_valid_tokens() {
        let mut server = LspServer::new();
        server
            .document_store
            .open_document("file:///good.lean", 1, "def foo : Nat := 1");
        let diags = server.validate_document("file:///good.lean");
        // Simple valid tokens should produce no diagnostics
        assert!(diags.is_empty());
    }

    // --- Symbol extraction tests ---

    #[test]
    fn test_extract_symbols() {
        let content = "def foo := 1\ntheorem bar : Prop := True\naxiom baz : Nat";
        let env = Environment::new();
        let result = analyze_document("test", content, &env);
        let names: Vec<&str> = result.symbols.iter().map(|s| s.name.as_str()).collect();
        assert!(names.contains(&"foo"));
        assert!(names.contains(&"bar"));
        assert!(names.contains(&"baz"));
    }

    #[test]
    fn test_extract_definitions() {
        let content = "def hello := 42\ninductive MyType where\n  | ctor";
        let env = Environment::new();
        let result = analyze_document("test", content, &env);
        assert!(result.definitions.len() >= 2);
        let def_names: Vec<&str> = result.definitions.iter().map(|d| d.name.as_str()).collect();
        assert!(def_names.contains(&"hello"));
        assert!(def_names.contains(&"MyType"));
    }

    // --- Hover tests ---

    #[test]
    fn test_hover_keyword() {
        let info = get_keyword_hover("def");
        assert!(info.is_some());
        assert!(info.expect("test operation should succeed").contains("def"));
    }

    #[test]
    fn test_hover_unknown() {
        let info = get_keyword_hover("not_a_keyword_xyz");
        assert!(info.is_none());
    }

    #[test]
    fn test_hover_in_document() {
        let mut server = LspServer::new();
        server
            .document_store
            .open_document("file:///h.lean", 1, "def myFunc := 1");
        let hover = server.get_hover_info_at("file:///h.lean", &Position::new(0, 0));
        // "def" is a keyword
        assert!(hover.is_some());
    }

    // --- Find references test ---

    #[test]
    fn test_find_references() {
        let content = "def foo := 1\ndef bar := foo";
        let refs = find_references_in_document("test", content, "foo");
        assert_eq!(refs.len(), 2); // declaration name + usage
    }

    // --- Document formatting test ---

    #[test]
    fn test_format_trailing_whitespace() {
        let edits = format_document("def x := 1   \ndef y := 2  ");
        assert!(!edits.is_empty());
        // Should have edits for trailing whitespace + missing newline
    }

    // --- JsonRpcMessage tests ---

    #[test]
    fn test_jsonrpc_message_roundtrip() {
        let msg = JsonRpcMessage::request(
            JsonValue::Number(42.0),
            "textDocument/completion",
            JsonValue::Object(Vec::new()),
        );
        let json = msg.to_json();
        let parsed = JsonRpcMessage::from_json(&json).expect("parsing should succeed");
        assert_eq!(
            parsed.id.expect("parsing should succeed"),
            JsonValue::Number(42.0)
        );
        assert_eq!(
            parsed.method.expect("parsing should succeed"),
            "textDocument/completion"
        );
    }

    #[test]
    fn test_jsonrpc_error_roundtrip() {
        let err = JsonRpcError::method_not_found("foo/bar");
        let json = err.to_json();
        let parsed = JsonRpcError::from_json(&json).expect("parsing should succeed");
        assert_eq!(parsed.code, METHOD_NOT_FOUND);
        assert!(parsed.message.contains("foo/bar"));
    }

    // --- LSP type serialization tests ---

    #[test]
    fn test_position_json_roundtrip() {
        let pos = Position::new(10, 25);
        let json = pos.to_json();
        let parsed = Position::from_json(&json).expect("parsing should succeed");
        assert_eq!(parsed, pos);
    }

    #[test]
    fn test_range_json_roundtrip() {
        let range = Range::new(Position::new(1, 0), Position::new(1, 10));
        let json = range.to_json();
        let parsed = Range::from_json(&json).expect("parsing should succeed");
        assert_eq!(parsed, range);
    }

    #[test]
    fn test_diagnostic_to_json() {
        let diag = Diagnostic::error(Range::single_line(0, 0, 5), "test error");
        let json = diag.to_json();
        assert_eq!(
            json.get("message").expect("key should exist").as_str(),
            Some("test error")
        );
        assert_eq!(
            json.get("severity").expect("key should exist").as_i64(),
            Some(1)
        );
    }

    #[test]
    fn test_completion_item_to_json() {
        let item = CompletionItem::keyword("theorem");
        let json = item.to_json();
        assert_eq!(
            json.get("label").expect("key should exist").as_str(),
            Some("theorem")
        );
        assert_eq!(
            json.get("kind").expect("key should exist").as_i64(),
            Some(CompletionItemKind::Keyword as i64)
        );
    }

    // --- Analysis cache tests ---

    #[test]
    fn test_analysis_cache() {
        let mut cache = AnalysisCache::new();
        assert!(cache.get("file:///test.lean", 1).is_none());
        cache.store("file:///test.lean", 1, AnalysisResult::default());
        assert!(cache.get("file:///test.lean", 1).is_some());
        assert!(cache.get("file:///test.lean", 2).is_none()); // wrong version
        cache.invalidate("file:///test.lean");
        assert!(cache.get("file:///test.lean", 1).is_none());
    }

    #[test]
    fn test_analysis_cache_clear() {
        let mut cache = AnalysisCache::new();
        cache.store("a", 1, AnalysisResult::default());
        cache.store("b", 1, AnalysisResult::default());
        cache.clear();
        assert!(cache.get("a", 1).is_none());
        assert!(cache.get("b", 1).is_none());
    }

    // --- Server capabilities tests ---

    #[test]
    fn test_server_capabilities_json() {
        let caps = ServerCapabilities::oxilean_defaults();
        let json = caps.to_json();
        assert!(json.get("textDocumentSync").is_some());
        assert!(json.get("completionProvider").is_some());
        assert!(json.get("hoverProvider").is_some());
        assert!(json.get("definitionProvider").is_some());
    }

    #[test]
    fn test_initialize_result_json() {
        let result = InitializeResult {
            capabilities: ServerCapabilities::oxilean_defaults(),
            server_name: "test".to_string(),
            server_version: "1.0.0".to_string(),
        };
        let json = result.to_json();
        let info = json.get("serverInfo").expect("key should exist");
        assert_eq!(
            info.get("name").expect("key should exist").as_str(),
            Some("test")
        );
    }

    // --- Document symbol test ---

    #[test]
    fn test_document_symbol_json() {
        let sym = DocumentSymbol {
            name: "foo".to_string(),
            detail: Some("def foo".to_string()),
            kind: SymbolKind::Function,
            range: Range::single_line(0, 0, 10),
            selection_range: Range::single_line(0, 4, 7),
            children: Vec::new(),
        };
        let json = sym.to_json();
        assert_eq!(
            json.get("name").expect("key should exist").as_str(),
            Some("foo")
        );
    }

    // --- Integration-style tests ---

    #[test]
    fn test_full_lsp_session() {
        let mut server = LspServer::new();

        // Initialize
        let init_msg = JsonRpcMessage::request(
            JsonValue::Number(1.0),
            "initialize",
            JsonValue::Object(Vec::new()),
        );
        let resp = server
            .handle_message(&init_msg)
            .expect("test operation should succeed");
        assert!(resp.result.is_some());

        // Open document
        let open_params = JsonValue::Object(vec![(
            "textDocument".to_string(),
            JsonValue::Object(vec![
                (
                    "uri".to_string(),
                    JsonValue::String("file:///session.lean".to_string()),
                ),
                (
                    "text".to_string(),
                    JsonValue::String("def add (a b : Nat) : Nat := a".to_string()),
                ),
                ("version".to_string(), JsonValue::Number(1.0)),
            ]),
        )]);
        let open_msg = JsonRpcMessage::notification("textDocument/didOpen", open_params);
        assert!(server.handle_message(&open_msg).is_none());

        // Document symbol
        let sym_params = JsonValue::Object(vec![(
            "textDocument".to_string(),
            JsonValue::Object(vec![(
                "uri".to_string(),
                JsonValue::String("file:///session.lean".to_string()),
            )]),
        )]);
        let sym_msg = JsonRpcMessage::request(
            JsonValue::Number(2.0),
            "textDocument/documentSymbol",
            sym_params,
        );
        let sym_resp = server
            .handle_message(&sym_msg)
            .expect("test operation should succeed");
        let symbols = sym_resp.result.expect("test operation should succeed");
        assert!(!symbols
            .as_array()
            .expect("type conversion should succeed")
            .is_empty());

        // Shutdown
        let shutdown_msg =
            JsonRpcMessage::request(JsonValue::Number(3.0), "shutdown", JsonValue::Null);
        let shutdown_resp = server
            .handle_message(&shutdown_msg)
            .expect("test operation should succeed");
        assert_eq!(shutdown_resp.result, Some(JsonValue::Null));
        assert!(server.shutdown_requested);
    }
}
