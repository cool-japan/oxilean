//! Multi-file HTML documentation generator.
//!
//! Produces one `.html` file per module plus an `index.html` listing all modules
//! from a collection of [`DocItem`]s.  Cross-references in doc-comments (written
//! as `[SymbolName]`) are resolved to relative links between pages.
//!
//! # Output layout
//!
//! ```text
//! <output_dir>/
//!   index.html          ← module listing with links to per-module pages
//!   root.html           ← items in the implicit top-level module
//!   Nat.html            ← items in the `Nat` namespace
//!   List.html           ← items in the `List` namespace
//!   Nat_Ops.html        ← items in `Nat::Ops` (`::`→`_` in file name)
//!   …
//! ```
//!
//! All paths are relative so the output directory can be served from any
//! hosting root without path-prefix configuration.

use crate::cross_ref::resolve_refs;
use crate::error::DocError;
use crate::extractor::DocItem;
use crate::markdown::{html_escape, render_markdown};
use crate::symbol_index::{module_to_page, name_to_anchor, SymbolIndex};
use crate::walker::{group_by_module, ModuleGroup};
use std::path::Path;

// ---------------------------------------------------------------------------
// Public entry point
// ---------------------------------------------------------------------------

/// Generate multi-file HTML documentation for a collection of [`DocItem`]s.
///
/// Writes one `.html` file per distinct module and an `index.html` into
/// `output_dir`.  The directory is created if it does not already exist.
///
/// # Errors
///
/// Returns [`DocError::Io`] on any filesystem failure.
pub fn generate_multi_file(items: &[DocItem], output_dir: &Path) -> Result<(), DocError> {
    let groups = group_by_module(items);
    let index = SymbolIndex::build(&groups);

    std::fs::create_dir_all(output_dir)?;

    // Write one page per module.
    for group in &groups {
        let page_name = module_to_page(&group.module_name);
        let html = render_module_page(group, &index, &page_name)?;
        std::fs::write(output_dir.join(&page_name), html)?;
    }

    // Write the top-level index page.
    let index_html = render_index_page(&groups)?;
    std::fs::write(output_dir.join("index.html"), index_html)?;

    // Write the client-side search index.
    let search_json = index.to_search_json();
    let search_path = output_dir.join("search-index.json");
    std::fs::write(&search_path, search_json.as_bytes())?;

    Ok(())
}

// ---------------------------------------------------------------------------
// Module page renderer
// ---------------------------------------------------------------------------

/// Render a single module's HTML page.
///
/// Uses the existing per-item renderer from [`render_html`](crate::renderer::render_html) for the body, but
/// wraps it in a two-column layout with a left-hand module navigation sidebar
/// and resolves cross-references in every doc-comment.
fn render_module_page(
    group: &ModuleGroup,
    symbol_index: &SymbolIndex,
    page_name: &str,
) -> Result<String, DocError> {
    // Build items with cross-references resolved in their doc-comments.
    let resolved_items: Vec<DocItem> = group
        .items
        .iter()
        .map(|item| {
            let resolved_doc = item
                .doc_comment
                .as_deref()
                .map(|text| resolve_refs(text, symbol_index, page_name));
            DocItem {
                doc_comment: resolved_doc,
                ..item.clone()
            }
        })
        .collect();

    // Use the existing renderer for the body cards (strip the outer HTML wrapper).
    // We call render_html which produces a complete page, then we re-embed it.
    // To avoid double-wrapping we render just the declaration sections inline.
    let mut html = String::with_capacity(8192);

    // ---- HTML head ----
    html.push_str("<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n");
    html.push_str("<meta charset=\"UTF-8\">\n");
    html.push_str("<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\n");
    html.push_str(&format!(
        "<title>Module {} — OxiLean Docs</title>\n",
        html_escape(&group.module_name)
    ));
    html.push_str("<style>\n");
    html.push_str(MULTI_STYLE);
    html.push_str("</style>\n");
    html.push_str(THEME_SCRIPT);
    html.push_str(SEARCH_SCRIPT);
    html.push_str("</head>\n<body>\n");
    html.push_str(THEME_BUTTON);

    // ---- Top nav bar ----
    html.push_str("<header class=\"topnav\">\n");
    html.push_str("  <a class=\"home-link\" href=\"index.html\">&#8962; Index</a>\n");
    html.push_str(&format!(
        "  <span class=\"module-title\">Module {}</span>\n",
        html_escape(&group.module_name)
    ));
    html.push_str("</header>\n");

    // ---- Main content: re-use single-file renderer body ----
    // render_html produces the full page; we embed it via the library function
    // but note resolved doc-comments already contain HTML so we must NOT
    // double-escape them.  We instead build the content from scratch here.
    html.push_str("<main>\n");
    html.push_str(&format!(
        "<h1 class=\"page-title\">Module {}</h1>\n",
        html_escape(&group.module_name)
    ));
    // ---- Search box ----
    html.push_str(SEARCH_BOX_HTML);

    if resolved_items.is_empty() {
        html.push_str("<p class=\"empty\">No declarations found.</p>\n");
    } else {
        html.push_str(&format!(
            "<p class=\"summary\">{} declaration{} documented.</p>\n",
            resolved_items.len(),
            if resolved_items.len() == 1 { "" } else { "s" }
        ));

        // Table of contents (module-local).
        render_module_toc(&mut html, &resolved_items);

        // Declaration cards.
        html.push_str("<div class=\"declarations\">\n");
        for item in &resolved_items {
            render_module_item(&mut html, item);
        }
        html.push_str("</div>\n");
    }

    html.push_str("</main>\n</body>\n</html>\n");

    Ok(html)
}

