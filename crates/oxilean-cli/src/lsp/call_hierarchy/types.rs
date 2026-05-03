//! Types for the LSP call hierarchy module.
//!
//! Provides data structures for `textDocument/prepareCallHierarchy`,
//! `callHierarchy/incomingCalls`, and `callHierarchy/outgoingCalls` LSP
//! requests on Lean4-like source files.

use crate::lsp::{JsonValue, Range};

// ============================================================================
// SymbolKind
// ============================================================================

/// LSP symbol kinds used in call hierarchy items.
///
/// Values match the Language Server Protocol specification section 3.17.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i32)]
pub enum SymbolKind {
    /// File symbol (1).
    File = 1,
    /// Module symbol (2).
    Module = 2,
    /// Namespace symbol (3).
    Namespace = 3,
    /// Class symbol (5).
    Class = 5,
    /// Method symbol (6).
    Method = 6,
    /// Property symbol (7).
    Property = 7,
    /// Constructor symbol (9).
    Constructor = 9,
    /// Enum symbol (10).
    Enum = 10,
    /// Function symbol (12) — used for `def`, `theorem`, `lemma`, `abbrev`.
    Function = 12,
    /// Variable symbol (13).
    Variable = 13,
    /// Constant symbol (14) — used for `axiom`.
    Constant = 14,
    /// Struct symbol (23) — used for `structure`, `inductive`, `class`.
    Struct = 23,
    /// Event symbol (24) — used for `instance`.
    Event = 24,
    /// Type parameter symbol (26).
    TypeParameter = 26,
}

impl SymbolKind {
    /// Classify a Lean4 declaration keyword into a `SymbolKind`.
    pub fn from_keyword(kw: &str) -> Self {
        match kw {
            "def" | "theorem" | "lemma" | "abbrev" | "partial" | "noncomputable" => {
                SymbolKind::Function
            }
            "axiom" | "postulate" | "opaque" => SymbolKind::Constant,
            "structure" | "inductive" | "class" => SymbolKind::Struct,
            "instance" => SymbolKind::Event,
            "namespace" | "section" | "module" => SymbolKind::Namespace,
            _ => SymbolKind::Function,
        }
    }

    /// Serialize to LSP integer JSON value.
    pub fn to_json(self) -> JsonValue {
        JsonValue::Number(self as i32 as f64)
    }

    /// Human-readable label.
    pub fn label(self) -> &'static str {
        match self {
            SymbolKind::File => "file",
            SymbolKind::Module => "module",
            SymbolKind::Namespace => "namespace",
            SymbolKind::Class => "class",
            SymbolKind::Method => "method",
            SymbolKind::Property => "property",
            SymbolKind::Constructor => "constructor",
            SymbolKind::Enum => "enum",
            SymbolKind::Function => "function",
            SymbolKind::Variable => "variable",
            SymbolKind::Constant => "constant",
            SymbolKind::Struct => "struct",
            SymbolKind::Event => "event",
            SymbolKind::TypeParameter => "typeParameter",
        }
    }
}

// ============================================================================
// CallHierarchyItem
// ============================================================================

/// A single node in the call hierarchy — a named declaration in a document.
///
/// Corresponds to the LSP `CallHierarchyItem` structure (LSP §3.16.3).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CallHierarchyItem {
    /// The name of the declaration (e.g., `"myFunc"`).
    pub name: String,
    /// The LSP symbol kind.
    pub kind: SymbolKind,
    /// The document URI where this declaration lives.
    pub uri: String,
    /// The full range of the declaration (including its body).
    pub range: Range,
    /// The range covering just the name token.
    pub selection_range: Range,
    /// Optional detail string (e.g., type signature or keyword).
    pub detail: Option<String>,
}

impl CallHierarchyItem {
    /// Create a new call hierarchy item.
    pub fn new(
        name: impl Into<String>,
        kind: SymbolKind,
        uri: impl Into<String>,
        range: Range,
        selection_range: Range,
    ) -> Self {
        Self {
            name: name.into(),
            kind,
            uri: uri.into(),
            range,
            selection_range,
            detail: None,
        }
    }

    /// Attach an optional detail string.
    pub fn with_detail(mut self, detail: impl Into<String>) -> Self {
        self.detail = Some(detail.into());
        self
    }

    /// Serialize to JSON (LSP `CallHierarchyItem` format).
    pub fn to_json(&self) -> JsonValue {
        let mut entries = vec![
            ("name".to_string(), JsonValue::String(self.name.clone())),
            ("kind".to_string(), self.kind.to_json()),
            ("uri".to_string(), JsonValue::String(self.uri.clone())),
            ("range".to_string(), self.range.to_json()),
            ("selectionRange".to_string(), self.selection_range.to_json()),
        ];
        if let Some(ref detail) = self.detail {
            entries.push(("detail".to_string(), JsonValue::String(detail.clone())));
        }
        JsonValue::Object(entries)
    }

