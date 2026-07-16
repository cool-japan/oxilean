//! A minimal, hand-rolled JSON parser (no `serde`).
//!
//! This parser is part of the trusted computing base and parses **untrusted**
//! input, so it is written to be robust under fuzzing:
//!
//! * **No recursion.** Nesting is handled with an explicit work stack, so a
//!   pathologically deep document cannot overflow the native call stack. A hard
//!   [`MAX_DEPTH`] bound rejects documents nested past a sane limit *before* any
//!   unbounded work happens.
//! * **No panics.** Every failure path returns a [`JsonError`] with a precise
//!   byte offset (relative to the start of the parsed slice).
//! * **Full string support.** All JSON escapes are decoded, including `\uXXXX`
//!   with UTF-16 surrogate pairs.
//!
//! The value model is deliberately small: objects, arrays, strings, integers
//! (kept as their decimal text so arbitrary-precision `natVal` payloads survive
//! losslessly), floats (only where the spec needs a number that is not an
//! index), booleans and null.

use std::collections::BTreeMap;

/// Maximum nesting depth accepted by the parser. lean4export objects are shallow
/// (a handful of levels), so this is a generous bound that still stops
/// nesting-based denial-of-service.
pub const MAX_DEPTH: usize = 128;

/// An error produced while parsing a single JSON value.
///
/// `offset` is the 0-based byte offset within the slice handed to the parser.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JsonError {
    /// 0-based byte offset within the parsed slice where the error was found.
    pub offset: usize,
    /// Human-readable description.
    pub message: String,
}

impl JsonError {
    fn new(offset: usize, message: impl Into<String>) -> Self {
        Self {
            offset,
            message: message.into(),
        }
    }
}

/// A parsed JSON value.
///
/// Numbers are split so that integers keep their exact decimal text — the
/// export format encodes indices as JSON integers and `natVal` as a decimal
/// *string*, but keeping integer text avoids any `u64`/`f64` round-trip even for
/// index fields.
#[derive(Debug, Clone, PartialEq)]
pub enum JsonValue {
    /// JSON `null`.
    Null,
    /// JSON boolean.
    Bool(bool),
    /// A JSON integer, preserved as its exact decimal text (may be negative).
    Int(String),
    /// A JSON floating-point number (only produced when a `.`, `e`, or `E`
    /// appears; the spec does not use these for indices).
    Float(f64),
    /// A JSON string with all escapes decoded.
    Str(String),
    /// A JSON array.
    Array(Vec<JsonValue>),
    /// A JSON object. Keys are unique (a duplicate key is a parse error).
    Object(BTreeMap<String, JsonValue>),
}

impl JsonValue {
    /// Borrow the object map, if this is an object.
    #[must_use]
    pub fn as_object(&self) -> Option<&BTreeMap<String, JsonValue>> {
        match self {
            JsonValue::Object(m) => Some(m),
            _ => None,
        }
    }

    /// Borrow the array, if this is an array.
    #[must_use]
    pub fn as_array(&self) -> Option<&[JsonValue]> {
        match self {
            JsonValue::Array(a) => Some(a),
            _ => None,
        }
    }

    /// Borrow the string, if this is a string.
    #[must_use]
    pub fn as_str(&self) -> Option<&str> {
        match self {
            JsonValue::Str(s) => Some(s),
            _ => None,
        }
    }

    /// Borrow the exact integer text, if this is an integer.
    #[must_use]
    pub fn as_int_str(&self) -> Option<&str> {
        match self {
            JsonValue::Int(s) => Some(s),
            _ => None,
        }
    }

    /// The boolean value, if this is a boolean.
    #[must_use]
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            JsonValue::Bool(b) => Some(*b),
            _ => None,
        }
    }
}

/// Parse a single JSON value from `bytes`, requiring that the whole slice is
/// consumed (only trailing ASCII whitespace is allowed).
///
/// # Errors
/// Returns a [`JsonError`] for any malformed input, including trailing
/// non-whitespace bytes and nesting deeper than [`MAX_DEPTH`].
pub fn parse(bytes: &[u8]) -> Result<JsonValue, JsonError> {
    let mut p = Parser { bytes, pos: 0 };
    p.skip_ws();
    let value = p.parse_value()?;
    p.skip_ws();
    if p.pos != p.bytes.len() {
        return Err(JsonError::new(p.pos, "trailing bytes after JSON value"));
    }
    Ok(value)
}

