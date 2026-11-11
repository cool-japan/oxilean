//! `TarjanScc` over small call graphs.

use super::*;
use crate::lcnf::LcnfLit;

fn call(callee: &str, rest: LcnfExpr) -> LcnfExpr {
    LcnfExpr::Let {
        id: LcnfVarId(1),
        name: "r".to_string(),
        ty: LcnfType::Nat,
        value: LcnfLetValue::App(LcnfArg::Lit(LcnfLit::Str(callee.to_string())), vec![]),
        body: Box::new(rest),
    }
}

fn decl(name: &str, callees: &[&str]) -> LcnfFunDecl {
    let mut body = LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Nat(0)));
    for callee in callees.iter().rev() {
        body = call(callee, body);
    }
    LcnfFunDecl {
        name: name.to_string(),
        original_name: None,
        params: Vec::new(),
        ret_type: LcnfType::Nat,
        body,
        is_recursive: false,
        is_lifted: false,
        inline_cost: 1,
    }
}

fn components(graph: &[(&str, &[&str])]) -> (TarjanScc, Vec<Vec<String>>) {
    let decls: Vec<LcnfFunDecl> = graph.iter().map(|(n, cs)| decl(n, cs)).collect();
    let mut scc = TarjanScc::new(&decls);
    scc.compute();
    let mut sets: Vec<Vec<String>> = scc
        .sccs
        .iter()
        .map(|c| {
            let mut c = c.clone();
            c.sort();
            c
        })
        .collect();
    sets.sort();
    (scc, sets)
}

fn names(sets: &[&[&str]]) -> Vec<Vec<String>> {
    sets.iter()
        .map(|c| c.iter().map(|s| s.to_string()).collect())
        .collect()
}

#[test]
fn an_acyclic_chain_has_one_component_per_function() {
    let (scc, sets) = components(&[("a", &["b"]), ("b", &["c"]), ("c", &[])]);
    assert_eq!(sets, names(&[&["a"], &["b"], &["c"]]));
    assert_eq!(scc.num_sccs(), 3);
    assert!(!scc.is_recursive("a") && !scc.is_recursive("b") && !scc.is_recursive("c"));
}

#[test]
fn mutual_recursion_is_one_component() {
    let (scc, sets) = components(&[("a", &["b"]), ("b", &["a"]), ("c", &["a"])]);
    assert_eq!(sets, names(&[&["a", "b"], &["c"]]));
    assert!(scc.is_recursive("a") && scc.is_recursive("b"));
    assert!(!scc.is_recursive("c"));
}

#[test]
fn a_cycle_reached_through_a_tail_is_found() {
    let (scc, sets) = components(&[("a", &["b"]), ("b", &["c"]), ("c", &["b"])]);
    assert_eq!(sets, names(&[&["a"], &["b", "c"]]));
    assert!(!scc.is_recursive("a"));
    assert!(scc.is_recursive("b") && scc.is_recursive("c"));
    assert_eq!(scc.scc_of("c").map(|c| c.len()), Some(2));
}

#[test]
fn a_larger_cycle_is_one_component() {
    let (_, sets) = components(&[
        ("a", &["b"]),
        ("b", &["c"]),
        ("c", &["d"]),
        ("d", &["a", "e"]),
        ("e", &[]),
    ]);
    assert_eq!(sets, names(&[&["a", "b", "c", "d"], &["e"]]));
}

#[test]
fn an_edge_into_a_finished_component_does_not_merge_components() {
    // a calls b and c; c calls b. b is finished (off the stack) when c looks at it.
    let (scc, sets) = components(&[("a", &["b", "c"]), ("b", &[]), ("c", &["b"])]);
    assert_eq!(sets, names(&[&["a"], &["b"], &["c"]]));
    assert!(!scc.is_recursive("c"));
}

#[test]
fn a_self_call_is_a_singleton_component() {
    let (scc, sets) = components(&[("a", &["a"]), ("b", &["a"])]);
    assert_eq!(sets, names(&[&["a"], &["b"]]));
    assert!(!scc.is_recursive("a"));
}

#[test]
fn calls_to_unknown_functions_are_ignored() {
    let (_, sets) = components(&[("a", &["extern_fn", "b"]), ("b", &["a"])]);
    assert_eq!(sets, names(&[&["a", "b"]]));
}
