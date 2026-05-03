//! Functions for parser error recovery.

use super::types::{
    ErrorClassifier, ErrorSeverity, ParseError, RecoveryPoint, RecoveryResult, RecoveryStrategy,
};

// ── Lean 4 keywords used for typo correction ────────────────────────────────

static LEAN4_KEYWORDS: &[&str] = &[
    "def",
    "theorem",
    "lemma",
    "axiom",
    "inductive",
    "structure",
    "class",
    "instance",
    "example",
    "noncomputable",
    "private",
    "protected",
    "where",
    "with",
    "fun",
    "match",
    "let",
    "have",
    "show",
    "from",
    "by",
    "do",
    "return",
    "if",
    "then",
    "else",
    "forall",
    "exists",
    "and",
    "or",
    "not",
    "true",
    "false",
    "Type",
    "Prop",
    "Sort",
    "Universe",
    "end",
    "namespace",
    "section",
    "variable",
    "import",
    "open",
    "export",
    "attribute",
    "derive",
    "abbrev",
    "opaque",
    "mutual",
];

// ── Error classification ─────────────────────────────────────────────────────

/// Classify an error message and return a canonical error code such as `"E0001"`.
///
/// Returns `None` when no pattern matches.
pub fn classify_error(msg: &str) -> Option<String> {
    let classifier = default_classifier();
    classifier.classify(msg)
}

/// Build the default [`ErrorClassifier`] with standard patterns.
fn default_classifier() -> ErrorClassifier {
    let mut c = ErrorClassifier::new();
    c.add_pattern("unexpected token", "E0001");
    c.add_pattern("expected identifier", "E0002");
    c.add_pattern("unclosed delimiter", "E0003");
    c.add_pattern("missing end", "E0004");
    c.add_pattern("missing closing", "E0005");
    c.add_pattern("undefined variable", "E0006");
    c.add_pattern("type mismatch", "E0007");
    c.add_pattern("expected `:=`", "E0008");
    c.add_pattern("expected `:`", "E0009");
    c.add_pattern("expected `)`", "E0010");
    c.add_pattern("expected `}`", "E0011");
    c.add_pattern("expected `]`", "E0012");
    c.add_pattern("missing `do`", "E0013");
    c.add_pattern("invalid pattern", "E0014");
    c.add_pattern("duplicate field", "E0015");
    c.add_pattern("unknown tactic", "E0016");
    c.add_pattern("universe level", "E0017");
    c.add_pattern("missing `by`", "E0018");
    c.add_pattern("undeclared operator", "E0019");
    c.add_pattern("ambiguous notation", "E0020");
    c
}

// ── Fix suggestions ──────────────────────────────────────────────────────────

/// Generate human-readable fix suggestions for the given error.
///
/// The suggestions are heuristic and meant to be shown verbatim in an IDE
/// or REPL error output.
pub fn suggest_fix(error: &ParseError, source: &str) -> Vec<String> {
    let mut suggestions: Vec<String> = Vec::new();

    let msg = error.message.to_lowercase();
    let (start, end) = error.span;
    let snippet = source.get(start..end).unwrap_or("");

    if msg.contains("unexpected token") || msg.contains("expected identifier") {
        if let Some(kw) = closest_keyword(snippet) {
            suggestions.push(format!("Did you mean `{}`?", kw));
        }
        suggestions.push("Check for stray punctuation or a misplaced keyword.".into());
    }

    if msg.contains("unclosed delimiter")
        || msg.contains("missing closing")
        || msg.contains("missing end")
    {
        let closing = infer_closing_delimiter(snippet);
        suggestions.push(format!("Add the matching closing delimiter `{}`.", closing));
    }

    if msg.contains("type mismatch") {
        suggestions.push("Ensure the expression type matches the expected type.".into());
        suggestions.push("Consider adding an explicit type annotation.".into());
    }

    if msg.contains("undefined variable") {
        suggestions.push(format!(
            "Check whether `{}` is in scope or has been imported.",
            snippet
        ));
    }

    if msg.contains("expected `:=`") {
        suggestions.push("Add `:=` after the declaration signature.".into());
    }

    if msg.contains("invalid pattern") {
        suggestions.push("Patterns must be constructor applications or variables.".into());
        suggestions.push("Wildcards `_` are allowed anywhere in a pattern.".into());
    }

    if suggestions.is_empty() {
        suggestions.push("Review the surrounding context for syntax errors.".into());
    }

    suggestions
}