/// A frame on the explicit work stack while building composite values.
enum Frame {
    /// Building an array; holds elements collected so far.
    Array(Vec<JsonValue>),
    /// Building an object; holds pairs collected so far and the pending key.
    Object(BTreeMap<String, JsonValue>, String),
}

struct Parser<'a> {
    bytes: &'a [u8],
    pos: usize,
}

impl<'a> Parser<'a> {
    #[inline]
    fn peek(&self) -> Option<u8> {
        self.bytes.get(self.pos).copied()
    }

    #[inline]
    fn bump(&mut self) -> Option<u8> {
        let b = self.bytes.get(self.pos).copied();
        if b.is_some() {
            self.pos += 1;
        }
        b
    }

    fn skip_ws(&mut self) {
        while let Some(b) = self.peek() {
            match b {
                b' ' | b'\t' | b'\n' | b'\r' => self.pos += 1,
                _ => break,
            }
        }
    }

    fn err<T>(&self, msg: impl Into<String>) -> Result<T, JsonError> {
        Err(JsonError::new(self.pos, msg))
    }

    /// Parse one value. Composite values (arrays/objects) are driven by an
    /// explicit stack so recursion never touches the native call stack.
    fn parse_value(&mut self) -> Result<JsonValue, JsonError> {
        let mut stack: Vec<Frame> = Vec::new();

        // The state machine has two phases:
        //   - "read a fresh value" (start of array element, object value, or top)
        //   - after producing a value, "attach it to the current container".
        loop {
            self.skip_ws();
            // Enforce depth on entry so we never even allocate past the bound.
            if stack.len() > MAX_DEPTH {
                return self.err(format!("nesting deeper than MAX_DEPTH ({MAX_DEPTH})"));
            }

            let first = match self.peek() {
                Some(b) => b,
                None => return self.err("unexpected end of input while expecting a value"),
            };

            // Produce the next scalar, or open a new container.
            let produced: JsonValue = match first {
                b'{' => {
                    self.pos += 1;
                    self.skip_ws();
                    if self.peek() == Some(b'}') {
                        self.pos += 1;
                        JsonValue::Object(BTreeMap::new())
                    } else {
                        let key = self.parse_object_key()?;
                        stack.push(Frame::Object(BTreeMap::new(), key));
                        continue;
                    }
                }
                b'[' => {
                    self.pos += 1;
                    self.skip_ws();
                    if self.peek() == Some(b']') {
                        self.pos += 1;
                        JsonValue::Array(Vec::new())
                    } else {
                        stack.push(Frame::Array(Vec::new()));
                        continue;
                    }
                }
                b'"' => JsonValue::Str(self.parse_string()?),
                b't' | b'f' => self.parse_bool()?,
                b'n' => self.parse_null()?,
                b'-' | b'0'..=b'9' => self.parse_number()?,
                other => {
                    return self.err(format!("unexpected byte 0x{other:02x} at value start"));
                }
            };

            // Attach `produced` to the enclosing container, unwinding completed
            // containers as we go.
            let mut value = produced;
            loop {
                match stack.last_mut() {
                    None => return Ok(value),
                    Some(Frame::Array(items)) => {
                        items.push(value);
                        self.skip_ws();
                        match self.bump() {
                            Some(b',') => break, // read next element
                            Some(b']') => {
                                // Close this array and continue attaching upward.
                                let Some(Frame::Array(items)) = stack.pop() else {
                                    return self.err("internal: array frame vanished");
                                };
                                value = JsonValue::Array(items);
                                continue;
                            }
                            Some(other) => {
                                return self
                                    .err(format!("expected ',' or ']', found 0x{other:02x}"));
                            }
                            None => return self.err("unexpected end of input inside array"),
                        }
                    }
                    Some(Frame::Object(map, key)) => {
                        let key = core::mem::take(key);
                        if map.insert(key.clone(), value).is_some() {
                            return self.err(format!("duplicate object key {key:?}"));
                        }
                        self.skip_ws();
                        match self.bump() {
                            Some(b',') => {
                                // Read next key before the next value.
                                self.skip_ws();
                                let next_key = self.parse_object_key()?;
                                if let Some(Frame::Object(_, slot)) = stack.last_mut() {
                                    *slot = next_key;
                                }
                                break; // read the value for next_key
                            }
                            Some(b'}') => {
                                let Some(Frame::Object(map, _)) = stack.pop() else {
                                    return self.err("internal: object frame vanished");
                                };
                                value = JsonValue::Object(map);
                                continue;
                            }
                            Some(other) => {
                                return self
                                    .err(format!("expected ',' or '}}', found 0x{other:02x}"));
                            }
                            None => return self.err("unexpected end of input inside object"),
                        }
                    }
                }
            }
        }
    }

