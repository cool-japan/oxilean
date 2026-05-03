//! LSP server core: LspConfig and LspServer implementation.

use std::collections::HashMap;

use oxilean_elab::info_tree::InfoTree;
use oxilean_kernel::{Environment, Name};

use super::analysis::{
    analyze_document, find_references_in_document, format_document, get_keyword_hover,
    make_code_action, AnalysisCache,
};
use super::document::{is_ident_char, Document, DocumentStore};
use super::json_rpc::{JsonRpcError, JsonRpcMessage, JsonValue, METHOD_NOT_FOUND};
use super::lsp_types::{
    CompletionItem, Hover, InitializeResult, Location, MarkupContent, ParameterInformation,
    Position, Range, ServerCapabilities, SignatureHelp, SignatureInformation, SymbolKind,
    TextDocumentItem, TextEdit,
};

// ── Position ↔ byte-offset helpers ────────────────────────────────────────────

/// Convert an LSP `Position` (line/character) to a byte offset in `content`.
///
/// Returns `None` if the line does not exist.
fn position_to_offset(content: &str, line: u32, character: u32) -> Option<usize> {
    let mut current_line: u32 = 0;
    let mut offset = 0usize;
    for ch in content.chars() {
        if current_line == line {
            // character is UTF-16 code unit count; approximate with char count for ASCII
            let target_char = character as usize;
            let line_start = offset;
            let mut col = 0usize;
            for c in content[line_start..].chars() {
                if col == target_char {
                    return Some(line_start + col);
                }
                if c == '\n' {
                    break;
                }
                col += c.len_utf8();
            }
            return Some(line_start + col.min(target_char));
        }
        if ch == '\n' {
            current_line += 1;
        }
        offset += ch.len_utf8();
    }
    if current_line == line {
        Some(offset)
    } else {
        None
    }
}

/// Convert a byte offset back to an LSP `Position`.
fn offset_to_position(content: &str, offset: usize) -> Position {
    let clamped = offset.min(content.len());
    let before = &content[..clamped];
    let line = before.chars().filter(|c| *c == '\n').count() as u32;
    let col = before
        .rfind('\n')
        .map(|nl| clamped - nl - 1)
        .unwrap_or(clamped) as u32;
    Position {
        line,
        character: col,
    }
}

// ── LspConfig ─────────────────────────────────────────────────────────────────

/// Configuration for the LSP server.
#[derive(Clone, Debug)]
pub struct LspConfig {
    /// Maximum number of diagnostics to report.
    pub max_diagnostics: usize,
    /// Whether to enable completions.
    pub enable_completions: bool,
    /// Whether to enable hover.
    pub enable_hover: bool,
    /// Whether to enable go-to-definition.
    pub enable_goto_definition: bool,
}

impl Default for LspConfig {
    fn default() -> Self {
        Self {
            max_diagnostics: 100,
            enable_completions: true,
            enable_hover: true,
            enable_goto_definition: true,
        }
    }
}

// ── LspServer ─────────────────────────────────────────────────────────────────

/// The core LSP server.
pub struct LspServer {
    /// Open document storage.
    pub document_store: DocumentStore,
    /// Kernel environment.
    pub env: Environment,
    /// Server configuration.
    pub config: LspConfig,
    /// Analysis cache.
    pub cache: AnalysisCache,
    /// Whether the server has been initialized.
    pub initialized: bool,
    /// Whether a shutdown was requested.
    pub shutdown_requested: bool,
    /// Elaboration-backed declaration info: maps URI → Vec<(name, type_str)>.
    pub doc_elaborations: HashMap<String, Vec<(String, String)>>,
    /// InfoTree results from the elaborator, keyed by URI.
    pub info_trees: HashMap<String, Vec<InfoTree>>,
}

impl LspServer {
    /// Create a new LSP server.
    pub fn new() -> Self {
        Self {
            document_store: DocumentStore::new(),
            env: Environment::new(),
            config: LspConfig::default(),
            cache: AnalysisCache::new(),
            initialized: false,
            shutdown_requested: false,
            doc_elaborations: HashMap::new(),
            info_trees: HashMap::new(),
        }
    }

    /// Create a new LSP server with custom config.
    pub fn with_config(config: LspConfig) -> Self {
        Self {
            document_store: DocumentStore::new(),
            env: Environment::new(),
            config,
            cache: AnalysisCache::new(),
            initialized: false,
            shutdown_requested: false,
            doc_elaborations: HashMap::new(),
            info_trees: HashMap::new(),
        }
    }

