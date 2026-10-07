//! `PassManager::topological_order` against a verbatim copy of its earlier
//! body, on random dependency sets with repeated edges, self-loops, cycles
//! and names that are not registered passes.

use super::{PassDependency, PassManager};
use std::collections::HashMap;

/// The earlier body, over the same two fields.
fn old_topological_order(
    pass_names: &[String],
    dependencies: &[PassDependency],
) -> Option<Vec<String>> {
    let mut in_degree: HashMap<&str, usize> = HashMap::new();
    let mut adj: HashMap<&str, Vec<&str>> = HashMap::new();
    for name in pass_names {
        in_degree.insert(name.as_str(), 0);
        adj.entry(name.as_str()).or_default();
    }
    for dep in dependencies {
        if pass_names.contains(&dep.pass) && pass_names.contains(&dep.depends_on) {
            adj.entry(dep.depends_on.as_str())
                .or_default()
                .push(dep.pass.as_str());
            *in_degree.entry(dep.pass.as_str()).or_insert(0) += 1;
        }
    }
    let mut queue: Vec<&str> = in_degree
        .iter()
        .filter(|(_, &deg)| deg == 0)
        .map(|(&name, _)| name)
        .collect();
    queue.sort();
    let mut result = Vec::new();
    while let Some(node) = queue.pop() {
        result.push(node.to_string());
        if let Some(neighbors) = adj.get(node) {
            for &neighbor in neighbors {
                let deg = in_degree.get_mut(neighbor).expect(
                    "neighbor must be in in_degree; all passes were inserted during initialization",
                );
                *deg -= 1;
                if *deg == 0 {
                    queue.push(neighbor);
                    queue.sort();
                }
            }
        }
    }
    if result.len() == pass_names.len() {
        Some(result)
    } else {
        None
    }
}

struct Lcg(u64);
impl Lcg {
    fn next(&mut self) -> u64 {
        self.0 = self
            .0
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        self.0 >> 33
    }
}

#[test]
fn order_equals_the_earlier_one_on_random_dependency_sets() {
    let mut rng = Lcg(0x70_9e);
    let mut cycles = 0;
    let mut orders = 0;
    for round in 0..3000u64 {
        let mut pm = PassManager::new();
        let passes = (round % 7) as usize;
        for i in 0..passes {
            pm.add_pass(format!("p{i}"));
        }
        let edges = rng.next() % 12;
        for _ in 0..edges {
            // Names up to `p8` include unregistered ones.
            let a = format!("p{}", rng.next() % 9);
            let b = format!("p{}", rng.next() % 9);
            let dep = PassDependency::new(a, b);
            // Mostly acyclic (from a higher to a lower index), sometimes not;
            // pushed directly so that a repeated edge stays repeated.
            if rng.next() % 4 == 0 || dep.pass > dep.depends_on {
                pm.dependencies.push(dep);
            }
        }
        let new = pm.topological_order();
        let old = old_topological_order(&pm.pass_names, &pm.dependencies);
        assert_eq!(new, old, "{:?} {:?}", pm.pass_names, pm.dependencies);
        assert_eq!(pm.has_cycle(), old.is_none());
        match old {
            Some(_) => orders += 1,
            None => cycles += 1,
        }
    }
    assert!(
        orders > 1000 && cycles > 100,
        "{orders} orders, {cycles} cycles"
    );
}
