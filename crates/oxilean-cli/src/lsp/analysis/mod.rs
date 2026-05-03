//! Analysis engine: document analysis, symbol extraction, reference finding.

use std::collections::HashMap;

use oxilean_elab::info_tree::{InfoData, InfoTree, InfoTreeBuilder};
use oxilean_elab::{elaborate_decl as elab_decl_impl, PendingDecl};
use oxilean_kernel::{check_declaration, Declaration, Environment, Name, ReducibilityHint};
use oxilean_parse::{Lexer, Parser, TokenKind};

use super::lsp_types::{
    Diagnostic, DiagnosticSeverity, DocumentSymbol, Location, Range, SymbolKind, TextEdit,
};

// ── DefinitionInfo / AnalysisResult ──────────────────────────────────────────

/// A definition found during analysis.
#[derive(Clone, Debug)]
pub struct DefinitionInfo {
    /// Name of the definition.
    pub name: String,
    /// Kind of the definition.
    pub kind: SymbolKind,
    /// Range where it's defined.
    pub range: Range,
    /// Type annotation if available.
    pub ty: Option<String>,
    /// Documentation if available.
    pub doc: Option<String>,
}

/// Result of analyzing a document.
#[derive(Clone, Debug, Default)]
pub struct AnalysisResult {
    /// Diagnostics found.
    pub diagnostics: Vec<Diagnostic>,
    /// Symbols found.
    pub symbols: Vec<DocumentSymbol>,
    /// Definitions found.
    pub definitions: Vec<DefinitionInfo>,
}

// ── Public analysis functions ─────────────────────────────────────────────────

/// Analyze a document using the lexer and parser.
pub fn analyze_document(uri: &str, content: &str, env: &Environment) -> AnalysisResult {
    let mut result = AnalysisResult::default();
    // Run lexer to find token-level errors
    let mut lexer = Lexer::new(content);
    let tokens = lexer.tokenize();
    for token in &tokens {
        if let TokenKind::Error(msg) = &token.kind {
            let line = if token.span.line > 0 {
                token.span.line as u32 - 1
            } else {
                0
            };
            let col = if token.span.column > 0 {
                token.span.column as u32 - 1
            } else {
                0
            };
            result.diagnostics.push(Diagnostic::error(
                Range::single_line(line, col, col + 1),
                format!("lexer error: {}", msg),
            ));
        }
    }

    // Extract symbols and definitions from tokens
    extract_symbols_from_tokens(uri, &tokens, &mut result);

    // Check environment for name collisions
    for def in &result.definitions {
        let name = Name::str(&def.name);
        if env.contains(&name) {
            result.diagnostics.push(Diagnostic::warning(
                def.range.clone(),
                format!("'{}' shadows existing declaration in environment", def.name),
            ));
        }
    }

    result
}

/// Extract symbols from token stream.
fn extract_symbols_from_tokens(
    _uri: &str,
    tokens: &[oxilean_parse::tokens::Token],
    result: &mut AnalysisResult,
) {
    let mut i = 0;
    while i < tokens.len() {
        let token = &tokens[i];
        let (decl_keyword, sym_kind) = match &token.kind {
            TokenKind::Definition => ("def", SymbolKind::Function),
            TokenKind::Theorem => ("theorem", SymbolKind::Method),
            TokenKind::Lemma => ("theorem", SymbolKind::Method),
            TokenKind::Axiom => ("axiom", SymbolKind::Constant),
            TokenKind::Inductive => ("inductive", SymbolKind::Enum),
            TokenKind::Structure => ("structure", SymbolKind::Struct),
            TokenKind::Class => ("class", SymbolKind::Class),
            TokenKind::Instance => ("instance", SymbolKind::Property),
            TokenKind::Namespace => ("namespace", SymbolKind::Namespace),
            _ => {
                i += 1;
                continue;
            }
        };

        // The next token should be the name
        if i + 1 < tokens.len() {
            if let TokenKind::Ident(name) = &tokens[i + 1].kind {
                let line = if token.span.line > 0 {
                    token.span.line as u32 - 1
                } else {
                    0
                };
                let col = if token.span.column > 0 {
                    token.span.column as u32 - 1
                } else {
                    0
                };
                let name_token = &tokens[i + 1];
                let name_col = if name_token.span.column > 0 {
                    name_token.span.column as u32 - 1
                } else {
                    0
                };
                let name_end = name_col + (name_token.span.end - name_token.span.start) as u32;

                let range = Range::single_line(line, col, name_end);
                let selection_range = Range::single_line(line, name_col, name_end);

                result.symbols.push(DocumentSymbol {
                    name: name.clone(),
                    detail: Some(format!("{} {}", decl_keyword, name)),
                    kind: sym_kind,
                    range: range.clone(),
                    selection_range,
                    children: Vec::new(),
                });

                result.definitions.push(DefinitionInfo {
                    name: name.clone(),
                    kind: sym_kind,
                    range,
                    ty: None,
                    doc: None,
                });
            }
        }
        i += 1;
    }
}

