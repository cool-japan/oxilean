//! Types for the LSP document symbols module.
//!
//! Provides data structures for `textDocument/documentSymbol` LSP requests,
//! supporting both hierarchical (`DocumentSymbol`) and flat (`SymbolInformation`)
//! response formats for Lean4-like source files.

use crate::lsp::{JsonValue, Location, Range};

/// LSP symbol kinds, as defined in the Language Server Protocol specification.
///
/// Each variant carries the integer discriminant used in JSON serialization.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i32)]
pub enum DocSymbolKind {
    /// A source file.
    File = 1,
    /// A namespace declaration.
    Namespace = 3,
    /// A function or `def` declaration.
    Function = 12,
    /// A variable or `let` binding.
    Variable = 13,
    /// A constant (axiom, postulate).
    Constant = 14,
    /// A string literal or `#eval` expression (informational).
    StringLiteral = 15,
    /// A `structure` or `inductive` type.
    Struct = 23,
    /// An `instance` declaration (used like an event/hook).
    Event = 24,
}

impl DocSymbolKind {
    /// Convert to LSP integer value.
    pub fn to_number(self) -> f64 {
        self as i32 as f64
    }

    /// Serialize to JSON.
    pub fn to_json(self) -> JsonValue {
        JsonValue::Number(self.to_number())
    }

    /// Return a human-readable label for the kind.
    pub fn label(self) -> &'static str {
        match self {
            DocSymbolKind::File => "file",
            DocSymbolKind::Namespace => "namespace",
            DocSymbolKind::Function => "function",
            DocSymbolKind::Variable => "variable",
            DocSymbolKind::Constant => "constant",
            DocSymbolKind::StringLiteral => "string",
            DocSymbolKind::Struct => "struct",
            DocSymbolKind::Event => "event",
        }
    }
}

/// A hierarchical document symbol, as returned by `textDocument/documentSymbol`
/// when the client supports `hierarchicalDocumentSymbolSupport`.
///
/// Children represent nested scopes (e.g., declarations inside a `namespace`).
#[derive(Clone, Debug)]
pub struct DocSymbol {
    /// The name of this symbol (e.g., `"foo"` for `def foo := ...`).
    pub name: String,
    /// An optional detail string (e.g., the type signature).
    pub detail: Option<String>,
    /// The LSP symbol kind.
    pub kind: DocSymbolKind,
    /// The full range of the symbol, including its body.
    pub range: Range,
    /// The range covering just the name (for cursor navigation).
    pub selection_range: Range,
    /// Nested child symbols (e.g., definitions inside a namespace).
    pub children: Vec<DocSymbol>,
}

impl DocSymbol {
    /// Create a new leaf symbol with no children.
    pub fn new(
        name: impl Into<String>,
        kind: DocSymbolKind,
        range: Range,
        selection_range: Range,
    ) -> Self {
        Self {
            name: name.into(),
            detail: None,
            kind,
            range,
            selection_range,
            children: Vec::new(),
        }
    }

    /// Attach a detail string (e.g., type signature) to the symbol.
    pub fn with_detail(mut self, detail: impl Into<String>) -> Self {
        self.detail = Some(detail.into());
        self
    }

    /// Add a child symbol into this symbol's scope.
    pub fn add_child(&mut self, child: DocSymbol) {
        self.children.push(child);
    }

    /// Serialize to JSON (LSP `DocumentSymbol` format).
    pub fn to_json(&self) -> JsonValue {
        let mut entries = vec![
            ("name".to_string(), JsonValue::String(self.name.clone())),
            ("kind".to_string(), self.kind.to_json()),
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

/// A flat symbol information entry, used when hierarchical support is unavailable.
///
/// Part of the LSP `SymbolInformation` interface. All symbols are emitted
/// as siblings with an optional `container_name` that links them to a parent.
#[derive(Clone, Debug)]
pub struct SymbolInformation {
    /// The symbol name.
    pub name: String,
    /// The LSP symbol kind.
    pub kind: DocSymbolKind,
    /// The location of the symbol declaration.
    pub location: Location,
    /// Optional name of the enclosing scope (e.g., namespace name).
    pub container_name: Option<String>,
}

impl SymbolInformation {
    /// Create a new symbol information entry.
    pub fn new(name: impl Into<String>, kind: DocSymbolKind, location: Location) -> Self {
        Self {
            name: name.into(),
            kind,
            location,
            container_name: None,
        }
    }

    /// Attach a container name to this entry.
    pub fn with_container(mut self, container: impl Into<String>) -> Self {
        self.container_name = Some(container.into());
        self
    }

    /// Serialize to JSON (LSP `SymbolInformation` format).
    pub fn to_json(&self) -> JsonValue {
        let mut entries = vec![
            ("name".to_string(), JsonValue::String(self.name.clone())),
            ("kind".to_string(), self.kind.to_json()),
            ("location".to_string(), self.location.to_json()),
        ];
        if let Some(ref container) = self.container_name {
            entries.push((
                "containerName".to_string(),
                JsonValue::String(container.clone()),
            ));
        }
        JsonValue::Object(entries)
    }
}

/// The response to a `textDocument/documentSymbol` request.
///
/// Clients that support `hierarchicalDocumentSymbolSupport` receive
/// `Hierarchical` (a tree of `DocSymbol`s); older clients receive
/// `Flat` (a list of `SymbolInformation`).
#[derive(Clone, Debug)]
pub enum DocumentSymbolResponse {
    /// Hierarchical document symbols (preferred).
    Hierarchical(Vec<DocSymbol>),
    /// Flat symbol information list (legacy).
    Flat(Vec<SymbolInformation>),
}

impl DocumentSymbolResponse {
    /// Serialize to JSON.
    pub fn to_json(&self) -> JsonValue {
        match self {
            DocumentSymbolResponse::Hierarchical(syms) => {
                JsonValue::Array(syms.iter().map(|s| s.to_json()).collect())
            }
            DocumentSymbolResponse::Flat(syms) => {
                JsonValue::Array(syms.iter().map(|s| s.to_json()).collect())
            }
        }
    }

    /// Return the number of top-level symbols.
    pub fn len(&self) -> usize {
        match self {
            DocumentSymbolResponse::Hierarchical(syms) => syms.len(),
            DocumentSymbolResponse::Flat(syms) => syms.len(),
        }
    }

    /// Return whether the response is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
