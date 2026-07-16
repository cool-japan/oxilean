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
use oxilean_export::{Reader, ReaderLimits, replay};
use oxilean_kernel::Environment;

let ndjson: &[u8] = /* lean4export output bytes */;
let mut env = Environment::new();
let limits = ReaderLimits::corpus(); // generous limits for full-corpus runs
let summary = replay(ndjson, &mut env, limits)?;
println!("{} decls replayed", summary.total);
```

The `replay` function drives a streaming NDJSON parser — it does NOT buffer the
entire file in memory. Index-node objects are budgeted; pathological inputs that
exceed the budget return a typed error rather than panicking or OOMing.

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
oxilean-export = "0.1.4"
```

- License: Apache-2.0
- Edition: 2021
- Rust version: 1.70+
- No build scripts; no proc macros; no C dependencies

Copyright 2026 COOLJAPAN OU (Team Kitasan). Apache-2.0.
