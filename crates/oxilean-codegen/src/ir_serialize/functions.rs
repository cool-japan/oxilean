//! Functions for the LCNF IR serialization module.

use std::collections::HashMap;

use super::types::{
    DeclKind, IrDeserializeResult, IrFormat, IrSerializeConfig, SerialDecl, SerializedIr,
};

// ── Current format version ────────────────────────────────────────────────────

/// The version number embedded into every freshly serialized snapshot.
pub const CURRENT_VERSION: u32 = 1;

// ── Top-level serialize / deserialize ────────────────────────────────────────

/// Serialize `module` to a string using the format specified in `cfg`.
pub fn serialize_ir(module: &SerializedIr, cfg: &IrSerializeConfig) -> String {
    match cfg.format {
        IrFormat::Text => ir_to_text(module),
        IrFormat::Json => ir_to_json(module),
        IrFormat::Binary => ir_to_binary_text(module),
    }
}

/// Deserialize the text `data` that was produced in `format`.
///
/// Only [`IrFormat::Text`] and [`IrFormat::Json`] are fully supported.
/// [`IrFormat::Binary`] returns [`IrDeserializeResult::Unsupported`].
pub fn deserialize_ir(data: &str, format: IrFormat) -> IrDeserializeResult {
    match format {
        IrFormat::Text => match ir_from_text(data) {
            Ok(ir) => {
                if ir.version != CURRENT_VERSION {
                    IrDeserializeResult::VersionMismatch {
                        expected: CURRENT_VERSION,
                        found: ir.version,
                    }
                } else {
                    IrDeserializeResult::Ok(ir)
                }
            }
            Err(e) => IrDeserializeResult::ParseError(e),
        },
        IrFormat::Json => match ir_from_json(data) {
            Ok(ir) => {
                if ir.version != CURRENT_VERSION {
                    IrDeserializeResult::VersionMismatch {
                        expected: CURRENT_VERSION,
                        found: ir.version,
                    }
                } else {
                    IrDeserializeResult::Ok(ir)
                }
            }
            Err(e) => IrDeserializeResult::ParseError(e),
        },
        IrFormat::Binary => {
            IrDeserializeResult::Unsupported("binary format is write-only in this version".into())
        }
    }
}

/// Serialize a single declaration according to `cfg`.
pub fn serialize_decl(decl: &SerialDecl, cfg: &IrSerializeConfig) -> String {
    let mut out = String::new();
    out.push_str(&format!("decl {} {}", decl.kind, decl.name));
    if cfg.include_types {
        out.push_str(&format!(" : {}", decl.type_));
    }
    if !decl.params.is_empty() {
        out.push_str(&format!(" params=[{}]", decl.params.join(", ")));
    }
    if cfg.include_proofs {
        if let Some(body) = &decl.body {
            if cfg.pretty {
                out.push_str(&format!("\n  body: {}", body));
            } else {
                out.push_str(&format!(" body={}", body));
            }
        }
    }
    out
}

// ── JSON format ───────────────────────────────────────────────────────────────

/// Serialize `module` to a JSON string (always pretty-printed).
pub fn ir_to_json(module: &SerializedIr) -> String {
    let mut out = String::new();
    out.push_str("{\n");
    out.push_str(&format!("  \"version\": {},\n", module.version));
    out.push_str(&format!(
        "  \"module_name\": \"{}\",\n",
        escape_json(&module.module_name)
    ));
    // metadata
    out.push_str("  \"metadata\": {\n");
    let meta_entries: Vec<String> = module
        .metadata
        .iter()
        .map(|(k, v)| format!("    \"{}\": \"{}\"", escape_json(k), escape_json(v)))
        .collect();
    out.push_str(&meta_entries.join(",\n"));
    if !meta_entries.is_empty() {
        out.push('\n');
    }
    out.push_str("  },\n");
    // declarations
    out.push_str("  \"declarations\": [\n");
    let decl_strs: Vec<String> = module
        .declarations
        .iter()
        .map(|d| serialize_decl_json(d))
        .collect();
    out.push_str(&decl_strs.join(",\n"));
    if !decl_strs.is_empty() {
        out.push('\n');
    }
    out.push_str("  ]\n");
    out.push('}');
    out
}

