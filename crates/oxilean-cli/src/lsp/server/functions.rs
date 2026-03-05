//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lsp::{
    analyze_document, format_json_value, parse_json_value, Diagnostic, Document, InitializeResult,
    JsonRpcError, JsonRpcMessage, JsonValue, Location, LspConfig, LspServer, Position,
    PublishDiagnosticsParams, Range, ServerCapabilities, SymbolInformation, SymbolKind,
};
use oxilean_kernel::{Environment, Name};
use oxilean_parse::{Lexer, TokenKind};
use std::io::{self, BufRead, Write as IoWrite};

use super::types::{
    CapabilityNegotiator, ClientCapabilities, DependencyGraph, DiagnosticThrottle, DispatchResult,
    HealthStatus, LspSession, MessageBatch, MessageChannel, RequestDispatcher, RequestTracker,
    SemanticTokenType, ServerConnectionInfo, ServerHealthChecker, ServerMetrics,
    ServerRequestQueue, ServerShutdownHandler, ServerState, ServerTransport, ServerWorkspace,
    TextDocumentEdit, WorkspaceFolder,
};

/// Helper to wrap a result in a DispatchResult::Response.
pub fn respond(id: Option<JsonValue>, result: JsonValue) -> DispatchResult {
    match id {
        Some(id_val) => DispatchResult::Response(JsonRpcMessage::response(id_val, result)),
        None => DispatchResult::Handled,
    }
}
/// Read a single LSP message from a reader using Content-Length framing.
pub fn read_lsp_message<R: BufRead>(reader: &mut R) -> Result<String, io::Error> {
    let mut content_length: Option<usize> = None;
    loop {
        let mut header_line = String::new();
        let bytes_read = reader.read_line(&mut header_line)?;
        if bytes_read == 0 {
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "connection closed",
            ));
        }
        let trimmed = header_line.trim();
        if trimmed.is_empty() {
            break;
        }
        if let Some(len_str) = trimmed.strip_prefix("Content-Length: ") {
            content_length = len_str.parse().ok();
        }
    }
    let length = content_length.ok_or_else(|| {
        io::Error::new(io::ErrorKind::InvalidData, "missing Content-Length header")
    })?;
    let mut body = vec![0u8; length];
    reader.read_exact(&mut body)?;
    String::from_utf8(body).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}
