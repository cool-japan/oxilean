//! Documentation Intermediate Representation (DocIR) for oxilean-codegen.
//!
//! This module defines [`DocIR`] — a flat, serialisable representation of the
//! documented declarations in a compiled LCNF module — and [`emit_doc_ir`],
//! which extracts it from an [`LcnfModule`].
//!
//! ## Limitations
//!
//! Doc comments are attached to declarations at the *parse* stage and are not
//! propagated into the codegen IR.  Consequently, [`DocIRItem::doc_comment`]
//! will always be an empty string when produced by [`emit_doc_ir`].  For
//! richly-documented output, callers should prefer source-level extraction
//! (see `oxilean-doc`'s `extractor` module) and use [`DocIR`] only as a
//! lightweight index of compiled declarations.

use crate::lcnf::types::{LcnfExternDecl, LcnfFunDecl, LcnfModule};

// ---------------------------------------------------------------------------
// Public types
// ---------------------------------------------------------------------------

/// Documentation IR — a flat list of documented declarations from one module.
///
/// Produced by [`emit_doc_ir`] and consumed by `oxilean-doc` to generate
/// documentation without re-parsing the source file.
#[derive(Debug, Clone, PartialEq)]
pub struct DocIR {
    /// The module name (typically the file path without extension).
    pub module_name: String,
    /// Documented declarations in declaration order.
    pub items: Vec<DocIRItem>,
}

/// A single documented declaration extracted from an [`LcnfModule`].
#[derive(Debug, Clone, PartialEq)]
pub struct DocIRItem {
    /// Fully-qualified declaration name.
    pub name: String,
    /// Declaration kind: `"def"`, `"axiom"`, or `"extern"`.
    ///
    /// `"def"` covers all top-level function declarations (including theorems
    /// and instances whose proofs have been compiled).  `"axiom"` / `"extern"`
    /// is used for declarations that have no compiled body (external stubs,
    /// axioms, opaques).
    pub kind: String,
    /// Pretty-printed type signature.
    ///
    /// Currently always an empty string — type information is partially erased
    /// by the LCNF lowering pass and cannot be reliably reconstructed here.
    pub signature: String,
    /// Doc comment text.
    ///
    /// Currently always empty; doc comments are not propagated into codegen.
    /// Use `oxilean-doc`'s source-level extractor for accurate doc comments.
    pub doc_comment: String,
    /// Whether this declaration was annotated with `@[deprecated]`.
    ///
    /// Not preserved through the LCNF pipeline; always `false` here.
    pub deprecated: bool,
}

// ---------------------------------------------------------------------------
// Extraction
// ---------------------------------------------------------------------------

/// Extract documentation IR from a compiled [`LcnfModule`].
///
/// Walks the module's [`LcnfFunDecl`]s (kind `"def"`) and
/// [`LcnfExternDecl`]s (kind `"axiom"`) in order, building one
/// [`DocIRItem`] per declaration.
///
/// # Limitations
///
/// - `signature` and `doc_comment` are empty strings: type information is
///   partially erased in LCNF, and doc comments are not plumbed through from
///   the parse stage.
/// - `deprecated` is always `false` for the same reason.
///
/// If you need signatures or doc comments, use `oxilean-doc`'s source-level
/// `extract()` instead, and use this function only for cross-referencing
/// compiled declaration *names*.
pub fn emit_doc_ir(module_name: &str, module: &LcnfModule) -> DocIR {
    let mut items: Vec<DocIRItem> =
        Vec::with_capacity(module.fun_decls.len() + module.extern_decls.len());

    for decl in &module.fun_decls {
        items.push(doc_ir_item_from_fun(decl));
    }
    for decl in &module.extern_decls {
        items.push(doc_ir_item_from_extern(decl));
    }

    DocIR {
        module_name: module_name.to_string(),
        items,
    }
}

// ---------------------------------------------------------------------------
// Private helpers
// ---------------------------------------------------------------------------

fn doc_ir_item_from_fun(decl: &LcnfFunDecl) -> DocIRItem {
    DocIRItem {
        name: decl.name.clone(),
        kind: "def".to_string(),
        signature: String::new(),
        doc_comment: String::new(),
        deprecated: false,
    }
}

fn doc_ir_item_from_extern(decl: &LcnfExternDecl) -> DocIRItem {
    DocIRItem {
        name: decl.name.clone(),
        kind: "axiom".to_string(),
        signature: String::new(),
        doc_comment: String::new(),
        deprecated: false,
    }
}

// ---------------------------------------------------------------------------
// Serialisation helpers (JSON via manual formatting — no serde dependency)
// ---------------------------------------------------------------------------

