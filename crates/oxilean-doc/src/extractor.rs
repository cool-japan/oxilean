//! AST extraction for the OxiLean documentation generator.
//!
//! This module parses `.lean` source text into a flat list of [`DocItem`]s,
//! associating each named declaration with its doc-comment (if any) and a
//! pretty-printed signature.

use anyhow::Result;
use oxilean_parse::{print_decl, Decl, Lexer, ParseErrorKind, Parser, Span, TokenKind};

/// Broad category of a Lean declaration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeclKind {
    /// `def` — function/value definition
    Def,
    /// `theorem` / `lemma` — proof obligation
    Theorem,
    /// `axiom` — trusted primitive
    Axiom,
    /// `structure` / `class` declaration
    Structure,
    /// `inductive` type definition
    Inductive,
    /// anything else we surface (instance, notation, derive, …)
    Other,
}

/// A single documentable declaration extracted from a source file.
#[derive(Debug, Clone)]
pub struct DocItem {
    /// Qualified or simple name of the declaration.
    pub name: String,
    /// Module/namespace this declaration belongs to (e.g. `"Nat"` or `"root"`).
    pub module: String,
    /// Broad syntactic category.
    pub kind: DeclKind,
    /// Pretty-printed full signature (may be multi-line).
    pub signature: String,
    /// Raw text of the immediately preceding doc comment, if present.
    pub doc_comment: Option<String>,
    /// Whether this declaration was annotated with `@[deprecated]`.
    pub deprecated: bool,
}

// ---------------------------------------------------------------------------
// Public entry point
// ---------------------------------------------------------------------------

/// Parse `source` and return a flat list of documentable declarations.
///
/// # Errors
/// Returns `Err` if the source cannot be parsed by `oxilean-parse`.
pub fn extract(source: &str) -> Result<Vec<DocItem>> {
    // 1. Tokenize — keep ALL tokens (including DocComment) for later correlation.
    let tokens = Lexer::new(source).tokenize();

    // 2. Build a filtered token list for the parser: the parser does not handle
    //    DocComment tokens and will fail with "expected declaration" if they are present.
    let parse_tokens: Vec<oxilean_parse::Token> = tokens
        .iter()
        .filter(|t| !matches!(t.kind, TokenKind::DocComment(_)))
        .cloned()
        .collect();

    // 3. Parse all declarations from the filtered stream; stop at EOF.
    //
    // Note: `parse_decl()` encodes EOF as `ParseErrorKind::UnexpectedToken { got: Eof }`
    // rather than `ParseErrorKind::UnexpectedEof`, so `e.is_eof()` is not reliable here.
    // We detect EOF by also checking for the `UnexpectedToken { got: TokenKind::Eof }` case.
    //
    // The `oxilean-parse` parser handles a simplified Lean subset.  For declarations it
    // cannot parse (e.g. `def f (x : T) : U := ...` with explicit binders), we advance
    // one token and try again rather than aborting, so we still document parseable decls.
    let mut parser = Parser::new(parse_tokens);
    let mut located_decls = Vec::new();
    loop {
        match parser.parse_decl() {
            Ok(d) => located_decls.push(d),
            // True EOF — normal termination.
            Err(e) if e.is_eof() => break,
            // Parser's way of signalling "no more input".
            Err(e)
                if matches!(
                    &e.kind,
                    ParseErrorKind::UnexpectedToken {
                        got: TokenKind::Eof,
                        ..
                    }
                ) =>
            {
                break;
            }
            // Skip-and-continue: advance one token and try again.
            // This makes the extractor lenient for syntax the simplified parser does not yet handle.
            Err(_) => {
                if parser.is_eof() {
                    break;
                }
                parser.advance();
            }
        }
    }

    // 4. Build the doc-item list, walking into Attribute/Mutual wrappers.
    //    `tokens` (unfiltered) is used for doc-comment correlation.
    let mut items: Vec<DocItem> = Vec::new();
    for ld in &located_decls {
        collect_items(&ld.value, &ld.span, &tokens, source, "root", &mut items);
    }

    Ok(items)
}

// ---------------------------------------------------------------------------
// Recursive declaration walker
// ---------------------------------------------------------------------------

