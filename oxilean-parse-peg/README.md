# leo4-lean4-parse

PEG-based Lean 4 parser. Built from scratch (using the [`peg`](https://crates.io/crates/peg) parser-generator crate) to escape the narrow subset of Lean 4 syntax that `oxilean-parse` v0.1.2 accepts — the parser that `leo4-oxilean-build`'s OX3 / OX4 textual pre-rewrites were trying to lift Lean 4 source *into*.

## Why a new parser?

`leo4-oxilean-build`'s OX3 + OX4 textual pre-rewrites covered the *cheap* surface mismatches (header binders, `inductive ... where`, `.ctorName` shorthand, attribute args, `deriving` clauses). What remained were *deep* Lean 4 features that textual rewriting can't safely handle:

- Binary operator notation (`==`, `+`, `<`, `&&`, …) — needs precedence + associativity-aware parsing.
- String interpolation (`s!"hello {x}!"`).
- Constructor name resolution (`none`, `some`, `Except.ok`).
- `if-then-else`, `match` arms with complex patterns.
- `do` notation with `←` binds.

Once the surface coverage gap got this deep, vendoring + patching `oxilean-parse` (76k LoC, Apache-2.0) stopped being practical and a from-scratch parser became the right move. PEG-based grammar definition keeps the surface declarative + extensible.

## Strict superset invariant

Where `leo4-lean4-parse` and `oxilean-parse` cover the same input shape, both **must produce equivalent ASTs** (modulo type-name differences). For inputs only `leo4-lean4-parse` accepts, `oxilean-parse` may legitimately reject — that's the strict-superset direction. A future test suite cross-checks the overlap.

## AST shape

The emitted AST is structurally compatible with `oxilean-parse`'s `Decl` / `SurfaceExpr` types (same constructor names, same field layouts) so downstream consumers (`oxilean-elab`, leo4-oxilean-build's transpile pipeline) can swap parsers transparently.

## Status (2026-05-22)

**Scaffold** — only the most basic header-binder `def` form parses today:

```
def NAME [binders]+ [: TYPE] := VALUE
```

with primitive type names (`Nat`, `UInt32`, `Bool`, …), simple bracket-balanced types, and bare-value bodies.

## Roadmap

Each step is a separate commit:

1. **(Scaffold)** PEG crate setup + minimal `def` grammar.
2. Expression grammar — binary operators with precedence (`==`, `<`, `+`, `*`, `&&`, …).
3. `if-then-else` + `match` arms.
4. Lambda + `fun … => …`.
5. `structure` + `inductive` + `deriving`.
6. Attribute lists (with args).
7. `do` notation.
8. String interpolation.
9. Full `Decl` enum (theorem / lemma / axiom / instance / class / namespace / open / variable / mutual).
10. Cross-check against `oxilean-parse` on shared corpus.
11. leo4-oxilean-build switches default parser to `leo4-lean4-parse`.

## Future: replacing oxilean-elab too

`oxilean-elab` is the next narrow-subset bottleneck (OX5 — elab env bootstrap, ctor-name resolution, etc.). Once `leo4-lean4-parse` is mature, an analogous `leo4-lean4-elab` sibling would close the elab gap by either binding to Lean 4's reference elaborator (via `lean_proc`-style export) or implementing a stripped-down elaborator over our AST. Tracked separately.

## License

Apache-2.0, same as `oxilean-parse`. The grammar is original work (PEG-defined); no source from `oxilean-parse` is copied. The AST shapes are designed to mirror `oxilean-parse`'s public types for interoperability.
