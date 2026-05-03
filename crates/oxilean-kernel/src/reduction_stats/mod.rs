//! Reduction Statistics Tracking for the OxiLean kernel.
//!
//! This module provides [`ReductionStats`] — a lightweight, zero-dependency
//! accumulator for counting and profiling WHNF reductions.
//!
//! # Tracked quantities
//!
//! | Field         | Meaning                                             |
//! |---------------|-----------------------------------------------------|
//! | `beta_count`  | β-reductions `(λ x, t) a → t[a/x]`                |
//! | `delta_count` | δ-reductions (definition unfoldings)                |
//! | `zeta_count`  | ζ-reductions `let x := v in t → t[v/x]`           |
//! | `iota_count`  | ι-reductions (recursor / match evaluations)         |
//! | `eta_count`   | η-reductions                                        |
//! | `level_count` | Universe-level simplifications                      |
//! | `total_steps` | Sum of all counters                                 |
//! | `max_depth`   | Maximum recursion depth seen in a single WHNF call  |
//!
//! # Usage
//!
//! ```
//! use oxilean_kernel::reduction_stats::ReductionStats;
//!
//! let mut stats = ReductionStats::new();
//! stats.increment_beta();
//! stats.increment_delta();
//! assert_eq!(stats.total(), 2);
//! assert_eq!(stats.beta_count, 1);
//! ```
//!
//! ## Depth tracking
//!
//! Use [`DepthGuard`] to automatically push/pop the depth counter in
//! recursive WHNF functions:
//!
//! ```ignore
//! use oxilean_kernel::reduction_stats::{ReductionStats, DepthGuard};
//!
//! fn whnf_recursive(expr: &Expr, stats: &mut ReductionStats) -> Expr {
//!     let _guard = DepthGuard::new(stats);
//!     // ... reduction logic ...
//!     todo!()
//! }
//! ```
//!
//! ## Session-level deltas
//!
//! [`ReductionSession`] captures a before/after delta for a single
//! type-checking task:
//!
//! ```ignore
//! use oxilean_kernel::reduction_stats::{ReductionStats, ReductionSession};
//!
//! let mut stats = ReductionStats::new();
//! let session = ReductionSession::begin(&stats);
//! // ... run type checker ...
//! let delta = session.finish(&stats);
//! println!("Steps: {}", delta.total());
//! ```

pub mod functions;
pub mod types;

pub use functions::*;
pub use types::*;