    /// Handle a JSON-RPC message and return a response (if any).
    pub fn handle_message(&mut self, msg: &JsonRpcMessage) -> Option<JsonRpcMessage> {
        let method = match &msg.method {
            Some(m) => m.clone(),
            None => {
                // This is a response, not a request; ignore it
                return None;
            }
        };
        let params = msg.params.clone().unwrap_or(JsonValue::Null);
        let id = msg.id.clone();

        match method.as_str() {
            "initialize" => {
                let result = self.handle_initialize(&params);
                id.map(|id_val| JsonRpcMessage::response(id_val, result))
            }
            "initialized" => {
                // Client confirms initialization, nothing to respond
                None
            }
            "shutdown" => {
                let result = self.handle_shutdown();
                id.map(|id_val| JsonRpcMessage::response(id_val, result))
            }
            "exit" => {
                // Server should exit
                None
            }
            "textDocument/didOpen" => {
                self.handle_text_document_did_open(&params);
                None
            }
            "textDocument/didChange" => {
                self.handle_text_document_did_change(&params);
                None
            }
            "textDocument/didClose" => {
                self.handle_text_document_did_close(&params);
                None
            }
            "textDocument/didSave" => {
                self.handle_text_document_did_save(&params);
                None
            }
            "textDocument/completion" => {
                let result = self.handle_completion(&params);
                id.map(|id_val| JsonRpcMessage::response(id_val, result))
            }
            "textDocument/hover" => {
                let result = self.handle_hover(&params);
                id.map(|id_val| JsonRpcMessage::response(id_val, result))
            }
            "textDocument/definition" => {
                let result = self.handle_goto_definition(&params);
                id.map(|id_val| JsonRpcMessage::response(id_val, result))
            }
            "textDocument/documentSymbol" => {
                let result = self.handle_document_symbol(&params);
                id.map(|id_val| JsonRpcMessage::response(id_val, result))
            }
            "textDocument/formatting" => {
                let result = self.handle_formatting(&params);
                id.map(|id_val| JsonRpcMessage::response(id_val, result))
            }
            "textDocument/signatureHelp" => {
                let result = self.handle_signature_help(&params);
                id.map(|id_val| JsonRpcMessage::response(id_val, result))
            }
            "textDocument/references" => {
                let result = self.handle_references(&params);
                id.map(|id_val| JsonRpcMessage::response(id_val, result))
            }
            "textDocument/codeAction" => {
                let result = self.handle_code_action(&params);
                id.map(|id_val| JsonRpcMessage::response(id_val, result))
            }
            _ => id.map(|id_val| {
                JsonRpcMessage::error_response(id_val, JsonRpcError::method_not_found(&method))
            }),
        }
    }

    /// Handle initialize request.
    pub fn handle_initialize(&mut self, _params: &JsonValue) -> JsonValue {
        self.initialized = true;
        let result = InitializeResult {
            capabilities: ServerCapabilities::oxilean_defaults(),
            server_name: "oxilean-lsp".to_string(),
            server_version: "0.1.2".to_string(),
        };
        result.to_json()
    }

    /// Handle shutdown request.
    pub fn handle_shutdown(&mut self) -> JsonValue {
        self.shutdown_requested = true;
        JsonValue::Null
    }

    /// Handle textDocument/didOpen.
    pub fn handle_text_document_did_open(&mut self, params: &JsonValue) {
        if let Some(td) = params.get("textDocument") {
            if let Ok(item) = TextDocumentItem::from_json(td) {
                self.document_store
                    .open_document(&item.uri, item.version, &item.text);
                self.cache.invalidate(&item.uri);
                let (decl_infos, trees, _diags) =
                    crate::lsp::analysis::elaborate_document(&item.text);
                self.doc_elaborations.insert(item.uri.clone(), decl_infos);
                self.info_trees.insert(item.uri.clone(), trees);
            }
        }
    }