/// Infer the closing delimiter to suggest given a snippet.
fn infer_closing_delimiter(snippet: &str) -> &'static str {
    match snippet {
        "(" => ")",
        "{" => "}",
        "[" => "]",
        _ => {
            // Check common open-without-close patterns
            if snippet.contains('(') && !snippet.contains(')') {
                ")"
            } else if snippet.contains('{') && !snippet.contains('}') {
                "}"
            } else if snippet.contains('[') && !snippet.contains(']') {
                "]"
            } else {
                "end"
            }
        }
    }
}

// ── Recovery passes ──────────────────────────────────────────────────────────

/// Attempt to recover from missing `end`/`)` tokens.
///
/// Scans `source` for unbalanced delimiters and appends the required closing
/// tokens.  All modifications are recorded in the returned [`RecoveryResult`].
pub fn recover_missing_end(source: &str) -> RecoveryResult {
    let mut errors: Vec<ParseError> = Vec::new();
    let mut recovery_points: Vec<RecoveryPoint> = Vec::new();
    let mut recovered = source.to_owned();

    // Track delimiter depth
    let mut paren_depth: i32 = 0;
    let mut brace_depth: i32 = 0;
    let mut bracket_depth: i32 = 0;
    let mut paren_open_offset: usize = 0;
    let mut brace_open_offset: usize = 0;
    let mut bracket_open_offset: usize = 0;

    let chars: Vec<(usize, char)> = source.char_indices().collect();

    for &(offset, ch) in &chars {
        match ch {
            '(' => {
                paren_depth += 1;
                paren_open_offset = offset;
            }
            ')' => {
                paren_depth -= 1;
                if paren_depth < 0 {
                    errors.push(
                        ParseError::new("unexpected closing delimiter `)`", (offset, offset + 1))
                            .with_severity(ErrorSeverity::Error)
                            .with_code("E0003"),
                    );
                    paren_depth = 0;
                }
            }
            '{' => {
                brace_depth += 1;
                brace_open_offset = offset;
            }
            '}' => {
                brace_depth -= 1;
                if brace_depth < 0 {
                    errors.push(
                        ParseError::new("unexpected closing delimiter `}`", (offset, offset + 1))
                            .with_severity(ErrorSeverity::Error)
                            .with_code("E0003"),
                    );
                    brace_depth = 0;
                }
            }
            '[' => {
                bracket_depth += 1;
                bracket_open_offset = offset;
            }
            ']' => {
                bracket_depth -= 1;
                if bracket_depth < 0 {
                    errors.push(
                        ParseError::new("unexpected closing delimiter `]`", (offset, offset + 1))
                            .with_severity(ErrorSeverity::Error)
                            .with_code("E0003"),
                    );
                    bracket_depth = 0;
                }
            }
            _ => {}
        }
    }

    // Append missing closing delimiters
    let tail = recovered.len();
    for _ in 0..bracket_depth {
        errors.push(
            ParseError::new(
                "missing closing delimiter `]`",
                (bracket_open_offset, bracket_open_offset + 1),
            )
            .with_severity(ErrorSeverity::Error)
            .with_code("E0012"),
        );
        recovery_points.push(RecoveryPoint {
            offset: tail,
            context: "end of source".into(),
            strategy: RecoveryStrategy::InsertToken { token: "]".into() },
        });
        recovered.push(']');
    }
    for _ in 0..brace_depth {
        errors.push(
            ParseError::new(
                "missing closing delimiter `}`",
                (brace_open_offset, brace_open_offset + 1),
            )
            .with_severity(ErrorSeverity::Error)
            .with_code("E0011"),
        );
        recovery_points.push(RecoveryPoint {
            offset: tail,
            context: "end of source".into(),
            strategy: RecoveryStrategy::InsertToken { token: "}".into() },
        });
        recovered.push('}');
    }
    for _ in 0..paren_depth {
        errors.push(
            ParseError::new(
                "missing closing delimiter `)`",
                (paren_open_offset, paren_open_offset + 1),
            )
            .with_severity(ErrorSeverity::Error)
            .with_code("E0010"),
        );
        recovery_points.push(RecoveryPoint {
            offset: tail,
            context: "end of source".into(),
            strategy: RecoveryStrategy::InsertToken { token: ")".into() },
        });
        recovered.push(')');
    }

    RecoveryResult {
        errors,
        recovery_points,
        recovered_source: recovered,
    }
}