// ---------------------------------------------------------------------------
// Index page renderer
// ---------------------------------------------------------------------------

/// Render the top-level `index.html` listing all modules.
fn render_index_page(groups: &[ModuleGroup]) -> Result<String, DocError> {
    let mut html = String::with_capacity(4096);

    html.push_str("<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n");
    html.push_str("<meta charset=\"UTF-8\">\n");
    html.push_str("<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\n");
    html.push_str("<title>OxiLean Documentation — Module Index</title>\n");
    html.push_str("<style>\n");
    html.push_str(MULTI_STYLE);
    html.push_str("</style>\n");
    html.push_str(THEME_SCRIPT);
    html.push_str(SEARCH_SCRIPT);
    html.push_str("</head>\n<body>\n");
    html.push_str(THEME_BUTTON);

    html.push_str("<main>\n");
    html.push_str("<h1 class=\"page-title\">Module Index</h1>\n");
    // Search box prominently placed after the page heading.
    html.push_str(SEARCH_BOX_HTML);

    if groups.is_empty() {
        html.push_str("<p class=\"empty\">No modules found.</p>\n");
    } else {
        html.push_str(&format!(
            "<p class=\"summary\">{} module{} documented.</p>\n",
            groups.len(),
            if groups.len() == 1 { "" } else { "s" }
        ));
        html.push_str("<ul class=\"module-list\">\n");
        for group in groups {
            let page = module_to_page(&group.module_name);
            let decl_count = group.items.len();
            html.push_str(&format!(
                "<li><a href=\"{page}\">{name}</a> \
                 <span class=\"decl-count\">{decl_count} declaration{s}</span></li>\n",
                page = html_escape(&page),
                name = html_escape(&group.module_name),
                decl_count = decl_count,
                s = if decl_count == 1 { "" } else { "s" },
            ));
        }
        html.push_str("</ul>\n");
    }

    html.push_str("</main>\n</body>\n</html>\n");

    Ok(html)
}

// ---------------------------------------------------------------------------
// Per-item rendering helpers (module-page variants)
// ---------------------------------------------------------------------------

/// Render the module-local table of contents.
fn render_module_toc(html: &mut String, items: &[DocItem]) {
    html.push_str("<nav class=\"toc\">\n<h2>Contents</h2>\n<ul>\n");
    for item in items {
        let anchor = name_to_anchor(&item.name);
        html.push_str(&format!(
            "<li><a href=\"#{anchor}\">{name}</a></li>\n",
            anchor = html_escape(&anchor),
            name = html_escape(&item.name),
        ));
    }
    html.push_str("</ul>\n</nav>\n");
}

