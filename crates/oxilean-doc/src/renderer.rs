//! HTML renderer for OxiLean documentation items.
//!
//! Generates a self-contained single-page HTML file from a list of
//! [`DocItem`]s.  All styling is embedded inline — no external resources.

use crate::extractor::{DeclKind, DocItem};

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/// Render a list of [`DocItem`]s into a complete, self-contained HTML page.
///
/// `title` is used for both the `<title>` element and the page `<h1>`.
pub fn render_html(title: &str, items: &[DocItem]) -> String {
    let mut html = String::with_capacity(4096 + items.len() * 512);

    // Head
    html.push_str("<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n");
    html.push_str("<meta charset=\"UTF-8\">\n");
    html.push_str("<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\n");
    html.push_str(&format!("<title>{}</title>\n", escape_html(title)));
    html.push_str("<style>\n");
    html.push_str(STYLE);
    html.push_str("</style>\n");
    html.push_str(THEME_SCRIPT);
    html.push_str("</head>\n<body>\n");
    html.push_str(THEME_BUTTON);

    // Page header
    html.push_str(&format!(
        "<h1 class=\"page-title\">{}</h1>\n",
        escape_html(title)
    ));

    if items.is_empty() {
        html.push_str("<p class=\"empty\">No declarations found.</p>\n");
    } else {
        // Summary count
        html.push_str(&format!(
            "<p class=\"summary\">{} declaration{} documented.</p>\n",
            items.len(),
            if items.len() == 1 { "" } else { "s" }
        ));

        // Table of contents
        render_toc(&mut html, items);

        // Declaration cards
        html.push_str("<div class=\"declarations\">\n");
        for item in items {
            render_item(&mut html, item);
        }
        html.push_str("</div>\n");
    }

    html.push_str("</body>\n</html>\n");
    html
}

// ---------------------------------------------------------------------------
// Private helpers
// ---------------------------------------------------------------------------

/// Render a compact table-of-contents list before the full declarations.
fn render_toc(html: &mut String, items: &[DocItem]) {
    html.push_str("<nav class=\"toc\">\n<h2>Contents</h2>\n<ul>\n");
    for item in items {
        let kind_str = kind_label(&item.kind);
        html.push_str(&format!(
            "<li><a href=\"#{name}\"><span class=\"toc-kind\">{kind}</span> {name}</a></li>\n",
            name = escape_html(&item.name),
            kind = kind_str,
        ));
    }
    html.push_str("</ul>\n</nav>\n");
}

/// Render a single declaration card.
fn render_item(html: &mut String, item: &DocItem) {
    let kind_str = kind_label(&item.kind);

    html.push_str(&format!(
        "<section class=\"decl\" id=\"{name}\">\n",
        name = escape_html(&item.name),
    ));

    // Heading row: [kind badge] [deprecated badge?] declaration-name
    let deprecated_badge = if item.deprecated {
        " <span class=\"deprecated-badge\">deprecated</span>"
    } else {
        ""
    };
    html.push_str(&format!(
        "<h2><span class=\"badge badge-{kind}\">{kind}</span>{dep} \
         <code class=\"decl-name\">{name}</code></h2>\n",
        kind = kind_str,
        dep = deprecated_badge,
        name = escape_html(&item.name),
    ));

    // Optional doc comment
    if let Some(doc) = &item.doc_comment {
        if !doc.is_empty() {
            html.push_str(&format!("<p class=\"doc\">{}</p>\n", escape_html(doc)));
        }
    }

    // Signature block
    html.push_str(&format!(
        "<pre class=\"sig\"><code>{}</code></pre>\n",
        escape_html(&item.signature),
    ));

    html.push_str("</section>\n");
}

/// Return the lower-case label for a [`DeclKind`].
fn kind_label(kind: &DeclKind) -> &'static str {
    match kind {
        DeclKind::Def => "def",
        DeclKind::Theorem => "theorem",
        DeclKind::Axiom => "axiom",
        DeclKind::Structure => "structure",
        DeclKind::Inductive => "inductive",
        DeclKind::Other => "decl",
    }
}

