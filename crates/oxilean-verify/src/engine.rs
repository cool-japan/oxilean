//! The streaming verify engine — the reusable core of the product.
//!
//! It reads a lean4export NDJSON stream (any [`BufRead`]), replays each
//! declaration through the `oxilean-kernel` checker via the `oxilean-export`
//! [`Replayer`], and emits one [`DeclEvent`] per declaration as it completes,
//! followed by a final [`Summary`]. This module is deliberately UI-free: the
//! native CLI and the future WASM front-end both drive it through
//! [`verify_stream`] and render the events themselves.
//!
//! ## The three buckets (engineering brief §7)
//!
//! Every declaration lands in exactly one of [`Verdict::Verified`],
//! [`Verdict::Unsupported`], or [`Verdict::Rejected`]. These are wired
//! separately from day one: `rejected` is an alarm and is never summed with
//! `unsupported`.
//!
//! ## Malformed input is not a rejection
//!
//! A broken file (bad JSON, a spec violation, a materialization-budget blowout)
//! is reported as a [`VerifyError`], distinct from any per-declaration verdict.
//! The caller maps it to a non-verdict exit code. "This file is broken" and
//! "this proof is wrong" are different conversations and never share a channel.

use oxilean_kernel::wall_clock::Instant;
use std::io::BufRead;

use oxilean_export::{
    read_streaming, ExportError, Limits, Position, ReadStats, ReplayOutcome, Replayer,
};

use crate::pins::EnvironmentPins;

/// The verdict for a single declaration. Exactly one of the three buckets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Verdict {
    /// The kernel checked it and it is a proof. Carries the wall time spent
    /// replaying this declaration, in microseconds.
    Verified {
        /// Microseconds spent checking this declaration.
        micros: u64,
    },
    /// The declaration is valid but this kernel/reader version does not
    /// implement a feature it needs. Carries the *named* missing feature.
    Unsupported {
        /// A stable feature string naming what is missing.
        feature: &'static str,
    },
    /// The kernel checked it and says it is **not** a proof. This is an alarm.
    Rejected {
        /// A human-readable rejection reason (rendered kernel error).
        reason: String,
    },
}

impl Verdict {
    /// The stable bucket label used in JSON and machine output.
    #[must_use]
    pub fn label(&self) -> &'static str {
        match self {
            Verdict::Verified { .. } => "verified",
            Verdict::Unsupported { .. } => "unsupported",
            Verdict::Rejected { .. } => "rejected",
        }
    }
}

/// A per-declaration event, streamed to the caller as each declaration is
/// checked.
#[derive(Debug, Clone)]
pub struct DeclEvent {
    /// The declaration's primary name (fully qualified).
    pub name: String,
    /// The declaration kind label (`"axiom"`, `"def"`, `"thm"`, ...).
    pub kind: &'static str,
    /// The verdict.
    pub verdict: Verdict,
    /// The 0-based index of this declaration in file order.
    pub index: usize,
}

/// The final three-bucket summary for one file.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Summary {
    /// Declarations verified.
    pub verified: usize,
    /// Declarations deferred as unsupported.
    pub unsupported: usize,
    /// Declarations rejected (the alarm bucket).
    pub rejected: usize,
    /// Total declarations seen (`verified + unsupported + rejected`).
    pub total: usize,
    /// Wall time for the whole file, in microseconds.
    pub wall_micros: u64,
}

impl Summary {
    /// Throughput in declarations per second, computed from [`Summary::total`]
    /// and [`Summary::wall_micros`]. Returns `0.0` when no time elapsed or no
    /// declarations were seen (avoids a division by zero without unwrapping).
    #[must_use]
    pub fn decls_per_sec(&self) -> f64 {
        if self.wall_micros == 0 || self.total == 0 {
            return 0.0;
        }
        (self.total as f64) * 1_000_000.0 / (self.wall_micros as f64)
    }

    /// Wall time in whole milliseconds (integer, for the deterministic JSON
    /// report). Rounds to nearest.
    #[must_use]
    pub fn wall_ms(&self) -> u64 {
        (self.wall_micros + 500) / 1_000
    }
}

/// The complete outcome of verifying one file: the environment pins read from
/// its header plus the record statistics and the summary. Returned by
/// [`verify_stream`] alongside the streamed events.
#[derive(Debug, Clone)]
pub struct FileReport {
    /// Provenance pins (tool version + format/toolchain fingerprints).
    pub pins: EnvironmentPins,
    /// Record counts from the reader.
    pub stats: ReadStats,
    /// The three-bucket summary.
    pub summary: Summary,
}

/// Options controlling a verify run.
#[derive(Debug, Clone, Copy, Default)]
pub struct VerifyOptions {
    /// Reader resource limits (materialization budget). `Limits::default()` is
    /// the small, untrusted-input budget.
    pub limits: Limits,
    /// Stop at the first [`Verdict::Rejected`] declaration.
    pub fail_fast: bool,
    /// Deterministic per-declaration resource budget, in kernel `Expr` nodes
    /// cloned (`None` = unlimited). A declaration that exhausts it lands in
    /// the *unsupported* bucket with the named feature
    /// [`oxilean_export::RESOURCE_LIMIT`] — one declaration can never OOM
    /// the process. See `oxilean_kernel::fuel`.
    pub per_decl_fuel: Option<u64>,
    /// Wall-clock per-declaration deadline (`None` = none). A backstop for
    /// reduction/def-eq loops that make no *metered* progress and so never
    /// exhaust the deterministic fuel; an over-time declaration is degraded like
    /// a fuel-exhausted one and reported as [`oxilean_export::RESOURCE_LIMIT`],
    /// never a hang. Machine-dependent, so it only bounds an otherwise-unbounded
    /// declaration — it never changes a verdict.
    pub per_decl_time_budget: Option<std::time::Duration>,
}