/// Render a single declaration card within a module page.
///
/// The `doc_comment` field may already contain resolved HTML links (from
/// [`resolve_refs`]).  It is then further rendered as Markdown (using
/// [`render_markdown`]) so that bold, italic, inline code, fenced code, and
/// link syntax in the raw doc-comment text are converted to HTML.
fn render_module_item(html: &mut String, item: &DocItem) {
    use crate::extractor::DeclKind;

    let kind_str = match item.kind {
        DeclKind::Def => "def",
        DeclKind::Theorem => "theorem",
        DeclKind::Axiom => "axiom",
        DeclKind::Structure => "structure",
        DeclKind::Inductive => "inductive",
        DeclKind::Other => "decl",
    };

    let anchor = name_to_anchor(&item.name);

    html.push_str(&format!(
        "<section class=\"decl\" id=\"{anchor}\">\n",
        anchor = html_escape(&anchor),
    ));

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
        name = html_escape(&item.name),
    ));

    // Doc comment: already has cross-refs resolved as HTML <a> links (from
    // `resolve_refs`). Pass through `render_markdown` to also handle bold,
    // italic, inline code, fenced blocks, and link syntax.
    if let Some(doc) = &item.doc_comment {
        if !doc.is_empty() {
            let rendered_doc = render_markdown(doc);
            html.push_str("<div class=\"doc\">");
            html.push_str(&rendered_doc);
            html.push_str("</div>\n");
        }
    }

    html.push_str(&format!(
        "<pre class=\"sig\"><code>{}</code></pre>\n",
        html_escape(&item.signature),
    ));

    html.push_str("</section>\n");
}

// ---------------------------------------------------------------------------
// Embedded stylesheet for multi-file pages (CSS custom properties for theming)
// ---------------------------------------------------------------------------

const MULTI_STYLE: &str = r#"
/* ---- Light theme defaults ---- */
:root {
    --bg: #ffffff;
    --fg: #1a1a1a;
    --accent: #2563eb;
    --card-bg: #ffffff;
    --card-border: #dde3f0;
    --toc-bg: #f0f4ff;
    --toc-border: #c8d8ff;
    --toc-heading: #333333;
    --sig-bg: #f5f7fa;
    --sig-border: #e2e8f0;
    --topnav-bg: #1e293b;
    --topnav-fg: #e2e8f0;
    --topnav-link: #93c5fd;
    --module-list-border: #e2e8f0;
    --summary: #555555;
    --empty: #888888;
    --muted: #888888;
    --decl-name: #111111;
    --doc: #444444;
    --doc-code-bg: #f0f4ff;
    --doc-pre-bg: #f5f7fa;
    --doc-pre-border: #e2e8f0;
    --search-border: #cccccc;
    --toggle-bg: #e2e8f0;
    --toggle-fg: #1a1a1a;
}

