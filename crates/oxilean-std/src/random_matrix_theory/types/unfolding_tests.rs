//! `LevelSpacingStats::number_variance` against a verbatim copy of the
//! previous unfolding, which read the last level back with `expect`.

use super::{number_variance, LevelSpacingStats};

fn old_number_variance(stats: &LevelSpacingStats, l: f64) -> f64 {
    let mut unfolded = vec![0.0f64];
    for &s in &stats.spacings {
        unfolded.push(unfolded.last().expect("starts with 0.0") + s);
    }
    number_variance(&unfolded, l)
}

#[test]
fn number_variance_is_bit_identical() {
    for len in 0..12usize {
        let stats = LevelSpacingStats {
            spacings: (0..len)
                .map(|i| 0.3 + (i as f64 * 1.37).sin().abs())
                .collect(),
        };
        for l in [0.5, 1.0, 2.5] {
            let (new, old) = (stats.number_variance(l), old_number_variance(&stats, l));
            assert_eq!(new.to_bits(), old.to_bits(), "len {len}, l {l}");
        }
    }
}
