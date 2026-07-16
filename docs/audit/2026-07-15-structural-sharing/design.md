# Structural sharing for the oxilean kernel `Expr`

Status: design accepted (synthesis of a 3-design panel; unanimous winner).
Date: 2026-07-15. Branch base: `0.1.3`. Author: design synthesizer.
Scope: `oxilean-kernel`, `oxilean-export`, with knock-on validation in `oxilean-verify` / `oxilean-verify-wasm`.

**Winner: Design 1 — staged `Box<Expr>` → `Rc<Expr>`.** All three judges selected it,
including the soundness-and-determinism–weighted judge (which the tie-break rule ranks
highest). The plan below is Design 1's staged migration, grafting the throughput and
memory levers the two losing designs contributed where they add no new soundness risk,
and correcting the factual errors the judges flagged.

---

## 1. Motivation (the numbers)

The kernel `Expr` is a deep `Box` tree
(`crates/oxilean-kernel/src/expr/types.rs:1023-1061`) whose hand-written `Clone`
charges one fuel unit per node deep-copied
(`crates/oxilean-kernel/src/expr/types.rs:1062-1082`). It carries derived deep
`PartialEq`/`Eq`/`Hash` (`expr/types.rs:1023`) and has **zero structural sharing**.

Two consequences drive both of the project's walls:

- **Materialization destroys DAG sharing.** The export reader's `build_expr`
  (`crates/oxilean-export/src/reader.rs:558-643`) does an iterative post-order
  expansion that `Box::new`s a fresh node per tree position
  (`reader.rs:613/618/623/629/633`). An export id referenced by *N* parents is
  re-expanded *N* times. Full Lean-core Init (lean4export NDJSON v3.1, 330 MB,
  57,277 decls) thereby explodes into **908,550,041 kernel nodes**. Streaming
  read peak is 3.79 GiB RSS.
- **The Environment holds those unshared trees.** A full Init verify peaks at
  **10.4 GiB RSS + ~6.7 GiB swap** on this 14 GiB box, wall 1:20:44,
  ~11.8 decls/s — memory-bound (swap thrash). lean4lean checks the same corpus
  ~8.2× faster; the 5× throughput budget is **not** met. Root cause of both: the
  unshared `Box` `Expr`.

Headline that must not regress (current Init, corpus preset):
**35,223 verified / 22,054 unsupported(named) / 0 rejected.** Unsupported roots:
175 clone-fuel-limited, 2 oversized, 1 nested-inductive (`Lean.Syntax`, cascading
to ~14k followers).

