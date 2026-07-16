# Audit: CLI Architecture and Test Infrastructure
## Repository: oxilean
## Date: 2026-07-12
## Auditor: Claude Sonnet 4.6

---

## 1. CLI Framework and Subcommand Architecture

### 1.1 oxilean-cli — The Main Binary Crate

**File**: `crates/oxilean-cli/Cargo.toml`

The `oxilean-cli` crate does NOT use clap or any external CLI framework.
Dependencies:
- `oxilean-kernel` (workspace)
- `oxilean-parse` (workspace)
- `oxilean-elab` (workspace)
- `oxilean-lint` (workspace)
- `oxilean-std` (workspace)

This is a **critical finding**: oxilean-cli already depends on the full stack
(kernel + parse + elab + lint + std), confirming the brief's requirement that
`oxilean-verify` must NOT live in this crate if the verify product's dep closure
is to be restricted to kernel+export only.

**CLI dispatch mechanism**: Manual `match args[1].as_str()` string dispatch.
See `crates/oxilean-cli/src/main/functions.rs` lines 17–52.

Registered subcommands (via match arm):
- `"check"` → `check_file(&args[2])`
- `"repl"` → `run_repl()`
- `"lsp"` | `"serve"` → `crate::lsp::server::run_server_stdio()`
- `"playground"` → `crate::commands::playground::run_playground(&config)`
- `"version"` → `print_version()`
- `"help"` → `print_help()`
- `_` → `check_file(&args[1])` (fallback)

**No `verify` subcommand exists** in the CLI. The `help` text
(`print_help()`, lines 84–105) does not mention a verify command.
The `commands_table()` function (lines 167–180) lists: check, repl, build,
bench, doc, fmt, lsp, export, version, help — no `verify`.
The `builtin_subcommands()` function (lines 924–987) similarly omits verify.

### 1.2 oxilake — The Package Manager (Uses Clap)

**File**: `crates/oxilake/Cargo.toml`

oxilake uses `clap = { version = "4.6", features = ["derive"] }` and `anyhow`.
This is the only crate in the workspace that actually uses clap for a real CLI.

oxilake subcommands (via `#[derive(Subcommand)]`, `src/main.rs` lines 25–79):
- `New { name, path }`
- `Build { manifest, release }`
- `Check { manifest, verbose }`
- `Test { manifest }`
- `Run { manifest }`
- `Fmt { manifest, check }`
- `Clean { package }`

oxilake dep list: `oxilean-build + oxilean-elab + oxilean-kernel + oxilean-parse + oxilean-std + oxicode + clap + serde + toml + anyhow`.
oxilake is also disqualified as the verify binary host for the same reason as cli.

### 1.3 oxilean-doc — Another Clap User

**File**: `crates/oxilean-doc/Cargo.toml`
Uses `clap = { version = "4.6", features = ["derive"] }` and `anyhow`.
Deps: `clap + anyhow + oxilean-parse + oxilean-codegen`.

---

## 2. Absence of oxilean-verify and oxilean-export Crates

Neither `oxilean-verify` nor `oxilean-export` exists as a crate in the workspace.
The workspace members (14 crates, `Cargo.toml` lines 3–17):

```
oxilake, oxilean, oxilean-build, oxilean-cli, oxilean-codegen,
oxilean-doc, oxilean-elab, oxilean-kernel, oxilean-lint, oxilean-meta,
oxilean-parse, oxilean-runtime, oxilean-std, oxilean-wasm
```

No reference to `lean4export`, `lean4lean`, `Lean4Export`, or `export_reader`
appears anywhere in the codebase (`grep` confirmed zero hits in all .rs/.toml/.md).

---

## 3. Design: oxilean-verify CLI (What Needs to Be Built)

The brief specifies an independent binary crate. Here is what needs to be designed:

### 3.1 Cargo.toml for oxilean-verify

```toml
[package]
name = "oxilean-verify"
version.workspace = true
edition.workspace = true
# ...

[[bin]]
name = "oxilean-verify"
path = "src/main.rs"

[dependencies]
oxilean-kernel = { workspace = true }
oxilean-export = { path = "../oxilean-export" }  # NEW crate, to be created
# NO other deps — this is the TCB boundary

[dev-dependencies]
# fuzz targets live in a separate fuzz/ crate; cargo-fuzz adds them
```

### 3.2 CLI Args Design

```
USAGE:
    oxilean-verify [OPTIONS] <FILE>...

ARGS:
    <FILE>...    lean4export files to verify (stdin if omitted)

OPTIONS:
    --json <PATH>      Write JSON report to PATH (stdout if "-")
    --jobs <N>         Parallel declaration threads [default: num_cpus]
    --strict           Treat 'unsupported' as 'rejected' (exit 1)
    -q, --quiet        Suppress per-decl streaming output
    -h, --help         Show this help
    -V, --version      Show version

EXIT CODES:
    0    All declarations verified (or unsupported in non-strict mode)
    1    One or more declarations rejected (alarm)
    2    Usage error / IO error / malformed export file
```