/// A failure to *complete* a verify run — distinct from any per-declaration
/// verdict. These map to the CLI's exit code 2 (usage/IO/malformed), never to
/// the rejection alarm.
#[derive(Debug)]
pub enum VerifyError {
    /// The export file is broken: invalid JSON, a spec violation, a bad index
    /// reference. **Not** a proof rejection.
    Malformed {
        /// Where the problem was detected in the stream.
        pos: Position,
        /// A human-readable description.
        message: String,
    },
    /// The reader itself hit a valid-but-unsupported construct (e.g. the
    /// materialization budget was exceeded) that prevented it from finishing
    /// the file. Carries the named feature. Also not a proof rejection.
    ReaderUnsupported {
        /// Where the construct appeared.
        pos: Position,
        /// The named missing capability.
        feature: &'static str,
    },
    /// An internal invariant in the reader was violated. Never expected.
    Internal {
        /// Where the invariant was checked.
        pos: Position,
        /// A description of the violated invariant.
        message: String,
    },
}

impl std::fmt::Display for VerifyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VerifyError::Malformed { pos, message } => {
                write!(f, "broken export file at {pos}: {message}")
            }
            VerifyError::ReaderUnsupported { pos, feature } => {
                write!(f, "cannot read this file at {pos}: {feature}")
            }
            VerifyError::Internal { pos, message } => {
                write!(f, "internal reader error at {pos}: {message}")
            }
        }
    }
}

impl std::error::Error for VerifyError {}

impl VerifyError {
    fn from_export(e: ExportError) -> Self {
        match e {
            ExportError::Malformed { pos, message } => VerifyError::Malformed { pos, message },
            ExportError::Unsupported { pos, feature } => {
                VerifyError::ReaderUnsupported { pos, feature }
            }
            ExportError::Internal { pos, message } => VerifyError::Internal { pos, message },
        }
    }
}

/// The internal sentinel message used to abort the reader loop for `fail_fast`.
/// It never escapes this module: [`verify_stream`] recognises it and returns a
/// clean summary instead of an error.
const FAIL_FAST_SENTINEL: &str = "__oxilean_verify_fail_fast__";

/// Verify a lean4export NDJSON stream, invoking `on_event` for each declaration
/// as it is checked, and returning the file report (pins + stats + summary).
///
/// `on_event` sees declarations in file order; its `index` field is the 0-based
/// declaration position. The callback cannot abort the run — abort is driven by
/// [`VerifyOptions::fail_fast`], which stops after the first rejection.
///
/// # Errors
/// Returns a [`VerifyError`] if the file cannot be read to completion (broken
/// input, reader-unsupported construct, or I/O). A per-declaration rejection is
/// **not** an error — it is a [`Verdict::Rejected`] event and is reflected in
/// the returned [`Summary`].
pub fn verify_stream<R, F>(
    reader: R,
    tool_version: &'static str,
    options: VerifyOptions,
    mut on_event: F,
) -> Result<FileReport, VerifyError>
where
    R: BufRead,
    F: FnMut(&DeclEvent),
{
    let mut replayer = match Replayer::new() {
        Ok(r) => r,
        Err(e) => {
            return Err(VerifyError::Internal {
                pos: Position::line_only(0),
                message: format!("kernel builtin init failed: {e}"),
            });
        }
    };
    replayer.set_per_decl_fuel(options.per_decl_fuel);
    replayer.set_per_decl_time_budget(options.per_decl_time_budget);

    let mut summary = Summary::default();
    let mut index = 0usize;
    let mut fail_fast_hit = false;

    let start = Instant::now();
    let read_result = read_streaming(reader, options.limits, |decl| {
        // Time only the kernel replay for the per-decl verdict.
        let decl_start = Instant::now();
        let entry = replayer.replay(&decl);
        let micros = decl_start.elapsed().as_micros() as u64;

        let verdict = match entry.outcome {
            ReplayOutcome::Checked => {
                summary.verified += 1;
                Verdict::Verified { micros }
            }
            ReplayOutcome::Unsupported { feature } => {
                summary.unsupported += 1;
                Verdict::Unsupported { feature }
            }
            ReplayOutcome::Rejected { reason } => {
                summary.rejected += 1;
                Verdict::Rejected { reason }
            }
        };

        let is_rejection = matches!(verdict, Verdict::Rejected { .. });
        let event = DeclEvent {
            name: entry.name.to_string(),
            kind: entry.kind,
            verdict,
            index,
        };
        on_event(&event);
        summary.total += 1;
        index += 1;

        if is_rejection && options.fail_fast {
            fail_fast_hit = true;
            // Abort the reader loop with a private sentinel we recognise below.
            return Err(ExportError::internal(
                Position::line_only(0),
                FAIL_FAST_SENTINEL,
            ));
        }
        Ok(())
    });

    summary.wall_micros = start.elapsed().as_micros() as u64;

    match read_result {
        Ok((meta, stats)) => {
            let pins = EnvironmentPins::from_meta(tool_version, &meta);
            Ok(FileReport {
                pins,
                stats,
                summary,
            })
        }
        // We stopped early on purpose (fail-fast). The reader did not finish, so
        // its Meta is unavailable; the summary is authoritative for the exit
        // code, which is the whole point of fail-fast. Synthesise pins with an
        // unknown format version.
        Err(ExportError::Internal { message, .. })
            if fail_fast_hit && message == FAIL_FAST_SENTINEL =>
        {
            Ok(FileReport {
                pins: EnvironmentPins::without_meta(tool_version),
                stats: ReadStats::default(),
                summary,
            })
        }
        Err(e) => Err(VerifyError::from_export(e)),
    }
}
