# oxilean-verify-wasm — *Kernel in a Tab*

WebAssembly bindings for [`oxilean-verify`](../oxilean-verify), the independent
Pure-Rust Lean 4 proof checker. This crate exists **only** to run the checker in
a browser tab: a static page reads a dropped `lean4export` `.ndjson`/`.export`
file in slices, streams each slice in, and gets one verdict per declaration back
— **verified**, **unsupported**, or **rejected** (the three buckets of the
engineering brief §7, kept strictly separate). Nothing is uploaded; the bytes
never leave the page's WebAssembly memory.

## The dependency closure is the product

The entire runtime dependency closure a reviewer must trust is:

| crate | role |
|---|---|
| `oxilean-kernel` | the trusted computing base — zero external deps, no `unsafe` |
| `oxilean-export` | the `lean4export` NDJSON reader + replay |
| `oxilean-verify` | the reusable streaming verify engine |
| `wasm-bindgen`, `js-sys` | JS glue codegen + the per-decl callback bridge |

`wasm-bindgen` is **not** feature-gated: this crate has no non-wasm build, so
dead-code elimination cannot strip the exports (the 0.1.2 "DCE ate the crate"
regression class is structurally impossible for this artifact).

`#![forbid(unsafe_code)]`.

## Two limits the native tool does not have

The kernel is recursive: a reduction nested inside another, and every level of
a term it walks, holds a frame of the native stack. The native tool runs on a
thread whose stack it chooses (512 MiB). A browser gives a module about a
megabyte, which can be neither enlarged nor measured, and running out of it is
an exception thrown through the module, not a verdict. This build therefore
sets two limits on the kernel:

| limit | value | over it, a declaration is `unsupported` with |
|---|---|---|
| reductions nested inside one another | 256 (`reduction_nesting_limit()`) | `resource limit exceeded: reduction depth or step limit` |
| depth of a term | 512 (`term_depth_limit()`) | `resource limit exceeded: term depth` |

Both are deterministic and neither is ever a rejection. Lean core `Init`
(57,277 declarations) and the Mathlib slice of the top-level README replay
under them with the verdicts of the native tool, declaration for declaration.
What they cost: a proof that evaluates a structural recursion more than about
250 steps deep, or mentions a term more than 512 levels deep (a list literal
of more than about 500 elements), is `unsupported` here and checked by the
native tool. Sizing, measured under node 26 (V8, 984 KB of stack, as in
Chrome): 0.80 KB per nested reduction, 0.86 KB per level of a term being
compared; the deepest combinations the limits allow ran in 312 KB. See
`docs/VERIFY.md`.

## Artifact

The 0.1.4 build (wasm-pack 0.15.0, wasm-bindgen 0.2.130, wasm-opt 126) is
168,785 B gzip -9 (about 165 KB; 427,914 B raw) with 41 exports. This crate
enables the kernel's `term-depth` feature for `wasm32` only (a native build of
the workspace does not pick it up through this crate). 387 SLOC in one source
file; 4 tests. Rust version: 1.89+.

## Build

```bash
wasm-pack build crates/oxilean-verify-wasm --release --target web \
  --out-dir pkg --out-name oxilean_verify
```

The npm package name is `@cooljapan/oxilean-verify` (see `Cargo.toml`
`[package.metadata.wasm-pack]`). Nothing is published from CI.

## JS API

```js
import init, { VerifySession, LimitsPreset } from "./pkg/oxilean_verify.js";
await init();

const session = new VerifySession(LimitsPreset.Default);
session.push_chunk(textSliceOfTheFile);   // once per File slice, in order
// ...
const summary = session.finish((decl) => {
  // decl.name, decl.kind, decl.verdict ("verified"|"unsupported"|"rejected"),
  // decl.detail (feature or reason), decl.ms
}, "Mathlib.Something.export");

// summary.verified · summary.unsupported · summary.rejected · summary.total
// summary.pins_json — the deterministic JSON report, built in the page
```

`reduction_nesting_limit()` and `term_depth_limit()` return the two limits of
the section above, for a page that wants to show them.

Copyright 2026 COOLJAPAN OU (Team Kitasan). Apache-2.0.
