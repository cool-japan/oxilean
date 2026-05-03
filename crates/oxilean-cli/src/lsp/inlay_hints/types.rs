//! Inlay hints types and request handler.
//!
//! Provides [`InlayHint`], [`InlayHintKind`], and [`InlayHintHandler`] for
//! serving `textDocument/inlayHint` LSP requests.
//!
//! Type hints are emitted after `let` bindings without an explicit type
//! annotation, and `have` expressions.  Parameter hints label call-site
//! argument positions with the matching Pi-binder name from the environment.

use crate::lsp::{Document, JsonValue, Position, Range};
use oxilean_kernel::{BinderInfo, Environment, Expr, Name};
use oxilean_parse::{Lexer, Token, TokenKind};

// ── Core types ────────────────────────────────────────────────────────────────

/// Kind of inlay hint, as defined in LSP 3.17 §3.17.15.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum InlayHintKind {
    /// Type annotation hint (kind = 1 in LSP spec).
    Type = 1,
    /// Parameter name hint (kind = 2 in LSP spec).
    Parameter = 2,
}

impl InlayHintKind {
    /// Return the LSP integer value.
    pub fn lsp_value(self) -> u32 {
        self as u32
    }

    /// Try to parse from an LSP integer.
    pub fn from_lsp_value(v: u32) -> Option<Self> {
        match v {
            1 => Some(Self::Type),
            2 => Some(Self::Parameter),
            _ => None,
        }
    }
}

/// An inlay hint for a document position.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InlayHint {
    /// Where in the document the hint should appear.
    pub position: Position,
    /// The label string shown in the editor.
    pub label: String,
    /// The semantic kind of this hint.
    pub kind: InlayHintKind,
}

impl InlayHint {
    /// Create a type annotation hint.
    ///
    /// The label is formatted as `: <ty>`.
    pub fn type_hint(position: Position, ty: impl Into<String>) -> Self {
        let ty = ty.into();
        Self {
            position,
            label: format!(": {}", ty),
            kind: InlayHintKind::Type,
        }
    }

    /// Create a parameter name hint.
    ///
    /// The label is formatted as `<name>:`.
    pub fn parameter_hint(position: Position, name: impl Into<String>) -> Self {
        let name = name.into();
        Self {
            position,
            label: format!("{}:", name),
            kind: InlayHintKind::Parameter,
        }
    }

    /// Serialize to LSP JSON.
    pub fn to_json(&self) -> JsonValue {
        JsonValue::Object(vec![
            ("position".to_string(), self.position.to_json()),
            ("label".to_string(), JsonValue::String(self.label.clone())),
            (
                "kind".to_string(),
                JsonValue::Number(self.kind.lsp_value() as f64),
            ),
            (
                "paddingLeft".to_string(),
                JsonValue::Bool(matches!(self.kind, InlayHintKind::Type)),
            ),
            (
                "paddingRight".to_string(),
                JsonValue::Bool(matches!(self.kind, InlayHintKind::Parameter)),
            ),
        ])
    }
}

// ── Handler ───────────────────────────────────────────────────────────────────

/// Handles `textDocument/inlayHint` requests.
///
/// Delegates to [`InlayHintComputer`] for the actual computation and
/// serialises the result to the LSP JSON format expected by clients.
pub struct InlayHintHandler<'a> {
    /// Kernel environment for type lookups.
    env: &'a Environment,
    /// Maximum number of hints to return per request.
    pub max_hints: usize,
    /// Whether to emit type hints.
    pub show_type_hints: bool,
    /// Whether to emit parameter hints.
    pub show_parameter_hints: bool,
}

impl<'a> InlayHintHandler<'a> {
    /// Create a handler with default settings.
    pub fn new(env: &'a Environment) -> Self {
        Self {
            env,
            max_hints: 200,
            show_type_hints: true,
            show_parameter_hints: true,
        }
    }