    /// Parse an object key: a string followed by `:`.
    fn parse_object_key(&mut self) -> Result<String, JsonError> {
        self.skip_ws();
        if self.peek() != Some(b'"') {
            return self.err("expected string key in object");
        }
        let key = self.parse_string()?;
        self.skip_ws();
        if self.bump() != Some(b':') {
            return self.err("expected ':' after object key");
        }
        Ok(key)
    }

    fn parse_bool(&mut self) -> Result<JsonValue, JsonError> {
        if self.bytes[self.pos..].starts_with(b"true") {
            self.pos += 4;
            Ok(JsonValue::Bool(true))
        } else if self.bytes[self.pos..].starts_with(b"false") {
            self.pos += 5;
            Ok(JsonValue::Bool(false))
        } else {
            self.err("invalid literal (expected true/false)")
        }
    }

    fn parse_null(&mut self) -> Result<JsonValue, JsonError> {
        if self.bytes[self.pos..].starts_with(b"null") {
            self.pos += 4;
            Ok(JsonValue::Null)
        } else {
            self.err("invalid literal (expected null)")
        }
    }

    /// Parse a JSON number. Integers keep exact text; only a fractional or
    /// exponent part promotes to a float.
    fn parse_number(&mut self) -> Result<JsonValue, JsonError> {
        let start = self.pos;
        let mut is_float = false;

        if self.peek() == Some(b'-') {
            self.pos += 1;
        }
        // Integer part.
        match self.peek() {
            Some(b'0') => {
                self.pos += 1;
                // JSON forbids leading zeros like 01; a lone 0 is fine.
            }
            Some(b'1'..=b'9') => {
                while matches!(self.peek(), Some(b'0'..=b'9')) {
                    self.pos += 1;
                }
            }
            _ => return self.err("invalid number: missing integer digits"),
        }
        // Fraction.
        if self.peek() == Some(b'.') {
            is_float = true;
            self.pos += 1;
            if !matches!(self.peek(), Some(b'0'..=b'9')) {
                return self.err("invalid number: missing fraction digits");
            }
            while matches!(self.peek(), Some(b'0'..=b'9')) {
                self.pos += 1;
            }
        }
        // Exponent.
        if matches!(self.peek(), Some(b'e' | b'E')) {
            is_float = true;
            self.pos += 1;
            if matches!(self.peek(), Some(b'+' | b'-')) {
                self.pos += 1;
            }
            if !matches!(self.peek(), Some(b'0'..=b'9')) {
                return self.err("invalid number: missing exponent digits");
            }
            while matches!(self.peek(), Some(b'0'..=b'9')) {
                self.pos += 1;
            }
        }

        let text = &self.bytes[start..self.pos];
        // Safe: the digits/sign/exponent bytes are all ASCII.
        let s = match core::str::from_utf8(text) {
            Ok(s) => s,
            Err(_) => return Err(JsonError::new(start, "invalid number encoding")),
        };
        if is_float {
            match s.parse::<f64>() {
                Ok(f) => Ok(JsonValue::Float(f)),
                Err(_) => Err(JsonError::new(start, "invalid float")),
            }
        } else {
            Ok(JsonValue::Int(s.to_string()))
        }
    }

