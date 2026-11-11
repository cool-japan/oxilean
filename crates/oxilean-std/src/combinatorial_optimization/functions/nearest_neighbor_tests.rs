//! `tsp_nearest_neighbor` against a verbatim copy of the previous code, which
//! read the tour's last vertex back with `expect`.

use super::tsp_nearest_neighbor;

fn old_tsp_nearest_neighbor(dist: &[Vec<f64>]) -> (Vec<usize>, f64) {
    let n = dist.len();
    if n == 0 {
        return (vec![], 0.0);
    }
    let mut visited = vec![false; n];
    let mut tour = vec![0usize];
    visited[0] = true;
    let mut cost = 0.0;
    for _ in 1..n {
        let last = *tour.last().expect("starts with 0");
        let next = (0..n).filter(|&j| !visited[j]).min_by(|&a, &b| {
            dist[last][a]
                .partial_cmp(&dist[last][b])
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        if let Some(next) = next {
            cost += dist[last][next];
            tour.push(next);
            visited[next] = true;
        }
    }
    cost += dist[*tour.last().expect("starts with 0")][tour[0]];
    (tour, cost)
}

#[test]
fn tours_are_unchanged() {
    let mut state = 0x5be0_cd19_137e_2179u64;
    let mut next = move || {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        state
    };
    for _ in 0..500 {
        let n = (next() % 8) as usize;
        let dist: Vec<Vec<f64>> = (0..n)
            .map(|_| (0..n).map(|_| (next() % 10) as f64 * 0.5).collect())
            .collect();
        let (tour, cost) = tsp_nearest_neighbor(&dist);
        let (old_tour, old_cost) = old_tsp_nearest_neighbor(&dist);
        assert_eq!(tour, old_tour);
        assert_eq!(cost.to_bits(), old_cost.to_bits());
    }
}
