//! Compares the Tarjan helper of `MutualRecGroup` with the previous form of
//! the same algorithm on every directed graph with up to four nodes.

use super::MutualRecGroup;

/// The previous form of the helper, its arguments gathered in one value.
struct Previous<'a> {
    adj: &'a [Vec<usize>],
    index_counter: usize,
    stack: Vec<usize>,
    on_stack: Vec<bool>,
    indices: Vec<Option<usize>>,
    lowlinks: Vec<usize>,
    result: Vec<Vec<usize>>,
}

impl Previous<'_> {
    fn tarjan_dfs(&mut self, v: usize) {
        self.indices[v] = Some(self.index_counter);
        self.lowlinks[v] = self.index_counter;
        self.index_counter += 1;
        self.stack.push(v);
        self.on_stack[v] = true;
        let adj = self.adj;
        for &w in &adj[v] {
            if self.indices[w].is_none() {
                self.tarjan_dfs(w);
                self.lowlinks[v] = self.lowlinks[v].min(self.lowlinks[w]);
            } else if self.on_stack[w] {
                self.lowlinks[v] = self.lowlinks[v]
                    .min(self.indices[w].expect("on-stack node must have an index"));
            }
        }
        if self.lowlinks[v] == self.indices[v].expect("current node must have an index") {
            let mut component = Vec::new();
            loop {
                let w = self.stack.pop().expect("stack must contain current node v");
                self.on_stack[w] = false;
                component.push(w);
                if w == v {
                    break;
                }
            }
            self.result.push(component);
        }
    }
}

fn sccs(adj: &[Vec<usize>], previous: bool) -> Vec<Vec<usize>> {
    let n = adj.len();
    if previous {
        let mut state = Previous {
            adj,
            index_counter: 0,
            stack: Vec::new(),
            on_stack: vec![false; n],
            indices: vec![None; n],
            lowlinks: vec![0; n],
            result: Vec::new(),
        };
        for v in 0..n {
            if state.indices[v].is_none() {
                state.tarjan_dfs(v);
            }
        }
        return state.result;
    }
    let mut index_counter = 0;
    let mut stack = Vec::new();
    let mut on_stack = vec![false; n];
    let mut indices: Vec<Option<usize>> = vec![None; n];
    let mut lowlinks = vec![0usize; n];
    let mut result = Vec::new();
    for v in 0..n {
        if indices[v].is_none() {
            MutualRecGroup::tarjan_dfs(
                v,
                adj,
                &mut index_counter,
                &mut stack,
                &mut on_stack,
                &mut indices,
                &mut lowlinks,
                &mut result,
            );
        }
    }
    result
}

#[test]
fn matches_the_previous_form_on_every_small_graph() {
    for n in 1..=4usize {
        for mask in 0u32..(1u32 << (n * n)) {
            let adj: Vec<Vec<usize>> = (0..n)
                .map(|from| {
                    (0..n)
                        .filter(|to| mask & (1 << (from * n + to)) != 0)
                        .collect()
                })
                .collect();
            assert_eq!(sccs(&adj, false), sccs(&adj, true), "graph {:?}", adj);
        }
    }
}

#[test]
fn finds_cycles_and_singletons() {
    let adj = vec![vec![1], vec![2], vec![0, 3], vec![]];
    assert_eq!(sccs(&adj, false), vec![vec![3], vec![2, 1, 0]]);
}
