//! `TreewidthHeuristic::run` against a verbatim copy of its previous loop,
//! a counted `for` that took the next vertex with `expect`.

use super::{TreewidthHeuristic, UndirectedGraph};

fn old_run(h: &mut TreewidthHeuristic) -> (Vec<usize>, usize) {
    let mut eliminated = vec![false; h.n];
    let mut order = Vec::with_capacity(h.n);
    let mut tw_bound = 0usize;
    for _ in 0..h.n {
        let v = (0..h.n)
            .filter(|&u| !eliminated[u])
            .min_by_key(|&u| h.fill_count(u))
            .expect("a vertex is left");
        let deg = h.adj[v].len();
        tw_bound = tw_bound.max(deg);
        let neighbors: Vec<usize> = h.adj[v].iter().copied().collect();
        for i in 0..neighbors.len() {
            for j in (i + 1)..neighbors.len() {
                let a = neighbors[i];
                let b = neighbors[j];
                if a != b {
                    h.adj[a].insert(b);
                    h.adj[b].insert(a);
                }
            }
        }
        for &u in &neighbors {
            h.adj[u].remove(&v);
        }
        h.adj[v].clear();
        eliminated[v] = true;
        order.push(v);
    }
    (order, tw_bound)
}

#[test]
fn min_fill_elimination_is_unchanged() {
    let mut state = 0x3c6e_f372_fe94_f82bu64;
    let mut next = move || {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        state
    };
    for _ in 0..600 {
        let n = (next() % 9) as usize;
        let mut g = UndirectedGraph::new(n);
        if n > 0 {
            for _ in 0..(next() % 20) {
                g.add_edge((next() % n as u64) as usize, (next() % n as u64) as usize);
            }
        }
        let mut new = TreewidthHeuristic::new(&g);
        let mut old = TreewidthHeuristic::new(&g);
        assert_eq!(new.run(), old_run(&mut old));
        assert_eq!(new.adj, old.adj);
    }
}
