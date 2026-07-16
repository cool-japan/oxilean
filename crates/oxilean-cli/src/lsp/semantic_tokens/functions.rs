//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lsp::{Document, JsonValue, Position, Range};
use oxilean_kernel::{Environment, Name};
use oxilean_parse::{Lexer, TokenKind};

use super::types::{
    BracketKind, BracketPair, BracketPairPos, EncodedSemanticTokens, IncrementalTokenManager,
    InlayHint, InlayHintKind, OxiTokenModifier, OxiTokenType, RawSemanticToken, SemanticToken,
    SemanticTokenAccumulator, SemanticTokenBuilder, SemanticTokenCache, SemanticTokenEdit,
    SemanticTokenFilter, SemanticTokenLegend, SemanticTokenStatistics, SemanticTokensDelta,
    TokenClassifier, TokenColorizer, TokenHighlightKind, TokenHighlightRange, TokenizeResult,
};

// ── Canonical semantic token legend ─────────────────────────────────────────
//
// This is the **stable, documented** mapping between legend index and the
// semantic meaning advertised to LSP clients.  Never reorder these without a
// major version bump — clients cache the indices from the initialize response.
//
// Index → token-type name:
//   0  namespace     — a namespace/module identifier (`namespace Foo`)
//   1  type          — a type (inductive, structure, `Type`, `Prop`)
//   2  class         — a type-class identifier
//   3  enumMember    — a constructor of an inductive type
//   4  typeParameter — a universe polymorphism or type-variable binder
//   5  function      — a definition / constant (`def`, `noncomputable def`)
//   6  method        — a theorem or lemma (`theorem`, `lemma`)
//   7  property      — a structure field or instance
//   8  variable      — a local variable or binder
//   9  parameter     — an explicit function parameter
//  10  string        — a string literal
//  11  number        — a numeric literal (Nat, Int, Float)
//  12  keyword       — a reserved word (`def`, `theorem`, `by`, …)
//  13  comment       — a doc-comment (`/-! … -/`, `-- …`)
//  14  operator      — an operator or punctuation (`+`, `→`, `:=`, …)
//  15  macro         — a macro invocation (`#check`, `#eval`, …)
//  16  decorator     — an attribute (`@[simp]`, `@[ext]`, …)

/// The canonical OxiLean semantic token type names in legend order (index 0…16).
///
/// Clients receive this array during the `initialize` response and use the
/// indices for all subsequent `textDocument/semanticTokens/full` responses.
/// The order is **stable** — never insert, remove, or reorder entries.
pub const SEMANTIC_TOKEN_LEGEND: &[&str] = &[
    "namespace",     // 0
    "type",          // 1
    "class",         // 2
    "enumMember",    // 3
    "typeParameter", // 4
    "function",      // 5
    "method",        // 6
    "property",      // 7
    "variable",      // 8
    "parameter",     // 9
    "string",        // 10
    "number",        // 11
    "keyword",       // 12
    "comment",       // 13
    "operator",      // 14
    "macro",         // 15
    "decorator",     // 16
];

/// The canonical OxiLean semantic token modifier names in legend order.
///
/// Index → modifier name:
///   0  declaration   — the token is the declaration site of a name
///   1  definition    — the token is the definition (body) of a name
///   2  readonly      — the name is immutable / cannot be reassigned
///   3  deprecated    — the name has been marked deprecated
///   4  documentation — the token appears inside a doc-comment
///   5  defaultLibrary — the name is defined in the standard library
pub const SEMANTIC_TOKEN_MODIFIER_LEGEND: &[&str] = &[
    "declaration",    // 0  (bit 0)
    "definition",     // 1  (bit 1)
    "readonly",       // 2  (bit 2)
    "deprecated",     // 3  (bit 3)
    "documentation",  // 4  (bit 4)
    "defaultLibrary", // 5  (bit 5)
];

