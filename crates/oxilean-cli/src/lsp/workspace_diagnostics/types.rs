//! Workspace diagnostics manager types.
//!
//! Tracks per-file diagnostics across the workspace and supports
//! bulk publishing via `textDocument/publishDiagnostics` notifications.

use crate::lsp::{Diagnostic, JsonRpcMessage, JsonValue, PublishDiagnosticsParams};
use std::collections::HashMap;

/// Manages diagnostics for all open files in the workspace.
///
/// Stores the latest set of diagnostics per URI and provides
/// methods to update, clear, and retrieve them. Also generates
/// `textDocument/publishDiagnostics` notifications for each file.
#[derive(Debug, Default)]
pub struct WorkspaceDiagnosticsManager {
    /// Map from document URI to its current diagnostics.
    diagnostics: HashMap<String, Vec<Diagnostic>>,
}

impl WorkspaceDiagnosticsManager {
    /// Create a new, empty manager.
    pub fn new() -> Self {
        Self {
            diagnostics: HashMap::new(),
        }
    }

    /// Update (replace) the diagnostics for a given URI.
    ///
    /// Any previously stored diagnostics for this URI are discarded.
    pub fn update_file(&mut self, uri: &str, diagnostics: Vec<Diagnostic>) {
        self.diagnostics.insert(uri.to_string(), diagnostics);
    }

    /// Return all diagnostics across all tracked files.
    pub fn get_all_diagnostics(&self) -> HashMap<String, Vec<Diagnostic>> {
        self.diagnostics.clone()
    }

    /// Return the diagnostics for a specific URI, or an empty slice.
    pub fn get_file_diagnostics(&self, uri: &str) -> &[Diagnostic] {
        self.diagnostics
            .get(uri)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }

    /// Remove all diagnostics for a given URI.
    ///
    /// After this call `get_file_diagnostics(uri)` returns an empty slice.
    pub fn clear_file(&mut self, uri: &str) {
        self.diagnostics.remove(uri);
    }

    /// Clear diagnostics for all files.
    pub fn clear_all(&mut self) {
        self.diagnostics.clear();
    }

    /// Return the number of URIs that have at least one diagnostic.
    pub fn file_count(&self) -> usize {
        self.diagnostics.len()
    }

    /// Return the total number of diagnostics across all files.
    pub fn total_diagnostic_count(&self) -> usize {
        self.diagnostics.values().map(|v| v.len()).sum()
    }

    /// Build a `textDocument/publishDiagnostics` JSON-RPC notification
    /// for the given URI with its current diagnostics.
    ///
    /// Returns `None` if the URI is not tracked.
    pub fn build_publish_notification(&self, uri: &str) -> Option<JsonRpcMessage> {
        let diags = self.diagnostics.get(uri)?;
        let params = PublishDiagnosticsParams {
            uri: uri.to_string(),
            diagnostics: diags.clone(),
            version: None,
        };
        Some(JsonRpcMessage::notification(
            "textDocument/publishDiagnostics",
            params.to_json(),
        ))
    }

    /// Build publish notifications for every tracked URI.
    ///
    /// This includes URIs that currently have zero diagnostics so that
    /// the client can clear stale markers.
    pub fn build_all_publish_notifications(&self) -> Vec<JsonRpcMessage> {
        let mut notifications = Vec::with_capacity(self.diagnostics.len());
        for (uri, diags) in &self.diagnostics {
            let params = PublishDiagnosticsParams {
                uri: uri.clone(),
                diagnostics: diags.clone(),
                version: None,
            };
            notifications.push(JsonRpcMessage::notification(
                "textDocument/publishDiagnostics",
                params.to_json(),
            ));
        }
        notifications
    }

