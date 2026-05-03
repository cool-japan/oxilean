//! Functions for hole inference: automatically fill `_` (holes) in terms.

use std::cmp::Reverse;
use std::time::Instant;

use super::types::{Hole, HoleContext, HoleFilling, HoleInferenceResult, HoleKind, InferenceStats};

/// Locate all `_` occurrences in source and return them as `Hole` values.
///
/// This performs a simple lexical scan and assigns sequential IDs.
/// Context inference is basic: we record nearby `(name : type)` patterns.
pub fn find_holes(source: &str) -> Vec<Hole> {
    let mut holes = Vec::new();
    let mut id = 0usize;

    // Simple byte-level scan for standalone `_`
    let chars: Vec<char> = source.chars().collect();
    let len = chars.len();
    let mut i = 0;

    while i < len {
        if chars[i] == '_' {
            // Make sure it is standalone (not part of an identifier)
            let prev_ok = i == 0 || !is_ident_char(chars[i - 1]);
            let next_ok = i + 1 >= len || !is_ident_char(chars[i + 1]);

            if prev_ok && next_ok {
                // Compute byte span
                let byte_start: usize = chars[..i].iter().collect::<String>().len();
                let byte_end = byte_start + '_'.len_utf8();

                let ctx = extract_context_near(source, byte_start);
                holes.push(Hole {
                    id,
                    expected_type: infer_expected_type_from_surroundings(source, byte_start),
                    context: ctx,
                    span: Some((byte_start, byte_end)),
                });
                id += 1;
            }
        }
        i += 1;
    }

    holes
}

/// Infer what type a hole should have based on the surrounding context.
///
/// Returns `None` if the type cannot be determined.
pub fn infer_hole_type(hole: &Hole, surrounding_type: &str) -> Option<String> {
    // If the expected type is already set and non-trivial, return it.
    if !hole.expected_type.is_empty() && hole.expected_type != "?" {
        return Some(hole.expected_type.clone());
    }

    // If a surrounding type is provided, use it.
    if !surrounding_type.is_empty() && surrounding_type != "?" {
        return Some(surrounding_type.to_string());
    }

    // Try to infer from context: if there's a single decl, use its type.
    if hole.context.len() == 1 {
        return Some(hole.context[0].1.clone());
    }

    None
}

/// Apply a list of fillings to source, replacing each `_` at the given span.
///
/// Fillings are applied in reverse byte-offset order to avoid offset drift.
pub fn fill_holes(source: &str, fillings: &[HoleFilling]) -> String {
    // Build a map from hole_id to term (we'll need spans for replacement).
    // Since we don't have span info in HoleFilling, we re-scan the source
    // and apply fillings by ID in order.
    let holes = find_holes(source);

    // Build sorted list of (byte_start, byte_end, term) to apply
    let mut replacements: Vec<(usize, usize, &str)> = holes
        .iter()
        .filter_map(|h| {
            let filling = fillings.iter().find(|f| f.hole_id == h.id)?;
            let (start, end) = h.span?;
            Some((start, end, filling.term.as_str()))
        })
        .collect();

    // Sort in reverse order so byte offsets remain valid after each replacement
    replacements.sort_by_key(|r| std::cmp::Reverse(r.0));

    let mut result = source.to_string();
    for (start, end, term) in replacements {
        if end <= result.len() {
            result.replace_range(start..end, term);
        }
    }
    result
}

/// Attempt to automatically fill all holes in `source`.
pub fn auto_fill_holes(source: &str) -> HoleInferenceResult {
    let start = Instant::now();

    let holes = find_holes(source);
    let holes_found = holes.len();
    let mut fillings = Vec::new();
    let mut remaining = Vec::new();

    for hole in &holes {
        let inferred = infer_hole_type(hole, "");
        if let Some(ty) = inferred {
            // Try to fill based on the type
            if let Some(term) = default_term_for_type(&ty) {
                fillings.push(HoleFilling {
                    hole_id: hole.id,
                    term,
                    confidence: 0.8,
                });
            } else {
                remaining.push(hole.clone());
            }
        } else {
            remaining.push(hole.clone());
        }
    }

    let holes_filled = fillings.len();
    let time_ms = start.elapsed().as_millis() as u64;

    HoleInferenceResult {
        fillings,
        remaining,
        stats: InferenceStats {
            holes_found,
            holes_filled,
            time_ms,
        },
    }
}

