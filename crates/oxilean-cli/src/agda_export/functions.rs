//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{
    AgdaClause, AgdaExportConfig, AgdaExportStats, AgdaExporter, AgdaFunctionDef, AgdaImportDecl,
    AgdaModuleConfig, AgdaPragma, AgdaProofTerm, AgdaRecord, ArgSpec, Constructor, ConversionStats,
    CoqExportConfig, CoqExporter, InductiveDecl, MultiFileExport, NameCache, OxiDecl, OxiDeclKind,
    RecordDecl, RecordField,
};

/// Sanitize an OxiLean name for use as an Agda identifier.
///
/// Agda identifiers may not start with digits and must avoid reserved words.
/// Dots in qualified names are preserved (Agda uses modules).
pub fn sanitize_agda_name(name: &str) -> String {
    const AGDA_RESERVED: &[&str] = &[
        "abstract",
        "codata",
        "coinductive",
        "constructor",
        "data",
        "do",
        "eta-equality",
        "field",
        "forall",
        "hiding",
        "import",
        "in",
        "inductive",
        "infix",
        "infixl",
        "infixr",
        "instance",
        "interleaved",
        "let",
        "macro",
        "module",
        "mutual",
        "no-eta-equality",
        "open",
        "overlap",
        "pattern",
        "postulate",
        "primitive",
        "private",
        "quote",
        "quoteTerm",
        "record",
        "renaming",
        "rewrite",
        "Set",
        "syntax",
        "tactic",
        "unquote",
        "unquoteDecl",
        "unquoteDef",
        "using",
        "variable",
        "where",
        "with",
    ];
    let sanitized: String = name
        .chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '_' || c == '\'' || c == '.' {
                c
            } else {
                '_'
            }
        })
        .collect();
    let sanitized = if sanitized.starts_with(|c: char| c.is_ascii_digit()) {
        format!("x_{}", sanitized)
    } else {
        sanitized
    };
    if AGDA_RESERVED.contains(&sanitized.as_str()) {
        format!("{}_", sanitized)
    } else {
        sanitized
    }
}
/// Sanitize an OxiLean name for use as a Coq identifier.
pub fn sanitize_coq_name(name: &str) -> String {
    const COQ_RESERVED: &[&str] = &[
        "Axiom",
        "CoFixpoint",
        "CoInductive",
        "Definition",
        "End",
        "Example",
        "Fixpoint",
        "Hypothesis",
        "Inductive",
        "Instance",
        "Lemma",
        "Module",
        "Notation",
        "Parameter",
        "Print",
        "Proof",
        "Prop",
        "Record",
        "Require",
        "Section",
        "Set",
        "SProp",
        "Theorem",
        "Type",
        "Variable",
        "forall",
        "fun",
        "let",
        "in",
        "match",
        "with",
        "end",
        "as",
        "return",
        "if",
        "then",
        "else",
        "fix",
        "cofix",
        "struct",
    ];
    let sanitized: String = name
        .chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '_' || c == '\'' {
                c
            } else {
                '_'
            }
        })
        .collect();
    let sanitized = if sanitized.starts_with(|c: char| c.is_ascii_digit()) {
        format!("x_{}", sanitized)
    } else {
        sanitized
    };
    if COQ_RESERVED.contains(&sanitized.as_str()) {
        format!("{}_", sanitized)
    } else {
        sanitized
    }
}
/// Translate an OxiLean universe level string to an Agda level expression.
///
/// - `"0"` → `"Set"`
/// - `"1"` → `"Set₁"`
/// - `"u"` → `"Set u"`
/// - `"u + 1"` → `"Set (lsuc u)"`
/// - `"max u v"` → `"Set (u ⊔ v)"`
pub fn oxilean_level_to_agda(level_str: &str) -> String {
    let s = level_str.trim();
    if s == "0" {
        return "Set".to_string();
    }
    if let Ok(n) = s.parse::<u32>() {
        let subscripts = ["₀", "₁", "₂", "₃", "₄", "₅", "₆", "₇", "₈", "₉"];
        if n < 10 {
            return format!("Set{}", subscripts[n as usize]);
        }
        return format!("Set{}", n);
    }
    if s.contains("max") {
        let inner = s.trim_start_matches("max").trim();
        let parts: Vec<&str> = inner.splitn(2, ' ').collect();
        if parts.len() == 2 {
            return format!("Set ({} ⊔ {})", parts[0], parts[1]);
        }
    }
    if s.contains('+') {
        let parts: Vec<&str> = s.splitn(2, '+').collect();
        if parts.len() == 2 {
            let base = parts[0].trim();
            let inc = parts[1].trim();
            if inc == "1" {
                return format!("Set (lsuc {})", base);
            }
        }
    }
    format!("Set {}", s)
}
/// Translate an OxiLean universe level string to a Coq Sort expression.
pub fn oxilean_level_to_coq(level_str: &str) -> String {
    let s = level_str.trim();
    if s == "0" {
        return "Prop".to_string();
    }
    if let Ok(_n) = s.parse::<u32>() {
        return "Type".to_string();
    }
    "Type".to_string()
}
/// Translate a simple OxiLean type string to an Agda type string.
///
/// Handles common primitives and arrow types.
pub fn translate_type_to_agda(ty: &str) -> String {
    let ty = ty.trim();
    match ty {
        "Nat" => return "ℕ".to_string(),
        "Int" => return "ℤ".to_string(),
        "String" => return "String".to_string(),
        "Bool" => return "Bool".to_string(),
        "Char" => return "Char".to_string(),
        "Float" => return "Float".to_string(),
        "Unit" | "()" => return "⊤".to_string(),
        "Empty" | "False" => return "⊥".to_string(),
        "Prop" => return "Set".to_string(),
        "Type" => return "Set₁".to_string(),
        _ => {}
    }
    if let Some(idx) = find_arrow(ty) {
        let lhs = ty[..idx].trim();
        let rhs = ty[idx + 2..].trim();
        return format!(
            "{} → {}",
            translate_type_to_agda(lhs),
            translate_type_to_agda(rhs)
        );
    }
    if let Some(inner) = ty.strip_prefix("List ") {
        return format!("List {}", translate_type_to_agda(inner.trim()));
    }
    if let Some(inner) = ty.strip_prefix("Option ") {
        return format!("Maybe {}", translate_type_to_agda(inner.trim()));
    }
    if ty.find(" × ").or_else(|| ty.find(" * ")).is_some() {
        let (lhs, rhs) = if let Some(i) = ty.find(" × ") {
            (&ty[..i], &ty[i + 3..])
        } else {
            let i = ty
                .find(" * ")
                .expect("find(' * ') is Some because or_else branch was taken");
            (&ty[..i], &ty[i + 3..])
        };
        return format!(
            "{} × {}",
            translate_type_to_agda(lhs.trim()),
            translate_type_to_agda(rhs.trim())
        );
    }
    sanitize_agda_name(ty)
}
/// Find the position of a top-level `->` in a type string (not inside parens).
fn find_arrow(s: &str) -> Option<usize> {
    let bytes = s.as_bytes();
    let mut depth = 0i32;
    let mut i = 0;
    while i + 1 < bytes.len() {
        match bytes[i] {
            b'(' | b'[' | b'{' => depth += 1,
            b')' | b']' | b'}' => depth -= 1,
            b'-' if depth == 0 && bytes[i + 1] == b'>' => return Some(i),
            _ => {}
        }
        i += 1;
    }
    None
}
/// Translate a simple OxiLean type string to a Coq type string.
pub fn translate_type_to_coq(ty: &str) -> String {
    let ty = ty.trim();
    match ty {
        "Nat" => return "nat".to_string(),
        "Int" => return "Z".to_string(),
        "String" => return "string".to_string(),
        "Bool" => return "bool".to_string(),
        "Char" => return "Ascii.ascii".to_string(),
        "Unit" | "()" => return "unit".to_string(),
        "Empty" | "False" => return "False".to_string(),
        "Prop" => return "Prop".to_string(),
        "Type" => return "Type".to_string(),
        _ => {}
    }
    if let Some(idx) = find_arrow(ty) {
        let lhs = ty[..idx].trim();
        let rhs = ty[idx + 2..].trim();
        return format!(
            "{} -> {}",
            translate_type_to_coq(lhs),
            translate_type_to_coq(rhs)
        );
    }
    if let Some(inner) = ty.strip_prefix("List ") {
        return format!("list {}", translate_type_to_coq(inner.trim()));
    }
    if let Some(inner) = ty.strip_prefix("Option ") {
        return format!("option {}", translate_type_to_coq(inner.trim()));
    }
    sanitize_coq_name(ty)
}
/// Convert a batch of OxiLean declarations to Agda.
pub fn batch_convert_to_agda(
    decls: &[OxiDecl],
    config: AgdaExportConfig,
) -> (AgdaExporter, ConversionStats) {
    let mut exporter = AgdaExporter::new(config);
    let mut stats = ConversionStats::default();
    let mut cache = NameCache::new();
    for decl in decls {
        let agda_name = cache.unique_agda_name(&decl.name);
        let agda_ty = translate_type_to_agda(&decl.type_sig);
        match decl.kind {
            OxiDeclKind::Axiom => {
                exporter.add_postulate(&agda_name, &agda_ty);
                stats.record_success();
            }
            OxiDeclKind::Def | OxiDeclKind::Theorem => {
                if let Some(body) = &decl.body {
                    exporter.add_def(&agda_name, &agda_ty, body);
                    stats.record_success();
                } else {
                    exporter.add_postulate(&agda_name, &agda_ty);
                    stats.record_skip();
                }
            }
            OxiDeclKind::Inductive | OxiDeclKind::Structure => {
                exporter.add_postulate(&agda_name, &agda_ty);
                stats.record_skip();
            }
        }
    }
    (exporter, stats)
}
/// Convert a batch of OxiLean declarations to Coq.
pub fn batch_convert_to_coq(
    decls: &[OxiDecl],
    config: CoqExportConfig,
) -> (CoqExporter, ConversionStats) {
    let mut exporter = CoqExporter::new(config);
    let mut stats = ConversionStats::default();
    let mut cache = NameCache::new();
    for decl in decls {
        let coq_name = cache.unique_coq_name(&decl.name);
        let coq_ty = translate_type_to_coq(&decl.type_sig);
        match decl.kind {
            OxiDeclKind::Axiom => {
                exporter.add_axiom(&coq_name, &coq_ty);
                stats.record_success();
            }
            OxiDeclKind::Def => {
                if let Some(body) = &decl.body {
                    exporter.add_definition(&coq_name, &coq_ty, body);
                    stats.record_success();
                } else {
                    exporter.add_axiom(&coq_name, &coq_ty);
                    stats.record_skip();
                }
            }
            OxiDeclKind::Theorem => {
                if let Some(proof) = &decl.body {
                    exporter.add_lemma(&coq_name, &coq_ty, proof);
                    stats.record_success();
                } else {
                    exporter.add_axiom(&coq_name, &coq_ty);
                    stats.record_skip();
                }
            }
            OxiDeclKind::Inductive | OxiDeclKind::Structure => {
                exporter.add_axiom(&coq_name, &coq_ty);
                stats.record_skip();
            }
        }
    }
    (exporter, stats)
}
#[cfg(test)]
mod tests {
    use super::*;
    fn default_agda_config() -> AgdaExportConfig {
        AgdaExportConfig {
            module_name: "MyModule".to_string(),
            use_unicode: true,
            indent_size: 2,
            imports: Vec::new(),
            options: Vec::new(),
        }
    }
    #[test]
    fn agda_export_module_header() {
        let exp = AgdaExporter::new(default_agda_config());
        let src = exp.export();
        assert!(src.starts_with("module MyModule where"));
    }
    #[test]
    fn agda_export_postulate() {
        let mut exp = AgdaExporter::new(default_agda_config());
        exp.add_postulate("Nat", "Set");
        let src = exp.export();
        assert!(src.contains("postulate"));
        assert!(src.contains("Nat : Set"));
    }
    #[test]
    fn agda_export_multiple_postulates_grouped() {
        let mut exp = AgdaExporter::new(default_agda_config());
        exp.add_postulate("Nat", "Set");
        exp.add_postulate("zero", "Nat");
        let src = exp.export();
        assert_eq!(src.matches("postulate").count(), 1);
    }
    #[test]
    fn agda_export_def() {
        let mut exp = AgdaExporter::new(default_agda_config());
        exp.add_def("id", "∀ {A : Set} → A → A", "λ x → x");
        let src = exp.export();
        assert!(src.contains("id : ∀ {A : Set} → A → A"));
        assert!(src.contains("id = λ x → x"));
    }
    #[test]
    fn agda_export_to_file() {
        let mut exp = AgdaExporter::new(default_agda_config());
        exp.add_postulate("P", "Set");
        let tmp = std::env::temp_dir().join("oxilean_agda_test.agda");
        exp.export_to_file(&tmp).expect("write failed");
        let content = std::fs::read_to_string(&tmp).expect("read failed");
        assert!(content.contains("module MyModule where"));
        let _ = std::fs::remove_file(&tmp);
    }
    #[test]
    fn sanitize_agda_reserved() {
        let s = sanitize_agda_name("where");
        assert_eq!(s, "where_");
    }
    #[test]
    fn sanitize_agda_digit_start() {
        let s = sanitize_agda_name("123foo");
        assert!(s.starts_with("x_"));
    }
    #[test]
    fn sanitize_agda_valid() {
        assert_eq!(sanitize_agda_name("myFunc"), "myFunc");
    }
    #[test]
    fn sanitize_agda_special_chars() {
        let s = sanitize_agda_name("my-func");
        assert!(!s.contains('-'));
    }
    #[test]
    fn sanitize_coq_reserved() {
        let s = sanitize_coq_name("match");
        assert!(s.ends_with('_'));
    }
    #[test]
    fn sanitize_coq_valid() {
        assert_eq!(sanitize_coq_name("myLemma"), "myLemma");
    }
    #[test]
    fn level_to_agda_zero() {
        assert_eq!(oxilean_level_to_agda("0"), "Set");
    }
    #[test]
    fn level_to_agda_one() {
        let s = oxilean_level_to_agda("1");
        assert!(s.contains("Set"), "got: {}", s);
    }
    #[test]
    fn level_to_agda_variable() {
        let s = oxilean_level_to_agda("u");
        assert!(s.contains('u'));
    }
    #[test]
    fn level_to_agda_max() {
        let s = oxilean_level_to_agda("max u v");
        assert!(s.contains('u') && s.contains('v'));
    }
    #[test]
    fn level_to_agda_succ() {
        let s = oxilean_level_to_agda("u + 1");
        assert!(s.contains("lsuc") || s.contains('u'));
    }
    #[test]
    fn level_to_coq_zero() {
        assert_eq!(oxilean_level_to_coq("0"), "Prop");
    }
    #[test]
    fn level_to_coq_nonzero() {
        assert_eq!(oxilean_level_to_coq("1"), "Type");
    }
    #[test]
    fn type_to_agda_nat() {
        assert_eq!(translate_type_to_agda("Nat"), "ℕ");
    }
    #[test]
    fn type_to_agda_bool() {
        assert_eq!(translate_type_to_agda("Bool"), "Bool");
    }
    #[test]
    fn type_to_agda_arrow() {
        let s = translate_type_to_agda("Nat -> Bool");
        assert!(s.contains('→'));
    }
    #[test]
    fn type_to_agda_list() {
        let s = translate_type_to_agda("List Nat");
        assert!(s.contains("List") && s.contains('ℕ'));
    }
    #[test]
    fn type_to_agda_option() {
        let s = translate_type_to_agda("Option Bool");
        assert!(s.contains("Maybe") && s.contains("Bool"));
    }
    #[test]
    fn type_to_coq_nat() {
        assert_eq!(translate_type_to_coq("Nat"), "nat");
    }
    #[test]
    fn type_to_coq_arrow() {
        let s = translate_type_to_coq("Nat -> Bool");
        assert!(s.contains("->"));
    }
    #[test]
    fn type_to_coq_list() {
        let s = translate_type_to_coq("List Nat");
        assert!(s.contains("list") && s.contains("nat"));
    }
    #[test]
    fn inductive_decl_agda_render() {
        let mut decl = InductiveDecl::new("MyNat", "Set");
        decl.add_constructor(Constructor::new("zero", "MyNat"));
        decl.add_constructor(
            Constructor::new("succ", "MyNat").with_arg(ArgSpec::explicit("n", "MyNat")),
        );
        let s = decl.render_agda(2);
        assert!(s.contains("data MyNat"));
        assert!(s.contains("zero"));
        assert!(s.contains("succ"));
    }
    #[test]
    fn inductive_decl_coq_render() {
        let mut decl = InductiveDecl::new("MyBool", "Set");
        decl.add_constructor(Constructor::new("tt", "MyBool"));
        decl.add_constructor(Constructor::new("ff", "MyBool"));
        let s = decl.render_coq();
        assert!(s.contains("Inductive MyBool"));
        assert!(s.contains("tt"));
        assert!(s.contains("ff"));
    }
    #[test]
    fn record_decl_agda_render() {
        let mut rec = RecordDecl::new("Pair", "Set").with_constructor("mkPair");
        rec.add_field(RecordField::new("fst", "ℕ"));
        rec.add_field(RecordField::new("snd", "ℕ"));
        let s = rec.render_agda(2);
        assert!(s.contains("record Pair"));
        assert!(s.contains("constructor mkPair"));
        assert!(s.contains("fst"));
        assert!(s.contains("snd"));
    }
    #[test]
    fn agda_function_def_render() {
        let mut def = AgdaFunctionDef::new("add", "ℕ → ℕ → ℕ");
        def.add_clause(AgdaClause::new(
            vec!["zero".to_string(), "m".to_string()],
            "m",
        ));
        def.add_clause(AgdaClause::new(
            vec!["(suc n)".to_string(), "m".to_string()],
            "suc (add n m)",
        ));
        let s = def.render();
        assert!(s.contains("add : ℕ → ℕ → ℕ"));
        assert!(s.contains("add zero m = m"));
        assert!(s.contains("add (suc n) m = suc (add n m)"));
    }
    #[test]
    fn agda_pragma_render() {
        assert_eq!(AgdaPragma::Terminating.render(), "{-# TERMINATING #-}");
        assert_eq!(
            AgdaPragma::NonTerminating.render(),
            "{-# NON_TERMINATING #-}"
        );
        assert!(AgdaPragma::Builtin {
            name: "NATURAL".to_string(),
            value: "Nat".to_string()
        }
        .render()
        .contains("BUILTIN"));
        assert!(AgdaPragma::Options {
            args: vec!["--allow-unsolved-metas".to_string()]
        }
        .render()
        .contains("OPTIONS"));
    }
    #[test]
    fn arg_spec_render_agda() {
        assert_eq!(ArgSpec::explicit("x", "ℕ").render_agda(), "(x : ℕ)");
        assert_eq!(ArgSpec::implicit("A", "Set").render_agda(), "{A : Set}");
        assert_eq!(
            ArgSpec::instance("inst", "Show A").render_agda(),
            "⦃inst : Show A⦄"
        );
    }
    #[test]
    fn arg_spec_render_coq() {
        assert_eq!(ArgSpec::explicit("x", "nat").render_coq(), "(x : nat)");
        assert_eq!(ArgSpec::implicit("A", "Type").render_coq(), "{A : Type}");
    }
    #[test]
    fn name_cache_unique_names() {
        let mut cache = NameCache::new();
        let a = cache.unique_agda_name("foo");
        let b = cache.unique_agda_name("foo");
        assert_ne!(a, b);
        assert_eq!(a, "foo");
        assert!(b.starts_with("foo_"));
    }
    #[test]
    fn name_cache_coq_unique() {
        let mut cache = NameCache::new();
        cache.unique_coq_name("bar");
        let b = cache.unique_coq_name("bar");
        assert!(b.starts_with("bar_"));
    }
    #[test]
    fn conversion_stats_rates() {
        let mut stats = ConversionStats::default();
        stats.record_success();
        stats.record_success();
        stats.record_skip();
        stats.record_error("oops");
        assert_eq!(stats.total, 4);
        assert_eq!(stats.success, 2);
        assert!((stats.success_rate() - 0.5).abs() < 0.01);
    }
    #[test]
    fn conversion_stats_summary() {
        let mut stats = ConversionStats::default();
        stats.record_success();
        let s = stats.summary();
        assert!(s.contains("total=1"));
    }
    #[test]
    fn batch_convert_axioms() {
        let decls = vec![
            OxiDecl::axiom("myAxiom", "Nat"),
            OxiDecl::axiom("another", "Bool"),
        ];
        let (exp, stats) = batch_convert_to_agda(&decls, AgdaExportConfig::minimal("Test"));
        let src = exp.export();
        assert!(src.contains("postulate"));
        assert_eq!(stats.success, 2);
    }
    #[test]
    fn batch_convert_defs() {
        let decls = vec![OxiDecl::def("myDef", "Nat -> Nat", "fun x -> x")];
        let (exp, stats) = batch_convert_to_agda(&decls, AgdaExportConfig::minimal("Test"));
        let src = exp.export();
        assert!(src.contains("myDef"));
        assert_eq!(stats.success, 1);
    }
    #[test]
    fn batch_convert_coq_axioms() {
        let decls = vec![OxiDecl::axiom("P", "Prop")];
        let (exp, stats) = batch_convert_to_coq(&decls, CoqExportConfig::new("TestSection"));
        let src = exp.export();
        assert!(src.contains("Axiom P"));
        assert_eq!(stats.success, 1);
    }
    #[test]
    fn batch_convert_coq_theorems() {
        let decls = vec![OxiDecl::theorem("myThm", "Nat -> Nat", "intro n; exact n")];
        let (exp, stats) = batch_convert_to_coq(&decls, CoqExportConfig::new("S"));
        let src = exp.export();
        assert!(src.contains("Lemma myThm"));
        assert_eq!(stats.success, 1);
    }
    #[test]
    fn coq_export_header_and_footer() {
        let exp = CoqExporter::new(CoqExportConfig {
            module_name: "TestSec".to_string(),
            imports: Vec::new(),
        });
        let src = exp.export();
        assert!(src.contains("Section TestSec."));
        assert!(src.contains("End TestSec."));
        assert!(src.contains("Require Import"));
    }
    #[test]
    fn coq_export_axiom() {
        let mut exp = CoqExporter::new(CoqExportConfig {
            module_name: "S".to_string(),
            imports: Vec::new(),
        });
        exp.add_axiom("em", "forall P : Prop, P \\/ ~P");
        let src = exp.export();
        assert!(src.contains("Axiom em : forall P : Prop, P \\/ ~P."));
    }
    #[test]
    fn coq_export_definition() {
        let mut exp = CoqExporter::new(CoqExportConfig {
            module_name: "S".to_string(),
            imports: Vec::new(),
        });
        exp.add_definition("myId", "nat -> nat", "fun n => n");
        let src = exp.export();
        assert!(src.contains("Definition myId : nat -> nat := fun n => n."));
    }
    #[test]
    fn coq_export_order_preserved() {
        let mut exp = CoqExporter::new(CoqExportConfig {
            module_name: "S".to_string(),
            imports: Vec::new(),
        });
        exp.add_axiom("A", "Prop");
        exp.add_definition("f", "nat", "0");
        let src = exp.export();
        let axiom_pos = src.find("Axiom A").expect("find should succeed");
        let def_pos = src.find("Definition f").expect("find should succeed");
        assert!(axiom_pos < def_pos);
    }
    #[test]
    fn coq_export_lemma() {
        let mut exp = CoqExporter::new(CoqExportConfig::new("S"));
        exp.add_lemma("trivial_eq", "1 = 1", "reflexivity");
        let src = exp.export();
        assert!(src.contains("Lemma trivial_eq : 1 = 1."));
        assert!(src.contains("Proof."));
        assert!(src.contains("Qed."));
    }
    #[test]
    fn coq_export_notation() {
        let mut exp = CoqExporter::new(CoqExportConfig::new("S"));
        exp.add_notation("x + y", "Nat.add x y");
        let src = exp.export();
        assert!(src.contains("Notation"));
    }
    #[test]
    fn coq_export_comment() {
        let mut exp = CoqExporter::new(CoqExportConfig::new("S"));
        exp.add_comment("This is a comment");
        let src = exp.export();
        assert!(src.contains("(* This is a comment *)"));
    }
    #[test]
    fn coq_export_inductive() {
        let mut exp = CoqExporter::new(CoqExportConfig::new("S"));
        let mut decl = InductiveDecl::new("MyList", "Set -> Set");
        decl.add_constructor(Constructor::new("nil", "MyList A"));
        exp.add_inductive(decl);
        let src = exp.export();
        assert!(src.contains("Inductive MyList"));
    }
    #[test]
    fn agda_config_minimal() {
        let cfg = AgdaExportConfig::minimal("Test");
        assert_eq!(cfg.module_name, "Test");
        assert!(cfg.imports.is_empty());
        assert!(cfg.options.is_empty());
    }
    #[test]
    fn agda_config_with_import() {
        let cfg = AgdaExportConfig::minimal("Test").with_import("Data.Nat");
        assert!(cfg.imports.contains(&"Data.Nat".to_string()));
    }
    #[test]
    fn agda_config_with_option() {
        let cfg = AgdaExportConfig::minimal("Test").with_option("--safe");
        assert!(cfg.options.contains(&"--safe".to_string()));
    }
    #[test]
    fn agda_export_with_options() {
        let cfg = AgdaExportConfig::minimal("Test").with_option("--safe");
        let exp = AgdaExporter::new(cfg);
        let src = exp.export();
        assert!(src.contains("OPTIONS"));
        assert!(src.contains("--safe"));
    }
    #[test]
    fn agda_export_with_imports() {
        let cfg = AgdaExportConfig::minimal("Test").with_import("Data.Nat");
        let exp = AgdaExporter::new(cfg);
        let src = exp.export();
        assert!(src.contains("open import Data.Nat"));
    }
    #[test]
    fn agda_export_comment() {
        let mut exp = AgdaExporter::new(AgdaExportConfig::minimal("Test"));
        exp.add_comment("This is a comment");
        let src = exp.export();
        assert!(src.contains("-- This is a comment"));
    }
    #[test]
    fn agda_export_open() {
        let mut exp = AgdaExporter::new(AgdaExportConfig::minimal("Test"));
        exp.add_open("Data.Nat", None);
        let src = exp.export();
        assert!(src.contains("open import Data.Nat"));
    }
    #[test]
    fn agda_export_open_using() {
        let mut exp = AgdaExporter::new(AgdaExportConfig::minimal("Test"));
        exp.add_open("Data.Nat", Some(vec!["ℕ".to_string(), "_+_".to_string()]));
        let src = exp.export();
        assert!(src.contains("using"));
    }
    #[test]
    fn agda_export_fixity() {
        let mut exp = AgdaExporter::new(AgdaExportConfig::minimal("Test"));
        exp.add_fixity("infixl", 6, "_+_");
        let src = exp.export();
        assert!(src.contains("infixl") && src.contains("6") && src.contains("_+_"));
    }
    #[test]
    fn agda_export_instance() {
        let mut exp = AgdaExporter::new(AgdaExportConfig::minimal("Test"));
        exp.add_instance("natEq", "Eq ℕ", "record { _==_ = natEqFn }");
        let src = exp.export();
        assert!(src.contains("instance"));
        assert!(src.contains("natEq"));
    }
    #[test]
    fn agda_export_pragma() {
        let mut exp = AgdaExporter::new(AgdaExportConfig::minimal("Test"));
        exp.add_pragma(AgdaPragma::Terminating);
        let src = exp.export();
        assert!(src.contains("TERMINATING"));
    }
    #[test]
    fn multi_file_export_count() {
        let mut mfe = MultiFileExport::new("OxiLean");
        let exp = AgdaExporter::new(AgdaExportConfig::minimal("OxiLean.Core"));
        mfe.add_module("Core", &exp);
        mfe.add_raw("OxiLean/Extra.agda", "-- extra\n");
        assert_eq!(mfe.file_count(), 2);
    }
    #[test]
    fn multi_file_export_paths() {
        let mut mfe = MultiFileExport::new("Foo");
        let exp = AgdaExporter::new(AgdaExportConfig::minimal("Foo.Bar"));
        mfe.add_module("Bar", &exp);
        let paths = mfe.paths();
        assert!(!paths.is_empty());
    }
    #[test]
    fn oxi_decl_kinds() {
        let d = OxiDecl::def("f", "Nat", "0");
        assert_eq!(d.kind, OxiDeclKind::Def);
        let a = OxiDecl::axiom("P", "Prop");
        assert_eq!(a.kind, OxiDeclKind::Axiom);
        let t = OxiDecl::theorem("T", "True", "trivial");
        assert_eq!(t.kind, OxiDeclKind::Theorem);
    }
    #[test]
    fn find_arrow_simple() {
        assert!(find_arrow("A -> B").is_some());
    }
    #[test]
    fn find_arrow_none() {
        assert!(find_arrow("A B C").is_none());
    }
    #[test]
    fn find_arrow_nested_parens() {
        assert!(find_arrow("(A -> B)").is_none());
    }
}
/// Return the agda_export module version.
#[allow(dead_code)]
pub fn agda_export_version() -> &'static str {
    "0.1.0"
}
#[cfg(test)]
mod agda_extra_tests {
    use super::*;
    #[test]
    fn test_agda_module_config_pragma() {
        let cfg = AgdaModuleConfig::default();
        let header = cfg.pragma_header();
        assert!(header.contains("--safe"));
        assert!(header.contains("module Main where"));
        assert!(header.contains("open import"));
    }
    #[test]
    fn test_agda_record_render() {
        let record = AgdaRecord::new("Point")
            .with_param("A", "Set")
            .with_field("x", "A")
            .with_field("y", "A")
            .with_constructor("mkPoint");
        let rendered = record.render();
        assert!(rendered.contains("record Point"));
        assert!(rendered.contains("constructor mkPoint"));
        assert!(rendered.contains("x : A"));
        assert!(rendered.contains("y : A"));
    }
    #[test]
    fn test_agda_proof_term_refl() {
        let term = AgdaProofTerm::Refl;
        assert_eq!(term.render(), "refl");
        assert_eq!(term.depth(), 0);
    }
    #[test]
    fn test_agda_proof_term_sym_trans() {
        let sym = AgdaProofTerm::Sym(Box::new(AgdaProofTerm::Refl));
        assert!(sym.render().contains("sym"));
        let trans =
            AgdaProofTerm::Trans(Box::new(AgdaProofTerm::Refl), Box::new(AgdaProofTerm::Refl));
        assert!(trans.render().contains("trans"));
        assert_eq!(trans.depth(), 1);
    }
    #[test]
    fn test_agda_proof_term_hole() {
        let hole = AgdaProofTerm::Hole("goal".to_string());
        assert_eq!(hole.render(), "?goal");
    }
    #[test]
    fn test_agda_export_stats() {
        let stats = AgdaExportStats {
            theorems_exported: 10,
            definitions_exported: 5,
            records_exported: 2,
            inductives_exported: 3,
            total_lines: 300,
        };
        let summary = stats.summary();
        assert!(summary.contains("10 theorems"));
        assert!(summary.contains("300 lines"));
    }
    #[test]
    fn test_agda_export_version() {
        assert!(!agda_export_version().is_empty());
    }
}
#[allow(dead_code)]
pub fn agda_sanitize_name(name: &str) -> String {
    name.chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '_' || c == '-' {
                c
            } else {
                '_'
            }
        })
        .collect()
}
#[allow(dead_code)]
pub fn agda_indent(level: usize) -> String {
    "  ".repeat(level)
}
#[allow(dead_code)]
pub fn agda_format_type_sig(name: &str, ty: &str) -> String {
    format!("{} : {}", name, ty)
}
#[allow(dead_code)]
pub fn agda_format_def(name: &str, args: &[&str], body: &str) -> String {
    if args.is_empty() {
        format!("{} = {}", name, body)
    } else {
        format!("{} {} = {}", name, args.join(" "), body)
    }
}
#[allow(dead_code)]
pub fn agda_comment(text: &str) -> String {
    format!("-- {}", text)
}
#[allow(dead_code)]
pub fn agda_block_comment(text: &str) -> String {
    format!("{{- {} -}}", text)
}
#[allow(dead_code)]
pub fn agda_pragma(name: &str, args: &str) -> String {
    format!("{{-# {} {} #-}}", name, args)
}
#[cfg(test)]
mod agda_extra_tests_3 {
    use super::*;
    #[test]
    fn test_import_decl_render() {
        let imp = AgdaImportDecl::new(vec!["Data".to_string(), "Nat".to_string()]);
        assert_eq!(imp.render(), "import Data.Nat");
    }
    #[test]
    fn test_sanitize_name() {
        assert_eq!(agda_sanitize_name("foo_bar"), "foo_bar");
        assert_eq!(agda_sanitize_name("foo.bar"), "foo_bar");
    }
    #[test]
    fn test_agda_comment() {
        assert_eq!(agda_comment("hello"), "-- hello");
    }
    #[test]
    fn test_agda_pragma() {
        assert_eq!(
            agda_pragma("BUILTIN", "NATURAL Nat"),
            "{-# BUILTIN NATURAL Nat #-}"
        );
    }
}
