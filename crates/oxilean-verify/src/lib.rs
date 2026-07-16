//! # oxilean-verify — an independent Lean 4 proof checker
//!
//! This crate is the **product** of the OxiLean verify campaign: a streaming
//! verify engine and the `oxilean-verify` CLI that drives it. It reads
//! [`lean4export`](https://github.com/leanprover/lean4export) NDJSON files, hands
//! each declaration to the `oxilean-kernel` type checker via the
//! `oxilean-export` reader, and reports a verdict for every one.
//!
//! ## The dependency closure *is* the product
//!
//! A reviewer auditing this checker audits its entire dependency closure. That
//! closure is exactly two crates — `oxilean-kernel` (the trusted computing base)
//! and `oxilean-export` (the reader) — and nothing else. There is deliberately
//! **no `clap` and no `serde`**: argument parsing ([`cli`]) and JSON output
//! ([`jsonw`], [`report`]) are hand-rolled here, and even the input fingerprint
//! ([`sha256`]) is a short, auditable transcription rather than a crate. The
//! crate is `#![forbid(unsafe_code)]`.
//!
//! ## Three buckets, always (engineering brief §7)
//!
//! Every declaration lands in exactly one of [`Verdict::Verified`],
//! [`Verdict::Unsupported`], or [`Verdict::Rejected`]. `rejected` is an **alarm**
//! and is wired separately from `unsupported` everywhere. Broken input is a
//! third, orthogonal thing — a [`VerifyError`], never a rejection — because
//! *"this file is broken"* and *"this proof is wrong"* are different
//! conversations.
//!
//! ## The engine is reusable
//!
//! [`verify_stream`] is UI-free: it takes any [`std::io::BufRead`], emits one
//! [`DeclEvent`] per declaration through a callback, and returns a
//! [`FileReport`]. The native CLI and the future WASM front-end both drive it
//! the same way.

#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![warn(clippy::all)]

pub mod cli;
pub mod engine;
pub mod jsonw;
pub mod pins;
pub mod report;
pub mod sha256;

pub use engine::{
    verify_stream, DeclEvent, FileReport, Summary, Verdict, VerifyError, VerifyOptions,
};
pub use pins::{EnvironmentPins, TOOL_NAME};
pub use report::{
    render_report, DeclRecord, InputMeta, Rejection, ReportAccumulator, UnsupportedGroup,
    UNSUPPORTED_DECL_CAP,
};

/// This tool's version, from `CARGO_PKG_VERSION` at build time.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

// Re-export the pins the reader is built against so downstream consumers (and
// the CLI `--version` output) can name them without depending on the reader
// crate directly.
pub use oxilean_export::{LEAN4EXPORT_COMMIT, LEAN_TOOLCHAIN, NDJSON_FORMAT_VERSION};