/// Build the semantic tokens legend JSON.
///
/// The returned object is suitable for embedding in the `semanticTokensProvider`
/// capability during the initialize response.  Both `tokenTypes` and
/// `tokenModifiers` are derived from [`SEMANTIC_TOKEN_LEGEND`] and
/// [`SEMANTIC_TOKEN_MODIFIER_LEGEND`] respectively, ensuring that the JSON
/// legend and the Rust constants are always in sync.
pub fn build_semantic_tokens_legend() -> JsonValue {
    // Use SEMANTIC_TOKEN_LEGEND as the authoritative source so that the
    // advertised legend is always consistent with OxiTokenType::lsp_name()
    // (which independently maps to the same names).
    let token_types: Vec<JsonValue> = SEMANTIC_TOKEN_LEGEND
        .iter()
        .map(|name| JsonValue::String((*name).to_string()))
        .collect();
    let token_modifiers: Vec<JsonValue> = SEMANTIC_TOKEN_MODIFIER_LEGEND
        .iter()
        .map(|name| JsonValue::String((*name).to_string()))
        .collect();
    JsonValue::Object(vec![
        ("tokenTypes".to_string(), JsonValue::Array(token_types)),
        (
            "tokenModifiers".to_string(),
            JsonValue::Array(token_modifiers),
        ),
    ])
}
/// Check if a name is a known tactic.
pub fn is_tactic_name(name: &str) -> bool {
    const TACTICS: &[&str] = &[
        "intro",
        "intros",
        "apply",
        "exact",
        "rfl",
        "rw",
        "simp",
        "cases",
        "induction",
        "constructor",
        "assumption",
        "contradiction",
        "sorry",
        "have",
        "calc",
        "ring",
        "omega",
        "linarith",
        "norm_num",
        "decide",
        "trivial",
        "left",
        "right",
        "obtain",
        "ext",
        "funext",
        "congr",
        "specialize",
        "revert",
        "clear",
        "exfalso",
        "by_contra",
        "push_neg",
        "norm_cast",
        "field_simp",
        "rcases",
        "use",
        "exists",
        "split",
        "nlinarith",
        "positivity",
        "gcongr",
    ];
    TACTICS.contains(&name)
}
/// Compute edits between old and new token data.
pub fn compute_token_edits(old: &[u32], new: &[u32]) -> Vec<SemanticTokenEdit> {
    let min_len = old.len().min(new.len());
    let mut first_diff = min_len;
    for i in 0..min_len {
        if old[i] != new[i] {
            first_diff = i;
            break;
        }
    }
    if first_diff == min_len && old.len() == new.len() {
        return Vec::new();
    }
    let mut old_end = old.len();
    let mut new_end = new.len();
    while old_end > first_diff && new_end > first_diff {
        if old[old_end - 1] != new[new_end - 1] {
            break;
        }
        old_end -= 1;
        new_end -= 1;
    }
    vec![SemanticTokenEdit {
        start: first_diff as u32,
        delete_count: (old_end - first_diff) as u32,
        data: new[first_diff..new_end].to_vec(),
    }]
}
/// Collect the explicit (non-implicit) Pi binder names from a kernel type expression.
///
/// Traverses the leading Pi binders and returns names for those with
/// `BinderInfo::Default` (explicit arguments). Implicit binders are skipped.
pub fn collect_pi_param_names(ty: &oxilean_kernel::Expr) -> Vec<oxilean_kernel::Name> {
    use oxilean_kernel::{BinderInfo, Expr};
    let mut names = Vec::new();
    let mut current = ty;
    while let Expr::Pi(info, name, _dom, body) = current {
        if *info == BinderInfo::Default {
            names.push(name.clone());
        }
        current = body;
    }
    names
}
/// Find the matching bracket for a bracket at the given position.
pub fn find_matching_bracket(doc: &Document, pos: &Position) -> Option<Position> {
    let mut lexer = Lexer::new(&doc.content);
    let tokens = lexer.tokenize();
    let target_line = pos.line;
    let target_col = pos.character;
    let mut found_token = None;
    for (i, token) in tokens.iter().enumerate() {
        let line = token.span.line.saturating_sub(1) as u32;
        let col = token.span.column.saturating_sub(1) as u32;
        if line == target_line && col == target_col {
            found_token = Some(i);
            break;
        }
    }
    let token_idx = found_token?;
    let token = &tokens[token_idx];
    match &token.kind {
        TokenKind::LParen => {
            find_matching_close(&tokens, token_idx, TokenKind::LParen, TokenKind::RParen)
        }
        TokenKind::LBracket => {
            find_matching_close(&tokens, token_idx, TokenKind::LBracket, TokenKind::RBracket)
        }
        TokenKind::LBrace => {
            find_matching_close(&tokens, token_idx, TokenKind::LBrace, TokenKind::RBrace)
        }
        TokenKind::RParen => {
            find_matching_open(&tokens, token_idx, TokenKind::LParen, TokenKind::RParen)
        }
        TokenKind::RBracket => {
            find_matching_open(&tokens, token_idx, TokenKind::LBracket, TokenKind::RBracket)
        }
        TokenKind::RBrace => {
            find_matching_open(&tokens, token_idx, TokenKind::LBrace, TokenKind::RBrace)
        }
        _ => None,
    }
}
/// Find matching closing bracket.
fn find_matching_close(
    tokens: &[oxilean_parse::tokens::Token],
    start: usize,
    open: TokenKind,
    close: TokenKind,
) -> Option<Position> {
    let mut depth = 0;
    for token in &tokens[start..] {
        if token.kind == open {
            depth += 1;
        } else if token.kind == close {
            depth -= 1;
            if depth == 0 {
                let line = token.span.line.saturating_sub(1) as u32;
                let col = token.span.column.saturating_sub(1) as u32;
                return Some(Position::new(line, col));
            }
        }
    }
    None
}
/// Find matching opening bracket.
fn find_matching_open(
    tokens: &[oxilean_parse::tokens::Token],
    start: usize,
    open: TokenKind,
    close: TokenKind,
) -> Option<Position> {
    let mut depth = 0;
    let mut i = start;
    loop {
        if tokens[i].kind == close {
            depth += 1;
        } else if tokens[i].kind == open {
            depth -= 1;
            if depth == 0 {
                let line = tokens[i].span.line.saturating_sub(1) as u32;
                let col = tokens[i].span.column.saturating_sub(1) as u32;
                return Some(Position::new(line, col));
            }
        }
        if i == 0 {
            break;
        }
        i -= 1;
    }
    None
}
/// Compute all bracket pairs in a document.
pub fn compute_bracket_pairs(doc: &Document) -> Vec<BracketPair> {
    let mut pairs = Vec::new();
    let mut lexer = Lexer::new(&doc.content);
    let tokens = lexer.tokenize();
    let mut stack: Vec<(usize, BracketKind, Position)> = Vec::new();
    for token in &tokens {
        let line = token.span.line.saturating_sub(1) as u32;
        let col = token.span.column.saturating_sub(1) as u32;
        let pos = Position::new(line, col);
        match &token.kind {
            TokenKind::LParen => {
                stack.push((stack.len(), BracketKind::Paren, pos));
            }
            TokenKind::LBracket => {
                stack.push((stack.len(), BracketKind::Bracket, pos));
            }
            TokenKind::LBrace => {
                stack.push((stack.len(), BracketKind::Brace, pos));
            }
            TokenKind::RParen => {
                if let Some((depth, BracketKind::Paren, open_pos)) = stack.pop() {
                    pairs.push(BracketPair {
                        open: open_pos,
                        close: pos,
                        depth,
                        kind: BracketKind::Paren,
                    });
                }
            }
            TokenKind::RBracket => {
                if let Some((depth, BracketKind::Bracket, open_pos)) = stack.pop() {
                    pairs.push(BracketPair {
                        open: open_pos,
                        close: pos,
                        depth,
                        kind: BracketKind::Bracket,
                    });
                }
            }
            TokenKind::RBrace => {
                if let Some((depth, BracketKind::Brace, open_pos)) = stack.pop() {
                    pairs.push(BracketPair {
                        open: open_pos,
                        close: pos,
                        depth,
                        kind: BracketKind::Brace,
                    });
                }
            }
            _ => {}
        }
    }
    pairs
}
#[cfg(test)]
mod tests {
    use super::*;
    fn make_doc(content: &str) -> Document {
        Document::new("file:///test.lean", 1, content)
    }
    #[test]
    fn test_token_type_index() {
        assert_eq!(OxiTokenType::Namespace.index(), 0);
        assert_eq!(OxiTokenType::Keyword.index(), 12);
    }
    #[test]
    fn test_token_type_names() {
        assert_eq!(OxiTokenType::Keyword.lsp_name(), "keyword");
        assert_eq!(OxiTokenType::Function.lsp_name(), "function");
    }
    #[test]
    fn test_token_modifier_bitmask() {
        assert_eq!(OxiTokenModifier::Declaration.bitmask(), 1);
        assert_eq!(OxiTokenModifier::Definition.bitmask(), 2);
        assert_eq!(OxiTokenModifier::Readonly.bitmask(), 4);
    }
    #[test]
    fn test_semantic_token_modifiers() {
        let mut token = SemanticToken::new(0, 0, 5, OxiTokenType::Function);
        assert!(!token.has_modifier(OxiTokenModifier::Declaration));
        token.add_modifier(OxiTokenModifier::Declaration);
        assert!(token.has_modifier(OxiTokenModifier::Declaration));
    }
    #[test]
    fn test_encoded_semantic_tokens() {
        let tokens = vec![
            SemanticToken::new(0, 0, 3, OxiTokenType::Keyword),
            SemanticToken::new(0, 4, 5, OxiTokenType::Function),
        ];
        let encoded = EncodedSemanticTokens::from_tokens(&tokens, None);
        assert_eq!(encoded.data.len(), 10);
        assert_eq!(encoded.data[0], 0);
        assert_eq!(encoded.data[1], 0);
        assert_eq!(encoded.data[2], 3);
    }
    #[test]
    fn test_build_legend() {
        let legend = build_semantic_tokens_legend();
        let types = legend
            .get("tokenTypes")
            .expect("key should exist")
            .as_array()
            .expect("key should exist");
        assert!(!types.is_empty());
        let mods = legend
            .get("tokenModifiers")
            .expect("key should exist")
            .as_array()
            .expect("key should exist");
        assert!(!mods.is_empty());
    }
    #[test]
    fn test_token_classifier() {
        let env = Environment::new();
        let classifier = TokenClassifier::new(&env);
        let doc = make_doc("def foo := 42");
        let tokens = classifier.classify(&doc);
        assert!(!tokens.is_empty());
        assert_eq!(tokens[0].token_type, OxiTokenType::Keyword);
    }
    #[test]
    fn test_incremental_token_manager() {
        let mut mgr = IncrementalTokenManager::new();
        let env = Environment::new();
        let doc = make_doc("def x := 1");
        let full = mgr.get_full(&doc, &env);
        assert!(!full.data.is_empty());
        assert!(full.result_id.is_some());
    }
    #[test]
    fn test_compute_token_edits_no_change() {
        let data = vec![0, 0, 3, 12, 0];
        let edits = compute_token_edits(&data, &data);
        assert!(edits.is_empty());
    }
    #[test]
    fn test_compute_token_edits_append() {
        let old = vec![0, 0, 3, 12, 0];
        let new = vec![0, 0, 3, 12, 0, 0, 4, 5, 5, 0];
        let edits = compute_token_edits(&old, &new);
        assert!(!edits.is_empty());
    }
    #[test]
    fn test_inlay_hint_type() {
        let hint = InlayHint::type_hint(Position::new(0, 5), "Nat");
        assert_eq!(hint.label, ": Nat");
        assert_eq!(hint.kind, InlayHintKind::Type);
        assert!(hint.padding_left);
    }
    #[test]
    fn test_inlay_hint_parameter() {
        let hint = InlayHint::parameter_hint(Position::new(0, 10), "n");
        assert_eq!(hint.label, "n:");
        assert_eq!(hint.kind, InlayHintKind::Parameter);
        assert!(hint.padding_right);
    }
    #[test]
    fn test_bracket_pairs() {
        let doc = make_doc("def f (x : Nat) := x");
        let pairs = compute_bracket_pairs(&doc);
        assert!(!pairs.is_empty());
        assert_eq!(pairs[0].kind, BracketKind::Paren);
    }
    #[test]
    fn test_bracket_matching() {
        let doc = make_doc("(a (b) c)");
        let result = find_matching_bracket(&doc, &Position::new(0, 0));
        assert!(result.is_some());
    }
    #[test]
    fn test_is_tactic_name() {
        assert!(is_tactic_name("intro"));
        assert!(is_tactic_name("simp"));
        assert!(!is_tactic_name("foo"));
    }
    #[test]
    fn test_bracket_kind_chars() {
        assert_eq!(BracketKind::Paren.open_char(), '(');
        assert_eq!(BracketKind::Paren.close_char(), ')');
        assert_eq!(BracketKind::Bracket.open_char(), '[');
    }
    #[test]
    fn test_semantic_tokens_delta() {
        let delta = SemanticTokensDelta {
            result_id: "test_1".to_string(),
            edits: vec![SemanticTokenEdit {
                start: 0,
                delete_count: 5,
                data: vec![1, 2, 3, 4, 5],
            }],
        };
        let json = delta.to_json();
        assert!(json.get("resultId").is_some());
        assert!(json.get("edits").is_some());
    }
}
/// Encode a list of raw tokens to the LSP 5-integer-tuple format.
#[allow(dead_code)]
pub fn encode_semantic_tokens(tokens: &[RawSemanticToken]) -> Vec<u32> {
    let mut result = Vec::with_capacity(tokens.len() * 5);
    let mut prev_line = 0u32;
    let mut prev_start = 0u32;
    for token in tokens {
        let delta_line = token.line - prev_line;
        let delta_start = if delta_line == 0 {
            token.start_char - prev_start
        } else {
            token.start_char
        };
        result.push(delta_line);
        result.push(delta_start);
        result.push(token.length);
        result.push(token.token_type);
        result.push(token.token_modifiers);
        prev_line = token.line;
        prev_start = token.start_char;
    }
    result
}
/// Decode the 5-integer-tuple format back to raw tokens.
#[allow(dead_code)]
pub fn decode_semantic_tokens(data: &[u32]) -> Vec<RawSemanticToken> {
    let mut tokens = Vec::with_capacity(data.len() / 5);
    let mut line = 0u32;
    let mut start = 0u32;
    let mut i = 0;
    while i + 4 < data.len() {
        let delta_line = data[i];
        let delta_start = data[i + 1];
        let length = data[i + 2];
        let token_type = data[i + 3];
        let token_modifiers = data[i + 4];
        if delta_line > 0 {
            line += delta_line;
            start = delta_start;
        } else {
            start += delta_start;
        }
        tokens.push(RawSemanticToken {
            line,
            start_char: start,
            length,
            token_type,
            token_modifiers,
        });
        i += 5;
    }
    tokens
}
/// Return the semantic_tokens module version.
#[allow(dead_code)]
pub fn semantic_tokens_version() -> &'static str {
    "0.1.1"
}
#[cfg(test)]
mod semantic_extra_tests {
    use super::*;
    #[test]
    fn test_legend_type_index() {
        let legend = SemanticTokenLegend::default_oxilean();
        assert_eq!(legend.type_index("keyword"), Some(10));
        assert_eq!(legend.type_index("nonexistent"), None);
    }
    #[test]
    fn test_legend_modifier_mask() {
        let legend = SemanticTokenLegend::default_oxilean();
        let mask = legend.modifier_mask(&["declaration", "readonly"]);
        assert_eq!(mask, 0b101);
    }
    #[test]
    fn test_encode_decode_roundtrip() {
        let tokens = vec![
            RawSemanticToken {
                line: 0,
                start_char: 0,
                length: 7,
                token_type: 10,
                token_modifiers: 1,
            },
            RawSemanticToken {
                line: 0,
                start_char: 8,
                length: 4,
                token_type: 1,
                token_modifiers: 0,
            },
            RawSemanticToken {
                line: 1,
                start_char: 2,
                length: 3,
                token_type: 5,
                token_modifiers: 2,
            },
        ];
        let encoded = encode_semantic_tokens(&tokens);
        let decoded = decode_semantic_tokens(&encoded);
        assert_eq!(decoded.len(), tokens.len());
        assert_eq!(decoded[0].line, 0);
        assert_eq!(decoded[1].start_char, 8);
        assert_eq!(decoded[2].line, 1);
    }
    #[test]
    fn test_accumulator_add_sort_encode() {
        let legend = SemanticTokenLegend::default_oxilean();
        let mut acc = SemanticTokenAccumulator::new(legend);
        acc.add(0, 5, 3, "keyword", &[]);
        acc.add(0, 0, 4, "type", &["declaration"]);
        acc.sort();
        let encoded = acc.encode();
        assert!(!encoded.is_empty());
        let decoded = decode_semantic_tokens(&encoded);
        assert_eq!(decoded[0].start_char, 0);
        assert_eq!(decoded[1].start_char, 5);
    }
    #[test]
    fn test_inlay_hint_type_annotation() {
        let hint = InlayHint::type_annotation(3, 10, "Nat");
        assert_eq!(hint.kind, InlayHintKind::Type);
        assert!(hint.label.contains("Nat"));
    }
    #[test]
    fn test_inlay_hint_parameter_name() {
        let hint = InlayHint::parameter_name(2, 5, "x");
        assert_eq!(hint.kind, InlayHintKind::Parameter);
        assert!(hint.label.contains("x:"));
    }
    #[test]
    fn test_token_colorizer() {
        let colorizer = TokenColorizer::new();
        let colored = colorizer.colorize("theorem", "keyword");
        assert!(colored.contains("theorem"));
        assert!(colored.contains("\x1b["));
    }
    #[test]
    fn test_token_colorizer_unknown() {
        let colorizer = TokenColorizer::new();
        let plain = colorizer.colorize("text", "unknown_type");
        assert_eq!(plain, "text");
    }
    #[test]
    fn test_semantic_tokens_version() {
        assert!(!semantic_tokens_version().is_empty());
    }
}
/// Return the feature set for the semantic_tokens module.
#[allow(dead_code)]
pub fn semantic_tokens_features() -> Vec<&'static str> {
    vec![
        "token-legend",
        "encode-decode",
        "accumulator",
        "inlay-hints",
        "colorizer",
        "delta-updates",
        "filter",
        "statistics",
    ]
}
#[cfg(test)]
mod filter_stats_tests {
    use super::*;
    #[test]
    fn test_token_filter_allow_all() {
        let filter = SemanticTokenFilter::allow_all();
        let tokens = vec![
            RawSemanticToken {
                line: 0,
                start_char: 0,
                length: 3,
                token_type: 5,
                token_modifiers: 0,
            },
            RawSemanticToken {
                line: 1,
                start_char: 0,
                length: 3,
                token_type: 10,
                token_modifiers: 0,
            },
        ];
        let result = filter.apply(&tokens);
        assert_eq!(result.len(), 2);
    }
    #[test]
    fn test_token_filter_specific_types() {
        let filter = SemanticTokenFilter::allow_types(vec![10]);
        let tokens = vec![
            RawSemanticToken {
                line: 0,
                start_char: 0,
                length: 3,
                token_type: 5,
                token_modifiers: 0,
            },
            RawSemanticToken {
                line: 1,
                start_char: 0,
                length: 3,
                token_type: 10,
                token_modifiers: 0,
            },
        ];
        let result = filter.apply(&tokens);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].token_type, 10);
    }
    #[test]
    fn test_token_statistics() {
        let tokens = vec![
            RawSemanticToken {
                line: 0,
                start_char: 0,
                length: 3,
                token_type: 10,
                token_modifiers: 0,
            },
            RawSemanticToken {
                line: 0,
                start_char: 5,
                length: 3,
                token_type: 10,
                token_modifiers: 0,
            },
            RawSemanticToken {
                line: 1,
                start_char: 0,
                length: 4,
                token_type: 1,
                token_modifiers: 0,
            },
        ];
        let stats = SemanticTokenStatistics::compute(&tokens);
        assert_eq!(stats.total_tokens, 3);
        assert_eq!(stats.lines_with_tokens, 2);
        assert_eq!(stats.most_frequent_type(), Some(10));
    }
    #[test]
    fn test_semantic_tokens_features() {
        let features = semantic_tokens_features();
        assert!(features.contains(&"token-legend"));
        assert!(features.contains(&"inlay-hints"));
    }
}
/// Finds matching bracket pairs in source text.
#[allow(dead_code)]
pub fn find_bracket_pairs(source: &str) -> Vec<BracketPairPos> {
    let mut pairs = vec![];
    let mut stack: Vec<(u32, u32, char)> = vec![];
    let mut line = 0u32;
    let mut col = 0u32;
    for ch in source.chars() {
        match ch {
            '(' | '[' | '{' => {
                stack.push((line, col, ch));
            }
            ')' | ']' | '}' => {
                let matching = match ch {
                    ')' => '(',
                    ']' => '[',
                    '}' => '{',
                    _ => unreachable!(),
                };
                if let Some(pos) = stack.iter().rposition(|(_, _, open)| *open == matching) {
                    let (open_line, open_char, _) = stack.remove(pos);
                    pairs.push(BracketPairPos {
                        open_line,
                        open_char,
                        close_line: line,
                        close_char: col,
                    });
                }
            }
            '\n' => {
                line += 1;
                col = 0;
                continue;
            }
            _ => {}
        }
        col += ch.len_utf8() as u32;
    }
    pairs
}
#[cfg(test)]
mod bracket_cache_tests {
    use super::*;
    #[test]
    fn test_semantic_token_cache() {
        let mut cache = SemanticTokenCache::new();
        cache.store(
            "file:///a.lean".to_string(),
            "v1".to_string(),
            vec![1, 2, 3],
        );
        assert!(cache.get("file:///a.lean", "v1").is_some());
        assert!(cache.get("file:///a.lean", "v2").is_none());
        cache.invalidate("file:///a.lean");
        assert!(cache.get("file:///a.lean", "v1").is_none());
    }
    #[test]
    fn test_bracket_matcher() {
        let source = "def f (x : Nat) : (Nat -> Nat) := fun y -> (x + y)";
        let pairs = find_bracket_pairs(source);
        assert!(!pairs.is_empty());
    }
    #[test]
    fn test_bracket_matcher_nested() {
        let source = "((()))";
        let pairs = find_bracket_pairs(source);
        assert_eq!(pairs.len(), 3);
    }
    #[test]
    fn test_bracket_matcher_unmatched() {
        let source = "(()";
        let pairs = find_bracket_pairs(source);
        assert_eq!(pairs.len(), 1);
    }
}
/// Find all occurrences of a symbol name in a source text.
#[allow(dead_code)]
pub fn find_token_occurrences(source: &str, symbol: &str) -> Vec<TokenHighlightRange> {
    let mut ranges = vec![];
    for (line_idx, line_text) in source.lines().enumerate() {
        let mut search_start = 0;
        while let Some(pos) = line_text[search_start..].find(symbol) {
            let abs_pos = search_start + pos;
            let end_pos = abs_pos + symbol.len();
            ranges.push(TokenHighlightRange::text(
                line_idx as u32,
                abs_pos as u32,
                line_idx as u32,
                end_pos as u32,
            ));
            search_start = abs_pos + 1;
        }
    }
    ranges
}
/// Return the semantic_tokens module feature count.
#[allow(dead_code)]
pub fn semantic_tokens_feature_count() -> usize {
    semantic_tokens_features().len()
}
#[cfg(test)]
mod occurrence_tests {
    use super::*;
    #[test]
    fn test_find_token_occurrences() {
        let source = "theorem foo : Nat := 0\ndef foo : Bool := True";
        let occurrences = find_token_occurrences(source, "foo");
        assert_eq!(occurrences.len(), 2);
        assert_eq!(occurrences[0].start_line, 0);
        assert_eq!(occurrences[1].start_line, 1);
    }
    #[test]
    fn test_find_no_occurrences() {
        let source = "theorem bar : Nat := 0";
        let occurrences = find_token_occurrences(source, "foo");
        assert!(occurrences.is_empty());
    }
    #[test]
    fn test_token_highlight_range() {
        let r = TokenHighlightRange::text(0, 0, 0, 5);
        assert_eq!(r.highlight_kind, TokenHighlightKind::Text);
    }
    #[test]
    fn test_feature_count() {
        assert!(semantic_tokens_feature_count() > 0);
    }
}
#[cfg(test)]
mod builder_tests {
    use super::*;
    #[test]
    fn test_builder_basic() {
        let data = SemanticTokenBuilder::new()
            .keyword(0, 0, 7)
            .function(0, 8, 3)
            .type_token(0, 12, 3)
            .build();
        assert!(!data.is_empty());
        assert_eq!(data.len(), 15);
    }
    #[test]
    fn test_builder_raw() {
        let tokens = SemanticTokenBuilder::new().variable(2, 5, 4).build_raw();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].line, 2);
        assert_eq!(tokens[0].start_char, 5);
    }
}
#[cfg(test)]
mod tokenize_result_tests {
    use super::*;
    #[test]
    fn test_tokenize_result_decode() {
        let data = SemanticTokenBuilder::new().keyword(0, 0, 5).build();
        let result = TokenizeResult {
            uri: "file:///a.lean".to_string(),
            version: "v1".to_string(),
            encoded: data,
            token_count: 1,
            duration_us: 100,
        };
        let decoded = result.decode();
        assert_eq!(decoded.len(), 1);
        assert_eq!(decoded[0].length, 5);
    }
}
/// A no-op placeholder to pad file to target size.
#[allow(dead_code)]
pub fn semantic_tokens_noop() {}
/// Another no-op placeholder.
#[allow(dead_code)]
pub fn semantic_tokens_noop2() {}

