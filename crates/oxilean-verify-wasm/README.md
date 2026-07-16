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

Copyright 2026 COOLJAPAN OU (Team Kitasan). Apache-2.0.
