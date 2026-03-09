//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{
    ExprToLatex, LatexBibEntry, LatexConfig, LatexDocument, LatexDocumentBuilder, LatexExportStats,
    LatexMacro, LatexMacroLib, LatexMacroLibrary, LatexProofExporter, LatexProofNode, LatexSection,
    LatexSectionBlock, LatexSymbolTable, LatexTable, LatexTheoremEnv, MathDisplayMode,
    ProofExportStyle, ProofStep, ProofTreeNode, SectionLevel, TikzDiagram, TikzEdge, TikzNode,
    TikzShape,
};
use oxilean_kernel::Name;

/// Escape special LaTeX characters.
pub fn escape_latex(s: &str) -> String {
    let mut result = String::new();
    for ch in s.chars() {
        match ch {
            '#' | '$' | '%' | '&' | '{' | '}' | '~' | '^' => {
                result.push('\\');
                result.push(ch);
            }
            '_' => result.push_str("\\_"),
            '\\' => result.push_str("\\textbackslash{}"),
            _ => result.push(ch),
        }
    }
    result
}
/// Export a definition to LaTeX.
pub fn export_definition(
    name: &str,
    params: &[(String, String)],
    ty: &str,
    body: Option<&str>,
) -> String {
    let converter = ExprToLatex::new();
    let mut s = String::new();
    s.push_str("\\begin{definition}\n");
    s.push_str(&format!("${}$", converter.convert_name(name)));
    if !params.is_empty() {
        let ps: Vec<String> = params
            .iter()
            .map(|(n, t)| {
                format!(
                    "{} : {}",
                    converter.convert_name(n),
                    converter.convert_type(t)
                )
            })
            .collect();
        s.push_str(&format!("$({})", ps.join(", ")));
    }
    s.push_str(&format!(" : ${}$", converter.convert_type(ty)));
    if let Some(body_str) = body {
        s.push_str(&format!(" $:= {}$", body_str));
    }
    s.push_str("\n\\end{definition}\n");
    s
}
/// Export a theorem to LaTeX.
pub fn export_theorem(name: &str, statement: &str, proof_steps: &[String]) -> String {
    let mut s = String::new();
    s.push_str(&format!("\\begin{{theorem}}[{}]\n", escape_latex(name)));
    s.push_str(&format!("${}$\n", statement));
    s.push_str("\\end{theorem}\n");
    if !proof_steps.is_empty() {
        s.push_str("\\begin{proof}\n");
        for step in proof_steps {
            s.push_str(step);
            s.push('\n');
        }
        s.push_str("\\end{proof}\n");
    }
    s
}
/// Quick export: convert a simple statement to math mode.
pub fn to_math(text: &str) -> String {
    let converter = ExprToLatex::new();
    format!("${}$", converter.convert_name(text))
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_escape_latex() {
        assert_eq!(escape_latex("a_b"), "a\\_b");
        assert_eq!(escape_latex("100%"), "100\\%");
        assert_eq!(escape_latex("a{b}"), "a\\{b\\}");
    }
    #[test]
    fn test_symbol_conversion() {
        let converter = ExprToLatex::new();
        assert_eq!(converter.convert_name("Nat"), "\\mathbb{N}");
        assert_eq!(converter.convert_name("Prop"), "\\mathrm{Prop}");
        assert_eq!(converter.convert_name("x"), "x");
    }
    #[test]
    fn test_type_conversion() {
        let converter = ExprToLatex::new();
        assert_eq!(
            converter.convert_type("Nat -> Nat"),
            "\\mathbb{N} \\to \\mathbb{N}"
        );
    }
    #[test]
    fn test_document_generation() {
        let config = LatexConfig {
            title: Some("Test Document".to_string()),
            full_document: true,
            ..Default::default()
        };
        let doc = LatexDocument::new(config);
        let output = doc.generate();
        assert!(output.contains("\\documentclass"));
        assert!(output.contains("\\begin{document}"));
        assert!(output.contains("\\end{document}"));
        assert!(output.contains("Test Document"));
    }
    #[test]
    fn test_theorem_env() {
        let mut doc = LatexDocument::default_doc();
        doc.add_theorem(
            Some("Fermat".to_string()),
            "\\forall n > 2,\\, x^n + y^n \\neq z^n".to_string(),
            Some("thm:fermat".to_string()),
        );
        let output = doc.generate();
        assert!(output.contains("\\begin{theorem}[Fermat]"));
        assert!(output.contains("\\label{thm:fermat}"));
    }
    #[test]
    fn test_minimal_config() {
        let config = LatexConfig::minimal();
        let doc = LatexDocument::new(config);
        let output = doc.generate();
        assert!(!output.contains("\\documentclass"));
    }
}
/// Wrap content in inline math mode.
pub fn inline_math(content: &str) -> String {
    format!("${}$", content)
}
/// Wrap content in display math mode.
pub fn display_math(content: &str) -> String {
    format!("\\[\n{}\n\\]", content)
}
/// Wrap in an equation environment with label.
pub fn equation_labeled(content: &str, label: &str) -> String {
    format!(
        "\\begin{{equation}}\n\\label{{{}}}\n{}\n\\end{{equation}}",
        label, content
    )
}
/// Format a fraction.
pub fn latex_frac(num: &str, den: &str) -> String {
    format!("\\frac{{{}}}{{{}}}", num, den)
}
/// Format a square root.
pub fn latex_sqrt(content: &str) -> String {
    format!("\\sqrt{{{}}}", content)
}
/// Format an absolute value.
pub fn latex_abs(content: &str) -> String {
    format!("\\left|{}\\right|", content)
}
/// Format a set builder notation.
pub fn latex_set_builder(var: &str, ty: &str, pred: &str) -> String {
    format!(
        "\\left\\{{\\, {} : {} \\mid {} \\,\\right\\}}",
        var, ty, pred
    )
}
/// Format a sum with bounds.
pub fn latex_sum(idx: &str, lo: &str, hi: &str, body: &str) -> String {
    format!("\\sum_{{{}={}}}^{{{}}} {}", idx, lo, hi, body)
}
/// Format a product with bounds.
pub fn latex_prod(idx: &str, lo: &str, hi: &str, body: &str) -> String {
    format!("\\prod_{{{}={}}}^{{{}}} {}", idx, lo, hi, body)
}
/// Format a definite integral.
pub fn latex_integral(lo: &str, hi: &str, body: &str, var: &str) -> String {
    format!("\\int_{{{}}}^{{{}}} {} \\, d{}", lo, hi, body, var)
}
#[cfg(test)]
mod extended_tests {
    use super::*;
    #[test]
    fn test_proof_tree_leaf() {
        let node = ProofTreeNode::leaf("A \\land B");
        assert_eq!(node.size(), 1);
        assert_eq!(node.depth(), 1);
    }
    #[test]
    fn test_proof_tree_size_depth() {
        let leaf1 = ProofTreeNode::leaf("P");
        let leaf2 = ProofTreeNode::leaf("Q");
        let root = ProofTreeNode::node("P \\land Q", "\\land I", vec![leaf1, leaf2]);
        assert_eq!(root.size(), 3);
        assert_eq!(root.depth(), 2);
    }
    #[test]
    fn test_proof_tree_render_bussproofs() {
        let node = ProofTreeNode::leaf("\\top");
        let out = node.render_bussproofs();
        assert!(out.contains("\\AxiomC"));
    }
    #[test]
    fn test_proof_tree_unary() {
        let leaf = ProofTreeNode::leaf("P");
        let root = ProofTreeNode::node("\\lnot\\lnot P", "\\lnot\\lnot I", vec![leaf]);
        let out = root.render_bussproofs();
        assert!(out.contains("\\UnaryInfC"));
    }
    #[test]
    fn test_proof_tree_binary() {
        let l = ProofTreeNode::leaf("A");
        let r = ProofTreeNode::leaf("B");
        let root = ProofTreeNode::node("A \\land B", "\\land I", vec![l, r]);
        let out = root.render_bussproofs();
        assert!(out.contains("\\BinaryInfC"));
    }
    #[test]
    fn test_latex_table_render() {
        let mut t = LatexTable::new("ll");
        t.add_header(vec!["Name", "Type"]);
        t.add_row(vec!["x".to_string(), "Nat".to_string()]);
        let out = t.render();
        assert!(out.contains("\\begin{tabular}"));
        assert!(out.contains("\\toprule"));
        assert!(out.contains("\\textbf{Name}"));
    }
    #[test]
    fn test_latex_table_with_caption() {
        let mut t = LatexTable::new("c");
        t.set_caption("My Table");
        t.set_label("tab:my");
        let out = t.render();
        assert!(out.contains("My Table"));
        assert!(out.contains("tab:my"));
    }
    #[test]
    fn test_inline_math() {
        let s = inline_math("x^2 + y^2");
        assert_eq!(s, "$x^2 + y^2$");
    }
    #[test]
    fn test_display_math() {
        let s = display_math("E = mc^2");
        assert!(s.contains("\\[") && s.contains("\\]"));
    }
    #[test]
    fn test_latex_frac() {
        assert_eq!(latex_frac("1", "2"), "\\frac{1}{2}");
    }
    #[test]
    fn test_latex_sqrt() {
        assert_eq!(latex_sqrt("x"), "\\sqrt{x}");
    }
    #[test]
    fn test_latex_abs() {
        let s = latex_abs("x - y");
        assert!(s.contains("\\left|") && s.contains("\\right|"));
    }
    #[test]
    fn test_latex_sum() {
        let s = latex_sum("i", "0", "n", "a_i");
        assert!(s.contains("\\sum") && s.contains("a_i"));
    }
    #[test]
    fn test_latex_prod() {
        let s = latex_prod("k", "1", "n", "k");
        assert!(s.contains("\\prod"));
    }
    #[test]
    fn test_latex_integral() {
        let s = latex_integral("0", "\\infty", "f(x)", "x");
        assert!(s.contains("\\int"));
    }
    #[test]
    fn test_macro_library_oxilean_standard() {
        let lib = LatexMacroLib::oxilean_standard();
        assert!(!lib.is_empty());
        let preamble = lib.render_preamble();
        assert!(preamble.contains("\\newcommand"));
    }
    #[test]
    fn test_macro_library_add_and_render() {
        let mut lib = LatexMacroLib::new();
        lib.add_macro("myCmd", 1, "\\textbf{#1}");
        assert_eq!(lib.len(), 1);
        let p = lib.render_preamble();
        assert!(p.contains("myCmd"));
    }
    #[test]
    fn test_latex_section_render() {
        let mut sec = LatexSectionBlock::new("Introduction", 1);
        sec.add_text("Some introductory text.");
        sec.add_theorem("\\forall n, n + 0 = n", Some("By induction."));
        let out = sec.render();
        assert!(out.contains("\\section{Introduction}"));
        assert!(out.contains("\\begin{theorem}"));
        assert_eq!(sec.theorem_count, 1);
        assert_eq!(sec.block_count(), 2);
    }
    #[test]
    fn test_latex_section_subsection() {
        let sec = LatexSectionBlock::new("Details", 2);
        let out = sec.render();
        assert!(out.contains("\\subsection{Details}"));
    }
    #[test]
    fn test_equation_labeled() {
        let s = equation_labeled("E = mc^2", "eq:einstein");
        assert!(s.contains("\\begin{equation}") && s.contains("eq:einstein"));
    }
    #[test]
    fn test_set_builder() {
        let s = latex_set_builder("x", "\\mathbb{N}", "x > 0");
        assert!(s.contains("\\mid"));
    }
    #[test]
    fn test_export_definition_fn() {
        let params = vec![("n".to_string(), "Nat".to_string())];
        let out = export_definition("succ", &params, "Nat", None);
        assert!(out.contains("\\begin{definition}"));
    }
    #[test]
    fn test_export_theorem_fn() {
        let steps = vec!["By refl.".to_string()];
        let out = export_theorem("test_thm", "P \\land Q", &steps);
        assert!(out.contains("\\begin{theorem}"));
        assert!(out.contains("\\begin{proof}"));
    }
    #[test]
    fn test_to_math_fn() {
        let s = to_math("Nat");
        assert!(s.starts_with('$') && s.ends_with('$'));
    }
    #[test]
    fn test_latex_config_add_macro() {
        let mut cfg = LatexConfig::default();
        cfg.add_macro("myMacro".to_string(), "\\alpha".to_string());
        assert!(cfg.macros.contains_key("myMacro"));
    }
    #[test]
    fn test_latex_config_add_package() {
        let mut cfg = LatexConfig::default();
        let prev_len = cfg.packages.len();
        cfg.add_package("tikz".to_string());
        assert_eq!(cfg.packages.len(), prev_len + 1);
        cfg.add_package("tikz".to_string());
        assert_eq!(cfg.packages.len(), prev_len + 1);
    }
}
#[cfg(test)]
mod latex_extra_tests {
    use super::*;
    #[test]
    fn test_math_display_mode_wrap() {
        let mode = MathDisplayMode::Inline;
        let wrapped = mode.wrap("x + y = z");
        assert!(wrapped.contains("$x + y = z$"));
    }
    #[test]
    fn test_math_display_equation() {
        let mode = MathDisplayMode::Equation;
        let wrapped = mode.wrap("E = mc^2");
        assert!(wrapped.contains("\\begin{equation}"));
        assert!(wrapped.contains("\\end{equation}"));
    }
    #[test]
    fn test_symbol_table_translate() {
        let table = LatexSymbolTable::new();
        assert_eq!(table.translate("forall"), "\\forall");
        assert_eq!(table.translate("->"), "\\to");
        assert_eq!(table.translate("unknown_token"), "unknown_token");
    }
    #[test]
    fn test_symbol_table_custom() {
        let mut table = LatexSymbolTable::new();
        table.add("my_sym", "\\mysymbol");
        assert_eq!(table.translate("my_sym"), "\\mysymbol");
    }
    #[test]
    fn test_proof_tree_leaf() {
        let leaf = LatexProofNode::leaf("A");
        let rendered = leaf.render_bussproofs();
        assert!(rendered.contains("\\AxiomC"));
        assert!(rendered.contains("$A$"));
    }
    #[test]
    fn test_proof_tree_binary() {
        let p1 = LatexProofNode::leaf("A");
        let p2 = LatexProofNode::leaf("B");
        let root = LatexProofNode::node("A /\\ B", vec![p1, p2], Some("And-I".to_string()));
        let rendered = root.render_bussproofs();
        assert!(rendered.contains("\\BinaryInfC"));
        assert!(rendered.contains("And-I"));
    }
    #[test]
    fn test_proof_tree_text_render() {
        let leaf = LatexProofNode::leaf("P");
        let rendered = leaf.render_text(0);
        assert!(rendered.contains("P"));
    }
    #[test]
    fn test_document_builder() {
        let doc = LatexDocumentBuilder::new("article")
            .add_package("amsmath")
            .add_package("amssymb")
            .with_title("My Proof")
            .with_author("Author")
            .add_section(LatexSection {
                level: SectionLevel::Section,
                title: "Introduction".to_string(),
                content: "This paper presents...".to_string(),
                label: Some("sec:intro".to_string()),
            })
            .build();
        assert!(doc.contains("\\documentclass{article}"));
        assert!(doc.contains("\\usepackage{amsmath}"));
        assert!(doc.contains("\\title{My Proof}"));
        assert!(doc.contains("\\section{Introduction}"));
        assert!(doc.contains("\\label{sec:intro}"));
    }
    #[test]
    fn test_theorem_env_definition() {
        let env = LatexTheoremEnv::new("theorem", "Theorem", None);
        let def = env.definition();
        assert!(def.contains("\\newtheorem{theorem}{Theorem}"));
    }
    #[test]
    fn test_theorem_env_render() {
        let env = LatexTheoremEnv::new("theorem", "Theorem", None);
        let rendered = env.render(
            Some("Pythagorean"),
            Some("thm:pyth"),
            "$a^2 + b^2 = c^2$",
            Some("By construction."),
        );
        assert!(rendered.contains("\\begin{theorem}"));
        assert!(rendered.contains("\\label{thm:pyth}"));
        assert!(rendered.contains("\\begin{proof}"));
    }
    #[test]
    fn test_standard_environments() {
        let envs = LatexTheoremEnv::standard_environments();
        assert_eq!(envs.len(), 7);
        assert!(envs.iter().any(|e| e.env_name == "theorem"));
        assert!(envs.iter().any(|e| e.env_name == "lemma"));
    }
    #[test]
    fn test_export_stats() {
        let stats = LatexExportStats {
            definitions_exported: 5,
            theorems_exported: 10,
            proofs_exported: 8,
            axioms_exported: 2,
            total_lines: 500,
        };
        let summary = stats.summary();
        assert!(summary.contains("10 theorems"));
        assert!(summary.contains("500 lines"));
    }
    #[test]
    fn test_section_level_commands() {
        assert_eq!(SectionLevel::Section.command(), "section");
        assert_eq!(SectionLevel::Subsection.command(), "subsection");
        assert_eq!(SectionLevel::Chapter.command(), "chapter");
    }
}
/// Return the latex_export module version.
#[allow(dead_code)]
pub fn latex_export_version() -> &'static str {
    "0.1.1"
}
#[cfg(test)]
mod latex_extra2_tests {
    use super::*;
    #[test]
    fn test_macro_to_latex_no_args() {
        let m = LatexMacro::new("myname", 0, "\\texttt{OxiLean}", "name");
        let s = m.to_latex();
        assert!(s.contains("\\newcommand{\\myname}"));
        assert!(!s.contains("[0]"));
    }
    #[test]
    fn test_macro_to_latex_with_args() {
        let m = LatexMacro::new("typedef", 2, "#1 : #2", "typedef");
        let s = m.to_latex();
        assert!(s.contains("[2]"));
    }
    #[test]
    fn test_macro_library_find() {
        let lib = LatexMacroLibrary::default_oxilean_macros();
        let found = lib.find("lean");
        assert!(found.is_some());
        let notfound = lib.find("nonexistent");
        assert!(notfound.is_none());
    }
    #[test]
    fn test_macro_library_preamble() {
        let lib = LatexMacroLibrary::default_oxilean_macros();
        let preamble = lib.render_preamble();
        assert!(preamble.contains("\\newcommand"));
    }
    #[test]
    fn test_proof_step_assumption() {
        let step = ProofStep::assumption("s1", "P is true");
        assert_eq!(step.id, "s1");
        assert_eq!(step.justification, Some("Assumption".to_string()));
    }
    #[test]
    fn test_proof_step_derived() {
        let step = ProofStep::derived("s2", "Q", "Modus ponens", vec!["s1".to_string()]);
        assert_eq!(step.premise_ids.len(), 1);
    }
    #[test]
    fn test_proof_exporter_prose() {
        let exporter = LatexProofExporter::new(ProofExportStyle::Prose);
        let steps = vec![
            ProofStep::assumption("s1", "A"),
            ProofStep::assumption("s2", "B"),
        ];
        let out = exporter.export_steps(&steps);
        assert!(out.contains("\\begin{proof}"));
        assert!(out.contains("We first observe"));
    }
    #[test]
    fn test_proof_exporter_itemized() {
        let exporter = LatexProofExporter::new(ProofExportStyle::Itemized);
        let steps = vec![ProofStep::assumption("s1", "P")];
        let out = exporter.export_steps(&steps);
        assert!(out.contains("\\begin{enumerate}"));
        assert!(out.contains("\\item"));
    }
    #[test]
    fn test_bib_entry() {
        let entry = LatexBibEntry::new("pierce2002", "book")
            .with_field("title", "Types and Programming Languages")
            .with_field("author", "Pierce, Benjamin C.")
            .with_field("year", "2002");
        let bib = entry.to_bibtex();
        assert!(bib.contains("@book{pierce2002,"));
        assert!(bib.contains("title = {Types and Programming Languages}"));
    }
    #[test]
    fn test_latex_export_version() {
        assert!(!latex_export_version().is_empty());
    }
}
#[cfg(test)]
mod tikz_tests {
    use super::*;
    #[test]
    fn test_tikz_diagram() {
        let mut diagram = TikzDiagram::new();
        diagram.add_node(TikzNode {
            id: "a".to_string(),
            label: "A".to_string(),
            x: 0.0,
            y: 0.0,
            shape: TikzShape::Circle,
        });
        diagram.add_node(TikzNode {
            id: "b".to_string(),
            label: "B".to_string(),
            x: 2.0,
            y: 0.0,
            shape: TikzShape::Rectangle,
        });
        diagram.add_edge(TikzEdge {
            from: "a".to_string(),
            to: "b".to_string(),
            label: Some("f".to_string()),
            style: "->".to_string(),
        });
        let rendered = diagram.render();
        assert!(rendered.contains("\\begin{tikzpicture}"));
        assert!(rendered.contains("circle,draw"));
        assert!(rendered.contains("node {f}"));
    }
}
