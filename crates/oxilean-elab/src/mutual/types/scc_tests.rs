//! Compares the Tarjan helper of `CallGraph` with the previous form of the
//! same algorithm on every directed graph with up to four nodes.

use super::CallGraph;
use oxilean_kernel::Name;

/// The previous form of the helper, its arguments gathered in one value.
struct Previous<'a> {
    adj: &'a [Vec<usize>],
    names: &'a [Name],
    index_counter: usize,
    stack: Vec<usize>,
    on_stack: Vec<bool>,
    indices: Vec<Option<usize>>,
    lowlinks: Vec<usize>,
    result: Vec<Vec<Name>>,
}

impl Previous<'_> {
    fn tarjan_visit(&mut self, v: usize) {
        self.indices[v] = Some(self.index_counter);
        self.lowlinks[v] = self.index_counter;
        self.index_counter += 1;
        self.stack.push(v);
        self.on_stack[v] = true;
        let adj = self.adj;
        for &w in &adj[v] {
            if self.indices[w].is_none() {
                self.tarjan_visit(w);
                self.lowlinks[v] = self.lowlinks[v].min(self.lowlinks[w]);
            } else if self.on_stack[w] {
                self.lowlinks[v] = self.lowlinks[v]
                    .min(self.indices[w].expect("w is on stack so indices[w] is set"));
            }
        }
        if self.lowlinks[v] == self.indices[v].expect("v was just assigned an index above") {
            let mut component = Vec::new();
            loop {
                let w = self.stack.pop().expect("v is on the stack");
                self.on_stack[w] = false;
                component.push(self.names[w].clone());
                if w == v {
                    break;
                }
            }
            self.result.push(component);
        }
    }
}

fn previous_sccs(adj: &[Vec<usize>], names: &[Name]) -> Vec<Vec<Name>> {
    let n = adj.len();
    let mut state = Previous {
        adj,
        names,
        index_counter: 0,
        stack: Vec::new(),
        on_stack: vec![false; n],
        indices: vec![None; n],
        lowlinks: vec![0; n],
        result: Vec::new(),
    };
    for v in 0..n {
        if state.indices[v].is_none() {
            state.tarjan_visit(v);
        }
    }
    state.result
}

fn current_sccs(adj: &[Vec<usize>], names: &[Name]) -> Vec<Vec<Name>> {
    let n = adj.len();
    let mut index_counter = 0;
    let mut stack = Vec::new();
    let mut on_stack = vec![false; n];
    let mut indices: Vec<Option<usize>> = vec![None; n];
    let mut lowlinks = vec![0usize; n];
    let mut result = Vec::new();
    for v in 0..n {
        if indices[v].is_none() {
            CallGraph::tarjan_visit(
                v,
                adj,
                &mut index_counter,
                &mut stack,
                &mut on_stack,
                &mut indices,
                &mut lowlinks,
                &mut result,
                names,
            );
        }
    }
    result
}

#[test]
fn matches_the_previous_form_on_every_small_graph() {
    let names: Vec<Name> = ["f", "g", "h", "k"].iter().map(|s| Name::str(*s)).collect();
    for n in 1..=4usize {
        for mask in 0u32..(1u32 << (n * n)) {
            let adj: Vec<Vec<usize>> = (0..n)
                .map(|from| {
                    (0..n)
                        .filter(|to| mask & (1 << (from * n + to)) != 0)
                        .collect()
                })
                .collect();
            assert_eq!(
                current_sccs(&adj, &names[..n]),
                previous_sccs(&adj, &names[..n]),
                "graph {:?}",
                adj
            );
        }
    }
}