/// Find all references to a name in a token stream.
pub fn find_references_in_document(uri: &str, content: &str, name: &str) -> Vec<Location> {
    let mut locations = Vec::new();
    let mut lexer = Lexer::new(content);
    let tokens = lexer.tokenize();
    for token in &tokens {
        if let TokenKind::Ident(ident) = &token.kind {
            if ident == name {
                let line = if token.span.line > 0 {
                    token.span.line as u32 - 1
                } else {
                    0
                };
                let col = if token.span.column > 0 {
                    token.span.column as u32 - 1
                } else {
                    0
                };
                let end_col = col + (token.span.end - token.span.start) as u32;
                locations.push(Location::new(uri, Range::single_line(line, col, end_col)));
            }
        }
    }
    locations
}

// ── elaborate_document ────────────────────────────────────────────────────────

/// Elaborate a source document through the real parser and elaborator pipeline.
///
/// Runs each declaration in `content` through lexing, parsing, elaboration,
/// and kernel type-checking.  Failures for individual declarations are recorded
/// as error diagnostics and do not abort processing of subsequent declarations.
///
/// Returns a triple:
/// - `decls`: list of `(name, pretty-type)` for every successfully elaborated
///   declaration.
/// - `info_trees`: one `InfoTree` per elaborated declaration, representing the
///   semantic information collected during elaboration.
/// - `diagnostics`: all errors (parse / elaboration / kernel) encountered.
pub fn elaborate_document(
    content: &str,
) -> (Vec<(String, String)>, Vec<InfoTree>, Vec<Diagnostic>) {
    let trimmed = content.trim();
    if trimmed.is_empty() {
        return (Vec::new(), Vec::new(), Vec::new());
    }

    let mut decls: Vec<(String, String)> = Vec::new();
    let mut info_trees: Vec<InfoTree> = Vec::new();
    let mut diagnostics: Vec<Diagnostic> = Vec::new();
    let mut env = Environment::new();

    let mut lexer = Lexer::new(content);
    let tokens = lexer.tokenize();
    let mut parser = Parser::new(tokens);

    loop {
        if parser.is_eof() {
            break;
        }
        let surface_decl = match parser.parse_decl() {
            Ok(located) => located,
            Err(e) => {
                let msg = e.to_string();
                if msg.contains("end of file") || msg.contains("EOF") {
                    break;
                }
                diagnostics.push(Diagnostic::new(
                    Range::single_line(0, 0, 0),
                    DiagnosticSeverity::Error,
                    format!("parse error: {}", msg),
                ));
                // Advance one token to prevent infinite loop when parser is stuck.
                parser.advance();
                continue;
            }
        };

        let inner_decl = &surface_decl.value;

        // Elaborate: surface syntax → PendingDecl
        let pending = match elab_decl_impl(&env, inner_decl) {
            Ok(p) => p,
            Err(e) => {
                diagnostics.push(Diagnostic::new(
                    Range::single_line(0, 0, 0),
                    DiagnosticSeverity::Error,
                    format!("elaboration error: {}", e),
                ));
                continue;
            }
        };

        // Extract the name and type string from the PendingDecl before consuming it.
        let (decl_name_str, decl_ty_str) = match &pending {
            PendingDecl::Definition { name, ty, .. } => (format!("{}", name), format!("{}", ty)),
            PendingDecl::Theorem { name, ty, .. } => (format!("{}", name), format!("{}", ty)),
            PendingDecl::Axiom { name, ty, .. } => (format!("{}", name), format!("{}", ty)),
            PendingDecl::Inductive { name, ty, .. } => (format!("{}", name), format!("{}", ty)),
            PendingDecl::Opaque { name, ty, .. } => (format!("{}", name), format!("{}", ty)),
        };

        // Convert PendingDecl → kernel Declaration for type-checking.
        let kernel_decl = pending_to_declaration(pending);

        // Kernel type-check
        match check_declaration(&mut env, kernel_decl) {
            Ok(()) => {
                decls.push((decl_name_str.clone(), decl_ty_str.clone()));

                // Build a minimal InfoTree recording this declaration's name and type.
                let mut builder = InfoTreeBuilder::new();
                let name_kernel = Name::str(&decl_name_str);
                builder.push_command_info(0, content.len(), decl_name_str, Some(name_kernel));
                let trees = builder.finish();
                info_trees.extend(trees);
            }
            Err(e) => {
                diagnostics.push(Diagnostic::new(
                    Range::single_line(0, 0, 0),
                    DiagnosticSeverity::Error,
                    format!("type error in '{}': {}", decl_name_str, e),
                ));
            }
        }
    }

    (decls, info_trees, diagnostics)
}

