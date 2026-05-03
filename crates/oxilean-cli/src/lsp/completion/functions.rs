//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lsp::{
    analyze_document, CompletionItem, CompletionItemKind, Document, MarkupContent, Position, Range,
};
use oxilean_kernel::Environment;

use super::types::{
    CompletionContext, CompletionEngine, CompletionHistory, CompletionList, CompletionPreview,
    CompletionPriority, CompletionScore, CompletionTriggerChar, ExpectedTypeInfo,
    ImportedCompletion, KeywordEntry, MultiEditCompletion, PostfixTemplate, SimpleTextEdit,
    SmartCompletionProvider, TacticEntry, TypeExpectationSource,
};

/// Check if a byte is an identifier character.
pub fn is_ident_char(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_' || b == b'.' || b == b'\''
}
/// Command-level keywords.
pub const COMMAND_KEYWORDS: &[&str] = &[
    "def",
    "theorem",
    "lemma",
    "axiom",
    "inductive",
    "structure",
    "class",
    "instance",
    "namespace",
    "section",
    "end",
    "variable",
    "open",
    "import",
    "export",
    "set_option",
    "attribute",
    "#check",
    "#eval",
    "#print",
];
/// All language keywords with documentation.
const KEYWORDS: &[KeywordEntry] = &[
    KeywordEntry {
        keyword: "def",
        doc: "Define a function or value",
        is_command: true,
    },
    KeywordEntry {
        keyword: "theorem",
        doc: "State and prove a proposition",
        is_command: true,
    },
    KeywordEntry {
        keyword: "lemma",
        doc: "Alias for theorem",
        is_command: true,
    },
    KeywordEntry {
        keyword: "axiom",
        doc: "Postulate a type without proof",
        is_command: true,
    },
    KeywordEntry {
        keyword: "inductive",
        doc: "Define an inductive data type",
        is_command: true,
    },
    KeywordEntry {
        keyword: "structure",
        doc: "Define a record type with named fields",
        is_command: true,
    },
    KeywordEntry {
        keyword: "class",
        doc: "Define a type class",
        is_command: true,
    },
    KeywordEntry {
        keyword: "instance",
        doc: "Provide a type class instance",
        is_command: true,
    },
    KeywordEntry {
        keyword: "namespace",
        doc: "Open a namespace scope",
        is_command: true,
    },
    KeywordEntry {
        keyword: "section",
        doc: "Begin a section for local variables",
        is_command: true,
    },
    KeywordEntry {
        keyword: "end",
        doc: "End a namespace or section",
        is_command: true,
    },
    KeywordEntry {
        keyword: "variable",
        doc: "Declare a section variable",
        is_command: true,
    },
    KeywordEntry {
        keyword: "open",
        doc: "Open a namespace for unqualified access",
        is_command: true,
    },
    KeywordEntry {
        keyword: "import",
        doc: "Import definitions from another module",
        is_command: true,
    },
    KeywordEntry {
        keyword: "export",
        doc: "Re-export names from a namespace",
        is_command: true,
    },
    KeywordEntry {
        keyword: "set_option",
        doc: "Set a configuration option",
        is_command: true,
    },
    KeywordEntry {
        keyword: "attribute",
        doc: "Apply an attribute to a declaration",
        is_command: true,
    },
    KeywordEntry {
        keyword: "#check",
        doc: "Check the type of an expression",
        is_command: true,
    },
    KeywordEntry {
        keyword: "#eval",
        doc: "Evaluate an expression",
        is_command: true,
    },
    KeywordEntry {
        keyword: "#print",
        doc: "Print information about a declaration",
        is_command: true,
    },
    KeywordEntry {
        keyword: "where",
        doc: "Introduce local definitions",
        is_command: false,
    },
    KeywordEntry {
        keyword: "let",
        doc: "Local binding",
        is_command: false,
    },
    KeywordEntry {
        keyword: "in",
        doc: "Body of a let expression",
        is_command: false,
    },
    KeywordEntry {
        keyword: "fun",
        doc: "Lambda abstraction",
        is_command: false,
    },
    KeywordEntry {
        keyword: "forall",
        doc: "Universal quantifier / dependent function type",
        is_command: false,
    },
    KeywordEntry {
        keyword: "match",
        doc: "Pattern matching",
        is_command: false,
    },
    KeywordEntry {
        keyword: "with",
        doc: "Begin match arms or class body",
        is_command: false,
    },
    KeywordEntry {
        keyword: "if",
        doc: "Conditional expression",
        is_command: false,
    },
    KeywordEntry {
        keyword: "then",
        doc: "True branch of if",
        is_command: false,
    },
    KeywordEntry {
        keyword: "else",
        doc: "False branch of if",
        is_command: false,
    },
    KeywordEntry {
        keyword: "do",
        doc: "Do-notation for monadic code",
        is_command: false,
    },
    KeywordEntry {
        keyword: "have",
        doc: "Introduce a local hypothesis",
        is_command: false,
    },
    KeywordEntry {
        keyword: "show",
        doc: "Annotate the expected type",
        is_command: false,
    },
    KeywordEntry {
        keyword: "by",
        doc: "Enter tactic mode",
        is_command: false,
    },
    KeywordEntry {
        keyword: "sorry",
        doc: "Placeholder for incomplete proof",
        is_command: false,
    },
    KeywordEntry {
        keyword: "Prop",
        doc: "The type of propositions (Sort 0)",
        is_command: false,
    },
    KeywordEntry {
        keyword: "Type",
        doc: "The type of types (Sort 1)",
        is_command: false,
    },
    KeywordEntry {
        keyword: "Sort",
        doc: "A universe sort",
        is_command: false,
    },
    KeywordEntry {
        keyword: "return",
        doc: "Return a value in do-notation",
        is_command: false,
    },
    KeywordEntry {
        keyword: "true",
        doc: "Boolean true",
        is_command: false,
    },
    KeywordEntry {
        keyword: "false",
        doc: "Boolean false",
        is_command: false,
    },
];
/// Get keyword completions filtered by prefix.
pub fn keyword_completions(prefix: &str) -> Vec<CompletionItem> {
    let mut items = Vec::new();
    for entry in KEYWORDS {
        if prefix.is_empty() || entry.keyword.starts_with(prefix) {
            items.push(CompletionItem {
                label: entry.keyword.to_string(),
                kind: CompletionItemKind::Keyword,
                detail: Some(entry.doc.to_string()),
                documentation: Some(MarkupContent::plain(entry.doc)),
                insert_text: None,
                sort_text: None,
            });
        }
    }
    items
}
/// All known tactics with descriptions.
const TACTICS: &[TacticEntry] = &[
    TacticEntry {
        name: "intro",
        doc: "Introduce a hypothesis",
    },
    TacticEntry {
        name: "intros",
        doc: "Introduce multiple hypotheses",
    },
    TacticEntry {
        name: "apply",
        doc: "Apply a lemma to the goal",
    },
    TacticEntry {
        name: "exact",
        doc: "Provide an exact proof term",
    },
    TacticEntry {
        name: "rfl",
        doc: "Prove by reflexivity",
    },
    TacticEntry {
        name: "rw",
        doc: "Rewrite using an equation",
    },
    TacticEntry {
        name: "simp",
        doc: "Simplify using lemma set",
    },
    TacticEntry {
        name: "ring",
        doc: "Prove ring equalities",
    },
    TacticEntry {
        name: "omega",
        doc: "Solve linear arithmetic",
    },
    TacticEntry {
        name: "linarith",
        doc: "Linear arithmetic decision procedure",
    },
    TacticEntry {
        name: "norm_num",
        doc: "Normalize numeric expressions",
    },
    TacticEntry {
        name: "cases",
        doc: "Case analysis on inductive type",
    },
    TacticEntry {
        name: "induction",
        doc: "Perform induction on a term",
    },
    TacticEntry {
        name: "constructor",
        doc: "Apply a constructor",
    },
    TacticEntry {
        name: "left",
        doc: "Choose the left disjunct",
    },
    TacticEntry {
        name: "right",
        doc: "Choose the right disjunct",
    },
    TacticEntry {
        name: "assumption",
        doc: "Close goal with a hypothesis",
    },
    TacticEntry {
        name: "contradiction",
        doc: "Close goal by finding a contradiction",
    },
    TacticEntry {
        name: "trivial",
        doc: "Solve trivial goals",
    },
    TacticEntry {
        name: "sorry",
        doc: "Admit the current goal (unsound)",
    },
    TacticEntry {
        name: "have",
        doc: "Introduce an intermediate lemma",
    },
    TacticEntry {
        name: "let",
        doc: "Introduce a local definition",
    },
    TacticEntry {
        name: "show",
        doc: "Change the goal to a definitionally equal one",
    },
    TacticEntry {
        name: "calc",
        doc: "Begin a calculational proof",
    },
    TacticEntry {
        name: "suffices",
        doc: "It suffices to show a simpler goal",
    },
    TacticEntry {
        name: "obtain",
        doc: "Destructure a hypothesis",
    },
    TacticEntry {
        name: "rcases",
        doc: "Recursive cases on a hypothesis",
    },
    TacticEntry {
        name: "ext",
        doc: "Apply extensionality",
    },
    TacticEntry {
        name: "funext",
        doc: "Apply function extensionality",
    },
    TacticEntry {
        name: "congr",
        doc: "Apply congruence",
    },
    TacticEntry {
        name: "decide",
        doc: "Decide a decidable proposition",
    },
    TacticEntry {
        name: "specialize",
        doc: "Specialize a hypothesis",
    },
    TacticEntry {
        name: "revert",
        doc: "Move a hypothesis into the goal",
    },
    TacticEntry {
        name: "clear",
        doc: "Remove a hypothesis",
    },
    TacticEntry {
        name: "rename_i",
        doc: "Rename inaccessible names",
    },
    TacticEntry {
        name: "repeat",
        doc: "Repeat a tactic",
    },
    TacticEntry {
        name: "first",
        doc: "Try tactics in order",
    },
    TacticEntry {
        name: "all_goals",
        doc: "Apply tactic to all goals",
    },
    TacticEntry {
        name: "any_goals",
        doc: "Apply tactic to any goal",
    },
    TacticEntry {
        name: "focus",
        doc: "Focus on the first goal",
    },
];
/// Get tactic completions filtered by prefix.
pub fn tactic_completions(prefix: &str) -> Vec<CompletionItem> {
    let mut items = Vec::new();
    for entry in TACTICS {
        if prefix.is_empty() || entry.name.starts_with(prefix) {
            items.push(CompletionItem {
                label: entry.name.to_string(),
                kind: CompletionItemKind::Method,
                detail: Some(format!("tactic: {}", entry.doc)),
                documentation: Some(MarkupContent::plain(entry.doc)),
                insert_text: None,
                sort_text: None,
            });
        }
    }
    items
}
/// Get completions for tactic arguments (e.g., hypothesis names).
pub fn tactic_argument_completions(
    doc: &Document,
    env: &Environment,
    prefix: &str,
) -> Vec<CompletionItem> {
    let mut items = Vec::new();
    let analysis = analyze_document(&doc.uri, &doc.content, env);
    for def in &analysis.definitions {
        if prefix.is_empty() || def.name.starts_with(prefix) {
            items.push(CompletionItem::variable(
                &def.name,
                def.ty.as_deref().unwrap_or("_"),
            ));
        }
    }
    for name in env.constant_names() {
        let name_str = name.to_string();
        if prefix.is_empty() || name_str.starts_with(prefix) {
            items.push(CompletionItem::function(&name_str, "declaration"));
        }
    }
    items
}
/// Get snippet completions filtered by prefix.
pub fn snippet_completions(prefix: &str) -> Vec<CompletionItem> {
    let mut items = Vec::new();
    if prefix.is_empty() || "def".starts_with(prefix) {
        items.push(def_snippet());
    }
    if prefix.is_empty() || "theorem".starts_with(prefix) {
        items.push(theorem_snippet());
    }
    if prefix.is_empty() || "match".starts_with(prefix) {
        items.push(match_snippet());
    }
    if prefix.is_empty() || "inductive".starts_with(prefix) {
        items.push(inductive_snippet());
    }
    if prefix.is_empty() || "structure".starts_with(prefix) {
        items.push(structure_snippet());
    }
    if prefix.is_empty() || "class".starts_with(prefix) {
        items.push(class_snippet());
    }
    if prefix.is_empty() || "instance".starts_with(prefix) {
        items.push(instance_snippet());
    }
    if prefix.is_empty() || "let".starts_with(prefix) {
        items.push(let_snippet());
    }
    if prefix.is_empty() || "have".starts_with(prefix) {
        items.push(have_snippet());
    }
    if prefix.is_empty() || "if".starts_with(prefix) {
        items.push(if_snippet());
    }
    items
}
/// Snippet for a `def` declaration.
pub fn def_snippet() -> CompletionItem {
    CompletionItem::snippet(
        "def ...",
        "def ${1:name} : ${2:Type} := ${0:sorry}",
        "definition template",
    )
}
/// Snippet for a `theorem` declaration.
pub fn theorem_snippet() -> CompletionItem {
    CompletionItem::snippet(
        "theorem ...",
        "theorem ${1:name} : ${2:Prop} := by\n  ${0:sorry}",
        "theorem template",
    )
}
/// Snippet for a `match` expression.
pub fn match_snippet() -> CompletionItem {
    CompletionItem::snippet(
        "match ...",
        "match ${1:x} with\n  | ${2:pattern} => ${0:body}",
        "match expression template",
    )
}
/// Snippet for an `inductive` type.
pub fn inductive_snippet() -> CompletionItem {
    CompletionItem::snippet(
        "inductive ...",
        "inductive ${1:Name} where\n  | ${2:ctor} : ${0:Name}",
        "inductive type template",
    )
}
/// Snippet for a `structure` declaration.
pub fn structure_snippet() -> CompletionItem {
    CompletionItem::snippet(
        "structure ...",
        "structure ${1:Name} where\n  ${2:field} : ${0:Type}",
        "structure template",
    )
}
/// Snippet for a `class` declaration.
fn class_snippet() -> CompletionItem {
    CompletionItem::snippet(
        "class ...",
        "class ${1:Name} (${2:a} : ${3:Type}) where\n  ${4:method} : ${0:Type}",
        "type class template",
    )
}
/// Snippet for an `instance` declaration.
fn instance_snippet() -> CompletionItem {
    CompletionItem::snippet(
        "instance ...",
        "instance : ${1:Class} ${2:Type} where\n  ${3:method} := ${0:sorry}",
        "instance template",
    )
}
/// Snippet for a `let` binding.
fn let_snippet() -> CompletionItem {
    CompletionItem::snippet(
        "let ...",
        "let ${1:x} := ${2:value}\n${0}",
        "let binding template",
    )
}
/// Snippet for a `have` statement.
fn have_snippet() -> CompletionItem {
    CompletionItem::snippet(
        "have ...",
        "have ${1:h} : ${2:Prop} := ${0:sorry}",
        "have hypothesis template",
    )
}
/// Snippet for an `if` expression.
fn if_snippet() -> CompletionItem {
    CompletionItem::snippet(
        "if ...",
        "if ${1:cond} then\n  ${2:a}\nelse\n  ${0:b}",
        "if-then-else template",
    )
}
#[cfg(test)]
mod tests {
    use super::*;
    fn make_doc(content: &str) -> Document {
        Document::new("file:///test.lean", 1, content)
    }
    #[test]
    fn test_completion_list_empty() {
        let list = CompletionList::empty();
        assert!(list.items.is_empty());
        assert!(!list.is_incomplete);
    }
    #[test]
    fn test_completion_list_new() {
        let items = vec![CompletionItem::keyword("def")];
        let list = CompletionList::new(items, true);
        assert_eq!(list.items.len(), 1);
        assert!(list.is_incomplete);
    }
    #[test]
    fn test_keyword_completions_empty_prefix() {
        let items = keyword_completions("");
        assert!(items.len() >= 30);
    }
    #[test]
    fn test_keyword_completions_with_prefix() {
        let items = keyword_completions("th");
        let labels: Vec<&str> = items.iter().map(|i| i.label.as_str()).collect();
        assert!(labels.contains(&"theorem"));
        assert!(labels.contains(&"then"));
        assert!(!labels.contains(&"def"));
    }
    #[test]
    fn test_keyword_completions_no_match() {
        let items = keyword_completions("zzzzz");
        assert!(items.is_empty());
    }
    #[test]
    fn test_tactic_completions_empty_prefix() {
        let items = tactic_completions("");
        assert!(items.len() >= 30);
    }
    #[test]
    fn test_tactic_completions_prefix() {
        let items = tactic_completions("in");
        let labels: Vec<&str> = items.iter().map(|i| i.label.as_str()).collect();
        assert!(labels.contains(&"intro"));
        assert!(labels.contains(&"intros"));
        assert!(labels.contains(&"induction"));
        assert!(!labels.contains(&"apply"));
    }
    #[test]
    fn test_snippet_completions_empty() {
        let items = snippet_completions("");
        assert!(items.len() >= 8);
    }
    #[test]
    fn test_snippet_completions_def() {
        let items = snippet_completions("de");
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].label, "def ...");
    }
    #[test]
    fn test_def_snippet_content() {
        let s = def_snippet();
        assert!(s
            .insert_text
            .as_ref()
            .expect("type conversion should succeed")
            .contains("${1:name}"));
    }
    #[test]
    fn test_theorem_snippet_content() {
        let s = theorem_snippet();
        assert!(s
            .insert_text
            .as_ref()
            .expect("type conversion should succeed")
            .contains("sorry"));
    }
    #[test]
    fn test_match_snippet_content() {
        let s = match_snippet();
        assert!(s
            .insert_text
            .as_ref()
            .expect("type conversion should succeed")
            .contains("pattern"));
    }
    #[test]
    fn test_inductive_snippet_content() {
        let s = inductive_snippet();
        assert!(s
            .insert_text
            .as_ref()
            .expect("type conversion should succeed")
            .contains("ctor"));
    }
    #[test]
    fn test_structure_snippet_content() {
        let s = structure_snippet();
        assert!(s
            .insert_text
            .as_ref()
            .expect("type conversion should succeed")
            .contains("field"));
    }
    #[test]
    fn test_engine_extract_prefix() {
        let env = Environment::new();
        let engine = CompletionEngine::new(&env);
        let doc = make_doc("def foo");
        let prefix = engine.extract_prefix(&doc, &Position::new(0, 7));
        assert_eq!(prefix, "foo");
    }
    #[test]
    fn test_engine_extract_prefix_empty() {
        let env = Environment::new();
        let engine = CompletionEngine::new(&env);
        let doc = make_doc("def ");
        let prefix = engine.extract_prefix(&doc, &Position::new(0, 4));
        assert_eq!(prefix, "");
    }
    #[test]
    fn test_determine_context_import() {
        let env = Environment::new();
        let engine = CompletionEngine::new(&env);
        let doc = make_doc("import Init");
        let ctx = engine.determine_context(&doc, &Position::new(0, 11));
        assert_eq!(ctx, CompletionContext::InImport);
    }
    #[test]
    fn test_determine_context_command() {
        let env = Environment::new();
        let engine = CompletionEngine::new(&env);
        let doc = make_doc("");
        let ctx = engine.determine_context(&doc, &Position::new(0, 0));
        assert_eq!(ctx, CompletionContext::InCommand);
    }
    #[test]
    fn test_determine_context_trigger_dot() {
        let env = Environment::new();
        let engine = CompletionEngine::new(&env);
        let doc = make_doc("x.");
        let ctx = engine.determine_context(&doc, &Position::new(0, 2));
        assert_eq!(ctx, CompletionContext::TriggerCharacter('.'));
    }
    #[test]
    fn test_determine_context_in_type() {
        let env = Environment::new();
        let engine = CompletionEngine::new(&env);
        let doc = make_doc("def foo : Na");
        let ctx = engine.determine_context(&doc, &Position::new(0, 12));
        assert_eq!(ctx, CompletionContext::InType);
    }
    #[test]
    fn test_complete_at_position_basic() {
        let env = Environment::new();
        let mut engine = CompletionEngine::new(&env);
        let doc = make_doc("th");
        let list = engine.complete_at_position(&doc, &Position::new(0, 2));
        let labels: Vec<&str> = list.items.iter().map(|i| i.label.as_str()).collect();
        assert!(labels.contains(&"theorem"));
    }
    #[test]
    fn test_score_completion_exact_prefix() {
        let env = Environment::new();
        let engine = CompletionEngine::new(&env);
        let item = CompletionItem::keyword("theorem");
        let score = engine.score_completion(&item, "th", &CompletionContext::InCommand);
        assert!(score > 100);
    }
    #[test]
    fn test_constructor_completions_empty_env() {
        let env = Environment::new();
        let engine = CompletionEngine::new(&env);
        let items = engine.constructor_completions("");
        assert!(items.is_empty());
    }
    #[test]
    fn test_tactic_argument_completions_empty_doc() {
        let env = Environment::new();
        let doc = make_doc("");
        let items = tactic_argument_completions(&doc, &env, "");
        assert!(items.is_empty());
    }
    #[test]
    fn test_field_completions_after_dot() {
        let env = Environment::new();
        let engine = CompletionEngine::new(&env);
        let doc = make_doc("x.");
        let items = engine.field_completions(&doc, &Position::new(0, 2));
        assert!(!items.is_empty());
    }
    #[test]
    fn test_module_completions() {
        let env = Environment::new();
        let engine = CompletionEngine::new(&env);
        let items = engine.module_completions("Init");
        assert!(!items.is_empty());
        assert!(items.iter().any(|i| i.label == "Init"));
    }
}
/// Score a completion label against a prefix using a simple priority model.
pub fn priority_score(label: &str, prefix: &str) -> CompletionPriority {
    if label == prefix {
        CompletionPriority::Exact
    } else if label.starts_with(prefix) {
        CompletionPriority::High
    } else if label.to_lowercase().starts_with(&prefix.to_lowercase()) {
        CompletionPriority::Medium
    } else {
        CompletionPriority::Low
    }
}
/// Extract the enclosing namespace from a document at a position.
pub fn extract_enclosing_namespace(doc: &Document, pos: &Position) -> Option<String> {
    let target_line = pos.line as usize;
    let mut stack: Vec<String> = Vec::new();
    for (i, line) in doc.content.lines().enumerate() {
        if i > target_line {
            break;
        }
        let trimmed = line.trim();
        if let Some(ns) = trimmed.strip_prefix("namespace ") {
            let name: String = ns
                .chars()
                .take_while(|c| c.is_alphanumeric() || *c == '_' || *c == '.')
                .collect();
            if !name.is_empty() {
                stack.push(name);
            }
        } else if trimmed.starts_with("end") && !stack.is_empty() {
            stack.pop();
        }
    }
    if stack.is_empty() {
        None
    } else {
        Some(stack.join("."))
    }
}
/// Generate namespace-qualified completions.
pub fn namespace_completions(env: &Environment, prefix: &str) -> Vec<CompletionItem> {
    let mut items = Vec::new();
    if let Some(dot) = prefix.rfind('.') {
        let ns = &prefix[..dot];
        let name_prefix = &prefix[dot + 1..];
        for name in env.constant_names() {
            let name_str = name.to_string();
            if let Some(rest) = name_str.strip_prefix(&format!("{}.", ns)) {
                if rest.starts_with(name_prefix) {
                    items.push(CompletionItem {
                        label: rest.to_string(),
                        kind: CompletionItemKind::Function,
                        detail: Some(format!("{}.{}", ns, rest)),
                        documentation: None,
                        insert_text: None,
                        sort_text: Some(priority_score(rest, name_prefix).sort_key()),
                    });
                }
            }
        }
    } else {
        for name in env.constant_names() {
            let name_str = name.to_string();
            if name_str.starts_with(prefix) {
                items.push(CompletionItem {
                    label: name_str,
                    kind: CompletionItemKind::Function,
                    detail: None,
                    documentation: None,
                    insert_text: None,
                    sort_text: None,
                });
            }
        }
    }
    items
}
/// Check if a document already imports a module.
pub fn document_has_import(doc: &Document, module: &str) -> bool {
    doc.content.lines().any(|line| {
        let t = line.trim();
        t == format!("import {}", module) || t.starts_with(&format!("import {} ", module))
    })
}
/// Compute import-adding completions based on a known module registry.
pub fn import_adding_completions(doc: &Document, prefix: &str) -> Vec<ImportedCompletion> {
    let mut items = Vec::new();
    let registry: &[(&str, &str, CompletionItemKind)] = &[
        ("Nat.add", "Init.Prelude", CompletionItemKind::Function),
        ("Nat.mul", "Init.Prelude", CompletionItemKind::Function),
        (
            "List.map",
            "Init.Data.List.Basic",
            CompletionItemKind::Function,
        ),
        (
            "List.filter",
            "Init.Data.List.Basic",
            CompletionItemKind::Function,
        ),
        (
            "Option.map",
            "Init.Data.Option.Basic",
            CompletionItemKind::Function,
        ),
        (
            "String.length",
            "Init.Data.String.Basic",
            CompletionItemKind::Function,
        ),
        ("IO.println", "Init.System.IO", CompletionItemKind::Function),
        ("decide", "Init.Decide", CompletionItemKind::Function),
        (
            "Finset",
            "Mathlib.Data.Finset.Basic",
            CompletionItemKind::Class,
        ),
        (
            "Ring",
            "Mathlib.Algebra.Ring.Basic",
            CompletionItemKind::Class,
        ),
    ];
    for &(name, module, kind) in registry {
        if prefix.is_empty() || name.starts_with(prefix) {
            let already = document_has_import(doc, module);
            let mut item = ImportedCompletion::new(name, module, kind);
            item.already_imported = already;
            items.push(item);
        }
    }
    items
}
/// The set of characters that trigger auto-completion.
pub const TRIGGER_CHARACTERS: &[char] = &['.', ':', ' ', '#', '(', '@'];
/// Check if a character is a completion trigger.
pub fn is_trigger_character(ch: char) -> bool {
    TRIGGER_CHARACTERS.contains(&ch)
}
/// Completions triggered by `#` (hash commands).
pub fn hash_command_completions() -> Vec<CompletionItem> {
    let commands: &[(&str, &str)] = &[
        ("#check", "Check the type of an expression"),
        ("#eval", "Evaluate an expression"),
        ("#print", "Print information about a declaration"),
        ("#reduce", "Reduce an expression to normal form"),
        ("#whnf", "Reduce to weak head normal form"),
        ("#check_failure", "Assert that a check fails"),
    ];
    commands
        .iter()
        .map(|&(cmd, doc)| CompletionItem {
            label: cmd.to_string(),
            kind: CompletionItemKind::Keyword,
            detail: Some(doc.to_string()),
            documentation: Some(MarkupContent::plain(doc)),
            insert_text: None,
            sort_text: None,
        })
        .collect()
}
/// Try to infer the expected type at a cursor position from surrounding context.
pub fn infer_expected_type(doc: &Document, pos: &Position) -> Option<ExpectedTypeInfo> {
    if let Some(line) = doc.get_line(pos.line) {
        let col = pos.character as usize;
        let before = &line[..col.min(line.len())];
        if let Some(colon_pos) = before.rfind(':') {
            let after_colon = before[colon_pos + 1..].trim_start();
            if !after_colon.is_empty() && !after_colon.contains(":=") {
                return Some(ExpectedTypeInfo {
                    ty: after_colon.to_string(),
                    confidence: 0.8,
                    source: TypeExpectationSource::Annotation,
                });
            }
        }
    }
    None
}
/// Built-in postfix templates.
pub fn postfix_templates() -> Vec<PostfixTemplate> {
    vec![
        PostfixTemplate {
            trigger: ".map".to_string(),
            template: "List.map (fun x -> x) $EXPR".to_string(),
            description: "Map a function over a list".to_string(),
        },
        PostfixTemplate {
            trigger: ".filter".to_string(),
            template: "List.filter (fun x -> true) $EXPR".to_string(),
            description: "Filter a list with a predicate".to_string(),
        },
        PostfixTemplate {
            trigger: ".length".to_string(),
            template: "List.length $EXPR".to_string(),
            description: "Get the length of a list".to_string(),
        },
        PostfixTemplate {
            trigger: ".some".to_string(),
            template: "Option.some $EXPR".to_string(),
            description: "Wrap in Option.some".to_string(),
        },
        PostfixTemplate {
            trigger: ".not".to_string(),
            template: "Not $EXPR".to_string(),
            description: "Negate a proposition".to_string(),
        },
        PostfixTemplate {
            trigger: ".match".to_string(),
            template: "match $EXPR with\n  | _ => _".to_string(),
            description: "Match on expression".to_string(),
        },
    ]
}
/// Compute postfix completions for the expression before the cursor.
pub fn postfix_completions(doc: &Document, pos: &Position) -> Vec<CompletionItem> {
    let mut items = Vec::new();
    if let Some(line) = doc.get_line(pos.line) {
        let col = (pos.character as usize).min(line.len());
        let before = &line[..col];
        if let Some(dot_pos) = before.rfind('.') {
            let suffix = &before[dot_pos..];
            let base_expr = before[..dot_pos].trim().to_string();
            let templates = postfix_templates();
            for tmpl in &templates {
                if tmpl.trigger.starts_with(suffix) || suffix == "." {
                    let expanded = tmpl.expand(&base_expr);
                    items.push(CompletionItem {
                        label: tmpl.trigger.trim_start_matches('.').to_string(),
                        kind: CompletionItemKind::Method,
                        detail: Some(tmpl.description.clone()),
                        documentation: Some(MarkupContent::plain(&format!(
                            "{}\n\nExpands to:\n```lean\n{}\n```",
                            tmpl.description, expanded
                        ))),
                        insert_text: Some(tmpl.trigger.trim_start_matches('.').to_string()),
                        sort_text: None,
                    });
                }
            }
        }
    }
    items
}
/// Compute a fuzzy match score for a query against a candidate label.
pub fn fuzzy_score(query: &str, candidate: &str) -> u32 {
    if query.is_empty() {
        return 50;
    }
    if candidate == query {
        return 100;
    }
    if candidate.starts_with(query) {
        return 90;
    }
    let query_lower = query.to_lowercase();
    let cand_lower = candidate.to_lowercase();
    if cand_lower.starts_with(&query_lower) {
        return 80;
    }
    if is_subsequence(query, candidate) {
        let score = 60u32.saturating_sub(candidate.len().saturating_sub(query.len()) as u32);
        return score.max(1);
    }
    0
}
/// Check if `query` is a subsequence of `candidate` (case-insensitive).
pub fn is_subsequence(query: &str, candidate: &str) -> bool {
    let mut qchars = query.chars().map(|c| c.to_lowercase().next().unwrap_or(c));
    let mut qc = qchars.next();
    for cc in candidate
        .chars()
        .map(|c| c.to_lowercase().next().unwrap_or(c))
    {
        if qc == Some(cc) {
            qc = qchars.next();
        }
        if qc.is_none() {
            return true;
        }
    }
    qc.is_none()
}
/// Sort a list of completion items by fuzzy score against a query.
pub fn fuzzy_filter_and_sort(items: Vec<CompletionItem>, query: &str) -> Vec<CompletionItem> {
    let mut scored: Vec<(u32, CompletionItem)> = items
        .into_iter()
        .filter_map(|item| {
            let score = fuzzy_score(query, &item.label);
            if score > 0 {
                Some((score, item))
            } else {
                None
            }
        })
        .collect();
    scored.sort_by_key(|b| std::cmp::Reverse(b.0));
    scored.into_iter().map(|(_, item)| item).collect()
}
/// Generate completion previews for a list of items.
pub fn attach_previews(items: &mut Vec<CompletionItem>, env: &Environment) {
    for item in items.iter_mut() {
        if item.documentation.is_some() {
            continue;
        }
        let name = oxilean_kernel::Name::str(&item.label);
        if let Some(ci) = env.find(&name) {
            let ty = format!("{:?}", ci.ty());
            let preview = CompletionPreview::from_env(&item.label, &ty);
            item.documentation = Some(MarkupContent::markdown(&preview.to_markdown()));
        }
    }
}
#[cfg(test)]
mod completion_new_tests {
    use super::*;
    fn make_doc(content: &str) -> Document {
        Document::new("file:///test.lean", 1, content)
    }
    #[test]
    fn test_priority_score_exact() {
        assert_eq!(priority_score("def", "def"), CompletionPriority::Exact);
    }
    #[test]
    fn test_priority_score_high() {
        assert_eq!(
            priority_score("definition", "def"),
            CompletionPriority::High
        );
    }
    #[test]
    fn test_priority_score_medium() {
        assert_eq!(priority_score("DEF", "def"), CompletionPriority::Medium);
    }
    #[test]
    fn test_priority_score_low() {
        assert_eq!(priority_score("xyz", "def"), CompletionPriority::Low);
    }
    #[test]
    fn test_sort_key_order() {
        assert!(CompletionPriority::Exact.sort_key() < CompletionPriority::High.sort_key());
        assert!(CompletionPriority::High.sort_key() < CompletionPriority::Medium.sort_key());
    }
    #[test]
    fn test_extract_enclosing_namespace_none() {
        let doc = make_doc("def foo := 1");
        assert!(extract_enclosing_namespace(&doc, &Position::new(0, 0)).is_none());
    }
    #[test]
    fn test_extract_enclosing_namespace_found() {
        let doc = make_doc("namespace Foo\ndef bar := 1\n");
        let ns = extract_enclosing_namespace(&doc, &Position::new(1, 0));
        assert_eq!(ns.as_deref(), Some("Foo"));
    }
    #[test]
    fn test_namespace_completions_empty_env() {
        let env = Environment::new();
        let items = namespace_completions(&env, "Nat.");
        assert!(items.is_empty());
    }
    #[test]
    fn test_imported_completion_new() {
        let ic = ImportedCompletion::new(
            "List.map",
            "Init.Data.List.Basic",
            CompletionItemKind::Function,
        );
        assert_eq!(ic.label, "List.map");
        assert!(!ic.already_imported);
    }
    #[test]
    fn test_imported_completion_to_item() {
        let ic = ImportedCompletion::new(
            "List.map",
            "Init.Data.List.Basic",
            CompletionItemKind::Function,
        );
        let item = ic.to_completion_item();
        assert!(item
            .detail
            .as_ref()
            .expect("type conversion should succeed")
            .contains("adds import"));
    }
    #[test]
    fn test_document_has_import_yes() {
        let doc = make_doc("import Init.Prelude\ndef foo := 1");
        assert!(document_has_import(&doc, "Init.Prelude"));
    }
    #[test]
    fn test_document_has_import_no() {
        let doc = make_doc("def foo := 1");
        assert!(!document_has_import(&doc, "Init.Prelude"));
    }
    #[test]
    fn test_import_adding_completions() {
        let doc = make_doc("def foo := 1");
        let items = import_adding_completions(&doc, "List");
        assert!(!items.is_empty());
    }
    #[test]
    fn test_is_trigger_character() {
        assert!(is_trigger_character('.'));
        assert!(!is_trigger_character('a'));
    }
    #[test]
    fn test_hash_command_completions() {
        let items = hash_command_completions();
        assert!(items.iter().any(|i| i.label == "#check"));
    }
    #[test]
    fn test_infer_expected_type_annotation() {
        let doc = make_doc("def foo : Nat");
        let info = infer_expected_type(&doc, &Position::new(0, 13));
        assert!(info.is_some());
    }
    #[test]
    fn test_type_expectation_source_eq() {
        assert_eq!(
            TypeExpectationSource::Annotation,
            TypeExpectationSource::Annotation
        );
        assert_ne!(
            TypeExpectationSource::Annotation,
            TypeExpectationSource::Application
        );
    }
    #[test]
    fn test_completion_history_record_rank() {
        let mut hist = CompletionHistory::new(10);
        hist.record("intro");
        hist.record("apply");
        assert_eq!(hist.rank("apply"), Some(0));
        assert_eq!(hist.rank("intro"), Some(1));
    }
    #[test]
    fn test_completion_history_dedup() {
        let mut hist = CompletionHistory::new(10);
        hist.record("intro");
        hist.record("apply");
        hist.record("intro");
        assert_eq!(hist.len(), 2);
        assert_eq!(hist.rank("intro"), Some(0));
    }
    #[test]
    fn test_completion_history_max_size() {
        let mut hist = CompletionHistory::new(3);
        hist.record("a");
        hist.record("b");
        hist.record("c");
        hist.record("d");
        assert_eq!(hist.len(), 3);
        assert!(hist.rank("a").is_none());
    }
    #[test]
    fn test_completion_history_boost_items() {
        let mut hist = CompletionHistory::new(10);
        hist.record("exact");
        let mut items = vec![CompletionItem::keyword("exact")];
        hist.boost_items(&mut items);
        assert!(items[0].sort_text.is_some());
    }
    #[test]
    fn test_completion_history_clear() {
        let mut hist = CompletionHistory::new(10);
        hist.record("intro");
        hist.clear();
        assert!(hist.is_empty());
    }
    #[test]
    fn test_simple_text_edit() {
        let edit = SimpleTextEdit {
            range: Range::single_line(0, 0, 3),
            new_text: "def".to_string(),
        };
        assert_eq!(edit.new_text, "def");
    }
    #[test]
    fn test_multi_edit_completion_new() {
        let edit = SimpleTextEdit {
            range: Range::single_line(1, 0, 0),
            new_text: "hello".to_string(),
        };
        let mc = MultiEditCompletion::new("hello", CompletionItemKind::Function, vec![edit]);
        assert_eq!(mc.label, "hello");
        assert_eq!(mc.edits.len(), 1);
    }
    #[test]
    fn test_multi_edit_completion_with_import() {
        let mc = MultiEditCompletion::with_import(
            "List.map",
            "Init.Data.List.Basic",
            Range::single_line(5, 0, 0),
        );
        assert_eq!(mc.edits.len(), 2);
        assert!(mc.edits[0].new_text.contains("import"));
    }
    #[test]
    fn test_postfix_template_expand() {
        let tmpl = PostfixTemplate {
            trigger: ".map".to_string(),
            template: "List.map f $EXPR".to_string(),
            description: "Map".to_string(),
        };
        assert_eq!(tmpl.expand("xs"), "List.map f xs");
    }
    #[test]
    fn test_postfix_templates_nonempty() {
        let templates = postfix_templates();
        assert!(!templates.is_empty());
        assert!(templates.iter().any(|t| t.trigger == ".map"));
    }
    #[test]
    fn test_postfix_completions_after_dot() {
        let doc = make_doc("myList.");
        let items = postfix_completions(&doc, &Position::new(0, 7));
        assert!(!items.is_empty());
    }
    #[test]
    fn test_postfix_completions_no_dot() {
        let doc = make_doc("myList");
        let items = postfix_completions(&doc, &Position::new(0, 6));
        assert!(items.is_empty());
    }
    #[test]
    fn test_smart_completion_nat() {
        let env = Environment::new();
        let provider = SmartCompletionProvider::new(&env);
        let items = provider.completions_for_type("Nat", "");
        assert!(!items.is_empty());
    }
    #[test]
    fn test_smart_completion_bool() {
        let env = Environment::new();
        let provider = SmartCompletionProvider::new(&env);
        let items = provider.completions_for_type("Bool", "");
        assert!(items.iter().any(|i| i.label == "true"));
    }
    #[test]
    fn test_smart_completion_prop() {
        let env = Environment::new();
        let provider = SmartCompletionProvider::new(&env);
        let items = provider.completions_for_type("Prop", "");
        assert!(items
            .iter()
            .any(|i| i.label == "True" || i.label == "False"));
    }
    #[test]
    fn test_fuzzy_score_exact() {
        assert_eq!(fuzzy_score("def", "def"), 100);
    }
    #[test]
    fn test_fuzzy_score_prefix() {
        assert!(fuzzy_score("def", "definition") >= 80);
    }
    #[test]
    fn test_fuzzy_score_subsequence() {
        assert!(fuzzy_score("dfn", "definition") > 0);
    }
    #[test]
    fn test_fuzzy_score_no_match() {
        assert_eq!(fuzzy_score("xyz", "abc"), 0);
    }
    #[test]
    fn test_is_subsequence_yes() {
        assert!(is_subsequence("tc", "tactic"));
    }
    #[test]
    fn test_is_subsequence_no() {
        assert!(!is_subsequence("xyz", "abc"));
    }
    #[test]
    fn test_fuzzy_filter_and_sort() {
        let items = vec![
            CompletionItem::keyword("definition"),
            CompletionItem::keyword("xyz_no_match_zzz"),
        ];
        let filtered = fuzzy_filter_and_sort(items, "def");
        assert_eq!(filtered.len(), 1);
    }
    #[test]
    fn test_completion_preview_keyword() {
        let p = CompletionPreview::keyword("def", "Define a function");
        assert_eq!(p.kind_label, "keyword");
    }
    #[test]
    fn test_completion_preview_tactic() {
        let p = CompletionPreview::tactic("intro", "Introduce a hypothesis");
        let md = p.to_markdown();
        assert!(md.contains("Tactic"));
    }
    #[test]
    fn test_completion_preview_from_env() {
        let p = CompletionPreview::from_env("foo", "Nat -> Nat");
        let md = p.to_markdown();
        assert!(md.contains("Nat"));
    }
    #[test]
    fn test_attach_previews_empty() {
        let env = Environment::new();
        let mut items: Vec<CompletionItem> = Vec::new();
        attach_previews(&mut items, &env);
        assert!(items.is_empty());
    }
}
#[allow(dead_code)]
pub fn rank_completions(mut scores: Vec<CompletionScore>) -> Vec<CompletionScore> {
    scores.sort_by(|a, b| {
        b.score
            .partial_cmp(&a.score)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    scores
}
#[allow(dead_code)]
pub fn filter_completions_by_prefix(items: &[String], prefix: &str) -> Vec<String> {
    items
        .iter()
        .filter(|s| s.starts_with(prefix))
        .cloned()
        .collect()
}
#[allow(dead_code)]
pub fn completion_item_label(raw: &str, detail: Option<&str>) -> String {
    match detail {
        Some(d) => format!("{} — {}", raw, d),
        None => raw.to_string(),
    }
}
#[allow(dead_code)]
pub fn standard_trigger_chars() -> Vec<CompletionTriggerChar> {
    vec![
        CompletionTriggerChar::new('.', "member access"),
        CompletionTriggerChar::new('#', "attribute"),
        CompletionTriggerChar::new('@', "decoration"),
        CompletionTriggerChar::new(' ', "whitespace after keyword"),
    ]
}
#[cfg(test)]
mod completion_extra_tests {
    use super::*;
    #[test]
    fn test_rank_completions() {
        let scores = vec![
            CompletionScore::new("foo", 0.3),
            CompletionScore::new("bar", 0.9),
            CompletionScore::new("baz", 0.6),
        ];
        let ranked = rank_completions(scores);
        assert_eq!(ranked[0].label, "bar");
        assert_eq!(ranked[2].label, "foo");
    }
    #[test]
    fn test_filter_by_prefix() {
        let items = vec!["foo".to_string(), "foobar".to_string(), "baz".to_string()];
        let result = filter_completions_by_prefix(&items, "foo");
        assert_eq!(result.len(), 2);
    }
    #[test]
    fn test_completion_item_label() {
        assert_eq!(completion_item_label("foo", Some("Nat")), "foo — Nat");
        assert_eq!(completion_item_label("bar", None), "bar");
    }
    #[test]
    fn test_standard_triggers() {
        let triggers = standard_trigger_chars();
        assert!(!triggers.is_empty());
        assert!(triggers.iter().any(|t| t.character == '.'));
    }
}
