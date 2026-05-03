//! Types for the LSP code actions module.
//!
//! Provides data structures for `textDocument/codeAction` LSP requests,
//! including quick-fixes, refactorings, and source actions for Lean4-like files.

use std::collections::HashMap;

use crate::lsp::{Diagnostic, JsonValue, Range};

/// The kind of a code action, following the LSP specification.
///
/// These correspond to the string-based code action kinds in LSP,
/// mapped to an enum for type safety.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum CodeActionKind {
    /// Quick-fixes for diagnostic issues (e.g. missing import, type error).
    QuickFix,
    /// General refactoring action.
    Refactor,
    /// Extract a sub-expression or sub-block into a new definition.
    RefactorExtract,
    /// Inline a definition at its use site.
    RefactorInline,
    /// Rewrite code to an equivalent form.
    RefactorRewrite,
    /// Source-level organization actions (e.g., sort imports).
    Source,
}

impl CodeActionKind {
    /// Convert to the LSP string representation.
    pub fn as_str(&self) -> &'static str {
        match self {
            CodeActionKind::QuickFix => "quickfix",
            CodeActionKind::Refactor => "refactor",
            CodeActionKind::RefactorExtract => "refactor.extract",
            CodeActionKind::RefactorInline => "refactor.inline",
            CodeActionKind::RefactorRewrite => "refactor.rewrite",
            CodeActionKind::Source => "source",
        }
    }

    /// Parse from an LSP string.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "quickfix" => Some(CodeActionKind::QuickFix),
            "refactor" => Some(CodeActionKind::Refactor),
            "refactor.extract" => Some(CodeActionKind::RefactorExtract),
            "refactor.inline" => Some(CodeActionKind::RefactorInline),
            "refactor.rewrite" => Some(CodeActionKind::RefactorRewrite),
            "source" => Some(CodeActionKind::Source),
            _ => None,
        }
    }

    /// Serialize to JSON string.
    pub fn to_json(&self) -> JsonValue {
        JsonValue::String(self.as_str().to_string())
    }
}

/// A single text edit within a document, replacing a range with new text.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CodeTextEdit {
    /// The range in the document to replace.
    pub range: Range,
    /// The new text to insert (empty string means deletion).
    pub new_text: String,
}

impl CodeTextEdit {
    /// Create a new text edit.
    pub fn new(range: Range, new_text: impl Into<String>) -> Self {
        Self {
            range,
            new_text: new_text.into(),
        }
    }

    /// Serialize to JSON (LSP `TextEdit` format).
    pub fn to_json(&self) -> JsonValue {
        JsonValue::Object(vec![
            ("range".to_string(), self.range.to_json()),
            (
                "newText".to_string(),
                JsonValue::String(self.new_text.clone()),
            ),
        ])
    }
}

/// A workspace-level edit mapping document URIs to their respective text edits.
#[derive(Clone, Debug, Default)]
pub struct WorkspaceEdit {
    /// Map from document URI to list of edits to apply in that document.
    pub changes: HashMap<String, Vec<CodeTextEdit>>,
}

impl WorkspaceEdit {
    /// Create an empty workspace edit.
    pub fn new() -> Self {
        Self {
            changes: HashMap::new(),
        }
    }

    /// Add a text edit for a specific document URI.
    pub fn add_edit(&mut self, uri: impl Into<String>, edit: CodeTextEdit) {
        self.changes.entry(uri.into()).or_default().push(edit);
    }

    /// Serialize to JSON (LSP `WorkspaceEdit` format).
    pub fn to_json(&self) -> JsonValue {
        let mut entries: Vec<(String, JsonValue)> = self
            .changes
            .iter()
            .map(|(uri, edits)| {
                let edit_array = JsonValue::Array(edits.iter().map(|e| e.to_json()).collect());
                (uri.clone(), edit_array)
            })
            .collect();
        // Sort for deterministic output
        entries.sort_by(|a, b| a.0.cmp(&b.0));
        JsonValue::Object(vec![("changes".to_string(), JsonValue::Object(entries))])
    }
}

