//! Cross-reference resolution for intra-crate doc-comment links.
//!
//! Scans text for `[Name]`-style references and replaces them with
//! relative HTML anchor tags using the [`SymbolIndex`].

use crate::renderer::escape_html;
use crate::symbol_index::SymbolIndex;

// ---------------------------------------------------------------------------
// Public entry point
// ---------------------------------------------------------------------------

/// Resolve `[Name]`-style references in `text` to relative HTML links.
///
/// # Arguments
///
/// * `text`      – raw doc-comment text that may contain `[Name]` references.
/// * `index`     – the symbol index built from all module groups.
/// * `from_page` – the HTML page that owns this doc-comment (e.g. `"List.html"`).
///   Used to compute relative links: if the target is on the same
///   page we emit `#anchor`, otherwise we emit `target_page.html#anchor`.
///
/// Unknown references (names not in the index) are left as literal `[Name]` text.
///
/// # HTML safety
///
/// All text that is *not* inside a resolved link is passed through [`escape_html`]
/// so the caller does not need to re-escape the returned string.
pub fn resolve_refs(text: &str, index: &SymbolIndex, from_page: &str) -> String {
    let mut out = String::with_capacity(text.len() * 2);
    let bytes = text.as_bytes();
    let len = bytes.len();
    let mut pos = 0;

    while pos < len {
        // Look for the start of a potential `[Name]` reference.
        if bytes[pos] == b'[' {
            if let Some(end) = find_closing_bracket(bytes, pos + 1) {
                let name = &text[pos + 1..end];
                // Only treat as a cross-reference if the name is non-empty and
                // contains no whitespace (avoids false-positives on markdown links).
                if !name.is_empty() && !name.contains(|c: char| c.is_ascii_whitespace()) {
                    if let Some(loc) = index.lookup(name) {
                        // Build the href: same page → fragment only; other page → relative path.
                        let href = if loc.page == from_page {
                            format!("#{}", loc.anchor)
                        } else {
                            format!("{}#{}", loc.page, loc.anchor)
                        };
                        out.push_str(&format!(
                            "<a href=\"{}\">{}</a>",
                            escape_html(&href),
                            escape_html(name),
                        ));
                        pos = end + 1; // skip past the `]`
                        continue;
                    }
                }
                // Unknown reference — emit literally with HTML escaping.
                out.push_str(&escape_html(&text[pos..end + 1]));
                pos = end + 1;
                continue;
            }
        }
        // Not a `[`, or no closing `]` found — emit this byte escaped.
        let ch_end = next_char_boundary(text, pos);
        out.push_str(&escape_html(&text[pos..ch_end]));
        pos = ch_end;
    }

    out
}

// ---------------------------------------------------------------------------
// Private helpers
// ---------------------------------------------------------------------------

/// Return the byte index of the first `]` after `start`, or `None`.
fn find_closing_bracket(bytes: &[u8], start: usize) -> Option<usize> {
    bytes[start..]
        .iter()
        .position(|&b| b == b']')
        .map(|i| start + i)
}

/// Return the byte index of the start of the next UTF-8 character after `pos`.
fn next_char_boundary(s: &str, pos: usize) -> usize {
    let bytes = s.as_bytes();
    let mut i = pos + 1;
    while i < bytes.len() && (bytes[i] & 0xC0) == 0x80 {
        i += 1;
    }
    i.min(bytes.len())
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::extractor::{DeclKind, DocItem};
    use crate::walker::{group_by_module, ModuleGroup};

    fn build_index_from_groups(groups: &[ModuleGroup]) -> SymbolIndex {
        SymbolIndex::build(groups)
    }

    fn item(name: &str, module: &str) -> DocItem {
        DocItem {
            name: name.to_string(),
            module: module.to_string(),
            kind: DeclKind::Def,
            signature: format!("def {name} := 0"),
            doc_comment: None,
            deprecated: false,
        }
    }

    #[test]
    fn test_cross_ref_to_other_page() {
        let items = vec![item("Nat.foo", "Nat"), item("List.bar", "List")];
        let groups = group_by_module(&items);
        let index = build_index_from_groups(&groups);

        let resolved = resolve_refs("[Nat.foo] is useful here", &index, "List.html");
        assert!(resolved.contains("href="), "should produce a link");
        assert!(resolved.contains("Nat.html"), "should point to Nat.html");
        assert!(resolved.contains("Nat.foo"), "symbol name preserved");
    }

    #[test]
    fn test_cross_ref_same_page() {
        let items = vec![item("Nat.add", "Nat"), item("Nat.zero", "Nat")];
        let groups = group_by_module(&items);
        let index = build_index_from_groups(&groups);

        let resolved = resolve_refs("[Nat.zero] see also", &index, "Nat.html");
        // Same-page ref: href starts with `#`
        assert!(resolved.contains("href=\"#"), "same-page ref uses fragment");
        assert!(resolved.contains("Nat.zero"), "symbol name preserved");
    }

    #[test]
    fn test_unknown_ref_preserved_literally() {
        let index = SymbolIndex::build(&[]);
        let resolved = resolve_refs("See [Unknown.Sym] for details.", &index, "root.html");
        // Not a link since the symbol is unknown.
        assert!(
            !resolved.contains("<a href"),
            "unknown ref should not become a link"
        );
        assert!(
            resolved.contains("Unknown.Sym"),
            "unknown ref text preserved"
        );
    }

    #[test]
    fn test_plain_text_html_escaped() {
        let index = SymbolIndex::build(&[]);
        let resolved = resolve_refs("x < y & z > w", &index, "root.html");
        assert!(resolved.contains("&lt;"), "< should be escaped");
        assert!(resolved.contains("&amp;"), "& should be escaped");
        assert!(resolved.contains("&gt;"), "> should be escaped");
    }

    #[test]
    fn test_multiple_refs_in_one_string() {
        let items = vec![item("Nat.add", "Nat"), item("Nat.zero", "Nat")];
        let groups = group_by_module(&items);
        let index = build_index_from_groups(&groups);

        let resolved = resolve_refs("Use [Nat.add] or [Nat.zero] here.", &index, "List.html");
        assert_eq!(
            resolved.matches("<a href").count(),
            2,
            "both refs should become links"
        );
    }

    #[test]
    fn test_empty_brackets_not_resolved() {
        let index = SymbolIndex::build(&[]);
        let resolved = resolve_refs("See [] for details.", &index, "root.html");
        // Empty brackets are not a valid ref — emitted literally.
        assert!(!resolved.contains("<a href"), "empty brackets not a link");
    }

    #[test]
    fn test_ref_with_whitespace_not_resolved() {
        let index = SymbolIndex::build(&[]);
        let resolved = resolve_refs("See [has space] for details.", &index, "root.html");
        assert!(!resolved.contains("<a href"), "whitespace ref not resolved");
    }
}
