//! `MetricSpace::mst_length` against a verbatim copy of its previous loop, a
//! counted `for` that took the next vertex with `expect`.

use super::MetricSpace;

fn old_mst_length(m: &MetricSpace) -> f64 {
    if m.n <= 1 {
        return 0.0;
    }
    let mut in_tree = vec![false; m.n];
    let mut min_edge = vec![f64::INFINITY; m.n];
    min_edge[0] = 0.0;
    let mut total = 0.0;
    for _ in 0..m.n {
        let u = (0..m.n)
            .filter(|&v| !in_tree[v])
            .min_by(|&a, &b| {
                min_edge[a]
                    .partial_cmp(&min_edge[b])
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .expect("a vertex is outside the tree");
        in_tree[u] = true;
        total += min_edge[u];
        for v in 0..m.n {
            if !in_tree[v] && m.dist[u][v] < min_edge[v] {
                min_edge[v] = m.dist[u][v];
            }
        }
    }
    total
}

#[test]
fn spanning_tree_length_is_unchanged() {
    let mut state = 0x3c6e_f372_a54f_f53au64;
    let mut next = move || {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        state
    };
    for _ in 0..600 {
        let n = (next() % 8) as usize;
        let mut m = MetricSpace::new(n);
        for i in 0..n {
            for j in (i + 1)..n {
                // Some distances infinite or NaN, to exercise the comparisons.
                let d = match next() % 10 {
                    0 => f64::INFINITY,
                    1 => f64::NAN,
                    k => k as f64 * 0.5,
                };
                m.set_dist(i, j, d);
            }
        }
        let (new, old) = (m.mst_length(), old_mst_length(&m));
        assert!(
            new == old || (new.is_nan() && old.is_nan()),
            "{new} vs {old}"
        );
    }
}
