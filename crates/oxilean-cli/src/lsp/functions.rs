//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{Environment, Name};
use oxilean_parse::{Lexer, TokenKind};

use super::types::{
    AnalysisResult, DefinitionInfo, Diagnostic, DocumentSymbol, JsonValue, Location, Range,
    SymbolKind, TextEdit,
};

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
pub(super) fn escape_json_string(s: &str) -> String {
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
pub(super) fn skip_whitespace(input: &[u8], mut pos: usize) -> usize {
    while pos < input.len() {
        match input[pos] {
            b' ' | b'\t' | b'\n' | b'\r' => pos += 1,
            _ => break,
        }
    }
    pos
}
pub(super) fn parse_value(input: &[u8], pos: usize) -> Result<(JsonValue, usize), String> {
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
pub(super) fn parse_null(input: &[u8], pos: usize) -> Result<(JsonValue, usize), String> {
    if input.len() >= pos + 4 && &input[pos..pos + 4] == b"null" {
        Ok((JsonValue::Null, pos + 4))
    } else {
        Err(format!("expected 'null' at position {}", pos))
    }
}
pub(super) fn parse_bool(input: &[u8], pos: usize) -> Result<(JsonValue, usize), String> {
    if input.len() >= pos + 4 && &input[pos..pos + 4] == b"true" {
        Ok((JsonValue::Bool(true), pos + 4))
    } else if input.len() >= pos + 5 && &input[pos..pos + 5] == b"false" {
        Ok((JsonValue::Bool(false), pos + 5))
    } else {
        Err(format!("expected boolean at position {}", pos))
    }
}
pub(super) fn parse_string(input: &[u8], pos: usize) -> Result<(JsonValue, usize), String> {
    let (s, end) = parse_raw_string(input, pos)?;
    Ok((JsonValue::String(s), end))
}
pub(super) fn parse_raw_string(input: &[u8], pos: usize) -> Result<(String, usize), String> {
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
pub(super) fn parse_number(input: &[u8], pos: usize) -> Result<(JsonValue, usize), String> {
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
pub(super) fn parse_array(input: &[u8], pos: usize) -> Result<(JsonValue, usize), String> {
    let mut i = pos + 1;
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
pub(super) fn parse_object(input: &[u8], pos: usize) -> Result<(JsonValue, usize), String> {
    let mut i = pos + 1;
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
pub(super) fn is_ident_char(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_' || b == b'.' || b == b'\''
}
/// Compute the byte offsets of each line start.
pub(super) fn compute_line_offsets(text: &str) -> Vec<usize> {
    let mut offsets = vec![0];
    for (i, byte) in text.bytes().enumerate() {
        if byte == b'\n' {
            offsets.push(i + 1);
        }
    }
    offsets
}
/// Analyze a document using the lexer and parser.
pub fn analyze_document(uri: &str, content: &str, env: &Environment) -> AnalysisResult {
    let mut result = AnalysisResult::default();
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
    extract_symbols_from_tokens(uri, &tokens, &mut result);
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
pub(super) fn extract_symbols_from_tokens(
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
/// Create a code action JSON value.
pub(super) fn make_code_action(title: &str, _uri: &str, edits: Vec<TextEdit>) -> JsonValue {
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
pub(super) fn get_keyword_hover(word: &str) -> Option<String> {
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
        "match" => {
            "**match** -- Pattern matching.\n\n```lean\nmatch x with\n  | pattern => result\n```"
        }
        "let" => "**let** -- Local binding.\n\n```lean\nlet x := value\nin body\n```",
        "if" => {
            "**if** -- Conditional expression.\n\n```lean\nif cond then a else b\n```"
        }
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
pub(super) fn format_document(content: &str) -> Vec<TextEdit> {
    let mut edits = Vec::new();
    let lines: Vec<&str> = content.lines().collect();
    for (i, line) in lines.iter().enumerate() {
        let trimmed_end = line.trim_end();
        if trimmed_end.len() != line.len() {
            edits.push(TextEdit::new(
                Range::single_line(i as u32, trimmed_end.len() as u32, line.len() as u32),
                "",
            ));
        }
    }
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