    /// Handle a `textDocument/inlayHint` JSON-RPC request params object.
    ///
    /// Returns a JSON array of inlay hint objects, or `null` if the document
    /// could not be found.
    pub fn handle(&self, params: &JsonValue, doc: &Document) -> JsonValue {
        let range = params
            .get("range")
            .and_then(|v| Range::from_json(v).ok())
            .unwrap_or_else(|| full_document_range(doc));

        let computer = InlayHintComputer {
            env: self.env,
            show_type_hints: self.show_type_hints,
            show_parameter_hints: self.show_parameter_hints,
            max_hints: self.max_hints,
        };
        let hints = computer.compute(doc, &range);
        JsonValue::Array(hints.iter().map(|h| h.to_json()).collect())
    }
}

/// Computes inlay hints for a document by lexing and matching against the
/// kernel environment.
pub(super) struct InlayHintComputer<'a> {
    pub(super) env: &'a Environment,
    pub(super) show_type_hints: bool,
    pub(super) show_parameter_hints: bool,
    pub(super) max_hints: usize,
}

impl<'a> InlayHintComputer<'a> {
    /// Compute all hints within `range` for `doc`.
    pub(super) fn compute(&self, doc: &Document, range: &Range) -> Vec<InlayHint> {
        let mut hints = Vec::new();
        if self.show_type_hints {
            hints.extend(self.type_hints(doc, range));
        }
        if self.show_parameter_hints {
            hints.extend(self.parameter_hints(doc, range));
        }
        // Sort by position for deterministic output.
        hints.sort_by(|a, b| {
            a.position
                .line
                .cmp(&b.position.line)
                .then(a.position.character.cmp(&b.position.character))
        });
        if hints.len() > self.max_hints {
            hints.truncate(self.max_hints);
        }
        hints
    }

    /// Emit `: <Type>` hints after `let` or `have` bindings with no explicit
    /// type annotation.
    fn type_hints(&self, doc: &Document, range: &Range) -> Vec<InlayHint> {
        let mut hints = Vec::new();
        let mut lexer = Lexer::new(&doc.content);
        let tokens = lexer.tokenize();
        let mut i = 0;

        while i < tokens.len() {
            let tok = &tokens[i];
            let line = tok.span.line.saturating_sub(1) as u32;

            // Skip tokens outside the requested range.
            if line < range.start.line {
                i += 1;
                continue;
            }
            if line > range.end.line {
                break;
            }

            // Match `let <ident>` or `have <ident>`.
            let is_let_or_have = matches!(tok.kind, TokenKind::Let | TokenKind::Have);

            if is_let_or_have && i + 1 < tokens.len() {
                if let TokenKind::Ident(name) = &tokens[i + 1].kind {
                    // Look ahead for `:` (type already present) or `:=` / `=`.
                    let mut has_explicit_type = false;
                    let mut j = i + 2;
                    while j < tokens.len() {
                        match &tokens[j].kind {
                            TokenKind::Colon => {
                                has_explicit_type = true;
                                break;
                            }
                            TokenKind::Assign => break,
                            // Binders and parens indicate no explicit ascription here.
                            TokenKind::LParen
                            | TokenKind::LBracket
                            | TokenKind::LBrace
                            | TokenKind::RParen
                            | TokenKind::RBracket
                            | TokenKind::RBrace => break,
                            _ => {}
                        }
                        j += 1;
                    }

                    if !has_explicit_type {
                        let kernel_name = Name::str(name);
                        if let Some(ci) = self.env.find(&kernel_name) {
                            let ty_str = format_type(ci.ty());
                            let name_tok = &tokens[i + 1];
                            let col = name_tok.span.column.saturating_sub(1) as u32;
                            let len = (name_tok.span.end - name_tok.span.start) as u32;
                            hints
                                .push(InlayHint::type_hint(Position::new(line, col + len), ty_str));
                        }
                    }
                }
            }
            i += 1;
        }
        hints
    }

