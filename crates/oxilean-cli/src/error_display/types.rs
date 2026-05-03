//! Types for structured, colorized diagnostic display.

/// ANSI terminal color / style selectors.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TermColor {
    Red,
    Green,
    Yellow,
    Blue,
    Cyan,
    White,
    Bold,
    Reset,
}

impl TermColor {
    /// Return the ANSI escape code for this color/style.
    pub fn ansi_code(self) -> &'static str {
        match self {
            TermColor::Red => "\x1b[31m",
            TermColor::Green => "\x1b[32m",
            TermColor::Yellow => "\x1b[33m",
            TermColor::Blue => "\x1b[34m",
            TermColor::Cyan => "\x1b[36m",
            TermColor::White => "\x1b[37m",
            TermColor::Bold => "\x1b[1m",
            TermColor::Reset => "\x1b[0m",
        }
    }
}

/// The kind of a diagnostic label, controlling how it is rendered.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LabelKind {
    /// The primary label — rendered with `^^^` underline.
    Primary,
    /// A secondary label — rendered with `---` underline.
    Secondary,
    /// An informational note — rendered with `...` underline.
    Note,
}

impl LabelKind {
    /// Return the underline character sequence for this kind.
    pub fn underline_char(self) -> char {
        match self {
            LabelKind::Primary => '^',
            LabelKind::Secondary => '-',
            LabelKind::Note => '.',
        }
    }

    /// Return the color associated with this label kind.
    pub fn color(self) -> TermColor {
        match self {
            LabelKind::Primary => TermColor::Red,
            LabelKind::Secondary => TermColor::Blue,
            LabelKind::Note => TermColor::Cyan,
        }
    }
}

/// A label pointing to a byte-span within source text.
#[derive(Debug, Clone)]
pub struct DiagnosticLabel {
    /// Byte span `(start, end)` (exclusive end) within the source string.
    pub span: (usize, usize),
    /// The message to display next to the underline.
    pub message: String,
    /// How this label should be styled.
    pub kind: LabelKind,
}

impl DiagnosticLabel {
    /// Create a new label.
    pub fn new(span: (usize, usize), message: impl Into<String>, kind: LabelKind) -> Self {
        Self {
            span,
            message: message.into(),
            kind,
        }
    }

    /// Shorthand for a primary label.
    pub fn primary(span: (usize, usize), message: impl Into<String>) -> Self {
        Self::new(span, message, LabelKind::Primary)
    }

    /// Shorthand for a secondary label.
    pub fn secondary(span: (usize, usize), message: impl Into<String>) -> Self {
        Self::new(span, message, LabelKind::Secondary)
    }

    /// Shorthand for a note label.
    pub fn note(span: (usize, usize), message: impl Into<String>) -> Self {
        Self::new(span, message, LabelKind::Note)
    }
}

/// Diagnostic severity level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Severity {
    /// A hint / suggestion (lowest priority).
    Hint,
    /// Informational message.
    Info,
    /// A warning that does not prevent success.
    Warning,
    /// A hard error (highest priority).
    Error,
}

impl Severity {
    /// Short label used in the diagnostic header.
    pub fn label(self) -> &'static str {
        match self {
            Severity::Error => "error",
            Severity::Warning => "warning",
            Severity::Info => "info",
            Severity::Hint => "hint",
        }
    }

    /// The terminal color associated with this severity.
    pub fn color(self) -> TermColor {
        match self {
            Severity::Error => TermColor::Red,
            Severity::Warning => TermColor::Yellow,
            Severity::Info => TermColor::Cyan,
            Severity::Hint => TermColor::Green,
        }
    }
}

/// A fully-structured diagnostic message, analogous to a rustc diagnostic.
#[derive(Debug, Clone)]
pub struct RichDiagnostic {
    /// Optional diagnostic code (e.g. `"E0001"`).
    pub code: Option<String>,
    /// Severity of this diagnostic.
    pub severity: Severity,
    /// Short one-line title.
    pub title: String,
    /// Span labels pointing into the source.
    pub labels: Vec<DiagnosticLabel>,
    /// Notes appended after the source view.
    pub notes: Vec<String>,
    /// Help messages suggesting fixes.
    pub help: Vec<String>,
    /// The source text that labels point into.
    pub source: Option<String>,
}

impl RichDiagnostic {
    /// Create a minimal diagnostic with a severity and title.
    pub fn new(severity: Severity, title: impl Into<String>) -> Self {
        Self {
            code: None,
            severity,
            title: title.into(),
            labels: Vec::new(),
            notes: Vec::new(),
            help: Vec::new(),
            source: None,
        }
    }

    /// Attach an error code.
    pub fn with_code(mut self, code: impl Into<String>) -> Self {
        self.code = Some(code.into());
        self
    }

    /// Attach source text.
    pub fn with_source(mut self, src: impl Into<String>) -> Self {
        self.source = Some(src.into());
        self
    }

    /// Add a diagnostic label.
    pub fn add_label(mut self, label: DiagnosticLabel) -> Self {
        self.labels.push(label);
        self
    }

    /// Add a note.
    pub fn add_note(mut self, note: impl Into<String>) -> Self {
        self.notes.push(note.into());
        self
    }

    /// Add a help message.
    pub fn add_help(mut self, help: impl Into<String>) -> Self {
        self.help.push(help.into());
        self
    }
}

/// Configuration for the diagnostic renderer.
#[derive(Debug, Clone)]
pub struct DisplayConfig {
    /// Whether to emit ANSI color escape codes.
    pub use_color: bool,
    /// Target line width for wrapping (0 = no wrapping).
    pub width: usize,
    /// Use Unicode box-drawing characters for borders.
    pub unicode_box: bool,
}

impl DisplayConfig {
    /// Create a plain-text configuration (no color, no unicode boxes).
    pub fn plain() -> Self {
        Self {
            use_color: false,
            width: 80,
            unicode_box: false,
        }
    }

    /// Create a colored configuration with unicode boxes.
    pub fn colored() -> Self {
        Self {
            use_color: true,
            width: 100,
            unicode_box: true,
        }
    }

    /// Return the vertical bar character based on the `unicode_box` setting.
    pub fn bar_char(&self) -> &'static str {
        if self.unicode_box {
            "│"
        } else {
            "|"
        }
    }

    /// Return the horizontal bar character.
    pub fn hbar_char(&self) -> &'static str {
        if self.unicode_box {
            "─"
        } else {
            "-"
        }
    }

    /// Return the top-left corner character.
    pub fn corner_char(&self) -> &'static str {
        if self.unicode_box {
            "╭"
        } else {
            "+"
        }
    }

    /// Return the arrow character for label messages.
    pub fn arrow_char(&self) -> &'static str {
        if self.unicode_box {
            "╰"
        } else {
            "`"
        }
    }
}

impl Default for DisplayConfig {
    fn default() -> Self {
        Self::plain()
    }
}
