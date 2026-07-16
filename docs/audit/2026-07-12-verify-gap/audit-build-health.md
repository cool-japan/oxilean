# Build Health Baseline Audit — oxilean

**Date:** 2026-07-12  
**Branch:** 0.1.3 (clean, one untracked directory docs/specs/)  
**Auditor:** Subagent (read-only, no modifications made)

---

## 1. Workspace Structure

14 crates in `crates/`:
- oxilake, oxilean, oxilean-build, oxilean-cli, oxilean-codegen, oxilean-doc
- oxilean-elab, oxilean-kernel, oxilean-lint, oxilean-meta, oxilean-parse
- oxilean-runtime, oxilean-std, oxilean-wasm

Plus a workspace root pseudo-package `oxilean-workspace` (publish = false) with integration tests.

Workspace version: 0.1.2 (Cargo.toml; branch name is 0.1.3, suggesting version bump in progress).

---

## 2. cargo check --workspace --all-features

**Command:** `cargo check --workspace --all-features 2>&1 | tail -40`

**Result:** CLEAN

- Errors: **0**
- Warnings: **0**
- Build time: 37.83s (cold/incremental)
- Final line: `Finished 'dev' profile [unoptimized + debuginfo] target(s) in 37.83s`

All 14 workspace members compiled without issue, including external deps:
- clap v4.6.1, oxicode v0.2.4, oxiarc-deflate v0.3.5, rhai v1.25.1
- oxiz-math/solver v0.2.3, egg v0.11.0, js-sys v0.3.103
- dashmap v6.2.1, parking_lot v0.12.5, lasso v0.7.3
- wasm-bindgen-macro v0.2.126, serde-wasm-bindgen v0.6.5

---

## 3. cargo check -p oxilean-kernel (standalone TCB build)

**Command:** `cargo check -p oxilean-kernel 2>&1 | tail -10`

**Result:** CLEAN

- Errors: **0**
- Warnings: **0**
- Final line: `Finished 'dev' profile [unoptimized + debuginfo] target(s) in 0.03s` (cached)

**TCB dependency check (Cargo.toml):**
- `[dependencies]` section is empty — zero runtime dependencies confirmed.
- `[dev-dependencies]` allow-list: `criterion = "0.8"` (benches only), `proptest` (workspace, tests only).
- `#![forbid(unsafe_code)]` present at `crates/oxilean-kernel/src/lib.rs:263`
- No `unsafe` blocks exist in source (grep confirmed — only doc comments and identifier names like `is_unsafe`, `unsafe_axioms`).

**Kernel source size:**
- Total: ~146,570 lines of Rust source across all `.rs` files.
- Largest individual files: expr_cache/types.rs (1895), congruence/types.rs (1791), infer/types.rs (1776), termination/types.rs (1749).

---

## 4. cargo nextest run -p oxilean-kernel

**Command:** `cargo nextest run -p oxilean-kernel --no-fail-fast 2>&1 | tail -25`

**Result:** ALL PASS

```
Summary [1.951s] 3310 tests run: 3310 passed, 0 skipped
```

- 3310 tests across 2 binaries
- Run time: ~2 seconds
- Includes property-based tests in `prop_tests` module:
  - `prop_numeric_level_normalize_fixed`
  - `prop_def_eq_distinct_lits_false`
  - `prop_whnf_lit_fixed_points`
  - `prop_name_str_roundtrip`
  - `prop_whnf_atoms_are_fixed_points`
  - `prop_name_roundtrip`
  - `prop_substitution_identity`
  - `prop_level_normalize_idempotent`
  - `prop_def_eq_reflexive`
  - `prop_whnf_idempotent`
  - `prop_lift_lower_bvars_identity`
  - `prop_instantiate_abstract_no_fvar`

No failures or skips.

---

## 5. cargo nextest run -p oxilean-wasm

**Command:** `cargo nextest run -p oxilean-wasm --no-fail-fast 2>&1 | tail -20`

**Result:** ALL PASS

```
Summary [0.035s] 59 tests run: 59 passed, 0 skipped
```

- 59 tests in `incremental` and `share` modules
- Run time: 0.035 seconds

Notable test modules:
- `incremental::functions::tests::*` — 15+ tests for incremental checking
- `share::tests::*` — base64url and compress/decompress roundtrip tests

---

## 6. Workspace Integration Tests (--workspace)

**Command:** `cargo nextest run --workspace --no-fail-fast 2>&1 | tail -3`

