//! `Incompatibility::almost_satisfied`: the one undecided package, if exactly
//! one term is still open and every decided term is satisfied.

use super::*;

fn incompatibility(packages: &[&str]) -> Incompatibility {
    let terms: BTreeMap<String, VersionSet> = packages
        .iter()
        .map(|name| (name.to_string(), VersionSet::universal()))
        .collect();
    Incompatibility::new(terms, IncompatibilityCause::Root)
}

fn decided(entries: &[(&str, Version)]) -> HashMap<String, Version> {
    entries
        .iter()
        .map(|(name, version)| (name.to_string(), version.clone()))
        .collect()
}

#[test]
fn no_terms_is_not_almost_satisfied() {
    let inc = incompatibility(&[]);
    assert_eq!(inc.almost_satisfied(&decided(&[])), None);
}

#[test]
fn all_terms_decided_and_satisfied_is_not_almost_satisfied() {
    let inc = incompatibility(&["a", "b"]);
    let map = decided(&[("a", Version::new(1, 0, 0)), ("b", Version::new(2, 0, 0))]);
    assert_eq!(inc.almost_satisfied(&map), None);
}

#[test]
fn exactly_one_open_term_is_returned() {
    let inc = incompatibility(&["a", "b", "c"]);
    let map = decided(&[("a", Version::new(1, 0, 0)), ("c", Version::new(3, 0, 0))]);
    assert_eq!(inc.almost_satisfied(&map), Some("b".to_string()));
}

#[test]
fn a_single_term_with_nothing_decided_is_returned() {
    let inc = incompatibility(&["only"]);
    assert_eq!(
        inc.almost_satisfied(&decided(&[])),
        Some("only".to_string())
    );
}

#[test]
fn two_open_terms_are_not_almost_satisfied() {
    let inc = incompatibility(&["a", "b", "c"]);
    let map = decided(&[("a", Version::new(1, 0, 0))]);
    assert_eq!(inc.almost_satisfied(&map), None);
}

#[test]
fn a_decided_term_outside_its_set_rules_the_incompatibility_out() {
    let mut terms: BTreeMap<String, VersionSet> = BTreeMap::new();
    terms.insert("a".to_string(), VersionSet::empty());
    terms.insert("b".to_string(), VersionSet::universal());
    let inc = Incompatibility::new(terms, IncompatibilityCause::Root);
    let map = decided(&[("a", Version::new(1, 0, 0))]);
    assert_eq!(inc.almost_satisfied(&map), None);
}