    /// Handle textDocument/didChange.
    pub fn handle_text_document_did_change(&mut self, params: &JsonValue) {
        let uri = params
            .get("textDocument")
            .and_then(|td| td.get("uri"))
            .and_then(|v| v.as_str());
        let version = params
            .get("textDocument")
            .and_then(|td| td.get("version"))
            .and_then(|v| v.as_i64());

        if let (Some(uri), Some(version)) = (uri, version) {
            // Full sync: take the last content change
            if let Some(changes) = params.get("contentChanges").and_then(|v| v.as_array()) {
                if let Some(last_change) = changes.last() {
                    if let Some(text) = last_change.get("text").and_then(|v| v.as_str()) {
                        self.document_store.update_document(uri, version, text);
                        self.cache.invalidate(uri);
                        let (decl_infos, trees, _diags) =
                            crate::lsp::analysis::elaborate_document(text);
                        self.doc_elaborations.insert(uri.to_string(), decl_infos);
                        self.info_trees.insert(uri.to_string(), trees);
                    }
                }
            }
        }
    }

    /// Handle textDocument/didClose.
    pub fn handle_text_document_did_close(&mut self, params: &JsonValue) {
        if let Some(uri) = params
            .get("textDocument")
            .and_then(|td| td.get("uri"))
            .and_then(|v| v.as_str())
        {
            self.document_store.close_document(uri);
            self.cache.invalidate(uri);
        }
    }

    /// Handle textDocument/didSave.
    ///
    /// Re-elaborates the saved document so InfoTree results are up-to-date.
    pub fn handle_text_document_did_save(&mut self, params: &JsonValue) {
        let uri = params
            .get("textDocument")
            .and_then(|td| td.get("uri"))
            .and_then(|v| v.as_str());
        // The `text` field is optional in didSave (only present when textDocumentSync
        // includeText is true). Fall back to the stored document content.
        let text_opt = params.get("text").and_then(|v| v.as_str());
        if let Some(uri) = uri {
            let content = if let Some(t) = text_opt {
                Some(t.to_string())
            } else {
                self.document_store
                    .get_document(uri)
                    .map(|d| d.content.clone())
            };
            if let Some(content) = content {
                self.cache.invalidate(uri);
                let (decl_infos, trees, _diags) =
                    crate::lsp::analysis::elaborate_document(&content);
                self.doc_elaborations.insert(uri.to_string(), decl_infos);
                self.info_trees.insert(uri.to_string(), trees);
            }
        }
    }

    /// Handle textDocument/completion.
    pub fn handle_completion(&self, params: &JsonValue) -> JsonValue {
        let uri = params
            .get("textDocument")
            .and_then(|td| td.get("uri"))
            .and_then(|v| v.as_str());
        let position = params
            .get("position")
            .and_then(|p| Position::from_json(p).ok());

        if let (Some(uri), Some(pos)) = (uri, position) {
            let items = self.get_completions_at(uri, &pos);
            JsonValue::Array(items.iter().map(|c| c.to_json()).collect())
        } else {
            JsonValue::Array(Vec::new())
        }
    }

    /// Handle textDocument/hover.
    pub fn handle_hover(&self, params: &JsonValue) -> JsonValue {
        let uri = params
            .get("textDocument")
            .and_then(|td| td.get("uri"))
            .and_then(|v| v.as_str());
        let position = params
            .get("position")
            .and_then(|p| Position::from_json(p).ok());

        if let (Some(uri), Some(pos)) = (uri, position) {
            if let Some(hover) = self.get_hover_info_at(uri, &pos) {
                return hover.to_json();
            }
        }
        JsonValue::Null
    }

    /// Handle textDocument/definition.
    pub fn handle_goto_definition(&self, params: &JsonValue) -> JsonValue {
        let uri = params
            .get("textDocument")
            .and_then(|td| td.get("uri"))
            .and_then(|v| v.as_str());
        let position = params
            .get("position")
            .and_then(|p| Position::from_json(p).ok());

        if let (Some(uri), Some(pos)) = (uri, position) {
            if let Some(location) = self.find_definition(uri, &pos) {
                return location.to_json();
            }
        }
        JsonValue::Null
    }

    /// Handle textDocument/documentSymbol.
    pub fn handle_document_symbol(&self, params: &JsonValue) -> JsonValue {
        let uri = params
            .get("textDocument")
            .and_then(|td| td.get("uri"))
            .and_then(|v| v.as_str());

        if let Some(uri) = uri {
            if let Some(doc) = self.document_store.get_document(uri) {
                let analysis = analyze_document(uri, &doc.content, &self.env);
                return JsonValue::Array(analysis.symbols.iter().map(|s| s.to_json()).collect());
            }
        }
        JsonValue::Array(Vec::new())
    }

