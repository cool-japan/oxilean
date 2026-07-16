/// Tests for `ConSource` and the updated `FarkasCert::sources` field.
use super::*;
use oxilean_kernel::{Expr, Level, Name};

#[test]
fn test_con_source_construction() {
    let source = ConSource {
        hyp_name: Some(Name::from_str("h")),
        hyp_type: Expr::Const(Name::from_str("Int.le"), vec![]),
        orient: ConOrient::LeZero,
    };
    assert_eq!(source.hyp_name, Some(Name::from_str("h")));
    assert!(matches!(source.orient, ConOrient::LeZero));
}

#[test]
fn test_con_source_negated_goal() {
    // hyp_name == None indicates the negated goal
    let source = ConSource {
        hyp_name: None,
        hyp_type: Expr::Sort(Level::Zero),
        orient: ConOrient::GeZero,
    };
    assert!(source.hyp_name.is_none());
    assert!(matches!(source.orient, ConOrient::GeZero));
}

#[test]
fn test_farkas_cert_sources_field_default_empty() {
    let cert = FarkasCert {
        entries: vec![],
        combined_rhs: Rat::new(-1, 1),
        sources: vec![],
    };
    assert_eq!(cert.sources.len(), 0);
}

#[test]
fn test_farkas_cert_new_sources_empty() {
    let cert = FarkasCert::new(vec![], Rat::new(-1, 1));
    assert_eq!(cert.sources.len(), 0);
}

#[test]
fn test_with_sources_builder() {
    let sources = vec![ConSource {
        hyp_name: Some(Name::from_str("h1")),
        hyp_type: Expr::Const(Name::from_str("LE.le"), vec![]),
        orient: ConOrient::LeZero,
    }];
    let cert = FarkasCert::new(vec![], Rat::new(-1, 1)).with_sources(sources);
    assert_eq!(cert.sources.len(), 1);
    assert_eq!(cert.sources[0].hyp_name, Some(Name::from_str("h1")));
}

#[test]
fn test_with_sources_empty_builder() {
    let cert = FarkasCert::new(vec![], Rat::new(-1, 1)).with_sources(vec![]);
    assert_eq!(cert.sources.len(), 0);
}

#[test]
fn test_farkas_cert_from_search_sources_empty() {
    let pairs = vec![(0usize, Rat::new(1, 1), ConOrient::GeZero)];
    let cert = FarkasCert::from_search(&pairs);
    assert_eq!(cert.sources.len(), 0);
}

#[test]
fn test_con_source_clone() {
    let source = ConSource {
        hyp_name: Some(Name::from_str("h")),
        hyp_type: Expr::Const(Name::from_str("Int.le"), vec![]),
        orient: ConOrient::GeZero,
    };
    let cloned = source.clone();
    assert_eq!(cloned.hyp_name, source.hyp_name);
}