### 3.3 Streaming Output Format (brief demo transcript match)

Per-declaration line (to stdout, one per decl as it completes):
```
ok       Nat.add                              1.2ms
ok       Nat.mul                              0.8ms
unsup    Quot.lift                 [quotient-missing]   0.1ms
REJECTED List.rec                 [iota-failed]        3.1ms
```

Summary line (after all decls):
```
1234 verified / 3 unsupported / 0 rejected
```

Verdict codes:
- `ok`       → verified
- `unsup`    → unsupported (named missing feature in brackets)
- `REJECTED` → alarm

### 3.4 JSON Report Schema

```json
{
  "tool_version": "0.1.3",
  "lean4export_commit": null,
  "toolchain": "x86_64-unknown-linux-gnu",
  "timestamp_utc": "2026-07-12T10:00:00Z",
  "input_file": "Mathlib.lean4",
  "summary": {
    "verified": 1234,
    "unsupported": 3,
    "rejected": 0,
    "total_ms": 4521
  },
  "declarations": [
    {
      "name": "Nat.add",
      "verdict": "verified",
      "ms": 1.2
    },
    {
      "name": "Quot.lift",
      "verdict": "unsupported",
      "reason": "quotient-missing",
      "ms": 0.1
    },
    {
      "name": "List.rec",
      "verdict": "rejected",
      "reason": "iota-failed",
      "ms": 3.1
    }
  ]
}
```

Fields:
- `tool_version`: `env!("CARGO_PKG_VERSION")` from oxilean-verify
- `lean4export_commit`: nullable string (populated if a `.lean4export.lock` file
  or metadata in the export file pins the commit)
- `toolchain`: `std::env::var("TARGET")` or detected at build time
- Per-decl `verdict`: one of `"verified"` | `"unsupported"` | `"rejected"`
- Per-decl `reason`: present for unsupported/rejected, absent for verified

---

## 4. Workspace Tests Directory

### 4.1 Test Files

Location: `tests/`

Files:
```
tests/
  integration_test.rs         (356 #[test])
  cli_test.rs                 (68 #[test])
  perf_test.rs                (26 #[test])
  formal_proofs_test/
    mod.rs                    (0 — just declares submodules)
    types.rs, functions.rs
    tests_logic.rs            (14 #[test])
    tests_nat.rs              (11 #[test])
    tests_polymorphism.rs     (13 #[test])
  mathlib_compat_test/
    mod.rs, types.rs, test_infra.rs, normalize*.rs
    tests_basic.rs            (21 #[test])
    tests_categories.rs       (2 #[test])
    tests_summary.rs          (5 #[test])
  mathlib_theorems_test/
    mod.rs                    (0)
    functions.rs              (162 #[test])
    functions_2.rs            (106 #[test])
    functions_3.rs            (52 #[test])
  std_library_test/
    mod.rs                    (0)
    functions.rs              (112 #[test])
```

Workspace tests dir total: **948** `#[test]` annotations.

These are integration tests registered in root `Cargo.toml` as `[[test]]` entries
(lines 79–103):
- `integration_test` → `tests/integration_test.rs`
- `mathlib_compat_test` → `tests/mathlib_compat_test/mod.rs`
- `mathlib_theorems_test` → `tests/mathlib_theorems_test/mod.rs`
- `std_library_test` → `tests/std_library_test/mod.rs`
- `formal_proofs_test` → `tests/formal_proofs_test/mod.rs`

Note: `cli_test.rs` and `perf_test.rs` exist on disk but are NOT registered as
`[[test]]` entries in the root `Cargo.toml`. They will not be picked up by
`cargo test --workspace` from the workspace root. This is a gap.

### 4.2 What Integration Tests Actually Test

- `integration_test.rs`: Lexer, parser, elaboration, type checking; uses
  `oxilean_elab`, `oxilean_kernel`, `oxilean_parse` — full pipeline tests on
  OxiLean/Lean surface syntax (not lean4export format).
- `mathlib_theorems_test/`: Parse + elaborate checks for well-known Lean theorems
  (Nat.add_comm, etc.) using `sorry`-body stubs. Tests parse correctness, not
  kernel correctness for those theorems.
- `formal_proofs_test/`: Logic, Nat, and polymorphism proofs via parse+elaborate.
- `cli_test.rs`: CLI-level tests using `std::process::Command` to exercise the
  `oxilean` binary. NOT registered — dead.
- `perf_test.rs`: Timing-based tests; NOT registered — dead.

### 4.3 Nextest Config

