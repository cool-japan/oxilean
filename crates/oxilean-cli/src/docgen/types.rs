//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use oxilean_kernel::Name;
use std::collections::HashMap;

/// Documentation for a definition.
#[derive(Clone, Debug)]
pub struct DocDefinition {
    /// Fully-qualified name.
    pub name: String,
    /// Pretty-printed type signature.
    pub type_sig: String,
    /// Doc comment.
    pub doc_comment: Option<String>,
    /// Attributes (e.g., `@[simp]`, `@[inline]`).
    pub attributes: Vec<String>,
    /// Source location.
    pub source_location: Option<SourceLocation>,
    /// Whether this is noncomputable.
    pub is_noncomputable: bool,
}
/// A single entry in the table of contents.
#[derive(Clone, Debug)]
pub struct TocEntry {
    /// Display name.
    pub name: String,
    /// URL (relative).
    pub url: String,
    /// Item kind.
    pub kind: DocItemKind,
    /// Children (sub-entries).
    pub children: Vec<TocEntry>,
}
impl TocEntry {
    /// Create a new TOC entry.
    pub fn new(name: &str, url: &str, kind: DocItemKind) -> Self {
        Self {
            name: name.to_string(),
            url: url.to_string(),
            kind,
            children: Vec::new(),
        }
    }
    /// Add a child entry.
    pub fn add_child(&mut self, child: TocEntry) {
        self.children.push(child);
    }
    /// Total number of entries (including self and all descendants).
    pub fn total_count(&self) -> usize {
        1 + self.children.iter().map(|c| c.total_count()).sum::<usize>()
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DocSectionEntry {
    pub id: String,
    pub title: String,
    pub level: u8,
    pub content: String,
}
#[allow(dead_code)]
impl DocSectionEntry {
    pub fn new(id: &str, title: &str, level: u8, content: &str) -> Self {
        Self {
            id: id.to_string(),
            title: title.to_string(),
            level,
            content: content.to_string(),
        }
    }
    pub fn to_markdown(&self) -> String {
        let hashes = "#".repeat(self.level as usize);
        format!(
            "{} {}

{}",
            hashes, self.title, self.content
        )
    }
    pub fn to_html(&self) -> String {
        format!(
            "<h{} id=\"{}\">{}</h{}>\n<div class=\"section-content\">{}\n</div>",
            self.level, self.id, self.title, self.level, self.content
        )
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DocIndex {
    pub entries: Vec<DocIndexEntry>,
}
#[allow(dead_code)]
impl DocIndex {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }
    pub fn add(&mut self, name: &str, kind: &str, anchor: &str, module: &str, summary: &str) {
        self.entries.push(DocIndexEntry {
            name: name.to_string(),
            kind: kind.to_string(),
            anchor: anchor.to_string(),
            module: module.to_string(),
            summary: summary.to_string(),
        });
    }
    pub fn find_by_name(&self, name: &str) -> Vec<&DocIndexEntry> {
        self.entries.iter().filter(|e| e.name == name).collect()
    }
    pub fn find_by_kind(&self, kind: &str) -> Vec<&DocIndexEntry> {
        self.entries.iter().filter(|e| e.kind == kind).collect()
    }
    pub fn to_markdown_toc(&self) -> String {
        self.entries
            .iter()
            .map(|e| format!("- [{}](#{}) ({})", e.name, e.anchor, e.kind))
            .collect::<Vec<_>>()
            .join(
                "
",
            )
    }
}
/// Table of contents for the documentation.
#[derive(Clone, Debug, Default)]
pub struct TableOfContents {
    /// Top-level entries.
    pub entries: Vec<TocEntry>,
}
impl TableOfContents {
    /// Create an empty TOC.
    pub fn new() -> Self {
        Self::default()
    }
    /// Total number of entries.
    pub fn total_count(&self) -> usize {
        self.entries.iter().map(|e| e.total_count()).sum()
    }
}
/// Documentation for an inductive type.
#[derive(Clone, Debug)]
pub struct DocInductive {
    /// Fully-qualified name.
    pub name: String,
    /// Pretty-printed type signature.
    pub type_sig: String,
    /// Constructors: (name, type_sig).
    pub constructors: Vec<(String, String)>,
    /// Doc comment.
    pub doc_comment: Option<String>,
    /// Source location.
    pub source_location: Option<SourceLocation>,
    /// Universe parameters.
    pub universe_params: Vec<String>,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DocgenStats {
    pub files_processed: usize,
    pub declarations_documented: usize,
    pub undocumented: usize,
    pub output_bytes: usize,
}
#[allow(dead_code)]
impl DocgenStats {
    pub fn new() -> Self {
        Self {
            files_processed: 0,
            declarations_documented: 0,
            undocumented: 0,
            output_bytes: 0,
        }
    }
    pub fn documentation_coverage(&self) -> f64 {
        let total = self.declarations_documented + self.undocumented;
        if total == 0 {
            1.0
        } else {
            self.declarations_documented as f64 / total as f64
        }
    }
    pub fn summary(&self) -> String {
        format!(
            "Processed {} files, {} documented / {} total ({:.1}%)",
            self.files_processed,
            self.declarations_documented,
            self.declarations_documented + self.undocumented,
            self.documentation_coverage() * 100.0,
        )
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DocSite {
    pub pages: Vec<DocPage>,
    pub base_url: String,
}
#[allow(dead_code)]
impl DocSite {
    pub fn new(base_url: &str) -> Self {
        Self {
            pages: Vec::new(),
            base_url: base_url.to_string(),
        }
    }
    pub fn add_page(&mut self, page: DocPage) {
        self.pages.push(page);
    }
    pub fn page_count(&self) -> usize {
        self.pages.len()
    }
    pub fn find_page(&self, path: &str) -> Option<&DocPage> {
        self.pages.iter().find(|p| p.path == path)
    }
    pub fn render_sitemap(&self) -> String {
        self.pages
            .iter()
            .map(|p| format!("{}/{}", self.base_url.trim_end_matches('/'), p.path))
            .collect::<Vec<_>>()
            .join(
                "
",
            )
    }
}
/// The kind of a documented item.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum DocItemKind {
    /// Module-level documentation.
    Module,
    /// `def` / `noncomputable def`.
    Definition,
    /// `theorem` / `lemma`.
    Theorem,
    /// `inductive` type.
    Inductive,
    /// `structure`.
    Structure,
    /// `class`.
    Class,
    /// `instance`.
    Instance,
    /// `tactic`.
    Tactic,
    /// `axiom`.
    Axiom,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DocIndexEntry {
    pub name: String,
    pub kind: String,
    pub anchor: String,
    pub module: String,
    pub summary: String,
}
/// Source location of a documented item.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SourceLocation {
    /// File path.
    pub file: String,
    /// Start line (1-based).
    pub line: usize,
    /// Start column (0-based).
    pub column: usize,
}
impl SourceLocation {
    /// Create a new source location.
    pub fn new(file: &str, line: usize, column: usize) -> Self {
        Self {
            file: file.to_string(),
            line,
            column,
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DocPage {
    pub title: String,
    pub path: String,
    pub sections: Vec<DocSectionEntry>,
    pub index: DocIndex,
}
#[allow(dead_code)]
impl DocPage {
    pub fn new(title: &str, path: &str) -> Self {
        Self {
            title: title.to_string(),
            path: path.to_string(),
            sections: Vec::new(),
            index: DocIndex::new(),
        }
    }
    pub fn add_section(&mut self, section: DocSectionEntry) {
        self.sections.push(section);
    }
    pub fn render_markdown(&self) -> String {
        let mut out = format!(
            "# {}

",
            self.title
        );
        for section in &self.sections {
            out.push_str(&section.to_markdown());
            out.push_str(
                "

",
            );
        }
        out
    }
    pub fn render_html(&self) -> String {
        let mut out = format!(
            "<html><head><title>{}</title></head><body>
",
            self.title
        );
        out.push_str(&format!(
            "<h1>{}</h1>
",
            self.title
        ));
        for section in &self.sections {
            out.push_str(&section.to_html());
            out.push('\n');
        }
        out.push_str(
            "</body></html>
",
        );
        out
    }
    pub fn total_content_length(&self) -> usize {
        self.sections.iter().map(|s| s.content.len()).sum()
    }
}
/// Extracts documentation from source code and kernel environments.
pub struct DocExtractor<'a> {
    /// Reference to the kernel environment for type information.
    env: &'a oxilean_kernel::Environment,
}
impl<'a> DocExtractor<'a> {
    /// Create a new extractor with an environment reference.
    pub fn new(env: &'a oxilean_kernel::Environment) -> Self {
        Self { env }
    }
    /// Extract documentation from source code text.
    pub fn extract_from_source(&self, source: &str, file_name: &str) -> DocModule {
        let mut module = DocModule::new(file_name);
        let lines: Vec<&str> = source.lines().collect();
        let mut idx = 0;
        while idx < lines.len() {
            let line = lines[idx].trim();
            if line.is_empty() {
                idx += 1;
                continue;
            }
            let (doc_comment, decl_idx) = extract_doc_comment_at(&lines, idx);
            idx = decl_idx;
            if idx >= lines.len() {
                if module.doc_comment.is_none() {
                    module.doc_comment = doc_comment;
                }
                break;
            }
            let line = lines[idx].trim();
            if let Some(item) = parse_definition_line(line, &doc_comment, file_name, idx + 1) {
                module.add_item(item);
            } else if let Some(item) = parse_theorem_line(line, &doc_comment, file_name, idx + 1) {
                module.add_item(item);
            } else if let Some(item) = parse_inductive_line(line, &doc_comment, file_name, idx + 1)
            {
                module.add_item(item);
            } else if let Some(item) = parse_structure_line(line, &doc_comment, file_name, idx + 1)
            {
                module.add_item(item);
            } else if let Some(item) = parse_class_line(line, &doc_comment, file_name, idx + 1) {
                module.add_item(item);
            } else if let Some(item) = parse_instance_line(line, &doc_comment, file_name, idx + 1) {
                module.add_item(item);
            } else if line.starts_with("import ") {
                let import_name = line.trim_start_matches("import ").trim().to_string();
                module.imports.push(import_name);
            } else if doc_comment.is_some() && module.doc_comment.is_none() {
                module.doc_comment = doc_comment;
            }
            idx += 1;
        }
        module
    }
    /// Extract documentation from the kernel environment.
    pub fn extract_from_env(&self) -> Vec<DocItem> {
        let mut items = Vec::new();
        for (name, ci) in self.env.constant_infos() {
            let name_str = name.to_string();
            let type_str = extract_type_signature_from_expr(ci.ty());
            match ci {
                oxilean_kernel::ConstantInfo::Axiom(_) => {
                    items.push(DocItem::Definition(DocDefinition {
                        name: name_str,
                        type_sig: type_str,
                        doc_comment: None,
                        attributes: vec!["axiom".to_string()],
                        source_location: None,
                        is_noncomputable: false,
                    }));
                }
                oxilean_kernel::ConstantInfo::Definition(_) => {
                    items.push(DocItem::Definition(DocDefinition {
                        name: name_str,
                        type_sig: type_str,
                        doc_comment: None,
                        attributes: Vec::new(),
                        source_location: None,
                        is_noncomputable: false,
                    }));
                }
                oxilean_kernel::ConstantInfo::Theorem(_) => {
                    items.push(DocItem::Theorem(DocTheorem {
                        name: name_str,
                        statement: type_str,
                        proof_status: ProofStatus::Proved,
                        doc_comment: None,
                        source_location: None,
                        attributes: Vec::new(),
                    }));
                }
                oxilean_kernel::ConstantInfo::Inductive(iv) => {
                    let ctors: Vec<(String, String)> = iv
                        .ctors
                        .iter()
                        .map(|c| {
                            let ctor_type = self
                                .env
                                .get_type(c)
                                .map(extract_type_signature_from_expr)
                                .unwrap_or_default();
                            (c.to_string(), ctor_type)
                        })
                        .collect();
                    items.push(DocItem::Inductive(DocInductive {
                        name: name_str,
                        type_sig: type_str,
                        constructors: ctors,
                        doc_comment: None,
                        source_location: None,
                        universe_params: iv
                            .common
                            .level_params
                            .iter()
                            .map(|n| n.to_string())
                            .collect(),
                    }));
                }
                _ => {}
            }
        }
        items
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DocOutputFormat {
    Html,
    Markdown,
    PlainText,
    Json,
}
#[allow(dead_code)]
impl DocOutputFormat {
    pub fn extension(&self) -> &str {
        match self {
            DocOutputFormat::Html => "html",
            DocOutputFormat::Markdown => "md",
            DocOutputFormat::PlainText => "txt",
            DocOutputFormat::Json => "json",
        }
    }
    pub fn mime_type(&self) -> &str {
        match self {
            DocOutputFormat::Html => "text/html",
            DocOutputFormat::Markdown => "text/markdown",
            DocOutputFormat::PlainText => "text/plain",
            DocOutputFormat::Json => "application/json",
        }
    }
}
/// Proof status of a theorem.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ProofStatus {
    /// Proof is provided.
    Proved,
    /// Proof uses sorry.
    Sorry,
    /// No proof (axiom).
    Axiom,
}
/// Documentation for a structure.
#[derive(Clone, Debug)]
pub struct DocStructure {
    /// Fully-qualified name.
    pub name: String,
    /// Pretty-printed type signature.
    pub type_sig: String,
    /// Fields.
    pub fields: Vec<DocField>,
    /// Parent structures/classes this extends.
    pub extends: Vec<String>,
    /// Doc comment.
    pub doc_comment: Option<String>,
    /// Source location.
    pub source_location: Option<SourceLocation>,
}
/// Documentation for an instance.
#[derive(Clone, Debug)]
pub struct DocInstance {
    /// Instance name (may be auto-generated).
    pub name: String,
    /// Pretty-printed type (the class + arguments).
    pub type_sig: String,
    /// Doc comment.
    pub doc_comment: Option<String>,
    /// Source location.
    pub source_location: Option<SourceLocation>,
    /// Priority.
    pub priority: Option<u32>,
}
/// The HTML documentation generator.
pub struct HtmlGenerator {
    /// Generation configuration.
    pub config: HtmlConfig,
}
impl HtmlGenerator {
    /// Create a new HTML generator.
    pub fn new(config: HtmlConfig) -> Self {
        Self { config }
    }
    /// Generate a full HTML page for a module.
    pub fn generate_module_page(&self, module: &DocModule) -> String {
        let mut html = String::new();
        html.push_str(&self.html_header(&format!("{} - {}", module.name, self.config.title)));
        html.push_str("<div class=\"module\">\n");
        html.push_str(&format!("<h1>Module {}</h1>\n", escape_html(&module.name)));
        if let Some(doc) = &module.doc_comment {
            html.push_str("<div class=\"module-doc\">\n");
            html.push_str(&render_markdown(doc));
            html.push_str("</div>\n");
        }
        if !module.imports.is_empty() {
            html.push_str("<h2>Imports</h2>\n<ul class=\"imports\">\n");
            for import in &module.imports {
                html.push_str(&format!("<li><code>{}</code></li>\n", escape_html(import)));
            }
            html.push_str("</ul>\n");
        }
        let groups: &[(DocItemKind, &str)] = &[
            (DocItemKind::Inductive, "Inductive Types"),
            (DocItemKind::Structure, "Structures"),
            (DocItemKind::Class, "Classes"),
            (DocItemKind::Definition, "Definitions"),
            (DocItemKind::Theorem, "Theorems"),
            (DocItemKind::Instance, "Instances"),
            (DocItemKind::Tactic, "Tactics"),
        ];
        for (kind, heading) in groups {
            let items: Vec<&DocItem> = module.items.iter().filter(|i| i.kind() == *kind).collect();
            if items.is_empty() {
                continue;
            }
            html.push_str(&format!("<h2>{}</h2>\n", heading));
            for item in items {
                html.push_str(&self.render_item(item));
            }
        }
        html.push_str("</div>\n");
        html.push_str(&self.html_footer());
        html
    }
    /// Generate an index page listing all modules.
    pub fn generate_index_page(&self, modules: &[DocModule]) -> String {
        let mut html = String::new();
        html.push_str(&self.html_header(&self.config.title));
        html.push_str("<div class=\"index\">\n");
        html.push_str(&format!("<h1>{}</h1>\n", escape_html(&self.config.title)));
        html.push_str("<h2>Modules</h2>\n<ul class=\"module-list\">\n");
        for module in modules {
            let url = format!("{}.html", module.name.replace('.', "/"));
            html.push_str(&format!(
                "<li><a href=\"{}\">{}</a>",
                escape_html(&url),
                escape_html(&module.name)
            ));
            if let Some(doc) = &module.doc_comment {
                let snippet = first_sentence(doc);
                html.push_str(&format!(" &mdash; {}", escape_html(&snippet)));
            }
            html.push_str("</li>\n");
        }
        html.push_str("</ul>\n</div>\n");
        html.push_str(&self.html_footer());
        html
    }
    /// Generate a JSON-like search index.
    pub fn generate_search_index(&self, index: &SearchIndex) -> String {
        serialize_index(index)
    }
    /// Render a single doc item to HTML.
    pub(crate) fn render_item(&self, item: &DocItem) -> String {
        let mut html = String::new();
        let kind = item.kind();
        let name = item.name();
        html.push_str(&format!(
            "<div class=\"decl decl-{}\" id=\"{}\">\n",
            kind,
            escape_html(name)
        ));
        html.push_str(&format!(
            "<h3><span class=\"decl-kind\">{}</span> <code>{}</code></h3>\n",
            kind,
            escape_html(name)
        ));
        if let Some(sig) = item.type_signature() {
            if !sig.is_empty() {
                html.push_str("<div class=\"type-sig\"><code>");
                html.push_str(&format_type_html(sig));
                html.push_str("</code></div>\n");
            }
        }
        if let Some(doc) = item.doc_comment() {
            html.push_str("<div class=\"doc-comment\">\n");
            html.push_str(&render_markdown(doc));
            html.push_str("</div>\n");
        }
        match item {
            DocItem::Inductive(ind) if !ind.constructors.is_empty() => {
                html.push_str("<h4>Constructors</h4>\n<ul class=\"constructors\">\n");
                for (ctor_name, ctor_sig) in &ind.constructors {
                    html.push_str(&format!(
                        "<li><code>{}</code> : <code>{}</code></li>\n",
                        escape_html(ctor_name),
                        format_type_html(ctor_sig)
                    ));
                }
                html.push_str("</ul>\n");
            }
            DocItem::Structure(s) if !s.fields.is_empty() => {
                html.push_str("<h4>Fields</h4>\n<table class=\"fields\">\n");
                html.push_str("<tr><th>Name</th><th>Type</th><th>Description</th></tr>\n");
                for field in &s.fields {
                    html.push_str(&format!(
                        "<tr><td><code>{}</code></td><td><code>{}</code></td><td>{}</td></tr>\n",
                        escape_html(&field.name),
                        format_type_html(&field.type_sig),
                        field
                            .doc_comment
                            .as_deref()
                            .map(escape_html)
                            .unwrap_or_default()
                    ));
                }
                html.push_str("</table>\n");
            }
            DocItem::Class(c) if !c.methods.is_empty() => {
                html.push_str("<h4>Methods</h4>\n<ul class=\"methods\">\n");
                for method in &c.methods {
                    html.push_str(&format!(
                        "<li><code>{}</code> : <code>{}</code>",
                        escape_html(&method.name),
                        format_type_html(&method.type_sig)
                    ));
                    if method.has_default {
                        html.push_str(" <span class=\"default\">(has default)</span>");
                    }
                    html.push_str("</li>\n");
                }
                html.push_str("</ul>\n");
            }
            _ => {}
        }
        if self.config.include_source {
            let loc = match item {
                DocItem::Definition(d) => d.source_location.as_ref(),
                DocItem::Theorem(t) => t.source_location.as_ref(),
                DocItem::Inductive(i) => i.source_location.as_ref(),
                DocItem::Structure(s) => s.source_location.as_ref(),
                DocItem::Class(c) => c.source_location.as_ref(),
                DocItem::Instance(i) => i.source_location.as_ref(),
                DocItem::Tactic(t) => t.source_location.as_ref(),
                _ => None,
            };
            if let Some(loc) = loc {
                html.push_str(&format!(
                    "<div class=\"source-link\"><a href=\"{}#L{}\">source</a></div>\n",
                    escape_html(&loc.file),
                    loc.line
                ));
            }
        }
        html.push_str("</div>\n");
        html
    }
    fn html_header(&self, title: &str) -> String {
        format!(
            "<!DOCTYPE html>\n<html>\n<head>\n<meta charset=\"utf-8\">\n\
             <title>{}</title>\n<style>\n{}\n</style>\n</head>\n<body>\n",
            escape_html(title),
            self.config.css_theme
        )
    }
    fn html_footer(&self) -> String {
        "</body>\n</html>\n".to_string()
    }
}
/// An entry in the search index.
#[derive(Clone, Debug)]
pub struct SearchEntry {
    /// Item name.
    pub name: String,
    /// Item kind.
    pub kind: DocItemKind,
    /// Module path containing this item.
    pub module_path: String,
    /// Short doc snippet (first sentence).
    pub doc_snippet: String,
    /// Type signature (if available).
    pub type_sig: String,
}
/// Documentation for a type class.
#[derive(Clone, Debug)]
pub struct DocClass {
    /// Fully-qualified name.
    pub name: String,
    /// Pretty-printed type signature.
    pub type_sig: String,
    /// Methods.
    pub methods: Vec<DocMethod>,
    /// Known instances.
    pub instances: Vec<String>,
    /// Doc comment.
    pub doc_comment: Option<String>,
    /// Source location.
    pub source_location: Option<SourceLocation>,
    /// Parent classes.
    pub extends: Vec<String>,
}
/// A searchable index over all documented items.
#[derive(Clone, Debug, Default)]
pub struct SearchIndex {
    /// All entries.
    pub entries: Vec<SearchEntry>,
}
impl SearchIndex {
    /// Create an empty index.
    pub fn new() -> Self {
        Self::default()
    }
    /// Add an entry.
    pub fn add(&mut self, entry: SearchEntry) {
        self.entries.push(entry);
    }
    /// Number of entries.
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    /// Whether the index is empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
    /// Full-text search over the index.
    pub fn search(&self, query: &str) -> Vec<&SearchEntry> {
        let query_lower = query.to_lowercase();
        let terms: Vec<&str> = query_lower.split_whitespace().collect();
        self.entries
            .iter()
            .filter(|entry| {
                let haystack = format!(
                    "{} {} {} {}",
                    entry.name, entry.doc_snippet, entry.type_sig, entry.kind
                )
                .to_lowercase();
                terms.iter().all(|term| haystack.contains(term))
            })
            .collect()
    }
    /// Search by name prefix.
    pub fn search_by_prefix(&self, prefix: &str) -> Vec<&SearchEntry> {
        let prefix_lower = prefix.to_lowercase();
        self.entries
            .iter()
            .filter(|e| e.name.to_lowercase().starts_with(&prefix_lower))
            .collect()
    }
    /// Search by kind.
    pub fn search_by_kind(&self, kind: &DocItemKind) -> Vec<&SearchEntry> {
        self.entries.iter().filter(|e| e.kind == *kind).collect()
    }
}
/// A top-level documented item.
#[derive(Clone, Debug)]
pub enum DocItem {
    /// A module.
    Module(DocModule),
    /// A definition.
    Definition(DocDefinition),
    /// A theorem.
    Theorem(DocTheorem),
    /// An inductive type.
    Inductive(DocInductive),
    /// A structure.
    Structure(DocStructure),
    /// A type class.
    Class(DocClass),
    /// An instance.
    Instance(DocInstance),
    /// A tactic.
    Tactic(DocTactic),
}
impl DocItem {
    /// Get the name of this item.
    pub fn name(&self) -> &str {
        match self {
            DocItem::Module(m) => &m.name,
            DocItem::Definition(d) => &d.name,
            DocItem::Theorem(t) => &t.name,
            DocItem::Inductive(i) => &i.name,
            DocItem::Structure(s) => &s.name,
            DocItem::Class(c) => &c.name,
            DocItem::Instance(i) => &i.name,
            DocItem::Tactic(t) => &t.name,
        }
    }
    /// Get the kind of this item.
    pub fn kind(&self) -> DocItemKind {
        match self {
            DocItem::Module(_) => DocItemKind::Module,
            DocItem::Definition(_) => DocItemKind::Definition,
            DocItem::Theorem(_) => DocItemKind::Theorem,
            DocItem::Inductive(_) => DocItemKind::Inductive,
            DocItem::Structure(_) => DocItemKind::Structure,
            DocItem::Class(_) => DocItemKind::Class,
            DocItem::Instance(_) => DocItemKind::Instance,
            DocItem::Tactic(_) => DocItemKind::Tactic,
        }
    }
    /// Get the doc comment, if any.
    pub fn doc_comment(&self) -> Option<&str> {
        match self {
            DocItem::Module(m) => m.doc_comment.as_deref(),
            DocItem::Definition(d) => d.doc_comment.as_deref(),
            DocItem::Theorem(t) => t.doc_comment.as_deref(),
            DocItem::Inductive(i) => i.doc_comment.as_deref(),
            DocItem::Structure(s) => s.doc_comment.as_deref(),
            DocItem::Class(c) => c.doc_comment.as_deref(),
            DocItem::Instance(i) => i.doc_comment.as_deref(),
            DocItem::Tactic(t) => t.doc_comment.as_deref(),
        }
    }
    /// Get the type signature, if available.
    pub fn type_signature(&self) -> Option<&str> {
        match self {
            DocItem::Definition(d) => Some(&d.type_sig),
            DocItem::Theorem(t) => Some(&t.statement),
            DocItem::Inductive(i) => Some(&i.type_sig),
            DocItem::Structure(s) => Some(&s.type_sig),
            DocItem::Class(c) => Some(&c.type_sig),
            _ => None,
        }
    }
}
/// Documentation for a module.
#[derive(Clone, Debug)]
pub struct DocModule {
    /// Fully-qualified module name.
    pub name: String,
    /// Doc comment at the module level.
    pub doc_comment: Option<String>,
    /// Items documented within this module.
    pub items: Vec<DocItem>,
    /// Names of submodules.
    pub submodules: Vec<String>,
    /// Import statements.
    pub imports: Vec<String>,
}
impl DocModule {
    /// Create a new empty doc module.
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            doc_comment: None,
            items: Vec::new(),
            submodules: Vec::new(),
            imports: Vec::new(),
        }
    }
    /// Add an item to the module.
    pub fn add_item(&mut self, item: DocItem) {
        self.items.push(item);
    }
    /// Count items by kind.
    pub fn count_by_kind(&self) -> HashMap<DocItemKind, usize> {
        let mut counts = HashMap::new();
        for item in &self.items {
            *counts.entry(item.kind()).or_insert(0) += 1;
        }
        counts
    }
}
/// Documentation for a tactic.
#[derive(Clone, Debug)]
pub struct DocTactic {
    /// Tactic name.
    pub name: String,
    /// Usage synopsis.
    pub synopsis: String,
    /// Doc comment.
    pub doc_comment: Option<String>,
    /// Source location.
    pub source_location: Option<SourceLocation>,
    /// Example usages.
    pub examples: Vec<String>,
}
/// Configuration for HTML generation.
#[derive(Clone, Debug)]
pub struct HtmlConfig {
    /// Title for the documentation.
    pub title: String,
    /// CSS theme name or inline CSS.
    pub css_theme: String,
    /// Whether to include source code links.
    pub include_source: bool,
    /// Whether to include proof terms.
    pub include_proofs: bool,
    /// Base URL for cross-references.
    pub base_url: String,
}
impl HtmlConfig {
    /// Create a default HTML configuration.
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
            css_theme: default_css().to_string(),
            include_source: true,
            include_proofs: false,
            base_url: ".".to_string(),
        }
    }
}
/// A method in a type class.
#[derive(Clone, Debug)]
pub struct DocMethod {
    /// Method name.
    pub name: String,
    /// Pretty-printed type signature.
    pub type_sig: String,
    /// Doc comment.
    pub doc_comment: Option<String>,
    /// Whether this method has a default implementation.
    pub has_default: bool,
}
/// Documentation for a theorem.
#[derive(Clone, Debug)]
pub struct DocTheorem {
    /// Fully-qualified name.
    pub name: String,
    /// Pretty-printed statement (proposition type).
    pub statement: String,
    /// Proof status.
    pub proof_status: ProofStatus,
    /// Doc comment.
    pub doc_comment: Option<String>,
    /// Source location.
    pub source_location: Option<SourceLocation>,
    /// Attributes.
    pub attributes: Vec<String>,
}
/// A field in a structure.
#[derive(Clone, Debug)]
pub struct DocField {
    /// Field name.
    pub name: String,
    /// Pretty-printed type.
    pub type_sig: String,
    /// Default value (if any).
    pub default_value: Option<String>,
    /// Doc comment.
    pub doc_comment: Option<String>,
}
