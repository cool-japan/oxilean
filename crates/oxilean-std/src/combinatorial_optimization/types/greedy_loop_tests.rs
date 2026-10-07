//! `GraphColoring::dsatur` and `BipartiteMatchingGraph::hopcroft_karp`
//! against verbatim copies of their previous loops (a counted `for` with
//! `expect`, and `is_none` followed by `expect`).

use super::{BipartiteMatchingGraph, GraphColoring};

fn old_dsatur(g: &GraphColoring) -> (usize, Vec<usize>) {
    let mut adj: Vec<Vec<usize>> = vec![vec![]; g.n];
    for &(u, v) in &g.edges {
        adj[u].push(v);
        adj[v].push(u);
    }
    let mut color = vec![usize::MAX; g.n];
    let mut saturation = vec![0usize; g.n];
    let mut colored = vec![false; g.n];
    let mut max_color = 0;
    for _ in 0..g.n {
        let u = (0..g.n)
            .filter(|&v| !colored[v])
            .max_by_key(|&v| (saturation[v], adj[v].len()))
            .expect("an uncoloured vertex is left");
        let used: std::collections::HashSet<usize> = adj[u]
            .iter()
            .filter_map(|&v| {
                if color[v] != usize::MAX {
                    Some(color[v])
                } else {
                    None
                }
            })
            .collect();
        let c = (0..).find(|c| !used.contains(c)).unwrap_or(0);
        color[u] = c;
        colored[u] = true;
        if c + 1 > max_color {
            max_color = c + 1;
        }
        for &v in &adj[u] {
            if !colored[v] {
                let neighbor_colors: std::collections::HashSet<usize> = adj[v]
                    .iter()
                    .filter_map(|&w| {
                        if color[w] != usize::MAX {
                            Some(color[w])
                        } else {
                            None
                        }
                    })
                    .collect();
                saturation[v] = neighbor_colors.len();
            }
        }
    }
    (max_color, color)
}

fn old_hopcroft_karp(
    g: &BipartiteMatchingGraph,
) -> (usize, Vec<Option<usize>>, Vec<Option<usize>>) {
    let mut match_l = vec![None; g.n_left];
    let mut match_r = vec![None; g.n_right];
    let mut size = 0;
    loop {
        let dist = g.bfs_phase(&match_l, &match_r);
        if dist.is_none() {
            break;
        }
        let mut dist = dist.expect("dist is Some");
        let mut augmented = false;
        for u in 0..g.n_left {
            if match_l[u].is_none() && g.dfs_phase(u, &mut dist, &mut match_l, &mut match_r) {
                size += 1;
                augmented = true;
            }
        }
        if !augmented {
            break;
        }
    }
    (size, match_l, match_r)
}

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }
    fn below(&mut self, n: u64) -> u64 {
        self.next() % n
    }
}

#[test]
fn dsatur_colours_as_before() {
    let mut rng = Rng(0x7a5c_3e1f_9b8d_6042);
    for _ in 0..800 {
        let n = rng.below(9) as usize;
        let edges: Vec<(usize, usize)> = if n == 0 {
            vec![]
        } else {
            (0..rng.below(16))
                .map(|_| (rng.below(n as u64) as usize, rng.below(n as u64) as usize))
                .filter(|(u, v)| u != v)
                .collect()
        };
        let g = GraphColoring::new(n, edges);
        assert_eq!(g.dsatur(), old_dsatur(&g));
    }
}

#[test]
fn hopcroft_karp_matches_as_before() {
    let mut rng = Rng(0x55aa_33cc_0ff0_1234);
    let mut matched = 0usize;
    for _ in 0..800 {
        let n_left = rng.below(6) as usize;
        let n_right = 1 + rng.below(6) as usize;
        let mut g = BipartiteMatchingGraph::new(n_left, n_right);
        if n_left > 0 {
            for _ in 0..rng.below(12) {
                g.add_edge(
                    rng.below(n_left as u64) as usize,
                    rng.below(n_right as u64) as usize,
                );
            }
        }
        let new = g.hopcroft_karp();
        matched += new.0;
        assert_eq!(new, old_hopcroft_karp(&g));
    }
    assert!(matched > 0);
}