Driving goal **M4**: verify the Mathlib corpus (~300k decls, export likely
2–10 GB NDJSON) inside the 12 GiB systemd cage on this 14 GiB machine. If Init's
~150× node blow-up roughly holds, materializing Mathlib unshared is on the order
of ~10¹⁰–10¹¹ nodes — infeasible. With sharing, materialization becomes
~O(#export ids), not O(tree size), and M4 becomes reachable.

> **Corpus counts — SETTLED by direct NDJSON measurement (2026-07-15).** The
> lean4export shared-DAG format emits one `ie` record per *distinct* expr node,
> one `in` per distinct name, one `il` per distinct level. Counting them settles
> the panel's 6.1M-vs-45M dispute with no kernel change — **both estimates were
> wrong.** Ground truth:
>
> | distinct nodes | Init | Mathlib |
> |---|---|---|
> | exprs (`ie`)  | **552,915**   | **8,199,591** |
> | names (`in`)  | 294,826       | 4,088,786     |
> | levels (`il`) | 575           | 42,760        |
>
> Init materializes to 908,550,041 tree nodes today from just 552,915 distinct
> exprs → a **~1,643× file-global sharing factor**. Consequence: a *file-global*
> shared representation of the **entire Mathlib environment** is ≈ 8.2M exprs ×
> ~80 B (Stage-A `Rc` node) ≈ **660 MB** for exprs + ≈ 4.1M names × ~64 B ≈
> **260 MB** — comfortably inside the 12 GiB cage. The memory blocker is real for
> the *unshared tree* (Mathlib would be ~10¹⁰ tree nodes) and fully dissolved by
> sharing. Design 1's earlier 16–32 GiB pessimism rested on the wrong 45M figure
> and does **not** apply. Budgets are still recalibrated empirically (§4.3), but
> M4 feasibility is no longer in question.

---

## 2. Chosen representation (Rust sketch)

The migration lands in two shapes. **Stage A** is the least-invasive swap that is
behaviour-preserving in isolation; **Stage E** is the throughput endpoint (a
cached-header newtype grafted from Design 3). Everything between (B–D) is
incremental and green.

### Stage A — swap the child pointer, keep the enum

```rust
use std::rc::Rc;

#[derive(PartialEq, Eq, Hash, Debug, Clone)] // Clone now DERIVED (was hand-written)
pub enum Expr {
    Sort(Level),
    BVar(u32),
    FVar(FVarId),
    Const(Name, Vec<Level>),
    App(Rc<Expr>, Rc<Expr>),
    Lam(BinderInfo, Name, Rc<Expr>, Rc<Expr>),
    Pi (BinderInfo, Name, Rc<Expr>, Rc<Expr>),
    Let(Name, Rc<Expr>, Rc<Expr>, Rc<Expr>),
    Lit(Literal),
    Proj(Name, u32, Rc<Expr>),
}

// The single metering + construction chokepoint (replaces `Box::new` for a child):
#[inline]
fn kid(e: Expr) -> Rc<Expr> {
    crate::fuel::charge(1); // charge at construction (see §4)
    Rc::new(e)
}

impl Expr {
    pub fn app(f: Expr, a: Expr) -> Expr { Expr::App(kid(f), kid(a)) }
    pub fn lam(bi: BinderInfo, n: Name, t: Expr, b: Expr) -> Expr { Expr::Lam(bi, n, kid(t), kid(b)) }
    // pi / let_ / proj analogous
}

// Iterative Drop — REQUIRED. A deep shared term dropped by its last owner would
// otherwise recurse and overflow the 512 MiB verify stack (and the wasm stack).
impl Drop for Expr {
    fn drop(&mut self) {
        // Worklist: for each child, take sole ownership if we are the last owner
        // (Rc::try_unwrap) and push its inner Expr to be dropped iteratively.
        // See §4 note on MSRV: use Rc::try_unwrap, not Rc::into_inner (1.70 floor).
    }
}
```

Why Stage A is a behaviour no-op:

- **Pattern matches are unchanged.** `match e { Expr::App(f, a) => .. }` binds
  `f, a` as `&Rc<Expr>` which `Deref`-coerces to `&Expr` for every `&Expr`-taking
  call — e.g. the recursion in `reduce/types.rs` and `subst/functions.rs` compiles
  untouched. Verified against the mirror recursion pattern at
  `subst/functions.rs:768-777`.
- **Equality/hash semantics are identical.** `std::rc::Rc<T>` delegates
  `PartialEq`/`Eq`/`Hash` deeply to the pointee, and (since `T: Eq`) short-circuits
  on `ptr_eq` — so the derives on `expr/types.rs:1023` keep exactly today's meaning
  *and* gain a free, sound ptr-eq fast path. The 21 `Expr`-keyed maps and the def-eq
  degradation machinery are untouched, so **rejected stays 0.**

Measured layout (standalone mirror; `Name`=32 B, `Level`=32 B, `Literal`=32 B):
Box node = 64 B; **Rc node (Stage A) = 80 B** (16 B strong+weak counts + 64 B enum);
handle in parent = 8 B (same as `Box`). Stage A **alone is a ~25% memory
regression** — it must never run the corpus without Stage B's sharing.

### Stage E — cached-header newtype (throughput endpoint, grafted from Design 3)

```rust
pub struct Expr(std::rc::Rc<ExprData>);

struct ExprData {
    hash: u64,               // cached structural hash        → O(1) Hash
    size: u32,               // cached node count             → O(1) expr_size_within
    flags: u32,              // looseBVarRange:30 | has_fvar:1 | has_mvar:1
    kind: ExprKind,
}

enum ExprKind {
    Sort(Level), BVar(u32), FVar(FVarId), Const(Name, Vec<Level>),
    App(Expr, Expr),
    Lam(BinderInfo, Name, Expr, Expr),
    Pi (BinderInfo, Name, Expr, Expr),
    Let(Name, Expr, Expr, Expr),
    Lit(Literal), Proj(Name, u32, Expr),
}

impl std::hash::Hash for Expr {                    // O(1): read the cached hash
    fn hash<H: std::hash::Hasher>(&self, h: &mut H) { self.0.hash.hash(h) }
}
impl PartialEq for Expr {                          // ptr-eq shortcut, else structural
    fn eq(&self, o: &Self) -> bool {
        Rc::ptr_eq(&self.0, &o.0)
            || (self.0.hash == o.0.hash && self.0.size == o.0.size
                && structural_eq(&self.0.kind, &o.0.kind))
    }
}
```

`Eq` stays **structural** (ptr-eq and the cached hash are only shortcuts, never the
definition of equality). `looseBVarRange` (1 + max loose de Bruijn index; 0 = closed)
is computed once at construction from the children's cached headers, and lets
`instantiate`/`lift` at depth *d* return the child `Rc` unchanged whenever
`range <= d` — this is the mechanism that makes substitution O(#affected) instead
of O(tree). Measured: `ExprData` 80 B, `Rc<ExprData>` heap block 96 B, handle 8 B.

Stage E's cost is that `Expr::App(f, a)` is no longer a bare variant, so construction
sites and pattern arms go through helpers (`Expr::app(..)` and a `kind()`/view
accessor). Stage 0 (§6) pays that churn up front on the *current* Box enum so Stage E
is a small, semantically-focused diff.

---

## 3. Where sharing arises (reader + kernel)

Sharing has **two independent sources**; the memory win rests entirely on the first,
so it lands first and does not depend on the second.

### 3.1 Reader materialization — the memory win (Stage B)

Today `build_expr` (`reader.rs:558`) re-expands each DAG id per parent. The change is
a memo parallel to the `enodes` table (`reader.rs:332`):

```rust
// New field on Reader (reader.rs:328) — or scratch, per materialize_expr call:
memo: Vec<Option<Expr>>,   // len == enodes.len(); id -> its unique Rc handle

// build_expr Enter(i): if let Some(e) = &self.memo[i] { out.push(e.clone()); continue }
// build_expr Exit(i):  let e = ...construct...; self.memo[i] = Some(e.clone()); out.push(e);
```

`e.clone()` is now an O(1) refcount bump, so a DAG node referenced *N* times yields
**one** `Rc`. Materialization becomes O(#distinct reachable ids). The
`materialize_expr` budget (`reader.rs:534`), currently charged as precomputed tree
size `esize` (`reader.rs:448`), is re-expressed as *distinct nodes allocated*; the
C22 per-decl cap (`reader.rs:541`) and `CORPUS_DECL_MATERIALIZE_BUDGET`
(`reader.rs:75`) are recalibrated in the same units.

**Memo scope** is a policy knob:

- *Per-decl-scoped* (reset at each `materialize_expr`): residency = one decl's
  distinct-node DAG; the **Mathlib-safe default**.
- *File-global* (persist across decls): maximal cross-decl sharing, residency
  comparable to today's ENode table.

Both capture the dominant within-decl sharing (a decl's type reappearing in its
value, repeated recursor motives, shared library constants) that produces the ~150×
factor. Default to per-decl-scoped for M4; a two-tier arena (§8 graft) can add a
persistent env tier later if measurement wants cross-decl sharing.

### 3.2 Substitution preservation + construction interning — the throughput win

- **Substitution preservation (Stage D).** The C16b builders — `subst/functions.rs`
  `instantiate_at`/`abstract_at`/`instantiate_many_at`/`parallel_subst`/`shift_bvars`,
  the `instantiate/` builders, and `expr_util::lift_loose_bvars_aux` — currently
  rebuild every node. Under `Rc` they return `Rc::clone` of an *untouched* subtree.
  Detect "untouched" cheaply two ways: (a) a per-pass `HashMap<*const Expr, Expr>`
  memo keyed by child pointer identity (works in Stage A, needs no metadata), or
  (b) cached `looseBVarRange` (Stage E) giving an O(1) `range <= depth` test.
- **Construction-time interning (Stage G, optional, grafted from Design 3).** A
  thread-local `HashMap<u64, Vec<Weak<ExprData>>>` consulted by the smart
  constructors deduplicates nodes *created during reduction* (whnf / instantiate /
  iota results), collapsing intermediate blow-ups (the `let`-tower,
  `Int.add_mul_ediv_right`) and yielding ptr-identity def-eq. It is **feature-gated
  and observationally transparent**: it only ever returns a node structurally equal
  to the request, and it never influences fuel (§4) — so verdicts and fuel are
  byte-identical with it on or off (the differential gate in §7 enforces this).

The unused `hash_cons/` `Idx<Expr>` arena is deliberately **not** used: threading
`Idx<Expr>` through every signature is a far larger API break than `Rc`. If Stage G's
interner (or any dedup map) is ever added, its dedup structure is an **index-only
open-addressing set** (a `Vec` of handles whose equality probes deref into the node
store), not a `HashMap<full-node, handle>` — the latter duplicates node contents and
is what makes the existing `hash_cons::HashConsArena` cache
(`crates/oxilean-kernel/src/hash_cons/…`) memory-wasteful at Mathlib scale (graft
from Design 2).

---

## 4. Fuel metering v2

### 4.1 What replaces `clone == work`

Today the meter is clone-count: `Expr::clone` charges 1/node deep-copied
(`expr/types.rs:1066`), and the C16b builders additionally charge 1/node *visited*
(documented at `fuel.rs:10-18`). Under `Rc`, `Expr::clone` becomes an O(1) refcount
bump — charging 1 for it would undercount a whole shared subtree as a single unit,
so **clone stops being the meter.** The replacement is a single charge at node
**construction**, inside `kid()` (§2): the count equals the number of `Rc<Expr>`
allocations the declaration performs — its true per-decl heap-allocation cost.

Concretely:

- Remove the `charge(1)` from `Expr::clone` (`expr/types.rs:1066`); `Clone` becomes
  derived.
- Remove the per-visit `charge(1)` at the top of the 10 subst/instantiate/lift
  builders — it is *subsumed* by construction charging: in the rebuild case, one
  construction == one old visit; in the newly-shared case, no construction ==
  correctly cheaper.
- Add the one `charge(1)` in `kid()`, and the **same** charge in the reader's
  `build_expr` per allocated node — unifying the meter across materialization and
  reduction.

The `fuel.rs` latch/degradation machinery is **untouched byte-for-byte** — only the
charge *sites* move. `set_budget`/`charge`/`is_exhausted`/`used` (`fuel.rs:57-97`)
are unchanged, so exhaustion still latches (`fuel.rs:74-80`) and the conservative
degradation still fires: `whnf` returns its input, `is_def_eq` decides syntactic-only
(the `fuel::is_exhausted()` guard at `def_eq/types.rs:1353`), `infer_type` aborts
typed. **Rejected stays 0.**

### 4.2 Determinism argument

The allocation count depends **only** on the reduction path (which redexes fire, in
what order) and the DAG structure — never on wall-clock, `Rc` addresses, allocator
state, or `HashMap` iteration order. The two dedup structures added by this work — the
id-keyed reader memo (a plain `Vec<Option<Expr>>`) and the ptr-keyed per-pass subst
memo — **dedup work only; they never decide an output.** The *set* of distinct nodes
constructed is path-determined, so `used()` is bit-identical across runs, across
allocator states, and (for Stage G) with the weak interner on or off. Fuel is charged
per constructor *call* (a hit and a miss both charge 1), so interner/eviction timing
is invisible to the meter.

One disclosed behaviour change: cached `looseBVarRange` (Stage E) lets substitution
skip untouched subtrees, which **lowers** visit counts versus today. This is a
correctness-preserving optimization Lean itself uses; the counts shift downward but
stay deterministic.

### 4.3 Budget recalibration plan

`DEFAULT_DECL_FUEL = 1 << 24` and `CORPUS_DECL_FUEL = 1 << 26`
(`crates/oxilean-export/src/replay.rs:712/720`) were derived from **clone** counts
(deep visits), calibrated against `Int.add_mul_ediv_right` (~26M nodes, 2.5×
headroom). Construction-count for the same decl is strictly ≤ old clone-count
(sharing skips reconstructing untouched/duplicate subtrees), so the constants must be
re-derived, not reused. Plan:

1. Land the new meter (Stages A–B).
2. Instrument `Reader::enodes.len()` at EOF to fix the true Init distinct-node count
   (settle the 6.1M-vs-45M question) before touching any constant.
3. Re-run the existing calibration scan **in the 12 GiB cage** under the new
   construction-meter; record the new max construction-count among
   legitimately-checkable decls; set `CORPUS_DECL_FUEL ≈ 2.5×` that.
4. Same for `CORPUS_DECL_MATERIALIZE_BUDGET` (`reader.rs:75`), re-expressed in
   distinct-nodes (which shrinks the 2 oversized decls, whose distinct-node count is
   far below their tree size).
5. **Gate:** verified non-decreasing AND rejected == 0 on full Init before the new
   constants merge.

### 4.4 Expected effect on the 175 clone-fuel roots

These exhausted because reduction rebuilt > 2²⁶ nodes, largely by re-cloning shared
subterms. Under structure-sharing substitution + memoized materialization their
construction-counts drop, often far below 2²⁶, so many move **unsupported →
verified** (the 175 shrinks; verified rises). Each shift is explainable as *decl X's
construction count fell from Y to Z < budget*. Because memory is now bounded by
*distinct* nodes, the budget can be raised affordably to admit the genuinely huge ones
(TODO_VERIFY notes `UInt64.toUInt32_mul` ~100.2M and `Array.foldlM_toList.aux`
~506.8M clone-nodes); any that still exceed budget remain **named-unsupported, never
rejected.**

---

## 5. ptr-equality fast paths

`Rc::ptr_eq` is a **sound refinement** of structural equality: the same immutable
`Rc` is trivially identical content, so no def-eq verdict changes and verified cannot
decrease. Hot paths that gain a ptr-identity early-out (Stage C, then sharpened by
Stage E's cached hash):

- `Expr::eq` — `Rc::ptr_eq` first (free in Stage A via `Rc`'s own `Eq`; explicit in
  Stage E).
- `DefEqChecker::is_def_eq` entry `if t == s` (`def_eq/types.rs:1345`) → O(1) when the
  terms are the same shared `Rc` (common after sharing).
- The def-eq cache `HashMap<(Expr, Expr), bool>` (`def_eq/types.rs:1267`) and the
  `EquivManager` (`def_eq/types.rs:1268`, `is_equiv`/`is_failure` at 1372/1375):
  probes short-circuit on ptr-equal keys.
- The whnf memo `HashMap<Expr, Expr>` (`reduce/types.rs:939`): same.
- `has_loose_bvars` / `expr_size_within` (used at `def_eq/types.rs:1368-1370`):
  memoized on node identity in Stage C, then O(1) header reads in Stage E.

**Cache-guard removal (grafted from Design 2).** Once hashing is O(1) (Stage E's
cached hash), the `expr_size_within(_, 4096)` guards that currently *refuse* to cache
any term over 4096 nodes — `WHNF_CACHE_MAX_NODES` (`reduce/types.rs:928`, applied at
`reduce/types.rs:1036/1150`) and `DEF_EQ_CACHE_MAX_NODES` (`def_eq/types.rs:1298`,
applied at `def_eq/types.rs:1369-1370`) — are removed for full-coverage caching.
Those guards exist *only* because deep hash-key retention turned heavy decls into
multi-GiB peaks (the code comments at `reduce/types.rs:1147-1149` and
`def_eq/types.rs:1363-1366` say exactly this); with O(1) cached hashes and shared
keys, full caching is affordable and directly helps the 175 clone-fuel roots that
today recompute giant intermediates on every visit.

---

## 6. Staged implementation plan

Each stage lands **green** (full workspace test suite + differential-vs-lean4lean).
The load-bearing memory win is Stages 0→A→B; C/D/E/F/G are incremental throughput and
Mathlib-margin levers.

**Stage 0 — mechanical normalization (green, no semantics).** On the *current* Box
enum, add smart constructors `Expr::sort/bvar/fvar/const/app/lam/pi/let_/proj/lit`
and a `kind()` accessor; codemod the 2,091 `Box::new` construction sites
(`grep Box::new crates/oxilean-kernel/src` count) and every destructuring match to go
through them. Pure syntactic churn; isolates the giant mechanical diff from any
semantic change so Stages A and E are small.
Files: `expr/types.rs`, plus every construction/match site (67 kernel files) — but
behaviour-preserving, guarded by the 33,231 tests.

**Stage A — representation swap, behaviour-preserving (green).** `Box<Expr>` →
`Rc<Expr>` at the **8 type sites, spread across three files** (correcting Design 2's
"8, all in `expr/types.rs`" error): `expr/types.rs:1047/1049/1054/1056/1060` (App,
Lam, Pi, Let, Proj), the `WhnfHead` mirror enum at `whnf/types.rs:1017/1019` (Lam,
Pi), and the `peel_lambdas` return type `Vec<(Name, BinderInfo, Box<Expr>)>` at
`subst/functions.rs:766`. Derive `Clone`; delete hand-written `Clone`
(`expr/types.rs:1062-1082`); add the iterative `Drop`. Convert the ~88
move-out-of-`Box` sites (`infer/types.rs:1178` `Ok(*dom)`, `:1356` `current = *cod`,
`:971`, `:1281`, `:1285`, and the by-value recursion in `normalize/functions.rs` /
`context/functions.rs`). Move the fuel meter from `Expr::clone` to `kid()`; remove
the 10 per-visit subst charges. **Do not run the corpus** (80 B > 64 B, no sharing =
regression). Recalibrate fuel with a small caged scan.
Files: `expr/types.rs`, `fuel.rs`, `whnf/types.rs`, `subst/functions.rs`,
`instantiate/functions.rs`, `expr_util/functions.rs`, `infer/types.rs`,
`normalize/functions.rs`, `context/functions.rs`.

> **MSRV correction (Judge 3's flaw).** The workspace floor is
> `rust-version = "1.70"` (`Cargo.toml:31`). `Rc::unwrap_or_clone` (stable 1.76) and
> `Rc::into_inner` (1.70 — actually fine) must **not** silently raise the floor; use
> `Rc::try_unwrap(rc).unwrap_or_else(|rc| (*rc).clone())` (stable since 1.4) for the
> move-out and iterative-Drop conversions, or bump the declared MSRV deliberately in
> a separate, reviewed change.

**Stage B — reader memoized materialization (green; THE memory win).** Add
`memo: Vec<Option<Expr>>` to `Reader` (`reader.rs:328`); short-circuit Enter /
insert-on-Exit in `build_expr` (`reader.rs:571-639`); re-express the
`materialize_expr` budget (`reader.rs:534`) and `CORPUS_DECL_MATERIALIZE_BUDGET`
(`reader.rs:75`) in distinct-nodes. **Caged full-Init run** gated on: peak RSS drop
(target ~5 GiB, no swap), rejected == 0, verified non-decreasing, oversized ≤ 2.
Files: `reader.rs`, `replay.rs`.

**Stage C — ptr_eq fast paths (green; throughput).** Explicit `Expr::eq` with
`Rc::ptr_eq` first; ptr-identity early-outs at `def_eq/types.rs:1345`, the def-eq
cache/`EquivManager`, and the whnf memo (`reduce/types.rs:939`); memoize
`has_loose_bvars`/`expr_size_within` on node identity. Property test: ptr_eq-true ⇒
structural-eq-true, and explicit `eq` agrees with the old derived `eq` on random
terms.
Files: `expr/types.rs`, `def_eq/types.rs`, `reduce/types.rs`, `expr_util/functions.rs`.

**Stage D — structure-sharing substitution (green; throughput + fewer nodes).**
subst/instantiate/lift return `Rc::clone` of untouched subtrees via a per-pass
ptr-memo. Recalibrate fuel again. Determinism test: same decl + budget → identical
`used()` across two runs.
Files: `subst/functions.rs`, `instantiate/functions.rs`, `expr_util/functions.rs`.

**Stage E — cached-header newtype (grafted from Design 3; closes the throughput
gap).** `Expr(Rc<ExprData>)` with cached `hash` + `size` + `looseBVarRange` +
`has_fvar`; O(1) `Hash`/`PartialEq`(ptr-eq shortcut)/`expr_size_within`/
`has_loose_bvars`; loose-range subst short-circuit. **Remove** the
`expr_size_within(_, 4096)` cache guards (`reduce/types.rs:928/1036/1150`,
`def_eq/types.rs:1298/1369-1370`) for full-coverage caching. Lands behind the Stage-0
accessor so pattern-match churn is already paid.
Files: `expr/types.rs`, `def_eq/types.rs`, `reduce/types.rs`, `expr_util/functions.rs`.

**Stage F — Name/Level interning (grafted from Design 2; REQUIRED for Mathlib
margin).** Design 1 left `Const(Name, Vec<Level>)` deep-copying on clone; at Mathlib
scale (a claimed 4.09M distinct names) that is the difference between an ~8 GiB and an
~3 GiB persistent floor. Intern `Name`/`Level` (e.g. `Rc<Name>`/`NameId`) behind an
**index-only open-addressing dedup set** (never `HashMap<full-node, id>`). Gate on
Mathlib memory margin; Init already passes at Stage E, so this is Mathlib-only and can
be scoped independently. This is the M4 lever, not optional.
Files: `expr/types.rs`, `name/…`, `level/…`, `reader.rs`.

**Stage G — OPTIONAL: weak interner for reduction intermediates (grafted from Design
3).** Thread-local `Weak<ExprData>` interner consulted by the smart constructors +
ptr-identity def-eq; amortized dead-`Weak` sweep + size cap with plain-alloc
fallback; feature-gated. Two-tier residency (persistent env tier + per-decl ephemeral
tier cleared at decl boundaries, mirroring `Reducer::clear_cache`) if a full Mathlib
pass shows residue growth. Gate: **identical verdicts AND identical fuel with the
interner on vs off** (§7).
Files: `expr/types.rs`, `def_eq/types.rs`, `reduce/types.rs`.

---

## 7. Verification plan

**Unit / property (per stage):**
- Stage A: all 3,439 kernel + 33,231 workspace tests green (behaviour-identical).
- Stage A: deep-`let`-tower Drop test (no stack overflow on last-owner drop of a
  deep shared term).
- Stage C: `ptr_eq(a,b) ⇒ a == b` and `explicit_eq == old_derived_eq` on random terms.
- Stage D/G: **two-run determinism** — same decl + same budget ⇒ identical `used()`.
- Stage E: `cached_hash == recomputed_structural_hash` and
  `cached_size == recomputed_size` on random terms (so no `HashMap` invariant breaks).

**Differential determinism gate (grafted from Design 3; cross-cutting CI):** for any
dedup structure the winner adds (reader memo, subst ptr-memo, weak interner), a build
that runs the optimization **ON** and one that runs it **OFF** must produce
**byte-identical `used()` fuel AND identical per-decl verdicts** on a fixed decl set.
This is the strongest available guarantee that dedup never leaks into results or fuel.

**Full-Init caged rerun** (`systemd-run` `MemoryMax=12G`, per machine-memory
discipline) after Stages B, and again after E/F:
- Peak RSS target ≤ ~5.5 GiB, **zero swap** (the 6.7 GiB swap must be eliminated).
- Throughput measured in the non-thrashing regime for the first time.

**Number-shift acceptance criteria (hard merge gates, every stage):**
1. **rejected == 0** — non-negotiable.
2. **verified ≥ 35,223** — verified may only rise or hold.
3. **Every numeric shift explained.** The verified set must be a **superset** of the
   old verified set (diff the verdict ledger); each unsupported→verified move
   attributed to a named decl whose construction-count fell below budget; the
   oversized bucket stays ≤ 2 (Stage B may shrink it to 0 as those decls'
   distinct-node counts fall under the re-expressed materialize budget); the
   nested-inductive `Lean.Syntax` root (and its ~14k followers) is unaffected by this
   refactor.

**wasm gate:** rebuild `wasm32-unknown-unknown` after Stages A, E, F; confirm gzip
stays under the 400 KB budget (144 KB today) and the "Kernel in a Tab" demo runs.

---

## 8. Risks & rollback

Each stage is independently green and independently revertable (git revert of the
stage's commits restores the prior representation; no stage is a point of no return
until its merge gate passes). Principal risks:

1. **Memory sharing without time sharing.** Naive `Rc` (Stages A–B) shares memory but
   leaves derived `Hash`/`Eq`, `has_loose_bvars`, `expr_size_within` O(tree) on a DAG,
   and the def-eq/whnf caches hash keys deep. *"Memory improved, throughput flat"* is
   the **expected** Stage-B-only state, not a failure. Mitigation: Stages C
   (ptr-fast-paths) and E (cached header) are the throughput closers; sequence them.
2. **Iterative `Drop` correctness.** A wrong worklist could leak or double-drop.
   Mitigation: land it *in* Stage A behind the deep-`let`-tower test; use
   `Rc::try_unwrap` (MSRV-safe) to take sole ownership before recursing.
3. **Fuel recalibration flips verdicts.** A mis-set constant could move
   currently-verified decls to unsupported (or admit ones that then hit a genuine
   wall). Mitigation: re-run the calibration scan under the new meter *before* fixing
   constants; gate every stage on verified-non-decreasing AND rejected == 0; treat any
   verified regression as a release blocker.
4. **Stage A alone is a ~25% memory regression** (80 B vs 64 B, no sharing).
   Mitigation: never ship or corpus-run Stage A alone; fold A+B into one release, or
   land B immediately after A.
5. **Determinism leak** (HashMap order / `Rc` addresses reaching results or fuel).
   Mitigation: charge at construction (set-determined), never at visit; ptr-keyed
   memos dedup work only; the ON/OFF differential gate (§7) in CI.
6. **The ~88 move-out sites are not mechanical.** A wrong conversion could
   double-charge fuel or change sharing. Mitigation: convert each with the
   `try_unwrap`-or-clone idiom, review individually, backstop with a fuel-count
   snapshot test on a fixed decl.
7. **Mathlib residency.** Holding a file-global DAG as `Rc` may not fit 14 GiB if the
   corpus is large. Mitigation: default to per-decl-scoped reader memo (residency =
   largest decl); Stage F interning shrinks the node; the two-tier arena bounds a full
   pass; flag the reader's id-index table as the remaining M4 scaling wall
   (pre-existing, independent of this refactor). **`Rc` auto-frees zero-refcount
   subtrees**, so — unlike a monotonic arena — reduction residue does not accumulate
   over a 300k-decl pass (this is the concrete reason to prefer `Rc` over Design 2's
   arena for M4).
8. **ptr-eq/hash soundness.** Must stay a refinement of structural eq and consistent
   with `Hash`. Mitigation: same immutable `Rc` ⇒ identical content (trivially sound);
   property-test explicit-eq == derived-eq and cached-hash == structural-hash.

---

## 9. Rejected alternatives

**Design 2 — thread-local hash-consing arena, `Expr` becomes a `Copy` `ExprId(u32)`
handle into a `RefCell<Vec<ExprNode>>`.** Best pure memory (24 B interned nodes;
claimed ~3.25 GB persistent Mathlib) and strong immediate throughput (id-eq/hash are
O(1) on the flip; removes the 4096-node cache guard). Rejected as the *primary* design
on three grounds the soundness-and-determinism lens weights heaviest. (a) It makes
**id-equality the semantic `PartialEq`/`Hash`**, so the 0-rejected record depends on a
single global invariant — dedup-key *completeness*; if any field (`BinderInfo`, binder
`Name`, `FVarId`) is ever omitted, two distinct terms collapse to one id and `a == b`
returns a **wrong def-eq accept**. Design 1's ptr-eq is safe tautologically; Design 2's
id-eq is safe only if the interner is *proven* complete — a fundamentally larger
soundness surface. (b) It is the **largest, riskiest diff**: `Expr` becomes an opaque
`Copy` handle, so ~5,768 variant sites and *every* match must migrate to `view()`, plus
966 `&Expr`→`Expr` and re-keyed caches — a single atomic representation flip of the core
type on a 148k-LOC kernel, the biggest possible break of the "API mostly untouched"
constraint. (c) The monotonic arena never frees, needing a per-decl ephemeral tier to
bound Mathlib residue, and the `RefCell` introduces a runtime double-borrow panic
surface. Its best ideas are grafted: Name/Level interning (Stage F), index-only dedup
set, and 4096-guard removal.

**Design 3 — newtype `Expr(Rc<ExprData>)` with a 16 B cached header + reader DAG memo
+ thread-local weak interner, from Stage 1.** Best throughput architecture (cached
hash/size/looseBVarRange make `Hash`/eq/guards O(1) without a `RefCell` hot-loop tax,
matching lean4lean's representation; `Eq` stays structural — cleaner than Design 2's
id-eq). Rejected as the *from-scratch* design because it front-loads risk that Design 1
defers: (a) it commits the soundness-critical cached-`looseBVarRange` substitution
short-circuit into the **core representation at Stage 1** — a wrong cached range silently
skips a needed instantiate/lift and could flip a verdict — whereas Design 1 keeps the
metadata-free ptr-memo as the Stage-D default and defers the cached header to optional
Stage E; (b) its nodes are the **fattest (96–112 B)** and it does not intern
`Name`/`Level`, so at the competing measured ~101.67M Mathlib node count its persistent
floor is ~11.4 GB — at/over the 12 GiB cage before the reader index and reduction
transients, so its central M4 promise is not established by its own arithmetic; (c) it
forces rewriting every `match e {…}` to `match e.kind() {…}` from the start. Its
throughput machinery *is* Design 1's endpoint, so it is grafted wholesale: the cached
header (Stage E), the weak interner (Stage G), and the ON/OFF differential determinism
gate (§7). (Note: Design 3's claim to "reuse `DefaultHasher::new()` at
`whnf/types.rs:1283`" mischaracterized that code — it is a `format!("{:?}")`,
string-allocating O(tree) hash — so the cached header **replaces** it, it does not reuse
it.)
