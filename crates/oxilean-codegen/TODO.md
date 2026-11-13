# oxilean-codegen — TODO

> Task list for the code generation crate.
> Last updated: 2026-11-13

## ✅ Completed

**Status**: COMPLETE — ~248,329 SLOC implemented across 1,226 source files

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

None reported.

---

## ✅ Completed: Additional Backends

- [x] WASM backend — `wasm_backend.rs` (WasmModule, WasmFunction, WasmInstr, 8 tests)
- [x] LLVM IR backend — `llvm_backend.rs` (LlvmModule, LlvmFunction, LlvmInstr, 8 tests)
- [x] JavaScript backend — `js_backend.rs` (JsModule, JsExpr, JsBackend, 13 tests)
- [x] C backend — `c_backend.rs` (CModule, CDecl, CExpr, 6 tests)
- [x] Additional optimization passes — `opt_passes.rs` (inlining, CSE, loop opts, 8 tests)
- [x] Profile-guided optimization

## v0.1.3 Doc-Gen IR Hooks

> Last updated: 2026-05-29

### Ring 1

- [x] Add `DocIR` (documentation intermediate representation) export from codegen (2026-05-30)
  - **Goal:** Allow `oxilean-doc` to request a structured doc IR from the codegen pipeline rather than re-parsing the source file
  - **Design:** Add `pub fn emit_doc_ir(module: &CompiledModule) -> DocIR` where `DocIR` is a flat Vec of `DocItem { name, signature, doc_comment, source_span }`; exportable to JSON via serde
  - **Files:** `src/` (identify compiled module representation), new `src/doc_ir/` module
  - **Prerequisites:** oxilean-doc crate (Ring 0) — implement after doc-gen exists
  - **Tests:** Golden DocIR for a fixture module
  - **Risk:** CompiledModule AST may not preserve doc comments — needs to be plumbed through from parse stage

## v0.1.4 (released 2026-11-12)

- [x] Hygiene pass (commit 930c328): `.unwrap()` / `.expect()` outside tests 35 -> 4. The native back end's linear-scan allocator now holds (interval, register) pairs, and the Ruby and EVM emitters build their constants and text by construction. The commit names Tarjan's stack pops among the calls deliberately left.
- [x] MSRV 1.89 clippy suggestions applied.
- [x] New tests cover the register allocator, Tarjan's SCC (`opt_inline`), topological ordering (`opt_passes`), Prolog quoting and Ruby emission.
