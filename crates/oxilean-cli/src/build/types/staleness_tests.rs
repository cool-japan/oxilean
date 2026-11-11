//! Tests for how `BuildGraph::compute_staleness` propagates staleness from a
//! changed module to everything that depends on it, directly or not.

use super::{BuildGraph, BuildNode};
use std::collections::HashMap;
use std::path::PathBuf;

fn node(name: &str, hash: u64, deps: &[&str]) -> BuildNode {
    let mut n = BuildNode::new(
        name.to_string(),
        PathBuf::from(format!("{}.lean", name)),
        PathBuf::from(format!("{}.olean", name)),
    );
    n.hash = hash;
    for d in deps {
        n.add_dependency((*d).to_string());
    }
    n
}

fn stale_names(graph: &BuildGraph) -> Vec<String> {
    let mut names: Vec<String> = graph
        .module_names()
        .into_iter()
        .filter(|n| graph.get_node(n).is_some_and(|node| node.is_stale))
        .collect();
    names.sort();
    names
}

#[test]
fn a_changed_module_makes_its_transitive_dependents_stale() {
    let mut graph = BuildGraph::new();
    graph.add_node(node("A", 1, &[]));
    graph.add_node(node("B", 2, &["A"]));
    graph.add_node(node("C", 3, &["B"]));
    graph.add_node(node("D", 4, &["C", "E"]));
    graph.add_node(node("E", 5, &[]));
    graph.add_node(node("F", 6, &["E"]));
    let mut cache: HashMap<String, u64> =
        [("A", 1), ("B", 2), ("C", 3), ("D", 4), ("E", 5), ("F", 6)]
            .iter()
            .map(|(k, v)| (k.to_string(), *v))
            .collect();
    graph.compute_staleness(&cache);
    assert!(stale_names(&graph).is_empty());
    cache.insert("A".to_string(), 100);
    graph.compute_staleness(&cache);
    assert_eq!(stale_names(&graph), vec!["A", "B", "C", "D"]);
    cache.insert("A".to_string(), 1);
    cache.remove("E");
    graph.compute_staleness(&cache);
    assert_eq!(stale_names(&graph), vec!["D", "E", "F"]);
}

#[test]
fn dependencies_outside_the_graph_do_not_make_a_module_stale() {
    let mut graph = BuildGraph::new();
    graph.add_node(node("A", 1, &["Missing"]));
    let cache: HashMap<String, u64> = [("A".to_string(), 1)].into_iter().collect();
    graph.compute_staleness(&cache);
    assert!(stale_names(&graph).is_empty());
}
