//! Where a declaration's work goes: counts of the constants the kernel
//! unfolds and the recursors it steps (feature `profile-unfold`).
//!
//! With the feature off every function here is an empty inline function. With
//! it on, each call adds one to a per-thread count for the constant, and
//! [`report_and_reset`](crate::profile::report_and_reset) — called when the next declaration's budget is armed
//! (`fuel::set_budget`) — prints the largest counts to stderr if the
//! declaration just finished used more than [`REPORT_MIN_FUEL`](crate::profile::REPORT_MIN_FUEL) units. A
//! diagnostic only: it reads names, decides nothing, changes no verdict.

use crate::Name;

/// A declaration is reported when it used at least this many units of fuel.
pub const REPORT_MIN_FUEL: u64 = 1 << 22;

/// What was counted.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Event {
    /// The reducer unfolded the constant (delta).
    ReduceUnfold,
    /// The def-eq checker unfolded the constant (lazy delta).
    DefEqUnfold,
    /// The reducer stepped the recursor (iota).
    Iota,
}

#[cfg(feature = "profile-unfold")]
mod imp {
    use super::{Event, Name, REPORT_MIN_FUEL};
    use std::cell::RefCell;
    use std::collections::HashMap;

    thread_local! {
        static COUNTS: RefCell<HashMap<(Event, Name), u64>> = RefCell::new(HashMap::new());
    }

    pub fn note(event: Event, name: &Name) {
        COUNTS.with(|c| *c.borrow_mut().entry((event, name.clone())).or_insert(0) += 1);
    }

    pub fn report_and_reset(fuel_used: u64) {
        COUNTS.with(|c| {
            let mut map = c.borrow_mut();
            if fuel_used >= REPORT_MIN_FUEL && !map.is_empty() {
                let mut rows: Vec<(&(Event, Name), &u64)> = map.iter().collect();
                rows.sort_by(|a, b| b.1.cmp(a.1));
                let total: u64 = map.values().sum();
                eprintln!("profile-unfold: declaration used {fuel_used} units; {total} events");
                for ((event, name), n) in rows.into_iter().take(40) {
                    eprintln!("profile-unfold:   {n:>10}  {event:?}  {name}");
                }
            }
            map.clear();
        });
    }
}

/// Count one event for `name`.
#[inline]
pub fn note(event: Event, name: &Name) {
    #[cfg(feature = "profile-unfold")]
    imp::note(event, name);
    #[cfg(not(feature = "profile-unfold"))]
    let _ = (event, name);
}

/// Report the counts of the declaration that used `fuel_used` units (when
/// it used enough), then start over.
#[inline]
pub fn report_and_reset(fuel_used: u64) {
    #[cfg(feature = "profile-unfold")]
    imp::report_and_reset(fuel_used);
    #[cfg(not(feature = "profile-unfold"))]
    let _ = fuel_used;
}
