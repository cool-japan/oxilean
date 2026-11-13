# oxilean-runtime

[![Crates.io](https://img.shields.io/crates/v/oxilean-runtime.svg)](https://crates.io/crates/oxilean-runtime)
[![Docs.rs](https://docs.rs/oxilean-runtime/badge.svg)](https://docs.rs/oxilean-runtime)

> **Runtime System for the OxiLean Theorem Prover**

`oxilean-runtime` implements the execution substrate for compiled OxiLean programs and proofs. It provides everything needed to evaluate terms that have already been type-checked by the kernel: memory management, closure representation, lazy evaluation, I/O, and parallel task scheduling.

Unlike the kernel -- which is the Trusted Computing Base (TCB) and must be auditable -- the runtime is **untrusted**: bugs here can cause incorrect evaluation results or crashes but never logical unsoundness (all terms are checked before reaching the runtime).

35,716 SLOC -- fully implemented runtime system (278 source files, 1,187 tests passing).

Part of the [OxiLean](https://github.com/cool-japan/oxilean) project -- a Lean-compatible theorem prover implemented in pure Rust.

## Overview

### Module Reference

| Module | Description |
|--------|-------------|
| `arena` | Typed and generational arena allocators (`BumpArena`, `GenerationalArena`, `RegionManager`) |
| `object` | Tagged-pointer runtime value representation (`RtObject`, `TypeTag`, `TypeRegistry`) |
| `rc` | Reference-counted smart pointers with elision analysis (`Rc`, `RtArc`, `CowBox`) |
| `closure` | Flat closure representation and partial application (`Closure`, `Pap`, `FunctionTable`) |
| `lazy_eval` | Call-by-need thunks with memoization (`Thunk`, `SharedThunk`, `MemoFn`) |
| `tco` | Trampoline-based tail-call optimization (`trampoline`, `TailCallDetector`) |
| `scheduler` | Work-stealing parallel task scheduler (`Scheduler`, `Worker`, `WorkStealingDeque`) |
| `io_runtime` | File, console, and string I/O (`IoRuntime`, `IoExecutor`, `StringFormatter`) |
| `bytecode_interp` | Bytecode interpreter for evaluation |
| `gc_strategies` | Pluggable garbage collection strategies |
| `memory_pool` | Pool-based allocator for fixed-size objects |
| `profiler` | Runtime profiling and statistics |
| `string_pool` | Interned string storage |
| `wasm_runtime` | WebAssembly-specific runtime adaptations |

### WASM / Bytecode Interpreter

| Entry Point | Description |
|-------------|-------------|
| `WasmModule::call_function` | Complete WebAssembly bytecode interpreter; all 157 `WasmInstruction` variants dispatched via `StackMachine`; structured control flow (`Block`/`Loop`/`If`/`Else`/`End`), branch instructions (`Br`/`BrIf`/`BrTable`/`Return`), frame-stack call dispatch (`Call`/`CallIndirect` with locals, return-PC, and label-base bookkeeping) |
| `WasmModule::register_function` | Register named functions into the `WasmModule.functions` registry for cross-function call dispatch |

### Scheduler Configuration

```rust
use oxilean_runtime::SchedulerConfig;

let config = SchedulerConfig {
    num_workers: 4,            // parallel evaluation threads
    work_stealing: true,       // idle workers steal from busy deques
    ..SchedulerConfig::new()   // remaining fields take their defaults
};
```

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
oxilean-runtime = "0.1.5"
```

### Arena Allocation

```rust
use oxilean_runtime::{BumpArena, TypedArena};

// Byte-level bump allocation: `alloc` returns an `ArenaOffset`
let mut bump = BumpArena::with_chunk_size(64 * 1024);
let offset = bump.alloc(16);

// Typed allocation: `alloc` returns an `ArenaIdx`
let mut arena: TypedArena<u64> = TypedArena::new();
let idx = arena.alloc(42u64);
assert_eq!(arena.get(idx), Some(&42));
```

### Closures and Partial Application

```rust
use oxilean_runtime::{Closure, FnPtr, Pap, PapResult, RtObject};

// A closure over a 2-argument function, partially applied to one argument
let closure = Closure::new(FnPtr { index: 0, module_id: 0 }, 2, vec![]);
let pap = Pap::new(closure, vec![RtObject::nat(1)]);
match pap.apply(&[RtObject::nat(2)]) {
    PapResult::Saturated { closure, args } => { /* ready to call */ }
    PapResult::UnderApplied(pap) => { /* still waiting for arguments */ }
    _ => {}
}
```

### Lazy Evaluation

```rust
use oxilean_runtime::SharedThunk;

// Wrap a computation in a lazily evaluated thunk
let thunk = SharedThunk::new(|| 6 * 7);
let value = thunk.force(); // computed only once, memoized thereafter
```

## Dependencies

- `oxilean-kernel` -- expression types and environment definitions

## Testing

```bash
cargo test -p oxilean-runtime
cargo test -p oxilean-runtime -- --nocapture
```

## License

Copyright COOLJAPAN OU (Team KitaSan). Apache-2.0 -- See [LICENSE](../../LICENSE) for details.