/// Convert a [`PendingDecl`] into a kernel [`Declaration`] suitable for
/// [`check_declaration`].  Inductive declarations are approximated as axioms
/// (the same strategy used in the command checker).
fn pending_to_declaration(pending: PendingDecl) -> Declaration {
    match pending {
        PendingDecl::Definition { name, ty, val, .. } => Declaration::Definition {
            name,
            univ_params: vec![],
            ty,
            val,
            hint: ReducibilityHint::Regular(0),
        },
        PendingDecl::Theorem {
            name, ty, proof, ..
        } => Declaration::Theorem {
            name,
            univ_params: vec![],
            ty,
            val: proof,
        },
        PendingDecl::Axiom { name, ty, .. } => Declaration::Axiom {
            name,
            univ_params: vec![],
            ty,
        },
        PendingDecl::Inductive { name, ty, .. } => Declaration::Axiom {
            name,
            univ_params: vec![],
            ty,
        },
        PendingDecl::Opaque { name, ty, val } => Declaration::Opaque {
            name,
            univ_params: vec![],
            ty,
            val,
        },
    }
}

// ── AnalysisCache ─────────────────────────────────────────────────────────────

/// Cache for analysis results.
#[derive(Debug, Default)]
pub struct AnalysisCache {
    /// Cached results by URI.
    results: HashMap<String, (i64, AnalysisResult)>,
}

impl AnalysisCache {
    /// Create a new empty cache.
    pub fn new() -> Self {
        Self {
            results: HashMap::new(),
        }
    }

    /// Get a cached result if the version matches.
    pub fn get(&self, uri: &str, version: i64) -> Option<&AnalysisResult> {
        self.results
            .get(uri)
            .filter(|(v, _)| *v == version)
            .map(|(_, r)| r)
    }

    /// Store a result.
    pub fn store(&mut self, uri: impl Into<String>, version: i64, result: AnalysisResult) {
        self.results.insert(uri.into(), (version, result));
    }

    /// Invalidate a cached result.
    pub fn invalidate(&mut self, uri: &str) {
        self.results.remove(uri);
    }

    /// Clear all cached results.
    pub fn clear(&mut self) {
        self.results.clear();
    }
}

// ── Utility functions ─────────────────────────────────────────────────────────

