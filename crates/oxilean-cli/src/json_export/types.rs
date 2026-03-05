//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use std::collections::HashMap;
use std::io::{self, Write};

#[allow(dead_code)]
pub struct StripNullFields;
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum JsonSchemaKind {
    Object,
    Array,
    String,
    Number,
    Boolean,
    Null,
}
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct ExportStats {
    pub total: usize,
    pub complete: usize,
    pub has_sorry: usize,
    pub incomplete: usize,
    pub axioms: usize,
}
/// Configuration for JSON export.
#[derive(Clone, Debug)]
pub struct JsonExportConfig {
    /// Whether to include source spans.
    pub include_spans: bool,
    /// Whether to include detailed type information.
    pub include_types: bool,
    /// Whether to pretty-print the output.
    pub pretty: bool,
    /// Maximum depth for recursive serialization.
    pub max_depth: usize,
    /// Whether to include proof terms.
    pub include_proofs: bool,
}
/// Builder for constructing JSON objects.
pub struct ObjectBuilder {
    fields: Vec<(String, JsonValue)>,
}
impl ObjectBuilder {
    /// Create a new object builder.
    pub fn new() -> Self {
        Self { fields: Vec::new() }
    }
    /// Add a field.
    pub fn field(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.fields.push((key.into(), value));
        self
    }
    /// Add a string field.
    pub fn str_field(self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.field(key, JsonValue::string(value))
    }
    /// Add an integer field.
    pub fn int_field(self, key: impl Into<String>, value: i64) -> Self {
        self.field(key, JsonValue::int(value))
    }
    /// Add a boolean field.
    pub fn bool_field(self, key: impl Into<String>, value: bool) -> Self {
        self.field(key, JsonValue::bool_val(value))
    }
    /// Add a nullable string field.
    pub fn optional_str(self, key: impl Into<String>, value: Option<&str>) -> Self {
        match value {
            Some(s) => self.field(key, JsonValue::string(s)),
            None => self.field(key, JsonValue::Null),
        }
    }
    /// Build the JSON object.
    pub fn build(self) -> JsonValue {
        JsonValue::Object(self.fields)
    }
}
impl ObjectBuilder {
    /// Add a float field to the object being built.
    pub fn float_field(self, key: &str, value: f64) -> Self {
        self.field(key, JsonValue::Float(value))
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum ProofStatus {
    Complete,
    Incomplete,
    HasSorry,
    Axiom,
}
#[allow(dead_code)]
impl ProofStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Complete => "complete",
            Self::Incomplete => "incomplete",
            Self::HasSorry => "has_sorry",
            Self::Axiom => "axiom",
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct JsonValueStats {
    pub object_count: usize,
    pub array_count: usize,
    pub string_count: usize,
    pub number_count: usize,
    pub bool_count: usize,
    pub null_count: usize,
    pub max_depth: usize,
}
/// Accumulates benchmark/test results into a JSON report.
pub struct ResultCollector {
    entries: Vec<JsonValue>,
    label: String,
}
impl ResultCollector {
    /// Create a new collector with the given label.
    pub fn new(label: impl Into<String>) -> Self {
        ResultCollector {
            entries: Vec::new(),
            label: label.into(),
        }
    }
    /// Record a timing result entry.
    pub fn record_timing(&mut self, name: &str, nanos: u64, success: bool) {
        let entry = ObjectBuilder::new()
            .str_field("name", name)
            .int_field("nanos", nanos as i64)
            .bool_field("success", success)
            .build();
        self.entries.push(entry);
    }
    /// Record a simple boolean pass/fail result.
    pub fn record_result(&mut self, name: &str, passed: bool) {
        let entry = ObjectBuilder::new()
            .str_field("name", name)
            .bool_field("passed", passed)
            .build();
        self.entries.push(entry);
    }
    /// Finalize and produce a `JsonValue` summary.
    pub fn finalize(self) -> JsonValue {
        let total = self.entries.len();
        let passed = self
            .entries
            .iter()
            .filter(|e| {
                e.get("passed") == Some(&JsonValue::Bool(true))
                    || e.get("success") == Some(&JsonValue::Bool(true))
            })
            .count();
        ObjectBuilder::new()
            .str_field("label", &self.label)
            .int_field("total", total as i64)
            .int_field("passed", passed as i64)
            .field("entries", JsonValue::Array(self.entries))
            .build()
    }
    /// Return the number of entries recorded so far.
    pub fn entry_count(&self) -> usize {
        self.entries.len()
    }
}
/// A streaming JSON writer that writes JSON directly to any `Write` target.
///
/// Unlike `JsonValue` which builds a complete in-memory tree,
/// `JsonWriter` writes tokens incrementally.
pub struct JsonWriter<W: Write> {
    writer: W,
    indent: usize,
    pretty: bool,
}
impl<W: Write> JsonWriter<W> {
    /// Create a new `JsonWriter` wrapping the given writer.
    pub fn new(writer: W, pretty: bool) -> Self {
        JsonWriter {
            writer,
            indent: 0,
            pretty,
        }
    }
    /// Write a raw string as a JSON string value (with quotes and escaping).
    pub fn write_string(&mut self, s: &str) -> io::Result<()> {
        write!(self.writer, "\"")?;
        for c in s.chars() {
            match c {
                '\"' => write!(self.writer, "\\\"")?,
                '\\' => write!(self.writer, "\\\\")?,
                '\n' => write!(self.writer, "\\n")?,
                '\r' => write!(self.writer, "\\r")?,
                '\t' => write!(self.writer, "\\t")?,
                c => write!(self.writer, "{}", c)?,
            }
        }
        write!(self.writer, "\"")
    }
    /// Write a JSON number.
    pub fn write_number(&mut self, n: i64) -> io::Result<()> {
        write!(self.writer, "{}", n)
    }
    /// Write a JSON float.
    pub fn write_float(&mut self, f: f64) -> io::Result<()> {
        write!(self.writer, "{}", f)
    }
    /// Write `true` or `false`.
    pub fn write_bool(&mut self, b: bool) -> io::Result<()> {
        write!(self.writer, "{}", if b { "true" } else { "false" })
    }
    /// Write `null`.
    pub fn write_null(&mut self) -> io::Result<()> {
        write!(self.writer, "null")
    }
    /// Write an already-serialized JSON value.
    pub fn write_raw(&mut self, s: &str) -> io::Result<()> {
        write!(self.writer, "{}", s)
    }
    fn newline_indent(&mut self) -> io::Result<()> {
        if self.pretty {
            writeln!(self.writer)?;
            for _ in 0..self.indent {
                write!(self.writer, "  ")?;
            }
        }
        Ok(())
    }
    /// Serialize a complete `JsonValue` to the writer.
    pub fn write_value(&mut self, v: &JsonValue) -> io::Result<()> {
        match v {
            JsonValue::Null => self.write_null(),
            JsonValue::Bool(b) => self.write_bool(*b),
            JsonValue::Integer(n) => self.write_number(*n),
            JsonValue::Float(f) => self.write_float(*f),
            JsonValue::Str(s) => self.write_string(s),
            JsonValue::Array(arr) => {
                write!(self.writer, "[")?;
                self.indent += 1;
                for (i, item) in arr.iter().enumerate() {
                    self.newline_indent()?;
                    self.write_value(item)?;
                    if i + 1 < arr.len() {
                        write!(self.writer, ",")?;
                    }
                }
                self.indent -= 1;
                if !arr.is_empty() {
                    self.newline_indent()?;
                }
                write!(self.writer, "]")
            }
            JsonValue::Object(entries) => {
                write!(self.writer, "{{")?;
                self.indent += 1;
                for (i, (key, val)) in entries.iter().enumerate() {
                    self.newline_indent()?;
                    self.write_string(key)?;
                    write!(self.writer, ": ")?;
                    self.write_value(val)?;
                    if i + 1 < entries.len() {
                        write!(self.writer, ",")?;
                    }
                }
                self.indent -= 1;
                if !entries.is_empty() {
                    self.newline_indent()?;
                }
                write!(self.writer, "}}")
            }
        }
    }
}
#[allow(dead_code)]
pub struct CatalogBuilder {
    records: Vec<ExportRecord>,
    title: String,
    version: String,
}
#[allow(dead_code)]
impl CatalogBuilder {
    pub fn new(title: &str, version: &str) -> Self {
        Self {
            records: Vec::new(),
            title: title.to_string(),
            version: version.to_string(),
        }
    }
    pub fn add(&mut self, record: ExportRecord) {
        self.records.push(record);
    }
    pub fn count(&self) -> usize {
        self.records.len()
    }
    pub fn build_json(&self) -> String {
        let records_json = export_records_to_json(&self.records);
        format!(
            "{{\"title\":\"{}\",\"version\":\"{}\",\"count\":{},\"records\":{}}}",
            self.title,
            self.version,
            self.records.len(),
            records_json
        )
    }
}
#[allow(dead_code)]
pub struct JsonPatch {
    pub ops: Vec<JsonPatchOp>,
}
#[allow(dead_code)]
impl JsonPatch {
    pub fn new() -> Self {
        Self { ops: Vec::new() }
    }
    pub fn add(mut self, path: &str, value: JsonValue) -> Self {
        self.ops.push(JsonPatchOp::Add {
            path: path.to_string(),
            value,
        });
        self
    }
    pub fn remove(mut self, path: &str) -> Self {
        self.ops.push(JsonPatchOp::Remove {
            path: path.to_string(),
        });
        self
    }
    pub fn replace(mut self, path: &str, value: JsonValue) -> Self {
        self.ops.push(JsonPatchOp::Replace {
            path: path.to_string(),
            value,
        });
        self
    }
    pub fn to_json_array_string(&self) -> String {
        let ops: Vec<String> = self.ops.iter().map(|o| o.to_json_string()).collect();
        format!("[{}]", ops.join(","))
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ExportRecord {
    pub name: String,
    pub kind: String,
    pub ty: String,
    pub status: ProofStatus,
    pub module: String,
}
#[allow(dead_code)]
pub struct JsonCatalogIndex {
    by_module: std::collections::HashMap<String, Vec<String>>,
    by_kind: std::collections::HashMap<String, Vec<String>>,
}
#[allow(dead_code)]
impl JsonCatalogIndex {
    pub fn build(records: &[ExportRecord]) -> Self {
        let mut by_module: std::collections::HashMap<String, Vec<String>> =
            std::collections::HashMap::new();
        let mut by_kind: std::collections::HashMap<String, Vec<String>> =
            std::collections::HashMap::new();
        for r in records {
            by_module
                .entry(r.module.clone())
                .or_default()
                .push(r.name.clone());
            by_kind
                .entry(r.kind.clone())
                .or_default()
                .push(r.name.clone());
        }
        Self { by_module, by_kind }
    }
    pub fn lookup_by_module(&self, module: &str) -> &[String] {
        self.by_module
            .get(module)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }
    pub fn lookup_by_kind(&self, kind: &str) -> &[String] {
        self.by_kind.get(kind).map(|v| v.as_slice()).unwrap_or(&[])
    }
    pub fn modules(&self) -> Vec<&str> {
        self.by_module.keys().map(|s| s.as_str()).collect()
    }
    pub fn kinds(&self) -> Vec<&str> {
        self.by_kind.keys().map(|s| s.as_str()).collect()
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct JsonSchema {
    pub kind: JsonSchemaKind,
    pub required: Vec<String>,
    pub properties: std::collections::HashMap<String, JsonSchema>,
    pub description: Option<String>,
}
#[allow(dead_code)]
impl JsonSchema {
    pub fn object() -> Self {
        Self {
            kind: JsonSchemaKind::Object,
            required: Vec::new(),
            properties: std::collections::HashMap::new(),
            description: None,
        }
    }
    pub fn string() -> Self {
        Self {
            kind: JsonSchemaKind::String,
            required: Vec::new(),
            properties: std::collections::HashMap::new(),
            description: None,
        }
    }
    pub fn number() -> Self {
        Self {
            kind: JsonSchemaKind::Number,
            required: Vec::new(),
            properties: std::collections::HashMap::new(),
            description: None,
        }
    }
    pub fn boolean() -> Self {
        Self {
            kind: JsonSchemaKind::Boolean,
            required: Vec::new(),
            properties: std::collections::HashMap::new(),
            description: None,
        }
    }
    pub fn add_property(mut self, name: &str, schema: JsonSchema) -> Self {
        self.properties.insert(name.to_string(), schema);
        self
    }
    pub fn require(mut self, field: &str) -> Self {
        self.required.push(field.to_string());
        self
    }
    pub fn describe(mut self, desc: &str) -> Self {
        self.description = Some(desc.to_string());
        self
    }
}
impl JsonSchema {
    #[allow(dead_code)]
    pub fn array_schema() -> Self {
        Self {
            kind: JsonSchemaKind::Array,
            required: Vec::new(),
            properties: std::collections::HashMap::new(),
            description: None,
        }
    }
}
/// A lightweight JSON value representation.
///
/// This avoids requiring `serde_json` as a dependency.
#[derive(Clone, Debug, PartialEq)]
pub enum JsonValue {
    /// JSON null.
    Null,
    /// JSON boolean.
    Bool(bool),
    /// JSON number (integer).
    Integer(i64),
    /// JSON number (float).
    Float(f64),
    /// JSON string.
    Str(String),
    /// JSON array.
    Array(Vec<JsonValue>),
    /// JSON object (ordered by insertion).
    Object(Vec<(String, JsonValue)>),
}
impl JsonValue {
    /// Create a null value.
    pub fn null() -> Self {
        JsonValue::Null
    }
    /// Create a boolean value.
    pub fn bool_val(b: bool) -> Self {
        JsonValue::Bool(b)
    }
    /// Create an integer value.
    pub fn int(n: i64) -> Self {
        JsonValue::Integer(n)
    }
    /// Create a float value.
    pub fn float(f: f64) -> Self {
        JsonValue::Float(f)
    }
    /// Create a string value.
    pub fn string(s: impl Into<String>) -> Self {
        JsonValue::Str(s.into())
    }
    /// Create an array value.
    pub fn array(items: Vec<JsonValue>) -> Self {
        JsonValue::Array(items)
    }
    /// Create an empty object.
    pub fn object() -> Self {
        JsonValue::Object(Vec::new())
    }
    /// Check if this is null.
    pub fn is_null(&self) -> bool {
        matches!(self, JsonValue::Null)
    }
    /// Check if this is an object.
    pub fn is_object(&self) -> bool {
        matches!(self, JsonValue::Object(_))
    }
    /// Check if this is an array.
    pub fn is_array(&self) -> bool {
        matches!(self, JsonValue::Array(_))
    }
    /// Get a field from an object.
    pub fn get(&self, key: &str) -> Option<&JsonValue> {
        if let JsonValue::Object(fields) = self {
            fields.iter().find(|(k, _)| k == key).map(|(_, v)| v)
        } else {
            None
        }
    }
    /// Insert a field into an object (mutates in place).
    pub fn insert(&mut self, key: impl Into<String>, value: JsonValue) {
        if let JsonValue::Object(fields) = self {
            let key = key.into();
            if let Some(entry) = fields.iter_mut().find(|(k, _)| *k == key) {
                entry.1 = value;
            } else {
                fields.push((key, value));
            }
        }
    }
    /// Push a value into an array.
    pub fn push(&mut self, value: JsonValue) {
        if let JsonValue::Array(items) = self {
            items.push(value);
        }
    }
    /// Pretty-print the JSON value.
    pub fn to_pretty_string(&self) -> String {
        let mut output = String::new();
        self.format_pretty(&mut output, 0);
        output
    }
    /// Compact JSON string.
    pub fn to_compact_string(&self) -> String {
        let mut output = String::new();
        self.format_compact(&mut output);
        output
    }
    fn format_pretty(&self, out: &mut String, indent: usize) {
        let indent_str = "  ".repeat(indent);
        let inner_indent = "  ".repeat(indent + 1);
        match self {
            JsonValue::Null => out.push_str("null"),
            JsonValue::Bool(b) => out.push_str(if *b { "true" } else { "false" }),
            JsonValue::Integer(n) => out.push_str(&n.to_string()),
            JsonValue::Float(f) => {
                if f.is_finite() {
                    out.push_str(&f.to_string());
                } else {
                    out.push_str("null");
                }
            }
            JsonValue::Str(s) => {
                out.push('"');
                out.push_str(&escape_json_string(s));
                out.push('"');
            }
            JsonValue::Array(items) => {
                if items.is_empty() {
                    out.push_str("[]");
                    return;
                }
                out.push_str("[\n");
                for (i, item) in items.iter().enumerate() {
                    out.push_str(&inner_indent);
                    item.format_pretty(out, indent + 1);
                    if i < items.len() - 1 {
                        out.push(',');
                    }
                    out.push('\n');
                }
                out.push_str(&indent_str);
                out.push(']');
            }
            JsonValue::Object(fields) => {
                if fields.is_empty() {
                    out.push_str("{}");
                    return;
                }
                out.push_str("{\n");
                for (i, (key, value)) in fields.iter().enumerate() {
                    out.push_str(&inner_indent);
                    out.push('"');
                    out.push_str(&escape_json_string(key));
                    out.push_str("\": ");
                    value.format_pretty(out, indent + 1);
                    if i < fields.len() - 1 {
                        out.push(',');
                    }
                    out.push('\n');
                }
                out.push_str(&indent_str);
                out.push('}');
            }
        }
    }
    fn format_compact(&self, out: &mut String) {
        match self {
            JsonValue::Null => out.push_str("null"),
            JsonValue::Bool(b) => out.push_str(if *b { "true" } else { "false" }),
            JsonValue::Integer(n) => out.push_str(&n.to_string()),
            JsonValue::Float(f) => {
                if f.is_finite() {
                    out.push_str(&f.to_string());
                } else {
                    out.push_str("null");
                }
            }
            JsonValue::Str(s) => {
                out.push('"');
                out.push_str(&escape_json_string(s));
                out.push('"');
            }
            JsonValue::Array(items) => {
                out.push('[');
                for (i, item) in items.iter().enumerate() {
                    if i > 0 {
                        out.push(',');
                    }
                    item.format_compact(out);
                }
                out.push(']');
            }
            JsonValue::Object(fields) => {
                out.push('{');
                for (i, (key, value)) in fields.iter().enumerate() {
                    if i > 0 {
                        out.push(',');
                    }
                    out.push('"');
                    out.push_str(&escape_json_string(key));
                    out.push_str("\":");
                    value.format_compact(out);
                }
                out.push('}');
            }
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum JsonPatchOp {
    Add { path: String, value: JsonValue },
    Remove { path: String },
    Replace { path: String, value: JsonValue },
    Move { from: String, path: String },
    Copy { from: String, path: String },
    Test { path: String, value: JsonValue },
}
#[allow(dead_code)]
impl JsonPatchOp {
    pub fn op_name(&self) -> &'static str {
        match self {
            Self::Add { .. } => "add",
            Self::Remove { .. } => "remove",
            Self::Replace { .. } => "replace",
            Self::Move { .. } => "move",
            Self::Copy { .. } => "copy",
            Self::Test { .. } => "test",
        }
    }
    pub fn to_json_string(&self) -> String {
        match self {
            Self::Add { path, value } => {
                format!(
                    "{{\"op\":\"add\",\"path\":\"{}\",\"value\":{}}}",
                    path,
                    value_to_json(value)
                )
            }
            Self::Remove { path } => {
                format!("{{\"op\":\"remove\",\"path\":\"{}\"}}", path)
            }
            Self::Replace { path, value } => {
                format!(
                    "{{\"op\":\"replace\",\"path\":\"{}\",\"value\":{}}}",
                    path,
                    value_to_json(value)
                )
            }
            Self::Move { from, path } => {
                format!(
                    "{{\"op\":\"move\",\"from\":\"{}\",\"path\":\"{}\"}}",
                    from, path
                )
            }
            Self::Copy { from, path } => {
                format!(
                    "{{\"op\":\"copy\",\"from\":\"{}\",\"path\":\"{}\"}}",
                    from, path
                )
            }
            Self::Test { path, value } => {
                format!(
                    "{{\"op\":\"test\",\"path\":\"{}\",\"value\":{}}}",
                    path,
                    value_to_json(value)
                )
            }
        }
    }
}
#[allow(dead_code)]
pub struct StreamJsonWriter<W: std::io::Write> {
    writer: W,
    item_count: usize,
    is_first: bool,
}
#[allow(dead_code)]
impl<W: std::io::Write> StreamJsonWriter<W> {
    pub fn new(writer: W) -> Self {
        Self {
            writer,
            item_count: 0,
            is_first: true,
        }
    }
    pub fn begin_array(&mut self) -> std::io::Result<()> {
        write!(self.writer, "[")
    }
    pub fn write_item(&mut self, json: &str) -> std::io::Result<()> {
        if !self.is_first {
            write!(self.writer, ",")?;
        }
        self.is_first = false;
        write!(self.writer, "{}", json)?;
        self.item_count += 1;
        Ok(())
    }
    pub fn end_array(&mut self) -> std::io::Result<()> {
        write!(self.writer, "]")
    }
    pub fn item_count(&self) -> usize {
        self.item_count
    }
}
/// A snapshot of the OxiLean environment in JSON format.
pub struct EnvironmentSnapshot {
    /// Definitions in the environment.
    pub definitions: Vec<JsonValue>,
    /// Theorems in the environment.
    pub theorems: Vec<JsonValue>,
    /// Inductives in the environment.
    pub inductives: Vec<JsonValue>,
    /// Axioms in the environment.
    pub axioms: Vec<JsonValue>,
    /// Notations in the environment.
    pub notations: Vec<JsonValue>,
    /// Metadata.
    pub metadata: HashMap<String, String>,
}
impl EnvironmentSnapshot {
    /// Create a new empty snapshot.
    pub fn new() -> Self {
        Self {
            definitions: Vec::new(),
            theorems: Vec::new(),
            inductives: Vec::new(),
            axioms: Vec::new(),
            notations: Vec::new(),
            metadata: HashMap::new(),
        }
    }
    /// Add a definition entry.
    pub fn add_definition(&mut self, name: &str, ty: &str, val: Option<&str>, attrs: &[String]) {
        let mut obj = ObjectBuilder::new()
            .str_field("name", name)
            .str_field("type", ty)
            .field(
                "attributes",
                JsonValue::array(attrs.iter().map(JsonValue::string).collect()),
            );
        if let Some(v) = val {
            obj = obj.str_field("value", v);
        }
        self.definitions.push(obj.build());
    }
    /// Add a theorem entry.
    pub fn add_theorem(&mut self, name: &str, statement: &str, proof_summary: Option<&str>) {
        let obj = ObjectBuilder::new()
            .str_field("name", name)
            .str_field("statement", statement)
            .optional_str("proof_summary", proof_summary);
        self.theorems.push(obj.build());
    }
    /// Add an inductive type entry.
    pub fn add_inductive(
        &mut self,
        name: &str,
        params: &[String],
        constructors: &[(String, String)],
    ) {
        let ctors = constructors
            .iter()
            .map(|(n, t)| {
                ObjectBuilder::new()
                    .str_field("name", n)
                    .str_field("type", t)
                    .build()
            })
            .collect();
        let obj = ObjectBuilder::new()
            .str_field("name", name)
            .field("params", names_to_json(params))
            .field("constructors", JsonValue::array(ctors));
        self.inductives.push(obj.build());
    }
    /// Add an axiom entry.
    pub fn add_axiom(&mut self, name: &str, ty: &str) {
        let obj = ObjectBuilder::new()
            .str_field("name", name)
            .str_field("type", ty);
        self.axioms.push(obj.build());
    }
    /// Set metadata.
    pub fn set_metadata(&mut self, key: &str, value: &str) {
        self.metadata.insert(key.to_string(), value.to_string());
    }
    /// Convert to JSON.
    pub fn to_json(&self) -> JsonValue {
        let meta = JsonValue::Object(
            self.metadata
                .iter()
                .map(|(k, v)| (k.clone(), JsonValue::string(v)))
                .collect(),
        );
        ObjectBuilder::new()
            .field("metadata", meta)
            .field("definitions", JsonValue::array(self.definitions.clone()))
            .field("theorems", JsonValue::array(self.theorems.clone()))
            .field("inductives", JsonValue::array(self.inductives.clone()))
            .field("axioms", JsonValue::array(self.axioms.clone()))
            .field("notations", JsonValue::array(self.notations.clone()))
            .build()
    }
    /// Generate pretty-printed JSON string.
    #[allow(clippy::inherent_to_string)]
    pub fn to_string(&self) -> String {
        self.to_json().to_pretty_string()
    }
    /// Write to a writer.
    pub fn write_to<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(self.to_string().as_bytes())
    }
}
#[allow(dead_code)]
pub struct JsonStreamDecoder {
    state: JsonDecodeState,
    depth: usize,
    buffer: String,
}
#[allow(dead_code)]
impl JsonStreamDecoder {
    pub fn new() -> Self {
        Self {
            state: JsonDecodeState::Idle,
            depth: 0,
            buffer: String::new(),
        }
    }
    pub fn feed(&mut self, chunk: &str) -> Vec<String> {
        let mut completed = Vec::new();
        self.buffer.push_str(chunk);
        while let Some(newline) = self.buffer.find('\n') {
            let line = self.buffer[..newline].trim().to_string();
            if !line.is_empty() {
                completed.push(line);
            }
            self.buffer = self.buffer[newline + 1..].to_string();
        }
        completed
    }
    pub fn flush(&mut self) -> Option<String> {
        if self.buffer.trim().is_empty() {
            return None;
        }
        let s = self.buffer.trim().to_string();
        self.buffer.clear();
        Some(s)
    }
    pub fn state(&self) -> &JsonDecodeState {
        &self.state
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum JsonDecodeState {
    Idle,
    InObject,
    InArray,
    InString,
    Done,
}
/// Export a proof term to JSON representation.
pub struct ProofExporter {
    /// Configuration.
    pub config: JsonExportConfig,
}
impl ProofExporter {
    /// Create a new proof exporter.
    pub fn new() -> Self {
        Self {
            config: JsonExportConfig::default(),
        }
    }
    /// Create with custom configuration.
    pub fn with_config(config: JsonExportConfig) -> Self {
        Self { config }
    }
    /// Export a proof step.
    pub fn export_step(
        &self,
        tactic_name: &str,
        args: &[String],
        goals_before: &[String],
        goals_after: &[String],
    ) -> JsonValue {
        ObjectBuilder::new()
            .str_field("tactic", tactic_name)
            .field(
                "args",
                JsonValue::array(args.iter().map(JsonValue::string).collect()),
            )
            .field(
                "goals_before",
                JsonValue::array(goals_before.iter().map(JsonValue::string).collect()),
            )
            .field(
                "goals_after",
                JsonValue::array(goals_after.iter().map(JsonValue::string).collect()),
            )
            .build()
    }
    /// Export a complete proof trace.
    pub fn export_proof(
        &self,
        theorem_name: &str,
        statement: &str,
        steps: &[JsonValue],
    ) -> JsonValue {
        ObjectBuilder::new()
            .str_field("theorem", theorem_name)
            .str_field("statement", statement)
            .field("steps", JsonValue::array(steps.to_vec()))
            .int_field("step_count", steps.len() as i64)
            .build()
    }
}