    /// Parse a `CallHierarchyItem` from a JSON value.
    pub fn from_json(val: &JsonValue) -> Result<Self, String> {
        let name = val
            .get("name")
            .and_then(|v| v.as_str())
            .ok_or("missing name")?
            .to_string();
        let kind_num = val
            .get("kind")
            .and_then(|v| v.as_i64())
            .ok_or("missing kind")?;
        let kind = symbol_kind_from_i64(kind_num);
        let uri = val
            .get("uri")
            .and_then(|v| v.as_str())
            .ok_or("missing uri")?
            .to_string();
        let range_val = val.get("range").ok_or("missing range")?;
        let range = Range::from_json(range_val)?;
        let sel_val = val.get("selectionRange").ok_or("missing selectionRange")?;
        let selection_range = Range::from_json(sel_val)?;
        let detail = val
            .get("detail")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        Ok(Self {
            name,
            kind,
            uri,
            range,
            selection_range,
            detail,
        })
    }
}

/// Convert an LSP integer kind value to `SymbolKind`, defaulting to `Function`.
fn symbol_kind_from_i64(n: i64) -> SymbolKind {
    match n {
        1 => SymbolKind::File,
        2 => SymbolKind::Module,
        3 => SymbolKind::Namespace,
        5 => SymbolKind::Class,
        6 => SymbolKind::Method,
        7 => SymbolKind::Property,
        9 => SymbolKind::Constructor,
        10 => SymbolKind::Enum,
        12 => SymbolKind::Function,
        13 => SymbolKind::Variable,
        14 => SymbolKind::Constant,
        23 => SymbolKind::Struct,
        24 => SymbolKind::Event,
        26 => SymbolKind::TypeParameter,
        _ => SymbolKind::Function,
    }
}

// ============================================================================
// CallHierarchyIncomingCall
// ============================================================================

/// Represents a single caller of a declaration (an incoming call edge).
///
/// The `from` field identifies the calling declaration; `from_ranges` lists
/// the specific ranges within `from`'s body where the callee is referenced.
///
/// Corresponds to `CallHierarchyIncomingCall` in LSP §3.16.3.
#[derive(Clone, Debug)]
pub struct CallHierarchyIncomingCall {
    /// The declaration that calls into the target item.
    pub from: CallHierarchyItem,
    /// The ranges inside `from`'s body where the call occurs.
    pub from_ranges: Vec<Range>,
}

impl CallHierarchyIncomingCall {
    /// Create a new incoming call record.
    pub fn new(from: CallHierarchyItem, from_ranges: Vec<Range>) -> Self {
        Self { from, from_ranges }
    }

    /// Serialize to JSON (LSP `CallHierarchyIncomingCall` format).
    pub fn to_json(&self) -> JsonValue {
        JsonValue::Object(vec![
            ("from".to_string(), self.from.to_json()),
            (
                "fromRanges".to_string(),
                JsonValue::Array(self.from_ranges.iter().map(|r| r.to_json()).collect()),
            ),
        ])
    }
}

// ============================================================================
// CallHierarchyOutgoingCall
// ============================================================================

/// Represents a single callee of a declaration (an outgoing call edge).
///
/// The `to` field identifies the called declaration; `from_ranges` lists the
/// specific ranges within the current item's body where the call appears.
///
/// Corresponds to `CallHierarchyOutgoingCall` in LSP §3.16.3.
#[derive(Clone, Debug)]
pub struct CallHierarchyOutgoingCall {
    /// The declaration that is called by the current item.
    pub to: CallHierarchyItem,
    /// The ranges inside the current item's body where this call appears.
    pub from_ranges: Vec<Range>,
}

impl CallHierarchyOutgoingCall {
    /// Create a new outgoing call record.
    pub fn new(to: CallHierarchyItem, from_ranges: Vec<Range>) -> Self {
        Self { to, from_ranges }
    }

    /// Serialize to JSON (LSP `CallHierarchyOutgoingCall` format).
    pub fn to_json(&self) -> JsonValue {
        JsonValue::Object(vec![
            ("to".to_string(), self.to.to_json()),
            (
                "fromRanges".to_string(),
                JsonValue::Array(self.from_ranges.iter().map(|r| r.to_json()).collect()),
            ),
        ])
    }
}

// ============================================================================
// CallHierarchyHandler
// ============================================================================

/// Stateless handler facade for call hierarchy LSP requests.
///
/// All state is passed in via the source text; the handler itself holds only
/// the document URI used as a default when one cannot be inferred from context.
#[derive(Clone, Debug, Default)]
pub struct CallHierarchyHandler {
    /// The default URI to embed in created `CallHierarchyItem`s.
    pub uri: String,
}

impl CallHierarchyHandler {
    /// Create a new handler for the given document URI.
    pub fn new(uri: impl Into<String>) -> Self {
        Self { uri: uri.into() }
    }
}
