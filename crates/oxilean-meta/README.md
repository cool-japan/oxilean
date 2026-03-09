# oxilean-meta

[![Crates.io](https://img.shields.io/crates/v/oxilean-meta.svg)](https://crates.io/crates/oxilean-meta)
[![Docs.rs](https://docs.rs/oxilean-meta/badge.svg)](https://docs.rs/oxilean-meta)

> **Meta Layer -- Metavariable-Aware Operations and Tactic Infrastructure**

`oxilean-meta` extends the trusted kernel with metavariable support, providing all the infrastructure required for elaboration, unification, type class instance synthesis, and interactive tactic-based proving. It mirrors Lean 4's `Lean.Meta` namespace and sits logically between the elaborator (`oxilean-elab`) and the kernel (`oxilean-kernel`).

This crate is **untrusted** with respect to soundness: metavariable assignment and unification code cannot corrupt the kernel environment. All proofs produced by the tactic system are ultimately re-verified by the kernel before being accepted.

152,716 SLOC -- comprehensive meta layer implementation (648 source files, 5,184 tests passing).

Part of the [OxiLean](https://github.com/cool-japan/oxilean) project -- a Lean-compatible theorem prover implemented in pure Rust.

## Overview

### Architecture

```text
+------------------------------------------------------+
|              Meta Layer (oxilean-meta)                |
+------------------------------------------------------+
|                                                       |
|  +------------------+  +---------------------------+  |
|  |  Core Meta Ops   |  |  Advanced Features        |  |
|  +------------------+  +---------------------------+  |
|  | MetaContext       |  | Instance Synthesis        |  |
|  | MetaWhnf          |  | Discrimination Trees     |  |
|  | MetaDefEq         |  | App Builder              |  |
|  | MetaInferType     |  | Congruence Theorems      |  |
|  | Level DefEq       |  |                           |  |
|  +------------------+  +---------------------------+  |
|                                                       |
|  +----------------------------------------------+    |
|  |  Tactic System                                |    |
|  +----------------------------------------------+    |
|  |  intro, apply, exact, rw, simp, omega, ...    |    |
|  |  Goal & TacticState management                |    |
|  |  Calc proofs, Omega linear arithmetic         |    |
|  +----------------------------------------------+    |
|                                                       |
+------------------------------------------------------+
                         |
                         | uses (read-only)
                         v
+------------------------------------------------------+
|              OxiLean Kernel (TCB)                     |
|        (Expr, WHNF, DefEq, Environment)              |
+------------------------------------------------------+
```

### Key Concepts

**MetaContext** holds all proof-session global state: every metavariable, its type, and its current assignment (or lack thereof). It is threaded through all meta operations but does not change the kernel environment.

**MetaState** holds local tactic goal-solving state: the current proof goal, subgoals, and the local context (variables and hypotheses in scope). It changes as tactics are applied.

**A proof goal** is displayed as:

```text
x : Nat
h : P x
|- Q x
```

where the variables and hypotheses above `|-` form the local context and `Q x` is the type to prove.

**Tactics** transform the goal list:

| Tactic | Effect |
|--------|--------|
| `intro h` | Introduce a hypothesis from a forall/arrow goal |
| `apply f` | Unify the goal type with the conclusion of `f` |
| `exact e` | Close the goal if `e` has exactly the goal type |
| `rw [h]` | Rewrite the goal using equation `h` |
| `simp [...]` | Simplify using a lemma set |
| `omega` | Solve linear arithmetic over integers and naturals |

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
oxilean-meta = "0.1.1"
```

### Creating a Meta Context

```rust,ignore
use oxilean_meta::{MetaContext, MetaConfig};
use oxilean_kernel::Environment;

let env = Environment::new();
let config = MetaConfig::default();
let mut meta_ctx = MetaContext::new(&env, config);
```

### Creating and Solving Metavariables

```rust,ignore
use oxilean_meta::MVarId;

let m1 = meta_ctx.mk_metavar(ty)?;
// ... unification happens ...
let solution = meta_ctx.get_assignment(m1)?;
```

### Running a Tactic

```rust,ignore
use oxilean_meta::TacticState;

let mut state = TacticState::new(&meta_ctx, goal)?;
// apply a tactic
state.apply_tactic(intro("h"))?;
let proof = state.close()?;
```

## Dependencies

- `oxilean-kernel` -- kernel expression types and environment

## Testing

```bash
cargo test -p oxilean-meta
cargo test -p oxilean-meta -- --nocapture
```

## License

Copyright COOLJAPAN OU (Team Kitasan). Apache-2.0 -- See [LICENSE](../../LICENSE) for details.
