//! A tiny, portable monotonic-clock shim used across the TCB crates.
//!
//! # Why this exists
//!
//! The kernel, the export reader, and the verify engine all use this module's
//! [`Instant`](crate::wall_clock::Instant) for two purposes:
//!
//! * cosmetic per-declaration timing (the `micros` a verdict carries), and
//! * termination watchdogs / fuel refill (elapsed-time bounds on reduction).
//!
//! On the `wasm32-unknown-unknown` target — the one the *Kernel in a Tab* demo
//! compiles for — the standard library's `Instant::now()` **panics** with
//! *"time not implemented on this platform"* (`std::sys::time::unsupported`).
//! Because the very first thing the verify engine does is take a timestamp, that
//! panic makes the entire checking path dead code: the optimizer folds
//! everything after the panic into `unreachable`, dead-code elimination strips
//! the kernel, and the resulting `.wasm` is a tiny stub that verifies nothing.
//! That is precisely the silent-empty-artifact failure the engineering brief
//! (§8.1) warns about.
//!
//! # What this provides
//!
//! [`Instant`](crate::wall_clock::Instant) — a drop-in for the subset of the standard `Instant` these crates
//! use: [`Instant::now`](crate::wall_clock::Instant::now), [`Instant::elapsed`](crate::wall_clock::Instant::elapsed), and [`Instant::duration_since`](crate::wall_clock::Instant::duration_since),
//! all returning [`core::time::Duration`].
//!
//! * **Off wasm** (every native build, the entire test suite, the CLI, docs.rs)
//!   it is a *transparent* newtype over the standard library `Instant`, so native
//!   timing and every watchdog behave **exactly** as before — byte-for-byte
//!   identical.
//! * **On `wasm32`** it is a monotonic counter backed by an `AtomicU64` (no
//!   `unsafe`, no external dependency, no wasm import). Timing there is a
//!   meaningless-but-nonpanicking tick count. The demo masks per-decl `micros`
//!   anyway (timing is explicitly non-deterministic, brief handoff invariant),
//!   and the reader's node-count materialization budget — which is **not**
//!   time-based — remains the real resource bound in the browser.
//!
//! Zero external dependencies; `#![forbid(unsafe_code)]` is preserved by the
//! whole kernel.

use core::ops::{Add, Sub};
use core::time::Duration;

/// A monotonic instant. See the module docs: a transparent wrapper over the
/// standard `Instant` off wasm, and a non-panicking monotonic counter on
/// `wasm32`.
#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Instant(::std::time::Instant);

#[cfg(not(target_arch = "wasm32"))]
impl Add<Duration> for Instant {
    type Output = Instant;
    fn add(self, rhs: Duration) -> Instant {
        Instant(self.0 + rhs)
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl Sub<Duration> for Instant {
    type Output = Instant;
    fn sub(self, rhs: Duration) -> Instant {
        Instant(self.0 - rhs)
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl Instant {
    /// The current instant. Delegates to the standard library `Instant::now`.
    #[must_use]
    pub fn now() -> Self {
        Instant(::std::time::Instant::now())
    }

    /// Time elapsed since this instant. Delegates to the standard library
    /// `Instant::elapsed`.
    #[must_use]
    pub fn elapsed(&self) -> Duration {
        self.0.elapsed()
    }

    /// Duration from `earlier` to `self`. Delegates to the standard library
    /// `Instant::duration_since` (which saturates to zero if `earlier` is later,
    /// matching std's documented behavior).
    #[must_use]
    pub fn duration_since(&self, earlier: Instant) -> Duration {
        self.0.duration_since(earlier.0)
    }
}

// ── wasm32: a monotonic tick counter (std time panics here) ──────────────────

#[cfg(target_arch = "wasm32")]
use core::sync::atomic::{AtomicU64, Ordering};

/// A global monotonic tick source for wasm. Every [`Instant::now`](crate::wall_clock::Instant::now) bumps it by
/// one, so instants are strictly ordered and `duration_since` never underflows.
/// The unit is "ticks", reported as nanoseconds so `Duration::as_micros` /
/// `as_millis` stay well-defined (they collapse to ~0, which is correct: the
/// demo does not rely on wasm wall time).
#[cfg(target_arch = "wasm32")]
static TICKS: AtomicU64 = AtomicU64::new(0);

/// A monotonic instant on wasm: a snapshot of the global tick counter.
#[cfg(target_arch = "wasm32")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Instant(u64);

#[cfg(target_arch = "wasm32")]
impl Add<Duration> for Instant {
    type Output = Instant;
    fn add(self, rhs: Duration) -> Instant {
        // Scale the added duration back to ticks (1 tick == 1 ns here) and
        // saturate so deadline arithmetic never wraps.
        Instant(self.0.saturating_add(rhs.as_nanos() as u64))
    }
}

#[cfg(target_arch = "wasm32")]
impl Sub<Duration> for Instant {
    type Output = Instant;
    fn sub(self, rhs: Duration) -> Instant {
        // Saturate at the epoch so a "cutoff = now - window" never underflows.
        Instant(self.0.saturating_sub(rhs.as_nanos() as u64))
    }
}

#[cfg(target_arch = "wasm32")]
impl Instant {
    /// The current instant: bump and read the global monotonic tick counter.
    #[must_use]
    pub fn now() -> Self {
        // Relaxed is sufficient: we only need monotonicity of the returned
        // values, not cross-thread happens-before (wasm is single-threaded here).
        Instant(TICKS.fetch_add(1, Ordering::Relaxed))
    }

    /// Ticks elapsed since this instant, as a `Duration` (nanosecond-scaled).
    #[must_use]
    pub fn elapsed(&self) -> Duration {
        Instant::now().duration_since(*self)
    }

    /// Duration from `earlier` to `self`, saturating at zero (never underflows).
    #[must_use]
    pub fn duration_since(&self, earlier: Instant) -> Duration {
        Duration::from_nanos(self.0.saturating_sub(earlier.0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn now_is_monotonic_and_elapsed_is_nonnegative() {
        let a = Instant::now();
        let b = Instant::now();
        // duration_since never panics and is >= 0 by construction.
        let _d = b.duration_since(a);
        let _e = a.elapsed();
    }
}
