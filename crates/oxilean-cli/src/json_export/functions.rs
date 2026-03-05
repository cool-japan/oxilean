//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::HashMap;

use super::types::{
    CatalogBuilder, EnvironmentSnapshot, ExportRecord, ExportStats, JsonCatalogIndex, JsonPatch,
    JsonPatchOp, JsonSchema, JsonSchemaKind, JsonStreamDecoder, JsonValue, JsonValueStats,
    JsonWriter, ObjectBuilder, ProofExporter, ProofStatus, ResultCollector, StreamJsonWriter,
    StripNullFields,
};

/// Escape special characters in a JSON string.
pub fn escape_json_string(s: &str) -> String {
    let mut result = String::new();
    for ch in s.chars() {
        match ch {
            '"' => result.push_str("\\\""),
            '\\' => result.push_str("\\\\"),
            '\n' => result.push_str("\\n"),
            '\r' => result.push_str("\\r"),
            '\t' => result.push_str("\\t"),
            c if c.is_control() => {
                result.push_str(&format!("\\u{:04x}", c as u32));
            }
            c => result.push(c),
        }
    }
    result
}
/// Export an expression-like node to JSON.
pub fn expr_to_json(kind: &str, fields: Vec<(String, JsonValue)>) -> JsonValue {
    let mut obj = ObjectBuilder::new().str_field("kind", kind);
    for (k, v) in fields {
        obj = obj.field(k, v);
    }
    obj.build()
}
/// Export a span to JSON.
pub fn span_to_json(start: usize, end: usize) -> JsonValue {
    ObjectBuilder::new()
        .int_field("start", start as i64)
        .int_field("end", end as i64)
        .build()
}
/// Export a name to JSON.
pub fn name_to_json(name: &str) -> JsonValue {
    JsonValue::string(name)
}
/// Export a list of names to JSON.
pub fn names_to_json(names: &[String]) -> JsonValue {
    JsonValue::array(names.iter().map(JsonValue::string).collect())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_json_null() {
        assert_eq!(JsonValue::null().to_compact_string(), "null");
    }
    #[test]
    fn test_json_bool() {
        assert_eq!(JsonValue::bool_val(true).to_compact_string(), "true");
        assert_eq!(JsonValue::bool_val(false).to_compact_string(), "false");
    }
    #[test]
    fn test_json_integer() {
        assert_eq!(JsonValue::int(42).to_compact_string(), "42");
    }
    #[test]
    fn test_json_string() {
        assert_eq!(JsonValue::string("hello").to_compact_string(), "\"hello\"");
    }
    #[test]
    fn test_json_string_escape() {
        assert_eq!(JsonValue::string("a\"b").to_compact_string(), "\"a\\\"b\"");
    }
    #[test]
    fn test_json_array() {
        let arr = JsonValue::array(vec![JsonValue::int(1), JsonValue::int(2)]);
        assert_eq!(arr.to_compact_string(), "[1,2]");
    }
    #[test]
    fn test_json_object() {
        let obj = ObjectBuilder::new()
            .str_field("name", "foo")
            .int_field("value", 42)
            .build();
        let compact = obj.to_compact_string();
        assert!(compact.contains("\"name\":\"foo\""));
        assert!(compact.contains("\"value\":42"));
    }
    #[test]
    fn test_environment_snapshot() {
        let mut snap = EnvironmentSnapshot::new();
        snap.add_definition("Nat.double", "Nat -> Nat", Some("fun n => n + n"), &[]);
        snap.add_theorem("Nat.double_zero", "Nat.double 0 = 0", None);
        snap.set_metadata("version", "0.1.0");
        let json = snap.to_json();
        assert!(json.is_object());
        assert!(json.get("definitions").is_some());
        assert!(json.get("theorems").is_some());
    }
    #[test]
    fn test_proof_exporter() {
        let exporter = ProofExporter::new();
        let step = exporter.export_step(
            "intro",
            &["h".to_string()],
            &["P -> P".to_string()],
            &["P".to_string()],
        );
        assert!(step.get("tactic").is_some());
    }
    #[test]
    fn test_span_to_json() {
        let span = span_to_json(10, 20);
        assert_eq!(span.get("start"), Some(&JsonValue::Integer(10)));
        assert_eq!(span.get("end"), Some(&JsonValue::Integer(20)));
    }
    #[test]
    fn test_object_builder_optional() {
        let obj = ObjectBuilder::new()
            .optional_str("present", Some("hello"))
            .optional_str("absent", None)
            .build();
        assert_eq!(obj.get("present"), Some(&JsonValue::string("hello")));
        assert_eq!(obj.get("absent"), Some(&JsonValue::Null));
    }
}
/// Parse a JSON string into a `JsonValue`.
///
/// This is a lightweight parser supporting strings, numbers, booleans,
/// null, arrays, and objects.
pub fn parse_json(input: &str) -> Option<JsonValue> {
    let chars: Vec<char> = input.chars().collect();
    let mut pos = 0;
    skip_whitespace(&chars, &mut pos);
    parse_value(&chars, &mut pos)
}
fn skip_whitespace(chars: &[char], pos: &mut usize) {
    while *pos < chars.len() && chars[*pos].is_ascii_whitespace() {
        *pos += 1;
    }
}
fn parse_value(chars: &[char], pos: &mut usize) -> Option<JsonValue> {
    skip_whitespace(chars, pos);
    if *pos >= chars.len() {
        return None;
    }
    match chars[*pos] {
        '"' => parse_string(chars, pos).map(JsonValue::Str),
        '[' => parse_array(chars, pos),
        '{' => parse_object(chars, pos),
        't' => {
            if chars.get(*pos..*pos + 4) == Some(&['t', 'r', 'u', 'e']) {
                *pos += 4;
                Some(JsonValue::Bool(true))
            } else {
                None
            }
        }
        'f' => {
            if chars.get(*pos..*pos + 5) == Some(&['f', 'a', 'l', 's', 'e']) {
                *pos += 5;
                Some(JsonValue::Bool(false))
            } else {
                None
            }
        }
        'n' => {
            if chars.get(*pos..*pos + 4) == Some(&['n', 'u', 'l', 'l']) {
                *pos += 4;
                Some(JsonValue::Null)
            } else {
                None
            }
        }
        c if c == '-' || c.is_ascii_digit() => parse_number(chars, pos),
        _ => None,
    }
}
fn parse_string(chars: &[char], pos: &mut usize) -> Option<String> {
    if chars.get(*pos) != Some(&'"') {
        return None;
    }
    *pos += 1;
    let mut s = String::new();
    while *pos < chars.len() {
        match chars[*pos] {
            '"' => {
                *pos += 1;
                return Some(s);
            }
            '\\' => {
                *pos += 1;
                if *pos >= chars.len() {
                    return None;
                }
                match chars[*pos] {
                    '"' => s.push('"'),
                    '\\' => s.push('\\'),
                    'n' => s.push('\n'),
                    'r' => s.push('\r'),
                    't' => s.push('\t'),
                    c => s.push(c),
                }
                *pos += 1;
            }
            c => {
                s.push(c);
                *pos += 1;
            }
        }
    }
    None
}
fn parse_number(chars: &[char], pos: &mut usize) -> Option<JsonValue> {
    let start = *pos;
    if chars.get(*pos) == Some(&'-') {
        *pos += 1;
    }
    while *pos < chars.len() && chars[*pos].is_ascii_digit() {
        *pos += 1;
    }
    let s: String = chars[start..*pos].iter().collect();
    s.parse::<i64>().ok().map(JsonValue::Integer)
}
fn parse_array(chars: &[char], pos: &mut usize) -> Option<JsonValue> {
    if chars.get(*pos) != Some(&'[') {
        return None;
    }
    *pos += 1;
    let mut items = Vec::new();
    skip_whitespace(chars, pos);
    if chars.get(*pos) == Some(&']') {
        *pos += 1;
        return Some(JsonValue::Array(items));
    }
    loop {
        let item = parse_value(chars, pos)?;
        items.push(item);
        skip_whitespace(chars, pos);
        match chars.get(*pos) {
            Some(&',') => {
                *pos += 1;
            }
            Some(&']') => {
                *pos += 1;
                break;
            }
            _ => return None,
        }
    }
    Some(JsonValue::Array(items))
}
fn parse_object(chars: &[char], pos: &mut usize) -> Option<JsonValue> {
    if chars.get(*pos) != Some(&'{') {
        return None;
    }
    *pos += 1;
    let mut entries: Vec<(String, JsonValue)> = Vec::new();
    skip_whitespace(chars, pos);
    if chars.get(*pos) == Some(&'}') {
        *pos += 1;
        return Some(JsonValue::Object(entries));
    }
    loop {
        skip_whitespace(chars, pos);
        let key = parse_string(chars, pos)?;
        skip_whitespace(chars, pos);
        if chars.get(*pos) != Some(&':') {
            return None;
        }
        *pos += 1;
        let val = parse_value(chars, pos)?;
        entries.push((key, val));
        skip_whitespace(chars, pos);
        match chars.get(*pos) {
            Some(&',') => {
                *pos += 1;
            }
            Some(&'}') => {
                *pos += 1;
                break;
            }
            _ => return None,
        }
    }
    Some(JsonValue::Object(entries))
}
#[cfg(test)]
mod extended_tests {
    use super::*;
    #[test]
    fn test_json_writer_string() {
        let mut buf = Vec::new();
        let mut w = JsonWriter::new(&mut buf, false);
        w.write_string("hello world")
            .expect("I/O operation should succeed");
        assert_eq!(
            String::from_utf8(buf).expect("test operation should succeed"),
            r#""hello world""#
        );
    }
    #[test]
    fn test_json_writer_string_escape() {
        let mut buf = Vec::new();
        let mut w = JsonWriter::new(&mut buf, false);
        w.write_string("say \"hi\"")
            .expect("I/O operation should succeed");
        let s = String::from_utf8(buf).expect("test operation should succeed");
        assert!(s.contains("\\"));
    }
    #[test]
    fn test_json_writer_bool() {
        let mut buf = Vec::new();
        let mut w = JsonWriter::new(&mut buf, false);
        w.write_bool(true).expect("I/O operation should succeed");
        assert_eq!(
            String::from_utf8(buf).expect("test operation should succeed"),
            "true"
        );
    }
    #[test]
    fn test_json_writer_number() {
        let mut buf = Vec::new();
        let mut w = JsonWriter::new(&mut buf, false);
        w.write_number(42).expect("I/O operation should succeed");
        assert_eq!(
            String::from_utf8(buf).expect("test operation should succeed"),
            "42"
        );
    }
    #[test]
    fn test_json_writer_null() {
        let mut buf = Vec::new();
        let mut w = JsonWriter::new(&mut buf, false);
        w.write_null().expect("I/O operation should succeed");
        assert_eq!(
            String::from_utf8(buf).expect("test operation should succeed"),
            "null"
        );
    }
    #[test]
    fn test_json_writer_value() {
        let v = JsonValue::Integer(99);
        let mut buf = Vec::new();
        let mut w = JsonWriter::new(&mut buf, false);
        w.write_value(&v).expect("I/O operation should succeed");
        assert_eq!(
            String::from_utf8(buf).expect("test operation should succeed"),
            "99"
        );
    }
    #[test]
    fn test_result_collector_record() {
        let mut c = ResultCollector::new("suite");
        c.record_result("test1", true);
        c.record_result("test2", false);
        assert_eq!(c.entry_count(), 2);
        let summary = c.finalize();
        assert_eq!(summary.get("total"), Some(&JsonValue::Integer(2)));
        assert_eq!(summary.get("passed"), Some(&JsonValue::Integer(1)));
    }
    #[test]
    fn test_result_collector_timing() {
        let mut c = ResultCollector::new("bench");
        c.record_timing("kernel", 500, true);
        assert_eq!(c.entry_count(), 1);
        let summary = c.finalize();
        assert_eq!(summary.get("total"), Some(&JsonValue::Integer(1)));
    }
    #[test]
    fn test_parse_json_null() {
        assert_eq!(parse_json("null"), Some(JsonValue::Null));
    }
    #[test]
    fn test_parse_json_bool() {
        assert_eq!(parse_json("true"), Some(JsonValue::Bool(true)));
        assert_eq!(parse_json("false"), Some(JsonValue::Bool(false)));
    }
    #[test]
    fn test_parse_json_number() {
        assert_eq!(parse_json("42"), Some(JsonValue::Integer(42)));
        assert_eq!(parse_json("-7"), Some(JsonValue::Integer(-7)));
    }
    #[test]
    fn test_parse_json_string() {
        assert_eq!(
            parse_json(r#""hello""#),
            Some(JsonValue::Str("hello".into()))
        );
    }
    #[test]
    fn test_parse_json_array() {
        let v = parse_json("[1, 2, 3]").expect("parsing should succeed");
        match v {
            JsonValue::Array(arr) => assert_eq!(arr.len(), 3),
            _ => panic!("Expected Array"),
        }
    }
    #[test]
    fn test_parse_json_object() {
        let v = parse_json(r#"{"key": "val"}"#).expect("parsing should succeed");
        match &v {
            JsonValue::Object(entries) => {
                let found = entries.iter().find(|(k, _)| k == "key").map(|(_, v)| v);
                assert_eq!(found, Some(&JsonValue::Str("val".into())));
            }
            _ => panic!("Expected Object"),
        }
    }
    #[test]
    fn test_object_builder_float_field() {
        let obj = ObjectBuilder::new().float_field("val", 1.23).build();
        assert_eq!(obj.get("val"), Some(&JsonValue::Float(1.23)));
    }
}
#[allow(dead_code)]
pub fn value_to_json(v: &JsonValue) -> String {
    match v {
        JsonValue::Str(s) => format!("\"{}\"", s.replace('"', "\\\"")),
        JsonValue::Integer(n) => format!("{}", n),
        JsonValue::Float(f) => format!("{:.6}", f),
        JsonValue::Bool(b) => b.to_string(),
        JsonValue::Null => "null".to_string(),
        JsonValue::Array(arr) => {
            format!(
                "[{}]",
                arr.iter().map(value_to_json).collect::<Vec<_>>().join(",")
            )
        }
        JsonValue::Object(obj) => {
            let fields: Vec<String> = obj
                .iter()
                .map(|(k, v)| format!("\"{}\":{}", k, value_to_json(v)))
                .collect();
            format!("{{{}}}", fields.join(","))
        }
    }
}
#[allow(dead_code)]
pub fn oxilean_export_schema() -> JsonSchema {
    JsonSchema::object()
        .add_property("version", JsonSchema::string().describe("OxiLean version"))
        .add_property("declarations", JsonSchema::array_schema())
        .add_property("errors", JsonSchema::array_schema())
        .require("version")
        .require("declarations")
        .describe("OxiLean export format")
}
#[allow(dead_code)]
pub fn write_ndjson_values(values: &[JsonValue]) -> String {
    values
        .iter()
        .map(value_to_json)
        .collect::<Vec<_>>()
        .join("\n")
}
#[allow(dead_code)]
pub fn parse_ndjson(input: &str) -> Vec<String> {
    input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| l.to_string())
        .collect()
}
#[allow(dead_code)]
pub fn json_merge_patch(
    target: &mut std::collections::HashMap<String, JsonValue>,
    patch: &std::collections::HashMap<String, JsonValue>,
) {
    for (k, v) in patch {
        match v {
            JsonValue::Null => {
                target.remove(k);
            }
            _ => {
                target.insert(k.clone(), v.clone());
            }
        }
    }
}
#[allow(dead_code)]
pub fn json_ptr<'a>(root: &'a JsonValue, pointer: &str) -> Option<&'a JsonValue> {
    if pointer.is_empty() {
        return Some(root);
    }
    if !pointer.starts_with('/') {
        return None;
    }
    let mut current = root;
    for token in pointer[1..].split('/') {
        let token = token.replace("~1", "/").replace("~0", "~");
        match current {
            JsonValue::Object(map) => {
                current = map.iter().find(|(k, _)| k == &token).map(|(_, v)| v)?;
            }
            JsonValue::Array(arr) => {
                let idx: usize = token.parse().ok()?;
                current = arr.get(idx)?;
            }
            _ => return None,
        }
    }
    Some(current)
}
#[allow(dead_code)]
pub fn json_deep_equal(a: &JsonValue, b: &JsonValue) -> bool {
    match (a, b) {
        (JsonValue::Null, JsonValue::Null) => true,
        (JsonValue::Bool(x), JsonValue::Bool(y)) => x == y,
        (JsonValue::Integer(x), JsonValue::Integer(y)) => x == y,
        (JsonValue::Float(x), JsonValue::Float(y)) => (x - y).abs() < 1e-10,
        (JsonValue::Str(x), JsonValue::Str(y)) => x == y,
        (JsonValue::Array(x), JsonValue::Array(y)) => {
            x.len() == y.len() && x.iter().zip(y.iter()).all(|(a, b)| json_deep_equal(a, b))
        }
        (JsonValue::Object(x), JsonValue::Object(y)) => {
            x.len() == y.len()
                && x.iter().all(|(k, v)| {
                    y.iter()
                        .find(|(yk, _)| yk == k)
                        .map(|(_, v2)| json_deep_equal(v, v2))
                        .unwrap_or(false)
                })
        }
        _ => false,
    }
}
#[allow(dead_code)]
pub fn json_stats(value: &JsonValue) -> JsonValueStats {
    let mut stats = JsonValueStats::default();
    fn recurse(v: &JsonValue, stats: &mut JsonValueStats, depth: usize) {
        if depth > stats.max_depth {
            stats.max_depth = depth;
        }
        match v {
            JsonValue::Object(m) => {
                stats.object_count += 1;
                for (_, val) in m.iter() {
                    recurse(val, stats, depth + 1);
                }
            }
            JsonValue::Array(a) => {
                stats.array_count += 1;
                for val in a {
                    recurse(val, stats, depth + 1);
                }
            }
            JsonValue::Str(_) => stats.string_count += 1,
            JsonValue::Integer(_) | JsonValue::Float(_) => stats.number_count += 1,
            JsonValue::Bool(_) => stats.bool_count += 1,
            JsonValue::Null => stats.null_count += 1,
        }
    }
    recurse(value, &mut stats, 0);
    stats
}
#[allow(dead_code)]
pub trait JsonValueTransform {
    fn transform(&self, value: JsonValue) -> JsonValue;
}
#[allow(dead_code)]
pub fn apply_json_transform(value: JsonValue, transform: &dyn JsonValueTransform) -> JsonValue {
    transform.transform(value)
}
#[allow(dead_code)]
pub fn minify_json(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let mut in_string = false;
    let mut prev_backslash = false;
    for ch in input.chars() {
        if in_string {
            out.push(ch);
            if ch == '\\' && !prev_backslash {
                prev_backslash = true;
            } else if ch == '"' && !prev_backslash {
                in_string = false;
                prev_backslash = false;
            } else {
                prev_backslash = false;
            }
        } else if ch == '"' {
            in_string = true;
            out.push(ch);
        } else if !ch.is_whitespace() {
            out.push(ch);
        }
    }
    out
}
#[allow(dead_code)]
pub fn pretty_print_json(value: &JsonValue, indent: usize) -> String {
    let pad = "  ".repeat(indent);
    let inner_pad = "  ".repeat(indent + 1);
    match value {
        JsonValue::Object(map) => {
            if map.is_empty() {
                return "{}".to_string();
            }
            let mut entries: Vec<(&String, &JsonValue)> = map.iter().map(|(k, v)| (k, v)).collect();
            entries.sort_by_key(|(k, _)| k.as_str());
            let fields: Vec<String> = entries
                .iter()
                .map(|(k, v)| {
                    format!(
                        "{}\"{}\": {}",
                        inner_pad,
                        k,
                        pretty_print_json(v, indent + 1)
                    )
                })
                .collect();
            format!("{{\n{}\n{}}}", fields.join(",\n"), pad)
        }
        JsonValue::Array(arr) => {
            if arr.is_empty() {
                return "[]".to_string();
            }
            let items: Vec<String> = arr
                .iter()
                .map(|v| format!("{}{}", inner_pad, pretty_print_json(v, indent + 1)))
                .collect();
            format!("[\n{}\n{}]", items.join(",\n"), pad)
        }
        JsonValue::Str(s) => format!("\"{}\"", s.replace('"', "\\\"")),
        JsonValue::Integer(n) => format!("{}", n),
        JsonValue::Float(f) => format!("{:.6}", f),
        JsonValue::Bool(b) => b.to_string(),
        JsonValue::Null => "null".to_string(),
    }
}
#[allow(dead_code)]
pub fn export_records_to_json(records: &[ExportRecord]) -> String {
    let items: Vec<String> = records
        .iter()
        .map(|r| {
            format!(
                "{{\"name\":\"{}\",\"kind\":\"{}\",\"status\":\"{}\",\"module\":\"{}\"}}",
                r.name,
                r.kind,
                r.status.as_str(),
                r.module
            )
        })
        .collect();
    format!("[{}]", items.join(","))
}
#[allow(dead_code)]
pub fn filter_by_status<'a>(
    records: &'a [ExportRecord],
    status: &ProofStatus,
) -> Vec<&'a ExportRecord> {
    records
        .iter()
        .filter(|r| r.status.as_str() == status.as_str())
        .collect()
}
#[allow(dead_code)]
pub fn count_per_module(records: &[ExportRecord]) -> std::collections::HashMap<String, usize> {
    let mut counts = std::collections::HashMap::new();
    for r in records {
        *counts.entry(r.module.clone()).or_default() += 1;
    }
    counts
}
#[cfg(test)]
mod json_export_extended_tests {
    use super::*;
    #[test]
    fn test_json_patch_add() {
        let patch = JsonPatch::new()
            .add("/foo", JsonValue::Str("bar".to_string()))
            .remove("/baz");
        let s = patch.to_json_array_string();
        assert!(s.contains("add"));
        assert!(s.contains("remove"));
    }
    #[test]
    fn test_json_merge_patch() {
        let mut target = std::collections::HashMap::new();
        target.insert("a".to_string(), JsonValue::Str("1".to_string()));
        target.insert("b".to_string(), JsonValue::Str("2".to_string()));
        let mut patch = std::collections::HashMap::new();
        patch.insert("a".to_string(), JsonValue::Str("updated".to_string()));
        patch.insert("b".to_string(), JsonValue::Null);
        json_merge_patch(&mut target, &patch);
        assert_eq!(
            target.get("a"),
            Some(&JsonValue::Str("updated".to_string()))
        );
        assert!(!target.contains_key("b"));
    }
    #[test]
    fn test_json_ptr_root() {
        let v = JsonValue::Str("x".to_string());
        assert_eq!(json_ptr(&v, ""), Some(&v));
    }
    #[test]
    fn test_json_deep_equal_strings() {
        let a = JsonValue::Str("hello".to_string());
        let b = JsonValue::Str("hello".to_string());
        assert!(json_deep_equal(&a, &b));
    }
    #[test]
    fn test_json_deep_equal_different() {
        let a = JsonValue::Integer(1);
        let b = JsonValue::Integer(2);
        assert!(!json_deep_equal(&a, &b));
    }
    #[test]
    fn test_json_stats_counts() {
        let m = vec![
            ("s".to_string(), JsonValue::Str("x".to_string())),
            ("n".to_string(), JsonValue::Integer(1)),
            ("b".to_string(), JsonValue::Bool(true)),
        ];
        let v = JsonValue::Object(m);
        let stats = json_stats(&v);
        assert_eq!(stats.object_count, 1);
        assert_eq!(stats.string_count, 1);
        assert_eq!(stats.number_count, 1);
        assert_eq!(stats.bool_count, 1);
    }
    #[test]
    fn test_strip_null_fields() {
        let m = vec![
            ("keep".to_string(), JsonValue::Str("v".to_string())),
            ("drop".to_string(), JsonValue::Null),
        ];
        let v = JsonValue::Object(m);
        let result = apply_json_transform(v, &StripNullFields);
        if let JsonValue::Object(map) = result {
            assert!(map.iter().any(|(k, _)| k == "keep"));
            assert!(!map.iter().any(|(k, _)| k == "drop"));
        } else {
            panic!("expected object");
        }
    }
    #[test]
    fn test_minify_json() {
        let input = "{ \"key\" : \"value\" }";
        let minified = minify_json(input);
        assert_eq!(minified, "{\"key\":\"value\"}");
    }
    #[test]
    fn test_pretty_print_json() {
        let m = vec![("key".to_string(), JsonValue::Str("val".to_string()))];
        let v = JsonValue::Object(m);
        let pretty = pretty_print_json(&v, 0);
        assert!(pretty.contains('\n'));
        assert!(pretty.contains("key"));
    }
    #[test]
    fn test_export_records_to_json() {
        let records = vec![ExportRecord {
            name: "foo".to_string(),
            kind: "theorem".to_string(),
            ty: "Nat".to_string(),
            status: ProofStatus::Complete,
            module: "Main".to_string(),
        }];
        let json = export_records_to_json(&records);
        assert!(json.contains("foo"));
        assert!(json.contains("complete"));
    }
    #[test]
    fn test_catalog_builder() {
        let mut builder = CatalogBuilder::new("MyLib", "0.1.0");
        builder.add(ExportRecord {
            name: "bar".to_string(),
            kind: "lemma".to_string(),
            ty: "Bool".to_string(),
            status: ProofStatus::HasSorry,
            module: "Util".to_string(),
        });
        assert_eq!(builder.count(), 1);
        let json = builder.build_json();
        assert!(json.contains("MyLib"));
        assert!(json.contains("bar"));
    }
    #[test]
    fn test_ndjson() {
        let values = vec![JsonValue::Integer(1), JsonValue::Integer(2)];
        let ndjson = write_ndjson_values(&values);
        let lines = parse_ndjson(&ndjson);
        assert_eq!(lines.len(), 2);
    }
    #[test]
    fn test_filter_by_status() {
        let records = vec![
            ExportRecord {
                name: "a".to_string(),
                kind: "theorem".to_string(),
                ty: "P".to_string(),
                status: ProofStatus::Complete,
                module: "M".to_string(),
            },
            ExportRecord {
                name: "b".to_string(),
                kind: "theorem".to_string(),
                ty: "Q".to_string(),
                status: ProofStatus::HasSorry,
                module: "M".to_string(),
            },
        ];
        let complete = filter_by_status(&records, &ProofStatus::Complete);
        assert_eq!(complete.len(), 1);
        assert_eq!(complete[0].name, "a");
    }
    #[test]
    fn test_count_per_module() {
        let records = vec![
            ExportRecord {
                name: "a".to_string(),
                kind: "t".to_string(),
                ty: "P".to_string(),
                status: ProofStatus::Complete,
                module: "A".to_string(),
            },
            ExportRecord {
                name: "b".to_string(),
                kind: "t".to_string(),
                ty: "Q".to_string(),
                status: ProofStatus::Complete,
                module: "A".to_string(),
            },
            ExportRecord {
                name: "c".to_string(),
                kind: "t".to_string(),
                ty: "R".to_string(),
                status: ProofStatus::Complete,
                module: "B".to_string(),
            },
        ];
        let counts = count_per_module(&records);
        assert_eq!(counts.get("A"), Some(&2));
        assert_eq!(counts.get("B"), Some(&1));
    }
}
#[allow(dead_code)]
pub fn json_template(template: &str, vars: &std::collections::HashMap<String, String>) -> String {
    let mut result = template.to_string();
    for (k, v) in vars {
        result = result.replace(&format!("{{{{{}}}}}", k), v);
    }
    result
}
#[allow(dead_code)]
pub fn jsonpath_query<'a>(root: &'a JsonValue, path: &str) -> Vec<&'a JsonValue> {
    if path == "$" {
        return vec![root];
    }
    let path = path.strip_prefix("$.").unwrap_or(path);
    let parts: Vec<&str> = path.splitn(2, '.').collect();
    match root {
        JsonValue::Object(map) => {
            if let Some(key) = parts.first() {
                if let Some((_, child)) = map.iter().find(|(k, _)| k == key) {
                    if parts.len() == 1 {
                        vec![child]
                    } else {
                        jsonpath_query(child, parts[1])
                    }
                } else {
                    vec![]
                }
            } else {
                vec![]
            }
        }
        JsonValue::Array(arr) => {
            if let Some(key) = parts.first() {
                if let Ok(idx) = key.parse::<usize>() {
                    if let Some(child) = arr.get(idx) {
                        if parts.len() == 1 {
                            vec![child]
                        } else {
                            jsonpath_query(child, parts[1])
                        }
                    } else {
                        vec![]
                    }
                } else {
                    arr.iter().flat_map(|v| jsonpath_query(v, path)).collect()
                }
            } else {
                vec![]
            }
        }
        _ => vec![],
    }
}
#[allow(dead_code)]
pub fn redact_json_keys(value: JsonValue, keys_to_redact: &[&str]) -> JsonValue {
    match value {
        JsonValue::Object(map) => {
            let new_map: Vec<(String, JsonValue)> = map
                .into_iter()
                .map(|(k, v)| {
                    if keys_to_redact.contains(&k.as_str()) {
                        (k, JsonValue::Str("***".to_string()))
                    } else {
                        (k, redact_json_keys(v, keys_to_redact))
                    }
                })
                .collect();
            JsonValue::Object(new_map)
        }
        JsonValue::Array(arr) => JsonValue::Array(
            arr.into_iter()
                .map(|v| redact_json_keys(v, keys_to_redact))
                .collect(),
        ),
        other => other,
    }
}
#[allow(dead_code)]
pub fn reformat_json(input: &str, indent: usize) -> String {
    let minified = minify_json(input);
    let mut out = String::new();
    let mut depth = 0usize;
    let mut in_str = false;
    let mut prev_bs = false;
    for ch in minified.chars() {
        if in_str {
            out.push(ch);
            if ch == '\\' && !prev_bs {
                prev_bs = true;
            } else if ch == '"' && !prev_bs {
                in_str = false;
                prev_bs = false;
            } else {
                prev_bs = false;
            }
        } else {
            match ch {
                '"' => {
                    in_str = true;
                    out.push(ch);
                }
                '{' | '[' => {
                    depth += 1;
                    out.push(ch);
                    out.push('\n');
                    out.push_str(&" ".repeat(indent * depth));
                }
                '}' | ']' => {
                    depth = depth.saturating_sub(1);
                    out.push('\n');
                    out.push_str(&" ".repeat(indent * depth));
                    out.push(ch);
                }
                ',' => {
                    out.push(ch);
                    out.push('\n');
                    out.push_str(&" ".repeat(indent * depth));
                }
                ':' => {
                    out.push(ch);
                    out.push(' ');
                }
                _ => {
                    out.push(ch);
                }
            }
        }
    }
    out
}
#[allow(dead_code)]
pub fn json_value_diff(a: &JsonValue, b: &JsonValue) -> Vec<String> {
    let mut diffs = Vec::new();
    match (a, b) {
        (JsonValue::Object(am), JsonValue::Object(bm)) => {
            for (k, av) in am.iter() {
                match bm.iter().find(|(bk, _)| bk == k).map(|(_, bv)| bv) {
                    None => diffs.push(format!("removed: {}", k)),
                    Some(bv) if !json_deep_equal(av, bv) => diffs.push(format!("changed: {}", k)),
                    _ => {}
                }
            }
            for (k, _) in bm.iter() {
                if !am.iter().any(|(ak, _)| ak == k) {
                    diffs.push(format!("added: {}", k));
                }
            }
        }
        _ if !json_deep_equal(a, b) => diffs.push("value differs".to_string()),
        _ => {}
    }
    diffs
}
#[allow(dead_code)]
pub fn compute_export_stats(records: &[ExportRecord]) -> ExportStats {
    let mut stats = ExportStats::default();
    stats.total = records.len();
    for r in records {
        match r.status {
            ProofStatus::Complete => stats.complete += 1,
            ProofStatus::HasSorry => stats.has_sorry += 1,
            ProofStatus::Incomplete => stats.incomplete += 1,
            ProofStatus::Axiom => stats.axioms += 1,
        }
    }
    stats
}
#[cfg(test)]
mod json_export_final_tests {
    use super::*;
    #[test]
    fn test_json_template() {
        let mut vars = std::collections::HashMap::new();
        vars.insert("name".to_string(), "OxiLean".to_string());
        let out = json_template("Hello {{name}}!", &vars);
        assert_eq!(out, "Hello OxiLean!");
    }
    #[test]
    fn test_jsonpath_simple() {
        let m = vec![("key".to_string(), JsonValue::Str("val".to_string()))];
        let root = JsonValue::Object(m);
        let results = jsonpath_query(&root, "$.key");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0], &JsonValue::Str("val".to_string()));
    }
    #[test]
    fn test_redact_json_keys() {
        let m = vec![
            ("token".to_string(), JsonValue::Str("secret".to_string())),
            ("name".to_string(), JsonValue::Str("oxilean".to_string())),
        ];
        let v = JsonValue::Object(m);
        let redacted = redact_json_keys(v, &["token"]);
        if let JsonValue::Object(map) = redacted {
            assert_eq!(
                map.iter().find(|(k, _)| k == "token").map(|(_, v)| v),
                Some(&JsonValue::Str("***".to_string()))
            );
            assert_eq!(
                map.iter().find(|(k, _)| k == "name").map(|(_, v)| v),
                Some(&JsonValue::Str("oxilean".to_string()))
            );
        } else {
            panic!("expected object");
        }
    }
    #[test]
    fn test_json_value_diff_changed() {
        let m1 = vec![("a".to_string(), JsonValue::Integer(1))];
        let m2 = vec![("a".to_string(), JsonValue::Integer(2))];
        let diffs = json_value_diff(&JsonValue::Object(m1), &JsonValue::Object(m2));
        assert!(diffs.iter().any(|d| d.contains("changed: a")));
    }
    #[test]
    fn test_compute_export_stats() {
        let records = vec![
            ExportRecord {
                name: "a".to_string(),
                kind: "t".to_string(),
                ty: "P".to_string(),
                status: ProofStatus::Complete,
                module: "M".to_string(),
            },
            ExportRecord {
                name: "b".to_string(),
                kind: "t".to_string(),
                ty: "Q".to_string(),
                status: ProofStatus::HasSorry,
                module: "M".to_string(),
            },
            ExportRecord {
                name: "c".to_string(),
                kind: "a".to_string(),
                ty: "R".to_string(),
                status: ProofStatus::Axiom,
                module: "M".to_string(),
            },
        ];
        let stats = compute_export_stats(&records);
        assert_eq!(stats.total, 3);
        assert_eq!(stats.complete, 1);
        assert_eq!(stats.has_sorry, 1);
        assert_eq!(stats.axioms, 1);
    }
    #[test]
    fn test_json_stream_decoder_feed() {
        let mut decoder = JsonStreamDecoder::new();
        let chunks = decoder.feed("{\"a\":1}\n{\"b\":2}\n");
        assert_eq!(chunks.len(), 2);
    }
    #[test]
    fn test_minify_preserves_content() {
        let input = "{ \"x\": 1, \"y\": \"hello world\" }";
        let minified = minify_json(input);
        assert!(minified.contains("\"x\""));
        assert!(minified.contains("hello world"));
        assert!(!minified.contains("  "));
    }
    #[test]
    fn test_reformat_json_basic() {
        let input = "{\"a\":1}";
        let formatted = reformat_json(input, 2);
        assert!(formatted.contains('\n'));
    }
    #[test]
    fn test_value_to_json_null() {
        let v = JsonValue::Null;
        assert_eq!(value_to_json(&v), "null");
    }
    #[test]
    fn test_value_to_json_bool() {
        assert_eq!(value_to_json(&JsonValue::Bool(true)), "true");
        assert_eq!(value_to_json(&JsonValue::Bool(false)), "false");
    }
    #[test]
    fn test_oxilean_export_schema() {
        let schema = oxilean_export_schema();
        assert!(schema.required.contains(&"version".to_string()));
        assert!(schema.properties.contains_key("declarations"));
    }
}
#[allow(dead_code)]
pub fn assert_non_empty_object(v: &JsonValue) -> bool {
    matches!(v, JsonValue::Object(m) if ! m.is_empty())
}
#[allow(dead_code)]
pub fn assert_has_keys(v: &JsonValue, keys: &[&str]) -> bool {
    match v {
        JsonValue::Object(m) => keys.iter().all(|k| m.iter().any(|(mk, _)| mk == k)),
        _ => false,
    }
}
#[allow(dead_code)]
pub fn assert_array_min_len(v: &JsonValue, min: usize) -> bool {
    matches!(v, JsonValue::Array(a) if a.len() >= min)
}
#[allow(dead_code)]
pub fn normalize_json_keys(value: JsonValue) -> JsonValue {
    match value {
        JsonValue::Object(map) => {
            let new_map: Vec<(String, JsonValue)> = map
                .into_iter()
                .map(|(k, v)| {
                    let normalized_key = k.to_lowercase().replace('-', "_").replace(' ', "_");
                    (normalized_key, normalize_json_keys(v))
                })
                .collect();
            JsonValue::Object(new_map)
        }
        JsonValue::Array(arr) => {
            JsonValue::Array(arr.into_iter().map(normalize_json_keys).collect())
        }
        other => other,
    }
}
#[allow(dead_code)]
pub fn estimate_json_size(value: &JsonValue) -> usize {
    match value {
        JsonValue::Null => 4,
        JsonValue::Bool(true) => 4,
        JsonValue::Bool(false) => 5,
        JsonValue::Integer(n) => n.to_string().len(),
        JsonValue::Float(f) => format!("{:.6}", f).len(),
        JsonValue::Str(s) => s.len() + 2,
        JsonValue::Array(arr) => {
            2 + arr.iter().map(estimate_json_size).sum::<usize>() + arr.len().saturating_sub(1)
        }
        JsonValue::Object(map) => {
            2 + map
                .iter()
                .map(|(k, v)| k.len() + 3 + estimate_json_size(v))
                .sum::<usize>()
                + map.len().saturating_sub(1)
        }
    }
}
#[cfg(test)]
mod json_export_index_tests {
    use super::*;
    #[test]
    fn test_catalog_index_lookup() {
        let records = vec![
            ExportRecord {
                name: "foo".to_string(),
                kind: "theorem".to_string(),
                ty: "P".to_string(),
                status: ProofStatus::Complete,
                module: "Algebra".to_string(),
            },
            ExportRecord {
                name: "bar".to_string(),
                kind: "lemma".to_string(),
                ty: "Q".to_string(),
                status: ProofStatus::Complete,
                module: "Algebra".to_string(),
            },
            ExportRecord {
                name: "baz".to_string(),
                kind: "theorem".to_string(),
                ty: "R".to_string(),
                status: ProofStatus::Complete,
                module: "Logic".to_string(),
            },
        ];
        let idx = JsonCatalogIndex::build(&records);
        assert_eq!(idx.lookup_by_module("Algebra").len(), 2);
        assert_eq!(idx.lookup_by_kind("theorem").len(), 2);
        assert_eq!(idx.lookup_by_module("Logic").len(), 1);
    }
    #[test]
    fn test_normalize_json_keys() {
        let m = vec![("My-Key".to_string(), JsonValue::Str("v".to_string()))];
        let v = JsonValue::Object(m);
        let result = normalize_json_keys(v);
        if let JsonValue::Object(map) = result {
            assert!(map.iter().any(|(k, _)| k == "my_key"));
        } else {
            panic!("expected object");
        }
    }
    #[test]
    fn test_estimate_json_size_null() {
        assert_eq!(estimate_json_size(&JsonValue::Null), 4);
    }
    #[test]
    fn test_estimate_json_size_string() {
        let s = JsonValue::Str("hello".to_string());
        assert_eq!(estimate_json_size(&s), 7);
    }
    #[test]
    fn test_assert_has_keys_true() {
        let m = vec![
            ("a".to_string(), JsonValue::Null),
            ("b".to_string(), JsonValue::Null),
        ];
        let v = JsonValue::Object(m);
        assert!(assert_has_keys(&v, &["a", "b"]));
    }
    #[test]
    fn test_assert_has_keys_false() {
        let m: Vec<(String, JsonValue)> = Vec::new();
        let v = JsonValue::Object(m);
        assert!(!assert_has_keys(&v, &["missing"]));
    }
    #[test]
    fn test_assert_array_min_len() {
        let v = JsonValue::Array(vec![JsonValue::Null; 5]);
        assert!(assert_array_min_len(&v, 3));
        assert!(!assert_array_min_len(&v, 10));
    }
}
#[allow(dead_code)]
pub fn json_merge_objects(
    a: std::collections::HashMap<String, JsonValue>,
    b: std::collections::HashMap<String, JsonValue>,
) -> std::collections::HashMap<String, JsonValue> {
    let mut result = a;
    for (k, v) in b {
        result.insert(k, v);
    }
    result
}
#[allow(dead_code)]
pub fn json_flatten(value: &JsonValue, prefix: &str) -> std::collections::HashMap<String, String> {
    let mut out = std::collections::HashMap::new();
    match value {
        JsonValue::Object(map) => {
            for (k, v) in map {
                let new_prefix = if prefix.is_empty() {
                    k.clone()
                } else {
                    format!("{}.{}", prefix, k)
                };
                let inner = json_flatten(v, &new_prefix);
                out.extend(inner);
            }
        }
        JsonValue::Array(arr) => {
            for (i, v) in arr.iter().enumerate() {
                let new_prefix = format!("{}[{}]", prefix, i);
                let inner = json_flatten(v, &new_prefix);
                out.extend(inner);
            }
        }
        other => {
            out.insert(prefix.to_string(), value_to_json(other));
        }
    }
    out
}
#[allow(dead_code)]
pub fn json_version() -> &'static str {
    "1.0.0"
}
#[cfg(test)]
mod json_final_tests {
    use super::*;
    #[test]
    fn test_json_merge_objects() {
        let mut a = std::collections::HashMap::new();
        a.insert("x".to_string(), JsonValue::Integer(1));
        let mut b = std::collections::HashMap::new();
        b.insert("y".to_string(), JsonValue::Integer(2));
        let merged = json_merge_objects(a, b);
        assert!(merged.contains_key("x"));
        assert!(merged.contains_key("y"));
    }
    #[test]
    fn test_json_flatten() {
        let m = vec![
            ("a".to_string(), JsonValue::Integer(1)),
            ("b".to_string(), JsonValue::Str("hello".to_string())),
        ];
        let v = JsonValue::Object(m);
        let flat = json_flatten(&v, "");
        assert!(flat.contains_key("a"));
        assert!(flat.contains_key("b"));
    }
    #[test]
    fn test_json_version() {
        assert!(!json_version().is_empty());
    }
    #[test]
    fn test_schema_kind_object() {
        let s = JsonSchema::object();
        assert!(matches!(s.kind, JsonSchemaKind::Object));
    }
    #[test]
    fn test_patch_op_name() {
        let op = JsonPatchOp::Remove {
            path: "/foo".to_string(),
        };
        assert_eq!(op.op_name(), "remove");
    }
    #[test]
    fn test_stream_writer() {
        let mut buf = Vec::new();
        {
            let mut writer = StreamJsonWriter::new(&mut buf);
            writer.begin_array().expect("I/O operation should succeed");
            writer
                .write_item("{\"a\":1}")
                .expect("I/O operation should succeed");
            writer
                .write_item("{\"b\":2}")
                .expect("I/O operation should succeed");
            writer.end_array().expect("I/O operation should succeed");
            assert_eq!(writer.item_count(), 2);
        }
        let s = String::from_utf8(buf).expect("test operation should succeed");
        assert!(s.starts_with('['));
        assert!(s.ends_with(']'));
    }
}
#[allow(dead_code)]
pub fn json_null() -> JsonValue {
    JsonValue::Null
}
#[allow(dead_code)]
pub fn json_true() -> JsonValue {
    JsonValue::Bool(true)
}
#[allow(dead_code)]
pub fn json_false() -> JsonValue {
    JsonValue::Bool(false)
}
#[allow(dead_code)]
pub fn json_string(s: &str) -> JsonValue {
    JsonValue::Str(s.to_string())
}
#[allow(dead_code)]
pub fn json_number(n: i64) -> JsonValue {
    JsonValue::Integer(n)
}
#[allow(dead_code)]
pub fn json_float(f: f64) -> JsonValue {
    JsonValue::Float(f)
}
#[allow(dead_code)]
pub fn json_array(items: Vec<JsonValue>) -> JsonValue {
    JsonValue::Array(items)
}
#[allow(dead_code)]
pub fn json_object(fields: Vec<(String, JsonValue)>) -> JsonValue {
    JsonValue::Object(fields.into_iter().collect())
}
#[allow(dead_code)]
pub fn json_is_truthy(v: &JsonValue) -> bool {
    !matches!(v, JsonValue::Null | JsonValue::Bool(false))
}
#[allow(dead_code)]
pub fn json_as_string(v: &JsonValue) -> Option<&str> {
    if let JsonValue::Str(s) = v {
        Some(s.as_str())
    } else {
        None
    }
}
#[allow(dead_code)]
pub fn json_as_number(v: &JsonValue) -> Option<i64> {
    if let JsonValue::Integer(n) = v {
        Some(*n)
    } else {
        None
    }
}
#[allow(dead_code)]
pub fn json_as_bool(v: &JsonValue) -> Option<bool> {
    if let JsonValue::Bool(b) = v {
        Some(*b)
    } else {
        None
    }
}
#[cfg(test)]
mod json_helpers_tests {
    use super::*;
    #[test]
    fn test_json_helpers() {
        assert!(json_is_truthy(&json_true()));
        assert!(!json_is_truthy(&json_null()));
        assert_eq!(json_as_string(&json_string("hi")), Some("hi"));
        assert_eq!(json_as_number(&json_number(42)), Some(42));
        assert_eq!(json_as_bool(&json_true()), Some(true));
    }
}
#[allow(dead_code)]
pub fn json_export_supports_patch() -> bool {
    true
}
#[allow(dead_code)]
pub fn json_export_supports_schema() -> bool {
    true
}
#[allow(dead_code)]
pub fn json_export_supports_ndjson() -> bool {
    true
}
#[allow(dead_code)]
pub fn json_export_supports_streaming() -> bool {
    true
}
#[allow(dead_code)]
pub fn json_export_supports_redaction() -> bool {
    true
}
#[allow(dead_code)]
pub fn json_export_version() -> &'static str {
    "2.0.0"
}
#[allow(dead_code)]
pub const JSON_EXPORT_MAX_DEPTH: usize = 100;
#[allow(dead_code)]
pub const JSON_EXPORT_MAX_KEYS: usize = 10_000;
#[allow(dead_code)]
pub const JSON_EXPORT_MAX_ARRAY_LEN: usize = 100_000;
#[allow(dead_code)]
pub const JSON_EXPORT_DEFAULT_INDENT: usize = 2;
#[allow(dead_code)]
pub const JSON_EXPORT_REDACT_MARKER: &str = "***";
#[allow(dead_code)]
pub const JSON_EXPORT_NDJSON_SEPARATOR: char = '\n';
#[allow(dead_code)]
pub const JSON_EXPORT_SCHEMA_VERSION: &str = "1.0";
#[allow(dead_code)]
pub const JSON_EXPORT_FORMAT: &str = "json";
#[allow(dead_code)]
pub const JSON_EXPORT_ENCODING: &str = "utf-8";
#[allow(dead_code)]
pub const JSON_EXPORT_AUTHOR: &str = "oxilean";
