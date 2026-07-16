# Lean 4 Export Format — Consolidated Specification

**Source authority:** [`leanprover/lean4export`](https://github.com/leanprover/lean4export)  
**Format version:** 3.1.0  
**Exporter name:** `lean4export`  
**Latest commit (as of audit):** `3de59f10bc4b4a0f2de698597aeb1246caa0df0a`  
  (message: "chore: bump toolchain to v4.32.0-rc1 (#39)", date: 2026-06-17)  
**Lean toolchain pinned by lean4export:** `leanprover/lean4:v4.32.0-rc1`  
  (from `lean-toolchain` file at master branch)

This document consolidates the authoritative format specification from:
1. `format_ndjson.md` in the lean4export repo (primary spec)
2. `Export.lean` in the lean4export repo (serializer — ground truth for exact field names)
3. `Export/Parse.lean` in the lean4export repo (reference parser/deserializer)

> **IMPORTANT FOR oxilean-verify IMPLEMENTERS:** The oxilean-export reader MUST be
> implemented from this spec, not from memory or from prior text-format lean4export
> (the older format used space-delimited text lines; the current format is NDJSON).
> See section "Version History" below.

---

## 1. File Structure

The format is **NDJSON** (Newline-Delimited JSON). Every line is a complete, valid JSON
object with no internal newlines (the spec notes: "the NDJSON format requires these
JSON objects to be rendered without any line breaks").

File layout:
```
<meta object>           ← exactly one, must be first line
<primitive or decl>     ← zero or more, one per line, in dependency order
...
```

### 1.1 Initial Metadata Object

```json
{"meta":{"exporter":{"name":"lean4export","version":"3.1.0"},"lean":{"githash":"<sha>","version":"<ver>"},"format":{"version":"3.1.0"}}}
```

Schema:
```
{
    "meta": {
        "exporter": {
            "name": string,
            "version": string
        },
        "lean": {
            "githash": string,
            "version": string
        },
        "format": {
            "version": string
        }
    }
}
```

The `format.version` field identifies the format version. Current value: `"3.1.0"`.

---

## 2. Primitive Tables

There are three primitive kinds: **Names**, **Levels**, and **Expressions**. Each
primitive, when first encountered, is assigned a sequential integer index (starting
from the values described below) and that index is placed in the JSON object so the
reader can cache it for later back-references.

### 2.1 Index Tag Fields

| Primitive kind | Index field name | Pre-seeded at index 0 |
|---|---|---|
| Name | `"in"` | `Name.anonymous` (empty name, not printed) |
| Level | `"il"` | `Level.zero` (universe level 0, not printed) |
| Expr | `"ie"` | (no pre-seed, first output expr gets index 0) |

The index is **0-based**. Index 0 for Names is pre-seeded with `anonymous`, and index 0
for Levels is pre-seeded with `zero`. These two are never emitted as lines; the reader
must initialize its tables accordingly:

```
nameMap[0]  = Name.anonymous   (pre-seeded, no line emitted)
levelMap[0] = Level.zero       (pre-seeded, no line emitted)
```

The reference parser (`Export/Parse.lean`) confirms this initialization:
```lean
nameMap  : Std.HashMap Nat Lean.Name := { (0, .anonymous) }
levelMap : Std.HashMap Nat Lean.Level := { (0, .zero) }
exprMap  : Std.HashMap Nat Lean.Expr := {}
```

---

## 3. Name Lines

A Name line encodes a hierarchical Lean name component. Names are built recursively:
a name is either `anonymous` (index 0, not emitted), a string extension of a prefix,
or a numeric extension of a prefix.

### 3.1 Name.str

```json
{"in": <integer>, "str": {"pre": <integer>, "str": <string>}}
```

- `"in"`: the new index assigned to this name
- `"str"."pre"`: index of the prefix name in the name table
- `"str"."str"`: the string component

### 3.2 Name.num

```json
{"in": <integer>, "num": {"pre": <integer>, "i": <integer>}}
```

- `"in"`: the new index assigned to this name
- `"num"."pre"`: index of the prefix name in the name table
- `"num"."i"`: the numeric component

**JSON key ordering note:** The reference parser normalizes key order so that `"in"`,
`"il"`, or `"ie"` always appears first in the pair. The serializer emits them with
the index key appearing after the type-specific key (e.g. `{"str":{...},"in":5}`).
Readers MUST NOT rely on key order in JSON objects (JSON objects are unordered by
spec). The reference parser explicitly normalizes order before matching.

---

## 4. Level Lines

Universe levels. Index field: `"il"`.

### 4.1 Level.succ

```json
{"succ": <integer>, "il": <integer>}
```

- `"succ"`: index into the level table of the predecessor level
- `"il"`: new index for this level

### 4.2 Level.max

```json
{"max": [<integer>, <integer>], "il": <integer>}
```

- `"max"`: array of two level indices [lhs, rhs]
- `"il"`: new index for this level

### 4.3 Level.imax

```json
{"imax": [<integer>, <integer>], "il": <integer>}
```

- `"imax"`: array of two level indices [lhs, rhs]
- `"il"`: new index for this level

`imax u v` is the impredicative maximum: `imax u 0 = 0`, `imax u (succ v) = max u (succ v)`.
This is the level used by `Prop` (Sort 0 = Prop, Sort 1 = Type, etc.).

### 4.4 Level.param

```json
{"param": <integer>, "il": <integer>}
```

- `"param"`: index into the **name** table of the universe parameter name
- `"il"`: new index for this level

**Note:** `Level.mvar` (unification metavariables) is never exported. The serializer
marks this as `unreachable!`.

---

## 5. Expression Lines

Expressions. Index field: `"ie"`. There are 12 variants.

### 5.1 Expr.bvar (bound variable)

```json
{"bvar": <integer>, "ie": <integer>}
```

- `"bvar"`: de Bruijn index (0 = innermost binder)
- `"ie"`: new expression index

### 5.2 Expr.sort (universe sort)

```json
{"sort": <integer>, "ie": <integer>}
```

- `"sort"`: index into the level table
- `"ie"`: new expression index

### 5.3 Expr.const (constant reference)

```json
{"const": {"name": <integer>, "us": [<integer>, ...]}, "ie": <integer>}
```

- `"const"."name"`: index into the name table of the constant's name
- `"const"."us"`: array of level indices (universe arguments)
- `"ie"`: new expression index

### 5.4 Expr.app (function application)

```json
{"app": {"fn": <integer>, "arg": <integer>}, "ie": <integer>}
```

- `"app"."fn"`: expression index of the function
- `"app"."arg"`: expression index of the argument
- `"ie"`: new expression index

### 5.5 Expr.lam (lambda abstraction)

```json
{"lam": {"name": <integer>, "type": <integer>, "body": <integer>, "binderInfo": <string>}, "ie": <integer>}
```

- `"lam"."name"`: name index (binder name, for pretty printing)
- `"lam"."type"`: expression index of the binder type
- `"lam"."body"`: expression index of the body (binder extends scope by 1)
- `"lam"."binderInfo"`: one of `"default"` | `"implicit"` | `"strictImplicit"` | `"instImplicit"`
- `"ie"`: new expression index

### 5.6 Expr.forallE (dependent function type / Pi type)

```json
{"forallE": {"name": <integer>, "type": <integer>, "body": <integer>, "binderInfo": <string>}, "ie": <integer>}
```

Same structure as `Expr.lam`. Represents `∀ (name : type), body` or `type → body`.

### 5.7 Expr.letE (let binding)

```json
{"letE": {"name": <integer>, "type": <integer>, "value": <integer>, "body": <integer>, "nondep": <boolean>}, "ie": <integer>}
```

- `"letE"."name"`: name index (binder name)
- `"letE"."type"`: expression index of the declared type
- `"letE"."value"`: expression index of the value
- `"letE"."body"`: expression index of the body
- `"letE"."nondep"`: optimization hint — `true` if the body does not depend on the binding.
  **IMPORTANT:** The serializer normalizes `nondep` to `false` when removing mdata (the
  default path) to avoid duplicate expression indices. Lean's `BEq` considers this flag
  but external checkers should treat expressions as identical regardless of `nondep`.
  See commit comment in Export.lean: `ammkrn/lean4export/commit/eb023e5`.
- `"ie"`: new expression index

### 5.8 Expr.proj (structure projection)

```json
{"proj": {"typeName": <integer>, "idx": <integer>, "struct": <integer>}, "ie": <integer>}
```

- `"proj"."typeName"`: name index of the structure type
- `"proj"."idx"`: 0-based projection index
- `"proj"."struct"`: expression index of the structure term
- `"ie"`: new expression index

### 5.9 Expr.lit / Literal.natVal (natural number literal)

```json
{"natVal": <string>, "ie": <integer>}
```

- `"natVal"`: the natural number as a **decimal string** (can be arbitrarily large —
  arbitrary precision required)
- `"ie"`: new expression index

**Note:** nat literal values are encoded as strings to avoid JSON integer overflow for
large Nat values. The reader must use arbitrary-precision arithmetic.

### 5.10 Expr.lit / Literal.strVal (string literal)

```json
{"strVal": <string>, "ie": <integer>}
```

- `"strVal"`: the string value (UTF-8)
- `"ie"`: new expression index

### 5.11 Expr.mdata (metadata annotation)

```json
{"mdata": {"expr": <integer>, "data": <object>}, "ie": <integer>}
```

- `"mdata"."expr"`: expression index of the annotated expression
- `"mdata"."data"`: a JSON object with metadata key-value pairs
- `"ie"`: new expression index

**Note:** `mdata` is only present when the exporter is invoked with `--export-mdata`.
By default the serializer strips `mdata` wrappers. The reference parser's `parseExprMdata`
currently discards `data` and returns `Expr.mdata {} expr`. Readers may safely discard
or ignore metadata content.

**Note:** `Expr.fvar` (free variables) and `Expr.mvar` (metavariables) are never exported.
The serializer panics on these: `panic! "cannot export free variables or metavariables"`.

---

## 6. Declaration Lines

Declarations are top-level JSON objects with a single key naming the declaration kind.
There is no `"ie"`, `"in"`, or `"il"` index; declarations are not indexed.

### 6.1 Axiom

```json
{"axiom": {"name": <integer>, "levelParams": [<integer>, ...], "type": <integer>, "isUnsafe": <boolean>}}
```

- `"name"`: name index
- `"levelParams"`: array of name indices for universe parameter names; the corresponding
  `Level.param` entries have been emitted earlier. Note: `dumpUparams` also emits the
  `Level.param` lines for these names.
- `"type"`: expression index of the axiom type
- `"isUnsafe"`: whether the axiom is marked `unsafe`

### 6.2 Definition

```json
{"def": {"name": <integer>, "levelParams": [<integer>, ...], "type": <integer>, "value": <integer>, "hints": <hints>, "safety": <string>, "all": [<integer>, ...]}}
```

- `"name"`, `"levelParams"`, `"type"`: as above
- `"value"`: expression index of the definition body
- `"hints"`: reducibility hints:
  - `"opaque"` — not unfolded by the simplifier
  - `"abbrev"` — always unfolded
  - `{"regular": <integer>}` — unfolded with given height
- `"safety"`: one of `"unsafe"` | `"safe"` | `"partial"`
- `"all"`: array of name indices for all definitions in the mutual group

### 6.3 Opaque

```json
{"opaque": {"name": <integer>, "levelParams": [<integer>, ...], "type": <integer>, "value": <integer>, "isUnsafe": <boolean>, "all": [<integer>, ...]}}
```

Similar to definition. `"value"` is the opaque value (hidden from the kernel during
type-checking; the kernel only sees the type).

### 6.4 Theorem

```json
{"thm": {"name": <integer>, "levelParams": [<integer>, ...], "type": <integer>, "value": <integer>, "all": [<integer>, ...]}}
```

Theorems are like definitions with `safety = safe` and `hints = opaque`, but they are
exported with the `"thm"` key. Their `"value"` is the proof term.

### 6.5 Quotient

The Lean kernel uses `Lean.Declaration.quotDecl` (a field-less constructor) to add
the four quotient primitives. The exporter emits all four as separate `"quot"` lines
for convenience. When a quotient constant is encountered, the exporter always exports
the full Quot package in order: `Quot`, `Quot.mk`, `Quot.lift`, `Quot.ind`.

```json
{"quot": {"name": <integer>, "levelParams": [<integer>, ...], "type": <integer>, "kind": <string>}}
```

- `"kind"`: one of:
  - `"type"` — `Quot` itself (the type former)
  - `"ctor"` — `Quot.mk` (the constructor)
  - `"lift"` — `Quot.lift` (the eliminator for propositions/functions)
  - `"ind"` — `Quot.ind` (the induction principle)

**For oxilean-verify:** The four quot declarations are axiomatic in Lean's kernel.
They must be added to the environment with their axiomatic types, not checked. The
computation rule `Quot.lift f h (Quot.mk r a) == f a` is a special iota-like rule.
See brief section 4(1).

### 6.6 Inductive Declaration (grouped)

The Lean kernel expects `Lean.Declaration.inductDecl` with the full mutual group and
derives recursors from it. The exporter emits the derived recursors for convenience
(but notes that a full checker re-derives them itself).

```json
{
  "inductive": {
    "types": [<InductiveVal>, ...],
    "ctors": [<ConstructorVal>, ...],
    "recs": [<RecursorVal>, ...]
  }
}
```

All types, constructors, and recursors in a mutual inductive group are bundled into a
single `"inductive"` object.

#### InductiveVal

```json
{
  "name": <integer>,
  "levelParams": [<integer>, ...],
  "type": <integer>,
  "numParams": <integer>,
  "numIndices": <integer>,
  "all": [<integer>, ...],
  "ctors": [<integer>, ...],
  "numNested": <integer>,
  "isRec": <boolean>,
  "isReflexive": <boolean>,
  "isUnsafe": <boolean>
}
```

- `"numParams"`: number of uniform parameters
- `"numIndices"`: number of indices
- `"all"`: array of name indices for all types in the mutual group
- `"ctors"`: array of name indices for constructors of this type
- `"numNested"`: number of nested inductives
- `"isRec"`: whether the type is recursive
- `"isReflexive"`: whether the type is reflexive (used for kernel rules)
- `"isUnsafe"`: whether the inductive is marked `unsafe`

#### ConstructorVal

```json
{
  "name": <integer>,
  "levelParams": [<integer>, ...],
  "type": <integer>,
  "induct": <integer>,
  "cidx": <integer>,
  "numParams": <integer>,
  "numFields": <integer>,
  "isUnsafe": <boolean>
}
```

- `"induct"`: name index of the parent inductive type
- `"cidx"`: 0-based constructor index (position among constructors of the inductive)
- `"numParams"`: number of parameters (same as parent inductive's `numParams`)
- `"numFields"`: number of constructor fields (arguments beyond parameters)
- `"isUnsafe"`: whether the constructor is marked `unsafe`

#### RecursorVal

```json
{
  "name": <integer>,
  "levelParams": [<integer>, ...],
  "type": <integer>,
  "all": [<integer>, ...],
  "numParams": <integer>,
  "numIndices": <integer>,
  "numMotives": <integer>,
  "numMinors": <integer>,
  "rules": [<RecursorRule>, ...],
  "k": <boolean>,
  "isUnsafe": <boolean>
}
```

- `"all"`: name indices of all inductive types in the mutual group
- `"numMotives"`: number of motive arguments (1 for non-mutual, N for mutual)
- `"numMinors"`: number of minor premises (one per constructor)
- `"rules"`: array of iota-reduction rules (one per constructor)
- `"k"`: K-axiom flag — `true` if the K-axiom applies (for inductive types with
  a single constructor and whose recursor has a non-dependent motive that can be
  `Eq`-like); affects iota reduction

#### RecursorRule

```json
{
  "ctor": <integer>,
  "nfields": <integer>,
  "rhs": <integer>
}
```

- `"ctor"`: name index of the constructor this rule applies to
- `"nfields"`: number of fields (arguments) of the constructor
- `"rhs"`: expression index of the RHS of the iota rule

**For oxilean-verify:** The brief requires that recursors are RE-DERIVED from the
inductive declaration rather than trusted as given. The exported `recs` should be
used as cross-checks but the kernel must re-derive and verify them.

---

## 7. Ordering and Dependency Invariants

- Primitive indices always refer to previously-seen entries (forward references do
  not occur in well-formed export files).
- Dependencies of a declaration are emitted before the declaration itself.
- For inductives: constructor dependencies are emitted before the `"inductive"` object.
- For quotients: `Eq` is emitted before the Quot package.
- Names and levels referenced in `levelParams` have their `Level.param` entries emitted
  by `dumpUparams`, which is called before the declaration is emitted.
- The `State.visitedConstants` set prevents re-export of already-seen constants.

---

## 8. Dependency Side Effects for Literals

When a `natVal` literal is emitted, the exporter also ensures `Nat` is exported (if
present in the environment). When a `strVal` literal is emitted, the exporter ensures
`Char.ofNat` and `String.ofList` are exported. These are the "implicit dependencies"
of literal expressions.

---

## 9. Redundant Information and Checker Policy

The format spec explicitly notes:
> "The export format contains information that is redundant and would likely be ignored
> or only validated by a full external checker (such as the types of recursors). These
> are included for the benefit of other tools that want all constants with their types."

**Implications for oxilean-verify:**
- The `"type"` field of constructors and recursors is redundant (derivable from the
  inductive declaration). A full checker should re-derive it and compare.
- The recursor `"rules"` RHS expressions are derivable. oxilean-verify should
  re-derive and not trust the exported RHS.
- The `isRec`, `isReflexive`, `cidx` fields are derived metadata; a full checker
  should re-derive and validate them.

---

## 10. CLI Flags Affecting the Format

| Flag | Effect on output |
|---|---|
| `--export-unsafe` | Includes `unsafe` declarations (normally omitted) |
| `--export-mdata` | Includes `Expr.mdata` nodes (normally stripped) |
| `--ignore-missing` | Silently skips missing constants (debugging) |

---

## 11. Version History and Format Differences

This spec covers **format version 3.1.0** (NDJSON). Earlier versions used a
space-delimited text format (the "old lean4export" format used by trepplein and
lean-checker). That format is completely different.

**Key differences from the old text format (for historical reference):**
- Old format: each line begins with an integer index or a keyword (`#AX`, `#DEF`,
  `#THM`, `#QUOT`, `#IND`, `#CTOR`, `#REC`, `NS`, `NI`, `US`, `UM`, `UIM`, `UP`,
  `EV`, `ES`, `EC`, `EA`, `EL`, `EP`, `EZ`, `EJ`, `ELN`, `ELS`)
- Old format: index 0 for names was pre-seeded as anonymous, level 0 as zero
  (same semantic, different syntax)
- New format (3.1.0): NDJSON, field names are explicit JSON keys

**FIXME-VERIFY:** Whether lean4export ever emitted a v1.0 or v2 format is not
confirmed from the sources fetched. The repository only documents format version 3.1.0.
The old text-line format was used by a different (earlier) `lean4export` tool or by
`lean-checker` / `trepplein`. If the brief's reference to "v0.1/v1.0/v2 version
differences" refers to the text format, those are documented in trepplein/lean-checker
source, NOT in the current `leanprover/lean4export` repo.

---

## 12. Reference Implementation Notes (from lean4lean)

From [`digama0/lean4lean`](https://github.com/digama0/lean4lean)
(commit `8865b155abbf68d3a827fb3568bf6839780163c2`, date: 2026-07-06):

lean4lean is a Lean 4 kernel re-implementation in Lean 4. It does NOT consume the
lean4export NDJSON format — instead it reads `.olean` files directly. It is listed
here because:

1. Its type-checker architecture (`TypeChecker.lean`) demonstrates correct handling
   of all the "hard parts" (quotients, eta, literals, inductives, universes).
2. Its `divergences.md` documents deliberate differences from the official Lean kernel.
3. Its `bugs-found.md` documents kernel bugs discovered through re-implementation.

Key modules for reference:
- `Lean4Lean/Quot.lean` — quotient type handling
- `Lean4Lean/Inductive/Add.lean` — constructing inductive recursors
- `Lean4Lean/Inductive/Reduce.lean` — inductive iota rules

These are relevant to the five hard parts in brief section 4.

---

## 13. Pinning Requirements (per brief)

The brief requires pinning both:

| Item | Value |
|---|---|
| lean4export commit | `3de59f10bc4b4a0f2de698597aeb1246caa0df0a` |
| Lean toolchain | `leanprover/lean4:v4.32.0-rc1` |

These should be recorded in the oxilean-export reader crate's documentation and
in the oxilean-verify CI configuration.

---

## 14. Deduplication Semantics

The serializer uses a cache (`visitedNames`, `visitedLevels`, `visitedExprs`) keyed
by structural equality of the Lean value. When a primitive is seen again, only its
existing index is returned — no new line is emitted. This means:

- The index tables are dense (no gaps).
- Each structurally-distinct primitive appears exactly once.
- The cache for expressions is pre-allocated for ~10 million entries.
- `letE` `nondep` is normalized to `false` (see section 5.7) to avoid spurious
  structural inequality between expressions that differ only in this optimization hint.

---

## 15. Complete JSON Key Reference Table

| Line type | Discriminator key | Index key | Payload keys |
|---|---|---|---|
| Name.str | `"str"` | `"in"` | `pre`, `str` |
| Name.num | `"num"` | `"in"` | `pre`, `i` |
| Level.succ | `"succ"` | `"il"` | (value is the integer directly) |
| Level.max | `"max"` | `"il"` | (value is [l1, l2] array) |
| Level.imax | `"imax"` | `"il"` | (value is [l1, l2] array) |
| Level.param | `"param"` | `"il"` | (value is name index integer) |
| Expr.bvar | `"bvar"` | `"ie"` | (value is de Bruijn index) |
| Expr.sort | `"sort"` | `"ie"` | (value is level index) |
| Expr.const | `"const"` | `"ie"` | `name`, `us` |
| Expr.app | `"app"` | `"ie"` | `fn`, `arg` |
| Expr.lam | `"lam"` | `"ie"` | `name`, `type`, `body`, `binderInfo` |
| Expr.forallE | `"forallE"` | `"ie"` | `name`, `type`, `body`, `binderInfo` |
| Expr.letE | `"letE"` | `"ie"` | `name`, `type`, `value`, `body`, `nondep` |
| Expr.proj | `"proj"` | `"ie"` | `typeName`, `idx`, `struct` |
| Expr.natVal | `"natVal"` | `"ie"` | (value is decimal string) |
| Expr.strVal | `"strVal"` | `"ie"` | (value is UTF-8 string) |
| Expr.mdata | `"mdata"` | `"ie"` | `expr`, `data` |
| Axiom | `"axiom"` | none | `name`, `levelParams`, `type`, `isUnsafe` |
| Definition | `"def"` | none | `name`, `levelParams`, `type`, `value`, `hints`, `safety`, `all` |
| Theorem | `"thm"` | none | `name`, `levelParams`, `type`, `value`, `all` |
| Opaque | `"opaque"` | none | `name`, `levelParams`, `type`, `value`, `isUnsafe`, `all` |
| Quotient | `"quot"` | none | `name`, `levelParams`, `type`, `kind` |
| Inductive group | `"inductive"` | none | `types`, `ctors`, `recs` |
| Metadata (first line) | `"meta"` | none | `exporter`, `lean`, `format` |

---

## 16. Notes on Key Ordering for the Reader

The reference parser (`Export/Parse.lean`, `parseItem` function) explicitly normalizes
key order before matching: any object where the second key is `"in"`, `"il"`, or `"ie"`
is reordered to put that key first. However, since JSON objects are unordered, a robust
reader MUST NOT rely on this ordering — use key lookup, not position.

```
match kv with
| [("in", .num idx), ("str", data)]   => ...   -- Name.str
| [("in", .num idx), ("num", data)]   => ...   -- Name.num
| [("il", .num idx), ("succ", data)]  => ...   -- Level.succ
...
| [("axiom", .obj data)] => ...
| [("def", .obj data)]   => ...
...
```

The pattern matching also shows that declaration lines have exactly one key (e.g.
`{"axiom": {...}}`), while primitive lines have exactly two keys (the type key and
the index key).

---

## 17. Fuzzing Targets for oxilean-export

Based on the spec, the following fuzz targets are required (per brief):

1. **Valid file fuzzing:** Ensure the parser never panics on any byte sequence.
2. **Truncated file:** Parser must handle EOF gracefully.
3. **Out-of-range index references:** e.g., `{"bvar": 0, "ie": 5}` where index 5 is
   used before being defined — must return an error, not panic.
4. **Large natVal strings:** Ensure bignum parsing does not overflow or panic.
5. **Invalid JSON:** Must return a parse error, not panic.
6. **Duplicate declaration names:** Must return an error (see `parseAxiomInfo` which
   calls `addConst` that checks for duplicates).
7. **Unknown declaration keys:** Must return a meaningful error (see `parseItem`
   final catch-all).

---

*End of consolidated specification.*
