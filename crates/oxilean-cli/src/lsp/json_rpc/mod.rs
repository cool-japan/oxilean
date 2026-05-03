//! JSON-RPC 2.0 protocol types and parsing for the LSP server.

use std::fmt;

// ── Error code constants ──────────────────────────────────────────────────────

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

// ── JsonValue ─────────────────────────────────────────────────────────────────

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

// ── Serialization ─────────────────────────────────────────────────────────────

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

// ── Parsing ───────────────────────────────────────────────────────────────────

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

pub(crate) fn parse_raw_string(input: &[u8], pos: usize) -> Result<(String, usize), String> {
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

// ── JsonRpcMessage ────────────────────────────────────────────────────────────

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

// ── JsonRpcError ──────────────────────────────────────────────────────────────

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