/// A code action that the server can offer to the client.
///
/// Corresponds to LSP's `CodeAction` interface. Either provides a workspace
/// edit directly, or a command to be executed by the client.
#[derive(Clone, Debug)]
pub struct CodeAction {
    /// Human-readable title shown in the UI.
    pub title: String,
    /// The kind of code action (e.g., QuickFix, Refactor).
    pub kind: CodeActionKind,
    /// Diagnostics this action resolves (may be empty for refactorings).
    pub diagnostics: Vec<Diagnostic>,
    /// Optional workspace edit to apply when the action is selected.
    pub edit: Option<WorkspaceEdit>,
    /// Optional command identifier to execute (server-side).
    pub command: Option<String>,
    /// Whether this is the preferred (most important) action among alternatives.
    pub is_preferred: bool,
}

impl CodeAction {
    /// Create a quick-fix action with a workspace edit.
    pub fn quick_fix(
        title: impl Into<String>,
        diagnostics: Vec<Diagnostic>,
        edit: WorkspaceEdit,
    ) -> Self {
        Self {
            title: title.into(),
            kind: CodeActionKind::QuickFix,
            diagnostics,
            edit: Some(edit),
            command: None,
            is_preferred: true,
        }
    }

    /// Create a refactor action.
    pub fn refactor(title: impl Into<String>, kind: CodeActionKind, edit: WorkspaceEdit) -> Self {
        Self {
            title: title.into(),
            kind,
            diagnostics: Vec::new(),
            edit: Some(edit),
            command: None,
            is_preferred: false,
        }
    }

    /// Serialize to JSON (LSP `CodeAction` format).
    pub fn to_json(&self) -> JsonValue {
        let mut entries = vec![
            ("title".to_string(), JsonValue::String(self.title.clone())),
            ("kind".to_string(), self.kind.to_json()),
            (
                "isPreferred".to_string(),
                JsonValue::Bool(self.is_preferred),
            ),
        ];
        if !self.diagnostics.is_empty() {
            let diag_json = JsonValue::Array(
                self.diagnostics
                    .iter()
                    .map(|d| {
                        JsonValue::Object(vec![
                            ("range".to_string(), d.range.to_json()),
                            (
                                "severity".to_string(),
                                JsonValue::Number(d.severity.to_number()),
                            ),
                            ("message".to_string(), JsonValue::String(d.message.clone())),
                        ])
                    })
                    .collect(),
            );
            entries.push(("diagnostics".to_string(), diag_json));
        }
        if let Some(ref edit) = self.edit {
            entries.push(("edit".to_string(), edit.to_json()));
        }
        if let Some(ref cmd) = self.command {
            entries.push(("command".to_string(), JsonValue::String(cmd.clone())));
        }
        JsonValue::Object(entries)
    }
}

/// Context provided with a `textDocument/codeAction` request.
///
/// Contains the diagnostics at the requested range and an optional filter
/// specifying which action kinds are desired.
#[derive(Clone, Debug, Default)]
pub struct CodeActionContext {
    /// Diagnostics at the current range (may be empty).
    pub diagnostics: Vec<Diagnostic>,
    /// If non-empty, restrict actions to these kinds only.
    pub only: Vec<CodeActionKind>,
}

impl CodeActionContext {
    /// Create an empty context.
    pub fn new() -> Self {
        Self::default()
    }

    /// Check whether a given kind is allowed by this context's filter.
    pub fn allows_kind(&self, kind: &CodeActionKind) -> bool {
        if self.only.is_empty() {
            return true;
        }
        self.only.contains(kind)
    }
}

/// Stateless provider struct that holds configuration for code action generation.
///
/// Instantiate once and call `provide` to produce actions for a given position.
#[derive(Clone, Debug, Default)]
pub struct CodeActionProvider {
    /// Whether to suggest `sorry` placeholders for unfinished proofs.
    pub suggest_sorry: bool,
    /// Whether to suggest snake_case renames.
    pub suggest_snake_case: bool,
    /// Whether to suggest type annotations.
    pub suggest_type_annotations: bool,
}

impl CodeActionProvider {
    /// Create a provider with all suggestions enabled.
    pub fn new() -> Self {
        Self {
            suggest_sorry: true,
            suggest_snake_case: true,
            suggest_type_annotations: true,
        }
    }

    /// Create a provider with all suggestions disabled (useful for testing).
    pub fn disabled() -> Self {
        Self {
            suggest_sorry: false,
            suggest_snake_case: false,
            suggest_type_annotations: false,
        }
    }
}
