//! Error taxonomy for the lean4export reader.
//!
//! Three distinct kinds are surfaced, because the product reports them in three
//! separate buckets:
//!
//! 1. [`ExportError::Malformed`] — the input is broken (bad JSON, a spec
//!    violation, an out-of-range or forward reference, a duplicate index). The
//!    *file* is at fault; a well-formed lean4export file never triggers this.
//! 2. [`ExportError::Unsupported`] — the input is valid per the spec, but this
//!    reader/kernel version does not handle the construct yet. Carries a
//!    **named** feature string so the report can enumerate exactly what is
//!    missing.
//! 3. [`ExportError::Internal`] — an invariant this reader believes should hold
//!    was violated. This should never fire on any input; it exists so that the
//!    reader never panics (fuzzing enforces this).

use core::fmt;

/// A source position, reported as a 1-based line number and a 0-based byte
/// offset within that line. Both are best-effort but precise for the common
/// case of one JSON object per physical line.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    /// 1-based line number within the NDJSON stream.
    pub line: usize,
    /// 0-based byte offset within the line where the problem was detected.
    pub column: usize,
}

impl Position {
    /// Construct a position from a 1-based line and a 0-based byte column.
    #[must_use]
    pub const fn new(line: usize, column: usize) -> Self {
        Self { line, column }
    }

    /// A position with an unknown column (offset 0), for line-level errors.
    #[must_use]
    pub const fn line_only(line: usize) -> Self {
        Self { line, column: 0 }
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "line {}, byte {}", self.line, self.column)
    }
}

/// The three-bucket error taxonomy for the reader. See the module docs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExportError {
    /// The input is broken: invalid JSON, a spec violation, a bad/forward
    /// index reference, or a duplicate index.
    Malformed {
        /// Where the problem was detected.
        pos: Position,
        /// A human-readable description of what went wrong.
        message: String,
    },
    /// The input is valid per the spec, but this reader/kernel version does not
    /// support the construct yet. The `feature` names the specific missing
    /// capability so the report can group by it.
    Unsupported {
        /// Where the unsupported construct appeared.
        pos: Position,
        /// A stable, named feature string (e.g. `"inductive replay"`).
        feature: &'static str,
    },
    /// An internal invariant was violated. Never expected to fire; present so
    /// the reader degrades to an error instead of panicking.
    Internal {
        /// Where the invariant was checked, if known.
        pos: Position,
        /// A description of the violated invariant.
        message: String,
    },
}

impl ExportError {
    /// Build a [`ExportError::Malformed`] error.
    #[must_use]
    pub fn malformed(pos: Position, message: impl Into<String>) -> Self {
        Self::Malformed {
            pos,
            message: message.into(),
        }
    }

    /// Build a [`ExportError::Unsupported`] error with a named feature.
    #[must_use]
    pub fn unsupported(pos: Position, feature: &'static str) -> Self {
        Self::Unsupported { pos, feature }
    }

    /// Build a [`ExportError::Internal`] error.
    #[must_use]
    pub fn internal(pos: Position, message: impl Into<String>) -> Self {
        Self::Internal {
            pos,
            message: message.into(),
        }
    }

    /// The source position associated with this error.
    #[must_use]
    pub fn position(&self) -> Position {
        match self {
            Self::Malformed { pos, .. }
            | Self::Unsupported { pos, .. }
            | Self::Internal { pos, .. } => *pos,
        }
    }

    /// A short, stable bucket label for three-bucket reporting.
    #[must_use]
    pub fn kind(&self) -> &'static str {
        match self {
            Self::Malformed { .. } => "malformed-input",
            Self::Unsupported { .. } => "unsupported-construct",
            Self::Internal { .. } => "internal-invariant",
        }
    }

    /// `true` when this error is [`ExportError::Malformed`].
    #[must_use]
    pub fn is_malformed(&self) -> bool {
        matches!(self, Self::Malformed { .. })
    }

    /// `true` when this error is [`ExportError::Unsupported`].
    #[must_use]
    pub fn is_unsupported(&self) -> bool {
        matches!(self, Self::Unsupported { .. })
    }
}

impl fmt::Display for ExportError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Malformed { pos, message } => {
                write!(f, "malformed input at {pos}: {message}")
            }
            Self::Unsupported { pos, feature } => {
                write!(f, "unsupported construct at {pos}: {feature}")
            }
            Self::Internal { pos, message } => {
                write!(f, "internal invariant violated at {pos}: {message}")
            }
        }
    }
}

impl std::error::Error for ExportError {}

/// Convenience result alias for reader operations.
pub type ExportResult<T> = Result<T, ExportError>;