fn serialize_decl_json(d: &SerialDecl) -> String {
    let body_str = match &d.body {
        Some(b) => format!("\"{}\"", escape_json(b)),
        None => "null".to_string(),
    };
    let params_str: Vec<String> = d
        .params
        .iter()
        .map(|p| format!("\"{}\"", escape_json(p)))
        .collect();
    format!(
        "    {{\"name\": \"{}\", \"kind\": \"{}\", \"type\": \"{}\", \"body\": {}, \"params\": [{}]}}",
        escape_json(&d.name),
        d.kind,
        escape_json(&d.type_),
        body_str,
        params_str.join(", ")
    )
}

/// Deserialize a JSON-encoded IR snapshot produced by [`ir_to_json`].
pub fn ir_from_json(s: &str) -> Result<SerializedIr, String> {
    // Minimal hand-rolled JSON parser sufficient for our own output format.
    let s = s.trim();
    if !s.starts_with('{') {
        return Err("expected JSON object".into());
    }

    let version = parse_json_u32(s, "version")
        .ok_or_else(|| "missing or invalid 'version' field".to_string())?;
    let module_name = parse_json_string(s, "module_name")
        .ok_or_else(|| "missing 'module_name' field".to_string())?;
    let metadata = parse_json_metadata(s);
    let declarations = parse_json_declarations(s)?;

    Ok(SerializedIr {
        version,
        module_name,
        declarations,
        metadata,
    })
}

// ── Text format ───────────────────────────────────────────────────────────────

/// Serialize `module` to a human-readable text format.
pub fn ir_to_text(module: &SerializedIr) -> String {
    let mut out = String::new();
    out.push_str(&format!("-- OxiLean IR version {}\n", module.version));
    out.push_str(&format!("module {}\n", module.module_name));
    for (k, v) in &module.metadata {
        out.push_str(&format!("-- meta {} = {}\n", k, v));
    }
    out.push('\n');
    for decl in &module.declarations {
        out.push_str(&format!("{} {} : {}", decl.kind, decl.name, decl.type_));
        if !decl.params.is_empty() {
            out.push_str(&format!("  -- params: {}", decl.params.join(", ")));
        }
        out.push('\n');
        if let Some(body) = &decl.body {
            out.push_str(&format!("  := {}\n", body));
        }
    }
    out
}

/// Deserialize the text format produced by [`ir_to_text`].
pub fn ir_from_text(s: &str) -> Result<SerializedIr, String> {
    let mut version: Option<u32> = None;
    let mut module_name: Option<String> = None;
    let mut metadata: HashMap<String, String> = HashMap::new();
    let mut declarations: Vec<SerialDecl> = Vec::new();

    let mut pending_decl: Option<SerialDecl> = None;

    for line in s.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if let Some(rest) = line.strip_prefix("-- OxiLean IR version ") {
            let v: u32 = rest
                .trim()
                .parse()
                .map_err(|_| format!("invalid version '{}'", rest.trim()))?;
            version = Some(v);
        } else if let Some(rest) = line.strip_prefix("-- meta ") {
            if let Some((k, v)) = rest.split_once(" = ") {
                metadata.insert(k.trim().to_string(), v.trim().to_string());
            }
        } else if let Some(rest) = line.strip_prefix("module ") {
            module_name = Some(rest.trim().to_string());
        } else if let Some(rest) = line.strip_prefix(":= ") {
            if let Some(ref mut d) = pending_decl {
                // Strip trailing params comment if present
                let body = if let Some((b, _)) = rest.split_once("  --") {
                    b.trim().to_string()
                } else {
                    rest.trim().to_string()
                };
                d.body = Some(body);
            }
        } else if line.starts_with("--") {
            // ignore other comments
        } else {
            // Try to parse a declaration line: `<kind> <name> : <type>  -- params: ...`
            if let Some(d) = parse_text_decl_line(line) {
                if let Some(prev) = pending_decl.take() {
                    declarations.push(prev);
                }
                pending_decl = Some(d);
            }
        }
    }
    if let Some(d) = pending_decl {
        declarations.push(d);
    }

    Ok(SerializedIr {
        version: version.unwrap_or(CURRENT_VERSION),
        module_name: module_name.unwrap_or_default(),
        declarations,
        metadata,
    })
}

