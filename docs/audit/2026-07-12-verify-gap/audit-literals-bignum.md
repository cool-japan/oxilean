# Audit: Nat/String literal reduction & arbitrary-precision arithmetic in oxilean-kernel

Repo: oxilean (branch 0.1.3, clean). Audit date 2026-07-12.
Brief section: (3) "Nat/String literal reduction with a hand-written zero-dependency
arbitrary-precision bignum (Nat.add/mul/sub/div/mod/pow/gcd/beq/ble/bitwise directly on
literals)", TCB hygiene (zero deps, forbid(unsafe), fuzzed, differential tests).

TL;DR verdict: **Partially present, with soundness bugs.** Literal reduction machinery
exists and is wired into the environment-aware WHNF used by def_eq, and most Lean nat-op
names are special-cased — but the representation is a fixed-width `u64` (not a bignum),
add/mul/pow/succ/shiftLeft silently wrap or panic on overflow, several ops deviate from
Lean semantics (String.length in bytes, pow exponent truncated to u32, shiftLeft drops
high bits), the NatLit ↔ `Nat.zero`/`Nat.succ` constructor bridge required by the Lean
kernel is completely missing (so `0 =?= Nat.zero` is *not* defeq and `Nat.rec` on a
literal is stuck), nat-op arguments are not WHNF'd before the extension fires (nested
arithmetic does not fold), there are 4–5 duplicated evaluator copies that disagree on
mod-by-zero, and there is **no arbitrary-precision integer implementation anywhere in the
workspace** other than a data-only limb container in the runtime. There is no fuzzing, no
differential testing, and no overflow test.

---

## 1. Representation of literals

### 1.1 The type

`crates/oxilean-kernel/src/expr/types.rs:831-885`:

```rust
/// Literal values supported natively.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum Literal {
    /// Natural number literal.
    Nat(u64),
    /// Integer literal (signed).
    Int(i64),
    /// String literal.
    Str(String),
}
```

- Wired into the expression grammar at `expr/types.rs:1032` (`Expr::Lit(Literal)`).
- Accessors: `as_nat() -> Option<u64>` (line 855), `as_int() -> Option<i64>` (863),
  `as_str() -> Option<&str>` (871), `type_name()` (879).
- `Display` impl in `expr/literal_traits.rs`.

**Findings:**
- `Nat` is a plain `u64`. There is **no** bignum, no u128, no unary fallback, no
  overflow error path.
- `Int(i64)` is a **deviation from Lean 4** — the Lean kernel has exactly two literal
  kinds (`natVal`, `strVal`). An Int literal in the kernel widens the TCB and is
  implemented with *wrapping* semantics (see §2.4), which is mathematically wrong for
  Lean's arbitrary-precision `Int`.

### 1.2 Overflow behavior

The reduction code uses **unchecked** `+`, `*`, `u64::pow`:

- `reduce/functions.rs:74` `Nat.succ`: `Literal::Nat(n + 1)`
- `reduce/functions.rs:86` `Nat.add`: `Literal::Nat(m + n)`
- `reduce/functions.rs:93` `Nat.mul`: `Literal::Nat(m * n)`
- `reduce/functions.rs:127` `Nat.pow`: `Literal::Nat(m.pow(*n as u32))`

Neither the workspace `[profile.release]` (root `Cargo.toml`, lines ~60-64: lto,
codegen-units, opt-level, strip — **no `overflow-checks = true`**) nor any crate profile
enables overflow checks. Therefore:

- **Debug/test builds: panic** on `Nat.add u64::MAX 1` (denial of service inside the
  "kernel", the checker aborts instead of reporting a verdict).
- **Release builds: silent two's-complement wrap.** This is a **soundness hole**: in a
  release-built verifier, `Nat.mul (2^32) (2^32)` whnf-reduces to `NatLit 0`, so the
  kernel would accept a proof of `2^32 * 2^32 = 0`, i.e. a proof of `False` is derivable
  by an adversarial export file using only literal arithmetic. This directly violates the
  brief's "rejected = alarm" bucket — the wrong bucket (verified) is produced.

