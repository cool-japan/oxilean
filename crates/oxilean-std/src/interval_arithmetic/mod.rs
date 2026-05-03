//! # Interval Arithmetic
//!
//! Rigorous interval arithmetic provides enclosures for computed real numbers,
//! guaranteeing that the true result always lies within the computed interval.
//!
//! ## Core Concept
//!
//! An **interval** `[a, b]` represents the set of all real numbers `x` with `a ≤ x ≤ b`.
//! Arithmetic operations on intervals satisfy:
//!
//! ```text
//! \[a,b\] + \[c,d\] = \[a+c, b+d\]
//! \[a,b\] - \[c,d\] = \[a-d, b-c\]
//! \[a,b\] × \[c,d\] = \[min(ac,ad,bc,bd), max(ac,ad,bc,bd)\]
//! \[a,b\] / \[c,d\] = \[a,b\] × \[1/d, 1/c\]   (if 0 ∉ \[c,d\])
//! ```
//!
//! ## Kaucher Intervals (Extended/Directed Intervals)
//!
//! **Kaucher arithmetic** extends classical intervals to allow "improper" intervals
//! `[a, b]` with `a > b`. This forms a complete lattice and supports modal interpretations:
//! - **Proper** `[a,b]` (a ≤ b): "for all x in \[a,b\]" — existential guarantee
//! - **Improper** `[b,a]` (b < a): "for all x in \[b,a\]" — universal guarantee
//!
//! ## Dependency Problem
//!
//! The main limitation: when the same variable appears multiple times in an expression,
//! interval arithmetic over-approximates because it treats each occurrence independently.
//! Example: `x - x ≠ \[0,0\]` in general (gives `[-w, w]` where `w = b - a`).
//!
//! ## Verified Computation
//!
//! Using outward rounding (rounding lower bounds down, upper bounds up), interval
//! arithmetic provides **rigorous enclosures** even in floating-point arithmetic.

pub mod functions;
pub mod types;

pub use functions::*;
pub use types::*;
