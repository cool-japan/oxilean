//! `CertBFSExt::bfs_from` against a verbatim copy of the previous code, which
//! read each dequeued vertex's distance back with `expect`.

use super::CertBFSExt;

fn old_bfs_from(g: &mut CertBFSExt, source: usize) {
    g.distances = vec![None; g.n_vertices];
    g.predecessors = vec![None; g.n_vertices];
    g.distances[source] = Some(0);
    let mut queue = std::collections::VecDeque::new();
    queue.push_back(source);
    while let Some(u) = queue.pop_front() {
        let d = g.distances[u].expect("u was enqueued after its distance was set");
        for &v in &g.adjacency[u].clone() {
            if g.distances[v].is_none() {
                g.distances[v] = Some(d + 1);
                g.predecessors[v] = Some(u);
                queue.push_back(v);
            }
        }
    }
}

#[test]
fn distances_and_predecessors_are_unchanged() {
    let mut state = 0x1f83_d9ab_fb41_bd6bu64;
    let mut next = move || {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        state
    };
    for _ in 0..500 {
        let n = 1 + (next() % 9) as usize;
        let mut g = CertBFSExt::new(n);
        for _ in 0..(next() % 14) {
            g.add_edge((next() % n as u64) as usize, (next() % n as u64) as usize);
        }
        let mut old = g.clone();
        let source = (next() % n as u64) as usize;
        g.bfs_from(source);
        old_bfs_from(&mut old, source);
        assert_eq!(g.distances, old.distances);
        assert_eq!(g.predecessors, old.predecessors);
    }
}