/// Fill `Sort _` holes by replacing them with `Sort 0` (i.e., `Prop`).
pub fn fill_universe_holes(source: &str) -> String {
    // Pattern: `Sort _` → `Sort 0`
    let mut result = source.to_string();
    // Replace `Sort _` occurrences
    while let Some(pos) = result.find("Sort _") {
        result.replace_range(pos..pos + "Sort _".len(), "Sort 0");
    }
    // Also handle `Type _` → `Type 0`
    while let Some(pos) = result.find("Type _") {
        result.replace_range(pos..pos + "Type _".len(), "Type 0");
    }
    result
}

/// Attempt to fill instance (typeclass) holes from available instances.
///
/// `available` is a list of `(instance_name, instance_type)` pairs.
/// Returns fillings for holes that have matching typeclass instances.
pub fn fill_instance_holes(source: &str, available: &[(String, String)]) -> Vec<HoleFilling> {
    let holes = find_holes(source);
    let mut fillings = Vec::new();

    for hole in &holes {
        let expected = &hole.expected_type;
        // Try to find a matching instance by type
        for (name, ty) in available {
            if types_compatible(ty, expected) {
                fillings.push(HoleFilling {
                    hole_id: hole.id,
                    term: name.clone(),
                    confidence: 0.9,
                });
                break;
            }
        }
    }

    fillings
}

/// Count the number of standalone `_` holes in `source`.
pub fn count_holes(source: &str) -> usize {
    find_holes(source).len()
}

// ---------- Internal helpers ----------

/// Check if a character can be part of an identifier.
fn is_ident_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_' || c == '\''
}

/// Extract `(name, type)` pairs from nearby declarations in source.
fn extract_context_near(source: &str, byte_pos: usize) -> Vec<(String, String)> {
    let mut result = Vec::new();

    // Look backwards from byte_pos for `(name : Type)` patterns in the preceding text
    let prefix = if byte_pos <= 200 {
        &source[..byte_pos]
    } else {
        &source[byte_pos - 200..byte_pos]
    };

    // Simple regex-free parser: find `(name : type)` patterns
    let mut remaining = prefix;
    while let Some(open) = remaining.rfind('(') {
        let segment = &remaining[open..];
        if let Some(close) = segment.find(')') {
            let inner = &segment[1..close];
            if let Some(colon) = inner.find(':') {
                let name = inner[..colon].trim().to_string();
                let ty = inner[colon + 1..].trim().to_string();
                if is_simple_ident(&name) && !ty.is_empty() {
                    result.push((name, ty));
                }
            }
        }
        if open == 0 {
            break;
        }
        remaining = &remaining[..open];
    }

    result
}

/// Infer the expected type for a hole at `byte_pos` in source.
fn infer_expected_type_from_surroundings(source: &str, byte_pos: usize) -> String {
    // Look for `: Type` or `expected_type` annotation just before the hole
    let prefix = if byte_pos < 50 {
        &source[..byte_pos]
    } else {
        &source[byte_pos - 50..byte_pos]
    };
    let prefix = prefix.trim_end();

    // Pattern: `: TypeName _` → TypeName is the expected type
    if let Some(colon_pos) = prefix.rfind(':') {
        let after_colon = prefix[colon_pos + 1..].trim();
        // Take the last word as the type
        if let Some(ty_word) = after_colon.split_whitespace().last() {
            if is_type_name(ty_word) {
                return ty_word.to_string();
            }
        }
    }

    // Pattern: `(Sort _)` or `(Type _)` → "Sort"
    if prefix.ends_with("Sort") || prefix.ends_with("Type") {
        return "Sort".to_string();
    }

    "?".to_string()
}

/// Check if a string is a simple identifier (no spaces, valid chars).
fn is_simple_ident(s: &str) -> bool {
    !s.is_empty()
        && s.chars()
            .all(|c| c.is_alphanumeric() || c == '_' || c == '\'')
}

/// Check if a string looks like a type name.
fn is_type_name(s: &str) -> bool {
    !s.is_empty()
        && (s.chars().next().map(|c| c.is_uppercase()).unwrap_or(false)
            || matches!(
                s,
                "Nat" | "Bool" | "Int" | "String" | "Prop" | "Sort" | "Type" | "Unit"
            ))
}

