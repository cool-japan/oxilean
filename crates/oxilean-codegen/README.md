# oxilean-codegen

[![Crates.io](https://img.shields.io/crates/v/oxilean-codegen.svg)](https://crates.io/crates/oxilean-codegen)
[![Docs.rs](https://docs.rs/oxilean-codegen/badge.svg)](https://docs.rs/oxilean-codegen)

> **Code Generation Backend for the OxiLean Theorem Prover**

`oxilean-codegen` compiles type-checked OxiLean kernel expressions into executable code for multiple target platforms. The pipeline first lowers expressions to a Lambda-Case Normal Form (LCNF) intermediate representation, applies a series of optimisation passes, and then emits target-specific code.

The crate is **untrusted** with respect to logical soundness -- any bug here can only affect the performance or correctness of extracted programs, not the validity of proofs, because all terms are independently verified by `oxilean-kernel` before reaching this stage.

243,915 SLOC -- comprehensive code generation with multiple backends and optimisation passes (1,074 source files, 4,570 tests passing).

Part of the [OxiLean](https://github.com/cool-japan/oxilean) project -- a Lean-compatible theorem prover implemented in pure Rust.

## Overview

### Compilation Pipeline

```text
Kernel Expr (type-checked)
       |
       v
+-------------+
|  to_lcnf    |  -> LCNF (Lambda-Case Normal Form) IR
+------+------+
       v
+---------------------------------+
|  Optimisation Passes            |
|  opt_passes / opt_dce / opt_join|  -> DCE, join-point analysis, inlining
|  opt_reuse / opt_specialize     |  -> RC reuse, specialisation, PGO
+------+--------------------------+
       v
+---------------------------------+
|  closure_convert                |  -> Flat closure representation
+------+--------------------------+
       v
+------------------------------------------------------+
|  Backend selection                                    |
|  +- native_backend   -> native Rust output            |
|  +- c_backend        -> portable C code               |
|  +- llvm_backend     -> LLVM IR (for LTO, advanced opt)|
|  +- wasm_backend     -> WebAssembly (WAT/wasm-opt)    |
|  +- js_backend       -> JavaScript / TypeScript       |
|  +- glsl_backend     -> GLSL shader output            |
|  +- wgsl_backend     -> WebGPU WGSL shaders           |
|  +- zig_backend      -> Zig (for FFI bridging)        |
|  +- ffi_bridge       -> C FFI header generation       |
+------------------------------------------------------+
```

### Module Reference

| Module | Description |
|--------|-------------|
| `lcnf` | LCNF intermediate representation types |
| `to_lcnf` | Kernel `Expr` -> LCNF lowering |
| `opt_passes` | Orchestration of all optimisation passes |
| `opt_dce` | Dead code elimination |
| `opt_join` | Join-point / CPS transformation |
| `opt_reuse` | Reference-count reuse optimisation |
| `opt_specialize` | Monomorphisation and function specialisation |
| `closure_convert` | Closure conversion to flat representation |
| `pipeline` | End-to-end compilation pipeline driver |
| `runtime_codegen` | Runtime support code generation |
| `pgo` | Profile-guided optimisation data collection |
| `native_backend` | Native Rust code emission |
| `c_backend` | C code emission |
| `llvm_backend` | LLVM IR emission |
| `wasm_backend` | WebAssembly code emission |
| `js_backend` | JavaScript code emission |
| `glsl_backend` | GLSL shader emission |
| `wgsl_backend` | WebGPU WGSL emission |
| `zig_backend` | Zig code emission |
| `ffi_bridge` | C FFI header generation |

### Configuration

```rust
use oxilean_codegen::{CodegenConfig, CodegenTarget};

let config = CodegenConfig {
    target: CodegenTarget::Rust,
    optimize: true,
    debug_info: false,
    emit_comments: true,
    inline_threshold: 50,
};
```

### Supported Targets

| Variant | Description |
|---------|-------------|
| `CodegenTarget::Rust` | Native Rust source (default) |
| `CodegenTarget::C` | Portable C99 source |
| `CodegenTarget::LlvmIr` | LLVM intermediate representation |
| `CodegenTarget::Interpreter` | Direct interpretation mode |

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
oxilean-codegen = "0.1.1"
```

## Dependencies

- `oxilean-kernel` -- kernel expression types (`Expr`, `Literal`)

## Testing

```bash
cargo test -p oxilean-codegen
cargo test -p oxilean-codegen -- --nocapture
```

## License

Copyright COOLJAPAN OU (Team Kitasan). Apache-2.0 -- See [LICENSE](../../LICENSE) for details.