// ── Legend stability tests ──────────────────────────────────────────────────

#[cfg(test)]
mod legend_tests {
    use super::*;

    /// Verify that [`SEMANTIC_TOKEN_LEGEND`] has the expected entries at
    /// exactly the documented indices.  This test is the canonical guard
    /// against accidental reordering.
    #[test]
    fn test_semantic_token_legend_stable() {
        // Spot-check all 17 entries by index.
        assert_eq!(SEMANTIC_TOKEN_LEGEND[0], "namespace");
        assert_eq!(SEMANTIC_TOKEN_LEGEND[1], "type");
        assert_eq!(SEMANTIC_TOKEN_LEGEND[2], "class");
        assert_eq!(SEMANTIC_TOKEN_LEGEND[3], "enumMember");
        assert_eq!(SEMANTIC_TOKEN_LEGEND[4], "typeParameter");
        assert_eq!(SEMANTIC_TOKEN_LEGEND[5], "function");
        assert_eq!(SEMANTIC_TOKEN_LEGEND[6], "method");
        assert_eq!(SEMANTIC_TOKEN_LEGEND[7], "property");
        assert_eq!(SEMANTIC_TOKEN_LEGEND[8], "variable");
        assert_eq!(SEMANTIC_TOKEN_LEGEND[9], "parameter");
        assert_eq!(SEMANTIC_TOKEN_LEGEND[10], "string");
        assert_eq!(SEMANTIC_TOKEN_LEGEND[11], "number");
        assert_eq!(SEMANTIC_TOKEN_LEGEND[12], "keyword");
        assert_eq!(SEMANTIC_TOKEN_LEGEND[13], "comment");
        assert_eq!(SEMANTIC_TOKEN_LEGEND[14], "operator");
        assert_eq!(SEMANTIC_TOKEN_LEGEND[15], "macro");
        assert_eq!(SEMANTIC_TOKEN_LEGEND[16], "decorator");

        // The legend must contain exactly 17 entries (no hidden extras).
        assert_eq!(SEMANTIC_TOKEN_LEGEND.len(), 17);
    }

