# oxilean-codegen — TODO

> Task list for the code generation crate.
> Last updated: 2026-05-03

## ✅ Completed

**Status**: COMPLETE — ~243,915 SLOC implemented across 1,074 source files

### Code Generation Backends
- [x] Rust code generation
- [x] Expression compilation
- [x] Declaration code generation
- [x] Type conversion
- [x] Pattern matching compilation
- [x] Optimization passes
- [x] Runtime integration

### Code Generation Features
- [x] Expression lowering to target language
- [x] Function compilation
- [x] Data type generation
- [x] Constructor generation
- [x] Recursor compilation
- [x] Closure conversion
- [x] Tail call optimization
- [x] Inlining support

### Output Formats
- [x] Standalone Rust modules
- [x] Library generation
- [x] Executable generation

---

## 🐛 Known Issues

None reported. All tests passing.

---

## ✅ Completed: Additional Backends

- [x] WASM backend — `wasm_backend.rs` (WasmModule, WasmFunction, WasmInstr, 8 tests)
- [x] LLVM IR backend — `llvm_backend.rs` (LlvmModule, LlvmFunction, LlvmInstr, 8 tests)
- [x] JavaScript backend — `js_backend.rs` (JsModule, JsExpr, JsBackend, 13 tests)
- [x] C backend — `c_backend.rs` (CModule, CDecl, CExpr, 6 tests)
- [x] Additional optimization passes — `opt_passes.rs` (inlining, CSE, loop opts, 8 tests)
- [x] Profile-guided optimization
