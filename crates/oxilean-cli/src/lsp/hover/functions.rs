//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lsp::{
    analyze_document, Document, DocumentStore, Hover, Location, MarkupContent, Position, Range,
    SymbolKind,
};
use oxilean_kernel::{Environment, Name};

use super::types::{
    CompletionItem, CompletionKind, CompletionProvider, DefinitionProvider, DocumentationGenerator,
    HoverCache, HoverEvent, HoverHistory, HoverMergeStrategy, HoverProvider, HoverResult,
    HoverSection, HoverSectionKind, NamespaceHoverProvider, OutlineProvider, OutlineSymbol,
    ParameterInfo, ProofGoal, ProofState, ReferenceContext, ReferenceProvider, RichHoverContent,
    SignatureHelpProvider, SignatureInfo,
};

/// Comprehensive keyword documentation.
pub const KEYWORD_DOCS: &[(&str, &str)] = &[
    (
        "def",
        "**def** -- Define a new function or value.\n\n```lean\ndef name : Type := value\n```\n\nUse `def` to introduce non-recursive definitions, or recursive ones\nwith decreasing arguments.",
    ),
    (
        "theorem",
        "**theorem** -- State and prove a proposition.\n\n```lean\ntheorem name : Prop := proof\ntheorem name : Prop := by tactic\n```\n\nThe body is erased at runtime (proof irrelevance).",
    ),
    (
        "lemma",
        "**lemma** -- Alias for `theorem`.\n\nConventionally used for smaller auxiliary results.",
    ),
    (
        "axiom",
        "**axiom** -- Postulate a type without proof.\n\n```lean\naxiom name : Type\n```\n\nWarning: axioms can introduce inconsistency.",
    ),
    (
        "inductive",
        "**inductive** -- Define an inductive data type.\n\n```lean\ninductive Name where\n  | ctor : ... -> Name\n```\n\nAutomatically generates recursors and case analysis principles.",
    ),
    (
        "structure",
        "**structure** -- Define a record type with named fields.\n\n```lean\nstructure Point where\n  x : Float\n  y : Float\n```\n\nA single-constructor inductive with projection functions.",
    ),
    (
        "class",
        "**class** -- Define a type class for ad-hoc polymorphism.\n\n```lean\nclass Inhabited (a : Type) where\n  default : a\n```\n\nType classes enable overloading and automatic instance resolution.",
    ),
    (
        "instance",
        "**instance** -- Provide a type class instance.\n\n```lean\ninstance : Inhabited Nat where\n  default := 0\n```",
    ),
    (
        "fun",
        "**fun** -- Lambda abstraction (anonymous function).\n\n```lean\nfun x => x + 1\nfun (x : Nat) (y : Nat) => x + y\n```",
    ),
    (
        "forall",
        "**forall** -- Universal quantification / dependent function type.\n\n```lean\nforall (x : Nat), x = x\n(x : Nat) -> P x        -- notation for forall\n```",
    ),
    (
        "match",
        "**match** -- Pattern matching.\n\n```lean\nmatch x with\n  | 0 => \"zero\"\n  | n + 1 => \"succ\"\n```",
    ),
    ("let", "**let** -- Local binding.\n\n```lean\nlet x := 42\nin x + 1\n```"),
    ("in", "**in** -- Body of a `let` or `do` expression."),
    (
        "if",
        "**if** -- Conditional expression.\n\n```lean\nif h : p then a else b   -- with proof\nif p then a else b        -- decidable\n```",
    ),
    ("then", "**then** -- True branch of an `if` expression."),
    ("else", "**else** -- False branch of an `if` expression."),
    (
        "do",
        "**do** -- Do-notation for monadic code.\n\n```lean\ndo\n  let x <- getLine\n  IO.println x\n```",
    ),
    (
        "by",
        "**by** -- Enter tactic mode to construct a proof.\n\n```lean\ntheorem p : 1 + 1 = 2 := by\n  rfl\n```",
    ),
    (
        "sorry",
        "**sorry** -- Placeholder for incomplete proofs.\n\nMarks the proof obligation as admitted (unsound).",
    ),
    (
        "where",
        "**where** -- Introduce local definitions after a declaration.\n\n```lean\ndef f (x : Nat) : Nat := g x + 1 where\n  g (n : Nat) : Nat := n * 2\n```",
    ),
    (
        "have",
        "**have** -- Introduce a local hypothesis.\n\n```lean\nhave h : P := proof\nbody\n```",
    ),
    (
        "show",
        "**show** -- Annotate the expected type of an expression.\n\n```lean\nshow Nat\nexact 42\n```",
    ),
    (
        "Prop",
        "**Prop** -- The type of propositions (`Sort 0`).\n\nAll terms of type `Prop` are proof-irrelevant.",
    ),
    (
        "Type",
        "**Type** -- The type of types (`Sort 1`).\n\n`Type 0` = `Type`, `Type 1`, etc.",
    ),
    ("Sort", "**Sort** -- A universe.\n\n`Sort 0` = `Prop`, `Sort 1` = `Type 0`, etc."),
    (
        "namespace",
        "**namespace** -- Open a namespace for declarations.\n\n```lean\nnamespace Foo\ndef bar := 42\nend Foo\n-- Access as Foo.bar\n```",
    ),
    (
        "section",
        "**section** -- Begin a section for local variables.\n\nVariables declared in a section are automatically\nincluded as implicit parameters.",
    ),
    (
        "open",
        "**open** -- Open a namespace to use its names unqualified.\n\n```lean\nopen Nat in\ndef f := succ zero\n```",
    ),
    (
        "import",
        "**import** -- Import definitions from another module.\n\n```lean\nimport Init.Prelude\nimport Mathlib.Tactic\n```",
    ),
    (
        "variable",
        "**variable** -- Declare a section variable.\n\n```lean\nvariable (n : Nat) (h : n > 0)\n```",
    ),
    ("end", "**end** -- Close a `namespace`, `section`, or `where` block."),
    (
        "return",
        "**return** -- Return a value in do-notation.\n\n```lean\ndo return 42\n```",
    ),
    ("true", "**true** -- Boolean true value of type `Bool`."),
    ("false", "**false** -- Boolean false value of type `Bool`."),
    ("with", "**with** -- Begin match arms or introduce a `where` clause."),
    ("export", "**export** -- Re-export names from an opened namespace."),
    (
        "set_option",
        "**set_option** -- Set a Lean option.\n\n```lean\nset_option maxRecDepth 1000\n```",
    ),
];
/// Tactic documentation for hover.
pub const TACTIC_DOCS: &[(&str, &str)] = &[
    (
        "intro",
        "Introduce one hypothesis from the goal into the context.\n\n```lean\nintro h\nintro x y z\n```",
    ),
    (
        "intros",
        "Introduce all leading hypotheses.\n\n```lean\nintros\nintros h1 h2\n```",
    ),
    (
        "apply",
        "Apply a lemma or hypothesis to the current goal.\n\n```lean\napply And.intro\n```",
    ),
    ("exact", "Close the goal with an exact proof term.\n\n```lean\nexact rfl\n```"),
    ("rfl", "Close the goal by reflexivity.\n\nWorks when the goal is `a = a`."),
    (
        "rw",
        "Rewrite the goal using an equation.\n\n```lean\nrw [h]\nrw [<- h]   -- rewrite backwards\n```",
    ),
    (
        "simp",
        "Simplify using the simp lemma set.\n\n```lean\nsimp\nsimp [h1, h2]\nsimp only [Nat.add_comm]\n```",
    ),
    (
        "cases",
        "Perform case analysis on an inductive value.\n\n```lean\ncases h\ncases n with\n| zero => ...\n| succ m => ...\n```",
    ),
    (
        "induction",
        "Perform induction on a term.\n\n```lean\ninduction n with\n| zero => ...\n| succ n ih => ...\n```",
    ),
    (
        "constructor",
        "Apply a constructor to split a goal.\n\nFor `And`: splits into two subgoals.\nFor `Exists`: introduces a witness.",
    ),
    ("assumption", "Close the goal if it matches a hypothesis."),
    ("contradiction", "Close the goal by finding a contradiction in the context."),
    ("sorry", "Admit the current goal (unsound)."),
    (
        "have",
        "Introduce an intermediate lemma in tactic mode.\n\n```lean\nhave h : P := by ...\n```",
    ),
    (
        "calc",
        "Begin a calculational proof.\n\n```lean\ncalc a = b := by ...\n     _ = c := by ...\n```",
    ),
    ("ring", "Prove equalities in commutative (semi)rings."),
    ("omega", "Solve linear arithmetic goals over natural numbers and integers."),
    ("linarith", "Linear arithmetic decision procedure."),
    ("norm_num", "Normalize numeric expressions and close numeric goals."),
    ("decide", "Decide a decidable proposition by computation."),
    ("trivial", "Solve trivial goals using `rfl`, `assumption`, and simple lemmas."),
    ("left", "Choose the left disjunct of an `Or` goal."),
    ("right", "Choose the right disjunct of an `Or` goal."),
    ("obtain", "Destructure a hypothesis.\n\n```lean\nobtain \\<h1, h2\\> := h\n```"),
    ("ext", "Apply extensionality to the goal."),
    ("funext", "Apply function extensionality.\n\n```lean\nfunext x\n```"),
    ("congr", "Apply congruence to reduce the goal."),
    (
        "specialize",
        "Specialize a hypothesis with arguments.\n\n```lean\nspecialize h 42\n```",
    ),
    ("revert", "Move a hypothesis back into the goal."),
    ("clear", "Remove a hypothesis from the context."),
];
/// Get the type of the symbol at the given position.
pub fn type_at_position(doc: &Document, pos: &Position, env: &Environment) -> Option<String> {
    let (word, _) = doc.word_at_position(pos)?;
    infer_type_at(&word, doc, env)
}
/// Infer the type of a name from the environment or document analysis.
pub fn infer_type_at(word: &str, doc: &Document, env: &Environment) -> Option<String> {
    let name = Name::str(word);
    if let Some(ci) = env.find(&name) {
        return Some(format!("{:?}", ci.ty()));
    }
    let analysis = analyze_document(&doc.uri, &doc.content, env);
    for def in &analysis.definitions {
        if def.name == word {
            return def.ty.clone();
        }
    }
    if word.chars().all(|c| c.is_ascii_digit()) {
        return Some("Nat".to_string());
    }
    if word.starts_with('"') && word.ends_with('"') {
        return Some("String".to_string());
    }
    if word == "true" || word == "false" {
        return Some("Bool".to_string());
    }
    None
}
/// Format a type string for hover display.
pub fn format_type_hover(name: &str, ty: &str) -> String {
    format!("```lean\n{} : {}\n```", name, ty)
}
/// Format a full declaration for hover display, including kind and documentation.
pub fn format_declaration_hover(kind: &str, name: &str, ty: &str) -> String {
    format!("```lean\n{} {} : {}\n```", kind, name, ty)
}
/// Split a type string on `->` at the top nesting level (depth 0).
///
/// Respects parentheses, square brackets, and curly braces so that
/// `(A -> B) -> C` splits into `["(A -> B)", " C"]` rather than three parts.
pub fn split_arrow_top(ty: &str) -> Vec<&str> {
    let mut parts = Vec::new();
    let mut depth: usize = 0;
    let mut start = 0;
    let bytes = ty.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        match bytes[i] {
            b'(' | b'[' | b'{' => depth += 1,
            b')' | b']' | b'}' => depth = depth.saturating_sub(1),
            b'-' if depth == 0 && i + 1 < bytes.len() && bytes[i + 1] == b'>' => {
                parts.push(&ty[start..i]);
                start = i + 2;
                i += 2;
                continue;
            }
            _ => {}
        }
        i += 1;
    }
    parts.push(&ty[start..]);
    parts
}
/// Build a readable label for the i-th parameter given its type segment string.
///
/// If the segment looks like `(name : Type)` or `name : Type`, extracts name.
/// Otherwise falls back to `arg{i} : {segment}`.
pub fn extract_param_label(i: usize, segment: &str) -> String {
    let inner = if segment.starts_with('(') && segment.ends_with(')') {
        &segment[1..segment.len() - 1]
    } else {
        segment
    };
    if let Some(colon) = inner.find(':') {
        let name_part = inner[..colon].trim();
        let type_part = inner[colon + 1..].trim();
        if !name_part.is_empty()
            && name_part
                .chars()
                .all(|c| c.is_alphanumeric() || c == '_' || c == '\'')
        {
            return format!("{} : {}", name_part, type_part);
        }
    }
    if segment.is_empty() {
        format!("arg{}", i)
    } else {
        format!("arg{} : {}", i, segment)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn make_doc(content: &str) -> Document {
        Document::new("file:///test.lean", 1, content)
    }
    #[test]
    fn test_hover_result_construction() {
        let result = HoverResult {
            content: "hello".to_string(),
            range: Range::single_line(0, 0, 5),
        };
        assert_eq!(result.content, "hello");
    }
    #[test]
    fn test_hover_keyword_def() {
        let env = Environment::new();
        let provider = HoverProvider::new(&env);
        let info = provider.hover_keyword("def");
        assert!(info.is_some());
        assert!(info.expect("test operation should succeed").contains("def"));
    }
    #[test]
    fn test_hover_keyword_unknown() {
        let env = Environment::new();
        let provider = HoverProvider::new(&env);
        let info = provider.hover_keyword("zzz_not_a_keyword");
        assert!(info.is_none());
    }
    #[test]
    fn test_hover_literal_nat() {
        let env = Environment::new();
        let provider = HoverProvider::new(&env);
        let info = provider.hover_literal("42");
        assert!(info.is_some());
        assert!(info.expect("test operation should succeed").contains("Nat"));
    }
    #[test]
    fn test_hover_literal_negative() {
        let env = Environment::new();
        let provider = HoverProvider::new(&env);
        let info = provider.hover_literal("-7");
        assert!(info.is_some());
        assert!(info.expect("test operation should succeed").contains("Int"));
    }
    #[test]
    fn test_hover_literal_not_number() {
        let env = Environment::new();
        let provider = HoverProvider::new(&env);
        assert!(provider.hover_literal("abc").is_none());
    }
    #[test]
    fn test_hover_tactic_intro() {
        let env = Environment::new();
        let provider = HoverProvider::new(&env);
        let info = provider.hover_tactic("intro");
        assert!(info.is_some());
        assert!(info
            .expect("test operation should succeed")
            .contains("hypothesis"));
    }
    #[test]
    fn test_hover_tactic_unknown() {
        let env = Environment::new();
        let provider = HoverProvider::new(&env);
        assert!(provider.hover_tactic("not_a_tactic_xyz").is_none());
    }
    #[test]
    fn test_hover_at_position_keyword() {
        let env = Environment::new();
        let provider = HoverProvider::new(&env);
        let doc = make_doc("def foo := 1");
        let result = provider.hover_at_position(&doc, &Position::new(0, 1));
        assert!(result.is_some());
        assert!(result
            .expect("test operation should succeed")
            .content
            .contains("def"));
    }
    #[test]
    fn test_definition_provider_in_source() {
        let env = Environment::new();
        let provider = DefinitionProvider::new(&env);
        let doc = make_doc("def foo := 1\ndef bar := foo");
        let loc = provider.goto_definition(&doc, &Position::new(1, 12));
        assert!(loc.is_some());
    }
    #[test]
    fn test_definition_provider_not_found() {
        let env = Environment::new();
        let provider = DefinitionProvider::new(&env);
        let doc = make_doc("def foo := 1");
        let loc = provider.goto_definition(&doc, &Position::new(0, 20));
        assert!(loc.is_none());
    }
    #[test]
    fn test_reference_provider_find_refs() {
        let env = Environment::new();
        let provider = ReferenceProvider::new(&env);
        let doc = make_doc("def foo := 1\ndef bar := foo");
        let ctx = ReferenceContext::default();
        let refs = provider.find_references(&doc, &Position::new(0, 5), &ctx);
        assert!(refs.len() >= 2);
    }
    #[test]
    fn test_reference_provider_workspace() {
        let env = Environment::new();
        let provider = ReferenceProvider::new(&env);
        let mut store = DocumentStore::new();
        store.open_document("file:///a.lean", 1, "def foo := 1");
        store.open_document("file:///b.lean", 1, "def bar := foo");
        let refs = provider.find_references_in_workspace(&store, "foo");
        assert!(refs.len() >= 2);
    }
    #[test]
    fn test_reference_context_default() {
        let ctx = ReferenceContext::default();
        assert!(ctx.include_declaration);
    }
    #[test]
    fn test_type_at_position_literal() {
        let doc = make_doc("42");
        let env = Environment::new();
        let ty = type_at_position(&doc, &Position::new(0, 1), &env);
        assert!(ty.is_some());
        assert_eq!(ty.expect("test operation should succeed"), "Nat");
    }
    #[test]
    fn test_infer_type_at_bool() {
        let doc = make_doc("true");
        let env = Environment::new();
        let ty = infer_type_at("true", &doc, &env);
        assert_eq!(ty.as_deref(), Some("Bool"));
    }
    #[test]
    fn test_format_type_hover() {
        let result = format_type_hover("x", "Nat");
        assert_eq!(result, "```lean\nx : Nat\n```");
    }
    #[test]
    fn test_format_declaration_hover() {
        let result = format_declaration_hover("def", "foo", "Nat -> Nat");
        assert_eq!(result, "```lean\ndef foo : Nat -> Nat\n```");
    }
    #[test]
    fn test_definition_in_imports() {
        let env = Environment::new();
        let provider = DefinitionProvider::new(&env);
        let mut store = DocumentStore::new();
        store.open_document("file:///other.lean", 1, "def myHelper := 1");
        let loc = provider.find_definition_in_imports(&store, "myHelper", "file:///main.lean");
        assert!(loc.is_some());
    }
}
#[cfg(test)]
mod extended_tests {
    use super::*;
    fn make_doc(content: &str) -> Document {
        Document::new("file:///test.lean", 1, content)
    }
    #[test]
    fn test_outline_find_symbol() {
        let env = Environment::new();
        let provider = OutlineProvider::new(&env);
        let doc = make_doc("def foo := 1");
        let sym = provider.find_symbol(&doc, "foo");
        assert!(sym.is_some());
        assert_eq!(sym.expect("test operation should succeed").name, "foo");
    }
    #[test]
    fn test_outline_symbol_fields() {
        let sym = OutlineSymbol {
            name: "myDef".to_string(),
            kind: SymbolKind::Function,
            range: Range::single_line(0, 0, 5),
            ty: Some("Nat".to_string()),
            children: Vec::new(),
        };
        assert_eq!(sym.name, "myDef");
        assert!(sym.children.is_empty());
    }
    #[test]
    fn test_completion_keywords() {
        let env = Environment::new();
        let provider = CompletionProvider::new(&env);
        let doc = make_doc("def");
        let items = provider.completions(&doc, &Position::new(0, 2));
        assert!(!items.is_empty());
        assert!(items.iter().any(|i| i.kind == CompletionKind::Keyword));
    }
    #[test]
    fn test_completion_tactics() {
        let env = Environment::new();
        let provider = CompletionProvider::new(&env);
        let doc = make_doc("int");
        let items = provider.completions(&doc, &Position::new(0, 3));
        assert!(items.iter().any(|i| i.kind == CompletionKind::Tactic));
    }
    #[test]
    fn test_completion_item_fields() {
        let item = CompletionItem {
            label: "intro".to_string(),
            detail: Some("tactic".to_string()),
            documentation: None,
            kind: CompletionKind::Tactic,
            insert_text: None,
        };
        assert_eq!(item.label, "intro");
        assert_eq!(item.kind, CompletionKind::Tactic);
    }
    #[test]
    fn test_signature_help_not_found() {
        let env = Environment::new();
        let provider = SignatureHelpProvider::new(&env);
        let doc = make_doc("xyz_not_a_def_abc");
        let result = provider.signature_help(&doc, &Position::new(0, 1));
        assert!(result.is_none());
    }
    #[test]
    fn test_completion_kind_variants() {
        assert_ne!(CompletionKind::Keyword, CompletionKind::Function);
        assert_eq!(CompletionKind::Snippet, CompletionKind::Snippet);
    }
    #[test]
    fn test_parameter_info_label() {
        let param = ParameterInfo {
            label: "x".to_string(),
            documentation: None,
        };
        assert_eq!(param.label, "x");
    }
    #[test]
    fn test_signature_info_fields() {
        let info = SignatureInfo {
            label: "foo : Nat -> Nat".to_string(),
            documentation: None,
            parameters: vec![ParameterInfo {
                label: "n".to_string(),
                documentation: None,
            }],
            active_parameter: Some(0),
        };
        assert_eq!(info.parameters.len(), 1);
        assert_eq!(info.active_parameter, Some(0));
    }
    #[test]
    fn test_outline_count_of_kind() {
        let env = Environment::new();
        let provider = OutlineProvider::new(&env);
        let doc = make_doc("def foo := 1\ndef bar := 2");
        let count = provider.count_of_kind(&doc, &SymbolKind::Function);
        assert!(count >= 2);
    }
}
/// Format a Lean type signature as a Markdown code block.
#[allow(dead_code)]
pub fn format_type_as_code_block(ty: &str) -> String {
    format!("```lean\n{}\n```", ty)
}
/// Format documentation with a header and optional examples.
#[allow(dead_code)]
pub fn format_doc_with_examples(header: &str, doc: &str, examples: &[&str]) -> String {
    let mut result = format!("**{}**\n\n{}", header, doc);
    if !examples.is_empty() {
        result.push_str("\n\n**Examples:**\n");
        for ex in examples {
            result.push_str(&format!("\n```lean\n{}\n```", ex));
        }
    }
    result
}
/// Format a deprecation warning for hover display.
#[allow(dead_code)]
pub fn format_deprecation_warning(reason: &str, replacement: Option<&str>) -> String {
    let mut msg = format!("⚠️ **Deprecated**: {}", reason);
    if let Some(r) = replacement {
        msg.push_str(&format!("\n\nUse `{}` instead.", r));
    }
    msg
}
/// Format a "see also" section for hover documentation.
#[allow(dead_code)]
pub fn format_see_also(names: &[&str]) -> String {
    if names.is_empty() {
        return String::new();
    }
    let links: Vec<String> = names.iter().map(|n| format!("`{}`", n)).collect();
    format!("\n\n**See also:** {}", links.join(", "))
}
/// Truncate hover text to a maximum length, adding ellipsis.
#[allow(dead_code)]
pub fn truncate_hover_text(text: &str, max_len: usize) -> String {
    if text.len() <= max_len {
        text.to_string()
    } else {
        let truncated = &text[..max_len.saturating_sub(3)];
        format!("{}...", truncated)
    }
}
/// Build a hover string for an inductive type with its constructors.
#[allow(dead_code)]
pub fn format_inductive_hover(
    name: &str,
    params: &[String],
    constructors: &[(String, String)],
) -> String {
    let params_str = if params.is_empty() {
        String::new()
    } else {
        format!(" ({})", params.join(") ("))
    };
    let mut result = format!("```lean\ninductive {}{} where\n", name, params_str);
    for (ctor_name, ctor_ty) in constructors {
        result.push_str(&format!("  | {} : {}\n", ctor_name, ctor_ty));
    }
    result.push_str("```");
    result
}
/// Build a hover string for a structure type with its fields.
#[allow(dead_code)]
pub fn format_structure_hover(name: &str, fields: &[(String, String)]) -> String {
    let mut result = format!("```lean\nstructure {} where\n", name);
    for (field_name, field_ty) in fields {
        result.push_str(&format!("  {} : {}\n", field_name, field_ty));
    }
    result.push_str("```");
    result
}
/// Build a hover string for a class with its methods.
#[allow(dead_code)]
pub fn format_class_hover(
    name: &str,
    type_params: &[String],
    methods: &[(String, String)],
) -> String {
    let params_str = if type_params.is_empty() {
        String::new()
    } else {
        format!(" ({})", type_params.join(") ("))
    };
    let mut result = format!("```lean\nclass{} {} where\n", params_str, name);
    for (method_name, method_ty) in methods {
        result.push_str(&format!("  {} : {}\n", method_name, method_ty));
    }
    result.push_str("```");
    result
}
/// Brief one-line documentation for keywords.
pub const KEYWORD_BRIEFS: &[(&str, &str)] = &[
    ("def", "Define a function or value"),
    ("theorem", "Prove a proposition"),
    ("lemma", "Prove an auxiliary proposition"),
    ("axiom", "Postulate a type without proof"),
    ("inductive", "Define an inductive data type"),
    ("structure", "Define a record type"),
    ("class", "Define a type class"),
    ("instance", "Provide a type class instance"),
    ("fun", "Lambda abstraction"),
    ("forall", "Universal quantifier"),
    ("match", "Pattern matching"),
    ("let", "Local binding"),
    ("have", "Introduce intermediate hypothesis"),
    ("by", "Enter tactic mode"),
    ("sorry", "Admit a goal (unsound)"),
    ("where", "Local definitions"),
    ("namespace", "Open a namespace"),
    ("section", "Begin a section"),
    ("open", "Open a namespace for access"),
    ("import", "Import a module"),
    ("variable", "Declare section variable"),
];
/// Provide hover documentation for Unicode math symbols.
#[allow(dead_code)]
pub fn hover_unicode_symbol(ch: char) -> Option<String> {
    UNICODE_SYMBOL_DOCS.iter().find_map(|&(sym, doc)| {
        if sym == ch {
            Some(format!(
                "**Unicode symbol** `{}` (U+{:04X})\n\n{}",
                sym, sym as u32, doc
            ))
        } else {
            None
        }
    })
}
/// Documentation for common Unicode math symbols.
const UNICODE_SYMBOL_DOCS: &[(char, &str)] = &[
    (
        '∀',
        "Universal quantifier (forall). Equivalent to `forall` keyword.",
    ),
    (
        '∃',
        "Existential quantifier (exists). Equivalent to `exists` keyword.",
    ),
    ('→', "Function arrow. Used in function types `A → B`."),
    ('↔', "Biconditional (iff). `A ↔ B` means `A ↔ B`."),
    ('∧', "Conjunction (and). `A ∧ B` means `A And B`."),
    ('∨', "Disjunction (or). `A ∨ B` means `A Or B`."),
    ('¬', "Negation (not). `¬A` means `Not A`."),
    ('≤', "Less-than-or-equal. `a ≤ b` means `a ≤ b`."),
    ('≥', "Greater-than-or-equal. `a ≥ b` means `a ≥ b`."),
    ('≠', "Not-equal. `a ≠ b` means `a ≠ b`."),
    ('α', "Type variable (alpha). Common for polymorphic types."),
    (
        'β',
        "Type variable (beta). Common for the second type parameter.",
    ),
    (
        'γ',
        "Type variable (gamma). Common for the third type parameter.",
    ),
    ('λ', "Lambda. Used as an alias for `fun` in some notations."),
    (
        '⟨',
        "Angle bracket (open). Used for anonymous constructor syntax.",
    ),
    (
        '⟩',
        "Angle bracket (close). Used for anonymous constructor syntax.",
    ),
    ('ℕ', "Natural numbers type. Equivalent to `Nat`."),
    ('ℤ', "Integers type. Equivalent to `Int`."),
    ('ℝ', "Real numbers type. Equivalent to `Real`."),
    ('ℚ', "Rational numbers type. Equivalent to `Rat`."),
    (
        '⊢',
        "Turnstile. Separates hypotheses from goal in sequent notation.",
    ),
    (
        '⊥',
        "Bottom / False. The uninhabited type or logical false.",
    ),
    ('⊤', "Top / True. The unit type or logical true."),
    ('∑', "Summation symbol. Used for `Finset.sum`."),
    ('∏', "Product symbol. Used for `Finset.prod`."),
];
/// Common type aliases and their expansions.
#[allow(dead_code)]
const TYPE_ALIASES: &[(&str, &str, &str)] = &[
    ("ℕ", "Nat", "Natural numbers (0, 1, 2, ...)"),
    ("ℤ", "Int", "Integers (..., -1, 0, 1, ...)"),
    ("ℝ", "Real", "Real numbers"),
    ("ℚ", "Rat", "Rational numbers (p/q)"),
    (
        "Bool",
        "Bool",
        "Boolean type with values `true` and `false`",
    ),
    ("String", "String", "UTF-8 string"),
    ("List", "List", "Linked list type"),
    ("Option", "Option", "Optional value: `none` or `some x`"),
    ("Prod", "Prod", "Product type (pair)"),
    ("Sum", "Sum", "Sum type (disjoint union)"),
    ("Fin", "Fin", "Finite type `Fin n` has elements 0..n-1"),
    ("Array", "Array", "Resizable array"),
    ("IO", "IO", "IO monad for side effects"),
];
/// Provide hover for a type alias or notation.
#[allow(dead_code)]
pub fn hover_type_alias(name: &str) -> Option<String> {
    TYPE_ALIASES.iter().find_map(|&(alias, expansion, doc)| {
        if alias == name || expansion == name {
            Some(format!(
                "**Type** `{}`\n\nExpansion: `{}`\n\n{}",
                alias, expansion, doc
            ))
        } else {
            None
        }
    })
}
/// Combines results from multiple hover sources.
#[allow(dead_code)]
pub fn merge_hover_results(
    results: Vec<Option<HoverResult>>,
    strategy: HoverMergeStrategy,
) -> Option<HoverResult> {
    let valid: Vec<HoverResult> = results.into_iter().flatten().collect();
    if valid.is_empty() {
        return None;
    }
    match strategy {
        HoverMergeStrategy::FirstWins => Some(
            valid
                .into_iter()
                .next()
                .expect("valid is non-empty: checked by early return"),
        ),
        HoverMergeStrategy::ConcatenateAll => {
            let first = valid[0].clone();
            let combined_content = valid
                .iter()
                .map(|r| r.content.as_str())
                .collect::<Vec<_>>()
                .join("\n\n---\n\n");
            Some(HoverResult {
                content: combined_content,
                range: first.range,
            })
        }
        HoverMergeStrategy::LongestWins => valid.into_iter().max_by_key(|r| r.content.len()),
    }
}
/// Get hover info for a hexadecimal literal.
#[allow(dead_code)]
pub fn hover_hex_literal(word: &str) -> Option<String> {
    if word.starts_with("0x") || word.starts_with("0X") {
        let hex_part = &word[2..];
        if let Ok(val) = u64::from_str_radix(hex_part, 16) {
            return Some(format!(
                "```lean\n{} : Nat\n```\nHexadecimal literal = {} (decimal)",
                word, val
            ));
        }
    }
    None
}
/// Get hover info for a binary literal.
#[allow(dead_code)]
pub fn hover_binary_literal(word: &str) -> Option<String> {
    if word.starts_with("0b") || word.starts_with("0B") {
        let bin_part = &word[2..];
        if let Ok(val) = u64::from_str_radix(bin_part, 2) {
            return Some(format!(
                "```lean\n{} : Nat\n```\nBinary literal = {} (decimal)",
                word, val
            ));
        }
    }
    None
}
/// Get hover info for an octal literal.
#[allow(dead_code)]
pub fn hover_octal_literal(word: &str) -> Option<String> {
    if word.starts_with("0o") || word.starts_with("0O") {
        let oct_part = &word[2..];
        if let Ok(val) = u64::from_str_radix(oct_part, 8) {
            return Some(format!(
                "```lean\n{} : Nat\n```\nOctal literal = {} (decimal)",
                word, val
            ));
        }
    }
    None
}
/// Documentation for common operators.
#[allow(dead_code)]
const OPERATOR_DOCS: &[(&str, &str)] = &[
    (
        "+",
        "Addition operator. `a + b : α` when `Add α` instance is available.",
    ),
    (
        "-",
        "Subtraction operator. `a - b : α` when `Sub α` instance is available.",
    ),
    (
        "*",
        "Multiplication operator. `a * b : α` when `Mul α` instance is available.",
    ),
    (
        "/",
        "Division operator. `a / b : α` when `Div α` instance is available.",
    ),
    (
        "%",
        "Modulo operator. `a % b : α` when `Mod α` instance is available.",
    ),
    (
        "^",
        "Power operator. `a ^ n : α` when `HPow α Nat α` instance is available.",
    ),
    ("=", "Propositional equality. `a = b : Prop`."),
    (
        "<",
        "Less-than. `a < b : Prop` when `LT α` instance is available.",
    ),
    (">", "Greater-than. `a > b : Prop`, defined as `b < a`."),
    (
        "<=",
        "Less-than-or-equal. `a <= b : Prop` when `LE α` instance is available.",
    ),
    (
        ">=",
        "Greater-than-or-equal. `a >= b : Prop`, defined as `b <= a`.",
    ),
    ("++", "Append. `a ++ b : List α` or `String`."),
    ("::", "Cons. `x :: xs : List α`."),
    ("&&", "Boolean and. `a && b : Bool`."),
    ("||", "Boolean or. `a || b : Bool`."),
    ("!", "Boolean not. `!b : Bool`."),
    (":=", "Definition assignment."),
    ("=>", "Arrow in match arms or `fun`."),
    ("->", "Function type arrow. `A -> B : Type`."),
    (
        "<|",
        "Pipe operator (right-to-left application). `f <| x` = `f x`.",
    ),
    (
        "|>",
        "Pipe operator (left-to-right application). `x |> f` = `f x`.",
    ),
    (
        "$",
        "Function application (low precedence). `f $ x` = `f x`.",
    ),
    ("#", "Hash (array literal prefix or special command)."),
];
/// Get hover documentation for an operator.
#[allow(dead_code)]
pub fn hover_operator(op: &str) -> Option<String> {
    OPERATOR_DOCS.iter().find_map(|&(o, doc)| {
        if o == op {
            Some(format!("**Operator** `{}`\n\n{}", op, doc))
        } else {
            None
        }
    })
}
#[cfg(test)]
mod hover_extended_tests {
    use super::*;
    #[test]
    fn test_format_type_as_code_block() {
        let result = format_type_as_code_block("Nat -> Nat");
        assert!(result.contains("```lean"));
        assert!(result.contains("Nat -> Nat"));
    }
    #[test]
    fn test_format_doc_with_examples() {
        let result = format_doc_with_examples("Foo", "This is foo", &["foo 1 2"]);
        assert!(result.contains("Foo"));
        assert!(result.contains("foo 1 2"));
    }
    #[test]
    fn test_format_doc_with_no_examples() {
        let result = format_doc_with_examples("Bar", "This is bar", &[]);
        assert!(result.contains("Bar"));
        assert!(!result.contains("Examples:"));
    }
    #[test]
    fn test_format_deprecation_warning() {
        let result = format_deprecation_warning("old API", Some("newFn"));
        assert!(result.contains("Deprecated"));
        assert!(result.contains("newFn"));
    }
    #[test]
    fn test_format_deprecation_no_replacement() {
        let result = format_deprecation_warning("old API", None);
        assert!(result.contains("Deprecated"));
        assert!(!result.contains("instead"));
    }
    #[test]
    fn test_format_see_also() {
        let result = format_see_also(&["Nat.add", "Nat.mul"]);
        assert!(result.contains("See also"));
        assert!(result.contains("Nat.add"));
    }
    #[test]
    fn test_format_see_also_empty() {
        let result = format_see_also(&[]);
        assert!(result.is_empty());
    }
    #[test]
    fn test_truncate_hover_text_short() {
        let result = truncate_hover_text("hello", 100);
        assert_eq!(result, "hello");
    }
    #[test]
    fn test_truncate_hover_text_long() {
        let result = truncate_hover_text("hello world foo bar baz", 10);
        assert!(result.len() <= 10);
        assert!(result.ends_with("..."));
    }
    #[test]
    fn test_hover_cache_insert_and_get() {
        let mut cache = HoverCache::new(10);
        let result = Some(HoverResult {
            content: "test".to_string(),
            range: Range::single_line(0, 0, 4),
        });
        cache.insert("file:///a.lean", 0, 0, result.clone());
        let got = cache.get("file:///a.lean", 0, 0);
        assert!(got.is_some());
    }
    #[test]
    fn test_hover_cache_invalidate() {
        let mut cache = HoverCache::new(10);
        cache.insert("file:///a.lean", 0, 0, None);
        cache.insert("file:///b.lean", 1, 2, None);
        cache.invalidate("file:///a.lean");
        assert!(cache.get("file:///a.lean", 0, 0).is_none());
        assert!(cache.get("file:///b.lean", 1, 2).is_some());
    }
    #[test]
    fn test_hover_cache_clear() {
        let mut cache = HoverCache::new(10);
        cache.insert("file:///a.lean", 0, 0, None);
        cache.clear();
        assert!(cache.is_empty());
    }
    #[test]
    fn test_proof_goal_format() {
        let mut goal = ProofGoal::new("P -> Q");
        goal.add_hyp("h", "P");
        let formatted = goal.format_hover();
        assert!(formatted.contains("h : P"));
        assert!(formatted.contains("P -> Q"));
    }
    #[test]
    fn test_proof_goal_closed() {
        let goal = ProofGoal::new("True");
        assert!(goal.is_closed());
    }
    #[test]
    fn test_proof_goal_not_closed() {
        let goal = ProofGoal::new("P ∧ Q");
        assert!(!goal.is_closed());
    }
    #[test]
    fn test_proof_state_no_goals() {
        let state = ProofState::new();
        let formatted = state.format_hover();
        assert!(formatted.contains("No goals"));
    }
    #[test]
    fn test_proof_state_with_goals() {
        let mut state = ProofState::new();
        state.add_goal(ProofGoal::new("1 = 1"));
        state.add_goal(ProofGoal::new("2 = 2"));
        assert_eq!(state.goal_count(), 2);
        let formatted = state.format_hover();
        assert!(formatted.contains("2 Goals"));
    }
    #[test]
    fn test_hover_unicode_symbol_forall() {
        let result = hover_unicode_symbol('∀');
        assert!(result.is_some());
        assert!(result
            .expect("test operation should succeed")
            .contains("Universal"));
    }
    #[test]
    fn test_hover_unicode_symbol_unknown() {
        let result = hover_unicode_symbol('A');
        assert!(result.is_none());
    }
    #[test]
    fn test_hover_type_alias_nat() {
        let result = hover_type_alias("ℕ");
        assert!(result.is_some());
        assert!(result
            .expect("test operation should succeed")
            .contains("Natural numbers"));
    }
    #[test]
    fn test_hover_operator_plus() {
        let result = hover_operator("+");
        assert!(result.is_some());
        assert!(result
            .expect("test operation should succeed")
            .contains("Addition"));
    }
    #[test]
    fn test_hover_operator_unknown() {
        let result = hover_operator("@@@");
        assert!(result.is_none());
    }
    #[test]
    fn test_hover_hex_literal() {
        let result = hover_hex_literal("0xFF");
        assert!(result.is_some());
        assert!(result
            .expect("test operation should succeed")
            .contains("255"));
    }
    #[test]
    fn test_hover_hex_literal_invalid() {
        let result = hover_hex_literal("hello");
        assert!(result.is_none());
    }
    #[test]
    fn test_hover_binary_literal() {
        let result = hover_binary_literal("0b1010");
        assert!(result.is_some());
        assert!(result
            .expect("test operation should succeed")
            .contains("10"));
    }
    #[test]
    fn test_hover_octal_literal() {
        let result = hover_octal_literal("0o17");
        assert!(result.is_some());
        assert!(result
            .expect("test operation should succeed")
            .contains("15"));
    }
    #[test]
    fn test_format_inductive_hover() {
        let result = format_inductive_hover(
            "Color",
            &[],
            &[
                ("red".to_string(), "Color".to_string()),
                ("blue".to_string(), "Color".to_string()),
            ],
        );
        assert!(result.contains("inductive Color"));
        assert!(result.contains("red"));
        assert!(result.contains("blue"));
    }
    #[test]
    fn test_format_structure_hover() {
        let result = format_structure_hover(
            "Point",
            &[
                ("x".to_string(), "Float".to_string()),
                ("y".to_string(), "Float".to_string()),
            ],
        );
        assert!(result.contains("structure Point"));
        assert!(result.contains("x : Float"));
    }
    #[test]
    fn test_format_class_hover() {
        let result = format_class_hover(
            "Repr",
            &["α".to_string()],
            &[("repr".to_string(), "α -> String".to_string())],
        );
        assert!(result.contains("class"));
        assert!(result.contains("Repr"));
        assert!(result.contains("repr"));
    }
    #[test]
    fn test_merge_hover_results_first_wins() {
        let r1 = Some(HoverResult {
            content: "first".to_string(),
            range: Range::single_line(0, 0, 5),
        });
        let r2 = Some(HoverResult {
            content: "second".to_string(),
            range: Range::single_line(0, 0, 6),
        });
        let merged = merge_hover_results(vec![r1, r2], HoverMergeStrategy::FirstWins);
        assert_eq!(
            merged.expect("test operation should succeed").content,
            "first"
        );
    }
    #[test]
    fn test_merge_hover_results_longest_wins() {
        let r1 = Some(HoverResult {
            content: "short".to_string(),
            range: Range::single_line(0, 0, 5),
        });
        let r2 = Some(HoverResult {
            content: "much longer content here".to_string(),
            range: Range::single_line(0, 0, 24),
        });
        let merged = merge_hover_results(vec![r1, r2], HoverMergeStrategy::LongestWins);
        assert!(merged.expect("test operation should succeed").content.len() > 5);
    }
    #[test]
    fn test_merge_hover_results_concatenate() {
        let r1 = Some(HoverResult {
            content: "part1".to_string(),
            range: Range::single_line(0, 0, 5),
        });
        let r2 = Some(HoverResult {
            content: "part2".to_string(),
            range: Range::single_line(0, 0, 5),
        });
        let merged = merge_hover_results(vec![r1, r2], HoverMergeStrategy::ConcatenateAll);
        let content = merged.expect("test operation should succeed").content;
        assert!(content.contains("part1"));
        assert!(content.contains("part2"));
    }
    #[test]
    fn test_merge_hover_results_all_none() {
        let merged = merge_hover_results(vec![None, None], HoverMergeStrategy::FirstWins);
        assert!(merged.is_none());
    }
    #[test]
    fn test_hover_history_record_and_rate() {
        let mut hist = HoverHistory::new(10);
        hist.record(HoverEvent {
            uri: "a".into(),
            line: 0,
            character: 0,
            word: None,
            hit: true,
        });
        hist.record(HoverEvent {
            uri: "a".into(),
            line: 1,
            character: 0,
            word: None,
            hit: false,
        });
        assert_eq!(hist.event_count(), 2);
        assert!((hist.hit_rate() - 0.5).abs() < 1e-9);
    }
    #[test]
    fn test_hover_history_max_size() {
        let mut hist = HoverHistory::new(3);
        for i in 0..5 {
            hist.record(HoverEvent {
                uri: "a".into(),
                line: i,
                character: 0,
                word: None,
                hit: true,
            });
        }
        assert_eq!(hist.event_count(), 3);
    }
    #[test]
    fn test_hover_history_clear() {
        let mut hist = HoverHistory::new(10);
        hist.record(HoverEvent {
            uri: "a".into(),
            line: 0,
            character: 0,
            word: None,
            hit: true,
        });
        hist.clear();
        assert_eq!(hist.event_count(), 0);
    }
    #[test]
    fn test_documentation_generator_keyword_brief() {
        let gen = DocumentationGenerator::new();
        let brief = gen.generate_keyword_brief("def");
        assert!(brief.is_some());
        assert!(brief
            .expect("test operation should succeed")
            .contains("function"));
    }
    #[test]
    fn test_documentation_generator_decl_doc() {
        let gen = DocumentationGenerator::new();
        let doc = gen.generate_decl_doc("def", "foo", "Nat -> Nat", Some("A doubling function"));
        assert!(doc.contains("def foo"));
        assert!(doc.contains("doubling"));
    }
    #[test]
    fn test_documentation_generator_tactic_doc() {
        let gen = DocumentationGenerator::new();
        let doc = gen.generate_tactic_doc("intro", "Introduce a hypothesis", Some("intro h"));
        assert!(doc.contains("intro"));
        assert!(doc.contains("Example"));
    }
    #[test]
    fn test_namespace_hover_provider_count() {
        let env = oxilean_kernel::Environment::new();
        let provider = NamespaceHoverProvider::new(&env);
        let count = provider.count_in_namespace("Nat");
        assert_eq!(count, 0);
    }
    #[test]
    fn test_format_type_alternatives() {
        let gen = DocumentationGenerator::new();
        let result = gen.format_type_alternatives(&["Nat", "Int", "Real"]);
        assert!(result.contains("Nat"));
        assert!(result.contains("Int"));
    }
    #[test]
    fn test_format_type_alternatives_empty() {
        let gen = DocumentationGenerator::new();
        let result = gen.format_type_alternatives(&[]);
        assert!(result.is_empty());
    }
}
#[allow(dead_code)]
pub fn truncate_hover_content(content: &str, max_chars: usize) -> String {
    if content.len() <= max_chars {
        content.to_string()
    } else {
        format!("{}...\n*(truncated)*", &content[..max_chars])
    }
}
#[allow(dead_code)]
pub fn format_hover_identifier(name: &str, namespace: Option<&str>) -> String {
    match namespace {
        Some(ns) => format!("`{}.{}`", ns, name),
        None => format!("`{}`", name),
    }
}
#[allow(dead_code)]
pub fn hover_markdown_code_block(lang: &str, code: &str) -> String {
    format!("```{}\n{}\n```", lang, code)
}
#[allow(dead_code)]
pub fn hover_bold(text: &str) -> String {
    format!("**{}**", text)
}
#[allow(dead_code)]
pub fn hover_italic(text: &str) -> String {
    format!("*{}*", text)
}
#[allow(dead_code)]
pub fn hover_link(label: &str, url: &str) -> String {
    format!("[{}]({})", label, url)
}
#[cfg(test)]
mod hover_extra_tests {
    use super::*;
    #[test]
    fn test_hover_section_type_sig() {
        let s = HoverSection::new(HoverSectionKind::TypeSignature, "foo : Nat");
        assert!(s.to_markdown().contains("```lean"));
    }
    #[test]
    fn test_rich_hover_content() {
        let mut h = RichHoverContent::new();
        h.add_type_sig("bar : Nat -> Nat");
        h.add_doc("Computes bar.");
        assert_eq!(h.section_count(), 2);
        let md = h.to_markdown();
        assert!(md.contains("Computes bar."));
    }
    #[test]
    fn test_truncate_hover() {
        let out = truncate_hover_content("hello world", 5);
        assert!(out.starts_with("hello"));
        assert!(out.contains("truncated"));
    }
    #[test]
    fn test_format_hover_identifier() {
        assert_eq!(format_hover_identifier("foo", Some("Nat")), "`Nat.foo`");
        assert_eq!(format_hover_identifier("bar", None), "`bar`");
    }
    #[test]
    fn test_hover_helpers() {
        assert_eq!(hover_bold("hello"), "**hello**");
        assert_eq!(hover_italic("world"), "*world*");
        assert!(hover_link("click", "http://example.com").contains("click"));
    }
}