No `.config/nextest.toml` or `nextest.toml` found anywhere in the repo.
`cargo nextest` would run tests with default settings.

---

## 5. Test Counts Per Crate

From `grep -c "#\[test\]"` across all src/**/*.rs files per crate:

| Crate             | #[test] count |
|-------------------|---------------|
| oxilean-kernel    | 3,470         |
| oxilean-meta      | 5,539         |
| oxilean-std       | 8,062         |
| oxilean-codegen   | 4,713         |
| oxilean-elab      | 3,556         |
| oxilean-parse     | 2,406         |
| oxilean-cli       | 2,278         |
| oxilean-runtime   | 1,162         |
| oxilean-build     | 882           |
| oxilean-lint      | 685           |
| oxilean-doc       | 97            |
| oxilake           | 63            |
| oxilean-wasm      | 59            |
| oxilean           | 0             |
| **Workspace tests/** | **948**    |
| **All crates total** | **32,972** |

**Kernel-specific count: 3,470** — close to the brief's claimed 3,444.
The small discrepancy (~26) is likely from a count including `bench_support/`
test functions that are `#[test]` attributed but in bench-support infra code.

---

## 6. Benchmark Infrastructure

### 6.1 Existing Criterion Benches

**File**: `crates/oxilean-kernel/benches/kernel_perf.rs`

Uses `criterion = { version = "0.8", features = ["html_reports"] }` (dev-dep only).

The bench file is 761 lines and defines 9 benchmark groups:
1. `bench_whnf` — beta reduction (chain depths 4/8/16/32), let chains, env delta
2. `bench_def_eq` — reflexivity, beta eq, delta eq, structural Pi, level max commutativity
3. `bench_infer` — Sort/literal/const/lambda/pi/app inference; nested lambdas
4. `bench_subst` — instantiate shallow/deep/wide, beta_step, beta_normalize
5. `bench_nat_arith` — reduce_nat_op add/mul small and large, whnf_env on add/mul
6. `bench_normalize` — lit/beta chain/let chain/normalize_whnf/normalize_env/deep_pi
7. `bench_alpha` — alpha equivalence refl, deep Pi, lambda rename, non-equiv
8. `bench_env` — init_builtin_env, sequential definition adds
9. `bench_reducer_cache` — warm/cold cache for whnf, env-aware cache

The bench file is auto-discovered by cargo (file in `benches/` dir), no explicit
`[[bench]]` entry needed.

**Critical gap**: All benchmarks operate on **hand-crafted AST nodes**, not on
a corpus of exported declarations from real Lean 4 files. There is **no**
`decls/sec` throughput measurement, no corpus loading, no `Throughput` criterion
measurement, and no comparison against lean4lean baseline.

### 6.2 What's Missing for lean4lean Baseline Comparison

The brief (section 8.2) requires:
1. A corpus of real lean4export files (e.g., Init.lean4, Std.lean4, partial Mathlib)
2. A bench that reads those files, parses declarations via `oxilean-export` reader,
   and verifies via kernel — measuring total decls/sec
3. A criterion `Throughput::Elements(n_decls)` measurement
4. A comparison table or CI-stored baseline for lean4lean comparison

None of this exists. The bench infra at the kernel level is purely synthetic
(expression-builder helpers, no file I/O, no export format).

### 6.3 bench_support Module

The `oxilean-kernel/src/bench_support/` module (12+ files) provides:
`BenchResult`, `ThroughputTracker`, `ThroughputUnit`, `BenchRegistry`, etc.
This is a rich internal benchmarking framework — it includes `ThroughputTracker`
and `ThroughputUnit` types — but they are not wired to criterion's `Throughput`
measurement and are not used by the `kernel_perf.rs` bench file.

---

## 7. CI Infrastructure

### 7.1 Active Workflows

Only one workflow is active (not in `.disabled`):
- `.github/workflows/npm-publish.yml` — WASM/npm publish

### 7.2 Disabled Workflows

- `.github/workflows.disabled/ci.yml`: Standard `cargo build + test + clippy + fmt`
  Uses `cargo test --workspace --no-fail-fast` (not nextest).
- `.github/workflows.disabled/bench.yml`: Runs `cargo test --workspace -- --test bench_ --nocapture`
  (a confused invocation that would run unit tests named `bench_*`, not criterion benches).

Both CI workflows are DISABLED — there is no active CI for the Rust code.

---

## 8. Unsafe Code Policy

**oxilean-kernel**: `#![forbid(unsafe_code)]` confirmed at line 263 of
`crates/oxilean-kernel/src/lib.rs`.

The kernel Cargo.toml shows `[dependencies]` with a comment:
`# ZERO external dependencies - this is the TCB (Trusted Computing Base)`

However, `main/functions.rs` in oxilean-cli uses `unsafe extern "C"` for signal
handling (SIGINT via libc `signal(2)` call, lines 455–510). This is in cli, not kernel,
so it does not violate the kernel's `forbid(unsafe_code)`.

---

## 9. Key Gaps for oxilean-verify Implementation

### 9.1 Missing Crates
- `crates/oxilean-verify/` — does not exist
- `crates/oxilean-export/` — does not exist (the lean4export reader)

### 9.2 Missing Infrastructure
- No lean4export text format parser anywhere in the codebase
- No fuzz target for the export reader (cargo-fuzz not set up at all)
- No `nextest.toml` config
- No active CI workflow for the Rust codebase
- No corpus-based benchmark (decls/sec throughput)
- `cli_test.rs` and `perf_test.rs` not registered in Cargo.toml

### 9.3 oxilean-cli Dep Closure (Proof That Verify Must Be Separate)

oxilean-cli depends on:
- `oxilean-kernel` (kernel)
- `oxilean-parse` (full surface parser, ~2,406 tests worth of code)
- `oxilean-elab` (elaborator, ~3,556 tests worth of code)
- `oxilean-lint` (linter, ~685 tests worth of code)
- `oxilean-std` (stdlib tactics, ~8,062 tests worth of code)

The brief's verify product dep closure must be ONLY: `oxilean-kernel + oxilean-export`.
Placing verify logic in oxilean-cli would violate this by importing the elaborator,
linter, and stdlib tactics — none of which belong in the TCB.

---

## 10. Recommendations for Implementation

1. Create `crates/oxilean-export/` — zero external deps, ≤3000 SLoC, #![forbid(unsafe_code)],
   parses the lean4export text format (version header, `#AX`, `#DEF`, `#IND`, `#REC`,
   `#QUOT` lines into kernel `Declaration` values).

2. Create `crates/oxilean-verify/` with only `oxilean-kernel` and `oxilean-export` deps.
   Use hand-written arg parsing (no clap) to keep the dep closure clean.
   Implement the 3-verdict system: verified / unsupported(reason) / rejected(reason).
   Exit codes: 0=all-ok, 1=rejected>0, 2=usage-io-malformed.
   Streaming stdout per-decl + summary line. Optional `--json` report.

3. Add `crates/oxilean-verify` and `crates/oxilean-export` to workspace members in
   root `Cargo.toml`.

4. Add a `fuzz/` directory under `oxilean-export` (cargo-fuzz convention):
   `fuzz/fuzz_targets/fuzz_export_reader.rs` using `libfuzzer_sys`.

5. Add `[[bench]]` to kernel Cargo.toml (or rely on auto-discovery which already works)
   and add a separate corpus benchmark that:
   - Reads a small bundled lean4export file from `benches/corpus/`
   - Measures decls/sec with `criterion::Throughput::Elements(n_decls as u64)`

6. Register `tests/cli_test.rs` and `tests/perf_test.rs` as `[[test]]` in root Cargo.toml,
   or move them to the appropriate crate.

7. Add `.config/nextest.toml` with `default-filter = "test()"` and a `[profile.ci]`
   section with `fail-fast = false`.

8. Re-enable or replace CI with a workflow that: builds workspace, runs nextest,
   checks formatting, runs cargo-fuzz for a short duration, and gates on WASM export-count.

---

## Appendix: File References

| File | Lines | Relevance |
|------|-------|-----------|
| `crates/oxilean-cli/Cargo.toml` | 1–30 | CLI deps (no clap, has elab+lint+std) |
| `crates/oxilean-cli/src/main.rs` | 69–71 | Entry → cli_main() |
| `crates/oxilean-cli/src/main/functions.rs` | 17–52 | Manual match dispatch, no verify |
| `crates/oxilean-cli/src/main/functions.rs` | 84–105 | print_help() — no verify |
| `crates/oxilake/Cargo.toml` | 25 | clap 4.6 derive usage |
| `crates/oxilake/src/main.rs` | 13–99 | clap Subcommand derive pattern |
| `crates/oxilean-kernel/Cargo.toml` | 20–26 | Zero deps + criterion dev-dep |
| `crates/oxilean-kernel/src/lib.rs` | 263 | `#![forbid(unsafe_code)]` |
| `crates/oxilean-kernel/benches/kernel_perf.rs` | 1–761 | Criterion benches (synthetic only) |
| `Cargo.toml` | 79–103 | [[test]] registrations (missing cli/perf) |
| `tests/integration_test.rs` | 1–∞ | 356 tests, full parse+elab pipeline |
| `tests/cli_test.rs` | 1–∞ | 68 tests, NOT registered in Cargo.toml |
| `tests/perf_test.rs` | 1–∞ | 26 tests, NOT registered in Cargo.toml |
| `.github/workflows.disabled/ci.yml` | all | Disabled CI; no nextest, no fuzz |
