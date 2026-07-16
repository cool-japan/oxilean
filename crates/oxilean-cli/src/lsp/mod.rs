//! LSP (Language Server Protocol) server for OxiLean.
//!
//! Implements a JSON-RPC based language server that provides IDE features
//! such as diagnostics, completions, hover, go-to-definition, and more.
//! Uses only std library + internal oxilean crates (zero external deps).

#![allow(dead_code)]

// ── Existing submodules ───────────────────────────────────────────────────────
pub mod call_hierarchy;
pub mod code_actions;
pub mod completion;
pub mod completion_adv;
pub mod diagnostics;
pub mod diagnostics_adv;
pub mod document_symbols;
pub mod folding_range;
pub mod hover;
pub mod hover_adv;
pub mod semantic_tokens;
pub mod server;
pub mod widgets;

// ── New submodules (refactored from this file) ────────────────────────────────
pub mod analysis;
pub mod document;
pub mod json_rpc;
pub mod lsp_server;
pub mod lsp_types;

// ── Re-exports: JSON-RPC ──────────────────────────────────────────────────────
pub use json_rpc::{
    format_json_value, parse_json_value, JsonRpcError, JsonRpcMessage, JsonValue, INTERNAL_ERROR,
    INVALID_PARAMS, INVALID_REQUEST, METHOD_NOT_FOUND, PARSE_ERROR, REQUEST_CANCELLED,
    SERVER_NOT_INITIALIZED,
};

// ── Re-exports: LSP types ─────────────────────────────────────────────────────
pub use lsp_types::{
    CompletionItem, CompletionItemKind, Diagnostic, DiagnosticSeverity, DocumentSymbol, Hover,
    InitializeResult, Location, MarkupContent, MarkupKind, ParameterInformation, Position,
    PublishDiagnosticsParams, Range, ServerCapabilities, SignatureHelp, SignatureInformation,
    SymbolInformation, SymbolKind, TextDocumentIdentifier, TextDocumentItem, TextEdit,
};

// ── Re-exports: Document management ──────────────────────────────────────────
pub use document::{Document, DocumentStore};

// ── Re-exports: Analysis engine ───────────────────────────────────────────────
pub use analysis::{
    analyze_document, find_references_in_document, format_document, get_keyword_hover,
    make_code_action, AnalysisCache, AnalysisResult, DefinitionInfo,
};

// ── Re-exports: LSP server ────────────────────────────────────────────────────
pub use lsp_server::{LspConfig, LspServer};

