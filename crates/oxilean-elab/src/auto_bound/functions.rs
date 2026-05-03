//! Functions for auto-bound implicit variable handling.

use super::types::{AutoBoundConfig, AutoBoundError, AutoBoundKind, AutoBoundResult, AutoBoundVar};

// ─── helpers ────────────────────────────────────────────────────────────────

/// Unicode Greek-letter lowercase set used for type-variable detection.
const GREEK_LOWER: &[&str] = &[
    "α", "β", "γ", "δ", "ε", "ζ", "η", "θ", "ι", "κ", "λ", "μ", "ν", "ξ", "ο", "π", "ρ", "σ", "τ",
    "υ", "φ", "χ", "ψ", "ω",
];

// ─── classifier helpers ─────────────────────────────────────────────────────

/// Returns `true` when `name` looks like a universe-level variable.
///
/// Matches: `u`, `v`, `u1`–`u9`, `v1`–`v9`.
pub fn is_universe_var(name: &str) -> bool {
    if name == "u" || name == "v" {
        return true;
    }
    if name.len() == 2 {
        let bytes = name.as_bytes();
        if (bytes[0] == b'u' || bytes[0] == b'v') && bytes[1].is_ascii_digit() {
            return true;
        }
    }
    false
}

/// Returns `true` when `name` looks like a type variable.
///
/// Matches: single lowercase ASCII letter `a`–`z` (excluding those claimed by
/// universe or natural-number categories), or a lowercase Greek letter.
pub fn is_type_var(name: &str) -> bool {
    // Greek letters are unambiguously type variables.
    if GREEK_LOWER.contains(&name) {
        return true;
    }
    // Single ASCII letters: exclude universe vars (u, v), Nat vars (n,m,k,i,j)
    // and proposition vars (h, p, q).
    if name.len() == 1 {
        match name.chars().next() {
            Some(c) if c.is_ascii_lowercase() => {
                !matches!(c, 'u' | 'v' | 'n' | 'm' | 'k' | 'i' | 'j' | 'h' | 'p' | 'q')
            }
            _ => false,
        }
    } else {
        false
    }
}

/// Returns `true` when `name` looks like a natural-number variable.
///
/// Matches: `n`, `m`, `k`, `i`, `j`.
pub fn is_nat_var(name: &str) -> bool {
    matches!(name, "n" | "m" | "k" | "i" | "j")
}

/// Classify an auto-bound name and return the expected Lean type string, or
/// `None` if we cannot confidently assign a type.
pub fn classify_auto_bound(name: &str) -> Option<String> {
    if is_universe_var(name) {
        Some("Level".to_string())
    } else if is_type_var(name) {
        Some("Type*".to_string())
    } else if is_nat_var(name) {
        Some("Nat".to_string())
    } else if matches!(name, "h" | "p" | "q") {
        Some("Prop".to_string())
    } else {
        None
    }
}

// ─── main API ───────────────────────────────────────────────────────────────

/// Tokenise `type_expr` and collect every identifier that looks like a
/// free variable (i.e. not in `bound`).
///
/// This is a heuristic lexer: it splits on whitespace and punctuation and
/// collects tokens that are valid Lean identifiers.
pub fn find_free_names(type_expr: &str, bound: &[String]) -> Vec<String> {
    let mut free: Vec<String> = Vec::new();
    // Lex: split on common punctuation.
    let tokens = tokenise(type_expr);
    for tok in &tokens {
        if is_lean_ident(tok) && !bound.contains(tok) && !free.contains(tok) {
            free.push(tok.clone());
        }
    }
    free
}

