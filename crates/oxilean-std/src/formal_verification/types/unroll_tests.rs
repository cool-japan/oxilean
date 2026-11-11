//! `BoundedModelChecker::unroll` against a verbatim copy of the previous
//! code, which read each path's last state back with `expect`.

use super::{BoundedModelChecker, KripkeStructure};

fn old_unroll(bmc: &BoundedModelChecker, ks: &KripkeStructure, start: usize) -> Vec<Vec<usize>> {
    let mut paths: Vec<Vec<usize>> = vec![vec![start]];
    for _ in 0..bmc.bound {
        let mut new_paths = Vec::new();
        for path in &paths {
            let last = *path.last().expect("a path starts with `start`");
            let succs: Vec<usize> = ks
                .transitions
                .iter()
                .filter(|(s, _)| *s == last)
                .map(|(_, d)| *d)
                .collect();
            if succs.is_empty() {
                new_paths.push(path.clone());
            } else {
                for s in succs {
                    let mut p = path.clone();
                    p.push(s);
                    new_paths.push(p);
                }
            }
        }
        paths = new_paths;
    }
    paths
}

#[test]
fn unrolled_paths_are_unchanged() {
    let mut state = 0x9b05_688c_2b3e_6c1fu64;
    let mut next = move || {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        state
    };
    for _ in 0..300 {
        let n = 1 + (next() % 4) as usize;
        let transitions: Vec<(usize, usize)> = (0..(next() % 7))
            .map(|_| ((next() % n as u64) as usize, (next() % n as u64) as usize))
            .collect();
        let ks = KripkeStructure::new(
            (0..n).map(|i| format!("s{i}")).collect(),
            transitions,
            vec![Vec::new(); n],
        );
        let bmc = BoundedModelChecker::new((next() % 4) as usize);
        let start = (next() % n as u64) as usize;
        assert_eq!(bmc.unroll(&ks, start), old_unroll(&bmc, &ks, start));
    }
}