/// Escape HTML special characters so that user-supplied strings are safe.
pub(crate) fn escape_html(s: &str) -> String {
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

// ---------------------------------------------------------------------------
// Embedded stylesheet (CSS custom properties for light/dark theming)
// ---------------------------------------------------------------------------

const STYLE: &str = r#"
/* ---- Light theme defaults ---- */
:root {
    --bg: #fafafa;
    --fg: #1a1a1a;
    --accent: #3b6cf4;
    --card-bg: #ffffff;
    --card-border: #dde3f0;
    --toc-bg: #f0f4ff;
    --toc-border: #c8d8ff;
    --toc-heading: #333333;
    --toc-kind: #666666;
    --sig-bg: #f5f7fa;
    --sig-border: #e2e8f0;
    --summary: #555555;
    --empty: #888888;
    --muted: #888888;
    --decl-name: #111111;
    --doc: #444444;
    --toggle-bg: #e2e8f0;
    --toggle-fg: #1a1a1a;
}

/* ---- Dark theme (explicit attribute) ---- */
[data-theme="dark"] {
    --bg: #1e1e2e;
    --fg: #cdd6f4;
    --accent: #89b4fa;
    --card-bg: #181825;
    --card-border: #313244;
    --toc-bg: #1e1e2e;
    --toc-border: #45475a;
    --toc-heading: #cdd6f4;
    --toc-kind: #a6adc8;
    --sig-bg: #181825;
    --sig-border: #313244;
    --summary: #a6adc8;
    --empty: #6c7086;
    --muted: #6c7086;
    --decl-name: #cdd6f4;
    --doc: #bac2de;
    --toggle-bg: #313244;
    --toggle-fg: #cdd6f4;
}

/* ---- System dark mode (when no explicit theme set) ---- */
@media (prefers-color-scheme: dark) {
    :root {
        --bg: #1e1e2e;
        --fg: #cdd6f4;
        --accent: #89b4fa;
        --card-bg: #181825;
        --card-border: #313244;
        --toc-bg: #1e1e2e;
        --toc-border: #45475a;
        --toc-heading: #cdd6f4;
        --toc-kind: #a6adc8;
        --sig-bg: #181825;
        --sig-border: #313244;
        --summary: #a6adc8;
        --empty: #6c7086;
        --muted: #6c7086;
        --decl-name: #cdd6f4;
        --doc: #bac2de;
        --toggle-bg: #313244;
        --toggle-fg: #cdd6f4;
    }
}

/* Reset & base */
*, *::before, *::after { box-sizing: border-box; margin: 0; padding: 0; }
body {
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif;
    font-size: 16px;
    line-height: 1.6;
    color: var(--fg);
    background: var(--bg);
    max-width: 960px;
    margin: 0 auto;
    padding: 2rem 1.5rem;
}

/* Theme toggle button */
#theme-toggle {
    position: fixed;
    top: 1rem;
    right: 1rem;
    padding: 0.35em 0.75em;
    border: none;
    border-radius: 4px;
    background: var(--toggle-bg);
    color: var(--toggle-fg);
    cursor: pointer;
    font-size: 0.85rem;
    z-index: 100;
}
#theme-toggle:hover { opacity: 0.85; }

/* Page title */
h1.page-title {
    font-size: 2rem;
    border-bottom: 3px solid var(--accent);
    padding-bottom: 0.5rem;
    margin-bottom: 1rem;
    color: var(--fg);
}

/* Summary line */
.summary { color: var(--summary); margin-bottom: 1.5rem; }
.empty { color: var(--empty); font-style: italic; }