    /// Build a cleared (empty-diagnostics) publish notification for `uri`.
    ///
    /// Useful when a document is closed and stale diagnostics should be
    /// removed from the client.
    pub fn build_clear_notification(uri: &str) -> JsonRpcMessage {
        let params = PublishDiagnosticsParams {
            uri: uri.to_string(),
            diagnostics: Vec::new(),
            version: None,
        };
        JsonRpcMessage::notification("textDocument/publishDiagnostics", params.to_json())
    }

    /// Merge additional diagnostics into a file's existing set.
    ///
    /// New diagnostics are appended; existing ones are preserved.
    pub fn merge_file(&mut self, uri: &str, additional: Vec<Diagnostic>) {
        let entry = self.diagnostics.entry(uri.to_string()).or_default();
        entry.extend(additional);
    }

    /// Return a sorted, deduplicated list of all tracked URIs.
    pub fn tracked_uris(&self) -> Vec<String> {
        let mut uris: Vec<String> = self.diagnostics.keys().cloned().collect();
        uris.sort();
        uris
    }
}

/// Snapshot of workspace diagnostics at a point in time.
#[derive(Debug, Clone)]
pub struct WorkspaceDiagnosticsSnapshot {
    /// Per-file diagnostics captured at snapshot time.
    pub files: HashMap<String, Vec<Diagnostic>>,
}

impl WorkspaceDiagnosticsSnapshot {
    /// Create a snapshot from the current manager state.
    pub fn from_manager(manager: &WorkspaceDiagnosticsManager) -> Self {
        Self {
            files: manager.get_all_diagnostics(),
        }
    }

    /// Total number of diagnostics across all files.
    pub fn total(&self) -> usize {
        self.files.values().map(|v| v.len()).sum()
    }

    /// Number of files with at least one diagnostic.
    pub fn file_count(&self) -> usize {
        self.files.len()
    }

