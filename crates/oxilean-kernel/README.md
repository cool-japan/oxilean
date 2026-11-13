# oxilean-kernel

[![Crates.io](https://img.shields.io/crates/v/oxilean-kernel.svg)](https://crates.io/crates/oxilean-kernel)
[![Docs.rs](https://docs.rs/oxilean-kernel/badge.svg)](https://docs.rs/oxilean-kernel)

> **The Trusted Computing Base (TCB) of OxiLean**

The kernel is the core responsible for type checking in the Calculus of Inductive Constructions (CiC). Only code in this crate needs to be trusted for logical soundness.

## Design Principles

- **Zero external dependencies** -- only `std` is used
- **No `unsafe` code** -- enforced by `#![forbid(unsafe_code)]`
- **126,740 SLOC** -- comprehensive implementation (965 source files under `src/`)
- **3,742 tests passing** -- comprehensive coverage
- **WASM-compatible** -- no system calls
- **MSRV 1.89**

```toml
[dependencies]
oxilean-kernel = "0.1.5"
```

### Cargo features

| Feature | Default | Effect |
|---------|---------|--------|
| `term-depth` | off | Track the depth of every shared subterm (four extra bytes each) so that `fuel::set_term_depth_limit` can report a term too deep for the caller's stack as a resource limit instead of overflowing it. `oxilean-verify-wasm` turns it on for `wasm32`. |

### Resource limits (`fuel`)

`fuel::set_reduction_nesting_limit` / `reduction_nesting_limit` bound how many
reductions may nest inside one another; `fuel::set_term_depth_limit` /
`term_depth_limit` / `term_depth_limit_hit` bound term depth (needs the
`term-depth` feature to be enforced). Over a limit, the check ends as a
resource limit, never as a type error.

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

Hierarchical names representing identifiers like `Nat.add.comm`. `Name` is an
interned `Rc` newtype with a structural hash stored at construction (0.1.4,
"Stage F"); match on it through `Name::view()`:

```rust
enum NameView<'a> {
    Anonymous,
    Str(&'a Name, &'a str),
    Num(&'a Name, u64),
}
// Convenience macro:
let n = name!("Nat", "add", "comm");  // -> Nat.add.comm
```

### `Level`

Universe levels for the sort hierarchy `Prop : Type 0 : Type 1 : ...`. Like
`Name`, `Level` is an `Rc` newtype with a stored hash; match on it through
`Level::view()`:

```rust
enum LevelView<'a> {
    Zero,                          // 0 (Prop)
    Succ(&'a Level),               // u + 1
    Max(&'a Level, &'a Level),     // max(u, v)
    IMax(&'a Level, &'a Level),    // imax(u, v) = 0 if v=0, else max(u,v)
    Param(&'a Name),               // universe parameter
    MVar(LevelMVarId),             // universe metavariable (unification)
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
| `App(Node, Node)` | Application | `f a` |
| `Lam(BinderInfo, Name, Node, Node)` | Lambda | `fun (x : T), body` |
| `Pi(BinderInfo, Name, Node, Node)` | Pi / forall | `Pi (x : T), body` |
| `Let(Name, Node, Node, Node)` | Let binding | `let x : T := v in body` |
| `Lit(Literal)` | Literal | `42`, `"hello"` |
| `Proj(Name, u32, Node)` | Projection | `s.1` |

Child edges are `Node` — a transparent wrapper over a shared allocation that
caches each subterm's `looseBVarRange`, rebuild-cost (saturating at
`expr::REBUILD_COST_MAX`) and structural hash, so `clone` is an O(1) refcount
bump, hashing a term is O(1), and a substitution can skip an untouched subtree
in O(1). `Node` `Deref`s to `Expr`, so pattern matching is unchanged. Build one
with `Node::new(expr)`; identity is `Node::ptr_eq` / `Node::as_ptr` (the
0.1.3 `Node::rc()` is gone).

### Performance and checking (0.1.4)

- `instantiate`, `lift`, `abstract` and universe instantiation walk terms as
  they are shared: a shared subterm is rebuilt once per walk, not once per path.
- Type inference remembers what it has typed (a type-inference cache), and
  definitional equality tries congruence first (`f a ≡ f b`) before unfolding.
- Nested inductives are elaborated to mutual inductives (`verify_nested_bundle`)
  when the family has no parameters and is nested through a container that is
  not part of a mutual block; any other nested inductive is refused as
  `UnsupportedNestedInductive`. Within that scope three shapes Lean's kernel
  accepts are still refused, with other errors (C25, C28 and C29 in the
  workspace's `TODO_VERIFY.md`). `add_inductive_family` takes no nested
  inductive.
- `check_constant_info` checks that a constant's terms are closed.

## Usage

```rust
use oxilean_kernel::{Name, Level, Expr, BinderInfo, Node, name};

// Create Prop (Sort 0)
let prop = Expr::Sort(Level::zero());

// Create Type 0 (Sort 1)
let type0 = Expr::Sort(Level::succ(Level::zero()));

// Create identity function type: Pi (alpha : Type 0), alpha -> alpha
let alpha = Name::str("alpha");
let id_type = Expr::Pi(
    BinderInfo::Implicit,
    alpha.clone(),
    Node::new(type0),
    Node::new(Expr::Pi(
        BinderInfo::Default,
        Name::anonymous(),
        Node::new(Expr::BVar(0)),
        Node::new(Expr::BVar(1)),
    )),
);
```

## Testing

```bash
cargo test -p oxilean-kernel
cargo test -p oxilean-kernel -- --nocapture  # verbose
```

## License

Copyright COOLJAPAN OU (Team KitaSan). Apache-2.0 -- See [LICENSE](../../LICENSE) for details.