    /// Handle textDocument/formatting.
    pub fn handle_formatting(&self, params: &JsonValue) -> JsonValue {
        let uri = params
            .get("textDocument")
            .and_then(|td| td.get("uri"))
            .and_then(|v| v.as_str());

        if let Some(uri) = uri {
            if let Some(doc) = self.document_store.get_document(uri) {
                let edits = format_document(&doc.content);
                return JsonValue::Array(edits.iter().map(|e| e.to_json()).collect());
            }
        }
        JsonValue::Array(Vec::new())
    }

    /// Handle textDocument/signatureHelp.
    pub fn handle_signature_help(&self, params: &JsonValue) -> JsonValue {
        let uri = params
            .get("textDocument")
            .and_then(|td| td.get("uri"))
            .and_then(|v| v.as_str());
        let position = params
            .get("position")
            .and_then(|p| Position::from_json(p).ok());

        if let (Some(uri), Some(pos)) = (uri, position) {
            if let Some(help) = self.get_signature_help(uri, &pos) {
                return help.to_json();
            }
        }
        JsonValue::Null
    }

    /// Handle textDocument/references.
    pub fn handle_references(&self, params: &JsonValue) -> JsonValue {
        let uri = params
            .get("textDocument")
            .and_then(|td| td.get("uri"))
            .and_then(|v| v.as_str());
        let position = params
            .get("position")
            .and_then(|p| Position::from_json(p).ok());

        if let (Some(uri), Some(pos)) = (uri, position) {
            if let Some(doc) = self.document_store.get_document(uri) {
                if let Some((word, _)) = doc.word_at_position(&pos) {
                    // ── InfoTree-backed references ────────────────────────────
                    // Resolve the name at the cursor, then find all references
                    // using the info tree byte-range results.
                    if let Some(trees) = self.info_trees.get(uri) {
                        if let Some(byte_offset) =
                            position_to_offset(&doc.content, pos.line, pos.character)
                        {
                            // Determine the target Name (via find_definition on the
                            // info tree, falling back to the word as a Name).
                            let target_name: Name = trees
                                .iter()
                                .find_map(|tree| {
                                    oxilean_elab::info_tree::find_definition(tree, byte_offset)
                                })
                                .unwrap_or_else(|| Name::str(&word));

                            // Collect all reference byte-ranges across every tree.
                            let byte_ranges: Vec<(usize, usize)> = trees
                                .iter()
                                .flat_map(|tree| {
                                    oxilean_elab::info_tree::find_references(tree, &target_name)
                                })
                                .collect();

                            if !byte_ranges.is_empty() {
                                let locations: Vec<JsonValue> = byte_ranges
                                    .iter()
                                    .map(|(start, end)| {
                                        let start_pos = offset_to_position(&doc.content, *start);
                                        let end_pos = offset_to_position(&doc.content, *end);
                                        let r = Range {
                                            start: start_pos,
                                            end: end_pos,
                                        };
                                        Location::new(uri, r).to_json()
                                    })
                                    .collect();
                                return JsonValue::Array(locations);
                            }
                        }
                    }

                    // Fall back to token-level reference finding
                    let locations = find_references_in_document(uri, &doc.content, &word);
                    return JsonValue::Array(locations.iter().map(|l| l.to_json()).collect());
                }
            }
        }
        JsonValue::Array(Vec::new())
    }

    /// Handle textDocument/codeAction.
    pub fn handle_code_action(&self, params: &JsonValue) -> JsonValue {
        let uri = params
            .get("textDocument")
            .and_then(|td| td.get("uri"))
            .and_then(|v| v.as_str());

        if let Some(uri) = uri {
            if let Some(doc) = self.document_store.get_document(uri) {
                let actions = self.get_code_actions(uri, &doc.content, params);
                return JsonValue::Array(actions);
            }
        }
        JsonValue::Array(Vec::new())
    }

    /// Validate a document and return diagnostics.
    pub fn validate_document(&self, uri: &str) -> Vec<super::lsp_types::Diagnostic> {
        if let Some(doc) = self.document_store.get_document(uri) {
            let analysis = analyze_document(uri, &doc.content, &self.env);
            analysis.diagnostics
        } else {
            Vec::new()
        }
    }

