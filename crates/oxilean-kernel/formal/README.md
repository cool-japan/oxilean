# `oxilean-kernel-formal`

Bounded model checking of `oxilean-kernel`'s public `BigNat` API with
[`cargo-formal`](https://github.com/cool-japan/cargo-formal).

This is a standalone package (its own `[workspace]`, `publish = false`) that
depends on `oxilean-kernel` by relative path and **changes nothing in it**.
It contains no copy of kernel code: `src/harness.rs` is an external
specification, and every call goes through `pub` items of
`oxilean_kernel::bignat::BigNat`.

## What is verified

`bignat` is, in its own module docs, "part of the trusted computing base:
deliberately small, self-contained, and audited by eye". It backs
`Literal::Nat`, so a carry, borrow or normalisation bug in it would be a wrong
*proof*, not a wrong number. The package therefore sets `self-host = true` in
`[package.metadata.formal]`, and the report says so.

Eleven harnesses state properties of `from_limbs`, `as_limbs`, `limb_count`,
`is_zero`, `is_one`, `to_u64`, `to_u32`, `bit_length`, `add`, `sub`, `beq`,
`ble`, `land`, `lor`, `lxor`, `shr` and `<BigNat as Ord>::cmp`. The limb-level
helpers (`add_limbs`, `sub_limbs`, `cmp_limbs`, `shr_bits`) are private and are
reached only *through* those entry points, which is the honest statement of
what a caller of the crate can rely on. Limb vectors are symbolic and hold at
most three limbs (`oxiformal::any_vec`), and `unwind` is 8.

## Running it

All four commands are run **from this directory**. `target/` and `Cargo.lock`
here are git-ignored.

```sh
# 1. plain, stable: type-check plus the concrete `plain_tests` witnesses
cargo build
cargo test

# 2. randomized execution: every harness becomes a #[test] over 256 draws
RUSTFLAGS="--cfg oxiformal_runtime_checks" cargo test

# 3. the `--cfg formal` type-check the driver build performs
RUSTFLAGS="--cfg formal -Zcrate-attr=feature(register_tool) -Zcrate-attr=register_tool(formal_tool)" \
  cargo +nightly-2026-06-20 check --target-dir target/formal-check

# 4. the verification itself
FORMAL_DRIVER=/path/to/formal-driver cargo formal check
```

Measured 2026-09-14 with the release `cargo-formal` 0.1.0 CLI rebuilt from the
current tree and the release `formal-driver` (dependency-body lowering (D1) and
on-demand monomorphic instance lowering (D2)), SMT backend OxiZ 0.3.3, rustc
`nightly-2026-06-20`, `--jobs 4`, a fresh `--target-dir`, machine otherwise
idle:

| build | result |
|---|---|
| `cargo build` | exit 0, 0 warnings |
| `cargo test` | 11 passed, 0 failed |
| `RUSTFLAGS="--cfg oxiformal_runtime_checks" cargo test` | 11 passed, 0 failed — **no harness is `#[should_panic]`**, because none of them panics under randomized draws |
| nightly `--cfg formal` check | exit 0, 0 warnings |
| `cargo clippy --all-targets -- -D warnings`, `cargo fmt -- --check` | exit 0 |
| `cargo formal check` | **188 VCs (0 cached) over 190 obligations, 615 s** from a fresh `--target-dir`, **exit 0** |

Exit 0 here means *nothing is refuted*. It does not mean everything is
decided: the sibling packages `oxiz-sat-formal` and `oxiarc-formal` exit 1
because they carry deliberate counterexamples, and this one carries none.

## Measured verdict table

Per-harness, the worst verdict over the obligations of that harness, under the
same worst-of-sites rule `EXPECTED.toml` uses
(`refuted` > `timeout` > `unknown` > `unsupported` > `proved`). `EXPECTED.toml`
carries the same result property by property, with the site-by-site breakdown.
`model-rejected` abbreviates an `unknown` whose message is
`solver-model-rejected`; `bounded` an `unknown` whose message is
`bounded: unwind=8 reached at …`.

| harness | target | worst | obligations |
|---|---|---|---|
| `from_limbs_normalizes_trailing_zeros_harness` | `from_limbs` | **unknown** | 4 — 3 `assert` (2 model-rejected, 1 bounded), 1 `unwinding-assertion` (model-rejected) |
| `to_u64_matches_the_limb_count_harness` | `to_u64` | **unknown** | 6 — 3 `assert` model-rejected, 2 `bounds-check` bounded, 1 `unwinding-assertion` model-rejected |
| `to_u32_agrees_with_to_u64_harness` | `to_u32` | **unknown** | 6 — 3 `assert` model-rejected, 2 `bounds-check` bounded, 1 `unwinding-assertion` model-rejected. *Was whole-harness `unsupported(no-body)` in wave 3* |
| `bit_length_bounds_harness` | `bit_length` | **unknown** | 11 — 3 `assert` model-rejected, 7 `arith-overflow` over 2 rows (4 model-rejected, 3 bounded), 1 `unwinding-assertion` model-rejected |
| `beq_agrees_with_limb_equality_harness` | `beq` | **unknown** | 3 — 2 `assert` bounded, 1 `unwinding-assertion` model-rejected |
| `cmp_matches_lexicographic_order_harness` | `Ord::cmp` | **timeout** | 40 over 11 rows — 32 unknown (all bounded, none model-rejected), 8 timeout. *Was whole-harness `unsupported(aliasing)` in wave 3* |
| `sub_is_truncating_and_ble_agrees_harness` | `sub`, `ble` | **timeout** | 57 over 19 rows — 55 timeout, 1 unknown (bounded), **1 proved**. *Was whole-harness `unsupported(unsupported-rvalue)` in wave 3* |
| `add_reaches_the_u128_carry_harness` | `add` | **unsupported(width)** | whole-harness refusal at encoding time, no VC minted — Rule W, `add_limbs`' `u128` carry |
| `shr_is_a_power_of_two_division_harness` | `shr` | **timeout** | 52 over 23 rows — 4 timeout, 46 unknown (4 model-rejected, 42 bounded), **2 proved** |
| `bitwise_ops_respect_the_limb_count_lattice_harness` | `land`, `lor`, `lxor` | **unsupported(iterator)** | whole-harness refusal at encoding time, no VC minted — `(0..n).map(..).collect()` in `land` |
| `as_limbs_and_the_predicates_agree_harness` | `as_limbs`, `limb_count`, `is_zero`, `is_one` | **unknown** | 9 — 5 `assert` bounded, 3 `bounds-check` bounded, 1 `unwinding-assertion` model-rejected |

`EXPECTED.toml` row tally: **72 rows over 11 harnesses — 0 proved / 0 refuted /
44 unknown / 26 timeout / 2 unsupported.** (The wave-3 file had 42 rows over
the same 11 harnesses; five of them were whole-harness `unsupported`.)

## Layer counters (the run's own numbers)

```
package oxilean-kernel-formal 0.1.0        self_host: true
  functions              0   annotated 0    harnesses 11
  unsafe                 0   covered 0      uncovered 0
  hygiene            PASS
  bmc                 3 proved / 0 refuted / 118 unknown / 67 timeout /
                     2 unsupported(iterator,width)
                     evidence  reproduction 188
  contract            0 proved / 0 refuted
  theorem            not run
  coverage           annotated/public 0.0%   unsafe covered 0.0%
  solver-model-rejected: 25
  dependency bodies  24 lowered (22 reachable)
  assumptions        MIR debug arithmetic; contract checks on; UB checks off;
                     heap invisible; unwind=8; seq bound=8
```

(`cargo formal status` re-reads the same `report.json` and quotes its `bmc` and
`contract` counters back on a `last check` row. The two rows that name file
paths are omitted above because they point into a scratch directory.)

`coverage` reads 0.0 % because this package declares no public functions of its
own — it is all harnesses over a dependency, and coverage counts *home*
functions. The **24 lowered dependency bodies (22 reachable)** are the `BigNat`
code under test; 0 monomorphic instances were needed, because nothing in
`bignat`'s harnessed surface is generic. The 188 `reproduction` records are one
per VC: the two whole-harness `unsupported` refusals mint no `vc/NNNN.smt2`.

The wave-3 README recorded "11 `unverifiable(raw-pointer)` items, all inside
lowered dependency bodies". **That is now zero** — `unverifiable_by_reason` is
empty in both `inventory.json` and `report.json`. Phase 2b stopped the
inventory's classifying type walk before the private fields of the containers
the encoder models as values, so `Vec<u64>` → `RawVec` → `Unique<u64>` →
`*const u64` no longer marks every `Vec`-holding function unverifiable.

## Load sensitivity: the `unknown`/`timeout` split is not a stable number

The boundary between `unknown` and `timeout` here is a **wall-clock** boundary,
set by `[package.metadata.formal] budget`'s 30 000 ms, and it moves with machine
load. The same source, the same pins, three runs:

| run | proved | refuted | unknown | timeout | unsupported | wall |
|---|---|---|---|---|---|---|
| 2026-09-09, quiet (Gate 5) | 3 | **0** | 124 | 61 | 2 | 556 s |
| 2026-09-14, under concurrent load | 3 | **0** | 101 | 84 | 2 | 758 s |
| 2026-09-14, quiet (the run this file records) | 3 | **0** | 118 | 67 | 2 | 615 s |
| 2026-09-15, Phase 3 defaults (autoharness off, modular on, vacuity check on), moderate concurrent load (uptime 1-min load ~13-14 of 8 cores: 13.79 before run1, 12.98 before run2; other D-wave/OxiZ/oxifunnel `cargo`/`nextest` jobs running) | 3 | **0** | 159 | 26 | 2 | 411 s |
| 2026-09-15, same binaries, same load class, run 2 | 3 | **0** | 147 | 38 | 2 | 451 s |

All five are 190 obligations. The movement is **asymmetric and bounded**: an
obligation can only trade `unknown` for `timeout` and back, never for `proved`
or `refuted`, because `bounded`/`solver-model-rejected` is a real answer from
the solver and a `timeout` is the absence of one — neither decides the
property. **A re-run that reports a different split is reproducing this
package, not regressing it.**

The 2026-09-15 Phase 3 re-measurement (release `cargo-formal` 0.1.0 rev
`0621fc0`, OxiZ 0.3.3, rustc nightly-2026-06-20, `--jobs 4`) reproduces this
file under the same asymmetry, run twice from fresh `--target-dir`s:
`refuted` stays 0, both whole-harness `unsupported` rows
(`add_reaches_the_u128_carry_harness` width, `bitwise_ops_respect_the_limb_count_lattice_harness`
iterator) are unchanged, and the three `proved` obligations are the same
three sites (`shr_is_a_power_of_two_division_harness#unwinding-assertion` at
`../src/bignat/mod.rs:405:13` and `src/harness.rs:363:5`,
`sub_is_truncating_and_ble_agrees_harness#unwinding-assertion` at
`../src/bignat/mod.rs:480:9`). `compare_expected.py --undecided-equivalent`
against `EXPECTED.toml` exits 0 for both runs; `--prev` between them reports
0 `FLIP`s and 12 `UNDECIDED-SWAP`s (unknown⇄timeout only), and against the
2026-09-14 quiet report 0 `FLIP`s, 0 added/removed ids, and 41
`UNDECIDED-SWAP`s. The machine was **not** quiet for this run (unlike the
2026-09-09/2026-09-14 rows): other agents' `cargo`/`nextest` jobs were
running throughout on the shared 8-core host, which plausibly explains why
this pair of runs is both faster (411 s / 451 s) and more `unknown`-heavy
than the quiet 2026-09-14 row — contention appears to make OxiZ give up
(`bounded`/`solver-model-rejected`) before the 30 000 ms wall clock, not
after it, shifting the split without changing any decided verdict.

What is stable, and what is not:

* **Stable — `refuted` is 0, in every run.** That is the claim this package
  exists to make.
* **Stable — the two `unsupported` harnesses.** Both are refused at encoding
  time (`time_ms` 0), before any budget applies.
* **Stable — the three `proved` obligations** (115 ms, 196 ms, 236 ms), three
  orders of magnitude inside the budget.
* **Stable — `sub_is_truncating_and_ble_agrees_harness`'s 1 proved / 55 timeout
  / 1 unknown**, identical in all three runs.
* **Not stable — any individual `unknown` vs `timeout` row**, and therefore not
  the per-harness worst verdict of `cmp_matches_lexicographic_order_harness`
  and `shr_is_a_power_of_two_division_harness` either. Both sit close enough to
  the budget that a quieter or busier machine can move them.

`EXPECTED.toml`'s header names the rows nearest the boundary in each direction,
with the solve times this run measured, so a reader can see which would move
first.

## Why there is so little `proved`

Three obligations are proved and **no row** is, and those are different
statements: all three proved obligations share a row with a site that times
out, so the worst-of-sites rule records their rows as `timeout`. The column is
0 by construction, not because nothing was decided.

Of the 187 obligations that are not proved, 2 are the whole-harness encoder
refusals below. The other 185 — the ones a solver was actually asked about —
are **not** encoder refusals. They split into exactly two causes:

**1. The `=0.3.3` pin (25 obligations, plus 18 of the 93 `bounded` ones).**
`cargo-formal` depends on the SMT solver `oxiz` at a crates.io pin of `=0.3.3`,
which answers `sat` with a model that does **not** satisfy the formula for a
class of Boolean-structure-over-bit-vector queries (upstream item U-Z10).
cargo-formal runs a **mandatory model check** on every reported counterexample,
so such an answer becomes `unknown` and never a `refuted` with a fabricated
witness. This run reports `solver-model-rejected: 25`.

A `bounded: unwind=8 reached` verdict means the encoder could not conclude that
some loop finished. The message names the loop, and the 93 split three ways —
which matters, because only the first group is downstream of the 25:

| loop named in the message | count | that loop's own `unwinding-assertion` |
|---|---|---|
| `:78:15`, `from_limbs`'s `while limbs.last() == Some(&0)` | 18 | `solver-model-rejected` wherever it is not itself a timeout |
| `:398:18`, `shr_bits`'s limb loop | 42 | `timeout` |
| `:492:14`, `cmp_limbs`'s reverse walk | 33 | `timeout` |

For the first group the link was measured on **2026-09-08**, by re-running
`as_limbs_and_the_predicates_agree_harness` (all of whose `bounded` obligations
name `:78:15`) at `--unwind 16`: verdict-for-verdict identical output, only the
number in the message changed. Raising `unwind` cannot help there; an OxiZ fix
can. That experiment has not been repeated on the current binaries, and it
never covered the other 75: the two harnesses that raise them did not encode at
all in wave 3, and their loops are held up by the budget rather than by the
pin.

U-Z10 is fixed in the `../oxiz` 0.3.4 working tree, which is **unreleased**;
`cargo-formal` keeps `oxiz = "=0.3.3"` and keeps Rule W in force until a
release carrying the fix passes `crates/formal-conformance`. These rows are
*expected* to move when the pin moves — an expectation, not a measurement: no
0.3.4 run of this package exists.

**2. Solver capacity (67 `timeout`s, plus the other 75 `bounded` ones).**
These are not the pin's fault and not an encoder refusal. They are concentrated in
the three harnesses that walk two symbolic limb vectors through `cmp_limbs`'s
reverse loop (`../src/bignat/mod.rs:492:14`) or `sub_limbs`'s borrow loop
(`:476:14`): 55 in `sub_is_truncating_and_ble_agrees_harness`, 8 in
`cmp_matches_lexicographic_order_harness`, 4 in
`shr_is_a_power_of_two_division_harness`. Eight-fold unrolling of a loop whose
body is a 64-bit comparison or an `overflowing_sub` pair, under a path
condition that already carries `from_limbs`'s own unrolled normalisation loop,
is simply a large bit-blasting problem. The 75 `bounded` obligations rooted at
`shr_bits`'s and `cmp_limbs`'s loops belong to this cause too: the loop whose
completion they wait on is one whose `unwinding-assertion` timed out.

So the honest reading of this table is **"the solver did not decide it"** —
never "the encoder cannot express it", and certainly never "the kernel is
wrong". Since wave 3 the encoder column has shrunk from five whole-harness
refusals to two: nine of the eleven harnesses now encode completely, where six
did on 2026-09-08.

### What changed since wave 3 (2026-09-08)

Three harnesses moved from whole-harness `unsupported` to fully encoding, which
is why the run grew from 86 VCs to 188 and `EXPECTED.toml` from 42 rows to 72.
All three are cargo-formal Phase 2b changes; none is a change to
`oxilean-kernel`.

| harness | was | what closed it |
|---|---|---|
| `to_u32_agrees_with_to_u64_harness` | `unsupported(no-body)`, "`BigNat::to_u32::{closure-0}` is not in the module set" | **P2-13** lowers closures a dependency only ever passes as a value, hooked at the single place a closure path is minted, so `to_u64().and_then(\|n\| ..)` no longer reports `no-body` |
| `cmp_matches_lexicographic_order_harness` | `unsupported(aliasing)`, "a reference to a symbolically indexed element", at `cmp_limbs`'s `a[i].cmp(&b[i])` (`:493:15`) | the encoder builtin **shared `&xs[i]` at a symbolic index resolving to the selected element's value** |
| `sub_is_truncating_and_ble_agrees_harness` | `unsupported(unsupported-rvalue)`, "a reference has no field", at `cmp_limbs`'s `a.len()` (`:489:8`) | the **ordering fallback to a user `PartialOrd`/`Ord` impl**, wired into `lt`/`le`/`gt`/`ge` as well as `cmp`, `partial_cmp`, `min`/`max` and `clamp` — `ble`'s `self <= rhs` on `&BigNat` now resolves through `<&A as PartialOrd<&A>>::le` to the hand-written impl |

`shr_is_a_power_of_two_division_harness` encoded in wave 3 already, but with 9
timeouts; it has 4 here, and three of its rows moved `timeout` → `unknown`. The
encoder gained **strength reduction of `/` and `%` by a constant power of two**
in Phase 2b, which is precisely the ask the wave-3 file recorded for
`shr_bits`'s `bits / 64` and `bits % 64` (`../src/bignat/mod.rs:391`, `:392`).
Two caveats on reading that as a clean win: the `division-by-zero` and
`remainder-by-zero` obligations at those sites are **still raised** (both
`bounded`), because MIR inserts them regardless of how the operation is
encoded; and the load-sensitivity section above applies to the row movement
too — the evidence is a wall clock, not an SMT term count.

A by-product worth recording: `sub_is_truncating_and_ble_agrees_harness` now
reaches `sub_limbs`' own internal debug assertions and raises them as
obligations — `debug_assert!(cmp_limbs(a, b) != Ordering::Less)` at
`../src/bignat/mod.rs:473` (key `panic`, raised at
`core/src/macros/mod.rs:290:13`) and `debug_assert_eq!(borrow, 0)` at `:483`
(key `assert`, raised at `core/src/macros/mod.rs:51:21`). Those are exactly the
two contracts the in-source-candidate table below proposes for `sub_limbs`.
The package now *raises* them instead of merely proposing them; both time out
on this run, so neither is proved yet.

## Self-host highlight: the false refutation this package found

This is the point of `self-host = true`, and it is the package's most valuable
single result so far.

On **2026-09-09** this package reported a `refuted` `bounds-check` on
`BigNat::from_limbs`, with the counterexample `in0 = vec![]` — a claim that the
proof kernel's bignum constructor panics on the empty limb vector. A runtime
check of the same input does not reproduce it, and the kernel is not wrong: the
**verifier** was.

The root cause was in cargo-formal's `formal-vcgen`, not in `oxilean-kernel`.
Resolving a reference through a value tree (`resolve_referents`) resolved
*every* enum variant's payload and *every* sequence slot under the caller's
path condition, un-narrowed. `slice::last` on a constant-length empty vector
produces a dead `Some` payload holding a reference at offset `u64::MAX`; that
dead payload raised a `bounds-check` which folds to `false` under a reachable
guard, and `from_limbs`'s `while limbs.last() == Some(&0)` is exactly the shape
that reaches it.

The fix narrows the guard rather than dropping the payload: each enum payload
is now resolved under `guard ∧ discr = d` and each sequence slot under
`guard ∧ k < len`. Every payload and slot is still resolved, so the value tree
keeps its shape and only the guard the obligations are raised under changes.
Note the precise claim: the obligation is **removed, not proved** — cargo-formal
stops asserting something it had no business asserting. The same class of
over-broad guard was hardened in `contains` and `starts_with`/`ends_with`, and
six regression tests pin the behaviour in cargo-formal's
`crates/formal-vcgen/tests/symex_dead_variant.rs` (three pairs: the
`bounds-check` is not raised, *and* the opposite claim is still refuted, for
`first` on an empty vector, `last` after push-and-pop, and a live variant).

**The payoff:** `refuted` is **0** in all three runs of the load-sensitivity
table above, including the two on 2026-09-14. The false refutation stays gone,
and the package that produced it is the one that keeps checking.

A verifier applied to a proof kernel is worth exactly as much as its lowest
false-positive rate, and this is how that rate gets measured: point it at code
whose correctness you can also establish by other means, and treat every
`refuted` as a claim about the verifier until it survives a runtime witness.

## What this package still names, and what it asks for

Four distinct things now stand between these eleven statements and a checked
kernel bignum — down from the wave-3 list of seven, three of which Phase 2b
closed (see the table above). Two are cargo-formal soundness rules, one is an
encoder gap, one is solver capacity.

| # | thing | where | evidence |
|---|---|---|---|
| 1 | **Rule W** — no SMT term wider than 64 bits, because OxiZ 0.3.3 mis-encodes wide `bvsub`/`bvneg` and can return a false `unsat`, the one wrong answer no model check can catch | `add_limbs`'s `u128` carry, `../src/bignat/mod.rs:459:17` | `add_reaches_the_u128_carry_harness` = `unsupported(width)`. Also covers `add_u64`, `succ`, `mul`, `mul_u64`, `div`, `rem`, `pow`, `checked_shl` |
| 2 | **U-Z10** — the pinned solver's wrong `sat`, turned into `unknown` by the mandatory model check | everywhere | 25 `solver-model-rejected` obligations, plus most of the 93 `bounded` ones downstream of them |
| 3 | **`map`/`collect`/`zip` are outside the iteration model** (cargo-formal TODO P3-23) | `land` `:296:29`, `lor` `:308`, `lxor` `:322` | `bitwise_ops_respect_the_limb_count_lattice_harness` = `unsupported(iterator)`; the harness aborts at `land`'s `map`, so that is the recorded reason |
| 4 | **Solver capacity on an eight-fold unrolled two-vector comparison or subtraction** | `cmp_limbs` `:492:14`, `sub_limbs` `:476:14` | the 67 `timeout`s, 55 of them in the `sub`/`ble` harness. Not a refusal and not the pin: the conditions are built, sent and simply not answered inside 30 000 ms |

One further, unrelated encoder note, found while writing the harnesses: a
single-element `vec![x]` is refused (`unsupported-type: union
std::mem::MaybeUninit has no logical layout`), while `vec![x; n]` and
`Vec::new()` + `push` both work. `BigNat::one()` and every
`From<u64>`/`From<u32>`/`From<usize>` impl are written as `vec![x]`, so no
harness here constructs a `BigNat` through them.

## In-source contract candidates

The honest end state is `#[oxiformal::requires]` / `#[ensures]` on the real
functions in `crates/oxilean-kernel/src/bignat/mod.rs`, with harnesses beside
them, which needs `oxiformal` published to crates.io. Until then, these are the
contracts this trial would put in the source. **Only real findings**, each with
its file:line and its evidence; none of them is a defect report.

| function | proposed contract | evidence |
|---|---|---|
| `BigNat::from_limbs` `mod.rs:77` | `ensures(\|r\| r.as_limbs().last() != Some(&0))` — the struct invariant documented at `:59-60` ("no trailing zero limbs; zero is the empty vector"), which every constructor and operator returning `Self` also owes | stated by `from_limbs_normalizes_trailing_zeros_harness`; measured `unknown` on this pin |
| `BigNat::to_u64` `:105` | `ensures(\|r\| r.is_some() == (self.limb_count() <= 1))` | stated by `to_u64_matches_the_limb_count_harness`; measured `unknown` (3 `assert` sites, all model-rejected) |
| `BigNat::bit_length` `:119` | `ensures(\|r\| (*r == 0) == self.is_zero())` and `ensures(\|r\| self.is_zero() \|\| *r > 64 * (self.limb_count() as u64 - 1))` — the second is what makes `log2` (`:355`, `bit_length() - 1`) total | stated by `bit_length_bounds_harness`; measured `unknown` |
| `sub_limbs` `:472` | `requires(cmp_limbs(a, b) != Ordering::Less)`, and `ensures` the post-loop `borrow == 0` | **now raised as obligations**, not just proposed: the existing `debug_assert!` at `:473` is the `panic` row and the `debug_assert_eq!(borrow, 0)` at `:483` is the fourth `assert` site of `sub_is_truncating_and_ble_agrees_harness`. Both `timeout` on this run |
| `add_into` `:579` | `requires(shift + x.len() < acc.len())` | an existing `debug_assert!(idx < acc.len())` at `:584` and the doc comment "guaranteed by the Karatsuba caller" at `:578` — a precondition the code states in prose and checks only in debug |
| `div_rem_limbs_by_u64` `:594` | `requires(d != 0)` | existing `debug_assert!` at `:595` |
| `div_rem_knuth` `:612` | `requires(v_in.len() >= 2)`, `requires(v.len() == n)` | existing `debug_assert!`s at `:614`, `:619` |
| `shl_slice` `:675` / `shr_slice` `:692` | `requires(shift < 64)` | **unchecked**: the doc says "`shift < 64`" but nothing enforces it, and `64 - shift` underflows while `limb << shift` panics above it. Reachable only through `div_rem_knuth` `:617`, `:618`, `:670`, which does pass a value below 64 — so this is a documentation-grade contract, not a live bug |

All but the `sub_limbs` pair remain out of reach from a harness on the current
pin, because the operators that lead to them (`mul`, `div`, `rem`) go through
the `u128` carries that Rule W refuses.

## Relationship to `cargo-formal`'s `examples/ecosystem/oxilean-bignat`

That package vendors three extracts of `bignat/mod.rs` into the cargo-formal
repository and harnesses the **private** limb helpers (`add_limbs`,
`sub_limbs`, `cmp_limbs`, `shl_slice`/`shr_slice`) directly. It stays there as
an in-repository regression fixture that needs no `oxilean` checkout. This
package is the public-API counterpart and is the one that lives in the
`oxilean` tree; it verifies the same code through the entry points a caller
actually has.

The two are complementary rather than redundant. Measured 2026-09-14, the
vendored fixture reports 46 proved / 0 refuted / 29 unknown / 0 timeout /
1 unsupported over 76 items; this package reports 3 proved / 0 refuted /
118 unknown / 67 timeout / 2 unsupported over 190. The fixture's four accessor
harnesses go through `from_limbs` just as these do, but its four limb-helper
harnesses call `add_limbs`, `sub_limbs`, `cmp_limbs` and the shift helpers with
slices directly, which is where its `proved` column comes from — a shape this
package cannot use, because those helpers are private and a caller of
`oxilean-kernel` cannot reach them. Neither table subsumes the other.