/// Heuristic: given a variable `name` and some usage `usage_context`, infer
/// the most likely Lean type.
///
/// Rules (in priority order):
/// 1. Context hint: if `usage_context` contains `name : <Type>`, use that.
/// 2. Greek letters (`α`, `β`, `γ`) → `Type*`
/// 3. `n`, `m`, `k`, `i`, `j` → `Nat`
/// 4. `h`, `p`, `q` → `Prop`
/// 5. Universe vars (`u`, `v`, `u1`, …) → `Level`
/// 6. Single-letter type var → `Type*`
/// 7. Otherwise `None`
pub fn infer_auto_bound_type(name: &str, usage_context: &str) -> Option<String> {
    // Context-based heuristic takes highest priority.
    if usage_context.contains(&format!("{name} : Nat"))
        || usage_context.contains(&format!("{name}: Nat"))
    {
        return Some("Nat".to_string());
    }
    if usage_context.contains(&format!("{name} : Type"))
        || usage_context.contains(&format!("{name}: Type"))
    {
        return Some("Type*".to_string());
    }
    if usage_context.contains(&format!("{name} : Prop"))
        || usage_context.contains(&format!("{name}: Prop"))
    {
        return Some("Prop".to_string());
    }
    // Fall back to name-only classification.
    classify_auto_bound(name)
}

/// Process a declaration type string, find all free names, and return an
/// [`AutoBoundResult`] with the implicit variables prepended to the type.
pub fn insert_auto_bounds(
    decl_type: &str,
    cfg: &AutoBoundConfig,
) -> Result<AutoBoundResult, AutoBoundError> {
    if !cfg.enable {
        return Ok(AutoBoundResult::unchanged(decl_type));
    }

    let bound: Vec<String> = cfg.ignore_names.clone();
    let free_names = find_free_names(decl_type, &bound);

    // Filter: exclude ignored names and known keywords.
    let candidates: Vec<String> = free_names
        .into_iter()
        .filter(|n| !cfg.is_ignored(n) && !is_lean_keyword(n))
        .collect();

    // Enforce max.
    if candidates.len() > cfg.max_auto_vars {
        return Err(AutoBoundError::TooManyVars {
            count: candidates.len(),
            max: cfg.max_auto_vars,
        });
    }

    if candidates.is_empty() {
        return Ok(AutoBoundResult::unchanged(decl_type));
    }

    // Build AutoBoundVar list.
    let mut vars: Vec<AutoBoundVar> = candidates
        .iter()
        .enumerate()
        .map(|(i, name)| {
            let inferred = infer_auto_bound_type(name, decl_type);
            let col = find_first_col(decl_type, name).unwrap_or(i as u32);
            let mut v = AutoBoundVar::new(name.clone(), (0, col));
            v.inferred_type = inferred;
            // Count occurrences.
            v.usage_count = decl_type.matches(name.as_str()).count();
            v
        })
        .collect();

    // Sort.
    if cfg.sort_alphabetically {
        vars.sort_by(|a, b| a.name.cmp(&b.name));
    } else {
        reorder_auto_bounds(&mut vars);
    }

    // Build the modified type: prepend `{name : Type} → …`.
    let prefix: String = vars
        .iter()
        .map(|v| match &v.inferred_type {
            Some(ty) => format!("{{{} : {}}} → ", v.name, ty),
            None => format!("{{{}}} → ", v.name),
        })
        .collect::<Vec<_>>()
        .join("");

    let modified_type = format!("{prefix}{decl_type}");

    Ok(AutoBoundResult {
        added_implicits: vars,
        modified_type,
        modified_body: None,
    })
}

/// Sort `vars` in-place: universe variables first, type variables second, term
/// variables last.  Within each group, preserve insertion order.
pub fn reorder_auto_bounds(vars: &mut Vec<AutoBoundVar>) {
    vars.sort_by_key(|v| classify_kind(&v.name));
}

// ─── internal helpers ────────────────────────────────────────────────────────

/// Classify the kind of an auto-bound variable name for ordering purposes.
fn classify_kind(name: &str) -> AutoBoundKind {
    if is_universe_var(name) {
        AutoBoundKind::Universe
    } else if is_type_var(name) {
        AutoBoundKind::TypeVar
    } else {
        AutoBoundKind::TermVar
    }
}

/// Simple Lean identifier check: starts with a letter or underscore, followed
/// by alphanumerics, underscores, or Unicode letters.
fn is_lean_ident(s: &str) -> bool {
    let mut chars = s.chars();
    match chars.next() {
        Some(c) if c.is_alphabetic() || c == '_' => {
            chars.all(|c| c.is_alphanumeric() || c == '_' || c == '\'')
        }
        _ => false,
    }
}

