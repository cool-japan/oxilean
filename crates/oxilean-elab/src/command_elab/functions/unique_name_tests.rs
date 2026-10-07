//! Tests for `resolve_unique_name` with one and with two candidates.

use super::super::types::{CommandState, DeclInfo};
use super::resolve_unique_name;
use oxilean_kernel::{Expr, Name};

fn decl() -> DeclInfo {
    DeclInfo {
        ty: Expr::Const(Name::str("Nat"), vec![]),
        val: None,
        namespace: vec![],
        is_theorem: false,
        univ_params: vec![],
    }
}

#[test]
fn a_name_reached_two_ways_is_ambiguous() {
    let mut state = CommandState::new();
    state.register_decl(Name::str("foo"), decl());
    state.register_decl(Name::str("A.foo"), decl());
    assert_eq!(
        resolve_unique_name("foo", &state).ok(),
        Some(Name::str("foo"))
    );
    state.open_namespaces.push(vec!["A".to_string()]);
    let err = resolve_unique_name("foo", &state).expect_err("two candidates");
    assert!(format!("{:?}", err).contains("ambiguous name 'foo'"));
    assert_eq!(
        resolve_unique_name("A.foo", &state).ok(),
        Some(Name::str("A.foo"))
    );
}