    /// Emit `<param>:` hints before each argument at call sites.
    fn parameter_hints(&self, doc: &Document, range: &Range) -> Vec<InlayHint> {
        let mut hints = Vec::new();
        let mut lexer = Lexer::new(&doc.content);
        let tokens = lexer.tokenize();
        let mut i = 0;

        while i < tokens.len() {
            let tok = &tokens[i];
            let line = tok.span.line.saturating_sub(1) as u32;

            if line < range.start.line {
                i += 1;
                continue;
            }
            if line > range.end.line {
                break;
            }

            // Match `<ident>(` — a function call.
            if let TokenKind::Ident(func_name) = &tok.kind {
                if i + 1 < tokens.len() && tokens[i + 1].kind == TokenKind::LParen {
                    let kernel_name = Name::str(func_name);
                    if let Some(ci) = self.env.find(&kernel_name) {
                        let param_names = collect_pi_param_names(ci.ty());
                        let mut arg_idx = i + 2; // skip past `(`
                        for param_name in &param_names {
                            if arg_idx >= tokens.len() {
                                break;
                            }
                            let arg_tok = &tokens[arg_idx];
                            if arg_tok.kind == TokenKind::RParen {
                                break;
                            }
                            let arg_line = arg_tok.span.line.saturating_sub(1) as u32;
                            let arg_col = arg_tok.span.column.saturating_sub(1) as u32;
                            hints.push(InlayHint::parameter_hint(
                                Position::new(arg_line, arg_col),
                                param_name.to_string(),
                            ));
                            // Advance past the argument (respecting nesting).
                            arg_idx = skip_argument(&tokens, arg_idx);
                        }
                    }
                }
            }
            i += 1;
        }
        hints
    }
}

// ── Utilities ─────────────────────────────────────────────────────────────────

/// Build a range that spans the entire document.
fn full_document_range(doc: &Document) -> Range {
    let line_count = doc.content.lines().count() as u32;
    Range::new(
        Position::new(0, 0),
        Position::new(line_count.max(1) - 1, u32::MAX),
    )
}

/// Format an `Expr` type as a short human-readable string.
fn format_type(ty: &Expr) -> String {
    format!("{:?}", ty)
}

/// Walk a Pi-type spine and collect binder names.
fn collect_pi_param_names(ty: &Expr) -> Vec<Name> {
    let mut names = Vec::new();
    let mut current = ty;
    // `Expr::Pi(BinderInfo, Name, Box<Expr> type, Box<Expr> body)`
    while let Expr::Pi(_binder_info, name, _ty, body) = current {
        names.push(name.clone());
        current = body;
    }
    names
}

