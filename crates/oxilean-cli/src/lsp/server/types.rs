//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use crate::lsp::{
    analyze_document, format_json_value, parse_json_value, Diagnostic, Document, InitializeResult,
    JsonRpcError, JsonRpcMessage, JsonValue, Location, LspConfig, LspServer, Position,
    PublishDiagnosticsParams, Range, ServerCapabilities, SymbolInformation, SymbolKind,
};
use oxilean_parse::{Lexer, TokenKind};
use std::process;
use std::sync::mpsc;

use std::collections::{HashMap, HashSet, VecDeque};

/// Client capabilities parsed from the initialize request.
#[derive(Clone, Debug, Default)]
pub struct ClientCapabilities {
    /// Whether the client supports dynamic registration.
    pub dynamic_registration: bool,
    /// Whether the client supports workspace folders.
    pub workspace_folders: bool,
    /// Whether the client supports snippet completions.
    pub snippet_support: bool,
    /// Whether the client supports markdown in hover.
    pub markdown_hover: bool,
    /// Whether the client supports code action resolve.
    pub code_action_resolve: bool,
    /// Whether the client supports semantic tokens.
    pub semantic_tokens: bool,
    /// Whether the client supports inlay hints.
    pub inlay_hints: bool,
    /// Whether the client supports pull diagnostics.
    pub pull_diagnostics: bool,
    /// Supported completion item kinds.
    pub completion_item_kinds: Vec<u32>,
    /// Supported code action kinds.
    pub code_action_kinds: Vec<String>,
}
impl ClientCapabilities {
    /// Parse from the initialize params JSON.
    pub fn from_json(params: &JsonValue) -> Self {
        let mut caps = Self::default();
        if let Some(client_caps) = params.get("capabilities") {
            if let Some(td) = client_caps.get("textDocument") {
                if let Some(completion) = td.get("completion") {
                    if let Some(ci) = completion.get("completionItem") {
                        caps.snippet_support = ci
                            .get("snippetSupport")
                            .and_then(|v| v.as_bool())
                            .unwrap_or(false);
                    }
                }
                if let Some(hover) = td.get("hover") {
                    if let Some(content_format) = hover.get("contentFormat") {
                        if let Some(arr) = content_format.as_array() {
                            for item in arr {
                                if item.as_str() == Some("markdown") {
                                    caps.markdown_hover = true;
                                }
                            }
                        }
                    }
                }
                if let Some(code_action) = td.get("codeAction") {
                    caps.code_action_resolve = code_action.get("resolveSupport").is_some();
                    if let Some(ca_kinds) = code_action
                        .get("codeActionLiteralSupport")
                        .and_then(|v| v.get("codeActionKind"))
                        .and_then(|v| v.get("valueSet"))
                        .and_then(|v| v.as_array())
                    {
                        for kind in ca_kinds {
                            if let Some(s) = kind.as_str() {
                                caps.code_action_kinds.push(s.to_string());
                            }
                        }
                    }
                }
                if td.get("semanticTokens").is_some() {
                    caps.semantic_tokens = true;
                }
                if td.get("inlayHint").is_some() {
                    caps.inlay_hints = true;
                }
            }
            if let Some(ws) = client_caps.get("workspace") {
                if let Some(wf) = ws.get("workspaceFolders") {
                    caps.workspace_folders = wf.as_bool().unwrap_or(false);
                }
            }
        }
        caps
    }
}
/// Message channel for async communication between threads.
pub struct MessageChannel {
    /// Sender end.
    pub sender: mpsc::Sender<JsonRpcMessage>,
    /// Receiver end.
    pub receiver: mpsc::Receiver<JsonRpcMessage>,
}
impl MessageChannel {
    /// Create a new message channel.
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel();
        Self { sender, receiver }
    }
}
/// The workspace tracked by the server.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct ServerWorkspace {
    pub root_uri: Option<String>,
    pub name: Option<String>,
    pub file_count: usize,
    pub settings: std::collections::HashMap<String, String>,
}
impl ServerWorkspace {
    /// Create a new workspace.
    #[allow(dead_code)]
    pub fn new(root_uri: Option<String>) -> Self {
        Self {
            name: root_uri
                .as_deref()
                .and_then(|u| u.rsplit('/').next())
                .map(String::from),
            root_uri,
            file_count: 0,
            settings: std::collections::HashMap::new(),
        }
    }
    /// Get a workspace setting.
    #[allow(dead_code)]
    pub fn get_setting(&self, key: &str) -> Option<&str> {
        self.settings.get(key).map(|s| s.as_str())
    }
    /// Set a workspace setting.
    #[allow(dead_code)]
    pub fn set_setting(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.settings.insert(key.into(), value.into());
    }
}
/// Configuration for workspace-level settings.
#[derive(Clone, Debug)]
pub struct WorkspaceConfig {
    /// Root URI of the workspace.
    pub root_uri: Option<String>,
    /// Root path (deprecated, but still supported).
    pub root_path: Option<String>,
    /// Workspace folders.
    pub workspace_folders: Vec<WorkspaceFolder>,
    /// Maximum number of diagnostics per file.
    pub max_diagnostics_per_file: usize,
    /// Whether to auto-check on save.
    pub check_on_save: bool,
    /// Whether to enable semantic tokens.
    pub semantic_tokens_enabled: bool,
    /// Whether to enable inlay hints.
    pub inlay_hints_enabled: bool,
}
/// Advanced request dispatcher with session management.
pub struct RequestDispatcher<'a> {
    /// Reference to the session.
    session: &'a mut LspSession,
}
impl<'a> RequestDispatcher<'a> {
    /// Create a new dispatcher.
    pub fn new(session: &'a mut LspSession) -> Self {
        Self { session }
    }
    /// Dispatch a single JSON-RPC message.
    pub fn dispatch(&mut self, msg: &JsonRpcMessage) -> DispatchResult {
        let method = match &msg.method {
            Some(m) => m.clone(),
            None => return DispatchResult::Handled,
        };
        let params = msg.params.clone().unwrap_or(JsonValue::Null);
        let id = msg.id.clone();
        match self.session.state {
            ServerState::Uninitialized if method != "initialize" => {
                return if let Some(id_val) = id {
                    DispatchResult::Response(JsonRpcMessage::error_response(
                        id_val,
                        JsonRpcError::new(-32002, "Server not initialized"),
                    ))
                } else {
                    DispatchResult::Handled
                };
            }
            ServerState::ShuttingDown if method != "exit" => {
                return if let Some(id_val) = id {
                    DispatchResult::Response(JsonRpcMessage::error_response(
                        id_val,
                        JsonRpcError::new(-32600, "Server is shutting down"),
                    ))
                } else {
                    DispatchResult::Handled
                };
            }
            ServerState::Exited => {
                return DispatchResult::Error("Server has exited".to_string());
            }
            _ => {}
        }
        match method.as_str() {
            "initialize" => self.handle_initialize(id, &params),
            "initialized" => {
                self.session.state = ServerState::Running;
                DispatchResult::Handled
            }
            "shutdown" => self.handle_shutdown(id),
            "exit" => {
                self.session.state = ServerState::Exited;
                DispatchResult::Exit
            }
            "textDocument/didOpen" => {
                self.handle_did_open(&params);
                DispatchResult::Handled
            }
            "textDocument/didChange" => {
                self.handle_did_change(&params);
                DispatchResult::Handled
            }
            "textDocument/didClose" => {
                self.handle_did_close(&params);
                DispatchResult::Handled
            }
            "textDocument/didSave" => {
                self.handle_did_save(&params);
                DispatchResult::Handled
            }
            "textDocument/willSave" => DispatchResult::Handled,
            "textDocument/completion" => {
                let result = self.session.server.handle_completion(&params);
                respond(id, result)
            }
            "textDocument/hover" => {
                let result = self.session.server.handle_hover(&params);
                respond(id, result)
            }
            "textDocument/definition" => {
                let result = self.session.server.handle_goto_definition(&params);
                respond(id, result)
            }
            "textDocument/references" => {
                let result = self.session.server.handle_references(&params);
                respond(id, result)
            }
            "textDocument/documentSymbol" => {
                let result = self.session.server.handle_document_symbol(&params);
                respond(id, result)
            }
            "textDocument/formatting" => {
                let result = self.session.server.handle_formatting(&params);
                respond(id, result)
            }
            "textDocument/signatureHelp" => {
                let result = self.session.server.handle_signature_help(&params);
                respond(id, result)
            }
            "textDocument/codeAction" => {
                let result = self.session.server.handle_code_action(&params);
                respond(id, result)
            }
            "textDocument/semanticTokens/full" => {
                let result = self.handle_semantic_tokens_full(&params);
                respond(id, result)
            }
            "textDocument/inlayHint" => {
                let result = self.handle_inlay_hints(&params);
                respond(id, result)
            }
            "workspace/didChangeConfiguration" => {
                self.handle_did_change_configuration(&params);
                DispatchResult::Handled
            }
            "workspace/didChangeWatchedFiles" => {
                self.handle_did_change_watched_files(&params);
                DispatchResult::Handled
            }
            "workspace/symbol" => {
                let result = self.handle_workspace_symbol(&params);
                respond(id, result)
            }
            "oxilean/goalState" => {
                let result = self.handle_goal_state(&params);
                respond(id, result)
            }
            "oxilean/proofTree" => {
                let result = self.handle_proof_tree(&params);
                respond(id, result)
            }
            "$/cancelRequest" => {
                self.handle_cancel_request(&params);
                DispatchResult::Handled
            }
            "$/setTrace" => DispatchResult::Handled,
            _ => {
                if let Some(id_val) = id {
                    DispatchResult::Response(JsonRpcMessage::error_response(
                        id_val,
                        JsonRpcError::method_not_found(&method),
                    ))
                } else {
                    DispatchResult::Handled
                }
            }
        }
    }
    /// Handle the initialize request.
    fn handle_initialize(&mut self, id: Option<JsonValue>, params: &JsonValue) -> DispatchResult {
        self.session.state = ServerState::Initializing;
        self.session.client_capabilities = ClientCapabilities::from_json(params);
        if let Some(root_uri) = params.get("rootUri").and_then(|v| v.as_str()) {
            self.session.workspace_config.root_uri = Some(root_uri.to_string());
        }
        if let Some(root_path) = params.get("rootPath").and_then(|v| v.as_str()) {
            self.session.workspace_config.root_path = Some(root_path.to_string());
        }
        if let Some(folders) = params.get("workspaceFolders").and_then(|v| v.as_array()) {
            for folder_val in folders {
                if let Ok(folder) = WorkspaceFolder::from_json(folder_val) {
                    self.session.workspace_config.workspace_folders.push(folder);
                }
            }
        }
        let mut capabilities = ServerCapabilities::oxilean_defaults();
        if self.session.client_capabilities.semantic_tokens {
            capabilities.text_document_sync = 2;
        }
        let result = InitializeResult {
            capabilities,
            server_name: "oxilean-lsp".to_string(),
            server_version: "0.2.0".to_string(),
        };
        self.session.server.initialized = true;
        respond(id, result.to_json())
    }
    /// Handle the shutdown request.
    fn handle_shutdown(&mut self, id: Option<JsonValue>) -> DispatchResult {
        self.session.state = ServerState::ShuttingDown;
        self.session.server.shutdown_requested = true;
        respond(id, JsonValue::Null)
    }
    /// Handle textDocument/didOpen.
    fn handle_did_open(&mut self, params: &JsonValue) {
        self.session.server.handle_text_document_did_open(params);
        if let Some(uri) = params
            .get("textDocument")
            .and_then(|td| td.get("uri"))
            .and_then(|v| v.as_str())
        {
            let diagnostics = self.session.server.validate_document(uri);
            self.session.publish_diagnostics(uri, diagnostics, None);
        }
    }
    /// Handle textDocument/didChange.
    fn handle_did_change(&mut self, params: &JsonValue) {
        let uri = params
            .get("textDocument")
            .and_then(|td| td.get("uri"))
            .and_then(|v| v.as_str())
            .map(String::from);
        let version = params
            .get("textDocument")
            .and_then(|td| td.get("version"))
            .and_then(|v| v.as_i64());
        if let Some(changes) = params.get("contentChanges").and_then(|v| v.as_array()) {
            if let Some(ref uri_str) = uri {
                for change in changes {
                    if let Some(range_val) = change.get("range") {
                        if let (Ok(range), Some(text)) = (
                            Range::from_json(range_val),
                            change.get("text").and_then(|v| v.as_str()),
                        ) {
                            self.apply_incremental_change(uri_str, &range, text);
                        }
                    } else if let Some(text) = change.get("text").and_then(|v| v.as_str()) {
                        if let Some(ver) = version {
                            self.session
                                .server
                                .document_store
                                .update_document(uri_str, ver, text);
                            self.session.server.cache.invalidate(uri_str);
                        }
                    }
                }
            }
        }
        if let Some(uri_str) = uri {
            let diagnostics = self.session.server.validate_document(&uri_str);
            self.session
                .publish_diagnostics(&uri_str, diagnostics, version);
        }
    }
    /// Apply an incremental text change to a document.
    fn apply_incremental_change(&mut self, uri: &str, range: &Range, new_text: &str) {
        if let Some(doc) = self.session.server.document_store.get_document(uri) {
            let start_offset = doc.position_to_offset(&range.start).unwrap_or(0);
            let end_offset = doc
                .position_to_offset(&range.end)
                .unwrap_or(doc.content.len());
            let mut new_content = String::with_capacity(
                doc.content.len() - (end_offset - start_offset) + new_text.len(),
            );
            new_content.push_str(&doc.content[..start_offset]);
            new_content.push_str(new_text);
            new_content.push_str(&doc.content[end_offset..]);
            let new_version = doc.version + 1;
            self.session
                .server
                .document_store
                .update_document(uri, new_version, new_content);
            self.session.server.cache.invalidate(uri);
        }
    }
    /// Handle textDocument/didClose.
    fn handle_did_close(&mut self, params: &JsonValue) {
        self.session.server.handle_text_document_did_close(params);
    }
    /// Handle textDocument/didSave.
    fn handle_did_save(&mut self, params: &JsonValue) {
        if !self.session.workspace_config.check_on_save {
            return;
        }
        if let Some(uri) = params
            .get("textDocument")
            .and_then(|td| td.get("uri"))
            .and_then(|v| v.as_str())
        {
            let diagnostics = self.session.server.validate_document(uri);
            self.session.publish_diagnostics(uri, diagnostics, None);
            let dependents = self.session.dependency_graph.get_dependents(uri);
            for dep_uri in dependents {
                let dep_diags = self.session.server.validate_document(&dep_uri);
                self.session.publish_diagnostics(&dep_uri, dep_diags, None);
            }
        }
    }
    /// Handle workspace/didChangeConfiguration.
    fn handle_did_change_configuration(&mut self, params: &JsonValue) {
        if let Some(settings) = params.get("settings") {
            if let Some(oxilean) = settings.get("oxilean") {
                if let Some(max_diag) = oxilean.get("maxDiagnostics").and_then(|v| v.as_i64()) {
                    self.session.workspace_config.max_diagnostics_per_file = max_diag as usize;
                    self.session.server.config.max_diagnostics = max_diag as usize;
                }
                if let Some(check_on_save) = oxilean.get("checkOnSave").and_then(|v| v.as_bool()) {
                    self.session.workspace_config.check_on_save = check_on_save;
                }
                if let Some(sem_tokens) = oxilean.get("semanticTokens").and_then(|v| v.as_bool()) {
                    self.session.workspace_config.semantic_tokens_enabled = sem_tokens;
                }
                if let Some(inlay) = oxilean.get("inlayHints").and_then(|v| v.as_bool()) {
                    self.session.workspace_config.inlay_hints_enabled = inlay;
                }
            }
        }
    }
    /// Handle workspace/didChangeWatchedFiles.
    fn handle_did_change_watched_files(&mut self, params: &JsonValue) {
        if let Some(changes) = params.get("changes").and_then(|v| v.as_array()) {
            for change in changes {
                let _uri = change.get("uri").and_then(|v| v.as_str());
                let _change_type = change.get("type").and_then(|v| v.as_i64());
            }
            self.session.server.cache.clear();
        }
    }
    /// Handle workspace/symbol.
    fn handle_workspace_symbol(&self, params: &JsonValue) -> JsonValue {
        let query = params.get("query").and_then(|v| v.as_str()).unwrap_or("");
        let mut symbols = Vec::new();
        for uri in self.session.server.document_store.uris() {
            if let Some(doc) = self.session.server.document_store.get_document(uri) {
                let analysis = analyze_document(uri, &doc.content, &self.session.server.env);
                for def in &analysis.definitions {
                    if query.is_empty() || def.name.contains(query) {
                        symbols.push(
                            SymbolInformation {
                                name: def.name.clone(),
                                kind: def.kind,
                                location: Location::new(uri.as_str(), def.range.clone()),
                            }
                            .to_json(),
                        );
                    }
                }
            }
        }
        for name in self.session.server.env.constant_names() {
            let name_str = name.to_string();
            if query.is_empty() || name_str.contains(query) {
                let kind = if self.session.server.env.is_inductive(name) {
                    SymbolKind::Enum
                } else if self.session.server.env.is_constructor(name) {
                    SymbolKind::Constant
                } else {
                    SymbolKind::Function
                };
                symbols.push(
                    SymbolInformation {
                        name: name_str,
                        kind,
                        location: Location::new(
                            "kernel://environment",
                            Range::single_line(0, 0, 0),
                        ),
                    }
                    .to_json(),
                );
            }
        }
        if symbols.len() > 200 {
            symbols.truncate(200);
        }
        JsonValue::Array(symbols)
    }
    /// Handle semantic tokens full request.
    fn handle_semantic_tokens_full(&self, params: &JsonValue) -> JsonValue {
        let uri = params
            .get("textDocument")
            .and_then(|td| td.get("uri"))
            .and_then(|v| v.as_str());
        if let Some(uri) = uri {
            if let Some(doc) = self.session.server.document_store.get_document(uri) {
                let data = compute_semantic_tokens_data(&doc.content, &self.session.server.env);
                return JsonValue::Object(vec![(
                    "data".to_string(),
                    JsonValue::Array(data.iter().map(|n| JsonValue::Number(*n as f64)).collect()),
                )]);
            }
        }
        JsonValue::Object(vec![("data".to_string(), JsonValue::Array(Vec::new()))])
    }
    /// Handle inlay hints request.
    fn handle_inlay_hints(&self, params: &JsonValue) -> JsonValue {
        let uri = params
            .get("textDocument")
            .and_then(|td| td.get("uri"))
            .and_then(|v| v.as_str());
        if let Some(uri) = uri {
            if let Some(doc) = self.session.server.document_store.get_document(uri) {
                let hints = compute_inlay_hints(doc, &self.session.server.env);
                return JsonValue::Array(hints);
            }
        }
        JsonValue::Array(Vec::new())
    }
    /// Handle $/cancelRequest notification.
    fn handle_cancel_request(&mut self, params: &JsonValue) {
        if let Some(id) = params.get("id") {
            let id_str = format_json_value(id);
            self.session.pending_requests.remove(&id_str);
        }
    }
    /// Handle custom oxilean/goalState request.
    fn handle_goal_state(&self, params: &JsonValue) -> JsonValue {
        let uri = params
            .get("textDocument")
            .and_then(|td| td.get("uri"))
            .and_then(|v| v.as_str());
        let position = params
            .get("position")
            .and_then(|p| Position::from_json(p).ok());
        if let (Some(uri), Some(pos)) = (uri, position) {
            if let Some(doc) = self.session.server.document_store.get_document(uri) {
                return compute_goal_state(doc, &pos, &self.session.server.env);
            }
        }
        JsonValue::Null
    }
    /// Handle custom oxilean/proofTree request.
    fn handle_proof_tree(&self, params: &JsonValue) -> JsonValue {
        let uri = params
            .get("textDocument")
            .and_then(|td| td.get("uri"))
            .and_then(|v| v.as_str());
        if let Some(uri) = uri {
            if let Some(doc) = self.session.server.document_store.get_document(uri) {
                return compute_proof_tree(doc, &self.session.server.env);
            }
        }
        JsonValue::Null
    }
}
/// Negotiates capabilities between client and server.
#[allow(dead_code)]
pub struct CapabilityNegotiator {
    server_capabilities: Vec<String>,
    client_capabilities: Vec<String>,
}
impl CapabilityNegotiator {
    /// Create a negotiator.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            server_capabilities: supported_lsp_capabilities()
                .into_iter()
                .map(String::from)
                .collect(),
            client_capabilities: vec![],
        }
    }
    /// Set the client's declared capabilities.
    #[allow(dead_code)]
    pub fn set_client_capabilities(&mut self, caps: Vec<String>) {
        self.client_capabilities = caps;
    }
    /// Return the intersection (mutually supported capabilities).
    #[allow(dead_code)]
    pub fn negotiated(&self) -> Vec<&str> {
        self.server_capabilities
            .iter()
            .filter(|sc| self.client_capabilities.iter().any(|cc| cc == *sc))
            .map(|s| s.as_str())
            .collect()
    }
    /// Check whether a specific capability was negotiated.
    #[allow(dead_code)]
    pub fn has(&self, capability: &str) -> bool {
        self.server_capabilities.iter().any(|c| c == capability)
    }
}
/// Semantic token types as defined by LSP.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SemanticTokenType {
    /// A namespace identifier.
    Namespace = 0,
    /// A type identifier.
    Type = 1,
    /// A class identifier.
    Class = 2,
    /// An enum identifier.
    Enum = 3,
    /// A type parameter.
    TypeParameter = 4,
    /// A function identifier.
    Function = 5,
    /// A method identifier.
    Method = 6,
    /// A property identifier.
    Property = 7,
    /// A variable identifier.
    Variable = 8,
    /// A parameter.
    Parameter = 9,
    /// A string literal.
    StringLit = 10,
    /// A number literal.
    Number = 11,
    /// A keyword.
    Keyword = 12,
    /// A comment.
    Comment = 13,
    /// An operator.
    Operator = 14,
}
/// Performance metrics for the LSP server.
#[allow(dead_code)]
#[derive(Clone, Debug, Default)]
pub struct ServerMetrics {
    pub total_requests: u64,
    pub successful_requests: u64,
    pub failed_requests: u64,
    pub total_notifications: u64,
    pub avg_request_latency_us: f64,
    pub max_request_latency_us: u64,
    pub documents_open: usize,
    pub diagnostics_published: u64,
}
impl ServerMetrics {
    /// Create new zeroed metrics.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    /// Record a request completion.
    #[allow(dead_code)]
    pub fn record_request(&mut self, success: bool, latency_us: u64) {
        self.total_requests += 1;
        if success {
            self.successful_requests += 1;
        } else {
            self.failed_requests += 1;
        }
        if latency_us > self.max_request_latency_us {
            self.max_request_latency_us = latency_us;
        }
        let n = self.total_requests as f64;
        self.avg_request_latency_us =
            (self.avg_request_latency_us * (n - 1.0) + latency_us as f64) / n;
    }
    /// Return the success rate as a percentage.
    #[allow(dead_code)]
    pub fn success_rate(&self) -> f64 {
        if self.total_requests == 0 {
            100.0
        } else {
            100.0 * self.successful_requests as f64 / self.total_requests as f64
        }
    }
}
/// A queue for pending server requests.
#[allow(dead_code)]
pub struct ServerRequestQueue {
    pending: std::collections::VecDeque<QueuedRequest>,
    max_size: usize,
}
impl ServerRequestQueue {
    /// Create a new queue.
    #[allow(dead_code)]
    pub fn new(max_size: usize) -> Self {
        Self {
            pending: std::collections::VecDeque::new(),
            max_size,
        }
    }
    /// Enqueue a request.
    #[allow(dead_code)]
    pub fn enqueue(&mut self, id: String, method: String, params: String) -> bool {
        if self.pending.len() >= self.max_size {
            return false;
        }
        self.pending.push_back(QueuedRequest {
            id,
            method,
            params,
            enqueued_at: std::time::Instant::now(),
        });
        true
    }
    /// Dequeue the next request.
    #[allow(dead_code)]
    pub fn dequeue(&mut self) -> Option<QueuedRequest> {
        self.pending.pop_front()
    }
    /// Return the number of pending requests.
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.pending.len()
    }
    /// Return whether the queue is empty.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.pending.is_empty()
    }
}
/// The result of dispatching a message.
pub enum DispatchResult {
    /// A response to send back to the client.
    Response(JsonRpcMessage),
    /// No response needed (notification was handled).
    Handled,
    /// Server should exit.
    Exit,
    /// An error occurred during dispatch.
    Error(String),
}
/// A workspace folder as specified by LSP.
#[derive(Clone, Debug)]
pub struct WorkspaceFolder {
    /// The associated URI for this workspace folder.
    pub uri: String,
    /// The name of the workspace folder.
    pub name: String,
}
impl WorkspaceFolder {
    /// Parse from JSON.
    pub fn from_json(val: &JsonValue) -> Result<Self, String> {
        let uri = val
            .get("uri")
            .and_then(|v| v.as_str())
            .ok_or("missing workspace folder uri")?
            .to_string();
        let name = val
            .get("name")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        Ok(Self { uri, name })
    }
    /// Serialize to JSON.
    pub fn to_json(&self) -> JsonValue {
        JsonValue::Object(vec![
            ("uri".to_string(), JsonValue::String(self.uri.clone())),
            ("name".to_string(), JsonValue::String(self.name.clone())),
        ])
    }
}
/// Performs health checks on the server.
#[allow(dead_code)]
pub struct ServerHealthChecker {
    pub checks: Vec<String>,
}
impl ServerHealthChecker {
    /// Create a new health checker.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            checks: vec![
                "document_store".to_string(),
                "workspace".to_string(),
                "elaborator".to_string(),
            ],
        }
    }
    /// Run a simple health check (always healthy in this stub).
    #[allow(dead_code)]
    pub fn check(&self) -> HealthStatus {
        HealthStatus::Healthy
    }
}
/// Rate limiter for diagnostic updates to avoid flooding the client.
pub struct DiagnosticThrottle {
    /// Minimum interval between diagnostic updates (in ms).
    pub min_interval_ms: u64,
    /// Last update time per URI (as a simple counter for now).
    last_update: HashMap<String, u64>,
    /// Current time counter.
    counter: u64,
}
impl DiagnosticThrottle {
    /// Create a new throttle with the given minimum interval.
    pub fn new(min_interval_ms: u64) -> Self {
        Self {
            min_interval_ms,
            last_update: HashMap::new(),
            counter: 0,
        }
    }
    /// Check if we should send diagnostics for the given URI.
    pub fn should_update(&mut self, uri: &str) -> bool {
        self.counter += 1;
        if let Some(&last) = self.last_update.get(uri) {
            if self.counter - last >= self.min_interval_ms {
                self.last_update.insert(uri.to_string(), self.counter);
                true
            } else {
                false
            }
        } else {
            self.last_update.insert(uri.to_string(), self.counter);
            true
        }
    }
    /// Force an update for the given URI.
    pub fn force_update(&mut self, uri: &str) {
        self.counter += 1;
        self.last_update.insert(uri.to_string(), self.counter);
    }
    /// Reset the throttle.
    pub fn reset(&mut self) {
        self.last_update.clear();
        self.counter = 0;
    }
}
/// Handles graceful server shutdown.
#[allow(dead_code)]
pub struct ServerShutdownHandler {
    pub received_shutdown: bool,
    pub cleanup_fns: Vec<String>,
}
impl ServerShutdownHandler {
    /// Create a new handler.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            received_shutdown: false,
            cleanup_fns: vec![],
        }
    }
    /// Register a cleanup action name.
    #[allow(dead_code)]
    pub fn register_cleanup(&mut self, name: impl Into<String>) {
        self.cleanup_fns.push(name.into());
    }
    /// Mark shutdown received.
    #[allow(dead_code)]
    pub fn mark_shutdown(&mut self) {
        self.received_shutdown = true;
    }
    /// Check if exit is allowed.
    #[allow(dead_code)]
    pub fn can_exit(&self) -> bool {
        self.received_shutdown
    }
}
/// How the server synchronizes document content.
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DocumentSyncMode {
    /// No synchronization
    None,
    /// Full document on each change
    Full,
    /// Incremental edits
    Incremental,
}
/// Information about the server connection.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct ServerConnectionInfo {
    pub transport: ServerTransport,
    pub pid: u32,
    pub server_version: String,
}
impl ServerConnectionInfo {
    /// Create info for stdio transport.
    #[allow(dead_code)]
    pub fn stdio() -> Self {
        Self {
            transport: ServerTransport::Stdio,
            pid: std::process::id(),
            server_version: server_module_version().to_string(),
        }
    }
    /// Create info for socket transport.
    #[allow(dead_code)]
    pub fn socket(port: u16) -> Self {
        Self {
            transport: ServerTransport::Socket { port },
            pid: std::process::id(),
            server_version: server_module_version().to_string(),
        }
    }
}
/// Tracks in-flight requests and provides cancellation support.
pub struct RequestTracker {
    /// Map from request ID to method name.
    in_flight: HashMap<String, RequestInfo>,
    /// Maximum number of concurrent requests.
    max_concurrent: usize,
}
impl RequestTracker {
    /// Create a new request tracker.
    pub fn new(max_concurrent: usize) -> Self {
        Self {
            in_flight: HashMap::new(),
            max_concurrent,
        }
    }
    /// Register a new in-flight request.
    pub fn register(&mut self, id: &str, method: &str, sequence: u64) -> bool {
        if self.in_flight.len() >= self.max_concurrent {
            return false;
        }
        self.in_flight.insert(
            id.to_string(),
            RequestInfo {
                method: method.to_string(),
                sequence,
                cancelled: false,
            },
        );
        true
    }
    /// Mark a request as completed.
    pub fn complete(&mut self, id: &str) -> Option<RequestInfo> {
        self.in_flight.remove(id)
    }
    /// Cancel a request.
    pub fn cancel(&mut self, id: &str) -> bool {
        if let Some(info) = self.in_flight.get_mut(id) {
            info.cancelled = true;
            true
        } else {
            false
        }
    }
    /// Check if a request has been cancelled.
    pub fn is_cancelled(&self, id: &str) -> bool {
        self.in_flight
            .get(id)
            .map(|info| info.cancelled)
            .unwrap_or(false)
    }
    /// Get the count of in-flight requests.
    pub fn in_flight_count(&self) -> usize {
        self.in_flight.len()
    }
    /// Remove all cancelled requests.
    pub fn prune_cancelled(&mut self) -> Vec<String> {
        let cancelled: Vec<String> = self
            .in_flight
            .iter()
            .filter(|(_, info)| info.cancelled)
            .map(|(id, _)| id.clone())
            .collect();
        for id in &cancelled {
            self.in_flight.remove(id);
        }
        cancelled
    }
}
/// A batch of messages to process together.
pub struct MessageBatch {
    /// The messages in this batch.
    pub messages: Vec<JsonRpcMessage>,
}
impl MessageBatch {
    /// Create a new empty batch.
    pub fn new() -> Self {
        Self {
            messages: Vec::new(),
        }
    }
    /// Add a message to the batch.
    pub fn push(&mut self, msg: JsonRpcMessage) {
        self.messages.push(msg);
    }
    /// Check if the batch is empty.
    pub fn is_empty(&self) -> bool {
        self.messages.is_empty()
    }
    /// Get the number of messages.
    pub fn len(&self) -> usize {
        self.messages.len()
    }
}
/// Health check status for the LSP server.
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum HealthStatus {
    Healthy,
    Degraded { reason: String },
    Unhealthy { reason: String },
}
/// Represents the lifecycle state of the LSP server.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ServerState {
    /// Server has not yet received an initialize request.
    Uninitialized,
    /// Server has received initialize but not yet initialized notification.
    Initializing,
    /// Server is fully initialized and ready to handle requests.
    Running,
    /// Server received a shutdown request, waiting for exit.
    ShuttingDown,
    /// Server has exited.
    Exited,
}
/// Information about an in-flight request.
#[derive(Clone, Debug)]
pub struct RequestInfo {
    /// The method name.
    pub method: String,
    /// When the request was received (as a monotonic counter).
    pub sequence: u64,
    /// Whether the request has been cancelled.
    pub cancelled: bool,
}
/// Server transport kind.
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ServerTransport {
    Stdio,
    Socket { port: u16 },
    Pipe { name: String },
}
/// State of a progress operation.
#[derive(Clone, Debug)]
pub struct ProgressState {
    /// Token identifying this progress operation.
    pub token: String,
    /// Current message displayed.
    pub message: Option<String>,
    /// Progress percentage (0-100).
    pub percentage: Option<u32>,
    /// Whether the operation is complete.
    pub done: bool,
}
/// A single text edit applied to a document.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct TextDocumentEdit {
    pub start_line: u32,
    pub start_char: u32,
    pub end_line: u32,
    pub end_char: u32,
    pub new_text: String,
}
impl TextDocumentEdit {
    /// Create an insertion at a position.
    #[allow(dead_code)]
    pub fn insert(line: u32, char_pos: u32, text: impl Into<String>) -> Self {
        Self {
            start_line: line,
            start_char: char_pos,
            end_line: line,
            end_char: char_pos,
            new_text: text.into(),
        }
    }
    /// Create a deletion of a range.
    #[allow(dead_code)]
    pub fn delete(start_line: u32, start_char: u32, end_line: u32, end_char: u32) -> Self {
        Self {
            start_line,
            start_char,
            end_line,
            end_char,
            new_text: String::new(),
        }
    }
    /// Create a replacement of a range.
    #[allow(dead_code)]
    pub fn replace(
        start_line: u32,
        start_char: u32,
        end_line: u32,
        end_char: u32,
        new_text: impl Into<String>,
    ) -> Self {
        Self {
            start_line,
            start_char,
            end_line,
            end_char,
            new_text: new_text.into(),
        }
    }
    /// Apply this edit to a string (simple line-based).
    #[allow(dead_code)]
    pub fn apply(&self, content: &str) -> String {
        let mut lines: Vec<String> = content.lines().map(String::from).collect();
        while lines.len() <= self.end_line as usize {
            lines.push(String::new());
        }
        if self.start_line == self.end_line {
            let line = &mut lines[self.start_line as usize];
            let start = self.start_char as usize;
            let end = self.end_char as usize;
            let end = end.min(line.len());
            let start = start.min(line.len());
            line.replace_range(start..end, &self.new_text);
        }
        lines.join("\n")
    }
}
/// A queued request.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct QueuedRequest {
    pub id: String,
    pub method: String,
    pub params: String,
    pub enqueued_at: std::time::Instant,
}
/// Tracks the session state of a running LSP server.
pub struct LspSession {
    /// The current server state.
    pub state: ServerState,
    /// Client capabilities negotiated during initialization.
    pub client_capabilities: ClientCapabilities,
    /// Workspace configuration.
    pub workspace_config: WorkspaceConfig,
    /// The core server instance handling requests.
    pub server: LspServer,
    /// Pending request IDs that have not yet been responded to.
    pub pending_requests: HashSet<String>,
    /// Progress tokens for long-running operations.
    pub progress_tokens: HashMap<String, ProgressState>,
    /// Outgoing notification queue.
    pub notification_queue: Vec<JsonRpcMessage>,
    /// File dependency graph for incremental checking.
    pub dependency_graph: DependencyGraph,
    /// Request counter for generating unique IDs.
    request_counter: u64,
}
impl LspSession {
    /// Create a new session.
    pub fn new() -> Self {
        Self {
            state: ServerState::Uninitialized,
            client_capabilities: ClientCapabilities::default(),
            workspace_config: WorkspaceConfig::default(),
            server: LspServer::new(),
            pending_requests: HashSet::new(),
            progress_tokens: HashMap::new(),
            notification_queue: Vec::new(),
            dependency_graph: DependencyGraph::new(),
            request_counter: 0,
        }
    }
    /// Create with custom configuration.
    pub fn with_config(config: LspConfig) -> Self {
        Self {
            state: ServerState::Uninitialized,
            client_capabilities: ClientCapabilities::default(),
            workspace_config: WorkspaceConfig::default(),
            server: LspServer::with_config(config),
            pending_requests: HashSet::new(),
            progress_tokens: HashMap::new(),
            notification_queue: Vec::new(),
            dependency_graph: DependencyGraph::new(),
            request_counter: 0,
        }
    }
    /// Generate a unique request ID.
    pub fn next_request_id(&mut self) -> JsonValue {
        self.request_counter += 1;
        JsonValue::Number(self.request_counter as f64)
    }
    /// Check if the server is in a state that can handle requests.
    pub fn can_handle_requests(&self) -> bool {
        self.state == ServerState::Running
    }
    /// Drain the notification queue.
    pub fn drain_notifications(&mut self) -> Vec<JsonRpcMessage> {
        std::mem::take(&mut self.notification_queue)
    }
    /// Queue a notification to be sent to the client.
    pub fn queue_notification(&mut self, method: &str, params: JsonValue) {
        self.notification_queue
            .push(JsonRpcMessage::notification(method, params));
    }
    /// Queue a diagnostics notification for a document.
    pub fn publish_diagnostics(
        &mut self,
        uri: &str,
        diagnostics: Vec<Diagnostic>,
        version: Option<i64>,
    ) {
        let params = PublishDiagnosticsParams {
            uri: uri.to_string(),
            diagnostics,
            version,
        };
        self.queue_notification("textDocument/publishDiagnostics", params.to_json());
    }
}
/// Tracks dependencies between files for incremental checking.
#[derive(Clone, Debug, Default)]
pub struct DependencyGraph {
    /// Maps a file URI to the set of files it depends on (imports).
    dependencies: HashMap<String, HashSet<String>>,
    /// Reverse map: file URI to the set of files that depend on it.
    dependents: HashMap<String, HashSet<String>>,
}
impl DependencyGraph {
    /// Create a new empty dependency graph.
    pub fn new() -> Self {
        Self {
            dependencies: HashMap::new(),
            dependents: HashMap::new(),
        }
    }
    /// Add a dependency: `from` depends on `to`.
    pub fn add_dependency(&mut self, from: &str, to: &str) {
        self.dependencies
            .entry(from.to_string())
            .or_default()
            .insert(to.to_string());
        self.dependents
            .entry(to.to_string())
            .or_default()
            .insert(from.to_string());
    }
    /// Remove all dependencies for a file.
    pub fn remove_file(&mut self, uri: &str) {
        if let Some(deps) = self.dependencies.remove(uri) {
            for dep in &deps {
                if let Some(rev) = self.dependents.get_mut(dep) {
                    rev.remove(uri);
                }
            }
        }
        if let Some(revs) = self.dependents.remove(uri) {
            for rev in &revs {
                if let Some(fwd) = self.dependencies.get_mut(rev) {
                    fwd.remove(uri);
                }
            }
        }
    }
    /// Get files that depend on the given file (direct dependents).
    pub fn get_dependents(&self, uri: &str) -> Vec<String> {
        self.dependents
            .get(uri)
            .map(|s| s.iter().cloned().collect())
            .unwrap_or_default()
    }
    /// Get files that the given file depends on (direct dependencies).
    pub fn get_dependencies(&self, uri: &str) -> Vec<String> {
        self.dependencies
            .get(uri)
            .map(|s| s.iter().cloned().collect())
            .unwrap_or_default()
    }
    /// Get all transitive dependents (files affected by a change).
    pub fn get_transitive_dependents(&self, uri: &str) -> Vec<String> {
        let mut visited = HashSet::new();
        let mut queue = vec![uri.to_string()];
        while let Some(current) = queue.pop() {
            if visited.contains(&current) {
                continue;
            }
            visited.insert(current.clone());
            if let Some(deps) = self.dependents.get(&current) {
                for dep in deps {
                    if !visited.contains(dep) {
                        queue.push(dep.clone());
                    }
                }
            }
        }
        visited.remove(uri);
        visited.into_iter().collect()
    }
    /// Update dependencies for a file by scanning its import statements.
    pub fn update_from_content(&mut self, uri: &str, content: &str) {
        if let Some(old_deps) = self.dependencies.remove(uri) {
            for dep in &old_deps {
                if let Some(rev) = self.dependents.get_mut(dep) {
                    rev.remove(uri);
                }
            }
        }
        let mut lexer = Lexer::new(content);
        let tokens = lexer.tokenize();
        let mut i = 0;
        while i < tokens.len() {
            if tokens[i].kind == TokenKind::Import {
                let mut module_parts = Vec::new();
                i += 1;
                while i < tokens.len() {
                    match &tokens[i].kind {
                        TokenKind::Ident(name) => module_parts.push(name.clone()),
                        TokenKind::Dot => {}
                        _ => break,
                    }
                    i += 1;
                }
                if !module_parts.is_empty() {
                    let module_path = module_parts.join(".");
                    let dep_uri = format!("file:///{}.lean", module_path.replace('.', "/"));
                    self.add_dependency(uri, &dep_uri);
                }
            } else {
                i += 1;
            }
        }
    }
    /// Clear the entire graph.
    pub fn clear(&mut self) {
        self.dependencies.clear();
        self.dependents.clear();
    }
    /// Get the number of tracked files.
    pub fn file_count(&self) -> usize {
        let mut all_files = HashSet::new();
        for (k, vs) in &self.dependencies {
            all_files.insert(k.clone());
            for v in vs {
                all_files.insert(v.clone());
            }
        }
        all_files.len()
    }
}
