//! LSP progress reporting types.
//!
//! Implements the `$/progress` notification mechanism defined in LSP 3.17.
//! Allows the server to report ongoing work (e.g., file indexing, type
//! checking) to the client with begin/report/end lifecycle messages.

use crate::lsp::{JsonRpcMessage, JsonValue};
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};

/// Global counter for generating unique progress token IDs.
static NEXT_TOKEN_ID: AtomicU64 = AtomicU64::new(1);

/// Generates a unique string token for a `$/progress` notification.
pub fn generate_progress_token(prefix: &str) -> String {
    let id = NEXT_TOKEN_ID.fetch_add(1, Ordering::Relaxed);
    format!("{}/{}", prefix, id)
}

/// State of a single progress token.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ProgressState {
    /// The work has been announced (`begin`) but no updates sent yet.
    Begun,
    /// Updates are being sent.
    Reporting,
    /// The work has finished (`end`).
    Ended,
}

/// Metadata recorded for each active progress token.
#[derive(Clone, Debug)]
pub struct ProgressEntry {
    /// The display title provided at `begin` time.
    pub title: String,
    /// Current lifecycle state.
    pub state: ProgressState,
    /// Most recent percentage (0–100), if reported.
    pub percentage: Option<u32>,
    /// Most recent message, if reported.
    pub message: Option<String>,
}

impl ProgressEntry {
    fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            state: ProgressState::Begun,
            percentage: None,
            message: None,
        }
    }
}

/// Sends `$/progress` notifications and manages token lifecycle.
///
/// # Example
///
/// ```rust
/// use oxilean_cli::lsp::progress::ProgressReporter;
///
/// let mut reporter = ProgressReporter::new();
/// let token = "my-task/1";
/// let notif = reporter.begin(token, "Checking workspace");
/// let notif2 = reporter.report(token, 50, "half done");
/// let notif3 = reporter.end(token);
/// ```
#[derive(Debug, Default)]
pub struct ProgressReporter {
    /// Active progress tokens.
    tokens: HashMap<String, ProgressEntry>,
}

impl ProgressReporter {
    /// Create a new reporter with no active tokens.
    pub fn new() -> Self {
        Self {
            tokens: HashMap::new(),
        }
    }

    /// Send a `$/progress` `begin` notification.
    ///
    /// Registers `token` as an active progress item with the given title.
    /// If the token is already active it is reset to the `Begun` state.
    pub fn begin(&mut self, token: &str, title: &str) -> JsonRpcMessage {
        self.tokens
            .insert(token.to_string(), ProgressEntry::new(title));
        build_begin_notification(token, title)
    }

    /// Send a `$/progress` `report` notification with a percentage and message.
    ///
    /// `pct` is clamped to 0–100.
    /// If `token` is unknown the notification is still emitted but the token
    /// is not tracked (callers should always call [`begin`](Self::begin) first).
    pub fn report(&mut self, token: &str, pct: u32, msg: &str) -> JsonRpcMessage {
        let clamped = pct.min(100);
        if let Some(entry) = self.tokens.get_mut(token) {
            entry.state = ProgressState::Reporting;
            entry.percentage = Some(clamped);
            entry.message = Some(msg.to_string());
        }
        build_report_notification(token, clamped, msg)
    }

    /// Send a `$/progress` `end` notification.
    ///
    /// Marks the token as ended and removes it from the active set.
    pub fn end(&mut self, token: &str) -> JsonRpcMessage {
        let msg = self
            .tokens
            .remove(token)
            .as_ref()
            .and_then(|e| e.message.clone());
        build_end_notification(token, msg.as_deref())
    }

    /// Cancel a progress token without sending an `end` notification.
    ///
    /// Use this when the operation was aborted unexpectedly.
    pub fn cancel(&mut self, token: &str) {
        self.tokens.remove(token);
    }

    /// Return `true` if `token` is currently active (between `begin` and `end`).
    pub fn is_active(&self, token: &str) -> bool {
        self.tokens.contains_key(token)
    }

    /// Return the current state of `token`, or `None` if unknown.
    pub fn state(&self, token: &str) -> Option<&ProgressState> {
        self.tokens.get(token).map(|e| &e.state)
    }

    /// Return the number of currently active tokens.
    pub fn active_count(&self) -> usize {
        self.tokens.len()
    }

    /// List all currently active token strings.
    pub fn active_tokens(&self) -> Vec<String> {
        let mut tokens: Vec<String> = self.tokens.keys().cloned().collect();
        tokens.sort();
        tokens
    }

    /// Create a new unique token and send a `begin` notification.
    ///
    /// The generated token string is returned alongside the notification.
    pub fn begin_new(&mut self, prefix: &str, title: &str) -> (String, JsonRpcMessage) {
        let token = generate_progress_token(prefix);
        let notif = self.begin(&token, title);
        (token, notif)
    }
}

// ── Internal notification builders ────────────────────────────────────────────

fn build_begin_notification(token: &str, title: &str) -> JsonRpcMessage {
    let value = JsonValue::Object(vec![
        ("kind".to_string(), JsonValue::String("begin".to_string())),
        ("title".to_string(), JsonValue::String(title.to_string())),
    ]);
    let params = JsonValue::Object(vec![
        ("token".to_string(), JsonValue::String(token.to_string())),
        ("value".to_string(), value),
    ]);
    JsonRpcMessage::notification("$/progress", params)
}

