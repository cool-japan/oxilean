//! Pure-Rust mini Markdown-to-HTML renderer for OxiLean doc comments.
//!
//! This renderer handles the subset of Markdown commonly found in Lean doc
//! comments: paragraphs, **bold**, *italic*, `inline code`, fenced code blocks,
//! `[text](url)` links, and HTML escaping of raw `<` and `&` characters.
//!
//! No external dependencies are used.

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/// Escape HTML special characters.
///
/// Converts `&`, `<`, `>`, and `"` to their named HTML entities.
/// This is the canonical HTML-escaping function for the doc crate; callers in
/// other modules should import this instead of implementing their own.
pub fn html_escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for ch in s.chars() {
        match ch {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&#39;"),
            other => out.push(other),
        }
    }
    out
}

/// Render a Markdown doc comment to HTML.
///
/// Supported features:
/// - Paragraphs (blank-line separated)
/// - `**bold**` → `<strong>bold</strong>`
/// - `*italic*` / `_italic_` → `<em>italic</em>`
/// - `` `inline code` `` → `<code>code</code>` (content HTML-escaped, no further processing)
/// - ` ``` `fenced code blocks` ``` ` → `<pre><code>…</code></pre>`
/// - `[text](url)` links (only `http://`, `https://`, `#`, and `/`-prefixed URLs)
/// - HTML escaping of `<` and `&` in plain text
///
/// Handles edge cases: unclosed `**` delimiters, empty input, nested backticks
/// (treated as a literal backtick).
pub fn render_markdown(input: &str) -> String {
    if input.trim().is_empty() {
        return String::new();
    }

    let mut output = String::new();
    let mut in_fenced = false;
    let mut fenced_content = String::new();
    let mut paragraph_lines: Vec<&str> = Vec::new();

    for line in input.lines() {
        if line.starts_with("```") {
            if in_fenced {
                // Close fence — emit accumulated content.
                output.push_str("<pre><code>");
                output.push_str(&html_escape(&fenced_content));
                output.push_str("</code></pre>\n");
                fenced_content.clear();
                in_fenced = false;
            } else {
                // Open fence — flush any pending paragraph first.
                flush_paragraph(&paragraph_lines, &mut output);
                paragraph_lines.clear();
                in_fenced = true;
            }
        } else if in_fenced {
            fenced_content.push_str(line);
            fenced_content.push('\n');
        } else if line.trim().is_empty() {
            flush_paragraph(&paragraph_lines, &mut output);
            paragraph_lines.clear();
        } else {
            paragraph_lines.push(line);
        }
    }

    // Handle anything remaining after iteration.
    if in_fenced {
        // Unclosed fence — treat remaining content as a code block.
        output.push_str("<pre><code>");
        output.push_str(&html_escape(&fenced_content));
        output.push_str("</code></pre>\n");
    } else {
        flush_paragraph(&paragraph_lines, &mut output);
    }

    output
}

// ---------------------------------------------------------------------------
// Block-level helpers
// ---------------------------------------------------------------------------

/// Flush a group of non-blank lines as a single `<p>…</p>`.
fn flush_paragraph(lines: &[&str], out: &mut String) {
    if lines.is_empty() {
        return;
    }
    let text = lines.join(" ");
    let rendered = render_inline(&text);
    out.push_str("<p>");
    out.push_str(&rendered);
    out.push_str("</p>\n");
}

// ---------------------------------------------------------------------------
// Inline renderer (state machine)
// ---------------------------------------------------------------------------

