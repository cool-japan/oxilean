//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use std::collections::HashMap;
use std::io::Write;

/// Statistics for a LaTeX export run.
#[allow(dead_code)]
#[derive(Clone, Debug, Default)]
pub struct LatexExportStats {
    pub definitions_exported: usize,
    pub theorems_exported: usize,
    pub proofs_exported: usize,
    pub axioms_exported: usize,
    pub total_lines: usize,
}
impl LatexExportStats {
    /// Create new zeroed stats.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    /// Return a formatted summary.
    #[allow(dead_code)]
    pub fn summary(&self) -> String {
        format!(
            "LaTeX Export: {} defs, {} theorems, {} proofs, {} axioms ({} lines)",
            self.definitions_exported,
            self.theorems_exported,
            self.proofs_exported,
            self.axioms_exported,
            self.total_lines,
        )
    }
}
/// A bibliography entry.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct LatexBibEntry {
    pub key: String,
    pub entry_type: String,
    pub fields: HashMap<String, String>,
}
impl LatexBibEntry {
    /// Create a new bib entry.
    #[allow(dead_code)]
    pub fn new(key: impl Into<String>, entry_type: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            entry_type: entry_type.into(),
            fields: HashMap::new(),
        }
    }
    /// Add a field.
    #[allow(dead_code)]
    pub fn with_field(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.fields.insert(name.into(), value.into());
        self
    }
    /// Render as BibTeX.
    #[allow(dead_code)]
    pub fn to_bibtex(&self) -> String {
        let mut out = format!("@{}{{{},\n", self.entry_type, self.key);
        let mut fields: Vec<(&String, &String)> = self.fields.iter().collect();
        fields.sort_by_key(|(k, _)| k.as_str());
        for (name, value) in fields {
            out.push_str(&format!("  {} = {{{}}},\n", name, value));
        }
        out.push_str("}\n");
        out
    }
}
/// An equation block.
#[derive(Clone, Debug)]
pub struct EquationBlock {
    /// The equations.
    pub equations: Vec<String>,
    /// Whether to number equations.
    pub numbered: bool,
    /// Whether to use align environment.
    pub aligned: bool,
}
/// A library of reusable LaTeX macros.
#[allow(dead_code)]
pub struct LatexMacroLibrary {
    macros: Vec<LatexMacro>,
}
impl LatexMacroLibrary {
    /// Create a new library with default OxiLean macros.
    #[allow(dead_code)]
    pub fn default_oxilean_macros() -> Self {
        let macros = vec![
            LatexMacro::new("oxilean", 0, "\\texttt{OxiLean}", "OxiLean name"),
            LatexMacro::new("typedef", 2, "\\mathtt{#1} : #2", "Type annotation"),
            LatexMacro::new("defeq", 0, "\\mathrel{:=}", "Definition equality"),
            LatexMacro::new("lean", 1, "\\texttt{#1}", "Lean code"),
            LatexMacro::new("thm", 1, "\\textbf{#1}", "Theorem reference"),
            LatexMacro::new("hyp", 1, "\\mathit{#1}", "Hypothesis name"),
            LatexMacro::new("tac", 1, "\\texttt{#1}", "Tactic name"),
            LatexMacro::new("type", 1, "\\mathsf{#1}", "Type name"),
        ];
        Self { macros }
    }
    /// Add a macro.
    #[allow(dead_code)]
    pub fn add(&mut self, macro_: LatexMacro) {
        self.macros.push(macro_);
    }
    /// Find a macro by name.
    #[allow(dead_code)]
    pub fn find(&self, name: &str) -> Option<&LatexMacro> {
        self.macros.iter().find(|m| m.name == name)
    }
    /// Render all macros as LaTeX preamble.
    #[allow(dead_code)]
    pub fn render_preamble(&self) -> String {
        self.macros.iter().map(|m| m.to_latex()).collect()
    }
}
/// Maps OxiLean symbols to their LaTeX representations.
#[allow(dead_code)]
pub struct LatexSymbolTable {
    map: HashMap<String, String>,
}
impl LatexSymbolTable {
    /// Create a new table with standard OxiLean-to-LaTeX mappings.
    #[allow(dead_code)]
    pub fn new() -> Self {
        let mut map = HashMap::new();
        map.insert("forall".to_string(), "\\forall".to_string());
        map.insert("exists".to_string(), "\\exists".to_string());
        map.insert("->".to_string(), "\\to".to_string());
        map.insert("<->".to_string(), "\\leftrightarrow".to_string());
        map.insert("/\\".to_string(), "\\wedge".to_string());
        map.insert("\\/".to_string(), "\\vee".to_string());
        map.insert("~".to_string(), "\\neg".to_string());
        map.insert("not".to_string(), "\\neg".to_string());
        map.insert("<=".to_string(), "\\leq".to_string());
        map.insert(">=".to_string(), "\\geq".to_string());
        map.insert("!=".to_string(), "\\neq".to_string());
        map.insert("=".to_string(), "=".to_string());
        map.insert("Nat".to_string(), "\\mathbb{N}".to_string());
        map.insert("Int".to_string(), "\\mathbb{Z}".to_string());
        map.insert("Real".to_string(), "\\mathbb{R}".to_string());
        map.insert("Complex".to_string(), "\\mathbb{C}".to_string());
        map.insert("Bool".to_string(), "\\mathbb{B}".to_string());
        map.insert("Prop".to_string(), "\\mathsf{Prop}".to_string());
        map.insert("Type".to_string(), "\\mathsf{Type}".to_string());
        map.insert("lambda".to_string(), "\\lambda".to_string());
        map.insert("fun".to_string(), "\\lambda".to_string());
        map.insert("alpha".to_string(), "\\alpha".to_string());
        map.insert("beta".to_string(), "\\beta".to_string());
        map.insert("gamma".to_string(), "\\gamma".to_string());
        map.insert("pi".to_string(), "\\pi".to_string());
        map.insert("sigma".to_string(), "\\sigma".to_string());
        map.insert("omega".to_string(), "\\omega".to_string());
        map.insert("subset".to_string(), "\\subseteq".to_string());
        map.insert("in".to_string(), "\\in".to_string());
        map.insert("empty".to_string(), "\\emptyset".to_string());
        Self { map }
    }
    /// Translate a token to its LaTeX form (or return as-is if unknown).
    #[allow(dead_code)]
    pub fn translate(&self, token: &str) -> String {
        self.map
            .get(token)
            .cloned()
            .unwrap_or_else(|| token.to_string())
    }
    /// Add or override a mapping.
    #[allow(dead_code)]
    pub fn add(&mut self, from: impl Into<String>, to: impl Into<String>) {
        self.map.insert(from.into(), to.into());
    }
    /// Return all mapped symbols as a sorted list.
    #[allow(dead_code)]
    pub fn all_symbols(&self) -> Vec<(&str, &str)> {
        let mut pairs: Vec<(&str, &str)> = self
            .map
            .iter()
            .map(|(k, v)| (k.as_str(), v.as_str()))
            .collect();
        pairs.sort_by_key(|(k, _)| *k);
        pairs
    }
}
/// Converts OxiLean surface expressions to LaTeX math mode.
pub struct ExprToLatex {
    /// Symbol mappings (OxiLean name -> LaTeX command).
    symbol_map: HashMap<String, String>,
    /// Whether to use semantic macros.
    pub use_semantic: bool,
}
impl ExprToLatex {
    /// Create a new converter with standard mappings.
    pub fn new() -> Self {
        let mut symbol_map = HashMap::new();
        symbol_map.insert("Prop".to_string(), "\\mathrm{Prop}".to_string());
        symbol_map.insert("Type".to_string(), "\\mathrm{Type}".to_string());
        symbol_map.insert("Sort".to_string(), "\\mathrm{Sort}".to_string());
        symbol_map.insert("Nat".to_string(), "\\mathbb{N}".to_string());
        symbol_map.insert("Int".to_string(), "\\mathbb{Z}".to_string());
        symbol_map.insert("Real".to_string(), "\\mathbb{R}".to_string());
        symbol_map.insert("Bool".to_string(), "\\mathbb{B}".to_string());
        symbol_map.insert("Unit".to_string(), "\\mathbf{1}".to_string());
        symbol_map.insert("Empty".to_string(), "\\bot".to_string());
        symbol_map.insert("And".to_string(), "\\land".to_string());
        symbol_map.insert("Or".to_string(), "\\lor".to_string());
        symbol_map.insert("Not".to_string(), "\\lnot".to_string());
        symbol_map.insert("True".to_string(), "\\top".to_string());
        symbol_map.insert("False".to_string(), "\\bot".to_string());
        symbol_map.insert("Iff".to_string(), "\\leftrightarrow".to_string());
        symbol_map.insert("+".to_string(), "+".to_string());
        symbol_map.insert("-".to_string(), "-".to_string());
        symbol_map.insert("*".to_string(), "\\cdot".to_string());
        symbol_map.insert("/".to_string(), "/".to_string());
        symbol_map.insert("^".to_string(), "^".to_string());
        symbol_map.insert("=".to_string(), "=".to_string());
        symbol_map.insert("<".to_string(), "<".to_string());
        symbol_map.insert(">".to_string(), ">".to_string());
        symbol_map.insert("<=".to_string(), "\\leq".to_string());
        symbol_map.insert(">=".to_string(), "\\geq".to_string());
        symbol_map.insert("!=".to_string(), "\\neq".to_string());
        symbol_map.insert("forall".to_string(), "\\forall".to_string());
        symbol_map.insert("exists".to_string(), "\\exists".to_string());
        symbol_map.insert("fun".to_string(), "\\lambda".to_string());
        symbol_map.insert("->".to_string(), "\\to".to_string());
        symbol_map.insert("=>".to_string(), "\\Rightarrow".to_string());
        symbol_map.insert("Set".to_string(), "\\mathrm{Set}".to_string());
        symbol_map.insert("Finset".to_string(), "\\mathrm{Finset}".to_string());
        symbol_map.insert("alpha".to_string(), "\\alpha".to_string());
        symbol_map.insert("beta".to_string(), "\\beta".to_string());
        symbol_map.insert("gamma".to_string(), "\\gamma".to_string());
        symbol_map.insert("delta".to_string(), "\\delta".to_string());
        symbol_map.insert("epsilon".to_string(), "\\epsilon".to_string());
        symbol_map.insert("sigma".to_string(), "\\sigma".to_string());
        symbol_map.insert("tau".to_string(), "\\tau".to_string());
        symbol_map.insert("phi".to_string(), "\\varphi".to_string());
        symbol_map.insert("psi".to_string(), "\\psi".to_string());
        symbol_map.insert("omega".to_string(), "\\omega".to_string());
        Self {
            symbol_map,
            use_semantic: true,
        }
    }
    /// Add a custom symbol mapping.
    pub fn add_symbol(&mut self, oxilean_name: String, latex: String) {
        self.symbol_map.insert(oxilean_name, latex);
    }
    /// Convert a name to LaTeX.
    pub fn convert_name(&self, name: &str) -> String {
        if let Some(latex) = self.symbol_map.get(name) {
            return latex.clone();
        }
        if name.contains('.') {
            let parts: Vec<&str> = name.split('.').collect();
            let last = parts.last().unwrap_or(&name);
            if let Some(latex) = self.symbol_map.get(*last) {
                return latex.clone();
            }
            return format!("\\mathrm{{{}}}", escape_latex(name));
        }
        if name.len() == 1
            && name
                .chars()
                .next()
                .expect("name is non-empty: len() == 1")
                .is_alphabetic()
        {
            return name.to_string();
        }
        if name.chars().all(|c| c.is_alphanumeric() || c == '_') {
            format!("\\mathrm{{{}}}", escape_latex(name))
        } else {
            escape_latex(name)
        }
    }
    /// Convert a type expression to LaTeX.
    pub fn convert_type(&self, ty: &str) -> String {
        let parts: Vec<&str> = ty.split("->").collect();
        if parts.len() > 1 {
            let converted: Vec<String> =
                parts.iter().map(|p| self.convert_name(p.trim())).collect();
            return converted.join(" \\to ");
        }
        self.convert_name(ty)
    }
    /// Convert a Pi type to LaTeX.
    pub fn convert_pi(&self, var: &str, ty: &str, body: &str) -> String {
        format!(
            "\\Pi ({} : {}),\\, {}",
            self.convert_name(var),
            self.convert_type(ty),
            body
        )
    }
    /// Convert a lambda to LaTeX.
    pub fn convert_lambda(&self, var: &str, ty: &str, body: &str) -> String {
        format!(
            "\\lambda ({} : {}),\\, {}",
            self.convert_name(var),
            self.convert_type(ty),
            body
        )
    }
    /// Convert a forall expression to LaTeX.
    pub fn convert_forall(&self, var: &str, ty: &str, body: &str) -> String {
        format!(
            "\\forall ({} : {}),\\, {}",
            self.convert_name(var),
            self.convert_type(ty),
            body
        )
    }
    /// Convert an application to LaTeX.
    pub fn convert_app(&self, func: &str, arg: &str) -> String {
        format!("{}\\;{}", func, arg)
    }
}
/// A node in a proof tree for tree-style LaTeX rendering.
#[derive(Clone, Debug)]
pub struct ProofTreeNode {
    /// The sequent or formula at this node.
    pub formula: String,
    /// Optional rule name applied.
    pub rule: Option<String>,
    /// Child nodes (premises).
    pub children: Vec<ProofTreeNode>,
}
impl ProofTreeNode {
    /// Create a leaf node (axiom).
    pub fn leaf(formula: impl Into<String>) -> Self {
        Self {
            formula: formula.into(),
            rule: None,
            children: Vec::new(),
        }
    }
    /// Create an internal node with a rule name.
    pub fn node(
        formula: impl Into<String>,
        rule: impl Into<String>,
        children: Vec<ProofTreeNode>,
    ) -> Self {
        Self {
            formula: formula.into(),
            rule: Some(rule.into()),
            children,
        }
    }
    /// Render this proof tree to a `bussproofs` LaTeX string.
    pub fn render_bussproofs(&self) -> String {
        let mut s = String::new();
        self.render_bp_impl(&mut s);
        s
    }
    fn render_bp_impl(&self, out: &mut String) {
        match self.children.len() {
            0 => {
                out.push_str("\\AxiomC{$");
                out.push_str(&self.formula);
                out.push_str("$}\n");
            }
            1 => {
                self.children[0].render_bp_impl(out);
                let rule = self.rule.as_deref().unwrap_or("");
                out.push_str(&format!("\\UnaryInfC[${}$]{{${} $}}\n", rule, self.formula));
            }
            2 => {
                self.children[0].render_bp_impl(out);
                self.children[1].render_bp_impl(out);
                let rule = self.rule.as_deref().unwrap_or("");
                out.push_str(&format!(
                    "\\BinaryInfC[${}$]{{${} $}}\n",
                    rule, self.formula
                ));
            }
            _ => {
                for child in &self.children {
                    child.render_bp_impl(out);
                }
                let rule = self.rule.as_deref().unwrap_or("");
                out.push_str(&format!(
                    "\\TrinaryInfC[${}$]{{${} $}}\n",
                    rule, self.formula
                ));
            }
        }
    }
    /// Count total nodes.
    pub fn size(&self) -> usize {
        1 + self.children.iter().map(|c| c.size()).sum::<usize>()
    }
    /// Depth of the tree.
    pub fn depth(&self) -> usize {
        if self.children.is_empty() {
            1
        } else {
            1 + self.children.iter().map(|c| c.depth()).max().unwrap_or(0)
        }
    }
}
/// A mathematical definition.
#[derive(Clone, Debug)]
pub struct MathDef {
    /// Name of the defined concept.
    pub name: String,
    /// Parameters.
    pub params: Vec<(String, String)>,
    /// Type expression (LaTeX math).
    pub ty: String,
    /// Definition body (LaTeX math).
    pub body: Option<String>,
    /// Optional label.
    pub label: Option<String>,
}
/// A node in a LaTeX proof tree (for bussproofs / proof environments).
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct LatexProofNode {
    pub conclusion: String,
    pub premises: Vec<LatexProofNode>,
    pub rule_name: Option<String>,
}
impl LatexProofNode {
    /// Create a leaf node.
    #[allow(dead_code)]
    pub fn leaf(conclusion: impl Into<String>) -> Self {
        Self {
            conclusion: conclusion.into(),
            premises: vec![],
            rule_name: None,
        }
    }
    /// Create a node with premises and an optional rule name.
    #[allow(dead_code)]
    pub fn node(
        conclusion: impl Into<String>,
        premises: Vec<LatexProofNode>,
        rule_name: Option<String>,
    ) -> Self {
        Self {
            conclusion: conclusion.into(),
            premises,
            rule_name,
        }
    }
    /// Render to bussproofs LaTeX.
    #[allow(dead_code)]
    pub fn render_bussproofs(&self) -> String {
        let n = self.premises.len();
        let inner: String = self
            .premises
            .iter()
            .map(|p| p.render_bussproofs())
            .collect();
        let rule = self
            .rule_name
            .as_deref()
            .map(|r| format!("\\RightLabel{{\\scriptsize {}}}", r))
            .unwrap_or_default();
        match n {
            0 => format!("\\AxiomC{{${}$}}\n", self.conclusion),
            1 => format!(
                "{}\n{}\n\\UnaryInfC{{${}$}}\n",
                inner, rule, self.conclusion
            ),
            2 => format!(
                "{}\n{}\n\\BinaryInfC{{${}$}}\n",
                inner, rule, self.conclusion
            ),
            3 => format!(
                "{}\n{}\n\\TrinaryInfC{{${}$}}\n",
                inner, rule, self.conclusion
            ),
            _ => format!("{}\n\\noLine\\UnaryInfC{{${}$}}\n", inner, self.conclusion),
        }
    }
    /// Render to a simple indented text (for debugging).
    #[allow(dead_code)]
    pub fn render_text(&self, depth: usize) -> String {
        let indent = "  ".repeat(depth);
        let rule_str = self
            .rule_name
            .as_deref()
            .map(|r| format!(" [{}]", r))
            .unwrap_or_default();
        let mut out = format!("{}{}{}\n", indent, self.conclusion, rule_str);
        for p in &self.premises {
            out.push_str(&p.render_text(depth + 1));
        }
        out
    }
}
/// A builder for full LaTeX documents.
#[allow(dead_code)]
pub struct LatexDocumentBuilder {
    document_class: String,
    packages: Vec<String>,
    preamble_extras: Vec<String>,
    title: Option<String>,
    author: Option<String>,
    sections: Vec<LatexSection>,
}
impl LatexDocumentBuilder {
    /// Create a new builder.
    #[allow(dead_code)]
    pub fn new(document_class: impl Into<String>) -> Self {
        Self {
            document_class: document_class.into(),
            packages: vec![],
            preamble_extras: vec![],
            title: None,
            author: None,
            sections: vec![],
        }
    }
    /// Add a package.
    #[allow(dead_code)]
    pub fn add_package(mut self, package: impl Into<String>) -> Self {
        self.packages.push(package.into());
        self
    }
    /// Add preamble content.
    #[allow(dead_code)]
    pub fn add_preamble(mut self, content: impl Into<String>) -> Self {
        self.preamble_extras.push(content.into());
        self
    }
    /// Set the title.
    #[allow(dead_code)]
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }
    /// Set the author.
    #[allow(dead_code)]
    pub fn with_author(mut self, author: impl Into<String>) -> Self {
        self.author = Some(author.into());
        self
    }
    /// Add a section.
    #[allow(dead_code)]
    pub fn add_section(mut self, section: LatexSection) -> Self {
        self.sections.push(section);
        self
    }
    /// Build the complete LaTeX document string.
    #[allow(dead_code)]
    pub fn build(&self) -> String {
        let mut out = format!("\\documentclass{{{}}}\n", self.document_class);
        for pkg in &self.packages {
            out.push_str(&format!("\\usepackage{{{}}}\n", pkg));
        }
        for extra in &self.preamble_extras {
            out.push_str(extra);
            out.push('\n');
        }
        if let Some(ref title) = self.title {
            out.push_str(&format!("\\title{{{}}}\n", title));
        }
        if let Some(ref author) = self.author {
            out.push_str(&format!("\\author{{{}}}\n", author));
        }
        out.push_str("\\begin{document}\n");
        if self.title.is_some() {
            out.push_str("\\maketitle\n");
        }
        for section in &self.sections {
            let cmd = section.level.command();
            out.push_str(&format!("\\{}{{{}}}\n", cmd, section.title));
            if let Some(ref label) = section.label {
                out.push_str(&format!("\\label{{{}}}\n", label));
            }
            out.push_str(&section.content);
            out.push('\n');
        }
        out.push_str("\\end{document}\n");
        out
    }
}
/// TikZ shape.
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TikzShape {
    Rectangle,
    Circle,
    Ellipse,
    Diamond,
    Plain,
}
impl TikzShape {
    /// Return the TikZ draw style string.
    #[allow(dead_code)]
    pub fn style_str(&self) -> &'static str {
        match self {
            TikzShape::Rectangle => "rectangle,draw",
            TikzShape::Circle => "circle,draw",
            TikzShape::Ellipse => "ellipse,draw",
            TikzShape::Diamond => "diamond,draw",
            TikzShape::Plain => "",
        }
    }
}
/// A TikZ diagram builder.
#[allow(dead_code)]
pub struct TikzDiagram {
    nodes: Vec<TikzNode>,
    edges: Vec<TikzEdge>,
}
impl TikzDiagram {
    /// Create a new diagram.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            nodes: vec![],
            edges: vec![],
        }
    }
    /// Add a node.
    #[allow(dead_code)]
    pub fn add_node(&mut self, node: TikzNode) {
        self.nodes.push(node);
    }
    /// Add an edge.
    #[allow(dead_code)]
    pub fn add_edge(&mut self, edge: TikzEdge) {
        self.edges.push(edge);
    }
    /// Render the diagram as a TikZ picture.
    #[allow(dead_code)]
    pub fn render(&self) -> String {
        let mut out = String::from("\\begin{tikzpicture}\n");
        for node in &self.nodes {
            let style = node.shape.style_str();
            if style.is_empty() {
                out.push_str(&format!(
                    "  \\node ({}) at ({},{}) {{{}}};\n",
                    node.id, node.x, node.y, node.label
                ));
            } else {
                out.push_str(&format!(
                    "  \\node[{}] ({}) at ({},{}) {{{}}};\n",
                    style, node.id, node.x, node.y, node.label
                ));
            }
        }
        for edge in &self.edges {
            let label_str = edge
                .label
                .as_deref()
                .map(|l| format!("node {{{}}} ", l))
                .unwrap_or_default();
            out.push_str(&format!(
                "  \\draw[{}] ({}) -- {}({});\n",
                edge.style, edge.from, label_str, edge.to
            ));
        }
        out.push_str("\\end{tikzpicture}\n");
        out
    }
}
/// A section of a LaTeX document.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct LatexSection {
    pub level: SectionLevel,
    pub title: String,
    pub content: String,
    pub label: Option<String>,
}
/// Configuration for LaTeX export.
#[derive(Clone, Debug)]
pub struct LatexConfig {
    /// Document class (e.g., "article", "book", "amsart").
    pub document_class: String,
    /// Additional packages to include.
    pub packages: Vec<String>,
    /// Whether to use AMS math packages.
    pub use_ams: bool,
    /// Whether to use hyperref.
    pub use_hyperref: bool,
    /// Whether to include proof environments.
    pub include_proofs: bool,
    /// Whether to number theorems.
    pub number_theorems: bool,
    /// Custom preamble text.
    pub custom_preamble: String,
    /// Custom macros.
    pub macros: HashMap<String, String>,
    /// Title of the document.
    pub title: Option<String>,
    /// Author of the document.
    pub author: Option<String>,
    /// Whether to generate a full document (with \begin{document}).
    pub full_document: bool,
    /// Font size.
    pub font_size: String,
    /// Encoding.
    pub encoding: String,
    /// Paper size.
    pub paper_size: String,
}
impl LatexConfig {
    /// Create a minimal configuration (no full document wrapper).
    pub fn minimal() -> Self {
        Self {
            full_document: false,
            include_proofs: true,
            ..Default::default()
        }
    }
    /// Add a custom LaTeX macro.
    pub fn add_macro(&mut self, name: String, definition: String) {
        self.macros.insert(name, definition);
    }
    /// Add a package.
    pub fn add_package(&mut self, package: String) {
        if !self.packages.contains(&package) {
            self.packages.push(package);
        }
    }
}
/// Style of proof export.
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ProofExportStyle {
    /// Human-readable prose proof
    Prose,
    /// Itemized step-by-step proof
    Itemized,
    /// Bussproofs proof tree
    BussProofs,
    /// ND (natural deduction) style
    NaturalDeduction,
}
/// A proof environment.
#[derive(Clone, Debug)]
pub struct ProofEnv {
    /// Proof steps (each is a LaTeX fragment).
    pub steps: Vec<String>,
    /// Optional qed symbol override.
    pub qed_symbol: Option<String>,
}
/// An edge in a TikZ diagram.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct TikzEdge {
    pub from: String,
    pub to: String,
    pub label: Option<String>,
    pub style: String,
}
/// A library of frequently used LaTeX macros for OxiLean output.
#[derive(Clone, Debug, Default)]
pub struct LatexMacroLib {
    macros: Vec<(String, u32, String)>,
}
impl LatexMacroLib {
    /// Create an empty library.
    pub fn new() -> Self {
        Self::default()
    }
    /// Create with OxiLean standard macros.
    pub fn oxilean_standard() -> Self {
        let mut lib = Self::new();
        lib.add_macro("Type", 0, "\\mathrm{Type}");
        lib.add_macro("Prop", 0, "\\mathrm{Prop}");
        lib.add_macro("Nat", 0, "\\mathbb{N}");
        lib.add_macro("Int", 0, "\\mathbb{Z}");
        lib.add_macro("Real", 0, "\\mathbb{R}");
        lib.add_macro("Sort", 1, "\\mathrm{Sort}_{#1}");
        lib.add_macro("OxFun", 2, "#1 \\to #2");
        lib.add_macro("OxForall", 3, "\\forall (#1 : #2),\\, #3");
        lib.add_macro("OxExists", 3, "\\exists (#1 : #2),\\, #3");
        lib.add_macro("OxEq", 3, "#2 =_{#1} #3");
        lib
    }
    /// Add a macro.
    pub fn add_macro(&mut self, name: &str, arity: u32, def: &str) {
        self.macros.push((name.to_string(), arity, def.to_string()));
    }
    /// Render all macros as `\newcommand` declarations.
    pub fn render_preamble(&self) -> String {
        let mut s = String::new();
        for (name, arity, def) in &self.macros {
            if *arity == 0 {
                s.push_str(&format!("\\newcommand{{\\{}}}{{{}}} \n", name, def));
            } else {
                s.push_str(&format!(
                    "\\newcommand{{\\{}}}[{}]{{{}}} \n",
                    name, arity, def
                ));
            }
        }
        s
    }
    /// Number of macros.
    pub fn len(&self) -> usize {
        self.macros.len()
    }
    /// Check if empty.
    pub fn is_empty(&self) -> bool {
        self.macros.is_empty()
    }
}
/// Exports OxiLean proof steps to LaTeX proof environments.
#[allow(dead_code)]
pub struct LatexProofExporter {
    pub style: ProofExportStyle,
    pub symbol_table: LatexSymbolTable,
}
impl LatexProofExporter {
    /// Create a new exporter.
    #[allow(dead_code)]
    pub fn new(style: ProofExportStyle) -> Self {
        Self {
            style,
            symbol_table: LatexSymbolTable::new(),
        }
    }
    /// Export a list of proof steps to LaTeX.
    #[allow(dead_code)]
    pub fn export_steps(&self, steps: &[ProofStep]) -> String {
        match self.style {
            ProofExportStyle::Prose => self.export_prose(steps),
            ProofExportStyle::Itemized => self.export_itemized(steps),
            ProofExportStyle::BussProofs => self.export_bussproofs(steps),
            ProofExportStyle::NaturalDeduction => self.export_nd(steps),
        }
    }
    fn export_prose(&self, steps: &[ProofStep]) -> String {
        let mut out = String::from("\\begin{proof}\n");
        for (i, step) in steps.iter().enumerate() {
            if i == 0 {
                out.push_str(&format!("We first observe that {}.\n", step.statement));
            } else if i == steps.len() - 1 {
                out.push_str(&format!("Therefore, {}.\n", step.statement));
            } else {
                out.push_str(&format!("Next, {}.\n", step.statement));
            }
        }
        out.push_str("\\end{proof}\n");
        out
    }
    fn export_itemized(&self, steps: &[ProofStep]) -> String {
        let mut out = String::from("\\begin{proof}\n\\begin{enumerate}\n");
        for step in steps {
            let justification = step
                .justification
                .as_deref()
                .map(|j| format!(" \\hfill ({})", j))
                .unwrap_or_default();
            out.push_str(&format!(
                "\\item {} {}\\label{{{}}}\n",
                step.statement, justification, step.id
            ));
        }
        out.push_str("\\end{enumerate}\n\\end{proof}\n");
        out
    }
    fn export_bussproofs(&self, steps: &[ProofStep]) -> String {
        let mut out = String::from("\\begin{prooftree}\n");
        for step in steps {
            let prem_count = step.premise_ids.len();
            match prem_count {
                0 => out.push_str(&format!("\\AxiomC{{${}$}}\n", step.statement)),
                1 => out.push_str(&format!("\\UnaryInfC{{${}$}}\n", step.statement)),
                2 => out.push_str(&format!("\\BinaryInfC{{${}$}}\n", step.statement)),
                _ => out.push_str(&format!("\\noLine\\UnaryInfC{{${}$}}\n", step.statement)),
            }
        }
        out.push_str("\\end{prooftree}\n");
        out
    }
    fn export_nd(&self, steps: &[ProofStep]) -> String {
        let mut out = String::from("\\begin{nd}\n");
        for step in steps {
            let rule = step.justification.as_deref().unwrap_or("assumption");
            out.push_str(&format!(
                "\\hypo{{{}}}{{${}$}}\n\\have{{{}}}{{${}$}}[{}]\n",
                step.id, step.statement, step.id, step.statement, rule
            ));
        }
        out.push_str("\\end{nd}\n");
        out
    }
}
/// Builds a LaTeX tabular environment.
#[derive(Clone, Debug)]
pub struct LatexTable {
    /// Column specification (e.g., "lcc|r").
    pub col_spec: String,
    /// Whether to use booktabs rules.
    pub booktabs: bool,
    /// Table rows.
    pub rows: Vec<Vec<String>>,
    /// Optional caption.
    pub caption: Option<String>,
    /// Optional label.
    pub label: Option<String>,
}
impl LatexTable {
    /// Create a new table with a column specification.
    pub fn new(col_spec: impl Into<String>) -> Self {
        Self {
            col_spec: col_spec.into(),
            booktabs: true,
            rows: Vec::new(),
            caption: None,
            label: None,
        }
    }
    /// Add a row.
    pub fn add_row(&mut self, cells: Vec<String>) {
        self.rows.push(cells);
    }
    /// Add a header row (bold cells).
    pub fn add_header(&mut self, cells: Vec<impl Into<String>>) {
        let bold_cells: Vec<String> = cells
            .into_iter()
            .map(|c| format!("\\textbf{{{}}}", c.into()))
            .collect();
        self.rows.push(bold_cells);
    }
    /// Set caption.
    pub fn set_caption(&mut self, caption: impl Into<String>) {
        self.caption = Some(caption.into());
    }
    /// Set label.
    pub fn set_label(&mut self, label: impl Into<String>) {
        self.label = Some(label.into());
    }
    /// Render to LaTeX string.
    pub fn render(&self) -> String {
        let mut s = String::new();
        s.push_str("\\begin{table}[h]\n\\centering\n");
        if let Some(ref cap) = self.caption {
            s.push_str(&format!("\\caption{{{}}}\n", cap));
        }
        if let Some(ref lab) = self.label {
            s.push_str(&format!("\\label{{{}}}\n", lab));
        }
        s.push_str(&format!("\\begin{{tabular}}{{{}}}\n", self.col_spec));
        if self.booktabs {
            s.push_str("\\toprule\n");
        } else {
            s.push_str("\\hline\n");
        }
        for (i, row) in self.rows.iter().enumerate() {
            s.push_str(&row.join(" & "));
            s.push_str(" \\\\\n");
            if i == 0 {
                s.push_str(if self.booktabs {
                    "\\midrule\n"
                } else {
                    "\\hline\n"
                });
            }
        }
        if self.booktabs {
            s.push_str("\\bottomrule\n");
        } else {
            s.push_str("\\hline\n");
        }
        s.push_str("\\end{tabular}\n");
        s.push_str("\\end{table}\n");
        s
    }
}
/// Section level.
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SectionLevel {
    Part,
    Chapter,
    Section,
    Subsection,
    Subsubsection,
    Paragraph,
}
impl SectionLevel {
    /// Return the LaTeX command name.
    #[allow(dead_code)]
    pub fn command(&self) -> &'static str {
        match self {
            SectionLevel::Part => "part",
            SectionLevel::Chapter => "chapter",
            SectionLevel::Section => "section",
            SectionLevel::Subsection => "subsection",
            SectionLevel::Subsubsection => "subsubsection",
            SectionLevel::Paragraph => "paragraph",
        }
    }
}
/// A LaTeX macro definition.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct LatexMacro {
    pub name: String,
    pub arg_count: u8,
    pub definition: String,
    pub description: String,
}
impl LatexMacro {
    /// Create a new macro.
    #[allow(dead_code)]
    pub fn new(
        name: impl Into<String>,
        arg_count: u8,
        definition: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            arg_count,
            definition: definition.into(),
            description: description.into(),
        }
    }
    /// Render as a \newcommand declaration.
    #[allow(dead_code)]
    pub fn to_latex(&self) -> String {
        if self.arg_count == 0 {
            format!(
                "\\newcommand{{\\{}}}{{{}}}% {}\n",
                self.name, self.definition, self.description
            )
        } else {
            format!(
                "\\newcommand{{\\{}}}[{}]{{{}}}% {}\n",
                self.name, self.arg_count, self.definition, self.description
            )
        }
    }
}
/// Helper for building a LaTeX section with numbered theorems and proofs.
#[derive(Clone, Debug)]
pub struct LatexSectionBlock {
    /// Section title.
    pub title: String,
    /// Section level (1=section, 2=subsection, …).
    pub level: u8,
    /// Content blocks (rendered LaTeX).
    pub blocks: Vec<String>,
    /// Theorem counter (for custom numbering).
    pub theorem_count: usize,
}
impl LatexSectionBlock {
    /// Create a new section.
    pub fn new(title: impl Into<String>, level: u8) -> Self {
        Self {
            title: title.into(),
            level,
            blocks: Vec::new(),
            theorem_count: 0,
        }
    }
    /// Add a text block.
    pub fn add_text(&mut self, text: impl Into<String>) {
        self.blocks.push(text.into());
    }
    /// Add a theorem block.
    pub fn add_theorem(&mut self, statement: &str, proof: Option<&str>) {
        self.theorem_count += 1;
        let mut block = format!("\\begin{{theorem}}\n${}$\n\\end{{theorem}}\n", statement);
        if let Some(p) = proof {
            block.push_str(&format!("\\begin{{proof}}\n{}\n\\end{{proof}}\n", p));
        }
        self.blocks.push(block);
    }
    /// Render the section to LaTeX.
    pub fn render(&self) -> String {
        let cmd = match self.level {
            1 => "section",
            2 => "subsection",
            _ => "subsubsection",
        };
        let mut s = format!("\\{}{{{}}}\n\n", cmd, escape_latex(&self.title));
        for block in &self.blocks {
            s.push_str(block);
            s.push('\n');
        }
        s
    }
    /// Number of content blocks.
    pub fn block_count(&self) -> usize {
        self.blocks.len()
    }
}
/// A theorem-like environment (theorem, lemma, corollary, etc.).
#[derive(Clone, Debug)]
pub struct TheoremEnv {
    /// Environment name (theorem, lemma, proposition, etc.).
    pub env_name: String,
    /// Optional theorem name/title.
    pub title: Option<String>,
    /// The statement (LaTeX math).
    pub statement: String,
    /// Optional label for cross-referencing.
    pub label: Option<String>,
}
/// A LaTeX document element.
#[derive(Clone, Debug)]
pub enum LatexElement {
    /// Raw LaTeX text.
    Raw(String),
    /// A section heading.
    Section(String, u8),
    /// A theorem-like environment.
    TheoremEnv(TheoremEnv),
    /// A proof environment.
    ProofEnv(ProofEnv),
    /// A definition in math mode.
    MathDef(MathDef),
    /// An equation or equation array.
    Equation(EquationBlock),
    /// A list of items.
    ItemList(Vec<String>),
    /// A comment (% in LaTeX).
    Comment(String),
    /// A label for cross-referencing.
    Label(String),
    /// A reference to a label.
    Ref(String),
    /// Blank line separator.
    BlankLine,
}
/// A node in a TikZ diagram.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct TikzNode {
    pub id: String,
    pub label: String,
    pub x: f64,
    pub y: f64,
    pub shape: TikzShape,
}
/// A LaTeX theorem-like environment.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct LatexTheoremEnv {
    pub env_name: String,
    pub display_name: String,
    pub counter: Option<String>,
}
impl LatexTheoremEnv {
    /// Create a new theorem environment definition.
    #[allow(dead_code)]
    pub fn new(
        env_name: impl Into<String>,
        display_name: impl Into<String>,
        counter: Option<String>,
    ) -> Self {
        Self {
            env_name: env_name.into(),
            display_name: display_name.into(),
            counter,
        }
    }
    /// Render the `\newtheorem` definition.
    #[allow(dead_code)]
    pub fn definition(&self) -> String {
        if let Some(ref counter) = self.counter {
            format!(
                "\\newtheorem{{{}}}[{}]{{{}}}\n",
                self.env_name, counter, self.display_name
            )
        } else {
            format!(
                "\\newtheorem{{{}}}{{{}}}\n",
                self.env_name, self.display_name
            )
        }
    }
    /// Render a theorem statement using this environment.
    #[allow(dead_code)]
    pub fn render(
        &self,
        name: Option<&str>,
        label: Option<&str>,
        statement: &str,
        proof: Option<&str>,
    ) -> String {
        let name_str = name.map(|n| format!("[{}]", n)).unwrap_or_default();
        let label_str = label
            .map(|l| format!("\\label{{{}}}\n", l))
            .unwrap_or_default();
        let proof_str = proof
            .map(|p| format!("\\begin{{proof}}\n{}\n\\end{{proof}}\n", p))
            .unwrap_or_default();
        format!(
            "\\begin{{{env}}}{name}\n{label}{statement}\n\\end{{{env}}}\n{proof}",
            env = self.env_name,
            name = name_str,
            label = label_str,
            statement = statement,
            proof = proof_str
        )
    }
    /// Standard theorem environments for a math paper.
    #[allow(dead_code)]
    pub fn standard_environments() -> Vec<Self> {
        vec![
            Self::new("theorem", "Theorem", None),
            Self::new("lemma", "Lemma", Some("theorem".to_string())),
            Self::new("corollary", "Corollary", Some("theorem".to_string())),
            Self::new("proposition", "Proposition", Some("theorem".to_string())),
            Self::new("definition", "Definition", Some("theorem".to_string())),
            Self::new("remark", "Remark", Some("theorem".to_string())),
            Self::new("example", "Example", Some("theorem".to_string())),
        ]
    }
}
/// A single step in a proof.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct ProofStep {
    pub id: String,
    pub statement: String,
    pub justification: Option<String>,
    pub premise_ids: Vec<String>,
}
impl ProofStep {
    /// Create an assumption step.
    #[allow(dead_code)]
    pub fn assumption(id: impl Into<String>, statement: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            statement: statement.into(),
            justification: Some("Assumption".to_string()),
            premise_ids: vec![],
        }
    }
    /// Create a derived step.
    #[allow(dead_code)]
    pub fn derived(
        id: impl Into<String>,
        statement: impl Into<String>,
        rule: impl Into<String>,
        premises: Vec<String>,
    ) -> Self {
        Self {
            id: id.into(),
            statement: statement.into(),
            justification: Some(rule.into()),
            premise_ids: premises,
        }
    }
}
/// Generates a complete LaTeX document from elements.
pub struct LatexDocument {
    /// Configuration.
    config: LatexConfig,
    /// Document elements.
    elements: Vec<LatexElement>,
    /// Expression converter.
    expr_converter: ExprToLatex,
}
impl LatexDocument {
    /// Create a new LaTeX document.
    pub fn new(config: LatexConfig) -> Self {
        Self {
            config,
            elements: Vec::new(),
            expr_converter: ExprToLatex::new(),
        }
    }
    /// Create with default configuration.
    pub fn default_doc() -> Self {
        Self::new(LatexConfig::default())
    }
    /// Add an element to the document.
    pub fn add(&mut self, element: LatexElement) {
        self.elements.push(element);
    }
    /// Add a section heading.
    pub fn add_section(&mut self, title: &str, level: u8) {
        self.elements
            .push(LatexElement::Section(title.to_string(), level));
    }
    /// Add a theorem.
    pub fn add_theorem(&mut self, name: Option<String>, statement: String, label: Option<String>) {
        self.elements.push(LatexElement::TheoremEnv(TheoremEnv {
            env_name: "theorem".to_string(),
            title: name,
            statement,
            label,
        }));
    }
    /// Add a lemma.
    pub fn add_lemma(&mut self, name: Option<String>, statement: String, label: Option<String>) {
        self.elements.push(LatexElement::TheoremEnv(TheoremEnv {
            env_name: "lemma".to_string(),
            title: name,
            statement,
            label,
        }));
    }
    /// Add a definition.
    pub fn add_definition(&mut self, def: MathDef) {
        self.elements.push(LatexElement::MathDef(def));
    }
    /// Add a proof.
    pub fn add_proof(&mut self, steps: Vec<String>) {
        self.elements.push(LatexElement::ProofEnv(ProofEnv {
            steps,
            qed_symbol: None,
        }));
    }
    /// Add a comment.
    pub fn add_comment(&mut self, text: &str) {
        self.elements.push(LatexElement::Comment(text.to_string()));
    }
    /// Add raw LaTeX.
    pub fn add_raw(&mut self, latex: &str) {
        self.elements.push(LatexElement::Raw(latex.to_string()));
    }
    /// Generate the preamble.
    fn generate_preamble(&self) -> String {
        let mut preamble = String::new();
        preamble.push_str(&format!(
            "\\documentclass[{},{}]{{{}}}\n",
            self.config.font_size, self.config.paper_size, self.config.document_class
        ));
        preamble.push('\n');
        preamble.push_str(&format!(
            "\\usepackage[{}]{{inputenc}}\n",
            self.config.encoding
        ));
        preamble.push_str("\\usepackage[T1]{fontenc}\n");
        preamble.push('\n');
        for pkg in &self.config.packages {
            preamble.push_str(&format!("\\usepackage{{{}}}\n", pkg));
        }
        if self.config.use_hyperref {
            preamble.push_str("\\usepackage{hyperref}\n");
        }
        preamble.push('\n');
        if self.config.use_ams {
            preamble.push_str("\\theoremstyle{plain}\n");
            if self.config.number_theorems {
                preamble.push_str("\\newtheorem{theorem}{Theorem}[section]\n");
                preamble.push_str("\\newtheorem{lemma}[theorem]{Lemma}\n");
                preamble.push_str("\\newtheorem{proposition}[theorem]{Proposition}\n");
                preamble.push_str("\\newtheorem{corollary}[theorem]{Corollary}\n");
            } else {
                preamble.push_str("\\newtheorem*{theorem}{Theorem}\n");
                preamble.push_str("\\newtheorem*{lemma}{Lemma}\n");
                preamble.push_str("\\newtheorem*{proposition}{Proposition}\n");
                preamble.push_str("\\newtheorem*{corollary}{Corollary}\n");
            }
            preamble.push_str("\\theoremstyle{definition}\n");
            preamble.push_str("\\newtheorem{definition}[theorem]{Definition}\n");
            preamble.push_str("\\newtheorem{example}[theorem]{Example}\n");
            preamble.push_str("\\theoremstyle{remark}\n");
            preamble.push_str("\\newtheorem*{remark}{Remark}\n");
            preamble.push('\n');
        }
        for (name, def) in &self.config.macros {
            preamble.push_str(&format!("\\newcommand{{\\{}}}{{{}}}\n", name, def));
        }
        preamble.push_str("\n% OxiLean-specific macros\n");
        preamble.push_str("\\newcommand{\\oxtype}[1]{\\mathsf{#1}}\n");
        preamble.push_str("\\newcommand{\\oxdef}[1]{\\mathsf{#1}}\n");
        preamble.push_str("\\newcommand{\\oxvar}[1]{#1}\n");
        preamble.push_str("\\newcommand{\\oxlit}[1]{\\mathtt{#1}}\n");
        preamble.push('\n');
        if !self.config.custom_preamble.is_empty() {
            preamble.push_str(&self.config.custom_preamble);
            preamble.push('\n');
        }
        preamble
    }
    /// Generate the full LaTeX document as a string.
    pub fn generate(&self) -> String {
        let mut output = String::new();
        if self.config.full_document {
            output.push_str(&self.generate_preamble());
            output.push('\n');
            if let Some(ref title) = self.config.title {
                output.push_str(&format!("\\title{{{}}}\n", escape_latex(title)));
            }
            if let Some(ref author) = self.config.author {
                output.push_str(&format!("\\author{{{}}}\n", escape_latex(author)));
            }
            output.push_str("\\date{\\today}\n");
            output.push('\n');
            output.push_str("\\begin{document}\n");
            if self.config.title.is_some() {
                output.push_str("\\maketitle\n");
            }
            output.push('\n');
        }
        for element in &self.elements {
            output.push_str(&self.render_element(element));
            output.push('\n');
        }
        if self.config.full_document {
            output.push_str("\n\\end{document}\n");
        }
        output
    }
    /// Render a single element.
    fn render_element(&self, element: &LatexElement) -> String {
        match element {
            LatexElement::Raw(text) => text.clone(),
            LatexElement::Section(title, level) => {
                let cmd = match level {
                    0 => "part",
                    1 => "section",
                    2 => "subsection",
                    3 => "subsubsection",
                    4 => "paragraph",
                    _ => "subparagraph",
                };
                format!("\\{}{{{}}}\n", cmd, escape_latex(title))
            }
            LatexElement::TheoremEnv(thm) => {
                let mut s = String::new();
                if let Some(ref title) = thm.title {
                    s.push_str(&format!(
                        "\\begin{{{}}}[{}]\n",
                        thm.env_name,
                        escape_latex(title)
                    ));
                } else {
                    s.push_str(&format!("\\begin{{{}}}\n", thm.env_name));
                }
                if let Some(ref label) = thm.label {
                    s.push_str(&format!("\\label{{{}}}\n", label));
                }
                s.push_str(&format!("${}$\n", thm.statement));
                s.push_str(&format!("\\end{{{}}}\n", thm.env_name));
                s
            }
            LatexElement::ProofEnv(proof) => {
                let mut s = String::new();
                s.push_str("\\begin{proof}\n");
                for step in &proof.steps {
                    s.push_str(step);
                    s.push('\n');
                }
                if let Some(ref qed) = proof.qed_symbol {
                    s.push_str(&format!("\\renewcommand{{\\qedsymbol}}{{{}}}\n", qed));
                }
                s.push_str("\\end{proof}\n");
                s
            }
            LatexElement::MathDef(def) => {
                let mut s = String::new();
                s.push_str("\\begin{definition}");
                s.push('\n');
                if let Some(ref label) = def.label {
                    s.push_str(&format!("\\label{{{}}}\n", label));
                }
                s.push_str(&format!(
                    "We define ${}",
                    self.expr_converter.convert_name(&def.name)
                ));
                if !def.params.is_empty() {
                    let params: Vec<String> = def
                        .params
                        .iter()
                        .map(|(n, t)| {
                            format!(
                                "{} : {}",
                                self.expr_converter.convert_name(n),
                                self.expr_converter.convert_type(t)
                            )
                        })
                        .collect();
                    s.push_str(&format!("({})", params.join(", ")));
                }
                s.push_str(&format!(
                    " : {}$",
                    self.expr_converter.convert_type(&def.ty)
                ));
                if let Some(ref body) = def.body {
                    s.push_str(&format!(" as ${}$", body));
                }
                s.push_str(".\n");
                s.push_str("\\end{definition}\n");
                s
            }
            LatexElement::Equation(eq) => {
                let mut s = String::new();
                if eq.aligned && eq.equations.len() > 1 {
                    let env = if eq.numbered { "align" } else { "align*" };
                    s.push_str(&format!("\\begin{{{}}}\n", env));
                    for (i, eq_str) in eq.equations.iter().enumerate() {
                        s.push_str(eq_str);
                        if i < eq.equations.len() - 1 {
                            s.push_str(" \\\\\n");
                        } else {
                            s.push('\n');
                        }
                    }
                    s.push_str(&format!("\\end{{{}}}\n", env));
                } else {
                    let env = if eq.numbered { "equation" } else { "equation*" };
                    for eq_str in &eq.equations {
                        s.push_str(&format!(
                            "\\begin{{{}}}\n{}\n\\end{{{}}}\n",
                            env, eq_str, env
                        ));
                    }
                }
                s
            }
            LatexElement::ItemList(items) => {
                let mut s = String::new();
                s.push_str("\\begin{itemize}\n");
                for item in items {
                    s.push_str(&format!("  \\item {}\n", item));
                }
                s.push_str("\\end{itemize}\n");
                s
            }
            LatexElement::Comment(text) => format!("% {}\n", text),
            LatexElement::Label(label) => format!("\\label{{{}}}\n", label),
            LatexElement::Ref(label) => format!("\\ref{{{}}}", label),
            LatexElement::BlankLine => "\n".to_string(),
        }
    }
    /// Write the document to a writer.
    pub fn write_to<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(self.generate().as_bytes())
    }
}
/// Math display mode.
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MathDisplayMode {
    /// Inline math: $...$
    Inline,
    /// Display math: \[...\]
    Display,
    /// Equation environment
    Equation,
    /// Align environment
    Align,
    /// Gather environment
    Gather,
    /// Multline environment
    Multline,
}
impl MathDisplayMode {
    /// Return the LaTeX opening delimiter.
    #[allow(dead_code)]
    pub fn open(&self) -> &'static str {
        match self {
            MathDisplayMode::Inline => "$",
            MathDisplayMode::Display => "\\[",
            MathDisplayMode::Equation => "\\begin{equation}",
            MathDisplayMode::Align => "\\begin{align*}",
            MathDisplayMode::Gather => "\\begin{gather}",
            MathDisplayMode::Multline => "\\begin{multline}",
        }
    }
    /// Return the LaTeX closing delimiter.
    #[allow(dead_code)]
    pub fn close(&self) -> &'static str {
        match self {
            MathDisplayMode::Inline => "$",
            MathDisplayMode::Display => "\\]",
            MathDisplayMode::Equation => "\\end{equation}",
            MathDisplayMode::Align => "\\end{align*}",
            MathDisplayMode::Gather => "\\end{gather}",
            MathDisplayMode::Multline => "\\end{multline}",
        }
    }
    /// Wrap content in this math mode.
    #[allow(dead_code)]
    pub fn wrap(&self, content: &str) -> String {
        match self {
            MathDisplayMode::Inline => format!("{}{}{}", self.open(), content, self.close()),
            _ => format!("{}\n{}\n{}", self.open(), content, self.close()),
        }
    }
}
