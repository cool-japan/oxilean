//! Tests for the duplicate-name warnings of `validate_locals`.

use super::super::types::LocalEntry;
use super::validate_locals;
use oxilean_kernel::{Expr, FVarId, Name};

fn hyp(id: u64, name: &str) -> LocalEntry {
    LocalEntry::hypothesis(
        FVarId(id),
        Name::str(name),
        Expr::Const(Name::str("Nat"), vec![]),
        0,
    )
}

#[test]
fn distinct_names_of_equal_length_raise_no_warning() {
    let entries: Vec<LocalEntry> = ["a", "b", "c", "d", "e", "f", "g", "h"]
        .iter()
        .enumerate()
        .map(|(i, n)| hyp(i as u64, n))
        .collect();
    let result = validate_locals(&entries);
    assert!(result.errors.is_empty());
    assert!(result.warnings.is_empty(), "{:?}", result.warnings);
}

#[test]
fn repeated_names_are_counted() {
    let entries = vec![
        hyp(1, "x"),
        hyp(2, "y"),
        hyp(3, "x"),
        hyp(4, "zz"),
        hyp(5, "x"),
        hyp(6, "zz"),
    ];
    let result = validate_locals(&entries);
    assert!(result.errors.is_empty());
    let mut warnings = result.warnings.clone();
    warnings.sort();
    assert_eq!(
        warnings,
        vec![
            "Name 'x' appears 3 times in context".to_string(),
            "Name 'zz' appears 2 times in context".to_string(),
        ]
    );
}
