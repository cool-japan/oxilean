//! Types for parser error recovery strategies.

use std::fmt;

/// Strategy the parser uses to recover from a parse error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecoveryStrategy {
    /// Skip tokens until one of the specified sync tokens is encountered.
    SkipToSync {
        /// Tokens that serve as synchronisation points.
        sync_tokens: Vec<String>,
    },
    /// Insert a synthetic token and continue parsing.
    InsertToken {
        /// The token text to insert.
        token: String,
    },
    /// Delete the current (unexpected) token and retry.
    DeleteToken,
    /// Replace the current token with a corrected version.
    ReplaceToken {
        /// Replacement token text.
        with: String,
    },
    /// Panic-mode recovery: abort the current production entirely.
    Panic,
}

/// A location where the parser successfully recovered from an error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecoveryPoint {
    /// Byte offset in the source where recovery occurred.
    pub offset: usize,
    /// Textual description of the parsing context at the time of recovery.
    pub context: String,
    /// The recovery strategy that was applied.
    pub strategy: RecoveryStrategy,
}

/// Severity level of a parse error.
///
/// Variants are ordered from least to most severe so that `Info < Warning < Error < Fatal`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ErrorSeverity {
    /// Informational hint; no correctness impact.
    Info,
    /// Suspicious code that may work but is likely wrong.
    Warning,
    /// Regular error that can be recovered from.
    Error,
    /// Unrecoverable error; parsing cannot continue.
    Fatal,
}

impl fmt::Display for ErrorSeverity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorSeverity::Fatal => write!(f, "fatal"),
            ErrorSeverity::Error => write!(f, "error"),
            ErrorSeverity::Warning => write!(f, "warning"),
            ErrorSeverity::Info => write!(f, "info"),
        }
    }
}

/// A single parse error with location and severity information.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseError {
    /// Human-readable error message.
    pub message: String,
    /// Byte span `(start, end)` in the source string.
    pub span: (usize, usize),
    /// Severity of this error.
    pub severity: ErrorSeverity,
    /// Optional error code such as `"E0001"`.
    pub code: Option<String>,
}

impl ParseError {
    /// Create a new error with the given message and span.
    pub fn new(message: impl Into<String>, span: (usize, usize)) -> Self {
        Self {
            message: message.into(),
            span,
            severity: ErrorSeverity::Error,
            code: None,
        }
    }

    /// Attach a severity level.
    pub fn with_severity(mut self, severity: ErrorSeverity) -> Self {
        self.severity = severity;
        self
    }

    /// Attach an error code.
    pub fn with_code(mut self, code: impl Into<String>) -> Self {
        self.code = Some(code.into());
        self
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(ref code) = self.code {
            write!(
                f,
                "[{}] {}: {} at {:?}",
                code, self.severity, self.message, self.span
            )
        } else {
            write!(f, "{}: {} at {:?}", self.severity, self.message, self.span)
        }
    }
}

/// The result of an error-recovery pass over a source string.
#[derive(Debug, Clone)]
pub struct RecoveryResult {
    /// All parse errors detected.
    pub errors: Vec<ParseError>,
    /// Points at which the parser recovered.
    pub recovery_points: Vec<RecoveryPoint>,
    /// Source text after any automatic corrections have been applied.
    pub recovered_source: String,
}

impl RecoveryResult {
    /// Create a result with no errors and the original source.
    pub fn clean(source: impl Into<String>) -> Self {
        Self {
            errors: Vec::new(),
            recovery_points: Vec::new(),
            recovered_source: source.into(),
        }
    }

    /// Returns `true` when the recovery result contains no errors.
    pub fn is_ok(&self) -> bool {
        self.errors.is_empty()
    }

    /// Returns `true` when recovery produced at least one fatal error.
    pub fn has_fatal(&self) -> bool {
        self.errors
            .iter()
            .any(|e| e.severity == ErrorSeverity::Fatal)
    }
}

/// Maps error message substrings to canonical error codes.
///
/// The classifier is built from `(pattern, code)` pairs and applies them in
/// insertion order via substring search.
#[derive(Debug, Clone, Default)]
pub struct ErrorClassifier {
    /// Ordered list of `(pattern, error_code)` pairs.
    pub patterns: Vec<(String, String)>,
}
