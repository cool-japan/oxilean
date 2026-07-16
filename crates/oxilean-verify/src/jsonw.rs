//! A tiny, hand-rolled JSON *writer*.
//!
//! The verify product does not depend on `serde` — the whole point is a small,
//! audited dependency closure. The report has a fixed, documented shape (see
//! `docs/VERIFY.md`), so we emit it directly with an explicit, stable field
//! order rather than reflecting over a struct. The writer is deterministic: the
//! same inputs always produce byte-identical output, which the CLI relies on for
//! its `--json` determinism guarantee.
//!
//! Output is pretty-printed with two-space indentation and `\n` newlines. Only
//! the JSON value types the report needs are supported: objects, arrays,
//! strings, integers, and `null`. Floating-point is deliberately avoided —
//! durations are emitted as integer milliseconds so reports are exact and
//! platform-independent.

use core::fmt::Write as _;

/// A streaming JSON writer that tracks indentation and produces deterministic,
/// pretty-printed output into an owned `String`.
pub struct JsonWriter {
    buf: String,
    indent: usize,
}

impl Default for JsonWriter {
    fn default() -> Self {
        Self::new()
    }
}

impl JsonWriter {
    /// A fresh writer with an empty buffer.
    #[must_use]
    pub fn new() -> Self {
        Self {
            buf: String::new(),
            indent: 0,
        }
    }

    /// Consume the writer, returning the accumulated JSON text (with a trailing
    /// newline).
    #[must_use]
    pub fn into_string(mut self) -> String {
        self.buf.push('\n');
        self.buf
    }

    /// Borrow the accumulated text so far (used in tests).
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.buf
    }

    fn write_indent(&mut self) {
        for _ in 0..self.indent {
            self.buf.push_str("  ");
        }
    }

    /// Begin an object: writes `{` and increases indentation. The caller emits
    /// members with [`ObjectScope`].
    pub fn object(&mut self) -> ObjectScope<'_> {
        self.buf.push('{');
        self.indent += 1;
        ObjectScope { w: self, count: 0 }
    }

    /// Begin an array: writes `[` and increases indentation.
    pub fn array(&mut self) -> ArrayScope<'_> {
        self.buf.push('[');
        self.indent += 1;
        ArrayScope { w: self, count: 0 }
    }

    /// Emit a quoted, escaped JSON string value at the current position.
    pub fn string_value(&mut self, s: &str) {
        write_json_string(&mut self.buf, s);
    }

    /// Emit an integer value at the current position.
    pub fn int_value(&mut self, n: i64) {
        // `write!` into a String cannot fail; ignore the Result rather than
        // unwrapping (COOLJAPAN: no unwrap in production code).
        let _ = write!(self.buf, "{n}");
    }

    /// Emit an unsigned integer value.
    pub fn uint_value(&mut self, n: u64) {
        let _ = write!(self.buf, "{n}");
    }

    /// Emit a literal `null`.
    pub fn null_value(&mut self) {
        self.buf.push_str("null");
    }

    /// Emit a boolean value.
    pub fn bool_value(&mut self, b: bool) {
        self.buf.push_str(if b { "true" } else { "false" });
    }
}

/// A scope for emitting object members. Dropping it (implicitly, at end of the
/// borrowing block) closes the object with a newline-indented `}`.
pub struct ObjectScope<'a> {
    w: &'a mut JsonWriter,
    count: usize,
}

