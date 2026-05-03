//! Rendering functions for structured diagnostic display.

use super::types::{
    DiagnosticLabel, DisplayConfig, LabelKind, RichDiagnostic, Severity, TermColor,
};

// ── low-level helpers ────────────────────────────────────────────────────────

/// Wrap `text` in the ANSI escape for `color` (and reset afterwards) when
/// `cfg.use_color` is true; otherwise return `text` unchanged.
pub fn colorize(text: &str, color: TermColor, cfg: &DisplayConfig) -> String {
    if cfg.use_color {
        format!(
            "{}{}{}",
            color.ansi_code(),
            text,
            TermColor::Reset.ansi_code()
        )
    } else {
        text.to_string()
    }
}

/// Apply `TermColor::Bold` to `text` when color is enabled.
fn bold(text: &str, cfg: &DisplayConfig) -> String {
    colorize(text, TermColor::Bold, cfg)
}

/// Format the diagnostic header line, e.g.
/// `error[E0001]: title` or `warning: missing return`.
fn format_header(diag: &RichDiagnostic, cfg: &DisplayConfig) -> String {
    let label = diag.severity.label();
    let code_part = match &diag.code {
        Some(code) => format!("[{}]", code),
        None => String::new(),
    };
    let header_text = format!("{}{}: {}", label, code_part, diag.title);
    colorize(&bold(&header_text, cfg), diag.severity.color(), cfg)
}

/// Convert a byte offset within `source` to a `(line, column)` pair
/// (both 1-based).  Returns `(1, 1)` if the offset is out of range.
fn offset_to_line_col(source: &str, offset: usize) -> (usize, usize) {
    let safe_offset = offset.min(source.len());
    let prefix = &source[..safe_offset];
    let line = prefix.chars().filter(|&c| c == '\n').count() + 1;
    let col = prefix
        .rfind('\n')
        .map_or(safe_offset, |pos| safe_offset - pos - 1)
        + 1;
    (line, col)
}

/// Return the text of line `line_no` (1-based) from `source`.
fn get_line(source: &str, line_no: usize) -> Option<&str> {
    source.lines().nth(line_no.saturating_sub(1))
}

/// Number of decimal digits needed to represent `n`.
fn digits(n: usize) -> usize {
    if n == 0 {
        1
    } else {
        (n as f64).log10() as usize + 1
    }
}

// ── span underline renderer ──────────────────────────────────────────────────

/// Render a single underlined span within a source listing.
///
/// Returns multiple lines:
/// 1. The source line with a line-number gutter.
/// 2. The underline row (`^^^` / `---` / `...`).
/// 3. (Optionally) a label message row.
pub fn render_span_underline(
    source: &str,
    span: (usize, usize),
    label: &str,
    kind: LabelKind,
    cfg: &DisplayConfig,
) -> String {
    let (start, end) = span;
    let (line_no, col_start) = offset_to_line_col(source, start);
    let (_, col_end) = offset_to_line_col(source, end.saturating_sub(1));

    let line_text = get_line(source, line_no).unwrap_or("");
    let gutter_width = digits(line_no) + 1; // e.g. "5 │"

    let bar = cfg.bar_char();
    let gutter = format!("{:>width$} {} ", line_no, bar, width = gutter_width);
    let empty_gutter = format!("{} {} ", " ".repeat(gutter_width), bar);

    // Source line row.
    let line_row = format!("{}{}", colorize(&gutter, TermColor::Blue, cfg), line_text);

    // Underline row.
    let underline_start = col_start.saturating_sub(1);
    let underline_len = (col_end.saturating_sub(col_start) + 1).max(1);
    let uc = kind.underline_char();
    let underline = uc.to_string().repeat(underline_len);
    let padding = " ".repeat(underline_start);
    let underline_colored = colorize(&underline, kind.color(), cfg);
    let underline_row = format!(
        "{}{}{}",
        colorize(&empty_gutter, TermColor::Blue, cfg),
        padding,
        underline_colored
    );

    let mut rows = vec![line_row, underline_row];

    // Message row.
    if !label.is_empty() {
        let arrow = cfg.arrow_char();
        let msg_padding = " ".repeat(underline_start);
        let msg_row = format!(
            "{}{}{} {}",
            colorize(&empty_gutter, TermColor::Blue, cfg),
            msg_padding,
            colorize(arrow, kind.color(), cfg),
            colorize(label, kind.color(), cfg)
        );
        rows.push(msg_row);
    }

    rows.join("\n")
}

// ── full diagnostic renderer ─────────────────────────────────────────────────

