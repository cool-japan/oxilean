//! Compares `MetaEqClass` with its previous form on deterministic sequences
//! of `union` and `find` operations, including the rank-raising case.

use super::MetaEqClass;
use std::collections::HashMap;

/// The previous form of the structure, kept as the reference.
#[derive(Default)]
struct PreviousEqClass {
    parent: HashMap<u64, u64>,
    rank: HashMap<u64, u32>,
}

impl PreviousEqClass {
    fn add(&mut self, id: u64) {
        self.parent.entry(id).or_insert(id);
        self.rank.entry(id).or_insert(0);
    }
    fn find(&mut self, id: u64) -> u64 {
        self.add(id);
        let parent = *self
            .parent
            .get(&id)
            .expect("id was just added via self.add");
        if parent == id {
            id
        } else {
            let root = self.find(parent);
            *self.parent.get_mut(&id).expect("id was added via self.add") = root;
            root
        }
    }
    fn union(&mut self, a: u64, b: u64) {
        let ra = self.find(a);
        let rb = self.find(b);
        if ra == rb {
            return;
        }
        let rank_a = *self.rank.get(&ra).unwrap_or(&0);
        let rank_b = *self.rank.get(&rb).unwrap_or(&0);
        if rank_a < rank_b {
            *self.parent.get_mut(&ra).expect("ra was added via find") = rb;
        } else if rank_a > rank_b {
            *self.parent.get_mut(&rb).expect("rb was added via find") = ra;
        } else {
            *self.parent.get_mut(&rb).expect("rb was added via find") = ra;
            *self.rank.get_mut(&ra).expect("ra was added via find") += 1;
        }
    }
}

#[test]
fn matches_the_previous_form_on_operation_sequences() {
    for seed in 1u64..=200 {
        let mut state = seed;
        let mut next = move |bound: u64| {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            state % bound
        };
        let mut current = MetaEqClass::new();
        let mut previous = PreviousEqClass::default();
        for _ in 0..60 {
            let a = next(24);
            let b = next(24);
            if next(3) == 0 {
                assert_eq!(current.find(a), previous.find(a), "seed {seed}");
            } else {
                current.union(a, b);
                previous.union(a, b);
            }
        }
        for id in 0..24 {
            assert_eq!(current.find(id), previous.find(id), "seed {seed} id {id}");
        }
        assert_eq!(current.parent, previous.parent, "seed {seed}");
        assert_eq!(current.rank, previous.rank, "seed {seed}");
    }
}

#[test]
fn union_by_rank_keeps_the_higher_ranked_root() {
    let mut classes = MetaEqClass::new();
    classes.union(1, 2);
    classes.union(3, 4);
    classes.union(1, 3);
    let big_root = classes.find(4);
    classes.union(9, 1);
    assert_eq!(classes.find(9), big_root);
    assert!(classes.same_class(2, 9));
    assert!(!classes.same_class(2, 7));
}
