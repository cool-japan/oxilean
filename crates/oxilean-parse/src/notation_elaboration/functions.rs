//! Functions for user-defined notation elaboration.

use super::types::{
    Assoc, ElabNotationResult, MixfixPart, NotationConflict, NotationDB, NotationDef, NotationKind,
};

// ────────────────────────────────────────────────────────────────────────────
// NotationDB implementation
// ────────────────────────────────────────────────────────────────────────────

impl NotationDB {
    /// Create an empty `NotationDB`.
    pub fn new() -> Self {
        Self {
            notations: Vec::new(),
        }
    }

    /// Add a notation definition, maintaining descending priority order.
    ///
    /// Returns `Err(NotationConflict)` if an identical-kind notation for the
    /// same symbol already exists **at the same priority**.  Identical
    /// priorities with different translations are considered conflicting.
    pub fn add(&mut self, def: NotationDef) -> Result<(), Box<NotationConflict>> {
        // Check for conflicts: same symbol, same kind-discriminant, same priority.
        for existing in &self.notations {
            if existing.symbol == def.symbol
                && same_kind_discriminant(&existing.kind, &def.kind)
                && existing.priority == def.priority
                && existing.translation != def.translation
            {
                return Err(Box::new(NotationConflict {
                    symbol: def.symbol.clone(),
                    existing: existing.clone(),
                    new_: def,
                }));
            }
        }

        // Insert maintaining descending priority order.
        let pos = self
            .notations
            .binary_search_by(|a| def.priority.cmp(&a.priority))
            .unwrap_or_else(|p| p);
        self.notations.insert(pos, def);
        Ok(())
    }

    /// Find the highest-priority prefix notation for `sym`.
    pub fn find_prefix(&self, sym: &str) -> Option<&NotationDef> {
        self.notations
            .iter()
            .find(|n| n.symbol == sym && matches!(n.kind, NotationKind::Prefix { .. }))
    }

    /// Find the highest-priority infix notation for `sym`.
    pub fn find_infix(&self, sym: &str) -> Option<&NotationDef> {
        self.notations
            .iter()
            .find(|n| n.symbol == sym && matches!(n.kind, NotationKind::Infix { .. }))
    }

    /// Find the highest-priority postfix notation for `sym`.
    pub fn find_postfix(&self, sym: &str) -> Option<&NotationDef> {
        self.notations
            .iter()
            .find(|n| n.symbol == sym && matches!(n.kind, NotationKind::Postfix { .. }))
    }

    /// Find the highest-priority mixfix notation for `sym`.
    pub fn find_mixfix(&self, sym: &str) -> Option<&NotationDef> {
        self.notations
            .iter()
            .find(|n| n.symbol == sym && matches!(n.kind, NotationKind::Mixfix { .. }))
    }
}

/// Returns `true` when `a` and `b` have the same kind discriminant.
fn same_kind_discriminant(a: &NotationKind, b: &NotationKind) -> bool {
    matches!(
        (a, b),
        (NotationKind::Prefix { .. }, NotationKind::Prefix { .. })
            | (NotationKind::Postfix { .. }, NotationKind::Postfix { .. })
            | (NotationKind::Infix { .. }, NotationKind::Infix { .. })
            | (NotationKind::Mixfix { .. }, NotationKind::Mixfix { .. })
    )
}

// ────────────────────────────────────────────────────────────────────────────
// Tokenisation helper
// ────────────────────────────────────────────────────────────────────────────

/// Split `source` into a flat list of tokens.
///
/// A "token" is either a maximal run of ASCII alphanumeric / `_` / `#`
/// characters, a UTF-8 multi-byte character, or a single ASCII punctuation
/// character.  Whitespace is retained as single-space tokens so that we can
/// reconstruct the approximate spacing after elaboration.
fn tokenise(source: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut cur = String::new();
    let mut chars = source.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch.is_ascii_whitespace() {
            if !cur.is_empty() {
                tokens.push(std::mem::take(&mut cur));
            }
            // Collapse whitespace.
            while chars
                .peek()
                .map(|c| c.is_ascii_whitespace())
                .unwrap_or(false)
            {
                chars.next();
            }
            tokens.push(" ".to_string());
        } else if ch.is_ascii_alphanumeric() || ch == '_' || ch == '#' || ch == '$' {
            cur.push(ch);
        } else if ch.is_ascii() {
            // ASCII punctuation / operator character — flush current token first.
            if !cur.is_empty() {
                tokens.push(std::mem::take(&mut cur));
            }
            tokens.push(ch.to_string());
        } else {
            // Multi-byte UTF-8 character (e.g. `∧`, `→`).
            if !cur.is_empty() {
                tokens.push(std::mem::take(&mut cur));
            }
            tokens.push(ch.to_string());
        }
    }
    if !cur.is_empty() {
        tokens.push(cur);
    }
    tokens
}