// ── Module operations ─────────────────────────────────────────────────────────

/// Merge two modules into one.  Returns an error if any declaration name
/// appears in both `a` and `b`.
pub fn merge_modules(mut a: SerializedIr, b: SerializedIr) -> Result<SerializedIr, String> {
    let a_names: std::collections::HashSet<&str> =
        a.declarations.iter().map(|d| d.name.as_str()).collect();
    for d in &b.declarations {
        if a_names.contains(d.name.as_str()) {
            return Err(format!(
                "name collision: '{}' exists in both modules",
                d.name
            ));
        }
    }
    a.declarations.extend(b.declarations);
    for (k, v) in b.metadata {
        a.metadata.entry(k).or_insert(v);
    }
    Ok(a)
}

/// Compute the diff between two module snapshots.
///
/// Returns `(added, removed, modified)` — each a `Vec<String>` of declaration
/// names.  A declaration is "modified" when the name exists in both modules but
/// the [`SerialDecl`] differs in any field.
pub fn diff_modules(
    old: &SerializedIr,
    new: &SerializedIr,
) -> (Vec<String>, Vec<String>, Vec<String>) {
    let old_map: HashMap<&str, &SerialDecl> = old
        .declarations
        .iter()
        .map(|d| (d.name.as_str(), d))
        .collect();
    let new_map: HashMap<&str, &SerialDecl> = new
        .declarations
        .iter()
        .map(|d| (d.name.as_str(), d))
        .collect();

    let mut added: Vec<String> = Vec::new();
    let mut removed: Vec<String> = Vec::new();
    let mut modified: Vec<String> = Vec::new();

    for (name, new_decl) in &new_map {
        match old_map.get(name) {
            None => added.push(name.to_string()),
            Some(old_decl) => {
                if old_decl != new_decl {
                    modified.push(name.to_string());
                }
            }
        }
    }
    for name in old_map.keys() {
        if !new_map.contains_key(name) {
            removed.push(name.to_string());
        }
    }

    added.sort();
    removed.sort();
    modified.sort();
    (added, removed, modified)
}

// ── Helpers ───────────────────────────────────────────────────────────────────

/// Escape special JSON characters inside a string value.
fn escape_json(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            other => out.push(other),
        }
    }
    out
}

/// Produce a simple binary-like text representation (base64-ish stub).
/// Real binary encoding would use a proper byte stream; this placeholder
/// keeps the format round-trip representable in a `String`.
fn ir_to_binary_text(module: &SerializedIr) -> String {
    // For the binary format we emit a compact single-line text that looks like
    // length-prefixed data while remaining valid UTF-8.
    let inner = ir_to_text(module);
    let encoded: String = inner
        .bytes()
        .map(|b| format!("{:02x}", b))
        .collect::<Vec<_>>()
        .join("");
    format!("OXI_BIN_V{}:{}", module.version, encoded)
}

/// Extract a `u32` field from a very simple JSON object string.
fn parse_json_u32(json: &str, key: &str) -> Option<u32> {
    let needle = format!("\"{}\":", key);
    let start = json.find(&needle)? + needle.len();
    let rest = json[start..].trim_start();
    let end = rest
        .find(|c: char| !c.is_ascii_digit())
        .unwrap_or(rest.len());
    rest[..end].parse().ok()
}

/// Extract a string field from a very simple JSON object string.
fn parse_json_string(json: &str, key: &str) -> Option<String> {
    let needle = format!("\"{}\":", key);
    let start = json.find(&needle)? + needle.len();
    let rest = json[start..].trim_start();
    if !rest.starts_with('"') {
        return None;
    }
    Some(unescape_json_string(&rest[1..])?.0)
}