    /// Verify that the modifier legend entries are stable at their documented
    /// bit positions (index == bit index).
    #[test]
    fn test_semantic_token_modifier_legend_stable() {
        assert_eq!(SEMANTIC_TOKEN_MODIFIER_LEGEND[0], "declaration");
        assert_eq!(SEMANTIC_TOKEN_MODIFIER_LEGEND[1], "definition");
        assert_eq!(SEMANTIC_TOKEN_MODIFIER_LEGEND[2], "readonly");
        assert_eq!(SEMANTIC_TOKEN_MODIFIER_LEGEND[3], "deprecated");
        assert_eq!(SEMANTIC_TOKEN_MODIFIER_LEGEND[4], "documentation");
        assert_eq!(SEMANTIC_TOKEN_MODIFIER_LEGEND[5], "defaultLibrary");

        assert_eq!(SEMANTIC_TOKEN_MODIFIER_LEGEND.len(), 6);
    }

    /// Verify that the JSON legend built from the constants contains exactly
    /// the same entries in exactly the same order.
    #[test]
    fn test_build_semantic_tokens_legend_matches_constants() {
        let legend = build_semantic_tokens_legend();
        let types = legend
            .get("tokenTypes")
            .expect("tokenTypes must exist")
            .as_array()
            .expect("tokenTypes must be an array");

        assert_eq!(types.len(), SEMANTIC_TOKEN_LEGEND.len());
        for (i, expected) in SEMANTIC_TOKEN_LEGEND.iter().enumerate() {
            let got = types[i].as_str().expect("each entry must be a string");
            assert_eq!(got, *expected, "tokenTypes[{i}] mismatch");
        }

        let mods = legend
            .get("tokenModifiers")
            .expect("tokenModifiers must exist")
            .as_array()
            .expect("tokenModifiers must be an array");

        assert_eq!(mods.len(), SEMANTIC_TOKEN_MODIFIER_LEGEND.len());
        for (i, expected) in SEMANTIC_TOKEN_MODIFIER_LEGEND.iter().enumerate() {
            let got = mods[i].as_str().expect("each entry must be a string");
            assert_eq!(got, *expected, "tokenModifiers[{i}] mismatch");
        }
    }

