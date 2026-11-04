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

# machine-readable JSON report to a file (`--json -` writes it to stdout;
# add --json-full for a per-declaration list)
oxilean-verify --json report.json path/to/export.ndjson
```

## Library API

```rust
use std::fs::File;
use std::io::BufReader;

use oxilean_verify::cli::LimitsPreset;
use oxilean_verify::{verify_stream, Verdict, VerifyOptions, VERSION};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let reader = BufReader::new(File::open("export.ndjson")?);
    let options = VerifyOptions {
        // The CLI's default (untrusted-input) per-declaration budget.
        per_decl_fuel: LimitsPreset::Default.decl_fuel(),
        ..VerifyOptions::default()
    };
    let report = verify_stream(reader, VERSION, options, |event| match &event.verdict {
        Verdict::Verified { .. } => {}
        Verdict::Unsupported { feature } => println!("{}: unsupported ({feature})", event.name),
        Verdict::Rejected { reason } => println!("{}: REJECTED: {reason}", event.name),
    })?;
    let s = &report.summary;
    println!(
        "{} verified · {} unsupported · {} rejected",
        s.verified, s.unsupported, s.rejected
    );
    Ok(())
}
```

A per-declaration rejection is not an `Err`. It arrives as a
`Verdict::Rejected` event and is counted in `report.summary`. `Err` means the
file itself could not be read: it is broken, uses an unsupported format, or
hit an I/O error.

The same `verify_stream` function is used by the CLI and by
`oxilean-verify-wasm` — the browser demo and the native binary drive identical
code paths.

## Crate details

```toml
[dependencies]
oxilean-verify = "0.1.4"
```

- License: Apache-2.0
- Edition: 2021
- Rust version: 1.70+

See `docs/VERIFY.md` for the full three-bucket schema, exit codes, JSON report
format, and unsupported-feature list.

Copyright 2026 COOLJAPAN OU (Team Kitasan). Apache-2.0.