/// Unescape a JSON string starting just after the opening `"`.
/// Returns `(unescaped, chars_consumed_including_closing_quote)`.
fn unescape_json_string(s: &str) -> Option<(String, usize)> {
    let mut out = String::new();
    let mut chars = s.char_indices();
    loop {
        let (i, c) = chars.next()?;
        match c {
            '"' => return Some((out, i + 1)),
            '\\' => {
                let (_, esc) = chars.next()?;
                match esc {
                    '"' => out.push('"'),
                    '\\' => out.push('\\'),
                    'n' => out.push('\n'),
                    'r' => out.push('\r'),
                    't' => out.push('\t'),
                    other => {
                        out.push('\\');
                        out.push(other);
                    }
                }
            }
            other => out.push(other),
        }
    }
}

/// Parse the `"metadata"` object from a JSON string.
fn parse_json_metadata(json: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();
    let needle = "\"metadata\":";
    let start = match json.find(needle) {
        Some(p) => p + needle.len(),
        None => return map,
    };
    let rest = json[start..].trim_start();
    if !rest.starts_with('{') {
        return map;
    }
    let inner = &rest[1..];
    // iterate over key-value pairs
    let mut pos = 0;
    loop {
        let chunk = inner[pos..].trim_start();
        if chunk.starts_with('}') || chunk.is_empty() {
            break;
        }
        if chunk.starts_with(',') {
            pos += inner[pos..].find(',').map(|x| x + 1).unwrap_or(1);
            continue;
        }
        if let Some(chunk_after_quote) = chunk.strip_prefix('"') {
            // parse key
            match unescape_json_string(chunk_after_quote) {
                Some((key, k_consumed)) => {
                    let after_key = chunk[1 + k_consumed..].trim_start();
                    if !after_key.starts_with(':') {
                        break;
                    }
                    let after_colon = after_key[1..].trim_start();
                    if let Some(after_colon_stripped) = after_colon.strip_prefix('"') {
                        match unescape_json_string(after_colon_stripped) {
                            Some((val, _)) => {
                                map.insert(key, val);
                            }
                            None => break,
                        }
                    }
                    // advance pos past this pair
                    match inner[pos..].find([',', '}']) {
                        Some(adv) => pos += adv + 1,
                        None => break,
                    }
                }
                None => break,
            }
        } else {
            pos += 1;
        }
    }
    map
}

/// Parse the `"declarations"` array from a JSON string.
fn parse_json_declarations(json: &str) -> Result<Vec<SerialDecl>, String> {
    let mut decls = Vec::new();
    let needle = "\"declarations\":";
    let start = match json.find(needle) {
        Some(p) => p + needle.len(),
        None => return Ok(decls),
    };
    let rest = json[start..].trim_start();
    if !rest.starts_with('[') {
        return Ok(decls);
    }
    // Find each declaration object by looking for `{...}` blocks.
    let mut depth = 0i32;
    let mut obj_start: Option<usize> = None;
    let chars: Vec<(usize, char)> = rest.char_indices().collect();
    for &(i, c) in &chars {
        match c {
            '{' => {
                depth += 1;
                if depth == 1 {
                    obj_start = Some(i);
                }
            }
            '}' => {
                depth -= 1;
                if depth == 0 {
                    if let Some(s) = obj_start.take() {
                        let obj = &rest[s..=i];
                        match parse_decl_json_obj(obj) {
                            Ok(d) => decls.push(d),
                            Err(e) => return Err(e),
                        }
                    }
                }
            }
            ']' if depth == 0 => break,
            _ => {}
        }
    }
    Ok(decls)
}

fn parse_decl_json_obj(obj: &str) -> Result<SerialDecl, String> {
    let name =
        parse_json_string(obj, "name").ok_or_else(|| "declaration missing 'name'".to_string())?;
    let kind_str =
        parse_json_string(obj, "kind").ok_or_else(|| "declaration missing 'kind'".to_string())?;
    let type_ = parse_json_string(obj, "type").unwrap_or_default();
    let body = parse_json_nullable_string(obj, "body");
    let params = parse_json_string_array(obj, "params");
    let kind = parse_decl_kind(&kind_str)?;
    Ok(SerialDecl {
        name,
        kind,
        type_,
        body,
        params,
    })
}