**Result:** ALL PASS (with skips)

```
Summary [27–30s] 32928 tests run: 32928 passed, 746 skipped
```

- 32,928 tests across 22 binaries
- 746 skipped (confirmed these are `#[ignore]`-tagged tests, not actual failures)
- Run time: ~30 seconds
- The "FAIL" / "failed" / "skipped" strings appearing in the grep output above are **test names** (e.g., `test_build_report_failed_modules`), not actual test failures.

**Workspace-root integration tests** (`tests/` directory, 4 test suites):
- `integration_test` — passed
- `mathlib_compat_test` — passed
- `mathlib_theorems_test` — passed
- `std_library_test` — passed (incl. `verify_nat_gcd_properties`, `verify_nat_add_zero`, etc.)
- `formal_proofs_test` — passed

---

## 7. cargo clippy -p oxilean-kernel --all-features

**Command:** `cargo clippy -p oxilean-kernel --all-features 2>&1 | tail -20`

**Result:** CLEAN

- Warnings: **0**
- Errors: **0**
- Final line: `Finished 'dev' profile [unoptimized + debuginfo] target(s) in 7.30s`

Clippy across entire workspace also produces 0 warnings and 0 errors.

---

## 8. Git Status

```
On branch 0.1.3
Your branch is up to date with 'origin/0.1.3'.

Untracked files:
  docs/specs/

nothing added to commit but untracked files present
```

Only untracked item: `docs/specs/lean4export-format.md` — a spec file written by a prior audit subagent. No staged or modified files. **Audit did not modify any repo file.**

---

## 9. Stub / Unimplemented Audit

**Kernel:**
- `todo!()` / `unimplemented!()` occurrences in kernel: **1** (in doc comment only):
  - `crates/oxilean-kernel/src/reduction_stats/mod.rs:42` — appears inside a `//!` doc comment example, not actual code.
- **No real stubs** in the kernel execution path.

**Legacy stub in inductive module:**
- `crates/oxilean-kernel/src/inductive/functions.rs:150–151`:
  ```rust
  /// Reduce a recursor application (iota-reduction).
  /// This is the legacy API. The new implementation is in `reduce.rs`.
  pub fn reduce_recursor(_rec_name: &Name, _args: &[Expr]) -> Option<Expr> {
      None
  }
  ```
  This function returns `None` unconditionally. It is labeled "legacy API" and the real implementation is in `reduce/types.rs:1160–1183` via `try_reduce_recursor`. This is a nominal stub but is not on any live call path.

**Other crates (oxilean-codegen):**
- `todo!()` appears in string templates for code generation output (not real stubs in Rust execution).

---

## 10. Key Observations Relevant to the oxilean-verify Brief

### 10.1 oxilean-verify / oxilean-export crate: ABSENT

Neither `oxilean-verify` nor `oxilean-export` crates exist in the workspace or on disk. The brief requires a new `oxilean-export` reader (lean4export NDJSON format, ≤3000 SLoC, zero deps, fuzzed) and an `oxilean-verify` CLI/WASM product. These are entirely missing.

The `docs/specs/lean4export-format.md` (untracked) documents the target format (NDJSON, version 3.1.0, lean4export commit `3de59f10`).

### 10.2 Nat Literal Storage: u64, NOT arbitrary-precision

`Literal::Nat(u64)` at `crates/oxilean-kernel/src/expr/types.rs:835`.

