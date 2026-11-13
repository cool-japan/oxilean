# `oxilean-verify` — an independent Lean 4 proof checker

`oxilean-verify` reads [`lean4export`](https://github.com/leanprover/lean4export)
NDJSON files and checks every declaration in them with the `oxilean-kernel` type
checker. It is the **product** of the OxiLean verify campaign: a second kernel,
written by someone else, in a different language, that agrees.

Its entire value is that its trusted computing base is small and auditable. The
dependency closure a reviewer must read is exactly two crates:

* [`oxilean-kernel`](../crates/oxilean-kernel) — the CiC type checker (the TCB;
  zero external runtime dependencies, `#![forbid(unsafe_code)]`).
* [`oxilean-export`](../crates/oxilean-export) — the lean4export NDJSON reader
  (depends only on the kernel; also zero external deps, also `forbid(unsafe)`).

`oxilean-verify` itself adds **no** further external dependencies: argument
parsing, JSON output, and even the input SHA-256 fingerprint are hand-rolled in
this crate, and the crate is `#![forbid(unsafe_code)]`. There is no `clap` and no
`serde`.

---

## The three buckets, always

Every declaration lands in **exactly one** of three buckets. They are wired
separately everywhere — CLI, JSON report, and library API — and are never summed
into a single figure. This is how every serious checker in this field reports,
and it is what makes the numbers usable by someone else.

| Bucket          | Meaning (verbatim-faithful to the engineering brief §7)               |
|-----------------|-----------------------------------------------------------------------|
| **verified**    | We checked it. It is a proof.                                          |
| **unsupported** | We do not implement a feature it needs. We say which one, by name.    |
| **rejected**    | We checked it and we say it is **not** a proof.                        |

### `rejected` is an alarm

> If `rejected` is ever non-zero, either we have a kernel bug or we have found
> something in Lean, and we need to know *instantly* which conversation we are
> in. If `unsupported` and `rejected` are ever summed into one figure, that alarm
> is silently disabled.

So the buckets are wired separately from day one, and **the CLI exits non-zero on
any rejection** (see exit codes below). A rejection is never merged with an
unsupported feature, and — crucially — a *broken input file* is never reported as
a rejection either: "this file is broken" and "this proof is wrong" are different
conversations and travel on different channels.

The `unsupported` list is published, with reasons, in the JSON report's
`unsupported_features` array. Every checker has one; publishing it is what invites
the community to close it *with* you.

---

## Exit codes (product-critical)

| Code | Meaning                                                                        |
|------|-------------------------------------------------------------------------------|
| `0`  | The run completed and **zero** declarations were rejected.                    |
| `1`  | The run completed and **at least one** declaration was **rejected** (alarm).  |
| `2`  | A usage error, an I/O error, or **malformed export input**.                   |

Exit `2` covers a *broken file* — invalid JSON, a spec violation, a bad index
reference, a materialization-budget blowout, a missing file, or a bad flag. **A
malformed export file is not a rejection.** It is reported on stderr with a
message that says the file is broken, and it never trips the exit-`1` alarm.

Under `--fail-fast`, the run stops at the first rejected declaration and exits
`1`.

---

## Command-line usage

```
oxilean-verify [OPTIONS] <FILE.ndjson>...
```

| Option            | Effect                                                                        |
|-------------------|-------------------------------------------------------------------------------|
| `--json <PATH>`   | Write a machine-readable JSON report to `PATH` (`-` writes it to stdout).      |
| `--json-full`     | Include a full per-declaration `decls` array in the JSON report.              |
| `--quiet`, `-q`   | Suppress the per-declaration stream; print only the summary line.            |
| `--no-color`      | Disable ANSI colour. The `NO_COLOR` environment variable does the same.      |
| `--limits <P>`    | Materialization budget preset: `default` (small, for untrusted input) or `corpus` (large, for trusted whole-corpus exports such as Lean core / Mathlib). The preset also sets the **per-declaration resource budget** (see below). |
| `--stack-size <MiB>` | Stack size of the verification thread, in MiB (default **512**).           |
| `--fail-fast`     | Stop at the first rejected declaration.                                      |
| `--version`, `-V` | Print the version and the pinned `lean4export` commit / Lean toolchain.       |
| `--help`, `-h`    | Print help.                                                                   |

When `--json -` is used, stdout carries **only** the JSON report; the
human-readable stream and summary are routed to stderr so machine output stays
pure.

### Deep reduction chains: the verification thread

Verification always runs on a dedicated thread created with
`std::thread::Builder::stack_size` (default 512 MiB, `--stack-size` to change).
Deep Lean-core reduction chains legitimately recurse far beyond the 8 MiB OS
default; reserving a large stack up front (the memory is *reserved*, not
committed — untouched pages cost nothing) means no `ulimit -s` incantation is
ever needed.

### One declaration can never OOM the process

Each `--limits` preset carries a deterministic **per-declaration resource
budget**, measured in units of kernel work: 2^24 for `default`, 2^26 for
`corpus`. A unit is one subterm that one of the kernel's term builders —
substitution, lifting, abstraction, universe instantiation, a clone — reaches,
whether it rebuilds the subterm or shares it; every node the kernel allocates
is charged one. On a 2023 laptop core a unit is a few tens of nanoseconds, so
the two budgets are about half a second and two seconds of work, and they bound
the nodes one declaration can have live at 16.8 and 67 million (72 bytes each).
The heaviest declaration of Lean core `Init` takes 6.7 million units.

A declaration that exhausts its budget is reported as **unsupported** with the
named feature `resource limit exceeded: per-declaration clone-fuel budget` —
never as rejected (the kernel degrades conservatively once the budget runs out:
reduction goes stuck, definitional equality decides only syntactic equality, so
no wrong verdict is possible in either direction). The cutoff is deterministic:
the same declaration with the same budget always lands in the same bucket. (The
feature string keeps the name the unit had through 0.1.3, when it counted cloned
nodes of terms held as trees.)

The `corpus` preset also arms a wall-clock deadline of 30 seconds per
declaration, for work that charges nothing (a universe-level comparison can
take exponential time without building a term). A declaration still running
then is reported under the same named feature. Unlike the budget this limit
depends on the machine; no declaration of Lean core `Init` or of the Mathlib
slice the README reports comes within a factor of a hundred of it.

The reducer has two limits of its own: a cap on the *nesting depth* of
reduction (a stack guard) and a cap on the *length* of one head-reduction chain
(10,000 steps). The second is what lets definitional equality give up on a
reduction that is going nowhere and try another route. A reduction cut short by
either cap is not a normal form, so a failure that follows it is not a verdict
on the proof.

When the chain cap fired and the declaration then failed, the checker repeats
the check under a fresh budget with that cap lifted. The budget bounds the chain
instead — every head-reduction step charges it — so a long chain of cheap steps
(a `decide` proof iterating `Nat.rec` tens of thousands of times) runs to its
normal form, and the declaration is verified, rejected, or reported as over
budget on its merits. A declaration that fails after the nesting-depth guard
fired is reported as **unsupported** with the named feature
`resource limit exceeded: reduction depth or step limit` — again never as
rejected, and deterministically. (A library user who arms no budget at all gets
that verdict for the chain cap too: with no budget nothing else would stop a
runaway chain, so the check is not repeated without the cap.)

### A small stack: two limits the command-line tool does not set

The kernel is recursive. A reduction nested inside another — the major premise
of a recursor, the structure of a projection, an argument of a literal
operation — holds a frame of the native stack until it is done, and so does
every level of a term while the term is walked: to substitute into it, to infer
its type, to compare it with another, to release it. `oxilean-verify` runs on
the thread described above and limits neither; how deep it goes is a matter of
the stack it was given.

A caller that cannot give the kernel such a stack — a WebAssembly module runs
on its host's, about a megabyte in a browser, which it can neither enlarge nor
measure — sets two limits on its thread before it reads a file. Both are off by
default.

| Limit | Set with | What is over it | Reported as `unsupported` with |
|---|---|---|---|
| nesting of reductions | `oxilean_kernel::fuel::set_reduction_nesting_limit` | a reduction that would nest deeper is not started; its term comes back unreduced | `resource limit exceeded: reduction depth or step limit` |
| depth of terms | `oxilean_kernel::fuel::set_term_depth_limit` | a declaration with a deeper term is not read into memory; a term that grows deeper while a declaration is checked spends that declaration's budget | `resource limit exceeded: term depth` |

Neither is ever a verdict: a term returned unreduced is still the same term, so
nothing can be verified that should not be, and a failure that follows a limit
is reported as that limit, never as `rejected`. Both are deterministic — they
depend on the terms, not on how much stack happened to be free. The two
fixtures `NestedReduction` and `DeepTerm` each carry a false theorem that Lean's
kernel rejects; the tests replay them under every limit from zero up and
require that it is never verified and that no sound declaration is ever
rejected.

The depth limit has two halves. The reader knows the depth of every expression
of a file from its tables and refuses a declaration with one too deep, in any
build. A term that is BUILT too deep — an argument a recursion adds to at every
step is hundreds of levels deep after hundreds of steps — is only noticed by a
kernel that tracks the depth of its terms: the `term-depth` feature of
`oxilean-kernel` (forwarded by `oxilean-export` and `oxilean-verify`). It costs
four bytes per shared subterm, which is why it is not on by default.

The WebAssembly build (`oxilean-verify-wasm`) has the feature on and sets both
limits: 256 nested reductions, terms 512 deep. Measured under node 26 (V8, 984 KB
of stack by default, as in Chrome), a level of nested reduction takes 0.80 KB of
the engine's stack and a level of a term being compared 0.86 KB; the deepest
combinations the limits allow ran in 312 KB, and Lean core `Init` and the
Mathlib slice replay with half the engine's stack and half the module's own.
Under these limits both corpora get, declaration for declaration, the verdicts
of the native tool (`Init` keeps its verdicts down to a limit of 64 nested
reductions, and needs terms between 256 and 512 deep). What the limits cost is
evaluation by structural
recursion more than about 250 steps deep, and terms more than 512 levels deep —
a list literal of more than about 500 elements: `unsupported` in a browser,
checked by the command-line tool.

### Example transcript

Mirroring the *Kernel in a Tab* demo (engineering brief §6):

```
$ oxilean-verify Mathlib.Analysis.SpecialFunctions.Log.Basic.ndjson

✓  Real.log_le_sub_one_of_pos                    1.2 ms
✓  Real.add_pow_le_pow_mul_pow_of_sq_le_sq       4.8 ms
✓  Real.exp_log                                  0.9 ms
⊘  Real.exp_approx        unsupported: Nat literal reduction
✓  Real.log_nonneg                               0.7 ms

1204 verified · 3 unsupported · 0 rejected
```

* `✓` (U+2713) — verified, with the per-declaration check time right-aligned.
* `⊘` (U+2298) — unsupported, with the **named** missing feature.
* `✗` (U+2717) — rejected, with the kernel's rejection reason.

The summary line separates the three counts with a middle dot (U+00B7) and never
merges them.

---

## JSON report schema

The JSON writer is hand-rolled and **deterministic**: the same run produces
byte-identical output (modulo the genuinely non-deterministic wall-time fields).
Field order is stable and is exactly as listed here. Durations are emitted as
integer milliseconds/microseconds — no floating point — so reports are exact and
platform-independent.

```json
{
  "tool": {
    "name": "oxilean-verify",
    "version": "0.1.2"
  },
  "pins": {
    "lean4export_commit": "3de59f10bc4b4a0f2de698597aeb1246caa0df0a",
    "lean_toolchain": "v4.32.0-rc1",
    "reader_format_version": "3.1.0",
    "file_format_version": "3.1.0",
    "file_lean_version": "4.32.0-rc1",
    "file_lean_githash": "b4812ae53eea93439ad5dce5a5c26591c31cb697"
  },
  "input": {
    "path": "simple_add.ndjson",
    "sha256": "22744c60...0277d6be",
    "size_bytes": 36629,
    "format_version": "3.1.0"
  },
  "totals": {
    "verified": 1,
    "unsupported": 22,
    "rejected": 0,
    "wall_ms": 5
  },
  "unsupported_features": [
    {
      "feature": "inductive replay lands with Wave-2 kernel API (re-derived recursors)",
      "count": 7,
      "decls_capped": false,
      "decls": ["Eq", "Nat", "HAdd", "Add", "PUnit", "PProd", "OfNat"]
    }
  ],
  "rejected": [],
  "decls": [ /* present only with --json-full */ ]
}
```

### Field reference

| Path                              | Type          | Meaning |
|-----------------------------------|---------------|---------|
| `tool.name` / `tool.version`      | string        | This binary's name and `CARGO_PKG_VERSION`. |
| `pins.lean4export_commit`         | string        | The `lean4export` git commit the reader targets. |
| `pins.lean_toolchain`             | string        | The Lean toolchain the reader targets. |
| `pins.reader_format_version`      | string        | The NDJSON format version the reader targets. |
| `pins.file_format_version`        | string \| null| The `format` version read from *this file's* `meta` record. |
| `pins.file_lean_version`          | string \| null| The Lean version *this file* was produced with. |
| `pins.file_lean_githash`          | string \| null| The Lean git hash *this file* was produced with. |
| `input.path`                      | string        | The input path as given (a summary string for multiple files). |
| `input.sha256`                    | string \| null| Lowercase hex SHA-256 of the file bytes (single-file runs). |
| `input.size_bytes`                | integer \| null | File size in bytes (single-file runs). |
| `input.format_version`            | string \| null| Same as `pins.file_format_version`, surfaced next to the file. |
| `totals.verified` / `.unsupported` / `.rejected` | integer | The three-bucket counts, kept separate. |
| `totals.wall_ms`                  | integer       | Total wall time for the run, in whole milliseconds. |
| `unsupported_features[].feature`  | string        | The named missing capability. |
| `unsupported_features[].count`    | integer       | **Exact** number of declarations deferred for this feature. |
| `unsupported_features[].decls_capped` | boolean   | `true` if the `decls` sample was truncated. |
| `unsupported_features[].decls`    | string[]      | Up to 32 declaration names (a sample; `count` is always exact). |
| `rejected[].name` / `.reason`     | string        | Each rejected declaration and the kernel's reason. |
| `decls[]`                         | array         | Present only with `--json-full`: `{name, kind, verdict, micros, detail}` per declaration. |

The `null` values in `pins`/`input` appear when the file could not be read to its
header (for example, when `--fail-fast` stops the run early); the build-time pins
are always present.

---

## Provenance pins

Every report records both classes of provenance (engineering brief §2, §7 —
*"record both in every report"*):

* **Build-time pins** — the tool version and the `lean4export` commit and Lean
  toolchain that *this checker's reader* is built against. Constant per binary.
* **Per-file pins** — the NDJSON `format` version and the exact Lean
  githash/version that *this particular input file* was produced with, read from
  its `meta` record.

Current pins:

* `lean4export` commit: `3de59f10bc4b4a0f2de698597aeb1246caa0df0a`
* Lean toolchain: `v4.32.0-rc1` (`leanprover/lean4:v4.32.0-rc1`)
* NDJSON format version: `3.1.0` (major version `3` required; `3.0.0` exports are
  also accepted — see the reader's discrepancy notes)

---

## Library API

The verify engine is UI-free and reusable (the WASM front-end drives it the same
way the CLI does):

```rust
use oxilean_verify::{verify_stream, Verdict, VerifyOptions};
use std::io::BufReader;

let file = std::fs::File::open("simple_add.ndjson")?;
let report = verify_stream(
    BufReader::new(file),
    oxilean_verify::VERSION,
    VerifyOptions::default(),
    |event| {
        // event: { name, kind, verdict, index }
        match &event.verdict {
            Verdict::Verified { micros }       => { /* ✓ */ }
            Verdict::Unsupported { feature }   => { /* ⊘ */ }
            Verdict::Rejected { reason }       => { /* ✗ — the alarm */ }
        }
    },
)?;
// report.summary: { verified, unsupported, rejected, total, wall_micros }
// report.pins:    environment pins (build-time + per-file)
```

`verify_stream` returns `Err(VerifyError)` **only** when the file cannot be read
to completion (malformed input, a reader-unsupported construct, or I/O). A
per-declaration rejection is *not* an error — it is a `Verdict::Rejected` event
reflected in the returned `Summary`.

---

*Copyright 2026 COOLJAPAN OU (Team KitaSan). Apache-2.0.*