impl ObjectScope<'_> {
    /// Write the `"key":` prefix for the next member, handling the leading comma
    /// and newline. Returns the underlying writer so the caller can emit the
    /// value (scalar, object, or array).
    pub fn key(&mut self, key: &str) -> &mut JsonWriter {
        if self.count > 0 {
            self.w.buf.push(',');
        }
        self.count += 1;
        self.w.buf.push('\n');
        self.w.write_indent();
        write_json_string(&mut self.w.buf, key);
        self.w.buf.push_str(": ");
        self.w
    }

    /// Convenience: a `"key": "value"` string member.
    pub fn str_field(&mut self, key: &str, value: &str) {
        self.key(key).string_value(value);
    }

    /// Convenience: a `"key": <int>` member.
    pub fn int_field(&mut self, key: &str, value: i64) {
        self.key(key).int_value(value);
    }

    /// Convenience: a `"key": <uint>` member.
    pub fn uint_field(&mut self, key: &str, value: u64) {
        self.key(key).uint_value(value);
    }

    /// Convenience: a `"key": null` member.
    pub fn null_field(&mut self, key: &str) {
        self.key(key).null_value();
    }

    /// Convenience: a `"key": <bool>` member.
    pub fn bool_field(&mut self, key: &str, value: bool) {
        self.key(key).bool_value(value);
    }
}

impl Drop for ObjectScope<'_> {
    fn drop(&mut self) {
        self.w.indent -= 1;
        if self.count > 0 {
            self.w.buf.push('\n');
            self.w.write_indent();
        }
        self.w.buf.push('}');
    }
}

/// A scope for emitting array elements. Dropping it closes the array.
pub struct ArrayScope<'a> {
    w: &'a mut JsonWriter,
    count: usize,
}

impl ArrayScope<'_> {
    /// Begin the next element, handling the leading comma and newline. Returns
    /// the underlying writer for the caller to emit the element value.
    pub fn element(&mut self) -> &mut JsonWriter {
        if self.count > 0 {
            self.w.buf.push(',');
        }
        self.count += 1;
        self.w.buf.push('\n');
        self.w.write_indent();
        self.w
    }
}

impl Drop for ArrayScope<'_> {
    fn drop(&mut self) {
        self.w.indent -= 1;
        if self.count > 0 {
            self.w.buf.push('\n');
            self.w.write_indent();
        }
        self.w.buf.push(']');
    }
}

/// Write a JSON string literal (with surrounding quotes) into `out`, escaping
/// per RFC 8259: `"`, `\`, the C0 control characters, and the shorthand
/// escapes.
pub fn write_json_string(out: &mut String, s: &str) {
    out.push('"');
    for ch in s.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            '\u{08}' => out.push_str("\\b"),
            '\u{0c}' => out.push_str("\\f"),
            c if (c as u32) < 0x20 => {
                let _ = write!(out, "\\u{:04x}", c as u32);
            }
            c => out.push(c),
        }
    }
    out.push('"');
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_object() {
        let mut w = JsonWriter::new();
        {
            let _obj = w.object();
        }
        assert_eq!(w.into_string(), "{}\n");
    }

    #[test]
    fn empty_array() {
        let mut w = JsonWriter::new();
        {
            let _arr = w.array();
        }
        assert_eq!(w.into_string(), "[]\n");
    }

    #[test]
    fn nested_shape_is_stable() {
        let mut w = JsonWriter::new();
        {
            let mut obj = w.object();
            obj.str_field("name", "oxilean-verify");
            obj.uint_field("count", 3);
            {
                let mut arr = obj.key("items").array();
                {
                    let mut inner = arr.element().object();
                    inner.str_field("k", "v");
                }
                arr.element().int_value(42);
            }
        }
        let out = w.into_string();
        assert_eq!(
            out,
            "{\n  \"name\": \"oxilean-verify\",\n  \"count\": 3,\n  \"items\": [\n    {\n      \"k\": \"v\"\n    },\n    42\n  ]\n}\n"
        );
    }

    #[test]
    fn escaping() {
        let mut s = String::new();
        write_json_string(&mut s, "a\"b\\c\nd\te\u{01}f");
        assert_eq!(s, "\"a\\\"b\\\\c\\nd\\te\\u0001f\"");
    }

    #[test]
    fn null_field() {
        let mut w = JsonWriter::new();
        {
            let mut obj = w.object();
            obj.null_field("sha256");
        }
        assert_eq!(w.into_string(), "{\n  \"sha256\": null\n}\n");
    }
}
