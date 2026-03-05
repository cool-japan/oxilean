//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::diag_codes;
use std::collections::HashMap;

/// A simplified runtime error type — a lossy projection of [`EvalError`].
///
/// Produced by [`EvalError::to_runtime_error`] when callers only need a flat
/// error enum (e.g. for FFI or legacy interfaces) and do not need the full
/// structured context.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RuntimeError {
    /// Division or modulo by zero.
    DivisionByZero,
    /// The call stack exceeded the maximum depth.
    StackOverflow,
    /// A runtime type mismatch was detected.
    TypeMismatch {
        /// Expected type name.
        expected: String,
        /// Actual type name.
        got: String,
    },
    /// An index was out of bounds.
    IndexOutOfBounds {
        /// The requested index.
        index: usize,
        /// The length of the collection.
        len: usize,
    },
    /// A `sorry` / axiom was reached.
    SorryReached(String),
    /// A user-facing panic.
    Panic(String),
    /// An I/O error.
    Io(String),
    /// An unimplemented feature was encountered.
    Unimplemented(String),
}

/// Rendering style for error messages.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RenderStyle {
    /// Plain text, no terminal colors.
    Plain,
    /// ANSI color codes for terminals that support them.
    Ansi,
    /// Compact single-line format.
    Compact,
    /// JSON-like structured format.
    Structured,
}
/// Filters a list of errors based on a predicate.
pub struct ErrorFilter;
impl ErrorFilter {
    /// Keep only errors that satisfy `pred`.
    pub fn keep<F>(errors: Vec<EvalError>, pred: F) -> Vec<EvalError>
    where
        F: Fn(&EvalError) -> bool,
    {
        errors.into_iter().filter(|e| pred(e)).collect()
    }
    /// Remove errors that satisfy `pred`.
    pub fn remove<F>(errors: Vec<EvalError>, pred: F) -> Vec<EvalError>
    where
        F: Fn(&EvalError) -> bool,
    {
        errors.into_iter().filter(|e| !pred(e)).collect()
    }
    /// Keep only fatal errors.
    pub fn keep_fatal(errors: Vec<EvalError>) -> Vec<EvalError> {
        Self::keep(errors, |e| e.kind.is_fatal())
    }
    /// Keep only errors from a specific kind name.
    pub fn keep_kind(errors: Vec<EvalError>, kind: &str) -> Vec<EvalError> {
        let k = kind.to_string();
        Self::keep(errors, move |e| e.kind.kind_name() == k.as_str())
    }
    /// Deduplicate errors by kind (keep one per kind name).
    pub fn dedup_by_kind(errors: Vec<EvalError>) -> Vec<EvalError> {
        let mut seen = std::collections::HashSet::new();
        errors
            .into_iter()
            .filter(|e| seen.insert(e.kind.kind_name()))
            .collect()
    }
}
/// Severity of an evaluation error for triage purposes.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ErrorSeverity {
    /// Informational — not a real error.
    Info,
    /// Warning — evaluation can continue but something is suspicious.
    Warning,
    /// Error — evaluation cannot continue normally.
    Error,
    /// Critical — runtime integrity may be compromised.
    Critical,
}
/// A collection of preformatted error message templates.
pub struct ErrorTemplates;
impl ErrorTemplates {
    /// Format a "wrong number of arguments" message.
    pub fn wrong_num_args(expected: usize, got: usize) -> EvalError {
        EvalError::new(EvalErrorKind::TypeMismatch {
            expected: format!("{} arguments", expected),
            got: format!("{} arguments", got),
        })
        .with_hint(format!(
            "The function expects {} arguments but {} were provided.",
            expected, got
        ))
    }
    /// Format an "integer overflow in addition" message.
    pub fn add_overflow(a: i64, b: i64) -> EvalError {
        EvalError::new(EvalErrorKind::ArithmeticOverflow {
            op: format!("{} + {}", a, b),
        })
    }
    /// Format an "integer overflow in multiplication" message.
    pub fn mul_overflow(a: i64, b: i64) -> EvalError {
        EvalError::new(EvalErrorKind::ArithmeticOverflow {
            op: format!("{} * {}", a, b),
        })
    }
    /// Format a "negative index" message.
    pub fn negative_index(index: i64) -> EvalError {
        EvalError::new(EvalErrorKind::TypeMismatch {
            expected: "non-negative index".to_string(),
            got: format!("negative index {}", index),
        })
    }
    /// Format an "empty list head" message.
    pub fn empty_list_head() -> EvalError {
        EvalError::new(EvalErrorKind::Panic {
            message: "List.head: empty list".to_string(),
        })
        .with_note("Use `List.head?` for safe access that returns an Option.")
    }
    /// Format an "empty list tail" message.
    pub fn empty_list_tail() -> EvalError {
        EvalError::new(EvalErrorKind::Panic {
            message: "List.tail: empty list".to_string(),
        })
        .with_note("Use `List.tail?` for safe access that returns an Option.")
    }
    /// Format an "assertion failed" message.
    pub fn assertion_failed(msg: impl Into<String>) -> EvalError {
        EvalError::new(EvalErrorKind::Panic {
            message: format!("assertion failed: {}", msg.into()),
        })
    }
    /// Format a "cast failed" message.
    pub fn cast_failed(from: impl Into<String>, to: impl Into<String>) -> EvalError {
        EvalError::new(EvalErrorKind::TypeMismatch {
            expected: to.into(),
            got: from.into(),
        })
        .with_hint("Ensure the value is of the correct type before casting.")
    }
}
/// A single frame in the evaluation call stack.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EvalFrame {
    /// Name of the function / declaration being evaluated.
    pub name: String,
    /// Source location of the call site.
    pub call_site: SourceSpan,
    /// Whether this frame was a tail call.
    pub is_tail_call: bool,
}
impl EvalFrame {
    /// Create a new non-tail-call frame.
    pub fn new(name: impl Into<String>, call_site: SourceSpan) -> Self {
        EvalFrame {
            name: name.into(),
            call_site,
            is_tail_call: false,
        }
    }
    /// Mark this frame as a tail call.
    pub fn tail_call(mut self) -> Self {
        self.is_tail_call = true;
        self
    }
}
/// Records the origin of an evaluation error.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ErrorSource {
    /// Error originated in a kernel check.
    Kernel,
    /// Error originated in the elaborator.
    Elaborator,
    /// Error originated in the type checker.
    TypeChecker,
    /// Error originated in user code (tactic, term).
    UserCode { decl_name: String },
    /// Error originated in the I/O monad.
    IoMonad,
    /// Error originated in the bytecode interpreter.
    BytecodeInterp { chunk_name: String, ip: usize },
    /// Unknown source.
    Unknown,
}
/// Accumulates multiple evaluation errors, useful for non-aborting checkers.
#[derive(Default, Debug)]
pub struct ErrorAccumulator {
    errors: Vec<EvalError>,
    max_errors: Option<usize>,
}
impl ErrorAccumulator {
    /// Create a new accumulator with no limit.
    pub fn new() -> Self {
        ErrorAccumulator::default()
    }
    /// Create an accumulator that stops after `max` errors.
    pub fn with_limit(max: usize) -> Self {
        ErrorAccumulator {
            errors: Vec::new(),
            max_errors: Some(max),
        }
    }
    /// Add an error. Returns `true` if the limit has been reached.
    pub fn push(&mut self, err: EvalError) -> bool {
        self.errors.push(err);
        self.max_errors.is_some_and(|m| self.errors.len() >= m)
    }
    /// Number of accumulated errors.
    pub fn len(&self) -> usize {
        self.errors.len()
    }
    /// Whether there are no errors.
    pub fn is_empty(&self) -> bool {
        self.errors.is_empty()
    }
    /// Whether the accumulator has reached its error limit.
    pub fn at_limit(&self) -> bool {
        self.max_errors.is_some_and(|m| self.errors.len() >= m)
    }
    /// Iterate over all accumulated errors.
    pub fn iter(&self) -> std::slice::Iter<'_, EvalError> {
        self.errors.iter()
    }
    /// Consume the accumulator and return all errors.
    pub fn into_errors(self) -> Vec<EvalError> {
        self.errors
    }
    /// Format all errors as a single diagnostic string.
    pub fn format_all(&self) -> String {
        self.errors
            .iter()
            .enumerate()
            .map(|(i, e)| format!("[{}] {}", i + 1, e))
            .collect::<Vec<_>>()
            .join("\n")
    }
}
/// Tracks how much "fuel" (evaluation steps) remains.
///
/// When fuel reaches zero, an [`EvalErrorKind::FuelExhausted`] error
/// should be raised.
#[derive(Clone, Debug)]
pub struct EvalQuota {
    remaining: Option<u64>,
    initial: Option<u64>,
    steps_taken: u64,
}
impl EvalQuota {
    /// Create an unlimited quota.
    pub fn unlimited() -> Self {
        EvalQuota {
            remaining: None,
            initial: None,
            steps_taken: 0,
        }
    }
    /// Create a quota with `limit` steps.
    pub fn limited(limit: u64) -> Self {
        EvalQuota {
            remaining: Some(limit),
            initial: Some(limit),
            steps_taken: 0,
        }
    }
    /// Consume `n` steps. Returns `Err` if fuel is exhausted.
    pub fn consume(&mut self, n: u64) -> Result<(), EvalError> {
        self.steps_taken += n;
        match &mut self.remaining {
            None => Ok(()),
            Some(r) => {
                if *r < n {
                    *r = 0;
                    Err(EvalErrorBuilder::fuel_exhausted(self.initial.unwrap_or(0)))
                } else {
                    *r -= n;
                    Ok(())
                }
            }
        }
    }
    /// Consume 1 step.
    pub fn tick(&mut self) -> Result<(), EvalError> {
        self.consume(1)
    }
    /// Whether fuel is exhausted.
    pub fn is_exhausted(&self) -> bool {
        matches!(self.remaining, Some(0))
    }
    /// Remaining steps, or `None` if unlimited.
    pub fn remaining(&self) -> Option<u64> {
        self.remaining
    }
    /// Total steps taken.
    pub fn steps_taken(&self) -> u64 {
        self.steps_taken
    }
    /// Reset the quota back to its initial limit.
    pub fn reset(&mut self) {
        self.remaining = self.initial;
        self.steps_taken = 0;
    }
}
/// A snapshot of the current evaluation context at a point in time.
#[derive(Clone, Debug)]
pub struct ContextSnapshot {
    /// Captured frames at snapshot time.
    pub frames: Vec<EvalFrame>,
    /// Timestamp (monotonic step count, not wall time).
    pub step: u64,
}
impl ContextSnapshot {
    /// Take a snapshot from an [`EvalErrorContext`] at the given step.
    pub fn take(ctx: &EvalErrorContext, step: u64) -> Self {
        ContextSnapshot {
            frames: ctx.frames.clone(),
            step,
        }
    }
    /// Convert back to an [`EvalError`] with these frames.
    pub fn into_error(self, kind: EvalErrorKind) -> EvalError {
        EvalError::new(kind).with_frames(self.frames)
    }
}
/// A structured evaluation error with context.
///
/// Contains the error kind, optional source location, the evaluation call
/// stack at the point of failure, and optional hints for the user.
#[derive(Clone, Debug)]
pub struct EvalError {
    /// What kind of error occurred.
    pub kind: EvalErrorKind,
    /// Optional source span where the error occurred.
    pub span: Option<SourceSpan>,
    /// The evaluation call stack at the point of failure (innermost first).
    pub frames: Vec<EvalFrame>,
    /// Optional user-facing hints on how to fix the error.
    pub hints: Vec<String>,
    /// Optional note providing extra context.
    pub note: Option<String>,
}
impl EvalError {
    /// Create a new evaluation error.
    pub fn new(kind: EvalErrorKind) -> Self {
        EvalError {
            kind,
            span: None,
            frames: Vec::new(),
            hints: Vec::new(),
            note: None,
        }
    }
    /// Attach a source span to this error.
    pub fn with_span(mut self, span: SourceSpan) -> Self {
        self.span = Some(span);
        self
    }
    /// Push a call frame onto the error's context.
    pub fn with_frame(mut self, frame: EvalFrame) -> Self {
        self.frames.push(frame);
        self
    }
    /// Push multiple call frames.
    pub fn with_frames(mut self, frames: impl IntoIterator<Item = EvalFrame>) -> Self {
        self.frames.extend(frames);
        self
    }
    /// Add a user-facing hint.
    pub fn with_hint(mut self, hint: impl Into<String>) -> Self {
        self.hints.push(hint.into());
        self
    }
    /// Add a note.
    pub fn with_note(mut self, note: impl Into<String>) -> Self {
        self.note = Some(note.into());
        self
    }
    /// Whether the error has any call stack frames.
    pub fn has_context(&self) -> bool {
        !self.frames.is_empty()
    }
    /// Convert to a [`RuntimeError`] (lossy — drops context).
    pub fn to_runtime_error(&self) -> RuntimeError {
        match &self.kind {
            EvalErrorKind::DivisionByZero => RuntimeError::DivisionByZero,
            EvalErrorKind::StackOverflow { .. } => RuntimeError::StackOverflow,
            EvalErrorKind::TypeMismatch { expected, got } => RuntimeError::TypeMismatch {
                expected: expected.clone(),
                got: got.clone(),
            },
            EvalErrorKind::IndexOutOfBounds { index, len } => RuntimeError::IndexOutOfBounds {
                index: *index,
                len: *len,
            },
            EvalErrorKind::SorryReached { name } => RuntimeError::SorryReached(name.clone()),
            EvalErrorKind::Panic { message } => RuntimeError::Panic(message.clone()),
            EvalErrorKind::Io { message } => RuntimeError::Io(message.clone()),
            EvalErrorKind::Unimplemented { feature } => {
                RuntimeError::Unimplemented(feature.clone())
            }
            other => RuntimeError::Panic(other.to_string()),
        }
    }
}
impl EvalError {
    /// Whether this error has a source span.
    pub fn has_span(&self) -> bool {
        self.span.is_some()
    }
    /// Whether this error has hints.
    pub fn has_hints(&self) -> bool {
        !self.hints.is_empty()
    }
    /// Short compact string representation.
    pub fn compact(&self) -> String {
        ErrorRenderer::compact().render(self)
    }
    /// Check whether the error kind matches a predicate.
    pub fn matches<F: Fn(&EvalErrorKind) -> bool>(&self, pred: F) -> bool {
        pred(&self.kind)
    }
}
impl EvalError {
    /// Compute the severity of this error based on its kind.
    pub fn severity(&self) -> ErrorSeverity {
        self.kind.default_severity()
    }
    /// Whether this error is at least a warning.
    pub fn is_at_least_warning(&self) -> bool {
        self.severity() >= ErrorSeverity::Warning
    }
    /// Whether this error is critical.
    pub fn is_critical(&self) -> bool {
        self.severity() == ErrorSeverity::Critical
    }
}
/// A byte-offset span inside a source file.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SourceSpan {
    /// Start byte offset (inclusive).
    pub start: usize,
    /// End byte offset (exclusive).
    pub end: usize,
    /// Optional file name / path.
    pub file: Option<String>,
}
impl SourceSpan {
    /// Create a new span.
    pub fn new(start: usize, end: usize, file: Option<String>) -> Self {
        SourceSpan { start, end, file }
    }
    /// Create a synthetic span (no file, zero-width).
    pub fn synthetic() -> Self {
        SourceSpan {
            start: 0,
            end: 0,
            file: None,
        }
    }
    /// Length of the span in bytes.
    pub fn len(&self) -> usize {
        self.end.saturating_sub(self.start)
    }
    /// Whether the span is zero-width.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
/// Renders an [`EvalError`] in various output formats.
pub struct ErrorRenderer {
    style: RenderStyle,
    include_hints: bool,
    include_note: bool,
    max_frames: usize,
}
impl ErrorRenderer {
    /// Create a new renderer with the given style.
    pub fn new(style: RenderStyle) -> Self {
        ErrorRenderer {
            style,
            include_hints: true,
            include_note: true,
            max_frames: 16,
        }
    }
    /// Create a plain-text renderer.
    pub fn plain() -> Self {
        Self::new(RenderStyle::Plain)
    }
    /// Create a compact single-line renderer.
    pub fn compact() -> Self {
        Self::new(RenderStyle::Compact)
    }
    /// Toggle hint display.
    pub fn with_hints(mut self, v: bool) -> Self {
        self.include_hints = v;
        self
    }
    /// Toggle note display.
    pub fn with_note(mut self, v: bool) -> Self {
        self.include_note = v;
        self
    }
    /// Set maximum number of frames to render.
    pub fn with_max_frames(mut self, n: usize) -> Self {
        self.max_frames = n;
        self
    }
    /// Render the error to a string.
    pub fn render(&self, err: &EvalError) -> String {
        match self.style {
            RenderStyle::Plain => self.render_plain(err),
            RenderStyle::Ansi => self.render_ansi(err),
            RenderStyle::Compact => self.render_compact(err),
            RenderStyle::Structured => self.render_structured(err),
        }
    }
    fn render_plain(&self, err: &EvalError) -> String {
        let mut out = format!("error: {}", err.kind);
        if let Some(span) = &err.span {
            out.push_str(&format!("\n  --> {}", span));
        }
        let frames_to_show = err.frames.len().min(self.max_frames);
        if frames_to_show > 0 {
            out.push_str("\ncall stack (innermost first):");
            for frame in err.frames.iter().take(frames_to_show) {
                out.push_str(&format!("\n{}", frame));
            }
            if err.frames.len() > frames_to_show {
                out.push_str(&format!(
                    "\n  ... and {} more frames",
                    err.frames.len() - frames_to_show
                ));
            }
        }
        if self.include_note {
            if let Some(note) = &err.note {
                out.push_str(&format!("\nnote: {}", note));
            }
        }
        if self.include_hints {
            for hint in &err.hints {
                out.push_str(&format!("\nhint: {}", hint));
            }
        }
        out
    }
    fn render_ansi(&self, err: &EvalError) -> String {
        let mut out = format!("\x1b[1;31merror\x1b[0m: {}", err.kind);
        if let Some(span) = &err.span {
            out.push_str(&format!("\n  \x1b[36m-->\x1b[0m {}", span));
        }
        let frames_to_show = err.frames.len().min(self.max_frames);
        if frames_to_show > 0 {
            out.push_str("\n\x1b[1mcall stack (innermost first):\x1b[0m");
            for frame in err.frames.iter().take(frames_to_show) {
                out.push_str(&format!("\n{}", frame));
            }
        }
        if self.include_note {
            if let Some(note) = &err.note {
                out.push_str(&format!("\n\x1b[1;33mnote\x1b[0m: {}", note));
            }
        }
        if self.include_hints {
            for hint in &err.hints {
                out.push_str(&format!("\n\x1b[1;34mhint\x1b[0m: {}", hint));
            }
        }
        out
    }
    fn render_compact(&self, err: &EvalError) -> String {
        match &err.span {
            Some(span) => format!("{}: {}", span, err.kind),
            None => format!("{}", err.kind),
        }
    }
    fn render_structured(&self, err: &EvalError) -> String {
        let frames: Vec<String> = err
            .frames
            .iter()
            .take(self.max_frames)
            .map(|f| format!("{{\"fn\":\"{}\",\"loc\":\"{}\"}}", f.name, f.call_site))
            .collect();
        let hints: Vec<String> = err.hints.iter().map(|h| format!("{:?}", h)).collect();
        format!(
            "{{\"kind\":\"{}\",\"frames\":[{}],\"hints\":[{}]}}",
            err.kind.kind_name(),
            frames.join(","),
            hints.join(",")
        )
    }
}
/// A chain of evaluation errors (cause → effect).
#[derive(Debug, Default)]
pub struct EvalErrorChain {
    errors: Vec<EvalError>,
}
impl EvalErrorChain {
    /// Create a new empty chain.
    pub fn new() -> Self {
        EvalErrorChain::default()
    }
    /// Append an error to the chain.
    pub fn push(mut self, err: EvalError) -> Self {
        self.errors.push(err);
        self
    }
    /// Number of errors in the chain.
    pub fn len(&self) -> usize {
        self.errors.len()
    }
    /// Whether the chain is empty.
    pub fn is_empty(&self) -> bool {
        self.errors.is_empty()
    }
    /// The root cause (first error in the chain).
    pub fn root_cause(&self) -> Option<&EvalError> {
        self.errors.first()
    }
    /// The most recent error (last in the chain).
    pub fn last_error(&self) -> Option<&EvalError> {
        self.errors.last()
    }
    /// Format the chain as a multi-line string.
    pub fn format(&self) -> String {
        self.errors
            .iter()
            .enumerate()
            .map(|(i, e)| {
                if i == 0 {
                    format!("root cause: {}", e.kind)
                } else {
                    format!("caused: {}", e.kind)
                }
            })
            .collect::<Vec<_>>()
            .join("\n  ")
    }
}
/// An error with attached source information.
#[derive(Clone, Debug)]
pub struct SourcedError {
    /// The underlying error.
    pub error: EvalError,
    /// Where the error came from.
    pub source: ErrorSource,
}
impl SourcedError {
    /// Create a new sourced error.
    pub fn new(error: EvalError, source: ErrorSource) -> Self {
        SourcedError { error, source }
    }
    /// Shorthand: create from a builder error with unknown source.
    pub fn unknown(error: EvalError) -> Self {
        Self::new(error, ErrorSource::Unknown)
    }
}
/// A diagnostic code associated with an error (for tool integration).
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct DiagCode {
    /// Category prefix (e.g., "E" for error, "W" for warning).
    pub prefix: char,
    /// Numeric code.
    pub number: u32,
}
impl DiagCode {
    /// Create a new diagnostic code.
    pub fn new(prefix: char, number: u32) -> Self {
        DiagCode { prefix, number }
    }
    /// Error code.
    pub fn error(number: u32) -> Self {
        DiagCode::new('E', number)
    }
    /// Warning code.
    pub fn warning(number: u32) -> Self {
        DiagCode::new('W', number)
    }
}
impl DiagCode {
    fn clone(&self) -> Self {
        DiagCode {
            prefix: self.prefix,
            number: self.number,
        }
    }
}
/// Tracks how many times each error kind has occurred.
#[derive(Default, Clone, Debug)]
pub struct EvalErrorStats {
    counts: std::collections::HashMap<String, u64>,
    total: u64,
}
impl EvalErrorStats {
    /// Create new empty stats.
    pub fn new() -> Self {
        EvalErrorStats::default()
    }
    /// Record an error.
    pub fn record(&mut self, err: &EvalError) {
        let key = err.kind.kind_name().to_string();
        *self.counts.entry(key).or_insert(0) += 1;
        self.total += 1;
    }
    /// Total number of recorded errors.
    pub fn total(&self) -> u64 {
        self.total
    }
    /// Count for a specific error kind name.
    pub fn count(&self, kind: &str) -> u64 {
        self.counts.get(kind).copied().unwrap_or(0)
    }
    /// Whether any errors have been recorded.
    pub fn has_errors(&self) -> bool {
        self.total > 0
    }
    /// Return the most frequent error kind.
    pub fn most_frequent(&self) -> Option<(&str, u64)> {
        self.counts
            .iter()
            .max_by_key(|(_, &v)| v)
            .map(|(k, &v)| (k.as_str(), v))
    }
    /// Reset all counts.
    pub fn reset(&mut self) {
        self.counts.clear();
        self.total = 0;
    }
}
/// A table mapping error kinds to recovery strategies.
#[derive(Clone, Debug)]
pub struct ErrorPolicy {
    entries: Vec<(EvalErrorKind, RecoveryStrategy)>,
    default_strategy: RecoveryStrategy,
}
impl ErrorPolicy {
    /// Create a new policy with the given default strategy.
    pub fn new(default: RecoveryStrategy) -> Self {
        ErrorPolicy {
            entries: Vec::new(),
            default_strategy: default,
        }
    }
    /// Create a strict policy that aborts on all errors.
    pub fn strict() -> Self {
        ErrorPolicy::new(RecoveryStrategy::Abort)
    }
    /// Create a lenient policy that logs and continues on all errors.
    pub fn lenient() -> Self {
        ErrorPolicy::new(RecoveryStrategy::LogAndContinue)
    }
    /// Register a specific strategy for a particular error kind.
    pub fn with(mut self, kind: EvalErrorKind, strategy: RecoveryStrategy) -> Self {
        self.entries.push((kind, strategy));
        self
    }
    /// Look up the strategy for a given error.
    pub fn strategy_for(&self, err: &EvalError) -> &RecoveryStrategy {
        for (kind, strategy) in &self.entries {
            if *kind == err.kind {
                return strategy;
            }
        }
        &self.default_strategy
    }
    /// Whether the policy allows continuation for the given error.
    pub fn allows_continuation(&self, err: &EvalError) -> bool {
        self.strategy_for(err).allows_continuation()
    }
}
/// Classification of what went wrong during evaluation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EvalErrorKind {
    /// Division or modulo by zero.
    DivisionByZero,
    /// The call stack exceeded the maximum depth.
    StackOverflow { max_depth: usize },
    /// A runtime type mismatch was detected.
    TypeMismatch {
        /// Expected type name.
        expected: String,
        /// Actual type name.
        got: String,
    },
    /// An index was out of bounds.
    IndexOutOfBounds {
        /// The requested index.
        index: usize,
        /// The length of the collection.
        len: usize,
    },
    /// A `sorry` / axiom was reached.
    SorryReached {
        /// Name of the sorry declaration.
        name: String,
    },
    /// The evaluator ran out of fuel (step limit exceeded).
    FuelExhausted {
        /// The step limit that was set.
        limit: u64,
    },
    /// An undefined variable was referenced.
    UndefinedVariable {
        /// Name of the variable.
        name: String,
    },
    /// An undefined global was referenced.
    UndefinedGlobal {
        /// Name of the global.
        name: String,
    },
    /// An arithmetic overflow occurred.
    ArithmeticOverflow {
        /// Description of the operation.
        op: String,
    },
    /// A non-exhaustive match failed.
    NonExhaustiveMatch {
        /// Description of the value being matched.
        value: String,
    },
    /// A user-facing panic (like Lean's `panic!`).
    Panic {
        /// The panic message.
        message: String,
    },
    /// An unimplemented feature was encountered.
    Unimplemented {
        /// Description of the missing feature.
        feature: String,
    },
    /// An I/O error.
    Io {
        /// The I/O error message.
        message: String,
    },
    /// A black-hole (infinite loop in lazy evaluation).
    BlackHole {
        /// Name of the thunk that formed a cycle.
        thunk_name: String,
    },
}
impl EvalErrorKind {
    /// Whether this is a fatal error (cannot be recovered from).
    pub fn is_fatal(&self) -> bool {
        matches!(
            self,
            EvalErrorKind::StackOverflow { .. }
                | EvalErrorKind::FuelExhausted { .. }
                | EvalErrorKind::BlackHole { .. }
        )
    }
    /// Whether this is a user-visible logic error.
    pub fn is_logic_error(&self) -> bool {
        matches!(
            self,
            EvalErrorKind::SorryReached { .. }
                | EvalErrorKind::NonExhaustiveMatch { .. }
                | EvalErrorKind::Panic { .. }
        )
    }
    /// Whether this is a resource error.
    pub fn is_resource_error(&self) -> bool {
        matches!(
            self,
            EvalErrorKind::StackOverflow { .. } | EvalErrorKind::FuelExhausted { .. }
        )
    }
    /// Whether this is a type error.
    pub fn is_type_error(&self) -> bool {
        matches!(self, EvalErrorKind::TypeMismatch { .. })
    }
    /// Short ASCII name for the kind (useful as metric keys).
    pub fn kind_name(&self) -> &'static str {
        match self {
            EvalErrorKind::DivisionByZero => "division_by_zero",
            EvalErrorKind::StackOverflow { .. } => "stack_overflow",
            EvalErrorKind::TypeMismatch { .. } => "type_mismatch",
            EvalErrorKind::IndexOutOfBounds { .. } => "index_out_of_bounds",
            EvalErrorKind::SorryReached { .. } => "sorry_reached",
            EvalErrorKind::FuelExhausted { .. } => "fuel_exhausted",
            EvalErrorKind::UndefinedVariable { .. } => "undefined_variable",
            EvalErrorKind::UndefinedGlobal { .. } => "undefined_global",
            EvalErrorKind::ArithmeticOverflow { .. } => "arithmetic_overflow",
            EvalErrorKind::NonExhaustiveMatch { .. } => "non_exhaustive_match",
            EvalErrorKind::Panic { .. } => "panic",
            EvalErrorKind::Unimplemented { .. } => "unimplemented",
            EvalErrorKind::Io { .. } => "io",
            EvalErrorKind::BlackHole { .. } => "black_hole",
        }
    }
}
impl EvalErrorKind {
    /// Map an error kind to its default severity.
    pub fn default_severity(&self) -> ErrorSeverity {
        match self {
            EvalErrorKind::DivisionByZero => ErrorSeverity::Error,
            EvalErrorKind::StackOverflow { .. } => ErrorSeverity::Critical,
            EvalErrorKind::TypeMismatch { .. } => ErrorSeverity::Error,
            EvalErrorKind::IndexOutOfBounds { .. } => ErrorSeverity::Error,
            EvalErrorKind::SorryReached { .. } => ErrorSeverity::Warning,
            EvalErrorKind::FuelExhausted { .. } => ErrorSeverity::Critical,
            EvalErrorKind::UndefinedVariable { .. } => ErrorSeverity::Error,
            EvalErrorKind::UndefinedGlobal { .. } => ErrorSeverity::Error,
            EvalErrorKind::ArithmeticOverflow { .. } => ErrorSeverity::Error,
            EvalErrorKind::NonExhaustiveMatch { .. } => ErrorSeverity::Error,
            EvalErrorKind::Panic { .. } => ErrorSeverity::Error,
            EvalErrorKind::Unimplemented { .. } => ErrorSeverity::Warning,
            EvalErrorKind::Io { .. } => ErrorSeverity::Error,
            EvalErrorKind::BlackHole { .. } => ErrorSeverity::Critical,
        }
    }
}
impl EvalErrorKind {
    /// Return the diagnostic code for this error kind.
    pub fn diag_code(&self) -> DiagCode {
        match self {
            EvalErrorKind::DivisionByZero => diag_codes::DIV_BY_ZERO,
            EvalErrorKind::StackOverflow { .. } => diag_codes::STACK_OVERFLOW,
            EvalErrorKind::TypeMismatch { .. } => diag_codes::TYPE_MISMATCH,
            EvalErrorKind::IndexOutOfBounds { .. } => diag_codes::INDEX_OOB,
            EvalErrorKind::SorryReached { .. } => diag_codes::SORRY_REACHED.clone(),
            EvalErrorKind::FuelExhausted { .. } => diag_codes::FUEL_EXHAUSTED,
            EvalErrorKind::UndefinedVariable { .. } => diag_codes::UNDEFINED_VAR,
            EvalErrorKind::UndefinedGlobal { .. } => diag_codes::UNDEFINED_GLOBAL,
            EvalErrorKind::ArithmeticOverflow { .. } => diag_codes::ARITH_OVERFLOW,
            EvalErrorKind::NonExhaustiveMatch { .. } => diag_codes::NON_EXHAUSTIVE,
            EvalErrorKind::Panic { .. } => diag_codes::PANIC,
            EvalErrorKind::Unimplemented { .. } => diag_codes::UNIMPLEMENTED.clone(),
            EvalErrorKind::Io { .. } => diag_codes::IO_ERROR,
            EvalErrorKind::BlackHole { .. } => diag_codes::BLACK_HOLE,
        }
    }
}
/// Convenience methods for constructing common [`EvalError`] values.
pub struct EvalErrorBuilder;
impl EvalErrorBuilder {
    /// Division by zero error.
    pub fn div_by_zero() -> EvalError {
        EvalError::new(EvalErrorKind::DivisionByZero)
            .with_hint("Ensure the divisor is non-zero before dividing.")
    }
    /// Stack overflow error.
    pub fn stack_overflow(max_depth: usize) -> EvalError {
        EvalError::new(EvalErrorKind::StackOverflow { max_depth })
            .with_hint("Consider using tail recursion or an iterative formulation.")
    }
    /// Type mismatch error.
    pub fn type_mismatch(expected: impl Into<String>, got: impl Into<String>) -> EvalError {
        EvalError::new(EvalErrorKind::TypeMismatch {
            expected: expected.into(),
            got: got.into(),
        })
    }
    /// Index out of bounds error.
    pub fn index_out_of_bounds(index: usize, len: usize) -> EvalError {
        EvalError::new(EvalErrorKind::IndexOutOfBounds { index, len })
            .with_hint(format!("Valid indices are 0..{}.", len.saturating_sub(1)))
    }
    /// Sorry reached error.
    pub fn sorry(name: impl Into<String>) -> EvalError {
        EvalError::new(EvalErrorKind::SorryReached { name: name.into() })
            .with_note("This declaration uses `sorry` and cannot be fully evaluated.")
    }
    /// Fuel exhausted error.
    pub fn fuel_exhausted(limit: u64) -> EvalError {
        EvalError::new(EvalErrorKind::FuelExhausted { limit })
            .with_hint("Increase the evaluation fuel limit or simplify the expression.")
    }
    /// Undefined variable error.
    pub fn undefined_var(name: impl Into<String>) -> EvalError {
        EvalError::new(EvalErrorKind::UndefinedVariable { name: name.into() })
    }
    /// Undefined global error.
    pub fn undefined_global(name: impl Into<String>) -> EvalError {
        let n = name.into();
        EvalError::new(EvalErrorKind::UndefinedGlobal { name: n.clone() })
            .with_hint(format!("Check that `{}` is imported or defined.", n))
    }
    /// Panic error.
    pub fn panic_msg(message: impl Into<String>) -> EvalError {
        EvalError::new(EvalErrorKind::Panic {
            message: message.into(),
        })
    }
    /// Black-hole (lazy cycle) error.
    pub fn black_hole(thunk_name: impl Into<String>) -> EvalError {
        EvalError::new(EvalErrorKind::BlackHole {
            thunk_name: thunk_name.into(),
        })
        .with_note(
            "A thunk was forced while it was already being evaluated — infinite loop detected.",
        )
    }
}
/// A strategy for recovering from evaluation errors.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RecoveryStrategy {
    /// Abort the entire evaluation.
    Abort,
    /// Return a default value and continue.
    ReturnDefault,
    /// Retry the operation up to `max_attempts` times.
    Retry { max_attempts: u32 },
    /// Fall back to a sorry / stub implementation.
    FallbackToSorry,
    /// Log the error and continue with `Unit`.
    LogAndContinue,
}
impl RecoveryStrategy {
    /// Whether this strategy allows the evaluation to continue.
    pub fn allows_continuation(&self) -> bool {
        !matches!(self, RecoveryStrategy::Abort)
    }
    /// Whether this strategy involves retrying.
    pub fn is_retry(&self) -> bool {
        matches!(self, RecoveryStrategy::Retry { .. })
    }
}
/// A context tracker that accumulates evaluation frames for error reporting.
///
/// Designed to be built on the call stack: push a frame when entering a
/// function, pop it when returning.
pub struct EvalErrorContext {
    /// The current frame stack (innermost last).
    frames: Vec<EvalFrame>,
    /// Whether context tracking is enabled.
    enabled: bool,
    /// Maximum number of frames to track (0 = unlimited).
    pub(crate) max_frames: usize,
}
impl EvalErrorContext {
    /// Create a new error context.
    pub fn new() -> Self {
        EvalErrorContext {
            frames: Vec::new(),
            enabled: true,
            max_frames: 64,
        }
    }
    /// Create a disabled context (no overhead for hot paths).
    pub fn disabled() -> Self {
        EvalErrorContext {
            frames: Vec::new(),
            enabled: false,
            max_frames: 0,
        }
    }
    /// Push a frame onto the context.
    pub fn push(&mut self, name: impl Into<String>, span: SourceSpan) {
        if !self.enabled {
            return;
        }
        if self.max_frames > 0 && self.frames.len() >= self.max_frames {
            return;
        }
        self.frames.push(EvalFrame::new(name, span));
    }
    /// Pop the most recent frame.
    pub fn pop(&mut self) {
        if self.enabled {
            self.frames.pop();
        }
    }
    /// Attach all accumulated frames to an error.
    pub fn annotate(&self, err: EvalError) -> EvalError {
        if self.frames.is_empty() {
            return err;
        }
        err.with_frames(self.frames.iter().rev().cloned())
    }
    /// Current stack depth.
    pub fn depth(&self) -> usize {
        self.frames.len()
    }
    /// Whether the context is empty.
    pub fn is_empty(&self) -> bool {
        self.frames.is_empty()
    }
    /// Clear all frames.
    pub fn clear(&mut self) {
        self.frames.clear();
    }
}
/// A formatted stack trace derived from an [`EvalError`].
#[derive(Clone, Debug)]
pub struct StackTrace {
    /// The error that produced this trace.
    pub error_msg: String,
    /// The formatted frame lines (innermost first).
    pub frame_lines: Vec<String>,
}
impl StackTrace {
    /// Build a stack trace from an [`EvalError`].
    pub fn from_error(err: &EvalError) -> Self {
        let error_msg = err.kind.to_string();
        let frame_lines = err.frames.iter().map(|f| f.to_string()).collect();
        StackTrace {
            error_msg,
            frame_lines,
        }
    }
    /// Number of frames in the trace.
    pub fn depth(&self) -> usize {
        self.frame_lines.len()
    }
    /// Format the stack trace as a multi-line string.
    pub fn format(&self) -> String {
        let mut out = format!("error: {}\n", self.error_msg);
        if !self.frame_lines.is_empty() {
            out.push_str("call stack:\n");
            for line in &self.frame_lines {
                out.push_str(&format!("  {}\n", line));
            }
        }
        out
    }
}
/// A no-op panic handler that silently ignores panics.
pub struct SilentPanicHandler;
/// A panic handler that prints the error to stderr.
pub struct StderrPanicHandler;
/// A panic handler that collects all panics into a vec (useful for testing).
pub struct CollectingPanicHandler {
    pub(super) panics: std::sync::Mutex<Vec<String>>,
}
impl CollectingPanicHandler {
    /// Create a new collecting handler.
    pub fn new() -> Self {
        CollectingPanicHandler {
            panics: std::sync::Mutex::new(Vec::new()),
        }
    }
    /// Get all recorded panic messages.
    pub fn collected(&self) -> Vec<String> {
        self.panics
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .clone()
    }
    /// Number of recorded panics.
    pub fn count(&self) -> usize {
        self.panics.lock().unwrap_or_else(|e| e.into_inner()).len()
    }
}
