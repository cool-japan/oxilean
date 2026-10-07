//! `CausalBroadcast` delivery against a verbatim copy of the previous code,
//! and `LeaderElectionRing::elect_leader` on its edge cases.

use super::{CausalBroadcast, LeaderElectionRing};
use std::collections::VecDeque;

/// The previous `try_deliver`, which removed by index with `expect`.
fn old_try_deliver(cb: &mut CausalBroadcast) {
    let mut delivered_any = true;
    while delivered_any {
        delivered_any = false;
        let mut i = 0;
        while i < cb.pending.len() {
            let deliverable = {
                let (sender, msg_vc, _) = &cb.pending[i];
                let sender_ok = msg_vc[*sender] == cb.vc[*sender] + 1;
                let others_ok = (0..cb.num_processes)
                    .filter(|&j| j != *sender)
                    .all(|j| msg_vc[j] <= cb.vc[j]);
                sender_ok && others_ok
            };
            if deliverable {
                let (sender, _, payload) = cb.pending.remove(i).expect("i < len");
                cb.vc[sender] += 1;
                cb.delivered.push((sender, payload));
                delivered_any = true;
            } else {
                i += 1;
            }
        }
    }
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

type State = (
    Vec<u64>,
    VecDeque<(usize, Vec<u64>, Vec<u8>)>,
    Vec<(usize, Vec<u8>)>,
);
fn state(cb: &CausalBroadcast) -> State {
    (cb.vc.clone(), cb.pending.clone(), cb.delivered.clone())
}

#[test]
fn delivery_order_and_pending_queue_equal_the_previous_ones() {
    let mut rng = Rng(0xdead_beef_cafe_f00d);
    let mut delivered = 0usize;
    for _ in 0..1500 {
        let n = 1 + rng.below(4) as usize;
        let mut new = CausalBroadcast::new(0, n);
        let mut old = CausalBroadcast::new(0, n);
        for k in 0..rng.below(14) {
            let sender = rng.below(n as u64) as usize;
            let vc: Vec<u64> = (0..n).map(|_| rng.below(4)).collect();
            let payload = vec![k as u8, sender as u8];
            new.receive(sender, vc.clone(), payload.clone());
            old.pending.push_back((sender, vc, payload));
            old_try_deliver(&mut old);
            assert_eq!(state(&new), state(&old));
        }
        delivered += new.delivered.len();
    }
    assert!(delivered > 100);
}

#[test]
fn leader_is_the_largest_id_and_an_empty_ring_elects_zero() {
    let mut empty = LeaderElectionRing::new(vec![]);
    assert_eq!(empty.elect_leader(), 0);
    assert_eq!(empty.leader, None);
    let mut ring = LeaderElectionRing::new(vec![4, 9, 2, 9, 7]);
    assert_eq!(ring.elect_leader(), 9);
    // `max_by_key` keeps the last of equal maxima, as before.
    assert_eq!(ring.leader, Some(3));
}