/* Table of contents */
.toc {
    background: var(--toc-bg);
    border: 1px solid var(--toc-border);
    border-radius: 6px;
    padding: 1rem 1.25rem;
    margin-bottom: 2rem;
}
.toc h2 { font-size: 1rem; margin-bottom: 0.5rem; color: var(--toc-heading); }
.toc ul { list-style: none; padding-left: 0.5rem; }
.toc li { margin: 0.2rem 0; }
.toc a { text-decoration: none; color: var(--accent); }
.toc a:hover { text-decoration: underline; }
.toc-kind {
    font-size: 0.75em;
    color: var(--toc-kind);
    font-variant: small-caps;
    margin-right: 0.25rem;
}

/* Declaration cards */
.declarations { display: flex; flex-direction: column; gap: 1.25rem; }
.decl {
    background: var(--card-bg);
    border: 1px solid var(--card-border);
    border-radius: 6px;
    padding: 1.25rem 1.5rem;
    scroll-margin-top: 1rem;
}
.decl h2 {
    font-size: 1.15rem;
    margin-bottom: 0.6rem;
    display: flex;
    align-items: center;
    gap: 0.5rem;
}

/* Kind badges */
.badge {
    display: inline-block;
    font-size: 0.7em;
    font-weight: 600;
    letter-spacing: 0.04em;
    text-transform: uppercase;
    padding: 0.15em 0.45em;
    border-radius: 3px;
    color: #fff;
}
.badge-def       { background: #2e7d32; }
.badge-theorem   { background: #1565c0; }
.badge-axiom     { background: #6a1a6a; }
.badge-structure { background: #b45309; }
.badge-inductive { background: #b45309; }
.badge-decl      { background: #546e7a; }

/* Deprecated badge */
.deprecated-badge {
    display: inline-block;
    font-size: 0.7em;
    font-weight: 600;
    letter-spacing: 0.04em;
    text-transform: uppercase;
    padding: 0.15em 0.45em;
    border-radius: 3px;
    background: #f38ba8;
    color: #1e1e2e;
}
del { text-decoration: line-through; color: var(--muted); }

/* Decl name */
code.decl-name { font-size: 1em; font-weight: 600; color: var(--decl-name); }

/* Doc comment */
.doc { color: var(--doc); margin-bottom: 0.75rem; white-space: pre-wrap; }

/* Signature block */
.sig {
    background: var(--sig-bg);
    border: 1px solid var(--sig-border);
    border-radius: 4px;
    padding: 0.75rem 1rem;
    overflow-x: auto;
    font-size: 0.9em;
    line-height: 1.5;
}
.sig code { font-family: "JetBrains Mono", "Fira Code", "Cascadia Code", monospace; }
"#;

/// JS snippet that reads/writes localStorage and toggles the `data-theme` attribute.
const THEME_SCRIPT: &str = r#"<script>
(function() {
  var saved = localStorage.getItem("theme");
  if (saved) { document.documentElement.setAttribute("data-theme", saved); }
  window.addEventListener("DOMContentLoaded", function() {
    var btn = document.getElementById("theme-toggle");
    if (!btn) return;
    btn.textContent = (document.documentElement.getAttribute("data-theme") === "dark") ? "Light" : "Dark";
    btn.addEventListener("click", function() {
      var current = document.documentElement.getAttribute("data-theme");
      var next = (current === "dark") ? "light" : "dark";
      document.documentElement.setAttribute("data-theme", next);
      localStorage.setItem("theme", next);
      btn.textContent = (next === "dark") ? "Light" : "Dark";
    });
  });
})();
</script>
"#;

/// Theme toggle button HTML fragment injected at the top of `<body>`.
const THEME_BUTTON: &str =
    "<button id=\"theme-toggle\" aria-label=\"Toggle theme\">Dark</button>\n";

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::extractor::{DeclKind, DocItem};

    fn make_item(name: &str, kind: DeclKind, sig: &str, doc: Option<&str>) -> DocItem {
        DocItem {
            name: name.to_string(),
            module: "root".to_string(),
            kind,
            signature: sig.to_string(),
            doc_comment: doc.map(str::to_string),
            deprecated: false,
        }
    }

    #[test]
    fn test_empty_items_produces_valid_html() {
        let html = render_html("Empty Module", &[]);
        assert!(html.contains("<!DOCTYPE html>"));
        assert!(html.contains("<title>Empty Module</title>"));
        assert!(html.contains("No declarations found."));
    }

    #[test]
    fn test_single_def_renders_correctly() {
        let items = [make_item(
            "double",
            DeclKind::Def,
            "def double (n : Nat) : Nat := n + n",
            Some("Doubles a natural number."),
        )];
        let html = render_html("MyModule", &items);
        assert!(html.contains("<!DOCTYPE html>"));
        assert!(html.contains("id=\"double\""));
        assert!(html.contains("badge-def"));
        assert!(html.contains("Doubles a natural number."));
        assert!(html.contains("def double"));
    }

    #[test]
    fn test_html_escape_in_names() {
        let items = [make_item(
            "foo<bar>",
            DeclKind::Other,
            "decl foo<bar>",
            None,
        )];
        let html = render_html("Test", &items);
        // Name must be escaped, raw `<` must not appear in id/content context.
        assert!(html.contains("foo&lt;bar&gt;"));
        assert!(!html.contains("<bar>"));
    }

    #[test]
    fn test_toc_contains_all_declarations() {
        let items = [
            make_item("foo", DeclKind::Def, "def foo := 0", None),
            make_item(
                "bar",
                DeclKind::Theorem,
                "theorem bar : True := trivial",
                None,
            ),
        ];
        let html = render_html("Toc Test", &items);
        assert!(html.contains("href=\"#foo\""));
        assert!(html.contains("href=\"#bar\""));
    }

    #[test]
    fn test_summary_count_singular() {
        let items = [make_item("x", DeclKind::Axiom, "axiom x : Nat", None)];
        let html = render_html("T", &items);
        assert!(html.contains("1 declaration documented."));
    }

    #[test]
    fn test_summary_count_plural() {
        let items = [
            make_item("a", DeclKind::Def, "def a := 1", None),
            make_item("b", DeclKind::Def, "def b := 2", None),
        ];
        let html = render_html("T", &items);
        assert!(html.contains("2 declarations documented."));
    }

    // ---- Theme tests ----

    #[test]
    fn test_theme_css_variables_present() {
        let html = render_html("ThemeTest", &[]);
        // Light+dark CSS custom properties must appear in the stylesheet.
        assert!(html.contains("--bg"), "CSS var --bg missing");
        assert!(
            html.contains("[data-theme=\"dark\"]"),
            "[data-theme=\"dark\"] block missing"
        );
        assert!(
            html.contains("prefers-color-scheme"),
            "@media prefers-color-scheme missing"
        );
    }

    #[test]
    fn test_theme_toggle_present() {
        let html = render_html("ThemeTest", &[]);
        assert!(html.contains("theme-toggle"), "theme-toggle button missing");
        assert!(
            html.contains("localStorage"),
            "localStorage reference missing"
        );
    }

    // ---- Deprecated badge tests ----

    #[test]
    fn test_deprecated_item_shows_badge() {
        let item = DocItem {
            name: "old_fn".to_string(),
            module: "root".to_string(),
            kind: DeclKind::Def,
            signature: "def old_fn := 0".to_string(),
            doc_comment: None,
            deprecated: true,
        };
        let html = render_html("Test", &[item]);
        assert!(
            html.contains("<span class=\"deprecated-badge\">deprecated</span>"),
            "deprecated badge span should be rendered for deprecated item"
        );
    }

    #[test]
    fn test_non_deprecated_item_no_badge() {
        let item = make_item("good_fn", DeclKind::Def, "def good_fn := 0", None);
        let html = render_html("Test", &[item]);
        // The CSS defines `.deprecated-badge` but it must not appear as an element in the HTML body.
        // Check that the badge span is not injected for a non-deprecated item.
        assert!(
            !html.contains("<span class=\"deprecated-badge\">"),
            "non-deprecated item must not have a deprecated badge span element"
        );
    }
}