`Literal::Nat` construction sites elsewhere confirm no guard anywhere (grep for
`checked_add|checked_mul` in the nat paths: none).

---

## 2. Which operations are special-cased, where, and semantic correctness

### 2.1 The wired path: `try_reduce_nat_app`

`crates/oxilean-kernel/src/reduce/functions.rs:64-224`
(`pub(super) fn try_reduce_nat_app(head: &Expr, args: &[Expr]) -> Option<Expr>`).

Called from the environment-aware WHNF `Reducer::whnf_core`
(`reduce/types.rs:1129-1134`), which is used by `Reducer::whnf_env`
(`reduce/types.rs:1069`) and hence by `DefEqChecker::is_def_eq_core`
(`def_eq/types.rs:1315-1317`) and the `TypeChecker` (`infer/types.rs:868-869,1142`).
So this is the real kernel-relevant path.

Per-op table (line numbers in reduce/functions.rs; Lean 4 kernel reference semantics in
brackets):

| Op | Lines | Implementation | Correct vs Lean? |
|---|---|---|---|
| `Nat.zero` (head) | 69-71 | any application headed by `Nat.zero` → `Lit 0`, args discarded | Only reachable ill-typed (`Nat.zero x`); **bare `Const Nat.zero` never becomes `Lit 0`** (Const case at reduce/types.rs:1097-1111 only delta-unfolds definitions with values; constructors have none) |
| `Nat.succ` | 72-76 | `n + 1` if arg is syntactically `Lit` | overflow: panic/wrap (§1.2); does NOT fire on `Nat.succ Nat.zero` or nested succ (arg not a Lit) |
| `Nat.pred` | 77-81 | `saturating_sub(1)` | semantics OK (pred 0 = 0); not a Lean kernel extension but harmless if consistent with the exported defn |
| `Nat.add` | 82-88 | `m + n` | **overflow unsound** (§1.2) |
| `Nat.mul` | 89-95 | `m * n` | **overflow unsound** |
| `Nat.sub` | 96-102 | `m.saturating_sub(*n)` | correct truncated subtraction (within u64) |
| `Nat.div` | 103-112 | `n != 0 ? m / n : 0` | correct (Lean: floor division, `x / 0 = 0`) |
| `Nat.mod` | 113-122 | `n != 0 ? m % n : m` | correct (Lean: `x % 0 = x`) |
| `Nat.pow` | 123-129 | `m.pow(*n as u32)` | **wrong twice**: exponent truncated `as u32` (e.g. `n = 2^32` → exponent 0 → result 1); `u64::pow` panics (debug) / wraps (release) on overflow |
| `Nat.beq` | 130-136 | `nat_bool_result(m == n)` | correct; see §6 |
| `Nat.ble` | 137-143 | `m <= n` | correct |
| `Nat.blt` | 144-149 | `m < n` | correct semantics; extension beyond the official Lean kernel op set (Lean reduces blt through its definition) — safe if consistent |
| `Nat.gcd` | 150-157 | Euclid via helper `gcd` (313-321) | correct within u64 (`gcd(0,b)=b`, `gcd(a,0)=a` both handled by loop) |
| `Nat.land` | 158-164 | `m & n` | correct within u64 |
| `Nat.lor` | 165-171 | `m \| n` | correct within u64 |
| `Nat.xor` | 172-178 | `m ^ n` | correct within u64 (Lean name `Nat.xor` matches) |
| `Nat.shiftLeft` | 179-187 | `n < 64`: `m << n`; `n >= 64`: **stuck** (falls through, returns None) | **unsound for n<64** when high bits exist: Lean `(2^63) <<< 1 = 2^64`, here → `0` (silently drops bits, no overflow check on `<<`); incomplete for n ≥ 64 (Lean value is huge; here stuck — safe-incomplete) |
| `Nat.shiftRight` | 188-197 | `n < 64`: `m >> n`; else `0` | correct for u64-representable m |
| `Nat.log2` | — | **MISSING** (grep `log2` in kernel: zero hits) | brief requires it |
| `String.length` | 198-202 | `s.len() as u64` (UTF-8 **bytes**) | **wrong**: Lean `String.length` counts chars (Unicode scalar values). `"é".length = 1` in Lean, `2` here → false equalities acceptable, e.g. kernel would accept `"é".length = 2` and reject `= 1` |
| `String.append` | 203-211 | Rust `push_str` concat | correct |
| `String.beq` | 212-218 | byte equality → Bool | semantics fine (UTF-8 byte equality = char-sequence equality); note Lean's kernel does not special-case `String.beq`; the actually needed `String.decEq` (Decidable, not Bool) is **missing** |