    /// Get completions at a position.
    pub fn get_completions_at(&self, uri: &str, pos: &Position) -> Vec<CompletionItem> {
        let mut items = Vec::new();

        // Get the document and try to determine context
        let prefix = if let Some(doc) = self.document_store.get_document(uri) {
            if let Some(line) = doc.get_line(pos.line) {
                let col = (pos.character as usize).min(line.len());
                // Walk backwards to find the prefix
                let mut start = col;
                while start > 0 && is_ident_char(line.as_bytes()[start - 1]) {
                    start -= 1;
                }
                line[start..col].to_string()
            } else {
                String::new()
            }
        } else {
            String::new()
        };

        // Add keyword completions
        let keywords = [
            "def",
            "theorem",
            "lemma",
            "axiom",
            "inductive",
            "structure",
            "class",
            "instance",
            "namespace",
            "section",
            "end",
            "variable",
            "open",
            "import",
            "export",
            "where",
            "let",
            "in",
            "fun",
            "forall",
            "match",
            "with",
            "if",
            "then",
            "else",
            "do",
            "have",
            "show",
            "by",
            "Prop",
            "Type",
            "Sort",
        ];
        for kw in &keywords {
            if prefix.is_empty() || kw.starts_with(&prefix) {
                items.push(CompletionItem::keyword(kw));
            }
        }

        // Add environment completions
        for name in self.env.constant_names() {
            let name_str = name.to_string();
            if prefix.is_empty() || name_str.starts_with(&prefix) {
                let detail = if self.env.is_inductive(name) {
                    "inductive type"
                } else if self.env.is_constructor(name) {
                    "constructor"
                } else {
                    "declaration"
                };
                items.push(CompletionItem::function(&name_str, detail));
            }
        }

        // Add snippet completions
        if prefix.is_empty() || "def".starts_with(&prefix) {
            items.push(CompletionItem::snippet(
                "def ...",
                "def ${1:name} : ${2:Type} := ${0:sorry}",
                "definition template",
            ));
        }
        if prefix.is_empty() || "theorem".starts_with(&prefix) {
            items.push(CompletionItem::snippet(
                "theorem ...",
                "theorem ${1:name} : ${2:Prop} := by\n  ${0:sorry}",
                "theorem template",
            ));
        }
        if prefix.is_empty() || "inductive".starts_with(&prefix) {
            items.push(CompletionItem::snippet(
                "inductive ...",
                "inductive ${1:Name} where\n  | ${0:ctor}",
                "inductive type template",
            ));
        }

        items
    }

    /// Get hover information at a position.
    pub fn get_hover_info_at(&self, uri: &str, pos: &Position) -> Option<Hover> {
        let doc = self.document_store.get_document(uri)?;
        let (word, range) = doc.word_at_position(pos)?;

        // ── InfoTree-backed hover (richest source of type info) ──────────────
        if let Some(trees) = self.info_trees.get(uri) {
            if let Some(byte_offset) = position_to_offset(&doc.content, pos.line, pos.character) {
                for tree in trees {
                    if let Some(hi) = oxilean_elab::info_tree::hover_info(tree, byte_offset) {
                        let md = hi.to_markdown();
                        if !md.trim().is_empty() {
                            return Some(Hover::new(MarkupContent::markdown(md), Some(range)));
                        }
                    }
                }
            }
        }

        // Check if it's a keyword
        let keyword_info = get_keyword_hover(&word);
        if let Some(info) = keyword_info {
            return Some(Hover::new(MarkupContent::markdown(info), Some(range)));
        }

        // Check environment (kernel-level declarations)
        let name = Name::str(&word);
        if let Some(ci) = self.env.find(&name) {
            let ty_str = format!("{:?}", ci.ty());
            let kind = if ci.is_inductive() {
                "inductive"
            } else if ci.is_constructor() {
                "constructor"
            } else if ci.is_axiom() {
                "axiom"
            } else {
                "definition"
            };
            let md = format!("```lean\n{} {} : {}\n```", kind, word, ty_str);
            return Some(Hover::new(MarkupContent::markdown(md), Some(range)));
        }

        // Try elaboration-backed hover (real type information from the elaborator)
        if let Some(decl_infos) = self.doc_elaborations.get(uri) {
            if let Some((_, ty)) = decl_infos.iter().find(|(n, _)| n == &word) {
                let md = format!("```lean\n{} : {}\n```", word, ty);
                return Some(Hover::new(MarkupContent::markdown(md), Some(range)));
            }
        }

        // Fall back to token-level analysis
        let analysis = analyze_document(uri, &doc.content, &self.env);
        for def in &analysis.definitions {
            if def.name == word {
                let kind_str = match def.kind {
                    SymbolKind::Function => "def",
                    SymbolKind::Method => "theorem",
                    SymbolKind::Constant => "axiom",
                    SymbolKind::Enum => "inductive",
                    SymbolKind::Struct => "structure",
                    SymbolKind::Class => "class",
                    _ => "declaration",
                };
                let md = format!("```lean\n{} {}\n```", kind_str, word);
                return Some(Hover::new(MarkupContent::markdown(md), Some(range)));
            }
        }

        None
    }