/// Write a single LSP message to a writer using Content-Length framing.
pub fn write_lsp_message<W: IoWrite>(writer: &mut W, content: &str) -> Result<(), io::Error> {
    let header = format!("Content-Length: {}\r\n\r\n", content.len());
    writer.write_all(header.as_bytes())?;
    writer.write_all(content.as_bytes())?;
    writer.flush()
}
/// Parse a raw JSON string into a JsonRpcMessage.
pub fn parse_message(raw: &str) -> Result<JsonRpcMessage, String> {
    let (val, _) = parse_json_value(raw)?;
    JsonRpcMessage::from_json(&val)
}
/// Serialize a JsonRpcMessage to a JSON string.
pub fn serialize_message(msg: &JsonRpcMessage) -> String {
    format_json_value(&msg.to_json())
}
/// Compute the encoded semantic token data for a document.
pub fn compute_semantic_tokens_data(content: &str, env: &Environment) -> Vec<u32> {
    let mut data = Vec::new();
    let mut lexer = Lexer::new(content);
    let tokens = lexer.tokenize();
    let mut prev_line: u32 = 0;
    let mut prev_col: u32 = 0;
    for token in &tokens {
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
        let length = (token.span.end - token.span.start) as u32;
        if length == 0 {
            continue;
        }
        let token_type = classify_token(&token.kind, env);
        let token_type_id = token_type as u32;
        let delta_line = line - prev_line;
        let delta_start = if delta_line == 0 {
            col.saturating_sub(prev_col)
        } else {
            col
        };
        data.push(delta_line);
        data.push(delta_start);
        data.push(length);
        data.push(token_type_id);
        data.push(0);
        prev_line = line;
        prev_col = col;
    }
    data
}
/// Classify a token kind into a semantic token type.
fn classify_token(kind: &TokenKind, env: &Environment) -> SemanticTokenType {
    match kind {
        TokenKind::Definition
        | TokenKind::Theorem
        | TokenKind::Lemma
        | TokenKind::Axiom
        | TokenKind::Inductive
        | TokenKind::Structure
        | TokenKind::Class
        | TokenKind::Instance
        | TokenKind::Where
        | TokenKind::Let
        | TokenKind::In
        | TokenKind::Fun
        | TokenKind::Forall
        | TokenKind::Match
        | TokenKind::With
        | TokenKind::If
        | TokenKind::Then
        | TokenKind::Else
        | TokenKind::Do
        | TokenKind::By
        | TokenKind::Import
        | TokenKind::Open
        | TokenKind::Namespace
        | TokenKind::End
        | TokenKind::Section
        | TokenKind::Variable
        | TokenKind::Have
        | TokenKind::Show
        | TokenKind::Return => SemanticTokenType::Keyword,
        TokenKind::Ident(name) => {
            let kernel_name = Name::str(name);
            if env.is_inductive(&kernel_name) {
                SemanticTokenType::Type
            } else if env.is_constructor(&kernel_name) {
                SemanticTokenType::Enum
            } else if env.find(&kernel_name).is_some() {
                SemanticTokenType::Function
            } else if name.starts_with(|c: char| c.is_uppercase()) {
                SemanticTokenType::Type
            } else {
                SemanticTokenType::Variable
            }
        }
        TokenKind::Nat(_) => SemanticTokenType::Number,
        TokenKind::String(_) => SemanticTokenType::StringLit,
        TokenKind::DocComment(_) => SemanticTokenType::Comment,
        TokenKind::Plus
        | TokenKind::Minus
        | TokenKind::Star
        | TokenKind::Slash
        | TokenKind::Arrow
        | TokenKind::FatArrow
        | TokenKind::Colon
        | TokenKind::Assign
        | TokenKind::Eq
        | TokenKind::Ne
        | TokenKind::Lt
        | TokenKind::Le
        | TokenKind::Gt
        | TokenKind::Ge
        | TokenKind::And
        | TokenKind::Or
        | TokenKind::Not
        | TokenKind::Bar
        | TokenKind::AndAnd
        | TokenKind::LeftArrow => SemanticTokenType::Operator,
        _ => SemanticTokenType::Variable,
    }
}
/// Compute inlay hints for a document.
pub fn compute_inlay_hints(doc: &Document, env: &Environment) -> Vec<JsonValue> {
    let mut hints = Vec::new();
    let mut lexer = Lexer::new(&doc.content);
    let tokens = lexer.tokenize();
    let mut i = 0;
    while i < tokens.len() {
        if matches!(tokens[i].kind, TokenKind::Definition | TokenKind::Let) && i + 2 < tokens.len()
        {
            if let TokenKind::Ident(name) = &tokens[i + 1].kind {
                let mut has_colon = false;
                let mut j = i + 2;
                while j < tokens.len() {
                    match &tokens[j].kind {
                        TokenKind::Colon => {
                            has_colon = true;
                            break;
                        }
                        TokenKind::Assign => break,
                        _ => {}
                    }
                    j += 1;
                }
                if !has_colon {
                    let kernel_name = Name::str(name);
                    if let Some(ci) = env.find(&kernel_name) {
                        let ty_str = format!("{:?}", ci.ty());
                        let name_token = &tokens[i + 1];
                        let line = if name_token.span.line > 0 {
                            name_token.span.line as u32 - 1
                        } else {
                            0
                        };
                        let col_end = if name_token.span.column > 0 {
                            name_token.span.column as u32 - 1
                                + (name_token.span.end - name_token.span.start) as u32
                        } else {
                            (name_token.span.end - name_token.span.start) as u32
                        };
                        hints.push(make_inlay_hint(line, col_end, &format!(" : {}", ty_str), 1));
                    }
                }
            }
        }
        i += 1;
    }
    hints
}
/// Create a single inlay hint JSON value.
fn make_inlay_hint(line: u32, character: u32, label: &str, kind: u32) -> JsonValue {
    JsonValue::Object(vec![
        (
            "position".to_string(),
            Position::new(line, character).to_json(),
        ),
        ("label".to_string(), JsonValue::String(label.to_string())),
        ("kind".to_string(), JsonValue::Number(kind as f64)),
        ("paddingLeft".to_string(), JsonValue::Bool(false)),
        ("paddingRight".to_string(), JsonValue::Bool(false)),
    ])
}
/// Compute the goal state at a position in a proof.
pub fn compute_goal_state(doc: &Document, pos: &Position, env: &Environment) -> JsonValue {
    let analysis = analyze_document(&doc.uri, &doc.content, env);
    let mut in_proof = false;
    let mut line_idx = pos.line as usize;
    while line_idx > 0 {
        if let Some(line) = doc.get_line(line_idx as u32) {
            let trimmed = line.trim();
            if trimmed == "by" || trimmed.starts_with("by ") {
                in_proof = true;
                break;
            }
            if ["def", "theorem", "lemma", "axiom", "inductive"]
                .iter()
                .any(|kw| trimmed.starts_with(kw))
            {
                break;
            }
        }
        line_idx -= 1;
    }
    if !in_proof {
        return JsonValue::Null;
    }
    let mut theorem_name = String::new();
    let mut theorem_type = String::new();
    for def in &analysis.definitions {
        if def.range.start.line <= pos.line {
            theorem_name = def.name.clone();
            if let Some(ref ty) = def.ty {
                theorem_type = ty.clone();
            } else {
                let kn = Name::str(&def.name);
                if let Some(ci) = env.find(&kn) {
                    theorem_type = format!("{:?}", ci.ty());
                }
            }
        }
    }
    JsonValue::Object(vec![
        (
            "goals".to_string(),
            JsonValue::Array(vec![JsonValue::Object(vec![
                ("hyps".to_string(), JsonValue::Array(Vec::new())),
                (
                    "type".to_string(),
                    JsonValue::String(if theorem_type.is_empty() {
                        "?goal".to_string()
                    } else {
                        theorem_type
                    }),
                ),
            ])]),
        ),
        ("theoremName".to_string(), JsonValue::String(theorem_name)),
    ])
}
/// Compute a proof tree for a document.
pub fn compute_proof_tree(doc: &Document, env: &Environment) -> JsonValue {
    let analysis = analyze_document(&doc.uri, &doc.content, env);
    let mut nodes = Vec::new();
    for def in &analysis.definitions {
        if def.kind == SymbolKind::Method {
            let ty = def.ty.as_deref().unwrap_or("?").to_string();
            nodes.push(JsonValue::Object(vec![
                ("name".to_string(), JsonValue::String(def.name.clone())),
                ("type".to_string(), JsonValue::String(ty)),
                (
                    "status".to_string(),
                    JsonValue::String("proven".to_string()),
                ),
                ("children".to_string(), JsonValue::Array(Vec::new())),
                ("range".to_string(), def.range.to_json()),
            ]));
        }
    }
    JsonValue::Object(vec![
        ("nodes".to_string(), JsonValue::Array(nodes)),
        ("uri".to_string(), JsonValue::String(doc.uri.clone())),
    ])
}
/// Run the LSP server main loop reading from stdin and writing to stdout.
pub fn run_server_stdio() -> Result<(), String> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut reader = stdin.lock();
    let mut writer = stdout.lock();
    let mut session = LspSession::new();
    loop {
        let raw = match read_lsp_message(&mut reader) {
            Ok(s) => s,
            Err(e) => {
                if e.kind() == io::ErrorKind::UnexpectedEof {
                    break;
                }
                return Err(format!("I/O error: {}", e));
            }
        };
        let msg = match parse_message(&raw) {
            Ok(m) => m,
            Err(e) => {
                let err_response =
                    JsonRpcMessage::error_response(JsonValue::Null, JsonRpcError::parse_error(e));
                let out = serialize_message(&err_response);
                let _ = write_lsp_message(&mut writer, &out);
                continue;
            }
        };
        let mut dispatcher = RequestDispatcher::new(&mut session);
        let result = dispatcher.dispatch(&msg);
        match result {
            DispatchResult::Response(response) => {
                let out = serialize_message(&response);
                let _ = write_lsp_message(&mut writer, &out);
            }
            DispatchResult::Handled => {}
            DispatchResult::Exit => break,
            DispatchResult::Error(e) => {
                return Err(e);
            }
        }
        let notifications = session.drain_notifications();
        for notif in notifications {
            let out = serialize_message(&notif);
            let _ = write_lsp_message(&mut writer, &out);
        }
    }
    Ok(())
}
/// Run the LSP server with custom reader and writer (for testing).
pub fn run_server_with_io<R: BufRead, W: IoWrite>(
    reader: &mut R,
    writer: &mut W,
) -> Result<(), String> {
    let mut session = LspSession::new();
    loop {
        let raw = match read_lsp_message(reader) {
            Ok(s) => s,
            Err(e) => {
                if e.kind() == io::ErrorKind::UnexpectedEof {
                    break;
                }
                return Err(format!("I/O error: {}", e));
            }
        };
        let msg = match parse_message(&raw) {
            Ok(m) => m,
            Err(e) => {
                let err_response =
                    JsonRpcMessage::error_response(JsonValue::Null, JsonRpcError::parse_error(e));
                let out = serialize_message(&err_response);
                let _ = write_lsp_message(writer, &out);
                continue;
            }
        };
        let mut dispatcher = RequestDispatcher::new(&mut session);
        let result = dispatcher.dispatch(&msg);
        match result {
            DispatchResult::Response(response) => {
                let out = serialize_message(&response);
                let _ = write_lsp_message(writer, &out);
            }
            DispatchResult::Handled => {}
            DispatchResult::Exit => break,
            DispatchResult::Error(e) => {
                return Err(e);
            }
        }
        let notifications = session.drain_notifications();
        for notif in notifications {
            let out = serialize_message(&notif);
            let _ = write_lsp_message(writer, &out);
        }
    }
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_server_state_transitions() {
        let session = LspSession::new();
        assert_eq!(session.state, ServerState::Uninitialized);
        assert!(!session.can_handle_requests());
    }
    #[test]
    fn test_workspace_folder_json() {
        let folder = WorkspaceFolder {
            uri: "file:///project".to_string(),
            name: "my-project".to_string(),
        };
        let json = folder.to_json();
        let parsed = WorkspaceFolder::from_json(&json).expect("parsing should succeed");
        assert_eq!(parsed.uri, folder.uri);
        assert_eq!(parsed.name, folder.name);
    }
    #[test]
    fn test_dependency_graph() {
        let mut graph = DependencyGraph::new();
        graph.add_dependency("a.lean", "b.lean");
        graph.add_dependency("a.lean", "c.lean");
        graph.add_dependency("b.lean", "c.lean");
        assert_eq!(graph.get_dependencies("a.lean").len(), 2);
        let deps_of_c = graph.get_dependents("c.lean");
        assert!(deps_of_c.len() >= 2);
        let transitive = graph.get_transitive_dependents("c.lean");
        assert!(transitive.contains(&"a.lean".to_string()));
        assert!(transitive.contains(&"b.lean".to_string()));
    }
    #[test]
    fn test_dependency_graph_remove() {
        let mut graph = DependencyGraph::new();
        graph.add_dependency("a.lean", "b.lean");
        graph.remove_file("a.lean");
        assert!(graph.get_dependencies("a.lean").is_empty());
        assert!(graph.get_dependents("b.lean").is_empty());
    }
    #[test]
    fn test_message_batch() {
        let mut batch = MessageBatch::new();
        assert!(batch.is_empty());
        batch.push(JsonRpcMessage::notification("test", JsonValue::Null));
        assert_eq!(batch.len(), 1);
        assert!(!batch.is_empty());
    }
    #[test]
    fn test_request_tracker() {
        let mut tracker = RequestTracker::new(10);
        assert!(tracker.register("1", "textDocument/hover", 1));
        assert_eq!(tracker.in_flight_count(), 1);
        assert!(!tracker.is_cancelled("1"));
        assert!(tracker.cancel("1"));
        assert!(tracker.is_cancelled("1"));
        let pruned = tracker.prune_cancelled();
        assert_eq!(pruned.len(), 1);
        assert_eq!(tracker.in_flight_count(), 0);
    }
    #[test]
    fn test_diagnostic_throttle() {
        let mut throttle = DiagnosticThrottle::new(5);
        assert!(throttle.should_update("file:///a.lean"));
        assert!(!throttle.should_update("file:///a.lean"));
    }
    #[test]
    fn test_lsp_session_notifications() {
        let mut session = LspSession::new();
        session.queue_notification("test/notification", JsonValue::Null);
        let notifs = session.drain_notifications();
        assert_eq!(notifs.len(), 1);
        assert!(session.drain_notifications().is_empty());
    }
    #[test]
    fn test_parse_and_serialize_message() {
        let msg = JsonRpcMessage::request(
            JsonValue::Number(1.0),
            "textDocument/hover",
            JsonValue::Null,
        );
        let serialized = serialize_message(&msg);
        let parsed = parse_message(&serialized).expect("parsing should succeed");
        assert_eq!(parsed.method.as_deref(), Some("textDocument/hover"));
    }
    #[test]
    fn test_client_capabilities_empty() {
        let caps = ClientCapabilities::from_json(&JsonValue::Null);
        assert!(!caps.snippet_support);
        assert!(!caps.markdown_hover);
    }
    #[test]
    fn test_message_channel() {
        let channel = MessageChannel::new();
        let msg = JsonRpcMessage::notification("test", JsonValue::Null);
        channel
            .sender
            .send(msg)
            .expect("channel send should succeed");
        let received = channel
            .receiver
            .recv()
            .expect("channel receive should succeed");
        assert_eq!(received.method.as_deref(), Some("test"));
    }
    #[test]
    fn test_dispatch_uninitialized() {
        let mut session = LspSession::new();
        let msg = JsonRpcMessage::request(
            JsonValue::Number(1.0),
            "textDocument/hover",
            JsonValue::Null,
        );
        let mut dispatcher = RequestDispatcher::new(&mut session);
        let result = dispatcher.dispatch(&msg);
        match result {
            DispatchResult::Response(resp) => {
                assert!(resp.error.is_some());
            }
            _ => panic!("expected error response for uninitialized server"),
        }
    }
    #[test]
    fn test_session_request_id() {
        let mut session = LspSession::new();
        let id1 = session.next_request_id();
        let id2 = session.next_request_id();
        assert_ne!(format_json_value(&id1), format_json_value(&id2));
    }
}
/// Return the server module version.
#[allow(dead_code)]
pub fn server_module_version() -> &'static str {
    "0.1.0"
}
/// Return the list of LSP capabilities supported.
#[allow(dead_code)]
pub fn supported_lsp_capabilities() -> Vec<&'static str> {
    vec![
        "textDocumentSync",
        "completionProvider",
        "hoverProvider",
        "definitionProvider",
        "referencesProvider",
        "documentSymbolProvider",
        "workspaceSymbolProvider",
        "codeActionProvider",
        "diagnosticProvider",
        "semanticTokensProvider",
        "inlayHintProvider",
        "renameProvider",
        "signatureHelpProvider",
    ]
}
#[cfg(test)]
mod server_extra_tests {
    use super::*;
    #[test]
    fn test_server_metrics() {
        let mut metrics = ServerMetrics::new();
        metrics.record_request(true, 1000);
        metrics.record_request(false, 500);
        assert_eq!(metrics.total_requests, 2);
        assert_eq!(metrics.successful_requests, 1);
        assert_eq!(metrics.failed_requests, 1);
        assert_eq!(metrics.max_request_latency_us, 1000);
        assert!((metrics.success_rate() - 50.0).abs() < 0.01);
    }
    #[test]
    fn test_text_document_edit_insert() {
        let edit = TextDocumentEdit::insert(0, 0, "theorem ");
        assert_eq!(edit.new_text, "theorem ");
        assert_eq!(edit.start_line, edit.end_line);
    }
    #[test]
    fn test_text_document_edit_apply() {
        let edit = TextDocumentEdit::replace(0, 0, 0, 5, "hello");
        let result = edit.apply("world foo");
        assert!(result.contains("hello"));
    }
    #[test]
    fn test_server_workspace() {
        let mut ws = ServerWorkspace::new(Some("file:///my_project".to_string()));
        assert_eq!(ws.name.as_deref(), Some("my_project"));
        ws.set_setting("maxDiagnostics", "100");
        assert_eq!(ws.get_setting("maxDiagnostics"), Some("100"));
    }
    #[test]
    fn test_server_request_queue() {
        let mut queue = ServerRequestQueue::new(10);
        assert!(queue.is_empty());
        let ok = queue.enqueue(
            "1".to_string(),
            "textDocument/hover".to_string(),
            "{}".to_string(),
        );
        assert!(ok);
        assert_eq!(queue.len(), 1);
        let req = queue.dequeue();
        assert!(req.is_some());
        assert!(queue.is_empty());
    }
    #[test]
    fn test_server_module_version() {
        assert!(!server_module_version().is_empty());
    }
    #[test]
    fn test_supported_lsp_capabilities() {
        let caps = supported_lsp_capabilities();
        assert!(caps.contains(&"completionProvider"));
        assert!(caps.contains(&"hoverProvider"));
    }
}
/// Return lsp server features.
#[allow(dead_code)]
pub fn server_features() -> Vec<&'static str> {
    vec![
        "stdio",
        "socket",
        "initialization",
        "shutdown",
        "document-sync",
        "completion",
        "hover",
        "diagnostics",
        "semantic-tokens",
        "inlay-hints",
        "code-actions",
        "workspace-symbols",
        "references",
    ]
}
#[cfg(test)]
mod server_negotiator_tests {
    use super::*;
    #[test]
    fn test_capability_negotiator() {
        let mut neg = CapabilityNegotiator::new();
        neg.set_client_capabilities(vec![
            "completionProvider".to_string(),
            "hoverProvider".to_string(),
            "unknownCap".to_string(),
        ]);
        let negotiated = neg.negotiated();
        assert!(negotiated.contains(&"completionProvider"));
        assert!(negotiated.contains(&"hoverProvider"));
        assert!(!negotiated.contains(&"unknownCap"));
    }
    #[test]
    fn test_capability_has() {
        let neg = CapabilityNegotiator::new();
        assert!(neg.has("completionProvider"));
        assert!(!neg.has("nonExistentCap"));
    }
    #[test]
    fn test_connection_info_stdio() {
        let info = ServerConnectionInfo::stdio();
        assert_eq!(info.transport, ServerTransport::Stdio);
        assert!(info.pid > 0);
    }
    #[test]
    fn test_server_features() {
        let features = server_features();
        assert!(features.contains(&"stdio"));
        assert!(features.contains(&"completion"));
    }
}
/// Format server uptime as a human-readable string.
#[allow(dead_code)]
pub fn format_uptime(secs: u64) -> String {
    let hours = secs / 3600;
    let minutes = (secs % 3600) / 60;
    let seconds = secs % 60;
    if hours > 0 {
        format!("{}h {}m {}s", hours, minutes, seconds)
    } else if minutes > 0 {
        format!("{}m {}s", minutes, seconds)
    } else {
        format!("{}s", seconds)
    }
}
#[cfg(test)]
mod shutdown_tests {
    use super::*;
    #[test]
    fn test_shutdown_handler() {
        let mut handler = ServerShutdownHandler::new();
        assert!(!handler.can_exit());
        handler.register_cleanup("save_state");
        handler.mark_shutdown();
        assert!(handler.can_exit());
        assert!(!handler.cleanup_fns.is_empty());
    }
    #[test]
    fn test_format_uptime() {
        assert_eq!(format_uptime(0), "0s");
        assert_eq!(format_uptime(65), "1m 5s");
        assert_eq!(format_uptime(3665), "1h 1m 5s");
    }
}
#[cfg(test)]
mod health_tests {
    use super::*;
    #[test]
    fn test_health_checker() {
        let checker = ServerHealthChecker::new();
        let status = checker.check();
        assert_eq!(status, HealthStatus::Healthy);
    }
}
/// Server module no-op.
#[allow(dead_code)]
pub fn server_noop() {}