// ── Tests ─────────────────────────────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;
    use oxilean_kernel::Environment;

    // --- JSON parsing tests ---

    #[test]
    fn test_parse_json_null() {
        let (val, _) = parse_json_value("null").expect("parsing should succeed");
        assert_eq!(val, JsonValue::Null);
    }

    #[test]
    fn test_parse_json_bool() {
        let (val, _) = parse_json_value("true").expect("parsing should succeed");
        assert_eq!(val, JsonValue::Bool(true));
        let (val, _) = parse_json_value("false").expect("parsing should succeed");
        assert_eq!(val, JsonValue::Bool(false));
    }

    #[test]
    fn test_parse_json_number() {
        let (val, _) = parse_json_value("42").expect("parsing should succeed");
        assert_eq!(val, JsonValue::Number(42.0));
        let (val, _) = parse_json_value("-7").expect("parsing should succeed");
        assert_eq!(val, JsonValue::Number(-7.0));
        let (val, _) = parse_json_value("3.5").expect("parsing should succeed");
        assert_eq!(val, JsonValue::Number(3.5));
    }

    #[test]
    fn test_parse_json_string() {
        let (val, _) = parse_json_value("\"hello\"").expect("parsing should succeed");
        assert_eq!(val, JsonValue::String("hello".to_string()));
    }

    #[test]
    fn test_parse_json_string_escapes() {
        let (val, _) = parse_json_value("\"a\\nb\"").expect("parsing should succeed");
        assert_eq!(val, JsonValue::String("a\nb".to_string()));
        let (val, _) = parse_json_value("\"a\\\\b\"").expect("parsing should succeed");
        assert_eq!(val, JsonValue::String("a\\b".to_string()));
        let (val, _) = parse_json_value("\"a\\\"b\"").expect("parsing should succeed");
        assert_eq!(val, JsonValue::String("a\"b".to_string()));
    }

    #[test]
    fn test_parse_json_array() {
        let (val, _) = parse_json_value("[1, 2, 3]").expect("parsing should succeed");
        assert_eq!(
            val,
            JsonValue::Array(vec![
                JsonValue::Number(1.0),
                JsonValue::Number(2.0),
                JsonValue::Number(3.0),
            ])
        );
    }

    #[test]
    fn test_parse_json_empty_array() {
        let (val, _) = parse_json_value("[]").expect("parsing should succeed");
        assert_eq!(val, JsonValue::Array(Vec::new()));
    }

    #[test]
    fn test_parse_json_object() {
        let (val, _) =
            parse_json_value("{\"a\": 1, \"b\": \"two\"}").expect("parsing should succeed");
        assert_eq!(
            val,
            JsonValue::Object(vec![
                ("a".to_string(), JsonValue::Number(1.0)),
                ("b".to_string(), JsonValue::String("two".to_string())),
            ])
        );
    }

    #[test]
    fn test_parse_json_empty_object() {
        let (val, _) = parse_json_value("{}").expect("parsing should succeed");
        assert_eq!(val, JsonValue::Object(Vec::new()));
    }

    #[test]
    fn test_parse_json_nested() {
        let input = "{\"items\": [1, {\"nested\": true}]}";
        let (val, _) = parse_json_value(input).expect("parsing should succeed");
        let items = val
            .get("items")
            .expect("key should exist")
            .as_array()
            .expect("key should exist");
        assert_eq!(items.len(), 2);
        assert_eq!(
            items[1].get("nested").expect("key should exist").as_bool(),
            Some(true)
        );
    }

    // --- JSON serialization tests ---

    #[test]
    fn test_format_json_null() {
        assert_eq!(format_json_value(&JsonValue::Null), "null");
    }

    #[test]
    fn test_format_json_string_escape() {
        let val = JsonValue::String("a\"b\nc".to_string());
        let s = format_json_value(&val);
        assert_eq!(s, "\"a\\\"b\\nc\"");
    }

    #[test]
    fn test_format_json_roundtrip() {
        let original = JsonValue::Object(vec![
            ("id".to_string(), JsonValue::Number(1.0)),
            ("method".to_string(), JsonValue::String("test".to_string())),
            ("active".to_string(), JsonValue::Bool(true)),
            ("data".to_string(), JsonValue::Null),
        ]);
        let s = format_json_value(&original);
        let (parsed, _) = parse_json_value(&s).expect("parsing should succeed");
        assert_eq!(original, parsed);
    }

    // --- Document store tests ---

    #[test]
    fn test_document_store_open_close() {
        let mut store = DocumentStore::new();
        assert!(store.is_empty());
        store.open_document("file:///test.lean", 1, "def x := 1");
        assert_eq!(store.len(), 1);
        assert!(store.get_document("file:///test.lean").is_some());
        store.close_document("file:///test.lean");
        assert!(store.is_empty());
    }

    #[test]
    fn test_document_store_update() {
        let mut store = DocumentStore::new();
        store.open_document("file:///a.lean", 1, "original");
        assert!(store.update_document("file:///a.lean", 2, "updated"));
        let doc = store
            .get_document("file:///a.lean")
            .expect("test operation should succeed");
        assert_eq!(doc.content, "updated");
        assert_eq!(doc.version, 2);
    }

    #[test]
    fn test_document_store_update_nonexistent() {
        let mut store = DocumentStore::new();
        assert!(!store.update_document("file:///nope.lean", 1, "text"));
    }

    // --- Position/offset conversion tests ---

    #[test]
    fn test_position_to_offset() {
        let doc = Document::new("test", 1, "hello\nworld\nfoo");
        assert_eq!(doc.position_to_offset(&Position::new(0, 0)), Some(0));
        assert_eq!(doc.position_to_offset(&Position::new(0, 5)), Some(5));
        assert_eq!(doc.position_to_offset(&Position::new(1, 0)), Some(6));
        assert_eq!(doc.position_to_offset(&Position::new(1, 3)), Some(9));
        assert_eq!(doc.position_to_offset(&Position::new(2, 0)), Some(12));
    }

    #[test]
    fn test_offset_to_position() {
        let doc = Document::new("test", 1, "hello\nworld\nfoo");
        assert_eq!(doc.offset_to_position(0), Position::new(0, 0));
        assert_eq!(doc.offset_to_position(5), Position::new(0, 5));
        assert_eq!(doc.offset_to_position(6), Position::new(1, 0));
        assert_eq!(doc.offset_to_position(9), Position::new(1, 3));
        assert_eq!(doc.offset_to_position(12), Position::new(2, 0));
    }

    #[test]
    fn test_get_line() {
        let doc = Document::new("test", 1, "line one\nline two\nline three");
        assert_eq!(doc.get_line(0), Some("line one"));
        assert_eq!(doc.get_line(1), Some("line two"));
        assert_eq!(doc.get_line(2), Some("line three"));
        assert_eq!(doc.get_line(3), None);
    }

    #[test]
    fn test_line_count() {
        let doc = Document::new("test", 1, "a\nb\nc\n");
        assert_eq!(doc.line_count(), 4); // 4 lines: "a", "b", "c", ""
    }

    #[test]
    fn test_word_at_position() {
        let doc = Document::new("test", 1, "def hello_world : Nat := 42");
        let (word, _) = doc
            .word_at_position(&Position::new(0, 5))
            .expect("test operation should succeed");
        assert_eq!(word, "hello_world");
    }

    #[test]
    fn test_word_at_position_no_word() {
        let doc = Document::new("test", 1, "   ");
        assert!(doc.word_at_position(&Position::new(0, 1)).is_none());
    }

    // --- Server initialization test ---

    #[test]
    fn test_server_initialize() {
        let mut server = LspServer::new();
        assert!(!server.initialized);
        let result = server.handle_initialize(&JsonValue::Object(Vec::new()));
        assert!(server.initialized);
        // Check that capabilities are returned
        assert!(result.get("capabilities").is_some());
        assert!(result.get("serverInfo").is_some());
    }

    #[test]
    fn test_server_shutdown() {
        let mut server = LspServer::new();
        server.initialized = true;
        let result = server.handle_shutdown();
        assert!(server.shutdown_requested);
        assert_eq!(result, JsonValue::Null);
    }

    // --- Message dispatch tests ---

    #[test]
    fn test_handle_message_initialize() {
        let mut server = LspServer::new();
        let msg = JsonRpcMessage::request(
            JsonValue::Number(1.0),
            "initialize",
            JsonValue::Object(Vec::new()),
        );
        let resp = server
            .handle_message(&msg)
            .expect("test operation should succeed");
        assert!(resp.result.is_some());
        assert!(resp.error.is_none());
    }

    #[test]
    fn test_handle_message_unknown_method() {
        let mut server = LspServer::new();
        let msg =
            JsonRpcMessage::request(JsonValue::Number(1.0), "unknown/method", JsonValue::Null);
        let resp = server
            .handle_message(&msg)
            .expect("test operation should succeed");
        assert!(resp.error.is_some());
        assert_eq!(
            resp.error.expect("test operation should succeed").code,
            METHOD_NOT_FOUND
        );
    }

    #[test]
    fn test_handle_did_open_and_completion() {
        let mut server = LspServer::new();
        server.initialized = true;

        // Open a document
        let open_params = JsonValue::Object(vec![(
            "textDocument".to_string(),
            JsonValue::Object(vec![
                (
                    "uri".to_string(),
                    JsonValue::String("file:///test.lean".to_string()),
                ),
                (
                    "languageId".to_string(),
                    JsonValue::String("lean4".to_string()),
                ),
                ("version".to_string(), JsonValue::Number(1.0)),
                (
                    "text".to_string(),
                    JsonValue::String("def foo := 1\n".to_string()),
                ),
            ]),
        )]);
        server.handle_text_document_did_open(&open_params);
        assert!(server
            .document_store
            .get_document("file:///test.lean")
            .is_some());

        // Request completions
        let comp_params = JsonValue::Object(vec![
            (
                "textDocument".to_string(),
                JsonValue::Object(vec![(
                    "uri".to_string(),
                    JsonValue::String("file:///test.lean".to_string()),
                )]),
            ),
            (
                "position".to_string(),
                JsonValue::Object(vec![
                    ("line".to_string(), JsonValue::Number(1.0)),
                    ("character".to_string(), JsonValue::Number(0.0)),
                ]),
            ),
        ]);
        let result = server.handle_completion(&comp_params);
        // Should have keyword completions at minimum
        assert!(
            result
                .as_array()
                .expect("type conversion should succeed")
                .len()
                > 5
        );
    }

    // --- Completion tests ---

    #[test]
    fn test_completions_keywords() {
        let server = LspServer::new();
        let completions = server.get_completions_at("file:///test.lean", &Position::new(0, 0));
        let labels: Vec<&str> = completions.iter().map(|c| c.label.as_str()).collect();
        assert!(labels.contains(&"def"));
        assert!(labels.contains(&"theorem"));
        assert!(labels.contains(&"axiom"));
    }

    #[test]
    fn test_completions_prefix_filter() {
        let mut server = LspServer::new();
        server
            .document_store
            .open_document("file:///t.lean", 1, "th");
        let completions = server.get_completions_at("file:///t.lean", &Position::new(0, 2));
        let labels: Vec<&str> = completions.iter().map(|c| c.label.as_str()).collect();
        assert!(labels.contains(&"theorem"));
        assert!(labels.contains(&"then"));
        assert!(!labels.contains(&"def"));
    }

    // --- Diagnostic tests ---

    #[test]
    fn test_validate_empty_document() {
        let mut server = LspServer::new();
        server
            .document_store
            .open_document("file:///empty.lean", 1, "");
        let diags = server.validate_document("file:///empty.lean");
        assert!(diags.is_empty());
    }

    #[test]
    fn test_validate_valid_tokens() {
        let mut server = LspServer::new();
        server
            .document_store
            .open_document("file:///good.lean", 1, "def foo : Nat := 1");
        let diags = server.validate_document("file:///good.lean");
        // Simple valid tokens should produce no diagnostics
        assert!(diags.is_empty());
    }

    // --- Symbol extraction tests ---

    #[test]
    fn test_extract_symbols() {
        let content = "def foo := 1\ntheorem bar : Prop := True\naxiom baz : Nat";
        let env = Environment::new();
        let result = analyze_document("test", content, &env);
        let names: Vec<&str> = result.symbols.iter().map(|s| s.name.as_str()).collect();
        assert!(names.contains(&"foo"));
        assert!(names.contains(&"bar"));
        assert!(names.contains(&"baz"));
    }

    #[test]
    fn test_extract_definitions() {
        let content = "def hello := 42\ninductive MyType where\n  | ctor";
        let env = Environment::new();
        let result = analyze_document("test", content, &env);
        assert!(result.definitions.len() >= 2);
        let def_names: Vec<&str> = result.definitions.iter().map(|d| d.name.as_str()).collect();
        assert!(def_names.contains(&"hello"));
        assert!(def_names.contains(&"MyType"));
    }

    // --- Hover tests ---

    #[test]
    fn test_hover_keyword() {
        let info = get_keyword_hover("def");
        assert!(info.is_some());
        assert!(info.expect("test operation should succeed").contains("def"));
    }

    #[test]
    fn test_hover_unknown() {
        let info = get_keyword_hover("not_a_keyword_xyz");
        assert!(info.is_none());
    }

    #[test]
    fn test_hover_in_document() {
        let mut server = LspServer::new();
        server
            .document_store
            .open_document("file:///h.lean", 1, "def myFunc := 1");
        let hover = server.get_hover_info_at("file:///h.lean", &Position::new(0, 0));
        // "def" is a keyword
        assert!(hover.is_some());
    }

    // --- Find references test ---

    #[test]
    fn test_find_references() {
        let content = "def foo := 1\ndef bar := foo";
        let refs = find_references_in_document("test", content, "foo");
        assert_eq!(refs.len(), 2); // declaration name + usage
    }

    // --- Document formatting test ---

    #[test]
    fn test_format_trailing_whitespace() {
        let edits = format_document("def x := 1   \ndef y := 2  ");
        assert!(!edits.is_empty());
        // Should have edits for trailing whitespace + missing newline
    }

    // --- JsonRpcMessage tests ---

    #[test]
    fn test_jsonrpc_message_roundtrip() {
        let msg = JsonRpcMessage::request(
            JsonValue::Number(42.0),
            "textDocument/completion",
            JsonValue::Object(Vec::new()),
        );
        let json = msg.to_json();
        let parsed = JsonRpcMessage::from_json(&json).expect("parsing should succeed");
        assert_eq!(
            parsed.id.expect("parsing should succeed"),
            JsonValue::Number(42.0)
        );
        assert_eq!(
            parsed.method.expect("parsing should succeed"),
            "textDocument/completion"
        );
    }

    #[test]
    fn test_jsonrpc_error_roundtrip() {
        let err = JsonRpcError::method_not_found("foo/bar");
        let json = err.to_json();
        let parsed = JsonRpcError::from_json(&json).expect("parsing should succeed");
        assert_eq!(parsed.code, METHOD_NOT_FOUND);
        assert!(parsed.message.contains("foo/bar"));
    }

    // --- LSP type serialization tests ---

    #[test]
    fn test_position_json_roundtrip() {
        let pos = Position::new(10, 25);
        let json = pos.to_json();
        let parsed = Position::from_json(&json).expect("parsing should succeed");
        assert_eq!(parsed, pos);
    }

    #[test]
    fn test_range_json_roundtrip() {
        let range = Range::new(Position::new(1, 0), Position::new(1, 10));
        let json = range.to_json();
        let parsed = Range::from_json(&json).expect("parsing should succeed");
        assert_eq!(parsed, range);
    }

    #[test]
    fn test_diagnostic_to_json() {
        let diag = Diagnostic::error(Range::single_line(0, 0, 5), "test error");
        let json = diag.to_json();
        assert_eq!(
            json.get("message").expect("key should exist").as_str(),
            Some("test error")
        );
        assert_eq!(
            json.get("severity").expect("key should exist").as_i64(),
            Some(1)
        );
    }

    #[test]
    fn test_server_capabilities_json() {
        let caps = ServerCapabilities::oxilean_defaults();
        let json = caps.to_json();
        assert!(json.get("hoverProvider").is_some());
        assert!(json.get("definitionProvider").is_some());
    }

    /// Verify that the capabilities advertised on initialize include
    /// `semanticTokensProvider` with both `full` and `range` set to `true`.
    #[test]
    fn test_semantic_tokens_provider_advertised() {
        let caps = ServerCapabilities::oxilean_defaults();
        let json = caps.to_json();

        let provider = json
            .get("semanticTokensProvider")
            .expect("semanticTokensProvider must be advertised");

        let full = provider
            .get("full")
            .and_then(|v| v.as_bool())
            .expect("semanticTokensProvider.full must be a bool");
        assert!(full, "semanticTokensProvider.full must be true");

        let range = provider
            .get("range")
            .and_then(|v| v.as_bool())
            .expect("semanticTokensProvider.range must be a bool");
        assert!(range, "semanticTokensProvider.range must be true");

        let legend = provider
            .get("legend")
            .expect("semanticTokensProvider.legend must exist");
        assert!(
            legend.get("tokenTypes").is_some(),
            "legend must have tokenTypes"
        );
        assert!(
            legend.get("tokenModifiers").is_some(),
            "legend must have tokenModifiers"
        );
    }

    /// Verify that the text document sync mode advertises incremental (2).
    #[test]
    fn test_text_document_sync_is_incremental() {
        let caps = ServerCapabilities::oxilean_defaults();
        let json = caps.to_json();
        let sync = json
            .get("textDocumentSync")
            .and_then(|v| v.as_i64())
            .expect("textDocumentSync must be a number");
        assert_eq!(sync, 2, "textDocumentSync must be 2 (Incremental)");
    }

    #[test]
    fn test_initialize_result_json() {
        let result = InitializeResult {
            capabilities: ServerCapabilities::oxilean_defaults(),
            server_name: "test".to_string(),
            server_version: "1.0.0".to_string(),
        };
        let json = result.to_json();
        let info = json.get("serverInfo").expect("key should exist");
        assert_eq!(
            info.get("name").expect("key should exist").as_str(),
            Some("test")
        );
    }

    // --- Document symbol test ---

    #[test]
    fn test_document_symbol_json() {
        let sym = DocumentSymbol {
            name: "foo".to_string(),
            detail: Some("def foo".to_string()),
            kind: SymbolKind::Function,
            range: Range::single_line(0, 0, 10),
            selection_range: Range::single_line(0, 4, 7),
            children: Vec::new(),
        };
        let json = sym.to_json();
        assert_eq!(
            json.get("name").expect("key should exist").as_str(),
            Some("foo")
        );
    }

    // --- Integration-style tests ---

    #[test]
    fn test_full_lsp_session() {
        let mut server = LspServer::new();

        // Initialize
        let init_msg = JsonRpcMessage::request(
            JsonValue::Number(1.0),
            "initialize",
            JsonValue::Object(Vec::new()),
        );
        let resp = server
            .handle_message(&init_msg)
            .expect("test operation should succeed");
        assert!(resp.result.is_some());

        // Open document
        let open_params = JsonValue::Object(vec![(
            "textDocument".to_string(),
            JsonValue::Object(vec![
                (
                    "uri".to_string(),
                    JsonValue::String("file:///session.lean".to_string()),
                ),
                (
                    "text".to_string(),
                    JsonValue::String("def add (a b : Nat) : Nat := a".to_string()),
                ),
                ("version".to_string(), JsonValue::Number(1.0)),
            ]),
        )]);
        let open_msg = JsonRpcMessage::notification("textDocument/didOpen", open_params);
        assert!(server.handle_message(&open_msg).is_none());

        // Document symbol
        let sym_params = JsonValue::Object(vec![(
            "textDocument".to_string(),
            JsonValue::Object(vec![(
                "uri".to_string(),
                JsonValue::String("file:///session.lean".to_string()),
            )]),
        )]);
        let sym_msg = JsonRpcMessage::request(
            JsonValue::Number(2.0),
            "textDocument/documentSymbol",
            sym_params,
        );
        let sym_resp = server
            .handle_message(&sym_msg)
            .expect("test operation should succeed");
        let symbols = sym_resp.result.expect("test operation should succeed");
        assert!(!symbols
            .as_array()
            .expect("type conversion should succeed")
            .is_empty());

        // Shutdown
        let shutdown_msg =
            JsonRpcMessage::request(JsonValue::Number(3.0), "shutdown", JsonValue::Null);
        let shutdown_resp = server
            .handle_message(&shutdown_msg)
            .expect("test operation should succeed");
        assert_eq!(shutdown_resp.result, Some(JsonValue::Null));
        assert!(server.shutdown_requested);
    }

    // ── In-process JSON-RPC integration tests ─────────────────────────────────

    /// Build a framed LSP message from a JSON string.
    fn make_lsp_frame(json: &str) -> Vec<u8> {
        let mut buf = Vec::new();
        let header = format!("Content-Length: {}\r\n\r\n", json.len());
        buf.extend_from_slice(header.as_bytes());
        buf.extend_from_slice(json.as_bytes());
        buf
    }

    /// Collect all JSON objects from the framed output buffer.
    fn collect_output_bodies(output: &[u8]) -> Vec<String> {
        let mut bodies = Vec::new();
        let mut pos = 0;
        while pos < output.len() {
            // Find "Content-Length: " header
            let remaining = &output[pos..];
            let header_start = match remaining.windows(16).position(|w| w == b"Content-Length: ") {
                Some(p) => p,
                None => break,
            };
            let after_prefix = pos + header_start + 16;
            // Find end of header line (\r\n\r\n)
            let header_slice = &output[after_prefix..];
            let crlf_crlf = b"\r\n\r\n";
            let sep_pos = match header_slice.windows(4).position(|w| w == crlf_crlf) {
                Some(p) => p,
                None => break,
            };
            let len_str = std::str::from_utf8(&header_slice[..sep_pos])
                .unwrap_or("")
                .trim();
            let content_len: usize = match len_str.parse() {
                Ok(n) => n,
                Err(_) => break,
            };
            let body_start = after_prefix + sep_pos + 4;
            let body_end = body_start + content_len;
            if body_end > output.len() {
                break;
            }
            if let Ok(s) = std::str::from_utf8(&output[body_start..body_end]) {
                bodies.push(s.to_string());
            }
            pos = body_end;
        }
        bodies
    }

    /// Verify that a JSON body string contains a top-level key with the given value substring.
    fn body_has_method(body: &str, method: &str) -> bool {
        body.contains(&format!("\"method\":\"{}\"", method))
            || body.contains(&format!("\"method\": \"{}\"", method))
    }

    #[test]
    fn test_lsp_integration_initialize_roundtrip() {
        // Prepare: a single "initialize" request followed by EOF
        let init_json = r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}"#;
        let input_bytes = make_lsp_frame(init_json);

        let mut reader = std::io::BufReader::new(std::io::Cursor::new(input_bytes));
        let mut output: Vec<u8> = Vec::new();

        super::server::run_server_with_io(&mut reader, &mut output)
            .expect("server should run without error");

        let bodies = collect_output_bodies(&output);
        assert!(
            !bodies.is_empty(),
            "expected at least one response in output"
        );
        // The initialize response must contain "capabilities"
        let init_resp = &bodies[0];
        assert!(
            init_resp.contains("\"capabilities\""),
            "initialize response should contain capabilities, got: {}",
            init_resp
        );
        assert!(
            init_resp.contains("\"id\""),
            "initialize response should have an id, got: {}",
            init_resp
        );
    }

    #[test]
    fn test_lsp_integration_did_open_no_error_produces_empty_diagnostics() {
        // Source with no lexer errors: clean definition
        let source = "def foo : Nat := 42";
        let did_open_params = format!(
            r#"{{"textDocument":{{"uri":"file:///test_clean.lean","languageId":"lean4","version":1,"text":{}}}}}"#,
            serde_json_string(source)
        );
        let init_json = r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}"#;
        let initialized_json = r#"{"jsonrpc":"2.0","method":"initialized","params":{}}"#;
        let did_open_json = format!(
            r#"{{"jsonrpc":"2.0","method":"textDocument/didOpen","params":{}}}"#,
            did_open_params
        );

        let mut input_bytes = Vec::new();
        input_bytes.extend(make_lsp_frame(init_json));
        input_bytes.extend(make_lsp_frame(initialized_json));
        input_bytes.extend(make_lsp_frame(&did_open_json));

        let mut reader = std::io::BufReader::new(std::io::Cursor::new(input_bytes));
        let mut output: Vec<u8> = Vec::new();

        super::server::run_server_with_io(&mut reader, &mut output)
            .expect("server should run without error");

        let bodies = collect_output_bodies(&output);
        // Find the publishDiagnostics notification
        let pub_diag: Vec<&String> = bodies
            .iter()
            .filter(|b| body_has_method(b, "textDocument/publishDiagnostics"))
            .collect();
        assert!(
            !pub_diag.is_empty(),
            "expected a publishDiagnostics notification, got bodies: {:?}",
            bodies
        );
        // For clean source, diagnostics array should be empty
        let diag_body = pub_diag[0];
        assert!(
            diag_body.contains("\"diagnostics\":[]"),
            "expected empty diagnostics for clean source, got: {}",
            diag_body
        );
    }

    #[test]
    fn test_lsp_integration_did_open_lexer_error_produces_diagnostic() {
        // Source with a guaranteed lexer error: "0x " (hex literal with no digits)
        let source = "0x ";
        let did_open_params = format!(
            r#"{{"textDocument":{{"uri":"file:///test_error.lean","languageId":"lean4","version":1,"text":{}}}}}"#,
            serde_json_string(source)
        );
        let init_json = r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}"#;
        let initialized_json = r#"{"jsonrpc":"2.0","method":"initialized","params":{}}"#;
        let did_open_json = format!(
            r#"{{"jsonrpc":"2.0","method":"textDocument/didOpen","params":{}}}"#,
            did_open_params
        );

        let mut input_bytes = Vec::new();
        input_bytes.extend(make_lsp_frame(init_json));
        input_bytes.extend(make_lsp_frame(initialized_json));
        input_bytes.extend(make_lsp_frame(&did_open_json));

        let mut reader = std::io::BufReader::new(std::io::Cursor::new(input_bytes));
        let mut output: Vec<u8> = Vec::new();

        super::server::run_server_with_io(&mut reader, &mut output)
            .expect("server should run without error");

        let bodies = collect_output_bodies(&output);
        let pub_diag: Vec<&String> = bodies
            .iter()
            .filter(|b| body_has_method(b, "textDocument/publishDiagnostics"))
            .collect();
        assert!(
            !pub_diag.is_empty(),
            "expected a publishDiagnostics notification, got bodies: {:?}",
            bodies
        );
        // For error source, diagnostics should be non-empty
        let diag_body = pub_diag[0];
        assert!(
            !diag_body.contains("\"diagnostics\":[]"),
            "expected non-empty diagnostics for error source, got: {}",
            diag_body
        );
        assert!(
            diag_body.contains("\"message\""),
            "expected diagnostic message field, got: {}",
            diag_body
        );
    }

    #[test]
    fn test_lsp_integration_pipeline_core_clean_source() {
        // Direct pipeline-core test: analyze_document on clean source
        use oxilean_kernel::Environment;
        let env = Environment::new();
        let result = analyze_document("file:///clean.lean", "def foo : Nat := 42", &env);
        assert!(
            result.diagnostics.is_empty(),
            "clean source should produce no diagnostics, got: {:?}",
            result.diagnostics
        );
    }

    #[test]
    fn test_lsp_integration_pipeline_core_lexer_error() {
        // Direct pipeline-core test: analyze_document on source with lexer error
        use oxilean_kernel::Environment;
        let env = Environment::new();
        // "0x " produces TokenKind::Error (hex literal with no digits)
        let result = analyze_document("file:///error.lean", "0x ", &env);
        assert!(
            !result.diagnostics.is_empty(),
            "source with lexer error should produce >= 1 diagnostic"
        );
        assert!(
            result.diagnostics[0].message.contains("lexer error"),
            "diagnostic should mention 'lexer error', got: {:?}",
            result.diagnostics[0].message
        );
    }

    #[test]
    fn test_lsp_integration_shutdown_sequence() {
        // Full sequence: initialize → initialized → shutdown → exit
        let init_json = r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}"#;
        let initialized_json = r#"{"jsonrpc":"2.0","method":"initialized","params":{}}"#;
        let shutdown_json = r#"{"jsonrpc":"2.0","id":2,"method":"shutdown","params":null}"#;
        let exit_json = r#"{"jsonrpc":"2.0","method":"exit","params":null}"#;

        let mut input_bytes = Vec::new();
        input_bytes.extend(make_lsp_frame(init_json));
        input_bytes.extend(make_lsp_frame(initialized_json));
        input_bytes.extend(make_lsp_frame(shutdown_json));
        input_bytes.extend(make_lsp_frame(exit_json));

        let mut reader = std::io::BufReader::new(std::io::Cursor::new(input_bytes));
        let mut output: Vec<u8> = Vec::new();

        super::server::run_server_with_io(&mut reader, &mut output)
            .expect("server should run without error");

        let bodies = collect_output_bodies(&output);
        // Must have initialize response (id=1) and shutdown response (id=2)
        assert!(
            bodies.len() >= 2,
            "expected at least 2 responses, got: {:?}",
            bodies
        );
        let shutdown_resp = bodies
            .iter()
            .find(|b| b.contains("\"id\":2") || b.contains("\"id\": 2"))
            .expect("shutdown response with id=2 should be present");
        assert!(
            shutdown_resp.contains("\"result\":null") || shutdown_resp.contains("\"result\": null"),
            "shutdown response result should be null, got: {}",
            shutdown_resp
        );
    }

    /// Minimal JSON string-escape for test fixtures (only escapes what JSON requires).
    fn serde_json_string(s: &str) -> String {
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
}
