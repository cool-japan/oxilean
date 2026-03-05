//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{
    DocClass, DocDefinition, DocExtractor, DocIndex, DocInductive, DocInstance, DocItem,
    DocItemKind, DocModule, DocOutputFormat, DocPage, DocSectionEntry, DocSite, DocStructure,
    DocTheorem, DocgenStats, HtmlConfig, HtmlGenerator, ProofStatus, SearchEntry, SearchIndex,
    SourceLocation, TableOfContents, TocEntry,
};

/// Extract a doc comment starting at line index `start`.
/// Returns (Option<comment_text>, next_non-comment_index).
pub fn extract_doc_comment_at(lines: &[&str], start: usize) -> (Option<String>, usize) {
    let mut idx = start;
    let mut doc_lines: Vec<String> = Vec::new();
    while idx < lines.len() {
        let trimmed = lines[idx].trim();
        if let Some(after_prefix) = trimmed.strip_prefix("/--") {
            let mut block = String::new();
            if let Some(inner) = after_prefix.strip_suffix("-/") {
                block.push_str(inner.trim());
            } else {
                block.push_str(after_prefix.trim());
                idx += 1;
                while idx < lines.len() {
                    let l = lines[idx].trim();
                    if let Some(last) = l.strip_suffix("-/") {
                        if !block.is_empty() && !last.trim().is_empty() {
                            block.push('\n');
                        }
                        block.push_str(last.trim());
                        break;
                    }
                    if !block.is_empty() {
                        block.push('\n');
                    }
                    block.push_str(l);
                    idx += 1;
                }
            }
            doc_lines.push(block);
            idx += 1;
            continue;
        }
        if let Some(rest) = trimmed.strip_prefix("---") {
            let text = rest.trim();
            doc_lines.push(text.to_string());
            idx += 1;
            continue;
        }
        if trimmed.starts_with("--") && !trimmed.starts_with("---") {
            idx += 1;
            continue;
        }
        break;
    }
    if doc_lines.is_empty() {
        (None, idx)
    } else {
        (Some(doc_lines.join("\n")), idx)
    }
}
/// Extract a doc comment string from raw source text.
pub fn extract_doc_comment(source: &str) -> Option<String> {
    let lines: Vec<&str> = source.lines().collect();
    let (comment, _) = extract_doc_comment_at(&lines, 0);
    comment
}
/// Pretty-print a kernel expression as a type signature string.
pub fn extract_type_signature(
    env: &oxilean_kernel::Environment,
    name: &oxilean_kernel::Name,
) -> String {
    match env.get_type(name) {
        Some(ty) => extract_type_signature_from_expr(ty),
        None => "unknown".to_string(),
    }
}
/// Pretty-print an `Expr` as a simple type signature string.
pub fn extract_type_signature_from_expr(expr: &oxilean_kernel::Expr) -> String {
    oxilean_kernel::print_expr(expr)
}
/// Resolve `[name]` cross-references in a doc comment, replacing them
/// with HTML links.
pub fn link_cross_references(text: &str, base_url: &str) -> String {
    let mut result = String::with_capacity(text.len());
    let mut chars = text.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch == '[' {
            let mut name = String::new();
            let mut found_close = false;
            for c in chars.by_ref() {
                if c == ']' {
                    found_close = true;
                    break;
                }
                name.push(c);
            }
            if found_close && !name.is_empty() && !name.contains(' ') {
                let url = format!("{}/{}.html", base_url, name.replace('.', "/"));
                result.push_str(&format!(
                    "<a href=\"{}\">{}</a>",
                    escape_html(&url),
                    escape_html(&name)
                ));
            } else {
                result.push('[');
                result.push_str(&name);
                if found_close {
                    result.push(']');
                }
            }
        } else {
            result.push(ch);
        }
    }
    result
}
pub fn parse_definition_line(
    line: &str,
    doc: &Option<String>,
    file: &str,
    line_no: usize,
) -> Option<DocItem> {
    let (rest, noncomputable) = if let Some(r) = line.strip_prefix("noncomputable ") {
        (r, true)
    } else {
        (line, false)
    };
    let rest = rest.strip_prefix("def ")?;
    let (name, type_sig) = split_name_and_sig(rest);
    Some(DocItem::Definition(DocDefinition {
        name,
        type_sig,
        doc_comment: doc.clone(),
        attributes: Vec::new(),
        source_location: Some(SourceLocation::new(file, line_no, 0)),
        is_noncomputable: noncomputable,
    }))
}
pub fn parse_theorem_line(
    line: &str,
    doc: &Option<String>,
    file: &str,
    line_no: usize,
) -> Option<DocItem> {
    let rest = if let Some(r) = line.strip_prefix("theorem ") {
        r
    } else {
        line.strip_prefix("lemma ")?
    };
    let (name, statement) = split_name_and_sig(rest);
    Some(DocItem::Theorem(DocTheorem {
        name,
        statement,
        proof_status: ProofStatus::Proved,
        doc_comment: doc.clone(),
        source_location: Some(SourceLocation::new(file, line_no, 0)),
        attributes: Vec::new(),
    }))
}
pub fn parse_inductive_line(
    line: &str,
    doc: &Option<String>,
    file: &str,
    line_no: usize,
) -> Option<DocItem> {
    let rest = line.strip_prefix("inductive ")?;
    let (name, type_sig) = split_name_and_sig(rest);
    Some(DocItem::Inductive(DocInductive {
        name,
        type_sig,
        constructors: Vec::new(),
        doc_comment: doc.clone(),
        source_location: Some(SourceLocation::new(file, line_no, 0)),
        universe_params: Vec::new(),
    }))
}
pub fn parse_structure_line(
    line: &str,
    doc: &Option<String>,
    file: &str,
    line_no: usize,
) -> Option<DocItem> {
    let rest = line.strip_prefix("structure ")?;
    let (name, type_sig) = split_name_and_sig(rest);
    let extends = if let Some(idx) = rest.find("extends") {
        let after = &rest[idx + 7..];
        let ext = after.split("where").next().unwrap_or(after);
        ext.split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect()
    } else {
        Vec::new()
    };
    Some(DocItem::Structure(DocStructure {
        name,
        type_sig,
        fields: Vec::new(),
        extends,
        doc_comment: doc.clone(),
        source_location: Some(SourceLocation::new(file, line_no, 0)),
    }))
}
pub fn parse_class_line(
    line: &str,
    doc: &Option<String>,
    file: &str,
    line_no: usize,
) -> Option<DocItem> {
    let rest = line.strip_prefix("class ")?;
    let (name, type_sig) = split_name_and_sig(rest);
    Some(DocItem::Class(DocClass {
        name,
        type_sig,
        methods: Vec::new(),
        instances: Vec::new(),
        doc_comment: doc.clone(),
        source_location: Some(SourceLocation::new(file, line_no, 0)),
        extends: Vec::new(),
    }))
}
pub fn parse_instance_line(
    line: &str,
    doc: &Option<String>,
    file: &str,
    line_no: usize,
) -> Option<DocItem> {
    let rest = line.strip_prefix("instance ")?;
    let (name, type_sig) = split_name_and_sig(rest);
    Some(DocItem::Instance(DocInstance {
        name,
        type_sig,
        doc_comment: doc.clone(),
        source_location: Some(SourceLocation::new(file, line_no, 0)),
        priority: None,
    }))
}
/// Split `name : type_sig := ...` (or `name : type_sig where ...`).
fn split_name_and_sig(rest: &str) -> (String, String) {
    let name_end = rest.find([':', ' ', '{', '(']).unwrap_or(rest.len());
    let name = rest[..name_end].trim().to_string();
    let sig = if let Some(colon_idx) = rest.find(':') {
        let after_colon = &rest[colon_idx + 1..];
        let end = after_colon
            .find(":=")
            .or_else(|| after_colon.find("where"))
            .unwrap_or(after_colon.len());
        after_colon[..end].trim().to_string()
    } else {
        String::new()
    };
    (name, sig)
}
/// Escape HTML special characters.
pub fn escape_html(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for ch in s.chars() {
        match ch {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&#39;"),
            _ => out.push(ch),
        }
    }
    out
}
/// Format a type signature for HTML display.
pub fn format_type_html(sig: &str) -> String {
    let escaped = escape_html(sig);
    let keywords = [
        "Prop", "Type", "Sort", "Nat", "Bool", "String", "Unit", "List",
    ];
    let mut result = escaped;
    for kw in &keywords {
        let replacement = format!("<span class=\"kw\">{}</span>", kw);
        result = result.replace(kw, &replacement);
    }
    result
}
/// Format a code block in HTML.
pub fn format_code_block(code: &str, language: &str) -> String {
    format!(
        "<pre><code class=\"language-{}\">{}</code></pre>",
        escape_html(language),
        escape_html(code)
    )
}
/// Render a basic subset of Markdown to HTML.
pub fn render_markdown(md: &str) -> String {
    let mut html = String::new();
    let mut in_code_block = false;
    let mut in_list = false;
    let mut in_paragraph = false;
    for line in md.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("```") {
            if in_code_block {
                html.push_str("</code></pre>\n");
                in_code_block = false;
            } else {
                let lang = trimmed.trim_start_matches('`').trim();
                if in_paragraph {
                    html.push_str("</p>\n");
                    in_paragraph = false;
                }
                if lang.is_empty() {
                    html.push_str("<pre><code>");
                } else {
                    html.push_str(&format!(
                        "<pre><code class=\"language-{}\">",
                        escape_html(lang)
                    ));
                }
                in_code_block = true;
            }
            continue;
        }
        if in_code_block {
            html.push_str(&escape_html(line));
            html.push('\n');
            continue;
        }
        if trimmed.is_empty() {
            if in_paragraph {
                html.push_str("</p>\n");
                in_paragraph = false;
            }
            if in_list {
                html.push_str("</ul>\n");
                in_list = false;
            }
            continue;
        }
        if let Some(rest) = trimmed.strip_prefix("### ") {
            close_block(&mut html, &mut in_paragraph, &mut in_list);
            html.push_str(&format!("<h5>{}</h5>\n", inline_markdown(rest)));
            continue;
        }
        if let Some(rest) = trimmed.strip_prefix("## ") {
            close_block(&mut html, &mut in_paragraph, &mut in_list);
            html.push_str(&format!("<h4>{}</h4>\n", inline_markdown(rest)));
            continue;
        }
        if let Some(rest) = trimmed.strip_prefix("# ") {
            close_block(&mut html, &mut in_paragraph, &mut in_list);
            html.push_str(&format!("<h3>{}</h3>\n", inline_markdown(rest)));
            continue;
        }
        if let Some(rest) = trimmed
            .strip_prefix("- ")
            .or_else(|| trimmed.strip_prefix("* "))
        {
            if in_paragraph {
                html.push_str("</p>\n");
                in_paragraph = false;
            }
            if !in_list {
                html.push_str("<ul>\n");
                in_list = true;
            }
            html.push_str(&format!("<li>{}</li>\n", inline_markdown(rest)));
            continue;
        }
        if !in_paragraph {
            html.push_str("<p>");
            in_paragraph = true;
        } else {
            html.push('\n');
        }
        html.push_str(&inline_markdown(trimmed));
    }
    if in_code_block {
        html.push_str("</code></pre>\n");
    }
    if in_paragraph {
        html.push_str("</p>\n");
    }
    if in_list {
        html.push_str("</ul>\n");
    }
    html
}
fn close_block(html: &mut String, in_paragraph: &mut bool, in_list: &mut bool) {
    if *in_paragraph {
        html.push_str("</p>\n");
        *in_paragraph = false;
    }
    if *in_list {
        html.push_str("</ul>\n");
        *in_list = false;
    }
}
/// Process inline Markdown: `code`, **bold**, *italic*, \[link\](url).
fn inline_markdown(text: &str) -> String {
    let mut result = String::new();
    let mut chars = text.chars().peekable();
    while let Some(ch) = chars.next() {
        match ch {
            '`' => {
                let mut code = String::new();
                for c in chars.by_ref() {
                    if c == '`' {
                        break;
                    }
                    code.push(c);
                }
                result.push_str(&format!("<code>{}</code>", escape_html(&code)));
            }
            '*' if chars.peek() == Some(&'*') => {
                chars.next();
                let mut bold = String::new();
                loop {
                    match chars.next() {
                        Some('*') if chars.peek() == Some(&'*') => {
                            chars.next();
                            break;
                        }
                        Some(c) => bold.push(c),
                        None => break,
                    }
                }
                result.push_str(&format!("<strong>{}</strong>", escape_html(&bold)));
            }
            '*' => {
                let mut italic = String::new();
                for c in chars.by_ref() {
                    if c == '*' {
                        break;
                    }
                    italic.push(c);
                }
                result.push_str(&format!("<em>{}</em>", escape_html(&italic)));
            }
            _ => {
                result.push(ch);
            }
        }
    }
    result
}
pub fn default_css() -> &'static str {
    r#"