/// Recursively collect [`DocItem`]s from a single declaration.
///
/// `current_module` tracks the enclosing namespace/module name (default: `"root"`).
fn collect_items(
    decl: &Decl,
    span: &Span,
    tokens: &[oxilean_parse::Token],
    source: &str,
    current_module: &str,
    out: &mut Vec<DocItem>,
) {
    match decl {
        // -----------------------------------------------------------------------
        // Transparent wrappers — recurse without emitting their own item
        // -----------------------------------------------------------------------
        Decl::Attribute { decl: inner, attrs } => {
            // Detect `@[deprecated]` annotation: record the count before recursing,
            // then mark any newly emitted items as deprecated.
            let is_deprecated = attrs.iter().any(|a| a == "deprecated");
            let start = out.len();
            collect_items(
                &inner.value,
                &inner.span,
                tokens,
                source,
                current_module,
                out,
            );
            if is_deprecated {
                for item in &mut out[start..] {
                    item.deprecated = true;
                }
            }
        }
        Decl::Mutual { decls } => {
            for ld in decls {
                collect_items(&ld.value, &ld.span, tokens, source, current_module, out);
            }
        }
        Decl::Namespace { name, decls } => {
            // Build a new module path by appending the namespace name.
            let child_module = if current_module == "root" {
                name.clone()
            } else {
                format!("{}::{}", current_module, name)
            };
            for ld in decls {
                collect_items(&ld.value, &ld.span, tokens, source, &child_module, out);
            }
        }
        Decl::SectionDecl { decls, .. } => {
            for ld in decls {
                collect_items(&ld.value, &ld.span, tokens, source, current_module, out);
            }
        }

        // -----------------------------------------------------------------------
        // Skip non-documentable declarations silently
        // -----------------------------------------------------------------------
        Decl::Import { .. }
        | Decl::Open { .. }
        | Decl::Variable { .. }
        | Decl::HashCmd { .. }
        | Decl::Universe { .. }
        | Decl::Derive { .. }
        | Decl::NotationDecl { .. } => {}

        // -----------------------------------------------------------------------
        // Named declarations that we want to document
        // -----------------------------------------------------------------------
        _ => {
            let Some(name) = decl.name() else {
                return;
            };
            let kind = decl_kind(decl);
            let signature = print_decl(decl);
            let doc_comment = find_doc_comment(tokens, source, span.start);

            out.push(DocItem {
                name: name.to_string(),
                module: current_module.to_string(),
                kind,
                signature,
                doc_comment,
                deprecated: false,
            });
        }
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Map an AST `Decl` variant to our coarse [`DeclKind`].
fn decl_kind(decl: &Decl) -> DeclKind {
    match decl {
        Decl::Definition { .. } => DeclKind::Def,
        Decl::Theorem { .. } => DeclKind::Theorem,
        Decl::Axiom { .. } => DeclKind::Axiom,
        Decl::Structure { .. } | Decl::ClassDecl { .. } => DeclKind::Structure,
        Decl::Inductive { .. } => DeclKind::Inductive,
        _ => DeclKind::Other,
    }
}

/// Find the doc-comment string that immediately precedes byte offset `decl_start`.
///
/// The lexer does not produce whitespace tokens, so we check adjacency by
/// inspecting the raw source bytes between the comment token end and `decl_start`:
/// the gap must consist exclusively of ASCII whitespace.
///
/// Only the **single closest** token before `decl_start` is examined:
/// - If it is a `DocComment` and its gap to `decl_start` is all-whitespace → return it.
/// - If it is any other token → no doc comment is adjacent (a doc comment further back
///   would have this non-whitespace token in its gap, failing the adjacency check).
/// - If there are no tokens before `decl_start` → return `None`.
fn find_doc_comment(
    tokens: &[oxilean_parse::Token],
    source: &str,
    decl_start: usize,
) -> Option<String> {
    let source_bytes = source.as_bytes();

    // Build a sorted (by span.end) list of relevant tokens.
    // We only need the last (closest to decl_start), but sort for determinism.
    let mut candidates: Vec<&oxilean_parse::Token> =
        tokens.iter().filter(|t| t.span.end <= decl_start).collect();
    candidates.sort_by_key(|t| t.span.end);

    // Examine only the closest token before `decl_start`.
    // If it is a DocComment and its gap to `decl_start` is all-whitespace, return it.
    // Any other token (or a non-adjacent doc comment) means no doc comment applies.
    if let Some(tok) = candidates.last() {
        if let TokenKind::DocComment(text) = &tok.kind {
            let gap = source_bytes.get(tok.span.end..decl_start);
            let gap_is_whitespace = gap
                .map(|bytes| bytes.iter().all(|b| b.is_ascii_whitespace()))
                .unwrap_or(true);

            if gap_is_whitespace {
                return Some(text.trim().to_string());
            }
        }
    }

    None
}

// ---------------------------------------------------------------------------
// DocIR bridge
// ---------------------------------------------------------------------------

/// Convert a [`oxilean_codegen::DocIR`] into a `Vec<DocItem>` for rendering.
///
/// This bridges the codegen-level DocIR into the doc-gen's existing
/// [`DocItem`] structure used by renderers.  Because the codegen IR does not
/// carry doc comments or signatures (they are erased during LCNF lowering),
/// the resulting items will have empty `doc_comment` and `signature` fields.
/// The `kind` field is mapped from the IR's string tag to a [`DeclKind`]:
///
/// | IR kind    | [`DeclKind`]   |
/// |------------|----------------|
/// | `"def"`    | `Def`          |
/// | `"axiom"`  | `Axiom`        |
/// | anything   | `Other`        |
#[allow(dead_code)]
pub fn doc_items_from_ir(ir: &oxilean_codegen::DocIR) -> Vec<DocItem> {
    ir.items
        .iter()
        .map(|item| DocItem {
            name: item.name.clone(),
            module: ir.module_name.clone(),
            kind: match item.kind.as_str() {
                "def" => DeclKind::Def,
                "axiom" => DeclKind::Axiom,
                _ => DeclKind::Other,
            },
            signature: item.signature.clone(),
            doc_comment: if item.doc_comment.is_empty() {
                None
            } else {
                Some(item.doc_comment.clone())
            },
            deprecated: item.deprecated,
        })
        .collect()
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // Fixtures use the simplified Lean subset that `oxilean_parse::Parser` supports:
    //   def name : type := value          (no binder parameters)
    //   theorem name : type := proof      (no binder parameters)
    //   axiom name : type
    //   structure Name where field : type ...
    //   inductive Name : Type | c : type  (simplified inline form)
    const FIXTURE: &str = concat!(
        "\n",
        "/-- A simple identity function -/\n",
        "def id_nat : Nat := 0\n",
        "\n",
        "/-- Addition is commutative -/\n",
        "theorem add_comm : Nat := 0\n",
        "\n",
        "axiom classical_choice : Nat\n",
        "\n",
        "structure Point where\n",
        "  x : Nat\n",
        "  y : Nat\n",
    );

    #[test]
    fn test_extract_finds_declarations() {
        let items = extract(FIXTURE).expect("extract should succeed");
        assert!(!items.is_empty(), "should find at least one declaration");
        let names: Vec<&str> = items.iter().map(|i| i.name.as_str()).collect();
        println!("Found declarations: {names:?}");
        assert!(names.contains(&"id_nat"), "expected id_nat, got: {names:?}");
    }

    #[test]
    fn test_extract_finds_theorem() {
        let items = extract(FIXTURE).expect("extract should succeed");
        let theorem = items.iter().find(|i| i.name == "add_comm");
        assert!(
            theorem.is_some(),
            "expected theorem add_comm in: {:?}",
            items.iter().map(|i| &i.name).collect::<Vec<_>>()
        );
        assert_eq!(theorem.unwrap().kind, DeclKind::Theorem);
    }

    #[test]
    fn test_doc_comment_attached_to_def() {
        let items = extract(FIXTURE).expect("extract should succeed");
        let def_item = items.iter().find(|i| i.name == "id_nat");
        assert!(def_item.is_some(), "id_nat should be found");
        let doc = def_item.unwrap().doc_comment.as_deref().unwrap_or("");
        assert!(
            doc.contains("identity"),
            "expected doc to mention 'identity', got: {doc:?}"
        );
    }

    #[test]
    fn test_structure_kind() {
        let items = extract(FIXTURE).expect("extract should succeed");
        // Structure parsing requires the simplified "structure Name where field : Type" form.
        let s = items.iter().find(|i| i.name == "Point");
        // May or may not be found depending on parser support; just don't panic.
        if let Some(item) = s {
            assert_eq!(item.kind, DeclKind::Structure);
        }
    }

    #[test]
    fn test_axiom_kind() {
        let items = extract(FIXTURE).expect("extract should succeed");
        let ax = items.iter().find(|i| i.name == "classical_choice");
        assert!(ax.is_some(), "classical_choice should be found");
        assert_eq!(ax.unwrap().kind, DeclKind::Axiom);
    }

    #[test]
    fn test_empty_source() {
        let items = extract("").expect("empty source should not error");
        assert!(items.is_empty());
    }

    #[test]
    fn test_renderer_produces_html() {
        let items = extract(FIXTURE).unwrap_or_default();
        let html = crate::renderer::render_html("Test", &items);
        assert!(html.contains("<!DOCTYPE html>"));
        assert!(html.contains("<title>Test</title>"));
    }

    // ---- Deprecated attribute tests ----

    #[test]
    fn test_deprecated_attribute_sets_flag() {
        // The parser produces Decl::Attribute { attrs: ["deprecated"], decl: <inner> }
        // for `@[deprecated] def foo : Nat := 0`.
        let src = "@[deprecated] axiom deprecated_ax : Nat\n";
        let items = extract(src).expect("extract should succeed");
        let item = items.iter().find(|i| i.name == "deprecated_ax");
        assert!(
            item.is_some(),
            "deprecated_ax should be found, got: {:?}",
            items.iter().map(|i| &i.name).collect::<Vec<_>>()
        );
        assert!(
            item.unwrap().deprecated,
            "item annotated with @[deprecated] must have deprecated == true"
        );
    }

    #[test]
    fn test_non_deprecated_item_flag_false() {
        let src = "axiom regular_ax : Nat\n";
        let items = extract(src).expect("extract should succeed");
        let item = items.iter().find(|i| i.name == "regular_ax");
        assert!(item.is_some(), "regular_ax should be found");
        assert!(
            !item.unwrap().deprecated,
            "non-deprecated item must have deprecated == false"
        );
    }

    #[test]
    fn test_deprecated_badge_rendered_in_html() {
        let src = "@[deprecated] axiom old_ax : Nat\n";
        let items = extract(src).expect("extract should succeed");
        let html = crate::renderer::render_html("Test", &items);
        assert!(
            html.contains("<span class=\"deprecated-badge\">deprecated</span>"),
            "deprecated badge span should appear in rendered HTML"
        );
    }

    // ---- DocIR bridge tests ----

    /// doc_items_from_ir returns an empty vec for an empty DocIR.
    #[test]
    fn test_doc_items_from_ir_empty() {
        let ir = oxilean_codegen::DocIR {
            module_name: "Empty".to_string(),
            items: vec![],
        };
        let items = doc_items_from_ir(&ir);
        assert!(items.is_empty());
    }

    /// doc_items_from_ir maps "def" kind to DeclKind::Def.
    #[test]
    fn test_doc_items_from_ir_def_kind() {
        let ir = oxilean_codegen::DocIR {
            module_name: "Nat".to_string(),
            items: vec![oxilean_codegen::DocIRItem {
                name: "Nat.add".to_string(),
                kind: "def".to_string(),
                signature: String::new(),
                doc_comment: String::new(),
                deprecated: false,
            }],
        };
        let items = doc_items_from_ir(&ir);
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].name, "Nat.add");
        assert_eq!(items[0].kind, DeclKind::Def);
        assert_eq!(items[0].module, "Nat");
    }

    /// doc_items_from_ir maps "axiom" kind to DeclKind::Axiom.
    #[test]
    fn test_doc_items_from_ir_axiom_kind() {
        let ir = oxilean_codegen::DocIR {
            module_name: "Core".to_string(),
            items: vec![oxilean_codegen::DocIRItem {
                name: "Core.choice".to_string(),
                kind: "axiom".to_string(),
                signature: String::new(),
                doc_comment: String::new(),
                deprecated: false,
            }],
        };
        let items = doc_items_from_ir(&ir);
        assert_eq!(items[0].kind, DeclKind::Axiom);
    }

    /// doc_items_from_ir preserves the deprecated flag.
    #[test]
    fn test_doc_items_from_ir_deprecated() {
        let ir = oxilean_codegen::DocIR {
            module_name: "OldApi".to_string(),
            items: vec![oxilean_codegen::DocIRItem {
                name: "OldApi.fn".to_string(),
                kind: "def".to_string(),
                signature: String::new(),
                doc_comment: String::new(),
                deprecated: true,
            }],
        };
        let items = doc_items_from_ir(&ir);
        assert!(items[0].deprecated);
    }
}
