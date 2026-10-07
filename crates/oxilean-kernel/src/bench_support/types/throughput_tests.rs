//! `ThroughputTracker::items_per_sec` against a verbatim copy of its earlier
//! body, on windows built directly from chosen instants.

use super::ThroughputTracker;
use crate::wall_clock::Instant;
use std::time::Duration;

/// The rate as the earlier body computed it.
fn earlier_items_per_sec(tracker: &ThroughputTracker) -> f64 {
    if tracker.events.len() < 2 {
        return 0.0;
    }
    let total_items: u64 = tracker.events.iter().map(|(_, c)| c).sum();
    let duration = tracker
        .events
        .back()
        .expect("events non-empty: checked len >= 2 above")
        .0
        .duration_since(
            tracker
                .events
                .front()
                .expect("events non-empty: checked len >= 2 above")
                .0,
        )
        .as_secs_f64();
    if duration < f64::EPSILON {
        return 0.0;
    }
    total_items as f64 / duration
}

#[test]
fn agrees_with_the_earlier_body_for_every_window_size() {
    let start = Instant::now();
    for count in 0..7u64 {
        for spacing_ms in [0u64, 1, 7, 250] {
            let events = (0..count)
                .map(|i| (start + Duration::from_millis(i * spacing_ms), 3 * i + 1))
                .collect();
            let tracker = ThroughputTracker {
                window_ms: 1000.0,
                events,
            };
            let rate = tracker.items_per_sec();
            assert_eq!(rate.to_bits(), earlier_items_per_sec(&tracker).to_bits());
            if count < 2 || spacing_ms == 0 {
                assert_eq!(rate, 0.0);
            } else {
                assert!(rate > 0.0);
            }
        }
    }
}
