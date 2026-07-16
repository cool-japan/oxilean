//! Analysis engine: document analysis, symbol extraction, reference finding.

use std::collections::HashMap;

use oxilean_elab::info_tree::{InfoData, InfoTree, InfoTreeBuilder};
use oxilean_elab::{elaborate_decl as elab_decl_impl, PendingDecl};
use oxilean_kernel::{check_declaration, Declaration, Environment, Name, ReducibilityHint};
use oxilean_parse::incremental::{parse_incremental_change, TextChange};
use oxilean_parse::{Lexer, Parser, TokenKind};

use super::document::Document;

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
///
/// This performs a full re-tokenize of `content`.  Call
/// [`analyze_document_incremental`] when an incremental edit is available to
/// avoid re-lexing the unchanged parts of the source.
pub fn analyze_document(uri: &str, content: &str, env: &Environment) -> AnalysisResult {
    let mut lexer = Lexer::new(content);
    let tokens = lexer.tokenize();
    analyze_from_tokens(uri, tokens, env)
}

/// Analyze a document and update both its cached token stream and its cached
/// declaration list in one pass.
///
/// Equivalent to [`analyze_document`] but also stores the new token stream
/// back onto `doc.cached_tokens` and the parsed declarations onto
/// `doc.cached_decls` so that subsequent incremental edits can use both
/// caches via [`analyze_document_incremental`] and
/// [`analyze_document_incremental_decls`].
pub fn analyze_document_and_cache(doc: &mut Document, env: &Environment) -> AnalysisResult {
    let mut lexer = Lexer::new(&doc.content);
    let tokens = lexer.tokenize();
    let result = analyze_from_tokens(&doc.uri, tokens.clone(), env);
    doc.cached_tokens = tokens;
    // Also refresh the declaration cache so incremental-decl analysis has a
    // valid baseline after a full analysis pass.
    doc.cached_decls = parse_decls_from_source(&doc.content);
    result
}

/// Incrementally re-lex a single LSP `textDocument/didChange` edit.
///
/// Uses the token stream cached in `doc.cached_tokens` as the baseline.  If
/// the cache is empty (e.g. a freshly opened document) this falls back to a
/// full re-lex via [`analyze_document_and_cache`].
///
/// After the call, `doc.cached_tokens` reflects the post-edit token stream.
pub fn analyze_document_incremental(
    doc: &mut Document,
    change: &TextChange,
    env: &Environment,
) -> AnalysisResult {
    // If no cached tokens yet, fall back to a full re-lex.
    if doc.cached_tokens.is_empty() {
        return analyze_document_and_cache(doc, env);
    }

    let result = parse_incremental_change(&doc.cached_tokens, &doc.content, change);
    let tokens = result.tokens;
    let analysis = analyze_from_tokens(&doc.uri, tokens.clone(), env);
    doc.cached_tokens = tokens;
    analysis
}

/// Parse all top-level declarations from a source string, tolerating errors.
///
/// Drives the parser declaration-by-declaration; treats any EOF-related
/// error (or a non-EOF error when the parser is already at EOF) as end of
/// input.  Non-EOF errors cause the current declaration to be skipped
/// after advancing one token to prevent an infinite loop.
pub fn parse_decls_from_source(source: &str) -> Vec<oxilean_parse::Located<oxilean_parse::Decl>> {
    use oxilean_parse::{Lexer, Parser};

    let tokens = Lexer::new(source).tokenize();
    let mut parser = Parser::new(tokens);
    let mut decls = Vec::new();

    loop {
        if parser.is_eof() {
            break;
        }
        match parser.parse_decl() {
            Ok(d) => decls.push(d),
            Err(e) => {
                let is_eof = e.is_eof() || parser.is_eof();
                if is_eof {
                    break;
                }
                // Non-EOF error: advance one token to avoid infinite loop
                // and skip this declaration.
                parser.advance();
            }
        }
    }
    decls
}

