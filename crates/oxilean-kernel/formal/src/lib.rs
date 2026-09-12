//! `oxilean-kernel-formal`: bounded model checking of `oxilean-kernel`'s
//! public `BigNat` API with [`cargo-formal`].
//!
//! This package verifies nothing about itself. It is a *specification*
//! package: [`harness`] states properties of [`oxilean_kernel::bignat::BigNat`]
//! and `cargo formal check` decides them with a bit-vector SMT solver.
//! `oxilean-kernel` is a plain path dependency and is never modified.
//!
//! # Why `BigNat`
//!
//! `bignat` is, in its own module docs, "part of the trusted computing base:
//! deliberately small, self-contained, and audited by eye". It backs
//! `Literal::Nat`, so every `Nat` literal reduction the kernel performs goes
//! through the limb arithmetic here. A carry, borrow or normalisation bug in
//! it would not be a wrong number — it would be a wrong *proof*. That makes
//! it the sharpest available self-host target, and the package sets
//! `self-host = true` in `[package.metadata.formal]` so the report says so.
//!
//! # Scope: the public API only
//!
//! Every harness calls `BigNat` through `pub` items — `from_limbs`,
//! `as_limbs`, `limb_count`, `is_zero`, `is_one`, `to_u64`, `to_u32`,
//! `bit_length`, `add`, `sub`, `beq`, `ble`, `land`, `lor`, `lxor`, `shr`,
//! and `Ord::cmp`. The limb-level helpers (`add_limbs`, `sub_limbs`,
//! `cmp_limbs`, `shl_bits`, `shr_bits`) are private and are reached only
//! *through* those entry points, which is the honest statement of what a
//! caller of the crate can rely on.
//!
//! # The three builds
//!
//! Exactly the contract `examples/checked-arith` in the `cargo-formal`
//! repository spells out, and each build proves something different:
//!
//! 1. **`cargo build`** (plain, stable). Harness bodies vanish entirely —
//!    neither `#[cfg(formal)]` nor `#[cfg(all(test, oxiformal_runtime_checks))]`
//!    applies — so this only checks that the package type-checks on stable
//!    with no warnings. `cargo test` in this mode runs the `plain_tests`
//!    module: concrete witnesses for the interesting rows plus sanity checks
//!    that pin the properties the harnesses state.
//! 2. **`RUSTFLAGS="--cfg oxiformal_runtime_checks" cargo test`**. Every
//!    harness becomes a `#[test]` that loops `oxiformal::rt::iterations`
//!    (256 by default) times through a random draw. A harness is marked
//!    `#[should_panic]` only where the randomized run really panics; none of
//!    the harnesses here is, and the report for this package records that
//!    measurement.
//! 3. **`cargo +nightly-2026-06-20 check` with `--cfg formal
//!    -Zcrate-attr=feature(register_tool) -Zcrate-attr=register_tool(formal_tool)`**
//!    and a separate `--target-dir` — exactly the flags the `cargo-formal`
//!    driver build uses. This type-checks the `#[cfg(formal)]` copy of every
//!    harness; it does not run the driver or the solver, so it says nothing
//!    about which harnesses are `proved`. That is what `EXPECTED.toml`
//!    records, **measured** by a real `cargo formal check` run.
//!
//! Then:
//!
//! ```text
//! FORMAL_DRIVER=<path to formal-driver> cargo formal check
//! ```
//!
//! # Reading a verdict
//!
//! `proved` means the solver found no input, within the harness's bounds,
//! that violates the property. `refuted` means a concrete counterexample
//! exists. `unknown` means the solver did not decide it; in this package the
//! reason is almost always `solver-model-rejected` — see below. `unsupported`
//! means the encoder could not build a verification condition at all, and is
//! a statement about the encoder (or about a deliberate soundness rule),
//! never about the code.
//!
//! # Why so many `unknown`s: cargo-formal is pinned to OxiZ 0.3.3
//!
//! `cargo-formal` depends on the SMT solver `oxiz` at a crates.io pin of
//! `=0.3.3`, which answers `sat` with a model that does not satisfy the
//! formula for a class of Boolean-structure-over-bit-vector queries
//! (upstream item U-Z10). `cargo-formal` runs a **mandatory model check** on
//! every reported counterexample, so such an answer is reported as `unknown`
//! and never as a `refuted` with a fabricated witness. Those rows are
//! labelled `solver-model-rejected` in the report and in `EXPECTED.toml`.
//! They are not encoder gaps; they move to `proved` once the pin moves to an
//! OxiZ release carrying the fix.
//!
//! [`cargo-formal`]: https://github.com/cool-japan/cargo-formal

#![forbid(unsafe_code)]

pub mod harness;
