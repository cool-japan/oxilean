//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use oxilean_kernel::*;
use std::collections::HashMap;

/// A batch of elaborator messages.
#[derive(Debug, Default)]
pub struct MessageBatch {
    messages: Vec<ElabMessage>,
}
impl MessageBatch {
    /// Create a new empty batch.
    pub fn new() -> Self {
        Self::default()
    }
    /// Add a message.
    pub fn add(&mut self, msg: ElabMessage) {
        self.messages.push(msg);
    }
    /// Whether there are any errors.
    pub fn has_errors(&self) -> bool {
        self.messages
            .iter()
            .any(|m| m.severity >= MsgSeverity::Error)
    }
    /// Number of messages.
    pub fn len(&self) -> usize {
        self.messages.len()
    }
    /// Whether empty.
    pub fn is_empty(&self) -> bool {
        self.messages.is_empty()
    }
    /// Iterate.
    pub fn iter(&self) -> std::slice::Iter<'_, ElabMessage> {
        self.messages.iter()
    }
    /// Collect error-level messages.
    pub fn errors(&self) -> Vec<&ElabMessage> {
        self.messages
            .iter()
            .filter(|m| m.severity == MsgSeverity::Error)
            .collect()
    }
    /// Collect warning-level messages.
    pub fn warnings(&self) -> Vec<&ElabMessage> {
        self.messages
            .iter()
            .filter(|m| m.severity == MsgSeverity::Warning)
            .collect()
    }
}
/// Options for pretty-printing error reports.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PrettyPrintOptions {
    /// Maximum width of the output (0 = unlimited).
    pub max_width: usize,
    /// Whether to use ANSI colour codes.
    pub use_colour: bool,
    /// Whether to include suggestions.
    pub show_suggestions: bool,
    /// Whether to include help text.
    pub show_help: bool,
    /// Display language.
    pub language: Language,
}
#[allow(dead_code)]
impl PrettyPrintOptions {
    /// Create minimal options (no colour, no suggestions, no help).
    pub fn minimal() -> Self {
        Self {
            max_width: 0,
            use_colour: false,
            show_suggestions: false,
            show_help: false,
            language: Language::English,
        }
    }
    /// Enable ANSI colours.
    pub fn with_colour(mut self) -> Self {
        self.use_colour = true;
        self
    }
    /// Set the maximum line width.
    pub fn with_width(mut self, w: usize) -> Self {
        self.max_width = w;
        self
    }
    /// Set the display language.
    pub fn with_language(mut self, lang: Language) -> Self {
        self.language = lang;
        self
    }
}
/// A simple trie node for fast prefix matching in name suggestions.
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct TrieNode {
    children: HashMap<char, TrieNode>,
    is_terminal: bool,
    word: Option<String>,
}
#[allow(dead_code)]
impl TrieNode {
    /// Create an empty node.
    pub fn new() -> Self {
        Self::default()
    }
    /// Insert a word into the trie.
    pub fn insert(&mut self, word: &str) {
        let mut cur = self;
        for ch in word.chars() {
            cur = cur.children.entry(ch).or_default();
        }
        cur.is_terminal = true;
        cur.word = Some(word.to_string());
    }
    /// Collect all words with the given prefix.
    pub fn words_with_prefix(&self, prefix: &str) -> Vec<String> {
        let mut cur = self;
        for ch in prefix.chars() {
            if let Some(next) = cur.children.get(&ch) {
                cur = next;
            } else {
                return Vec::new();
            }
        }
        let mut results = Vec::new();
        cur.collect_words(&mut results);
        results
    }
    fn collect_words(&self, results: &mut Vec<String>) {
        if let Some(ref w) = self.word {
            results.push(w.clone());
        }
        for child in self.children.values() {
            child.collect_words(results);
        }
    }
    /// Check whether the exact word exists in the trie.
    pub fn contains(&self, word: &str) -> bool {
        let mut cur = self;
        for ch in word.chars() {
            if let Some(next) = cur.children.get(&ch) {
                cur = next;
            } else {
                return false;
            }
        }
        cur.is_terminal
    }
    /// Number of terminal words in the trie.
    pub fn word_count(&self) -> usize {
        let mut count = if self.is_terminal { 1 } else { 0 };
        for child in self.children.values() {
            count += child.word_count();
        }
        count
    }
}
/// A chain of elaborator errors, each caused by the next.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct ErrorChain {
    errors: Vec<ElabMessage>,
}
#[allow(dead_code)]
impl ErrorChain {
    /// Create an empty chain.
    pub fn new() -> Self {
        Self::default()
    }
    /// Push an error onto the chain (innermost last).
    pub fn push(&mut self, msg: ElabMessage) {
        self.errors.push(msg);
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
    pub fn root_cause(&self) -> Option<&ElabMessage> {
        self.errors.first()
    }
    /// The immediate error (last in the chain).
    pub fn immediate(&self) -> Option<&ElabMessage> {
        self.errors.last()
    }
    /// Format the full chain.
    pub fn format_chain(&self) -> String {
        self.errors
            .iter()
            .enumerate()
            .map(|(i, m)| format!("  #{}: {}", i, m.format_diagnostic()))
            .collect::<Vec<_>>()
            .join("\n")
    }
    /// Clear the chain.
    pub fn clear(&mut self) {
        self.errors.clear();
    }
}
/// Suggestion engine for various error types
pub struct Suggestions;
impl Suggestions {
    /// Suggest a similar name using edit distance
    pub fn suggest_similar_name(undefined: &str, available: &[&str]) -> Option<String> {
        find_similar(undefined, available, 2).first().cloned()
    }
    /// Suggest an import for a missing name
    pub fn suggest_import(name: &str, modules: &[&str]) -> Option<String> {
        modules
            .iter()
            .find(|m| m.contains(name))
            .map(|m| format!("use {};", m))
    }
    /// Suggest a constructor for a type
    pub fn suggest_constructor(type_name: &str, constructors: &[&str]) -> Vec<String> {
        constructors
            .iter()
            .filter(|c| c.starts_with(&type_name.chars().next().unwrap_or('_').to_string()))
            .map(|s| s.to_string())
            .collect()
    }
    /// Suggest a field for a record
    pub fn suggest_field(prefix: &str, available_fields: &[&str]) -> Vec<String> {
        find_similar(prefix, available_fields, 2)
    }
    /// Suggest a tactic for proof state
    pub fn suggest_tactic(goal: &str) -> Vec<String> {
        let mut suggestions = Vec::new();
        if goal.contains("Forall") || goal.contains("∀") {
            suggestions.push("intro".to_string());
        }
        if goal.contains("∃") || goal.contains("Exists") {
            suggestions.push("exists".to_string());
        }
        if goal.contains("∧") || goal.contains("And") {
            suggestions.push("constructor".to_string());
        }
        if goal.contains("→") || goal.contains("->") {
            suggestions.push("intro".to_string());
        }
        suggestions
    }
}
/// Supported display languages.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Language {
    /// English (default).
    #[default]
    English,
    /// Japanese.
    Japanese,
    /// German.
    German,
    /// French.
    French,
}
/// A structured message produced by the elaborator.
#[derive(Debug, Clone)]
pub struct ElabMessage {
    /// Severity of the message.
    pub severity: MsgSeverity,
    /// Error code.
    pub code: Option<ErrorCode>,
    /// Message text.
    pub text: String,
    /// Source location (line, column).
    pub location: Option<(usize, usize)>,
}
impl ElabMessage {
    /// Create an error message.
    pub fn error(code: ErrorCode, text: impl Into<String>) -> Self {
        Self {
            severity: MsgSeverity::Error,
            code: Some(code),
            text: text.into(),
            location: None,
        }
    }
    /// Create a warning message.
    pub fn warning(code: ErrorCode, text: impl Into<String>) -> Self {
        Self {
            severity: MsgSeverity::Warning,
            code: Some(code),
            text: text.into(),
            location: None,
        }
    }
    /// Create an info message (no error code).
    pub fn info(text: impl Into<String>) -> Self {
        Self {
            severity: MsgSeverity::Info,
            code: None,
            text: text.into(),
            location: None,
        }
    }
    /// Attach a source location.
    pub fn at(mut self, line: usize, column: usize) -> Self {
        self.location = Some((line, column));
        self
    }
    /// Format the message as a compiler-style diagnostic string.
    pub fn format_diagnostic(&self) -> String {
        let prefix = match &self.code {
            Some(c) => format!("[{}] {}: ", c, self.severity),
            None => format!("{}: ", self.severity),
        };
        let loc = match &self.location {
            Some((l, c)) => format!(" ({}:{})", l, c),
            None => String::new(),
        };
        format!("{}{}{}", prefix, self.text, loc)
    }
}
/// Error reporting with multiple errors
#[derive(Debug)]
pub struct ErrorReport {
    /// Collection of errors
    errors: Vec<ErrorContext>,
}
impl ErrorReport {
    /// Create a new error report
    pub fn new() -> Self {
        ErrorReport { errors: Vec::new() }
    }
    /// Add an error to the report
    pub fn add_error(&mut self, error: ErrorContext) {
        self.errors.push(error);
    }
    /// Get all errors
    pub fn errors(&self) -> &[ErrorContext] {
        &self.errors
    }
    /// Check if there are any errors
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }
    /// Get error count
    pub fn error_count(&self) -> usize {
        self.errors.len()
    }
    /// Format all errors
    pub fn format_all(&self) -> String {
        let mut result = String::new();
        for (i, error) in self.errors.iter().enumerate() {
            result.push_str(&error.format_full());
            if i < self.errors.len() - 1 {
                result.push_str("\n---\n\n");
            }
        }
        result
    }
}
/// A contiguous span in source code.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    /// Start byte offset.
    pub start: usize,
    /// End byte offset (exclusive).
    pub end: usize,
    /// Line number (1-based).
    pub line: usize,
    /// Column of `start` (0-based).
    pub column: usize,
}
#[allow(dead_code)]
impl Span {
    /// Create a point span (zero-length).
    pub fn point(line: usize, column: usize, offset: usize) -> Self {
        Self {
            start: offset,
            end: offset,
            line,
            column,
        }
    }
    /// Create a span from explicit offsets.
    pub fn new(start: usize, end: usize, line: usize, column: usize) -> Self {
        Self {
            start,
            end,
            line,
            column,
        }
    }
    /// Length in bytes.
    pub fn len(&self) -> usize {
        self.end.saturating_sub(self.start)
    }
    /// Whether the span is empty.
    pub fn is_empty(&self) -> bool {
        self.start >= self.end
    }
    /// Whether this span contains the given byte offset.
    pub fn contains_offset(&self, offset: usize) -> bool {
        offset >= self.start && offset < self.end
    }
    /// Merge two spans into the smallest enclosing span.
    pub fn merge(self, other: Span) -> Self {
        let start = self.start.min(other.start);
        let end = self.end.max(other.end);
        let (line, column) = if self.start <= other.start {
            (self.line, self.column)
        } else {
            (other.line, other.column)
        };
        Self {
            start,
            end,
            line,
            column,
        }
    }
    /// Format as `line:col`.
    pub fn format_location(&self) -> String {
        format!("{}:{}", self.line, self.column)
    }
}
/// Error formatter with contextual help and suggestions
pub struct ErrorFormatter;
impl ErrorFormatter {
    /// Format a type error with suggestions
    pub fn format_type_error(expected: &str, actual: &str, context: &str) -> String {
        format!(
            "[E2000] Type Mismatch\n\
             Expected type: {}\n\
             Actual type:   {}\n\
             In context:    {}\n\
             \n\
             Try: Adding explicit type annotations or checking function signatures",
            expected, actual, context
        )
    }
    /// Format a name error with suggestions
    pub fn format_name_error(name: &str, suggestions: &[String]) -> String {
        let mut msg = format!("[E3000] Undefined Name: `{}`\n", name);
        if !suggestions.is_empty() {
            msg.push_str("Similar names:\n");
            for sugg in suggestions.iter().take(3) {
                msg.push_str(&format!("  - {}\n", sugg));
            }
        }
        msg
    }
    /// Format a universe error
    pub fn format_universe_error(level: u32, required: u32) -> String {
        format!(
            "[E4000] Universe Level Too Small\n\
             Current level:  .{}\n\
             Required level: .{}\n\
             \n\
             Try: Increasing universe level or using @u polymorphism",
            level, required
        )
    }
    /// Format a match error
    pub fn format_match_error(missing_patterns: &[String]) -> String {
        let mut msg = "[E5000] Non-Exhaustive Pattern Match\n".to_string();
        msg.push_str("Missing patterns:\n");
        for pattern in missing_patterns.iter().take(5) {
            msg.push_str(&format!("  - {}\n", pattern));
        }
        msg
    }
    /// Format a recursion error
    pub fn format_recursion_error(func_name: &str, cycle: &[String]) -> String {
        let mut msg = format!("[E2009] Circular Definition: `{}`\n", func_name);
        msg.push_str("Recursion cycle:\n");
        for item in cycle {
            msg.push_str(&format!("  -> {}\n", item));
        }
        msg
    }
}
/// A labelled span used to annotate source code in a diagnostic.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SpanLabel {
    /// The span.
    pub span: Span,
    /// Label text (may be empty for primary spans).
    pub label: String,
    /// Whether this is the primary (error site) span.
    pub is_primary: bool,
}
#[allow(dead_code)]
impl SpanLabel {
    /// Create a primary labelled span.
    pub fn primary(span: Span, label: impl Into<String>) -> Self {
        Self {
            span,
            label: label.into(),
            is_primary: true,
        }
    }
    /// Create a secondary labelled span.
    pub fn secondary(span: Span, label: impl Into<String>) -> Self {
        Self {
            span,
            label: label.into(),
            is_primary: false,
        }
    }
}
/// A suggested fix to apply to source code.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FixSuggestion {
    /// Human-readable description of the fix.
    pub description: String,
    /// Replacement text.
    pub replacement: String,
    /// Span to be replaced.
    pub span: Span,
    /// Confidence: 0.0 = uncertain, 1.0 = definite.
    pub confidence: f64,
}
#[allow(dead_code)]
impl FixSuggestion {
    /// Create a fix suggestion.
    pub fn new(
        description: impl Into<String>,
        replacement: impl Into<String>,
        span: Span,
        confidence: f64,
    ) -> Self {
        Self {
            description: description.into(),
            replacement: replacement.into(),
            span,
            confidence,
        }
    }
    /// Whether the fix is highly confident (≥ 0.9).
    pub fn is_confident(&self) -> bool {
        self.confidence >= 0.9
    }
    /// Format as a human-readable string.
    pub fn format(&self) -> String {
        format!(
            "suggestion ({}%): {} → replace at {}:{} with `{}`",
            (self.confidence * 100.0) as u32,
            self.description,
            self.span.line,
            self.span.column,
            self.replacement,
        )
    }
}
/// Statistics computed from a collection of errors.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct ErrorStats {
    /// Total number of errors.
    pub total: usize,
    /// Number of errors by severity.
    pub by_severity: HashMap<String, usize>,
    /// Number of errors by error code category (thousands digit).
    pub by_category: HashMap<u32, usize>,
}
#[allow(dead_code)]
impl ErrorStats {
    /// Create empty stats.
    pub fn new() -> Self {
        Self::default()
    }
    /// Compute stats from a slice of error contexts.
    pub fn from_errors(errors: &[ErrorContext]) -> Self {
        let mut stats = Self::new();
        stats.total = errors.len();
        for err in errors {
            *stats.by_severity.entry("error".to_string()).or_insert(0) += 1;
            let category = err.code.code_number() / 1000;
            *stats.by_category.entry(category).or_insert(0) += 1;
        }
        stats
    }
    /// Compute stats from a `MessageBatch`.
    pub fn from_batch(batch: &MessageBatch) -> Self {
        let mut stats = Self::new();
        stats.total = batch.len();
        for msg in batch.iter() {
            let sev_key = format!("{}", msg.severity);
            *stats.by_severity.entry(sev_key).or_insert(0) += 1;
            if let Some(code) = &msg.code {
                let category = code.code_number() / 1000;
                *stats.by_category.entry(category).or_insert(0) += 1;
            }
        }
        stats
    }
    /// Count errors in category (1 = syntax, 2 = types, etc.).
    pub fn count_category(&self, category: u32) -> usize {
        *self.by_category.get(&category).unwrap_or(&0)
    }
    /// Format as a one-line summary.
    pub fn summary(&self) -> String {
        format!(
            "total={} syntax={} types={} scope={} universe={} pattern={}",
            self.total,
            self.count_category(1),
            self.count_category(2),
            self.count_category(3),
            self.count_category(4),
            self.count_category(5),
        )
    }
}
/// Additional error codes in the E3000–E5999 range.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExtErrorCode {
    /// Private field accessed from outside its module (E3100).
    E3100,
    /// Constructor used as a function (E3101).
    E3101,
    /// Recursive definition needs termination annotation (E3102).
    E3102,
    /// Mutual recursion not allowed in this context (E3103).
    E3103,
    /// Name reserved by language (E3104).
    E3104,
    /// Missing namespace qualifier (E3105).
    E3105,
    /// Notation already registered (E3106).
    E3106,
    /// Attribute already applied (E3107).
    E3107,
    /// Attribute requires different form (E3108).
    E3108,
    /// Export without import (E3109).
    E3109,
    /// Duplicate instance (E4100).
    E4100,
    /// Orphan instance (E4101).
    E4101,
    /// Instance loops in search (E4102).
    E4102,
    /// Instance type too abstract for synthesis (E4103).
    E4103,
    /// Diamond instance conflict (E4104).
    E4104,
    /// Instance priority conflict (E4105).
    E4105,
    /// Default instance shadowed (E4106).
    E4106,
    /// Typeclass hierarchy inconsistent (E4107).
    E4107,
    /// Instance argument not synthesizable (E4108).
    E4108,
    /// Overlapping instances (E4109).
    E4109,
    /// Tactic failed (E5100).
    E5100,
    /// Goal already proved (E5101).
    E5101,
    /// Unknown tactic (E5102).
    E5102,
    /// Tactic has wrong number of arguments (E5103).
    E5103,
    /// Hypothesis not found (E5104).
    E5104,
    /// Rewrite rule has wrong type (E5105).
    E5105,
    /// Induction variable is not inductive (E5106).
    E5106,
    /// `have` hypothesis already exists (E5107).
    E5107,
    /// `exact` term has wrong type (E5108).
    E5108,
    /// `apply` tactic creates too many goals (E5109).
    E5109,
    /// `simp` lemma has unsupported form (E5110).
    E5110,
}
#[allow(dead_code)]
impl ExtErrorCode {
    /// Numeric value of the extended error code.
    pub fn code_number(self) -> u32 {
        match self {
            ExtErrorCode::E3100 => 3100,
            ExtErrorCode::E3101 => 3101,
            ExtErrorCode::E3102 => 3102,
            ExtErrorCode::E3103 => 3103,
            ExtErrorCode::E3104 => 3104,
            ExtErrorCode::E3105 => 3105,
            ExtErrorCode::E3106 => 3106,
            ExtErrorCode::E3107 => 3107,
            ExtErrorCode::E3108 => 3108,
            ExtErrorCode::E3109 => 3109,
            ExtErrorCode::E4100 => 4100,
            ExtErrorCode::E4101 => 4101,
            ExtErrorCode::E4102 => 4102,
            ExtErrorCode::E4103 => 4103,
            ExtErrorCode::E4104 => 4104,
            ExtErrorCode::E4105 => 4105,
            ExtErrorCode::E4106 => 4106,
            ExtErrorCode::E4107 => 4107,
            ExtErrorCode::E4108 => 4108,
            ExtErrorCode::E4109 => 4109,
            ExtErrorCode::E5100 => 5100,
            ExtErrorCode::E5101 => 5101,
            ExtErrorCode::E5102 => 5102,
            ExtErrorCode::E5103 => 5103,
            ExtErrorCode::E5104 => 5104,
            ExtErrorCode::E5105 => 5105,
            ExtErrorCode::E5106 => 5106,
            ExtErrorCode::E5107 => 5107,
            ExtErrorCode::E5108 => 5108,
            ExtErrorCode::E5109 => 5109,
            ExtErrorCode::E5110 => 5110,
        }
    }
    /// Human-readable description.
    pub fn description(self) -> &'static str {
        match self {
            ExtErrorCode::E3100 => "Private field accessed outside module",
            ExtErrorCode::E3101 => "Constructor used as a function",
            ExtErrorCode::E3102 => "Recursive definition needs termination annotation",
            ExtErrorCode::E3103 => "Mutual recursion not allowed in this context",
            ExtErrorCode::E3104 => "Name reserved by language",
            ExtErrorCode::E3105 => "Missing namespace qualifier",
            ExtErrorCode::E3106 => "Notation already registered",
            ExtErrorCode::E3107 => "Attribute already applied",
            ExtErrorCode::E3108 => "Attribute requires different form",
            ExtErrorCode::E3109 => "Export without import",
            ExtErrorCode::E4100 => "Duplicate instance",
            ExtErrorCode::E4101 => "Orphan instance",
            ExtErrorCode::E4102 => "Instance loops in search",
            ExtErrorCode::E4103 => "Instance type too abstract for synthesis",
            ExtErrorCode::E4104 => "Diamond instance conflict",
            ExtErrorCode::E4105 => "Instance priority conflict",
            ExtErrorCode::E4106 => "Default instance shadowed",
            ExtErrorCode::E4107 => "Typeclass hierarchy inconsistent",
            ExtErrorCode::E4108 => "Instance argument not synthesizable",
            ExtErrorCode::E4109 => "Overlapping instances",
            ExtErrorCode::E5100 => "Tactic failed",
            ExtErrorCode::E5101 => "Goal already proved",
            ExtErrorCode::E5102 => "Unknown tactic",
            ExtErrorCode::E5103 => "Tactic has wrong number of arguments",
            ExtErrorCode::E5104 => "Hypothesis not found",
            ExtErrorCode::E5105 => "Rewrite rule has wrong type",
            ExtErrorCode::E5106 => "Induction variable is not inductive",
            ExtErrorCode::E5107 => "have hypothesis already exists",
            ExtErrorCode::E5108 => "exact term has wrong type",
            ExtErrorCode::E5109 => "apply tactic creates too many goals",
            ExtErrorCode::E5110 => "simp lemma has unsupported form",
        }
    }
    /// Whether this is a hard error (vs. a warning-level issue).
    pub fn is_hard_error(self) -> bool {
        !matches!(
            self,
            ExtErrorCode::E3105 | ExtErrorCode::E3106 | ExtErrorCode::E3107
        )
    }
}
/// Error code enumeration covering all major error categories
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ErrorCode {
    /// Unexpected token (E1000)
    E1000,
    /// Expected identifier (E1001)
    E1001,
    /// Unmatched bracket (E1002)
    E1002,
    /// Invalid syntax (E1003)
    E1003,
    /// Duplicate definition (E1004)
    E1004,
    /// Invalid escape sequence (E1005)
    E1005,
    /// Unexpected end of file (E1006)
    E1006,
    /// Invalid operator (E1007)
    E1007,
    /// Missing semicolon (E1008)
    E1008,
    /// Invalid number literal (E1009)
    E1009,
    /// Malformed string (E1010)
    E1010,
    /// Type mismatch (E2000)
    E2000,
    /// Not a function type (E2001)
    E2001,
    /// Not a product type (E2002)
    E2002,
    /// Function argument count mismatch (E2003)
    E2003,
    /// Type annotation required (E2004)
    E2004,
    /// Cannot unify types (E2005)
    E2005,
    /// Occurs check failure (E2006)
    E2006,
    /// Cannot infer type (E2007)
    E2007,
    /// Invalid type constructor (E2008)
    E2008,
    /// Circular type definition (E2009)
    E2009,
    /// Universe inconsistency (E2010)
    E2010,
    /// Kind mismatch (E2011)
    E2011,
    /// Impredicative instantiation (E2012)
    E2012,
    /// Constraint unsolvable (E2013)
    E2013,
    /// Ambiguous type (E2014)
    E2014,
    /// Incompatible type constructor (E2015)
    E2015,
    /// Invalid coercion (E2016)
    E2016,
    /// Type parameter out of scope (E2017)
    E2017,
    /// Unsupported type feature (E2018)
    E2018,
    /// Multiple possible types (E2019)
    E2019,
    /// Type variable already bound (E2020)
    E2020,
    /// Undefined variable (E3000)
    E3000,
    /// Undefined type (E3001)
    E3001,
    /// Undefined module (E3002)
    E3002,
    /// Undefined field (E3003)
    E3003,
    /// Ambiguous name (E3004)
    E3004,
    /// Name shadowing (E3005)
    E3005,
    /// Inaccessible name (E3006)
    E3006,
    /// Import conflict (E3007)
    E3007,
    /// Circular import (E3008)
    E3008,
    /// Not in scope (E3009)
    E3009,
    /// Namespace error (E3010)
    E3010,
    /// Universe level too small (E4000)
    E4000,
    /// Universe inconsistency (E4001)
    E4001,
    /// Invalid universe variable (E4002)
    E4002,
    /// Universe constraint violation (E4003)
    E4003,
    /// Predicative restriction (E4004)
    E4004,
    /// Universe instantiation failure (E4005)
    E4005,
    /// Invalid universe polymorphism (E4006)
    E4006,
    /// Universe variable conflict (E4007)
    E4007,
    /// Impredicative universe (E4008)
    E4008,
    /// Universe cycle detected (E4009)
    E4009,
    /// Invalid universe elimination (E4010)
    E4010,
    /// Non-exhaustive pattern match (E5000)
    E5000,
    /// Unreachable pattern (E5001)
    E5001,
    /// Invalid pattern (E5002)
    E5002,
    /// Pattern type mismatch (E5003)
    E5003,
    /// Duplicate pattern (E5004)
    E5004,
    /// Unbound pattern variable (E5005)
    E5005,
    /// Invalid as-pattern (E5006)
    E5006,
    /// Wildcard not at end (E5007)
    E5007,
    /// Cannot match function type (E5008)
    E5008,
    /// Pattern guard failure (E5009)
    E5009,
    /// Invalid guard expression (E5010)
    E5010,
}
impl ErrorCode {
    /// Convert error code to numeric value
    pub fn code_number(self) -> u32 {
        match self {
            ErrorCode::E1000 => 1000,
            ErrorCode::E1001 => 1001,
            ErrorCode::E1002 => 1002,
            ErrorCode::E1003 => 1003,
            ErrorCode::E1004 => 1004,
            ErrorCode::E1005 => 1005,
            ErrorCode::E1006 => 1006,
            ErrorCode::E1007 => 1007,
            ErrorCode::E1008 => 1008,
            ErrorCode::E1009 => 1009,
            ErrorCode::E1010 => 1010,
            ErrorCode::E2000 => 2000,
            ErrorCode::E2001 => 2001,
            ErrorCode::E2002 => 2002,
            ErrorCode::E2003 => 2003,
            ErrorCode::E2004 => 2004,
            ErrorCode::E2005 => 2005,
            ErrorCode::E2006 => 2006,
            ErrorCode::E2007 => 2007,
            ErrorCode::E2008 => 2008,
            ErrorCode::E2009 => 2009,
            ErrorCode::E2010 => 2010,
            ErrorCode::E2011 => 2011,
            ErrorCode::E2012 => 2012,
            ErrorCode::E2013 => 2013,
            ErrorCode::E2014 => 2014,
            ErrorCode::E2015 => 2015,
            ErrorCode::E2016 => 2016,
            ErrorCode::E2017 => 2017,
            ErrorCode::E2018 => 2018,
            ErrorCode::E2019 => 2019,
            ErrorCode::E2020 => 2020,
            ErrorCode::E3000 => 3000,
            ErrorCode::E3001 => 3001,
            ErrorCode::E3002 => 3002,
            ErrorCode::E3003 => 3003,
            ErrorCode::E3004 => 3004,
            ErrorCode::E3005 => 3005,
            ErrorCode::E3006 => 3006,
            ErrorCode::E3007 => 3007,
            ErrorCode::E3008 => 3008,
            ErrorCode::E3009 => 3009,
            ErrorCode::E3010 => 3010,
            ErrorCode::E4000 => 4000,
            ErrorCode::E4001 => 4001,
            ErrorCode::E4002 => 4002,
            ErrorCode::E4003 => 4003,
            ErrorCode::E4004 => 4004,
            ErrorCode::E4005 => 4005,
            ErrorCode::E4006 => 4006,
            ErrorCode::E4007 => 4007,
            ErrorCode::E4008 => 4008,
            ErrorCode::E4009 => 4009,
            ErrorCode::E4010 => 4010,
            ErrorCode::E5000 => 5000,
            ErrorCode::E5001 => 5001,
            ErrorCode::E5002 => 5002,
            ErrorCode::E5003 => 5003,
            ErrorCode::E5004 => 5004,
            ErrorCode::E5005 => 5005,
            ErrorCode::E5006 => 5006,
            ErrorCode::E5007 => 5007,
            ErrorCode::E5008 => 5008,
            ErrorCode::E5009 => 5009,
            ErrorCode::E5010 => 5010,
        }
    }
    /// Get human-readable description of error code
    pub fn description(self) -> &'static str {
        match self {
            ErrorCode::E1000 => "Unexpected token",
            ErrorCode::E1001 => "Expected identifier",
            ErrorCode::E1002 => "Unmatched bracket",
            ErrorCode::E1003 => "Invalid syntax",
            ErrorCode::E1004 => "Duplicate definition",
            ErrorCode::E1005 => "Invalid escape sequence",
            ErrorCode::E1006 => "Unexpected end of file",
            ErrorCode::E1007 => "Invalid operator",
            ErrorCode::E1008 => "Missing semicolon",
            ErrorCode::E1009 => "Invalid number literal",
            ErrorCode::E1010 => "Malformed string",
            ErrorCode::E2000 => "Type mismatch",
            ErrorCode::E2001 => "Not a function type",
            ErrorCode::E2002 => "Not a product type",
            ErrorCode::E2003 => "Function argument count mismatch",
            ErrorCode::E2004 => "Type annotation required",
            ErrorCode::E2005 => "Cannot unify types",
            ErrorCode::E2006 => "Occurs check failure",
            ErrorCode::E2007 => "Cannot infer type",
            ErrorCode::E2008 => "Invalid type constructor",
            ErrorCode::E2009 => "Circular type definition",
            ErrorCode::E2010 => "Universe inconsistency",
            ErrorCode::E2011 => "Kind mismatch",
            ErrorCode::E2012 => "Impredicative instantiation",
            ErrorCode::E2013 => "Constraint unsolvable",
            ErrorCode::E2014 => "Ambiguous type",
            ErrorCode::E2015 => "Incompatible type constructor",
            ErrorCode::E2016 => "Invalid coercion",
            ErrorCode::E2017 => "Type parameter out of scope",
            ErrorCode::E2018 => "Unsupported type feature",
            ErrorCode::E2019 => "Multiple possible types",
            ErrorCode::E2020 => "Type variable already bound",
            ErrorCode::E3000 => "Undefined variable",
            ErrorCode::E3001 => "Undefined type",
            ErrorCode::E3002 => "Undefined module",
            ErrorCode::E3003 => "Undefined field",
            ErrorCode::E3004 => "Ambiguous name",
            ErrorCode::E3005 => "Name shadowing",
            ErrorCode::E3006 => "Inaccessible name",
            ErrorCode::E3007 => "Import conflict",
            ErrorCode::E3008 => "Circular import",
            ErrorCode::E3009 => "Not in scope",
            ErrorCode::E3010 => "Namespace error",
            ErrorCode::E4000 => "Universe level too small",
            ErrorCode::E4001 => "Universe inconsistency",
            ErrorCode::E4002 => "Invalid universe variable",
            ErrorCode::E4003 => "Universe constraint violation",
            ErrorCode::E4004 => "Predicative restriction",
            ErrorCode::E4005 => "Universe instantiation failure",
            ErrorCode::E4006 => "Invalid universe polymorphism",
            ErrorCode::E4007 => "Universe variable conflict",
            ErrorCode::E4008 => "Impredicative universe",
            ErrorCode::E4009 => "Universe cycle detected",
            ErrorCode::E4010 => "Invalid universe elimination",
            ErrorCode::E5000 => "Non-exhaustive pattern match",
            ErrorCode::E5001 => "Unreachable pattern",
            ErrorCode::E5002 => "Invalid pattern",
            ErrorCode::E5003 => "Pattern type mismatch",
            ErrorCode::E5004 => "Duplicate pattern",
            ErrorCode::E5005 => "Unbound pattern variable",
            ErrorCode::E5006 => "Invalid as-pattern",
            ErrorCode::E5007 => "Wildcard not at end",
            ErrorCode::E5008 => "Cannot match function type",
            ErrorCode::E5009 => "Pattern guard failure",
            ErrorCode::E5010 => "Invalid guard expression",
        }
    }
}
/// Error context tracking
#[derive(Debug, Clone)]
pub struct ErrorContext {
    /// Error code
    pub code: ErrorCode,
    /// Human-readable error message
    pub message: String,
    /// Source line number
    pub line: usize,
    /// Source column number
    pub column: usize,
    /// Suggested fixes
    pub suggestions: Vec<String>,
    /// Additional help text
    pub help: Option<String>,
}
impl ErrorContext {
    /// Create a new error context
    pub fn new(code: ErrorCode, message: String, line: usize, column: usize) -> Self {
        ErrorContext {
            code,
            message,
            line,
            column,
            suggestions: Vec::new(),
            help: None,
        }
    }
    /// Add suggestions to the error context
    pub fn with_suggestions(mut self, suggestions: Vec<String>) -> Self {
        self.suggestions = suggestions;
        self
    }
    /// Add help text to the error context
    pub fn with_help(mut self, help: String) -> Self {
        self.help = Some(help);
        self
    }
    /// Format the complete error message
    pub fn format_full(&self) -> String {
        let mut result = String::new();
        result.push_str(&format!("[{}] {}\n", self.code, self.code.description()));
        result.push_str(&format!(
            "  at line {}, column {}\n",
            self.line, self.column
        ));
        result.push_str(&format!("  {}\n\n", self.message));
        if !self.suggestions.is_empty() {
            result.push_str("Suggestions:\n");
            for sugg in &self.suggestions {
                result.push_str(&format!("  - {}\n", sugg));
            }
            result.push('\n');
        }
        if let Some(help) = &self.help {
            result.push_str("Help:\n");
            result.push_str(help);
            result.push('\n');
        }
        result
    }
}
/// Severity level for an error message.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum MsgSeverity {
    /// Informational message.
    Info,
    /// Hint — non-blocking.
    Hint,
    /// Warning.
    Warning,
    /// Hard error.
    Error,
    /// Fatal (stops compilation).
    Fatal,
}
/// A multi-span diagnostic.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct MultiSpanDiagnostic {
    /// Primary error code.
    pub code: ErrorCode,
    /// Severity.
    pub severity: MsgSeverity,
    /// Main message.
    pub message: String,
    /// Annotated spans.
    pub spans: Vec<SpanLabel>,
    /// Help notes.
    pub notes: Vec<String>,
    /// Suggested fixes.
    pub suggestions: Vec<FixSuggestion>,
}
#[allow(dead_code)]
impl MultiSpanDiagnostic {
    /// Create a new multi-span diagnostic.
    pub fn new(code: ErrorCode, severity: MsgSeverity, message: impl Into<String>) -> Self {
        Self {
            code,
            severity,
            message: message.into(),
            spans: Vec::new(),
            notes: Vec::new(),
            suggestions: Vec::new(),
        }
    }
    /// Add a primary span.
    pub fn with_primary(mut self, span: Span, label: impl Into<String>) -> Self {
        self.spans.push(SpanLabel::primary(span, label));
        self
    }
    /// Add a secondary span.
    pub fn with_secondary(mut self, span: Span, label: impl Into<String>) -> Self {
        self.spans.push(SpanLabel::secondary(span, label));
        self
    }
    /// Add a help note.
    pub fn with_note(mut self, note: impl Into<String>) -> Self {
        self.notes.push(note.into());
        self
    }
    /// Add a fix suggestion.
    pub fn with_suggestion(mut self, suggestion: FixSuggestion) -> Self {
        self.suggestions.push(suggestion);
        self
    }
    /// Format the diagnostic as a compact string.
    pub fn format_compact(&self) -> String {
        let prefix = format!("[{}] {}: {}", self.code, self.severity, self.message);
        let mut lines = vec![prefix];
        for span in &self.spans {
            lines.push(format!(
                "  {} at {}:{}  {}",
                if span.is_primary { "-->" } else { "  +" },
                span.span.line,
                span.span.column,
                span.label,
            ));
        }
        for note in &self.notes {
            lines.push(format!("  note: {}", note));
        }
        lines.join("\n")
    }
    /// Return the primary span, if any.
    pub fn primary_span(&self) -> Option<&SpanLabel> {
        self.spans.iter().find(|s| s.is_primary)
    }
    /// Return all secondary spans.
    pub fn secondary_spans(&self) -> Vec<&SpanLabel> {
        self.spans.iter().filter(|s| !s.is_primary).collect()
    }
}
/// A name database backed by a trie for fast prefix lookups.
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct NameDatabase {
    root: TrieNode,
}
#[allow(dead_code)]
impl NameDatabase {
    /// Create an empty database.
    pub fn new() -> Self {
        Self::default()
    }
    /// Add a name to the database.
    pub fn add(&mut self, name: &str) {
        self.root.insert(name);
    }
    /// Add multiple names.
    pub fn add_all(&mut self, names: &[&str]) {
        for n in names {
            self.add(n);
        }
    }
    /// Return names that start with the given prefix.
    pub fn completions(&self, prefix: &str) -> Vec<String> {
        self.root.words_with_prefix(prefix)
    }
    /// Check if a name is in the database.
    pub fn contains(&self, name: &str) -> bool {
        self.root.contains(name)
    }
    /// Number of names in the database.
    pub fn len(&self) -> usize {
        self.root.word_count()
    }
    /// Whether the database is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    /// Suggest similar names (completions + edit-distance).
    pub fn suggest(&self, name: &str, max_distance: usize) -> Vec<String> {
        let mut suggestions = self.completions(name);
        let all_prefix = self.completions(&name.chars().take(2).collect::<String>());
        for candidate in &all_prefix {
            let dist = edit_distance(name, candidate);
            if dist <= max_distance && dist > 0 && !suggestions.contains(candidate) {
                suggestions.push(candidate.clone());
            }
        }
        suggestions
    }
}
