# oxilean-wasm

[![Crates.io](https://img.shields.io/crates/v/oxilean-wasm.svg)](https://crates.io/crates/oxilean-wasm)
[![Docs.rs](https://docs.rs/oxilean-wasm/badge.svg)](https://docs.rs/oxilean-wasm)
[![npm](https://img.shields.io/npm/v/@cooljapan/oxilean)](https://www.npmjs.com/package/@cooljapan/oxilean)

> **WebAssembly Bindings for the OxiLean Theorem Prover**

`oxilean-wasm` exposes the OxiLean type checker and proof engine as a WebAssembly module that can be loaded directly in web browsers or Node.js environments. It provides a JavaScript/TypeScript-friendly API so that web applications, online proof assistants, and editor extensions can drive OxiLean without any server-side infrastructure.

The npm package is published as [`@cooljapan/oxilean`](https://www.npmjs.com/package/@cooljapan/oxilean).

47 tests passing -- full integration with the kernel/parse/elab pipeline.

Part of the [OxiLean](https://github.com/cool-japan/oxilean) project -- a Lean-compatible theorem prover implemented in pure Rust.

## Install

```bash
npm install @cooljapan/oxilean
```

## Modules

| Module | Description |
|--------|-------------|
| `api` | Core OxiLean API (platform-independent logic) |
| `types` | Value types, result structs, and conversion helpers |
| `error` | WASM-compatible error type |
| `wasm_api` | `#[wasm_bindgen]` entry points for JavaScript *(requires `wasm` feature)* |

## Feature Flags

| Feature | Enables |
|---------|---------|
| `wasm` | `wasm-bindgen`, `serde`, `serde_json`, `serde-wasm-bindgen`, `js-sys` |

## API

The `wasm_api` module exports the `WasmOxiLean` class and convenience functions:

```typescript
import { WasmOxiLean, checkSource, getVersion } from '@cooljapan/oxilean';

const ox = new WasmOxiLean();

// Check source code
const result = ox.check('theorem foo : True := trivial');
// result: { success: boolean, declarations: DeclInfo[], errors: ErrorInfo[], warnings: WarningInfo[] }

// REPL
const repl = ox.repl('#check Nat');
// repl: { output: string, goals: GoalInfo[], success: boolean, error?: string }

// Completions at a position
const completions = ox.completions(source, line, col);
// completions: CompletionItem[]

// Hover info at a position
const hover = ox.hoverInfo(source, line, col); // string | null

// Format source code
const formatted = ox.format(source); // string

// Properties
ox.sessionId;       // getter -- unique session identifier
ox.history();       // string[] -- REPL command history
ox.clearHistory();  // clear REPL history
WasmOxiLean.version(); // static -- OxiLean version string

// Convenience functions (no instance needed)
checkSource('theorem foo : True := trivial');
getVersion();

// Clean up WASM memory when done
ox.free();
```

## Building

The build script produces three targets: bundler (webpack/vite), web (browser direct), and nodejs.

```bash
# From the crate directory
bash build-wasm.sh
```

This creates:

- `pkg/` -- bundler target (primary)
- `pkg-web/` -- web target
- `pkg-nodejs/` -- nodejs target

## Dependencies

All dependencies are optional and gated behind the `wasm` feature:

- `wasm-bindgen` 0.2.114
- `serde` + `serde_json`
- `serde-wasm-bindgen` 0.6.5
- `js-sys` 0.3.83

## Testing

```bash
# Unit tests (native target)
cargo test -p oxilean-wasm

# WASM tests in headless browser (requires wasm-pack)
wasm-pack test crates/oxilean-wasm --headless --chrome
```

## License

Copyright COOLJAPAN OU (Team Kitasan). Apache-2.0 -- See [LICENSE](../../LICENSE) for details.
