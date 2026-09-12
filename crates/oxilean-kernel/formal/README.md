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

Measured 2026-09-08 with the release `cargo-formal` CLI and release
`formal-driver` (Phase 2b build, with dependency-body and monomorphic-instance
lowering), SMT backend OxiZ 0.3.3, nightly `2026-09-06` for step 3:

| build | result |
|---|---|
| `cargo build` | exit 0, 0 warnings |
| `cargo test` | 11 passed, 0 failed |
| `RUSTFLAGS="--cfg oxiformal_runtime_checks" cargo test` | 11 passed, 0 failed — **no harness is `#[should_panic]`**, because none of them panics under randomized draws |
| nightly `--cfg formal` check | exit 0, 0 warnings |
| `cargo formal check` | 86 VCs / 91 obligations, ~80 s from a fresh `--target-dir` |

## Measured verdict table

Per-harness, the worst verdict over the obligations of that harness.
`EXPECTED.toml` carries the same result property by property, with the
site-by-site breakdown.

| harness | target | measured |
|---|---|---|
| `from_limbs_normalizes_trailing_zeros_harness` | `from_limbs` | **unknown** — 3 `assert` (2 solver-model-rejected, 1 bounded), `unwinding-assertion` solver-model-rejected |
| `to_u64_matches_the_limb_count_harness` | `to_u64` | **unknown** — 3 `assert` solver-model-rejected, 2 `bounds-check` bounded |
| `to_u32_agrees_with_to_u64_harness` | `to_u32` | **unsupported(no-body)** — `BigNat::to_u32::{closure-0}` is not in the module set |
| `bit_length_bounds_harness` | `bit_length` | **unknown** — 3 `assert` obligations and 7 `arith-overflow` obligations (2 rows), solver-model-rejected or bounded |
| `beq_agrees_with_limb_equality_harness` | `beq` | **unknown** — 2 `assert` bounded, `unwinding-assertion` solver-model-rejected |
| `cmp_matches_lexicographic_order_harness` | `Ord::cmp` | **unsupported(aliasing)** — "a reference to a symbolically indexed element", `cmp_limbs` `a[i].cmp(&b[i])` |
| `sub_is_truncating_and_ble_agrees_harness` | `sub`, `ble` | **unsupported(unsupported-rvalue)** — "a reference has no field", `cmp_limbs` `a.len()` |
| `add_reaches_the_u128_carry_harness` | `add` | **unsupported(width)** — Rule W, `add_limbs`' `u128` carry |
| `shr_is_a_power_of_two_division_harness` | `shr` | **timeout** — 9 obligations hit the 30 000 ms budget; 2 `unwinding-assertion`s **proved**, the rest bounded |
| `bitwise_ops_respect_the_limb_count_lattice_harness` | `land`, `lor`, `lxor` | **unsupported(iterator)** — `(0..n).map(..).collect()` in `land` |
| `as_limbs_and_the_predicates_agree_harness` | `as_limbs`, `limb_count`, `is_zero`, `is_one` | **unknown** — 5 `assert` and 3 `bounds-check` bounded, `unwinding-assertion` solver-model-rejected |

`EXPECTED.toml` row tally: **42 rows over 11 harnesses — 0 proved / 0 refuted /
31 unknown / 6 timeout / 5 unsupported.**

## Layer counters (the run's own numbers)

```
package oxilean-kernel-formal 0.1.0        self_host: true
  functions              0   annotated 0    harnesses 11
  unsafe                 0   covered 0      uncovered 0
  hygiene            PASS
  bmc                 2 proved / 0 refuted / 75 unknown / 9 timeout /
                     5 unsupported(aliasing,iterator,no-body,unsupported-rvalue,width)
                     evidence  reproduction 86
  contract            0 proved / 0 refuted
  theorem            not run
  coverage           annotated/public 0.0%   unsafe covered 0.0%
  solver-model-rejected: 17
  dependency bodies  22 lowered (22 reachable)
  assumptions        MIR debug arithmetic; contract checks on; UB checks off;
                     heap invisible; unwind=8; seq bound=8
```