/// Suggest a correction for a typo in `token` by finding the closest
/// alternative in `alternatives` using Levenshtein edit distance.
///
/// Returns `None` if no alternative is within edit distance 3.
pub fn recover_typo(source: &str, token: &str, alternatives: &[&str]) -> Option<String> {
    let _ = source; // source context reserved for future expansion
    let threshold = 3_usize;
    let mut best_dist = usize::MAX;
    let mut best: Option<String> = None;

    for &alt in alternatives {
        let d = levenshtein(token, alt);
        if d < best_dist {
            best_dist = d;
            best = Some(alt.to_owned());
        }
    }

    if best_dist <= threshold {
        best
    } else {
        None
    }
}

/// Compute the Levenshtein edit distance between strings `a` and `b`.
pub fn levenshtein(a: &str, b: &str) -> usize {
    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();
    let m = a_chars.len();
    let n = b_chars.len();

    // dp[j] = edit distance between a[0..i] and b[0..j]
    let mut prev: Vec<usize> = (0..=n).collect();
    let mut curr = vec![0_usize; n + 1];

    for i in 1..=m {
        curr[0] = i;
        for j in 1..=n {
            let cost = if a_chars[i - 1] == b_chars[j - 1] {
                0
            } else {
                1
            };
            curr[j] = (prev[j] + 1).min(curr[j - 1] + 1).min(prev[j - 1] + cost);
        }
        std::mem::swap(&mut prev, &mut curr);
    }

    prev[n]
}

/// Find the closest Lean 4 keyword to `token`.
///
/// Returns `None` when the token is already a valid keyword or when no
/// keyword is within edit distance 3.
pub fn closest_keyword(token: &str) -> Option<String> {
    // If it is already an exact keyword, no suggestion needed.
    if LEAN4_KEYWORDS.contains(&token) {
        return None;
    }
    recover_typo("", token, LEAN4_KEYWORDS)
}

// ── Heuristic error detection ────────────────────────────────────────────────

/// Run a heuristic analysis of `source` and return a list of [`ParseError`]s.
///
/// This is not a complete parser; it only catches common structural mistakes
/// that can be detected with simple pattern matching.
pub fn analyze_parse_errors(source: &str) -> Vec<ParseError> {
    let mut errors: Vec<ParseError> = Vec::new();

    // Unbalanced delimiters
    let delim_result = recover_missing_end(source);
    errors.extend(delim_result.errors);

    // Lines that look like they should have `:=` but don't
    for (line_idx, line) in source.lines().enumerate() {
        let trimmed = line.trim();
        let line_start = line_offset_in_source(source, line_idx);

        if (trimmed.starts_with("def ")
            || trimmed.starts_with("theorem ")
            || trimmed.starts_with("lemma "))
            && !trimmed.contains(":=")
            && !trimmed.ends_with("where")
            && !trimmed.is_empty()
        {
            let span = (line_start, line_start + line.len());
            errors.push(
                ParseError::new("declaration may be missing `:=`", span)
                    .with_severity(ErrorSeverity::Warning)
                    .with_code("E0008"),
            );
        }

        // Unterminated string literals (simple heuristic: odd number of `"`)
        let quote_count = trimmed.chars().filter(|&c| c == '"').count();
        if quote_count % 2 != 0 {
            let span = (line_start, line_start + line.len());
            errors.push(
                ParseError::new("unterminated string literal", span)
                    .with_severity(ErrorSeverity::Error)
                    .with_code("E0003"),
            );
        }
    }

    errors
}