/// Very small set of Lean 4 keywords that should never be auto-bound.
fn is_lean_keyword(s: &str) -> bool {
    matches!(
        s,
        "def"
            | "theorem"
            | "lemma"
            | "example"
            | "where"
            | "let"
            | "fun"
            | "match"
            | "with"
            | "if"
            | "then"
            | "else"
            | "return"
            | "do"
            | "by"
            | "have"
            | "show"
            | "from"
            | "import"
            | "open"
            | "namespace"
            | "end"
            | "class"
            | "structure"
            | "instance"
            | "inductive"
            | "Type"
            | "Prop"
            | "Sort"
            | "Nat"
            | "Int"
            | "Bool"
            | "String"
            | "List"
            | "true"
            | "false"
    )
}

/// Tokenise `expr` into identifier-like tokens.
fn tokenise(expr: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    for ch in expr.chars() {
        if ch.is_alphanumeric() || ch == '_' || ch == '\'' || ch > '\x7f' {
            current.push(ch);
        } else {
            if !current.is_empty() {
                tokens.push(std::mem::take(&mut current));
            }
        }
    }
    if !current.is_empty() {
        tokens.push(current);
    }
    tokens
}

/// Find the column offset of the first occurrence of `name` in `text`.
fn find_first_col(text: &str, name: &str) -> Option<u32> {
    text.find(name)
        .map(|byte_pos| text[..byte_pos].chars().count() as u32)
}