// ────────────────────────────────────────────────────────────────────────────
// resolve_mixfix
// ────────────────────────────────────────────────────────────────────────────

/// Try to match `tokens` against the mixfix pattern in `def`.
///
/// Returns `Some(translated)` when the pattern matches, `None` otherwise.
///
/// The matching algorithm handles `MixfixPart::Literal` (exact token) and
/// `MixfixPart::Placeholder` (consumes one non-whitespace token as an
/// argument).  The translation template uses `$0`, `$1`, … to refer to
/// captured arguments.
pub fn resolve_mixfix(tokens: &[String], def: &NotationDef) -> Option<String> {
    let parts = match &def.kind {
        NotationKind::Mixfix { parts } => parts,
        _ => return None,
    };

    // Filter whitespace tokens from the input for matching purposes.
    let non_ws: Vec<&str> = tokens
        .iter()
        .filter(|t| t.as_str() != " ")
        .map(|t| t.as_str())
        .collect();

    let mut tok_idx = 0usize;
    let mut args: Vec<String> = Vec::new();

    for part in parts {
        match part {
            MixfixPart::Literal(lit) => {
                if tok_idx >= non_ws.len() || non_ws[tok_idx] != lit.as_str() {
                    return None;
                }
                tok_idx += 1;
            }
            MixfixPart::Placeholder(_prec) => {
                if tok_idx >= non_ws.len() {
                    return None;
                }
                args.push(non_ws[tok_idx].to_string());
                tok_idx += 1;
            }
        }
    }

    // All tokens must be consumed for a complete match.
    if tok_idx != non_ws.len() {
        return None;
    }

    let mut translation = def.translation.clone();
    for (i, arg) in args.iter().enumerate() {
        translation = translation.replace(&format!("${}", i), arg);
    }
    Some(translation)
}

// ────────────────────────────────────────────────────────────────────────────
// elaborate_notation
// ────────────────────────────────────────────────────────────────────────────