impl DocIRItem {
    /// Serialise this item to a JSON object string.
    pub fn to_json(&self) -> String {
        format!(
            r#"{{"name":{},"kind":{},"signature":{},"doc_comment":{},"deprecated":{}}}"#,
            json_string(&self.name),
            json_string(&self.kind),
            json_string(&self.signature),
            json_string(&self.doc_comment),
            self.deprecated,
        )
    }
}

impl DocIR {
    /// Serialise the entire DocIR to a JSON object string.
    pub fn to_json(&self) -> String {
        let items_json: Vec<String> = self.items.iter().map(|i| i.to_json()).collect();
        format!(
            r#"{{"module_name":{},"items":[{}]}}"#,
            json_string(&self.module_name),
            items_json.join(","),
        )
    }

    /// Deserialise a [`DocIR`] from a JSON string previously produced by
    /// [`DocIR::to_json`].
    ///
    /// This is a minimal round-trip parser covering only the exact output
    /// format emitted by `to_json`.  It is not a general JSON parser.
    ///
    /// Returns `None` if the input cannot be parsed.
    pub fn from_json(s: &str) -> Option<DocIR> {
        let s = s.trim();
        // Expect: {"module_name":"...","items":[...]}
        let module_name = extract_json_string_field(s, "module_name")?;
        let items_raw = extract_json_array_field(s, "items")?;
        let items = parse_json_items(&items_raw);
        Some(DocIR { module_name, items })
    }
}

/// Escape a string for JSON embedding.
fn json_string(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for ch in s.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

/// Extract the value of a JSON string field from a flat JSON object string.
///
/// Looks for `"key":"value"` patterns only; does not handle nested objects.
fn extract_json_string_field(s: &str, key: &str) -> Option<String> {
    let needle = format!("\"{}\":\"", key);
    let start = s.find(&needle)? + needle.len();
    let rest = &s[start..];
    let mut value = String::new();
    let mut chars = rest.chars();
    while let Some(ch) = chars.next() {
        match ch {
            '"' => return Some(value),
            '\\' => match chars.next()? {
                '"' => value.push('"'),
                '\\' => value.push('\\'),
                'n' => value.push('\n'),
                'r' => value.push('\r'),
                't' => value.push('\t'),
                other => {
                    value.push('\\');
                    value.push(other);
                }
            },
            c => value.push(c),
        }
    }
    None
}

/// Extract the raw JSON array value of a field from a flat JSON object string.
fn extract_json_array_field(s: &str, key: &str) -> Option<String> {
    let needle = format!("\"{}\":[", key);
    let start = s.find(&needle)? + needle.len() - 1; // points at '['
    let rest = &s[start..];
    let mut depth = 0i32;
    let mut end = 0;
    for (i, ch) in rest.char_indices() {
        match ch {
            '[' => depth += 1,
            ']' => {
                depth -= 1;
                if depth == 0 {
                    end = i;
                    break;
                }
            }
            _ => {}
        }
    }
    if end == 0 && depth != 0 {
        return None;
    }
    Some(rest[1..end].to_string()) // strip outer '[' and ']'
}

/// Parse a comma-separated list of JSON item objects from the raw array body.
fn parse_json_items(s: &str) -> Vec<DocIRItem> {
    if s.trim().is_empty() {
        return Vec::new();
    }
    // Split on `},{` boundaries
    let mut items = Vec::new();
    let mut depth = 0i32;
    let mut current_start = 0;
    for (i, ch) in s.char_indices() {
        match ch {
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth == 0 {
                    let obj = &s[current_start..=i];
                    if let Some(item) = parse_json_item(obj) {
                        items.push(item);
                    }
                    current_start = i + 1;
                }
            }
            _ => {}
        }
    }
    items
}

