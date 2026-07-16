//! Fuzz target: bytes → parse + replay into a fresh kernel environment.
//!
//! This is the **strongest** of the three fuzz targets: it runs the full
//! pipeline — NDJSON reader → kernel type-checker — under tight resource
//! limits. Any panic or OOM that is reachable from untrusted bytes is a P0
//! bug.
//!
//! # What this tests
//!
//! The reader pipeline (`fuzz_ndjson_parse`) already guards against panics
//! in the reader. This target goes further: it also exercises the **kernel**
//! with arbitrary declaration shapes. A well-formed-but-adversarial NDJSON
//! file could construct expressions that trigger panics deep inside the
//! kernel's definitional equality, WHNF reduction, or inductive family checks.
//!
//! # Invariant
//!
//! For *any* byte sequence, `replay_streaming` must not panic, must not OOM
//! (the node budget is tight), and must not produce a `Rejected` outcome for
//! reasons other than genuine kernel rejections (not a crash). The three-bucket
//! structure must be maintained: `Ok(_)` from `replay_streaming` means the
//! reader was well-formed (individually malformed declarations land in
//! `Rejected`, not as reader errors).

#![no_main]

use libfuzzer_sys::fuzz_target;
use oxilean_export::{replay_streaming, Limits, ReplayLimits};

fuzz_target!(|data: &[u8]| {
    // Tight limits for the fuzzer: the reader budget is smaller than the
    // production default (2^20 vs 2^26 nodes) to keep each fuzz iteration
    // fast while still exercising all code paths. The replay itself is not
    // time-limited (no per_decl_time_budget) so the fuzzer can find slow
    // paths — but the node budget caps memory.
    let limits = ReplayLimits {
        read: Limits {
            materialize_budget: 1 << 20,
            // Same tight cap per declaration (C22): one adversarial decl must
            // not dominate an iteration any more than the whole file may.
            decl_materialize_budget: 1 << 20,
        },
        per_decl_time_budget: None,
        // Hard, deterministic per-declaration kernel fuel (C16): tight enough
        // (2^20 cloned nodes) that the fuzzer finds logic bugs, not allocator
        // throughput; over-fuel declarations land in the NAMED resource-limit
        // bucket, never a panic or an OOM.
        per_decl_fuel: Some(1 << 20),
    };

    // The replay itself must not panic or OOM regardless of the input.
    // We do not care about the outcome — `Ok` or `Err` are both fine; a
    // crash is not.
    let _ = replay_streaming(std::io::Cursor::new(data), limits);
});
