//! `RollingHash::push` against a straight-line statement of the same update.

use super::*;
use std::collections::VecDeque;

const BASE: u64 = 257;
const MODULUS: u64 = 1_000_000_007;

/// The update `push` performs, written out over plain local state: multiply
/// by the base and add the byte; once the window holds more than
/// `window_size` bytes, drop the oldest one and subtract its `base_pow`
/// contribution.
struct Reference {
    window_size: usize,
    current: u64,
    base_pow: u64,
    window: VecDeque<u8>,
}

impl Reference {
    fn new(window_size: usize) -> Self {
        let mut base_pow = 1u64;
        for _ in 0..window_size.saturating_sub(1) {
            base_pow = base_pow.wrapping_mul(BASE) % MODULUS;
        }
        Self {
            window_size,
            current: 0,
            base_pow,
            window: VecDeque::new(),
        }
    }

    fn push(&mut self, byte: u8) -> u64 {
        self.current = (self.current.wrapping_mul(BASE) + byte as u64) % MODULUS;
        self.window.push_back(byte);
        if self.window.len() > self.window_size {
            let old = match self.window.pop_front() {
                Some(old) => old,
                None => panic!("the window holds more than window_size >= 0 bytes"),
            };
            let rem = self.base_pow.wrapping_mul(old as u64) % MODULUS;
            self.current = (self.current + MODULUS - rem) % MODULUS;
        }
        self.current
    }
}

#[test]
fn push_follows_the_reference_update_for_every_window_size() {
    let data: Vec<u8> = (0u32..300).map(|i| (i * 37 % 251) as u8).collect();
    for window_size in [0usize, 1, 2, 3, 7, 16, 64] {
        let mut rolling = RollingHash::new(window_size);
        let mut reference = Reference::new(window_size);
        for (i, &byte) in data.iter().enumerate() {
            assert_eq!(
                rolling.push(byte),
                reference.push(byte),
                "window {window_size}, byte {i}"
            );
            assert_eq!(rolling.window.len(), reference.window.len());
            assert_eq!(rolling.window_full(), i + 1 >= window_size);
        }
    }
}

#[test]
fn the_window_never_holds_more_than_window_size_bytes_after_a_push() {
    let mut rolling = RollingHash::new(4);
    for byte in 0u8..50 {
        rolling.push(byte);
        assert!(rolling.window.len() <= 4);
    }
    assert!(rolling.window_full());
}