/* ---- Dark theme (Catppuccin Mocha) ---- */
[data-theme="dark"] {
    --bg: #1e1e2e;
    --fg: #cdd6f4;
    --accent: #89b4fa;
    --card-bg: #181825;
    --card-border: #313244;
    --toc-bg: #181825;
    --toc-border: #45475a;
    --toc-heading: #cdd6f4;
    --sig-bg: #181825;
    --sig-border: #313244;
    --topnav-bg: #11111b;
    --topnav-fg: #cdd6f4;
    --topnav-link: #89b4fa;
    --module-list-border: #45475a;
    --summary: #a6adc8;
    --empty: #6c7086;
    --muted: #6c7086;
    --decl-name: #cdd6f4;
    --doc: #bac2de;
    --doc-code-bg: #313244;
    --doc-pre-bg: #181825;
    --doc-pre-border: #313244;
    --search-border: #45475a;
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
        --toc-bg: #181825;
        --toc-border: #45475a;
        --toc-heading: #cdd6f4;
        --sig-bg: #181825;
        --sig-border: #313244;
        --topnav-bg: #11111b;
        --topnav-fg: #cdd6f4;
        --topnav-link: #89b4fa;
        --module-list-border: #45475a;
        --summary: #a6adc8;
        --empty: #6c7086;
        --muted: #6c7086;
        --decl-name: #cdd6f4;
        --doc: #bac2de;
        --doc-code-bg: #313244;
        --doc-pre-bg: #181825;
        --doc-pre-border: #313244;
        --search-border: #45475a;
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

/* Top navigation bar */
.topnav {
    display: flex;
    align-items: center;
    gap: 1.5rem;
    background: var(--topnav-bg);
    color: var(--topnav-fg);
    padding: 0.6rem 2rem;
    font-size: 0.95rem;
}
.topnav .home-link {
    color: var(--topnav-link);
    text-decoration: none;
    font-weight: 600;
}
.topnav .home-link:hover { text-decoration: underline; }
.topnav .module-title { font-weight: 500; }

/* Main content area */
main {
    max-width: 960px;
    margin: 0 auto;
    padding: 2rem 1.5rem;
}

/* Page title */
h1.page-title {
    font-size: 2rem;
    border-bottom: 3px solid var(--accent);
    padding-bottom: 0.5rem;
    margin-bottom: 1rem;
    color: var(--fg);
}

/* Summary / empty */
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

/* Module index list */
.module-list { list-style: none; padding: 0; }
.module-list li {
    padding: 0.5rem 0;
    border-bottom: 1px solid var(--module-list-border);
    display: flex;
    align-items: center;
    gap: 1rem;
}
.module-list a {
    font-weight: 600;
    color: var(--accent);
    text-decoration: none;
    font-size: 1.05rem;
}
.module-list a:hover { text-decoration: underline; }
.decl-count { color: var(--muted); font-size: 0.875rem; }

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

/* Doc comment (may contain resolved <a href> links and rendered Markdown) */
.doc { color: var(--doc); margin-bottom: 0.75rem; }
.doc p { margin-bottom: 0.5rem; }
.doc a { color: var(--accent); }
.doc pre { background: var(--doc-pre-bg); border: 1px solid var(--doc-pre-border); border-radius: 4px;
           padding: 0.6rem 0.8rem; overflow-x: auto; font-size: 0.85em; margin-bottom: 0.5rem; }
.doc code { font-family: "JetBrains Mono", "Fira Code", "Cascadia Code", monospace;
            background: var(--doc-code-bg); padding: 0.1em 0.3em; border-radius: 3px; font-size: 0.9em; }
.doc pre code { background: none; padding: 0; }

/* Search box */
#search-box { width: 100%; padding: 8px; margin: 8px 0 12px; font-size: 14px;
              border: 1px solid var(--search-border); border-radius: 4px;
              background: var(--card-bg); color: var(--fg); }
#search-results { list-style: none; padding: 0; margin-bottom: 1rem; }
#search-results li { padding: 4px 0; }
#search-results a { color: var(--accent); text-decoration: none; }
#search-results a:hover { text-decoration: underline; }

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

/// JS snippet for theme toggle (same as single-file renderer).
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