/// Return the byte offset of the start of the `n`-th line (0-indexed).
pub(super) fn line_offset_in_source(source: &str, n: usize) -> usize {
    source
        .char_indices()
        .filter(|&(_, c)| c == '\n')
        .map(|(i, _)| i + 1)
        .nth(n)
        .unwrap_or(source.len())
        .min(source.len())
}

// ── Multi-line error display ─────────────────────────────────────────────────

/// Format a [`ParseError`] with surrounding source context for display.
///
/// `context_lines` controls how many lines before and after the error are shown.
pub fn format_error_with_context(source: &str, error: &ParseError, context_lines: usize) -> String {
    let lines: Vec<&str> = source.lines().collect();

    // Determine which line contains the error start
    let (err_line, err_col) = offset_to_line_col(source, error.span.0);

    let first_line = err_line.saturating_sub(context_lines);
    let last_line = (err_line + context_lines).min(lines.len().saturating_sub(1));

    let mut out = String::new();

    // Header
    let code_str = error
        .code
        .as_deref()
        .map(|c| format!("[{}] ", c))
        .unwrap_or_default();
    out.push_str(&format!(
        "{}{}: {}\n",
        code_str, error.severity, error.message
    ));

    // Context lines
    for idx in first_line..=last_line {
        let line_no = idx + 1;
        let line_text = lines.get(idx).copied().unwrap_or("");
        out.push_str(&format!("{:>4} | {}\n", line_no, line_text));

        // Draw a caret on the error line
        if idx == err_line {
            let caret_pos = err_col + 6; // account for line number prefix
            let caret: String = " ".repeat(caret_pos) + "^";
            out.push_str(&caret);
            out.push('\n');
        }
    }

    out
}

/// Convert a byte offset in `source` to a 0-based `(line, col)` pair.
pub(super) fn offset_to_line_col(source: &str, offset: usize) -> (usize, usize) {
    let safe_offset = offset.min(source.len());
    let before = &source[..safe_offset];
    let line = before.chars().filter(|&c| c == '\n').count();
    let col = before
        .rfind('\n')
        .map(|nl| safe_offset - nl - 1)
        .unwrap_or(safe_offset);
    (line, col)
}

// ── ErrorClassifier impls ────────────────────────────────────────────────────

impl ErrorClassifier {
    /// Create an empty classifier.
    pub fn new() -> Self {
        Self {
            patterns: Vec::new(),
        }
    }

    /// Register a `(pattern, error_code)` pair.
    ///
    /// Patterns are matched as substring (case-insensitive) in insertion order.
    pub fn add_pattern(&mut self, pattern: impl Into<String>, code: impl Into<String>) {
        self.patterns.push((pattern.into(), code.into()));
    }

    /// Classify `msg` and return the first matching error code.
    pub fn classify(&self, msg: &str) -> Option<String> {
        let lower = msg.to_lowercase();
        for (pat, code) in &self.patterns {
            if lower.contains(pat.as_str()) {
                return Some(code.clone());
            }
        }
        None
    }
}

// ── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // levenshtein
    #[test]
    fn test_levenshtein_equal() {
        assert_eq!(levenshtein("def", "def"), 0);
    }

    #[test]
    fn test_levenshtein_single_sub() {
        assert_eq!(levenshtein("def", "deb"), 1);
    }

    #[test]
    fn test_levenshtein_insert() {
        assert_eq!(levenshtein("def", "deff"), 1);
    }

    #[test]
    fn test_levenshtein_delete() {
        assert_eq!(levenshtein("deff", "def"), 1);
    }

    #[test]
    fn test_levenshtein_empty() {
        assert_eq!(levenshtein("", ""), 0);
        assert_eq!(levenshtein("abc", ""), 3);
        assert_eq!(levenshtein("", "abc"), 3);
    }

    #[test]
    fn test_levenshtein_transposition() {
        // "teh" -> "the": 1 substitution or 2 edits depending on metric
        // basic levenshtein: 2 (delete 'e', insert 'e' after 'h' )
        assert!(levenshtein("teh", "the") <= 2);
    }

    // closest_keyword
    #[test]
    fn test_closest_keyword_exact() {
        // Exact match returns None (already a keyword)
        assert_eq!(closest_keyword("def"), None);
    }

    #[test]
    fn test_closest_keyword_typo() {
        let result = closest_keyword("dfe");
        assert_eq!(result, Some("def".into()));
    }

    #[test]
    fn test_closest_keyword_theorem_typo() {
        let result = closest_keyword("theoram");
        assert_eq!(result.as_deref(), Some("theorem"));
    }

    #[test]
    fn test_closest_keyword_far() {
        // Completely unrelated string
        let result = closest_keyword("zzzzzzzzzzz");
        assert_eq!(result, None);
    }

    // recover_typo
    #[test]
    fn test_recover_typo_found() {
        let alts = ["hello", "world", "rust"];
        let result = recover_typo("", "helo", &alts);
        assert_eq!(result.as_deref(), Some("hello"));
    }

    #[test]
    fn test_recover_typo_none() {
        let alts = ["hello", "world"];
        let result = recover_typo("", "xyzxyz", &alts);
        assert_eq!(result, None);
    }

    // recover_missing_end
    #[test]
    fn test_recover_missing_paren() {
        let src = "def f (x : Nat := x + 1";
        let result = recover_missing_end(src);
        assert!(result.recovered_source.ends_with(')'));
        assert!(!result.errors.is_empty());
    }

    #[test]
    fn test_recover_balanced() {
        let src = "def f (x : Nat) := x";
        let result = recover_missing_end(src);
        assert!(result.errors.is_empty());
        assert_eq!(result.recovered_source, src);
    }

    #[test]
    fn test_recover_missing_brace() {
        let src = "structure Foo where { x : Nat";
        let result = recover_missing_end(src);
        assert!(result.recovered_source.ends_with('}'));
    }

    #[test]
    fn test_recover_missing_bracket() {
        let src = "let xs := [1, 2, 3";
        let result = recover_missing_end(src);
        assert!(result.recovered_source.ends_with(']'));
    }

    #[test]
    fn test_recover_extra_closing() {
        let src = "def f := 1)";
        let result = recover_missing_end(src);
        // Should record an error for the unexpected `)`
        assert!(!result.errors.is_empty());
    }

    // classify_error
    #[test]
    fn test_classify_unexpected_token() {
        assert_eq!(classify_error("unexpected token `+`"), Some("E0001".into()));
    }

    #[test]
    fn test_classify_expected_identifier() {
        assert_eq!(
            classify_error("expected identifier here"),
            Some("E0002".into())
        );
    }

    #[test]
    fn test_classify_no_match() {
        assert_eq!(classify_error("some unrelated message"), None);
    }

    // ErrorClassifier
    #[test]
    fn test_classifier_add_and_classify() {
        let mut c = ErrorClassifier::new();
        c.add_pattern("foo error", "E9001");
        assert_eq!(c.classify("this is a foo error"), Some("E9001".into()));
    }

    #[test]
    fn test_classifier_case_insensitive() {
        let mut c = ErrorClassifier::new();
        c.add_pattern("foo error", "E9001");
        assert_eq!(c.classify("FOO ERROR"), Some("E9001".into()));
    }

    #[test]
    fn test_classifier_first_match_wins() {
        let mut c = ErrorClassifier::new();
        c.add_pattern("unexpected", "E0001");
        c.add_pattern("token", "E9999");
        assert_eq!(c.classify("unexpected token"), Some("E0001".into()));
    }

    // suggest_fix
    #[test]
    fn test_suggest_fix_unclosed() {
        let err = ParseError::new("unclosed delimiter", (0, 1));
        let suggestions = suggest_fix(&err, "(");
        assert!(!suggestions.is_empty());
        assert!(suggestions.iter().any(|s| s.contains(')')));
    }

    #[test]
    fn test_suggest_fix_type_mismatch() {
        let err = ParseError::new("type mismatch", (0, 3));
        let suggestions = suggest_fix(&err, "Nat");
        assert!(suggestions
            .iter()
            .any(|s| s.to_lowercase().contains("type")));
    }

    #[test]
    fn test_suggest_fix_fallback() {
        let err = ParseError::new("some obscure error", (0, 0));
        let suggestions = suggest_fix(&err, "");
        assert!(!suggestions.is_empty());
    }

    // analyze_parse_errors
    #[test]
    fn test_analyze_def_missing_assign() {
        let src = "def foo (x : Nat)";
        let errors = analyze_parse_errors(src);
        assert!(errors.iter().any(|e| e.code.as_deref() == Some("E0008")));
    }

    #[test]
    fn test_analyze_clean_source() {
        let src = "def foo := 1";
        let errors = analyze_parse_errors(src);
        // No structural errors expected for this minimal source
        assert!(errors.iter().all(|e| e.severity != ErrorSeverity::Fatal));
    }

    // format_error_with_context
    #[test]
    fn test_format_error_with_context_basic() {
        let src = "line one\nline two\nline three";
        let err = ParseError::new("unexpected token", (9, 13)).with_code("E0001");
        let output = format_error_with_context(src, &err, 1);
        assert!(output.contains("E0001"));
        assert!(output.contains("unexpected token"));
        assert!(output.contains("line two"));
    }

    #[test]
    fn test_format_error_zero_context() {
        let src = "hello world";
        let err = ParseError::new("oops", (0, 5));
        let output = format_error_with_context(src, &err, 0);
        assert!(output.contains("oops"));
        assert!(output.contains("hello world"));
    }

    // RecoveryResult helpers
    #[test]
    fn test_recovery_result_clean() {
        let r = RecoveryResult::clean("source");
        assert!(r.is_ok());
        assert!(!r.has_fatal());
    }

    #[test]
    fn test_recovery_result_has_fatal() {
        let mut r = RecoveryResult::clean("source");
        r.errors
            .push(ParseError::new("fatal!", (0, 0)).with_severity(ErrorSeverity::Fatal));
        assert!(r.has_fatal());
        assert!(!r.is_ok());
    }

    // ErrorSeverity ordering
    #[test]
    fn test_severity_ordering() {
        assert!(ErrorSeverity::Fatal > ErrorSeverity::Error);
        assert!(ErrorSeverity::Error > ErrorSeverity::Warning);
        assert!(ErrorSeverity::Warning > ErrorSeverity::Info);
    }

    // ParseError display
    #[test]
    fn test_parse_error_display_with_code() {
        let e = ParseError::new("oops", (0, 1)).with_code("E0001");
        let s = format!("{}", e);
        assert!(s.contains("E0001"));
        assert!(s.contains("oops"));
    }

    #[test]
    fn test_parse_error_display_no_code() {
        let e = ParseError::new("oops", (0, 1));
        let s = format!("{}", e);
        assert!(s.contains("oops"));
    }
}
