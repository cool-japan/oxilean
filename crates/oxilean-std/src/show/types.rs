//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Expr, Level, Literal, Name};

use super::functions::*;
use std::fmt;

/// A diagnostic display wrapper for error messages.
#[allow(dead_code)]
pub struct DiagnosticDisplay {
    /// The error message.
    pub message: String,
    /// Optional source location hint.
    pub location: Option<String>,
    /// Severity level: 0 = note, 1 = warning, 2 = error.
    pub severity: u8,
    /// Optional code snippet.
    pub snippet: Option<String>,
}
impl DiagnosticDisplay {
    /// Create a new diagnostic.
    pub fn new(message: impl Into<String>, severity: u8) -> Self {
        DiagnosticDisplay {
            message: message.into(),
            location: None,
            severity,
            snippet: None,
        }
    }
    /// Add a source location.
    pub fn with_location(mut self, loc: impl Into<String>) -> Self {
        self.location = Some(loc.into());
        self
    }
    /// Add a code snippet.
    pub fn with_snippet(mut self, snip: impl Into<String>) -> Self {
        self.snippet = Some(snip.into());
        self
    }
    /// Render the diagnostic to a string.
    pub fn render(&self) -> String {
        let prefix = match self.severity {
            0 => "note",
            1 => "warning",
            _ => "error",
        };
        let loc = self
            .location
            .as_deref()
            .map(|l| format!(" [{}]", l))
            .unwrap_or_default();
        let snip = self
            .snippet
            .as_deref()
            .map(|s| format!("\n  | {}", s))
            .unwrap_or_default();
        format!("{}{}: {}{}", prefix, loc, self.message, snip)
    }
}
/// Selects a high-level display mode for show operations.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ShowMode {
    /// Short single-line output.
    Short,
    /// Full multi-line output.
    Full,
    /// Debug / internal representation.
    Debug,
}
impl ShowMode {
    /// Convert to a `ShowConfig`.
    pub fn to_config(self) -> ShowConfig {
        match self {
            ShowMode::Short => ShowConfig::compact(),
            ShowMode::Full => ShowConfig::wide(),
            ShowMode::Debug => ShowConfig::default().with_implicit().with_levels(),
        }
    }
}
/// Registry of named show formatters.
#[allow(clippy::type_complexity)]
pub struct ShowRegistry {
    formatters: Vec<(String, Box<dyn Fn(&Expr) -> String + Send + Sync>)>,
}
impl ShowRegistry {
    /// Create an empty registry.
    pub fn new() -> Self {
        ShowRegistry {
            formatters: Vec::new(),
        }
    }
    /// Register a formatter under a name.
    pub fn register<F>(&mut self, name: &str, f: F)
    where
        F: Fn(&Expr) -> String + Send + Sync + 'static,
    {
        self.formatters.push((name.to_string(), Box::new(f)));
    }
    /// Format an expression with all registered formatters, returning a map.
    pub fn format_all(&self, expr: &Expr) -> Vec<(String, String)> {
        self.formatters
            .iter()
            .map(|(n, f)| (n.clone(), f(expr)))
            .collect()
    }
    /// Number of registered formatters.
    pub fn len(&self) -> usize {
        self.formatters.len()
    }
    /// Whether the registry is empty.
    pub fn is_empty(&self) -> bool {
        self.formatters.is_empty()
    }
}
/// A formatted output value with metadata about how it was produced.
#[allow(dead_code)]
pub struct FormattedOutput {
    /// The rendered string.
    pub rendered: String,
    /// The name of the formatter that produced it.
    pub formatter: String,
    /// Whether the output was truncated.
    pub truncated: bool,
    /// Character count.
    pub char_count: usize,
}
impl FormattedOutput {
    /// Create a new `FormattedOutput`.
    pub fn new(rendered: impl Into<String>, formatter: impl Into<String>, truncated: bool) -> Self {
        let r = rendered.into();
        let cc = r.chars().count();
        FormattedOutput {
            rendered: r,
            formatter: formatter.into(),
            truncated,
            char_count: cc,
        }
    }
    /// Create from a show call.
    pub fn from_show<T: Show>(
        value: &T,
        formatter: impl Into<String>,
        max_len: Option<usize>,
    ) -> Self {
        let s = value.show();
        let (rendered, truncated) = match max_len {
            Some(n) if s.chars().count() > n => (truncate_show(&s, n), true),
            _ => (s, false),
        };
        FormattedOutput::new(rendered, formatter, truncated)
    }
}
/// A difference-string (ShowS) representation: a function `String → String`
/// encoded as a closure stored with its description.
#[allow(dead_code)]
pub struct ShowS {
    /// Human-readable label for this ShowS value.
    pub label: String,
    /// The accumulated prefix string.
    pub prefix: String,
}
impl ShowS {
    /// Create a new ShowS from a prefix string.
    pub fn new(label: impl Into<String>, prefix: impl Into<String>) -> Self {
        ShowS {
            label: label.into(),
            prefix: prefix.into(),
        }
    }
    /// Apply this ShowS: prepend the prefix to `rest`.
    pub fn apply(&self, rest: &str) -> String {
        format!("{}{}", self.prefix, rest)
    }
    /// Compose two ShowS values: `self` then `other`.
    pub fn compose(&self, other: &ShowS) -> ShowS {
        ShowS {
            label: format!("{}.{}", self.label, other.label),
            prefix: format!("{}{}", self.prefix, other.prefix),
        }
    }
    /// The identity ShowS (no prefix).
    pub fn identity() -> ShowS {
        ShowS {
            label: "id".to_string(),
            prefix: String::new(),
        }
    }
}
/// Configuration controlling how terms are shown.
#[derive(Clone, Debug)]
pub struct ShowConfig {
    /// Use compact (single-line) output.
    pub compact: bool,
    /// Restrict output to ASCII characters.
    pub ascii_only: bool,
    /// Maximum display depth (None = unlimited).
    pub max_depth: Option<usize>,
    /// Whether to show implicit arguments.
    pub show_implicit: bool,
    /// Whether to show universe levels.
    pub show_levels: bool,
    /// Whether to show binder types.
    pub show_binder_types: bool,
    /// Indentation step in spaces.
    pub indent_step: usize,
}
impl ShowConfig {
    /// Create a compact (single-line) config.
    pub fn compact() -> Self {
        Self {
            compact: true,
            ..Self::default()
        }
    }
    /// Create a wide (multi-line) config.
    pub fn wide() -> Self {
        Self {
            compact: false,
            ..Self::default()
        }
    }
    /// Create an ASCII-only config.
    pub fn ascii() -> Self {
        Self {
            ascii_only: true,
            ..Self::default()
        }
    }
    /// Set a depth limit.
    pub fn with_depth(self, d: usize) -> Self {
        Self {
            max_depth: Some(d),
            ..self
        }
    }
    /// Remove the depth limit.
    pub fn unlimited(self) -> Self {
        Self {
            max_depth: None,
            ..self
        }
    }
    /// Show implicit arguments.
    pub fn with_implicit(self) -> Self {
        Self {
            show_implicit: true,
            ..self
        }
    }
    /// Show universe levels.
    pub fn with_levels(self) -> Self {
        Self {
            show_levels: true,
            ..self
        }
    }
    /// Arrow character (Unicode or ASCII).
    pub fn arrow(&self) -> &'static str {
        if self.ascii_only {
            "->"
        } else {
            "→"
        }
    }
    /// Lambda character.
    pub fn lambda(&self) -> &'static str {
        "fun"
    }
    /// Pi/forall character.
    pub fn forall_kw(&self) -> &'static str {
        if self.ascii_only {
            "forall"
        } else {
            "∀"
        }
    }
}
/// Statistics collected during show operations (for debugging).
#[derive(Clone, Debug, Default)]
pub struct ShowStats {
    /// Number of expressions rendered.
    pub exprs_shown: u64,
    /// Number of depth-limit truncations.
    pub depth_truncations: u64,
    /// Total characters produced.
    pub chars_produced: u64,
}
impl ShowStats {
    /// Create zeroed stats.
    pub fn new() -> Self {
        Self::default()
    }
    /// Record a show operation.
    pub fn record(&mut self, output_len: usize, truncated: bool) {
        self.exprs_shown += 1;
        self.chars_produced += output_len as u64;
        if truncated {
            self.depth_truncations += 1;
        }
    }
}
/// Wraps any `T: Show` to implement `std::fmt::Display`.
pub struct Showable<T: Show>(pub T);
/// A simple algebraic document type for pretty printing.
#[allow(dead_code)]
pub struct PrettyDoc {
    /// The flattened text representation.
    pub text: String,
    /// The preferred line width.
    pub width: usize,
    /// The indentation level.
    pub indent: usize,
}
impl PrettyDoc {
    /// Create a text leaf document.
    pub fn text(s: impl Into<String>) -> Self {
        let t = s.into();
        let w = t.len();
        PrettyDoc {
            text: t,
            width: w,
            indent: 0,
        }
    }
    /// Concatenate two documents horizontally.
    pub fn concat(a: &PrettyDoc, b: &PrettyDoc) -> PrettyDoc {
        PrettyDoc {
            text: format!("{}{}", a.text, b.text),
            width: a.width + b.width,
            indent: a.indent,
        }
    }
    /// Nest a document by increasing its indent.
    pub fn nest(d: &PrettyDoc, extra: usize) -> PrettyDoc {
        PrettyDoc {
            text: d.text.clone(),
            width: d.width,
            indent: d.indent + extra,
        }
    }
    /// Render the document to a string.
    pub fn render(&self) -> String {
        self.text.clone()
    }
}
/// Registry extension for Show instances over a type parameter.
#[allow(dead_code)]
pub struct ShowRegistryExt<T> {
    /// Name of the type this registry extends.
    pub type_name: String,
    /// Phantom to hold `T`.
    _phantom: std::marker::PhantomData<T>,
    /// Registered formatter names.
    pub formatter_names: Vec<String>,
}
impl<T> ShowRegistryExt<T> {
    /// Create a new extension registry.
    pub fn new(type_name: impl Into<String>) -> Self {
        ShowRegistryExt {
            type_name: type_name.into(),
            _phantom: std::marker::PhantomData,
            formatter_names: Vec::new(),
        }
    }
    /// Register a named formatter.
    pub fn register_name(&mut self, name: impl Into<String>) {
        self.formatter_names.push(name.into());
    }
    /// Number of registered formatters.
    pub fn count(&self) -> usize {
        self.formatter_names.len()
    }
}
/// A string buffer with indentation support.
#[derive(Clone, Debug, Default)]
pub struct ShowBuffer {
    /// Accumulated output.
    pub(super) buf: String,
    /// Current indentation level.
    pub(super) indent: usize,
    /// Indentation step (spaces per level).
    pub(super) step: usize,
}
impl ShowBuffer {
    /// Create a new empty buffer.
    pub fn new(step: usize) -> Self {
        Self {
            buf: String::new(),
            indent: 0,
            step,
        }
    }
    /// Push a string directly.
    pub fn push(&mut self, s: &str) {
        self.buf.push_str(s);
    }
    /// Push a character.
    pub fn push_char(&mut self, c: char) {
        self.buf.push(c);
    }
    /// Push a newline followed by current indentation.
    pub fn newline(&mut self) {
        self.buf.push('\n');
        for _ in 0..self.indent * self.step {
            self.buf.push(' ');
        }
    }
    /// Increase indentation.
    pub fn indent(&mut self) {
        self.indent += 1;
    }
    /// Decrease indentation (saturating).
    pub fn dedent(&mut self) {
        self.indent = self.indent.saturating_sub(1);
    }
    /// Consume the buffer and return the accumulated string.
    pub fn finish(self) -> String {
        self.buf
    }
    /// Current accumulated length in bytes.
    pub fn len(&self) -> usize {
        self.buf.len()
    }
    /// Whether the buffer is empty.
    pub fn is_empty(&self) -> bool {
        self.buf.is_empty()
    }
    /// Push an indented block: indent, run f, dedent.
    pub fn with_indent<F: FnOnce(&mut ShowBuffer)>(&mut self, f: F) {
        self.indent();
        f(self);
        self.dedent();
    }
}