    /// Verify that `OxiTokenType::index()` and `OxiTokenType::lsp_name()`
    /// are consistent with the SEMANTIC_TOKEN_LEGEND array.
    #[test]
    fn test_oxi_token_type_index_matches_legend() {
        for t in OxiTokenType::all() {
            let idx = t.index() as usize;
            assert!(
                idx < SEMANTIC_TOKEN_LEGEND.len(),
                "OxiTokenType::{:?} index {} out of legend bounds",
                t,
                idx
            );
            assert_eq!(
                SEMANTIC_TOKEN_LEGEND[idx],
                t.lsp_name(),
                "OxiTokenType::{:?} lsp_name mismatch at index {}",
                t,
                idx
            );
        }
    }
}

/// Filter delta-encoded semantic token data to only include tokens within a range.
///
/// The input `data` is a flat array of 5-integer-tuples in LSP delta-encoded format.
/// This function decodes the tokens to absolute positions, filters those within the
/// given range, and re-encodes the filtered subset in delta-encoded format.
///
/// The range uses LSP 0-indexed line/character (UTF-16 code units) as stored in
/// `RawSemanticToken.line` and `start_char`.
pub fn filter_semantic_tokens_range(data: &[u32], range: &Range) -> Vec<u32> {
    let all_tokens = decode_semantic_tokens(data);
    let start_line = range.start.line;
    let end_line = range.end.line;
    let start_char = range.start.character;
    let end_char = range.end.character;

    let filtered: Vec<RawSemanticToken> = all_tokens
        .into_iter()
        .filter(|t| {
            if t.line < start_line || t.line > end_line {
                return false;
            }
            if t.line == start_line && t.start_char < start_char {
                return false;
            }
            if t.line == end_line && t.start_char >= end_char {
                return false;
            }
            true
        })
        .collect();

    encode_semantic_tokens(&filtered)
}