/// Render inline Markdown elements within a single paragraph of text.
///
/// Processes the string left-to-right, recognising:
/// - `` `code` ``
/// - `**bold**`
/// - `*italic*` / `_italic_`
/// - `[text](url)` links
/// - Raw `<`, `>`, `&`, `'` → HTML entities
fn render_inline(s: &str) -> String {
    let chars: Vec<char> = s.chars().collect();
    let mut result = String::with_capacity(s.len() + 16);
    let mut i = 0;

    while i < chars.len() {
        // --- Backtick: inline code ---
        if chars[i] == '`' {
            if let Some(end) = chars[i + 1..].iter().position(|&c| c == '`') {
                let code: String = chars[i + 1..i + 1 + end].iter().collect();
                result.push_str("<code>");
                result.push_str(&html_escape(&code));
                result.push_str("</code>");
                i += end + 2;
                continue;
            }
            // No closing backtick — emit as literal (escaped).
            result.push_str("&#96;");
            i += 1;
            continue;
        }

        // --- Double star: **bold** ---
        if i + 1 < chars.len() && chars[i] == '*' && chars[i + 1] == '*' {
            if let Some(end) = find_closing_seq(&chars, i + 2, &['*', '*']) {
                let inner: String = chars[i + 2..end].iter().collect();
                result.push_str("<strong>");
                result.push_str(&render_inline(&inner));
                result.push_str("</strong>");
                i = end + 2;
                continue;
            }
            // No closing ** — emit as literal.
            result.push('*');
            i += 1;
            continue;
        }

        // --- Single star or underscore: *italic* / _italic_ ---
        if chars[i] == '*' || chars[i] == '_' {
            let delim = chars[i];
            if let Some(rel) = chars[i + 1..].iter().position(|&c| c == delim) {
                let end = i + 1 + rel;
                let inner: String = chars[i + 1..end].iter().collect();
                result.push_str("<em>");
                result.push_str(&render_inline(&inner));
                result.push_str("</em>");
                i = end + 1;
                continue;
            }
            // No closing delimiter — emit as literal.
            result.push(chars[i]);
            i += 1;
            continue;
        }

        // --- Link: [text](url) ---
        if chars[i] == '[' {
            if let Some((text_end, url_start, url_end)) = find_link(&chars, i) {
                let text: String = chars[i + 1..text_end].iter().collect();
                let url: String = chars[url_start..url_end].iter().collect();
                // Only allow safe URL schemes.
                if url.starts_with("http://")
                    || url.starts_with("https://")
                    || url.starts_with('#')
                    || url.starts_with('/')
                {
                    result.push_str("<a href=\"");
                    result.push_str(&html_escape(&url));
                    result.push_str("\">");
                    result.push_str(&render_inline(&text));
                    result.push_str("</a>");
                    i = url_end + 1;
                    continue;
                }
                // Unsafe URL — fall through to emit `[` literally.
            }
            // No valid link syntax — emit `[` escaped.
            result.push('[');
            i += 1;
            continue;
        }

        // --- Default: HTML-escape and advance ---
        match chars[i] {
            '&' => result.push_str("&amp;"),
            '<' => result.push_str("&lt;"),
            '>' => result.push_str("&gt;"),
            '"' => result.push_str("&quot;"),
            '\'' => result.push_str("&#39;"),
            c => result.push(c),
        }
        i += 1;
    }

    result
}

// ---------------------------------------------------------------------------
// Search helpers
// ---------------------------------------------------------------------------

/// Find the position of the first occurrence of `delim` (a 2-char sequence)
/// starting from index `start` in `chars`.
///
/// Returns the index of the first character of the delimiter sequence, or
/// `None` if not found.
fn find_closing_seq(chars: &[char], start: usize, delim: &[char; 2]) -> Option<usize> {
    let len = chars.len();
    if len < 2 {
        return None;
    }
    (start..len.saturating_sub(1)).find(|&i| chars[i] == delim[0] && chars[i + 1] == delim[1])
}