/// Elaborate all notations in `source` according to `db`.
///
/// This performs a single left-to-right pass over the token stream:
///
/// 1. When a token matches a prefix notation symbol, it is replaced by its
///    translation applied to the following argument token.
/// 2. When a token matches an infix notation symbol (between two tokens), it
///    is replaced by its translation applied to the left and right operands.
/// 3. Mixfix patterns are attempted on the entire remaining suffix.
///
/// The elaboration is intentionally simple — a full Pratt parser would need
/// to be integrated with the main parser.  This function serves as a
/// post-processing step to desugar notation in already-tokenised source.
pub fn elaborate_notation(source: &str, db: &NotationDB) -> ElabNotationResult {
    let original = source.to_string();
    let tokens = tokenise(source);
    let mut applied: Vec<String> = Vec::new();
    let mut result_tokens: Vec<String> = Vec::new();
    let mut i = 0usize;

    // Non-whitespace token accessor.
    let nws_tokens: Vec<(usize, &str)> = tokens
        .iter()
        .enumerate()
        .filter(|(_, t)| t.as_str() != " ")
        .map(|(idx, t)| (idx, t.as_str()))
        .collect();

    let mut processed = vec![false; tokens.len()];

    let mut nws_idx = 0usize;
    while nws_idx < nws_tokens.len() {
        let (tok_pos, tok) = nws_tokens[nws_idx];

        // Try prefix notation.
        if let Some(def) = db.find_prefix(tok) {
            // Need at least one more non-ws token as argument.
            if nws_idx + 1 < nws_tokens.len() {
                let (arg_pos, arg) = nws_tokens[nws_idx + 1];
                let expanded = def.translation.replace("$0", arg).replace("$1", tok);
                // Mark both tokens as processed.
                (i..=arg_pos).for_each(|p| processed[p] = true);
                result_tokens.push(expanded);
                applied.push(def.symbol.clone());
                // Skip whitespace tokens up to arg_pos + 1.
                i = arg_pos + 1;
                nws_idx += 2;
                continue;
            }
        }

        // Try infix notation: check if previous result and next token can form an infix.
        if let Some(def) = db.find_infix(tok) {
            if nws_idx > 0 && nws_idx + 1 < nws_tokens.len() {
                let (_, right) = nws_tokens[nws_idx + 1];
                // The "left" operand is the last result token (non-ws).
                if let Some(left) = result_tokens.last().cloned() {
                    let left_trimmed = left.trim().to_string();
                    // Remove last result token so we can replace it.
                    result_tokens.pop();
                    let expanded = def
                        .translation
                        .replace("$0", &left_trimmed)
                        .replace("$1", right);
                    let (arg_pos, _) = nws_tokens[nws_idx + 1];
                    (tok_pos..=arg_pos).for_each(|p| processed[p] = true);
                    result_tokens.push(expanded);
                    applied.push(def.symbol.clone());
                    i = arg_pos + 1;
                    nws_idx += 2;
                    continue;
                }
            }
        }

        // No notation matched — emit token as-is.
        for p in i..=tok_pos {
            if !processed[p] {
                result_tokens.push(tokens[p].clone());
            }
        }
        i = tok_pos + 1;
        nws_idx += 1;
    }

    // Emit any remaining tokens.
    for p in i..tokens.len() {
        if !processed[p] {
            result_tokens.push(tokens[p].clone());
        }
    }

    let elaborated = result_tokens.join("");
    ElabNotationResult {
        original,
        elaborated,
        applied_notations: applied,
    }
}

// ────────────────────────────────────────────────────────────────────────────
// standard_notations
// ────────────────────────────────────────────────────────────────────────────

/// Build a `NotationDB` populated with standard mathematical notations.
///
/// Supported symbols:
/// `∧`, `∨`, `¬`, `→`, `↔`, `∀`, `∃`, `=`, `≠`, `≤`, `≥`,
/// `+`, `-`, `*`, `/`, `^`, `∘`, `∈`, `∉`, `⊆`, `⊂`
pub fn standard_notations() -> NotationDB {
    let mut db = NotationDB::new();

    // Logic operators.
    let _ = db.add(infix("∧", "And $0 $1", 35, Assoc::Right, 100));
    let _ = db.add(infix("∨", "Or $0 $1", 30, Assoc::Right, 100));
    let _ = db.add(prefix("¬", "Not $0", 40, 100));
    let _ = db.add(infix("→", "Implies $0 $1", 25, Assoc::Right, 100));
    let _ = db.add(infix("↔", "Iff $0 $1", 20, Assoc::None, 100));

    // Quantifiers (treat as prefix — binder syntax needs special parser support).
    let _ = db.add(prefix("∀", "Forall $0", 10, 100));
    let _ = db.add(prefix("∃", "Exists $0", 10, 100));

    // Equality / comparison.
    let _ = db.add(infix("=", "Eq $0 $1", 50, Assoc::None, 100));
    let _ = db.add(infix("≠", "Ne $0 $1", 50, Assoc::None, 100));
    let _ = db.add(infix("≤", "Le $0 $1", 50, Assoc::None, 100));
    let _ = db.add(infix("≥", "Ge $0 $1", 50, Assoc::None, 100));

    // Arithmetic.
    let _ = db.add(infix("+", "Add $0 $1", 65, Assoc::Left, 100));
    let _ = db.add(infix("-", "Sub $0 $1", 65, Assoc::Left, 100));
    let _ = db.add(infix("*", "Mul $0 $1", 70, Assoc::Left, 100));
    let _ = db.add(infix("/", "Div $0 $1", 70, Assoc::Left, 100));
    let _ = db.add(infix("^", "Pow $0 $1", 75, Assoc::Right, 100));

    // Function composition and set operations.
    let _ = db.add(infix("∘", "Comp $0 $1", 80, Assoc::Right, 100));
    let _ = db.add(infix("∈", "Mem $0 $1", 50, Assoc::None, 100));
    let _ = db.add(infix("∉", "NotMem $0 $1", 50, Assoc::None, 100));
    let _ = db.add(infix("⊆", "Subset $0 $1", 50, Assoc::None, 100));
    let _ = db.add(infix("⊂", "StrictSubset $0 $1", 50, Assoc::None, 100));

    db
}

