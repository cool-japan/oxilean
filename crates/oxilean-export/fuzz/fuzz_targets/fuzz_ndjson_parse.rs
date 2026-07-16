//! Fuzz target: arbitrary bytes through the full NDJSON reader pipeline.
//!
//! Invariant under fuzz: the reader NEVER panics, never overflows the stack
//! (nesting and materialization are depth/budget-limited), and never OOMs
//! (input-derived allocations are capped by the materialization budget), for
//! any byte sequence. Errors are fine; crashes are bugs.

#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    // A tight budget (2^20 nodes ≈ tens of MB worst case) keeps the fuzzer
    // exploring parser logic instead of allocator throughput, while running
    // exactly the same code paths as the default budget.
    let limits = oxilean_export::Limits {
        materialize_budget: 1 << 20,
        // For fuzzing the cumulative budget is already the strongest bound;
        // give the per-declaration cap (C22) the same tight value so a single
        // adversarial declaration cannot dominate an iteration either.
        decl_materialize_budget: 1 << 20,
    };
    // Full pipeline: bytes -> lines -> JSON -> records -> kernel types.
    // The result (Ok or three-bucket error) is irrelevant; not crashing is
    // the property.
    let _ = oxilean_export::read_with_limits(std::io::Cursor::new(data), limits);
});
