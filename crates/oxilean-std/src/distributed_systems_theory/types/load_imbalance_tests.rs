//! `ConsistentHashRing::load_imbalance` against a verbatim copy of the
//! previous code, which took the maximum and minimum load with `expect`.

use super::ConsistentHashRing;

fn old_load_imbalance(ring: &ConsistentHashRing, num_keys: u64) -> f64 {
    let dist = ring.key_distribution(num_keys);
    if dist.is_empty() || num_keys == 0 {
        return 0.0;
    }
    let max_load = *dist.iter().max().expect("dist is non-empty") as f64;
    let min_load = *dist.iter().min().expect("dist is non-empty") as f64;
    let avg_load = num_keys as f64 / ring.num_nodes as f64;
    if avg_load == 0.0 {
        0.0
    } else {
        (max_load - min_load) / avg_load
    }
}

#[test]
fn load_imbalance_is_unchanged() {
    for num_nodes in 1..6 {
        for virtual_nodes in 1..4 {
            let ring = ConsistentHashRing::new(num_nodes, virtual_nodes);
            for num_keys in 0..60 {
                let (new, old) = (
                    ring.load_imbalance(num_keys),
                    old_load_imbalance(&ring, num_keys),
                );
                assert_eq!(
                    new.to_bits(),
                    old.to_bits(),
                    "{num_nodes} {virtual_nodes} {num_keys}"
                );
            }
        }
    }
    // No physical node: no keys, no imbalance.
    assert_eq!(ConsistentHashRing::new(0, 3).load_imbalance(0), 0.0);
}