/// Parse a possibly-null JSON string field.
fn parse_json_nullable_string(json: &str, key: &str) -> Option<String> {
    let needle = format!("\"{}\":", key);
    let start = json.find(&needle)? + needle.len();
    let rest = json[start..].trim_start();
    if rest.starts_with("null") {
        return None;
    }
    if let Some(rest_stripped) = rest.strip_prefix('"') {
        return unescape_json_string(rest_stripped).map(|(s, _)| s);
    }
    None
}

/// Parse a JSON array of strings for the given key.
fn parse_json_string_array(json: &str, key: &str) -> Vec<String> {
    let needle = format!("\"{}\":", key);
    let start = match json.find(&needle) {
        Some(p) => p + needle.len(),
        None => return Vec::new(),
    };
    let rest = json[start..].trim_start();
    if !rest.starts_with('[') {
        return Vec::new();
    }
    let inner = &rest[1..];
    let mut result = Vec::new();
    let mut pos = 0;
    loop {
        let chunk = inner[pos..].trim_start();
        if chunk.starts_with(']') || chunk.is_empty() {
            break;
        }
        if chunk.starts_with(',') {
            pos += 1;
            continue;
        }
        if let Some(chunk_after_quote) = chunk.strip_prefix('"') {
            match unescape_json_string(chunk_after_quote) {
                Some((s, consumed)) => {
                    result.push(s);
                    // advance pos
                    let skip = inner[pos..].trim_start().len();
                    let leading = inner[pos..].len() - skip;
                    pos += leading + 1 + consumed;
                }
                None => break,
            }
        } else {
            pos += 1;
        }
    }
    result
}

/// Parse a text-format declaration line.
fn parse_text_decl_line(line: &str) -> Option<SerialDecl> {
    // Possible forms:
    //   `<kind> <name> : <type>  -- params: p1, p2`
    //   `<kind> <name> : <type>`
    let line = if let Some((main, _comment)) = line.split_once("  --") {
        main.trim()
    } else {
        line
    };

    let (params_raw, rest) = if let Some((r, p)) = line.split_once("  -- params: ") {
        (
            p.split(',')
                .map(|s| s.trim().to_string())
                .collect::<Vec<_>>(),
            r,
        )
    } else {
        (Vec::new(), line)
    };

    // Split on first space to get kind token
    let (kind_token, after_kind) = rest.split_once(' ')?;
    let kind = parse_decl_kind(kind_token).ok()?;
    // Split on " : " to get name and type
    let (name, type_) = after_kind.split_once(" : ")?;
    Some(SerialDecl {
        name: name.trim().to_string(),
        kind,
        type_: type_.trim().to_string(),
        body: None,
        params: params_raw,
    })
}