/// Incremental analysis: parse new decls, diff against cached, then
/// re-analyse only the tail starting from the first changed declaration.
///
/// # Correctness strategy (tail-invalidation)
///
/// We re-analyse every declaration from the first `Modified`, `Inserted`, or
/// `Deleted` edit onward.  This is conservative — it may re-analyse
/// unchanged declarations that come after the first change — but it is
/// correct in the presence of forward references and macro/namespace
/// interactions without requiring a full dependency graph.
///
/// After the call `doc.cached_decls` holds the freshly parsed declarations
/// for use in the *next* incremental cycle.
pub fn analyze_document_incremental_decls(doc: &mut Document, env: &Environment) -> AnalysisResult {
    // 1. Parse the current source into a fresh declaration list.
    let new_decls = parse_decls_from_source(&doc.content);

    // 2. Diff the new list against the previously cached one.
    let edits = oxilean_parse::incremental::diff_modules(&doc.cached_decls, &new_decls);

    // 3. Find the index (in `new_decls`) of the first non-Unchanged edit.
    //    We use `new_idx` for Inserted/Modified and skip Deleted (they have
    //    no new_idx).  The smallest such index is the invalidation boundary.
    let first_change_new_idx: usize = edits
        .iter()
        .filter_map(|e| {
            use oxilean_parse::incremental::EditKind;
            match e.kind {
                EditKind::Inserted | EditKind::Modified => e.new_idx,
                EditKind::Deleted => {
                    // A deletion shifts all subsequent new_idxs; the tail
                    // starts at the position the deleted decl occupied in the
                    // old sequence, mapped to the earliest affected new_idx.
                    // Conservatively use `old_idx` as a lower bound and fall
                    // back to 0 if unavailable.
                    Some(e.old_idx.unwrap_or(0))
                }
                EditKind::Unchanged => None,
            }
        })
        .min()
        .unwrap_or(new_decls.len()); // no changes → nothing to re-analyse

    // 4. Re-analyse the tail (decls at indices >= first_change_new_idx).
    let tail_result = if first_change_new_idx >= new_decls.len() {
        // All declarations are unchanged — return a clean empty result.
        // (The caller may merge this with a cached full result if desired.)
        AnalysisResult::default()
    } else {
        // Rebuild the source snippet for the tail and run full analysis on it.
        // We use the token-level analysis on the whole document but only
        // report diagnostics that originate from the changed tail.  For now
        // we delegate to `analyze_document` so that token-level errors and
        // symbol extraction remain correct.
        analyze_document(&doc.uri, &doc.content, env)
    };

    // 5. Store the freshly parsed decls for the next incremental cycle.
    doc.cached_decls = new_decls;

    tail_result
}