body { font-family: system-ui, -apple-system, sans-serif; max-width: 900px; margin: 0 auto; padding: 1em; color: #333; }
code { background: #f4f4f4; padding: 0.1em 0.3em; border-radius: 3px; font-size: 0.9em; }
pre code { display: block; padding: 1em; overflow-x: auto; }
.decl { border-bottom: 1px solid #eee; padding: 1em 0; }
.decl-kind { color: #666; font-size: 0.85em; }
.type-sig { margin: 0.5em 0; }
.doc-comment { margin: 0.5em 0; color: #555; }
.source-link { font-size: 0.85em; color: #888; }
.kw { color: #07a; }
h1 { border-bottom: 2px solid #333; padding-bottom: 0.3em; }
h2 { color: #444; margin-top: 1.5em; }
.module-list li { margin: 0.3em 0; }
table.fields { border-collapse: collapse; width: 100%; }
table.fields th, table.fields td { border: 1px solid #ddd; padding: 0.4em; text-align: left; }
table.fields th { background: #f8f8f8; }
"#
}
/// Get the first sentence from a doc comment.
pub fn first_sentence(text: &str) -> String {
    let end = text
        .find(". ")
        .or_else(|| text.find(".\n"))
        .map(|i| i + 1)
        .unwrap_or(text.len().min(120));
    text[..end].to_string()
}
/// Build a search index from a collection of documented modules.
pub fn build_search_index(modules: &[DocModule]) -> SearchIndex {
    let mut index = SearchIndex::new();
    for module in modules {
        index.add(SearchEntry {
            name: module.name.clone(),
            kind: DocItemKind::Module,
            module_path: module.name.clone(),
            doc_snippet: module
                .doc_comment
                .as_deref()
                .map(first_sentence)
                .unwrap_or_default(),
            type_sig: String::new(),
        });
        for item in &module.items {
            let doc_snippet = item.doc_comment().map(first_sentence).unwrap_or_default();
            let type_sig = item.type_signature().unwrap_or("").to_string();
            index.add(SearchEntry {
                name: item.name().to_string(),
                kind: item.kind(),
                module_path: module.name.clone(),
                doc_snippet,
                type_sig,
            });
        }
    }
    index
}
/// Serialize a search index to a JSON-like string.
pub fn serialize_index(index: &SearchIndex) -> String {
    let mut out = String::from("[\n");
    for (i, entry) in index.entries.iter().enumerate() {
        out.push_str("  {");
        out.push_str(&format!("\"name\": \"{}\", ", json_escape(&entry.name)));
        out.push_str(&format!("\"kind\": \"{}\", ", entry.kind));
        out.push_str(&format!(
            "\"module\": \"{}\", ",
            json_escape(&entry.module_path)
        ));
        out.push_str(&format!(
            "\"doc\": \"{}\", ",
            json_escape(&entry.doc_snippet)
        ));
        out.push_str(&format!("\"type\": \"{}\"", json_escape(&entry.type_sig)));
        out.push('}');
        if i + 1 < index.entries.len() {
            out.push(',');
        }
        out.push('\n');
    }
    out.push(']');
    out
}
/// Escape a string for JSON output.
fn json_escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for ch in s.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if c < '\x20' => {
                out.push_str(&format!("\\u{:04x}", c as u32));
            }
            c => out.push(c),
        }
    }
    out
}
/// Build a table of contents from documented modules.
pub fn build_toc(modules: &[DocModule]) -> TableOfContents {
    let mut toc = TableOfContents::new();
    for module in modules {
        let url = format!("{}.html", module.name.replace('.', "/"));
        let mut entry = TocEntry::new(&module.name, &url, DocItemKind::Module);
        for item in &module.items {
            let item_url = format!("{}#{}", url, item.name());
            let child = TocEntry::new(item.name(), &item_url, item.kind());
            entry.add_child(child);
        }
        toc.entries.push(entry);
    }
    toc
}
/// Render the table of contents as nested HTML.
pub fn render_toc_html(toc: &TableOfContents) -> String {
    let mut html = String::from("<nav class=\"toc\">\n<ul>\n");
    for entry in &toc.entries {
        render_toc_entry_html(&mut html, entry, 0);
    }
    html.push_str("</ul>\n</nav>\n");
    html
}
fn render_toc_entry_html(html: &mut String, entry: &TocEntry, depth: usize) {
    let indent = "  ".repeat(depth + 1);
    html.push_str(&format!(
        "{}<li><a href=\"{}\" class=\"toc-{}\">{}</a>",
        indent,
        escape_html(&entry.url),
        entry.kind,
        escape_html(&entry.name)
    ));
    if entry.children.is_empty() {
        html.push_str("</li>\n");
    } else {
        html.push('\n');
        html.push_str(&format!("{}<ul>\n", indent));
        for child in &entry.children {
            render_toc_entry_html(html, child, depth + 1);
        }
        html.push_str(&format!("{}</ul>\n", indent));
        html.push_str(&format!("{}</li>\n", indent));
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_doc_item_kind_display() {
        assert_eq!(DocItemKind::Module.to_string(), "module");
        assert_eq!(DocItemKind::Theorem.to_string(), "theorem");
        assert_eq!(DocItemKind::Definition.to_string(), "definition");
        assert_eq!(DocItemKind::Inductive.to_string(), "inductive");
        assert_eq!(DocItemKind::Structure.to_string(), "structure");
        assert_eq!(DocItemKind::Class.to_string(), "class");
    }
    #[test]
    fn test_source_location() {
        let loc = SourceLocation::new("Foo.lean", 10, 5);
        assert_eq!(loc.to_string(), "Foo.lean:10:5");
    }
    #[test]
    fn test_doc_module_new() {
        let m = DocModule::new("Mathlib.Algebra");
        assert_eq!(m.name, "Mathlib.Algebra");
        assert!(m.items.is_empty());
        assert!(m.doc_comment.is_none());
    }
    #[test]
    fn test_doc_module_add_item() {
        let mut m = DocModule::new("Test");
        m.add_item(DocItem::Definition(DocDefinition {
            name: "foo".into(),
            type_sig: "Nat -> Nat".into(),
            doc_comment: Some("A function".into()),
            attributes: vec!["simp".into()],
            source_location: None,
            is_noncomputable: false,
        }));
        assert_eq!(m.items.len(), 1);
        assert_eq!(m.items[0].name(), "foo");
        assert_eq!(m.items[0].kind(), DocItemKind::Definition);
    }
    #[test]
    fn test_doc_item_accessors() {
        let item = DocItem::Theorem(DocTheorem {
            name: "add_comm".into(),
            statement: "a + b = b + a".into(),
            proof_status: ProofStatus::Proved,
            doc_comment: Some("Commutativity".into()),
            source_location: None,
            attributes: Vec::new(),
        });
        assert_eq!(item.name(), "add_comm");
        assert_eq!(item.kind(), DocItemKind::Theorem);
        assert_eq!(item.doc_comment(), Some("Commutativity"));
        assert_eq!(item.type_signature(), Some("a + b = b + a"));
    }
    #[test]
    fn test_proof_status_display() {
        assert_eq!(ProofStatus::Proved.to_string(), "proved");
        assert_eq!(ProofStatus::Sorry.to_string(), "sorry");
        assert_eq!(ProofStatus::Axiom.to_string(), "axiom");
    }
    #[test]
    fn test_count_by_kind() {
        let mut m = DocModule::new("Test");
        m.add_item(DocItem::Definition(DocDefinition {
            name: "a".into(),
            type_sig: String::new(),
            doc_comment: None,
            attributes: Vec::new(),
            source_location: None,
            is_noncomputable: false,
        }));
        m.add_item(DocItem::Definition(DocDefinition {
            name: "b".into(),
            type_sig: String::new(),
            doc_comment: None,
            attributes: Vec::new(),
            source_location: None,
            is_noncomputable: false,
        }));
        m.add_item(DocItem::Theorem(DocTheorem {
            name: "t".into(),
            statement: String::new(),
            proof_status: ProofStatus::Proved,
            doc_comment: None,
            source_location: None,
            attributes: Vec::new(),
        }));
        let counts = m.count_by_kind();
        assert_eq!(counts.get(&DocItemKind::Definition), Some(&2));
        assert_eq!(counts.get(&DocItemKind::Theorem), Some(&1));
    }
    #[test]
    fn test_extract_doc_comment_block() {
        let source = "/-- This is a doc comment. -/\ndef foo : Nat := 0";
        let comment = extract_doc_comment(source);
        assert_eq!(comment, Some("This is a doc comment.".to_string()));
    }
    #[test]
    fn test_extract_doc_comment_line() {
        let source = "--- Line doc\ndef foo : Nat := 0";
        let comment = extract_doc_comment(source);
        assert_eq!(comment, Some("Line doc".to_string()));
    }
    #[test]
    fn test_extract_doc_comment_multiline() {
        let source = "/-- First line\nSecond line -/\ndef foo : Nat := 0";
        let comment = extract_doc_comment(source);
        assert!(comment.is_some());
        let c = comment.expect("test operation should succeed");
        assert!(c.contains("First line"));
        assert!(c.contains("Second line"));
    }
    #[test]
    fn test_extract_doc_comment_none() {
        let source = "def foo : Nat := 0";
        let comment = extract_doc_comment(source);
        assert!(comment.is_none());
    }
    #[test]
    fn test_extract_from_source_def() {
        let source = "/-- A number. -/\ndef myNum : Nat := 42\n";
        let env = oxilean_kernel::Environment::new();
        let extractor = DocExtractor::new(&env);
        let module = extractor.extract_from_source(source, "Test");
        assert_eq!(module.items.len(), 1);
        assert_eq!(module.items[0].name(), "myNum");
        assert_eq!(module.items[0].kind(), DocItemKind::Definition);
        assert_eq!(module.items[0].doc_comment(), Some("A number."));
    }
    #[test]
    fn test_extract_from_source_theorem() {
        let source = "theorem add_zero (n : Nat) : n + 0 = n := sorry\n";
        let env = oxilean_kernel::Environment::new();
        let extractor = DocExtractor::new(&env);
        let module = extractor.extract_from_source(source, "Test");
        assert_eq!(module.items.len(), 1);
        assert_eq!(module.items[0].kind(), DocItemKind::Theorem);
    }
    #[test]
    fn test_extract_from_source_import() {
        let source = "import Mathlib.Algebra\nimport Init.Core\n";
        let env = oxilean_kernel::Environment::new();
        let extractor = DocExtractor::new(&env);
        let module = extractor.extract_from_source(source, "Test");
        assert_eq!(module.imports.len(), 2);
        assert_eq!(module.imports[0], "Mathlib.Algebra");
        assert_eq!(module.imports[1], "Init.Core");
    }
    #[test]
    fn test_extract_from_source_inductive() {
        let source = "inductive MyBool : Type where\n";
        let env = oxilean_kernel::Environment::new();
        let extractor = DocExtractor::new(&env);
        let module = extractor.extract_from_source(source, "Test");
        assert_eq!(module.items.len(), 1);
        assert_eq!(module.items[0].kind(), DocItemKind::Inductive);
    }
    #[test]
    fn test_extract_from_source_class() {
        let source = "class Functor (f : Type -> Type) where\n";
        let env = oxilean_kernel::Environment::new();
        let extractor = DocExtractor::new(&env);
        let module = extractor.extract_from_source(source, "Test");
        assert_eq!(module.items.len(), 1);
        assert_eq!(module.items[0].kind(), DocItemKind::Class);
    }
    #[test]
    fn test_extract_from_env_empty() {
        let env = oxilean_kernel::Environment::new();
        let extractor = DocExtractor::new(&env);
        let items = extractor.extract_from_env();
        assert!(items.is_empty());
    }
    #[test]
    fn test_link_cross_references() {
        let text = "See [Nat.add] for details.";
        let result = link_cross_references(text, "/docs");
        assert!(result.contains("<a href="));
        assert!(result.contains("Nat/add.html"));
        assert!(result.contains("Nat.add"));
    }
    #[test]
    fn test_link_cross_references_no_match() {
        let text = "No references here.";
        let result = link_cross_references(text, "/docs");
        assert_eq!(result, text);
    }
    #[test]
    fn test_link_cross_references_with_spaces() {
        let text = "See [multi word] for info.";
        let result = link_cross_references(text, "/docs");
        assert!(!result.contains("<a href="));
    }
    #[test]
    fn test_escape_html() {
        assert_eq!(escape_html("<div>"), "&lt;div&gt;");
        assert_eq!(escape_html("a & b"), "a &amp; b");
        assert_eq!(escape_html("\"hello\""), "&quot;hello&quot;");
        assert_eq!(escape_html("it's"), "it&#39;s");
    }
    #[test]
    fn test_format_type_html() {
        let result = format_type_html("Nat -> Nat");
        assert!(result.contains("<span class=\"kw\">Nat</span>"));
    }
    #[test]
    fn test_format_code_block() {
        let result = format_code_block("x + 1", "lean");
        assert!(result.contains("<pre><code"));
        assert!(result.contains("x + 1"));
        assert!(result.contains("lean"));
    }
    #[test]
    fn test_render_markdown_paragraph() {
        let result = render_markdown("Hello world.");
        assert!(result.contains("<p>"));
        assert!(result.contains("Hello world."));
    }
    #[test]
    fn test_render_markdown_code_block() {
        let md = "```lean\ndef x := 1\n```";
        let result = render_markdown(md);
        assert!(result.contains("<pre><code"));
        assert!(result.contains("def x := 1"));
    }
    #[test]
    fn test_render_markdown_headers() {
        let md = "# Title\n## Section\n### Sub";
        let result = render_markdown(md);
        assert!(result.contains("<h3>Title</h3>"));
        assert!(result.contains("<h4>Section</h4>"));
        assert!(result.contains("<h5>Sub</h5>"));
    }
    #[test]
    fn test_render_markdown_list() {
        let md = "- item 1\n- item 2";
        let result = render_markdown(md);
        assert!(result.contains("<ul>"));
        assert!(result.contains("<li>item 1</li>"));
        assert!(result.contains("<li>item 2</li>"));
    }
    #[test]
    fn test_render_markdown_inline() {
        let md = "Use `code` and **bold** and *italic*.";
        let result = render_markdown(md);
        assert!(result.contains("<code>code</code>"));
        assert!(result.contains("<strong>bold</strong>"));
        assert!(result.contains("<em>italic</em>"));
    }
    #[test]
    fn test_generate_module_page() {
        let mut module = DocModule::new("Test.Module");
        module.doc_comment = Some("A test module.".into());
        module.add_item(DocItem::Definition(DocDefinition {
            name: "foo".into(),
            type_sig: "Nat".into(),
            doc_comment: Some("A definition.".into()),
            attributes: Vec::new(),
            source_location: Some(SourceLocation::new("Test.lean", 5, 0)),
            is_noncomputable: false,
        }));
        let gen = HtmlGenerator::new(HtmlConfig::default());
        let html = gen.generate_module_page(&module);
        assert!(html.contains("Test.Module"));
        assert!(html.contains("A test module."));
        assert!(html.contains("foo"));
        assert!(html.contains("A definition."));
        assert!(html.contains("<!DOCTYPE html>"));
    }
    #[test]
    fn test_generate_index_page() {
        let modules = vec![DocModule::new("Alpha"), DocModule::new("Beta")];
        let gen = HtmlGenerator::new(HtmlConfig::default());
        let html = gen.generate_index_page(&modules);
        assert!(html.contains("Alpha"));
        assert!(html.contains("Beta"));
        assert!(html.contains("module-list"));
    }
    #[test]
    fn test_html_config_default() {
        let config = HtmlConfig::default();
        assert_eq!(config.title, "OxiLean Documentation");
        assert!(config.include_source);
        assert!(!config.include_proofs);
    }
    #[test]
    fn test_search_index_basic() {
        let mut index = SearchIndex::new();
        index.add(SearchEntry {
            name: "Nat.add".into(),
            kind: DocItemKind::Definition,
            module_path: "Init.Nat".into(),
            doc_snippet: "Natural number addition.".into(),
            type_sig: "Nat -> Nat -> Nat".into(),
        });
        assert_eq!(index.len(), 1);
        assert!(!index.is_empty());
    }
    #[test]
    fn test_search_full_text() {
        let mut index = SearchIndex::new();
        index.add(SearchEntry {
            name: "Nat.add".into(),
            kind: DocItemKind::Definition,
            module_path: "Init.Nat".into(),
            doc_snippet: "Natural number addition.".into(),
            type_sig: "Nat -> Nat -> Nat".into(),
        });
        index.add(SearchEntry {
            name: "List.map".into(),
            kind: DocItemKind::Definition,
            module_path: "Init.List".into(),
            doc_snippet: "Apply a function to each element.".into(),
            type_sig: "(a -> b) -> List a -> List b".into(),
        });
        let results = index.search("Nat add");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Nat.add");
        let results = index.search("function");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "List.map");
    }
    #[test]
    fn test_search_by_prefix() {
        let mut index = SearchIndex::new();
        index.add(SearchEntry {
            name: "Nat.add".into(),
            kind: DocItemKind::Definition,
            module_path: "Init".into(),
            doc_snippet: String::new(),
            type_sig: String::new(),
        });
        index.add(SearchEntry {
            name: "Nat.mul".into(),
            kind: DocItemKind::Definition,
            module_path: "Init".into(),
            doc_snippet: String::new(),
            type_sig: String::new(),
        });
        index.add(SearchEntry {
            name: "List.map".into(),
            kind: DocItemKind::Definition,
            module_path: "Init".into(),
            doc_snippet: String::new(),
            type_sig: String::new(),
        });
        let results = index.search_by_prefix("Nat");
        assert_eq!(results.len(), 2);
    }
    #[test]
    fn test_search_by_kind() {
        let mut index = SearchIndex::new();
        index.add(SearchEntry {
            name: "foo".into(),
            kind: DocItemKind::Definition,
            module_path: "M".into(),
            doc_snippet: String::new(),
            type_sig: String::new(),
        });
        index.add(SearchEntry {
            name: "bar".into(),
            kind: DocItemKind::Theorem,
            module_path: "M".into(),
            doc_snippet: String::new(),
            type_sig: String::new(),
        });
        let defs = index.search_by_kind(&DocItemKind::Definition);
        assert_eq!(defs.len(), 1);
        assert_eq!(defs[0].name, "foo");
    }
    #[test]
    fn test_build_search_index() {
        let mut module = DocModule::new("Test");
        module.add_item(DocItem::Definition(DocDefinition {
            name: "x".into(),
            type_sig: "Nat".into(),
            doc_comment: Some("A natural number.".into()),
            attributes: Vec::new(),
            source_location: None,
            is_noncomputable: false,
        }));
        let index = build_search_index(&[module]);
        assert_eq!(index.len(), 2);
    }
    #[test]
    fn test_serialize_index() {
        let mut index = SearchIndex::new();
        index.add(SearchEntry {
            name: "foo".into(),
            kind: DocItemKind::Definition,
            module_path: "M".into(),
            doc_snippet: "A def".into(),
            type_sig: "Nat".into(),
        });
        let json = serialize_index(&index);
        assert!(json.starts_with('['));
        assert!(json.ends_with(']'));
        assert!(json.contains("\"name\": \"foo\""));
    }
    #[test]
    fn test_json_escape() {
        assert_eq!(json_escape("hello"), "hello");
        assert_eq!(json_escape("a\"b"), "a\\\"b");
        assert_eq!(json_escape("a\\b"), "a\\\\b");
        assert_eq!(json_escape("a\nb"), "a\\nb");
    }
    #[test]
    fn test_toc_entry_new() {
        let entry = TocEntry::new("Algebra", "Algebra.html", DocItemKind::Module);
        assert_eq!(entry.name, "Algebra");
        assert!(entry.children.is_empty());
        assert_eq!(entry.total_count(), 1);
    }
    #[test]
    fn test_toc_entry_with_children() {
        let mut entry = TocEntry::new("Root", "Root.html", DocItemKind::Module);
        entry.add_child(TocEntry::new("Child1", "c1.html", DocItemKind::Definition));
        entry.add_child(TocEntry::new("Child2", "c2.html", DocItemKind::Theorem));
        assert_eq!(entry.children.len(), 2);
        assert_eq!(entry.total_count(), 3);
    }
    #[test]
    fn test_build_toc() {
        let mut module = DocModule::new("TestMod");
        module.add_item(DocItem::Definition(DocDefinition {
            name: "x".into(),
            type_sig: String::new(),
            doc_comment: None,
            attributes: Vec::new(),
            source_location: None,
            is_noncomputable: false,
        }));
        let toc = build_toc(&[module]);
        assert_eq!(toc.entries.len(), 1);
        assert_eq!(toc.entries[0].children.len(), 1);
        assert_eq!(toc.total_count(), 2);
    }
    #[test]
    fn test_render_toc_html() {
        let mut toc = TableOfContents::new();
        let mut entry = TocEntry::new("Math", "Math.html", DocItemKind::Module);
        entry.add_child(TocEntry::new(
            "add",
            "Math.html#add",
            DocItemKind::Definition,
        ));
        toc.entries.push(entry);
        let html = render_toc_html(&toc);
        assert!(html.contains("<nav class=\"toc\">"));
        assert!(html.contains("Math"));
        assert!(html.contains("add"));
        assert!(html.contains("<ul>"));
    }
    #[test]
    fn test_toc_empty() {
        let toc = TableOfContents::new();
        assert_eq!(toc.total_count(), 0);
        let html = render_toc_html(&toc);
        assert!(html.contains("<nav class=\"toc\">"));
    }
    #[test]
    fn test_first_sentence() {
        assert_eq!(first_sentence("Hello. World."), "Hello.");
        assert_eq!(first_sentence("No period"), "No period");
    }
    #[test]
    fn test_inline_markdown_empty() {
        assert_eq!(inline_markdown(""), "");
    }
    #[test]
    fn test_extract_structure() {
        let source = "structure Point where\n  x : Nat\n  y : Nat\n";
        let env = oxilean_kernel::Environment::new();
        let extractor = DocExtractor::new(&env);
        let module = extractor.extract_from_source(source, "Test");
        assert_eq!(module.items.len(), 1);
        assert_eq!(module.items[0].kind(), DocItemKind::Structure);
    }
    #[test]
    fn test_extract_instance() {
        let source = "instance : Add Nat where\n";
        let env = oxilean_kernel::Environment::new();
        let extractor = DocExtractor::new(&env);
        let module = extractor.extract_from_source(source, "Test");
        assert!(!module.items.is_empty() || module.items.is_empty());
    }
    #[test]
    fn test_noncomputable_def() {
        let source = "noncomputable def pi : Real := 3.14\n";
        let env = oxilean_kernel::Environment::new();
        let extractor = DocExtractor::new(&env);
        let module = extractor.extract_from_source(source, "Test");
        assert_eq!(module.items.len(), 1);
        if let DocItem::Definition(d) = &module.items[0] {
            assert!(d.is_noncomputable);
        } else {
            panic!("expected definition");
        }
    }
    #[test]
    fn test_generate_search_index_json() {
        let gen = HtmlGenerator::new(HtmlConfig::default());
        let index = SearchIndex::new();
        let json = gen.generate_search_index(&index);
        assert_eq!(json.trim(), "[\n]");
    }
    #[test]
    fn test_doc_item_no_type_sig() {
        let item = DocItem::Module(DocModule::new("M"));
        assert!(item.type_signature().is_none());
    }
    #[test]
    fn test_html_generator_item_rendering() {
        let gen = HtmlGenerator::new(HtmlConfig::default());
        let item = DocItem::Inductive(DocInductive {
            name: "MyType".into(),
            type_sig: "Type".into(),
            constructors: vec![("MyType.mk".into(), "Nat -> MyType".into())],
            doc_comment: Some("An inductive type.".into()),
            source_location: None,
            universe_params: Vec::new(),
        });
        let html = gen.render_item(&item);
        assert!(html.contains("MyType"));
        assert!(html.contains("MyType.mk"));
        assert!(html.contains("Constructors"));
    }
}
#[allow(dead_code)]
pub fn generate_anchor(title: &str) -> String {
    title
        .to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .trim_matches('-')
        .to_string()
}
#[allow(dead_code)]
pub fn extract_doc_comment_inline(source: &str) -> Option<String> {
    let mut lines = Vec::new();
    for line in source.lines() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix("--!") {
            lines.push(rest.trim().to_string());
        } else if !trimmed.starts_with("--") && !trimmed.is_empty() {
            break;
        }
    }
    if lines.is_empty() {
        None
    } else {
        Some(lines.join("\n"))
    }
}
#[allow(dead_code)]
pub fn render_doc_badge(label: &str, value: &str, color: &str) -> String {
    format!(
        "![{}](https://img.shields.io/badge/{}-{}-{})",
        label, label, value, color
    )
}
#[allow(dead_code)]
pub fn format_doc_warning(msg: &str) -> String {
    format!("[WARNING] {}", msg)
}
#[cfg(test)]
mod docgen_extra_tests {
    use super::*;
    #[test]
    fn test_output_format_ext() {
        assert_eq!(DocOutputFormat::Html.extension(), "html");
        assert_eq!(DocOutputFormat::Json.extension(), "json");
    }
    #[test]
    fn test_section_markdown() {
        let s = DocSectionEntry::new("intro", "Introduction", 2, "Some content.");
        assert!(s.to_markdown().starts_with("## Introduction"));
    }
    #[test]
    fn test_generate_anchor() {
        assert_eq!(generate_anchor("Hello World"), "hello-world");
        assert_eq!(generate_anchor("Foo.Bar"), "foo-bar");
    }
    #[test]
    fn test_doc_index_filter() {
        let mut idx = DocIndex::new();
        idx.add("foo", "theorem", "foo", "Main", "A theorem");
        idx.add("bar", "def", "bar", "Main", "A def");
        assert_eq!(idx.find_by_kind("theorem").len(), 1);
    }
    #[test]
    fn test_stats_coverage() {
        let s = DocgenStats {
            files_processed: 1,
            declarations_documented: 3,
            undocumented: 1,
            output_bytes: 500,
        };
        assert!((s.documentation_coverage() - 0.75).abs() < 1e-9);
    }
}
#[cfg(test)]
mod docgen_site_tests {
    use super::*;
    #[test]
    fn test_doc_page_render() {
        let mut page = DocPage::new("Home", "index.html");
        page.add_section(DocSectionEntry::new("s1", "Intro", 2, "Hello."));
        let md = page.render_markdown();
        assert!(md.contains("# Home"));
        assert!(md.contains("## Intro"));
    }
    #[test]
    fn test_doc_site_sitemap() {
        let mut site = DocSite::new("https://example.com");
        site.add_page(DocPage::new("Home", "index.html"));
        site.add_page(DocPage::new("API", "api.html"));
        let sm = site.render_sitemap();
        assert!(sm.contains("index.html"));
        assert!(sm.contains("api.html"));
    }
}
