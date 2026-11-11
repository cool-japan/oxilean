//! `default_pattern_for_inductive` for a sum type whose constructors have zero,
//! one and several fields.

use super::*;

fn well_known(name: &str) -> InductiveInfo {
    match get_well_known_inductive_info(name) {
        Some(info) => info,
        None => panic!("{name} must be a well-known inductive"),
    }
}

#[test]
fn single_field_constructor_gives_that_field_name_directly() {
    let pat = default_pattern_for_inductive(&well_known("Option"));
    assert_eq!(
        pat,
        RcasesPattern::Alts(vec![
            RcasesPattern::Clear,
            RcasesPattern::One("val".to_string()),
        ])
    );
}

#[test]
fn multi_field_constructor_gives_a_tuple() {
    let pat = default_pattern_for_inductive(&well_known("List"));
    match pat {
        RcasesPattern::Alts(alts) => {
            assert_eq!(alts.len(), 2);
            assert_eq!(alts[0], RcasesPattern::Clear);
            match &alts[1] {
                RcasesPattern::Tuple(fields) => assert_eq!(fields.len(), 2),
                other => panic!("expected a tuple for List.cons, got {other:?}"),
            }
        }
        other => panic!("expected alternatives for List, got {other:?}"),
    }
}
