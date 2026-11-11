# oxilean-elab

[![Crates.io](https://img.shields.io/crates/v/oxilean-elab.svg)](https://crates.io/crates/oxilean-elab)
[![Docs.rs](https://docs.rs/oxilean-elab/badge.svg)](https://docs.rs/oxilean-elab)

> **Elaborator for the OxiLean theorem prover**

The elaborator translates surface syntax (from the parser) into kernel-checked terms. It handles type inference, implicit argument insertion, unification, pattern match compilation, and tactic execution. This crate is **untrusted** -- the kernel independently verifies all produced terms.

103,291 SLOC -- fully implemented elaboration pipeline (514 source files, 3,511 tests passing).

## Architecture

```
Surface AST (from oxilean-parse)
    |
    v
+------------------------------------------+
|              Elaborator                   |
|                                           |
|  +---------------+    +---------------+   |
|  |  Meta-vars    |<---|  Unification  |   |
|  | (metavar.rs)  |    |  (unify.rs)   |   |
|  +------+--------+    +-------+-------+   |
|         |                     |           |
|  +------v---------------------v------+    |
|  |    Expression Elaboration         |    |
|  |      (elab_expr.rs)               |    |
|  +------+----------------------------+    |
|         |                                 |
|  +------v----------------------------+    |
|  |    Declaration Elaboration        |    |
|  |      (elab_decl.rs)               |    |
|  +------+----------------------------+    |
|         |                                 |
|  +------v----------------------------+    |
|  |       Tactic Engine               |    |
|  |     (tactic/mod.rs)               |    |
|  +-----------------------------------+    |
+------------------------------------------+
    |
    v
Kernel Expr terms -> sent to oxilean-kernel for verification
```

## Module Overview

| Module | Status | Description |
|--------|--------|-------------|
| `metavar` | Implemented | Metavariable context and assignment tracking |
| `unify`, `solver` | Implemented | Higher-order unification and constraint solving |
| `elab_expr`, `elaborate` | Implemented | Expression elaboration |
| `elab_decl`, `command_elab` | Implemented | Declaration and command elaboration |
| `pattern_match`, `equation`, `mutual` | Implemented | Pattern match compilation, equations, mutual recursion |
| `tactic`, `tactic_auto`, `metaprog` | Implemented | Tactic engine, core tactics, auto search, user-defined tactics |
| `meta_bridge` | Implemented | Bridge to the `oxilean-meta` tactic implementations (omega, polyrith, cc, simp, ...) |
| `typeclass`, `instance`, `coercion`, `derive` | Implemented | Type classes, instances, coercions, `deriving` |

## Key Concepts

### Meta-variables

Meta-variables (`?m`) represent "holes" in partially-constructed terms. The elaborator creates metavariables for:
- Implicit arguments (`{alpha : Type}` -> insert `?alpha`)
- User-written holes (`_`)
- Tactic goals

Metavariables are **never** sent to the kernel -- they must all be resolved during elaboration.

### Unification

Given `?m x1 ... xn =? t`, the unifier finds assignments for `?m`. Strategies:

1. **First-order**: `?m =? t` where `?m` not in `t` -> assign `?m := t`
2. **Pattern (Miller)**: `?m x1 ... xn =? t` where `xi` are distinct FVars -> `?m := fun x1...xn. t`
3. **Postponement**: when unification cannot proceed, delay and retry later

### Implicit Argument Insertion

When applying `f : {alpha : Type} -> alpha -> alpha` to an argument `a`:
1. Create metavariable `?alpha` for the implicit parameter
2. Apply: `f ?alpha a`
3. Unification determines `?alpha` from context

### Tactic Engine

Tactics operate on a `TacticState` containing goals:

```rust,ignore
pub struct TacticState {
    goals: Vec<Goal>,                    // open goals
    pub solved: Vec<Name>,               // names of solved goals
    pub certificate: Option<oxilean_meta::ProofCertificate>,
}

pub struct Goal {
    pub name: Name,
    pub mvar_id: u64,                          // the hole to fill
    pub hypotheses: Vec<(Name, Expr)>,         // local hypotheses
    pub local_ctx: Vec<(Name, Expr, Option<Expr>)>,
    pub target: Expr,                          // what needs to be proved
    pub tag: Option<String>,
}
```

### Core Tactics

| Tactic | Description |
|--------|-------------|
| `intro` | Introduce Pi binders as local hypotheses |
| `exact` | Provide an exact proof term |
| `assumption` | Search local context for a matching hypothesis |
| `apply` | Apply a lemma, generating subgoals for remaining arguments |
| `cases` | Case split on an inductive term |
| `induction` | Structural induction with induction hypotheses |
| `rfl` / `refl` | Prove `a = a` by reflexivity |
| `rewrite` / `rw` | Rewrite using an equality proof |
| `simp` | Simplification engine with oriented rewrite lemmas |

## Dependencies

- `oxilean-kernel` -- for `Expr`, `Name`, `Level`, `Environment` types
- `oxilean-parse` -- for surface AST types

## Testing

```bash
cargo test -p oxilean-elab
```

## License

Copyright COOLJAPAN OU (Team Kitasan). Apache-2.0 -- See [LICENSE](../../LICENSE) for details.