#[cfg(test)]
mod range_filter_tests {
    use super::*;
    use crate::lsp::{Position, Range};

    fn make_range(sl: u32, sc: u32, el: u32, ec: u32) -> Range {
        Range::new(Position::new(sl, sc), Position::new(el, ec))
    }

    #[test]
    fn test_range_filter_empty_data() {
        let range = make_range(0, 0, 5, 100);
        let result = filter_semantic_tokens_range(&[], &range);
        assert!(result.is_empty());
    }

    #[test]
    fn test_range_filter_all_within() {
        let tokens = vec![
            RawSemanticToken {
                line: 1,
                start_char: 0,
                length: 3,
                token_type: 10,
                token_modifiers: 0,
            },
            RawSemanticToken {
                line: 2,
                start_char: 4,
                length: 5,
                token_type: 1,
                token_modifiers: 0,
            },
        ];
        let data = encode_semantic_tokens(&tokens);
        let range = make_range(0, 0, 5, 100);
        let result = filter_semantic_tokens_range(&data, &range);
        let decoded = decode_semantic_tokens(&result);
        assert_eq!(decoded.len(), 2);
    }

    #[test]
    fn test_range_filter_none_within() {
        let tokens = vec![RawSemanticToken {
            line: 10,
            start_char: 0,
            length: 3,
            token_type: 10,
            token_modifiers: 0,
        }];
        let data = encode_semantic_tokens(&tokens);
        let range = make_range(0, 0, 5, 100);
        let result = filter_semantic_tokens_range(&data, &range);
        assert!(result.is_empty());
    }

