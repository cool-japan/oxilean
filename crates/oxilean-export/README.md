# oxilean-export

lean4export NDJSON v3.1.0 reader and replay engine, feeding the oxilean-verify
trusted computing base (TCB).

## Purpose

`oxilean-export` parses the NDJSON output produced by
[lean4export](https://github.com/leanprover/lean4export) and replays each
declaration into the `oxilean-kernel` checker. It is the intake layer of the
independent Lean 4 proof checker ("Kernel in a Tab").

## TCB story

This crate is inside the TCB. Its dependency closure is intentionally minimal:

| crate | role |
|---|---|
| `oxilean-kernel` | the only runtime dependency |

Zero external crates from crates.io appear in the release build.
`#![forbid(unsafe_code)]`.

Every object type in the export format (`#DEF`, `#AXIOM`, `#IND`, `#QUOT`, etc.)
is represented as a typed Rust struct; no `unsafe` pointer casts, no
transmutes, no `from_utf8_unchecked`.

## Usage

```rust
use std::fs::File;
use std::io::BufReader;

use oxilean_export::{replay_streaming, ReplayLimits};

let reader = BufReader::new(File::open("export.ndjson")?);
let run = replay_streaming(reader, ReplayLimits::corpus())?; // generous limits for full-corpus runs
println!("{} decls replayed", run.stats.decls_replayed);
for entry in &run.report.entries {
    // entry.name, entry.kind, entry.outcome (ReplayOutcome)
}
```

`replay_streaming` drives a streaming NDJSON parser — it does NOT buffer the
entire file in memory. Index-node objects are budgeted; pathological inputs that
exceed the budget return a typed error rather than panicking or OOMing.

## Cargo features

| Feature | Default | Effect |
|---|---|---|
| `term-depth` | off | Forwards to `oxilean-kernel/term-depth`: the kernel tracks term depth so `oxilean_kernel::fuel::set_term_depth_limit` can be enforced. A term over the limit is reported as the named unsupported feature `TERM_DEPTH_LIMIT` (`"resource limit exceeded: term depth"`), never a rejection. |

0.1.4 also replays nested inductives without parameters (via the kernel's
nested-to-mutual elaboration; a nested inductive with parameters, or one nested
through a mutual container, is the named unsupported feature
`NESTED_INDUCTIVES`, and three shapes without parameters that Lean's kernel
accepts are rejected — C25, C28 and C29 in the workspace's `TODO_VERIFY.md`) and counts the per-declaration budget in subterms reached by a
term builder; new fixtures live under the workspace's `tests/fixtures/lean4export/`.

## Format pin

| Property | Value |
|---|---|
| lean4export format | NDJSON v3.1.0 |
| Lean toolchain | v4.32.0-rc1 |
| Supported object types | `#NS`, `#NI`, `#US`, `#UI`, `#UM`, `#UIM`, `#UP`, `#EV`, `#ES`, `#EC`, `#EA`, `#EL`, `#EP`, `#EI`, `#ELL`, `#ELT`, `#EP`, `#DEF`, `#OPAQUE`, `#AXIOM`, `#QUOT`, `#IND`, `#CTOR`, `#REC`, `#THM` |

See `docs/VERIFY.md` for the full format specification and unsupported-feature
list.

## Crate details

```toml
[dependencies]
oxilean-export = "0.1.6"
```

- Size: 2,530 SLOC in 7 source files under `src/`; 99 tests
- License: Apache-2.0
- Edition: 2021
- Rust version: 1.89+
- No build scripts; no proc macros; no C dependencies

Copyright 2026 COOLJAPAN OU (Team KitaSan). Apache-2.0.