fn build_report_notification(token: &str, pct: u32, msg: &str) -> JsonRpcMessage {
    let value = JsonValue::Object(vec![
        ("kind".to_string(), JsonValue::String("report".to_string())),
        ("percentage".to_string(), JsonValue::Number(pct as f64)),
        ("message".to_string(), JsonValue::String(msg.to_string())),
    ]);
    let params = JsonValue::Object(vec![
        ("token".to_string(), JsonValue::String(token.to_string())),
        ("value".to_string(), value),
    ]);
    JsonRpcMessage::notification("$/progress", params)
}

fn build_end_notification(token: &str, message: Option<&str>) -> JsonRpcMessage {
    let mut value_fields = vec![("kind".to_string(), JsonValue::String("end".to_string()))];
    if let Some(m) = message {
        value_fields.push(("message".to_string(), JsonValue::String(m.to_string())));
    }
    let value = JsonValue::Object(value_fields);
    let params = JsonValue::Object(vec![
        ("token".to_string(), JsonValue::String(token.to_string())),
        ("value".to_string(), value),
    ]);
    JsonRpcMessage::notification("$/progress", params)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_kind(notif: &JsonRpcMessage) -> Option<String> {
        notif
            .params
            .as_ref()?
            .get("value")?
            .get("kind")?
            .as_str()
            .map(String::from)
    }

    fn get_token(notif: &JsonRpcMessage) -> Option<String> {
        notif
            .params
            .as_ref()?
            .get("token")?
            .as_str()
            .map(String::from)
    }

    fn get_pct(notif: &JsonRpcMessage) -> Option<i64> {
        notif
            .params
            .as_ref()?
            .get("value")?
            .get("percentage")?
            .as_i64()
    }

    #[test]
    fn test_begin_notification_method() {
        let mut r = ProgressReporter::new();
        let n = r.begin("tok/1", "Checking");
        assert_eq!(n.method.as_deref(), Some("$/progress"));
        assert_eq!(get_kind(&n).as_deref(), Some("begin"));
        assert_eq!(get_token(&n).as_deref(), Some("tok/1"));
    }

    #[test]
    fn test_report_notification() {
        let mut r = ProgressReporter::new();
        r.begin("tok/1", "t");
        let n = r.report("tok/1", 42, "almost there");
        assert_eq!(get_kind(&n).as_deref(), Some("report"));
        assert_eq!(get_pct(&n), Some(42));
    }

    #[test]
    fn test_report_clamps_to_100() {
        let mut r = ProgressReporter::new();
        r.begin("tok/1", "t");
        let n = r.report("tok/1", 200, "over");
        assert_eq!(get_pct(&n), Some(100));
        assert_eq!(r.state("tok/1"), Some(&ProgressState::Reporting));
    }

    #[test]
    fn test_end_notification() {
        let mut r = ProgressReporter::new();
        r.begin("tok/1", "t");
        r.report("tok/1", 50, "half");
        let n = r.end("tok/1");
        assert_eq!(get_kind(&n).as_deref(), Some("end"));
        assert!(!r.is_active("tok/1"));
    }

    #[test]
    fn test_is_active_lifecycle() {
        let mut r = ProgressReporter::new();
        assert!(!r.is_active("tok"));
        r.begin("tok", "t");
        assert!(r.is_active("tok"));
        r.end("tok");
        assert!(!r.is_active("tok"));
    }

    #[test]
    fn test_cancel_removes_token() {
        let mut r = ProgressReporter::new();
        r.begin("tok", "t");
        r.cancel("tok");
        assert!(!r.is_active("tok"));
    }

    #[test]
    fn test_active_count() {
        let mut r = ProgressReporter::new();
        r.begin("a", "A");
        r.begin("b", "B");
        assert_eq!(r.active_count(), 2);
        r.end("a");
        assert_eq!(r.active_count(), 1);
    }

    #[test]
    fn test_begin_new_returns_unique_tokens() {
        let mut r = ProgressReporter::new();
        let (tok1, _) = r.begin_new("task", "T1");
        let (tok2, _) = r.begin_new("task", "T2");
        assert_ne!(tok1, tok2);
        assert!(tok1.starts_with("task/"));
        assert!(tok2.starts_with("task/"));
    }

    #[test]
    fn test_generate_progress_token_unique() {
        let t1 = generate_progress_token("x");
        let t2 = generate_progress_token("x");
        assert_ne!(t1, t2);
    }

    #[test]
    fn test_state_tracking() {
        let mut r = ProgressReporter::new();
        r.begin("tok", "t");
        assert_eq!(r.state("tok"), Some(&ProgressState::Begun));
        r.report("tok", 30, "in progress");
        assert_eq!(r.state("tok"), Some(&ProgressState::Reporting));
        r.end("tok");
        assert_eq!(r.state("tok"), None);
    }

    #[test]
    fn test_active_tokens_sorted() {
        let mut r = ProgressReporter::new();
        r.begin("z", "Z");
        r.begin("a", "A");
        let tokens = r.active_tokens();
        assert_eq!(tokens[0], "a");
        assert_eq!(tokens[1], "z");
    }
}