`coverage` reads 0.0 % because this package declares no public functions of its
own — it is all harnesses over a dependency, and coverage counts *home*
functions. The 22 lowered dependency bodies are the `BigNat` code under test.
The audit layer additionally reports 11 `unverifiable(raw-pointer)` items,
all inside lowered dependency bodies (`Vec`/slice internals), none in a
harness.

## Why nothing is `proved`: cargo-formal is pinned to OxiZ 0.3.3

`cargo-formal` depends on the SMT solver `oxiz` at a crates.io pin of `=0.3.3`,
which answers `sat` with a model that does **not** satisfy the formula for a
class of Boolean-structure-over-bit-vector queries (upstream item U-Z10).
cargo-formal runs a **mandatory model check** on every reported
counterexample, so such an answer becomes `unknown` and never a `refuted` with
a fabricated witness. This run reports

* **`solver-model-rejected: 17`**, and
* **58 further `bounded: unwind=8 reached`** obligations that are a
  *consequence* of those 17, not of the bound.

The second claim was measured, not assumed. Every `unwinding-assertion` for
`BigNat::from_limbs`'s `while limbs.last() == Some(&0)` normalisation loop
(`../src/bignat/mod.rs:78:15`) is one of the 17 model-rejected rows; because
"the loop finished" cannot be discharged, every obligation on the truncated
path is reported `bounded`. Re-running
`as_limbs_and_the_predicates_agree_harness` with `--unwind 16` produces
verdict-for-verdict identical output with only the number in the message
changed. Raising `unwind` cannot help here; an OxiZ fix can.

**Those rows move to `proved` (or to a real `refuted`) once cargo-formal's
`oxiz` pin moves off `=0.3.3` to a release carrying the U-Z10 fix.** Until
then this table should be read as "the solver did not decide it", not as "the
encoder cannot express it" and certainly not as "the kernel is wrong".

The 9 `timeout`s are a separate matter and are not the pin's fault: see the
`shr` entry in the gap list below.

All the numbers above are from a **cold** run (fresh `--target-dir`); every row
of `EXPECTED.toml` was checked mechanically against that run and none disagrees.
A **warm** re-run reports 76 unknown / 8 timeout and `solver-model-rejected: 18`,
because one borderline obligation of the `shr` harness finishes inside 30 s when
it is not competing with 85 other solves. No `EXPECTED.toml` row changes either
way.

## What this package names, and what it asks for

Five distinct things stand between these eleven statements and a checked kernel
bignum. Two are cargo-formal soundness rules, three are encoder gaps.

| # | thing | where | evidence |
|---|---|---|---|
| 1 | **Rule W** — no SMT term wider than 64 bits, because OxiZ 0.3.3/0.3.4 mis-encode wide `bvsub`/`bvneg` and can return a false `unsat` | `add_limbs`'s `u128` carry, `../src/bignat/mod.rs:459:17` | `add_reaches_the_u128_carry_harness` = `unsupported(width)`. Also covers `add_u64`, `succ`, `mul`, `mul_u64`, `div`, `rem`, `pow`, `checked_shl` |
| 2 | **U-Z10** — the pinned solver's wrong `sat` | everywhere | 17 `solver-model-rejected` + 58 dependent `bounded` rows |
| 3 | **a reference to a symbolically indexed element** | `cmp_limbs` `a[i].cmp(&b[i])`, `:493:15` | `cmp_matches_lexicographic_order_harness` = `unsupported(aliasing)`. Note the Phase 2b ordering fallback *does* resolve `<BigNat as Ord>::cmp` to the hand-written impl and inline it; the refusal is three frames deeper |
| 4 | **a `<=` on `&BigNat` arrives at the user impl one indirection too deep** | `cmp_limbs` `a.len()`, `:489:8`, reached through `<&A as PartialOrd<&A>>::le` | `sub_is_truncating_and_ble_agrees_harness` = `unsupported(unsupported-rvalue)`, "a reference has no field". That this is a *distinct* refusal from item 3, and not item 3 in disguise, was measured: hoisting a direct `a.cmp(&b)` above the `ble` call in that same harness moves the reason to `aliasing` at `:493:15`. The harness keeps `ble` first on purpose, so the package records both |
| 5 | **closures inside a dependency body are not lowered** | `to_u32`'s `and_then(\|n\| ..)`, `:115:9` | `to_u32_agrees_with_to_u64_harness` = `unsupported(no-body)` |
| 6 | **`map`/`collect`/`zip` are outside the iteration model** | `land` `:296:29`, `lor` `:307`, `lxor` `:321` | `bitwise_ops_respect_the_limb_count_lattice_harness` = `unsupported(iterator)` |
| 7 | **division by a power-of-two *constant* is bit-blasted in full** | `shr_bits` `let limb_shift = (bits / 64)` `:391` and `(bits % 64)` `:392` | the 9 `timeout`s of `shr_is_a_power_of_two_division_harness`. Strength-reducing `/ 64` and `% 64` to a shift and a mask is the concrete ask |