// Helper constructors for NotationDef.
fn infix(sym: &str, trans: &str, prec: u32, assoc: Assoc, priority: i32) -> NotationDef {
    NotationDef {
        symbol: sym.to_string(),
        kind: NotationKind::Infix { prec, assoc },
        translation: trans.to_string(),
        priority,
    }
}

fn prefix(sym: &str, trans: &str, prec: u32, priority: i32) -> NotationDef {
    NotationDef {
        symbol: sym.to_string(),
        kind: NotationKind::Prefix { prec },
        translation: trans.to_string(),
        priority,
    }
}

// ────────────────────────────────────────────────────────────────────────────
// check_notation_precedence
// ────────────────────────────────────────────────────────────────────────────

/// Analyse `db` for ambiguous parse situations and return warning strings.
///
/// Two notations create an ambiguity when:
/// - Both are infix with the same precedence level but different associativity
///   (or one is `Assoc::None` while the other is left/right).
/// - A prefix and an infix share the same symbol (always ambiguous without
///   contextual parsing).
pub fn check_notation_precedence(db: &NotationDB) -> Vec<String> {
    let mut warnings = Vec::new();

    let n = db.notations.len();
    for i in 0..n {
        for j in (i + 1)..n {
            let a = &db.notations[i];
            let b = &db.notations[j];

            // Prefix + infix same symbol.
            if a.symbol == b.symbol {
                let a_prefix = matches!(a.kind, NotationKind::Prefix { .. });
                let b_infix = matches!(b.kind, NotationKind::Infix { .. });
                let a_infix = matches!(a.kind, NotationKind::Infix { .. });
                let b_prefix = matches!(b.kind, NotationKind::Prefix { .. });

                if (a_prefix && b_infix) || (a_infix && b_prefix) {
                    warnings.push(format!(
                        "ambiguous: '{}' is both prefix and infix",
                        a.symbol
                    ));
                }
            }

            // Two infix with same prec but different assoc.
            if let (
                NotationKind::Infix {
                    prec: pa,
                    assoc: aa,
                },
                NotationKind::Infix {
                    prec: pb,
                    assoc: ab,
                },
            ) = (&a.kind, &b.kind)
            {
                if pa == pb && aa != ab {
                    warnings.push(format!(
                        "ambiguous precedence: '{}' and '{}' both at prec {} with different associativity",
                        a.symbol, b.symbol, pa
                    ));
                }
            }
        }
    }

    warnings
}