Also missing entirely: `Nat.decEq` in the wired path (only in a dead helper, §2.5),
`Nat.repr`, `String.toNat`/`Nat` parsing bridges, all `Char` handling, and the
StrLit ↔ `String.mk (List.cons (Char.ofNat …) …)` unfolding the Lean kernel performs so
that string literals can be consumed by recursors / decEq.

### 2.2 Argument WHNF gap (incompleteness)

`whnf_core` (`reduce/types.rs:1113-1136`) collects the spine args **raw**
(`get_app_args(expr).cloned()`), whnfs only the head, then calls
`try_reduce_nat_app(&head_whnf, &args)`. The extension only matches when args are
*syntactically* `Expr::Lit`. Consequences:

- `Nat.add (Nat.add 1 2) 3` never folds to `6`; def_eq against `Lit 6` then reaches
  `try_lazy_delta` (`def_eq/types.rs:1362-1400`), which finds `Nat.add` is an axiom with
  no value (`builtin/functions.rs:355-393` registers all nat binops and cmps as
  `ConstantInfo::Axiom`) → `(None, None) => false`. **Nested literal arithmetic is not
  defeq-complete.** The Lean kernel whnfs the operands before applying the extension.
- Same for `Nat.succ (Nat.succ (Lit 3))` vs `Lit 5` → false.
- Caveat: in a *real exported environment*, `Nat.add` has a definition (structural
  recursion via `Nat.rec` after equation compilation … actually via `Nat.rec` at kernel
  level), so lazy delta could in principle grind it out by unary unfolding — but only if
  `Nat.rec` can consume literals, which it cannot (§2.3). So still incomplete.

### 2.3 Missing NatLit ↔ constructor bridge (major)

The Lean 4 kernel treats `NatLit 0` defeq `Nat.zero` and `NatLit (n+1)` defeq
`Nat.succ (NatLit n)`, and expands literals to constructor form when a recursor consumes
them. In oxilean-kernel:

- No such expansion exists anywhere. Greps for constructor-expansion patterns
  (`Nat.succ` built from `Literal::Nat(n-1)`, `to_ctor`, `expand_lit` …) return nothing.
- `try_reduce_recursor` (`reduce/types.rs:1157-1185`) requires the major premise's whnf
  head to be `Expr::Const` naming a constructor (`env.is_constructor`). An
  `Expr::Lit(Nat)` major premise → `return None` → **`Nat.rec` applied to any literal is
  stuck**. Every proof that does induction/cases on a numeral will fail (presumably
  landing in the wrong verdict bucket — "rejected" instead of "verified").
- def_eq: `(Lit, Const/App)` pairs fall to `_ => try_lazy_delta` (`def_eq/types.rs:1357`);
  constructors have no value → false. So `0 ≠defeq Nat.zero`, `1 ≠defeq Nat.succ 0`.
- Same for strings: no StrLit → `String.mk [...]` unfolding, so any exported lemma that
  pattern-matches a string literal is stuck.

### 2.4 The Int literal extension (deviation, unsound if triggered)

`try_reduce_int_app`, `reduce/functions.rs:226-317`, also wired into whnf_core
(`reduce/types.rs:1132-1134`):