/// Advance `start` past a single call argument (respects parentheses nesting).
///
/// Returns the index of the token *after* the argument separator (`,`) or
/// `start + 1` if the argument list ends.
fn skip_argument(tokens: &[Token], start: usize) -> usize {
    let mut depth: i32 = 0;
    let mut i = start;
    while i < tokens.len() {
        match tokens[i].kind {
            TokenKind::LParen | TokenKind::LBracket | TokenKind::LBrace => depth += 1,
            TokenKind::RParen | TokenKind::RBracket | TokenKind::RBrace => {
                if depth == 0 {
                    // End of argument list — don't consume the `)`.
                    return i;
                }
                depth -= 1;
            }
            TokenKind::Comma if depth == 0 => return i + 1,
            _ => {}
        }
        i += 1;
    }
    i
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── InlayHintKind ──────────────────────────────────────────────────────

    #[test]
    fn test_inlay_hint_kind_type_value() {
        assert_eq!(InlayHintKind::Type.lsp_value(), 1);
    }

    #[test]
    fn test_inlay_hint_kind_parameter_value() {
        assert_eq!(InlayHintKind::Parameter.lsp_value(), 2);
    }

    #[test]
    fn test_inlay_hint_kind_from_lsp_value() {
        assert_eq!(InlayHintKind::from_lsp_value(1), Some(InlayHintKind::Type));
        assert_eq!(
            InlayHintKind::from_lsp_value(2),
            Some(InlayHintKind::Parameter)
        );
        assert_eq!(InlayHintKind::from_lsp_value(0), None);
        assert_eq!(InlayHintKind::from_lsp_value(3), None);
    }

    // ── InlayHint construction ─────────────────────────────────────────────

    #[test]
    fn test_type_hint_label() {
        let h = InlayHint::type_hint(Position::new(0, 5), "Nat");
        assert_eq!(h.label, ": Nat");
        assert_eq!(h.kind, InlayHintKind::Type);
        assert_eq!(h.position, Position::new(0, 5));
    }

    #[test]
    fn test_parameter_hint_label() {
        let h = InlayHint::parameter_hint(Position::new(1, 2), "n");
        assert_eq!(h.label, "n:");
        assert_eq!(h.kind, InlayHintKind::Parameter);
    }

    // ── JSON serialisation ─────────────────────────────────────────────────

    #[test]
    fn test_type_hint_to_json_kind_field() {
        let h = InlayHint::type_hint(Position::new(0, 0), "Bool");
        let j = h.to_json();
        let kind = j.get("kind").and_then(|v| v.as_i64());
        assert_eq!(kind, Some(1));
    }

    #[test]
    fn test_parameter_hint_to_json_kind_field() {
        let h = InlayHint::parameter_hint(Position::new(0, 0), "x");
        let j = h.to_json();
        let kind = j.get("kind").and_then(|v| v.as_i64());
        assert_eq!(kind, Some(2));
    }

    #[test]
    fn test_type_hint_padding_left() {
        let h = InlayHint::type_hint(Position::new(0, 0), "T");
        let j = h.to_json();
        assert_eq!(j.get("paddingLeft").and_then(|v| v.as_bool()), Some(true));
        assert_eq!(j.get("paddingRight").and_then(|v| v.as_bool()), Some(false));
    }

    #[test]
    fn test_parameter_hint_padding_right() {
        let h = InlayHint::parameter_hint(Position::new(0, 0), "p");
        let j = h.to_json();
        assert_eq!(j.get("paddingLeft").and_then(|v| v.as_bool()), Some(false));
        assert_eq!(j.get("paddingRight").and_then(|v| v.as_bool()), Some(true));
    }

    // ── Handler with empty environment ────────────────────────────────────

    #[test]
    fn test_handler_empty_env_returns_array() {
        let env = Environment::new();
        let handler = InlayHintHandler::new(&env);
        let doc = Document::new("file:///t.lean", 1, "let x := 5\n");
        let params = JsonValue::Null;
        let result = handler.handle(&params, &doc);
        assert!(matches!(result, JsonValue::Array(_)));
    }

    #[test]
    fn test_handler_returns_empty_for_empty_doc() {
        let env = Environment::new();
        let handler = InlayHintHandler::new(&env);
        let doc = Document::new("file:///empty.lean", 1, "");
        let params = JsonValue::Null;
        let result = handler.handle(&params, &doc);
        if let JsonValue::Array(arr) = result {
            assert!(arr.is_empty());
        } else {
            panic!("Expected array");
        }
    }

    // ── Computer unit tests ───────────────────────────────────────────────

    #[test]
    fn test_computer_no_hints_unknown_names() {
        let env = Environment::new();
        let computer = InlayHintComputer {
            env: &env,
            show_type_hints: true,
            show_parameter_hints: true,
            max_hints: 100,
        };
        let doc = Document::new("file:///t.lean", 1, "let foo := bar baz\n");
        let range = full_document_range(&doc);
        // No names in env → no hints.
        assert!(computer.compute(&doc, &range).is_empty());
    }

    #[test]
    fn test_computer_max_hints_cap() {
        let env = Environment::new();
        let computer = InlayHintComputer {
            env: &env,
            show_type_hints: true,
            show_parameter_hints: true,
            max_hints: 0,
        };
        let doc = Document::new("file:///t.lean", 1, "let a := 1\nlet b := 2\n");
        let range = full_document_range(&doc);
        let hints = computer.compute(&doc, &range);
        assert!(hints.is_empty());
    }

    // ── skip_argument utility ─────────────────────────────────────────────

    #[test]
    fn test_full_document_range_non_empty_doc() {
        let doc = Document::new("file:///t.lean", 1, "a\nb\nc\n");
        let r = full_document_range(&doc);
        assert_eq!(r.start, Position::new(0, 0));
        assert!(r.end.line >= 2);
    }
}
