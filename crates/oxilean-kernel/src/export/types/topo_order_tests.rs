//! `ModuleDependencyGraph::topological_order` on acyclic and cyclic graphs.

use super::ModuleDependencyGraph;

fn graph(modules: &[&str], deps: &[(&str, &str)]) -> ModuleDependencyGraph {
    let mut g = ModuleDependencyGraph::new();
    for m in modules {
        g.add_module(m.to_string());
    }
    for (from, to) in deps {
        g.add_dep(from.to_string(), to.to_string());
    }
    g
}

#[test]
fn every_module_is_ordered_or_the_cycle_is_reported() {
    let g = graph(&["a", "b", "c", "d"], &[("b", "a"), ("c", "b"), ("d", "a")]);
    let order = g.topological_order();
    assert!(order.is_ok(), "{order:?}");
    let order = order.unwrap_or_default();
    assert_eq!(order.len(), 4);
    let position = |m: &str| order.iter().position(|x| x == m);
    // A module comes after every module it depends on.
    assert!(position("a") < position("b"));
    assert!(position("b") < position("c"));
    assert!(position("a") < position("d"));

    let cyclic = graph(&["a", "b"], &[("a", "b"), ("b", "a")]);
    assert_eq!(
        cyclic.topological_order(),
        Err("Cycle detected in module dependency graph".to_string())
    );
    assert_eq!(ModuleDependencyGraph::new().topological_order(), Ok(vec![]));
}
