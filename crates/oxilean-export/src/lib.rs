//! # oxilean-export — the lean4export NDJSON reader
//!
//! This crate reads the [lean4export] NDJSON format and converts it into
//! `oxilean_kernel` types, feeding the oxilean-verify trusted computing base.
//! Per the engineering brief: *"The reader is a week of work. The checking is
//! the project."* — so this crate stays lean, auditable, and depends on nothing
//! but the kernel.
//!
//! ## Pins
//!
//! This reader targets a specific, pinned version of the format and toolchain:
//!
//! * **lean4export commit:** `3de59f10bc4b4a0f2de698597aeb1246caa0df0a`
//! * **Lean toolchain:** `leanprover/lean4:v4.32.0-rc1`
//! * **NDJSON format version:** `3.1.0` (major version `3` is required; the
//!   reader also accepts `3.0.0` exports, which differ in a few record shapes —
//!   see the discrepancy notes below).
//!
//! ## Pipeline
//!
//! 1. [`read`] / [`read_str`] stream the NDJSON, one record per line, from any
//!    [`std::io::BufRead`]. Records are dispatched and interned into dense,
//!    Vec-based index tables (names, levels, expressions), with the
//!    forward-reference and duplicate-index discipline enforced.
//! 2. Declarations are surfaced as kernel-typed [`ExportDecl`] values.
//! 3. [`replay_file`] / [`replay_streaming`] drive an `oxilean_kernel`
//!    [`oxilean_kernel::Environment`] from those declarations, starting from
//!    an **empty** environment (a true checker: every constant — inductives,
//!    constructors, kernel-re-derived recursors, the four quotient
//!    primitives — must come from the export itself). See [`replay`].
//!
//! ## Error taxonomy
//!
//! Every failure is one of three buckets ([`ExportError`]): malformed input
//! (the file is broken), an unsupported-but-valid construct (carrying a *named*
//! feature), or an internal invariant violation. The reader never panics; this
//! is enforced by the `fuzz_ndjson_parse` fuzz target.
//!
//! ## Spec ⇄ fixture discrepancies (fixtures are ground truth)
//!
//! * The spec documents the `inductive` record with keys `types`/`ctors`/`recs`
//!   (format 3.1.0). The 3.0.0 fixtures instead use
//!   `inductiveVals`/`constructorVals`/`recursorVals`. The reader accepts both.
//! * The spec shows `{"thm": {...}}` and `{"def": {...}}` as single objects. In
//!   3.0.0 exports these payloads are single-element arrays
//!   (`{"thm": [{...}]}`), used to carry mutual groups. The reader accepts an
//!   object *or* an array of member objects for `def`/`thm`/`opaque`.
//! * The spec pins format 3.1.0, but the `Nat.add_succ` fixture is a valid
//!   3.0.0 export. Major version `3` is what the reader validates.
//!
//! [lean4export]: https://github.com/leanprover/lean4export

#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![warn(clippy::all)]

pub mod error;
pub mod json;
pub mod model;
pub mod replay;

mod convert;
mod reader;

pub use error::{ExportError, ExportResult, Position};
pub use model::{ExportDecl, InductiveBundle, Meta, OversizedVal};
pub use reader::{
    read, read_str, read_streaming, read_with_limits, ExportFile, Limits, ReadStats,
    BUDGET_FEATURE, CORPUS_DECL_MATERIALIZE_BUDGET, CORPUS_MATERIALIZE_BUDGET, DECL_BUDGET_FEATURE,
    DEFAULT_MATERIALIZE_BUDGET,
};
pub use replay::{
    replay_decl, replay_file, replay_streaming, ReplayEntry, ReplayLimits, ReplayOutcome,
    ReplayReport, ReplayRun, ReplayStats, Replayer, CORPUS_DECL_FUEL, DEFAULT_DECL_FUEL,
    DEFERRED_DEPENDENCY, NESTED_INDUCTIVES, RESOURCE_LIMIT,
};

/// The lean4export git commit this reader is pinned to.
pub const LEAN4EXPORT_COMMIT: &str = "3de59f10bc4b4a0f2de698597aeb1246caa0df0a";

/// The Lean toolchain version this reader is pinned to.
pub const LEAN_TOOLCHAIN: &str = "v4.32.0-rc1";

/// The NDJSON format version this reader targets.
pub const NDJSON_FORMAT_VERSION: &str = "3.1.0";

/// The required NDJSON format major version.
pub const NDJSON_FORMAT_MAJOR: u32 = 3;