The brief (section 4.3) requires a **zero-dependency arbitrary-precision bignum** for Nat literal reduction. The current kernel uses `u64` — meaning Nat literals > 2^64-1 will overflow silently (Rust's `u64::pow` truncates, `m + n` wraps, etc.). This is a critical correctness gap for large-Nat proofs from Mathlib.

Current operations in `reduce/functions.rs` all use native Rust arithmetic on `u64`:
- `Nat.add`: `m + n` (wraps on overflow)
- `Nat.mul`: `m * n` (wraps on overflow)
- `Nat.pow`: `m.pow(*n as u32)` (u32 cast and potential overflow)
- `Nat.shiftLeft`: guarded by `if *n < 64` only (returns `None` rather than the correct large value for shift ≥ 64)

### 10.3 Quotient Types: PRESENT, APPEARS CORRECT

`Quot.lift f h (Quot.mk r a) → f a` reduction is implemented at:
- `crates/oxilean-kernel/src/quotient/functions.rs:76–96`
- Tests present and passing.

`QuotientKernel` struct at `crates/oxilean-kernel/src/quotient/types.rs:182`.

### 10.4 Structural Eta: PRESENT

`StructEta` module at `crates/oxilean-kernel/src/struct_eta/`:
- `is_structure_type()` and `eta_expand_struct()` tested.
- Multiple extended test modules (`tests_struct_eta_extended` through `tests_struct_eta_engine`).

### 10.5 Iota Reduction: PRESENT via reduce/types.rs

`try_reduce_recursor` at `crates/oxilean-kernel/src/reduce/types.rs:1155–1183`:
- Looks up `RecursorVal`, finds major argument, WHNFs it, pattern-matches constructor, applies `RecursorRule`.
- `instantiate_recursor_rhs` at `reduce/functions.rs:28–63` performs the actual substitution.
- The legacy `reduce_recursor()` in `inductive/functions.rs:150` returns `None` and is unused.

**Risk:** No evidence of nested or mutual inductive re-derivation of recursors — the brief requires "RE-DERIVING recursors ourselves rather than trusting exported ones, strict positivity re-check." Current code accepts `RecursorVal` from the environment directly without re-deriving. This is a brief gap.

### 10.6 Universe Level Definitional Equality: PRESENT

`level` module with `normalize`, `level_leq`, `level_eq`, `imax_simplify`, `substitute_level_params` — all listed in TODO.md as complete and passing proptest.

### 10.7 3-Verdict System: ABSENT

No `Verdict` enum with `Verified / Unsupported / Rejected` buckets exists anywhere in the codebase. The brief requires the CLI to exit non-zero on `Rejected`. This is part of the missing `oxilean-verify` product.

### 10.8 WASM Size Gate / Export-Count Gate: ABSENT

No CI gate checks WASM binary size against 400KB gzip limit. No export-count gate exists. The CI workflows are in `.github/workflows.disabled/` (disabled), and neither contains a size check. The npm-publish.yml in `.github/workflows/` shows the WASM size but does not gate on it.

### 10.9 cargo-fuzz on export reader: ABSENT

No `fuzz/` directory anywhere in the workspace. No fuzz targets. No `cargo-fuzz` configuration. The brief requires fuzzing of the export reader in CI.

### 10.10 Dependency Allow-List Violation (oxilean-wasm)

`oxilean-wasm/Cargo.toml` depends on:
- `oxilean-kernel`, `oxilean-parse`, `oxilean-elab`, `oxiarc-deflate`
- `wasm-bindgen`, `serde`, `serde_json`, `serde-wasm-bindgen`, `js-sys`

The brief says "dependency allow-list (kernel+export+wasm-bindgen only for the verify product)". The current wasm crate pulls in `oxilean-parse` and `oxilean-elab` which bring heavy dependencies (rhai, egg, parking_lot, etc.) — not conformant with the brief's allow-list for `oxilean-verify`.

---

## 11. CI Status

- Active workflow: only `npm-publish.yml` in `.github/workflows/` (triggered on tags or manual dispatch).
- `ci.yml` and `bench.yml` are **disabled** (in `workflows.disabled/`).
- No active CI runs tests, clippy, or fuzz on push/PR.
- The disabled `ci.yml` uses `cargo test` not `cargo nextest`, no fuzz step, no wasm size gate.

---

## 12. Summary Table

| Check | Result |
|-------|--------|
| `cargo check --workspace --all-features` | PASS (0 errors, 0 warnings) |
| `cargo check -p oxilean-kernel` | PASS (0 errors, 0 warnings) |
| `cargo nextest run -p oxilean-kernel` | PASS (3310/3310) |
| `cargo nextest run -p oxilean-wasm` | PASS (59/59) |
| `cargo nextest run --workspace` | PASS (32928/32928, 746 skipped) |
| `cargo clippy -p oxilean-kernel --all-features` | PASS (0 warnings) |
| `cargo clippy --workspace --all-features` | PASS (0 warnings) |
| `git status` | Clean (1 untracked dir, audit-introduced only) |
| `oxilean-verify` crate exists | NO — absent |
| `oxilean-export` reader crate exists | NO — absent |
| Nat literal bignum (arbitrary precision) | NO — uses u64 |
| 3-verdict system (Verified/Unsupported/Rejected) | NO — absent |
| WASM size gate (≤400KB gzip) | NO — absent |
| WASM export-count gate | NO — absent |
| cargo-fuzz on export reader | NO — absent |
| Active CI (non-disabled) | Only npm-publish.yml |