/// Get hover information for Lean keywords.
pub fn get_keyword_hover(word: &str) -> Option<String> {
    let info = match word {
        "def" | "definition" => {
            "**def** -- Define a new function or value.\n\n```lean\ndef name : Type := value\n```"
        }
        "theorem" | "lemma" => {
            "**theorem** -- State and prove a proposition.\n\n```lean\ntheorem name : Prop := proof\n```"
        }
        "axiom" => {
            "**axiom** -- Postulate a type without proof.\n\n```lean\naxiom name : Type\n```"
        }
        "inductive" => {
            "**inductive** -- Define an inductive type.\n\n```lean\ninductive Name where\n  | ctor : Name\n```"
        }
        "structure" => "**structure** -- Define a record type with named fields.",
        "class" => "**class** -- Define a type class for ad-hoc polymorphism.",
        "instance" => "**instance** -- Provide a type class instance.",
        "fun" => "**fun** -- Lambda abstraction.\n\n```lean\nfun x => x + 1\n```",
        "forall" => "**forall** -- Universal quantification / dependent function type.",
        "match" => "**match** -- Pattern matching.\n\n```lean\nmatch x with\n  | pattern => result\n```",
        "let" => "**let** -- Local binding.\n\n```lean\nlet x := value\nin body\n```",
        "if" => "**if** -- Conditional expression.\n\n```lean\nif cond then a else b\n```",
        "do" => "**do** -- Do-notation for monadic code.",
        "by" => "**by** -- Enter tactic mode to construct a proof.",
        "sorry" => "**sorry** -- Placeholder for incomplete proofs (axiom).",
        "Prop" => "**Prop** -- The type of propositions (`Sort 0`).",
        "Type" => "**Type** -- The type of types (`Sort 1`).",
        "Sort" => "**Sort** -- The type of a universe level.",
        "where" => "**where** -- Introduce local definitions after a declaration.",
        "have" => "**have** -- Introduce a local hypothesis.",
        "show" => "**show** -- Annotate the expected type of an expression.",
        "namespace" => "**namespace** -- Open a namespace for declarations.",
        "section" => "**section** -- Begin a section for local variables.",
        "open" => "**open** -- Open a namespace to use its names unqualified.",
        "import" => "**import** -- Import definitions from another module.",
        _ => return None,
    };
    Some(info.to_string())
}

/// Simple document formatting: normalize whitespace.
pub fn format_document(content: &str) -> Vec<TextEdit> {
    let mut edits = Vec::new();
    let lines: Vec<&str> = content.lines().collect();

    for (i, line) in lines.iter().enumerate() {
        // Remove trailing whitespace
        let trimmed_end = line.trim_end();
        if trimmed_end.len() != line.len() {
            edits.push(TextEdit::new(
                Range::single_line(i as u32, trimmed_end.len() as u32, line.len() as u32),
                "",
            ));
        }
    }

    // Ensure file ends with a newline
    if !content.is_empty() && !content.ends_with('\n') {
        let last_line = lines.len().saturating_sub(1);
        let last_col = lines.last().map_or(0, |l| l.len());
        edits.push(TextEdit::new(
            Range::single_line(last_line as u32, last_col as u32, last_col as u32),
            "\n",
        ));
    }

    edits
}

/// Create a code action JSON value.
pub fn make_code_action(
    title: &str,
    _uri: &str,
    edits: Vec<TextEdit>,
) -> super::json_rpc::JsonValue {
    use super::json_rpc::JsonValue;
    let mut entries = vec![
        ("title".to_string(), JsonValue::String(title.to_string())),
        (
            "kind".to_string(),
            JsonValue::String("quickfix".to_string()),
        ),
    ];
    if !edits.is_empty() {
        entries.push((
            "edit".to_string(),
            JsonValue::Object(vec![(
                "changes".to_string(),
                JsonValue::Array(edits.iter().map(|e| e.to_json()).collect()),
            )]),
        ));
    }
    JsonValue::Object(entries)
}