// ─── tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::auto_bound::types::{AutoBoundConfig, AutoBoundError};

    // --- is_universe_var ---

    #[test]
    fn test_universe_var_u() {
        assert!(is_universe_var("u"));
    }

    #[test]
    fn test_universe_var_v() {
        assert!(is_universe_var("v"));
    }

    #[test]
    fn test_universe_var_u1() {
        assert!(is_universe_var("u1"));
    }

    #[test]
    fn test_universe_var_v9() {
        assert!(is_universe_var("v9"));
    }

    #[test]
    fn test_universe_var_not_w() {
        assert!(!is_universe_var("w"));
    }

    #[test]
    fn test_universe_var_not_alpha() {
        assert!(!is_universe_var("α"));
    }

    // --- is_type_var ---

    #[test]
    fn test_type_var_alpha() {
        assert!(is_type_var("α"));
    }

    #[test]
    fn test_type_var_beta() {
        assert!(is_type_var("β"));
    }

    #[test]
    fn test_type_var_single_a() {
        assert!(is_type_var("a"));
    }

    #[test]
    fn test_type_var_not_n() {
        // n is a Nat var
        assert!(!is_type_var("n"));
    }

    #[test]
    fn test_type_var_not_h() {
        // h is a Prop var
        assert!(!is_type_var("h"));
    }

    // --- is_nat_var ---

    #[test]
    fn test_nat_var_n() {
        assert!(is_nat_var("n"));
    }

    #[test]
    fn test_nat_var_m() {
        assert!(is_nat_var("m"));
    }

    #[test]
    fn test_nat_var_k() {
        assert!(is_nat_var("k"));
    }

    #[test]
    fn test_nat_var_not_x() {
        assert!(!is_nat_var("x"));
    }

    // --- classify_auto_bound ---

    #[test]
    fn test_classify_alpha_gives_type() {
        assert_eq!(classify_auto_bound("α"), Some("Type*".to_string()));
    }

    #[test]
    fn test_classify_n_gives_nat() {
        assert_eq!(classify_auto_bound("n"), Some("Nat".to_string()));
    }

    #[test]
    fn test_classify_h_gives_prop() {
        assert_eq!(classify_auto_bound("h"), Some("Prop".to_string()));
    }

    #[test]
    fn test_classify_u_gives_level() {
        assert_eq!(classify_auto_bound("u"), Some("Level".to_string()));
    }

    #[test]
    fn test_classify_unknown_returns_none() {
        assert_eq!(classify_auto_bound("myVar"), None);
    }

    // --- find_free_names ---

    #[test]
    fn test_find_free_names_basic() {
        let names = find_free_names("α → β → γ", &[]);
        assert!(names.contains(&"α".to_string()));
        assert!(names.contains(&"β".to_string()));
        assert!(names.contains(&"γ".to_string()));
    }

    #[test]
    fn test_find_free_names_excludes_bound() {
        let bound = vec!["α".to_string()];
        let names = find_free_names("α → β", &bound);
        assert!(!names.contains(&"α".to_string()));
        assert!(names.contains(&"β".to_string()));
    }

    #[test]
    fn test_find_free_names_no_duplicates() {
        let names = find_free_names("n → n → n", &[]);
        assert_eq!(names.iter().filter(|s| s.as_str() == "n").count(), 1);
    }

    // --- infer_auto_bound_type ---

    #[test]
    fn test_infer_type_alpha() {
        assert_eq!(infer_auto_bound_type("α", ""), Some("Type*".to_string()));
    }

    #[test]
    fn test_infer_type_n() {
        assert_eq!(infer_auto_bound_type("n", ""), Some("Nat".to_string()));
    }

    #[test]
    fn test_infer_type_context_hint() {
        assert_eq!(
            infer_auto_bound_type("x", "x : Nat"),
            Some("Nat".to_string())
        );
    }

    // --- insert_auto_bounds ---

    #[test]
    fn test_insert_auto_bounds_basic() {
        let cfg = AutoBoundConfig::default_config();
        let result = insert_auto_bounds("α → Nat", &cfg).expect("should succeed");
        assert!(result.has_additions());
        assert!(result.modified_type.contains("{α"));
    }

    #[test]
    fn test_insert_auto_bounds_disabled() {
        let cfg = AutoBoundConfig::disabled();
        let result = insert_auto_bounds("α → Nat", &cfg).expect("should succeed");
        assert!(!result.has_additions());
        assert_eq!(result.modified_type, "α → Nat");
    }

    #[test]
    fn test_insert_auto_bounds_too_many() {
        let mut cfg = AutoBoundConfig::default_config();
        cfg.max_auto_vars = 1;
        // α and β are both free
        let err = insert_auto_bounds("α → β → γ", &cfg).expect_err("should fail");
        assert!(matches!(err, AutoBoundError::TooManyVars { .. }));
    }

    #[test]
    fn test_insert_auto_bounds_ignore_list() {
        let mut cfg = AutoBoundConfig::default_config();
        cfg.ignore_names.push("α".to_string());
        let result = insert_auto_bounds("α → β", &cfg).expect("should succeed");
        // α is ignored, only β should appear
        assert!(!result.added_implicits.iter().any(|v| v.name == "α"));
    }

    #[test]
    fn test_insert_auto_bounds_alpha_sorted() {
        let mut cfg = AutoBoundConfig::default_config();
        cfg.sort_alphabetically = true;
        let result = insert_auto_bounds("β → α", &cfg).expect("should succeed");
        let names: Vec<&str> = result
            .added_implicits
            .iter()
            .map(|v| v.name.as_str())
            .collect();
        // Alphabetically α < β
        let mut sorted = names.clone();
        sorted.sort();
        assert_eq!(names, sorted);
    }

    // --- reorder_auto_bounds ---

    #[test]
    fn test_reorder_universe_first() {
        let mut vars = vec![
            AutoBoundVar::new("n", (0, 0)),
            AutoBoundVar::new("α", (0, 1)),
            AutoBoundVar::new("u", (0, 2)),
        ];
        reorder_auto_bounds(&mut vars);
        assert_eq!(vars[0].name, "u");
        assert_eq!(vars[1].name, "α");
        assert_eq!(vars[2].name, "n");
    }

    #[test]
    fn test_no_free_names_returns_unchanged() {
        let cfg = AutoBoundConfig::default_config();
        let result = insert_auto_bounds("Nat → Nat", &cfg).expect("should succeed");
        assert!(!result.has_additions());
    }
}
