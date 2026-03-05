//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use oxilean_parse::SurfaceExpr;

/// A diff between original and formatted text.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct FormatDiff {
    /// The original text.
    pub original: String,
    /// The formatted text.
    pub formatted: String,
    /// Individual changes.
    pub changes: Vec<FormatChange>,
}
#[allow(dead_code)]
impl FormatDiff {
    /// Compute the diff between original and formatted text.
    pub fn compute_diff(original: &str, formatted: &str) -> Self {
        let orig_lines: Vec<&str> = original.lines().collect();
        let fmt_lines: Vec<&str> = formatted.lines().collect();
        let mut changes = Vec::new();
        let max_lines = orig_lines.len().max(fmt_lines.len());
        for i in 0..max_lines {
            let orig = orig_lines.get(i).unwrap_or(&"");
            let fmt = fmt_lines.get(i).unwrap_or(&"");
            if orig != fmt {
                changes.push(FormatChange {
                    line: i + 1,
                    old_text: orig.to_string(),
                    new_text: fmt.to_string(),
                });
            }
        }
        Self {
            original: original.to_string(),
            formatted: formatted.to_string(),
            changes,
        }
    }
    /// Apply the changes to produce the formatted text.
    pub fn apply_changes(&self) -> String {
        self.formatted.clone()
    }
    /// Show the diff in a human-readable format.
    pub fn show_diff(&self) -> String {
        if self.changes.is_empty() {
            return "No changes.".to_string();
        }
        let mut output = String::new();
        for change in &self.changes {
            output.push_str(&format!("Line {}:\n", change.line));
            output.push_str(&format!("  - {}\n", change.old_text));
            output.push_str(&format!("  + {}\n", change.new_text));
        }
        output.push_str(&format!("\n{} change(s) total.", self.changes.len()));
        output
    }
    /// Check if there are any changes.
    pub fn has_changes(&self) -> bool {
        !self.changes.is_empty()
    }
    /// Get the number of changes.
    pub fn num_changes(&self) -> usize {
        self.changes.len()
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct IndentWriter {
    indent_level: usize,
    indent_str: String,
    output: String,
}
#[allow(dead_code)]
impl IndentWriter {
    pub fn new(indent_str: &str) -> Self {
        Self {
            indent_level: 0,
            indent_str: indent_str.to_string(),
            output: String::new(),
        }
    }
    pub fn indent(&mut self) {
        self.indent_level += 1;
    }
    pub fn dedent(&mut self) {
        if self.indent_level > 0 {
            self.indent_level -= 1;
        }
    }
    pub fn write_line(&mut self, line: &str) {
        let prefix = self.indent_str.repeat(self.indent_level);
        self.output.push_str(&prefix);
        self.output.push_str(line);
        self.output.push('\n');
    }
    pub fn finish(self) -> String {
        self.output
    }
}
/// Format rules that can be applied to source text.
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(dead_code)]
pub enum FormatRule {
    /// Indent blocks by N spaces.
    IndentBlock(usize),
    /// Align equals signs within a block.
    AlignEquals,
    /// Compact arguments on one line if possible.
    CompactArgs,
    /// Break line before `where` keyword.
    BreakBeforeWhere,
    /// Normalize spacing around operators.
    NormalizeOperators,
}
/// Rendering mode for the pretty printer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
enum Mode {
    /// Flat (single-line) mode.
    Flat,
    /// Break (multi-line) mode.
    Break,
}
/// Represents a format-on-save configuration.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct FormatOnSaveConfig {
    /// Enable format-on-save
    pub enabled: bool,
    /// File patterns to format automatically
    pub patterns: Vec<String>,
    /// Restore point for reverting changes
    pub keep_backups: bool,
}
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Alignment {
    Left,
    Right,
    Center,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FormatterConfig {
    pub line_width: usize,
    pub tab_size: usize,
    pub use_spaces: bool,
    pub trailing_newline: bool,
}
/// Code formatter.
#[allow(dead_code)]
pub struct Formatter {
    /// Formatting options
    options: FormatOptions,
}
#[allow(dead_code)]
impl Formatter {
    /// Create a new formatter with default options.
    pub fn new() -> Self {
        Self {
            options: FormatOptions::default(),
        }
    }
    /// Create a formatter with custom options.
    pub fn with_options(options: FormatOptions) -> Self {
        Self { options }
    }
    /// Format an expression.
    pub fn format_expr(&self, expr: &SurfaceExpr) -> String {
        format!("{}", expr)
    }
    /// Format a file.
    pub fn format_file(&self, content: &str) -> Result<String, String> {
        let mut result = String::new();
        let lines: Vec<&str> = content.lines().collect();
        for line in &lines {
            let formatted = self.format_line(line);
            result.push_str(&formatted);
            result.push('\n');
        }
        if !content.ends_with('\n') && result.ends_with('\n') {
            result.pop();
        }
        Ok(result)
    }
    /// Format a single line.
    fn format_line(&self, line: &str) -> String {
        let trimmed = line.trim_end();
        if trimmed.is_empty() {
            return String::new();
        }
        let indent = line.len() - line.trim_start().len();
        let indent_str = if self.options.use_spaces {
            " ".repeat(indent)
        } else {
            "\t".repeat(indent / self.options.indent_width)
        };
        let content = trimmed.trim_start();
        let normalized = self.normalize_whitespace_in_line(content);
        format!("{}{}", indent_str, normalized)
    }
    /// Normalize whitespace within a line of content.
    pub(crate) fn normalize_whitespace_in_line(&self, content: &str) -> String {
        let mut result = String::new();
        let mut in_string = false;
        let mut prev_was_space = false;
        for ch in content.chars() {
            if ch == '"' {
                in_string = !in_string;
                result.push(ch);
                prev_was_space = false;
                continue;
            }
            if in_string {
                result.push(ch);
                prev_was_space = false;
                continue;
            }
            if ch.is_whitespace() {
                if !prev_was_space {
                    result.push(' ');
                    prev_was_space = true;
                }
            } else {
                result.push(ch);
                prev_was_space = false;
            }
        }
        result
    }
    /// Format source text, applying all rules.
    pub fn format_source(&self, source: &str) -> String {
        let mut result = source.to_string();
        result = self.normalize_whitespace(&result);
        result = self.sort_imports(&result);
        result
    }
    /// Normalize whitespace in source text.
    pub fn normalize_whitespace(&self, source: &str) -> String {
        let mut lines: Vec<String> = Vec::new();
        let mut consecutive_empty = 0;
        for line in source.lines() {
            let trimmed = line.trim_end();
            if trimmed.is_empty() {
                consecutive_empty += 1;
                if consecutive_empty <= 1 {
                    lines.push(String::new());
                }
            } else {
                consecutive_empty = 0;
                lines.push(trimmed.to_string());
            }
        }
        while lines.last().is_some_and(|l| l.is_empty()) {
            lines.pop();
        }
        let mut result = lines.join("\n");
        if source.ends_with('\n') {
            result.push('\n');
        }
        result
    }
    /// Sort import statements alphabetically.
    pub fn sort_imports(&self, source: &str) -> String {
        let lines: Vec<&str> = source.lines().collect();
        let mut result_lines: Vec<String> = Vec::new();
        let mut import_block: Vec<String> = Vec::new();
        let mut in_import_block = false;
        for line in &lines {
            let trimmed = line.trim();
            if trimmed.starts_with("import ") || trimmed.starts_with("open ") {
                in_import_block = true;
                import_block.push(line.to_string());
            } else {
                if in_import_block {
                    import_block.sort();
                    result_lines.append(&mut import_block);
                    in_import_block = false;
                }
                result_lines.push(line.to_string());
            }
        }
        if !import_block.is_empty() {
            import_block.sort();
            result_lines.extend(import_block);
        }
        result_lines.join("\n")
    }
    /// Indent a block of text by the configured amount.
    pub fn indent_block(&self, text: &str) -> String {
        let indent = if self.options.use_spaces {
            " ".repeat(self.options.indent_width)
        } else {
            "\t".to_string()
        };
        text.lines()
            .map(|line| {
                if line.trim().is_empty() {
                    String::new()
                } else {
                    format!("{}{}", indent, line)
                }
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
    /// Wrap a long line at appropriate break points.
    pub fn wrap_line(&self, line: &str) -> String {
        if line.len() <= self.options.max_width {
            return line.to_string();
        }
        let indent = line.len() - line.trim_start().len();
        let continuation_indent = indent + self.options.indent_width;
        let cont_prefix = " ".repeat(continuation_indent);
        let mut result = String::new();
        let mut current_line = String::new();
        for word in line.split_whitespace() {
            if current_line.is_empty() {
                current_line = " ".repeat(indent) + word;
            } else if current_line.len() + 1 + word.len() > self.options.max_width {
                result.push_str(&current_line);
                result.push('\n');
                current_line = format!("{}{}", cont_prefix, word);
            } else {
                current_line.push(' ');
                current_line.push_str(word);
            }
        }
        if !current_line.is_empty() {
            result.push_str(&current_line);
        }
        result
    }
    /// Get the current options.
    pub fn options(&self) -> &FormatOptions {
        &self.options
    }
    /// Set options.
    pub fn set_options(&mut self, options: FormatOptions) {
        self.options = options;
    }
}
#[allow(dead_code)]
pub struct TableFormatter {
    columns: Vec<ColumnSpec>,
    separator: char,
    border: bool,
}
#[allow(dead_code)]
impl TableFormatter {
    pub fn new(columns: Vec<ColumnSpec>) -> Self {
        Self {
            columns,
            separator: '|',
            border: true,
        }
    }
    pub fn with_separator(mut self, sep: char) -> Self {
        self.separator = sep;
        self
    }
    pub fn with_border(mut self, border: bool) -> Self {
        self.border = border;
        self
    }
    pub fn format_header(&self) -> String {
        let cells: Vec<String> = self
            .columns
            .iter()
            .map(|c| c.format_cell(&c.header.clone()))
            .collect();
        cells.join(&self.separator.to_string())
    }
    pub fn format_row(&self, values: &[&str]) -> String {
        let cells: Vec<String> = self
            .columns
            .iter()
            .zip(values.iter())
            .map(|(col, val)| col.format_cell(val))
            .collect();
        cells.join(&self.separator.to_string())
    }
    pub fn format_separator(&self) -> String {
        let parts: Vec<String> = self.columns.iter().map(|c| "-".repeat(c.width)).collect();
        parts.join(&self.separator.to_string())
    }
}
/// A comment in source code.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Comment {
    /// Line number where comment starts (1-based)
    pub line: usize,
    /// The comment text (including markers)
    pub text: String,
    /// Whether this is a line comment or block comment
    pub is_line_comment: bool,
}
/// A pretty-printing document (Wadler-Lindig style).
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum Doc {
    /// Empty document.
    Nil,
    /// A text fragment.
    Text(String),
    /// A line break. When flattened, becomes a single space.
    Line,
    /// A hard line break that cannot be flattened.
    HardLine,
    /// Increase indentation by n for the inner document.
    Nest(usize, Box<Doc>),
    /// A group that may be flattened to a single line if it fits.
    Group(Box<Doc>),
    /// Concatenation of two documents.
    Concat(Box<Doc>, Box<Doc>),
    /// Flat alternative: in flat mode use first, in break mode use second.
    FlatAlt(Box<Doc>, Box<Doc>),
}
#[allow(dead_code)]
impl Doc {
    /// Create a text document.
    pub fn text(s: impl Into<String>) -> Self {
        Doc::Text(s.into())
    }
    /// Create a nested document.
    pub fn nest(indent: usize, doc: Doc) -> Self {
        Doc::Nest(indent, Box::new(doc))
    }
    /// Create a group document.
    pub fn group(doc: Doc) -> Self {
        Doc::Group(Box::new(doc))
    }
    /// Concatenate two documents.
    pub fn concat(a: Doc, b: Doc) -> Self {
        Doc::Concat(Box::new(a), Box::new(b))
    }
    /// Concatenate with a space separator.
    pub fn space_concat(a: Doc, b: Doc) -> Self {
        Doc::Concat(
            Box::new(a),
            Box::new(Doc::Concat(
                Box::new(Doc::Text(" ".to_string())),
                Box::new(b),
            )),
        )
    }
    /// Concatenate with a line separator.
    pub fn line_concat(a: Doc, b: Doc) -> Self {
        Doc::Concat(
            Box::new(a),
            Box::new(Doc::Concat(Box::new(Doc::Line), Box::new(b))),
        )
    }
    /// Create a flat alternative.
    pub fn flat_alt(flat: Doc, breaking: Doc) -> Self {
        Doc::FlatAlt(Box::new(flat), Box::new(breaking))
    }
    /// Pretty-print to a string with the given maximum width.
    pub fn pretty_print(&self, max_width: usize) -> String {
        let mut output = String::new();
        let mut stack: Vec<LayoutEntry> = vec![LayoutEntry {
            indent: 0,
            mode: Mode::Break,
            doc: self.clone(),
        }];
        let mut col: usize = 0;
        while let Some(entry) = stack.pop() {
            match entry.doc {
                Doc::Nil => {}
                Doc::Text(s) => {
                    col += s.len();
                    output.push_str(&s);
                }
                Doc::Line => {
                    if entry.mode == Mode::Flat {
                        output.push(' ');
                        col += 1;
                    } else {
                        output.push('\n');
                        let spaces = " ".repeat(entry.indent);
                        output.push_str(&spaces);
                        col = entry.indent;
                    }
                }
                Doc::HardLine => {
                    output.push('\n');
                    let spaces = " ".repeat(entry.indent);
                    output.push_str(&spaces);
                    col = entry.indent;
                }
                Doc::Nest(n, inner) => {
                    stack.push(LayoutEntry {
                        indent: entry.indent + n,
                        mode: entry.mode,
                        doc: *inner,
                    });
                }
                Doc::Group(inner) => {
                    if fits(&flatten(&inner), max_width.saturating_sub(col)) {
                        stack.push(LayoutEntry {
                            indent: entry.indent,
                            mode: Mode::Flat,
                            doc: *inner,
                        });
                    } else {
                        stack.push(LayoutEntry {
                            indent: entry.indent,
                            mode: Mode::Break,
                            doc: *inner,
                        });
                    }
                }
                Doc::Concat(a, b) => {
                    stack.push(LayoutEntry {
                        indent: entry.indent,
                        mode: entry.mode,
                        doc: *b,
                    });
                    stack.push(LayoutEntry {
                        indent: entry.indent,
                        mode: entry.mode,
                        doc: *a,
                    });
                }
                Doc::FlatAlt(flat, breaking) => {
                    if entry.mode == Mode::Flat {
                        stack.push(LayoutEntry {
                            indent: entry.indent,
                            mode: entry.mode,
                            doc: *flat,
                        });
                    } else {
                        stack.push(LayoutEntry {
                            indent: entry.indent,
                            mode: entry.mode,
                            doc: *breaking,
                        });
                    }
                }
            }
        }
        output
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum FunctionStyle {
    Compact,
    Expanded,
    Smart,
}
/// Style configuration for formatting.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct StyleConfig {
    /// Use function application style: compact or expanded
    pub function_style: FunctionStyle,
    /// Use pattern in match arms: compact or expanded
    pub pattern_style: PatternStyle,
    /// Alignment rules
    pub alignment: AlignmentStyle,
}
/// Code formatter options.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct FormatOptions {
    /// Indentation width
    pub indent_width: usize,
    /// Maximum line width
    pub max_width: usize,
    /// Use spaces instead of tabs
    pub use_spaces: bool,
    /// Active formatting rules
    pub rules: Vec<FormatRule>,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum PatternStyle {
    Compact,
    Expanded,
}
/// Advanced line breaker for long expressions.
#[allow(dead_code)]
pub struct LineBreaker {
    max_width: usize,
    indent_width: usize,
}
#[allow(dead_code)]
impl LineBreaker {
    /// Create a new line breaker.
    pub fn new(max_width: usize, indent_width: usize) -> Self {
        Self {
            max_width,
            indent_width,
        }
    }
    /// Break a line at appropriate points.
    pub fn break_line(&self, line: &str) -> String {
        if line.len() <= self.max_width {
            return line.to_string();
        }
        let mut result = String::new();
        let mut current_line = String::new();
        let mut paren_depth = 0;
        for ch in line.chars() {
            match ch {
                '(' | '{' | '[' => {
                    current_line.push(ch);
                    paren_depth += 1;
                }
                ')' | '}' | ']' => {
                    current_line.push(ch);
                    paren_depth -= 1;
                }
                ',' | ';' if paren_depth == 0 => {
                    current_line.push(ch);
                    current_line.push(' ');
                    if current_line.len() > self.max_width {
                        result.push_str(current_line.trim_end());
                        result.push('\n');
                        current_line.clear();
                    }
                }
                _ => {
                    current_line.push(ch);
                }
            }
        }
        if !current_line.is_empty() {
            result.push_str(&current_line);
        }
        result
    }
    /// Break function arguments across lines.
    pub fn break_function_args(&self, args: &str) -> String {
        if args.len() <= self.max_width {
            return args.to_string();
        }
        let mut result = String::new();
        let mut current = String::new();
        let mut depth: i32 = 0;
        for ch in args.chars() {
            match ch {
                '(' | '{' | '[' => {
                    current.push(ch);
                    depth += 1;
                }
                ')' | '}' | ']' => {
                    current.push(ch);
                    depth = depth.saturating_sub(1);
                }
                ',' if depth == 0 => {
                    current.push(',');
                    result.push_str(&current);
                    result.push('\n');
                    let indent = " ".repeat(self.indent_width);
                    result.push_str(&indent);
                    current.clear();
                }
                _ => current.push(ch),
            }
        }
        if !current.is_empty() {
            result.push_str(current.trim_start());
        }
        result
    }
}
/// A single formatting change.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct FormatChange {
    /// Line number (1-based).
    pub line: usize,
    /// Original text.
    pub old_text: String,
    /// New text.
    pub new_text: String,
}
/// Import group categories.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ImportGroup {
    Stdlib,
    External,
    Local,
}
/// An entry on the layout stack during rendering.
#[derive(Debug, Clone)]
#[allow(dead_code)]
struct LayoutEntry {
    indent: usize,
    mode: Mode,
    doc: Doc,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ColumnSpec {
    pub width: usize,
    pub alignment: Alignment,
    pub header: String,
}
#[allow(dead_code)]
impl ColumnSpec {
    pub fn new(header: &str, width: usize, alignment: Alignment) -> Self {
        Self {
            header: header.to_string(),
            width,
            alignment,
        }
    }
    pub fn format_cell(&self, value: &str) -> String {
        match self.alignment {
            Alignment::Left => format!("{:<width$}", value, width = self.width),
            Alignment::Right => format!("{:>width$}", value, width = self.width),
            Alignment::Center => {
                let pad = self.width.saturating_sub(value.len());
                let lpad = pad / 2;
                let rpad = pad - lpad;
                format!("{}{}{}", " ".repeat(lpad), value, " ".repeat(rpad))
            }
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum AlignmentStyle {
    None,
    Pipes,
    Equals,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FormatStats {
    pub lines_processed: usize,
    pub lines_changed: usize,
    pub bytes_before: usize,
    pub bytes_after: usize,
}
#[allow(dead_code)]
impl FormatStats {
    pub fn new() -> Self {
        Self {
            lines_processed: 0,
            lines_changed: 0,
            bytes_before: 0,
            bytes_after: 0,
        }
    }
    pub fn change_ratio(&self) -> f64 {
        if self.lines_processed == 0 {
            0.0
        } else {
            self.lines_changed as f64 / self.lines_processed as f64
        }
    }
    pub fn byte_delta(&self) -> i64 {
        self.bytes_after as i64 - self.bytes_before as i64
    }
}