- `Int.ofNat (Lit n)` → `Int(*n as i64)` (line 256): **n > i64::MAX wraps negative**.
- `Int.negSucc (Lit n)` → `-(n as i64) - 1` (263-268): same wrap.
- `Int.neg/add/mul/sub` use `wrapping_neg/wrapping_add/wrapping_mul/wrapping_sub`
  (276-303): deliberate silent wrap — mathematically wrong for Lean `Int` and a
  soundness hole analogous to §1.2 (here even in debug builds, since wrapping_* never
  panics).
- `Int.ble/blt/beq` (305-315) correct on the (wrapped) i64s.
- Lean's kernel has no Int literals at all; exports encode integers via
  `Int.ofNat`/`Int.negSucc` over Nat literals. This whole extension should be deleted
  from a minimal-TCB verifier.

### 2.5 Duplicated, disagreeing evaluator copies (drift hazard)

Four additional literal evaluators exist besides the wired one:

1. `simp/functions.rs:178-215` `eval_nat` (used by `fold_nat_constants`, line 724):
   - `Nat.mod`: `checked_rem(rhs).unwrap_or(0)` → **`x % 0 = 0`, WRONG** (Lean: `x`),
     and **inconsistent with the wired path** which returns `m`.
   - `Nat.pow`: `saturating_pow(rhs as u32)` → saturates at u64::MAX (wrong, differently
     wrong from the wired path's wrapping).
   - `Nat.add`/`Nat.mul`: unchecked overflow.
   - extras: `Nat.min`, `Nat.max`, `Nat.pred`, `Nat.zero`.
2. `simp/functions.rs:49-60` `try_simplify_nat_op`: only `Nat.succ`, unchecked `n + 1`.
3. `reduce/functions.rs:400-415` `reduce_nat_op` ("legacy API", pub): succ/add/mul
   unchecked, sub saturating.
4. `reduce/functions.rs:706-755` `eval_nat_binop` (`#[allow(dead_code)]`, pub):
   mod-by-zero **correct** (`lhs`), pow saturating (wrong),
   `Nat.shiftLeft`: `checked_shl(rhs as u32).unwrap_or(0)` → **`m <<< 64` evaluates to
   `0`** (Lean: `m * 2^64`) — actively wrong result, not just stuck; extra `Nat.lcm`.
5. `reduce/functions.rs:757-766` `eval_nat_cmp` (dead): maps `Nat.decEq` to
   `"true"/"false"` strings — note `Nat.decEq` must produce
   `Decidable.isTrue/isFalse` *with a proof argument*, not a Bool; if this helper were
   ever wired as-is it would be ill-typed.

The brief's demand for ONE hand-written bignum module with exhaustive differential tests
is precisely the cure for this drift (the two mod-by-zero behaviors already disagree).

### 2.6 Non-env reduction paths do not fold literals

- `Reducer::whnf` (no env, `reduce/types.rs:965-1004`): beta/zeta only — no nat ops.
- `Reducer::whnf_delta` (1008+): delta via lookup closure — no nat ops.
- `simp::normalize` (`simp/functions.rs:62-83`) uses the non-env `whnf`, so it does not
  fold `Nat.add 1 2` either (only `fold_nat_constants` does, with its wrong mod).

---

## 3. String/Char specifics

- Ops present (wired): `String.length` (bytes — WRONG, see §2.1 table), `String.append`
  (correct), `String.beq` (correct semantics; non-standard extension).
- `String.decEq`: absent (only `Nat.decEq` string-name appears in dead
  `eval_nat_cmp`).
- `Char`: no kernel handling at all (`grep Char` in kernel reduce/builtin: nothing).
- `String.mk` / `String.data` bridging for StrLit ↔ `List Char`: absent. The Lean kernel
  needs this to run `String.rec`/decEq on literals.
- `Nat.repr`, `String.toNat`: absent.
- Builtin env registers `String`, `String.length`, `String.append`, `String.beq` as
  axioms with types (`builtin/functions.rs:395-462`), consistent with the reduction
  extension but *not* with a real Lean export where these are definitions.

Encoding note: `Literal::Str(String)` is Rust `String` (UTF-8, no unpaired surrogates).
Lean `String` is also a sequence of Unicode scalar values, so representation is
adequate; only the *length* semantics (bytes vs chars) is wrong. If lean4export contains
strings with escapes, the (future) export reader must decode them; nothing exists yet.

---

## 4. Arbitrary-precision availability across the workspace

- **oxilean-kernel**: `Cargo.toml` `[dependencies]` is empty ("ZERO external
  dependencies - this is the TCB"), dev-deps only criterion + proptest. Verified no
  `num_bigint` import in kernel src. `#![forbid(unsafe_code)]` at `src/lib.rs:263`.
  Claim holds. But therefore also: **no bignum in the kernel**.
- **num-bigint**: workspace dep (root Cargo.toml:43) consumed ONLY by `oxilean-meta`
  (`crates/oxilean-meta/Cargo.toml:24`; single use site
  `oxilean-meta/src/tactic/polyrith/functions.rs:1169`). Not in the verify TCB path,
  but note the brief's dependency allow-list (kernel + export + wasm-bindgen) — meta is
  outside the verify product anyway.
- **oxilean-runtime**: `src/object/types.rs:34-38` `BigNatData { header, digits:
  Vec<u64> }` ("Limbs, base-2^64, least significant first") and `BigIntData` (14-21) —
  **data containers only, zero arithmetic**. The only constructor
  (`object/rtobject_big_nat_group.rs:13-19`) stores a single u64 limb. No add/mul/div
  on limbs anywhere (grep `limbs|Karatsuba` — no algorithm hits).
- **oxilean-codegen**: `runtime_codegen` has a `BigNatCodegen`
  (`runtime_codegen/types.rs:1240+`) that *emits code strings* for target backends;
  JS backend maps Nat to JS `BigInt` (`js_backend/types.rs:361-362` — ironically the
  JS *codegen* is arbitrary-precision while the kernel is not, though its literal
  carrier is still `BigInt(i64)`). No host-side arithmetic.
- **oxilean-std** `src/nat/`: number-theory utilities on `u64` (fib, totient, mobius,
  collatz) — irrelevant.
- **Karatsuba**: zero hits workspace-wide.

**Conclusion: nothing reusable exists; the bignum must be written from scratch** (or the
runtime's limb layout adopted as the representation with algorithms added).

---

## 5. def_eq wiring for literals

- Fast path exists: `DefEqChecker::is_def_eq_core` whnfs both sides via `whnf_env`
  (`def_eq/types.rs:1315-1316`) then `(Expr::Lit(l1), Expr::Lit(l2)) => l1 == l2`
  (`def_eq/types.rs:1350`). Since whnf_env applies `try_reduce_nat_app`, top-level
  one-step literal arithmetic *is* decided (e.g. `Nat.add (Lit 2) (Lit 2)` =?= `Lit 4`
  works).
- Structural-eq fallbacks also compare literals: `def_eq/functions.rs:153`
  (`syntactic_eq`), `conversion/functions.rs:554`, `whnf/functions.rs:116`
  (`WhnfHead::Lit(la) == lb`).
- **Missing**: `NatLit n+1` defeq `Nat.succ (NatLit n)`; `NatLit 0` defeq `Nat.zero`;
  literal expansion before iota (§2.3). Lean's kernel is incomplete without these on any
  real mathlib-style export.
- `quick_infer_type` (proof-irrelevance helper) types literals at
  `def_eq/types.rs:1457-1461` (Nat/Int/String), consistent with
  `TypeChecker::infer_type` at `infer/types.rs:934-936`. Note both hardcode
  `Const("Nat")` etc. without checking the environment actually declares `Nat` — in an
  adversarial export that redefines `Nat`, a literal would silently type at whatever
  `Nat` names (acceptable in Lean too, but worth noting for the verifier: lean4lean
  checks the expected constants exist with the right signature before enabling the
  extension; oxilean does not).

---

## 6. Bool results from Nat.beq/ble/blt

`nat_bool_result` (`reduce/functions.rs:322-329`): returns
`Expr::Const(Name::str("Bool.true"), vec![])` / `"Bool.false"` — matches the builtin
environment's Bool constructors (`builtin/functions.rs:93-168`, `Bool.true`/`Bool.false`
registered as `ConstructorVal` with `induct: Bool`, zero level params, so empty level
list is correct). `try_reduce_int_app` has its own identical `bool_result` closure
(reduce/functions.rs:237-243). Correct, provided the loaded export also uses standard
names (it does in Lean 4). No check that `Bool.true` exists in the loaded env before the
extension fires — same caveat as §5.

---

## 7. Parsing/ingest of numerals (pipeline context)

- `oxilean-parse/src/lexer/types.rs:460`: `let num: u64 = int_str.parse().unwrap_or(0);`
  → any surface numeral > `u64::MAX` **silently becomes 0**. (Elab passes it through:
  `oxilean-elab/src/structure/functions.rs:63`.) Not kernel TCB, but a verifier ingest
  path with the same disease.
- Kernel's own `export` module (`src/export/functions.rs`) is a **custom binary
  serialization**, not lean4export: literals round-trip as fixed 8-byte LE u64
  (write:125-135, read:375-381; Int stored `as u64`→`as i64`). A lean4export `#ELN`
  payload is an arbitrary-size decimal string — unreadable in this format.
- **No lean4export text reader exists anywhere in the workspace** (grep
  `lean4export|#ELN|#ELS`: zero hits), and there is no `oxilean-verify` or
  `oxilean-export` crate (14 members, none match). The brief's export reader is
  greenfield; its bignum-decimal parsing requirement lands on the same new Nat type.

---

## 8. Test coverage and gaps

Present:
- Unit tests for the wired path: `reduce/functions.rs:505-575`
  (`test_nat_extended_ops`: div 10/3, mod 10%3, gcd 12,8; `test_nat_bitwise_ops`:
  land/lor/xor; `test_string_ops`: length "hello"=5, append) and Int tests (600-700,
  including one stuck-arg test).
- Property tests: `crates/oxilean-kernel/tests/prop_tests.rs` — literal WHNF
  fixed-point over full `0..=u64::MAX` (line 416), defeq of distinct small literals is
  false (426-440). Values otherwise bounded ≤ 2000.
- Benches: `benches/kernel_perf.rs` exercises Nat.add/Nat.mul via whnf_env.

Gaps (all of these are required by the brief or by soundness):
- **No overflow tests** (`Nat.add u64::MAX 1`, `Nat.mul 2^33 2^33`, `Nat.pow 2 64`,
  `Nat.pow x (2^32)`, `shiftLeft` boundary) — the current code would fail them
  (panic in debug, wrong value in release).
- **No mod-by-zero / div-by-zero tests** on the wired path (the simp copy would fail a
  mod-zero test today).
- **No differential tests** against a reference bignum (num-bigint as dev-dep is
  explicitly allowed by the brief) or against Lean itself.
- **No fuzzing** anywhere (`find … -name "fuzz*"`: nothing; no cargo-fuzz targets, no
  fuzz CI).
- No tests for NatLit/ctor defeq (`0 =?= Nat.zero`) or `Nat.rec` on literals — they
  would expose §2.3 immediately.
- No Unicode `String.length` test (would expose the bytes/chars bug).
- No native-vs-WASM determinism tests (brief's build-hygiene item; literal arithmetic
  in u64 is deterministic, but panic-on-overflow differs from wasm trap behavior).

---

## 9. Distance from brief & migration path

### 9.1 What the brief wants
A hand-written, auditable, unsafe-free, zero-dep arbitrary-precision Nat inside the
kernel (schoolbook + Karatsuba), all listed ops (add/mul/sub/div/mod/pow/gcd/beq/ble/
land/lor/lxor/shiftLeft/shiftRight/log2) computed directly on literals, exhaustive
differential tests (test-only reference dep OK) and fuzzing.

### 9.2 Current distance
- Representation: 0% (u64, no bignum anywhere reusable).
- Op coverage on the wired path: ~13/15 named ops present by name; but 5 of them
  (add, mul, pow, succ, shiftLeft) are wrong outside small ranges, and log2 missing.
- Kernel-completeness plumbing (lit↔ctor, arg whnf): missing — this is as much work as
  the bignum itself and gates real export files.
- Testing/fuzzing per brief: essentially 0%.

### 9.3 Recommended migration

1. **New module `oxilean_kernel::nat` (e.g. `src/bignat/`)** — the NatLit payload type:
   ```rust
   #[derive(Clone, PartialEq, Eq, Hash, Debug)]  // must match Literal's derives
   pub struct Nat { limbs: Vec<u64> }             // LSF, normalized (no trailing 0s)
   ```
   Optionally a small-value inline variant (`enum Repr { Small(u64), Big(Vec<u64>) }`)
   for allocation-free common case; keep `Ord` for ble/blt. Algorithms: schoolbook
   add/sub/cmp; schoolbook mul with Karatsuba above ~32 limbs; Knuth-D division
   (or shift-subtract initially — simpler to audit, still correct); pow by squaring with
   size guard (see step 6); binary or Euclid gcd; bitwise ops limb-wise; shifts;
   `log2 = bit_length - 1` (Lean: `Nat.log2 0 = 0`); decimal `from_str`
   (for the future lean4export reader) and `to_string` (Display, Nat.repr).
   The runtime's `BigNatData` limb layout (`Vec<u64>` LSF) can be mirrored for later
   sharing, but keep the kernel copy self-contained.
2. **Change `Literal::Nat(u64)` → `Literal::Nat(Nat)`** (expr/types.rs:835) and **delete
   `Literal::Int(i64)`** from the kernel (or feature-gate it out of the verify build);
   Int is not a Lean kernel literal and its wrapping arithmetic is indefensible in a
   TCB. Blast radius measured: `Literal::Nat(` appears **1092 times across 148 files**
   workspace-wide, **380 in the kernel** (majority in `#[cfg(test)]` blocks and the
   SplitRS-generated satellite modules). Mitigation: add
   `impl From<u64> for Nat` + a helper `Expr::nat_lit(u64)`/`Literal::nat(u64)` and do a
   mechanical rewrite `Literal::Nat(x)` → `Literal::Nat(x.into())` (pattern positions
   need `Literal::Nat(n) if let Some(n)=n.as_u64()` or a `to_u64()` accessor; keep
   `as_nat() -> Option<u64>` as a best-effort accessor to limit churn in non-TCB
   crates). Consumers that must be touched by hand: kernel `export/functions.rs`
   (u64 LE encoding → length-prefixed limbs or decimal), `pretty_print`/`elab`/`cli`
   formatters (Display already exists), `oxilean-parse` lexer
   (`parse().unwrap_or(0)` → big decimal parse), codegen JS `BigInt(i64)` carrier.
3. **Single evaluator**: replace `try_reduce_nat_app` internals with calls into
   `bignat`; delete/redirect the four duplicate evaluators (`simp::eval_nat`,
   `simp::try_simplify_nat_op`, `reduce::reduce_nat_op`, `reduce::eval_nat_binop`,
   `reduce::eval_nat_cmp`) so exactly one arithmetic implementation exists.
4. **Fix wiring**: in `whnf_core`, whnf the first two spine args before calling the
   nat extension (Lean does this); add the lit↔ctor bridge:
   - whnf/defeq: `Lit 0 ⇒ Const Nat.zero` comparison case and
     `Lit (n+1) ⇒ App(Nat.succ, Lit n)` on demand (both directions);
   - iota: in `try_reduce_recursor`, when the major premise whnfs to `Lit(Nat)`,
     expand one constructor layer (`0 → Nat.zero`, `n+1 → Nat.succ (Lit n)`), and for
     `Lit(Str)` expand to `String.mk` + `List Char` if String recursion is to be
     supported (or return "unsupported: string recursion" per the 3-bucket rule).
5. **Fix String.length** to `s.chars().count()`, add `String.decEq` handling if kept,
   or drop the String extension beyond what Lean's kernel does (mk/lit conversion) to
   minimize TCB.
6. **Resource guards** (verifier robustness): cap pow/shiftLeft result sizes
   (e.g. refuse to materialize > N limbs and return "unsupported: literal too large"
   rather than OOM) — with bignum, `Nat.pow 2 (2^60)` becomes a memory bomb; the
   3-bucket design gives a natural home for this ("unsupported"), never "verified".
7. **Tests**: differential proptest vs `num_bigint::BigUint` (dev-dependency only) over
   all ops incl. boundary values (0, 1, u64::MAX, u64::MAX±1, multi-limb, div/mod by 0,
   pow exponents ≥ 2^32, shift counts 0/63/64/65); u64 fast-path vs generic-path
   cross-check; `cargo fuzz` targets: (a) decimal parse/print round-trip, (b) op
   evaluator vs reference, (c) the future export reader. Add `overflow-checks = true`
   to `[profile.release]` as belt-and-braces during migration.
8. Note for the verify product: the kernel currently carries ~146k lines
   (SplitRS-generated satellite modules — `abstract/`, `abstract_interp/`, dozens of
   `*_traits.rs`); the bignum should not follow that pattern — one small hand-written
   module, since the brief's TCB budget (and the lib.rs claim of "~3,500 SLOC") demand
   auditable code.

---

## 10. Key file index

- `crates/oxilean-kernel/Cargo.toml` — zero runtime deps (TCB claim, holds).
- `crates/oxilean-kernel/src/lib.rs:263` — `#![forbid(unsafe_code)]`.
- `crates/oxilean-kernel/src/expr/types.rs:831-885` — `Literal` enum (`Nat(u64)`,
  `Int(i64)`, `Str(String)`); `Expr::Lit` at 1032.
- `crates/oxilean-kernel/src/reduce/functions.rs:64-224` — `try_reduce_nat_app`
  (the wired evaluator; per-op lines in §2.1); 226-317 `try_reduce_int_app`;
  313-329 `gcd`, `nat_bool_result`; 400-415 `reduce_nat_op` (legacy);
  706-766 `eval_nat_binop`/`eval_nat_cmp` (dead, disagreeing).
- `crates/oxilean-kernel/src/reduce/types.rs:1083-1155` — `whnf_core` (wiring; nat/int
  ext at 1129-1134; args not whnf'd); 1157-1185 `try_reduce_recursor` (no literal
  expansion); 965-1004 non-env `whnf` (no nat ops).
- `crates/oxilean-kernel/src/def_eq/types.rs:1291-1360` — `DefEqChecker::is_def_eq`,
  literal fast path 1350, `try_lazy_delta` 1362, `quick_infer_type` literal typing
  1457-1461.
- `crates/oxilean-kernel/src/infer/types.rs:934-936` — literal typing in TypeChecker.
- `crates/oxilean-kernel/src/builtin/functions.rs:246-393` — Nat inductive +
  `register_nat_ops` (ops registered as axioms, no values); 395-462 String axioms;
  93-168 Bool constructors.
- `crates/oxilean-kernel/src/simp/functions.rs:49-60,178-215,721-750` — duplicate
  evaluators (`try_simplify_nat_op`, `eval_nat` with wrong mod-by-zero,
  `fold_nat_constants`).
- `crates/oxilean-kernel/src/export/functions.rs:125-135,375-381` — binary literal
  (de)serialization, fixed u64.
- `crates/oxilean-kernel/tests/prop_tests.rs:401-441` — literal property tests
  (no overflow/differential coverage).
- `crates/oxilean-parse/src/lexer/types.rs:460` — numeral `parse().unwrap_or(0)`.
- `crates/oxilean-runtime/src/object/types.rs:34-38` +
  `object/rtobject_big_nat_group.rs` — data-only BigNat limb container.
- `crates/oxilean-meta/Cargo.toml:24` — sole num-bigint consumer (polyrith).
- Root `Cargo.toml` — num-bigint workspace dep; `[profile.release]` without
  overflow-checks.
