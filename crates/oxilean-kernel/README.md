# oxilean-kernel

[![Crates.io](https://img.shields.io/crates/v/oxilean-kernel.svg)](https://crates.io/crates/oxilean-kernel)
[![Docs.rs](https://docs.rs/oxilean-kernel/badge.svg)](https://docs.rs/oxilean-kernel)

> **The Trusted Computing Base (TCB) of OxiLean**

The kernel is the core responsible for type checking in the Calculus of Inductive Constructions (CiC). Only code in this crate needs to be trusted for logical soundness.

## Design Principles

- **Zero external dependencies** -- only `std` is used
- **No `unsafe` code** -- enforced by `#![forbid(unsafe_code)]`
- **115,444 SLOC** -- comprehensive implementation (904 source files)
- **3,444 tests passing** -- comprehensive coverage
- **WASM-compatible** -- no system calls

## Module Overview

| Module | Status | Description |
|--------|--------|-------------|
| `arena.rs` | Implemented | Typed arena allocator with `Idx<T>` index type |
| `name.rs` | Implemented | Hierarchical names (`Nat.add.comm`) with `name!` macro |
| `level.rs` | Implemented | Universe levels (`Zero`, `Succ`, `Max`, `IMax`, `Param`) |
| `expr.rs` | Implemented | Core expression type (11 variants) |
| `subst.rs` | Implemented | Substitution: `instantiate`, `abstract`, `lift_bvars` |
| `reduce.rs` | Implemented | WHNF reduction (beta, delta, zeta, iota, projection, quotient) |
| `infer.rs` | Implemented | Type inference for all `Expr` forms |
| `def_eq.rs` | Implemented | Definitional equality with proof irrelevance |
| `check.rs` | Implemented | Declaration checking (Axiom, Definition, Theorem) |
| `inductive.rs` | Implemented | Inductive types, positivity check, recursor generation |
| `env.rs` | Implemented | Global environment (declaration storage) |
| `whnf.rs` | Implemented | Weak head normal form evaluation |
| `quot.rs` | Implemented | Quotient type support |
| `builtin.rs` | Implemented | Built-in constant definitions and operations |

## Core Types

### `Arena<T>` & `Idx<T>`

Typed arena allocator. All expression nodes are allocated contiguously for cache-friendly traversal. `Idx<T>` is a `u32`-based index with zero-cost `PhantomData` typing.

```rust
let mut arena = Arena::new();
let idx: Idx<MyType> = arena.alloc(value);
let val: &MyType = arena.get(idx);
```

### `Name`

Hierarchical names representing identifiers like `Nat.add.comm`:

```rust
enum Name {
    Anonymous,
    Str(Box<Name>, String),
    Num(Box<Name>, u64),
}
// Convenience macro:
let n = name!("Nat", "add", "comm");  // -> Nat.add.comm
```

### `Level`

Universe levels for the sort hierarchy `Prop : Type 0 : Type 1 : ...`:

```rust
enum Level {
    Zero,                          // 0 (Prop)
    Succ(Box<Level>),              // u + 1
    Max(Box<Level>, Box<Level>),   // max(u, v)
    IMax(Box<Level>, Box<Level>),  // imax(u, v) = 0 if v=0, else max(u,v)
    Param(Name),                   // universe parameter
}
```

### `Expr`

The core expression type -- all terms in the type theory:

| Variant | Description | Example |
|---------|-------------|---------|
| `BVar(u32)` | Bound variable (de Bruijn index) | `#0`, `#1` |
| `FVar(FVarId)` | Free variable (unique ID) | `x`, `alpha` |
| `Sort(Level)` | Universe sort | `Prop`, `Type u` |
| `Const(Name, Vec<Level>)` | Named constant | `Nat.add.{u}` |
| `App(Box<Expr>, Box<Expr>)` | Application | `f a` |
| `Lam(BinderInfo, Name, Box<Expr>, Box<Expr>)` | Lambda | `fun (x : T), body` |
| `Pi(BinderInfo, Name, Box<Expr>, Box<Expr>)` | Pi / forall | `Pi (x : T), body` |
| `Let(Name, Box<Expr>, Box<Expr>, Box<Expr>)` | Let binding | `let x : T := v in body` |
| `Lit(Literal)` | Literal | `42`, `"hello"` |
| `Proj(Name, u32, Box<Expr>)` | Projection | `s.1` |

## Usage

```rust
use oxilean_kernel::{Name, Level, Expr, BinderInfo, Arena, Idx, name};

// Create Prop (Sort 0)
let prop = Expr::Sort(Level::zero());

// Create Type 0 (Sort 1)
let type0 = Expr::Sort(Level::succ(Level::zero()));

// Create identity function type: Pi (alpha : Type 0), alpha -> alpha
let alpha = Name::str("alpha");
let id_type = Expr::Pi(
    BinderInfo::Implicit,
    alpha.clone(),
    Box::new(type0),
    Box::new(Expr::Pi(
        BinderInfo::Default,
        Name::Anonymous,
        Box::new(Expr::BVar(0)),
        Box::new(Expr::BVar(1)),
    )),
);
```

## Testing

```bash
cargo test -p oxilean-kernel
cargo test -p oxilean-kernel -- --nocapture  # verbose
```

## License

Copyright COOLJAPAN OU (Team Kitasan). Apache-2.0 -- See [LICENSE](../../LICENSE) for details.