One further, unrelated encoder note, found while writing the harnesses: a
single-element `vec![x]` is refused (`unsupported-type: union
std::mem::MaybeUninit has no logical layout`), while `vec![x; n]` and
`Vec::new()` + `push` both work. `BigNat::one()` and every
`From<u64>`/`From<u32>`/`From<usize>` impl are written as `vec![n]`, so no
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
| `BigNat::to_u64` `:105` | `ensures(\|r\| r.is_some() == (self.limb_count() <= 1))` | stated by `to_u64_matches_the_limb_count_harness` |
| `BigNat::bit_length` `:119` | `ensures(\|r\| (*r == 0) == self.is_zero())` and `ensures(\|r\| self.is_zero() \|\| *r > 64 * (self.limb_count() as u64 - 1))` — the second is what makes `log2` (`:349`, `bit_length() - 1`) total | stated by `bit_length_bounds_harness` |
| `sub_limbs` `:472` | `requires(cmp_limbs(a, b) != Ordering::Less)` | an existing `debug_assert!` at `:473`; the post-loop `debug_assert_eq!(borrow, 0)` at `:483` is the matching `ensures` |
| `add_into` `:579` | `requires(shift + x.len() < acc.len())` | an existing `debug_assert!(idx < acc.len())` at `:584` and the doc comment "guaranteed by the Karatsuba caller" at `:578` — a precondition the code states in prose and checks only in debug |
| `div_rem_limbs_by_u64` `:594` | `requires(d != 0)` | existing `debug_assert!` at `:595` |
| `div_rem_knuth` `:612` | `requires(v_in.len() >= 2)`, `requires(v.len() == n)` | existing `debug_assert!`s at `:614`, `:619` |
| `shl_slice` `:675` / `shr_slice` `:692` | `requires(shift < 64)` | **unchecked**: the doc says "`shift < 64`" but nothing enforces it, and `64 - shift` underflows while `limb << shift` panics above it. Reachable only through `div_rem_knuth` `:617`, `:618`, `:670`, which does pass a value below 64 — so this is a documentation-grade contract, not a live bug |

Every one of these is reachable in principle; none is reachable from a harness
on the current pin, because the operators that lead to them (`mul`, `div`,
`rem`) go through the `u128` carries that Rule W refuses.

## Relationship to `cargo-formal`'s `examples/ecosystem/oxilean-bignat`

That package vendors three extracts of `bignat/mod.rs` into the cargo-formal
repository and harnesses the **private** limb helpers (`add_limbs`,
`sub_limbs`, `cmp_limbs`, `shl_slice`/`shr_slice`) directly. It stays there as
an in-repository regression fixture that needs no `oxilean` checkout. This
package is the public-API counterpart and is the one that lives in the
`oxilean` tree; it verifies the same code through the entry points a caller
actually has.