    /// Find the definition location of a name.
    pub fn find_definition(&self, uri: &str, pos: &Position) -> Option<Location> {
        let doc = self.document_store.get_document(uri)?;
        let (word, _) = doc.word_at_position(pos)?;

        // ── InfoTree-backed go-to-definition ─────────────────────────────────
        // Resolve the kernel Name via the info tree, then locate it in analysis.
        let resolved_name: Option<Name> = self.info_trees.get(uri).and_then(|trees| {
            let byte_offset = position_to_offset(&doc.content, pos.line, pos.character)?;
            trees
                .iter()
                .find_map(|tree| oxilean_elab::info_tree::find_definition(tree, byte_offset))
        });

        // Determine which name string to search for in analysis.definitions.
        let search_word: String = if let Some(ref kname) = resolved_name {
            format!("{}", kname)
        } else {
            word.clone()
        };

        // Search in the current document's analysis
        let analysis = analyze_document(uri, &doc.content, &self.env);
        for def in &analysis.definitions {
            if def.name == search_word || def.name == word {
                return Some(Location::new(uri, def.range.clone()));
            }
        }

        None
    }

    /// Get signature help at a position.
    fn get_signature_help(&self, uri: &str, pos: &Position) -> Option<SignatureHelp> {
        let doc = self.document_store.get_document(uri)?;
        let line = doc.get_line(pos.line)?;
        let col = (pos.character as usize).min(line.len());

        // Walk backwards to find function name before '('
        let before = &line[..col];
        let paren_pos = before.rfind('(')?;
        let name_end = paren_pos;
        let mut name_start = name_end;
        while name_start > 0 && is_ident_char(before.as_bytes()[name_start - 1]) {
            name_start -= 1;
        }
        if name_start == name_end {
            return None;
        }
        let func_name = &before[name_start..name_end];

        // Count commas to determine active parameter
        let after_paren = &before[paren_pos + 1..];
        let active_param = after_paren.chars().filter(|c| *c == ',').count() as u32;

        // Look up in environment
        let name = Name::str(func_name);
        if self.env.find(&name).is_some() {
            return Some(SignatureHelp {
                signatures: vec![SignatureInformation {
                    label: format!("{} (...)", func_name),
                    documentation: None,
                    parameters: vec![ParameterInformation {
                        label: "arg".to_string(),
                        documentation: None,
                    }],
                }],
                active_signature: 0,
                active_parameter: active_param,
            });
        }

        // Return a placeholder if the function is locally defined
        let analysis = analyze_document(uri, &doc.content, &self.env);
        for def in &analysis.definitions {
            if def.name == func_name {
                return Some(SignatureHelp {
                    signatures: vec![SignatureInformation {
                        label: format!("{} (...)", func_name),
                        documentation: None,
                        parameters: vec![],
                    }],
                    active_signature: 0,
                    active_parameter: active_param,
                });
            }
        }

        None
    }

    /// Get code actions for a context.
    fn get_code_actions(&self, uri: &str, content: &str, params: &JsonValue) -> Vec<JsonValue> {
        let mut actions = Vec::new();
        let diagnostics = params
            .get("context")
            .and_then(|c| c.get("diagnostics"))
            .and_then(|d| d.as_array());

        if let Some(diags) = diagnostics {
            for diag in diags {
                if let Some(msg) = diag.get("message").and_then(|m| m.as_str()) {
                    // Offer quick fixes based on diagnostic messages
                    if msg.contains("shadows existing declaration") {
                        actions.push(make_code_action(
                            "Rename to avoid shadowing",
                            uri,
                            Vec::new(),
                        ));
                    }
                }
            }
        }

        // Offer "add sorry" if there's an incomplete proof region
        let _ = content; // Used above conceptually; prevent unused warning
        actions
    }
}

impl Default for LspServer {
    fn default() -> Self {
        Self::new()
    }
}