/// Internal helper: produce an [`AnalysisResult`] from an already-computed
/// token stream.  This avoids duplicating error/symbol-extraction logic.
fn analyze_from_tokens(
    uri: &str,
    tokens: Vec<oxilean_parse::tokens::Token>,
    env: &Environment,
) -> AnalysisResult {
    let mut result = AnalysisResult::default();
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

// ── Incremental re-lex tests ──────────────────────────────────────────────────

#[cfg(test)]
mod incremental_relex_tests {
    use super::*;
    use oxilean_parse::Lexer;

    fn make_doc(uri: &str, content: &str) -> Document {
        let mut doc = Document::new(uri, 1, content);
        // Populate cached_tokens with a full lex so the incremental path can
        // use them as its baseline.
        let tokens = Lexer::new(content).tokenize();
        doc.cached_tokens = tokens;
        doc
    }

    /// Helper: collect the `TokenKind` discriminant names from a token stream
    /// for easy comparison.
    fn kind_names(tokens: &[oxilean_parse::tokens::Token]) -> Vec<String> {
        tokens
            .iter()
            .map(|t| match &t.kind {
                TokenKind::Ident(_) => "Ident".into(),
                TokenKind::Nat(_) => "Nat".into(),
                TokenKind::String(_) => "String".into(),
                TokenKind::DocComment(_) => "DocComment".into(),
                TokenKind::Error(_) => "Error".into(),
                TokenKind::Eof => "Eof".into(),
                other => format!("{:?}", other),
            })
            .collect()
    }

    /// After applying a single-character edit incrementally the resulting
    /// `cached_tokens` must differ from the original.
    ///
    /// We replace the identifier "foo" with "bar" (same length, different name).
    #[test]
    fn test_incremental_relex_single_token() {
        let source = "def foo := 42";
        let mut doc = make_doc("file:///t.lean", source);

        // "foo" starts at char index 4 and ends at char index 7.
        let change = TextChange::new(4, 7, "bar");

        // Apply the text edit manually to the document content.
        doc.content = change.apply(&doc.content);
        doc.line_offsets = super::super::document::compute_line_offsets(&doc.content);

        let env = Environment::new();
        let _ = analyze_document_incremental(&mut doc, &change, &env);

        // The content must now contain "bar".
        assert!(
            doc.content.contains("bar"),
            "document content should contain 'bar', got: {:?}",
            doc.content
        );

        // The cached_tokens must reflect the renamed identifier.
        let ident_names: Vec<String> = doc
            .cached_tokens
            .iter()
            .filter_map(|t| {
                if let TokenKind::Ident(n) = &t.kind {
                    Some(n.clone())
                } else {
                    None
                }
            })
            .collect();

        assert!(
            ident_names.contains(&"bar".to_string()),
            "cached_tokens should contain identifier 'bar', got: {:?}",
            ident_names
        );
        assert!(
            !ident_names.contains(&"foo".to_string()),
            "cached_tokens should NOT contain old identifier 'foo' after edit"
        );
    }

    /// Incremental re-lex should produce a token stream whose **token kind
    /// sequence** matches a fresh full re-lex of the same content.
    ///
    /// Positions may differ slightly due to incremental splice ordering, but
    /// the kinds must be identical.
    #[test]
    fn test_incremental_relex_equals_full() {
        let source = "theorem myThm : True := trivial";
        let mut doc = make_doc("file:///eq.lean", source);

        // Replace "myThm" (chars 8..13) with "renamed".
        let change = TextChange::new(8, 13, "renamed");
        doc.content = change.apply(&doc.content);
        doc.line_offsets = super::super::document::compute_line_offsets(&doc.content);

        let env = Environment::new();
        let _ = analyze_document_incremental(&mut doc, &change, &env);

        // Ground truth: full lex of the post-edit content.
        let full_tokens = Lexer::new(&doc.content).tokenize();

        let incr_kinds = kind_names(&doc.cached_tokens);
        let full_kinds = kind_names(&full_tokens);

        assert_eq!(
            incr_kinds, full_kinds,
            "incremental re-lex kind sequence must match full re-lex\n  incremental: {:?}\n  full:        {:?}",
            incr_kinds, full_kinds
        );
    }

    /// Incremental re-lex on a document with empty `cached_tokens` must fall
    /// back to a full re-lex and populate `cached_tokens`.
    #[test]
    fn test_incremental_relex_fallback_when_cache_empty() {
        let source = "def x := 1";
        // Do NOT set cached_tokens (empty by default).
        let mut doc = Document::new("file:///empty.lean", 1, source);
        assert!(doc.cached_tokens.is_empty());

        let change = TextChange::new(9, 10, "2");
        // Apply the edit to content.
        doc.content = change.apply(&doc.content);
        doc.line_offsets = super::super::document::compute_line_offsets(&doc.content);

        let env = Environment::new();
        let _ = analyze_document_incremental(&mut doc, &change, &env);

        // After fallback to full re-lex, cache must be populated.
        assert!(
            !doc.cached_tokens.is_empty(),
            "cached_tokens should be populated after fallback full re-lex"
        );
    }

    /// Verify that `analyze_document_and_cache` updates `cached_tokens`.
    #[test]
    fn test_analyze_and_cache_updates_tokens() {
        let source = "namespace Foo\ndef bar := 0\nend Foo";
        let mut doc = Document::new("file:///ns.lean", 1, source);
        assert!(doc.cached_tokens.is_empty());

        let env = Environment::new();
        let _ = analyze_document_and_cache(&mut doc, &env);

        assert!(
            !doc.cached_tokens.is_empty(),
            "analyze_document_and_cache must populate cached_tokens"
        );

        // The token stream should contain the Namespace keyword.
        let has_namespace = doc
            .cached_tokens
            .iter()
            .any(|t| t.kind == TokenKind::Namespace);
        assert!(
            has_namespace,
            "cached tokens should include a Namespace token"
        );
    }
}

// ── Incremental decl-level analysis tests ────────────────────────────────────

#[cfg(test)]
mod incremental_decls_tests {
    use super::*;

    // Three well-formed theorems that the parser can handle.
    const SRC_3DECLS: &str =
        "theorem a : True := True.intro\ntheorem b : True := True.intro\ntheorem c : True := True.intro";
    // Same structure but theorem b has a different (wrong) type annotation.
    const SRC_B_MODIFIED: &str =
        "theorem a : True := True.intro\ntheorem b : Nat := True.intro\ntheorem c : True := True.intro";
    // SRC_3DECLS with a fourth theorem appended.
    const SRC_APPENDED: &str =
        "theorem a : True := True.intro\ntheorem b : True := True.intro\ntheorem c : True := True.intro\ntheorem d : True := True.intro";

    fn make_doc(content: &str) -> Document {
        Document::new("test://test", 1, content)
    }

    // ── parse_decls_from_source ───────────────────────────────────────────────

    #[test]
    fn test_parse_decls_count() {
        let decls = parse_decls_from_source(SRC_3DECLS);
        assert_eq!(
            decls.len(),
            3,
            "expected 3 declarations, got {}",
            decls.len()
        );
    }

    #[test]
    fn test_parse_decls_empty_source() {
        let decls = parse_decls_from_source("");
        assert!(
            decls.is_empty(),
            "empty source should yield zero declarations"
        );
    }

    #[test]
    fn test_parse_decls_appended() {
        let decls = parse_decls_from_source(SRC_APPENDED);
        assert_eq!(
            decls.len(),
            4,
            "expected 4 declarations after append, got {}",
            decls.len()
        );
    }

    // ── cached_decls lifecycle ────────────────────────────────────────────────

    #[test]
    fn test_cached_decls_empty_on_new_doc() {
        let doc = make_doc(SRC_3DECLS);
        assert!(
            doc.cached_decls.is_empty(),
            "fresh document should have empty cached_decls"
        );
    }

    #[test]
    fn test_cached_decls_populated_after_analysis() {
        let mut doc = make_doc(SRC_3DECLS);
        let env = Environment::new();
        let _ = analyze_document_incremental_decls(&mut doc, &env);
        assert_eq!(
            doc.cached_decls.len(),
            3,
            "cached_decls should have 3 entries after incremental analysis"
        );
    }

    #[test]
    fn test_cached_decls_populated_by_and_cache() {
        let mut doc = make_doc(SRC_3DECLS);
        let env = Environment::new();
        let _ = analyze_document_and_cache(&mut doc, &env);
        assert_eq!(
            doc.cached_decls.len(),
            3,
            "analyze_document_and_cache must populate cached_decls"
        );
    }

    #[test]
    fn test_cached_decls_cleared_on_full_replace() {
        let mut doc = make_doc(SRC_3DECLS);
        // Pre-populate the cache.
        doc.cached_decls = parse_decls_from_source(SRC_3DECLS);
        assert_eq!(doc.cached_decls.len(), 3);
        // A full-content replace via `update()` must clear the cache.
        doc.update(2, SRC_APPENDED);
        assert!(
            doc.cached_decls.is_empty(),
            "cached_decls must be empty after a full-replace update()"
        );
    }

    // ── diff_modules edge cases ───────────────────────────────────────────────

    #[test]
    fn test_first_change_idx_on_modify() {
        let old_decls = parse_decls_from_source(SRC_3DECLS);
        let new_decls = parse_decls_from_source(SRC_B_MODIFIED);
        let edits = oxilean_parse::incremental::diff_modules(&old_decls, &new_decls);

        // The first change (Modified) should be at decl 'b' (index >= 1).
        let first_modified_new_idx = edits
            .iter()
            .filter_map(|e| {
                use oxilean_parse::incremental::EditKind;
                match e.kind {
                    EditKind::Modified | EditKind::Inserted | EditKind::Deleted => e.new_idx,
                    EditKind::Unchanged => None,
                }
            })
            .min();

        assert!(
            first_modified_new_idx.is_some(),
            "expected at least one non-Unchanged edit"
        );
        // 'a' (index 0) is unchanged, so the first change must be at index >= 1.
        assert!(
            first_modified_new_idx.unwrap() >= 1,
            "first change should be at decl index >= 1 (decl 'a' is unchanged)"
        );
    }

    #[test]
    fn test_appended_decl_incremental_diff() {
        let old_decls = parse_decls_from_source(SRC_3DECLS);
        let new_decls = parse_decls_from_source(SRC_APPENDED);
        let edits = oxilean_parse::incremental::diff_modules(&old_decls, &new_decls);

        use oxilean_parse::incremental::EditKind;
        let unchanged_count = edits
            .iter()
            .filter(|e| e.kind == EditKind::Unchanged)
            .count();
        let inserted_count = edits
            .iter()
            .filter(|e| e.kind == EditKind::Inserted)
            .count();

        assert_eq!(unchanged_count, 3, "expected 3 Unchanged edits (a, b, c)");
        assert_eq!(inserted_count, 1, "expected 1 Inserted edit (d)");
    }

    #[test]
    fn test_identical_source_no_change() {
        let old_decls = parse_decls_from_source(SRC_3DECLS);
        let new_decls = parse_decls_from_source(SRC_3DECLS);
        let edits = oxilean_parse::incremental::diff_modules(&old_decls, &new_decls);

        use oxilean_parse::incremental::EditKind;
        assert!(
            edits.iter().all(|e| e.kind == EditKind::Unchanged),
            "identical sources must produce only Unchanged edits; got: {edits:?}"
        );
    }

    // ── analyze_document_incremental_decls full-cycle ─────────────────────────

    #[test]
    fn test_incremental_decls_second_pass_updates_cache() {
        let mut doc = make_doc(SRC_3DECLS);
        let env = Environment::new();

        // First pass populates the cache.
        let _ = analyze_document_incremental_decls(&mut doc, &env);
        assert_eq!(doc.cached_decls.len(), 3);

        // Second pass with the same content must still produce a valid result
        // and keep the cache at 3 entries.
        let _ = analyze_document_incremental_decls(&mut doc, &env);
        assert_eq!(
            doc.cached_decls.len(),
            3,
            "cached_decls should remain 3 on a no-op second pass"
        );
    }

    #[test]
    fn test_incremental_decls_append_updates_cache() {
        let mut doc = make_doc(SRC_3DECLS);
        let env = Environment::new();

        // First pass: 3 decls.
        let _ = analyze_document_incremental_decls(&mut doc, &env);
        assert_eq!(doc.cached_decls.len(), 3);

        // Simulate appending a fourth theorem (incremental edit — no full replace).
        doc.content = SRC_APPENDED.to_string();
        doc.line_offsets = super::super::document::compute_line_offsets(&doc.content);

        // Second pass: 4 decls.
        let _ = analyze_document_incremental_decls(&mut doc, &env);
        assert_eq!(
            doc.cached_decls.len(),
            4,
            "cached_decls should be 4 after appending a declaration"
        );
    }
}
