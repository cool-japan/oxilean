//! `treewidth_upper_bound` against a verbatim copy of its previous loop, a
//! counted `for` that took the next vertex with `expect`.

use super::treewidth_upper_bound;

fn old_treewidth_upper_bound(adj: &[Vec<usize>]) -> usize {
    let n = adj.len();
    if n == 0 {
        return 0;
    }
    let mut remaining: Vec<bool> = vec![true; n];
    let mut adj_copy: Vec<std::collections::HashSet<usize>> = adj
        .iter()
        .map(|nbrs| nbrs.iter().cloned().collect())
        .collect();
    let mut max_clique = 0usize;
    for _ in 0..n {
        let v = (0..n)
            .filter(|&u| remaining[u])
            .min_by_key(|&u| adj_copy[u].len())
            .expect("a vertex remains");
        let deg = adj_copy[v].len();
        max_clique = max_clique.max(deg);
        let nbrs: Vec<usize> = adj_copy[v].iter().cloned().collect();
        for i in 0..nbrs.len() {
            for j in (i + 1)..nbrs.len() {
                adj_copy[nbrs[i]].insert(nbrs[j]);
                adj_copy[nbrs[j]].insert(nbrs[i]);
            }
        }
        remaining[v] = false;
        for &u in &nbrs {
            adj_copy[u].remove(&v);
        }
        adj_copy[v].clear();
    }
    max_clique
}

#[test]
fn treewidth_bound_is_unchanged() {
    let mut state = 0xbb67_ae85_84ca_a73bu64;
    let mut next = move || {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        state
    };
    for _ in 0..600 {
        let n = (next() % 9) as usize;
        let mut adj = vec![Vec::new(); n];
        if n > 0 {
            for _ in 0..(next() % 20) {
                let u = (next() % n as u64) as usize;
                let v = (next() % n as u64) as usize;
                if u != v {
                    adj[u].push(v);
                    adj[v].push(u);
                }
            }
        }
        assert_eq!(treewidth_upper_bound(&adj), old_treewidth_upper_bound(&adj));
    }
}
