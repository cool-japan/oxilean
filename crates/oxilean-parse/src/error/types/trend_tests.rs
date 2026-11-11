//! `ErrorRateTracker::trend`: newest committed window minus oldest.

use super::*;

fn tracker_with(window_size: usize, per_window: &[usize]) -> ErrorRateTracker {
    let mut tracker = ErrorRateTracker::new(window_size);
    for &errors in per_window {
        tracker.record(errors);
        tracker.commit_window();
    }
    tracker
}

#[test]
fn fewer_than_two_windows_have_no_trend() {
    assert_eq!(tracker_with(3, &[]).trend(), 0.0);
    assert_eq!(tracker_with(3, &[5]).trend(), 0.0);
}

#[test]
fn two_windows_give_newest_minus_oldest() {
    assert_eq!(tracker_with(3, &[1, 4]).trend(), 3.0);
    assert_eq!(tracker_with(3, &[4, 1]).trend(), -3.0);
    assert_eq!(tracker_with(3, &[2, 2]).trend(), 0.0);
}

#[test]
fn trend_spans_only_the_windows_still_held() {
    assert_eq!(tracker_with(3, &[1, 9, 2, 6]).trend(), -3.0);
    assert_eq!(tracker_with(2, &[1, 4, 2]).trend(), -2.0);
}