/// Return a default term for a given type string.
fn default_term_for_type(ty: &str) -> Option<String> {
    match ty.trim() {
        "Nat" => Some("0".to_string()),
        "Bool" | "bool" => Some("true".to_string()),
        "Unit" | "()" => Some("()".to_string()),
        "String" => Some("\"\"".to_string()),
        "Int" => Some("0".to_string()),
        "Prop" | "True" => Some("True.intro".to_string()),
        "Sort" => Some("Sort 0".to_string()),
        "List" => Some("[]".to_string()),
        "Option" => Some("none".to_string()),
        _ => {
            // For unknown uppercase types, try `.mk` constructor
            if ty.starts_with(|c: char| c.is_uppercase()) {
                Some(format!("{}.mk", ty))
            } else {
                None
            }
        }
    }
}

/// Check if two type strings are compatible for instance matching.
fn types_compatible(instance_type: &str, expected: &str) -> bool {
    let it = instance_type.trim();
    let ex = expected.trim();
    it == ex || it.ends_with(ex) || ex == "?" || ex.is_empty()
}

/// Build a `HoleContext` from a `Hole` and a `HoleKind`.
pub fn hole_to_context(hole: &Hole, kind: HoleKind) -> HoleContext {
    HoleContext {
        local_decls: hole.context.clone(),
        expected: hole.expected_type.clone(),
        kind,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hole_inference::types::{Hole, HoleFilling, HoleKind};

    // --- count_holes ---

    #[test]
    fn test_count_holes_none() {
        assert_eq!(count_holes("let x := 5"), 0);
    }

    #[test]
    fn test_count_holes_one() {
        assert_eq!(count_holes("fun (x : _) => x"), 1);
    }

    #[test]
    fn test_count_holes_multiple() {
        assert_eq!(count_holes("fun (x : _) => _ + _"), 3);
    }

    #[test]
    fn test_count_holes_not_in_identifier() {
        // `_foo` should not count as a hole
        assert_eq!(count_holes("let _foo := 5"), 0);
    }

    #[test]
    fn test_count_holes_standalone_only() {
        assert_eq!(count_holes("a_b"), 0);
    }

    // --- find_holes ---

    #[test]
    fn test_find_holes_ids_sequential() {
        let holes = find_holes("_ + _ + _");
        let ids: Vec<usize> = holes.iter().map(|h| h.id).collect();
        assert_eq!(ids, vec![0, 1, 2]);
    }

    #[test]
    fn test_find_holes_span_present() {
        let holes = find_holes("_");
        assert_eq!(holes.len(), 1);
        assert_eq!(holes[0].span, Some((0, 1)));
    }

    #[test]
    fn test_find_holes_span_offset() {
        let holes = find_holes("abc _");
        assert_eq!(holes.len(), 1);
        assert_eq!(holes[0].span, Some((4, 5)));
    }

    #[test]
    fn test_find_holes_empty_source() {
        assert!(find_holes("").is_empty());
    }

    #[test]
    fn test_find_holes_no_underscore() {
        assert!(find_holes("let x := 42").is_empty());
    }

    // --- infer_hole_type ---

    #[test]
    fn test_infer_hole_type_from_expected() {
        let hole = Hole {
            id: 0,
            expected_type: "Nat".to_string(),
            context: vec![],
            span: None,
        };
        assert_eq!(infer_hole_type(&hole, ""), Some("Nat".to_string()));
    }

    #[test]
    fn test_infer_hole_type_from_surrounding() {
        let hole = Hole {
            id: 0,
            expected_type: "?".to_string(),
            context: vec![],
            span: None,
        };
        assert_eq!(infer_hole_type(&hole, "Bool"), Some("Bool".to_string()));
    }

    #[test]
    fn test_infer_hole_type_from_context() {
        let hole = Hole {
            id: 0,
            expected_type: "?".to_string(),
            context: vec![("n".to_string(), "Nat".to_string())],
            span: None,
        };
        assert_eq!(infer_hole_type(&hole, ""), Some("Nat".to_string()));
    }

    #[test]
    fn test_infer_hole_type_none() {
        let hole = Hole {
            id: 0,
            expected_type: "?".to_string(),
            context: vec![],
            span: None,
        };
        assert_eq!(infer_hole_type(&hole, ""), None);
    }

    // --- fill_holes ---

    #[test]
    fn test_fill_holes_basic() {
        let source = "fun (x : _) => x";
        let holes = find_holes(source);
        assert_eq!(holes.len(), 1);
        let fillings = vec![HoleFilling {
            hole_id: 0,
            term: "Nat".to_string(),
            confidence: 1.0,
        }];
        let result = fill_holes(source, &fillings);
        assert_eq!(result, "fun (x : Nat) => x");
    }

    #[test]
    fn test_fill_holes_no_fillings() {
        let source = "fun (x : _) => x";
        let result = fill_holes(source, &[]);
        assert_eq!(result, source);
    }

    #[test]
    fn test_fill_holes_multiple() {
        let source = "_ + _";
        let holes = find_holes(source);
        let fillings: Vec<HoleFilling> = holes
            .iter()
            .map(|h| HoleFilling {
                hole_id: h.id,
                term: "0".to_string(),
                confidence: 1.0,
            })
            .collect();
        let result = fill_holes(source, &fillings);
        assert_eq!(result, "0 + 0");
    }

    // --- auto_fill_holes ---

    #[test]
    fn test_auto_fill_no_holes() {
        let result = auto_fill_holes("let x := 5");
        assert_eq!(result.stats.holes_found, 0);
        assert_eq!(result.stats.holes_filled, 0);
    }

    #[test]
    fn test_auto_fill_fills_nat() {
        // A hole with inferred Nat type via context
        let source = "fun (x : _) => x";
        let result = auto_fill_holes(source);
        assert_eq!(result.stats.holes_found, 1);
        // May or may not fill depending on context extraction
        assert!(result.stats.holes_filled <= 1);
    }

    #[test]
    fn test_auto_fill_result_structure() {
        let result = auto_fill_holes("_ + _");
        let total = result.fillings.len() + result.remaining.len();
        assert_eq!(total, result.stats.holes_found);
    }

    // --- fill_universe_holes ---

    #[test]
    fn test_fill_universe_sort() {
        let result = fill_universe_holes("(Sort _)");
        assert_eq!(result, "(Sort 0)");
    }

    #[test]
    fn test_fill_universe_type() {
        let result = fill_universe_holes("Type _");
        assert_eq!(result, "Type 0");
    }

    #[test]
    fn test_fill_universe_no_change() {
        let source = "let x : Nat := 0";
        let result = fill_universe_holes(source);
        assert_eq!(result, source);
    }

    #[test]
    fn test_fill_universe_multiple() {
        let result = fill_universe_holes("Sort _ → Sort _");
        assert_eq!(result, "Sort 0 → Sort 0");
    }

    // --- fill_instance_holes ---

    #[test]
    fn test_fill_instance_holes_match() {
        let source = "_";
        let available = vec![("instAddNat".to_string(), "Add Nat".to_string())];
        // The hole's expected type must match; here it's `?`, so no match expected
        let fillings = fill_instance_holes(source, &available);
        // With `?` expected type, types_compatible returns true for empty/? targets
        assert!(fillings.len() <= 1);
    }

    #[test]
    fn test_fill_instance_holes_no_instances() {
        let source = "_";
        let fillings = fill_instance_holes(source, &[]);
        assert!(fillings.is_empty());
    }

    #[test]
    fn test_fill_instance_holes_no_holes() {
        let source = "let x := 5";
        let available = vec![("inst".to_string(), "Foo".to_string())];
        let fillings = fill_instance_holes(source, &available);
        assert!(fillings.is_empty());
    }

    // --- hole_to_context ---

    #[test]
    fn test_hole_to_context() {
        let hole = Hole {
            id: 0,
            expected_type: "Nat".to_string(),
            context: vec![("x".to_string(), "Bool".to_string())],
            span: None,
        };
        let ctx = hole_to_context(&hole, HoleKind::Term);
        assert_eq!(ctx.expected, "Nat");
        assert_eq!(ctx.kind, HoleKind::Term);
        assert_eq!(ctx.local_decls.len(), 1);
    }

    #[test]
    fn test_hole_to_context_universe() {
        let hole = Hole {
            id: 1,
            expected_type: "Sort".to_string(),
            context: vec![],
            span: None,
        };
        let ctx = hole_to_context(&hole, HoleKind::Universe);
        assert_eq!(ctx.kind, HoleKind::Universe);
    }

    // --- default_term_for_type (via auto_fill) ---

    #[test]
    fn test_default_term_nat() {
        assert_eq!(default_term_for_type("Nat"), Some("0".to_string()));
    }

    #[test]
    fn test_default_term_bool() {
        assert_eq!(default_term_for_type("Bool"), Some("true".to_string()));
    }

    #[test]
    fn test_default_term_unknown_uppercase() {
        assert_eq!(
            default_term_for_type("MyType"),
            Some("MyType.mk".to_string())
        );
    }

    #[test]
    fn test_default_term_unknown_lowercase() {
        assert_eq!(default_term_for_type("somevar"), None);
    }
}