/// Convert a string into a [`DeclKind`].
fn parse_decl_kind(s: &str) -> Result<DeclKind, String> {
    match s {
        "def" => Ok(DeclKind::Def),
        "theorem" => Ok(DeclKind::Theorem),
        "axiom" => Ok(DeclKind::Axiom),
        "inductive" => Ok(DeclKind::Inductive),
        "constructor" => Ok(DeclKind::Constructor),
        "recursor" => Ok(DeclKind::Recursor),
        other => Err(format!("unknown DeclKind '{}'", other)),
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::super::types::{DeclKind, IrFormat, IrSerializeConfig, SerialDecl, SerializedIr};
    use super::*;
    use std::collections::HashMap;

    fn sample_ir() -> SerializedIr {
        let mut meta = HashMap::new();
        meta.insert("compiler".to_string(), "oxilean-0.1.2".to_string());
        SerializedIr {
            version: CURRENT_VERSION,
            module_name: "TestModule".to_string(),
            declarations: vec![
                SerialDecl {
                    name: "myDef".to_string(),
                    kind: DeclKind::Def,
                    type_: "Nat".to_string(),
                    body: Some("42".to_string()),
                    params: vec![],
                },
                SerialDecl {
                    name: "myTheorem".to_string(),
                    kind: DeclKind::Theorem,
                    type_: "a = a".to_string(),
                    body: Some("rfl".to_string()),
                    params: vec!["a".to_string()],
                },
            ],
            metadata: meta,
        }
    }

    // ── serialize_ir / deserialize_ir ─────────────────────────────────────────

    #[test]
    fn test_serialize_ir_text_roundtrip() {
        let ir = sample_ir();
        let cfg = IrSerializeConfig {
            format: IrFormat::Text,
            ..Default::default()
        };
        let text = serialize_ir(&ir, &cfg);
        let result = deserialize_ir(&text, IrFormat::Text);
        match result {
            IrDeserializeResult::Ok(deserialized) => {
                assert_eq!(deserialized.module_name, ir.module_name);
                assert_eq!(deserialized.declarations.len(), ir.declarations.len());
            }
            other => panic!("expected Ok, got {:?}", other),
        }
    }

    #[test]
    fn test_serialize_ir_json_roundtrip() {
        let ir = sample_ir();
        let cfg = IrSerializeConfig {
            format: IrFormat::Json,
            ..Default::default()
        };
        let json = serialize_ir(&ir, &cfg);
        let result = deserialize_ir(&json, IrFormat::Json);
        match result {
            IrDeserializeResult::Ok(deserialized) => {
                assert_eq!(deserialized.module_name, ir.module_name);
                assert_eq!(deserialized.version, CURRENT_VERSION);
            }
            other => panic!("expected Ok, got {:?}", other),
        }
    }

    #[test]
    fn test_deserialize_binary_returns_unsupported() {
        let result = deserialize_ir("some data", IrFormat::Binary);
        assert!(matches!(result, IrDeserializeResult::Unsupported(_)));
    }

    #[test]
    fn test_deserialize_invalid_text_returns_parse_error() {
        // Not a parse error in our lenient parser — it produces an empty module.
        // A version string with garbage IS a parse error.
        let result = deserialize_ir("-- OxiLean IR version abc\nmodule Foo", IrFormat::Text);
        assert!(matches!(result, IrDeserializeResult::ParseError(_)));
    }

    // ── ir_to_json / ir_from_json ─────────────────────────────────────────────

    #[test]
    fn test_ir_to_json_contains_module_name() {
        let ir = sample_ir();
        let json = ir_to_json(&ir);
        assert!(json.contains("TestModule"));
    }

    #[test]
    fn test_ir_to_json_contains_version() {
        let ir = sample_ir();
        let json = ir_to_json(&ir);
        assert!(json.contains(&CURRENT_VERSION.to_string()));
    }

    #[test]
    fn test_ir_from_json_parses_version() {
        let ir = sample_ir();
        let json = ir_to_json(&ir);
        let parsed = ir_from_json(&json).expect("parse failed");
        assert_eq!(parsed.version, CURRENT_VERSION);
    }

    #[test]
    fn test_ir_from_json_parses_declarations() {
        let ir = sample_ir();
        let json = ir_to_json(&ir);
        let parsed = ir_from_json(&json).expect("parse failed");
        assert_eq!(parsed.declarations.len(), 2);
    }

    #[test]
    fn test_ir_from_json_decl_names() {
        let ir = sample_ir();
        let json = ir_to_json(&ir);
        let parsed = ir_from_json(&json).expect("parse failed");
        let names: Vec<_> = parsed
            .declarations
            .iter()
            .map(|d| d.name.as_str())
            .collect();
        assert!(names.contains(&"myDef"));
        assert!(names.contains(&"myTheorem"));
    }

    #[test]
    fn test_ir_from_json_decl_kind() {
        let ir = sample_ir();
        let json = ir_to_json(&ir);
        let parsed = ir_from_json(&json).expect("parse failed");
        let def_decl = parsed
            .declarations
            .iter()
            .find(|d| d.name == "myDef")
            .expect("myDef not found");
        assert_eq!(def_decl.kind, DeclKind::Def);
    }

    #[test]
    fn test_ir_from_json_error_on_empty() {
        assert!(ir_from_json("").is_err());
    }

    // ── ir_to_text / ir_from_text ─────────────────────────────────────────────

    #[test]
    fn test_ir_to_text_contains_module() {
        let ir = sample_ir();
        let text = ir_to_text(&ir);
        assert!(text.contains("module TestModule"));
    }

    #[test]
    fn test_ir_to_text_contains_def() {
        let ir = sample_ir();
        let text = ir_to_text(&ir);
        assert!(text.contains("def myDef"));
    }

    #[test]
    fn test_ir_from_text_roundtrip_module_name() {
        let ir = sample_ir();
        let text = ir_to_text(&ir);
        let parsed = ir_from_text(&text).expect("parse failed");
        assert_eq!(parsed.module_name, "TestModule");
    }

    #[test]
    fn test_ir_from_text_roundtrip_decl_count() {
        let ir = sample_ir();
        let text = ir_to_text(&ir);
        let parsed = ir_from_text(&text).expect("parse failed");
        assert_eq!(parsed.declarations.len(), 2);
    }

    #[test]
    fn test_ir_from_text_roundtrip_body() {
        let ir = sample_ir();
        let text = ir_to_text(&ir);
        let parsed = ir_from_text(&text).expect("parse failed");
        let d = parsed
            .declarations
            .iter()
            .find(|d| d.name == "myDef")
            .expect("myDef not found");
        assert_eq!(d.body, Some("42".to_string()));
    }

    // ── serialize_decl ────────────────────────────────────────────────────────

    #[test]
    fn test_serialize_decl_with_type() {
        let d = SerialDecl {
            name: "foo".to_string(),
            kind: DeclKind::Axiom,
            type_: "Prop".to_string(),
            body: None,
            params: vec![],
        };
        let cfg = IrSerializeConfig::default();
        let s = serialize_decl(&d, &cfg);
        assert!(s.contains("axiom foo"));
        assert!(s.contains("Prop"));
    }

    #[test]
    fn test_serialize_decl_no_type() {
        let d = SerialDecl {
            name: "bar".to_string(),
            kind: DeclKind::Def,
            type_: "Int".to_string(),
            body: Some("0".to_string()),
            params: vec![],
        };
        let cfg = IrSerializeConfig {
            include_types: false,
            ..Default::default()
        };
        let s = serialize_decl(&d, &cfg);
        assert!(!s.contains("Int"));
    }

    #[test]
    fn test_serialize_decl_no_proof() {
        let d = SerialDecl {
            name: "thm".to_string(),
            kind: DeclKind::Theorem,
            type_: "True".to_string(),
            body: Some("trivial".to_string()),
            params: vec![],
        };
        let cfg = IrSerializeConfig {
            include_proofs: false,
            ..Default::default()
        };
        let s = serialize_decl(&d, &cfg);
        assert!(!s.contains("trivial"));
    }

    // ── merge_modules ─────────────────────────────────────────────────────────

    #[test]
    fn test_merge_modules_success() {
        let a = SerializedIr {
            version: CURRENT_VERSION,
            module_name: "A".to_string(),
            declarations: vec![SerialDecl {
                name: "x".to_string(),
                kind: DeclKind::Def,
                type_: "Nat".to_string(),
                body: None,
                params: vec![],
            }],
            metadata: HashMap::new(),
        };
        let b = SerializedIr {
            version: CURRENT_VERSION,
            module_name: "B".to_string(),
            declarations: vec![SerialDecl {
                name: "y".to_string(),
                kind: DeclKind::Def,
                type_: "Int".to_string(),
                body: None,
                params: vec![],
            }],
            metadata: HashMap::new(),
        };
        let merged = merge_modules(a, b).expect("merge failed");
        assert_eq!(merged.declarations.len(), 2);
    }

    #[test]
    fn test_merge_modules_collision() {
        let decl = SerialDecl {
            name: "collision".to_string(),
            kind: DeclKind::Def,
            type_: "Nat".to_string(),
            body: None,
            params: vec![],
        };
        let a = SerializedIr {
            version: CURRENT_VERSION,
            module_name: "A".to_string(),
            declarations: vec![decl.clone()],
            metadata: HashMap::new(),
        };
        let b = SerializedIr {
            version: CURRENT_VERSION,
            module_name: "B".to_string(),
            declarations: vec![decl],
            metadata: HashMap::new(),
        };
        assert!(merge_modules(a, b).is_err());
    }

    // ── diff_modules ──────────────────────────────────────────────────────────

    #[test]
    fn test_diff_added() {
        let old = SerializedIr {
            version: CURRENT_VERSION,
            module_name: "M".to_string(),
            declarations: vec![],
            metadata: HashMap::new(),
        };
        let new = SerializedIr {
            version: CURRENT_VERSION,
            module_name: "M".to_string(),
            declarations: vec![SerialDecl {
                name: "newDecl".to_string(),
                kind: DeclKind::Def,
                type_: "Nat".to_string(),
                body: None,
                params: vec![],
            }],
            metadata: HashMap::new(),
        };
        let (added, removed, modified) = diff_modules(&old, &new);
        assert_eq!(added, vec!["newDecl"]);
        assert!(removed.is_empty());
        assert!(modified.is_empty());
    }

    #[test]
    fn test_diff_removed() {
        let decl = SerialDecl {
            name: "gone".to_string(),
            kind: DeclKind::Axiom,
            type_: "False".to_string(),
            body: None,
            params: vec![],
        };
        let old = SerializedIr {
            version: CURRENT_VERSION,
            module_name: "M".to_string(),
            declarations: vec![decl],
            metadata: HashMap::new(),
        };
        let new = SerializedIr {
            version: CURRENT_VERSION,
            module_name: "M".to_string(),
            declarations: vec![],
            metadata: HashMap::new(),
        };
        let (added, removed, modified) = diff_modules(&old, &new);
        assert!(added.is_empty());
        assert_eq!(removed, vec!["gone"]);
        assert!(modified.is_empty());
    }

    #[test]
    fn test_diff_modified() {
        let d_old = SerialDecl {
            name: "f".to_string(),
            kind: DeclKind::Def,
            type_: "Nat".to_string(),
            body: Some("1".to_string()),
            params: vec![],
        };
        let d_new = SerialDecl {
            name: "f".to_string(),
            kind: DeclKind::Def,
            type_: "Nat".to_string(),
            body: Some("2".to_string()),
            params: vec![],
        };
        let old = SerializedIr {
            version: CURRENT_VERSION,
            module_name: "M".to_string(),
            declarations: vec![d_old],
            metadata: HashMap::new(),
        };
        let new = SerializedIr {
            version: CURRENT_VERSION,
            module_name: "M".to_string(),
            declarations: vec![d_new],
            metadata: HashMap::new(),
        };
        let (added, removed, modified) = diff_modules(&old, &new);
        assert!(added.is_empty());
        assert!(removed.is_empty());
        assert_eq!(modified, vec!["f"]);
    }

    #[test]
    fn test_diff_no_change() {
        let ir = sample_ir();
        let (added, removed, modified) = diff_modules(&ir, &ir);
        assert!(added.is_empty());
        assert!(removed.is_empty());
        assert!(modified.is_empty());
    }

    // ── version mismatch ──────────────────────────────────────────────────────

    #[test]
    fn test_version_mismatch_in_text() {
        let text = "-- OxiLean IR version 99\nmodule Foo\n";
        let result = deserialize_ir(text, IrFormat::Text);
        assert!(matches!(
            result,
            IrDeserializeResult::VersionMismatch {
                expected: _,
                found: 99
            }
        ));
    }

    #[test]
    fn test_version_mismatch_in_json() {
        let json =
            "{\"version\": 99, \"module_name\": \"Foo\", \"metadata\": {}, \"declarations\": []}";
        let result = deserialize_ir(json, IrFormat::Json);
        assert!(matches!(
            result,
            IrDeserializeResult::VersionMismatch {
                expected: _,
                found: 99
            }
        ));
    }
}