    /// Parse a JSON string starting at the opening quote. Decodes all escapes,
    /// including `\uXXXX` surrogate pairs.
    fn parse_string(&mut self) -> Result<String, JsonError> {
        // Consume opening quote.
        if self.bump() != Some(b'"') {
            return self.err("expected opening '\"'");
        }
        let mut out = String::new();
        loop {
            let b = match self.bump() {
                Some(b) => b,
                None => return self.err("unterminated string"),
            };
            match b {
                b'"' => return Ok(out),
                b'\\' => self.parse_escape(&mut out)?,
                // Control characters must be escaped in JSON.
                0x00..=0x1f => {
                    return Err(JsonError::new(
                        self.pos - 1,
                        "unescaped control character in string",
                    ));
                }
                // ASCII (single byte).
                0x20..=0x7f => out.push(b as char),
                // A UTF-8 lead byte: gather the continuation bytes and validate.
                lead => {
                    let len = utf8_len(lead);
                    if len == 0 {
                        return Err(JsonError::new(self.pos - 1, "invalid UTF-8 lead byte"));
                    }
                    let start = self.pos - 1;
                    let end = start + len;
                    if end > self.bytes.len() {
                        return Err(JsonError::new(start, "truncated UTF-8 sequence"));
                    }
                    match core::str::from_utf8(&self.bytes[start..end]) {
                        Ok(s) => {
                            out.push_str(s);
                            self.pos = end;
                        }
                        Err(_) => return Err(JsonError::new(start, "invalid UTF-8 sequence")),
                    }
                }
            }
        }
    }

    /// Parse the body of an escape sequence (the byte after `\`).
    fn parse_escape(&mut self, out: &mut String) -> Result<(), JsonError> {
        let e = match self.bump() {
            Some(e) => e,
            None => return self.err("unterminated escape sequence"),
        };
        match e {
            b'"' => out.push('"'),
            b'\\' => out.push('\\'),
            b'/' => out.push('/'),
            b'b' => out.push('\u{0008}'),
            b'f' => out.push('\u{000c}'),
            b'n' => out.push('\n'),
            b'r' => out.push('\r'),
            b't' => out.push('\t'),
            b'u' => {
                let hi = self.parse_hex4()?;
                if (0xd800..=0xdbff).contains(&hi) {
                    // High surrogate: must be followed by \uXXXX low surrogate.
                    if self.bump() != Some(b'\\') || self.bump() != Some(b'u') {
                        return Err(JsonError::new(
                            self.pos,
                            "expected low surrogate after high surrogate",
                        ));
                    }
                    let lo = self.parse_hex4()?;
                    if !(0xdc00..=0xdfff).contains(&lo) {
                        return Err(JsonError::new(self.pos, "invalid low surrogate"));
                    }
                    let c = 0x1_0000 + ((u32::from(hi) - 0xd800) << 10) + (u32::from(lo) - 0xdc00);
                    match char::from_u32(c) {
                        Some(ch) => out.push(ch),
                        None => {
                            return Err(JsonError::new(self.pos, "invalid surrogate pair"));
                        }
                    }
                } else if (0xdc00..=0xdfff).contains(&hi) {
                    return Err(JsonError::new(self.pos, "unexpected lone low surrogate"));
                } else {
                    match char::from_u32(u32::from(hi)) {
                        Some(ch) => out.push(ch),
                        None => {
                            return Err(JsonError::new(self.pos, "invalid \\u escape"));
                        }
                    }
                }
            }
            other => {
                return Err(JsonError::new(
                    self.pos - 1,
                    format!("invalid escape '\\{}'", other as char),
                ));
            }
        }
        Ok(())
    }

    /// Parse exactly four hex digits into a `u16`.
    fn parse_hex4(&mut self) -> Result<u16, JsonError> {
        let start = self.pos;
        let mut acc: u16 = 0;
        for _ in 0..4 {
            let d = match self.bump() {
                Some(d) => d,
                None => return Err(JsonError::new(start, "truncated \\u escape")),
            };
            let v = match d {
                b'0'..=b'9' => d - b'0',
                b'a'..=b'f' => d - b'a' + 10,
                b'A'..=b'F' => d - b'A' + 10,
                _ => {
                    return Err(JsonError::new(
                        self.pos - 1,
                        "invalid hex digit in \\u escape",
                    ))
                }
            };
            acc = acc * 16 + u16::from(v);
        }
        Ok(acc)
    }
}

/// Expected total byte length of a UTF-8 sequence from its lead byte, or 0 for
/// an invalid lead byte.
#[inline]
fn utf8_len(lead: u8) -> usize {
    match lead {
        0x00..=0x7f => 1,
        0xc0..=0xdf => 2,
        0xe0..=0xef => 3,
        0xf0..=0xf7 => 4,
        _ => 0,
    }
}