    /// Serialize the snapshot to a JSON array of per-file objects.
    pub fn to_json(&self) -> JsonValue {
        let entries: Vec<JsonValue> = self
            .files
            .iter()
            .map(|(uri, diags)| {
                let diag_json: Vec<JsonValue> = diags.iter().map(|d| d.to_json()).collect();
                JsonValue::Object(vec![
                    ("uri".to_string(), JsonValue::String(uri.clone())),
                    ("diagnostics".to_string(), JsonValue::Array(diag_json)),
                ])
            })
            .collect();
        JsonValue::Array(entries)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lsp::{DiagnosticSeverity, Range};

    fn make_diagnostic(msg: &str, sev: DiagnosticSeverity) -> Diagnostic {
        Diagnostic {
            range: Range::default(),
            severity: sev,
            message: msg.to_string(),
            source: Some("test".to_string()),
            code: None,
        }
    }

    #[test]
    fn test_update_and_get_file() {
        let mut mgr = WorkspaceDiagnosticsManager::new();
        let d = make_diagnostic("err", DiagnosticSeverity::Error);
        mgr.update_file("file:///a.lean", vec![d.clone()]);
        let got = mgr.get_file_diagnostics("file:///a.lean");
        assert_eq!(got.len(), 1);
        assert_eq!(got[0].message, "err");
    }

    #[test]
    fn test_update_replaces_existing() {
        let mut mgr = WorkspaceDiagnosticsManager::new();
        mgr.update_file(
            "file:///a.lean",
            vec![make_diagnostic("old", DiagnosticSeverity::Error)],
        );
        mgr.update_file(
            "file:///a.lean",
            vec![make_diagnostic("new", DiagnosticSeverity::Warning)],
        );
        let got = mgr.get_file_diagnostics("file:///a.lean");
        assert_eq!(got.len(), 1);
        assert_eq!(got[0].message, "new");
    }

    #[test]
    fn test_clear_file() {
        let mut mgr = WorkspaceDiagnosticsManager::new();
        mgr.update_file(
            "file:///a.lean",
            vec![make_diagnostic("x", DiagnosticSeverity::Error)],
        );
        mgr.clear_file("file:///a.lean");
        assert_eq!(mgr.get_file_diagnostics("file:///a.lean").len(), 0);
        assert_eq!(mgr.file_count(), 0);
    }

    #[test]
    fn test_get_all_diagnostics() {
        let mut mgr = WorkspaceDiagnosticsManager::new();
        mgr.update_file(
            "file:///a.lean",
            vec![make_diagnostic("a", DiagnosticSeverity::Error)],
        );
        mgr.update_file(
            "file:///b.lean",
            vec![make_diagnostic("b", DiagnosticSeverity::Warning)],
        );
        let all = mgr.get_all_diagnostics();
        assert_eq!(all.len(), 2);
        assert!(all.contains_key("file:///a.lean"));
        assert!(all.contains_key("file:///b.lean"));
    }

    #[test]
    fn test_total_diagnostic_count() {
        let mut mgr = WorkspaceDiagnosticsManager::new();
        mgr.update_file(
            "file:///a.lean",
            vec![
                make_diagnostic("x", DiagnosticSeverity::Error),
                make_diagnostic("y", DiagnosticSeverity::Warning),
            ],
        );
        mgr.update_file(
            "file:///b.lean",
            vec![make_diagnostic("z", DiagnosticSeverity::Hint)],
        );
        assert_eq!(mgr.total_diagnostic_count(), 3);
    }

    #[test]
    fn test_build_publish_notification() {
        let mut mgr = WorkspaceDiagnosticsManager::new();
        mgr.update_file(
            "file:///a.lean",
            vec![make_diagnostic("e", DiagnosticSeverity::Error)],
        );
        let notif = mgr.build_publish_notification("file:///a.lean");
        assert!(notif.is_some());
        let n = notif.unwrap();
        assert_eq!(n.method.as_deref(), Some("textDocument/publishDiagnostics"));
    }

    #[test]
    fn test_build_publish_notification_missing_uri() {
        let mgr = WorkspaceDiagnosticsManager::new();
        assert!(mgr
            .build_publish_notification("file:///missing.lean")
            .is_none());
    }

    #[test]
    fn test_build_all_publish_notifications() {
        let mut mgr = WorkspaceDiagnosticsManager::new();
        mgr.update_file("file:///a.lean", vec![]);
        mgr.update_file("file:///b.lean", vec![]);
        let notifs = mgr.build_all_publish_notifications();
        assert_eq!(notifs.len(), 2);
    }

    #[test]
    fn test_build_clear_notification() {
        let n = WorkspaceDiagnosticsManager::build_clear_notification("file:///a.lean");
        assert_eq!(n.method.as_deref(), Some("textDocument/publishDiagnostics"));
    }

    #[test]
    fn test_merge_file() {
        let mut mgr = WorkspaceDiagnosticsManager::new();
        mgr.update_file(
            "file:///a.lean",
            vec![make_diagnostic("first", DiagnosticSeverity::Error)],
        );
        mgr.merge_file(
            "file:///a.lean",
            vec![make_diagnostic("second", DiagnosticSeverity::Warning)],
        );
        assert_eq!(mgr.get_file_diagnostics("file:///a.lean").len(), 2);
    }

    #[test]
    fn test_tracked_uris_sorted() {
        let mut mgr = WorkspaceDiagnosticsManager::new();
        mgr.update_file("file:///z.lean", vec![]);
        mgr.update_file("file:///a.lean", vec![]);
        let uris = mgr.tracked_uris();
        assert_eq!(uris[0], "file:///a.lean");
        assert_eq!(uris[1], "file:///z.lean");
    }

    #[test]
    fn test_snapshot() {
        let mut mgr = WorkspaceDiagnosticsManager::new();
        mgr.update_file(
            "file:///a.lean",
            vec![make_diagnostic("e", DiagnosticSeverity::Error)],
        );
        let snap = WorkspaceDiagnosticsSnapshot::from_manager(&mgr);
        assert_eq!(snap.total(), 1);
        assert_eq!(snap.file_count(), 1);
    }
}
