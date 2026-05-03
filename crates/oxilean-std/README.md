# oxilean-std

[![Crates.io](https://img.shields.io/crates/v/oxilean-std.svg)](https://crates.io/crates/oxilean-std)
[![Docs.rs](https://docs.rs/oxilean-std/badge.svg)](https://docs.rs/oxilean-std)

> **Standard Library for the OxiLean Theorem Prover**

The OxiLean standard library provides essential inductive types, type classes, and theorems that form the foundation for formal mathematics in OxiLean. It is logically a layer above the kernel: everything defined here is ultimately verified by `oxilean-kernel`, so bugs in the standard library can cause incorrect proofs but never unsoundness.

The library is organized into primitive types (`Nat`, `Bool`, `List`, `Option`, `Result`), algebraic type classes (`Eq`, `Ord`, `Functor`, `Monad`, `Monoid`), and core theorems covering equality, logic, arithmetic, and ordering.

416,133 SLOC -- the largest crate in the OxiLean workspace, providing comprehensive standard library coverage (1,105 source files, 7,977 tests passing).

Part of the [OxiLean](https://github.com/cool-japan/oxilean) project -- a Lean-compatible theorem prover implemented in pure Rust.

## Overview

`oxilean-std` covers three broad areas:

### Primitive Types

Inductive definitions of the core data types used throughout OxiLean proofs:

```text
Nat : Type 0          -- Peano natural numbers
  | zero : Nat
  | succ : Nat -> Nat

Bool : Prop
  | true  : Bool
  | false : Bool

List alpha : Type 0
  | nil  : List alpha
  | cons : alpha -> List alpha -> List alpha

Option alpha : Type 0
  | none : Option alpha
  | some : alpha -> Option alpha
```

Each inductive type is accompanied by its recursor (for primitive recursion) and induction principle (for structural inductive proofs).

### Type Classes

Ad-hoc polymorphism via type classes mirrors the Lean 4 `Std` hierarchy:

| Class | Description |
|-------|-------------|
| `Eq` | Setoid equality with `refl`, `symm`, `trans` |
| `Ord` | Total ordering via `compare : alpha -> alpha -> Ordering` |
| `Functor` | Covariant mapping over a type constructor |
| `Monad` | Sequential computation with `pure` and `bind` |
| `Monoid` | Associative binary operation with an identity element |
| `Semigroup` | Associative binary operation |
| `Decidable` | Computationally decidable propositions |

### Core Theorems & Lemmas

Reusable lemmas grouped by subject:

- **`eq.rs`** -- `Eq.refl`, `Eq.symm`, `Eq.trans`, congruence
- **`logic.rs` / `prop.rs`** -- `And`, `Or`, `Not`, `Iff`, classical axioms
- **`nat.rs`** -- arithmetic (`add_comm`, `mul_assoc`, `zero_add`, ...)
- **`int.rs`** -- integer arithmetic and order
- **`ord.rs` / `order.rs`** -- ordering properties and monotonicity lemmas

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
oxilean-std = "0.1.2"
```

Example OxiLean proof using standard library theorems:

```text
-- Uses Nat from the standard library
theorem two_plus_three : 2 + 3 = 5 := by
  norm_num

-- Uses List from the standard library
def my_list : List Nat := [1, 2, 3]

-- Uses Eq type class instance for Nat
#check (inferInstance : Eq Nat)
```

## Dependencies

- `oxilean-kernel` -- expression types (`Expr`, `Level`, `Name`) and environment

## Testing

```bash
cargo test -p oxilean-std
cargo test -p oxilean-std -- --nocapture
```

## License

Copyright COOLJAPAN OU (Team Kitasan). Apache-2.0 -- See [LICENSE](../../LICENSE) for details.