/// Vanilla-JS search script embedded in every generated page.
///
/// Loads `search-index.json` from the appropriate relative path, then filters
/// entries by the typed query and injects result links into the `#search-results` list.
const SEARCH_SCRIPT: &str = r#"<script>
(function() {
  var idx = null;
  function loadIndex(base) {
    fetch(base + 'search-index.json')
      .then(function(r) { return r.json(); })
      .then(function(d) { idx = d; })
      .catch(function() { /* search index unavailable — silently skip */ });
  }
  function search(q) {
    if (!idx || !q) return [];
    q = q.toLowerCase();
    return idx.filter(function(e) {
      return e.name.toLowerCase().indexOf(q) !== -1;
    }).slice(0, 20);
  }
  window.addEventListener('DOMContentLoaded', function() {
    var input = document.getElementById('search-input');
    var results = document.getElementById('search-results');
    if (!input || !results) return;
    // Detect base path from current URL depth so the fetch works from
    // both root (index.html) and nested module pages.
    var depth = (window.location.pathname.match(/\//g) || []).length - 1;
    var base = depth > 0 ? '../'.repeat(depth) : './';
    loadIndex(base);
    input.addEventListener('input', function() {
      var hits = search(this.value);
      results.innerHTML = hits.map(function(e) {
        return '<li><a href="' + base + e.page + '#' + e.anchor + '">'
          + e.name + '</a></li>';
      }).join('');
    });
  });
})();
</script>
"#;

/// HTML fragment for the search input box and results list.
const SEARCH_BOX_HTML: &str =
    "<input id=\"search-input\" type=\"search\" placeholder=\"Search symbols\u{2026}\" \
     aria-label=\"Search symbols\" />\n\
     <ul id=\"search-results\"></ul>\n";

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::extractor::{DeclKind, DocItem};

    fn make_item(name: &str, module: &str, doc: Option<&str>) -> DocItem {
        DocItem {
            name: name.to_string(),
            module: module.to_string(),
            kind: DeclKind::Def,
            signature: format!("def {name} := 0"),
            doc_comment: doc.map(str::to_string),
            deprecated: false,
        }
    }

    #[test]
    fn test_multi_file_generates_files() {
        let items = vec![
            make_item("Nat.add", "Nat", Some("Adds two naturals.")),
            make_item("Nat.zero", "Nat", None),
            make_item("List.map", "List", Some("Maps over a list.")),
        ];
        let out = std::env::temp_dir().join("oxilean_doc_test_multi_v1");
        std::fs::remove_dir_all(&out).ok();

        generate_multi_file(&items, &out).expect("multi-file generation should succeed");

        assert!(
            out.join("Nat.html").exists(),
            "Nat module page should exist"
        );
        assert!(
            out.join("List.html").exists(),
            "List module page should exist"
        );
        assert!(out.join("index.html").exists(), "index.html should exist");
    }

    #[test]
    fn test_multi_file_index_lists_all_modules() {
        let items = vec![
            make_item("Nat.add", "Nat", None),
            make_item("List.map", "List", None),
        ];
        let out = std::env::temp_dir().join("oxilean_doc_test_multi_idx");
        std::fs::remove_dir_all(&out).ok();

        generate_multi_file(&items, &out).expect("generation succeeds");

        let index_html =
            std::fs::read_to_string(out.join("index.html")).expect("index.html readable");
        assert!(index_html.contains("Nat.html"), "index links to Nat");
        assert!(index_html.contains("List.html"), "index links to List");
    }

    #[test]
    fn test_cross_reference_resolves() {
        let a_item = make_item("Nat.foo", "Nat", None);
        let b_item = make_item("List.bar", "List", Some("[Nat.foo] is useful here"));
        let groups = group_by_module(&[a_item, b_item]);
        let symbol_index = SymbolIndex::build(&groups);
        let resolved = resolve_refs("[Nat.foo] is useful here", &symbol_index, "List.html");
        assert!(
            resolved.contains("href="),
            "cross-ref should produce an anchor tag"
        );
        assert!(resolved.contains("Nat.foo"), "symbol name preserved");
    }

    #[test]
    fn test_module_page_contains_declarations() {
        let items = vec![
            make_item("Nat.add", "Nat", None),
            make_item("Nat.zero", "Nat", Some("The zero value.")),
        ];
        let out = std::env::temp_dir().join("oxilean_doc_test_multi_page");
        std::fs::remove_dir_all(&out).ok();

        generate_multi_file(&items, &out).expect("generation succeeds");

        let page = std::fs::read_to_string(out.join("Nat.html")).expect("Nat.html readable");
        assert!(page.contains("Nat.add"), "Nat page has Nat.add");
        assert!(page.contains("Nat.zero"), "Nat page has Nat.zero");
        assert!(page.contains("The zero value."), "doc comment present");
    }

    #[test]
    fn test_module_page_links_back_to_index() {
        let items = vec![make_item("Nat.add", "Nat", None)];
        let out = std::env::temp_dir().join("oxilean_doc_test_multi_nav");
        std::fs::remove_dir_all(&out).ok();

        generate_multi_file(&items, &out).expect("generation succeeds");

        let page = std::fs::read_to_string(out.join("Nat.html")).expect("Nat.html readable");
        assert!(
            page.contains("index.html"),
            "module page links back to index"
        );
    }

    #[test]
    fn test_empty_items_produces_index_only() {
        let out = std::env::temp_dir().join("oxilean_doc_test_multi_empty");
        std::fs::remove_dir_all(&out).ok();

        generate_multi_file(&[], &out).expect("empty generation succeeds");

        assert!(out.join("index.html").exists(), "index.html written");
        let index_html = std::fs::read_to_string(out.join("index.html")).expect("readable");
        assert!(index_html.contains("No modules found."));
    }

    #[test]
    fn test_symbol_index_lookup() {
        let groups = vec![ModuleGroup {
            module_name: "Nat".to_string(),
            items: vec![make_item("Nat.add", "Nat", None)],
        }];
        let index = SymbolIndex::build(&groups);
        assert!(index.lookup("Nat.add").is_some());
        assert!(index.lookup("missing").is_none());
    }

    #[test]
    fn test_cross_ref_module_page_cross_link() {
        // Module A defines `Nat.foo`; module B has a doc-comment referencing it.
        let items = vec![
            make_item("Nat.foo", "Nat", None),
            make_item("List.bar", "List", Some("[Nat.foo] is great")),
        ];
        let out = std::env::temp_dir().join("oxilean_doc_test_multi_xref");
        std::fs::remove_dir_all(&out).ok();

        generate_multi_file(&items, &out).expect("generation succeeds");

        let list_page = std::fs::read_to_string(out.join("List.html")).expect("List.html readable");
        // The resolved doc-comment should contain a link to Nat.html
        assert!(
            list_page.contains("Nat.html"),
            "List.html should link to Nat.html for [Nat.foo] ref"
        );
    }

    /// Reproduce the render_html behaviour for a single-file scenario, ensuring
    /// the multi-file module page does not break the existing single-file API.
    #[test]
    fn test_single_file_api_still_works() {
        use crate::renderer::render_html;
        let items = vec![make_item("foo", "root", Some("A simple def."))];
        let html = render_html("root", &items);
        assert!(html.contains("<!DOCTYPE html>"));
        assert!(html.contains("foo"));
    }

    #[test]
    fn test_search_index_json_written() {
        let items = vec![
            make_item("Nat.add", "Nat", None),
            make_item("List.map", "List", None),
        ];
        let out = std::env::temp_dir().join("oxilean_doc_test_search_idx");
        std::fs::remove_dir_all(&out).ok();

        generate_multi_file(&items, &out).expect("generation succeeds");

        let search_path = out.join("search-index.json");
        assert!(search_path.exists(), "search-index.json should be written");

        let json = std::fs::read_to_string(&search_path).expect("search-index.json readable");
        assert!(json.starts_with('['), "JSON must start with [");
        assert!(json.ends_with(']'), "JSON must end with ]");
        assert!(json.contains("Nat.add"), "JSON should contain Nat.add");
        assert!(json.contains("List.map"), "JSON should contain List.map");
        assert!(
            json.contains("Nat.html"),
            "JSON should reference Nat.html page"
        );
    }

    #[test]
    fn test_search_box_in_index_html() {
        let items = vec![make_item("Foo.bar", "Foo", None)];
        let out = std::env::temp_dir().join("oxilean_doc_test_search_box");
        std::fs::remove_dir_all(&out).ok();

        generate_multi_file(&items, &out).expect("generation succeeds");

        let index_html =
            std::fs::read_to_string(out.join("index.html")).expect("index.html readable");
        assert!(
            index_html.contains("search-input"),
            "index.html should have search input"
        );
        assert!(
            index_html.contains("search-results"),
            "index.html should have search results list"
        );
        assert!(
            index_html.contains("search-index.json"),
            "index.html should reference search-index.json"
        );
    }

    #[test]
    fn test_search_box_in_module_page() {
        let items = vec![make_item("Foo.bar", "Foo", None)];
        let out = std::env::temp_dir().join("oxilean_doc_test_search_mod");
        std::fs::remove_dir_all(&out).ok();

        generate_multi_file(&items, &out).expect("generation succeeds");

        let page = std::fs::read_to_string(out.join("Foo.html")).expect("Foo.html readable");
        assert!(
            page.contains("search-input"),
            "module page should have search input"
        );
    }

    #[test]
    fn test_markdown_rendered_in_module_page() {
        // Doc comment contains bold + code; verify the module page HTML renders them.
        let items = vec![make_item(
            "Nat.add",
            "Nat",
            Some("Adds **two** naturals. Use `Nat.zero` as base."),
        )];
        let out = std::env::temp_dir().join("oxilean_doc_test_md_render");
        std::fs::remove_dir_all(&out).ok();

        generate_multi_file(&items, &out).expect("generation succeeds");

        let page = std::fs::read_to_string(out.join("Nat.html")).expect("Nat.html readable");
        assert!(
            page.contains("<strong>two</strong>"),
            "bold should be rendered as <strong>, got page snippet involving 'two'"
        );
        assert!(
            page.contains("<code>Nat.zero</code>"),
            "inline code should be rendered as <code>"
        );
    }

    // ---- Theme tests ----

    #[test]
    fn test_multi_file_theme_variables_in_index() {
        let items = vec![make_item("Foo.bar", "Foo", None)];
        let out = std::env::temp_dir().join("oxilean_doc_test_theme_idx");
        std::fs::remove_dir_all(&out).ok();

        generate_multi_file(&items, &out).expect("generation succeeds");

        let html = std::fs::read_to_string(out.join("index.html")).expect("index.html readable");
        assert!(html.contains("--bg"), "CSS var --bg missing in index.html");
        assert!(
            html.contains("[data-theme=\"dark\"]"),
            "[data-theme=\"dark\"] block missing in index.html"
        );
        assert!(
            html.contains("theme-toggle"),
            "theme-toggle button missing in index.html"
        );
        assert!(
            html.contains("localStorage"),
            "localStorage missing in index.html"
        );
    }

    #[test]
    fn test_multi_file_theme_variables_in_module_page() {
        let items = vec![make_item("Foo.bar", "Foo", None)];
        let out = std::env::temp_dir().join("oxilean_doc_test_theme_mod");
        std::fs::remove_dir_all(&out).ok();

        generate_multi_file(&items, &out).expect("generation succeeds");

        let page = std::fs::read_to_string(out.join("Foo.html")).expect("Foo.html readable");
        assert!(page.contains("--bg"), "CSS var --bg missing in module page");
        assert!(
            page.contains("[data-theme=\"dark\"]"),
            "[data-theme=\"dark\"] block missing in module page"
        );
        assert!(
            page.contains("theme-toggle"),
            "theme-toggle button missing in module page"
        );
    }

    // ---- Deprecated badge tests ----

    #[test]
    fn test_deprecated_item_badge_in_module_page() {
        let mut item = make_item("Foo.old", "Foo", Some("Obsolete."));
        item.deprecated = true;
        let out = std::env::temp_dir().join("oxilean_doc_test_dep_badge");
        std::fs::remove_dir_all(&out).ok();

        generate_multi_file(&[item], &out).expect("generation succeeds");

        let page = std::fs::read_to_string(out.join("Foo.html")).expect("Foo.html readable");
        assert!(
            page.contains("<span class=\"deprecated-badge\">deprecated</span>"),
            "deprecated badge span should appear in module page for deprecated item"
        );
    }

    #[test]
    fn test_non_deprecated_item_no_badge_in_module_page() {
        let item = make_item("Foo.current", "Foo", None);
        let out = std::env::temp_dir().join("oxilean_doc_test_no_dep_badge");
        std::fs::remove_dir_all(&out).ok();

        generate_multi_file(&[item], &out).expect("generation succeeds");

        let page = std::fs::read_to_string(out.join("Foo.html")).expect("Foo.html readable");
        // The CSS defines `.deprecated-badge` but the badge span element must not appear
        // for non-deprecated items.
        assert!(
            !page.contains("<span class=\"deprecated-badge\">"),
            "non-deprecated item must not have a deprecated badge span element in module page"
        );
    }
}