/// Parse a single `DocIRItem` from a JSON object string.
fn parse_json_item(s: &str) -> Option<DocIRItem> {
    let name = extract_json_string_field(s, "name")?;
    let kind = extract_json_string_field(s, "kind")?;
    let signature = extract_json_string_field(s, "signature").unwrap_or_default();
    let doc_comment = extract_json_string_field(s, "doc_comment").unwrap_or_default();
    let deprecated = s.contains("\"deprecated\":true");
    Some(DocIRItem {
        name,
        kind,
        signature,
        doc_comment,
        deprecated,
    })
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lcnf::types::{LcnfExpr, LcnfModule, LcnfType};

    fn make_fun_decl(name: &str) -> crate::lcnf::types::LcnfFunDecl {
        crate::lcnf::types::LcnfFunDecl {
            name: name.to_string(),
            original_name: None,
            params: vec![],
            ret_type: LcnfType::Unit,
            body: LcnfExpr::Unreachable,
            is_recursive: false,
            is_lifted: false,
            inline_cost: 0,
        }
    }

    fn make_extern_decl(name: &str) -> crate::lcnf::types::LcnfExternDecl {
        crate::lcnf::types::LcnfExternDecl {
            name: name.to_string(),
            params: vec![],
            ret_type: LcnfType::Unit,
        }
    }

    /// DocIRItem can be constructed and Debug-printed.
    #[test]
    fn test_doc_ir_item_construct_and_debug() {
        let item = DocIRItem {
            name: "Nat.add".to_string(),
            kind: "def".to_string(),
            signature: "(n m : Nat) : Nat".to_string(),
            doc_comment: "Add two natural numbers.".to_string(),
            deprecated: false,
        };
        let debug_str = format!("{:?}", item);
        assert!(debug_str.contains("Nat.add"));
        assert!(debug_str.contains("def"));
    }

    /// DocIR can be serialised to a JSON string.
    #[test]
    fn test_doc_ir_to_json() {
        let ir = DocIR {
            module_name: "Nat".to_string(),
            items: vec![DocIRItem {
                name: "Nat.zero".to_string(),
                kind: "def".to_string(),
                signature: String::new(),
                doc_comment: String::new(),
                deprecated: false,
            }],
        };
        let json = ir.to_json();
        assert!(json.contains("\"module_name\":\"Nat\""));
        assert!(json.contains("\"Nat.zero\""));
        assert!(json.contains("\"kind\":\"def\""));
    }

    /// emit_doc_ir with an empty module returns DocIR with no items.
    #[test]
    fn test_emit_doc_ir_empty() {
        let module = LcnfModule::default();
        let ir = emit_doc_ir("Empty", &module);
        assert_eq!(ir.module_name, "Empty");
        assert!(ir.items.is_empty());
    }

    /// emit_doc_ir with N fun_decls returns N items with correct names.
    #[test]
    fn test_emit_doc_ir_fun_decls() {
        let mut module = LcnfModule::default();
        module.fun_decls.push(make_fun_decl("Foo.bar"));
        module.fun_decls.push(make_fun_decl("Foo.baz"));
        let ir = emit_doc_ir("Foo", &module);
        assert_eq!(ir.items.len(), 2);
        assert_eq!(ir.items[0].name, "Foo.bar");
        assert_eq!(ir.items[0].kind, "def");
        assert_eq!(ir.items[1].name, "Foo.baz");
    }

    /// DocIR round-trips through JSON: serialise then deserialise, check equality.
    #[test]
    fn test_doc_ir_json_roundtrip() {
        let original = DocIR {
            module_name: "MyModule".to_string(),
            items: vec![
                DocIRItem {
                    name: "MyModule.foo".to_string(),
                    kind: "def".to_string(),
                    signature: String::new(),
                    doc_comment: String::new(),
                    deprecated: false,
                },
                DocIRItem {
                    name: "MyModule.Axiom1".to_string(),
                    kind: "axiom".to_string(),
                    signature: String::new(),
                    doc_comment: String::new(),
                    deprecated: true,
                },
            ],
        };
        let json = original.to_json();
        let recovered = DocIR::from_json(&json).expect("round-trip parse failed");
        assert_eq!(recovered.module_name, original.module_name);
        assert_eq!(recovered.items.len(), original.items.len());
        assert_eq!(recovered.items[0].name, original.items[0].name);
        assert_eq!(recovered.items[1].name, original.items[1].name);
    }

    /// The deprecated field is preserved in JSON round-trips.
    #[test]
    fn test_deprecated_field_preserved() {
        let item = DocIRItem {
            name: "OldApi.fn".to_string(),
            kind: "def".to_string(),
            signature: String::new(),
            doc_comment: String::new(),
            deprecated: true,
        };
        let ir = DocIR {
            module_name: "OldApi".to_string(),
            items: vec![item],
        };
        let json = ir.to_json();
        let recovered = DocIR::from_json(&json).expect("parse failed");
        assert!(recovered.items[0].deprecated);
    }

    /// emit_doc_ir with extern_decls returns items with kind "axiom".
    #[test]
    fn test_emit_doc_ir_extern_decls() {
        let mut module = LcnfModule::default();
        module.extern_decls.push(make_extern_decl("Nat.rec"));
        let ir = emit_doc_ir("Nat", &module);
        assert_eq!(ir.items.len(), 1);
        assert_eq!(ir.items[0].kind, "axiom");
        assert_eq!(ir.items[0].name, "Nat.rec");
    }
}