/// Parse a `[text](url)` link starting at `open` (the index of `[`).
///
/// Returns `(text_end, url_start, url_end)` on success, where:
/// - `text_end` is the index of the closing `]`
/// - `url_start` is the index of the first character of the URL
/// - `url_end` is the index of the closing `)` (exclusive start of char after URL)
fn find_link(chars: &[char], open: usize) -> Option<(usize, usize, usize)> {
    // Find the `]` after `[`.
    let close_bracket = chars[open + 1..]
        .iter()
        .position(|&c| c == ']')
        .map(|p| p + open + 1)?;

    // Immediately after `]` must come `(`.
    if close_bracket + 1 >= chars.len() || chars[close_bracket + 1] != '(' {
        return None;
    }

    let url_start = close_bracket + 2;

    // Find the closing `)`.
    let url_end = chars[url_start..]
        .iter()
        .position(|&c| c == ')')
        .map(|p| p + url_start)?;

    Some((close_bracket, url_start, url_end))
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // --- html_escape ---

    #[test]
    fn test_html_escape_ampersand() {
        assert_eq!(html_escape("a & b"), "a &amp; b");
    }

    #[test]
    fn test_html_escape_lt_gt() {
        assert_eq!(html_escape("a < b > c"), "a &lt; b &gt; c");
    }

    #[test]
    fn test_html_escape_quote() {
        assert_eq!(html_escape("say \"hi\""), "say &quot;hi&quot;");
    }

    #[test]
    fn test_html_escape_no_change() {
        assert_eq!(html_escape("hello world"), "hello world");
    }

    // --- render_markdown: empty ---

    #[test]
    fn test_empty() {
        assert_eq!(render_markdown(""), "");
    }

    #[test]
    fn test_whitespace_only() {
        assert_eq!(render_markdown("   \n   "), "");
    }

    // --- render_markdown: paragraphs ---

    #[test]
    fn test_plain_text() {
        assert_eq!(render_markdown("hello world"), "<p>hello world</p>\n");
    }

    #[test]
    fn test_paragraphs() {
        let md = "para one\n\npara two";
        let html = render_markdown(md);
        assert_eq!(html.matches("<p>").count(), 2, "should have two paragraphs");
        assert!(html.contains("<p>para one</p>"));
        assert!(html.contains("<p>para two</p>"));
    }

    #[test]
    fn test_multi_line_single_paragraph() {
        // Lines without a blank line between them form one paragraph.
        let md = "line one\nline two\nline three";
        let html = render_markdown(md);
        assert_eq!(html.matches("<p>").count(), 1);
        assert!(html.contains("line one line two line three"));
    }

    // --- render_markdown: bold ---

    #[test]
    fn test_bold() {
        let html = render_markdown("**bold**");
        assert!(html.contains("<strong>bold</strong>"), "got: {html}");
    }

    #[test]
    fn test_bold_in_sentence() {
        let html = render_markdown("This is **important** text.");
        assert!(html.contains("<strong>important</strong>"));
        assert!(html.contains("This is "));
        assert!(html.contains(" text."));
    }

    #[test]
    fn test_unclosed_bold_literal() {
        // Unclosed ** should not produce a <strong> tag.
        let html = render_markdown("**unclosed");
        assert!(!html.contains("<strong>"));
        assert!(html.contains("**unclosed"));
    }

    // --- render_markdown: italic ---

    #[test]
    fn test_italic_star() {
        let html = render_markdown("*italic*");
        assert!(html.contains("<em>italic</em>"), "got: {html}");
    }

    #[test]
    fn test_italic_underscore() {
        let html = render_markdown("_italic_");
        assert!(html.contains("<em>italic</em>"), "got: {html}");
    }

    // --- render_markdown: inline code ---

    #[test]
    fn test_inline_code() {
        let html = render_markdown("`code`");
        assert!(html.contains("<code>code</code>"), "got: {html}");
    }

    #[test]
    fn test_code_html_escaped() {
        let html = render_markdown("`a < b`");
        assert!(html.contains("<code>a &lt; b</code>"), "got: {html}");
    }

    #[test]
    fn test_code_no_inline_processing() {
        // Content inside backticks must NOT be processed as Markdown.
        let html = render_markdown("`**not bold**`");
        assert!(
            html.contains("<code>**not bold**</code>"),
            "bold markers must be escaped inside code, got: {html}"
        );
    }

    // --- render_markdown: fenced code ---

    #[test]
    fn test_fenced_code() {
        let md = "```\nlet x = 1;\n```";
        let html = render_markdown(md);
        assert!(html.contains("<pre><code>"), "got: {html}");
        assert!(html.contains("let x = 1;"), "got: {html}");
        assert!(html.contains("</code></pre>"), "got: {html}");
    }

    #[test]
    fn test_fenced_code_html_escaped() {
        let md = "```\na < b && c > d\n```";
        let html = render_markdown(md);
        assert!(html.contains("&lt;"), "got: {html}");
        assert!(html.contains("&amp;"), "got: {html}");
    }

    #[test]
    fn test_unclosed_fence_treated_as_code() {
        let md = "```\nsome code\n";
        let html = render_markdown(md);
        assert!(html.contains("<pre><code>"), "got: {html}");
        assert!(html.contains("some code"), "got: {html}");
    }

    // --- render_markdown: links ---

    #[test]
    fn test_link_https() {
        let html = render_markdown("[text](https://example.com)");
        assert!(
            html.contains("<a href=\"https://example.com\">text</a>"),
            "got: {html}"
        );
    }

    #[test]
    fn test_link_http() {
        let html = render_markdown("[click](http://foo.com)");
        assert!(html.contains("<a href=\"http://foo.com\">"), "got: {html}");
    }

    #[test]
    fn test_link_fragment() {
        let html = render_markdown("[anchor](#section)");
        assert!(
            html.contains("<a href=\"#section\">anchor</a>"),
            "got: {html}"
        );
    }

    #[test]
    fn test_link_absolute_path() {
        let html = render_markdown("[page](/docs/page)");
        assert!(html.contains("<a href=\"/docs/page\">"), "got: {html}");
    }

    #[test]
    fn test_no_unsafe_url_javascript() {
        let html = render_markdown("[click](javascript:alert(1))");
        assert!(
            !html.contains("<a href"),
            "javascript: URL must not become a link, got: {html}"
        );
    }

    #[test]
    fn test_no_unsafe_url_data() {
        let html = render_markdown("[x](data:text/html,<h1>)");
        assert!(
            !html.contains("<a href"),
            "data: URL must not become a link, got: {html}"
        );
    }

    // --- render_markdown: HTML escaping in plain text ---

    #[test]
    fn test_html_escape_in_text() {
        let html = render_markdown("a < b && c > d");
        assert!(html.contains("&lt;"), "got: {html}");
        assert!(html.contains("&amp;"), "got: {html}");
        assert!(html.contains("&gt;"), "got: {html}");
    }

    // --- render_markdown: combined ---

    #[test]
    fn test_fenced_then_paragraph() {
        let md = "```\ncode here\n```\n\nA paragraph after.";
        let html = render_markdown(md);
        assert!(html.contains("<pre><code>"), "got: {html}");
        assert!(html.contains("<p>A paragraph after.</p>"), "got: {html}");
    }

    #[test]
    fn test_paragraph_then_fenced() {
        let md = "Intro text.\n\n```\ncode here\n```";
        let html = render_markdown(md);
        assert!(html.contains("<p>Intro text.</p>"), "got: {html}");
        assert!(html.contains("<pre><code>"), "got: {html}");
    }

    #[test]
    fn test_bold_italic_code_combined() {
        let html = render_markdown("**bold** and *italic* and `code`");
        assert!(html.contains("<strong>bold</strong>"));
        assert!(html.contains("<em>italic</em>"));
        assert!(html.contains("<code>code</code>"));
    }
}
