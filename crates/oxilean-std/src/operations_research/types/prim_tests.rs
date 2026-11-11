//! `PrimMst::run` against a verbatim copy of its previous loop, a counted
//! `for` that took the next vertex with `expect`.

use super::PrimMst;

fn old_run(p: &PrimMst) -> (i64, Vec<(usize, usize, i64)>) {
    let n = p.n;
    let mut in_mst = vec![false; n];
    let mut key = vec![i64::MAX; n];
    let mut parent = vec![usize::MAX; n];
    key[0] = 0;
    let mut mst_edges = Vec::new();
    let mut total = 0_i64;
    for _ in 0..n {
        let u = (0..n)
            .filter(|&v| !in_mst[v])
            .min_by_key(|&v| key[v])
            .expect("an unvisited vertex is left");
        in_mst[u] = true;
        if parent[u] != usize::MAX {
            let w = key[u];
            mst_edges.push((parent[u], u, w));
            total += w;
        }
        for v in 0..n {
            if !in_mst[v] && p.cost[u][v] < key[v] {
                key[v] = p.cost[u][v];
                parent[v] = u;
            }
        }
    }
    (total, mst_edges)
}

#[test]
fn prim_builds_the_same_tree_as_before() {
    let mut state = 0x6a09_e667_f3bc_c908u64;
    let mut next = move || {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        state
    };
    // `run` starts from vertex 0, so every graph has at least one vertex.
    for _ in 0..600 {
        let n = 1 + (next() % 8) as usize;
        let mut p = PrimMst::new(n);
        for _ in 0..(next() % 20) {
            let u = (next() % n as u64) as usize;
            let v = (next() % n as u64) as usize;
            p.add_edge(u, v, (next() % 9) as i64);
        }
        assert_eq!(p.run(), old_run(&p));
    }
}