/// Render a `RichDiagnostic` as a multi-line string in rustc style.
///
/// Example output (plain text):
/// ```text
/// error[E0001]: undeclared variable `x`
///  --> src/main.oxilean:3:5
///   |
/// 3 | let y := x + 1
///   |          ^ undeclared
///   |
///   = note: make sure the variable is declared before use
///   = help: add `let x := 0` before this expression
/// ```
pub fn render_diagnostic(diag: &RichDiagnostic, cfg: &DisplayConfig) -> String {
    let mut out = Vec::new();

    // ── header ──────────────────────────────────────────────────────────────
    out.push(format_header(diag, cfg));

    // ── source view with labels ──────────────────────────────────────────────
    if let Some(ref source) = diag.source {
        if !diag.labels.is_empty() {
            // Find the line range we need to display.
            let (min_line, max_line) = label_line_range(source, &diag.labels);

            let gutter_width = digits(max_line) + 1;
            let bar = cfg.bar_char();
            let corner = cfg.corner_char();
            let hbar = cfg.hbar_char();

            // " --> file:line:col" (use first primary label for location).
            if let Some(primary) = diag.labels.iter().find(|l| l.kind == LabelKind::Primary) {
                let (line_no, col) = offset_to_line_col(source, primary.span.0);
                let arrow_line = format!(
                    " {} src:{}:{}",
                    colorize("-->", TermColor::Blue, cfg),
                    line_no,
                    col
                );
                out.push(arrow_line);
            }

            // Top border line.
            let border_top = format!(
                "{}{} {}",
                " ".repeat(gutter_width + 2),
                colorize(corner, TermColor::Blue, cfg),
                colorize(hbar, TermColor::Blue, cfg),
            );
            out.push(border_top);

            // Render each relevant source line.
            for line_no in min_line..=max_line {
                let line_text = get_line(source, line_no).unwrap_or("");
                let gutter = format!("{:>width$} {} ", line_no, bar, width = gutter_width);
                let source_row =
                    format!("{}{}", colorize(&gutter, TermColor::Blue, cfg), line_text);
                out.push(source_row);

                // Render any labels that fall on this line.
                let labels_on_line: Vec<&DiagnosticLabel> = diag
                    .labels
                    .iter()
                    .filter(|l| {
                        let (ln, _) = offset_to_line_col(source, l.span.0);
                        ln == line_no
                    })
                    .collect();

                for lbl in labels_on_line {
                    let rendered =
                        render_span_underline(source, lbl.span, &lbl.message, lbl.kind, cfg);
                    // The first line of `rendered` is the source line we already printed;
                    // skip it and only emit the underline + message lines.
                    let extra: Vec<&str> = rendered.lines().skip(1).collect();
                    for row in extra {
                        out.push(row.to_string());
                    }
                }
            }

            // Bottom gutter separator.
            let empty_gutter = format!(
                "{} {} ",
                " ".repeat(gutter_width),
                colorize(bar, TermColor::Blue, cfg)
            );
            out.push(empty_gutter);
        }
    }

    // ── notes ────────────────────────────────────────────────────────────────
    for note in &diag.notes {
        let prefix = colorize("= note:", TermColor::Cyan, cfg);
        out.push(format!("  {} {}", prefix, note));
    }

    // ── help ─────────────────────────────────────────────────────────────────
    for help in &diag.help {
        let prefix = colorize("= help:", TermColor::Green, cfg);
        out.push(format!("  {} {}", prefix, help));
    }

    out.join("\n")
}

/// Return the (min_line, max_line) range (1-based, inclusive) that the
/// labels span.
fn label_line_range(source: &str, labels: &[DiagnosticLabel]) -> (usize, usize) {
    let mut min_line = usize::MAX;
    let mut max_line = 1;
    for lbl in labels {
        let (ln_start, _) = offset_to_line_col(source, lbl.span.0);
        let (ln_end, _) = offset_to_line_col(source, lbl.span.1.saturating_sub(1));
        min_line = min_line.min(ln_start);
        max_line = max_line.max(ln_end);
    }
    if min_line == usize::MAX {
        (1, 1)
    } else {
        (min_line, max_line)
    }
}

// ── simple one-liner renderer ─────────────────────────────────────────────────

/// Render a simple one-line diagnostic without source context.
///
/// Output: `severity: message`
pub fn render_simple(severity: Severity, msg: &str, cfg: &DisplayConfig) -> String {
    let label = colorize(severity.label(), severity.color(), cfg);
    let message = colorize(msg, TermColor::White, cfg);
    format!("{}: {}", bold(&label, cfg), message)
}

// ── keyword highlighter ───────────────────────────────────────────────────────

/// Return `source` with all occurrences of each keyword in `keywords` wrapped
/// in bold + cyan coloring (when `cfg.use_color` is true).
pub fn highlight_keyword(source: &str, keywords: &[&str], cfg: &DisplayConfig) -> String {
    if !cfg.use_color || keywords.is_empty() {
        return source.to_string();
    }

    let mut result = source.to_string();
    for &kw in keywords {
        if kw.is_empty() {
            continue;
        }
        let highlighted = format!(
            "{}{}{}{}",
            TermColor::Bold.ansi_code(),
            TermColor::Cyan.ansi_code(),
            kw,
            TermColor::Reset.ansi_code()
        );
        result = result.replace(kw, &highlighted);
    }
    result
}

// ── tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error_display::types::{
        DiagnosticLabel, DisplayConfig, LabelKind, RichDiagnostic, Severity, TermColor,
    };

    fn plain() -> DisplayConfig {
        DisplayConfig::plain()
    }

    fn colored() -> DisplayConfig {
        DisplayConfig::colored()
    }

    // ── colorize ─────────────────────────────────────────────────────────────

    #[test]
    fn test_colorize_plain_no_escape() {
        let result = colorize("hello", TermColor::Red, &plain());
        assert_eq!(result, "hello");
    }

    #[test]
    fn test_colorize_colored_has_escape() {
        let result = colorize("hello", TermColor::Red, &colored());
        assert!(result.contains("\x1b[31m"));
        assert!(result.contains("hello"));
        assert!(result.contains("\x1b[0m"));
    }

    #[test]
    fn test_colorize_bold() {
        let result = colorize("bold text", TermColor::Bold, &colored());
        assert!(result.contains("\x1b[1m"));
    }

    #[test]
    fn test_colorize_empty_string() {
        let result = colorize("", TermColor::Red, &colored());
        // Should contain ANSI codes but no visible text.
        assert!(!result.contains("hello"));
    }

    // ── render_simple ────────────────────────────────────────────────────────

    #[test]
    fn test_render_simple_error_plain() {
        let out = render_simple(Severity::Error, "undefined variable", &plain());
        assert!(out.contains("error"));
        assert!(out.contains("undefined variable"));
    }

    #[test]
    fn test_render_simple_warning() {
        let out = render_simple(Severity::Warning, "unused import", &plain());
        assert!(out.contains("warning"));
        assert!(out.contains("unused import"));
    }

    #[test]
    fn test_render_simple_info() {
        let out = render_simple(Severity::Info, "elaboration started", &plain());
        assert!(out.contains("info"));
    }

    #[test]
    fn test_render_simple_hint() {
        let out = render_simple(Severity::Hint, "try using rfl", &plain());
        assert!(out.contains("hint"));
        assert!(out.contains("try using rfl"));
    }

    #[test]
    fn test_render_simple_colored_has_ansi() {
        let out = render_simple(Severity::Error, "oops", &colored());
        assert!(out.contains("\x1b["));
    }

    // ── render_diagnostic header ─────────────────────────────────────────────

    #[test]
    fn test_render_diagnostic_header_no_code() {
        let diag = RichDiagnostic::new(Severity::Error, "title only");
        let out = render_diagnostic(&diag, &plain());
        assert!(out.contains("error: title only"));
    }

    #[test]
    fn test_render_diagnostic_header_with_code() {
        let diag = RichDiagnostic::new(Severity::Error, "bad type").with_code("E0042");
        let out = render_diagnostic(&diag, &plain());
        assert!(out.contains("error[E0042]: bad type"));
    }

    #[test]
    fn test_render_diagnostic_warning_header() {
        let diag = RichDiagnostic::new(Severity::Warning, "unused");
        let out = render_diagnostic(&diag, &plain());
        assert!(out.contains("warning: unused"));
    }

    // ── render_diagnostic notes / help ───────────────────────────────────────

    #[test]
    fn test_render_diagnostic_note() {
        let diag = RichDiagnostic::new(Severity::Error, "err").add_note("check your imports");
        let out = render_diagnostic(&diag, &plain());
        assert!(out.contains("note:"));
        assert!(out.contains("check your imports"));
    }

    #[test]
    fn test_render_diagnostic_help() {
        let diag =
            RichDiagnostic::new(Severity::Error, "err").add_help("add `open Nat` at the top");
        let out = render_diagnostic(&diag, &plain());
        assert!(out.contains("help:"));
        assert!(out.contains("add `open Nat`"));
    }

    #[test]
    fn test_render_diagnostic_multiple_notes() {
        let diag = RichDiagnostic::new(Severity::Warning, "w")
            .add_note("note one")
            .add_note("note two");
        let out = render_diagnostic(&diag, &plain());
        assert!(out.contains("note one"));
        assert!(out.contains("note two"));
    }

    // ── render_diagnostic with source ────────────────────────────────────────

    #[test]
    fn test_render_diagnostic_source_line_shown() {
        let source = "let x := 1\nlet y := x + z\nlet w := 0";
        let diag = RichDiagnostic::new(Severity::Error, "undefined z")
            .with_source(source)
            .add_label(DiagnosticLabel::primary((23, 24), "not found"));
        let out = render_diagnostic(&diag, &plain());
        assert!(out.contains("let y := x + z"));
    }

    #[test]
    fn test_render_diagnostic_underline_present() {
        let source = "let x := 1\nfoo bar baz\n";
        let diag = RichDiagnostic::new(Severity::Error, "err")
            .with_source(source)
            .add_label(DiagnosticLabel::primary((11, 14), "here"));
        let out = render_diagnostic(&diag, &plain());
        // Primary underlines use '^'.
        assert!(out.contains('^'));
    }

    #[test]
    fn test_render_diagnostic_secondary_underline() {
        let source = "alpha beta gamma";
        let diag = RichDiagnostic::new(Severity::Warning, "w")
            .with_source(source)
            .add_label(DiagnosticLabel::secondary((6, 10), "secondary"));
        let out = render_diagnostic(&diag, &plain());
        assert!(out.contains('-'));
    }

    #[test]
    fn test_render_diagnostic_note_underline() {
        let source = "some text here";
        let diag = RichDiagnostic::new(Severity::Info, "info")
            .with_source(source)
            .add_label(DiagnosticLabel::note((5, 9), "a note"));
        let out = render_diagnostic(&diag, &plain());
        assert!(out.contains('.'));
    }

    // ── render_span_underline ────────────────────────────────────────────────

    #[test]
    fn test_render_span_underline_produces_carets() {
        let source = "hello world";
        let out = render_span_underline(source, (6, 11), "world", LabelKind::Primary, &plain());
        assert!(out.contains('^'));
        assert!(out.contains("world"));
    }

    #[test]
    fn test_render_span_underline_message_row() {
        let source = "abcdef";
        let out = render_span_underline(source, (0, 3), "abc here", LabelKind::Primary, &plain());
        assert!(out.contains("abc here"));
    }

    #[test]
    fn test_render_span_underline_no_message() {
        let source = "abcdef";
        let out = render_span_underline(source, (0, 3), "", LabelKind::Primary, &plain());
        // Should still have caret row but no extra message row.
        let line_count = out.lines().count();
        assert_eq!(line_count, 2); // source + underline only
    }

    #[test]
    fn test_render_span_underline_multiline_source() {
        let source = "line one\nline two\nline three";
        let out =
            render_span_underline(source, (9, 17), "line two", LabelKind::Secondary, &plain());
        assert!(out.contains('-'));
        assert!(out.contains("line two"));
    }

    // ── highlight_keyword ─────────────────────────────────────────────────────

    #[test]
    fn test_highlight_keyword_no_color() {
        let result = highlight_keyword("let x := 0", &["let"], &plain());
        // No color → unchanged.
        assert_eq!(result, "let x := 0");
    }

    #[test]
    fn test_highlight_keyword_colored() {
        let result = highlight_keyword("let x := 0", &["let"], &colored());
        assert!(result.contains("\x1b["));
        assert!(result.contains("let"));
    }

    #[test]
    fn test_highlight_keyword_multiple() {
        let result = highlight_keyword("fun x => x", &["fun", "=>"], &colored());
        assert!(result.contains("fun"));
        assert!(result.contains("=>"));
    }

    #[test]
    fn test_highlight_keyword_empty_keywords() {
        let src = "unchanged";
        let result = highlight_keyword(src, &[], &colored());
        assert_eq!(result, src);
    }

    #[test]
    fn test_highlight_keyword_empty_source() {
        let result = highlight_keyword("", &["let"], &colored());
        assert_eq!(result, "");
    }

    // ── offset_to_line_col ───────────────────────────────────────────────────

    #[test]
    fn test_offset_to_line_col_first_line() {
        let source = "hello world";
        let (ln, col) = offset_to_line_col(source, 0);
        assert_eq!((ln, col), (1, 1));
    }

    #[test]
    fn test_offset_to_line_col_second_line() {
        let source = "line1\nline2";
        let (ln, col) = offset_to_line_col(source, 6);
        assert_eq!((ln, col), (2, 1));
    }

    #[test]
    fn test_offset_to_line_col_end_of_source() {
        let source = "abc";
        let (ln, col) = offset_to_line_col(source, 3);
        assert_eq!((ln, col), (1, 4));
    }

    // ── severity ordering ─────────────────────────────────────────────────────

    #[test]
    fn test_severity_ordering() {
        assert!(Severity::Error > Severity::Warning);
        assert!(Severity::Warning > Severity::Info);
        assert!(Severity::Info > Severity::Hint);
    }

    // ── DisplayConfig ─────────────────────────────────────────────────────────

    #[test]
    fn test_display_config_plain_bar() {
        let cfg = DisplayConfig::plain();
        assert_eq!(cfg.bar_char(), "|");
    }

    #[test]
    fn test_display_config_unicode_bar() {
        let cfg = DisplayConfig::colored();
        assert_eq!(cfg.bar_char(), "│");
    }
}