// ────────────────────────────────────────────────────────────────────────────
// Tests
// ────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::notation_elaboration::types::*;

    fn mk_infix(sym: &str, trans: &str, prec: u32, assoc: Assoc) -> NotationDef {
        NotationDef {
            symbol: sym.to_string(),
            kind: NotationKind::Infix { prec, assoc },
            translation: trans.to_string(),
            priority: 0,
        }
    }

    fn mk_prefix(sym: &str, trans: &str, prec: u32) -> NotationDef {
        NotationDef {
            symbol: sym.to_string(),
            kind: NotationKind::Prefix { prec },
            translation: trans.to_string(),
            priority: 0,
        }
    }

    fn mk_mixfix(sym: &str, trans: &str, parts: Vec<MixfixPart>) -> NotationDef {
        NotationDef {
            symbol: sym.to_string(),
            kind: NotationKind::Mixfix { parts },
            translation: trans.to_string(),
            priority: 0,
        }
    }

    // ── NotationDB::new ──────────────────────────────────────────────────────

    #[test]
    fn test_db_new_empty() {
        let db = NotationDB::new();
        assert!(db.notations.is_empty());
    }

    // ── NotationDB::add ──────────────────────────────────────────────────────

    #[test]
    fn test_add_simple() {
        let mut db = NotationDB::new();
        let def = mk_infix("+", "Add $0 $1", 65, Assoc::Left);
        assert!(db.add(def).is_ok());
        assert_eq!(db.notations.len(), 1);
    }

    #[test]
    fn test_add_conflict() {
        let mut db = NotationDB::new();
        let a = NotationDef {
            symbol: "+".into(),
            kind: NotationKind::Infix {
                prec: 65,
                assoc: Assoc::Left,
            },
            translation: "Add $0 $1".into(),
            priority: 10,
        };
        let b = NotationDef {
            symbol: "+".into(),
            kind: NotationKind::Infix {
                prec: 65,
                assoc: Assoc::Left,
            },
            translation: "Plus $0 $1".into(), // different translation → conflict
            priority: 10,
        };
        assert!(db.add(a).is_ok());
        assert!(db.add(b).is_err());
    }

    #[test]
    fn test_add_same_translation_no_conflict() {
        let mut db = NotationDB::new();
        let a = NotationDef {
            symbol: "+".into(),
            kind: NotationKind::Infix {
                prec: 65,
                assoc: Assoc::Left,
            },
            translation: "Add $0 $1".into(),
            priority: 10,
        };
        let b = a.clone();
        assert!(db.add(a).is_ok());
        assert!(db.add(b).is_ok());
    }

    #[test]
    fn test_add_different_priority_no_conflict() {
        let mut db = NotationDB::new();
        let a = NotationDef {
            symbol: "+".into(),
            kind: NotationKind::Infix {
                prec: 65,
                assoc: Assoc::Left,
            },
            translation: "Add $0 $1".into(),
            priority: 10,
        };
        let b = NotationDef {
            priority: 20,
            ..a.clone()
        };
        assert!(db.add(a).is_ok());
        assert!(db.add(b).is_ok());
    }

    #[test]
    fn test_add_maintains_priority_order() {
        let mut db = NotationDB::new();
        let lo = NotationDef {
            symbol: "op".into(),
            kind: NotationKind::Prefix { prec: 10 },
            translation: "Lo $0".into(),
            priority: 1,
        };
        let hi = NotationDef {
            symbol: "op".into(),
            kind: NotationKind::Prefix { prec: 10 },
            translation: "Hi $0".into(),
            priority: 99,
        };
        let _ = db.add(lo);
        let _ = db.add(hi);
        // Higher priority should come first.
        assert!(db.notations[0].priority >= db.notations[1].priority);
    }

    // ── NotationDB::find_* ───────────────────────────────────────────────────

    #[test]
    fn test_find_prefix() {
        let mut db = NotationDB::new();
        let _ = db.add(mk_prefix("¬", "Not $0", 40));
        assert!(db.find_prefix("¬").is_some());
        assert!(db.find_prefix("→").is_none());
    }

    #[test]
    fn test_find_infix() {
        let mut db = NotationDB::new();
        let _ = db.add(mk_infix("∧", "And $0 $1", 35, Assoc::Right));
        assert!(db.find_infix("∧").is_some());
        assert!(db.find_infix("∨").is_none());
    }

    #[test]
    fn test_find_postfix() {
        let mut db = NotationDB::new();
        let def = NotationDef {
            symbol: "!".into(),
            kind: NotationKind::Postfix { prec: 90 },
            translation: "Factorial $0".into(),
            priority: 0,
        };
        let _ = db.add(def);
        assert!(db.find_postfix("!").is_some());
        assert!(db.find_postfix("?").is_none());
    }

    #[test]
    fn test_find_mixfix() {
        let mut db = NotationDB::new();
        let def = mk_mixfix(
            "if_then_else",
            "ite $0 $1 $2",
            vec![
                MixfixPart::Literal("if".into()),
                MixfixPart::Placeholder(0),
                MixfixPart::Literal("then".into()),
                MixfixPart::Placeholder(0),
                MixfixPart::Literal("else".into()),
                MixfixPart::Placeholder(0),
            ],
        );
        let _ = db.add(def);
        assert!(db.find_mixfix("if_then_else").is_some());
    }

    // ── resolve_mixfix ───────────────────────────────────────────────────────

    #[test]
    fn test_resolve_mixfix_basic() {
        let def = mk_mixfix(
            "if_then_else",
            "ite $0 $1 $2",
            vec![
                MixfixPart::Literal("if".into()),
                MixfixPart::Placeholder(0),
                MixfixPart::Literal("then".into()),
                MixfixPart::Placeholder(0),
                MixfixPart::Literal("else".into()),
                MixfixPart::Placeholder(0),
            ],
        );
        let tokens: Vec<String> = vec![
            "if".into(),
            "p".into(),
            "then".into(),
            "a".into(),
            "else".into(),
            "b".into(),
        ];
        let result = resolve_mixfix(&tokens, &def);
        assert!(result.is_some());
        let s = result.unwrap();
        assert!(s.contains("p"));
        assert!(s.contains("a"));
        assert!(s.contains("b"));
    }

    #[test]
    fn test_resolve_mixfix_no_match() {
        let def = mk_mixfix(
            "if_then_else",
            "ite $0 $1 $2",
            vec![
                MixfixPart::Literal("if".into()),
                MixfixPart::Placeholder(0),
                MixfixPart::Literal("then".into()),
                MixfixPart::Placeholder(0),
                MixfixPart::Literal("else".into()),
                MixfixPart::Placeholder(0),
            ],
        );
        // Missing "else" clause.
        let tokens: Vec<String> = vec!["if".into(), "p".into(), "then".into(), "a".into()];
        assert!(resolve_mixfix(&tokens, &def).is_none());
    }

    #[test]
    fn test_resolve_mixfix_wrong_kind() {
        let def = mk_infix("+", "Add $0 $1", 65, Assoc::Left);
        let tokens: Vec<String> = vec!["a".into(), "+".into(), "b".into()];
        assert!(resolve_mixfix(&tokens, &def).is_none());
    }

    #[test]
    fn test_resolve_mixfix_extra_tokens() {
        let def = mk_mixfix(
            "pair",
            "Pair $0 $1",
            vec![
                MixfixPart::Placeholder(0),
                MixfixPart::Literal(",".into()),
                MixfixPart::Placeholder(0),
            ],
        );
        // Extra token at end.
        let tokens: Vec<String> = vec!["a".into(), ",".into(), "b".into(), "c".into()];
        assert!(resolve_mixfix(&tokens, &def).is_none());
    }

    // ── elaborate_notation ───────────────────────────────────────────────────

    #[test]
    fn test_elaborate_empty() {
        let db = NotationDB::new();
        let result = elaborate_notation("", &db);
        assert_eq!(result.original, "");
        assert_eq!(result.elaborated, "");
        assert!(result.applied_notations.is_empty());
    }

    #[test]
    fn test_elaborate_no_match() {
        let db = NotationDB::new();
        let result = elaborate_notation("x + y", &db);
        assert_eq!(result.original, "x + y");
        assert!(result.applied_notations.is_empty());
    }

    #[test]
    fn test_elaborate_prefix_negation() {
        let db = standard_notations();
        // ¬ is prefix.
        let result = elaborate_notation("¬ P", &db);
        assert!(result.applied_notations.contains(&"¬".to_string()));
        assert!(result.elaborated.contains("Not"));
    }

    #[test]
    fn test_elaborate_infix_and() {
        let db = standard_notations();
        let result = elaborate_notation("P ∧ Q", &db);
        assert!(result.applied_notations.contains(&"∧".to_string()));
        assert!(result.elaborated.contains("And"));
    }

    #[test]
    fn test_elaborate_original_preserved() {
        let db = standard_notations();
        let src = "a ∨ b";
        let result = elaborate_notation(src, &db);
        assert_eq!(result.original, src);
    }

    // ── standard_notations ───────────────────────────────────────────────────

    #[test]
    fn test_standard_notations_count() {
        let db = standard_notations();
        // We register exactly 21 notations.
        assert_eq!(db.notations.len(), 21);
    }

    #[test]
    fn test_standard_has_arithmetic() {
        let db = standard_notations();
        assert!(db.find_infix("+").is_some());
        assert!(db.find_infix("-").is_some());
        assert!(db.find_infix("*").is_some());
        assert!(db.find_infix("/").is_some());
        assert!(db.find_infix("^").is_some());
    }

    #[test]
    fn test_standard_has_logic() {
        let db = standard_notations();
        assert!(db.find_infix("∧").is_some());
        assert!(db.find_infix("∨").is_some());
        assert!(db.find_prefix("¬").is_some());
        assert!(db.find_infix("→").is_some());
        assert!(db.find_infix("↔").is_some());
    }

    #[test]
    fn test_standard_has_set_ops() {
        let db = standard_notations();
        assert!(db.find_infix("∈").is_some());
        assert!(db.find_infix("∉").is_some());
        assert!(db.find_infix("⊆").is_some());
        assert!(db.find_infix("⊂").is_some());
    }

    #[test]
    fn test_standard_has_composition() {
        let db = standard_notations();
        assert!(db.find_infix("∘").is_some());
    }

    #[test]
    fn test_standard_has_quantifiers() {
        let db = standard_notations();
        assert!(db.find_prefix("∀").is_some());
        assert!(db.find_prefix("∃").is_some());
    }

    // ── check_notation_precedence ────────────────────────────────────────────

    #[test]
    fn test_check_prec_no_warnings_clean_db() {
        let db = NotationDB::new();
        let warnings = check_notation_precedence(&db);
        assert!(warnings.is_empty());
    }

    #[test]
    fn test_check_prec_ambiguous_assoc() {
        let mut db = NotationDB::new();
        // Two *different* infix symbols at the same precedence with different assoc
        // — parsing `a ⊕ b ⊞ c` would be ambiguous.
        let _ = db.add(mk_infix("⊕", "Xor $0 $1", 40, Assoc::Left));
        let _ = db.add(mk_infix("⊞", "Xor2 $0 $1", 40, Assoc::Right));
        let warnings = check_notation_precedence(&db);
        assert!(!warnings.is_empty());
    }

    #[test]
    fn test_check_prec_prefix_and_infix_same_sym() {
        let mut db = NotationDB::new();
        let _ = db.add(mk_prefix("-", "Neg $0", 80));
        let _ = db.add(mk_infix("-", "Sub $0 $1", 65, Assoc::Left));
        let warnings = check_notation_precedence(&db);
        assert!(warnings.iter().any(|w| w.contains("both prefix and infix")));
    }

    #[test]
    fn test_check_prec_different_prec_no_warning() {
        let mut db = NotationDB::new();
        let _ = db.add(mk_infix("+", "Add $0 $1", 65, Assoc::Left));
        let _ = db.add(mk_infix("*", "Mul $0 $1", 70, Assoc::Left));
        let warnings = check_notation_precedence(&db);
        assert!(warnings.is_empty());
    }

    // ── Display impls ────────────────────────────────────────────────────────

    #[test]
    fn test_assoc_display() {
        assert_eq!(format!("{}", Assoc::Left), "left");
        assert_eq!(format!("{}", Assoc::Right), "right");
        assert_eq!(format!("{}", Assoc::None), "none");
    }

    #[test]
    fn test_notation_kind_display() {
        let k = NotationKind::Infix {
            prec: 65,
            assoc: Assoc::Left,
        };
        assert!(format!("{}", k).contains("infix"));
        let k2 = NotationKind::Prefix { prec: 80 };
        assert!(format!("{}", k2).contains("prefix"));
    }

    #[test]
    fn test_notation_conflict_display() {
        let a = mk_infix("+", "Add $0 $1", 65, Assoc::Left);
        let b = mk_infix("+", "Plus $0 $1", 65, Assoc::Left);
        let conflict = NotationConflict {
            symbol: "+".into(),
            existing: a,
            new_: b,
        };
        let s = format!("{}", conflict);
        assert!(s.contains("conflict"));
        assert!(s.contains("+"));
    }

    #[test]
    fn test_mixfix_part_display() {
        assert_eq!(format!("{}", MixfixPart::Literal("if".into())), "if");
        assert_eq!(format!("{}", MixfixPart::Placeholder(10)), "_10");
    }
}