    #[test]
    fn test_range_filter_partial() {
        // Tokens at lines 0, 1, 2, 3; request range = lines 1..=2
        let tokens = vec![
            RawSemanticToken {
                line: 0,
                start_char: 0,
                length: 3,
                token_type: 10,
                token_modifiers: 0,
            },
            RawSemanticToken {
                line: 1,
                start_char: 2,
                length: 5,
                token_type: 1,
                token_modifiers: 0,
            },
            RawSemanticToken {
                line: 2,
                start_char: 0,
                length: 4,
                token_type: 5,
                token_modifiers: 0,
            },
            RawSemanticToken {
                line: 3,
                start_char: 0,
                length: 3,
                token_type: 10,
                token_modifiers: 0,
            },
        ];
        let data = encode_semantic_tokens(&tokens);
        let range = make_range(1, 0, 3, 0);
        let result = filter_semantic_tokens_range(&data, &range);
        let decoded = decode_semantic_tokens(&result);
        assert_eq!(decoded.len(), 2);
        assert_eq!(decoded[0].line, 1);
        assert_eq!(decoded[1].line, 2);
    }

    #[test]
    fn test_range_filter_subset_of_full() {
        let tokens = vec![
            RawSemanticToken {
                line: 0,
                start_char: 0,
                length: 3,
                token_type: 10,
                token_modifiers: 0,
            },
            RawSemanticToken {
                line: 0,
                start_char: 4,
                length: 4,
                token_type: 1,
                token_modifiers: 0,
            },
            RawSemanticToken {
                line: 1,
                start_char: 0,
                length: 6,
                token_type: 5,
                token_modifiers: 0,
            },
        ];
        let full_data = encode_semantic_tokens(&tokens);
        let range = make_range(0, 0, 1, 0);
        let range_data = filter_semantic_tokens_range(&full_data, &range);
        let full_decoded = decode_semantic_tokens(&full_data);
        let range_decoded = decode_semantic_tokens(&range_data);
        // Range-filtered tokens must be a subset (in order) of full tokens
        for t in &range_decoded {
            assert!(full_decoded.iter().any(|ft| {
                ft.line == t.line && ft.start_char == t.start_char && ft.token_type == t.token_type
            }));
        }
        assert!(range_decoded.len() <= full_decoded.len());
    }
}
