//! # RtArc - Trait Implementations
//!
//! This module contains trait implementations for `RtArc`.
//!
//! ## Implemented Traits
//!
//! - `Debug`
//! - `Display`
//! - `PartialEq`
//! - `Eq`
//!
//! `Send` and `Sync` are the auto traits: `RtArc<T>` holds a single
//! `std::sync::Arc<ArcInner<T>>`, and `ArcInner<T>` holds a `T` and an
//! `AtomicU32`, so `RtArc<T>` is `Send` and `Sync` exactly when
//! `T: Send + Sync` without an `unsafe impl`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::RtArc;
use std::fmt;

impl<T: fmt::Debug> fmt::Debug for RtArc<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RtArc")
            .field("value", &self.inner.value)
            .field("strong_count", &self.strong_count())
            .field("weak_count", &self.weak_count())
            .finish()
    }
}

impl<T: fmt::Display> fmt::Display for RtArc<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.value.fmt(f)
    }
}

impl<T: PartialEq> PartialEq for RtArc<T> {
    fn eq(&self, other: &Self) -> bool {
        self.inner.value == other.inner.value
    }
}

impl<T: Eq> Eq for RtArc<T> {}

#[cfg(test)]
mod auto_trait_tests;
