# oxilean-verify

Independent Lean 4 proof checker — streaming three-bucket verdict engine and
product CLI.

## Purpose

`oxilean-verify` reads a lean4export `.ndjson` file and classifies each
declaration into exactly one of three buckets:

| Verdict | Meaning | CLI exit code |
|---|---|---|
| `verified` | Lean's proof is accepted by the oxilean-kernel | 0 (all verified) |
| `unsupported` | A named missing feature; the checker conservatively skips | 0 (with count) |
| `rejected` | The checker decided the proof is **wrong** — alarm | 1 |

`rejected` is the alarm: it means "we checked it and it is not a proof."
A non-zero exit on any `rejected` verdict makes the checker CI-friendly.

`unsupported` always carries a named reason (e.g., `"nested_inductive"`) so
that the feature gap can be tracked and closed.

## TCB story

The dependency closure is the TCB for the verification product:

| crate | role |
|---|---|
| `oxilean-kernel` | zero-external-dep proof-checking TCB |
| `oxilean-export` | lean4export NDJSON reader |

No clap, serde, tokio, or any other external crate appears in the release
build. Argument parsing and JSON output are hand-rolled in this crate.
`#![forbid(unsafe_code)]`.

## Usage

```bash
# verify a lean4export file, print streaming verdicts, exit 1 on any rejected
oxilean-verify path/to/Mathlib.Something.ndjson

# run on a large corpus with adjusted memory/time limits
oxilean-verify --limits corpus path/to/Init.ndjson

# machine-readable JSON report to a file
oxilean-verify --report report.json path/to/export.ndjson
```

## Library API

```rust
use oxilean_verify::{verify_stream, LimitsPreset, VerifySummary};
use oxilean_kernel::Environment;

let ndjson: &[u8] = /* lean4export bytes */;
let mut summary = VerifySummary::default();
verify_stream(ndjson, LimitsPreset::Default, |decl| {
    println!("{}: {}", decl.name, decl.verdict);
    summary.tally(&decl);
})?;
```

The same `verify_stream` function is used by the CLI and by
`oxilean-verify-wasm` — the browser demo and the native binary drive identical
code paths.

## Crate details

```toml
[dependencies]
oxilean-verify = "0.1.3"
```

- License: Apache-2.0
- Edition: 2021
- Rust version: 1.70+

See `docs/VERIFY.md` for the full three-bucket schema, exit codes, JSON report
format, and unsupported-feature list.

Copyright 2026 COOLJAPAN OU (Team Kitasan). Apache-2.0.
