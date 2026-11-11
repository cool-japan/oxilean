//! `christofides_serdyukov` against a verbatim copy of the previous code,
//! whose circuit walk popped the stack with `expect`.

use super::{christofides_serdyukov, kruskal_mst};

fn old_christofides_serdyukov(dist: &[Vec<i64>]) -> (i64, Vec<usize>) {
    let n = dist.len();
    if n == 0 {
        return (0, vec![]);
    }
    if n == 1 {
        return (0, vec![0]);
    }
    if n == 2 {
        return (dist[0][1] + dist[1][0], vec![0, 1]);
    }
    let mut edges = Vec::new();
    for i in 0..n {
        for j in (i + 1)..n {
            edges.push((i, j, dist[i][j]));
        }
    }
    let (_, mst_edges) = kruskal_mst(n, &edges);
    let mut mst_adj = vec![vec![]; n];
    let mut degree = vec![0usize; n];
    for (u, v) in &mst_edges {
        mst_adj[*u].push(*v);
        mst_adj[*v].push(*u);
        degree[*u] += 1;
        degree[*v] += 1;
    }
    let odd_verts: Vec<usize> = (0..n).filter(|&v| degree[v] % 2 == 1).collect();
    let mut matched = vec![false; odd_verts.len()];
    let mut matching = Vec::new();
    for i in 0..odd_verts.len() {
        if matched[i] {
            continue;
        }
        let best = (0..odd_verts.len())
            .filter(|&j| j != i && !matched[j])
            .min_by_key(|&j| dist[odd_verts[i]][odd_verts[j]]);
        if let Some(j) = best {
            matching.push((odd_verts[i], odd_verts[j]));
            matched[i] = true;
            matched[j] = true;
        }
    }
    let mut multi_adj = mst_adj.clone();
    for (u, v) in &matching {
        multi_adj[*u].push(*v);
        multi_adj[*v].push(*u);
    }
    let mut adj_idx = vec![0usize; n];
    let mut circuit = Vec::new();
    let mut stack = vec![0usize];
    while let Some(&cur) = stack.last() {
        if adj_idx[cur] < multi_adj[cur].len() {
            let next = multi_adj[cur][adj_idx[cur]];
            adj_idx[cur] += 1;
            stack.push(next);
        } else {
            circuit.push(stack.pop().expect("the loop saw a top element"));
        }
    }
    circuit.reverse();
    let mut visited = vec![false; n];
    let mut tour: Vec<usize> = circuit
        .into_iter()
        .filter(|&v| {
            if !visited[v] {
                visited[v] = true;
                true
            } else {
                false
            }
        })
        .collect();
    for v in 0..n {
        if !visited[v] {
            tour.push(v);
        }
    }
    let cost: i64 = (0..tour.len())
        .map(|i| dist[tour[i]][tour[(i + 1) % tour.len()]])
        .sum();
    (cost, tour)
}

#[test]
fn tours_and_costs_are_unchanged() {
    let mut state = 0xa54f_f53a_5f1d_36f1u64;
    let mut next = move || {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        state
    };
    for _ in 0..500 {
        let n = (next() % 9) as usize;
        let mut dist = vec![vec![0i64; n]; n];
        for i in 0..n {
            for j in (i + 1)..n {
                let d = 1 + (next() % 20) as i64;
                dist[i][j] = d;
                dist[j][i] = d;
            }
        }
        assert_eq!(
            christofides_serdyukov(&dist),
            old_christofides_serdyukov(&dist)
        );
    }
}
